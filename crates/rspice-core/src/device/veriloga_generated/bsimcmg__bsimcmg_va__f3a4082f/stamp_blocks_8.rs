#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_128(
        p: &Parameters,
        var_dvsat: f64,
        var_dvsat_dn0: f64,
        var_dvsat_dn10: f64,
        var_dvsat_dn11: f64,
        var_dvsat_dn13: f64,
        var_dvsat_dn14: f64,
        var_dvsat_dn2: f64,
        var_dvsat_dn3: f64,
        var_dvsat_dn4: f64,
        var_dvsat_dn5: f64,
        var_dvsat_dn6: f64,
        var_dvsat_dn7: f64,
        var_dvsat_dn8: f64,
        var_dvsat_dn9: f64,
        var_esatl: f64,
        var_esatl_dn0: f64,
        var_esatl_dn10: f64,
        var_esatl_dn11: f64,
        var_esatl_dn13: f64,
        var_esatl_dn14: f64,
        var_esatl_dn2: f64,
        var_esatl_dn3: f64,
        var_esatl_dn4: f64,
        var_esatl_dn5: f64,
        var_esatl_dn6: f64,
        var_esatl_dn7: f64,
        var_esatl_dn8: f64,
        var_esatl_dn9: f64,
        var_guard624: f64,
        var_guard626: f64,
        var_guard630: f64,
        var_guard631: f64,
        var_k0_t: f64,
        var_k0_t_dn4: f64,
        var_k0si_t: f64,
        var_k0si_t_dn4: f64,
        var_leff_1: f64,
        var_leff_1_dn0: f64,
        var_leff_1_dn10: f64,
        var_leff_1_dn11: f64,
        var_leff_1_dn13: f64,
        var_leff_1_dn14: f64,
        var_leff_1_dn2: f64,
        var_leff_1_dn3: f64,
        var_leff_1_dn4: f64,
        var_leff_1_dn5: f64,
        var_leff_1_dn6: f64,
        var_leff_1_dn7: f64,
        var_leff_1_dn8: f64,
        var_leff_1_dn9: f64,
        var_nvtm: f64,
        var_nvtm_dn0: f64,
        var_nvtm_dn10: f64,
        var_nvtm_dn11: f64,
        var_nvtm_dn13: f64,
        var_nvtm_dn14: f64,
        var_nvtm_dn2: f64,
        var_nvtm_dn3: f64,
        var_nvtm_dn4: f64,
        var_nvtm_dn5: f64,
        var_nvtm_dn6: f64,
        var_nvtm_dn7: f64,
        var_nvtm_dn8: f64,
        var_nvtm_dn9: f64,
        var_q0: f64,
        var_q0_dn0: f64,
        var_q0_dn10: f64,
        var_q0_dn11: f64,
        var_q0_dn13: f64,
        var_q0_dn14: f64,
        var_q0_dn2: f64,
        var_q0_dn3: f64,
        var_q0_dn4: f64,
        var_q0_dn5: f64,
        var_q0_dn6: f64,
        var_q0_dn7: f64,
        var_q0_dn8: f64,
        var_q0_dn9: f64,
        var_qia: f64,
        var_qia_dn0: f64,
        var_qia_dn10: f64,
        var_qia_dn11: f64,
        var_qia_dn13: f64,
        var_qia_dn14: f64,
        var_qia_dn2: f64,
        var_qia_dn3: f64,
        var_qia_dn4: f64,
        var_qia_dn5: f64,
        var_qia_dn6: f64,
        var_qia_dn7: f64,
        var_qia_dn8: f64,
        var_qia_dn9: f64,
        var_qid: f64,
        var_qid_dn0: f64,
        var_qid_dn10: f64,
        var_qid_dn11: f64,
        var_qid_dn13: f64,
        var_qid_dn14: f64,
        var_qid_dn2: f64,
        var_qid_dn3: f64,
        var_qid_dn4: f64,
        var_qid_dn5: f64,
        var_qid_dn6: f64,
        var_qid_dn7: f64,
        var_qid_dn8: f64,
        var_qid_dn9: f64,
        var_qinv: f64,
        var_qinv_dn0: f64,
        var_qinv_dn10: f64,
        var_qinv_dn11: f64,
        var_qinv_dn13: f64,
        var_qinv_dn14: f64,
        var_qinv_dn2: f64,
        var_qinv_dn3: f64,
        var_qinv_dn4: f64,
        var_qinv_dn5: f64,
        var_qinv_dn6: f64,
        var_qinv_dn7: f64,
        var_qinv_dn8: f64,
        var_qinv_dn9: f64,
        var_qis: f64,
        var_qis_dn0: f64,
        var_qis_dn10: f64,
        var_qis_dn11: f64,
        var_qis_dn13: f64,
        var_qis_dn14: f64,
        var_qis_dn2: f64,
        var_qis_dn3: f64,
        var_qis_dn4: f64,
        var_qis_dn5: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_qis_dn9: f64,
        var_rdsi: f64,
        var_rdsi_dn0: f64,
        var_rdsi_dn10: f64,
        var_rdsi_dn11: f64,
        var_rdsi_dn13: f64,
        var_rdsi_dn14: f64,
        var_rdsi_dn2: f64,
        var_rdsi_dn3: f64,
        var_rdsi_dn4: f64,
        var_rdsi_dn5: f64,
        var_rdsi_dn6: f64,
        var_rdsi_dn7: f64,
        var_rdsi_dn8: f64,
        var_rdsi_dn9: f64,
        var_ueff: f64,
        var_ueff_dn0: f64,
        var_ueff_dn10: f64,
        var_ueff_dn11: f64,
        var_ueff_dn13: f64,
        var_ueff_dn14: f64,
        var_ueff_dn2: f64,
        var_ueff_dn3: f64,
        var_ueff_dn4: f64,
        var_ueff_dn5: f64,
        var_ueff_dn6: f64,
        var_ueff_dn7: f64,
        var_ueff_dn8: f64,
        var_ueff_dn9: f64,
        var_vdsat: f64,
        var_vdsat_dn0: f64,
        var_vdsat_dn10: f64,
        var_vdsat_dn11: f64,
        var_vdsat_dn13: f64,
        var_vdsat_dn14: f64,
        var_vdsat_dn2: f64,
        var_vdsat_dn3: f64,
        var_vdsat_dn4: f64,
        var_vdsat_dn5: f64,
        var_vdsat_dn6: f64,
        var_vdsat_dn7: f64,
        var_vdsat_dn8: f64,
        var_vdsat_dn9: f64,
        var_vdseff_1: f64,
        var_vdseff_1_dn0: f64,
        var_vdseff_1_dn10: f64,
        var_vdseff_1_dn11: f64,
        var_vdseff_1_dn13: f64,
        var_vdseff_1_dn14: f64,
        var_vdseff_1_dn2: f64,
        var_vdseff_1_dn3: f64,
        var_vdseff_1_dn4: f64,
        var_vdseff_1_dn5: f64,
        var_vdseff_1_dn6: f64,
        var_vdseff_1_dn7: f64,
        var_vdseff_1_dn8: f64,
        var_vdseff_1_dn9: f64,
        var_dvsat3_slot: &mut f64,
        var_dvsat3_dn0_slot: &mut f64,
        var_dvsat3_dn10_slot: &mut f64,
        var_dvsat3_dn11_slot: &mut f64,
        var_dvsat3_dn13_slot: &mut f64,
        var_dvsat3_dn14_slot: &mut f64,
        var_dvsat3_dn2_slot: &mut f64,
        var_dvsat3_dn3_slot: &mut f64,
        var_dvsat3_dn4_slot: &mut f64,
        var_dvsat3_dn5_slot: &mut f64,
        var_dvsat3_dn6_slot: &mut f64,
        var_dvsat3_dn7_slot: &mut f64,
        var_dvsat3_dn8_slot: &mut f64,
        var_dvsat3_dn9_slot: &mut f64,
        var_guard632_slot: &mut f64,
        var_guard633_slot: &mut f64,
        var_guard634_slot: &mut f64,
        var_mnud0_slot: &mut f64,
        var_mnud0_dn0_slot: &mut f64,
        var_mnud0_dn10_slot: &mut f64,
        var_mnud0_dn11_slot: &mut f64,
        var_mnud0_dn13_slot: &mut f64,
        var_mnud0_dn14_slot: &mut f64,
        var_mnud0_dn2_slot: &mut f64,
        var_mnud0_dn3_slot: &mut f64,
        var_mnud0_dn4_slot: &mut f64,
        var_mnud0_dn5_slot: &mut f64,
        var_mnud0_dn6_slot: &mut f64,
        var_mnud0_dn7_slot: &mut f64,
        var_mnud0_dn8_slot: &mut f64,
        var_mnud0_dn9_slot: &mut f64,
        var_noibeta_slot: &mut f64,
        var_noibeta_dn0_slot: &mut f64,
        var_noibeta_dn10_slot: &mut f64,
        var_noibeta_dn11_slot: &mut f64,
        var_noibeta_dn13_slot: &mut f64,
        var_noibeta_dn14_slot: &mut f64,
        var_noibeta_dn2_slot: &mut f64,
        var_noibeta_dn3_slot: &mut f64,
        var_noibeta_dn4_slot: &mut f64,
        var_noibeta_dn5_slot: &mut f64,
        var_noibeta_dn6_slot: &mut f64,
        var_noibeta_dn7_slot: &mut f64,
        var_noibeta_dn8_slot: &mut f64,
        var_noibeta_dn9_slot: &mut f64,
        var_noicorr_slot: &mut f64,
        var_noicorr_dn0_slot: &mut f64,
        var_noicorr_dn10_slot: &mut f64,
        var_noicorr_dn11_slot: &mut f64,
        var_noicorr_dn13_slot: &mut f64,
        var_noicorr_dn14_slot: &mut f64,
        var_noicorr_dn2_slot: &mut f64,
        var_noicorr_dn3_slot: &mut f64,
        var_noicorr_dn4_slot: &mut f64,
        var_noicorr_dn5_slot: &mut f64,
        var_noicorr_dn6_slot: &mut f64,
        var_noicorr_dn7_slot: &mut f64,
        var_noicorr_dn8_slot: &mut f64,
        var_noicorr_dn9_slot: &mut f64,
        var_noieta_slot: &mut f64,
        var_noieta_dn0_slot: &mut f64,
        var_noieta_dn10_slot: &mut f64,
        var_noieta_dn11_slot: &mut f64,
        var_noieta_dn13_slot: &mut f64,
        var_noieta_dn14_slot: &mut f64,
        var_noieta_dn2_slot: &mut f64,
        var_noieta_dn3_slot: &mut f64,
        var_noieta_dn4_slot: &mut f64,
        var_noieta_dn5_slot: &mut f64,
        var_noieta_dn6_slot: &mut f64,
        var_noieta_dn7_slot: &mut f64,
        var_noieta_dn8_slot: &mut f64,
        var_noieta_dn9_slot: &mut f64,
        var_noilowid_slot: &mut f64,
        var_noilowid_dn0_slot: &mut f64,
        var_noilowid_dn10_slot: &mut f64,
        var_noilowid_dn11_slot: &mut f64,
        var_noilowid_dn13_slot: &mut f64,
        var_noilowid_dn14_slot: &mut f64,
        var_noilowid_dn2_slot: &mut f64,
        var_noilowid_dn3_slot: &mut f64,
        var_noilowid_dn4_slot: &mut f64,
        var_noilowid_dn5_slot: &mut f64,
        var_noilowid_dn6_slot: &mut f64,
        var_noilowid_dn7_slot: &mut f64,
        var_noilowid_dn8_slot: &mut f64,
        var_noilowid_dn9_slot: &mut f64,
        var_noitheta_slot: &mut f64,
        var_noitheta_dn0_slot: &mut f64,
        var_noitheta_dn10_slot: &mut f64,
        var_noitheta_dn11_slot: &mut f64,
        var_noitheta_dn13_slot: &mut f64,
        var_noitheta_dn14_slot: &mut f64,
        var_noitheta_dn2_slot: &mut f64,
        var_noitheta_dn3_slot: &mut f64,
        var_noitheta_dn4_slot: &mut f64,
        var_noitheta_dn5_slot: &mut f64,
        var_noitheta_dn6_slot: &mut f64,
        var_noitheta_dn7_slot: &mut f64,
        var_noitheta_dn8_slot: &mut f64,
        var_noitheta_dn9_slot: &mut f64,
        var_noiwi_slot: &mut f64,
        var_noiwi_dn0_slot: &mut f64,
        var_noiwi_dn10_slot: &mut f64,
        var_noiwi_dn11_slot: &mut f64,
        var_noiwi_dn13_slot: &mut f64,
        var_noiwi_dn14_slot: &mut f64,
        var_noiwi_dn2_slot: &mut f64,
        var_noiwi_dn3_slot: &mut f64,
        var_noiwi_dn4_slot: &mut f64,
        var_noiwi_dn5_slot: &mut f64,
        var_noiwi_dn6_slot: &mut f64,
        var_noiwi_dn7_slot: &mut f64,
        var_noiwi_dn8_slot: &mut f64,
        var_noiwi_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
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
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
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
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
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
        var_t3_dn13_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
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
        var_t4_dn13_slot: &mut f64,
        var_t4_dn14_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
    ) {
        let mut var_dvsat3: f64 = *var_dvsat3_slot;
        let mut var_dvsat3_dn0: f64 = *var_dvsat3_dn0_slot;
        let mut var_dvsat3_dn10: f64 = *var_dvsat3_dn10_slot;
        let mut var_dvsat3_dn11: f64 = *var_dvsat3_dn11_slot;
        let mut var_dvsat3_dn13: f64 = *var_dvsat3_dn13_slot;
        let mut var_dvsat3_dn14: f64 = *var_dvsat3_dn14_slot;
        let mut var_dvsat3_dn2: f64 = *var_dvsat3_dn2_slot;
        let mut var_dvsat3_dn3: f64 = *var_dvsat3_dn3_slot;
        let mut var_dvsat3_dn4: f64 = *var_dvsat3_dn4_slot;
        let mut var_dvsat3_dn5: f64 = *var_dvsat3_dn5_slot;
        let mut var_dvsat3_dn6: f64 = *var_dvsat3_dn6_slot;
        let mut var_dvsat3_dn7: f64 = *var_dvsat3_dn7_slot;
        let mut var_dvsat3_dn8: f64 = *var_dvsat3_dn8_slot;
        let mut var_dvsat3_dn9: f64 = *var_dvsat3_dn9_slot;
        let mut var_guard632: f64 = *var_guard632_slot;
        let mut var_guard633: f64 = *var_guard633_slot;
        let mut var_guard634: f64 = *var_guard634_slot;
        let mut var_mnud0: f64 = *var_mnud0_slot;
        let mut var_mnud0_dn0: f64 = *var_mnud0_dn0_slot;
        let mut var_mnud0_dn10: f64 = *var_mnud0_dn10_slot;
        let mut var_mnud0_dn11: f64 = *var_mnud0_dn11_slot;
        let mut var_mnud0_dn13: f64 = *var_mnud0_dn13_slot;
        let mut var_mnud0_dn14: f64 = *var_mnud0_dn14_slot;
        let mut var_mnud0_dn2: f64 = *var_mnud0_dn2_slot;
        let mut var_mnud0_dn3: f64 = *var_mnud0_dn3_slot;
        let mut var_mnud0_dn4: f64 = *var_mnud0_dn4_slot;
        let mut var_mnud0_dn5: f64 = *var_mnud0_dn5_slot;
        let mut var_mnud0_dn6: f64 = *var_mnud0_dn6_slot;
        let mut var_mnud0_dn7: f64 = *var_mnud0_dn7_slot;
        let mut var_mnud0_dn8: f64 = *var_mnud0_dn8_slot;
        let mut var_mnud0_dn9: f64 = *var_mnud0_dn9_slot;
        let mut var_noibeta: f64 = *var_noibeta_slot;
        let mut var_noibeta_dn0: f64 = *var_noibeta_dn0_slot;
        let mut var_noibeta_dn10: f64 = *var_noibeta_dn10_slot;
        let mut var_noibeta_dn11: f64 = *var_noibeta_dn11_slot;
        let mut var_noibeta_dn13: f64 = *var_noibeta_dn13_slot;
        let mut var_noibeta_dn14: f64 = *var_noibeta_dn14_slot;
        let mut var_noibeta_dn2: f64 = *var_noibeta_dn2_slot;
        let mut var_noibeta_dn3: f64 = *var_noibeta_dn3_slot;
        let mut var_noibeta_dn4: f64 = *var_noibeta_dn4_slot;
        let mut var_noibeta_dn5: f64 = *var_noibeta_dn5_slot;
        let mut var_noibeta_dn6: f64 = *var_noibeta_dn6_slot;
        let mut var_noibeta_dn7: f64 = *var_noibeta_dn7_slot;
        let mut var_noibeta_dn8: f64 = *var_noibeta_dn8_slot;
        let mut var_noibeta_dn9: f64 = *var_noibeta_dn9_slot;
        let mut var_noicorr: f64 = *var_noicorr_slot;
        let mut var_noicorr_dn0: f64 = *var_noicorr_dn0_slot;
        let mut var_noicorr_dn10: f64 = *var_noicorr_dn10_slot;
        let mut var_noicorr_dn11: f64 = *var_noicorr_dn11_slot;
        let mut var_noicorr_dn13: f64 = *var_noicorr_dn13_slot;
        let mut var_noicorr_dn14: f64 = *var_noicorr_dn14_slot;
        let mut var_noicorr_dn2: f64 = *var_noicorr_dn2_slot;
        let mut var_noicorr_dn3: f64 = *var_noicorr_dn3_slot;
        let mut var_noicorr_dn4: f64 = *var_noicorr_dn4_slot;
        let mut var_noicorr_dn5: f64 = *var_noicorr_dn5_slot;
        let mut var_noicorr_dn6: f64 = *var_noicorr_dn6_slot;
        let mut var_noicorr_dn7: f64 = *var_noicorr_dn7_slot;
        let mut var_noicorr_dn8: f64 = *var_noicorr_dn8_slot;
        let mut var_noicorr_dn9: f64 = *var_noicorr_dn9_slot;
        let mut var_noieta: f64 = *var_noieta_slot;
        let mut var_noieta_dn0: f64 = *var_noieta_dn0_slot;
        let mut var_noieta_dn10: f64 = *var_noieta_dn10_slot;
        let mut var_noieta_dn11: f64 = *var_noieta_dn11_slot;
        let mut var_noieta_dn13: f64 = *var_noieta_dn13_slot;
        let mut var_noieta_dn14: f64 = *var_noieta_dn14_slot;
        let mut var_noieta_dn2: f64 = *var_noieta_dn2_slot;
        let mut var_noieta_dn3: f64 = *var_noieta_dn3_slot;
        let mut var_noieta_dn4: f64 = *var_noieta_dn4_slot;
        let mut var_noieta_dn5: f64 = *var_noieta_dn5_slot;
        let mut var_noieta_dn6: f64 = *var_noieta_dn6_slot;
        let mut var_noieta_dn7: f64 = *var_noieta_dn7_slot;
        let mut var_noieta_dn8: f64 = *var_noieta_dn8_slot;
        let mut var_noieta_dn9: f64 = *var_noieta_dn9_slot;
        let mut var_noilowid: f64 = *var_noilowid_slot;
        let mut var_noilowid_dn0: f64 = *var_noilowid_dn0_slot;
        let mut var_noilowid_dn10: f64 = *var_noilowid_dn10_slot;
        let mut var_noilowid_dn11: f64 = *var_noilowid_dn11_slot;
        let mut var_noilowid_dn13: f64 = *var_noilowid_dn13_slot;
        let mut var_noilowid_dn14: f64 = *var_noilowid_dn14_slot;
        let mut var_noilowid_dn2: f64 = *var_noilowid_dn2_slot;
        let mut var_noilowid_dn3: f64 = *var_noilowid_dn3_slot;
        let mut var_noilowid_dn4: f64 = *var_noilowid_dn4_slot;
        let mut var_noilowid_dn5: f64 = *var_noilowid_dn5_slot;
        let mut var_noilowid_dn6: f64 = *var_noilowid_dn6_slot;
        let mut var_noilowid_dn7: f64 = *var_noilowid_dn7_slot;
        let mut var_noilowid_dn8: f64 = *var_noilowid_dn8_slot;
        let mut var_noilowid_dn9: f64 = *var_noilowid_dn9_slot;
        let mut var_noitheta: f64 = *var_noitheta_slot;
        let mut var_noitheta_dn0: f64 = *var_noitheta_dn0_slot;
        let mut var_noitheta_dn10: f64 = *var_noitheta_dn10_slot;
        let mut var_noitheta_dn11: f64 = *var_noitheta_dn11_slot;
        let mut var_noitheta_dn13: f64 = *var_noitheta_dn13_slot;
        let mut var_noitheta_dn14: f64 = *var_noitheta_dn14_slot;
        let mut var_noitheta_dn2: f64 = *var_noitheta_dn2_slot;
        let mut var_noitheta_dn3: f64 = *var_noitheta_dn3_slot;
        let mut var_noitheta_dn4: f64 = *var_noitheta_dn4_slot;
        let mut var_noitheta_dn5: f64 = *var_noitheta_dn5_slot;
        let mut var_noitheta_dn6: f64 = *var_noitheta_dn6_slot;
        let mut var_noitheta_dn7: f64 = *var_noitheta_dn7_slot;
        let mut var_noitheta_dn8: f64 = *var_noitheta_dn8_slot;
        let mut var_noitheta_dn9: f64 = *var_noitheta_dn9_slot;
        let mut var_noiwi: f64 = *var_noiwi_slot;
        let mut var_noiwi_dn0: f64 = *var_noiwi_dn0_slot;
        let mut var_noiwi_dn10: f64 = *var_noiwi_dn10_slot;
        let mut var_noiwi_dn11: f64 = *var_noiwi_dn11_slot;
        let mut var_noiwi_dn13: f64 = *var_noiwi_dn13_slot;
        let mut var_noiwi_dn14: f64 = *var_noiwi_dn14_slot;
        let mut var_noiwi_dn2: f64 = *var_noiwi_dn2_slot;
        let mut var_noiwi_dn3: f64 = *var_noiwi_dn3_slot;
        let mut var_noiwi_dn4: f64 = *var_noiwi_dn4_slot;
        let mut var_noiwi_dn5: f64 = *var_noiwi_dn5_slot;
        let mut var_noiwi_dn6: f64 = *var_noiwi_dn6_slot;
        let mut var_noiwi_dn7: f64 = *var_noiwi_dn7_slot;
        let mut var_noiwi_dn8: f64 = *var_noiwi_dn8_slot;
        let mut var_noiwi_dn9: f64 = *var_noiwi_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
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
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
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
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
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
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
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
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn14: f64 = *var_t4_dn14_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;

        let (assign33250_e55736, assign33250_e55736_d_n0, assign33250_e55736_d_n2, assign33250_e55736_d_n3, assign33250_e55736_d_n4, assign33250_e55736_d_n5, assign33250_e55736_d_n6, assign33250_e55736_d_n7, assign33250_e55736_d_n8, assign33250_e55736_d_n9, assign33250_e55736_d_n10, assign33250_e55736_d_n11, assign33250_e55736_d_n13, assign33250_e55736_d_n14,) = {
    if ((((var_guard624 != 0.0) && (var_guard626 == 0.0)) && (var_guard630 != 0.0)) && (var_guard631 != 0.0)) {
        let assign33250_e55699: f64 = (var_qis + 0.5);
        let assign33250_e55702: f64 = (var_qid + 0.5);
        let assign33250_e55703: f64 = (assign33250_e55699 / assign33250_e55702);
        let (assign33250_e55728, assign33250_e55728_d_n0, assign33250_e55728_d_n2, assign33250_e55728_d_n3, assign33250_e55728_d_n4, assign33250_e55728_d_n5, assign33250_e55728_d_n6, assign33250_e55728_d_n7, assign33250_e55728_d_n8, assign33250_e55728_d_n9, assign33250_e55728_d_n10, assign33250_e55728_d_n11, assign33250_e55728_d_n13, assign33250_e55728_d_n14,) = {
            if (!(assign33250_e55703 > 1e-38)) {
                let assign33250_e55708: f64 = (-87.498233534);
                (assign33250_e55708, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign33250_e55711: f64 = (var_qis + 0.5);
                let assign33250_e55714: f64 = (var_qid + 0.5);
                let assign33250_e55715: f64 = (assign33250_e55711 / assign33250_e55714);
                let (assign33250_e55727, assign33250_e55727_d_n0, assign33250_e55727_d_n2, assign33250_e55727_d_n3, assign33250_e55727_d_n4, assign33250_e55727_d_n5, assign33250_e55727_d_n6, assign33250_e55727_d_n7, assign33250_e55727_d_n8, assign33250_e55727_d_n9, assign33250_e55727_d_n10, assign33250_e55727_d_n11, assign33250_e55727_d_n13, assign33250_e55727_d_n14,) = {
                    if (assign33250_e55715 > 1e-38) {
                        let assign33250_e55720: f64 = (var_qis + 0.5);
                        let assign33250_e55723: f64 = (var_qid + 0.5);
                        let assign33250_e55724: f64 = (assign33250_e55720 / assign33250_e55723);
                        let assign33250_e55725: f64 = (assign33250_e55724).ln();
                        (assign33250_e55725, ((((var_qis_dn0 * assign33250_e55723) - (assign33250_e55720 * var_qid_dn0)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((var_qis_dn2 * assign33250_e55723) - (assign33250_e55720 * var_qid_dn2)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((var_qis_dn3 * assign33250_e55723) - (assign33250_e55720 * var_qid_dn3)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((var_qis_dn4 * assign33250_e55723) - (assign33250_e55720 * var_qid_dn4)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((var_qis_dn5 * assign33250_e55723) - (assign33250_e55720 * var_qid_dn5)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((var_qis_dn6 * assign33250_e55723) - (assign33250_e55720 * var_qid_dn6)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((var_qis_dn7 * assign33250_e55723) - (assign33250_e55720 * var_qid_dn7)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((var_qis_dn8 * assign33250_e55723) - (assign33250_e55720 * var_qid_dn8)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((var_qis_dn9 * assign33250_e55723) - (assign33250_e55720 * var_qid_dn9)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((var_qis_dn10 * assign33250_e55723) - (assign33250_e55720 * var_qid_dn10)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((var_qis_dn11 * assign33250_e55723) - (assign33250_e55720 * var_qid_dn11)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((var_qis_dn13 * assign33250_e55723) - (assign33250_e55720 * var_qid_dn13)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724), ((((var_qis_dn14 * assign33250_e55723) - (assign33250_e55720 * var_qid_dn14)) / (assign33250_e55723 * assign33250_e55723)) / assign33250_e55724),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign33250_e55727, assign33250_e55727_d_n0, assign33250_e55727_d_n2, assign33250_e55727_d_n3, assign33250_e55727_d_n4, assign33250_e55727_d_n5, assign33250_e55727_d_n6, assign33250_e55727_d_n7, assign33250_e55727_d_n8, assign33250_e55727_d_n9, assign33250_e55727_d_n10, assign33250_e55727_d_n11, assign33250_e55727_d_n13, assign33250_e55727_d_n14,)
            }
        };
        let assign33250_e55731: f64 = (var_qis + var_qid);
        let assign33250_e55733: f64 = (assign33250_e55731 + 1.0);
        let assign33250_e55734: f64 = (assign33250_e55728 * assign33250_e55733);
        (assign33250_e55734, ((assign33250_e55728_d_n0 * assign33250_e55733) + (assign33250_e55728 * (var_qis_dn0 + var_qid_dn0))), ((assign33250_e55728_d_n2 * assign33250_e55733) + (assign33250_e55728 * (var_qis_dn2 + var_qid_dn2))), ((assign33250_e55728_d_n3 * assign33250_e55733) + (assign33250_e55728 * (var_qis_dn3 + var_qid_dn3))), ((assign33250_e55728_d_n4 * assign33250_e55733) + (assign33250_e55728 * (var_qis_dn4 + var_qid_dn4))), ((assign33250_e55728_d_n5 * assign33250_e55733) + (assign33250_e55728 * (var_qis_dn5 + var_qid_dn5))), ((assign33250_e55728_d_n6 * assign33250_e55733) + (assign33250_e55728 * (var_qis_dn6 + var_qid_dn6))), ((assign33250_e55728_d_n7 * assign33250_e55733) + (assign33250_e55728 * (var_qis_dn7 + var_qid_dn7))), ((assign33250_e55728_d_n8 * assign33250_e55733) + (assign33250_e55728 * (var_qis_dn8 + var_qid_dn8))), ((assign33250_e55728_d_n9 * assign33250_e55733) + (assign33250_e55728 * (var_qis_dn9 + var_qid_dn9))), ((assign33250_e55728_d_n10 * assign33250_e55733) + (assign33250_e55728 * (var_qis_dn10 + var_qid_dn10))), ((assign33250_e55728_d_n11 * assign33250_e55733) + (assign33250_e55728 * (var_qis_dn11 + var_qid_dn11))), ((assign33250_e55728_d_n13 * assign33250_e55733) + (assign33250_e55728 * (var_qis_dn13 + var_qid_dn13))), ((assign33250_e55728_d_n14 * assign33250_e55733) + (assign33250_e55728 * (var_qis_dn14 + var_qid_dn14))),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn13, var_t3_dn14,)
    }
};
        var_t3 = assign33250_e55736;
        var_t3_dn0 = assign33250_e55736_d_n0;
        var_t3_dn2 = assign33250_e55736_d_n2;
        var_t3_dn3 = assign33250_e55736_d_n3;
        var_t3_dn4 = assign33250_e55736_d_n4;
        var_t3_dn5 = assign33250_e55736_d_n5;
        var_t3_dn6 = assign33250_e55736_d_n6;
        var_t3_dn7 = assign33250_e55736_d_n7;
        var_t3_dn8 = assign33250_e55736_d_n8;
        var_t3_dn9 = assign33250_e55736_d_n9;
        var_t3_dn10 = assign33250_e55736_d_n10;
        var_t3_dn11 = assign33250_e55736_d_n11;
        var_t3_dn13 = assign33250_e55736_d_n13;
        var_t3_dn14 = assign33250_e55736_d_n14;

        let (assign33260_e55751, assign33260_e55751_d_n0, assign33260_e55751_d_n2, assign33260_e55751_d_n3, assign33260_e55751_d_n4, assign33260_e55751_d_n5, assign33260_e55751_d_n6, assign33260_e55751_d_n7, assign33260_e55751_d_n8, assign33260_e55751_d_n9, assign33260_e55751_d_n10, assign33260_e55751_d_n11, assign33260_e55751_d_n13, assign33260_e55751_d_n14,) = {
    if ((((var_guard624 != 0.0) && (var_guard626 == 0.0)) && (var_guard630 != 0.0)) && (var_guard631 != 0.0)) {
        let assign33260_e55748: f64 = (var_qis - var_qid);
        let assign33260_e55749: f64 = (2.0 * assign33260_e55748);
        (assign33260_e55749, (2.0 * (var_qis_dn0 - var_qid_dn0)), (2.0 * (var_qis_dn2 - var_qid_dn2)), (2.0 * (var_qis_dn3 - var_qid_dn3)), (2.0 * (var_qis_dn4 - var_qid_dn4)), (2.0 * (var_qis_dn5 - var_qid_dn5)), (2.0 * (var_qis_dn6 - var_qid_dn6)), (2.0 * (var_qis_dn7 - var_qid_dn7)), (2.0 * (var_qis_dn8 - var_qid_dn8)), (2.0 * (var_qis_dn9 - var_qid_dn9)), (2.0 * (var_qis_dn10 - var_qid_dn10)), (2.0 * (var_qis_dn11 - var_qid_dn11)), (2.0 * (var_qis_dn13 - var_qid_dn13)), (2.0 * (var_qis_dn14 - var_qid_dn14)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11, var_t4_dn13, var_t4_dn14,)
    }
};
        var_t4 = assign33260_e55751;
        var_t4_dn0 = assign33260_e55751_d_n0;
        var_t4_dn2 = assign33260_e55751_d_n2;
        var_t4_dn3 = assign33260_e55751_d_n3;
        var_t4_dn4 = assign33260_e55751_d_n4;
        var_t4_dn5 = assign33260_e55751_d_n5;
        var_t4_dn6 = assign33260_e55751_d_n6;
        var_t4_dn7 = assign33260_e55751_d_n7;
        var_t4_dn8 = assign33260_e55751_d_n8;
        var_t4_dn9 = assign33260_e55751_d_n9;
        var_t4_dn10 = assign33260_e55751_d_n10;
        var_t4_dn11 = assign33260_e55751_d_n11;
        var_t4_dn13 = assign33260_e55751_d_n13;
        var_t4_dn14 = assign33260_e55751_d_n14;

        let assign33300_e55814: f64 = if p.p72 == 0.0 { 1.0 } else { 0.0 };
        var_guard632 = assign33300_e55814;

        let assign33310_e55817: f64 = if p.p72 == 1.0 { 1.0 } else { 0.0 };
        var_guard633 = assign33310_e55817;

        let (assign33320_e55823, assign33320_e55823_d_n0, assign33320_e55823_d_n2, assign33320_e55823_d_n3, assign33320_e55823_d_n4, assign33320_e55823_d_n5, assign33320_e55823_d_n6, assign33320_e55823_d_n7, assign33320_e55823_d_n8, assign33320_e55823_d_n9, assign33320_e55823_d_n10, assign33320_e55823_d_n11, assign33320_e55823_d_n13, assign33320_e55823_d_n14,) = {
    if (var_guard632 != 0.0) {
        let assign33320_e55821: f64 = (var_ueff * var_qinv);
        (assign33320_e55821, ((var_ueff_dn0 * var_qinv) + (var_ueff * var_qinv_dn0)), ((var_ueff_dn2 * var_qinv) + (var_ueff * var_qinv_dn2)), ((var_ueff_dn3 * var_qinv) + (var_ueff * var_qinv_dn3)), ((var_ueff_dn4 * var_qinv) + (var_ueff * var_qinv_dn4)), ((var_ueff_dn5 * var_qinv) + (var_ueff * var_qinv_dn5)), ((var_ueff_dn6 * var_qinv) + (var_ueff * var_qinv_dn6)), ((var_ueff_dn7 * var_qinv) + (var_ueff * var_qinv_dn7)), ((var_ueff_dn8 * var_qinv) + (var_ueff * var_qinv_dn8)), ((var_ueff_dn9 * var_qinv) + (var_ueff * var_qinv_dn9)), ((var_ueff_dn10 * var_qinv) + (var_ueff * var_qinv_dn10)), ((var_ueff_dn11 * var_qinv) + (var_ueff * var_qinv_dn11)), ((var_ueff_dn13 * var_qinv) + (var_ueff * var_qinv_dn13)), ((var_ueff_dn14 * var_qinv) + (var_ueff * var_qinv_dn14)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign33320_e55823;
        var_t0_dn0 = assign33320_e55823_d_n0;
        var_t0_dn2 = assign33320_e55823_d_n2;
        var_t0_dn3 = assign33320_e55823_d_n3;
        var_t0_dn4 = assign33320_e55823_d_n4;
        var_t0_dn5 = assign33320_e55823_d_n5;
        var_t0_dn6 = assign33320_e55823_d_n6;
        var_t0_dn7 = assign33320_e55823_d_n7;
        var_t0_dn8 = assign33320_e55823_d_n8;
        var_t0_dn9 = assign33320_e55823_d_n9;
        var_t0_dn10 = assign33320_e55823_d_n10;
        var_t0_dn11 = assign33320_e55823_d_n11;
        var_t0_dn13 = assign33320_e55823_d_n13;
        var_t0_dn14 = assign33320_e55823_d_n14;

        let (assign33330_e55833, assign33330_e55833_d_n0, assign33330_e55833_d_n2, assign33330_e55833_d_n3, assign33330_e55833_d_n4, assign33330_e55833_d_n5, assign33330_e55833_d_n6, assign33330_e55833_d_n7, assign33330_e55833_d_n8, assign33330_e55833_d_n9, assign33330_e55833_d_n10, assign33330_e55833_d_n11, assign33330_e55833_d_n13, assign33330_e55833_d_n14,) = {
    if (var_guard632 != 0.0) {
        let assign33330_e55827: f64 = (var_t0 * var_rdsi);
        let assign33330_e55830: f64 = (var_leff_1 * var_leff_1);
        let assign33330_e55831: f64 = (assign33330_e55827 + assign33330_e55830);
        (assign33330_e55831, (((var_t0_dn0 * var_rdsi) + (var_t0 * var_rdsi_dn0)) + ((var_leff_1_dn0 * var_leff_1) + (var_leff_1 * var_leff_1_dn0))), (((var_t0_dn2 * var_rdsi) + (var_t0 * var_rdsi_dn2)) + ((var_leff_1_dn2 * var_leff_1) + (var_leff_1 * var_leff_1_dn2))), (((var_t0_dn3 * var_rdsi) + (var_t0 * var_rdsi_dn3)) + ((var_leff_1_dn3 * var_leff_1) + (var_leff_1 * var_leff_1_dn3))), (((var_t0_dn4 * var_rdsi) + (var_t0 * var_rdsi_dn4)) + ((var_leff_1_dn4 * var_leff_1) + (var_leff_1 * var_leff_1_dn4))), (((var_t0_dn5 * var_rdsi) + (var_t0 * var_rdsi_dn5)) + ((var_leff_1_dn5 * var_leff_1) + (var_leff_1 * var_leff_1_dn5))), (((var_t0_dn6 * var_rdsi) + (var_t0 * var_rdsi_dn6)) + ((var_leff_1_dn6 * var_leff_1) + (var_leff_1 * var_leff_1_dn6))), (((var_t0_dn7 * var_rdsi) + (var_t0 * var_rdsi_dn7)) + ((var_leff_1_dn7 * var_leff_1) + (var_leff_1 * var_leff_1_dn7))), (((var_t0_dn8 * var_rdsi) + (var_t0 * var_rdsi_dn8)) + ((var_leff_1_dn8 * var_leff_1) + (var_leff_1 * var_leff_1_dn8))), (((var_t0_dn9 * var_rdsi) + (var_t0 * var_rdsi_dn9)) + ((var_leff_1_dn9 * var_leff_1) + (var_leff_1 * var_leff_1_dn9))), (((var_t0_dn10 * var_rdsi) + (var_t0 * var_rdsi_dn10)) + ((var_leff_1_dn10 * var_leff_1) + (var_leff_1 * var_leff_1_dn10))), (((var_t0_dn11 * var_rdsi) + (var_t0 * var_rdsi_dn11)) + ((var_leff_1_dn11 * var_leff_1) + (var_leff_1 * var_leff_1_dn11))), (((var_t0_dn13 * var_rdsi) + (var_t0 * var_rdsi_dn13)) + ((var_leff_1_dn13 * var_leff_1) + (var_leff_1 * var_leff_1_dn13))), (((var_t0_dn14 * var_rdsi) + (var_t0 * var_rdsi_dn14)) + ((var_leff_1_dn14 * var_leff_1) + (var_leff_1 * var_leff_1_dn14))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn13, var_t1_dn14,)
    }
};
        var_t1 = assign33330_e55833;
        var_t1_dn0 = assign33330_e55833_d_n0;
        var_t1_dn2 = assign33330_e55833_d_n2;
        var_t1_dn3 = assign33330_e55833_d_n3;
        var_t1_dn4 = assign33330_e55833_d_n4;
        var_t1_dn5 = assign33330_e55833_d_n5;
        var_t1_dn6 = assign33330_e55833_d_n6;
        var_t1_dn7 = assign33330_e55833_d_n7;
        var_t1_dn8 = assign33330_e55833_d_n8;
        var_t1_dn9 = assign33330_e55833_d_n9;
        var_t1_dn10 = assign33330_e55833_d_n10;
        var_t1_dn11 = assign33330_e55833_d_n11;
        var_t1_dn13 = assign33330_e55833_d_n13;
        var_t1_dn14 = assign33330_e55833_d_n14;

        let (assign33360_e55860, assign33360_e55860_d_n0, assign33360_e55860_d_n2, assign33360_e55860_d_n3, assign33360_e55860_d_n4, assign33360_e55860_d_n5, assign33360_e55860_d_n6, assign33360_e55860_d_n7, assign33360_e55860_d_n8, assign33360_e55860_d_n9, assign33360_e55860_d_n10, assign33360_e55860_d_n11, assign33360_e55860_d_n13, assign33360_e55860_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33360_e55858: f64 = (var_qia / var_esatl);
        (assign33360_e55858, (((var_qia_dn0 * var_esatl) - (var_qia * var_esatl_dn0)) / (var_esatl * var_esatl)), (((var_qia_dn2 * var_esatl) - (var_qia * var_esatl_dn2)) / (var_esatl * var_esatl)), (((var_qia_dn3 * var_esatl) - (var_qia * var_esatl_dn3)) / (var_esatl * var_esatl)), (((var_qia_dn4 * var_esatl) - (var_qia * var_esatl_dn4)) / (var_esatl * var_esatl)), (((var_qia_dn5 * var_esatl) - (var_qia * var_esatl_dn5)) / (var_esatl * var_esatl)), (((var_qia_dn6 * var_esatl) - (var_qia * var_esatl_dn6)) / (var_esatl * var_esatl)), (((var_qia_dn7 * var_esatl) - (var_qia * var_esatl_dn7)) / (var_esatl * var_esatl)), (((var_qia_dn8 * var_esatl) - (var_qia * var_esatl_dn8)) / (var_esatl * var_esatl)), (((var_qia_dn9 * var_esatl) - (var_qia * var_esatl_dn9)) / (var_esatl * var_esatl)), (((var_qia_dn10 * var_esatl) - (var_qia * var_esatl_dn10)) / (var_esatl * var_esatl)), (((var_qia_dn11 * var_esatl) - (var_qia * var_esatl_dn11)) / (var_esatl * var_esatl)), (((var_qia_dn13 * var_esatl) - (var_qia * var_esatl_dn13)) / (var_esatl * var_esatl)), (((var_qia_dn14 * var_esatl) - (var_qia * var_esatl_dn14)) / (var_esatl * var_esatl)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign33360_e55860;
        var_t0_dn0 = assign33360_e55860_d_n0;
        var_t0_dn2 = assign33360_e55860_d_n2;
        var_t0_dn3 = assign33360_e55860_d_n3;
        var_t0_dn4 = assign33360_e55860_d_n4;
        var_t0_dn5 = assign33360_e55860_d_n5;
        var_t0_dn6 = assign33360_e55860_d_n6;
        var_t0_dn7 = assign33360_e55860_d_n7;
        var_t0_dn8 = assign33360_e55860_d_n8;
        var_t0_dn9 = assign33360_e55860_d_n9;
        var_t0_dn10 = assign33360_e55860_d_n10;
        var_t0_dn11 = assign33360_e55860_d_n11;
        var_t0_dn13 = assign33360_e55860_d_n13;
        var_t0_dn14 = assign33360_e55860_d_n14;

        let (assign33370_e55869, assign33370_e55869_d_n0, assign33370_e55869_d_n2, assign33370_e55869_d_n3, assign33370_e55869_d_n4, assign33370_e55869_d_n5, assign33370_e55869_d_n6, assign33370_e55869_d_n7, assign33370_e55869_d_n8, assign33370_e55869_d_n9, assign33370_e55869_d_n10, assign33370_e55869_d_n11, assign33370_e55869_d_n13, assign33370_e55869_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33370_e55867: f64 = (var_t0 * var_t0);
        (assign33370_e55867, ((var_t0_dn0 * var_t0) + (var_t0 * var_t0_dn0)), ((var_t0_dn2 * var_t0) + (var_t0 * var_t0_dn2)), ((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)), ((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)), ((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)), ((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)), ((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)), ((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)), ((var_t0_dn9 * var_t0) + (var_t0 * var_t0_dn9)), ((var_t0_dn10 * var_t0) + (var_t0 * var_t0_dn10)), ((var_t0_dn11 * var_t0) + (var_t0 * var_t0_dn11)), ((var_t0_dn13 * var_t0) + (var_t0 * var_t0_dn13)), ((var_t0_dn14 * var_t0) + (var_t0 * var_t0_dn14)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign33370_e55869;
        var_t0_dn0 = assign33370_e55869_d_n0;
        var_t0_dn2 = assign33370_e55869_d_n2;
        var_t0_dn3 = assign33370_e55869_d_n3;
        var_t0_dn4 = assign33370_e55869_d_n4;
        var_t0_dn5 = assign33370_e55869_d_n5;
        var_t0_dn6 = assign33370_e55869_d_n6;
        var_t0_dn7 = assign33370_e55869_d_n7;
        var_t0_dn8 = assign33370_e55869_d_n8;
        var_t0_dn9 = assign33370_e55869_d_n9;
        var_t0_dn10 = assign33370_e55869_d_n10;
        var_t0_dn11 = assign33370_e55869_d_n11;
        var_t0_dn13 = assign33370_e55869_d_n13;
        var_t0_dn14 = assign33370_e55869_d_n14;

        let (assign33380_e55884, assign33380_e55884_d_n0, assign33380_e55884_d_n2, assign33380_e55884_d_n3, assign33380_e55884_d_n4, assign33380_e55884_d_n5, assign33380_e55884_d_n6, assign33380_e55884_d_n7, assign33380_e55884_d_n8, assign33380_e55884_d_n9, assign33380_e55884_d_n10, assign33380_e55884_d_n11, assign33380_e55884_d_n13, assign33380_e55884_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33380_e55878: f64 = (var_t0 * p.p1709);
        let assign33380_e55880: f64 = (assign33380_e55878 * var_leff_1);
        let assign33380_e55881: f64 = (1.0 + assign33380_e55880);
        let assign33380_e55882: f64 = (p.p1708 * assign33380_e55881);
        (assign33380_e55882, (p.p1708 * (((var_t0_dn0 * p.p1709) * var_leff_1) + (assign33380_e55878 * var_leff_1_dn0))), (p.p1708 * (((var_t0_dn2 * p.p1709) * var_leff_1) + (assign33380_e55878 * var_leff_1_dn2))), (p.p1708 * (((var_t0_dn3 * p.p1709) * var_leff_1) + (assign33380_e55878 * var_leff_1_dn3))), (p.p1708 * (((var_t0_dn4 * p.p1709) * var_leff_1) + (assign33380_e55878 * var_leff_1_dn4))), (p.p1708 * (((var_t0_dn5 * p.p1709) * var_leff_1) + (assign33380_e55878 * var_leff_1_dn5))), (p.p1708 * (((var_t0_dn6 * p.p1709) * var_leff_1) + (assign33380_e55878 * var_leff_1_dn6))), (p.p1708 * (((var_t0_dn7 * p.p1709) * var_leff_1) + (assign33380_e55878 * var_leff_1_dn7))), (p.p1708 * (((var_t0_dn8 * p.p1709) * var_leff_1) + (assign33380_e55878 * var_leff_1_dn8))), (p.p1708 * (((var_t0_dn9 * p.p1709) * var_leff_1) + (assign33380_e55878 * var_leff_1_dn9))), (p.p1708 * (((var_t0_dn10 * p.p1709) * var_leff_1) + (assign33380_e55878 * var_leff_1_dn10))), (p.p1708 * (((var_t0_dn11 * p.p1709) * var_leff_1) + (assign33380_e55878 * var_leff_1_dn11))), (p.p1708 * (((var_t0_dn13 * p.p1709) * var_leff_1) + (assign33380_e55878 * var_leff_1_dn13))), (p.p1708 * (((var_t0_dn14 * p.p1709) * var_leff_1) + (assign33380_e55878 * var_leff_1_dn14))),)
    } else {
        (var_noibeta, var_noibeta_dn0, var_noibeta_dn2, var_noibeta_dn3, var_noibeta_dn4, var_noibeta_dn5, var_noibeta_dn6, var_noibeta_dn7, var_noibeta_dn8, var_noibeta_dn9, var_noibeta_dn10, var_noibeta_dn11, var_noibeta_dn13, var_noibeta_dn14,)
    }
};
        var_noibeta = assign33380_e55884;
        var_noibeta_dn0 = assign33380_e55884_d_n0;
        var_noibeta_dn2 = assign33380_e55884_d_n2;
        var_noibeta_dn3 = assign33380_e55884_d_n3;
        var_noibeta_dn4 = assign33380_e55884_d_n4;
        var_noibeta_dn5 = assign33380_e55884_d_n5;
        var_noibeta_dn6 = assign33380_e55884_d_n6;
        var_noibeta_dn7 = assign33380_e55884_d_n7;
        var_noibeta_dn8 = assign33380_e55884_d_n8;
        var_noibeta_dn9 = assign33380_e55884_d_n9;
        var_noibeta_dn10 = assign33380_e55884_d_n10;
        var_noibeta_dn11 = assign33380_e55884_d_n11;
        var_noibeta_dn13 = assign33380_e55884_d_n13;
        var_noibeta_dn14 = assign33380_e55884_d_n14;

        let (assign33390_e55899, assign33390_e55899_d_n0, assign33390_e55899_d_n2, assign33390_e55899_d_n3, assign33390_e55899_d_n4, assign33390_e55899_d_n5, assign33390_e55899_d_n6, assign33390_e55899_d_n7, assign33390_e55899_d_n8, assign33390_e55899_d_n9, assign33390_e55899_d_n10, assign33390_e55899_d_n11, assign33390_e55899_d_n13, assign33390_e55899_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33390_e55893: f64 = (var_t0 * p.p1711);
        let assign33390_e55895: f64 = (assign33390_e55893 * var_leff_1);
        let assign33390_e55896: f64 = (1.0 + assign33390_e55895);
        let assign33390_e55897: f64 = (p.p1710 * assign33390_e55896);
        (assign33390_e55897, (p.p1710 * (((var_t0_dn0 * p.p1711) * var_leff_1) + (assign33390_e55893 * var_leff_1_dn0))), (p.p1710 * (((var_t0_dn2 * p.p1711) * var_leff_1) + (assign33390_e55893 * var_leff_1_dn2))), (p.p1710 * (((var_t0_dn3 * p.p1711) * var_leff_1) + (assign33390_e55893 * var_leff_1_dn3))), (p.p1710 * (((var_t0_dn4 * p.p1711) * var_leff_1) + (assign33390_e55893 * var_leff_1_dn4))), (p.p1710 * (((var_t0_dn5 * p.p1711) * var_leff_1) + (assign33390_e55893 * var_leff_1_dn5))), (p.p1710 * (((var_t0_dn6 * p.p1711) * var_leff_1) + (assign33390_e55893 * var_leff_1_dn6))), (p.p1710 * (((var_t0_dn7 * p.p1711) * var_leff_1) + (assign33390_e55893 * var_leff_1_dn7))), (p.p1710 * (((var_t0_dn8 * p.p1711) * var_leff_1) + (assign33390_e55893 * var_leff_1_dn8))), (p.p1710 * (((var_t0_dn9 * p.p1711) * var_leff_1) + (assign33390_e55893 * var_leff_1_dn9))), (p.p1710 * (((var_t0_dn10 * p.p1711) * var_leff_1) + (assign33390_e55893 * var_leff_1_dn10))), (p.p1710 * (((var_t0_dn11 * p.p1711) * var_leff_1) + (assign33390_e55893 * var_leff_1_dn11))), (p.p1710 * (((var_t0_dn13 * p.p1711) * var_leff_1) + (assign33390_e55893 * var_leff_1_dn13))), (p.p1710 * (((var_t0_dn14 * p.p1711) * var_leff_1) + (assign33390_e55893 * var_leff_1_dn14))),)
    } else {
        (var_noitheta, var_noitheta_dn0, var_noitheta_dn2, var_noitheta_dn3, var_noitheta_dn4, var_noitheta_dn5, var_noitheta_dn6, var_noitheta_dn7, var_noitheta_dn8, var_noitheta_dn9, var_noitheta_dn10, var_noitheta_dn11, var_noitheta_dn13, var_noitheta_dn14,)
    }
};
        var_noitheta = assign33390_e55899;
        var_noitheta_dn0 = assign33390_e55899_d_n0;
        var_noitheta_dn2 = assign33390_e55899_d_n2;
        var_noitheta_dn3 = assign33390_e55899_d_n3;
        var_noitheta_dn4 = assign33390_e55899_d_n4;
        var_noitheta_dn5 = assign33390_e55899_d_n5;
        var_noitheta_dn6 = assign33390_e55899_d_n6;
        var_noitheta_dn7 = assign33390_e55899_d_n7;
        var_noitheta_dn8 = assign33390_e55899_d_n8;
        var_noitheta_dn9 = assign33390_e55899_d_n9;
        var_noitheta_dn10 = assign33390_e55899_d_n10;
        var_noitheta_dn11 = assign33390_e55899_d_n11;
        var_noitheta_dn13 = assign33390_e55899_d_n13;
        var_noitheta_dn14 = assign33390_e55899_d_n14;

        let (assign33400_e55914, assign33400_e55914_d_n0, assign33400_e55914_d_n2, assign33400_e55914_d_n3, assign33400_e55914_d_n4, assign33400_e55914_d_n5, assign33400_e55914_d_n6, assign33400_e55914_d_n7, assign33400_e55914_d_n8, assign33400_e55914_d_n9, assign33400_e55914_d_n10, assign33400_e55914_d_n11, assign33400_e55914_d_n13, assign33400_e55914_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33400_e55908: f64 = (var_t0 * p.p1713);
        let assign33400_e55910: f64 = (assign33400_e55908 * var_leff_1);
        let assign33400_e55911: f64 = (1.0 + assign33400_e55910);
        let assign33400_e55912: f64 = (p.p1712 * assign33400_e55911);
        (assign33400_e55912, (p.p1712 * (((var_t0_dn0 * p.p1713) * var_leff_1) + (assign33400_e55908 * var_leff_1_dn0))), (p.p1712 * (((var_t0_dn2 * p.p1713) * var_leff_1) + (assign33400_e55908 * var_leff_1_dn2))), (p.p1712 * (((var_t0_dn3 * p.p1713) * var_leff_1) + (assign33400_e55908 * var_leff_1_dn3))), (p.p1712 * (((var_t0_dn4 * p.p1713) * var_leff_1) + (assign33400_e55908 * var_leff_1_dn4))), (p.p1712 * (((var_t0_dn5 * p.p1713) * var_leff_1) + (assign33400_e55908 * var_leff_1_dn5))), (p.p1712 * (((var_t0_dn6 * p.p1713) * var_leff_1) + (assign33400_e55908 * var_leff_1_dn6))), (p.p1712 * (((var_t0_dn7 * p.p1713) * var_leff_1) + (assign33400_e55908 * var_leff_1_dn7))), (p.p1712 * (((var_t0_dn8 * p.p1713) * var_leff_1) + (assign33400_e55908 * var_leff_1_dn8))), (p.p1712 * (((var_t0_dn9 * p.p1713) * var_leff_1) + (assign33400_e55908 * var_leff_1_dn9))), (p.p1712 * (((var_t0_dn10 * p.p1713) * var_leff_1) + (assign33400_e55908 * var_leff_1_dn10))), (p.p1712 * (((var_t0_dn11 * p.p1713) * var_leff_1) + (assign33400_e55908 * var_leff_1_dn11))), (p.p1712 * (((var_t0_dn13 * p.p1713) * var_leff_1) + (assign33400_e55908 * var_leff_1_dn13))), (p.p1712 * (((var_t0_dn14 * p.p1713) * var_leff_1) + (assign33400_e55908 * var_leff_1_dn14))),)
    } else {
        (var_noicorr, var_noicorr_dn0, var_noicorr_dn2, var_noicorr_dn3, var_noicorr_dn4, var_noicorr_dn5, var_noicorr_dn6, var_noicorr_dn7, var_noicorr_dn8, var_noicorr_dn9, var_noicorr_dn10, var_noicorr_dn11, var_noicorr_dn13, var_noicorr_dn14,)
    }
};
        var_noicorr = assign33400_e55914;
        var_noicorr_dn0 = assign33400_e55914_d_n0;
        var_noicorr_dn2 = assign33400_e55914_d_n2;
        var_noicorr_dn3 = assign33400_e55914_d_n3;
        var_noicorr_dn4 = assign33400_e55914_d_n4;
        var_noicorr_dn5 = assign33400_e55914_d_n5;
        var_noicorr_dn6 = assign33400_e55914_d_n6;
        var_noicorr_dn7 = assign33400_e55914_d_n7;
        var_noicorr_dn8 = assign33400_e55914_d_n8;
        var_noicorr_dn9 = assign33400_e55914_d_n9;
        var_noicorr_dn10 = assign33400_e55914_d_n10;
        var_noicorr_dn11 = assign33400_e55914_d_n11;
        var_noicorr_dn13 = assign33400_e55914_d_n13;
        var_noicorr_dn14 = assign33400_e55914_d_n14;

        let (assign33410_e55929, assign33410_e55929_d_n0, assign33410_e55929_d_n2, assign33410_e55929_d_n3, assign33410_e55929_d_n4, assign33410_e55929_d_n5, assign33410_e55929_d_n6, assign33410_e55929_d_n7, assign33410_e55929_d_n8, assign33410_e55929_d_n9, assign33410_e55929_d_n10, assign33410_e55929_d_n11, assign33410_e55929_d_n13, assign33410_e55929_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33410_e55923: f64 = (var_t0 * p.p1715);
        let assign33410_e55925: f64 = (assign33410_e55923 * var_leff_1);
        let assign33410_e55926: f64 = (1.0 + assign33410_e55925);
        let assign33410_e55927: f64 = (p.p1714 * assign33410_e55926);
        (assign33410_e55927, (p.p1714 * (((var_t0_dn0 * p.p1715) * var_leff_1) + (assign33410_e55923 * var_leff_1_dn0))), (p.p1714 * (((var_t0_dn2 * p.p1715) * var_leff_1) + (assign33410_e55923 * var_leff_1_dn2))), (p.p1714 * (((var_t0_dn3 * p.p1715) * var_leff_1) + (assign33410_e55923 * var_leff_1_dn3))), (p.p1714 * (((var_t0_dn4 * p.p1715) * var_leff_1) + (assign33410_e55923 * var_leff_1_dn4))), (p.p1714 * (((var_t0_dn5 * p.p1715) * var_leff_1) + (assign33410_e55923 * var_leff_1_dn5))), (p.p1714 * (((var_t0_dn6 * p.p1715) * var_leff_1) + (assign33410_e55923 * var_leff_1_dn6))), (p.p1714 * (((var_t0_dn7 * p.p1715) * var_leff_1) + (assign33410_e55923 * var_leff_1_dn7))), (p.p1714 * (((var_t0_dn8 * p.p1715) * var_leff_1) + (assign33410_e55923 * var_leff_1_dn8))), (p.p1714 * (((var_t0_dn9 * p.p1715) * var_leff_1) + (assign33410_e55923 * var_leff_1_dn9))), (p.p1714 * (((var_t0_dn10 * p.p1715) * var_leff_1) + (assign33410_e55923 * var_leff_1_dn10))), (p.p1714 * (((var_t0_dn11 * p.p1715) * var_leff_1) + (assign33410_e55923 * var_leff_1_dn11))), (p.p1714 * (((var_t0_dn13 * p.p1715) * var_leff_1) + (assign33410_e55923 * var_leff_1_dn13))), (p.p1714 * (((var_t0_dn14 * p.p1715) * var_leff_1) + (assign33410_e55923 * var_leff_1_dn14))),)
    } else {
        (var_noilowid, var_noilowid_dn0, var_noilowid_dn2, var_noilowid_dn3, var_noilowid_dn4, var_noilowid_dn5, var_noilowid_dn6, var_noilowid_dn7, var_noilowid_dn8, var_noilowid_dn9, var_noilowid_dn10, var_noilowid_dn11, var_noilowid_dn13, var_noilowid_dn14,)
    }
};
        var_noilowid = assign33410_e55929;
        var_noilowid_dn0 = assign33410_e55929_d_n0;
        var_noilowid_dn2 = assign33410_e55929_d_n2;
        var_noilowid_dn3 = assign33410_e55929_d_n3;
        var_noilowid_dn4 = assign33410_e55929_d_n4;
        var_noilowid_dn5 = assign33410_e55929_d_n5;
        var_noilowid_dn6 = assign33410_e55929_d_n6;
        var_noilowid_dn7 = assign33410_e55929_d_n7;
        var_noilowid_dn8 = assign33410_e55929_d_n8;
        var_noilowid_dn9 = assign33410_e55929_d_n9;
        var_noilowid_dn10 = assign33410_e55929_d_n10;
        var_noilowid_dn11 = assign33410_e55929_d_n11;
        var_noilowid_dn13 = assign33410_e55929_d_n13;
        var_noilowid_dn14 = assign33410_e55929_d_n14;

        let (assign33420_e55940, assign33420_e55940_d_n0, assign33420_e55940_d_n2, assign33420_e55940_d_n3, assign33420_e55940_d_n4, assign33420_e55940_d_n5, assign33420_e55940_d_n6, assign33420_e55940_d_n7, assign33420_e55940_d_n8, assign33420_e55940_d_n9, assign33420_e55940_d_n10, assign33420_e55940_d_n11, assign33420_e55940_d_n13, assign33420_e55940_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33420_e55936: f64 = (3.0 * var_noibeta);
        let assign33420_e55938: f64 = (assign33420_e55936 * var_noibeta);
        (assign33420_e55938, (((3.0 * var_noibeta_dn0) * var_noibeta) + (assign33420_e55936 * var_noibeta_dn0)), (((3.0 * var_noibeta_dn2) * var_noibeta) + (assign33420_e55936 * var_noibeta_dn2)), (((3.0 * var_noibeta_dn3) * var_noibeta) + (assign33420_e55936 * var_noibeta_dn3)), (((3.0 * var_noibeta_dn4) * var_noibeta) + (assign33420_e55936 * var_noibeta_dn4)), (((3.0 * var_noibeta_dn5) * var_noibeta) + (assign33420_e55936 * var_noibeta_dn5)), (((3.0 * var_noibeta_dn6) * var_noibeta) + (assign33420_e55936 * var_noibeta_dn6)), (((3.0 * var_noibeta_dn7) * var_noibeta) + (assign33420_e55936 * var_noibeta_dn7)), (((3.0 * var_noibeta_dn8) * var_noibeta) + (assign33420_e55936 * var_noibeta_dn8)), (((3.0 * var_noibeta_dn9) * var_noibeta) + (assign33420_e55936 * var_noibeta_dn9)), (((3.0 * var_noibeta_dn10) * var_noibeta) + (assign33420_e55936 * var_noibeta_dn10)), (((3.0 * var_noibeta_dn11) * var_noibeta) + (assign33420_e55936 * var_noibeta_dn11)), (((3.0 * var_noibeta_dn13) * var_noibeta) + (assign33420_e55936 * var_noibeta_dn13)), (((3.0 * var_noibeta_dn14) * var_noibeta) + (assign33420_e55936 * var_noibeta_dn14)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn13, var_t1_dn14,)
    }
};
        var_t1 = assign33420_e55940;
        var_t1_dn0 = assign33420_e55940_d_n0;
        var_t1_dn2 = assign33420_e55940_d_n2;
        var_t1_dn3 = assign33420_e55940_d_n3;
        var_t1_dn4 = assign33420_e55940_d_n4;
        var_t1_dn5 = assign33420_e55940_d_n5;
        var_t1_dn6 = assign33420_e55940_d_n6;
        var_t1_dn7 = assign33420_e55940_d_n7;
        var_t1_dn8 = assign33420_e55940_d_n8;
        var_t1_dn9 = assign33420_e55940_d_n9;
        var_t1_dn10 = assign33420_e55940_d_n10;
        var_t1_dn11 = assign33420_e55940_d_n11;
        var_t1_dn13 = assign33420_e55940_d_n13;
        var_t1_dn14 = assign33420_e55940_d_n14;

        let (assign33430_e55951, assign33430_e55951_d_n0, assign33430_e55951_d_n2, assign33430_e55951_d_n3, assign33430_e55951_d_n4, assign33430_e55951_d_n5, assign33430_e55951_d_n6, assign33430_e55951_d_n7, assign33430_e55951_d_n8, assign33430_e55951_d_n9, assign33430_e55951_d_n10, assign33430_e55951_d_n11, assign33430_e55951_d_n13, assign33430_e55951_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33430_e55947: f64 = (7.5 * var_noitheta);
        let assign33430_e55949: f64 = (assign33430_e55947 * var_noitheta);
        (assign33430_e55949, (((7.5 * var_noitheta_dn0) * var_noitheta) + (assign33430_e55947 * var_noitheta_dn0)), (((7.5 * var_noitheta_dn2) * var_noitheta) + (assign33430_e55947 * var_noitheta_dn2)), (((7.5 * var_noitheta_dn3) * var_noitheta) + (assign33430_e55947 * var_noitheta_dn3)), (((7.5 * var_noitheta_dn4) * var_noitheta) + (assign33430_e55947 * var_noitheta_dn4)), (((7.5 * var_noitheta_dn5) * var_noitheta) + (assign33430_e55947 * var_noitheta_dn5)), (((7.5 * var_noitheta_dn6) * var_noitheta) + (assign33430_e55947 * var_noitheta_dn6)), (((7.5 * var_noitheta_dn7) * var_noitheta) + (assign33430_e55947 * var_noitheta_dn7)), (((7.5 * var_noitheta_dn8) * var_noitheta) + (assign33430_e55947 * var_noitheta_dn8)), (((7.5 * var_noitheta_dn9) * var_noitheta) + (assign33430_e55947 * var_noitheta_dn9)), (((7.5 * var_noitheta_dn10) * var_noitheta) + (assign33430_e55947 * var_noitheta_dn10)), (((7.5 * var_noitheta_dn11) * var_noitheta) + (assign33430_e55947 * var_noitheta_dn11)), (((7.5 * var_noitheta_dn13) * var_noitheta) + (assign33430_e55947 * var_noitheta_dn13)), (((7.5 * var_noitheta_dn14) * var_noitheta) + (assign33430_e55947 * var_noitheta_dn14)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign33430_e55951;
        var_t2_dn0 = assign33430_e55951_d_n0;
        var_t2_dn2 = assign33430_e55951_d_n2;
        var_t2_dn3 = assign33430_e55951_d_n3;
        var_t2_dn4 = assign33430_e55951_d_n4;
        var_t2_dn5 = assign33430_e55951_d_n5;
        var_t2_dn6 = assign33430_e55951_d_n6;
        var_t2_dn7 = assign33430_e55951_d_n7;
        var_t2_dn8 = assign33430_e55951_d_n8;
        var_t2_dn9 = assign33430_e55951_d_n9;
        var_t2_dn10 = assign33430_e55951_d_n10;
        var_t2_dn11 = assign33430_e55951_d_n11;
        var_t2_dn13 = assign33430_e55951_d_n13;
        var_t2_dn14 = assign33430_e55951_d_n14;

        let (assign33440_e55960, assign33440_e55960_d_n0, assign33440_e55960_d_n2, assign33440_e55960_d_n3, assign33440_e55960_d_n4, assign33440_e55960_d_n5, assign33440_e55960_d_n6, assign33440_e55960_d_n7, assign33440_e55960_d_n8, assign33440_e55960_d_n9, assign33440_e55960_d_n10, assign33440_e55960_d_n11, assign33440_e55960_d_n13, assign33440_e55960_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33440_e55958: f64 = (2.5298 * var_noicorr);
        (assign33440_e55958, (2.5298 * var_noicorr_dn0), (2.5298 * var_noicorr_dn2), (2.5298 * var_noicorr_dn3), (2.5298 * var_noicorr_dn4), (2.5298 * var_noicorr_dn5), (2.5298 * var_noicorr_dn6), (2.5298 * var_noicorr_dn7), (2.5298 * var_noicorr_dn8), (2.5298 * var_noicorr_dn9), (2.5298 * var_noicorr_dn10), (2.5298 * var_noicorr_dn11), (2.5298 * var_noicorr_dn13), (2.5298 * var_noicorr_dn14),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn13, var_t3_dn14,)
    }
};
        var_t3 = assign33440_e55960;
        var_t3_dn0 = assign33440_e55960_d_n0;
        var_t3_dn2 = assign33440_e55960_d_n2;
        var_t3_dn3 = assign33440_e55960_d_n3;
        var_t3_dn4 = assign33440_e55960_d_n4;
        var_t3_dn5 = assign33440_e55960_d_n5;
        var_t3_dn6 = assign33440_e55960_d_n6;
        var_t3_dn7 = assign33440_e55960_d_n7;
        var_t3_dn8 = assign33440_e55960_d_n8;
        var_t3_dn9 = assign33440_e55960_d_n9;
        var_t3_dn10 = assign33440_e55960_d_n10;
        var_t3_dn11 = assign33440_e55960_d_n11;
        var_t3_dn13 = assign33440_e55960_d_n13;
        var_t3_dn14 = assign33440_e55960_d_n14;

        let (assign33450_e55975, assign33450_e55975_d_n0, assign33450_e55975_d_n2, assign33450_e55975_d_n3, assign33450_e55975_d_n4, assign33450_e55975_d_n5, assign33450_e55975_d_n6, assign33450_e55975_d_n7, assign33450_e55975_d_n8, assign33450_e55975_d_n9, assign33450_e55975_d_n10, assign33450_e55975_d_n11, assign33450_e55975_d_n13, assign33450_e55975_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33450_e55967: f64 = (var_qid / var_qis);
        let assign33450_e55971: f64 = (var_vdseff_1 / var_vdsat);
        let assign33450_e55972: f64 = (1.0 - assign33450_e55971);
        let assign33450_e55973: f64 = (assign33450_e55967 * assign33450_e55972);
        (assign33450_e55973, (((((var_qid_dn0 * var_qis) - (var_qid * var_qis_dn0)) / (var_qis * var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((var_vdseff_1_dn0 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn0)) / (var_vdsat * var_vdsat))))), (((((var_qid_dn2 * var_qis) - (var_qid * var_qis_dn2)) / (var_qis * var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((var_vdseff_1_dn2 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn2)) / (var_vdsat * var_vdsat))))), (((((var_qid_dn3 * var_qis) - (var_qid * var_qis_dn3)) / (var_qis * var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((var_vdseff_1_dn3 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn3)) / (var_vdsat * var_vdsat))))), (((((var_qid_dn4 * var_qis) - (var_qid * var_qis_dn4)) / (var_qis * var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((var_vdseff_1_dn4 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn4)) / (var_vdsat * var_vdsat))))), (((((var_qid_dn5 * var_qis) - (var_qid * var_qis_dn5)) / (var_qis * var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((var_vdseff_1_dn5 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn5)) / (var_vdsat * var_vdsat))))), (((((var_qid_dn6 * var_qis) - (var_qid * var_qis_dn6)) / (var_qis * var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((var_vdseff_1_dn6 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn6)) / (var_vdsat * var_vdsat))))), (((((var_qid_dn7 * var_qis) - (var_qid * var_qis_dn7)) / (var_qis * var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((var_vdseff_1_dn7 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn7)) / (var_vdsat * var_vdsat))))), (((((var_qid_dn8 * var_qis) - (var_qid * var_qis_dn8)) / (var_qis * var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((var_vdseff_1_dn8 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn8)) / (var_vdsat * var_vdsat))))), (((((var_qid_dn9 * var_qis) - (var_qid * var_qis_dn9)) / (var_qis * var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((var_vdseff_1_dn9 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn9)) / (var_vdsat * var_vdsat))))), (((((var_qid_dn10 * var_qis) - (var_qid * var_qis_dn10)) / (var_qis * var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((var_vdseff_1_dn10 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn10)) / (var_vdsat * var_vdsat))))), (((((var_qid_dn11 * var_qis) - (var_qid * var_qis_dn11)) / (var_qis * var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((var_vdseff_1_dn11 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn11)) / (var_vdsat * var_vdsat))))), (((((var_qid_dn13 * var_qis) - (var_qid * var_qis_dn13)) / (var_qis * var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((var_vdseff_1_dn13 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn13)) / (var_vdsat * var_vdsat))))), (((((var_qid_dn14 * var_qis) - (var_qid * var_qis_dn14)) / (var_qis * var_qis)) * assign33450_e55972) + (assign33450_e55967 * (-(((var_vdseff_1_dn14 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn14)) / (var_vdsat * var_vdsat))))),)
    } else {
        (var_noieta, var_noieta_dn0, var_noieta_dn2, var_noieta_dn3, var_noieta_dn4, var_noieta_dn5, var_noieta_dn6, var_noieta_dn7, var_noieta_dn8, var_noieta_dn9, var_noieta_dn10, var_noieta_dn11, var_noieta_dn13, var_noieta_dn14,)
    }
};
        var_noieta = assign33450_e55975;
        var_noieta_dn0 = assign33450_e55975_d_n0;
        var_noieta_dn2 = assign33450_e55975_d_n2;
        var_noieta_dn3 = assign33450_e55975_d_n3;
        var_noieta_dn4 = assign33450_e55975_d_n4;
        var_noieta_dn5 = assign33450_e55975_d_n5;
        var_noieta_dn6 = assign33450_e55975_d_n6;
        var_noieta_dn7 = assign33450_e55975_d_n7;
        var_noieta_dn8 = assign33450_e55975_d_n8;
        var_noieta_dn9 = assign33450_e55975_d_n9;
        var_noieta_dn10 = assign33450_e55975_d_n10;
        var_noieta_dn11 = assign33450_e55975_d_n11;
        var_noieta_dn13 = assign33450_e55975_d_n13;
        var_noieta_dn14 = assign33450_e55975_d_n14;

        let (assign33460_e55986, assign33460_e55986_d_n0, assign33460_e55986_d_n2, assign33460_e55986_d_n3, assign33460_e55986_d_n4, assign33460_e55986_d_n5, assign33460_e55986_d_n6, assign33460_e55986_d_n7, assign33460_e55986_d_n8, assign33460_e55986_d_n9, assign33460_e55986_d_n10, assign33460_e55986_d_n11, assign33460_e55986_d_n13, assign33460_e55986_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33460_e55982: f64 = (var_dvsat * var_dvsat);
        let assign33460_e55984: f64 = (assign33460_e55982 * var_dvsat);
        (assign33460_e55984, ((((var_dvsat_dn0 * var_dvsat) + (var_dvsat * var_dvsat_dn0)) * var_dvsat) + (assign33460_e55982 * var_dvsat_dn0)), ((((var_dvsat_dn2 * var_dvsat) + (var_dvsat * var_dvsat_dn2)) * var_dvsat) + (assign33460_e55982 * var_dvsat_dn2)), ((((var_dvsat_dn3 * var_dvsat) + (var_dvsat * var_dvsat_dn3)) * var_dvsat) + (assign33460_e55982 * var_dvsat_dn3)), ((((var_dvsat_dn4 * var_dvsat) + (var_dvsat * var_dvsat_dn4)) * var_dvsat) + (assign33460_e55982 * var_dvsat_dn4)), ((((var_dvsat_dn5 * var_dvsat) + (var_dvsat * var_dvsat_dn5)) * var_dvsat) + (assign33460_e55982 * var_dvsat_dn5)), ((((var_dvsat_dn6 * var_dvsat) + (var_dvsat * var_dvsat_dn6)) * var_dvsat) + (assign33460_e55982 * var_dvsat_dn6)), ((((var_dvsat_dn7 * var_dvsat) + (var_dvsat * var_dvsat_dn7)) * var_dvsat) + (assign33460_e55982 * var_dvsat_dn7)), ((((var_dvsat_dn8 * var_dvsat) + (var_dvsat * var_dvsat_dn8)) * var_dvsat) + (assign33460_e55982 * var_dvsat_dn8)), ((((var_dvsat_dn9 * var_dvsat) + (var_dvsat * var_dvsat_dn9)) * var_dvsat) + (assign33460_e55982 * var_dvsat_dn9)), ((((var_dvsat_dn10 * var_dvsat) + (var_dvsat * var_dvsat_dn10)) * var_dvsat) + (assign33460_e55982 * var_dvsat_dn10)), ((((var_dvsat_dn11 * var_dvsat) + (var_dvsat * var_dvsat_dn11)) * var_dvsat) + (assign33460_e55982 * var_dvsat_dn11)), ((((var_dvsat_dn13 * var_dvsat) + (var_dvsat * var_dvsat_dn13)) * var_dvsat) + (assign33460_e55982 * var_dvsat_dn13)), ((((var_dvsat_dn14 * var_dvsat) + (var_dvsat * var_dvsat_dn14)) * var_dvsat) + (assign33460_e55982 * var_dvsat_dn14)),)
    } else {
        (var_dvsat3, var_dvsat3_dn0, var_dvsat3_dn2, var_dvsat3_dn3, var_dvsat3_dn4, var_dvsat3_dn5, var_dvsat3_dn6, var_dvsat3_dn7, var_dvsat3_dn8, var_dvsat3_dn9, var_dvsat3_dn10, var_dvsat3_dn11, var_dvsat3_dn13, var_dvsat3_dn14,)
    }
};
        var_dvsat3 = assign33460_e55986;
        var_dvsat3_dn0 = assign33460_e55986_d_n0;
        var_dvsat3_dn2 = assign33460_e55986_d_n2;
        var_dvsat3_dn3 = assign33460_e55986_d_n3;
        var_dvsat3_dn4 = assign33460_e55986_d_n4;
        var_dvsat3_dn5 = assign33460_e55986_d_n5;
        var_dvsat3_dn6 = assign33460_e55986_d_n6;
        var_dvsat3_dn7 = assign33460_e55986_d_n7;
        var_dvsat3_dn8 = assign33460_e55986_d_n8;
        var_dvsat3_dn9 = assign33460_e55986_d_n9;
        var_dvsat3_dn10 = assign33460_e55986_d_n10;
        var_dvsat3_dn11 = assign33460_e55986_d_n11;
        var_dvsat3_dn13 = assign33460_e55986_d_n13;
        var_dvsat3_dn14 = assign33460_e55986_d_n14;

        let (assign33470_e55997, assign33470_e55997_d_n0, assign33470_e55997_d_n2, assign33470_e55997_d_n3, assign33470_e55997_d_n4, assign33470_e55997_d_n5, assign33470_e55997_d_n6, assign33470_e55997_d_n7, assign33470_e55997_d_n8, assign33470_e55997_d_n9, assign33470_e55997_d_n10, assign33470_e55997_d_n11, assign33470_e55997_d_n13, assign33470_e55997_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33470_e55994: f64 = (var_q0 + var_qia);
        let assign33470_e55995: f64 = (var_q0 / assign33470_e55994);
        (assign33470_e55995, (((var_q0_dn0 * assign33470_e55994) - (var_q0 * (var_q0_dn0 + var_qia_dn0))) / (assign33470_e55994 * assign33470_e55994)), (((var_q0_dn2 * assign33470_e55994) - (var_q0 * (var_q0_dn2 + var_qia_dn2))) / (assign33470_e55994 * assign33470_e55994)), (((var_q0_dn3 * assign33470_e55994) - (var_q0 * (var_q0_dn3 + var_qia_dn3))) / (assign33470_e55994 * assign33470_e55994)), (((var_q0_dn4 * assign33470_e55994) - (var_q0 * (var_q0_dn4 + var_qia_dn4))) / (assign33470_e55994 * assign33470_e55994)), (((var_q0_dn5 * assign33470_e55994) - (var_q0 * (var_q0_dn5 + var_qia_dn5))) / (assign33470_e55994 * assign33470_e55994)), (((var_q0_dn6 * assign33470_e55994) - (var_q0 * (var_q0_dn6 + var_qia_dn6))) / (assign33470_e55994 * assign33470_e55994)), (((var_q0_dn7 * assign33470_e55994) - (var_q0 * (var_q0_dn7 + var_qia_dn7))) / (assign33470_e55994 * assign33470_e55994)), (((var_q0_dn8 * assign33470_e55994) - (var_q0 * (var_q0_dn8 + var_qia_dn8))) / (assign33470_e55994 * assign33470_e55994)), (((var_q0_dn9 * assign33470_e55994) - (var_q0 * (var_q0_dn9 + var_qia_dn9))) / (assign33470_e55994 * assign33470_e55994)), (((var_q0_dn10 * assign33470_e55994) - (var_q0 * (var_q0_dn10 + var_qia_dn10))) / (assign33470_e55994 * assign33470_e55994)), (((var_q0_dn11 * assign33470_e55994) - (var_q0 * (var_q0_dn11 + var_qia_dn11))) / (assign33470_e55994 * assign33470_e55994)), (((var_q0_dn13 * assign33470_e55994) - (var_q0 * (var_q0_dn13 + var_qia_dn13))) / (assign33470_e55994 * assign33470_e55994)), (((var_q0_dn14 * assign33470_e55994) - (var_q0 * (var_q0_dn14 + var_qia_dn14))) / (assign33470_e55994 * assign33470_e55994)),)
    } else {
        (var_noiwi, var_noiwi_dn0, var_noiwi_dn2, var_noiwi_dn3, var_noiwi_dn4, var_noiwi_dn5, var_noiwi_dn6, var_noiwi_dn7, var_noiwi_dn8, var_noiwi_dn9, var_noiwi_dn10, var_noiwi_dn11, var_noiwi_dn13, var_noiwi_dn14,)
    }
};
        var_noiwi = assign33470_e55997;
        var_noiwi_dn0 = assign33470_e55997_d_n0;
        var_noiwi_dn2 = assign33470_e55997_d_n2;
        var_noiwi_dn3 = assign33470_e55997_d_n3;
        var_noiwi_dn4 = assign33470_e55997_d_n4;
        var_noiwi_dn5 = assign33470_e55997_d_n5;
        var_noiwi_dn6 = assign33470_e55997_d_n6;
        var_noiwi_dn7 = assign33470_e55997_d_n7;
        var_noiwi_dn8 = assign33470_e55997_d_n8;
        var_noiwi_dn9 = assign33470_e55997_d_n9;
        var_noiwi_dn10 = assign33470_e55997_d_n10;
        var_noiwi_dn11 = assign33470_e55997_d_n11;
        var_noiwi_dn13 = assign33470_e55997_d_n13;
        var_noiwi_dn14 = assign33470_e55997_d_n14;

        let (assign33480_e56014, assign33480_e56014_d_n0, assign33480_e56014_d_n2, assign33480_e56014_d_n3, assign33480_e56014_d_n4, assign33480_e56014_d_n5, assign33480_e56014_d_n6, assign33480_e56014_d_n7, assign33480_e56014_d_n8, assign33480_e56014_d_n9, assign33480_e56014_d_n10, assign33480_e56014_d_n11, assign33480_e56014_d_n13, assign33480_e56014_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33480_e56005: f64 = (0.0_f64).max(var_k0si_t);
        let assign33480_e56007: f64 = (assign33480_e56005 * var_qis);
        let assign33480_e56010: f64 = (2.0 * var_nvtm);
        let assign33480_e56011: f64 = (assign33480_e56007 + assign33480_e56010);
        let assign33480_e56012: f64 = (var_k0_t / assign33480_e56011);
        (assign33480_e56012, (-((var_k0_t * ((assign33480_e56005 * var_qis_dn0) + (2.0 * var_nvtm_dn0))) / (assign33480_e56011 * assign33480_e56011))), (-((var_k0_t * ((assign33480_e56005 * var_qis_dn2) + (2.0 * var_nvtm_dn2))) / (assign33480_e56011 * assign33480_e56011))), (-((var_k0_t * ((assign33480_e56005 * var_qis_dn3) + (2.0 * var_nvtm_dn3))) / (assign33480_e56011 * assign33480_e56011))), (((var_k0_t_dn4 * assign33480_e56011) - (var_k0_t * (((if 0.0 >= var_k0si_t { 0.0 } else { var_k0si_t_dn4 } * var_qis) + (assign33480_e56005 * var_qis_dn4)) + (2.0 * var_nvtm_dn4)))) / (assign33480_e56011 * assign33480_e56011)), (-((var_k0_t * ((assign33480_e56005 * var_qis_dn5) + (2.0 * var_nvtm_dn5))) / (assign33480_e56011 * assign33480_e56011))), (-((var_k0_t * ((assign33480_e56005 * var_qis_dn6) + (2.0 * var_nvtm_dn6))) / (assign33480_e56011 * assign33480_e56011))), (-((var_k0_t * ((assign33480_e56005 * var_qis_dn7) + (2.0 * var_nvtm_dn7))) / (assign33480_e56011 * assign33480_e56011))), (-((var_k0_t * ((assign33480_e56005 * var_qis_dn8) + (2.0 * var_nvtm_dn8))) / (assign33480_e56011 * assign33480_e56011))), (-((var_k0_t * ((assign33480_e56005 * var_qis_dn9) + (2.0 * var_nvtm_dn9))) / (assign33480_e56011 * assign33480_e56011))), (-((var_k0_t * ((assign33480_e56005 * var_qis_dn10) + (2.0 * var_nvtm_dn10))) / (assign33480_e56011 * assign33480_e56011))), (-((var_k0_t * ((assign33480_e56005 * var_qis_dn11) + (2.0 * var_nvtm_dn11))) / (assign33480_e56011 * assign33480_e56011))), (-((var_k0_t * ((assign33480_e56005 * var_qis_dn13) + (2.0 * var_nvtm_dn13))) / (assign33480_e56011 * assign33480_e56011))), (-((var_k0_t * ((assign33480_e56005 * var_qis_dn14) + (2.0 * var_nvtm_dn14))) / (assign33480_e56011 * assign33480_e56011))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11, var_t4_dn13, var_t4_dn14,)
    }
};
        var_t4 = assign33480_e56014;
        var_t4_dn0 = assign33480_e56014_d_n0;
        var_t4_dn2 = assign33480_e56014_d_n2;
        var_t4_dn3 = assign33480_e56014_d_n3;
        var_t4_dn4 = assign33480_e56014_d_n4;
        var_t4_dn5 = assign33480_e56014_d_n5;
        var_t4_dn6 = assign33480_e56014_d_n6;
        var_t4_dn7 = assign33480_e56014_d_n7;
        var_t4_dn8 = assign33480_e56014_d_n8;
        var_t4_dn9 = assign33480_e56014_d_n9;
        var_t4_dn10 = assign33480_e56014_d_n10;
        var_t4_dn11 = assign33480_e56014_d_n11;
        var_t4_dn13 = assign33480_e56014_d_n13;
        var_t4_dn14 = assign33480_e56014_d_n14;

        let (assign33490_e56023, assign33490_e56023_d_n0, assign33490_e56023_d_n2, assign33490_e56023_d_n3, assign33490_e56023_d_n4, assign33490_e56023_d_n5, assign33490_e56023_d_n6, assign33490_e56023_d_n7, assign33490_e56023_d_n8, assign33490_e56023_d_n9, assign33490_e56023_d_n10, assign33490_e56023_d_n11, assign33490_e56023_d_n13, assign33490_e56023_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33490_e56020: f64 = (-var_t4);
        let assign33490_e56021: f64 = { let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign33490_e56021, ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t4_dn0)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t4_dn2)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t4_dn3)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t4_dn4)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t4_dn5)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t4_dn6)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t4_dn7)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t4_dn8)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t4_dn9)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t4_dn10)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t4_dn11)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t4_dn13)), ({ let limited_exp_arg = assign33490_e56020; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t4_dn14)),)
    } else {
        (var_mnud0, var_mnud0_dn0, var_mnud0_dn2, var_mnud0_dn3, var_mnud0_dn4, var_mnud0_dn5, var_mnud0_dn6, var_mnud0_dn7, var_mnud0_dn8, var_mnud0_dn9, var_mnud0_dn10, var_mnud0_dn11, var_mnud0_dn13, var_mnud0_dn14,)
    }
};
        var_mnud0 = assign33490_e56023;
        var_mnud0_dn0 = assign33490_e56023_d_n0;
        var_mnud0_dn2 = assign33490_e56023_d_n2;
        var_mnud0_dn3 = assign33490_e56023_d_n3;
        var_mnud0_dn4 = assign33490_e56023_d_n4;
        var_mnud0_dn5 = assign33490_e56023_d_n5;
        var_mnud0_dn6 = assign33490_e56023_d_n6;
        var_mnud0_dn7 = assign33490_e56023_d_n7;
        var_mnud0_dn8 = assign33490_e56023_d_n8;
        var_mnud0_dn9 = assign33490_e56023_d_n9;
        var_mnud0_dn10 = assign33490_e56023_d_n10;
        var_mnud0_dn11 = assign33490_e56023_d_n11;
        var_mnud0_dn13 = assign33490_e56023_d_n13;
        var_mnud0_dn14 = assign33490_e56023_d_n14;

        let assign33500_e56026: f64 = if p.p61 == 2.0 { 1.0 } else { 0.0 };
        var_guard634 = assign33500_e56026;

        *var_dvsat3_slot = var_dvsat3;
        *var_dvsat3_dn0_slot = var_dvsat3_dn0;
        *var_dvsat3_dn10_slot = var_dvsat3_dn10;
        *var_dvsat3_dn11_slot = var_dvsat3_dn11;
        *var_dvsat3_dn13_slot = var_dvsat3_dn13;
        *var_dvsat3_dn14_slot = var_dvsat3_dn14;
        *var_dvsat3_dn2_slot = var_dvsat3_dn2;
        *var_dvsat3_dn3_slot = var_dvsat3_dn3;
        *var_dvsat3_dn4_slot = var_dvsat3_dn4;
        *var_dvsat3_dn5_slot = var_dvsat3_dn5;
        *var_dvsat3_dn6_slot = var_dvsat3_dn6;
        *var_dvsat3_dn7_slot = var_dvsat3_dn7;
        *var_dvsat3_dn8_slot = var_dvsat3_dn8;
        *var_dvsat3_dn9_slot = var_dvsat3_dn9;
        *var_guard632_slot = var_guard632;
        *var_guard633_slot = var_guard633;
        *var_guard634_slot = var_guard634;
        *var_mnud0_slot = var_mnud0;
        *var_mnud0_dn0_slot = var_mnud0_dn0;
        *var_mnud0_dn10_slot = var_mnud0_dn10;
        *var_mnud0_dn11_slot = var_mnud0_dn11;
        *var_mnud0_dn13_slot = var_mnud0_dn13;
        *var_mnud0_dn14_slot = var_mnud0_dn14;
        *var_mnud0_dn2_slot = var_mnud0_dn2;
        *var_mnud0_dn3_slot = var_mnud0_dn3;
        *var_mnud0_dn4_slot = var_mnud0_dn4;
        *var_mnud0_dn5_slot = var_mnud0_dn5;
        *var_mnud0_dn6_slot = var_mnud0_dn6;
        *var_mnud0_dn7_slot = var_mnud0_dn7;
        *var_mnud0_dn8_slot = var_mnud0_dn8;
        *var_mnud0_dn9_slot = var_mnud0_dn9;
        *var_noibeta_slot = var_noibeta;
        *var_noibeta_dn0_slot = var_noibeta_dn0;
        *var_noibeta_dn10_slot = var_noibeta_dn10;
        *var_noibeta_dn11_slot = var_noibeta_dn11;
        *var_noibeta_dn13_slot = var_noibeta_dn13;
        *var_noibeta_dn14_slot = var_noibeta_dn14;
        *var_noibeta_dn2_slot = var_noibeta_dn2;
        *var_noibeta_dn3_slot = var_noibeta_dn3;
        *var_noibeta_dn4_slot = var_noibeta_dn4;
        *var_noibeta_dn5_slot = var_noibeta_dn5;
        *var_noibeta_dn6_slot = var_noibeta_dn6;
        *var_noibeta_dn7_slot = var_noibeta_dn7;
        *var_noibeta_dn8_slot = var_noibeta_dn8;
        *var_noibeta_dn9_slot = var_noibeta_dn9;
        *var_noicorr_slot = var_noicorr;
        *var_noicorr_dn0_slot = var_noicorr_dn0;
        *var_noicorr_dn10_slot = var_noicorr_dn10;
        *var_noicorr_dn11_slot = var_noicorr_dn11;
        *var_noicorr_dn13_slot = var_noicorr_dn13;
        *var_noicorr_dn14_slot = var_noicorr_dn14;
        *var_noicorr_dn2_slot = var_noicorr_dn2;
        *var_noicorr_dn3_slot = var_noicorr_dn3;
        *var_noicorr_dn4_slot = var_noicorr_dn4;
        *var_noicorr_dn5_slot = var_noicorr_dn5;
        *var_noicorr_dn6_slot = var_noicorr_dn6;
        *var_noicorr_dn7_slot = var_noicorr_dn7;
        *var_noicorr_dn8_slot = var_noicorr_dn8;
        *var_noicorr_dn9_slot = var_noicorr_dn9;
        *var_noieta_slot = var_noieta;
        *var_noieta_dn0_slot = var_noieta_dn0;
        *var_noieta_dn10_slot = var_noieta_dn10;
        *var_noieta_dn11_slot = var_noieta_dn11;
        *var_noieta_dn13_slot = var_noieta_dn13;
        *var_noieta_dn14_slot = var_noieta_dn14;
        *var_noieta_dn2_slot = var_noieta_dn2;
        *var_noieta_dn3_slot = var_noieta_dn3;
        *var_noieta_dn4_slot = var_noieta_dn4;
        *var_noieta_dn5_slot = var_noieta_dn5;
        *var_noieta_dn6_slot = var_noieta_dn6;
        *var_noieta_dn7_slot = var_noieta_dn7;
        *var_noieta_dn8_slot = var_noieta_dn8;
        *var_noieta_dn9_slot = var_noieta_dn9;
        *var_noilowid_slot = var_noilowid;
        *var_noilowid_dn0_slot = var_noilowid_dn0;
        *var_noilowid_dn10_slot = var_noilowid_dn10;
        *var_noilowid_dn11_slot = var_noilowid_dn11;
        *var_noilowid_dn13_slot = var_noilowid_dn13;
        *var_noilowid_dn14_slot = var_noilowid_dn14;
        *var_noilowid_dn2_slot = var_noilowid_dn2;
        *var_noilowid_dn3_slot = var_noilowid_dn3;
        *var_noilowid_dn4_slot = var_noilowid_dn4;
        *var_noilowid_dn5_slot = var_noilowid_dn5;
        *var_noilowid_dn6_slot = var_noilowid_dn6;
        *var_noilowid_dn7_slot = var_noilowid_dn7;
        *var_noilowid_dn8_slot = var_noilowid_dn8;
        *var_noilowid_dn9_slot = var_noilowid_dn9;
        *var_noitheta_slot = var_noitheta;
        *var_noitheta_dn0_slot = var_noitheta_dn0;
        *var_noitheta_dn10_slot = var_noitheta_dn10;
        *var_noitheta_dn11_slot = var_noitheta_dn11;
        *var_noitheta_dn13_slot = var_noitheta_dn13;
        *var_noitheta_dn14_slot = var_noitheta_dn14;
        *var_noitheta_dn2_slot = var_noitheta_dn2;
        *var_noitheta_dn3_slot = var_noitheta_dn3;
        *var_noitheta_dn4_slot = var_noitheta_dn4;
        *var_noitheta_dn5_slot = var_noitheta_dn5;
        *var_noitheta_dn6_slot = var_noitheta_dn6;
        *var_noitheta_dn7_slot = var_noitheta_dn7;
        *var_noitheta_dn8_slot = var_noitheta_dn8;
        *var_noitheta_dn9_slot = var_noitheta_dn9;
        *var_noiwi_slot = var_noiwi;
        *var_noiwi_dn0_slot = var_noiwi_dn0;
        *var_noiwi_dn10_slot = var_noiwi_dn10;
        *var_noiwi_dn11_slot = var_noiwi_dn11;
        *var_noiwi_dn13_slot = var_noiwi_dn13;
        *var_noiwi_dn14_slot = var_noiwi_dn14;
        *var_noiwi_dn2_slot = var_noiwi_dn2;
        *var_noiwi_dn3_slot = var_noiwi_dn3;
        *var_noiwi_dn4_slot = var_noiwi_dn4;
        *var_noiwi_dn5_slot = var_noiwi_dn5;
        *var_noiwi_dn6_slot = var_noiwi_dn6;
        *var_noiwi_dn7_slot = var_noiwi_dn7;
        *var_noiwi_dn8_slot = var_noiwi_dn8;
        *var_noiwi_dn9_slot = var_noiwi_dn9;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
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
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
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
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
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
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn3_slot = var_t3_dn3;
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
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn14_slot = var_t4_dn14;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
    }

    pub(super) fn stamp_transient_block_129(
        p: &Parameters,
        var_eefffactor: f64,
        var_eta_mu: f64,
        var_eta_mu_dn4: f64,
        var_eu_a: f64,
        var_eu_a_dn0: f64,
        var_eu_a_dn10: f64,
        var_eu_a_dn11: f64,
        var_eu_a_dn13: f64,
        var_eu_a_dn14: f64,
        var_eu_a_dn2: f64,
        var_eu_a_dn3: f64,
        var_eu_a_dn4: f64,
        var_eu_a_dn5: f64,
        var_eu_a_dn6: f64,
        var_eu_a_dn7: f64,
        var_eu_a_dn8: f64,
        var_eu_a_dn9: f64,
        var_guard632: f64,
        var_guard633: f64,
        var_guard634: f64,
        var_k2_t: f64,
        var_k2_t_dn4: f64,
        var_k2si_t: f64,
        var_k2si_t_dn4: f64,
        var_nvtm: f64,
        var_nvtm_dn0: f64,
        var_nvtm_dn10: f64,
        var_nvtm_dn11: f64,
        var_nvtm_dn13: f64,
        var_nvtm_dn14: f64,
        var_nvtm_dn2: f64,
        var_nvtm_dn3: f64,
        var_nvtm_dn4: f64,
        var_nvtm_dn5: f64,
        var_nvtm_dn6: f64,
        var_nvtm_dn7: f64,
        var_nvtm_dn8: f64,
        var_nvtm_dn9: f64,
        var_phibe_i: f64,
        var_prwgs_i: f64,
        var_q0: f64,
        var_q0_dn0: f64,
        var_q0_dn10: f64,
        var_q0_dn11: f64,
        var_q0_dn13: f64,
        var_q0_dn14: f64,
        var_q0_dn2: f64,
        var_q0_dn3: f64,
        var_q0_dn4: f64,
        var_q0_dn5: f64,
        var_q0_dn6: f64,
        var_q0_dn7: f64,
        var_q0_dn8: f64,
        var_q0_dn9: f64,
        var_qb0: f64,
        var_qba: f64,
        var_qba_dn0: f64,
        var_qba_dn10: f64,
        var_qba_dn11: f64,
        var_qba_dn13: f64,
        var_qba_dn14: f64,
        var_qba_dn2: f64,
        var_qba_dn3: f64,
        var_qba_dn4: f64,
        var_qba_dn5: f64,
        var_qba_dn6: f64,
        var_qba_dn7: f64,
        var_qba_dn8: f64,
        var_qba_dn9: f64,
        var_qis: f64,
        var_qis_dn0: f64,
        var_qis_dn10: f64,
        var_qis_dn11: f64,
        var_qis_dn13: f64,
        var_qis_dn14: f64,
        var_qis_dn2: f64,
        var_qis_dn3: f64,
        var_qis_dn4: f64,
        var_qis_dn5: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_qis_dn9: f64,
        var_ua_a: f64,
        var_ua_a_dn0: f64,
        var_ua_a_dn10: f64,
        var_ua_a_dn11: f64,
        var_ua_a_dn13: f64,
        var_ua_a_dn14: f64,
        var_ua_a_dn2: f64,
        var_ua_a_dn3: f64,
        var_ua_a_dn4: f64,
        var_ua_a_dn5: f64,
        var_ua_a_dn6: f64,
        var_ua_a_dn7: f64,
        var_ua_a_dn8: f64,
        var_ua_a_dn9: f64,
        var_uc_a: f64,
        var_uc_a_dn0: f64,
        var_uc_a_dn10: f64,
        var_uc_a_dn11: f64,
        var_uc_a_dn13: f64,
        var_uc_a_dn14: f64,
        var_uc_a_dn2: f64,
        var_uc_a_dn3: f64,
        var_uc_a_dn4: f64,
        var_uc_a_dn5: f64,
        var_uc_a_dn6: f64,
        var_uc_a_dn7: f64,
        var_uc_a_dn8: f64,
        var_uc_a_dn9: f64,
        var_ucs_t: f64,
        var_ucs_t_dn4: f64,
        var_ud_a: f64,
        var_ud_a_dn0: f64,
        var_ud_a_dn10: f64,
        var_ud_a_dn11: f64,
        var_ud_a_dn13: f64,
        var_ud_a_dn14: f64,
        var_ud_a_dn2: f64,
        var_ud_a_dn3: f64,
        var_ud_a_dn4: f64,
        var_ud_a_dn5: f64,
        var_ud_a_dn6: f64,
        var_ud_a_dn7: f64,
        var_ud_a_dn8: f64,
        var_ud_a_dn9: f64,
        var_veseff: f64,
        var_veseff_dn0: f64,
        var_veseff_dn10: f64,
        var_veseff_dn11: f64,
        var_veseff_dn13: f64,
        var_veseff_dn14: f64,
        var_veseff_dn2: f64,
        var_veseff_dn3: f64,
        var_veseff_dn4: f64,
        var_veseff_dn5: f64,
        var_veseff_dn6: f64,
        var_veseff_dn7: f64,
        var_veseff_dn8: f64,
        var_veseff_dn9: f64,
        var_dmob0_slot: &mut f64,
        var_dmob0_dn0_slot: &mut f64,
        var_dmob0_dn10_slot: &mut f64,
        var_dmob0_dn11_slot: &mut f64,
        var_dmob0_dn13_slot: &mut f64,
        var_dmob0_dn14_slot: &mut f64,
        var_dmob0_dn2_slot: &mut f64,
        var_dmob0_dn3_slot: &mut f64,
        var_dmob0_dn4_slot: &mut f64,
        var_dmob0_dn5_slot: &mut f64,
        var_dmob0_dn6_slot: &mut f64,
        var_dmob0_dn7_slot: &mut f64,
        var_dmob0_dn8_slot: &mut f64,
        var_dmob0_dn9_slot: &mut f64,
        var_dvsat0_slot: &mut f64,
        var_eeffm0_slot: &mut f64,
        var_eeffm0_dn0_slot: &mut f64,
        var_eeffm0_dn10_slot: &mut f64,
        var_eeffm0_dn11_slot: &mut f64,
        var_eeffm0_dn13_slot: &mut f64,
        var_eeffm0_dn14_slot: &mut f64,
        var_eeffm0_dn2_slot: &mut f64,
        var_eeffm0_dn3_slot: &mut f64,
        var_eeffm0_dn4_slot: &mut f64,
        var_eeffm0_dn5_slot: &mut f64,
        var_eeffm0_dn6_slot: &mut f64,
        var_eeffm0_dn7_slot: &mut f64,
        var_eeffm0_dn8_slot: &mut f64,
        var_eeffm0_dn9_slot: &mut f64,
        var_etaiv0_slot: &mut f64,
        var_etaiv0_dn0_slot: &mut f64,
        var_etaiv0_dn10_slot: &mut f64,
        var_etaiv0_dn11_slot: &mut f64,
        var_etaiv0_dn13_slot: &mut f64,
        var_etaiv0_dn14_slot: &mut f64,
        var_etaiv0_dn2_slot: &mut f64,
        var_etaiv0_dn3_slot: &mut f64,
        var_etaiv0_dn4_slot: &mut f64,
        var_etaiv0_dn5_slot: &mut f64,
        var_etaiv0_dn6_slot: &mut f64,
        var_etaiv0_dn7_slot: &mut f64,
        var_etaiv0_dn8_slot: &mut f64,
        var_etaiv0_dn9_slot: &mut f64,
        var_guard635_slot: &mut f64,
        var_guard636_slot: &mut f64,
        var_guard637_slot: &mut f64,
        var_guard638_slot: &mut f64,
        var_ids0_ov_dqi0_slot: &mut f64,
        var_ids0_ov_dqi0_dn0_slot: &mut f64,
        var_ids0_ov_dqi0_dn10_slot: &mut f64,
        var_ids0_ov_dqi0_dn11_slot: &mut f64,
        var_ids0_ov_dqi0_dn13_slot: &mut f64,
        var_ids0_ov_dqi0_dn14_slot: &mut f64,
        var_ids0_ov_dqi0_dn2_slot: &mut f64,
        var_ids0_ov_dqi0_dn3_slot: &mut f64,
        var_ids0_ov_dqi0_dn4_slot: &mut f64,
        var_ids0_ov_dqi0_dn5_slot: &mut f64,
        var_ids0_ov_dqi0_dn6_slot: &mut f64,
        var_ids0_ov_dqi0_dn7_slot: &mut f64,
        var_ids0_ov_dqi0_dn8_slot: &mut f64,
        var_ids0_ov_dqi0_dn9_slot: &mut f64,
        var_mob0_slot: &mut f64,
        var_mob0_dn0_slot: &mut f64,
        var_mob0_dn10_slot: &mut f64,
        var_mob0_dn11_slot: &mut f64,
        var_mob0_dn13_slot: &mut f64,
        var_mob0_dn14_slot: &mut f64,
        var_mob0_dn2_slot: &mut f64,
        var_mob0_dn3_slot: &mut f64,
        var_mob0_dn4_slot: &mut f64,
        var_mob0_dn5_slot: &mut f64,
        var_mob0_dn6_slot: &mut f64,
        var_mob0_dn7_slot: &mut f64,
        var_mob0_dn8_slot: &mut f64,
        var_mob0_dn9_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn13_slot: &mut f64,
        var_t4_dn14_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
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
        var_t5_dn13_slot: &mut f64,
        var_t5_dn14_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
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
        var_t6_dn13_slot: &mut f64,
        var_t6_dn14_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
    ) {
        let mut var_dmob0: f64 = *var_dmob0_slot;
        let mut var_dmob0_dn0: f64 = *var_dmob0_dn0_slot;
        let mut var_dmob0_dn10: f64 = *var_dmob0_dn10_slot;
        let mut var_dmob0_dn11: f64 = *var_dmob0_dn11_slot;
        let mut var_dmob0_dn13: f64 = *var_dmob0_dn13_slot;
        let mut var_dmob0_dn14: f64 = *var_dmob0_dn14_slot;
        let mut var_dmob0_dn2: f64 = *var_dmob0_dn2_slot;
        let mut var_dmob0_dn3: f64 = *var_dmob0_dn3_slot;
        let mut var_dmob0_dn4: f64 = *var_dmob0_dn4_slot;
        let mut var_dmob0_dn5: f64 = *var_dmob0_dn5_slot;
        let mut var_dmob0_dn6: f64 = *var_dmob0_dn6_slot;
        let mut var_dmob0_dn7: f64 = *var_dmob0_dn7_slot;
        let mut var_dmob0_dn8: f64 = *var_dmob0_dn8_slot;
        let mut var_dmob0_dn9: f64 = *var_dmob0_dn9_slot;
        let mut var_dvsat0: f64 = *var_dvsat0_slot;
        let mut var_eeffm0: f64 = *var_eeffm0_slot;
        let mut var_eeffm0_dn0: f64 = *var_eeffm0_dn0_slot;
        let mut var_eeffm0_dn10: f64 = *var_eeffm0_dn10_slot;
        let mut var_eeffm0_dn11: f64 = *var_eeffm0_dn11_slot;
        let mut var_eeffm0_dn13: f64 = *var_eeffm0_dn13_slot;
        let mut var_eeffm0_dn14: f64 = *var_eeffm0_dn14_slot;
        let mut var_eeffm0_dn2: f64 = *var_eeffm0_dn2_slot;
        let mut var_eeffm0_dn3: f64 = *var_eeffm0_dn3_slot;
        let mut var_eeffm0_dn4: f64 = *var_eeffm0_dn4_slot;
        let mut var_eeffm0_dn5: f64 = *var_eeffm0_dn5_slot;
        let mut var_eeffm0_dn6: f64 = *var_eeffm0_dn6_slot;
        let mut var_eeffm0_dn7: f64 = *var_eeffm0_dn7_slot;
        let mut var_eeffm0_dn8: f64 = *var_eeffm0_dn8_slot;
        let mut var_eeffm0_dn9: f64 = *var_eeffm0_dn9_slot;
        let mut var_etaiv0: f64 = *var_etaiv0_slot;
        let mut var_etaiv0_dn0: f64 = *var_etaiv0_dn0_slot;
        let mut var_etaiv0_dn10: f64 = *var_etaiv0_dn10_slot;
        let mut var_etaiv0_dn11: f64 = *var_etaiv0_dn11_slot;
        let mut var_etaiv0_dn13: f64 = *var_etaiv0_dn13_slot;
        let mut var_etaiv0_dn14: f64 = *var_etaiv0_dn14_slot;
        let mut var_etaiv0_dn2: f64 = *var_etaiv0_dn2_slot;
        let mut var_etaiv0_dn3: f64 = *var_etaiv0_dn3_slot;
        let mut var_etaiv0_dn4: f64 = *var_etaiv0_dn4_slot;
        let mut var_etaiv0_dn5: f64 = *var_etaiv0_dn5_slot;
        let mut var_etaiv0_dn6: f64 = *var_etaiv0_dn6_slot;
        let mut var_etaiv0_dn7: f64 = *var_etaiv0_dn7_slot;
        let mut var_etaiv0_dn8: f64 = *var_etaiv0_dn8_slot;
        let mut var_etaiv0_dn9: f64 = *var_etaiv0_dn9_slot;
        let mut var_guard635: f64 = *var_guard635_slot;
        let mut var_guard636: f64 = *var_guard636_slot;
        let mut var_guard637: f64 = *var_guard637_slot;
        let mut var_guard638: f64 = *var_guard638_slot;
        let mut var_ids0_ov_dqi0: f64 = *var_ids0_ov_dqi0_slot;
        let mut var_ids0_ov_dqi0_dn0: f64 = *var_ids0_ov_dqi0_dn0_slot;
        let mut var_ids0_ov_dqi0_dn10: f64 = *var_ids0_ov_dqi0_dn10_slot;
        let mut var_ids0_ov_dqi0_dn11: f64 = *var_ids0_ov_dqi0_dn11_slot;
        let mut var_ids0_ov_dqi0_dn13: f64 = *var_ids0_ov_dqi0_dn13_slot;
        let mut var_ids0_ov_dqi0_dn14: f64 = *var_ids0_ov_dqi0_dn14_slot;
        let mut var_ids0_ov_dqi0_dn2: f64 = *var_ids0_ov_dqi0_dn2_slot;
        let mut var_ids0_ov_dqi0_dn3: f64 = *var_ids0_ov_dqi0_dn3_slot;
        let mut var_ids0_ov_dqi0_dn4: f64 = *var_ids0_ov_dqi0_dn4_slot;
        let mut var_ids0_ov_dqi0_dn5: f64 = *var_ids0_ov_dqi0_dn5_slot;
        let mut var_ids0_ov_dqi0_dn6: f64 = *var_ids0_ov_dqi0_dn6_slot;
        let mut var_ids0_ov_dqi0_dn7: f64 = *var_ids0_ov_dqi0_dn7_slot;
        let mut var_ids0_ov_dqi0_dn8: f64 = *var_ids0_ov_dqi0_dn8_slot;
        let mut var_ids0_ov_dqi0_dn9: f64 = *var_ids0_ov_dqi0_dn9_slot;
        let mut var_mob0: f64 = *var_mob0_slot;
        let mut var_mob0_dn0: f64 = *var_mob0_dn0_slot;
        let mut var_mob0_dn10: f64 = *var_mob0_dn10_slot;
        let mut var_mob0_dn11: f64 = *var_mob0_dn11_slot;
        let mut var_mob0_dn13: f64 = *var_mob0_dn13_slot;
        let mut var_mob0_dn14: f64 = *var_mob0_dn14_slot;
        let mut var_mob0_dn2: f64 = *var_mob0_dn2_slot;
        let mut var_mob0_dn3: f64 = *var_mob0_dn3_slot;
        let mut var_mob0_dn4: f64 = *var_mob0_dn4_slot;
        let mut var_mob0_dn5: f64 = *var_mob0_dn5_slot;
        let mut var_mob0_dn6: f64 = *var_mob0_dn6_slot;
        let mut var_mob0_dn7: f64 = *var_mob0_dn7_slot;
        let mut var_mob0_dn8: f64 = *var_mob0_dn8_slot;
        let mut var_mob0_dn9: f64 = *var_mob0_dn9_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn14: f64 = *var_t4_dn14_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
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
        let mut var_t5_dn13: f64 = *var_t5_dn13_slot;
        let mut var_t5_dn14: f64 = *var_t5_dn14_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
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
        let mut var_t6_dn13: f64 = *var_t6_dn13_slot;
        let mut var_t6_dn14: f64 = *var_t6_dn14_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;

        let (assign33510_e56070, assign33510_e56070_d_n0, assign33510_e56070_d_n2, assign33510_e56070_d_n3, assign33510_e56070_d_n4, assign33510_e56070_d_n5, assign33510_e56070_d_n6, assign33510_e56070_d_n7, assign33510_e56070_d_n8, assign33510_e56070_d_n9, assign33510_e56070_d_n10, assign33510_e56070_d_n11, assign33510_e56070_d_n13, assign33510_e56070_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && (var_guard634 != 0.0)) {
        let assign33510_e56035: f64 = (-10000.0);
        let assign33510_e56037: f64 = (assign33510_e56035 * 1e-6);
        let (assign33510_e56068, assign33510_e56068_d_n4,) = {
            if (!(var_k2_t < assign33510_e56037)) {
                let assign33510_e56044: f64 = (var_k2_t * var_k2_t);
                let assign33510_e56047: f64 = (4.0 * 1e-6);
                let assign33510_e56049: f64 = (assign33510_e56047 * 1e-6);
                let assign33510_e56050: f64 = (assign33510_e56044 + assign33510_e56049);
                let assign33510_e56051: f64 = (assign33510_e56050).sqrt();
                let assign33510_e56052: f64 = (var_k2_t + assign33510_e56051);
                let assign33510_e56053: f64 = (0.5 * assign33510_e56052);
                (assign33510_e56053, (0.5 * (var_k2_t_dn4 + (((var_k2_t_dn4 * var_k2_t) + (var_k2_t * var_k2_t_dn4)) / (2.0 * assign33510_e56051)))),)
            } else {
                let assign33510_e56056: f64 = (-10000.0);
                let assign33510_e56058: f64 = (assign33510_e56056 * 1e-6);
                let (assign33510_e56067, assign33510_e56067_d_n4,) = {
                    if (var_k2_t < assign33510_e56058) {
                        let assign33510_e56061: f64 = (-1e-6);
                        let assign33510_e56063: f64 = (assign33510_e56061 * 1e-6);
                        let assign33510_e56065: f64 = (assign33510_e56063 / var_k2_t);
                        (assign33510_e56065, (-((assign33510_e56063 * var_k2_t_dn4) / (var_k2_t * var_k2_t))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign33510_e56067, assign33510_e56067_d_n4,)
            }
        };
        (assign33510_e56068, 0.0, 0.0, 0.0, assign33510_e56068_d_n4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11, var_t4_dn13, var_t4_dn14,)
    }
};
        var_t4 = assign33510_e56070;
        var_t4_dn0 = assign33510_e56070_d_n0;
        var_t4_dn2 = assign33510_e56070_d_n2;
        var_t4_dn3 = assign33510_e56070_d_n3;
        var_t4_dn4 = assign33510_e56070_d_n4;
        var_t4_dn5 = assign33510_e56070_d_n5;
        var_t4_dn6 = assign33510_e56070_d_n6;
        var_t4_dn7 = assign33510_e56070_d_n7;
        var_t4_dn8 = assign33510_e56070_d_n8;
        var_t4_dn9 = assign33510_e56070_d_n9;
        var_t4_dn10 = assign33510_e56070_d_n10;
        var_t4_dn11 = assign33510_e56070_d_n11;
        var_t4_dn13 = assign33510_e56070_d_n13;
        var_t4_dn14 = assign33510_e56070_d_n14;

        let (assign33520_e56089, assign33520_e56089_d_n0, assign33520_e56089_d_n2, assign33520_e56089_d_n3, assign33520_e56089_d_n4, assign33520_e56089_d_n5, assign33520_e56089_d_n6, assign33520_e56089_d_n7, assign33520_e56089_d_n8, assign33520_e56089_d_n9, assign33520_e56089_d_n10, assign33520_e56089_d_n11, assign33520_e56089_d_n13, assign33520_e56089_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && (var_guard634 != 0.0)) {
        let assign33520_e56080: f64 = (0.0_f64).max(var_k2si_t);
        let assign33520_e56082: f64 = (assign33520_e56080 * var_qis);
        let assign33520_e56085: f64 = (2.0 * var_nvtm);
        let assign33520_e56086: f64 = (assign33520_e56082 + assign33520_e56085);
        let assign33520_e56087: f64 = (var_t4 / assign33520_e56086);
        (assign33520_e56087, (((var_t4_dn0 * assign33520_e56086) - (var_t4 * ((assign33520_e56080 * var_qis_dn0) + (2.0 * var_nvtm_dn0)))) / (assign33520_e56086 * assign33520_e56086)), (((var_t4_dn2 * assign33520_e56086) - (var_t4 * ((assign33520_e56080 * var_qis_dn2) + (2.0 * var_nvtm_dn2)))) / (assign33520_e56086 * assign33520_e56086)), (((var_t4_dn3 * assign33520_e56086) - (var_t4 * ((assign33520_e56080 * var_qis_dn3) + (2.0 * var_nvtm_dn3)))) / (assign33520_e56086 * assign33520_e56086)), (((var_t4_dn4 * assign33520_e56086) - (var_t4 * (((if 0.0 >= var_k2si_t { 0.0 } else { var_k2si_t_dn4 } * var_qis) + (assign33520_e56080 * var_qis_dn4)) + (2.0 * var_nvtm_dn4)))) / (assign33520_e56086 * assign33520_e56086)), (((var_t4_dn5 * assign33520_e56086) - (var_t4 * ((assign33520_e56080 * var_qis_dn5) + (2.0 * var_nvtm_dn5)))) / (assign33520_e56086 * assign33520_e56086)), (((var_t4_dn6 * assign33520_e56086) - (var_t4 * ((assign33520_e56080 * var_qis_dn6) + (2.0 * var_nvtm_dn6)))) / (assign33520_e56086 * assign33520_e56086)), (((var_t4_dn7 * assign33520_e56086) - (var_t4 * ((assign33520_e56080 * var_qis_dn7) + (2.0 * var_nvtm_dn7)))) / (assign33520_e56086 * assign33520_e56086)), (((var_t4_dn8 * assign33520_e56086) - (var_t4 * ((assign33520_e56080 * var_qis_dn8) + (2.0 * var_nvtm_dn8)))) / (assign33520_e56086 * assign33520_e56086)), (((var_t4_dn9 * assign33520_e56086) - (var_t4 * ((assign33520_e56080 * var_qis_dn9) + (2.0 * var_nvtm_dn9)))) / (assign33520_e56086 * assign33520_e56086)), (((var_t4_dn10 * assign33520_e56086) - (var_t4 * ((assign33520_e56080 * var_qis_dn10) + (2.0 * var_nvtm_dn10)))) / (assign33520_e56086 * assign33520_e56086)), (((var_t4_dn11 * assign33520_e56086) - (var_t4 * ((assign33520_e56080 * var_qis_dn11) + (2.0 * var_nvtm_dn11)))) / (assign33520_e56086 * assign33520_e56086)), (((var_t4_dn13 * assign33520_e56086) - (var_t4 * ((assign33520_e56080 * var_qis_dn13) + (2.0 * var_nvtm_dn13)))) / (assign33520_e56086 * assign33520_e56086)), (((var_t4_dn14 * assign33520_e56086) - (var_t4 * ((assign33520_e56080 * var_qis_dn14) + (2.0 * var_nvtm_dn14)))) / (assign33520_e56086 * assign33520_e56086)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11, var_t5_dn13, var_t5_dn14,)
    }
};
        var_t5 = assign33520_e56089;
        var_t5_dn0 = assign33520_e56089_d_n0;
        var_t5_dn2 = assign33520_e56089_d_n2;
        var_t5_dn3 = assign33520_e56089_d_n3;
        var_t5_dn4 = assign33520_e56089_d_n4;
        var_t5_dn5 = assign33520_e56089_d_n5;
        var_t5_dn6 = assign33520_e56089_d_n6;
        var_t5_dn7 = assign33520_e56089_d_n7;
        var_t5_dn8 = assign33520_e56089_d_n8;
        var_t5_dn9 = assign33520_e56089_d_n9;
        var_t5_dn10 = assign33520_e56089_d_n10;
        var_t5_dn11 = assign33520_e56089_d_n11;
        var_t5_dn13 = assign33520_e56089_d_n13;
        var_t5_dn14 = assign33520_e56089_d_n14;

        let (assign33530_e56104, assign33530_e56104_d_n0, assign33530_e56104_d_n2, assign33530_e56104_d_n3, assign33530_e56104_d_n4, assign33530_e56104_d_n5, assign33530_e56104_d_n6, assign33530_e56104_d_n7, assign33530_e56104_d_n8, assign33530_e56104_d_n9, assign33530_e56104_d_n10, assign33530_e56104_d_n11, assign33530_e56104_d_n13, assign33530_e56104_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && (var_guard634 != 0.0)) {
        let assign33530_e56098: f64 = (var_phibe_i - var_veseff);
        let assign33530_e56099: f64 = (assign33530_e56098).sqrt();
        let assign33530_e56101: f64 = (var_phibe_i).sqrt();
        let assign33530_e56102: f64 = (assign33530_e56099 - assign33530_e56101);
        (assign33530_e56102, ((-var_veseff_dn0) / (2.0 * assign33530_e56099)), ((-var_veseff_dn2) / (2.0 * assign33530_e56099)), ((-var_veseff_dn3) / (2.0 * assign33530_e56099)), ((-var_veseff_dn4) / (2.0 * assign33530_e56099)), ((-var_veseff_dn5) / (2.0 * assign33530_e56099)), ((-var_veseff_dn6) / (2.0 * assign33530_e56099)), ((-var_veseff_dn7) / (2.0 * assign33530_e56099)), ((-var_veseff_dn8) / (2.0 * assign33530_e56099)), ((-var_veseff_dn9) / (2.0 * assign33530_e56099)), ((-var_veseff_dn10) / (2.0 * assign33530_e56099)), ((-var_veseff_dn11) / (2.0 * assign33530_e56099)), ((-var_veseff_dn13) / (2.0 * assign33530_e56099)), ((-var_veseff_dn14) / (2.0 * assign33530_e56099)),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn13, var_t6_dn14,)
    }
};
        var_t6 = assign33530_e56104;
        var_t6_dn0 = assign33530_e56104_d_n0;
        var_t6_dn2 = assign33530_e56104_d_n2;
        var_t6_dn3 = assign33530_e56104_d_n3;
        var_t6_dn4 = assign33530_e56104_d_n4;
        var_t6_dn5 = assign33530_e56104_d_n5;
        var_t6_dn6 = assign33530_e56104_d_n6;
        var_t6_dn7 = assign33530_e56104_d_n7;
        var_t6_dn8 = assign33530_e56104_d_n8;
        var_t6_dn9 = assign33530_e56104_d_n9;
        var_t6_dn10 = assign33530_e56104_d_n10;
        var_t6_dn11 = assign33530_e56104_d_n11;
        var_t6_dn13 = assign33530_e56104_d_n13;
        var_t6_dn14 = assign33530_e56104_d_n14;

        let (assign33540_e56117, assign33540_e56117_d_n0, assign33540_e56117_d_n2, assign33540_e56117_d_n3, assign33540_e56117_d_n4, assign33540_e56117_d_n5, assign33540_e56117_d_n6, assign33540_e56117_d_n7, assign33540_e56117_d_n8, assign33540_e56117_d_n9, assign33540_e56117_d_n10, assign33540_e56117_d_n11, assign33540_e56117_d_n13, assign33540_e56117_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && (var_guard634 != 0.0)) {
        let assign33540_e56112: f64 = (-var_t5);
        let assign33540_e56114: f64 = (assign33540_e56112 * var_t6);
        let assign33540_e56115: f64 = { let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign33540_e56115, ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-var_t5_dn0) * var_t6) + (assign33540_e56112 * var_t6_dn0))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-var_t5_dn2) * var_t6) + (assign33540_e56112 * var_t6_dn2))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-var_t5_dn3) * var_t6) + (assign33540_e56112 * var_t6_dn3))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-var_t5_dn4) * var_t6) + (assign33540_e56112 * var_t6_dn4))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-var_t5_dn5) * var_t6) + (assign33540_e56112 * var_t6_dn5))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-var_t5_dn6) * var_t6) + (assign33540_e56112 * var_t6_dn6))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-var_t5_dn7) * var_t6) + (assign33540_e56112 * var_t6_dn7))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-var_t5_dn8) * var_t6) + (assign33540_e56112 * var_t6_dn8))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-var_t5_dn9) * var_t6) + (assign33540_e56112 * var_t6_dn9))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-var_t5_dn10) * var_t6) + (assign33540_e56112 * var_t6_dn10))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-var_t5_dn11) * var_t6) + (assign33540_e56112 * var_t6_dn11))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-var_t5_dn13) * var_t6) + (assign33540_e56112 * var_t6_dn13))), ({ let limited_exp_arg = assign33540_e56114; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((-var_t5_dn14) * var_t6) + (assign33540_e56112 * var_t6_dn14))),)
    } else {
        (var_mob0, var_mob0_dn0, var_mob0_dn2, var_mob0_dn3, var_mob0_dn4, var_mob0_dn5, var_mob0_dn6, var_mob0_dn7, var_mob0_dn8, var_mob0_dn9, var_mob0_dn10, var_mob0_dn11, var_mob0_dn13, var_mob0_dn14,)
    }
};
        var_mob0 = assign33540_e56117;
        var_mob0_dn0 = assign33540_e56117_d_n0;
        var_mob0_dn2 = assign33540_e56117_d_n2;
        var_mob0_dn3 = assign33540_e56117_d_n3;
        var_mob0_dn4 = assign33540_e56117_d_n4;
        var_mob0_dn5 = assign33540_e56117_d_n5;
        var_mob0_dn6 = assign33540_e56117_d_n6;
        var_mob0_dn7 = assign33540_e56117_d_n7;
        var_mob0_dn8 = assign33540_e56117_d_n8;
        var_mob0_dn9 = assign33540_e56117_d_n9;
        var_mob0_dn10 = assign33540_e56117_d_n10;
        var_mob0_dn11 = assign33540_e56117_d_n11;
        var_mob0_dn13 = assign33540_e56117_d_n13;
        var_mob0_dn14 = assign33540_e56117_d_n14;

        let (assign33550_e56127, assign33550_e56127_d_n0, assign33550_e56127_d_n2, assign33550_e56127_d_n3, assign33550_e56127_d_n4, assign33550_e56127_d_n5, assign33550_e56127_d_n6, assign33550_e56127_d_n7, assign33550_e56127_d_n8, assign33550_e56127_d_n9, assign33550_e56127_d_n10, assign33550_e56127_d_n11, assign33550_e56127_d_n13, assign33550_e56127_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && (var_guard634 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mob0, var_mob0_dn0, var_mob0_dn2, var_mob0_dn3, var_mob0_dn4, var_mob0_dn5, var_mob0_dn6, var_mob0_dn7, var_mob0_dn8, var_mob0_dn9, var_mob0_dn10, var_mob0_dn11, var_mob0_dn13, var_mob0_dn14,)
    }
};
        var_mob0 = assign33550_e56127;
        var_mob0_dn0 = assign33550_e56127_d_n0;
        var_mob0_dn2 = assign33550_e56127_d_n2;
        var_mob0_dn3 = assign33550_e56127_d_n3;
        var_mob0_dn4 = assign33550_e56127_d_n4;
        var_mob0_dn5 = assign33550_e56127_d_n5;
        var_mob0_dn6 = assign33550_e56127_d_n6;
        var_mob0_dn7 = assign33550_e56127_d_n7;
        var_mob0_dn8 = assign33550_e56127_d_n8;
        var_mob0_dn9 = assign33550_e56127_d_n9;
        var_mob0_dn10 = assign33550_e56127_d_n10;
        var_mob0_dn11 = assign33550_e56127_d_n11;
        var_mob0_dn13 = assign33550_e56127_d_n13;
        var_mob0_dn14 = assign33550_e56127_d_n14;

        let (assign33560_e56140, assign33560_e56140_d_n0, assign33560_e56140_d_n2, assign33560_e56140_d_n3, assign33560_e56140_d_n4, assign33560_e56140_d_n5, assign33560_e56140_d_n6, assign33560_e56140_d_n7, assign33560_e56140_d_n8, assign33560_e56140_d_n9, assign33560_e56140_d_n10, assign33560_e56140_d_n11, assign33560_e56140_d_n13, assign33560_e56140_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33560_e56136: f64 = (var_eta_mu * var_qis);
        let assign33560_e56137: f64 = (var_qba + assign33560_e56136);
        let assign33560_e56138: f64 = (var_eefffactor * assign33560_e56137);
        (assign33560_e56138, (var_eefffactor * (var_qba_dn0 + (var_eta_mu * var_qis_dn0))), (var_eefffactor * (var_qba_dn2 + (var_eta_mu * var_qis_dn2))), (var_eefffactor * (var_qba_dn3 + (var_eta_mu * var_qis_dn3))), (var_eefffactor * (var_qba_dn4 + ((var_eta_mu_dn4 * var_qis) + (var_eta_mu * var_qis_dn4)))), (var_eefffactor * (var_qba_dn5 + (var_eta_mu * var_qis_dn5))), (var_eefffactor * (var_qba_dn6 + (var_eta_mu * var_qis_dn6))), (var_eefffactor * (var_qba_dn7 + (var_eta_mu * var_qis_dn7))), (var_eefffactor * (var_qba_dn8 + (var_eta_mu * var_qis_dn8))), (var_eefffactor * (var_qba_dn9 + (var_eta_mu * var_qis_dn9))), (var_eefffactor * (var_qba_dn10 + (var_eta_mu * var_qis_dn10))), (var_eefffactor * (var_qba_dn11 + (var_eta_mu * var_qis_dn11))), (var_eefffactor * (var_qba_dn13 + (var_eta_mu * var_qis_dn13))), (var_eefffactor * (var_qba_dn14 + (var_eta_mu * var_qis_dn14))),)
    } else {
        (var_eeffm0, var_eeffm0_dn0, var_eeffm0_dn2, var_eeffm0_dn3, var_eeffm0_dn4, var_eeffm0_dn5, var_eeffm0_dn6, var_eeffm0_dn7, var_eeffm0_dn8, var_eeffm0_dn9, var_eeffm0_dn10, var_eeffm0_dn11, var_eeffm0_dn13, var_eeffm0_dn14,)
    }
};
        var_eeffm0 = assign33560_e56140;
        var_eeffm0_dn0 = assign33560_e56140_d_n0;
        var_eeffm0_dn2 = assign33560_e56140_d_n2;
        var_eeffm0_dn3 = assign33560_e56140_d_n3;
        var_eeffm0_dn4 = assign33560_e56140_d_n4;
        var_eeffm0_dn5 = assign33560_e56140_d_n5;
        var_eeffm0_dn6 = assign33560_e56140_d_n6;
        var_eeffm0_dn7 = assign33560_e56140_d_n7;
        var_eeffm0_dn8 = assign33560_e56140_d_n8;
        var_eeffm0_dn9 = assign33560_e56140_d_n9;
        var_eeffm0_dn10 = assign33560_e56140_d_n10;
        var_eeffm0_dn11 = assign33560_e56140_d_n11;
        var_eeffm0_dn13 = assign33560_e56140_d_n13;
        var_eeffm0_dn14 = assign33560_e56140_d_n14;

        let (assign33570_e56156, assign33570_e56156_d_n0, assign33570_e56156_d_n2, assign33570_e56156_d_n3, assign33570_e56156_d_n4, assign33570_e56156_d_n5, assign33570_e56156_d_n6, assign33570_e56156_d_n7, assign33570_e56156_d_n8, assign33570_e56156_d_n9, assign33570_e56156_d_n10, assign33570_e56156_d_n11, assign33570_e56156_d_n13, assign33570_e56156_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33570_e56149: f64 = (var_qis / var_qb0);
        let assign33570_e56150: f64 = (assign33570_e56149).abs();
        let assign33570_e56151: f64 = (1.0 + assign33570_e56150);
        let assign33570_e56152: f64 = (0.5 * assign33570_e56151);
        let assign33570_e56154: f64 = (assign33570_e56152).powf(var_ucs_t);
        (assign33570_e56154, if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign33570_e56152).powf(var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn0 / var_qb0) } else { (-(var_qis_dn0 / var_qb0)) }))) } } else { (assign33570_e56154 * (var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn0 / var_qb0) } else { (-(var_qis_dn0 / var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign33570_e56152).powf(var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn2 / var_qb0) } else { (-(var_qis_dn2 / var_qb0)) }))) } } else { (assign33570_e56154 * (var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn2 / var_qb0) } else { (-(var_qis_dn2 / var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign33570_e56152).powf(var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn3 / var_qb0) } else { (-(var_qis_dn3 / var_qb0)) }))) } } else { (assign33570_e56154 * (var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn3 / var_qb0) } else { (-(var_qis_dn3 / var_qb0)) }) / assign33570_e56152))) }, if var_ucs_t_dn4 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign33570_e56152).powf(var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn4 / var_qb0) } else { (-(var_qis_dn4 / var_qb0)) }))) } } else { (assign33570_e56154 * ((var_ucs_t_dn4 * (assign33570_e56152).ln()) + (var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn4 / var_qb0) } else { (-(var_qis_dn4 / var_qb0)) }) / assign33570_e56152)))) }, if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign33570_e56152).powf(var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn5 / var_qb0) } else { (-(var_qis_dn5 / var_qb0)) }))) } } else { (assign33570_e56154 * (var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn5 / var_qb0) } else { (-(var_qis_dn5 / var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign33570_e56152).powf(var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn6 / var_qb0) } else { (-(var_qis_dn6 / var_qb0)) }))) } } else { (assign33570_e56154 * (var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn6 / var_qb0) } else { (-(var_qis_dn6 / var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign33570_e56152).powf(var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn7 / var_qb0) } else { (-(var_qis_dn7 / var_qb0)) }))) } } else { (assign33570_e56154 * (var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn7 / var_qb0) } else { (-(var_qis_dn7 / var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign33570_e56152).powf(var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn8 / var_qb0) } else { (-(var_qis_dn8 / var_qb0)) }))) } } else { (assign33570_e56154 * (var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn8 / var_qb0) } else { (-(var_qis_dn8 / var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign33570_e56152).powf(var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn9 / var_qb0) } else { (-(var_qis_dn9 / var_qb0)) }))) } } else { (assign33570_e56154 * (var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn9 / var_qb0) } else { (-(var_qis_dn9 / var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign33570_e56152).powf(var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn10 / var_qb0) } else { (-(var_qis_dn10 / var_qb0)) }))) } } else { (assign33570_e56154 * (var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn10 / var_qb0) } else { (-(var_qis_dn10 / var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign33570_e56152).powf(var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn11 / var_qb0) } else { (-(var_qis_dn11 / var_qb0)) }))) } } else { (assign33570_e56154 * (var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn11 / var_qb0) } else { (-(var_qis_dn11 / var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign33570_e56152).powf(var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn13 / var_qb0) } else { (-(var_qis_dn13 / var_qb0)) }))) } } else { (assign33570_e56154 * (var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn13 / var_qb0) } else { (-(var_qis_dn13 / var_qb0)) }) / assign33570_e56152))) }, if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign33570_e56152).powf(var_ucs_t - 1.0) * (0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn14 / var_qb0) } else { (-(var_qis_dn14 / var_qb0)) }))) } } else { (assign33570_e56154 * (var_ucs_t * ((0.5 * if assign33570_e56149 >= 0.0 { (var_qis_dn14 / var_qb0) } else { (-(var_qis_dn14 / var_qb0)) }) / assign33570_e56152))) },)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11, var_t4_dn13, var_t4_dn14,)
    }
};
        var_t4 = assign33570_e56156;
        var_t4_dn0 = assign33570_e56156_d_n0;
        var_t4_dn2 = assign33570_e56156_d_n2;
        var_t4_dn3 = assign33570_e56156_d_n3;
        var_t4_dn4 = assign33570_e56156_d_n4;
        var_t4_dn5 = assign33570_e56156_d_n5;
        var_t4_dn6 = assign33570_e56156_d_n6;
        var_t4_dn7 = assign33570_e56156_d_n7;
        var_t4_dn8 = assign33570_e56156_d_n8;
        var_t4_dn9 = assign33570_e56156_d_n9;
        var_t4_dn10 = assign33570_e56156_d_n10;
        var_t4_dn11 = assign33570_e56156_d_n11;
        var_t4_dn13 = assign33570_e56156_d_n13;
        var_t4_dn14 = assign33570_e56156_d_n14;

        let assign33580_e56159: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard635 = assign33580_e56159;

        let (assign33590_e56181, assign33590_e56181_d_n0, assign33590_e56181_d_n2, assign33590_e56181_d_n3, assign33590_e56181_d_n4, assign33590_e56181_d_n5, assign33590_e56181_d_n6, assign33590_e56181_d_n7, assign33590_e56181_d_n8, assign33590_e56181_d_n9, assign33590_e56181_d_n10, assign33590_e56181_d_n11, assign33590_e56181_d_n13, assign33590_e56181_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && (var_guard635 != 0.0)) {
        let assign33590_e56169: f64 = (var_uc_a * var_veseff);
        let assign33590_e56170: f64 = (var_ua_a + assign33590_e56169);
        let assign33590_e56172: f64 = (var_eeffm0).abs();
        let assign33590_e56174: f64 = (assign33590_e56172).powf(var_eu_a);
        let assign33590_e56175: f64 = (assign33590_e56170 * assign33590_e56174);
        let assign33590_e56178: f64 = (var_ud_a / var_t4);
        let assign33590_e56179: f64 = (assign33590_e56175 + assign33590_e56178);
        (assign33590_e56179, ((((var_ua_a_dn0 + ((var_uc_a_dn0 * var_veseff) + (var_uc_a * var_veseff_dn0))) * assign33590_e56174) + (assign33590_e56170 * if var_eu_a_dn0 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33590_e56172).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn0 } else { (-var_eeffm0_dn0) })) } } else { (assign33590_e56174 * ((var_eu_a_dn0 * (assign33590_e56172).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn0 } else { (-var_eeffm0_dn0) } / assign33590_e56172)))) })) + (((var_ud_a_dn0 * var_t4) - (var_ud_a * var_t4_dn0)) / (var_t4 * var_t4))), ((((var_ua_a_dn2 + ((var_uc_a_dn2 * var_veseff) + (var_uc_a * var_veseff_dn2))) * assign33590_e56174) + (assign33590_e56170 * if var_eu_a_dn2 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33590_e56172).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn2 } else { (-var_eeffm0_dn2) })) } } else { (assign33590_e56174 * ((var_eu_a_dn2 * (assign33590_e56172).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn2 } else { (-var_eeffm0_dn2) } / assign33590_e56172)))) })) + (((var_ud_a_dn2 * var_t4) - (var_ud_a * var_t4_dn2)) / (var_t4 * var_t4))), ((((var_ua_a_dn3 + ((var_uc_a_dn3 * var_veseff) + (var_uc_a * var_veseff_dn3))) * assign33590_e56174) + (assign33590_e56170 * if var_eu_a_dn3 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33590_e56172).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn3 } else { (-var_eeffm0_dn3) })) } } else { (assign33590_e56174 * ((var_eu_a_dn3 * (assign33590_e56172).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn3 } else { (-var_eeffm0_dn3) } / assign33590_e56172)))) })) + (((var_ud_a_dn3 * var_t4) - (var_ud_a * var_t4_dn3)) / (var_t4 * var_t4))), ((((var_ua_a_dn4 + ((var_uc_a_dn4 * var_veseff) + (var_uc_a * var_veseff_dn4))) * assign33590_e56174) + (assign33590_e56170 * if var_eu_a_dn4 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33590_e56172).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn4 } else { (-var_eeffm0_dn4) })) } } else { (assign33590_e56174 * ((var_eu_a_dn4 * (assign33590_e56172).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn4 } else { (-var_eeffm0_dn4) } / assign33590_e56172)))) })) + (((var_ud_a_dn4 * var_t4) - (var_ud_a * var_t4_dn4)) / (var_t4 * var_t4))), ((((var_ua_a_dn5 + ((var_uc_a_dn5 * var_veseff) + (var_uc_a * var_veseff_dn5))) * assign33590_e56174) + (assign33590_e56170 * if var_eu_a_dn5 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33590_e56172).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn5 } else { (-var_eeffm0_dn5) })) } } else { (assign33590_e56174 * ((var_eu_a_dn5 * (assign33590_e56172).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn5 } else { (-var_eeffm0_dn5) } / assign33590_e56172)))) })) + (((var_ud_a_dn5 * var_t4) - (var_ud_a * var_t4_dn5)) / (var_t4 * var_t4))), ((((var_ua_a_dn6 + ((var_uc_a_dn6 * var_veseff) + (var_uc_a * var_veseff_dn6))) * assign33590_e56174) + (assign33590_e56170 * if var_eu_a_dn6 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33590_e56172).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn6 } else { (-var_eeffm0_dn6) })) } } else { (assign33590_e56174 * ((var_eu_a_dn6 * (assign33590_e56172).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn6 } else { (-var_eeffm0_dn6) } / assign33590_e56172)))) })) + (((var_ud_a_dn6 * var_t4) - (var_ud_a * var_t4_dn6)) / (var_t4 * var_t4))), ((((var_ua_a_dn7 + ((var_uc_a_dn7 * var_veseff) + (var_uc_a * var_veseff_dn7))) * assign33590_e56174) + (assign33590_e56170 * if var_eu_a_dn7 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33590_e56172).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn7 } else { (-var_eeffm0_dn7) })) } } else { (assign33590_e56174 * ((var_eu_a_dn7 * (assign33590_e56172).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn7 } else { (-var_eeffm0_dn7) } / assign33590_e56172)))) })) + (((var_ud_a_dn7 * var_t4) - (var_ud_a * var_t4_dn7)) / (var_t4 * var_t4))), ((((var_ua_a_dn8 + ((var_uc_a_dn8 * var_veseff) + (var_uc_a * var_veseff_dn8))) * assign33590_e56174) + (assign33590_e56170 * if var_eu_a_dn8 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33590_e56172).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn8 } else { (-var_eeffm0_dn8) })) } } else { (assign33590_e56174 * ((var_eu_a_dn8 * (assign33590_e56172).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn8 } else { (-var_eeffm0_dn8) } / assign33590_e56172)))) })) + (((var_ud_a_dn8 * var_t4) - (var_ud_a * var_t4_dn8)) / (var_t4 * var_t4))), ((((var_ua_a_dn9 + ((var_uc_a_dn9 * var_veseff) + (var_uc_a * var_veseff_dn9))) * assign33590_e56174) + (assign33590_e56170 * if var_eu_a_dn9 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33590_e56172).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn9 } else { (-var_eeffm0_dn9) })) } } else { (assign33590_e56174 * ((var_eu_a_dn9 * (assign33590_e56172).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn9 } else { (-var_eeffm0_dn9) } / assign33590_e56172)))) })) + (((var_ud_a_dn9 * var_t4) - (var_ud_a * var_t4_dn9)) / (var_t4 * var_t4))), ((((var_ua_a_dn10 + ((var_uc_a_dn10 * var_veseff) + (var_uc_a * var_veseff_dn10))) * assign33590_e56174) + (assign33590_e56170 * if var_eu_a_dn10 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33590_e56172).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn10 } else { (-var_eeffm0_dn10) })) } } else { (assign33590_e56174 * ((var_eu_a_dn10 * (assign33590_e56172).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn10 } else { (-var_eeffm0_dn10) } / assign33590_e56172)))) })) + (((var_ud_a_dn10 * var_t4) - (var_ud_a * var_t4_dn10)) / (var_t4 * var_t4))), ((((var_ua_a_dn11 + ((var_uc_a_dn11 * var_veseff) + (var_uc_a * var_veseff_dn11))) * assign33590_e56174) + (assign33590_e56170 * if var_eu_a_dn11 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33590_e56172).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn11 } else { (-var_eeffm0_dn11) })) } } else { (assign33590_e56174 * ((var_eu_a_dn11 * (assign33590_e56172).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn11 } else { (-var_eeffm0_dn11) } / assign33590_e56172)))) })) + (((var_ud_a_dn11 * var_t4) - (var_ud_a * var_t4_dn11)) / (var_t4 * var_t4))), ((((var_ua_a_dn13 + ((var_uc_a_dn13 * var_veseff) + (var_uc_a * var_veseff_dn13))) * assign33590_e56174) + (assign33590_e56170 * if var_eu_a_dn13 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33590_e56172).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn13 } else { (-var_eeffm0_dn13) })) } } else { (assign33590_e56174 * ((var_eu_a_dn13 * (assign33590_e56172).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn13 } else { (-var_eeffm0_dn13) } / assign33590_e56172)))) })) + (((var_ud_a_dn13 * var_t4) - (var_ud_a * var_t4_dn13)) / (var_t4 * var_t4))), ((((var_ua_a_dn14 + ((var_uc_a_dn14 * var_veseff) + (var_uc_a * var_veseff_dn14))) * assign33590_e56174) + (assign33590_e56170 * if var_eu_a_dn14 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33590_e56172).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn14 } else { (-var_eeffm0_dn14) })) } } else { (assign33590_e56174 * ((var_eu_a_dn14 * (assign33590_e56172).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn14 } else { (-var_eeffm0_dn14) } / assign33590_e56172)))) })) + (((var_ud_a_dn14 * var_t4) - (var_ud_a * var_t4_dn14)) / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11, var_t5_dn13, var_t5_dn14,)
    }
};
        var_t5 = assign33590_e56181;
        var_t5_dn0 = assign33590_e56181_d_n0;
        var_t5_dn2 = assign33590_e56181_d_n2;
        var_t5_dn3 = assign33590_e56181_d_n3;
        var_t5_dn4 = assign33590_e56181_d_n4;
        var_t5_dn5 = assign33590_e56181_d_n5;
        var_t5_dn6 = assign33590_e56181_d_n6;
        var_t5_dn7 = assign33590_e56181_d_n7;
        var_t5_dn8 = assign33590_e56181_d_n8;
        var_t5_dn9 = assign33590_e56181_d_n9;
        var_t5_dn10 = assign33590_e56181_d_n10;
        var_t5_dn11 = assign33590_e56181_d_n11;
        var_t5_dn13 = assign33590_e56181_d_n13;
        var_t5_dn14 = assign33590_e56181_d_n14;

        let (assign33600_e56200, assign33600_e56200_d_n0, assign33600_e56200_d_n2, assign33600_e56200_d_n3, assign33600_e56200_d_n4, assign33600_e56200_d_n5, assign33600_e56200_d_n6, assign33600_e56200_d_n7, assign33600_e56200_d_n8, assign33600_e56200_d_n9, assign33600_e56200_d_n10, assign33600_e56200_d_n11, assign33600_e56200_d_n13, assign33600_e56200_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && (var_guard635 == 0.0)) {
        let assign33600_e56191: f64 = (var_eeffm0).abs();
        let assign33600_e56193: f64 = (assign33600_e56191).powf(var_eu_a);
        let assign33600_e56194: f64 = (var_ua_a * assign33600_e56193);
        let assign33600_e56197: f64 = (var_ud_a / var_t4);
        let assign33600_e56198: f64 = (assign33600_e56194 + assign33600_e56197);
        (assign33600_e56198, (((var_ua_a_dn0 * assign33600_e56193) + (var_ua_a * if var_eu_a_dn0 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33600_e56191).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn0 } else { (-var_eeffm0_dn0) })) } } else { (assign33600_e56193 * ((var_eu_a_dn0 * (assign33600_e56191).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn0 } else { (-var_eeffm0_dn0) } / assign33600_e56191)))) })) + (((var_ud_a_dn0 * var_t4) - (var_ud_a * var_t4_dn0)) / (var_t4 * var_t4))), (((var_ua_a_dn2 * assign33600_e56193) + (var_ua_a * if var_eu_a_dn2 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33600_e56191).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn2 } else { (-var_eeffm0_dn2) })) } } else { (assign33600_e56193 * ((var_eu_a_dn2 * (assign33600_e56191).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn2 } else { (-var_eeffm0_dn2) } / assign33600_e56191)))) })) + (((var_ud_a_dn2 * var_t4) - (var_ud_a * var_t4_dn2)) / (var_t4 * var_t4))), (((var_ua_a_dn3 * assign33600_e56193) + (var_ua_a * if var_eu_a_dn3 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33600_e56191).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn3 } else { (-var_eeffm0_dn3) })) } } else { (assign33600_e56193 * ((var_eu_a_dn3 * (assign33600_e56191).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn3 } else { (-var_eeffm0_dn3) } / assign33600_e56191)))) })) + (((var_ud_a_dn3 * var_t4) - (var_ud_a * var_t4_dn3)) / (var_t4 * var_t4))), (((var_ua_a_dn4 * assign33600_e56193) + (var_ua_a * if var_eu_a_dn4 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33600_e56191).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn4 } else { (-var_eeffm0_dn4) })) } } else { (assign33600_e56193 * ((var_eu_a_dn4 * (assign33600_e56191).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn4 } else { (-var_eeffm0_dn4) } / assign33600_e56191)))) })) + (((var_ud_a_dn4 * var_t4) - (var_ud_a * var_t4_dn4)) / (var_t4 * var_t4))), (((var_ua_a_dn5 * assign33600_e56193) + (var_ua_a * if var_eu_a_dn5 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33600_e56191).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn5 } else { (-var_eeffm0_dn5) })) } } else { (assign33600_e56193 * ((var_eu_a_dn5 * (assign33600_e56191).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn5 } else { (-var_eeffm0_dn5) } / assign33600_e56191)))) })) + (((var_ud_a_dn5 * var_t4) - (var_ud_a * var_t4_dn5)) / (var_t4 * var_t4))), (((var_ua_a_dn6 * assign33600_e56193) + (var_ua_a * if var_eu_a_dn6 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33600_e56191).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn6 } else { (-var_eeffm0_dn6) })) } } else { (assign33600_e56193 * ((var_eu_a_dn6 * (assign33600_e56191).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn6 } else { (-var_eeffm0_dn6) } / assign33600_e56191)))) })) + (((var_ud_a_dn6 * var_t4) - (var_ud_a * var_t4_dn6)) / (var_t4 * var_t4))), (((var_ua_a_dn7 * assign33600_e56193) + (var_ua_a * if var_eu_a_dn7 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33600_e56191).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn7 } else { (-var_eeffm0_dn7) })) } } else { (assign33600_e56193 * ((var_eu_a_dn7 * (assign33600_e56191).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn7 } else { (-var_eeffm0_dn7) } / assign33600_e56191)))) })) + (((var_ud_a_dn7 * var_t4) - (var_ud_a * var_t4_dn7)) / (var_t4 * var_t4))), (((var_ua_a_dn8 * assign33600_e56193) + (var_ua_a * if var_eu_a_dn8 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33600_e56191).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn8 } else { (-var_eeffm0_dn8) })) } } else { (assign33600_e56193 * ((var_eu_a_dn8 * (assign33600_e56191).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn8 } else { (-var_eeffm0_dn8) } / assign33600_e56191)))) })) + (((var_ud_a_dn8 * var_t4) - (var_ud_a * var_t4_dn8)) / (var_t4 * var_t4))), (((var_ua_a_dn9 * assign33600_e56193) + (var_ua_a * if var_eu_a_dn9 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33600_e56191).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn9 } else { (-var_eeffm0_dn9) })) } } else { (assign33600_e56193 * ((var_eu_a_dn9 * (assign33600_e56191).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn9 } else { (-var_eeffm0_dn9) } / assign33600_e56191)))) })) + (((var_ud_a_dn9 * var_t4) - (var_ud_a * var_t4_dn9)) / (var_t4 * var_t4))), (((var_ua_a_dn10 * assign33600_e56193) + (var_ua_a * if var_eu_a_dn10 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33600_e56191).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn10 } else { (-var_eeffm0_dn10) })) } } else { (assign33600_e56193 * ((var_eu_a_dn10 * (assign33600_e56191).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn10 } else { (-var_eeffm0_dn10) } / assign33600_e56191)))) })) + (((var_ud_a_dn10 * var_t4) - (var_ud_a * var_t4_dn10)) / (var_t4 * var_t4))), (((var_ua_a_dn11 * assign33600_e56193) + (var_ua_a * if var_eu_a_dn11 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33600_e56191).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn11 } else { (-var_eeffm0_dn11) })) } } else { (assign33600_e56193 * ((var_eu_a_dn11 * (assign33600_e56191).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn11 } else { (-var_eeffm0_dn11) } / assign33600_e56191)))) })) + (((var_ud_a_dn11 * var_t4) - (var_ud_a * var_t4_dn11)) / (var_t4 * var_t4))), (((var_ua_a_dn13 * assign33600_e56193) + (var_ua_a * if var_eu_a_dn13 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33600_e56191).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn13 } else { (-var_eeffm0_dn13) })) } } else { (assign33600_e56193 * ((var_eu_a_dn13 * (assign33600_e56191).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn13 } else { (-var_eeffm0_dn13) } / assign33600_e56191)))) })) + (((var_ud_a_dn13 * var_t4) - (var_ud_a * var_t4_dn13)) / (var_t4 * var_t4))), (((var_ua_a_dn14 * assign33600_e56193) + (var_ua_a * if var_eu_a_dn14 == 0.0 && ((var_eu_a) as f64).is_finite() && ((var_eu_a) as f64).fract() == 0.0 { if var_eu_a == 0.0 { 0.0 } else { (var_eu_a * ((assign33600_e56191).powf(var_eu_a - 1.0) * if var_eeffm0 >= 0.0 { var_eeffm0_dn14 } else { (-var_eeffm0_dn14) })) } } else { (assign33600_e56193 * ((var_eu_a_dn14 * (assign33600_e56191).ln()) + (var_eu_a * (if var_eeffm0 >= 0.0 { var_eeffm0_dn14 } else { (-var_eeffm0_dn14) } / assign33600_e56191)))) })) + (((var_ud_a_dn14 * var_t4) - (var_ud_a * var_t4_dn14)) / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11, var_t5_dn13, var_t5_dn14,)
    }
};
        var_t5 = assign33600_e56200;
        var_t5_dn0 = assign33600_e56200_d_n0;
        var_t5_dn2 = assign33600_e56200_d_n2;
        var_t5_dn3 = assign33600_e56200_d_n3;
        var_t5_dn4 = assign33600_e56200_d_n4;
        var_t5_dn5 = assign33600_e56200_d_n5;
        var_t5_dn6 = assign33600_e56200_d_n6;
        var_t5_dn7 = assign33600_e56200_d_n7;
        var_t5_dn8 = assign33600_e56200_d_n8;
        var_t5_dn9 = assign33600_e56200_d_n9;
        var_t5_dn10 = assign33600_e56200_d_n10;
        var_t5_dn11 = assign33600_e56200_d_n11;
        var_t5_dn13 = assign33600_e56200_d_n13;
        var_t5_dn14 = assign33600_e56200_d_n14;

        let (assign33610_e56209, assign33610_e56209_d_n0, assign33610_e56209_d_n2, assign33610_e56209_d_n3, assign33610_e56209_d_n4, assign33610_e56209_d_n5, assign33610_e56209_d_n6, assign33610_e56209_d_n7, assign33610_e56209_d_n8, assign33610_e56209_d_n9, assign33610_e56209_d_n10, assign33610_e56209_d_n11, assign33610_e56209_d_n13, assign33610_e56209_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33610_e56207: f64 = (1.0 + var_t5);
        (assign33610_e56207, var_t5_dn0, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11, var_t5_dn13, var_t5_dn14,)
    } else {
        (var_dmob0, var_dmob0_dn0, var_dmob0_dn2, var_dmob0_dn3, var_dmob0_dn4, var_dmob0_dn5, var_dmob0_dn6, var_dmob0_dn7, var_dmob0_dn8, var_dmob0_dn9, var_dmob0_dn10, var_dmob0_dn11, var_dmob0_dn13, var_dmob0_dn14,)
    }
};
        var_dmob0 = assign33610_e56209;
        var_dmob0_dn0 = assign33610_e56209_d_n0;
        var_dmob0_dn2 = assign33610_e56209_d_n2;
        var_dmob0_dn3 = assign33610_e56209_d_n3;
        var_dmob0_dn4 = assign33610_e56209_d_n4;
        var_dmob0_dn5 = assign33610_e56209_d_n5;
        var_dmob0_dn6 = assign33610_e56209_d_n6;
        var_dmob0_dn7 = assign33610_e56209_d_n7;
        var_dmob0_dn8 = assign33610_e56209_d_n8;
        var_dmob0_dn9 = assign33610_e56209_d_n9;
        var_dmob0_dn10 = assign33610_e56209_d_n10;
        var_dmob0_dn11 = assign33610_e56209_d_n11;
        var_dmob0_dn13 = assign33610_e56209_d_n13;
        var_dmob0_dn14 = assign33610_e56209_d_n14;

        let (assign33620_e56235, assign33620_e56235_d_n0, assign33620_e56235_d_n2, assign33620_e56235_d_n3, assign33620_e56235_d_n4, assign33620_e56235_d_n5, assign33620_e56235_d_n6, assign33620_e56235_d_n7, assign33620_e56235_d_n8, assign33620_e56235_d_n9, assign33620_e56235_d_n10, assign33620_e56235_d_n11, assign33620_e56235_d_n13, assign33620_e56235_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33620_e56217: f64 = (var_dmob0 + 1.0);
        let assign33620_e56220: f64 = (var_dmob0 - 1.0);
        let assign33620_e56223: f64 = (var_dmob0 - 1.0);
        let assign33620_e56224: f64 = (assign33620_e56220 * assign33620_e56223);
        let assign33620_e56227: f64 = (0.25 * p.p604);
        let assign33620_e56229: f64 = (assign33620_e56227 * p.p604);
        let assign33620_e56230: f64 = (assign33620_e56224 + assign33620_e56229);
        let assign33620_e56231: f64 = (assign33620_e56230).sqrt();
        let assign33620_e56232: f64 = (assign33620_e56217 + assign33620_e56231);
        let assign33620_e56233: f64 = (0.5 * assign33620_e56232);
        (assign33620_e56233, (0.5 * (var_dmob0_dn0 + (((var_dmob0_dn0 * assign33620_e56223) + (assign33620_e56220 * var_dmob0_dn0)) / (2.0 * assign33620_e56231)))), (0.5 * (var_dmob0_dn2 + (((var_dmob0_dn2 * assign33620_e56223) + (assign33620_e56220 * var_dmob0_dn2)) / (2.0 * assign33620_e56231)))), (0.5 * (var_dmob0_dn3 + (((var_dmob0_dn3 * assign33620_e56223) + (assign33620_e56220 * var_dmob0_dn3)) / (2.0 * assign33620_e56231)))), (0.5 * (var_dmob0_dn4 + (((var_dmob0_dn4 * assign33620_e56223) + (assign33620_e56220 * var_dmob0_dn4)) / (2.0 * assign33620_e56231)))), (0.5 * (var_dmob0_dn5 + (((var_dmob0_dn5 * assign33620_e56223) + (assign33620_e56220 * var_dmob0_dn5)) / (2.0 * assign33620_e56231)))), (0.5 * (var_dmob0_dn6 + (((var_dmob0_dn6 * assign33620_e56223) + (assign33620_e56220 * var_dmob0_dn6)) / (2.0 * assign33620_e56231)))), (0.5 * (var_dmob0_dn7 + (((var_dmob0_dn7 * assign33620_e56223) + (assign33620_e56220 * var_dmob0_dn7)) / (2.0 * assign33620_e56231)))), (0.5 * (var_dmob0_dn8 + (((var_dmob0_dn8 * assign33620_e56223) + (assign33620_e56220 * var_dmob0_dn8)) / (2.0 * assign33620_e56231)))), (0.5 * (var_dmob0_dn9 + (((var_dmob0_dn9 * assign33620_e56223) + (assign33620_e56220 * var_dmob0_dn9)) / (2.0 * assign33620_e56231)))), (0.5 * (var_dmob0_dn10 + (((var_dmob0_dn10 * assign33620_e56223) + (assign33620_e56220 * var_dmob0_dn10)) / (2.0 * assign33620_e56231)))), (0.5 * (var_dmob0_dn11 + (((var_dmob0_dn11 * assign33620_e56223) + (assign33620_e56220 * var_dmob0_dn11)) / (2.0 * assign33620_e56231)))), (0.5 * (var_dmob0_dn13 + (((var_dmob0_dn13 * assign33620_e56223) + (assign33620_e56220 * var_dmob0_dn13)) / (2.0 * assign33620_e56231)))), (0.5 * (var_dmob0_dn14 + (((var_dmob0_dn14 * assign33620_e56223) + (assign33620_e56220 * var_dmob0_dn14)) / (2.0 * assign33620_e56231)))),)
    } else {
        (var_dmob0, var_dmob0_dn0, var_dmob0_dn2, var_dmob0_dn3, var_dmob0_dn4, var_dmob0_dn5, var_dmob0_dn6, var_dmob0_dn7, var_dmob0_dn8, var_dmob0_dn9, var_dmob0_dn10, var_dmob0_dn11, var_dmob0_dn13, var_dmob0_dn14,)
    }
};
        var_dmob0 = assign33620_e56235;
        var_dmob0_dn0 = assign33620_e56235_d_n0;
        var_dmob0_dn2 = assign33620_e56235_d_n2;
        var_dmob0_dn3 = assign33620_e56235_d_n3;
        var_dmob0_dn4 = assign33620_e56235_d_n4;
        var_dmob0_dn5 = assign33620_e56235_d_n5;
        var_dmob0_dn6 = assign33620_e56235_d_n6;
        var_dmob0_dn7 = assign33620_e56235_d_n7;
        var_dmob0_dn8 = assign33620_e56235_d_n8;
        var_dmob0_dn9 = assign33620_e56235_d_n9;
        var_dmob0_dn10 = assign33620_e56235_d_n10;
        var_dmob0_dn11 = assign33620_e56235_d_n11;
        var_dmob0_dn13 = assign33620_e56235_d_n13;
        var_dmob0_dn14 = assign33620_e56235_d_n14;

        let (assign33630_e56244, assign33630_e56244_d_n0, assign33630_e56244_d_n2, assign33630_e56244_d_n3, assign33630_e56244_d_n4, assign33630_e56244_d_n5, assign33630_e56244_d_n6, assign33630_e56244_d_n7, assign33630_e56244_d_n8, assign33630_e56244_d_n9, assign33630_e56244_d_n10, assign33630_e56244_d_n11, assign33630_e56244_d_n13, assign33630_e56244_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33630_e56242: f64 = (var_dmob0 / p.p24);
        (assign33630_e56242, (var_dmob0_dn0 / p.p24), (var_dmob0_dn2 / p.p24), (var_dmob0_dn3 / p.p24), (var_dmob0_dn4 / p.p24), (var_dmob0_dn5 / p.p24), (var_dmob0_dn6 / p.p24), (var_dmob0_dn7 / p.p24), (var_dmob0_dn8 / p.p24), (var_dmob0_dn9 / p.p24), (var_dmob0_dn10 / p.p24), (var_dmob0_dn11 / p.p24), (var_dmob0_dn13 / p.p24), (var_dmob0_dn14 / p.p24),)
    } else {
        (var_dmob0, var_dmob0_dn0, var_dmob0_dn2, var_dmob0_dn3, var_dmob0_dn4, var_dmob0_dn5, var_dmob0_dn6, var_dmob0_dn7, var_dmob0_dn8, var_dmob0_dn9, var_dmob0_dn10, var_dmob0_dn11, var_dmob0_dn13, var_dmob0_dn14,)
    }
};
        var_dmob0 = assign33630_e56244;
        var_dmob0_dn0 = assign33630_e56244_d_n0;
        var_dmob0_dn2 = assign33630_e56244_d_n2;
        var_dmob0_dn3 = assign33630_e56244_d_n3;
        var_dmob0_dn4 = assign33630_e56244_d_n4;
        var_dmob0_dn5 = assign33630_e56244_d_n5;
        var_dmob0_dn6 = assign33630_e56244_d_n6;
        var_dmob0_dn7 = assign33630_e56244_d_n7;
        var_dmob0_dn8 = assign33630_e56244_d_n8;
        var_dmob0_dn9 = assign33630_e56244_d_n9;
        var_dmob0_dn10 = assign33630_e56244_d_n10;
        var_dmob0_dn11 = assign33630_e56244_d_n11;
        var_dmob0_dn13 = assign33630_e56244_d_n13;
        var_dmob0_dn14 = assign33630_e56244_d_n14;

        let (assign33640_e56255,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33640_e56252: f64 = (0.25 * p.p453);
        let assign33640_e56253: f64 = (1.0 + assign33640_e56252);
        (assign33640_e56253,)
    } else {
        (var_dvsat0,)
    }
};
        var_dvsat0 = assign33640_e56255;

        let (assign33650_e56266, assign33650_e56266_d_n0, assign33650_e56266_d_n2, assign33650_e56266_d_n3, assign33650_e56266_d_n4, assign33650_e56266_d_n5, assign33650_e56266_d_n6, assign33650_e56266_d_n7, assign33650_e56266_d_n8, assign33650_e56266_d_n9, assign33650_e56266_d_n10, assign33650_e56266_d_n11, assign33650_e56266_d_n13, assign33650_e56266_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33650_e56263: f64 = (var_q0 + var_qis);
        let assign33650_e56264: f64 = (var_q0 / assign33650_e56263);
        (assign33650_e56264, (((var_q0_dn0 * assign33650_e56263) - (var_q0 * (var_q0_dn0 + var_qis_dn0))) / (assign33650_e56263 * assign33650_e56263)), (((var_q0_dn2 * assign33650_e56263) - (var_q0 * (var_q0_dn2 + var_qis_dn2))) / (assign33650_e56263 * assign33650_e56263)), (((var_q0_dn3 * assign33650_e56263) - (var_q0 * (var_q0_dn3 + var_qis_dn3))) / (assign33650_e56263 * assign33650_e56263)), (((var_q0_dn4 * assign33650_e56263) - (var_q0 * (var_q0_dn4 + var_qis_dn4))) / (assign33650_e56263 * assign33650_e56263)), (((var_q0_dn5 * assign33650_e56263) - (var_q0 * (var_q0_dn5 + var_qis_dn5))) / (assign33650_e56263 * assign33650_e56263)), (((var_q0_dn6 * assign33650_e56263) - (var_q0 * (var_q0_dn6 + var_qis_dn6))) / (assign33650_e56263 * assign33650_e56263)), (((var_q0_dn7 * assign33650_e56263) - (var_q0 * (var_q0_dn7 + var_qis_dn7))) / (assign33650_e56263 * assign33650_e56263)), (((var_q0_dn8 * assign33650_e56263) - (var_q0 * (var_q0_dn8 + var_qis_dn8))) / (assign33650_e56263 * assign33650_e56263)), (((var_q0_dn9 * assign33650_e56263) - (var_q0 * (var_q0_dn9 + var_qis_dn9))) / (assign33650_e56263 * assign33650_e56263)), (((var_q0_dn10 * assign33650_e56263) - (var_q0 * (var_q0_dn10 + var_qis_dn10))) / (assign33650_e56263 * assign33650_e56263)), (((var_q0_dn11 * assign33650_e56263) - (var_q0 * (var_q0_dn11 + var_qis_dn11))) / (assign33650_e56263 * assign33650_e56263)), (((var_q0_dn13 * assign33650_e56263) - (var_q0 * (var_q0_dn13 + var_qis_dn13))) / (assign33650_e56263 * assign33650_e56263)), (((var_q0_dn14 * assign33650_e56263) - (var_q0 * (var_q0_dn14 + var_qis_dn14))) / (assign33650_e56263 * assign33650_e56263)),)
    } else {
        (var_etaiv0, var_etaiv0_dn0, var_etaiv0_dn2, var_etaiv0_dn3, var_etaiv0_dn4, var_etaiv0_dn5, var_etaiv0_dn6, var_etaiv0_dn7, var_etaiv0_dn8, var_etaiv0_dn9, var_etaiv0_dn10, var_etaiv0_dn11, var_etaiv0_dn13, var_etaiv0_dn14,)
    }
};
        var_etaiv0 = assign33650_e56266;
        var_etaiv0_dn0 = assign33650_e56266_d_n0;
        var_etaiv0_dn2 = assign33650_e56266_d_n2;
        var_etaiv0_dn3 = assign33650_e56266_d_n3;
        var_etaiv0_dn4 = assign33650_e56266_d_n4;
        var_etaiv0_dn5 = assign33650_e56266_d_n5;
        var_etaiv0_dn6 = assign33650_e56266_d_n6;
        var_etaiv0_dn7 = assign33650_e56266_d_n7;
        var_etaiv0_dn8 = assign33650_e56266_d_n8;
        var_etaiv0_dn9 = assign33650_e56266_d_n9;
        var_etaiv0_dn10 = assign33650_e56266_d_n10;
        var_etaiv0_dn11 = assign33650_e56266_d_n11;
        var_etaiv0_dn13 = assign33650_e56266_d_n13;
        var_etaiv0_dn14 = assign33650_e56266_d_n14;

        let (assign33660_e56277, assign33660_e56277_d_n0, assign33660_e56277_d_n2, assign33660_e56277_d_n3, assign33660_e56277_d_n4, assign33660_e56277_d_n5, assign33660_e56277_d_n6, assign33660_e56277_d_n7, assign33660_e56277_d_n8, assign33660_e56277_d_n9, assign33660_e56277_d_n10, assign33660_e56277_d_n11, assign33660_e56277_d_n13, assign33660_e56277_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33660_e56273: f64 = (2.0 - var_etaiv0);
        let assign33660_e56275: f64 = (assign33660_e56273 * var_nvtm);
        (assign33660_e56275, (((-var_etaiv0_dn0) * var_nvtm) + (assign33660_e56273 * var_nvtm_dn0)), (((-var_etaiv0_dn2) * var_nvtm) + (assign33660_e56273 * var_nvtm_dn2)), (((-var_etaiv0_dn3) * var_nvtm) + (assign33660_e56273 * var_nvtm_dn3)), (((-var_etaiv0_dn4) * var_nvtm) + (assign33660_e56273 * var_nvtm_dn4)), (((-var_etaiv0_dn5) * var_nvtm) + (assign33660_e56273 * var_nvtm_dn5)), (((-var_etaiv0_dn6) * var_nvtm) + (assign33660_e56273 * var_nvtm_dn6)), (((-var_etaiv0_dn7) * var_nvtm) + (assign33660_e56273 * var_nvtm_dn7)), (((-var_etaiv0_dn8) * var_nvtm) + (assign33660_e56273 * var_nvtm_dn8)), (((-var_etaiv0_dn9) * var_nvtm) + (assign33660_e56273 * var_nvtm_dn9)), (((-var_etaiv0_dn10) * var_nvtm) + (assign33660_e56273 * var_nvtm_dn10)), (((-var_etaiv0_dn11) * var_nvtm) + (assign33660_e56273 * var_nvtm_dn11)), (((-var_etaiv0_dn13) * var_nvtm) + (assign33660_e56273 * var_nvtm_dn13)), (((-var_etaiv0_dn14) * var_nvtm) + (assign33660_e56273 * var_nvtm_dn14)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11, var_t4_dn13, var_t4_dn14,)
    }
};
        var_t4 = assign33660_e56277;
        var_t4_dn0 = assign33660_e56277_d_n0;
        var_t4_dn2 = assign33660_e56277_d_n2;
        var_t4_dn3 = assign33660_e56277_d_n3;
        var_t4_dn4 = assign33660_e56277_d_n4;
        var_t4_dn5 = assign33660_e56277_d_n5;
        var_t4_dn6 = assign33660_e56277_d_n6;
        var_t4_dn7 = assign33660_e56277_d_n7;
        var_t4_dn8 = assign33660_e56277_d_n8;
        var_t4_dn9 = assign33660_e56277_d_n9;
        var_t4_dn10 = assign33660_e56277_d_n10;
        var_t4_dn11 = assign33660_e56277_d_n11;
        var_t4_dn13 = assign33660_e56277_d_n13;
        var_t4_dn14 = assign33660_e56277_d_n14;

        let (assign33670_e56286, assign33670_e56286_d_n0, assign33670_e56286_d_n2, assign33670_e56286_d_n3, assign33670_e56286_d_n4, assign33670_e56286_d_n5, assign33670_e56286_d_n6, assign33670_e56286_d_n7, assign33670_e56286_d_n8, assign33670_e56286_d_n9, assign33670_e56286_d_n10, assign33670_e56286_d_n11, assign33670_e56286_d_n13, assign33670_e56286_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33670_e56284: f64 = (var_qis + var_t4);
        (assign33670_e56284, (var_qis_dn0 + var_t4_dn0), (var_qis_dn2 + var_t4_dn2), (var_qis_dn3 + var_t4_dn3), (var_qis_dn4 + var_t4_dn4), (var_qis_dn5 + var_t4_dn5), (var_qis_dn6 + var_t4_dn6), (var_qis_dn7 + var_t4_dn7), (var_qis_dn8 + var_t4_dn8), (var_qis_dn9 + var_t4_dn9), (var_qis_dn10 + var_t4_dn10), (var_qis_dn11 + var_t4_dn11), (var_qis_dn13 + var_t4_dn13), (var_qis_dn14 + var_t4_dn14),)
    } else {
        (var_ids0_ov_dqi0, var_ids0_ov_dqi0_dn0, var_ids0_ov_dqi0_dn2, var_ids0_ov_dqi0_dn3, var_ids0_ov_dqi0_dn4, var_ids0_ov_dqi0_dn5, var_ids0_ov_dqi0_dn6, var_ids0_ov_dqi0_dn7, var_ids0_ov_dqi0_dn8, var_ids0_ov_dqi0_dn9, var_ids0_ov_dqi0_dn10, var_ids0_ov_dqi0_dn11, var_ids0_ov_dqi0_dn13, var_ids0_ov_dqi0_dn14,)
    }
};
        var_ids0_ov_dqi0 = assign33670_e56286;
        var_ids0_ov_dqi0_dn0 = assign33670_e56286_d_n0;
        var_ids0_ov_dqi0_dn2 = assign33670_e56286_d_n2;
        var_ids0_ov_dqi0_dn3 = assign33670_e56286_d_n3;
        var_ids0_ov_dqi0_dn4 = assign33670_e56286_d_n4;
        var_ids0_ov_dqi0_dn5 = assign33670_e56286_d_n5;
        var_ids0_ov_dqi0_dn6 = assign33670_e56286_d_n6;
        var_ids0_ov_dqi0_dn7 = assign33670_e56286_d_n7;
        var_ids0_ov_dqi0_dn8 = assign33670_e56286_d_n8;
        var_ids0_ov_dqi0_dn9 = assign33670_e56286_d_n9;
        var_ids0_ov_dqi0_dn10 = assign33670_e56286_d_n10;
        var_ids0_ov_dqi0_dn11 = assign33670_e56286_d_n11;
        var_ids0_ov_dqi0_dn13 = assign33670_e56286_d_n13;
        var_ids0_ov_dqi0_dn14 = assign33670_e56286_d_n14;

        let assign33680_e56289: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        var_guard636 = assign33680_e56289;

        let assign33690_e56292: f64 = if p.p64 == 1.0 { 1.0 } else { 0.0 };
        var_guard637 = assign33690_e56292;

        let assign33700_e56295: f64 = if p.p64 == 2.0 { 1.0 } else { 0.0 };
        var_guard638 = assign33700_e56295;

        let (assign33710_e56308, assign33710_e56308_d_n0, assign33710_e56308_d_n2, assign33710_e56308_d_n3, assign33710_e56308_d_n4, assign33710_e56308_d_n5, assign33710_e56308_d_n6, assign33710_e56308_d_n7, assign33710_e56308_d_n8, assign33710_e56308_d_n9, assign33710_e56308_d_n10, assign33710_e56308_d_n11, assign33710_e56308_d_n13, assign33710_e56308_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && (var_guard636 != 0.0)) {
        let assign33710_e56305: f64 = (var_prwgs_i * var_qis);
        let assign33710_e56306: f64 = (1.0 + assign33710_e56305);
        (assign33710_e56306, (var_prwgs_i * var_qis_dn0), (var_prwgs_i * var_qis_dn2), (var_prwgs_i * var_qis_dn3), (var_prwgs_i * var_qis_dn4), (var_prwgs_i * var_qis_dn5), (var_prwgs_i * var_qis_dn6), (var_prwgs_i * var_qis_dn7), (var_prwgs_i * var_qis_dn8), (var_prwgs_i * var_qis_dn9), (var_prwgs_i * var_qis_dn10), (var_prwgs_i * var_qis_dn11), (var_prwgs_i * var_qis_dn13), (var_prwgs_i * var_qis_dn14),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11, var_t4_dn13, var_t4_dn14,)
    }
};
        var_t4 = assign33710_e56308;
        var_t4_dn0 = assign33710_e56308_d_n0;
        var_t4_dn2 = assign33710_e56308_d_n2;
        var_t4_dn3 = assign33710_e56308_d_n3;
        var_t4_dn4 = assign33710_e56308_d_n4;
        var_t4_dn5 = assign33710_e56308_d_n5;
        var_t4_dn6 = assign33710_e56308_d_n6;
        var_t4_dn7 = assign33710_e56308_d_n7;
        var_t4_dn8 = assign33710_e56308_d_n8;
        var_t4_dn9 = assign33710_e56308_d_n9;
        var_t4_dn10 = assign33710_e56308_d_n10;
        var_t4_dn11 = assign33710_e56308_d_n11;
        var_t4_dn13 = assign33710_e56308_d_n13;
        var_t4_dn14 = assign33710_e56308_d_n14;

        let (assign33720_e56319, assign33720_e56319_d_n0, assign33720_e56319_d_n2, assign33720_e56319_d_n3, assign33720_e56319_d_n4, assign33720_e56319_d_n5, assign33720_e56319_d_n6, assign33720_e56319_d_n7, assign33720_e56319_d_n8, assign33720_e56319_d_n9, assign33720_e56319_d_n10, assign33720_e56319_d_n11, assign33720_e56319_d_n13, assign33720_e56319_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && (var_guard636 != 0.0)) {
        let assign33720_e56317: f64 = (1.0 / var_t4);
        (assign33720_e56317, (-(var_t4_dn0 / (var_t4 * var_t4))), (-(var_t4_dn2 / (var_t4 * var_t4))), (-(var_t4_dn3 / (var_t4 * var_t4))), (-(var_t4_dn4 / (var_t4 * var_t4))), (-(var_t4_dn5 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn7 / (var_t4 * var_t4))), (-(var_t4_dn8 / (var_t4 * var_t4))), (-(var_t4_dn9 / (var_t4 * var_t4))), (-(var_t4_dn10 / (var_t4 * var_t4))), (-(var_t4_dn11 / (var_t4 * var_t4))), (-(var_t4_dn13 / (var_t4 * var_t4))), (-(var_t4_dn14 / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11, var_t5_dn13, var_t5_dn14,)
    }
};
        var_t5 = assign33720_e56319;
        var_t5_dn0 = assign33720_e56319_d_n0;
        var_t5_dn2 = assign33720_e56319_d_n2;
        var_t5_dn3 = assign33720_e56319_d_n3;
        var_t5_dn4 = assign33720_e56319_d_n4;
        var_t5_dn5 = assign33720_e56319_d_n5;
        var_t5_dn6 = assign33720_e56319_d_n6;
        var_t5_dn7 = assign33720_e56319_d_n7;
        var_t5_dn8 = assign33720_e56319_d_n8;
        var_t5_dn9 = assign33720_e56319_d_n9;
        var_t5_dn10 = assign33720_e56319_d_n10;
        var_t5_dn11 = assign33720_e56319_d_n11;
        var_t5_dn13 = assign33720_e56319_d_n13;
        var_t5_dn14 = assign33720_e56319_d_n14;

        let (assign33730_e56337, assign33730_e56337_d_n0, assign33730_e56337_d_n2, assign33730_e56337_d_n3, assign33730_e56337_d_n4, assign33730_e56337_d_n5, assign33730_e56337_d_n6, assign33730_e56337_d_n7, assign33730_e56337_d_n8, assign33730_e56337_d_n9, assign33730_e56337_d_n10, assign33730_e56337_d_n11, assign33730_e56337_d_n13, assign33730_e56337_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && (var_guard636 != 0.0)) {
        let assign33730_e56330: f64 = (var_t5 * var_t5);
        let assign33730_e56332: f64 = (assign33730_e56330 + 0.01);
        let assign33730_e56333: f64 = (assign33730_e56332).sqrt();
        let assign33730_e56334: f64 = (var_t5 + assign33730_e56333);
        let assign33730_e56335: f64 = (0.5 * assign33730_e56334);
        (assign33730_e56335, (0.5 * (var_t5_dn0 + (((var_t5_dn0 * var_t5) + (var_t5 * var_t5_dn0)) / (2.0 * assign33730_e56333)))), (0.5 * (var_t5_dn2 + (((var_t5_dn2 * var_t5) + (var_t5 * var_t5_dn2)) / (2.0 * assign33730_e56333)))), (0.5 * (var_t5_dn3 + (((var_t5_dn3 * var_t5) + (var_t5 * var_t5_dn3)) / (2.0 * assign33730_e56333)))), (0.5 * (var_t5_dn4 + (((var_t5_dn4 * var_t5) + (var_t5 * var_t5_dn4)) / (2.0 * assign33730_e56333)))), (0.5 * (var_t5_dn5 + (((var_t5_dn5 * var_t5) + (var_t5 * var_t5_dn5)) / (2.0 * assign33730_e56333)))), (0.5 * (var_t5_dn6 + (((var_t5_dn6 * var_t5) + (var_t5 * var_t5_dn6)) / (2.0 * assign33730_e56333)))), (0.5 * (var_t5_dn7 + (((var_t5_dn7 * var_t5) + (var_t5 * var_t5_dn7)) / (2.0 * assign33730_e56333)))), (0.5 * (var_t5_dn8 + (((var_t5_dn8 * var_t5) + (var_t5 * var_t5_dn8)) / (2.0 * assign33730_e56333)))), (0.5 * (var_t5_dn9 + (((var_t5_dn9 * var_t5) + (var_t5 * var_t5_dn9)) / (2.0 * assign33730_e56333)))), (0.5 * (var_t5_dn10 + (((var_t5_dn10 * var_t5) + (var_t5 * var_t5_dn10)) / (2.0 * assign33730_e56333)))), (0.5 * (var_t5_dn11 + (((var_t5_dn11 * var_t5) + (var_t5 * var_t5_dn11)) / (2.0 * assign33730_e56333)))), (0.5 * (var_t5_dn13 + (((var_t5_dn13 * var_t5) + (var_t5 * var_t5_dn13)) / (2.0 * assign33730_e56333)))), (0.5 * (var_t5_dn14 + (((var_t5_dn14 * var_t5) + (var_t5 * var_t5_dn14)) / (2.0 * assign33730_e56333)))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn13, var_t6_dn14,)
    }
};
        var_t6 = assign33730_e56337;
        var_t6_dn0 = assign33730_e56337_d_n0;
        var_t6_dn2 = assign33730_e56337_d_n2;
        var_t6_dn3 = assign33730_e56337_d_n3;
        var_t6_dn4 = assign33730_e56337_d_n4;
        var_t6_dn5 = assign33730_e56337_d_n5;
        var_t6_dn6 = assign33730_e56337_d_n6;
        var_t6_dn7 = assign33730_e56337_d_n7;
        var_t6_dn8 = assign33730_e56337_d_n8;
        var_t6_dn9 = assign33730_e56337_d_n9;
        var_t6_dn10 = assign33730_e56337_d_n10;
        var_t6_dn11 = assign33730_e56337_d_n11;
        var_t6_dn13 = assign33730_e56337_d_n13;
        var_t6_dn14 = assign33730_e56337_d_n14;

        *var_dmob0_slot = var_dmob0;
        *var_dmob0_dn0_slot = var_dmob0_dn0;
        *var_dmob0_dn10_slot = var_dmob0_dn10;
        *var_dmob0_dn11_slot = var_dmob0_dn11;
        *var_dmob0_dn13_slot = var_dmob0_dn13;
        *var_dmob0_dn14_slot = var_dmob0_dn14;
        *var_dmob0_dn2_slot = var_dmob0_dn2;
        *var_dmob0_dn3_slot = var_dmob0_dn3;
        *var_dmob0_dn4_slot = var_dmob0_dn4;
        *var_dmob0_dn5_slot = var_dmob0_dn5;
        *var_dmob0_dn6_slot = var_dmob0_dn6;
        *var_dmob0_dn7_slot = var_dmob0_dn7;
        *var_dmob0_dn8_slot = var_dmob0_dn8;
        *var_dmob0_dn9_slot = var_dmob0_dn9;
        *var_dvsat0_slot = var_dvsat0;
        *var_eeffm0_slot = var_eeffm0;
        *var_eeffm0_dn0_slot = var_eeffm0_dn0;
        *var_eeffm0_dn10_slot = var_eeffm0_dn10;
        *var_eeffm0_dn11_slot = var_eeffm0_dn11;
        *var_eeffm0_dn13_slot = var_eeffm0_dn13;
        *var_eeffm0_dn14_slot = var_eeffm0_dn14;
        *var_eeffm0_dn2_slot = var_eeffm0_dn2;
        *var_eeffm0_dn3_slot = var_eeffm0_dn3;
        *var_eeffm0_dn4_slot = var_eeffm0_dn4;
        *var_eeffm0_dn5_slot = var_eeffm0_dn5;
        *var_eeffm0_dn6_slot = var_eeffm0_dn6;
        *var_eeffm0_dn7_slot = var_eeffm0_dn7;
        *var_eeffm0_dn8_slot = var_eeffm0_dn8;
        *var_eeffm0_dn9_slot = var_eeffm0_dn9;
        *var_etaiv0_slot = var_etaiv0;
        *var_etaiv0_dn0_slot = var_etaiv0_dn0;
        *var_etaiv0_dn10_slot = var_etaiv0_dn10;
        *var_etaiv0_dn11_slot = var_etaiv0_dn11;
        *var_etaiv0_dn13_slot = var_etaiv0_dn13;
        *var_etaiv0_dn14_slot = var_etaiv0_dn14;
        *var_etaiv0_dn2_slot = var_etaiv0_dn2;
        *var_etaiv0_dn3_slot = var_etaiv0_dn3;
        *var_etaiv0_dn4_slot = var_etaiv0_dn4;
        *var_etaiv0_dn5_slot = var_etaiv0_dn5;
        *var_etaiv0_dn6_slot = var_etaiv0_dn6;
        *var_etaiv0_dn7_slot = var_etaiv0_dn7;
        *var_etaiv0_dn8_slot = var_etaiv0_dn8;
        *var_etaiv0_dn9_slot = var_etaiv0_dn9;
        *var_guard635_slot = var_guard635;
        *var_guard636_slot = var_guard636;
        *var_guard637_slot = var_guard637;
        *var_guard638_slot = var_guard638;
        *var_ids0_ov_dqi0_slot = var_ids0_ov_dqi0;
        *var_ids0_ov_dqi0_dn0_slot = var_ids0_ov_dqi0_dn0;
        *var_ids0_ov_dqi0_dn10_slot = var_ids0_ov_dqi0_dn10;
        *var_ids0_ov_dqi0_dn11_slot = var_ids0_ov_dqi0_dn11;
        *var_ids0_ov_dqi0_dn13_slot = var_ids0_ov_dqi0_dn13;
        *var_ids0_ov_dqi0_dn14_slot = var_ids0_ov_dqi0_dn14;
        *var_ids0_ov_dqi0_dn2_slot = var_ids0_ov_dqi0_dn2;
        *var_ids0_ov_dqi0_dn3_slot = var_ids0_ov_dqi0_dn3;
        *var_ids0_ov_dqi0_dn4_slot = var_ids0_ov_dqi0_dn4;
        *var_ids0_ov_dqi0_dn5_slot = var_ids0_ov_dqi0_dn5;
        *var_ids0_ov_dqi0_dn6_slot = var_ids0_ov_dqi0_dn6;
        *var_ids0_ov_dqi0_dn7_slot = var_ids0_ov_dqi0_dn7;
        *var_ids0_ov_dqi0_dn8_slot = var_ids0_ov_dqi0_dn8;
        *var_ids0_ov_dqi0_dn9_slot = var_ids0_ov_dqi0_dn9;
        *var_mob0_slot = var_mob0;
        *var_mob0_dn0_slot = var_mob0_dn0;
        *var_mob0_dn10_slot = var_mob0_dn10;
        *var_mob0_dn11_slot = var_mob0_dn11;
        *var_mob0_dn13_slot = var_mob0_dn13;
        *var_mob0_dn14_slot = var_mob0_dn14;
        *var_mob0_dn2_slot = var_mob0_dn2;
        *var_mob0_dn3_slot = var_mob0_dn3;
        *var_mob0_dn4_slot = var_mob0_dn4;
        *var_mob0_dn5_slot = var_mob0_dn5;
        *var_mob0_dn6_slot = var_mob0_dn6;
        *var_mob0_dn7_slot = var_mob0_dn7;
        *var_mob0_dn8_slot = var_mob0_dn8;
        *var_mob0_dn9_slot = var_mob0_dn9;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn14_slot = var_t4_dn14;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn3_slot = var_t4_dn3;
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
        *var_t5_dn13_slot = var_t5_dn13;
        *var_t5_dn14_slot = var_t5_dn14;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn3_slot = var_t5_dn3;
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
        *var_t6_dn13_slot = var_t6_dn13;
        *var_t6_dn14_slot = var_t6_dn14;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
    }

    pub(super) fn stamp_transient_block_130(
        p: &Parameters,
        var_beta_v: f64,
        var_beta_v_dn0: f64,
        var_beta_v_dn10: f64,
        var_beta_v_dn11: f64,
        var_beta_v_dn13: f64,
        var_beta_v_dn14: f64,
        var_beta_v_dn2: f64,
        var_beta_v_dn3: f64,
        var_beta_v_dn4: f64,
        var_beta_v_dn5: f64,
        var_beta_v_dn6: f64,
        var_beta_v_dn7: f64,
        var_beta_v_dn8: f64,
        var_beta_v_dn9: f64,
        var_dmob0: f64,
        var_dmob0_dn0: f64,
        var_dmob0_dn10: f64,
        var_dmob0_dn11: f64,
        var_dmob0_dn13: f64,
        var_dmob0_dn14: f64,
        var_dmob0_dn2: f64,
        var_dmob0_dn3: f64,
        var_dmob0_dn4: f64,
        var_dmob0_dn5: f64,
        var_dmob0_dn6: f64,
        var_dmob0_dn7: f64,
        var_dmob0_dn8: f64,
        var_dmob0_dn9: f64,
        var_dvsat0: f64,
        var_guard632: f64,
        var_guard633: f64,
        var_guard636: f64,
        var_guard637: f64,
        var_guard638: f64,
        var_ids0_ov_dqi0: f64,
        var_ids0_ov_dqi0_dn0: f64,
        var_ids0_ov_dqi0_dn10: f64,
        var_ids0_ov_dqi0_dn11: f64,
        var_ids0_ov_dqi0_dn13: f64,
        var_ids0_ov_dqi0_dn14: f64,
        var_ids0_ov_dqi0_dn2: f64,
        var_ids0_ov_dqi0_dn3: f64,
        var_ids0_ov_dqi0_dn4: f64,
        var_ids0_ov_dqi0_dn5: f64,
        var_ids0_ov_dqi0_dn6: f64,
        var_ids0_ov_dqi0_dn7: f64,
        var_ids0_ov_dqi0_dn8: f64,
        var_ids0_ov_dqi0_dn9: f64,
        var_mnud0: f64,
        var_mnud0_dn0: f64,
        var_mnud0_dn10: f64,
        var_mnud0_dn11: f64,
        var_mnud0_dn13: f64,
        var_mnud0_dn14: f64,
        var_mnud0_dn2: f64,
        var_mnud0_dn3: f64,
        var_mnud0_dn4: f64,
        var_mnud0_dn5: f64,
        var_mnud0_dn6: f64,
        var_mnud0_dn7: f64,
        var_mnud0_dn8: f64,
        var_mnud0_dn9: f64,
        var_mob0: f64,
        var_mob0_dn0: f64,
        var_mob0_dn10: f64,
        var_mob0_dn11: f64,
        var_mob0_dn13: f64,
        var_mob0_dn14: f64,
        var_mob0_dn2: f64,
        var_mob0_dn3: f64,
        var_mob0_dn4: f64,
        var_mob0_dn5: f64,
        var_mob0_dn6: f64,
        var_mob0_dn7: f64,
        var_mob0_dn8: f64,
        var_mob0_dn9: f64,
        var_nfintotal: f64,
        var_noieta: f64,
        var_noieta_dn0: f64,
        var_noieta_dn10: f64,
        var_noieta_dn11: f64,
        var_noieta_dn13: f64,
        var_noieta_dn14: f64,
        var_noieta_dn2: f64,
        var_noieta_dn3: f64,
        var_noieta_dn4: f64,
        var_noieta_dn5: f64,
        var_noieta_dn6: f64,
        var_noieta_dn7: f64,
        var_noieta_dn8: f64,
        var_noieta_dn9: f64,
        var_noiwi: f64,
        var_noiwi_dn0: f64,
        var_noiwi_dn10: f64,
        var_noiwi_dn11: f64,
        var_noiwi_dn13: f64,
        var_noiwi_dn14: f64,
        var_noiwi_dn2: f64,
        var_noiwi_dn3: f64,
        var_noiwi_dn4: f64,
        var_noiwi_dn5: f64,
        var_noiwi_dn6: f64,
        var_noiwi_dn7: f64,
        var_noiwi_dn8: f64,
        var_noiwi_dn9: f64,
        var_nvtm: f64,
        var_nvtm_dn0: f64,
        var_nvtm_dn10: f64,
        var_nvtm_dn11: f64,
        var_nvtm_dn13: f64,
        var_nvtm_dn14: f64,
        var_nvtm_dn2: f64,
        var_nvtm_dn3: f64,
        var_nvtm_dn4: f64,
        var_nvtm_dn5: f64,
        var_nvtm_dn6: f64,
        var_nvtm_dn7: f64,
        var_nvtm_dn8: f64,
        var_nvtm_dn9: f64,
        var_prwgs_i: f64,
        var_qis: f64,
        var_qis_dn0: f64,
        var_qis_dn10: f64,
        var_qis_dn11: f64,
        var_qis_dn13: f64,
        var_qis_dn14: f64,
        var_qis_dn2: f64,
        var_qis_dn3: f64,
        var_qis_dn4: f64,
        var_qis_dn5: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_qis_dn9: f64,
        var_rdraingeo: f64,
        var_rdraingeo_dn0: f64,
        var_rdraingeo_dn10: f64,
        var_rdraingeo_dn11: f64,
        var_rdraingeo_dn13: f64,
        var_rdraingeo_dn14: f64,
        var_rdraingeo_dn2: f64,
        var_rdraingeo_dn3: f64,
        var_rdraingeo_dn4: f64,
        var_rdraingeo_dn5: f64,
        var_rdraingeo_dn6: f64,
        var_rdraingeo_dn7: f64,
        var_rdraingeo_dn8: f64,
        var_rdraingeo_dn9: f64,
        var_rdstemp: f64,
        var_rdstemp_dn0: f64,
        var_rdstemp_dn10: f64,
        var_rdstemp_dn11: f64,
        var_rdstemp_dn13: f64,
        var_rdstemp_dn14: f64,
        var_rdstemp_dn2: f64,
        var_rdstemp_dn3: f64,
        var_rdstemp_dn4: f64,
        var_rdstemp_dn5: f64,
        var_rdstemp_dn6: f64,
        var_rdstemp_dn7: f64,
        var_rdstemp_dn8: f64,
        var_rdstemp_dn9: f64,
        var_rdsw_i: f64,
        var_rdsw_i_dn0: f64,
        var_rdsw_i_dn10: f64,
        var_rdsw_i_dn11: f64,
        var_rdsw_i_dn13: f64,
        var_rdsw_i_dn14: f64,
        var_rdsw_i_dn2: f64,
        var_rdsw_i_dn3: f64,
        var_rdsw_i_dn4: f64,
        var_rdsw_i_dn5: f64,
        var_rdsw_i_dn6: f64,
        var_rdsw_i_dn7: f64,
        var_rdsw_i_dn8: f64,
        var_rdsw_i_dn9: f64,
        var_rsourcegeo: f64,
        var_rsourcegeo_dn0: f64,
        var_rsourcegeo_dn10: f64,
        var_rsourcegeo_dn11: f64,
        var_rsourcegeo_dn13: f64,
        var_rsourcegeo_dn14: f64,
        var_rsourcegeo_dn2: f64,
        var_rsourcegeo_dn3: f64,
        var_rsourcegeo_dn4: f64,
        var_rsourcegeo_dn5: f64,
        var_rsourcegeo_dn6: f64,
        var_rsourcegeo_dn7: f64,
        var_rsourcegeo_dn8: f64,
        var_rsourcegeo_dn9: f64,
        var_weffwrfactor: f64,
        var_dr0_slot: &mut f64,
        var_dr0_dn0_slot: &mut f64,
        var_dr0_dn10_slot: &mut f64,
        var_dr0_dn11_slot: &mut f64,
        var_dr0_dn13_slot: &mut f64,
        var_dr0_dn14_slot: &mut f64,
        var_dr0_dn2_slot: &mut f64,
        var_dr0_dn3_slot: &mut f64,
        var_dr0_dn4_slot: &mut f64,
        var_dr0_dn5_slot: &mut f64,
        var_dr0_dn6_slot: &mut f64,
        var_dr0_dn7_slot: &mut f64,
        var_dr0_dn8_slot: &mut f64,
        var_dr0_dn9_slot: &mut f64,
        var_noigd0_slot: &mut f64,
        var_noigd0_dn0_slot: &mut f64,
        var_noigd0_dn10_slot: &mut f64,
        var_noigd0_dn11_slot: &mut f64,
        var_noigd0_dn13_slot: &mut f64,
        var_noigd0_dn14_slot: &mut f64,
        var_noigd0_dn2_slot: &mut f64,
        var_noigd0_dn3_slot: &mut f64,
        var_noigd0_dn4_slot: &mut f64,
        var_noigd0_dn5_slot: &mut f64,
        var_noigd0_dn6_slot: &mut f64,
        var_noigd0_dn7_slot: &mut f64,
        var_noigd0_dn8_slot: &mut f64,
        var_noigd0_dn9_slot: &mut f64,
        var_rdsi0_slot: &mut f64,
        var_rdsi0_dn0_slot: &mut f64,
        var_rdsi0_dn10_slot: &mut f64,
        var_rdsi0_dn11_slot: &mut f64,
        var_rdsi0_dn13_slot: &mut f64,
        var_rdsi0_dn14_slot: &mut f64,
        var_rdsi0_dn2_slot: &mut f64,
        var_rdsi0_dn3_slot: &mut f64,
        var_rdsi0_dn4_slot: &mut f64,
        var_rdsi0_dn5_slot: &mut f64,
        var_rdsi0_dn6_slot: &mut f64,
        var_rdsi0_dn7_slot: &mut f64,
        var_rdsi0_dn8_slot: &mut f64,
        var_rdsi0_dn9_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn13_slot: &mut f64,
        var_t4_dn14_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_2_slot: &mut f64,
        var_t5_2_dn0_slot: &mut f64,
        var_t5_2_dn10_slot: &mut f64,
        var_t5_2_dn11_slot: &mut f64,
        var_t5_2_dn13_slot: &mut f64,
        var_t5_2_dn14_slot: &mut f64,
        var_t5_2_dn2_slot: &mut f64,
        var_t5_2_dn3_slot: &mut f64,
        var_t5_2_dn4_slot: &mut f64,
        var_t5_2_dn5_slot: &mut f64,
        var_t5_2_dn6_slot: &mut f64,
        var_t5_2_dn7_slot: &mut f64,
        var_t5_2_dn8_slot: &mut f64,
        var_t5_2_dn9_slot: &mut f64,
        var_t5_3_slot: &mut f64,
        var_t5_3_dn0_slot: &mut f64,
        var_t5_3_dn10_slot: &mut f64,
        var_t5_3_dn11_slot: &mut f64,
        var_t5_3_dn13_slot: &mut f64,
        var_t5_3_dn14_slot: &mut f64,
        var_t5_3_dn2_slot: &mut f64,
        var_t5_3_dn3_slot: &mut f64,
        var_t5_3_dn4_slot: &mut f64,
        var_t5_3_dn5_slot: &mut f64,
        var_t5_3_dn6_slot: &mut f64,
        var_t5_3_dn7_slot: &mut f64,
        var_t5_3_dn8_slot: &mut f64,
        var_t5_3_dn9_slot: &mut f64,
        var_t5_4_slot: &mut f64,
        var_t5_4_dn0_slot: &mut f64,
        var_t5_4_dn10_slot: &mut f64,
        var_t5_4_dn11_slot: &mut f64,
        var_t5_4_dn13_slot: &mut f64,
        var_t5_4_dn14_slot: &mut f64,
        var_t5_4_dn2_slot: &mut f64,
        var_t5_4_dn3_slot: &mut f64,
        var_t5_4_dn4_slot: &mut f64,
        var_t5_4_dn5_slot: &mut f64,
        var_t5_4_dn6_slot: &mut f64,
        var_t5_4_dn7_slot: &mut f64,
        var_t5_4_dn8_slot: &mut f64,
        var_t5_4_dn9_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn13_slot: &mut f64,
        var_t5_dn14_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
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
        var_t6_dn13_slot: &mut f64,
        var_t6_dn14_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_2_slot: &mut f64,
        var_t7_2_dn0_slot: &mut f64,
        var_t7_2_dn10_slot: &mut f64,
        var_t7_2_dn11_slot: &mut f64,
        var_t7_2_dn13_slot: &mut f64,
        var_t7_2_dn14_slot: &mut f64,
        var_t7_2_dn2_slot: &mut f64,
        var_t7_2_dn3_slot: &mut f64,
        var_t7_2_dn4_slot: &mut f64,
        var_t7_2_dn5_slot: &mut f64,
        var_t7_2_dn6_slot: &mut f64,
        var_t7_2_dn7_slot: &mut f64,
        var_t7_2_dn8_slot: &mut f64,
        var_t7_2_dn9_slot: &mut f64,
        var_t7_3_slot: &mut f64,
        var_t7_3_dn0_slot: &mut f64,
        var_t7_3_dn10_slot: &mut f64,
        var_t7_3_dn11_slot: &mut f64,
        var_t7_3_dn13_slot: &mut f64,
        var_t7_3_dn14_slot: &mut f64,
        var_t7_3_dn2_slot: &mut f64,
        var_t7_3_dn3_slot: &mut f64,
        var_t7_3_dn4_slot: &mut f64,
        var_t7_3_dn5_slot: &mut f64,
        var_t7_3_dn6_slot: &mut f64,
        var_t7_3_dn7_slot: &mut f64,
        var_t7_3_dn8_slot: &mut f64,
        var_t7_3_dn9_slot: &mut f64,
        var_t7_4_slot: &mut f64,
        var_t7_4_dn0_slot: &mut f64,
        var_t7_4_dn10_slot: &mut f64,
        var_t7_4_dn11_slot: &mut f64,
        var_t7_4_dn13_slot: &mut f64,
        var_t7_4_dn14_slot: &mut f64,
        var_t7_4_dn2_slot: &mut f64,
        var_t7_4_dn3_slot: &mut f64,
        var_t7_4_dn4_slot: &mut f64,
        var_t7_4_dn5_slot: &mut f64,
        var_t7_4_dn6_slot: &mut f64,
        var_t7_4_dn7_slot: &mut f64,
        var_t7_4_dn8_slot: &mut f64,
        var_t7_4_dn9_slot: &mut f64,
        var_t7_5_slot: &mut f64,
        var_t7_5_dn0_slot: &mut f64,
        var_t7_5_dn10_slot: &mut f64,
        var_t7_5_dn11_slot: &mut f64,
        var_t7_5_dn13_slot: &mut f64,
        var_t7_5_dn14_slot: &mut f64,
        var_t7_5_dn2_slot: &mut f64,
        var_t7_5_dn3_slot: &mut f64,
        var_t7_5_dn4_slot: &mut f64,
        var_t7_5_dn5_slot: &mut f64,
        var_t7_5_dn6_slot: &mut f64,
        var_t7_5_dn7_slot: &mut f64,
        var_t7_5_dn8_slot: &mut f64,
        var_t7_5_dn9_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn13_slot: &mut f64,
        var_t7_dn14_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn3_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_dn9_slot: &mut f64,
    ) {
        let mut var_dr0: f64 = *var_dr0_slot;
        let mut var_dr0_dn0: f64 = *var_dr0_dn0_slot;
        let mut var_dr0_dn10: f64 = *var_dr0_dn10_slot;
        let mut var_dr0_dn11: f64 = *var_dr0_dn11_slot;
        let mut var_dr0_dn13: f64 = *var_dr0_dn13_slot;
        let mut var_dr0_dn14: f64 = *var_dr0_dn14_slot;
        let mut var_dr0_dn2: f64 = *var_dr0_dn2_slot;
        let mut var_dr0_dn3: f64 = *var_dr0_dn3_slot;
        let mut var_dr0_dn4: f64 = *var_dr0_dn4_slot;
        let mut var_dr0_dn5: f64 = *var_dr0_dn5_slot;
        let mut var_dr0_dn6: f64 = *var_dr0_dn6_slot;
        let mut var_dr0_dn7: f64 = *var_dr0_dn7_slot;
        let mut var_dr0_dn8: f64 = *var_dr0_dn8_slot;
        let mut var_dr0_dn9: f64 = *var_dr0_dn9_slot;
        let mut var_noigd0: f64 = *var_noigd0_slot;
        let mut var_noigd0_dn0: f64 = *var_noigd0_dn0_slot;
        let mut var_noigd0_dn10: f64 = *var_noigd0_dn10_slot;
        let mut var_noigd0_dn11: f64 = *var_noigd0_dn11_slot;
        let mut var_noigd0_dn13: f64 = *var_noigd0_dn13_slot;
        let mut var_noigd0_dn14: f64 = *var_noigd0_dn14_slot;
        let mut var_noigd0_dn2: f64 = *var_noigd0_dn2_slot;
        let mut var_noigd0_dn3: f64 = *var_noigd0_dn3_slot;
        let mut var_noigd0_dn4: f64 = *var_noigd0_dn4_slot;
        let mut var_noigd0_dn5: f64 = *var_noigd0_dn5_slot;
        let mut var_noigd0_dn6: f64 = *var_noigd0_dn6_slot;
        let mut var_noigd0_dn7: f64 = *var_noigd0_dn7_slot;
        let mut var_noigd0_dn8: f64 = *var_noigd0_dn8_slot;
        let mut var_noigd0_dn9: f64 = *var_noigd0_dn9_slot;
        let mut var_rdsi0: f64 = *var_rdsi0_slot;
        let mut var_rdsi0_dn0: f64 = *var_rdsi0_dn0_slot;
        let mut var_rdsi0_dn10: f64 = *var_rdsi0_dn10_slot;
        let mut var_rdsi0_dn11: f64 = *var_rdsi0_dn11_slot;
        let mut var_rdsi0_dn13: f64 = *var_rdsi0_dn13_slot;
        let mut var_rdsi0_dn14: f64 = *var_rdsi0_dn14_slot;
        let mut var_rdsi0_dn2: f64 = *var_rdsi0_dn2_slot;
        let mut var_rdsi0_dn3: f64 = *var_rdsi0_dn3_slot;
        let mut var_rdsi0_dn4: f64 = *var_rdsi0_dn4_slot;
        let mut var_rdsi0_dn5: f64 = *var_rdsi0_dn5_slot;
        let mut var_rdsi0_dn6: f64 = *var_rdsi0_dn6_slot;
        let mut var_rdsi0_dn7: f64 = *var_rdsi0_dn7_slot;
        let mut var_rdsi0_dn8: f64 = *var_rdsi0_dn8_slot;
        let mut var_rdsi0_dn9: f64 = *var_rdsi0_dn9_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn14: f64 = *var_t4_dn14_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_2: f64 = *var_t5_2_slot;
        let mut var_t5_2_dn0: f64 = *var_t5_2_dn0_slot;
        let mut var_t5_2_dn10: f64 = *var_t5_2_dn10_slot;
        let mut var_t5_2_dn11: f64 = *var_t5_2_dn11_slot;
        let mut var_t5_2_dn13: f64 = *var_t5_2_dn13_slot;
        let mut var_t5_2_dn14: f64 = *var_t5_2_dn14_slot;
        let mut var_t5_2_dn2: f64 = *var_t5_2_dn2_slot;
        let mut var_t5_2_dn3: f64 = *var_t5_2_dn3_slot;
        let mut var_t5_2_dn4: f64 = *var_t5_2_dn4_slot;
        let mut var_t5_2_dn5: f64 = *var_t5_2_dn5_slot;
        let mut var_t5_2_dn6: f64 = *var_t5_2_dn6_slot;
        let mut var_t5_2_dn7: f64 = *var_t5_2_dn7_slot;
        let mut var_t5_2_dn8: f64 = *var_t5_2_dn8_slot;
        let mut var_t5_2_dn9: f64 = *var_t5_2_dn9_slot;
        let mut var_t5_3: f64 = *var_t5_3_slot;
        let mut var_t5_3_dn0: f64 = *var_t5_3_dn0_slot;
        let mut var_t5_3_dn10: f64 = *var_t5_3_dn10_slot;
        let mut var_t5_3_dn11: f64 = *var_t5_3_dn11_slot;
        let mut var_t5_3_dn13: f64 = *var_t5_3_dn13_slot;
        let mut var_t5_3_dn14: f64 = *var_t5_3_dn14_slot;
        let mut var_t5_3_dn2: f64 = *var_t5_3_dn2_slot;
        let mut var_t5_3_dn3: f64 = *var_t5_3_dn3_slot;
        let mut var_t5_3_dn4: f64 = *var_t5_3_dn4_slot;
        let mut var_t5_3_dn5: f64 = *var_t5_3_dn5_slot;
        let mut var_t5_3_dn6: f64 = *var_t5_3_dn6_slot;
        let mut var_t5_3_dn7: f64 = *var_t5_3_dn7_slot;
        let mut var_t5_3_dn8: f64 = *var_t5_3_dn8_slot;
        let mut var_t5_3_dn9: f64 = *var_t5_3_dn9_slot;
        let mut var_t5_4: f64 = *var_t5_4_slot;
        let mut var_t5_4_dn0: f64 = *var_t5_4_dn0_slot;
        let mut var_t5_4_dn10: f64 = *var_t5_4_dn10_slot;
        let mut var_t5_4_dn11: f64 = *var_t5_4_dn11_slot;
        let mut var_t5_4_dn13: f64 = *var_t5_4_dn13_slot;
        let mut var_t5_4_dn14: f64 = *var_t5_4_dn14_slot;
        let mut var_t5_4_dn2: f64 = *var_t5_4_dn2_slot;
        let mut var_t5_4_dn3: f64 = *var_t5_4_dn3_slot;
        let mut var_t5_4_dn4: f64 = *var_t5_4_dn4_slot;
        let mut var_t5_4_dn5: f64 = *var_t5_4_dn5_slot;
        let mut var_t5_4_dn6: f64 = *var_t5_4_dn6_slot;
        let mut var_t5_4_dn7: f64 = *var_t5_4_dn7_slot;
        let mut var_t5_4_dn8: f64 = *var_t5_4_dn8_slot;
        let mut var_t5_4_dn9: f64 = *var_t5_4_dn9_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn13: f64 = *var_t5_dn13_slot;
        let mut var_t5_dn14: f64 = *var_t5_dn14_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
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
        let mut var_t6_dn13: f64 = *var_t6_dn13_slot;
        let mut var_t6_dn14: f64 = *var_t6_dn14_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_2: f64 = *var_t7_2_slot;
        let mut var_t7_2_dn0: f64 = *var_t7_2_dn0_slot;
        let mut var_t7_2_dn10: f64 = *var_t7_2_dn10_slot;
        let mut var_t7_2_dn11: f64 = *var_t7_2_dn11_slot;
        let mut var_t7_2_dn13: f64 = *var_t7_2_dn13_slot;
        let mut var_t7_2_dn14: f64 = *var_t7_2_dn14_slot;
        let mut var_t7_2_dn2: f64 = *var_t7_2_dn2_slot;
        let mut var_t7_2_dn3: f64 = *var_t7_2_dn3_slot;
        let mut var_t7_2_dn4: f64 = *var_t7_2_dn4_slot;
        let mut var_t7_2_dn5: f64 = *var_t7_2_dn5_slot;
        let mut var_t7_2_dn6: f64 = *var_t7_2_dn6_slot;
        let mut var_t7_2_dn7: f64 = *var_t7_2_dn7_slot;
        let mut var_t7_2_dn8: f64 = *var_t7_2_dn8_slot;
        let mut var_t7_2_dn9: f64 = *var_t7_2_dn9_slot;
        let mut var_t7_3: f64 = *var_t7_3_slot;
        let mut var_t7_3_dn0: f64 = *var_t7_3_dn0_slot;
        let mut var_t7_3_dn10: f64 = *var_t7_3_dn10_slot;
        let mut var_t7_3_dn11: f64 = *var_t7_3_dn11_slot;
        let mut var_t7_3_dn13: f64 = *var_t7_3_dn13_slot;
        let mut var_t7_3_dn14: f64 = *var_t7_3_dn14_slot;
        let mut var_t7_3_dn2: f64 = *var_t7_3_dn2_slot;
        let mut var_t7_3_dn3: f64 = *var_t7_3_dn3_slot;
        let mut var_t7_3_dn4: f64 = *var_t7_3_dn4_slot;
        let mut var_t7_3_dn5: f64 = *var_t7_3_dn5_slot;
        let mut var_t7_3_dn6: f64 = *var_t7_3_dn6_slot;
        let mut var_t7_3_dn7: f64 = *var_t7_3_dn7_slot;
        let mut var_t7_3_dn8: f64 = *var_t7_3_dn8_slot;
        let mut var_t7_3_dn9: f64 = *var_t7_3_dn9_slot;
        let mut var_t7_4: f64 = *var_t7_4_slot;
        let mut var_t7_4_dn0: f64 = *var_t7_4_dn0_slot;
        let mut var_t7_4_dn10: f64 = *var_t7_4_dn10_slot;
        let mut var_t7_4_dn11: f64 = *var_t7_4_dn11_slot;
        let mut var_t7_4_dn13: f64 = *var_t7_4_dn13_slot;
        let mut var_t7_4_dn14: f64 = *var_t7_4_dn14_slot;
        let mut var_t7_4_dn2: f64 = *var_t7_4_dn2_slot;
        let mut var_t7_4_dn3: f64 = *var_t7_4_dn3_slot;
        let mut var_t7_4_dn4: f64 = *var_t7_4_dn4_slot;
        let mut var_t7_4_dn5: f64 = *var_t7_4_dn5_slot;
        let mut var_t7_4_dn6: f64 = *var_t7_4_dn6_slot;
        let mut var_t7_4_dn7: f64 = *var_t7_4_dn7_slot;
        let mut var_t7_4_dn8: f64 = *var_t7_4_dn8_slot;
        let mut var_t7_4_dn9: f64 = *var_t7_4_dn9_slot;
        let mut var_t7_5: f64 = *var_t7_5_slot;
        let mut var_t7_5_dn0: f64 = *var_t7_5_dn0_slot;
        let mut var_t7_5_dn10: f64 = *var_t7_5_dn10_slot;
        let mut var_t7_5_dn11: f64 = *var_t7_5_dn11_slot;
        let mut var_t7_5_dn13: f64 = *var_t7_5_dn13_slot;
        let mut var_t7_5_dn14: f64 = *var_t7_5_dn14_slot;
        let mut var_t7_5_dn2: f64 = *var_t7_5_dn2_slot;
        let mut var_t7_5_dn3: f64 = *var_t7_5_dn3_slot;
        let mut var_t7_5_dn4: f64 = *var_t7_5_dn4_slot;
        let mut var_t7_5_dn5: f64 = *var_t7_5_dn5_slot;
        let mut var_t7_5_dn6: f64 = *var_t7_5_dn6_slot;
        let mut var_t7_5_dn7: f64 = *var_t7_5_dn7_slot;
        let mut var_t7_5_dn8: f64 = *var_t7_5_dn8_slot;
        let mut var_t7_5_dn9: f64 = *var_t7_5_dn9_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn13: f64 = *var_t7_dn13_slot;
        let mut var_t7_dn14: f64 = *var_t7_dn14_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn3: f64 = *var_t7_dn3_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_dn9: f64 = *var_t7_dn9_slot;

        let (assign33740_e56354, assign33740_e56354_d_n0, assign33740_e56354_d_n2, assign33740_e56354_d_n3, assign33740_e56354_d_n4, assign33740_e56354_d_n5, assign33740_e56354_d_n6, assign33740_e56354_d_n7, assign33740_e56354_d_n8, assign33740_e56354_d_n9, assign33740_e56354_d_n10, assign33740_e56354_d_n11, assign33740_e56354_d_n13, assign33740_e56354_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && (var_guard636 != 0.0)) {
        let assign33740_e56348: f64 = (var_rdsw_i * var_t6);
        let assign33740_e56349: f64 = (p.p908 + assign33740_e56348);
        let assign33740_e56350: f64 = (var_rdstemp * assign33740_e56349);
        let assign33740_e56352: f64 = (assign33740_e56350 * var_weffwrfactor);
        (assign33740_e56352, (((var_rdstemp_dn0 * assign33740_e56349) + (var_rdstemp * ((var_rdsw_i_dn0 * var_t6) + (var_rdsw_i * var_t6_dn0)))) * var_weffwrfactor), (((var_rdstemp_dn2 * assign33740_e56349) + (var_rdstemp * ((var_rdsw_i_dn2 * var_t6) + (var_rdsw_i * var_t6_dn2)))) * var_weffwrfactor), (((var_rdstemp_dn3 * assign33740_e56349) + (var_rdstemp * ((var_rdsw_i_dn3 * var_t6) + (var_rdsw_i * var_t6_dn3)))) * var_weffwrfactor), (((var_rdstemp_dn4 * assign33740_e56349) + (var_rdstemp * ((var_rdsw_i_dn4 * var_t6) + (var_rdsw_i * var_t6_dn4)))) * var_weffwrfactor), (((var_rdstemp_dn5 * assign33740_e56349) + (var_rdstemp * ((var_rdsw_i_dn5 * var_t6) + (var_rdsw_i * var_t6_dn5)))) * var_weffwrfactor), (((var_rdstemp_dn6 * assign33740_e56349) + (var_rdstemp * ((var_rdsw_i_dn6 * var_t6) + (var_rdsw_i * var_t6_dn6)))) * var_weffwrfactor), (((var_rdstemp_dn7 * assign33740_e56349) + (var_rdstemp * ((var_rdsw_i_dn7 * var_t6) + (var_rdsw_i * var_t6_dn7)))) * var_weffwrfactor), (((var_rdstemp_dn8 * assign33740_e56349) + (var_rdstemp * ((var_rdsw_i_dn8 * var_t6) + (var_rdsw_i * var_t6_dn8)))) * var_weffwrfactor), (((var_rdstemp_dn9 * assign33740_e56349) + (var_rdstemp * ((var_rdsw_i_dn9 * var_t6) + (var_rdsw_i * var_t6_dn9)))) * var_weffwrfactor), (((var_rdstemp_dn10 * assign33740_e56349) + (var_rdstemp * ((var_rdsw_i_dn10 * var_t6) + (var_rdsw_i * var_t6_dn10)))) * var_weffwrfactor), (((var_rdstemp_dn11 * assign33740_e56349) + (var_rdstemp * ((var_rdsw_i_dn11 * var_t6) + (var_rdsw_i * var_t6_dn11)))) * var_weffwrfactor), (((var_rdstemp_dn13 * assign33740_e56349) + (var_rdstemp * ((var_rdsw_i_dn13 * var_t6) + (var_rdsw_i * var_t6_dn13)))) * var_weffwrfactor), (((var_rdstemp_dn14 * assign33740_e56349) + (var_rdstemp * ((var_rdsw_i_dn14 * var_t6) + (var_rdsw_i * var_t6_dn14)))) * var_weffwrfactor),)
    } else {
        (var_rdsi0, var_rdsi0_dn0, var_rdsi0_dn2, var_rdsi0_dn3, var_rdsi0_dn4, var_rdsi0_dn5, var_rdsi0_dn6, var_rdsi0_dn7, var_rdsi0_dn8, var_rdsi0_dn9, var_rdsi0_dn10, var_rdsi0_dn11, var_rdsi0_dn13, var_rdsi0_dn14,)
    }
};
        var_rdsi0 = assign33740_e56354;
        var_rdsi0_dn0 = assign33740_e56354_d_n0;
        var_rdsi0_dn2 = assign33740_e56354_d_n2;
        var_rdsi0_dn3 = assign33740_e56354_d_n3;
        var_rdsi0_dn4 = assign33740_e56354_d_n4;
        var_rdsi0_dn5 = assign33740_e56354_d_n5;
        var_rdsi0_dn6 = assign33740_e56354_d_n6;
        var_rdsi0_dn7 = assign33740_e56354_d_n7;
        var_rdsi0_dn8 = assign33740_e56354_d_n8;
        var_rdsi0_dn9 = assign33740_e56354_d_n9;
        var_rdsi0_dn10 = assign33740_e56354_d_n10;
        var_rdsi0_dn11 = assign33740_e56354_d_n11;
        var_rdsi0_dn13 = assign33740_e56354_d_n13;
        var_rdsi0_dn14 = assign33740_e56354_d_n14;

        let (assign33750_e56375, assign33750_e56375_d_n0, assign33750_e56375_d_n2, assign33750_e56375_d_n3, assign33750_e56375_d_n4, assign33750_e56375_d_n5, assign33750_e56375_d_n6, assign33750_e56375_d_n7, assign33750_e56375_d_n8, assign33750_e56375_d_n9, assign33750_e56375_d_n10, assign33750_e56375_d_n11, assign33750_e56375_d_n13, assign33750_e56375_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && (var_guard636 != 0.0)) {
        let assign33750_e56364: f64 = (var_nfintotal * var_beta_v);
        let assign33750_e56366: f64 = (assign33750_e56364 * var_ids0_ov_dqi0);
        let assign33750_e56369: f64 = (var_dmob0 * var_dvsat0);
        let assign33750_e56370: f64 = (assign33750_e56366 / assign33750_e56369);
        let assign33750_e56372: f64 = (assign33750_e56370 * var_rdsi0);
        let assign33750_e56373: f64 = (1.0 + assign33750_e56372);
        (assign33750_e56373, ((((((((var_nfintotal * var_beta_v_dn0) * var_ids0_ov_dqi0) + (assign33750_e56364 * var_ids0_ov_dqi0_dn0)) * assign33750_e56369) - (assign33750_e56366 * (var_dmob0_dn0 * var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * var_rdsi0) + (assign33750_e56370 * var_rdsi0_dn0)), ((((((((var_nfintotal * var_beta_v_dn2) * var_ids0_ov_dqi0) + (assign33750_e56364 * var_ids0_ov_dqi0_dn2)) * assign33750_e56369) - (assign33750_e56366 * (var_dmob0_dn2 * var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * var_rdsi0) + (assign33750_e56370 * var_rdsi0_dn2)), ((((((((var_nfintotal * var_beta_v_dn3) * var_ids0_ov_dqi0) + (assign33750_e56364 * var_ids0_ov_dqi0_dn3)) * assign33750_e56369) - (assign33750_e56366 * (var_dmob0_dn3 * var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * var_rdsi0) + (assign33750_e56370 * var_rdsi0_dn3)), ((((((((var_nfintotal * var_beta_v_dn4) * var_ids0_ov_dqi0) + (assign33750_e56364 * var_ids0_ov_dqi0_dn4)) * assign33750_e56369) - (assign33750_e56366 * (var_dmob0_dn4 * var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * var_rdsi0) + (assign33750_e56370 * var_rdsi0_dn4)), ((((((((var_nfintotal * var_beta_v_dn5) * var_ids0_ov_dqi0) + (assign33750_e56364 * var_ids0_ov_dqi0_dn5)) * assign33750_e56369) - (assign33750_e56366 * (var_dmob0_dn5 * var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * var_rdsi0) + (assign33750_e56370 * var_rdsi0_dn5)), ((((((((var_nfintotal * var_beta_v_dn6) * var_ids0_ov_dqi0) + (assign33750_e56364 * var_ids0_ov_dqi0_dn6)) * assign33750_e56369) - (assign33750_e56366 * (var_dmob0_dn6 * var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * var_rdsi0) + (assign33750_e56370 * var_rdsi0_dn6)), ((((((((var_nfintotal * var_beta_v_dn7) * var_ids0_ov_dqi0) + (assign33750_e56364 * var_ids0_ov_dqi0_dn7)) * assign33750_e56369) - (assign33750_e56366 * (var_dmob0_dn7 * var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * var_rdsi0) + (assign33750_e56370 * var_rdsi0_dn7)), ((((((((var_nfintotal * var_beta_v_dn8) * var_ids0_ov_dqi0) + (assign33750_e56364 * var_ids0_ov_dqi0_dn8)) * assign33750_e56369) - (assign33750_e56366 * (var_dmob0_dn8 * var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * var_rdsi0) + (assign33750_e56370 * var_rdsi0_dn8)), ((((((((var_nfintotal * var_beta_v_dn9) * var_ids0_ov_dqi0) + (assign33750_e56364 * var_ids0_ov_dqi0_dn9)) * assign33750_e56369) - (assign33750_e56366 * (var_dmob0_dn9 * var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * var_rdsi0) + (assign33750_e56370 * var_rdsi0_dn9)), ((((((((var_nfintotal * var_beta_v_dn10) * var_ids0_ov_dqi0) + (assign33750_e56364 * var_ids0_ov_dqi0_dn10)) * assign33750_e56369) - (assign33750_e56366 * (var_dmob0_dn10 * var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * var_rdsi0) + (assign33750_e56370 * var_rdsi0_dn10)), ((((((((var_nfintotal * var_beta_v_dn11) * var_ids0_ov_dqi0) + (assign33750_e56364 * var_ids0_ov_dqi0_dn11)) * assign33750_e56369) - (assign33750_e56366 * (var_dmob0_dn11 * var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * var_rdsi0) + (assign33750_e56370 * var_rdsi0_dn11)), ((((((((var_nfintotal * var_beta_v_dn13) * var_ids0_ov_dqi0) + (assign33750_e56364 * var_ids0_ov_dqi0_dn13)) * assign33750_e56369) - (assign33750_e56366 * (var_dmob0_dn13 * var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * var_rdsi0) + (assign33750_e56370 * var_rdsi0_dn13)), ((((((((var_nfintotal * var_beta_v_dn14) * var_ids0_ov_dqi0) + (assign33750_e56364 * var_ids0_ov_dqi0_dn14)) * assign33750_e56369) - (assign33750_e56366 * (var_dmob0_dn14 * var_dvsat0))) / (assign33750_e56369 * assign33750_e56369)) * var_rdsi0) + (assign33750_e56370 * var_rdsi0_dn14)),)
    } else {
        (var_dr0, var_dr0_dn0, var_dr0_dn2, var_dr0_dn3, var_dr0_dn4, var_dr0_dn5, var_dr0_dn6, var_dr0_dn7, var_dr0_dn8, var_dr0_dn9, var_dr0_dn10, var_dr0_dn11, var_dr0_dn13, var_dr0_dn14,)
    }
};
        var_dr0 = assign33750_e56375;
        var_dr0_dn0 = assign33750_e56375_d_n0;
        var_dr0_dn2 = assign33750_e56375_d_n2;
        var_dr0_dn3 = assign33750_e56375_d_n3;
        var_dr0_dn4 = assign33750_e56375_d_n4;
        var_dr0_dn5 = assign33750_e56375_d_n5;
        var_dr0_dn6 = assign33750_e56375_d_n6;
        var_dr0_dn7 = assign33750_e56375_d_n7;
        var_dr0_dn8 = assign33750_e56375_d_n8;
        var_dr0_dn9 = assign33750_e56375_d_n9;
        var_dr0_dn10 = assign33750_e56375_d_n10;
        var_dr0_dn11 = assign33750_e56375_d_n11;
        var_dr0_dn13 = assign33750_e56375_d_n13;
        var_dr0_dn14 = assign33750_e56375_d_n14;

        let (assign33760_e56387, assign33760_e56387_d_n0, assign33760_e56387_d_n2, assign33760_e56387_d_n3, assign33760_e56387_d_n4, assign33760_e56387_d_n5, assign33760_e56387_d_n6, assign33760_e56387_d_n7, assign33760_e56387_d_n8, assign33760_e56387_d_n9, assign33760_e56387_d_n10, assign33760_e56387_d_n11, assign33760_e56387_d_n13, assign33760_e56387_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && ((var_guard637 != 0.0) && (var_guard636 == 0.0))) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dr0, var_dr0_dn0, var_dr0_dn2, var_dr0_dn3, var_dr0_dn4, var_dr0_dn5, var_dr0_dn6, var_dr0_dn7, var_dr0_dn8, var_dr0_dn9, var_dr0_dn10, var_dr0_dn11, var_dr0_dn13, var_dr0_dn14,)
    }
};
        var_dr0 = assign33760_e56387;
        var_dr0_dn0 = assign33760_e56387_d_n0;
        var_dr0_dn2 = assign33760_e56387_d_n2;
        var_dr0_dn3 = assign33760_e56387_d_n3;
        var_dr0_dn4 = assign33760_e56387_d_n4;
        var_dr0_dn5 = assign33760_e56387_d_n5;
        var_dr0_dn6 = assign33760_e56387_d_n6;
        var_dr0_dn7 = assign33760_e56387_d_n7;
        var_dr0_dn8 = assign33760_e56387_d_n8;
        var_dr0_dn9 = assign33760_e56387_d_n9;
        var_dr0_dn10 = assign33760_e56387_d_n10;
        var_dr0_dn11 = assign33760_e56387_d_n11;
        var_dr0_dn13 = assign33760_e56387_d_n13;
        var_dr0_dn14 = assign33760_e56387_d_n14;

        let (assign33770_e56405, assign33770_e56405_d_n0, assign33770_e56405_d_n2, assign33770_e56405_d_n3, assign33770_e56405_d_n4, assign33770_e56405_d_n5, assign33770_e56405_d_n6, assign33770_e56405_d_n7, assign33770_e56405_d_n8, assign33770_e56405_d_n9, assign33770_e56405_d_n10, assign33770_e56405_d_n11, assign33770_e56405_d_n13, assign33770_e56405_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && ((var_guard638 != 0.0) && (!((var_guard636 != 0.0) || (var_guard637 != 0.0))))) {
        let assign33770_e56402: f64 = (var_prwgs_i * var_qis);
        let assign33770_e56403: f64 = (1.0 + assign33770_e56402);
        (assign33770_e56403, (var_prwgs_i * var_qis_dn0), (var_prwgs_i * var_qis_dn2), (var_prwgs_i * var_qis_dn3), (var_prwgs_i * var_qis_dn4), (var_prwgs_i * var_qis_dn5), (var_prwgs_i * var_qis_dn6), (var_prwgs_i * var_qis_dn7), (var_prwgs_i * var_qis_dn8), (var_prwgs_i * var_qis_dn9), (var_prwgs_i * var_qis_dn10), (var_prwgs_i * var_qis_dn11), (var_prwgs_i * var_qis_dn13), (var_prwgs_i * var_qis_dn14),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11, var_t4_dn13, var_t4_dn14,)
    }
};
        var_t4 = assign33770_e56405;
        var_t4_dn0 = assign33770_e56405_d_n0;
        var_t4_dn2 = assign33770_e56405_d_n2;
        var_t4_dn3 = assign33770_e56405_d_n3;
        var_t4_dn4 = assign33770_e56405_d_n4;
        var_t4_dn5 = assign33770_e56405_d_n5;
        var_t4_dn6 = assign33770_e56405_d_n6;
        var_t4_dn7 = assign33770_e56405_d_n7;
        var_t4_dn8 = assign33770_e56405_d_n8;
        var_t4_dn9 = assign33770_e56405_d_n9;
        var_t4_dn10 = assign33770_e56405_d_n10;
        var_t4_dn11 = assign33770_e56405_d_n11;
        var_t4_dn13 = assign33770_e56405_d_n13;
        var_t4_dn14 = assign33770_e56405_d_n14;

        let (assign33780_e56421, assign33780_e56421_d_n0, assign33780_e56421_d_n2, assign33780_e56421_d_n3, assign33780_e56421_d_n4, assign33780_e56421_d_n5, assign33780_e56421_d_n6, assign33780_e56421_d_n7, assign33780_e56421_d_n8, assign33780_e56421_d_n9, assign33780_e56421_d_n10, assign33780_e56421_d_n11, assign33780_e56421_d_n13, assign33780_e56421_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && ((var_guard638 != 0.0) && (!((var_guard636 != 0.0) || (var_guard637 != 0.0))))) {
        let assign33780_e56419: f64 = (1.0 / var_t4);
        (assign33780_e56419, (-(var_t4_dn0 / (var_t4 * var_t4))), (-(var_t4_dn2 / (var_t4 * var_t4))), (-(var_t4_dn3 / (var_t4 * var_t4))), (-(var_t4_dn4 / (var_t4 * var_t4))), (-(var_t4_dn5 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn7 / (var_t4 * var_t4))), (-(var_t4_dn8 / (var_t4 * var_t4))), (-(var_t4_dn9 / (var_t4 * var_t4))), (-(var_t4_dn10 / (var_t4 * var_t4))), (-(var_t4_dn11 / (var_t4 * var_t4))), (-(var_t4_dn13 / (var_t4 * var_t4))), (-(var_t4_dn14 / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11, var_t5_dn13, var_t5_dn14,)
    }
};
        var_t5 = assign33780_e56421;
        var_t5_dn0 = assign33780_e56421_d_n0;
        var_t5_dn2 = assign33780_e56421_d_n2;
        var_t5_dn3 = assign33780_e56421_d_n3;
        var_t5_dn4 = assign33780_e56421_d_n4;
        var_t5_dn5 = assign33780_e56421_d_n5;
        var_t5_dn6 = assign33780_e56421_d_n6;
        var_t5_dn7 = assign33780_e56421_d_n7;
        var_t5_dn8 = assign33780_e56421_d_n8;
        var_t5_dn9 = assign33780_e56421_d_n9;
        var_t5_dn10 = assign33780_e56421_d_n10;
        var_t5_dn11 = assign33780_e56421_d_n11;
        var_t5_dn13 = assign33780_e56421_d_n13;
        var_t5_dn14 = assign33780_e56421_d_n14;

        let (assign33790_e56444, assign33790_e56444_d_n0, assign33790_e56444_d_n2, assign33790_e56444_d_n3, assign33790_e56444_d_n4, assign33790_e56444_d_n5, assign33790_e56444_d_n6, assign33790_e56444_d_n7, assign33790_e56444_d_n8, assign33790_e56444_d_n9, assign33790_e56444_d_n10, assign33790_e56444_d_n11, assign33790_e56444_d_n13, assign33790_e56444_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && ((var_guard638 != 0.0) && (!((var_guard636 != 0.0) || (var_guard637 != 0.0))))) {
        let assign33790_e56437: f64 = (var_t5 * var_t5);
        let assign33790_e56439: f64 = (assign33790_e56437 + 0.01);
        let assign33790_e56440: f64 = (assign33790_e56439).sqrt();
        let assign33790_e56441: f64 = (var_t5 + assign33790_e56440);
        let assign33790_e56442: f64 = (0.5 * assign33790_e56441);
        (assign33790_e56442, (0.5 * (var_t5_dn0 + (((var_t5_dn0 * var_t5) + (var_t5 * var_t5_dn0)) / (2.0 * assign33790_e56440)))), (0.5 * (var_t5_dn2 + (((var_t5_dn2 * var_t5) + (var_t5 * var_t5_dn2)) / (2.0 * assign33790_e56440)))), (0.5 * (var_t5_dn3 + (((var_t5_dn3 * var_t5) + (var_t5 * var_t5_dn3)) / (2.0 * assign33790_e56440)))), (0.5 * (var_t5_dn4 + (((var_t5_dn4 * var_t5) + (var_t5 * var_t5_dn4)) / (2.0 * assign33790_e56440)))), (0.5 * (var_t5_dn5 + (((var_t5_dn5 * var_t5) + (var_t5 * var_t5_dn5)) / (2.0 * assign33790_e56440)))), (0.5 * (var_t5_dn6 + (((var_t5_dn6 * var_t5) + (var_t5 * var_t5_dn6)) / (2.0 * assign33790_e56440)))), (0.5 * (var_t5_dn7 + (((var_t5_dn7 * var_t5) + (var_t5 * var_t5_dn7)) / (2.0 * assign33790_e56440)))), (0.5 * (var_t5_dn8 + (((var_t5_dn8 * var_t5) + (var_t5 * var_t5_dn8)) / (2.0 * assign33790_e56440)))), (0.5 * (var_t5_dn9 + (((var_t5_dn9 * var_t5) + (var_t5 * var_t5_dn9)) / (2.0 * assign33790_e56440)))), (0.5 * (var_t5_dn10 + (((var_t5_dn10 * var_t5) + (var_t5 * var_t5_dn10)) / (2.0 * assign33790_e56440)))), (0.5 * (var_t5_dn11 + (((var_t5_dn11 * var_t5) + (var_t5 * var_t5_dn11)) / (2.0 * assign33790_e56440)))), (0.5 * (var_t5_dn13 + (((var_t5_dn13 * var_t5) + (var_t5 * var_t5_dn13)) / (2.0 * assign33790_e56440)))), (0.5 * (var_t5_dn14 + (((var_t5_dn14 * var_t5) + (var_t5 * var_t5_dn14)) / (2.0 * assign33790_e56440)))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn13, var_t6_dn14,)
    }
};
        var_t6 = assign33790_e56444;
        var_t6_dn0 = assign33790_e56444_d_n0;
        var_t6_dn2 = assign33790_e56444_d_n2;
        var_t6_dn3 = assign33790_e56444_d_n3;
        var_t6_dn4 = assign33790_e56444_d_n4;
        var_t6_dn5 = assign33790_e56444_d_n5;
        var_t6_dn6 = assign33790_e56444_d_n6;
        var_t6_dn7 = assign33790_e56444_d_n7;
        var_t6_dn8 = assign33790_e56444_d_n8;
        var_t6_dn9 = assign33790_e56444_d_n9;
        var_t6_dn10 = assign33790_e56444_d_n10;
        var_t6_dn11 = assign33790_e56444_d_n11;
        var_t6_dn13 = assign33790_e56444_d_n13;
        var_t6_dn14 = assign33790_e56444_d_n14;

        let (assign33800_e56464, assign33800_e56464_d_n0, assign33800_e56464_d_n2, assign33800_e56464_d_n3, assign33800_e56464_d_n4, assign33800_e56464_d_n5, assign33800_e56464_d_n6, assign33800_e56464_d_n7, assign33800_e56464_d_n8, assign33800_e56464_d_n9, assign33800_e56464_d_n10, assign33800_e56464_d_n11, assign33800_e56464_d_n13, assign33800_e56464_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && ((var_guard638 != 0.0) && (!((var_guard636 != 0.0) || (var_guard637 != 0.0))))) {
        let assign33800_e56459: f64 = (var_rdsw_i * var_t6);
        let assign33800_e56460: f64 = (p.p908 + assign33800_e56459);
        let assign33800_e56462: f64 = (assign33800_e56460 * var_weffwrfactor);
        (assign33800_e56462, (((var_rdsw_i_dn0 * var_t6) + (var_rdsw_i * var_t6_dn0)) * var_weffwrfactor), (((var_rdsw_i_dn2 * var_t6) + (var_rdsw_i * var_t6_dn2)) * var_weffwrfactor), (((var_rdsw_i_dn3 * var_t6) + (var_rdsw_i * var_t6_dn3)) * var_weffwrfactor), (((var_rdsw_i_dn4 * var_t6) + (var_rdsw_i * var_t6_dn4)) * var_weffwrfactor), (((var_rdsw_i_dn5 * var_t6) + (var_rdsw_i * var_t6_dn5)) * var_weffwrfactor), (((var_rdsw_i_dn6 * var_t6) + (var_rdsw_i * var_t6_dn6)) * var_weffwrfactor), (((var_rdsw_i_dn7 * var_t6) + (var_rdsw_i * var_t6_dn7)) * var_weffwrfactor), (((var_rdsw_i_dn8 * var_t6) + (var_rdsw_i * var_t6_dn8)) * var_weffwrfactor), (((var_rdsw_i_dn9 * var_t6) + (var_rdsw_i * var_t6_dn9)) * var_weffwrfactor), (((var_rdsw_i_dn10 * var_t6) + (var_rdsw_i * var_t6_dn10)) * var_weffwrfactor), (((var_rdsw_i_dn11 * var_t6) + (var_rdsw_i * var_t6_dn11)) * var_weffwrfactor), (((var_rdsw_i_dn13 * var_t6) + (var_rdsw_i * var_t6_dn13)) * var_weffwrfactor), (((var_rdsw_i_dn14 * var_t6) + (var_rdsw_i * var_t6_dn14)) * var_weffwrfactor),)
    } else {
        (var_rdsi0, var_rdsi0_dn0, var_rdsi0_dn2, var_rdsi0_dn3, var_rdsi0_dn4, var_rdsi0_dn5, var_rdsi0_dn6, var_rdsi0_dn7, var_rdsi0_dn8, var_rdsi0_dn9, var_rdsi0_dn10, var_rdsi0_dn11, var_rdsi0_dn13, var_rdsi0_dn14,)
    }
};
        var_rdsi0 = assign33800_e56464;
        var_rdsi0_dn0 = assign33800_e56464_d_n0;
        var_rdsi0_dn2 = assign33800_e56464_d_n2;
        var_rdsi0_dn3 = assign33800_e56464_d_n3;
        var_rdsi0_dn4 = assign33800_e56464_d_n4;
        var_rdsi0_dn5 = assign33800_e56464_d_n5;
        var_rdsi0_dn6 = assign33800_e56464_d_n6;
        var_rdsi0_dn7 = assign33800_e56464_d_n7;
        var_rdsi0_dn8 = assign33800_e56464_d_n8;
        var_rdsi0_dn9 = assign33800_e56464_d_n9;
        var_rdsi0_dn10 = assign33800_e56464_d_n10;
        var_rdsi0_dn11 = assign33800_e56464_d_n11;
        var_rdsi0_dn13 = assign33800_e56464_d_n13;
        var_rdsi0_dn14 = assign33800_e56464_d_n14;

        let (assign33810_e56484, assign33810_e56484_d_n0, assign33810_e56484_d_n2, assign33810_e56484_d_n3, assign33810_e56484_d_n4, assign33810_e56484_d_n5, assign33810_e56484_d_n6, assign33810_e56484_d_n7, assign33810_e56484_d_n8, assign33810_e56484_d_n9, assign33810_e56484_d_n10, assign33810_e56484_d_n11, assign33810_e56484_d_n13, assign33810_e56484_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && ((var_guard638 != 0.0) && (!((var_guard636 != 0.0) || (var_guard637 != 0.0))))) {
        let assign33810_e56479: f64 = (var_rsourcegeo + var_rdraingeo);
        let assign33810_e56481: f64 = (assign33810_e56479 + var_rdsi0);
        let assign33810_e56482: f64 = (var_rdstemp * assign33810_e56481);
        (assign33810_e56482, ((var_rdstemp_dn0 * assign33810_e56481) + (var_rdstemp * ((var_rsourcegeo_dn0 + var_rdraingeo_dn0) + var_rdsi0_dn0))), ((var_rdstemp_dn2 * assign33810_e56481) + (var_rdstemp * ((var_rsourcegeo_dn2 + var_rdraingeo_dn2) + var_rdsi0_dn2))), ((var_rdstemp_dn3 * assign33810_e56481) + (var_rdstemp * ((var_rsourcegeo_dn3 + var_rdraingeo_dn3) + var_rdsi0_dn3))), ((var_rdstemp_dn4 * assign33810_e56481) + (var_rdstemp * ((var_rsourcegeo_dn4 + var_rdraingeo_dn4) + var_rdsi0_dn4))), ((var_rdstemp_dn5 * assign33810_e56481) + (var_rdstemp * ((var_rsourcegeo_dn5 + var_rdraingeo_dn5) + var_rdsi0_dn5))), ((var_rdstemp_dn6 * assign33810_e56481) + (var_rdstemp * ((var_rsourcegeo_dn6 + var_rdraingeo_dn6) + var_rdsi0_dn6))), ((var_rdstemp_dn7 * assign33810_e56481) + (var_rdstemp * ((var_rsourcegeo_dn7 + var_rdraingeo_dn7) + var_rdsi0_dn7))), ((var_rdstemp_dn8 * assign33810_e56481) + (var_rdstemp * ((var_rsourcegeo_dn8 + var_rdraingeo_dn8) + var_rdsi0_dn8))), ((var_rdstemp_dn9 * assign33810_e56481) + (var_rdstemp * ((var_rsourcegeo_dn9 + var_rdraingeo_dn9) + var_rdsi0_dn9))), ((var_rdstemp_dn10 * assign33810_e56481) + (var_rdstemp * ((var_rsourcegeo_dn10 + var_rdraingeo_dn10) + var_rdsi0_dn10))), ((var_rdstemp_dn11 * assign33810_e56481) + (var_rdstemp * ((var_rsourcegeo_dn11 + var_rdraingeo_dn11) + var_rdsi0_dn11))), ((var_rdstemp_dn13 * assign33810_e56481) + (var_rdstemp * ((var_rsourcegeo_dn13 + var_rdraingeo_dn13) + var_rdsi0_dn13))), ((var_rdstemp_dn14 * assign33810_e56481) + (var_rdstemp * ((var_rsourcegeo_dn14 + var_rdraingeo_dn14) + var_rdsi0_dn14))),)
    } else {
        (var_rdsi0, var_rdsi0_dn0, var_rdsi0_dn2, var_rdsi0_dn3, var_rdsi0_dn4, var_rdsi0_dn5, var_rdsi0_dn6, var_rdsi0_dn7, var_rdsi0_dn8, var_rdsi0_dn9, var_rdsi0_dn10, var_rdsi0_dn11, var_rdsi0_dn13, var_rdsi0_dn14,)
    }
};
        var_rdsi0 = assign33810_e56484;
        var_rdsi0_dn0 = assign33810_e56484_d_n0;
        var_rdsi0_dn2 = assign33810_e56484_d_n2;
        var_rdsi0_dn3 = assign33810_e56484_d_n3;
        var_rdsi0_dn4 = assign33810_e56484_d_n4;
        var_rdsi0_dn5 = assign33810_e56484_d_n5;
        var_rdsi0_dn6 = assign33810_e56484_d_n6;
        var_rdsi0_dn7 = assign33810_e56484_d_n7;
        var_rdsi0_dn8 = assign33810_e56484_d_n8;
        var_rdsi0_dn9 = assign33810_e56484_d_n9;
        var_rdsi0_dn10 = assign33810_e56484_d_n10;
        var_rdsi0_dn11 = assign33810_e56484_d_n11;
        var_rdsi0_dn13 = assign33810_e56484_d_n13;
        var_rdsi0_dn14 = assign33810_e56484_d_n14;

        let (assign33820_e56510, assign33820_e56510_d_n0, assign33820_e56510_d_n2, assign33820_e56510_d_n3, assign33820_e56510_d_n4, assign33820_e56510_d_n5, assign33820_e56510_d_n6, assign33820_e56510_d_n7, assign33820_e56510_d_n8, assign33820_e56510_d_n9, assign33820_e56510_d_n10, assign33820_e56510_d_n11, assign33820_e56510_d_n13, assign33820_e56510_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && ((var_guard638 != 0.0) && (!((var_guard636 != 0.0) || (var_guard637 != 0.0))))) {
        let assign33820_e56499: f64 = (var_nfintotal * var_beta_v);
        let assign33820_e56501: f64 = (assign33820_e56499 * var_ids0_ov_dqi0);
        let assign33820_e56504: f64 = (var_dmob0 * var_dvsat0);
        let assign33820_e56505: f64 = (assign33820_e56501 / assign33820_e56504);
        let assign33820_e56507: f64 = (assign33820_e56505 * var_rdsi0);
        let assign33820_e56508: f64 = (1.0 + assign33820_e56507);
        (assign33820_e56508, ((((((((var_nfintotal * var_beta_v_dn0) * var_ids0_ov_dqi0) + (assign33820_e56499 * var_ids0_ov_dqi0_dn0)) * assign33820_e56504) - (assign33820_e56501 * (var_dmob0_dn0 * var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * var_rdsi0) + (assign33820_e56505 * var_rdsi0_dn0)), ((((((((var_nfintotal * var_beta_v_dn2) * var_ids0_ov_dqi0) + (assign33820_e56499 * var_ids0_ov_dqi0_dn2)) * assign33820_e56504) - (assign33820_e56501 * (var_dmob0_dn2 * var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * var_rdsi0) + (assign33820_e56505 * var_rdsi0_dn2)), ((((((((var_nfintotal * var_beta_v_dn3) * var_ids0_ov_dqi0) + (assign33820_e56499 * var_ids0_ov_dqi0_dn3)) * assign33820_e56504) - (assign33820_e56501 * (var_dmob0_dn3 * var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * var_rdsi0) + (assign33820_e56505 * var_rdsi0_dn3)), ((((((((var_nfintotal * var_beta_v_dn4) * var_ids0_ov_dqi0) + (assign33820_e56499 * var_ids0_ov_dqi0_dn4)) * assign33820_e56504) - (assign33820_e56501 * (var_dmob0_dn4 * var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * var_rdsi0) + (assign33820_e56505 * var_rdsi0_dn4)), ((((((((var_nfintotal * var_beta_v_dn5) * var_ids0_ov_dqi0) + (assign33820_e56499 * var_ids0_ov_dqi0_dn5)) * assign33820_e56504) - (assign33820_e56501 * (var_dmob0_dn5 * var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * var_rdsi0) + (assign33820_e56505 * var_rdsi0_dn5)), ((((((((var_nfintotal * var_beta_v_dn6) * var_ids0_ov_dqi0) + (assign33820_e56499 * var_ids0_ov_dqi0_dn6)) * assign33820_e56504) - (assign33820_e56501 * (var_dmob0_dn6 * var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * var_rdsi0) + (assign33820_e56505 * var_rdsi0_dn6)), ((((((((var_nfintotal * var_beta_v_dn7) * var_ids0_ov_dqi0) + (assign33820_e56499 * var_ids0_ov_dqi0_dn7)) * assign33820_e56504) - (assign33820_e56501 * (var_dmob0_dn7 * var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * var_rdsi0) + (assign33820_e56505 * var_rdsi0_dn7)), ((((((((var_nfintotal * var_beta_v_dn8) * var_ids0_ov_dqi0) + (assign33820_e56499 * var_ids0_ov_dqi0_dn8)) * assign33820_e56504) - (assign33820_e56501 * (var_dmob0_dn8 * var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * var_rdsi0) + (assign33820_e56505 * var_rdsi0_dn8)), ((((((((var_nfintotal * var_beta_v_dn9) * var_ids0_ov_dqi0) + (assign33820_e56499 * var_ids0_ov_dqi0_dn9)) * assign33820_e56504) - (assign33820_e56501 * (var_dmob0_dn9 * var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * var_rdsi0) + (assign33820_e56505 * var_rdsi0_dn9)), ((((((((var_nfintotal * var_beta_v_dn10) * var_ids0_ov_dqi0) + (assign33820_e56499 * var_ids0_ov_dqi0_dn10)) * assign33820_e56504) - (assign33820_e56501 * (var_dmob0_dn10 * var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * var_rdsi0) + (assign33820_e56505 * var_rdsi0_dn10)), ((((((((var_nfintotal * var_beta_v_dn11) * var_ids0_ov_dqi0) + (assign33820_e56499 * var_ids0_ov_dqi0_dn11)) * assign33820_e56504) - (assign33820_e56501 * (var_dmob0_dn11 * var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * var_rdsi0) + (assign33820_e56505 * var_rdsi0_dn11)), ((((((((var_nfintotal * var_beta_v_dn13) * var_ids0_ov_dqi0) + (assign33820_e56499 * var_ids0_ov_dqi0_dn13)) * assign33820_e56504) - (assign33820_e56501 * (var_dmob0_dn13 * var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * var_rdsi0) + (assign33820_e56505 * var_rdsi0_dn13)), ((((((((var_nfintotal * var_beta_v_dn14) * var_ids0_ov_dqi0) + (assign33820_e56499 * var_ids0_ov_dqi0_dn14)) * assign33820_e56504) - (assign33820_e56501 * (var_dmob0_dn14 * var_dvsat0))) / (assign33820_e56504 * assign33820_e56504)) * var_rdsi0) + (assign33820_e56505 * var_rdsi0_dn14)),)
    } else {
        (var_dr0, var_dr0_dn0, var_dr0_dn2, var_dr0_dn3, var_dr0_dn4, var_dr0_dn5, var_dr0_dn6, var_dr0_dn7, var_dr0_dn8, var_dr0_dn9, var_dr0_dn10, var_dr0_dn11, var_dr0_dn13, var_dr0_dn14,)
    }
};
        var_dr0 = assign33820_e56510;
        var_dr0_dn0 = assign33820_e56510_d_n0;
        var_dr0_dn2 = assign33820_e56510_d_n2;
        var_dr0_dn3 = assign33820_e56510_d_n3;
        var_dr0_dn4 = assign33820_e56510_d_n4;
        var_dr0_dn5 = assign33820_e56510_d_n5;
        var_dr0_dn6 = assign33820_e56510_d_n6;
        var_dr0_dn7 = assign33820_e56510_d_n7;
        var_dr0_dn8 = assign33820_e56510_d_n8;
        var_dr0_dn9 = assign33820_e56510_d_n9;
        var_dr0_dn10 = assign33820_e56510_d_n10;
        var_dr0_dn11 = assign33820_e56510_d_n11;
        var_dr0_dn13 = assign33820_e56510_d_n13;
        var_dr0_dn14 = assign33820_e56510_d_n14;

        let (assign33830_e56531, assign33830_e56531_d_n0, assign33830_e56531_d_n2, assign33830_e56531_d_n3, assign33830_e56531_d_n4, assign33830_e56531_d_n5, assign33830_e56531_d_n6, assign33830_e56531_d_n7, assign33830_e56531_d_n8, assign33830_e56531_d_n9, assign33830_e56531_d_n10, assign33830_e56531_d_n11, assign33830_e56531_d_n13, assign33830_e56531_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33830_e56517: f64 = (var_nfintotal * var_beta_v);
        let assign33830_e56519: f64 = (assign33830_e56517 * var_qis);
        let assign33830_e56521: f64 = (assign33830_e56519 * var_mnud0);
        let assign33830_e56523: f64 = (assign33830_e56521 * var_mob0);
        let assign33830_e56526: f64 = (var_dmob0 * var_dvsat0);
        let assign33830_e56528: f64 = (assign33830_e56526 * var_dr0);
        let assign33830_e56529: f64 = (assign33830_e56523 / assign33830_e56528);
        (assign33830_e56529, ((((((((((var_nfintotal * var_beta_v_dn0) * var_qis) + (assign33830_e56517 * var_qis_dn0)) * var_mnud0) + (assign33830_e56519 * var_mnud0_dn0)) * var_mob0) + (assign33830_e56521 * var_mob0_dn0)) * assign33830_e56528) - (assign33830_e56523 * (((var_dmob0_dn0 * var_dvsat0) * var_dr0) + (assign33830_e56526 * var_dr0_dn0)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((var_nfintotal * var_beta_v_dn2) * var_qis) + (assign33830_e56517 * var_qis_dn2)) * var_mnud0) + (assign33830_e56519 * var_mnud0_dn2)) * var_mob0) + (assign33830_e56521 * var_mob0_dn2)) * assign33830_e56528) - (assign33830_e56523 * (((var_dmob0_dn2 * var_dvsat0) * var_dr0) + (assign33830_e56526 * var_dr0_dn2)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((var_nfintotal * var_beta_v_dn3) * var_qis) + (assign33830_e56517 * var_qis_dn3)) * var_mnud0) + (assign33830_e56519 * var_mnud0_dn3)) * var_mob0) + (assign33830_e56521 * var_mob0_dn3)) * assign33830_e56528) - (assign33830_e56523 * (((var_dmob0_dn3 * var_dvsat0) * var_dr0) + (assign33830_e56526 * var_dr0_dn3)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((var_nfintotal * var_beta_v_dn4) * var_qis) + (assign33830_e56517 * var_qis_dn4)) * var_mnud0) + (assign33830_e56519 * var_mnud0_dn4)) * var_mob0) + (assign33830_e56521 * var_mob0_dn4)) * assign33830_e56528) - (assign33830_e56523 * (((var_dmob0_dn4 * var_dvsat0) * var_dr0) + (assign33830_e56526 * var_dr0_dn4)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((var_nfintotal * var_beta_v_dn5) * var_qis) + (assign33830_e56517 * var_qis_dn5)) * var_mnud0) + (assign33830_e56519 * var_mnud0_dn5)) * var_mob0) + (assign33830_e56521 * var_mob0_dn5)) * assign33830_e56528) - (assign33830_e56523 * (((var_dmob0_dn5 * var_dvsat0) * var_dr0) + (assign33830_e56526 * var_dr0_dn5)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((var_nfintotal * var_beta_v_dn6) * var_qis) + (assign33830_e56517 * var_qis_dn6)) * var_mnud0) + (assign33830_e56519 * var_mnud0_dn6)) * var_mob0) + (assign33830_e56521 * var_mob0_dn6)) * assign33830_e56528) - (assign33830_e56523 * (((var_dmob0_dn6 * var_dvsat0) * var_dr0) + (assign33830_e56526 * var_dr0_dn6)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((var_nfintotal * var_beta_v_dn7) * var_qis) + (assign33830_e56517 * var_qis_dn7)) * var_mnud0) + (assign33830_e56519 * var_mnud0_dn7)) * var_mob0) + (assign33830_e56521 * var_mob0_dn7)) * assign33830_e56528) - (assign33830_e56523 * (((var_dmob0_dn7 * var_dvsat0) * var_dr0) + (assign33830_e56526 * var_dr0_dn7)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((var_nfintotal * var_beta_v_dn8) * var_qis) + (assign33830_e56517 * var_qis_dn8)) * var_mnud0) + (assign33830_e56519 * var_mnud0_dn8)) * var_mob0) + (assign33830_e56521 * var_mob0_dn8)) * assign33830_e56528) - (assign33830_e56523 * (((var_dmob0_dn8 * var_dvsat0) * var_dr0) + (assign33830_e56526 * var_dr0_dn8)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((var_nfintotal * var_beta_v_dn9) * var_qis) + (assign33830_e56517 * var_qis_dn9)) * var_mnud0) + (assign33830_e56519 * var_mnud0_dn9)) * var_mob0) + (assign33830_e56521 * var_mob0_dn9)) * assign33830_e56528) - (assign33830_e56523 * (((var_dmob0_dn9 * var_dvsat0) * var_dr0) + (assign33830_e56526 * var_dr0_dn9)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((var_nfintotal * var_beta_v_dn10) * var_qis) + (assign33830_e56517 * var_qis_dn10)) * var_mnud0) + (assign33830_e56519 * var_mnud0_dn10)) * var_mob0) + (assign33830_e56521 * var_mob0_dn10)) * assign33830_e56528) - (assign33830_e56523 * (((var_dmob0_dn10 * var_dvsat0) * var_dr0) + (assign33830_e56526 * var_dr0_dn10)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((var_nfintotal * var_beta_v_dn11) * var_qis) + (assign33830_e56517 * var_qis_dn11)) * var_mnud0) + (assign33830_e56519 * var_mnud0_dn11)) * var_mob0) + (assign33830_e56521 * var_mob0_dn11)) * assign33830_e56528) - (assign33830_e56523 * (((var_dmob0_dn11 * var_dvsat0) * var_dr0) + (assign33830_e56526 * var_dr0_dn11)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((var_nfintotal * var_beta_v_dn13) * var_qis) + (assign33830_e56517 * var_qis_dn13)) * var_mnud0) + (assign33830_e56519 * var_mnud0_dn13)) * var_mob0) + (assign33830_e56521 * var_mob0_dn13)) * assign33830_e56528) - (assign33830_e56523 * (((var_dmob0_dn13 * var_dvsat0) * var_dr0) + (assign33830_e56526 * var_dr0_dn13)))) / (assign33830_e56528 * assign33830_e56528)), ((((((((((var_nfintotal * var_beta_v_dn14) * var_qis) + (assign33830_e56517 * var_qis_dn14)) * var_mnud0) + (assign33830_e56519 * var_mnud0_dn14)) * var_mob0) + (assign33830_e56521 * var_mob0_dn14)) * assign33830_e56528) - (assign33830_e56523 * (((var_dmob0_dn14 * var_dvsat0) * var_dr0) + (assign33830_e56526 * var_dr0_dn14)))) / (assign33830_e56528 * assign33830_e56528)),)
    } else {
        (var_noigd0, var_noigd0_dn0, var_noigd0_dn2, var_noigd0_dn3, var_noigd0_dn4, var_noigd0_dn5, var_noigd0_dn6, var_noigd0_dn7, var_noigd0_dn8, var_noigd0_dn9, var_noigd0_dn10, var_noigd0_dn11, var_noigd0_dn13, var_noigd0_dn14,)
    }
};
        var_noigd0 = assign33830_e56531;
        var_noigd0_dn0 = assign33830_e56531_d_n0;
        var_noigd0_dn2 = assign33830_e56531_d_n2;
        var_noigd0_dn3 = assign33830_e56531_d_n3;
        var_noigd0_dn4 = assign33830_e56531_d_n4;
        var_noigd0_dn5 = assign33830_e56531_d_n5;
        var_noigd0_dn6 = assign33830_e56531_d_n6;
        var_noigd0_dn7 = assign33830_e56531_d_n7;
        var_noigd0_dn8 = assign33830_e56531_d_n8;
        var_noigd0_dn9 = assign33830_e56531_d_n9;
        var_noigd0_dn10 = assign33830_e56531_d_n10;
        var_noigd0_dn11 = assign33830_e56531_d_n11;
        var_noigd0_dn13 = assign33830_e56531_d_n13;
        var_noigd0_dn14 = assign33830_e56531_d_n14;

        let (assign33840_e56540, assign33840_e56540_d_n0, assign33840_e56540_d_n2, assign33840_e56540_d_n3, assign33840_e56540_d_n4, assign33840_e56540_d_n5, assign33840_e56540_d_n6, assign33840_e56540_d_n7, assign33840_e56540_d_n8, assign33840_e56540_d_n9, assign33840_e56540_d_n10, assign33840_e56540_d_n11, assign33840_e56540_d_n13, assign33840_e56540_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33840_e56538: f64 = (1.0 + var_noieta);
        (assign33840_e56538, var_noieta_dn0, var_noieta_dn2, var_noieta_dn3, var_noieta_dn4, var_noieta_dn5, var_noieta_dn6, var_noieta_dn7, var_noieta_dn8, var_noieta_dn9, var_noieta_dn10, var_noieta_dn11, var_noieta_dn13, var_noieta_dn14,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11, var_t4_dn13, var_t4_dn14,)
    }
};
        var_t4 = assign33840_e56540;
        var_t4_dn0 = assign33840_e56540_d_n0;
        var_t4_dn2 = assign33840_e56540_d_n2;
        var_t4_dn3 = assign33840_e56540_d_n3;
        var_t4_dn4 = assign33840_e56540_d_n4;
        var_t4_dn5 = assign33840_e56540_d_n5;
        var_t4_dn6 = assign33840_e56540_d_n6;
        var_t4_dn7 = assign33840_e56540_d_n7;
        var_t4_dn8 = assign33840_e56540_d_n8;
        var_t4_dn9 = assign33840_e56540_d_n9;
        var_t4_dn10 = assign33840_e56540_d_n10;
        var_t4_dn11 = assign33840_e56540_d_n11;
        var_t4_dn13 = assign33840_e56540_d_n13;
        var_t4_dn14 = assign33840_e56540_d_n14;

        let (assign33850_e56549, assign33850_e56549_d_n0, assign33850_e56549_d_n2, assign33850_e56549_d_n3, assign33850_e56549_d_n4, assign33850_e56549_d_n5, assign33850_e56549_d_n6, assign33850_e56549_d_n7, assign33850_e56549_d_n8, assign33850_e56549_d_n9, assign33850_e56549_d_n10, assign33850_e56549_d_n11, assign33850_e56549_d_n13, assign33850_e56549_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33850_e56547: f64 = (1.0 - var_noieta);
        (assign33850_e56547, (-var_noieta_dn0), (-var_noieta_dn2), (-var_noieta_dn3), (-var_noieta_dn4), (-var_noieta_dn5), (-var_noieta_dn6), (-var_noieta_dn7), (-var_noieta_dn8), (-var_noieta_dn9), (-var_noieta_dn10), (-var_noieta_dn11), (-var_noieta_dn13), (-var_noieta_dn14),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11, var_t5_dn13, var_t5_dn14,)
    }
};
        var_t5 = assign33850_e56549;
        var_t5_dn0 = assign33850_e56549_d_n0;
        var_t5_dn2 = assign33850_e56549_d_n2;
        var_t5_dn3 = assign33850_e56549_d_n3;
        var_t5_dn4 = assign33850_e56549_d_n4;
        var_t5_dn5 = assign33850_e56549_d_n5;
        var_t5_dn6 = assign33850_e56549_d_n6;
        var_t5_dn7 = assign33850_e56549_d_n7;
        var_t5_dn8 = assign33850_e56549_d_n8;
        var_t5_dn9 = assign33850_e56549_d_n9;
        var_t5_dn10 = assign33850_e56549_d_n10;
        var_t5_dn11 = assign33850_e56549_d_n11;
        var_t5_dn13 = assign33850_e56549_d_n13;
        var_t5_dn14 = assign33850_e56549_d_n14;

        let (assign33860_e56562, assign33860_e56562_d_n0, assign33860_e56562_d_n2, assign33860_e56562_d_n3, assign33860_e56562_d_n4, assign33860_e56562_d_n5, assign33860_e56562_d_n6, assign33860_e56562_d_n7, assign33860_e56562_d_n8, assign33860_e56562_d_n9, assign33860_e56562_d_n10, assign33860_e56562_d_n11, assign33860_e56562_d_n13, assign33860_e56562_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33860_e56556: f64 = (2.0 * var_noiwi);
        let assign33860_e56558: f64 = (assign33860_e56556 / var_qis);
        let assign33860_e56560: f64 = (assign33860_e56558 * var_nvtm);
        (assign33860_e56560, ((((((2.0 * var_noiwi_dn0) * var_qis) - (assign33860_e56556 * var_qis_dn0)) / (var_qis * var_qis)) * var_nvtm) + (assign33860_e56558 * var_nvtm_dn0)), ((((((2.0 * var_noiwi_dn2) * var_qis) - (assign33860_e56556 * var_qis_dn2)) / (var_qis * var_qis)) * var_nvtm) + (assign33860_e56558 * var_nvtm_dn2)), ((((((2.0 * var_noiwi_dn3) * var_qis) - (assign33860_e56556 * var_qis_dn3)) / (var_qis * var_qis)) * var_nvtm) + (assign33860_e56558 * var_nvtm_dn3)), ((((((2.0 * var_noiwi_dn4) * var_qis) - (assign33860_e56556 * var_qis_dn4)) / (var_qis * var_qis)) * var_nvtm) + (assign33860_e56558 * var_nvtm_dn4)), ((((((2.0 * var_noiwi_dn5) * var_qis) - (assign33860_e56556 * var_qis_dn5)) / (var_qis * var_qis)) * var_nvtm) + (assign33860_e56558 * var_nvtm_dn5)), ((((((2.0 * var_noiwi_dn6) * var_qis) - (assign33860_e56556 * var_qis_dn6)) / (var_qis * var_qis)) * var_nvtm) + (assign33860_e56558 * var_nvtm_dn6)), ((((((2.0 * var_noiwi_dn7) * var_qis) - (assign33860_e56556 * var_qis_dn7)) / (var_qis * var_qis)) * var_nvtm) + (assign33860_e56558 * var_nvtm_dn7)), ((((((2.0 * var_noiwi_dn8) * var_qis) - (assign33860_e56556 * var_qis_dn8)) / (var_qis * var_qis)) * var_nvtm) + (assign33860_e56558 * var_nvtm_dn8)), ((((((2.0 * var_noiwi_dn9) * var_qis) - (assign33860_e56556 * var_qis_dn9)) / (var_qis * var_qis)) * var_nvtm) + (assign33860_e56558 * var_nvtm_dn9)), ((((((2.0 * var_noiwi_dn10) * var_qis) - (assign33860_e56556 * var_qis_dn10)) / (var_qis * var_qis)) * var_nvtm) + (assign33860_e56558 * var_nvtm_dn10)), ((((((2.0 * var_noiwi_dn11) * var_qis) - (assign33860_e56556 * var_qis_dn11)) / (var_qis * var_qis)) * var_nvtm) + (assign33860_e56558 * var_nvtm_dn11)), ((((((2.0 * var_noiwi_dn13) * var_qis) - (assign33860_e56556 * var_qis_dn13)) / (var_qis * var_qis)) * var_nvtm) + (assign33860_e56558 * var_nvtm_dn13)), ((((((2.0 * var_noiwi_dn14) * var_qis) - (assign33860_e56556 * var_qis_dn14)) / (var_qis * var_qis)) * var_nvtm) + (assign33860_e56558 * var_nvtm_dn14)),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn13, var_t6_dn14,)
    }
};
        var_t6 = assign33860_e56562;
        var_t6_dn0 = assign33860_e56562_d_n0;
        var_t6_dn2 = assign33860_e56562_d_n2;
        var_t6_dn3 = assign33860_e56562_d_n3;
        var_t6_dn4 = assign33860_e56562_d_n4;
        var_t6_dn5 = assign33860_e56562_d_n5;
        var_t6_dn6 = assign33860_e56562_d_n6;
        var_t6_dn7 = assign33860_e56562_d_n7;
        var_t6_dn8 = assign33860_e56562_d_n8;
        var_t6_dn9 = assign33860_e56562_d_n9;
        var_t6_dn10 = assign33860_e56562_d_n10;
        var_t6_dn11 = assign33860_e56562_d_n11;
        var_t6_dn13 = assign33860_e56562_d_n13;
        var_t6_dn14 = assign33860_e56562_d_n14;

        let (assign33870_e56571, assign33870_e56571_d_n0, assign33870_e56571_d_n2, assign33870_e56571_d_n3, assign33870_e56571_d_n4, assign33870_e56571_d_n5, assign33870_e56571_d_n6, assign33870_e56571_d_n7, assign33870_e56571_d_n8, assign33870_e56571_d_n9, assign33870_e56571_d_n10, assign33870_e56571_d_n11, assign33870_e56571_d_n13, assign33870_e56571_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33870_e56569: f64 = (var_t4 + var_t6);
        (assign33870_e56569, (var_t4_dn0 + var_t6_dn0), (var_t4_dn2 + var_t6_dn2), (var_t4_dn3 + var_t6_dn3), (var_t4_dn4 + var_t6_dn4), (var_t4_dn5 + var_t6_dn5), (var_t4_dn6 + var_t6_dn6), (var_t4_dn7 + var_t6_dn7), (var_t4_dn8 + var_t6_dn8), (var_t4_dn9 + var_t6_dn9), (var_t4_dn10 + var_t6_dn10), (var_t4_dn11 + var_t6_dn11), (var_t4_dn13 + var_t6_dn13), (var_t4_dn14 + var_t6_dn14),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn3, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11, var_t7_dn13, var_t7_dn14,)
    }
};
        var_t7 = assign33870_e56571;
        var_t7_dn0 = assign33870_e56571_d_n0;
        var_t7_dn2 = assign33870_e56571_d_n2;
        var_t7_dn3 = assign33870_e56571_d_n3;
        var_t7_dn4 = assign33870_e56571_d_n4;
        var_t7_dn5 = assign33870_e56571_d_n5;
        var_t7_dn6 = assign33870_e56571_d_n6;
        var_t7_dn7 = assign33870_e56571_d_n7;
        var_t7_dn8 = assign33870_e56571_d_n8;
        var_t7_dn9 = assign33870_e56571_d_n9;
        var_t7_dn10 = assign33870_e56571_d_n10;
        var_t7_dn11 = assign33870_e56571_d_n11;
        var_t7_dn13 = assign33870_e56571_d_n13;
        var_t7_dn14 = assign33870_e56571_d_n14;

        let (assign33880_e56580, assign33880_e56580_d_n0, assign33880_e56580_d_n2, assign33880_e56580_d_n3, assign33880_e56580_d_n4, assign33880_e56580_d_n5, assign33880_e56580_d_n6, assign33880_e56580_d_n7, assign33880_e56580_d_n8, assign33880_e56580_d_n9, assign33880_e56580_d_n10, assign33880_e56580_d_n11, assign33880_e56580_d_n13, assign33880_e56580_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33880_e56578: f64 = (var_t5 * var_t5);
        (assign33880_e56578, ((var_t5_dn0 * var_t5) + (var_t5 * var_t5_dn0)), ((var_t5_dn2 * var_t5) + (var_t5 * var_t5_dn2)), ((var_t5_dn3 * var_t5) + (var_t5 * var_t5_dn3)), ((var_t5_dn4 * var_t5) + (var_t5 * var_t5_dn4)), ((var_t5_dn5 * var_t5) + (var_t5 * var_t5_dn5)), ((var_t5_dn6 * var_t5) + (var_t5 * var_t5_dn6)), ((var_t5_dn7 * var_t5) + (var_t5 * var_t5_dn7)), ((var_t5_dn8 * var_t5) + (var_t5 * var_t5_dn8)), ((var_t5_dn9 * var_t5) + (var_t5 * var_t5_dn9)), ((var_t5_dn10 * var_t5) + (var_t5 * var_t5_dn10)), ((var_t5_dn11 * var_t5) + (var_t5 * var_t5_dn11)), ((var_t5_dn13 * var_t5) + (var_t5 * var_t5_dn13)), ((var_t5_dn14 * var_t5) + (var_t5 * var_t5_dn14)),)
    } else {
        (var_t5_2, var_t5_2_dn0, var_t5_2_dn2, var_t5_2_dn3, var_t5_2_dn4, var_t5_2_dn5, var_t5_2_dn6, var_t5_2_dn7, var_t5_2_dn8, var_t5_2_dn9, var_t5_2_dn10, var_t5_2_dn11, var_t5_2_dn13, var_t5_2_dn14,)
    }
};
        var_t5_2 = assign33880_e56580;
        var_t5_2_dn0 = assign33880_e56580_d_n0;
        var_t5_2_dn2 = assign33880_e56580_d_n2;
        var_t5_2_dn3 = assign33880_e56580_d_n3;
        var_t5_2_dn4 = assign33880_e56580_d_n4;
        var_t5_2_dn5 = assign33880_e56580_d_n5;
        var_t5_2_dn6 = assign33880_e56580_d_n6;
        var_t5_2_dn7 = assign33880_e56580_d_n7;
        var_t5_2_dn8 = assign33880_e56580_d_n8;
        var_t5_2_dn9 = assign33880_e56580_d_n9;
        var_t5_2_dn10 = assign33880_e56580_d_n10;
        var_t5_2_dn11 = assign33880_e56580_d_n11;
        var_t5_2_dn13 = assign33880_e56580_d_n13;
        var_t5_2_dn14 = assign33880_e56580_d_n14;

        let (assign33890_e56589, assign33890_e56589_d_n0, assign33890_e56589_d_n2, assign33890_e56589_d_n3, assign33890_e56589_d_n4, assign33890_e56589_d_n5, assign33890_e56589_d_n6, assign33890_e56589_d_n7, assign33890_e56589_d_n8, assign33890_e56589_d_n9, assign33890_e56589_d_n10, assign33890_e56589_d_n11, assign33890_e56589_d_n13, assign33890_e56589_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33890_e56587: f64 = (var_t5_2 * var_t5);
        (assign33890_e56587, ((var_t5_2_dn0 * var_t5) + (var_t5_2 * var_t5_dn0)), ((var_t5_2_dn2 * var_t5) + (var_t5_2 * var_t5_dn2)), ((var_t5_2_dn3 * var_t5) + (var_t5_2 * var_t5_dn3)), ((var_t5_2_dn4 * var_t5) + (var_t5_2 * var_t5_dn4)), ((var_t5_2_dn5 * var_t5) + (var_t5_2 * var_t5_dn5)), ((var_t5_2_dn6 * var_t5) + (var_t5_2 * var_t5_dn6)), ((var_t5_2_dn7 * var_t5) + (var_t5_2 * var_t5_dn7)), ((var_t5_2_dn8 * var_t5) + (var_t5_2 * var_t5_dn8)), ((var_t5_2_dn9 * var_t5) + (var_t5_2 * var_t5_dn9)), ((var_t5_2_dn10 * var_t5) + (var_t5_2 * var_t5_dn10)), ((var_t5_2_dn11 * var_t5) + (var_t5_2 * var_t5_dn11)), ((var_t5_2_dn13 * var_t5) + (var_t5_2 * var_t5_dn13)), ((var_t5_2_dn14 * var_t5) + (var_t5_2 * var_t5_dn14)),)
    } else {
        (var_t5_3, var_t5_3_dn0, var_t5_3_dn2, var_t5_3_dn3, var_t5_3_dn4, var_t5_3_dn5, var_t5_3_dn6, var_t5_3_dn7, var_t5_3_dn8, var_t5_3_dn9, var_t5_3_dn10, var_t5_3_dn11, var_t5_3_dn13, var_t5_3_dn14,)
    }
};
        var_t5_3 = assign33890_e56589;
        var_t5_3_dn0 = assign33890_e56589_d_n0;
        var_t5_3_dn2 = assign33890_e56589_d_n2;
        var_t5_3_dn3 = assign33890_e56589_d_n3;
        var_t5_3_dn4 = assign33890_e56589_d_n4;
        var_t5_3_dn5 = assign33890_e56589_d_n5;
        var_t5_3_dn6 = assign33890_e56589_d_n6;
        var_t5_3_dn7 = assign33890_e56589_d_n7;
        var_t5_3_dn8 = assign33890_e56589_d_n8;
        var_t5_3_dn9 = assign33890_e56589_d_n9;
        var_t5_3_dn10 = assign33890_e56589_d_n10;
        var_t5_3_dn11 = assign33890_e56589_d_n11;
        var_t5_3_dn13 = assign33890_e56589_d_n13;
        var_t5_3_dn14 = assign33890_e56589_d_n14;

        let (assign33900_e56598, assign33900_e56598_d_n0, assign33900_e56598_d_n2, assign33900_e56598_d_n3, assign33900_e56598_d_n4, assign33900_e56598_d_n5, assign33900_e56598_d_n6, assign33900_e56598_d_n7, assign33900_e56598_d_n8, assign33900_e56598_d_n9, assign33900_e56598_d_n10, assign33900_e56598_d_n11, assign33900_e56598_d_n13, assign33900_e56598_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33900_e56596: f64 = (var_t5_3 * var_t5);
        (assign33900_e56596, ((var_t5_3_dn0 * var_t5) + (var_t5_3 * var_t5_dn0)), ((var_t5_3_dn2 * var_t5) + (var_t5_3 * var_t5_dn2)), ((var_t5_3_dn3 * var_t5) + (var_t5_3 * var_t5_dn3)), ((var_t5_3_dn4 * var_t5) + (var_t5_3 * var_t5_dn4)), ((var_t5_3_dn5 * var_t5) + (var_t5_3 * var_t5_dn5)), ((var_t5_3_dn6 * var_t5) + (var_t5_3 * var_t5_dn6)), ((var_t5_3_dn7 * var_t5) + (var_t5_3 * var_t5_dn7)), ((var_t5_3_dn8 * var_t5) + (var_t5_3 * var_t5_dn8)), ((var_t5_3_dn9 * var_t5) + (var_t5_3 * var_t5_dn9)), ((var_t5_3_dn10 * var_t5) + (var_t5_3 * var_t5_dn10)), ((var_t5_3_dn11 * var_t5) + (var_t5_3 * var_t5_dn11)), ((var_t5_3_dn13 * var_t5) + (var_t5_3 * var_t5_dn13)), ((var_t5_3_dn14 * var_t5) + (var_t5_3 * var_t5_dn14)),)
    } else {
        (var_t5_4, var_t5_4_dn0, var_t5_4_dn2, var_t5_4_dn3, var_t5_4_dn4, var_t5_4_dn5, var_t5_4_dn6, var_t5_4_dn7, var_t5_4_dn8, var_t5_4_dn9, var_t5_4_dn10, var_t5_4_dn11, var_t5_4_dn13, var_t5_4_dn14,)
    }
};
        var_t5_4 = assign33900_e56598;
        var_t5_4_dn0 = assign33900_e56598_d_n0;
        var_t5_4_dn2 = assign33900_e56598_d_n2;
        var_t5_4_dn3 = assign33900_e56598_d_n3;
        var_t5_4_dn4 = assign33900_e56598_d_n4;
        var_t5_4_dn5 = assign33900_e56598_d_n5;
        var_t5_4_dn6 = assign33900_e56598_d_n6;
        var_t5_4_dn7 = assign33900_e56598_d_n7;
        var_t5_4_dn8 = assign33900_e56598_d_n8;
        var_t5_4_dn9 = assign33900_e56598_d_n9;
        var_t5_4_dn10 = assign33900_e56598_d_n10;
        var_t5_4_dn11 = assign33900_e56598_d_n11;
        var_t5_4_dn13 = assign33900_e56598_d_n13;
        var_t5_4_dn14 = assign33900_e56598_d_n14;

        let (assign33910_e56607, assign33910_e56607_d_n0, assign33910_e56607_d_n2, assign33910_e56607_d_n3, assign33910_e56607_d_n4, assign33910_e56607_d_n5, assign33910_e56607_d_n6, assign33910_e56607_d_n7, assign33910_e56607_d_n8, assign33910_e56607_d_n9, assign33910_e56607_d_n10, assign33910_e56607_d_n11, assign33910_e56607_d_n13, assign33910_e56607_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33910_e56605: f64 = (var_t7 * var_t7);
        (assign33910_e56605, ((var_t7_dn0 * var_t7) + (var_t7 * var_t7_dn0)), ((var_t7_dn2 * var_t7) + (var_t7 * var_t7_dn2)), ((var_t7_dn3 * var_t7) + (var_t7 * var_t7_dn3)), ((var_t7_dn4 * var_t7) + (var_t7 * var_t7_dn4)), ((var_t7_dn5 * var_t7) + (var_t7 * var_t7_dn5)), ((var_t7_dn6 * var_t7) + (var_t7 * var_t7_dn6)), ((var_t7_dn7 * var_t7) + (var_t7 * var_t7_dn7)), ((var_t7_dn8 * var_t7) + (var_t7 * var_t7_dn8)), ((var_t7_dn9 * var_t7) + (var_t7 * var_t7_dn9)), ((var_t7_dn10 * var_t7) + (var_t7 * var_t7_dn10)), ((var_t7_dn11 * var_t7) + (var_t7 * var_t7_dn11)), ((var_t7_dn13 * var_t7) + (var_t7 * var_t7_dn13)), ((var_t7_dn14 * var_t7) + (var_t7 * var_t7_dn14)),)
    } else {
        (var_t7_2, var_t7_2_dn0, var_t7_2_dn2, var_t7_2_dn3, var_t7_2_dn4, var_t7_2_dn5, var_t7_2_dn6, var_t7_2_dn7, var_t7_2_dn8, var_t7_2_dn9, var_t7_2_dn10, var_t7_2_dn11, var_t7_2_dn13, var_t7_2_dn14,)
    }
};
        var_t7_2 = assign33910_e56607;
        var_t7_2_dn0 = assign33910_e56607_d_n0;
        var_t7_2_dn2 = assign33910_e56607_d_n2;
        var_t7_2_dn3 = assign33910_e56607_d_n3;
        var_t7_2_dn4 = assign33910_e56607_d_n4;
        var_t7_2_dn5 = assign33910_e56607_d_n5;
        var_t7_2_dn6 = assign33910_e56607_d_n6;
        var_t7_2_dn7 = assign33910_e56607_d_n7;
        var_t7_2_dn8 = assign33910_e56607_d_n8;
        var_t7_2_dn9 = assign33910_e56607_d_n9;
        var_t7_2_dn10 = assign33910_e56607_d_n10;
        var_t7_2_dn11 = assign33910_e56607_d_n11;
        var_t7_2_dn13 = assign33910_e56607_d_n13;
        var_t7_2_dn14 = assign33910_e56607_d_n14;

        let (assign33920_e56616, assign33920_e56616_d_n0, assign33920_e56616_d_n2, assign33920_e56616_d_n3, assign33920_e56616_d_n4, assign33920_e56616_d_n5, assign33920_e56616_d_n6, assign33920_e56616_d_n7, assign33920_e56616_d_n8, assign33920_e56616_d_n9, assign33920_e56616_d_n10, assign33920_e56616_d_n11, assign33920_e56616_d_n13, assign33920_e56616_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33920_e56614: f64 = (var_t7_2 * var_t7);
        (assign33920_e56614, ((var_t7_2_dn0 * var_t7) + (var_t7_2 * var_t7_dn0)), ((var_t7_2_dn2 * var_t7) + (var_t7_2 * var_t7_dn2)), ((var_t7_2_dn3 * var_t7) + (var_t7_2 * var_t7_dn3)), ((var_t7_2_dn4 * var_t7) + (var_t7_2 * var_t7_dn4)), ((var_t7_2_dn5 * var_t7) + (var_t7_2 * var_t7_dn5)), ((var_t7_2_dn6 * var_t7) + (var_t7_2 * var_t7_dn6)), ((var_t7_2_dn7 * var_t7) + (var_t7_2 * var_t7_dn7)), ((var_t7_2_dn8 * var_t7) + (var_t7_2 * var_t7_dn8)), ((var_t7_2_dn9 * var_t7) + (var_t7_2 * var_t7_dn9)), ((var_t7_2_dn10 * var_t7) + (var_t7_2 * var_t7_dn10)), ((var_t7_2_dn11 * var_t7) + (var_t7_2 * var_t7_dn11)), ((var_t7_2_dn13 * var_t7) + (var_t7_2 * var_t7_dn13)), ((var_t7_2_dn14 * var_t7) + (var_t7_2 * var_t7_dn14)),)
    } else {
        (var_t7_3, var_t7_3_dn0, var_t7_3_dn2, var_t7_3_dn3, var_t7_3_dn4, var_t7_3_dn5, var_t7_3_dn6, var_t7_3_dn7, var_t7_3_dn8, var_t7_3_dn9, var_t7_3_dn10, var_t7_3_dn11, var_t7_3_dn13, var_t7_3_dn14,)
    }
};
        var_t7_3 = assign33920_e56616;
        var_t7_3_dn0 = assign33920_e56616_d_n0;
        var_t7_3_dn2 = assign33920_e56616_d_n2;
        var_t7_3_dn3 = assign33920_e56616_d_n3;
        var_t7_3_dn4 = assign33920_e56616_d_n4;
        var_t7_3_dn5 = assign33920_e56616_d_n5;
        var_t7_3_dn6 = assign33920_e56616_d_n6;
        var_t7_3_dn7 = assign33920_e56616_d_n7;
        var_t7_3_dn8 = assign33920_e56616_d_n8;
        var_t7_3_dn9 = assign33920_e56616_d_n9;
        var_t7_3_dn10 = assign33920_e56616_d_n10;
        var_t7_3_dn11 = assign33920_e56616_d_n11;
        var_t7_3_dn13 = assign33920_e56616_d_n13;
        var_t7_3_dn14 = assign33920_e56616_d_n14;

        let (assign33930_e56625, assign33930_e56625_d_n0, assign33930_e56625_d_n2, assign33930_e56625_d_n3, assign33930_e56625_d_n4, assign33930_e56625_d_n5, assign33930_e56625_d_n6, assign33930_e56625_d_n7, assign33930_e56625_d_n8, assign33930_e56625_d_n9, assign33930_e56625_d_n10, assign33930_e56625_d_n11, assign33930_e56625_d_n13, assign33930_e56625_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33930_e56623: f64 = (var_t7_3 * var_t7);
        (assign33930_e56623, ((var_t7_3_dn0 * var_t7) + (var_t7_3 * var_t7_dn0)), ((var_t7_3_dn2 * var_t7) + (var_t7_3 * var_t7_dn2)), ((var_t7_3_dn3 * var_t7) + (var_t7_3 * var_t7_dn3)), ((var_t7_3_dn4 * var_t7) + (var_t7_3 * var_t7_dn4)), ((var_t7_3_dn5 * var_t7) + (var_t7_3 * var_t7_dn5)), ((var_t7_3_dn6 * var_t7) + (var_t7_3 * var_t7_dn6)), ((var_t7_3_dn7 * var_t7) + (var_t7_3 * var_t7_dn7)), ((var_t7_3_dn8 * var_t7) + (var_t7_3 * var_t7_dn8)), ((var_t7_3_dn9 * var_t7) + (var_t7_3 * var_t7_dn9)), ((var_t7_3_dn10 * var_t7) + (var_t7_3 * var_t7_dn10)), ((var_t7_3_dn11 * var_t7) + (var_t7_3 * var_t7_dn11)), ((var_t7_3_dn13 * var_t7) + (var_t7_3 * var_t7_dn13)), ((var_t7_3_dn14 * var_t7) + (var_t7_3 * var_t7_dn14)),)
    } else {
        (var_t7_4, var_t7_4_dn0, var_t7_4_dn2, var_t7_4_dn3, var_t7_4_dn4, var_t7_4_dn5, var_t7_4_dn6, var_t7_4_dn7, var_t7_4_dn8, var_t7_4_dn9, var_t7_4_dn10, var_t7_4_dn11, var_t7_4_dn13, var_t7_4_dn14,)
    }
};
        var_t7_4 = assign33930_e56625;
        var_t7_4_dn0 = assign33930_e56625_d_n0;
        var_t7_4_dn2 = assign33930_e56625_d_n2;
        var_t7_4_dn3 = assign33930_e56625_d_n3;
        var_t7_4_dn4 = assign33930_e56625_d_n4;
        var_t7_4_dn5 = assign33930_e56625_d_n5;
        var_t7_4_dn6 = assign33930_e56625_d_n6;
        var_t7_4_dn7 = assign33930_e56625_d_n7;
        var_t7_4_dn8 = assign33930_e56625_d_n8;
        var_t7_4_dn9 = assign33930_e56625_d_n9;
        var_t7_4_dn10 = assign33930_e56625_d_n10;
        var_t7_4_dn11 = assign33930_e56625_d_n11;
        var_t7_4_dn13 = assign33930_e56625_d_n13;
        var_t7_4_dn14 = assign33930_e56625_d_n14;

        let (assign33940_e56634, assign33940_e56634_d_n0, assign33940_e56634_d_n2, assign33940_e56634_d_n3, assign33940_e56634_d_n4, assign33940_e56634_d_n5, assign33940_e56634_d_n6, assign33940_e56634_d_n7, assign33940_e56634_d_n8, assign33940_e56634_d_n9, assign33940_e56634_d_n10, assign33940_e56634_d_n11, assign33940_e56634_d_n13, assign33940_e56634_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33940_e56632: f64 = (var_t7_4 * var_t7);
        (assign33940_e56632, ((var_t7_4_dn0 * var_t7) + (var_t7_4 * var_t7_dn0)), ((var_t7_4_dn2 * var_t7) + (var_t7_4 * var_t7_dn2)), ((var_t7_4_dn3 * var_t7) + (var_t7_4 * var_t7_dn3)), ((var_t7_4_dn4 * var_t7) + (var_t7_4 * var_t7_dn4)), ((var_t7_4_dn5 * var_t7) + (var_t7_4 * var_t7_dn5)), ((var_t7_4_dn6 * var_t7) + (var_t7_4 * var_t7_dn6)), ((var_t7_4_dn7 * var_t7) + (var_t7_4 * var_t7_dn7)), ((var_t7_4_dn8 * var_t7) + (var_t7_4 * var_t7_dn8)), ((var_t7_4_dn9 * var_t7) + (var_t7_4 * var_t7_dn9)), ((var_t7_4_dn10 * var_t7) + (var_t7_4 * var_t7_dn10)), ((var_t7_4_dn11 * var_t7) + (var_t7_4 * var_t7_dn11)), ((var_t7_4_dn13 * var_t7) + (var_t7_4 * var_t7_dn13)), ((var_t7_4_dn14 * var_t7) + (var_t7_4 * var_t7_dn14)),)
    } else {
        (var_t7_5, var_t7_5_dn0, var_t7_5_dn2, var_t7_5_dn3, var_t7_5_dn4, var_t7_5_dn5, var_t7_5_dn6, var_t7_5_dn7, var_t7_5_dn8, var_t7_5_dn9, var_t7_5_dn10, var_t7_5_dn11, var_t7_5_dn13, var_t7_5_dn14,)
    }
};
        var_t7_5 = assign33940_e56634;
        var_t7_5_dn0 = assign33940_e56634_d_n0;
        var_t7_5_dn2 = assign33940_e56634_d_n2;
        var_t7_5_dn3 = assign33940_e56634_d_n3;
        var_t7_5_dn4 = assign33940_e56634_d_n4;
        var_t7_5_dn5 = assign33940_e56634_d_n5;
        var_t7_5_dn6 = assign33940_e56634_d_n6;
        var_t7_5_dn7 = assign33940_e56634_d_n7;
        var_t7_5_dn8 = assign33940_e56634_d_n8;
        var_t7_5_dn9 = assign33940_e56634_d_n9;
        var_t7_5_dn10 = assign33940_e56634_d_n10;
        var_t7_5_dn11 = assign33940_e56634_d_n11;
        var_t7_5_dn13 = assign33940_e56634_d_n13;
        var_t7_5_dn14 = assign33940_e56634_d_n14;

        *var_dr0_slot = var_dr0;
        *var_dr0_dn0_slot = var_dr0_dn0;
        *var_dr0_dn10_slot = var_dr0_dn10;
        *var_dr0_dn11_slot = var_dr0_dn11;
        *var_dr0_dn13_slot = var_dr0_dn13;
        *var_dr0_dn14_slot = var_dr0_dn14;
        *var_dr0_dn2_slot = var_dr0_dn2;
        *var_dr0_dn3_slot = var_dr0_dn3;
        *var_dr0_dn4_slot = var_dr0_dn4;
        *var_dr0_dn5_slot = var_dr0_dn5;
        *var_dr0_dn6_slot = var_dr0_dn6;
        *var_dr0_dn7_slot = var_dr0_dn7;
        *var_dr0_dn8_slot = var_dr0_dn8;
        *var_dr0_dn9_slot = var_dr0_dn9;
        *var_noigd0_slot = var_noigd0;
        *var_noigd0_dn0_slot = var_noigd0_dn0;
        *var_noigd0_dn10_slot = var_noigd0_dn10;
        *var_noigd0_dn11_slot = var_noigd0_dn11;
        *var_noigd0_dn13_slot = var_noigd0_dn13;
        *var_noigd0_dn14_slot = var_noigd0_dn14;
        *var_noigd0_dn2_slot = var_noigd0_dn2;
        *var_noigd0_dn3_slot = var_noigd0_dn3;
        *var_noigd0_dn4_slot = var_noigd0_dn4;
        *var_noigd0_dn5_slot = var_noigd0_dn5;
        *var_noigd0_dn6_slot = var_noigd0_dn6;
        *var_noigd0_dn7_slot = var_noigd0_dn7;
        *var_noigd0_dn8_slot = var_noigd0_dn8;
        *var_noigd0_dn9_slot = var_noigd0_dn9;
        *var_rdsi0_slot = var_rdsi0;
        *var_rdsi0_dn0_slot = var_rdsi0_dn0;
        *var_rdsi0_dn10_slot = var_rdsi0_dn10;
        *var_rdsi0_dn11_slot = var_rdsi0_dn11;
        *var_rdsi0_dn13_slot = var_rdsi0_dn13;
        *var_rdsi0_dn14_slot = var_rdsi0_dn14;
        *var_rdsi0_dn2_slot = var_rdsi0_dn2;
        *var_rdsi0_dn3_slot = var_rdsi0_dn3;
        *var_rdsi0_dn4_slot = var_rdsi0_dn4;
        *var_rdsi0_dn5_slot = var_rdsi0_dn5;
        *var_rdsi0_dn6_slot = var_rdsi0_dn6;
        *var_rdsi0_dn7_slot = var_rdsi0_dn7;
        *var_rdsi0_dn8_slot = var_rdsi0_dn8;
        *var_rdsi0_dn9_slot = var_rdsi0_dn9;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn14_slot = var_t4_dn14;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t5_slot = var_t5;
        *var_t5_2_slot = var_t5_2;
        *var_t5_2_dn0_slot = var_t5_2_dn0;
        *var_t5_2_dn10_slot = var_t5_2_dn10;
        *var_t5_2_dn11_slot = var_t5_2_dn11;
        *var_t5_2_dn13_slot = var_t5_2_dn13;
        *var_t5_2_dn14_slot = var_t5_2_dn14;
        *var_t5_2_dn2_slot = var_t5_2_dn2;
        *var_t5_2_dn3_slot = var_t5_2_dn3;
        *var_t5_2_dn4_slot = var_t5_2_dn4;
        *var_t5_2_dn5_slot = var_t5_2_dn5;
        *var_t5_2_dn6_slot = var_t5_2_dn6;
        *var_t5_2_dn7_slot = var_t5_2_dn7;
        *var_t5_2_dn8_slot = var_t5_2_dn8;
        *var_t5_2_dn9_slot = var_t5_2_dn9;
        *var_t5_3_slot = var_t5_3;
        *var_t5_3_dn0_slot = var_t5_3_dn0;
        *var_t5_3_dn10_slot = var_t5_3_dn10;
        *var_t5_3_dn11_slot = var_t5_3_dn11;
        *var_t5_3_dn13_slot = var_t5_3_dn13;
        *var_t5_3_dn14_slot = var_t5_3_dn14;
        *var_t5_3_dn2_slot = var_t5_3_dn2;
        *var_t5_3_dn3_slot = var_t5_3_dn3;
        *var_t5_3_dn4_slot = var_t5_3_dn4;
        *var_t5_3_dn5_slot = var_t5_3_dn5;
        *var_t5_3_dn6_slot = var_t5_3_dn6;
        *var_t5_3_dn7_slot = var_t5_3_dn7;
        *var_t5_3_dn8_slot = var_t5_3_dn8;
        *var_t5_3_dn9_slot = var_t5_3_dn9;
        *var_t5_4_slot = var_t5_4;
        *var_t5_4_dn0_slot = var_t5_4_dn0;
        *var_t5_4_dn10_slot = var_t5_4_dn10;
        *var_t5_4_dn11_slot = var_t5_4_dn11;
        *var_t5_4_dn13_slot = var_t5_4_dn13;
        *var_t5_4_dn14_slot = var_t5_4_dn14;
        *var_t5_4_dn2_slot = var_t5_4_dn2;
        *var_t5_4_dn3_slot = var_t5_4_dn3;
        *var_t5_4_dn4_slot = var_t5_4_dn4;
        *var_t5_4_dn5_slot = var_t5_4_dn5;
        *var_t5_4_dn6_slot = var_t5_4_dn6;
        *var_t5_4_dn7_slot = var_t5_4_dn7;
        *var_t5_4_dn8_slot = var_t5_4_dn8;
        *var_t5_4_dn9_slot = var_t5_4_dn9;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn13_slot = var_t5_dn13;
        *var_t5_dn14_slot = var_t5_dn14;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn3_slot = var_t5_dn3;
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
        *var_t6_dn13_slot = var_t6_dn13;
        *var_t6_dn14_slot = var_t6_dn14;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_t7_slot = var_t7;
        *var_t7_2_slot = var_t7_2;
        *var_t7_2_dn0_slot = var_t7_2_dn0;
        *var_t7_2_dn10_slot = var_t7_2_dn10;
        *var_t7_2_dn11_slot = var_t7_2_dn11;
        *var_t7_2_dn13_slot = var_t7_2_dn13;
        *var_t7_2_dn14_slot = var_t7_2_dn14;
        *var_t7_2_dn2_slot = var_t7_2_dn2;
        *var_t7_2_dn3_slot = var_t7_2_dn3;
        *var_t7_2_dn4_slot = var_t7_2_dn4;
        *var_t7_2_dn5_slot = var_t7_2_dn5;
        *var_t7_2_dn6_slot = var_t7_2_dn6;
        *var_t7_2_dn7_slot = var_t7_2_dn7;
        *var_t7_2_dn8_slot = var_t7_2_dn8;
        *var_t7_2_dn9_slot = var_t7_2_dn9;
        *var_t7_3_slot = var_t7_3;
        *var_t7_3_dn0_slot = var_t7_3_dn0;
        *var_t7_3_dn10_slot = var_t7_3_dn10;
        *var_t7_3_dn11_slot = var_t7_3_dn11;
        *var_t7_3_dn13_slot = var_t7_3_dn13;
        *var_t7_3_dn14_slot = var_t7_3_dn14;
        *var_t7_3_dn2_slot = var_t7_3_dn2;
        *var_t7_3_dn3_slot = var_t7_3_dn3;
        *var_t7_3_dn4_slot = var_t7_3_dn4;
        *var_t7_3_dn5_slot = var_t7_3_dn5;
        *var_t7_3_dn6_slot = var_t7_3_dn6;
        *var_t7_3_dn7_slot = var_t7_3_dn7;
        *var_t7_3_dn8_slot = var_t7_3_dn8;
        *var_t7_3_dn9_slot = var_t7_3_dn9;
        *var_t7_4_slot = var_t7_4;
        *var_t7_4_dn0_slot = var_t7_4_dn0;
        *var_t7_4_dn10_slot = var_t7_4_dn10;
        *var_t7_4_dn11_slot = var_t7_4_dn11;
        *var_t7_4_dn13_slot = var_t7_4_dn13;
        *var_t7_4_dn14_slot = var_t7_4_dn14;
        *var_t7_4_dn2_slot = var_t7_4_dn2;
        *var_t7_4_dn3_slot = var_t7_4_dn3;
        *var_t7_4_dn4_slot = var_t7_4_dn4;
        *var_t7_4_dn5_slot = var_t7_4_dn5;
        *var_t7_4_dn6_slot = var_t7_4_dn6;
        *var_t7_4_dn7_slot = var_t7_4_dn7;
        *var_t7_4_dn8_slot = var_t7_4_dn8;
        *var_t7_4_dn9_slot = var_t7_4_dn9;
        *var_t7_5_slot = var_t7_5;
        *var_t7_5_dn0_slot = var_t7_5_dn0;
        *var_t7_5_dn10_slot = var_t7_5_dn10;
        *var_t7_5_dn11_slot = var_t7_5_dn11;
        *var_t7_5_dn13_slot = var_t7_5_dn13;
        *var_t7_5_dn14_slot = var_t7_5_dn14;
        *var_t7_5_dn2_slot = var_t7_5_dn2;
        *var_t7_5_dn3_slot = var_t7_5_dn3;
        *var_t7_5_dn4_slot = var_t7_5_dn4;
        *var_t7_5_dn5_slot = var_t7_5_dn5;
        *var_t7_5_dn6_slot = var_t7_5_dn6;
        *var_t7_5_dn7_slot = var_t7_5_dn7;
        *var_t7_5_dn8_slot = var_t7_5_dn8;
        *var_t7_5_dn9_slot = var_t7_5_dn9;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn13_slot = var_t7_dn13;
        *var_t7_dn14_slot = var_t7_dn14;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn3_slot = var_t7_dn3;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_dn9_slot = var_t7_dn9;
    }

    pub(super) fn stamp_transient_block_131(
        p: &Parameters,
        var_coxeff: f64,
        var_coxeff_dn0: f64,
        var_coxeff_dn10: f64,
        var_coxeff_dn11: f64,
        var_coxeff_dn13: f64,
        var_coxeff_dn14: f64,
        var_coxeff_dn2: f64,
        var_coxeff_dn3: f64,
        var_coxeff_dn4: f64,
        var_coxeff_dn5: f64,
        var_coxeff_dn6: f64,
        var_coxeff_dn7: f64,
        var_coxeff_dn8: f64,
        var_coxeff_dn9: f64,
        var_dvsat: f64,
        var_dvsat3: f64,
        var_dvsat3_dn0: f64,
        var_dvsat3_dn10: f64,
        var_dvsat3_dn11: f64,
        var_dvsat3_dn13: f64,
        var_dvsat3_dn14: f64,
        var_dvsat3_dn2: f64,
        var_dvsat3_dn3: f64,
        var_dvsat3_dn4: f64,
        var_dvsat3_dn5: f64,
        var_dvsat3_dn6: f64,
        var_dvsat3_dn7: f64,
        var_dvsat3_dn8: f64,
        var_dvsat3_dn9: f64,
        var_dvsat_dn0: f64,
        var_dvsat_dn10: f64,
        var_dvsat_dn11: f64,
        var_dvsat_dn13: f64,
        var_dvsat_dn14: f64,
        var_dvsat_dn2: f64,
        var_dvsat_dn3: f64,
        var_dvsat_dn4: f64,
        var_dvsat_dn5: f64,
        var_dvsat_dn6: f64,
        var_dvsat_dn7: f64,
        var_dvsat_dn8: f64,
        var_dvsat_dn9: f64,
        var_guard632: f64,
        var_guard633: f64,
        var_leffcv_1: f64,
        var_leffcv_1_dn0: f64,
        var_leffcv_1_dn10: f64,
        var_leffcv_1_dn11: f64,
        var_leffcv_1_dn13: f64,
        var_leffcv_1_dn14: f64,
        var_leffcv_1_dn2: f64,
        var_leffcv_1_dn3: f64,
        var_leffcv_1_dn4: f64,
        var_leffcv_1_dn5: f64,
        var_leffcv_1_dn6: f64,
        var_leffcv_1_dn7: f64,
        var_leffcv_1_dn8: f64,
        var_leffcv_1_dn9: f64,
        var_moc: f64,
        var_moc_dn0: f64,
        var_moc_dn10: f64,
        var_moc_dn11: f64,
        var_moc_dn13: f64,
        var_moc_dn14: f64,
        var_moc_dn2: f64,
        var_moc_dn3: f64,
        var_moc_dn4: f64,
        var_moc_dn5: f64,
        var_moc_dn6: f64,
        var_moc_dn7: f64,
        var_moc_dn8: f64,
        var_moc_dn9: f64,
        var_nfintotal: f64,
        var_noigd0: f64,
        var_noigd0_dn0: f64,
        var_noigd0_dn10: f64,
        var_noigd0_dn11: f64,
        var_noigd0_dn13: f64,
        var_noigd0_dn14: f64,
        var_noigd0_dn2: f64,
        var_noigd0_dn3: f64,
        var_noigd0_dn4: f64,
        var_noigd0_dn5: f64,
        var_noigd0_dn6: f64,
        var_noigd0_dn7: f64,
        var_noigd0_dn8: f64,
        var_noigd0_dn9: f64,
        var_noilowid: f64,
        var_noilowid_dn0: f64,
        var_noilowid_dn10: f64,
        var_noilowid_dn11: f64,
        var_noilowid_dn13: f64,
        var_noilowid_dn14: f64,
        var_noilowid_dn2: f64,
        var_noilowid_dn3: f64,
        var_noilowid_dn4: f64,
        var_noilowid_dn5: f64,
        var_noilowid_dn6: f64,
        var_noilowid_dn7: f64,
        var_noilowid_dn8: f64,
        var_noilowid_dn9: f64,
        var_qia: f64,
        var_qia_dn0: f64,
        var_qia_dn10: f64,
        var_qia_dn11: f64,
        var_qia_dn13: f64,
        var_qia_dn14: f64,
        var_qia_dn2: f64,
        var_qia_dn3: f64,
        var_qia_dn4: f64,
        var_qia_dn5: f64,
        var_qia_dn6: f64,
        var_qia_dn7: f64,
        var_qia_dn8: f64,
        var_qia_dn9: f64,
        var_sigvds: f64,
        var_t1: f64,
        var_t1_dn0: f64,
        var_t1_dn10: f64,
        var_t1_dn11: f64,
        var_t1_dn13: f64,
        var_t1_dn14: f64,
        var_t1_dn2: f64,
        var_t1_dn3: f64,
        var_t1_dn4: f64,
        var_t1_dn5: f64,
        var_t1_dn6: f64,
        var_t1_dn7: f64,
        var_t1_dn8: f64,
        var_t1_dn9: f64,
        var_t2: f64,
        var_t2_dn0: f64,
        var_t2_dn10: f64,
        var_t2_dn11: f64,
        var_t2_dn13: f64,
        var_t2_dn14: f64,
        var_t2_dn2: f64,
        var_t2_dn3: f64,
        var_t2_dn4: f64,
        var_t2_dn5: f64,
        var_t2_dn6: f64,
        var_t2_dn7: f64,
        var_t2_dn8: f64,
        var_t2_dn9: f64,
        var_t3: f64,
        var_t3_dn0: f64,
        var_t3_dn10: f64,
        var_t3_dn11: f64,
        var_t3_dn13: f64,
        var_t3_dn14: f64,
        var_t3_dn2: f64,
        var_t3_dn3: f64,
        var_t3_dn4: f64,
        var_t3_dn5: f64,
        var_t3_dn6: f64,
        var_t3_dn7: f64,
        var_t3_dn8: f64,
        var_t3_dn9: f64,
        var_t4: f64,
        var_t4_dn0: f64,
        var_t4_dn10: f64,
        var_t4_dn11: f64,
        var_t4_dn13: f64,
        var_t4_dn14: f64,
        var_t4_dn2: f64,
        var_t4_dn3: f64,
        var_t4_dn4: f64,
        var_t4_dn5: f64,
        var_t4_dn6: f64,
        var_t4_dn7: f64,
        var_t4_dn8: f64,
        var_t4_dn9: f64,
        var_t5: f64,
        var_t5_2: f64,
        var_t5_2_dn0: f64,
        var_t5_2_dn10: f64,
        var_t5_2_dn11: f64,
        var_t5_2_dn13: f64,
        var_t5_2_dn14: f64,
        var_t5_2_dn2: f64,
        var_t5_2_dn3: f64,
        var_t5_2_dn4: f64,
        var_t5_2_dn5: f64,
        var_t5_2_dn6: f64,
        var_t5_2_dn7: f64,
        var_t5_2_dn8: f64,
        var_t5_2_dn9: f64,
        var_t5_3: f64,
        var_t5_3_dn0: f64,
        var_t5_3_dn10: f64,
        var_t5_3_dn11: f64,
        var_t5_3_dn13: f64,
        var_t5_3_dn14: f64,
        var_t5_3_dn2: f64,
        var_t5_3_dn3: f64,
        var_t5_3_dn4: f64,
        var_t5_3_dn5: f64,
        var_t5_3_dn6: f64,
        var_t5_3_dn7: f64,
        var_t5_3_dn8: f64,
        var_t5_3_dn9: f64,
        var_t5_4: f64,
        var_t5_4_dn0: f64,
        var_t5_4_dn10: f64,
        var_t5_4_dn11: f64,
        var_t5_4_dn13: f64,
        var_t5_4_dn14: f64,
        var_t5_4_dn2: f64,
        var_t5_4_dn3: f64,
        var_t5_4_dn4: f64,
        var_t5_4_dn5: f64,
        var_t5_4_dn6: f64,
        var_t5_4_dn7: f64,
        var_t5_4_dn8: f64,
        var_t5_4_dn9: f64,
        var_t5_dn0: f64,
        var_t5_dn10: f64,
        var_t5_dn11: f64,
        var_t5_dn13: f64,
        var_t5_dn14: f64,
        var_t5_dn2: f64,
        var_t5_dn3: f64,
        var_t5_dn4: f64,
        var_t5_dn5: f64,
        var_t5_dn6: f64,
        var_t5_dn7: f64,
        var_t5_dn8: f64,
        var_t5_dn9: f64,
        var_t6: f64,
        var_t6_dn0: f64,
        var_t6_dn10: f64,
        var_t6_dn11: f64,
        var_t6_dn13: f64,
        var_t6_dn14: f64,
        var_t6_dn2: f64,
        var_t6_dn3: f64,
        var_t6_dn4: f64,
        var_t6_dn5: f64,
        var_t6_dn6: f64,
        var_t6_dn7: f64,
        var_t6_dn8: f64,
        var_t6_dn9: f64,
        var_t7: f64,
        var_t7_2: f64,
        var_t7_2_dn0: f64,
        var_t7_2_dn10: f64,
        var_t7_2_dn11: f64,
        var_t7_2_dn13: f64,
        var_t7_2_dn14: f64,
        var_t7_2_dn2: f64,
        var_t7_2_dn3: f64,
        var_t7_2_dn4: f64,
        var_t7_2_dn5: f64,
        var_t7_2_dn6: f64,
        var_t7_2_dn7: f64,
        var_t7_2_dn8: f64,
        var_t7_2_dn9: f64,
        var_t7_3: f64,
        var_t7_3_dn0: f64,
        var_t7_3_dn10: f64,
        var_t7_3_dn11: f64,
        var_t7_3_dn13: f64,
        var_t7_3_dn14: f64,
        var_t7_3_dn2: f64,
        var_t7_3_dn3: f64,
        var_t7_3_dn4: f64,
        var_t7_3_dn5: f64,
        var_t7_3_dn6: f64,
        var_t7_3_dn7: f64,
        var_t7_3_dn8: f64,
        var_t7_3_dn9: f64,
        var_t7_4: f64,
        var_t7_4_dn0: f64,
        var_t7_4_dn10: f64,
        var_t7_4_dn11: f64,
        var_t7_4_dn13: f64,
        var_t7_4_dn14: f64,
        var_t7_4_dn2: f64,
        var_t7_4_dn3: f64,
        var_t7_4_dn4: f64,
        var_t7_4_dn5: f64,
        var_t7_4_dn6: f64,
        var_t7_4_dn7: f64,
        var_t7_4_dn8: f64,
        var_t7_4_dn9: f64,
        var_t7_5: f64,
        var_t7_5_dn0: f64,
        var_t7_5_dn10: f64,
        var_t7_5_dn11: f64,
        var_t7_5_dn13: f64,
        var_t7_5_dn14: f64,
        var_t7_5_dn2: f64,
        var_t7_5_dn3: f64,
        var_t7_5_dn4: f64,
        var_t7_5_dn5: f64,
        var_t7_5_dn6: f64,
        var_t7_5_dn7: f64,
        var_t7_5_dn8: f64,
        var_t7_5_dn9: f64,
        var_t7_dn0: f64,
        var_t7_dn10: f64,
        var_t7_dn11: f64,
        var_t7_dn13: f64,
        var_t7_dn14: f64,
        var_t7_dn2: f64,
        var_t7_dn3: f64,
        var_t7_dn4: f64,
        var_t7_dn5: f64,
        var_t7_dn6: f64,
        var_t7_dn7: f64,
        var_t7_dn8: f64,
        var_t7_dn9: f64,
        var_vdsat: f64,
        var_vdsat_dn0: f64,
        var_vdsat_dn10: f64,
        var_vdsat_dn11: f64,
        var_vdsat_dn13: f64,
        var_vdsat_dn14: f64,
        var_vdsat_dn2: f64,
        var_vdsat_dn3: f64,
        var_vdsat_dn4: f64,
        var_vdsat_dn5: f64,
        var_vdsat_dn6: f64,
        var_vdsat_dn7: f64,
        var_vdsat_dn8: f64,
        var_vdsat_dn9: f64,
        var_vdseff_1: f64,
        var_vdseff_1_dn0: f64,
        var_vdseff_1_dn10: f64,
        var_vdseff_1_dn11: f64,
        var_vdseff_1_dn13: f64,
        var_vdseff_1_dn14: f64,
        var_vdseff_1_dn2: f64,
        var_vdseff_1_dn3: f64,
        var_vdseff_1_dn4: f64,
        var_vdseff_1_dn5: f64,
        var_vdseff_1_dn6: f64,
        var_vdseff_1_dn7: f64,
        var_vdseff_1_dn8: f64,
        var_vdseff_1_dn9: f64,
        var_weffcv0: f64,
        var_ctnoi_slot: &mut f64,
        var_ctnoi_dn0_slot: &mut f64,
        var_ctnoi_dn10_slot: &mut f64,
        var_ctnoi_dn11_slot: &mut f64,
        var_ctnoi_dn13_slot: &mut f64,
        var_ctnoi_dn14_slot: &mut f64,
        var_ctnoi_dn2_slot: &mut f64,
        var_ctnoi_dn3_slot: &mut f64,
        var_ctnoi_dn4_slot: &mut f64,
        var_ctnoi_dn5_slot: &mut f64,
        var_ctnoi_dn6_slot: &mut f64,
        var_ctnoi_dn7_slot: &mut f64,
        var_ctnoi_dn8_slot: &mut f64,
        var_ctnoi_dn9_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_delta1_slot: &mut f64,
        var_delta1_dn0_slot: &mut f64,
        var_delta1_dn10_slot: &mut f64,
        var_delta1_dn11_slot: &mut f64,
        var_delta1_dn13_slot: &mut f64,
        var_delta1_dn14_slot: &mut f64,
        var_delta1_dn2_slot: &mut f64,
        var_delta1_dn3_slot: &mut f64,
        var_delta1_dn4_slot: &mut f64,
        var_delta1_dn5_slot: &mut f64,
        var_delta1_dn6_slot: &mut f64,
        var_delta1_dn7_slot: &mut f64,
        var_delta1_dn8_slot: &mut f64,
        var_delta1_dn9_slot: &mut f64,
        var_delta2_slot: &mut f64,
        var_delta2_dn0_slot: &mut f64,
        var_delta2_dn10_slot: &mut f64,
        var_delta2_dn11_slot: &mut f64,
        var_delta2_dn13_slot: &mut f64,
        var_delta2_dn14_slot: &mut f64,
        var_delta2_dn2_slot: &mut f64,
        var_delta2_dn3_slot: &mut f64,
        var_delta2_dn4_slot: &mut f64,
        var_delta2_dn5_slot: &mut f64,
        var_delta2_dn6_slot: &mut f64,
        var_delta2_dn7_slot: &mut f64,
        var_delta2_dn8_slot: &mut f64,
        var_delta2_dn9_slot: &mut f64,
        var_delta3_slot: &mut f64,
        var_delta3_dn0_slot: &mut f64,
        var_delta3_dn10_slot: &mut f64,
        var_delta3_dn11_slot: &mut f64,
        var_delta3_dn13_slot: &mut f64,
        var_delta3_dn14_slot: &mut f64,
        var_delta3_dn2_slot: &mut f64,
        var_delta3_dn3_slot: &mut f64,
        var_delta3_dn4_slot: &mut f64,
        var_delta3_dn5_slot: &mut f64,
        var_delta3_dn6_slot: &mut f64,
        var_delta3_dn7_slot: &mut f64,
        var_delta3_dn8_slot: &mut f64,
        var_delta3_dn9_slot: &mut f64,
        var_delta_dn0_slot: &mut f64,
        var_delta_dn10_slot: &mut f64,
        var_delta_dn11_slot: &mut f64,
        var_delta_dn13_slot: &mut f64,
        var_delta_dn14_slot: &mut f64,
        var_delta_dn2_slot: &mut f64,
        var_delta_dn3_slot: &mut f64,
        var_delta_dn4_slot: &mut f64,
        var_delta_dn5_slot: &mut f64,
        var_delta_dn6_slot: &mut f64,
        var_delta_dn7_slot: &mut f64,
        var_delta_dn8_slot: &mut f64,
        var_delta_dn9_slot: &mut f64,
        var_epsilon_slot: &mut f64,
        var_epsilon1_slot: &mut f64,
        var_epsilon1_dn0_slot: &mut f64,
        var_epsilon1_dn10_slot: &mut f64,
        var_epsilon1_dn11_slot: &mut f64,
        var_epsilon1_dn13_slot: &mut f64,
        var_epsilon1_dn14_slot: &mut f64,
        var_epsilon1_dn2_slot: &mut f64,
        var_epsilon1_dn3_slot: &mut f64,
        var_epsilon1_dn4_slot: &mut f64,
        var_epsilon1_dn5_slot: &mut f64,
        var_epsilon1_dn6_slot: &mut f64,
        var_epsilon1_dn7_slot: &mut f64,
        var_epsilon1_dn8_slot: &mut f64,
        var_epsilon1_dn9_slot: &mut f64,
        var_epsilon2_slot: &mut f64,
        var_epsilon2_dn0_slot: &mut f64,
        var_epsilon2_dn10_slot: &mut f64,
        var_epsilon2_dn11_slot: &mut f64,
        var_epsilon2_dn13_slot: &mut f64,
        var_epsilon2_dn14_slot: &mut f64,
        var_epsilon2_dn2_slot: &mut f64,
        var_epsilon2_dn3_slot: &mut f64,
        var_epsilon2_dn4_slot: &mut f64,
        var_epsilon2_dn5_slot: &mut f64,
        var_epsilon2_dn6_slot: &mut f64,
        var_epsilon2_dn7_slot: &mut f64,
        var_epsilon2_dn8_slot: &mut f64,
        var_epsilon2_dn9_slot: &mut f64,
        var_epsilon_dn0_slot: &mut f64,
        var_epsilon_dn10_slot: &mut f64,
        var_epsilon_dn11_slot: &mut f64,
        var_epsilon_dn13_slot: &mut f64,
        var_epsilon_dn14_slot: &mut f64,
        var_epsilon_dn2_slot: &mut f64,
        var_epsilon_dn3_slot: &mut f64,
        var_epsilon_dn4_slot: &mut f64,
        var_epsilon_dn5_slot: &mut f64,
        var_epsilon_dn6_slot: &mut f64,
        var_epsilon_dn7_slot: &mut f64,
        var_epsilon_dn8_slot: &mut f64,
        var_epsilon_dn9_slot: &mut f64,
        var_gamma_slot: &mut f64,
        var_gamma1_slot: &mut f64,
        var_gamma1_dn0_slot: &mut f64,
        var_gamma1_dn10_slot: &mut f64,
        var_gamma1_dn11_slot: &mut f64,
        var_gamma1_dn13_slot: &mut f64,
        var_gamma1_dn14_slot: &mut f64,
        var_gamma1_dn2_slot: &mut f64,
        var_gamma1_dn3_slot: &mut f64,
        var_gamma1_dn4_slot: &mut f64,
        var_gamma1_dn5_slot: &mut f64,
        var_gamma1_dn6_slot: &mut f64,
        var_gamma1_dn7_slot: &mut f64,
        var_gamma1_dn8_slot: &mut f64,
        var_gamma1_dn9_slot: &mut f64,
        var_gamma2_slot: &mut f64,
        var_gamma2_dn0_slot: &mut f64,
        var_gamma2_dn10_slot: &mut f64,
        var_gamma2_dn11_slot: &mut f64,
        var_gamma2_dn13_slot: &mut f64,
        var_gamma2_dn14_slot: &mut f64,
        var_gamma2_dn2_slot: &mut f64,
        var_gamma2_dn3_slot: &mut f64,
        var_gamma2_dn4_slot: &mut f64,
        var_gamma2_dn5_slot: &mut f64,
        var_gamma2_dn6_slot: &mut f64,
        var_gamma2_dn7_slot: &mut f64,
        var_gamma2_dn8_slot: &mut f64,
        var_gamma2_dn9_slot: &mut f64,
        var_gamma_dn0_slot: &mut f64,
        var_gamma_dn10_slot: &mut f64,
        var_gamma_dn11_slot: &mut f64,
        var_gamma_dn13_slot: &mut f64,
        var_gamma_dn14_slot: &mut f64,
        var_gamma_dn2_slot: &mut f64,
        var_gamma_dn3_slot: &mut f64,
        var_gamma_dn4_slot: &mut f64,
        var_gamma_dn5_slot: &mut f64,
        var_gamma_dn6_slot: &mut f64,
        var_gamma_dn7_slot: &mut f64,
        var_gamma_dn8_slot: &mut f64,
        var_gamma_dn9_slot: &mut f64,
        var_guard639_slot: &mut f64,
        var_guard640_slot: &mut f64,
        var_guard641_slot: &mut f64,
        var_guard642_slot: &mut f64,
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
        var_guard654_slot: &mut f64,
        var_guard655_slot: &mut f64,
        var_guard656_slot: &mut f64,
        var_guard657_slot: &mut f64,
        var_guard658_slot: &mut f64,
        var_sigrat_slot: &mut f64,
        var_sigrat_dn0_slot: &mut f64,
        var_sigrat_dn10_slot: &mut f64,
        var_sigrat_dn11_slot: &mut f64,
        var_sigrat_dn13_slot: &mut f64,
        var_sigrat_dn14_slot: &mut f64,
        var_sigrat_dn2_slot: &mut f64,
        var_sigrat_dn3_slot: &mut f64,
        var_sigrat_dn4_slot: &mut f64,
        var_sigrat_dn5_slot: &mut f64,
        var_sigrat_dn6_slot: &mut f64,
        var_sigrat_dn7_slot: &mut f64,
        var_sigrat_dn8_slot: &mut f64,
        var_sigrat_dn9_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn0_slot: &mut f64,
        var_t8_dn10_slot: &mut f64,
        var_t8_dn11_slot: &mut f64,
        var_t8_dn13_slot: &mut f64,
        var_t8_dn14_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn3_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_dn9_slot: &mut f64,
    ) {
        let mut var_ctnoi: f64 = *var_ctnoi_slot;
        let mut var_ctnoi_dn0: f64 = *var_ctnoi_dn0_slot;
        let mut var_ctnoi_dn10: f64 = *var_ctnoi_dn10_slot;
        let mut var_ctnoi_dn11: f64 = *var_ctnoi_dn11_slot;
        let mut var_ctnoi_dn13: f64 = *var_ctnoi_dn13_slot;
        let mut var_ctnoi_dn14: f64 = *var_ctnoi_dn14_slot;
        let mut var_ctnoi_dn2: f64 = *var_ctnoi_dn2_slot;
        let mut var_ctnoi_dn3: f64 = *var_ctnoi_dn3_slot;
        let mut var_ctnoi_dn4: f64 = *var_ctnoi_dn4_slot;
        let mut var_ctnoi_dn5: f64 = *var_ctnoi_dn5_slot;
        let mut var_ctnoi_dn6: f64 = *var_ctnoi_dn6_slot;
        let mut var_ctnoi_dn7: f64 = *var_ctnoi_dn7_slot;
        let mut var_ctnoi_dn8: f64 = *var_ctnoi_dn8_slot;
        let mut var_ctnoi_dn9: f64 = *var_ctnoi_dn9_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta1: f64 = *var_delta1_slot;
        let mut var_delta1_dn0: f64 = *var_delta1_dn0_slot;
        let mut var_delta1_dn10: f64 = *var_delta1_dn10_slot;
        let mut var_delta1_dn11: f64 = *var_delta1_dn11_slot;
        let mut var_delta1_dn13: f64 = *var_delta1_dn13_slot;
        let mut var_delta1_dn14: f64 = *var_delta1_dn14_slot;
        let mut var_delta1_dn2: f64 = *var_delta1_dn2_slot;
        let mut var_delta1_dn3: f64 = *var_delta1_dn3_slot;
        let mut var_delta1_dn4: f64 = *var_delta1_dn4_slot;
        let mut var_delta1_dn5: f64 = *var_delta1_dn5_slot;
        let mut var_delta1_dn6: f64 = *var_delta1_dn6_slot;
        let mut var_delta1_dn7: f64 = *var_delta1_dn7_slot;
        let mut var_delta1_dn8: f64 = *var_delta1_dn8_slot;
        let mut var_delta1_dn9: f64 = *var_delta1_dn9_slot;
        let mut var_delta2: f64 = *var_delta2_slot;
        let mut var_delta2_dn0: f64 = *var_delta2_dn0_slot;
        let mut var_delta2_dn10: f64 = *var_delta2_dn10_slot;
        let mut var_delta2_dn11: f64 = *var_delta2_dn11_slot;
        let mut var_delta2_dn13: f64 = *var_delta2_dn13_slot;
        let mut var_delta2_dn14: f64 = *var_delta2_dn14_slot;
        let mut var_delta2_dn2: f64 = *var_delta2_dn2_slot;
        let mut var_delta2_dn3: f64 = *var_delta2_dn3_slot;
        let mut var_delta2_dn4: f64 = *var_delta2_dn4_slot;
        let mut var_delta2_dn5: f64 = *var_delta2_dn5_slot;
        let mut var_delta2_dn6: f64 = *var_delta2_dn6_slot;
        let mut var_delta2_dn7: f64 = *var_delta2_dn7_slot;
        let mut var_delta2_dn8: f64 = *var_delta2_dn8_slot;
        let mut var_delta2_dn9: f64 = *var_delta2_dn9_slot;
        let mut var_delta3: f64 = *var_delta3_slot;
        let mut var_delta3_dn0: f64 = *var_delta3_dn0_slot;
        let mut var_delta3_dn10: f64 = *var_delta3_dn10_slot;
        let mut var_delta3_dn11: f64 = *var_delta3_dn11_slot;
        let mut var_delta3_dn13: f64 = *var_delta3_dn13_slot;
        let mut var_delta3_dn14: f64 = *var_delta3_dn14_slot;
        let mut var_delta3_dn2: f64 = *var_delta3_dn2_slot;
        let mut var_delta3_dn3: f64 = *var_delta3_dn3_slot;
        let mut var_delta3_dn4: f64 = *var_delta3_dn4_slot;
        let mut var_delta3_dn5: f64 = *var_delta3_dn5_slot;
        let mut var_delta3_dn6: f64 = *var_delta3_dn6_slot;
        let mut var_delta3_dn7: f64 = *var_delta3_dn7_slot;
        let mut var_delta3_dn8: f64 = *var_delta3_dn8_slot;
        let mut var_delta3_dn9: f64 = *var_delta3_dn9_slot;
        let mut var_delta_dn0: f64 = *var_delta_dn0_slot;
        let mut var_delta_dn10: f64 = *var_delta_dn10_slot;
        let mut var_delta_dn11: f64 = *var_delta_dn11_slot;
        let mut var_delta_dn13: f64 = *var_delta_dn13_slot;
        let mut var_delta_dn14: f64 = *var_delta_dn14_slot;
        let mut var_delta_dn2: f64 = *var_delta_dn2_slot;
        let mut var_delta_dn3: f64 = *var_delta_dn3_slot;
        let mut var_delta_dn4: f64 = *var_delta_dn4_slot;
        let mut var_delta_dn5: f64 = *var_delta_dn5_slot;
        let mut var_delta_dn6: f64 = *var_delta_dn6_slot;
        let mut var_delta_dn7: f64 = *var_delta_dn7_slot;
        let mut var_delta_dn8: f64 = *var_delta_dn8_slot;
        let mut var_delta_dn9: f64 = *var_delta_dn9_slot;
        let mut var_epsilon: f64 = *var_epsilon_slot;
        let mut var_epsilon1: f64 = *var_epsilon1_slot;
        let mut var_epsilon1_dn0: f64 = *var_epsilon1_dn0_slot;
        let mut var_epsilon1_dn10: f64 = *var_epsilon1_dn10_slot;
        let mut var_epsilon1_dn11: f64 = *var_epsilon1_dn11_slot;
        let mut var_epsilon1_dn13: f64 = *var_epsilon1_dn13_slot;
        let mut var_epsilon1_dn14: f64 = *var_epsilon1_dn14_slot;
        let mut var_epsilon1_dn2: f64 = *var_epsilon1_dn2_slot;
        let mut var_epsilon1_dn3: f64 = *var_epsilon1_dn3_slot;
        let mut var_epsilon1_dn4: f64 = *var_epsilon1_dn4_slot;
        let mut var_epsilon1_dn5: f64 = *var_epsilon1_dn5_slot;
        let mut var_epsilon1_dn6: f64 = *var_epsilon1_dn6_slot;
        let mut var_epsilon1_dn7: f64 = *var_epsilon1_dn7_slot;
        let mut var_epsilon1_dn8: f64 = *var_epsilon1_dn8_slot;
        let mut var_epsilon1_dn9: f64 = *var_epsilon1_dn9_slot;
        let mut var_epsilon2: f64 = *var_epsilon2_slot;
        let mut var_epsilon2_dn0: f64 = *var_epsilon2_dn0_slot;
        let mut var_epsilon2_dn10: f64 = *var_epsilon2_dn10_slot;
        let mut var_epsilon2_dn11: f64 = *var_epsilon2_dn11_slot;
        let mut var_epsilon2_dn13: f64 = *var_epsilon2_dn13_slot;
        let mut var_epsilon2_dn14: f64 = *var_epsilon2_dn14_slot;
        let mut var_epsilon2_dn2: f64 = *var_epsilon2_dn2_slot;
        let mut var_epsilon2_dn3: f64 = *var_epsilon2_dn3_slot;
        let mut var_epsilon2_dn4: f64 = *var_epsilon2_dn4_slot;
        let mut var_epsilon2_dn5: f64 = *var_epsilon2_dn5_slot;
        let mut var_epsilon2_dn6: f64 = *var_epsilon2_dn6_slot;
        let mut var_epsilon2_dn7: f64 = *var_epsilon2_dn7_slot;
        let mut var_epsilon2_dn8: f64 = *var_epsilon2_dn8_slot;
        let mut var_epsilon2_dn9: f64 = *var_epsilon2_dn9_slot;
        let mut var_epsilon_dn0: f64 = *var_epsilon_dn0_slot;
        let mut var_epsilon_dn10: f64 = *var_epsilon_dn10_slot;
        let mut var_epsilon_dn11: f64 = *var_epsilon_dn11_slot;
        let mut var_epsilon_dn13: f64 = *var_epsilon_dn13_slot;
        let mut var_epsilon_dn14: f64 = *var_epsilon_dn14_slot;
        let mut var_epsilon_dn2: f64 = *var_epsilon_dn2_slot;
        let mut var_epsilon_dn3: f64 = *var_epsilon_dn3_slot;
        let mut var_epsilon_dn4: f64 = *var_epsilon_dn4_slot;
        let mut var_epsilon_dn5: f64 = *var_epsilon_dn5_slot;
        let mut var_epsilon_dn6: f64 = *var_epsilon_dn6_slot;
        let mut var_epsilon_dn7: f64 = *var_epsilon_dn7_slot;
        let mut var_epsilon_dn8: f64 = *var_epsilon_dn8_slot;
        let mut var_epsilon_dn9: f64 = *var_epsilon_dn9_slot;
        let mut var_gamma: f64 = *var_gamma_slot;
        let mut var_gamma1: f64 = *var_gamma1_slot;
        let mut var_gamma1_dn0: f64 = *var_gamma1_dn0_slot;
        let mut var_gamma1_dn10: f64 = *var_gamma1_dn10_slot;
        let mut var_gamma1_dn11: f64 = *var_gamma1_dn11_slot;
        let mut var_gamma1_dn13: f64 = *var_gamma1_dn13_slot;
        let mut var_gamma1_dn14: f64 = *var_gamma1_dn14_slot;
        let mut var_gamma1_dn2: f64 = *var_gamma1_dn2_slot;
        let mut var_gamma1_dn3: f64 = *var_gamma1_dn3_slot;
        let mut var_gamma1_dn4: f64 = *var_gamma1_dn4_slot;
        let mut var_gamma1_dn5: f64 = *var_gamma1_dn5_slot;
        let mut var_gamma1_dn6: f64 = *var_gamma1_dn6_slot;
        let mut var_gamma1_dn7: f64 = *var_gamma1_dn7_slot;
        let mut var_gamma1_dn8: f64 = *var_gamma1_dn8_slot;
        let mut var_gamma1_dn9: f64 = *var_gamma1_dn9_slot;
        let mut var_gamma2: f64 = *var_gamma2_slot;
        let mut var_gamma2_dn0: f64 = *var_gamma2_dn0_slot;
        let mut var_gamma2_dn10: f64 = *var_gamma2_dn10_slot;
        let mut var_gamma2_dn11: f64 = *var_gamma2_dn11_slot;
        let mut var_gamma2_dn13: f64 = *var_gamma2_dn13_slot;
        let mut var_gamma2_dn14: f64 = *var_gamma2_dn14_slot;
        let mut var_gamma2_dn2: f64 = *var_gamma2_dn2_slot;
        let mut var_gamma2_dn3: f64 = *var_gamma2_dn3_slot;
        let mut var_gamma2_dn4: f64 = *var_gamma2_dn4_slot;
        let mut var_gamma2_dn5: f64 = *var_gamma2_dn5_slot;
        let mut var_gamma2_dn6: f64 = *var_gamma2_dn6_slot;
        let mut var_gamma2_dn7: f64 = *var_gamma2_dn7_slot;
        let mut var_gamma2_dn8: f64 = *var_gamma2_dn8_slot;
        let mut var_gamma2_dn9: f64 = *var_gamma2_dn9_slot;
        let mut var_gamma_dn0: f64 = *var_gamma_dn0_slot;
        let mut var_gamma_dn10: f64 = *var_gamma_dn10_slot;
        let mut var_gamma_dn11: f64 = *var_gamma_dn11_slot;
        let mut var_gamma_dn13: f64 = *var_gamma_dn13_slot;
        let mut var_gamma_dn14: f64 = *var_gamma_dn14_slot;
        let mut var_gamma_dn2: f64 = *var_gamma_dn2_slot;
        let mut var_gamma_dn3: f64 = *var_gamma_dn3_slot;
        let mut var_gamma_dn4: f64 = *var_gamma_dn4_slot;
        let mut var_gamma_dn5: f64 = *var_gamma_dn5_slot;
        let mut var_gamma_dn6: f64 = *var_gamma_dn6_slot;
        let mut var_gamma_dn7: f64 = *var_gamma_dn7_slot;
        let mut var_gamma_dn8: f64 = *var_gamma_dn8_slot;
        let mut var_gamma_dn9: f64 = *var_gamma_dn9_slot;
        let mut var_guard639: f64 = *var_guard639_slot;
        let mut var_guard640: f64 = *var_guard640_slot;
        let mut var_guard641: f64 = *var_guard641_slot;
        let mut var_guard642: f64 = *var_guard642_slot;
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
        let mut var_guard654: f64 = *var_guard654_slot;
        let mut var_guard655: f64 = *var_guard655_slot;
        let mut var_guard656: f64 = *var_guard656_slot;
        let mut var_guard657: f64 = *var_guard657_slot;
        let mut var_guard658: f64 = *var_guard658_slot;
        let mut var_sigrat: f64 = *var_sigrat_slot;
        let mut var_sigrat_dn0: f64 = *var_sigrat_dn0_slot;
        let mut var_sigrat_dn10: f64 = *var_sigrat_dn10_slot;
        let mut var_sigrat_dn11: f64 = *var_sigrat_dn11_slot;
        let mut var_sigrat_dn13: f64 = *var_sigrat_dn13_slot;
        let mut var_sigrat_dn14: f64 = *var_sigrat_dn14_slot;
        let mut var_sigrat_dn2: f64 = *var_sigrat_dn2_slot;
        let mut var_sigrat_dn3: f64 = *var_sigrat_dn3_slot;
        let mut var_sigrat_dn4: f64 = *var_sigrat_dn4_slot;
        let mut var_sigrat_dn5: f64 = *var_sigrat_dn5_slot;
        let mut var_sigrat_dn6: f64 = *var_sigrat_dn6_slot;
        let mut var_sigrat_dn7: f64 = *var_sigrat_dn7_slot;
        let mut var_sigrat_dn8: f64 = *var_sigrat_dn8_slot;
        let mut var_sigrat_dn9: f64 = *var_sigrat_dn9_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn0: f64 = *var_t8_dn0_slot;
        let mut var_t8_dn10: f64 = *var_t8_dn10_slot;
        let mut var_t8_dn11: f64 = *var_t8_dn11_slot;
        let mut var_t8_dn13: f64 = *var_t8_dn13_slot;
        let mut var_t8_dn14: f64 = *var_t8_dn14_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn3: f64 = *var_t8_dn3_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_dn9: f64 = *var_t8_dn9_slot;

        let (assign33950_e56643, assign33950_e56643_d_n0, assign33950_e56643_d_n2, assign33950_e56643_d_n3, assign33950_e56643_d_n4, assign33950_e56643_d_n5, assign33950_e56643_d_n6, assign33950_e56643_d_n7, assign33950_e56643_d_n8, assign33950_e56643_d_n9, assign33950_e56643_d_n10, assign33950_e56643_d_n11, assign33950_e56643_d_n13, assign33950_e56643_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33950_e56641: f64 = (0.5 * var_t4);
        (assign33950_e56641, (0.5 * var_t4_dn0), (0.5 * var_t4_dn2), (0.5 * var_t4_dn3), (0.5 * var_t4_dn4), (0.5 * var_t4_dn5), (0.5 * var_t4_dn6), (0.5 * var_t4_dn7), (0.5 * var_t4_dn8), (0.5 * var_t4_dn9), (0.5 * var_t4_dn10), (0.5 * var_t4_dn11), (0.5 * var_t4_dn13), (0.5 * var_t4_dn14),)
    } else {
        (var_gamma1, var_gamma1_dn0, var_gamma1_dn2, var_gamma1_dn3, var_gamma1_dn4, var_gamma1_dn5, var_gamma1_dn6, var_gamma1_dn7, var_gamma1_dn8, var_gamma1_dn9, var_gamma1_dn10, var_gamma1_dn11, var_gamma1_dn13, var_gamma1_dn14,)
    }
};
        var_gamma1 = assign33950_e56643;
        var_gamma1_dn0 = assign33950_e56643_d_n0;
        var_gamma1_dn2 = assign33950_e56643_d_n2;
        var_gamma1_dn3 = assign33950_e56643_d_n3;
        var_gamma1_dn4 = assign33950_e56643_d_n4;
        var_gamma1_dn5 = assign33950_e56643_d_n5;
        var_gamma1_dn6 = assign33950_e56643_d_n6;
        var_gamma1_dn7 = assign33950_e56643_d_n7;
        var_gamma1_dn8 = assign33950_e56643_d_n8;
        var_gamma1_dn9 = assign33950_e56643_d_n9;
        var_gamma1_dn10 = assign33950_e56643_d_n10;
        var_gamma1_dn11 = assign33950_e56643_d_n11;
        var_gamma1_dn13 = assign33950_e56643_d_n13;
        var_gamma1_dn14 = assign33950_e56643_d_n14;

        let (assign33960_e56654, assign33960_e56654_d_n0, assign33960_e56654_d_n2, assign33960_e56654_d_n3, assign33960_e56654_d_n4, assign33960_e56654_d_n5, assign33960_e56654_d_n6, assign33960_e56654_d_n7, assign33960_e56654_d_n8, assign33960_e56654_d_n9, assign33960_e56654_d_n10, assign33960_e56654_d_n11, assign33960_e56654_d_n13, assign33960_e56654_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33960_e56651: f64 = (6.0 * var_t7);
        let assign33960_e56652: f64 = (var_t5_2 / assign33960_e56651);
        (assign33960_e56652, (((var_t5_2_dn0 * assign33960_e56651) - (var_t5_2 * (6.0 * var_t7_dn0))) / (assign33960_e56651 * assign33960_e56651)), (((var_t5_2_dn2 * assign33960_e56651) - (var_t5_2 * (6.0 * var_t7_dn2))) / (assign33960_e56651 * assign33960_e56651)), (((var_t5_2_dn3 * assign33960_e56651) - (var_t5_2 * (6.0 * var_t7_dn3))) / (assign33960_e56651 * assign33960_e56651)), (((var_t5_2_dn4 * assign33960_e56651) - (var_t5_2 * (6.0 * var_t7_dn4))) / (assign33960_e56651 * assign33960_e56651)), (((var_t5_2_dn5 * assign33960_e56651) - (var_t5_2 * (6.0 * var_t7_dn5))) / (assign33960_e56651 * assign33960_e56651)), (((var_t5_2_dn6 * assign33960_e56651) - (var_t5_2 * (6.0 * var_t7_dn6))) / (assign33960_e56651 * assign33960_e56651)), (((var_t5_2_dn7 * assign33960_e56651) - (var_t5_2 * (6.0 * var_t7_dn7))) / (assign33960_e56651 * assign33960_e56651)), (((var_t5_2_dn8 * assign33960_e56651) - (var_t5_2 * (6.0 * var_t7_dn8))) / (assign33960_e56651 * assign33960_e56651)), (((var_t5_2_dn9 * assign33960_e56651) - (var_t5_2 * (6.0 * var_t7_dn9))) / (assign33960_e56651 * assign33960_e56651)), (((var_t5_2_dn10 * assign33960_e56651) - (var_t5_2 * (6.0 * var_t7_dn10))) / (assign33960_e56651 * assign33960_e56651)), (((var_t5_2_dn11 * assign33960_e56651) - (var_t5_2 * (6.0 * var_t7_dn11))) / (assign33960_e56651 * assign33960_e56651)), (((var_t5_2_dn13 * assign33960_e56651) - (var_t5_2 * (6.0 * var_t7_dn13))) / (assign33960_e56651 * assign33960_e56651)), (((var_t5_2_dn14 * assign33960_e56651) - (var_t5_2 * (6.0 * var_t7_dn14))) / (assign33960_e56651 * assign33960_e56651)),)
    } else {
        (var_gamma2, var_gamma2_dn0, var_gamma2_dn2, var_gamma2_dn3, var_gamma2_dn4, var_gamma2_dn5, var_gamma2_dn6, var_gamma2_dn7, var_gamma2_dn8, var_gamma2_dn9, var_gamma2_dn10, var_gamma2_dn11, var_gamma2_dn13, var_gamma2_dn14,)
    }
};
        var_gamma2 = assign33960_e56654;
        var_gamma2_dn0 = assign33960_e56654_d_n0;
        var_gamma2_dn2 = assign33960_e56654_d_n2;
        var_gamma2_dn3 = assign33960_e56654_d_n3;
        var_gamma2_dn4 = assign33960_e56654_d_n4;
        var_gamma2_dn5 = assign33960_e56654_d_n5;
        var_gamma2_dn6 = assign33960_e56654_d_n6;
        var_gamma2_dn7 = assign33960_e56654_d_n7;
        var_gamma2_dn8 = assign33960_e56654_d_n8;
        var_gamma2_dn9 = assign33960_e56654_d_n9;
        var_gamma2_dn10 = assign33960_e56654_d_n10;
        var_gamma2_dn11 = assign33960_e56654_d_n11;
        var_gamma2_dn13 = assign33960_e56654_d_n13;
        var_gamma2_dn14 = assign33960_e56654_d_n14;

        let (assign33970_e56667, assign33970_e56667_d_n0, assign33970_e56667_d_n2, assign33970_e56667_d_n3, assign33970_e56667_d_n4, assign33970_e56667_d_n5, assign33970_e56667_d_n6, assign33970_e56667_d_n7, assign33970_e56667_d_n8, assign33970_e56667_d_n9, assign33970_e56667_d_n10, assign33970_e56667_d_n11, assign33970_e56667_d_n13, assign33970_e56667_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33970_e56661: f64 = (var_moc / var_dvsat);
        let assign33970_e56664: f64 = (var_gamma1 + var_gamma2);
        let assign33970_e56665: f64 = (assign33970_e56661 * assign33970_e56664);
        (assign33970_e56665, (((((var_moc_dn0 * var_dvsat) - (var_moc * var_dvsat_dn0)) / (var_dvsat * var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (var_gamma1_dn0 + var_gamma2_dn0))), (((((var_moc_dn2 * var_dvsat) - (var_moc * var_dvsat_dn2)) / (var_dvsat * var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (var_gamma1_dn2 + var_gamma2_dn2))), (((((var_moc_dn3 * var_dvsat) - (var_moc * var_dvsat_dn3)) / (var_dvsat * var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (var_gamma1_dn3 + var_gamma2_dn3))), (((((var_moc_dn4 * var_dvsat) - (var_moc * var_dvsat_dn4)) / (var_dvsat * var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (var_gamma1_dn4 + var_gamma2_dn4))), (((((var_moc_dn5 * var_dvsat) - (var_moc * var_dvsat_dn5)) / (var_dvsat * var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (var_gamma1_dn5 + var_gamma2_dn5))), (((((var_moc_dn6 * var_dvsat) - (var_moc * var_dvsat_dn6)) / (var_dvsat * var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (var_gamma1_dn6 + var_gamma2_dn6))), (((((var_moc_dn7 * var_dvsat) - (var_moc * var_dvsat_dn7)) / (var_dvsat * var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (var_gamma1_dn7 + var_gamma2_dn7))), (((((var_moc_dn8 * var_dvsat) - (var_moc * var_dvsat_dn8)) / (var_dvsat * var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (var_gamma1_dn8 + var_gamma2_dn8))), (((((var_moc_dn9 * var_dvsat) - (var_moc * var_dvsat_dn9)) / (var_dvsat * var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (var_gamma1_dn9 + var_gamma2_dn9))), (((((var_moc_dn10 * var_dvsat) - (var_moc * var_dvsat_dn10)) / (var_dvsat * var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (var_gamma1_dn10 + var_gamma2_dn10))), (((((var_moc_dn11 * var_dvsat) - (var_moc * var_dvsat_dn11)) / (var_dvsat * var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (var_gamma1_dn11 + var_gamma2_dn11))), (((((var_moc_dn13 * var_dvsat) - (var_moc * var_dvsat_dn13)) / (var_dvsat * var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (var_gamma1_dn13 + var_gamma2_dn13))), (((((var_moc_dn14 * var_dvsat) - (var_moc * var_dvsat_dn14)) / (var_dvsat * var_dvsat)) * assign33970_e56664) + (assign33970_e56661 * (var_gamma1_dn14 + var_gamma2_dn14))),)
    } else {
        (var_gamma, var_gamma_dn0, var_gamma_dn2, var_gamma_dn3, var_gamma_dn4, var_gamma_dn5, var_gamma_dn6, var_gamma_dn7, var_gamma_dn8, var_gamma_dn9, var_gamma_dn10, var_gamma_dn11, var_gamma_dn13, var_gamma_dn14,)
    }
};
        var_gamma = assign33970_e56667;
        var_gamma_dn0 = assign33970_e56667_d_n0;
        var_gamma_dn2 = assign33970_e56667_d_n2;
        var_gamma_dn3 = assign33970_e56667_d_n3;
        var_gamma_dn4 = assign33970_e56667_d_n4;
        var_gamma_dn5 = assign33970_e56667_d_n5;
        var_gamma_dn6 = assign33970_e56667_d_n6;
        var_gamma_dn7 = assign33970_e56667_d_n7;
        var_gamma_dn8 = assign33970_e56667_d_n8;
        var_gamma_dn9 = assign33970_e56667_d_n9;
        var_gamma_dn10 = assign33970_e56667_d_n10;
        var_gamma_dn11 = assign33970_e56667_d_n11;
        var_gamma_dn13 = assign33970_e56667_d_n13;
        var_gamma_dn14 = assign33970_e56667_d_n14;

        let (assign33980_e56676, assign33980_e56676_d_n0, assign33980_e56676_d_n2, assign33980_e56676_d_n3, assign33980_e56676_d_n4, assign33980_e56676_d_n5, assign33980_e56676_d_n6, assign33980_e56676_d_n7, assign33980_e56676_d_n8, assign33980_e56676_d_n9, assign33980_e56676_d_n10, assign33980_e56676_d_n11, assign33980_e56676_d_n13, assign33980_e56676_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33980_e56674: f64 = (var_t4 / var_t7_2);
        (assign33980_e56674, (((var_t4_dn0 * var_t7_2) - (var_t4 * var_t7_2_dn0)) / (var_t7_2 * var_t7_2)), (((var_t4_dn2 * var_t7_2) - (var_t4 * var_t7_2_dn2)) / (var_t7_2 * var_t7_2)), (((var_t4_dn3 * var_t7_2) - (var_t4 * var_t7_2_dn3)) / (var_t7_2 * var_t7_2)), (((var_t4_dn4 * var_t7_2) - (var_t4 * var_t7_2_dn4)) / (var_t7_2 * var_t7_2)), (((var_t4_dn5 * var_t7_2) - (var_t4 * var_t7_2_dn5)) / (var_t7_2 * var_t7_2)), (((var_t4_dn6 * var_t7_2) - (var_t4 * var_t7_2_dn6)) / (var_t7_2 * var_t7_2)), (((var_t4_dn7 * var_t7_2) - (var_t4 * var_t7_2_dn7)) / (var_t7_2 * var_t7_2)), (((var_t4_dn8 * var_t7_2) - (var_t4 * var_t7_2_dn8)) / (var_t7_2 * var_t7_2)), (((var_t4_dn9 * var_t7_2) - (var_t4 * var_t7_2_dn9)) / (var_t7_2 * var_t7_2)), (((var_t4_dn10 * var_t7_2) - (var_t4 * var_t7_2_dn10)) / (var_t7_2 * var_t7_2)), (((var_t4_dn11 * var_t7_2) - (var_t4 * var_t7_2_dn11)) / (var_t7_2 * var_t7_2)), (((var_t4_dn13 * var_t7_2) - (var_t4 * var_t7_2_dn13)) / (var_t7_2 * var_t7_2)), (((var_t4_dn14 * var_t7_2) - (var_t4 * var_t7_2_dn14)) / (var_t7_2 * var_t7_2)),)
    } else {
        (var_delta1, var_delta1_dn0, var_delta1_dn2, var_delta1_dn3, var_delta1_dn4, var_delta1_dn5, var_delta1_dn6, var_delta1_dn7, var_delta1_dn8, var_delta1_dn9, var_delta1_dn10, var_delta1_dn11, var_delta1_dn13, var_delta1_dn14,)
    }
};
        var_delta1 = assign33980_e56676;
        var_delta1_dn0 = assign33980_e56676_d_n0;
        var_delta1_dn2 = assign33980_e56676_d_n2;
        var_delta1_dn3 = assign33980_e56676_d_n3;
        var_delta1_dn4 = assign33980_e56676_d_n4;
        var_delta1_dn5 = assign33980_e56676_d_n5;
        var_delta1_dn6 = assign33980_e56676_d_n6;
        var_delta1_dn7 = assign33980_e56676_d_n7;
        var_delta1_dn8 = assign33980_e56676_d_n8;
        var_delta1_dn9 = assign33980_e56676_d_n9;
        var_delta1_dn10 = assign33980_e56676_d_n10;
        var_delta1_dn11 = assign33980_e56676_d_n11;
        var_delta1_dn13 = assign33980_e56676_d_n13;
        var_delta1_dn14 = assign33980_e56676_d_n14;

        let (assign33990_e56693, assign33990_e56693_d_n0, assign33990_e56693_d_n2, assign33990_e56693_d_n3, assign33990_e56693_d_n4, assign33990_e56693_d_n5, assign33990_e56693_d_n6, assign33990_e56693_d_n7, assign33990_e56693_d_n8, assign33990_e56693_d_n9, assign33990_e56693_d_n10, assign33990_e56693_d_n11, assign33990_e56693_d_n13, assign33990_e56693_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign33990_e56683: f64 = (6.0 * var_t4);
        let assign33990_e56685: f64 = (assign33990_e56683 + var_t6);
        let assign33990_e56687: f64 = (assign33990_e56685 * var_t5_2);
        let assign33990_e56690: f64 = (15.0 * var_t7_4);
        let assign33990_e56691: f64 = (assign33990_e56687 / assign33990_e56690);
        (assign33990_e56691, (((((((6.0 * var_t4_dn0) + var_t6_dn0) * var_t5_2) + (assign33990_e56685 * var_t5_2_dn0)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * var_t7_4_dn0))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * var_t4_dn2) + var_t6_dn2) * var_t5_2) + (assign33990_e56685 * var_t5_2_dn2)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * var_t7_4_dn2))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * var_t4_dn3) + var_t6_dn3) * var_t5_2) + (assign33990_e56685 * var_t5_2_dn3)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * var_t7_4_dn3))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * var_t4_dn4) + var_t6_dn4) * var_t5_2) + (assign33990_e56685 * var_t5_2_dn4)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * var_t7_4_dn4))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * var_t4_dn5) + var_t6_dn5) * var_t5_2) + (assign33990_e56685 * var_t5_2_dn5)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * var_t7_4_dn5))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * var_t4_dn6) + var_t6_dn6) * var_t5_2) + (assign33990_e56685 * var_t5_2_dn6)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * var_t7_4_dn6))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * var_t4_dn7) + var_t6_dn7) * var_t5_2) + (assign33990_e56685 * var_t5_2_dn7)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * var_t7_4_dn7))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * var_t4_dn8) + var_t6_dn8) * var_t5_2) + (assign33990_e56685 * var_t5_2_dn8)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * var_t7_4_dn8))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * var_t4_dn9) + var_t6_dn9) * var_t5_2) + (assign33990_e56685 * var_t5_2_dn9)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * var_t7_4_dn9))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * var_t4_dn10) + var_t6_dn10) * var_t5_2) + (assign33990_e56685 * var_t5_2_dn10)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * var_t7_4_dn10))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * var_t4_dn11) + var_t6_dn11) * var_t5_2) + (assign33990_e56685 * var_t5_2_dn11)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * var_t7_4_dn11))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * var_t4_dn13) + var_t6_dn13) * var_t5_2) + (assign33990_e56685 * var_t5_2_dn13)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * var_t7_4_dn13))) / (assign33990_e56690 * assign33990_e56690)), (((((((6.0 * var_t4_dn14) + var_t6_dn14) * var_t5_2) + (assign33990_e56685 * var_t5_2_dn14)) * assign33990_e56690) - (assign33990_e56687 * (15.0 * var_t7_4_dn14))) / (assign33990_e56690 * assign33990_e56690)),)
    } else {
        (var_delta2, var_delta2_dn0, var_delta2_dn2, var_delta2_dn3, var_delta2_dn4, var_delta2_dn5, var_delta2_dn6, var_delta2_dn7, var_delta2_dn8, var_delta2_dn9, var_delta2_dn10, var_delta2_dn11, var_delta2_dn13, var_delta2_dn14,)
    }
};
        var_delta2 = assign33990_e56693;
        var_delta2_dn0 = assign33990_e56693_d_n0;
        var_delta2_dn2 = assign33990_e56693_d_n2;
        var_delta2_dn3 = assign33990_e56693_d_n3;
        var_delta2_dn4 = assign33990_e56693_d_n4;
        var_delta2_dn5 = assign33990_e56693_d_n5;
        var_delta2_dn6 = assign33990_e56693_d_n6;
        var_delta2_dn7 = assign33990_e56693_d_n7;
        var_delta2_dn8 = assign33990_e56693_d_n8;
        var_delta2_dn9 = assign33990_e56693_d_n9;
        var_delta2_dn10 = assign33990_e56693_d_n10;
        var_delta2_dn11 = assign33990_e56693_d_n11;
        var_delta2_dn13 = assign33990_e56693_d_n13;
        var_delta2_dn14 = assign33990_e56693_d_n14;

        let (assign34000_e56704, assign34000_e56704_d_n0, assign34000_e56704_d_n2, assign34000_e56704_d_n3, assign34000_e56704_d_n4, assign34000_e56704_d_n5, assign34000_e56704_d_n6, assign34000_e56704_d_n7, assign34000_e56704_d_n8, assign34000_e56704_d_n9, assign34000_e56704_d_n10, assign34000_e56704_d_n11, assign34000_e56704_d_n13, assign34000_e56704_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign34000_e56701: f64 = (9.0 * var_t7_5);
        let assign34000_e56702: f64 = (var_t5_4 / assign34000_e56701);
        (assign34000_e56702, (((var_t5_4_dn0 * assign34000_e56701) - (var_t5_4 * (9.0 * var_t7_5_dn0))) / (assign34000_e56701 * assign34000_e56701)), (((var_t5_4_dn2 * assign34000_e56701) - (var_t5_4 * (9.0 * var_t7_5_dn2))) / (assign34000_e56701 * assign34000_e56701)), (((var_t5_4_dn3 * assign34000_e56701) - (var_t5_4 * (9.0 * var_t7_5_dn3))) / (assign34000_e56701 * assign34000_e56701)), (((var_t5_4_dn4 * assign34000_e56701) - (var_t5_4 * (9.0 * var_t7_5_dn4))) / (assign34000_e56701 * assign34000_e56701)), (((var_t5_4_dn5 * assign34000_e56701) - (var_t5_4 * (9.0 * var_t7_5_dn5))) / (assign34000_e56701 * assign34000_e56701)), (((var_t5_4_dn6 * assign34000_e56701) - (var_t5_4 * (9.0 * var_t7_5_dn6))) / (assign34000_e56701 * assign34000_e56701)), (((var_t5_4_dn7 * assign34000_e56701) - (var_t5_4 * (9.0 * var_t7_5_dn7))) / (assign34000_e56701 * assign34000_e56701)), (((var_t5_4_dn8 * assign34000_e56701) - (var_t5_4 * (9.0 * var_t7_5_dn8))) / (assign34000_e56701 * assign34000_e56701)), (((var_t5_4_dn9 * assign34000_e56701) - (var_t5_4 * (9.0 * var_t7_5_dn9))) / (assign34000_e56701 * assign34000_e56701)), (((var_t5_4_dn10 * assign34000_e56701) - (var_t5_4 * (9.0 * var_t7_5_dn10))) / (assign34000_e56701 * assign34000_e56701)), (((var_t5_4_dn11 * assign34000_e56701) - (var_t5_4 * (9.0 * var_t7_5_dn11))) / (assign34000_e56701 * assign34000_e56701)), (((var_t5_4_dn13 * assign34000_e56701) - (var_t5_4 * (9.0 * var_t7_5_dn13))) / (assign34000_e56701 * assign34000_e56701)), (((var_t5_4_dn14 * assign34000_e56701) - (var_t5_4 * (9.0 * var_t7_5_dn14))) / (assign34000_e56701 * assign34000_e56701)),)
    } else {
        (var_delta3, var_delta3_dn0, var_delta3_dn2, var_delta3_dn3, var_delta3_dn4, var_delta3_dn5, var_delta3_dn6, var_delta3_dn7, var_delta3_dn8, var_delta3_dn9, var_delta3_dn10, var_delta3_dn11, var_delta3_dn13, var_delta3_dn14,)
    }
};
        var_delta3 = assign34000_e56704;
        var_delta3_dn0 = assign34000_e56704_d_n0;
        var_delta3_dn2 = assign34000_e56704_d_n2;
        var_delta3_dn3 = assign34000_e56704_d_n3;
        var_delta3_dn4 = assign34000_e56704_d_n4;
        var_delta3_dn5 = assign34000_e56704_d_n5;
        var_delta3_dn6 = assign34000_e56704_d_n6;
        var_delta3_dn7 = assign34000_e56704_d_n7;
        var_delta3_dn8 = assign34000_e56704_d_n8;
        var_delta3_dn9 = assign34000_e56704_d_n9;
        var_delta3_dn10 = assign34000_e56704_d_n10;
        var_delta3_dn11 = assign34000_e56704_d_n11;
        var_delta3_dn13 = assign34000_e56704_d_n13;
        var_delta3_dn14 = assign34000_e56704_d_n14;

        let (assign34010_e56721, assign34010_e56721_d_n0, assign34010_e56721_d_n2, assign34010_e56721_d_n3, assign34010_e56721_d_n4, assign34010_e56721_d_n5, assign34010_e56721_d_n6, assign34010_e56721_d_n7, assign34010_e56721_d_n8, assign34010_e56721_d_n9, assign34010_e56721_d_n10, assign34010_e56721_d_n11, assign34010_e56721_d_n13, assign34010_e56721_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign34010_e56711: f64 = (var_moc / 6.0);
        let assign34010_e56713: f64 = (assign34010_e56711 * var_dvsat3);
        let assign34010_e56716: f64 = (var_delta1 - var_delta2);
        let assign34010_e56718: f64 = (assign34010_e56716 + var_delta3);
        let assign34010_e56719: f64 = (assign34010_e56713 * assign34010_e56718);
        (assign34010_e56719, (((((var_moc_dn0 / 6.0) * var_dvsat3) + (assign34010_e56711 * var_dvsat3_dn0)) * assign34010_e56718) + (assign34010_e56713 * ((var_delta1_dn0 - var_delta2_dn0) + var_delta3_dn0))), (((((var_moc_dn2 / 6.0) * var_dvsat3) + (assign34010_e56711 * var_dvsat3_dn2)) * assign34010_e56718) + (assign34010_e56713 * ((var_delta1_dn2 - var_delta2_dn2) + var_delta3_dn2))), (((((var_moc_dn3 / 6.0) * var_dvsat3) + (assign34010_e56711 * var_dvsat3_dn3)) * assign34010_e56718) + (assign34010_e56713 * ((var_delta1_dn3 - var_delta2_dn3) + var_delta3_dn3))), (((((var_moc_dn4 / 6.0) * var_dvsat3) + (assign34010_e56711 * var_dvsat3_dn4)) * assign34010_e56718) + (assign34010_e56713 * ((var_delta1_dn4 - var_delta2_dn4) + var_delta3_dn4))), (((((var_moc_dn5 / 6.0) * var_dvsat3) + (assign34010_e56711 * var_dvsat3_dn5)) * assign34010_e56718) + (assign34010_e56713 * ((var_delta1_dn5 - var_delta2_dn5) + var_delta3_dn5))), (((((var_moc_dn6 / 6.0) * var_dvsat3) + (assign34010_e56711 * var_dvsat3_dn6)) * assign34010_e56718) + (assign34010_e56713 * ((var_delta1_dn6 - var_delta2_dn6) + var_delta3_dn6))), (((((var_moc_dn7 / 6.0) * var_dvsat3) + (assign34010_e56711 * var_dvsat3_dn7)) * assign34010_e56718) + (assign34010_e56713 * ((var_delta1_dn7 - var_delta2_dn7) + var_delta3_dn7))), (((((var_moc_dn8 / 6.0) * var_dvsat3) + (assign34010_e56711 * var_dvsat3_dn8)) * assign34010_e56718) + (assign34010_e56713 * ((var_delta1_dn8 - var_delta2_dn8) + var_delta3_dn8))), (((((var_moc_dn9 / 6.0) * var_dvsat3) + (assign34010_e56711 * var_dvsat3_dn9)) * assign34010_e56718) + (assign34010_e56713 * ((var_delta1_dn9 - var_delta2_dn9) + var_delta3_dn9))), (((((var_moc_dn10 / 6.0) * var_dvsat3) + (assign34010_e56711 * var_dvsat3_dn10)) * assign34010_e56718) + (assign34010_e56713 * ((var_delta1_dn10 - var_delta2_dn10) + var_delta3_dn10))), (((((var_moc_dn11 / 6.0) * var_dvsat3) + (assign34010_e56711 * var_dvsat3_dn11)) * assign34010_e56718) + (assign34010_e56713 * ((var_delta1_dn11 - var_delta2_dn11) + var_delta3_dn11))), (((((var_moc_dn13 / 6.0) * var_dvsat3) + (assign34010_e56711 * var_dvsat3_dn13)) * assign34010_e56718) + (assign34010_e56713 * ((var_delta1_dn13 - var_delta2_dn13) + var_delta3_dn13))), (((((var_moc_dn14 / 6.0) * var_dvsat3) + (assign34010_e56711 * var_dvsat3_dn14)) * assign34010_e56718) + (assign34010_e56713 * ((var_delta1_dn14 - var_delta2_dn14) + var_delta3_dn14))),)
    } else {
        (var_delta, var_delta_dn0, var_delta_dn2, var_delta_dn3, var_delta_dn4, var_delta_dn5, var_delta_dn6, var_delta_dn7, var_delta_dn8, var_delta_dn9, var_delta_dn10, var_delta_dn11, var_delta_dn13, var_delta_dn14,)
    }
};
        var_delta = assign34010_e56721;
        var_delta_dn0 = assign34010_e56721_d_n0;
        var_delta_dn2 = assign34010_e56721_d_n2;
        var_delta_dn3 = assign34010_e56721_d_n3;
        var_delta_dn4 = assign34010_e56721_d_n4;
        var_delta_dn5 = assign34010_e56721_d_n5;
        var_delta_dn6 = assign34010_e56721_d_n6;
        var_delta_dn7 = assign34010_e56721_d_n7;
        var_delta_dn8 = assign34010_e56721_d_n8;
        var_delta_dn9 = assign34010_e56721_d_n9;
        var_delta_dn10 = assign34010_e56721_d_n10;
        var_delta_dn11 = assign34010_e56721_d_n11;
        var_delta_dn13 = assign34010_e56721_d_n13;
        var_delta_dn14 = assign34010_e56721_d_n14;

        let (assign34020_e56730, assign34020_e56730_d_n0, assign34020_e56730_d_n2, assign34020_e56730_d_n3, assign34020_e56730_d_n4, assign34020_e56730_d_n5, assign34020_e56730_d_n6, assign34020_e56730_d_n7, assign34020_e56730_d_n8, assign34020_e56730_d_n9, assign34020_e56730_d_n10, assign34020_e56730_d_n11, assign34020_e56730_d_n13, assign34020_e56730_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign34020_e56728: f64 = (var_t5 / var_t7);
        (assign34020_e56728, (((var_t5_dn0 * var_t7) - (var_t5 * var_t7_dn0)) / (var_t7 * var_t7)), (((var_t5_dn2 * var_t7) - (var_t5 * var_t7_dn2)) / (var_t7 * var_t7)), (((var_t5_dn3 * var_t7) - (var_t5 * var_t7_dn3)) / (var_t7 * var_t7)), (((var_t5_dn4 * var_t7) - (var_t5 * var_t7_dn4)) / (var_t7 * var_t7)), (((var_t5_dn5 * var_t7) - (var_t5 * var_t7_dn5)) / (var_t7 * var_t7)), (((var_t5_dn6 * var_t7) - (var_t5 * var_t7_dn6)) / (var_t7 * var_t7)), (((var_t5_dn7 * var_t7) - (var_t5 * var_t7_dn7)) / (var_t7 * var_t7)), (((var_t5_dn8 * var_t7) - (var_t5 * var_t7_dn8)) / (var_t7 * var_t7)), (((var_t5_dn9 * var_t7) - (var_t5 * var_t7_dn9)) / (var_t7 * var_t7)), (((var_t5_dn10 * var_t7) - (var_t5 * var_t7_dn10)) / (var_t7 * var_t7)), (((var_t5_dn11 * var_t7) - (var_t5 * var_t7_dn11)) / (var_t7 * var_t7)), (((var_t5_dn13 * var_t7) - (var_t5 * var_t7_dn13)) / (var_t7 * var_t7)), (((var_t5_dn14 * var_t7) - (var_t5 * var_t7_dn14)) / (var_t7 * var_t7)),)
    } else {
        (var_epsilon1, var_epsilon1_dn0, var_epsilon1_dn2, var_epsilon1_dn3, var_epsilon1_dn4, var_epsilon1_dn5, var_epsilon1_dn6, var_epsilon1_dn7, var_epsilon1_dn8, var_epsilon1_dn9, var_epsilon1_dn10, var_epsilon1_dn11, var_epsilon1_dn13, var_epsilon1_dn14,)
    }
};
        var_epsilon1 = assign34020_e56730;
        var_epsilon1_dn0 = assign34020_e56730_d_n0;
        var_epsilon1_dn2 = assign34020_e56730_d_n2;
        var_epsilon1_dn3 = assign34020_e56730_d_n3;
        var_epsilon1_dn4 = assign34020_e56730_d_n4;
        var_epsilon1_dn5 = assign34020_e56730_d_n5;
        var_epsilon1_dn6 = assign34020_e56730_d_n6;
        var_epsilon1_dn7 = assign34020_e56730_d_n7;
        var_epsilon1_dn8 = assign34020_e56730_d_n8;
        var_epsilon1_dn9 = assign34020_e56730_d_n9;
        var_epsilon1_dn10 = assign34020_e56730_d_n10;
        var_epsilon1_dn11 = assign34020_e56730_d_n11;
        var_epsilon1_dn13 = assign34020_e56730_d_n13;
        var_epsilon1_dn14 = assign34020_e56730_d_n14;

        let (assign34030_e56741, assign34030_e56741_d_n0, assign34030_e56741_d_n2, assign34030_e56741_d_n3, assign34030_e56741_d_n4, assign34030_e56741_d_n5, assign34030_e56741_d_n6, assign34030_e56741_d_n7, assign34030_e56741_d_n8, assign34030_e56741_d_n9, assign34030_e56741_d_n10, assign34030_e56741_d_n11, assign34030_e56741_d_n13, assign34030_e56741_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign34030_e56738: f64 = (3.0 * var_t7_3);
        let assign34030_e56739: f64 = (var_t5_3 / assign34030_e56738);
        (assign34030_e56739, (((var_t5_3_dn0 * assign34030_e56738) - (var_t5_3 * (3.0 * var_t7_3_dn0))) / (assign34030_e56738 * assign34030_e56738)), (((var_t5_3_dn2 * assign34030_e56738) - (var_t5_3 * (3.0 * var_t7_3_dn2))) / (assign34030_e56738 * assign34030_e56738)), (((var_t5_3_dn3 * assign34030_e56738) - (var_t5_3 * (3.0 * var_t7_3_dn3))) / (assign34030_e56738 * assign34030_e56738)), (((var_t5_3_dn4 * assign34030_e56738) - (var_t5_3 * (3.0 * var_t7_3_dn4))) / (assign34030_e56738 * assign34030_e56738)), (((var_t5_3_dn5 * assign34030_e56738) - (var_t5_3 * (3.0 * var_t7_3_dn5))) / (assign34030_e56738 * assign34030_e56738)), (((var_t5_3_dn6 * assign34030_e56738) - (var_t5_3 * (3.0 * var_t7_3_dn6))) / (assign34030_e56738 * assign34030_e56738)), (((var_t5_3_dn7 * assign34030_e56738) - (var_t5_3 * (3.0 * var_t7_3_dn7))) / (assign34030_e56738 * assign34030_e56738)), (((var_t5_3_dn8 * assign34030_e56738) - (var_t5_3 * (3.0 * var_t7_3_dn8))) / (assign34030_e56738 * assign34030_e56738)), (((var_t5_3_dn9 * assign34030_e56738) - (var_t5_3 * (3.0 * var_t7_3_dn9))) / (assign34030_e56738 * assign34030_e56738)), (((var_t5_3_dn10 * assign34030_e56738) - (var_t5_3 * (3.0 * var_t7_3_dn10))) / (assign34030_e56738 * assign34030_e56738)), (((var_t5_3_dn11 * assign34030_e56738) - (var_t5_3 * (3.0 * var_t7_3_dn11))) / (assign34030_e56738 * assign34030_e56738)), (((var_t5_3_dn13 * assign34030_e56738) - (var_t5_3 * (3.0 * var_t7_3_dn13))) / (assign34030_e56738 * assign34030_e56738)), (((var_t5_3_dn14 * assign34030_e56738) - (var_t5_3 * (3.0 * var_t7_3_dn14))) / (assign34030_e56738 * assign34030_e56738)),)
    } else {
        (var_epsilon2, var_epsilon2_dn0, var_epsilon2_dn2, var_epsilon2_dn3, var_epsilon2_dn4, var_epsilon2_dn5, var_epsilon2_dn6, var_epsilon2_dn7, var_epsilon2_dn8, var_epsilon2_dn9, var_epsilon2_dn10, var_epsilon2_dn11, var_epsilon2_dn13, var_epsilon2_dn14,)
    }
};
        var_epsilon2 = assign34030_e56741;
        var_epsilon2_dn0 = assign34030_e56741_d_n0;
        var_epsilon2_dn2 = assign34030_e56741_d_n2;
        var_epsilon2_dn3 = assign34030_e56741_d_n3;
        var_epsilon2_dn4 = assign34030_e56741_d_n4;
        var_epsilon2_dn5 = assign34030_e56741_d_n5;
        var_epsilon2_dn6 = assign34030_e56741_d_n6;
        var_epsilon2_dn7 = assign34030_e56741_d_n7;
        var_epsilon2_dn8 = assign34030_e56741_d_n8;
        var_epsilon2_dn9 = assign34030_e56741_d_n9;
        var_epsilon2_dn10 = assign34030_e56741_d_n10;
        var_epsilon2_dn11 = assign34030_e56741_d_n11;
        var_epsilon2_dn13 = assign34030_e56741_d_n13;
        var_epsilon2_dn14 = assign34030_e56741_d_n14;

        let (assign34040_e56756, assign34040_e56756_d_n0, assign34040_e56756_d_n2, assign34040_e56756_d_n3, assign34040_e56756_d_n4, assign34040_e56756_d_n5, assign34040_e56756_d_n6, assign34040_e56756_d_n7, assign34040_e56756_d_n8, assign34040_e56756_d_n9, assign34040_e56756_d_n10, assign34040_e56756_d_n11, assign34040_e56756_d_n13, assign34040_e56756_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign34040_e56748: f64 = (var_moc / 6.0);
        let assign34040_e56750: f64 = (assign34040_e56748 * var_dvsat);
        let assign34040_e56753: f64 = (var_epsilon1 - var_epsilon2);
        let assign34040_e56754: f64 = (assign34040_e56750 * assign34040_e56753);
        (assign34040_e56754, (((((var_moc_dn0 / 6.0) * var_dvsat) + (assign34040_e56748 * var_dvsat_dn0)) * assign34040_e56753) + (assign34040_e56750 * (var_epsilon1_dn0 - var_epsilon2_dn0))), (((((var_moc_dn2 / 6.0) * var_dvsat) + (assign34040_e56748 * var_dvsat_dn2)) * assign34040_e56753) + (assign34040_e56750 * (var_epsilon1_dn2 - var_epsilon2_dn2))), (((((var_moc_dn3 / 6.0) * var_dvsat) + (assign34040_e56748 * var_dvsat_dn3)) * assign34040_e56753) + (assign34040_e56750 * (var_epsilon1_dn3 - var_epsilon2_dn3))), (((((var_moc_dn4 / 6.0) * var_dvsat) + (assign34040_e56748 * var_dvsat_dn4)) * assign34040_e56753) + (assign34040_e56750 * (var_epsilon1_dn4 - var_epsilon2_dn4))), (((((var_moc_dn5 / 6.0) * var_dvsat) + (assign34040_e56748 * var_dvsat_dn5)) * assign34040_e56753) + (assign34040_e56750 * (var_epsilon1_dn5 - var_epsilon2_dn5))), (((((var_moc_dn6 / 6.0) * var_dvsat) + (assign34040_e56748 * var_dvsat_dn6)) * assign34040_e56753) + (assign34040_e56750 * (var_epsilon1_dn6 - var_epsilon2_dn6))), (((((var_moc_dn7 / 6.0) * var_dvsat) + (assign34040_e56748 * var_dvsat_dn7)) * assign34040_e56753) + (assign34040_e56750 * (var_epsilon1_dn7 - var_epsilon2_dn7))), (((((var_moc_dn8 / 6.0) * var_dvsat) + (assign34040_e56748 * var_dvsat_dn8)) * assign34040_e56753) + (assign34040_e56750 * (var_epsilon1_dn8 - var_epsilon2_dn8))), (((((var_moc_dn9 / 6.0) * var_dvsat) + (assign34040_e56748 * var_dvsat_dn9)) * assign34040_e56753) + (assign34040_e56750 * (var_epsilon1_dn9 - var_epsilon2_dn9))), (((((var_moc_dn10 / 6.0) * var_dvsat) + (assign34040_e56748 * var_dvsat_dn10)) * assign34040_e56753) + (assign34040_e56750 * (var_epsilon1_dn10 - var_epsilon2_dn10))), (((((var_moc_dn11 / 6.0) * var_dvsat) + (assign34040_e56748 * var_dvsat_dn11)) * assign34040_e56753) + (assign34040_e56750 * (var_epsilon1_dn11 - var_epsilon2_dn11))), (((((var_moc_dn13 / 6.0) * var_dvsat) + (assign34040_e56748 * var_dvsat_dn13)) * assign34040_e56753) + (assign34040_e56750 * (var_epsilon1_dn13 - var_epsilon2_dn13))), (((((var_moc_dn14 / 6.0) * var_dvsat) + (assign34040_e56748 * var_dvsat_dn14)) * assign34040_e56753) + (assign34040_e56750 * (var_epsilon1_dn14 - var_epsilon2_dn14))),)
    } else {
        (var_epsilon, var_epsilon_dn0, var_epsilon_dn2, var_epsilon_dn3, var_epsilon_dn4, var_epsilon_dn5, var_epsilon_dn6, var_epsilon_dn7, var_epsilon_dn8, var_epsilon_dn9, var_epsilon_dn10, var_epsilon_dn11, var_epsilon_dn13, var_epsilon_dn14,)
    }
};
        var_epsilon = assign34040_e56756;
        var_epsilon_dn0 = assign34040_e56756_d_n0;
        var_epsilon_dn2 = assign34040_e56756_d_n2;
        var_epsilon_dn3 = assign34040_e56756_d_n3;
        var_epsilon_dn4 = assign34040_e56756_d_n4;
        var_epsilon_dn5 = assign34040_e56756_d_n5;
        var_epsilon_dn6 = assign34040_e56756_d_n6;
        var_epsilon_dn7 = assign34040_e56756_d_n7;
        var_epsilon_dn8 = assign34040_e56756_d_n8;
        var_epsilon_dn9 = assign34040_e56756_d_n9;
        var_epsilon_dn10 = assign34040_e56756_d_n10;
        var_epsilon_dn11 = assign34040_e56756_d_n11;
        var_epsilon_dn13 = assign34040_e56756_d_n13;
        var_epsilon_dn14 = assign34040_e56756_d_n14;

        let (assign34050_e56770, assign34050_e56770_d_n0, assign34050_e56770_d_n2, assign34050_e56770_d_n3, assign34050_e56770_d_n4, assign34050_e56770_d_n5, assign34050_e56770_d_n6, assign34050_e56770_d_n7, assign34050_e56770_d_n8, assign34050_e56770_d_n9, assign34050_e56770_d_n10, assign34050_e56770_d_n11, assign34050_e56770_d_n13, assign34050_e56770_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign34050_e56763: f64 = (var_t3 * var_epsilon);
        let assign34050_e56766: f64 = (var_gamma * var_delta);
        let assign34050_e56767: f64 = (assign34050_e56766).sqrt();
        let assign34050_e56768: f64 = (assign34050_e56763 / assign34050_e56767);
        (assign34050_e56768, (((((var_t3_dn0 * var_epsilon) + (var_t3 * var_epsilon_dn0)) * assign34050_e56767) - (assign34050_e56763 * (((var_gamma_dn0 * var_delta) + (var_gamma * var_delta_dn0)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((var_t3_dn2 * var_epsilon) + (var_t3 * var_epsilon_dn2)) * assign34050_e56767) - (assign34050_e56763 * (((var_gamma_dn2 * var_delta) + (var_gamma * var_delta_dn2)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((var_t3_dn3 * var_epsilon) + (var_t3 * var_epsilon_dn3)) * assign34050_e56767) - (assign34050_e56763 * (((var_gamma_dn3 * var_delta) + (var_gamma * var_delta_dn3)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((var_t3_dn4 * var_epsilon) + (var_t3 * var_epsilon_dn4)) * assign34050_e56767) - (assign34050_e56763 * (((var_gamma_dn4 * var_delta) + (var_gamma * var_delta_dn4)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((var_t3_dn5 * var_epsilon) + (var_t3 * var_epsilon_dn5)) * assign34050_e56767) - (assign34050_e56763 * (((var_gamma_dn5 * var_delta) + (var_gamma * var_delta_dn5)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((var_t3_dn6 * var_epsilon) + (var_t3 * var_epsilon_dn6)) * assign34050_e56767) - (assign34050_e56763 * (((var_gamma_dn6 * var_delta) + (var_gamma * var_delta_dn6)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((var_t3_dn7 * var_epsilon) + (var_t3 * var_epsilon_dn7)) * assign34050_e56767) - (assign34050_e56763 * (((var_gamma_dn7 * var_delta) + (var_gamma * var_delta_dn7)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((var_t3_dn8 * var_epsilon) + (var_t3 * var_epsilon_dn8)) * assign34050_e56767) - (assign34050_e56763 * (((var_gamma_dn8 * var_delta) + (var_gamma * var_delta_dn8)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((var_t3_dn9 * var_epsilon) + (var_t3 * var_epsilon_dn9)) * assign34050_e56767) - (assign34050_e56763 * (((var_gamma_dn9 * var_delta) + (var_gamma * var_delta_dn9)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((var_t3_dn10 * var_epsilon) + (var_t3 * var_epsilon_dn10)) * assign34050_e56767) - (assign34050_e56763 * (((var_gamma_dn10 * var_delta) + (var_gamma * var_delta_dn10)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((var_t3_dn11 * var_epsilon) + (var_t3 * var_epsilon_dn11)) * assign34050_e56767) - (assign34050_e56763 * (((var_gamma_dn11 * var_delta) + (var_gamma * var_delta_dn11)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((var_t3_dn13 * var_epsilon) + (var_t3 * var_epsilon_dn13)) * assign34050_e56767) - (assign34050_e56763 * (((var_gamma_dn13 * var_delta) + (var_gamma * var_delta_dn13)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)), (((((var_t3_dn14 * var_epsilon) + (var_t3 * var_epsilon_dn14)) * assign34050_e56767) - (assign34050_e56763 * (((var_gamma_dn14 * var_delta) + (var_gamma * var_delta_dn14)) / (2.0 * assign34050_e56767)))) / (assign34050_e56767 * assign34050_e56767)),)
    } else {
        (var_ctnoi, var_ctnoi_dn0, var_ctnoi_dn2, var_ctnoi_dn3, var_ctnoi_dn4, var_ctnoi_dn5, var_ctnoi_dn6, var_ctnoi_dn7, var_ctnoi_dn8, var_ctnoi_dn9, var_ctnoi_dn10, var_ctnoi_dn11, var_ctnoi_dn13, var_ctnoi_dn14,)
    }
};
        var_ctnoi = assign34050_e56770;
        var_ctnoi_dn0 = assign34050_e56770_d_n0;
        var_ctnoi_dn2 = assign34050_e56770_d_n2;
        var_ctnoi_dn3 = assign34050_e56770_d_n3;
        var_ctnoi_dn4 = assign34050_e56770_d_n4;
        var_ctnoi_dn5 = assign34050_e56770_d_n5;
        var_ctnoi_dn6 = assign34050_e56770_d_n6;
        var_ctnoi_dn7 = assign34050_e56770_d_n7;
        var_ctnoi_dn8 = assign34050_e56770_d_n8;
        var_ctnoi_dn9 = assign34050_e56770_d_n9;
        var_ctnoi_dn10 = assign34050_e56770_d_n10;
        var_ctnoi_dn11 = assign34050_e56770_d_n11;
        var_ctnoi_dn13 = assign34050_e56770_d_n13;
        var_ctnoi_dn14 = assign34050_e56770_d_n14;

        let assign34060_e56773: f64 = if var_ctnoi > 1.0 { 1.0 } else { 0.0 };
        var_guard639 = assign34060_e56773;

        let (assign34070_e56782, assign34070_e56782_d_n0, assign34070_e56782_d_n2, assign34070_e56782_d_n3, assign34070_e56782_d_n4, assign34070_e56782_d_n5, assign34070_e56782_d_n6, assign34070_e56782_d_n7, assign34070_e56782_d_n8, assign34070_e56782_d_n9, assign34070_e56782_d_n10, assign34070_e56782_d_n11, assign34070_e56782_d_n13, assign34070_e56782_d_n14,) = {
    if (((var_guard633 != 0.0) && (var_guard632 == 0.0)) && (var_guard639 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ctnoi, var_ctnoi_dn0, var_ctnoi_dn2, var_ctnoi_dn3, var_ctnoi_dn4, var_ctnoi_dn5, var_ctnoi_dn6, var_ctnoi_dn7, var_ctnoi_dn8, var_ctnoi_dn9, var_ctnoi_dn10, var_ctnoi_dn11, var_ctnoi_dn13, var_ctnoi_dn14,)
    }
};
        var_ctnoi = assign34070_e56782;
        var_ctnoi_dn0 = assign34070_e56782_d_n0;
        var_ctnoi_dn2 = assign34070_e56782_d_n2;
        var_ctnoi_dn3 = assign34070_e56782_d_n3;
        var_ctnoi_dn4 = assign34070_e56782_d_n4;
        var_ctnoi_dn5 = assign34070_e56782_d_n5;
        var_ctnoi_dn6 = assign34070_e56782_d_n6;
        var_ctnoi_dn7 = assign34070_e56782_d_n7;
        var_ctnoi_dn8 = assign34070_e56782_d_n8;
        var_ctnoi_dn9 = assign34070_e56782_d_n9;
        var_ctnoi_dn10 = assign34070_e56782_d_n10;
        var_ctnoi_dn11 = assign34070_e56782_d_n11;
        var_ctnoi_dn13 = assign34070_e56782_d_n13;
        var_ctnoi_dn14 = assign34070_e56782_d_n14;

        let assign34080_e56785: f64 = if var_ctnoi < 0.0 { 1.0 } else { 0.0 };
        var_guard640 = assign34080_e56785;

        let (assign34090_e56797, assign34090_e56797_d_n0, assign34090_e56797_d_n2, assign34090_e56797_d_n3, assign34090_e56797_d_n4, assign34090_e56797_d_n5, assign34090_e56797_d_n6, assign34090_e56797_d_n7, assign34090_e56797_d_n8, assign34090_e56797_d_n9, assign34090_e56797_d_n10, assign34090_e56797_d_n11, assign34090_e56797_d_n13, assign34090_e56797_d_n14,) = {
    if ((((var_guard633 != 0.0) && (var_guard632 == 0.0)) && (var_guard639 == 0.0)) && (var_guard640 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ctnoi, var_ctnoi_dn0, var_ctnoi_dn2, var_ctnoi_dn3, var_ctnoi_dn4, var_ctnoi_dn5, var_ctnoi_dn6, var_ctnoi_dn7, var_ctnoi_dn8, var_ctnoi_dn9, var_ctnoi_dn10, var_ctnoi_dn11, var_ctnoi_dn13, var_ctnoi_dn14,)
    }
};
        var_ctnoi = assign34090_e56797;
        var_ctnoi_dn0 = assign34090_e56797_d_n0;
        var_ctnoi_dn2 = assign34090_e56797_d_n2;
        var_ctnoi_dn3 = assign34090_e56797_d_n3;
        var_ctnoi_dn4 = assign34090_e56797_d_n4;
        var_ctnoi_dn5 = assign34090_e56797_d_n5;
        var_ctnoi_dn6 = assign34090_e56797_d_n6;
        var_ctnoi_dn7 = assign34090_e56797_d_n7;
        var_ctnoi_dn8 = assign34090_e56797_d_n8;
        var_ctnoi_dn9 = assign34090_e56797_d_n9;
        var_ctnoi_dn10 = assign34090_e56797_d_n10;
        var_ctnoi_dn11 = assign34090_e56797_d_n11;
        var_ctnoi_dn13 = assign34090_e56797_d_n13;
        var_ctnoi_dn14 = assign34090_e56797_d_n14;

        let (assign34100_e56816, assign34100_e56816_d_n0, assign34100_e56816_d_n2, assign34100_e56816_d_n3, assign34100_e56816_d_n4, assign34100_e56816_d_n5, assign34100_e56816_d_n6, assign34100_e56816_d_n7, assign34100_e56816_d_n8, assign34100_e56816_d_n9, assign34100_e56816_d_n10, assign34100_e56816_d_n11, assign34100_e56816_d_n13, assign34100_e56816_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign34100_e56805: f64 = (var_noilowid * var_noilowid);
        let assign34100_e56808: f64 = (p.p1716 + var_qia);
        let assign34100_e56809: f64 = (assign34100_e56805 / assign34100_e56808);
        let assign34100_e56812: f64 = (var_vdseff_1 / var_vdsat);
        let assign34100_e56813: f64 = (assign34100_e56809 * assign34100_e56812);
        let assign34100_e56814: f64 = (1.0 + assign34100_e56813);
        (assign34100_e56814, (((((((var_noilowid_dn0 * var_noilowid) + (var_noilowid * var_noilowid_dn0)) * assign34100_e56808) - (assign34100_e56805 * var_qia_dn0)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((var_vdseff_1_dn0 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn0)) / (var_vdsat * var_vdsat)))), (((((((var_noilowid_dn2 * var_noilowid) + (var_noilowid * var_noilowid_dn2)) * assign34100_e56808) - (assign34100_e56805 * var_qia_dn2)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((var_vdseff_1_dn2 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn2)) / (var_vdsat * var_vdsat)))), (((((((var_noilowid_dn3 * var_noilowid) + (var_noilowid * var_noilowid_dn3)) * assign34100_e56808) - (assign34100_e56805 * var_qia_dn3)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((var_vdseff_1_dn3 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn3)) / (var_vdsat * var_vdsat)))), (((((((var_noilowid_dn4 * var_noilowid) + (var_noilowid * var_noilowid_dn4)) * assign34100_e56808) - (assign34100_e56805 * var_qia_dn4)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((var_vdseff_1_dn4 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn4)) / (var_vdsat * var_vdsat)))), (((((((var_noilowid_dn5 * var_noilowid) + (var_noilowid * var_noilowid_dn5)) * assign34100_e56808) - (assign34100_e56805 * var_qia_dn5)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((var_vdseff_1_dn5 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn5)) / (var_vdsat * var_vdsat)))), (((((((var_noilowid_dn6 * var_noilowid) + (var_noilowid * var_noilowid_dn6)) * assign34100_e56808) - (assign34100_e56805 * var_qia_dn6)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((var_vdseff_1_dn6 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn6)) / (var_vdsat * var_vdsat)))), (((((((var_noilowid_dn7 * var_noilowid) + (var_noilowid * var_noilowid_dn7)) * assign34100_e56808) - (assign34100_e56805 * var_qia_dn7)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((var_vdseff_1_dn7 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn7)) / (var_vdsat * var_vdsat)))), (((((((var_noilowid_dn8 * var_noilowid) + (var_noilowid * var_noilowid_dn8)) * assign34100_e56808) - (assign34100_e56805 * var_qia_dn8)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((var_vdseff_1_dn8 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn8)) / (var_vdsat * var_vdsat)))), (((((((var_noilowid_dn9 * var_noilowid) + (var_noilowid * var_noilowid_dn9)) * assign34100_e56808) - (assign34100_e56805 * var_qia_dn9)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((var_vdseff_1_dn9 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn9)) / (var_vdsat * var_vdsat)))), (((((((var_noilowid_dn10 * var_noilowid) + (var_noilowid * var_noilowid_dn10)) * assign34100_e56808) - (assign34100_e56805 * var_qia_dn10)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((var_vdseff_1_dn10 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn10)) / (var_vdsat * var_vdsat)))), (((((((var_noilowid_dn11 * var_noilowid) + (var_noilowid * var_noilowid_dn11)) * assign34100_e56808) - (assign34100_e56805 * var_qia_dn11)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((var_vdseff_1_dn11 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn11)) / (var_vdsat * var_vdsat)))), (((((((var_noilowid_dn13 * var_noilowid) + (var_noilowid * var_noilowid_dn13)) * assign34100_e56808) - (assign34100_e56805 * var_qia_dn13)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((var_vdseff_1_dn13 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn13)) / (var_vdsat * var_vdsat)))), (((((((var_noilowid_dn14 * var_noilowid) + (var_noilowid * var_noilowid_dn14)) * assign34100_e56808) - (assign34100_e56805 * var_qia_dn14)) / (assign34100_e56808 * assign34100_e56808)) * assign34100_e56812) + (assign34100_e56809 * (((var_vdseff_1_dn14 * var_vdsat) - (var_vdseff_1 * var_vdsat_dn14)) / (var_vdsat * var_vdsat)))),)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn3, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn11, var_t8_dn13, var_t8_dn14,)
    }
};
        var_t8 = assign34100_e56816;
        var_t8_dn0 = assign34100_e56816_d_n0;
        var_t8_dn2 = assign34100_e56816_d_n2;
        var_t8_dn3 = assign34100_e56816_d_n3;
        var_t8_dn4 = assign34100_e56816_d_n4;
        var_t8_dn5 = assign34100_e56816_d_n5;
        var_t8_dn6 = assign34100_e56816_d_n6;
        var_t8_dn7 = assign34100_e56816_d_n7;
        var_t8_dn8 = assign34100_e56816_d_n8;
        var_t8_dn9 = assign34100_e56816_d_n9;
        var_t8_dn10 = assign34100_e56816_d_n10;
        var_t8_dn11 = assign34100_e56816_d_n11;
        var_t8_dn13 = assign34100_e56816_d_n13;
        var_t8_dn14 = assign34100_e56816_d_n14;

        let (assign34110_e56833, assign34110_e56833_d_n0, assign34110_e56833_d_n2, assign34110_e56833_d_n3, assign34110_e56833_d_n4, assign34110_e56833_d_n5, assign34110_e56833_d_n6, assign34110_e56833_d_n7, assign34110_e56833_d_n8, assign34110_e56833_d_n9, assign34110_e56833_d_n10, assign34110_e56833_d_n11, assign34110_e56833_d_n13, assign34110_e56833_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign34110_e56823: f64 = (var_moc / var_dvsat);
        let assign34110_e56826: f64 = (var_t8 * var_gamma1);
        let assign34110_e56829: f64 = (var_t1 * var_gamma2);
        let assign34110_e56830: f64 = (assign34110_e56826 + assign34110_e56829);
        let assign34110_e56831: f64 = (assign34110_e56823 * assign34110_e56830);
        (assign34110_e56831, (((((var_moc_dn0 * var_dvsat) - (var_moc * var_dvsat_dn0)) / (var_dvsat * var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((var_t8_dn0 * var_gamma1) + (var_t8 * var_gamma1_dn0)) + ((var_t1_dn0 * var_gamma2) + (var_t1 * var_gamma2_dn0))))), (((((var_moc_dn2 * var_dvsat) - (var_moc * var_dvsat_dn2)) / (var_dvsat * var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((var_t8_dn2 * var_gamma1) + (var_t8 * var_gamma1_dn2)) + ((var_t1_dn2 * var_gamma2) + (var_t1 * var_gamma2_dn2))))), (((((var_moc_dn3 * var_dvsat) - (var_moc * var_dvsat_dn3)) / (var_dvsat * var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((var_t8_dn3 * var_gamma1) + (var_t8 * var_gamma1_dn3)) + ((var_t1_dn3 * var_gamma2) + (var_t1 * var_gamma2_dn3))))), (((((var_moc_dn4 * var_dvsat) - (var_moc * var_dvsat_dn4)) / (var_dvsat * var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((var_t8_dn4 * var_gamma1) + (var_t8 * var_gamma1_dn4)) + ((var_t1_dn4 * var_gamma2) + (var_t1 * var_gamma2_dn4))))), (((((var_moc_dn5 * var_dvsat) - (var_moc * var_dvsat_dn5)) / (var_dvsat * var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((var_t8_dn5 * var_gamma1) + (var_t8 * var_gamma1_dn5)) + ((var_t1_dn5 * var_gamma2) + (var_t1 * var_gamma2_dn5))))), (((((var_moc_dn6 * var_dvsat) - (var_moc * var_dvsat_dn6)) / (var_dvsat * var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((var_t8_dn6 * var_gamma1) + (var_t8 * var_gamma1_dn6)) + ((var_t1_dn6 * var_gamma2) + (var_t1 * var_gamma2_dn6))))), (((((var_moc_dn7 * var_dvsat) - (var_moc * var_dvsat_dn7)) / (var_dvsat * var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((var_t8_dn7 * var_gamma1) + (var_t8 * var_gamma1_dn7)) + ((var_t1_dn7 * var_gamma2) + (var_t1 * var_gamma2_dn7))))), (((((var_moc_dn8 * var_dvsat) - (var_moc * var_dvsat_dn8)) / (var_dvsat * var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((var_t8_dn8 * var_gamma1) + (var_t8 * var_gamma1_dn8)) + ((var_t1_dn8 * var_gamma2) + (var_t1 * var_gamma2_dn8))))), (((((var_moc_dn9 * var_dvsat) - (var_moc * var_dvsat_dn9)) / (var_dvsat * var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((var_t8_dn9 * var_gamma1) + (var_t8 * var_gamma1_dn9)) + ((var_t1_dn9 * var_gamma2) + (var_t1 * var_gamma2_dn9))))), (((((var_moc_dn10 * var_dvsat) - (var_moc * var_dvsat_dn10)) / (var_dvsat * var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((var_t8_dn10 * var_gamma1) + (var_t8 * var_gamma1_dn10)) + ((var_t1_dn10 * var_gamma2) + (var_t1 * var_gamma2_dn10))))), (((((var_moc_dn11 * var_dvsat) - (var_moc * var_dvsat_dn11)) / (var_dvsat * var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((var_t8_dn11 * var_gamma1) + (var_t8 * var_gamma1_dn11)) + ((var_t1_dn11 * var_gamma2) + (var_t1 * var_gamma2_dn11))))), (((((var_moc_dn13 * var_dvsat) - (var_moc * var_dvsat_dn13)) / (var_dvsat * var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((var_t8_dn13 * var_gamma1) + (var_t8 * var_gamma1_dn13)) + ((var_t1_dn13 * var_gamma2) + (var_t1 * var_gamma2_dn13))))), (((((var_moc_dn14 * var_dvsat) - (var_moc * var_dvsat_dn14)) / (var_dvsat * var_dvsat)) * assign34110_e56830) + (assign34110_e56823 * (((var_t8_dn14 * var_gamma1) + (var_t8 * var_gamma1_dn14)) + ((var_t1_dn14 * var_gamma2) + (var_t1 * var_gamma2_dn14))))),)
    } else {
        (var_gamma, var_gamma_dn0, var_gamma_dn2, var_gamma_dn3, var_gamma_dn4, var_gamma_dn5, var_gamma_dn6, var_gamma_dn7, var_gamma_dn8, var_gamma_dn9, var_gamma_dn10, var_gamma_dn11, var_gamma_dn13, var_gamma_dn14,)
    }
};
        var_gamma = assign34110_e56833;
        var_gamma_dn0 = assign34110_e56833_d_n0;
        var_gamma_dn2 = assign34110_e56833_d_n2;
        var_gamma_dn3 = assign34110_e56833_d_n3;
        var_gamma_dn4 = assign34110_e56833_d_n4;
        var_gamma_dn5 = assign34110_e56833_d_n5;
        var_gamma_dn6 = assign34110_e56833_d_n6;
        var_gamma_dn7 = assign34110_e56833_d_n7;
        var_gamma_dn8 = assign34110_e56833_d_n8;
        var_gamma_dn9 = assign34110_e56833_d_n9;
        var_gamma_dn10 = assign34110_e56833_d_n10;
        var_gamma_dn11 = assign34110_e56833_d_n11;
        var_gamma_dn13 = assign34110_e56833_d_n13;
        var_gamma_dn14 = assign34110_e56833_d_n14;

        let (assign34130_e56867, assign34130_e56867_d_n0, assign34130_e56867_d_n2, assign34130_e56867_d_n3, assign34130_e56867_d_n4, assign34130_e56867_d_n5, assign34130_e56867_d_n6, assign34130_e56867_d_n7, assign34130_e56867_d_n8, assign34130_e56867_d_n9, assign34130_e56867_d_n10, assign34130_e56867_d_n11, assign34130_e56867_d_n13, assign34130_e56867_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign34130_e56855: f64 = (var_moc / 6.0);
        let assign34130_e56857: f64 = (assign34130_e56855 * var_dvsat3);
        let assign34130_e56859: f64 = (assign34130_e56857 * var_t2);
        let assign34130_e56862: f64 = (var_delta1 - var_delta2);
        let assign34130_e56864: f64 = (assign34130_e56862 + var_delta3);
        let assign34130_e56865: f64 = (assign34130_e56859 * assign34130_e56864);
        (assign34130_e56865, (((((((var_moc_dn0 / 6.0) * var_dvsat3) + (assign34130_e56855 * var_dvsat3_dn0)) * var_t2) + (assign34130_e56857 * var_t2_dn0)) * assign34130_e56864) + (assign34130_e56859 * ((var_delta1_dn0 - var_delta2_dn0) + var_delta3_dn0))), (((((((var_moc_dn2 / 6.0) * var_dvsat3) + (assign34130_e56855 * var_dvsat3_dn2)) * var_t2) + (assign34130_e56857 * var_t2_dn2)) * assign34130_e56864) + (assign34130_e56859 * ((var_delta1_dn2 - var_delta2_dn2) + var_delta3_dn2))), (((((((var_moc_dn3 / 6.0) * var_dvsat3) + (assign34130_e56855 * var_dvsat3_dn3)) * var_t2) + (assign34130_e56857 * var_t2_dn3)) * assign34130_e56864) + (assign34130_e56859 * ((var_delta1_dn3 - var_delta2_dn3) + var_delta3_dn3))), (((((((var_moc_dn4 / 6.0) * var_dvsat3) + (assign34130_e56855 * var_dvsat3_dn4)) * var_t2) + (assign34130_e56857 * var_t2_dn4)) * assign34130_e56864) + (assign34130_e56859 * ((var_delta1_dn4 - var_delta2_dn4) + var_delta3_dn4))), (((((((var_moc_dn5 / 6.0) * var_dvsat3) + (assign34130_e56855 * var_dvsat3_dn5)) * var_t2) + (assign34130_e56857 * var_t2_dn5)) * assign34130_e56864) + (assign34130_e56859 * ((var_delta1_dn5 - var_delta2_dn5) + var_delta3_dn5))), (((((((var_moc_dn6 / 6.0) * var_dvsat3) + (assign34130_e56855 * var_dvsat3_dn6)) * var_t2) + (assign34130_e56857 * var_t2_dn6)) * assign34130_e56864) + (assign34130_e56859 * ((var_delta1_dn6 - var_delta2_dn6) + var_delta3_dn6))), (((((((var_moc_dn7 / 6.0) * var_dvsat3) + (assign34130_e56855 * var_dvsat3_dn7)) * var_t2) + (assign34130_e56857 * var_t2_dn7)) * assign34130_e56864) + (assign34130_e56859 * ((var_delta1_dn7 - var_delta2_dn7) + var_delta3_dn7))), (((((((var_moc_dn8 / 6.0) * var_dvsat3) + (assign34130_e56855 * var_dvsat3_dn8)) * var_t2) + (assign34130_e56857 * var_t2_dn8)) * assign34130_e56864) + (assign34130_e56859 * ((var_delta1_dn8 - var_delta2_dn8) + var_delta3_dn8))), (((((((var_moc_dn9 / 6.0) * var_dvsat3) + (assign34130_e56855 * var_dvsat3_dn9)) * var_t2) + (assign34130_e56857 * var_t2_dn9)) * assign34130_e56864) + (assign34130_e56859 * ((var_delta1_dn9 - var_delta2_dn9) + var_delta3_dn9))), (((((((var_moc_dn10 / 6.0) * var_dvsat3) + (assign34130_e56855 * var_dvsat3_dn10)) * var_t2) + (assign34130_e56857 * var_t2_dn10)) * assign34130_e56864) + (assign34130_e56859 * ((var_delta1_dn10 - var_delta2_dn10) + var_delta3_dn10))), (((((((var_moc_dn11 / 6.0) * var_dvsat3) + (assign34130_e56855 * var_dvsat3_dn11)) * var_t2) + (assign34130_e56857 * var_t2_dn11)) * assign34130_e56864) + (assign34130_e56859 * ((var_delta1_dn11 - var_delta2_dn11) + var_delta3_dn11))), (((((((var_moc_dn13 / 6.0) * var_dvsat3) + (assign34130_e56855 * var_dvsat3_dn13)) * var_t2) + (assign34130_e56857 * var_t2_dn13)) * assign34130_e56864) + (assign34130_e56859 * ((var_delta1_dn13 - var_delta2_dn13) + var_delta3_dn13))), (((((((var_moc_dn14 / 6.0) * var_dvsat3) + (assign34130_e56855 * var_dvsat3_dn14)) * var_t2) + (assign34130_e56857 * var_t2_dn14)) * assign34130_e56864) + (assign34130_e56859 * ((var_delta1_dn14 - var_delta2_dn14) + var_delta3_dn14))),)
    } else {
        (var_delta, var_delta_dn0, var_delta_dn2, var_delta_dn3, var_delta_dn4, var_delta_dn5, var_delta_dn6, var_delta_dn7, var_delta_dn8, var_delta_dn9, var_delta_dn10, var_delta_dn11, var_delta_dn13, var_delta_dn14,)
    }
};
        var_delta = assign34130_e56867;
        var_delta_dn0 = assign34130_e56867_d_n0;
        var_delta_dn2 = assign34130_e56867_d_n2;
        var_delta_dn3 = assign34130_e56867_d_n3;
        var_delta_dn4 = assign34130_e56867_d_n4;
        var_delta_dn5 = assign34130_e56867_d_n5;
        var_delta_dn6 = assign34130_e56867_d_n6;
        var_delta_dn7 = assign34130_e56867_d_n7;
        var_delta_dn8 = assign34130_e56867_d_n8;
        var_delta_dn9 = assign34130_e56867_d_n9;
        var_delta_dn10 = assign34130_e56867_d_n10;
        var_delta_dn11 = assign34130_e56867_d_n11;
        var_delta_dn13 = assign34130_e56867_d_n13;
        var_delta_dn14 = assign34130_e56867_d_n14;

        let (assign34140_e56887, assign34140_e56887_d_n0, assign34140_e56887_d_n2, assign34140_e56887_d_n3, assign34140_e56887_d_n4, assign34140_e56887_d_n5, assign34140_e56887_d_n6, assign34140_e56887_d_n7, assign34140_e56887_d_n8, assign34140_e56887_d_n9, assign34140_e56887_d_n10, assign34140_e56887_d_n11, assign34140_e56887_d_n13, assign34140_e56887_d_n14,) = {
    if ((var_guard633 != 0.0) && (var_guard632 == 0.0)) {
        let assign34140_e56874: f64 = (var_delta / var_gamma);
        let assign34140_e56875: f64 = (assign34140_e56874).sqrt();
        let assign34140_e56877: f64 = (assign34140_e56875 * var_nfintotal);
        let assign34140_e56879: f64 = (assign34140_e56877 * var_coxeff);
        let assign34140_e56881: f64 = (assign34140_e56879 * var_weffcv0);
        let assign34140_e56883: f64 = (assign34140_e56881 * var_leffcv_1);
        let assign34140_e56885: f64 = (assign34140_e56883 / var_noigd0);
        (assign34140_e56885, (((((((((((((var_delta_dn0 * var_gamma) - (var_delta * var_gamma_dn0)) / (var_gamma * var_gamma)) / (2.0 * assign34140_e56875)) * var_nfintotal) * var_coxeff) + (assign34140_e56877 * var_coxeff_dn0)) * var_weffcv0) * var_leffcv_1) + (assign34140_e56881 * var_leffcv_1_dn0)) * var_noigd0) - (assign34140_e56883 * var_noigd0_dn0)) / (var_noigd0 * var_noigd0)), (((((((((((((var_delta_dn2 * var_gamma) - (var_delta * var_gamma_dn2)) / (var_gamma * var_gamma)) / (2.0 * assign34140_e56875)) * var_nfintotal) * var_coxeff) + (assign34140_e56877 * var_coxeff_dn2)) * var_weffcv0) * var_leffcv_1) + (assign34140_e56881 * var_leffcv_1_dn2)) * var_noigd0) - (assign34140_e56883 * var_noigd0_dn2)) / (var_noigd0 * var_noigd0)), (((((((((((((var_delta_dn3 * var_gamma) - (var_delta * var_gamma_dn3)) / (var_gamma * var_gamma)) / (2.0 * assign34140_e56875)) * var_nfintotal) * var_coxeff) + (assign34140_e56877 * var_coxeff_dn3)) * var_weffcv0) * var_leffcv_1) + (assign34140_e56881 * var_leffcv_1_dn3)) * var_noigd0) - (assign34140_e56883 * var_noigd0_dn3)) / (var_noigd0 * var_noigd0)), (((((((((((((var_delta_dn4 * var_gamma) - (var_delta * var_gamma_dn4)) / (var_gamma * var_gamma)) / (2.0 * assign34140_e56875)) * var_nfintotal) * var_coxeff) + (assign34140_e56877 * var_coxeff_dn4)) * var_weffcv0) * var_leffcv_1) + (assign34140_e56881 * var_leffcv_1_dn4)) * var_noigd0) - (assign34140_e56883 * var_noigd0_dn4)) / (var_noigd0 * var_noigd0)), (((((((((((((var_delta_dn5 * var_gamma) - (var_delta * var_gamma_dn5)) / (var_gamma * var_gamma)) / (2.0 * assign34140_e56875)) * var_nfintotal) * var_coxeff) + (assign34140_e56877 * var_coxeff_dn5)) * var_weffcv0) * var_leffcv_1) + (assign34140_e56881 * var_leffcv_1_dn5)) * var_noigd0) - (assign34140_e56883 * var_noigd0_dn5)) / (var_noigd0 * var_noigd0)), (((((((((((((var_delta_dn6 * var_gamma) - (var_delta * var_gamma_dn6)) / (var_gamma * var_gamma)) / (2.0 * assign34140_e56875)) * var_nfintotal) * var_coxeff) + (assign34140_e56877 * var_coxeff_dn6)) * var_weffcv0) * var_leffcv_1) + (assign34140_e56881 * var_leffcv_1_dn6)) * var_noigd0) - (assign34140_e56883 * var_noigd0_dn6)) / (var_noigd0 * var_noigd0)), (((((((((((((var_delta_dn7 * var_gamma) - (var_delta * var_gamma_dn7)) / (var_gamma * var_gamma)) / (2.0 * assign34140_e56875)) * var_nfintotal) * var_coxeff) + (assign34140_e56877 * var_coxeff_dn7)) * var_weffcv0) * var_leffcv_1) + (assign34140_e56881 * var_leffcv_1_dn7)) * var_noigd0) - (assign34140_e56883 * var_noigd0_dn7)) / (var_noigd0 * var_noigd0)), (((((((((((((var_delta_dn8 * var_gamma) - (var_delta * var_gamma_dn8)) / (var_gamma * var_gamma)) / (2.0 * assign34140_e56875)) * var_nfintotal) * var_coxeff) + (assign34140_e56877 * var_coxeff_dn8)) * var_weffcv0) * var_leffcv_1) + (assign34140_e56881 * var_leffcv_1_dn8)) * var_noigd0) - (assign34140_e56883 * var_noigd0_dn8)) / (var_noigd0 * var_noigd0)), (((((((((((((var_delta_dn9 * var_gamma) - (var_delta * var_gamma_dn9)) / (var_gamma * var_gamma)) / (2.0 * assign34140_e56875)) * var_nfintotal) * var_coxeff) + (assign34140_e56877 * var_coxeff_dn9)) * var_weffcv0) * var_leffcv_1) + (assign34140_e56881 * var_leffcv_1_dn9)) * var_noigd0) - (assign34140_e56883 * var_noigd0_dn9)) / (var_noigd0 * var_noigd0)), (((((((((((((var_delta_dn10 * var_gamma) - (var_delta * var_gamma_dn10)) / (var_gamma * var_gamma)) / (2.0 * assign34140_e56875)) * var_nfintotal) * var_coxeff) + (assign34140_e56877 * var_coxeff_dn10)) * var_weffcv0) * var_leffcv_1) + (assign34140_e56881 * var_leffcv_1_dn10)) * var_noigd0) - (assign34140_e56883 * var_noigd0_dn10)) / (var_noigd0 * var_noigd0)), (((((((((((((var_delta_dn11 * var_gamma) - (var_delta * var_gamma_dn11)) / (var_gamma * var_gamma)) / (2.0 * assign34140_e56875)) * var_nfintotal) * var_coxeff) + (assign34140_e56877 * var_coxeff_dn11)) * var_weffcv0) * var_leffcv_1) + (assign34140_e56881 * var_leffcv_1_dn11)) * var_noigd0) - (assign34140_e56883 * var_noigd0_dn11)) / (var_noigd0 * var_noigd0)), (((((((((((((var_delta_dn13 * var_gamma) - (var_delta * var_gamma_dn13)) / (var_gamma * var_gamma)) / (2.0 * assign34140_e56875)) * var_nfintotal) * var_coxeff) + (assign34140_e56877 * var_coxeff_dn13)) * var_weffcv0) * var_leffcv_1) + (assign34140_e56881 * var_leffcv_1_dn13)) * var_noigd0) - (assign34140_e56883 * var_noigd0_dn13)) / (var_noigd0 * var_noigd0)), (((((((((((((var_delta_dn14 * var_gamma) - (var_delta * var_gamma_dn14)) / (var_gamma * var_gamma)) / (2.0 * assign34140_e56875)) * var_nfintotal) * var_coxeff) + (assign34140_e56877 * var_coxeff_dn14)) * var_weffcv0) * var_leffcv_1) + (assign34140_e56881 * var_leffcv_1_dn14)) * var_noigd0) - (assign34140_e56883 * var_noigd0_dn14)) / (var_noigd0 * var_noigd0)),)
    } else {
        (var_sigrat, var_sigrat_dn0, var_sigrat_dn2, var_sigrat_dn3, var_sigrat_dn4, var_sigrat_dn5, var_sigrat_dn6, var_sigrat_dn7, var_sigrat_dn8, var_sigrat_dn9, var_sigrat_dn10, var_sigrat_dn11, var_sigrat_dn13, var_sigrat_dn14,)
    }
};
        var_sigrat = assign34140_e56887;
        var_sigrat_dn0 = assign34140_e56887_d_n0;
        var_sigrat_dn2 = assign34140_e56887_d_n2;
        var_sigrat_dn3 = assign34140_e56887_d_n3;
        var_sigrat_dn4 = assign34140_e56887_d_n4;
        var_sigrat_dn5 = assign34140_e56887_d_n5;
        var_sigrat_dn6 = assign34140_e56887_d_n6;
        var_sigrat_dn7 = assign34140_e56887_d_n7;
        var_sigrat_dn8 = assign34140_e56887_d_n8;
        var_sigrat_dn9 = assign34140_e56887_d_n9;
        var_sigrat_dn10 = assign34140_e56887_d_n10;
        var_sigrat_dn11 = assign34140_e56887_d_n11;
        var_sigrat_dn13 = assign34140_e56887_d_n13;
        var_sigrat_dn14 = assign34140_e56887_d_n14;

        let assign34150_e56890: f64 = if var_sigvds > 0.0 { 1.0 } else { 0.0 };
        var_guard641 = assign34150_e56890;

        let assign34160_e56893: f64 = if p.p73 == 2.0 { 1.0 } else { 0.0 };
        var_guard642 = assign34160_e56893;

        let assign34180_e56899: f64 = if var_sigvds > 0.0 { 1.0 } else { 0.0 };
        var_guard644 = assign34180_e56899;

        let assign34190_e56902: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard645 = assign34190_e56902;

        let assign34200_e56917: f64 = if ((p.p70 == 2.0) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0))) { 1.0 } else { 0.0 };
        var_guard646 = assign34200_e56917;

        let assign34210_e56920: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard647 = assign34210_e56920;

        let assign34220_e56935: f64 = if ((p.p70 == 2.0) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0))) { 1.0 } else { 0.0 };
        var_guard648 = assign34220_e56935;

        let assign34230_e56938: f64 = if p.p61 == 0.0 { 1.0 } else { 0.0 };
        var_guard649 = assign34230_e56938;

        let assign34240_e56941: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard650 = assign34240_e56941;

        let assign34250_e56944: f64 = if p.p76 != 2.0 { 1.0 } else { 0.0 };
        var_guard651 = assign34250_e56944;

        let assign34260_e56947: f64 = if p.p65 == 1.0 { 1.0 } else { 0.0 };
        var_guard652 = assign34260_e56947;

        let assign34270_e56950: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };
        var_guard653 = assign34270_e56950;

        let assign34280_e56953: f64 = if p.p65 == 1.0 { 1.0 } else { 0.0 };
        var_guard654 = assign34280_e56953;

        let assign34290_e56956: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };
        var_guard655 = assign34290_e56956;

        let assign34300_e56959: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard656 = assign34300_e56959;

        let assign34310_e56962: f64 = if p.p64 == 1.0 { 1.0 } else { 0.0 };
        var_guard657 = assign34310_e56962;

        let assign34320_e56965: f64 = if p.p1910 > 0.0 { 1.0 } else { 0.0 };
        var_guard658 = assign34320_e56965;

        *var_ctnoi_slot = var_ctnoi;
        *var_ctnoi_dn0_slot = var_ctnoi_dn0;
        *var_ctnoi_dn10_slot = var_ctnoi_dn10;
        *var_ctnoi_dn11_slot = var_ctnoi_dn11;
        *var_ctnoi_dn13_slot = var_ctnoi_dn13;
        *var_ctnoi_dn14_slot = var_ctnoi_dn14;
        *var_ctnoi_dn2_slot = var_ctnoi_dn2;
        *var_ctnoi_dn3_slot = var_ctnoi_dn3;
        *var_ctnoi_dn4_slot = var_ctnoi_dn4;
        *var_ctnoi_dn5_slot = var_ctnoi_dn5;
        *var_ctnoi_dn6_slot = var_ctnoi_dn6;
        *var_ctnoi_dn7_slot = var_ctnoi_dn7;
        *var_ctnoi_dn8_slot = var_ctnoi_dn8;
        *var_ctnoi_dn9_slot = var_ctnoi_dn9;
        *var_delta_slot = var_delta;
        *var_delta1_slot = var_delta1;
        *var_delta1_dn0_slot = var_delta1_dn0;
        *var_delta1_dn10_slot = var_delta1_dn10;
        *var_delta1_dn11_slot = var_delta1_dn11;
        *var_delta1_dn13_slot = var_delta1_dn13;
        *var_delta1_dn14_slot = var_delta1_dn14;
        *var_delta1_dn2_slot = var_delta1_dn2;
        *var_delta1_dn3_slot = var_delta1_dn3;
        *var_delta1_dn4_slot = var_delta1_dn4;
        *var_delta1_dn5_slot = var_delta1_dn5;
        *var_delta1_dn6_slot = var_delta1_dn6;
        *var_delta1_dn7_slot = var_delta1_dn7;
        *var_delta1_dn8_slot = var_delta1_dn8;
        *var_delta1_dn9_slot = var_delta1_dn9;
        *var_delta2_slot = var_delta2;
        *var_delta2_dn0_slot = var_delta2_dn0;
        *var_delta2_dn10_slot = var_delta2_dn10;
        *var_delta2_dn11_slot = var_delta2_dn11;
        *var_delta2_dn13_slot = var_delta2_dn13;
        *var_delta2_dn14_slot = var_delta2_dn14;
        *var_delta2_dn2_slot = var_delta2_dn2;
        *var_delta2_dn3_slot = var_delta2_dn3;
        *var_delta2_dn4_slot = var_delta2_dn4;
        *var_delta2_dn5_slot = var_delta2_dn5;
        *var_delta2_dn6_slot = var_delta2_dn6;
        *var_delta2_dn7_slot = var_delta2_dn7;
        *var_delta2_dn8_slot = var_delta2_dn8;
        *var_delta2_dn9_slot = var_delta2_dn9;
        *var_delta3_slot = var_delta3;
        *var_delta3_dn0_slot = var_delta3_dn0;
        *var_delta3_dn10_slot = var_delta3_dn10;
        *var_delta3_dn11_slot = var_delta3_dn11;
        *var_delta3_dn13_slot = var_delta3_dn13;
        *var_delta3_dn14_slot = var_delta3_dn14;
        *var_delta3_dn2_slot = var_delta3_dn2;
        *var_delta3_dn3_slot = var_delta3_dn3;
        *var_delta3_dn4_slot = var_delta3_dn4;
        *var_delta3_dn5_slot = var_delta3_dn5;
        *var_delta3_dn6_slot = var_delta3_dn6;
        *var_delta3_dn7_slot = var_delta3_dn7;
        *var_delta3_dn8_slot = var_delta3_dn8;
        *var_delta3_dn9_slot = var_delta3_dn9;
        *var_delta_dn0_slot = var_delta_dn0;
        *var_delta_dn10_slot = var_delta_dn10;
        *var_delta_dn11_slot = var_delta_dn11;
        *var_delta_dn13_slot = var_delta_dn13;
        *var_delta_dn14_slot = var_delta_dn14;
        *var_delta_dn2_slot = var_delta_dn2;
        *var_delta_dn3_slot = var_delta_dn3;
        *var_delta_dn4_slot = var_delta_dn4;
        *var_delta_dn5_slot = var_delta_dn5;
        *var_delta_dn6_slot = var_delta_dn6;
        *var_delta_dn7_slot = var_delta_dn7;
        *var_delta_dn8_slot = var_delta_dn8;
        *var_delta_dn9_slot = var_delta_dn9;
        *var_epsilon_slot = var_epsilon;
        *var_epsilon1_slot = var_epsilon1;
        *var_epsilon1_dn0_slot = var_epsilon1_dn0;
        *var_epsilon1_dn10_slot = var_epsilon1_dn10;
        *var_epsilon1_dn11_slot = var_epsilon1_dn11;
        *var_epsilon1_dn13_slot = var_epsilon1_dn13;
        *var_epsilon1_dn14_slot = var_epsilon1_dn14;
        *var_epsilon1_dn2_slot = var_epsilon1_dn2;
        *var_epsilon1_dn3_slot = var_epsilon1_dn3;
        *var_epsilon1_dn4_slot = var_epsilon1_dn4;
        *var_epsilon1_dn5_slot = var_epsilon1_dn5;
        *var_epsilon1_dn6_slot = var_epsilon1_dn6;
        *var_epsilon1_dn7_slot = var_epsilon1_dn7;
        *var_epsilon1_dn8_slot = var_epsilon1_dn8;
        *var_epsilon1_dn9_slot = var_epsilon1_dn9;
        *var_epsilon2_slot = var_epsilon2;
        *var_epsilon2_dn0_slot = var_epsilon2_dn0;
        *var_epsilon2_dn10_slot = var_epsilon2_dn10;
        *var_epsilon2_dn11_slot = var_epsilon2_dn11;
        *var_epsilon2_dn13_slot = var_epsilon2_dn13;
        *var_epsilon2_dn14_slot = var_epsilon2_dn14;
        *var_epsilon2_dn2_slot = var_epsilon2_dn2;
        *var_epsilon2_dn3_slot = var_epsilon2_dn3;
        *var_epsilon2_dn4_slot = var_epsilon2_dn4;
        *var_epsilon2_dn5_slot = var_epsilon2_dn5;
        *var_epsilon2_dn6_slot = var_epsilon2_dn6;
        *var_epsilon2_dn7_slot = var_epsilon2_dn7;
        *var_epsilon2_dn8_slot = var_epsilon2_dn8;
        *var_epsilon2_dn9_slot = var_epsilon2_dn9;
        *var_epsilon_dn0_slot = var_epsilon_dn0;
        *var_epsilon_dn10_slot = var_epsilon_dn10;
        *var_epsilon_dn11_slot = var_epsilon_dn11;
        *var_epsilon_dn13_slot = var_epsilon_dn13;
        *var_epsilon_dn14_slot = var_epsilon_dn14;
        *var_epsilon_dn2_slot = var_epsilon_dn2;
        *var_epsilon_dn3_slot = var_epsilon_dn3;
        *var_epsilon_dn4_slot = var_epsilon_dn4;
        *var_epsilon_dn5_slot = var_epsilon_dn5;
        *var_epsilon_dn6_slot = var_epsilon_dn6;
        *var_epsilon_dn7_slot = var_epsilon_dn7;
        *var_epsilon_dn8_slot = var_epsilon_dn8;
        *var_epsilon_dn9_slot = var_epsilon_dn9;
        *var_gamma_slot = var_gamma;
        *var_gamma1_slot = var_gamma1;
        *var_gamma1_dn0_slot = var_gamma1_dn0;
        *var_gamma1_dn10_slot = var_gamma1_dn10;
        *var_gamma1_dn11_slot = var_gamma1_dn11;
        *var_gamma1_dn13_slot = var_gamma1_dn13;
        *var_gamma1_dn14_slot = var_gamma1_dn14;
        *var_gamma1_dn2_slot = var_gamma1_dn2;
        *var_gamma1_dn3_slot = var_gamma1_dn3;
        *var_gamma1_dn4_slot = var_gamma1_dn4;
        *var_gamma1_dn5_slot = var_gamma1_dn5;
        *var_gamma1_dn6_slot = var_gamma1_dn6;
        *var_gamma1_dn7_slot = var_gamma1_dn7;
        *var_gamma1_dn8_slot = var_gamma1_dn8;
        *var_gamma1_dn9_slot = var_gamma1_dn9;
        *var_gamma2_slot = var_gamma2;
        *var_gamma2_dn0_slot = var_gamma2_dn0;
        *var_gamma2_dn10_slot = var_gamma2_dn10;
        *var_gamma2_dn11_slot = var_gamma2_dn11;
        *var_gamma2_dn13_slot = var_gamma2_dn13;
        *var_gamma2_dn14_slot = var_gamma2_dn14;
        *var_gamma2_dn2_slot = var_gamma2_dn2;
        *var_gamma2_dn3_slot = var_gamma2_dn3;
        *var_gamma2_dn4_slot = var_gamma2_dn4;
        *var_gamma2_dn5_slot = var_gamma2_dn5;
        *var_gamma2_dn6_slot = var_gamma2_dn6;
        *var_gamma2_dn7_slot = var_gamma2_dn7;
        *var_gamma2_dn8_slot = var_gamma2_dn8;
        *var_gamma2_dn9_slot = var_gamma2_dn9;
        *var_gamma_dn0_slot = var_gamma_dn0;
        *var_gamma_dn10_slot = var_gamma_dn10;
        *var_gamma_dn11_slot = var_gamma_dn11;
        *var_gamma_dn13_slot = var_gamma_dn13;
        *var_gamma_dn14_slot = var_gamma_dn14;
        *var_gamma_dn2_slot = var_gamma_dn2;
        *var_gamma_dn3_slot = var_gamma_dn3;
        *var_gamma_dn4_slot = var_gamma_dn4;
        *var_gamma_dn5_slot = var_gamma_dn5;
        *var_gamma_dn6_slot = var_gamma_dn6;
        *var_gamma_dn7_slot = var_gamma_dn7;
        *var_gamma_dn8_slot = var_gamma_dn8;
        *var_gamma_dn9_slot = var_gamma_dn9;
        *var_guard639_slot = var_guard639;
        *var_guard640_slot = var_guard640;
        *var_guard641_slot = var_guard641;
        *var_guard642_slot = var_guard642;
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
        *var_guard654_slot = var_guard654;
        *var_guard655_slot = var_guard655;
        *var_guard656_slot = var_guard656;
        *var_guard657_slot = var_guard657;
        *var_guard658_slot = var_guard658;
        *var_sigrat_slot = var_sigrat;
        *var_sigrat_dn0_slot = var_sigrat_dn0;
        *var_sigrat_dn10_slot = var_sigrat_dn10;
        *var_sigrat_dn11_slot = var_sigrat_dn11;
        *var_sigrat_dn13_slot = var_sigrat_dn13;
        *var_sigrat_dn14_slot = var_sigrat_dn14;
        *var_sigrat_dn2_slot = var_sigrat_dn2;
        *var_sigrat_dn3_slot = var_sigrat_dn3;
        *var_sigrat_dn4_slot = var_sigrat_dn4;
        *var_sigrat_dn5_slot = var_sigrat_dn5;
        *var_sigrat_dn6_slot = var_sigrat_dn6;
        *var_sigrat_dn7_slot = var_sigrat_dn7;
        *var_sigrat_dn8_slot = var_sigrat_dn8;
        *var_sigrat_dn9_slot = var_sigrat_dn9;
        *var_t8_slot = var_t8;
        *var_t8_dn0_slot = var_t8_dn0;
        *var_t8_dn10_slot = var_t8_dn10;
        *var_t8_dn11_slot = var_t8_dn11;
        *var_t8_dn13_slot = var_t8_dn13;
        *var_t8_dn14_slot = var_t8_dn14;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn3_slot = var_t8_dn3;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_dn9_slot = var_t8_dn9;
    }

    pub(super) fn stamp_transient_block_132(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_guard657: f64,
        var_guard658: f64,
        var_nfintotal: f64,
        var_qis: f64,
        var_qis_dn0: f64,
        var_qis_dn10: f64,
        var_qis_dn11: f64,
        var_qis_dn13: f64,
        var_qis_dn14: f64,
        var_qis_dn2: f64,
        var_qis_dn3: f64,
        var_qis_dn4: f64,
        var_qis_dn5: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_qis_dn9: f64,
        var_weff0: f64,
        var_weffwrfactor: f64,
        var_guard659_slot: &mut f64,
        var_guard660_slot: &mut f64,
        var_isat_rd_slot: &mut f64,
        var_isat_rd_dn0_slot: &mut f64,
        var_isat_rd_dn10_slot: &mut f64,
        var_isat_rd_dn11_slot: &mut f64,
        var_isat_rd_dn13_slot: &mut f64,
        var_isat_rd_dn14_slot: &mut f64,
        var_isat_rd_dn2_slot: &mut f64,
        var_isat_rd_dn3_slot: &mut f64,
        var_isat_rd_dn4_slot: &mut f64,
        var_isat_rd_dn5_slot: &mut f64,
        var_isat_rd_dn6_slot: &mut f64,
        var_isat_rd_dn7_slot: &mut f64,
        var_isat_rd_dn8_slot: &mut f64,
        var_isat_rd_dn9_slot: &mut f64,
        var_rdstempvs_slot: &mut f64,
        var_rdstempvs_dn4_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
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
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
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
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
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
        var_t3_dn13_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
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
        var_t4_dn13_slot: &mut f64,
        var_t4_dn14_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
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
        var_t5_dn13_slot: &mut f64,
        var_t5_dn14_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_dn9_slot: &mut f64,
        var_vsatrsd_eff_slot: &mut f64,
        var_vsatrsd_eff_dn0_slot: &mut f64,
        var_vsatrsd_eff_dn10_slot: &mut f64,
        var_vsatrsd_eff_dn11_slot: &mut f64,
        var_vsatrsd_eff_dn13_slot: &mut f64,
        var_vsatrsd_eff_dn14_slot: &mut f64,
        var_vsatrsd_eff_dn2_slot: &mut f64,
        var_vsatrsd_eff_dn3_slot: &mut f64,
        var_vsatrsd_eff_dn4_slot: &mut f64,
        var_vsatrsd_eff_dn5_slot: &mut f64,
        var_vsatrsd_eff_dn6_slot: &mut f64,
        var_vsatrsd_eff_dn7_slot: &mut f64,
        var_vsatrsd_eff_dn8_slot: &mut f64,
        var_vsatrsd_eff_dn9_slot: &mut f64,
        var_vsatrsd_t_slot: &mut f64,
        var_vsatrsd_t_dn4_slot: &mut f64,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let mut var_guard659: f64 = *var_guard659_slot;
        let mut var_guard660: f64 = *var_guard660_slot;
        let mut var_isat_rd: f64 = *var_isat_rd_slot;
        let mut var_isat_rd_dn0: f64 = *var_isat_rd_dn0_slot;
        let mut var_isat_rd_dn10: f64 = *var_isat_rd_dn10_slot;
        let mut var_isat_rd_dn11: f64 = *var_isat_rd_dn11_slot;
        let mut var_isat_rd_dn13: f64 = *var_isat_rd_dn13_slot;
        let mut var_isat_rd_dn14: f64 = *var_isat_rd_dn14_slot;
        let mut var_isat_rd_dn2: f64 = *var_isat_rd_dn2_slot;
        let mut var_isat_rd_dn3: f64 = *var_isat_rd_dn3_slot;
        let mut var_isat_rd_dn4: f64 = *var_isat_rd_dn4_slot;
        let mut var_isat_rd_dn5: f64 = *var_isat_rd_dn5_slot;
        let mut var_isat_rd_dn6: f64 = *var_isat_rd_dn6_slot;
        let mut var_isat_rd_dn7: f64 = *var_isat_rd_dn7_slot;
        let mut var_isat_rd_dn8: f64 = *var_isat_rd_dn8_slot;
        let mut var_isat_rd_dn9: f64 = *var_isat_rd_dn9_slot;
        let mut var_rdstempvs: f64 = *var_rdstempvs_slot;
        let mut var_rdstempvs_dn4: f64 = *var_rdstempvs_dn4_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
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
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
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
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
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
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
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
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn14: f64 = *var_t4_dn14_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
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
        let mut var_t5_dn13: f64 = *var_t5_dn13_slot;
        let mut var_t5_dn14: f64 = *var_t5_dn14_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_dn9: f64 = *var_t5_dn9_slot;
        let mut var_vsatrsd_eff: f64 = *var_vsatrsd_eff_slot;
        let mut var_vsatrsd_eff_dn0: f64 = *var_vsatrsd_eff_dn0_slot;
        let mut var_vsatrsd_eff_dn10: f64 = *var_vsatrsd_eff_dn10_slot;
        let mut var_vsatrsd_eff_dn11: f64 = *var_vsatrsd_eff_dn11_slot;
        let mut var_vsatrsd_eff_dn13: f64 = *var_vsatrsd_eff_dn13_slot;
        let mut var_vsatrsd_eff_dn14: f64 = *var_vsatrsd_eff_dn14_slot;
        let mut var_vsatrsd_eff_dn2: f64 = *var_vsatrsd_eff_dn2_slot;
        let mut var_vsatrsd_eff_dn3: f64 = *var_vsatrsd_eff_dn3_slot;
        let mut var_vsatrsd_eff_dn4: f64 = *var_vsatrsd_eff_dn4_slot;
        let mut var_vsatrsd_eff_dn5: f64 = *var_vsatrsd_eff_dn5_slot;
        let mut var_vsatrsd_eff_dn6: f64 = *var_vsatrsd_eff_dn6_slot;
        let mut var_vsatrsd_eff_dn7: f64 = *var_vsatrsd_eff_dn7_slot;
        let mut var_vsatrsd_eff_dn8: f64 = *var_vsatrsd_eff_dn8_slot;
        let mut var_vsatrsd_eff_dn9: f64 = *var_vsatrsd_eff_dn9_slot;
        let mut var_vsatrsd_t: f64 = *var_vsatrsd_t_slot;
        let mut var_vsatrsd_t_dn4: f64 = *var_vsatrsd_t_dn4_slot;

        let (assign34330_e57042, assign34330_e57042_d_n4,) = {
    if ((var_guard657 != 0.0) && (var_guard658 != 0.0)) {
        let assign34330_e56972: f64 = (p.p1912 * var_deltemp);
        let assign34330_e56973: f64 = (1.0 + assign34330_e56972);
        let assign34330_e56975: f64 = (assign34330_e56973 - 1e-6);
        let assign34330_e56977: f64 = (-10000.0);
        let assign34330_e56979: f64 = (assign34330_e56977 * 0.001);
        let (assign34330_e57040, assign34330_e57040_d_n4,) = {
            if (!(assign34330_e56975 < assign34330_e56979)) {
                let assign34330_e56986: f64 = (p.p1912 * var_deltemp);
                let assign34330_e56987: f64 = (1.0 + assign34330_e56986);
                let assign34330_e56989: f64 = (assign34330_e56987 - 1e-6);
                let assign34330_e56993: f64 = (p.p1912 * var_deltemp);
                let assign34330_e56994: f64 = (1.0 + assign34330_e56993);
                let assign34330_e56996: f64 = (assign34330_e56994 - 1e-6);
                let assign34330_e57000: f64 = (p.p1912 * var_deltemp);
                let assign34330_e57001: f64 = (1.0 + assign34330_e57000);
                let assign34330_e57003: f64 = (assign34330_e57001 - 1e-6);
                let assign34330_e57004: f64 = (assign34330_e56996 * assign34330_e57003);
                let assign34330_e57007: f64 = (4.0 * 0.001);
                let assign34330_e57009: f64 = (assign34330_e57007 * 0.001);
                let assign34330_e57010: f64 = (assign34330_e57004 + assign34330_e57009);
                let assign34330_e57011: f64 = (assign34330_e57010).sqrt();
                let assign34330_e57012: f64 = (assign34330_e56989 + assign34330_e57011);
                let assign34330_e57013: f64 = (0.5 * assign34330_e57012);
                (assign34330_e57013, (0.5 * ((p.p1912 * var_deltemp_dn4) + ((((p.p1912 * var_deltemp_dn4) * assign34330_e57003) + (assign34330_e56996 * (p.p1912 * var_deltemp_dn4))) / (2.0 * assign34330_e57011)))),)
            } else {
                let assign34330_e57017: f64 = (p.p1912 * var_deltemp);
                let assign34330_e57018: f64 = (1.0 + assign34330_e57017);
                let assign34330_e57020: f64 = (assign34330_e57018 - 1e-6);
                let assign34330_e57022: f64 = (-10000.0);
                let assign34330_e57024: f64 = (assign34330_e57022 * 0.001);
                let (assign34330_e57039, assign34330_e57039_d_n4,) = {
                    if (assign34330_e57020 < assign34330_e57024) {
                        let assign34330_e57027: f64 = (-0.001);
                        let assign34330_e57029: f64 = (assign34330_e57027 * 0.001);
                        let assign34330_e57033: f64 = (p.p1912 * var_deltemp);
                        let assign34330_e57034: f64 = (1.0 + assign34330_e57033);
                        let assign34330_e57036: f64 = (assign34330_e57034 - 1e-6);
                        let assign34330_e57037: f64 = (assign34330_e57029 / assign34330_e57036);
                        (assign34330_e57037, (-((assign34330_e57029 * (p.p1912 * var_deltemp_dn4)) / (assign34330_e57036 * assign34330_e57036))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign34330_e57039, assign34330_e57039_d_n4,)
            }
        };
        (assign34330_e57040, assign34330_e57040_d_n4,)
    } else {
        (var_rdstempvs, var_rdstempvs_dn4,)
    }
};
        var_rdstempvs = assign34330_e57042;
        var_rdstempvs_dn4 = assign34330_e57042_d_n4;

        let assign34340_e57045: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard659 = assign34340_e57045;

        let (assign34350_e57096, assign34350_e57096_d_n4,) = {
    if (((var_guard657 != 0.0) && (var_guard658 != 0.0)) && (var_guard659 != 0.0)) {
        let assign34350_e57053: f64 = (-p.p1904);
        let assign34350_e57056: f64 = (-p.p1913);
        let assign34350_e57058: f64 = (assign34350_e57056 * var_deltemp);
        let assign34350_e57060: f64 = (-p.p1904);
        let assign34350_e57061: f64 = (assign34350_e57058 - assign34350_e57060);
        let assign34350_e57063: f64 = (assign34350_e57061 - 1e-6);
        let assign34350_e57065: f64 = (-p.p1913);
        let assign34350_e57067: f64 = (assign34350_e57065 * var_deltemp);
        let assign34350_e57069: f64 = (-p.p1904);
        let assign34350_e57070: f64 = (assign34350_e57067 - assign34350_e57069);
        let assign34350_e57072: f64 = (assign34350_e57070 - 1e-6);
        let assign34350_e57074: f64 = (-p.p1913);
        let assign34350_e57076: f64 = (assign34350_e57074 * var_deltemp);
        let assign34350_e57078: f64 = (-p.p1904);
        let assign34350_e57079: f64 = (assign34350_e57076 - assign34350_e57078);
        let assign34350_e57081: f64 = (assign34350_e57079 - 1e-6);
        let assign34350_e57082: f64 = (assign34350_e57072 * assign34350_e57081);
        let assign34350_e57085: f64 = (-p.p1904);
        let assign34350_e57086: f64 = (4.0 * assign34350_e57085);
        let assign34350_e57088: f64 = (assign34350_e57086 * 1e-6);
        let assign34350_e57089: f64 = (assign34350_e57082 - assign34350_e57088);
        let assign34350_e57090: f64 = (assign34350_e57089).sqrt();
        let assign34350_e57091: f64 = (assign34350_e57063 + assign34350_e57090);
        let assign34350_e57092: f64 = (0.5 * assign34350_e57091);
        let assign34350_e57093: f64 = (assign34350_e57053 + assign34350_e57092);
        let assign34350_e57094: f64 = (p.p1904 + assign34350_e57093);
        (assign34350_e57094, (0.5 * ((assign34350_e57056 * var_deltemp_dn4) + ((((assign34350_e57065 * var_deltemp_dn4) * assign34350_e57081) + (assign34350_e57072 * (assign34350_e57074 * var_deltemp_dn4))) / (2.0 * assign34350_e57090)))),)
    } else {
        (var_vsatrsd_t, var_vsatrsd_t_dn4,)
    }
};
        var_vsatrsd_t = assign34350_e57096;
        var_vsatrsd_t_dn4 = assign34350_e57096_d_n4;

        let (assign34360_e57184, assign34360_e57184_d_n4,) = {
    if (((var_guard657 != 0.0) && (var_guard658 != 0.0)) && (var_guard659 == 0.0)) {
        let assign34360_e57106: f64 = (-p.p1913);
        let assign34360_e57108: f64 = (assign34360_e57106 * var_deltemp);
        let assign34360_e57109: f64 = (1.0 + assign34360_e57108);
        let assign34360_e57111: f64 = (assign34360_e57109 - 1e-6);
        let assign34360_e57113: f64 = (-10000.0);
        let assign34360_e57115: f64 = (assign34360_e57113 * 0.001);
        let (assign34360_e57181, assign34360_e57181_d_n4,) = {
            if (!(assign34360_e57111 < assign34360_e57115)) {
                let assign34360_e57121: f64 = (-p.p1913);
                let assign34360_e57123: f64 = (assign34360_e57121 * var_deltemp);
                let assign34360_e57124: f64 = (1.0 + assign34360_e57123);
                let assign34360_e57126: f64 = (assign34360_e57124 - 1e-6);
                let assign34360_e57129: f64 = (-p.p1913);
                let assign34360_e57131: f64 = (assign34360_e57129 * var_deltemp);
                let assign34360_e57132: f64 = (1.0 + assign34360_e57131);
                let assign34360_e57134: f64 = (assign34360_e57132 - 1e-6);
                let assign34360_e57137: f64 = (-p.p1913);
                let assign34360_e57139: f64 = (assign34360_e57137 * var_deltemp);
                let assign34360_e57140: f64 = (1.0 + assign34360_e57139);
                let assign34360_e57142: f64 = (assign34360_e57140 - 1e-6);
                let assign34360_e57143: f64 = (assign34360_e57134 * assign34360_e57142);
                let assign34360_e57146: f64 = (4.0 * 0.001);
                let assign34360_e57148: f64 = (assign34360_e57146 * 0.001);
                let assign34360_e57149: f64 = (assign34360_e57143 + assign34360_e57148);
                let assign34360_e57150: f64 = (assign34360_e57149).sqrt();
                let assign34360_e57151: f64 = (assign34360_e57126 + assign34360_e57150);
                let assign34360_e57152: f64 = (0.5 * assign34360_e57151);
                (assign34360_e57152, (0.5 * ((assign34360_e57121 * var_deltemp_dn4) + ((((assign34360_e57129 * var_deltemp_dn4) * assign34360_e57142) + (assign34360_e57134 * (assign34360_e57137 * var_deltemp_dn4))) / (2.0 * assign34360_e57150)))),)
            } else {
                let assign34360_e57155: f64 = (-p.p1913);
                let assign34360_e57157: f64 = (assign34360_e57155 * var_deltemp);
                let assign34360_e57158: f64 = (1.0 + assign34360_e57157);
                let assign34360_e57160: f64 = (assign34360_e57158 - 1e-6);
                let assign34360_e57162: f64 = (-10000.0);
                let assign34360_e57164: f64 = (assign34360_e57162 * 0.001);
                let (assign34360_e57180, assign34360_e57180_d_n4,) = {
                    if (assign34360_e57160 < assign34360_e57164) {
                        let assign34360_e57167: f64 = (-0.001);
                        let assign34360_e57169: f64 = (assign34360_e57167 * 0.001);
                        let assign34360_e57172: f64 = (-p.p1913);
                        let assign34360_e57174: f64 = (assign34360_e57172 * var_deltemp);
                        let assign34360_e57175: f64 = (1.0 + assign34360_e57174);
                        let assign34360_e57177: f64 = (assign34360_e57175 - 1e-6);
                        let assign34360_e57178: f64 = (assign34360_e57169 / assign34360_e57177);
                        (assign34360_e57178, (-((assign34360_e57169 * (assign34360_e57172 * var_deltemp_dn4)) / (assign34360_e57177 * assign34360_e57177))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign34360_e57180, assign34360_e57180_d_n4,)
            }
        };
        let assign34360_e57182: f64 = (p.p1904 * assign34360_e57181);
        (assign34360_e57182, (p.p1904 * assign34360_e57181_d_n4),)
    } else {
        (var_vsatrsd_t, var_vsatrsd_t_dn4,)
    }
};
        var_vsatrsd_t = assign34360_e57184;
        var_vsatrsd_t_dn4 = assign34360_e57184_d_n4;

        let (assign34370_e57192, assign34370_e57192_d_n0, assign34370_e57192_d_n2, assign34370_e57192_d_n3, assign34370_e57192_d_n4, assign34370_e57192_d_n5, assign34370_e57192_d_n6, assign34370_e57192_d_n7, assign34370_e57192_d_n8, assign34370_e57192_d_n9, assign34370_e57192_d_n10, assign34370_e57192_d_n11, assign34370_e57192_d_n13, assign34370_e57192_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard658 != 0.0)) {
        let assign34370_e57190: f64 = (var_qis - p.p1906);
        (assign34370_e57190, var_qis_dn0, var_qis_dn2, var_qis_dn3, var_qis_dn4, var_qis_dn5, var_qis_dn6, var_qis_dn7, var_qis_dn8, var_qis_dn9, var_qis_dn10, var_qis_dn11, var_qis_dn13, var_qis_dn14,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign34370_e57192;
        var_t0_dn0 = assign34370_e57192_d_n0;
        var_t0_dn2 = assign34370_e57192_d_n2;
        var_t0_dn3 = assign34370_e57192_d_n3;
        var_t0_dn4 = assign34370_e57192_d_n4;
        var_t0_dn5 = assign34370_e57192_d_n5;
        var_t0_dn6 = assign34370_e57192_d_n6;
        var_t0_dn7 = assign34370_e57192_d_n7;
        var_t0_dn8 = assign34370_e57192_d_n8;
        var_t0_dn9 = assign34370_e57192_d_n9;
        var_t0_dn10 = assign34370_e57192_d_n10;
        var_t0_dn11 = assign34370_e57192_d_n11;
        var_t0_dn13 = assign34370_e57192_d_n13;
        var_t0_dn14 = assign34370_e57192_d_n14;

        let (assign34380_e57217, assign34380_e57217_d_n0, assign34380_e57217_d_n2, assign34380_e57217_d_n3, assign34380_e57217_d_n4, assign34380_e57217_d_n5, assign34380_e57217_d_n6, assign34380_e57217_d_n7, assign34380_e57217_d_n8, assign34380_e57217_d_n9, assign34380_e57217_d_n10, assign34380_e57217_d_n11, assign34380_e57217_d_n13, assign34380_e57217_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard658 != 0.0)) {
        let assign34380_e57199: f64 = (var_t0 + 0.1);
        let assign34380_e57202: f64 = (var_t0 - 0.1);
        let assign34380_e57205: f64 = (var_t0 - 0.1);
        let assign34380_e57206: f64 = (assign34380_e57202 * assign34380_e57205);
        let assign34380_e57209: f64 = (0.25 * 2.0);
        let assign34380_e57211: f64 = (assign34380_e57209 * 2.0);
        let assign34380_e57212: f64 = (assign34380_e57206 + assign34380_e57211);
        let assign34380_e57213: f64 = (assign34380_e57212).sqrt();
        let assign34380_e57214: f64 = (assign34380_e57199 + assign34380_e57213);
        let assign34380_e57215: f64 = (0.5 * assign34380_e57214);
        (assign34380_e57215, (0.5 * (var_t0_dn0 + (((var_t0_dn0 * assign34380_e57205) + (assign34380_e57202 * var_t0_dn0)) / (2.0 * assign34380_e57213)))), (0.5 * (var_t0_dn2 + (((var_t0_dn2 * assign34380_e57205) + (assign34380_e57202 * var_t0_dn2)) / (2.0 * assign34380_e57213)))), (0.5 * (var_t0_dn3 + (((var_t0_dn3 * assign34380_e57205) + (assign34380_e57202 * var_t0_dn3)) / (2.0 * assign34380_e57213)))), (0.5 * (var_t0_dn4 + (((var_t0_dn4 * assign34380_e57205) + (assign34380_e57202 * var_t0_dn4)) / (2.0 * assign34380_e57213)))), (0.5 * (var_t0_dn5 + (((var_t0_dn5 * assign34380_e57205) + (assign34380_e57202 * var_t0_dn5)) / (2.0 * assign34380_e57213)))), (0.5 * (var_t0_dn6 + (((var_t0_dn6 * assign34380_e57205) + (assign34380_e57202 * var_t0_dn6)) / (2.0 * assign34380_e57213)))), (0.5 * (var_t0_dn7 + (((var_t0_dn7 * assign34380_e57205) + (assign34380_e57202 * var_t0_dn7)) / (2.0 * assign34380_e57213)))), (0.5 * (var_t0_dn8 + (((var_t0_dn8 * assign34380_e57205) + (assign34380_e57202 * var_t0_dn8)) / (2.0 * assign34380_e57213)))), (0.5 * (var_t0_dn9 + (((var_t0_dn9 * assign34380_e57205) + (assign34380_e57202 * var_t0_dn9)) / (2.0 * assign34380_e57213)))), (0.5 * (var_t0_dn10 + (((var_t0_dn10 * assign34380_e57205) + (assign34380_e57202 * var_t0_dn10)) / (2.0 * assign34380_e57213)))), (0.5 * (var_t0_dn11 + (((var_t0_dn11 * assign34380_e57205) + (assign34380_e57202 * var_t0_dn11)) / (2.0 * assign34380_e57213)))), (0.5 * (var_t0_dn13 + (((var_t0_dn13 * assign34380_e57205) + (assign34380_e57202 * var_t0_dn13)) / (2.0 * assign34380_e57213)))), (0.5 * (var_t0_dn14 + (((var_t0_dn14 * assign34380_e57205) + (assign34380_e57202 * var_t0_dn14)) / (2.0 * assign34380_e57213)))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign34380_e57217;
        var_t0_dn0 = assign34380_e57217_d_n0;
        var_t0_dn2 = assign34380_e57217_d_n2;
        var_t0_dn3 = assign34380_e57217_d_n3;
        var_t0_dn4 = assign34380_e57217_d_n4;
        var_t0_dn5 = assign34380_e57217_d_n5;
        var_t0_dn6 = assign34380_e57217_d_n6;
        var_t0_dn7 = assign34380_e57217_d_n7;
        var_t0_dn8 = assign34380_e57217_d_n8;
        var_t0_dn9 = assign34380_e57217_d_n9;
        var_t0_dn10 = assign34380_e57217_d_n10;
        var_t0_dn11 = assign34380_e57217_d_n11;
        var_t0_dn13 = assign34380_e57217_d_n13;
        var_t0_dn14 = assign34380_e57217_d_n14;

        let (assign34390_e57233, assign34390_e57233_d_n0, assign34390_e57233_d_n2, assign34390_e57233_d_n3, assign34390_e57233_d_n4, assign34390_e57233_d_n5, assign34390_e57233_d_n6, assign34390_e57233_d_n7, assign34390_e57233_d_n8, assign34390_e57233_d_n9, assign34390_e57233_d_n10, assign34390_e57233_d_n11, assign34390_e57233_d_n13, assign34390_e57233_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard658 != 0.0)) {
        let assign34390_e57223: f64 = (10.0 * p.p1907);
        let assign34390_e57225: f64 = (assign34390_e57223 * var_t0);
        let assign34390_e57228: f64 = (10.0 * p.p1907);
        let assign34390_e57230: f64 = (assign34390_e57228 + var_t0);
        let assign34390_e57231: f64 = (assign34390_e57225 / assign34390_e57230);
        (assign34390_e57231, ((((assign34390_e57223 * var_t0_dn0) * assign34390_e57230) - (assign34390_e57225 * var_t0_dn0)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * var_t0_dn2) * assign34390_e57230) - (assign34390_e57225 * var_t0_dn2)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * var_t0_dn3) * assign34390_e57230) - (assign34390_e57225 * var_t0_dn3)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * var_t0_dn4) * assign34390_e57230) - (assign34390_e57225 * var_t0_dn4)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * var_t0_dn5) * assign34390_e57230) - (assign34390_e57225 * var_t0_dn5)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * var_t0_dn6) * assign34390_e57230) - (assign34390_e57225 * var_t0_dn6)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * var_t0_dn7) * assign34390_e57230) - (assign34390_e57225 * var_t0_dn7)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * var_t0_dn8) * assign34390_e57230) - (assign34390_e57225 * var_t0_dn8)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * var_t0_dn9) * assign34390_e57230) - (assign34390_e57225 * var_t0_dn9)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * var_t0_dn10) * assign34390_e57230) - (assign34390_e57225 * var_t0_dn10)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * var_t0_dn11) * assign34390_e57230) - (assign34390_e57225 * var_t0_dn11)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * var_t0_dn13) * assign34390_e57230) - (assign34390_e57225 * var_t0_dn13)) / (assign34390_e57230 * assign34390_e57230)), ((((assign34390_e57223 * var_t0_dn14) * assign34390_e57230) - (assign34390_e57225 * var_t0_dn14)) / (assign34390_e57230 * assign34390_e57230)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn13, var_t1_dn14,)
    }
};
        var_t1 = assign34390_e57233;
        var_t1_dn0 = assign34390_e57233_d_n0;
        var_t1_dn2 = assign34390_e57233_d_n2;
        var_t1_dn3 = assign34390_e57233_d_n3;
        var_t1_dn4 = assign34390_e57233_d_n4;
        var_t1_dn5 = assign34390_e57233_d_n5;
        var_t1_dn6 = assign34390_e57233_d_n6;
        var_t1_dn7 = assign34390_e57233_d_n7;
        var_t1_dn8 = assign34390_e57233_d_n8;
        var_t1_dn9 = assign34390_e57233_d_n9;
        var_t1_dn10 = assign34390_e57233_d_n10;
        var_t1_dn11 = assign34390_e57233_d_n11;
        var_t1_dn13 = assign34390_e57233_d_n13;
        var_t1_dn14 = assign34390_e57233_d_n14;

        let (assign34400_e57245, assign34400_e57245_d_n0, assign34400_e57245_d_n2, assign34400_e57245_d_n3, assign34400_e57245_d_n4, assign34400_e57245_d_n5, assign34400_e57245_d_n6, assign34400_e57245_d_n7, assign34400_e57245_d_n8, assign34400_e57245_d_n9, assign34400_e57245_d_n10, assign34400_e57245_d_n11, assign34400_e57245_d_n13, assign34400_e57245_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard658 != 0.0)) {
        let assign34400_e57241: f64 = (p.p1905 * var_t1);
        let assign34400_e57242: f64 = (1.0 + assign34400_e57241);
        let assign34400_e57243: f64 = (var_vsatrsd_t * assign34400_e57242);
        (assign34400_e57243, (var_vsatrsd_t * (p.p1905 * var_t1_dn0)), (var_vsatrsd_t * (p.p1905 * var_t1_dn2)), (var_vsatrsd_t * (p.p1905 * var_t1_dn3)), ((var_vsatrsd_t_dn4 * assign34400_e57242) + (var_vsatrsd_t * (p.p1905 * var_t1_dn4))), (var_vsatrsd_t * (p.p1905 * var_t1_dn5)), (var_vsatrsd_t * (p.p1905 * var_t1_dn6)), (var_vsatrsd_t * (p.p1905 * var_t1_dn7)), (var_vsatrsd_t * (p.p1905 * var_t1_dn8)), (var_vsatrsd_t * (p.p1905 * var_t1_dn9)), (var_vsatrsd_t * (p.p1905 * var_t1_dn10)), (var_vsatrsd_t * (p.p1905 * var_t1_dn11)), (var_vsatrsd_t * (p.p1905 * var_t1_dn13)), (var_vsatrsd_t * (p.p1905 * var_t1_dn14)),)
    } else {
        (var_vsatrsd_eff, var_vsatrsd_eff_dn0, var_vsatrsd_eff_dn2, var_vsatrsd_eff_dn3, var_vsatrsd_eff_dn4, var_vsatrsd_eff_dn5, var_vsatrsd_eff_dn6, var_vsatrsd_eff_dn7, var_vsatrsd_eff_dn8, var_vsatrsd_eff_dn9, var_vsatrsd_eff_dn10, var_vsatrsd_eff_dn11, var_vsatrsd_eff_dn13, var_vsatrsd_eff_dn14,)
    }
};
        var_vsatrsd_eff = assign34400_e57245;
        var_vsatrsd_eff_dn0 = assign34400_e57245_d_n0;
        var_vsatrsd_eff_dn2 = assign34400_e57245_d_n2;
        var_vsatrsd_eff_dn3 = assign34400_e57245_d_n3;
        var_vsatrsd_eff_dn4 = assign34400_e57245_d_n4;
        var_vsatrsd_eff_dn5 = assign34400_e57245_d_n5;
        var_vsatrsd_eff_dn6 = assign34400_e57245_d_n6;
        var_vsatrsd_eff_dn7 = assign34400_e57245_d_n7;
        var_vsatrsd_eff_dn8 = assign34400_e57245_d_n8;
        var_vsatrsd_eff_dn9 = assign34400_e57245_d_n9;
        var_vsatrsd_eff_dn10 = assign34400_e57245_d_n10;
        var_vsatrsd_eff_dn11 = assign34400_e57245_d_n11;
        var_vsatrsd_eff_dn13 = assign34400_e57245_d_n13;
        var_vsatrsd_eff_dn14 = assign34400_e57245_d_n14;

        let (assign34410_e57286, assign34410_e57286_d_n0, assign34410_e57286_d_n2, assign34410_e57286_d_n3, assign34410_e57286_d_n4, assign34410_e57286_d_n5, assign34410_e57286_d_n6, assign34410_e57286_d_n7, assign34410_e57286_d_n8, assign34410_e57286_d_n9, assign34410_e57286_d_n10, assign34410_e57286_d_n11, assign34410_e57286_d_n13, assign34410_e57286_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard658 != 0.0)) {
        let assign34410_e57251: f64 = (-10000.0);
        let assign34410_e57253: f64 = (assign34410_e57251 * 10.0);
        let (assign34410_e57284, assign34410_e57284_d_n0, assign34410_e57284_d_n2, assign34410_e57284_d_n3, assign34410_e57284_d_n4, assign34410_e57284_d_n5, assign34410_e57284_d_n6, assign34410_e57284_d_n7, assign34410_e57284_d_n8, assign34410_e57284_d_n9, assign34410_e57284_d_n10, assign34410_e57284_d_n11, assign34410_e57284_d_n13, assign34410_e57284_d_n14,) = {
            if (!(var_vsatrsd_eff < assign34410_e57253)) {
                let assign34410_e57260: f64 = (var_vsatrsd_eff * var_vsatrsd_eff);
                let assign34410_e57263: f64 = (4.0 * 10.0);
                let assign34410_e57265: f64 = (assign34410_e57263 * 10.0);
                let assign34410_e57266: f64 = (assign34410_e57260 + assign34410_e57265);
                let assign34410_e57267: f64 = (assign34410_e57266).sqrt();
                let assign34410_e57268: f64 = (var_vsatrsd_eff + assign34410_e57267);
                let assign34410_e57269: f64 = (0.5 * assign34410_e57268);
                (assign34410_e57269, (0.5 * (var_vsatrsd_eff_dn0 + (((var_vsatrsd_eff_dn0 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn0)) / (2.0 * assign34410_e57267)))), (0.5 * (var_vsatrsd_eff_dn2 + (((var_vsatrsd_eff_dn2 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn2)) / (2.0 * assign34410_e57267)))), (0.5 * (var_vsatrsd_eff_dn3 + (((var_vsatrsd_eff_dn3 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn3)) / (2.0 * assign34410_e57267)))), (0.5 * (var_vsatrsd_eff_dn4 + (((var_vsatrsd_eff_dn4 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn4)) / (2.0 * assign34410_e57267)))), (0.5 * (var_vsatrsd_eff_dn5 + (((var_vsatrsd_eff_dn5 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn5)) / (2.0 * assign34410_e57267)))), (0.5 * (var_vsatrsd_eff_dn6 + (((var_vsatrsd_eff_dn6 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn6)) / (2.0 * assign34410_e57267)))), (0.5 * (var_vsatrsd_eff_dn7 + (((var_vsatrsd_eff_dn7 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn7)) / (2.0 * assign34410_e57267)))), (0.5 * (var_vsatrsd_eff_dn8 + (((var_vsatrsd_eff_dn8 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn8)) / (2.0 * assign34410_e57267)))), (0.5 * (var_vsatrsd_eff_dn9 + (((var_vsatrsd_eff_dn9 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn9)) / (2.0 * assign34410_e57267)))), (0.5 * (var_vsatrsd_eff_dn10 + (((var_vsatrsd_eff_dn10 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn10)) / (2.0 * assign34410_e57267)))), (0.5 * (var_vsatrsd_eff_dn11 + (((var_vsatrsd_eff_dn11 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn11)) / (2.0 * assign34410_e57267)))), (0.5 * (var_vsatrsd_eff_dn13 + (((var_vsatrsd_eff_dn13 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn13)) / (2.0 * assign34410_e57267)))), (0.5 * (var_vsatrsd_eff_dn14 + (((var_vsatrsd_eff_dn14 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn14)) / (2.0 * assign34410_e57267)))),)
            } else {
                let assign34410_e57272: f64 = (-10000.0);
                let assign34410_e57274: f64 = (assign34410_e57272 * 10.0);
                let (assign34410_e57283, assign34410_e57283_d_n0, assign34410_e57283_d_n2, assign34410_e57283_d_n3, assign34410_e57283_d_n4, assign34410_e57283_d_n5, assign34410_e57283_d_n6, assign34410_e57283_d_n7, assign34410_e57283_d_n8, assign34410_e57283_d_n9, assign34410_e57283_d_n10, assign34410_e57283_d_n11, assign34410_e57283_d_n13, assign34410_e57283_d_n14,) = {
                    if (var_vsatrsd_eff < assign34410_e57274) {
                        let assign34410_e57277: f64 = (-10.0);
                        let assign34410_e57279: f64 = (assign34410_e57277 * 10.0);
                        let assign34410_e57281: f64 = (assign34410_e57279 / var_vsatrsd_eff);
                        (assign34410_e57281, (-((assign34410_e57279 * var_vsatrsd_eff_dn0) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34410_e57279 * var_vsatrsd_eff_dn2) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34410_e57279 * var_vsatrsd_eff_dn3) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34410_e57279 * var_vsatrsd_eff_dn4) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34410_e57279 * var_vsatrsd_eff_dn5) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34410_e57279 * var_vsatrsd_eff_dn6) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34410_e57279 * var_vsatrsd_eff_dn7) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34410_e57279 * var_vsatrsd_eff_dn8) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34410_e57279 * var_vsatrsd_eff_dn9) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34410_e57279 * var_vsatrsd_eff_dn10) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34410_e57279 * var_vsatrsd_eff_dn11) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34410_e57279 * var_vsatrsd_eff_dn13) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34410_e57279 * var_vsatrsd_eff_dn14) / (var_vsatrsd_eff * var_vsatrsd_eff))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign34410_e57283, assign34410_e57283_d_n0, assign34410_e57283_d_n2, assign34410_e57283_d_n3, assign34410_e57283_d_n4, assign34410_e57283_d_n5, assign34410_e57283_d_n6, assign34410_e57283_d_n7, assign34410_e57283_d_n8, assign34410_e57283_d_n9, assign34410_e57283_d_n10, assign34410_e57283_d_n11, assign34410_e57283_d_n13, assign34410_e57283_d_n14,)
            }
        };
        (assign34410_e57284, assign34410_e57284_d_n0, assign34410_e57284_d_n2, assign34410_e57284_d_n3, assign34410_e57284_d_n4, assign34410_e57284_d_n5, assign34410_e57284_d_n6, assign34410_e57284_d_n7, assign34410_e57284_d_n8, assign34410_e57284_d_n9, assign34410_e57284_d_n10, assign34410_e57284_d_n11, assign34410_e57284_d_n13, assign34410_e57284_d_n14,)
    } else {
        (var_vsatrsd_eff, var_vsatrsd_eff_dn0, var_vsatrsd_eff_dn2, var_vsatrsd_eff_dn3, var_vsatrsd_eff_dn4, var_vsatrsd_eff_dn5, var_vsatrsd_eff_dn6, var_vsatrsd_eff_dn7, var_vsatrsd_eff_dn8, var_vsatrsd_eff_dn9, var_vsatrsd_eff_dn10, var_vsatrsd_eff_dn11, var_vsatrsd_eff_dn13, var_vsatrsd_eff_dn14,)
    }
};
        var_vsatrsd_eff = assign34410_e57286;
        var_vsatrsd_eff_dn0 = assign34410_e57286_d_n0;
        var_vsatrsd_eff_dn2 = assign34410_e57286_d_n2;
        var_vsatrsd_eff_dn3 = assign34410_e57286_d_n3;
        var_vsatrsd_eff_dn4 = assign34410_e57286_d_n4;
        var_vsatrsd_eff_dn5 = assign34410_e57286_d_n5;
        var_vsatrsd_eff_dn6 = assign34410_e57286_d_n6;
        var_vsatrsd_eff_dn7 = assign34410_e57286_d_n7;
        var_vsatrsd_eff_dn8 = assign34410_e57286_d_n8;
        var_vsatrsd_eff_dn9 = assign34410_e57286_d_n9;
        var_vsatrsd_eff_dn10 = assign34410_e57286_d_n10;
        var_vsatrsd_eff_dn11 = assign34410_e57286_d_n11;
        var_vsatrsd_eff_dn13 = assign34410_e57286_d_n13;
        var_vsatrsd_eff_dn14 = assign34410_e57286_d_n14;

        let (assign34420_e57298, assign34420_e57298_d_n0, assign34420_e57298_d_n2, assign34420_e57298_d_n3, assign34420_e57298_d_n4, assign34420_e57298_d_n5, assign34420_e57298_d_n6, assign34420_e57298_d_n7, assign34420_e57298_d_n8, assign34420_e57298_d_n9, assign34420_e57298_d_n10, assign34420_e57298_d_n11, assign34420_e57298_d_n13, assign34420_e57298_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard658 != 0.0)) {
        let assign34420_e57292: f64 = (var_nfintotal * var_weff0);
        let assign34420_e57294: f64 = (assign34420_e57292 * 1.60219e-19);
        let assign34420_e57296: f64 = (assign34420_e57294 * var_vsatrsd_eff);
        (assign34420_e57296, (assign34420_e57294 * var_vsatrsd_eff_dn0), (assign34420_e57294 * var_vsatrsd_eff_dn2), (assign34420_e57294 * var_vsatrsd_eff_dn3), (assign34420_e57294 * var_vsatrsd_eff_dn4), (assign34420_e57294 * var_vsatrsd_eff_dn5), (assign34420_e57294 * var_vsatrsd_eff_dn6), (assign34420_e57294 * var_vsatrsd_eff_dn7), (assign34420_e57294 * var_vsatrsd_eff_dn8), (assign34420_e57294 * var_vsatrsd_eff_dn9), (assign34420_e57294 * var_vsatrsd_eff_dn10), (assign34420_e57294 * var_vsatrsd_eff_dn11), (assign34420_e57294 * var_vsatrsd_eff_dn13), (assign34420_e57294 * var_vsatrsd_eff_dn14),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign34420_e57298;
        var_t2_dn0 = assign34420_e57298_d_n0;
        var_t2_dn2 = assign34420_e57298_d_n2;
        var_t2_dn3 = assign34420_e57298_d_n3;
        var_t2_dn4 = assign34420_e57298_d_n4;
        var_t2_dn5 = assign34420_e57298_d_n5;
        var_t2_dn6 = assign34420_e57298_d_n6;
        var_t2_dn7 = assign34420_e57298_d_n7;
        var_t2_dn8 = assign34420_e57298_d_n8;
        var_t2_dn9 = assign34420_e57298_d_n9;
        var_t2_dn10 = assign34420_e57298_d_n10;
        var_t2_dn11 = assign34420_e57298_d_n11;
        var_t2_dn13 = assign34420_e57298_d_n13;
        var_t2_dn14 = assign34420_e57298_d_n14;

        let (assign34430_e57305, assign34430_e57305_d_n0, assign34430_e57305_d_n2, assign34430_e57305_d_n3, assign34430_e57305_d_n4, assign34430_e57305_d_n5, assign34430_e57305_d_n6, assign34430_e57305_d_n7, assign34430_e57305_d_n8, assign34430_e57305_d_n9, assign34430_e57305_d_n10, assign34430_e57305_d_n11, assign34430_e57305_d_n13, assign34430_e57305_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard658 != 0.0)) {
        let assign34430_e57303: f64 = ((nv9 - nv7)).abs();
        (assign34430_e57303, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, if (nv9 - nv7) >= 0.0 { -1.0 } else { 1.0 }, 0.0, if (nv9 - nv7) >= 0.0 { 1.0 } else { (-1.0) }, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11, var_t5_dn13, var_t5_dn14,)
    }
};
        var_t5 = assign34430_e57305;
        var_t5_dn0 = assign34430_e57305_d_n0;
        var_t5_dn2 = assign34430_e57305_d_n2;
        var_t5_dn3 = assign34430_e57305_d_n3;
        var_t5_dn4 = assign34430_e57305_d_n4;
        var_t5_dn5 = assign34430_e57305_d_n5;
        var_t5_dn6 = assign34430_e57305_d_n6;
        var_t5_dn7 = assign34430_e57305_d_n7;
        var_t5_dn8 = assign34430_e57305_d_n8;
        var_t5_dn9 = assign34430_e57305_d_n9;
        var_t5_dn10 = assign34430_e57305_d_n10;
        var_t5_dn11 = assign34430_e57305_d_n11;
        var_t5_dn13 = assign34430_e57305_d_n13;
        var_t5_dn14 = assign34430_e57305_d_n14;

        let assign34440_e57308: f64 = if p.p1917 == 0.0 { 1.0 } else { 0.0 };
        var_guard660 = assign34440_e57308;

        let (assign34450_e57316, assign34450_e57316_d_n0, assign34450_e57316_d_n2, assign34450_e57316_d_n3, assign34450_e57316_d_n4, assign34450_e57316_d_n5, assign34450_e57316_d_n6, assign34450_e57316_d_n7, assign34450_e57316_d_n8, assign34450_e57316_d_n9, assign34450_e57316_d_n10, assign34450_e57316_d_n11, assign34450_e57316_d_n13, assign34450_e57316_d_n14,) = {
    if (((var_guard657 != 0.0) && (var_guard658 != 0.0)) && (var_guard660 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn13, var_t3_dn14,)
    }
};
        var_t3 = assign34450_e57316;
        var_t3_dn0 = assign34450_e57316_d_n0;
        var_t3_dn2 = assign34450_e57316_d_n2;
        var_t3_dn3 = assign34450_e57316_d_n3;
        var_t3_dn4 = assign34450_e57316_d_n4;
        var_t3_dn5 = assign34450_e57316_d_n5;
        var_t3_dn6 = assign34450_e57316_d_n6;
        var_t3_dn7 = assign34450_e57316_d_n7;
        var_t3_dn8 = assign34450_e57316_d_n8;
        var_t3_dn9 = assign34450_e57316_d_n9;
        var_t3_dn10 = assign34450_e57316_d_n10;
        var_t3_dn11 = assign34450_e57316_d_n11;
        var_t3_dn13 = assign34450_e57316_d_n13;
        var_t3_dn14 = assign34450_e57316_d_n14;

        let (assign34460_e57350, assign34460_e57350_d_n0, assign34460_e57350_d_n2, assign34460_e57350_d_n3, assign34460_e57350_d_n4, assign34460_e57350_d_n5, assign34460_e57350_d_n6, assign34460_e57350_d_n7, assign34460_e57350_d_n8, assign34460_e57350_d_n9, assign34460_e57350_d_n10, assign34460_e57350_d_n11, assign34460_e57350_d_n13, assign34460_e57350_d_n14,) = {
    if (((var_guard657 != 0.0) && (var_guard658 != 0.0)) && (var_guard660 == 0.0)) {
        let assign34460_e57326: f64 = (var_t5 - p.p1916);
        let assign34460_e57328: f64 = assign34460_e57326;
        let assign34460_e57331: f64 = (var_t5 - p.p1916);
        let assign34460_e57333: f64 = assign34460_e57331;
        let assign34460_e57336: f64 = (var_t5 - p.p1916);
        let assign34460_e57338: f64 = assign34460_e57336;
        let assign34460_e57339: f64 = (assign34460_e57333 * assign34460_e57338);
        let assign34460_e57342: f64 = (0.25 * 0.5);
        let assign34460_e57344: f64 = (assign34460_e57342 * 0.5);
        let assign34460_e57345: f64 = (assign34460_e57339 + assign34460_e57344);
        let assign34460_e57346: f64 = (assign34460_e57345).sqrt();
        let assign34460_e57347: f64 = (assign34460_e57328 + assign34460_e57346);
        let assign34460_e57348: f64 = (0.5 * assign34460_e57347);
        (assign34460_e57348, (0.5 * (var_t5_dn0 + (((var_t5_dn0 * assign34460_e57338) + (assign34460_e57333 * var_t5_dn0)) / (2.0 * assign34460_e57346)))), (0.5 * (var_t5_dn2 + (((var_t5_dn2 * assign34460_e57338) + (assign34460_e57333 * var_t5_dn2)) / (2.0 * assign34460_e57346)))), (0.5 * (var_t5_dn3 + (((var_t5_dn3 * assign34460_e57338) + (assign34460_e57333 * var_t5_dn3)) / (2.0 * assign34460_e57346)))), (0.5 * (var_t5_dn4 + (((var_t5_dn4 * assign34460_e57338) + (assign34460_e57333 * var_t5_dn4)) / (2.0 * assign34460_e57346)))), (0.5 * (var_t5_dn5 + (((var_t5_dn5 * assign34460_e57338) + (assign34460_e57333 * var_t5_dn5)) / (2.0 * assign34460_e57346)))), (0.5 * (var_t5_dn6 + (((var_t5_dn6 * assign34460_e57338) + (assign34460_e57333 * var_t5_dn6)) / (2.0 * assign34460_e57346)))), (0.5 * (var_t5_dn7 + (((var_t5_dn7 * assign34460_e57338) + (assign34460_e57333 * var_t5_dn7)) / (2.0 * assign34460_e57346)))), (0.5 * (var_t5_dn8 + (((var_t5_dn8 * assign34460_e57338) + (assign34460_e57333 * var_t5_dn8)) / (2.0 * assign34460_e57346)))), (0.5 * (var_t5_dn9 + (((var_t5_dn9 * assign34460_e57338) + (assign34460_e57333 * var_t5_dn9)) / (2.0 * assign34460_e57346)))), (0.5 * (var_t5_dn10 + (((var_t5_dn10 * assign34460_e57338) + (assign34460_e57333 * var_t5_dn10)) / (2.0 * assign34460_e57346)))), (0.5 * (var_t5_dn11 + (((var_t5_dn11 * assign34460_e57338) + (assign34460_e57333 * var_t5_dn11)) / (2.0 * assign34460_e57346)))), (0.5 * (var_t5_dn13 + (((var_t5_dn13 * assign34460_e57338) + (assign34460_e57333 * var_t5_dn13)) / (2.0 * assign34460_e57346)))), (0.5 * (var_t5_dn14 + (((var_t5_dn14 * assign34460_e57338) + (assign34460_e57333 * var_t5_dn14)) / (2.0 * assign34460_e57346)))),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn13, var_t3_dn14,)
    }
};
        var_t3 = assign34460_e57350;
        var_t3_dn0 = assign34460_e57350_d_n0;
        var_t3_dn2 = assign34460_e57350_d_n2;
        var_t3_dn3 = assign34460_e57350_d_n3;
        var_t3_dn4 = assign34460_e57350_d_n4;
        var_t3_dn5 = assign34460_e57350_d_n5;
        var_t3_dn6 = assign34460_e57350_d_n6;
        var_t3_dn7 = assign34460_e57350_d_n7;
        var_t3_dn8 = assign34460_e57350_d_n8;
        var_t3_dn9 = assign34460_e57350_d_n9;
        var_t3_dn10 = assign34460_e57350_d_n10;
        var_t3_dn11 = assign34460_e57350_d_n11;
        var_t3_dn13 = assign34460_e57350_d_n13;
        var_t3_dn14 = assign34460_e57350_d_n14;

        let (assign34470_e57363, assign34470_e57363_d_n0, assign34470_e57363_d_n2, assign34470_e57363_d_n3, assign34470_e57363_d_n4, assign34470_e57363_d_n5, assign34470_e57363_d_n6, assign34470_e57363_d_n7, assign34470_e57363_d_n8, assign34470_e57363_d_n9, assign34470_e57363_d_n10, assign34470_e57363_d_n11, assign34470_e57363_d_n13, assign34470_e57363_d_n14,) = {
    if (((var_guard657 != 0.0) && (var_guard658 != 0.0)) && (var_guard660 == 0.0)) {
        let assign34470_e57360: f64 = (var_t3 * p.p1917);
        let assign34470_e57361: f64 = (1.0 + assign34470_e57360);
        (assign34470_e57361, (var_t3_dn0 * p.p1917), (var_t3_dn2 * p.p1917), (var_t3_dn3 * p.p1917), (var_t3_dn4 * p.p1917), (var_t3_dn5 * p.p1917), (var_t3_dn6 * p.p1917), (var_t3_dn7 * p.p1917), (var_t3_dn8 * p.p1917), (var_t3_dn9 * p.p1917), (var_t3_dn10 * p.p1917), (var_t3_dn11 * p.p1917), (var_t3_dn13 * p.p1917), (var_t3_dn14 * p.p1917),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn13, var_t3_dn14,)
    }
};
        var_t3 = assign34470_e57363;
        var_t3_dn0 = assign34470_e57363_d_n0;
        var_t3_dn2 = assign34470_e57363_d_n2;
        var_t3_dn3 = assign34470_e57363_d_n3;
        var_t3_dn4 = assign34470_e57363_d_n4;
        var_t3_dn5 = assign34470_e57363_d_n5;
        var_t3_dn6 = assign34470_e57363_d_n6;
        var_t3_dn7 = assign34470_e57363_d_n7;
        var_t3_dn8 = assign34470_e57363_d_n8;
        var_t3_dn9 = assign34470_e57363_d_n9;
        var_t3_dn10 = assign34470_e57363_d_n10;
        var_t3_dn11 = assign34470_e57363_d_n11;
        var_t3_dn13 = assign34470_e57363_d_n13;
        var_t3_dn14 = assign34470_e57363_d_n14;

        let (assign34480_e57373, assign34480_e57373_d_n0, assign34480_e57373_d_n2, assign34480_e57373_d_n3, assign34480_e57373_d_n4, assign34480_e57373_d_n5, assign34480_e57373_d_n6, assign34480_e57373_d_n7, assign34480_e57373_d_n8, assign34480_e57373_d_n9, assign34480_e57373_d_n10, assign34480_e57373_d_n11, assign34480_e57373_d_n13, assign34480_e57373_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard658 != 0.0)) {
        let assign34480_e57369: f64 = (var_t2 * p.p1903);
        let assign34480_e57371: f64 = (assign34480_e57369 * var_t3);
        (assign34480_e57371, (((var_t2_dn0 * p.p1903) * var_t3) + (assign34480_e57369 * var_t3_dn0)), (((var_t2_dn2 * p.p1903) * var_t3) + (assign34480_e57369 * var_t3_dn2)), (((var_t2_dn3 * p.p1903) * var_t3) + (assign34480_e57369 * var_t3_dn3)), (((var_t2_dn4 * p.p1903) * var_t3) + (assign34480_e57369 * var_t3_dn4)), (((var_t2_dn5 * p.p1903) * var_t3) + (assign34480_e57369 * var_t3_dn5)), (((var_t2_dn6 * p.p1903) * var_t3) + (assign34480_e57369 * var_t3_dn6)), (((var_t2_dn7 * p.p1903) * var_t3) + (assign34480_e57369 * var_t3_dn7)), (((var_t2_dn8 * p.p1903) * var_t3) + (assign34480_e57369 * var_t3_dn8)), (((var_t2_dn9 * p.p1903) * var_t3) + (assign34480_e57369 * var_t3_dn9)), (((var_t2_dn10 * p.p1903) * var_t3) + (assign34480_e57369 * var_t3_dn10)), (((var_t2_dn11 * p.p1903) * var_t3) + (assign34480_e57369 * var_t3_dn11)), (((var_t2_dn13 * p.p1903) * var_t3) + (assign34480_e57369 * var_t3_dn13)), (((var_t2_dn14 * p.p1903) * var_t3) + (assign34480_e57369 * var_t3_dn14)),)
    } else {
        (var_isat_rd, var_isat_rd_dn0, var_isat_rd_dn2, var_isat_rd_dn3, var_isat_rd_dn4, var_isat_rd_dn5, var_isat_rd_dn6, var_isat_rd_dn7, var_isat_rd_dn8, var_isat_rd_dn9, var_isat_rd_dn10, var_isat_rd_dn11, var_isat_rd_dn13, var_isat_rd_dn14,)
    }
};
        var_isat_rd = assign34480_e57373;
        var_isat_rd_dn0 = assign34480_e57373_d_n0;
        var_isat_rd_dn2 = assign34480_e57373_d_n2;
        var_isat_rd_dn3 = assign34480_e57373_d_n3;
        var_isat_rd_dn4 = assign34480_e57373_d_n4;
        var_isat_rd_dn5 = assign34480_e57373_d_n5;
        var_isat_rd_dn6 = assign34480_e57373_d_n6;
        var_isat_rd_dn7 = assign34480_e57373_d_n7;
        var_isat_rd_dn8 = assign34480_e57373_d_n8;
        var_isat_rd_dn9 = assign34480_e57373_d_n9;
        var_isat_rd_dn10 = assign34480_e57373_d_n10;
        var_isat_rd_dn11 = assign34480_e57373_d_n11;
        var_isat_rd_dn13 = assign34480_e57373_d_n13;
        var_isat_rd_dn14 = assign34480_e57373_d_n14;

        let (assign34490_e57383, assign34490_e57383_d_n0, assign34490_e57383_d_n2, assign34490_e57383_d_n3, assign34490_e57383_d_n4, assign34490_e57383_d_n5, assign34490_e57383_d_n6, assign34490_e57383_d_n7, assign34490_e57383_d_n8, assign34490_e57383_d_n9, assign34490_e57383_d_n10, assign34490_e57383_d_n11, assign34490_e57383_d_n13, assign34490_e57383_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard658 != 0.0)) {
        let assign34490_e57379: f64 = (var_rdstempvs * p.p1910);
        let assign34490_e57381: f64 = (assign34490_e57379 * var_weffwrfactor);
        (assign34490_e57381, 0.0, 0.0, 0.0, ((var_rdstempvs_dn4 * p.p1910) * var_weffwrfactor), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11, var_t4_dn13, var_t4_dn14,)
    }
};
        var_t4 = assign34490_e57383;
        var_t4_dn0 = assign34490_e57383_d_n0;
        var_t4_dn2 = assign34490_e57383_d_n2;
        var_t4_dn3 = assign34490_e57383_d_n3;
        var_t4_dn4 = assign34490_e57383_d_n4;
        var_t4_dn5 = assign34490_e57383_d_n5;
        var_t4_dn6 = assign34490_e57383_d_n6;
        var_t4_dn7 = assign34490_e57383_d_n7;
        var_t4_dn8 = assign34490_e57383_d_n8;
        var_t4_dn9 = assign34490_e57383_d_n9;
        var_t4_dn10 = assign34490_e57383_d_n10;
        var_t4_dn11 = assign34490_e57383_d_n11;
        var_t4_dn13 = assign34490_e57383_d_n13;
        var_t4_dn14 = assign34490_e57383_d_n14;

        *var_guard659_slot = var_guard659;
        *var_guard660_slot = var_guard660;
        *var_isat_rd_slot = var_isat_rd;
        *var_isat_rd_dn0_slot = var_isat_rd_dn0;
        *var_isat_rd_dn10_slot = var_isat_rd_dn10;
        *var_isat_rd_dn11_slot = var_isat_rd_dn11;
        *var_isat_rd_dn13_slot = var_isat_rd_dn13;
        *var_isat_rd_dn14_slot = var_isat_rd_dn14;
        *var_isat_rd_dn2_slot = var_isat_rd_dn2;
        *var_isat_rd_dn3_slot = var_isat_rd_dn3;
        *var_isat_rd_dn4_slot = var_isat_rd_dn4;
        *var_isat_rd_dn5_slot = var_isat_rd_dn5;
        *var_isat_rd_dn6_slot = var_isat_rd_dn6;
        *var_isat_rd_dn7_slot = var_isat_rd_dn7;
        *var_isat_rd_dn8_slot = var_isat_rd_dn8;
        *var_isat_rd_dn9_slot = var_isat_rd_dn9;
        *var_rdstempvs_slot = var_rdstempvs;
        *var_rdstempvs_dn4_slot = var_rdstempvs_dn4;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
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
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
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
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
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
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn3_slot = var_t3_dn3;
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
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn14_slot = var_t4_dn14;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn3_slot = var_t4_dn3;
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
        *var_t5_dn13_slot = var_t5_dn13;
        *var_t5_dn14_slot = var_t5_dn14;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_dn9_slot = var_t5_dn9;
        *var_vsatrsd_eff_slot = var_vsatrsd_eff;
        *var_vsatrsd_eff_dn0_slot = var_vsatrsd_eff_dn0;
        *var_vsatrsd_eff_dn10_slot = var_vsatrsd_eff_dn10;
        *var_vsatrsd_eff_dn11_slot = var_vsatrsd_eff_dn11;
        *var_vsatrsd_eff_dn13_slot = var_vsatrsd_eff_dn13;
        *var_vsatrsd_eff_dn14_slot = var_vsatrsd_eff_dn14;
        *var_vsatrsd_eff_dn2_slot = var_vsatrsd_eff_dn2;
        *var_vsatrsd_eff_dn3_slot = var_vsatrsd_eff_dn3;
        *var_vsatrsd_eff_dn4_slot = var_vsatrsd_eff_dn4;
        *var_vsatrsd_eff_dn5_slot = var_vsatrsd_eff_dn5;
        *var_vsatrsd_eff_dn6_slot = var_vsatrsd_eff_dn6;
        *var_vsatrsd_eff_dn7_slot = var_vsatrsd_eff_dn7;
        *var_vsatrsd_eff_dn8_slot = var_vsatrsd_eff_dn8;
        *var_vsatrsd_eff_dn9_slot = var_vsatrsd_eff_dn9;
        *var_vsatrsd_t_slot = var_vsatrsd_t;
        *var_vsatrsd_t_dn4_slot = var_vsatrsd_t_dn4;
    }

    pub(super) fn stamp_transient_block_133(
        p: &Parameters,
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_guard657: f64,
        var_guard658: f64,
        var_isat_rd: f64,
        var_isat_rd_dn0: f64,
        var_isat_rd_dn10: f64,
        var_isat_rd_dn11: f64,
        var_isat_rd_dn13: f64,
        var_isat_rd_dn14: f64,
        var_isat_rd_dn2: f64,
        var_isat_rd_dn3: f64,
        var_isat_rd_dn4: f64,
        var_isat_rd_dn5: f64,
        var_isat_rd_dn6: f64,
        var_isat_rd_dn7: f64,
        var_isat_rd_dn8: f64,
        var_isat_rd_dn9: f64,
        var_nfintotal: f64,
        var_qis: f64,
        var_qis_dn0: f64,
        var_qis_dn10: f64,
        var_qis_dn11: f64,
        var_qis_dn13: f64,
        var_qis_dn14: f64,
        var_qis_dn2: f64,
        var_qis_dn3: f64,
        var_qis_dn4: f64,
        var_qis_dn5: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_qis_dn9: f64,
        var_t5: f64,
        var_t5_dn0: f64,
        var_t5_dn10: f64,
        var_t5_dn11: f64,
        var_t5_dn13: f64,
        var_t5_dn14: f64,
        var_t5_dn2: f64,
        var_t5_dn3: f64,
        var_t5_dn4: f64,
        var_t5_dn5: f64,
        var_t5_dn6: f64,
        var_t5_dn7: f64,
        var_t5_dn8: f64,
        var_t5_dn9: f64,
        var_weff0: f64,
        var_weffwrfactor: f64,
        var_delta_vsrd_slot: &mut f64,
        var_delta_vsrd_dn0_slot: &mut f64,
        var_delta_vsrd_dn10_slot: &mut f64,
        var_delta_vsrd_dn11_slot: &mut f64,
        var_delta_vsrd_dn13_slot: &mut f64,
        var_delta_vsrd_dn14_slot: &mut f64,
        var_delta_vsrd_dn2_slot: &mut f64,
        var_delta_vsrd_dn3_slot: &mut f64,
        var_delta_vsrd_dn4_slot: &mut f64,
        var_delta_vsrd_dn5_slot: &mut f64,
        var_delta_vsrd_dn6_slot: &mut f64,
        var_delta_vsrd_dn7_slot: &mut f64,
        var_delta_vsrd_dn8_slot: &mut f64,
        var_delta_vsrd_dn9_slot: &mut f64,
        var_guard661_slot: &mut f64,
        var_guard662_slot: &mut f64,
        var_guard663_slot: &mut f64,
        var_isat_rs_slot: &mut f64,
        var_isat_rs_dn0_slot: &mut f64,
        var_isat_rs_dn10_slot: &mut f64,
        var_isat_rs_dn11_slot: &mut f64,
        var_isat_rs_dn13_slot: &mut f64,
        var_isat_rs_dn14_slot: &mut f64,
        var_isat_rs_dn2_slot: &mut f64,
        var_isat_rs_dn3_slot: &mut f64,
        var_isat_rs_dn4_slot: &mut f64,
        var_isat_rs_dn5_slot: &mut f64,
        var_isat_rs_dn6_slot: &mut f64,
        var_isat_rs_dn7_slot: &mut f64,
        var_isat_rs_dn8_slot: &mut f64,
        var_isat_rs_dn9_slot: &mut f64,
        var_rdstempvs_slot: &mut f64,
        var_rdstempvs_dn4_slot: &mut f64,
        var_rvs_d_slot: &mut f64,
        var_rvs_d_dn0_slot: &mut f64,
        var_rvs_d_dn10_slot: &mut f64,
        var_rvs_d_dn11_slot: &mut f64,
        var_rvs_d_dn13_slot: &mut f64,
        var_rvs_d_dn14_slot: &mut f64,
        var_rvs_d_dn2_slot: &mut f64,
        var_rvs_d_dn3_slot: &mut f64,
        var_rvs_d_dn4_slot: &mut f64,
        var_rvs_d_dn5_slot: &mut f64,
        var_rvs_d_dn6_slot: &mut f64,
        var_rvs_d_dn7_slot: &mut f64,
        var_rvs_d_dn8_slot: &mut f64,
        var_rvs_d_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
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
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
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
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn13_slot: &mut f64,
        var_t4_dn14_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn13_slot: &mut f64,
        var_t6_dn14_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_vsat_rd_slot: &mut f64,
        var_vsat_rd_dn0_slot: &mut f64,
        var_vsat_rd_dn10_slot: &mut f64,
        var_vsat_rd_dn11_slot: &mut f64,
        var_vsat_rd_dn13_slot: &mut f64,
        var_vsat_rd_dn14_slot: &mut f64,
        var_vsat_rd_dn2_slot: &mut f64,
        var_vsat_rd_dn3_slot: &mut f64,
        var_vsat_rd_dn4_slot: &mut f64,
        var_vsat_rd_dn5_slot: &mut f64,
        var_vsat_rd_dn6_slot: &mut f64,
        var_vsat_rd_dn7_slot: &mut f64,
        var_vsat_rd_dn8_slot: &mut f64,
        var_vsat_rd_dn9_slot: &mut f64,
        var_vsatrsd_eff_slot: &mut f64,
        var_vsatrsd_eff_dn0_slot: &mut f64,
        var_vsatrsd_eff_dn10_slot: &mut f64,
        var_vsatrsd_eff_dn11_slot: &mut f64,
        var_vsatrsd_eff_dn13_slot: &mut f64,
        var_vsatrsd_eff_dn14_slot: &mut f64,
        var_vsatrsd_eff_dn2_slot: &mut f64,
        var_vsatrsd_eff_dn3_slot: &mut f64,
        var_vsatrsd_eff_dn4_slot: &mut f64,
        var_vsatrsd_eff_dn5_slot: &mut f64,
        var_vsatrsd_eff_dn6_slot: &mut f64,
        var_vsatrsd_eff_dn7_slot: &mut f64,
        var_vsatrsd_eff_dn8_slot: &mut f64,
        var_vsatrsd_eff_dn9_slot: &mut f64,
        var_vsatrsd_t_slot: &mut f64,
        var_vsatrsd_t_dn4_slot: &mut f64,
    ) {
        let mut var_delta_vsrd: f64 = *var_delta_vsrd_slot;
        let mut var_delta_vsrd_dn0: f64 = *var_delta_vsrd_dn0_slot;
        let mut var_delta_vsrd_dn10: f64 = *var_delta_vsrd_dn10_slot;
        let mut var_delta_vsrd_dn11: f64 = *var_delta_vsrd_dn11_slot;
        let mut var_delta_vsrd_dn13: f64 = *var_delta_vsrd_dn13_slot;
        let mut var_delta_vsrd_dn14: f64 = *var_delta_vsrd_dn14_slot;
        let mut var_delta_vsrd_dn2: f64 = *var_delta_vsrd_dn2_slot;
        let mut var_delta_vsrd_dn3: f64 = *var_delta_vsrd_dn3_slot;
        let mut var_delta_vsrd_dn4: f64 = *var_delta_vsrd_dn4_slot;
        let mut var_delta_vsrd_dn5: f64 = *var_delta_vsrd_dn5_slot;
        let mut var_delta_vsrd_dn6: f64 = *var_delta_vsrd_dn6_slot;
        let mut var_delta_vsrd_dn7: f64 = *var_delta_vsrd_dn7_slot;
        let mut var_delta_vsrd_dn8: f64 = *var_delta_vsrd_dn8_slot;
        let mut var_delta_vsrd_dn9: f64 = *var_delta_vsrd_dn9_slot;
        let mut var_guard661: f64 = *var_guard661_slot;
        let mut var_guard662: f64 = *var_guard662_slot;
        let mut var_guard663: f64 = *var_guard663_slot;
        let mut var_isat_rs: f64 = *var_isat_rs_slot;
        let mut var_isat_rs_dn0: f64 = *var_isat_rs_dn0_slot;
        let mut var_isat_rs_dn10: f64 = *var_isat_rs_dn10_slot;
        let mut var_isat_rs_dn11: f64 = *var_isat_rs_dn11_slot;
        let mut var_isat_rs_dn13: f64 = *var_isat_rs_dn13_slot;
        let mut var_isat_rs_dn14: f64 = *var_isat_rs_dn14_slot;
        let mut var_isat_rs_dn2: f64 = *var_isat_rs_dn2_slot;
        let mut var_isat_rs_dn3: f64 = *var_isat_rs_dn3_slot;
        let mut var_isat_rs_dn4: f64 = *var_isat_rs_dn4_slot;
        let mut var_isat_rs_dn5: f64 = *var_isat_rs_dn5_slot;
        let mut var_isat_rs_dn6: f64 = *var_isat_rs_dn6_slot;
        let mut var_isat_rs_dn7: f64 = *var_isat_rs_dn7_slot;
        let mut var_isat_rs_dn8: f64 = *var_isat_rs_dn8_slot;
        let mut var_isat_rs_dn9: f64 = *var_isat_rs_dn9_slot;
        let mut var_rdstempvs: f64 = *var_rdstempvs_slot;
        let mut var_rdstempvs_dn4: f64 = *var_rdstempvs_dn4_slot;
        let mut var_rvs_d: f64 = *var_rvs_d_slot;
        let mut var_rvs_d_dn0: f64 = *var_rvs_d_dn0_slot;
        let mut var_rvs_d_dn10: f64 = *var_rvs_d_dn10_slot;
        let mut var_rvs_d_dn11: f64 = *var_rvs_d_dn11_slot;
        let mut var_rvs_d_dn13: f64 = *var_rvs_d_dn13_slot;
        let mut var_rvs_d_dn14: f64 = *var_rvs_d_dn14_slot;
        let mut var_rvs_d_dn2: f64 = *var_rvs_d_dn2_slot;
        let mut var_rvs_d_dn3: f64 = *var_rvs_d_dn3_slot;
        let mut var_rvs_d_dn4: f64 = *var_rvs_d_dn4_slot;
        let mut var_rvs_d_dn5: f64 = *var_rvs_d_dn5_slot;
        let mut var_rvs_d_dn6: f64 = *var_rvs_d_dn6_slot;
        let mut var_rvs_d_dn7: f64 = *var_rvs_d_dn7_slot;
        let mut var_rvs_d_dn8: f64 = *var_rvs_d_dn8_slot;
        let mut var_rvs_d_dn9: f64 = *var_rvs_d_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
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
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
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
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn14: f64 = *var_t4_dn14_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn13: f64 = *var_t6_dn13_slot;
        let mut var_t6_dn14: f64 = *var_t6_dn14_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_vsat_rd: f64 = *var_vsat_rd_slot;
        let mut var_vsat_rd_dn0: f64 = *var_vsat_rd_dn0_slot;
        let mut var_vsat_rd_dn10: f64 = *var_vsat_rd_dn10_slot;
        let mut var_vsat_rd_dn11: f64 = *var_vsat_rd_dn11_slot;
        let mut var_vsat_rd_dn13: f64 = *var_vsat_rd_dn13_slot;
        let mut var_vsat_rd_dn14: f64 = *var_vsat_rd_dn14_slot;
        let mut var_vsat_rd_dn2: f64 = *var_vsat_rd_dn2_slot;
        let mut var_vsat_rd_dn3: f64 = *var_vsat_rd_dn3_slot;
        let mut var_vsat_rd_dn4: f64 = *var_vsat_rd_dn4_slot;
        let mut var_vsat_rd_dn5: f64 = *var_vsat_rd_dn5_slot;
        let mut var_vsat_rd_dn6: f64 = *var_vsat_rd_dn6_slot;
        let mut var_vsat_rd_dn7: f64 = *var_vsat_rd_dn7_slot;
        let mut var_vsat_rd_dn8: f64 = *var_vsat_rd_dn8_slot;
        let mut var_vsat_rd_dn9: f64 = *var_vsat_rd_dn9_slot;
        let mut var_vsatrsd_eff: f64 = *var_vsatrsd_eff_slot;
        let mut var_vsatrsd_eff_dn0: f64 = *var_vsatrsd_eff_dn0_slot;
        let mut var_vsatrsd_eff_dn10: f64 = *var_vsatrsd_eff_dn10_slot;
        let mut var_vsatrsd_eff_dn11: f64 = *var_vsatrsd_eff_dn11_slot;
        let mut var_vsatrsd_eff_dn13: f64 = *var_vsatrsd_eff_dn13_slot;
        let mut var_vsatrsd_eff_dn14: f64 = *var_vsatrsd_eff_dn14_slot;
        let mut var_vsatrsd_eff_dn2: f64 = *var_vsatrsd_eff_dn2_slot;
        let mut var_vsatrsd_eff_dn3: f64 = *var_vsatrsd_eff_dn3_slot;
        let mut var_vsatrsd_eff_dn4: f64 = *var_vsatrsd_eff_dn4_slot;
        let mut var_vsatrsd_eff_dn5: f64 = *var_vsatrsd_eff_dn5_slot;
        let mut var_vsatrsd_eff_dn6: f64 = *var_vsatrsd_eff_dn6_slot;
        let mut var_vsatrsd_eff_dn7: f64 = *var_vsatrsd_eff_dn7_slot;
        let mut var_vsatrsd_eff_dn8: f64 = *var_vsatrsd_eff_dn8_slot;
        let mut var_vsatrsd_eff_dn9: f64 = *var_vsatrsd_eff_dn9_slot;
        let mut var_vsatrsd_t: f64 = *var_vsatrsd_t_slot;
        let mut var_vsatrsd_t_dn4: f64 = *var_vsatrsd_t_dn4_slot;

        let (assign34500_e57391, assign34500_e57391_d_n0, assign34500_e57391_d_n2, assign34500_e57391_d_n3, assign34500_e57391_d_n4, assign34500_e57391_d_n5, assign34500_e57391_d_n6, assign34500_e57391_d_n7, assign34500_e57391_d_n8, assign34500_e57391_d_n9, assign34500_e57391_d_n10, assign34500_e57391_d_n11, assign34500_e57391_d_n13, assign34500_e57391_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard658 != 0.0)) {
        let assign34500_e57389: f64 = (var_isat_rd * var_t4);
        (assign34500_e57389, ((var_isat_rd_dn0 * var_t4) + (var_isat_rd * var_t4_dn0)), ((var_isat_rd_dn2 * var_t4) + (var_isat_rd * var_t4_dn2)), ((var_isat_rd_dn3 * var_t4) + (var_isat_rd * var_t4_dn3)), ((var_isat_rd_dn4 * var_t4) + (var_isat_rd * var_t4_dn4)), ((var_isat_rd_dn5 * var_t4) + (var_isat_rd * var_t4_dn5)), ((var_isat_rd_dn6 * var_t4) + (var_isat_rd * var_t4_dn6)), ((var_isat_rd_dn7 * var_t4) + (var_isat_rd * var_t4_dn7)), ((var_isat_rd_dn8 * var_t4) + (var_isat_rd * var_t4_dn8)), ((var_isat_rd_dn9 * var_t4) + (var_isat_rd * var_t4_dn9)), ((var_isat_rd_dn10 * var_t4) + (var_isat_rd * var_t4_dn10)), ((var_isat_rd_dn11 * var_t4) + (var_isat_rd * var_t4_dn11)), ((var_isat_rd_dn13 * var_t4) + (var_isat_rd * var_t4_dn13)), ((var_isat_rd_dn14 * var_t4) + (var_isat_rd * var_t4_dn14)),)
    } else {
        (var_vsat_rd, var_vsat_rd_dn0, var_vsat_rd_dn2, var_vsat_rd_dn3, var_vsat_rd_dn4, var_vsat_rd_dn5, var_vsat_rd_dn6, var_vsat_rd_dn7, var_vsat_rd_dn8, var_vsat_rd_dn9, var_vsat_rd_dn10, var_vsat_rd_dn11, var_vsat_rd_dn13, var_vsat_rd_dn14,)
    }
};
        var_vsat_rd = assign34500_e57391;
        var_vsat_rd_dn0 = assign34500_e57391_d_n0;
        var_vsat_rd_dn2 = assign34500_e57391_d_n2;
        var_vsat_rd_dn3 = assign34500_e57391_d_n3;
        var_vsat_rd_dn4 = assign34500_e57391_d_n4;
        var_vsat_rd_dn5 = assign34500_e57391_d_n5;
        var_vsat_rd_dn6 = assign34500_e57391_d_n6;
        var_vsat_rd_dn7 = assign34500_e57391_d_n7;
        var_vsat_rd_dn8 = assign34500_e57391_d_n8;
        var_vsat_rd_dn9 = assign34500_e57391_d_n9;
        var_vsat_rd_dn10 = assign34500_e57391_d_n10;
        var_vsat_rd_dn11 = assign34500_e57391_d_n11;
        var_vsat_rd_dn13 = assign34500_e57391_d_n13;
        var_vsat_rd_dn14 = assign34500_e57391_d_n14;

        let (assign34510_e57415, assign34510_e57415_d_n0, assign34510_e57415_d_n2, assign34510_e57415_d_n3, assign34510_e57415_d_n4, assign34510_e57415_d_n5, assign34510_e57415_d_n6, assign34510_e57415_d_n7, assign34510_e57415_d_n8, assign34510_e57415_d_n9, assign34510_e57415_d_n10, assign34510_e57415_d_n11, assign34510_e57415_d_n13, assign34510_e57415_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard658 != 0.0)) {
        let assign34510_e57398: f64 = (4.0 - p.p1908);
        let assign34510_e57399: f64 = (var_t5).powf(assign34510_e57398);
        let assign34510_e57403: f64 = (4.0 - p.p1908);
        let assign34510_e57404: f64 = (var_t5).powf(assign34510_e57403);
        let assign34510_e57409: f64 = (4.0 - p.p1908);
        let assign34510_e57410: f64 = (var_vsat_rd).powf(assign34510_e57409);
        let assign34510_e57411: f64 = (p.p1914 * assign34510_e57410);
        let assign34510_e57412: f64 = (assign34510_e57404 + assign34510_e57411);
        let assign34510_e57413: f64 = (assign34510_e57399 / assign34510_e57412);
        (assign34510_e57413, (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((var_t5).powf(assign34510_e57398 - 1.0) * var_t5_dn0)) } } else { (assign34510_e57399 * (assign34510_e57398 * (var_t5_dn0 / var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((var_t5).powf(assign34510_e57403 - 1.0) * var_t5_dn0)) } } else { (assign34510_e57404 * (assign34510_e57403 * (var_t5_dn0 / var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((var_vsat_rd).powf(assign34510_e57409 - 1.0) * var_vsat_rd_dn0)) } } else { (assign34510_e57410 * (assign34510_e57409 * (var_vsat_rd_dn0 / var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((var_t5).powf(assign34510_e57398 - 1.0) * var_t5_dn2)) } } else { (assign34510_e57399 * (assign34510_e57398 * (var_t5_dn2 / var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((var_t5).powf(assign34510_e57403 - 1.0) * var_t5_dn2)) } } else { (assign34510_e57404 * (assign34510_e57403 * (var_t5_dn2 / var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((var_vsat_rd).powf(assign34510_e57409 - 1.0) * var_vsat_rd_dn2)) } } else { (assign34510_e57410 * (assign34510_e57409 * (var_vsat_rd_dn2 / var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((var_t5).powf(assign34510_e57398 - 1.0) * var_t5_dn3)) } } else { (assign34510_e57399 * (assign34510_e57398 * (var_t5_dn3 / var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((var_t5).powf(assign34510_e57403 - 1.0) * var_t5_dn3)) } } else { (assign34510_e57404 * (assign34510_e57403 * (var_t5_dn3 / var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((var_vsat_rd).powf(assign34510_e57409 - 1.0) * var_vsat_rd_dn3)) } } else { (assign34510_e57410 * (assign34510_e57409 * (var_vsat_rd_dn3 / var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((var_t5).powf(assign34510_e57398 - 1.0) * var_t5_dn4)) } } else { (assign34510_e57399 * (assign34510_e57398 * (var_t5_dn4 / var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((var_t5).powf(assign34510_e57403 - 1.0) * var_t5_dn4)) } } else { (assign34510_e57404 * (assign34510_e57403 * (var_t5_dn4 / var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((var_vsat_rd).powf(assign34510_e57409 - 1.0) * var_vsat_rd_dn4)) } } else { (assign34510_e57410 * (assign34510_e57409 * (var_vsat_rd_dn4 / var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((var_t5).powf(assign34510_e57398 - 1.0) * var_t5_dn5)) } } else { (assign34510_e57399 * (assign34510_e57398 * (var_t5_dn5 / var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((var_t5).powf(assign34510_e57403 - 1.0) * var_t5_dn5)) } } else { (assign34510_e57404 * (assign34510_e57403 * (var_t5_dn5 / var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((var_vsat_rd).powf(assign34510_e57409 - 1.0) * var_vsat_rd_dn5)) } } else { (assign34510_e57410 * (assign34510_e57409 * (var_vsat_rd_dn5 / var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((var_t5).powf(assign34510_e57398 - 1.0) * var_t5_dn6)) } } else { (assign34510_e57399 * (assign34510_e57398 * (var_t5_dn6 / var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((var_t5).powf(assign34510_e57403 - 1.0) * var_t5_dn6)) } } else { (assign34510_e57404 * (assign34510_e57403 * (var_t5_dn6 / var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((var_vsat_rd).powf(assign34510_e57409 - 1.0) * var_vsat_rd_dn6)) } } else { (assign34510_e57410 * (assign34510_e57409 * (var_vsat_rd_dn6 / var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((var_t5).powf(assign34510_e57398 - 1.0) * var_t5_dn7)) } } else { (assign34510_e57399 * (assign34510_e57398 * (var_t5_dn7 / var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((var_t5).powf(assign34510_e57403 - 1.0) * var_t5_dn7)) } } else { (assign34510_e57404 * (assign34510_e57403 * (var_t5_dn7 / var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((var_vsat_rd).powf(assign34510_e57409 - 1.0) * var_vsat_rd_dn7)) } } else { (assign34510_e57410 * (assign34510_e57409 * (var_vsat_rd_dn7 / var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((var_t5).powf(assign34510_e57398 - 1.0) * var_t5_dn8)) } } else { (assign34510_e57399 * (assign34510_e57398 * (var_t5_dn8 / var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((var_t5).powf(assign34510_e57403 - 1.0) * var_t5_dn8)) } } else { (assign34510_e57404 * (assign34510_e57403 * (var_t5_dn8 / var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((var_vsat_rd).powf(assign34510_e57409 - 1.0) * var_vsat_rd_dn8)) } } else { (assign34510_e57410 * (assign34510_e57409 * (var_vsat_rd_dn8 / var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((var_t5).powf(assign34510_e57398 - 1.0) * var_t5_dn9)) } } else { (assign34510_e57399 * (assign34510_e57398 * (var_t5_dn9 / var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((var_t5).powf(assign34510_e57403 - 1.0) * var_t5_dn9)) } } else { (assign34510_e57404 * (assign34510_e57403 * (var_t5_dn9 / var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((var_vsat_rd).powf(assign34510_e57409 - 1.0) * var_vsat_rd_dn9)) } } else { (assign34510_e57410 * (assign34510_e57409 * (var_vsat_rd_dn9 / var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((var_t5).powf(assign34510_e57398 - 1.0) * var_t5_dn10)) } } else { (assign34510_e57399 * (assign34510_e57398 * (var_t5_dn10 / var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((var_t5).powf(assign34510_e57403 - 1.0) * var_t5_dn10)) } } else { (assign34510_e57404 * (assign34510_e57403 * (var_t5_dn10 / var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((var_vsat_rd).powf(assign34510_e57409 - 1.0) * var_vsat_rd_dn10)) } } else { (assign34510_e57410 * (assign34510_e57409 * (var_vsat_rd_dn10 / var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((var_t5).powf(assign34510_e57398 - 1.0) * var_t5_dn11)) } } else { (assign34510_e57399 * (assign34510_e57398 * (var_t5_dn11 / var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((var_t5).powf(assign34510_e57403 - 1.0) * var_t5_dn11)) } } else { (assign34510_e57404 * (assign34510_e57403 * (var_t5_dn11 / var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((var_vsat_rd).powf(assign34510_e57409 - 1.0) * var_vsat_rd_dn11)) } } else { (assign34510_e57410 * (assign34510_e57409 * (var_vsat_rd_dn11 / var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((var_t5).powf(assign34510_e57398 - 1.0) * var_t5_dn13)) } } else { (assign34510_e57399 * (assign34510_e57398 * (var_t5_dn13 / var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((var_t5).powf(assign34510_e57403 - 1.0) * var_t5_dn13)) } } else { (assign34510_e57404 * (assign34510_e57403 * (var_t5_dn13 / var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((var_vsat_rd).powf(assign34510_e57409 - 1.0) * var_vsat_rd_dn13)) } } else { (assign34510_e57410 * (assign34510_e57409 * (var_vsat_rd_dn13 / var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)), (((if 0.0 == 0.0 && ((assign34510_e57398) as f64).is_finite() && ((assign34510_e57398) as f64).fract() == 0.0 { if assign34510_e57398 == 0.0 { 0.0 } else { (assign34510_e57398 * ((var_t5).powf(assign34510_e57398 - 1.0) * var_t5_dn14)) } } else { (assign34510_e57399 * (assign34510_e57398 * (var_t5_dn14 / var_t5))) } * assign34510_e57412) - (assign34510_e57399 * (if 0.0 == 0.0 && ((assign34510_e57403) as f64).is_finite() && ((assign34510_e57403) as f64).fract() == 0.0 { if assign34510_e57403 == 0.0 { 0.0 } else { (assign34510_e57403 * ((var_t5).powf(assign34510_e57403 - 1.0) * var_t5_dn14)) } } else { (assign34510_e57404 * (assign34510_e57403 * (var_t5_dn14 / var_t5))) } + (p.p1914 * if 0.0 == 0.0 && ((assign34510_e57409) as f64).is_finite() && ((assign34510_e57409) as f64).fract() == 0.0 { if assign34510_e57409 == 0.0 { 0.0 } else { (assign34510_e57409 * ((var_vsat_rd).powf(assign34510_e57409 - 1.0) * var_vsat_rd_dn14)) } } else { (assign34510_e57410 * (assign34510_e57409 * (var_vsat_rd_dn14 / var_vsat_rd))) })))) / (assign34510_e57412 * assign34510_e57412)),)
    } else {
        (var_delta_vsrd, var_delta_vsrd_dn0, var_delta_vsrd_dn2, var_delta_vsrd_dn3, var_delta_vsrd_dn4, var_delta_vsrd_dn5, var_delta_vsrd_dn6, var_delta_vsrd_dn7, var_delta_vsrd_dn8, var_delta_vsrd_dn9, var_delta_vsrd_dn10, var_delta_vsrd_dn11, var_delta_vsrd_dn13, var_delta_vsrd_dn14,)
    }
};
        var_delta_vsrd = assign34510_e57415;
        var_delta_vsrd_dn0 = assign34510_e57415_d_n0;
        var_delta_vsrd_dn2 = assign34510_e57415_d_n2;
        var_delta_vsrd_dn3 = assign34510_e57415_d_n3;
        var_delta_vsrd_dn4 = assign34510_e57415_d_n4;
        var_delta_vsrd_dn5 = assign34510_e57415_d_n5;
        var_delta_vsrd_dn6 = assign34510_e57415_d_n6;
        var_delta_vsrd_dn7 = assign34510_e57415_d_n7;
        var_delta_vsrd_dn8 = assign34510_e57415_d_n8;
        var_delta_vsrd_dn9 = assign34510_e57415_d_n9;
        var_delta_vsrd_dn10 = assign34510_e57415_d_n10;
        var_delta_vsrd_dn11 = assign34510_e57415_d_n11;
        var_delta_vsrd_dn13 = assign34510_e57415_d_n13;
        var_delta_vsrd_dn14 = assign34510_e57415_d_n14;

        let (assign34520_e57429, assign34520_e57429_d_n0, assign34520_e57429_d_n2, assign34520_e57429_d_n3, assign34520_e57429_d_n4, assign34520_e57429_d_n5, assign34520_e57429_d_n6, assign34520_e57429_d_n7, assign34520_e57429_d_n8, assign34520_e57429_d_n9, assign34520_e57429_d_n10, assign34520_e57429_d_n11, assign34520_e57429_d_n13, assign34520_e57429_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard658 != 0.0)) {
        let assign34520_e57422: f64 = (1.0 / p.p1908);
        let assign34520_e57423: f64 = (var_delta_vsrd).powf(assign34520_e57422);
        let assign34520_e57425: f64 = (assign34520_e57423 * var_t5);
        let assign34520_e57427: f64 = (assign34520_e57425 / var_vsat_rd);
        (assign34520_e57427, (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((var_delta_vsrd).powf(assign34520_e57422 - 1.0) * var_delta_vsrd_dn0)) } } else { (assign34520_e57423 * (assign34520_e57422 * (var_delta_vsrd_dn0 / var_delta_vsrd))) } * var_t5) + (assign34520_e57423 * var_t5_dn0)) * var_vsat_rd) - (assign34520_e57425 * var_vsat_rd_dn0)) / (var_vsat_rd * var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((var_delta_vsrd).powf(assign34520_e57422 - 1.0) * var_delta_vsrd_dn2)) } } else { (assign34520_e57423 * (assign34520_e57422 * (var_delta_vsrd_dn2 / var_delta_vsrd))) } * var_t5) + (assign34520_e57423 * var_t5_dn2)) * var_vsat_rd) - (assign34520_e57425 * var_vsat_rd_dn2)) / (var_vsat_rd * var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((var_delta_vsrd).powf(assign34520_e57422 - 1.0) * var_delta_vsrd_dn3)) } } else { (assign34520_e57423 * (assign34520_e57422 * (var_delta_vsrd_dn3 / var_delta_vsrd))) } * var_t5) + (assign34520_e57423 * var_t5_dn3)) * var_vsat_rd) - (assign34520_e57425 * var_vsat_rd_dn3)) / (var_vsat_rd * var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((var_delta_vsrd).powf(assign34520_e57422 - 1.0) * var_delta_vsrd_dn4)) } } else { (assign34520_e57423 * (assign34520_e57422 * (var_delta_vsrd_dn4 / var_delta_vsrd))) } * var_t5) + (assign34520_e57423 * var_t5_dn4)) * var_vsat_rd) - (assign34520_e57425 * var_vsat_rd_dn4)) / (var_vsat_rd * var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((var_delta_vsrd).powf(assign34520_e57422 - 1.0) * var_delta_vsrd_dn5)) } } else { (assign34520_e57423 * (assign34520_e57422 * (var_delta_vsrd_dn5 / var_delta_vsrd))) } * var_t5) + (assign34520_e57423 * var_t5_dn5)) * var_vsat_rd) - (assign34520_e57425 * var_vsat_rd_dn5)) / (var_vsat_rd * var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((var_delta_vsrd).powf(assign34520_e57422 - 1.0) * var_delta_vsrd_dn6)) } } else { (assign34520_e57423 * (assign34520_e57422 * (var_delta_vsrd_dn6 / var_delta_vsrd))) } * var_t5) + (assign34520_e57423 * var_t5_dn6)) * var_vsat_rd) - (assign34520_e57425 * var_vsat_rd_dn6)) / (var_vsat_rd * var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((var_delta_vsrd).powf(assign34520_e57422 - 1.0) * var_delta_vsrd_dn7)) } } else { (assign34520_e57423 * (assign34520_e57422 * (var_delta_vsrd_dn7 / var_delta_vsrd))) } * var_t5) + (assign34520_e57423 * var_t5_dn7)) * var_vsat_rd) - (assign34520_e57425 * var_vsat_rd_dn7)) / (var_vsat_rd * var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((var_delta_vsrd).powf(assign34520_e57422 - 1.0) * var_delta_vsrd_dn8)) } } else { (assign34520_e57423 * (assign34520_e57422 * (var_delta_vsrd_dn8 / var_delta_vsrd))) } * var_t5) + (assign34520_e57423 * var_t5_dn8)) * var_vsat_rd) - (assign34520_e57425 * var_vsat_rd_dn8)) / (var_vsat_rd * var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((var_delta_vsrd).powf(assign34520_e57422 - 1.0) * var_delta_vsrd_dn9)) } } else { (assign34520_e57423 * (assign34520_e57422 * (var_delta_vsrd_dn9 / var_delta_vsrd))) } * var_t5) + (assign34520_e57423 * var_t5_dn9)) * var_vsat_rd) - (assign34520_e57425 * var_vsat_rd_dn9)) / (var_vsat_rd * var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((var_delta_vsrd).powf(assign34520_e57422 - 1.0) * var_delta_vsrd_dn10)) } } else { (assign34520_e57423 * (assign34520_e57422 * (var_delta_vsrd_dn10 / var_delta_vsrd))) } * var_t5) + (assign34520_e57423 * var_t5_dn10)) * var_vsat_rd) - (assign34520_e57425 * var_vsat_rd_dn10)) / (var_vsat_rd * var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((var_delta_vsrd).powf(assign34520_e57422 - 1.0) * var_delta_vsrd_dn11)) } } else { (assign34520_e57423 * (assign34520_e57422 * (var_delta_vsrd_dn11 / var_delta_vsrd))) } * var_t5) + (assign34520_e57423 * var_t5_dn11)) * var_vsat_rd) - (assign34520_e57425 * var_vsat_rd_dn11)) / (var_vsat_rd * var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((var_delta_vsrd).powf(assign34520_e57422 - 1.0) * var_delta_vsrd_dn13)) } } else { (assign34520_e57423 * (assign34520_e57422 * (var_delta_vsrd_dn13 / var_delta_vsrd))) } * var_t5) + (assign34520_e57423 * var_t5_dn13)) * var_vsat_rd) - (assign34520_e57425 * var_vsat_rd_dn13)) / (var_vsat_rd * var_vsat_rd)), (((((if 0.0 == 0.0 && ((assign34520_e57422) as f64).is_finite() && ((assign34520_e57422) as f64).fract() == 0.0 { if assign34520_e57422 == 0.0 { 0.0 } else { (assign34520_e57422 * ((var_delta_vsrd).powf(assign34520_e57422 - 1.0) * var_delta_vsrd_dn14)) } } else { (assign34520_e57423 * (assign34520_e57422 * (var_delta_vsrd_dn14 / var_delta_vsrd))) } * var_t5) + (assign34520_e57423 * var_t5_dn14)) * var_vsat_rd) - (assign34520_e57425 * var_vsat_rd_dn14)) / (var_vsat_rd * var_vsat_rd)),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn13, var_t6_dn14,)
    }
};
        var_t6 = assign34520_e57429;
        var_t6_dn0 = assign34520_e57429_d_n0;
        var_t6_dn2 = assign34520_e57429_d_n2;
        var_t6_dn3 = assign34520_e57429_d_n3;
        var_t6_dn4 = assign34520_e57429_d_n4;
        var_t6_dn5 = assign34520_e57429_d_n5;
        var_t6_dn6 = assign34520_e57429_d_n6;
        var_t6_dn7 = assign34520_e57429_d_n7;
        var_t6_dn8 = assign34520_e57429_d_n8;
        var_t6_dn9 = assign34520_e57429_d_n9;
        var_t6_dn10 = assign34520_e57429_d_n10;
        var_t6_dn11 = assign34520_e57429_d_n11;
        var_t6_dn13 = assign34520_e57429_d_n13;
        var_t6_dn14 = assign34520_e57429_d_n14;

        let (assign34530_e57445, assign34530_e57445_d_n0, assign34530_e57445_d_n2, assign34530_e57445_d_n3, assign34530_e57445_d_n4, assign34530_e57445_d_n5, assign34530_e57445_d_n6, assign34530_e57445_d_n7, assign34530_e57445_d_n8, assign34530_e57445_d_n9, assign34530_e57445_d_n10, assign34530_e57445_d_n11, assign34530_e57445_d_n13, assign34530_e57445_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard658 != 0.0)) {
        let assign34530_e57437: f64 = (var_t6).powf(p.p1908);
        let assign34530_e57438: f64 = (1.0 + assign34530_e57437);
        let assign34530_e57441: f64 = (1.0 / p.p1908);
        let assign34530_e57442: f64 = (assign34530_e57438).powf(assign34530_e57441);
        let assign34530_e57443: f64 = (var_t4 * assign34530_e57442);
        (assign34530_e57443, ((var_t4_dn0 * assign34530_e57442) + (var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn0)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn0 / var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn0)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn0 / var_t6))) } / assign34530_e57438))) })), ((var_t4_dn2 * assign34530_e57442) + (var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn2)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn2 / var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn2)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn2 / var_t6))) } / assign34530_e57438))) })), ((var_t4_dn3 * assign34530_e57442) + (var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn3)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn3 / var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn3)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn3 / var_t6))) } / assign34530_e57438))) })), ((var_t4_dn4 * assign34530_e57442) + (var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn4)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn4 / var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn4)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn4 / var_t6))) } / assign34530_e57438))) })), ((var_t4_dn5 * assign34530_e57442) + (var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn5)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn5 / var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn5)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn5 / var_t6))) } / assign34530_e57438))) })), ((var_t4_dn6 * assign34530_e57442) + (var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn6)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn6 / var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn6)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn6 / var_t6))) } / assign34530_e57438))) })), ((var_t4_dn7 * assign34530_e57442) + (var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn7)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn7 / var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn7)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn7 / var_t6))) } / assign34530_e57438))) })), ((var_t4_dn8 * assign34530_e57442) + (var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn8)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn8 / var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn8)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn8 / var_t6))) } / assign34530_e57438))) })), ((var_t4_dn9 * assign34530_e57442) + (var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn9)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn9 / var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn9)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn9 / var_t6))) } / assign34530_e57438))) })), ((var_t4_dn10 * assign34530_e57442) + (var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn10)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn10 / var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn10)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn10 / var_t6))) } / assign34530_e57438))) })), ((var_t4_dn11 * assign34530_e57442) + (var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn11)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn11 / var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn11)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn11 / var_t6))) } / assign34530_e57438))) })), ((var_t4_dn13 * assign34530_e57442) + (var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn13)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn13 / var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn13)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn13 / var_t6))) } / assign34530_e57438))) })), ((var_t4_dn14 * assign34530_e57442) + (var_t4 * if 0.0 == 0.0 && ((assign34530_e57441) as f64).is_finite() && ((assign34530_e57441) as f64).fract() == 0.0 { if assign34530_e57441 == 0.0 { 0.0 } else { (assign34530_e57441 * ((assign34530_e57438).powf(assign34530_e57441 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn14)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn14 / var_t6))) })) } } else { (assign34530_e57442 * (assign34530_e57441 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn14)) } } else { (assign34530_e57437 * (p.p1908 * (var_t6_dn14 / var_t6))) } / assign34530_e57438))) })),)
    } else {
        (var_rvs_d, var_rvs_d_dn0, var_rvs_d_dn2, var_rvs_d_dn3, var_rvs_d_dn4, var_rvs_d_dn5, var_rvs_d_dn6, var_rvs_d_dn7, var_rvs_d_dn8, var_rvs_d_dn9, var_rvs_d_dn10, var_rvs_d_dn11, var_rvs_d_dn13, var_rvs_d_dn14,)
    }
};
        var_rvs_d = assign34530_e57445;
        var_rvs_d_dn0 = assign34530_e57445_d_n0;
        var_rvs_d_dn2 = assign34530_e57445_d_n2;
        var_rvs_d_dn3 = assign34530_e57445_d_n3;
        var_rvs_d_dn4 = assign34530_e57445_d_n4;
        var_rvs_d_dn5 = assign34530_e57445_d_n5;
        var_rvs_d_dn6 = assign34530_e57445_d_n6;
        var_rvs_d_dn7 = assign34530_e57445_d_n7;
        var_rvs_d_dn8 = assign34530_e57445_d_n8;
        var_rvs_d_dn9 = assign34530_e57445_d_n9;
        var_rvs_d_dn10 = assign34530_e57445_d_n10;
        var_rvs_d_dn11 = assign34530_e57445_d_n11;
        var_rvs_d_dn13 = assign34530_e57445_d_n13;
        var_rvs_d_dn14 = assign34530_e57445_d_n14;

        let assign34540_e57448: f64 = if p.p1911 > 0.0 { 1.0 } else { 0.0 };
        var_guard661 = assign34540_e57448;

        let assign34550_e57451: f64 = if p.p1910 == 0.0 { 1.0 } else { 0.0 };
        var_guard662 = assign34550_e57451;

        let (assign34560_e57530, assign34560_e57530_d_n4,) = {
    if (((var_guard657 != 0.0) && (var_guard661 != 0.0)) && (var_guard662 != 0.0)) {
        let assign34560_e57460: f64 = (p.p1912 * var_deltemp);
        let assign34560_e57461: f64 = (1.0 + assign34560_e57460);
        let assign34560_e57463: f64 = (assign34560_e57461 - 1e-6);
        let assign34560_e57465: f64 = (-10000.0);
        let assign34560_e57467: f64 = (assign34560_e57465 * 0.001);
        let (assign34560_e57528, assign34560_e57528_d_n4,) = {
            if (!(assign34560_e57463 < assign34560_e57467)) {
                let assign34560_e57474: f64 = (p.p1912 * var_deltemp);
                let assign34560_e57475: f64 = (1.0 + assign34560_e57474);
                let assign34560_e57477: f64 = (assign34560_e57475 - 1e-6);
                let assign34560_e57481: f64 = (p.p1912 * var_deltemp);
                let assign34560_e57482: f64 = (1.0 + assign34560_e57481);
                let assign34560_e57484: f64 = (assign34560_e57482 - 1e-6);
                let assign34560_e57488: f64 = (p.p1912 * var_deltemp);
                let assign34560_e57489: f64 = (1.0 + assign34560_e57488);
                let assign34560_e57491: f64 = (assign34560_e57489 - 1e-6);
                let assign34560_e57492: f64 = (assign34560_e57484 * assign34560_e57491);
                let assign34560_e57495: f64 = (4.0 * 0.001);
                let assign34560_e57497: f64 = (assign34560_e57495 * 0.001);
                let assign34560_e57498: f64 = (assign34560_e57492 + assign34560_e57497);
                let assign34560_e57499: f64 = (assign34560_e57498).sqrt();
                let assign34560_e57500: f64 = (assign34560_e57477 + assign34560_e57499);
                let assign34560_e57501: f64 = (0.5 * assign34560_e57500);
                (assign34560_e57501, (0.5 * ((p.p1912 * var_deltemp_dn4) + ((((p.p1912 * var_deltemp_dn4) * assign34560_e57491) + (assign34560_e57484 * (p.p1912 * var_deltemp_dn4))) / (2.0 * assign34560_e57499)))),)
            } else {
                let assign34560_e57505: f64 = (p.p1912 * var_deltemp);
                let assign34560_e57506: f64 = (1.0 + assign34560_e57505);
                let assign34560_e57508: f64 = (assign34560_e57506 - 1e-6);
                let assign34560_e57510: f64 = (-10000.0);
                let assign34560_e57512: f64 = (assign34560_e57510 * 0.001);
                let (assign34560_e57527, assign34560_e57527_d_n4,) = {
                    if (assign34560_e57508 < assign34560_e57512) {
                        let assign34560_e57515: f64 = (-0.001);
                        let assign34560_e57517: f64 = (assign34560_e57515 * 0.001);
                        let assign34560_e57521: f64 = (p.p1912 * var_deltemp);
                        let assign34560_e57522: f64 = (1.0 + assign34560_e57521);
                        let assign34560_e57524: f64 = (assign34560_e57522 - 1e-6);
                        let assign34560_e57525: f64 = (assign34560_e57517 / assign34560_e57524);
                        (assign34560_e57525, (-((assign34560_e57517 * (p.p1912 * var_deltemp_dn4)) / (assign34560_e57524 * assign34560_e57524))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign34560_e57527, assign34560_e57527_d_n4,)
            }
        };
        (assign34560_e57528, assign34560_e57528_d_n4,)
    } else {
        (var_rdstempvs, var_rdstempvs_dn4,)
    }
};
        var_rdstempvs = assign34560_e57530;
        var_rdstempvs_dn4 = assign34560_e57530_d_n4;

        let assign34570_e57533: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        var_guard663 = assign34570_e57533;

        let (assign34580_e57586, assign34580_e57586_d_n4,) = {
    if ((((var_guard657 != 0.0) && (var_guard661 != 0.0)) && (var_guard662 != 0.0)) && (var_guard663 != 0.0)) {
        let assign34580_e57543: f64 = (-p.p1904);
        let assign34580_e57546: f64 = (-p.p1913);
        let assign34580_e57548: f64 = (assign34580_e57546 * var_deltemp);
        let assign34580_e57550: f64 = (-p.p1904);
        let assign34580_e57551: f64 = (assign34580_e57548 - assign34580_e57550);
        let assign34580_e57553: f64 = (assign34580_e57551 - 1e-6);
        let assign34580_e57555: f64 = (-p.p1913);
        let assign34580_e57557: f64 = (assign34580_e57555 * var_deltemp);
        let assign34580_e57559: f64 = (-p.p1904);
        let assign34580_e57560: f64 = (assign34580_e57557 - assign34580_e57559);
        let assign34580_e57562: f64 = (assign34580_e57560 - 1e-6);
        let assign34580_e57564: f64 = (-p.p1913);
        let assign34580_e57566: f64 = (assign34580_e57564 * var_deltemp);
        let assign34580_e57568: f64 = (-p.p1904);
        let assign34580_e57569: f64 = (assign34580_e57566 - assign34580_e57568);
        let assign34580_e57571: f64 = (assign34580_e57569 - 1e-6);
        let assign34580_e57572: f64 = (assign34580_e57562 * assign34580_e57571);
        let assign34580_e57575: f64 = (-p.p1904);
        let assign34580_e57576: f64 = (4.0 * assign34580_e57575);
        let assign34580_e57578: f64 = (assign34580_e57576 * 1e-6);
        let assign34580_e57579: f64 = (assign34580_e57572 - assign34580_e57578);
        let assign34580_e57580: f64 = (assign34580_e57579).sqrt();
        let assign34580_e57581: f64 = (assign34580_e57553 + assign34580_e57580);
        let assign34580_e57582: f64 = (0.5 * assign34580_e57581);
        let assign34580_e57583: f64 = (assign34580_e57543 + assign34580_e57582);
        let assign34580_e57584: f64 = (p.p1904 + assign34580_e57583);
        (assign34580_e57584, (0.5 * ((assign34580_e57546 * var_deltemp_dn4) + ((((assign34580_e57555 * var_deltemp_dn4) * assign34580_e57571) + (assign34580_e57562 * (assign34580_e57564 * var_deltemp_dn4))) / (2.0 * assign34580_e57580)))),)
    } else {
        (var_vsatrsd_t, var_vsatrsd_t_dn4,)
    }
};
        var_vsatrsd_t = assign34580_e57586;
        var_vsatrsd_t_dn4 = assign34580_e57586_d_n4;

        let (assign34590_e57676, assign34590_e57676_d_n4,) = {
    if ((((var_guard657 != 0.0) && (var_guard661 != 0.0)) && (var_guard662 != 0.0)) && (var_guard663 == 0.0)) {
        let assign34590_e57598: f64 = (-p.p1913);
        let assign34590_e57600: f64 = (assign34590_e57598 * var_deltemp);
        let assign34590_e57601: f64 = (1.0 + assign34590_e57600);
        let assign34590_e57603: f64 = (assign34590_e57601 - 1e-6);
        let assign34590_e57605: f64 = (-10000.0);
        let assign34590_e57607: f64 = (assign34590_e57605 * 0.001);
        let (assign34590_e57673, assign34590_e57673_d_n4,) = {
            if (!(assign34590_e57603 < assign34590_e57607)) {
                let assign34590_e57613: f64 = (-p.p1913);
                let assign34590_e57615: f64 = (assign34590_e57613 * var_deltemp);
                let assign34590_e57616: f64 = (1.0 + assign34590_e57615);
                let assign34590_e57618: f64 = (assign34590_e57616 - 1e-6);
                let assign34590_e57621: f64 = (-p.p1913);
                let assign34590_e57623: f64 = (assign34590_e57621 * var_deltemp);
                let assign34590_e57624: f64 = (1.0 + assign34590_e57623);
                let assign34590_e57626: f64 = (assign34590_e57624 - 1e-6);
                let assign34590_e57629: f64 = (-p.p1913);
                let assign34590_e57631: f64 = (assign34590_e57629 * var_deltemp);
                let assign34590_e57632: f64 = (1.0 + assign34590_e57631);
                let assign34590_e57634: f64 = (assign34590_e57632 - 1e-6);
                let assign34590_e57635: f64 = (assign34590_e57626 * assign34590_e57634);
                let assign34590_e57638: f64 = (4.0 * 0.001);
                let assign34590_e57640: f64 = (assign34590_e57638 * 0.001);
                let assign34590_e57641: f64 = (assign34590_e57635 + assign34590_e57640);
                let assign34590_e57642: f64 = (assign34590_e57641).sqrt();
                let assign34590_e57643: f64 = (assign34590_e57618 + assign34590_e57642);
                let assign34590_e57644: f64 = (0.5 * assign34590_e57643);
                (assign34590_e57644, (0.5 * ((assign34590_e57613 * var_deltemp_dn4) + ((((assign34590_e57621 * var_deltemp_dn4) * assign34590_e57634) + (assign34590_e57626 * (assign34590_e57629 * var_deltemp_dn4))) / (2.0 * assign34590_e57642)))),)
            } else {
                let assign34590_e57647: f64 = (-p.p1913);
                let assign34590_e57649: f64 = (assign34590_e57647 * var_deltemp);
                let assign34590_e57650: f64 = (1.0 + assign34590_e57649);
                let assign34590_e57652: f64 = (assign34590_e57650 - 1e-6);
                let assign34590_e57654: f64 = (-10000.0);
                let assign34590_e57656: f64 = (assign34590_e57654 * 0.001);
                let (assign34590_e57672, assign34590_e57672_d_n4,) = {
                    if (assign34590_e57652 < assign34590_e57656) {
                        let assign34590_e57659: f64 = (-0.001);
                        let assign34590_e57661: f64 = (assign34590_e57659 * 0.001);
                        let assign34590_e57664: f64 = (-p.p1913);
                        let assign34590_e57666: f64 = (assign34590_e57664 * var_deltemp);
                        let assign34590_e57667: f64 = (1.0 + assign34590_e57666);
                        let assign34590_e57669: f64 = (assign34590_e57667 - 1e-6);
                        let assign34590_e57670: f64 = (assign34590_e57661 / assign34590_e57669);
                        (assign34590_e57670, (-((assign34590_e57661 * (assign34590_e57664 * var_deltemp_dn4)) / (assign34590_e57669 * assign34590_e57669))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign34590_e57672, assign34590_e57672_d_n4,)
            }
        };
        let assign34590_e57674: f64 = (p.p1904 * assign34590_e57673);
        (assign34590_e57674, (p.p1904 * assign34590_e57673_d_n4),)
    } else {
        (var_vsatrsd_t, var_vsatrsd_t_dn4,)
    }
};
        var_vsatrsd_t = assign34590_e57676;
        var_vsatrsd_t_dn4 = assign34590_e57676_d_n4;

        let (assign34600_e57686, assign34600_e57686_d_n0, assign34600_e57686_d_n2, assign34600_e57686_d_n3, assign34600_e57686_d_n4, assign34600_e57686_d_n5, assign34600_e57686_d_n6, assign34600_e57686_d_n7, assign34600_e57686_d_n8, assign34600_e57686_d_n9, assign34600_e57686_d_n10, assign34600_e57686_d_n11, assign34600_e57686_d_n13, assign34600_e57686_d_n14,) = {
    if (((var_guard657 != 0.0) && (var_guard661 != 0.0)) && (var_guard662 != 0.0)) {
        let assign34600_e57684: f64 = (var_qis - p.p1906);
        (assign34600_e57684, var_qis_dn0, var_qis_dn2, var_qis_dn3, var_qis_dn4, var_qis_dn5, var_qis_dn6, var_qis_dn7, var_qis_dn8, var_qis_dn9, var_qis_dn10, var_qis_dn11, var_qis_dn13, var_qis_dn14,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign34600_e57686;
        var_t0_dn0 = assign34600_e57686_d_n0;
        var_t0_dn2 = assign34600_e57686_d_n2;
        var_t0_dn3 = assign34600_e57686_d_n3;
        var_t0_dn4 = assign34600_e57686_d_n4;
        var_t0_dn5 = assign34600_e57686_d_n5;
        var_t0_dn6 = assign34600_e57686_d_n6;
        var_t0_dn7 = assign34600_e57686_d_n7;
        var_t0_dn8 = assign34600_e57686_d_n8;
        var_t0_dn9 = assign34600_e57686_d_n9;
        var_t0_dn10 = assign34600_e57686_d_n10;
        var_t0_dn11 = assign34600_e57686_d_n11;
        var_t0_dn13 = assign34600_e57686_d_n13;
        var_t0_dn14 = assign34600_e57686_d_n14;

        let (assign34610_e57713, assign34610_e57713_d_n0, assign34610_e57713_d_n2, assign34610_e57713_d_n3, assign34610_e57713_d_n4, assign34610_e57713_d_n5, assign34610_e57713_d_n6, assign34610_e57713_d_n7, assign34610_e57713_d_n8, assign34610_e57713_d_n9, assign34610_e57713_d_n10, assign34610_e57713_d_n11, assign34610_e57713_d_n13, assign34610_e57713_d_n14,) = {
    if (((var_guard657 != 0.0) && (var_guard661 != 0.0)) && (var_guard662 != 0.0)) {
        let assign34610_e57695: f64 = (var_t0 + 0.1);
        let assign34610_e57698: f64 = (var_t0 - 0.1);
        let assign34610_e57701: f64 = (var_t0 - 0.1);
        let assign34610_e57702: f64 = (assign34610_e57698 * assign34610_e57701);
        let assign34610_e57705: f64 = (0.25 * 2.0);
        let assign34610_e57707: f64 = (assign34610_e57705 * 2.0);
        let assign34610_e57708: f64 = (assign34610_e57702 + assign34610_e57707);
        let assign34610_e57709: f64 = (assign34610_e57708).sqrt();
        let assign34610_e57710: f64 = (assign34610_e57695 + assign34610_e57709);
        let assign34610_e57711: f64 = (0.5 * assign34610_e57710);
        (assign34610_e57711, (0.5 * (var_t0_dn0 + (((var_t0_dn0 * assign34610_e57701) + (assign34610_e57698 * var_t0_dn0)) / (2.0 * assign34610_e57709)))), (0.5 * (var_t0_dn2 + (((var_t0_dn2 * assign34610_e57701) + (assign34610_e57698 * var_t0_dn2)) / (2.0 * assign34610_e57709)))), (0.5 * (var_t0_dn3 + (((var_t0_dn3 * assign34610_e57701) + (assign34610_e57698 * var_t0_dn3)) / (2.0 * assign34610_e57709)))), (0.5 * (var_t0_dn4 + (((var_t0_dn4 * assign34610_e57701) + (assign34610_e57698 * var_t0_dn4)) / (2.0 * assign34610_e57709)))), (0.5 * (var_t0_dn5 + (((var_t0_dn5 * assign34610_e57701) + (assign34610_e57698 * var_t0_dn5)) / (2.0 * assign34610_e57709)))), (0.5 * (var_t0_dn6 + (((var_t0_dn6 * assign34610_e57701) + (assign34610_e57698 * var_t0_dn6)) / (2.0 * assign34610_e57709)))), (0.5 * (var_t0_dn7 + (((var_t0_dn7 * assign34610_e57701) + (assign34610_e57698 * var_t0_dn7)) / (2.0 * assign34610_e57709)))), (0.5 * (var_t0_dn8 + (((var_t0_dn8 * assign34610_e57701) + (assign34610_e57698 * var_t0_dn8)) / (2.0 * assign34610_e57709)))), (0.5 * (var_t0_dn9 + (((var_t0_dn9 * assign34610_e57701) + (assign34610_e57698 * var_t0_dn9)) / (2.0 * assign34610_e57709)))), (0.5 * (var_t0_dn10 + (((var_t0_dn10 * assign34610_e57701) + (assign34610_e57698 * var_t0_dn10)) / (2.0 * assign34610_e57709)))), (0.5 * (var_t0_dn11 + (((var_t0_dn11 * assign34610_e57701) + (assign34610_e57698 * var_t0_dn11)) / (2.0 * assign34610_e57709)))), (0.5 * (var_t0_dn13 + (((var_t0_dn13 * assign34610_e57701) + (assign34610_e57698 * var_t0_dn13)) / (2.0 * assign34610_e57709)))), (0.5 * (var_t0_dn14 + (((var_t0_dn14 * assign34610_e57701) + (assign34610_e57698 * var_t0_dn14)) / (2.0 * assign34610_e57709)))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign34610_e57713;
        var_t0_dn0 = assign34610_e57713_d_n0;
        var_t0_dn2 = assign34610_e57713_d_n2;
        var_t0_dn3 = assign34610_e57713_d_n3;
        var_t0_dn4 = assign34610_e57713_d_n4;
        var_t0_dn5 = assign34610_e57713_d_n5;
        var_t0_dn6 = assign34610_e57713_d_n6;
        var_t0_dn7 = assign34610_e57713_d_n7;
        var_t0_dn8 = assign34610_e57713_d_n8;
        var_t0_dn9 = assign34610_e57713_d_n9;
        var_t0_dn10 = assign34610_e57713_d_n10;
        var_t0_dn11 = assign34610_e57713_d_n11;
        var_t0_dn13 = assign34610_e57713_d_n13;
        var_t0_dn14 = assign34610_e57713_d_n14;

        let (assign34620_e57731, assign34620_e57731_d_n0, assign34620_e57731_d_n2, assign34620_e57731_d_n3, assign34620_e57731_d_n4, assign34620_e57731_d_n5, assign34620_e57731_d_n6, assign34620_e57731_d_n7, assign34620_e57731_d_n8, assign34620_e57731_d_n9, assign34620_e57731_d_n10, assign34620_e57731_d_n11, assign34620_e57731_d_n13, assign34620_e57731_d_n14,) = {
    if (((var_guard657 != 0.0) && (var_guard661 != 0.0)) && (var_guard662 != 0.0)) {
        let assign34620_e57721: f64 = (10.0 * p.p1907);
        let assign34620_e57723: f64 = (assign34620_e57721 * var_t0);
        let assign34620_e57726: f64 = (10.0 * p.p1907);
        let assign34620_e57728: f64 = (assign34620_e57726 + var_t0);
        let assign34620_e57729: f64 = (assign34620_e57723 / assign34620_e57728);
        (assign34620_e57729, ((((assign34620_e57721 * var_t0_dn0) * assign34620_e57728) - (assign34620_e57723 * var_t0_dn0)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * var_t0_dn2) * assign34620_e57728) - (assign34620_e57723 * var_t0_dn2)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * var_t0_dn3) * assign34620_e57728) - (assign34620_e57723 * var_t0_dn3)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * var_t0_dn4) * assign34620_e57728) - (assign34620_e57723 * var_t0_dn4)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * var_t0_dn5) * assign34620_e57728) - (assign34620_e57723 * var_t0_dn5)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * var_t0_dn6) * assign34620_e57728) - (assign34620_e57723 * var_t0_dn6)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * var_t0_dn7) * assign34620_e57728) - (assign34620_e57723 * var_t0_dn7)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * var_t0_dn8) * assign34620_e57728) - (assign34620_e57723 * var_t0_dn8)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * var_t0_dn9) * assign34620_e57728) - (assign34620_e57723 * var_t0_dn9)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * var_t0_dn10) * assign34620_e57728) - (assign34620_e57723 * var_t0_dn10)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * var_t0_dn11) * assign34620_e57728) - (assign34620_e57723 * var_t0_dn11)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * var_t0_dn13) * assign34620_e57728) - (assign34620_e57723 * var_t0_dn13)) / (assign34620_e57728 * assign34620_e57728)), ((((assign34620_e57721 * var_t0_dn14) * assign34620_e57728) - (assign34620_e57723 * var_t0_dn14)) / (assign34620_e57728 * assign34620_e57728)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn13, var_t1_dn14,)
    }
};
        var_t1 = assign34620_e57731;
        var_t1_dn0 = assign34620_e57731_d_n0;
        var_t1_dn2 = assign34620_e57731_d_n2;
        var_t1_dn3 = assign34620_e57731_d_n3;
        var_t1_dn4 = assign34620_e57731_d_n4;
        var_t1_dn5 = assign34620_e57731_d_n5;
        var_t1_dn6 = assign34620_e57731_d_n6;
        var_t1_dn7 = assign34620_e57731_d_n7;
        var_t1_dn8 = assign34620_e57731_d_n8;
        var_t1_dn9 = assign34620_e57731_d_n9;
        var_t1_dn10 = assign34620_e57731_d_n10;
        var_t1_dn11 = assign34620_e57731_d_n11;
        var_t1_dn13 = assign34620_e57731_d_n13;
        var_t1_dn14 = assign34620_e57731_d_n14;

        let (assign34630_e57745, assign34630_e57745_d_n0, assign34630_e57745_d_n2, assign34630_e57745_d_n3, assign34630_e57745_d_n4, assign34630_e57745_d_n5, assign34630_e57745_d_n6, assign34630_e57745_d_n7, assign34630_e57745_d_n8, assign34630_e57745_d_n9, assign34630_e57745_d_n10, assign34630_e57745_d_n11, assign34630_e57745_d_n13, assign34630_e57745_d_n14,) = {
    if (((var_guard657 != 0.0) && (var_guard661 != 0.0)) && (var_guard662 != 0.0)) {
        let assign34630_e57741: f64 = (p.p1905 * var_t1);
        let assign34630_e57742: f64 = (1.0 + assign34630_e57741);
        let assign34630_e57743: f64 = (var_vsatrsd_t * assign34630_e57742);
        (assign34630_e57743, (var_vsatrsd_t * (p.p1905 * var_t1_dn0)), (var_vsatrsd_t * (p.p1905 * var_t1_dn2)), (var_vsatrsd_t * (p.p1905 * var_t1_dn3)), ((var_vsatrsd_t_dn4 * assign34630_e57742) + (var_vsatrsd_t * (p.p1905 * var_t1_dn4))), (var_vsatrsd_t * (p.p1905 * var_t1_dn5)), (var_vsatrsd_t * (p.p1905 * var_t1_dn6)), (var_vsatrsd_t * (p.p1905 * var_t1_dn7)), (var_vsatrsd_t * (p.p1905 * var_t1_dn8)), (var_vsatrsd_t * (p.p1905 * var_t1_dn9)), (var_vsatrsd_t * (p.p1905 * var_t1_dn10)), (var_vsatrsd_t * (p.p1905 * var_t1_dn11)), (var_vsatrsd_t * (p.p1905 * var_t1_dn13)), (var_vsatrsd_t * (p.p1905 * var_t1_dn14)),)
    } else {
        (var_vsatrsd_eff, var_vsatrsd_eff_dn0, var_vsatrsd_eff_dn2, var_vsatrsd_eff_dn3, var_vsatrsd_eff_dn4, var_vsatrsd_eff_dn5, var_vsatrsd_eff_dn6, var_vsatrsd_eff_dn7, var_vsatrsd_eff_dn8, var_vsatrsd_eff_dn9, var_vsatrsd_eff_dn10, var_vsatrsd_eff_dn11, var_vsatrsd_eff_dn13, var_vsatrsd_eff_dn14,)
    }
};
        var_vsatrsd_eff = assign34630_e57745;
        var_vsatrsd_eff_dn0 = assign34630_e57745_d_n0;
        var_vsatrsd_eff_dn2 = assign34630_e57745_d_n2;
        var_vsatrsd_eff_dn3 = assign34630_e57745_d_n3;
        var_vsatrsd_eff_dn4 = assign34630_e57745_d_n4;
        var_vsatrsd_eff_dn5 = assign34630_e57745_d_n5;
        var_vsatrsd_eff_dn6 = assign34630_e57745_d_n6;
        var_vsatrsd_eff_dn7 = assign34630_e57745_d_n7;
        var_vsatrsd_eff_dn8 = assign34630_e57745_d_n8;
        var_vsatrsd_eff_dn9 = assign34630_e57745_d_n9;
        var_vsatrsd_eff_dn10 = assign34630_e57745_d_n10;
        var_vsatrsd_eff_dn11 = assign34630_e57745_d_n11;
        var_vsatrsd_eff_dn13 = assign34630_e57745_d_n13;
        var_vsatrsd_eff_dn14 = assign34630_e57745_d_n14;

        let (assign34640_e57788, assign34640_e57788_d_n0, assign34640_e57788_d_n2, assign34640_e57788_d_n3, assign34640_e57788_d_n4, assign34640_e57788_d_n5, assign34640_e57788_d_n6, assign34640_e57788_d_n7, assign34640_e57788_d_n8, assign34640_e57788_d_n9, assign34640_e57788_d_n10, assign34640_e57788_d_n11, assign34640_e57788_d_n13, assign34640_e57788_d_n14,) = {
    if (((var_guard657 != 0.0) && (var_guard661 != 0.0)) && (var_guard662 != 0.0)) {
        let assign34640_e57753: f64 = (-10000.0);
        let assign34640_e57755: f64 = (assign34640_e57753 * 10.0);
        let (assign34640_e57786, assign34640_e57786_d_n0, assign34640_e57786_d_n2, assign34640_e57786_d_n3, assign34640_e57786_d_n4, assign34640_e57786_d_n5, assign34640_e57786_d_n6, assign34640_e57786_d_n7, assign34640_e57786_d_n8, assign34640_e57786_d_n9, assign34640_e57786_d_n10, assign34640_e57786_d_n11, assign34640_e57786_d_n13, assign34640_e57786_d_n14,) = {
            if (!(var_vsatrsd_eff < assign34640_e57755)) {
                let assign34640_e57762: f64 = (var_vsatrsd_eff * var_vsatrsd_eff);
                let assign34640_e57765: f64 = (4.0 * 10.0);
                let assign34640_e57767: f64 = (assign34640_e57765 * 10.0);
                let assign34640_e57768: f64 = (assign34640_e57762 + assign34640_e57767);
                let assign34640_e57769: f64 = (assign34640_e57768).sqrt();
                let assign34640_e57770: f64 = (var_vsatrsd_eff + assign34640_e57769);
                let assign34640_e57771: f64 = (0.5 * assign34640_e57770);
                (assign34640_e57771, (0.5 * (var_vsatrsd_eff_dn0 + (((var_vsatrsd_eff_dn0 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn0)) / (2.0 * assign34640_e57769)))), (0.5 * (var_vsatrsd_eff_dn2 + (((var_vsatrsd_eff_dn2 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn2)) / (2.0 * assign34640_e57769)))), (0.5 * (var_vsatrsd_eff_dn3 + (((var_vsatrsd_eff_dn3 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn3)) / (2.0 * assign34640_e57769)))), (0.5 * (var_vsatrsd_eff_dn4 + (((var_vsatrsd_eff_dn4 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn4)) / (2.0 * assign34640_e57769)))), (0.5 * (var_vsatrsd_eff_dn5 + (((var_vsatrsd_eff_dn5 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn5)) / (2.0 * assign34640_e57769)))), (0.5 * (var_vsatrsd_eff_dn6 + (((var_vsatrsd_eff_dn6 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn6)) / (2.0 * assign34640_e57769)))), (0.5 * (var_vsatrsd_eff_dn7 + (((var_vsatrsd_eff_dn7 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn7)) / (2.0 * assign34640_e57769)))), (0.5 * (var_vsatrsd_eff_dn8 + (((var_vsatrsd_eff_dn8 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn8)) / (2.0 * assign34640_e57769)))), (0.5 * (var_vsatrsd_eff_dn9 + (((var_vsatrsd_eff_dn9 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn9)) / (2.0 * assign34640_e57769)))), (0.5 * (var_vsatrsd_eff_dn10 + (((var_vsatrsd_eff_dn10 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn10)) / (2.0 * assign34640_e57769)))), (0.5 * (var_vsatrsd_eff_dn11 + (((var_vsatrsd_eff_dn11 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn11)) / (2.0 * assign34640_e57769)))), (0.5 * (var_vsatrsd_eff_dn13 + (((var_vsatrsd_eff_dn13 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn13)) / (2.0 * assign34640_e57769)))), (0.5 * (var_vsatrsd_eff_dn14 + (((var_vsatrsd_eff_dn14 * var_vsatrsd_eff) + (var_vsatrsd_eff * var_vsatrsd_eff_dn14)) / (2.0 * assign34640_e57769)))),)
            } else {
                let assign34640_e57774: f64 = (-10000.0);
                let assign34640_e57776: f64 = (assign34640_e57774 * 10.0);
                let (assign34640_e57785, assign34640_e57785_d_n0, assign34640_e57785_d_n2, assign34640_e57785_d_n3, assign34640_e57785_d_n4, assign34640_e57785_d_n5, assign34640_e57785_d_n6, assign34640_e57785_d_n7, assign34640_e57785_d_n8, assign34640_e57785_d_n9, assign34640_e57785_d_n10, assign34640_e57785_d_n11, assign34640_e57785_d_n13, assign34640_e57785_d_n14,) = {
                    if (var_vsatrsd_eff < assign34640_e57776) {
                        let assign34640_e57779: f64 = (-10.0);
                        let assign34640_e57781: f64 = (assign34640_e57779 * 10.0);
                        let assign34640_e57783: f64 = (assign34640_e57781 / var_vsatrsd_eff);
                        (assign34640_e57783, (-((assign34640_e57781 * var_vsatrsd_eff_dn0) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34640_e57781 * var_vsatrsd_eff_dn2) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34640_e57781 * var_vsatrsd_eff_dn3) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34640_e57781 * var_vsatrsd_eff_dn4) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34640_e57781 * var_vsatrsd_eff_dn5) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34640_e57781 * var_vsatrsd_eff_dn6) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34640_e57781 * var_vsatrsd_eff_dn7) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34640_e57781 * var_vsatrsd_eff_dn8) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34640_e57781 * var_vsatrsd_eff_dn9) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34640_e57781 * var_vsatrsd_eff_dn10) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34640_e57781 * var_vsatrsd_eff_dn11) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34640_e57781 * var_vsatrsd_eff_dn13) / (var_vsatrsd_eff * var_vsatrsd_eff))), (-((assign34640_e57781 * var_vsatrsd_eff_dn14) / (var_vsatrsd_eff * var_vsatrsd_eff))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign34640_e57785, assign34640_e57785_d_n0, assign34640_e57785_d_n2, assign34640_e57785_d_n3, assign34640_e57785_d_n4, assign34640_e57785_d_n5, assign34640_e57785_d_n6, assign34640_e57785_d_n7, assign34640_e57785_d_n8, assign34640_e57785_d_n9, assign34640_e57785_d_n10, assign34640_e57785_d_n11, assign34640_e57785_d_n13, assign34640_e57785_d_n14,)
            }
        };
        (assign34640_e57786, assign34640_e57786_d_n0, assign34640_e57786_d_n2, assign34640_e57786_d_n3, assign34640_e57786_d_n4, assign34640_e57786_d_n5, assign34640_e57786_d_n6, assign34640_e57786_d_n7, assign34640_e57786_d_n8, assign34640_e57786_d_n9, assign34640_e57786_d_n10, assign34640_e57786_d_n11, assign34640_e57786_d_n13, assign34640_e57786_d_n14,)
    } else {
        (var_vsatrsd_eff, var_vsatrsd_eff_dn0, var_vsatrsd_eff_dn2, var_vsatrsd_eff_dn3, var_vsatrsd_eff_dn4, var_vsatrsd_eff_dn5, var_vsatrsd_eff_dn6, var_vsatrsd_eff_dn7, var_vsatrsd_eff_dn8, var_vsatrsd_eff_dn9, var_vsatrsd_eff_dn10, var_vsatrsd_eff_dn11, var_vsatrsd_eff_dn13, var_vsatrsd_eff_dn14,)
    }
};
        var_vsatrsd_eff = assign34640_e57788;
        var_vsatrsd_eff_dn0 = assign34640_e57788_d_n0;
        var_vsatrsd_eff_dn2 = assign34640_e57788_d_n2;
        var_vsatrsd_eff_dn3 = assign34640_e57788_d_n3;
        var_vsatrsd_eff_dn4 = assign34640_e57788_d_n4;
        var_vsatrsd_eff_dn5 = assign34640_e57788_d_n5;
        var_vsatrsd_eff_dn6 = assign34640_e57788_d_n6;
        var_vsatrsd_eff_dn7 = assign34640_e57788_d_n7;
        var_vsatrsd_eff_dn8 = assign34640_e57788_d_n8;
        var_vsatrsd_eff_dn9 = assign34640_e57788_d_n9;
        var_vsatrsd_eff_dn10 = assign34640_e57788_d_n10;
        var_vsatrsd_eff_dn11 = assign34640_e57788_d_n11;
        var_vsatrsd_eff_dn13 = assign34640_e57788_d_n13;
        var_vsatrsd_eff_dn14 = assign34640_e57788_d_n14;

        let (assign34650_e57802, assign34650_e57802_d_n0, assign34650_e57802_d_n2, assign34650_e57802_d_n3, assign34650_e57802_d_n4, assign34650_e57802_d_n5, assign34650_e57802_d_n6, assign34650_e57802_d_n7, assign34650_e57802_d_n8, assign34650_e57802_d_n9, assign34650_e57802_d_n10, assign34650_e57802_d_n11, assign34650_e57802_d_n13, assign34650_e57802_d_n14,) = {
    if (((var_guard657 != 0.0) && (var_guard661 != 0.0)) && (var_guard662 != 0.0)) {
        let assign34650_e57796: f64 = (var_nfintotal * var_weff0);
        let assign34650_e57798: f64 = (assign34650_e57796 * 1.60219e-19);
        let assign34650_e57800: f64 = (assign34650_e57798 * var_vsatrsd_eff);
        (assign34650_e57800, (assign34650_e57798 * var_vsatrsd_eff_dn0), (assign34650_e57798 * var_vsatrsd_eff_dn2), (assign34650_e57798 * var_vsatrsd_eff_dn3), (assign34650_e57798 * var_vsatrsd_eff_dn4), (assign34650_e57798 * var_vsatrsd_eff_dn5), (assign34650_e57798 * var_vsatrsd_eff_dn6), (assign34650_e57798 * var_vsatrsd_eff_dn7), (assign34650_e57798 * var_vsatrsd_eff_dn8), (assign34650_e57798 * var_vsatrsd_eff_dn9), (assign34650_e57798 * var_vsatrsd_eff_dn10), (assign34650_e57798 * var_vsatrsd_eff_dn11), (assign34650_e57798 * var_vsatrsd_eff_dn13), (assign34650_e57798 * var_vsatrsd_eff_dn14),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn13, var_t2_dn14,)
    }
};
        var_t2 = assign34650_e57802;
        var_t2_dn0 = assign34650_e57802_d_n0;
        var_t2_dn2 = assign34650_e57802_d_n2;
        var_t2_dn3 = assign34650_e57802_d_n3;
        var_t2_dn4 = assign34650_e57802_d_n4;
        var_t2_dn5 = assign34650_e57802_d_n5;
        var_t2_dn6 = assign34650_e57802_d_n6;
        var_t2_dn7 = assign34650_e57802_d_n7;
        var_t2_dn8 = assign34650_e57802_d_n8;
        var_t2_dn9 = assign34650_e57802_d_n9;
        var_t2_dn10 = assign34650_e57802_d_n10;
        var_t2_dn11 = assign34650_e57802_d_n11;
        var_t2_dn13 = assign34650_e57802_d_n13;
        var_t2_dn14 = assign34650_e57802_d_n14;

        let (assign34660_e57810, assign34660_e57810_d_n0, assign34660_e57810_d_n2, assign34660_e57810_d_n3, assign34660_e57810_d_n4, assign34660_e57810_d_n5, assign34660_e57810_d_n6, assign34660_e57810_d_n7, assign34660_e57810_d_n8, assign34660_e57810_d_n9, assign34660_e57810_d_n10, assign34660_e57810_d_n11, assign34660_e57810_d_n13, assign34660_e57810_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard661 != 0.0)) {
        let assign34660_e57808: f64 = (var_t2 * p.p1909);
        (assign34660_e57808, (var_t2_dn0 * p.p1909), (var_t2_dn2 * p.p1909), (var_t2_dn3 * p.p1909), (var_t2_dn4 * p.p1909), (var_t2_dn5 * p.p1909), (var_t2_dn6 * p.p1909), (var_t2_dn7 * p.p1909), (var_t2_dn8 * p.p1909), (var_t2_dn9 * p.p1909), (var_t2_dn10 * p.p1909), (var_t2_dn11 * p.p1909), (var_t2_dn13 * p.p1909), (var_t2_dn14 * p.p1909),)
    } else {
        (var_isat_rs, var_isat_rs_dn0, var_isat_rs_dn2, var_isat_rs_dn3, var_isat_rs_dn4, var_isat_rs_dn5, var_isat_rs_dn6, var_isat_rs_dn7, var_isat_rs_dn8, var_isat_rs_dn9, var_isat_rs_dn10, var_isat_rs_dn11, var_isat_rs_dn13, var_isat_rs_dn14,)
    }
};
        var_isat_rs = assign34660_e57810;
        var_isat_rs_dn0 = assign34660_e57810_d_n0;
        var_isat_rs_dn2 = assign34660_e57810_d_n2;
        var_isat_rs_dn3 = assign34660_e57810_d_n3;
        var_isat_rs_dn4 = assign34660_e57810_d_n4;
        var_isat_rs_dn5 = assign34660_e57810_d_n5;
        var_isat_rs_dn6 = assign34660_e57810_d_n6;
        var_isat_rs_dn7 = assign34660_e57810_d_n7;
        var_isat_rs_dn8 = assign34660_e57810_d_n8;
        var_isat_rs_dn9 = assign34660_e57810_d_n9;
        var_isat_rs_dn10 = assign34660_e57810_d_n10;
        var_isat_rs_dn11 = assign34660_e57810_d_n11;
        var_isat_rs_dn13 = assign34660_e57810_d_n13;
        var_isat_rs_dn14 = assign34660_e57810_d_n14;

        let (assign34670_e57820, assign34670_e57820_d_n0, assign34670_e57820_d_n2, assign34670_e57820_d_n3, assign34670_e57820_d_n4, assign34670_e57820_d_n5, assign34670_e57820_d_n6, assign34670_e57820_d_n7, assign34670_e57820_d_n8, assign34670_e57820_d_n9, assign34670_e57820_d_n10, assign34670_e57820_d_n11, assign34670_e57820_d_n13, assign34670_e57820_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard661 != 0.0)) {
        let assign34670_e57816: f64 = (var_rdstempvs * p.p1911);
        let assign34670_e57818: f64 = (assign34670_e57816 * var_weffwrfactor);
        (assign34670_e57818, 0.0, 0.0, 0.0, ((var_rdstempvs_dn4 * p.p1911) * var_weffwrfactor), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11, var_t4_dn13, var_t4_dn14,)
    }
};
        var_t4 = assign34670_e57820;
        var_t4_dn0 = assign34670_e57820_d_n0;
        var_t4_dn2 = assign34670_e57820_d_n2;
        var_t4_dn3 = assign34670_e57820_d_n3;
        var_t4_dn4 = assign34670_e57820_d_n4;
        var_t4_dn5 = assign34670_e57820_d_n5;
        var_t4_dn6 = assign34670_e57820_d_n6;
        var_t4_dn7 = assign34670_e57820_d_n7;
        var_t4_dn8 = assign34670_e57820_d_n8;
        var_t4_dn9 = assign34670_e57820_d_n9;
        var_t4_dn10 = assign34670_e57820_d_n10;
        var_t4_dn11 = assign34670_e57820_d_n11;
        var_t4_dn13 = assign34670_e57820_d_n13;
        var_t4_dn14 = assign34670_e57820_d_n14;

        *var_delta_vsrd_slot = var_delta_vsrd;
        *var_delta_vsrd_dn0_slot = var_delta_vsrd_dn0;
        *var_delta_vsrd_dn10_slot = var_delta_vsrd_dn10;
        *var_delta_vsrd_dn11_slot = var_delta_vsrd_dn11;
        *var_delta_vsrd_dn13_slot = var_delta_vsrd_dn13;
        *var_delta_vsrd_dn14_slot = var_delta_vsrd_dn14;
        *var_delta_vsrd_dn2_slot = var_delta_vsrd_dn2;
        *var_delta_vsrd_dn3_slot = var_delta_vsrd_dn3;
        *var_delta_vsrd_dn4_slot = var_delta_vsrd_dn4;
        *var_delta_vsrd_dn5_slot = var_delta_vsrd_dn5;
        *var_delta_vsrd_dn6_slot = var_delta_vsrd_dn6;
        *var_delta_vsrd_dn7_slot = var_delta_vsrd_dn7;
        *var_delta_vsrd_dn8_slot = var_delta_vsrd_dn8;
        *var_delta_vsrd_dn9_slot = var_delta_vsrd_dn9;
        *var_guard661_slot = var_guard661;
        *var_guard662_slot = var_guard662;
        *var_guard663_slot = var_guard663;
        *var_isat_rs_slot = var_isat_rs;
        *var_isat_rs_dn0_slot = var_isat_rs_dn0;
        *var_isat_rs_dn10_slot = var_isat_rs_dn10;
        *var_isat_rs_dn11_slot = var_isat_rs_dn11;
        *var_isat_rs_dn13_slot = var_isat_rs_dn13;
        *var_isat_rs_dn14_slot = var_isat_rs_dn14;
        *var_isat_rs_dn2_slot = var_isat_rs_dn2;
        *var_isat_rs_dn3_slot = var_isat_rs_dn3;
        *var_isat_rs_dn4_slot = var_isat_rs_dn4;
        *var_isat_rs_dn5_slot = var_isat_rs_dn5;
        *var_isat_rs_dn6_slot = var_isat_rs_dn6;
        *var_isat_rs_dn7_slot = var_isat_rs_dn7;
        *var_isat_rs_dn8_slot = var_isat_rs_dn8;
        *var_isat_rs_dn9_slot = var_isat_rs_dn9;
        *var_rdstempvs_slot = var_rdstempvs;
        *var_rdstempvs_dn4_slot = var_rdstempvs_dn4;
        *var_rvs_d_slot = var_rvs_d;
        *var_rvs_d_dn0_slot = var_rvs_d_dn0;
        *var_rvs_d_dn10_slot = var_rvs_d_dn10;
        *var_rvs_d_dn11_slot = var_rvs_d_dn11;
        *var_rvs_d_dn13_slot = var_rvs_d_dn13;
        *var_rvs_d_dn14_slot = var_rvs_d_dn14;
        *var_rvs_d_dn2_slot = var_rvs_d_dn2;
        *var_rvs_d_dn3_slot = var_rvs_d_dn3;
        *var_rvs_d_dn4_slot = var_rvs_d_dn4;
        *var_rvs_d_dn5_slot = var_rvs_d_dn5;
        *var_rvs_d_dn6_slot = var_rvs_d_dn6;
        *var_rvs_d_dn7_slot = var_rvs_d_dn7;
        *var_rvs_d_dn8_slot = var_rvs_d_dn8;
        *var_rvs_d_dn9_slot = var_rvs_d_dn9;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
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
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
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
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn14_slot = var_t4_dn14;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn13_slot = var_t6_dn13;
        *var_t6_dn14_slot = var_t6_dn14;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_vsat_rd_slot = var_vsat_rd;
        *var_vsat_rd_dn0_slot = var_vsat_rd_dn0;
        *var_vsat_rd_dn10_slot = var_vsat_rd_dn10;
        *var_vsat_rd_dn11_slot = var_vsat_rd_dn11;
        *var_vsat_rd_dn13_slot = var_vsat_rd_dn13;
        *var_vsat_rd_dn14_slot = var_vsat_rd_dn14;
        *var_vsat_rd_dn2_slot = var_vsat_rd_dn2;
        *var_vsat_rd_dn3_slot = var_vsat_rd_dn3;
        *var_vsat_rd_dn4_slot = var_vsat_rd_dn4;
        *var_vsat_rd_dn5_slot = var_vsat_rd_dn5;
        *var_vsat_rd_dn6_slot = var_vsat_rd_dn6;
        *var_vsat_rd_dn7_slot = var_vsat_rd_dn7;
        *var_vsat_rd_dn8_slot = var_vsat_rd_dn8;
        *var_vsat_rd_dn9_slot = var_vsat_rd_dn9;
        *var_vsatrsd_eff_slot = var_vsatrsd_eff;
        *var_vsatrsd_eff_dn0_slot = var_vsatrsd_eff_dn0;
        *var_vsatrsd_eff_dn10_slot = var_vsatrsd_eff_dn10;
        *var_vsatrsd_eff_dn11_slot = var_vsatrsd_eff_dn11;
        *var_vsatrsd_eff_dn13_slot = var_vsatrsd_eff_dn13;
        *var_vsatrsd_eff_dn14_slot = var_vsatrsd_eff_dn14;
        *var_vsatrsd_eff_dn2_slot = var_vsatrsd_eff_dn2;
        *var_vsatrsd_eff_dn3_slot = var_vsatrsd_eff_dn3;
        *var_vsatrsd_eff_dn4_slot = var_vsatrsd_eff_dn4;
        *var_vsatrsd_eff_dn5_slot = var_vsatrsd_eff_dn5;
        *var_vsatrsd_eff_dn6_slot = var_vsatrsd_eff_dn6;
        *var_vsatrsd_eff_dn7_slot = var_vsatrsd_eff_dn7;
        *var_vsatrsd_eff_dn8_slot = var_vsatrsd_eff_dn8;
        *var_vsatrsd_eff_dn9_slot = var_vsatrsd_eff_dn9;
        *var_vsatrsd_t_slot = var_vsatrsd_t;
        *var_vsatrsd_t_dn4_slot = var_vsatrsd_t_dn4;
    }

    pub(super) fn stamp_transient_block_134(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_cox: f64,
        var_epssub: f64,
        var_guard657: f64,
        var_guard661: f64,
        var_isat_rs: f64,
        var_isat_rs_dn0: f64,
        var_isat_rs_dn10: f64,
        var_isat_rs_dn11: f64,
        var_isat_rs_dn13: f64,
        var_isat_rs_dn14: f64,
        var_isat_rs_dn2: f64,
        var_isat_rs_dn3: f64,
        var_isat_rs_dn4: f64,
        var_isat_rs_dn5: f64,
        var_isat_rs_dn6: f64,
        var_isat_rs_dn7: f64,
        var_isat_rs_dn8: f64,
        var_isat_rs_dn9: f64,
        var_ni: f64,
        var_ni_dn0: f64,
        var_ni_dn10: f64,
        var_ni_dn11: f64,
        var_ni_dn13: f64,
        var_ni_dn14: f64,
        var_ni_dn2: f64,
        var_ni_dn3: f64,
        var_ni_dn4: f64,
        var_ni_dn5: f64,
        var_ni_dn6: f64,
        var_ni_dn7: f64,
        var_ni_dn8: f64,
        var_ni_dn9: f64,
        var_qbs: f64,
        var_rc: f64,
        var_rdrain: f64,
        var_rdrain_dn0: f64,
        var_rdrain_dn10: f64,
        var_rdrain_dn11: f64,
        var_rdrain_dn13: f64,
        var_rdrain_dn14: f64,
        var_rdrain_dn2: f64,
        var_rdrain_dn3: f64,
        var_rdrain_dn4: f64,
        var_rdrain_dn5: f64,
        var_rdrain_dn6: f64,
        var_rdrain_dn7: f64,
        var_rdrain_dn8: f64,
        var_rdrain_dn9: f64,
        var_rdraingeo: f64,
        var_rsource: f64,
        var_rsource_dn0: f64,
        var_rsource_dn10: f64,
        var_rsource_dn11: f64,
        var_rsource_dn13: f64,
        var_rsource_dn14: f64,
        var_rsource_dn2: f64,
        var_rsource_dn3: f64,
        var_rsource_dn4: f64,
        var_rsource_dn5: f64,
        var_rsource_dn6: f64,
        var_rsource_dn7: f64,
        var_rsource_dn8: f64,
        var_rsource_dn9: f64,
        var_rsourcegeo: f64,
        var_rvs_d: f64,
        var_rvs_d_dn0: f64,
        var_rvs_d_dn10: f64,
        var_rvs_d_dn11: f64,
        var_rvs_d_dn13: f64,
        var_rvs_d_dn14: f64,
        var_rvs_d_dn2: f64,
        var_rvs_d_dn3: f64,
        var_rvs_d_dn4: f64,
        var_rvs_d_dn5: f64,
        var_rvs_d_dn6: f64,
        var_rvs_d_dn7: f64,
        var_rvs_d_dn8: f64,
        var_rvs_d_dn9: f64,
        var_t4: f64,
        var_t4_dn0: f64,
        var_t4_dn10: f64,
        var_t4_dn11: f64,
        var_t4_dn13: f64,
        var_t4_dn14: f64,
        var_t4_dn2: f64,
        var_t4_dn3: f64,
        var_t4_dn4: f64,
        var_t4_dn5: f64,
        var_t4_dn6: f64,
        var_t4_dn7: f64,
        var_t4_dn8: f64,
        var_t4_dn9: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_xrcrg1_i: f64,
        var_delta_vsrs_slot: &mut f64,
        var_delta_vsrs_dn0_slot: &mut f64,
        var_delta_vsrs_dn10_slot: &mut f64,
        var_delta_vsrs_dn11_slot: &mut f64,
        var_delta_vsrs_dn13_slot: &mut f64,
        var_delta_vsrs_dn14_slot: &mut f64,
        var_delta_vsrs_dn2_slot: &mut f64,
        var_delta_vsrs_dn3_slot: &mut f64,
        var_delta_vsrs_dn4_slot: &mut f64,
        var_delta_vsrs_dn5_slot: &mut f64,
        var_delta_vsrs_dn6_slot: &mut f64,
        var_delta_vsrs_dn7_slot: &mut f64,
        var_delta_vsrs_dn8_slot: &mut f64,
        var_delta_vsrs_dn9_slot: &mut f64,
        var_gdpr_slot: &mut f64,
        var_gdpr_dn0_slot: &mut f64,
        var_gdpr_dn10_slot: &mut f64,
        var_gdpr_dn11_slot: &mut f64,
        var_gdpr_dn13_slot: &mut f64,
        var_gdpr_dn14_slot: &mut f64,
        var_gdpr_dn2_slot: &mut f64,
        var_gdpr_dn3_slot: &mut f64,
        var_gdpr_dn4_slot: &mut f64,
        var_gdpr_dn5_slot: &mut f64,
        var_gdpr_dn6_slot: &mut f64,
        var_gdpr_dn7_slot: &mut f64,
        var_gdpr_dn8_slot: &mut f64,
        var_gdpr_dn9_slot: &mut f64,
        var_gspr_slot: &mut f64,
        var_gspr_dn0_slot: &mut f64,
        var_gspr_dn10_slot: &mut f64,
        var_gspr_dn11_slot: &mut f64,
        var_gspr_dn13_slot: &mut f64,
        var_gspr_dn14_slot: &mut f64,
        var_gspr_dn2_slot: &mut f64,
        var_gspr_dn3_slot: &mut f64,
        var_gspr_dn4_slot: &mut f64,
        var_gspr_dn5_slot: &mut f64,
        var_gspr_dn6_slot: &mut f64,
        var_gspr_dn7_slot: &mut f64,
        var_gspr_dn8_slot: &mut f64,
        var_gspr_dn9_slot: &mut f64,
        var_guard664_slot: &mut f64,
        var_guard665_slot: &mut f64,
        var_guard666_slot: &mut f64,
        var_guard667_slot: &mut f64,
        var_guard668_slot: &mut f64,
        var_guard669_slot: &mut f64,
        var_guard677_slot: &mut f64,
        var_guard682_slot: &mut f64,
        var_guard683_slot: &mut f64,
        var_guard684_slot: &mut f64,
        var_guard685_slot: &mut f64,
        var_guard686_slot: &mut f64,
        var_gvs_d_slot: &mut f64,
        var_gvs_d_dn0_slot: &mut f64,
        var_gvs_d_dn10_slot: &mut f64,
        var_gvs_d_dn11_slot: &mut f64,
        var_gvs_d_dn13_slot: &mut f64,
        var_gvs_d_dn14_slot: &mut f64,
        var_gvs_d_dn2_slot: &mut f64,
        var_gvs_d_dn3_slot: &mut f64,
        var_gvs_d_dn4_slot: &mut f64,
        var_gvs_d_dn5_slot: &mut f64,
        var_gvs_d_dn6_slot: &mut f64,
        var_gvs_d_dn7_slot: &mut f64,
        var_gvs_d_dn8_slot: &mut f64,
        var_gvs_d_dn9_slot: &mut f64,
        var_gvs_s_slot: &mut f64,
        var_gvs_s_dn0_slot: &mut f64,
        var_gvs_s_dn10_slot: &mut f64,
        var_gvs_s_dn11_slot: &mut f64,
        var_gvs_s_dn13_slot: &mut f64,
        var_gvs_s_dn14_slot: &mut f64,
        var_gvs_s_dn2_slot: &mut f64,
        var_gvs_s_dn3_slot: &mut f64,
        var_gvs_s_dn4_slot: &mut f64,
        var_gvs_s_dn5_slot: &mut f64,
        var_gvs_s_dn6_slot: &mut f64,
        var_gvs_s_dn7_slot: &mut f64,
        var_gvs_s_dn8_slot: &mut f64,
        var_gvs_s_dn9_slot: &mut f64,
        var_q0_slot: &mut f64,
        var_q0_dn0_slot: &mut f64,
        var_q0_dn10_slot: &mut f64,
        var_q0_dn11_slot: &mut f64,
        var_q0_dn13_slot: &mut f64,
        var_q0_dn14_slot: &mut f64,
        var_q0_dn2_slot: &mut f64,
        var_q0_dn3_slot: &mut f64,
        var_q0_dn4_slot: &mut f64,
        var_q0_dn5_slot: &mut f64,
        var_q0_dn6_slot: &mut f64,
        var_q0_dn7_slot: &mut f64,
        var_q0_dn8_slot: &mut f64,
        var_q0_dn9_slot: &mut f64,
        var_rvs_s_slot: &mut f64,
        var_rvs_s_dn0_slot: &mut f64,
        var_rvs_s_dn10_slot: &mut f64,
        var_rvs_s_dn11_slot: &mut f64,
        var_rvs_s_dn13_slot: &mut f64,
        var_rvs_s_dn14_slot: &mut f64,
        var_rvs_s_dn2_slot: &mut f64,
        var_rvs_s_dn3_slot: &mut f64,
        var_rvs_s_dn4_slot: &mut f64,
        var_rvs_s_dn5_slot: &mut f64,
        var_rvs_s_dn6_slot: &mut f64,
        var_rvs_s_dn7_slot: &mut f64,
        var_rvs_s_dn8_slot: &mut f64,
        var_rvs_s_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
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
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
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
        var_t3_dn13_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn13_slot: &mut f64,
        var_t5_dn14_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
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
        var_t6_dn13_slot: &mut f64,
        var_t6_dn14_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_vsat_rs_slot: &mut f64,
        var_vsat_rs_dn0_slot: &mut f64,
        var_vsat_rs_dn10_slot: &mut f64,
        var_vsat_rs_dn11_slot: &mut f64,
        var_vsat_rs_dn13_slot: &mut f64,
        var_vsat_rs_dn14_slot: &mut f64,
        var_vsat_rs_dn2_slot: &mut f64,
        var_vsat_rs_dn3_slot: &mut f64,
        var_vsat_rs_dn4_slot: &mut f64,
        var_vsat_rs_dn5_slot: &mut f64,
        var_vsat_rs_dn6_slot: &mut f64,
        var_vsat_rs_dn7_slot: &mut f64,
        var_vsat_rs_dn8_slot: &mut f64,
        var_vsat_rs_dn9_slot: &mut f64,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let mut var_delta_vsrs: f64 = *var_delta_vsrs_slot;
        let mut var_delta_vsrs_dn0: f64 = *var_delta_vsrs_dn0_slot;
        let mut var_delta_vsrs_dn10: f64 = *var_delta_vsrs_dn10_slot;
        let mut var_delta_vsrs_dn11: f64 = *var_delta_vsrs_dn11_slot;
        let mut var_delta_vsrs_dn13: f64 = *var_delta_vsrs_dn13_slot;
        let mut var_delta_vsrs_dn14: f64 = *var_delta_vsrs_dn14_slot;
        let mut var_delta_vsrs_dn2: f64 = *var_delta_vsrs_dn2_slot;
        let mut var_delta_vsrs_dn3: f64 = *var_delta_vsrs_dn3_slot;
        let mut var_delta_vsrs_dn4: f64 = *var_delta_vsrs_dn4_slot;
        let mut var_delta_vsrs_dn5: f64 = *var_delta_vsrs_dn5_slot;
        let mut var_delta_vsrs_dn6: f64 = *var_delta_vsrs_dn6_slot;
        let mut var_delta_vsrs_dn7: f64 = *var_delta_vsrs_dn7_slot;
        let mut var_delta_vsrs_dn8: f64 = *var_delta_vsrs_dn8_slot;
        let mut var_delta_vsrs_dn9: f64 = *var_delta_vsrs_dn9_slot;
        let mut var_gdpr: f64 = *var_gdpr_slot;
        let mut var_gdpr_dn0: f64 = *var_gdpr_dn0_slot;
        let mut var_gdpr_dn10: f64 = *var_gdpr_dn10_slot;
        let mut var_gdpr_dn11: f64 = *var_gdpr_dn11_slot;
        let mut var_gdpr_dn13: f64 = *var_gdpr_dn13_slot;
        let mut var_gdpr_dn14: f64 = *var_gdpr_dn14_slot;
        let mut var_gdpr_dn2: f64 = *var_gdpr_dn2_slot;
        let mut var_gdpr_dn3: f64 = *var_gdpr_dn3_slot;
        let mut var_gdpr_dn4: f64 = *var_gdpr_dn4_slot;
        let mut var_gdpr_dn5: f64 = *var_gdpr_dn5_slot;
        let mut var_gdpr_dn6: f64 = *var_gdpr_dn6_slot;
        let mut var_gdpr_dn7: f64 = *var_gdpr_dn7_slot;
        let mut var_gdpr_dn8: f64 = *var_gdpr_dn8_slot;
        let mut var_gdpr_dn9: f64 = *var_gdpr_dn9_slot;
        let mut var_gspr: f64 = *var_gspr_slot;
        let mut var_gspr_dn0: f64 = *var_gspr_dn0_slot;
        let mut var_gspr_dn10: f64 = *var_gspr_dn10_slot;
        let mut var_gspr_dn11: f64 = *var_gspr_dn11_slot;
        let mut var_gspr_dn13: f64 = *var_gspr_dn13_slot;
        let mut var_gspr_dn14: f64 = *var_gspr_dn14_slot;
        let mut var_gspr_dn2: f64 = *var_gspr_dn2_slot;
        let mut var_gspr_dn3: f64 = *var_gspr_dn3_slot;
        let mut var_gspr_dn4: f64 = *var_gspr_dn4_slot;
        let mut var_gspr_dn5: f64 = *var_gspr_dn5_slot;
        let mut var_gspr_dn6: f64 = *var_gspr_dn6_slot;
        let mut var_gspr_dn7: f64 = *var_gspr_dn7_slot;
        let mut var_gspr_dn8: f64 = *var_gspr_dn8_slot;
        let mut var_gspr_dn9: f64 = *var_gspr_dn9_slot;
        let mut var_guard664: f64 = *var_guard664_slot;
        let mut var_guard665: f64 = *var_guard665_slot;
        let mut var_guard666: f64 = *var_guard666_slot;
        let mut var_guard667: f64 = *var_guard667_slot;
        let mut var_guard668: f64 = *var_guard668_slot;
        let mut var_guard669: f64 = *var_guard669_slot;
        let mut var_guard677: f64 = *var_guard677_slot;
        let mut var_guard682: f64 = *var_guard682_slot;
        let mut var_guard683: f64 = *var_guard683_slot;
        let mut var_guard684: f64 = *var_guard684_slot;
        let mut var_guard685: f64 = *var_guard685_slot;
        let mut var_guard686: f64 = *var_guard686_slot;
        let mut var_gvs_d: f64 = *var_gvs_d_slot;
        let mut var_gvs_d_dn0: f64 = *var_gvs_d_dn0_slot;
        let mut var_gvs_d_dn10: f64 = *var_gvs_d_dn10_slot;
        let mut var_gvs_d_dn11: f64 = *var_gvs_d_dn11_slot;
        let mut var_gvs_d_dn13: f64 = *var_gvs_d_dn13_slot;
        let mut var_gvs_d_dn14: f64 = *var_gvs_d_dn14_slot;
        let mut var_gvs_d_dn2: f64 = *var_gvs_d_dn2_slot;
        let mut var_gvs_d_dn3: f64 = *var_gvs_d_dn3_slot;
        let mut var_gvs_d_dn4: f64 = *var_gvs_d_dn4_slot;
        let mut var_gvs_d_dn5: f64 = *var_gvs_d_dn5_slot;
        let mut var_gvs_d_dn6: f64 = *var_gvs_d_dn6_slot;
        let mut var_gvs_d_dn7: f64 = *var_gvs_d_dn7_slot;
        let mut var_gvs_d_dn8: f64 = *var_gvs_d_dn8_slot;
        let mut var_gvs_d_dn9: f64 = *var_gvs_d_dn9_slot;
        let mut var_gvs_s: f64 = *var_gvs_s_slot;
        let mut var_gvs_s_dn0: f64 = *var_gvs_s_dn0_slot;
        let mut var_gvs_s_dn10: f64 = *var_gvs_s_dn10_slot;
        let mut var_gvs_s_dn11: f64 = *var_gvs_s_dn11_slot;
        let mut var_gvs_s_dn13: f64 = *var_gvs_s_dn13_slot;
        let mut var_gvs_s_dn14: f64 = *var_gvs_s_dn14_slot;
        let mut var_gvs_s_dn2: f64 = *var_gvs_s_dn2_slot;
        let mut var_gvs_s_dn3: f64 = *var_gvs_s_dn3_slot;
        let mut var_gvs_s_dn4: f64 = *var_gvs_s_dn4_slot;
        let mut var_gvs_s_dn5: f64 = *var_gvs_s_dn5_slot;
        let mut var_gvs_s_dn6: f64 = *var_gvs_s_dn6_slot;
        let mut var_gvs_s_dn7: f64 = *var_gvs_s_dn7_slot;
        let mut var_gvs_s_dn8: f64 = *var_gvs_s_dn8_slot;
        let mut var_gvs_s_dn9: f64 = *var_gvs_s_dn9_slot;
        let mut var_q0: f64 = *var_q0_slot;
        let mut var_q0_dn0: f64 = *var_q0_dn0_slot;
        let mut var_q0_dn10: f64 = *var_q0_dn10_slot;
        let mut var_q0_dn11: f64 = *var_q0_dn11_slot;
        let mut var_q0_dn13: f64 = *var_q0_dn13_slot;
        let mut var_q0_dn14: f64 = *var_q0_dn14_slot;
        let mut var_q0_dn2: f64 = *var_q0_dn2_slot;
        let mut var_q0_dn3: f64 = *var_q0_dn3_slot;
        let mut var_q0_dn4: f64 = *var_q0_dn4_slot;
        let mut var_q0_dn5: f64 = *var_q0_dn5_slot;
        let mut var_q0_dn6: f64 = *var_q0_dn6_slot;
        let mut var_q0_dn7: f64 = *var_q0_dn7_slot;
        let mut var_q0_dn8: f64 = *var_q0_dn8_slot;
        let mut var_q0_dn9: f64 = *var_q0_dn9_slot;
        let mut var_rvs_s: f64 = *var_rvs_s_slot;
        let mut var_rvs_s_dn0: f64 = *var_rvs_s_dn0_slot;
        let mut var_rvs_s_dn10: f64 = *var_rvs_s_dn10_slot;
        let mut var_rvs_s_dn11: f64 = *var_rvs_s_dn11_slot;
        let mut var_rvs_s_dn13: f64 = *var_rvs_s_dn13_slot;
        let mut var_rvs_s_dn14: f64 = *var_rvs_s_dn14_slot;
        let mut var_rvs_s_dn2: f64 = *var_rvs_s_dn2_slot;
        let mut var_rvs_s_dn3: f64 = *var_rvs_s_dn3_slot;
        let mut var_rvs_s_dn4: f64 = *var_rvs_s_dn4_slot;
        let mut var_rvs_s_dn5: f64 = *var_rvs_s_dn5_slot;
        let mut var_rvs_s_dn6: f64 = *var_rvs_s_dn6_slot;
        let mut var_rvs_s_dn7: f64 = *var_rvs_s_dn7_slot;
        let mut var_rvs_s_dn8: f64 = *var_rvs_s_dn8_slot;
        let mut var_rvs_s_dn9: f64 = *var_rvs_s_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
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
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
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
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn13: f64 = *var_t5_dn13_slot;
        let mut var_t5_dn14: f64 = *var_t5_dn14_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
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
        let mut var_t6_dn13: f64 = *var_t6_dn13_slot;
        let mut var_t6_dn14: f64 = *var_t6_dn14_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_vsat_rs: f64 = *var_vsat_rs_slot;
        let mut var_vsat_rs_dn0: f64 = *var_vsat_rs_dn0_slot;
        let mut var_vsat_rs_dn10: f64 = *var_vsat_rs_dn10_slot;
        let mut var_vsat_rs_dn11: f64 = *var_vsat_rs_dn11_slot;
        let mut var_vsat_rs_dn13: f64 = *var_vsat_rs_dn13_slot;
        let mut var_vsat_rs_dn14: f64 = *var_vsat_rs_dn14_slot;
        let mut var_vsat_rs_dn2: f64 = *var_vsat_rs_dn2_slot;
        let mut var_vsat_rs_dn3: f64 = *var_vsat_rs_dn3_slot;
        let mut var_vsat_rs_dn4: f64 = *var_vsat_rs_dn4_slot;
        let mut var_vsat_rs_dn5: f64 = *var_vsat_rs_dn5_slot;
        let mut var_vsat_rs_dn6: f64 = *var_vsat_rs_dn6_slot;
        let mut var_vsat_rs_dn7: f64 = *var_vsat_rs_dn7_slot;
        let mut var_vsat_rs_dn8: f64 = *var_vsat_rs_dn8_slot;
        let mut var_vsat_rs_dn9: f64 = *var_vsat_rs_dn9_slot;

        let (assign34680_e57828, assign34680_e57828_d_n0, assign34680_e57828_d_n2, assign34680_e57828_d_n3, assign34680_e57828_d_n4, assign34680_e57828_d_n5, assign34680_e57828_d_n6, assign34680_e57828_d_n7, assign34680_e57828_d_n8, assign34680_e57828_d_n9, assign34680_e57828_d_n10, assign34680_e57828_d_n11, assign34680_e57828_d_n13, assign34680_e57828_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard661 != 0.0)) {
        let assign34680_e57826: f64 = (var_isat_rs * var_t4);
        (assign34680_e57826, ((var_isat_rs_dn0 * var_t4) + (var_isat_rs * var_t4_dn0)), ((var_isat_rs_dn2 * var_t4) + (var_isat_rs * var_t4_dn2)), ((var_isat_rs_dn3 * var_t4) + (var_isat_rs * var_t4_dn3)), ((var_isat_rs_dn4 * var_t4) + (var_isat_rs * var_t4_dn4)), ((var_isat_rs_dn5 * var_t4) + (var_isat_rs * var_t4_dn5)), ((var_isat_rs_dn6 * var_t4) + (var_isat_rs * var_t4_dn6)), ((var_isat_rs_dn7 * var_t4) + (var_isat_rs * var_t4_dn7)), ((var_isat_rs_dn8 * var_t4) + (var_isat_rs * var_t4_dn8)), ((var_isat_rs_dn9 * var_t4) + (var_isat_rs * var_t4_dn9)), ((var_isat_rs_dn10 * var_t4) + (var_isat_rs * var_t4_dn10)), ((var_isat_rs_dn11 * var_t4) + (var_isat_rs * var_t4_dn11)), ((var_isat_rs_dn13 * var_t4) + (var_isat_rs * var_t4_dn13)), ((var_isat_rs_dn14 * var_t4) + (var_isat_rs * var_t4_dn14)),)
    } else {
        (var_vsat_rs, var_vsat_rs_dn0, var_vsat_rs_dn2, var_vsat_rs_dn3, var_vsat_rs_dn4, var_vsat_rs_dn5, var_vsat_rs_dn6, var_vsat_rs_dn7, var_vsat_rs_dn8, var_vsat_rs_dn9, var_vsat_rs_dn10, var_vsat_rs_dn11, var_vsat_rs_dn13, var_vsat_rs_dn14,)
    }
};
        var_vsat_rs = assign34680_e57828;
        var_vsat_rs_dn0 = assign34680_e57828_d_n0;
        var_vsat_rs_dn2 = assign34680_e57828_d_n2;
        var_vsat_rs_dn3 = assign34680_e57828_d_n3;
        var_vsat_rs_dn4 = assign34680_e57828_d_n4;
        var_vsat_rs_dn5 = assign34680_e57828_d_n5;
        var_vsat_rs_dn6 = assign34680_e57828_d_n6;
        var_vsat_rs_dn7 = assign34680_e57828_d_n7;
        var_vsat_rs_dn8 = assign34680_e57828_d_n8;
        var_vsat_rs_dn9 = assign34680_e57828_d_n9;
        var_vsat_rs_dn10 = assign34680_e57828_d_n10;
        var_vsat_rs_dn11 = assign34680_e57828_d_n11;
        var_vsat_rs_dn13 = assign34680_e57828_d_n13;
        var_vsat_rs_dn14 = assign34680_e57828_d_n14;

        let (assign34690_e57835, assign34690_e57835_d_n0, assign34690_e57835_d_n2, assign34690_e57835_d_n3, assign34690_e57835_d_n4, assign34690_e57835_d_n5, assign34690_e57835_d_n6, assign34690_e57835_d_n7, assign34690_e57835_d_n8, assign34690_e57835_d_n9, assign34690_e57835_d_n10, assign34690_e57835_d_n11, assign34690_e57835_d_n13, assign34690_e57835_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard661 != 0.0)) {
        let assign34690_e57833: f64 = ((nv6 - nv8)).abs();
        (assign34690_e57833, 0.0, 0.0, 0.0, 0.0, 0.0, if (nv6 - nv8) >= 0.0 { 1.0 } else { (-1.0) }, 0.0, if (nv6 - nv8) >= 0.0 { -1.0 } else { 1.0 }, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11, var_t5_dn13, var_t5_dn14,)
    }
};
        var_t5 = assign34690_e57835;
        var_t5_dn0 = assign34690_e57835_d_n0;
        var_t5_dn2 = assign34690_e57835_d_n2;
        var_t5_dn3 = assign34690_e57835_d_n3;
        var_t5_dn4 = assign34690_e57835_d_n4;
        var_t5_dn5 = assign34690_e57835_d_n5;
        var_t5_dn6 = assign34690_e57835_d_n6;
        var_t5_dn7 = assign34690_e57835_d_n7;
        var_t5_dn8 = assign34690_e57835_d_n8;
        var_t5_dn9 = assign34690_e57835_d_n9;
        var_t5_dn10 = assign34690_e57835_d_n10;
        var_t5_dn11 = assign34690_e57835_d_n11;
        var_t5_dn13 = assign34690_e57835_d_n13;
        var_t5_dn14 = assign34690_e57835_d_n14;

        let (assign34700_e57859, assign34700_e57859_d_n0, assign34700_e57859_d_n2, assign34700_e57859_d_n3, assign34700_e57859_d_n4, assign34700_e57859_d_n5, assign34700_e57859_d_n6, assign34700_e57859_d_n7, assign34700_e57859_d_n8, assign34700_e57859_d_n9, assign34700_e57859_d_n10, assign34700_e57859_d_n11, assign34700_e57859_d_n13, assign34700_e57859_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard661 != 0.0)) {
        let assign34700_e57842: f64 = (4.0 - p.p1908);
        let assign34700_e57843: f64 = (var_t5).powf(assign34700_e57842);
        let assign34700_e57847: f64 = (4.0 - p.p1908);
        let assign34700_e57848: f64 = (var_t5).powf(assign34700_e57847);
        let assign34700_e57853: f64 = (4.0 - p.p1908);
        let assign34700_e57854: f64 = (var_vsat_rs).powf(assign34700_e57853);
        let assign34700_e57855: f64 = (p.p1915 * assign34700_e57854);
        let assign34700_e57856: f64 = (assign34700_e57848 + assign34700_e57855);
        let assign34700_e57857: f64 = (assign34700_e57843 / assign34700_e57856);
        (assign34700_e57857, (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((var_t5).powf(assign34700_e57842 - 1.0) * var_t5_dn0)) } } else { (assign34700_e57843 * (assign34700_e57842 * (var_t5_dn0 / var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((var_t5).powf(assign34700_e57847 - 1.0) * var_t5_dn0)) } } else { (assign34700_e57848 * (assign34700_e57847 * (var_t5_dn0 / var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((var_vsat_rs).powf(assign34700_e57853 - 1.0) * var_vsat_rs_dn0)) } } else { (assign34700_e57854 * (assign34700_e57853 * (var_vsat_rs_dn0 / var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((var_t5).powf(assign34700_e57842 - 1.0) * var_t5_dn2)) } } else { (assign34700_e57843 * (assign34700_e57842 * (var_t5_dn2 / var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((var_t5).powf(assign34700_e57847 - 1.0) * var_t5_dn2)) } } else { (assign34700_e57848 * (assign34700_e57847 * (var_t5_dn2 / var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((var_vsat_rs).powf(assign34700_e57853 - 1.0) * var_vsat_rs_dn2)) } } else { (assign34700_e57854 * (assign34700_e57853 * (var_vsat_rs_dn2 / var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((var_t5).powf(assign34700_e57842 - 1.0) * var_t5_dn3)) } } else { (assign34700_e57843 * (assign34700_e57842 * (var_t5_dn3 / var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((var_t5).powf(assign34700_e57847 - 1.0) * var_t5_dn3)) } } else { (assign34700_e57848 * (assign34700_e57847 * (var_t5_dn3 / var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((var_vsat_rs).powf(assign34700_e57853 - 1.0) * var_vsat_rs_dn3)) } } else { (assign34700_e57854 * (assign34700_e57853 * (var_vsat_rs_dn3 / var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((var_t5).powf(assign34700_e57842 - 1.0) * var_t5_dn4)) } } else { (assign34700_e57843 * (assign34700_e57842 * (var_t5_dn4 / var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((var_t5).powf(assign34700_e57847 - 1.0) * var_t5_dn4)) } } else { (assign34700_e57848 * (assign34700_e57847 * (var_t5_dn4 / var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((var_vsat_rs).powf(assign34700_e57853 - 1.0) * var_vsat_rs_dn4)) } } else { (assign34700_e57854 * (assign34700_e57853 * (var_vsat_rs_dn4 / var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((var_t5).powf(assign34700_e57842 - 1.0) * var_t5_dn5)) } } else { (assign34700_e57843 * (assign34700_e57842 * (var_t5_dn5 / var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((var_t5).powf(assign34700_e57847 - 1.0) * var_t5_dn5)) } } else { (assign34700_e57848 * (assign34700_e57847 * (var_t5_dn5 / var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((var_vsat_rs).powf(assign34700_e57853 - 1.0) * var_vsat_rs_dn5)) } } else { (assign34700_e57854 * (assign34700_e57853 * (var_vsat_rs_dn5 / var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((var_t5).powf(assign34700_e57842 - 1.0) * var_t5_dn6)) } } else { (assign34700_e57843 * (assign34700_e57842 * (var_t5_dn6 / var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((var_t5).powf(assign34700_e57847 - 1.0) * var_t5_dn6)) } } else { (assign34700_e57848 * (assign34700_e57847 * (var_t5_dn6 / var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((var_vsat_rs).powf(assign34700_e57853 - 1.0) * var_vsat_rs_dn6)) } } else { (assign34700_e57854 * (assign34700_e57853 * (var_vsat_rs_dn6 / var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((var_t5).powf(assign34700_e57842 - 1.0) * var_t5_dn7)) } } else { (assign34700_e57843 * (assign34700_e57842 * (var_t5_dn7 / var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((var_t5).powf(assign34700_e57847 - 1.0) * var_t5_dn7)) } } else { (assign34700_e57848 * (assign34700_e57847 * (var_t5_dn7 / var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((var_vsat_rs).powf(assign34700_e57853 - 1.0) * var_vsat_rs_dn7)) } } else { (assign34700_e57854 * (assign34700_e57853 * (var_vsat_rs_dn7 / var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((var_t5).powf(assign34700_e57842 - 1.0) * var_t5_dn8)) } } else { (assign34700_e57843 * (assign34700_e57842 * (var_t5_dn8 / var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((var_t5).powf(assign34700_e57847 - 1.0) * var_t5_dn8)) } } else { (assign34700_e57848 * (assign34700_e57847 * (var_t5_dn8 / var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((var_vsat_rs).powf(assign34700_e57853 - 1.0) * var_vsat_rs_dn8)) } } else { (assign34700_e57854 * (assign34700_e57853 * (var_vsat_rs_dn8 / var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((var_t5).powf(assign34700_e57842 - 1.0) * var_t5_dn9)) } } else { (assign34700_e57843 * (assign34700_e57842 * (var_t5_dn9 / var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((var_t5).powf(assign34700_e57847 - 1.0) * var_t5_dn9)) } } else { (assign34700_e57848 * (assign34700_e57847 * (var_t5_dn9 / var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((var_vsat_rs).powf(assign34700_e57853 - 1.0) * var_vsat_rs_dn9)) } } else { (assign34700_e57854 * (assign34700_e57853 * (var_vsat_rs_dn9 / var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((var_t5).powf(assign34700_e57842 - 1.0) * var_t5_dn10)) } } else { (assign34700_e57843 * (assign34700_e57842 * (var_t5_dn10 / var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((var_t5).powf(assign34700_e57847 - 1.0) * var_t5_dn10)) } } else { (assign34700_e57848 * (assign34700_e57847 * (var_t5_dn10 / var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((var_vsat_rs).powf(assign34700_e57853 - 1.0) * var_vsat_rs_dn10)) } } else { (assign34700_e57854 * (assign34700_e57853 * (var_vsat_rs_dn10 / var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((var_t5).powf(assign34700_e57842 - 1.0) * var_t5_dn11)) } } else { (assign34700_e57843 * (assign34700_e57842 * (var_t5_dn11 / var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((var_t5).powf(assign34700_e57847 - 1.0) * var_t5_dn11)) } } else { (assign34700_e57848 * (assign34700_e57847 * (var_t5_dn11 / var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((var_vsat_rs).powf(assign34700_e57853 - 1.0) * var_vsat_rs_dn11)) } } else { (assign34700_e57854 * (assign34700_e57853 * (var_vsat_rs_dn11 / var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((var_t5).powf(assign34700_e57842 - 1.0) * var_t5_dn13)) } } else { (assign34700_e57843 * (assign34700_e57842 * (var_t5_dn13 / var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((var_t5).powf(assign34700_e57847 - 1.0) * var_t5_dn13)) } } else { (assign34700_e57848 * (assign34700_e57847 * (var_t5_dn13 / var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((var_vsat_rs).powf(assign34700_e57853 - 1.0) * var_vsat_rs_dn13)) } } else { (assign34700_e57854 * (assign34700_e57853 * (var_vsat_rs_dn13 / var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)), (((if 0.0 == 0.0 && ((assign34700_e57842) as f64).is_finite() && ((assign34700_e57842) as f64).fract() == 0.0 { if assign34700_e57842 == 0.0 { 0.0 } else { (assign34700_e57842 * ((var_t5).powf(assign34700_e57842 - 1.0) * var_t5_dn14)) } } else { (assign34700_e57843 * (assign34700_e57842 * (var_t5_dn14 / var_t5))) } * assign34700_e57856) - (assign34700_e57843 * (if 0.0 == 0.0 && ((assign34700_e57847) as f64).is_finite() && ((assign34700_e57847) as f64).fract() == 0.0 { if assign34700_e57847 == 0.0 { 0.0 } else { (assign34700_e57847 * ((var_t5).powf(assign34700_e57847 - 1.0) * var_t5_dn14)) } } else { (assign34700_e57848 * (assign34700_e57847 * (var_t5_dn14 / var_t5))) } + (p.p1915 * if 0.0 == 0.0 && ((assign34700_e57853) as f64).is_finite() && ((assign34700_e57853) as f64).fract() == 0.0 { if assign34700_e57853 == 0.0 { 0.0 } else { (assign34700_e57853 * ((var_vsat_rs).powf(assign34700_e57853 - 1.0) * var_vsat_rs_dn14)) } } else { (assign34700_e57854 * (assign34700_e57853 * (var_vsat_rs_dn14 / var_vsat_rs))) })))) / (assign34700_e57856 * assign34700_e57856)),)
    } else {
        (var_delta_vsrs, var_delta_vsrs_dn0, var_delta_vsrs_dn2, var_delta_vsrs_dn3, var_delta_vsrs_dn4, var_delta_vsrs_dn5, var_delta_vsrs_dn6, var_delta_vsrs_dn7, var_delta_vsrs_dn8, var_delta_vsrs_dn9, var_delta_vsrs_dn10, var_delta_vsrs_dn11, var_delta_vsrs_dn13, var_delta_vsrs_dn14,)
    }
};
        var_delta_vsrs = assign34700_e57859;
        var_delta_vsrs_dn0 = assign34700_e57859_d_n0;
        var_delta_vsrs_dn2 = assign34700_e57859_d_n2;
        var_delta_vsrs_dn3 = assign34700_e57859_d_n3;
        var_delta_vsrs_dn4 = assign34700_e57859_d_n4;
        var_delta_vsrs_dn5 = assign34700_e57859_d_n5;
        var_delta_vsrs_dn6 = assign34700_e57859_d_n6;
        var_delta_vsrs_dn7 = assign34700_e57859_d_n7;
        var_delta_vsrs_dn8 = assign34700_e57859_d_n8;
        var_delta_vsrs_dn9 = assign34700_e57859_d_n9;
        var_delta_vsrs_dn10 = assign34700_e57859_d_n10;
        var_delta_vsrs_dn11 = assign34700_e57859_d_n11;
        var_delta_vsrs_dn13 = assign34700_e57859_d_n13;
        var_delta_vsrs_dn14 = assign34700_e57859_d_n14;

        let (assign34710_e57873, assign34710_e57873_d_n0, assign34710_e57873_d_n2, assign34710_e57873_d_n3, assign34710_e57873_d_n4, assign34710_e57873_d_n5, assign34710_e57873_d_n6, assign34710_e57873_d_n7, assign34710_e57873_d_n8, assign34710_e57873_d_n9, assign34710_e57873_d_n10, assign34710_e57873_d_n11, assign34710_e57873_d_n13, assign34710_e57873_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard661 != 0.0)) {
        let assign34710_e57866: f64 = (1.0 / p.p1908);
        let assign34710_e57867: f64 = (var_delta_vsrs).powf(assign34710_e57866);
        let assign34710_e57869: f64 = (assign34710_e57867 * var_t5);
        let assign34710_e57871: f64 = (assign34710_e57869 / var_vsat_rs);
        (assign34710_e57871, (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((var_delta_vsrs).powf(assign34710_e57866 - 1.0) * var_delta_vsrs_dn0)) } } else { (assign34710_e57867 * (assign34710_e57866 * (var_delta_vsrs_dn0 / var_delta_vsrs))) } * var_t5) + (assign34710_e57867 * var_t5_dn0)) * var_vsat_rs) - (assign34710_e57869 * var_vsat_rs_dn0)) / (var_vsat_rs * var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((var_delta_vsrs).powf(assign34710_e57866 - 1.0) * var_delta_vsrs_dn2)) } } else { (assign34710_e57867 * (assign34710_e57866 * (var_delta_vsrs_dn2 / var_delta_vsrs))) } * var_t5) + (assign34710_e57867 * var_t5_dn2)) * var_vsat_rs) - (assign34710_e57869 * var_vsat_rs_dn2)) / (var_vsat_rs * var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((var_delta_vsrs).powf(assign34710_e57866 - 1.0) * var_delta_vsrs_dn3)) } } else { (assign34710_e57867 * (assign34710_e57866 * (var_delta_vsrs_dn3 / var_delta_vsrs))) } * var_t5) + (assign34710_e57867 * var_t5_dn3)) * var_vsat_rs) - (assign34710_e57869 * var_vsat_rs_dn3)) / (var_vsat_rs * var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((var_delta_vsrs).powf(assign34710_e57866 - 1.0) * var_delta_vsrs_dn4)) } } else { (assign34710_e57867 * (assign34710_e57866 * (var_delta_vsrs_dn4 / var_delta_vsrs))) } * var_t5) + (assign34710_e57867 * var_t5_dn4)) * var_vsat_rs) - (assign34710_e57869 * var_vsat_rs_dn4)) / (var_vsat_rs * var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((var_delta_vsrs).powf(assign34710_e57866 - 1.0) * var_delta_vsrs_dn5)) } } else { (assign34710_e57867 * (assign34710_e57866 * (var_delta_vsrs_dn5 / var_delta_vsrs))) } * var_t5) + (assign34710_e57867 * var_t5_dn5)) * var_vsat_rs) - (assign34710_e57869 * var_vsat_rs_dn5)) / (var_vsat_rs * var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((var_delta_vsrs).powf(assign34710_e57866 - 1.0) * var_delta_vsrs_dn6)) } } else { (assign34710_e57867 * (assign34710_e57866 * (var_delta_vsrs_dn6 / var_delta_vsrs))) } * var_t5) + (assign34710_e57867 * var_t5_dn6)) * var_vsat_rs) - (assign34710_e57869 * var_vsat_rs_dn6)) / (var_vsat_rs * var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((var_delta_vsrs).powf(assign34710_e57866 - 1.0) * var_delta_vsrs_dn7)) } } else { (assign34710_e57867 * (assign34710_e57866 * (var_delta_vsrs_dn7 / var_delta_vsrs))) } * var_t5) + (assign34710_e57867 * var_t5_dn7)) * var_vsat_rs) - (assign34710_e57869 * var_vsat_rs_dn7)) / (var_vsat_rs * var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((var_delta_vsrs).powf(assign34710_e57866 - 1.0) * var_delta_vsrs_dn8)) } } else { (assign34710_e57867 * (assign34710_e57866 * (var_delta_vsrs_dn8 / var_delta_vsrs))) } * var_t5) + (assign34710_e57867 * var_t5_dn8)) * var_vsat_rs) - (assign34710_e57869 * var_vsat_rs_dn8)) / (var_vsat_rs * var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((var_delta_vsrs).powf(assign34710_e57866 - 1.0) * var_delta_vsrs_dn9)) } } else { (assign34710_e57867 * (assign34710_e57866 * (var_delta_vsrs_dn9 / var_delta_vsrs))) } * var_t5) + (assign34710_e57867 * var_t5_dn9)) * var_vsat_rs) - (assign34710_e57869 * var_vsat_rs_dn9)) / (var_vsat_rs * var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((var_delta_vsrs).powf(assign34710_e57866 - 1.0) * var_delta_vsrs_dn10)) } } else { (assign34710_e57867 * (assign34710_e57866 * (var_delta_vsrs_dn10 / var_delta_vsrs))) } * var_t5) + (assign34710_e57867 * var_t5_dn10)) * var_vsat_rs) - (assign34710_e57869 * var_vsat_rs_dn10)) / (var_vsat_rs * var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((var_delta_vsrs).powf(assign34710_e57866 - 1.0) * var_delta_vsrs_dn11)) } } else { (assign34710_e57867 * (assign34710_e57866 * (var_delta_vsrs_dn11 / var_delta_vsrs))) } * var_t5) + (assign34710_e57867 * var_t5_dn11)) * var_vsat_rs) - (assign34710_e57869 * var_vsat_rs_dn11)) / (var_vsat_rs * var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((var_delta_vsrs).powf(assign34710_e57866 - 1.0) * var_delta_vsrs_dn13)) } } else { (assign34710_e57867 * (assign34710_e57866 * (var_delta_vsrs_dn13 / var_delta_vsrs))) } * var_t5) + (assign34710_e57867 * var_t5_dn13)) * var_vsat_rs) - (assign34710_e57869 * var_vsat_rs_dn13)) / (var_vsat_rs * var_vsat_rs)), (((((if 0.0 == 0.0 && ((assign34710_e57866) as f64).is_finite() && ((assign34710_e57866) as f64).fract() == 0.0 { if assign34710_e57866 == 0.0 { 0.0 } else { (assign34710_e57866 * ((var_delta_vsrs).powf(assign34710_e57866 - 1.0) * var_delta_vsrs_dn14)) } } else { (assign34710_e57867 * (assign34710_e57866 * (var_delta_vsrs_dn14 / var_delta_vsrs))) } * var_t5) + (assign34710_e57867 * var_t5_dn14)) * var_vsat_rs) - (assign34710_e57869 * var_vsat_rs_dn14)) / (var_vsat_rs * var_vsat_rs)),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn13, var_t6_dn14,)
    }
};
        var_t6 = assign34710_e57873;
        var_t6_dn0 = assign34710_e57873_d_n0;
        var_t6_dn2 = assign34710_e57873_d_n2;
        var_t6_dn3 = assign34710_e57873_d_n3;
        var_t6_dn4 = assign34710_e57873_d_n4;
        var_t6_dn5 = assign34710_e57873_d_n5;
        var_t6_dn6 = assign34710_e57873_d_n6;
        var_t6_dn7 = assign34710_e57873_d_n7;
        var_t6_dn8 = assign34710_e57873_d_n8;
        var_t6_dn9 = assign34710_e57873_d_n9;
        var_t6_dn10 = assign34710_e57873_d_n10;
        var_t6_dn11 = assign34710_e57873_d_n11;
        var_t6_dn13 = assign34710_e57873_d_n13;
        var_t6_dn14 = assign34710_e57873_d_n14;

        let (assign34720_e57889, assign34720_e57889_d_n0, assign34720_e57889_d_n2, assign34720_e57889_d_n3, assign34720_e57889_d_n4, assign34720_e57889_d_n5, assign34720_e57889_d_n6, assign34720_e57889_d_n7, assign34720_e57889_d_n8, assign34720_e57889_d_n9, assign34720_e57889_d_n10, assign34720_e57889_d_n11, assign34720_e57889_d_n13, assign34720_e57889_d_n14,) = {
    if ((var_guard657 != 0.0) && (var_guard661 != 0.0)) {
        let assign34720_e57881: f64 = (var_t6).powf(p.p1908);
        let assign34720_e57882: f64 = (1.0 + assign34720_e57881);
        let assign34720_e57885: f64 = (1.0 / p.p1908);
        let assign34720_e57886: f64 = (assign34720_e57882).powf(assign34720_e57885);
        let assign34720_e57887: f64 = (var_t4 * assign34720_e57886);
        (assign34720_e57887, ((var_t4_dn0 * assign34720_e57886) + (var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn0)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn0 / var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn0)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn0 / var_t6))) } / assign34720_e57882))) })), ((var_t4_dn2 * assign34720_e57886) + (var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn2)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn2 / var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn2)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn2 / var_t6))) } / assign34720_e57882))) })), ((var_t4_dn3 * assign34720_e57886) + (var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn3)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn3 / var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn3)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn3 / var_t6))) } / assign34720_e57882))) })), ((var_t4_dn4 * assign34720_e57886) + (var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn4)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn4 / var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn4)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn4 / var_t6))) } / assign34720_e57882))) })), ((var_t4_dn5 * assign34720_e57886) + (var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn5)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn5 / var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn5)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn5 / var_t6))) } / assign34720_e57882))) })), ((var_t4_dn6 * assign34720_e57886) + (var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn6)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn6 / var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn6)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn6 / var_t6))) } / assign34720_e57882))) })), ((var_t4_dn7 * assign34720_e57886) + (var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn7)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn7 / var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn7)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn7 / var_t6))) } / assign34720_e57882))) })), ((var_t4_dn8 * assign34720_e57886) + (var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn8)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn8 / var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn8)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn8 / var_t6))) } / assign34720_e57882))) })), ((var_t4_dn9 * assign34720_e57886) + (var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn9)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn9 / var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn9)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn9 / var_t6))) } / assign34720_e57882))) })), ((var_t4_dn10 * assign34720_e57886) + (var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn10)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn10 / var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn10)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn10 / var_t6))) } / assign34720_e57882))) })), ((var_t4_dn11 * assign34720_e57886) + (var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn11)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn11 / var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn11)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn11 / var_t6))) } / assign34720_e57882))) })), ((var_t4_dn13 * assign34720_e57886) + (var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn13)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn13 / var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn13)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn13 / var_t6))) } / assign34720_e57882))) })), ((var_t4_dn14 * assign34720_e57886) + (var_t4 * if 0.0 == 0.0 && ((assign34720_e57885) as f64).is_finite() && ((assign34720_e57885) as f64).fract() == 0.0 { if assign34720_e57885 == 0.0 { 0.0 } else { (assign34720_e57885 * ((assign34720_e57882).powf(assign34720_e57885 - 1.0) * if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn14)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn14 / var_t6))) })) } } else { (assign34720_e57886 * (assign34720_e57885 * (if 0.0 == 0.0 && ((p.p1908) as f64).is_finite() && ((p.p1908) as f64).fract() == 0.0 { if p.p1908 == 0.0 { 0.0 } else { (p.p1908 * ((var_t6).powf(p.p1908 - 1.0) * var_t6_dn14)) } } else { (assign34720_e57881 * (p.p1908 * (var_t6_dn14 / var_t6))) } / assign34720_e57882))) })),)
    } else {
        (var_rvs_s, var_rvs_s_dn0, var_rvs_s_dn2, var_rvs_s_dn3, var_rvs_s_dn4, var_rvs_s_dn5, var_rvs_s_dn6, var_rvs_s_dn7, var_rvs_s_dn8, var_rvs_s_dn9, var_rvs_s_dn10, var_rvs_s_dn11, var_rvs_s_dn13, var_rvs_s_dn14,)
    }
};
        var_rvs_s = assign34720_e57889;
        var_rvs_s_dn0 = assign34720_e57889_d_n0;
        var_rvs_s_dn2 = assign34720_e57889_d_n2;
        var_rvs_s_dn3 = assign34720_e57889_d_n3;
        var_rvs_s_dn4 = assign34720_e57889_d_n4;
        var_rvs_s_dn5 = assign34720_e57889_d_n5;
        var_rvs_s_dn6 = assign34720_e57889_d_n6;
        var_rvs_s_dn7 = assign34720_e57889_d_n7;
        var_rvs_s_dn8 = assign34720_e57889_d_n8;
        var_rvs_s_dn9 = assign34720_e57889_d_n9;
        var_rvs_s_dn10 = assign34720_e57889_d_n10;
        var_rvs_s_dn11 = assign34720_e57889_d_n11;
        var_rvs_s_dn13 = assign34720_e57889_d_n13;
        var_rvs_s_dn14 = assign34720_e57889_d_n14;

        let assign34730_e57896: f64 = if ((p.p64 != 2.0) && (var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        var_guard664 = assign34730_e57896;

        let (assign34740_e57902, assign34740_e57902_d_n0, assign34740_e57902_d_n2, assign34740_e57902_d_n3, assign34740_e57902_d_n4, assign34740_e57902_d_n5, assign34740_e57902_d_n6, assign34740_e57902_d_n7, assign34740_e57902_d_n8, assign34740_e57902_d_n9, assign34740_e57902_d_n10, assign34740_e57902_d_n11, assign34740_e57902_d_n13, assign34740_e57902_d_n14,) = {
    if (var_guard664 != 0.0) {
        let assign34740_e57900: f64 = (1.0 / var_rdrain);
        (assign34740_e57900, (-(var_rdrain_dn0 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn2 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn3 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn4 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn5 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn6 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn7 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn8 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn9 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn10 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn11 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn13 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn14 / (var_rdrain * var_rdrain))),)
    } else {
        (var_gdpr, var_gdpr_dn0, var_gdpr_dn2, var_gdpr_dn3, var_gdpr_dn4, var_gdpr_dn5, var_gdpr_dn6, var_gdpr_dn7, var_gdpr_dn8, var_gdpr_dn9, var_gdpr_dn10, var_gdpr_dn11, var_gdpr_dn13, var_gdpr_dn14,)
    }
};
        var_gdpr = assign34740_e57902;
        var_gdpr_dn0 = assign34740_e57902_d_n0;
        var_gdpr_dn2 = assign34740_e57902_d_n2;
        var_gdpr_dn3 = assign34740_e57902_d_n3;
        var_gdpr_dn4 = assign34740_e57902_d_n4;
        var_gdpr_dn5 = assign34740_e57902_d_n5;
        var_gdpr_dn6 = assign34740_e57902_d_n6;
        var_gdpr_dn7 = assign34740_e57902_d_n7;
        var_gdpr_dn8 = assign34740_e57902_d_n8;
        var_gdpr_dn9 = assign34740_e57902_d_n9;
        var_gdpr_dn10 = assign34740_e57902_d_n10;
        var_gdpr_dn11 = assign34740_e57902_d_n11;
        var_gdpr_dn13 = assign34740_e57902_d_n13;
        var_gdpr_dn14 = assign34740_e57902_d_n14;

        let assign34750_e57909: f64 = if ((p.p64 == 1.0) && (p.p1910 > 0.0)) { 1.0 } else { 0.0 };
        var_guard665 = assign34750_e57909;

        let (assign34760_e57917, assign34760_e57917_d_n0, assign34760_e57917_d_n2, assign34760_e57917_d_n3, assign34760_e57917_d_n4, assign34760_e57917_d_n5, assign34760_e57917_d_n6, assign34760_e57917_d_n7, assign34760_e57917_d_n8, assign34760_e57917_d_n9, assign34760_e57917_d_n10, assign34760_e57917_d_n11, assign34760_e57917_d_n13, assign34760_e57917_d_n14,) = {
    if ((var_guard664 != 0.0) && (var_guard665 != 0.0)) {
        let assign34760_e57915: f64 = (1.0 / var_rvs_d);
        (assign34760_e57915, (-(var_rvs_d_dn0 / (var_rvs_d * var_rvs_d))), (-(var_rvs_d_dn2 / (var_rvs_d * var_rvs_d))), (-(var_rvs_d_dn3 / (var_rvs_d * var_rvs_d))), (-(var_rvs_d_dn4 / (var_rvs_d * var_rvs_d))), (-(var_rvs_d_dn5 / (var_rvs_d * var_rvs_d))), (-(var_rvs_d_dn6 / (var_rvs_d * var_rvs_d))), (-(var_rvs_d_dn7 / (var_rvs_d * var_rvs_d))), (-(var_rvs_d_dn8 / (var_rvs_d * var_rvs_d))), (-(var_rvs_d_dn9 / (var_rvs_d * var_rvs_d))), (-(var_rvs_d_dn10 / (var_rvs_d * var_rvs_d))), (-(var_rvs_d_dn11 / (var_rvs_d * var_rvs_d))), (-(var_rvs_d_dn13 / (var_rvs_d * var_rvs_d))), (-(var_rvs_d_dn14 / (var_rvs_d * var_rvs_d))),)
    } else {
        (var_gvs_d, var_gvs_d_dn0, var_gvs_d_dn2, var_gvs_d_dn3, var_gvs_d_dn4, var_gvs_d_dn5, var_gvs_d_dn6, var_gvs_d_dn7, var_gvs_d_dn8, var_gvs_d_dn9, var_gvs_d_dn10, var_gvs_d_dn11, var_gvs_d_dn13, var_gvs_d_dn14,)
    }
};
        var_gvs_d = assign34760_e57917;
        var_gvs_d_dn0 = assign34760_e57917_d_n0;
        var_gvs_d_dn2 = assign34760_e57917_d_n2;
        var_gvs_d_dn3 = assign34760_e57917_d_n3;
        var_gvs_d_dn4 = assign34760_e57917_d_n4;
        var_gvs_d_dn5 = assign34760_e57917_d_n5;
        var_gvs_d_dn6 = assign34760_e57917_d_n6;
        var_gvs_d_dn7 = assign34760_e57917_d_n7;
        var_gvs_d_dn8 = assign34760_e57917_d_n8;
        var_gvs_d_dn9 = assign34760_e57917_d_n9;
        var_gvs_d_dn10 = assign34760_e57917_d_n10;
        var_gvs_d_dn11 = assign34760_e57917_d_n11;
        var_gvs_d_dn13 = assign34760_e57917_d_n13;
        var_gvs_d_dn14 = assign34760_e57917_d_n14;

        let assign34770_e57924: f64 = if ((p.p64 != 2.0) && (var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        var_guard666 = assign34770_e57924;

        let (assign34780_e57930, assign34780_e57930_d_n0, assign34780_e57930_d_n2, assign34780_e57930_d_n3, assign34780_e57930_d_n4, assign34780_e57930_d_n5, assign34780_e57930_d_n6, assign34780_e57930_d_n7, assign34780_e57930_d_n8, assign34780_e57930_d_n9, assign34780_e57930_d_n10, assign34780_e57930_d_n11, assign34780_e57930_d_n13, assign34780_e57930_d_n14,) = {
    if (var_guard666 != 0.0) {
        let assign34780_e57928: f64 = (1.0 / var_rsource);
        (assign34780_e57928, (-(var_rsource_dn0 / (var_rsource * var_rsource))), (-(var_rsource_dn2 / (var_rsource * var_rsource))), (-(var_rsource_dn3 / (var_rsource * var_rsource))), (-(var_rsource_dn4 / (var_rsource * var_rsource))), (-(var_rsource_dn5 / (var_rsource * var_rsource))), (-(var_rsource_dn6 / (var_rsource * var_rsource))), (-(var_rsource_dn7 / (var_rsource * var_rsource))), (-(var_rsource_dn8 / (var_rsource * var_rsource))), (-(var_rsource_dn9 / (var_rsource * var_rsource))), (-(var_rsource_dn10 / (var_rsource * var_rsource))), (-(var_rsource_dn11 / (var_rsource * var_rsource))), (-(var_rsource_dn13 / (var_rsource * var_rsource))), (-(var_rsource_dn14 / (var_rsource * var_rsource))),)
    } else {
        (var_gspr, var_gspr_dn0, var_gspr_dn2, var_gspr_dn3, var_gspr_dn4, var_gspr_dn5, var_gspr_dn6, var_gspr_dn7, var_gspr_dn8, var_gspr_dn9, var_gspr_dn10, var_gspr_dn11, var_gspr_dn13, var_gspr_dn14,)
    }
};
        var_gspr = assign34780_e57930;
        var_gspr_dn0 = assign34780_e57930_d_n0;
        var_gspr_dn2 = assign34780_e57930_d_n2;
        var_gspr_dn3 = assign34780_e57930_d_n3;
        var_gspr_dn4 = assign34780_e57930_d_n4;
        var_gspr_dn5 = assign34780_e57930_d_n5;
        var_gspr_dn6 = assign34780_e57930_d_n6;
        var_gspr_dn7 = assign34780_e57930_d_n7;
        var_gspr_dn8 = assign34780_e57930_d_n8;
        var_gspr_dn9 = assign34780_e57930_d_n9;
        var_gspr_dn10 = assign34780_e57930_d_n10;
        var_gspr_dn11 = assign34780_e57930_d_n11;
        var_gspr_dn13 = assign34780_e57930_d_n13;
        var_gspr_dn14 = assign34780_e57930_d_n14;

        let assign34790_e57937: f64 = if ((p.p64 == 1.0) && (p.p1911 > 0.0)) { 1.0 } else { 0.0 };
        var_guard667 = assign34790_e57937;

        let (assign34800_e57945, assign34800_e57945_d_n0, assign34800_e57945_d_n2, assign34800_e57945_d_n3, assign34800_e57945_d_n4, assign34800_e57945_d_n5, assign34800_e57945_d_n6, assign34800_e57945_d_n7, assign34800_e57945_d_n8, assign34800_e57945_d_n9, assign34800_e57945_d_n10, assign34800_e57945_d_n11, assign34800_e57945_d_n13, assign34800_e57945_d_n14,) = {
    if ((var_guard666 != 0.0) && (var_guard667 != 0.0)) {
        let assign34800_e57943: f64 = (1.0 / var_rvs_s);
        (assign34800_e57943, (-(var_rvs_s_dn0 / (var_rvs_s * var_rvs_s))), (-(var_rvs_s_dn2 / (var_rvs_s * var_rvs_s))), (-(var_rvs_s_dn3 / (var_rvs_s * var_rvs_s))), (-(var_rvs_s_dn4 / (var_rvs_s * var_rvs_s))), (-(var_rvs_s_dn5 / (var_rvs_s * var_rvs_s))), (-(var_rvs_s_dn6 / (var_rvs_s * var_rvs_s))), (-(var_rvs_s_dn7 / (var_rvs_s * var_rvs_s))), (-(var_rvs_s_dn8 / (var_rvs_s * var_rvs_s))), (-(var_rvs_s_dn9 / (var_rvs_s * var_rvs_s))), (-(var_rvs_s_dn10 / (var_rvs_s * var_rvs_s))), (-(var_rvs_s_dn11 / (var_rvs_s * var_rvs_s))), (-(var_rvs_s_dn13 / (var_rvs_s * var_rvs_s))), (-(var_rvs_s_dn14 / (var_rvs_s * var_rvs_s))),)
    } else {
        (var_gvs_s, var_gvs_s_dn0, var_gvs_s_dn2, var_gvs_s_dn3, var_gvs_s_dn4, var_gvs_s_dn5, var_gvs_s_dn6, var_gvs_s_dn7, var_gvs_s_dn8, var_gvs_s_dn9, var_gvs_s_dn10, var_gvs_s_dn11, var_gvs_s_dn13, var_gvs_s_dn14,)
    }
};
        var_gvs_s = assign34800_e57945;
        var_gvs_s_dn0 = assign34800_e57945_d_n0;
        var_gvs_s_dn2 = assign34800_e57945_d_n2;
        var_gvs_s_dn3 = assign34800_e57945_d_n3;
        var_gvs_s_dn4 = assign34800_e57945_d_n4;
        var_gvs_s_dn5 = assign34800_e57945_d_n5;
        var_gvs_s_dn6 = assign34800_e57945_d_n6;
        var_gvs_s_dn7 = assign34800_e57945_d_n7;
        var_gvs_s_dn8 = assign34800_e57945_d_n8;
        var_gvs_s_dn9 = assign34800_e57945_d_n9;
        var_gvs_s_dn10 = assign34800_e57945_d_n10;
        var_gvs_s_dn11 = assign34800_e57945_d_n11;
        var_gvs_s_dn13 = assign34800_e57945_d_n13;
        var_gvs_s_dn14 = assign34800_e57945_d_n14;

        let assign34810_e57952: f64 = if ((p.p73 == 1.0) && (var_xrcrg1_i != 0.0)) { 1.0 } else { 0.0 };
        var_guard668 = assign34810_e57952;

        let assign34820_e57955: f64 = if p.p73 == 2.0 { 1.0 } else { 0.0 };
        var_guard669 = assign34820_e57955;

        let assign34900_e57995: f64 = if p.p72 == 0.0 { 1.0 } else { 0.0 };
        var_guard677 = assign34900_e57995;

        let assign34950_e58014: f64 = if ((p.p74 != 0.0) && (p.p1791 > 0.0)) { 1.0 } else { 0.0 };
        var_guard682 = assign34950_e58014;

        let assign34960_e58021: f64 = if ((p.p64 != 2.0) && (var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        var_guard683 = assign34960_e58021;

        let assign34970_e58028: f64 = if ((p.p64 == 1.0) && (p.p1910 > 0.0)) { 1.0 } else { 0.0 };
        var_guard684 = assign34970_e58028;

        let assign34980_e58035: f64 = if ((p.p64 != 2.0) && (var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        var_guard685 = assign34980_e58035;

        let assign34990_e58042: f64 = if ((p.p64 == 1.0) && (p.p1911 > 0.0)) { 1.0 } else { 0.0 };
        var_guard686 = assign34990_e58042;

        let assign35580_e58822: f64 = (10.0 * var_vtm);
        let assign35580_e58824: f64 = (assign35580_e58822 / var_rc);
        let assign35580_e58827: f64 = (2.0 * var_qbs);
        let assign35580_e58828: f64 = (assign35580_e58824 + assign35580_e58827);
        var_q0 = assign35580_e58828;
        var_q0_dn0 = 0.0;
        var_q0_dn2 = 0.0;
        var_q0_dn3 = 0.0;
        var_q0_dn4 = ((10.0 * var_vtm_dn4) / var_rc);
        var_q0_dn5 = 0.0;
        var_q0_dn6 = 0.0;
        var_q0_dn7 = 0.0;
        var_q0_dn8 = 0.0;
        var_q0_dn9 = 0.0;
        var_q0_dn10 = 0.0;
        var_q0_dn11 = 0.0;
        var_q0_dn13 = 0.0;
        var_q0_dn14 = 0.0;

        let assign35590_e58832: f64 = (var_vtm + var_q0);
        let assign35590_e58833: f64 = (var_vtm * assign35590_e58832);
        var_t1 = assign35590_e58833;
        var_t1_dn0 = (var_vtm * var_q0_dn0);
        var_t1_dn2 = (var_vtm * var_q0_dn2);
        var_t1_dn3 = (var_vtm * var_q0_dn3);
        var_t1_dn4 = ((var_vtm_dn4 * assign35590_e58832) + (var_vtm * (var_vtm_dn4 + var_q0_dn4)));
        var_t1_dn5 = (var_vtm * var_q0_dn5);
        var_t1_dn6 = (var_vtm * var_q0_dn6);
        var_t1_dn7 = (var_vtm * var_q0_dn7);
        var_t1_dn8 = (var_vtm * var_q0_dn8);
        var_t1_dn9 = (var_vtm * var_q0_dn9);
        var_t1_dn10 = (var_vtm * var_q0_dn10);
        var_t1_dn11 = (var_vtm * var_q0_dn11);
        var_t1_dn13 = (var_vtm * var_q0_dn13);
        var_t1_dn14 = (var_vtm * var_q0_dn14);

        let assign35600_e58836: f64 = (var_cox * var_cox);
        let assign35600_e58838: f64 = (assign35600_e58836 * var_t1);
        var_t2 = assign35600_e58838;
        var_t2_dn0 = (assign35600_e58836 * var_t1_dn0);
        var_t2_dn2 = (assign35600_e58836 * var_t1_dn2);
        var_t2_dn3 = (assign35600_e58836 * var_t1_dn3);
        var_t2_dn4 = (assign35600_e58836 * var_t1_dn4);
        var_t2_dn5 = (assign35600_e58836 * var_t1_dn5);
        var_t2_dn6 = (assign35600_e58836 * var_t1_dn6);
        var_t2_dn7 = (assign35600_e58836 * var_t1_dn7);
        var_t2_dn8 = (assign35600_e58836 * var_t1_dn8);
        var_t2_dn9 = (assign35600_e58836 * var_t1_dn9);
        var_t2_dn10 = (assign35600_e58836 * var_t1_dn10);
        var_t2_dn11 = (assign35600_e58836 * var_t1_dn11);
        var_t2_dn13 = (assign35600_e58836 * var_t1_dn13);
        var_t2_dn14 = (assign35600_e58836 * var_t1_dn14);

        let assign35610_e58841: f64 = (2.0 * 1.60219e-19);
        let assign35610_e58843: f64 = (assign35610_e58841 * var_ni);
        let assign35610_e58845: f64 = (assign35610_e58843 * var_epssub);
        let assign35610_e58847: f64 = (assign35610_e58845 * var_vtm);
        var_t3 = assign35610_e58847;
        var_t3_dn0 = (((assign35610_e58841 * var_ni_dn0) * var_epssub) * var_vtm);
        var_t3_dn2 = (((assign35610_e58841 * var_ni_dn2) * var_epssub) * var_vtm);
        var_t3_dn3 = (((assign35610_e58841 * var_ni_dn3) * var_epssub) * var_vtm);
        var_t3_dn4 = ((((assign35610_e58841 * var_ni_dn4) * var_epssub) * var_vtm) + (assign35610_e58845 * var_vtm_dn4));
        var_t3_dn5 = (((assign35610_e58841 * var_ni_dn5) * var_epssub) * var_vtm);
        var_t3_dn6 = (((assign35610_e58841 * var_ni_dn6) * var_epssub) * var_vtm);
        var_t3_dn7 = (((assign35610_e58841 * var_ni_dn7) * var_epssub) * var_vtm);
        var_t3_dn8 = (((assign35610_e58841 * var_ni_dn8) * var_epssub) * var_vtm);
        var_t3_dn9 = (((assign35610_e58841 * var_ni_dn9) * var_epssub) * var_vtm);
        var_t3_dn10 = (((assign35610_e58841 * var_ni_dn10) * var_epssub) * var_vtm);
        var_t3_dn11 = (((assign35610_e58841 * var_ni_dn11) * var_epssub) * var_vtm);
        var_t3_dn13 = (((assign35610_e58841 * var_ni_dn13) * var_epssub) * var_vtm);
        var_t3_dn14 = (((assign35610_e58841 * var_ni_dn14) * var_epssub) * var_vtm);

        *var_delta_vsrs_slot = var_delta_vsrs;
        *var_delta_vsrs_dn0_slot = var_delta_vsrs_dn0;
        *var_delta_vsrs_dn10_slot = var_delta_vsrs_dn10;
        *var_delta_vsrs_dn11_slot = var_delta_vsrs_dn11;
        *var_delta_vsrs_dn13_slot = var_delta_vsrs_dn13;
        *var_delta_vsrs_dn14_slot = var_delta_vsrs_dn14;
        *var_delta_vsrs_dn2_slot = var_delta_vsrs_dn2;
        *var_delta_vsrs_dn3_slot = var_delta_vsrs_dn3;
        *var_delta_vsrs_dn4_slot = var_delta_vsrs_dn4;
        *var_delta_vsrs_dn5_slot = var_delta_vsrs_dn5;
        *var_delta_vsrs_dn6_slot = var_delta_vsrs_dn6;
        *var_delta_vsrs_dn7_slot = var_delta_vsrs_dn7;
        *var_delta_vsrs_dn8_slot = var_delta_vsrs_dn8;
        *var_delta_vsrs_dn9_slot = var_delta_vsrs_dn9;
        *var_gdpr_slot = var_gdpr;
        *var_gdpr_dn0_slot = var_gdpr_dn0;
        *var_gdpr_dn10_slot = var_gdpr_dn10;
        *var_gdpr_dn11_slot = var_gdpr_dn11;
        *var_gdpr_dn13_slot = var_gdpr_dn13;
        *var_gdpr_dn14_slot = var_gdpr_dn14;
        *var_gdpr_dn2_slot = var_gdpr_dn2;
        *var_gdpr_dn3_slot = var_gdpr_dn3;
        *var_gdpr_dn4_slot = var_gdpr_dn4;
        *var_gdpr_dn5_slot = var_gdpr_dn5;
        *var_gdpr_dn6_slot = var_gdpr_dn6;
        *var_gdpr_dn7_slot = var_gdpr_dn7;
        *var_gdpr_dn8_slot = var_gdpr_dn8;
        *var_gdpr_dn9_slot = var_gdpr_dn9;
        *var_gspr_slot = var_gspr;
        *var_gspr_dn0_slot = var_gspr_dn0;
        *var_gspr_dn10_slot = var_gspr_dn10;
        *var_gspr_dn11_slot = var_gspr_dn11;
        *var_gspr_dn13_slot = var_gspr_dn13;
        *var_gspr_dn14_slot = var_gspr_dn14;
        *var_gspr_dn2_slot = var_gspr_dn2;
        *var_gspr_dn3_slot = var_gspr_dn3;
        *var_gspr_dn4_slot = var_gspr_dn4;
        *var_gspr_dn5_slot = var_gspr_dn5;
        *var_gspr_dn6_slot = var_gspr_dn6;
        *var_gspr_dn7_slot = var_gspr_dn7;
        *var_gspr_dn8_slot = var_gspr_dn8;
        *var_gspr_dn9_slot = var_gspr_dn9;
        *var_guard664_slot = var_guard664;
        *var_guard665_slot = var_guard665;
        *var_guard666_slot = var_guard666;
        *var_guard667_slot = var_guard667;
        *var_guard668_slot = var_guard668;
        *var_guard669_slot = var_guard669;
        *var_guard677_slot = var_guard677;
        *var_guard682_slot = var_guard682;
        *var_guard683_slot = var_guard683;
        *var_guard684_slot = var_guard684;
        *var_guard685_slot = var_guard685;
        *var_guard686_slot = var_guard686;
        *var_gvs_d_slot = var_gvs_d;
        *var_gvs_d_dn0_slot = var_gvs_d_dn0;
        *var_gvs_d_dn10_slot = var_gvs_d_dn10;
        *var_gvs_d_dn11_slot = var_gvs_d_dn11;
        *var_gvs_d_dn13_slot = var_gvs_d_dn13;
        *var_gvs_d_dn14_slot = var_gvs_d_dn14;
        *var_gvs_d_dn2_slot = var_gvs_d_dn2;
        *var_gvs_d_dn3_slot = var_gvs_d_dn3;
        *var_gvs_d_dn4_slot = var_gvs_d_dn4;
        *var_gvs_d_dn5_slot = var_gvs_d_dn5;
        *var_gvs_d_dn6_slot = var_gvs_d_dn6;
        *var_gvs_d_dn7_slot = var_gvs_d_dn7;
        *var_gvs_d_dn8_slot = var_gvs_d_dn8;
        *var_gvs_d_dn9_slot = var_gvs_d_dn9;
        *var_gvs_s_slot = var_gvs_s;
        *var_gvs_s_dn0_slot = var_gvs_s_dn0;
        *var_gvs_s_dn10_slot = var_gvs_s_dn10;
        *var_gvs_s_dn11_slot = var_gvs_s_dn11;
        *var_gvs_s_dn13_slot = var_gvs_s_dn13;
        *var_gvs_s_dn14_slot = var_gvs_s_dn14;
        *var_gvs_s_dn2_slot = var_gvs_s_dn2;
        *var_gvs_s_dn3_slot = var_gvs_s_dn3;
        *var_gvs_s_dn4_slot = var_gvs_s_dn4;
        *var_gvs_s_dn5_slot = var_gvs_s_dn5;
        *var_gvs_s_dn6_slot = var_gvs_s_dn6;
        *var_gvs_s_dn7_slot = var_gvs_s_dn7;
        *var_gvs_s_dn8_slot = var_gvs_s_dn8;
        *var_gvs_s_dn9_slot = var_gvs_s_dn9;
        *var_q0_slot = var_q0;
        *var_q0_dn0_slot = var_q0_dn0;
        *var_q0_dn10_slot = var_q0_dn10;
        *var_q0_dn11_slot = var_q0_dn11;
        *var_q0_dn13_slot = var_q0_dn13;
        *var_q0_dn14_slot = var_q0_dn14;
        *var_q0_dn2_slot = var_q0_dn2;
        *var_q0_dn3_slot = var_q0_dn3;
        *var_q0_dn4_slot = var_q0_dn4;
        *var_q0_dn5_slot = var_q0_dn5;
        *var_q0_dn6_slot = var_q0_dn6;
        *var_q0_dn7_slot = var_q0_dn7;
        *var_q0_dn8_slot = var_q0_dn8;
        *var_q0_dn9_slot = var_q0_dn9;
        *var_rvs_s_slot = var_rvs_s;
        *var_rvs_s_dn0_slot = var_rvs_s_dn0;
        *var_rvs_s_dn10_slot = var_rvs_s_dn10;
        *var_rvs_s_dn11_slot = var_rvs_s_dn11;
        *var_rvs_s_dn13_slot = var_rvs_s_dn13;
        *var_rvs_s_dn14_slot = var_rvs_s_dn14;
        *var_rvs_s_dn2_slot = var_rvs_s_dn2;
        *var_rvs_s_dn3_slot = var_rvs_s_dn3;
        *var_rvs_s_dn4_slot = var_rvs_s_dn4;
        *var_rvs_s_dn5_slot = var_rvs_s_dn5;
        *var_rvs_s_dn6_slot = var_rvs_s_dn6;
        *var_rvs_s_dn7_slot = var_rvs_s_dn7;
        *var_rvs_s_dn8_slot = var_rvs_s_dn8;
        *var_rvs_s_dn9_slot = var_rvs_s_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
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
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn3_slot = var_t2_dn3;
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
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn13_slot = var_t5_dn13;
        *var_t5_dn14_slot = var_t5_dn14;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn3_slot = var_t5_dn3;
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
        *var_t6_dn13_slot = var_t6_dn13;
        *var_t6_dn14_slot = var_t6_dn14;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_vsat_rs_slot = var_vsat_rs;
        *var_vsat_rs_dn0_slot = var_vsat_rs_dn0;
        *var_vsat_rs_dn10_slot = var_vsat_rs_dn10;
        *var_vsat_rs_dn11_slot = var_vsat_rs_dn11;
        *var_vsat_rs_dn13_slot = var_vsat_rs_dn13;
        *var_vsat_rs_dn14_slot = var_vsat_rs_dn14;
        *var_vsat_rs_dn2_slot = var_vsat_rs_dn2;
        *var_vsat_rs_dn3_slot = var_vsat_rs_dn3;
        *var_vsat_rs_dn4_slot = var_vsat_rs_dn4;
        *var_vsat_rs_dn5_slot = var_vsat_rs_dn5;
        *var_vsat_rs_dn6_slot = var_vsat_rs_dn6;
        *var_vsat_rs_dn7_slot = var_vsat_rs_dn7;
        *var_vsat_rs_dn8_slot = var_vsat_rs_dn8;
        *var_vsat_rs_dn9_slot = var_vsat_rs_dn9;
    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(188, 0.0);

        s.store_scalar(197, 0.0);

        s.store_scalar(263, 0.0);

        s.store_scalar(264, 0.0);

        s.store_scalar(265, 0.0);

        s.store_scalar(266, 0.0);

        s.store_scalar(267, 0.0);

        s.store_scalar(268, 0.0);

        s.store_scalar(269, 0.0);

        s.store_scalar(270, 0.0);

        s.store_scalar(271, 0.0);

        s.store_scalar(272, 0.0);

        s.store_scalar(273, 0.0);

        s.store_scalar(274, 0.0);

        s.store_scalar(275, 0.0);

        s.store_scalar(276, 0.0);

        s.store_scalar(277, 0.0);

        s.store_scalar(278, 0.0);

        s.store_scalar(279, 0.0);

        s.store_scalar(280, 0.0);

        s.store_scalar(281, 0.0);

        s.store_scalar(282, 0.0);

        s.store_scalar(283, 0.0);

        s.store_scalar(284, 0.0);

        s.store_scalar(285, 0.0);

        s.store_scalar(286, 0.0);

        s.store_scalar(287, 0.0);

        s.store_scalar(288, 0.0);

        s.store_scalar(289, 0.0);

        s.store_scalar(290, 0.0);

        s.store_scalar(291, 0.0);

        s.store_scalar(292, 0.0);

        s.store_scalar(300, 0.0);

        s.store_scalar(302, 0.0);

        s.store_scalar(305, 0.0);

        s.store_scalar(314, 0.0);

        s.store_scalar(315, 0.0);

        s.store_scalar(316, 0.0);

        s.store_scalar(320, 0.0);

        s.store_scalar(333, 0.0);

        s.store_scalar(335, 0.0);

        s.store_scalar(338, 0.0);

        s.store_scalar(258, 0.0);

        s.store_scalar(857, 0.0);

        s.store_scalar(373, 0.0);

        s.store_scalar(401, 0.0);

        s.store_scalar(417, 0.0);

        s.store_scalar(453, 0.0);

        s.store_scalar(756, 0.0);

        s.store_scalar(757, 0.0);

        s.store_scalar(255, 0.0);

        s.store_scalar(758, 0.0);

        s.store_scalar(759, 0.0);

        s.store_scalar(760, 0.0);

        s.store_scalar(770, 0.0);

        s.store_scalar(771, 0.0);

        s.store_scalar(251, 0.0);

        s.store_scalar(772, 0.0);

        s.store_scalar(773, 0.0);

        s.store_scalar(774, 0.0);

        s.store_scalar(494, 0.0);

        s.store_scalar(495, 0.0);

        s.store_scalar(496, 0.0);

        s.store_scalar(498, 0.0);

        s.store_scalar(499, 0.0);

        s.store_scalar(523, 0.0);

        s.store_scalar(524, 0.0);

        s.store_scalar(525, 0.0);

        s.store_scalar(526, 0.0);

        s.store_scalar(527, 0.0);

        s.store_scalar(528, 0.0);

        s.store_scalar(529, 0.0);

        s.store_scalar(533, 0.0);

        s.store_scalar(537, 0.0);

        s.store_scalar(538, 0.0);

        s.store_scalar(539, 0.0);

        s.store_scalar(540, 0.0);

        s.store_scalar(546, 0.0);

        s.store_scalar(547, 0.0);

        s.store_scalar(541, 0.0);

        s.store_scalar(542, 0.0);

        s.store_scalar(543, 0.0);

        s.store_scalar(553, 0.0);

        s.store_scalar(554, 0.0);

        s.store_scalar(548, 0.0);

        s.store_scalar(549, 0.0);

        s.store_scalar(550, 0.0);

        s.store_scalar(557, 0.0);

        s.store_scalar(558, 0.0);

        s.store_scalar(559, 0.0);

        s.store_scalar(560, 0.0);

        s.store_scalar(561, 0.0);

        s.store_scalar(562, 0.0);

        s.store_scalar(563, 0.0);

        s.store_scalar(564, 0.0);

        s.store_scalar(565, 0.0);

        s.store_scalar(566, 0.0);

        s.store_scalar(567, 0.0);

        s.store_scalar(568, 0.0);

        s.store_scalar(589, 0.0);

        s.store_scalar(574, 0.0);

        s.store_scalar(575, 0.0);

        s.store_scalar(620, 0.0);

        s.store_scalar(632, 0.0);

        s.store_scalar(634, 0.0);

        s.store_scalar(668, 0.0);

        s.store_scalar(665, 0.0);

        s.store_scalar(677, 0.0);

        s.store_scalar(806, 0.0);

        s.store_scalar(370, 0.0);

        s.store_scalar(689, 0.0);

        s.store_scalar(690, 0.0);

        s.store_scalar(691, 0.0);

        s.store_scalar(692, 0.0);

        s.store_scalar(693, 0.0);

        s.store_scalar(871, 0.0);

        s.store_scalar(872, 0.0);

        s.store_scalar(680, 0.0);

        s.store_scalar(699, 0.0);

        s.store_scalar(658, 0.0);

        s.store_scalar(791, 0.0);

        s.store_scalar(701, 0.0);

        s.store_scalar(851, 0.0);

        s.store_scalar(706, 0.0);

        s.store_scalar(710, 0.0);

        s.store_scalar(815, 0.0);

        s.store_scalar(809, 0.0);

        s.store_scalar(817, 0.0);

        s.store_scalar(816, 0.0);

        s.store_scalar(818, 0.0);

        s.store_scalar(845, 0.0);

        s.store_scalar(846, 0.0);

        s.store_scalar(825, 0.0);

        s.store_scalar(828, 0.0);

        s.store_scalar(843, 0.0);

        s.store_scalar(844, 0.0);

        s.store_scalar(715, 0.0);

        s.store_scalar(717, 0.0);

        s.store_scalar(796, 0.0);

        s.store_scalar(646, 0.0);

        s.store_scalar(647, 0.0);

        s.store_scalar(645, 0.0);

        s.store_scalar(644, 0.0);

        s.store_scalar(893, 0.0);

        s.store_scalar(894, 0.0);

        s.store_scalar(895, 0.0);

        s.store_scalar(896, 0.0);

        s.store_scalar(898, 0.0);

        s.store_scalar(903, 0.0);

        s.store_scalar(904, 0.0);

        s.store_scalar(923, 0.0);

        s.store_scalar(392, 0.0);

        s.store_scalar(393, 0.0);

        s.store_scalar(503, 0.0);

        s.store_scalar(504, 0.0);

        s.store_scalar(949, 0.0);

        s.store_scalar(950, 0.0);

        s.store_scalar(951, 0.0);

        s.store_scalar(952, 0.0);

        s.store_scalar(953, 0.0);

        s.store_scalar(955, 0.0);

        s.store_scalar(956, 0.0);

        s.store_scalar(957, 0.0);

        s.store_scalar(958, 0.0);

        s.store_scalar(959, 0.0);

        s.store_scalar(1004, 0.0);

        s.store_scalar(1005, 0.0);

        s.store_scalar(1006, 0.0);

        s.store_scalar(1007, 0.0);

        s.store_scalar(1008, 0.0);

        s.store_scalar(1009, 0.0);

        s.store_scalar(983, 1.0);

        s.store_scalar(960, 0.0);

        s.store_scalar(961, 0.0);

        s.store_scalar(962, 0.0);

        s.store_scalar(963, 0.0);

        s.store_scalar(964, 0.0);

        s.store_scalar(965, 0.0);

        s.store_scalar(984, 0.0);

        s.store_scalar(985, 0.0);

        s.store_scalar(986, 0.0);

        s.store_scalar(1010, 0.0);

        s.store_scalar(1011, 0.0);

        s.store_scalar(1012, 0.0);

        s.store_scalar(882, 0.0);

        s.store_scalar(883, 0.0);

        s.store_scalar(884, 0.0);

        s.store_scalar(885, 0.0);

        s.store_scalar(886, 0.0);

        s.store_scalar(887, 0.0);

        s.store_scalar(888, 0.0);

        s.store_scalar(889, 0.0);

        s.store_scalar(890, 0.0);

        s.store_scalar(891, 0.0);

        s.store_scalar(892, 0.0);

        s.store_scalar(119, 0.0);

        s.store_scalar(120, 0.0);

        s.store_scalar(118, 0.0);

        s.store_scalar(117, 0.0);

        s.store_scalar(233, 0.0);

        s.store_scalar(234, 0.0);

        s.store_scalar(182, 0.0);

        s.store_scalar(142, 0.0);

        s.store_scalar(324, 0.0);

        s.store_scalar(327, 0.0);

        s.store_scalar(306, 0.0);

        s.store_scalar(307, 0.0);

        s.store_scalar(310, 0.0);

        s.store_scalar(311, 0.0);

        s.store_scalar(313, 0.0);

        s.store_scalar(312, 0.0);

        s.store_scalar(331, 0.0);

        s.store_scalar(330, 0.0);

        s.store_scalar(1039, 0.0);

        s.store_scalar(446, 0.0);

        s.store_scalar(576, 0.0);

        s.b[1057] = (p.p60 == 1.0);
        s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });

        if s.b[1057] {
            s.store_scalar(114, 1.0);
        }

        if (!s.b[1057]) {
            s.store_scalar(114, (-1.0));
        }

        s.store_scalar(143, (p.p103 * 8.8542e-12));

        s.store_scalar(144, (p.p1088 * 8.8542e-12));

        s.store_scalar(165, ((p.p102 * 8.8542e-12) / p.p91));

        s.store_scalar(145, (p.p103 / p.p102));

        s.store_scalar(381, (0.916 * 9.11e-31));

        s.store_scalar(382, (0.19 * 9.11e-31));

        s.store_scalar(383, (0.19 * 9.11e-31));

        s.store_scalar(384, (0.417 * 9.11e-31));

        s.store_scalar(385, 4.0);

        s.store_scalar(386, 2.0);

        s.store_scalar(876, (((p.p109 + ((1e-6 * p.p110) / p.p0)) + (p.p111 / p.p5)) + ((p.p112 * 1e-6) / (p.p0 * p.p5))));

        s.store_scalar(878, (((p.p117 + ((1e-6 * p.p118) / p.p0)) + (p.p119 / p.p5)) + ((p.p120 * 1e-6) / (p.p0 * p.p5))));

        s.store_scalar(877, (((p.p113 + ((1e-6 * p.p114) / p.p0)) + (p.p115 / p.p5)) + ((p.p116 * 1e-6) / (p.p0 * p.p5))));

        s.store_scalar(149, (p.p0 + s.v[876]));

        s.b[1058] = (s.v[149] <= 0.0);
        s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });

        if s.b[1058] {
            s.store_scalar(149, p.p0);
        }

        s.store_powf(168, 149, (-p.p84));

        s.store_offset_scaled(150, 168, p.p83, s.v[877]);

        s.store_offset_scaled_ad(151, A::powf(A::offset(s.ad_value(149), s.v[878]), (-p.p84)), p.p83, s.v[877]);

        s.store_offset_scaled(152, 168, p.p88, p.p85);

        s.store_sub_scaled_inputs(153, 149, 1.0, 150, 2.0);

        s.store_sub_scaled_ad_lhs(155, A::offset(s.ad_value(149), s.v[878]), 151, 2.0);

        s.store_sub_scaled_inputs(156, 149, 1.0, 152, 2.0);

        s.store_offset(157, 156, (-p.p86));

        s.b[1059] = (s.v[153] <= 0.0);
        s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });

        if s.b[1059] {
            s.copy_ad(153, 149);
        }

        s.b[1061] = (s.v[155] <= 0.0);
        s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });

        if s.b[1061] {
            s.copy_ad(155, 149);
        }

        s.b[1063] = (s.v[156] <= 0.0);
        s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1063] {
            s.copy_ad(156, 149);
        }

        s.b[1065] = (p.p61 != 0.0);
        s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });

        s.b[1066] = (s.v[157] <= 0.0);
        s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });

        if (s.b[1065] && s.b[1066]) {
            s.copy_ad(157, 149);
        }

        s.b[1068] = (p.p62 == 5.0);
        s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });

        if s.b[1068] {
            s.store_scalar(879, (((((p.p121 + ((1e-6 * p.p122) / p.p0)) + (p.p123 / p.p5)) + ((p.p124 * 1e-6) / (p.p0 * p.p5))) + ((1e-6 * p.p125) / p.p43)) + ((p.p126 * 1e-12) / (p.p0 * p.p43))));
            s.store_scalar(880, (((((p.p127 + ((1e-6 * p.p128) / p.p0)) + (p.p129 / p.p5)) + ((p.p130 * 1e-6) / (p.p0 * p.p5))) + ((1e-6 * p.p131) / p.p43)) + ((p.p132 * 1e-12) / (p.p0 * p.p43))));
        }

        if (!s.b[1068]) {
            s.store_scalar(879, 0.0);
            s.store_scalar(880, 0.0);
        }

        s.store_offset(161, 879, p.p43);

        s.store_add(162, 161, 880);

        s.b[1069] = (p.p62 == 5.0);
        s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });

        s.b[1070] = (s.v[162] <= 0.0);
        s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });

        if (s.b[1069] && s.b[1070]) {
            s.store_scalar(162, p.p43);
        }

        s.store_scalar(115, (p.p5 * p.p59));

        s.store_div_from_scalar(635, 1e-6, 155);

        s.store_scalar(636, (1.0 / p.p5));

        s.store_div_from_scalar_scaled_input(637, 1e-6, 155, p.p5);

        s.b[1072] = (p.p62 == 5.0);
        s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });

        if s.b[1072] {
            s.store_div_from_scalar(638, 1e-6, 162);
            s.store_div_from_scalar_mul_ad(639, 1e-12, s.ad_value(162), s.ad_value(155));
        }

        if (!s.b[1072]) {
            s.store_scalar(638, 0.0);
            s.store_scalar(639, 0.0);
        }

        s.store_add_scaled_inputs4_offset_indices(640, 635, p.p134, 637, p.p136, 638, 0.0, 639, 0.0, ((p.p133) + ((s.v[636] * p.p135))));

        s.b[1073] = (p.p95 != 0.0);
        s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });

        if s.b[1073] {
            s.store_scale(640, 640, (1.0 + ((p.p95 / p.p5) * (if (!((1.0 + (p.p5 / p.p96)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p96)) > 1e-38) { (((1.0 + (p.p5 / p.p96))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1074] = (s.v[640] <= 0.0);
        s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });

        if s.b[1074] {
            s.store_scalar(640, 1e22);
        }

        s.b[1076] = (p.p62 == 0.0);
        s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });

        s.b[1077] = (p.p62 == 1.0);
        s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });

        s.b[1078] = (p.p62 == 2.0);
        s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });

        s.b[1079] = (p.p62 == 3.0);
        s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });

        s.b[1080] = (p.p62 == 4.0);
        s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });

        s.b[1081] = (p.p62 == 5.0);
        s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });

        s.b[1082] = ((p.p1802 == 0.0) || (p.p1803 == 0.0));
        s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });

        if (s.b[1076] && s.b[1082]) {
            s.store_scalar(895, (2.0 * p.p92));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, (p.p92 * p.p3));
        }

        if (s.b[1076] && (!s.b[1082])) {
            s.store_scalar(895, (2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));
        }

        s.b[1083] = ((p.p1802 == 0.0) || (p.p1803 == 0.0));
        s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });

        if ((s.b[1077] && (!s.b[1076])) && s.b[1083]) {
            s.store_scalar(895, ((2.0 * p.p92) + p.p3));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, (p.p92 * p.p3));
        }

        if ((s.b[1077] && (!s.b[1076])) && (!s.b[1083])) {
            s.store_scalar(895, ((2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()) + p.p1802));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));
        }

        s.b[1084] = ((p.p1802 == 0.0) || (p.p1803 == 0.0));
        s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });

        if ((s.b[1078] && (!(s.b[1076] || s.b[1077]))) && s.b[1084]) {
            s.store_scalar(895, ((2.0 * p.p92) + (2.0 * p.p3)));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, (p.p92 * p.p3));
        }

        if ((s.b[1078] && (!(s.b[1076] || s.b[1077]))) && (!s.b[1084])) {
            s.store_scalar(895, (((2.0 * ((((p.p92 * p.p92) + (((p.p1802 - p.p1803) * (p.p1802 - p.p1803)) / 4.0))) as f64).sqrt()) + p.p1802) + p.p1803));
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
            s.store_scalar(894, ((p.p92 * (p.p1802 + p.p1803)) / 2.0));
        }

        if (s.b[1078] && (!(s.b[1076] || s.b[1077]))) {
            s.store_scalar(896, p.p1803);
        }

        if (s.b[1079] && (!((s.b[1076] || s.b[1077]) || s.b[1078]))) {
            s.store_scalar(895, (3.141592653589793 * p.p2));
        }

        if (s.b[1079] && (!((s.b[1076] || s.b[1077]) || s.b[1078]))) {
            s.store_scalar(893, ((((2.0 * 3.141592653589793) * p.p102) * 8.8542e-12) / (if (!((1.0 + ((2.0 * p.p89) / p.p2)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + ((2.0 * p.p89) / p.p2)) > 1e-38) { (((1.0 + ((2.0 * p.p89) / p.p2))) as f64).ln() } else { 0.0 }) })));
        }

        if (s.b[1079] && (!((s.b[1076] || s.b[1077]) || s.b[1078]))) {
            s.store_scalar(894, (((3.141592653589793 * p.p2) * p.p2) / 4.0));
            s.store_scalar(896, p.p2);
        }

        if (s.b[1080] && (!(((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]))) {
            s.store_scalar(895, p.p1801);
            s.store_scalar(893, p.p1800);
            s.store_scalar(894, p.p1799);
        }

        if (s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) {
            s.store_offset_scaled(954, 161, 2.0, ((((p.p40) * (2.0))) + (p.p44)));
            s.store_offset_scaled(948, 161, p.p40, p.p45);
            s.copy_ad(895, 954);
            s.copy_ad(894, 948);
        }

        s.b[1085] = (p.p56 > 1.0);
        s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });

        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1085]) {
            s.store_offset_scaled(955, 161, 2.0, ((((p.p40) * (2.0))) + (p.p46)));
            s.store_offset_scaled(949, 161, p.p40, p.p47);
            s.store_add(895, 954, 955);
            s.store_add(894, 948, 949);
        }

        s.b[1086] = (p.p56 > 2.0);
        s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });

        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1086]) {
            s.store_offset_scaled(956, 161, 2.0, ((((p.p40) * (2.0))) + (p.p48)));
            s.store_offset_scaled(950, 161, p.p40, p.p49);
            s.store_add_scaled_inputs3_indices(895, 954, 1.0, 955, 1.0, 956, 1.0);
            s.store_add_scaled_inputs3_indices(894, 948, 1.0, 949, 1.0, 950, 1.0);
        }

        s.b[1087] = (p.p56 > 3.0);
        s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });

        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1087]) {
            s.store_offset_scaled(957, 161, 2.0, ((((p.p40) * (2.0))) + (p.p50)));
            s.store_offset_scaled(951, 161, p.p40, p.p51);
            s.store_add_scaled_inputs4_indices(895, 954, 1.0, 955, 1.0, 956, 1.0, 957, 1.0);
            s.store_add_scaled_inputs4_indices(894, 948, 1.0, 949, 1.0, 950, 1.0, 951, 1.0);
        }

        s.b[1088] = (p.p56 > 4.0);
        s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });

        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1088]) {
            s.store_offset_scaled(958, 161, 2.0, ((((p.p40) * (2.0))) + (p.p52)));
            s.store_offset_scaled(952, 161, p.p40, p.p53);
            s.store_add_ad_lhs(895, A::add_scaled_inputs4(s.ad_value(954), 1.0, s.ad_value(955), 1.0, s.ad_value(956), 1.0, s.ad_value(957), 1.0), 958);
            s.store_add_ad_lhs(894, A::add_scaled_inputs4(s.ad_value(948), 1.0, s.ad_value(949), 1.0, s.ad_value(950), 1.0, s.ad_value(951), 1.0), 952);
        }

        s.b[1089] = (p.p56 > 5.0);
        s.store_scalar(1089, if s.b[1089] { 1.0 } else { 0.0 });

        if ((s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) && s.b[1089]) {
            s.store_offset_scaled(959, 161, 2.0, ((((p.p40) * (2.0))) + (p.p54)));
            s.store_offset_scaled(953, 161, p.p40, p.p55);
            s.store_add_ad_lhs(895, A::add(A::add_scaled_inputs4(s.ad_value(954), 1.0, s.ad_value(955), 1.0, s.ad_value(956), 1.0, s.ad_value(957), 1.0), s.ad_value(958)), 959);
            s.store_add_ad_lhs(894, A::add(A::add_scaled_inputs4(s.ad_value(948), 1.0, s.ad_value(949), 1.0, s.ad_value(950), 1.0, s.ad_value(951), 1.0), s.ad_value(952)), 953);
        }

        if (s.b[1081] && (!((((s.b[1076] || s.b[1077]) || s.b[1078]) || s.b[1079]) || s.b[1080]))) {
            s.store_scalar(896, p.p43);
            s.store_scale(893, 895, (p.p102 * (8.8542e-12 * 1.0 / (p.p89))));
        }

        s.store_div_scaled_inputs_mixed_ia(898, 893, 2.0, A::div_scaled_inputs(A::square(s.ad_value(895)), s.v[143], s.ad_value(894), 1.0), 1.0);

        s.store_div_scaled_product_indices(903, 640, 894, (-1.60219e-19), 893, 1.0);

        s.store_div(163, 893, 895);

        s.b[1090] = (p.p61 != 0.0);
        s.store_scalar(1090, if s.b[1090] { 1.0 } else { 0.0 });

        if s.b[1090] {
            s.store_scale(494, 163, (p.p89 * 1.0 / (p.p1528)));
        }

        s.store_offset(158, 895, (-p.p93));

        s.store_offset(159, 895, (-p.p94));

        s.b[1091] = (p.p62 == 5.0);
        s.store_scalar(1091, if s.b[1091] { 1.0 } else { 0.0 });

        if s.b[1091] {
            s.store_offset(160, 158, (-((2.0 * p.p56) * p.p87)));
        }

        if (!s.b[1091]) {
            s.copy_ad(160, 158);
        }

        s.b[1092] = (p.p62 == 5.0);
        s.store_scalar(1092, if s.b[1092] { 1.0 } else { 0.0 });

        s.b[1093] = (p.p61 != 0.0);
        s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });

        s.b[1094] = (s.v[160] <= 0.0);
        s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });

        if ((s.b[1092] && s.b[1093]) && s.b[1094]) {
            s.copy_ad(160, 895);
        }

        s.store_scalar(446, p.p1085);

        s.store_add_scaled_inputs4_offset_indices(641, 635, p.p138, 637, p.p140, 638, p.p141, 639, p.p142, ((p.p137) + ((s.v[636] * p.p139))));

        s.store_add_scaled_inputs4_offset_indices(666, 635, p.p189, 637, p.p191, 638, p.p192, 639, p.p193, ((p.p188) + ((s.v[636] * p.p190))));

        s.store_add_scaled_inputs4_offset_indices(662, 635, p.p201, 637, p.p203, 638, p.p204, 639, p.p205, ((p.p200) + ((s.v[636] * p.p202))));

        s.store_add_scaled_inputs4_offset_indices(663, 635, p.p207, 637, p.p209, 638, p.p210, 639, p.p211, ((p.p206) + ((s.v[636] * p.p208))));

        s.store_add_scaled_inputs4_offset_indices(667, 635, p.p219, 637, p.p221, 638, p.p222, 639, p.p223, ((p.p218) + ((s.v[636] * p.p220))));

        s.store_add_scaled_inputs4_offset_indices(670, 635, p.p225, 637, p.p227, 638, p.p228, 639, p.p229, ((p.p224) + ((s.v[636] * p.p226))));

        s.store_add_scaled_inputs4_offset_indices(671, 635, p.p231, 637, p.p233, 638, p.p234, 639, p.p235, ((p.p230) + ((s.v[636] * p.p232))));

        s.store_add_scaled_inputs4_offset_indices(672, 635, p.p237, 637, p.p239, 638, p.p240, 639, p.p241, ((p.p236) + ((s.v[636] * p.p238))));

        s.store_add_scaled_inputs4_offset_indices(673, 635, p.p243, 637, p.p245, 638, p.p246, 639, p.p247, ((p.p242) + ((s.v[636] * p.p244))));

        s.store_add_scaled_inputs4_offset_indices(674, 635, p.p249, 637, p.p251, 638, p.p252, 639, p.p253, ((p.p248) + ((s.v[636] * p.p250))));

        s.store_add_scaled_inputs4_offset_indices(678, 635, p.p267, 637, p.p269, 638, p.p270, 639, p.p271, ((p.p266) + ((s.v[636] * p.p268))));

        s.store_add_scaled_inputs4_offset_indices(802, 635, p.p273, 637, p.p275, 638, p.p276, 639, p.p277, ((p.p272) + ((s.v[636] * p.p274))));

        s.store_add_scaled_inputs4_offset_indices(803, 635, p.p279, 637, p.p281, 638, p.p282, 639, p.p283, ((p.p278) + ((s.v[636] * p.p280))));

        s.store_add_scaled_inputs4_offset_indices(804, 635, p.p285, 637, p.p287, 638, p.p288, 639, p.p289, ((p.p284) + ((s.v[636] * p.p286))));

        s.store_add_scaled_inputs4_offset_indices(683, 635, p.p297, 637, p.p299, 638, p.p300, 639, p.p301, ((p.p296) + ((s.v[636] * p.p298))));

        s.store_add_scaled_inputs4_offset_indices(684, 635, p.p303, 637, p.p305, 638, p.p306, 639, p.p307, ((p.p302) + ((s.v[636] * p.p304))));

        s.store_add_scaled_inputs4_offset_indices(685, 635, p.p309, 637, p.p311, 638, p.p312, 639, p.p313, ((p.p308) + ((s.v[636] * p.p310))));

        s.store_add_scaled_inputs4_offset_indices(686, 635, p.p315, 637, p.p317, 638, p.p318, 639, p.p319, ((p.p314) + ((s.v[636] * p.p316))));

        s.store_add_scaled_inputs4_offset_indices(687, 635, p.p321, 637, p.p323, 638, p.p324, 639, p.p325, ((p.p320) + ((s.v[636] * p.p322))));

        s.store_add_scaled_inputs4_offset_indices(688, 635, p.p327, 637, p.p329, 638, p.p330, 639, p.p331, ((p.p326) + ((s.v[636] * p.p328))));

        s.store_add_scaled_inputs4_offset_indices(867, 635, p.p333, 637, p.p335, 638, p.p336, 639, p.p337, ((p.p332) + ((s.v[636] * p.p334))));

        s.store_add_scaled_inputs4_offset_indices(868, 635, p.p339, 637, p.p341, 638, p.p342, 639, p.p343, ((p.p338) + ((s.v[636] * p.p340))));

        s.store_add_scaled_inputs4_offset_indices(869, 635, p.p345, 637, p.p347, 638, p.p348, 639, p.p349, ((p.p344) + ((s.v[636] * p.p346))));

        s.store_add_scaled_inputs4_offset_indices(870, 635, p.p351, 637, p.p353, 638, p.p354, 639, p.p355, ((p.p350) + ((s.v[636] * p.p352))));

        s.store_add_scaled_inputs4_offset_indices(654, 635, p.p404, 637, p.p406, 638, p.p407, 639, p.p408, ((p.p403) + ((s.v[636] * p.p405))));

        s.store_add_scaled_inputs4_offset_indices(655, 635, p.p410, 637, p.p412, 638, p.p413, 639, p.p414, ((p.p409) + ((s.v[636] * p.p411))));

        s.store_add_scaled_inputs4_offset_indices(656, 635, p.p416, 637, p.p418, 638, p.p419, 639, p.p420, ((p.p415) + ((s.v[636] * p.p417))));

        s.store_add_scaled_inputs4_offset_indices(661, 635, p.p422, 637, p.p424, 638, p.p425, 639, p.p426, ((p.p421) + ((s.v[636] * p.p423))));

        s.store_add_scaled_inputs4_offset_indices(679, 635, p.p456, 637, p.p458, 638, p.p459, 639, p.p460, ((p.p455) + ((s.v[636] * p.p457))));

        s.store_add_scaled_inputs4_offset_indices(698, 635, p.p468, 637, p.p470, 638, p.p471, 639, p.p472, ((p.p467) + ((s.v[636] * p.p469))));

        s.store_add_scaled_inputs4_offset_indices(702, 635, p.p507, 637, p.p509, 638, p.p510, 639, p.p511, ((p.p506) + ((s.v[636] * p.p508))));

        s.store_add_scaled_inputs4_offset_indices(881, 635, p.p513, 637, p.p515, 638, p.p516, 639, p.p517, ((p.p512) + ((s.v[636] * p.p514))));

        s.store_add_scaled_inputs4_offset_indices(694, 635, p.p480, 637, p.p482, 638, p.p483, 639, p.p484, ((p.p479) + ((s.v[636] * p.p481))));

        s.store_add_scaled_inputs4_offset_indices(695, 635, p.p486, 637, p.p488, 638, p.p489, 639, p.p490, ((p.p485) + ((s.v[636] * p.p487))));

        s.store_add_scaled_inputs4_offset_indices(696, 635, p.p519, 637, p.p521, 638, p.p522, 639, p.p523, ((p.p518) + ((s.v[636] * p.p520))));

        s.store_add_scaled_inputs4_offset_indices(697, 635, p.p525, 637, p.p527, 638, p.p528, 639, p.p529, ((p.p524) + ((s.v[636] * p.p526))));

        s.store_add_scaled_inputs4_offset_indices(657, 635, p.p493, 637, p.p495, 638, p.p496, 639, p.p497, ((p.p492) + ((s.v[636] * p.p494))));

        s.store_add_scaled_inputs4_offset_indices(790, 635, p.p532, 637, p.p534, 638, p.p535, 639, p.p536, ((p.p531) + ((s.v[636] * p.p533))));

        s.store_add_scaled_inputs4_offset_indices(700, 635, p.p544, 637, p.p546, 638, p.p547, 639, p.p548, ((p.p543) + ((s.v[636] * p.p545))));

        s.store_add_scaled_inputs4_offset_indices(704, 635, p.p606, 637, p.p608, 638, p.p609, 639, p.p610, ((p.p605) + ((s.v[636] * p.p607))));

        s.store_add_scaled_inputs4_offset_indices(707, 635, p.p624, 637, p.p626, 638, p.p627, 639, p.p628, ((p.p623) + ((s.v[636] * p.p625))));

        s.store_add_scaled_inputs4_offset_indices(703, 635, p.p630, 637, p.p632, 638, p.p633, 639, p.p634, ((p.p629) + ((s.v[636] * p.p631))));

        s.store_add_scaled_inputs4_offset_indices(807, 635, p.p642, 637, p.p644, 638, p.p645, 639, p.p646, ((p.p641) + ((s.v[636] * p.p643))));

        s.store_add_scaled_inputs4_offset_indices(811, 635, p.p678, 637, p.p680, 638, p.p681, 639, p.p682, ((p.p677) + ((s.v[636] * p.p679))));

        s.store_add_scaled_inputs4_offset_indices(812, 635, p.p690, 637, p.p692, 638, p.p693, 639, p.p694, ((p.p689) + ((s.v[636] * p.p691))));

        s.store_add_scaled_inputs4_offset_indices(814, 635, p.p708, 637, p.p710, 638, p.p711, 639, p.p712, ((p.p707) + ((s.v[636] * p.p709))));

        s.store_add_scaled_inputs4_offset_indices(325, 635, p.p714, 637, p.p716, 638, p.p717, 639, p.p718, ((p.p713) + ((s.v[636] * p.p715))));

        s.store_add_scaled_inputs4_offset_indices(326, 635, p.p720, 637, p.p722, 638, p.p723, 639, p.p724, ((p.p719) + ((s.v[636] * p.p721))));

        s.store_add_scaled_inputs4_offset_indices(328, 635, p.p726, 637, p.p728, 638, p.p729, 639, p.p730, ((p.p725) + ((s.v[636] * p.p727))));

        s.store_add_scaled_inputs4_offset_indices(329, 635, p.p732, 637, p.p734, 638, p.p735, 639, p.p736, ((p.p731) + ((s.v[636] * p.p733))));

        s.store_add_scaled_inputs4_offset_indices(792, 635, p.p1027, 637, p.p1029, 638, p.p1030, 639, p.p1031, ((p.p1025) + ((s.v[636] * p.p1028))));

        s.store_add_scaled_inputs4_offset_indices(793, 635, p.p1039, 637, p.p1041, 638, p.p1042, 639, p.p1043, ((p.p1038) + ((s.v[636] * p.p1040))));

        s.store_add_scaled_inputs4_offset_indices(794, 635, p.p1045, 637, p.p1047, 638, p.p1048, 639, p.p1049, ((p.p1044) + ((s.v[636] * p.p1046))));

        s.store_add_scaled_inputs4_offset_indices(798, 635, p.p1051, 637, p.p1053, 638, p.p1054, 639, p.p1055, ((p.p1050) + ((s.v[636] * p.p1052))));

        s.store_add_scaled_inputs4_offset_indices(800, 635, p.p1057, 637, p.p1059, 638, p.p1060, 639, p.p1061, ((p.p1056) + ((s.v[636] * p.p1058))));

        s.store_add_scaled_inputs4_offset_indices(799, 635, p.p1063, 637, p.p1065, 638, p.p1066, 639, p.p1067, ((p.p1062) + ((s.v[636] * p.p1064))));

        s.store_add_scaled_inputs4_offset_indices(801, 635, p.p1069, 637, p.p1071, 638, p.p1072, 639, p.p1073, ((p.p1068) + ((s.v[636] * p.p1070))));

        s.store_add_scaled_inputs4_offset_indices(709, 635, p.p926, 637, p.p928, 638, p.p929, 639, p.p930, ((p.p925) + ((s.v[636] * p.p927))));

        s.store_add_scaled_inputs4_offset_indices(853, 635, p.p932, 637, p.p934, 638, p.p935, 639, p.p936, ((p.p931) + ((s.v[636] * p.p933))));

        s.store_add_scaled_inputs4_offset_indices(852, 635, p.p938, 637, p.p940, 638, p.p941, 639, p.p942, ((p.p937) + ((s.v[636] * p.p939))));

        s.store_add_scaled_inputs4_offset_indices(712, 635, p.p950, 637, p.p952, 638, p.p953, 639, p.p954, ((p.p949) + ((s.v[636] * p.p951))));

        s.store_add_scaled_inputs4_offset_indices(711, 635, p.p944, 637, p.p946, 638, p.p947, 639, p.p948, ((p.p943) + ((s.v[636] * p.p945))));

        s.store_add_scaled_inputs4_offset_indices(713, 635, p.p956, 637, p.p958, 638, p.p959, 639, p.p960, ((p.p955) + ((s.v[636] * p.p957))));

        s.store_add_scaled_inputs4_offset_indices(714, 635, p.p986, 637, p.p988, 638, p.p989, 639, p.p990, ((p.p985) + ((s.v[636] * p.p987))));

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs4_offset_indices(716, 635, p.p992, 637, p.p994, 638, p.p995, 639, p.p996, ((p.p991) + ((s.v[636] * p.p993))));

        s.store_add_scaled_inputs4_offset_indices(719, 635, p.p1010, 637, p.p1012, 638, p.p1013, 639, p.p1014, ((p.p1009) + ((s.v[636] * p.p1011))));

        s.store_add_scaled_inputs4_offset_indices(720, 635, p.p1016, 637, p.p1018, 638, p.p1019, 639, p.p1020, ((p.p1015) + ((s.v[636] * p.p1017))));

        s.store_add_scaled_inputs4_offset_indices(721, 635, p.p1120, 637, p.p1122, 638, p.p1123, 639, p.p1124, ((p.p1119) + ((s.v[636] * p.p1121))));

        s.store_add_scaled_inputs4_offset_indices(722, 635, p.p1126, 637, p.p1128, 638, p.p1129, 639, p.p1130, ((p.p1125) + ((s.v[636] * p.p1127))));

        s.store_add_scaled_inputs4_offset_indices(723, 635, p.p1132, 637, p.p1134, 638, p.p1135, 639, p.p1136, ((p.p1131) + ((s.v[636] * p.p1133))));

        s.store_add_scaled_inputs4_offset_indices(724, 635, p.p1138, 637, p.p1140, 638, p.p1141, 639, p.p1142, ((p.p1137) + ((s.v[636] * p.p1139))));

        s.store_add_scaled_inputs4_offset_indices(725, 635, p.p1144, 637, p.p1146, 638, p.p1147, 639, p.p1148, ((p.p1143) + ((s.v[636] * p.p1145))));

        s.store_add_scaled_inputs4_offset_indices(726, 635, p.p1150, 637, p.p1152, 638, p.p1153, 639, p.p1154, ((p.p1149) + ((s.v[636] * p.p1151))));

        s.store_add_scaled_inputs4_offset_indices(727, 635, p.p1156, 637, p.p1158, 638, p.p1159, 639, p.p1160, ((p.p1155) + ((s.v[636] * p.p1157))));

        s.store_add_scaled_inputs4_offset_indices(728, 635, p.p1162, 637, p.p1164, 638, p.p1165, 639, p.p1166, ((p.p1161) + ((s.v[636] * p.p1163))));

        s.store_add_scaled_inputs4_offset_indices(729, 635, p.p1168, 637, p.p1170, 638, p.p1171, 639, p.p1172, ((p.p1167) + ((s.v[636] * p.p1169))));

        s.store_add_scaled_inputs4_offset_indices(730, 635, p.p1174, 637, p.p1176, 638, p.p1177, 639, p.p1178, ((p.p1173) + ((s.v[636] * p.p1175))));

        s.store_add_scaled_inputs4_offset_indices(731, 635, p.p1180, 637, p.p1182, 638, p.p1183, 639, p.p1184, ((p.p1179) + ((s.v[636] * p.p1181))));

        s.store_add_scaled_inputs4_offset_indices(732, 635, p.p1186, 637, p.p1188, 638, p.p1189, 639, p.p1190, ((p.p1185) + ((s.v[636] * p.p1187))));

        s.store_add_scaled_inputs4_offset_indices(733, 635, p.p1192, 637, p.p1194, 638, p.p1195, 639, p.p1196, ((p.p1191) + ((s.v[636] * p.p1193))));

        s.store_add_scaled_inputs4_offset_indices(734, 635, p.p1198, 637, p.p1200, 638, p.p1201, 639, p.p1202, ((p.p1197) + ((s.v[636] * p.p1199))));

        s.store_add_scaled_inputs4_offset_indices(735, 635, p.p1204, 637, p.p1206, 638, p.p1207, 639, p.p1208, ((p.p1203) + ((s.v[636] * p.p1205))));

        s.store_add_scaled_inputs4_offset_indices(736, 635, p.p1210, 637, p.p1212, 638, p.p1213, 639, p.p1214, ((p.p1209) + ((s.v[636] * p.p1211))));

        s.store_add_scaled_inputs4_offset_indices(737, 635, p.p1216, 637, p.p1218, 638, p.p1219, 639, p.p1220, ((p.p1215) + ((s.v[636] * p.p1217))));

        s.store_add_scaled_inputs4_offset_indices(738, 635, p.p1222, 637, p.p1224, 638, p.p1225, 639, p.p1226, ((p.p1221) + ((s.v[636] * p.p1223))));

        s.store_add_scaled_inputs4_offset_indices(739, 635, p.p1228, 637, p.p1230, 638, p.p1231, 639, p.p1232, ((p.p1227) + ((s.v[636] * p.p1229))));

        s.store_add_scaled_inputs4_offset_indices(740, 635, p.p1234, 637, p.p1236, 638, p.p1237, 639, p.p1238, ((p.p1233) + ((s.v[636] * p.p1235))));

        s.store_add_scaled_inputs4_offset_indices(743, 635, p.p1240, 637, p.p1242, 638, p.p1243, 639, p.p1244, ((p.p1239) + ((s.v[636] * p.p1241))));

        s.store_add_scaled_inputs4_offset_indices(744, 635, p.p1246, 637, p.p1248, 638, p.p1249, 639, p.p1250, ((p.p1245) + ((s.v[636] * p.p1247))));

        s.store_add_scaled_inputs4_offset_indices(745, 635, p.p1252, 637, p.p1254, 638, p.p1255, 639, p.p1256, ((p.p1251) + ((s.v[636] * p.p1253))));

        s.store_add_scaled_inputs4_offset_indices(746, 635, p.p1258, 637, p.p1260, 638, p.p1261, 639, p.p1262, ((p.p1257) + ((s.v[636] * p.p1259))));

        s.store_add_scaled_inputs4_offset_indices(742, 635, p.p1264, 637, p.p1266, 638, p.p1267, 639, p.p1268, ((p.p1263) + ((s.v[636] * p.p1265))));

        s.store_add_scaled_inputs4_offset_indices(747, 635, p.p1270, 637, p.p1272, 638, p.p1273, 639, p.p1274, ((p.p1269) + ((s.v[636] * p.p1271))));

        s.store_add_scaled_inputs4_offset_indices(748, 635, p.p1276, 637, p.p1278, 638, p.p1279, 639, p.p1280, ((p.p1275) + ((s.v[636] * p.p1277))));

        s.store_add_scaled_inputs4_offset_indices(749, 635, p.p1282, 637, p.p1284, 638, p.p1285, 639, p.p1286, ((p.p1281) + ((s.v[636] * p.p1283))));

        s.store_add_scaled_inputs4_offset_indices(750, 635, p.p1288, 637, p.p1290, 638, p.p1291, 639, p.p1292, ((p.p1287) + ((s.v[636] * p.p1289))));

        s.store_add_scaled_inputs4_offset_indices(751, 635, p.p1294, 637, p.p1296, 638, p.p1297, 639, p.p1298, ((p.p1293) + ((s.v[636] * p.p1295))));

        s.store_add_scaled_inputs4_offset_indices(752, 635, p.p1330, 637, p.p1332, 638, p.p1333, 639, p.p1334, ((p.p1329) + ((s.v[636] * p.p1331))));

        s.store_add_scaled_inputs4_offset_indices(753, 635, p.p1336, 637, p.p1338, 638, p.p1339, 639, p.p1340, ((p.p1335) + ((s.v[636] * p.p1337))));

        s.store_add_scaled_inputs4_offset_indices(754, 635, p.p1342, 637, p.p1344, 638, p.p1345, 639, p.p1346, ((p.p1341) + ((s.v[636] * p.p1343))));

        s.store_add_scaled_inputs4_offset_indices(755, 635, p.p1348, 637, p.p1350, 638, p.p1351, 639, p.p1352, ((p.p1347) + ((s.v[636] * p.p1349))));

        s.store_add_scaled_inputs4_offset_indices(761, 635, p.p1300, 637, p.p1302, 638, p.p1303, 639, p.p1304, ((p.p1299) + ((s.v[636] * p.p1301))));

        s.store_add_scaled_inputs4_offset_indices(762, 635, p.p1306, 637, p.p1308, 638, p.p1309, 639, p.p1310, ((p.p1305) + ((s.v[636] * p.p1307))));

        s.store_add_scaled_inputs4_offset_indices(763, 635, p.p1312, 637, p.p1314, 638, p.p1315, 639, p.p1316, ((p.p1311) + ((s.v[636] * p.p1313))));

        s.store_add_scaled_inputs4_offset_indices(764, 635, p.p1318, 637, p.p1320, 638, p.p1321, 639, p.p1322, ((p.p1317) + ((s.v[636] * p.p1319))));

        s.store_add_scaled_inputs4_offset_indices(765, 635, p.p1324, 637, p.p1326, 638, p.p1327, 639, p.p1328, ((p.p1323) + ((s.v[636] * p.p1325))));

        s.store_add_scaled_inputs4_offset_indices(766, 635, p.p1354, 637, p.p1356, 638, p.p1357, 639, p.p1358, ((p.p1353) + ((s.v[636] * p.p1355))));

        s.store_add_scaled_inputs4_offset_indices(767, 635, p.p1360, 637, p.p1362, 638, p.p1363, 639, p.p1364, ((p.p1359) + ((s.v[636] * p.p1361))));

        s.store_add_scaled_inputs4_offset_indices(768, 635, p.p1366, 637, p.p1368, 638, p.p1369, 639, p.p1370, ((p.p1365) + ((s.v[636] * p.p1367))));

        s.store_add_scaled_inputs4_offset_indices(769, 635, p.p1372, 637, p.p1374, 638, p.p1375, 639, p.p1376, ((p.p1371) + ((s.v[636] * p.p1373))));

        s.store_add_scaled_inputs4_offset_indices(775, 635, p.p1445, 637, p.p1447, 638, p.p1448, 639, p.p1449, ((p.p1444) + ((s.v[636] * p.p1446))));

        s.store_add_scaled_inputs4_offset_indices(776, 635, p.p1451, 637, p.p1453, 638, p.p1454, 639, p.p1455, ((p.p1450) + ((s.v[636] * p.p1452))));

        s.store_add_scaled_inputs4_offset_indices(777, 635, p.p1463, 637, p.p1465, 638, p.p1466, 639, p.p1467, ((p.p1462) + ((s.v[636] * p.p1464))));

        s.store_add_scaled_inputs4_offset_indices(778, 635, p.p1469, 637, p.p1471, 638, p.p1472, 639, p.p1473, ((p.p1468) + ((s.v[636] * p.p1470))));

        s.store_add_scaled_inputs4_offset_indices(779, 635, p.p1457, 637, p.p1459, 638, p.p1460, 639, p.p1461, ((p.p1456) + ((s.v[636] * p.p1458))));

        s.store_add_scaled_inputs4_offset_indices(780, 635, p.p1475, 637, p.p1477, 638, p.p1478, 639, p.p1479, ((p.p1474) + ((s.v[636] * p.p1476))));

        s.store_add_scaled_inputs4_offset_indices(781, 635, p.p1481, 637, p.p1483, 638, p.p1484, 639, p.p1485, ((p.p1480) + ((s.v[636] * p.p1482))));

        s.store_add_scaled_inputs4_offset_indices(782, 635, p.p1487, 637, p.p1489, 638, p.p1490, 639, p.p1491, ((p.p1486) + ((s.v[636] * p.p1488))));

        s.store_add_scaled_inputs4_offset_indices(783, 635, p.p1493, 637, p.p1495, 638, p.p1496, 639, p.p1497, ((p.p1492) + ((s.v[636] * p.p1494))));

        s.store_add_scaled_inputs4_offset_indices(784, 635, p.p1499, 637, p.p1501, 638, p.p1502, 639, p.p1503, ((p.p1498) + ((s.v[636] * p.p1500))));

        s.store_add_scaled_inputs4_offset_indices(785, 635, p.p1505, 637, p.p1507, 638, p.p1508, 639, p.p1509, ((p.p1504) + ((s.v[636] * p.p1506))));

        s.store_add_scaled_inputs4_offset_indices(786, 635, p.p1511, 637, p.p1513, 638, p.p1514, 639, p.p1515, ((p.p1510) + ((s.v[636] * p.p1512))));

        s.store_add_scaled_inputs4_offset_indices(787, 635, p.p1517, 637, p.p1519, 638, p.p1520, 639, p.p1521, ((p.p1516) + ((s.v[636] * p.p1518))));

        s.store_add_scaled_inputs4_offset_indices(788, 635, p.p1523, 637, p.p1525, 638, p.p1526, 639, p.p1527, ((p.p1522) + ((s.v[636] * p.p1524))));

        s.store_add_scaled_inputs4_offset_indices(789, 635, p.p1763, 637, p.p1765, 638, p.p1766, 639, p.p1767, ((p.p1762) + ((s.v[636] * p.p1764))));

        s.store_add_scaled_inputs4_offset_indices(643, 635, p.p1531, 637, p.p1533, 638, p.p1534, 639, p.p1535, ((p.p1530) + ((s.v[636] * p.p1532))));

        s.store_add_scaled_inputs4_offset_indices(642, 635, p.p1537, 637, p.p1539, 638, p.p1540, 639, p.p1541, ((p.p1536) + ((s.v[636] * p.p1538))));

        s.store_add_scaled_inputs4_offset_indices(644, 635, p.p29, 637, p.p31, 638, p.p32, 639, p.p33, ((p.p28) + ((s.v[636] * p.p30))));

        s.store_add_scaled_inputs4_offset_indices(645, 635, p.p35, 637, p.p37, 638, p.p38, 639, p.p39, ((p.p34) + ((s.v[636] * p.p36))));

        s.store_add_scaled_inputs4_offset_indices(648, 635, p.p1548, 637, p.p1550, 638, p.p1551, 639, p.p1552, ((p.p1547) + ((s.v[636] * p.p1549))));

        s.store_add_scaled_inputs4_offset_indices(649, 635, p.p1554, 637, p.p1556, 638, p.p1557, 639, p.p1558, ((p.p1553) + ((s.v[636] * p.p1555))));

        s.store_add_scaled_inputs4_offset_indices(650, 635, p.p1560, 637, p.p1562, 638, p.p1563, 639, p.p1564, ((p.p1559) + ((s.v[636] * p.p1561))));

        s.store_add_scaled_inputs4_offset_indices(651, 635, p.p1566, 637, p.p1568, 638, p.p1569, 639, p.p1570, ((p.p1565) + ((s.v[636] * p.p1567))));

        s.store_add_scaled_inputs4_offset_indices(652, 635, p.p1572, 637, p.p1574, 638, p.p1575, 639, p.p1576, ((p.p1571) + ((s.v[636] * p.p1573))));

        s.store_add_scaled_inputs4_offset_indices(653, 635, p.p1578, 637, p.p1580, 638, p.p1581, 639, p.p1582, ((p.p1577) + ((s.v[636] * p.p1579))));

        s.store_add_scaled_inputs4_offset_indices(865, 635, p.p1657, 637, p.p1659, 638, p.p1660, 639, p.p1661, ((p.p1656) + ((s.v[636] * p.p1658))));

        s.store_add_scaled_inputs4_offset_indices(866, 635, p.p1663, 637, p.p1665, 638, p.p1666, 639, p.p1667, ((p.p1662) + ((s.v[636] * p.p1664))));

        s.store_add_scaled_inputs4_offset_indices(836, 635, p.p738, 637, p.p740, 638, p.p741, 639, p.p742, ((p.p737) + ((s.v[636] * p.p739))));

        s.store_add_scaled_inputs4_offset_indices(837, 635, p.p756, 637, p.p758, 638, p.p759, 639, p.p760, ((p.p755) + ((s.v[636] * p.p757))));

        s.store_add_scaled_inputs4_offset_indices(838, 635, p.p768, 637, p.p770, 638, p.p771, 639, p.p772, ((p.p767) + ((s.v[636] * p.p769))));

        s.store_add_scaled_inputs4_offset_indices(842, 635, p.p786, 637, p.p788, 638, p.p789, 639, p.p790, ((p.p785) + ((s.v[636] * p.p787))));

        s.store_add_scaled_inputs4_offset_indices(823, 635, p.p792, 637, p.p794, 638, p.p795, 639, p.p796, ((p.p791) + ((s.v[636] * p.p793))));

        s.store_add_scaled_inputs4_offset_indices(824, 635, p.p810, 637, p.p812, 638, p.p813, 639, p.p814, ((p.p809) + ((s.v[636] * p.p811))));

        s.store_add_scaled_inputs4_offset_indices(847, 635, p.p822, 637, p.p824, 638, p.p825, 639, p.p826, ((p.p821) + ((s.v[636] * p.p823))));

        s.store_add_scaled_inputs4_offset_indices(830, 635, p.p846, 637, p.p848, 638, p.p849, 639, p.p850, ((p.p845) + ((s.v[636] * p.p847))));

        s.store_add_scaled_inputs4_offset_indices(831, 635, p.p864, 637, p.p866, 638, p.p867, 639, p.p868, ((p.p863) + ((s.v[636] * p.p865))));

        s.store_add_scaled_inputs4_offset_indices(834, 635, p.p876, 637, p.p878, 638, p.p879, 639, p.p880, ((p.p875) + ((s.v[636] * p.p877))));

        s.store_add_scaled_inputs4_offset_indices(835, 635, p.p882, 637, p.p884, 638, p.p885, 639, p.p886, ((p.p881) + ((s.v[636] * p.p883))));

        s.store_add_scaled_inputs4_offset_indices(848, 635, p.p576, 637, p.p578, 638, p.p579, 639, p.p580, ((p.p575) + ((s.v[636] * p.p577))));

        s.store_add_scaled_inputs4_offset_indices(849, 635, p.p556, 637, p.p558, 638, p.p559, 639, p.p560, ((p.p555) + ((s.v[636] * p.p557))));

        s.store_add_scaled_inputs4_offset_indices(850, 635, p.p569, 637, p.p571, 638, p.p572, 639, p.p573, ((p.p568) + ((s.v[636] * p.p570))));

        s.store_add_scaled_inputs4_offset_indices(854, 635, p.p962, 637, p.p964, 638, p.p965, 639, p.p966, ((p.p961) + ((s.v[636] * p.p963))));

        s.store_add_scaled_inputs4_offset_indices(855, 635, p.p968, 637, p.p970, 638, p.p971, 639, p.p972, ((p.p967) + ((s.v[636] * p.p969))));

        s.store_add_scaled_inputs4_offset_indices(856, 635, p.p974, 637, p.p976, 638, p.p977, 639, p.p978, ((p.p973) + ((s.v[636] * p.p975))));

        s.store_add_scaled_inputs4_offset_indices(857, 635, p.p980, 637, p.p982, 638, p.p983, 639, p.p984, ((p.p979) + ((s.v[636] * p.p981))));

        s.store_add_scaled_inputs4_offset_indices(858, 635, p.p1742, 637, p.p1744, 638, p.p1745, 639, p.p1746, ((p.p1741) + ((s.v[636] * p.p1743))));

        s.store_add_scaled_inputs4_offset_indices(859, 635, p.p1751, 637, p.p1753, 638, p.p1754, 639, p.p1755, ((p.p1750) + ((s.v[636] * p.p1752))));

        s.store_add_scaled_inputs4_offset_indices(860, 635, p.p1757, 637, p.p1759, 638, p.p1760, 639, p.p1761, ((p.p1756) + ((s.v[636] * p.p1758))));

        s.store_add_scaled_inputs4_offset_indices(862, 635, p.p1769, 637, p.p1771, 638, p.p1772, 639, p.p1773, ((p.p1768) + ((s.v[636] * p.p1770))));

        s.store_add_scaled_inputs4_offset_indices(863, 635, p.p1775, 637, p.p1777, 638, p.p1778, 639, p.p1779, ((p.p1774) + ((s.v[636] * p.p1776))));

        s.store_add_scaled_inputs4_offset_indices(681, 635, p.p177, 637, p.p179, 638, p.p180, 639, p.p181, ((p.p176) + ((s.v[636] * p.p178))));

        s.store_add_scaled_inputs4_offset_indices(682, 635, p.p183, 637, p.p185, 638, p.p186, 639, p.p187, ((p.p182) + ((s.v[636] * p.p184))));

        s.store_add_scaled_inputs4_offset_indices(574, 635, p.p1690, 637, p.p1692, 638, p.p1693, 639, p.p1694, ((p.p1689) + ((s.v[636] * p.p1691))));

        s.store_add_scaled_inputs4_offset_indices(576, 635, p.p1702, 637, p.p1704, 638, p.p1705, 639, p.p1706, ((p.p1701) + ((s.v[636] * p.p1703))));

        s.store_add_scaled_inputs4_offset_indices(575, 635, p.p1696, 637, p.p1698, 638, p.p1699, 639, p.p1700, ((p.p1695) + ((s.v[636] * p.p1697))));

        s.b[1096] = (p.p61 != 0.0);
        s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });

        if s.b[1096] {
            s.store_add_scaled_inputs4_offset_indices(689, 635, p.p357, 637, p.p359, 638, p.p360, 639, p.p361, ((p.p356) + ((s.v[636] * p.p358))));
            s.store_add_scaled_inputs4_offset_indices(690, 635, p.p363, 637, p.p365, 638, p.p366, 639, p.p367, ((p.p362) + ((s.v[636] * p.p364))));
            s.store_add_scaled_inputs4_offset_indices(691, 635, p.p369, 637, p.p371, 638, p.p372, 639, p.p373, ((p.p368) + ((s.v[636] * p.p370))));
            s.store_add_scaled_inputs4_offset_indices(809, 635, p.p660, 637, p.p662, 638, p.p663, 639, p.p664, ((p.p659) + ((s.v[636] * p.p661))));
            s.store_add_scaled_inputs4_offset_indices(828, 635, p.p828, 637, p.p830, 638, p.p831, 639, p.p832, ((p.p827) + ((s.v[636] * p.p829))));
        }

        s.b[1097] = (p.p61 == 2.0);
        s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });

        if (s.b[1096] && s.b[1097]) {
            s.store_add_scaled_inputs4_offset_indices(871, 635, p.p387, 637, p.p389, 638, p.p390, 639, p.p391, ((p.p386) + ((s.v[636] * p.p388))));
            s.store_add_scaled_inputs4_offset_indices(872, 635, p.p393, 637, p.p395, 638, p.p396, 639, p.p397, ((p.p392) + ((s.v[636] * p.p394))));
            s.store_add_scaled_inputs4_offset_indices(692, 635, p.p375, 637, p.p377, 638, p.p378, 639, p.p379, ((p.p374) + ((s.v[636] * p.p376))));
            s.store_add_scaled_inputs4_offset_indices(693, 635, p.p381, 637, p.p383, 638, p.p384, 639, p.p385, ((p.p380) + ((s.v[636] * p.p382))));
        }

        s.b[1098] = (((p.p70 == 2.0) || (p.p70 == 3.0)) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0)));
        s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });

        if (s.b[1096] && s.b[1098]) {
            s.store_add_scaled_inputs4_offset_indices(756, 635, p.p1378, 637, p.p1380, 638, p.p1381, 639, p.p1382, ((p.p1377) + ((s.v[636] * p.p1379))));
            s.store_add_scaled_inputs4_offset_indices(757, 635, p.p1384, 637, p.p1386, 638, p.p1387, 639, p.p1388, ((p.p1383) + ((s.v[636] * p.p1385))));
            s.store_add_scaled_inputs4_offset_indices(758, 635, p.p1390, 637, p.p1392, 638, p.p1393, 639, p.p1394, ((p.p1389) + ((s.v[636] * p.p1391))));
            s.store_add_scaled_inputs4_offset_indices(759, 635, p.p1396, 637, p.p1398, 638, p.p1399, 639, p.p1400, ((p.p1395) + ((s.v[636] * p.p1397))));
            s.store_add_scaled_inputs4_offset_indices(760, 635, p.p1402, 637, p.p1404, 638, p.p1405, 639, p.p1406, ((p.p1401) + ((s.v[636] * p.p1403))));
            s.store_add_scaled_inputs4_offset_indices(770, 635, p.p1408, 637, p.p1410, 638, p.p1411, 639, p.p1412, ((p.p1407) + ((s.v[636] * p.p1409))));
            s.store_add_scaled_inputs4_offset_indices(771, 635, p.p1414, 637, p.p1416, 638, p.p1417, 639, p.p1418, ((p.p1413) + ((s.v[636] * p.p1415))));
            s.store_add_scaled_inputs4_offset_indices(772, 635, p.p1420, 637, p.p1422, 638, p.p1423, 639, p.p1424, ((p.p1419) + ((s.v[636] * p.p1421))));
            s.store_add_scaled_inputs4_offset_indices(773, 635, p.p1426, 637, p.p1428, 638, p.p1429, 639, p.p1430, ((p.p1425) + ((s.v[636] * p.p1427))));
            s.store_add_scaled_inputs4_offset_indices(774, 635, p.p1432, 637, p.p1434, 638, p.p1435, 639, p.p1436, ((p.p1431) + ((s.v[636] * p.p1433))));
        }

        s.b[1099] = (p.p66 != 0.0);
        s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });

        if s.b[1099] {
            s.store_add_scaled_inputs4_offset_indices(665, 635, p.p213, 637, p.p215, 638, p.p216, 639, p.p217, ((p.p212) + ((s.v[636] * p.p214))));
            s.store_add_scaled_inputs4_offset_indices(668, 635, p.p195, 637, p.p197, 638, p.p198, 639, p.p199, ((p.p194) + ((s.v[636] * p.p196))));
            s.store_add_scaled_inputs4_offset_indices(677, 635, p.p255, 637, p.p257, 638, p.p258, 639, p.p259, ((p.p254) + ((s.v[636] * p.p256))));
            s.store_add_scaled_inputs4_offset_indices(699, 635, p.p474, 637, p.p476, 638, p.p477, 639, p.p478, ((p.p473) + ((s.v[636] * p.p475))));
            s.store_add_scaled_inputs4_offset_indices(791, 635, p.p538, 637, p.p540, 638, p.p541, 639, p.p542, ((p.p537) + ((s.v[636] * p.p539))));
            s.store_add_scaled_inputs4_offset_indices(701, 635, p.p550, 637, p.p552, 638, p.p553, 639, p.p554, ((p.p549) + ((s.v[636] * p.p551))));
            s.store_add_scaled_inputs4_offset_indices(715, 635, p.p998, 637, p.p1000, 638, p.p1001, 639, p.p1002, ((p.p997) + ((s.v[636] * p.p999))));
            s.store_add_scaled_inputs4_offset_indices(717, 635, p.p1004, 637, p.p1006, 638, p.p1007, 639, p.p1008, ((p.p1003) + ((s.v[636] * p.p1005))));
            s.store_add_scaled_inputs4_offset_indices(796, 635, p.p1033, 637, p.p1035, 638, p.p1036, 639, p.p1037, ((p.p1032) + ((s.v[636] * p.p1034))));
            s.store_add_scaled_inputs4_offset_indices(806, 635, p.p291, 637, p.p293, 638, p.p294, 639, p.p295, ((p.p290) + ((s.v[636] * p.p292))));
            s.store_add_scaled_inputs4_offset_indices(680, 635, p.p462, 637, p.p464, 638, p.p465, 639, p.p466, ((p.p461) + ((s.v[636] * p.p463))));
            s.store_add_scaled_inputs4_offset_indices(658, 635, p.p501, 637, p.p503, 638, p.p504, 639, p.p505, ((p.p500) + ((s.v[636] * p.p502))));
            s.store_add_scaled_inputs4_offset_indices(706, 635, p.p612, 637, p.p614, 638, p.p615, 639, p.p616, ((p.p611) + ((s.v[636] * p.p613))));
            s.store_add_scaled_inputs4_offset_indices(815, 635, p.p648, 637, p.p650, 638, p.p651, 639, p.p652, ((p.p647) + ((s.v[636] * p.p649))));
            s.store_add_scaled_inputs4_offset_indices(710, 635, p.p636, 637, p.p638, 638, p.p639, 639, p.p640, ((p.p635) + ((s.v[636] * p.p637))));
            s.store_add_scaled_inputs4_offset_indices(816, 635, p.p684, 637, p.p686, 638, p.p687, 639, p.p688, ((p.p683) + ((s.v[636] * p.p685))));
            s.store_add_scaled_inputs4_offset_indices(818, 635, p.p696, 637, p.p698, 638, p.p699, 639, p.p700, ((p.p695) + ((s.v[636] * p.p697))));
            s.store_add_scaled_inputs4_offset_indices(845, 635, p.p744, 637, p.p746, 638, p.p747, 639, p.p748, ((p.p743) + ((s.v[636] * p.p745))));
            s.store_add_scaled_inputs4_offset_indices(846, 635, p.p774, 637, p.p776, 638, p.p777, 639, p.p778, ((p.p773) + ((s.v[636] * p.p775))));
            s.store_add_scaled_inputs4_offset_indices(825, 635, p.p798, 637, p.p800, 638, p.p801, 639, p.p802, ((p.p797) + ((s.v[636] * p.p799))));
            s.store_add_scaled_inputs4_offset_indices(844, 635, p.p852, 637, p.p854, 638, p.p855, 639, p.p856, ((p.p851) + ((s.v[636] * p.p853))));
            s.store_add_scaled_inputs4_offset_indices(851, 635, p.p563, 637, p.p565, 638, p.p566, 639, p.p567, ((p.p562) + ((s.v[636] * p.p564))));
        }

        s.b[1100] = (p.p61 != 0.0);
        s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });

        if (s.b[1099] && s.b[1100]) {
            s.store_add_scaled_inputs4_offset_indices(817, 635, p.p666, 637, p.p668, 638, p.p669, 639, p.p670, ((p.p665) + ((s.v[636] * p.p667))));
            s.store_add_scaled_inputs4_offset_indices(843, 635, p.p834, 637, p.p836, 638, p.p837, 639, p.p838, ((p.p833) + ((s.v[636] * p.p835))));
        }

        s.b[1101] = (p.p67 == 1.0);
        s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });

        if s.b[1101] {
            s.store_add_scaled_inputs4_offset_indices(705, 635, p.p618, 637, p.p620, 638, p.p621, 639, p.p622, ((p.p617) + ((s.v[636] * p.p619))));
        }

        s.b[1102] = (p.p582 != 0.0);
        s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });

        if (s.b[1101] && s.b[1102]) {
            s.store_scale(705, 705, (1.0 + ((p.p582 / p.p5) * (if (!((1.0 + (p.p5 / p.p585)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p585)) > 1e-38) { (((1.0 + (p.p5 / p.p585))) as f64).ln() } else { 0.0 }) }))));
        }

        if s.b[1101] {
            s.store_add_scaled_inputs4_offset_indices(808, 635, p.p654, 637, p.p656, 638, p.p657, 639, p.p658, ((p.p653) + ((s.v[636] * p.p655))));
            s.store_add_scaled_inputs4_offset_indices(813, 635, p.p702, 637, p.p704, 638, p.p705, 639, p.p706, ((p.p701) + ((s.v[636] * p.p703))));
            s.store_add_scaled_inputs4_offset_indices(839, 635, p.p750, 637, p.p752, 638, p.p753, 639, p.p754, ((p.p749) + ((s.v[636] * p.p751))));
            s.store_add_scaled_inputs4_offset_indices(840, 635, p.p762, 637, p.p764, 638, p.p765, 639, p.p766, ((p.p761) + ((s.v[636] * p.p763))));
            s.store_add_scaled_inputs4_offset_indices(841, 635, p.p780, 637, p.p782, 638, p.p783, 639, p.p784, ((p.p779) + ((s.v[636] * p.p781))));
            s.store_add_scaled_inputs4_offset_indices(826, 635, p.p804, 637, p.p806, 638, p.p807, 639, p.p808, ((p.p803) + ((s.v[636] * p.p805))));
            s.store_add_scaled_inputs4_offset_indices(827, 635, p.p816, 637, p.p818, 638, p.p819, 639, p.p820, ((p.p815) + ((s.v[636] * p.p817))));
            s.store_add_scaled_inputs4_offset_indices(832, 635, p.p858, 637, p.p860, 638, p.p861, 639, p.p862, ((p.p857) + ((s.v[636] * p.p859))));
            s.store_add_scaled_inputs4_offset_indices(833, 635, p.p870, 637, p.p872, 638, p.p873, 639, p.p874, ((p.p869) + ((s.v[636] * p.p871))));
        }

        s.b[1103] = (p.p61 != 0.0);
        s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });

        if (s.b[1101] && s.b[1103]) {
            s.store_add_scaled_inputs4_offset_indices(810, 635, p.p672, 637, p.p674, 638, p.p675, 639, p.p676, ((p.p671) + ((s.v[636] * p.p673))));
            s.store_add_scaled_inputs4_offset_indices(829, 635, p.p840, 637, p.p842, 638, p.p843, 639, p.p844, ((p.p839) + ((s.v[636] * p.p841))));
        }

        if s.b[1101] {
            s.store_add_scaled_inputs4_offset_indices(675, 635, p.p261, 637, p.p263, 638, p.p264, 639, p.p265, ((p.p260) + ((s.v[636] * p.p262))));
        }

        s.b[1104] = (p.p161 != 0.0);
        s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });

        if (s.b[1101] && s.b[1104]) {
            s.store_scale(675, 675, (1.0 + ((p.p161 / p.p5) * (if (!((1.0 + (p.p5 / p.p162)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p162)) > 1e-38) { (((1.0 + (p.p5 / p.p162))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1105] = (p.p21 != 0.0);
        s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });

        if (s.b[1101] && s.b[1105]) {
            s.store_mul_scale_offset_rhs(705, 705, 153, ((p.p5 - p.p21) * p.p588), 1.0);
            s.store_mul_scale_offset_rhs(675, 675, 153, ((p.p5 - p.p21) * p.p163), 1.0);
        }

        s.b[1107] = (p.p57 == 1.0);
        s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });

        if s.b[1107] {
            s.store_add_scaled_inputs4_offset_indices(882, 635, p.p1808, 637, p.p1810, 638, p.p1811, 639, p.p1812, ((p.p1807) + ((s.v[636] * p.p1809))));
            s.store_add_scaled_inputs4_offset_indices(883, 635, p.p1815, 637, p.p1817, 638, p.p1818, 639, p.p1819, ((p.p1814) + ((s.v[636] * p.p1816))));
            s.store_add_scaled_inputs4_offset_indices(884, 635, p.p1822, 637, p.p1824, 638, p.p1825, 639, p.p1826, ((p.p1821) + ((s.v[636] * p.p1823))));
            s.store_add_scaled_inputs4_offset_indices(885, 635, p.p1830, 637, p.p1832, 638, p.p1833, 639, p.p1834, ((p.p1829) + ((s.v[636] * p.p1831))));
            s.store_add_scaled_inputs4_offset_indices(886, 635, p.p1836, 637, p.p1838, 638, p.p1839, 639, p.p1840, ((p.p1835) + ((s.v[636] * p.p1837))));
            s.store_add_scaled_inputs4_offset_indices(887, 635, p.p1842, 637, p.p1844, 638, p.p1845, 639, p.p1846, ((p.p1841) + ((s.v[636] * p.p1843))));
            s.store_add_scaled_inputs4_offset_indices(888, 635, p.p1854, 637, p.p1856, 638, p.p1857, 639, p.p1858, ((p.p1853) + ((s.v[636] * p.p1855))));
            s.store_add_scaled_inputs4_offset_indices(889, 635, p.p1860, 637, p.p1862, 638, p.p1863, 639, p.p1864, ((p.p1859) + ((s.v[636] * p.p1861))));
            s.store_add_scaled_inputs4_offset_indices(890, 635, p.p1870, 637, p.p1872, 638, p.p1873, 639, p.p1874, ((p.p1869) + ((s.v[636] * p.p1871))));
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1107] {
            s.store_add_scaled_inputs4_offset_indices(891, 635, p.p1876, 637, p.p1878, 638, p.p1879, 639, p.p1880, ((p.p1875) + ((s.v[636] * p.p1877))));
            s.store_add_scaled_inputs4_offset_indices(892, 635, p.p1882, 637, p.p1884, 638, p.p1885, 639, p.p1886, ((p.p1881) + ((s.v[636] * p.p1883))));
        }

        s.b[1108] = (p.p100 != 0.0);
        s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });

        if s.b[1108] {
            s.store_scale(641, 641, (1.0 + ((p.p100 / p.p5) * (if (!((1.0 + (p.p5 / p.p101)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p101)) > 1e-38) { (((1.0 + (p.p5 / p.p101))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1109] = (p.p158 != 0.0);
        s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });

        if s.b[1109] {
            s.store_scale(673, 673, (1.0 + ((p.p158 / p.p5) * (if (!((1.0 + (p.p5 / p.p159)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p159)) > 1e-38) { (((1.0 + (p.p5 / p.p159))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1110] = (p.p152 != 0.0);
        s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });

        if s.b[1110] {
            s.store_scale(662, 662, (1.0 + ((p.p152 / p.p5) * (if (!((1.0 + (p.p5 / p.p153)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p153)) > 1e-38) { (((1.0 + (p.p5 / p.p153))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1111] = (p.p154 != 0.0);
        s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });

        if s.b[1111] {
            s.store_scale(663, 663, (1.0 + ((p.p154 / p.p5) * (if (!((1.0 + (p.p5 / p.p155)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p155)) > 1e-38) { (((1.0 + (p.p5 / p.p155))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1112] = (p.p156 != 0.0);
        s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });

        if s.b[1112] {
            s.store_scale(665, 665, (1.0 + ((p.p156 / p.p5) * (if (!((1.0 + (p.p5 / p.p157)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p157)) > 1e-38) { (((1.0 + (p.p5 / p.p157))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1113] = (p.p428 != 0.0);
        s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });

        if s.b[1113] {
            s.store_scale(679, 679, (1.0 + ((p.p428 / p.p5) * (if (!((1.0 + (p.p5 / p.p429)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p429)) > 1e-38) { (((1.0 + (p.p5 / p.p429))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1114] = (p.p432 != 0.0);
        s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });

        if s.b[1114] {
            s.store_scale(698, 698, (1.0 + ((p.p432 / p.p5) * (if (!((1.0 + (p.p5 / p.p433)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p433)) > 1e-38) { (((1.0 + (p.p5 / p.p433))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1115] = (p.p434 != 0.0);
        s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });

        if s.b[1115] {
            s.store_scale(699, 699, (1.0 + ((p.p434 / p.p5) * (if (!((1.0 + (p.p5 / p.p435)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p435)) > 1e-38) { (((1.0 + (p.p5 / p.p435))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1116] = (p.p581 != 0.0);
        s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });

        if s.b[1116] {
            s.store_scale(704, 704, (1.0 + ((p.p581 / p.p5) * (if (!((1.0 + (p.p5 / p.p584)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p584)) > 1e-38) { (((1.0 + (p.p5 / p.p584))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1117] = (p.p583 != 0.0);
        s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });

        if s.b[1117] {
            s.store_scale(706, 706, (1.0 + ((p.p583 / p.p5) * (if (!((1.0 + (p.p5 / p.p586)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p5 / p.p586)) > 1e-38) { (((1.0 + (p.p5 / p.p586))) as f64).ln() } else { 0.0 }) }))));
        }

        s.b[1118] = (p.p21 != 0.0);
        s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });

        if s.b[1118] {
            s.store_mul_scale_offset_rhs(641, 641, 153, ((p.p5 - p.p21) * p.p99), 1.0);
            s.store_mul_scale_offset_rhs(673, 673, 153, ((p.p5 - p.p21) * p.p160), 1.0);
            s.store_mul_scale_offset_rhs(704, 704, 153, ((p.p5 - p.p21) * p.p587), 1.0);
        }

        s.store_ln(154, 153);

        s.store_add_scaled_inputs(641, 641, 1.0, 153, p.p98);

        s.store_add_scaled_inputs(661, 661, 1.0, 153, p.p427);

        s.b[1119] = (p.p589 > 0.0);
        s.store_scalar(1119, if s.b[1119] { 1.0 } else { 0.0 });

        if s.b[1119] {
            s.store_mul_sub_from_scalar_ad_rhs(704, 704, 1.0, A::mul(s.ad_value(703), A::exp_scaled_input(s.ad_value(154), (-p.p589))));
        }

        if (!s.b[1119]) {
            s.store_mul_sub_from_scalar_rhs(704, 704, 1.0, 703);
        }

        s.store_add_scaled_inputs_ad_rhs(807, 807, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p593))), p.p591);

        s.store_add_scaled_inputs_ad_rhs(812, 812, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p601))), p.p599);

        s.store_add_scaled_inputs_ad_rhs(811, 811, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p597))), p.p595);

        s.b[1120] = (p.p66 != 0.0);
        s.store_scalar(1120, if s.b[1120] { 1.0 } else { 0.0 });

        if s.b[1120] {
            s.store_add_scaled_inputs_ad_rhs(815, 815, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p594))), p.p592);
            s.store_add_scaled_inputs_ad_rhs(818, 818, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p602))), p.p600);
            s.store_add_scaled_inputs_ad_rhs(816, 816, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p598))), p.p596);
        }

        s.b[1121] = (p.p590 > 0.0);
        s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });

        if (s.b[1120] && s.b[1121]) {
            s.store_mul_sub_from_scalar_ad_rhs(706, 706, 1.0, A::mul(s.ad_value(710), A::exp_scaled_input(s.ad_value(154), (-p.p590))));
        }

        if (s.b[1120] && (!s.b[1121])) {
            s.store_mul_sub_from_scalar_rhs(706, 706, 1.0, 710);
        }

        s.b[1122] = (p.p64 == 1.0);
        s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });

        if s.b[1122] {
            s.store_add_scaled_inputs_ad_rhs(853, 853, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p913))), p.p912);
            s.store_add_scaled_inputs_ad_rhs(852, 852, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p916))), p.p915);
        }

        if (!s.b[1122]) {
            s.store_add_scaled_inputs_ad_rhs(709, 709, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p910))), p.p909);
        }

        s.store_add_scaled_inputs_ad_rhs(792, 792, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p1023))), p.p1021);

        s.b[1123] = (p.p66 != 0.0);
        s.store_scalar(1123, if s.b[1123] { 1.0 } else { 0.0 });

        if s.b[1123] {
            s.store_add_scaled_inputs_ad_rhs(796, 796, 1.0, A::exp_scaled_input(s.ad_value(154), (-p.p1024)), p.p1022);
        }

        s.store_add_scaled_inputs_ad_rhs(790, 790, 1.0, A::exp_scaled_input(s.ad_value(154), (-p.p445)), p.p444);

        s.b[1124] = (p.p66 != 0.0);
        s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });

        if s.b[1124] {
            s.store_add_scaled_inputs_ad_rhs(791, 791, 1.0, A::exp_scaled_input(s.ad_value(154), (-p.p447)), p.p446);
        }

        s.store_add_scaled_inputs_ad_rhs(700, 700, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p449))), p.p448);

        s.b[1125] = (p.p66 != 0.0);
        s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });

        if s.b[1125] {
            s.store_add_scaled_inputs_ad_rhs(701, 701, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p449))), p.p448);
        }

        s.store_add_scaled_inputs_ad_rhs(679, 679, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p431))), p.p430);

        s.store_add_scaled_inputs_ad_rhs(698, 698, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p437))), p.p436);

        s.b[1126] = (p.p66 != 0.0);
        s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });

        if s.b[1126] {
            s.store_add_scaled_inputs_ad_rhs(699, 699, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p437))), p.p436);
        }

        s.store_add_scaled_inputs_ad_rhs(695, 695, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p439))), p.p438);

        s.store_add_scaled_inputs_ad_rhs(697, 697, 1.0, A::limited_exp_scaled_input(s.ad_value(156), (-1.0 / (p.p443))), p.p442);

        s.store_add_scaled_inputs_ad_rhs(702, 702, 1.0, A::limited_exp_scaled_input(s.ad_value(156), (-1.0 / (p.p441))), p.p440);

        s.store_add_scaled_inputs_ad_rhs(681, 681, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p168))), p.p167);

        s.store_add_scaled_inputs_ad_rhs(682, 682, 1.0, A::limited_exp_scaled_input(s.ad_value(153), (-1.0 / (p.p170))), p.p169);

        s.b[1127] = ((s.v[655] > 0.0) || (s.v[656] > 0.0));
        s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });

        if s.b[1127] {
            s.store_offset_scaled_ad(376, A::limited_exp_scaled_input(A::div_scaled_inputs(s.ad_value(894), 2.0, s.ad_value(895), 1.0), (-1.0 / (p.p399))), p.p398, 1.0);
            s.store_mul_div_scaled_inputs_indices(373, 376, 894, 2.0, 895, 1.0);
        }

        s.b[1130] = (s.v[576] <= 0.0);
        s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });

        if s.b[1130] {
            s.store_scalar(576, 0.05);
        }

        s.b[1135] = (s.v[641] <= 0.0);
        s.store_scalar(1135, if s.b[1135] { 1.0 } else { 0.0 });

        if s.b[1135] {
            s.store_scalar(641, 4.61);
        }

        s.b[1136] = (p.p61 != 0.0);
        s.store_scalar(1136, if s.b[1136] { 1.0 } else { 0.0 });

        s.b[1137] = (s.v[690] < 1e-6);
        s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });

        if (s.b[1136] && s.b[1137]) {
            s.store_scalar(690, 1e-6);
        }

        s.b[1138] = (s.v[857] < 0.0);
        s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });

        if s.b[1138] {
            s.store_scalar(857, 0.01);
        }

        s.b[1139] = (s.v[576] < 0.0);
        s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });

        if s.b[1139] {
            s.store_scalar(576, 0.05);
        }

        s.b[1140] = (s.v[574] < 0.0);
        s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });

        if s.b[1140] {
            s.store_scalar(574, p.p1682);
        }

        s.b[1141] = (s.v[575] < 0.0);
        s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });

        if s.b[1141] {
            s.store_scalar(575, 1.2);
        }

        s.b[1142] = (s.v[644] < 0.0);
        s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });

        if s.b[1142] {
            s.store_scalar(644, 0.0);
        }

        s.b[1143] = (s.v[645] < 0.0);
        s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });

        if s.b[1143] {
            s.store_scalar(645, 0.0);
        }

        s.b[1144] = (s.v[679] <= 0.0);
        s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });

        if s.b[1144] {
            s.store_scalar(679, 85000.0);
        }

        s.b[1145] = (s.v[698] <= 0.0);
        s.store_scalar(1145, if s.b[1145] { 1.0 } else { 0.0 });

        if s.b[1145] {
            s.store_scalar(698, 85000.0);
        }

        s.b[1146] = ((p.p66 != 0.0) && (s.v[699] <= 0.0));
        s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });

        if s.b[1146] {
            s.store_scalar(699, 85000.0);
        }

        s.b[1147] = (s.v[670] <= 0.0);
        s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });

        if s.b[1147] {
            s.store_scalar(670, 0.6);
        }

        s.b[1148] = (s.v[671] <= 0.0);
        s.store_scalar(1148, if s.b[1148] { 1.0 } else { 0.0 });

        if s.b[1148] {
            s.store_scalar(671, 0.6);
        }

        s.b[1152] = (s.v[678] <= 0.0);
        s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });

        if s.b[1152] {
            s.store_scalar(678, 1.06);
        }

        s.b[1153] = (s.v[673] < 0.0);
        s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });

        if s.b[1153] {
            s.store_scalar(673, 0.0);
        }

        s.b[1154] = (s.v[677] < 0.0);
        s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });

        if s.b[1154] {
            s.store_scalar(677, 0.0);
        }

        s.b[1155] = (s.v[803] < (-s.v[153]));
        s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });

        if s.b[1155] {
            s.store_scalar(803, 0.0);
        }

        s.b[1156] = (s.v[685] < 0.0);
        s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });

        if s.b[1156] {
            s.store_scalar(685, 0.0);
        }

        s.b[1157] = (s.v[687] < 0.0);
        s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });

        if s.b[1157] {
            s.store_scalar(687, 0.0);
        }

        s.b[1158] = ((p.p61 != 0.0) && (s.v[689] < 0.2));
        s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });

        if s.b[1158] {
            s.store_scalar(689, 0.2);
        }

        s.b[1159] = ((p.p61 != 0.0) && (s.v[689] > 1.2));
        s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });

        if s.b[1159] {
            s.store_scalar(689, 1.2);
        }

        s.b[1160] = (s.v[695] < 2.0);
        s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });

        if s.b[1160] {
            s.store_scalar(695, 2.0);
        }

        s.b[1161] = (s.v[697] < 2.0);
        s.store_scalar(1161, if s.b[1161] { 1.0 } else { 0.0 });

        if s.b[1161] {
            s.store_scalar(697, 2.0);
        }

        s.b[1162] = (s.v[704] < 0.0);
        s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });

        if s.b[1162] {
            s.store_scalar(704, 0.03);
        }

        s.b[1163] = (s.v[807] < 0.0);
        s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });

        if s.b[1163] {
            s.store_scalar(807, 0.0);
        }

        s.b[1164] = (s.v[811] < 0.0);
        s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });

        if s.b[1164] {
            s.store_scalar(811, 0.0);
        }

        s.b[1165] = (s.v[812] < 0.0);
        s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });

        if s.b[1165] {
            s.store_scalar(812, 0.0);
        }

        s.b[1166] = (s.v[814] < 0.0);
        s.store_scalar(1166, if s.b[1166] { 1.0 } else { 0.0 });

        if s.b[1166] {
            s.store_scalar(814, 0.0);
        }

        s.b[1167] = (s.v[707] < 0.0);
        s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });

        if s.b[1167] {
            s.store_scalar(707, 0.0);
        }

        s.b[1168] = (s.v[709] < 0.0);
        s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });

        if s.b[1168] {
            s.store_scalar(709, 0.0);
        }

        s.b[1169] = (s.v[853] < 0.0);
        s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });

        if s.b[1169] {
            s.store_scalar(853, 0.0);
        }

        s.b[1170] = (s.v[852] < 0.0);
        s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });

        if s.b[1170] {
            s.store_scalar(852, 0.0);
        }

        s.b[1171] = (s.v[712] < 0.0);
        s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });

        if s.b[1171] {
            s.store_scalar(712, 0.0);
        }

        s.b[1172] = (s.v[711] < 0.0);
        s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });

        if s.b[1172] {
            s.store_scalar(711, 0.0);
        }

        s.b[1175] = (p.p66 != 0.0);
        s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });

        s.b[1178] = (s.v[706] < 0.0);
        s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });

        if (s.b[1175] && s.b[1178]) {
            s.store_scalar(706, 0.0);
        }

        s.b[1179] = (s.v[815] < 0.0);
        s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });

        if (s.b[1175] && s.b[1179]) {
            s.store_scalar(815, 0.0);
        }

        s.b[1180] = (s.v[816] < 0.0);
        s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });

        if (s.b[1175] && s.b[1180]) {
            s.store_scalar(816, 0.0);
        }

        s.b[1181] = (s.v[818] < 0.0);
        s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });

        if (s.b[1175] && s.b[1181]) {
            s.store_scalar(818, 0.0);
        }

        s.b[1183] = (s.v[719] <= 0.0);
        s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });

        if s.b[1183] {
            s.store_scalar(719, 1.06);
        }

        s.b[1184] = (s.v[790] < 2.0);
        s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });

        if s.b[1184] {
            s.store_scalar(790, 2.0);
        }

        s.b[1185] = (p.p66 != 0.0);
        s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1186] = (s.v[791] < 2.0);
        s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });

        if (s.b[1185] && s.b[1186]) {
            s.store_scalar(791, 2.0);
        }

        s.b[1187] = (s.v[700] < 0.0);
        s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });

        if s.b[1187] {
            s.store_scalar(700, 0.0);
        }

        s.b[1188] = (s.v[749] < 0.0);
        s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });

        if s.b[1188] {
            s.store_scalar(749, 0.0);
        }

        s.b[1189] = (s.v[763] < 0.0);
        s.store_scalar(1189, if s.b[1189] { 1.0 } else { 0.0 });

        if s.b[1189] {
            s.store_scalar(763, 0.0);
        }

        s.b[1190] = (p.p69 != 0.0);
        s.store_scalar(1190, if s.b[1190] { 1.0 } else { 0.0 });

        s.b[1191] = (s.v[726] <= 0.0);
        s.store_scalar(1191, if s.b[1191] { 1.0 } else { 0.0 });

        if (s.b[1190] && s.b[1191]) {
            s.store_scalar(726, 3.0);
        }

        s.b[1192] = (s.v[731] <= 0.0);
        s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });

        if (s.b[1190] && s.b[1192]) {
            s.store_scalar(731, 1.0);
        }

        s.b[1193] = (p.p68 != 0.0);
        s.store_scalar(1193, if s.b[1193] { 1.0 } else { 0.0 });

        s.b[1194] = (s.v[742] <= 0.0);
        s.store_scalar(1194, if s.b[1194] { 1.0 } else { 0.0 });

        if (s.b[1193] && s.b[1194]) {
            s.store_scalar(742, 1.0);
        }

        s.b[1195] = (s.v[736] <= 0.0);
        s.store_scalar(1195, if s.b[1195] { 1.0 } else { 0.0 });

        if (s.b[1193] && s.b[1195]) {
            s.store_scalar(736, 1.0);
        }

        s.b[1213] = (s.v[648] < 0.0);
        s.store_scalar(1213, if s.b[1213] { 1.0 } else { 0.0 });

        if s.b[1213] {
            s.store_scalar(648, 0.0);
        }

        s.b[1214] = (s.v[649] < 0.0);
        s.store_scalar(1214, if s.b[1214] { 1.0 } else { 0.0 });

        if s.b[1214] {
            s.store_scalar(649, 0.0);
        }

        s.b[1215] = (s.v[643] < 0.0);
        s.store_scalar(1215, if s.b[1215] { 1.0 } else { 0.0 });

        if s.b[1215] {
            s.store_scalar(643, 0.0);
        }

        s.b[1216] = (s.v[642] < 0.0);
        s.store_scalar(1216, if s.b[1216] { 1.0 } else { 0.0 });

        if s.b[1216] {
            s.store_scalar(642, 0.0);
        }

        s.b[1217] = (s.v[650] < 0.0);
        s.store_scalar(1217, if s.b[1217] { 1.0 } else { 0.0 });

        if s.b[1217] {
            s.store_scalar(650, 0.0);
        }

        s.b[1218] = (s.v[651] <= 0.02);
        s.store_scalar(1218, if s.b[1218] { 1.0 } else { 0.0 });

        if s.b[1218] {
            s.store_scalar(651, 0.02);
        }

        s.b[1219] = (s.v[652] <= 0.02);
        s.store_scalar(1219, if s.b[1219] { 1.0 } else { 0.0 });

        if s.b[1219] {
            s.store_scalar(652, 0.02);
        }

        s.b[1220] = (s.v[653] <= 0.02);
        s.store_scalar(1220, if s.b[1220] { 1.0 } else { 0.0 });

        if s.b[1220] {
            s.store_scalar(653, 0.02);
        }

        s.b[1221] = (s.v[446] < (-p.p4));
        s.store_scalar(1221, if s.b[1221] { 1.0 } else { 0.0 });

        if s.b[1221] {
            s.store_scalar(446, 0.0);
        }

        s.b[1222] = (p.p57 == 1.0);
        s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });

        s.b[1223] = ((s.v[882] < 1.0) || (s.v[882] > 3.0));
        s.store_scalar(1223, if s.b[1223] { 1.0 } else { 0.0 });

        if (s.b[1222] && s.b[1223]) {
            s.store_scalar(882, 2.0);
        }

        s.b[1224] = ((s.v[883] < 1.0) || (s.v[883] > 3.0));
        s.store_scalar(1224, if s.b[1224] { 1.0 } else { 0.0 });

        if (s.b[1222] && s.b[1224]) {
            s.store_scalar(883, 2.6);
        }

        s.b[1225] = ((s.v[884] < 1.0) || (s.v[884] > 3.0));
        s.store_scalar(1225, if s.b[1225] { 1.0 } else { 0.0 });

        if (s.b[1222] && s.b[1225]) {
            s.store_scalar(884, 2.6);
        }

        s.b[1226] = (s.v[885] < 0.0);
        s.store_scalar(1226, if s.b[1226] { 1.0 } else { 0.0 });

        if (s.b[1222] && s.b[1226]) {
            s.store_scalar(885, 14.0);
        }

        s.b[1227] = (s.v[886] < 0.0);
        s.store_scalar(1227, if s.b[1227] { 1.0 } else { 0.0 });

        if (s.b[1222] && s.b[1227]) {
            s.store_scalar(886, 24.0);
        }

        s.b[1228] = (s.v[887] < 0.0);
        s.store_scalar(1228, if s.b[1228] { 1.0 } else { 0.0 });

        if (s.b[1222] && s.b[1228]) {
            s.store_scalar(887, 24.0);
        }

        s.b[1229] = (s.v[888] < 0.0);
        s.store_scalar(1229, if s.b[1229] { 1.0 } else { 0.0 });

        if (s.b[1222] && s.b[1229]) {
            s.store_scalar(888, 0.139);
        }

        s.b[1230] = (s.v[889] < 0.0);
        s.store_scalar(1230, if s.b[1230] { 1.0 } else { 0.0 });

        if (s.b[1222] && s.b[1230]) {
            s.store_scalar(889, 2.0);
        }

        s.b[1231] = (s.v[890] < 0.0);
        s.store_scalar(1231, if s.b[1231] { 1.0 } else { 0.0 });

        if (s.b[1222] && s.b[1231]) {
            s.store_scalar(890, 11.2);
        }

        s.b[1232] = (s.v[891] < 0.0);
        s.store_scalar(1232, if s.b[1232] { 1.0 } else { 0.0 });

        if (s.b[1222] && s.b[1232]) {
            s.store_scalar(891, 8.02);
        }

        s.b[1233] = (s.v[892] < 0.0);
        s.store_scalar(1233, if s.b[1233] { 1.0 } else { 0.0 });

        if (s.b[1222] && s.b[1233]) {
            s.store_scalar(892, 6.18);
        }

        s.b[1234] = ((p.p74 != 0.0) && (p.p1791 > 0.0));
        s.store_scalar(1234, if s.b[1234] { 1.0 } else { 0.0 });

        s.b[1235] = (p.p1795 != 0.0);
        s.store_scalar(1235, if s.b[1235] { 1.0 } else { 0.0 });

        if (s.b[1234] && s.b[1235]) {
            s.store_scalar(169, (p.p1793 * ((p.p59) as f64).powf(p.p1795)));
        }

        if (s.b[1234] && (!s.b[1235])) {
            s.store_scalar(169, p.p1793);
        }

        s.b[1236] = (p.p1794 != 0.0);
        s.store_scalar(1236, if s.b[1236] { 1.0 } else { 0.0 });

        if (s.b[1234] && s.b[1236]) {
            s.store_scalar(170, ((p.p1797 * p.p4) * ((s.v[115]) as f64).powf(p.p1794)));
        }

        if (s.b[1234] && (!s.b[1236])) {
            s.store_scalar(170, (p.p1797 * p.p4));
        }

        s.b[1237] = (p.p62 == 5.0);
        s.store_scalar(1237, if s.b[1237] { 1.0 } else { 0.0 });

        s.b[1238] = (p.p1796 != 0.0);
        s.store_scalar(1238, if s.b[1238] { 1.0 } else { 0.0 });

        if ((s.b[1234] && s.b[1237]) && s.b[1238]) {
            s.store_scalar(171, (((p.p1798 * p.p59) * p.p43) * ((p.p56) as f64).powf(p.p1796)));
        }

        if ((s.b[1234] && s.b[1237]) && (!s.b[1238])) {
            s.store_scalar(171, ((p.p1798 * p.p59) * p.p43));
        }

        if (s.b[1234] && (!s.b[1237])) {
            s.store_scalar(171, 0.0);
        }

        if s.b[1234] {
            s.store_add_scaled_inputs3_indices(634, 169, p.p1792, 170, p.p1792, 171, p.p1792);
        }

        s.b[1241] = (p.p77 == 0.0);
        s.store_scalar(1241, if s.b[1241] { 1.0 } else { 0.0 });

        if s.b[1241] {
            s.store_scalar(190, (p.p1078 * p.p18));
            s.store_scalar(191, (p.p1079 * p.p19));
        }

        s.b[1242] = (p.p1080 > 0.0);
        s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });

        if ((!s.b[1241]) && s.b[1242]) {
            s.store_scalar(444, ((p.p4 * p.p92) + ((p.p3 + ((p.p4 - p.p3) * p.p1084)) * p.p1080)));
        }

        if ((!s.b[1241]) && (!s.b[1242])) {
            s.store_scalar(444, (p.p4 * (1e-9_f64).max((p.p92 + p.p1080))));
        }

        if (!s.b[1241]) {
            s.store_offset(445, 446, p.p4);
        }

        s.b[1243] = param_given[1083];
        s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });

        if ((!s.b[1241]) && s.b[1243]) {
            s.store_scalar(431, p.p1083);
        }

        if ((!s.b[1241]) && (!s.b[1243])) {
            s.store_scalar(429, (if (p.p60 == 1.0) { 1417.0 } else { 470.5 }));
        }

        s.b[1244] = (p.p60 == 1.0);
        s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });

        if (((!s.b[1241]) && (!s.b[1243])) && s.b[1244]) {
            s.store_scalar(168, (((p.p97 / 9.68e22)) as f64).powf(0.68));
            s.store_scalar(169, (3.43e26 / p.p97));
            s.store_scaled_sub_ad(430, A::offset(A::div_scaled_offset_numerator(s.ad_value(429), 1.0, (-52.2), A::offset(s.ad_value(168), 1.0), 1.0), 52.2), A::div_scalar_offset_denominator(43.4, A::square(s.ad_value(169)), 1.0, 1.0), 0.0001);
        }

        if (((!s.b[1241]) && (!s.b[1243])) && (!s.b[1244])) {
            s.store_scalar(168, (((p.p97 / 2.23e22)) as f64).powf(0.719));
            s.store_scalar(169, (6.1e26 / p.p97));
            s.store_scaled_sub_ad(430, A::offset(A::div_scaled_offset_numerator(s.ad_value(429), 1.0, (-44.9), A::offset(s.ad_value(168), 1.0), 1.0), 44.9), A::div_scalar_offset_denominator(29.0, A::square(s.ad_value(169)), 1.0, 1.0), 0.0001);
        }

        if ((!s.b[1241]) && (!s.b[1243])) {
            s.store_div_from_scalar_scaled_input(431, 1.0, 430, (1.60219e-19 * p.p97));
        }

        if (!s.b[1241]) {
            s.store_scalar(433, ((55.0 * 3.141592653589793) / 180.0));
            s.store_min_with_scalar(432, 444, (1e-18_f64).max((p.p3 * (p.p92 + (0.0_f64).min(p.p1080)))));
            s.store_scaled_mul_ad(434, A::div(s.ad_value(431), A::tan(s.ad_value(433))), A::add_scaled_inputs3(A::div_from_scalar(1.0, A::sqrt(s.ad_value(432))), 1.0, A::div_from_scalar(2.0, A::sqrt(s.ad_value(444))), (-1.0), A::sqrt(A::div(s.ad_value(432), A::square(s.ad_value(444)))), 1.0), 1.0 / ((((3.141592653589793) as f64).sqrt() * p.p5)));
            s.store_offset_scaled(436, 444, p.p5, p.p1092);
            s.store_offset_scaled(437, 445, p.p5, p.p1093);
            s.store_sqrt_ad(435, A::div_scaled_inputs(s.ad_value(436), p.p1082, A::mul(s.ad_value(431), s.ad_value(437)), 1.0));
            s.store_div_from_scalar(438, p.p20, 435);
            s.store_limited_exp_scaled_input(168, 438, 2.0);
        }

        s.b[1245] = (p.p1086 == 1.0);
        s.store_scalar(1245, if s.b[1245] { 1.0 } else { 0.0 });

        if ((!s.b[1241]) && s.b[1245]) {
            s.store_scaled_mul(439, 431, 435, 1.0 / (p.p1082));
            s.store_mul_offset_rhs(169, 168, 439, 1.0);
            s.store_sub_offset_lhs(170, 169, 1.0, 439);
            s.store_add_offset_lhs(171, 169, (-1.0), 439);
        }

        if ((!s.b[1241]) && (!s.b[1245])) {
            s.store_offset(170, 168, 1.0);
            s.store_offset(171, 168, (-1.0));
        }

        if (!s.b[1241]) {
            s.store_div_scaled_product3_by_product(440, s.ad_value(431), s.ad_value(435), s.ad_value(170), 1.0, s.ad_value(436), s.ad_value(171), 1.0);
        }

        s.b[1246] = (p.p1080 < (-1e-10));
        s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });

        if ((!s.b[1241]) && s.b[1246]) {
            s.store_scalar(441, (p.p1082 / (((-p.p1080) * p.p3) * p.p5)));
            s.store_div_scaled_product_mixed_aia(442, A::add(s.ad_value(440), s.ad_value(434)), 441, 1.0, A::add_scaled_inputs3(s.ad_value(440), 1.0, s.ad_value(434), 1.0, s.ad_value(441), 1.0), 1.0);
        }

        if ((!s.b[1241]) && (!s.b[1246])) {
            s.store_add(442, 440, 434);
        }

        if (!s.b[1241]) {
            s.store_scale(443, 442, (1.0 / (p.p59) * (0.0_f64).max(((((p.p1094 + (p.p1095 * p.p3)) + (p.p1096 * p.p4)) + (p.p1097 * p.p20)) + (p.p1098 * p.p1080)))));
            s.copy_ad(190, 443);
            s.copy_ad(191, 443);
        }

        s.b[1247] = (p.p64 == 0.0);
        s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });

        s.b[1248] = (s.v[190] < p.p151);
        s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });

        if (s.b[1247] && s.b[1248]) {
            s.store_scalar(190, 0.0);
        }

        s.b[1249] = (s.v[191] < p.p151);
        s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });

        if (s.b[1247] && s.b[1249]) {
            s.store_scalar(191, 0.0);
        }

        s.b[1250] = (s.v[190] <= p.p151);
        s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });

        if ((!s.b[1247]) && s.b[1250]) {
            s.store_scalar(190, p.p151);
        }

        s.b[1251] = (s.v[191] <= p.p151);
        s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });

        if ((!s.b[1247]) && s.b[1251]) {
            s.store_scalar(191, p.p151);
        }

        s.b[1252] = (p.p78 != 1.0);
        s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });

        s.b[1253] = param_given[1542];
        s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });

        if (s.b[1252] && s.b[1253]) {
            s.store_scalar(646, p.p1542);
        }

        s.b[1254] = (param_given[85] && (p.p85 > 0.0));
        s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });

        if ((s.b[1252] && (!s.b[1253])) && s.b[1254]) {
            s.store_max_from_scalar_ad(646, 0.0, A::sub_scaled_inputs(s.ad_value(163), p.p85, s.ad_value(648), 1.0));
        }

        s.b[1255] = (p.p78 == 3.0);
        s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });

        if (((s.b[1252] && (!s.b[1253])) && (!s.b[1254])) && s.b[1255]) {
            s.store_scale(646, 163, (0.3 * p.p43));
        }

        if (((s.b[1252] && (!s.b[1253])) && (!s.b[1254])) && (!s.b[1255])) {
            s.store_scale(646, 163, (0.3 * p.p3));
        }

        s.b[1256] = param_given[1543];
        s.store_scalar(1256, if s.b[1256] { 1.0 } else { 0.0 });

        if (s.b[1252] && s.b[1256]) {
            s.store_scalar(647, p.p1543);
        }

        s.b[1257] = (param_given[85] && (p.p85 > 0.0));
        s.store_scalar(1257, if s.b[1257] { 1.0 } else { 0.0 });

        if ((s.b[1252] && (!s.b[1256])) && s.b[1257]) {
            s.store_max_from_scalar_ad(647, 0.0, A::sub_scaled_inputs(s.ad_value(163), p.p85, s.ad_value(649), 1.0));
        }

        s.b[1258] = (p.p78 == 3.0);
        s.store_scalar(1258, if s.b[1258] { 1.0 } else { 0.0 });

        if (((s.b[1252] && (!s.b[1256])) && (!s.b[1257])) && s.b[1258]) {
            s.store_scale(647, 163, (0.3 * p.p43));
        }

        if (((s.b[1252] && (!s.b[1256])) && (!s.b[1257])) && (!s.b[1258])) {
            s.store_scale(647, 163, (0.3 * p.p3));
        }

        s.b[1259] = (p.p78 == 2.0);
        s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });

        if s.b[1259] {
            s.store_scalar(447, (p.p1089 + p.p1090));
            s.store_scalar(449, (0.5 * (p.p4 - p.p3)));
            s.store_max_from_scalar_ad(448, 0.0, A::offset(s.ad_value(449), (-p.p90)));
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1259] {
            s.store_scalar(450, (0.0_f64).max((p.p1080 + p.p1081)));
        }

        s.b[1260] = (p.p1090 > 0.0);
        s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });

        if (s.b[1259] && s.b[1260]) {
            s.store_scalar(168, (3.467e-11 * (if (!(((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38)) { (-87.498233534) } else { (if (((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38) { ((((1e-7 * p.p1088) / (3.9 * p.p1087))) as f64).ln() } else { 0.0 }) })));
        }

        if (s.b[1259] && s.b[1260]) {
            s.store_scale(169, 450, (0.942 * (s.v[144] * 1.0 / (p.p1087))));
            s.store_scaled_add(451, 168, 169, (p.p3 + ((p.p4 - p.p3) * p.p1084)));
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(447), 0.2, (p.p90 * 0.2), s.ad_value(450), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(447), p.p90), s.ad_value(450)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(450), A::offset(s.ad_value(447), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1261] = (s.v[933] > 80.0);
        s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });

        if ((s.b[1259] && (!s.b[1260])) && s.b[1261]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1259] && (!s.b[1260])) && (!s.b[1261])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(450), 1.0, s.ad_value(447), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(447), 1.0, p.p90, s.ad_value(450), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_scaled_add(938, 934, 937, p.p3);
            s.store_div(930, 928, 447);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(447), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(447)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(447), s.ad_value(930), 1.0), 447);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), 944, 1.0);
        }

        if (s.b[1259] && (!s.b[1260])) {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p3, 933, ((-0.5) * p.p3), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p3));
            s.store_add(451, 938, 947);
        }

        s.b[1262] = (p.p1090 > 0.0);
        s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });

        if (s.b[1259] && s.b[1262]) {
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(448), 0.2, (p.p90 * 0.2), s.ad_value(449), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1263] = (s.v[933] > 80.0);
        s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });

        if ((s.b[1259] && s.b[1262]) && s.b[1263]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1259] && s.b[1262]) && (!s.b[1263])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p.p90, s.ad_value(449), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_scaled_add(938, 934, 937, p.p92);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.7), 944, 1.0);
        }

        if (s.b[1259] && s.b[1262]) {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p92, 933, ((-0.5) * p.p92), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p92));
            s.store_add(452, 938, 947);
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(448), 0.2, (p.p90 * 0.2), s.ad_value(449), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1264] = (s.v[933] > 80.0);
        s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });

        if ((s.b[1259] && (!s.b[1262])) && s.b[1264]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1259] && (!s.b[1262])) && (!s.b[1264])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p.p90, s.ad_value(449), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_scaled_add(938, 934, 937, p.p92);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1259] && (!s.b[1262])) {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), 944, 1.0);
        }

        if (s.b[1259] && (!s.b[1262])) {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p92, 933, ((-0.5) * p.p92), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p92));
            s.store_add(452, 938, 947);
        }

        s.b[1265] = (p.p1090 > 0.0);
        s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });

        if (s.b[1259] && s.b[1265]) {
            s.store_scalar(454, 0.0);
        }

        s.b[1266] = (p.p1080 > 0.0);
        s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });

        if ((s.b[1259] && (!s.b[1265])) && s.b[1266]) {
            s.store_scalar(454, ((p.p4 - p.p3) * ((p.p1080 * p.p1084) + p.p1081)));
        }

        if ((s.b[1259] && (!s.b[1265])) && (!s.b[1266])) {
            s.store_scale(454, 450, (p.p4 - p.p3));
        }

        if s.b[1259] {
            s.store_offset_scaled(455, 454, ((p.p5) * ((s.v[144] * 1.0 / (p.p1087)))), ((((p.p1092) + (p.p1091))) * ((s.v[144] * 1.0 / (p.p1087)))));
            s.store_add_scaled_inputs3_indices(453, 455, p.p59, 451, (p.p5 * p.p59), 452, ((p.p1103 * (p.p5 * 2.0)) * p.p59));
            s.store_scale(453, 453, (0.0_f64).max((((p.p1099 + (p.p1100 * p.p3)) + (p.p1101 * p.p4)) + (p.p1102 * p.p20))));
        }

        s.b[1267] = (p.p78 == 3.0);
        s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });

        if s.b[1267] {
            s.store_scalar(447, (p.p1089 + p.p1090));
            s.store_scalar(449, (0.5 * (p.p4 - p.p43)));
            s.store_max_from_scalar_ad(448, 0.0, A::offset(s.ad_value(449), (-p.p90)));
            s.store_scalar(450, (0.0_f64).max((p.p1080 + p.p1081)));
            s.store_scalar(1031, (0.5 * p.p41));
        }

        s.b[1268] = (p.p1090 > 0.0);
        s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });

        if (s.b[1267] && s.b[1268]) {
            s.store_scalar(168, (3.467e-11 * (if (!(((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38)) { (-87.498233534) } else { (if (((1e-7 * p.p1088) / (3.9 * p.p1087)) > 1e-38) { ((((1e-7 * p.p1088) / (3.9 * p.p1087))) as f64).ln() } else { 0.0 }) })));
        }

        if (s.b[1267] && s.b[1268]) {
            s.store_scale(169, 450, (0.942 * (s.v[144] * 1.0 / (p.p1087))));
            s.store_scaled_add(1034, 168, 169, (p.p43 + ((p.p4 - p.p43) * p.p1084)));
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(447), 0.2, (p.p90 * 0.2), s.ad_value(450), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(447), p.p90), s.ad_value(450)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(450), A::offset(s.ad_value(447), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1269] = (s.v[933] > 80.0);
        s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });

        if ((s.b[1267] && (!s.b[1268])) && s.b[1269]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1267] && (!s.b[1268])) && (!s.b[1269])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(450), 1.0, s.ad_value(447), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(447), 1.0, p.p90, s.ad_value(450), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_scaled_add(938, 934, 937, p.p43);
            s.store_div(930, 928, 447);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(447), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(447)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(447), s.ad_value(930), 1.0), 447);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), 944, 1.0);
        }

        if (s.b[1267] && (!s.b[1268])) {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p43, 933, ((-0.5) * p.p43), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p43));
            s.store_add(1034, 938, 947);
        }

        if s.b[1267] {
            s.store_offset_div_from_scalar_ad(925, (0.2 * (p.p1089 + p.p90)), s.ad_value(1031), 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub_from_scalar((p.p1089 + p.p90), s.ad_value(1031)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_with_scalar(929, 1031, (p.p1089 + p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1270] = (s.v[933] > 80.0);
        s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });

        if (s.b[1267] && s.b[1270]) {
            s.copy_ad(934, 932);
        }

        if (s.b[1267] && (!s.b[1270])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if s.b[1267] {
            s.store_scale_ad(935, A::min(A::scale(s.ad_value(1031), 1.0 / ((p.p1089 + p.p90))), A::div_from_scalar((p.p1089 + p.p90), s.ad_value(1031))), 0.5);
            s.store_mul(936, 927, 935);
        }

        if s.b[1267] {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if s.b[1267] {
            s.store_scaled_add(938, 934, 937, p.p43);
            s.store_scale(930, 928, 1.0 / (p.p1089));
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_offset_add_scaled_inputs_mixed_ai(940, A::offset(A::mul(A::sqrt(A::scale_offset(s.ad_value(930), (p.p1089 * p.p1089), (((p.p1089 * p.p1089)) + (((p.p90 * p.p90) + ((2.0 * p.p1089) * p.p90)))))), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, 930, p.p1089, p.p1089);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if s.b[1267] {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if s.b[1267] {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if s.b[1267] {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), 944, 1.0);
        }

        if s.b[1267] {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p43, 933, ((-0.5) * p.p43), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p43));
            s.store_add(1035, 938, 947);
        }

        s.b[1271] = (p.p1090 > 0.0);
        s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });

        if (s.b[1267] && s.b[1271]) {
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(448), 0.2, (p.p90 * 0.2), s.ad_value(449), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1272] = (s.v[933] > 80.0);
        s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });

        if ((s.b[1267] && s.b[1271]) && s.b[1272]) {
            s.copy_ad(934, 932);
        }

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1267] && s.b[1271]) && (!s.b[1272])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p.p90, s.ad_value(449), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_scaled_add(938, 934, 937, p.p40);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.7), 944, 1.0);
        }

        if (s.b[1267] && s.b[1271]) {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p40, 933, ((-0.5) * p.p40), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p40));
            s.store_add(1036, 938, 947);
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(448), 0.2, (p.p90 * 0.2), s.ad_value(449), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1273] = (s.v[933] > 80.0);
        s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });

        if ((s.b[1267] && (!s.b[1271])) && s.b[1273]) {
            s.copy_ad(934, 932);
        }

        if ((s.b[1267] && (!s.b[1271])) && (!s.b[1273])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p.p90, s.ad_value(449), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_scaled_add(938, 934, 937, p.p40);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), 944, 1.0);
        }

        if (s.b[1267] && (!s.b[1271])) {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p40, 933, ((-0.5) * p.p40), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p40));
            s.store_add(1036, 938, 947);
        }

        if s.b[1267] {
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(448), 0.2, (p.p90 * 0.2), s.ad_value(449), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

        s.b[1274] = (s.v[933] > 80.0);
        s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });

        if (s.b[1267] && s.b[1274]) {
            s.copy_ad(934, 932);
        }

        if (s.b[1267] && (!s.b[1274])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if s.b[1267] {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p.p90, s.ad_value(449), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if s.b[1267] {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if s.b[1267] {
            s.store_scaled_add(938, 934, 937, p.p40);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if s.b[1267] {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if s.b[1267] {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if s.b[1267] {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), 944, 1.0);
        }

        if s.b[1267] {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p40, 933, ((-0.5) * p.p40), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p40));
            s.store_add(1037, 938, 947);
            s.store_offset_div_scaled_offset_numerator(925, s.ad_value(448), 0.2, (p.p90 * 0.2), s.ad_value(449), 1.0, 2.3);
            s.store_scalar(926, 1.05);
            s.store_abs_ad(927, A::sub(A::offset(s.ad_value(448), p.p90), s.ad_value(449)));
            s.store_scale(928, 926, p.p1087);
            s.store_min_ad(929, s.ad_value(449), A::offset(s.ad_value(448), p.p90));
            s.store_div_from_scalar_offset_input(930, p.p1087, 925, 1.0);
            s.store_scalar(931, 1700000000000.0);
            s.store_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p.p1087)));
            s.store_mul(933, 931, 932);
        }

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1275] = (s.v[933] > 80.0);
        s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });

        if (s.b[1267] && s.b[1275]) {
            s.copy_ad(934, 932);
        }

        if (s.b[1267] && (!s.b[1275])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }

        if s.b[1267] {
            s.store_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p.p90, 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p.p90, s.ad_value(449), 1.0)), 0.5);
            s.store_mul(936, 927, 935);
        }

        if s.b[1267] {
            s.store_scale_ad(937, {
                if (!(((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p.p1087 + ((0.5 * 3.141592653589793) * s.v[936])) / p.p1087) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p.p1087), 1.0 / (p.p1087))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }

        if s.b[1267] {
            s.store_scaled_add(938, 934, 937, p.p42);
            s.store_div(930, 928, 448);
            s.store_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);
            s.store_add_ad_lhs(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p.p90), (p.p90 * p.p90)), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p.p90), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);
            s.store_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p.p90, A::scaled_offset(s.ad_value(930), 2.0, p.p90), 1.0);
        }

        if s.b[1267] {
            s.store_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }

        if s.b[1267] {
            s.store_mul(943, 925, 926);
            s.store_sqrt_square_offset(944, 943, 1.0);
            s.store_add_ad_lhs(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p.p90 * p.p90), s.ad_value(943), s.ad_value(928), (2.0 * p.p90)), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p.p90, A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);
            s.store_mul_scaled_ad_lhs(945, A::offset(s.ad_value(944), 1.0), 943, p.p90);
        }

        if s.b[1267] {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), 944, 1.0);
        }

        if s.b[1267] {
            s.store_scalar(627, 1.2e-12);
            s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p.p42, 933, ((-0.5) * p.p42), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p.p42));
            s.store_add(1038, 938, 947);
        }

        s.b[1276] = (p.p1090 > 0.0);
        s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });

        if (s.b[1267] && s.b[1276]) {
            s.store_scalar(1032, 0.0);
        }

        s.b[1277] = (p.p1080 > 0.0);
        s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });

        if ((s.b[1267] && (!s.b[1276])) && s.b[1277]) {
            s.store_scalar(1032, ((p.p4 - p.p43) * ((p.p1080 * p.p1084) + p.p1081)));
        }

        if ((s.b[1267] && (!s.b[1276])) && (!s.b[1277])) {
            s.store_scale(1032, 450, (p.p4 - p.p43));
        }

        if s.b[1267] {
            s.store_scale(1033, 1031, (p.p4 - p.p43));
            s.store_scaled_offset_ad(455, A::add_scaled_inputs(s.ad_value(1032), p.p5, s.ad_value(1033), ((2.0 * p.p56) * p.p5)), ((p.p1092) + (p.p1091)), (s.v[144] * 1.0 / (p.p1087)));
            s.store_scaled_add_ad(453, A::add_scaled_inputs3(s.ad_value(455), 1.0, s.ad_value(1034), p.p5, s.ad_value(1035), ((2.0 * p.p56) * p.p5)), A::add_scaled_inputs3(s.ad_value(1036), (p.p1103 * (p.p5 * 2.0)), s.ad_value(1037), ((p.p56 - 1.0) * (p.p1103 * (p.p5 * 2.0))), s.ad_value(1038), (p.p1103 * (p.p5 * 2.0))), p.p59);
            s.store_scale(453, 453, (0.0_f64).max((((p.p1099 + (p.p1100 * p.p43)) + (p.p1101 * p.p4)) + (p.p1102 * p.p20))));
        }

        s.store_scalar(168, (p.p1583 * (if (!((1.0 + (p.p92 / p.p91)) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p.p92 / p.p91)) > 1e-38) { (((1.0 + (p.p92 / p.p91))) as f64).ln() } else { 0.0 }) })));

        s.store_scalar(515, ((s.v[165] * p.p7) + (s.v[168] * (0.0_f64).max((p.p9 - (p.p4 * s.v[115]))))));

        s.store_scalar(516, ((s.v[165] * p.p8) + (s.v[168] * (0.0_f64).max((p.p10 - (p.p4 * s.v[115]))))));

        s.b[1278] = (p.p62 != 5.0);
        s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });

        if s.b[1278] {
            s.store_scale(517, 149, (((p.p1544 * p.p59) * p.p6) + (p.p1545 * s.v[115])));
        }

        if (!s.b[1278]) {
            s.store_mul_scale_offset_rhs(517, 149, 161, ((p.p1546) * (s.v[115])), ((((p.p1545) * (s.v[115]))) + (((p.p1544 * p.p59) * p.p6))));
        }

        s.store_scalar(420, (1e-8 / (s.v[145] * p.p89)));

        s.store_div_from_scalar_scaled_ad(189, 1.0, A::pow(A::scale(s.ad_value(158), 1000000.0), s.ad_value(713)), s.v[115]);

        s.store_scalar(578, (((((s.v[145] * p.p89) * 0.5) * p.p3)) as f64).sqrt());

        s.store_sqrt_ad(351, A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(894), s.v[143], s.ad_value(893), 1.0), A::div_scaled_product_by_product(s.ad_value(894), s.ad_value(893), 1.0, s.ad_value(895), s.ad_value(895), (2.0 * s.v[143])), 1.0));

        s.b[1279] = (!param_given[172]);
        s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });

        if s.b[1279] {
            s.store_offset_div_scaled_product(360, s.ad_value(670), s.ad_value(153), 1.0, s.ad_value(351), 1.0, 1e-6);
        }

        s.b[1280] = (s.v[360] < 40.0);
        s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });

        if (s.b[1279] && s.b[1280]) {
            s.store_div_from_scalar_offset_ad(361, 0.5, A::cosh(s.ad_value(360)), (-1.0));
        }

        if (s.b[1279] && (!s.b[1280])) {
            s.store_limited_exp_neg_input(361, 360);
        }

        if (!s.b[1279]) {
            s.store_scalar(361, p.p172);
        }

        s.b[1281] = (!param_given[174]);
        s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });

        if s.b[1281] {
            s.store_offset_div_scaled_product(360, s.ad_value(671), s.ad_value(153), 1.0, s.ad_value(351), 1.0, 1e-6);
        }

        s.b[1282] = (s.v[360] < 40.0);
        s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });

        if (s.b[1281] && s.b[1282]) {
            s.store_div_from_scalar_offset_ad(362, 0.5, A::cosh(s.ad_value(360)), (-1.0));
        }

        if (s.b[1281] && (!s.b[1282])) {
            s.store_limited_exp_neg_input(362, 360);
        }

        if (!s.b[1281]) {
            s.store_scalar(362, p.p174);
        }

        s.b[1283] = (!param_given[173]);
        s.store_scalar(1283, if s.b[1283] { 1.0 } else { 0.0 });

        if s.b[1283] {
            s.store_offset_div_scaled_product(360, s.ad_value(678), s.ad_value(153), 1.0, s.ad_value(351), 1.0, 1e-6);
        }

        s.b[1284] = (s.v[360] < 40.0);
        s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });

        if (s.b[1283] && s.b[1284]) {
            s.store_div_from_scalar_offset_ad(363, 0.5, A::cosh(s.ad_value(360)), (-1.0));
        }

        if (s.b[1283] && (!s.b[1284])) {
            s.store_limited_exp_neg_input(363, 360);
        }

        if (!s.b[1283]) {
            s.store_scalar(363, p.p173);
        }

        s.store_offset_sqrt_ad(364, A::offset(A::div(s.ad_value(803), s.ad_value(153)), 1.0), (-1.0));

        s.store_offset_div_scaled_product(360, s.ad_value(678), s.ad_value(153), 1.0, s.ad_value(351), 1.0, 1e-6);

        s.b[1285] = (s.v[360] < 40.0);
        s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });

        if s.b[1285] {
            s.store_div_from_scalar_ad(365, 1.0, A::max_with_scalar(A::scale_offset(A::cosh(s.ad_value(360)), p.p171, (((((-2.0)) * (p.p171))) + (1.0))), 1e-6));
        }

        if (!s.b[1285]) {
            let assign13360_ad_e17673: A = A::limited_exp_scaled_input(s.ad_value(360), -1.0);
            s.store_div_ad(365, assign13360_ad_e17673, A::max_with_scalar(A::offset(assign13360_ad_e17673, p.p171), 1e-6));
        }

        s.store_div_scaled_product_indices(396, 640, 894, 1.60219e-19, 893, 1.0);

        s.b[1286] = (p.p60 == 1.0);
        s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });

        if s.b[1286] {
            s.store_scalar(485, 745669000000.0);
        }

        if (!s.b[1286]) {
            s.store_scalar(485, 1166450000000.0);
        }

        s.store_scalar(168, (p.p1109 * p.p1109));

        s.store_scale(169, 742, p.p1109);

        s.store_square(170, 169);

        s.b[1287] = (p.p1717 < (-273.15));
        s.store_scalar(1287, if s.b[1287] { 1.0 } else { 0.0 });

        if s.b[1287] {
            s.store_scalar(228, 300.15);
        }

        if (!s.b[1287]) {
            s.store_scalar(228, (p.p1717 + 273.15));
        }

        s.b[1288] = (p.p57 == 1.0);
        s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });

        if s.b[1288] {
            s.store_add_ad_lhs(960, A::scale_offset(s.ad_value(882), (-1.0 / ((1.0 + { let limited_exp_arg = (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((p.p1806) * (1.0 / ((1.0 + { let limited_exp_arg = (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))), 882);
        }

        if s.b[1288] {
            s.store_add_ad_lhs(961, A::scale_offset(s.ad_value(883), (-1.0 / ((1.0 + { let limited_exp_arg = (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((p.p1813) * (1.0 / ((1.0 + { let limited_exp_arg = (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))), 883);
        }

        if s.b[1288] {
            s.store_add_ad_lhs(962, A::scale_offset(s.ad_value(884), (-1.0 / ((1.0 + { let limited_exp_arg = (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((p.p1820) * (1.0 / ((1.0 + { let limited_exp_arg = (((p.p1827 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1828); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))), 884);
        }

        if s.b[1288] {
            s.store_scaled_add_sqrt_square_offset_ad(963, A::offset(s.ad_value(885), ((-p.p1847) / (1.0 + { let limited_exp_arg = (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((0.25 * 0.001) * 0.001), 0.5);
        }

        if s.b[1288] {
            s.store_scaled_add_sqrt_square_offset_ad(964, A::offset(s.ad_value(886), ((-p.p1848) / (1.0 + { let limited_exp_arg = (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((0.25 * 0.001) * 0.001), 0.5);
        }

        if s.b[1288] {
            s.store_scaled_add_sqrt_square_offset_ad(965, A::offset(s.ad_value(887), ((-p.p1849) / (1.0 + { let limited_exp_arg = (((p.p1850 * 1000000000.0) - (p.p43 * 1000000000.0)) / p.p1851); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((0.25 * 0.001) * 0.001), 0.5);
        }

        if s.b[1288] {
            let assign13590_ad_e18065: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(960), (-1.001), 1.001), ((0.25 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs3_offset_mixed_iaa(966, 960, ((0.5 * 1.001) * 0.5), assign13590_ad_e18065, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(960), (-1.001), 1.001), 0.5, assign13590_ad_e18065, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-1.001) * 1.001))) * 0.5), (0.25 * 0.001));
        }

        if s.b[1288] {
            let assign13600_ad_e18185: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(960), (-2.001), 1.001), ((0.25 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs3_offset_mixed_iaa(969, 960, ((0.5 * 1.001) * 0.5), assign13600_ad_e18185, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(960), (-2.001), 1.001), 0.5, assign13600_ad_e18185, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-2.001) * 1.001))) * 0.5), (0.25 * 0.001));
        }

        if s.b[1288] {
            let assign13610_ad_e18305: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(961), (-1.001), 1.001), ((0.25 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs3_offset_mixed_iaa(967, 961, ((0.5 * 1.001) * 0.5), assign13610_ad_e18305, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(961), (-1.001), 1.001), 0.5, assign13610_ad_e18305, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-1.001) * 1.001))) * 0.5), (0.25 * 0.001));
        }

        if s.b[1288] {
            let assign13620_ad_e18425: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(961), (-2.001), 1.001), ((0.25 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs3_offset_mixed_iaa(970, 961, ((0.5 * 1.001) * 0.5), assign13620_ad_e18425, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(961), (-2.001), 1.001), 0.5, assign13620_ad_e18425, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-2.001) * 1.001))) * 0.5), (0.25 * 0.001));
        }

        if s.b[1288] {
            let assign13630_ad_e18545: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(962), (-1.001), 1.001), ((0.25 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs3_offset_mixed_iaa(968, 962, ((0.5 * 1.001) * 0.5), assign13630_ad_e18545, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(962), (-1.001), 1.001), 0.5, assign13630_ad_e18545, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-1.001) * 1.001))) * 0.5), (0.25 * 0.001));
        }

        if s.b[1288] {
            let assign13640_ad_e18665: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(962), (-2.001), 1.001), ((0.25 * 0.001) * 0.001));
            s.store_offset_add_scaled_inputs3_offset_mixed_iaa(971, 962, ((0.5 * 1.001) * 0.5), assign13640_ad_e18665, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(962), (-2.001), 1.001), 0.5, assign13640_ad_e18665, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-2.001) * 1.001))) * 0.5), (0.25 * 0.001));
        }

        if s.b[1288] {
            s.store_mul_pow_mixed_aii(976, A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(969)), 158, 966);
            s.store_div(979, 976, 893);
            s.store_mul_pow_mixed_aii(977, A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(970)), 158, 967);
            s.store_div(980, 977, 893);
            s.store_mul_pow_mixed_aii(978, A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(971)), 158, 968);
            s.store_div(981, 978, 893);
        }

        if s.b[1288] {
            s.store_scalar(982, (0.5 * (((1.0 / (1.0 + { let limited_exp_arg = ((2.75 - (p.p40 * 1000000000.0)) / 0.78); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) + 0.5) + ((((((1.0 / (1.0 + { let limited_exp_arg = ((2.75 - (p.p40 * 1000000000.0)) / 0.78); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - 0.5) * ((1.0 / (1.0 + { let limited_exp_arg = ((2.75 - (p.p40 * 1000000000.0)) / 0.78); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - 0.5)) + ((0.25 * 0.003) * 0.003))) as f64).sqrt())));
        }

        if s.b[1288] {
            s.store_add_div_lhs(983, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(982), A::sub(s.ad_value(960), s.ad_value(882))), A::sub_from_scalar(p.p1806, s.ad_value(882)), 982);
            s.store_div_from_scalar_offset_ad(984, 1.0, A::limited_exp_scaled_input(A::offset(s.ad_value(983), (-0.999)), 1.0 / (0.0001)), 1.0);
            s.store_scalar(1013, (((((0.5 * p.p40) * p.p40) * 1e18) - ((1.5 * p.p40) * 1000000000.0)) + 2.0));
            s.store_offset_sub_scaled_inputs(1014, A::offset(s.ad_value(1013), 4.0), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(1013), (-4.0)), ((0.25 * 0.01) * 0.01)), 0.5, (0.25 * 0.01));
        }

        if s.b[1288] {
            let assign13760_ad_e18948: A = A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1893);
            let assign13760_ad_e19005: A = A::sqrt_square_offset(A::scale_offset(assign13760_ad_e18948, ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893))), ((s.v[168]) + ((-18100.0)))), ((0.25 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs3_offset(974, assign13760_ad_e18948, ((0.5 * ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893)))) * 0.5), assign13760_ad_e19005, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scale_offset(assign13760_ad_e18948, ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p.p1893))), ((s.v[168]) + (18100.0))), 0.5, assign13760_ad_e19005, 0.5), (-924000.0)), ((0.25 * 9240.0) * 9240.0)), (-0.5), ((924000.0 + (0.5 * ((s.v[168]) + (18100.0)))) * 0.5), (0.25 * 9240.0));
        }

        if s.b[1288] {
            let assign13770_ad_e19176: A = A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1894);
            let assign13770_ad_e19233: A = A::sqrt_square_offset(A::scale_offset(assign13770_ad_e19176, ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894))), 5.5), ((0.25 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs3_offset(975, assign13770_ad_e19176, ((0.5 * ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894)))) * 0.5), assign13770_ad_e19233, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scale_offset(assign13770_ad_e19176, ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p.p1894))), 5.5), 0.5, assign13770_ad_e19233, 0.5), (-8.0)), ((0.25 * 0.01) * 0.01)), (-0.5), ((8.0 + (0.5 * 5.5)) * 0.5), (0.25 * 0.01));
        }

        if s.b[1288] {
            s.store_scalar(972, ((120.66 * ((4.0) as f64).powf(p.p1895)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1895)));
            s.store_scalar(973, ((2.0 * ((4.0) as f64).powf(p.p1896)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1896)));
            s.store_scalar(989, ((107.0 * ((4.0) as f64).powf(p.p1897)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1897)));
        }

        if s.b[1288] {
            let assign13810_ad_e19446: A = A::powf(A::sub_from_scalar((p.p40 * 1000000000.0), s.ad_value(1014)), p.p1898);
            let assign13810_ad_e19485: A = A::sqrt_square_offset(A::scale_offset(assign13810_ad_e19446, 0.1, ((0.7) + ((-0.5)))), ((0.25 * 0.01) * 0.01));
            s.store_offset_add_scaled_inputs3_offset(990, assign13810_ad_e19446, ((0.5 * 0.1) * 0.5), assign13810_ad_e19485, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scale_offset(assign13810_ad_e19446, 0.1, ((0.7) + (0.5))), 0.5, assign13810_ad_e19485, 0.5), (-1.0)), ((0.25 * 0.01) * 0.01)), (-0.5), ((1.0 + (0.5 * ((0.7) + (0.5)))) * 0.5), (0.25 * 0.01));
        }

        if s.b[1288] {
            s.store_scalar(991, ((103.0 * ((4.0) as f64).powf(p.p1899)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1899)));
            s.store_scalar(992, ((1.5 * ((4.0) as f64).powf(p.p1900)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1900)));
            s.store_scalar(993, ((833.0 * ((4.0) as f64).powf(p.p1901)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1901)));
            s.store_scalar(994, ((3.4 * ((4.0) as f64).powf(p.p1902)) / (((p.p40 * 1000000000.0)) as f64).powf(p.p1902)));
            s.store_div_ad_rhs(987, 974, A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(975), p.p1867)));
            s.store_div_ad_rhs(988, 972, A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(973), p.p1868)));
        }

        if s.b[1288] {
            let assign13880_ad_e19701: A = A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(975), p.p1867));
            s.store_add_scaled_inputs4_mixed_iaia(985, 888, 0.5, A::div(s.ad_value(974), assign13880_ad_e19701), (p.p1865 * 0.5), 987, ((-p.p1865) * 0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(888), 1.0, A::div(s.ad_value(974), assign13880_ad_e19701), p.p1865, s.ad_value(987), (-p.p1865)), ((0.25 * 0.01) * 0.01)), 0.5);
        }

        if s.b[1288] {
            let assign13890_ad_e19766: A = A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(973), p.p1868));
            s.store_add_scaled_inputs4_mixed_iaia(986, 889, 0.5, A::div(s.ad_value(972), assign13890_ad_e19766), (p.p1866 * 0.5), 988, ((-p.p1866) * 0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(889), 1.0, A::div(s.ad_value(972), assign13890_ad_e19766), p.p1866, s.ad_value(988), (-p.p1866)), ((0.25 * 0.01) * 0.01)), 0.5);
        }

        if s.b[1288] {
            let assign13900_ad_e19831: A = A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(990), p.p1890));
            let assign13900_ad_e19835: A = A::powf(A::scale_offset(assign13900_ad_e19831, 5.0, 1.0), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(995, A::div(s.ad_value(989), assign13900_ad_e19835), ((0.25 * 0.1) * 0.1), 0.5);
        }

        if s.b[1288] {
            let assign13910_ad_e19896: A = A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(990), p.p1890));
            let assign13910_ad_e19900: A = A::powf(A::scale_offset(assign13910_ad_e19896, 5.0, 1.0), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(996, A::div(s.ad_value(989), assign13910_ad_e19900), ((0.25 * 0.1) * 0.1), 0.5);
        }

        if s.b[1288] {
            s.store_add_scaled_inputs3_indices(997, 890, 1.0, 995, p.p1887, 996, (-p.p1887));
        }

        if s.b[1288] {
            let assign13930_ad_e19971: A = A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(992), p.p1891));
            let assign13930_ad_e19975: A = A::powf(A::scale_offset(assign13930_ad_e19971, 5.0, 1.0), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(998, A::div(s.ad_value(991), assign13930_ad_e19975), ((0.25 * 0.1) * 0.1), 0.5);
        }

        if s.b[1288] {
            let assign13940_ad_e20036: A = A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(992), p.p1891));
            let assign13940_ad_e20040: A = A::powf(A::scale_offset(assign13940_ad_e20036, 5.0, 1.0), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(999, A::div(s.ad_value(991), assign13940_ad_e20040), ((0.25 * 0.1) * 0.1), 0.5);
        }

        if s.b[1288] {
            s.store_add_scaled_inputs3_indices(1000, 891, 1.0, 998, p.p1888, 999, (-p.p1888));
        }

        if s.b[1288] {
            let assign13960_ad_e20111: A = A::pow_from_scalar((p.p43 * 1000000000.0), A::scale(s.ad_value(994), p.p1892));
            let assign13960_ad_e20115: A = A::powf(A::scale_offset(assign13960_ad_e20111, 5.0, 1.0), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(1001, A::div(s.ad_value(993), assign13960_ad_e20115), ((0.25 * 0.1) * 0.1), 0.5);
        }

    }
}
