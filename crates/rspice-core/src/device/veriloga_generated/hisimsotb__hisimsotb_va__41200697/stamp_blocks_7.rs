#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_21(
        p: &Parameters,
        var_beta_inv: f64,
        var_beta_inv_dn4: f64,
        var_c_box: f64,
        var_c_box_fd_inv: f64,
        var_c_fox: f64,
        var_c_fox_dn0: f64,
        var_c_fox_dn10: f64,
        var_c_fox_dn11: f64,
        var_c_fox_dn12: f64,
        var_c_fox_dn2: f64,
        var_c_fox_dn4: f64,
        var_c_fox_dn5: f64,
        var_c_fox_dn6: f64,
        var_c_fox_dn8: f64,
        var_c_soi_inv: f64,
        var_cnst0bulk: f64,
        var_cnst0bulk_dn0: f64,
        var_cnst0bulk_dn10: f64,
        var_cnst0bulk_dn11: f64,
        var_cnst0bulk_dn12: f64,
        var_cnst0bulk_dn2: f64,
        var_cnst0bulk_dn4: f64,
        var_cnst0bulk_dn5: f64,
        var_cnst0bulk_dn6: f64,
        var_cnst0bulk_dn8: f64,
        var_guard74: f64,
        var_guard75: f64,
        var_lp_s0_max: f64,
        var_n_subbl: f64,
        var_n_subbl_dn0: f64,
        var_n_subbl_dn10: f64,
        var_n_subbl_dn11: f64,
        var_n_subbl_dn12: f64,
        var_n_subbl_dn2: f64,
        var_n_subbl_dn4: f64,
        var_n_subbl_dn5: f64,
        var_n_subbl_dn6: f64,
        var_n_subbl_dn8: f64,
        var_phi_b_dep0: f64,
        var_phi_b_dep0_dn0: f64,
        var_phi_b_dep0_dn10: f64,
        var_phi_b_dep0_dn11: f64,
        var_phi_b_dep0_dn12: f64,
        var_phi_b_dep0_dn2: f64,
        var_phi_b_dep0_dn4: f64,
        var_phi_b_dep0_dn5: f64,
        var_phi_b_dep0_dn6: f64,
        var_phi_b_dep0_dn8: f64,
        var_q_fd_soi: f64,
        var_q_fd_soi_dn0: f64,
        var_q_fd_soi_dn10: f64,
        var_q_fd_soi_dn11: f64,
        var_q_fd_soi_dn12: f64,
        var_q_fd_soi_dn2: f64,
        var_q_fd_soi_dn4: f64,
        var_q_fd_soi_dn5: f64,
        var_q_fd_soi_dn6: f64,
        var_q_fd_soi_dn8: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn4: f64,
        var_uc_nsubs_dn5: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn8: f64,
        var_vbsbiz: f64,
        var_vbsbiz_dn0: f64,
        var_vbsbiz_dn10: f64,
        var_vbsbiz_dn11: f64,
        var_vbsbiz_dn12: f64,
        var_vbsbiz_dn2: f64,
        var_vbsbiz_dn4: f64,
        var_vbsbiz_dn5: f64,
        var_vbsbiz_dn6: f64,
        var_vbsbiz_dn8: f64,
        var_vgs: f64,
        var_vgs_fb: f64,
        var_wdsoi: f64,
        var_dpsb_slot: &mut f64,
        var_dpsb_dn0_slot: &mut f64,
        var_dpsb_dn10_slot: &mut f64,
        var_dpsb_dn11_slot: &mut f64,
        var_dpsb_dn12_slot: &mut f64,
        var_dpsb_dn2_slot: &mut f64,
        var_dpsb_dn4_slot: &mut f64,
        var_dpsb_dn5_slot: &mut f64,
        var_dpsb_dn6_slot: &mut f64,
        var_dpsb_dn8_slot: &mut f64,
        var_dpsb_rv_slot: &mut f64,
        var_flg_depmode_slot: &mut f64,
        var_flg_depmode_rv_slot: &mut f64,
        var_flg_zone_slot: &mut f64,
        var_flg_zone_rv_slot: &mut f64,
        var_guard76_slot: &mut f64,
        var_guard76_rv_slot: &mut f64,
        var_guard77_slot: &mut f64,
        var_guard77_rv_slot: &mut f64,
        var_guard78_slot: &mut f64,
        var_guard78_rv_slot: &mut f64,
        var_guard79_slot: &mut f64,
        var_guard79_rv_slot: &mut f64,
        var_guard80_slot: &mut f64,
        var_guard80_rv_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
        var_lp_s0_rv_slot: &mut f64,
        var_pf1_slot: &mut f64,
        var_pf11_slot: &mut f64,
        var_pf11_dn0_slot: &mut f64,
        var_pf11_dn10_slot: &mut f64,
        var_pf11_dn11_slot: &mut f64,
        var_pf11_dn12_slot: &mut f64,
        var_pf11_dn2_slot: &mut f64,
        var_pf11_dn4_slot: &mut f64,
        var_pf11_dn5_slot: &mut f64,
        var_pf11_dn6_slot: &mut f64,
        var_pf11_dn8_slot: &mut f64,
        var_pf11_rv_slot: &mut f64,
        var_pf1_dn0_slot: &mut f64,
        var_pf1_dn10_slot: &mut f64,
        var_pf1_dn11_slot: &mut f64,
        var_pf1_dn12_slot: &mut f64,
        var_pf1_dn2_slot: &mut f64,
        var_pf1_dn4_slot: &mut f64,
        var_pf1_dn5_slot: &mut f64,
        var_pf1_dn6_slot: &mut f64,
        var_pf1_dn8_slot: &mut f64,
        var_pf1_rv_slot: &mut f64,
        var_phi_b_dep_slot: &mut f64,
        var_phi_b_dep_dn0_slot: &mut f64,
        var_phi_b_dep_dn10_slot: &mut f64,
        var_phi_b_dep_dn11_slot: &mut f64,
        var_phi_b_dep_dn12_slot: &mut f64,
        var_phi_b_dep_dn2_slot: &mut f64,
        var_phi_b_dep_dn4_slot: &mut f64,
        var_phi_b_dep_dn5_slot: &mut f64,
        var_phi_b_dep_dn6_slot: &mut f64,
        var_phi_b_dep_dn8_slot: &mut f64,
        var_phi_b_dep_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn12_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn12_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn0_slot: &mut f64,
        var_t8_dn10_slot: &mut f64,
        var_t8_dn11_slot: &mut f64,
        var_t8_dn12_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_rv_slot: &mut f64,
        var_vgs_shift_slot: &mut f64,
        var_vgs_shift_dn0_slot: &mut f64,
        var_vgs_shift_dn10_slot: &mut f64,
        var_vgs_shift_dn11_slot: &mut f64,
        var_vgs_shift_dn12_slot: &mut f64,
        var_vgs_shift_dn2_slot: &mut f64,
        var_vgs_shift_dn4_slot: &mut f64,
        var_vgs_shift_dn5_slot: &mut f64,
        var_vgs_shift_dn6_slot: &mut f64,
        var_vgs_shift_dn8_slot: &mut f64,
        var_vgs_shift_rv_slot: &mut f64,
    ) {
        let mut var_dpsb: f64 = *var_dpsb_slot;
        let mut var_dpsb_dn0: f64 = *var_dpsb_dn0_slot;
        let mut var_dpsb_dn10: f64 = *var_dpsb_dn10_slot;
        let mut var_dpsb_dn11: f64 = *var_dpsb_dn11_slot;
        let mut var_dpsb_dn12: f64 = *var_dpsb_dn12_slot;
        let mut var_dpsb_dn2: f64 = *var_dpsb_dn2_slot;
        let mut var_dpsb_dn4: f64 = *var_dpsb_dn4_slot;
        let mut var_dpsb_dn5: f64 = *var_dpsb_dn5_slot;
        let mut var_dpsb_dn6: f64 = *var_dpsb_dn6_slot;
        let mut var_dpsb_dn8: f64 = *var_dpsb_dn8_slot;
        let mut var_dpsb_rv: f64 = *var_dpsb_rv_slot;
        let mut var_flg_depmode: f64 = *var_flg_depmode_slot;
        let mut var_flg_depmode_rv: f64 = *var_flg_depmode_rv_slot;
        let mut var_flg_zone: f64 = *var_flg_zone_slot;
        let mut var_flg_zone_rv: f64 = *var_flg_zone_rv_slot;
        let mut var_guard76: f64 = *var_guard76_slot;
        let mut var_guard76_rv: f64 = *var_guard76_rv_slot;
        let mut var_guard77: f64 = *var_guard77_slot;
        let mut var_guard77_rv: f64 = *var_guard77_rv_slot;
        let mut var_guard78: f64 = *var_guard78_slot;
        let mut var_guard78_rv: f64 = *var_guard78_rv_slot;
        let mut var_guard79: f64 = *var_guard79_slot;
        let mut var_guard79_rv: f64 = *var_guard79_rv_slot;
        let mut var_guard80: f64 = *var_guard80_slot;
        let mut var_guard80_rv: f64 = *var_guard80_rv_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
        let mut var_lp_s0_rv: f64 = *var_lp_s0_rv_slot;
        let mut var_pf1: f64 = *var_pf1_slot;
        let mut var_pf11: f64 = *var_pf11_slot;
        let mut var_pf11_dn0: f64 = *var_pf11_dn0_slot;
        let mut var_pf11_dn10: f64 = *var_pf11_dn10_slot;
        let mut var_pf11_dn11: f64 = *var_pf11_dn11_slot;
        let mut var_pf11_dn12: f64 = *var_pf11_dn12_slot;
        let mut var_pf11_dn2: f64 = *var_pf11_dn2_slot;
        let mut var_pf11_dn4: f64 = *var_pf11_dn4_slot;
        let mut var_pf11_dn5: f64 = *var_pf11_dn5_slot;
        let mut var_pf11_dn6: f64 = *var_pf11_dn6_slot;
        let mut var_pf11_dn8: f64 = *var_pf11_dn8_slot;
        let mut var_pf11_rv: f64 = *var_pf11_rv_slot;
        let mut var_pf1_dn0: f64 = *var_pf1_dn0_slot;
        let mut var_pf1_dn10: f64 = *var_pf1_dn10_slot;
        let mut var_pf1_dn11: f64 = *var_pf1_dn11_slot;
        let mut var_pf1_dn12: f64 = *var_pf1_dn12_slot;
        let mut var_pf1_dn2: f64 = *var_pf1_dn2_slot;
        let mut var_pf1_dn4: f64 = *var_pf1_dn4_slot;
        let mut var_pf1_dn5: f64 = *var_pf1_dn5_slot;
        let mut var_pf1_dn6: f64 = *var_pf1_dn6_slot;
        let mut var_pf1_dn8: f64 = *var_pf1_dn8_slot;
        let mut var_pf1_rv: f64 = *var_pf1_rv_slot;
        let mut var_phi_b_dep: f64 = *var_phi_b_dep_slot;
        let mut var_phi_b_dep_dn0: f64 = *var_phi_b_dep_dn0_slot;
        let mut var_phi_b_dep_dn10: f64 = *var_phi_b_dep_dn10_slot;
        let mut var_phi_b_dep_dn11: f64 = *var_phi_b_dep_dn11_slot;
        let mut var_phi_b_dep_dn12: f64 = *var_phi_b_dep_dn12_slot;
        let mut var_phi_b_dep_dn2: f64 = *var_phi_b_dep_dn2_slot;
        let mut var_phi_b_dep_dn4: f64 = *var_phi_b_dep_dn4_slot;
        let mut var_phi_b_dep_dn5: f64 = *var_phi_b_dep_dn5_slot;
        let mut var_phi_b_dep_dn6: f64 = *var_phi_b_dep_dn6_slot;
        let mut var_phi_b_dep_dn8: f64 = *var_phi_b_dep_dn8_slot;
        let mut var_phi_b_dep_rv: f64 = *var_phi_b_dep_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn12: f64 = *var_t6_dn12_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn12: f64 = *var_t7_dn12_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn0: f64 = *var_t8_dn0_slot;
        let mut var_t8_dn10: f64 = *var_t8_dn10_slot;
        let mut var_t8_dn11: f64 = *var_t8_dn11_slot;
        let mut var_t8_dn12: f64 = *var_t8_dn12_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_rv: f64 = *var_t8_rv_slot;
        let mut var_vgs_shift: f64 = *var_vgs_shift_slot;
        let mut var_vgs_shift_dn0: f64 = *var_vgs_shift_dn0_slot;
        let mut var_vgs_shift_dn10: f64 = *var_vgs_shift_dn10_slot;
        let mut var_vgs_shift_dn11: f64 = *var_vgs_shift_dn11_slot;
        let mut var_vgs_shift_dn12: f64 = *var_vgs_shift_dn12_slot;
        let mut var_vgs_shift_dn2: f64 = *var_vgs_shift_dn2_slot;
        let mut var_vgs_shift_dn4: f64 = *var_vgs_shift_dn4_slot;
        let mut var_vgs_shift_dn5: f64 = *var_vgs_shift_dn5_slot;
        let mut var_vgs_shift_dn6: f64 = *var_vgs_shift_dn6_slot;
        let mut var_vgs_shift_dn8: f64 = *var_vgs_shift_dn8_slot;
        let mut var_vgs_shift_rv: f64 = *var_vgs_shift_rv_slot;

        let mut assign7150_loop_guard: usize = 0;
        while {
            let assign7150_cond_e5303: f64 = if (((var_guard74 == 0.0) && (var_guard75 != 0.0)) && (var_lp_s0 <= var_lp_s0_max)) { 1.0 } else { 0.0 };
            assign7150_cond_e5303 != 0.0
        } {
            assign7150_loop_guard += 1;
            assert!(assign7150_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign7150_body0_e5318, assign7150_body0_e5318_d_n0, assign7150_body0_e5318_d_n2, assign7150_body0_e5318_d_n4, assign7150_body0_e5318_d_n5, assign7150_body0_e5318_d_n6, assign7150_body0_e5318_d_n8, assign7150_body0_e5318_d_n10, assign7150_body0_e5318_d_n11, assign7150_body0_e5318_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body0_e5311: f64 = (2.0 * 1.6021918e-19);
        let assign7150_body0_e5313: f64 = (assign7150_body0_e5311 * 1.034943e-10);
        let assign7150_body0_e5315: f64 = (assign7150_body0_e5313 * var_n_subbl);
        let assign7150_body0_e5316: f64 = (var_c_box / assign7150_body0_e5315);
        (assign7150_body0_e5316, (-((var_c_box * (assign7150_body0_e5313 * var_n_subbl_dn0)) / (assign7150_body0_e5315 * assign7150_body0_e5315))), (-((var_c_box * (assign7150_body0_e5313 * var_n_subbl_dn2)) / (assign7150_body0_e5315 * assign7150_body0_e5315))), (-((var_c_box * (assign7150_body0_e5313 * var_n_subbl_dn4)) / (assign7150_body0_e5315 * assign7150_body0_e5315))), (-((var_c_box * (assign7150_body0_e5313 * var_n_subbl_dn5)) / (assign7150_body0_e5315 * assign7150_body0_e5315))), (-((var_c_box * (assign7150_body0_e5313 * var_n_subbl_dn6)) / (assign7150_body0_e5315 * assign7150_body0_e5315))), (-((var_c_box * (assign7150_body0_e5313 * var_n_subbl_dn8)) / (assign7150_body0_e5315 * assign7150_body0_e5315))), (-((var_c_box * (assign7150_body0_e5313 * var_n_subbl_dn10)) / (assign7150_body0_e5315 * assign7150_body0_e5315))), (-((var_c_box * (assign7150_body0_e5313 * var_n_subbl_dn11)) / (assign7150_body0_e5315 * assign7150_body0_e5315))), (-((var_c_box * (assign7150_body0_e5313 * var_n_subbl_dn12)) / (assign7150_body0_e5315 * assign7150_body0_e5315))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
            var_t0 = assign7150_body0_e5318;
            var_t0_dn0 = assign7150_body0_e5318_d_n0;
            var_t0_dn2 = assign7150_body0_e5318_d_n2;
            var_t0_dn4 = assign7150_body0_e5318_d_n4;
            var_t0_dn5 = assign7150_body0_e5318_d_n5;
            var_t0_dn6 = assign7150_body0_e5318_d_n6;
            var_t0_dn8 = assign7150_body0_e5318_d_n8;
            var_t0_dn10 = assign7150_body0_e5318_d_n10;
            var_t0_dn11 = assign7150_body0_e5318_d_n11;
            var_t0_dn12 = assign7150_body0_e5318_d_n12;
            var_t0_rv = 0.0;
            let (assign7150_body1_e5329, assign7150_body1_e5329_d_n0, assign7150_body1_e5329_d_n2, assign7150_body1_e5329_d_n4, assign7150_body1_e5329_d_n5, assign7150_body1_e5329_d_n6, assign7150_body1_e5329_d_n8, assign7150_body1_e5329_d_n10, assign7150_body1_e5329_d_n11, assign7150_body1_e5329_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body1_e5326: f64 = (var_c_box * var_c_soi_inv);
        let assign7150_body1_e5327: f64 = (1.0 + assign7150_body1_e5326);
        (assign7150_body1_e5327, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
            var_t1 = assign7150_body1_e5329;
            var_t1_dn0 = assign7150_body1_e5329_d_n0;
            var_t1_dn2 = assign7150_body1_e5329_d_n2;
            var_t1_dn4 = assign7150_body1_e5329_d_n4;
            var_t1_dn5 = assign7150_body1_e5329_d_n5;
            var_t1_dn6 = assign7150_body1_e5329_d_n6;
            var_t1_dn8 = assign7150_body1_e5329_d_n8;
            var_t1_dn10 = assign7150_body1_e5329_d_n10;
            var_t1_dn11 = assign7150_body1_e5329_d_n11;
            var_t1_dn12 = assign7150_body1_e5329_d_n12;
            var_t1_rv = 0.0;
            let (assign7150_body2_e5347, assign7150_body2_e5347_d_n0, assign7150_body2_e5347_d_n2, assign7150_body2_e5347_d_n4, assign7150_body2_e5347_d_n5, assign7150_body2_e5347_d_n6, assign7150_body2_e5347_d_n8, assign7150_body2_e5347_d_n10, assign7150_body2_e5347_d_n11, assign7150_body2_e5347_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body2_e5337: f64 = (-var_q_fd_soi);
        let assign7150_body2_e5338: f64 = (0.5 * assign7150_body2_e5337);
        let assign7150_body2_e5340: f64 = (assign7150_body2_e5338 * var_c_soi_inv);
        let assign7150_body2_e5342: f64 = (assign7150_body2_e5340 + var_beta_inv);
        let assign7150_body2_e5344: f64 = (assign7150_body2_e5342 + var_vbsbiz);
        let assign7150_body2_e5345: f64 = (var_c_box * assign7150_body2_e5344);
        (assign7150_body2_e5345, (var_c_box * (((0.5 * (-var_q_fd_soi_dn0)) * var_c_soi_inv) + var_vbsbiz_dn0)), (var_c_box * (((0.5 * (-var_q_fd_soi_dn2)) * var_c_soi_inv) + var_vbsbiz_dn2)), (var_c_box * ((((0.5 * (-var_q_fd_soi_dn4)) * var_c_soi_inv) + var_beta_inv_dn4) + var_vbsbiz_dn4)), (var_c_box * (((0.5 * (-var_q_fd_soi_dn5)) * var_c_soi_inv) + var_vbsbiz_dn5)), (var_c_box * (((0.5 * (-var_q_fd_soi_dn6)) * var_c_soi_inv) + var_vbsbiz_dn6)), (var_c_box * (((0.5 * (-var_q_fd_soi_dn8)) * var_c_soi_inv) + var_vbsbiz_dn8)), (var_c_box * (((0.5 * (-var_q_fd_soi_dn10)) * var_c_soi_inv) + var_vbsbiz_dn10)), (var_c_box * (((0.5 * (-var_q_fd_soi_dn11)) * var_c_soi_inv) + var_vbsbiz_dn11)), (var_c_box * (((0.5 * (-var_q_fd_soi_dn12)) * var_c_soi_inv) + var_vbsbiz_dn12)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
            var_t2 = assign7150_body2_e5347;
            var_t2_dn0 = assign7150_body2_e5347_d_n0;
            var_t2_dn2 = assign7150_body2_e5347_d_n2;
            var_t2_dn4 = assign7150_body2_e5347_d_n4;
            var_t2_dn5 = assign7150_body2_e5347_d_n5;
            var_t2_dn6 = assign7150_body2_e5347_d_n6;
            var_t2_dn8 = assign7150_body2_e5347_d_n8;
            var_t2_dn10 = assign7150_body2_e5347_d_n10;
            var_t2_dn11 = assign7150_body2_e5347_d_n11;
            var_t2_dn12 = assign7150_body2_e5347_d_n12;
            var_t2_rv = 0.0;
            let (assign7150_body3_e5360, assign7150_body3_e5360_d_n0, assign7150_body3_e5360_d_n2, assign7150_body3_e5360_d_n4, assign7150_body3_e5360_d_n5, assign7150_body3_e5360_d_n6, assign7150_body3_e5360_d_n8, assign7150_body3_e5360_d_n10, assign7150_body3_e5360_d_n11, assign7150_body3_e5360_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body3_e5354: f64 = (2.0 * var_t0);
        let assign7150_body3_e5356: f64 = (assign7150_body3_e5354 * var_c_fox);
        let assign7150_body3_e5358: f64 = (assign7150_body3_e5356 * var_c_fox);
        (assign7150_body3_e5358, (((((2.0 * var_t0_dn0) * var_c_fox) + (assign7150_body3_e5354 * var_c_fox_dn0)) * var_c_fox) + (assign7150_body3_e5356 * var_c_fox_dn0)), (((((2.0 * var_t0_dn2) * var_c_fox) + (assign7150_body3_e5354 * var_c_fox_dn2)) * var_c_fox) + (assign7150_body3_e5356 * var_c_fox_dn2)), (((((2.0 * var_t0_dn4) * var_c_fox) + (assign7150_body3_e5354 * var_c_fox_dn4)) * var_c_fox) + (assign7150_body3_e5356 * var_c_fox_dn4)), (((((2.0 * var_t0_dn5) * var_c_fox) + (assign7150_body3_e5354 * var_c_fox_dn5)) * var_c_fox) + (assign7150_body3_e5356 * var_c_fox_dn5)), (((((2.0 * var_t0_dn6) * var_c_fox) + (assign7150_body3_e5354 * var_c_fox_dn6)) * var_c_fox) + (assign7150_body3_e5356 * var_c_fox_dn6)), (((((2.0 * var_t0_dn8) * var_c_fox) + (assign7150_body3_e5354 * var_c_fox_dn8)) * var_c_fox) + (assign7150_body3_e5356 * var_c_fox_dn8)), (((((2.0 * var_t0_dn10) * var_c_fox) + (assign7150_body3_e5354 * var_c_fox_dn10)) * var_c_fox) + (assign7150_body3_e5356 * var_c_fox_dn10)), (((((2.0 * var_t0_dn11) * var_c_fox) + (assign7150_body3_e5354 * var_c_fox_dn11)) * var_c_fox) + (assign7150_body3_e5356 * var_c_fox_dn11)), (((((2.0 * var_t0_dn12) * var_c_fox) + (assign7150_body3_e5354 * var_c_fox_dn12)) * var_c_fox) + (assign7150_body3_e5356 * var_c_fox_dn12)),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
            var_t7 = assign7150_body3_e5360;
            var_t7_dn0 = assign7150_body3_e5360_d_n0;
            var_t7_dn2 = assign7150_body3_e5360_d_n2;
            var_t7_dn4 = assign7150_body3_e5360_d_n4;
            var_t7_dn5 = assign7150_body3_e5360_d_n5;
            var_t7_dn6 = assign7150_body3_e5360_d_n6;
            var_t7_dn8 = assign7150_body3_e5360_d_n8;
            var_t7_dn10 = assign7150_body3_e5360_d_n10;
            var_t7_dn11 = assign7150_body3_e5360_d_n11;
            var_t7_dn12 = assign7150_body3_e5360_d_n12;
            var_t7_rv = 0.0;
            let (assign7150_body4_e5383, assign7150_body4_e5383_d_n0, assign7150_body4_e5383_d_n2, assign7150_body4_e5383_d_n4, assign7150_body4_e5383_d_n5, assign7150_body4_e5383_d_n6, assign7150_body4_e5383_d_n8, assign7150_body4_e5383_d_n10, assign7150_body4_e5383_d_n11, assign7150_body4_e5383_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body4_e5368: f64 = (var_t1 * var_c_fox);
        let assign7150_body4_e5369: f64 = (var_c_box + assign7150_body4_e5368);
        let assign7150_body4_e5372: f64 = (2.0 * var_t0);
        let assign7150_body4_e5374: f64 = (assign7150_body4_e5372 * var_c_fox);
        let assign7150_body4_e5376: f64 = (assign7150_body4_e5374 * var_q_fd_soi);
        let assign7150_body4_e5377: f64 = (assign7150_body4_e5369 + assign7150_body4_e5376);
        let assign7150_body4_e5380: f64 = (var_t7 * var_vgs_shift);
        let assign7150_body4_e5381: f64 = (assign7150_body4_e5377 + assign7150_body4_e5380);
        (assign7150_body4_e5381, ((((var_t1_dn0 * var_c_fox) + (var_t1 * var_c_fox_dn0)) + (((((2.0 * var_t0_dn0) * var_c_fox) + (assign7150_body4_e5372 * var_c_fox_dn0)) * var_q_fd_soi) + (assign7150_body4_e5374 * var_q_fd_soi_dn0))) + ((var_t7_dn0 * var_vgs_shift) + (var_t7 * var_vgs_shift_dn0))), ((((var_t1_dn2 * var_c_fox) + (var_t1 * var_c_fox_dn2)) + (((((2.0 * var_t0_dn2) * var_c_fox) + (assign7150_body4_e5372 * var_c_fox_dn2)) * var_q_fd_soi) + (assign7150_body4_e5374 * var_q_fd_soi_dn2))) + ((var_t7_dn2 * var_vgs_shift) + (var_t7 * var_vgs_shift_dn2))), ((((var_t1_dn4 * var_c_fox) + (var_t1 * var_c_fox_dn4)) + (((((2.0 * var_t0_dn4) * var_c_fox) + (assign7150_body4_e5372 * var_c_fox_dn4)) * var_q_fd_soi) + (assign7150_body4_e5374 * var_q_fd_soi_dn4))) + ((var_t7_dn4 * var_vgs_shift) + (var_t7 * var_vgs_shift_dn4))), ((((var_t1_dn5 * var_c_fox) + (var_t1 * var_c_fox_dn5)) + (((((2.0 * var_t0_dn5) * var_c_fox) + (assign7150_body4_e5372 * var_c_fox_dn5)) * var_q_fd_soi) + (assign7150_body4_e5374 * var_q_fd_soi_dn5))) + ((var_t7_dn5 * var_vgs_shift) + (var_t7 * var_vgs_shift_dn5))), ((((var_t1_dn6 * var_c_fox) + (var_t1 * var_c_fox_dn6)) + (((((2.0 * var_t0_dn6) * var_c_fox) + (assign7150_body4_e5372 * var_c_fox_dn6)) * var_q_fd_soi) + (assign7150_body4_e5374 * var_q_fd_soi_dn6))) + ((var_t7_dn6 * var_vgs_shift) + (var_t7 * var_vgs_shift_dn6))), ((((var_t1_dn8 * var_c_fox) + (var_t1 * var_c_fox_dn8)) + (((((2.0 * var_t0_dn8) * var_c_fox) + (assign7150_body4_e5372 * var_c_fox_dn8)) * var_q_fd_soi) + (assign7150_body4_e5374 * var_q_fd_soi_dn8))) + ((var_t7_dn8 * var_vgs_shift) + (var_t7 * var_vgs_shift_dn8))), ((((var_t1_dn10 * var_c_fox) + (var_t1 * var_c_fox_dn10)) + (((((2.0 * var_t0_dn10) * var_c_fox) + (assign7150_body4_e5372 * var_c_fox_dn10)) * var_q_fd_soi) + (assign7150_body4_e5374 * var_q_fd_soi_dn10))) + ((var_t7_dn10 * var_vgs_shift) + (var_t7 * var_vgs_shift_dn10))), ((((var_t1_dn11 * var_c_fox) + (var_t1 * var_c_fox_dn11)) + (((((2.0 * var_t0_dn11) * var_c_fox) + (assign7150_body4_e5372 * var_c_fox_dn11)) * var_q_fd_soi) + (assign7150_body4_e5374 * var_q_fd_soi_dn11))) + ((var_t7_dn11 * var_vgs_shift) + (var_t7 * var_vgs_shift_dn11))), ((((var_t1_dn12 * var_c_fox) + (var_t1 * var_c_fox_dn12)) + (((((2.0 * var_t0_dn12) * var_c_fox) + (assign7150_body4_e5372 * var_c_fox_dn12)) * var_q_fd_soi) + (assign7150_body4_e5374 * var_q_fd_soi_dn12))) + ((var_t7_dn12 * var_vgs_shift) + (var_t7 * var_vgs_shift_dn12))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign7150_body4_e5383;
            var_t4_dn0 = assign7150_body4_e5383_d_n0;
            var_t4_dn2 = assign7150_body4_e5383_d_n2;
            var_t4_dn4 = assign7150_body4_e5383_d_n4;
            var_t4_dn5 = assign7150_body4_e5383_d_n5;
            var_t4_dn6 = assign7150_body4_e5383_d_n6;
            var_t4_dn8 = assign7150_body4_e5383_d_n8;
            var_t4_dn10 = assign7150_body4_e5383_d_n10;
            var_t4_dn11 = assign7150_body4_e5383_d_n11;
            var_t4_dn12 = assign7150_body4_e5383_d_n12;
            var_t4_rv = 0.0;
            let (assign7150_body5_e5400, assign7150_body5_e5400_d_n0, assign7150_body5_e5400_d_n2, assign7150_body5_e5400_d_n4, assign7150_body5_e5400_d_n5, assign7150_body5_e5400_d_n6, assign7150_body5_e5400_d_n8, assign7150_body5_e5400_d_n10, assign7150_body5_e5400_d_n11, assign7150_body5_e5400_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body5_e5390: f64 = (2.0 * var_c_box);
        let assign7150_body5_e5392: f64 = (assign7150_body5_e5390 * var_c_fox);
        let assign7150_body5_e5394: f64 = (assign7150_body5_e5392 * 2.0);
        let assign7150_body5_e5396: f64 = (assign7150_body5_e5394 * var_t0);
        let assign7150_body5_e5398: f64 = (assign7150_body5_e5396 * var_c_fox);
        (assign7150_body5_e5398, ((((((assign7150_body5_e5390 * var_c_fox_dn0) * 2.0) * var_t0) + (assign7150_body5_e5394 * var_t0_dn0)) * var_c_fox) + (assign7150_body5_e5396 * var_c_fox_dn0)), ((((((assign7150_body5_e5390 * var_c_fox_dn2) * 2.0) * var_t0) + (assign7150_body5_e5394 * var_t0_dn2)) * var_c_fox) + (assign7150_body5_e5396 * var_c_fox_dn2)), ((((((assign7150_body5_e5390 * var_c_fox_dn4) * 2.0) * var_t0) + (assign7150_body5_e5394 * var_t0_dn4)) * var_c_fox) + (assign7150_body5_e5396 * var_c_fox_dn4)), ((((((assign7150_body5_e5390 * var_c_fox_dn5) * 2.0) * var_t0) + (assign7150_body5_e5394 * var_t0_dn5)) * var_c_fox) + (assign7150_body5_e5396 * var_c_fox_dn5)), ((((((assign7150_body5_e5390 * var_c_fox_dn6) * 2.0) * var_t0) + (assign7150_body5_e5394 * var_t0_dn6)) * var_c_fox) + (assign7150_body5_e5396 * var_c_fox_dn6)), ((((((assign7150_body5_e5390 * var_c_fox_dn8) * 2.0) * var_t0) + (assign7150_body5_e5394 * var_t0_dn8)) * var_c_fox) + (assign7150_body5_e5396 * var_c_fox_dn8)), ((((((assign7150_body5_e5390 * var_c_fox_dn10) * 2.0) * var_t0) + (assign7150_body5_e5394 * var_t0_dn10)) * var_c_fox) + (assign7150_body5_e5396 * var_c_fox_dn10)), ((((((assign7150_body5_e5390 * var_c_fox_dn11) * 2.0) * var_t0) + (assign7150_body5_e5394 * var_t0_dn11)) * var_c_fox) + (assign7150_body5_e5396 * var_c_fox_dn11)), ((((((assign7150_body5_e5390 * var_c_fox_dn12) * 2.0) * var_t0) + (assign7150_body5_e5394 * var_t0_dn12)) * var_c_fox) + (assign7150_body5_e5396 * var_c_fox_dn12)),)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn8, var_t8_dn10, var_t8_dn11, var_t8_dn12,)
    }
};
            var_t8 = assign7150_body5_e5400;
            var_t8_dn0 = assign7150_body5_e5400_d_n0;
            var_t8_dn2 = assign7150_body5_e5400_d_n2;
            var_t8_dn4 = assign7150_body5_e5400_d_n4;
            var_t8_dn5 = assign7150_body5_e5400_d_n5;
            var_t8_dn6 = assign7150_body5_e5400_d_n6;
            var_t8_dn8 = assign7150_body5_e5400_d_n8;
            var_t8_dn10 = assign7150_body5_e5400_d_n10;
            var_t8_dn11 = assign7150_body5_e5400_d_n11;
            var_t8_dn12 = assign7150_body5_e5400_d_n12;
            var_t8_rv = 0.0;
            let (assign7150_body6_e5441, assign7150_body6_e5441_d_n0, assign7150_body6_e5441_d_n2, assign7150_body6_e5441_d_n4, assign7150_body6_e5441_d_n5, assign7150_body6_e5441_d_n6, assign7150_body6_e5441_d_n8, assign7150_body6_e5441_d_n10, assign7150_body6_e5441_d_n11, assign7150_body6_e5441_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body6_e5407: f64 = (var_c_box * var_c_box);
        let assign7150_body6_e5410: f64 = (var_t1 * var_t1);
        let assign7150_body6_e5413: f64 = (4.0 * var_t0);
        let assign7150_body6_e5415: f64 = (assign7150_body6_e5413 * var_t2);
        let assign7150_body6_e5416: f64 = (assign7150_body6_e5410 - assign7150_body6_e5415);
        let assign7150_body6_e5418: f64 = (assign7150_body6_e5416 * var_c_fox);
        let assign7150_body6_e5420: f64 = (assign7150_body6_e5418 * var_c_fox);
        let assign7150_body6_e5421: f64 = (assign7150_body6_e5407 + assign7150_body6_e5420);
        let assign7150_body6_e5424: f64 = (2.0 * var_c_box);
        let assign7150_body6_e5426: f64 = (assign7150_body6_e5424 * var_c_fox);
        let assign7150_body6_e5430: f64 = (2.0 * var_t0);
        let assign7150_body6_e5432: f64 = (assign7150_body6_e5430 * var_q_fd_soi);
        let assign7150_body6_e5433: f64 = (var_t1 + assign7150_body6_e5432);
        let assign7150_body6_e5434: f64 = (assign7150_body6_e5426 * assign7150_body6_e5433);
        let assign7150_body6_e5435: f64 = (assign7150_body6_e5421 + assign7150_body6_e5434);
        let assign7150_body6_e5438: f64 = (var_t8 * var_vgs_shift);
        let assign7150_body6_e5439: f64 = (assign7150_body6_e5435 + assign7150_body6_e5438);
        (assign7150_body6_e5439, (((((((((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) - (((4.0 * var_t0_dn0) * var_t2) + (assign7150_body6_e5413 * var_t2_dn0))) * var_c_fox) + (assign7150_body6_e5416 * var_c_fox_dn0)) * var_c_fox) + (assign7150_body6_e5418 * var_c_fox_dn0)) + (((assign7150_body6_e5424 * var_c_fox_dn0) * assign7150_body6_e5433) + (assign7150_body6_e5426 * (var_t1_dn0 + (((2.0 * var_t0_dn0) * var_q_fd_soi) + (assign7150_body6_e5430 * var_q_fd_soi_dn0)))))) + ((var_t8_dn0 * var_vgs_shift) + (var_t8 * var_vgs_shift_dn0))), (((((((((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) - (((4.0 * var_t0_dn2) * var_t2) + (assign7150_body6_e5413 * var_t2_dn2))) * var_c_fox) + (assign7150_body6_e5416 * var_c_fox_dn2)) * var_c_fox) + (assign7150_body6_e5418 * var_c_fox_dn2)) + (((assign7150_body6_e5424 * var_c_fox_dn2) * assign7150_body6_e5433) + (assign7150_body6_e5426 * (var_t1_dn2 + (((2.0 * var_t0_dn2) * var_q_fd_soi) + (assign7150_body6_e5430 * var_q_fd_soi_dn2)))))) + ((var_t8_dn2 * var_vgs_shift) + (var_t8 * var_vgs_shift_dn2))), (((((((((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) - (((4.0 * var_t0_dn4) * var_t2) + (assign7150_body6_e5413 * var_t2_dn4))) * var_c_fox) + (assign7150_body6_e5416 * var_c_fox_dn4)) * var_c_fox) + (assign7150_body6_e5418 * var_c_fox_dn4)) + (((assign7150_body6_e5424 * var_c_fox_dn4) * assign7150_body6_e5433) + (assign7150_body6_e5426 * (var_t1_dn4 + (((2.0 * var_t0_dn4) * var_q_fd_soi) + (assign7150_body6_e5430 * var_q_fd_soi_dn4)))))) + ((var_t8_dn4 * var_vgs_shift) + (var_t8 * var_vgs_shift_dn4))), (((((((((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) - (((4.0 * var_t0_dn5) * var_t2) + (assign7150_body6_e5413 * var_t2_dn5))) * var_c_fox) + (assign7150_body6_e5416 * var_c_fox_dn5)) * var_c_fox) + (assign7150_body6_e5418 * var_c_fox_dn5)) + (((assign7150_body6_e5424 * var_c_fox_dn5) * assign7150_body6_e5433) + (assign7150_body6_e5426 * (var_t1_dn5 + (((2.0 * var_t0_dn5) * var_q_fd_soi) + (assign7150_body6_e5430 * var_q_fd_soi_dn5)))))) + ((var_t8_dn5 * var_vgs_shift) + (var_t8 * var_vgs_shift_dn5))), (((((((((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) - (((4.0 * var_t0_dn6) * var_t2) + (assign7150_body6_e5413 * var_t2_dn6))) * var_c_fox) + (assign7150_body6_e5416 * var_c_fox_dn6)) * var_c_fox) + (assign7150_body6_e5418 * var_c_fox_dn6)) + (((assign7150_body6_e5424 * var_c_fox_dn6) * assign7150_body6_e5433) + (assign7150_body6_e5426 * (var_t1_dn6 + (((2.0 * var_t0_dn6) * var_q_fd_soi) + (assign7150_body6_e5430 * var_q_fd_soi_dn6)))))) + ((var_t8_dn6 * var_vgs_shift) + (var_t8 * var_vgs_shift_dn6))), (((((((((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) - (((4.0 * var_t0_dn8) * var_t2) + (assign7150_body6_e5413 * var_t2_dn8))) * var_c_fox) + (assign7150_body6_e5416 * var_c_fox_dn8)) * var_c_fox) + (assign7150_body6_e5418 * var_c_fox_dn8)) + (((assign7150_body6_e5424 * var_c_fox_dn8) * assign7150_body6_e5433) + (assign7150_body6_e5426 * (var_t1_dn8 + (((2.0 * var_t0_dn8) * var_q_fd_soi) + (assign7150_body6_e5430 * var_q_fd_soi_dn8)))))) + ((var_t8_dn8 * var_vgs_shift) + (var_t8 * var_vgs_shift_dn8))), (((((((((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) - (((4.0 * var_t0_dn10) * var_t2) + (assign7150_body6_e5413 * var_t2_dn10))) * var_c_fox) + (assign7150_body6_e5416 * var_c_fox_dn10)) * var_c_fox) + (assign7150_body6_e5418 * var_c_fox_dn10)) + (((assign7150_body6_e5424 * var_c_fox_dn10) * assign7150_body6_e5433) + (assign7150_body6_e5426 * (var_t1_dn10 + (((2.0 * var_t0_dn10) * var_q_fd_soi) + (assign7150_body6_e5430 * var_q_fd_soi_dn10)))))) + ((var_t8_dn10 * var_vgs_shift) + (var_t8 * var_vgs_shift_dn10))), (((((((((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) - (((4.0 * var_t0_dn11) * var_t2) + (assign7150_body6_e5413 * var_t2_dn11))) * var_c_fox) + (assign7150_body6_e5416 * var_c_fox_dn11)) * var_c_fox) + (assign7150_body6_e5418 * var_c_fox_dn11)) + (((assign7150_body6_e5424 * var_c_fox_dn11) * assign7150_body6_e5433) + (assign7150_body6_e5426 * (var_t1_dn11 + (((2.0 * var_t0_dn11) * var_q_fd_soi) + (assign7150_body6_e5430 * var_q_fd_soi_dn11)))))) + ((var_t8_dn11 * var_vgs_shift) + (var_t8 * var_vgs_shift_dn11))), (((((((((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)) - (((4.0 * var_t0_dn12) * var_t2) + (assign7150_body6_e5413 * var_t2_dn12))) * var_c_fox) + (assign7150_body6_e5416 * var_c_fox_dn12)) * var_c_fox) + (assign7150_body6_e5418 * var_c_fox_dn12)) + (((assign7150_body6_e5424 * var_c_fox_dn12) * assign7150_body6_e5433) + (assign7150_body6_e5426 * (var_t1_dn12 + (((2.0 * var_t0_dn12) * var_q_fd_soi) + (assign7150_body6_e5430 * var_q_fd_soi_dn12)))))) + ((var_t8_dn12 * var_vgs_shift) + (var_t8 * var_vgs_shift_dn12))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign7150_body6_e5441;
            var_t5_dn0 = assign7150_body6_e5441_d_n0;
            var_t5_dn2 = assign7150_body6_e5441_d_n2;
            var_t5_dn4 = assign7150_body6_e5441_d_n4;
            var_t5_dn5 = assign7150_body6_e5441_d_n5;
            var_t5_dn6 = assign7150_body6_e5441_d_n6;
            var_t5_dn8 = assign7150_body6_e5441_d_n8;
            var_t5_dn10 = assign7150_body6_e5441_d_n10;
            var_t5_dn11 = assign7150_body6_e5441_d_n11;
            var_t5_dn12 = assign7150_body6_e5441_d_n12;
            var_t5_rv = 0.0;
            let (assign7150_body7_e5449, assign7150_body7_e5449_d_n0, assign7150_body7_e5449_d_n2, assign7150_body7_e5449_d_n4, assign7150_body7_e5449_d_n5, assign7150_body7_e5449_d_n6, assign7150_body7_e5449_d_n8, assign7150_body7_e5449_d_n10, assign7150_body7_e5449_d_n11, assign7150_body7_e5449_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body7_e5447: f64 = (var_t5).sqrt();
        (assign7150_body7_e5447, (var_t5_dn0 / (2.0 * assign7150_body7_e5447)), (var_t5_dn2 / (2.0 * assign7150_body7_e5447)), (var_t5_dn4 / (2.0 * assign7150_body7_e5447)), (var_t5_dn5 / (2.0 * assign7150_body7_e5447)), (var_t5_dn6 / (2.0 * assign7150_body7_e5447)), (var_t5_dn8 / (2.0 * assign7150_body7_e5447)), (var_t5_dn10 / (2.0 * assign7150_body7_e5447)), (var_t5_dn11 / (2.0 * assign7150_body7_e5447)), (var_t5_dn12 / (2.0 * assign7150_body7_e5447)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign7150_body7_e5449;
            var_t5_dn0 = assign7150_body7_e5449_d_n0;
            var_t5_dn2 = assign7150_body7_e5449_d_n2;
            var_t5_dn4 = assign7150_body7_e5449_d_n4;
            var_t5_dn5 = assign7150_body7_e5449_d_n5;
            var_t5_dn6 = assign7150_body7_e5449_d_n6;
            var_t5_dn8 = assign7150_body7_e5449_d_n8;
            var_t5_dn10 = assign7150_body7_e5449_d_n10;
            var_t5_dn11 = assign7150_body7_e5449_d_n11;
            var_t5_dn12 = assign7150_body7_e5449_d_n12;
            var_t5_rv = 0.0;
            let (assign7150_body8_e5460, assign7150_body8_e5460_d_n0, assign7150_body8_e5460_d_n2, assign7150_body8_e5460_d_n4, assign7150_body8_e5460_d_n5, assign7150_body8_e5460_d_n6, assign7150_body8_e5460_d_n8, assign7150_body8_e5460_d_n10, assign7150_body8_e5460_d_n11, assign7150_body8_e5460_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body8_e5457: f64 = (2.0 * var_t5);
        let assign7150_body8_e5458: f64 = (var_t8 / assign7150_body8_e5457);
        (assign7150_body8_e5458, (((var_t8_dn0 * assign7150_body8_e5457) - (var_t8 * (2.0 * var_t5_dn0))) / (assign7150_body8_e5457 * assign7150_body8_e5457)), (((var_t8_dn2 * assign7150_body8_e5457) - (var_t8 * (2.0 * var_t5_dn2))) / (assign7150_body8_e5457 * assign7150_body8_e5457)), (((var_t8_dn4 * assign7150_body8_e5457) - (var_t8 * (2.0 * var_t5_dn4))) / (assign7150_body8_e5457 * assign7150_body8_e5457)), (((var_t8_dn5 * assign7150_body8_e5457) - (var_t8 * (2.0 * var_t5_dn5))) / (assign7150_body8_e5457 * assign7150_body8_e5457)), (((var_t8_dn6 * assign7150_body8_e5457) - (var_t8 * (2.0 * var_t5_dn6))) / (assign7150_body8_e5457 * assign7150_body8_e5457)), (((var_t8_dn8 * assign7150_body8_e5457) - (var_t8 * (2.0 * var_t5_dn8))) / (assign7150_body8_e5457 * assign7150_body8_e5457)), (((var_t8_dn10 * assign7150_body8_e5457) - (var_t8 * (2.0 * var_t5_dn10))) / (assign7150_body8_e5457 * assign7150_body8_e5457)), (((var_t8_dn11 * assign7150_body8_e5457) - (var_t8 * (2.0 * var_t5_dn11))) / (assign7150_body8_e5457 * assign7150_body8_e5457)), (((var_t8_dn12 * assign7150_body8_e5457) - (var_t8 * (2.0 * var_t5_dn12))) / (assign7150_body8_e5457 * assign7150_body8_e5457)),)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn8, var_t8_dn10, var_t8_dn11, var_t8_dn12,)
    }
};
            var_t8 = assign7150_body8_e5460;
            var_t8_dn0 = assign7150_body8_e5460_d_n0;
            var_t8_dn2 = assign7150_body8_e5460_d_n2;
            var_t8_dn4 = assign7150_body8_e5460_d_n4;
            var_t8_dn5 = assign7150_body8_e5460_d_n5;
            var_t8_dn6 = assign7150_body8_e5460_d_n6;
            var_t8_dn8 = assign7150_body8_e5460_d_n8;
            var_t8_dn10 = assign7150_body8_e5460_d_n10;
            var_t8_dn11 = assign7150_body8_e5460_d_n11;
            var_t8_dn12 = assign7150_body8_e5460_d_n12;
            var_t8_rv = 0.0;
            let (assign7150_body9_e5475, assign7150_body9_e5475_d_n0, assign7150_body9_e5475_d_n2, assign7150_body9_e5475_d_n4, assign7150_body9_e5475_d_n5, assign7150_body9_e5475_d_n6, assign7150_body9_e5475_d_n8, assign7150_body9_e5475_d_n10, assign7150_body9_e5475_d_n11, assign7150_body9_e5475_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body9_e5468: f64 = (2.0 * var_t0);
        let assign7150_body9_e5470: f64 = (assign7150_body9_e5468 * var_c_fox);
        let assign7150_body9_e5472: f64 = (assign7150_body9_e5470 * var_c_fox);
        let assign7150_body9_e5473: f64 = (1.0 / assign7150_body9_e5472);
        (assign7150_body9_e5473, (-((((((2.0 * var_t0_dn0) * var_c_fox) + (assign7150_body9_e5468 * var_c_fox_dn0)) * var_c_fox) + (assign7150_body9_e5470 * var_c_fox_dn0)) / (assign7150_body9_e5472 * assign7150_body9_e5472))), (-((((((2.0 * var_t0_dn2) * var_c_fox) + (assign7150_body9_e5468 * var_c_fox_dn2)) * var_c_fox) + (assign7150_body9_e5470 * var_c_fox_dn2)) / (assign7150_body9_e5472 * assign7150_body9_e5472))), (-((((((2.0 * var_t0_dn4) * var_c_fox) + (assign7150_body9_e5468 * var_c_fox_dn4)) * var_c_fox) + (assign7150_body9_e5470 * var_c_fox_dn4)) / (assign7150_body9_e5472 * assign7150_body9_e5472))), (-((((((2.0 * var_t0_dn5) * var_c_fox) + (assign7150_body9_e5468 * var_c_fox_dn5)) * var_c_fox) + (assign7150_body9_e5470 * var_c_fox_dn5)) / (assign7150_body9_e5472 * assign7150_body9_e5472))), (-((((((2.0 * var_t0_dn6) * var_c_fox) + (assign7150_body9_e5468 * var_c_fox_dn6)) * var_c_fox) + (assign7150_body9_e5470 * var_c_fox_dn6)) / (assign7150_body9_e5472 * assign7150_body9_e5472))), (-((((((2.0 * var_t0_dn8) * var_c_fox) + (assign7150_body9_e5468 * var_c_fox_dn8)) * var_c_fox) + (assign7150_body9_e5470 * var_c_fox_dn8)) / (assign7150_body9_e5472 * assign7150_body9_e5472))), (-((((((2.0 * var_t0_dn10) * var_c_fox) + (assign7150_body9_e5468 * var_c_fox_dn10)) * var_c_fox) + (assign7150_body9_e5470 * var_c_fox_dn10)) / (assign7150_body9_e5472 * assign7150_body9_e5472))), (-((((((2.0 * var_t0_dn11) * var_c_fox) + (assign7150_body9_e5468 * var_c_fox_dn11)) * var_c_fox) + (assign7150_body9_e5470 * var_c_fox_dn11)) / (assign7150_body9_e5472 * assign7150_body9_e5472))), (-((((((2.0 * var_t0_dn12) * var_c_fox) + (assign7150_body9_e5468 * var_c_fox_dn12)) * var_c_fox) + (assign7150_body9_e5470 * var_c_fox_dn12)) / (assign7150_body9_e5472 * assign7150_body9_e5472))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
            var_t6 = assign7150_body9_e5475;
            var_t6_dn0 = assign7150_body9_e5475_d_n0;
            var_t6_dn2 = assign7150_body9_e5475_d_n2;
            var_t6_dn4 = assign7150_body9_e5475_d_n4;
            var_t6_dn5 = assign7150_body9_e5475_d_n5;
            var_t6_dn6 = assign7150_body9_e5475_d_n6;
            var_t6_dn8 = assign7150_body9_e5475_d_n8;
            var_t6_dn10 = assign7150_body9_e5475_d_n10;
            var_t6_dn11 = assign7150_body9_e5475_d_n11;
            var_t6_dn12 = assign7150_body9_e5475_d_n12;
            var_t6_rv = 0.0;
            let (assign7150_body10_e5486, assign7150_body10_e5486_d_n0, assign7150_body10_e5486_d_n2, assign7150_body10_e5486_d_n4, assign7150_body10_e5486_d_n5, assign7150_body10_e5486_d_n6, assign7150_body10_e5486_d_n8, assign7150_body10_e5486_d_n10, assign7150_body10_e5486_d_n11, assign7150_body10_e5486_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body10_e5483: f64 = (var_t4 - var_t5);
        let assign7150_body10_e5484: f64 = (var_t6 * assign7150_body10_e5483);
        (assign7150_body10_e5484, ((var_t6_dn0 * assign7150_body10_e5483) + (var_t6 * (var_t4_dn0 - var_t5_dn0))), ((var_t6_dn2 * assign7150_body10_e5483) + (var_t6 * (var_t4_dn2 - var_t5_dn2))), ((var_t6_dn4 * assign7150_body10_e5483) + (var_t6 * (var_t4_dn4 - var_t5_dn4))), ((var_t6_dn5 * assign7150_body10_e5483) + (var_t6 * (var_t4_dn5 - var_t5_dn5))), ((var_t6_dn6 * assign7150_body10_e5483) + (var_t6 * (var_t4_dn6 - var_t5_dn6))), ((var_t6_dn8 * assign7150_body10_e5483) + (var_t6 * (var_t4_dn8 - var_t5_dn8))), ((var_t6_dn10 * assign7150_body10_e5483) + (var_t6 * (var_t4_dn10 - var_t5_dn10))), ((var_t6_dn11 * assign7150_body10_e5483) + (var_t6 * (var_t4_dn11 - var_t5_dn11))), ((var_t6_dn12 * assign7150_body10_e5483) + (var_t6 * (var_t4_dn12 - var_t5_dn12))),)
    } else {
        (var_pf1, var_pf1_dn0, var_pf1_dn2, var_pf1_dn4, var_pf1_dn5, var_pf1_dn6, var_pf1_dn8, var_pf1_dn10, var_pf1_dn11, var_pf1_dn12,)
    }
};
            var_pf1 = assign7150_body10_e5486;
            var_pf1_dn0 = assign7150_body10_e5486_d_n0;
            var_pf1_dn2 = assign7150_body10_e5486_d_n2;
            var_pf1_dn4 = assign7150_body10_e5486_d_n4;
            var_pf1_dn5 = assign7150_body10_e5486_d_n5;
            var_pf1_dn6 = assign7150_body10_e5486_d_n6;
            var_pf1_dn8 = assign7150_body10_e5486_d_n8;
            var_pf1_dn10 = assign7150_body10_e5486_d_n10;
            var_pf1_dn11 = assign7150_body10_e5486_d_n11;
            var_pf1_dn12 = assign7150_body10_e5486_d_n12;
            var_pf1_rv = 0.0;
            let (assign7150_body11_e5497, assign7150_body11_e5497_d_n0, assign7150_body11_e5497_d_n2, assign7150_body11_e5497_d_n4, assign7150_body11_e5497_d_n5, assign7150_body11_e5497_d_n6, assign7150_body11_e5497_d_n8, assign7150_body11_e5497_d_n10, assign7150_body11_e5497_d_n11, assign7150_body11_e5497_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body11_e5494: f64 = (var_t7 - var_t8);
        let assign7150_body11_e5495: f64 = (var_t6 * assign7150_body11_e5494);
        (assign7150_body11_e5495, ((var_t6_dn0 * assign7150_body11_e5494) + (var_t6 * (var_t7_dn0 - var_t8_dn0))), ((var_t6_dn2 * assign7150_body11_e5494) + (var_t6 * (var_t7_dn2 - var_t8_dn2))), ((var_t6_dn4 * assign7150_body11_e5494) + (var_t6 * (var_t7_dn4 - var_t8_dn4))), ((var_t6_dn5 * assign7150_body11_e5494) + (var_t6 * (var_t7_dn5 - var_t8_dn5))), ((var_t6_dn6 * assign7150_body11_e5494) + (var_t6 * (var_t7_dn6 - var_t8_dn6))), ((var_t6_dn8 * assign7150_body11_e5494) + (var_t6 * (var_t7_dn8 - var_t8_dn8))), ((var_t6_dn10 * assign7150_body11_e5494) + (var_t6 * (var_t7_dn10 - var_t8_dn10))), ((var_t6_dn11 * assign7150_body11_e5494) + (var_t6 * (var_t7_dn11 - var_t8_dn11))), ((var_t6_dn12 * assign7150_body11_e5494) + (var_t6 * (var_t7_dn12 - var_t8_dn12))),)
    } else {
        (var_pf11, var_pf11_dn0, var_pf11_dn2, var_pf11_dn4, var_pf11_dn5, var_pf11_dn6, var_pf11_dn8, var_pf11_dn10, var_pf11_dn11, var_pf11_dn12,)
    }
};
            var_pf11 = assign7150_body11_e5497;
            var_pf11_dn0 = assign7150_body11_e5497_d_n0;
            var_pf11_dn2 = assign7150_body11_e5497_d_n2;
            var_pf11_dn4 = assign7150_body11_e5497_d_n4;
            var_pf11_dn5 = assign7150_body11_e5497_d_n5;
            var_pf11_dn6 = assign7150_body11_e5497_d_n6;
            var_pf11_dn8 = assign7150_body11_e5497_d_n8;
            var_pf11_dn10 = assign7150_body11_e5497_d_n10;
            var_pf11_dn11 = assign7150_body11_e5497_d_n11;
            var_pf11_dn12 = assign7150_body11_e5497_d_n12;
            var_pf11_rv = 0.0;
            let (assign7150_body12_e5507, assign7150_body12_e5507_d_n0, assign7150_body12_e5507_d_n2, assign7150_body12_e5507_d_n4, assign7150_body12_e5507_d_n5, assign7150_body12_e5507_d_n6, assign7150_body12_e5507_d_n8, assign7150_body12_e5507_d_n10, assign7150_body12_e5507_d_n11, assign7150_body12_e5507_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body12_e5503: f64 = (-var_pf1);
        let assign7150_body12_e5505: f64 = (assign7150_body12_e5503 / var_pf11);
        (assign7150_body12_e5505, ((((-var_pf1_dn0) * var_pf11) - (assign7150_body12_e5503 * var_pf11_dn0)) / (var_pf11 * var_pf11)), ((((-var_pf1_dn2) * var_pf11) - (assign7150_body12_e5503 * var_pf11_dn2)) / (var_pf11 * var_pf11)), ((((-var_pf1_dn4) * var_pf11) - (assign7150_body12_e5503 * var_pf11_dn4)) / (var_pf11 * var_pf11)), ((((-var_pf1_dn5) * var_pf11) - (assign7150_body12_e5503 * var_pf11_dn5)) / (var_pf11 * var_pf11)), ((((-var_pf1_dn6) * var_pf11) - (assign7150_body12_e5503 * var_pf11_dn6)) / (var_pf11 * var_pf11)), ((((-var_pf1_dn8) * var_pf11) - (assign7150_body12_e5503 * var_pf11_dn8)) / (var_pf11 * var_pf11)), ((((-var_pf1_dn10) * var_pf11) - (assign7150_body12_e5503 * var_pf11_dn10)) / (var_pf11 * var_pf11)), ((((-var_pf1_dn11) * var_pf11) - (assign7150_body12_e5503 * var_pf11_dn11)) / (var_pf11 * var_pf11)), ((((-var_pf1_dn12) * var_pf11) - (assign7150_body12_e5503 * var_pf11_dn12)) / (var_pf11 * var_pf11)),)
    } else {
        (var_dpsb, var_dpsb_dn0, var_dpsb_dn2, var_dpsb_dn4, var_dpsb_dn5, var_dpsb_dn6, var_dpsb_dn8, var_dpsb_dn10, var_dpsb_dn11, var_dpsb_dn12,)
    }
};
            var_dpsb = assign7150_body12_e5507;
            var_dpsb_dn0 = assign7150_body12_e5507_d_n0;
            var_dpsb_dn2 = assign7150_body12_e5507_d_n2;
            var_dpsb_dn4 = assign7150_body12_e5507_d_n4;
            var_dpsb_dn5 = assign7150_body12_e5507_d_n5;
            var_dpsb_dn6 = assign7150_body12_e5507_d_n6;
            var_dpsb_dn8 = assign7150_body12_e5507_d_n8;
            var_dpsb_dn10 = assign7150_body12_e5507_d_n10;
            var_dpsb_dn11 = assign7150_body12_e5507_d_n11;
            var_dpsb_dn12 = assign7150_body12_e5507_d_n12;
            var_dpsb_rv = 0.0;
            let assign7150_body13_e5509: f64 = (var_dpsb).abs();
            let assign7150_body13_e5511: f64 = if assign7150_body13_e5509 < 1e-12 { 1.0 } else { 0.0 };
            var_guard76 = assign7150_body13_e5511;
            var_guard76_rv = 0.0;
            let (assign7150_body14_e5520,) = {
    if (((var_guard74 == 0.0) && (var_guard75 != 0.0)) && (var_guard76 != 0.0)) {
        (var_lp_s0_max,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign7150_body14_e5520;
            var_lp_s0_rv = 0.0;
            let assign7150_body15_e5523: f64 = if var_dpsb > 0.1 { 1.0 } else { 0.0 };
            var_guard77 = assign7150_body15_e5523;
            var_guard77_rv = 0.0;
            let (assign7150_body16_e5535, assign7150_body16_e5535_d_n0, assign7150_body16_e5535_d_n2, assign7150_body16_e5535_d_n4, assign7150_body16_e5535_d_n5, assign7150_body16_e5535_d_n6, assign7150_body16_e5535_d_n8, assign7150_body16_e5535_d_n10, assign7150_body16_e5535_d_n11, assign7150_body16_e5535_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard75 != 0.0)) && (var_guard76 == 0.0)) && (var_guard77 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dpsb, var_dpsb_dn0, var_dpsb_dn2, var_dpsb_dn4, var_dpsb_dn5, var_dpsb_dn6, var_dpsb_dn8, var_dpsb_dn10, var_dpsb_dn11, var_dpsb_dn12,)
    }
};
            var_dpsb = assign7150_body16_e5535;
            var_dpsb_dn0 = assign7150_body16_e5535_d_n0;
            var_dpsb_dn2 = assign7150_body16_e5535_d_n2;
            var_dpsb_dn4 = assign7150_body16_e5535_d_n4;
            var_dpsb_dn5 = assign7150_body16_e5535_d_n5;
            var_dpsb_dn6 = assign7150_body16_e5535_d_n6;
            var_dpsb_dn8 = assign7150_body16_e5535_d_n8;
            var_dpsb_dn10 = assign7150_body16_e5535_d_n10;
            var_dpsb_dn11 = assign7150_body16_e5535_d_n11;
            var_dpsb_dn12 = assign7150_body16_e5535_d_n12;
            var_dpsb_rv = 0.0;
            let assign7150_body17_e5538: f64 = (-0.1);
            let assign7150_body17_e5539: f64 = if var_dpsb < assign7150_body17_e5538 { 1.0 } else { 0.0 };
            var_guard78 = assign7150_body17_e5539;
            var_guard78_rv = 0.0;
            let (assign7150_body18_e5555, assign7150_body18_e5555_d_n0, assign7150_body18_e5555_d_n2, assign7150_body18_e5555_d_n4, assign7150_body18_e5555_d_n5, assign7150_body18_e5555_d_n6, assign7150_body18_e5555_d_n8, assign7150_body18_e5555_d_n10, assign7150_body18_e5555_d_n11, assign7150_body18_e5555_d_n12,) = {
    if (((((var_guard74 == 0.0) && (var_guard75 != 0.0)) && (var_guard76 == 0.0)) && (var_guard77 == 0.0)) && (var_guard78 != 0.0)) {
        let assign7150_body18_e5553: f64 = (-0.1);
        (assign7150_body18_e5553, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dpsb, var_dpsb_dn0, var_dpsb_dn2, var_dpsb_dn4, var_dpsb_dn5, var_dpsb_dn6, var_dpsb_dn8, var_dpsb_dn10, var_dpsb_dn11, var_dpsb_dn12,)
    }
};
            var_dpsb = assign7150_body18_e5555;
            var_dpsb_dn0 = assign7150_body18_e5555_d_n0;
            var_dpsb_dn2 = assign7150_body18_e5555_d_n2;
            var_dpsb_dn4 = assign7150_body18_e5555_d_n4;
            var_dpsb_dn5 = assign7150_body18_e5555_d_n5;
            var_dpsb_dn6 = assign7150_body18_e5555_d_n6;
            var_dpsb_dn8 = assign7150_body18_e5555_d_n8;
            var_dpsb_dn10 = assign7150_body18_e5555_d_n10;
            var_dpsb_dn11 = assign7150_body18_e5555_d_n11;
            var_dpsb_dn12 = assign7150_body18_e5555_d_n12;
            var_dpsb_rv = 0.0;
            let (assign7150_body19_e5564, assign7150_body19_e5564_d_n0, assign7150_body19_e5564_d_n2, assign7150_body19_e5564_d_n4, assign7150_body19_e5564_d_n5, assign7150_body19_e5564_d_n6, assign7150_body19_e5564_d_n8, assign7150_body19_e5564_d_n10, assign7150_body19_e5564_d_n11, assign7150_body19_e5564_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body19_e5562: f64 = (var_vgs_shift + var_dpsb);
        (assign7150_body19_e5562, (var_vgs_shift_dn0 + var_dpsb_dn0), (var_vgs_shift_dn2 + var_dpsb_dn2), (var_vgs_shift_dn4 + var_dpsb_dn4), (var_vgs_shift_dn5 + var_dpsb_dn5), (var_vgs_shift_dn6 + var_dpsb_dn6), (var_vgs_shift_dn8 + var_dpsb_dn8), (var_vgs_shift_dn10 + var_dpsb_dn10), (var_vgs_shift_dn11 + var_dpsb_dn11), (var_vgs_shift_dn12 + var_dpsb_dn12),)
    } else {
        (var_vgs_shift, var_vgs_shift_dn0, var_vgs_shift_dn2, var_vgs_shift_dn4, var_vgs_shift_dn5, var_vgs_shift_dn6, var_vgs_shift_dn8, var_vgs_shift_dn10, var_vgs_shift_dn11, var_vgs_shift_dn12,)
    }
};
            var_vgs_shift = assign7150_body19_e5564;
            var_vgs_shift_dn0 = assign7150_body19_e5564_d_n0;
            var_vgs_shift_dn2 = assign7150_body19_e5564_d_n2;
            var_vgs_shift_dn4 = assign7150_body19_e5564_d_n4;
            var_vgs_shift_dn5 = assign7150_body19_e5564_d_n5;
            var_vgs_shift_dn6 = assign7150_body19_e5564_d_n6;
            var_vgs_shift_dn8 = assign7150_body19_e5564_d_n8;
            var_vgs_shift_dn10 = assign7150_body19_e5564_d_n10;
            var_vgs_shift_dn11 = assign7150_body19_e5564_d_n11;
            var_vgs_shift_dn12 = assign7150_body19_e5564_d_n12;
            var_vgs_shift_rv = 0.0;
            let (assign7150_body20_e5573,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body20_e5571: f64 = (var_lp_s0 + 1.0);
        (assign7150_body20_e5571,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign7150_body20_e5573;
            var_lp_s0_rv = 0.0;
        }

        let assign7160_e5577: f64 = (var_vgs_fb + var_vgs_shift);
        let assign7160_e5578: f64 = if var_vgs < assign7160_e5577 { 1.0 } else { 0.0 };
        var_guard79 = assign7160_e5578;
        var_guard79_rv = 0.0;

        let (assign7170_e5585,) = {
    if ((var_guard74 == 0.0) && (var_guard79 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_depmode,)
    }
};
        var_flg_depmode = assign7170_e5585;
        var_flg_depmode_rv = 0.0;

        let (assign7180_e5593,) = {
    if ((var_guard74 == 0.0) && (var_guard79 != 0.0)) {
        let assign7180_e5591: f64 = (-1.0);
        (assign7180_e5591,)
    } else {
        (var_flg_zone,)
    }
};
        var_flg_zone = assign7180_e5593;
        var_flg_zone_rv = 0.0;

        let (assign7190_e5600, assign7190_e5600_d_n0, assign7190_e5600_d_n2, assign7190_e5600_d_n4, assign7190_e5600_d_n5, assign7190_e5600_d_n6, assign7190_e5600_d_n8, assign7190_e5600_d_n10, assign7190_e5600_d_n11, assign7190_e5600_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard79 != 0.0)) {
        (var_phi_b_dep0, var_phi_b_dep0_dn0, var_phi_b_dep0_dn2, var_phi_b_dep0_dn4, var_phi_b_dep0_dn5, var_phi_b_dep0_dn6, var_phi_b_dep0_dn8, var_phi_b_dep0_dn10, var_phi_b_dep0_dn11, var_phi_b_dep0_dn12,)
    } else {
        (var_phi_b_dep, var_phi_b_dep_dn0, var_phi_b_dep_dn2, var_phi_b_dep_dn4, var_phi_b_dep_dn5, var_phi_b_dep_dn6, var_phi_b_dep_dn8, var_phi_b_dep_dn10, var_phi_b_dep_dn11, var_phi_b_dep_dn12,)
    }
};
        var_phi_b_dep = assign7190_e5600;
        var_phi_b_dep_dn0 = assign7190_e5600_d_n0;
        var_phi_b_dep_dn2 = assign7190_e5600_d_n2;
        var_phi_b_dep_dn4 = assign7190_e5600_d_n4;
        var_phi_b_dep_dn5 = assign7190_e5600_d_n5;
        var_phi_b_dep_dn6 = assign7190_e5600_d_n6;
        var_phi_b_dep_dn8 = assign7190_e5600_d_n8;
        var_phi_b_dep_dn10 = assign7190_e5600_d_n10;
        var_phi_b_dep_dn11 = assign7190_e5600_d_n11;
        var_phi_b_dep_dn12 = assign7190_e5600_d_n12;
        var_phi_b_dep_rv = 0.0;

        let (assign7200_e5616, assign7200_e5616_d_n0, assign7200_e5616_d_n2, assign7200_e5616_d_n4, assign7200_e5616_d_n5, assign7200_e5616_d_n6, assign7200_e5616_d_n8, assign7200_e5616_d_n10, assign7200_e5616_d_n11, assign7200_e5616_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard79 != 0.0)) {
        let assign7200_e5607: f64 = (2.0 * 1.034943e-10);
        let assign7200_e5609: f64 = (assign7200_e5607 / 1.6021918e-19);
        let assign7200_e5611: f64 = (assign7200_e5609 * var_phi_b_dep);
        let assign7200_e5613: f64 = (assign7200_e5611 / var_uc_nsubs);
        let assign7200_e5614: f64 = (assign7200_e5613).sqrt();
        (assign7200_e5614, (((((assign7200_e5609 * var_phi_b_dep_dn0) * var_uc_nsubs) - (assign7200_e5611 * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7200_e5614)), (((((assign7200_e5609 * var_phi_b_dep_dn2) * var_uc_nsubs) - (assign7200_e5611 * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7200_e5614)), (((((assign7200_e5609 * var_phi_b_dep_dn4) * var_uc_nsubs) - (assign7200_e5611 * var_uc_nsubs_dn4)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7200_e5614)), (((((assign7200_e5609 * var_phi_b_dep_dn5) * var_uc_nsubs) - (assign7200_e5611 * var_uc_nsubs_dn5)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7200_e5614)), (((((assign7200_e5609 * var_phi_b_dep_dn6) * var_uc_nsubs) - (assign7200_e5611 * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7200_e5614)), (((((assign7200_e5609 * var_phi_b_dep_dn8) * var_uc_nsubs) - (assign7200_e5611 * var_uc_nsubs_dn8)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7200_e5614)), (((((assign7200_e5609 * var_phi_b_dep_dn10) * var_uc_nsubs) - (assign7200_e5611 * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7200_e5614)), (((((assign7200_e5609 * var_phi_b_dep_dn11) * var_uc_nsubs) - (assign7200_e5611 * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7200_e5614)), (((((assign7200_e5609 * var_phi_b_dep_dn12) * var_uc_nsubs) - (assign7200_e5611 * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7200_e5614)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign7200_e5616;
        var_t1_dn0 = assign7200_e5616_d_n0;
        var_t1_dn2 = assign7200_e5616_d_n2;
        var_t1_dn4 = assign7200_e5616_d_n4;
        var_t1_dn5 = assign7200_e5616_d_n5;
        var_t1_dn6 = assign7200_e5616_d_n6;
        var_t1_dn8 = assign7200_e5616_d_n8;
        var_t1_dn10 = assign7200_e5616_d_n10;
        var_t1_dn11 = assign7200_e5616_d_n11;
        var_t1_dn12 = assign7200_e5616_d_n12;
        var_t1_rv = 0.0;

        let (assign7210_e5629, assign7210_e5629_d_n0, assign7210_e5629_d_n2, assign7210_e5629_d_n4, assign7210_e5629_d_n5, assign7210_e5629_d_n6, assign7210_e5629_d_n8, assign7210_e5629_d_n10, assign7210_e5629_d_n11, assign7210_e5629_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard79 != 0.0)) {
        let assign7210_e5623: f64 = (var_cnst0bulk * var_cnst0bulk);
        let assign7210_e5625: f64 = (assign7210_e5623 * var_c_box_fd_inv);
        let assign7210_e5627: f64 = (assign7210_e5625 * var_c_box_fd_inv);
        (assign7210_e5627, ((((var_cnst0bulk_dn0 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn0)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn2 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn2)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn4 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn4)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn5 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn5)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn6 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn6)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn8 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn8)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn10 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn10)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn11 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn11)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn12 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn12)) * var_c_box_fd_inv) * var_c_box_fd_inv),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
        var_t0 = assign7210_e5629;
        var_t0_dn0 = assign7210_e5629_d_n0;
        var_t0_dn2 = assign7210_e5629_d_n2;
        var_t0_dn4 = assign7210_e5629_d_n4;
        var_t0_dn5 = assign7210_e5629_d_n5;
        var_t0_dn6 = assign7210_e5629_d_n6;
        var_t0_dn8 = assign7210_e5629_d_n8;
        var_t0_dn10 = assign7210_e5629_d_n10;
        var_t0_dn11 = assign7210_e5629_d_n11;
        var_t0_dn12 = assign7210_e5629_d_n12;
        var_t0_rv = 0.0;

        let assign7220_e5632: f64 = (var_wdsoi + var_t1);
        let assign7220_e5634: f64 = if assign7220_e5632 < p.p227 { 1.0 } else { 0.0 };
        var_guard80 = assign7220_e5634;
        var_guard80_rv = 0.0;

        let (assign7230_e5648, assign7230_e5648_d_n0, assign7230_e5648_d_n2, assign7230_e5648_d_n4, assign7230_e5648_d_n5, assign7230_e5648_d_n6, assign7230_e5648_d_n8, assign7230_e5648_d_n10, assign7230_e5648_d_n11, assign7230_e5648_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) {
        let assign7230_e5642: f64 = (-var_vbsbiz);
        let assign7230_e5645: f64 = (10.0 * 2.220446049250313e-16);
        let assign7230_e5646: f64 = (assign7230_e5642 + assign7230_e5645);
        (assign7230_e5646, (-var_vbsbiz_dn0), (-var_vbsbiz_dn2), (-var_vbsbiz_dn4), (-var_vbsbiz_dn5), (-var_vbsbiz_dn6), (-var_vbsbiz_dn8), (-var_vbsbiz_dn10), (-var_vbsbiz_dn11), (-var_vbsbiz_dn12),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign7230_e5648;
        var_t1_dn0 = assign7230_e5648_d_n0;
        var_t1_dn2 = assign7230_e5648_d_n2;
        var_t1_dn4 = assign7230_e5648_d_n4;
        var_t1_dn5 = assign7230_e5648_d_n5;
        var_t1_dn6 = assign7230_e5648_d_n6;
        var_t1_dn8 = assign7230_e5648_d_n8;
        var_t1_dn10 = assign7230_e5648_d_n10;
        var_t1_dn11 = assign7230_e5648_d_n11;
        var_t1_dn12 = assign7230_e5648_d_n12;
        var_t1_rv = 0.0;

        *var_dpsb_slot = var_dpsb;
        *var_dpsb_dn0_slot = var_dpsb_dn0;
        *var_dpsb_dn10_slot = var_dpsb_dn10;
        *var_dpsb_dn11_slot = var_dpsb_dn11;
        *var_dpsb_dn12_slot = var_dpsb_dn12;
        *var_dpsb_dn2_slot = var_dpsb_dn2;
        *var_dpsb_dn4_slot = var_dpsb_dn4;
        *var_dpsb_dn5_slot = var_dpsb_dn5;
        *var_dpsb_dn6_slot = var_dpsb_dn6;
        *var_dpsb_dn8_slot = var_dpsb_dn8;
        *var_dpsb_rv_slot = var_dpsb_rv;
        *var_flg_depmode_slot = var_flg_depmode;
        *var_flg_depmode_rv_slot = var_flg_depmode_rv;
        *var_flg_zone_slot = var_flg_zone;
        *var_flg_zone_rv_slot = var_flg_zone_rv;
        *var_guard76_slot = var_guard76;
        *var_guard76_rv_slot = var_guard76_rv;
        *var_guard77_slot = var_guard77;
        *var_guard77_rv_slot = var_guard77_rv;
        *var_guard78_slot = var_guard78;
        *var_guard78_rv_slot = var_guard78_rv;
        *var_guard79_slot = var_guard79;
        *var_guard79_rv_slot = var_guard79_rv;
        *var_guard80_slot = var_guard80;
        *var_guard80_rv_slot = var_guard80_rv;
        *var_lp_s0_slot = var_lp_s0;
        *var_lp_s0_rv_slot = var_lp_s0_rv;
        *var_pf1_slot = var_pf1;
        *var_pf11_slot = var_pf11;
        *var_pf11_dn0_slot = var_pf11_dn0;
        *var_pf11_dn10_slot = var_pf11_dn10;
        *var_pf11_dn11_slot = var_pf11_dn11;
        *var_pf11_dn12_slot = var_pf11_dn12;
        *var_pf11_dn2_slot = var_pf11_dn2;
        *var_pf11_dn4_slot = var_pf11_dn4;
        *var_pf11_dn5_slot = var_pf11_dn5;
        *var_pf11_dn6_slot = var_pf11_dn6;
        *var_pf11_dn8_slot = var_pf11_dn8;
        *var_pf11_rv_slot = var_pf11_rv;
        *var_pf1_dn0_slot = var_pf1_dn0;
        *var_pf1_dn10_slot = var_pf1_dn10;
        *var_pf1_dn11_slot = var_pf1_dn11;
        *var_pf1_dn12_slot = var_pf1_dn12;
        *var_pf1_dn2_slot = var_pf1_dn2;
        *var_pf1_dn4_slot = var_pf1_dn4;
        *var_pf1_dn5_slot = var_pf1_dn5;
        *var_pf1_dn6_slot = var_pf1_dn6;
        *var_pf1_dn8_slot = var_pf1_dn8;
        *var_pf1_rv_slot = var_pf1_rv;
        *var_phi_b_dep_slot = var_phi_b_dep;
        *var_phi_b_dep_dn0_slot = var_phi_b_dep_dn0;
        *var_phi_b_dep_dn10_slot = var_phi_b_dep_dn10;
        *var_phi_b_dep_dn11_slot = var_phi_b_dep_dn11;
        *var_phi_b_dep_dn12_slot = var_phi_b_dep_dn12;
        *var_phi_b_dep_dn2_slot = var_phi_b_dep_dn2;
        *var_phi_b_dep_dn4_slot = var_phi_b_dep_dn4;
        *var_phi_b_dep_dn5_slot = var_phi_b_dep_dn5;
        *var_phi_b_dep_dn6_slot = var_phi_b_dep_dn6;
        *var_phi_b_dep_dn8_slot = var_phi_b_dep_dn8;
        *var_phi_b_dep_rv_slot = var_phi_b_dep_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn12_slot = var_t6_dn12;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_rv_slot = var_t6_rv;
        *var_t7_slot = var_t7;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn12_slot = var_t7_dn12;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_rv_slot = var_t7_rv;
        *var_t8_slot = var_t8;
        *var_t8_dn0_slot = var_t8_dn0;
        *var_t8_dn10_slot = var_t8_dn10;
        *var_t8_dn11_slot = var_t8_dn11;
        *var_t8_dn12_slot = var_t8_dn12;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_rv_slot = var_t8_rv;
        *var_vgs_shift_slot = var_vgs_shift;
        *var_vgs_shift_dn0_slot = var_vgs_shift_dn0;
        *var_vgs_shift_dn10_slot = var_vgs_shift_dn10;
        *var_vgs_shift_dn11_slot = var_vgs_shift_dn11;
        *var_vgs_shift_dn12_slot = var_vgs_shift_dn12;
        *var_vgs_shift_dn2_slot = var_vgs_shift_dn2;
        *var_vgs_shift_dn4_slot = var_vgs_shift_dn4;
        *var_vgs_shift_dn5_slot = var_vgs_shift_dn5;
        *var_vgs_shift_dn6_slot = var_vgs_shift_dn6;
        *var_vgs_shift_dn8_slot = var_vgs_shift_dn8;
        *var_vgs_shift_rv_slot = var_vgs_shift_rv;
    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn4: f64,
        var_cnst1bulk: f64,
        var_cnst1bulk_dn0: f64,
        var_cnst1bulk_dn10: f64,
        var_cnst1bulk_dn11: f64,
        var_cnst1bulk_dn12: f64,
        var_cnst1bulk_dn2: f64,
        var_cnst1bulk_dn4: f64,
        var_cnst1bulk_dn5: f64,
        var_cnst1bulk_dn6: f64,
        var_cnst1bulk_dn8: f64,
        var_guard74: f64,
        var_guard79: f64,
        var_guard80: f64,
        var_pb2_bulk: f64,
        var_phi_s0_soi: f64,
        var_phi_s0_soi_dn0: f64,
        var_phi_s0_soi_dn10: f64,
        var_phi_s0_soi_dn11: f64,
        var_phi_s0_soi_dn12: f64,
        var_phi_s0_soi_dn2: f64,
        var_phi_s0_soi_dn4: f64,
        var_phi_s0_soi_dn5: f64,
        var_phi_s0_soi_dn6: f64,
        var_phi_s0_soi_dn8: f64,
        var_q_fd_soi: f64,
        var_q_fd_soi_dn0: f64,
        var_q_fd_soi_dn10: f64,
        var_q_fd_soi_dn11: f64,
        var_q_fd_soi_dn12: f64,
        var_q_fd_soi_dn2: f64,
        var_q_fd_soi_dn4: f64,
        var_q_fd_soi_dn5: f64,
        var_q_fd_soi_dn6: f64,
        var_q_fd_soi_dn8: f64,
        var_t0: f64,
        var_t0_dn0: f64,
        var_t0_dn10: f64,
        var_t0_dn11: f64,
        var_t0_dn12: f64,
        var_t0_dn2: f64,
        var_t0_dn4: f64,
        var_t0_dn5: f64,
        var_t0_dn6: f64,
        var_t0_dn8: f64,
        var_vbsbiz: f64,
        var_vbsbiz_dn0: f64,
        var_vbsbiz_dn10: f64,
        var_vbsbiz_dn11: f64,
        var_vbsbiz_dn12: f64,
        var_vbsbiz_dn2: f64,
        var_vbsbiz_dn4: f64,
        var_vbsbiz_dn5: f64,
        var_vbsbiz_dn6: f64,
        var_vbsbiz_dn8: f64,
        var_guard81_slot: &mut f64,
        var_guard81_rv_slot: &mut f64,
        var_guard82_slot: &mut f64,
        var_guard82_rv_slot: &mut f64,
        var_phi_s0_bulk_slot: &mut f64,
        var_phi_s0_bulk_dn0_slot: &mut f64,
        var_phi_s0_bulk_dn10_slot: &mut f64,
        var_phi_s0_bulk_dn11_slot: &mut f64,
        var_phi_s0_bulk_dn12_slot: &mut f64,
        var_phi_s0_bulk_dn2_slot: &mut f64,
        var_phi_s0_bulk_dn4_slot: &mut f64,
        var_phi_s0_bulk_dn5_slot: &mut f64,
        var_phi_s0_bulk_dn6_slot: &mut f64,
        var_phi_s0_bulk_dn8_slot: &mut f64,
        var_phi_s0_bulk_rv_slot: &mut f64,
        var_psb_inia_slot: &mut f64,
        var_psb_inia_dn0_slot: &mut f64,
        var_psb_inia_dn10_slot: &mut f64,
        var_psb_inia_dn11_slot: &mut f64,
        var_psb_inia_dn12_slot: &mut f64,
        var_psb_inia_dn2_slot: &mut f64,
        var_psb_inia_dn4_slot: &mut f64,
        var_psb_inia_dn5_slot: &mut f64,
        var_psb_inia_dn6_slot: &mut f64,
        var_psb_inia_dn8_slot: &mut f64,
        var_psb_inia_rv_slot: &mut f64,
        var_psb_inib_slot: &mut f64,
        var_psb_inib_dn0_slot: &mut f64,
        var_psb_inib_dn10_slot: &mut f64,
        var_psb_inib_dn11_slot: &mut f64,
        var_psb_inib_dn12_slot: &mut f64,
        var_psb_inib_dn2_slot: &mut f64,
        var_psb_inib_dn4_slot: &mut f64,
        var_psb_inib_dn5_slot: &mut f64,
        var_psb_inib_dn6_slot: &mut f64,
        var_psb_inib_dn8_slot: &mut f64,
        var_psb_inib_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_guard81: f64 = *var_guard81_slot;
        let mut var_guard81_rv: f64 = *var_guard81_rv_slot;
        let mut var_guard82: f64 = *var_guard82_slot;
        let mut var_guard82_rv: f64 = *var_guard82_rv_slot;
        let mut var_phi_s0_bulk: f64 = *var_phi_s0_bulk_slot;
        let mut var_phi_s0_bulk_dn0: f64 = *var_phi_s0_bulk_dn0_slot;
        let mut var_phi_s0_bulk_dn10: f64 = *var_phi_s0_bulk_dn10_slot;
        let mut var_phi_s0_bulk_dn11: f64 = *var_phi_s0_bulk_dn11_slot;
        let mut var_phi_s0_bulk_dn12: f64 = *var_phi_s0_bulk_dn12_slot;
        let mut var_phi_s0_bulk_dn2: f64 = *var_phi_s0_bulk_dn2_slot;
        let mut var_phi_s0_bulk_dn4: f64 = *var_phi_s0_bulk_dn4_slot;
        let mut var_phi_s0_bulk_dn5: f64 = *var_phi_s0_bulk_dn5_slot;
        let mut var_phi_s0_bulk_dn6: f64 = *var_phi_s0_bulk_dn6_slot;
        let mut var_phi_s0_bulk_dn8: f64 = *var_phi_s0_bulk_dn8_slot;
        let mut var_phi_s0_bulk_rv: f64 = *var_phi_s0_bulk_rv_slot;
        let mut var_psb_inia: f64 = *var_psb_inia_slot;
        let mut var_psb_inia_dn0: f64 = *var_psb_inia_dn0_slot;
        let mut var_psb_inia_dn10: f64 = *var_psb_inia_dn10_slot;
        let mut var_psb_inia_dn11: f64 = *var_psb_inia_dn11_slot;
        let mut var_psb_inia_dn12: f64 = *var_psb_inia_dn12_slot;
        let mut var_psb_inia_dn2: f64 = *var_psb_inia_dn2_slot;
        let mut var_psb_inia_dn4: f64 = *var_psb_inia_dn4_slot;
        let mut var_psb_inia_dn5: f64 = *var_psb_inia_dn5_slot;
        let mut var_psb_inia_dn6: f64 = *var_psb_inia_dn6_slot;
        let mut var_psb_inia_dn8: f64 = *var_psb_inia_dn8_slot;
        let mut var_psb_inia_rv: f64 = *var_psb_inia_rv_slot;
        let mut var_psb_inib: f64 = *var_psb_inib_slot;
        let mut var_psb_inib_dn0: f64 = *var_psb_inib_dn0_slot;
        let mut var_psb_inib_dn10: f64 = *var_psb_inib_dn10_slot;
        let mut var_psb_inib_dn11: f64 = *var_psb_inib_dn11_slot;
        let mut var_psb_inib_dn12: f64 = *var_psb_inib_dn12_slot;
        let mut var_psb_inib_dn2: f64 = *var_psb_inib_dn2_slot;
        let mut var_psb_inib_dn4: f64 = *var_psb_inib_dn4_slot;
        let mut var_psb_inib_dn5: f64 = *var_psb_inib_dn5_slot;
        let mut var_psb_inib_dn6: f64 = *var_psb_inib_dn6_slot;
        let mut var_psb_inib_dn8: f64 = *var_psb_inib_dn8_slot;
        let mut var_psb_inib_rv: f64 = *var_psb_inib_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign7240_e5679, assign7240_e5679_d_n0, assign7240_e5679_d_n2, assign7240_e5679_d_n4, assign7240_e5679_d_n5, assign7240_e5679_d_n6, assign7240_e5679_d_n8, assign7240_e5679_d_n10, assign7240_e5679_d_n11, assign7240_e5679_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) {
        let assign7240_e5657: f64 = (2.0 * var_t1);
        let assign7240_e5660: f64 = (var_t0 * var_beta);
        let assign7240_e5661: f64 = (assign7240_e5657 + assign7240_e5660);
        let assign7240_e5664: f64 = (2.0 * var_t1);
        let assign7240_e5667: f64 = (var_t0 * var_beta);
        let assign7240_e5668: f64 = (assign7240_e5664 + assign7240_e5667);
        let assign7240_e5669: f64 = (assign7240_e5661 * assign7240_e5668);
        let assign7240_e5673: f64 = (var_t1 * var_t1);
        let assign7240_e5675: f64 = (assign7240_e5673 + var_t0);
        let assign7240_e5676: f64 = (4.0 * assign7240_e5675);
        let assign7240_e5677: f64 = (assign7240_e5669 - assign7240_e5676);
        (assign7240_e5677, (((((2.0 * var_t1_dn0) + (var_t0_dn0 * var_beta)) * assign7240_e5668) + (assign7240_e5661 * ((2.0 * var_t1_dn0) + (var_t0_dn0 * var_beta)))) - (4.0 * (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) + var_t0_dn0))), (((((2.0 * var_t1_dn2) + (var_t0_dn2 * var_beta)) * assign7240_e5668) + (assign7240_e5661 * ((2.0 * var_t1_dn2) + (var_t0_dn2 * var_beta)))) - (4.0 * (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) + var_t0_dn2))), (((((2.0 * var_t1_dn4) + ((var_t0_dn4 * var_beta) + (var_t0 * var_beta_dn4))) * assign7240_e5668) + (assign7240_e5661 * ((2.0 * var_t1_dn4) + ((var_t0_dn4 * var_beta) + (var_t0 * var_beta_dn4))))) - (4.0 * (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) + var_t0_dn4))), (((((2.0 * var_t1_dn5) + (var_t0_dn5 * var_beta)) * assign7240_e5668) + (assign7240_e5661 * ((2.0 * var_t1_dn5) + (var_t0_dn5 * var_beta)))) - (4.0 * (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) + var_t0_dn5))), (((((2.0 * var_t1_dn6) + (var_t0_dn6 * var_beta)) * assign7240_e5668) + (assign7240_e5661 * ((2.0 * var_t1_dn6) + (var_t0_dn6 * var_beta)))) - (4.0 * (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) + var_t0_dn6))), (((((2.0 * var_t1_dn8) + (var_t0_dn8 * var_beta)) * assign7240_e5668) + (assign7240_e5661 * ((2.0 * var_t1_dn8) + (var_t0_dn8 * var_beta)))) - (4.0 * (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) + var_t0_dn8))), (((((2.0 * var_t1_dn10) + (var_t0_dn10 * var_beta)) * assign7240_e5668) + (assign7240_e5661 * ((2.0 * var_t1_dn10) + (var_t0_dn10 * var_beta)))) - (4.0 * (((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) + var_t0_dn10))), (((((2.0 * var_t1_dn11) + (var_t0_dn11 * var_beta)) * assign7240_e5668) + (assign7240_e5661 * ((2.0 * var_t1_dn11) + (var_t0_dn11 * var_beta)))) - (4.0 * (((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) + var_t0_dn11))), (((((2.0 * var_t1_dn12) + (var_t0_dn12 * var_beta)) * assign7240_e5668) + (assign7240_e5661 * ((2.0 * var_t1_dn12) + (var_t0_dn12 * var_beta)))) - (4.0 * (((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)) + var_t0_dn12))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign7240_e5679;
        var_t2_dn0 = assign7240_e5679_d_n0;
        var_t2_dn2 = assign7240_e5679_d_n2;
        var_t2_dn4 = assign7240_e5679_d_n4;
        var_t2_dn5 = assign7240_e5679_d_n5;
        var_t2_dn6 = assign7240_e5679_d_n6;
        var_t2_dn8 = assign7240_e5679_d_n8;
        var_t2_dn10 = assign7240_e5679_d_n10;
        var_t2_dn11 = assign7240_e5679_d_n11;
        var_t2_dn12 = assign7240_e5679_d_n12;
        var_t2_rv = 0.0;

        let (assign7250_e5697, assign7250_e5697_d_n0, assign7250_e5697_d_n2, assign7250_e5697_d_n4, assign7250_e5697_d_n5, assign7250_e5697_d_n6, assign7250_e5697_d_n8, assign7250_e5697_d_n10, assign7250_e5697_d_n11, assign7250_e5697_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) {
        let assign7250_e5689: f64 = (10.0 * 2.220446049250313e-16);
        let (assign7250_e5695, assign7250_e5695_d_n0, assign7250_e5695_d_n2, assign7250_e5695_d_n4, assign7250_e5695_d_n5, assign7250_e5695_d_n6, assign7250_e5695_d_n8, assign7250_e5695_d_n10, assign7250_e5695_d_n11, assign7250_e5695_d_n12,) = {
            if (var_t2 >= assign7250_e5689) {
                (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
            } else {
                let assign7250_e5694: f64 = (10.0 * 2.220446049250313e-16);
                (assign7250_e5694, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign7250_e5695, assign7250_e5695_d_n0, assign7250_e5695_d_n2, assign7250_e5695_d_n4, assign7250_e5695_d_n5, assign7250_e5695_d_n6, assign7250_e5695_d_n8, assign7250_e5695_d_n10, assign7250_e5695_d_n11, assign7250_e5695_d_n12,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign7250_e5697;
        var_t2_dn0 = assign7250_e5697_d_n0;
        var_t2_dn2 = assign7250_e5697_d_n2;
        var_t2_dn4 = assign7250_e5697_d_n4;
        var_t2_dn5 = assign7250_e5697_d_n5;
        var_t2_dn6 = assign7250_e5697_d_n6;
        var_t2_dn8 = assign7250_e5697_d_n8;
        var_t2_dn10 = assign7250_e5697_d_n10;
        var_t2_dn11 = assign7250_e5697_d_n11;
        var_t2_dn12 = assign7250_e5697_d_n12;
        var_t2_rv = 0.0;

        let (assign7260_e5707, assign7260_e5707_d_n0, assign7260_e5707_d_n2, assign7260_e5707_d_n4, assign7260_e5707_d_n5, assign7260_e5707_d_n6, assign7260_e5707_d_n8, assign7260_e5707_d_n10, assign7260_e5707_d_n11, assign7260_e5707_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) {
        let assign7260_e5705: f64 = (var_t2).sqrt();
        (assign7260_e5705, (var_t2_dn0 / (2.0 * assign7260_e5705)), (var_t2_dn2 / (2.0 * assign7260_e5705)), (var_t2_dn4 / (2.0 * assign7260_e5705)), (var_t2_dn5 / (2.0 * assign7260_e5705)), (var_t2_dn6 / (2.0 * assign7260_e5705)), (var_t2_dn8 / (2.0 * assign7260_e5705)), (var_t2_dn10 / (2.0 * assign7260_e5705)), (var_t2_dn11 / (2.0 * assign7260_e5705)), (var_t2_dn12 / (2.0 * assign7260_e5705)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign7260_e5707;
        var_t2_dn0 = assign7260_e5707_d_n0;
        var_t2_dn2 = assign7260_e5707_d_n2;
        var_t2_dn4 = assign7260_e5707_d_n4;
        var_t2_dn5 = assign7260_e5707_d_n5;
        var_t2_dn6 = assign7260_e5707_d_n6;
        var_t2_dn8 = assign7260_e5707_d_n8;
        var_t2_dn10 = assign7260_e5707_d_n10;
        var_t2_dn11 = assign7260_e5707_d_n11;
        var_t2_dn12 = assign7260_e5707_d_n12;
        var_t2_rv = 0.0;

        let (assign7270_e5722, assign7270_e5722_d_n0, assign7270_e5722_d_n2, assign7270_e5722_d_n4, assign7270_e5722_d_n5, assign7270_e5722_d_n6, assign7270_e5722_d_n8, assign7270_e5722_d_n10, assign7270_e5722_d_n11, assign7270_e5722_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) {
        let assign7270_e5716: f64 = (2.0 * var_t1);
        let assign7270_e5719: f64 = (var_t0 * var_beta);
        let assign7270_e5720: f64 = (assign7270_e5716 + assign7270_e5719);
        (assign7270_e5720, ((2.0 * var_t1_dn0) + (var_t0_dn0 * var_beta)), ((2.0 * var_t1_dn2) + (var_t0_dn2 * var_beta)), ((2.0 * var_t1_dn4) + ((var_t0_dn4 * var_beta) + (var_t0 * var_beta_dn4))), ((2.0 * var_t1_dn5) + (var_t0_dn5 * var_beta)), ((2.0 * var_t1_dn6) + (var_t0_dn6 * var_beta)), ((2.0 * var_t1_dn8) + (var_t0_dn8 * var_beta)), ((2.0 * var_t1_dn10) + (var_t0_dn10 * var_beta)), ((2.0 * var_t1_dn11) + (var_t0_dn11 * var_beta)), ((2.0 * var_t1_dn12) + (var_t0_dn12 * var_beta)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign7270_e5722;
        var_t3_dn0 = assign7270_e5722_d_n0;
        var_t3_dn2 = assign7270_e5722_d_n2;
        var_t3_dn4 = assign7270_e5722_d_n4;
        var_t3_dn5 = assign7270_e5722_d_n5;
        var_t3_dn6 = assign7270_e5722_d_n6;
        var_t3_dn8 = assign7270_e5722_d_n8;
        var_t3_dn10 = assign7270_e5722_d_n10;
        var_t3_dn11 = assign7270_e5722_d_n11;
        var_t3_dn12 = assign7270_e5722_d_n12;
        var_t3_rv = 0.0;

        let (assign7280_e5735, assign7280_e5735_d_n0, assign7280_e5735_d_n2, assign7280_e5735_d_n4, assign7280_e5735_d_n5, assign7280_e5735_d_n6, assign7280_e5735_d_n8, assign7280_e5735_d_n10, assign7280_e5735_d_n11, assign7280_e5735_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) {
        let assign7280_e5731: f64 = (var_t3 - var_t2);
        let assign7280_e5733: f64 = (assign7280_e5731 / 2.0);
        (assign7280_e5733, ((var_t3_dn0 - var_t2_dn0) / 2.0), ((var_t3_dn2 - var_t2_dn2) / 2.0), ((var_t3_dn4 - var_t2_dn4) / 2.0), ((var_t3_dn5 - var_t2_dn5) / 2.0), ((var_t3_dn6 - var_t2_dn6) / 2.0), ((var_t3_dn8 - var_t2_dn8) / 2.0), ((var_t3_dn10 - var_t2_dn10) / 2.0), ((var_t3_dn11 - var_t2_dn11) / 2.0), ((var_t3_dn12 - var_t2_dn12) / 2.0),)
    } else {
        (var_psb_inia, var_psb_inia_dn0, var_psb_inia_dn2, var_psb_inia_dn4, var_psb_inia_dn5, var_psb_inia_dn6, var_psb_inia_dn8, var_psb_inia_dn10, var_psb_inia_dn11, var_psb_inia_dn12,)
    }
};
        var_psb_inia = assign7280_e5735;
        var_psb_inia_dn0 = assign7280_e5735_d_n0;
        var_psb_inia_dn2 = assign7280_e5735_d_n2;
        var_psb_inia_dn4 = assign7280_e5735_d_n4;
        var_psb_inia_dn5 = assign7280_e5735_d_n5;
        var_psb_inia_dn6 = assign7280_e5735_d_n6;
        var_psb_inia_dn8 = assign7280_e5735_d_n8;
        var_psb_inia_dn10 = assign7280_e5735_d_n10;
        var_psb_inia_dn11 = assign7280_e5735_d_n11;
        var_psb_inia_dn12 = assign7280_e5735_d_n12;
        var_psb_inia_rv = 0.0;

        let (assign7290_e5757, assign7290_e5757_d_n0, assign7290_e5757_d_n2, assign7290_e5757_d_n4, assign7290_e5757_d_n5, assign7290_e5757_d_n6, assign7290_e5757_d_n8, assign7290_e5757_d_n10, assign7290_e5757_d_n11, assign7290_e5757_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) {
        let assign7290_e5744: f64 = (var_t1 * var_t1);
        let assign7290_e5746: f64 = (assign7290_e5744 / var_t0);
        let assign7290_e5748: f64 = (assign7290_e5746 / var_cnst1bulk);
        let assign7290_e5749: f64 = (assign7290_e5748).ln();
        let assign7290_e5753: f64 = (2.0 / var_t1);
        let assign7290_e5754: f64 = (var_beta + assign7290_e5753);
        let assign7290_e5755: f64 = (assign7290_e5749 / assign7290_e5754);
        (assign7290_e5755, ((((((((((((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) * var_t0) - (assign7290_e5744 * var_t0_dn0)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7290_e5746 * var_cnst1bulk_dn0)) / (var_cnst1bulk * var_cnst1bulk)) / assign7290_e5748) * assign7290_e5754) - (assign7290_e5749 * (-((2.0 * var_t1_dn0) / (var_t1 * var_t1))))) / (assign7290_e5754 * assign7290_e5754)), ((((((((((((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) * var_t0) - (assign7290_e5744 * var_t0_dn2)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7290_e5746 * var_cnst1bulk_dn2)) / (var_cnst1bulk * var_cnst1bulk)) / assign7290_e5748) * assign7290_e5754) - (assign7290_e5749 * (-((2.0 * var_t1_dn2) / (var_t1 * var_t1))))) / (assign7290_e5754 * assign7290_e5754)), ((((((((((((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) * var_t0) - (assign7290_e5744 * var_t0_dn4)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7290_e5746 * var_cnst1bulk_dn4)) / (var_cnst1bulk * var_cnst1bulk)) / assign7290_e5748) * assign7290_e5754) - (assign7290_e5749 * (var_beta_dn4 + (-((2.0 * var_t1_dn4) / (var_t1 * var_t1)))))) / (assign7290_e5754 * assign7290_e5754)), ((((((((((((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) * var_t0) - (assign7290_e5744 * var_t0_dn5)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7290_e5746 * var_cnst1bulk_dn5)) / (var_cnst1bulk * var_cnst1bulk)) / assign7290_e5748) * assign7290_e5754) - (assign7290_e5749 * (-((2.0 * var_t1_dn5) / (var_t1 * var_t1))))) / (assign7290_e5754 * assign7290_e5754)), ((((((((((((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) * var_t0) - (assign7290_e5744 * var_t0_dn6)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7290_e5746 * var_cnst1bulk_dn6)) / (var_cnst1bulk * var_cnst1bulk)) / assign7290_e5748) * assign7290_e5754) - (assign7290_e5749 * (-((2.0 * var_t1_dn6) / (var_t1 * var_t1))))) / (assign7290_e5754 * assign7290_e5754)), ((((((((((((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) * var_t0) - (assign7290_e5744 * var_t0_dn8)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7290_e5746 * var_cnst1bulk_dn8)) / (var_cnst1bulk * var_cnst1bulk)) / assign7290_e5748) * assign7290_e5754) - (assign7290_e5749 * (-((2.0 * var_t1_dn8) / (var_t1 * var_t1))))) / (assign7290_e5754 * assign7290_e5754)), ((((((((((((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) * var_t0) - (assign7290_e5744 * var_t0_dn10)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7290_e5746 * var_cnst1bulk_dn10)) / (var_cnst1bulk * var_cnst1bulk)) / assign7290_e5748) * assign7290_e5754) - (assign7290_e5749 * (-((2.0 * var_t1_dn10) / (var_t1 * var_t1))))) / (assign7290_e5754 * assign7290_e5754)), ((((((((((((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) * var_t0) - (assign7290_e5744 * var_t0_dn11)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7290_e5746 * var_cnst1bulk_dn11)) / (var_cnst1bulk * var_cnst1bulk)) / assign7290_e5748) * assign7290_e5754) - (assign7290_e5749 * (-((2.0 * var_t1_dn11) / (var_t1 * var_t1))))) / (assign7290_e5754 * assign7290_e5754)), ((((((((((((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)) * var_t0) - (assign7290_e5744 * var_t0_dn12)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7290_e5746 * var_cnst1bulk_dn12)) / (var_cnst1bulk * var_cnst1bulk)) / assign7290_e5748) * assign7290_e5754) - (assign7290_e5749 * (-((2.0 * var_t1_dn12) / (var_t1 * var_t1))))) / (assign7290_e5754 * assign7290_e5754)),)
    } else {
        (var_psb_inib, var_psb_inib_dn0, var_psb_inib_dn2, var_psb_inib_dn4, var_psb_inib_dn5, var_psb_inib_dn6, var_psb_inib_dn8, var_psb_inib_dn10, var_psb_inib_dn11, var_psb_inib_dn12,)
    }
};
        var_psb_inib = assign7290_e5757;
        var_psb_inib_dn0 = assign7290_e5757_d_n0;
        var_psb_inib_dn2 = assign7290_e5757_d_n2;
        var_psb_inib_dn4 = assign7290_e5757_d_n4;
        var_psb_inib_dn5 = assign7290_e5757_d_n5;
        var_psb_inib_dn6 = assign7290_e5757_d_n6;
        var_psb_inib_dn8 = assign7290_e5757_d_n8;
        var_psb_inib_dn10 = assign7290_e5757_d_n10;
        var_psb_inib_dn11 = assign7290_e5757_d_n11;
        var_psb_inib_dn12 = assign7290_e5757_d_n12;
        var_psb_inib_rv = 0.0;

        let assign7300_e5760: f64 = if var_psb_inia < var_pb2_bulk { 1.0 } else { 0.0 };
        var_guard81 = assign7300_e5760;
        var_guard81_rv = 0.0;

        let (assign7310_e5771, assign7310_e5771_d_n0, assign7310_e5771_d_n2, assign7310_e5771_d_n4, assign7310_e5771_d_n5, assign7310_e5771_d_n6, assign7310_e5771_d_n8, assign7310_e5771_d_n10, assign7310_e5771_d_n11, assign7310_e5771_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) && (var_guard81 != 0.0)) {
        (var_psb_inia, var_psb_inia_dn0, var_psb_inia_dn2, var_psb_inia_dn4, var_psb_inia_dn5, var_psb_inia_dn6, var_psb_inia_dn8, var_psb_inia_dn10, var_psb_inia_dn11, var_psb_inia_dn12,)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
        var_phi_s0_bulk = assign7310_e5771;
        var_phi_s0_bulk_dn0 = assign7310_e5771_d_n0;
        var_phi_s0_bulk_dn2 = assign7310_e5771_d_n2;
        var_phi_s0_bulk_dn4 = assign7310_e5771_d_n4;
        var_phi_s0_bulk_dn5 = assign7310_e5771_d_n5;
        var_phi_s0_bulk_dn6 = assign7310_e5771_d_n6;
        var_phi_s0_bulk_dn8 = assign7310_e5771_d_n8;
        var_phi_s0_bulk_dn10 = assign7310_e5771_d_n10;
        var_phi_s0_bulk_dn11 = assign7310_e5771_d_n11;
        var_phi_s0_bulk_dn12 = assign7310_e5771_d_n12;
        var_phi_s0_bulk_rv = 0.0;

        let (assign7320_e5787, assign7320_e5787_d_n0, assign7320_e5787_d_n2, assign7320_e5787_d_n4, assign7320_e5787_d_n5, assign7320_e5787_d_n6, assign7320_e5787_d_n8, assign7320_e5787_d_n10, assign7320_e5787_d_n11, assign7320_e5787_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) && (var_guard81 == 0.0)) {
        let assign7320_e5783: f64 = (var_psb_inib - var_psb_inia);
        let assign7320_e5785: f64 = (assign7320_e5783 - 0.0008);
        (assign7320_e5785, (var_psb_inib_dn0 - var_psb_inia_dn0), (var_psb_inib_dn2 - var_psb_inia_dn2), (var_psb_inib_dn4 - var_psb_inia_dn4), (var_psb_inib_dn5 - var_psb_inia_dn5), (var_psb_inib_dn6 - var_psb_inia_dn6), (var_psb_inib_dn8 - var_psb_inia_dn8), (var_psb_inib_dn10 - var_psb_inia_dn10), (var_psb_inib_dn11 - var_psb_inia_dn11), (var_psb_inib_dn12 - var_psb_inia_dn12),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn8, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12,)
    }
};
        var_tmf1 = assign7320_e5787;
        var_tmf1_dn0 = assign7320_e5787_d_n0;
        var_tmf1_dn2 = assign7320_e5787_d_n2;
        var_tmf1_dn4 = assign7320_e5787_d_n4;
        var_tmf1_dn5 = assign7320_e5787_d_n5;
        var_tmf1_dn6 = assign7320_e5787_d_n6;
        var_tmf1_dn8 = assign7320_e5787_d_n8;
        var_tmf1_dn10 = assign7320_e5787_d_n10;
        var_tmf1_dn11 = assign7320_e5787_d_n11;
        var_tmf1_dn12 = assign7320_e5787_d_n12;
        var_tmf1_rv = 0.0;

        let (assign7330_e5803, assign7330_e5803_d_n0, assign7330_e5803_d_n2, assign7330_e5803_d_n4, assign7330_e5803_d_n5, assign7330_e5803_d_n6, assign7330_e5803_d_n8, assign7330_e5803_d_n10, assign7330_e5803_d_n11, assign7330_e5803_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) && (var_guard81 == 0.0)) {
        let assign7330_e5799: f64 = (4.0 * var_psb_inib);
        let assign7330_e5801: f64 = (assign7330_e5799 * 0.0008);
        (assign7330_e5801, ((4.0 * var_psb_inib_dn0) * 0.0008), ((4.0 * var_psb_inib_dn2) * 0.0008), ((4.0 * var_psb_inib_dn4) * 0.0008), ((4.0 * var_psb_inib_dn5) * 0.0008), ((4.0 * var_psb_inib_dn6) * 0.0008), ((4.0 * var_psb_inib_dn8) * 0.0008), ((4.0 * var_psb_inib_dn10) * 0.0008), ((4.0 * var_psb_inib_dn11) * 0.0008), ((4.0 * var_psb_inib_dn12) * 0.0008),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
        var_tmf2 = assign7330_e5803;
        var_tmf2_dn0 = assign7330_e5803_d_n0;
        var_tmf2_dn2 = assign7330_e5803_d_n2;
        var_tmf2_dn4 = assign7330_e5803_d_n4;
        var_tmf2_dn5 = assign7330_e5803_d_n5;
        var_tmf2_dn6 = assign7330_e5803_d_n6;
        var_tmf2_dn8 = assign7330_e5803_d_n8;
        var_tmf2_dn10 = assign7330_e5803_d_n10;
        var_tmf2_dn11 = assign7330_e5803_d_n11;
        var_tmf2_dn12 = assign7330_e5803_d_n12;
        var_tmf2_rv = 0.0;

        let (assign7340_e5821, assign7340_e5821_d_n0, assign7340_e5821_d_n2, assign7340_e5821_d_n4, assign7340_e5821_d_n5, assign7340_e5821_d_n6, assign7340_e5821_d_n8, assign7340_e5821_d_n10, assign7340_e5821_d_n11, assign7340_e5821_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) && (var_guard81 == 0.0)) {
        let (assign7340_e5819, assign7340_e5819_d_n0, assign7340_e5819_d_n2, assign7340_e5819_d_n4, assign7340_e5819_d_n5, assign7340_e5819_d_n6, assign7340_e5819_d_n8, assign7340_e5819_d_n10, assign7340_e5819_d_n11, assign7340_e5819_d_n12,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
            } else {
                let assign7340_e5818: f64 = (-var_tmf2);
                (assign7340_e5818, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn8), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12),)
            }
        };
        (assign7340_e5819, assign7340_e5819_d_n0, assign7340_e5819_d_n2, assign7340_e5819_d_n4, assign7340_e5819_d_n5, assign7340_e5819_d_n6, assign7340_e5819_d_n8, assign7340_e5819_d_n10, assign7340_e5819_d_n11, assign7340_e5819_d_n12,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
        var_tmf2 = assign7340_e5821;
        var_tmf2_dn0 = assign7340_e5821_d_n0;
        var_tmf2_dn2 = assign7340_e5821_d_n2;
        var_tmf2_dn4 = assign7340_e5821_d_n4;
        var_tmf2_dn5 = assign7340_e5821_d_n5;
        var_tmf2_dn6 = assign7340_e5821_d_n6;
        var_tmf2_dn8 = assign7340_e5821_d_n8;
        var_tmf2_dn10 = assign7340_e5821_d_n10;
        var_tmf2_dn11 = assign7340_e5821_d_n11;
        var_tmf2_dn12 = assign7340_e5821_d_n12;
        var_tmf2_rv = 0.0;

        let (assign7350_e5838, assign7350_e5838_d_n0, assign7350_e5838_d_n2, assign7350_e5838_d_n4, assign7350_e5838_d_n5, assign7350_e5838_d_n6, assign7350_e5838_d_n8, assign7350_e5838_d_n10, assign7350_e5838_d_n11, assign7350_e5838_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) && (var_guard81 == 0.0)) {
        let assign7350_e5833: f64 = (var_tmf1 * var_tmf1);
        let assign7350_e5835: f64 = (assign7350_e5833 + var_tmf2);
        let assign7350_e5836: f64 = (assign7350_e5835).sqrt();
        (assign7350_e5836, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign7350_e5836)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign7350_e5836)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign7350_e5836)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign7350_e5836)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign7350_e5836)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign7350_e5836)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign7350_e5836)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign7350_e5836)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign7350_e5836)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
        var_tmf2 = assign7350_e5838;
        var_tmf2_dn0 = assign7350_e5838_d_n0;
        var_tmf2_dn2 = assign7350_e5838_d_n2;
        var_tmf2_dn4 = assign7350_e5838_d_n4;
        var_tmf2_dn5 = assign7350_e5838_d_n5;
        var_tmf2_dn6 = assign7350_e5838_d_n6;
        var_tmf2_dn8 = assign7350_e5838_d_n8;
        var_tmf2_dn10 = assign7350_e5838_d_n10;
        var_tmf2_dn11 = assign7350_e5838_d_n11;
        var_tmf2_dn12 = assign7350_e5838_d_n12;
        var_tmf2_rv = 0.0;

        let (assign7360_e5856, assign7360_e5856_d_n0, assign7360_e5856_d_n2, assign7360_e5856_d_n4, assign7360_e5856_d_n5, assign7360_e5856_d_n6, assign7360_e5856_d_n8, assign7360_e5856_d_n10, assign7360_e5856_d_n11, assign7360_e5856_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) && (var_guard81 == 0.0)) {
        let assign7360_e5852: f64 = (var_tmf1 / var_tmf2);
        let assign7360_e5853: f64 = (1.0 + assign7360_e5852);
        let assign7360_e5854: f64 = (0.5 * assign7360_e5853);
        (assign7360_e5854, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn12 * var_tmf2) - (var_tmf1 * var_tmf2_dn12)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign7360_e5856;
        var_t1_dn0 = assign7360_e5856_d_n0;
        var_t1_dn2 = assign7360_e5856_d_n2;
        var_t1_dn4 = assign7360_e5856_d_n4;
        var_t1_dn5 = assign7360_e5856_d_n5;
        var_t1_dn6 = assign7360_e5856_d_n6;
        var_t1_dn8 = assign7360_e5856_d_n8;
        var_t1_dn10 = assign7360_e5856_d_n10;
        var_t1_dn11 = assign7360_e5856_d_n11;
        var_t1_dn12 = assign7360_e5856_d_n12;
        var_t1_rv = 0.0;

        let (assign7370_e5874, assign7370_e5874_d_n0, assign7370_e5874_d_n2, assign7370_e5874_d_n4, assign7370_e5874_d_n5, assign7370_e5874_d_n6, assign7370_e5874_d_n8, assign7370_e5874_d_n10, assign7370_e5874_d_n11, assign7370_e5874_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) && (var_guard81 == 0.0)) {
        let assign7370_e5870: f64 = (var_tmf1 + var_tmf2);
        let assign7370_e5871: f64 = (0.5 * assign7370_e5870);
        let assign7370_e5872: f64 = (var_psb_inib - assign7370_e5871);
        (assign7370_e5872, (var_psb_inib_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_psb_inib_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_psb_inib_dn4 - (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_psb_inib_dn5 - (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_psb_inib_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_psb_inib_dn8 - (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), (var_psb_inib_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_psb_inib_dn11 - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (var_psb_inib_dn12 - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))),)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
        var_phi_s0_bulk = assign7370_e5874;
        var_phi_s0_bulk_dn0 = assign7370_e5874_d_n0;
        var_phi_s0_bulk_dn2 = assign7370_e5874_d_n2;
        var_phi_s0_bulk_dn4 = assign7370_e5874_d_n4;
        var_phi_s0_bulk_dn5 = assign7370_e5874_d_n5;
        var_phi_s0_bulk_dn6 = assign7370_e5874_d_n6;
        var_phi_s0_bulk_dn8 = assign7370_e5874_d_n8;
        var_phi_s0_bulk_dn10 = assign7370_e5874_d_n10;
        var_phi_s0_bulk_dn11 = assign7370_e5874_d_n11;
        var_phi_s0_bulk_dn12 = assign7370_e5874_d_n12;
        var_phi_s0_bulk_rv = 0.0;

        let (assign7380_e5895, assign7380_e5895_d_n0, assign7380_e5895_d_n2, assign7380_e5895_d_n4, assign7380_e5895_d_n5, assign7380_e5895_d_n6, assign7380_e5895_d_n8, assign7380_e5895_d_n10, assign7380_e5895_d_n11, assign7380_e5895_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) {
        let assign7380_e5884: f64 = (var_vbsbiz - var_phi_s0_soi);
        let assign7380_e5887: f64 = (var_q_fd_soi / 2.0);
        let assign7380_e5889: f64 = (assign7380_e5887 * p.p227);
        let assign7380_e5891: f64 = (assign7380_e5889 / 1.034943e-10);
        let assign7380_e5892: f64 = (assign7380_e5884 - assign7380_e5891);
        let assign7380_e5893: f64 = (-assign7380_e5892);
        (assign7380_e5893, (-((var_vbsbiz_dn0 - var_phi_s0_soi_dn0) - (((var_q_fd_soi_dn0 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn2 - var_phi_s0_soi_dn2) - (((var_q_fd_soi_dn2 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn4 - var_phi_s0_soi_dn4) - (((var_q_fd_soi_dn4 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn5 - var_phi_s0_soi_dn5) - (((var_q_fd_soi_dn5 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn6 - var_phi_s0_soi_dn6) - (((var_q_fd_soi_dn6 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn8 - var_phi_s0_soi_dn8) - (((var_q_fd_soi_dn8 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn10 - var_phi_s0_soi_dn10) - (((var_q_fd_soi_dn10 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn11 - var_phi_s0_soi_dn11) - (((var_q_fd_soi_dn11 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn12 - var_phi_s0_soi_dn12) - (((var_q_fd_soi_dn12 / 2.0) * p.p227) / 1.034943e-10))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign7380_e5895;
        var_t1_dn0 = assign7380_e5895_d_n0;
        var_t1_dn2 = assign7380_e5895_d_n2;
        var_t1_dn4 = assign7380_e5895_d_n4;
        var_t1_dn5 = assign7380_e5895_d_n5;
        var_t1_dn6 = assign7380_e5895_d_n6;
        var_t1_dn8 = assign7380_e5895_d_n8;
        var_t1_dn10 = assign7380_e5895_d_n10;
        var_t1_dn11 = assign7380_e5895_d_n11;
        var_t1_dn12 = assign7380_e5895_d_n12;
        var_t1_rv = 0.0;

        let (assign7390_e5927, assign7390_e5927_d_n0, assign7390_e5927_d_n2, assign7390_e5927_d_n4, assign7390_e5927_d_n5, assign7390_e5927_d_n6, assign7390_e5927_d_n8, assign7390_e5927_d_n10, assign7390_e5927_d_n11, assign7390_e5927_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) {
        let assign7390_e5905: f64 = (2.0 * var_t1);
        let assign7390_e5908: f64 = (var_t0 * var_beta);
        let assign7390_e5909: f64 = (assign7390_e5905 + assign7390_e5908);
        let assign7390_e5912: f64 = (2.0 * var_t1);
        let assign7390_e5915: f64 = (var_t0 * var_beta);
        let assign7390_e5916: f64 = (assign7390_e5912 + assign7390_e5915);
        let assign7390_e5917: f64 = (assign7390_e5909 * assign7390_e5916);
        let assign7390_e5921: f64 = (var_t1 * var_t1);
        let assign7390_e5923: f64 = (assign7390_e5921 + var_t0);
        let assign7390_e5924: f64 = (4.0 * assign7390_e5923);
        let assign7390_e5925: f64 = (assign7390_e5917 - assign7390_e5924);
        (assign7390_e5925, (((((2.0 * var_t1_dn0) + (var_t0_dn0 * var_beta)) * assign7390_e5916) + (assign7390_e5909 * ((2.0 * var_t1_dn0) + (var_t0_dn0 * var_beta)))) - (4.0 * (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) + var_t0_dn0))), (((((2.0 * var_t1_dn2) + (var_t0_dn2 * var_beta)) * assign7390_e5916) + (assign7390_e5909 * ((2.0 * var_t1_dn2) + (var_t0_dn2 * var_beta)))) - (4.0 * (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) + var_t0_dn2))), (((((2.0 * var_t1_dn4) + ((var_t0_dn4 * var_beta) + (var_t0 * var_beta_dn4))) * assign7390_e5916) + (assign7390_e5909 * ((2.0 * var_t1_dn4) + ((var_t0_dn4 * var_beta) + (var_t0 * var_beta_dn4))))) - (4.0 * (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) + var_t0_dn4))), (((((2.0 * var_t1_dn5) + (var_t0_dn5 * var_beta)) * assign7390_e5916) + (assign7390_e5909 * ((2.0 * var_t1_dn5) + (var_t0_dn5 * var_beta)))) - (4.0 * (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) + var_t0_dn5))), (((((2.0 * var_t1_dn6) + (var_t0_dn6 * var_beta)) * assign7390_e5916) + (assign7390_e5909 * ((2.0 * var_t1_dn6) + (var_t0_dn6 * var_beta)))) - (4.0 * (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) + var_t0_dn6))), (((((2.0 * var_t1_dn8) + (var_t0_dn8 * var_beta)) * assign7390_e5916) + (assign7390_e5909 * ((2.0 * var_t1_dn8) + (var_t0_dn8 * var_beta)))) - (4.0 * (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) + var_t0_dn8))), (((((2.0 * var_t1_dn10) + (var_t0_dn10 * var_beta)) * assign7390_e5916) + (assign7390_e5909 * ((2.0 * var_t1_dn10) + (var_t0_dn10 * var_beta)))) - (4.0 * (((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) + var_t0_dn10))), (((((2.0 * var_t1_dn11) + (var_t0_dn11 * var_beta)) * assign7390_e5916) + (assign7390_e5909 * ((2.0 * var_t1_dn11) + (var_t0_dn11 * var_beta)))) - (4.0 * (((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) + var_t0_dn11))), (((((2.0 * var_t1_dn12) + (var_t0_dn12 * var_beta)) * assign7390_e5916) + (assign7390_e5909 * ((2.0 * var_t1_dn12) + (var_t0_dn12 * var_beta)))) - (4.0 * (((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)) + var_t0_dn12))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign7390_e5927;
        var_t2_dn0 = assign7390_e5927_d_n0;
        var_t2_dn2 = assign7390_e5927_d_n2;
        var_t2_dn4 = assign7390_e5927_d_n4;
        var_t2_dn5 = assign7390_e5927_d_n5;
        var_t2_dn6 = assign7390_e5927_d_n6;
        var_t2_dn8 = assign7390_e5927_d_n8;
        var_t2_dn10 = assign7390_e5927_d_n10;
        var_t2_dn11 = assign7390_e5927_d_n11;
        var_t2_dn12 = assign7390_e5927_d_n12;
        var_t2_rv = 0.0;

        let (assign7400_e5946, assign7400_e5946_d_n0, assign7400_e5946_d_n2, assign7400_e5946_d_n4, assign7400_e5946_d_n5, assign7400_e5946_d_n6, assign7400_e5946_d_n8, assign7400_e5946_d_n10, assign7400_e5946_d_n11, assign7400_e5946_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) {
        let assign7400_e5938: f64 = (10.0 * 2.220446049250313e-16);
        let (assign7400_e5944, assign7400_e5944_d_n0, assign7400_e5944_d_n2, assign7400_e5944_d_n4, assign7400_e5944_d_n5, assign7400_e5944_d_n6, assign7400_e5944_d_n8, assign7400_e5944_d_n10, assign7400_e5944_d_n11, assign7400_e5944_d_n12,) = {
            if (var_t2 >= assign7400_e5938) {
                (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
            } else {
                let assign7400_e5943: f64 = (10.0 * 2.220446049250313e-16);
                (assign7400_e5943, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign7400_e5944, assign7400_e5944_d_n0, assign7400_e5944_d_n2, assign7400_e5944_d_n4, assign7400_e5944_d_n5, assign7400_e5944_d_n6, assign7400_e5944_d_n8, assign7400_e5944_d_n10, assign7400_e5944_d_n11, assign7400_e5944_d_n12,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign7400_e5946;
        var_t2_dn0 = assign7400_e5946_d_n0;
        var_t2_dn2 = assign7400_e5946_d_n2;
        var_t2_dn4 = assign7400_e5946_d_n4;
        var_t2_dn5 = assign7400_e5946_d_n5;
        var_t2_dn6 = assign7400_e5946_d_n6;
        var_t2_dn8 = assign7400_e5946_d_n8;
        var_t2_dn10 = assign7400_e5946_d_n10;
        var_t2_dn11 = assign7400_e5946_d_n11;
        var_t2_dn12 = assign7400_e5946_d_n12;
        var_t2_rv = 0.0;

        let (assign7410_e5957, assign7410_e5957_d_n0, assign7410_e5957_d_n2, assign7410_e5957_d_n4, assign7410_e5957_d_n5, assign7410_e5957_d_n6, assign7410_e5957_d_n8, assign7410_e5957_d_n10, assign7410_e5957_d_n11, assign7410_e5957_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) {
        let assign7410_e5955: f64 = (var_t2).sqrt();
        (assign7410_e5955, (var_t2_dn0 / (2.0 * assign7410_e5955)), (var_t2_dn2 / (2.0 * assign7410_e5955)), (var_t2_dn4 / (2.0 * assign7410_e5955)), (var_t2_dn5 / (2.0 * assign7410_e5955)), (var_t2_dn6 / (2.0 * assign7410_e5955)), (var_t2_dn8 / (2.0 * assign7410_e5955)), (var_t2_dn10 / (2.0 * assign7410_e5955)), (var_t2_dn11 / (2.0 * assign7410_e5955)), (var_t2_dn12 / (2.0 * assign7410_e5955)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign7410_e5957;
        var_t2_dn0 = assign7410_e5957_d_n0;
        var_t2_dn2 = assign7410_e5957_d_n2;
        var_t2_dn4 = assign7410_e5957_d_n4;
        var_t2_dn5 = assign7410_e5957_d_n5;
        var_t2_dn6 = assign7410_e5957_d_n6;
        var_t2_dn8 = assign7410_e5957_d_n8;
        var_t2_dn10 = assign7410_e5957_d_n10;
        var_t2_dn11 = assign7410_e5957_d_n11;
        var_t2_dn12 = assign7410_e5957_d_n12;
        var_t2_rv = 0.0;

        let (assign7420_e5973, assign7420_e5973_d_n0, assign7420_e5973_d_n2, assign7420_e5973_d_n4, assign7420_e5973_d_n5, assign7420_e5973_d_n6, assign7420_e5973_d_n8, assign7420_e5973_d_n10, assign7420_e5973_d_n11, assign7420_e5973_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) {
        let assign7420_e5967: f64 = (2.0 * var_t1);
        let assign7420_e5970: f64 = (var_t0 * var_beta);
        let assign7420_e5971: f64 = (assign7420_e5967 + assign7420_e5970);
        (assign7420_e5971, ((2.0 * var_t1_dn0) + (var_t0_dn0 * var_beta)), ((2.0 * var_t1_dn2) + (var_t0_dn2 * var_beta)), ((2.0 * var_t1_dn4) + ((var_t0_dn4 * var_beta) + (var_t0 * var_beta_dn4))), ((2.0 * var_t1_dn5) + (var_t0_dn5 * var_beta)), ((2.0 * var_t1_dn6) + (var_t0_dn6 * var_beta)), ((2.0 * var_t1_dn8) + (var_t0_dn8 * var_beta)), ((2.0 * var_t1_dn10) + (var_t0_dn10 * var_beta)), ((2.0 * var_t1_dn11) + (var_t0_dn11 * var_beta)), ((2.0 * var_t1_dn12) + (var_t0_dn12 * var_beta)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign7420_e5973;
        var_t3_dn0 = assign7420_e5973_d_n0;
        var_t3_dn2 = assign7420_e5973_d_n2;
        var_t3_dn4 = assign7420_e5973_d_n4;
        var_t3_dn5 = assign7420_e5973_d_n5;
        var_t3_dn6 = assign7420_e5973_d_n6;
        var_t3_dn8 = assign7420_e5973_d_n8;
        var_t3_dn10 = assign7420_e5973_d_n10;
        var_t3_dn11 = assign7420_e5973_d_n11;
        var_t3_dn12 = assign7420_e5973_d_n12;
        var_t3_rv = 0.0;

        let (assign7430_e5987, assign7430_e5987_d_n0, assign7430_e5987_d_n2, assign7430_e5987_d_n4, assign7430_e5987_d_n5, assign7430_e5987_d_n6, assign7430_e5987_d_n8, assign7430_e5987_d_n10, assign7430_e5987_d_n11, assign7430_e5987_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) {
        let assign7430_e5983: f64 = (var_t3 - var_t2);
        let assign7430_e5985: f64 = (assign7430_e5983 / 2.0);
        (assign7430_e5985, ((var_t3_dn0 - var_t2_dn0) / 2.0), ((var_t3_dn2 - var_t2_dn2) / 2.0), ((var_t3_dn4 - var_t2_dn4) / 2.0), ((var_t3_dn5 - var_t2_dn5) / 2.0), ((var_t3_dn6 - var_t2_dn6) / 2.0), ((var_t3_dn8 - var_t2_dn8) / 2.0), ((var_t3_dn10 - var_t2_dn10) / 2.0), ((var_t3_dn11 - var_t2_dn11) / 2.0), ((var_t3_dn12 - var_t2_dn12) / 2.0),)
    } else {
        (var_psb_inia, var_psb_inia_dn0, var_psb_inia_dn2, var_psb_inia_dn4, var_psb_inia_dn5, var_psb_inia_dn6, var_psb_inia_dn8, var_psb_inia_dn10, var_psb_inia_dn11, var_psb_inia_dn12,)
    }
};
        var_psb_inia = assign7430_e5987;
        var_psb_inia_dn0 = assign7430_e5987_d_n0;
        var_psb_inia_dn2 = assign7430_e5987_d_n2;
        var_psb_inia_dn4 = assign7430_e5987_d_n4;
        var_psb_inia_dn5 = assign7430_e5987_d_n5;
        var_psb_inia_dn6 = assign7430_e5987_d_n6;
        var_psb_inia_dn8 = assign7430_e5987_d_n8;
        var_psb_inia_dn10 = assign7430_e5987_d_n10;
        var_psb_inia_dn11 = assign7430_e5987_d_n11;
        var_psb_inia_dn12 = assign7430_e5987_d_n12;
        var_psb_inia_rv = 0.0;

        let (assign7440_e6010, assign7440_e6010_d_n0, assign7440_e6010_d_n2, assign7440_e6010_d_n4, assign7440_e6010_d_n5, assign7440_e6010_d_n6, assign7440_e6010_d_n8, assign7440_e6010_d_n10, assign7440_e6010_d_n11, assign7440_e6010_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) {
        let assign7440_e5997: f64 = (var_t1 * var_t1);
        let assign7440_e5999: f64 = (assign7440_e5997 / var_t0);
        let assign7440_e6001: f64 = (assign7440_e5999 / var_cnst1bulk);
        let assign7440_e6002: f64 = (assign7440_e6001).ln();
        let assign7440_e6006: f64 = (2.0 / var_t1);
        let assign7440_e6007: f64 = (var_beta + assign7440_e6006);
        let assign7440_e6008: f64 = (assign7440_e6002 / assign7440_e6007);
        (assign7440_e6008, ((((((((((((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) * var_t0) - (assign7440_e5997 * var_t0_dn0)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7440_e5999 * var_cnst1bulk_dn0)) / (var_cnst1bulk * var_cnst1bulk)) / assign7440_e6001) * assign7440_e6007) - (assign7440_e6002 * (-((2.0 * var_t1_dn0) / (var_t1 * var_t1))))) / (assign7440_e6007 * assign7440_e6007)), ((((((((((((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) * var_t0) - (assign7440_e5997 * var_t0_dn2)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7440_e5999 * var_cnst1bulk_dn2)) / (var_cnst1bulk * var_cnst1bulk)) / assign7440_e6001) * assign7440_e6007) - (assign7440_e6002 * (-((2.0 * var_t1_dn2) / (var_t1 * var_t1))))) / (assign7440_e6007 * assign7440_e6007)), ((((((((((((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) * var_t0) - (assign7440_e5997 * var_t0_dn4)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7440_e5999 * var_cnst1bulk_dn4)) / (var_cnst1bulk * var_cnst1bulk)) / assign7440_e6001) * assign7440_e6007) - (assign7440_e6002 * (var_beta_dn4 + (-((2.0 * var_t1_dn4) / (var_t1 * var_t1)))))) / (assign7440_e6007 * assign7440_e6007)), ((((((((((((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) * var_t0) - (assign7440_e5997 * var_t0_dn5)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7440_e5999 * var_cnst1bulk_dn5)) / (var_cnst1bulk * var_cnst1bulk)) / assign7440_e6001) * assign7440_e6007) - (assign7440_e6002 * (-((2.0 * var_t1_dn5) / (var_t1 * var_t1))))) / (assign7440_e6007 * assign7440_e6007)), ((((((((((((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) * var_t0) - (assign7440_e5997 * var_t0_dn6)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7440_e5999 * var_cnst1bulk_dn6)) / (var_cnst1bulk * var_cnst1bulk)) / assign7440_e6001) * assign7440_e6007) - (assign7440_e6002 * (-((2.0 * var_t1_dn6) / (var_t1 * var_t1))))) / (assign7440_e6007 * assign7440_e6007)), ((((((((((((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) * var_t0) - (assign7440_e5997 * var_t0_dn8)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7440_e5999 * var_cnst1bulk_dn8)) / (var_cnst1bulk * var_cnst1bulk)) / assign7440_e6001) * assign7440_e6007) - (assign7440_e6002 * (-((2.0 * var_t1_dn8) / (var_t1 * var_t1))))) / (assign7440_e6007 * assign7440_e6007)), ((((((((((((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) * var_t0) - (assign7440_e5997 * var_t0_dn10)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7440_e5999 * var_cnst1bulk_dn10)) / (var_cnst1bulk * var_cnst1bulk)) / assign7440_e6001) * assign7440_e6007) - (assign7440_e6002 * (-((2.0 * var_t1_dn10) / (var_t1 * var_t1))))) / (assign7440_e6007 * assign7440_e6007)), ((((((((((((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) * var_t0) - (assign7440_e5997 * var_t0_dn11)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7440_e5999 * var_cnst1bulk_dn11)) / (var_cnst1bulk * var_cnst1bulk)) / assign7440_e6001) * assign7440_e6007) - (assign7440_e6002 * (-((2.0 * var_t1_dn11) / (var_t1 * var_t1))))) / (assign7440_e6007 * assign7440_e6007)), ((((((((((((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)) * var_t0) - (assign7440_e5997 * var_t0_dn12)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign7440_e5999 * var_cnst1bulk_dn12)) / (var_cnst1bulk * var_cnst1bulk)) / assign7440_e6001) * assign7440_e6007) - (assign7440_e6002 * (-((2.0 * var_t1_dn12) / (var_t1 * var_t1))))) / (assign7440_e6007 * assign7440_e6007)),)
    } else {
        (var_psb_inib, var_psb_inib_dn0, var_psb_inib_dn2, var_psb_inib_dn4, var_psb_inib_dn5, var_psb_inib_dn6, var_psb_inib_dn8, var_psb_inib_dn10, var_psb_inib_dn11, var_psb_inib_dn12,)
    }
};
        var_psb_inib = assign7440_e6010;
        var_psb_inib_dn0 = assign7440_e6010_d_n0;
        var_psb_inib_dn2 = assign7440_e6010_d_n2;
        var_psb_inib_dn4 = assign7440_e6010_d_n4;
        var_psb_inib_dn5 = assign7440_e6010_d_n5;
        var_psb_inib_dn6 = assign7440_e6010_d_n6;
        var_psb_inib_dn8 = assign7440_e6010_d_n8;
        var_psb_inib_dn10 = assign7440_e6010_d_n10;
        var_psb_inib_dn11 = assign7440_e6010_d_n11;
        var_psb_inib_dn12 = assign7440_e6010_d_n12;
        var_psb_inib_rv = 0.0;

        let assign7450_e6013: f64 = if var_psb_inia < var_pb2_bulk { 1.0 } else { 0.0 };
        var_guard82 = assign7450_e6013;
        var_guard82_rv = 0.0;

        let (assign7460_e6025, assign7460_e6025_d_n0, assign7460_e6025_d_n2, assign7460_e6025_d_n4, assign7460_e6025_d_n5, assign7460_e6025_d_n6, assign7460_e6025_d_n8, assign7460_e6025_d_n10, assign7460_e6025_d_n11, assign7460_e6025_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) && (var_guard82 != 0.0)) {
        (var_psb_inia, var_psb_inia_dn0, var_psb_inia_dn2, var_psb_inia_dn4, var_psb_inia_dn5, var_psb_inia_dn6, var_psb_inia_dn8, var_psb_inia_dn10, var_psb_inia_dn11, var_psb_inia_dn12,)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
        var_phi_s0_bulk = assign7460_e6025;
        var_phi_s0_bulk_dn0 = assign7460_e6025_d_n0;
        var_phi_s0_bulk_dn2 = assign7460_e6025_d_n2;
        var_phi_s0_bulk_dn4 = assign7460_e6025_d_n4;
        var_phi_s0_bulk_dn5 = assign7460_e6025_d_n5;
        var_phi_s0_bulk_dn6 = assign7460_e6025_d_n6;
        var_phi_s0_bulk_dn8 = assign7460_e6025_d_n8;
        var_phi_s0_bulk_dn10 = assign7460_e6025_d_n10;
        var_phi_s0_bulk_dn11 = assign7460_e6025_d_n11;
        var_phi_s0_bulk_dn12 = assign7460_e6025_d_n12;
        var_phi_s0_bulk_rv = 0.0;

        *var_guard81_slot = var_guard81;
        *var_guard81_rv_slot = var_guard81_rv;
        *var_guard82_slot = var_guard82;
        *var_guard82_rv_slot = var_guard82_rv;
        *var_phi_s0_bulk_slot = var_phi_s0_bulk;
        *var_phi_s0_bulk_dn0_slot = var_phi_s0_bulk_dn0;
        *var_phi_s0_bulk_dn10_slot = var_phi_s0_bulk_dn10;
        *var_phi_s0_bulk_dn11_slot = var_phi_s0_bulk_dn11;
        *var_phi_s0_bulk_dn12_slot = var_phi_s0_bulk_dn12;
        *var_phi_s0_bulk_dn2_slot = var_phi_s0_bulk_dn2;
        *var_phi_s0_bulk_dn4_slot = var_phi_s0_bulk_dn4;
        *var_phi_s0_bulk_dn5_slot = var_phi_s0_bulk_dn5;
        *var_phi_s0_bulk_dn6_slot = var_phi_s0_bulk_dn6;
        *var_phi_s0_bulk_dn8_slot = var_phi_s0_bulk_dn8;
        *var_phi_s0_bulk_rv_slot = var_phi_s0_bulk_rv;
        *var_psb_inia_slot = var_psb_inia;
        *var_psb_inia_dn0_slot = var_psb_inia_dn0;
        *var_psb_inia_dn10_slot = var_psb_inia_dn10;
        *var_psb_inia_dn11_slot = var_psb_inia_dn11;
        *var_psb_inia_dn12_slot = var_psb_inia_dn12;
        *var_psb_inia_dn2_slot = var_psb_inia_dn2;
        *var_psb_inia_dn4_slot = var_psb_inia_dn4;
        *var_psb_inia_dn5_slot = var_psb_inia_dn5;
        *var_psb_inia_dn6_slot = var_psb_inia_dn6;
        *var_psb_inia_dn8_slot = var_psb_inia_dn8;
        *var_psb_inia_rv_slot = var_psb_inia_rv;
        *var_psb_inib_slot = var_psb_inib;
        *var_psb_inib_dn0_slot = var_psb_inib_dn0;
        *var_psb_inib_dn10_slot = var_psb_inib_dn10;
        *var_psb_inib_dn11_slot = var_psb_inib_dn11;
        *var_psb_inib_dn12_slot = var_psb_inib_dn12;
        *var_psb_inib_dn2_slot = var_psb_inib_dn2;
        *var_psb_inib_dn4_slot = var_psb_inib_dn4;
        *var_psb_inib_dn5_slot = var_psb_inib_dn5;
        *var_psb_inib_dn6_slot = var_psb_inib_dn6;
        *var_psb_inib_dn8_slot = var_psb_inib_dn8;
        *var_psb_inib_rv_slot = var_psb_inib_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_23(
        p: &Parameters,
        var_guard74: f64,
        var_guard79: f64,
        var_guard80: f64,
        var_guard82: f64,
        var_phi_b_dep: f64,
        var_phi_b_dep_dn0: f64,
        var_phi_b_dep_dn10: f64,
        var_phi_b_dep_dn11: f64,
        var_phi_b_dep_dn12: f64,
        var_phi_b_dep_dn2: f64,
        var_phi_b_dep_dn4: f64,
        var_phi_b_dep_dn5: f64,
        var_phi_b_dep_dn6: f64,
        var_phi_b_dep_dn8: f64,
        var_psb_inia: f64,
        var_psb_inia_dn0: f64,
        var_psb_inia_dn10: f64,
        var_psb_inia_dn11: f64,
        var_psb_inia_dn12: f64,
        var_psb_inia_dn2: f64,
        var_psb_inia_dn4: f64,
        var_psb_inia_dn5: f64,
        var_psb_inia_dn6: f64,
        var_psb_inia_dn8: f64,
        var_psb_inib: f64,
        var_psb_inib_dn0: f64,
        var_psb_inib_dn10: f64,
        var_psb_inib_dn11: f64,
        var_psb_inib_dn12: f64,
        var_psb_inib_dn2: f64,
        var_psb_inib_dn4: f64,
        var_psb_inib_dn5: f64,
        var_psb_inib_dn6: f64,
        var_psb_inib_dn8: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn4: f64,
        var_uc_nsubs_dn5: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn8: f64,
        var_wdsoi: f64,
        var_guard83_slot: &mut f64,
        var_guard83_rv_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
        var_lp_s0_rv_slot: &mut f64,
        var_phi_s0_bulk_slot: &mut f64,
        var_phi_s0_bulk_dn0_slot: &mut f64,
        var_phi_s0_bulk_dn10_slot: &mut f64,
        var_phi_s0_bulk_dn11_slot: &mut f64,
        var_phi_s0_bulk_dn12_slot: &mut f64,
        var_phi_s0_bulk_dn2_slot: &mut f64,
        var_phi_s0_bulk_dn4_slot: &mut f64,
        var_phi_s0_bulk_dn5_slot: &mut f64,
        var_phi_s0_bulk_dn6_slot: &mut f64,
        var_phi_s0_bulk_dn8_slot: &mut f64,
        var_phi_s0_bulk_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_guard83: f64 = *var_guard83_slot;
        let mut var_guard83_rv: f64 = *var_guard83_rv_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
        let mut var_lp_s0_rv: f64 = *var_lp_s0_rv_slot;
        let mut var_phi_s0_bulk: f64 = *var_phi_s0_bulk_slot;
        let mut var_phi_s0_bulk_dn0: f64 = *var_phi_s0_bulk_dn0_slot;
        let mut var_phi_s0_bulk_dn10: f64 = *var_phi_s0_bulk_dn10_slot;
        let mut var_phi_s0_bulk_dn11: f64 = *var_phi_s0_bulk_dn11_slot;
        let mut var_phi_s0_bulk_dn12: f64 = *var_phi_s0_bulk_dn12_slot;
        let mut var_phi_s0_bulk_dn2: f64 = *var_phi_s0_bulk_dn2_slot;
        let mut var_phi_s0_bulk_dn4: f64 = *var_phi_s0_bulk_dn4_slot;
        let mut var_phi_s0_bulk_dn5: f64 = *var_phi_s0_bulk_dn5_slot;
        let mut var_phi_s0_bulk_dn6: f64 = *var_phi_s0_bulk_dn6_slot;
        let mut var_phi_s0_bulk_dn8: f64 = *var_phi_s0_bulk_dn8_slot;
        let mut var_phi_s0_bulk_rv: f64 = *var_phi_s0_bulk_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign7470_e6042, assign7470_e6042_d_n0, assign7470_e6042_d_n2, assign7470_e6042_d_n4, assign7470_e6042_d_n5, assign7470_e6042_d_n6, assign7470_e6042_d_n8, assign7470_e6042_d_n10, assign7470_e6042_d_n11, assign7470_e6042_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) && (var_guard82 == 0.0)) {
        let assign7470_e6038: f64 = (var_psb_inib - var_psb_inia);
        let assign7470_e6040: f64 = (assign7470_e6038 - 0.0008);
        (assign7470_e6040, (var_psb_inib_dn0 - var_psb_inia_dn0), (var_psb_inib_dn2 - var_psb_inia_dn2), (var_psb_inib_dn4 - var_psb_inia_dn4), (var_psb_inib_dn5 - var_psb_inia_dn5), (var_psb_inib_dn6 - var_psb_inia_dn6), (var_psb_inib_dn8 - var_psb_inia_dn8), (var_psb_inib_dn10 - var_psb_inia_dn10), (var_psb_inib_dn11 - var_psb_inia_dn11), (var_psb_inib_dn12 - var_psb_inia_dn12),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn8, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12,)
    }
};
        var_tmf1 = assign7470_e6042;
        var_tmf1_dn0 = assign7470_e6042_d_n0;
        var_tmf1_dn2 = assign7470_e6042_d_n2;
        var_tmf1_dn4 = assign7470_e6042_d_n4;
        var_tmf1_dn5 = assign7470_e6042_d_n5;
        var_tmf1_dn6 = assign7470_e6042_d_n6;
        var_tmf1_dn8 = assign7470_e6042_d_n8;
        var_tmf1_dn10 = assign7470_e6042_d_n10;
        var_tmf1_dn11 = assign7470_e6042_d_n11;
        var_tmf1_dn12 = assign7470_e6042_d_n12;
        var_tmf1_rv = 0.0;

        let (assign7480_e6059, assign7480_e6059_d_n0, assign7480_e6059_d_n2, assign7480_e6059_d_n4, assign7480_e6059_d_n5, assign7480_e6059_d_n6, assign7480_e6059_d_n8, assign7480_e6059_d_n10, assign7480_e6059_d_n11, assign7480_e6059_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) && (var_guard82 == 0.0)) {
        let assign7480_e6055: f64 = (4.0 * var_psb_inib);
        let assign7480_e6057: f64 = (assign7480_e6055 * 0.0008);
        (assign7480_e6057, ((4.0 * var_psb_inib_dn0) * 0.0008), ((4.0 * var_psb_inib_dn2) * 0.0008), ((4.0 * var_psb_inib_dn4) * 0.0008), ((4.0 * var_psb_inib_dn5) * 0.0008), ((4.0 * var_psb_inib_dn6) * 0.0008), ((4.0 * var_psb_inib_dn8) * 0.0008), ((4.0 * var_psb_inib_dn10) * 0.0008), ((4.0 * var_psb_inib_dn11) * 0.0008), ((4.0 * var_psb_inib_dn12) * 0.0008),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
        var_tmf2 = assign7480_e6059;
        var_tmf2_dn0 = assign7480_e6059_d_n0;
        var_tmf2_dn2 = assign7480_e6059_d_n2;
        var_tmf2_dn4 = assign7480_e6059_d_n4;
        var_tmf2_dn5 = assign7480_e6059_d_n5;
        var_tmf2_dn6 = assign7480_e6059_d_n6;
        var_tmf2_dn8 = assign7480_e6059_d_n8;
        var_tmf2_dn10 = assign7480_e6059_d_n10;
        var_tmf2_dn11 = assign7480_e6059_d_n11;
        var_tmf2_dn12 = assign7480_e6059_d_n12;
        var_tmf2_rv = 0.0;

        let (assign7490_e6078, assign7490_e6078_d_n0, assign7490_e6078_d_n2, assign7490_e6078_d_n4, assign7490_e6078_d_n5, assign7490_e6078_d_n6, assign7490_e6078_d_n8, assign7490_e6078_d_n10, assign7490_e6078_d_n11, assign7490_e6078_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) && (var_guard82 == 0.0)) {
        let (assign7490_e6076, assign7490_e6076_d_n0, assign7490_e6076_d_n2, assign7490_e6076_d_n4, assign7490_e6076_d_n5, assign7490_e6076_d_n6, assign7490_e6076_d_n8, assign7490_e6076_d_n10, assign7490_e6076_d_n11, assign7490_e6076_d_n12,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
            } else {
                let assign7490_e6075: f64 = (-var_tmf2);
                (assign7490_e6075, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn8), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12),)
            }
        };
        (assign7490_e6076, assign7490_e6076_d_n0, assign7490_e6076_d_n2, assign7490_e6076_d_n4, assign7490_e6076_d_n5, assign7490_e6076_d_n6, assign7490_e6076_d_n8, assign7490_e6076_d_n10, assign7490_e6076_d_n11, assign7490_e6076_d_n12,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
        var_tmf2 = assign7490_e6078;
        var_tmf2_dn0 = assign7490_e6078_d_n0;
        var_tmf2_dn2 = assign7490_e6078_d_n2;
        var_tmf2_dn4 = assign7490_e6078_d_n4;
        var_tmf2_dn5 = assign7490_e6078_d_n5;
        var_tmf2_dn6 = assign7490_e6078_d_n6;
        var_tmf2_dn8 = assign7490_e6078_d_n8;
        var_tmf2_dn10 = assign7490_e6078_d_n10;
        var_tmf2_dn11 = assign7490_e6078_d_n11;
        var_tmf2_dn12 = assign7490_e6078_d_n12;
        var_tmf2_rv = 0.0;

        let (assign7500_e6096, assign7500_e6096_d_n0, assign7500_e6096_d_n2, assign7500_e6096_d_n4, assign7500_e6096_d_n5, assign7500_e6096_d_n6, assign7500_e6096_d_n8, assign7500_e6096_d_n10, assign7500_e6096_d_n11, assign7500_e6096_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) && (var_guard82 == 0.0)) {
        let assign7500_e6091: f64 = (var_tmf1 * var_tmf1);
        let assign7500_e6093: f64 = (assign7500_e6091 + var_tmf2);
        let assign7500_e6094: f64 = (assign7500_e6093).sqrt();
        (assign7500_e6094, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign7500_e6094)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign7500_e6094)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign7500_e6094)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign7500_e6094)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign7500_e6094)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign7500_e6094)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign7500_e6094)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign7500_e6094)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign7500_e6094)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
        var_tmf2 = assign7500_e6096;
        var_tmf2_dn0 = assign7500_e6096_d_n0;
        var_tmf2_dn2 = assign7500_e6096_d_n2;
        var_tmf2_dn4 = assign7500_e6096_d_n4;
        var_tmf2_dn5 = assign7500_e6096_d_n5;
        var_tmf2_dn6 = assign7500_e6096_d_n6;
        var_tmf2_dn8 = assign7500_e6096_d_n8;
        var_tmf2_dn10 = assign7500_e6096_d_n10;
        var_tmf2_dn11 = assign7500_e6096_d_n11;
        var_tmf2_dn12 = assign7500_e6096_d_n12;
        var_tmf2_rv = 0.0;

        let (assign7510_e6115, assign7510_e6115_d_n0, assign7510_e6115_d_n2, assign7510_e6115_d_n4, assign7510_e6115_d_n5, assign7510_e6115_d_n6, assign7510_e6115_d_n8, assign7510_e6115_d_n10, assign7510_e6115_d_n11, assign7510_e6115_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) && (var_guard82 == 0.0)) {
        let assign7510_e6111: f64 = (var_tmf1 / var_tmf2);
        let assign7510_e6112: f64 = (1.0 + assign7510_e6111);
        let assign7510_e6113: f64 = (0.5 * assign7510_e6112);
        (assign7510_e6113, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn12 * var_tmf2) - (var_tmf1 * var_tmf2_dn12)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign7510_e6115;
        var_t1_dn0 = assign7510_e6115_d_n0;
        var_t1_dn2 = assign7510_e6115_d_n2;
        var_t1_dn4 = assign7510_e6115_d_n4;
        var_t1_dn5 = assign7510_e6115_d_n5;
        var_t1_dn6 = assign7510_e6115_d_n6;
        var_t1_dn8 = assign7510_e6115_d_n8;
        var_t1_dn10 = assign7510_e6115_d_n10;
        var_t1_dn11 = assign7510_e6115_d_n11;
        var_t1_dn12 = assign7510_e6115_d_n12;
        var_t1_rv = 0.0;

        let (assign7520_e6134, assign7520_e6134_d_n0, assign7520_e6134_d_n2, assign7520_e6134_d_n4, assign7520_e6134_d_n5, assign7520_e6134_d_n6, assign7520_e6134_d_n8, assign7520_e6134_d_n10, assign7520_e6134_d_n11, assign7520_e6134_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) && (var_guard82 == 0.0)) {
        let assign7520_e6130: f64 = (var_tmf1 + var_tmf2);
        let assign7520_e6131: f64 = (0.5 * assign7520_e6130);
        let assign7520_e6132: f64 = (var_psb_inib - assign7520_e6131);
        (assign7520_e6132, (var_psb_inib_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_psb_inib_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_psb_inib_dn4 - (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_psb_inib_dn5 - (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_psb_inib_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_psb_inib_dn8 - (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), (var_psb_inib_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_psb_inib_dn11 - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (var_psb_inib_dn12 - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))),)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
        var_phi_s0_bulk = assign7520_e6134;
        var_phi_s0_bulk_dn0 = assign7520_e6134_d_n0;
        var_phi_s0_bulk_dn2 = assign7520_e6134_d_n2;
        var_phi_s0_bulk_dn4 = assign7520_e6134_d_n4;
        var_phi_s0_bulk_dn5 = assign7520_e6134_d_n5;
        var_phi_s0_bulk_dn6 = assign7520_e6134_d_n6;
        var_phi_s0_bulk_dn8 = assign7520_e6134_d_n8;
        var_phi_s0_bulk_dn10 = assign7520_e6134_d_n10;
        var_phi_s0_bulk_dn11 = assign7520_e6134_d_n11;
        var_phi_s0_bulk_dn12 = assign7520_e6134_d_n12;
        var_phi_s0_bulk_rv = 0.0;

        let (assign7530_e6150, assign7530_e6150_d_n0, assign7530_e6150_d_n2, assign7530_e6150_d_n4, assign7530_e6150_d_n5, assign7530_e6150_d_n6, assign7530_e6150_d_n8, assign7530_e6150_d_n10, assign7530_e6150_d_n11, assign7530_e6150_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard79 != 0.0)) {
        let assign7530_e6141: f64 = (2.0 * 1.034943e-10);
        let assign7530_e6143: f64 = (assign7530_e6141 / 1.6021918e-19);
        let assign7530_e6145: f64 = (assign7530_e6143 * var_phi_b_dep);
        let assign7530_e6147: f64 = (assign7530_e6145 / var_uc_nsubs);
        let assign7530_e6148: f64 = (assign7530_e6147).sqrt();
        (assign7530_e6148, (((((assign7530_e6143 * var_phi_b_dep_dn0) * var_uc_nsubs) - (assign7530_e6145 * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7530_e6148)), (((((assign7530_e6143 * var_phi_b_dep_dn2) * var_uc_nsubs) - (assign7530_e6145 * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7530_e6148)), (((((assign7530_e6143 * var_phi_b_dep_dn4) * var_uc_nsubs) - (assign7530_e6145 * var_uc_nsubs_dn4)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7530_e6148)), (((((assign7530_e6143 * var_phi_b_dep_dn5) * var_uc_nsubs) - (assign7530_e6145 * var_uc_nsubs_dn5)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7530_e6148)), (((((assign7530_e6143 * var_phi_b_dep_dn6) * var_uc_nsubs) - (assign7530_e6145 * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7530_e6148)), (((((assign7530_e6143 * var_phi_b_dep_dn8) * var_uc_nsubs) - (assign7530_e6145 * var_uc_nsubs_dn8)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7530_e6148)), (((((assign7530_e6143 * var_phi_b_dep_dn10) * var_uc_nsubs) - (assign7530_e6145 * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7530_e6148)), (((((assign7530_e6143 * var_phi_b_dep_dn11) * var_uc_nsubs) - (assign7530_e6145 * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7530_e6148)), (((((assign7530_e6143 * var_phi_b_dep_dn12) * var_uc_nsubs) - (assign7530_e6145 * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign7530_e6148)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign7530_e6150;
        var_t1_dn0 = assign7530_e6150_d_n0;
        var_t1_dn2 = assign7530_e6150_d_n2;
        var_t1_dn4 = assign7530_e6150_d_n4;
        var_t1_dn5 = assign7530_e6150_d_n5;
        var_t1_dn6 = assign7530_e6150_d_n6;
        var_t1_dn8 = assign7530_e6150_d_n8;
        var_t1_dn10 = assign7530_e6150_d_n10;
        var_t1_dn11 = assign7530_e6150_d_n11;
        var_t1_dn12 = assign7530_e6150_d_n12;
        var_t1_rv = 0.0;

        let assign7540_e6153: f64 = (var_wdsoi + var_t1);
        let assign7540_e6155: f64 = if assign7540_e6153 < p.p227 { 1.0 } else { 0.0 };
        var_guard83 = assign7540_e6155;
        var_guard83_rv = 0.0;

        let (assign7550_e6164,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        (0.0,)
    } else {
        (var_lp_s0,)
    }
};
        var_lp_s0 = assign7550_e6164;
        var_lp_s0_rv = 0.0;

        *var_guard83_slot = var_guard83;
        *var_guard83_rv_slot = var_guard83_rv;
        *var_lp_s0_slot = var_lp_s0;
        *var_lp_s0_rv_slot = var_lp_s0_rv;
        *var_phi_s0_bulk_slot = var_phi_s0_bulk;
        *var_phi_s0_bulk_dn0_slot = var_phi_s0_bulk_dn0;
        *var_phi_s0_bulk_dn10_slot = var_phi_s0_bulk_dn10;
        *var_phi_s0_bulk_dn11_slot = var_phi_s0_bulk_dn11;
        *var_phi_s0_bulk_dn12_slot = var_phi_s0_bulk_dn12;
        *var_phi_s0_bulk_dn2_slot = var_phi_s0_bulk_dn2;
        *var_phi_s0_bulk_dn4_slot = var_phi_s0_bulk_dn4;
        *var_phi_s0_bulk_dn5_slot = var_phi_s0_bulk_dn5;
        *var_phi_s0_bulk_dn6_slot = var_phi_s0_bulk_dn6;
        *var_phi_s0_bulk_dn8_slot = var_phi_s0_bulk_dn8;
        *var_phi_s0_bulk_rv_slot = var_phi_s0_bulk_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_24(
        var_beta: f64,
        var_beta_dn4: f64,
        var_c0bulk: f64,
        var_c0bulk_dn0: f64,
        var_c0bulk_dn10: f64,
        var_c0bulk_dn11: f64,
        var_c0bulk_dn12: f64,
        var_c0bulk_dn2: f64,
        var_c0bulk_dn4: f64,
        var_c0bulk_dn5: f64,
        var_c0bulk_dn6: f64,
        var_c0bulk_dn8: f64,
        var_c_box: f64,
        var_cnst0bulk: f64,
        var_cnst0bulk_dn0: f64,
        var_cnst0bulk_dn10: f64,
        var_cnst0bulk_dn11: f64,
        var_cnst0bulk_dn12: f64,
        var_cnst0bulk_dn2: f64,
        var_cnst0bulk_dn4: f64,
        var_cnst0bulk_dn5: f64,
        var_cnst0bulk_dn6: f64,
        var_cnst0bulk_dn8: f64,
        var_cnst1bulk: f64,
        var_cnst1bulk_dn0: f64,
        var_cnst1bulk_dn10: f64,
        var_cnst1bulk_dn11: f64,
        var_cnst1bulk_dn12: f64,
        var_cnst1bulk_dn2: f64,
        var_cnst1bulk_dn4: f64,
        var_cnst1bulk_dn5: f64,
        var_cnst1bulk_dn6: f64,
        var_cnst1bulk_dn8: f64,
        var_guard74: f64,
        var_guard79: f64,
        var_guard83: f64,
        var_lp_s0_max: f64,
        var_q_fd_soi: f64,
        var_q_fd_soi_dn0: f64,
        var_q_fd_soi_dn10: f64,
        var_q_fd_soi_dn11: f64,
        var_q_fd_soi_dn12: f64,
        var_q_fd_soi_dn2: f64,
        var_q_fd_soi_dn4: f64,
        var_q_fd_soi_dn5: f64,
        var_q_fd_soi_dn6: f64,
        var_q_fd_soi_dn8: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn4: f64,
        var_uc_nsubs_dn5: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn8: f64,
        var_vbsbiz: f64,
        var_vbsbiz_dn0: f64,
        var_vbsbiz_dn10: f64,
        var_vbsbiz_dn11: f64,
        var_vbsbiz_dn12: f64,
        var_vbsbiz_dn2: f64,
        var_vbsbiz_dn4: f64,
        var_vbsbiz_dn5: f64,
        var_vbsbiz_dn6: f64,
        var_vbsbiz_dn8: f64,
        var_guard84_slot: &mut f64,
        var_guard84_rv_slot: &mut f64,
        var_guard85_slot: &mut f64,
        var_guard85_rv_slot: &mut f64,
        var_guard86_slot: &mut f64,
        var_guard86_rv_slot: &mut f64,
        var_guard87_slot: &mut f64,
        var_guard87_rv_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
        var_lp_s0_rv_slot: &mut f64,
        var_phi_b_dep_slot: &mut f64,
        var_phi_b_dep_dn0_slot: &mut f64,
        var_phi_b_dep_dn10_slot: &mut f64,
        var_phi_b_dep_dn11_slot: &mut f64,
        var_phi_b_dep_dn12_slot: &mut f64,
        var_phi_b_dep_dn2_slot: &mut f64,
        var_phi_b_dep_dn4_slot: &mut f64,
        var_phi_b_dep_dn5_slot: &mut f64,
        var_phi_b_dep_dn6_slot: &mut f64,
        var_phi_b_dep_dn8_slot: &mut f64,
        var_phi_b_dep_dpsb_slot: &mut f64,
        var_phi_b_dep_dpsb_dn0_slot: &mut f64,
        var_phi_b_dep_dpsb_dn10_slot: &mut f64,
        var_phi_b_dep_dpsb_dn11_slot: &mut f64,
        var_phi_b_dep_dpsb_dn12_slot: &mut f64,
        var_phi_b_dep_dpsb_dn2_slot: &mut f64,
        var_phi_b_dep_dpsb_dn4_slot: &mut f64,
        var_phi_b_dep_dpsb_dn5_slot: &mut f64,
        var_phi_b_dep_dpsb_dn6_slot: &mut f64,
        var_phi_b_dep_dpsb_dn8_slot: &mut f64,
        var_phi_b_dep_dpsb_rv_slot: &mut f64,
        var_phi_b_dep_rv_slot: &mut f64,
        var_phi_s0_bulk_slot: &mut f64,
        var_phi_s0_bulk_dn0_slot: &mut f64,
        var_phi_s0_bulk_dn10_slot: &mut f64,
        var_phi_s0_bulk_dn11_slot: &mut f64,
        var_phi_s0_bulk_dn12_slot: &mut f64,
        var_phi_s0_bulk_dn2_slot: &mut f64,
        var_phi_s0_bulk_dn4_slot: &mut f64,
        var_phi_s0_bulk_dn5_slot: &mut f64,
        var_phi_s0_bulk_dn6_slot: &mut f64,
        var_phi_s0_bulk_dn8_slot: &mut f64,
        var_phi_s0_bulk_rv_slot: &mut f64,
        var_q_s0_bulk_slot: &mut f64,
        var_q_s0_bulk_dn0_slot: &mut f64,
        var_q_s0_bulk_dn10_slot: &mut f64,
        var_q_s0_bulk_dn11_slot: &mut f64,
        var_q_s0_bulk_dn12_slot: &mut f64,
        var_q_s0_bulk_dn2_slot: &mut f64,
        var_q_s0_bulk_dn4_slot: &mut f64,
        var_q_s0_bulk_dn5_slot: &mut f64,
        var_q_s0_bulk_dn6_slot: &mut f64,
        var_q_s0_bulk_dn8_slot: &mut f64,
        var_q_s0_bulk_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn12_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn12_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn0_slot: &mut f64,
        var_t8_dn10_slot: &mut f64,
        var_t8_dn11_slot: &mut f64,
        var_t8_dn12_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_guard84: f64 = *var_guard84_slot;
        let mut var_guard84_rv: f64 = *var_guard84_rv_slot;
        let mut var_guard85: f64 = *var_guard85_slot;
        let mut var_guard85_rv: f64 = *var_guard85_rv_slot;
        let mut var_guard86: f64 = *var_guard86_slot;
        let mut var_guard86_rv: f64 = *var_guard86_rv_slot;
        let mut var_guard87: f64 = *var_guard87_slot;
        let mut var_guard87_rv: f64 = *var_guard87_rv_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
        let mut var_lp_s0_rv: f64 = *var_lp_s0_rv_slot;
        let mut var_phi_b_dep: f64 = *var_phi_b_dep_slot;
        let mut var_phi_b_dep_dn0: f64 = *var_phi_b_dep_dn0_slot;
        let mut var_phi_b_dep_dn10: f64 = *var_phi_b_dep_dn10_slot;
        let mut var_phi_b_dep_dn11: f64 = *var_phi_b_dep_dn11_slot;
        let mut var_phi_b_dep_dn12: f64 = *var_phi_b_dep_dn12_slot;
        let mut var_phi_b_dep_dn2: f64 = *var_phi_b_dep_dn2_slot;
        let mut var_phi_b_dep_dn4: f64 = *var_phi_b_dep_dn4_slot;
        let mut var_phi_b_dep_dn5: f64 = *var_phi_b_dep_dn5_slot;
        let mut var_phi_b_dep_dn6: f64 = *var_phi_b_dep_dn6_slot;
        let mut var_phi_b_dep_dn8: f64 = *var_phi_b_dep_dn8_slot;
        let mut var_phi_b_dep_dpsb: f64 = *var_phi_b_dep_dpsb_slot;
        let mut var_phi_b_dep_dpsb_dn0: f64 = *var_phi_b_dep_dpsb_dn0_slot;
        let mut var_phi_b_dep_dpsb_dn10: f64 = *var_phi_b_dep_dpsb_dn10_slot;
        let mut var_phi_b_dep_dpsb_dn11: f64 = *var_phi_b_dep_dpsb_dn11_slot;
        let mut var_phi_b_dep_dpsb_dn12: f64 = *var_phi_b_dep_dpsb_dn12_slot;
        let mut var_phi_b_dep_dpsb_dn2: f64 = *var_phi_b_dep_dpsb_dn2_slot;
        let mut var_phi_b_dep_dpsb_dn4: f64 = *var_phi_b_dep_dpsb_dn4_slot;
        let mut var_phi_b_dep_dpsb_dn5: f64 = *var_phi_b_dep_dpsb_dn5_slot;
        let mut var_phi_b_dep_dpsb_dn6: f64 = *var_phi_b_dep_dpsb_dn6_slot;
        let mut var_phi_b_dep_dpsb_dn8: f64 = *var_phi_b_dep_dpsb_dn8_slot;
        let mut var_phi_b_dep_dpsb_rv: f64 = *var_phi_b_dep_dpsb_rv_slot;
        let mut var_phi_b_dep_rv: f64 = *var_phi_b_dep_rv_slot;
        let mut var_phi_s0_bulk: f64 = *var_phi_s0_bulk_slot;
        let mut var_phi_s0_bulk_dn0: f64 = *var_phi_s0_bulk_dn0_slot;
        let mut var_phi_s0_bulk_dn10: f64 = *var_phi_s0_bulk_dn10_slot;
        let mut var_phi_s0_bulk_dn11: f64 = *var_phi_s0_bulk_dn11_slot;
        let mut var_phi_s0_bulk_dn12: f64 = *var_phi_s0_bulk_dn12_slot;
        let mut var_phi_s0_bulk_dn2: f64 = *var_phi_s0_bulk_dn2_slot;
        let mut var_phi_s0_bulk_dn4: f64 = *var_phi_s0_bulk_dn4_slot;
        let mut var_phi_s0_bulk_dn5: f64 = *var_phi_s0_bulk_dn5_slot;
        let mut var_phi_s0_bulk_dn6: f64 = *var_phi_s0_bulk_dn6_slot;
        let mut var_phi_s0_bulk_dn8: f64 = *var_phi_s0_bulk_dn8_slot;
        let mut var_phi_s0_bulk_rv: f64 = *var_phi_s0_bulk_rv_slot;
        let mut var_q_s0_bulk: f64 = *var_q_s0_bulk_slot;
        let mut var_q_s0_bulk_dn0: f64 = *var_q_s0_bulk_dn0_slot;
        let mut var_q_s0_bulk_dn10: f64 = *var_q_s0_bulk_dn10_slot;
        let mut var_q_s0_bulk_dn11: f64 = *var_q_s0_bulk_dn11_slot;
        let mut var_q_s0_bulk_dn12: f64 = *var_q_s0_bulk_dn12_slot;
        let mut var_q_s0_bulk_dn2: f64 = *var_q_s0_bulk_dn2_slot;
        let mut var_q_s0_bulk_dn4: f64 = *var_q_s0_bulk_dn4_slot;
        let mut var_q_s0_bulk_dn5: f64 = *var_q_s0_bulk_dn5_slot;
        let mut var_q_s0_bulk_dn6: f64 = *var_q_s0_bulk_dn6_slot;
        let mut var_q_s0_bulk_dn8: f64 = *var_q_s0_bulk_dn8_slot;
        let mut var_q_s0_bulk_rv: f64 = *var_q_s0_bulk_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn12: f64 = *var_t6_dn12_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn12: f64 = *var_t7_dn12_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn0: f64 = *var_t8_dn0_slot;
        let mut var_t8_dn10: f64 = *var_t8_dn10_slot;
        let mut var_t8_dn11: f64 = *var_t8_dn11_slot;
        let mut var_t8_dn12: f64 = *var_t8_dn12_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_rv: f64 = *var_t8_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let mut assign7560_loop_guard: usize = 0;
        while {
            let assign7560_cond_e6174: f64 = if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) && (var_lp_s0 < var_lp_s0_max)) { 1.0 } else { 0.0 };
            assign7560_cond_e6174 != 0.0
        } {
            assign7560_loop_guard += 1;
            assert!(assign7560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign7560_body0_e6183, assign7560_body0_e6183_d_n0, assign7560_body0_e6183_d_n2, assign7560_body0_e6183_d_n4, assign7560_body0_e6183_d_n5, assign7560_body0_e6183_d_n6, assign7560_body0_e6183_d_n8, assign7560_body0_e6183_d_n10, assign7560_body0_e6183_d_n11, assign7560_body0_e6183_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        (var_cnst0bulk, var_cnst0bulk_dn0, var_cnst0bulk_dn2, var_cnst0bulk_dn4, var_cnst0bulk_dn5, var_cnst0bulk_dn6, var_cnst0bulk_dn8, var_cnst0bulk_dn10, var_cnst0bulk_dn11, var_cnst0bulk_dn12,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
            var_t1 = assign7560_body0_e6183;
            var_t1_dn0 = assign7560_body0_e6183_d_n0;
            var_t1_dn2 = assign7560_body0_e6183_d_n2;
            var_t1_dn4 = assign7560_body0_e6183_d_n4;
            var_t1_dn5 = assign7560_body0_e6183_d_n5;
            var_t1_dn6 = assign7560_body0_e6183_d_n6;
            var_t1_dn8 = assign7560_body0_e6183_d_n8;
            var_t1_dn10 = assign7560_body0_e6183_d_n10;
            var_t1_dn11 = assign7560_body0_e6183_d_n11;
            var_t1_dn12 = assign7560_body0_e6183_d_n12;
            var_t1_rv = 0.0;
            let (assign7560_body1_e6194, assign7560_body1_e6194_d_n0, assign7560_body1_e6194_d_n2, assign7560_body1_e6194_d_n4, assign7560_body1_e6194_d_n5, assign7560_body1_e6194_d_n6, assign7560_body1_e6194_d_n8, assign7560_body1_e6194_d_n10, assign7560_body1_e6194_d_n11, assign7560_body1_e6194_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body1_e6192: f64 = (var_beta * var_phi_s0_bulk);
        (assign7560_body1_e6192, (var_beta * var_phi_s0_bulk_dn0), (var_beta * var_phi_s0_bulk_dn2), ((var_beta_dn4 * var_phi_s0_bulk) + (var_beta * var_phi_s0_bulk_dn4)), (var_beta * var_phi_s0_bulk_dn5), (var_beta * var_phi_s0_bulk_dn6), (var_beta * var_phi_s0_bulk_dn8), (var_beta * var_phi_s0_bulk_dn10), (var_beta * var_phi_s0_bulk_dn11), (var_beta * var_phi_s0_bulk_dn12),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
            var_t2 = assign7560_body1_e6194;
            var_t2_dn0 = assign7560_body1_e6194_d_n0;
            var_t2_dn2 = assign7560_body1_e6194_d_n2;
            var_t2_dn4 = assign7560_body1_e6194_d_n4;
            var_t2_dn5 = assign7560_body1_e6194_d_n5;
            var_t2_dn6 = assign7560_body1_e6194_d_n6;
            var_t2_dn8 = assign7560_body1_e6194_d_n8;
            var_t2_dn10 = assign7560_body1_e6194_d_n10;
            var_t2_dn11 = assign7560_body1_e6194_d_n11;
            var_t2_dn12 = assign7560_body1_e6194_d_n12;
            var_t2_rv = 0.0;
            let (assign7560_body2_e6205, assign7560_body2_e6205_d_n0, assign7560_body2_e6205_d_n2, assign7560_body2_e6205_d_n4, assign7560_body2_e6205_d_n5, assign7560_body2_e6205_d_n6, assign7560_body2_e6205_d_n8, assign7560_body2_e6205_d_n10, assign7560_body2_e6205_d_n11, assign7560_body2_e6205_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body2_e6202: f64 = (-var_t2);
        let assign7560_body2_e6203: f64 = (assign7560_body2_e6202).exp();
        (assign7560_body2_e6203, (assign7560_body2_e6203 * (-var_t2_dn0)), (assign7560_body2_e6203 * (-var_t2_dn2)), (assign7560_body2_e6203 * (-var_t2_dn4)), (assign7560_body2_e6203 * (-var_t2_dn5)), (assign7560_body2_e6203 * (-var_t2_dn6)), (assign7560_body2_e6203 * (-var_t2_dn8)), (assign7560_body2_e6203 * (-var_t2_dn10)), (assign7560_body2_e6203 * (-var_t2_dn11)), (assign7560_body2_e6203 * (-var_t2_dn12)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
            var_t3 = assign7560_body2_e6205;
            var_t3_dn0 = assign7560_body2_e6205_d_n0;
            var_t3_dn2 = assign7560_body2_e6205_d_n2;
            var_t3_dn4 = assign7560_body2_e6205_d_n4;
            var_t3_dn5 = assign7560_body2_e6205_d_n5;
            var_t3_dn6 = assign7560_body2_e6205_d_n6;
            var_t3_dn8 = assign7560_body2_e6205_d_n8;
            var_t3_dn10 = assign7560_body2_e6205_d_n10;
            var_t3_dn11 = assign7560_body2_e6205_d_n11;
            var_t3_dn12 = assign7560_body2_e6205_d_n12;
            var_t3_rv = 0.0;
            let assign7560_body3_e6208: f64 = if var_phi_s0_bulk > 1e-8 { 1.0 } else { 0.0 };
            var_guard84 = assign7560_body3_e6208;
            var_guard84_rv = 0.0;
            let (assign7560_body4_e6222, assign7560_body4_e6222_d_n0, assign7560_body4_e6222_d_n2, assign7560_body4_e6222_d_n4, assign7560_body4_e6222_d_n5, assign7560_body4_e6222_d_n6, assign7560_body4_e6222_d_n8, assign7560_body4_e6222_d_n10, assign7560_body4_e6222_d_n11, assign7560_body4_e6222_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) && (var_guard84 != 0.0)) {
        let assign7560_body4_e6219: f64 = (var_beta * var_phi_s0_bulk);
        let assign7560_body4_e6220: f64 = (assign7560_body4_e6219).exp();
        (assign7560_body4_e6220, (assign7560_body4_e6220 * (var_beta * var_phi_s0_bulk_dn0)), (assign7560_body4_e6220 * (var_beta * var_phi_s0_bulk_dn2)), (assign7560_body4_e6220 * ((var_beta_dn4 * var_phi_s0_bulk) + (var_beta * var_phi_s0_bulk_dn4))), (assign7560_body4_e6220 * (var_beta * var_phi_s0_bulk_dn5)), (assign7560_body4_e6220 * (var_beta * var_phi_s0_bulk_dn6)), (assign7560_body4_e6220 * (var_beta * var_phi_s0_bulk_dn8)), (assign7560_body4_e6220 * (var_beta * var_phi_s0_bulk_dn10)), (assign7560_body4_e6220 * (var_beta * var_phi_s0_bulk_dn11)), (assign7560_body4_e6220 * (var_beta * var_phi_s0_bulk_dn12)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
            var_t0 = assign7560_body4_e6222;
            var_t0_dn0 = assign7560_body4_e6222_d_n0;
            var_t0_dn2 = assign7560_body4_e6222_d_n2;
            var_t0_dn4 = assign7560_body4_e6222_d_n4;
            var_t0_dn5 = assign7560_body4_e6222_d_n5;
            var_t0_dn6 = assign7560_body4_e6222_d_n6;
            var_t0_dn8 = assign7560_body4_e6222_d_n8;
            var_t0_dn10 = assign7560_body4_e6222_d_n10;
            var_t0_dn11 = assign7560_body4_e6222_d_n11;
            var_t0_dn12 = assign7560_body4_e6222_d_n12;
            var_t0_rv = 0.0;
            let (assign7560_body5_e6247, assign7560_body5_e6247_d_n0, assign7560_body5_e6247_d_n2, assign7560_body5_e6247_d_n4, assign7560_body5_e6247_d_n5, assign7560_body5_e6247_d_n6, assign7560_body5_e6247_d_n8, assign7560_body5_e6247_d_n10, assign7560_body5_e6247_d_n11, assign7560_body5_e6247_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) && (var_guard84 != 0.0)) {
        let assign7560_body5_e6232: f64 = (-var_t1);
        let assign7560_body5_e6235: f64 = (var_t3 + var_t2);
        let assign7560_body5_e6237: f64 = (assign7560_body5_e6235 - 1.0);
        let assign7560_body5_e6241: f64 = (var_t0 - 1.0);
        let assign7560_body5_e6242: f64 = (var_cnst1bulk * assign7560_body5_e6241);
        let assign7560_body5_e6243: f64 = (assign7560_body5_e6237 + assign7560_body5_e6242);
        let assign7560_body5_e6244: f64 = (assign7560_body5_e6243).sqrt();
        let assign7560_body5_e6245: f64 = (assign7560_body5_e6232 * assign7560_body5_e6244);
        (assign7560_body5_e6245, (((-var_t1_dn0) * assign7560_body5_e6244) + (assign7560_body5_e6232 * (((var_t3_dn0 + var_t2_dn0) + ((var_cnst1bulk_dn0 * assign7560_body5_e6241) + (var_cnst1bulk * var_t0_dn0))) / (2.0 * assign7560_body5_e6244)))), (((-var_t1_dn2) * assign7560_body5_e6244) + (assign7560_body5_e6232 * (((var_t3_dn2 + var_t2_dn2) + ((var_cnst1bulk_dn2 * assign7560_body5_e6241) + (var_cnst1bulk * var_t0_dn2))) / (2.0 * assign7560_body5_e6244)))), (((-var_t1_dn4) * assign7560_body5_e6244) + (assign7560_body5_e6232 * (((var_t3_dn4 + var_t2_dn4) + ((var_cnst1bulk_dn4 * assign7560_body5_e6241) + (var_cnst1bulk * var_t0_dn4))) / (2.0 * assign7560_body5_e6244)))), (((-var_t1_dn5) * assign7560_body5_e6244) + (assign7560_body5_e6232 * (((var_t3_dn5 + var_t2_dn5) + ((var_cnst1bulk_dn5 * assign7560_body5_e6241) + (var_cnst1bulk * var_t0_dn5))) / (2.0 * assign7560_body5_e6244)))), (((-var_t1_dn6) * assign7560_body5_e6244) + (assign7560_body5_e6232 * (((var_t3_dn6 + var_t2_dn6) + ((var_cnst1bulk_dn6 * assign7560_body5_e6241) + (var_cnst1bulk * var_t0_dn6))) / (2.0 * assign7560_body5_e6244)))), (((-var_t1_dn8) * assign7560_body5_e6244) + (assign7560_body5_e6232 * (((var_t3_dn8 + var_t2_dn8) + ((var_cnst1bulk_dn8 * assign7560_body5_e6241) + (var_cnst1bulk * var_t0_dn8))) / (2.0 * assign7560_body5_e6244)))), (((-var_t1_dn10) * assign7560_body5_e6244) + (assign7560_body5_e6232 * (((var_t3_dn10 + var_t2_dn10) + ((var_cnst1bulk_dn10 * assign7560_body5_e6241) + (var_cnst1bulk * var_t0_dn10))) / (2.0 * assign7560_body5_e6244)))), (((-var_t1_dn11) * assign7560_body5_e6244) + (assign7560_body5_e6232 * (((var_t3_dn11 + var_t2_dn11) + ((var_cnst1bulk_dn11 * assign7560_body5_e6241) + (var_cnst1bulk * var_t0_dn11))) / (2.0 * assign7560_body5_e6244)))), (((-var_t1_dn12) * assign7560_body5_e6244) + (assign7560_body5_e6232 * (((var_t3_dn12 + var_t2_dn12) + ((var_cnst1bulk_dn12 * assign7560_body5_e6241) + (var_cnst1bulk * var_t0_dn12))) / (2.0 * assign7560_body5_e6244)))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign7560_body5_e6247;
            var_t4_dn0 = assign7560_body5_e6247_d_n0;
            var_t4_dn2 = assign7560_body5_e6247_d_n2;
            var_t4_dn4 = assign7560_body5_e6247_d_n4;
            var_t4_dn5 = assign7560_body5_e6247_d_n5;
            var_t4_dn6 = assign7560_body5_e6247_d_n6;
            var_t4_dn8 = assign7560_body5_e6247_d_n8;
            var_t4_dn10 = assign7560_body5_e6247_d_n10;
            var_t4_dn11 = assign7560_body5_e6247_d_n11;
            var_t4_dn12 = assign7560_body5_e6247_d_n12;
            var_t4_rv = 0.0;
            let (assign7560_body6_e6269, assign7560_body6_e6269_d_n0, assign7560_body6_e6269_d_n2, assign7560_body6_e6269_d_n4, assign7560_body6_e6269_d_n5, assign7560_body6_e6269_d_n6, assign7560_body6_e6269_d_n8, assign7560_body6_e6269_d_n10, assign7560_body6_e6269_d_n11, assign7560_body6_e6269_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) && (var_guard84 != 0.0)) {
        let assign7560_body6_e6258: f64 = (var_c0bulk / var_t4);
        let assign7560_body6_e6260: f64 = (-var_t3);
        let assign7560_body6_e6262: f64 = (assign7560_body6_e6260 + 1.0);
        let assign7560_body6_e6265: f64 = (var_cnst1bulk * var_t0);
        let assign7560_body6_e6266: f64 = (assign7560_body6_e6262 + assign7560_body6_e6265);
        let assign7560_body6_e6267: f64 = (assign7560_body6_e6258 * assign7560_body6_e6266);
        (assign7560_body6_e6267, (((((var_c0bulk_dn0 * var_t4) - (var_c0bulk * var_t4_dn0)) / (var_t4 * var_t4)) * assign7560_body6_e6266) + (assign7560_body6_e6258 * ((-var_t3_dn0) + ((var_cnst1bulk_dn0 * var_t0) + (var_cnst1bulk * var_t0_dn0))))), (((((var_c0bulk_dn2 * var_t4) - (var_c0bulk * var_t4_dn2)) / (var_t4 * var_t4)) * assign7560_body6_e6266) + (assign7560_body6_e6258 * ((-var_t3_dn2) + ((var_cnst1bulk_dn2 * var_t0) + (var_cnst1bulk * var_t0_dn2))))), (((((var_c0bulk_dn4 * var_t4) - (var_c0bulk * var_t4_dn4)) / (var_t4 * var_t4)) * assign7560_body6_e6266) + (assign7560_body6_e6258 * ((-var_t3_dn4) + ((var_cnst1bulk_dn4 * var_t0) + (var_cnst1bulk * var_t0_dn4))))), (((((var_c0bulk_dn5 * var_t4) - (var_c0bulk * var_t4_dn5)) / (var_t4 * var_t4)) * assign7560_body6_e6266) + (assign7560_body6_e6258 * ((-var_t3_dn5) + ((var_cnst1bulk_dn5 * var_t0) + (var_cnst1bulk * var_t0_dn5))))), (((((var_c0bulk_dn6 * var_t4) - (var_c0bulk * var_t4_dn6)) / (var_t4 * var_t4)) * assign7560_body6_e6266) + (assign7560_body6_e6258 * ((-var_t3_dn6) + ((var_cnst1bulk_dn6 * var_t0) + (var_cnst1bulk * var_t0_dn6))))), (((((var_c0bulk_dn8 * var_t4) - (var_c0bulk * var_t4_dn8)) / (var_t4 * var_t4)) * assign7560_body6_e6266) + (assign7560_body6_e6258 * ((-var_t3_dn8) + ((var_cnst1bulk_dn8 * var_t0) + (var_cnst1bulk * var_t0_dn8))))), (((((var_c0bulk_dn10 * var_t4) - (var_c0bulk * var_t4_dn10)) / (var_t4 * var_t4)) * assign7560_body6_e6266) + (assign7560_body6_e6258 * ((-var_t3_dn10) + ((var_cnst1bulk_dn10 * var_t0) + (var_cnst1bulk * var_t0_dn10))))), (((((var_c0bulk_dn11 * var_t4) - (var_c0bulk * var_t4_dn11)) / (var_t4 * var_t4)) * assign7560_body6_e6266) + (assign7560_body6_e6258 * ((-var_t3_dn11) + ((var_cnst1bulk_dn11 * var_t0) + (var_cnst1bulk * var_t0_dn11))))), (((((var_c0bulk_dn12 * var_t4) - (var_c0bulk * var_t4_dn12)) / (var_t4 * var_t4)) * assign7560_body6_e6266) + (assign7560_body6_e6258 * ((-var_t3_dn12) + ((var_cnst1bulk_dn12 * var_t0) + (var_cnst1bulk * var_t0_dn12))))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign7560_body6_e6269;
            var_t5_dn0 = assign7560_body6_e6269_d_n0;
            var_t5_dn2 = assign7560_body6_e6269_d_n2;
            var_t5_dn4 = assign7560_body6_e6269_d_n4;
            var_t5_dn5 = assign7560_body6_e6269_d_n5;
            var_t5_dn6 = assign7560_body6_e6269_d_n6;
            var_t5_dn8 = assign7560_body6_e6269_d_n8;
            var_t5_dn10 = assign7560_body6_e6269_d_n10;
            var_t5_dn11 = assign7560_body6_e6269_d_n11;
            var_t5_dn12 = assign7560_body6_e6269_d_n12;
            var_t5_rv = 0.0;
            let assign7560_body7_e6272: f64 = (-1e-8);
            let assign7560_body7_e6273: f64 = if var_phi_s0_bulk < assign7560_body7_e6272 { 1.0 } else { 0.0 };
            var_guard85 = assign7560_body7_e6273;
            var_guard85_rv = 0.0;
            let (assign7560_body8_e6294, assign7560_body8_e6294_d_n0, assign7560_body8_e6294_d_n2, assign7560_body8_e6294_d_n4, assign7560_body8_e6294_d_n5, assign7560_body8_e6294_d_n6, assign7560_body8_e6294_d_n8, assign7560_body8_e6294_d_n10, assign7560_body8_e6294_d_n11, assign7560_body8_e6294_d_n12,) = {
    if (((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) && (var_guard84 == 0.0)) && (var_guard85 != 0.0)) {
        let assign7560_body8_e6288: f64 = (var_t3 + var_t2);
        let assign7560_body8_e6290: f64 = (assign7560_body8_e6288 - 1.0);
        let assign7560_body8_e6291: f64 = (assign7560_body8_e6290).sqrt();
        let assign7560_body8_e6292: f64 = (var_t1 * assign7560_body8_e6291);
        (assign7560_body8_e6292, ((var_t1_dn0 * assign7560_body8_e6291) + (var_t1 * ((var_t3_dn0 + var_t2_dn0) / (2.0 * assign7560_body8_e6291)))), ((var_t1_dn2 * assign7560_body8_e6291) + (var_t1 * ((var_t3_dn2 + var_t2_dn2) / (2.0 * assign7560_body8_e6291)))), ((var_t1_dn4 * assign7560_body8_e6291) + (var_t1 * ((var_t3_dn4 + var_t2_dn4) / (2.0 * assign7560_body8_e6291)))), ((var_t1_dn5 * assign7560_body8_e6291) + (var_t1 * ((var_t3_dn5 + var_t2_dn5) / (2.0 * assign7560_body8_e6291)))), ((var_t1_dn6 * assign7560_body8_e6291) + (var_t1 * ((var_t3_dn6 + var_t2_dn6) / (2.0 * assign7560_body8_e6291)))), ((var_t1_dn8 * assign7560_body8_e6291) + (var_t1 * ((var_t3_dn8 + var_t2_dn8) / (2.0 * assign7560_body8_e6291)))), ((var_t1_dn10 * assign7560_body8_e6291) + (var_t1 * ((var_t3_dn10 + var_t2_dn10) / (2.0 * assign7560_body8_e6291)))), ((var_t1_dn11 * assign7560_body8_e6291) + (var_t1 * ((var_t3_dn11 + var_t2_dn11) / (2.0 * assign7560_body8_e6291)))), ((var_t1_dn12 * assign7560_body8_e6291) + (var_t1 * ((var_t3_dn12 + var_t2_dn12) / (2.0 * assign7560_body8_e6291)))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign7560_body8_e6294;
            var_t4_dn0 = assign7560_body8_e6294_d_n0;
            var_t4_dn2 = assign7560_body8_e6294_d_n2;
            var_t4_dn4 = assign7560_body8_e6294_d_n4;
            var_t4_dn5 = assign7560_body8_e6294_d_n5;
            var_t4_dn6 = assign7560_body8_e6294_d_n6;
            var_t4_dn8 = assign7560_body8_e6294_d_n8;
            var_t4_dn10 = assign7560_body8_e6294_d_n10;
            var_t4_dn11 = assign7560_body8_e6294_d_n11;
            var_t4_dn12 = assign7560_body8_e6294_d_n12;
            var_t4_rv = 0.0;
            let (assign7560_body9_e6315, assign7560_body9_e6315_d_n0, assign7560_body9_e6315_d_n2, assign7560_body9_e6315_d_n4, assign7560_body9_e6315_d_n5, assign7560_body9_e6315_d_n6, assign7560_body9_e6315_d_n8, assign7560_body9_e6315_d_n10, assign7560_body9_e6315_d_n11, assign7560_body9_e6315_d_n12,) = {
    if (((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) && (var_guard84 == 0.0)) && (var_guard85 != 0.0)) {
        let assign7560_body9_e6308: f64 = (var_c0bulk / var_t4);
        let assign7560_body9_e6310: f64 = (-var_t3);
        let assign7560_body9_e6312: f64 = (assign7560_body9_e6310 + 1.0);
        let assign7560_body9_e6313: f64 = (assign7560_body9_e6308 * assign7560_body9_e6312);
        (assign7560_body9_e6313, (((((var_c0bulk_dn0 * var_t4) - (var_c0bulk * var_t4_dn0)) / (var_t4 * var_t4)) * assign7560_body9_e6312) + (assign7560_body9_e6308 * (-var_t3_dn0))), (((((var_c0bulk_dn2 * var_t4) - (var_c0bulk * var_t4_dn2)) / (var_t4 * var_t4)) * assign7560_body9_e6312) + (assign7560_body9_e6308 * (-var_t3_dn2))), (((((var_c0bulk_dn4 * var_t4) - (var_c0bulk * var_t4_dn4)) / (var_t4 * var_t4)) * assign7560_body9_e6312) + (assign7560_body9_e6308 * (-var_t3_dn4))), (((((var_c0bulk_dn5 * var_t4) - (var_c0bulk * var_t4_dn5)) / (var_t4 * var_t4)) * assign7560_body9_e6312) + (assign7560_body9_e6308 * (-var_t3_dn5))), (((((var_c0bulk_dn6 * var_t4) - (var_c0bulk * var_t4_dn6)) / (var_t4 * var_t4)) * assign7560_body9_e6312) + (assign7560_body9_e6308 * (-var_t3_dn6))), (((((var_c0bulk_dn8 * var_t4) - (var_c0bulk * var_t4_dn8)) / (var_t4 * var_t4)) * assign7560_body9_e6312) + (assign7560_body9_e6308 * (-var_t3_dn8))), (((((var_c0bulk_dn10 * var_t4) - (var_c0bulk * var_t4_dn10)) / (var_t4 * var_t4)) * assign7560_body9_e6312) + (assign7560_body9_e6308 * (-var_t3_dn10))), (((((var_c0bulk_dn11 * var_t4) - (var_c0bulk * var_t4_dn11)) / (var_t4 * var_t4)) * assign7560_body9_e6312) + (assign7560_body9_e6308 * (-var_t3_dn11))), (((((var_c0bulk_dn12 * var_t4) - (var_c0bulk * var_t4_dn12)) / (var_t4 * var_t4)) * assign7560_body9_e6312) + (assign7560_body9_e6308 * (-var_t3_dn12))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign7560_body9_e6315;
            var_t5_dn0 = assign7560_body9_e6315_d_n0;
            var_t5_dn2 = assign7560_body9_e6315_d_n2;
            var_t5_dn4 = assign7560_body9_e6315_d_n4;
            var_t5_dn5 = assign7560_body9_e6315_d_n5;
            var_t5_dn6 = assign7560_body9_e6315_d_n6;
            var_t5_dn8 = assign7560_body9_e6315_d_n8;
            var_t5_dn10 = assign7560_body9_e6315_d_n10;
            var_t5_dn11 = assign7560_body9_e6315_d_n11;
            var_t5_dn12 = assign7560_body9_e6315_d_n12;
            var_t5_rv = 0.0;
            let (assign7560_body10_e6338, assign7560_body10_e6338_d_n0, assign7560_body10_e6338_d_n2, assign7560_body10_e6338_d_n4, assign7560_body10_e6338_d_n5, assign7560_body10_e6338_d_n6, assign7560_body10_e6338_d_n8, assign7560_body10_e6338_d_n10, assign7560_body10_e6338_d_n11, assign7560_body10_e6338_d_n12,) = {
    if (((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) && (var_guard84 == 0.0)) && (var_guard85 == 0.0)) {
        let assign7560_body10_e6330: f64 = (var_c0bulk / var_beta);
        let assign7560_body10_e6331: f64 = (assign7560_body10_e6330).sqrt();
        let assign7560_body10_e6332: f64 = (-assign7560_body10_e6331);
        let assign7560_body10_e6334: f64 = (assign7560_body10_e6332 * var_beta);
        let assign7560_body10_e6336: f64 = (assign7560_body10_e6334 * var_phi_s0_bulk);
        (assign7560_body10_e6336, ((((-((var_c0bulk_dn0 / var_beta) / (2.0 * assign7560_body10_e6331))) * var_beta) * var_phi_s0_bulk) + (assign7560_body10_e6334 * var_phi_s0_bulk_dn0)), ((((-((var_c0bulk_dn2 / var_beta) / (2.0 * assign7560_body10_e6331))) * var_beta) * var_phi_s0_bulk) + (assign7560_body10_e6334 * var_phi_s0_bulk_dn2)), (((((-((((var_c0bulk_dn4 * var_beta) - (var_c0bulk * var_beta_dn4)) / (var_beta * var_beta)) / (2.0 * assign7560_body10_e6331))) * var_beta) + (assign7560_body10_e6332 * var_beta_dn4)) * var_phi_s0_bulk) + (assign7560_body10_e6334 * var_phi_s0_bulk_dn4)), ((((-((var_c0bulk_dn5 / var_beta) / (2.0 * assign7560_body10_e6331))) * var_beta) * var_phi_s0_bulk) + (assign7560_body10_e6334 * var_phi_s0_bulk_dn5)), ((((-((var_c0bulk_dn6 / var_beta) / (2.0 * assign7560_body10_e6331))) * var_beta) * var_phi_s0_bulk) + (assign7560_body10_e6334 * var_phi_s0_bulk_dn6)), ((((-((var_c0bulk_dn8 / var_beta) / (2.0 * assign7560_body10_e6331))) * var_beta) * var_phi_s0_bulk) + (assign7560_body10_e6334 * var_phi_s0_bulk_dn8)), ((((-((var_c0bulk_dn10 / var_beta) / (2.0 * assign7560_body10_e6331))) * var_beta) * var_phi_s0_bulk) + (assign7560_body10_e6334 * var_phi_s0_bulk_dn10)), ((((-((var_c0bulk_dn11 / var_beta) / (2.0 * assign7560_body10_e6331))) * var_beta) * var_phi_s0_bulk) + (assign7560_body10_e6334 * var_phi_s0_bulk_dn11)), ((((-((var_c0bulk_dn12 / var_beta) / (2.0 * assign7560_body10_e6331))) * var_beta) * var_phi_s0_bulk) + (assign7560_body10_e6334 * var_phi_s0_bulk_dn12)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign7560_body10_e6338;
            var_t4_dn0 = assign7560_body10_e6338_d_n0;
            var_t4_dn2 = assign7560_body10_e6338_d_n2;
            var_t4_dn4 = assign7560_body10_e6338_d_n4;
            var_t4_dn5 = assign7560_body10_e6338_d_n5;
            var_t4_dn6 = assign7560_body10_e6338_d_n6;
            var_t4_dn8 = assign7560_body10_e6338_d_n8;
            var_t4_dn10 = assign7560_body10_e6338_d_n10;
            var_t4_dn11 = assign7560_body10_e6338_d_n11;
            var_t4_dn12 = assign7560_body10_e6338_d_n12;
            var_t4_rv = 0.0;
            let (assign7560_body11_e6357, assign7560_body11_e6357_d_n0, assign7560_body11_e6357_d_n2, assign7560_body11_e6357_d_n4, assign7560_body11_e6357_d_n5, assign7560_body11_e6357_d_n6, assign7560_body11_e6357_d_n8, assign7560_body11_e6357_d_n10, assign7560_body11_e6357_d_n11, assign7560_body11_e6357_d_n12,) = {
    if (((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) && (var_guard84 == 0.0)) && (var_guard85 == 0.0)) {
        let assign7560_body11_e6353: f64 = (var_c0bulk * var_beta);
        let assign7560_body11_e6354: f64 = (assign7560_body11_e6353).sqrt();
        let assign7560_body11_e6355: f64 = (-assign7560_body11_e6354);
        (assign7560_body11_e6355, (-((var_c0bulk_dn0 * var_beta) / (2.0 * assign7560_body11_e6354))), (-((var_c0bulk_dn2 * var_beta) / (2.0 * assign7560_body11_e6354))), (-(((var_c0bulk_dn4 * var_beta) + (var_c0bulk * var_beta_dn4)) / (2.0 * assign7560_body11_e6354))), (-((var_c0bulk_dn5 * var_beta) / (2.0 * assign7560_body11_e6354))), (-((var_c0bulk_dn6 * var_beta) / (2.0 * assign7560_body11_e6354))), (-((var_c0bulk_dn8 * var_beta) / (2.0 * assign7560_body11_e6354))), (-((var_c0bulk_dn10 * var_beta) / (2.0 * assign7560_body11_e6354))), (-((var_c0bulk_dn11 * var_beta) / (2.0 * assign7560_body11_e6354))), (-((var_c0bulk_dn12 * var_beta) / (2.0 * assign7560_body11_e6354))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign7560_body11_e6357;
            var_t5_dn0 = assign7560_body11_e6357_d_n0;
            var_t5_dn2 = assign7560_body11_e6357_d_n2;
            var_t5_dn4 = assign7560_body11_e6357_d_n4;
            var_t5_dn5 = assign7560_body11_e6357_d_n5;
            var_t5_dn6 = assign7560_body11_e6357_d_n6;
            var_t5_dn8 = assign7560_body11_e6357_d_n8;
            var_t5_dn10 = assign7560_body11_e6357_d_n10;
            var_t5_dn11 = assign7560_body11_e6357_d_n11;
            var_t5_dn12 = assign7560_body11_e6357_d_n12;
            var_t5_rv = 0.0;
            let (assign7560_body12_e6375, assign7560_body12_e6375_d_n0, assign7560_body12_e6375_d_n2, assign7560_body12_e6375_d_n4, assign7560_body12_e6375_d_n5, assign7560_body12_e6375_d_n6, assign7560_body12_e6375_d_n8, assign7560_body12_e6375_d_n10, assign7560_body12_e6375_d_n11, assign7560_body12_e6375_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body12_e6366: f64 = (var_t4 * var_t4);
        let assign7560_body12_e6369: f64 = (4.0 * 1e-10);
        let assign7560_body12_e6371: f64 = (assign7560_body12_e6369 * 1e-10);
        let assign7560_body12_e6372: f64 = (assign7560_body12_e6366 + assign7560_body12_e6371);
        let assign7560_body12_e6373: f64 = (assign7560_body12_e6372).sqrt();
        (assign7560_body12_e6373, (((var_t4_dn0 * var_t4) + (var_t4 * var_t4_dn0)) / (2.0 * assign7560_body12_e6373)), (((var_t4_dn2 * var_t4) + (var_t4 * var_t4_dn2)) / (2.0 * assign7560_body12_e6373)), (((var_t4_dn4 * var_t4) + (var_t4 * var_t4_dn4)) / (2.0 * assign7560_body12_e6373)), (((var_t4_dn5 * var_t4) + (var_t4 * var_t4_dn5)) / (2.0 * assign7560_body12_e6373)), (((var_t4_dn6 * var_t4) + (var_t4 * var_t4_dn6)) / (2.0 * assign7560_body12_e6373)), (((var_t4_dn8 * var_t4) + (var_t4 * var_t4_dn8)) / (2.0 * assign7560_body12_e6373)), (((var_t4_dn10 * var_t4) + (var_t4 * var_t4_dn10)) / (2.0 * assign7560_body12_e6373)), (((var_t4_dn11 * var_t4) + (var_t4 * var_t4_dn11)) / (2.0 * assign7560_body12_e6373)), (((var_t4_dn12 * var_t4) + (var_t4 * var_t4_dn12)) / (2.0 * assign7560_body12_e6373)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
            var_tmf2 = assign7560_body12_e6375;
            var_tmf2_dn0 = assign7560_body12_e6375_d_n0;
            var_tmf2_dn2 = assign7560_body12_e6375_d_n2;
            var_tmf2_dn4 = assign7560_body12_e6375_d_n4;
            var_tmf2_dn5 = assign7560_body12_e6375_d_n5;
            var_tmf2_dn6 = assign7560_body12_e6375_d_n6;
            var_tmf2_dn8 = assign7560_body12_e6375_d_n8;
            var_tmf2_dn10 = assign7560_body12_e6375_d_n10;
            var_tmf2_dn11 = assign7560_body12_e6375_d_n11;
            var_tmf2_dn12 = assign7560_body12_e6375_d_n12;
            var_tmf2_rv = 0.0;
            let (assign7560_body13_e6390, assign7560_body13_e6390_d_n0, assign7560_body13_e6390_d_n2, assign7560_body13_e6390_d_n4, assign7560_body13_e6390_d_n5, assign7560_body13_e6390_d_n6, assign7560_body13_e6390_d_n8, assign7560_body13_e6390_d_n10, assign7560_body13_e6390_d_n11, assign7560_body13_e6390_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body13_e6386: f64 = (var_t4 / var_tmf2);
        let assign7560_body13_e6387: f64 = (1.0 + assign7560_body13_e6386);
        let assign7560_body13_e6388: f64 = (0.5 * assign7560_body13_e6387);
        (assign7560_body13_e6388, (0.5 * (((var_t4_dn0 * var_tmf2) - (var_t4 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn2 * var_tmf2) - (var_t4 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn4 * var_tmf2) - (var_t4 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn5 * var_tmf2) - (var_t4 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn6 * var_tmf2) - (var_t4 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn8 * var_tmf2) - (var_t4 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn10 * var_tmf2) - (var_t4 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn11 * var_tmf2) - (var_t4 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn12 * var_tmf2) - (var_t4 * var_tmf2_dn12)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
            var_t7 = assign7560_body13_e6390;
            var_t7_dn0 = assign7560_body13_e6390_d_n0;
            var_t7_dn2 = assign7560_body13_e6390_d_n2;
            var_t7_dn4 = assign7560_body13_e6390_d_n4;
            var_t7_dn5 = assign7560_body13_e6390_d_n5;
            var_t7_dn6 = assign7560_body13_e6390_d_n6;
            var_t7_dn8 = assign7560_body13_e6390_d_n8;
            var_t7_dn10 = assign7560_body13_e6390_d_n10;
            var_t7_dn11 = assign7560_body13_e6390_d_n11;
            var_t7_dn12 = assign7560_body13_e6390_d_n12;
            var_t7_rv = 0.0;
            let (assign7560_body14_e6407, assign7560_body14_e6407_d_n0, assign7560_body14_e6407_d_n2, assign7560_body14_e6407_d_n4, assign7560_body14_e6407_d_n5, assign7560_body14_e6407_d_n6, assign7560_body14_e6407_d_n8, assign7560_body14_e6407_d_n10, assign7560_body14_e6407_d_n11, assign7560_body14_e6407_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body14_e6400: f64 = (var_t4 + var_tmf2);
        let assign7560_body14_e6401: f64 = (0.5 * assign7560_body14_e6400);
        let assign7560_body14_e6404: f64 = (1e-10 * 1e-10);
        let assign7560_body14_e6405: f64 = (assign7560_body14_e6401 + assign7560_body14_e6404);
        (assign7560_body14_e6405, (0.5 * (var_t4_dn0 + var_tmf2_dn0)), (0.5 * (var_t4_dn2 + var_tmf2_dn2)), (0.5 * (var_t4_dn4 + var_tmf2_dn4)), (0.5 * (var_t4_dn5 + var_tmf2_dn5)), (0.5 * (var_t4_dn6 + var_tmf2_dn6)), (0.5 * (var_t4_dn8 + var_tmf2_dn8)), (0.5 * (var_t4_dn10 + var_tmf2_dn10)), (0.5 * (var_t4_dn11 + var_tmf2_dn11)), (0.5 * (var_t4_dn12 + var_tmf2_dn12)),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
            var_t6 = assign7560_body14_e6407;
            var_t6_dn0 = assign7560_body14_e6407_d_n0;
            var_t6_dn2 = assign7560_body14_e6407_d_n2;
            var_t6_dn4 = assign7560_body14_e6407_d_n4;
            var_t6_dn5 = assign7560_body14_e6407_d_n5;
            var_t6_dn6 = assign7560_body14_e6407_d_n6;
            var_t6_dn8 = assign7560_body14_e6407_d_n8;
            var_t6_dn10 = assign7560_body14_e6407_d_n10;
            var_t6_dn11 = assign7560_body14_e6407_d_n11;
            var_t6_dn12 = assign7560_body14_e6407_d_n12;
            var_t6_rv = 0.0;
            let assign7560_body15_e6410: f64 = if var_t6 < 0.0 { 1.0 } else { 0.0 };
            var_guard86 = assign7560_body15_e6410;
            var_guard86_rv = 0.0;
            let (assign7560_body16_e6421, assign7560_body16_e6421_d_n0, assign7560_body16_e6421_d_n2, assign7560_body16_e6421_d_n4, assign7560_body16_e6421_d_n5, assign7560_body16_e6421_d_n6, assign7560_body16_e6421_d_n8, assign7560_body16_e6421_d_n10, assign7560_body16_e6421_d_n11, assign7560_body16_e6421_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) && (var_guard86 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
            var_t6 = assign7560_body16_e6421;
            var_t6_dn0 = assign7560_body16_e6421_d_n0;
            var_t6_dn2 = assign7560_body16_e6421_d_n2;
            var_t6_dn4 = assign7560_body16_e6421_d_n4;
            var_t6_dn5 = assign7560_body16_e6421_d_n5;
            var_t6_dn6 = assign7560_body16_e6421_d_n6;
            var_t6_dn8 = assign7560_body16_e6421_d_n8;
            var_t6_dn10 = assign7560_body16_e6421_d_n10;
            var_t6_dn11 = assign7560_body16_e6421_d_n11;
            var_t6_dn12 = assign7560_body16_e6421_d_n12;
            var_t6_rv = 0.0;
            let (assign7560_body17_e6432, assign7560_body17_e6432_d_n0, assign7560_body17_e6432_d_n2, assign7560_body17_e6432_d_n4, assign7560_body17_e6432_d_n5, assign7560_body17_e6432_d_n6, assign7560_body17_e6432_d_n8, assign7560_body17_e6432_d_n10, assign7560_body17_e6432_d_n11, assign7560_body17_e6432_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) && (var_guard86 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
            var_t7 = assign7560_body17_e6432;
            var_t7_dn0 = assign7560_body17_e6432_d_n0;
            var_t7_dn2 = assign7560_body17_e6432_d_n2;
            var_t7_dn4 = assign7560_body17_e6432_d_n4;
            var_t7_dn5 = assign7560_body17_e6432_d_n5;
            var_t7_dn6 = assign7560_body17_e6432_d_n6;
            var_t7_dn8 = assign7560_body17_e6432_d_n8;
            var_t7_dn10 = assign7560_body17_e6432_d_n10;
            var_t7_dn11 = assign7560_body17_e6432_d_n11;
            var_t7_dn12 = assign7560_body17_e6432_d_n12;
            var_t7_rv = 0.0;
            let (assign7560_body18_e6446, assign7560_body18_e6446_d_n0, assign7560_body18_e6446_d_n2, assign7560_body18_e6446_d_n4, assign7560_body18_e6446_d_n5, assign7560_body18_e6446_d_n6, assign7560_body18_e6446_d_n8, assign7560_body18_e6446_d_n10, assign7560_body18_e6446_d_n11, assign7560_body18_e6446_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body18_e6440: f64 = (-var_q_fd_soi);
        let assign7560_body18_e6442: f64 = (assign7560_body18_e6440 - var_t6);
        let assign7560_body18_e6444: f64 = (assign7560_body18_e6442 - 1e-13);
        (assign7560_body18_e6444, ((-var_q_fd_soi_dn0) - var_t6_dn0), ((-var_q_fd_soi_dn2) - var_t6_dn2), ((-var_q_fd_soi_dn4) - var_t6_dn4), ((-var_q_fd_soi_dn5) - var_t6_dn5), ((-var_q_fd_soi_dn6) - var_t6_dn6), ((-var_q_fd_soi_dn8) - var_t6_dn8), ((-var_q_fd_soi_dn10) - var_t6_dn10), ((-var_q_fd_soi_dn11) - var_t6_dn11), ((-var_q_fd_soi_dn12) - var_t6_dn12),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn8, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12,)
    }
};
            var_tmf1 = assign7560_body18_e6446;
            var_tmf1_dn0 = assign7560_body18_e6446_d_n0;
            var_tmf1_dn2 = assign7560_body18_e6446_d_n2;
            var_tmf1_dn4 = assign7560_body18_e6446_d_n4;
            var_tmf1_dn5 = assign7560_body18_e6446_d_n5;
            var_tmf1_dn6 = assign7560_body18_e6446_d_n6;
            var_tmf1_dn8 = assign7560_body18_e6446_d_n8;
            var_tmf1_dn10 = assign7560_body18_e6446_d_n10;
            var_tmf1_dn11 = assign7560_body18_e6446_d_n11;
            var_tmf1_dn12 = assign7560_body18_e6446_d_n12;
            var_tmf1_rv = 0.0;
            let (assign7560_body19_e6460, assign7560_body19_e6460_d_n0, assign7560_body19_e6460_d_n2, assign7560_body19_e6460_d_n4, assign7560_body19_e6460_d_n5, assign7560_body19_e6460_d_n6, assign7560_body19_e6460_d_n8, assign7560_body19_e6460_d_n10, assign7560_body19_e6460_d_n11, assign7560_body19_e6460_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body19_e6455: f64 = (-var_q_fd_soi);
        let assign7560_body19_e6456: f64 = (4.0 * assign7560_body19_e6455);
        let assign7560_body19_e6458: f64 = (assign7560_body19_e6456 * 1e-13);
        (assign7560_body19_e6458, ((4.0 * (-var_q_fd_soi_dn0)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn2)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn4)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn5)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn6)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn8)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn10)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn11)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn12)) * 1e-13),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
            var_tmf2 = assign7560_body19_e6460;
            var_tmf2_dn0 = assign7560_body19_e6460_d_n0;
            var_tmf2_dn2 = assign7560_body19_e6460_d_n2;
            var_tmf2_dn4 = assign7560_body19_e6460_d_n4;
            var_tmf2_dn5 = assign7560_body19_e6460_d_n5;
            var_tmf2_dn6 = assign7560_body19_e6460_d_n6;
            var_tmf2_dn8 = assign7560_body19_e6460_d_n8;
            var_tmf2_dn10 = assign7560_body19_e6460_d_n10;
            var_tmf2_dn11 = assign7560_body19_e6460_d_n11;
            var_tmf2_dn12 = assign7560_body19_e6460_d_n12;
            var_tmf2_rv = 0.0;
            let (assign7560_body20_e6475, assign7560_body20_e6475_d_n0, assign7560_body20_e6475_d_n2, assign7560_body20_e6475_d_n4, assign7560_body20_e6475_d_n5, assign7560_body20_e6475_d_n6, assign7560_body20_e6475_d_n8, assign7560_body20_e6475_d_n10, assign7560_body20_e6475_d_n11, assign7560_body20_e6475_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let (assign7560_body20_e6473, assign7560_body20_e6473_d_n0, assign7560_body20_e6473_d_n2, assign7560_body20_e6473_d_n4, assign7560_body20_e6473_d_n5, assign7560_body20_e6473_d_n6, assign7560_body20_e6473_d_n8, assign7560_body20_e6473_d_n10, assign7560_body20_e6473_d_n11, assign7560_body20_e6473_d_n12,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
            } else {
                let assign7560_body20_e6472: f64 = (-var_tmf2);
                (assign7560_body20_e6472, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn8), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12),)
            }
        };
        (assign7560_body20_e6473, assign7560_body20_e6473_d_n0, assign7560_body20_e6473_d_n2, assign7560_body20_e6473_d_n4, assign7560_body20_e6473_d_n5, assign7560_body20_e6473_d_n6, assign7560_body20_e6473_d_n8, assign7560_body20_e6473_d_n10, assign7560_body20_e6473_d_n11, assign7560_body20_e6473_d_n12,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
            var_tmf2 = assign7560_body20_e6475;
            var_tmf2_dn0 = assign7560_body20_e6475_d_n0;
            var_tmf2_dn2 = assign7560_body20_e6475_d_n2;
            var_tmf2_dn4 = assign7560_body20_e6475_d_n4;
            var_tmf2_dn5 = assign7560_body20_e6475_d_n5;
            var_tmf2_dn6 = assign7560_body20_e6475_d_n6;
            var_tmf2_dn8 = assign7560_body20_e6475_d_n8;
            var_tmf2_dn10 = assign7560_body20_e6475_d_n10;
            var_tmf2_dn11 = assign7560_body20_e6475_d_n11;
            var_tmf2_dn12 = assign7560_body20_e6475_d_n12;
            var_tmf2_rv = 0.0;
            let (assign7560_body21_e6489, assign7560_body21_e6489_d_n0, assign7560_body21_e6489_d_n2, assign7560_body21_e6489_d_n4, assign7560_body21_e6489_d_n5, assign7560_body21_e6489_d_n6, assign7560_body21_e6489_d_n8, assign7560_body21_e6489_d_n10, assign7560_body21_e6489_d_n11, assign7560_body21_e6489_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body21_e6484: f64 = (var_tmf1 * var_tmf1);
        let assign7560_body21_e6486: f64 = (assign7560_body21_e6484 + var_tmf2);
        let assign7560_body21_e6487: f64 = (assign7560_body21_e6486).sqrt();
        (assign7560_body21_e6487, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign7560_body21_e6487)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign7560_body21_e6487)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign7560_body21_e6487)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign7560_body21_e6487)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign7560_body21_e6487)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign7560_body21_e6487)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign7560_body21_e6487)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign7560_body21_e6487)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign7560_body21_e6487)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
            var_tmf2 = assign7560_body21_e6489;
            var_tmf2_dn0 = assign7560_body21_e6489_d_n0;
            var_tmf2_dn2 = assign7560_body21_e6489_d_n2;
            var_tmf2_dn4 = assign7560_body21_e6489_d_n4;
            var_tmf2_dn5 = assign7560_body21_e6489_d_n5;
            var_tmf2_dn6 = assign7560_body21_e6489_d_n6;
            var_tmf2_dn8 = assign7560_body21_e6489_d_n8;
            var_tmf2_dn10 = assign7560_body21_e6489_d_n10;
            var_tmf2_dn11 = assign7560_body21_e6489_d_n11;
            var_tmf2_dn12 = assign7560_body21_e6489_d_n12;
            var_tmf2_rv = 0.0;
            let (assign7560_body22_e6504, assign7560_body22_e6504_d_n0, assign7560_body22_e6504_d_n2, assign7560_body22_e6504_d_n4, assign7560_body22_e6504_d_n5, assign7560_body22_e6504_d_n6, assign7560_body22_e6504_d_n8, assign7560_body22_e6504_d_n10, assign7560_body22_e6504_d_n11, assign7560_body22_e6504_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body22_e6500: f64 = (var_tmf1 / var_tmf2);
        let assign7560_body22_e6501: f64 = (1.0 + assign7560_body22_e6500);
        let assign7560_body22_e6502: f64 = (0.5 * assign7560_body22_e6501);
        (assign7560_body22_e6502, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn12 * var_tmf2) - (var_tmf1 * var_tmf2_dn12)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn8, var_t8_dn10, var_t8_dn11, var_t8_dn12,)
    }
};
            var_t8 = assign7560_body22_e6504;
            var_t8_dn0 = assign7560_body22_e6504_d_n0;
            var_t8_dn2 = assign7560_body22_e6504_d_n2;
            var_t8_dn4 = assign7560_body22_e6504_d_n4;
            var_t8_dn5 = assign7560_body22_e6504_d_n5;
            var_t8_dn6 = assign7560_body22_e6504_d_n6;
            var_t8_dn8 = assign7560_body22_e6504_d_n8;
            var_t8_dn10 = assign7560_body22_e6504_d_n10;
            var_t8_dn11 = assign7560_body22_e6504_d_n11;
            var_t8_dn12 = assign7560_body22_e6504_d_n12;
            var_t8_rv = 0.0;
            let (assign7560_body23_e6520, assign7560_body23_e6520_d_n0, assign7560_body23_e6520_d_n2, assign7560_body23_e6520_d_n4, assign7560_body23_e6520_d_n5, assign7560_body23_e6520_d_n6, assign7560_body23_e6520_d_n8, assign7560_body23_e6520_d_n10, assign7560_body23_e6520_d_n11, assign7560_body23_e6520_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body23_e6512: f64 = (-var_q_fd_soi);
        let assign7560_body23_e6516: f64 = (var_tmf1 + var_tmf2);
        let assign7560_body23_e6517: f64 = (0.5 * assign7560_body23_e6516);
        let assign7560_body23_e6518: f64 = (assign7560_body23_e6512 - assign7560_body23_e6517);
        (assign7560_body23_e6518, ((-var_q_fd_soi_dn0) - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), ((-var_q_fd_soi_dn2) - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), ((-var_q_fd_soi_dn4) - (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), ((-var_q_fd_soi_dn5) - (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), ((-var_q_fd_soi_dn6) - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), ((-var_q_fd_soi_dn8) - (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), ((-var_q_fd_soi_dn10) - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), ((-var_q_fd_soi_dn11) - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), ((-var_q_fd_soi_dn12) - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
            var_t6 = assign7560_body23_e6520;
            var_t6_dn0 = assign7560_body23_e6520_d_n0;
            var_t6_dn2 = assign7560_body23_e6520_d_n2;
            var_t6_dn4 = assign7560_body23_e6520_d_n4;
            var_t6_dn5 = assign7560_body23_e6520_d_n5;
            var_t6_dn6 = assign7560_body23_e6520_d_n6;
            var_t6_dn8 = assign7560_body23_e6520_d_n8;
            var_t6_dn10 = assign7560_body23_e6520_d_n10;
            var_t6_dn11 = assign7560_body23_e6520_d_n11;
            var_t6_dn12 = assign7560_body23_e6520_d_n12;
            var_t6_rv = 0.0;
            let (assign7560_body24_e6533, assign7560_body24_e6533_d_n0, assign7560_body24_e6533_d_n2, assign7560_body24_e6533_d_n4, assign7560_body24_e6533_d_n5, assign7560_body24_e6533_d_n6, assign7560_body24_e6533_d_n8, assign7560_body24_e6533_d_n10, assign7560_body24_e6533_d_n11, assign7560_body24_e6533_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body24_e6530: f64 = (var_t5 * var_t8);
        let assign7560_body24_e6531: f64 = (var_t7 * assign7560_body24_e6530);
        (assign7560_body24_e6531, ((var_t7_dn0 * assign7560_body24_e6530) + (var_t7 * ((var_t5_dn0 * var_t8) + (var_t5 * var_t8_dn0)))), ((var_t7_dn2 * assign7560_body24_e6530) + (var_t7 * ((var_t5_dn2 * var_t8) + (var_t5 * var_t8_dn2)))), ((var_t7_dn4 * assign7560_body24_e6530) + (var_t7 * ((var_t5_dn4 * var_t8) + (var_t5 * var_t8_dn4)))), ((var_t7_dn5 * assign7560_body24_e6530) + (var_t7 * ((var_t5_dn5 * var_t8) + (var_t5 * var_t8_dn5)))), ((var_t7_dn6 * assign7560_body24_e6530) + (var_t7 * ((var_t5_dn6 * var_t8) + (var_t5 * var_t8_dn6)))), ((var_t7_dn8 * assign7560_body24_e6530) + (var_t7 * ((var_t5_dn8 * var_t8) + (var_t5 * var_t8_dn8)))), ((var_t7_dn10 * assign7560_body24_e6530) + (var_t7 * ((var_t5_dn10 * var_t8) + (var_t5 * var_t8_dn10)))), ((var_t7_dn11 * assign7560_body24_e6530) + (var_t7 * ((var_t5_dn11 * var_t8) + (var_t5 * var_t8_dn11)))), ((var_t7_dn12 * assign7560_body24_e6530) + (var_t7 * ((var_t5_dn12 * var_t8) + (var_t5 * var_t8_dn12)))),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
            var_t7 = assign7560_body24_e6533;
            var_t7_dn0 = assign7560_body24_e6533_d_n0;
            var_t7_dn2 = assign7560_body24_e6533_d_n2;
            var_t7_dn4 = assign7560_body24_e6533_d_n4;
            var_t7_dn5 = assign7560_body24_e6533_d_n5;
            var_t7_dn6 = assign7560_body24_e6533_d_n6;
            var_t7_dn8 = assign7560_body24_e6533_d_n8;
            var_t7_dn10 = assign7560_body24_e6533_d_n10;
            var_t7_dn11 = assign7560_body24_e6533_d_n11;
            var_t7_dn12 = assign7560_body24_e6533_d_n12;
            var_t7_rv = 0.0;
            let (assign7560_body25_e6552, assign7560_body25_e6552_d_n0, assign7560_body25_e6552_d_n2, assign7560_body25_e6552_d_n4, assign7560_body25_e6552_d_n5, assign7560_body25_e6552_d_n6, assign7560_body25_e6552_d_n8, assign7560_body25_e6552_d_n10, assign7560_body25_e6552_d_n11, assign7560_body25_e6552_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body25_e6542: f64 = (var_t6 * var_t6);
        let assign7560_body25_e6544: f64 = (assign7560_body25_e6542 / 2.0);
        let assign7560_body25_e6546: f64 = (assign7560_body25_e6544 / 1.034943e-10);
        let assign7560_body25_e6548: f64 = (assign7560_body25_e6546 / 1.6021918e-19);
        let assign7560_body25_e6550: f64 = (assign7560_body25_e6548 / var_uc_nsubs);
        (assign7560_body25_e6550, ((((((((var_t6_dn0 * var_t6) + (var_t6 * var_t6_dn0)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7560_body25_e6548 * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn2 * var_t6) + (var_t6 * var_t6_dn2)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7560_body25_e6548 * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn4 * var_t6) + (var_t6 * var_t6_dn4)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7560_body25_e6548 * var_uc_nsubs_dn4)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn5 * var_t6) + (var_t6 * var_t6_dn5)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7560_body25_e6548 * var_uc_nsubs_dn5)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn6 * var_t6) + (var_t6 * var_t6_dn6)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7560_body25_e6548 * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn8 * var_t6) + (var_t6 * var_t6_dn8)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7560_body25_e6548 * var_uc_nsubs_dn8)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn10 * var_t6) + (var_t6 * var_t6_dn10)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7560_body25_e6548 * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn11 * var_t6) + (var_t6 * var_t6_dn11)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7560_body25_e6548 * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn12 * var_t6) + (var_t6 * var_t6_dn12)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7560_body25_e6548 * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)),)
    } else {
        (var_phi_b_dep, var_phi_b_dep_dn0, var_phi_b_dep_dn2, var_phi_b_dep_dn4, var_phi_b_dep_dn5, var_phi_b_dep_dn6, var_phi_b_dep_dn8, var_phi_b_dep_dn10, var_phi_b_dep_dn11, var_phi_b_dep_dn12,)
    }
};
            var_phi_b_dep = assign7560_body25_e6552;
            var_phi_b_dep_dn0 = assign7560_body25_e6552_d_n0;
            var_phi_b_dep_dn2 = assign7560_body25_e6552_d_n2;
            var_phi_b_dep_dn4 = assign7560_body25_e6552_d_n4;
            var_phi_b_dep_dn5 = assign7560_body25_e6552_d_n5;
            var_phi_b_dep_dn6 = assign7560_body25_e6552_d_n6;
            var_phi_b_dep_dn8 = assign7560_body25_e6552_d_n8;
            var_phi_b_dep_dn10 = assign7560_body25_e6552_d_n10;
            var_phi_b_dep_dn11 = assign7560_body25_e6552_d_n11;
            var_phi_b_dep_dn12 = assign7560_body25_e6552_d_n12;
            var_phi_b_dep_rv = 0.0;
            let (assign7560_body26_e6567, assign7560_body26_e6567_d_n0, assign7560_body26_e6567_d_n2, assign7560_body26_e6567_d_n4, assign7560_body26_e6567_d_n5, assign7560_body26_e6567_d_n6, assign7560_body26_e6567_d_n8, assign7560_body26_e6567_d_n10, assign7560_body26_e6567_d_n11, assign7560_body26_e6567_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body26_e6561: f64 = (2.0 * var_phi_b_dep);
        let assign7560_body26_e6563: f64 = (assign7560_body26_e6561 * var_t7);
        let assign7560_body26_e6565: f64 = (assign7560_body26_e6563 / var_t6);
        (assign7560_body26_e6565, ((((((2.0 * var_phi_b_dep_dn0) * var_t7) + (assign7560_body26_e6561 * var_t7_dn0)) * var_t6) - (assign7560_body26_e6563 * var_t6_dn0)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn2) * var_t7) + (assign7560_body26_e6561 * var_t7_dn2)) * var_t6) - (assign7560_body26_e6563 * var_t6_dn2)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn4) * var_t7) + (assign7560_body26_e6561 * var_t7_dn4)) * var_t6) - (assign7560_body26_e6563 * var_t6_dn4)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn5) * var_t7) + (assign7560_body26_e6561 * var_t7_dn5)) * var_t6) - (assign7560_body26_e6563 * var_t6_dn5)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn6) * var_t7) + (assign7560_body26_e6561 * var_t7_dn6)) * var_t6) - (assign7560_body26_e6563 * var_t6_dn6)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn8) * var_t7) + (assign7560_body26_e6561 * var_t7_dn8)) * var_t6) - (assign7560_body26_e6563 * var_t6_dn8)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn10) * var_t7) + (assign7560_body26_e6561 * var_t7_dn10)) * var_t6) - (assign7560_body26_e6563 * var_t6_dn10)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn11) * var_t7) + (assign7560_body26_e6561 * var_t7_dn11)) * var_t6) - (assign7560_body26_e6563 * var_t6_dn11)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn12) * var_t7) + (assign7560_body26_e6561 * var_t7_dn12)) * var_t6) - (assign7560_body26_e6563 * var_t6_dn12)) / (var_t6 * var_t6)),)
    } else {
        (var_phi_b_dep_dpsb, var_phi_b_dep_dpsb_dn0, var_phi_b_dep_dpsb_dn2, var_phi_b_dep_dpsb_dn4, var_phi_b_dep_dpsb_dn5, var_phi_b_dep_dpsb_dn6, var_phi_b_dep_dpsb_dn8, var_phi_b_dep_dpsb_dn10, var_phi_b_dep_dpsb_dn11, var_phi_b_dep_dpsb_dn12,)
    }
};
            var_phi_b_dep_dpsb = assign7560_body26_e6567;
            var_phi_b_dep_dpsb_dn0 = assign7560_body26_e6567_d_n0;
            var_phi_b_dep_dpsb_dn2 = assign7560_body26_e6567_d_n2;
            var_phi_b_dep_dpsb_dn4 = assign7560_body26_e6567_d_n4;
            var_phi_b_dep_dpsb_dn5 = assign7560_body26_e6567_d_n5;
            var_phi_b_dep_dpsb_dn6 = assign7560_body26_e6567_d_n6;
            var_phi_b_dep_dpsb_dn8 = assign7560_body26_e6567_d_n8;
            var_phi_b_dep_dpsb_dn10 = assign7560_body26_e6567_d_n10;
            var_phi_b_dep_dpsb_dn11 = assign7560_body26_e6567_d_n11;
            var_phi_b_dep_dpsb_dn12 = assign7560_body26_e6567_d_n12;
            var_phi_b_dep_dpsb_rv = 0.0;
            let (assign7560_body27_e6596, assign7560_body27_e6596_d_n0, assign7560_body27_e6596_d_n2, assign7560_body27_e6596_d_n4, assign7560_body27_e6596_d_n5, assign7560_body27_e6596_d_n6, assign7560_body27_e6596_d_n8, assign7560_body27_e6596_d_n10, assign7560_body27_e6596_d_n11, assign7560_body27_e6596_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body27_e6576: f64 = (-var_phi_s0_bulk);
        let assign7560_body27_e6579: f64 = (var_t4 / var_c_box);
        let assign7560_body27_e6580: f64 = (assign7560_body27_e6576 + assign7560_body27_e6579);
        let assign7560_body27_e6582: f64 = (assign7560_body27_e6580 - var_vbsbiz);
        let assign7560_body27_e6584: f64 = (assign7560_body27_e6582 + var_phi_b_dep);
        let assign7560_body27_e6586: f64 = (-1.0);
        let assign7560_body27_e6589: f64 = (var_t5 / var_c_box);
        let assign7560_body27_e6590: f64 = (assign7560_body27_e6586 + assign7560_body27_e6589);
        let assign7560_body27_e6592: f64 = (assign7560_body27_e6590 + var_phi_b_dep_dpsb);
        let assign7560_body27_e6593: f64 = (assign7560_body27_e6584 / assign7560_body27_e6592);
        let assign7560_body27_e6594: f64 = (var_phi_s0_bulk - assign7560_body27_e6593);
        (assign7560_body27_e6594, (var_phi_s0_bulk_dn0 - (((((((-var_phi_s0_bulk_dn0) + (var_t4_dn0 / var_c_box)) - var_vbsbiz_dn0) + var_phi_b_dep_dn0) * assign7560_body27_e6592) - (assign7560_body27_e6584 * ((var_t5_dn0 / var_c_box) + var_phi_b_dep_dpsb_dn0))) / (assign7560_body27_e6592 * assign7560_body27_e6592))), (var_phi_s0_bulk_dn2 - (((((((-var_phi_s0_bulk_dn2) + (var_t4_dn2 / var_c_box)) - var_vbsbiz_dn2) + var_phi_b_dep_dn2) * assign7560_body27_e6592) - (assign7560_body27_e6584 * ((var_t5_dn2 / var_c_box) + var_phi_b_dep_dpsb_dn2))) / (assign7560_body27_e6592 * assign7560_body27_e6592))), (var_phi_s0_bulk_dn4 - (((((((-var_phi_s0_bulk_dn4) + (var_t4_dn4 / var_c_box)) - var_vbsbiz_dn4) + var_phi_b_dep_dn4) * assign7560_body27_e6592) - (assign7560_body27_e6584 * ((var_t5_dn4 / var_c_box) + var_phi_b_dep_dpsb_dn4))) / (assign7560_body27_e6592 * assign7560_body27_e6592))), (var_phi_s0_bulk_dn5 - (((((((-var_phi_s0_bulk_dn5) + (var_t4_dn5 / var_c_box)) - var_vbsbiz_dn5) + var_phi_b_dep_dn5) * assign7560_body27_e6592) - (assign7560_body27_e6584 * ((var_t5_dn5 / var_c_box) + var_phi_b_dep_dpsb_dn5))) / (assign7560_body27_e6592 * assign7560_body27_e6592))), (var_phi_s0_bulk_dn6 - (((((((-var_phi_s0_bulk_dn6) + (var_t4_dn6 / var_c_box)) - var_vbsbiz_dn6) + var_phi_b_dep_dn6) * assign7560_body27_e6592) - (assign7560_body27_e6584 * ((var_t5_dn6 / var_c_box) + var_phi_b_dep_dpsb_dn6))) / (assign7560_body27_e6592 * assign7560_body27_e6592))), (var_phi_s0_bulk_dn8 - (((((((-var_phi_s0_bulk_dn8) + (var_t4_dn8 / var_c_box)) - var_vbsbiz_dn8) + var_phi_b_dep_dn8) * assign7560_body27_e6592) - (assign7560_body27_e6584 * ((var_t5_dn8 / var_c_box) + var_phi_b_dep_dpsb_dn8))) / (assign7560_body27_e6592 * assign7560_body27_e6592))), (var_phi_s0_bulk_dn10 - (((((((-var_phi_s0_bulk_dn10) + (var_t4_dn10 / var_c_box)) - var_vbsbiz_dn10) + var_phi_b_dep_dn10) * assign7560_body27_e6592) - (assign7560_body27_e6584 * ((var_t5_dn10 / var_c_box) + var_phi_b_dep_dpsb_dn10))) / (assign7560_body27_e6592 * assign7560_body27_e6592))), (var_phi_s0_bulk_dn11 - (((((((-var_phi_s0_bulk_dn11) + (var_t4_dn11 / var_c_box)) - var_vbsbiz_dn11) + var_phi_b_dep_dn11) * assign7560_body27_e6592) - (assign7560_body27_e6584 * ((var_t5_dn11 / var_c_box) + var_phi_b_dep_dpsb_dn11))) / (assign7560_body27_e6592 * assign7560_body27_e6592))), (var_phi_s0_bulk_dn12 - (((((((-var_phi_s0_bulk_dn12) + (var_t4_dn12 / var_c_box)) - var_vbsbiz_dn12) + var_phi_b_dep_dn12) * assign7560_body27_e6592) - (assign7560_body27_e6584 * ((var_t5_dn12 / var_c_box) + var_phi_b_dep_dpsb_dn12))) / (assign7560_body27_e6592 * assign7560_body27_e6592))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
            var_t6 = assign7560_body27_e6596;
            var_t6_dn0 = assign7560_body27_e6596_d_n0;
            var_t6_dn2 = assign7560_body27_e6596_d_n2;
            var_t6_dn4 = assign7560_body27_e6596_d_n4;
            var_t6_dn5 = assign7560_body27_e6596_d_n5;
            var_t6_dn6 = assign7560_body27_e6596_d_n6;
            var_t6_dn8 = assign7560_body27_e6596_d_n8;
            var_t6_dn10 = assign7560_body27_e6596_d_n10;
            var_t6_dn11 = assign7560_body27_e6596_d_n11;
            var_t6_dn12 = assign7560_body27_e6596_d_n12;
            var_t6_rv = 0.0;
            let assign7560_body28_e6599: f64 = (var_t6 - var_phi_s0_bulk);
            let assign7560_body28_e6600: f64 = (assign7560_body28_e6599).abs();
            let assign7560_body28_e6602: f64 = if assign7560_body28_e6600 < 0.001 { 1.0 } else { 0.0 };
            var_guard87 = assign7560_body28_e6602;
            var_guard87_rv = 0.0;
            let (assign7560_body29_e6613,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) && (var_guard87 != 0.0)) {
        (var_lp_s0_max,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign7560_body29_e6613;
            var_lp_s0_rv = 0.0;
            let (assign7560_body30_e6622, assign7560_body30_e6622_d_n0, assign7560_body30_e6622_d_n2, assign7560_body30_e6622_d_n4, assign7560_body30_e6622_d_n5, assign7560_body30_e6622_d_n6, assign7560_body30_e6622_d_n8, assign7560_body30_e6622_d_n10, assign7560_body30_e6622_d_n11, assign7560_body30_e6622_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
            var_phi_s0_bulk = assign7560_body30_e6622;
            var_phi_s0_bulk_dn0 = assign7560_body30_e6622_d_n0;
            var_phi_s0_bulk_dn2 = assign7560_body30_e6622_d_n2;
            var_phi_s0_bulk_dn4 = assign7560_body30_e6622_d_n4;
            var_phi_s0_bulk_dn5 = assign7560_body30_e6622_d_n5;
            var_phi_s0_bulk_dn6 = assign7560_body30_e6622_d_n6;
            var_phi_s0_bulk_dn8 = assign7560_body30_e6622_d_n8;
            var_phi_s0_bulk_dn10 = assign7560_body30_e6622_d_n10;
            var_phi_s0_bulk_dn11 = assign7560_body30_e6622_d_n11;
            var_phi_s0_bulk_dn12 = assign7560_body30_e6622_d_n12;
            var_phi_s0_bulk_rv = 0.0;
            let (assign7560_body31_e6631, assign7560_body31_e6631_d_n0, assign7560_body31_e6631_d_n2, assign7560_body31_e6631_d_n4, assign7560_body31_e6631_d_n5, assign7560_body31_e6631_d_n6, assign7560_body31_e6631_d_n8, assign7560_body31_e6631_d_n10, assign7560_body31_e6631_d_n11, assign7560_body31_e6631_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    } else {
        (var_q_s0_bulk, var_q_s0_bulk_dn0, var_q_s0_bulk_dn2, var_q_s0_bulk_dn4, var_q_s0_bulk_dn5, var_q_s0_bulk_dn6, var_q_s0_bulk_dn8, var_q_s0_bulk_dn10, var_q_s0_bulk_dn11, var_q_s0_bulk_dn12,)
    }
};
            var_q_s0_bulk = assign7560_body31_e6631;
            var_q_s0_bulk_dn0 = assign7560_body31_e6631_d_n0;
            var_q_s0_bulk_dn2 = assign7560_body31_e6631_d_n2;
            var_q_s0_bulk_dn4 = assign7560_body31_e6631_d_n4;
            var_q_s0_bulk_dn5 = assign7560_body31_e6631_d_n5;
            var_q_s0_bulk_dn6 = assign7560_body31_e6631_d_n6;
            var_q_s0_bulk_dn8 = assign7560_body31_e6631_d_n8;
            var_q_s0_bulk_dn10 = assign7560_body31_e6631_d_n10;
            var_q_s0_bulk_dn11 = assign7560_body31_e6631_d_n11;
            var_q_s0_bulk_dn12 = assign7560_body31_e6631_d_n12;
            var_q_s0_bulk_rv = 0.0;
            let (assign7560_body32_e6642,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body32_e6640: f64 = (var_lp_s0 + 1.0);
        (assign7560_body32_e6640,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign7560_body32_e6642;
            var_lp_s0_rv = 0.0;
        }

        *var_guard84_slot = var_guard84;
        *var_guard84_rv_slot = var_guard84_rv;
        *var_guard85_slot = var_guard85;
        *var_guard85_rv_slot = var_guard85_rv;
        *var_guard86_slot = var_guard86;
        *var_guard86_rv_slot = var_guard86_rv;
        *var_guard87_slot = var_guard87;
        *var_guard87_rv_slot = var_guard87_rv;
        *var_lp_s0_slot = var_lp_s0;
        *var_lp_s0_rv_slot = var_lp_s0_rv;
        *var_phi_b_dep_slot = var_phi_b_dep;
        *var_phi_b_dep_dn0_slot = var_phi_b_dep_dn0;
        *var_phi_b_dep_dn10_slot = var_phi_b_dep_dn10;
        *var_phi_b_dep_dn11_slot = var_phi_b_dep_dn11;
        *var_phi_b_dep_dn12_slot = var_phi_b_dep_dn12;
        *var_phi_b_dep_dn2_slot = var_phi_b_dep_dn2;
        *var_phi_b_dep_dn4_slot = var_phi_b_dep_dn4;
        *var_phi_b_dep_dn5_slot = var_phi_b_dep_dn5;
        *var_phi_b_dep_dn6_slot = var_phi_b_dep_dn6;
        *var_phi_b_dep_dn8_slot = var_phi_b_dep_dn8;
        *var_phi_b_dep_dpsb_slot = var_phi_b_dep_dpsb;
        *var_phi_b_dep_dpsb_dn0_slot = var_phi_b_dep_dpsb_dn0;
        *var_phi_b_dep_dpsb_dn10_slot = var_phi_b_dep_dpsb_dn10;
        *var_phi_b_dep_dpsb_dn11_slot = var_phi_b_dep_dpsb_dn11;
        *var_phi_b_dep_dpsb_dn12_slot = var_phi_b_dep_dpsb_dn12;
        *var_phi_b_dep_dpsb_dn2_slot = var_phi_b_dep_dpsb_dn2;
        *var_phi_b_dep_dpsb_dn4_slot = var_phi_b_dep_dpsb_dn4;
        *var_phi_b_dep_dpsb_dn5_slot = var_phi_b_dep_dpsb_dn5;
        *var_phi_b_dep_dpsb_dn6_slot = var_phi_b_dep_dpsb_dn6;
        *var_phi_b_dep_dpsb_dn8_slot = var_phi_b_dep_dpsb_dn8;
        *var_phi_b_dep_dpsb_rv_slot = var_phi_b_dep_dpsb_rv;
        *var_phi_b_dep_rv_slot = var_phi_b_dep_rv;
        *var_phi_s0_bulk_slot = var_phi_s0_bulk;
        *var_phi_s0_bulk_dn0_slot = var_phi_s0_bulk_dn0;
        *var_phi_s0_bulk_dn10_slot = var_phi_s0_bulk_dn10;
        *var_phi_s0_bulk_dn11_slot = var_phi_s0_bulk_dn11;
        *var_phi_s0_bulk_dn12_slot = var_phi_s0_bulk_dn12;
        *var_phi_s0_bulk_dn2_slot = var_phi_s0_bulk_dn2;
        *var_phi_s0_bulk_dn4_slot = var_phi_s0_bulk_dn4;
        *var_phi_s0_bulk_dn5_slot = var_phi_s0_bulk_dn5;
        *var_phi_s0_bulk_dn6_slot = var_phi_s0_bulk_dn6;
        *var_phi_s0_bulk_dn8_slot = var_phi_s0_bulk_dn8;
        *var_phi_s0_bulk_rv_slot = var_phi_s0_bulk_rv;
        *var_q_s0_bulk_slot = var_q_s0_bulk;
        *var_q_s0_bulk_dn0_slot = var_q_s0_bulk_dn0;
        *var_q_s0_bulk_dn10_slot = var_q_s0_bulk_dn10;
        *var_q_s0_bulk_dn11_slot = var_q_s0_bulk_dn11;
        *var_q_s0_bulk_dn12_slot = var_q_s0_bulk_dn12;
        *var_q_s0_bulk_dn2_slot = var_q_s0_bulk_dn2;
        *var_q_s0_bulk_dn4_slot = var_q_s0_bulk_dn4;
        *var_q_s0_bulk_dn5_slot = var_q_s0_bulk_dn5;
        *var_q_s0_bulk_dn6_slot = var_q_s0_bulk_dn6;
        *var_q_s0_bulk_dn8_slot = var_q_s0_bulk_dn8;
        *var_q_s0_bulk_rv_slot = var_q_s0_bulk_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn12_slot = var_t6_dn12;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_rv_slot = var_t6_rv;
        *var_t7_slot = var_t7;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn12_slot = var_t7_dn12;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_rv_slot = var_t7_rv;
        *var_t8_slot = var_t8;
        *var_t8_dn0_slot = var_t8_dn0;
        *var_t8_dn10_slot = var_t8_dn10;
        *var_t8_dn11_slot = var_t8_dn11;
        *var_t8_dn12_slot = var_t8_dn12;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_rv_slot = var_t8_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_25(
        var_guard74: f64,
        var_guard79: f64,
        var_guard83: f64,
        var_lp_s0_slot: &mut f64,
        var_lp_s0_rv_slot: &mut f64,
    ) {
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
        let mut var_lp_s0_rv: f64 = *var_lp_s0_rv_slot;

        let (assign7570_e6652,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        (0.0,)
    } else {
        (var_lp_s0,)
    }
};
        var_lp_s0 = assign7570_e6652;
        var_lp_s0_rv = 0.0;

        *var_lp_s0_slot = var_lp_s0;
        *var_lp_s0_rv_slot = var_lp_s0_rv;
    }

    pub(super) fn stamp_reactive_block_26(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn4: f64,
        var_c0bulk: f64,
        var_c0bulk_dn0: f64,
        var_c0bulk_dn10: f64,
        var_c0bulk_dn11: f64,
        var_c0bulk_dn12: f64,
        var_c0bulk_dn2: f64,
        var_c0bulk_dn4: f64,
        var_c0bulk_dn5: f64,
        var_c0bulk_dn6: f64,
        var_c0bulk_dn8: f64,
        var_c_box: f64,
        var_cnst0bulk: f64,
        var_cnst0bulk_dn0: f64,
        var_cnst0bulk_dn10: f64,
        var_cnst0bulk_dn11: f64,
        var_cnst0bulk_dn12: f64,
        var_cnst0bulk_dn2: f64,
        var_cnst0bulk_dn4: f64,
        var_cnst0bulk_dn5: f64,
        var_cnst0bulk_dn6: f64,
        var_cnst0bulk_dn8: f64,
        var_cnst1bulk: f64,
        var_cnst1bulk_dn0: f64,
        var_cnst1bulk_dn10: f64,
        var_cnst1bulk_dn11: f64,
        var_cnst1bulk_dn12: f64,
        var_cnst1bulk_dn2: f64,
        var_cnst1bulk_dn4: f64,
        var_cnst1bulk_dn5: f64,
        var_cnst1bulk_dn6: f64,
        var_cnst1bulk_dn8: f64,
        var_guard74: f64,
        var_guard79: f64,
        var_guard83: f64,
        var_lp_s0_max: f64,
        var_phi_s0_soi: f64,
        var_phi_s0_soi_dn0: f64,
        var_phi_s0_soi_dn10: f64,
        var_phi_s0_soi_dn11: f64,
        var_phi_s0_soi_dn12: f64,
        var_phi_s0_soi_dn2: f64,
        var_phi_s0_soi_dn4: f64,
        var_phi_s0_soi_dn5: f64,
        var_phi_s0_soi_dn6: f64,
        var_phi_s0_soi_dn8: f64,
        var_q_fd_soi: f64,
        var_q_fd_soi_dn0: f64,
        var_q_fd_soi_dn10: f64,
        var_q_fd_soi_dn11: f64,
        var_q_fd_soi_dn12: f64,
        var_q_fd_soi_dn2: f64,
        var_q_fd_soi_dn4: f64,
        var_q_fd_soi_dn5: f64,
        var_q_fd_soi_dn6: f64,
        var_q_fd_soi_dn8: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn4: f64,
        var_uc_nsubs_dn5: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn8: f64,
        var_vbsbiz: f64,
        var_vbsbiz_dn0: f64,
        var_vbsbiz_dn10: f64,
        var_vbsbiz_dn11: f64,
        var_vbsbiz_dn12: f64,
        var_vbsbiz_dn2: f64,
        var_vbsbiz_dn4: f64,
        var_vbsbiz_dn5: f64,
        var_vbsbiz_dn6: f64,
        var_vbsbiz_dn8: f64,
        var_guard88_slot: &mut f64,
        var_guard88_rv_slot: &mut f64,
        var_guard89_slot: &mut f64,
        var_guard89_rv_slot: &mut f64,
        var_guard90_slot: &mut f64,
        var_guard90_rv_slot: &mut f64,
        var_guard91_slot: &mut f64,
        var_guard91_rv_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
        var_lp_s0_rv_slot: &mut f64,
        var_phi_b_dep_slot: &mut f64,
        var_phi_b_dep_dn0_slot: &mut f64,
        var_phi_b_dep_dn10_slot: &mut f64,
        var_phi_b_dep_dn11_slot: &mut f64,
        var_phi_b_dep_dn12_slot: &mut f64,
        var_phi_b_dep_dn2_slot: &mut f64,
        var_phi_b_dep_dn4_slot: &mut f64,
        var_phi_b_dep_dn5_slot: &mut f64,
        var_phi_b_dep_dn6_slot: &mut f64,
        var_phi_b_dep_dn8_slot: &mut f64,
        var_phi_b_dep_dpsb_slot: &mut f64,
        var_phi_b_dep_dpsb_dn0_slot: &mut f64,
        var_phi_b_dep_dpsb_dn10_slot: &mut f64,
        var_phi_b_dep_dpsb_dn11_slot: &mut f64,
        var_phi_b_dep_dpsb_dn12_slot: &mut f64,
        var_phi_b_dep_dpsb_dn2_slot: &mut f64,
        var_phi_b_dep_dpsb_dn4_slot: &mut f64,
        var_phi_b_dep_dpsb_dn5_slot: &mut f64,
        var_phi_b_dep_dpsb_dn6_slot: &mut f64,
        var_phi_b_dep_dpsb_dn8_slot: &mut f64,
        var_phi_b_dep_dpsb_rv_slot: &mut f64,
        var_phi_b_dep_rv_slot: &mut f64,
        var_phi_s0_bulk_slot: &mut f64,
        var_phi_s0_bulk_dn0_slot: &mut f64,
        var_phi_s0_bulk_dn10_slot: &mut f64,
        var_phi_s0_bulk_dn11_slot: &mut f64,
        var_phi_s0_bulk_dn12_slot: &mut f64,
        var_phi_s0_bulk_dn2_slot: &mut f64,
        var_phi_s0_bulk_dn4_slot: &mut f64,
        var_phi_s0_bulk_dn5_slot: &mut f64,
        var_phi_s0_bulk_dn6_slot: &mut f64,
        var_phi_s0_bulk_dn8_slot: &mut f64,
        var_phi_s0_bulk_rv_slot: &mut f64,
        var_q_s0_bulk_slot: &mut f64,
        var_q_s0_bulk_dn0_slot: &mut f64,
        var_q_s0_bulk_dn10_slot: &mut f64,
        var_q_s0_bulk_dn11_slot: &mut f64,
        var_q_s0_bulk_dn12_slot: &mut f64,
        var_q_s0_bulk_dn2_slot: &mut f64,
        var_q_s0_bulk_dn4_slot: &mut f64,
        var_q_s0_bulk_dn5_slot: &mut f64,
        var_q_s0_bulk_dn6_slot: &mut f64,
        var_q_s0_bulk_dn8_slot: &mut f64,
        var_q_s0_bulk_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn12_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn12_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn0_slot: &mut f64,
        var_t8_dn10_slot: &mut f64,
        var_t8_dn11_slot: &mut f64,
        var_t8_dn12_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_guard88: f64 = *var_guard88_slot;
        let mut var_guard88_rv: f64 = *var_guard88_rv_slot;
        let mut var_guard89: f64 = *var_guard89_slot;
        let mut var_guard89_rv: f64 = *var_guard89_rv_slot;
        let mut var_guard90: f64 = *var_guard90_slot;
        let mut var_guard90_rv: f64 = *var_guard90_rv_slot;
        let mut var_guard91: f64 = *var_guard91_slot;
        let mut var_guard91_rv: f64 = *var_guard91_rv_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
        let mut var_lp_s0_rv: f64 = *var_lp_s0_rv_slot;
        let mut var_phi_b_dep: f64 = *var_phi_b_dep_slot;
        let mut var_phi_b_dep_dn0: f64 = *var_phi_b_dep_dn0_slot;
        let mut var_phi_b_dep_dn10: f64 = *var_phi_b_dep_dn10_slot;
        let mut var_phi_b_dep_dn11: f64 = *var_phi_b_dep_dn11_slot;
        let mut var_phi_b_dep_dn12: f64 = *var_phi_b_dep_dn12_slot;
        let mut var_phi_b_dep_dn2: f64 = *var_phi_b_dep_dn2_slot;
        let mut var_phi_b_dep_dn4: f64 = *var_phi_b_dep_dn4_slot;
        let mut var_phi_b_dep_dn5: f64 = *var_phi_b_dep_dn5_slot;
        let mut var_phi_b_dep_dn6: f64 = *var_phi_b_dep_dn6_slot;
        let mut var_phi_b_dep_dn8: f64 = *var_phi_b_dep_dn8_slot;
        let mut var_phi_b_dep_dpsb: f64 = *var_phi_b_dep_dpsb_slot;
        let mut var_phi_b_dep_dpsb_dn0: f64 = *var_phi_b_dep_dpsb_dn0_slot;
        let mut var_phi_b_dep_dpsb_dn10: f64 = *var_phi_b_dep_dpsb_dn10_slot;
        let mut var_phi_b_dep_dpsb_dn11: f64 = *var_phi_b_dep_dpsb_dn11_slot;
        let mut var_phi_b_dep_dpsb_dn12: f64 = *var_phi_b_dep_dpsb_dn12_slot;
        let mut var_phi_b_dep_dpsb_dn2: f64 = *var_phi_b_dep_dpsb_dn2_slot;
        let mut var_phi_b_dep_dpsb_dn4: f64 = *var_phi_b_dep_dpsb_dn4_slot;
        let mut var_phi_b_dep_dpsb_dn5: f64 = *var_phi_b_dep_dpsb_dn5_slot;
        let mut var_phi_b_dep_dpsb_dn6: f64 = *var_phi_b_dep_dpsb_dn6_slot;
        let mut var_phi_b_dep_dpsb_dn8: f64 = *var_phi_b_dep_dpsb_dn8_slot;
        let mut var_phi_b_dep_dpsb_rv: f64 = *var_phi_b_dep_dpsb_rv_slot;
        let mut var_phi_b_dep_rv: f64 = *var_phi_b_dep_rv_slot;
        let mut var_phi_s0_bulk: f64 = *var_phi_s0_bulk_slot;
        let mut var_phi_s0_bulk_dn0: f64 = *var_phi_s0_bulk_dn0_slot;
        let mut var_phi_s0_bulk_dn10: f64 = *var_phi_s0_bulk_dn10_slot;
        let mut var_phi_s0_bulk_dn11: f64 = *var_phi_s0_bulk_dn11_slot;
        let mut var_phi_s0_bulk_dn12: f64 = *var_phi_s0_bulk_dn12_slot;
        let mut var_phi_s0_bulk_dn2: f64 = *var_phi_s0_bulk_dn2_slot;
        let mut var_phi_s0_bulk_dn4: f64 = *var_phi_s0_bulk_dn4_slot;
        let mut var_phi_s0_bulk_dn5: f64 = *var_phi_s0_bulk_dn5_slot;
        let mut var_phi_s0_bulk_dn6: f64 = *var_phi_s0_bulk_dn6_slot;
        let mut var_phi_s0_bulk_dn8: f64 = *var_phi_s0_bulk_dn8_slot;
        let mut var_phi_s0_bulk_rv: f64 = *var_phi_s0_bulk_rv_slot;
        let mut var_q_s0_bulk: f64 = *var_q_s0_bulk_slot;
        let mut var_q_s0_bulk_dn0: f64 = *var_q_s0_bulk_dn0_slot;
        let mut var_q_s0_bulk_dn10: f64 = *var_q_s0_bulk_dn10_slot;
        let mut var_q_s0_bulk_dn11: f64 = *var_q_s0_bulk_dn11_slot;
        let mut var_q_s0_bulk_dn12: f64 = *var_q_s0_bulk_dn12_slot;
        let mut var_q_s0_bulk_dn2: f64 = *var_q_s0_bulk_dn2_slot;
        let mut var_q_s0_bulk_dn4: f64 = *var_q_s0_bulk_dn4_slot;
        let mut var_q_s0_bulk_dn5: f64 = *var_q_s0_bulk_dn5_slot;
        let mut var_q_s0_bulk_dn6: f64 = *var_q_s0_bulk_dn6_slot;
        let mut var_q_s0_bulk_dn8: f64 = *var_q_s0_bulk_dn8_slot;
        let mut var_q_s0_bulk_rv: f64 = *var_q_s0_bulk_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn12: f64 = *var_t6_dn12_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn12: f64 = *var_t7_dn12_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn0: f64 = *var_t8_dn0_slot;
        let mut var_t8_dn10: f64 = *var_t8_dn10_slot;
        let mut var_t8_dn11: f64 = *var_t8_dn11_slot;
        let mut var_t8_dn12: f64 = *var_t8_dn12_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_rv: f64 = *var_t8_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let mut assign7580_loop_guard: usize = 0;
        while {
            let assign7580_cond_e6663: f64 = if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) && (var_lp_s0 < var_lp_s0_max)) { 1.0 } else { 0.0 };
            assign7580_cond_e6663 != 0.0
        } {
            assign7580_loop_guard += 1;
            assert!(assign7580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign7580_body0_e6673, assign7580_body0_e6673_d_n0, assign7580_body0_e6673_d_n2, assign7580_body0_e6673_d_n4, assign7580_body0_e6673_d_n5, assign7580_body0_e6673_d_n6, assign7580_body0_e6673_d_n8, assign7580_body0_e6673_d_n10, assign7580_body0_e6673_d_n11, assign7580_body0_e6673_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        (var_cnst0bulk, var_cnst0bulk_dn0, var_cnst0bulk_dn2, var_cnst0bulk_dn4, var_cnst0bulk_dn5, var_cnst0bulk_dn6, var_cnst0bulk_dn8, var_cnst0bulk_dn10, var_cnst0bulk_dn11, var_cnst0bulk_dn12,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
            var_t1 = assign7580_body0_e6673;
            var_t1_dn0 = assign7580_body0_e6673_d_n0;
            var_t1_dn2 = assign7580_body0_e6673_d_n2;
            var_t1_dn4 = assign7580_body0_e6673_d_n4;
            var_t1_dn5 = assign7580_body0_e6673_d_n5;
            var_t1_dn6 = assign7580_body0_e6673_d_n6;
            var_t1_dn8 = assign7580_body0_e6673_d_n8;
            var_t1_dn10 = assign7580_body0_e6673_d_n10;
            var_t1_dn11 = assign7580_body0_e6673_d_n11;
            var_t1_dn12 = assign7580_body0_e6673_d_n12;
            var_t1_rv = 0.0;
            let (assign7580_body1_e6685, assign7580_body1_e6685_d_n0, assign7580_body1_e6685_d_n2, assign7580_body1_e6685_d_n4, assign7580_body1_e6685_d_n5, assign7580_body1_e6685_d_n6, assign7580_body1_e6685_d_n8, assign7580_body1_e6685_d_n10, assign7580_body1_e6685_d_n11, assign7580_body1_e6685_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body1_e6683: f64 = (var_beta * var_phi_s0_bulk);
        (assign7580_body1_e6683, (var_beta * var_phi_s0_bulk_dn0), (var_beta * var_phi_s0_bulk_dn2), ((var_beta_dn4 * var_phi_s0_bulk) + (var_beta * var_phi_s0_bulk_dn4)), (var_beta * var_phi_s0_bulk_dn5), (var_beta * var_phi_s0_bulk_dn6), (var_beta * var_phi_s0_bulk_dn8), (var_beta * var_phi_s0_bulk_dn10), (var_beta * var_phi_s0_bulk_dn11), (var_beta * var_phi_s0_bulk_dn12),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
            var_t2 = assign7580_body1_e6685;
            var_t2_dn0 = assign7580_body1_e6685_d_n0;
            var_t2_dn2 = assign7580_body1_e6685_d_n2;
            var_t2_dn4 = assign7580_body1_e6685_d_n4;
            var_t2_dn5 = assign7580_body1_e6685_d_n5;
            var_t2_dn6 = assign7580_body1_e6685_d_n6;
            var_t2_dn8 = assign7580_body1_e6685_d_n8;
            var_t2_dn10 = assign7580_body1_e6685_d_n10;
            var_t2_dn11 = assign7580_body1_e6685_d_n11;
            var_t2_dn12 = assign7580_body1_e6685_d_n12;
            var_t2_rv = 0.0;
            let (assign7580_body2_e6697, assign7580_body2_e6697_d_n0, assign7580_body2_e6697_d_n2, assign7580_body2_e6697_d_n4, assign7580_body2_e6697_d_n5, assign7580_body2_e6697_d_n6, assign7580_body2_e6697_d_n8, assign7580_body2_e6697_d_n10, assign7580_body2_e6697_d_n11, assign7580_body2_e6697_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body2_e6694: f64 = (-var_t2);
        let assign7580_body2_e6695: f64 = (assign7580_body2_e6694).exp();
        (assign7580_body2_e6695, (assign7580_body2_e6695 * (-var_t2_dn0)), (assign7580_body2_e6695 * (-var_t2_dn2)), (assign7580_body2_e6695 * (-var_t2_dn4)), (assign7580_body2_e6695 * (-var_t2_dn5)), (assign7580_body2_e6695 * (-var_t2_dn6)), (assign7580_body2_e6695 * (-var_t2_dn8)), (assign7580_body2_e6695 * (-var_t2_dn10)), (assign7580_body2_e6695 * (-var_t2_dn11)), (assign7580_body2_e6695 * (-var_t2_dn12)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
            var_t3 = assign7580_body2_e6697;
            var_t3_dn0 = assign7580_body2_e6697_d_n0;
            var_t3_dn2 = assign7580_body2_e6697_d_n2;
            var_t3_dn4 = assign7580_body2_e6697_d_n4;
            var_t3_dn5 = assign7580_body2_e6697_d_n5;
            var_t3_dn6 = assign7580_body2_e6697_d_n6;
            var_t3_dn8 = assign7580_body2_e6697_d_n8;
            var_t3_dn10 = assign7580_body2_e6697_d_n10;
            var_t3_dn11 = assign7580_body2_e6697_d_n11;
            var_t3_dn12 = assign7580_body2_e6697_d_n12;
            var_t3_rv = 0.0;
            let assign7580_body3_e6700: f64 = if var_phi_s0_bulk > 1e-8 { 1.0 } else { 0.0 };
            var_guard88 = assign7580_body3_e6700;
            var_guard88_rv = 0.0;
            let (assign7580_body4_e6715, assign7580_body4_e6715_d_n0, assign7580_body4_e6715_d_n2, assign7580_body4_e6715_d_n4, assign7580_body4_e6715_d_n5, assign7580_body4_e6715_d_n6, assign7580_body4_e6715_d_n8, assign7580_body4_e6715_d_n10, assign7580_body4_e6715_d_n11, assign7580_body4_e6715_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) && (var_guard88 != 0.0)) {
        let assign7580_body4_e6712: f64 = (var_beta * var_phi_s0_bulk);
        let assign7580_body4_e6713: f64 = (assign7580_body4_e6712).exp();
        (assign7580_body4_e6713, (assign7580_body4_e6713 * (var_beta * var_phi_s0_bulk_dn0)), (assign7580_body4_e6713 * (var_beta * var_phi_s0_bulk_dn2)), (assign7580_body4_e6713 * ((var_beta_dn4 * var_phi_s0_bulk) + (var_beta * var_phi_s0_bulk_dn4))), (assign7580_body4_e6713 * (var_beta * var_phi_s0_bulk_dn5)), (assign7580_body4_e6713 * (var_beta * var_phi_s0_bulk_dn6)), (assign7580_body4_e6713 * (var_beta * var_phi_s0_bulk_dn8)), (assign7580_body4_e6713 * (var_beta * var_phi_s0_bulk_dn10)), (assign7580_body4_e6713 * (var_beta * var_phi_s0_bulk_dn11)), (assign7580_body4_e6713 * (var_beta * var_phi_s0_bulk_dn12)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
            var_t0 = assign7580_body4_e6715;
            var_t0_dn0 = assign7580_body4_e6715_d_n0;
            var_t0_dn2 = assign7580_body4_e6715_d_n2;
            var_t0_dn4 = assign7580_body4_e6715_d_n4;
            var_t0_dn5 = assign7580_body4_e6715_d_n5;
            var_t0_dn6 = assign7580_body4_e6715_d_n6;
            var_t0_dn8 = assign7580_body4_e6715_d_n8;
            var_t0_dn10 = assign7580_body4_e6715_d_n10;
            var_t0_dn11 = assign7580_body4_e6715_d_n11;
            var_t0_dn12 = assign7580_body4_e6715_d_n12;
            var_t0_rv = 0.0;
            let (assign7580_body5_e6741, assign7580_body5_e6741_d_n0, assign7580_body5_e6741_d_n2, assign7580_body5_e6741_d_n4, assign7580_body5_e6741_d_n5, assign7580_body5_e6741_d_n6, assign7580_body5_e6741_d_n8, assign7580_body5_e6741_d_n10, assign7580_body5_e6741_d_n11, assign7580_body5_e6741_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) && (var_guard88 != 0.0)) {
        let assign7580_body5_e6726: f64 = (-var_t1);
        let assign7580_body5_e6729: f64 = (var_t3 + var_t2);
        let assign7580_body5_e6731: f64 = (assign7580_body5_e6729 - 1.0);
        let assign7580_body5_e6735: f64 = (var_t0 - 1.0);
        let assign7580_body5_e6736: f64 = (var_cnst1bulk * assign7580_body5_e6735);
        let assign7580_body5_e6737: f64 = (assign7580_body5_e6731 + assign7580_body5_e6736);
        let assign7580_body5_e6738: f64 = (assign7580_body5_e6737).sqrt();
        let assign7580_body5_e6739: f64 = (assign7580_body5_e6726 * assign7580_body5_e6738);
        (assign7580_body5_e6739, (((-var_t1_dn0) * assign7580_body5_e6738) + (assign7580_body5_e6726 * (((var_t3_dn0 + var_t2_dn0) + ((var_cnst1bulk_dn0 * assign7580_body5_e6735) + (var_cnst1bulk * var_t0_dn0))) / (2.0 * assign7580_body5_e6738)))), (((-var_t1_dn2) * assign7580_body5_e6738) + (assign7580_body5_e6726 * (((var_t3_dn2 + var_t2_dn2) + ((var_cnst1bulk_dn2 * assign7580_body5_e6735) + (var_cnst1bulk * var_t0_dn2))) / (2.0 * assign7580_body5_e6738)))), (((-var_t1_dn4) * assign7580_body5_e6738) + (assign7580_body5_e6726 * (((var_t3_dn4 + var_t2_dn4) + ((var_cnst1bulk_dn4 * assign7580_body5_e6735) + (var_cnst1bulk * var_t0_dn4))) / (2.0 * assign7580_body5_e6738)))), (((-var_t1_dn5) * assign7580_body5_e6738) + (assign7580_body5_e6726 * (((var_t3_dn5 + var_t2_dn5) + ((var_cnst1bulk_dn5 * assign7580_body5_e6735) + (var_cnst1bulk * var_t0_dn5))) / (2.0 * assign7580_body5_e6738)))), (((-var_t1_dn6) * assign7580_body5_e6738) + (assign7580_body5_e6726 * (((var_t3_dn6 + var_t2_dn6) + ((var_cnst1bulk_dn6 * assign7580_body5_e6735) + (var_cnst1bulk * var_t0_dn6))) / (2.0 * assign7580_body5_e6738)))), (((-var_t1_dn8) * assign7580_body5_e6738) + (assign7580_body5_e6726 * (((var_t3_dn8 + var_t2_dn8) + ((var_cnst1bulk_dn8 * assign7580_body5_e6735) + (var_cnst1bulk * var_t0_dn8))) / (2.0 * assign7580_body5_e6738)))), (((-var_t1_dn10) * assign7580_body5_e6738) + (assign7580_body5_e6726 * (((var_t3_dn10 + var_t2_dn10) + ((var_cnst1bulk_dn10 * assign7580_body5_e6735) + (var_cnst1bulk * var_t0_dn10))) / (2.0 * assign7580_body5_e6738)))), (((-var_t1_dn11) * assign7580_body5_e6738) + (assign7580_body5_e6726 * (((var_t3_dn11 + var_t2_dn11) + ((var_cnst1bulk_dn11 * assign7580_body5_e6735) + (var_cnst1bulk * var_t0_dn11))) / (2.0 * assign7580_body5_e6738)))), (((-var_t1_dn12) * assign7580_body5_e6738) + (assign7580_body5_e6726 * (((var_t3_dn12 + var_t2_dn12) + ((var_cnst1bulk_dn12 * assign7580_body5_e6735) + (var_cnst1bulk * var_t0_dn12))) / (2.0 * assign7580_body5_e6738)))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign7580_body5_e6741;
            var_t4_dn0 = assign7580_body5_e6741_d_n0;
            var_t4_dn2 = assign7580_body5_e6741_d_n2;
            var_t4_dn4 = assign7580_body5_e6741_d_n4;
            var_t4_dn5 = assign7580_body5_e6741_d_n5;
            var_t4_dn6 = assign7580_body5_e6741_d_n6;
            var_t4_dn8 = assign7580_body5_e6741_d_n8;
            var_t4_dn10 = assign7580_body5_e6741_d_n10;
            var_t4_dn11 = assign7580_body5_e6741_d_n11;
            var_t4_dn12 = assign7580_body5_e6741_d_n12;
            var_t4_rv = 0.0;
            let (assign7580_body6_e6764, assign7580_body6_e6764_d_n0, assign7580_body6_e6764_d_n2, assign7580_body6_e6764_d_n4, assign7580_body6_e6764_d_n5, assign7580_body6_e6764_d_n6, assign7580_body6_e6764_d_n8, assign7580_body6_e6764_d_n10, assign7580_body6_e6764_d_n11, assign7580_body6_e6764_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) && (var_guard88 != 0.0)) {
        let assign7580_body6_e6753: f64 = (var_c0bulk / var_t4);
        let assign7580_body6_e6755: f64 = (-var_t3);
        let assign7580_body6_e6757: f64 = (assign7580_body6_e6755 + 1.0);
        let assign7580_body6_e6760: f64 = (var_cnst1bulk * var_t0);
        let assign7580_body6_e6761: f64 = (assign7580_body6_e6757 + assign7580_body6_e6760);
        let assign7580_body6_e6762: f64 = (assign7580_body6_e6753 * assign7580_body6_e6761);
        (assign7580_body6_e6762, (((((var_c0bulk_dn0 * var_t4) - (var_c0bulk * var_t4_dn0)) / (var_t4 * var_t4)) * assign7580_body6_e6761) + (assign7580_body6_e6753 * ((-var_t3_dn0) + ((var_cnst1bulk_dn0 * var_t0) + (var_cnst1bulk * var_t0_dn0))))), (((((var_c0bulk_dn2 * var_t4) - (var_c0bulk * var_t4_dn2)) / (var_t4 * var_t4)) * assign7580_body6_e6761) + (assign7580_body6_e6753 * ((-var_t3_dn2) + ((var_cnst1bulk_dn2 * var_t0) + (var_cnst1bulk * var_t0_dn2))))), (((((var_c0bulk_dn4 * var_t4) - (var_c0bulk * var_t4_dn4)) / (var_t4 * var_t4)) * assign7580_body6_e6761) + (assign7580_body6_e6753 * ((-var_t3_dn4) + ((var_cnst1bulk_dn4 * var_t0) + (var_cnst1bulk * var_t0_dn4))))), (((((var_c0bulk_dn5 * var_t4) - (var_c0bulk * var_t4_dn5)) / (var_t4 * var_t4)) * assign7580_body6_e6761) + (assign7580_body6_e6753 * ((-var_t3_dn5) + ((var_cnst1bulk_dn5 * var_t0) + (var_cnst1bulk * var_t0_dn5))))), (((((var_c0bulk_dn6 * var_t4) - (var_c0bulk * var_t4_dn6)) / (var_t4 * var_t4)) * assign7580_body6_e6761) + (assign7580_body6_e6753 * ((-var_t3_dn6) + ((var_cnst1bulk_dn6 * var_t0) + (var_cnst1bulk * var_t0_dn6))))), (((((var_c0bulk_dn8 * var_t4) - (var_c0bulk * var_t4_dn8)) / (var_t4 * var_t4)) * assign7580_body6_e6761) + (assign7580_body6_e6753 * ((-var_t3_dn8) + ((var_cnst1bulk_dn8 * var_t0) + (var_cnst1bulk * var_t0_dn8))))), (((((var_c0bulk_dn10 * var_t4) - (var_c0bulk * var_t4_dn10)) / (var_t4 * var_t4)) * assign7580_body6_e6761) + (assign7580_body6_e6753 * ((-var_t3_dn10) + ((var_cnst1bulk_dn10 * var_t0) + (var_cnst1bulk * var_t0_dn10))))), (((((var_c0bulk_dn11 * var_t4) - (var_c0bulk * var_t4_dn11)) / (var_t4 * var_t4)) * assign7580_body6_e6761) + (assign7580_body6_e6753 * ((-var_t3_dn11) + ((var_cnst1bulk_dn11 * var_t0) + (var_cnst1bulk * var_t0_dn11))))), (((((var_c0bulk_dn12 * var_t4) - (var_c0bulk * var_t4_dn12)) / (var_t4 * var_t4)) * assign7580_body6_e6761) + (assign7580_body6_e6753 * ((-var_t3_dn12) + ((var_cnst1bulk_dn12 * var_t0) + (var_cnst1bulk * var_t0_dn12))))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign7580_body6_e6764;
            var_t5_dn0 = assign7580_body6_e6764_d_n0;
            var_t5_dn2 = assign7580_body6_e6764_d_n2;
            var_t5_dn4 = assign7580_body6_e6764_d_n4;
            var_t5_dn5 = assign7580_body6_e6764_d_n5;
            var_t5_dn6 = assign7580_body6_e6764_d_n6;
            var_t5_dn8 = assign7580_body6_e6764_d_n8;
            var_t5_dn10 = assign7580_body6_e6764_d_n10;
            var_t5_dn11 = assign7580_body6_e6764_d_n11;
            var_t5_dn12 = assign7580_body6_e6764_d_n12;
            var_t5_rv = 0.0;
            let assign7580_body7_e6767: f64 = (-1e-8);
            let assign7580_body7_e6768: f64 = if var_phi_s0_bulk < assign7580_body7_e6767 { 1.0 } else { 0.0 };
            var_guard89 = assign7580_body7_e6768;
            var_guard89_rv = 0.0;
            let (assign7580_body8_e6790, assign7580_body8_e6790_d_n0, assign7580_body8_e6790_d_n2, assign7580_body8_e6790_d_n4, assign7580_body8_e6790_d_n5, assign7580_body8_e6790_d_n6, assign7580_body8_e6790_d_n8, assign7580_body8_e6790_d_n10, assign7580_body8_e6790_d_n11, assign7580_body8_e6790_d_n12,) = {
    if (((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) && (var_guard88 == 0.0)) && (var_guard89 != 0.0)) {
        let assign7580_body8_e6784: f64 = (var_t3 + var_t2);
        let assign7580_body8_e6786: f64 = (assign7580_body8_e6784 - 1.0);
        let assign7580_body8_e6787: f64 = (assign7580_body8_e6786).sqrt();
        let assign7580_body8_e6788: f64 = (var_t1 * assign7580_body8_e6787);
        (assign7580_body8_e6788, ((var_t1_dn0 * assign7580_body8_e6787) + (var_t1 * ((var_t3_dn0 + var_t2_dn0) / (2.0 * assign7580_body8_e6787)))), ((var_t1_dn2 * assign7580_body8_e6787) + (var_t1 * ((var_t3_dn2 + var_t2_dn2) / (2.0 * assign7580_body8_e6787)))), ((var_t1_dn4 * assign7580_body8_e6787) + (var_t1 * ((var_t3_dn4 + var_t2_dn4) / (2.0 * assign7580_body8_e6787)))), ((var_t1_dn5 * assign7580_body8_e6787) + (var_t1 * ((var_t3_dn5 + var_t2_dn5) / (2.0 * assign7580_body8_e6787)))), ((var_t1_dn6 * assign7580_body8_e6787) + (var_t1 * ((var_t3_dn6 + var_t2_dn6) / (2.0 * assign7580_body8_e6787)))), ((var_t1_dn8 * assign7580_body8_e6787) + (var_t1 * ((var_t3_dn8 + var_t2_dn8) / (2.0 * assign7580_body8_e6787)))), ((var_t1_dn10 * assign7580_body8_e6787) + (var_t1 * ((var_t3_dn10 + var_t2_dn10) / (2.0 * assign7580_body8_e6787)))), ((var_t1_dn11 * assign7580_body8_e6787) + (var_t1 * ((var_t3_dn11 + var_t2_dn11) / (2.0 * assign7580_body8_e6787)))), ((var_t1_dn12 * assign7580_body8_e6787) + (var_t1 * ((var_t3_dn12 + var_t2_dn12) / (2.0 * assign7580_body8_e6787)))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign7580_body8_e6790;
            var_t4_dn0 = assign7580_body8_e6790_d_n0;
            var_t4_dn2 = assign7580_body8_e6790_d_n2;
            var_t4_dn4 = assign7580_body8_e6790_d_n4;
            var_t4_dn5 = assign7580_body8_e6790_d_n5;
            var_t4_dn6 = assign7580_body8_e6790_d_n6;
            var_t4_dn8 = assign7580_body8_e6790_d_n8;
            var_t4_dn10 = assign7580_body8_e6790_d_n10;
            var_t4_dn11 = assign7580_body8_e6790_d_n11;
            var_t4_dn12 = assign7580_body8_e6790_d_n12;
            var_t4_rv = 0.0;
            let (assign7580_body9_e6812, assign7580_body9_e6812_d_n0, assign7580_body9_e6812_d_n2, assign7580_body9_e6812_d_n4, assign7580_body9_e6812_d_n5, assign7580_body9_e6812_d_n6, assign7580_body9_e6812_d_n8, assign7580_body9_e6812_d_n10, assign7580_body9_e6812_d_n11, assign7580_body9_e6812_d_n12,) = {
    if (((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) && (var_guard88 == 0.0)) && (var_guard89 != 0.0)) {
        let assign7580_body9_e6805: f64 = (var_c0bulk / var_t4);
        let assign7580_body9_e6807: f64 = (-var_t3);
        let assign7580_body9_e6809: f64 = (assign7580_body9_e6807 + 1.0);
        let assign7580_body9_e6810: f64 = (assign7580_body9_e6805 * assign7580_body9_e6809);
        (assign7580_body9_e6810, (((((var_c0bulk_dn0 * var_t4) - (var_c0bulk * var_t4_dn0)) / (var_t4 * var_t4)) * assign7580_body9_e6809) + (assign7580_body9_e6805 * (-var_t3_dn0))), (((((var_c0bulk_dn2 * var_t4) - (var_c0bulk * var_t4_dn2)) / (var_t4 * var_t4)) * assign7580_body9_e6809) + (assign7580_body9_e6805 * (-var_t3_dn2))), (((((var_c0bulk_dn4 * var_t4) - (var_c0bulk * var_t4_dn4)) / (var_t4 * var_t4)) * assign7580_body9_e6809) + (assign7580_body9_e6805 * (-var_t3_dn4))), (((((var_c0bulk_dn5 * var_t4) - (var_c0bulk * var_t4_dn5)) / (var_t4 * var_t4)) * assign7580_body9_e6809) + (assign7580_body9_e6805 * (-var_t3_dn5))), (((((var_c0bulk_dn6 * var_t4) - (var_c0bulk * var_t4_dn6)) / (var_t4 * var_t4)) * assign7580_body9_e6809) + (assign7580_body9_e6805 * (-var_t3_dn6))), (((((var_c0bulk_dn8 * var_t4) - (var_c0bulk * var_t4_dn8)) / (var_t4 * var_t4)) * assign7580_body9_e6809) + (assign7580_body9_e6805 * (-var_t3_dn8))), (((((var_c0bulk_dn10 * var_t4) - (var_c0bulk * var_t4_dn10)) / (var_t4 * var_t4)) * assign7580_body9_e6809) + (assign7580_body9_e6805 * (-var_t3_dn10))), (((((var_c0bulk_dn11 * var_t4) - (var_c0bulk * var_t4_dn11)) / (var_t4 * var_t4)) * assign7580_body9_e6809) + (assign7580_body9_e6805 * (-var_t3_dn11))), (((((var_c0bulk_dn12 * var_t4) - (var_c0bulk * var_t4_dn12)) / (var_t4 * var_t4)) * assign7580_body9_e6809) + (assign7580_body9_e6805 * (-var_t3_dn12))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign7580_body9_e6812;
            var_t5_dn0 = assign7580_body9_e6812_d_n0;
            var_t5_dn2 = assign7580_body9_e6812_d_n2;
            var_t5_dn4 = assign7580_body9_e6812_d_n4;
            var_t5_dn5 = assign7580_body9_e6812_d_n5;
            var_t5_dn6 = assign7580_body9_e6812_d_n6;
            var_t5_dn8 = assign7580_body9_e6812_d_n8;
            var_t5_dn10 = assign7580_body9_e6812_d_n10;
            var_t5_dn11 = assign7580_body9_e6812_d_n11;
            var_t5_dn12 = assign7580_body9_e6812_d_n12;
            var_t5_rv = 0.0;
            let (assign7580_body10_e6836, assign7580_body10_e6836_d_n0, assign7580_body10_e6836_d_n2, assign7580_body10_e6836_d_n4, assign7580_body10_e6836_d_n5, assign7580_body10_e6836_d_n6, assign7580_body10_e6836_d_n8, assign7580_body10_e6836_d_n10, assign7580_body10_e6836_d_n11, assign7580_body10_e6836_d_n12,) = {
    if (((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) && (var_guard88 == 0.0)) && (var_guard89 == 0.0)) {
        let assign7580_body10_e6828: f64 = (var_c0bulk / var_beta);
        let assign7580_body10_e6829: f64 = (assign7580_body10_e6828).sqrt();
        let assign7580_body10_e6830: f64 = (-assign7580_body10_e6829);
        let assign7580_body10_e6832: f64 = (assign7580_body10_e6830 * var_beta);
        let assign7580_body10_e6834: f64 = (assign7580_body10_e6832 * var_phi_s0_bulk);
        (assign7580_body10_e6834, ((((-((var_c0bulk_dn0 / var_beta) / (2.0 * assign7580_body10_e6829))) * var_beta) * var_phi_s0_bulk) + (assign7580_body10_e6832 * var_phi_s0_bulk_dn0)), ((((-((var_c0bulk_dn2 / var_beta) / (2.0 * assign7580_body10_e6829))) * var_beta) * var_phi_s0_bulk) + (assign7580_body10_e6832 * var_phi_s0_bulk_dn2)), (((((-((((var_c0bulk_dn4 * var_beta) - (var_c0bulk * var_beta_dn4)) / (var_beta * var_beta)) / (2.0 * assign7580_body10_e6829))) * var_beta) + (assign7580_body10_e6830 * var_beta_dn4)) * var_phi_s0_bulk) + (assign7580_body10_e6832 * var_phi_s0_bulk_dn4)), ((((-((var_c0bulk_dn5 / var_beta) / (2.0 * assign7580_body10_e6829))) * var_beta) * var_phi_s0_bulk) + (assign7580_body10_e6832 * var_phi_s0_bulk_dn5)), ((((-((var_c0bulk_dn6 / var_beta) / (2.0 * assign7580_body10_e6829))) * var_beta) * var_phi_s0_bulk) + (assign7580_body10_e6832 * var_phi_s0_bulk_dn6)), ((((-((var_c0bulk_dn8 / var_beta) / (2.0 * assign7580_body10_e6829))) * var_beta) * var_phi_s0_bulk) + (assign7580_body10_e6832 * var_phi_s0_bulk_dn8)), ((((-((var_c0bulk_dn10 / var_beta) / (2.0 * assign7580_body10_e6829))) * var_beta) * var_phi_s0_bulk) + (assign7580_body10_e6832 * var_phi_s0_bulk_dn10)), ((((-((var_c0bulk_dn11 / var_beta) / (2.0 * assign7580_body10_e6829))) * var_beta) * var_phi_s0_bulk) + (assign7580_body10_e6832 * var_phi_s0_bulk_dn11)), ((((-((var_c0bulk_dn12 / var_beta) / (2.0 * assign7580_body10_e6829))) * var_beta) * var_phi_s0_bulk) + (assign7580_body10_e6832 * var_phi_s0_bulk_dn12)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign7580_body10_e6836;
            var_t4_dn0 = assign7580_body10_e6836_d_n0;
            var_t4_dn2 = assign7580_body10_e6836_d_n2;
            var_t4_dn4 = assign7580_body10_e6836_d_n4;
            var_t4_dn5 = assign7580_body10_e6836_d_n5;
            var_t4_dn6 = assign7580_body10_e6836_d_n6;
            var_t4_dn8 = assign7580_body10_e6836_d_n8;
            var_t4_dn10 = assign7580_body10_e6836_d_n10;
            var_t4_dn11 = assign7580_body10_e6836_d_n11;
            var_t4_dn12 = assign7580_body10_e6836_d_n12;
            var_t4_rv = 0.0;
            let (assign7580_body11_e6856, assign7580_body11_e6856_d_n0, assign7580_body11_e6856_d_n2, assign7580_body11_e6856_d_n4, assign7580_body11_e6856_d_n5, assign7580_body11_e6856_d_n6, assign7580_body11_e6856_d_n8, assign7580_body11_e6856_d_n10, assign7580_body11_e6856_d_n11, assign7580_body11_e6856_d_n12,) = {
    if (((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) && (var_guard88 == 0.0)) && (var_guard89 == 0.0)) {
        let assign7580_body11_e6852: f64 = (var_c0bulk * var_beta);
        let assign7580_body11_e6853: f64 = (assign7580_body11_e6852).sqrt();
        let assign7580_body11_e6854: f64 = (-assign7580_body11_e6853);
        (assign7580_body11_e6854, (-((var_c0bulk_dn0 * var_beta) / (2.0 * assign7580_body11_e6853))), (-((var_c0bulk_dn2 * var_beta) / (2.0 * assign7580_body11_e6853))), (-(((var_c0bulk_dn4 * var_beta) + (var_c0bulk * var_beta_dn4)) / (2.0 * assign7580_body11_e6853))), (-((var_c0bulk_dn5 * var_beta) / (2.0 * assign7580_body11_e6853))), (-((var_c0bulk_dn6 * var_beta) / (2.0 * assign7580_body11_e6853))), (-((var_c0bulk_dn8 * var_beta) / (2.0 * assign7580_body11_e6853))), (-((var_c0bulk_dn10 * var_beta) / (2.0 * assign7580_body11_e6853))), (-((var_c0bulk_dn11 * var_beta) / (2.0 * assign7580_body11_e6853))), (-((var_c0bulk_dn12 * var_beta) / (2.0 * assign7580_body11_e6853))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign7580_body11_e6856;
            var_t5_dn0 = assign7580_body11_e6856_d_n0;
            var_t5_dn2 = assign7580_body11_e6856_d_n2;
            var_t5_dn4 = assign7580_body11_e6856_d_n4;
            var_t5_dn5 = assign7580_body11_e6856_d_n5;
            var_t5_dn6 = assign7580_body11_e6856_d_n6;
            var_t5_dn8 = assign7580_body11_e6856_d_n8;
            var_t5_dn10 = assign7580_body11_e6856_d_n10;
            var_t5_dn11 = assign7580_body11_e6856_d_n11;
            var_t5_dn12 = assign7580_body11_e6856_d_n12;
            var_t5_rv = 0.0;
            let (assign7580_body12_e6875, assign7580_body12_e6875_d_n0, assign7580_body12_e6875_d_n2, assign7580_body12_e6875_d_n4, assign7580_body12_e6875_d_n5, assign7580_body12_e6875_d_n6, assign7580_body12_e6875_d_n8, assign7580_body12_e6875_d_n10, assign7580_body12_e6875_d_n11, assign7580_body12_e6875_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body12_e6866: f64 = (var_t4 * var_t4);
        let assign7580_body12_e6869: f64 = (4.0 * 1e-10);
        let assign7580_body12_e6871: f64 = (assign7580_body12_e6869 * 1e-10);
        let assign7580_body12_e6872: f64 = (assign7580_body12_e6866 + assign7580_body12_e6871);
        let assign7580_body12_e6873: f64 = (assign7580_body12_e6872).sqrt();
        (assign7580_body12_e6873, (((var_t4_dn0 * var_t4) + (var_t4 * var_t4_dn0)) / (2.0 * assign7580_body12_e6873)), (((var_t4_dn2 * var_t4) + (var_t4 * var_t4_dn2)) / (2.0 * assign7580_body12_e6873)), (((var_t4_dn4 * var_t4) + (var_t4 * var_t4_dn4)) / (2.0 * assign7580_body12_e6873)), (((var_t4_dn5 * var_t4) + (var_t4 * var_t4_dn5)) / (2.0 * assign7580_body12_e6873)), (((var_t4_dn6 * var_t4) + (var_t4 * var_t4_dn6)) / (2.0 * assign7580_body12_e6873)), (((var_t4_dn8 * var_t4) + (var_t4 * var_t4_dn8)) / (2.0 * assign7580_body12_e6873)), (((var_t4_dn10 * var_t4) + (var_t4 * var_t4_dn10)) / (2.0 * assign7580_body12_e6873)), (((var_t4_dn11 * var_t4) + (var_t4 * var_t4_dn11)) / (2.0 * assign7580_body12_e6873)), (((var_t4_dn12 * var_t4) + (var_t4 * var_t4_dn12)) / (2.0 * assign7580_body12_e6873)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
            var_tmf2 = assign7580_body12_e6875;
            var_tmf2_dn0 = assign7580_body12_e6875_d_n0;
            var_tmf2_dn2 = assign7580_body12_e6875_d_n2;
            var_tmf2_dn4 = assign7580_body12_e6875_d_n4;
            var_tmf2_dn5 = assign7580_body12_e6875_d_n5;
            var_tmf2_dn6 = assign7580_body12_e6875_d_n6;
            var_tmf2_dn8 = assign7580_body12_e6875_d_n8;
            var_tmf2_dn10 = assign7580_body12_e6875_d_n10;
            var_tmf2_dn11 = assign7580_body12_e6875_d_n11;
            var_tmf2_dn12 = assign7580_body12_e6875_d_n12;
            var_tmf2_rv = 0.0;
            let (assign7580_body13_e6891, assign7580_body13_e6891_d_n0, assign7580_body13_e6891_d_n2, assign7580_body13_e6891_d_n4, assign7580_body13_e6891_d_n5, assign7580_body13_e6891_d_n6, assign7580_body13_e6891_d_n8, assign7580_body13_e6891_d_n10, assign7580_body13_e6891_d_n11, assign7580_body13_e6891_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body13_e6887: f64 = (var_t4 / var_tmf2);
        let assign7580_body13_e6888: f64 = (1.0 + assign7580_body13_e6887);
        let assign7580_body13_e6889: f64 = (0.5 * assign7580_body13_e6888);
        (assign7580_body13_e6889, (0.5 * (((var_t4_dn0 * var_tmf2) - (var_t4 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn2 * var_tmf2) - (var_t4 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn4 * var_tmf2) - (var_t4 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn5 * var_tmf2) - (var_t4 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn6 * var_tmf2) - (var_t4 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn8 * var_tmf2) - (var_t4 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn10 * var_tmf2) - (var_t4 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn11 * var_tmf2) - (var_t4 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t4_dn12 * var_tmf2) - (var_t4 * var_tmf2_dn12)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
            var_t7 = assign7580_body13_e6891;
            var_t7_dn0 = assign7580_body13_e6891_d_n0;
            var_t7_dn2 = assign7580_body13_e6891_d_n2;
            var_t7_dn4 = assign7580_body13_e6891_d_n4;
            var_t7_dn5 = assign7580_body13_e6891_d_n5;
            var_t7_dn6 = assign7580_body13_e6891_d_n6;
            var_t7_dn8 = assign7580_body13_e6891_d_n8;
            var_t7_dn10 = assign7580_body13_e6891_d_n10;
            var_t7_dn11 = assign7580_body13_e6891_d_n11;
            var_t7_dn12 = assign7580_body13_e6891_d_n12;
            var_t7_rv = 0.0;
            let (assign7580_body14_e6909, assign7580_body14_e6909_d_n0, assign7580_body14_e6909_d_n2, assign7580_body14_e6909_d_n4, assign7580_body14_e6909_d_n5, assign7580_body14_e6909_d_n6, assign7580_body14_e6909_d_n8, assign7580_body14_e6909_d_n10, assign7580_body14_e6909_d_n11, assign7580_body14_e6909_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body14_e6902: f64 = (var_t4 + var_tmf2);
        let assign7580_body14_e6903: f64 = (0.5 * assign7580_body14_e6902);
        let assign7580_body14_e6906: f64 = (1e-10 * 1e-10);
        let assign7580_body14_e6907: f64 = (assign7580_body14_e6903 + assign7580_body14_e6906);
        (assign7580_body14_e6907, (0.5 * (var_t4_dn0 + var_tmf2_dn0)), (0.5 * (var_t4_dn2 + var_tmf2_dn2)), (0.5 * (var_t4_dn4 + var_tmf2_dn4)), (0.5 * (var_t4_dn5 + var_tmf2_dn5)), (0.5 * (var_t4_dn6 + var_tmf2_dn6)), (0.5 * (var_t4_dn8 + var_tmf2_dn8)), (0.5 * (var_t4_dn10 + var_tmf2_dn10)), (0.5 * (var_t4_dn11 + var_tmf2_dn11)), (0.5 * (var_t4_dn12 + var_tmf2_dn12)),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
            var_t6 = assign7580_body14_e6909;
            var_t6_dn0 = assign7580_body14_e6909_d_n0;
            var_t6_dn2 = assign7580_body14_e6909_d_n2;
            var_t6_dn4 = assign7580_body14_e6909_d_n4;
            var_t6_dn5 = assign7580_body14_e6909_d_n5;
            var_t6_dn6 = assign7580_body14_e6909_d_n6;
            var_t6_dn8 = assign7580_body14_e6909_d_n8;
            var_t6_dn10 = assign7580_body14_e6909_d_n10;
            var_t6_dn11 = assign7580_body14_e6909_d_n11;
            var_t6_dn12 = assign7580_body14_e6909_d_n12;
            var_t6_rv = 0.0;
            let assign7580_body15_e6912: f64 = if var_t6 < 0.0 { 1.0 } else { 0.0 };
            var_guard90 = assign7580_body15_e6912;
            var_guard90_rv = 0.0;
            let (assign7580_body16_e6924, assign7580_body16_e6924_d_n0, assign7580_body16_e6924_d_n2, assign7580_body16_e6924_d_n4, assign7580_body16_e6924_d_n5, assign7580_body16_e6924_d_n6, assign7580_body16_e6924_d_n8, assign7580_body16_e6924_d_n10, assign7580_body16_e6924_d_n11, assign7580_body16_e6924_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) && (var_guard90 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
            var_t6 = assign7580_body16_e6924;
            var_t6_dn0 = assign7580_body16_e6924_d_n0;
            var_t6_dn2 = assign7580_body16_e6924_d_n2;
            var_t6_dn4 = assign7580_body16_e6924_d_n4;
            var_t6_dn5 = assign7580_body16_e6924_d_n5;
            var_t6_dn6 = assign7580_body16_e6924_d_n6;
            var_t6_dn8 = assign7580_body16_e6924_d_n8;
            var_t6_dn10 = assign7580_body16_e6924_d_n10;
            var_t6_dn11 = assign7580_body16_e6924_d_n11;
            var_t6_dn12 = assign7580_body16_e6924_d_n12;
            var_t6_rv = 0.0;
            let (assign7580_body17_e6936, assign7580_body17_e6936_d_n0, assign7580_body17_e6936_d_n2, assign7580_body17_e6936_d_n4, assign7580_body17_e6936_d_n5, assign7580_body17_e6936_d_n6, assign7580_body17_e6936_d_n8, assign7580_body17_e6936_d_n10, assign7580_body17_e6936_d_n11, assign7580_body17_e6936_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) && (var_guard90 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
            var_t7 = assign7580_body17_e6936;
            var_t7_dn0 = assign7580_body17_e6936_d_n0;
            var_t7_dn2 = assign7580_body17_e6936_d_n2;
            var_t7_dn4 = assign7580_body17_e6936_d_n4;
            var_t7_dn5 = assign7580_body17_e6936_d_n5;
            var_t7_dn6 = assign7580_body17_e6936_d_n6;
            var_t7_dn8 = assign7580_body17_e6936_d_n8;
            var_t7_dn10 = assign7580_body17_e6936_d_n10;
            var_t7_dn11 = assign7580_body17_e6936_d_n11;
            var_t7_dn12 = assign7580_body17_e6936_d_n12;
            var_t7_rv = 0.0;
            let (assign7580_body18_e6951, assign7580_body18_e6951_d_n0, assign7580_body18_e6951_d_n2, assign7580_body18_e6951_d_n4, assign7580_body18_e6951_d_n5, assign7580_body18_e6951_d_n6, assign7580_body18_e6951_d_n8, assign7580_body18_e6951_d_n10, assign7580_body18_e6951_d_n11, assign7580_body18_e6951_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body18_e6945: f64 = (-var_q_fd_soi);
        let assign7580_body18_e6947: f64 = (assign7580_body18_e6945 - var_t6);
        let assign7580_body18_e6949: f64 = (assign7580_body18_e6947 - 1e-13);
        (assign7580_body18_e6949, ((-var_q_fd_soi_dn0) - var_t6_dn0), ((-var_q_fd_soi_dn2) - var_t6_dn2), ((-var_q_fd_soi_dn4) - var_t6_dn4), ((-var_q_fd_soi_dn5) - var_t6_dn5), ((-var_q_fd_soi_dn6) - var_t6_dn6), ((-var_q_fd_soi_dn8) - var_t6_dn8), ((-var_q_fd_soi_dn10) - var_t6_dn10), ((-var_q_fd_soi_dn11) - var_t6_dn11), ((-var_q_fd_soi_dn12) - var_t6_dn12),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn8, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12,)
    }
};
            var_tmf1 = assign7580_body18_e6951;
            var_tmf1_dn0 = assign7580_body18_e6951_d_n0;
            var_tmf1_dn2 = assign7580_body18_e6951_d_n2;
            var_tmf1_dn4 = assign7580_body18_e6951_d_n4;
            var_tmf1_dn5 = assign7580_body18_e6951_d_n5;
            var_tmf1_dn6 = assign7580_body18_e6951_d_n6;
            var_tmf1_dn8 = assign7580_body18_e6951_d_n8;
            var_tmf1_dn10 = assign7580_body18_e6951_d_n10;
            var_tmf1_dn11 = assign7580_body18_e6951_d_n11;
            var_tmf1_dn12 = assign7580_body18_e6951_d_n12;
            var_tmf1_rv = 0.0;
            let (assign7580_body19_e6966, assign7580_body19_e6966_d_n0, assign7580_body19_e6966_d_n2, assign7580_body19_e6966_d_n4, assign7580_body19_e6966_d_n5, assign7580_body19_e6966_d_n6, assign7580_body19_e6966_d_n8, assign7580_body19_e6966_d_n10, assign7580_body19_e6966_d_n11, assign7580_body19_e6966_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body19_e6961: f64 = (-var_q_fd_soi);
        let assign7580_body19_e6962: f64 = (4.0 * assign7580_body19_e6961);
        let assign7580_body19_e6964: f64 = (assign7580_body19_e6962 * 1e-13);
        (assign7580_body19_e6964, ((4.0 * (-var_q_fd_soi_dn0)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn2)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn4)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn5)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn6)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn8)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn10)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn11)) * 1e-13), ((4.0 * (-var_q_fd_soi_dn12)) * 1e-13),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
            var_tmf2 = assign7580_body19_e6966;
            var_tmf2_dn0 = assign7580_body19_e6966_d_n0;
            var_tmf2_dn2 = assign7580_body19_e6966_d_n2;
            var_tmf2_dn4 = assign7580_body19_e6966_d_n4;
            var_tmf2_dn5 = assign7580_body19_e6966_d_n5;
            var_tmf2_dn6 = assign7580_body19_e6966_d_n6;
            var_tmf2_dn8 = assign7580_body19_e6966_d_n8;
            var_tmf2_dn10 = assign7580_body19_e6966_d_n10;
            var_tmf2_dn11 = assign7580_body19_e6966_d_n11;
            var_tmf2_dn12 = assign7580_body19_e6966_d_n12;
            var_tmf2_rv = 0.0;
            let (assign7580_body20_e6982, assign7580_body20_e6982_d_n0, assign7580_body20_e6982_d_n2, assign7580_body20_e6982_d_n4, assign7580_body20_e6982_d_n5, assign7580_body20_e6982_d_n6, assign7580_body20_e6982_d_n8, assign7580_body20_e6982_d_n10, assign7580_body20_e6982_d_n11, assign7580_body20_e6982_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let (assign7580_body20_e6980, assign7580_body20_e6980_d_n0, assign7580_body20_e6980_d_n2, assign7580_body20_e6980_d_n4, assign7580_body20_e6980_d_n5, assign7580_body20_e6980_d_n6, assign7580_body20_e6980_d_n8, assign7580_body20_e6980_d_n10, assign7580_body20_e6980_d_n11, assign7580_body20_e6980_d_n12,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
            } else {
                let assign7580_body20_e6979: f64 = (-var_tmf2);
                (assign7580_body20_e6979, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn8), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12),)
            }
        };
        (assign7580_body20_e6980, assign7580_body20_e6980_d_n0, assign7580_body20_e6980_d_n2, assign7580_body20_e6980_d_n4, assign7580_body20_e6980_d_n5, assign7580_body20_e6980_d_n6, assign7580_body20_e6980_d_n8, assign7580_body20_e6980_d_n10, assign7580_body20_e6980_d_n11, assign7580_body20_e6980_d_n12,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
            var_tmf2 = assign7580_body20_e6982;
            var_tmf2_dn0 = assign7580_body20_e6982_d_n0;
            var_tmf2_dn2 = assign7580_body20_e6982_d_n2;
            var_tmf2_dn4 = assign7580_body20_e6982_d_n4;
            var_tmf2_dn5 = assign7580_body20_e6982_d_n5;
            var_tmf2_dn6 = assign7580_body20_e6982_d_n6;
            var_tmf2_dn8 = assign7580_body20_e6982_d_n8;
            var_tmf2_dn10 = assign7580_body20_e6982_d_n10;
            var_tmf2_dn11 = assign7580_body20_e6982_d_n11;
            var_tmf2_dn12 = assign7580_body20_e6982_d_n12;
            var_tmf2_rv = 0.0;
            let (assign7580_body21_e6997, assign7580_body21_e6997_d_n0, assign7580_body21_e6997_d_n2, assign7580_body21_e6997_d_n4, assign7580_body21_e6997_d_n5, assign7580_body21_e6997_d_n6, assign7580_body21_e6997_d_n8, assign7580_body21_e6997_d_n10, assign7580_body21_e6997_d_n11, assign7580_body21_e6997_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body21_e6992: f64 = (var_tmf1 * var_tmf1);
        let assign7580_body21_e6994: f64 = (assign7580_body21_e6992 + var_tmf2);
        let assign7580_body21_e6995: f64 = (assign7580_body21_e6994).sqrt();
        (assign7580_body21_e6995, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign7580_body21_e6995)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign7580_body21_e6995)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign7580_body21_e6995)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign7580_body21_e6995)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign7580_body21_e6995)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign7580_body21_e6995)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign7580_body21_e6995)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign7580_body21_e6995)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign7580_body21_e6995)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
            var_tmf2 = assign7580_body21_e6997;
            var_tmf2_dn0 = assign7580_body21_e6997_d_n0;
            var_tmf2_dn2 = assign7580_body21_e6997_d_n2;
            var_tmf2_dn4 = assign7580_body21_e6997_d_n4;
            var_tmf2_dn5 = assign7580_body21_e6997_d_n5;
            var_tmf2_dn6 = assign7580_body21_e6997_d_n6;
            var_tmf2_dn8 = assign7580_body21_e6997_d_n8;
            var_tmf2_dn10 = assign7580_body21_e6997_d_n10;
            var_tmf2_dn11 = assign7580_body21_e6997_d_n11;
            var_tmf2_dn12 = assign7580_body21_e6997_d_n12;
            var_tmf2_rv = 0.0;
            let (assign7580_body22_e7013, assign7580_body22_e7013_d_n0, assign7580_body22_e7013_d_n2, assign7580_body22_e7013_d_n4, assign7580_body22_e7013_d_n5, assign7580_body22_e7013_d_n6, assign7580_body22_e7013_d_n8, assign7580_body22_e7013_d_n10, assign7580_body22_e7013_d_n11, assign7580_body22_e7013_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body22_e7009: f64 = (var_tmf1 / var_tmf2);
        let assign7580_body22_e7010: f64 = (1.0 + assign7580_body22_e7009);
        let assign7580_body22_e7011: f64 = (0.5 * assign7580_body22_e7010);
        (assign7580_body22_e7011, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn12 * var_tmf2) - (var_tmf1 * var_tmf2_dn12)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn8, var_t8_dn10, var_t8_dn11, var_t8_dn12,)
    }
};
            var_t8 = assign7580_body22_e7013;
            var_t8_dn0 = assign7580_body22_e7013_d_n0;
            var_t8_dn2 = assign7580_body22_e7013_d_n2;
            var_t8_dn4 = assign7580_body22_e7013_d_n4;
            var_t8_dn5 = assign7580_body22_e7013_d_n5;
            var_t8_dn6 = assign7580_body22_e7013_d_n6;
            var_t8_dn8 = assign7580_body22_e7013_d_n8;
            var_t8_dn10 = assign7580_body22_e7013_d_n10;
            var_t8_dn11 = assign7580_body22_e7013_d_n11;
            var_t8_dn12 = assign7580_body22_e7013_d_n12;
            var_t8_rv = 0.0;
            let (assign7580_body23_e7030, assign7580_body23_e7030_d_n0, assign7580_body23_e7030_d_n2, assign7580_body23_e7030_d_n4, assign7580_body23_e7030_d_n5, assign7580_body23_e7030_d_n6, assign7580_body23_e7030_d_n8, assign7580_body23_e7030_d_n10, assign7580_body23_e7030_d_n11, assign7580_body23_e7030_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body23_e7022: f64 = (-var_q_fd_soi);
        let assign7580_body23_e7026: f64 = (var_tmf1 + var_tmf2);
        let assign7580_body23_e7027: f64 = (0.5 * assign7580_body23_e7026);
        let assign7580_body23_e7028: f64 = (assign7580_body23_e7022 - assign7580_body23_e7027);
        (assign7580_body23_e7028, ((-var_q_fd_soi_dn0) - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), ((-var_q_fd_soi_dn2) - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), ((-var_q_fd_soi_dn4) - (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), ((-var_q_fd_soi_dn5) - (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), ((-var_q_fd_soi_dn6) - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), ((-var_q_fd_soi_dn8) - (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), ((-var_q_fd_soi_dn10) - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), ((-var_q_fd_soi_dn11) - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), ((-var_q_fd_soi_dn12) - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
            var_t6 = assign7580_body23_e7030;
            var_t6_dn0 = assign7580_body23_e7030_d_n0;
            var_t6_dn2 = assign7580_body23_e7030_d_n2;
            var_t6_dn4 = assign7580_body23_e7030_d_n4;
            var_t6_dn5 = assign7580_body23_e7030_d_n5;
            var_t6_dn6 = assign7580_body23_e7030_d_n6;
            var_t6_dn8 = assign7580_body23_e7030_d_n8;
            var_t6_dn10 = assign7580_body23_e7030_d_n10;
            var_t6_dn11 = assign7580_body23_e7030_d_n11;
            var_t6_dn12 = assign7580_body23_e7030_d_n12;
            var_t6_rv = 0.0;
            let (assign7580_body24_e7044, assign7580_body24_e7044_d_n0, assign7580_body24_e7044_d_n2, assign7580_body24_e7044_d_n4, assign7580_body24_e7044_d_n5, assign7580_body24_e7044_d_n6, assign7580_body24_e7044_d_n8, assign7580_body24_e7044_d_n10, assign7580_body24_e7044_d_n11, assign7580_body24_e7044_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body24_e7041: f64 = (var_t5 * var_t8);
        let assign7580_body24_e7042: f64 = (var_t7 * assign7580_body24_e7041);
        (assign7580_body24_e7042, ((var_t7_dn0 * assign7580_body24_e7041) + (var_t7 * ((var_t5_dn0 * var_t8) + (var_t5 * var_t8_dn0)))), ((var_t7_dn2 * assign7580_body24_e7041) + (var_t7 * ((var_t5_dn2 * var_t8) + (var_t5 * var_t8_dn2)))), ((var_t7_dn4 * assign7580_body24_e7041) + (var_t7 * ((var_t5_dn4 * var_t8) + (var_t5 * var_t8_dn4)))), ((var_t7_dn5 * assign7580_body24_e7041) + (var_t7 * ((var_t5_dn5 * var_t8) + (var_t5 * var_t8_dn5)))), ((var_t7_dn6 * assign7580_body24_e7041) + (var_t7 * ((var_t5_dn6 * var_t8) + (var_t5 * var_t8_dn6)))), ((var_t7_dn8 * assign7580_body24_e7041) + (var_t7 * ((var_t5_dn8 * var_t8) + (var_t5 * var_t8_dn8)))), ((var_t7_dn10 * assign7580_body24_e7041) + (var_t7 * ((var_t5_dn10 * var_t8) + (var_t5 * var_t8_dn10)))), ((var_t7_dn11 * assign7580_body24_e7041) + (var_t7 * ((var_t5_dn11 * var_t8) + (var_t5 * var_t8_dn11)))), ((var_t7_dn12 * assign7580_body24_e7041) + (var_t7 * ((var_t5_dn12 * var_t8) + (var_t5 * var_t8_dn12)))),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
            var_t7 = assign7580_body24_e7044;
            var_t7_dn0 = assign7580_body24_e7044_d_n0;
            var_t7_dn2 = assign7580_body24_e7044_d_n2;
            var_t7_dn4 = assign7580_body24_e7044_d_n4;
            var_t7_dn5 = assign7580_body24_e7044_d_n5;
            var_t7_dn6 = assign7580_body24_e7044_d_n6;
            var_t7_dn8 = assign7580_body24_e7044_d_n8;
            var_t7_dn10 = assign7580_body24_e7044_d_n10;
            var_t7_dn11 = assign7580_body24_e7044_d_n11;
            var_t7_dn12 = assign7580_body24_e7044_d_n12;
            var_t7_rv = 0.0;
            let (assign7580_body25_e7064, assign7580_body25_e7064_d_n0, assign7580_body25_e7064_d_n2, assign7580_body25_e7064_d_n4, assign7580_body25_e7064_d_n5, assign7580_body25_e7064_d_n6, assign7580_body25_e7064_d_n8, assign7580_body25_e7064_d_n10, assign7580_body25_e7064_d_n11, assign7580_body25_e7064_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body25_e7054: f64 = (var_t6 * var_t6);
        let assign7580_body25_e7056: f64 = (assign7580_body25_e7054 / 2.0);
        let assign7580_body25_e7058: f64 = (assign7580_body25_e7056 / 1.034943e-10);
        let assign7580_body25_e7060: f64 = (assign7580_body25_e7058 / 1.6021918e-19);
        let assign7580_body25_e7062: f64 = (assign7580_body25_e7060 / var_uc_nsubs);
        (assign7580_body25_e7062, ((((((((var_t6_dn0 * var_t6) + (var_t6 * var_t6_dn0)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7580_body25_e7060 * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn2 * var_t6) + (var_t6 * var_t6_dn2)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7580_body25_e7060 * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn4 * var_t6) + (var_t6 * var_t6_dn4)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7580_body25_e7060 * var_uc_nsubs_dn4)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn5 * var_t6) + (var_t6 * var_t6_dn5)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7580_body25_e7060 * var_uc_nsubs_dn5)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn6 * var_t6) + (var_t6 * var_t6_dn6)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7580_body25_e7060 * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn8 * var_t6) + (var_t6 * var_t6_dn8)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7580_body25_e7060 * var_uc_nsubs_dn8)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn10 * var_t6) + (var_t6 * var_t6_dn10)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7580_body25_e7060 * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn11 * var_t6) + (var_t6 * var_t6_dn11)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7580_body25_e7060 * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_t6_dn12 * var_t6) + (var_t6 * var_t6_dn12)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign7580_body25_e7060 * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)),)
    } else {
        (var_phi_b_dep, var_phi_b_dep_dn0, var_phi_b_dep_dn2, var_phi_b_dep_dn4, var_phi_b_dep_dn5, var_phi_b_dep_dn6, var_phi_b_dep_dn8, var_phi_b_dep_dn10, var_phi_b_dep_dn11, var_phi_b_dep_dn12,)
    }
};
            var_phi_b_dep = assign7580_body25_e7064;
            var_phi_b_dep_dn0 = assign7580_body25_e7064_d_n0;
            var_phi_b_dep_dn2 = assign7580_body25_e7064_d_n2;
            var_phi_b_dep_dn4 = assign7580_body25_e7064_d_n4;
            var_phi_b_dep_dn5 = assign7580_body25_e7064_d_n5;
            var_phi_b_dep_dn6 = assign7580_body25_e7064_d_n6;
            var_phi_b_dep_dn8 = assign7580_body25_e7064_d_n8;
            var_phi_b_dep_dn10 = assign7580_body25_e7064_d_n10;
            var_phi_b_dep_dn11 = assign7580_body25_e7064_d_n11;
            var_phi_b_dep_dn12 = assign7580_body25_e7064_d_n12;
            var_phi_b_dep_rv = 0.0;
            let (assign7580_body26_e7080, assign7580_body26_e7080_d_n0, assign7580_body26_e7080_d_n2, assign7580_body26_e7080_d_n4, assign7580_body26_e7080_d_n5, assign7580_body26_e7080_d_n6, assign7580_body26_e7080_d_n8, assign7580_body26_e7080_d_n10, assign7580_body26_e7080_d_n11, assign7580_body26_e7080_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body26_e7074: f64 = (2.0 * var_phi_b_dep);
        let assign7580_body26_e7076: f64 = (assign7580_body26_e7074 * var_t7);
        let assign7580_body26_e7078: f64 = (assign7580_body26_e7076 / var_t6);
        (assign7580_body26_e7078, ((((((2.0 * var_phi_b_dep_dn0) * var_t7) + (assign7580_body26_e7074 * var_t7_dn0)) * var_t6) - (assign7580_body26_e7076 * var_t6_dn0)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn2) * var_t7) + (assign7580_body26_e7074 * var_t7_dn2)) * var_t6) - (assign7580_body26_e7076 * var_t6_dn2)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn4) * var_t7) + (assign7580_body26_e7074 * var_t7_dn4)) * var_t6) - (assign7580_body26_e7076 * var_t6_dn4)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn5) * var_t7) + (assign7580_body26_e7074 * var_t7_dn5)) * var_t6) - (assign7580_body26_e7076 * var_t6_dn5)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn6) * var_t7) + (assign7580_body26_e7074 * var_t7_dn6)) * var_t6) - (assign7580_body26_e7076 * var_t6_dn6)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn8) * var_t7) + (assign7580_body26_e7074 * var_t7_dn8)) * var_t6) - (assign7580_body26_e7076 * var_t6_dn8)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn10) * var_t7) + (assign7580_body26_e7074 * var_t7_dn10)) * var_t6) - (assign7580_body26_e7076 * var_t6_dn10)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn11) * var_t7) + (assign7580_body26_e7074 * var_t7_dn11)) * var_t6) - (assign7580_body26_e7076 * var_t6_dn11)) / (var_t6 * var_t6)), ((((((2.0 * var_phi_b_dep_dn12) * var_t7) + (assign7580_body26_e7074 * var_t7_dn12)) * var_t6) - (assign7580_body26_e7076 * var_t6_dn12)) / (var_t6 * var_t6)),)
    } else {
        (var_phi_b_dep_dpsb, var_phi_b_dep_dpsb_dn0, var_phi_b_dep_dpsb_dn2, var_phi_b_dep_dpsb_dn4, var_phi_b_dep_dpsb_dn5, var_phi_b_dep_dpsb_dn6, var_phi_b_dep_dpsb_dn8, var_phi_b_dep_dpsb_dn10, var_phi_b_dep_dpsb_dn11, var_phi_b_dep_dpsb_dn12,)
    }
};
            var_phi_b_dep_dpsb = assign7580_body26_e7080;
            var_phi_b_dep_dpsb_dn0 = assign7580_body26_e7080_d_n0;
            var_phi_b_dep_dpsb_dn2 = assign7580_body26_e7080_d_n2;
            var_phi_b_dep_dpsb_dn4 = assign7580_body26_e7080_d_n4;
            var_phi_b_dep_dpsb_dn5 = assign7580_body26_e7080_d_n5;
            var_phi_b_dep_dpsb_dn6 = assign7580_body26_e7080_d_n6;
            var_phi_b_dep_dpsb_dn8 = assign7580_body26_e7080_d_n8;
            var_phi_b_dep_dpsb_dn10 = assign7580_body26_e7080_d_n10;
            var_phi_b_dep_dpsb_dn11 = assign7580_body26_e7080_d_n11;
            var_phi_b_dep_dpsb_dn12 = assign7580_body26_e7080_d_n12;
            var_phi_b_dep_dpsb_rv = 0.0;
            let (assign7580_body27_e7127, assign7580_body27_e7127_d_n0, assign7580_body27_e7127_d_n2, assign7580_body27_e7127_d_n4, assign7580_body27_e7127_d_n5, assign7580_body27_e7127_d_n6, assign7580_body27_e7127_d_n8, assign7580_body27_e7127_d_n10, assign7580_body27_e7127_d_n11, assign7580_body27_e7127_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body27_e7091: f64 = (var_phi_s0_soi - var_phi_s0_bulk);
        let assign7580_body27_e7094: f64 = (var_t4 / var_c_box);
        let assign7580_body27_e7095: f64 = (assign7580_body27_e7091 + assign7580_body27_e7094);
        let assign7580_body27_e7099: f64 = (var_q_fd_soi / 2.0);
        let assign7580_body27_e7100: f64 = (var_t4 + assign7580_body27_e7099);
        let assign7580_body27_e7102: f64 = (assign7580_body27_e7100 * p.p227);
        let assign7580_body27_e7104: f64 = (assign7580_body27_e7102 / 1.034943e-10);
        let assign7580_body27_e7105: f64 = (assign7580_body27_e7095 + assign7580_body27_e7104);
        let assign7580_body27_e7107: f64 = (assign7580_body27_e7105 - var_vbsbiz);
        let assign7580_body27_e7109: f64 = (assign7580_body27_e7107 + var_phi_b_dep);
        let assign7580_body27_e7111: f64 = (-1.0);
        let assign7580_body27_e7114: f64 = (var_t5 / var_c_box);
        let assign7580_body27_e7115: f64 = (assign7580_body27_e7111 + assign7580_body27_e7114);
        let assign7580_body27_e7118: f64 = (var_t5 * p.p227);
        let assign7580_body27_e7120: f64 = (assign7580_body27_e7118 / 1.034943e-10);
        let assign7580_body27_e7121: f64 = (assign7580_body27_e7115 + assign7580_body27_e7120);
        let assign7580_body27_e7123: f64 = (assign7580_body27_e7121 + var_phi_b_dep_dpsb);
        let assign7580_body27_e7124: f64 = (assign7580_body27_e7109 / assign7580_body27_e7123);
        let assign7580_body27_e7125: f64 = (var_phi_s0_bulk - assign7580_body27_e7124);
        (assign7580_body27_e7125, (var_phi_s0_bulk_dn0 - ((((((((var_phi_s0_soi_dn0 - var_phi_s0_bulk_dn0) + (var_t4_dn0 / var_c_box)) + (((var_t4_dn0 + (var_q_fd_soi_dn0 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn0) + var_phi_b_dep_dn0) * assign7580_body27_e7123) - (assign7580_body27_e7109 * (((var_t5_dn0 / var_c_box) + ((var_t5_dn0 * p.p227) / 1.034943e-10)) + var_phi_b_dep_dpsb_dn0))) / (assign7580_body27_e7123 * assign7580_body27_e7123))), (var_phi_s0_bulk_dn2 - ((((((((var_phi_s0_soi_dn2 - var_phi_s0_bulk_dn2) + (var_t4_dn2 / var_c_box)) + (((var_t4_dn2 + (var_q_fd_soi_dn2 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn2) + var_phi_b_dep_dn2) * assign7580_body27_e7123) - (assign7580_body27_e7109 * (((var_t5_dn2 / var_c_box) + ((var_t5_dn2 * p.p227) / 1.034943e-10)) + var_phi_b_dep_dpsb_dn2))) / (assign7580_body27_e7123 * assign7580_body27_e7123))), (var_phi_s0_bulk_dn4 - ((((((((var_phi_s0_soi_dn4 - var_phi_s0_bulk_dn4) + (var_t4_dn4 / var_c_box)) + (((var_t4_dn4 + (var_q_fd_soi_dn4 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn4) + var_phi_b_dep_dn4) * assign7580_body27_e7123) - (assign7580_body27_e7109 * (((var_t5_dn4 / var_c_box) + ((var_t5_dn4 * p.p227) / 1.034943e-10)) + var_phi_b_dep_dpsb_dn4))) / (assign7580_body27_e7123 * assign7580_body27_e7123))), (var_phi_s0_bulk_dn5 - ((((((((var_phi_s0_soi_dn5 - var_phi_s0_bulk_dn5) + (var_t4_dn5 / var_c_box)) + (((var_t4_dn5 + (var_q_fd_soi_dn5 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn5) + var_phi_b_dep_dn5) * assign7580_body27_e7123) - (assign7580_body27_e7109 * (((var_t5_dn5 / var_c_box) + ((var_t5_dn5 * p.p227) / 1.034943e-10)) + var_phi_b_dep_dpsb_dn5))) / (assign7580_body27_e7123 * assign7580_body27_e7123))), (var_phi_s0_bulk_dn6 - ((((((((var_phi_s0_soi_dn6 - var_phi_s0_bulk_dn6) + (var_t4_dn6 / var_c_box)) + (((var_t4_dn6 + (var_q_fd_soi_dn6 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn6) + var_phi_b_dep_dn6) * assign7580_body27_e7123) - (assign7580_body27_e7109 * (((var_t5_dn6 / var_c_box) + ((var_t5_dn6 * p.p227) / 1.034943e-10)) + var_phi_b_dep_dpsb_dn6))) / (assign7580_body27_e7123 * assign7580_body27_e7123))), (var_phi_s0_bulk_dn8 - ((((((((var_phi_s0_soi_dn8 - var_phi_s0_bulk_dn8) + (var_t4_dn8 / var_c_box)) + (((var_t4_dn8 + (var_q_fd_soi_dn8 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn8) + var_phi_b_dep_dn8) * assign7580_body27_e7123) - (assign7580_body27_e7109 * (((var_t5_dn8 / var_c_box) + ((var_t5_dn8 * p.p227) / 1.034943e-10)) + var_phi_b_dep_dpsb_dn8))) / (assign7580_body27_e7123 * assign7580_body27_e7123))), (var_phi_s0_bulk_dn10 - ((((((((var_phi_s0_soi_dn10 - var_phi_s0_bulk_dn10) + (var_t4_dn10 / var_c_box)) + (((var_t4_dn10 + (var_q_fd_soi_dn10 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn10) + var_phi_b_dep_dn10) * assign7580_body27_e7123) - (assign7580_body27_e7109 * (((var_t5_dn10 / var_c_box) + ((var_t5_dn10 * p.p227) / 1.034943e-10)) + var_phi_b_dep_dpsb_dn10))) / (assign7580_body27_e7123 * assign7580_body27_e7123))), (var_phi_s0_bulk_dn11 - ((((((((var_phi_s0_soi_dn11 - var_phi_s0_bulk_dn11) + (var_t4_dn11 / var_c_box)) + (((var_t4_dn11 + (var_q_fd_soi_dn11 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn11) + var_phi_b_dep_dn11) * assign7580_body27_e7123) - (assign7580_body27_e7109 * (((var_t5_dn11 / var_c_box) + ((var_t5_dn11 * p.p227) / 1.034943e-10)) + var_phi_b_dep_dpsb_dn11))) / (assign7580_body27_e7123 * assign7580_body27_e7123))), (var_phi_s0_bulk_dn12 - ((((((((var_phi_s0_soi_dn12 - var_phi_s0_bulk_dn12) + (var_t4_dn12 / var_c_box)) + (((var_t4_dn12 + (var_q_fd_soi_dn12 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn12) + var_phi_b_dep_dn12) * assign7580_body27_e7123) - (assign7580_body27_e7109 * (((var_t5_dn12 / var_c_box) + ((var_t5_dn12 * p.p227) / 1.034943e-10)) + var_phi_b_dep_dpsb_dn12))) / (assign7580_body27_e7123 * assign7580_body27_e7123))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
            var_t6 = assign7580_body27_e7127;
            var_t6_dn0 = assign7580_body27_e7127_d_n0;
            var_t6_dn2 = assign7580_body27_e7127_d_n2;
            var_t6_dn4 = assign7580_body27_e7127_d_n4;
            var_t6_dn5 = assign7580_body27_e7127_d_n5;
            var_t6_dn6 = assign7580_body27_e7127_d_n6;
            var_t6_dn8 = assign7580_body27_e7127_d_n8;
            var_t6_dn10 = assign7580_body27_e7127_d_n10;
            var_t6_dn11 = assign7580_body27_e7127_d_n11;
            var_t6_dn12 = assign7580_body27_e7127_d_n12;
            var_t6_rv = 0.0;
            let assign7580_body28_e7130: f64 = (var_t6 - var_phi_s0_bulk);
            let assign7580_body28_e7131: f64 = (assign7580_body28_e7130).abs();
            let assign7580_body28_e7133: f64 = if assign7580_body28_e7131 < 0.001 { 1.0 } else { 0.0 };
            var_guard91 = assign7580_body28_e7133;
            var_guard91_rv = 0.0;
            let (assign7580_body29_e7145,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) && (var_guard91 != 0.0)) {
        (var_lp_s0_max,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign7580_body29_e7145;
            var_lp_s0_rv = 0.0;
            let (assign7580_body30_e7155, assign7580_body30_e7155_d_n0, assign7580_body30_e7155_d_n2, assign7580_body30_e7155_d_n4, assign7580_body30_e7155_d_n5, assign7580_body30_e7155_d_n6, assign7580_body30_e7155_d_n8, assign7580_body30_e7155_d_n10, assign7580_body30_e7155_d_n11, assign7580_body30_e7155_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
            var_phi_s0_bulk = assign7580_body30_e7155;
            var_phi_s0_bulk_dn0 = assign7580_body30_e7155_d_n0;
            var_phi_s0_bulk_dn2 = assign7580_body30_e7155_d_n2;
            var_phi_s0_bulk_dn4 = assign7580_body30_e7155_d_n4;
            var_phi_s0_bulk_dn5 = assign7580_body30_e7155_d_n5;
            var_phi_s0_bulk_dn6 = assign7580_body30_e7155_d_n6;
            var_phi_s0_bulk_dn8 = assign7580_body30_e7155_d_n8;
            var_phi_s0_bulk_dn10 = assign7580_body30_e7155_d_n10;
            var_phi_s0_bulk_dn11 = assign7580_body30_e7155_d_n11;
            var_phi_s0_bulk_dn12 = assign7580_body30_e7155_d_n12;
            var_phi_s0_bulk_rv = 0.0;
            let (assign7580_body31_e7165, assign7580_body31_e7165_d_n0, assign7580_body31_e7165_d_n2, assign7580_body31_e7165_d_n4, assign7580_body31_e7165_d_n5, assign7580_body31_e7165_d_n6, assign7580_body31_e7165_d_n8, assign7580_body31_e7165_d_n10, assign7580_body31_e7165_d_n11, assign7580_body31_e7165_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    } else {
        (var_q_s0_bulk, var_q_s0_bulk_dn0, var_q_s0_bulk_dn2, var_q_s0_bulk_dn4, var_q_s0_bulk_dn5, var_q_s0_bulk_dn6, var_q_s0_bulk_dn8, var_q_s0_bulk_dn10, var_q_s0_bulk_dn11, var_q_s0_bulk_dn12,)
    }
};
            var_q_s0_bulk = assign7580_body31_e7165;
            var_q_s0_bulk_dn0 = assign7580_body31_e7165_d_n0;
            var_q_s0_bulk_dn2 = assign7580_body31_e7165_d_n2;
            var_q_s0_bulk_dn4 = assign7580_body31_e7165_d_n4;
            var_q_s0_bulk_dn5 = assign7580_body31_e7165_d_n5;
            var_q_s0_bulk_dn6 = assign7580_body31_e7165_d_n6;
            var_q_s0_bulk_dn8 = assign7580_body31_e7165_d_n8;
            var_q_s0_bulk_dn10 = assign7580_body31_e7165_d_n10;
            var_q_s0_bulk_dn11 = assign7580_body31_e7165_d_n11;
            var_q_s0_bulk_dn12 = assign7580_body31_e7165_d_n12;
            var_q_s0_bulk_rv = 0.0;
            let (assign7580_body32_e7177,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body32_e7175: f64 = (var_lp_s0 + 1.0);
        (assign7580_body32_e7175,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign7580_body32_e7177;
            var_lp_s0_rv = 0.0;
        }

        *var_guard88_slot = var_guard88;
        *var_guard88_rv_slot = var_guard88_rv;
        *var_guard89_slot = var_guard89;
        *var_guard89_rv_slot = var_guard89_rv;
        *var_guard90_slot = var_guard90;
        *var_guard90_rv_slot = var_guard90_rv;
        *var_guard91_slot = var_guard91;
        *var_guard91_rv_slot = var_guard91_rv;
        *var_lp_s0_slot = var_lp_s0;
        *var_lp_s0_rv_slot = var_lp_s0_rv;
        *var_phi_b_dep_slot = var_phi_b_dep;
        *var_phi_b_dep_dn0_slot = var_phi_b_dep_dn0;
        *var_phi_b_dep_dn10_slot = var_phi_b_dep_dn10;
        *var_phi_b_dep_dn11_slot = var_phi_b_dep_dn11;
        *var_phi_b_dep_dn12_slot = var_phi_b_dep_dn12;
        *var_phi_b_dep_dn2_slot = var_phi_b_dep_dn2;
        *var_phi_b_dep_dn4_slot = var_phi_b_dep_dn4;
        *var_phi_b_dep_dn5_slot = var_phi_b_dep_dn5;
        *var_phi_b_dep_dn6_slot = var_phi_b_dep_dn6;
        *var_phi_b_dep_dn8_slot = var_phi_b_dep_dn8;
        *var_phi_b_dep_dpsb_slot = var_phi_b_dep_dpsb;
        *var_phi_b_dep_dpsb_dn0_slot = var_phi_b_dep_dpsb_dn0;
        *var_phi_b_dep_dpsb_dn10_slot = var_phi_b_dep_dpsb_dn10;
        *var_phi_b_dep_dpsb_dn11_slot = var_phi_b_dep_dpsb_dn11;
        *var_phi_b_dep_dpsb_dn12_slot = var_phi_b_dep_dpsb_dn12;
        *var_phi_b_dep_dpsb_dn2_slot = var_phi_b_dep_dpsb_dn2;
        *var_phi_b_dep_dpsb_dn4_slot = var_phi_b_dep_dpsb_dn4;
        *var_phi_b_dep_dpsb_dn5_slot = var_phi_b_dep_dpsb_dn5;
        *var_phi_b_dep_dpsb_dn6_slot = var_phi_b_dep_dpsb_dn6;
        *var_phi_b_dep_dpsb_dn8_slot = var_phi_b_dep_dpsb_dn8;
        *var_phi_b_dep_dpsb_rv_slot = var_phi_b_dep_dpsb_rv;
        *var_phi_b_dep_rv_slot = var_phi_b_dep_rv;
        *var_phi_s0_bulk_slot = var_phi_s0_bulk;
        *var_phi_s0_bulk_dn0_slot = var_phi_s0_bulk_dn0;
        *var_phi_s0_bulk_dn10_slot = var_phi_s0_bulk_dn10;
        *var_phi_s0_bulk_dn11_slot = var_phi_s0_bulk_dn11;
        *var_phi_s0_bulk_dn12_slot = var_phi_s0_bulk_dn12;
        *var_phi_s0_bulk_dn2_slot = var_phi_s0_bulk_dn2;
        *var_phi_s0_bulk_dn4_slot = var_phi_s0_bulk_dn4;
        *var_phi_s0_bulk_dn5_slot = var_phi_s0_bulk_dn5;
        *var_phi_s0_bulk_dn6_slot = var_phi_s0_bulk_dn6;
        *var_phi_s0_bulk_dn8_slot = var_phi_s0_bulk_dn8;
        *var_phi_s0_bulk_rv_slot = var_phi_s0_bulk_rv;
        *var_q_s0_bulk_slot = var_q_s0_bulk;
        *var_q_s0_bulk_dn0_slot = var_q_s0_bulk_dn0;
        *var_q_s0_bulk_dn10_slot = var_q_s0_bulk_dn10;
        *var_q_s0_bulk_dn11_slot = var_q_s0_bulk_dn11;
        *var_q_s0_bulk_dn12_slot = var_q_s0_bulk_dn12;
        *var_q_s0_bulk_dn2_slot = var_q_s0_bulk_dn2;
        *var_q_s0_bulk_dn4_slot = var_q_s0_bulk_dn4;
        *var_q_s0_bulk_dn5_slot = var_q_s0_bulk_dn5;
        *var_q_s0_bulk_dn6_slot = var_q_s0_bulk_dn6;
        *var_q_s0_bulk_dn8_slot = var_q_s0_bulk_dn8;
        *var_q_s0_bulk_rv_slot = var_q_s0_bulk_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn12_slot = var_t6_dn12;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_rv_slot = var_t6_rv;
        *var_t7_slot = var_t7;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn12_slot = var_t7_dn12;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_rv_slot = var_t7_rv;
        *var_t8_slot = var_t8;
        *var_t8_dn0_slot = var_t8_dn0;
        *var_t8_dn10_slot = var_t8_dn10;
        *var_t8_dn11_slot = var_t8_dn11;
        *var_t8_dn12_slot = var_t8_dn12;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_rv_slot = var_t8_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_27(
        p: &Parameters,
        var_beta: f64,
        var_beta2: f64,
        var_beta2_dn4: f64,
        var_beta_dn4: f64,
        var_c_box: f64,
        var_c_fox: f64,
        var_c_fox_dn0: f64,
        var_c_fox_dn10: f64,
        var_c_fox_dn11: f64,
        var_c_fox_dn12: f64,
        var_c_fox_dn2: f64,
        var_c_fox_dn4: f64,
        var_c_fox_dn5: f64,
        var_c_fox_dn6: f64,
        var_c_fox_dn8: f64,
        var_cnst1soi: f64,
        var_cnst1soi_dn0: f64,
        var_cnst1soi_dn10: f64,
        var_cnst1soi_dn11: f64,
        var_cnst1soi_dn12: f64,
        var_cnst1soi_dn2: f64,
        var_cnst1soi_dn4: f64,
        var_cnst1soi_dn5: f64,
        var_cnst1soi_dn6: f64,
        var_cnst1soi_dn8: f64,
        var_cnstc_foxi: f64,
        var_cnstc_foxi_dn0: f64,
        var_cnstc_foxi_dn10: f64,
        var_cnstc_foxi_dn11: f64,
        var_cnstc_foxi_dn12: f64,
        var_cnstc_foxi_dn2: f64,
        var_cnstc_foxi_dn4: f64,
        var_cnstc_foxi_dn5: f64,
        var_cnstc_foxi_dn6: f64,
        var_cnstc_foxi_dn8: f64,
        var_fac1p2: f64,
        var_fac1p2_dn0: f64,
        var_fac1p2_dn10: f64,
        var_fac1p2_dn11: f64,
        var_fac1p2_dn12: f64,
        var_fac1p2_dn2: f64,
        var_fac1p2_dn4: f64,
        var_fac1p2_dn5: f64,
        var_fac1p2_dn6: f64,
        var_fac1p2_dn8: f64,
        var_guard74: f64,
        var_guard79: f64,
        var_q_fd_soi: f64,
        var_q_fd_soi_dn0: f64,
        var_q_fd_soi_dn10: f64,
        var_q_fd_soi_dn11: f64,
        var_q_fd_soi_dn12: f64,
        var_q_fd_soi_dn2: f64,
        var_q_fd_soi_dn4: f64,
        var_q_fd_soi_dn5: f64,
        var_q_fd_soi_dn6: f64,
        var_q_fd_soi_dn8: f64,
        var_q_s0_bulk: f64,
        var_q_s0_bulk_dn0: f64,
        var_q_s0_bulk_dn10: f64,
        var_q_s0_bulk_dn11: f64,
        var_q_s0_bulk_dn12: f64,
        var_q_s0_bulk_dn2: f64,
        var_q_s0_bulk_dn4: f64,
        var_q_s0_bulk_dn5: f64,
        var_q_s0_bulk_dn6: f64,
        var_q_s0_bulk_dn8: f64,
        var_shift: f64,
        var_shift_dn0: f64,
        var_shift_dn10: f64,
        var_shift_dn11: f64,
        var_shift_dn12: f64,
        var_shift_dn2: f64,
        var_shift_dn4: f64,
        var_shift_dn5: f64,
        var_shift_dn6: f64,
        var_shift_dn8: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn4: f64,
        var_uc_nsubs_dn5: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn8: f64,
        var_vbs: f64,
        var_vbs_dn0: f64,
        var_vbs_dn10: f64,
        var_vbs_dn11: f64,
        var_vbs_dn12: f64,
        var_vbs_dn2: f64,
        var_vbs_dn4: f64,
        var_vbs_dn5: f64,
        var_vbs_dn6: f64,
        var_vbs_dn8: f64,
        var_vbsbiz: f64,
        var_vbsbiz_dn0: f64,
        var_vbsbiz_dn10: f64,
        var_vbsbiz_dn11: f64,
        var_vbsbiz_dn12: f64,
        var_vbsbiz_dn2: f64,
        var_vbsbiz_dn4: f64,
        var_vbsbiz_dn5: f64,
        var_vbsbiz_dn6: f64,
        var_vbsbiz_dn8: f64,
        var_vgpz: f64,
        var_vgpz_dn0: f64,
        var_vgpz_dn10: f64,
        var_vgpz_dn11: f64,
        var_vgpz_dn12: f64,
        var_vgpz_dn2: f64,
        var_vgpz_dn4: f64,
        var_vgpz_dn5: f64,
        var_vgpz_dn6: f64,
        var_vgpz_dn8: f64,
        var_vgs: f64,
        var_vth: f64,
        var_guard92_slot: &mut f64,
        var_guard92_rv_slot: &mut f64,
        var_guard93_slot: &mut f64,
        var_guard93_rv_slot: &mut f64,
        var_guard94_slot: &mut f64,
        var_guard94_rv_slot: &mut f64,
        var_phi_b0_soi_slot: &mut f64,
        var_phi_b0_soi_dn0_slot: &mut f64,
        var_phi_b0_soi_dn10_slot: &mut f64,
        var_phi_b0_soi_dn11_slot: &mut f64,
        var_phi_b0_soi_dn12_slot: &mut f64,
        var_phi_b0_soi_dn2_slot: &mut f64,
        var_phi_b0_soi_dn4_slot: &mut f64,
        var_phi_b0_soi_dn5_slot: &mut f64,
        var_phi_b0_soi_dn6_slot: &mut f64,
        var_phi_b0_soi_dn8_slot: &mut f64,
        var_phi_b0_soi_rv_slot: &mut f64,
        var_phi_s0_bulk_slot: &mut f64,
        var_phi_s0_bulk_dn0_slot: &mut f64,
        var_phi_s0_bulk_dn10_slot: &mut f64,
        var_phi_s0_bulk_dn11_slot: &mut f64,
        var_phi_s0_bulk_dn12_slot: &mut f64,
        var_phi_s0_bulk_dn2_slot: &mut f64,
        var_phi_s0_bulk_dn4_slot: &mut f64,
        var_phi_s0_bulk_dn5_slot: &mut f64,
        var_phi_s0_bulk_dn6_slot: &mut f64,
        var_phi_s0_bulk_dn8_slot: &mut f64,
        var_phi_s0_bulk_rv_slot: &mut f64,
        var_ps0_ini_slot: &mut f64,
        var_ps0_ini_dn0_slot: &mut f64,
        var_ps0_ini_dn10_slot: &mut f64,
        var_ps0_ini_dn11_slot: &mut f64,
        var_ps0_ini_dn12_slot: &mut f64,
        var_ps0_ini_dn2_slot: &mut f64,
        var_ps0_ini_dn4_slot: &mut f64,
        var_ps0_ini_dn5_slot: &mut f64,
        var_ps0_ini_dn6_slot: &mut f64,
        var_ps0_ini_dn8_slot: &mut f64,
        var_ps0_ini_rv_slot: &mut f64,
        var_ps0_inia_slot: &mut f64,
        var_ps0_inia_dn0_slot: &mut f64,
        var_ps0_inia_dn10_slot: &mut f64,
        var_ps0_inia_dn11_slot: &mut f64,
        var_ps0_inia_dn12_slot: &mut f64,
        var_ps0_inia_dn2_slot: &mut f64,
        var_ps0_inia_dn4_slot: &mut f64,
        var_ps0_inia_dn5_slot: &mut f64,
        var_ps0_inia_dn6_slot: &mut f64,
        var_ps0_inia_dn8_slot: &mut f64,
        var_ps0_inia_rv_slot: &mut f64,
        var_ps0_inib_slot: &mut f64,
        var_ps0_inib_dn0_slot: &mut f64,
        var_ps0_inib_dn10_slot: &mut f64,
        var_ps0_inib_dn11_slot: &mut f64,
        var_ps0_inib_dn12_slot: &mut f64,
        var_ps0_inib_dn2_slot: &mut f64,
        var_ps0_inib_dn4_slot: &mut f64,
        var_ps0_inib_dn5_slot: &mut f64,
        var_ps0_inib_dn6_slot: &mut f64,
        var_ps0_inib_dn8_slot: &mut f64,
        var_ps0_inib_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tx_slot: &mut f64,
        var_tx_dn0_slot: &mut f64,
        var_tx_dn10_slot: &mut f64,
        var_tx_dn11_slot: &mut f64,
        var_tx_dn12_slot: &mut f64,
        var_tx_dn2_slot: &mut f64,
        var_tx_dn4_slot: &mut f64,
        var_tx_dn5_slot: &mut f64,
        var_tx_dn6_slot: &mut f64,
        var_tx_dn8_slot: &mut f64,
        var_tx_rv_slot: &mut f64,
        var_x2_slot: &mut f64,
        var_x2_dn0_slot: &mut f64,
        var_x2_dn10_slot: &mut f64,
        var_x2_dn11_slot: &mut f64,
        var_x2_dn12_slot: &mut f64,
        var_x2_dn2_slot: &mut f64,
        var_x2_dn4_slot: &mut f64,
        var_x2_dn5_slot: &mut f64,
        var_x2_dn6_slot: &mut f64,
        var_x2_dn8_slot: &mut f64,
        var_x2_rv_slot: &mut f64,
    ) {
        let mut var_guard92: f64 = *var_guard92_slot;
        let mut var_guard92_rv: f64 = *var_guard92_rv_slot;
        let mut var_guard93: f64 = *var_guard93_slot;
        let mut var_guard93_rv: f64 = *var_guard93_rv_slot;
        let mut var_guard94: f64 = *var_guard94_slot;
        let mut var_guard94_rv: f64 = *var_guard94_rv_slot;
        let mut var_phi_b0_soi: f64 = *var_phi_b0_soi_slot;
        let mut var_phi_b0_soi_dn0: f64 = *var_phi_b0_soi_dn0_slot;
        let mut var_phi_b0_soi_dn10: f64 = *var_phi_b0_soi_dn10_slot;
        let mut var_phi_b0_soi_dn11: f64 = *var_phi_b0_soi_dn11_slot;
        let mut var_phi_b0_soi_dn12: f64 = *var_phi_b0_soi_dn12_slot;
        let mut var_phi_b0_soi_dn2: f64 = *var_phi_b0_soi_dn2_slot;
        let mut var_phi_b0_soi_dn4: f64 = *var_phi_b0_soi_dn4_slot;
        let mut var_phi_b0_soi_dn5: f64 = *var_phi_b0_soi_dn5_slot;
        let mut var_phi_b0_soi_dn6: f64 = *var_phi_b0_soi_dn6_slot;
        let mut var_phi_b0_soi_dn8: f64 = *var_phi_b0_soi_dn8_slot;
        let mut var_phi_b0_soi_rv: f64 = *var_phi_b0_soi_rv_slot;
        let mut var_phi_s0_bulk: f64 = *var_phi_s0_bulk_slot;
        let mut var_phi_s0_bulk_dn0: f64 = *var_phi_s0_bulk_dn0_slot;
        let mut var_phi_s0_bulk_dn10: f64 = *var_phi_s0_bulk_dn10_slot;
        let mut var_phi_s0_bulk_dn11: f64 = *var_phi_s0_bulk_dn11_slot;
        let mut var_phi_s0_bulk_dn12: f64 = *var_phi_s0_bulk_dn12_slot;
        let mut var_phi_s0_bulk_dn2: f64 = *var_phi_s0_bulk_dn2_slot;
        let mut var_phi_s0_bulk_dn4: f64 = *var_phi_s0_bulk_dn4_slot;
        let mut var_phi_s0_bulk_dn5: f64 = *var_phi_s0_bulk_dn5_slot;
        let mut var_phi_s0_bulk_dn6: f64 = *var_phi_s0_bulk_dn6_slot;
        let mut var_phi_s0_bulk_dn8: f64 = *var_phi_s0_bulk_dn8_slot;
        let mut var_phi_s0_bulk_rv: f64 = *var_phi_s0_bulk_rv_slot;
        let mut var_ps0_ini: f64 = *var_ps0_ini_slot;
        let mut var_ps0_ini_dn0: f64 = *var_ps0_ini_dn0_slot;
        let mut var_ps0_ini_dn10: f64 = *var_ps0_ini_dn10_slot;
        let mut var_ps0_ini_dn11: f64 = *var_ps0_ini_dn11_slot;
        let mut var_ps0_ini_dn12: f64 = *var_ps0_ini_dn12_slot;
        let mut var_ps0_ini_dn2: f64 = *var_ps0_ini_dn2_slot;
        let mut var_ps0_ini_dn4: f64 = *var_ps0_ini_dn4_slot;
        let mut var_ps0_ini_dn5: f64 = *var_ps0_ini_dn5_slot;
        let mut var_ps0_ini_dn6: f64 = *var_ps0_ini_dn6_slot;
        let mut var_ps0_ini_dn8: f64 = *var_ps0_ini_dn8_slot;
        let mut var_ps0_ini_rv: f64 = *var_ps0_ini_rv_slot;
        let mut var_ps0_inia: f64 = *var_ps0_inia_slot;
        let mut var_ps0_inia_dn0: f64 = *var_ps0_inia_dn0_slot;
        let mut var_ps0_inia_dn10: f64 = *var_ps0_inia_dn10_slot;
        let mut var_ps0_inia_dn11: f64 = *var_ps0_inia_dn11_slot;
        let mut var_ps0_inia_dn12: f64 = *var_ps0_inia_dn12_slot;
        let mut var_ps0_inia_dn2: f64 = *var_ps0_inia_dn2_slot;
        let mut var_ps0_inia_dn4: f64 = *var_ps0_inia_dn4_slot;
        let mut var_ps0_inia_dn5: f64 = *var_ps0_inia_dn5_slot;
        let mut var_ps0_inia_dn6: f64 = *var_ps0_inia_dn6_slot;
        let mut var_ps0_inia_dn8: f64 = *var_ps0_inia_dn8_slot;
        let mut var_ps0_inia_rv: f64 = *var_ps0_inia_rv_slot;
        let mut var_ps0_inib: f64 = *var_ps0_inib_slot;
        let mut var_ps0_inib_dn0: f64 = *var_ps0_inib_dn0_slot;
        let mut var_ps0_inib_dn10: f64 = *var_ps0_inib_dn10_slot;
        let mut var_ps0_inib_dn11: f64 = *var_ps0_inib_dn11_slot;
        let mut var_ps0_inib_dn12: f64 = *var_ps0_inib_dn12_slot;
        let mut var_ps0_inib_dn2: f64 = *var_ps0_inib_dn2_slot;
        let mut var_ps0_inib_dn4: f64 = *var_ps0_inib_dn4_slot;
        let mut var_ps0_inib_dn5: f64 = *var_ps0_inib_dn5_slot;
        let mut var_ps0_inib_dn6: f64 = *var_ps0_inib_dn6_slot;
        let mut var_ps0_inib_dn8: f64 = *var_ps0_inib_dn8_slot;
        let mut var_ps0_inib_rv: f64 = *var_ps0_inib_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tx: f64 = *var_tx_slot;
        let mut var_tx_dn0: f64 = *var_tx_dn0_slot;
        let mut var_tx_dn10: f64 = *var_tx_dn10_slot;
        let mut var_tx_dn11: f64 = *var_tx_dn11_slot;
        let mut var_tx_dn12: f64 = *var_tx_dn12_slot;
        let mut var_tx_dn2: f64 = *var_tx_dn2_slot;
        let mut var_tx_dn4: f64 = *var_tx_dn4_slot;
        let mut var_tx_dn5: f64 = *var_tx_dn5_slot;
        let mut var_tx_dn6: f64 = *var_tx_dn6_slot;
        let mut var_tx_dn8: f64 = *var_tx_dn8_slot;
        let mut var_tx_rv: f64 = *var_tx_rv_slot;
        let mut var_x2: f64 = *var_x2_slot;
        let mut var_x2_dn0: f64 = *var_x2_dn0_slot;
        let mut var_x2_dn10: f64 = *var_x2_dn10_slot;
        let mut var_x2_dn11: f64 = *var_x2_dn11_slot;
        let mut var_x2_dn12: f64 = *var_x2_dn12_slot;
        let mut var_x2_dn2: f64 = *var_x2_dn2_slot;
        let mut var_x2_dn4: f64 = *var_x2_dn4_slot;
        let mut var_x2_dn5: f64 = *var_x2_dn5_slot;
        let mut var_x2_dn6: f64 = *var_x2_dn6_slot;
        let mut var_x2_dn8: f64 = *var_x2_dn8_slot;
        let mut var_x2_rv: f64 = *var_x2_rv_slot;

        let (assign7590_e7186, assign7590_e7186_d_n0, assign7590_e7186_d_n2, assign7590_e7186_d_n4, assign7590_e7186_d_n5, assign7590_e7186_d_n6, assign7590_e7186_d_n8, assign7590_e7186_d_n10, assign7590_e7186_d_n11, assign7590_e7186_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard79 != 0.0)) {
        let assign7590_e7184: f64 = (var_vbsbiz + var_phi_s0_bulk);
        (assign7590_e7184, (var_vbsbiz_dn0 + var_phi_s0_bulk_dn0), (var_vbsbiz_dn2 + var_phi_s0_bulk_dn2), (var_vbsbiz_dn4 + var_phi_s0_bulk_dn4), (var_vbsbiz_dn5 + var_phi_s0_bulk_dn5), (var_vbsbiz_dn6 + var_phi_s0_bulk_dn6), (var_vbsbiz_dn8 + var_phi_s0_bulk_dn8), (var_vbsbiz_dn10 + var_phi_s0_bulk_dn10), (var_vbsbiz_dn11 + var_phi_s0_bulk_dn11), (var_vbsbiz_dn12 + var_phi_s0_bulk_dn12),)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
        var_phi_s0_bulk = assign7590_e7186;
        var_phi_s0_bulk_dn0 = assign7590_e7186_d_n0;
        var_phi_s0_bulk_dn2 = assign7590_e7186_d_n2;
        var_phi_s0_bulk_dn4 = assign7590_e7186_d_n4;
        var_phi_s0_bulk_dn5 = assign7590_e7186_d_n5;
        var_phi_s0_bulk_dn6 = assign7590_e7186_d_n6;
        var_phi_s0_bulk_dn8 = assign7590_e7186_d_n8;
        var_phi_s0_bulk_dn10 = assign7590_e7186_d_n10;
        var_phi_s0_bulk_dn11 = assign7590_e7186_d_n11;
        var_phi_s0_bulk_dn12 = assign7590_e7186_d_n12;
        var_phi_s0_bulk_rv = 0.0;

        let (assign7600_e7197, assign7600_e7197_d_n0, assign7600_e7197_d_n2, assign7600_e7197_d_n4, assign7600_e7197_d_n5, assign7600_e7197_d_n6, assign7600_e7197_d_n8, assign7600_e7197_d_n10, assign7600_e7197_d_n11, assign7600_e7197_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard79 != 0.0)) {
        let assign7600_e7194: f64 = (var_q_s0_bulk / var_c_box);
        let assign7600_e7195: f64 = (var_phi_s0_bulk - assign7600_e7194);
        (assign7600_e7195, (var_phi_s0_bulk_dn0 - (var_q_s0_bulk_dn0 / var_c_box)), (var_phi_s0_bulk_dn2 - (var_q_s0_bulk_dn2 / var_c_box)), (var_phi_s0_bulk_dn4 - (var_q_s0_bulk_dn4 / var_c_box)), (var_phi_s0_bulk_dn5 - (var_q_s0_bulk_dn5 / var_c_box)), (var_phi_s0_bulk_dn6 - (var_q_s0_bulk_dn6 / var_c_box)), (var_phi_s0_bulk_dn8 - (var_q_s0_bulk_dn8 / var_c_box)), (var_phi_s0_bulk_dn10 - (var_q_s0_bulk_dn10 / var_c_box)), (var_phi_s0_bulk_dn11 - (var_q_s0_bulk_dn11 / var_c_box)), (var_phi_s0_bulk_dn12 - (var_q_s0_bulk_dn12 / var_c_box)),)
    } else {
        (var_phi_b0_soi, var_phi_b0_soi_dn0, var_phi_b0_soi_dn2, var_phi_b0_soi_dn4, var_phi_b0_soi_dn5, var_phi_b0_soi_dn6, var_phi_b0_soi_dn8, var_phi_b0_soi_dn10, var_phi_b0_soi_dn11, var_phi_b0_soi_dn12,)
    }
};
        var_phi_b0_soi = assign7600_e7197;
        var_phi_b0_soi_dn0 = assign7600_e7197_d_n0;
        var_phi_b0_soi_dn2 = assign7600_e7197_d_n2;
        var_phi_b0_soi_dn4 = assign7600_e7197_d_n4;
        var_phi_b0_soi_dn5 = assign7600_e7197_d_n5;
        var_phi_b0_soi_dn6 = assign7600_e7197_d_n6;
        var_phi_b0_soi_dn8 = assign7600_e7197_d_n8;
        var_phi_b0_soi_dn10 = assign7600_e7197_d_n10;
        var_phi_b0_soi_dn11 = assign7600_e7197_d_n11;
        var_phi_b0_soi_dn12 = assign7600_e7197_d_n12;
        var_phi_b0_soi_rv = 0.0;

        let (assign7610_e7216, assign7610_e7216_d_n0, assign7610_e7216_d_n2, assign7610_e7216_d_n4, assign7610_e7216_d_n5, assign7610_e7216_d_n6, assign7610_e7216_d_n8, assign7610_e7216_d_n10, assign7610_e7216_d_n11, assign7610_e7216_d_n12,) = {
    if (var_guard74 == 0.0) {
        let assign7610_e7205: f64 = (var_vgpz - var_vbs);
        let assign7610_e7206: f64 = (var_beta * assign7610_e7205);
        let assign7610_e7208: f64 = (assign7610_e7206 - 1.0);
        let assign7610_e7209: f64 = (4.0 * assign7610_e7208);
        let assign7610_e7212: f64 = (var_fac1p2 * var_beta2);
        let assign7610_e7213: f64 = (assign7610_e7209 / assign7610_e7212);
        let assign7610_e7214: f64 = (1.0 + assign7610_e7213);
        (assign7610_e7214, ((((4.0 * (var_beta * (var_vgpz_dn0 - var_vbs_dn0))) * assign7610_e7212) - (assign7610_e7209 * (var_fac1p2_dn0 * var_beta2))) / (assign7610_e7212 * assign7610_e7212)), ((((4.0 * (var_beta * (var_vgpz_dn2 - var_vbs_dn2))) * assign7610_e7212) - (assign7610_e7209 * (var_fac1p2_dn2 * var_beta2))) / (assign7610_e7212 * assign7610_e7212)), ((((4.0 * ((var_beta_dn4 * assign7610_e7205) + (var_beta * (var_vgpz_dn4 - var_vbs_dn4)))) * assign7610_e7212) - (assign7610_e7209 * ((var_fac1p2_dn4 * var_beta2) + (var_fac1p2 * var_beta2_dn4)))) / (assign7610_e7212 * assign7610_e7212)), ((((4.0 * (var_beta * (var_vgpz_dn5 - var_vbs_dn5))) * assign7610_e7212) - (assign7610_e7209 * (var_fac1p2_dn5 * var_beta2))) / (assign7610_e7212 * assign7610_e7212)), ((((4.0 * (var_beta * (var_vgpz_dn6 - var_vbs_dn6))) * assign7610_e7212) - (assign7610_e7209 * (var_fac1p2_dn6 * var_beta2))) / (assign7610_e7212 * assign7610_e7212)), ((((4.0 * (var_beta * (var_vgpz_dn8 - var_vbs_dn8))) * assign7610_e7212) - (assign7610_e7209 * (var_fac1p2_dn8 * var_beta2))) / (assign7610_e7212 * assign7610_e7212)), ((((4.0 * (var_beta * (var_vgpz_dn10 - var_vbs_dn10))) * assign7610_e7212) - (assign7610_e7209 * (var_fac1p2_dn10 * var_beta2))) / (assign7610_e7212 * assign7610_e7212)), ((((4.0 * (var_beta * (var_vgpz_dn11 - var_vbs_dn11))) * assign7610_e7212) - (assign7610_e7209 * (var_fac1p2_dn11 * var_beta2))) / (assign7610_e7212 * assign7610_e7212)), ((((4.0 * (var_beta * (var_vgpz_dn12 - var_vbs_dn12))) * assign7610_e7212) - (assign7610_e7209 * (var_fac1p2_dn12 * var_beta2))) / (assign7610_e7212 * assign7610_e7212)),)
    } else {
        (var_tx, var_tx_dn0, var_tx_dn2, var_tx_dn4, var_tx_dn5, var_tx_dn6, var_tx_dn8, var_tx_dn10, var_tx_dn11, var_tx_dn12,)
    }
};
        var_tx = assign7610_e7216;
        var_tx_dn0 = assign7610_e7216_d_n0;
        var_tx_dn2 = assign7610_e7216_d_n2;
        var_tx_dn4 = assign7610_e7216_d_n4;
        var_tx_dn5 = assign7610_e7216_d_n5;
        var_tx_dn6 = assign7610_e7216_d_n6;
        var_tx_dn8 = assign7610_e7216_d_n8;
        var_tx_dn10 = assign7610_e7216_d_n10;
        var_tx_dn11 = assign7610_e7216_d_n11;
        var_tx_dn12 = assign7610_e7216_d_n12;
        var_tx_rv = 0.0;

        let (assign7620_e7230, assign7620_e7230_d_n0, assign7620_e7230_d_n2, assign7620_e7230_d_n4, assign7620_e7230_d_n5, assign7620_e7230_d_n6, assign7620_e7230_d_n8, assign7620_e7230_d_n10, assign7620_e7230_d_n11, assign7620_e7230_d_n12,) = {
    if (var_guard74 == 0.0) {
        let assign7620_e7222: f64 = (10.0 * 2.220446049250313e-16);
        let (assign7620_e7228, assign7620_e7228_d_n0, assign7620_e7228_d_n2, assign7620_e7228_d_n4, assign7620_e7228_d_n5, assign7620_e7228_d_n6, assign7620_e7228_d_n8, assign7620_e7228_d_n10, assign7620_e7228_d_n11, assign7620_e7228_d_n12,) = {
            if (var_tx >= assign7620_e7222) {
                (var_tx, var_tx_dn0, var_tx_dn2, var_tx_dn4, var_tx_dn5, var_tx_dn6, var_tx_dn8, var_tx_dn10, var_tx_dn11, var_tx_dn12,)
            } else {
                let assign7620_e7227: f64 = (10.0 * 2.220446049250313e-16);
                (assign7620_e7227, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign7620_e7228, assign7620_e7228_d_n0, assign7620_e7228_d_n2, assign7620_e7228_d_n4, assign7620_e7228_d_n5, assign7620_e7228_d_n6, assign7620_e7228_d_n8, assign7620_e7228_d_n10, assign7620_e7228_d_n11, assign7620_e7228_d_n12,)
    } else {
        (var_tx, var_tx_dn0, var_tx_dn2, var_tx_dn4, var_tx_dn5, var_tx_dn6, var_tx_dn8, var_tx_dn10, var_tx_dn11, var_tx_dn12,)
    }
};
        var_tx = assign7620_e7230;
        var_tx_dn0 = assign7620_e7230_d_n0;
        var_tx_dn2 = assign7620_e7230_d_n2;
        var_tx_dn4 = assign7620_e7230_d_n4;
        var_tx_dn5 = assign7620_e7230_d_n5;
        var_tx_dn6 = assign7620_e7230_d_n6;
        var_tx_dn8 = assign7620_e7230_d_n8;
        var_tx_dn10 = assign7620_e7230_d_n10;
        var_tx_dn11 = assign7620_e7230_d_n11;
        var_tx_dn12 = assign7620_e7230_d_n12;
        var_tx_rv = 0.0;

        let (assign7630_e7246, assign7630_e7246_d_n0, assign7630_e7246_d_n2, assign7630_e7246_d_n4, assign7630_e7246_d_n5, assign7630_e7246_d_n6, assign7630_e7246_d_n8, assign7630_e7246_d_n10, assign7630_e7246_d_n11, assign7630_e7246_d_n12,) = {
    if (var_guard74 == 0.0) {
        let assign7630_e7236: f64 = (var_fac1p2 * var_beta);
        let assign7630_e7238: f64 = (assign7630_e7236 * 0.5);
        let assign7630_e7241: f64 = (var_tx).sqrt();
        let assign7630_e7242: f64 = (1.0 - assign7630_e7241);
        let assign7630_e7243: f64 = (assign7630_e7238 * assign7630_e7242);
        let assign7630_e7244: f64 = (var_vgpz + assign7630_e7243);
        (assign7630_e7244, (var_vgpz_dn0 + ((((var_fac1p2_dn0 * var_beta) * 0.5) * assign7630_e7242) + (assign7630_e7238 * (-(var_tx_dn0 / (2.0 * assign7630_e7241)))))), (var_vgpz_dn2 + ((((var_fac1p2_dn2 * var_beta) * 0.5) * assign7630_e7242) + (assign7630_e7238 * (-(var_tx_dn2 / (2.0 * assign7630_e7241)))))), (var_vgpz_dn4 + (((((var_fac1p2_dn4 * var_beta) + (var_fac1p2 * var_beta_dn4)) * 0.5) * assign7630_e7242) + (assign7630_e7238 * (-(var_tx_dn4 / (2.0 * assign7630_e7241)))))), (var_vgpz_dn5 + ((((var_fac1p2_dn5 * var_beta) * 0.5) * assign7630_e7242) + (assign7630_e7238 * (-(var_tx_dn5 / (2.0 * assign7630_e7241)))))), (var_vgpz_dn6 + ((((var_fac1p2_dn6 * var_beta) * 0.5) * assign7630_e7242) + (assign7630_e7238 * (-(var_tx_dn6 / (2.0 * assign7630_e7241)))))), (var_vgpz_dn8 + ((((var_fac1p2_dn8 * var_beta) * 0.5) * assign7630_e7242) + (assign7630_e7238 * (-(var_tx_dn8 / (2.0 * assign7630_e7241)))))), (var_vgpz_dn10 + ((((var_fac1p2_dn10 * var_beta) * 0.5) * assign7630_e7242) + (assign7630_e7238 * (-(var_tx_dn10 / (2.0 * assign7630_e7241)))))), (var_vgpz_dn11 + ((((var_fac1p2_dn11 * var_beta) * 0.5) * assign7630_e7242) + (assign7630_e7238 * (-(var_tx_dn11 / (2.0 * assign7630_e7241)))))), (var_vgpz_dn12 + ((((var_fac1p2_dn12 * var_beta) * 0.5) * assign7630_e7242) + (assign7630_e7238 * (-(var_tx_dn12 / (2.0 * assign7630_e7241)))))),)
    } else {
        (var_ps0_inia, var_ps0_inia_dn0, var_ps0_inia_dn2, var_ps0_inia_dn4, var_ps0_inia_dn5, var_ps0_inia_dn6, var_ps0_inia_dn8, var_ps0_inia_dn10, var_ps0_inia_dn11, var_ps0_inia_dn12,)
    }
};
        var_ps0_inia = assign7630_e7246;
        var_ps0_inia_dn0 = assign7630_e7246_d_n0;
        var_ps0_inia_dn2 = assign7630_e7246_d_n2;
        var_ps0_inia_dn4 = assign7630_e7246_d_n4;
        var_ps0_inia_dn5 = assign7630_e7246_d_n5;
        var_ps0_inia_dn6 = assign7630_e7246_d_n6;
        var_ps0_inia_dn8 = assign7630_e7246_d_n8;
        var_ps0_inia_dn10 = assign7630_e7246_d_n10;
        var_ps0_inia_dn11 = assign7630_e7246_d_n11;
        var_ps0_inia_dn12 = assign7630_e7246_d_n12;
        var_ps0_inia_rv = 0.0;

        let (assign7640_e7253, assign7640_e7253_d_n0, assign7640_e7253_d_n2, assign7640_e7253_d_n4, assign7640_e7253_d_n5, assign7640_e7253_d_n6, assign7640_e7253_d_n8, assign7640_e7253_d_n10, assign7640_e7253_d_n11, assign7640_e7253_d_n12,) = {
    if (var_guard74 == 0.0) {
        let assign7640_e7251: f64 = (1.0 / var_c_fox);
        (assign7640_e7251, (-(var_c_fox_dn0 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn2 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn4 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn5 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn6 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn8 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn10 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn11 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn12 / (var_c_fox * var_c_fox))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
        var_t0 = assign7640_e7253;
        var_t0_dn0 = assign7640_e7253_d_n0;
        var_t0_dn2 = assign7640_e7253_d_n2;
        var_t0_dn4 = assign7640_e7253_d_n4;
        var_t0_dn5 = assign7640_e7253_d_n5;
        var_t0_dn6 = assign7640_e7253_d_n6;
        var_t0_dn8 = assign7640_e7253_d_n8;
        var_t0_dn10 = assign7640_e7253_d_n10;
        var_t0_dn11 = assign7640_e7253_d_n11;
        var_t0_dn12 = assign7640_e7253_d_n12;
        var_t0_rv = 0.0;

        let (assign7650_e7260, assign7650_e7260_d_n0, assign7650_e7260_d_n2, assign7650_e7260_d_n4, assign7650_e7260_d_n5, assign7650_e7260_d_n6, assign7650_e7260_d_n8, assign7650_e7260_d_n10, assign7650_e7260_d_n11, assign7650_e7260_d_n12,) = {
    if (var_guard74 == 0.0) {
        let assign7650_e7258: f64 = (p.p227 / 1.034943e-10);
        (assign7650_e7258, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign7650_e7260;
        var_t1_dn0 = assign7650_e7260_d_n0;
        var_t1_dn2 = assign7650_e7260_d_n2;
        var_t1_dn4 = assign7650_e7260_d_n4;
        var_t1_dn5 = assign7650_e7260_d_n5;
        var_t1_dn6 = assign7650_e7260_d_n6;
        var_t1_dn8 = assign7650_e7260_d_n8;
        var_t1_dn10 = assign7650_e7260_d_n10;
        var_t1_dn11 = assign7650_e7260_d_n11;
        var_t1_dn12 = assign7650_e7260_d_n12;
        var_t1_rv = 0.0;

        let (assign7660_e7267, assign7660_e7267_d_n0, assign7660_e7267_d_n2, assign7660_e7267_d_n4, assign7660_e7267_d_n5, assign7660_e7267_d_n6, assign7660_e7267_d_n8, assign7660_e7267_d_n10, assign7660_e7267_d_n11, assign7660_e7267_d_n12,) = {
    if (var_guard74 == 0.0) {
        let assign7660_e7265: f64 = (1.0 / var_c_box);
        (assign7660_e7265, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign7660_e7267;
        var_t2_dn0 = assign7660_e7267_d_n0;
        var_t2_dn2 = assign7660_e7267_d_n2;
        var_t2_dn4 = assign7660_e7267_d_n4;
        var_t2_dn5 = assign7660_e7267_d_n5;
        var_t2_dn6 = assign7660_e7267_d_n6;
        var_t2_dn8 = assign7660_e7267_d_n8;
        var_t2_dn10 = assign7660_e7267_d_n10;
        var_t2_dn11 = assign7660_e7267_d_n11;
        var_t2_dn12 = assign7660_e7267_d_n12;
        var_t2_rv = 0.0;

        let (assign7670_e7278, assign7670_e7278_d_n0, assign7670_e7278_d_n2, assign7670_e7278_d_n4, assign7670_e7278_d_n5, assign7670_e7278_d_n6, assign7670_e7278_d_n8, assign7670_e7278_d_n10, assign7670_e7278_d_n11, assign7670_e7278_d_n12,) = {
    if (var_guard74 == 0.0) {
        let assign7670_e7273: f64 = (var_t0 + var_t1);
        let assign7670_e7275: f64 = (assign7670_e7273 + var_t2);
        let assign7670_e7276: f64 = (1.0 / assign7670_e7275);
        (assign7670_e7276, (-(((var_t0_dn0 + var_t1_dn0) + var_t2_dn0) / (assign7670_e7275 * assign7670_e7275))), (-(((var_t0_dn2 + var_t1_dn2) + var_t2_dn2) / (assign7670_e7275 * assign7670_e7275))), (-(((var_t0_dn4 + var_t1_dn4) + var_t2_dn4) / (assign7670_e7275 * assign7670_e7275))), (-(((var_t0_dn5 + var_t1_dn5) + var_t2_dn5) / (assign7670_e7275 * assign7670_e7275))), (-(((var_t0_dn6 + var_t1_dn6) + var_t2_dn6) / (assign7670_e7275 * assign7670_e7275))), (-(((var_t0_dn8 + var_t1_dn8) + var_t2_dn8) / (assign7670_e7275 * assign7670_e7275))), (-(((var_t0_dn10 + var_t1_dn10) + var_t2_dn10) / (assign7670_e7275 * assign7670_e7275))), (-(((var_t0_dn11 + var_t1_dn11) + var_t2_dn11) / (assign7670_e7275 * assign7670_e7275))), (-(((var_t0_dn12 + var_t1_dn12) + var_t2_dn12) / (assign7670_e7275 * assign7670_e7275))),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign7670_e7278;
        var_t3_dn0 = assign7670_e7278_d_n0;
        var_t3_dn2 = assign7670_e7278_d_n2;
        var_t3_dn4 = assign7670_e7278_d_n4;
        var_t3_dn5 = assign7670_e7278_d_n5;
        var_t3_dn6 = assign7670_e7278_d_n6;
        var_t3_dn8 = assign7670_e7278_d_n8;
        var_t3_dn10 = assign7670_e7278_d_n10;
        var_t3_dn11 = assign7670_e7278_d_n11;
        var_t3_dn12 = assign7670_e7278_d_n12;
        var_t3_rv = 0.0;

        let assign7680_e7281: f64 = (var_vgs - var_shift);
        let assign7680_e7283: f64 = if assign7680_e7281 <= var_vth { 1.0 } else { 0.0 };
        var_guard92 = assign7680_e7283;
        var_guard92_rv = 0.0;

        let (assign7690_e7304, assign7690_e7304_d_n0, assign7690_e7304_d_n2, assign7690_e7304_d_n4, assign7690_e7304_d_n5, assign7690_e7304_d_n6, assign7690_e7304_d_n8, assign7690_e7304_d_n10, assign7690_e7304_d_n11, assign7690_e7304_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard92 != 0.0)) {
        let (assign7690_e7302, assign7690_e7302_d_n0, assign7690_e7302_d_n2, assign7690_e7302_d_n4, assign7690_e7302_d_n5, assign7690_e7302_d_n6, assign7690_e7302_d_n8, assign7690_e7302_d_n10, assign7690_e7302_d_n11, assign7690_e7302_d_n12,) = {
            if (var_ps0_inia > 0.0) {
                let assign7690_e7293: f64 = (1.6021918e-19 * var_uc_nsubs);
                let assign7690_e7295: f64 = (assign7690_e7293 * 2.0);
                let assign7690_e7297: f64 = (assign7690_e7295 * 1.034943e-10);
                let assign7690_e7299: f64 = (assign7690_e7297 * var_ps0_inia);
                let assign7690_e7300: f64 = (assign7690_e7299).sqrt();
                (assign7690_e7300, ((((((1.6021918e-19 * var_uc_nsubs_dn0) * 2.0) * 1.034943e-10) * var_ps0_inia) + (assign7690_e7297 * var_ps0_inia_dn0)) / (2.0 * assign7690_e7300)), ((((((1.6021918e-19 * var_uc_nsubs_dn2) * 2.0) * 1.034943e-10) * var_ps0_inia) + (assign7690_e7297 * var_ps0_inia_dn2)) / (2.0 * assign7690_e7300)), ((((((1.6021918e-19 * var_uc_nsubs_dn4) * 2.0) * 1.034943e-10) * var_ps0_inia) + (assign7690_e7297 * var_ps0_inia_dn4)) / (2.0 * assign7690_e7300)), ((((((1.6021918e-19 * var_uc_nsubs_dn5) * 2.0) * 1.034943e-10) * var_ps0_inia) + (assign7690_e7297 * var_ps0_inia_dn5)) / (2.0 * assign7690_e7300)), ((((((1.6021918e-19 * var_uc_nsubs_dn6) * 2.0) * 1.034943e-10) * var_ps0_inia) + (assign7690_e7297 * var_ps0_inia_dn6)) / (2.0 * assign7690_e7300)), ((((((1.6021918e-19 * var_uc_nsubs_dn8) * 2.0) * 1.034943e-10) * var_ps0_inia) + (assign7690_e7297 * var_ps0_inia_dn8)) / (2.0 * assign7690_e7300)), ((((((1.6021918e-19 * var_uc_nsubs_dn10) * 2.0) * 1.034943e-10) * var_ps0_inia) + (assign7690_e7297 * var_ps0_inia_dn10)) / (2.0 * assign7690_e7300)), ((((((1.6021918e-19 * var_uc_nsubs_dn11) * 2.0) * 1.034943e-10) * var_ps0_inia) + (assign7690_e7297 * var_ps0_inia_dn11)) / (2.0 * assign7690_e7300)), ((((((1.6021918e-19 * var_uc_nsubs_dn12) * 2.0) * 1.034943e-10) * var_ps0_inia) + (assign7690_e7297 * var_ps0_inia_dn12)) / (2.0 * assign7690_e7300)),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign7690_e7302, assign7690_e7302_d_n0, assign7690_e7302_d_n2, assign7690_e7302_d_n4, assign7690_e7302_d_n5, assign7690_e7302_d_n6, assign7690_e7302_d_n8, assign7690_e7302_d_n10, assign7690_e7302_d_n11, assign7690_e7302_d_n12,)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
        var_t5 = assign7690_e7304;
        var_t5_dn0 = assign7690_e7304_d_n0;
        var_t5_dn2 = assign7690_e7304_d_n2;
        var_t5_dn4 = assign7690_e7304_d_n4;
        var_t5_dn5 = assign7690_e7304_d_n5;
        var_t5_dn6 = assign7690_e7304_d_n6;
        var_t5_dn8 = assign7690_e7304_d_n8;
        var_t5_dn10 = assign7690_e7304_d_n10;
        var_t5_dn11 = assign7690_e7304_d_n11;
        var_t5_dn12 = assign7690_e7304_d_n12;
        var_t5_rv = 0.0;

        let (assign7700_e7316, assign7700_e7316_d_n0, assign7700_e7316_d_n2, assign7700_e7316_d_n4, assign7700_e7316_d_n5, assign7700_e7316_d_n6, assign7700_e7316_d_n8, assign7700_e7316_d_n10, assign7700_e7316_d_n11, assign7700_e7316_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard92 != 0.0)) {
        let (assign7700_e7314, assign7700_e7314_d_n0, assign7700_e7314_d_n2, assign7700_e7314_d_n4, assign7700_e7314_d_n5, assign7700_e7314_d_n6, assign7700_e7314_d_n8, assign7700_e7314_d_n10, assign7700_e7314_d_n11, assign7700_e7314_d_n12,) = {
            if (var_q_fd_soi <= var_t5) {
                (var_q_fd_soi, var_q_fd_soi_dn0, var_q_fd_soi_dn2, var_q_fd_soi_dn4, var_q_fd_soi_dn5, var_q_fd_soi_dn6, var_q_fd_soi_dn8, var_q_fd_soi_dn10, var_q_fd_soi_dn11, var_q_fd_soi_dn12,)
            } else {
                (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
            }
        };
        (assign7700_e7314, assign7700_e7314_d_n0, assign7700_e7314_d_n2, assign7700_e7314_d_n4, assign7700_e7314_d_n5, assign7700_e7314_d_n6, assign7700_e7314_d_n8, assign7700_e7314_d_n10, assign7700_e7314_d_n11, assign7700_e7314_d_n12,)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
        var_t5 = assign7700_e7316;
        var_t5_dn0 = assign7700_e7316_d_n0;
        var_t5_dn2 = assign7700_e7316_d_n2;
        var_t5_dn4 = assign7700_e7316_d_n4;
        var_t5_dn5 = assign7700_e7316_d_n5;
        var_t5_dn6 = assign7700_e7316_d_n6;
        var_t5_dn8 = assign7700_e7316_d_n8;
        var_t5_dn10 = assign7700_e7316_d_n10;
        var_t5_dn11 = assign7700_e7316_d_n11;
        var_t5_dn12 = assign7700_e7316_d_n12;
        var_t5_rv = 0.0;

        let (assign7710_e7336, assign7710_e7336_d_n0, assign7710_e7336_d_n2, assign7710_e7336_d_n4, assign7710_e7336_d_n5, assign7710_e7336_d_n6, assign7710_e7336_d_n8, assign7710_e7336_d_n10, assign7710_e7336_d_n11, assign7710_e7336_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard92 != 0.0)) {
        let assign7710_e7324: f64 = (var_vgpz - var_vbsbiz);
        let assign7710_e7328: f64 = (0.5 * var_t1);
        let assign7710_e7329: f64 = (var_t2 + assign7710_e7328);
        let assign7710_e7331: f64 = (-var_t5);
        let assign7710_e7332: f64 = (assign7710_e7329 * assign7710_e7331);
        let assign7710_e7333: f64 = (assign7710_e7324 + assign7710_e7332);
        let assign7710_e7334: f64 = (var_t3 * assign7710_e7333);
        (assign7710_e7334, ((var_t3_dn0 * assign7710_e7333) + (var_t3 * ((var_vgpz_dn0 - var_vbsbiz_dn0) + (((var_t2_dn0 + (0.5 * var_t1_dn0)) * assign7710_e7331) + (assign7710_e7329 * (-var_t5_dn0)))))), ((var_t3_dn2 * assign7710_e7333) + (var_t3 * ((var_vgpz_dn2 - var_vbsbiz_dn2) + (((var_t2_dn2 + (0.5 * var_t1_dn2)) * assign7710_e7331) + (assign7710_e7329 * (-var_t5_dn2)))))), ((var_t3_dn4 * assign7710_e7333) + (var_t3 * ((var_vgpz_dn4 - var_vbsbiz_dn4) + (((var_t2_dn4 + (0.5 * var_t1_dn4)) * assign7710_e7331) + (assign7710_e7329 * (-var_t5_dn4)))))), ((var_t3_dn5 * assign7710_e7333) + (var_t3 * ((var_vgpz_dn5 - var_vbsbiz_dn5) + (((var_t2_dn5 + (0.5 * var_t1_dn5)) * assign7710_e7331) + (assign7710_e7329 * (-var_t5_dn5)))))), ((var_t3_dn6 * assign7710_e7333) + (var_t3 * ((var_vgpz_dn6 - var_vbsbiz_dn6) + (((var_t2_dn6 + (0.5 * var_t1_dn6)) * assign7710_e7331) + (assign7710_e7329 * (-var_t5_dn6)))))), ((var_t3_dn8 * assign7710_e7333) + (var_t3 * ((var_vgpz_dn8 - var_vbsbiz_dn8) + (((var_t2_dn8 + (0.5 * var_t1_dn8)) * assign7710_e7331) + (assign7710_e7329 * (-var_t5_dn8)))))), ((var_t3_dn10 * assign7710_e7333) + (var_t3 * ((var_vgpz_dn10 - var_vbsbiz_dn10) + (((var_t2_dn10 + (0.5 * var_t1_dn10)) * assign7710_e7331) + (assign7710_e7329 * (-var_t5_dn10)))))), ((var_t3_dn11 * assign7710_e7333) + (var_t3 * ((var_vgpz_dn11 - var_vbsbiz_dn11) + (((var_t2_dn11 + (0.5 * var_t1_dn11)) * assign7710_e7331) + (assign7710_e7329 * (-var_t5_dn11)))))), ((var_t3_dn12 * assign7710_e7333) + (var_t3 * ((var_vgpz_dn12 - var_vbsbiz_dn12) + (((var_t2_dn12 + (0.5 * var_t1_dn12)) * assign7710_e7331) + (assign7710_e7329 * (-var_t5_dn12)))))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
        var_t4 = assign7710_e7336;
        var_t4_dn0 = assign7710_e7336_d_n0;
        var_t4_dn2 = assign7710_e7336_d_n2;
        var_t4_dn4 = assign7710_e7336_d_n4;
        var_t4_dn5 = assign7710_e7336_d_n5;
        var_t4_dn6 = assign7710_e7336_d_n6;
        var_t4_dn8 = assign7710_e7336_d_n8;
        var_t4_dn10 = assign7710_e7336_d_n10;
        var_t4_dn11 = assign7710_e7336_d_n11;
        var_t4_dn12 = assign7710_e7336_d_n12;
        var_t4_rv = 0.0;

        let (assign7720_e7357, assign7720_e7357_d_n0, assign7720_e7357_d_n2, assign7720_e7357_d_n4, assign7720_e7357_d_n5, assign7720_e7357_d_n6, assign7720_e7357_d_n8, assign7720_e7357_d_n10, assign7720_e7357_d_n11, assign7720_e7357_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard92 == 0.0)) {
        let assign7720_e7345: f64 = (var_vgpz - var_vbsbiz);
        let assign7720_e7349: f64 = (0.5 * var_t1);
        let assign7720_e7350: f64 = (var_t2 + assign7720_e7349);
        let assign7720_e7352: f64 = (-var_q_fd_soi);
        let assign7720_e7353: f64 = (assign7720_e7350 * assign7720_e7352);
        let assign7720_e7354: f64 = (assign7720_e7345 + assign7720_e7353);
        let assign7720_e7355: f64 = (var_t3 * assign7720_e7354);
        (assign7720_e7355, ((var_t3_dn0 * assign7720_e7354) + (var_t3 * ((var_vgpz_dn0 - var_vbsbiz_dn0) + (((var_t2_dn0 + (0.5 * var_t1_dn0)) * assign7720_e7352) + (assign7720_e7350 * (-var_q_fd_soi_dn0)))))), ((var_t3_dn2 * assign7720_e7354) + (var_t3 * ((var_vgpz_dn2 - var_vbsbiz_dn2) + (((var_t2_dn2 + (0.5 * var_t1_dn2)) * assign7720_e7352) + (assign7720_e7350 * (-var_q_fd_soi_dn2)))))), ((var_t3_dn4 * assign7720_e7354) + (var_t3 * ((var_vgpz_dn4 - var_vbsbiz_dn4) + (((var_t2_dn4 + (0.5 * var_t1_dn4)) * assign7720_e7352) + (assign7720_e7350 * (-var_q_fd_soi_dn4)))))), ((var_t3_dn5 * assign7720_e7354) + (var_t3 * ((var_vgpz_dn5 - var_vbsbiz_dn5) + (((var_t2_dn5 + (0.5 * var_t1_dn5)) * assign7720_e7352) + (assign7720_e7350 * (-var_q_fd_soi_dn5)))))), ((var_t3_dn6 * assign7720_e7354) + (var_t3 * ((var_vgpz_dn6 - var_vbsbiz_dn6) + (((var_t2_dn6 + (0.5 * var_t1_dn6)) * assign7720_e7352) + (assign7720_e7350 * (-var_q_fd_soi_dn6)))))), ((var_t3_dn8 * assign7720_e7354) + (var_t3 * ((var_vgpz_dn8 - var_vbsbiz_dn8) + (((var_t2_dn8 + (0.5 * var_t1_dn8)) * assign7720_e7352) + (assign7720_e7350 * (-var_q_fd_soi_dn8)))))), ((var_t3_dn10 * assign7720_e7354) + (var_t3 * ((var_vgpz_dn10 - var_vbsbiz_dn10) + (((var_t2_dn10 + (0.5 * var_t1_dn10)) * assign7720_e7352) + (assign7720_e7350 * (-var_q_fd_soi_dn10)))))), ((var_t3_dn11 * assign7720_e7354) + (var_t3 * ((var_vgpz_dn11 - var_vbsbiz_dn11) + (((var_t2_dn11 + (0.5 * var_t1_dn11)) * assign7720_e7352) + (assign7720_e7350 * (-var_q_fd_soi_dn11)))))), ((var_t3_dn12 * assign7720_e7354) + (var_t3 * ((var_vgpz_dn12 - var_vbsbiz_dn12) + (((var_t2_dn12 + (0.5 * var_t1_dn12)) * assign7720_e7352) + (assign7720_e7350 * (-var_q_fd_soi_dn12)))))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
        var_t4 = assign7720_e7357;
        var_t4_dn0 = assign7720_e7357_d_n0;
        var_t4_dn2 = assign7720_e7357_d_n2;
        var_t4_dn4 = assign7720_e7357_d_n4;
        var_t4_dn5 = assign7720_e7357_d_n5;
        var_t4_dn6 = assign7720_e7357_d_n6;
        var_t4_dn8 = assign7720_e7357_d_n8;
        var_t4_dn10 = assign7720_e7357_d_n10;
        var_t4_dn11 = assign7720_e7357_d_n11;
        var_t4_dn12 = assign7720_e7357_d_n12;
        var_t4_rv = 0.0;

        let (assign7730_e7366, assign7730_e7366_d_n0, assign7730_e7366_d_n2, assign7730_e7366_d_n4, assign7730_e7366_d_n5, assign7730_e7366_d_n6, assign7730_e7366_d_n8, assign7730_e7366_d_n10, assign7730_e7366_d_n11, assign7730_e7366_d_n12,) = {
    if (var_guard74 == 0.0) {
        let assign7730_e7363: f64 = (var_t4 / var_c_fox);
        let assign7730_e7364: f64 = (var_vgpz - assign7730_e7363);
        (assign7730_e7364, (var_vgpz_dn0 - (((var_t4_dn0 * var_c_fox) - (var_t4 * var_c_fox_dn0)) / (var_c_fox * var_c_fox))), (var_vgpz_dn2 - (((var_t4_dn2 * var_c_fox) - (var_t4 * var_c_fox_dn2)) / (var_c_fox * var_c_fox))), (var_vgpz_dn4 - (((var_t4_dn4 * var_c_fox) - (var_t4 * var_c_fox_dn4)) / (var_c_fox * var_c_fox))), (var_vgpz_dn5 - (((var_t4_dn5 * var_c_fox) - (var_t4 * var_c_fox_dn5)) / (var_c_fox * var_c_fox))), (var_vgpz_dn6 - (((var_t4_dn6 * var_c_fox) - (var_t4 * var_c_fox_dn6)) / (var_c_fox * var_c_fox))), (var_vgpz_dn8 - (((var_t4_dn8 * var_c_fox) - (var_t4 * var_c_fox_dn8)) / (var_c_fox * var_c_fox))), (var_vgpz_dn10 - (((var_t4_dn10 * var_c_fox) - (var_t4 * var_c_fox_dn10)) / (var_c_fox * var_c_fox))), (var_vgpz_dn11 - (((var_t4_dn11 * var_c_fox) - (var_t4 * var_c_fox_dn11)) / (var_c_fox * var_c_fox))), (var_vgpz_dn12 - (((var_t4_dn12 * var_c_fox) - (var_t4 * var_c_fox_dn12)) / (var_c_fox * var_c_fox))),)
    } else {
        (var_ps0_inia, var_ps0_inia_dn0, var_ps0_inia_dn2, var_ps0_inia_dn4, var_ps0_inia_dn5, var_ps0_inia_dn6, var_ps0_inia_dn8, var_ps0_inia_dn10, var_ps0_inia_dn11, var_ps0_inia_dn12,)
    }
};
        var_ps0_inia = assign7730_e7366;
        var_ps0_inia_dn0 = assign7730_e7366_d_n0;
        var_ps0_inia_dn2 = assign7730_e7366_d_n2;
        var_ps0_inia_dn4 = assign7730_e7366_d_n4;
        var_ps0_inia_dn5 = assign7730_e7366_d_n5;
        var_ps0_inia_dn6 = assign7730_e7366_d_n6;
        var_ps0_inia_dn8 = assign7730_e7366_d_n8;
        var_ps0_inia_dn10 = assign7730_e7366_d_n10;
        var_ps0_inia_dn11 = assign7730_e7366_d_n11;
        var_ps0_inia_dn12 = assign7730_e7366_d_n12;
        var_ps0_inia_rv = 0.0;

        let (assign7740_e7371, assign7740_e7371_d_n0, assign7740_e7371_d_n2, assign7740_e7371_d_n4, assign7740_e7371_d_n5, assign7740_e7371_d_n6, assign7740_e7371_d_n8, assign7740_e7371_d_n10, assign7740_e7371_d_n11, assign7740_e7371_d_n12,) = {
    if (var_guard74 == 0.0) {
        (var_ps0_inia, var_ps0_inia_dn0, var_ps0_inia_dn2, var_ps0_inia_dn4, var_ps0_inia_dn5, var_ps0_inia_dn6, var_ps0_inia_dn8, var_ps0_inia_dn10, var_ps0_inia_dn11, var_ps0_inia_dn12,)
    } else {
        (var_ps0_ini, var_ps0_ini_dn0, var_ps0_ini_dn2, var_ps0_ini_dn4, var_ps0_ini_dn5, var_ps0_ini_dn6, var_ps0_ini_dn8, var_ps0_ini_dn10, var_ps0_ini_dn11, var_ps0_ini_dn12,)
    }
};
        var_ps0_ini = assign7740_e7371;
        var_ps0_ini_dn0 = assign7740_e7371_d_n0;
        var_ps0_ini_dn2 = assign7740_e7371_d_n2;
        var_ps0_ini_dn4 = assign7740_e7371_d_n4;
        var_ps0_ini_dn5 = assign7740_e7371_d_n5;
        var_ps0_ini_dn6 = assign7740_e7371_d_n6;
        var_ps0_ini_dn8 = assign7740_e7371_d_n8;
        var_ps0_ini_dn10 = assign7740_e7371_d_n10;
        var_ps0_ini_dn11 = assign7740_e7371_d_n11;
        var_ps0_ini_dn12 = assign7740_e7371_d_n12;
        var_ps0_ini_rv = 0.0;

        let assign7750_e7374: f64 = (var_vgs - var_shift);
        let assign7750_e7376: f64 = if assign7750_e7374 > var_vth { 1.0 } else { 0.0 };
        var_guard93 = assign7750_e7376;
        var_guard93_rv = 0.0;

        let (assign7760_e7387, assign7760_e7387_d_n0, assign7760_e7387_d_n2, assign7760_e7387_d_n4, assign7760_e7387_d_n5, assign7760_e7387_d_n6, assign7760_e7387_d_n8, assign7760_e7387_d_n10, assign7760_e7387_d_n11, assign7760_e7387_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard93 != 0.0)) {
        let assign7760_e7383: f64 = (1.0 / var_cnst1soi);
        let assign7760_e7385: f64 = (assign7760_e7383 / var_cnstc_foxi);
        (assign7760_e7385, ((((-(var_cnst1soi_dn0 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7760_e7383 * var_cnstc_foxi_dn0)) / (var_cnstc_foxi * var_cnstc_foxi)), ((((-(var_cnst1soi_dn2 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7760_e7383 * var_cnstc_foxi_dn2)) / (var_cnstc_foxi * var_cnstc_foxi)), ((((-(var_cnst1soi_dn4 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7760_e7383 * var_cnstc_foxi_dn4)) / (var_cnstc_foxi * var_cnstc_foxi)), ((((-(var_cnst1soi_dn5 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7760_e7383 * var_cnstc_foxi_dn5)) / (var_cnstc_foxi * var_cnstc_foxi)), ((((-(var_cnst1soi_dn6 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7760_e7383 * var_cnstc_foxi_dn6)) / (var_cnstc_foxi * var_cnstc_foxi)), ((((-(var_cnst1soi_dn8 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7760_e7383 * var_cnstc_foxi_dn8)) / (var_cnstc_foxi * var_cnstc_foxi)), ((((-(var_cnst1soi_dn10 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7760_e7383 * var_cnstc_foxi_dn10)) / (var_cnstc_foxi * var_cnstc_foxi)), ((((-(var_cnst1soi_dn11 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7760_e7383 * var_cnstc_foxi_dn11)) / (var_cnstc_foxi * var_cnstc_foxi)), ((((-(var_cnst1soi_dn12 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7760_e7383 * var_cnstc_foxi_dn12)) / (var_cnstc_foxi * var_cnstc_foxi)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign7760_e7387;
        var_t1_dn0 = assign7760_e7387_d_n0;
        var_t1_dn2 = assign7760_e7387_d_n2;
        var_t1_dn4 = assign7760_e7387_d_n4;
        var_t1_dn5 = assign7760_e7387_d_n5;
        var_t1_dn6 = assign7760_e7387_d_n6;
        var_t1_dn8 = assign7760_e7387_d_n8;
        var_t1_dn10 = assign7760_e7387_d_n10;
        var_t1_dn11 = assign7760_e7387_d_n11;
        var_t1_dn12 = assign7760_e7387_d_n12;
        var_t1_rv = 0.0;

        let (assign7770_e7402, assign7770_e7402_d_n0, assign7770_e7402_d_n2, assign7770_e7402_d_n4, assign7770_e7402_d_n5, assign7770_e7402_d_n6, assign7770_e7402_d_n8, assign7770_e7402_d_n10, assign7770_e7402_d_n11, assign7770_e7402_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard93 != 0.0)) {
        let assign7770_e7395: f64 = (var_vgpz - var_shift);
        let assign7770_e7396: f64 = (var_t1 * assign7770_e7395);
        let assign7770_e7399: f64 = (var_vgpz - var_shift);
        let assign7770_e7400: f64 = (assign7770_e7396 * assign7770_e7399);
        (assign7770_e7400, ((((var_t1_dn0 * assign7770_e7395) + (var_t1 * (var_vgpz_dn0 - var_shift_dn0))) * assign7770_e7399) + (assign7770_e7396 * (var_vgpz_dn0 - var_shift_dn0))), ((((var_t1_dn2 * assign7770_e7395) + (var_t1 * (var_vgpz_dn2 - var_shift_dn2))) * assign7770_e7399) + (assign7770_e7396 * (var_vgpz_dn2 - var_shift_dn2))), ((((var_t1_dn4 * assign7770_e7395) + (var_t1 * (var_vgpz_dn4 - var_shift_dn4))) * assign7770_e7399) + (assign7770_e7396 * (var_vgpz_dn4 - var_shift_dn4))), ((((var_t1_dn5 * assign7770_e7395) + (var_t1 * (var_vgpz_dn5 - var_shift_dn5))) * assign7770_e7399) + (assign7770_e7396 * (var_vgpz_dn5 - var_shift_dn5))), ((((var_t1_dn6 * assign7770_e7395) + (var_t1 * (var_vgpz_dn6 - var_shift_dn6))) * assign7770_e7399) + (assign7770_e7396 * (var_vgpz_dn6 - var_shift_dn6))), ((((var_t1_dn8 * assign7770_e7395) + (var_t1 * (var_vgpz_dn8 - var_shift_dn8))) * assign7770_e7399) + (assign7770_e7396 * (var_vgpz_dn8 - var_shift_dn8))), ((((var_t1_dn10 * assign7770_e7395) + (var_t1 * (var_vgpz_dn10 - var_shift_dn10))) * assign7770_e7399) + (assign7770_e7396 * (var_vgpz_dn10 - var_shift_dn10))), ((((var_t1_dn11 * assign7770_e7395) + (var_t1 * (var_vgpz_dn11 - var_shift_dn11))) * assign7770_e7399) + (assign7770_e7396 * (var_vgpz_dn11 - var_shift_dn11))), ((((var_t1_dn12 * assign7770_e7395) + (var_t1 * (var_vgpz_dn12 - var_shift_dn12))) * assign7770_e7399) + (assign7770_e7396 * (var_vgpz_dn12 - var_shift_dn12))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign7770_e7402;
        var_t2_dn0 = assign7770_e7402_d_n0;
        var_t2_dn2 = assign7770_e7402_d_n2;
        var_t2_dn4 = assign7770_e7402_d_n4;
        var_t2_dn5 = assign7770_e7402_d_n5;
        var_t2_dn6 = assign7770_e7402_d_n6;
        var_t2_dn8 = assign7770_e7402_d_n8;
        var_t2_dn10 = assign7770_e7402_d_n10;
        var_t2_dn11 = assign7770_e7402_d_n11;
        var_t2_dn12 = assign7770_e7402_d_n12;
        var_t2_rv = 0.0;

        let (assign7780_e7415, assign7780_e7415_d_n0, assign7780_e7415_d_n2, assign7780_e7415_d_n4, assign7780_e7415_d_n5, assign7780_e7415_d_n6, assign7780_e7415_d_n8, assign7780_e7415_d_n10, assign7780_e7415_d_n11, assign7780_e7415_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard93 != 0.0)) {
        let assign7780_e7411: f64 = (var_vgpz - var_shift);
        let assign7780_e7412: f64 = (2.0 / assign7780_e7411);
        let assign7780_e7413: f64 = (var_beta + assign7780_e7412);
        (assign7780_e7413, (-((2.0 * (var_vgpz_dn0 - var_shift_dn0)) / (assign7780_e7411 * assign7780_e7411))), (-((2.0 * (var_vgpz_dn2 - var_shift_dn2)) / (assign7780_e7411 * assign7780_e7411))), (var_beta_dn4 + (-((2.0 * (var_vgpz_dn4 - var_shift_dn4)) / (assign7780_e7411 * assign7780_e7411)))), (-((2.0 * (var_vgpz_dn5 - var_shift_dn5)) / (assign7780_e7411 * assign7780_e7411))), (-((2.0 * (var_vgpz_dn6 - var_shift_dn6)) / (assign7780_e7411 * assign7780_e7411))), (-((2.0 * (var_vgpz_dn8 - var_shift_dn8)) / (assign7780_e7411 * assign7780_e7411))), (-((2.0 * (var_vgpz_dn10 - var_shift_dn10)) / (assign7780_e7411 * assign7780_e7411))), (-((2.0 * (var_vgpz_dn11 - var_shift_dn11)) / (assign7780_e7411 * assign7780_e7411))), (-((2.0 * (var_vgpz_dn12 - var_shift_dn12)) / (assign7780_e7411 * assign7780_e7411))),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign7780_e7415;
        var_t3_dn0 = assign7780_e7415_d_n0;
        var_t3_dn2 = assign7780_e7415_d_n2;
        var_t3_dn4 = assign7780_e7415_d_n4;
        var_t3_dn5 = assign7780_e7415_d_n5;
        var_t3_dn6 = assign7780_e7415_d_n6;
        var_t3_dn8 = assign7780_e7415_d_n8;
        var_t3_dn10 = assign7780_e7415_d_n10;
        var_t3_dn11 = assign7780_e7415_d_n11;
        var_t3_dn12 = assign7780_e7415_d_n12;
        var_t3_rv = 0.0;

        let (assign7790_e7425, assign7790_e7425_d_n0, assign7790_e7425_d_n2, assign7790_e7425_d_n4, assign7790_e7425_d_n5, assign7790_e7425_d_n6, assign7790_e7425_d_n8, assign7790_e7425_d_n10, assign7790_e7425_d_n11, assign7790_e7425_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard93 != 0.0)) {
        let assign7790_e7421: f64 = (var_t2).ln();
        let assign7790_e7423: f64 = (assign7790_e7421 / var_t3);
        (assign7790_e7423, ((((var_t2_dn0 / var_t2) * var_t3) - (assign7790_e7421 * var_t3_dn0)) / (var_t3 * var_t3)), ((((var_t2_dn2 / var_t2) * var_t3) - (assign7790_e7421 * var_t3_dn2)) / (var_t3 * var_t3)), ((((var_t2_dn4 / var_t2) * var_t3) - (assign7790_e7421 * var_t3_dn4)) / (var_t3 * var_t3)), ((((var_t2_dn5 / var_t2) * var_t3) - (assign7790_e7421 * var_t3_dn5)) / (var_t3 * var_t3)), ((((var_t2_dn6 / var_t2) * var_t3) - (assign7790_e7421 * var_t3_dn6)) / (var_t3 * var_t3)), ((((var_t2_dn8 / var_t2) * var_t3) - (assign7790_e7421 * var_t3_dn8)) / (var_t3 * var_t3)), ((((var_t2_dn10 / var_t2) * var_t3) - (assign7790_e7421 * var_t3_dn10)) / (var_t3 * var_t3)), ((((var_t2_dn11 / var_t2) * var_t3) - (assign7790_e7421 * var_t3_dn11)) / (var_t3 * var_t3)), ((((var_t2_dn12 / var_t2) * var_t3) - (assign7790_e7421 * var_t3_dn12)) / (var_t3 * var_t3)),)
    } else {
        (var_ps0_inib, var_ps0_inib_dn0, var_ps0_inib_dn2, var_ps0_inib_dn4, var_ps0_inib_dn5, var_ps0_inib_dn6, var_ps0_inib_dn8, var_ps0_inib_dn10, var_ps0_inib_dn11, var_ps0_inib_dn12,)
    }
};
        var_ps0_inib = assign7790_e7425;
        var_ps0_inib_dn0 = assign7790_e7425_d_n0;
        var_ps0_inib_dn2 = assign7790_e7425_d_n2;
        var_ps0_inib_dn4 = assign7790_e7425_d_n4;
        var_ps0_inib_dn5 = assign7790_e7425_d_n5;
        var_ps0_inib_dn6 = assign7790_e7425_d_n6;
        var_ps0_inib_dn8 = assign7790_e7425_d_n8;
        var_ps0_inib_dn10 = assign7790_e7425_d_n10;
        var_ps0_inib_dn11 = assign7790_e7425_d_n11;
        var_ps0_inib_dn12 = assign7790_e7425_d_n12;
        var_ps0_inib_rv = 0.0;

        let assign7800_e7429: f64 = (var_ps0_inib - 0.15);
        let assign7800_e7434: f64 = if ((var_ps0_inia > assign7800_e7429) && (0.15 >= 0.0)) { 1.0 } else { 0.0 };
        var_guard94 = assign7800_e7434;
        var_guard94_rv = 0.0;

        let (assign7810_e7447, assign7810_e7447_d_n0, assign7810_e7447_d_n2, assign7810_e7447_d_n4, assign7810_e7447_d_n5, assign7810_e7447_d_n6, assign7810_e7447_d_n8, assign7810_e7447_d_n10, assign7810_e7447_d_n11, assign7810_e7447_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        let assign7810_e7443: f64 = (var_ps0_inia - var_ps0_inib);
        let assign7810_e7445: f64 = (assign7810_e7443 + 0.15);
        (assign7810_e7445, (var_ps0_inia_dn0 - var_ps0_inib_dn0), (var_ps0_inia_dn2 - var_ps0_inib_dn2), (var_ps0_inia_dn4 - var_ps0_inib_dn4), (var_ps0_inia_dn5 - var_ps0_inib_dn5), (var_ps0_inia_dn6 - var_ps0_inib_dn6), (var_ps0_inia_dn8 - var_ps0_inib_dn8), (var_ps0_inia_dn10 - var_ps0_inib_dn10), (var_ps0_inia_dn11 - var_ps0_inib_dn11), (var_ps0_inia_dn12 - var_ps0_inib_dn12),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn8, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12,)
    }
};
        var_tmf1 = assign7810_e7447;
        var_tmf1_dn0 = assign7810_e7447_d_n0;
        var_tmf1_dn2 = assign7810_e7447_d_n2;
        var_tmf1_dn4 = assign7810_e7447_d_n4;
        var_tmf1_dn5 = assign7810_e7447_d_n5;
        var_tmf1_dn6 = assign7810_e7447_d_n6;
        var_tmf1_dn8 = assign7810_e7447_d_n8;
        var_tmf1_dn10 = assign7810_e7447_d_n10;
        var_tmf1_dn11 = assign7810_e7447_d_n11;
        var_tmf1_dn12 = assign7810_e7447_d_n12;
        var_tmf1_rv = 0.0;

        let (assign7820_e7458, assign7820_e7458_d_n0, assign7820_e7458_d_n2, assign7820_e7458_d_n4, assign7820_e7458_d_n5, assign7820_e7458_d_n6, assign7820_e7458_d_n8, assign7820_e7458_d_n10, assign7820_e7458_d_n11, assign7820_e7458_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        let assign7820_e7456: f64 = (var_tmf1 * var_tmf1);
        (assign7820_e7456, ((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)), ((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)), ((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)), ((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)), ((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)), ((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)), ((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)), ((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)), ((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn4, var_x2_dn5, var_x2_dn6, var_x2_dn8, var_x2_dn10, var_x2_dn11, var_x2_dn12,)
    }
};
        var_x2 = assign7820_e7458;
        var_x2_dn0 = assign7820_e7458_d_n0;
        var_x2_dn2 = assign7820_e7458_d_n2;
        var_x2_dn4 = assign7820_e7458_d_n4;
        var_x2_dn5 = assign7820_e7458_d_n5;
        var_x2_dn6 = assign7820_e7458_d_n6;
        var_x2_dn8 = assign7820_e7458_d_n8;
        var_x2_dn10 = assign7820_e7458_d_n10;
        var_x2_dn11 = assign7820_e7458_d_n11;
        var_x2_dn12 = assign7820_e7458_d_n12;
        var_x2_rv = 0.0;

        *var_guard92_slot = var_guard92;
        *var_guard92_rv_slot = var_guard92_rv;
        *var_guard93_slot = var_guard93;
        *var_guard93_rv_slot = var_guard93_rv;
        *var_guard94_slot = var_guard94;
        *var_guard94_rv_slot = var_guard94_rv;
        *var_phi_b0_soi_slot = var_phi_b0_soi;
        *var_phi_b0_soi_dn0_slot = var_phi_b0_soi_dn0;
        *var_phi_b0_soi_dn10_slot = var_phi_b0_soi_dn10;
        *var_phi_b0_soi_dn11_slot = var_phi_b0_soi_dn11;
        *var_phi_b0_soi_dn12_slot = var_phi_b0_soi_dn12;
        *var_phi_b0_soi_dn2_slot = var_phi_b0_soi_dn2;
        *var_phi_b0_soi_dn4_slot = var_phi_b0_soi_dn4;
        *var_phi_b0_soi_dn5_slot = var_phi_b0_soi_dn5;
        *var_phi_b0_soi_dn6_slot = var_phi_b0_soi_dn6;
        *var_phi_b0_soi_dn8_slot = var_phi_b0_soi_dn8;
        *var_phi_b0_soi_rv_slot = var_phi_b0_soi_rv;
        *var_phi_s0_bulk_slot = var_phi_s0_bulk;
        *var_phi_s0_bulk_dn0_slot = var_phi_s0_bulk_dn0;
        *var_phi_s0_bulk_dn10_slot = var_phi_s0_bulk_dn10;
        *var_phi_s0_bulk_dn11_slot = var_phi_s0_bulk_dn11;
        *var_phi_s0_bulk_dn12_slot = var_phi_s0_bulk_dn12;
        *var_phi_s0_bulk_dn2_slot = var_phi_s0_bulk_dn2;
        *var_phi_s0_bulk_dn4_slot = var_phi_s0_bulk_dn4;
        *var_phi_s0_bulk_dn5_slot = var_phi_s0_bulk_dn5;
        *var_phi_s0_bulk_dn6_slot = var_phi_s0_bulk_dn6;
        *var_phi_s0_bulk_dn8_slot = var_phi_s0_bulk_dn8;
        *var_phi_s0_bulk_rv_slot = var_phi_s0_bulk_rv;
        *var_ps0_ini_slot = var_ps0_ini;
        *var_ps0_ini_dn0_slot = var_ps0_ini_dn0;
        *var_ps0_ini_dn10_slot = var_ps0_ini_dn10;
        *var_ps0_ini_dn11_slot = var_ps0_ini_dn11;
        *var_ps0_ini_dn12_slot = var_ps0_ini_dn12;
        *var_ps0_ini_dn2_slot = var_ps0_ini_dn2;
        *var_ps0_ini_dn4_slot = var_ps0_ini_dn4;
        *var_ps0_ini_dn5_slot = var_ps0_ini_dn5;
        *var_ps0_ini_dn6_slot = var_ps0_ini_dn6;
        *var_ps0_ini_dn8_slot = var_ps0_ini_dn8;
        *var_ps0_ini_rv_slot = var_ps0_ini_rv;
        *var_ps0_inia_slot = var_ps0_inia;
        *var_ps0_inia_dn0_slot = var_ps0_inia_dn0;
        *var_ps0_inia_dn10_slot = var_ps0_inia_dn10;
        *var_ps0_inia_dn11_slot = var_ps0_inia_dn11;
        *var_ps0_inia_dn12_slot = var_ps0_inia_dn12;
        *var_ps0_inia_dn2_slot = var_ps0_inia_dn2;
        *var_ps0_inia_dn4_slot = var_ps0_inia_dn4;
        *var_ps0_inia_dn5_slot = var_ps0_inia_dn5;
        *var_ps0_inia_dn6_slot = var_ps0_inia_dn6;
        *var_ps0_inia_dn8_slot = var_ps0_inia_dn8;
        *var_ps0_inia_rv_slot = var_ps0_inia_rv;
        *var_ps0_inib_slot = var_ps0_inib;
        *var_ps0_inib_dn0_slot = var_ps0_inib_dn0;
        *var_ps0_inib_dn10_slot = var_ps0_inib_dn10;
        *var_ps0_inib_dn11_slot = var_ps0_inib_dn11;
        *var_ps0_inib_dn12_slot = var_ps0_inib_dn12;
        *var_ps0_inib_dn2_slot = var_ps0_inib_dn2;
        *var_ps0_inib_dn4_slot = var_ps0_inib_dn4;
        *var_ps0_inib_dn5_slot = var_ps0_inib_dn5;
        *var_ps0_inib_dn6_slot = var_ps0_inib_dn6;
        *var_ps0_inib_dn8_slot = var_ps0_inib_dn8;
        *var_ps0_inib_rv_slot = var_ps0_inib_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_rv_slot = var_t5_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tx_slot = var_tx;
        *var_tx_dn0_slot = var_tx_dn0;
        *var_tx_dn10_slot = var_tx_dn10;
        *var_tx_dn11_slot = var_tx_dn11;
        *var_tx_dn12_slot = var_tx_dn12;
        *var_tx_dn2_slot = var_tx_dn2;
        *var_tx_dn4_slot = var_tx_dn4;
        *var_tx_dn5_slot = var_tx_dn5;
        *var_tx_dn6_slot = var_tx_dn6;
        *var_tx_dn8_slot = var_tx_dn8;
        *var_tx_rv_slot = var_tx_rv;
        *var_x2_slot = var_x2;
        *var_x2_dn0_slot = var_x2_dn0;
        *var_x2_dn10_slot = var_x2_dn10;
        *var_x2_dn11_slot = var_x2_dn11;
        *var_x2_dn12_slot = var_x2_dn12;
        *var_x2_dn2_slot = var_x2_dn2;
        *var_x2_dn4_slot = var_x2_dn4;
        *var_x2_dn5_slot = var_x2_dn5;
        *var_x2_dn6_slot = var_x2_dn6;
        *var_x2_dn8_slot = var_x2_dn8;
        *var_x2_rv_slot = var_x2_rv;
    }

    pub(super) fn stamp_reactive_block_28(
        p: &Parameters,
        var_guard74: f64,
        var_guard93: f64,
        var_guard94: f64,
        var_ps0_inia: f64,
        var_ps0_inia_dn0: f64,
        var_ps0_inia_dn10: f64,
        var_ps0_inia_dn11: f64,
        var_ps0_inia_dn12: f64,
        var_ps0_inia_dn2: f64,
        var_ps0_inia_dn4: f64,
        var_ps0_inia_dn5: f64,
        var_ps0_inia_dn6: f64,
        var_ps0_inia_dn8: f64,
        var_ps0_inib: f64,
        var_ps0_inib_dn0: f64,
        var_ps0_inib_dn10: f64,
        var_ps0_inib_dn11: f64,
        var_ps0_inib_dn12: f64,
        var_ps0_inib_dn2: f64,
        var_ps0_inib_dn4: f64,
        var_ps0_inib_dn5: f64,
        var_ps0_inib_dn6: f64,
        var_ps0_inib_dn8: f64,
        var_tmf1: f64,
        var_tmf1_dn0: f64,
        var_tmf1_dn10: f64,
        var_tmf1_dn11: f64,
        var_tmf1_dn12: f64,
        var_tmf1_dn2: f64,
        var_tmf1_dn4: f64,
        var_tmf1_dn5: f64,
        var_tmf1_dn6: f64,
        var_tmf1_dn8: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn4: f64,
        var_uc_nsubs_dn5: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn8: f64,
        var_x2: f64,
        var_x2_dn0: f64,
        var_x2_dn10: f64,
        var_x2_dn11: f64,
        var_x2_dn12: f64,
        var_x2_dn2: f64,
        var_x2_dn4: f64,
        var_x2_dn5: f64,
        var_x2_dn6: f64,
        var_x2_dn8: f64,
        var_arg_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn10_slot: &mut f64,
        var_arg_dn11_slot: &mut f64,
        var_arg_dn12_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn11_slot: &mut f64,
        var_dnm_dn12_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn4_slot: &mut f64,
        var_dnm_dn5_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn8_slot: &mut f64,
        var_dnm_rv_slot: &mut f64,
        var_guard100_slot: &mut f64,
        var_guard100_rv_slot: &mut f64,
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
        var_m0_slot: &mut f64,
        var_m0_rv_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_mm_rv_slot: &mut f64,
        var_ps0_ini_slot: &mut f64,
        var_ps0_ini_dn0_slot: &mut f64,
        var_ps0_ini_dn10_slot: &mut f64,
        var_ps0_ini_dn11_slot: &mut f64,
        var_ps0_ini_dn12_slot: &mut f64,
        var_ps0_ini_dn2_slot: &mut f64,
        var_ps0_ini_dn4_slot: &mut f64,
        var_ps0_ini_dn5_slot: &mut f64,
        var_ps0_ini_dn6_slot: &mut f64,
        var_ps0_ini_dn8_slot: &mut f64,
        var_ps0_ini_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_tmf0_slot: &mut f64,
        var_tmf0_dn0_slot: &mut f64,
        var_tmf0_dn10_slot: &mut f64,
        var_tmf0_dn11_slot: &mut f64,
        var_tmf0_dn12_slot: &mut f64,
        var_tmf0_dn2_slot: &mut f64,
        var_tmf0_dn4_slot: &mut f64,
        var_tmf0_dn5_slot: &mut f64,
        var_tmf0_dn6_slot: &mut f64,
        var_tmf0_dn8_slot: &mut f64,
        var_tmf0_rv_slot: &mut f64,
        var_wdsoi_slot: &mut f64,
        var_wdsoi_dn0_slot: &mut f64,
        var_wdsoi_dn10_slot: &mut f64,
        var_wdsoi_dn11_slot: &mut f64,
        var_wdsoi_dn12_slot: &mut f64,
        var_wdsoi_dn2_slot: &mut f64,
        var_wdsoi_dn4_slot: &mut f64,
        var_wdsoi_dn5_slot: &mut f64,
        var_wdsoi_dn6_slot: &mut f64,
        var_wdsoi_dn8_slot: &mut f64,
        var_wdsoi_rv_slot: &mut f64,
        var_xmax2_slot: &mut f64,
        var_xmax2_dn0_slot: &mut f64,
        var_xmax2_dn10_slot: &mut f64,
        var_xmax2_dn11_slot: &mut f64,
        var_xmax2_dn12_slot: &mut f64,
        var_xmax2_dn2_slot: &mut f64,
        var_xmax2_dn4_slot: &mut f64,
        var_xmax2_dn5_slot: &mut f64,
        var_xmax2_dn6_slot: &mut f64,
        var_xmax2_dn8_slot: &mut f64,
        var_xmax2_rv_slot: &mut f64,
        var_xmp_slot: &mut f64,
        var_xmp_dn0_slot: &mut f64,
        var_xmp_dn10_slot: &mut f64,
        var_xmp_dn11_slot: &mut f64,
        var_xmp_dn12_slot: &mut f64,
        var_xmp_dn2_slot: &mut f64,
        var_xmp_dn4_slot: &mut f64,
        var_xmp_dn5_slot: &mut f64,
        var_xmp_dn6_slot: &mut f64,
        var_xmp_dn8_slot: &mut f64,
        var_xmp_rv_slot: &mut f64,
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn11_slot: &mut f64,
        var_xp_dn12_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn4_slot: &mut f64,
        var_xp_dn5_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn8_slot: &mut f64,
        var_xp_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn10: f64 = *var_arg_dn10_slot;
        let mut var_arg_dn11: f64 = *var_arg_dn11_slot;
        let mut var_arg_dn12: f64 = *var_arg_dn12_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn11: f64 = *var_dnm_dn11_slot;
        let mut var_dnm_dn12: f64 = *var_dnm_dn12_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn4: f64 = *var_dnm_dn4_slot;
        let mut var_dnm_dn5: f64 = *var_dnm_dn5_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn8: f64 = *var_dnm_dn8_slot;
        let mut var_dnm_rv: f64 = *var_dnm_rv_slot;
        let mut var_guard100: f64 = *var_guard100_slot;
        let mut var_guard100_rv: f64 = *var_guard100_rv_slot;
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
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_m0_rv: f64 = *var_m0_rv_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_mm_rv: f64 = *var_mm_rv_slot;
        let mut var_ps0_ini: f64 = *var_ps0_ini_slot;
        let mut var_ps0_ini_dn0: f64 = *var_ps0_ini_dn0_slot;
        let mut var_ps0_ini_dn10: f64 = *var_ps0_ini_dn10_slot;
        let mut var_ps0_ini_dn11: f64 = *var_ps0_ini_dn11_slot;
        let mut var_ps0_ini_dn12: f64 = *var_ps0_ini_dn12_slot;
        let mut var_ps0_ini_dn2: f64 = *var_ps0_ini_dn2_slot;
        let mut var_ps0_ini_dn4: f64 = *var_ps0_ini_dn4_slot;
        let mut var_ps0_ini_dn5: f64 = *var_ps0_ini_dn5_slot;
        let mut var_ps0_ini_dn6: f64 = *var_ps0_ini_dn6_slot;
        let mut var_ps0_ini_dn8: f64 = *var_ps0_ini_dn8_slot;
        let mut var_ps0_ini_rv: f64 = *var_ps0_ini_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_tmf0: f64 = *var_tmf0_slot;
        let mut var_tmf0_dn0: f64 = *var_tmf0_dn0_slot;
        let mut var_tmf0_dn10: f64 = *var_tmf0_dn10_slot;
        let mut var_tmf0_dn11: f64 = *var_tmf0_dn11_slot;
        let mut var_tmf0_dn12: f64 = *var_tmf0_dn12_slot;
        let mut var_tmf0_dn2: f64 = *var_tmf0_dn2_slot;
        let mut var_tmf0_dn4: f64 = *var_tmf0_dn4_slot;
        let mut var_tmf0_dn5: f64 = *var_tmf0_dn5_slot;
        let mut var_tmf0_dn6: f64 = *var_tmf0_dn6_slot;
        let mut var_tmf0_dn8: f64 = *var_tmf0_dn8_slot;
        let mut var_tmf0_rv: f64 = *var_tmf0_rv_slot;
        let mut var_wdsoi: f64 = *var_wdsoi_slot;
        let mut var_wdsoi_dn0: f64 = *var_wdsoi_dn0_slot;
        let mut var_wdsoi_dn10: f64 = *var_wdsoi_dn10_slot;
        let mut var_wdsoi_dn11: f64 = *var_wdsoi_dn11_slot;
        let mut var_wdsoi_dn12: f64 = *var_wdsoi_dn12_slot;
        let mut var_wdsoi_dn2: f64 = *var_wdsoi_dn2_slot;
        let mut var_wdsoi_dn4: f64 = *var_wdsoi_dn4_slot;
        let mut var_wdsoi_dn5: f64 = *var_wdsoi_dn5_slot;
        let mut var_wdsoi_dn6: f64 = *var_wdsoi_dn6_slot;
        let mut var_wdsoi_dn8: f64 = *var_wdsoi_dn8_slot;
        let mut var_wdsoi_rv: f64 = *var_wdsoi_rv_slot;
        let mut var_xmax2: f64 = *var_xmax2_slot;
        let mut var_xmax2_dn0: f64 = *var_xmax2_dn0_slot;
        let mut var_xmax2_dn10: f64 = *var_xmax2_dn10_slot;
        let mut var_xmax2_dn11: f64 = *var_xmax2_dn11_slot;
        let mut var_xmax2_dn12: f64 = *var_xmax2_dn12_slot;
        let mut var_xmax2_dn2: f64 = *var_xmax2_dn2_slot;
        let mut var_xmax2_dn4: f64 = *var_xmax2_dn4_slot;
        let mut var_xmax2_dn5: f64 = *var_xmax2_dn5_slot;
        let mut var_xmax2_dn6: f64 = *var_xmax2_dn6_slot;
        let mut var_xmax2_dn8: f64 = *var_xmax2_dn8_slot;
        let mut var_xmax2_rv: f64 = *var_xmax2_rv_slot;
        let mut var_xmp: f64 = *var_xmp_slot;
        let mut var_xmp_dn0: f64 = *var_xmp_dn0_slot;
        let mut var_xmp_dn10: f64 = *var_xmp_dn10_slot;
        let mut var_xmp_dn11: f64 = *var_xmp_dn11_slot;
        let mut var_xmp_dn12: f64 = *var_xmp_dn12_slot;
        let mut var_xmp_dn2: f64 = *var_xmp_dn2_slot;
        let mut var_xmp_dn4: f64 = *var_xmp_dn4_slot;
        let mut var_xmp_dn5: f64 = *var_xmp_dn5_slot;
        let mut var_xmp_dn6: f64 = *var_xmp_dn6_slot;
        let mut var_xmp_dn8: f64 = *var_xmp_dn8_slot;
        let mut var_xmp_rv: f64 = *var_xmp_rv_slot;
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn11: f64 = *var_xp_dn11_slot;
        let mut var_xp_dn12: f64 = *var_xp_dn12_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn4: f64 = *var_xp_dn4_slot;
        let mut var_xp_dn5: f64 = *var_xp_dn5_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn8: f64 = *var_xp_dn8_slot;
        let mut var_xp_rv: f64 = *var_xp_rv_slot;

        let (assign7830_e7469, assign7830_e7469_d_n0, assign7830_e7469_d_n2, assign7830_e7469_d_n4, assign7830_e7469_d_n5, assign7830_e7469_d_n6, assign7830_e7469_d_n8, assign7830_e7469_d_n10, assign7830_e7469_d_n11, assign7830_e7469_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        let assign7830_e7467: f64 = (0.15 * 0.15);
        (assign7830_e7467, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn4, var_xmax2_dn5, var_xmax2_dn6, var_xmax2_dn8, var_xmax2_dn10, var_xmax2_dn11, var_xmax2_dn12,)
    }
};
        var_xmax2 = assign7830_e7469;
        var_xmax2_dn0 = assign7830_e7469_d_n0;
        var_xmax2_dn2 = assign7830_e7469_d_n2;
        var_xmax2_dn4 = assign7830_e7469_d_n4;
        var_xmax2_dn5 = assign7830_e7469_d_n5;
        var_xmax2_dn6 = assign7830_e7469_d_n6;
        var_xmax2_dn8 = assign7830_e7469_d_n8;
        var_xmax2_dn10 = assign7830_e7469_d_n10;
        var_xmax2_dn11 = assign7830_e7469_d_n11;
        var_xmax2_dn12 = assign7830_e7469_d_n12;
        var_xmax2_rv = 0.0;

        let (assign7840_e7478, assign7840_e7478_d_n0, assign7840_e7478_d_n2, assign7840_e7478_d_n4, assign7840_e7478_d_n5, assign7840_e7478_d_n6, assign7840_e7478_d_n8, assign7840_e7478_d_n10, assign7840_e7478_d_n11, assign7840_e7478_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn8, var_xp_dn10, var_xp_dn11, var_xp_dn12,)
    }
};
        var_xp = assign7840_e7478;
        var_xp_dn0 = assign7840_e7478_d_n0;
        var_xp_dn2 = assign7840_e7478_d_n2;
        var_xp_dn4 = assign7840_e7478_d_n4;
        var_xp_dn5 = assign7840_e7478_d_n5;
        var_xp_dn6 = assign7840_e7478_d_n6;
        var_xp_dn8 = assign7840_e7478_d_n8;
        var_xp_dn10 = assign7840_e7478_d_n10;
        var_xp_dn11 = assign7840_e7478_d_n11;
        var_xp_dn12 = assign7840_e7478_d_n12;
        var_xp_rv = 0.0;

        let (assign7850_e7487, assign7850_e7487_d_n0, assign7850_e7487_d_n2, assign7850_e7487_d_n4, assign7850_e7487_d_n5, assign7850_e7487_d_n6, assign7850_e7487_d_n8, assign7850_e7487_d_n10, assign7850_e7487_d_n11, assign7850_e7487_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn8, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12,)
    }
};
        var_xmp = assign7850_e7487;
        var_xmp_dn0 = assign7850_e7487_d_n0;
        var_xmp_dn2 = assign7850_e7487_d_n2;
        var_xmp_dn4 = assign7850_e7487_d_n4;
        var_xmp_dn5 = assign7850_e7487_d_n5;
        var_xmp_dn6 = assign7850_e7487_d_n6;
        var_xmp_dn8 = assign7850_e7487_d_n8;
        var_xmp_dn10 = assign7850_e7487_d_n10;
        var_xmp_dn11 = assign7850_e7487_d_n11;
        var_xmp_dn12 = assign7850_e7487_d_n12;
        var_xmp_rv = 0.0;

        let (assign7860_e7496,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign7860_e7496;
        var_m0_rv = 0.0;

        let (assign7870_e7505,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign7870_e7505;
        var_mm_rv = 0.0;

        let (assign7880_e7514, assign7880_e7514_d_n0, assign7880_e7514_d_n2, assign7880_e7514_d_n4, assign7880_e7514_d_n5, assign7880_e7514_d_n6, assign7880_e7514_d_n8, assign7880_e7514_d_n10, assign7880_e7514_d_n11, assign7880_e7514_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn8, var_arg_dn10, var_arg_dn11, var_arg_dn12,)
    }
};
        var_arg = assign7880_e7514;
        var_arg_dn0 = assign7880_e7514_d_n0;
        var_arg_dn2 = assign7880_e7514_d_n2;
        var_arg_dn4 = assign7880_e7514_d_n4;
        var_arg_dn5 = assign7880_e7514_d_n5;
        var_arg_dn6 = assign7880_e7514_d_n6;
        var_arg_dn8 = assign7880_e7514_d_n8;
        var_arg_dn10 = assign7880_e7514_d_n10;
        var_arg_dn11 = assign7880_e7514_d_n11;
        var_arg_dn12 = assign7880_e7514_d_n12;
        var_arg_rv = 0.0;

        let (assign7890_e7523, assign7890_e7523_d_n0, assign7890_e7523_d_n2, assign7890_e7523_d_n4, assign7890_e7523_d_n5, assign7890_e7523_d_n6, assign7890_e7523_d_n8, assign7890_e7523_d_n10, assign7890_e7523_d_n11, assign7890_e7523_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn8, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12,)
    }
};
        var_dnm = assign7890_e7523;
        var_dnm_dn0 = assign7890_e7523_d_n0;
        var_dnm_dn2 = assign7890_e7523_d_n2;
        var_dnm_dn4 = assign7890_e7523_d_n4;
        var_dnm_dn5 = assign7890_e7523_d_n5;
        var_dnm_dn6 = assign7890_e7523_d_n6;
        var_dnm_dn8 = assign7890_e7523_d_n8;
        var_dnm_dn10 = assign7890_e7523_d_n10;
        var_dnm_dn11 = assign7890_e7523_d_n11;
        var_dnm_dn12 = assign7890_e7523_d_n12;
        var_dnm_rv = 0.0;

        let (assign7900_e7534, assign7900_e7534_d_n0, assign7900_e7534_d_n2, assign7900_e7534_d_n4, assign7900_e7534_d_n5, assign7900_e7534_d_n6, assign7900_e7534_d_n8, assign7900_e7534_d_n10, assign7900_e7534_d_n11, assign7900_e7534_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        let assign7900_e7532: f64 = (var_xp * var_x2);
        (assign7900_e7532, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn8, var_xp_dn10, var_xp_dn11, var_xp_dn12,)
    }
};
        var_xp = assign7900_e7534;
        var_xp_dn0 = assign7900_e7534_d_n0;
        var_xp_dn2 = assign7900_e7534_d_n2;
        var_xp_dn4 = assign7900_e7534_d_n4;
        var_xp_dn5 = assign7900_e7534_d_n5;
        var_xp_dn6 = assign7900_e7534_d_n6;
        var_xp_dn8 = assign7900_e7534_d_n8;
        var_xp_dn10 = assign7900_e7534_d_n10;
        var_xp_dn11 = assign7900_e7534_d_n11;
        var_xp_dn12 = assign7900_e7534_d_n12;
        var_xp_rv = 0.0;

        let (assign7910_e7545, assign7910_e7545_d_n0, assign7910_e7545_d_n2, assign7910_e7545_d_n4, assign7910_e7545_d_n5, assign7910_e7545_d_n6, assign7910_e7545_d_n8, assign7910_e7545_d_n10, assign7910_e7545_d_n11, assign7910_e7545_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        let assign7910_e7543: f64 = (var_xmp * var_xmax2);
        (assign7910_e7543, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn8, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12,)
    }
};
        var_xmp = assign7910_e7545;
        var_xmp_dn0 = assign7910_e7545_d_n0;
        var_xmp_dn2 = assign7910_e7545_d_n2;
        var_xmp_dn4 = assign7910_e7545_d_n4;
        var_xmp_dn5 = assign7910_e7545_d_n5;
        var_xmp_dn6 = assign7910_e7545_d_n6;
        var_xmp_dn8 = assign7910_e7545_d_n8;
        var_xmp_dn10 = assign7910_e7545_d_n10;
        var_xmp_dn11 = assign7910_e7545_d_n11;
        var_xmp_dn12 = assign7910_e7545_d_n12;
        var_xmp_rv = 0.0;

        let (assign7920_e7556, assign7920_e7556_d_n0, assign7920_e7556_d_n2, assign7920_e7556_d_n4, assign7920_e7556_d_n5, assign7920_e7556_d_n6, assign7920_e7556_d_n8, assign7920_e7556_d_n10, assign7920_e7556_d_n11, assign7920_e7556_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        let assign7920_e7554: f64 = (var_xp + var_xmp);
        (assign7920_e7554, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn4 + var_xmp_dn4), (var_xp_dn5 + var_xmp_dn5), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn8 + var_xmp_dn8), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn11 + var_xmp_dn11), (var_xp_dn12 + var_xmp_dn12),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn8, var_arg_dn10, var_arg_dn11, var_arg_dn12,)
    }
};
        var_arg = assign7920_e7556;
        var_arg_dn0 = assign7920_e7556_d_n0;
        var_arg_dn2 = assign7920_e7556_d_n2;
        var_arg_dn4 = assign7920_e7556_d_n4;
        var_arg_dn5 = assign7920_e7556_d_n5;
        var_arg_dn6 = assign7920_e7556_d_n6;
        var_arg_dn8 = assign7920_e7556_d_n8;
        var_arg_dn10 = assign7920_e7556_d_n10;
        var_arg_dn11 = assign7920_e7556_d_n11;
        var_arg_dn12 = assign7920_e7556_d_n12;
        var_arg_rv = 0.0;

        let (assign7930_e7565, assign7930_e7565_d_n0, assign7930_e7565_d_n2, assign7930_e7565_d_n4, assign7930_e7565_d_n5, assign7930_e7565_d_n6, assign7930_e7565_d_n8, assign7930_e7565_d_n10, assign7930_e7565_d_n11, assign7930_e7565_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn8, var_arg_dn10, var_arg_dn11, var_arg_dn12,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn8, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12,)
    }
};
        var_dnm = assign7930_e7565;
        var_dnm_dn0 = assign7930_e7565_d_n0;
        var_dnm_dn2 = assign7930_e7565_d_n2;
        var_dnm_dn4 = assign7930_e7565_d_n4;
        var_dnm_dn5 = assign7930_e7565_d_n5;
        var_dnm_dn6 = assign7930_e7565_d_n6;
        var_dnm_dn8 = assign7930_e7565_d_n8;
        var_dnm_dn10 = assign7930_e7565_d_n10;
        var_dnm_dn11 = assign7930_e7565_d_n11;
        var_dnm_dn12 = assign7930_e7565_d_n12;
        var_dnm_rv = 0.0;

        let assign7940_e7580: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard95 = assign7940_e7580;
        var_guard95_rv = 0.0;

        let assign7950_e7583: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard96 = assign7950_e7583;
        var_guard96_rv = 0.0;

        let (assign7960_e7596,) = {
    if (((((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) && (var_guard95 != 0.0)) && (var_guard96 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign7960_e7596;
        var_mm_rv = 0.0;

        let assign7970_e7599: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard97 = assign7970_e7599;
        var_guard97_rv = 0.0;

        let (assign7980_e7615,) = {
    if ((((((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) && (var_guard95 != 0.0)) && (var_guard96 == 0.0)) && (var_guard97 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign7980_e7615;
        var_mm_rv = 0.0;

        let assign7990_e7618: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard98 = assign7990_e7618;
        var_guard98_rv = 0.0;

        let (assign8000_e7637,) = {
    if (((((((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) && (var_guard95 != 0.0)) && (var_guard96 == 0.0)) && (var_guard97 == 0.0)) && (var_guard98 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign8000_e7637;
        var_mm_rv = 0.0;

        let assign8010_e7640: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard99 = assign8010_e7640;
        var_guard99_rv = 0.0;

        let (assign8020_e7662,) = {
    if ((((((((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) && (var_guard95 != 0.0)) && (var_guard96 == 0.0)) && (var_guard97 == 0.0)) && (var_guard98 == 0.0)) && (var_guard99 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign8020_e7662;
        var_mm_rv = 0.0;

        let (assign8030_e7673,) = {
    if ((((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) && (var_guard95 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign8030_e7673;
        var_m0_rv = 0.0;

        let mut assign8040_loop_guard: usize = 0;
        while {
            let assign8040_cond_e7685: f64 = if (((((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) && (var_guard95 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign8040_cond_e7685 != 0.0
        } {
            assign8040_loop_guard += 1;
            assert!(assign8040_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign8040_body0_e7697, assign8040_body0_e7697_d_n0, assign8040_body0_e7697_d_n2, assign8040_body0_e7697_d_n4, assign8040_body0_e7697_d_n5, assign8040_body0_e7697_d_n6, assign8040_body0_e7697_d_n8, assign8040_body0_e7697_d_n10, assign8040_body0_e7697_d_n11, assign8040_body0_e7697_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) && (var_guard95 != 0.0)) {
        let assign8040_body0_e7695: f64 = (var_dnm).sqrt();
        (assign8040_body0_e7695, (var_dnm_dn0 / (2.0 * assign8040_body0_e7695)), (var_dnm_dn2 / (2.0 * assign8040_body0_e7695)), (var_dnm_dn4 / (2.0 * assign8040_body0_e7695)), (var_dnm_dn5 / (2.0 * assign8040_body0_e7695)), (var_dnm_dn6 / (2.0 * assign8040_body0_e7695)), (var_dnm_dn8 / (2.0 * assign8040_body0_e7695)), (var_dnm_dn10 / (2.0 * assign8040_body0_e7695)), (var_dnm_dn11 / (2.0 * assign8040_body0_e7695)), (var_dnm_dn12 / (2.0 * assign8040_body0_e7695)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn8, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12,)
    }
};
            var_dnm = assign8040_body0_e7697;
            var_dnm_dn0 = assign8040_body0_e7697_d_n0;
            var_dnm_dn2 = assign8040_body0_e7697_d_n2;
            var_dnm_dn4 = assign8040_body0_e7697_d_n4;
            var_dnm_dn5 = assign8040_body0_e7697_d_n5;
            var_dnm_dn6 = assign8040_body0_e7697_d_n6;
            var_dnm_dn8 = assign8040_body0_e7697_d_n8;
            var_dnm_dn10 = assign8040_body0_e7697_d_n10;
            var_dnm_dn11 = assign8040_body0_e7697_d_n11;
            var_dnm_dn12 = assign8040_body0_e7697_d_n12;
            var_dnm_rv = 0.0;
            let (assign8040_body1_e7710,) = {
    if ((((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) && (var_guard95 != 0.0)) {
        let assign8040_body1_e7708: f64 = (var_m0 + 1.0);
        (assign8040_body1_e7708,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign8040_body1_e7710;
            var_m0_rv = 0.0;
        }

        let (assign8050_e7728, assign8050_e7728_d_n0, assign8050_e7728_d_n2, assign8050_e7728_d_n4, assign8050_e7728_d_n5, assign8050_e7728_d_n6, assign8050_e7728_d_n8, assign8050_e7728_d_n10, assign8050_e7728_d_n11, assign8050_e7728_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) && (var_guard95 == 0.0)) {
        let assign8050_e7724: f64 = 2.0;
        let assign8050_e7725: f64 = (1.0 / assign8050_e7724);
        let assign8050_e7726: f64 = (var_dnm).powf(assign8050_e7725);
        (assign8050_e7726, if 0.0 == 0.0 && ((assign8050_e7725) as f64).is_finite() && ((assign8050_e7725) as f64).fract() == 0.0 { if assign8050_e7725 == 0.0 { 0.0 } else { (assign8050_e7725 * ((var_dnm).powf(assign8050_e7725 - 1.0) * var_dnm_dn0)) } } else { (assign8050_e7726 * (assign8050_e7725 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign8050_e7725) as f64).is_finite() && ((assign8050_e7725) as f64).fract() == 0.0 { if assign8050_e7725 == 0.0 { 0.0 } else { (assign8050_e7725 * ((var_dnm).powf(assign8050_e7725 - 1.0) * var_dnm_dn2)) } } else { (assign8050_e7726 * (assign8050_e7725 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign8050_e7725) as f64).is_finite() && ((assign8050_e7725) as f64).fract() == 0.0 { if assign8050_e7725 == 0.0 { 0.0 } else { (assign8050_e7725 * ((var_dnm).powf(assign8050_e7725 - 1.0) * var_dnm_dn4)) } } else { (assign8050_e7726 * (assign8050_e7725 * (var_dnm_dn4 / var_dnm))) }, if 0.0 == 0.0 && ((assign8050_e7725) as f64).is_finite() && ((assign8050_e7725) as f64).fract() == 0.0 { if assign8050_e7725 == 0.0 { 0.0 } else { (assign8050_e7725 * ((var_dnm).powf(assign8050_e7725 - 1.0) * var_dnm_dn5)) } } else { (assign8050_e7726 * (assign8050_e7725 * (var_dnm_dn5 / var_dnm))) }, if 0.0 == 0.0 && ((assign8050_e7725) as f64).is_finite() && ((assign8050_e7725) as f64).fract() == 0.0 { if assign8050_e7725 == 0.0 { 0.0 } else { (assign8050_e7725 * ((var_dnm).powf(assign8050_e7725 - 1.0) * var_dnm_dn6)) } } else { (assign8050_e7726 * (assign8050_e7725 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign8050_e7725) as f64).is_finite() && ((assign8050_e7725) as f64).fract() == 0.0 { if assign8050_e7725 == 0.0 { 0.0 } else { (assign8050_e7725 * ((var_dnm).powf(assign8050_e7725 - 1.0) * var_dnm_dn8)) } } else { (assign8050_e7726 * (assign8050_e7725 * (var_dnm_dn8 / var_dnm))) }, if 0.0 == 0.0 && ((assign8050_e7725) as f64).is_finite() && ((assign8050_e7725) as f64).fract() == 0.0 { if assign8050_e7725 == 0.0 { 0.0 } else { (assign8050_e7725 * ((var_dnm).powf(assign8050_e7725 - 1.0) * var_dnm_dn10)) } } else { (assign8050_e7726 * (assign8050_e7725 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign8050_e7725) as f64).is_finite() && ((assign8050_e7725) as f64).fract() == 0.0 { if assign8050_e7725 == 0.0 { 0.0 } else { (assign8050_e7725 * ((var_dnm).powf(assign8050_e7725 - 1.0) * var_dnm_dn11)) } } else { (assign8050_e7726 * (assign8050_e7725 * (var_dnm_dn11 / var_dnm))) }, if 0.0 == 0.0 && ((assign8050_e7725) as f64).is_finite() && ((assign8050_e7725) as f64).fract() == 0.0 { if assign8050_e7725 == 0.0 { 0.0 } else { (assign8050_e7725 * ((var_dnm).powf(assign8050_e7725 - 1.0) * var_dnm_dn12)) } } else { (assign8050_e7726 * (assign8050_e7725 * (var_dnm_dn12 / var_dnm))) },)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn8, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12,)
    }
};
        var_dnm = assign8050_e7728;
        var_dnm_dn0 = assign8050_e7728_d_n0;
        var_dnm_dn2 = assign8050_e7728_d_n2;
        var_dnm_dn4 = assign8050_e7728_d_n4;
        var_dnm_dn5 = assign8050_e7728_d_n5;
        var_dnm_dn6 = assign8050_e7728_d_n6;
        var_dnm_dn8 = assign8050_e7728_d_n8;
        var_dnm_dn10 = assign8050_e7728_d_n10;
        var_dnm_dn11 = assign8050_e7728_d_n11;
        var_dnm_dn12 = assign8050_e7728_d_n12;
        var_dnm_rv = 0.0;

        let (assign8060_e7741, assign8060_e7741_d_n0, assign8060_e7741_d_n2, assign8060_e7741_d_n4, assign8060_e7741_d_n5, assign8060_e7741_d_n6, assign8060_e7741_d_n8, assign8060_e7741_d_n10, assign8060_e7741_d_n11, assign8060_e7741_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        let assign8060_e7738: f64 = (var_dnm + 1e-50);
        let assign8060_e7739: f64 = (1.0 / assign8060_e7738);
        (assign8060_e7739, (-(var_dnm_dn0 / (assign8060_e7738 * assign8060_e7738))), (-(var_dnm_dn2 / (assign8060_e7738 * assign8060_e7738))), (-(var_dnm_dn4 / (assign8060_e7738 * assign8060_e7738))), (-(var_dnm_dn5 / (assign8060_e7738 * assign8060_e7738))), (-(var_dnm_dn6 / (assign8060_e7738 * assign8060_e7738))), (-(var_dnm_dn8 / (assign8060_e7738 * assign8060_e7738))), (-(var_dnm_dn10 / (assign8060_e7738 * assign8060_e7738))), (-(var_dnm_dn11 / (assign8060_e7738 * assign8060_e7738))), (-(var_dnm_dn12 / (assign8060_e7738 * assign8060_e7738))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn8, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12,)
    }
};
        var_dnm = assign8060_e7741;
        var_dnm_dn0 = assign8060_e7741_d_n0;
        var_dnm_dn2 = assign8060_e7741_d_n2;
        var_dnm_dn4 = assign8060_e7741_d_n4;
        var_dnm_dn5 = assign8060_e7741_d_n5;
        var_dnm_dn6 = assign8060_e7741_d_n6;
        var_dnm_dn8 = assign8060_e7741_d_n8;
        var_dnm_dn10 = assign8060_e7741_d_n10;
        var_dnm_dn11 = assign8060_e7741_d_n11;
        var_dnm_dn12 = assign8060_e7741_d_n12;
        var_dnm_rv = 0.0;

        let (assign8070_e7754, assign8070_e7754_d_n0, assign8070_e7754_d_n2, assign8070_e7754_d_n4, assign8070_e7754_d_n5, assign8070_e7754_d_n6, assign8070_e7754_d_n8, assign8070_e7754_d_n10, assign8070_e7754_d_n11, assign8070_e7754_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        let assign8070_e7750: f64 = (var_tmf1 * 0.15);
        let assign8070_e7752: f64 = (assign8070_e7750 * var_dnm);
        (assign8070_e7752, (((var_tmf1_dn0 * 0.15) * var_dnm) + (assign8070_e7750 * var_dnm_dn0)), (((var_tmf1_dn2 * 0.15) * var_dnm) + (assign8070_e7750 * var_dnm_dn2)), (((var_tmf1_dn4 * 0.15) * var_dnm) + (assign8070_e7750 * var_dnm_dn4)), (((var_tmf1_dn5 * 0.15) * var_dnm) + (assign8070_e7750 * var_dnm_dn5)), (((var_tmf1_dn6 * 0.15) * var_dnm) + (assign8070_e7750 * var_dnm_dn6)), (((var_tmf1_dn8 * 0.15) * var_dnm) + (assign8070_e7750 * var_dnm_dn8)), (((var_tmf1_dn10 * 0.15) * var_dnm) + (assign8070_e7750 * var_dnm_dn10)), (((var_tmf1_dn11 * 0.15) * var_dnm) + (assign8070_e7750 * var_dnm_dn11)), (((var_tmf1_dn12 * 0.15) * var_dnm) + (assign8070_e7750 * var_dnm_dn12)),)
    } else {
        (var_tmf0, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn4, var_tmf0_dn5, var_tmf0_dn6, var_tmf0_dn8, var_tmf0_dn10, var_tmf0_dn11, var_tmf0_dn12,)
    }
};
        var_tmf0 = assign8070_e7754;
        var_tmf0_dn0 = assign8070_e7754_d_n0;
        var_tmf0_dn2 = assign8070_e7754_d_n2;
        var_tmf0_dn4 = assign8070_e7754_d_n4;
        var_tmf0_dn5 = assign8070_e7754_d_n5;
        var_tmf0_dn6 = assign8070_e7754_d_n6;
        var_tmf0_dn8 = assign8070_e7754_d_n8;
        var_tmf0_dn10 = assign8070_e7754_d_n10;
        var_tmf0_dn11 = assign8070_e7754_d_n11;
        var_tmf0_dn12 = assign8070_e7754_d_n12;
        var_tmf0_rv = 0.0;

        let (assign8080_e7771, assign8080_e7771_d_n0, assign8080_e7771_d_n2, assign8080_e7771_d_n4, assign8080_e7771_d_n5, assign8080_e7771_d_n6, assign8080_e7771_d_n8, assign8080_e7771_d_n10, assign8080_e7771_d_n11, assign8080_e7771_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        let assign8080_e7763: f64 = (0.15 * var_xmp);
        let assign8080_e7765: f64 = (assign8080_e7763 * var_dnm);
        let assign8080_e7768: f64 = (var_arg + 1e-50);
        let assign8080_e7769: f64 = (assign8080_e7765 / assign8080_e7768);
        (assign8080_e7769, ((((((0.15 * var_xmp_dn0) * var_dnm) + (assign8080_e7763 * var_dnm_dn0)) * assign8080_e7768) - (assign8080_e7765 * var_arg_dn0)) / (assign8080_e7768 * assign8080_e7768)), ((((((0.15 * var_xmp_dn2) * var_dnm) + (assign8080_e7763 * var_dnm_dn2)) * assign8080_e7768) - (assign8080_e7765 * var_arg_dn2)) / (assign8080_e7768 * assign8080_e7768)), ((((((0.15 * var_xmp_dn4) * var_dnm) + (assign8080_e7763 * var_dnm_dn4)) * assign8080_e7768) - (assign8080_e7765 * var_arg_dn4)) / (assign8080_e7768 * assign8080_e7768)), ((((((0.15 * var_xmp_dn5) * var_dnm) + (assign8080_e7763 * var_dnm_dn5)) * assign8080_e7768) - (assign8080_e7765 * var_arg_dn5)) / (assign8080_e7768 * assign8080_e7768)), ((((((0.15 * var_xmp_dn6) * var_dnm) + (assign8080_e7763 * var_dnm_dn6)) * assign8080_e7768) - (assign8080_e7765 * var_arg_dn6)) / (assign8080_e7768 * assign8080_e7768)), ((((((0.15 * var_xmp_dn8) * var_dnm) + (assign8080_e7763 * var_dnm_dn8)) * assign8080_e7768) - (assign8080_e7765 * var_arg_dn8)) / (assign8080_e7768 * assign8080_e7768)), ((((((0.15 * var_xmp_dn10) * var_dnm) + (assign8080_e7763 * var_dnm_dn10)) * assign8080_e7768) - (assign8080_e7765 * var_arg_dn10)) / (assign8080_e7768 * assign8080_e7768)), ((((((0.15 * var_xmp_dn11) * var_dnm) + (assign8080_e7763 * var_dnm_dn11)) * assign8080_e7768) - (assign8080_e7765 * var_arg_dn11)) / (assign8080_e7768 * assign8080_e7768)), ((((((0.15 * var_xmp_dn12) * var_dnm) + (assign8080_e7763 * var_dnm_dn12)) * assign8080_e7768) - (assign8080_e7765 * var_arg_dn12)) / (assign8080_e7768 * assign8080_e7768)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign8080_e7771;
        var_t1_dn0 = assign8080_e7771_d_n0;
        var_t1_dn2 = assign8080_e7771_d_n2;
        var_t1_dn4 = assign8080_e7771_d_n4;
        var_t1_dn5 = assign8080_e7771_d_n5;
        var_t1_dn6 = assign8080_e7771_d_n6;
        var_t1_dn8 = assign8080_e7771_d_n8;
        var_t1_dn10 = assign8080_e7771_d_n10;
        var_t1_dn11 = assign8080_e7771_d_n11;
        var_t1_dn12 = assign8080_e7771_d_n12;
        var_t1_rv = 0.0;

        let (assign8090_e7784, assign8090_e7784_d_n0, assign8090_e7784_d_n2, assign8090_e7784_d_n4, assign8090_e7784_d_n5, assign8090_e7784_d_n6, assign8090_e7784_d_n8, assign8090_e7784_d_n10, assign8090_e7784_d_n11, assign8090_e7784_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        let assign8090_e7780: f64 = (var_ps0_inib - 0.15);
        let assign8090_e7782: f64 = (assign8090_e7780 + var_tmf0);
        (assign8090_e7782, (var_ps0_inib_dn0 + var_tmf0_dn0), (var_ps0_inib_dn2 + var_tmf0_dn2), (var_ps0_inib_dn4 + var_tmf0_dn4), (var_ps0_inib_dn5 + var_tmf0_dn5), (var_ps0_inib_dn6 + var_tmf0_dn6), (var_ps0_inib_dn8 + var_tmf0_dn8), (var_ps0_inib_dn10 + var_tmf0_dn10), (var_ps0_inib_dn11 + var_tmf0_dn11), (var_ps0_inib_dn12 + var_tmf0_dn12),)
    } else {
        (var_ps0_ini, var_ps0_ini_dn0, var_ps0_ini_dn2, var_ps0_ini_dn4, var_ps0_ini_dn5, var_ps0_ini_dn6, var_ps0_ini_dn8, var_ps0_ini_dn10, var_ps0_ini_dn11, var_ps0_ini_dn12,)
    }
};
        var_ps0_ini = assign8090_e7784;
        var_ps0_ini_dn0 = assign8090_e7784_d_n0;
        var_ps0_ini_dn2 = assign8090_e7784_d_n2;
        var_ps0_ini_dn4 = assign8090_e7784_d_n4;
        var_ps0_ini_dn5 = assign8090_e7784_d_n5;
        var_ps0_ini_dn6 = assign8090_e7784_d_n6;
        var_ps0_ini_dn8 = assign8090_e7784_d_n8;
        var_ps0_ini_dn10 = assign8090_e7784_d_n10;
        var_ps0_ini_dn11 = assign8090_e7784_d_n11;
        var_ps0_ini_dn12 = assign8090_e7784_d_n12;
        var_ps0_ini_rv = 0.0;

        let (assign8100_e7793, assign8100_e7793_d_n0, assign8100_e7793_d_n2, assign8100_e7793_d_n4, assign8100_e7793_d_n5, assign8100_e7793_d_n6, assign8100_e7793_d_n8, assign8100_e7793_d_n10, assign8100_e7793_d_n11, assign8100_e7793_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign8100_e7793;
        var_t1_dn0 = assign8100_e7793_d_n0;
        var_t1_dn2 = assign8100_e7793_d_n2;
        var_t1_dn4 = assign8100_e7793_d_n4;
        var_t1_dn5 = assign8100_e7793_d_n5;
        var_t1_dn6 = assign8100_e7793_d_n6;
        var_t1_dn8 = assign8100_e7793_d_n8;
        var_t1_dn10 = assign8100_e7793_d_n10;
        var_t1_dn11 = assign8100_e7793_d_n11;
        var_t1_dn12 = assign8100_e7793_d_n12;
        var_t1_rv = 0.0;

        let (assign8110_e7803, assign8110_e7803_d_n0, assign8110_e7803_d_n2, assign8110_e7803_d_n4, assign8110_e7803_d_n5, assign8110_e7803_d_n6, assign8110_e7803_d_n8, assign8110_e7803_d_n10, assign8110_e7803_d_n11, assign8110_e7803_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 == 0.0)) {
        (var_ps0_inia, var_ps0_inia_dn0, var_ps0_inia_dn2, var_ps0_inia_dn4, var_ps0_inia_dn5, var_ps0_inia_dn6, var_ps0_inia_dn8, var_ps0_inia_dn10, var_ps0_inia_dn11, var_ps0_inia_dn12,)
    } else {
        (var_ps0_ini, var_ps0_ini_dn0, var_ps0_ini_dn2, var_ps0_ini_dn4, var_ps0_ini_dn5, var_ps0_ini_dn6, var_ps0_ini_dn8, var_ps0_ini_dn10, var_ps0_ini_dn11, var_ps0_ini_dn12,)
    }
};
        var_ps0_ini = assign8110_e7803;
        var_ps0_ini_dn0 = assign8110_e7803_d_n0;
        var_ps0_ini_dn2 = assign8110_e7803_d_n2;
        var_ps0_ini_dn4 = assign8110_e7803_d_n4;
        var_ps0_ini_dn5 = assign8110_e7803_d_n5;
        var_ps0_ini_dn6 = assign8110_e7803_d_n6;
        var_ps0_ini_dn8 = assign8110_e7803_d_n8;
        var_ps0_ini_dn10 = assign8110_e7803_d_n10;
        var_ps0_ini_dn11 = assign8110_e7803_d_n11;
        var_ps0_ini_dn12 = assign8110_e7803_d_n12;
        var_ps0_ini_rv = 0.0;

        let (assign8120_e7813, assign8120_e7813_d_n0, assign8120_e7813_d_n2, assign8120_e7813_d_n4, assign8120_e7813_d_n5, assign8120_e7813_d_n6, assign8120_e7813_d_n8, assign8120_e7813_d_n10, assign8120_e7813_d_n11, assign8120_e7813_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign8120_e7813;
        var_t1_dn0 = assign8120_e7813_d_n0;
        var_t1_dn2 = assign8120_e7813_d_n2;
        var_t1_dn4 = assign8120_e7813_d_n4;
        var_t1_dn5 = assign8120_e7813_d_n5;
        var_t1_dn6 = assign8120_e7813_d_n6;
        var_t1_dn8 = assign8120_e7813_d_n8;
        var_t1_dn10 = assign8120_e7813_d_n10;
        var_t1_dn11 = assign8120_e7813_d_n11;
        var_t1_dn12 = assign8120_e7813_d_n12;
        var_t1_rv = 0.0;

        let (assign8130_e7832, assign8130_e7832_d_n0, assign8130_e7832_d_n2, assign8130_e7832_d_n4, assign8130_e7832_d_n5, assign8130_e7832_d_n6, assign8130_e7832_d_n8, assign8130_e7832_d_n10, assign8130_e7832_d_n11, assign8130_e7832_d_n12,) = {
    if (var_guard74 == 0.0) {
        let (assign8130_e7830, assign8130_e7830_d_n0, assign8130_e7830_d_n2, assign8130_e7830_d_n4, assign8130_e7830_d_n5, assign8130_e7830_d_n6, assign8130_e7830_d_n8, assign8130_e7830_d_n10, assign8130_e7830_d_n11, assign8130_e7830_d_n12,) = {
            if (var_ps0_ini > 0.0) {
                let assign8130_e7821: f64 = (2.0 * 1.034943e-10);
                let assign8130_e7823: f64 = (assign8130_e7821 / 1.6021918e-19);
                let assign8130_e7825: f64 = (assign8130_e7823 * var_ps0_ini);
                let assign8130_e7827: f64 = (assign8130_e7825 / var_uc_nsubs);
                let assign8130_e7828: f64 = (assign8130_e7827).sqrt();
                (assign8130_e7828, (((((assign8130_e7823 * var_ps0_ini_dn0) * var_uc_nsubs) - (assign8130_e7825 * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign8130_e7828)), (((((assign8130_e7823 * var_ps0_ini_dn2) * var_uc_nsubs) - (assign8130_e7825 * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign8130_e7828)), (((((assign8130_e7823 * var_ps0_ini_dn4) * var_uc_nsubs) - (assign8130_e7825 * var_uc_nsubs_dn4)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign8130_e7828)), (((((assign8130_e7823 * var_ps0_ini_dn5) * var_uc_nsubs) - (assign8130_e7825 * var_uc_nsubs_dn5)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign8130_e7828)), (((((assign8130_e7823 * var_ps0_ini_dn6) * var_uc_nsubs) - (assign8130_e7825 * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign8130_e7828)), (((((assign8130_e7823 * var_ps0_ini_dn8) * var_uc_nsubs) - (assign8130_e7825 * var_uc_nsubs_dn8)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign8130_e7828)), (((((assign8130_e7823 * var_ps0_ini_dn10) * var_uc_nsubs) - (assign8130_e7825 * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign8130_e7828)), (((((assign8130_e7823 * var_ps0_ini_dn11) * var_uc_nsubs) - (assign8130_e7825 * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign8130_e7828)), (((((assign8130_e7823 * var_ps0_ini_dn12) * var_uc_nsubs) - (assign8130_e7825 * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign8130_e7828)),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign8130_e7830, assign8130_e7830_d_n0, assign8130_e7830_d_n2, assign8130_e7830_d_n4, assign8130_e7830_d_n5, assign8130_e7830_d_n6, assign8130_e7830_d_n8, assign8130_e7830_d_n10, assign8130_e7830_d_n11, assign8130_e7830_d_n12,)
    } else {
        (var_wdsoi, var_wdsoi_dn0, var_wdsoi_dn2, var_wdsoi_dn4, var_wdsoi_dn5, var_wdsoi_dn6, var_wdsoi_dn8, var_wdsoi_dn10, var_wdsoi_dn11, var_wdsoi_dn12,)
    }
};
        var_wdsoi = assign8130_e7832;
        var_wdsoi_dn0 = assign8130_e7832_d_n0;
        var_wdsoi_dn2 = assign8130_e7832_d_n2;
        var_wdsoi_dn4 = assign8130_e7832_d_n4;
        var_wdsoi_dn5 = assign8130_e7832_d_n5;
        var_wdsoi_dn6 = assign8130_e7832_d_n6;
        var_wdsoi_dn8 = assign8130_e7832_d_n8;
        var_wdsoi_dn10 = assign8130_e7832_d_n10;
        var_wdsoi_dn11 = assign8130_e7832_d_n11;
        var_wdsoi_dn12 = assign8130_e7832_d_n12;
        var_wdsoi_rv = 0.0;

        let assign8140_e7835: f64 = if var_wdsoi < p.p227 { 1.0 } else { 0.0 };
        var_guard100 = assign8140_e7835;
        var_guard100_rv = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn10_slot = var_arg_dn10;
        *var_arg_dn11_slot = var_arg_dn11;
        *var_arg_dn12_slot = var_arg_dn12;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_rv_slot = var_arg_rv;
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn11_slot = var_dnm_dn11;
        *var_dnm_dn12_slot = var_dnm_dn12;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn4_slot = var_dnm_dn4;
        *var_dnm_dn5_slot = var_dnm_dn5;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn8_slot = var_dnm_dn8;
        *var_dnm_rv_slot = var_dnm_rv;
        *var_guard100_slot = var_guard100;
        *var_guard100_rv_slot = var_guard100_rv;
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
        *var_m0_slot = var_m0;
        *var_m0_rv_slot = var_m0_rv;
        *var_mm_slot = var_mm;
        *var_mm_rv_slot = var_mm_rv;
        *var_ps0_ini_slot = var_ps0_ini;
        *var_ps0_ini_dn0_slot = var_ps0_ini_dn0;
        *var_ps0_ini_dn10_slot = var_ps0_ini_dn10;
        *var_ps0_ini_dn11_slot = var_ps0_ini_dn11;
        *var_ps0_ini_dn12_slot = var_ps0_ini_dn12;
        *var_ps0_ini_dn2_slot = var_ps0_ini_dn2;
        *var_ps0_ini_dn4_slot = var_ps0_ini_dn4;
        *var_ps0_ini_dn5_slot = var_ps0_ini_dn5;
        *var_ps0_ini_dn6_slot = var_ps0_ini_dn6;
        *var_ps0_ini_dn8_slot = var_ps0_ini_dn8;
        *var_ps0_ini_rv_slot = var_ps0_ini_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_tmf0_slot = var_tmf0;
        *var_tmf0_dn0_slot = var_tmf0_dn0;
        *var_tmf0_dn10_slot = var_tmf0_dn10;
        *var_tmf0_dn11_slot = var_tmf0_dn11;
        *var_tmf0_dn12_slot = var_tmf0_dn12;
        *var_tmf0_dn2_slot = var_tmf0_dn2;
        *var_tmf0_dn4_slot = var_tmf0_dn4;
        *var_tmf0_dn5_slot = var_tmf0_dn5;
        *var_tmf0_dn6_slot = var_tmf0_dn6;
        *var_tmf0_dn8_slot = var_tmf0_dn8;
        *var_tmf0_rv_slot = var_tmf0_rv;
        *var_wdsoi_slot = var_wdsoi;
        *var_wdsoi_dn0_slot = var_wdsoi_dn0;
        *var_wdsoi_dn10_slot = var_wdsoi_dn10;
        *var_wdsoi_dn11_slot = var_wdsoi_dn11;
        *var_wdsoi_dn12_slot = var_wdsoi_dn12;
        *var_wdsoi_dn2_slot = var_wdsoi_dn2;
        *var_wdsoi_dn4_slot = var_wdsoi_dn4;
        *var_wdsoi_dn5_slot = var_wdsoi_dn5;
        *var_wdsoi_dn6_slot = var_wdsoi_dn6;
        *var_wdsoi_dn8_slot = var_wdsoi_dn8;
        *var_wdsoi_rv_slot = var_wdsoi_rv;
        *var_xmax2_slot = var_xmax2;
        *var_xmax2_dn0_slot = var_xmax2_dn0;
        *var_xmax2_dn10_slot = var_xmax2_dn10;
        *var_xmax2_dn11_slot = var_xmax2_dn11;
        *var_xmax2_dn12_slot = var_xmax2_dn12;
        *var_xmax2_dn2_slot = var_xmax2_dn2;
        *var_xmax2_dn4_slot = var_xmax2_dn4;
        *var_xmax2_dn5_slot = var_xmax2_dn5;
        *var_xmax2_dn6_slot = var_xmax2_dn6;
        *var_xmax2_dn8_slot = var_xmax2_dn8;
        *var_xmax2_rv_slot = var_xmax2_rv;
        *var_xmp_slot = var_xmp;
        *var_xmp_dn0_slot = var_xmp_dn0;
        *var_xmp_dn10_slot = var_xmp_dn10;
        *var_xmp_dn11_slot = var_xmp_dn11;
        *var_xmp_dn12_slot = var_xmp_dn12;
        *var_xmp_dn2_slot = var_xmp_dn2;
        *var_xmp_dn4_slot = var_xmp_dn4;
        *var_xmp_dn5_slot = var_xmp_dn5;
        *var_xmp_dn6_slot = var_xmp_dn6;
        *var_xmp_dn8_slot = var_xmp_dn8;
        *var_xmp_rv_slot = var_xmp_rv;
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn11_slot = var_xp_dn11;
        *var_xp_dn12_slot = var_xp_dn12;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn4_slot = var_xp_dn4;
        *var_xp_dn5_slot = var_xp_dn5;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn8_slot = var_xp_dn8;
        *var_xp_rv_slot = var_xp_rv;
    }

    pub(super) fn stamp_reactive_block_29(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn4: f64,
        var_c_box_fd_inv: f64,
        var_cnst0bulk: f64,
        var_cnst0bulk_dn0: f64,
        var_cnst0bulk_dn10: f64,
        var_cnst0bulk_dn11: f64,
        var_cnst0bulk_dn12: f64,
        var_cnst0bulk_dn2: f64,
        var_cnst0bulk_dn4: f64,
        var_cnst0bulk_dn5: f64,
        var_cnst0bulk_dn6: f64,
        var_cnst0bulk_dn8: f64,
        var_cnst1bulk: f64,
        var_cnst1bulk_dn0: f64,
        var_cnst1bulk_dn10: f64,
        var_cnst1bulk_dn11: f64,
        var_cnst1bulk_dn12: f64,
        var_cnst1bulk_dn2: f64,
        var_cnst1bulk_dn4: f64,
        var_cnst1bulk_dn5: f64,
        var_cnst1bulk_dn6: f64,
        var_cnst1bulk_dn8: f64,
        var_guard100: f64,
        var_guard74: f64,
        var_pb2_bulk: f64,
        var_ps0_ini: f64,
        var_ps0_ini_dn0: f64,
        var_ps0_ini_dn10: f64,
        var_ps0_ini_dn11: f64,
        var_ps0_ini_dn12: f64,
        var_ps0_ini_dn2: f64,
        var_ps0_ini_dn4: f64,
        var_ps0_ini_dn5: f64,
        var_ps0_ini_dn6: f64,
        var_ps0_ini_dn8: f64,
        var_ps0_inia: f64,
        var_ps0_inia_dn0: f64,
        var_ps0_inia_dn10: f64,
        var_ps0_inia_dn11: f64,
        var_ps0_inia_dn12: f64,
        var_ps0_inia_dn2: f64,
        var_ps0_inia_dn4: f64,
        var_ps0_inia_dn5: f64,
        var_ps0_inia_dn6: f64,
        var_ps0_inia_dn8: f64,
        var_q_fd_soi: f64,
        var_q_fd_soi_dn0: f64,
        var_q_fd_soi_dn10: f64,
        var_q_fd_soi_dn11: f64,
        var_q_fd_soi_dn12: f64,
        var_q_fd_soi_dn2: f64,
        var_q_fd_soi_dn4: f64,
        var_q_fd_soi_dn5: f64,
        var_q_fd_soi_dn6: f64,
        var_q_fd_soi_dn8: f64,
        var_vbsbiz: f64,
        var_vbsbiz_dn0: f64,
        var_vbsbiz_dn10: f64,
        var_vbsbiz_dn11: f64,
        var_vbsbiz_dn12: f64,
        var_vbsbiz_dn2: f64,
        var_vbsbiz_dn4: f64,
        var_vbsbiz_dn5: f64,
        var_vbsbiz_dn6: f64,
        var_vbsbiz_dn8: f64,
        var_flg_depmode_slot: &mut f64,
        var_flg_depmode_rv_slot: &mut f64,
        var_guard101_slot: &mut f64,
        var_guard101_rv_slot: &mut f64,
        var_guard102_slot: &mut f64,
        var_guard102_rv_slot: &mut f64,
        var_phi_s0_bulk_slot: &mut f64,
        var_phi_s0_bulk_dn0_slot: &mut f64,
        var_phi_s0_bulk_dn10_slot: &mut f64,
        var_phi_s0_bulk_dn11_slot: &mut f64,
        var_phi_s0_bulk_dn12_slot: &mut f64,
        var_phi_s0_bulk_dn2_slot: &mut f64,
        var_phi_s0_bulk_dn4_slot: &mut f64,
        var_phi_s0_bulk_dn5_slot: &mut f64,
        var_phi_s0_bulk_dn6_slot: &mut f64,
        var_phi_s0_bulk_dn8_slot: &mut f64,
        var_phi_s0_bulk_rv_slot: &mut f64,
        var_phi_s0_soi_slot: &mut f64,
        var_phi_s0_soi_dn0_slot: &mut f64,
        var_phi_s0_soi_dn10_slot: &mut f64,
        var_phi_s0_soi_dn11_slot: &mut f64,
        var_phi_s0_soi_dn12_slot: &mut f64,
        var_phi_s0_soi_dn2_slot: &mut f64,
        var_phi_s0_soi_dn4_slot: &mut f64,
        var_phi_s0_soi_dn5_slot: &mut f64,
        var_phi_s0_soi_dn6_slot: &mut f64,
        var_phi_s0_soi_dn8_slot: &mut f64,
        var_phi_s0_soi_rv_slot: &mut f64,
        var_psb_inia_slot: &mut f64,
        var_psb_inia_dn0_slot: &mut f64,
        var_psb_inia_dn10_slot: &mut f64,
        var_psb_inia_dn11_slot: &mut f64,
        var_psb_inia_dn12_slot: &mut f64,
        var_psb_inia_dn2_slot: &mut f64,
        var_psb_inia_dn4_slot: &mut f64,
        var_psb_inia_dn5_slot: &mut f64,
        var_psb_inia_dn6_slot: &mut f64,
        var_psb_inia_dn8_slot: &mut f64,
        var_psb_inia_rv_slot: &mut f64,
        var_psb_inib_slot: &mut f64,
        var_psb_inib_dn0_slot: &mut f64,
        var_psb_inib_dn10_slot: &mut f64,
        var_psb_inib_dn11_slot: &mut f64,
        var_psb_inib_dn12_slot: &mut f64,
        var_psb_inib_dn2_slot: &mut f64,
        var_psb_inib_dn4_slot: &mut f64,
        var_psb_inib_dn5_slot: &mut f64,
        var_psb_inib_dn6_slot: &mut f64,
        var_psb_inib_dn8_slot: &mut f64,
        var_psb_inib_rv_slot: &mut f64,
        var_psl_lim_slot: &mut f64,
        var_psl_lim_dn0_slot: &mut f64,
        var_psl_lim_dn10_slot: &mut f64,
        var_psl_lim_dn11_slot: &mut f64,
        var_psl_lim_dn12_slot: &mut f64,
        var_psl_lim_dn2_slot: &mut f64,
        var_psl_lim_dn4_slot: &mut f64,
        var_psl_lim_dn5_slot: &mut f64,
        var_psl_lim_dn6_slot: &mut f64,
        var_psl_lim_dn8_slot: &mut f64,
        var_psl_lim_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_flg_depmode: f64 = *var_flg_depmode_slot;
        let mut var_flg_depmode_rv: f64 = *var_flg_depmode_rv_slot;
        let mut var_guard101: f64 = *var_guard101_slot;
        let mut var_guard101_rv: f64 = *var_guard101_rv_slot;
        let mut var_guard102: f64 = *var_guard102_slot;
        let mut var_guard102_rv: f64 = *var_guard102_rv_slot;
        let mut var_phi_s0_bulk: f64 = *var_phi_s0_bulk_slot;
        let mut var_phi_s0_bulk_dn0: f64 = *var_phi_s0_bulk_dn0_slot;
        let mut var_phi_s0_bulk_dn10: f64 = *var_phi_s0_bulk_dn10_slot;
        let mut var_phi_s0_bulk_dn11: f64 = *var_phi_s0_bulk_dn11_slot;
        let mut var_phi_s0_bulk_dn12: f64 = *var_phi_s0_bulk_dn12_slot;
        let mut var_phi_s0_bulk_dn2: f64 = *var_phi_s0_bulk_dn2_slot;
        let mut var_phi_s0_bulk_dn4: f64 = *var_phi_s0_bulk_dn4_slot;
        let mut var_phi_s0_bulk_dn5: f64 = *var_phi_s0_bulk_dn5_slot;
        let mut var_phi_s0_bulk_dn6: f64 = *var_phi_s0_bulk_dn6_slot;
        let mut var_phi_s0_bulk_dn8: f64 = *var_phi_s0_bulk_dn8_slot;
        let mut var_phi_s0_bulk_rv: f64 = *var_phi_s0_bulk_rv_slot;
        let mut var_phi_s0_soi: f64 = *var_phi_s0_soi_slot;
        let mut var_phi_s0_soi_dn0: f64 = *var_phi_s0_soi_dn0_slot;
        let mut var_phi_s0_soi_dn10: f64 = *var_phi_s0_soi_dn10_slot;
        let mut var_phi_s0_soi_dn11: f64 = *var_phi_s0_soi_dn11_slot;
        let mut var_phi_s0_soi_dn12: f64 = *var_phi_s0_soi_dn12_slot;
        let mut var_phi_s0_soi_dn2: f64 = *var_phi_s0_soi_dn2_slot;
        let mut var_phi_s0_soi_dn4: f64 = *var_phi_s0_soi_dn4_slot;
        let mut var_phi_s0_soi_dn5: f64 = *var_phi_s0_soi_dn5_slot;
        let mut var_phi_s0_soi_dn6: f64 = *var_phi_s0_soi_dn6_slot;
        let mut var_phi_s0_soi_dn8: f64 = *var_phi_s0_soi_dn8_slot;
        let mut var_phi_s0_soi_rv: f64 = *var_phi_s0_soi_rv_slot;
        let mut var_psb_inia: f64 = *var_psb_inia_slot;
        let mut var_psb_inia_dn0: f64 = *var_psb_inia_dn0_slot;
        let mut var_psb_inia_dn10: f64 = *var_psb_inia_dn10_slot;
        let mut var_psb_inia_dn11: f64 = *var_psb_inia_dn11_slot;
        let mut var_psb_inia_dn12: f64 = *var_psb_inia_dn12_slot;
        let mut var_psb_inia_dn2: f64 = *var_psb_inia_dn2_slot;
        let mut var_psb_inia_dn4: f64 = *var_psb_inia_dn4_slot;
        let mut var_psb_inia_dn5: f64 = *var_psb_inia_dn5_slot;
        let mut var_psb_inia_dn6: f64 = *var_psb_inia_dn6_slot;
        let mut var_psb_inia_dn8: f64 = *var_psb_inia_dn8_slot;
        let mut var_psb_inia_rv: f64 = *var_psb_inia_rv_slot;
        let mut var_psb_inib: f64 = *var_psb_inib_slot;
        let mut var_psb_inib_dn0: f64 = *var_psb_inib_dn0_slot;
        let mut var_psb_inib_dn10: f64 = *var_psb_inib_dn10_slot;
        let mut var_psb_inib_dn11: f64 = *var_psb_inib_dn11_slot;
        let mut var_psb_inib_dn12: f64 = *var_psb_inib_dn12_slot;
        let mut var_psb_inib_dn2: f64 = *var_psb_inib_dn2_slot;
        let mut var_psb_inib_dn4: f64 = *var_psb_inib_dn4_slot;
        let mut var_psb_inib_dn5: f64 = *var_psb_inib_dn5_slot;
        let mut var_psb_inib_dn6: f64 = *var_psb_inib_dn6_slot;
        let mut var_psb_inib_dn8: f64 = *var_psb_inib_dn8_slot;
        let mut var_psb_inib_rv: f64 = *var_psb_inib_rv_slot;
        let mut var_psl_lim: f64 = *var_psl_lim_slot;
        let mut var_psl_lim_dn0: f64 = *var_psl_lim_dn0_slot;
        let mut var_psl_lim_dn10: f64 = *var_psl_lim_dn10_slot;
        let mut var_psl_lim_dn11: f64 = *var_psl_lim_dn11_slot;
        let mut var_psl_lim_dn12: f64 = *var_psl_lim_dn12_slot;
        let mut var_psl_lim_dn2: f64 = *var_psl_lim_dn2_slot;
        let mut var_psl_lim_dn4: f64 = *var_psl_lim_dn4_slot;
        let mut var_psl_lim_dn5: f64 = *var_psl_lim_dn5_slot;
        let mut var_psl_lim_dn6: f64 = *var_psl_lim_dn6_slot;
        let mut var_psl_lim_dn8: f64 = *var_psl_lim_dn8_slot;
        let mut var_psl_lim_rv: f64 = *var_psl_lim_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign8150_e7842,) = {
    if ((var_guard74 == 0.0) && (var_guard100 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_depmode,)
    }
};
        var_flg_depmode = assign8150_e7842;
        var_flg_depmode_rv = 0.0;

        let (assign8160_e7850,) = {
    if ((var_guard74 == 0.0) && (var_guard100 == 0.0)) {
        (2.0,)
    } else {
        (var_flg_depmode,)
    }
};
        var_flg_depmode = assign8160_e7850;
        var_flg_depmode_rv = 0.0;

        let (assign8170_e7855, assign8170_e7855_d_n0, assign8170_e7855_d_n2, assign8170_e7855_d_n4, assign8170_e7855_d_n5, assign8170_e7855_d_n6, assign8170_e7855_d_n8, assign8170_e7855_d_n10, assign8170_e7855_d_n11, assign8170_e7855_d_n12,) = {
    if (var_guard74 == 0.0) {
        (var_ps0_ini, var_ps0_ini_dn0, var_ps0_ini_dn2, var_ps0_ini_dn4, var_ps0_ini_dn5, var_ps0_ini_dn6, var_ps0_ini_dn8, var_ps0_ini_dn10, var_ps0_ini_dn11, var_ps0_ini_dn12,)
    } else {
        (var_phi_s0_soi, var_phi_s0_soi_dn0, var_phi_s0_soi_dn2, var_phi_s0_soi_dn4, var_phi_s0_soi_dn5, var_phi_s0_soi_dn6, var_phi_s0_soi_dn8, var_phi_s0_soi_dn10, var_phi_s0_soi_dn11, var_phi_s0_soi_dn12,)
    }
};
        var_phi_s0_soi = assign8170_e7855;
        var_phi_s0_soi_dn0 = assign8170_e7855_d_n0;
        var_phi_s0_soi_dn2 = assign8170_e7855_d_n2;
        var_phi_s0_soi_dn4 = assign8170_e7855_d_n4;
        var_phi_s0_soi_dn5 = assign8170_e7855_d_n5;
        var_phi_s0_soi_dn6 = assign8170_e7855_d_n6;
        var_phi_s0_soi_dn8 = assign8170_e7855_d_n8;
        var_phi_s0_soi_dn10 = assign8170_e7855_d_n10;
        var_phi_s0_soi_dn11 = assign8170_e7855_d_n11;
        var_phi_s0_soi_dn12 = assign8170_e7855_d_n12;
        var_phi_s0_soi_rv = 0.0;

        let (assign8180_e7860, assign8180_e7860_d_n0, assign8180_e7860_d_n2, assign8180_e7860_d_n4, assign8180_e7860_d_n5, assign8180_e7860_d_n6, assign8180_e7860_d_n8, assign8180_e7860_d_n10, assign8180_e7860_d_n11, assign8180_e7860_d_n12,) = {
    if (var_guard74 == 0.0) {
        (var_ps0_inia, var_ps0_inia_dn0, var_ps0_inia_dn2, var_ps0_inia_dn4, var_ps0_inia_dn5, var_ps0_inia_dn6, var_ps0_inia_dn8, var_ps0_inia_dn10, var_ps0_inia_dn11, var_ps0_inia_dn12,)
    } else {
        (var_psl_lim, var_psl_lim_dn0, var_psl_lim_dn2, var_psl_lim_dn4, var_psl_lim_dn5, var_psl_lim_dn6, var_psl_lim_dn8, var_psl_lim_dn10, var_psl_lim_dn11, var_psl_lim_dn12,)
    }
};
        var_psl_lim = assign8180_e7860;
        var_psl_lim_dn0 = assign8180_e7860_d_n0;
        var_psl_lim_dn2 = assign8180_e7860_d_n2;
        var_psl_lim_dn4 = assign8180_e7860_d_n4;
        var_psl_lim_dn5 = assign8180_e7860_d_n5;
        var_psl_lim_dn6 = assign8180_e7860_d_n6;
        var_psl_lim_dn8 = assign8180_e7860_d_n8;
        var_psl_lim_dn10 = assign8180_e7860_d_n10;
        var_psl_lim_dn11 = assign8180_e7860_d_n11;
        var_psl_lim_dn12 = assign8180_e7860_d_n12;
        var_psl_lim_rv = 0.0;

        let (assign8190_e7871, assign8190_e7871_d_n0, assign8190_e7871_d_n2, assign8190_e7871_d_n4, assign8190_e7871_d_n5, assign8190_e7871_d_n6, assign8190_e7871_d_n8, assign8190_e7871_d_n10, assign8190_e7871_d_n11, assign8190_e7871_d_n12,) = {
    if (var_guard74 == 0.0) {
        let assign8190_e7865: f64 = (var_cnst0bulk * var_cnst0bulk);
        let assign8190_e7867: f64 = (assign8190_e7865 * var_c_box_fd_inv);
        let assign8190_e7869: f64 = (assign8190_e7867 * var_c_box_fd_inv);
        (assign8190_e7869, ((((var_cnst0bulk_dn0 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn0)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn2 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn2)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn4 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn4)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn5 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn5)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn6 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn6)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn8 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn8)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn10 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn10)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn11 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn11)) * var_c_box_fd_inv) * var_c_box_fd_inv), ((((var_cnst0bulk_dn12 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn12)) * var_c_box_fd_inv) * var_c_box_fd_inv),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
        var_t0 = assign8190_e7871;
        var_t0_dn0 = assign8190_e7871_d_n0;
        var_t0_dn2 = assign8190_e7871_d_n2;
        var_t0_dn4 = assign8190_e7871_d_n4;
        var_t0_dn5 = assign8190_e7871_d_n5;
        var_t0_dn6 = assign8190_e7871_d_n6;
        var_t0_dn8 = assign8190_e7871_d_n8;
        var_t0_dn10 = assign8190_e7871_d_n10;
        var_t0_dn11 = assign8190_e7871_d_n11;
        var_t0_dn12 = assign8190_e7871_d_n12;
        var_t0_rv = 0.0;

        let assign8200_e7874: f64 = if var_flg_depmode == 1.0 { 1.0 } else { 0.0 };
        var_guard101 = assign8200_e7874;
        var_guard101_rv = 0.0;

        let (assign8210_e7882, assign8210_e7882_d_n0, assign8210_e7882_d_n2, assign8210_e7882_d_n4, assign8210_e7882_d_n5, assign8210_e7882_d_n6, assign8210_e7882_d_n8, assign8210_e7882_d_n10, assign8210_e7882_d_n11, assign8210_e7882_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard101 != 0.0)) {
        let assign8210_e7880: f64 = (-var_vbsbiz);
        (assign8210_e7880, (-var_vbsbiz_dn0), (-var_vbsbiz_dn2), (-var_vbsbiz_dn4), (-var_vbsbiz_dn5), (-var_vbsbiz_dn6), (-var_vbsbiz_dn8), (-var_vbsbiz_dn10), (-var_vbsbiz_dn11), (-var_vbsbiz_dn12),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign8210_e7882;
        var_t1_dn0 = assign8210_e7882_d_n0;
        var_t1_dn2 = assign8210_e7882_d_n2;
        var_t1_dn4 = assign8210_e7882_d_n4;
        var_t1_dn5 = assign8210_e7882_d_n5;
        var_t1_dn6 = assign8210_e7882_d_n6;
        var_t1_dn8 = assign8210_e7882_d_n8;
        var_t1_dn10 = assign8210_e7882_d_n10;
        var_t1_dn11 = assign8210_e7882_d_n11;
        var_t1_dn12 = assign8210_e7882_d_n12;
        var_t1_rv = 0.0;

        let (assign8220_e7911, assign8220_e7911_d_n0, assign8220_e7911_d_n2, assign8220_e7911_d_n4, assign8220_e7911_d_n5, assign8220_e7911_d_n6, assign8220_e7911_d_n8, assign8220_e7911_d_n10, assign8220_e7911_d_n11, assign8220_e7911_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard101 != 0.0)) {
        let assign8220_e7889: f64 = (2.0 * var_t1);
        let assign8220_e7892: f64 = (var_t0 * var_beta);
        let assign8220_e7893: f64 = (assign8220_e7889 + assign8220_e7892);
        let assign8220_e7896: f64 = (2.0 * var_t1);
        let assign8220_e7899: f64 = (var_t0 * var_beta);
        let assign8220_e7900: f64 = (assign8220_e7896 + assign8220_e7899);
        let assign8220_e7901: f64 = (assign8220_e7893 * assign8220_e7900);
        let assign8220_e7905: f64 = (var_t1 * var_t1);
        let assign8220_e7907: f64 = (assign8220_e7905 + var_t0);
        let assign8220_e7908: f64 = (4.0 * assign8220_e7907);
        let assign8220_e7909: f64 = (assign8220_e7901 - assign8220_e7908);
        (assign8220_e7909, (((((2.0 * var_t1_dn0) + (var_t0_dn0 * var_beta)) * assign8220_e7900) + (assign8220_e7893 * ((2.0 * var_t1_dn0) + (var_t0_dn0 * var_beta)))) - (4.0 * (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) + var_t0_dn0))), (((((2.0 * var_t1_dn2) + (var_t0_dn2 * var_beta)) * assign8220_e7900) + (assign8220_e7893 * ((2.0 * var_t1_dn2) + (var_t0_dn2 * var_beta)))) - (4.0 * (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) + var_t0_dn2))), (((((2.0 * var_t1_dn4) + ((var_t0_dn4 * var_beta) + (var_t0 * var_beta_dn4))) * assign8220_e7900) + (assign8220_e7893 * ((2.0 * var_t1_dn4) + ((var_t0_dn4 * var_beta) + (var_t0 * var_beta_dn4))))) - (4.0 * (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) + var_t0_dn4))), (((((2.0 * var_t1_dn5) + (var_t0_dn5 * var_beta)) * assign8220_e7900) + (assign8220_e7893 * ((2.0 * var_t1_dn5) + (var_t0_dn5 * var_beta)))) - (4.0 * (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) + var_t0_dn5))), (((((2.0 * var_t1_dn6) + (var_t0_dn6 * var_beta)) * assign8220_e7900) + (assign8220_e7893 * ((2.0 * var_t1_dn6) + (var_t0_dn6 * var_beta)))) - (4.0 * (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) + var_t0_dn6))), (((((2.0 * var_t1_dn8) + (var_t0_dn8 * var_beta)) * assign8220_e7900) + (assign8220_e7893 * ((2.0 * var_t1_dn8) + (var_t0_dn8 * var_beta)))) - (4.0 * (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) + var_t0_dn8))), (((((2.0 * var_t1_dn10) + (var_t0_dn10 * var_beta)) * assign8220_e7900) + (assign8220_e7893 * ((2.0 * var_t1_dn10) + (var_t0_dn10 * var_beta)))) - (4.0 * (((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) + var_t0_dn10))), (((((2.0 * var_t1_dn11) + (var_t0_dn11 * var_beta)) * assign8220_e7900) + (assign8220_e7893 * ((2.0 * var_t1_dn11) + (var_t0_dn11 * var_beta)))) - (4.0 * (((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) + var_t0_dn11))), (((((2.0 * var_t1_dn12) + (var_t0_dn12 * var_beta)) * assign8220_e7900) + (assign8220_e7893 * ((2.0 * var_t1_dn12) + (var_t0_dn12 * var_beta)))) - (4.0 * (((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)) + var_t0_dn12))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign8220_e7911;
        var_t2_dn0 = assign8220_e7911_d_n0;
        var_t2_dn2 = assign8220_e7911_d_n2;
        var_t2_dn4 = assign8220_e7911_d_n4;
        var_t2_dn5 = assign8220_e7911_d_n5;
        var_t2_dn6 = assign8220_e7911_d_n6;
        var_t2_dn8 = assign8220_e7911_d_n8;
        var_t2_dn10 = assign8220_e7911_d_n10;
        var_t2_dn11 = assign8220_e7911_d_n11;
        var_t2_dn12 = assign8220_e7911_d_n12;
        var_t2_rv = 0.0;

        let (assign8230_e7927, assign8230_e7927_d_n0, assign8230_e7927_d_n2, assign8230_e7927_d_n4, assign8230_e7927_d_n5, assign8230_e7927_d_n6, assign8230_e7927_d_n8, assign8230_e7927_d_n10, assign8230_e7927_d_n11, assign8230_e7927_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard101 != 0.0)) {
        let assign8230_e7919: f64 = (10.0 * 2.220446049250313e-16);
        let (assign8230_e7925, assign8230_e7925_d_n0, assign8230_e7925_d_n2, assign8230_e7925_d_n4, assign8230_e7925_d_n5, assign8230_e7925_d_n6, assign8230_e7925_d_n8, assign8230_e7925_d_n10, assign8230_e7925_d_n11, assign8230_e7925_d_n12,) = {
            if (var_t2 >= assign8230_e7919) {
                (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
            } else {
                let assign8230_e7924: f64 = (10.0 * 2.220446049250313e-16);
                (assign8230_e7924, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign8230_e7925, assign8230_e7925_d_n0, assign8230_e7925_d_n2, assign8230_e7925_d_n4, assign8230_e7925_d_n5, assign8230_e7925_d_n6, assign8230_e7925_d_n8, assign8230_e7925_d_n10, assign8230_e7925_d_n11, assign8230_e7925_d_n12,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign8230_e7927;
        var_t2_dn0 = assign8230_e7927_d_n0;
        var_t2_dn2 = assign8230_e7927_d_n2;
        var_t2_dn4 = assign8230_e7927_d_n4;
        var_t2_dn5 = assign8230_e7927_d_n5;
        var_t2_dn6 = assign8230_e7927_d_n6;
        var_t2_dn8 = assign8230_e7927_d_n8;
        var_t2_dn10 = assign8230_e7927_d_n10;
        var_t2_dn11 = assign8230_e7927_d_n11;
        var_t2_dn12 = assign8230_e7927_d_n12;
        var_t2_rv = 0.0;

        let (assign8240_e7935, assign8240_e7935_d_n0, assign8240_e7935_d_n2, assign8240_e7935_d_n4, assign8240_e7935_d_n5, assign8240_e7935_d_n6, assign8240_e7935_d_n8, assign8240_e7935_d_n10, assign8240_e7935_d_n11, assign8240_e7935_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard101 != 0.0)) {
        let assign8240_e7933: f64 = (var_t2).sqrt();
        (assign8240_e7933, (var_t2_dn0 / (2.0 * assign8240_e7933)), (var_t2_dn2 / (2.0 * assign8240_e7933)), (var_t2_dn4 / (2.0 * assign8240_e7933)), (var_t2_dn5 / (2.0 * assign8240_e7933)), (var_t2_dn6 / (2.0 * assign8240_e7933)), (var_t2_dn8 / (2.0 * assign8240_e7933)), (var_t2_dn10 / (2.0 * assign8240_e7933)), (var_t2_dn11 / (2.0 * assign8240_e7933)), (var_t2_dn12 / (2.0 * assign8240_e7933)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign8240_e7935;
        var_t2_dn0 = assign8240_e7935_d_n0;
        var_t2_dn2 = assign8240_e7935_d_n2;
        var_t2_dn4 = assign8240_e7935_d_n4;
        var_t2_dn5 = assign8240_e7935_d_n5;
        var_t2_dn6 = assign8240_e7935_d_n6;
        var_t2_dn8 = assign8240_e7935_d_n8;
        var_t2_dn10 = assign8240_e7935_d_n10;
        var_t2_dn11 = assign8240_e7935_d_n11;
        var_t2_dn12 = assign8240_e7935_d_n12;
        var_t2_rv = 0.0;

        let (assign8250_e7948, assign8250_e7948_d_n0, assign8250_e7948_d_n2, assign8250_e7948_d_n4, assign8250_e7948_d_n5, assign8250_e7948_d_n6, assign8250_e7948_d_n8, assign8250_e7948_d_n10, assign8250_e7948_d_n11, assign8250_e7948_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard101 != 0.0)) {
        let assign8250_e7942: f64 = (2.0 * var_t1);
        let assign8250_e7945: f64 = (var_t0 * var_beta);
        let assign8250_e7946: f64 = (assign8250_e7942 + assign8250_e7945);
        (assign8250_e7946, ((2.0 * var_t1_dn0) + (var_t0_dn0 * var_beta)), ((2.0 * var_t1_dn2) + (var_t0_dn2 * var_beta)), ((2.0 * var_t1_dn4) + ((var_t0_dn4 * var_beta) + (var_t0 * var_beta_dn4))), ((2.0 * var_t1_dn5) + (var_t0_dn5 * var_beta)), ((2.0 * var_t1_dn6) + (var_t0_dn6 * var_beta)), ((2.0 * var_t1_dn8) + (var_t0_dn8 * var_beta)), ((2.0 * var_t1_dn10) + (var_t0_dn10 * var_beta)), ((2.0 * var_t1_dn11) + (var_t0_dn11 * var_beta)), ((2.0 * var_t1_dn12) + (var_t0_dn12 * var_beta)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign8250_e7948;
        var_t3_dn0 = assign8250_e7948_d_n0;
        var_t3_dn2 = assign8250_e7948_d_n2;
        var_t3_dn4 = assign8250_e7948_d_n4;
        var_t3_dn5 = assign8250_e7948_d_n5;
        var_t3_dn6 = assign8250_e7948_d_n6;
        var_t3_dn8 = assign8250_e7948_d_n8;
        var_t3_dn10 = assign8250_e7948_d_n10;
        var_t3_dn11 = assign8250_e7948_d_n11;
        var_t3_dn12 = assign8250_e7948_d_n12;
        var_t3_rv = 0.0;

        let (assign8260_e7959, assign8260_e7959_d_n0, assign8260_e7959_d_n2, assign8260_e7959_d_n4, assign8260_e7959_d_n5, assign8260_e7959_d_n6, assign8260_e7959_d_n8, assign8260_e7959_d_n10, assign8260_e7959_d_n11, assign8260_e7959_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard101 != 0.0)) {
        let assign8260_e7955: f64 = (var_t3 - var_t2);
        let assign8260_e7957: f64 = (assign8260_e7955 / 2.0);
        (assign8260_e7957, ((var_t3_dn0 - var_t2_dn0) / 2.0), ((var_t3_dn2 - var_t2_dn2) / 2.0), ((var_t3_dn4 - var_t2_dn4) / 2.0), ((var_t3_dn5 - var_t2_dn5) / 2.0), ((var_t3_dn6 - var_t2_dn6) / 2.0), ((var_t3_dn8 - var_t2_dn8) / 2.0), ((var_t3_dn10 - var_t2_dn10) / 2.0), ((var_t3_dn11 - var_t2_dn11) / 2.0), ((var_t3_dn12 - var_t2_dn12) / 2.0),)
    } else {
        (var_psb_inia, var_psb_inia_dn0, var_psb_inia_dn2, var_psb_inia_dn4, var_psb_inia_dn5, var_psb_inia_dn6, var_psb_inia_dn8, var_psb_inia_dn10, var_psb_inia_dn11, var_psb_inia_dn12,)
    }
};
        var_psb_inia = assign8260_e7959;
        var_psb_inia_dn0 = assign8260_e7959_d_n0;
        var_psb_inia_dn2 = assign8260_e7959_d_n2;
        var_psb_inia_dn4 = assign8260_e7959_d_n4;
        var_psb_inia_dn5 = assign8260_e7959_d_n5;
        var_psb_inia_dn6 = assign8260_e7959_d_n6;
        var_psb_inia_dn8 = assign8260_e7959_d_n8;
        var_psb_inia_dn10 = assign8260_e7959_d_n10;
        var_psb_inia_dn11 = assign8260_e7959_d_n11;
        var_psb_inia_dn12 = assign8260_e7959_d_n12;
        var_psb_inia_rv = 0.0;

        let (assign8270_e7979, assign8270_e7979_d_n0, assign8270_e7979_d_n2, assign8270_e7979_d_n4, assign8270_e7979_d_n5, assign8270_e7979_d_n6, assign8270_e7979_d_n8, assign8270_e7979_d_n10, assign8270_e7979_d_n11, assign8270_e7979_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard101 != 0.0)) {
        let assign8270_e7966: f64 = (var_t1 * var_t1);
        let assign8270_e7968: f64 = (assign8270_e7966 / var_t0);
        let assign8270_e7970: f64 = (assign8270_e7968 / var_cnst1bulk);
        let assign8270_e7971: f64 = (assign8270_e7970).ln();
        let assign8270_e7975: f64 = (2.0 / var_t1);
        let assign8270_e7976: f64 = (var_beta + assign8270_e7975);
        let assign8270_e7977: f64 = (assign8270_e7971 / assign8270_e7976);
        (assign8270_e7977, ((((((((((((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) * var_t0) - (assign8270_e7966 * var_t0_dn0)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8270_e7968 * var_cnst1bulk_dn0)) / (var_cnst1bulk * var_cnst1bulk)) / assign8270_e7970) * assign8270_e7976) - (assign8270_e7971 * (-((2.0 * var_t1_dn0) / (var_t1 * var_t1))))) / (assign8270_e7976 * assign8270_e7976)), ((((((((((((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) * var_t0) - (assign8270_e7966 * var_t0_dn2)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8270_e7968 * var_cnst1bulk_dn2)) / (var_cnst1bulk * var_cnst1bulk)) / assign8270_e7970) * assign8270_e7976) - (assign8270_e7971 * (-((2.0 * var_t1_dn2) / (var_t1 * var_t1))))) / (assign8270_e7976 * assign8270_e7976)), ((((((((((((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) * var_t0) - (assign8270_e7966 * var_t0_dn4)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8270_e7968 * var_cnst1bulk_dn4)) / (var_cnst1bulk * var_cnst1bulk)) / assign8270_e7970) * assign8270_e7976) - (assign8270_e7971 * (var_beta_dn4 + (-((2.0 * var_t1_dn4) / (var_t1 * var_t1)))))) / (assign8270_e7976 * assign8270_e7976)), ((((((((((((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) * var_t0) - (assign8270_e7966 * var_t0_dn5)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8270_e7968 * var_cnst1bulk_dn5)) / (var_cnst1bulk * var_cnst1bulk)) / assign8270_e7970) * assign8270_e7976) - (assign8270_e7971 * (-((2.0 * var_t1_dn5) / (var_t1 * var_t1))))) / (assign8270_e7976 * assign8270_e7976)), ((((((((((((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) * var_t0) - (assign8270_e7966 * var_t0_dn6)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8270_e7968 * var_cnst1bulk_dn6)) / (var_cnst1bulk * var_cnst1bulk)) / assign8270_e7970) * assign8270_e7976) - (assign8270_e7971 * (-((2.0 * var_t1_dn6) / (var_t1 * var_t1))))) / (assign8270_e7976 * assign8270_e7976)), ((((((((((((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) * var_t0) - (assign8270_e7966 * var_t0_dn8)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8270_e7968 * var_cnst1bulk_dn8)) / (var_cnst1bulk * var_cnst1bulk)) / assign8270_e7970) * assign8270_e7976) - (assign8270_e7971 * (-((2.0 * var_t1_dn8) / (var_t1 * var_t1))))) / (assign8270_e7976 * assign8270_e7976)), ((((((((((((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) * var_t0) - (assign8270_e7966 * var_t0_dn10)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8270_e7968 * var_cnst1bulk_dn10)) / (var_cnst1bulk * var_cnst1bulk)) / assign8270_e7970) * assign8270_e7976) - (assign8270_e7971 * (-((2.0 * var_t1_dn10) / (var_t1 * var_t1))))) / (assign8270_e7976 * assign8270_e7976)), ((((((((((((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) * var_t0) - (assign8270_e7966 * var_t0_dn11)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8270_e7968 * var_cnst1bulk_dn11)) / (var_cnst1bulk * var_cnst1bulk)) / assign8270_e7970) * assign8270_e7976) - (assign8270_e7971 * (-((2.0 * var_t1_dn11) / (var_t1 * var_t1))))) / (assign8270_e7976 * assign8270_e7976)), ((((((((((((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)) * var_t0) - (assign8270_e7966 * var_t0_dn12)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8270_e7968 * var_cnst1bulk_dn12)) / (var_cnst1bulk * var_cnst1bulk)) / assign8270_e7970) * assign8270_e7976) - (assign8270_e7971 * (-((2.0 * var_t1_dn12) / (var_t1 * var_t1))))) / (assign8270_e7976 * assign8270_e7976)),)
    } else {
        (var_psb_inib, var_psb_inib_dn0, var_psb_inib_dn2, var_psb_inib_dn4, var_psb_inib_dn5, var_psb_inib_dn6, var_psb_inib_dn8, var_psb_inib_dn10, var_psb_inib_dn11, var_psb_inib_dn12,)
    }
};
        var_psb_inib = assign8270_e7979;
        var_psb_inib_dn0 = assign8270_e7979_d_n0;
        var_psb_inib_dn2 = assign8270_e7979_d_n2;
        var_psb_inib_dn4 = assign8270_e7979_d_n4;
        var_psb_inib_dn5 = assign8270_e7979_d_n5;
        var_psb_inib_dn6 = assign8270_e7979_d_n6;
        var_psb_inib_dn8 = assign8270_e7979_d_n8;
        var_psb_inib_dn10 = assign8270_e7979_d_n10;
        var_psb_inib_dn11 = assign8270_e7979_d_n11;
        var_psb_inib_dn12 = assign8270_e7979_d_n12;
        var_psb_inib_rv = 0.0;

        let assign8280_e7982: f64 = if var_psb_inia < var_pb2_bulk { 1.0 } else { 0.0 };
        var_guard102 = assign8280_e7982;
        var_guard102_rv = 0.0;

        let (assign8290_e7991, assign8290_e7991_d_n0, assign8290_e7991_d_n2, assign8290_e7991_d_n4, assign8290_e7991_d_n5, assign8290_e7991_d_n6, assign8290_e7991_d_n8, assign8290_e7991_d_n10, assign8290_e7991_d_n11, assign8290_e7991_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard101 != 0.0)) && (var_guard102 != 0.0)) {
        (var_psb_inia, var_psb_inia_dn0, var_psb_inia_dn2, var_psb_inia_dn4, var_psb_inia_dn5, var_psb_inia_dn6, var_psb_inia_dn8, var_psb_inia_dn10, var_psb_inia_dn11, var_psb_inia_dn12,)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
        var_phi_s0_bulk = assign8290_e7991;
        var_phi_s0_bulk_dn0 = assign8290_e7991_d_n0;
        var_phi_s0_bulk_dn2 = assign8290_e7991_d_n2;
        var_phi_s0_bulk_dn4 = assign8290_e7991_d_n4;
        var_phi_s0_bulk_dn5 = assign8290_e7991_d_n5;
        var_phi_s0_bulk_dn6 = assign8290_e7991_d_n6;
        var_phi_s0_bulk_dn8 = assign8290_e7991_d_n8;
        var_phi_s0_bulk_dn10 = assign8290_e7991_d_n10;
        var_phi_s0_bulk_dn11 = assign8290_e7991_d_n11;
        var_phi_s0_bulk_dn12 = assign8290_e7991_d_n12;
        var_phi_s0_bulk_rv = 0.0;

        let (assign8300_e8005, assign8300_e8005_d_n0, assign8300_e8005_d_n2, assign8300_e8005_d_n4, assign8300_e8005_d_n5, assign8300_e8005_d_n6, assign8300_e8005_d_n8, assign8300_e8005_d_n10, assign8300_e8005_d_n11, assign8300_e8005_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard101 != 0.0)) && (var_guard102 == 0.0)) {
        let assign8300_e8001: f64 = (var_psb_inib - var_psb_inia);
        let assign8300_e8003: f64 = (assign8300_e8001 - 0.0008);
        (assign8300_e8003, (var_psb_inib_dn0 - var_psb_inia_dn0), (var_psb_inib_dn2 - var_psb_inia_dn2), (var_psb_inib_dn4 - var_psb_inia_dn4), (var_psb_inib_dn5 - var_psb_inia_dn5), (var_psb_inib_dn6 - var_psb_inia_dn6), (var_psb_inib_dn8 - var_psb_inia_dn8), (var_psb_inib_dn10 - var_psb_inia_dn10), (var_psb_inib_dn11 - var_psb_inia_dn11), (var_psb_inib_dn12 - var_psb_inia_dn12),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn8, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12,)
    }
};
        var_tmf1 = assign8300_e8005;
        var_tmf1_dn0 = assign8300_e8005_d_n0;
        var_tmf1_dn2 = assign8300_e8005_d_n2;
        var_tmf1_dn4 = assign8300_e8005_d_n4;
        var_tmf1_dn5 = assign8300_e8005_d_n5;
        var_tmf1_dn6 = assign8300_e8005_d_n6;
        var_tmf1_dn8 = assign8300_e8005_d_n8;
        var_tmf1_dn10 = assign8300_e8005_d_n10;
        var_tmf1_dn11 = assign8300_e8005_d_n11;
        var_tmf1_dn12 = assign8300_e8005_d_n12;
        var_tmf1_rv = 0.0;

        let (assign8310_e8019, assign8310_e8019_d_n0, assign8310_e8019_d_n2, assign8310_e8019_d_n4, assign8310_e8019_d_n5, assign8310_e8019_d_n6, assign8310_e8019_d_n8, assign8310_e8019_d_n10, assign8310_e8019_d_n11, assign8310_e8019_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard101 != 0.0)) && (var_guard102 == 0.0)) {
        let assign8310_e8015: f64 = (4.0 * var_psb_inib);
        let assign8310_e8017: f64 = (assign8310_e8015 * 0.0008);
        (assign8310_e8017, ((4.0 * var_psb_inib_dn0) * 0.0008), ((4.0 * var_psb_inib_dn2) * 0.0008), ((4.0 * var_psb_inib_dn4) * 0.0008), ((4.0 * var_psb_inib_dn5) * 0.0008), ((4.0 * var_psb_inib_dn6) * 0.0008), ((4.0 * var_psb_inib_dn8) * 0.0008), ((4.0 * var_psb_inib_dn10) * 0.0008), ((4.0 * var_psb_inib_dn11) * 0.0008), ((4.0 * var_psb_inib_dn12) * 0.0008),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
        var_tmf2 = assign8310_e8019;
        var_tmf2_dn0 = assign8310_e8019_d_n0;
        var_tmf2_dn2 = assign8310_e8019_d_n2;
        var_tmf2_dn4 = assign8310_e8019_d_n4;
        var_tmf2_dn5 = assign8310_e8019_d_n5;
        var_tmf2_dn6 = assign8310_e8019_d_n6;
        var_tmf2_dn8 = assign8310_e8019_d_n8;
        var_tmf2_dn10 = assign8310_e8019_d_n10;
        var_tmf2_dn11 = assign8310_e8019_d_n11;
        var_tmf2_dn12 = assign8310_e8019_d_n12;
        var_tmf2_rv = 0.0;

        let (assign8320_e8035, assign8320_e8035_d_n0, assign8320_e8035_d_n2, assign8320_e8035_d_n4, assign8320_e8035_d_n5, assign8320_e8035_d_n6, assign8320_e8035_d_n8, assign8320_e8035_d_n10, assign8320_e8035_d_n11, assign8320_e8035_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard101 != 0.0)) && (var_guard102 == 0.0)) {
        let (assign8320_e8033, assign8320_e8033_d_n0, assign8320_e8033_d_n2, assign8320_e8033_d_n4, assign8320_e8033_d_n5, assign8320_e8033_d_n6, assign8320_e8033_d_n8, assign8320_e8033_d_n10, assign8320_e8033_d_n11, assign8320_e8033_d_n12,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
            } else {
                let assign8320_e8032: f64 = (-var_tmf2);
                (assign8320_e8032, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn8), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12),)
            }
        };
        (assign8320_e8033, assign8320_e8033_d_n0, assign8320_e8033_d_n2, assign8320_e8033_d_n4, assign8320_e8033_d_n5, assign8320_e8033_d_n6, assign8320_e8033_d_n8, assign8320_e8033_d_n10, assign8320_e8033_d_n11, assign8320_e8033_d_n12,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
        var_tmf2 = assign8320_e8035;
        var_tmf2_dn0 = assign8320_e8035_d_n0;
        var_tmf2_dn2 = assign8320_e8035_d_n2;
        var_tmf2_dn4 = assign8320_e8035_d_n4;
        var_tmf2_dn5 = assign8320_e8035_d_n5;
        var_tmf2_dn6 = assign8320_e8035_d_n6;
        var_tmf2_dn8 = assign8320_e8035_d_n8;
        var_tmf2_dn10 = assign8320_e8035_d_n10;
        var_tmf2_dn11 = assign8320_e8035_d_n11;
        var_tmf2_dn12 = assign8320_e8035_d_n12;
        var_tmf2_rv = 0.0;

        let (assign8330_e8050, assign8330_e8050_d_n0, assign8330_e8050_d_n2, assign8330_e8050_d_n4, assign8330_e8050_d_n5, assign8330_e8050_d_n6, assign8330_e8050_d_n8, assign8330_e8050_d_n10, assign8330_e8050_d_n11, assign8330_e8050_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard101 != 0.0)) && (var_guard102 == 0.0)) {
        let assign8330_e8045: f64 = (var_tmf1 * var_tmf1);
        let assign8330_e8047: f64 = (assign8330_e8045 + var_tmf2);
        let assign8330_e8048: f64 = (assign8330_e8047).sqrt();
        (assign8330_e8048, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign8330_e8048)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign8330_e8048)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign8330_e8048)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign8330_e8048)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign8330_e8048)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign8330_e8048)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign8330_e8048)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign8330_e8048)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign8330_e8048)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
        var_tmf2 = assign8330_e8050;
        var_tmf2_dn0 = assign8330_e8050_d_n0;
        var_tmf2_dn2 = assign8330_e8050_d_n2;
        var_tmf2_dn4 = assign8330_e8050_d_n4;
        var_tmf2_dn5 = assign8330_e8050_d_n5;
        var_tmf2_dn6 = assign8330_e8050_d_n6;
        var_tmf2_dn8 = assign8330_e8050_d_n8;
        var_tmf2_dn10 = assign8330_e8050_d_n10;
        var_tmf2_dn11 = assign8330_e8050_d_n11;
        var_tmf2_dn12 = assign8330_e8050_d_n12;
        var_tmf2_rv = 0.0;

        let (assign8340_e8066, assign8340_e8066_d_n0, assign8340_e8066_d_n2, assign8340_e8066_d_n4, assign8340_e8066_d_n5, assign8340_e8066_d_n6, assign8340_e8066_d_n8, assign8340_e8066_d_n10, assign8340_e8066_d_n11, assign8340_e8066_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard101 != 0.0)) && (var_guard102 == 0.0)) {
        let assign8340_e8062: f64 = (var_tmf1 / var_tmf2);
        let assign8340_e8063: f64 = (1.0 + assign8340_e8062);
        let assign8340_e8064: f64 = (0.5 * assign8340_e8063);
        (assign8340_e8064, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn12 * var_tmf2) - (var_tmf1 * var_tmf2_dn12)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign8340_e8066;
        var_t1_dn0 = assign8340_e8066_d_n0;
        var_t1_dn2 = assign8340_e8066_d_n2;
        var_t1_dn4 = assign8340_e8066_d_n4;
        var_t1_dn5 = assign8340_e8066_d_n5;
        var_t1_dn6 = assign8340_e8066_d_n6;
        var_t1_dn8 = assign8340_e8066_d_n8;
        var_t1_dn10 = assign8340_e8066_d_n10;
        var_t1_dn11 = assign8340_e8066_d_n11;
        var_t1_dn12 = assign8340_e8066_d_n12;
        var_t1_rv = 0.0;

        let (assign8350_e8082, assign8350_e8082_d_n0, assign8350_e8082_d_n2, assign8350_e8082_d_n4, assign8350_e8082_d_n5, assign8350_e8082_d_n6, assign8350_e8082_d_n8, assign8350_e8082_d_n10, assign8350_e8082_d_n11, assign8350_e8082_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard101 != 0.0)) && (var_guard102 == 0.0)) {
        let assign8350_e8078: f64 = (var_tmf1 + var_tmf2);
        let assign8350_e8079: f64 = (0.5 * assign8350_e8078);
        let assign8350_e8080: f64 = (var_psb_inib - assign8350_e8079);
        (assign8350_e8080, (var_psb_inib_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_psb_inib_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_psb_inib_dn4 - (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_psb_inib_dn5 - (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_psb_inib_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_psb_inib_dn8 - (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), (var_psb_inib_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_psb_inib_dn11 - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (var_psb_inib_dn12 - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))),)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
        var_phi_s0_bulk = assign8350_e8082;
        var_phi_s0_bulk_dn0 = assign8350_e8082_d_n0;
        var_phi_s0_bulk_dn2 = assign8350_e8082_d_n2;
        var_phi_s0_bulk_dn4 = assign8350_e8082_d_n4;
        var_phi_s0_bulk_dn5 = assign8350_e8082_d_n5;
        var_phi_s0_bulk_dn6 = assign8350_e8082_d_n6;
        var_phi_s0_bulk_dn8 = assign8350_e8082_d_n8;
        var_phi_s0_bulk_dn10 = assign8350_e8082_d_n10;
        var_phi_s0_bulk_dn11 = assign8350_e8082_d_n11;
        var_phi_s0_bulk_dn12 = assign8350_e8082_d_n12;
        var_phi_s0_bulk_rv = 0.0;

        let (assign8360_e8101, assign8360_e8101_d_n0, assign8360_e8101_d_n2, assign8360_e8101_d_n4, assign8360_e8101_d_n5, assign8360_e8101_d_n6, assign8360_e8101_d_n8, assign8360_e8101_d_n10, assign8360_e8101_d_n11, assign8360_e8101_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard101 == 0.0)) {
        let assign8360_e8090: f64 = (var_vbsbiz - var_phi_s0_soi);
        let assign8360_e8093: f64 = (var_q_fd_soi / 2.0);
        let assign8360_e8095: f64 = (assign8360_e8093 * p.p227);
        let assign8360_e8097: f64 = (assign8360_e8095 / 1.034943e-10);
        let assign8360_e8098: f64 = (assign8360_e8090 - assign8360_e8097);
        let assign8360_e8099: f64 = (-assign8360_e8098);
        (assign8360_e8099, (-((var_vbsbiz_dn0 - var_phi_s0_soi_dn0) - (((var_q_fd_soi_dn0 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn2 - var_phi_s0_soi_dn2) - (((var_q_fd_soi_dn2 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn4 - var_phi_s0_soi_dn4) - (((var_q_fd_soi_dn4 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn5 - var_phi_s0_soi_dn5) - (((var_q_fd_soi_dn5 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn6 - var_phi_s0_soi_dn6) - (((var_q_fd_soi_dn6 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn8 - var_phi_s0_soi_dn8) - (((var_q_fd_soi_dn8 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn10 - var_phi_s0_soi_dn10) - (((var_q_fd_soi_dn10 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn11 - var_phi_s0_soi_dn11) - (((var_q_fd_soi_dn11 / 2.0) * p.p227) / 1.034943e-10))), (-((var_vbsbiz_dn12 - var_phi_s0_soi_dn12) - (((var_q_fd_soi_dn12 / 2.0) * p.p227) / 1.034943e-10))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign8360_e8101;
        var_t1_dn0 = assign8360_e8101_d_n0;
        var_t1_dn2 = assign8360_e8101_d_n2;
        var_t1_dn4 = assign8360_e8101_d_n4;
        var_t1_dn5 = assign8360_e8101_d_n5;
        var_t1_dn6 = assign8360_e8101_d_n6;
        var_t1_dn8 = assign8360_e8101_d_n8;
        var_t1_dn10 = assign8360_e8101_d_n10;
        var_t1_dn11 = assign8360_e8101_d_n11;
        var_t1_dn12 = assign8360_e8101_d_n12;
        var_t1_rv = 0.0;

        let (assign8370_e8131, assign8370_e8131_d_n0, assign8370_e8131_d_n2, assign8370_e8131_d_n4, assign8370_e8131_d_n5, assign8370_e8131_d_n6, assign8370_e8131_d_n8, assign8370_e8131_d_n10, assign8370_e8131_d_n11, assign8370_e8131_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard101 == 0.0)) {
        let assign8370_e8109: f64 = (2.0 * var_t1);
        let assign8370_e8112: f64 = (var_t0 * var_beta);
        let assign8370_e8113: f64 = (assign8370_e8109 + assign8370_e8112);
        let assign8370_e8116: f64 = (2.0 * var_t1);
        let assign8370_e8119: f64 = (var_t0 * var_beta);
        let assign8370_e8120: f64 = (assign8370_e8116 + assign8370_e8119);
        let assign8370_e8121: f64 = (assign8370_e8113 * assign8370_e8120);
        let assign8370_e8125: f64 = (var_t1 * var_t1);
        let assign8370_e8127: f64 = (assign8370_e8125 + var_t0);
        let assign8370_e8128: f64 = (4.0 * assign8370_e8127);
        let assign8370_e8129: f64 = (assign8370_e8121 - assign8370_e8128);
        (assign8370_e8129, (((((2.0 * var_t1_dn0) + (var_t0_dn0 * var_beta)) * assign8370_e8120) + (assign8370_e8113 * ((2.0 * var_t1_dn0) + (var_t0_dn0 * var_beta)))) - (4.0 * (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) + var_t0_dn0))), (((((2.0 * var_t1_dn2) + (var_t0_dn2 * var_beta)) * assign8370_e8120) + (assign8370_e8113 * ((2.0 * var_t1_dn2) + (var_t0_dn2 * var_beta)))) - (4.0 * (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) + var_t0_dn2))), (((((2.0 * var_t1_dn4) + ((var_t0_dn4 * var_beta) + (var_t0 * var_beta_dn4))) * assign8370_e8120) + (assign8370_e8113 * ((2.0 * var_t1_dn4) + ((var_t0_dn4 * var_beta) + (var_t0 * var_beta_dn4))))) - (4.0 * (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) + var_t0_dn4))), (((((2.0 * var_t1_dn5) + (var_t0_dn5 * var_beta)) * assign8370_e8120) + (assign8370_e8113 * ((2.0 * var_t1_dn5) + (var_t0_dn5 * var_beta)))) - (4.0 * (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) + var_t0_dn5))), (((((2.0 * var_t1_dn6) + (var_t0_dn6 * var_beta)) * assign8370_e8120) + (assign8370_e8113 * ((2.0 * var_t1_dn6) + (var_t0_dn6 * var_beta)))) - (4.0 * (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) + var_t0_dn6))), (((((2.0 * var_t1_dn8) + (var_t0_dn8 * var_beta)) * assign8370_e8120) + (assign8370_e8113 * ((2.0 * var_t1_dn8) + (var_t0_dn8 * var_beta)))) - (4.0 * (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) + var_t0_dn8))), (((((2.0 * var_t1_dn10) + (var_t0_dn10 * var_beta)) * assign8370_e8120) + (assign8370_e8113 * ((2.0 * var_t1_dn10) + (var_t0_dn10 * var_beta)))) - (4.0 * (((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) + var_t0_dn10))), (((((2.0 * var_t1_dn11) + (var_t0_dn11 * var_beta)) * assign8370_e8120) + (assign8370_e8113 * ((2.0 * var_t1_dn11) + (var_t0_dn11 * var_beta)))) - (4.0 * (((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) + var_t0_dn11))), (((((2.0 * var_t1_dn12) + (var_t0_dn12 * var_beta)) * assign8370_e8120) + (assign8370_e8113 * ((2.0 * var_t1_dn12) + (var_t0_dn12 * var_beta)))) - (4.0 * (((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)) + var_t0_dn12))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign8370_e8131;
        var_t2_dn0 = assign8370_e8131_d_n0;
        var_t2_dn2 = assign8370_e8131_d_n2;
        var_t2_dn4 = assign8370_e8131_d_n4;
        var_t2_dn5 = assign8370_e8131_d_n5;
        var_t2_dn6 = assign8370_e8131_d_n6;
        var_t2_dn8 = assign8370_e8131_d_n8;
        var_t2_dn10 = assign8370_e8131_d_n10;
        var_t2_dn11 = assign8370_e8131_d_n11;
        var_t2_dn12 = assign8370_e8131_d_n12;
        var_t2_rv = 0.0;

        let (assign8380_e8148, assign8380_e8148_d_n0, assign8380_e8148_d_n2, assign8380_e8148_d_n4, assign8380_e8148_d_n5, assign8380_e8148_d_n6, assign8380_e8148_d_n8, assign8380_e8148_d_n10, assign8380_e8148_d_n11, assign8380_e8148_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard101 == 0.0)) {
        let assign8380_e8140: f64 = (10.0 * 2.220446049250313e-16);
        let (assign8380_e8146, assign8380_e8146_d_n0, assign8380_e8146_d_n2, assign8380_e8146_d_n4, assign8380_e8146_d_n5, assign8380_e8146_d_n6, assign8380_e8146_d_n8, assign8380_e8146_d_n10, assign8380_e8146_d_n11, assign8380_e8146_d_n12,) = {
            if (var_t2 >= assign8380_e8140) {
                (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
            } else {
                let assign8380_e8145: f64 = (10.0 * 2.220446049250313e-16);
                (assign8380_e8145, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign8380_e8146, assign8380_e8146_d_n0, assign8380_e8146_d_n2, assign8380_e8146_d_n4, assign8380_e8146_d_n5, assign8380_e8146_d_n6, assign8380_e8146_d_n8, assign8380_e8146_d_n10, assign8380_e8146_d_n11, assign8380_e8146_d_n12,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign8380_e8148;
        var_t2_dn0 = assign8380_e8148_d_n0;
        var_t2_dn2 = assign8380_e8148_d_n2;
        var_t2_dn4 = assign8380_e8148_d_n4;
        var_t2_dn5 = assign8380_e8148_d_n5;
        var_t2_dn6 = assign8380_e8148_d_n6;
        var_t2_dn8 = assign8380_e8148_d_n8;
        var_t2_dn10 = assign8380_e8148_d_n10;
        var_t2_dn11 = assign8380_e8148_d_n11;
        var_t2_dn12 = assign8380_e8148_d_n12;
        var_t2_rv = 0.0;

        let (assign8390_e8157, assign8390_e8157_d_n0, assign8390_e8157_d_n2, assign8390_e8157_d_n4, assign8390_e8157_d_n5, assign8390_e8157_d_n6, assign8390_e8157_d_n8, assign8390_e8157_d_n10, assign8390_e8157_d_n11, assign8390_e8157_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard101 == 0.0)) {
        let assign8390_e8155: f64 = (var_t2).sqrt();
        (assign8390_e8155, (var_t2_dn0 / (2.0 * assign8390_e8155)), (var_t2_dn2 / (2.0 * assign8390_e8155)), (var_t2_dn4 / (2.0 * assign8390_e8155)), (var_t2_dn5 / (2.0 * assign8390_e8155)), (var_t2_dn6 / (2.0 * assign8390_e8155)), (var_t2_dn8 / (2.0 * assign8390_e8155)), (var_t2_dn10 / (2.0 * assign8390_e8155)), (var_t2_dn11 / (2.0 * assign8390_e8155)), (var_t2_dn12 / (2.0 * assign8390_e8155)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign8390_e8157;
        var_t2_dn0 = assign8390_e8157_d_n0;
        var_t2_dn2 = assign8390_e8157_d_n2;
        var_t2_dn4 = assign8390_e8157_d_n4;
        var_t2_dn5 = assign8390_e8157_d_n5;
        var_t2_dn6 = assign8390_e8157_d_n6;
        var_t2_dn8 = assign8390_e8157_d_n8;
        var_t2_dn10 = assign8390_e8157_d_n10;
        var_t2_dn11 = assign8390_e8157_d_n11;
        var_t2_dn12 = assign8390_e8157_d_n12;
        var_t2_rv = 0.0;

        *var_flg_depmode_slot = var_flg_depmode;
        *var_flg_depmode_rv_slot = var_flg_depmode_rv;
        *var_guard101_slot = var_guard101;
        *var_guard101_rv_slot = var_guard101_rv;
        *var_guard102_slot = var_guard102;
        *var_guard102_rv_slot = var_guard102_rv;
        *var_phi_s0_bulk_slot = var_phi_s0_bulk;
        *var_phi_s0_bulk_dn0_slot = var_phi_s0_bulk_dn0;
        *var_phi_s0_bulk_dn10_slot = var_phi_s0_bulk_dn10;
        *var_phi_s0_bulk_dn11_slot = var_phi_s0_bulk_dn11;
        *var_phi_s0_bulk_dn12_slot = var_phi_s0_bulk_dn12;
        *var_phi_s0_bulk_dn2_slot = var_phi_s0_bulk_dn2;
        *var_phi_s0_bulk_dn4_slot = var_phi_s0_bulk_dn4;
        *var_phi_s0_bulk_dn5_slot = var_phi_s0_bulk_dn5;
        *var_phi_s0_bulk_dn6_slot = var_phi_s0_bulk_dn6;
        *var_phi_s0_bulk_dn8_slot = var_phi_s0_bulk_dn8;
        *var_phi_s0_bulk_rv_slot = var_phi_s0_bulk_rv;
        *var_phi_s0_soi_slot = var_phi_s0_soi;
        *var_phi_s0_soi_dn0_slot = var_phi_s0_soi_dn0;
        *var_phi_s0_soi_dn10_slot = var_phi_s0_soi_dn10;
        *var_phi_s0_soi_dn11_slot = var_phi_s0_soi_dn11;
        *var_phi_s0_soi_dn12_slot = var_phi_s0_soi_dn12;
        *var_phi_s0_soi_dn2_slot = var_phi_s0_soi_dn2;
        *var_phi_s0_soi_dn4_slot = var_phi_s0_soi_dn4;
        *var_phi_s0_soi_dn5_slot = var_phi_s0_soi_dn5;
        *var_phi_s0_soi_dn6_slot = var_phi_s0_soi_dn6;
        *var_phi_s0_soi_dn8_slot = var_phi_s0_soi_dn8;
        *var_phi_s0_soi_rv_slot = var_phi_s0_soi_rv;
        *var_psb_inia_slot = var_psb_inia;
        *var_psb_inia_dn0_slot = var_psb_inia_dn0;
        *var_psb_inia_dn10_slot = var_psb_inia_dn10;
        *var_psb_inia_dn11_slot = var_psb_inia_dn11;
        *var_psb_inia_dn12_slot = var_psb_inia_dn12;
        *var_psb_inia_dn2_slot = var_psb_inia_dn2;
        *var_psb_inia_dn4_slot = var_psb_inia_dn4;
        *var_psb_inia_dn5_slot = var_psb_inia_dn5;
        *var_psb_inia_dn6_slot = var_psb_inia_dn6;
        *var_psb_inia_dn8_slot = var_psb_inia_dn8;
        *var_psb_inia_rv_slot = var_psb_inia_rv;
        *var_psb_inib_slot = var_psb_inib;
        *var_psb_inib_dn0_slot = var_psb_inib_dn0;
        *var_psb_inib_dn10_slot = var_psb_inib_dn10;
        *var_psb_inib_dn11_slot = var_psb_inib_dn11;
        *var_psb_inib_dn12_slot = var_psb_inib_dn12;
        *var_psb_inib_dn2_slot = var_psb_inib_dn2;
        *var_psb_inib_dn4_slot = var_psb_inib_dn4;
        *var_psb_inib_dn5_slot = var_psb_inib_dn5;
        *var_psb_inib_dn6_slot = var_psb_inib_dn6;
        *var_psb_inib_dn8_slot = var_psb_inib_dn8;
        *var_psb_inib_rv_slot = var_psb_inib_rv;
        *var_psl_lim_slot = var_psl_lim;
        *var_psl_lim_dn0_slot = var_psl_lim_dn0;
        *var_psl_lim_dn10_slot = var_psl_lim_dn10;
        *var_psl_lim_dn11_slot = var_psl_lim_dn11;
        *var_psl_lim_dn12_slot = var_psl_lim_dn12;
        *var_psl_lim_dn2_slot = var_psl_lim_dn2;
        *var_psl_lim_dn4_slot = var_psl_lim_dn4;
        *var_psl_lim_dn5_slot = var_psl_lim_dn5;
        *var_psl_lim_dn6_slot = var_psl_lim_dn6;
        *var_psl_lim_dn8_slot = var_psl_lim_dn8;
        *var_psl_lim_rv_slot = var_psl_lim_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_30(
        var_beta: f64,
        var_beta_dn4: f64,
        var_cnst1bulk: f64,
        var_cnst1bulk_dn0: f64,
        var_cnst1bulk_dn10: f64,
        var_cnst1bulk_dn11: f64,
        var_cnst1bulk_dn12: f64,
        var_cnst1bulk_dn2: f64,
        var_cnst1bulk_dn4: f64,
        var_cnst1bulk_dn5: f64,
        var_cnst1bulk_dn6: f64,
        var_cnst1bulk_dn8: f64,
        var_guard101: f64,
        var_guard74: f64,
        var_pb2_bulk: f64,
        var_t0: f64,
        var_t0_dn0: f64,
        var_t0_dn10: f64,
        var_t0_dn11: f64,
        var_t0_dn12: f64,
        var_t0_dn2: f64,
        var_t0_dn4: f64,
        var_t0_dn5: f64,
        var_t0_dn6: f64,
        var_t0_dn8: f64,
        var_t2: f64,
        var_t2_dn0: f64,
        var_t2_dn10: f64,
        var_t2_dn11: f64,
        var_t2_dn12: f64,
        var_t2_dn2: f64,
        var_t2_dn4: f64,
        var_t2_dn5: f64,
        var_t2_dn6: f64,
        var_t2_dn8: f64,
        var_flg_depmode_slot: &mut f64,
        var_flg_depmode_rv_slot: &mut f64,
        var_guard103_slot: &mut f64,
        var_guard103_rv_slot: &mut f64,
        var_guard104_slot: &mut f64,
        var_guard104_rv_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
        var_lp_s0_rv_slot: &mut f64,
        var_phi_s0_bulk_slot: &mut f64,
        var_phi_s0_bulk_dn0_slot: &mut f64,
        var_phi_s0_bulk_dn10_slot: &mut f64,
        var_phi_s0_bulk_dn11_slot: &mut f64,
        var_phi_s0_bulk_dn12_slot: &mut f64,
        var_phi_s0_bulk_dn2_slot: &mut f64,
        var_phi_s0_bulk_dn4_slot: &mut f64,
        var_phi_s0_bulk_dn5_slot: &mut f64,
        var_phi_s0_bulk_dn6_slot: &mut f64,
        var_phi_s0_bulk_dn8_slot: &mut f64,
        var_phi_s0_bulk_rv_slot: &mut f64,
        var_psb_inia_slot: &mut f64,
        var_psb_inia_dn0_slot: &mut f64,
        var_psb_inia_dn10_slot: &mut f64,
        var_psb_inia_dn11_slot: &mut f64,
        var_psb_inia_dn12_slot: &mut f64,
        var_psb_inia_dn2_slot: &mut f64,
        var_psb_inia_dn4_slot: &mut f64,
        var_psb_inia_dn5_slot: &mut f64,
        var_psb_inia_dn6_slot: &mut f64,
        var_psb_inia_dn8_slot: &mut f64,
        var_psb_inia_rv_slot: &mut f64,
        var_psb_inib_slot: &mut f64,
        var_psb_inib_dn0_slot: &mut f64,
        var_psb_inib_dn10_slot: &mut f64,
        var_psb_inib_dn11_slot: &mut f64,
        var_psb_inib_dn12_slot: &mut f64,
        var_psb_inib_dn2_slot: &mut f64,
        var_psb_inib_dn4_slot: &mut f64,
        var_psb_inib_dn5_slot: &mut f64,
        var_psb_inib_dn6_slot: &mut f64,
        var_psb_inib_dn8_slot: &mut f64,
        var_psb_inib_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_flg_depmode: f64 = *var_flg_depmode_slot;
        let mut var_flg_depmode_rv: f64 = *var_flg_depmode_rv_slot;
        let mut var_guard103: f64 = *var_guard103_slot;
        let mut var_guard103_rv: f64 = *var_guard103_rv_slot;
        let mut var_guard104: f64 = *var_guard104_slot;
        let mut var_guard104_rv: f64 = *var_guard104_rv_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
        let mut var_lp_s0_rv: f64 = *var_lp_s0_rv_slot;
        let mut var_phi_s0_bulk: f64 = *var_phi_s0_bulk_slot;
        let mut var_phi_s0_bulk_dn0: f64 = *var_phi_s0_bulk_dn0_slot;
        let mut var_phi_s0_bulk_dn10: f64 = *var_phi_s0_bulk_dn10_slot;
        let mut var_phi_s0_bulk_dn11: f64 = *var_phi_s0_bulk_dn11_slot;
        let mut var_phi_s0_bulk_dn12: f64 = *var_phi_s0_bulk_dn12_slot;
        let mut var_phi_s0_bulk_dn2: f64 = *var_phi_s0_bulk_dn2_slot;
        let mut var_phi_s0_bulk_dn4: f64 = *var_phi_s0_bulk_dn4_slot;
        let mut var_phi_s0_bulk_dn5: f64 = *var_phi_s0_bulk_dn5_slot;
        let mut var_phi_s0_bulk_dn6: f64 = *var_phi_s0_bulk_dn6_slot;
        let mut var_phi_s0_bulk_dn8: f64 = *var_phi_s0_bulk_dn8_slot;
        let mut var_phi_s0_bulk_rv: f64 = *var_phi_s0_bulk_rv_slot;
        let mut var_psb_inia: f64 = *var_psb_inia_slot;
        let mut var_psb_inia_dn0: f64 = *var_psb_inia_dn0_slot;
        let mut var_psb_inia_dn10: f64 = *var_psb_inia_dn10_slot;
        let mut var_psb_inia_dn11: f64 = *var_psb_inia_dn11_slot;
        let mut var_psb_inia_dn12: f64 = *var_psb_inia_dn12_slot;
        let mut var_psb_inia_dn2: f64 = *var_psb_inia_dn2_slot;
        let mut var_psb_inia_dn4: f64 = *var_psb_inia_dn4_slot;
        let mut var_psb_inia_dn5: f64 = *var_psb_inia_dn5_slot;
        let mut var_psb_inia_dn6: f64 = *var_psb_inia_dn6_slot;
        let mut var_psb_inia_dn8: f64 = *var_psb_inia_dn8_slot;
        let mut var_psb_inia_rv: f64 = *var_psb_inia_rv_slot;
        let mut var_psb_inib: f64 = *var_psb_inib_slot;
        let mut var_psb_inib_dn0: f64 = *var_psb_inib_dn0_slot;
        let mut var_psb_inib_dn10: f64 = *var_psb_inib_dn10_slot;
        let mut var_psb_inib_dn11: f64 = *var_psb_inib_dn11_slot;
        let mut var_psb_inib_dn12: f64 = *var_psb_inib_dn12_slot;
        let mut var_psb_inib_dn2: f64 = *var_psb_inib_dn2_slot;
        let mut var_psb_inib_dn4: f64 = *var_psb_inib_dn4_slot;
        let mut var_psb_inib_dn5: f64 = *var_psb_inib_dn5_slot;
        let mut var_psb_inib_dn6: f64 = *var_psb_inib_dn6_slot;
        let mut var_psb_inib_dn8: f64 = *var_psb_inib_dn8_slot;
        let mut var_psb_inib_rv: f64 = *var_psb_inib_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign8400_e8171, assign8400_e8171_d_n0, assign8400_e8171_d_n2, assign8400_e8171_d_n4, assign8400_e8171_d_n5, assign8400_e8171_d_n6, assign8400_e8171_d_n8, assign8400_e8171_d_n10, assign8400_e8171_d_n11, assign8400_e8171_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard101 == 0.0)) {
        let assign8400_e8165: f64 = (2.0 * var_t1);
        let assign8400_e8168: f64 = (var_t0 * var_beta);
        let assign8400_e8169: f64 = (assign8400_e8165 + assign8400_e8168);
        (assign8400_e8169, ((2.0 * var_t1_dn0) + (var_t0_dn0 * var_beta)), ((2.0 * var_t1_dn2) + (var_t0_dn2 * var_beta)), ((2.0 * var_t1_dn4) + ((var_t0_dn4 * var_beta) + (var_t0 * var_beta_dn4))), ((2.0 * var_t1_dn5) + (var_t0_dn5 * var_beta)), ((2.0 * var_t1_dn6) + (var_t0_dn6 * var_beta)), ((2.0 * var_t1_dn8) + (var_t0_dn8 * var_beta)), ((2.0 * var_t1_dn10) + (var_t0_dn10 * var_beta)), ((2.0 * var_t1_dn11) + (var_t0_dn11 * var_beta)), ((2.0 * var_t1_dn12) + (var_t0_dn12 * var_beta)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign8400_e8171;
        var_t3_dn0 = assign8400_e8171_d_n0;
        var_t3_dn2 = assign8400_e8171_d_n2;
        var_t3_dn4 = assign8400_e8171_d_n4;
        var_t3_dn5 = assign8400_e8171_d_n5;
        var_t3_dn6 = assign8400_e8171_d_n6;
        var_t3_dn8 = assign8400_e8171_d_n8;
        var_t3_dn10 = assign8400_e8171_d_n10;
        var_t3_dn11 = assign8400_e8171_d_n11;
        var_t3_dn12 = assign8400_e8171_d_n12;
        var_t3_rv = 0.0;

        let (assign8410_e8183, assign8410_e8183_d_n0, assign8410_e8183_d_n2, assign8410_e8183_d_n4, assign8410_e8183_d_n5, assign8410_e8183_d_n6, assign8410_e8183_d_n8, assign8410_e8183_d_n10, assign8410_e8183_d_n11, assign8410_e8183_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard101 == 0.0)) {
        let assign8410_e8179: f64 = (var_t3 - var_t2);
        let assign8410_e8181: f64 = (assign8410_e8179 / 2.0);
        (assign8410_e8181, ((var_t3_dn0 - var_t2_dn0) / 2.0), ((var_t3_dn2 - var_t2_dn2) / 2.0), ((var_t3_dn4 - var_t2_dn4) / 2.0), ((var_t3_dn5 - var_t2_dn5) / 2.0), ((var_t3_dn6 - var_t2_dn6) / 2.0), ((var_t3_dn8 - var_t2_dn8) / 2.0), ((var_t3_dn10 - var_t2_dn10) / 2.0), ((var_t3_dn11 - var_t2_dn11) / 2.0), ((var_t3_dn12 - var_t2_dn12) / 2.0),)
    } else {
        (var_psb_inia, var_psb_inia_dn0, var_psb_inia_dn2, var_psb_inia_dn4, var_psb_inia_dn5, var_psb_inia_dn6, var_psb_inia_dn8, var_psb_inia_dn10, var_psb_inia_dn11, var_psb_inia_dn12,)
    }
};
        var_psb_inia = assign8410_e8183;
        var_psb_inia_dn0 = assign8410_e8183_d_n0;
        var_psb_inia_dn2 = assign8410_e8183_d_n2;
        var_psb_inia_dn4 = assign8410_e8183_d_n4;
        var_psb_inia_dn5 = assign8410_e8183_d_n5;
        var_psb_inia_dn6 = assign8410_e8183_d_n6;
        var_psb_inia_dn8 = assign8410_e8183_d_n8;
        var_psb_inia_dn10 = assign8410_e8183_d_n10;
        var_psb_inia_dn11 = assign8410_e8183_d_n11;
        var_psb_inia_dn12 = assign8410_e8183_d_n12;
        var_psb_inia_rv = 0.0;

        let (assign8420_e8204, assign8420_e8204_d_n0, assign8420_e8204_d_n2, assign8420_e8204_d_n4, assign8420_e8204_d_n5, assign8420_e8204_d_n6, assign8420_e8204_d_n8, assign8420_e8204_d_n10, assign8420_e8204_d_n11, assign8420_e8204_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard101 == 0.0)) {
        let assign8420_e8191: f64 = (var_t1 * var_t1);
        let assign8420_e8193: f64 = (assign8420_e8191 / var_t0);
        let assign8420_e8195: f64 = (assign8420_e8193 / var_cnst1bulk);
        let assign8420_e8196: f64 = (assign8420_e8195).ln();
        let assign8420_e8200: f64 = (2.0 / var_t1);
        let assign8420_e8201: f64 = (var_beta + assign8420_e8200);
        let assign8420_e8202: f64 = (assign8420_e8196 / assign8420_e8201);
        (assign8420_e8202, ((((((((((((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) * var_t0) - (assign8420_e8191 * var_t0_dn0)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8420_e8193 * var_cnst1bulk_dn0)) / (var_cnst1bulk * var_cnst1bulk)) / assign8420_e8195) * assign8420_e8201) - (assign8420_e8196 * (-((2.0 * var_t1_dn0) / (var_t1 * var_t1))))) / (assign8420_e8201 * assign8420_e8201)), ((((((((((((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) * var_t0) - (assign8420_e8191 * var_t0_dn2)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8420_e8193 * var_cnst1bulk_dn2)) / (var_cnst1bulk * var_cnst1bulk)) / assign8420_e8195) * assign8420_e8201) - (assign8420_e8196 * (-((2.0 * var_t1_dn2) / (var_t1 * var_t1))))) / (assign8420_e8201 * assign8420_e8201)), ((((((((((((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) * var_t0) - (assign8420_e8191 * var_t0_dn4)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8420_e8193 * var_cnst1bulk_dn4)) / (var_cnst1bulk * var_cnst1bulk)) / assign8420_e8195) * assign8420_e8201) - (assign8420_e8196 * (var_beta_dn4 + (-((2.0 * var_t1_dn4) / (var_t1 * var_t1)))))) / (assign8420_e8201 * assign8420_e8201)), ((((((((((((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) * var_t0) - (assign8420_e8191 * var_t0_dn5)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8420_e8193 * var_cnst1bulk_dn5)) / (var_cnst1bulk * var_cnst1bulk)) / assign8420_e8195) * assign8420_e8201) - (assign8420_e8196 * (-((2.0 * var_t1_dn5) / (var_t1 * var_t1))))) / (assign8420_e8201 * assign8420_e8201)), ((((((((((((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) * var_t0) - (assign8420_e8191 * var_t0_dn6)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8420_e8193 * var_cnst1bulk_dn6)) / (var_cnst1bulk * var_cnst1bulk)) / assign8420_e8195) * assign8420_e8201) - (assign8420_e8196 * (-((2.0 * var_t1_dn6) / (var_t1 * var_t1))))) / (assign8420_e8201 * assign8420_e8201)), ((((((((((((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) * var_t0) - (assign8420_e8191 * var_t0_dn8)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8420_e8193 * var_cnst1bulk_dn8)) / (var_cnst1bulk * var_cnst1bulk)) / assign8420_e8195) * assign8420_e8201) - (assign8420_e8196 * (-((2.0 * var_t1_dn8) / (var_t1 * var_t1))))) / (assign8420_e8201 * assign8420_e8201)), ((((((((((((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) * var_t0) - (assign8420_e8191 * var_t0_dn10)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8420_e8193 * var_cnst1bulk_dn10)) / (var_cnst1bulk * var_cnst1bulk)) / assign8420_e8195) * assign8420_e8201) - (assign8420_e8196 * (-((2.0 * var_t1_dn10) / (var_t1 * var_t1))))) / (assign8420_e8201 * assign8420_e8201)), ((((((((((((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) * var_t0) - (assign8420_e8191 * var_t0_dn11)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8420_e8193 * var_cnst1bulk_dn11)) / (var_cnst1bulk * var_cnst1bulk)) / assign8420_e8195) * assign8420_e8201) - (assign8420_e8196 * (-((2.0 * var_t1_dn11) / (var_t1 * var_t1))))) / (assign8420_e8201 * assign8420_e8201)), ((((((((((((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)) * var_t0) - (assign8420_e8191 * var_t0_dn12)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign8420_e8193 * var_cnst1bulk_dn12)) / (var_cnst1bulk * var_cnst1bulk)) / assign8420_e8195) * assign8420_e8201) - (assign8420_e8196 * (-((2.0 * var_t1_dn12) / (var_t1 * var_t1))))) / (assign8420_e8201 * assign8420_e8201)),)
    } else {
        (var_psb_inib, var_psb_inib_dn0, var_psb_inib_dn2, var_psb_inib_dn4, var_psb_inib_dn5, var_psb_inib_dn6, var_psb_inib_dn8, var_psb_inib_dn10, var_psb_inib_dn11, var_psb_inib_dn12,)
    }
};
        var_psb_inib = assign8420_e8204;
        var_psb_inib_dn0 = assign8420_e8204_d_n0;
        var_psb_inib_dn2 = assign8420_e8204_d_n2;
        var_psb_inib_dn4 = assign8420_e8204_d_n4;
        var_psb_inib_dn5 = assign8420_e8204_d_n5;
        var_psb_inib_dn6 = assign8420_e8204_d_n6;
        var_psb_inib_dn8 = assign8420_e8204_d_n8;
        var_psb_inib_dn10 = assign8420_e8204_d_n10;
        var_psb_inib_dn11 = assign8420_e8204_d_n11;
        var_psb_inib_dn12 = assign8420_e8204_d_n12;
        var_psb_inib_rv = 0.0;

        let assign8430_e8207: f64 = if var_psb_inia < var_pb2_bulk { 1.0 } else { 0.0 };
        var_guard103 = assign8430_e8207;
        var_guard103_rv = 0.0;

        let (assign8440_e8217, assign8440_e8217_d_n0, assign8440_e8217_d_n2, assign8440_e8217_d_n4, assign8440_e8217_d_n5, assign8440_e8217_d_n6, assign8440_e8217_d_n8, assign8440_e8217_d_n10, assign8440_e8217_d_n11, assign8440_e8217_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard101 == 0.0)) && (var_guard103 != 0.0)) {
        (var_psb_inia, var_psb_inia_dn0, var_psb_inia_dn2, var_psb_inia_dn4, var_psb_inia_dn5, var_psb_inia_dn6, var_psb_inia_dn8, var_psb_inia_dn10, var_psb_inia_dn11, var_psb_inia_dn12,)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
        var_phi_s0_bulk = assign8440_e8217;
        var_phi_s0_bulk_dn0 = assign8440_e8217_d_n0;
        var_phi_s0_bulk_dn2 = assign8440_e8217_d_n2;
        var_phi_s0_bulk_dn4 = assign8440_e8217_d_n4;
        var_phi_s0_bulk_dn5 = assign8440_e8217_d_n5;
        var_phi_s0_bulk_dn6 = assign8440_e8217_d_n6;
        var_phi_s0_bulk_dn8 = assign8440_e8217_d_n8;
        var_phi_s0_bulk_dn10 = assign8440_e8217_d_n10;
        var_phi_s0_bulk_dn11 = assign8440_e8217_d_n11;
        var_phi_s0_bulk_dn12 = assign8440_e8217_d_n12;
        var_phi_s0_bulk_rv = 0.0;

        let (assign8450_e8232, assign8450_e8232_d_n0, assign8450_e8232_d_n2, assign8450_e8232_d_n4, assign8450_e8232_d_n5, assign8450_e8232_d_n6, assign8450_e8232_d_n8, assign8450_e8232_d_n10, assign8450_e8232_d_n11, assign8450_e8232_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard101 == 0.0)) && (var_guard103 == 0.0)) {
        let assign8450_e8228: f64 = (var_psb_inib - var_psb_inia);
        let assign8450_e8230: f64 = (assign8450_e8228 - 0.0008);
        (assign8450_e8230, (var_psb_inib_dn0 - var_psb_inia_dn0), (var_psb_inib_dn2 - var_psb_inia_dn2), (var_psb_inib_dn4 - var_psb_inia_dn4), (var_psb_inib_dn5 - var_psb_inia_dn5), (var_psb_inib_dn6 - var_psb_inia_dn6), (var_psb_inib_dn8 - var_psb_inia_dn8), (var_psb_inib_dn10 - var_psb_inia_dn10), (var_psb_inib_dn11 - var_psb_inia_dn11), (var_psb_inib_dn12 - var_psb_inia_dn12),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn8, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12,)
    }
};
        var_tmf1 = assign8450_e8232;
        var_tmf1_dn0 = assign8450_e8232_d_n0;
        var_tmf1_dn2 = assign8450_e8232_d_n2;
        var_tmf1_dn4 = assign8450_e8232_d_n4;
        var_tmf1_dn5 = assign8450_e8232_d_n5;
        var_tmf1_dn6 = assign8450_e8232_d_n6;
        var_tmf1_dn8 = assign8450_e8232_d_n8;
        var_tmf1_dn10 = assign8450_e8232_d_n10;
        var_tmf1_dn11 = assign8450_e8232_d_n11;
        var_tmf1_dn12 = assign8450_e8232_d_n12;
        var_tmf1_rv = 0.0;

        let (assign8460_e8247, assign8460_e8247_d_n0, assign8460_e8247_d_n2, assign8460_e8247_d_n4, assign8460_e8247_d_n5, assign8460_e8247_d_n6, assign8460_e8247_d_n8, assign8460_e8247_d_n10, assign8460_e8247_d_n11, assign8460_e8247_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard101 == 0.0)) && (var_guard103 == 0.0)) {
        let assign8460_e8243: f64 = (4.0 * var_psb_inib);
        let assign8460_e8245: f64 = (assign8460_e8243 * 0.0008);
        (assign8460_e8245, ((4.0 * var_psb_inib_dn0) * 0.0008), ((4.0 * var_psb_inib_dn2) * 0.0008), ((4.0 * var_psb_inib_dn4) * 0.0008), ((4.0 * var_psb_inib_dn5) * 0.0008), ((4.0 * var_psb_inib_dn6) * 0.0008), ((4.0 * var_psb_inib_dn8) * 0.0008), ((4.0 * var_psb_inib_dn10) * 0.0008), ((4.0 * var_psb_inib_dn11) * 0.0008), ((4.0 * var_psb_inib_dn12) * 0.0008),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
        var_tmf2 = assign8460_e8247;
        var_tmf2_dn0 = assign8460_e8247_d_n0;
        var_tmf2_dn2 = assign8460_e8247_d_n2;
        var_tmf2_dn4 = assign8460_e8247_d_n4;
        var_tmf2_dn5 = assign8460_e8247_d_n5;
        var_tmf2_dn6 = assign8460_e8247_d_n6;
        var_tmf2_dn8 = assign8460_e8247_d_n8;
        var_tmf2_dn10 = assign8460_e8247_d_n10;
        var_tmf2_dn11 = assign8460_e8247_d_n11;
        var_tmf2_dn12 = assign8460_e8247_d_n12;
        var_tmf2_rv = 0.0;

        let (assign8470_e8264, assign8470_e8264_d_n0, assign8470_e8264_d_n2, assign8470_e8264_d_n4, assign8470_e8264_d_n5, assign8470_e8264_d_n6, assign8470_e8264_d_n8, assign8470_e8264_d_n10, assign8470_e8264_d_n11, assign8470_e8264_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard101 == 0.0)) && (var_guard103 == 0.0)) {
        let (assign8470_e8262, assign8470_e8262_d_n0, assign8470_e8262_d_n2, assign8470_e8262_d_n4, assign8470_e8262_d_n5, assign8470_e8262_d_n6, assign8470_e8262_d_n8, assign8470_e8262_d_n10, assign8470_e8262_d_n11, assign8470_e8262_d_n12,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
            } else {
                let assign8470_e8261: f64 = (-var_tmf2);
                (assign8470_e8261, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn8), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12),)
            }
        };
        (assign8470_e8262, assign8470_e8262_d_n0, assign8470_e8262_d_n2, assign8470_e8262_d_n4, assign8470_e8262_d_n5, assign8470_e8262_d_n6, assign8470_e8262_d_n8, assign8470_e8262_d_n10, assign8470_e8262_d_n11, assign8470_e8262_d_n12,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
        var_tmf2 = assign8470_e8264;
        var_tmf2_dn0 = assign8470_e8264_d_n0;
        var_tmf2_dn2 = assign8470_e8264_d_n2;
        var_tmf2_dn4 = assign8470_e8264_d_n4;
        var_tmf2_dn5 = assign8470_e8264_d_n5;
        var_tmf2_dn6 = assign8470_e8264_d_n6;
        var_tmf2_dn8 = assign8470_e8264_d_n8;
        var_tmf2_dn10 = assign8470_e8264_d_n10;
        var_tmf2_dn11 = assign8470_e8264_d_n11;
        var_tmf2_dn12 = assign8470_e8264_d_n12;
        var_tmf2_rv = 0.0;

        let (assign8480_e8280, assign8480_e8280_d_n0, assign8480_e8280_d_n2, assign8480_e8280_d_n4, assign8480_e8280_d_n5, assign8480_e8280_d_n6, assign8480_e8280_d_n8, assign8480_e8280_d_n10, assign8480_e8280_d_n11, assign8480_e8280_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard101 == 0.0)) && (var_guard103 == 0.0)) {
        let assign8480_e8275: f64 = (var_tmf1 * var_tmf1);
        let assign8480_e8277: f64 = (assign8480_e8275 + var_tmf2);
        let assign8480_e8278: f64 = (assign8480_e8277).sqrt();
        (assign8480_e8278, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign8480_e8278)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign8480_e8278)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign8480_e8278)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign8480_e8278)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign8480_e8278)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign8480_e8278)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign8480_e8278)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign8480_e8278)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign8480_e8278)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
        var_tmf2 = assign8480_e8280;
        var_tmf2_dn0 = assign8480_e8280_d_n0;
        var_tmf2_dn2 = assign8480_e8280_d_n2;
        var_tmf2_dn4 = assign8480_e8280_d_n4;
        var_tmf2_dn5 = assign8480_e8280_d_n5;
        var_tmf2_dn6 = assign8480_e8280_d_n6;
        var_tmf2_dn8 = assign8480_e8280_d_n8;
        var_tmf2_dn10 = assign8480_e8280_d_n10;
        var_tmf2_dn11 = assign8480_e8280_d_n11;
        var_tmf2_dn12 = assign8480_e8280_d_n12;
        var_tmf2_rv = 0.0;

        let (assign8490_e8297, assign8490_e8297_d_n0, assign8490_e8297_d_n2, assign8490_e8297_d_n4, assign8490_e8297_d_n5, assign8490_e8297_d_n6, assign8490_e8297_d_n8, assign8490_e8297_d_n10, assign8490_e8297_d_n11, assign8490_e8297_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard101 == 0.0)) && (var_guard103 == 0.0)) {
        let assign8490_e8293: f64 = (var_tmf1 / var_tmf2);
        let assign8490_e8294: f64 = (1.0 + assign8490_e8293);
        let assign8490_e8295: f64 = (0.5 * assign8490_e8294);
        (assign8490_e8295, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn12 * var_tmf2) - (var_tmf1 * var_tmf2_dn12)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign8490_e8297;
        var_t1_dn0 = assign8490_e8297_d_n0;
        var_t1_dn2 = assign8490_e8297_d_n2;
        var_t1_dn4 = assign8490_e8297_d_n4;
        var_t1_dn5 = assign8490_e8297_d_n5;
        var_t1_dn6 = assign8490_e8297_d_n6;
        var_t1_dn8 = assign8490_e8297_d_n8;
        var_t1_dn10 = assign8490_e8297_d_n10;
        var_t1_dn11 = assign8490_e8297_d_n11;
        var_t1_dn12 = assign8490_e8297_d_n12;
        var_t1_rv = 0.0;

        let (assign8500_e8314, assign8500_e8314_d_n0, assign8500_e8314_d_n2, assign8500_e8314_d_n4, assign8500_e8314_d_n5, assign8500_e8314_d_n6, assign8500_e8314_d_n8, assign8500_e8314_d_n10, assign8500_e8314_d_n11, assign8500_e8314_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard101 == 0.0)) && (var_guard103 == 0.0)) {
        let assign8500_e8310: f64 = (var_tmf1 + var_tmf2);
        let assign8500_e8311: f64 = (0.5 * assign8500_e8310);
        let assign8500_e8312: f64 = (var_psb_inib - assign8500_e8311);
        (assign8500_e8312, (var_psb_inib_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_psb_inib_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_psb_inib_dn4 - (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_psb_inib_dn5 - (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_psb_inib_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_psb_inib_dn8 - (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), (var_psb_inib_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_psb_inib_dn11 - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (var_psb_inib_dn12 - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))),)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
        var_phi_s0_bulk = assign8500_e8314;
        var_phi_s0_bulk_dn0 = assign8500_e8314_d_n0;
        var_phi_s0_bulk_dn2 = assign8500_e8314_d_n2;
        var_phi_s0_bulk_dn4 = assign8500_e8314_d_n4;
        var_phi_s0_bulk_dn5 = assign8500_e8314_d_n5;
        var_phi_s0_bulk_dn6 = assign8500_e8314_d_n6;
        var_phi_s0_bulk_dn8 = assign8500_e8314_d_n8;
        var_phi_s0_bulk_dn10 = assign8500_e8314_d_n10;
        var_phi_s0_bulk_dn11 = assign8500_e8314_d_n11;
        var_phi_s0_bulk_dn12 = assign8500_e8314_d_n12;
        var_phi_s0_bulk_rv = 0.0;

        let assign8510_e8319: f64 = if ((var_flg_depmode == 1.0) && (0.0 != 0.0)) { 1.0 } else { 0.0 };
        var_guard104 = assign8510_e8319;
        var_guard104_rv = 0.0;

        let (assign8520_e8326,) = {
    if ((var_guard74 == 0.0) && (var_guard104 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_depmode,)
    }
};
        var_flg_depmode = assign8520_e8326;
        var_flg_depmode_rv = 0.0;

        let (assign8530_e8333,) = {
    if ((var_guard74 == 0.0) && (var_guard104 != 0.0)) {
        (0.0,)
    } else {
        (var_lp_s0,)
    }
};
        var_lp_s0 = assign8530_e8333;
        var_lp_s0_rv = 0.0;

        *var_flg_depmode_slot = var_flg_depmode;
        *var_flg_depmode_rv_slot = var_flg_depmode_rv;
        *var_guard103_slot = var_guard103;
        *var_guard103_rv_slot = var_guard103_rv;
        *var_guard104_slot = var_guard104;
        *var_guard104_rv_slot = var_guard104_rv;
        *var_lp_s0_slot = var_lp_s0;
        *var_lp_s0_rv_slot = var_lp_s0_rv;
        *var_phi_s0_bulk_slot = var_phi_s0_bulk;
        *var_phi_s0_bulk_dn0_slot = var_phi_s0_bulk_dn0;
        *var_phi_s0_bulk_dn10_slot = var_phi_s0_bulk_dn10;
        *var_phi_s0_bulk_dn11_slot = var_phi_s0_bulk_dn11;
        *var_phi_s0_bulk_dn12_slot = var_phi_s0_bulk_dn12;
        *var_phi_s0_bulk_dn2_slot = var_phi_s0_bulk_dn2;
        *var_phi_s0_bulk_dn4_slot = var_phi_s0_bulk_dn4;
        *var_phi_s0_bulk_dn5_slot = var_phi_s0_bulk_dn5;
        *var_phi_s0_bulk_dn6_slot = var_phi_s0_bulk_dn6;
        *var_phi_s0_bulk_dn8_slot = var_phi_s0_bulk_dn8;
        *var_phi_s0_bulk_rv_slot = var_phi_s0_bulk_rv;
        *var_psb_inia_slot = var_psb_inia;
        *var_psb_inia_dn0_slot = var_psb_inia_dn0;
        *var_psb_inia_dn10_slot = var_psb_inia_dn10;
        *var_psb_inia_dn11_slot = var_psb_inia_dn11;
        *var_psb_inia_dn12_slot = var_psb_inia_dn12;
        *var_psb_inia_dn2_slot = var_psb_inia_dn2;
        *var_psb_inia_dn4_slot = var_psb_inia_dn4;
        *var_psb_inia_dn5_slot = var_psb_inia_dn5;
        *var_psb_inia_dn6_slot = var_psb_inia_dn6;
        *var_psb_inia_dn8_slot = var_psb_inia_dn8;
        *var_psb_inia_rv_slot = var_psb_inia_rv;
        *var_psb_inib_slot = var_psb_inib;
        *var_psb_inib_dn0_slot = var_psb_inib_dn0;
        *var_psb_inib_dn10_slot = var_psb_inib_dn10;
        *var_psb_inib_dn11_slot = var_psb_inib_dn11;
        *var_psb_inib_dn12_slot = var_psb_inib_dn12;
        *var_psb_inib_dn2_slot = var_psb_inib_dn2;
        *var_psb_inib_dn4_slot = var_psb_inib_dn4;
        *var_psb_inib_dn5_slot = var_psb_inib_dn5;
        *var_psb_inib_dn6_slot = var_psb_inib_dn6;
        *var_psb_inib_dn8_slot = var_psb_inib_dn8;
        *var_psb_inib_rv_slot = var_psb_inib_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_31(
        var_beta: f64,
        var_beta_dn4: f64,
        var_c0bulk: f64,
        var_c0bulk_dn0: f64,
        var_c0bulk_dn10: f64,
        var_c0bulk_dn11: f64,
        var_c0bulk_dn12: f64,
        var_c0bulk_dn2: f64,
        var_c0bulk_dn4: f64,
        var_c0bulk_dn5: f64,
        var_c0bulk_dn6: f64,
        var_c0bulk_dn8: f64,
        var_c_box: f64,
        var_cnst0bulk: f64,
        var_cnst0bulk_dn0: f64,
        var_cnst0bulk_dn10: f64,
        var_cnst0bulk_dn11: f64,
        var_cnst0bulk_dn12: f64,
        var_cnst0bulk_dn2: f64,
        var_cnst0bulk_dn4: f64,
        var_cnst0bulk_dn5: f64,
        var_cnst0bulk_dn6: f64,
        var_cnst0bulk_dn8: f64,
        var_cnst1bulk: f64,
        var_cnst1bulk_dn0: f64,
        var_cnst1bulk_dn10: f64,
        var_cnst1bulk_dn11: f64,
        var_cnst1bulk_dn12: f64,
        var_cnst1bulk_dn2: f64,
        var_cnst1bulk_dn4: f64,
        var_cnst1bulk_dn5: f64,
        var_cnst1bulk_dn6: f64,
        var_cnst1bulk_dn8: f64,
        var_guard104: f64,
        var_guard74: f64,
        var_lp_s0_max: f64,
        var_phi_s0_soi: f64,
        var_phi_s0_soi_dn0: f64,
        var_phi_s0_soi_dn10: f64,
        var_phi_s0_soi_dn11: f64,
        var_phi_s0_soi_dn12: f64,
        var_phi_s0_soi_dn2: f64,
        var_phi_s0_soi_dn4: f64,
        var_phi_s0_soi_dn5: f64,
        var_phi_s0_soi_dn6: f64,
        var_phi_s0_soi_dn8: f64,
        var_ps0_inia: f64,
        var_ps0_inia_dn0: f64,
        var_ps0_inia_dn10: f64,
        var_ps0_inia_dn11: f64,
        var_ps0_inia_dn12: f64,
        var_ps0_inia_dn2: f64,
        var_ps0_inia_dn4: f64,
        var_ps0_inia_dn5: f64,
        var_ps0_inia_dn6: f64,
        var_ps0_inia_dn8: f64,
        var_vbsbiz: f64,
        var_vbsbiz_dn0: f64,
        var_vbsbiz_dn10: f64,
        var_vbsbiz_dn11: f64,
        var_vbsbiz_dn12: f64,
        var_vbsbiz_dn2: f64,
        var_vbsbiz_dn4: f64,
        var_vbsbiz_dn5: f64,
        var_vbsbiz_dn6: f64,
        var_vbsbiz_dn8: f64,
        var_flg_depmode_slot: &mut f64,
        var_flg_depmode_rv_slot: &mut f64,
        var_guard105_slot: &mut f64,
        var_guard105_rv_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard106_rv_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard107_rv_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard108_rv_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
        var_lp_s0_rv_slot: &mut f64,
        var_phi_b0_soi_slot: &mut f64,
        var_phi_b0_soi_dn0_slot: &mut f64,
        var_phi_b0_soi_dn10_slot: &mut f64,
        var_phi_b0_soi_dn11_slot: &mut f64,
        var_phi_b0_soi_dn12_slot: &mut f64,
        var_phi_b0_soi_dn2_slot: &mut f64,
        var_phi_b0_soi_dn4_slot: &mut f64,
        var_phi_b0_soi_dn5_slot: &mut f64,
        var_phi_b0_soi_dn6_slot: &mut f64,
        var_phi_b0_soi_dn8_slot: &mut f64,
        var_phi_b0_soi_rv_slot: &mut f64,
        var_phi_s0_bulk_slot: &mut f64,
        var_phi_s0_bulk_dn0_slot: &mut f64,
        var_phi_s0_bulk_dn10_slot: &mut f64,
        var_phi_s0_bulk_dn11_slot: &mut f64,
        var_phi_s0_bulk_dn12_slot: &mut f64,
        var_phi_s0_bulk_dn2_slot: &mut f64,
        var_phi_s0_bulk_dn4_slot: &mut f64,
        var_phi_s0_bulk_dn5_slot: &mut f64,
        var_phi_s0_bulk_dn6_slot: &mut f64,
        var_phi_s0_bulk_dn8_slot: &mut f64,
        var_phi_s0_bulk_rv_slot: &mut f64,
        var_ps0_slot: &mut f64,
        var_ps0_dn0_slot: &mut f64,
        var_ps0_dn10_slot: &mut f64,
        var_ps0_dn11_slot: &mut f64,
        var_ps0_dn12_slot: &mut f64,
        var_ps0_dn2_slot: &mut f64,
        var_ps0_dn4_slot: &mut f64,
        var_ps0_dn5_slot: &mut f64,
        var_ps0_dn6_slot: &mut f64,
        var_ps0_dn8_slot: &mut f64,
        var_ps0_rv_slot: &mut f64,
        var_ps_conv_ini_slot: &mut f64,
        var_ps_conv_ini_rv_slot: &mut f64,
        var_q_s0_bulk_slot: &mut f64,
        var_q_s0_bulk_dn0_slot: &mut f64,
        var_q_s0_bulk_dn10_slot: &mut f64,
        var_q_s0_bulk_dn11_slot: &mut f64,
        var_q_s0_bulk_dn12_slot: &mut f64,
        var_q_s0_bulk_dn2_slot: &mut f64,
        var_q_s0_bulk_dn4_slot: &mut f64,
        var_q_s0_bulk_dn5_slot: &mut f64,
        var_q_s0_bulk_dn6_slot: &mut f64,
        var_q_s0_bulk_dn8_slot: &mut f64,
        var_q_s0_bulk_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn12_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn12_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
    ) {
        let mut var_flg_depmode: f64 = *var_flg_depmode_slot;
        let mut var_flg_depmode_rv: f64 = *var_flg_depmode_rv_slot;
        let mut var_guard105: f64 = *var_guard105_slot;
        let mut var_guard105_rv: f64 = *var_guard105_rv_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard106_rv: f64 = *var_guard106_rv_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard107_rv: f64 = *var_guard107_rv_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard108_rv: f64 = *var_guard108_rv_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
        let mut var_lp_s0_rv: f64 = *var_lp_s0_rv_slot;
        let mut var_phi_b0_soi: f64 = *var_phi_b0_soi_slot;
        let mut var_phi_b0_soi_dn0: f64 = *var_phi_b0_soi_dn0_slot;
        let mut var_phi_b0_soi_dn10: f64 = *var_phi_b0_soi_dn10_slot;
        let mut var_phi_b0_soi_dn11: f64 = *var_phi_b0_soi_dn11_slot;
        let mut var_phi_b0_soi_dn12: f64 = *var_phi_b0_soi_dn12_slot;
        let mut var_phi_b0_soi_dn2: f64 = *var_phi_b0_soi_dn2_slot;
        let mut var_phi_b0_soi_dn4: f64 = *var_phi_b0_soi_dn4_slot;
        let mut var_phi_b0_soi_dn5: f64 = *var_phi_b0_soi_dn5_slot;
        let mut var_phi_b0_soi_dn6: f64 = *var_phi_b0_soi_dn6_slot;
        let mut var_phi_b0_soi_dn8: f64 = *var_phi_b0_soi_dn8_slot;
        let mut var_phi_b0_soi_rv: f64 = *var_phi_b0_soi_rv_slot;
        let mut var_phi_s0_bulk: f64 = *var_phi_s0_bulk_slot;
        let mut var_phi_s0_bulk_dn0: f64 = *var_phi_s0_bulk_dn0_slot;
        let mut var_phi_s0_bulk_dn10: f64 = *var_phi_s0_bulk_dn10_slot;
        let mut var_phi_s0_bulk_dn11: f64 = *var_phi_s0_bulk_dn11_slot;
        let mut var_phi_s0_bulk_dn12: f64 = *var_phi_s0_bulk_dn12_slot;
        let mut var_phi_s0_bulk_dn2: f64 = *var_phi_s0_bulk_dn2_slot;
        let mut var_phi_s0_bulk_dn4: f64 = *var_phi_s0_bulk_dn4_slot;
        let mut var_phi_s0_bulk_dn5: f64 = *var_phi_s0_bulk_dn5_slot;
        let mut var_phi_s0_bulk_dn6: f64 = *var_phi_s0_bulk_dn6_slot;
        let mut var_phi_s0_bulk_dn8: f64 = *var_phi_s0_bulk_dn8_slot;
        let mut var_phi_s0_bulk_rv: f64 = *var_phi_s0_bulk_rv_slot;
        let mut var_ps0: f64 = *var_ps0_slot;
        let mut var_ps0_dn0: f64 = *var_ps0_dn0_slot;
        let mut var_ps0_dn10: f64 = *var_ps0_dn10_slot;
        let mut var_ps0_dn11: f64 = *var_ps0_dn11_slot;
        let mut var_ps0_dn12: f64 = *var_ps0_dn12_slot;
        let mut var_ps0_dn2: f64 = *var_ps0_dn2_slot;
        let mut var_ps0_dn4: f64 = *var_ps0_dn4_slot;
        let mut var_ps0_dn5: f64 = *var_ps0_dn5_slot;
        let mut var_ps0_dn6: f64 = *var_ps0_dn6_slot;
        let mut var_ps0_dn8: f64 = *var_ps0_dn8_slot;
        let mut var_ps0_rv: f64 = *var_ps0_rv_slot;
        let mut var_ps_conv_ini: f64 = *var_ps_conv_ini_slot;
        let mut var_ps_conv_ini_rv: f64 = *var_ps_conv_ini_rv_slot;
        let mut var_q_s0_bulk: f64 = *var_q_s0_bulk_slot;
        let mut var_q_s0_bulk_dn0: f64 = *var_q_s0_bulk_dn0_slot;
        let mut var_q_s0_bulk_dn10: f64 = *var_q_s0_bulk_dn10_slot;
        let mut var_q_s0_bulk_dn11: f64 = *var_q_s0_bulk_dn11_slot;
        let mut var_q_s0_bulk_dn12: f64 = *var_q_s0_bulk_dn12_slot;
        let mut var_q_s0_bulk_dn2: f64 = *var_q_s0_bulk_dn2_slot;
        let mut var_q_s0_bulk_dn4: f64 = *var_q_s0_bulk_dn4_slot;
        let mut var_q_s0_bulk_dn5: f64 = *var_q_s0_bulk_dn5_slot;
        let mut var_q_s0_bulk_dn6: f64 = *var_q_s0_bulk_dn6_slot;
        let mut var_q_s0_bulk_dn8: f64 = *var_q_s0_bulk_dn8_slot;
        let mut var_q_s0_bulk_rv: f64 = *var_q_s0_bulk_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn12: f64 = *var_t6_dn12_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn12: f64 = *var_t7_dn12_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;

        let mut assign8540_loop_guard: usize = 0;
        while {
            let assign8540_cond_e8341: f64 = if (((var_guard74 == 0.0) && (var_guard104 != 0.0)) && (var_lp_s0 < var_lp_s0_max)) { 1.0 } else { 0.0 };
            assign8540_cond_e8341 != 0.0
        } {
            assign8540_loop_guard += 1;
            assert!(assign8540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign8540_body0_e8348, assign8540_body0_e8348_d_n0, assign8540_body0_e8348_d_n2, assign8540_body0_e8348_d_n4, assign8540_body0_e8348_d_n5, assign8540_body0_e8348_d_n6, assign8540_body0_e8348_d_n8, assign8540_body0_e8348_d_n10, assign8540_body0_e8348_d_n11, assign8540_body0_e8348_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 != 0.0)) {
        (var_cnst0bulk, var_cnst0bulk_dn0, var_cnst0bulk_dn2, var_cnst0bulk_dn4, var_cnst0bulk_dn5, var_cnst0bulk_dn6, var_cnst0bulk_dn8, var_cnst0bulk_dn10, var_cnst0bulk_dn11, var_cnst0bulk_dn12,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
            var_t1 = assign8540_body0_e8348;
            var_t1_dn0 = assign8540_body0_e8348_d_n0;
            var_t1_dn2 = assign8540_body0_e8348_d_n2;
            var_t1_dn4 = assign8540_body0_e8348_d_n4;
            var_t1_dn5 = assign8540_body0_e8348_d_n5;
            var_t1_dn6 = assign8540_body0_e8348_d_n6;
            var_t1_dn8 = assign8540_body0_e8348_d_n8;
            var_t1_dn10 = assign8540_body0_e8348_d_n10;
            var_t1_dn11 = assign8540_body0_e8348_d_n11;
            var_t1_dn12 = assign8540_body0_e8348_d_n12;
            var_t1_rv = 0.0;
            let (assign8540_body1_e8357, assign8540_body1_e8357_d_n0, assign8540_body1_e8357_d_n2, assign8540_body1_e8357_d_n4, assign8540_body1_e8357_d_n5, assign8540_body1_e8357_d_n6, assign8540_body1_e8357_d_n8, assign8540_body1_e8357_d_n10, assign8540_body1_e8357_d_n11, assign8540_body1_e8357_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 != 0.0)) {
        let assign8540_body1_e8355: f64 = (var_beta * var_phi_s0_bulk);
        (assign8540_body1_e8355, (var_beta * var_phi_s0_bulk_dn0), (var_beta * var_phi_s0_bulk_dn2), ((var_beta_dn4 * var_phi_s0_bulk) + (var_beta * var_phi_s0_bulk_dn4)), (var_beta * var_phi_s0_bulk_dn5), (var_beta * var_phi_s0_bulk_dn6), (var_beta * var_phi_s0_bulk_dn8), (var_beta * var_phi_s0_bulk_dn10), (var_beta * var_phi_s0_bulk_dn11), (var_beta * var_phi_s0_bulk_dn12),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
            var_t2 = assign8540_body1_e8357;
            var_t2_dn0 = assign8540_body1_e8357_d_n0;
            var_t2_dn2 = assign8540_body1_e8357_d_n2;
            var_t2_dn4 = assign8540_body1_e8357_d_n4;
            var_t2_dn5 = assign8540_body1_e8357_d_n5;
            var_t2_dn6 = assign8540_body1_e8357_d_n6;
            var_t2_dn8 = assign8540_body1_e8357_d_n8;
            var_t2_dn10 = assign8540_body1_e8357_d_n10;
            var_t2_dn11 = assign8540_body1_e8357_d_n11;
            var_t2_dn12 = assign8540_body1_e8357_d_n12;
            var_t2_rv = 0.0;
            let (assign8540_body2_e8366, assign8540_body2_e8366_d_n0, assign8540_body2_e8366_d_n2, assign8540_body2_e8366_d_n4, assign8540_body2_e8366_d_n5, assign8540_body2_e8366_d_n6, assign8540_body2_e8366_d_n8, assign8540_body2_e8366_d_n10, assign8540_body2_e8366_d_n11, assign8540_body2_e8366_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 != 0.0)) {
        let assign8540_body2_e8363: f64 = (-var_t2);
        let assign8540_body2_e8364: f64 = (assign8540_body2_e8363).exp();
        (assign8540_body2_e8364, (assign8540_body2_e8364 * (-var_t2_dn0)), (assign8540_body2_e8364 * (-var_t2_dn2)), (assign8540_body2_e8364 * (-var_t2_dn4)), (assign8540_body2_e8364 * (-var_t2_dn5)), (assign8540_body2_e8364 * (-var_t2_dn6)), (assign8540_body2_e8364 * (-var_t2_dn8)), (assign8540_body2_e8364 * (-var_t2_dn10)), (assign8540_body2_e8364 * (-var_t2_dn11)), (assign8540_body2_e8364 * (-var_t2_dn12)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
            var_t3 = assign8540_body2_e8366;
            var_t3_dn0 = assign8540_body2_e8366_d_n0;
            var_t3_dn2 = assign8540_body2_e8366_d_n2;
            var_t3_dn4 = assign8540_body2_e8366_d_n4;
            var_t3_dn5 = assign8540_body2_e8366_d_n5;
            var_t3_dn6 = assign8540_body2_e8366_d_n6;
            var_t3_dn8 = assign8540_body2_e8366_d_n8;
            var_t3_dn10 = assign8540_body2_e8366_d_n10;
            var_t3_dn11 = assign8540_body2_e8366_d_n11;
            var_t3_dn12 = assign8540_body2_e8366_d_n12;
            var_t3_rv = 0.0;
            let assign8540_body3_e8369: f64 = if var_phi_s0_bulk > 1e-8 { 1.0 } else { 0.0 };
            var_guard105 = assign8540_body3_e8369;
            var_guard105_rv = 0.0;
            let (assign8540_body4_e8381, assign8540_body4_e8381_d_n0, assign8540_body4_e8381_d_n2, assign8540_body4_e8381_d_n4, assign8540_body4_e8381_d_n5, assign8540_body4_e8381_d_n6, assign8540_body4_e8381_d_n8, assign8540_body4_e8381_d_n10, assign8540_body4_e8381_d_n11, assign8540_body4_e8381_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 != 0.0)) && (var_guard105 != 0.0)) {
        let assign8540_body4_e8378: f64 = (var_beta * var_phi_s0_bulk);
        let assign8540_body4_e8379: f64 = (assign8540_body4_e8378).exp();
        (assign8540_body4_e8379, (assign8540_body4_e8379 * (var_beta * var_phi_s0_bulk_dn0)), (assign8540_body4_e8379 * (var_beta * var_phi_s0_bulk_dn2)), (assign8540_body4_e8379 * ((var_beta_dn4 * var_phi_s0_bulk) + (var_beta * var_phi_s0_bulk_dn4))), (assign8540_body4_e8379 * (var_beta * var_phi_s0_bulk_dn5)), (assign8540_body4_e8379 * (var_beta * var_phi_s0_bulk_dn6)), (assign8540_body4_e8379 * (var_beta * var_phi_s0_bulk_dn8)), (assign8540_body4_e8379 * (var_beta * var_phi_s0_bulk_dn10)), (assign8540_body4_e8379 * (var_beta * var_phi_s0_bulk_dn11)), (assign8540_body4_e8379 * (var_beta * var_phi_s0_bulk_dn12)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
            var_t0 = assign8540_body4_e8381;
            var_t0_dn0 = assign8540_body4_e8381_d_n0;
            var_t0_dn2 = assign8540_body4_e8381_d_n2;
            var_t0_dn4 = assign8540_body4_e8381_d_n4;
            var_t0_dn5 = assign8540_body4_e8381_d_n5;
            var_t0_dn6 = assign8540_body4_e8381_d_n6;
            var_t0_dn8 = assign8540_body4_e8381_d_n8;
            var_t0_dn10 = assign8540_body4_e8381_d_n10;
            var_t0_dn11 = assign8540_body4_e8381_d_n11;
            var_t0_dn12 = assign8540_body4_e8381_d_n12;
            var_t0_rv = 0.0;
            let (assign8540_body5_e8404, assign8540_body5_e8404_d_n0, assign8540_body5_e8404_d_n2, assign8540_body5_e8404_d_n4, assign8540_body5_e8404_d_n5, assign8540_body5_e8404_d_n6, assign8540_body5_e8404_d_n8, assign8540_body5_e8404_d_n10, assign8540_body5_e8404_d_n11, assign8540_body5_e8404_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 != 0.0)) && (var_guard105 != 0.0)) {
        let assign8540_body5_e8389: f64 = (-var_t1);
        let assign8540_body5_e8392: f64 = (var_t3 + var_t2);
        let assign8540_body5_e8394: f64 = (assign8540_body5_e8392 - 1.0);
        let assign8540_body5_e8398: f64 = (var_t0 - 1.0);
        let assign8540_body5_e8399: f64 = (var_cnst1bulk * assign8540_body5_e8398);
        let assign8540_body5_e8400: f64 = (assign8540_body5_e8394 + assign8540_body5_e8399);
        let assign8540_body5_e8401: f64 = (assign8540_body5_e8400).sqrt();
        let assign8540_body5_e8402: f64 = (assign8540_body5_e8389 * assign8540_body5_e8401);
        (assign8540_body5_e8402, (((-var_t1_dn0) * assign8540_body5_e8401) + (assign8540_body5_e8389 * (((var_t3_dn0 + var_t2_dn0) + ((var_cnst1bulk_dn0 * assign8540_body5_e8398) + (var_cnst1bulk * var_t0_dn0))) / (2.0 * assign8540_body5_e8401)))), (((-var_t1_dn2) * assign8540_body5_e8401) + (assign8540_body5_e8389 * (((var_t3_dn2 + var_t2_dn2) + ((var_cnst1bulk_dn2 * assign8540_body5_e8398) + (var_cnst1bulk * var_t0_dn2))) / (2.0 * assign8540_body5_e8401)))), (((-var_t1_dn4) * assign8540_body5_e8401) + (assign8540_body5_e8389 * (((var_t3_dn4 + var_t2_dn4) + ((var_cnst1bulk_dn4 * assign8540_body5_e8398) + (var_cnst1bulk * var_t0_dn4))) / (2.0 * assign8540_body5_e8401)))), (((-var_t1_dn5) * assign8540_body5_e8401) + (assign8540_body5_e8389 * (((var_t3_dn5 + var_t2_dn5) + ((var_cnst1bulk_dn5 * assign8540_body5_e8398) + (var_cnst1bulk * var_t0_dn5))) / (2.0 * assign8540_body5_e8401)))), (((-var_t1_dn6) * assign8540_body5_e8401) + (assign8540_body5_e8389 * (((var_t3_dn6 + var_t2_dn6) + ((var_cnst1bulk_dn6 * assign8540_body5_e8398) + (var_cnst1bulk * var_t0_dn6))) / (2.0 * assign8540_body5_e8401)))), (((-var_t1_dn8) * assign8540_body5_e8401) + (assign8540_body5_e8389 * (((var_t3_dn8 + var_t2_dn8) + ((var_cnst1bulk_dn8 * assign8540_body5_e8398) + (var_cnst1bulk * var_t0_dn8))) / (2.0 * assign8540_body5_e8401)))), (((-var_t1_dn10) * assign8540_body5_e8401) + (assign8540_body5_e8389 * (((var_t3_dn10 + var_t2_dn10) + ((var_cnst1bulk_dn10 * assign8540_body5_e8398) + (var_cnst1bulk * var_t0_dn10))) / (2.0 * assign8540_body5_e8401)))), (((-var_t1_dn11) * assign8540_body5_e8401) + (assign8540_body5_e8389 * (((var_t3_dn11 + var_t2_dn11) + ((var_cnst1bulk_dn11 * assign8540_body5_e8398) + (var_cnst1bulk * var_t0_dn11))) / (2.0 * assign8540_body5_e8401)))), (((-var_t1_dn12) * assign8540_body5_e8401) + (assign8540_body5_e8389 * (((var_t3_dn12 + var_t2_dn12) + ((var_cnst1bulk_dn12 * assign8540_body5_e8398) + (var_cnst1bulk * var_t0_dn12))) / (2.0 * assign8540_body5_e8401)))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign8540_body5_e8404;
            var_t4_dn0 = assign8540_body5_e8404_d_n0;
            var_t4_dn2 = assign8540_body5_e8404_d_n2;
            var_t4_dn4 = assign8540_body5_e8404_d_n4;
            var_t4_dn5 = assign8540_body5_e8404_d_n5;
            var_t4_dn6 = assign8540_body5_e8404_d_n6;
            var_t4_dn8 = assign8540_body5_e8404_d_n8;
            var_t4_dn10 = assign8540_body5_e8404_d_n10;
            var_t4_dn11 = assign8540_body5_e8404_d_n11;
            var_t4_dn12 = assign8540_body5_e8404_d_n12;
            var_t4_rv = 0.0;
            let (assign8540_body6_e8424, assign8540_body6_e8424_d_n0, assign8540_body6_e8424_d_n2, assign8540_body6_e8424_d_n4, assign8540_body6_e8424_d_n5, assign8540_body6_e8424_d_n6, assign8540_body6_e8424_d_n8, assign8540_body6_e8424_d_n10, assign8540_body6_e8424_d_n11, assign8540_body6_e8424_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 != 0.0)) && (var_guard105 != 0.0)) {
        let assign8540_body6_e8413: f64 = (var_c0bulk / var_t4);
        let assign8540_body6_e8415: f64 = (-var_t3);
        let assign8540_body6_e8417: f64 = (assign8540_body6_e8415 + 1.0);
        let assign8540_body6_e8420: f64 = (var_cnst1bulk * var_t0);
        let assign8540_body6_e8421: f64 = (assign8540_body6_e8417 + assign8540_body6_e8420);
        let assign8540_body6_e8422: f64 = (assign8540_body6_e8413 * assign8540_body6_e8421);
        (assign8540_body6_e8422, (((((var_c0bulk_dn0 * var_t4) - (var_c0bulk * var_t4_dn0)) / (var_t4 * var_t4)) * assign8540_body6_e8421) + (assign8540_body6_e8413 * ((-var_t3_dn0) + ((var_cnst1bulk_dn0 * var_t0) + (var_cnst1bulk * var_t0_dn0))))), (((((var_c0bulk_dn2 * var_t4) - (var_c0bulk * var_t4_dn2)) / (var_t4 * var_t4)) * assign8540_body6_e8421) + (assign8540_body6_e8413 * ((-var_t3_dn2) + ((var_cnst1bulk_dn2 * var_t0) + (var_cnst1bulk * var_t0_dn2))))), (((((var_c0bulk_dn4 * var_t4) - (var_c0bulk * var_t4_dn4)) / (var_t4 * var_t4)) * assign8540_body6_e8421) + (assign8540_body6_e8413 * ((-var_t3_dn4) + ((var_cnst1bulk_dn4 * var_t0) + (var_cnst1bulk * var_t0_dn4))))), (((((var_c0bulk_dn5 * var_t4) - (var_c0bulk * var_t4_dn5)) / (var_t4 * var_t4)) * assign8540_body6_e8421) + (assign8540_body6_e8413 * ((-var_t3_dn5) + ((var_cnst1bulk_dn5 * var_t0) + (var_cnst1bulk * var_t0_dn5))))), (((((var_c0bulk_dn6 * var_t4) - (var_c0bulk * var_t4_dn6)) / (var_t4 * var_t4)) * assign8540_body6_e8421) + (assign8540_body6_e8413 * ((-var_t3_dn6) + ((var_cnst1bulk_dn6 * var_t0) + (var_cnst1bulk * var_t0_dn6))))), (((((var_c0bulk_dn8 * var_t4) - (var_c0bulk * var_t4_dn8)) / (var_t4 * var_t4)) * assign8540_body6_e8421) + (assign8540_body6_e8413 * ((-var_t3_dn8) + ((var_cnst1bulk_dn8 * var_t0) + (var_cnst1bulk * var_t0_dn8))))), (((((var_c0bulk_dn10 * var_t4) - (var_c0bulk * var_t4_dn10)) / (var_t4 * var_t4)) * assign8540_body6_e8421) + (assign8540_body6_e8413 * ((-var_t3_dn10) + ((var_cnst1bulk_dn10 * var_t0) + (var_cnst1bulk * var_t0_dn10))))), (((((var_c0bulk_dn11 * var_t4) - (var_c0bulk * var_t4_dn11)) / (var_t4 * var_t4)) * assign8540_body6_e8421) + (assign8540_body6_e8413 * ((-var_t3_dn11) + ((var_cnst1bulk_dn11 * var_t0) + (var_cnst1bulk * var_t0_dn11))))), (((((var_c0bulk_dn12 * var_t4) - (var_c0bulk * var_t4_dn12)) / (var_t4 * var_t4)) * assign8540_body6_e8421) + (assign8540_body6_e8413 * ((-var_t3_dn12) + ((var_cnst1bulk_dn12 * var_t0) + (var_cnst1bulk * var_t0_dn12))))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign8540_body6_e8424;
            var_t5_dn0 = assign8540_body6_e8424_d_n0;
            var_t5_dn2 = assign8540_body6_e8424_d_n2;
            var_t5_dn4 = assign8540_body6_e8424_d_n4;
            var_t5_dn5 = assign8540_body6_e8424_d_n5;
            var_t5_dn6 = assign8540_body6_e8424_d_n6;
            var_t5_dn8 = assign8540_body6_e8424_d_n8;
            var_t5_dn10 = assign8540_body6_e8424_d_n10;
            var_t5_dn11 = assign8540_body6_e8424_d_n11;
            var_t5_dn12 = assign8540_body6_e8424_d_n12;
            var_t5_rv = 0.0;
            let assign8540_body7_e8427: f64 = (-1e-8);
            let assign8540_body7_e8428: f64 = if var_phi_s0_bulk < assign8540_body7_e8427 { 1.0 } else { 0.0 };
            var_guard106 = assign8540_body7_e8428;
            var_guard106_rv = 0.0;
            let (assign8540_body8_e8447, assign8540_body8_e8447_d_n0, assign8540_body8_e8447_d_n2, assign8540_body8_e8447_d_n4, assign8540_body8_e8447_d_n5, assign8540_body8_e8447_d_n6, assign8540_body8_e8447_d_n8, assign8540_body8_e8447_d_n10, assign8540_body8_e8447_d_n11, assign8540_body8_e8447_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard104 != 0.0)) && (var_guard105 == 0.0)) && (var_guard106 != 0.0)) {
        let assign8540_body8_e8441: f64 = (var_t3 + var_t2);
        let assign8540_body8_e8443: f64 = (assign8540_body8_e8441 - 1.0);
        let assign8540_body8_e8444: f64 = (assign8540_body8_e8443).sqrt();
        let assign8540_body8_e8445: f64 = (var_t1 * assign8540_body8_e8444);
        (assign8540_body8_e8445, ((var_t1_dn0 * assign8540_body8_e8444) + (var_t1 * ((var_t3_dn0 + var_t2_dn0) / (2.0 * assign8540_body8_e8444)))), ((var_t1_dn2 * assign8540_body8_e8444) + (var_t1 * ((var_t3_dn2 + var_t2_dn2) / (2.0 * assign8540_body8_e8444)))), ((var_t1_dn4 * assign8540_body8_e8444) + (var_t1 * ((var_t3_dn4 + var_t2_dn4) / (2.0 * assign8540_body8_e8444)))), ((var_t1_dn5 * assign8540_body8_e8444) + (var_t1 * ((var_t3_dn5 + var_t2_dn5) / (2.0 * assign8540_body8_e8444)))), ((var_t1_dn6 * assign8540_body8_e8444) + (var_t1 * ((var_t3_dn6 + var_t2_dn6) / (2.0 * assign8540_body8_e8444)))), ((var_t1_dn8 * assign8540_body8_e8444) + (var_t1 * ((var_t3_dn8 + var_t2_dn8) / (2.0 * assign8540_body8_e8444)))), ((var_t1_dn10 * assign8540_body8_e8444) + (var_t1 * ((var_t3_dn10 + var_t2_dn10) / (2.0 * assign8540_body8_e8444)))), ((var_t1_dn11 * assign8540_body8_e8444) + (var_t1 * ((var_t3_dn11 + var_t2_dn11) / (2.0 * assign8540_body8_e8444)))), ((var_t1_dn12 * assign8540_body8_e8444) + (var_t1 * ((var_t3_dn12 + var_t2_dn12) / (2.0 * assign8540_body8_e8444)))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign8540_body8_e8447;
            var_t4_dn0 = assign8540_body8_e8447_d_n0;
            var_t4_dn2 = assign8540_body8_e8447_d_n2;
            var_t4_dn4 = assign8540_body8_e8447_d_n4;
            var_t4_dn5 = assign8540_body8_e8447_d_n5;
            var_t4_dn6 = assign8540_body8_e8447_d_n6;
            var_t4_dn8 = assign8540_body8_e8447_d_n8;
            var_t4_dn10 = assign8540_body8_e8447_d_n10;
            var_t4_dn11 = assign8540_body8_e8447_d_n11;
            var_t4_dn12 = assign8540_body8_e8447_d_n12;
            var_t4_rv = 0.0;
            let (assign8540_body9_e8466, assign8540_body9_e8466_d_n0, assign8540_body9_e8466_d_n2, assign8540_body9_e8466_d_n4, assign8540_body9_e8466_d_n5, assign8540_body9_e8466_d_n6, assign8540_body9_e8466_d_n8, assign8540_body9_e8466_d_n10, assign8540_body9_e8466_d_n11, assign8540_body9_e8466_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard104 != 0.0)) && (var_guard105 == 0.0)) && (var_guard106 != 0.0)) {
        let assign8540_body9_e8459: f64 = (var_c0bulk / var_t4);
        let assign8540_body9_e8461: f64 = (-var_t3);
        let assign8540_body9_e8463: f64 = (assign8540_body9_e8461 + 1.0);
        let assign8540_body9_e8464: f64 = (assign8540_body9_e8459 * assign8540_body9_e8463);
        (assign8540_body9_e8464, (((((var_c0bulk_dn0 * var_t4) - (var_c0bulk * var_t4_dn0)) / (var_t4 * var_t4)) * assign8540_body9_e8463) + (assign8540_body9_e8459 * (-var_t3_dn0))), (((((var_c0bulk_dn2 * var_t4) - (var_c0bulk * var_t4_dn2)) / (var_t4 * var_t4)) * assign8540_body9_e8463) + (assign8540_body9_e8459 * (-var_t3_dn2))), (((((var_c0bulk_dn4 * var_t4) - (var_c0bulk * var_t4_dn4)) / (var_t4 * var_t4)) * assign8540_body9_e8463) + (assign8540_body9_e8459 * (-var_t3_dn4))), (((((var_c0bulk_dn5 * var_t4) - (var_c0bulk * var_t4_dn5)) / (var_t4 * var_t4)) * assign8540_body9_e8463) + (assign8540_body9_e8459 * (-var_t3_dn5))), (((((var_c0bulk_dn6 * var_t4) - (var_c0bulk * var_t4_dn6)) / (var_t4 * var_t4)) * assign8540_body9_e8463) + (assign8540_body9_e8459 * (-var_t3_dn6))), (((((var_c0bulk_dn8 * var_t4) - (var_c0bulk * var_t4_dn8)) / (var_t4 * var_t4)) * assign8540_body9_e8463) + (assign8540_body9_e8459 * (-var_t3_dn8))), (((((var_c0bulk_dn10 * var_t4) - (var_c0bulk * var_t4_dn10)) / (var_t4 * var_t4)) * assign8540_body9_e8463) + (assign8540_body9_e8459 * (-var_t3_dn10))), (((((var_c0bulk_dn11 * var_t4) - (var_c0bulk * var_t4_dn11)) / (var_t4 * var_t4)) * assign8540_body9_e8463) + (assign8540_body9_e8459 * (-var_t3_dn11))), (((((var_c0bulk_dn12 * var_t4) - (var_c0bulk * var_t4_dn12)) / (var_t4 * var_t4)) * assign8540_body9_e8463) + (assign8540_body9_e8459 * (-var_t3_dn12))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign8540_body9_e8466;
            var_t5_dn0 = assign8540_body9_e8466_d_n0;
            var_t5_dn2 = assign8540_body9_e8466_d_n2;
            var_t5_dn4 = assign8540_body9_e8466_d_n4;
            var_t5_dn5 = assign8540_body9_e8466_d_n5;
            var_t5_dn6 = assign8540_body9_e8466_d_n6;
            var_t5_dn8 = assign8540_body9_e8466_d_n8;
            var_t5_dn10 = assign8540_body9_e8466_d_n10;
            var_t5_dn11 = assign8540_body9_e8466_d_n11;
            var_t5_dn12 = assign8540_body9_e8466_d_n12;
            var_t5_rv = 0.0;
            let (assign8540_body10_e8487, assign8540_body10_e8487_d_n0, assign8540_body10_e8487_d_n2, assign8540_body10_e8487_d_n4, assign8540_body10_e8487_d_n5, assign8540_body10_e8487_d_n6, assign8540_body10_e8487_d_n8, assign8540_body10_e8487_d_n10, assign8540_body10_e8487_d_n11, assign8540_body10_e8487_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard104 != 0.0)) && (var_guard105 == 0.0)) && (var_guard106 == 0.0)) {
        let assign8540_body10_e8479: f64 = (var_c0bulk / var_beta);
        let assign8540_body10_e8480: f64 = (assign8540_body10_e8479).sqrt();
        let assign8540_body10_e8481: f64 = (-assign8540_body10_e8480);
        let assign8540_body10_e8483: f64 = (assign8540_body10_e8481 * var_beta);
        let assign8540_body10_e8485: f64 = (assign8540_body10_e8483 * var_phi_s0_bulk);
        (assign8540_body10_e8485, ((((-((var_c0bulk_dn0 / var_beta) / (2.0 * assign8540_body10_e8480))) * var_beta) * var_phi_s0_bulk) + (assign8540_body10_e8483 * var_phi_s0_bulk_dn0)), ((((-((var_c0bulk_dn2 / var_beta) / (2.0 * assign8540_body10_e8480))) * var_beta) * var_phi_s0_bulk) + (assign8540_body10_e8483 * var_phi_s0_bulk_dn2)), (((((-((((var_c0bulk_dn4 * var_beta) - (var_c0bulk * var_beta_dn4)) / (var_beta * var_beta)) / (2.0 * assign8540_body10_e8480))) * var_beta) + (assign8540_body10_e8481 * var_beta_dn4)) * var_phi_s0_bulk) + (assign8540_body10_e8483 * var_phi_s0_bulk_dn4)), ((((-((var_c0bulk_dn5 / var_beta) / (2.0 * assign8540_body10_e8480))) * var_beta) * var_phi_s0_bulk) + (assign8540_body10_e8483 * var_phi_s0_bulk_dn5)), ((((-((var_c0bulk_dn6 / var_beta) / (2.0 * assign8540_body10_e8480))) * var_beta) * var_phi_s0_bulk) + (assign8540_body10_e8483 * var_phi_s0_bulk_dn6)), ((((-((var_c0bulk_dn8 / var_beta) / (2.0 * assign8540_body10_e8480))) * var_beta) * var_phi_s0_bulk) + (assign8540_body10_e8483 * var_phi_s0_bulk_dn8)), ((((-((var_c0bulk_dn10 / var_beta) / (2.0 * assign8540_body10_e8480))) * var_beta) * var_phi_s0_bulk) + (assign8540_body10_e8483 * var_phi_s0_bulk_dn10)), ((((-((var_c0bulk_dn11 / var_beta) / (2.0 * assign8540_body10_e8480))) * var_beta) * var_phi_s0_bulk) + (assign8540_body10_e8483 * var_phi_s0_bulk_dn11)), ((((-((var_c0bulk_dn12 / var_beta) / (2.0 * assign8540_body10_e8480))) * var_beta) * var_phi_s0_bulk) + (assign8540_body10_e8483 * var_phi_s0_bulk_dn12)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign8540_body10_e8487;
            var_t4_dn0 = assign8540_body10_e8487_d_n0;
            var_t4_dn2 = assign8540_body10_e8487_d_n2;
            var_t4_dn4 = assign8540_body10_e8487_d_n4;
            var_t4_dn5 = assign8540_body10_e8487_d_n5;
            var_t4_dn6 = assign8540_body10_e8487_d_n6;
            var_t4_dn8 = assign8540_body10_e8487_d_n8;
            var_t4_dn10 = assign8540_body10_e8487_d_n10;
            var_t4_dn11 = assign8540_body10_e8487_d_n11;
            var_t4_dn12 = assign8540_body10_e8487_d_n12;
            var_t4_rv = 0.0;
            let (assign8540_body11_e8504, assign8540_body11_e8504_d_n0, assign8540_body11_e8504_d_n2, assign8540_body11_e8504_d_n4, assign8540_body11_e8504_d_n5, assign8540_body11_e8504_d_n6, assign8540_body11_e8504_d_n8, assign8540_body11_e8504_d_n10, assign8540_body11_e8504_d_n11, assign8540_body11_e8504_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard104 != 0.0)) && (var_guard105 == 0.0)) && (var_guard106 == 0.0)) {
        let assign8540_body11_e8500: f64 = (var_c0bulk * var_beta);
        let assign8540_body11_e8501: f64 = (assign8540_body11_e8500).sqrt();
        let assign8540_body11_e8502: f64 = (-assign8540_body11_e8501);
        (assign8540_body11_e8502, (-((var_c0bulk_dn0 * var_beta) / (2.0 * assign8540_body11_e8501))), (-((var_c0bulk_dn2 * var_beta) / (2.0 * assign8540_body11_e8501))), (-(((var_c0bulk_dn4 * var_beta) + (var_c0bulk * var_beta_dn4)) / (2.0 * assign8540_body11_e8501))), (-((var_c0bulk_dn5 * var_beta) / (2.0 * assign8540_body11_e8501))), (-((var_c0bulk_dn6 * var_beta) / (2.0 * assign8540_body11_e8501))), (-((var_c0bulk_dn8 * var_beta) / (2.0 * assign8540_body11_e8501))), (-((var_c0bulk_dn10 * var_beta) / (2.0 * assign8540_body11_e8501))), (-((var_c0bulk_dn11 * var_beta) / (2.0 * assign8540_body11_e8501))), (-((var_c0bulk_dn12 * var_beta) / (2.0 * assign8540_body11_e8501))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign8540_body11_e8504;
            var_t5_dn0 = assign8540_body11_e8504_d_n0;
            var_t5_dn2 = assign8540_body11_e8504_d_n2;
            var_t5_dn4 = assign8540_body11_e8504_d_n4;
            var_t5_dn5 = assign8540_body11_e8504_d_n5;
            var_t5_dn6 = assign8540_body11_e8504_d_n6;
            var_t5_dn8 = assign8540_body11_e8504_d_n8;
            var_t5_dn10 = assign8540_body11_e8504_d_n10;
            var_t5_dn11 = assign8540_body11_e8504_d_n11;
            var_t5_dn12 = assign8540_body11_e8504_d_n12;
            var_t5_rv = 0.0;
            let (assign8540_body12_e8527, assign8540_body12_e8527_d_n0, assign8540_body12_e8527_d_n2, assign8540_body12_e8527_d_n4, assign8540_body12_e8527_d_n5, assign8540_body12_e8527_d_n6, assign8540_body12_e8527_d_n8, assign8540_body12_e8527_d_n10, assign8540_body12_e8527_d_n11, assign8540_body12_e8527_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 != 0.0)) {
        let assign8540_body12_e8511: f64 = (-var_phi_s0_bulk);
        let assign8540_body12_e8514: f64 = (var_t4 / var_c_box);
        let assign8540_body12_e8515: f64 = (assign8540_body12_e8511 + assign8540_body12_e8514);
        let assign8540_body12_e8517: f64 = (assign8540_body12_e8515 - var_vbsbiz);
        let assign8540_body12_e8519: f64 = (-1.0);
        let assign8540_body12_e8522: f64 = (var_t5 / var_c_box);
        let assign8540_body12_e8523: f64 = (assign8540_body12_e8519 + assign8540_body12_e8522);
        let assign8540_body12_e8524: f64 = (assign8540_body12_e8517 / assign8540_body12_e8523);
        let assign8540_body12_e8525: f64 = (var_phi_s0_bulk - assign8540_body12_e8524);
        (assign8540_body12_e8525, (var_phi_s0_bulk_dn0 - ((((((-var_phi_s0_bulk_dn0) + (var_t4_dn0 / var_c_box)) - var_vbsbiz_dn0) * assign8540_body12_e8523) - (assign8540_body12_e8517 * (var_t5_dn0 / var_c_box))) / (assign8540_body12_e8523 * assign8540_body12_e8523))), (var_phi_s0_bulk_dn2 - ((((((-var_phi_s0_bulk_dn2) + (var_t4_dn2 / var_c_box)) - var_vbsbiz_dn2) * assign8540_body12_e8523) - (assign8540_body12_e8517 * (var_t5_dn2 / var_c_box))) / (assign8540_body12_e8523 * assign8540_body12_e8523))), (var_phi_s0_bulk_dn4 - ((((((-var_phi_s0_bulk_dn4) + (var_t4_dn4 / var_c_box)) - var_vbsbiz_dn4) * assign8540_body12_e8523) - (assign8540_body12_e8517 * (var_t5_dn4 / var_c_box))) / (assign8540_body12_e8523 * assign8540_body12_e8523))), (var_phi_s0_bulk_dn5 - ((((((-var_phi_s0_bulk_dn5) + (var_t4_dn5 / var_c_box)) - var_vbsbiz_dn5) * assign8540_body12_e8523) - (assign8540_body12_e8517 * (var_t5_dn5 / var_c_box))) / (assign8540_body12_e8523 * assign8540_body12_e8523))), (var_phi_s0_bulk_dn6 - ((((((-var_phi_s0_bulk_dn6) + (var_t4_dn6 / var_c_box)) - var_vbsbiz_dn6) * assign8540_body12_e8523) - (assign8540_body12_e8517 * (var_t5_dn6 / var_c_box))) / (assign8540_body12_e8523 * assign8540_body12_e8523))), (var_phi_s0_bulk_dn8 - ((((((-var_phi_s0_bulk_dn8) + (var_t4_dn8 / var_c_box)) - var_vbsbiz_dn8) * assign8540_body12_e8523) - (assign8540_body12_e8517 * (var_t5_dn8 / var_c_box))) / (assign8540_body12_e8523 * assign8540_body12_e8523))), (var_phi_s0_bulk_dn10 - ((((((-var_phi_s0_bulk_dn10) + (var_t4_dn10 / var_c_box)) - var_vbsbiz_dn10) * assign8540_body12_e8523) - (assign8540_body12_e8517 * (var_t5_dn10 / var_c_box))) / (assign8540_body12_e8523 * assign8540_body12_e8523))), (var_phi_s0_bulk_dn11 - ((((((-var_phi_s0_bulk_dn11) + (var_t4_dn11 / var_c_box)) - var_vbsbiz_dn11) * assign8540_body12_e8523) - (assign8540_body12_e8517 * (var_t5_dn11 / var_c_box))) / (assign8540_body12_e8523 * assign8540_body12_e8523))), (var_phi_s0_bulk_dn12 - ((((((-var_phi_s0_bulk_dn12) + (var_t4_dn12 / var_c_box)) - var_vbsbiz_dn12) * assign8540_body12_e8523) - (assign8540_body12_e8517 * (var_t5_dn12 / var_c_box))) / (assign8540_body12_e8523 * assign8540_body12_e8523))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
            var_t6 = assign8540_body12_e8527;
            var_t6_dn0 = assign8540_body12_e8527_d_n0;
            var_t6_dn2 = assign8540_body12_e8527_d_n2;
            var_t6_dn4 = assign8540_body12_e8527_d_n4;
            var_t6_dn5 = assign8540_body12_e8527_d_n5;
            var_t6_dn6 = assign8540_body12_e8527_d_n6;
            var_t6_dn8 = assign8540_body12_e8527_d_n8;
            var_t6_dn10 = assign8540_body12_e8527_d_n10;
            var_t6_dn11 = assign8540_body12_e8527_d_n11;
            var_t6_dn12 = assign8540_body12_e8527_d_n12;
            var_t6_rv = 0.0;
            let assign8540_body13_e8530: f64 = (var_t6 - var_phi_s0_bulk);
            let assign8540_body13_e8531: f64 = (assign8540_body13_e8530).abs();
            let assign8540_body13_e8533: f64 = if assign8540_body13_e8531 < 0.001 { 1.0 } else { 0.0 };
            var_guard107 = assign8540_body13_e8533;
            var_guard107_rv = 0.0;
            let (assign8540_body14_e8542, assign8540_body14_e8542_d_n0, assign8540_body14_e8542_d_n2, assign8540_body14_e8542_d_n4, assign8540_body14_e8542_d_n5, assign8540_body14_e8542_d_n6, assign8540_body14_e8542_d_n8, assign8540_body14_e8542_d_n10, assign8540_body14_e8542_d_n11, assign8540_body14_e8542_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 != 0.0)) && (var_guard107 != 0.0)) {
        (var_lp_s0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
            var_t7 = assign8540_body14_e8542;
            var_t7_dn0 = assign8540_body14_e8542_d_n0;
            var_t7_dn2 = assign8540_body14_e8542_d_n2;
            var_t7_dn4 = assign8540_body14_e8542_d_n4;
            var_t7_dn5 = assign8540_body14_e8542_d_n5;
            var_t7_dn6 = assign8540_body14_e8542_d_n6;
            var_t7_dn8 = assign8540_body14_e8542_d_n8;
            var_t7_dn10 = assign8540_body14_e8542_d_n10;
            var_t7_dn11 = assign8540_body14_e8542_d_n11;
            var_t7_dn12 = assign8540_body14_e8542_d_n12;
            var_t7_rv = 0.0;
            let (assign8540_body15_e8551,) = {
    if (((var_guard74 == 0.0) && (var_guard104 != 0.0)) && (var_guard107 != 0.0)) {
        (var_lp_s0_max,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign8540_body15_e8551;
            var_lp_s0_rv = 0.0;
            let (assign8540_body16_e8558, assign8540_body16_e8558_d_n0, assign8540_body16_e8558_d_n2, assign8540_body16_e8558_d_n4, assign8540_body16_e8558_d_n5, assign8540_body16_e8558_d_n6, assign8540_body16_e8558_d_n8, assign8540_body16_e8558_d_n10, assign8540_body16_e8558_d_n11, assign8540_body16_e8558_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 != 0.0)) {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
            var_phi_s0_bulk = assign8540_body16_e8558;
            var_phi_s0_bulk_dn0 = assign8540_body16_e8558_d_n0;
            var_phi_s0_bulk_dn2 = assign8540_body16_e8558_d_n2;
            var_phi_s0_bulk_dn4 = assign8540_body16_e8558_d_n4;
            var_phi_s0_bulk_dn5 = assign8540_body16_e8558_d_n5;
            var_phi_s0_bulk_dn6 = assign8540_body16_e8558_d_n6;
            var_phi_s0_bulk_dn8 = assign8540_body16_e8558_d_n8;
            var_phi_s0_bulk_dn10 = assign8540_body16_e8558_d_n10;
            var_phi_s0_bulk_dn11 = assign8540_body16_e8558_d_n11;
            var_phi_s0_bulk_dn12 = assign8540_body16_e8558_d_n12;
            var_phi_s0_bulk_rv = 0.0;
            let (assign8540_body17_e8565, assign8540_body17_e8565_d_n0, assign8540_body17_e8565_d_n2, assign8540_body17_e8565_d_n4, assign8540_body17_e8565_d_n5, assign8540_body17_e8565_d_n6, assign8540_body17_e8565_d_n8, assign8540_body17_e8565_d_n10, assign8540_body17_e8565_d_n11, assign8540_body17_e8565_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 != 0.0)) {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    } else {
        (var_q_s0_bulk, var_q_s0_bulk_dn0, var_q_s0_bulk_dn2, var_q_s0_bulk_dn4, var_q_s0_bulk_dn5, var_q_s0_bulk_dn6, var_q_s0_bulk_dn8, var_q_s0_bulk_dn10, var_q_s0_bulk_dn11, var_q_s0_bulk_dn12,)
    }
};
            var_q_s0_bulk = assign8540_body17_e8565;
            var_q_s0_bulk_dn0 = assign8540_body17_e8565_d_n0;
            var_q_s0_bulk_dn2 = assign8540_body17_e8565_d_n2;
            var_q_s0_bulk_dn4 = assign8540_body17_e8565_d_n4;
            var_q_s0_bulk_dn5 = assign8540_body17_e8565_d_n5;
            var_q_s0_bulk_dn6 = assign8540_body17_e8565_d_n6;
            var_q_s0_bulk_dn8 = assign8540_body17_e8565_d_n8;
            var_q_s0_bulk_dn10 = assign8540_body17_e8565_d_n10;
            var_q_s0_bulk_dn11 = assign8540_body17_e8565_d_n11;
            var_q_s0_bulk_dn12 = assign8540_body17_e8565_d_n12;
            var_q_s0_bulk_rv = 0.0;
            let (assign8540_body18_e8574,) = {
    if ((var_guard74 == 0.0) && (var_guard104 != 0.0)) {
        let assign8540_body18_e8572: f64 = (var_lp_s0 + 1.0);
        (assign8540_body18_e8572,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign8540_body18_e8574;
            var_lp_s0_rv = 0.0;
        }

        let (assign8550_e8583, assign8550_e8583_d_n0, assign8550_e8583_d_n2, assign8550_e8583_d_n4, assign8550_e8583_d_n5, assign8550_e8583_d_n6, assign8550_e8583_d_n8, assign8550_e8583_d_n10, assign8550_e8583_d_n11, assign8550_e8583_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 != 0.0)) {
        let assign8550_e8581: f64 = (var_vbsbiz + var_phi_s0_bulk);
        (assign8550_e8581, (var_vbsbiz_dn0 + var_phi_s0_bulk_dn0), (var_vbsbiz_dn2 + var_phi_s0_bulk_dn2), (var_vbsbiz_dn4 + var_phi_s0_bulk_dn4), (var_vbsbiz_dn5 + var_phi_s0_bulk_dn5), (var_vbsbiz_dn6 + var_phi_s0_bulk_dn6), (var_vbsbiz_dn8 + var_phi_s0_bulk_dn8), (var_vbsbiz_dn10 + var_phi_s0_bulk_dn10), (var_vbsbiz_dn11 + var_phi_s0_bulk_dn11), (var_vbsbiz_dn12 + var_phi_s0_bulk_dn12),)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
        var_phi_s0_bulk = assign8550_e8583;
        var_phi_s0_bulk_dn0 = assign8550_e8583_d_n0;
        var_phi_s0_bulk_dn2 = assign8550_e8583_d_n2;
        var_phi_s0_bulk_dn4 = assign8550_e8583_d_n4;
        var_phi_s0_bulk_dn5 = assign8550_e8583_d_n5;
        var_phi_s0_bulk_dn6 = assign8550_e8583_d_n6;
        var_phi_s0_bulk_dn8 = assign8550_e8583_d_n8;
        var_phi_s0_bulk_dn10 = assign8550_e8583_d_n10;
        var_phi_s0_bulk_dn11 = assign8550_e8583_d_n11;
        var_phi_s0_bulk_dn12 = assign8550_e8583_d_n12;
        var_phi_s0_bulk_rv = 0.0;

        let (assign8560_e8594, assign8560_e8594_d_n0, assign8560_e8594_d_n2, assign8560_e8594_d_n4, assign8560_e8594_d_n5, assign8560_e8594_d_n6, assign8560_e8594_d_n8, assign8560_e8594_d_n10, assign8560_e8594_d_n11, assign8560_e8594_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 != 0.0)) {
        let assign8560_e8591: f64 = (var_q_s0_bulk / var_c_box);
        let assign8560_e8592: f64 = (var_phi_s0_bulk - assign8560_e8591);
        (assign8560_e8592, (var_phi_s0_bulk_dn0 - (var_q_s0_bulk_dn0 / var_c_box)), (var_phi_s0_bulk_dn2 - (var_q_s0_bulk_dn2 / var_c_box)), (var_phi_s0_bulk_dn4 - (var_q_s0_bulk_dn4 / var_c_box)), (var_phi_s0_bulk_dn5 - (var_q_s0_bulk_dn5 / var_c_box)), (var_phi_s0_bulk_dn6 - (var_q_s0_bulk_dn6 / var_c_box)), (var_phi_s0_bulk_dn8 - (var_q_s0_bulk_dn8 / var_c_box)), (var_phi_s0_bulk_dn10 - (var_q_s0_bulk_dn10 / var_c_box)), (var_phi_s0_bulk_dn11 - (var_q_s0_bulk_dn11 / var_c_box)), (var_phi_s0_bulk_dn12 - (var_q_s0_bulk_dn12 / var_c_box)),)
    } else {
        (var_phi_b0_soi, var_phi_b0_soi_dn0, var_phi_b0_soi_dn2, var_phi_b0_soi_dn4, var_phi_b0_soi_dn5, var_phi_b0_soi_dn6, var_phi_b0_soi_dn8, var_phi_b0_soi_dn10, var_phi_b0_soi_dn11, var_phi_b0_soi_dn12,)
    }
};
        var_phi_b0_soi = assign8560_e8594;
        var_phi_b0_soi_dn0 = assign8560_e8594_d_n0;
        var_phi_b0_soi_dn2 = assign8560_e8594_d_n2;
        var_phi_b0_soi_dn4 = assign8560_e8594_d_n4;
        var_phi_b0_soi_dn5 = assign8560_e8594_d_n5;
        var_phi_b0_soi_dn6 = assign8560_e8594_d_n6;
        var_phi_b0_soi_dn8 = assign8560_e8594_d_n8;
        var_phi_b0_soi_dn10 = assign8560_e8594_d_n10;
        var_phi_b0_soi_dn11 = assign8560_e8594_d_n11;
        var_phi_b0_soi_dn12 = assign8560_e8594_d_n12;
        var_phi_b0_soi_rv = 0.0;

        let (assign8570_e8602,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        (2.0,)
    } else {
        (var_flg_depmode,)
    }
};
        var_flg_depmode = assign8570_e8602;
        var_flg_depmode_rv = 0.0;

        let assign8580_e8605: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        var_guard108 = assign8580_e8605;
        var_guard108_rv = 0.0;

        let (assign8590_e8617,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard108 != 0.0)) {
        let assign8590_e8615: f64 = (1e-12 * 100.0);
        (assign8590_e8615,)
    } else {
        (var_ps_conv_ini,)
    }
};
        var_ps_conv_ini = assign8590_e8617;
        var_ps_conv_ini_rv = 0.0;

        let (assign8600_e8627, assign8600_e8627_d_n0, assign8600_e8627_d_n2, assign8600_e8627_d_n4, assign8600_e8627_d_n5, assign8600_e8627_d_n6, assign8600_e8627_d_n8, assign8600_e8627_d_n10, assign8600_e8627_d_n11, assign8600_e8627_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard108 != 0.0)) {
        (var_ps0_inia, var_ps0_inia_dn0, var_ps0_inia_dn2, var_ps0_inia_dn4, var_ps0_inia_dn5, var_ps0_inia_dn6, var_ps0_inia_dn8, var_ps0_inia_dn10, var_ps0_inia_dn11, var_ps0_inia_dn12,)
    } else {
        (var_ps0, var_ps0_dn0, var_ps0_dn2, var_ps0_dn4, var_ps0_dn5, var_ps0_dn6, var_ps0_dn8, var_ps0_dn10, var_ps0_dn11, var_ps0_dn12,)
    }
};
        var_ps0 = assign8600_e8627;
        var_ps0_dn0 = assign8600_e8627_d_n0;
        var_ps0_dn2 = assign8600_e8627_d_n2;
        var_ps0_dn4 = assign8600_e8627_d_n4;
        var_ps0_dn5 = assign8600_e8627_d_n5;
        var_ps0_dn6 = assign8600_e8627_d_n6;
        var_ps0_dn8 = assign8600_e8627_d_n8;
        var_ps0_dn10 = assign8600_e8627_d_n10;
        var_ps0_dn11 = assign8600_e8627_d_n11;
        var_ps0_dn12 = assign8600_e8627_d_n12;
        var_ps0_rv = 0.0;

        let (assign8610_e8638,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard108 == 0.0)) {
        (0.001,)
    } else {
        (var_ps_conv_ini,)
    }
};
        var_ps_conv_ini = assign8610_e8638;
        var_ps_conv_ini_rv = 0.0;

        let (assign8620_e8649, assign8620_e8649_d_n0, assign8620_e8649_d_n2, assign8620_e8649_d_n4, assign8620_e8649_d_n5, assign8620_e8649_d_n6, assign8620_e8649_d_n8, assign8620_e8649_d_n10, assign8620_e8649_d_n11, assign8620_e8649_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard108 == 0.0)) {
        (var_phi_s0_soi, var_phi_s0_soi_dn0, var_phi_s0_soi_dn2, var_phi_s0_soi_dn4, var_phi_s0_soi_dn5, var_phi_s0_soi_dn6, var_phi_s0_soi_dn8, var_phi_s0_soi_dn10, var_phi_s0_soi_dn11, var_phi_s0_soi_dn12,)
    } else {
        (var_ps0, var_ps0_dn0, var_ps0_dn2, var_ps0_dn4, var_ps0_dn5, var_ps0_dn6, var_ps0_dn8, var_ps0_dn10, var_ps0_dn11, var_ps0_dn12,)
    }
};
        var_ps0 = assign8620_e8649;
        var_ps0_dn0 = assign8620_e8649_d_n0;
        var_ps0_dn2 = assign8620_e8649_d_n2;
        var_ps0_dn4 = assign8620_e8649_d_n4;
        var_ps0_dn5 = assign8620_e8649_d_n5;
        var_ps0_dn6 = assign8620_e8649_d_n6;
        var_ps0_dn8 = assign8620_e8649_d_n8;
        var_ps0_dn10 = assign8620_e8649_d_n10;
        var_ps0_dn11 = assign8620_e8649_d_n11;
        var_ps0_dn12 = assign8620_e8649_d_n12;
        var_ps0_rv = 0.0;

        let (assign8630_e8657,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        (0.0,)
    } else {
        (var_lp_s0,)
    }
};
        var_lp_s0 = assign8630_e8657;
        var_lp_s0_rv = 0.0;

        *var_flg_depmode_slot = var_flg_depmode;
        *var_flg_depmode_rv_slot = var_flg_depmode_rv;
        *var_guard105_slot = var_guard105;
        *var_guard105_rv_slot = var_guard105_rv;
        *var_guard106_slot = var_guard106;
        *var_guard106_rv_slot = var_guard106_rv;
        *var_guard107_slot = var_guard107;
        *var_guard107_rv_slot = var_guard107_rv;
        *var_guard108_slot = var_guard108;
        *var_guard108_rv_slot = var_guard108_rv;
        *var_lp_s0_slot = var_lp_s0;
        *var_lp_s0_rv_slot = var_lp_s0_rv;
        *var_phi_b0_soi_slot = var_phi_b0_soi;
        *var_phi_b0_soi_dn0_slot = var_phi_b0_soi_dn0;
        *var_phi_b0_soi_dn10_slot = var_phi_b0_soi_dn10;
        *var_phi_b0_soi_dn11_slot = var_phi_b0_soi_dn11;
        *var_phi_b0_soi_dn12_slot = var_phi_b0_soi_dn12;
        *var_phi_b0_soi_dn2_slot = var_phi_b0_soi_dn2;
        *var_phi_b0_soi_dn4_slot = var_phi_b0_soi_dn4;
        *var_phi_b0_soi_dn5_slot = var_phi_b0_soi_dn5;
        *var_phi_b0_soi_dn6_slot = var_phi_b0_soi_dn6;
        *var_phi_b0_soi_dn8_slot = var_phi_b0_soi_dn8;
        *var_phi_b0_soi_rv_slot = var_phi_b0_soi_rv;
        *var_phi_s0_bulk_slot = var_phi_s0_bulk;
        *var_phi_s0_bulk_dn0_slot = var_phi_s0_bulk_dn0;
        *var_phi_s0_bulk_dn10_slot = var_phi_s0_bulk_dn10;
        *var_phi_s0_bulk_dn11_slot = var_phi_s0_bulk_dn11;
        *var_phi_s0_bulk_dn12_slot = var_phi_s0_bulk_dn12;
        *var_phi_s0_bulk_dn2_slot = var_phi_s0_bulk_dn2;
        *var_phi_s0_bulk_dn4_slot = var_phi_s0_bulk_dn4;
        *var_phi_s0_bulk_dn5_slot = var_phi_s0_bulk_dn5;
        *var_phi_s0_bulk_dn6_slot = var_phi_s0_bulk_dn6;
        *var_phi_s0_bulk_dn8_slot = var_phi_s0_bulk_dn8;
        *var_phi_s0_bulk_rv_slot = var_phi_s0_bulk_rv;
        *var_ps0_slot = var_ps0;
        *var_ps0_dn0_slot = var_ps0_dn0;
        *var_ps0_dn10_slot = var_ps0_dn10;
        *var_ps0_dn11_slot = var_ps0_dn11;
        *var_ps0_dn12_slot = var_ps0_dn12;
        *var_ps0_dn2_slot = var_ps0_dn2;
        *var_ps0_dn4_slot = var_ps0_dn4;
        *var_ps0_dn5_slot = var_ps0_dn5;
        *var_ps0_dn6_slot = var_ps0_dn6;
        *var_ps0_dn8_slot = var_ps0_dn8;
        *var_ps0_rv_slot = var_ps0_rv;
        *var_ps_conv_ini_slot = var_ps_conv_ini;
        *var_ps_conv_ini_rv_slot = var_ps_conv_ini_rv;
        *var_q_s0_bulk_slot = var_q_s0_bulk;
        *var_q_s0_bulk_dn0_slot = var_q_s0_bulk_dn0;
        *var_q_s0_bulk_dn10_slot = var_q_s0_bulk_dn10;
        *var_q_s0_bulk_dn11_slot = var_q_s0_bulk_dn11;
        *var_q_s0_bulk_dn12_slot = var_q_s0_bulk_dn12;
        *var_q_s0_bulk_dn2_slot = var_q_s0_bulk_dn2;
        *var_q_s0_bulk_dn4_slot = var_q_s0_bulk_dn4;
        *var_q_s0_bulk_dn5_slot = var_q_s0_bulk_dn5;
        *var_q_s0_bulk_dn6_slot = var_q_s0_bulk_dn6;
        *var_q_s0_bulk_dn8_slot = var_q_s0_bulk_dn8;
        *var_q_s0_bulk_rv_slot = var_q_s0_bulk_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn12_slot = var_t6_dn12;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_rv_slot = var_t6_rv;
        *var_t7_slot = var_t7;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn12_slot = var_t7_dn12;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_rv_slot = var_t7_rv;
    }

    pub(super) fn stamp_reactive_block_32(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn4: f64,
        var_c0bulk: f64,
        var_c0bulk_dn0: f64,
        var_c0bulk_dn10: f64,
        var_c0bulk_dn11: f64,
        var_c0bulk_dn12: f64,
        var_c0bulk_dn2: f64,
        var_c0bulk_dn4: f64,
        var_c0bulk_dn5: f64,
        var_c0bulk_dn6: f64,
        var_c0bulk_dn8: f64,
        var_c_box: f64,
        var_cnst0bulk: f64,
        var_cnst0bulk_dn0: f64,
        var_cnst0bulk_dn10: f64,
        var_cnst0bulk_dn11: f64,
        var_cnst0bulk_dn12: f64,
        var_cnst0bulk_dn2: f64,
        var_cnst0bulk_dn4: f64,
        var_cnst0bulk_dn5: f64,
        var_cnst0bulk_dn6: f64,
        var_cnst0bulk_dn8: f64,
        var_cnst1bulk: f64,
        var_cnst1bulk_dn0: f64,
        var_cnst1bulk_dn10: f64,
        var_cnst1bulk_dn11: f64,
        var_cnst1bulk_dn12: f64,
        var_cnst1bulk_dn2: f64,
        var_cnst1bulk_dn4: f64,
        var_cnst1bulk_dn5: f64,
        var_cnst1bulk_dn6: f64,
        var_cnst1bulk_dn8: f64,
        var_guard104: f64,
        var_guard74: f64,
        var_lp_s0_max: f64,
        var_phi_s0_soi: f64,
        var_phi_s0_soi_dn0: f64,
        var_phi_s0_soi_dn10: f64,
        var_phi_s0_soi_dn11: f64,
        var_phi_s0_soi_dn12: f64,
        var_phi_s0_soi_dn2: f64,
        var_phi_s0_soi_dn4: f64,
        var_phi_s0_soi_dn5: f64,
        var_phi_s0_soi_dn6: f64,
        var_phi_s0_soi_dn8: f64,
        var_ps0_inia: f64,
        var_ps0_inia_dn0: f64,
        var_ps0_inia_dn10: f64,
        var_ps0_inia_dn11: f64,
        var_ps0_inia_dn12: f64,
        var_ps0_inia_dn2: f64,
        var_ps0_inia_dn4: f64,
        var_ps0_inia_dn5: f64,
        var_ps0_inia_dn6: f64,
        var_ps0_inia_dn8: f64,
        var_q_fd_soi: f64,
        var_q_fd_soi_dn0: f64,
        var_q_fd_soi_dn10: f64,
        var_q_fd_soi_dn11: f64,
        var_q_fd_soi_dn12: f64,
        var_q_fd_soi_dn2: f64,
        var_q_fd_soi_dn4: f64,
        var_q_fd_soi_dn5: f64,
        var_q_fd_soi_dn6: f64,
        var_q_fd_soi_dn8: f64,
        var_vbsbiz: f64,
        var_vbsbiz_dn0: f64,
        var_vbsbiz_dn10: f64,
        var_vbsbiz_dn11: f64,
        var_vbsbiz_dn12: f64,
        var_vbsbiz_dn2: f64,
        var_vbsbiz_dn4: f64,
        var_vbsbiz_dn5: f64,
        var_vbsbiz_dn6: f64,
        var_vbsbiz_dn8: f64,
        var_guard109_slot: &mut f64,
        var_guard109_rv_slot: &mut f64,
        var_guard110_slot: &mut f64,
        var_guard110_rv_slot: &mut f64,
        var_guard111_slot: &mut f64,
        var_guard111_rv_slot: &mut f64,
        var_guard112_slot: &mut f64,
        var_guard112_rv_slot: &mut f64,
        var_guard113_slot: &mut f64,
        var_guard113_rv_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
        var_lp_s0_rv_slot: &mut f64,
        var_phi_s0_bulk_slot: &mut f64,
        var_phi_s0_bulk_dn0_slot: &mut f64,
        var_phi_s0_bulk_dn10_slot: &mut f64,
        var_phi_s0_bulk_dn11_slot: &mut f64,
        var_phi_s0_bulk_dn12_slot: &mut f64,
        var_phi_s0_bulk_dn2_slot: &mut f64,
        var_phi_s0_bulk_dn4_slot: &mut f64,
        var_phi_s0_bulk_dn5_slot: &mut f64,
        var_phi_s0_bulk_dn6_slot: &mut f64,
        var_phi_s0_bulk_dn8_slot: &mut f64,
        var_phi_s0_bulk_rv_slot: &mut f64,
        var_ps0_slot: &mut f64,
        var_ps0_dn0_slot: &mut f64,
        var_ps0_dn10_slot: &mut f64,
        var_ps0_dn11_slot: &mut f64,
        var_ps0_dn12_slot: &mut f64,
        var_ps0_dn2_slot: &mut f64,
        var_ps0_dn4_slot: &mut f64,
        var_ps0_dn5_slot: &mut f64,
        var_ps0_dn6_slot: &mut f64,
        var_ps0_dn8_slot: &mut f64,
        var_ps0_rv_slot: &mut f64,
        var_ps_conv_ini_slot: &mut f64,
        var_ps_conv_ini_rv_slot: &mut f64,
        var_q_s0_bulk_slot: &mut f64,
        var_q_s0_bulk_dep_slot: &mut f64,
        var_q_s0_bulk_dep_dn0_slot: &mut f64,
        var_q_s0_bulk_dep_dn10_slot: &mut f64,
        var_q_s0_bulk_dep_dn11_slot: &mut f64,
        var_q_s0_bulk_dep_dn12_slot: &mut f64,
        var_q_s0_bulk_dep_dn2_slot: &mut f64,
        var_q_s0_bulk_dep_dn4_slot: &mut f64,
        var_q_s0_bulk_dep_dn5_slot: &mut f64,
        var_q_s0_bulk_dep_dn6_slot: &mut f64,
        var_q_s0_bulk_dep_dn8_slot: &mut f64,
        var_q_s0_bulk_dep_rv_slot: &mut f64,
        var_q_s0_bulk_dn0_slot: &mut f64,
        var_q_s0_bulk_dn10_slot: &mut f64,
        var_q_s0_bulk_dn11_slot: &mut f64,
        var_q_s0_bulk_dn12_slot: &mut f64,
        var_q_s0_bulk_dn2_slot: &mut f64,
        var_q_s0_bulk_dn4_slot: &mut f64,
        var_q_s0_bulk_dn5_slot: &mut f64,
        var_q_s0_bulk_dn6_slot: &mut f64,
        var_q_s0_bulk_dn8_slot: &mut f64,
        var_q_s0_bulk_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn12_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn12_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
    ) {
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard109_rv: f64 = *var_guard109_rv_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard110_rv: f64 = *var_guard110_rv_slot;
        let mut var_guard111: f64 = *var_guard111_slot;
        let mut var_guard111_rv: f64 = *var_guard111_rv_slot;
        let mut var_guard112: f64 = *var_guard112_slot;
        let mut var_guard112_rv: f64 = *var_guard112_rv_slot;
        let mut var_guard113: f64 = *var_guard113_slot;
        let mut var_guard113_rv: f64 = *var_guard113_rv_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
        let mut var_lp_s0_rv: f64 = *var_lp_s0_rv_slot;
        let mut var_phi_s0_bulk: f64 = *var_phi_s0_bulk_slot;
        let mut var_phi_s0_bulk_dn0: f64 = *var_phi_s0_bulk_dn0_slot;
        let mut var_phi_s0_bulk_dn10: f64 = *var_phi_s0_bulk_dn10_slot;
        let mut var_phi_s0_bulk_dn11: f64 = *var_phi_s0_bulk_dn11_slot;
        let mut var_phi_s0_bulk_dn12: f64 = *var_phi_s0_bulk_dn12_slot;
        let mut var_phi_s0_bulk_dn2: f64 = *var_phi_s0_bulk_dn2_slot;
        let mut var_phi_s0_bulk_dn4: f64 = *var_phi_s0_bulk_dn4_slot;
        let mut var_phi_s0_bulk_dn5: f64 = *var_phi_s0_bulk_dn5_slot;
        let mut var_phi_s0_bulk_dn6: f64 = *var_phi_s0_bulk_dn6_slot;
        let mut var_phi_s0_bulk_dn8: f64 = *var_phi_s0_bulk_dn8_slot;
        let mut var_phi_s0_bulk_rv: f64 = *var_phi_s0_bulk_rv_slot;
        let mut var_ps0: f64 = *var_ps0_slot;
        let mut var_ps0_dn0: f64 = *var_ps0_dn0_slot;
        let mut var_ps0_dn10: f64 = *var_ps0_dn10_slot;
        let mut var_ps0_dn11: f64 = *var_ps0_dn11_slot;
        let mut var_ps0_dn12: f64 = *var_ps0_dn12_slot;
        let mut var_ps0_dn2: f64 = *var_ps0_dn2_slot;
        let mut var_ps0_dn4: f64 = *var_ps0_dn4_slot;
        let mut var_ps0_dn5: f64 = *var_ps0_dn5_slot;
        let mut var_ps0_dn6: f64 = *var_ps0_dn6_slot;
        let mut var_ps0_dn8: f64 = *var_ps0_dn8_slot;
        let mut var_ps0_rv: f64 = *var_ps0_rv_slot;
        let mut var_ps_conv_ini: f64 = *var_ps_conv_ini_slot;
        let mut var_ps_conv_ini_rv: f64 = *var_ps_conv_ini_rv_slot;
        let mut var_q_s0_bulk: f64 = *var_q_s0_bulk_slot;
        let mut var_q_s0_bulk_dep: f64 = *var_q_s0_bulk_dep_slot;
        let mut var_q_s0_bulk_dep_dn0: f64 = *var_q_s0_bulk_dep_dn0_slot;
        let mut var_q_s0_bulk_dep_dn10: f64 = *var_q_s0_bulk_dep_dn10_slot;
        let mut var_q_s0_bulk_dep_dn11: f64 = *var_q_s0_bulk_dep_dn11_slot;
        let mut var_q_s0_bulk_dep_dn12: f64 = *var_q_s0_bulk_dep_dn12_slot;
        let mut var_q_s0_bulk_dep_dn2: f64 = *var_q_s0_bulk_dep_dn2_slot;
        let mut var_q_s0_bulk_dep_dn4: f64 = *var_q_s0_bulk_dep_dn4_slot;
        let mut var_q_s0_bulk_dep_dn5: f64 = *var_q_s0_bulk_dep_dn5_slot;
        let mut var_q_s0_bulk_dep_dn6: f64 = *var_q_s0_bulk_dep_dn6_slot;
        let mut var_q_s0_bulk_dep_dn8: f64 = *var_q_s0_bulk_dep_dn8_slot;
        let mut var_q_s0_bulk_dep_rv: f64 = *var_q_s0_bulk_dep_rv_slot;
        let mut var_q_s0_bulk_dn0: f64 = *var_q_s0_bulk_dn0_slot;
        let mut var_q_s0_bulk_dn10: f64 = *var_q_s0_bulk_dn10_slot;
        let mut var_q_s0_bulk_dn11: f64 = *var_q_s0_bulk_dn11_slot;
        let mut var_q_s0_bulk_dn12: f64 = *var_q_s0_bulk_dn12_slot;
        let mut var_q_s0_bulk_dn2: f64 = *var_q_s0_bulk_dn2_slot;
        let mut var_q_s0_bulk_dn4: f64 = *var_q_s0_bulk_dn4_slot;
        let mut var_q_s0_bulk_dn5: f64 = *var_q_s0_bulk_dn5_slot;
        let mut var_q_s0_bulk_dn6: f64 = *var_q_s0_bulk_dn6_slot;
        let mut var_q_s0_bulk_dn8: f64 = *var_q_s0_bulk_dn8_slot;
        let mut var_q_s0_bulk_rv: f64 = *var_q_s0_bulk_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn12: f64 = *var_t6_dn12_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn12: f64 = *var_t7_dn12_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;

        let mut assign8640_loop_guard: usize = 0;
        while {
            let assign8640_cond_e8666: f64 = if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_lp_s0 < var_lp_s0_max)) { 1.0 } else { 0.0 };
            assign8640_cond_e8666 != 0.0
        } {
            assign8640_loop_guard += 1;
            assert!(assign8640_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign8640_body0_e8674, assign8640_body0_e8674_d_n0, assign8640_body0_e8674_d_n2, assign8640_body0_e8674_d_n4, assign8640_body0_e8674_d_n5, assign8640_body0_e8674_d_n6, assign8640_body0_e8674_d_n8, assign8640_body0_e8674_d_n10, assign8640_body0_e8674_d_n11, assign8640_body0_e8674_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        (var_cnst0bulk, var_cnst0bulk_dn0, var_cnst0bulk_dn2, var_cnst0bulk_dn4, var_cnst0bulk_dn5, var_cnst0bulk_dn6, var_cnst0bulk_dn8, var_cnst0bulk_dn10, var_cnst0bulk_dn11, var_cnst0bulk_dn12,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
            var_t1 = assign8640_body0_e8674;
            var_t1_dn0 = assign8640_body0_e8674_d_n0;
            var_t1_dn2 = assign8640_body0_e8674_d_n2;
            var_t1_dn4 = assign8640_body0_e8674_d_n4;
            var_t1_dn5 = assign8640_body0_e8674_d_n5;
            var_t1_dn6 = assign8640_body0_e8674_d_n6;
            var_t1_dn8 = assign8640_body0_e8674_d_n8;
            var_t1_dn10 = assign8640_body0_e8674_d_n10;
            var_t1_dn11 = assign8640_body0_e8674_d_n11;
            var_t1_dn12 = assign8640_body0_e8674_d_n12;
            var_t1_rv = 0.0;
            let (assign8640_body1_e8684, assign8640_body1_e8684_d_n0, assign8640_body1_e8684_d_n2, assign8640_body1_e8684_d_n4, assign8640_body1_e8684_d_n5, assign8640_body1_e8684_d_n6, assign8640_body1_e8684_d_n8, assign8640_body1_e8684_d_n10, assign8640_body1_e8684_d_n11, assign8640_body1_e8684_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        let assign8640_body1_e8682: f64 = (var_beta * var_phi_s0_bulk);
        (assign8640_body1_e8682, (var_beta * var_phi_s0_bulk_dn0), (var_beta * var_phi_s0_bulk_dn2), ((var_beta_dn4 * var_phi_s0_bulk) + (var_beta * var_phi_s0_bulk_dn4)), (var_beta * var_phi_s0_bulk_dn5), (var_beta * var_phi_s0_bulk_dn6), (var_beta * var_phi_s0_bulk_dn8), (var_beta * var_phi_s0_bulk_dn10), (var_beta * var_phi_s0_bulk_dn11), (var_beta * var_phi_s0_bulk_dn12),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
            var_t2 = assign8640_body1_e8684;
            var_t2_dn0 = assign8640_body1_e8684_d_n0;
            var_t2_dn2 = assign8640_body1_e8684_d_n2;
            var_t2_dn4 = assign8640_body1_e8684_d_n4;
            var_t2_dn5 = assign8640_body1_e8684_d_n5;
            var_t2_dn6 = assign8640_body1_e8684_d_n6;
            var_t2_dn8 = assign8640_body1_e8684_d_n8;
            var_t2_dn10 = assign8640_body1_e8684_d_n10;
            var_t2_dn11 = assign8640_body1_e8684_d_n11;
            var_t2_dn12 = assign8640_body1_e8684_d_n12;
            var_t2_rv = 0.0;
            let (assign8640_body2_e8694, assign8640_body2_e8694_d_n0, assign8640_body2_e8694_d_n2, assign8640_body2_e8694_d_n4, assign8640_body2_e8694_d_n5, assign8640_body2_e8694_d_n6, assign8640_body2_e8694_d_n8, assign8640_body2_e8694_d_n10, assign8640_body2_e8694_d_n11, assign8640_body2_e8694_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        let assign8640_body2_e8691: f64 = (-var_t2);
        let assign8640_body2_e8692: f64 = (assign8640_body2_e8691).exp();
        (assign8640_body2_e8692, (assign8640_body2_e8692 * (-var_t2_dn0)), (assign8640_body2_e8692 * (-var_t2_dn2)), (assign8640_body2_e8692 * (-var_t2_dn4)), (assign8640_body2_e8692 * (-var_t2_dn5)), (assign8640_body2_e8692 * (-var_t2_dn6)), (assign8640_body2_e8692 * (-var_t2_dn8)), (assign8640_body2_e8692 * (-var_t2_dn10)), (assign8640_body2_e8692 * (-var_t2_dn11)), (assign8640_body2_e8692 * (-var_t2_dn12)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
            var_t3 = assign8640_body2_e8694;
            var_t3_dn0 = assign8640_body2_e8694_d_n0;
            var_t3_dn2 = assign8640_body2_e8694_d_n2;
            var_t3_dn4 = assign8640_body2_e8694_d_n4;
            var_t3_dn5 = assign8640_body2_e8694_d_n5;
            var_t3_dn6 = assign8640_body2_e8694_d_n6;
            var_t3_dn8 = assign8640_body2_e8694_d_n8;
            var_t3_dn10 = assign8640_body2_e8694_d_n10;
            var_t3_dn11 = assign8640_body2_e8694_d_n11;
            var_t3_dn12 = assign8640_body2_e8694_d_n12;
            var_t3_rv = 0.0;
            let assign8640_body3_e8697: f64 = if var_phi_s0_bulk > 1e-8 { 1.0 } else { 0.0 };
            var_guard109 = assign8640_body3_e8697;
            var_guard109_rv = 0.0;
            let (assign8640_body4_e8710, assign8640_body4_e8710_d_n0, assign8640_body4_e8710_d_n2, assign8640_body4_e8710_d_n4, assign8640_body4_e8710_d_n5, assign8640_body4_e8710_d_n6, assign8640_body4_e8710_d_n8, assign8640_body4_e8710_d_n10, assign8640_body4_e8710_d_n11, assign8640_body4_e8710_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard109 != 0.0)) {
        let assign8640_body4_e8707: f64 = (var_beta * var_phi_s0_bulk);
        let assign8640_body4_e8708: f64 = (assign8640_body4_e8707).exp();
        (assign8640_body4_e8708, (assign8640_body4_e8708 * (var_beta * var_phi_s0_bulk_dn0)), (assign8640_body4_e8708 * (var_beta * var_phi_s0_bulk_dn2)), (assign8640_body4_e8708 * ((var_beta_dn4 * var_phi_s0_bulk) + (var_beta * var_phi_s0_bulk_dn4))), (assign8640_body4_e8708 * (var_beta * var_phi_s0_bulk_dn5)), (assign8640_body4_e8708 * (var_beta * var_phi_s0_bulk_dn6)), (assign8640_body4_e8708 * (var_beta * var_phi_s0_bulk_dn8)), (assign8640_body4_e8708 * (var_beta * var_phi_s0_bulk_dn10)), (assign8640_body4_e8708 * (var_beta * var_phi_s0_bulk_dn11)), (assign8640_body4_e8708 * (var_beta * var_phi_s0_bulk_dn12)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
            var_t0 = assign8640_body4_e8710;
            var_t0_dn0 = assign8640_body4_e8710_d_n0;
            var_t0_dn2 = assign8640_body4_e8710_d_n2;
            var_t0_dn4 = assign8640_body4_e8710_d_n4;
            var_t0_dn5 = assign8640_body4_e8710_d_n5;
            var_t0_dn6 = assign8640_body4_e8710_d_n6;
            var_t0_dn8 = assign8640_body4_e8710_d_n8;
            var_t0_dn10 = assign8640_body4_e8710_d_n10;
            var_t0_dn11 = assign8640_body4_e8710_d_n11;
            var_t0_dn12 = assign8640_body4_e8710_d_n12;
            var_t0_rv = 0.0;
            let (assign8640_body5_e8734, assign8640_body5_e8734_d_n0, assign8640_body5_e8734_d_n2, assign8640_body5_e8734_d_n4, assign8640_body5_e8734_d_n5, assign8640_body5_e8734_d_n6, assign8640_body5_e8734_d_n8, assign8640_body5_e8734_d_n10, assign8640_body5_e8734_d_n11, assign8640_body5_e8734_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard109 != 0.0)) {
        let assign8640_body5_e8719: f64 = (-var_t1);
        let assign8640_body5_e8722: f64 = (var_t3 + var_t2);
        let assign8640_body5_e8724: f64 = (assign8640_body5_e8722 - 1.0);
        let assign8640_body5_e8728: f64 = (var_t0 - 1.0);
        let assign8640_body5_e8729: f64 = (var_cnst1bulk * assign8640_body5_e8728);
        let assign8640_body5_e8730: f64 = (assign8640_body5_e8724 + assign8640_body5_e8729);
        let assign8640_body5_e8731: f64 = (assign8640_body5_e8730).sqrt();
        let assign8640_body5_e8732: f64 = (assign8640_body5_e8719 * assign8640_body5_e8731);
        (assign8640_body5_e8732, (((-var_t1_dn0) * assign8640_body5_e8731) + (assign8640_body5_e8719 * (((var_t3_dn0 + var_t2_dn0) + ((var_cnst1bulk_dn0 * assign8640_body5_e8728) + (var_cnst1bulk * var_t0_dn0))) / (2.0 * assign8640_body5_e8731)))), (((-var_t1_dn2) * assign8640_body5_e8731) + (assign8640_body5_e8719 * (((var_t3_dn2 + var_t2_dn2) + ((var_cnst1bulk_dn2 * assign8640_body5_e8728) + (var_cnst1bulk * var_t0_dn2))) / (2.0 * assign8640_body5_e8731)))), (((-var_t1_dn4) * assign8640_body5_e8731) + (assign8640_body5_e8719 * (((var_t3_dn4 + var_t2_dn4) + ((var_cnst1bulk_dn4 * assign8640_body5_e8728) + (var_cnst1bulk * var_t0_dn4))) / (2.0 * assign8640_body5_e8731)))), (((-var_t1_dn5) * assign8640_body5_e8731) + (assign8640_body5_e8719 * (((var_t3_dn5 + var_t2_dn5) + ((var_cnst1bulk_dn5 * assign8640_body5_e8728) + (var_cnst1bulk * var_t0_dn5))) / (2.0 * assign8640_body5_e8731)))), (((-var_t1_dn6) * assign8640_body5_e8731) + (assign8640_body5_e8719 * (((var_t3_dn6 + var_t2_dn6) + ((var_cnst1bulk_dn6 * assign8640_body5_e8728) + (var_cnst1bulk * var_t0_dn6))) / (2.0 * assign8640_body5_e8731)))), (((-var_t1_dn8) * assign8640_body5_e8731) + (assign8640_body5_e8719 * (((var_t3_dn8 + var_t2_dn8) + ((var_cnst1bulk_dn8 * assign8640_body5_e8728) + (var_cnst1bulk * var_t0_dn8))) / (2.0 * assign8640_body5_e8731)))), (((-var_t1_dn10) * assign8640_body5_e8731) + (assign8640_body5_e8719 * (((var_t3_dn10 + var_t2_dn10) + ((var_cnst1bulk_dn10 * assign8640_body5_e8728) + (var_cnst1bulk * var_t0_dn10))) / (2.0 * assign8640_body5_e8731)))), (((-var_t1_dn11) * assign8640_body5_e8731) + (assign8640_body5_e8719 * (((var_t3_dn11 + var_t2_dn11) + ((var_cnst1bulk_dn11 * assign8640_body5_e8728) + (var_cnst1bulk * var_t0_dn11))) / (2.0 * assign8640_body5_e8731)))), (((-var_t1_dn12) * assign8640_body5_e8731) + (assign8640_body5_e8719 * (((var_t3_dn12 + var_t2_dn12) + ((var_cnst1bulk_dn12 * assign8640_body5_e8728) + (var_cnst1bulk * var_t0_dn12))) / (2.0 * assign8640_body5_e8731)))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign8640_body5_e8734;
            var_t4_dn0 = assign8640_body5_e8734_d_n0;
            var_t4_dn2 = assign8640_body5_e8734_d_n2;
            var_t4_dn4 = assign8640_body5_e8734_d_n4;
            var_t4_dn5 = assign8640_body5_e8734_d_n5;
            var_t4_dn6 = assign8640_body5_e8734_d_n6;
            var_t4_dn8 = assign8640_body5_e8734_d_n8;
            var_t4_dn10 = assign8640_body5_e8734_d_n10;
            var_t4_dn11 = assign8640_body5_e8734_d_n11;
            var_t4_dn12 = assign8640_body5_e8734_d_n12;
            var_t4_rv = 0.0;
            let (assign8640_body6_e8755, assign8640_body6_e8755_d_n0, assign8640_body6_e8755_d_n2, assign8640_body6_e8755_d_n4, assign8640_body6_e8755_d_n5, assign8640_body6_e8755_d_n6, assign8640_body6_e8755_d_n8, assign8640_body6_e8755_d_n10, assign8640_body6_e8755_d_n11, assign8640_body6_e8755_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard109 != 0.0)) {
        let assign8640_body6_e8744: f64 = (var_c0bulk / var_t4);
        let assign8640_body6_e8746: f64 = (-var_t3);
        let assign8640_body6_e8748: f64 = (assign8640_body6_e8746 + 1.0);
        let assign8640_body6_e8751: f64 = (var_cnst1bulk * var_t0);
        let assign8640_body6_e8752: f64 = (assign8640_body6_e8748 + assign8640_body6_e8751);
        let assign8640_body6_e8753: f64 = (assign8640_body6_e8744 * assign8640_body6_e8752);
        (assign8640_body6_e8753, (((((var_c0bulk_dn0 * var_t4) - (var_c0bulk * var_t4_dn0)) / (var_t4 * var_t4)) * assign8640_body6_e8752) + (assign8640_body6_e8744 * ((-var_t3_dn0) + ((var_cnst1bulk_dn0 * var_t0) + (var_cnst1bulk * var_t0_dn0))))), (((((var_c0bulk_dn2 * var_t4) - (var_c0bulk * var_t4_dn2)) / (var_t4 * var_t4)) * assign8640_body6_e8752) + (assign8640_body6_e8744 * ((-var_t3_dn2) + ((var_cnst1bulk_dn2 * var_t0) + (var_cnst1bulk * var_t0_dn2))))), (((((var_c0bulk_dn4 * var_t4) - (var_c0bulk * var_t4_dn4)) / (var_t4 * var_t4)) * assign8640_body6_e8752) + (assign8640_body6_e8744 * ((-var_t3_dn4) + ((var_cnst1bulk_dn4 * var_t0) + (var_cnst1bulk * var_t0_dn4))))), (((((var_c0bulk_dn5 * var_t4) - (var_c0bulk * var_t4_dn5)) / (var_t4 * var_t4)) * assign8640_body6_e8752) + (assign8640_body6_e8744 * ((-var_t3_dn5) + ((var_cnst1bulk_dn5 * var_t0) + (var_cnst1bulk * var_t0_dn5))))), (((((var_c0bulk_dn6 * var_t4) - (var_c0bulk * var_t4_dn6)) / (var_t4 * var_t4)) * assign8640_body6_e8752) + (assign8640_body6_e8744 * ((-var_t3_dn6) + ((var_cnst1bulk_dn6 * var_t0) + (var_cnst1bulk * var_t0_dn6))))), (((((var_c0bulk_dn8 * var_t4) - (var_c0bulk * var_t4_dn8)) / (var_t4 * var_t4)) * assign8640_body6_e8752) + (assign8640_body6_e8744 * ((-var_t3_dn8) + ((var_cnst1bulk_dn8 * var_t0) + (var_cnst1bulk * var_t0_dn8))))), (((((var_c0bulk_dn10 * var_t4) - (var_c0bulk * var_t4_dn10)) / (var_t4 * var_t4)) * assign8640_body6_e8752) + (assign8640_body6_e8744 * ((-var_t3_dn10) + ((var_cnst1bulk_dn10 * var_t0) + (var_cnst1bulk * var_t0_dn10))))), (((((var_c0bulk_dn11 * var_t4) - (var_c0bulk * var_t4_dn11)) / (var_t4 * var_t4)) * assign8640_body6_e8752) + (assign8640_body6_e8744 * ((-var_t3_dn11) + ((var_cnst1bulk_dn11 * var_t0) + (var_cnst1bulk * var_t0_dn11))))), (((((var_c0bulk_dn12 * var_t4) - (var_c0bulk * var_t4_dn12)) / (var_t4 * var_t4)) * assign8640_body6_e8752) + (assign8640_body6_e8744 * ((-var_t3_dn12) + ((var_cnst1bulk_dn12 * var_t0) + (var_cnst1bulk * var_t0_dn12))))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign8640_body6_e8755;
            var_t5_dn0 = assign8640_body6_e8755_d_n0;
            var_t5_dn2 = assign8640_body6_e8755_d_n2;
            var_t5_dn4 = assign8640_body6_e8755_d_n4;
            var_t5_dn5 = assign8640_body6_e8755_d_n5;
            var_t5_dn6 = assign8640_body6_e8755_d_n6;
            var_t5_dn8 = assign8640_body6_e8755_d_n8;
            var_t5_dn10 = assign8640_body6_e8755_d_n10;
            var_t5_dn11 = assign8640_body6_e8755_d_n11;
            var_t5_dn12 = assign8640_body6_e8755_d_n12;
            var_t5_rv = 0.0;
            let assign8640_body7_e8758: f64 = (-1e-8);
            let assign8640_body7_e8759: f64 = if var_phi_s0_bulk < assign8640_body7_e8758 { 1.0 } else { 0.0 };
            var_guard110 = assign8640_body7_e8759;
            var_guard110_rv = 0.0;
            let (assign8640_body8_e8779, assign8640_body8_e8779_d_n0, assign8640_body8_e8779_d_n2, assign8640_body8_e8779_d_n4, assign8640_body8_e8779_d_n5, assign8640_body8_e8779_d_n6, assign8640_body8_e8779_d_n8, assign8640_body8_e8779_d_n10, assign8640_body8_e8779_d_n11, assign8640_body8_e8779_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard109 == 0.0)) && (var_guard110 != 0.0)) {
        let assign8640_body8_e8773: f64 = (var_t3 + var_t2);
        let assign8640_body8_e8775: f64 = (assign8640_body8_e8773 - 1.0);
        let assign8640_body8_e8776: f64 = (assign8640_body8_e8775).sqrt();
        let assign8640_body8_e8777: f64 = (var_t1 * assign8640_body8_e8776);
        (assign8640_body8_e8777, ((var_t1_dn0 * assign8640_body8_e8776) + (var_t1 * ((var_t3_dn0 + var_t2_dn0) / (2.0 * assign8640_body8_e8776)))), ((var_t1_dn2 * assign8640_body8_e8776) + (var_t1 * ((var_t3_dn2 + var_t2_dn2) / (2.0 * assign8640_body8_e8776)))), ((var_t1_dn4 * assign8640_body8_e8776) + (var_t1 * ((var_t3_dn4 + var_t2_dn4) / (2.0 * assign8640_body8_e8776)))), ((var_t1_dn5 * assign8640_body8_e8776) + (var_t1 * ((var_t3_dn5 + var_t2_dn5) / (2.0 * assign8640_body8_e8776)))), ((var_t1_dn6 * assign8640_body8_e8776) + (var_t1 * ((var_t3_dn6 + var_t2_dn6) / (2.0 * assign8640_body8_e8776)))), ((var_t1_dn8 * assign8640_body8_e8776) + (var_t1 * ((var_t3_dn8 + var_t2_dn8) / (2.0 * assign8640_body8_e8776)))), ((var_t1_dn10 * assign8640_body8_e8776) + (var_t1 * ((var_t3_dn10 + var_t2_dn10) / (2.0 * assign8640_body8_e8776)))), ((var_t1_dn11 * assign8640_body8_e8776) + (var_t1 * ((var_t3_dn11 + var_t2_dn11) / (2.0 * assign8640_body8_e8776)))), ((var_t1_dn12 * assign8640_body8_e8776) + (var_t1 * ((var_t3_dn12 + var_t2_dn12) / (2.0 * assign8640_body8_e8776)))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign8640_body8_e8779;
            var_t4_dn0 = assign8640_body8_e8779_d_n0;
            var_t4_dn2 = assign8640_body8_e8779_d_n2;
            var_t4_dn4 = assign8640_body8_e8779_d_n4;
            var_t4_dn5 = assign8640_body8_e8779_d_n5;
            var_t4_dn6 = assign8640_body8_e8779_d_n6;
            var_t4_dn8 = assign8640_body8_e8779_d_n8;
            var_t4_dn10 = assign8640_body8_e8779_d_n10;
            var_t4_dn11 = assign8640_body8_e8779_d_n11;
            var_t4_dn12 = assign8640_body8_e8779_d_n12;
            var_t4_rv = 0.0;
            let (assign8640_body9_e8799, assign8640_body9_e8799_d_n0, assign8640_body9_e8799_d_n2, assign8640_body9_e8799_d_n4, assign8640_body9_e8799_d_n5, assign8640_body9_e8799_d_n6, assign8640_body9_e8799_d_n8, assign8640_body9_e8799_d_n10, assign8640_body9_e8799_d_n11, assign8640_body9_e8799_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard109 == 0.0)) && (var_guard110 != 0.0)) {
        let assign8640_body9_e8792: f64 = (var_c0bulk / var_t4);
        let assign8640_body9_e8794: f64 = (-var_t3);
        let assign8640_body9_e8796: f64 = (assign8640_body9_e8794 + 1.0);
        let assign8640_body9_e8797: f64 = (assign8640_body9_e8792 * assign8640_body9_e8796);
        (assign8640_body9_e8797, (((((var_c0bulk_dn0 * var_t4) - (var_c0bulk * var_t4_dn0)) / (var_t4 * var_t4)) * assign8640_body9_e8796) + (assign8640_body9_e8792 * (-var_t3_dn0))), (((((var_c0bulk_dn2 * var_t4) - (var_c0bulk * var_t4_dn2)) / (var_t4 * var_t4)) * assign8640_body9_e8796) + (assign8640_body9_e8792 * (-var_t3_dn2))), (((((var_c0bulk_dn4 * var_t4) - (var_c0bulk * var_t4_dn4)) / (var_t4 * var_t4)) * assign8640_body9_e8796) + (assign8640_body9_e8792 * (-var_t3_dn4))), (((((var_c0bulk_dn5 * var_t4) - (var_c0bulk * var_t4_dn5)) / (var_t4 * var_t4)) * assign8640_body9_e8796) + (assign8640_body9_e8792 * (-var_t3_dn5))), (((((var_c0bulk_dn6 * var_t4) - (var_c0bulk * var_t4_dn6)) / (var_t4 * var_t4)) * assign8640_body9_e8796) + (assign8640_body9_e8792 * (-var_t3_dn6))), (((((var_c0bulk_dn8 * var_t4) - (var_c0bulk * var_t4_dn8)) / (var_t4 * var_t4)) * assign8640_body9_e8796) + (assign8640_body9_e8792 * (-var_t3_dn8))), (((((var_c0bulk_dn10 * var_t4) - (var_c0bulk * var_t4_dn10)) / (var_t4 * var_t4)) * assign8640_body9_e8796) + (assign8640_body9_e8792 * (-var_t3_dn10))), (((((var_c0bulk_dn11 * var_t4) - (var_c0bulk * var_t4_dn11)) / (var_t4 * var_t4)) * assign8640_body9_e8796) + (assign8640_body9_e8792 * (-var_t3_dn11))), (((((var_c0bulk_dn12 * var_t4) - (var_c0bulk * var_t4_dn12)) / (var_t4 * var_t4)) * assign8640_body9_e8796) + (assign8640_body9_e8792 * (-var_t3_dn12))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign8640_body9_e8799;
            var_t5_dn0 = assign8640_body9_e8799_d_n0;
            var_t5_dn2 = assign8640_body9_e8799_d_n2;
            var_t5_dn4 = assign8640_body9_e8799_d_n4;
            var_t5_dn5 = assign8640_body9_e8799_d_n5;
            var_t5_dn6 = assign8640_body9_e8799_d_n6;
            var_t5_dn8 = assign8640_body9_e8799_d_n8;
            var_t5_dn10 = assign8640_body9_e8799_d_n10;
            var_t5_dn11 = assign8640_body9_e8799_d_n11;
            var_t5_dn12 = assign8640_body9_e8799_d_n12;
            var_t5_rv = 0.0;
            let (assign8640_body10_e8821, assign8640_body10_e8821_d_n0, assign8640_body10_e8821_d_n2, assign8640_body10_e8821_d_n4, assign8640_body10_e8821_d_n5, assign8640_body10_e8821_d_n6, assign8640_body10_e8821_d_n8, assign8640_body10_e8821_d_n10, assign8640_body10_e8821_d_n11, assign8640_body10_e8821_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard109 == 0.0)) && (var_guard110 == 0.0)) {
        let assign8640_body10_e8813: f64 = (var_c0bulk / var_beta);
        let assign8640_body10_e8814: f64 = (assign8640_body10_e8813).sqrt();
        let assign8640_body10_e8815: f64 = (-assign8640_body10_e8814);
        let assign8640_body10_e8817: f64 = (assign8640_body10_e8815 * var_beta);
        let assign8640_body10_e8819: f64 = (assign8640_body10_e8817 * var_phi_s0_bulk);
        (assign8640_body10_e8819, ((((-((var_c0bulk_dn0 / var_beta) / (2.0 * assign8640_body10_e8814))) * var_beta) * var_phi_s0_bulk) + (assign8640_body10_e8817 * var_phi_s0_bulk_dn0)), ((((-((var_c0bulk_dn2 / var_beta) / (2.0 * assign8640_body10_e8814))) * var_beta) * var_phi_s0_bulk) + (assign8640_body10_e8817 * var_phi_s0_bulk_dn2)), (((((-((((var_c0bulk_dn4 * var_beta) - (var_c0bulk * var_beta_dn4)) / (var_beta * var_beta)) / (2.0 * assign8640_body10_e8814))) * var_beta) + (assign8640_body10_e8815 * var_beta_dn4)) * var_phi_s0_bulk) + (assign8640_body10_e8817 * var_phi_s0_bulk_dn4)), ((((-((var_c0bulk_dn5 / var_beta) / (2.0 * assign8640_body10_e8814))) * var_beta) * var_phi_s0_bulk) + (assign8640_body10_e8817 * var_phi_s0_bulk_dn5)), ((((-((var_c0bulk_dn6 / var_beta) / (2.0 * assign8640_body10_e8814))) * var_beta) * var_phi_s0_bulk) + (assign8640_body10_e8817 * var_phi_s0_bulk_dn6)), ((((-((var_c0bulk_dn8 / var_beta) / (2.0 * assign8640_body10_e8814))) * var_beta) * var_phi_s0_bulk) + (assign8640_body10_e8817 * var_phi_s0_bulk_dn8)), ((((-((var_c0bulk_dn10 / var_beta) / (2.0 * assign8640_body10_e8814))) * var_beta) * var_phi_s0_bulk) + (assign8640_body10_e8817 * var_phi_s0_bulk_dn10)), ((((-((var_c0bulk_dn11 / var_beta) / (2.0 * assign8640_body10_e8814))) * var_beta) * var_phi_s0_bulk) + (assign8640_body10_e8817 * var_phi_s0_bulk_dn11)), ((((-((var_c0bulk_dn12 / var_beta) / (2.0 * assign8640_body10_e8814))) * var_beta) * var_phi_s0_bulk) + (assign8640_body10_e8817 * var_phi_s0_bulk_dn12)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign8640_body10_e8821;
            var_t4_dn0 = assign8640_body10_e8821_d_n0;
            var_t4_dn2 = assign8640_body10_e8821_d_n2;
            var_t4_dn4 = assign8640_body10_e8821_d_n4;
            var_t4_dn5 = assign8640_body10_e8821_d_n5;
            var_t4_dn6 = assign8640_body10_e8821_d_n6;
            var_t4_dn8 = assign8640_body10_e8821_d_n8;
            var_t4_dn10 = assign8640_body10_e8821_d_n10;
            var_t4_dn11 = assign8640_body10_e8821_d_n11;
            var_t4_dn12 = assign8640_body10_e8821_d_n12;
            var_t4_rv = 0.0;
            let (assign8640_body11_e8839, assign8640_body11_e8839_d_n0, assign8640_body11_e8839_d_n2, assign8640_body11_e8839_d_n4, assign8640_body11_e8839_d_n5, assign8640_body11_e8839_d_n6, assign8640_body11_e8839_d_n8, assign8640_body11_e8839_d_n10, assign8640_body11_e8839_d_n11, assign8640_body11_e8839_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard109 == 0.0)) && (var_guard110 == 0.0)) {
        let assign8640_body11_e8835: f64 = (var_c0bulk * var_beta);
        let assign8640_body11_e8836: f64 = (assign8640_body11_e8835).sqrt();
        let assign8640_body11_e8837: f64 = (-assign8640_body11_e8836);
        (assign8640_body11_e8837, (-((var_c0bulk_dn0 * var_beta) / (2.0 * assign8640_body11_e8836))), (-((var_c0bulk_dn2 * var_beta) / (2.0 * assign8640_body11_e8836))), (-(((var_c0bulk_dn4 * var_beta) + (var_c0bulk * var_beta_dn4)) / (2.0 * assign8640_body11_e8836))), (-((var_c0bulk_dn5 * var_beta) / (2.0 * assign8640_body11_e8836))), (-((var_c0bulk_dn6 * var_beta) / (2.0 * assign8640_body11_e8836))), (-((var_c0bulk_dn8 * var_beta) / (2.0 * assign8640_body11_e8836))), (-((var_c0bulk_dn10 * var_beta) / (2.0 * assign8640_body11_e8836))), (-((var_c0bulk_dn11 * var_beta) / (2.0 * assign8640_body11_e8836))), (-((var_c0bulk_dn12 * var_beta) / (2.0 * assign8640_body11_e8836))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign8640_body11_e8839;
            var_t5_dn0 = assign8640_body11_e8839_d_n0;
            var_t5_dn2 = assign8640_body11_e8839_d_n2;
            var_t5_dn4 = assign8640_body11_e8839_d_n4;
            var_t5_dn5 = assign8640_body11_e8839_d_n5;
            var_t5_dn6 = assign8640_body11_e8839_d_n6;
            var_t5_dn8 = assign8640_body11_e8839_d_n8;
            var_t5_dn10 = assign8640_body11_e8839_d_n10;
            var_t5_dn11 = assign8640_body11_e8839_d_n11;
            var_t5_dn12 = assign8640_body11_e8839_d_n12;
            var_t5_rv = 0.0;
            let (assign8640_body12_e8880, assign8640_body12_e8880_d_n0, assign8640_body12_e8880_d_n2, assign8640_body12_e8880_d_n4, assign8640_body12_e8880_d_n5, assign8640_body12_e8880_d_n6, assign8640_body12_e8880_d_n8, assign8640_body12_e8880_d_n10, assign8640_body12_e8880_d_n11, assign8640_body12_e8880_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        let assign8640_body12_e8848: f64 = (var_ps0 - var_phi_s0_bulk);
        let assign8640_body12_e8851: f64 = (var_t4 / var_c_box);
        let assign8640_body12_e8852: f64 = (assign8640_body12_e8848 + assign8640_body12_e8851);
        let assign8640_body12_e8856: f64 = (var_q_fd_soi / 2.0);
        let assign8640_body12_e8857: f64 = (var_t4 + assign8640_body12_e8856);
        let assign8640_body12_e8859: f64 = (assign8640_body12_e8857 * p.p227);
        let assign8640_body12_e8861: f64 = (assign8640_body12_e8859 / 1.034943e-10);
        let assign8640_body12_e8862: f64 = (assign8640_body12_e8852 + assign8640_body12_e8861);
        let assign8640_body12_e8864: f64 = (assign8640_body12_e8862 - var_vbsbiz);
        let assign8640_body12_e8866: f64 = (-1.0);
        let assign8640_body12_e8869: f64 = (var_t5 / var_c_box);
        let assign8640_body12_e8870: f64 = (assign8640_body12_e8866 + assign8640_body12_e8869);
        let assign8640_body12_e8873: f64 = (var_t5 * p.p227);
        let assign8640_body12_e8875: f64 = (assign8640_body12_e8873 / 1.034943e-10);
        let assign8640_body12_e8876: f64 = (assign8640_body12_e8870 + assign8640_body12_e8875);
        let assign8640_body12_e8877: f64 = (assign8640_body12_e8864 / assign8640_body12_e8876);
        let assign8640_body12_e8878: f64 = (var_phi_s0_bulk - assign8640_body12_e8877);
        (assign8640_body12_e8878, (var_phi_s0_bulk_dn0 - (((((((var_ps0_dn0 - var_phi_s0_bulk_dn0) + (var_t4_dn0 / var_c_box)) + (((var_t4_dn0 + (var_q_fd_soi_dn0 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn0) * assign8640_body12_e8876) - (assign8640_body12_e8864 * ((var_t5_dn0 / var_c_box) + ((var_t5_dn0 * p.p227) / 1.034943e-10)))) / (assign8640_body12_e8876 * assign8640_body12_e8876))), (var_phi_s0_bulk_dn2 - (((((((var_ps0_dn2 - var_phi_s0_bulk_dn2) + (var_t4_dn2 / var_c_box)) + (((var_t4_dn2 + (var_q_fd_soi_dn2 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn2) * assign8640_body12_e8876) - (assign8640_body12_e8864 * ((var_t5_dn2 / var_c_box) + ((var_t5_dn2 * p.p227) / 1.034943e-10)))) / (assign8640_body12_e8876 * assign8640_body12_e8876))), (var_phi_s0_bulk_dn4 - (((((((var_ps0_dn4 - var_phi_s0_bulk_dn4) + (var_t4_dn4 / var_c_box)) + (((var_t4_dn4 + (var_q_fd_soi_dn4 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn4) * assign8640_body12_e8876) - (assign8640_body12_e8864 * ((var_t5_dn4 / var_c_box) + ((var_t5_dn4 * p.p227) / 1.034943e-10)))) / (assign8640_body12_e8876 * assign8640_body12_e8876))), (var_phi_s0_bulk_dn5 - (((((((var_ps0_dn5 - var_phi_s0_bulk_dn5) + (var_t4_dn5 / var_c_box)) + (((var_t4_dn5 + (var_q_fd_soi_dn5 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn5) * assign8640_body12_e8876) - (assign8640_body12_e8864 * ((var_t5_dn5 / var_c_box) + ((var_t5_dn5 * p.p227) / 1.034943e-10)))) / (assign8640_body12_e8876 * assign8640_body12_e8876))), (var_phi_s0_bulk_dn6 - (((((((var_ps0_dn6 - var_phi_s0_bulk_dn6) + (var_t4_dn6 / var_c_box)) + (((var_t4_dn6 + (var_q_fd_soi_dn6 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn6) * assign8640_body12_e8876) - (assign8640_body12_e8864 * ((var_t5_dn6 / var_c_box) + ((var_t5_dn6 * p.p227) / 1.034943e-10)))) / (assign8640_body12_e8876 * assign8640_body12_e8876))), (var_phi_s0_bulk_dn8 - (((((((var_ps0_dn8 - var_phi_s0_bulk_dn8) + (var_t4_dn8 / var_c_box)) + (((var_t4_dn8 + (var_q_fd_soi_dn8 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn8) * assign8640_body12_e8876) - (assign8640_body12_e8864 * ((var_t5_dn8 / var_c_box) + ((var_t5_dn8 * p.p227) / 1.034943e-10)))) / (assign8640_body12_e8876 * assign8640_body12_e8876))), (var_phi_s0_bulk_dn10 - (((((((var_ps0_dn10 - var_phi_s0_bulk_dn10) + (var_t4_dn10 / var_c_box)) + (((var_t4_dn10 + (var_q_fd_soi_dn10 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn10) * assign8640_body12_e8876) - (assign8640_body12_e8864 * ((var_t5_dn10 / var_c_box) + ((var_t5_dn10 * p.p227) / 1.034943e-10)))) / (assign8640_body12_e8876 * assign8640_body12_e8876))), (var_phi_s0_bulk_dn11 - (((((((var_ps0_dn11 - var_phi_s0_bulk_dn11) + (var_t4_dn11 / var_c_box)) + (((var_t4_dn11 + (var_q_fd_soi_dn11 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn11) * assign8640_body12_e8876) - (assign8640_body12_e8864 * ((var_t5_dn11 / var_c_box) + ((var_t5_dn11 * p.p227) / 1.034943e-10)))) / (assign8640_body12_e8876 * assign8640_body12_e8876))), (var_phi_s0_bulk_dn12 - (((((((var_ps0_dn12 - var_phi_s0_bulk_dn12) + (var_t4_dn12 / var_c_box)) + (((var_t4_dn12 + (var_q_fd_soi_dn12 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn12) * assign8640_body12_e8876) - (assign8640_body12_e8864 * ((var_t5_dn12 / var_c_box) + ((var_t5_dn12 * p.p227) / 1.034943e-10)))) / (assign8640_body12_e8876 * assign8640_body12_e8876))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
            var_t6 = assign8640_body12_e8880;
            var_t6_dn0 = assign8640_body12_e8880_d_n0;
            var_t6_dn2 = assign8640_body12_e8880_d_n2;
            var_t6_dn4 = assign8640_body12_e8880_d_n4;
            var_t6_dn5 = assign8640_body12_e8880_d_n5;
            var_t6_dn6 = assign8640_body12_e8880_d_n6;
            var_t6_dn8 = assign8640_body12_e8880_d_n8;
            var_t6_dn10 = assign8640_body12_e8880_d_n10;
            var_t6_dn11 = assign8640_body12_e8880_d_n11;
            var_t6_dn12 = assign8640_body12_e8880_d_n12;
            var_t6_rv = 0.0;
            let assign8640_body13_e8883: f64 = (var_t6 - var_phi_s0_bulk);
            let assign8640_body13_e8884: f64 = (assign8640_body13_e8883).abs();
            let assign8640_body13_e8886: f64 = if assign8640_body13_e8884 < var_ps_conv_ini { 1.0 } else { 0.0 };
            var_guard111 = assign8640_body13_e8886;
            var_guard111_rv = 0.0;
            let (assign8640_body14_e8896, assign8640_body14_e8896_d_n0, assign8640_body14_e8896_d_n2, assign8640_body14_e8896_d_n4, assign8640_body14_e8896_d_n5, assign8640_body14_e8896_d_n6, assign8640_body14_e8896_d_n8, assign8640_body14_e8896_d_n10, assign8640_body14_e8896_d_n11, assign8640_body14_e8896_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard111 != 0.0)) {
        (var_lp_s0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
            var_t7 = assign8640_body14_e8896;
            var_t7_dn0 = assign8640_body14_e8896_d_n0;
            var_t7_dn2 = assign8640_body14_e8896_d_n2;
            var_t7_dn4 = assign8640_body14_e8896_d_n4;
            var_t7_dn5 = assign8640_body14_e8896_d_n5;
            var_t7_dn6 = assign8640_body14_e8896_d_n6;
            var_t7_dn8 = assign8640_body14_e8896_d_n8;
            var_t7_dn10 = assign8640_body14_e8896_d_n10;
            var_t7_dn11 = assign8640_body14_e8896_d_n11;
            var_t7_dn12 = assign8640_body14_e8896_d_n12;
            var_t7_rv = 0.0;
            let (assign8640_body15_e8906,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard111 != 0.0)) {
        (var_lp_s0_max,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign8640_body15_e8906;
            var_lp_s0_rv = 0.0;
            let (assign8640_body16_e8914, assign8640_body16_e8914_d_n0, assign8640_body16_e8914_d_n2, assign8640_body16_e8914_d_n4, assign8640_body16_e8914_d_n5, assign8640_body16_e8914_d_n6, assign8640_body16_e8914_d_n8, assign8640_body16_e8914_d_n10, assign8640_body16_e8914_d_n11, assign8640_body16_e8914_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
            var_phi_s0_bulk = assign8640_body16_e8914;
            var_phi_s0_bulk_dn0 = assign8640_body16_e8914_d_n0;
            var_phi_s0_bulk_dn2 = assign8640_body16_e8914_d_n2;
            var_phi_s0_bulk_dn4 = assign8640_body16_e8914_d_n4;
            var_phi_s0_bulk_dn5 = assign8640_body16_e8914_d_n5;
            var_phi_s0_bulk_dn6 = assign8640_body16_e8914_d_n6;
            var_phi_s0_bulk_dn8 = assign8640_body16_e8914_d_n8;
            var_phi_s0_bulk_dn10 = assign8640_body16_e8914_d_n10;
            var_phi_s0_bulk_dn11 = assign8640_body16_e8914_d_n11;
            var_phi_s0_bulk_dn12 = assign8640_body16_e8914_d_n12;
            var_phi_s0_bulk_rv = 0.0;
            let (assign8640_body17_e8922, assign8640_body17_e8922_d_n0, assign8640_body17_e8922_d_n2, assign8640_body17_e8922_d_n4, assign8640_body17_e8922_d_n5, assign8640_body17_e8922_d_n6, assign8640_body17_e8922_d_n8, assign8640_body17_e8922_d_n10, assign8640_body17_e8922_d_n11, assign8640_body17_e8922_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    } else {
        (var_q_s0_bulk, var_q_s0_bulk_dn0, var_q_s0_bulk_dn2, var_q_s0_bulk_dn4, var_q_s0_bulk_dn5, var_q_s0_bulk_dn6, var_q_s0_bulk_dn8, var_q_s0_bulk_dn10, var_q_s0_bulk_dn11, var_q_s0_bulk_dn12,)
    }
};
            var_q_s0_bulk = assign8640_body17_e8922;
            var_q_s0_bulk_dn0 = assign8640_body17_e8922_d_n0;
            var_q_s0_bulk_dn2 = assign8640_body17_e8922_d_n2;
            var_q_s0_bulk_dn4 = assign8640_body17_e8922_d_n4;
            var_q_s0_bulk_dn5 = assign8640_body17_e8922_d_n5;
            var_q_s0_bulk_dn6 = assign8640_body17_e8922_d_n6;
            var_q_s0_bulk_dn8 = assign8640_body17_e8922_d_n8;
            var_q_s0_bulk_dn10 = assign8640_body17_e8922_d_n10;
            var_q_s0_bulk_dn11 = assign8640_body17_e8922_d_n11;
            var_q_s0_bulk_dn12 = assign8640_body17_e8922_d_n12;
            var_q_s0_bulk_rv = 0.0;
            let (assign8640_body18_e8932,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        let assign8640_body18_e8930: f64 = (var_lp_s0 + 1.0);
        (assign8640_body18_e8930,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign8640_body18_e8932;
            var_lp_s0_rv = 0.0;
        }

        let assign8650_e8935: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        var_guard112 = assign8650_e8935;
        var_guard112_rv = 0.0;

        let (assign8660_e8945, assign8660_e8945_d_n0, assign8660_e8945_d_n2, assign8660_e8945_d_n4, assign8660_e8945_d_n5, assign8660_e8945_d_n6, assign8660_e8945_d_n8, assign8660_e8945_d_n10, assign8660_e8945_d_n11, assign8660_e8945_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard112 != 0.0)) {
        (var_q_s0_bulk, var_q_s0_bulk_dn0, var_q_s0_bulk_dn2, var_q_s0_bulk_dn4, var_q_s0_bulk_dn5, var_q_s0_bulk_dn6, var_q_s0_bulk_dn8, var_q_s0_bulk_dn10, var_q_s0_bulk_dn11, var_q_s0_bulk_dn12,)
    } else {
        (var_q_s0_bulk_dep, var_q_s0_bulk_dep_dn0, var_q_s0_bulk_dep_dn2, var_q_s0_bulk_dep_dn4, var_q_s0_bulk_dep_dn5, var_q_s0_bulk_dep_dn6, var_q_s0_bulk_dep_dn8, var_q_s0_bulk_dep_dn10, var_q_s0_bulk_dep_dn11, var_q_s0_bulk_dep_dn12,)
    }
};
        var_q_s0_bulk_dep = assign8660_e8945;
        var_q_s0_bulk_dep_dn0 = assign8660_e8945_d_n0;
        var_q_s0_bulk_dep_dn2 = assign8660_e8945_d_n2;
        var_q_s0_bulk_dep_dn4 = assign8660_e8945_d_n4;
        var_q_s0_bulk_dep_dn5 = assign8660_e8945_d_n5;
        var_q_s0_bulk_dep_dn6 = assign8660_e8945_d_n6;
        var_q_s0_bulk_dep_dn8 = assign8660_e8945_d_n8;
        var_q_s0_bulk_dep_dn10 = assign8660_e8945_d_n10;
        var_q_s0_bulk_dep_dn11 = assign8660_e8945_d_n11;
        var_q_s0_bulk_dep_dn12 = assign8660_e8945_d_n12;
        var_q_s0_bulk_dep_rv = 0.0;

        let assign8670_e8948: f64 = if 1.0 == 0.0 { 1.0 } else { 0.0 };
        var_guard113 = assign8670_e8948;
        var_guard113_rv = 0.0;

        let (assign8680_e8960,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard113 != 0.0)) {
        let assign8680_e8958: f64 = (1e-12 * 100.0);
        (assign8680_e8958,)
    } else {
        (var_ps_conv_ini,)
    }
};
        var_ps_conv_ini = assign8680_e8960;
        var_ps_conv_ini_rv = 0.0;

        let (assign8690_e8970, assign8690_e8970_d_n0, assign8690_e8970_d_n2, assign8690_e8970_d_n4, assign8690_e8970_d_n5, assign8690_e8970_d_n6, assign8690_e8970_d_n8, assign8690_e8970_d_n10, assign8690_e8970_d_n11, assign8690_e8970_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard113 != 0.0)) {
        (var_ps0_inia, var_ps0_inia_dn0, var_ps0_inia_dn2, var_ps0_inia_dn4, var_ps0_inia_dn5, var_ps0_inia_dn6, var_ps0_inia_dn8, var_ps0_inia_dn10, var_ps0_inia_dn11, var_ps0_inia_dn12,)
    } else {
        (var_ps0, var_ps0_dn0, var_ps0_dn2, var_ps0_dn4, var_ps0_dn5, var_ps0_dn6, var_ps0_dn8, var_ps0_dn10, var_ps0_dn11, var_ps0_dn12,)
    }
};
        var_ps0 = assign8690_e8970;
        var_ps0_dn0 = assign8690_e8970_d_n0;
        var_ps0_dn2 = assign8690_e8970_d_n2;
        var_ps0_dn4 = assign8690_e8970_d_n4;
        var_ps0_dn5 = assign8690_e8970_d_n5;
        var_ps0_dn6 = assign8690_e8970_d_n6;
        var_ps0_dn8 = assign8690_e8970_d_n8;
        var_ps0_dn10 = assign8690_e8970_d_n10;
        var_ps0_dn11 = assign8690_e8970_d_n11;
        var_ps0_dn12 = assign8690_e8970_d_n12;
        var_ps0_rv = 0.0;

        let (assign8700_e8981,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard113 == 0.0)) {
        (0.001,)
    } else {
        (var_ps_conv_ini,)
    }
};
        var_ps_conv_ini = assign8700_e8981;
        var_ps_conv_ini_rv = 0.0;

        let (assign8710_e8992, assign8710_e8992_d_n0, assign8710_e8992_d_n2, assign8710_e8992_d_n4, assign8710_e8992_d_n5, assign8710_e8992_d_n6, assign8710_e8992_d_n8, assign8710_e8992_d_n10, assign8710_e8992_d_n11, assign8710_e8992_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard113 == 0.0)) {
        (var_phi_s0_soi, var_phi_s0_soi_dn0, var_phi_s0_soi_dn2, var_phi_s0_soi_dn4, var_phi_s0_soi_dn5, var_phi_s0_soi_dn6, var_phi_s0_soi_dn8, var_phi_s0_soi_dn10, var_phi_s0_soi_dn11, var_phi_s0_soi_dn12,)
    } else {
        (var_ps0, var_ps0_dn0, var_ps0_dn2, var_ps0_dn4, var_ps0_dn5, var_ps0_dn6, var_ps0_dn8, var_ps0_dn10, var_ps0_dn11, var_ps0_dn12,)
    }
};
        var_ps0 = assign8710_e8992;
        var_ps0_dn0 = assign8710_e8992_d_n0;
        var_ps0_dn2 = assign8710_e8992_d_n2;
        var_ps0_dn4 = assign8710_e8992_d_n4;
        var_ps0_dn5 = assign8710_e8992_d_n5;
        var_ps0_dn6 = assign8710_e8992_d_n6;
        var_ps0_dn8 = assign8710_e8992_d_n8;
        var_ps0_dn10 = assign8710_e8992_d_n10;
        var_ps0_dn11 = assign8710_e8992_d_n11;
        var_ps0_dn12 = assign8710_e8992_d_n12;
        var_ps0_rv = 0.0;

        let (assign8720_e9000,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        (0.0,)
    } else {
        (var_lp_s0,)
    }
};
        var_lp_s0 = assign8720_e9000;
        var_lp_s0_rv = 0.0;

        *var_guard109_slot = var_guard109;
        *var_guard109_rv_slot = var_guard109_rv;
        *var_guard110_slot = var_guard110;
        *var_guard110_rv_slot = var_guard110_rv;
        *var_guard111_slot = var_guard111;
        *var_guard111_rv_slot = var_guard111_rv;
        *var_guard112_slot = var_guard112;
        *var_guard112_rv_slot = var_guard112_rv;
        *var_guard113_slot = var_guard113;
        *var_guard113_rv_slot = var_guard113_rv;
        *var_lp_s0_slot = var_lp_s0;
        *var_lp_s0_rv_slot = var_lp_s0_rv;
        *var_phi_s0_bulk_slot = var_phi_s0_bulk;
        *var_phi_s0_bulk_dn0_slot = var_phi_s0_bulk_dn0;
        *var_phi_s0_bulk_dn10_slot = var_phi_s0_bulk_dn10;
        *var_phi_s0_bulk_dn11_slot = var_phi_s0_bulk_dn11;
        *var_phi_s0_bulk_dn12_slot = var_phi_s0_bulk_dn12;
        *var_phi_s0_bulk_dn2_slot = var_phi_s0_bulk_dn2;
        *var_phi_s0_bulk_dn4_slot = var_phi_s0_bulk_dn4;
        *var_phi_s0_bulk_dn5_slot = var_phi_s0_bulk_dn5;
        *var_phi_s0_bulk_dn6_slot = var_phi_s0_bulk_dn6;
        *var_phi_s0_bulk_dn8_slot = var_phi_s0_bulk_dn8;
        *var_phi_s0_bulk_rv_slot = var_phi_s0_bulk_rv;
        *var_ps0_slot = var_ps0;
        *var_ps0_dn0_slot = var_ps0_dn0;
        *var_ps0_dn10_slot = var_ps0_dn10;
        *var_ps0_dn11_slot = var_ps0_dn11;
        *var_ps0_dn12_slot = var_ps0_dn12;
        *var_ps0_dn2_slot = var_ps0_dn2;
        *var_ps0_dn4_slot = var_ps0_dn4;
        *var_ps0_dn5_slot = var_ps0_dn5;
        *var_ps0_dn6_slot = var_ps0_dn6;
        *var_ps0_dn8_slot = var_ps0_dn8;
        *var_ps0_rv_slot = var_ps0_rv;
        *var_ps_conv_ini_slot = var_ps_conv_ini;
        *var_ps_conv_ini_rv_slot = var_ps_conv_ini_rv;
        *var_q_s0_bulk_slot = var_q_s0_bulk;
        *var_q_s0_bulk_dep_slot = var_q_s0_bulk_dep;
        *var_q_s0_bulk_dep_dn0_slot = var_q_s0_bulk_dep_dn0;
        *var_q_s0_bulk_dep_dn10_slot = var_q_s0_bulk_dep_dn10;
        *var_q_s0_bulk_dep_dn11_slot = var_q_s0_bulk_dep_dn11;
        *var_q_s0_bulk_dep_dn12_slot = var_q_s0_bulk_dep_dn12;
        *var_q_s0_bulk_dep_dn2_slot = var_q_s0_bulk_dep_dn2;
        *var_q_s0_bulk_dep_dn4_slot = var_q_s0_bulk_dep_dn4;
        *var_q_s0_bulk_dep_dn5_slot = var_q_s0_bulk_dep_dn5;
        *var_q_s0_bulk_dep_dn6_slot = var_q_s0_bulk_dep_dn6;
        *var_q_s0_bulk_dep_dn8_slot = var_q_s0_bulk_dep_dn8;
        *var_q_s0_bulk_dep_rv_slot = var_q_s0_bulk_dep_rv;
        *var_q_s0_bulk_dn0_slot = var_q_s0_bulk_dn0;
        *var_q_s0_bulk_dn10_slot = var_q_s0_bulk_dn10;
        *var_q_s0_bulk_dn11_slot = var_q_s0_bulk_dn11;
        *var_q_s0_bulk_dn12_slot = var_q_s0_bulk_dn12;
        *var_q_s0_bulk_dn2_slot = var_q_s0_bulk_dn2;
        *var_q_s0_bulk_dn4_slot = var_q_s0_bulk_dn4;
        *var_q_s0_bulk_dn5_slot = var_q_s0_bulk_dn5;
        *var_q_s0_bulk_dn6_slot = var_q_s0_bulk_dn6;
        *var_q_s0_bulk_dn8_slot = var_q_s0_bulk_dn8;
        *var_q_s0_bulk_rv_slot = var_q_s0_bulk_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn12_slot = var_t6_dn12;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_rv_slot = var_t6_rv;
        *var_t7_slot = var_t7;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn12_slot = var_t7_dn12;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_rv_slot = var_t7_rv;
    }

    pub(super) fn stamp_reactive_block_33(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn4: f64,
        var_c0bulk: f64,
        var_c0bulk_dn0: f64,
        var_c0bulk_dn10: f64,
        var_c0bulk_dn11: f64,
        var_c0bulk_dn12: f64,
        var_c0bulk_dn2: f64,
        var_c0bulk_dn4: f64,
        var_c0bulk_dn5: f64,
        var_c0bulk_dn6: f64,
        var_c0bulk_dn8: f64,
        var_c_box: f64,
        var_cnst0bulk: f64,
        var_cnst0bulk_dn0: f64,
        var_cnst0bulk_dn10: f64,
        var_cnst0bulk_dn11: f64,
        var_cnst0bulk_dn12: f64,
        var_cnst0bulk_dn2: f64,
        var_cnst0bulk_dn4: f64,
        var_cnst0bulk_dn5: f64,
        var_cnst0bulk_dn6: f64,
        var_cnst0bulk_dn8: f64,
        var_cnst1bulk: f64,
        var_cnst1bulk_dn0: f64,
        var_cnst1bulk_dn10: f64,
        var_cnst1bulk_dn11: f64,
        var_cnst1bulk_dn12: f64,
        var_cnst1bulk_dn2: f64,
        var_cnst1bulk_dn4: f64,
        var_cnst1bulk_dn5: f64,
        var_cnst1bulk_dn6: f64,
        var_cnst1bulk_dn8: f64,
        var_guard104: f64,
        var_guard74: f64,
        var_lp_s0_max: f64,
        var_phi_s0_soi: f64,
        var_phi_s0_soi_dn0: f64,
        var_phi_s0_soi_dn10: f64,
        var_phi_s0_soi_dn11: f64,
        var_phi_s0_soi_dn12: f64,
        var_phi_s0_soi_dn2: f64,
        var_phi_s0_soi_dn4: f64,
        var_phi_s0_soi_dn5: f64,
        var_phi_s0_soi_dn6: f64,
        var_phi_s0_soi_dn8: f64,
        var_ps0: f64,
        var_ps0_dn0: f64,
        var_ps0_dn10: f64,
        var_ps0_dn11: f64,
        var_ps0_dn12: f64,
        var_ps0_dn2: f64,
        var_ps0_dn4: f64,
        var_ps0_dn5: f64,
        var_ps0_dn6: f64,
        var_ps0_dn8: f64,
        var_ps_conv_ini: f64,
        var_q_fd_soi: f64,
        var_q_fd_soi_dn0: f64,
        var_q_fd_soi_dn10: f64,
        var_q_fd_soi_dn11: f64,
        var_q_fd_soi_dn12: f64,
        var_q_fd_soi_dn2: f64,
        var_q_fd_soi_dn4: f64,
        var_q_fd_soi_dn5: f64,
        var_q_fd_soi_dn6: f64,
        var_q_fd_soi_dn8: f64,
        var_vbsbiz: f64,
        var_vbsbiz_dn0: f64,
        var_vbsbiz_dn10: f64,
        var_vbsbiz_dn11: f64,
        var_vbsbiz_dn12: f64,
        var_vbsbiz_dn2: f64,
        var_vbsbiz_dn4: f64,
        var_vbsbiz_dn5: f64,
        var_vbsbiz_dn6: f64,
        var_vbsbiz_dn8: f64,
        var_guard114_slot: &mut f64,
        var_guard114_rv_slot: &mut f64,
        var_guard115_slot: &mut f64,
        var_guard115_rv_slot: &mut f64,
        var_guard116_slot: &mut f64,
        var_guard116_rv_slot: &mut f64,
        var_guard117_slot: &mut f64,
        var_guard117_rv_slot: &mut f64,
        var_guard118_slot: &mut f64,
        var_guard118_rv_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
        var_lp_s0_rv_slot: &mut f64,
        var_lp_sl_slot: &mut f64,
        var_lp_sl_rv_slot: &mut f64,
        var_phi_b0_soi_slot: &mut f64,
        var_phi_b0_soi_dn0_slot: &mut f64,
        var_phi_b0_soi_dn10_slot: &mut f64,
        var_phi_b0_soi_dn11_slot: &mut f64,
        var_phi_b0_soi_dn12_slot: &mut f64,
        var_phi_b0_soi_dn2_slot: &mut f64,
        var_phi_b0_soi_dn4_slot: &mut f64,
        var_phi_b0_soi_dn5_slot: &mut f64,
        var_phi_b0_soi_dn6_slot: &mut f64,
        var_phi_b0_soi_dn8_slot: &mut f64,
        var_phi_b0_soi_rv_slot: &mut f64,
        var_phi_s0_bulk_slot: &mut f64,
        var_phi_s0_bulk_dn0_slot: &mut f64,
        var_phi_s0_bulk_dn10_slot: &mut f64,
        var_phi_s0_bulk_dn11_slot: &mut f64,
        var_phi_s0_bulk_dn12_slot: &mut f64,
        var_phi_s0_bulk_dn2_slot: &mut f64,
        var_phi_s0_bulk_dn4_slot: &mut f64,
        var_phi_s0_bulk_dn5_slot: &mut f64,
        var_phi_s0_bulk_dn6_slot: &mut f64,
        var_phi_s0_bulk_dn8_slot: &mut f64,
        var_phi_s0_bulk_rv_slot: &mut f64,
        var_q_s0_bulk_slot: &mut f64,
        var_q_s0_bulk_dep_slot: &mut f64,
        var_q_s0_bulk_dep_dn0_slot: &mut f64,
        var_q_s0_bulk_dep_dn10_slot: &mut f64,
        var_q_s0_bulk_dep_dn11_slot: &mut f64,
        var_q_s0_bulk_dep_dn12_slot: &mut f64,
        var_q_s0_bulk_dep_dn2_slot: &mut f64,
        var_q_s0_bulk_dep_dn4_slot: &mut f64,
        var_q_s0_bulk_dep_dn5_slot: &mut f64,
        var_q_s0_bulk_dep_dn6_slot: &mut f64,
        var_q_s0_bulk_dep_dn8_slot: &mut f64,
        var_q_s0_bulk_dep_rv_slot: &mut f64,
        var_q_s0_bulk_dn0_slot: &mut f64,
        var_q_s0_bulk_dn10_slot: &mut f64,
        var_q_s0_bulk_dn11_slot: &mut f64,
        var_q_s0_bulk_dn12_slot: &mut f64,
        var_q_s0_bulk_dn2_slot: &mut f64,
        var_q_s0_bulk_dn4_slot: &mut f64,
        var_q_s0_bulk_dn5_slot: &mut f64,
        var_q_s0_bulk_dn6_slot: &mut f64,
        var_q_s0_bulk_dn8_slot: &mut f64,
        var_q_s0_bulk_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn12_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn12_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_x2_slot: &mut f64,
        var_x2_dn0_slot: &mut f64,
        var_x2_dn10_slot: &mut f64,
        var_x2_dn11_slot: &mut f64,
        var_x2_dn12_slot: &mut f64,
        var_x2_dn2_slot: &mut f64,
        var_x2_dn4_slot: &mut f64,
        var_x2_dn5_slot: &mut f64,
        var_x2_dn6_slot: &mut f64,
        var_x2_dn8_slot: &mut f64,
        var_x2_rv_slot: &mut f64,
        var_xmax2_slot: &mut f64,
        var_xmax2_dn0_slot: &mut f64,
        var_xmax2_dn10_slot: &mut f64,
        var_xmax2_dn11_slot: &mut f64,
        var_xmax2_dn12_slot: &mut f64,
        var_xmax2_dn2_slot: &mut f64,
        var_xmax2_dn4_slot: &mut f64,
        var_xmax2_dn5_slot: &mut f64,
        var_xmax2_dn6_slot: &mut f64,
        var_xmax2_dn8_slot: &mut f64,
        var_xmax2_rv_slot: &mut f64,
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn11_slot: &mut f64,
        var_xp_dn12_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn4_slot: &mut f64,
        var_xp_dn5_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn8_slot: &mut f64,
        var_xp_rv_slot: &mut f64,
    ) {
        let mut var_guard114: f64 = *var_guard114_slot;
        let mut var_guard114_rv: f64 = *var_guard114_rv_slot;
        let mut var_guard115: f64 = *var_guard115_slot;
        let mut var_guard115_rv: f64 = *var_guard115_rv_slot;
        let mut var_guard116: f64 = *var_guard116_slot;
        let mut var_guard116_rv: f64 = *var_guard116_rv_slot;
        let mut var_guard117: f64 = *var_guard117_slot;
        let mut var_guard117_rv: f64 = *var_guard117_rv_slot;
        let mut var_guard118: f64 = *var_guard118_slot;
        let mut var_guard118_rv: f64 = *var_guard118_rv_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
        let mut var_lp_s0_rv: f64 = *var_lp_s0_rv_slot;
        let mut var_lp_sl: f64 = *var_lp_sl_slot;
        let mut var_lp_sl_rv: f64 = *var_lp_sl_rv_slot;
        let mut var_phi_b0_soi: f64 = *var_phi_b0_soi_slot;
        let mut var_phi_b0_soi_dn0: f64 = *var_phi_b0_soi_dn0_slot;
        let mut var_phi_b0_soi_dn10: f64 = *var_phi_b0_soi_dn10_slot;
        let mut var_phi_b0_soi_dn11: f64 = *var_phi_b0_soi_dn11_slot;
        let mut var_phi_b0_soi_dn12: f64 = *var_phi_b0_soi_dn12_slot;
        let mut var_phi_b0_soi_dn2: f64 = *var_phi_b0_soi_dn2_slot;
        let mut var_phi_b0_soi_dn4: f64 = *var_phi_b0_soi_dn4_slot;
        let mut var_phi_b0_soi_dn5: f64 = *var_phi_b0_soi_dn5_slot;
        let mut var_phi_b0_soi_dn6: f64 = *var_phi_b0_soi_dn6_slot;
        let mut var_phi_b0_soi_dn8: f64 = *var_phi_b0_soi_dn8_slot;
        let mut var_phi_b0_soi_rv: f64 = *var_phi_b0_soi_rv_slot;
        let mut var_phi_s0_bulk: f64 = *var_phi_s0_bulk_slot;
        let mut var_phi_s0_bulk_dn0: f64 = *var_phi_s0_bulk_dn0_slot;
        let mut var_phi_s0_bulk_dn10: f64 = *var_phi_s0_bulk_dn10_slot;
        let mut var_phi_s0_bulk_dn11: f64 = *var_phi_s0_bulk_dn11_slot;
        let mut var_phi_s0_bulk_dn12: f64 = *var_phi_s0_bulk_dn12_slot;
        let mut var_phi_s0_bulk_dn2: f64 = *var_phi_s0_bulk_dn2_slot;
        let mut var_phi_s0_bulk_dn4: f64 = *var_phi_s0_bulk_dn4_slot;
        let mut var_phi_s0_bulk_dn5: f64 = *var_phi_s0_bulk_dn5_slot;
        let mut var_phi_s0_bulk_dn6: f64 = *var_phi_s0_bulk_dn6_slot;
        let mut var_phi_s0_bulk_dn8: f64 = *var_phi_s0_bulk_dn8_slot;
        let mut var_phi_s0_bulk_rv: f64 = *var_phi_s0_bulk_rv_slot;
        let mut var_q_s0_bulk: f64 = *var_q_s0_bulk_slot;
        let mut var_q_s0_bulk_dep: f64 = *var_q_s0_bulk_dep_slot;
        let mut var_q_s0_bulk_dep_dn0: f64 = *var_q_s0_bulk_dep_dn0_slot;
        let mut var_q_s0_bulk_dep_dn10: f64 = *var_q_s0_bulk_dep_dn10_slot;
        let mut var_q_s0_bulk_dep_dn11: f64 = *var_q_s0_bulk_dep_dn11_slot;
        let mut var_q_s0_bulk_dep_dn12: f64 = *var_q_s0_bulk_dep_dn12_slot;
        let mut var_q_s0_bulk_dep_dn2: f64 = *var_q_s0_bulk_dep_dn2_slot;
        let mut var_q_s0_bulk_dep_dn4: f64 = *var_q_s0_bulk_dep_dn4_slot;
        let mut var_q_s0_bulk_dep_dn5: f64 = *var_q_s0_bulk_dep_dn5_slot;
        let mut var_q_s0_bulk_dep_dn6: f64 = *var_q_s0_bulk_dep_dn6_slot;
        let mut var_q_s0_bulk_dep_dn8: f64 = *var_q_s0_bulk_dep_dn8_slot;
        let mut var_q_s0_bulk_dep_rv: f64 = *var_q_s0_bulk_dep_rv_slot;
        let mut var_q_s0_bulk_dn0: f64 = *var_q_s0_bulk_dn0_slot;
        let mut var_q_s0_bulk_dn10: f64 = *var_q_s0_bulk_dn10_slot;
        let mut var_q_s0_bulk_dn11: f64 = *var_q_s0_bulk_dn11_slot;
        let mut var_q_s0_bulk_dn12: f64 = *var_q_s0_bulk_dn12_slot;
        let mut var_q_s0_bulk_dn2: f64 = *var_q_s0_bulk_dn2_slot;
        let mut var_q_s0_bulk_dn4: f64 = *var_q_s0_bulk_dn4_slot;
        let mut var_q_s0_bulk_dn5: f64 = *var_q_s0_bulk_dn5_slot;
        let mut var_q_s0_bulk_dn6: f64 = *var_q_s0_bulk_dn6_slot;
        let mut var_q_s0_bulk_dn8: f64 = *var_q_s0_bulk_dn8_slot;
        let mut var_q_s0_bulk_rv: f64 = *var_q_s0_bulk_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn12: f64 = *var_t6_dn12_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn12: f64 = *var_t7_dn12_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_x2: f64 = *var_x2_slot;
        let mut var_x2_dn0: f64 = *var_x2_dn0_slot;
        let mut var_x2_dn10: f64 = *var_x2_dn10_slot;
        let mut var_x2_dn11: f64 = *var_x2_dn11_slot;
        let mut var_x2_dn12: f64 = *var_x2_dn12_slot;
        let mut var_x2_dn2: f64 = *var_x2_dn2_slot;
        let mut var_x2_dn4: f64 = *var_x2_dn4_slot;
        let mut var_x2_dn5: f64 = *var_x2_dn5_slot;
        let mut var_x2_dn6: f64 = *var_x2_dn6_slot;
        let mut var_x2_dn8: f64 = *var_x2_dn8_slot;
        let mut var_x2_rv: f64 = *var_x2_rv_slot;
        let mut var_xmax2: f64 = *var_xmax2_slot;
        let mut var_xmax2_dn0: f64 = *var_xmax2_dn0_slot;
        let mut var_xmax2_dn10: f64 = *var_xmax2_dn10_slot;
        let mut var_xmax2_dn11: f64 = *var_xmax2_dn11_slot;
        let mut var_xmax2_dn12: f64 = *var_xmax2_dn12_slot;
        let mut var_xmax2_dn2: f64 = *var_xmax2_dn2_slot;
        let mut var_xmax2_dn4: f64 = *var_xmax2_dn4_slot;
        let mut var_xmax2_dn5: f64 = *var_xmax2_dn5_slot;
        let mut var_xmax2_dn6: f64 = *var_xmax2_dn6_slot;
        let mut var_xmax2_dn8: f64 = *var_xmax2_dn8_slot;
        let mut var_xmax2_rv: f64 = *var_xmax2_rv_slot;
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn11: f64 = *var_xp_dn11_slot;
        let mut var_xp_dn12: f64 = *var_xp_dn12_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn4: f64 = *var_xp_dn4_slot;
        let mut var_xp_dn5: f64 = *var_xp_dn5_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn8: f64 = *var_xp_dn8_slot;
        let mut var_xp_rv: f64 = *var_xp_rv_slot;

        let mut assign8730_loop_guard: usize = 0;
        while {
            let assign8730_cond_e9009: f64 = if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_lp_s0 < var_lp_s0_max)) { 1.0 } else { 0.0 };
            assign8730_cond_e9009 != 0.0
        } {
            assign8730_loop_guard += 1;
            assert!(assign8730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign8730_body0_e9017, assign8730_body0_e9017_d_n0, assign8730_body0_e9017_d_n2, assign8730_body0_e9017_d_n4, assign8730_body0_e9017_d_n5, assign8730_body0_e9017_d_n6, assign8730_body0_e9017_d_n8, assign8730_body0_e9017_d_n10, assign8730_body0_e9017_d_n11, assign8730_body0_e9017_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        (var_cnst0bulk, var_cnst0bulk_dn0, var_cnst0bulk_dn2, var_cnst0bulk_dn4, var_cnst0bulk_dn5, var_cnst0bulk_dn6, var_cnst0bulk_dn8, var_cnst0bulk_dn10, var_cnst0bulk_dn11, var_cnst0bulk_dn12,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
            var_t1 = assign8730_body0_e9017;
            var_t1_dn0 = assign8730_body0_e9017_d_n0;
            var_t1_dn2 = assign8730_body0_e9017_d_n2;
            var_t1_dn4 = assign8730_body0_e9017_d_n4;
            var_t1_dn5 = assign8730_body0_e9017_d_n5;
            var_t1_dn6 = assign8730_body0_e9017_d_n6;
            var_t1_dn8 = assign8730_body0_e9017_d_n8;
            var_t1_dn10 = assign8730_body0_e9017_d_n10;
            var_t1_dn11 = assign8730_body0_e9017_d_n11;
            var_t1_dn12 = assign8730_body0_e9017_d_n12;
            var_t1_rv = 0.0;
            let (assign8730_body1_e9027, assign8730_body1_e9027_d_n0, assign8730_body1_e9027_d_n2, assign8730_body1_e9027_d_n4, assign8730_body1_e9027_d_n5, assign8730_body1_e9027_d_n6, assign8730_body1_e9027_d_n8, assign8730_body1_e9027_d_n10, assign8730_body1_e9027_d_n11, assign8730_body1_e9027_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        let assign8730_body1_e9025: f64 = (var_beta * var_phi_s0_bulk);
        (assign8730_body1_e9025, (var_beta * var_phi_s0_bulk_dn0), (var_beta * var_phi_s0_bulk_dn2), ((var_beta_dn4 * var_phi_s0_bulk) + (var_beta * var_phi_s0_bulk_dn4)), (var_beta * var_phi_s0_bulk_dn5), (var_beta * var_phi_s0_bulk_dn6), (var_beta * var_phi_s0_bulk_dn8), (var_beta * var_phi_s0_bulk_dn10), (var_beta * var_phi_s0_bulk_dn11), (var_beta * var_phi_s0_bulk_dn12),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
            var_t2 = assign8730_body1_e9027;
            var_t2_dn0 = assign8730_body1_e9027_d_n0;
            var_t2_dn2 = assign8730_body1_e9027_d_n2;
            var_t2_dn4 = assign8730_body1_e9027_d_n4;
            var_t2_dn5 = assign8730_body1_e9027_d_n5;
            var_t2_dn6 = assign8730_body1_e9027_d_n6;
            var_t2_dn8 = assign8730_body1_e9027_d_n8;
            var_t2_dn10 = assign8730_body1_e9027_d_n10;
            var_t2_dn11 = assign8730_body1_e9027_d_n11;
            var_t2_dn12 = assign8730_body1_e9027_d_n12;
            var_t2_rv = 0.0;
            let (assign8730_body2_e9037, assign8730_body2_e9037_d_n0, assign8730_body2_e9037_d_n2, assign8730_body2_e9037_d_n4, assign8730_body2_e9037_d_n5, assign8730_body2_e9037_d_n6, assign8730_body2_e9037_d_n8, assign8730_body2_e9037_d_n10, assign8730_body2_e9037_d_n11, assign8730_body2_e9037_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        let assign8730_body2_e9034: f64 = (-var_t2);
        let assign8730_body2_e9035: f64 = (assign8730_body2_e9034).exp();
        (assign8730_body2_e9035, (assign8730_body2_e9035 * (-var_t2_dn0)), (assign8730_body2_e9035 * (-var_t2_dn2)), (assign8730_body2_e9035 * (-var_t2_dn4)), (assign8730_body2_e9035 * (-var_t2_dn5)), (assign8730_body2_e9035 * (-var_t2_dn6)), (assign8730_body2_e9035 * (-var_t2_dn8)), (assign8730_body2_e9035 * (-var_t2_dn10)), (assign8730_body2_e9035 * (-var_t2_dn11)), (assign8730_body2_e9035 * (-var_t2_dn12)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
            var_t3 = assign8730_body2_e9037;
            var_t3_dn0 = assign8730_body2_e9037_d_n0;
            var_t3_dn2 = assign8730_body2_e9037_d_n2;
            var_t3_dn4 = assign8730_body2_e9037_d_n4;
            var_t3_dn5 = assign8730_body2_e9037_d_n5;
            var_t3_dn6 = assign8730_body2_e9037_d_n6;
            var_t3_dn8 = assign8730_body2_e9037_d_n8;
            var_t3_dn10 = assign8730_body2_e9037_d_n10;
            var_t3_dn11 = assign8730_body2_e9037_d_n11;
            var_t3_dn12 = assign8730_body2_e9037_d_n12;
            var_t3_rv = 0.0;
            let assign8730_body3_e9040: f64 = if var_phi_s0_bulk > 1e-8 { 1.0 } else { 0.0 };
            var_guard114 = assign8730_body3_e9040;
            var_guard114_rv = 0.0;
            let (assign8730_body4_e9053, assign8730_body4_e9053_d_n0, assign8730_body4_e9053_d_n2, assign8730_body4_e9053_d_n4, assign8730_body4_e9053_d_n5, assign8730_body4_e9053_d_n6, assign8730_body4_e9053_d_n8, assign8730_body4_e9053_d_n10, assign8730_body4_e9053_d_n11, assign8730_body4_e9053_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard114 != 0.0)) {
        let assign8730_body4_e9050: f64 = (var_beta * var_phi_s0_bulk);
        let assign8730_body4_e9051: f64 = (assign8730_body4_e9050).exp();
        (assign8730_body4_e9051, (assign8730_body4_e9051 * (var_beta * var_phi_s0_bulk_dn0)), (assign8730_body4_e9051 * (var_beta * var_phi_s0_bulk_dn2)), (assign8730_body4_e9051 * ((var_beta_dn4 * var_phi_s0_bulk) + (var_beta * var_phi_s0_bulk_dn4))), (assign8730_body4_e9051 * (var_beta * var_phi_s0_bulk_dn5)), (assign8730_body4_e9051 * (var_beta * var_phi_s0_bulk_dn6)), (assign8730_body4_e9051 * (var_beta * var_phi_s0_bulk_dn8)), (assign8730_body4_e9051 * (var_beta * var_phi_s0_bulk_dn10)), (assign8730_body4_e9051 * (var_beta * var_phi_s0_bulk_dn11)), (assign8730_body4_e9051 * (var_beta * var_phi_s0_bulk_dn12)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
            var_t0 = assign8730_body4_e9053;
            var_t0_dn0 = assign8730_body4_e9053_d_n0;
            var_t0_dn2 = assign8730_body4_e9053_d_n2;
            var_t0_dn4 = assign8730_body4_e9053_d_n4;
            var_t0_dn5 = assign8730_body4_e9053_d_n5;
            var_t0_dn6 = assign8730_body4_e9053_d_n6;
            var_t0_dn8 = assign8730_body4_e9053_d_n8;
            var_t0_dn10 = assign8730_body4_e9053_d_n10;
            var_t0_dn11 = assign8730_body4_e9053_d_n11;
            var_t0_dn12 = assign8730_body4_e9053_d_n12;
            var_t0_rv = 0.0;
            let (assign8730_body5_e9077, assign8730_body5_e9077_d_n0, assign8730_body5_e9077_d_n2, assign8730_body5_e9077_d_n4, assign8730_body5_e9077_d_n5, assign8730_body5_e9077_d_n6, assign8730_body5_e9077_d_n8, assign8730_body5_e9077_d_n10, assign8730_body5_e9077_d_n11, assign8730_body5_e9077_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard114 != 0.0)) {
        let assign8730_body5_e9062: f64 = (-var_t1);
        let assign8730_body5_e9065: f64 = (var_t3 + var_t2);
        let assign8730_body5_e9067: f64 = (assign8730_body5_e9065 - 1.0);
        let assign8730_body5_e9071: f64 = (var_t0 - 1.0);
        let assign8730_body5_e9072: f64 = (var_cnst1bulk * assign8730_body5_e9071);
        let assign8730_body5_e9073: f64 = (assign8730_body5_e9067 + assign8730_body5_e9072);
        let assign8730_body5_e9074: f64 = (assign8730_body5_e9073).sqrt();
        let assign8730_body5_e9075: f64 = (assign8730_body5_e9062 * assign8730_body5_e9074);
        (assign8730_body5_e9075, (((-var_t1_dn0) * assign8730_body5_e9074) + (assign8730_body5_e9062 * (((var_t3_dn0 + var_t2_dn0) + ((var_cnst1bulk_dn0 * assign8730_body5_e9071) + (var_cnst1bulk * var_t0_dn0))) / (2.0 * assign8730_body5_e9074)))), (((-var_t1_dn2) * assign8730_body5_e9074) + (assign8730_body5_e9062 * (((var_t3_dn2 + var_t2_dn2) + ((var_cnst1bulk_dn2 * assign8730_body5_e9071) + (var_cnst1bulk * var_t0_dn2))) / (2.0 * assign8730_body5_e9074)))), (((-var_t1_dn4) * assign8730_body5_e9074) + (assign8730_body5_e9062 * (((var_t3_dn4 + var_t2_dn4) + ((var_cnst1bulk_dn4 * assign8730_body5_e9071) + (var_cnst1bulk * var_t0_dn4))) / (2.0 * assign8730_body5_e9074)))), (((-var_t1_dn5) * assign8730_body5_e9074) + (assign8730_body5_e9062 * (((var_t3_dn5 + var_t2_dn5) + ((var_cnst1bulk_dn5 * assign8730_body5_e9071) + (var_cnst1bulk * var_t0_dn5))) / (2.0 * assign8730_body5_e9074)))), (((-var_t1_dn6) * assign8730_body5_e9074) + (assign8730_body5_e9062 * (((var_t3_dn6 + var_t2_dn6) + ((var_cnst1bulk_dn6 * assign8730_body5_e9071) + (var_cnst1bulk * var_t0_dn6))) / (2.0 * assign8730_body5_e9074)))), (((-var_t1_dn8) * assign8730_body5_e9074) + (assign8730_body5_e9062 * (((var_t3_dn8 + var_t2_dn8) + ((var_cnst1bulk_dn8 * assign8730_body5_e9071) + (var_cnst1bulk * var_t0_dn8))) / (2.0 * assign8730_body5_e9074)))), (((-var_t1_dn10) * assign8730_body5_e9074) + (assign8730_body5_e9062 * (((var_t3_dn10 + var_t2_dn10) + ((var_cnst1bulk_dn10 * assign8730_body5_e9071) + (var_cnst1bulk * var_t0_dn10))) / (2.0 * assign8730_body5_e9074)))), (((-var_t1_dn11) * assign8730_body5_e9074) + (assign8730_body5_e9062 * (((var_t3_dn11 + var_t2_dn11) + ((var_cnst1bulk_dn11 * assign8730_body5_e9071) + (var_cnst1bulk * var_t0_dn11))) / (2.0 * assign8730_body5_e9074)))), (((-var_t1_dn12) * assign8730_body5_e9074) + (assign8730_body5_e9062 * (((var_t3_dn12 + var_t2_dn12) + ((var_cnst1bulk_dn12 * assign8730_body5_e9071) + (var_cnst1bulk * var_t0_dn12))) / (2.0 * assign8730_body5_e9074)))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign8730_body5_e9077;
            var_t4_dn0 = assign8730_body5_e9077_d_n0;
            var_t4_dn2 = assign8730_body5_e9077_d_n2;
            var_t4_dn4 = assign8730_body5_e9077_d_n4;
            var_t4_dn5 = assign8730_body5_e9077_d_n5;
            var_t4_dn6 = assign8730_body5_e9077_d_n6;
            var_t4_dn8 = assign8730_body5_e9077_d_n8;
            var_t4_dn10 = assign8730_body5_e9077_d_n10;
            var_t4_dn11 = assign8730_body5_e9077_d_n11;
            var_t4_dn12 = assign8730_body5_e9077_d_n12;
            var_t4_rv = 0.0;
            let (assign8730_body6_e9098, assign8730_body6_e9098_d_n0, assign8730_body6_e9098_d_n2, assign8730_body6_e9098_d_n4, assign8730_body6_e9098_d_n5, assign8730_body6_e9098_d_n6, assign8730_body6_e9098_d_n8, assign8730_body6_e9098_d_n10, assign8730_body6_e9098_d_n11, assign8730_body6_e9098_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard114 != 0.0)) {
        let assign8730_body6_e9087: f64 = (var_c0bulk / var_t4);
        let assign8730_body6_e9089: f64 = (-var_t3);
        let assign8730_body6_e9091: f64 = (assign8730_body6_e9089 + 1.0);
        let assign8730_body6_e9094: f64 = (var_cnst1bulk * var_t0);
        let assign8730_body6_e9095: f64 = (assign8730_body6_e9091 + assign8730_body6_e9094);
        let assign8730_body6_e9096: f64 = (assign8730_body6_e9087 * assign8730_body6_e9095);
        (assign8730_body6_e9096, (((((var_c0bulk_dn0 * var_t4) - (var_c0bulk * var_t4_dn0)) / (var_t4 * var_t4)) * assign8730_body6_e9095) + (assign8730_body6_e9087 * ((-var_t3_dn0) + ((var_cnst1bulk_dn0 * var_t0) + (var_cnst1bulk * var_t0_dn0))))), (((((var_c0bulk_dn2 * var_t4) - (var_c0bulk * var_t4_dn2)) / (var_t4 * var_t4)) * assign8730_body6_e9095) + (assign8730_body6_e9087 * ((-var_t3_dn2) + ((var_cnst1bulk_dn2 * var_t0) + (var_cnst1bulk * var_t0_dn2))))), (((((var_c0bulk_dn4 * var_t4) - (var_c0bulk * var_t4_dn4)) / (var_t4 * var_t4)) * assign8730_body6_e9095) + (assign8730_body6_e9087 * ((-var_t3_dn4) + ((var_cnst1bulk_dn4 * var_t0) + (var_cnst1bulk * var_t0_dn4))))), (((((var_c0bulk_dn5 * var_t4) - (var_c0bulk * var_t4_dn5)) / (var_t4 * var_t4)) * assign8730_body6_e9095) + (assign8730_body6_e9087 * ((-var_t3_dn5) + ((var_cnst1bulk_dn5 * var_t0) + (var_cnst1bulk * var_t0_dn5))))), (((((var_c0bulk_dn6 * var_t4) - (var_c0bulk * var_t4_dn6)) / (var_t4 * var_t4)) * assign8730_body6_e9095) + (assign8730_body6_e9087 * ((-var_t3_dn6) + ((var_cnst1bulk_dn6 * var_t0) + (var_cnst1bulk * var_t0_dn6))))), (((((var_c0bulk_dn8 * var_t4) - (var_c0bulk * var_t4_dn8)) / (var_t4 * var_t4)) * assign8730_body6_e9095) + (assign8730_body6_e9087 * ((-var_t3_dn8) + ((var_cnst1bulk_dn8 * var_t0) + (var_cnst1bulk * var_t0_dn8))))), (((((var_c0bulk_dn10 * var_t4) - (var_c0bulk * var_t4_dn10)) / (var_t4 * var_t4)) * assign8730_body6_e9095) + (assign8730_body6_e9087 * ((-var_t3_dn10) + ((var_cnst1bulk_dn10 * var_t0) + (var_cnst1bulk * var_t0_dn10))))), (((((var_c0bulk_dn11 * var_t4) - (var_c0bulk * var_t4_dn11)) / (var_t4 * var_t4)) * assign8730_body6_e9095) + (assign8730_body6_e9087 * ((-var_t3_dn11) + ((var_cnst1bulk_dn11 * var_t0) + (var_cnst1bulk * var_t0_dn11))))), (((((var_c0bulk_dn12 * var_t4) - (var_c0bulk * var_t4_dn12)) / (var_t4 * var_t4)) * assign8730_body6_e9095) + (assign8730_body6_e9087 * ((-var_t3_dn12) + ((var_cnst1bulk_dn12 * var_t0) + (var_cnst1bulk * var_t0_dn12))))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign8730_body6_e9098;
            var_t5_dn0 = assign8730_body6_e9098_d_n0;
            var_t5_dn2 = assign8730_body6_e9098_d_n2;
            var_t5_dn4 = assign8730_body6_e9098_d_n4;
            var_t5_dn5 = assign8730_body6_e9098_d_n5;
            var_t5_dn6 = assign8730_body6_e9098_d_n6;
            var_t5_dn8 = assign8730_body6_e9098_d_n8;
            var_t5_dn10 = assign8730_body6_e9098_d_n10;
            var_t5_dn11 = assign8730_body6_e9098_d_n11;
            var_t5_dn12 = assign8730_body6_e9098_d_n12;
            var_t5_rv = 0.0;
            let assign8730_body7_e9101: f64 = (-1e-8);
            let assign8730_body7_e9102: f64 = if var_phi_s0_bulk < assign8730_body7_e9101 { 1.0 } else { 0.0 };
            var_guard115 = assign8730_body7_e9102;
            var_guard115_rv = 0.0;
            let (assign8730_body8_e9122, assign8730_body8_e9122_d_n0, assign8730_body8_e9122_d_n2, assign8730_body8_e9122_d_n4, assign8730_body8_e9122_d_n5, assign8730_body8_e9122_d_n6, assign8730_body8_e9122_d_n8, assign8730_body8_e9122_d_n10, assign8730_body8_e9122_d_n11, assign8730_body8_e9122_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard114 == 0.0)) && (var_guard115 != 0.0)) {
        let assign8730_body8_e9116: f64 = (var_t3 + var_t2);
        let assign8730_body8_e9118: f64 = (assign8730_body8_e9116 - 1.0);
        let assign8730_body8_e9119: f64 = (assign8730_body8_e9118).sqrt();
        let assign8730_body8_e9120: f64 = (var_t1 * assign8730_body8_e9119);
        (assign8730_body8_e9120, ((var_t1_dn0 * assign8730_body8_e9119) + (var_t1 * ((var_t3_dn0 + var_t2_dn0) / (2.0 * assign8730_body8_e9119)))), ((var_t1_dn2 * assign8730_body8_e9119) + (var_t1 * ((var_t3_dn2 + var_t2_dn2) / (2.0 * assign8730_body8_e9119)))), ((var_t1_dn4 * assign8730_body8_e9119) + (var_t1 * ((var_t3_dn4 + var_t2_dn4) / (2.0 * assign8730_body8_e9119)))), ((var_t1_dn5 * assign8730_body8_e9119) + (var_t1 * ((var_t3_dn5 + var_t2_dn5) / (2.0 * assign8730_body8_e9119)))), ((var_t1_dn6 * assign8730_body8_e9119) + (var_t1 * ((var_t3_dn6 + var_t2_dn6) / (2.0 * assign8730_body8_e9119)))), ((var_t1_dn8 * assign8730_body8_e9119) + (var_t1 * ((var_t3_dn8 + var_t2_dn8) / (2.0 * assign8730_body8_e9119)))), ((var_t1_dn10 * assign8730_body8_e9119) + (var_t1 * ((var_t3_dn10 + var_t2_dn10) / (2.0 * assign8730_body8_e9119)))), ((var_t1_dn11 * assign8730_body8_e9119) + (var_t1 * ((var_t3_dn11 + var_t2_dn11) / (2.0 * assign8730_body8_e9119)))), ((var_t1_dn12 * assign8730_body8_e9119) + (var_t1 * ((var_t3_dn12 + var_t2_dn12) / (2.0 * assign8730_body8_e9119)))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign8730_body8_e9122;
            var_t4_dn0 = assign8730_body8_e9122_d_n0;
            var_t4_dn2 = assign8730_body8_e9122_d_n2;
            var_t4_dn4 = assign8730_body8_e9122_d_n4;
            var_t4_dn5 = assign8730_body8_e9122_d_n5;
            var_t4_dn6 = assign8730_body8_e9122_d_n6;
            var_t4_dn8 = assign8730_body8_e9122_d_n8;
            var_t4_dn10 = assign8730_body8_e9122_d_n10;
            var_t4_dn11 = assign8730_body8_e9122_d_n11;
            var_t4_dn12 = assign8730_body8_e9122_d_n12;
            var_t4_rv = 0.0;
            let (assign8730_body9_e9142, assign8730_body9_e9142_d_n0, assign8730_body9_e9142_d_n2, assign8730_body9_e9142_d_n4, assign8730_body9_e9142_d_n5, assign8730_body9_e9142_d_n6, assign8730_body9_e9142_d_n8, assign8730_body9_e9142_d_n10, assign8730_body9_e9142_d_n11, assign8730_body9_e9142_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard114 == 0.0)) && (var_guard115 != 0.0)) {
        let assign8730_body9_e9135: f64 = (var_c0bulk / var_t4);
        let assign8730_body9_e9137: f64 = (-var_t3);
        let assign8730_body9_e9139: f64 = (assign8730_body9_e9137 + 1.0);
        let assign8730_body9_e9140: f64 = (assign8730_body9_e9135 * assign8730_body9_e9139);
        (assign8730_body9_e9140, (((((var_c0bulk_dn0 * var_t4) - (var_c0bulk * var_t4_dn0)) / (var_t4 * var_t4)) * assign8730_body9_e9139) + (assign8730_body9_e9135 * (-var_t3_dn0))), (((((var_c0bulk_dn2 * var_t4) - (var_c0bulk * var_t4_dn2)) / (var_t4 * var_t4)) * assign8730_body9_e9139) + (assign8730_body9_e9135 * (-var_t3_dn2))), (((((var_c0bulk_dn4 * var_t4) - (var_c0bulk * var_t4_dn4)) / (var_t4 * var_t4)) * assign8730_body9_e9139) + (assign8730_body9_e9135 * (-var_t3_dn4))), (((((var_c0bulk_dn5 * var_t4) - (var_c0bulk * var_t4_dn5)) / (var_t4 * var_t4)) * assign8730_body9_e9139) + (assign8730_body9_e9135 * (-var_t3_dn5))), (((((var_c0bulk_dn6 * var_t4) - (var_c0bulk * var_t4_dn6)) / (var_t4 * var_t4)) * assign8730_body9_e9139) + (assign8730_body9_e9135 * (-var_t3_dn6))), (((((var_c0bulk_dn8 * var_t4) - (var_c0bulk * var_t4_dn8)) / (var_t4 * var_t4)) * assign8730_body9_e9139) + (assign8730_body9_e9135 * (-var_t3_dn8))), (((((var_c0bulk_dn10 * var_t4) - (var_c0bulk * var_t4_dn10)) / (var_t4 * var_t4)) * assign8730_body9_e9139) + (assign8730_body9_e9135 * (-var_t3_dn10))), (((((var_c0bulk_dn11 * var_t4) - (var_c0bulk * var_t4_dn11)) / (var_t4 * var_t4)) * assign8730_body9_e9139) + (assign8730_body9_e9135 * (-var_t3_dn11))), (((((var_c0bulk_dn12 * var_t4) - (var_c0bulk * var_t4_dn12)) / (var_t4 * var_t4)) * assign8730_body9_e9139) + (assign8730_body9_e9135 * (-var_t3_dn12))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign8730_body9_e9142;
            var_t5_dn0 = assign8730_body9_e9142_d_n0;
            var_t5_dn2 = assign8730_body9_e9142_d_n2;
            var_t5_dn4 = assign8730_body9_e9142_d_n4;
            var_t5_dn5 = assign8730_body9_e9142_d_n5;
            var_t5_dn6 = assign8730_body9_e9142_d_n6;
            var_t5_dn8 = assign8730_body9_e9142_d_n8;
            var_t5_dn10 = assign8730_body9_e9142_d_n10;
            var_t5_dn11 = assign8730_body9_e9142_d_n11;
            var_t5_dn12 = assign8730_body9_e9142_d_n12;
            var_t5_rv = 0.0;
            let (assign8730_body10_e9164, assign8730_body10_e9164_d_n0, assign8730_body10_e9164_d_n2, assign8730_body10_e9164_d_n4, assign8730_body10_e9164_d_n5, assign8730_body10_e9164_d_n6, assign8730_body10_e9164_d_n8, assign8730_body10_e9164_d_n10, assign8730_body10_e9164_d_n11, assign8730_body10_e9164_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard114 == 0.0)) && (var_guard115 == 0.0)) {
        let assign8730_body10_e9156: f64 = (var_c0bulk / var_beta);
        let assign8730_body10_e9157: f64 = (assign8730_body10_e9156).sqrt();
        let assign8730_body10_e9158: f64 = (-assign8730_body10_e9157);
        let assign8730_body10_e9160: f64 = (assign8730_body10_e9158 * var_beta);
        let assign8730_body10_e9162: f64 = (assign8730_body10_e9160 * var_phi_s0_bulk);
        (assign8730_body10_e9162, ((((-((var_c0bulk_dn0 / var_beta) / (2.0 * assign8730_body10_e9157))) * var_beta) * var_phi_s0_bulk) + (assign8730_body10_e9160 * var_phi_s0_bulk_dn0)), ((((-((var_c0bulk_dn2 / var_beta) / (2.0 * assign8730_body10_e9157))) * var_beta) * var_phi_s0_bulk) + (assign8730_body10_e9160 * var_phi_s0_bulk_dn2)), (((((-((((var_c0bulk_dn4 * var_beta) - (var_c0bulk * var_beta_dn4)) / (var_beta * var_beta)) / (2.0 * assign8730_body10_e9157))) * var_beta) + (assign8730_body10_e9158 * var_beta_dn4)) * var_phi_s0_bulk) + (assign8730_body10_e9160 * var_phi_s0_bulk_dn4)), ((((-((var_c0bulk_dn5 / var_beta) / (2.0 * assign8730_body10_e9157))) * var_beta) * var_phi_s0_bulk) + (assign8730_body10_e9160 * var_phi_s0_bulk_dn5)), ((((-((var_c0bulk_dn6 / var_beta) / (2.0 * assign8730_body10_e9157))) * var_beta) * var_phi_s0_bulk) + (assign8730_body10_e9160 * var_phi_s0_bulk_dn6)), ((((-((var_c0bulk_dn8 / var_beta) / (2.0 * assign8730_body10_e9157))) * var_beta) * var_phi_s0_bulk) + (assign8730_body10_e9160 * var_phi_s0_bulk_dn8)), ((((-((var_c0bulk_dn10 / var_beta) / (2.0 * assign8730_body10_e9157))) * var_beta) * var_phi_s0_bulk) + (assign8730_body10_e9160 * var_phi_s0_bulk_dn10)), ((((-((var_c0bulk_dn11 / var_beta) / (2.0 * assign8730_body10_e9157))) * var_beta) * var_phi_s0_bulk) + (assign8730_body10_e9160 * var_phi_s0_bulk_dn11)), ((((-((var_c0bulk_dn12 / var_beta) / (2.0 * assign8730_body10_e9157))) * var_beta) * var_phi_s0_bulk) + (assign8730_body10_e9160 * var_phi_s0_bulk_dn12)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign8730_body10_e9164;
            var_t4_dn0 = assign8730_body10_e9164_d_n0;
            var_t4_dn2 = assign8730_body10_e9164_d_n2;
            var_t4_dn4 = assign8730_body10_e9164_d_n4;
            var_t4_dn5 = assign8730_body10_e9164_d_n5;
            var_t4_dn6 = assign8730_body10_e9164_d_n6;
            var_t4_dn8 = assign8730_body10_e9164_d_n8;
            var_t4_dn10 = assign8730_body10_e9164_d_n10;
            var_t4_dn11 = assign8730_body10_e9164_d_n11;
            var_t4_dn12 = assign8730_body10_e9164_d_n12;
            var_t4_rv = 0.0;
            let (assign8730_body11_e9182, assign8730_body11_e9182_d_n0, assign8730_body11_e9182_d_n2, assign8730_body11_e9182_d_n4, assign8730_body11_e9182_d_n5, assign8730_body11_e9182_d_n6, assign8730_body11_e9182_d_n8, assign8730_body11_e9182_d_n10, assign8730_body11_e9182_d_n11, assign8730_body11_e9182_d_n12,) = {
    if ((((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard114 == 0.0)) && (var_guard115 == 0.0)) {
        let assign8730_body11_e9178: f64 = (var_c0bulk * var_beta);
        let assign8730_body11_e9179: f64 = (assign8730_body11_e9178).sqrt();
        let assign8730_body11_e9180: f64 = (-assign8730_body11_e9179);
        (assign8730_body11_e9180, (-((var_c0bulk_dn0 * var_beta) / (2.0 * assign8730_body11_e9179))), (-((var_c0bulk_dn2 * var_beta) / (2.0 * assign8730_body11_e9179))), (-(((var_c0bulk_dn4 * var_beta) + (var_c0bulk * var_beta_dn4)) / (2.0 * assign8730_body11_e9179))), (-((var_c0bulk_dn5 * var_beta) / (2.0 * assign8730_body11_e9179))), (-((var_c0bulk_dn6 * var_beta) / (2.0 * assign8730_body11_e9179))), (-((var_c0bulk_dn8 * var_beta) / (2.0 * assign8730_body11_e9179))), (-((var_c0bulk_dn10 * var_beta) / (2.0 * assign8730_body11_e9179))), (-((var_c0bulk_dn11 * var_beta) / (2.0 * assign8730_body11_e9179))), (-((var_c0bulk_dn12 * var_beta) / (2.0 * assign8730_body11_e9179))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign8730_body11_e9182;
            var_t5_dn0 = assign8730_body11_e9182_d_n0;
            var_t5_dn2 = assign8730_body11_e9182_d_n2;
            var_t5_dn4 = assign8730_body11_e9182_d_n4;
            var_t5_dn5 = assign8730_body11_e9182_d_n5;
            var_t5_dn6 = assign8730_body11_e9182_d_n6;
            var_t5_dn8 = assign8730_body11_e9182_d_n8;
            var_t5_dn10 = assign8730_body11_e9182_d_n10;
            var_t5_dn11 = assign8730_body11_e9182_d_n11;
            var_t5_dn12 = assign8730_body11_e9182_d_n12;
            var_t5_rv = 0.0;
            let (assign8730_body12_e9223, assign8730_body12_e9223_d_n0, assign8730_body12_e9223_d_n2, assign8730_body12_e9223_d_n4, assign8730_body12_e9223_d_n5, assign8730_body12_e9223_d_n6, assign8730_body12_e9223_d_n8, assign8730_body12_e9223_d_n10, assign8730_body12_e9223_d_n11, assign8730_body12_e9223_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        let assign8730_body12_e9191: f64 = (var_ps0 - var_phi_s0_bulk);
        let assign8730_body12_e9194: f64 = (var_t4 / var_c_box);
        let assign8730_body12_e9195: f64 = (assign8730_body12_e9191 + assign8730_body12_e9194);
        let assign8730_body12_e9199: f64 = (var_q_fd_soi / 2.0);
        let assign8730_body12_e9200: f64 = (var_t4 + assign8730_body12_e9199);
        let assign8730_body12_e9202: f64 = (assign8730_body12_e9200 * p.p227);
        let assign8730_body12_e9204: f64 = (assign8730_body12_e9202 / 1.034943e-10);
        let assign8730_body12_e9205: f64 = (assign8730_body12_e9195 + assign8730_body12_e9204);
        let assign8730_body12_e9207: f64 = (assign8730_body12_e9205 - var_vbsbiz);
        let assign8730_body12_e9209: f64 = (-1.0);
        let assign8730_body12_e9212: f64 = (var_t5 / var_c_box);
        let assign8730_body12_e9213: f64 = (assign8730_body12_e9209 + assign8730_body12_e9212);
        let assign8730_body12_e9216: f64 = (var_t5 * p.p227);
        let assign8730_body12_e9218: f64 = (assign8730_body12_e9216 / 1.034943e-10);
        let assign8730_body12_e9219: f64 = (assign8730_body12_e9213 + assign8730_body12_e9218);
        let assign8730_body12_e9220: f64 = (assign8730_body12_e9207 / assign8730_body12_e9219);
        let assign8730_body12_e9221: f64 = (var_phi_s0_bulk - assign8730_body12_e9220);
        (assign8730_body12_e9221, (var_phi_s0_bulk_dn0 - (((((((var_ps0_dn0 - var_phi_s0_bulk_dn0) + (var_t4_dn0 / var_c_box)) + (((var_t4_dn0 + (var_q_fd_soi_dn0 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn0) * assign8730_body12_e9219) - (assign8730_body12_e9207 * ((var_t5_dn0 / var_c_box) + ((var_t5_dn0 * p.p227) / 1.034943e-10)))) / (assign8730_body12_e9219 * assign8730_body12_e9219))), (var_phi_s0_bulk_dn2 - (((((((var_ps0_dn2 - var_phi_s0_bulk_dn2) + (var_t4_dn2 / var_c_box)) + (((var_t4_dn2 + (var_q_fd_soi_dn2 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn2) * assign8730_body12_e9219) - (assign8730_body12_e9207 * ((var_t5_dn2 / var_c_box) + ((var_t5_dn2 * p.p227) / 1.034943e-10)))) / (assign8730_body12_e9219 * assign8730_body12_e9219))), (var_phi_s0_bulk_dn4 - (((((((var_ps0_dn4 - var_phi_s0_bulk_dn4) + (var_t4_dn4 / var_c_box)) + (((var_t4_dn4 + (var_q_fd_soi_dn4 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn4) * assign8730_body12_e9219) - (assign8730_body12_e9207 * ((var_t5_dn4 / var_c_box) + ((var_t5_dn4 * p.p227) / 1.034943e-10)))) / (assign8730_body12_e9219 * assign8730_body12_e9219))), (var_phi_s0_bulk_dn5 - (((((((var_ps0_dn5 - var_phi_s0_bulk_dn5) + (var_t4_dn5 / var_c_box)) + (((var_t4_dn5 + (var_q_fd_soi_dn5 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn5) * assign8730_body12_e9219) - (assign8730_body12_e9207 * ((var_t5_dn5 / var_c_box) + ((var_t5_dn5 * p.p227) / 1.034943e-10)))) / (assign8730_body12_e9219 * assign8730_body12_e9219))), (var_phi_s0_bulk_dn6 - (((((((var_ps0_dn6 - var_phi_s0_bulk_dn6) + (var_t4_dn6 / var_c_box)) + (((var_t4_dn6 + (var_q_fd_soi_dn6 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn6) * assign8730_body12_e9219) - (assign8730_body12_e9207 * ((var_t5_dn6 / var_c_box) + ((var_t5_dn6 * p.p227) / 1.034943e-10)))) / (assign8730_body12_e9219 * assign8730_body12_e9219))), (var_phi_s0_bulk_dn8 - (((((((var_ps0_dn8 - var_phi_s0_bulk_dn8) + (var_t4_dn8 / var_c_box)) + (((var_t4_dn8 + (var_q_fd_soi_dn8 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn8) * assign8730_body12_e9219) - (assign8730_body12_e9207 * ((var_t5_dn8 / var_c_box) + ((var_t5_dn8 * p.p227) / 1.034943e-10)))) / (assign8730_body12_e9219 * assign8730_body12_e9219))), (var_phi_s0_bulk_dn10 - (((((((var_ps0_dn10 - var_phi_s0_bulk_dn10) + (var_t4_dn10 / var_c_box)) + (((var_t4_dn10 + (var_q_fd_soi_dn10 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn10) * assign8730_body12_e9219) - (assign8730_body12_e9207 * ((var_t5_dn10 / var_c_box) + ((var_t5_dn10 * p.p227) / 1.034943e-10)))) / (assign8730_body12_e9219 * assign8730_body12_e9219))), (var_phi_s0_bulk_dn11 - (((((((var_ps0_dn11 - var_phi_s0_bulk_dn11) + (var_t4_dn11 / var_c_box)) + (((var_t4_dn11 + (var_q_fd_soi_dn11 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn11) * assign8730_body12_e9219) - (assign8730_body12_e9207 * ((var_t5_dn11 / var_c_box) + ((var_t5_dn11 * p.p227) / 1.034943e-10)))) / (assign8730_body12_e9219 * assign8730_body12_e9219))), (var_phi_s0_bulk_dn12 - (((((((var_ps0_dn12 - var_phi_s0_bulk_dn12) + (var_t4_dn12 / var_c_box)) + (((var_t4_dn12 + (var_q_fd_soi_dn12 / 2.0)) * p.p227) / 1.034943e-10)) - var_vbsbiz_dn12) * assign8730_body12_e9219) - (assign8730_body12_e9207 * ((var_t5_dn12 / var_c_box) + ((var_t5_dn12 * p.p227) / 1.034943e-10)))) / (assign8730_body12_e9219 * assign8730_body12_e9219))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
            var_t6 = assign8730_body12_e9223;
            var_t6_dn0 = assign8730_body12_e9223_d_n0;
            var_t6_dn2 = assign8730_body12_e9223_d_n2;
            var_t6_dn4 = assign8730_body12_e9223_d_n4;
            var_t6_dn5 = assign8730_body12_e9223_d_n5;
            var_t6_dn6 = assign8730_body12_e9223_d_n6;
            var_t6_dn8 = assign8730_body12_e9223_d_n8;
            var_t6_dn10 = assign8730_body12_e9223_d_n10;
            var_t6_dn11 = assign8730_body12_e9223_d_n11;
            var_t6_dn12 = assign8730_body12_e9223_d_n12;
            var_t6_rv = 0.0;
            let assign8730_body13_e9226: f64 = (var_t6 - var_phi_s0_bulk);
            let assign8730_body13_e9227: f64 = (assign8730_body13_e9226).abs();
            let assign8730_body13_e9229: f64 = if assign8730_body13_e9227 < var_ps_conv_ini { 1.0 } else { 0.0 };
            var_guard116 = assign8730_body13_e9229;
            var_guard116_rv = 0.0;
            let (assign8730_body14_e9239, assign8730_body14_e9239_d_n0, assign8730_body14_e9239_d_n2, assign8730_body14_e9239_d_n4, assign8730_body14_e9239_d_n5, assign8730_body14_e9239_d_n6, assign8730_body14_e9239_d_n8, assign8730_body14_e9239_d_n10, assign8730_body14_e9239_d_n11, assign8730_body14_e9239_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard116 != 0.0)) {
        (var_lp_s0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
            var_t7 = assign8730_body14_e9239;
            var_t7_dn0 = assign8730_body14_e9239_d_n0;
            var_t7_dn2 = assign8730_body14_e9239_d_n2;
            var_t7_dn4 = assign8730_body14_e9239_d_n4;
            var_t7_dn5 = assign8730_body14_e9239_d_n5;
            var_t7_dn6 = assign8730_body14_e9239_d_n6;
            var_t7_dn8 = assign8730_body14_e9239_d_n8;
            var_t7_dn10 = assign8730_body14_e9239_d_n10;
            var_t7_dn11 = assign8730_body14_e9239_d_n11;
            var_t7_dn12 = assign8730_body14_e9239_d_n12;
            var_t7_rv = 0.0;
            let (assign8730_body15_e9249,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard116 != 0.0)) {
        (var_lp_s0_max,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign8730_body15_e9249;
            var_lp_s0_rv = 0.0;
            let (assign8730_body16_e9257, assign8730_body16_e9257_d_n0, assign8730_body16_e9257_d_n2, assign8730_body16_e9257_d_n4, assign8730_body16_e9257_d_n5, assign8730_body16_e9257_d_n6, assign8730_body16_e9257_d_n8, assign8730_body16_e9257_d_n10, assign8730_body16_e9257_d_n11, assign8730_body16_e9257_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
            var_phi_s0_bulk = assign8730_body16_e9257;
            var_phi_s0_bulk_dn0 = assign8730_body16_e9257_d_n0;
            var_phi_s0_bulk_dn2 = assign8730_body16_e9257_d_n2;
            var_phi_s0_bulk_dn4 = assign8730_body16_e9257_d_n4;
            var_phi_s0_bulk_dn5 = assign8730_body16_e9257_d_n5;
            var_phi_s0_bulk_dn6 = assign8730_body16_e9257_d_n6;
            var_phi_s0_bulk_dn8 = assign8730_body16_e9257_d_n8;
            var_phi_s0_bulk_dn10 = assign8730_body16_e9257_d_n10;
            var_phi_s0_bulk_dn11 = assign8730_body16_e9257_d_n11;
            var_phi_s0_bulk_dn12 = assign8730_body16_e9257_d_n12;
            var_phi_s0_bulk_rv = 0.0;
            let (assign8730_body17_e9265, assign8730_body17_e9265_d_n0, assign8730_body17_e9265_d_n2, assign8730_body17_e9265_d_n4, assign8730_body17_e9265_d_n5, assign8730_body17_e9265_d_n6, assign8730_body17_e9265_d_n8, assign8730_body17_e9265_d_n10, assign8730_body17_e9265_d_n11, assign8730_body17_e9265_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    } else {
        (var_q_s0_bulk, var_q_s0_bulk_dn0, var_q_s0_bulk_dn2, var_q_s0_bulk_dn4, var_q_s0_bulk_dn5, var_q_s0_bulk_dn6, var_q_s0_bulk_dn8, var_q_s0_bulk_dn10, var_q_s0_bulk_dn11, var_q_s0_bulk_dn12,)
    }
};
            var_q_s0_bulk = assign8730_body17_e9265;
            var_q_s0_bulk_dn0 = assign8730_body17_e9265_d_n0;
            var_q_s0_bulk_dn2 = assign8730_body17_e9265_d_n2;
            var_q_s0_bulk_dn4 = assign8730_body17_e9265_d_n4;
            var_q_s0_bulk_dn5 = assign8730_body17_e9265_d_n5;
            var_q_s0_bulk_dn6 = assign8730_body17_e9265_d_n6;
            var_q_s0_bulk_dn8 = assign8730_body17_e9265_d_n8;
            var_q_s0_bulk_dn10 = assign8730_body17_e9265_d_n10;
            var_q_s0_bulk_dn11 = assign8730_body17_e9265_d_n11;
            var_q_s0_bulk_dn12 = assign8730_body17_e9265_d_n12;
            var_q_s0_bulk_rv = 0.0;
            let (assign8730_body18_e9275,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        let assign8730_body18_e9273: f64 = (var_lp_s0 + 1.0);
        (assign8730_body18_e9273,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign8730_body18_e9275;
            var_lp_s0_rv = 0.0;
        }

        let assign8740_e9278: f64 = if 1.0 == 0.0 { 1.0 } else { 0.0 };
        var_guard117 = assign8740_e9278;
        var_guard117_rv = 0.0;

        let (assign8750_e9288, assign8750_e9288_d_n0, assign8750_e9288_d_n2, assign8750_e9288_d_n4, assign8750_e9288_d_n5, assign8750_e9288_d_n6, assign8750_e9288_d_n8, assign8750_e9288_d_n10, assign8750_e9288_d_n11, assign8750_e9288_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard117 != 0.0)) {
        (var_q_s0_bulk, var_q_s0_bulk_dn0, var_q_s0_bulk_dn2, var_q_s0_bulk_dn4, var_q_s0_bulk_dn5, var_q_s0_bulk_dn6, var_q_s0_bulk_dn8, var_q_s0_bulk_dn10, var_q_s0_bulk_dn11, var_q_s0_bulk_dn12,)
    } else {
        (var_q_s0_bulk_dep, var_q_s0_bulk_dep_dn0, var_q_s0_bulk_dep_dn2, var_q_s0_bulk_dep_dn4, var_q_s0_bulk_dep_dn5, var_q_s0_bulk_dep_dn6, var_q_s0_bulk_dep_dn8, var_q_s0_bulk_dep_dn10, var_q_s0_bulk_dep_dn11, var_q_s0_bulk_dep_dn12,)
    }
};
        var_q_s0_bulk_dep = assign8750_e9288;
        var_q_s0_bulk_dep_dn0 = assign8750_e9288_d_n0;
        var_q_s0_bulk_dep_dn2 = assign8750_e9288_d_n2;
        var_q_s0_bulk_dep_dn4 = assign8750_e9288_d_n4;
        var_q_s0_bulk_dep_dn5 = assign8750_e9288_d_n5;
        var_q_s0_bulk_dep_dn6 = assign8750_e9288_d_n6;
        var_q_s0_bulk_dep_dn8 = assign8750_e9288_d_n8;
        var_q_s0_bulk_dep_dn10 = assign8750_e9288_d_n10;
        var_q_s0_bulk_dep_dn11 = assign8750_e9288_d_n11;
        var_q_s0_bulk_dep_dn12 = assign8750_e9288_d_n12;
        var_q_s0_bulk_dep_rv = 0.0;

        let (assign8760_e9296,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        (0.0,)
    } else {
        (var_lp_sl,)
    }
};
        var_lp_sl = assign8760_e9296;
        var_lp_sl_rv = 0.0;

        let (assign8770_e9305, assign8770_e9305_d_n0, assign8770_e9305_d_n2, assign8770_e9305_d_n4, assign8770_e9305_d_n5, assign8770_e9305_d_n6, assign8770_e9305_d_n8, assign8770_e9305_d_n10, assign8770_e9305_d_n11, assign8770_e9305_d_n12,) = {
    if (var_guard74 == 0.0) {
        let assign8770_e9301: f64 = (var_vbsbiz + var_phi_s0_bulk);
        let assign8770_e9303: f64 = (assign8770_e9301 - 0.01);
        (assign8770_e9303, (var_vbsbiz_dn0 + var_phi_s0_bulk_dn0), (var_vbsbiz_dn2 + var_phi_s0_bulk_dn2), (var_vbsbiz_dn4 + var_phi_s0_bulk_dn4), (var_vbsbiz_dn5 + var_phi_s0_bulk_dn5), (var_vbsbiz_dn6 + var_phi_s0_bulk_dn6), (var_vbsbiz_dn8 + var_phi_s0_bulk_dn8), (var_vbsbiz_dn10 + var_phi_s0_bulk_dn10), (var_vbsbiz_dn11 + var_phi_s0_bulk_dn11), (var_vbsbiz_dn12 + var_phi_s0_bulk_dn12),)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
        var_phi_s0_bulk = assign8770_e9305;
        var_phi_s0_bulk_dn0 = assign8770_e9305_d_n0;
        var_phi_s0_bulk_dn2 = assign8770_e9305_d_n2;
        var_phi_s0_bulk_dn4 = assign8770_e9305_d_n4;
        var_phi_s0_bulk_dn5 = assign8770_e9305_d_n5;
        var_phi_s0_bulk_dn6 = assign8770_e9305_d_n6;
        var_phi_s0_bulk_dn8 = assign8770_e9305_d_n8;
        var_phi_s0_bulk_dn10 = assign8770_e9305_d_n10;
        var_phi_s0_bulk_dn11 = assign8770_e9305_d_n11;
        var_phi_s0_bulk_dn12 = assign8770_e9305_d_n12;
        var_phi_s0_bulk_rv = 0.0;

        let (assign8780_e9314, assign8780_e9314_d_n0, assign8780_e9314_d_n2, assign8780_e9314_d_n4, assign8780_e9314_d_n5, assign8780_e9314_d_n6, assign8780_e9314_d_n8, assign8780_e9314_d_n10, assign8780_e9314_d_n11, assign8780_e9314_d_n12,) = {
    if (var_guard74 == 0.0) {
        let assign8780_e9311: f64 = (var_q_s0_bulk / var_c_box);
        let assign8780_e9312: f64 = (var_phi_s0_bulk - assign8780_e9311);
        (assign8780_e9312, (var_phi_s0_bulk_dn0 - (var_q_s0_bulk_dn0 / var_c_box)), (var_phi_s0_bulk_dn2 - (var_q_s0_bulk_dn2 / var_c_box)), (var_phi_s0_bulk_dn4 - (var_q_s0_bulk_dn4 / var_c_box)), (var_phi_s0_bulk_dn5 - (var_q_s0_bulk_dn5 / var_c_box)), (var_phi_s0_bulk_dn6 - (var_q_s0_bulk_dn6 / var_c_box)), (var_phi_s0_bulk_dn8 - (var_q_s0_bulk_dn8 / var_c_box)), (var_phi_s0_bulk_dn10 - (var_q_s0_bulk_dn10 / var_c_box)), (var_phi_s0_bulk_dn11 - (var_q_s0_bulk_dn11 / var_c_box)), (var_phi_s0_bulk_dn12 - (var_q_s0_bulk_dn12 / var_c_box)),)
    } else {
        (var_phi_b0_soi, var_phi_b0_soi_dn0, var_phi_b0_soi_dn2, var_phi_b0_soi_dn4, var_phi_b0_soi_dn5, var_phi_b0_soi_dn6, var_phi_b0_soi_dn8, var_phi_b0_soi_dn10, var_phi_b0_soi_dn11, var_phi_b0_soi_dn12,)
    }
};
        var_phi_b0_soi = assign8780_e9314;
        var_phi_b0_soi_dn0 = assign8780_e9314_d_n0;
        var_phi_b0_soi_dn2 = assign8780_e9314_d_n2;
        var_phi_b0_soi_dn4 = assign8780_e9314_d_n4;
        var_phi_b0_soi_dn5 = assign8780_e9314_d_n5;
        var_phi_b0_soi_dn6 = assign8780_e9314_d_n6;
        var_phi_b0_soi_dn8 = assign8780_e9314_d_n8;
        var_phi_b0_soi_dn10 = assign8780_e9314_d_n10;
        var_phi_b0_soi_dn11 = assign8780_e9314_d_n11;
        var_phi_b0_soi_dn12 = assign8780_e9314_d_n12;
        var_phi_b0_soi_rv = 0.0;

        let assign8790_e9318: f64 = (var_phi_s0_soi - 0.15);
        let assign8790_e9323: f64 = if ((var_phi_b0_soi > assign8790_e9318) && (0.15 >= 0.0)) { 1.0 } else { 0.0 };
        var_guard118 = assign8790_e9323;
        var_guard118_rv = 0.0;

        let (assign8800_e9334, assign8800_e9334_d_n0, assign8800_e9334_d_n2, assign8800_e9334_d_n4, assign8800_e9334_d_n5, assign8800_e9334_d_n6, assign8800_e9334_d_n8, assign8800_e9334_d_n10, assign8800_e9334_d_n11, assign8800_e9334_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        let assign8800_e9330: f64 = (var_phi_b0_soi - var_phi_s0_soi);
        let assign8800_e9332: f64 = (assign8800_e9330 + 0.15);
        (assign8800_e9332, (var_phi_b0_soi_dn0 - var_phi_s0_soi_dn0), (var_phi_b0_soi_dn2 - var_phi_s0_soi_dn2), (var_phi_b0_soi_dn4 - var_phi_s0_soi_dn4), (var_phi_b0_soi_dn5 - var_phi_s0_soi_dn5), (var_phi_b0_soi_dn6 - var_phi_s0_soi_dn6), (var_phi_b0_soi_dn8 - var_phi_s0_soi_dn8), (var_phi_b0_soi_dn10 - var_phi_s0_soi_dn10), (var_phi_b0_soi_dn11 - var_phi_s0_soi_dn11), (var_phi_b0_soi_dn12 - var_phi_s0_soi_dn12),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn8, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12,)
    }
};
        var_tmf1 = assign8800_e9334;
        var_tmf1_dn0 = assign8800_e9334_d_n0;
        var_tmf1_dn2 = assign8800_e9334_d_n2;
        var_tmf1_dn4 = assign8800_e9334_d_n4;
        var_tmf1_dn5 = assign8800_e9334_d_n5;
        var_tmf1_dn6 = assign8800_e9334_d_n6;
        var_tmf1_dn8 = assign8800_e9334_d_n8;
        var_tmf1_dn10 = assign8800_e9334_d_n10;
        var_tmf1_dn11 = assign8800_e9334_d_n11;
        var_tmf1_dn12 = assign8800_e9334_d_n12;
        var_tmf1_rv = 0.0;

        let (assign8810_e9343, assign8810_e9343_d_n0, assign8810_e9343_d_n2, assign8810_e9343_d_n4, assign8810_e9343_d_n5, assign8810_e9343_d_n6, assign8810_e9343_d_n8, assign8810_e9343_d_n10, assign8810_e9343_d_n11, assign8810_e9343_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        let assign8810_e9341: f64 = (var_tmf1 * var_tmf1);
        (assign8810_e9341, ((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)), ((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)), ((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)), ((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)), ((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)), ((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)), ((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)), ((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)), ((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn4, var_x2_dn5, var_x2_dn6, var_x2_dn8, var_x2_dn10, var_x2_dn11, var_x2_dn12,)
    }
};
        var_x2 = assign8810_e9343;
        var_x2_dn0 = assign8810_e9343_d_n0;
        var_x2_dn2 = assign8810_e9343_d_n2;
        var_x2_dn4 = assign8810_e9343_d_n4;
        var_x2_dn5 = assign8810_e9343_d_n5;
        var_x2_dn6 = assign8810_e9343_d_n6;
        var_x2_dn8 = assign8810_e9343_d_n8;
        var_x2_dn10 = assign8810_e9343_d_n10;
        var_x2_dn11 = assign8810_e9343_d_n11;
        var_x2_dn12 = assign8810_e9343_d_n12;
        var_x2_rv = 0.0;

        let (assign8820_e9352, assign8820_e9352_d_n0, assign8820_e9352_d_n2, assign8820_e9352_d_n4, assign8820_e9352_d_n5, assign8820_e9352_d_n6, assign8820_e9352_d_n8, assign8820_e9352_d_n10, assign8820_e9352_d_n11, assign8820_e9352_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        let assign8820_e9350: f64 = (0.15 * 0.15);
        (assign8820_e9350, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn4, var_xmax2_dn5, var_xmax2_dn6, var_xmax2_dn8, var_xmax2_dn10, var_xmax2_dn11, var_xmax2_dn12,)
    }
};
        var_xmax2 = assign8820_e9352;
        var_xmax2_dn0 = assign8820_e9352_d_n0;
        var_xmax2_dn2 = assign8820_e9352_d_n2;
        var_xmax2_dn4 = assign8820_e9352_d_n4;
        var_xmax2_dn5 = assign8820_e9352_d_n5;
        var_xmax2_dn6 = assign8820_e9352_d_n6;
        var_xmax2_dn8 = assign8820_e9352_d_n8;
        var_xmax2_dn10 = assign8820_e9352_d_n10;
        var_xmax2_dn11 = assign8820_e9352_d_n11;
        var_xmax2_dn12 = assign8820_e9352_d_n12;
        var_xmax2_rv = 0.0;

        let (assign8830_e9359, assign8830_e9359_d_n0, assign8830_e9359_d_n2, assign8830_e9359_d_n4, assign8830_e9359_d_n5, assign8830_e9359_d_n6, assign8830_e9359_d_n8, assign8830_e9359_d_n10, assign8830_e9359_d_n11, assign8830_e9359_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn8, var_xp_dn10, var_xp_dn11, var_xp_dn12,)
    }
};
        var_xp = assign8830_e9359;
        var_xp_dn0 = assign8830_e9359_d_n0;
        var_xp_dn2 = assign8830_e9359_d_n2;
        var_xp_dn4 = assign8830_e9359_d_n4;
        var_xp_dn5 = assign8830_e9359_d_n5;
        var_xp_dn6 = assign8830_e9359_d_n6;
        var_xp_dn8 = assign8830_e9359_d_n8;
        var_xp_dn10 = assign8830_e9359_d_n10;
        var_xp_dn11 = assign8830_e9359_d_n11;
        var_xp_dn12 = assign8830_e9359_d_n12;
        var_xp_rv = 0.0;

        *var_guard114_slot = var_guard114;
        *var_guard114_rv_slot = var_guard114_rv;
        *var_guard115_slot = var_guard115;
        *var_guard115_rv_slot = var_guard115_rv;
        *var_guard116_slot = var_guard116;
        *var_guard116_rv_slot = var_guard116_rv;
        *var_guard117_slot = var_guard117;
        *var_guard117_rv_slot = var_guard117_rv;
        *var_guard118_slot = var_guard118;
        *var_guard118_rv_slot = var_guard118_rv;
        *var_lp_s0_slot = var_lp_s0;
        *var_lp_s0_rv_slot = var_lp_s0_rv;
        *var_lp_sl_slot = var_lp_sl;
        *var_lp_sl_rv_slot = var_lp_sl_rv;
        *var_phi_b0_soi_slot = var_phi_b0_soi;
        *var_phi_b0_soi_dn0_slot = var_phi_b0_soi_dn0;
        *var_phi_b0_soi_dn10_slot = var_phi_b0_soi_dn10;
        *var_phi_b0_soi_dn11_slot = var_phi_b0_soi_dn11;
        *var_phi_b0_soi_dn12_slot = var_phi_b0_soi_dn12;
        *var_phi_b0_soi_dn2_slot = var_phi_b0_soi_dn2;
        *var_phi_b0_soi_dn4_slot = var_phi_b0_soi_dn4;
        *var_phi_b0_soi_dn5_slot = var_phi_b0_soi_dn5;
        *var_phi_b0_soi_dn6_slot = var_phi_b0_soi_dn6;
        *var_phi_b0_soi_dn8_slot = var_phi_b0_soi_dn8;
        *var_phi_b0_soi_rv_slot = var_phi_b0_soi_rv;
        *var_phi_s0_bulk_slot = var_phi_s0_bulk;
        *var_phi_s0_bulk_dn0_slot = var_phi_s0_bulk_dn0;
        *var_phi_s0_bulk_dn10_slot = var_phi_s0_bulk_dn10;
        *var_phi_s0_bulk_dn11_slot = var_phi_s0_bulk_dn11;
        *var_phi_s0_bulk_dn12_slot = var_phi_s0_bulk_dn12;
        *var_phi_s0_bulk_dn2_slot = var_phi_s0_bulk_dn2;
        *var_phi_s0_bulk_dn4_slot = var_phi_s0_bulk_dn4;
        *var_phi_s0_bulk_dn5_slot = var_phi_s0_bulk_dn5;
        *var_phi_s0_bulk_dn6_slot = var_phi_s0_bulk_dn6;
        *var_phi_s0_bulk_dn8_slot = var_phi_s0_bulk_dn8;
        *var_phi_s0_bulk_rv_slot = var_phi_s0_bulk_rv;
        *var_q_s0_bulk_slot = var_q_s0_bulk;
        *var_q_s0_bulk_dep_slot = var_q_s0_bulk_dep;
        *var_q_s0_bulk_dep_dn0_slot = var_q_s0_bulk_dep_dn0;
        *var_q_s0_bulk_dep_dn10_slot = var_q_s0_bulk_dep_dn10;
        *var_q_s0_bulk_dep_dn11_slot = var_q_s0_bulk_dep_dn11;
        *var_q_s0_bulk_dep_dn12_slot = var_q_s0_bulk_dep_dn12;
        *var_q_s0_bulk_dep_dn2_slot = var_q_s0_bulk_dep_dn2;
        *var_q_s0_bulk_dep_dn4_slot = var_q_s0_bulk_dep_dn4;
        *var_q_s0_bulk_dep_dn5_slot = var_q_s0_bulk_dep_dn5;
        *var_q_s0_bulk_dep_dn6_slot = var_q_s0_bulk_dep_dn6;
        *var_q_s0_bulk_dep_dn8_slot = var_q_s0_bulk_dep_dn8;
        *var_q_s0_bulk_dep_rv_slot = var_q_s0_bulk_dep_rv;
        *var_q_s0_bulk_dn0_slot = var_q_s0_bulk_dn0;
        *var_q_s0_bulk_dn10_slot = var_q_s0_bulk_dn10;
        *var_q_s0_bulk_dn11_slot = var_q_s0_bulk_dn11;
        *var_q_s0_bulk_dn12_slot = var_q_s0_bulk_dn12;
        *var_q_s0_bulk_dn2_slot = var_q_s0_bulk_dn2;
        *var_q_s0_bulk_dn4_slot = var_q_s0_bulk_dn4;
        *var_q_s0_bulk_dn5_slot = var_q_s0_bulk_dn5;
        *var_q_s0_bulk_dn6_slot = var_q_s0_bulk_dn6;
        *var_q_s0_bulk_dn8_slot = var_q_s0_bulk_dn8;
        *var_q_s0_bulk_rv_slot = var_q_s0_bulk_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn12_slot = var_t6_dn12;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_rv_slot = var_t6_rv;
        *var_t7_slot = var_t7;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn12_slot = var_t7_dn12;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_rv_slot = var_t7_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_x2_slot = var_x2;
        *var_x2_dn0_slot = var_x2_dn0;
        *var_x2_dn10_slot = var_x2_dn10;
        *var_x2_dn11_slot = var_x2_dn11;
        *var_x2_dn12_slot = var_x2_dn12;
        *var_x2_dn2_slot = var_x2_dn2;
        *var_x2_dn4_slot = var_x2_dn4;
        *var_x2_dn5_slot = var_x2_dn5;
        *var_x2_dn6_slot = var_x2_dn6;
        *var_x2_dn8_slot = var_x2_dn8;
        *var_x2_rv_slot = var_x2_rv;
        *var_xmax2_slot = var_xmax2;
        *var_xmax2_dn0_slot = var_xmax2_dn0;
        *var_xmax2_dn10_slot = var_xmax2_dn10;
        *var_xmax2_dn11_slot = var_xmax2_dn11;
        *var_xmax2_dn12_slot = var_xmax2_dn12;
        *var_xmax2_dn2_slot = var_xmax2_dn2;
        *var_xmax2_dn4_slot = var_xmax2_dn4;
        *var_xmax2_dn5_slot = var_xmax2_dn5;
        *var_xmax2_dn6_slot = var_xmax2_dn6;
        *var_xmax2_dn8_slot = var_xmax2_dn8;
        *var_xmax2_rv_slot = var_xmax2_rv;
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn11_slot = var_xp_dn11;
        *var_xp_dn12_slot = var_xp_dn12;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn4_slot = var_xp_dn4;
        *var_xp_dn5_slot = var_xp_dn5;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn8_slot = var_xp_dn8;
        *var_xp_rv_slot = var_xp_rv;
    }

    pub(super) fn stamp_reactive_block_34(
        p: &Parameters,
        var_dppg: f64,
        var_dppg_dn0: f64,
        var_dppg_dn10: f64,
        var_dppg_dn11: f64,
        var_dppg_dn12: f64,
        var_dppg_dn2: f64,
        var_dppg_dn4: f64,
        var_dppg_dn5: f64,
        var_dppg_dn6: f64,
        var_dppg_dn8: f64,
        var_dvth: f64,
        var_dvth_dn0: f64,
        var_dvth_dn10: f64,
        var_dvth_dn11: f64,
        var_dvth_dn12: f64,
        var_dvth_dn2: f64,
        var_dvth_dn4: f64,
        var_dvth_dn5: f64,
        var_dvth_dn6: f64,
        var_dvth_dn8: f64,
        var_guard118: f64,
        var_guard74: f64,
        var_phi_s0_soi: f64,
        var_phi_s0_soi_dn0: f64,
        var_phi_s0_soi_dn10: f64,
        var_phi_s0_soi_dn11: f64,
        var_phi_s0_soi_dn12: f64,
        var_phi_s0_soi_dn2: f64,
        var_phi_s0_soi_dn4: f64,
        var_phi_s0_soi_dn5: f64,
        var_phi_s0_soi_dn6: f64,
        var_phi_s0_soi_dn8: f64,
        var_tmf1: f64,
        var_tmf1_dn0: f64,
        var_tmf1_dn10: f64,
        var_tmf1_dn11: f64,
        var_tmf1_dn12: f64,
        var_tmf1_dn2: f64,
        var_tmf1_dn4: f64,
        var_tmf1_dn5: f64,
        var_tmf1_dn6: f64,
        var_tmf1_dn8: f64,
        var_vfbsub0: f64,
        var_vgs: f64,
        var_vgs_fb: f64,
        var_vgsz: f64,
        var_vgsz_dn0: f64,
        var_vgsz_dn10: f64,
        var_vgsz_dn11: f64,
        var_vgsz_dn12: f64,
        var_vgsz_dn2: f64,
        var_vgsz_dn4: f64,
        var_vgsz_dn5: f64,
        var_vgsz_dn6: f64,
        var_vgsz_dn8: f64,
        var_x2: f64,
        var_x2_dn0: f64,
        var_x2_dn10: f64,
        var_x2_dn11: f64,
        var_x2_dn12: f64,
        var_x2_dn2: f64,
        var_x2_dn4: f64,
        var_x2_dn5: f64,
        var_x2_dn6: f64,
        var_x2_dn8: f64,
        var_xmax2: f64,
        var_xmax2_dn0: f64,
        var_xmax2_dn10: f64,
        var_xmax2_dn11: f64,
        var_xmax2_dn12: f64,
        var_xmax2_dn2: f64,
        var_xmax2_dn4: f64,
        var_xmax2_dn5: f64,
        var_xmax2_dn6: f64,
        var_xmax2_dn8: f64,
        var_arg_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn10_slot: &mut f64,
        var_arg_dn11_slot: &mut f64,
        var_arg_dn12_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn11_slot: &mut f64,
        var_dnm_dn12_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn4_slot: &mut f64,
        var_dnm_dn5_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn8_slot: &mut f64,
        var_dnm_rv_slot: &mut f64,
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
        var_guard124_slot: &mut f64,
        var_guard124_rv_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_m0_rv_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_mm_rv_slot: &mut f64,
        var_phi_b0_soi_slot: &mut f64,
        var_phi_b0_soi_dn0_slot: &mut f64,
        var_phi_b0_soi_dn10_slot: &mut f64,
        var_phi_b0_soi_dn11_slot: &mut f64,
        var_phi_b0_soi_dn12_slot: &mut f64,
        var_phi_b0_soi_dn2_slot: &mut f64,
        var_phi_b0_soi_dn4_slot: &mut f64,
        var_phi_b0_soi_dn5_slot: &mut f64,
        var_phi_b0_soi_dn6_slot: &mut f64,
        var_phi_b0_soi_dn8_slot: &mut f64,
        var_phi_b0_soi_rv_slot: &mut f64,
        var_phi_b0_soip_slot: &mut f64,
        var_phi_b0_soip_dn0_slot: &mut f64,
        var_phi_b0_soip_dn10_slot: &mut f64,
        var_phi_b0_soip_dn11_slot: &mut f64,
        var_phi_b0_soip_dn12_slot: &mut f64,
        var_phi_b0_soip_dn2_slot: &mut f64,
        var_phi_b0_soip_dn4_slot: &mut f64,
        var_phi_b0_soip_dn5_slot: &mut f64,
        var_phi_b0_soip_dn6_slot: &mut f64,
        var_phi_b0_soip_dn8_slot: &mut f64,
        var_phi_b0_soip_rv_slot: &mut f64,
        var_sti2_dlt_slot: &mut f64,
        var_sti2_dlt_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_tmf0_slot: &mut f64,
        var_tmf0_dn0_slot: &mut f64,
        var_tmf0_dn10_slot: &mut f64,
        var_tmf0_dn11_slot: &mut f64,
        var_tmf0_dn12_slot: &mut f64,
        var_tmf0_dn2_slot: &mut f64,
        var_tmf0_dn4_slot: &mut f64,
        var_tmf0_dn5_slot: &mut f64,
        var_tmf0_dn6_slot: &mut f64,
        var_tmf0_dn8_slot: &mut f64,
        var_tmf0_rv_slot: &mut f64,
        var_vfbsub1_slot: &mut f64,
        var_vfbsub1_rv_slot: &mut f64,
        var_vgpsub_slot: &mut f64,
        var_vgpsub_dn0_slot: &mut f64,
        var_vgpsub_dn10_slot: &mut f64,
        var_vgpsub_dn11_slot: &mut f64,
        var_vgpsub_dn12_slot: &mut f64,
        var_vgpsub_dn2_slot: &mut f64,
        var_vgpsub_dn4_slot: &mut f64,
        var_vgpsub_dn5_slot: &mut f64,
        var_vgpsub_dn6_slot: &mut f64,
        var_vgpsub_dn8_slot: &mut f64,
        var_vgpsub_rv_slot: &mut f64,
        var_xmp_slot: &mut f64,
        var_xmp_dn0_slot: &mut f64,
        var_xmp_dn10_slot: &mut f64,
        var_xmp_dn11_slot: &mut f64,
        var_xmp_dn12_slot: &mut f64,
        var_xmp_dn2_slot: &mut f64,
        var_xmp_dn4_slot: &mut f64,
        var_xmp_dn5_slot: &mut f64,
        var_xmp_dn6_slot: &mut f64,
        var_xmp_dn8_slot: &mut f64,
        var_xmp_rv_slot: &mut f64,
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn11_slot: &mut f64,
        var_xp_dn12_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn4_slot: &mut f64,
        var_xp_dn5_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn8_slot: &mut f64,
        var_xp_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn10: f64 = *var_arg_dn10_slot;
        let mut var_arg_dn11: f64 = *var_arg_dn11_slot;
        let mut var_arg_dn12: f64 = *var_arg_dn12_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn11: f64 = *var_dnm_dn11_slot;
        let mut var_dnm_dn12: f64 = *var_dnm_dn12_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn4: f64 = *var_dnm_dn4_slot;
        let mut var_dnm_dn5: f64 = *var_dnm_dn5_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn8: f64 = *var_dnm_dn8_slot;
        let mut var_dnm_rv: f64 = *var_dnm_rv_slot;
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
        let mut var_guard124: f64 = *var_guard124_slot;
        let mut var_guard124_rv: f64 = *var_guard124_rv_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_m0_rv: f64 = *var_m0_rv_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_mm_rv: f64 = *var_mm_rv_slot;
        let mut var_phi_b0_soi: f64 = *var_phi_b0_soi_slot;
        let mut var_phi_b0_soi_dn0: f64 = *var_phi_b0_soi_dn0_slot;
        let mut var_phi_b0_soi_dn10: f64 = *var_phi_b0_soi_dn10_slot;
        let mut var_phi_b0_soi_dn11: f64 = *var_phi_b0_soi_dn11_slot;
        let mut var_phi_b0_soi_dn12: f64 = *var_phi_b0_soi_dn12_slot;
        let mut var_phi_b0_soi_dn2: f64 = *var_phi_b0_soi_dn2_slot;
        let mut var_phi_b0_soi_dn4: f64 = *var_phi_b0_soi_dn4_slot;
        let mut var_phi_b0_soi_dn5: f64 = *var_phi_b0_soi_dn5_slot;
        let mut var_phi_b0_soi_dn6: f64 = *var_phi_b0_soi_dn6_slot;
        let mut var_phi_b0_soi_dn8: f64 = *var_phi_b0_soi_dn8_slot;
        let mut var_phi_b0_soi_rv: f64 = *var_phi_b0_soi_rv_slot;
        let mut var_phi_b0_soip: f64 = *var_phi_b0_soip_slot;
        let mut var_phi_b0_soip_dn0: f64 = *var_phi_b0_soip_dn0_slot;
        let mut var_phi_b0_soip_dn10: f64 = *var_phi_b0_soip_dn10_slot;
        let mut var_phi_b0_soip_dn11: f64 = *var_phi_b0_soip_dn11_slot;
        let mut var_phi_b0_soip_dn12: f64 = *var_phi_b0_soip_dn12_slot;
        let mut var_phi_b0_soip_dn2: f64 = *var_phi_b0_soip_dn2_slot;
        let mut var_phi_b0_soip_dn4: f64 = *var_phi_b0_soip_dn4_slot;
        let mut var_phi_b0_soip_dn5: f64 = *var_phi_b0_soip_dn5_slot;
        let mut var_phi_b0_soip_dn6: f64 = *var_phi_b0_soip_dn6_slot;
        let mut var_phi_b0_soip_dn8: f64 = *var_phi_b0_soip_dn8_slot;
        let mut var_phi_b0_soip_rv: f64 = *var_phi_b0_soip_rv_slot;
        let mut var_sti2_dlt: f64 = *var_sti2_dlt_slot;
        let mut var_sti2_dlt_rv: f64 = *var_sti2_dlt_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_tmf0: f64 = *var_tmf0_slot;
        let mut var_tmf0_dn0: f64 = *var_tmf0_dn0_slot;
        let mut var_tmf0_dn10: f64 = *var_tmf0_dn10_slot;
        let mut var_tmf0_dn11: f64 = *var_tmf0_dn11_slot;
        let mut var_tmf0_dn12: f64 = *var_tmf0_dn12_slot;
        let mut var_tmf0_dn2: f64 = *var_tmf0_dn2_slot;
        let mut var_tmf0_dn4: f64 = *var_tmf0_dn4_slot;
        let mut var_tmf0_dn5: f64 = *var_tmf0_dn5_slot;
        let mut var_tmf0_dn6: f64 = *var_tmf0_dn6_slot;
        let mut var_tmf0_dn8: f64 = *var_tmf0_dn8_slot;
        let mut var_tmf0_rv: f64 = *var_tmf0_rv_slot;
        let mut var_vfbsub1: f64 = *var_vfbsub1_slot;
        let mut var_vfbsub1_rv: f64 = *var_vfbsub1_rv_slot;
        let mut var_vgpsub: f64 = *var_vgpsub_slot;
        let mut var_vgpsub_dn0: f64 = *var_vgpsub_dn0_slot;
        let mut var_vgpsub_dn10: f64 = *var_vgpsub_dn10_slot;
        let mut var_vgpsub_dn11: f64 = *var_vgpsub_dn11_slot;
        let mut var_vgpsub_dn12: f64 = *var_vgpsub_dn12_slot;
        let mut var_vgpsub_dn2: f64 = *var_vgpsub_dn2_slot;
        let mut var_vgpsub_dn4: f64 = *var_vgpsub_dn4_slot;
        let mut var_vgpsub_dn5: f64 = *var_vgpsub_dn5_slot;
        let mut var_vgpsub_dn6: f64 = *var_vgpsub_dn6_slot;
        let mut var_vgpsub_dn8: f64 = *var_vgpsub_dn8_slot;
        let mut var_vgpsub_rv: f64 = *var_vgpsub_rv_slot;
        let mut var_xmp: f64 = *var_xmp_slot;
        let mut var_xmp_dn0: f64 = *var_xmp_dn0_slot;
        let mut var_xmp_dn10: f64 = *var_xmp_dn10_slot;
        let mut var_xmp_dn11: f64 = *var_xmp_dn11_slot;
        let mut var_xmp_dn12: f64 = *var_xmp_dn12_slot;
        let mut var_xmp_dn2: f64 = *var_xmp_dn2_slot;
        let mut var_xmp_dn4: f64 = *var_xmp_dn4_slot;
        let mut var_xmp_dn5: f64 = *var_xmp_dn5_slot;
        let mut var_xmp_dn6: f64 = *var_xmp_dn6_slot;
        let mut var_xmp_dn8: f64 = *var_xmp_dn8_slot;
        let mut var_xmp_rv: f64 = *var_xmp_rv_slot;
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn11: f64 = *var_xp_dn11_slot;
        let mut var_xp_dn12: f64 = *var_xp_dn12_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn4: f64 = *var_xp_dn4_slot;
        let mut var_xp_dn5: f64 = *var_xp_dn5_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn8: f64 = *var_xp_dn8_slot;
        let mut var_xp_rv: f64 = *var_xp_rv_slot;

        let (assign8840_e9366, assign8840_e9366_d_n0, assign8840_e9366_d_n2, assign8840_e9366_d_n4, assign8840_e9366_d_n5, assign8840_e9366_d_n6, assign8840_e9366_d_n8, assign8840_e9366_d_n10, assign8840_e9366_d_n11, assign8840_e9366_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn8, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12,)
    }
};
        var_xmp = assign8840_e9366;
        var_xmp_dn0 = assign8840_e9366_d_n0;
        var_xmp_dn2 = assign8840_e9366_d_n2;
        var_xmp_dn4 = assign8840_e9366_d_n4;
        var_xmp_dn5 = assign8840_e9366_d_n5;
        var_xmp_dn6 = assign8840_e9366_d_n6;
        var_xmp_dn8 = assign8840_e9366_d_n8;
        var_xmp_dn10 = assign8840_e9366_d_n10;
        var_xmp_dn11 = assign8840_e9366_d_n11;
        var_xmp_dn12 = assign8840_e9366_d_n12;
        var_xmp_rv = 0.0;

        let (assign8850_e9373,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign8850_e9373;
        var_m0_rv = 0.0;

        let (assign8860_e9380,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign8860_e9380;
        var_mm_rv = 0.0;

        let (assign8870_e9387, assign8870_e9387_d_n0, assign8870_e9387_d_n2, assign8870_e9387_d_n4, assign8870_e9387_d_n5, assign8870_e9387_d_n6, assign8870_e9387_d_n8, assign8870_e9387_d_n10, assign8870_e9387_d_n11, assign8870_e9387_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn8, var_arg_dn10, var_arg_dn11, var_arg_dn12,)
    }
};
        var_arg = assign8870_e9387;
        var_arg_dn0 = assign8870_e9387_d_n0;
        var_arg_dn2 = assign8870_e9387_d_n2;
        var_arg_dn4 = assign8870_e9387_d_n4;
        var_arg_dn5 = assign8870_e9387_d_n5;
        var_arg_dn6 = assign8870_e9387_d_n6;
        var_arg_dn8 = assign8870_e9387_d_n8;
        var_arg_dn10 = assign8870_e9387_d_n10;
        var_arg_dn11 = assign8870_e9387_d_n11;
        var_arg_dn12 = assign8870_e9387_d_n12;
        var_arg_rv = 0.0;

        let (assign8880_e9394, assign8880_e9394_d_n0, assign8880_e9394_d_n2, assign8880_e9394_d_n4, assign8880_e9394_d_n5, assign8880_e9394_d_n6, assign8880_e9394_d_n8, assign8880_e9394_d_n10, assign8880_e9394_d_n11, assign8880_e9394_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn8, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12,)
    }
};
        var_dnm = assign8880_e9394;
        var_dnm_dn0 = assign8880_e9394_d_n0;
        var_dnm_dn2 = assign8880_e9394_d_n2;
        var_dnm_dn4 = assign8880_e9394_d_n4;
        var_dnm_dn5 = assign8880_e9394_d_n5;
        var_dnm_dn6 = assign8880_e9394_d_n6;
        var_dnm_dn8 = assign8880_e9394_d_n8;
        var_dnm_dn10 = assign8880_e9394_d_n10;
        var_dnm_dn11 = assign8880_e9394_d_n11;
        var_dnm_dn12 = assign8880_e9394_d_n12;
        var_dnm_rv = 0.0;

        let (assign8890_e9403, assign8890_e9403_d_n0, assign8890_e9403_d_n2, assign8890_e9403_d_n4, assign8890_e9403_d_n5, assign8890_e9403_d_n6, assign8890_e9403_d_n8, assign8890_e9403_d_n10, assign8890_e9403_d_n11, assign8890_e9403_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        let assign8890_e9401: f64 = (var_xp * var_x2);
        (assign8890_e9401, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn8, var_xp_dn10, var_xp_dn11, var_xp_dn12,)
    }
};
        var_xp = assign8890_e9403;
        var_xp_dn0 = assign8890_e9403_d_n0;
        var_xp_dn2 = assign8890_e9403_d_n2;
        var_xp_dn4 = assign8890_e9403_d_n4;
        var_xp_dn5 = assign8890_e9403_d_n5;
        var_xp_dn6 = assign8890_e9403_d_n6;
        var_xp_dn8 = assign8890_e9403_d_n8;
        var_xp_dn10 = assign8890_e9403_d_n10;
        var_xp_dn11 = assign8890_e9403_d_n11;
        var_xp_dn12 = assign8890_e9403_d_n12;
        var_xp_rv = 0.0;

        let (assign8900_e9412, assign8900_e9412_d_n0, assign8900_e9412_d_n2, assign8900_e9412_d_n4, assign8900_e9412_d_n5, assign8900_e9412_d_n6, assign8900_e9412_d_n8, assign8900_e9412_d_n10, assign8900_e9412_d_n11, assign8900_e9412_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        let assign8900_e9410: f64 = (var_xmp * var_xmax2);
        (assign8900_e9410, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn8, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12,)
    }
};
        var_xmp = assign8900_e9412;
        var_xmp_dn0 = assign8900_e9412_d_n0;
        var_xmp_dn2 = assign8900_e9412_d_n2;
        var_xmp_dn4 = assign8900_e9412_d_n4;
        var_xmp_dn5 = assign8900_e9412_d_n5;
        var_xmp_dn6 = assign8900_e9412_d_n6;
        var_xmp_dn8 = assign8900_e9412_d_n8;
        var_xmp_dn10 = assign8900_e9412_d_n10;
        var_xmp_dn11 = assign8900_e9412_d_n11;
        var_xmp_dn12 = assign8900_e9412_d_n12;
        var_xmp_rv = 0.0;

        let (assign8910_e9421, assign8910_e9421_d_n0, assign8910_e9421_d_n2, assign8910_e9421_d_n4, assign8910_e9421_d_n5, assign8910_e9421_d_n6, assign8910_e9421_d_n8, assign8910_e9421_d_n10, assign8910_e9421_d_n11, assign8910_e9421_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        let assign8910_e9419: f64 = (var_xp + var_xmp);
        (assign8910_e9419, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn4 + var_xmp_dn4), (var_xp_dn5 + var_xmp_dn5), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn8 + var_xmp_dn8), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn11 + var_xmp_dn11), (var_xp_dn12 + var_xmp_dn12),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn8, var_arg_dn10, var_arg_dn11, var_arg_dn12,)
    }
};
        var_arg = assign8910_e9421;
        var_arg_dn0 = assign8910_e9421_d_n0;
        var_arg_dn2 = assign8910_e9421_d_n2;
        var_arg_dn4 = assign8910_e9421_d_n4;
        var_arg_dn5 = assign8910_e9421_d_n5;
        var_arg_dn6 = assign8910_e9421_d_n6;
        var_arg_dn8 = assign8910_e9421_d_n8;
        var_arg_dn10 = assign8910_e9421_d_n10;
        var_arg_dn11 = assign8910_e9421_d_n11;
        var_arg_dn12 = assign8910_e9421_d_n12;
        var_arg_rv = 0.0;

        let (assign8920_e9428, assign8920_e9428_d_n0, assign8920_e9428_d_n2, assign8920_e9428_d_n4, assign8920_e9428_d_n5, assign8920_e9428_d_n6, assign8920_e9428_d_n8, assign8920_e9428_d_n10, assign8920_e9428_d_n11, assign8920_e9428_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn8, var_arg_dn10, var_arg_dn11, var_arg_dn12,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn8, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12,)
    }
};
        var_dnm = assign8920_e9428;
        var_dnm_dn0 = assign8920_e9428_d_n0;
        var_dnm_dn2 = assign8920_e9428_d_n2;
        var_dnm_dn4 = assign8920_e9428_d_n4;
        var_dnm_dn5 = assign8920_e9428_d_n5;
        var_dnm_dn6 = assign8920_e9428_d_n6;
        var_dnm_dn8 = assign8920_e9428_d_n8;
        var_dnm_dn10 = assign8920_e9428_d_n10;
        var_dnm_dn11 = assign8920_e9428_d_n11;
        var_dnm_dn12 = assign8920_e9428_d_n12;
        var_dnm_rv = 0.0;

        let assign8930_e9443: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard119 = assign8930_e9443;
        var_guard119_rv = 0.0;

        let assign8940_e9446: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard120 = assign8940_e9446;
        var_guard120_rv = 0.0;

        let (assign8950_e9457,) = {
    if ((((var_guard74 == 0.0) && (var_guard118 != 0.0)) && (var_guard119 != 0.0)) && (var_guard120 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign8950_e9457;
        var_mm_rv = 0.0;

        let assign8960_e9460: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard121 = assign8960_e9460;
        var_guard121_rv = 0.0;

        let (assign8970_e9474,) = {
    if (((((var_guard74 == 0.0) && (var_guard118 != 0.0)) && (var_guard119 != 0.0)) && (var_guard120 == 0.0)) && (var_guard121 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign8970_e9474;
        var_mm_rv = 0.0;

        let assign8980_e9477: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard122 = assign8980_e9477;
        var_guard122_rv = 0.0;

        let (assign8990_e9494,) = {
    if ((((((var_guard74 == 0.0) && (var_guard118 != 0.0)) && (var_guard119 != 0.0)) && (var_guard120 == 0.0)) && (var_guard121 == 0.0)) && (var_guard122 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign8990_e9494;
        var_mm_rv = 0.0;

        let assign9000_e9497: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard123 = assign9000_e9497;
        var_guard123_rv = 0.0;

        let (assign9010_e9517,) = {
    if (((((((var_guard74 == 0.0) && (var_guard118 != 0.0)) && (var_guard119 != 0.0)) && (var_guard120 == 0.0)) && (var_guard121 == 0.0)) && (var_guard122 == 0.0)) && (var_guard123 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign9010_e9517;
        var_mm_rv = 0.0;

        let (assign9020_e9526,) = {
    if (((var_guard74 == 0.0) && (var_guard118 != 0.0)) && (var_guard119 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign9020_e9526;
        var_m0_rv = 0.0;

        let mut assign9030_loop_guard: usize = 0;
        while {
            let assign9030_cond_e9536: f64 = if ((((var_guard74 == 0.0) && (var_guard118 != 0.0)) && (var_guard119 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign9030_cond_e9536 != 0.0
        } {
            assign9030_loop_guard += 1;
            assert!(assign9030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9030_body0_e9546, assign9030_body0_e9546_d_n0, assign9030_body0_e9546_d_n2, assign9030_body0_e9546_d_n4, assign9030_body0_e9546_d_n5, assign9030_body0_e9546_d_n6, assign9030_body0_e9546_d_n8, assign9030_body0_e9546_d_n10, assign9030_body0_e9546_d_n11, assign9030_body0_e9546_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard118 != 0.0)) && (var_guard119 != 0.0)) {
        let assign9030_body0_e9544: f64 = (var_dnm).sqrt();
        (assign9030_body0_e9544, (var_dnm_dn0 / (2.0 * assign9030_body0_e9544)), (var_dnm_dn2 / (2.0 * assign9030_body0_e9544)), (var_dnm_dn4 / (2.0 * assign9030_body0_e9544)), (var_dnm_dn5 / (2.0 * assign9030_body0_e9544)), (var_dnm_dn6 / (2.0 * assign9030_body0_e9544)), (var_dnm_dn8 / (2.0 * assign9030_body0_e9544)), (var_dnm_dn10 / (2.0 * assign9030_body0_e9544)), (var_dnm_dn11 / (2.0 * assign9030_body0_e9544)), (var_dnm_dn12 / (2.0 * assign9030_body0_e9544)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn8, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12,)
    }
};
            var_dnm = assign9030_body0_e9546;
            var_dnm_dn0 = assign9030_body0_e9546_d_n0;
            var_dnm_dn2 = assign9030_body0_e9546_d_n2;
            var_dnm_dn4 = assign9030_body0_e9546_d_n4;
            var_dnm_dn5 = assign9030_body0_e9546_d_n5;
            var_dnm_dn6 = assign9030_body0_e9546_d_n6;
            var_dnm_dn8 = assign9030_body0_e9546_d_n8;
            var_dnm_dn10 = assign9030_body0_e9546_d_n10;
            var_dnm_dn11 = assign9030_body0_e9546_d_n11;
            var_dnm_dn12 = assign9030_body0_e9546_d_n12;
            var_dnm_rv = 0.0;
            let (assign9030_body1_e9557,) = {
    if (((var_guard74 == 0.0) && (var_guard118 != 0.0)) && (var_guard119 != 0.0)) {
        let assign9030_body1_e9555: f64 = (var_m0 + 1.0);
        (assign9030_body1_e9555,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign9030_body1_e9557;
            var_m0_rv = 0.0;
        }

        let (assign9040_e9573, assign9040_e9573_d_n0, assign9040_e9573_d_n2, assign9040_e9573_d_n4, assign9040_e9573_d_n5, assign9040_e9573_d_n6, assign9040_e9573_d_n8, assign9040_e9573_d_n10, assign9040_e9573_d_n11, assign9040_e9573_d_n12,) = {
    if (((var_guard74 == 0.0) && (var_guard118 != 0.0)) && (var_guard119 == 0.0)) {
        let assign9040_e9569: f64 = 2.0;
        let assign9040_e9570: f64 = (1.0 / assign9040_e9569);
        let assign9040_e9571: f64 = (var_dnm).powf(assign9040_e9570);
        (assign9040_e9571, if 0.0 == 0.0 && ((assign9040_e9570) as f64).is_finite() && ((assign9040_e9570) as f64).fract() == 0.0 { if assign9040_e9570 == 0.0 { 0.0 } else { (assign9040_e9570 * ((var_dnm).powf(assign9040_e9570 - 1.0) * var_dnm_dn0)) } } else { (assign9040_e9571 * (assign9040_e9570 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign9040_e9570) as f64).is_finite() && ((assign9040_e9570) as f64).fract() == 0.0 { if assign9040_e9570 == 0.0 { 0.0 } else { (assign9040_e9570 * ((var_dnm).powf(assign9040_e9570 - 1.0) * var_dnm_dn2)) } } else { (assign9040_e9571 * (assign9040_e9570 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign9040_e9570) as f64).is_finite() && ((assign9040_e9570) as f64).fract() == 0.0 { if assign9040_e9570 == 0.0 { 0.0 } else { (assign9040_e9570 * ((var_dnm).powf(assign9040_e9570 - 1.0) * var_dnm_dn4)) } } else { (assign9040_e9571 * (assign9040_e9570 * (var_dnm_dn4 / var_dnm))) }, if 0.0 == 0.0 && ((assign9040_e9570) as f64).is_finite() && ((assign9040_e9570) as f64).fract() == 0.0 { if assign9040_e9570 == 0.0 { 0.0 } else { (assign9040_e9570 * ((var_dnm).powf(assign9040_e9570 - 1.0) * var_dnm_dn5)) } } else { (assign9040_e9571 * (assign9040_e9570 * (var_dnm_dn5 / var_dnm))) }, if 0.0 == 0.0 && ((assign9040_e9570) as f64).is_finite() && ((assign9040_e9570) as f64).fract() == 0.0 { if assign9040_e9570 == 0.0 { 0.0 } else { (assign9040_e9570 * ((var_dnm).powf(assign9040_e9570 - 1.0) * var_dnm_dn6)) } } else { (assign9040_e9571 * (assign9040_e9570 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign9040_e9570) as f64).is_finite() && ((assign9040_e9570) as f64).fract() == 0.0 { if assign9040_e9570 == 0.0 { 0.0 } else { (assign9040_e9570 * ((var_dnm).powf(assign9040_e9570 - 1.0) * var_dnm_dn8)) } } else { (assign9040_e9571 * (assign9040_e9570 * (var_dnm_dn8 / var_dnm))) }, if 0.0 == 0.0 && ((assign9040_e9570) as f64).is_finite() && ((assign9040_e9570) as f64).fract() == 0.0 { if assign9040_e9570 == 0.0 { 0.0 } else { (assign9040_e9570 * ((var_dnm).powf(assign9040_e9570 - 1.0) * var_dnm_dn10)) } } else { (assign9040_e9571 * (assign9040_e9570 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign9040_e9570) as f64).is_finite() && ((assign9040_e9570) as f64).fract() == 0.0 { if assign9040_e9570 == 0.0 { 0.0 } else { (assign9040_e9570 * ((var_dnm).powf(assign9040_e9570 - 1.0) * var_dnm_dn11)) } } else { (assign9040_e9571 * (assign9040_e9570 * (var_dnm_dn11 / var_dnm))) }, if 0.0 == 0.0 && ((assign9040_e9570) as f64).is_finite() && ((assign9040_e9570) as f64).fract() == 0.0 { if assign9040_e9570 == 0.0 { 0.0 } else { (assign9040_e9570 * ((var_dnm).powf(assign9040_e9570 - 1.0) * var_dnm_dn12)) } } else { (assign9040_e9571 * (assign9040_e9570 * (var_dnm_dn12 / var_dnm))) },)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn8, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12,)
    }
};
        var_dnm = assign9040_e9573;
        var_dnm_dn0 = assign9040_e9573_d_n0;
        var_dnm_dn2 = assign9040_e9573_d_n2;
        var_dnm_dn4 = assign9040_e9573_d_n4;
        var_dnm_dn5 = assign9040_e9573_d_n5;
        var_dnm_dn6 = assign9040_e9573_d_n6;
        var_dnm_dn8 = assign9040_e9573_d_n8;
        var_dnm_dn10 = assign9040_e9573_d_n10;
        var_dnm_dn11 = assign9040_e9573_d_n11;
        var_dnm_dn12 = assign9040_e9573_d_n12;
        var_dnm_rv = 0.0;

        let (assign9050_e9584, assign9050_e9584_d_n0, assign9050_e9584_d_n2, assign9050_e9584_d_n4, assign9050_e9584_d_n5, assign9050_e9584_d_n6, assign9050_e9584_d_n8, assign9050_e9584_d_n10, assign9050_e9584_d_n11, assign9050_e9584_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        let assign9050_e9581: f64 = (var_dnm + 1e-50);
        let assign9050_e9582: f64 = (1.0 / assign9050_e9581);
        (assign9050_e9582, (-(var_dnm_dn0 / (assign9050_e9581 * assign9050_e9581))), (-(var_dnm_dn2 / (assign9050_e9581 * assign9050_e9581))), (-(var_dnm_dn4 / (assign9050_e9581 * assign9050_e9581))), (-(var_dnm_dn5 / (assign9050_e9581 * assign9050_e9581))), (-(var_dnm_dn6 / (assign9050_e9581 * assign9050_e9581))), (-(var_dnm_dn8 / (assign9050_e9581 * assign9050_e9581))), (-(var_dnm_dn10 / (assign9050_e9581 * assign9050_e9581))), (-(var_dnm_dn11 / (assign9050_e9581 * assign9050_e9581))), (-(var_dnm_dn12 / (assign9050_e9581 * assign9050_e9581))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn8, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12,)
    }
};
        var_dnm = assign9050_e9584;
        var_dnm_dn0 = assign9050_e9584_d_n0;
        var_dnm_dn2 = assign9050_e9584_d_n2;
        var_dnm_dn4 = assign9050_e9584_d_n4;
        var_dnm_dn5 = assign9050_e9584_d_n5;
        var_dnm_dn6 = assign9050_e9584_d_n6;
        var_dnm_dn8 = assign9050_e9584_d_n8;
        var_dnm_dn10 = assign9050_e9584_d_n10;
        var_dnm_dn11 = assign9050_e9584_d_n11;
        var_dnm_dn12 = assign9050_e9584_d_n12;
        var_dnm_rv = 0.0;

        let (assign9060_e9595, assign9060_e9595_d_n0, assign9060_e9595_d_n2, assign9060_e9595_d_n4, assign9060_e9595_d_n5, assign9060_e9595_d_n6, assign9060_e9595_d_n8, assign9060_e9595_d_n10, assign9060_e9595_d_n11, assign9060_e9595_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        let assign9060_e9591: f64 = (var_tmf1 * 0.15);
        let assign9060_e9593: f64 = (assign9060_e9591 * var_dnm);
        (assign9060_e9593, (((var_tmf1_dn0 * 0.15) * var_dnm) + (assign9060_e9591 * var_dnm_dn0)), (((var_tmf1_dn2 * 0.15) * var_dnm) + (assign9060_e9591 * var_dnm_dn2)), (((var_tmf1_dn4 * 0.15) * var_dnm) + (assign9060_e9591 * var_dnm_dn4)), (((var_tmf1_dn5 * 0.15) * var_dnm) + (assign9060_e9591 * var_dnm_dn5)), (((var_tmf1_dn6 * 0.15) * var_dnm) + (assign9060_e9591 * var_dnm_dn6)), (((var_tmf1_dn8 * 0.15) * var_dnm) + (assign9060_e9591 * var_dnm_dn8)), (((var_tmf1_dn10 * 0.15) * var_dnm) + (assign9060_e9591 * var_dnm_dn10)), (((var_tmf1_dn11 * 0.15) * var_dnm) + (assign9060_e9591 * var_dnm_dn11)), (((var_tmf1_dn12 * 0.15) * var_dnm) + (assign9060_e9591 * var_dnm_dn12)),)
    } else {
        (var_tmf0, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn4, var_tmf0_dn5, var_tmf0_dn6, var_tmf0_dn8, var_tmf0_dn10, var_tmf0_dn11, var_tmf0_dn12,)
    }
};
        var_tmf0 = assign9060_e9595;
        var_tmf0_dn0 = assign9060_e9595_d_n0;
        var_tmf0_dn2 = assign9060_e9595_d_n2;
        var_tmf0_dn4 = assign9060_e9595_d_n4;
        var_tmf0_dn5 = assign9060_e9595_d_n5;
        var_tmf0_dn6 = assign9060_e9595_d_n6;
        var_tmf0_dn8 = assign9060_e9595_d_n8;
        var_tmf0_dn10 = assign9060_e9595_d_n10;
        var_tmf0_dn11 = assign9060_e9595_d_n11;
        var_tmf0_dn12 = assign9060_e9595_d_n12;
        var_tmf0_rv = 0.0;

        let (assign9070_e9610, assign9070_e9610_d_n0, assign9070_e9610_d_n2, assign9070_e9610_d_n4, assign9070_e9610_d_n5, assign9070_e9610_d_n6, assign9070_e9610_d_n8, assign9070_e9610_d_n10, assign9070_e9610_d_n11, assign9070_e9610_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        let assign9070_e9602: f64 = (0.15 * var_xmp);
        let assign9070_e9604: f64 = (assign9070_e9602 * var_dnm);
        let assign9070_e9607: f64 = (var_arg + 1e-50);
        let assign9070_e9608: f64 = (assign9070_e9604 / assign9070_e9607);
        (assign9070_e9608, ((((((0.15 * var_xmp_dn0) * var_dnm) + (assign9070_e9602 * var_dnm_dn0)) * assign9070_e9607) - (assign9070_e9604 * var_arg_dn0)) / (assign9070_e9607 * assign9070_e9607)), ((((((0.15 * var_xmp_dn2) * var_dnm) + (assign9070_e9602 * var_dnm_dn2)) * assign9070_e9607) - (assign9070_e9604 * var_arg_dn2)) / (assign9070_e9607 * assign9070_e9607)), ((((((0.15 * var_xmp_dn4) * var_dnm) + (assign9070_e9602 * var_dnm_dn4)) * assign9070_e9607) - (assign9070_e9604 * var_arg_dn4)) / (assign9070_e9607 * assign9070_e9607)), ((((((0.15 * var_xmp_dn5) * var_dnm) + (assign9070_e9602 * var_dnm_dn5)) * assign9070_e9607) - (assign9070_e9604 * var_arg_dn5)) / (assign9070_e9607 * assign9070_e9607)), ((((((0.15 * var_xmp_dn6) * var_dnm) + (assign9070_e9602 * var_dnm_dn6)) * assign9070_e9607) - (assign9070_e9604 * var_arg_dn6)) / (assign9070_e9607 * assign9070_e9607)), ((((((0.15 * var_xmp_dn8) * var_dnm) + (assign9070_e9602 * var_dnm_dn8)) * assign9070_e9607) - (assign9070_e9604 * var_arg_dn8)) / (assign9070_e9607 * assign9070_e9607)), ((((((0.15 * var_xmp_dn10) * var_dnm) + (assign9070_e9602 * var_dnm_dn10)) * assign9070_e9607) - (assign9070_e9604 * var_arg_dn10)) / (assign9070_e9607 * assign9070_e9607)), ((((((0.15 * var_xmp_dn11) * var_dnm) + (assign9070_e9602 * var_dnm_dn11)) * assign9070_e9607) - (assign9070_e9604 * var_arg_dn11)) / (assign9070_e9607 * assign9070_e9607)), ((((((0.15 * var_xmp_dn12) * var_dnm) + (assign9070_e9602 * var_dnm_dn12)) * assign9070_e9607) - (assign9070_e9604 * var_arg_dn12)) / (assign9070_e9607 * assign9070_e9607)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
        var_t0 = assign9070_e9610;
        var_t0_dn0 = assign9070_e9610_d_n0;
        var_t0_dn2 = assign9070_e9610_d_n2;
        var_t0_dn4 = assign9070_e9610_d_n4;
        var_t0_dn5 = assign9070_e9610_d_n5;
        var_t0_dn6 = assign9070_e9610_d_n6;
        var_t0_dn8 = assign9070_e9610_d_n8;
        var_t0_dn10 = assign9070_e9610_d_n10;
        var_t0_dn11 = assign9070_e9610_d_n11;
        var_t0_dn12 = assign9070_e9610_d_n12;
        var_t0_rv = 0.0;

        let (assign9080_e9621, assign9080_e9621_d_n0, assign9080_e9621_d_n2, assign9080_e9621_d_n4, assign9080_e9621_d_n5, assign9080_e9621_d_n6, assign9080_e9621_d_n8, assign9080_e9621_d_n10, assign9080_e9621_d_n11, assign9080_e9621_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        let assign9080_e9617: f64 = (var_phi_s0_soi - 0.15);
        let assign9080_e9619: f64 = (assign9080_e9617 + var_tmf0);
        (assign9080_e9619, (var_phi_s0_soi_dn0 + var_tmf0_dn0), (var_phi_s0_soi_dn2 + var_tmf0_dn2), (var_phi_s0_soi_dn4 + var_tmf0_dn4), (var_phi_s0_soi_dn5 + var_tmf0_dn5), (var_phi_s0_soi_dn6 + var_tmf0_dn6), (var_phi_s0_soi_dn8 + var_tmf0_dn8), (var_phi_s0_soi_dn10 + var_tmf0_dn10), (var_phi_s0_soi_dn11 + var_tmf0_dn11), (var_phi_s0_soi_dn12 + var_tmf0_dn12),)
    } else {
        (var_phi_b0_soi, var_phi_b0_soi_dn0, var_phi_b0_soi_dn2, var_phi_b0_soi_dn4, var_phi_b0_soi_dn5, var_phi_b0_soi_dn6, var_phi_b0_soi_dn8, var_phi_b0_soi_dn10, var_phi_b0_soi_dn11, var_phi_b0_soi_dn12,)
    }
};
        var_phi_b0_soi = assign9080_e9621;
        var_phi_b0_soi_dn0 = assign9080_e9621_d_n0;
        var_phi_b0_soi_dn2 = assign9080_e9621_d_n2;
        var_phi_b0_soi_dn4 = assign9080_e9621_d_n4;
        var_phi_b0_soi_dn5 = assign9080_e9621_d_n5;
        var_phi_b0_soi_dn6 = assign9080_e9621_d_n6;
        var_phi_b0_soi_dn8 = assign9080_e9621_d_n8;
        var_phi_b0_soi_dn10 = assign9080_e9621_d_n10;
        var_phi_b0_soi_dn11 = assign9080_e9621_d_n11;
        var_phi_b0_soi_dn12 = assign9080_e9621_d_n12;
        var_phi_b0_soi_rv = 0.0;

        let (assign9090_e9628, assign9090_e9628_d_n0, assign9090_e9628_d_n2, assign9090_e9628_d_n4, assign9090_e9628_d_n5, assign9090_e9628_d_n6, assign9090_e9628_d_n8, assign9090_e9628_d_n10, assign9090_e9628_d_n11, assign9090_e9628_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
        var_t0 = assign9090_e9628;
        var_t0_dn0 = assign9090_e9628_d_n0;
        var_t0_dn2 = assign9090_e9628_d_n2;
        var_t0_dn4 = assign9090_e9628_d_n4;
        var_t0_dn5 = assign9090_e9628_d_n5;
        var_t0_dn6 = assign9090_e9628_d_n6;
        var_t0_dn8 = assign9090_e9628_d_n8;
        var_t0_dn10 = assign9090_e9628_d_n10;
        var_t0_dn11 = assign9090_e9628_d_n11;
        var_t0_dn12 = assign9090_e9628_d_n12;
        var_t0_rv = 0.0;

        let (assign9100_e9636, assign9100_e9636_d_n0, assign9100_e9636_d_n2, assign9100_e9636_d_n4, assign9100_e9636_d_n5, assign9100_e9636_d_n6, assign9100_e9636_d_n8, assign9100_e9636_d_n10, assign9100_e9636_d_n11, assign9100_e9636_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 == 0.0)) {
        (var_phi_b0_soi, var_phi_b0_soi_dn0, var_phi_b0_soi_dn2, var_phi_b0_soi_dn4, var_phi_b0_soi_dn5, var_phi_b0_soi_dn6, var_phi_b0_soi_dn8, var_phi_b0_soi_dn10, var_phi_b0_soi_dn11, var_phi_b0_soi_dn12,)
    } else {
        (var_phi_b0_soi, var_phi_b0_soi_dn0, var_phi_b0_soi_dn2, var_phi_b0_soi_dn4, var_phi_b0_soi_dn5, var_phi_b0_soi_dn6, var_phi_b0_soi_dn8, var_phi_b0_soi_dn10, var_phi_b0_soi_dn11, var_phi_b0_soi_dn12,)
    }
};
        var_phi_b0_soi = assign9100_e9636;
        var_phi_b0_soi_dn0 = assign9100_e9636_d_n0;
        var_phi_b0_soi_dn2 = assign9100_e9636_d_n2;
        var_phi_b0_soi_dn4 = assign9100_e9636_d_n4;
        var_phi_b0_soi_dn5 = assign9100_e9636_d_n5;
        var_phi_b0_soi_dn6 = assign9100_e9636_d_n6;
        var_phi_b0_soi_dn8 = assign9100_e9636_d_n8;
        var_phi_b0_soi_dn10 = assign9100_e9636_d_n10;
        var_phi_b0_soi_dn11 = assign9100_e9636_d_n11;
        var_phi_b0_soi_dn12 = assign9100_e9636_d_n12;
        var_phi_b0_soi_rv = 0.0;

        let (assign9110_e9644, assign9110_e9644_d_n0, assign9110_e9644_d_n2, assign9110_e9644_d_n4, assign9110_e9644_d_n5, assign9110_e9644_d_n6, assign9110_e9644_d_n8, assign9110_e9644_d_n10, assign9110_e9644_d_n11, assign9110_e9644_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard118 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
        var_t0 = assign9110_e9644;
        var_t0_dn0 = assign9110_e9644_d_n0;
        var_t0_dn2 = assign9110_e9644_d_n2;
        var_t0_dn4 = assign9110_e9644_d_n4;
        var_t0_dn5 = assign9110_e9644_d_n5;
        var_t0_dn6 = assign9110_e9644_d_n6;
        var_t0_dn8 = assign9110_e9644_d_n8;
        var_t0_dn10 = assign9110_e9644_d_n10;
        var_t0_dn11 = assign9110_e9644_d_n11;
        var_t0_dn12 = assign9110_e9644_d_n12;
        var_t0_rv = 0.0;

        let (assign9120_e9649, assign9120_e9649_d_n0, assign9120_e9649_d_n2, assign9120_e9649_d_n4, assign9120_e9649_d_n5, assign9120_e9649_d_n6, assign9120_e9649_d_n8, assign9120_e9649_d_n10, assign9120_e9649_d_n11, assign9120_e9649_d_n12,) = {
    if (var_guard74 == 0.0) {
        (var_phi_b0_soi, var_phi_b0_soi_dn0, var_phi_b0_soi_dn2, var_phi_b0_soi_dn4, var_phi_b0_soi_dn5, var_phi_b0_soi_dn6, var_phi_b0_soi_dn8, var_phi_b0_soi_dn10, var_phi_b0_soi_dn11, var_phi_b0_soi_dn12,)
    } else {
        (var_phi_b0_soip, var_phi_b0_soip_dn0, var_phi_b0_soip_dn2, var_phi_b0_soip_dn4, var_phi_b0_soip_dn5, var_phi_b0_soip_dn6, var_phi_b0_soip_dn8, var_phi_b0_soip_dn10, var_phi_b0_soip_dn11, var_phi_b0_soip_dn12,)
    }
};
        var_phi_b0_soip = assign9120_e9649;
        var_phi_b0_soip_dn0 = assign9120_e9649_d_n0;
        var_phi_b0_soip_dn2 = assign9120_e9649_d_n2;
        var_phi_b0_soip_dn4 = assign9120_e9649_d_n4;
        var_phi_b0_soip_dn5 = assign9120_e9649_d_n5;
        var_phi_b0_soip_dn6 = assign9120_e9649_d_n6;
        var_phi_b0_soip_dn8 = assign9120_e9649_d_n8;
        var_phi_b0_soip_dn10 = assign9120_e9649_d_n10;
        var_phi_b0_soip_dn11 = assign9120_e9649_d_n11;
        var_phi_b0_soip_dn12 = assign9120_e9649_d_n12;
        var_phi_b0_soip_rv = 0.0;

        let assign9130_e9656: f64 = (var_vgs_fb + 0.2);
        let assign9130_e9658: f64 = if ((p.p15 == 1.0) && (var_vgs > assign9130_e9656)) { 1.0 } else { 0.0 };
        var_guard124 = assign9130_e9658;
        var_guard124_rv = 0.0;

        let (assign9140_e9662,) = {
    if (var_guard124 != 0.0) {
        (var_vfbsub0,)
    } else {
        (var_vfbsub1,)
    }
};
        var_vfbsub1 = assign9140_e9662;
        var_vfbsub1_rv = 0.0;

        let (assign9150_e9672, assign9150_e9672_d_n0, assign9150_e9672_d_n2, assign9150_e9672_d_n4, assign9150_e9672_d_n5, assign9150_e9672_d_n6, assign9150_e9672_d_n8, assign9150_e9672_d_n10, assign9150_e9672_d_n11, assign9150_e9672_d_n12,) = {
    if (var_guard124 != 0.0) {
        let assign9150_e9666: f64 = (var_vgsz - var_vfbsub1);
        let assign9150_e9668: f64 = (assign9150_e9666 + var_dvth);
        let assign9150_e9670: f64 = (assign9150_e9668 - var_dppg);
        (assign9150_e9670, ((var_vgsz_dn0 + var_dvth_dn0) - var_dppg_dn0), ((var_vgsz_dn2 + var_dvth_dn2) - var_dppg_dn2), ((var_vgsz_dn4 + var_dvth_dn4) - var_dppg_dn4), ((var_vgsz_dn5 + var_dvth_dn5) - var_dppg_dn5), ((var_vgsz_dn6 + var_dvth_dn6) - var_dppg_dn6), ((var_vgsz_dn8 + var_dvth_dn8) - var_dppg_dn8), ((var_vgsz_dn10 + var_dvth_dn10) - var_dppg_dn10), ((var_vgsz_dn11 + var_dvth_dn11) - var_dppg_dn11), ((var_vgsz_dn12 + var_dvth_dn12) - var_dppg_dn12),)
    } else {
        (var_vgpsub, var_vgpsub_dn0, var_vgpsub_dn2, var_vgpsub_dn4, var_vgpsub_dn5, var_vgpsub_dn6, var_vgpsub_dn8, var_vgpsub_dn10, var_vgpsub_dn11, var_vgpsub_dn12,)
    }
};
        var_vgpsub = assign9150_e9672;
        var_vgpsub_dn0 = assign9150_e9672_d_n0;
        var_vgpsub_dn2 = assign9150_e9672_d_n2;
        var_vgpsub_dn4 = assign9150_e9672_d_n4;
        var_vgpsub_dn5 = assign9150_e9672_d_n5;
        var_vgpsub_dn6 = assign9150_e9672_d_n6;
        var_vgpsub_dn8 = assign9150_e9672_d_n8;
        var_vgpsub_dn10 = assign9150_e9672_d_n10;
        var_vgpsub_dn11 = assign9150_e9672_d_n11;
        var_vgpsub_dn12 = assign9150_e9672_d_n12;
        var_vgpsub_rv = 0.0;

        let (assign9160_e9676,) = {
    if (var_guard124 != 0.0) {
        (p.p136,)
    } else {
        (var_sti2_dlt,)
    }
};
        var_sti2_dlt = assign9160_e9676;
        var_sti2_dlt_rv = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn10_slot = var_arg_dn10;
        *var_arg_dn11_slot = var_arg_dn11;
        *var_arg_dn12_slot = var_arg_dn12;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_rv_slot = var_arg_rv;
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn11_slot = var_dnm_dn11;
        *var_dnm_dn12_slot = var_dnm_dn12;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn4_slot = var_dnm_dn4;
        *var_dnm_dn5_slot = var_dnm_dn5;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn8_slot = var_dnm_dn8;
        *var_dnm_rv_slot = var_dnm_rv;
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
        *var_guard124_slot = var_guard124;
        *var_guard124_rv_slot = var_guard124_rv;
        *var_m0_slot = var_m0;
        *var_m0_rv_slot = var_m0_rv;
        *var_mm_slot = var_mm;
        *var_mm_rv_slot = var_mm_rv;
        *var_phi_b0_soi_slot = var_phi_b0_soi;
        *var_phi_b0_soi_dn0_slot = var_phi_b0_soi_dn0;
        *var_phi_b0_soi_dn10_slot = var_phi_b0_soi_dn10;
        *var_phi_b0_soi_dn11_slot = var_phi_b0_soi_dn11;
        *var_phi_b0_soi_dn12_slot = var_phi_b0_soi_dn12;
        *var_phi_b0_soi_dn2_slot = var_phi_b0_soi_dn2;
        *var_phi_b0_soi_dn4_slot = var_phi_b0_soi_dn4;
        *var_phi_b0_soi_dn5_slot = var_phi_b0_soi_dn5;
        *var_phi_b0_soi_dn6_slot = var_phi_b0_soi_dn6;
        *var_phi_b0_soi_dn8_slot = var_phi_b0_soi_dn8;
        *var_phi_b0_soi_rv_slot = var_phi_b0_soi_rv;
        *var_phi_b0_soip_slot = var_phi_b0_soip;
        *var_phi_b0_soip_dn0_slot = var_phi_b0_soip_dn0;
        *var_phi_b0_soip_dn10_slot = var_phi_b0_soip_dn10;
        *var_phi_b0_soip_dn11_slot = var_phi_b0_soip_dn11;
        *var_phi_b0_soip_dn12_slot = var_phi_b0_soip_dn12;
        *var_phi_b0_soip_dn2_slot = var_phi_b0_soip_dn2;
        *var_phi_b0_soip_dn4_slot = var_phi_b0_soip_dn4;
        *var_phi_b0_soip_dn5_slot = var_phi_b0_soip_dn5;
        *var_phi_b0_soip_dn6_slot = var_phi_b0_soip_dn6;
        *var_phi_b0_soip_dn8_slot = var_phi_b0_soip_dn8;
        *var_phi_b0_soip_rv_slot = var_phi_b0_soip_rv;
        *var_sti2_dlt_slot = var_sti2_dlt;
        *var_sti2_dlt_rv_slot = var_sti2_dlt_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_rv_slot = var_t0_rv;
        *var_tmf0_slot = var_tmf0;
        *var_tmf0_dn0_slot = var_tmf0_dn0;
        *var_tmf0_dn10_slot = var_tmf0_dn10;
        *var_tmf0_dn11_slot = var_tmf0_dn11;
        *var_tmf0_dn12_slot = var_tmf0_dn12;
        *var_tmf0_dn2_slot = var_tmf0_dn2;
        *var_tmf0_dn4_slot = var_tmf0_dn4;
        *var_tmf0_dn5_slot = var_tmf0_dn5;
        *var_tmf0_dn6_slot = var_tmf0_dn6;
        *var_tmf0_dn8_slot = var_tmf0_dn8;
        *var_tmf0_rv_slot = var_tmf0_rv;
        *var_vfbsub1_slot = var_vfbsub1;
        *var_vfbsub1_rv_slot = var_vfbsub1_rv;
        *var_vgpsub_slot = var_vgpsub;
        *var_vgpsub_dn0_slot = var_vgpsub_dn0;
        *var_vgpsub_dn10_slot = var_vgpsub_dn10;
        *var_vgpsub_dn11_slot = var_vgpsub_dn11;
        *var_vgpsub_dn12_slot = var_vgpsub_dn12;
        *var_vgpsub_dn2_slot = var_vgpsub_dn2;
        *var_vgpsub_dn4_slot = var_vgpsub_dn4;
        *var_vgpsub_dn5_slot = var_vgpsub_dn5;
        *var_vgpsub_dn6_slot = var_vgpsub_dn6;
        *var_vgpsub_dn8_slot = var_vgpsub_dn8;
        *var_vgpsub_rv_slot = var_vgpsub_rv;
        *var_xmp_slot = var_xmp;
        *var_xmp_dn0_slot = var_xmp_dn0;
        *var_xmp_dn10_slot = var_xmp_dn10;
        *var_xmp_dn11_slot = var_xmp_dn11;
        *var_xmp_dn12_slot = var_xmp_dn12;
        *var_xmp_dn2_slot = var_xmp_dn2;
        *var_xmp_dn4_slot = var_xmp_dn4;
        *var_xmp_dn5_slot = var_xmp_dn5;
        *var_xmp_dn6_slot = var_xmp_dn6;
        *var_xmp_dn8_slot = var_xmp_dn8;
        *var_xmp_rv_slot = var_xmp_rv;
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn11_slot = var_xp_dn11;
        *var_xp_dn12_slot = var_xp_dn12;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn4_slot = var_xp_dn4;
        *var_xp_dn5_slot = var_xp_dn5;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn8_slot = var_xp_dn8;
        *var_xp_rv_slot = var_xp_rv;
    }

    pub(super) fn stamp_reactive_block_35(
        var_beta: f64,
        var_beta_dn4: f64,
        var_c_fox: f64,
        var_c_fox_dn0: f64,
        var_c_fox_dn10: f64,
        var_c_fox_dn11: f64,
        var_c_fox_dn12: f64,
        var_c_fox_dn2: f64,
        var_c_fox_dn4: f64,
        var_c_fox_dn5: f64,
        var_c_fox_dn6: f64,
        var_c_fox_dn8: f64,
        var_guard124: f64,
        var_leff: f64,
        var_leff_dn0: f64,
        var_leff_dn10: f64,
        var_leff_dn11: f64,
        var_leff_dn12: f64,
        var_leff_dn2: f64,
        var_leff_dn4: f64,
        var_leff_dn5: f64,
        var_leff_dn6: f64,
        var_leff_dn8: f64,
        var_nin: f64,
        var_nin_dn0: f64,
        var_nin_dn10: f64,
        var_nin_dn11: f64,
        var_nin_dn12: f64,
        var_nin_dn2: f64,
        var_nin_dn4: f64,
        var_nin_dn5: f64,
        var_nin_dn6: f64,
        var_nin_dn8: f64,
        var_sti2_dlt: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn4: f64,
        var_uc_nsubs_dn5: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn8: f64,
        var_vgpsub: f64,
        var_vgpsub_dn0: f64,
        var_vgpsub_dn10: f64,
        var_vgpsub_dn11: f64,
        var_vgpsub_dn12: f64,
        var_vgpsub_dn2: f64,
        var_vgpsub_dn4: f64,
        var_vgpsub_dn5: f64,
        var_vgpsub_dn6: f64,
        var_vgpsub_dn8: f64,
        var_weff: f64,
        var_weff_dn0: f64,
        var_weff_dn10: f64,
        var_weff_dn11: f64,
        var_weff_dn12: f64,
        var_weff_dn2: f64,
        var_weff_dn4: f64,
        var_weff_dn5: f64,
        var_weff_dn6: f64,
        var_weff_dn8: f64,
        var_asti_slot: &mut f64,
        var_asti_dn0_slot: &mut f64,
        var_asti_dn10_slot: &mut f64,
        var_asti_dn11_slot: &mut f64,
        var_asti_dn12_slot: &mut f64,
        var_asti_dn2_slot: &mut f64,
        var_asti_dn4_slot: &mut f64,
        var_asti_dn5_slot: &mut f64,
        var_asti_dn6_slot: &mut f64,
        var_asti_dn8_slot: &mut f64,
        var_asti_rv_slot: &mut f64,
        var_costi0_slot: &mut f64,
        var_costi0_dn0_slot: &mut f64,
        var_costi0_dn10_slot: &mut f64,
        var_costi0_dn11_slot: &mut f64,
        var_costi0_dn12_slot: &mut f64,
        var_costi0_dn2_slot: &mut f64,
        var_costi0_dn4_slot: &mut f64,
        var_costi0_dn5_slot: &mut f64,
        var_costi0_dn6_slot: &mut f64,
        var_costi0_dn8_slot: &mut f64,
        var_costi0_rv_slot: &mut f64,
        var_costi1_slot: &mut f64,
        var_costi1_dn0_slot: &mut f64,
        var_costi1_dn10_slot: &mut f64,
        var_costi1_dn11_slot: &mut f64,
        var_costi1_dn12_slot: &mut f64,
        var_costi1_dn2_slot: &mut f64,
        var_costi1_dn4_slot: &mut f64,
        var_costi1_dn5_slot: &mut f64,
        var_costi1_dn6_slot: &mut f64,
        var_costi1_dn8_slot: &mut f64,
        var_costi1_rv_slot: &mut f64,
        var_costi3_slot: &mut f64,
        var_costi3_dn0_slot: &mut f64,
        var_costi3_dn10_slot: &mut f64,
        var_costi3_dn11_slot: &mut f64,
        var_costi3_dn12_slot: &mut f64,
        var_costi3_dn2_slot: &mut f64,
        var_costi3_dn4_slot: &mut f64,
        var_costi3_dn5_slot: &mut f64,
        var_costi3_dn6_slot: &mut f64,
        var_costi3_dn8_slot: &mut f64,
        var_costi3_rv_slot: &mut f64,
        var_costi4_slot: &mut f64,
        var_costi4_dn0_slot: &mut f64,
        var_costi4_dn10_slot: &mut f64,
        var_costi4_dn11_slot: &mut f64,
        var_costi4_dn12_slot: &mut f64,
        var_costi4_dn2_slot: &mut f64,
        var_costi4_dn4_slot: &mut f64,
        var_costi4_dn5_slot: &mut f64,
        var_costi4_dn6_slot: &mut f64,
        var_costi4_dn8_slot: &mut f64,
        var_costi4_rv_slot: &mut f64,
        var_costi5_slot: &mut f64,
        var_costi5_dn0_slot: &mut f64,
        var_costi5_dn10_slot: &mut f64,
        var_costi5_dn11_slot: &mut f64,
        var_costi5_dn12_slot: &mut f64,
        var_costi5_dn2_slot: &mut f64,
        var_costi5_dn4_slot: &mut f64,
        var_costi5_dn5_slot: &mut f64,
        var_costi5_dn6_slot: &mut f64,
        var_costi5_dn8_slot: &mut f64,
        var_costi5_rv_slot: &mut f64,
        var_costi6_slot: &mut f64,
        var_costi6_dn0_slot: &mut f64,
        var_costi6_dn10_slot: &mut f64,
        var_costi6_dn11_slot: &mut f64,
        var_costi6_dn12_slot: &mut f64,
        var_costi6_dn2_slot: &mut f64,
        var_costi6_dn4_slot: &mut f64,
        var_costi6_dn5_slot: &mut f64,
        var_costi6_dn6_slot: &mut f64,
        var_costi6_dn8_slot: &mut f64,
        var_costi6_rv_slot: &mut f64,
        var_costi7_slot: &mut f64,
        var_costi7_dn0_slot: &mut f64,
        var_costi7_dn10_slot: &mut f64,
        var_costi7_dn11_slot: &mut f64,
        var_costi7_dn12_slot: &mut f64,
        var_costi7_dn2_slot: &mut f64,
        var_costi7_dn4_slot: &mut f64,
        var_costi7_dn5_slot: &mut f64,
        var_costi7_dn6_slot: &mut f64,
        var_costi7_dn8_slot: &mut f64,
        var_costi7_rv_slot: &mut f64,
        var_expsti_slot: &mut f64,
        var_expsti_dn0_slot: &mut f64,
        var_expsti_dn10_slot: &mut f64,
        var_expsti_dn11_slot: &mut f64,
        var_expsti_dn12_slot: &mut f64,
        var_expsti_dn2_slot: &mut f64,
        var_expsti_dn4_slot: &mut f64,
        var_expsti_dn5_slot: &mut f64,
        var_expsti_dn6_slot: &mut f64,
        var_expsti_dn8_slot: &mut f64,
        var_expsti_rv_slot: &mut f64,
        var_guard125_slot: &mut f64,
        var_guard125_rv_slot: &mut f64,
        var_idssti_slot: &mut f64,
        var_idssti_dn0_slot: &mut f64,
        var_idssti_dn10_slot: &mut f64,
        var_idssti_dn11_slot: &mut f64,
        var_idssti_dn12_slot: &mut f64,
        var_idssti_dn2_slot: &mut f64,
        var_idssti_dn4_slot: &mut f64,
        var_idssti_dn5_slot: &mut f64,
        var_idssti_dn6_slot: &mut f64,
        var_idssti_dn8_slot: &mut f64,
        var_idssti_rv_slot: &mut f64,
        var_lred_slot: &mut f64,
        var_lred_dn0_slot: &mut f64,
        var_lred_dn10_slot: &mut f64,
        var_lred_dn11_slot: &mut f64,
        var_lred_dn12_slot: &mut f64,
        var_lred_dn2_slot: &mut f64,
        var_lred_dn4_slot: &mut f64,
        var_lred_dn5_slot: &mut f64,
        var_lred_dn6_slot: &mut f64,
        var_lred_dn8_slot: &mut f64,
        var_lred_rv_slot: &mut f64,
        var_mu_slot: &mut f64,
        var_mu_dn0_slot: &mut f64,
        var_mu_dn10_slot: &mut f64,
        var_mu_dn11_slot: &mut f64,
        var_mu_dn12_slot: &mut f64,
        var_mu_dn2_slot: &mut f64,
        var_mu_dn4_slot: &mut f64,
        var_mu_dn5_slot: &mut f64,
        var_mu_dn6_slot: &mut f64,
        var_mu_dn8_slot: &mut f64,
        var_mu_rv_slot: &mut f64,
        var_psab_slot: &mut f64,
        var_psab_dn0_slot: &mut f64,
        var_psab_dn10_slot: &mut f64,
        var_psab_dn11_slot: &mut f64,
        var_psab_dn12_slot: &mut f64,
        var_psab_dn2_slot: &mut f64,
        var_psab_dn4_slot: &mut f64,
        var_psab_dn5_slot: &mut f64,
        var_psab_dn6_slot: &mut f64,
        var_psab_dn8_slot: &mut f64,
        var_psab_rv_slot: &mut f64,
        var_psasti_slot: &mut f64,
        var_psasti_dn0_slot: &mut f64,
        var_psasti_dn10_slot: &mut f64,
        var_psasti_dn11_slot: &mut f64,
        var_psasti_dn12_slot: &mut f64,
        var_psasti_dn2_slot: &mut f64,
        var_psasti_dn4_slot: &mut f64,
        var_psasti_dn5_slot: &mut f64,
        var_psasti_dn6_slot: &mut f64,
        var_psasti_dn8_slot: &mut f64,
        var_psasti_rv_slot: &mut f64,
        var_psbsti_slot: &mut f64,
        var_psbsti_dn0_slot: &mut f64,
        var_psbsti_dn10_slot: &mut f64,
        var_psbsti_dn11_slot: &mut f64,
        var_psbsti_dn12_slot: &mut f64,
        var_psbsti_dn2_slot: &mut f64,
        var_psbsti_dn4_slot: &mut f64,
        var_psbsti_dn5_slot: &mut f64,
        var_psbsti_dn6_slot: &mut f64,
        var_psbsti_dn8_slot: &mut f64,
        var_psbsti_rv_slot: &mut f64,
        var_psti_slot: &mut f64,
        var_psti_dn0_slot: &mut f64,
        var_psti_dn10_slot: &mut f64,
        var_psti_dn11_slot: &mut f64,
        var_psti_dn12_slot: &mut f64,
        var_psti_dn2_slot: &mut f64,
        var_psti_dn4_slot: &mut f64,
        var_psti_dn5_slot: &mut f64,
        var_psti_dn6_slot: &mut f64,
        var_psti_dn8_slot: &mut f64,
        var_psti_rv_slot: &mut f64,
        var_qn0sti_slot: &mut f64,
        var_qn0sti_dn0_slot: &mut f64,
        var_qn0sti_dn10_slot: &mut f64,
        var_qn0sti_dn11_slot: &mut f64,
        var_qn0sti_dn12_slot: &mut f64,
        var_qn0sti_dn2_slot: &mut f64,
        var_qn0sti_dn4_slot: &mut f64,
        var_qn0sti_dn5_slot: &mut f64,
        var_qn0sti_dn6_slot: &mut f64,
        var_qn0sti_dn8_slot: &mut f64,
        var_qn0sti_rv_slot: &mut f64,
        var_sq1sti_slot: &mut f64,
        var_sq1sti_dn0_slot: &mut f64,
        var_sq1sti_dn10_slot: &mut f64,
        var_sq1sti_dn11_slot: &mut f64,
        var_sq1sti_dn12_slot: &mut f64,
        var_sq1sti_dn2_slot: &mut f64,
        var_sq1sti_dn4_slot: &mut f64,
        var_sq1sti_dn5_slot: &mut f64,
        var_sq1sti_dn6_slot: &mut f64,
        var_sq1sti_dn8_slot: &mut f64,
        var_sq1sti_rv_slot: &mut f64,
        var_sq2sti_slot: &mut f64,
        var_sq2sti_dn0_slot: &mut f64,
        var_sq2sti_dn10_slot: &mut f64,
        var_sq2sti_dn11_slot: &mut f64,
        var_sq2sti_dn12_slot: &mut f64,
        var_sq2sti_dn2_slot: &mut f64,
        var_sq2sti_dn4_slot: &mut f64,
        var_sq2sti_dn5_slot: &mut f64,
        var_sq2sti_dn6_slot: &mut f64,
        var_sq2sti_dn8_slot: &mut f64,
        var_sq2sti_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_vgssti_slot: &mut f64,
        var_vgssti_dn0_slot: &mut f64,
        var_vgssti_dn10_slot: &mut f64,
        var_vgssti_dn11_slot: &mut f64,
        var_vgssti_dn12_slot: &mut f64,
        var_vgssti_dn2_slot: &mut f64,
        var_vgssti_dn4_slot: &mut f64,
        var_vgssti_dn5_slot: &mut f64,
        var_vgssti_dn6_slot: &mut f64,
        var_vgssti_dn8_slot: &mut f64,
        var_vgssti_rv_slot: &mut f64,
    ) {
        let mut var_asti: f64 = *var_asti_slot;
        let mut var_asti_dn0: f64 = *var_asti_dn0_slot;
        let mut var_asti_dn10: f64 = *var_asti_dn10_slot;
        let mut var_asti_dn11: f64 = *var_asti_dn11_slot;
        let mut var_asti_dn12: f64 = *var_asti_dn12_slot;
        let mut var_asti_dn2: f64 = *var_asti_dn2_slot;
        let mut var_asti_dn4: f64 = *var_asti_dn4_slot;
        let mut var_asti_dn5: f64 = *var_asti_dn5_slot;
        let mut var_asti_dn6: f64 = *var_asti_dn6_slot;
        let mut var_asti_dn8: f64 = *var_asti_dn8_slot;
        let mut var_asti_rv: f64 = *var_asti_rv_slot;
        let mut var_costi0: f64 = *var_costi0_slot;
        let mut var_costi0_dn0: f64 = *var_costi0_dn0_slot;
        let mut var_costi0_dn10: f64 = *var_costi0_dn10_slot;
        let mut var_costi0_dn11: f64 = *var_costi0_dn11_slot;
        let mut var_costi0_dn12: f64 = *var_costi0_dn12_slot;
        let mut var_costi0_dn2: f64 = *var_costi0_dn2_slot;
        let mut var_costi0_dn4: f64 = *var_costi0_dn4_slot;
        let mut var_costi0_dn5: f64 = *var_costi0_dn5_slot;
        let mut var_costi0_dn6: f64 = *var_costi0_dn6_slot;
        let mut var_costi0_dn8: f64 = *var_costi0_dn8_slot;
        let mut var_costi0_rv: f64 = *var_costi0_rv_slot;
        let mut var_costi1: f64 = *var_costi1_slot;
        let mut var_costi1_dn0: f64 = *var_costi1_dn0_slot;
        let mut var_costi1_dn10: f64 = *var_costi1_dn10_slot;
        let mut var_costi1_dn11: f64 = *var_costi1_dn11_slot;
        let mut var_costi1_dn12: f64 = *var_costi1_dn12_slot;
        let mut var_costi1_dn2: f64 = *var_costi1_dn2_slot;
        let mut var_costi1_dn4: f64 = *var_costi1_dn4_slot;
        let mut var_costi1_dn5: f64 = *var_costi1_dn5_slot;
        let mut var_costi1_dn6: f64 = *var_costi1_dn6_slot;
        let mut var_costi1_dn8: f64 = *var_costi1_dn8_slot;
        let mut var_costi1_rv: f64 = *var_costi1_rv_slot;
        let mut var_costi3: f64 = *var_costi3_slot;
        let mut var_costi3_dn0: f64 = *var_costi3_dn0_slot;
        let mut var_costi3_dn10: f64 = *var_costi3_dn10_slot;
        let mut var_costi3_dn11: f64 = *var_costi3_dn11_slot;
        let mut var_costi3_dn12: f64 = *var_costi3_dn12_slot;
        let mut var_costi3_dn2: f64 = *var_costi3_dn2_slot;
        let mut var_costi3_dn4: f64 = *var_costi3_dn4_slot;
        let mut var_costi3_dn5: f64 = *var_costi3_dn5_slot;
        let mut var_costi3_dn6: f64 = *var_costi3_dn6_slot;
        let mut var_costi3_dn8: f64 = *var_costi3_dn8_slot;
        let mut var_costi3_rv: f64 = *var_costi3_rv_slot;
        let mut var_costi4: f64 = *var_costi4_slot;
        let mut var_costi4_dn0: f64 = *var_costi4_dn0_slot;
        let mut var_costi4_dn10: f64 = *var_costi4_dn10_slot;
        let mut var_costi4_dn11: f64 = *var_costi4_dn11_slot;
        let mut var_costi4_dn12: f64 = *var_costi4_dn12_slot;
        let mut var_costi4_dn2: f64 = *var_costi4_dn2_slot;
        let mut var_costi4_dn4: f64 = *var_costi4_dn4_slot;
        let mut var_costi4_dn5: f64 = *var_costi4_dn5_slot;
        let mut var_costi4_dn6: f64 = *var_costi4_dn6_slot;
        let mut var_costi4_dn8: f64 = *var_costi4_dn8_slot;
        let mut var_costi4_rv: f64 = *var_costi4_rv_slot;
        let mut var_costi5: f64 = *var_costi5_slot;
        let mut var_costi5_dn0: f64 = *var_costi5_dn0_slot;
        let mut var_costi5_dn10: f64 = *var_costi5_dn10_slot;
        let mut var_costi5_dn11: f64 = *var_costi5_dn11_slot;
        let mut var_costi5_dn12: f64 = *var_costi5_dn12_slot;
        let mut var_costi5_dn2: f64 = *var_costi5_dn2_slot;
        let mut var_costi5_dn4: f64 = *var_costi5_dn4_slot;
        let mut var_costi5_dn5: f64 = *var_costi5_dn5_slot;
        let mut var_costi5_dn6: f64 = *var_costi5_dn6_slot;
        let mut var_costi5_dn8: f64 = *var_costi5_dn8_slot;
        let mut var_costi5_rv: f64 = *var_costi5_rv_slot;
        let mut var_costi6: f64 = *var_costi6_slot;
        let mut var_costi6_dn0: f64 = *var_costi6_dn0_slot;
        let mut var_costi6_dn10: f64 = *var_costi6_dn10_slot;
        let mut var_costi6_dn11: f64 = *var_costi6_dn11_slot;
        let mut var_costi6_dn12: f64 = *var_costi6_dn12_slot;
        let mut var_costi6_dn2: f64 = *var_costi6_dn2_slot;
        let mut var_costi6_dn4: f64 = *var_costi6_dn4_slot;
        let mut var_costi6_dn5: f64 = *var_costi6_dn5_slot;
        let mut var_costi6_dn6: f64 = *var_costi6_dn6_slot;
        let mut var_costi6_dn8: f64 = *var_costi6_dn8_slot;
        let mut var_costi6_rv: f64 = *var_costi6_rv_slot;
        let mut var_costi7: f64 = *var_costi7_slot;
        let mut var_costi7_dn0: f64 = *var_costi7_dn0_slot;
        let mut var_costi7_dn10: f64 = *var_costi7_dn10_slot;
        let mut var_costi7_dn11: f64 = *var_costi7_dn11_slot;
        let mut var_costi7_dn12: f64 = *var_costi7_dn12_slot;
        let mut var_costi7_dn2: f64 = *var_costi7_dn2_slot;
        let mut var_costi7_dn4: f64 = *var_costi7_dn4_slot;
        let mut var_costi7_dn5: f64 = *var_costi7_dn5_slot;
        let mut var_costi7_dn6: f64 = *var_costi7_dn6_slot;
        let mut var_costi7_dn8: f64 = *var_costi7_dn8_slot;
        let mut var_costi7_rv: f64 = *var_costi7_rv_slot;
        let mut var_expsti: f64 = *var_expsti_slot;
        let mut var_expsti_dn0: f64 = *var_expsti_dn0_slot;
        let mut var_expsti_dn10: f64 = *var_expsti_dn10_slot;
        let mut var_expsti_dn11: f64 = *var_expsti_dn11_slot;
        let mut var_expsti_dn12: f64 = *var_expsti_dn12_slot;
        let mut var_expsti_dn2: f64 = *var_expsti_dn2_slot;
        let mut var_expsti_dn4: f64 = *var_expsti_dn4_slot;
        let mut var_expsti_dn5: f64 = *var_expsti_dn5_slot;
        let mut var_expsti_dn6: f64 = *var_expsti_dn6_slot;
        let mut var_expsti_dn8: f64 = *var_expsti_dn8_slot;
        let mut var_expsti_rv: f64 = *var_expsti_rv_slot;
        let mut var_guard125: f64 = *var_guard125_slot;
        let mut var_guard125_rv: f64 = *var_guard125_rv_slot;
        let mut var_idssti: f64 = *var_idssti_slot;
        let mut var_idssti_dn0: f64 = *var_idssti_dn0_slot;
        let mut var_idssti_dn10: f64 = *var_idssti_dn10_slot;
        let mut var_idssti_dn11: f64 = *var_idssti_dn11_slot;
        let mut var_idssti_dn12: f64 = *var_idssti_dn12_slot;
        let mut var_idssti_dn2: f64 = *var_idssti_dn2_slot;
        let mut var_idssti_dn4: f64 = *var_idssti_dn4_slot;
        let mut var_idssti_dn5: f64 = *var_idssti_dn5_slot;
        let mut var_idssti_dn6: f64 = *var_idssti_dn6_slot;
        let mut var_idssti_dn8: f64 = *var_idssti_dn8_slot;
        let mut var_idssti_rv: f64 = *var_idssti_rv_slot;
        let mut var_lred: f64 = *var_lred_slot;
        let mut var_lred_dn0: f64 = *var_lred_dn0_slot;
        let mut var_lred_dn10: f64 = *var_lred_dn10_slot;
        let mut var_lred_dn11: f64 = *var_lred_dn11_slot;
        let mut var_lred_dn12: f64 = *var_lred_dn12_slot;
        let mut var_lred_dn2: f64 = *var_lred_dn2_slot;
        let mut var_lred_dn4: f64 = *var_lred_dn4_slot;
        let mut var_lred_dn5: f64 = *var_lred_dn5_slot;
        let mut var_lred_dn6: f64 = *var_lred_dn6_slot;
        let mut var_lred_dn8: f64 = *var_lred_dn8_slot;
        let mut var_lred_rv: f64 = *var_lred_rv_slot;
        let mut var_mu: f64 = *var_mu_slot;
        let mut var_mu_dn0: f64 = *var_mu_dn0_slot;
        let mut var_mu_dn10: f64 = *var_mu_dn10_slot;
        let mut var_mu_dn11: f64 = *var_mu_dn11_slot;
        let mut var_mu_dn12: f64 = *var_mu_dn12_slot;
        let mut var_mu_dn2: f64 = *var_mu_dn2_slot;
        let mut var_mu_dn4: f64 = *var_mu_dn4_slot;
        let mut var_mu_dn5: f64 = *var_mu_dn5_slot;
        let mut var_mu_dn6: f64 = *var_mu_dn6_slot;
        let mut var_mu_dn8: f64 = *var_mu_dn8_slot;
        let mut var_mu_rv: f64 = *var_mu_rv_slot;
        let mut var_psab: f64 = *var_psab_slot;
        let mut var_psab_dn0: f64 = *var_psab_dn0_slot;
        let mut var_psab_dn10: f64 = *var_psab_dn10_slot;
        let mut var_psab_dn11: f64 = *var_psab_dn11_slot;
        let mut var_psab_dn12: f64 = *var_psab_dn12_slot;
        let mut var_psab_dn2: f64 = *var_psab_dn2_slot;
        let mut var_psab_dn4: f64 = *var_psab_dn4_slot;
        let mut var_psab_dn5: f64 = *var_psab_dn5_slot;
        let mut var_psab_dn6: f64 = *var_psab_dn6_slot;
        let mut var_psab_dn8: f64 = *var_psab_dn8_slot;
        let mut var_psab_rv: f64 = *var_psab_rv_slot;
        let mut var_psasti: f64 = *var_psasti_slot;
        let mut var_psasti_dn0: f64 = *var_psasti_dn0_slot;
        let mut var_psasti_dn10: f64 = *var_psasti_dn10_slot;
        let mut var_psasti_dn11: f64 = *var_psasti_dn11_slot;
        let mut var_psasti_dn12: f64 = *var_psasti_dn12_slot;
        let mut var_psasti_dn2: f64 = *var_psasti_dn2_slot;
        let mut var_psasti_dn4: f64 = *var_psasti_dn4_slot;
        let mut var_psasti_dn5: f64 = *var_psasti_dn5_slot;
        let mut var_psasti_dn6: f64 = *var_psasti_dn6_slot;
        let mut var_psasti_dn8: f64 = *var_psasti_dn8_slot;
        let mut var_psasti_rv: f64 = *var_psasti_rv_slot;
        let mut var_psbsti: f64 = *var_psbsti_slot;
        let mut var_psbsti_dn0: f64 = *var_psbsti_dn0_slot;
        let mut var_psbsti_dn10: f64 = *var_psbsti_dn10_slot;
        let mut var_psbsti_dn11: f64 = *var_psbsti_dn11_slot;
        let mut var_psbsti_dn12: f64 = *var_psbsti_dn12_slot;
        let mut var_psbsti_dn2: f64 = *var_psbsti_dn2_slot;
        let mut var_psbsti_dn4: f64 = *var_psbsti_dn4_slot;
        let mut var_psbsti_dn5: f64 = *var_psbsti_dn5_slot;
        let mut var_psbsti_dn6: f64 = *var_psbsti_dn6_slot;
        let mut var_psbsti_dn8: f64 = *var_psbsti_dn8_slot;
        let mut var_psbsti_rv: f64 = *var_psbsti_rv_slot;
        let mut var_psti: f64 = *var_psti_slot;
        let mut var_psti_dn0: f64 = *var_psti_dn0_slot;
        let mut var_psti_dn10: f64 = *var_psti_dn10_slot;
        let mut var_psti_dn11: f64 = *var_psti_dn11_slot;
        let mut var_psti_dn12: f64 = *var_psti_dn12_slot;
        let mut var_psti_dn2: f64 = *var_psti_dn2_slot;
        let mut var_psti_dn4: f64 = *var_psti_dn4_slot;
        let mut var_psti_dn5: f64 = *var_psti_dn5_slot;
        let mut var_psti_dn6: f64 = *var_psti_dn6_slot;
        let mut var_psti_dn8: f64 = *var_psti_dn8_slot;
        let mut var_psti_rv: f64 = *var_psti_rv_slot;
        let mut var_qn0sti: f64 = *var_qn0sti_slot;
        let mut var_qn0sti_dn0: f64 = *var_qn0sti_dn0_slot;
        let mut var_qn0sti_dn10: f64 = *var_qn0sti_dn10_slot;
        let mut var_qn0sti_dn11: f64 = *var_qn0sti_dn11_slot;
        let mut var_qn0sti_dn12: f64 = *var_qn0sti_dn12_slot;
        let mut var_qn0sti_dn2: f64 = *var_qn0sti_dn2_slot;
        let mut var_qn0sti_dn4: f64 = *var_qn0sti_dn4_slot;
        let mut var_qn0sti_dn5: f64 = *var_qn0sti_dn5_slot;
        let mut var_qn0sti_dn6: f64 = *var_qn0sti_dn6_slot;
        let mut var_qn0sti_dn8: f64 = *var_qn0sti_dn8_slot;
        let mut var_qn0sti_rv: f64 = *var_qn0sti_rv_slot;
        let mut var_sq1sti: f64 = *var_sq1sti_slot;
        let mut var_sq1sti_dn0: f64 = *var_sq1sti_dn0_slot;
        let mut var_sq1sti_dn10: f64 = *var_sq1sti_dn10_slot;
        let mut var_sq1sti_dn11: f64 = *var_sq1sti_dn11_slot;
        let mut var_sq1sti_dn12: f64 = *var_sq1sti_dn12_slot;
        let mut var_sq1sti_dn2: f64 = *var_sq1sti_dn2_slot;
        let mut var_sq1sti_dn4: f64 = *var_sq1sti_dn4_slot;
        let mut var_sq1sti_dn5: f64 = *var_sq1sti_dn5_slot;
        let mut var_sq1sti_dn6: f64 = *var_sq1sti_dn6_slot;
        let mut var_sq1sti_dn8: f64 = *var_sq1sti_dn8_slot;
        let mut var_sq1sti_rv: f64 = *var_sq1sti_rv_slot;
        let mut var_sq2sti: f64 = *var_sq2sti_slot;
        let mut var_sq2sti_dn0: f64 = *var_sq2sti_dn0_slot;
        let mut var_sq2sti_dn10: f64 = *var_sq2sti_dn10_slot;
        let mut var_sq2sti_dn11: f64 = *var_sq2sti_dn11_slot;
        let mut var_sq2sti_dn12: f64 = *var_sq2sti_dn12_slot;
        let mut var_sq2sti_dn2: f64 = *var_sq2sti_dn2_slot;
        let mut var_sq2sti_dn4: f64 = *var_sq2sti_dn4_slot;
        let mut var_sq2sti_dn5: f64 = *var_sq2sti_dn5_slot;
        let mut var_sq2sti_dn6: f64 = *var_sq2sti_dn6_slot;
        let mut var_sq2sti_dn8: f64 = *var_sq2sti_dn8_slot;
        let mut var_sq2sti_rv: f64 = *var_sq2sti_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_vgssti: f64 = *var_vgssti_slot;
        let mut var_vgssti_dn0: f64 = *var_vgssti_dn0_slot;
        let mut var_vgssti_dn10: f64 = *var_vgssti_dn10_slot;
        let mut var_vgssti_dn11: f64 = *var_vgssti_dn11_slot;
        let mut var_vgssti_dn12: f64 = *var_vgssti_dn12_slot;
        let mut var_vgssti_dn2: f64 = *var_vgssti_dn2_slot;
        let mut var_vgssti_dn4: f64 = *var_vgssti_dn4_slot;
        let mut var_vgssti_dn5: f64 = *var_vgssti_dn5_slot;
        let mut var_vgssti_dn6: f64 = *var_vgssti_dn6_slot;
        let mut var_vgssti_dn8: f64 = *var_vgssti_dn8_slot;
        let mut var_vgssti_rv: f64 = *var_vgssti_rv_slot;

        let (assign9170_e9680, assign9170_e9680_d_n0, assign9170_e9680_d_n2, assign9170_e9680_d_n4, assign9170_e9680_d_n5, assign9170_e9680_d_n6, assign9170_e9680_d_n8, assign9170_e9680_d_n10, assign9170_e9680_d_n11, assign9170_e9680_d_n12,) = {
    if (var_guard124 != 0.0) {
        (var_vgpsub, var_vgpsub_dn0, var_vgpsub_dn2, var_vgpsub_dn4, var_vgpsub_dn5, var_vgpsub_dn6, var_vgpsub_dn8, var_vgpsub_dn10, var_vgpsub_dn11, var_vgpsub_dn12,)
    } else {
        (var_vgssti, var_vgssti_dn0, var_vgssti_dn2, var_vgssti_dn4, var_vgssti_dn5, var_vgssti_dn6, var_vgssti_dn8, var_vgssti_dn10, var_vgssti_dn11, var_vgssti_dn12,)
    }
};
        var_vgssti = assign9170_e9680;
        var_vgssti_dn0 = assign9170_e9680_d_n0;
        var_vgssti_dn2 = assign9170_e9680_d_n2;
        var_vgssti_dn4 = assign9170_e9680_d_n4;
        var_vgssti_dn5 = assign9170_e9680_d_n5;
        var_vgssti_dn6 = assign9170_e9680_d_n6;
        var_vgssti_dn8 = assign9170_e9680_d_n8;
        var_vgssti_dn10 = assign9170_e9680_d_n10;
        var_vgssti_dn11 = assign9170_e9680_d_n11;
        var_vgssti_dn12 = assign9170_e9680_d_n12;
        var_vgssti_rv = 0.0;

        let (assign9180_e9693, assign9180_e9693_d_n0, assign9180_e9693_d_n2, assign9180_e9693_d_n4, assign9180_e9693_d_n5, assign9180_e9693_d_n6, assign9180_e9693_d_n8, assign9180_e9693_d_n10, assign9180_e9693_d_n11, assign9180_e9693_d_n12,) = {
    if (var_guard124 != 0.0) {
        let assign9180_e9684: f64 = (2.0 * 1.6021918e-19);
        let assign9180_e9686: f64 = (assign9180_e9684 * var_uc_nsubs);
        let assign9180_e9688: f64 = (assign9180_e9686 * 1.034943e-10);
        let assign9180_e9690: f64 = (assign9180_e9688 / var_beta);
        let assign9180_e9691: f64 = (assign9180_e9690).sqrt();
        (assign9180_e9691, ((((assign9180_e9684 * var_uc_nsubs_dn0) * 1.034943e-10) / var_beta) / (2.0 * assign9180_e9691)), ((((assign9180_e9684 * var_uc_nsubs_dn2) * 1.034943e-10) / var_beta) / (2.0 * assign9180_e9691)), ((((((assign9180_e9684 * var_uc_nsubs_dn4) * 1.034943e-10) * var_beta) - (assign9180_e9688 * var_beta_dn4)) / (var_beta * var_beta)) / (2.0 * assign9180_e9691)), ((((assign9180_e9684 * var_uc_nsubs_dn5) * 1.034943e-10) / var_beta) / (2.0 * assign9180_e9691)), ((((assign9180_e9684 * var_uc_nsubs_dn6) * 1.034943e-10) / var_beta) / (2.0 * assign9180_e9691)), ((((assign9180_e9684 * var_uc_nsubs_dn8) * 1.034943e-10) / var_beta) / (2.0 * assign9180_e9691)), ((((assign9180_e9684 * var_uc_nsubs_dn10) * 1.034943e-10) / var_beta) / (2.0 * assign9180_e9691)), ((((assign9180_e9684 * var_uc_nsubs_dn11) * 1.034943e-10) / var_beta) / (2.0 * assign9180_e9691)), ((((assign9180_e9684 * var_uc_nsubs_dn12) * 1.034943e-10) / var_beta) / (2.0 * assign9180_e9691)),)
    } else {
        (var_costi0, var_costi0_dn0, var_costi0_dn2, var_costi0_dn4, var_costi0_dn5, var_costi0_dn6, var_costi0_dn8, var_costi0_dn10, var_costi0_dn11, var_costi0_dn12,)
    }
};
        var_costi0 = assign9180_e9693;
        var_costi0_dn0 = assign9180_e9693_d_n0;
        var_costi0_dn2 = assign9180_e9693_d_n2;
        var_costi0_dn4 = assign9180_e9693_d_n4;
        var_costi0_dn5 = assign9180_e9693_d_n5;
        var_costi0_dn6 = assign9180_e9693_d_n6;
        var_costi0_dn8 = assign9180_e9693_d_n8;
        var_costi0_dn10 = assign9180_e9693_d_n10;
        var_costi0_dn11 = assign9180_e9693_d_n11;
        var_costi0_dn12 = assign9180_e9693_d_n12;
        var_costi0_rv = 0.0;

        let (assign9190_e9703, assign9190_e9703_d_n0, assign9190_e9703_d_n2, assign9190_e9703_d_n4, assign9190_e9703_d_n5, assign9190_e9703_d_n6, assign9190_e9703_d_n8, assign9190_e9703_d_n10, assign9190_e9703_d_n11, assign9190_e9703_d_n12,) = {
    if (var_guard124 != 0.0) {
        let assign9190_e9697: f64 = (var_nin * var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / var_uc_nsubs;
        let assign9190_e9699: f64 = (assign9190_e9697 * __rspice_inv_cse_0);
        let assign9190_e9701: f64 = (assign9190_e9699 * __rspice_inv_cse_0);
        (assign9190_e9701, ((((((((var_nin_dn0 * var_nin) + (var_nin * var_nin_dn0)) * var_uc_nsubs) - (assign9190_e9697 * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)) * var_uc_nsubs) - (assign9190_e9699 * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_nin_dn2 * var_nin) + (var_nin * var_nin_dn2)) * var_uc_nsubs) - (assign9190_e9697 * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)) * var_uc_nsubs) - (assign9190_e9699 * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_nin_dn4 * var_nin) + (var_nin * var_nin_dn4)) * var_uc_nsubs) - (assign9190_e9697 * var_uc_nsubs_dn4)) / (var_uc_nsubs * var_uc_nsubs)) * var_uc_nsubs) - (assign9190_e9699 * var_uc_nsubs_dn4)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_nin_dn5 * var_nin) + (var_nin * var_nin_dn5)) * var_uc_nsubs) - (assign9190_e9697 * var_uc_nsubs_dn5)) / (var_uc_nsubs * var_uc_nsubs)) * var_uc_nsubs) - (assign9190_e9699 * var_uc_nsubs_dn5)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_nin_dn6 * var_nin) + (var_nin * var_nin_dn6)) * var_uc_nsubs) - (assign9190_e9697 * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)) * var_uc_nsubs) - (assign9190_e9699 * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_nin_dn8 * var_nin) + (var_nin * var_nin_dn8)) * var_uc_nsubs) - (assign9190_e9697 * var_uc_nsubs_dn8)) / (var_uc_nsubs * var_uc_nsubs)) * var_uc_nsubs) - (assign9190_e9699 * var_uc_nsubs_dn8)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_nin_dn10 * var_nin) + (var_nin * var_nin_dn10)) * var_uc_nsubs) - (assign9190_e9697 * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)) * var_uc_nsubs) - (assign9190_e9699 * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_nin_dn11 * var_nin) + (var_nin * var_nin_dn11)) * var_uc_nsubs) - (assign9190_e9697 * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)) * var_uc_nsubs) - (assign9190_e9699 * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)), ((((((((var_nin_dn12 * var_nin) + (var_nin * var_nin_dn12)) * var_uc_nsubs) - (assign9190_e9697 * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)) * var_uc_nsubs) - (assign9190_e9699 * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)),)
    } else {
        (var_costi1, var_costi1_dn0, var_costi1_dn2, var_costi1_dn4, var_costi1_dn5, var_costi1_dn6, var_costi1_dn8, var_costi1_dn10, var_costi1_dn11, var_costi1_dn12,)
    }
};
        var_costi1 = assign9190_e9703;
        var_costi1_dn0 = assign9190_e9703_d_n0;
        var_costi1_dn2 = assign9190_e9703_d_n2;
        var_costi1_dn4 = assign9190_e9703_d_n4;
        var_costi1_dn5 = assign9190_e9703_d_n5;
        var_costi1_dn6 = assign9190_e9703_d_n6;
        var_costi1_dn8 = assign9190_e9703_d_n8;
        var_costi1_dn10 = assign9190_e9703_d_n10;
        var_costi1_dn11 = assign9190_e9703_d_n11;
        var_costi1_dn12 = assign9190_e9703_d_n12;
        var_costi1_rv = 0.0;

        let (assign9200_e9713, assign9200_e9713_d_n0, assign9200_e9713_d_n2, assign9200_e9713_d_n4, assign9200_e9713_d_n5, assign9200_e9713_d_n6, assign9200_e9713_d_n8, assign9200_e9713_d_n10, assign9200_e9713_d_n11, assign9200_e9713_d_n12,) = {
    if (var_guard124 != 0.0) {
        let assign9200_e9707: f64 = (var_costi0 * var_costi0);
        let __rspice_inv_cse_1: f64 = 1.0 / var_c_fox;
        let assign9200_e9709: f64 = (assign9200_e9707 * __rspice_inv_cse_1);
        let assign9200_e9711: f64 = (assign9200_e9709 * __rspice_inv_cse_1);
        (assign9200_e9711, ((((((((var_costi0_dn0 * var_costi0) + (var_costi0 * var_costi0_dn0)) * var_c_fox) - (assign9200_e9707 * var_c_fox_dn0)) / (var_c_fox * var_c_fox)) * var_c_fox) - (assign9200_e9709 * var_c_fox_dn0)) / (var_c_fox * var_c_fox)), ((((((((var_costi0_dn2 * var_costi0) + (var_costi0 * var_costi0_dn2)) * var_c_fox) - (assign9200_e9707 * var_c_fox_dn2)) / (var_c_fox * var_c_fox)) * var_c_fox) - (assign9200_e9709 * var_c_fox_dn2)) / (var_c_fox * var_c_fox)), ((((((((var_costi0_dn4 * var_costi0) + (var_costi0 * var_costi0_dn4)) * var_c_fox) - (assign9200_e9707 * var_c_fox_dn4)) / (var_c_fox * var_c_fox)) * var_c_fox) - (assign9200_e9709 * var_c_fox_dn4)) / (var_c_fox * var_c_fox)), ((((((((var_costi0_dn5 * var_costi0) + (var_costi0 * var_costi0_dn5)) * var_c_fox) - (assign9200_e9707 * var_c_fox_dn5)) / (var_c_fox * var_c_fox)) * var_c_fox) - (assign9200_e9709 * var_c_fox_dn5)) / (var_c_fox * var_c_fox)), ((((((((var_costi0_dn6 * var_costi0) + (var_costi0 * var_costi0_dn6)) * var_c_fox) - (assign9200_e9707 * var_c_fox_dn6)) / (var_c_fox * var_c_fox)) * var_c_fox) - (assign9200_e9709 * var_c_fox_dn6)) / (var_c_fox * var_c_fox)), ((((((((var_costi0_dn8 * var_costi0) + (var_costi0 * var_costi0_dn8)) * var_c_fox) - (assign9200_e9707 * var_c_fox_dn8)) / (var_c_fox * var_c_fox)) * var_c_fox) - (assign9200_e9709 * var_c_fox_dn8)) / (var_c_fox * var_c_fox)), ((((((((var_costi0_dn10 * var_costi0) + (var_costi0 * var_costi0_dn10)) * var_c_fox) - (assign9200_e9707 * var_c_fox_dn10)) / (var_c_fox * var_c_fox)) * var_c_fox) - (assign9200_e9709 * var_c_fox_dn10)) / (var_c_fox * var_c_fox)), ((((((((var_costi0_dn11 * var_costi0) + (var_costi0 * var_costi0_dn11)) * var_c_fox) - (assign9200_e9707 * var_c_fox_dn11)) / (var_c_fox * var_c_fox)) * var_c_fox) - (assign9200_e9709 * var_c_fox_dn11)) / (var_c_fox * var_c_fox)), ((((((((var_costi0_dn12 * var_costi0) + (var_costi0 * var_costi0_dn12)) * var_c_fox) - (assign9200_e9707 * var_c_fox_dn12)) / (var_c_fox * var_c_fox)) * var_c_fox) - (assign9200_e9709 * var_c_fox_dn12)) / (var_c_fox * var_c_fox)),)
    } else {
        (var_costi3, var_costi3_dn0, var_costi3_dn2, var_costi3_dn4, var_costi3_dn5, var_costi3_dn6, var_costi3_dn8, var_costi3_dn10, var_costi3_dn11, var_costi3_dn12,)
    }
};
        var_costi3 = assign9200_e9713;
        var_costi3_dn0 = assign9200_e9713_d_n0;
        var_costi3_dn2 = assign9200_e9713_d_n2;
        var_costi3_dn4 = assign9200_e9713_d_n4;
        var_costi3_dn5 = assign9200_e9713_d_n5;
        var_costi3_dn6 = assign9200_e9713_d_n6;
        var_costi3_dn8 = assign9200_e9713_d_n8;
        var_costi3_dn10 = assign9200_e9713_d_n10;
        var_costi3_dn11 = assign9200_e9713_d_n11;
        var_costi3_dn12 = assign9200_e9713_d_n12;
        var_costi3_rv = 0.0;

        let (assign9210_e9721, assign9210_e9721_d_n0, assign9210_e9721_d_n2, assign9210_e9721_d_n4, assign9210_e9721_d_n5, assign9210_e9721_d_n6, assign9210_e9721_d_n8, assign9210_e9721_d_n10, assign9210_e9721_d_n11, assign9210_e9721_d_n12,) = {
    if (var_guard124 != 0.0) {
        let assign9210_e9717: f64 = (var_costi3 * var_beta);
        let assign9210_e9719: f64 = (assign9210_e9717 / 2.0);
        (assign9210_e9719, ((var_costi3_dn0 * var_beta) / 2.0), ((var_costi3_dn2 * var_beta) / 2.0), (((var_costi3_dn4 * var_beta) + (var_costi3 * var_beta_dn4)) / 2.0), ((var_costi3_dn5 * var_beta) / 2.0), ((var_costi3_dn6 * var_beta) / 2.0), ((var_costi3_dn8 * var_beta) / 2.0), ((var_costi3_dn10 * var_beta) / 2.0), ((var_costi3_dn11 * var_beta) / 2.0), ((var_costi3_dn12 * var_beta) / 2.0),)
    } else {
        (var_costi4, var_costi4_dn0, var_costi4_dn2, var_costi4_dn4, var_costi4_dn5, var_costi4_dn6, var_costi4_dn8, var_costi4_dn10, var_costi4_dn11, var_costi4_dn12,)
    }
};
        var_costi4 = assign9210_e9721;
        var_costi4_dn0 = assign9210_e9721_d_n0;
        var_costi4_dn2 = assign9210_e9721_d_n2;
        var_costi4_dn4 = assign9210_e9721_d_n4;
        var_costi4_dn5 = assign9210_e9721_d_n5;
        var_costi4_dn6 = assign9210_e9721_d_n6;
        var_costi4_dn8 = assign9210_e9721_d_n8;
        var_costi4_dn10 = assign9210_e9721_d_n10;
        var_costi4_dn11 = assign9210_e9721_d_n11;
        var_costi4_dn12 = assign9210_e9721_d_n12;
        var_costi4_rv = 0.0;

        let (assign9220_e9729, assign9220_e9729_d_n0, assign9220_e9729_d_n2, assign9220_e9729_d_n4, assign9220_e9729_d_n5, assign9220_e9729_d_n6, assign9220_e9729_d_n8, assign9220_e9729_d_n10, assign9220_e9729_d_n11, assign9220_e9729_d_n12,) = {
    if (var_guard124 != 0.0) {
        let assign9220_e9725: f64 = (var_costi4 * var_beta);
        let assign9220_e9727: f64 = (assign9220_e9725 * 2.0);
        (assign9220_e9727, ((var_costi4_dn0 * var_beta) * 2.0), ((var_costi4_dn2 * var_beta) * 2.0), (((var_costi4_dn4 * var_beta) + (var_costi4 * var_beta_dn4)) * 2.0), ((var_costi4_dn5 * var_beta) * 2.0), ((var_costi4_dn6 * var_beta) * 2.0), ((var_costi4_dn8 * var_beta) * 2.0), ((var_costi4_dn10 * var_beta) * 2.0), ((var_costi4_dn11 * var_beta) * 2.0), ((var_costi4_dn12 * var_beta) * 2.0),)
    } else {
        (var_costi5, var_costi5_dn0, var_costi5_dn2, var_costi5_dn4, var_costi5_dn5, var_costi5_dn6, var_costi5_dn8, var_costi5_dn10, var_costi5_dn11, var_costi5_dn12,)
    }
};
        var_costi5 = assign9220_e9729;
        var_costi5_dn0 = assign9220_e9729_d_n0;
        var_costi5_dn2 = assign9220_e9729_d_n2;
        var_costi5_dn4 = assign9220_e9729_d_n4;
        var_costi5_dn5 = assign9220_e9729_d_n5;
        var_costi5_dn6 = assign9220_e9729_d_n6;
        var_costi5_dn8 = assign9220_e9729_d_n8;
        var_costi5_dn10 = assign9220_e9729_d_n10;
        var_costi5_dn11 = assign9220_e9729_d_n11;
        var_costi5_dn12 = assign9220_e9729_d_n12;
        var_costi5_rv = 0.0;

        let (assign9230_e9744, assign9230_e9744_d_n0, assign9230_e9744_d_n2, assign9230_e9744_d_n4, assign9230_e9744_d_n5, assign9230_e9744_d_n6, assign9230_e9744_d_n8, assign9230_e9744_d_n10, assign9230_e9744_d_n11, assign9230_e9744_d_n12,) = {
    if (var_guard124 != 0.0) {
        let assign9230_e9735: f64 = (var_beta * var_vgssti);
        let assign9230_e9737: f64 = (assign9230_e9735 - 1.0);
        let assign9230_e9738: f64 = (4.0 * assign9230_e9737);
        let assign9230_e9740: f64 = (assign9230_e9738 / var_costi5);
        let assign9230_e9741: f64 = (1.0 + assign9230_e9740);
        let assign9230_e9742: f64 = (assign9230_e9741).sqrt();
        (assign9230_e9742, (((((4.0 * (var_beta * var_vgssti_dn0)) * var_costi5) - (assign9230_e9738 * var_costi5_dn0)) / (var_costi5 * var_costi5)) / (2.0 * assign9230_e9742)), (((((4.0 * (var_beta * var_vgssti_dn2)) * var_costi5) - (assign9230_e9738 * var_costi5_dn2)) / (var_costi5 * var_costi5)) / (2.0 * assign9230_e9742)), (((((4.0 * ((var_beta_dn4 * var_vgssti) + (var_beta * var_vgssti_dn4))) * var_costi5) - (assign9230_e9738 * var_costi5_dn4)) / (var_costi5 * var_costi5)) / (2.0 * assign9230_e9742)), (((((4.0 * (var_beta * var_vgssti_dn5)) * var_costi5) - (assign9230_e9738 * var_costi5_dn5)) / (var_costi5 * var_costi5)) / (2.0 * assign9230_e9742)), (((((4.0 * (var_beta * var_vgssti_dn6)) * var_costi5) - (assign9230_e9738 * var_costi5_dn6)) / (var_costi5 * var_costi5)) / (2.0 * assign9230_e9742)), (((((4.0 * (var_beta * var_vgssti_dn8)) * var_costi5) - (assign9230_e9738 * var_costi5_dn8)) / (var_costi5 * var_costi5)) / (2.0 * assign9230_e9742)), (((((4.0 * (var_beta * var_vgssti_dn10)) * var_costi5) - (assign9230_e9738 * var_costi5_dn10)) / (var_costi5 * var_costi5)) / (2.0 * assign9230_e9742)), (((((4.0 * (var_beta * var_vgssti_dn11)) * var_costi5) - (assign9230_e9738 * var_costi5_dn11)) / (var_costi5 * var_costi5)) / (2.0 * assign9230_e9742)), (((((4.0 * (var_beta * var_vgssti_dn12)) * var_costi5) - (assign9230_e9738 * var_costi5_dn12)) / (var_costi5 * var_costi5)) / (2.0 * assign9230_e9742)),)
    } else {
        (var_costi6, var_costi6_dn0, var_costi6_dn2, var_costi6_dn4, var_costi6_dn5, var_costi6_dn6, var_costi6_dn8, var_costi6_dn10, var_costi6_dn11, var_costi6_dn12,)
    }
};
        var_costi6 = assign9230_e9744;
        var_costi6_dn0 = assign9230_e9744_d_n0;
        var_costi6_dn2 = assign9230_e9744_d_n2;
        var_costi6_dn4 = assign9230_e9744_d_n4;
        var_costi6_dn5 = assign9230_e9744_d_n5;
        var_costi6_dn6 = assign9230_e9744_d_n6;
        var_costi6_dn8 = assign9230_e9744_d_n8;
        var_costi6_dn10 = assign9230_e9744_d_n10;
        var_costi6_dn11 = assign9230_e9744_d_n11;
        var_costi6_dn12 = assign9230_e9744_d_n12;
        var_costi6_rv = 0.0;

        let (assign9240_e9754, assign9240_e9754_d_n0, assign9240_e9754_d_n2, assign9240_e9754_d_n4, assign9240_e9754_d_n5, assign9240_e9754_d_n6, assign9240_e9754_d_n8, assign9240_e9754_d_n10, assign9240_e9754_d_n11, assign9240_e9754_d_n12,) = {
    if (var_guard124 != 0.0) {
        let assign9240_e9750: f64 = (1.0 - var_costi6);
        let assign9240_e9751: f64 = (var_costi4 * assign9240_e9750);
        let assign9240_e9752: f64 = (var_vgssti + assign9240_e9751);
        (assign9240_e9752, (var_vgssti_dn0 + ((var_costi4_dn0 * assign9240_e9750) + (var_costi4 * (-var_costi6_dn0)))), (var_vgssti_dn2 + ((var_costi4_dn2 * assign9240_e9750) + (var_costi4 * (-var_costi6_dn2)))), (var_vgssti_dn4 + ((var_costi4_dn4 * assign9240_e9750) + (var_costi4 * (-var_costi6_dn4)))), (var_vgssti_dn5 + ((var_costi4_dn5 * assign9240_e9750) + (var_costi4 * (-var_costi6_dn5)))), (var_vgssti_dn6 + ((var_costi4_dn6 * assign9240_e9750) + (var_costi4 * (-var_costi6_dn6)))), (var_vgssti_dn8 + ((var_costi4_dn8 * assign9240_e9750) + (var_costi4 * (-var_costi6_dn8)))), (var_vgssti_dn10 + ((var_costi4_dn10 * assign9240_e9750) + (var_costi4 * (-var_costi6_dn10)))), (var_vgssti_dn11 + ((var_costi4_dn11 * assign9240_e9750) + (var_costi4 * (-var_costi6_dn11)))), (var_vgssti_dn12 + ((var_costi4_dn12 * assign9240_e9750) + (var_costi4 * (-var_costi6_dn12)))),)
    } else {
        (var_psasti, var_psasti_dn0, var_psasti_dn2, var_psasti_dn4, var_psasti_dn5, var_psasti_dn6, var_psasti_dn8, var_psasti_dn10, var_psasti_dn11, var_psasti_dn12,)
    }
};
        var_psasti = assign9240_e9754;
        var_psasti_dn0 = assign9240_e9754_d_n0;
        var_psasti_dn2 = assign9240_e9754_d_n2;
        var_psasti_dn4 = assign9240_e9754_d_n4;
        var_psasti_dn5 = assign9240_e9754_d_n5;
        var_psasti_dn6 = assign9240_e9754_d_n6;
        var_psasti_dn8 = assign9240_e9754_d_n8;
        var_psasti_dn10 = assign9240_e9754_d_n10;
        var_psasti_dn11 = assign9240_e9754_d_n11;
        var_psasti_dn12 = assign9240_e9754_d_n12;
        var_psasti_rv = 0.0;

        let (assign9250_e9762, assign9250_e9762_d_n0, assign9250_e9762_d_n2, assign9250_e9762_d_n4, assign9250_e9762_d_n5, assign9250_e9762_d_n6, assign9250_e9762_d_n8, assign9250_e9762_d_n10, assign9250_e9762_d_n11, assign9250_e9762_d_n12,) = {
    if (var_guard124 != 0.0) {
        let assign9250_e9758: f64 = (1.0 / var_costi1);
        let assign9250_e9760: f64 = (assign9250_e9758 / var_costi3);
        (assign9250_e9760, ((((-(var_costi1_dn0 / (var_costi1 * var_costi1))) * var_costi3) - (assign9250_e9758 * var_costi3_dn0)) / (var_costi3 * var_costi3)), ((((-(var_costi1_dn2 / (var_costi1 * var_costi1))) * var_costi3) - (assign9250_e9758 * var_costi3_dn2)) / (var_costi3 * var_costi3)), ((((-(var_costi1_dn4 / (var_costi1 * var_costi1))) * var_costi3) - (assign9250_e9758 * var_costi3_dn4)) / (var_costi3 * var_costi3)), ((((-(var_costi1_dn5 / (var_costi1 * var_costi1))) * var_costi3) - (assign9250_e9758 * var_costi3_dn5)) / (var_costi3 * var_costi3)), ((((-(var_costi1_dn6 / (var_costi1 * var_costi1))) * var_costi3) - (assign9250_e9758 * var_costi3_dn6)) / (var_costi3 * var_costi3)), ((((-(var_costi1_dn8 / (var_costi1 * var_costi1))) * var_costi3) - (assign9250_e9758 * var_costi3_dn8)) / (var_costi3 * var_costi3)), ((((-(var_costi1_dn10 / (var_costi1 * var_costi1))) * var_costi3) - (assign9250_e9758 * var_costi3_dn10)) / (var_costi3 * var_costi3)), ((((-(var_costi1_dn11 / (var_costi1 * var_costi1))) * var_costi3) - (assign9250_e9758 * var_costi3_dn11)) / (var_costi3 * var_costi3)), ((((-(var_costi1_dn12 / (var_costi1 * var_costi1))) * var_costi3) - (assign9250_e9758 * var_costi3_dn12)) / (var_costi3 * var_costi3)),)
    } else {
        (var_asti, var_asti_dn0, var_asti_dn2, var_asti_dn4, var_asti_dn5, var_asti_dn6, var_asti_dn8, var_asti_dn10, var_asti_dn11, var_asti_dn12,)
    }
};
        var_asti = assign9250_e9762;
        var_asti_dn0 = assign9250_e9762_d_n0;
        var_asti_dn2 = assign9250_e9762_d_n2;
        var_asti_dn4 = assign9250_e9762_d_n4;
        var_asti_dn5 = assign9250_e9762_d_n5;
        var_asti_dn6 = assign9250_e9762_d_n6;
        var_asti_dn8 = assign9250_e9762_d_n8;
        var_asti_dn10 = assign9250_e9762_d_n10;
        var_asti_dn11 = assign9250_e9762_d_n11;
        var_asti_dn12 = assign9250_e9762_d_n12;
        var_asti_rv = 0.0;

        let (assign9260_e9777, assign9260_e9777_d_n0, assign9260_e9777_d_n2, assign9260_e9777_d_n4, assign9260_e9777_d_n5, assign9260_e9777_d_n6, assign9260_e9777_d_n8, assign9260_e9777_d_n10, assign9260_e9777_d_n11, assign9260_e9777_d_n12,) = {
    if (var_guard124 != 0.0) {
        let assign9260_e9767: f64 = (var_vgssti * var_vgssti);
        let assign9260_e9768: f64 = (var_asti * assign9260_e9767);
        let assign9260_e9769: f64 = (assign9260_e9768).ln();
        let assign9260_e9773: f64 = (2.0 / var_vgssti);
        let assign9260_e9774: f64 = (var_beta + assign9260_e9773);
        let assign9260_e9775: f64 = (assign9260_e9769 / assign9260_e9774);
        (assign9260_e9775, ((((((var_asti_dn0 * assign9260_e9767) + (var_asti * ((var_vgssti_dn0 * var_vgssti) + (var_vgssti * var_vgssti_dn0)))) / assign9260_e9768) * assign9260_e9774) - (assign9260_e9769 * (-((2.0 * var_vgssti_dn0) / (var_vgssti * var_vgssti))))) / (assign9260_e9774 * assign9260_e9774)), ((((((var_asti_dn2 * assign9260_e9767) + (var_asti * ((var_vgssti_dn2 * var_vgssti) + (var_vgssti * var_vgssti_dn2)))) / assign9260_e9768) * assign9260_e9774) - (assign9260_e9769 * (-((2.0 * var_vgssti_dn2) / (var_vgssti * var_vgssti))))) / (assign9260_e9774 * assign9260_e9774)), ((((((var_asti_dn4 * assign9260_e9767) + (var_asti * ((var_vgssti_dn4 * var_vgssti) + (var_vgssti * var_vgssti_dn4)))) / assign9260_e9768) * assign9260_e9774) - (assign9260_e9769 * (var_beta_dn4 + (-((2.0 * var_vgssti_dn4) / (var_vgssti * var_vgssti)))))) / (assign9260_e9774 * assign9260_e9774)), ((((((var_asti_dn5 * assign9260_e9767) + (var_asti * ((var_vgssti_dn5 * var_vgssti) + (var_vgssti * var_vgssti_dn5)))) / assign9260_e9768) * assign9260_e9774) - (assign9260_e9769 * (-((2.0 * var_vgssti_dn5) / (var_vgssti * var_vgssti))))) / (assign9260_e9774 * assign9260_e9774)), ((((((var_asti_dn6 * assign9260_e9767) + (var_asti * ((var_vgssti_dn6 * var_vgssti) + (var_vgssti * var_vgssti_dn6)))) / assign9260_e9768) * assign9260_e9774) - (assign9260_e9769 * (-((2.0 * var_vgssti_dn6) / (var_vgssti * var_vgssti))))) / (assign9260_e9774 * assign9260_e9774)), ((((((var_asti_dn8 * assign9260_e9767) + (var_asti * ((var_vgssti_dn8 * var_vgssti) + (var_vgssti * var_vgssti_dn8)))) / assign9260_e9768) * assign9260_e9774) - (assign9260_e9769 * (-((2.0 * var_vgssti_dn8) / (var_vgssti * var_vgssti))))) / (assign9260_e9774 * assign9260_e9774)), ((((((var_asti_dn10 * assign9260_e9767) + (var_asti * ((var_vgssti_dn10 * var_vgssti) + (var_vgssti * var_vgssti_dn10)))) / assign9260_e9768) * assign9260_e9774) - (assign9260_e9769 * (-((2.0 * var_vgssti_dn10) / (var_vgssti * var_vgssti))))) / (assign9260_e9774 * assign9260_e9774)), ((((((var_asti_dn11 * assign9260_e9767) + (var_asti * ((var_vgssti_dn11 * var_vgssti) + (var_vgssti * var_vgssti_dn11)))) / assign9260_e9768) * assign9260_e9774) - (assign9260_e9769 * (-((2.0 * var_vgssti_dn11) / (var_vgssti * var_vgssti))))) / (assign9260_e9774 * assign9260_e9774)), ((((((var_asti_dn12 * assign9260_e9767) + (var_asti * ((var_vgssti_dn12 * var_vgssti) + (var_vgssti * var_vgssti_dn12)))) / assign9260_e9768) * assign9260_e9774) - (assign9260_e9769 * (-((2.0 * var_vgssti_dn12) / (var_vgssti * var_vgssti))))) / (assign9260_e9774 * assign9260_e9774)),)
    } else {
        (var_psbsti, var_psbsti_dn0, var_psbsti_dn2, var_psbsti_dn4, var_psbsti_dn5, var_psbsti_dn6, var_psbsti_dn8, var_psbsti_dn10, var_psbsti_dn11, var_psbsti_dn12,)
    }
};
        var_psbsti = assign9260_e9777;
        var_psbsti_dn0 = assign9260_e9777_d_n0;
        var_psbsti_dn2 = assign9260_e9777_d_n2;
        var_psbsti_dn4 = assign9260_e9777_d_n4;
        var_psbsti_dn5 = assign9260_e9777_d_n5;
        var_psbsti_dn6 = assign9260_e9777_d_n6;
        var_psbsti_dn8 = assign9260_e9777_d_n8;
        var_psbsti_dn10 = assign9260_e9777_d_n10;
        var_psbsti_dn11 = assign9260_e9777_d_n11;
        var_psbsti_dn12 = assign9260_e9777_d_n12;
        var_psbsti_rv = 0.0;

        let (assign9270_e9785, assign9270_e9785_d_n0, assign9270_e9785_d_n2, assign9270_e9785_d_n4, assign9270_e9785_d_n5, assign9270_e9785_d_n6, assign9270_e9785_d_n8, assign9270_e9785_d_n10, assign9270_e9785_d_n11, assign9270_e9785_d_n12,) = {
    if (var_guard124 != 0.0) {
        let assign9270_e9781: f64 = (var_psbsti - var_psasti);
        let assign9270_e9783: f64 = (assign9270_e9781 - var_sti2_dlt);
        (assign9270_e9783, (var_psbsti_dn0 - var_psasti_dn0), (var_psbsti_dn2 - var_psasti_dn2), (var_psbsti_dn4 - var_psasti_dn4), (var_psbsti_dn5 - var_psasti_dn5), (var_psbsti_dn6 - var_psasti_dn6), (var_psbsti_dn8 - var_psasti_dn8), (var_psbsti_dn10 - var_psasti_dn10), (var_psbsti_dn11 - var_psasti_dn11), (var_psbsti_dn12 - var_psasti_dn12),)
    } else {
        (var_psab, var_psab_dn0, var_psab_dn2, var_psab_dn4, var_psab_dn5, var_psab_dn6, var_psab_dn8, var_psab_dn10, var_psab_dn11, var_psab_dn12,)
    }
};
        var_psab = assign9270_e9785;
        var_psab_dn0 = assign9270_e9785_d_n0;
        var_psab_dn2 = assign9270_e9785_d_n2;
        var_psab_dn4 = assign9270_e9785_d_n4;
        var_psab_dn5 = assign9270_e9785_d_n5;
        var_psab_dn6 = assign9270_e9785_d_n6;
        var_psab_dn8 = assign9270_e9785_d_n8;
        var_psab_dn10 = assign9270_e9785_d_n10;
        var_psab_dn11 = assign9270_e9785_d_n11;
        var_psab_dn12 = assign9270_e9785_d_n12;
        var_psab_rv = 0.0;

        let (assign9280_e9804, assign9280_e9804_d_n0, assign9280_e9804_d_n2, assign9280_e9804_d_n4, assign9280_e9804_d_n5, assign9280_e9804_d_n6, assign9280_e9804_d_n8, assign9280_e9804_d_n10, assign9280_e9804_d_n11, assign9280_e9804_d_n12,) = {
    if (var_guard124 != 0.0) {
        let assign9280_e9792: f64 = (var_psab * var_psab);
        let assign9280_e9795: f64 = (4.0 * var_sti2_dlt);
        let assign9280_e9797: f64 = (assign9280_e9795 * var_psbsti);
        let assign9280_e9798: f64 = (assign9280_e9792 + assign9280_e9797);
        let assign9280_e9799: f64 = (assign9280_e9798).sqrt();
        let assign9280_e9800: f64 = (var_psab + assign9280_e9799);
        let assign9280_e9801: f64 = (0.5 * assign9280_e9800);
        let assign9280_e9802: f64 = (var_psbsti - assign9280_e9801);
        (assign9280_e9802, (var_psbsti_dn0 - (0.5 * (var_psab_dn0 + ((((var_psab_dn0 * var_psab) + (var_psab * var_psab_dn0)) + (assign9280_e9795 * var_psbsti_dn0)) / (2.0 * assign9280_e9799))))), (var_psbsti_dn2 - (0.5 * (var_psab_dn2 + ((((var_psab_dn2 * var_psab) + (var_psab * var_psab_dn2)) + (assign9280_e9795 * var_psbsti_dn2)) / (2.0 * assign9280_e9799))))), (var_psbsti_dn4 - (0.5 * (var_psab_dn4 + ((((var_psab_dn4 * var_psab) + (var_psab * var_psab_dn4)) + (assign9280_e9795 * var_psbsti_dn4)) / (2.0 * assign9280_e9799))))), (var_psbsti_dn5 - (0.5 * (var_psab_dn5 + ((((var_psab_dn5 * var_psab) + (var_psab * var_psab_dn5)) + (assign9280_e9795 * var_psbsti_dn5)) / (2.0 * assign9280_e9799))))), (var_psbsti_dn6 - (0.5 * (var_psab_dn6 + ((((var_psab_dn6 * var_psab) + (var_psab * var_psab_dn6)) + (assign9280_e9795 * var_psbsti_dn6)) / (2.0 * assign9280_e9799))))), (var_psbsti_dn8 - (0.5 * (var_psab_dn8 + ((((var_psab_dn8 * var_psab) + (var_psab * var_psab_dn8)) + (assign9280_e9795 * var_psbsti_dn8)) / (2.0 * assign9280_e9799))))), (var_psbsti_dn10 - (0.5 * (var_psab_dn10 + ((((var_psab_dn10 * var_psab) + (var_psab * var_psab_dn10)) + (assign9280_e9795 * var_psbsti_dn10)) / (2.0 * assign9280_e9799))))), (var_psbsti_dn11 - (0.5 * (var_psab_dn11 + ((((var_psab_dn11 * var_psab) + (var_psab * var_psab_dn11)) + (assign9280_e9795 * var_psbsti_dn11)) / (2.0 * assign9280_e9799))))), (var_psbsti_dn12 - (0.5 * (var_psab_dn12 + ((((var_psab_dn12 * var_psab) + (var_psab * var_psab_dn12)) + (assign9280_e9795 * var_psbsti_dn12)) / (2.0 * assign9280_e9799))))),)
    } else {
        (var_psti, var_psti_dn0, var_psti_dn2, var_psti_dn4, var_psti_dn5, var_psti_dn6, var_psti_dn8, var_psti_dn10, var_psti_dn11, var_psti_dn12,)
    }
};
        var_psti = assign9280_e9804;
        var_psti_dn0 = assign9280_e9804_d_n0;
        var_psti_dn2 = assign9280_e9804_d_n2;
        var_psti_dn4 = assign9280_e9804_d_n4;
        var_psti_dn5 = assign9280_e9804_d_n5;
        var_psti_dn6 = assign9280_e9804_d_n6;
        var_psti_dn8 = assign9280_e9804_d_n8;
        var_psti_dn10 = assign9280_e9804_d_n10;
        var_psti_dn11 = assign9280_e9804_d_n11;
        var_psti_dn12 = assign9280_e9804_d_n12;
        var_psti_rv = 0.0;

        let (assign9290_e9811, assign9290_e9811_d_n0, assign9290_e9811_d_n2, assign9290_e9811_d_n4, assign9290_e9811_d_n5, assign9290_e9811_d_n6, assign9290_e9811_d_n8, assign9290_e9811_d_n10, assign9290_e9811_d_n11, assign9290_e9811_d_n12,) = {
    if (var_guard124 != 0.0) {
        let assign9290_e9808: f64 = (var_beta * var_psti);
        let assign9290_e9809: f64 = (assign9290_e9808).exp();
        (assign9290_e9809, (assign9290_e9809 * (var_beta * var_psti_dn0)), (assign9290_e9809 * (var_beta * var_psti_dn2)), (assign9290_e9809 * ((var_beta_dn4 * var_psti) + (var_beta * var_psti_dn4))), (assign9290_e9809 * (var_beta * var_psti_dn5)), (assign9290_e9809 * (var_beta * var_psti_dn6)), (assign9290_e9809 * (var_beta * var_psti_dn8)), (assign9290_e9809 * (var_beta * var_psti_dn10)), (assign9290_e9809 * (var_beta * var_psti_dn11)), (assign9290_e9809 * (var_beta * var_psti_dn12)),)
    } else {
        (var_expsti, var_expsti_dn0, var_expsti_dn2, var_expsti_dn4, var_expsti_dn5, var_expsti_dn6, var_expsti_dn8, var_expsti_dn10, var_expsti_dn11, var_expsti_dn12,)
    }
};
        var_expsti = assign9290_e9811;
        var_expsti_dn0 = assign9290_e9811_d_n0;
        var_expsti_dn2 = assign9290_e9811_d_n2;
        var_expsti_dn4 = assign9290_e9811_d_n4;
        var_expsti_dn5 = assign9290_e9811_d_n5;
        var_expsti_dn6 = assign9290_e9811_d_n6;
        var_expsti_dn8 = assign9290_e9811_d_n8;
        var_expsti_dn10 = assign9290_e9811_d_n10;
        var_expsti_dn11 = assign9290_e9811_d_n11;
        var_expsti_dn12 = assign9290_e9811_d_n12;
        var_expsti_rv = 0.0;

        let (assign9300_e9823, assign9300_e9823_d_n0, assign9300_e9823_d_n2, assign9300_e9823_d_n4, assign9300_e9823_d_n5, assign9300_e9823_d_n6, assign9300_e9823_d_n8, assign9300_e9823_d_n10, assign9300_e9823_d_n11, assign9300_e9823_d_n12,) = {
    if (var_guard124 != 0.0) {
        let assign9300_e9815: f64 = (var_beta * var_psti);
        let assign9300_e9817: f64 = (assign9300_e9815 - 1.0);
        let assign9300_e9820: f64 = (var_costi1 * var_expsti);
        let assign9300_e9821: f64 = (assign9300_e9817 + assign9300_e9820);
        (assign9300_e9821, ((var_beta * var_psti_dn0) + ((var_costi1_dn0 * var_expsti) + (var_costi1 * var_expsti_dn0))), ((var_beta * var_psti_dn2) + ((var_costi1_dn2 * var_expsti) + (var_costi1 * var_expsti_dn2))), (((var_beta_dn4 * var_psti) + (var_beta * var_psti_dn4)) + ((var_costi1_dn4 * var_expsti) + (var_costi1 * var_expsti_dn4))), ((var_beta * var_psti_dn5) + ((var_costi1_dn5 * var_expsti) + (var_costi1 * var_expsti_dn5))), ((var_beta * var_psti_dn6) + ((var_costi1_dn6 * var_expsti) + (var_costi1 * var_expsti_dn6))), ((var_beta * var_psti_dn8) + ((var_costi1_dn8 * var_expsti) + (var_costi1 * var_expsti_dn8))), ((var_beta * var_psti_dn10) + ((var_costi1_dn10 * var_expsti) + (var_costi1 * var_expsti_dn10))), ((var_beta * var_psti_dn11) + ((var_costi1_dn11 * var_expsti) + (var_costi1 * var_expsti_dn11))), ((var_beta * var_psti_dn12) + ((var_costi1_dn12 * var_expsti) + (var_costi1 * var_expsti_dn12))),)
    } else {
        (var_sq1sti, var_sq1sti_dn0, var_sq1sti_dn2, var_sq1sti_dn4, var_sq1sti_dn5, var_sq1sti_dn6, var_sq1sti_dn8, var_sq1sti_dn10, var_sq1sti_dn11, var_sq1sti_dn12,)
    }
};
        var_sq1sti = assign9300_e9823;
        var_sq1sti_dn0 = assign9300_e9823_d_n0;
        var_sq1sti_dn2 = assign9300_e9823_d_n2;
        var_sq1sti_dn4 = assign9300_e9823_d_n4;
        var_sq1sti_dn5 = assign9300_e9823_d_n5;
        var_sq1sti_dn6 = assign9300_e9823_d_n6;
        var_sq1sti_dn8 = assign9300_e9823_d_n8;
        var_sq1sti_dn10 = assign9300_e9823_d_n10;
        var_sq1sti_dn11 = assign9300_e9823_d_n11;
        var_sq1sti_dn12 = assign9300_e9823_d_n12;
        var_sq1sti_rv = 0.0;

        let (assign9310_e9831, assign9310_e9831_d_n0, assign9310_e9831_d_n2, assign9310_e9831_d_n4, assign9310_e9831_d_n5, assign9310_e9831_d_n6, assign9310_e9831_d_n8, assign9310_e9831_d_n10, assign9310_e9831_d_n11, assign9310_e9831_d_n12,) = {
    if (var_guard124 != 0.0) {
        let assign9310_e9827: f64 = (var_beta * var_psti);
        let assign9310_e9829: f64 = (assign9310_e9827 - 1.0);
        (assign9310_e9829, (var_beta * var_psti_dn0), (var_beta * var_psti_dn2), ((var_beta_dn4 * var_psti) + (var_beta * var_psti_dn4)), (var_beta * var_psti_dn5), (var_beta * var_psti_dn6), (var_beta * var_psti_dn8), (var_beta * var_psti_dn10), (var_beta * var_psti_dn11), (var_beta * var_psti_dn12),)
    } else {
        (var_sq2sti, var_sq2sti_dn0, var_sq2sti_dn2, var_sq2sti_dn4, var_sq2sti_dn5, var_sq2sti_dn6, var_sq2sti_dn8, var_sq2sti_dn10, var_sq2sti_dn11, var_sq2sti_dn12,)
    }
};
        var_sq2sti = assign9310_e9831;
        var_sq2sti_dn0 = assign9310_e9831_d_n0;
        var_sq2sti_dn2 = assign9310_e9831_d_n2;
        var_sq2sti_dn4 = assign9310_e9831_d_n4;
        var_sq2sti_dn5 = assign9310_e9831_d_n5;
        var_sq2sti_dn6 = assign9310_e9831_d_n6;
        var_sq2sti_dn8 = assign9310_e9831_d_n8;
        var_sq2sti_dn10 = assign9310_e9831_d_n10;
        var_sq2sti_dn11 = assign9310_e9831_d_n11;
        var_sq2sti_dn12 = assign9310_e9831_d_n12;
        var_sq2sti_rv = 0.0;

        let assign9320_e9838: f64 = if ((var_sq1sti > 0.0) && (var_sq2sti > 0.0)) { 1.0 } else { 0.0 };
        var_guard125 = assign9320_e9838;
        var_guard125_rv = 0.0;

        let (assign9330_e9853, assign9330_e9853_d_n0, assign9330_e9853_d_n2, assign9330_e9853_d_n4, assign9330_e9853_d_n5, assign9330_e9853_d_n6, assign9330_e9853_d_n8, assign9330_e9853_d_n10, assign9330_e9853_d_n11, assign9330_e9853_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9330_e9844: f64 = (var_beta * var_psti);
        let assign9330_e9846: f64 = (assign9330_e9844 - 1.0);
        let assign9330_e9849: f64 = (var_costi1 * var_expsti);
        let assign9330_e9850: f64 = (assign9330_e9846 + assign9330_e9849);
        let assign9330_e9851: f64 = (assign9330_e9850).sqrt();
        (assign9330_e9851, (((var_beta * var_psti_dn0) + ((var_costi1_dn0 * var_expsti) + (var_costi1 * var_expsti_dn0))) / (2.0 * assign9330_e9851)), (((var_beta * var_psti_dn2) + ((var_costi1_dn2 * var_expsti) + (var_costi1 * var_expsti_dn2))) / (2.0 * assign9330_e9851)), ((((var_beta_dn4 * var_psti) + (var_beta * var_psti_dn4)) + ((var_costi1_dn4 * var_expsti) + (var_costi1 * var_expsti_dn4))) / (2.0 * assign9330_e9851)), (((var_beta * var_psti_dn5) + ((var_costi1_dn5 * var_expsti) + (var_costi1 * var_expsti_dn5))) / (2.0 * assign9330_e9851)), (((var_beta * var_psti_dn6) + ((var_costi1_dn6 * var_expsti) + (var_costi1 * var_expsti_dn6))) / (2.0 * assign9330_e9851)), (((var_beta * var_psti_dn8) + ((var_costi1_dn8 * var_expsti) + (var_costi1 * var_expsti_dn8))) / (2.0 * assign9330_e9851)), (((var_beta * var_psti_dn10) + ((var_costi1_dn10 * var_expsti) + (var_costi1 * var_expsti_dn10))) / (2.0 * assign9330_e9851)), (((var_beta * var_psti_dn11) + ((var_costi1_dn11 * var_expsti) + (var_costi1 * var_expsti_dn11))) / (2.0 * assign9330_e9851)), (((var_beta * var_psti_dn12) + ((var_costi1_dn12 * var_expsti) + (var_costi1 * var_expsti_dn12))) / (2.0 * assign9330_e9851)),)
    } else {
        (var_sq1sti, var_sq1sti_dn0, var_sq1sti_dn2, var_sq1sti_dn4, var_sq1sti_dn5, var_sq1sti_dn6, var_sq1sti_dn8, var_sq1sti_dn10, var_sq1sti_dn11, var_sq1sti_dn12,)
    }
};
        var_sq1sti = assign9330_e9853;
        var_sq1sti_dn0 = assign9330_e9853_d_n0;
        var_sq1sti_dn2 = assign9330_e9853_d_n2;
        var_sq1sti_dn4 = assign9330_e9853_d_n4;
        var_sq1sti_dn5 = assign9330_e9853_d_n5;
        var_sq1sti_dn6 = assign9330_e9853_d_n6;
        var_sq1sti_dn8 = assign9330_e9853_d_n8;
        var_sq1sti_dn10 = assign9330_e9853_d_n10;
        var_sq1sti_dn11 = assign9330_e9853_d_n11;
        var_sq1sti_dn12 = assign9330_e9853_d_n12;
        var_sq1sti_rv = 0.0;

        let (assign9340_e9864, assign9340_e9864_d_n0, assign9340_e9864_d_n2, assign9340_e9864_d_n4, assign9340_e9864_d_n5, assign9340_e9864_d_n6, assign9340_e9864_d_n8, assign9340_e9864_d_n10, assign9340_e9864_d_n11, assign9340_e9864_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9340_e9859: f64 = (var_beta * var_psti);
        let assign9340_e9861: f64 = (assign9340_e9859 - 1.0);
        let assign9340_e9862: f64 = (assign9340_e9861).sqrt();
        (assign9340_e9862, ((var_beta * var_psti_dn0) / (2.0 * assign9340_e9862)), ((var_beta * var_psti_dn2) / (2.0 * assign9340_e9862)), (((var_beta_dn4 * var_psti) + (var_beta * var_psti_dn4)) / (2.0 * assign9340_e9862)), ((var_beta * var_psti_dn5) / (2.0 * assign9340_e9862)), ((var_beta * var_psti_dn6) / (2.0 * assign9340_e9862)), ((var_beta * var_psti_dn8) / (2.0 * assign9340_e9862)), ((var_beta * var_psti_dn10) / (2.0 * assign9340_e9862)), ((var_beta * var_psti_dn11) / (2.0 * assign9340_e9862)), ((var_beta * var_psti_dn12) / (2.0 * assign9340_e9862)),)
    } else {
        (var_sq2sti, var_sq2sti_dn0, var_sq2sti_dn2, var_sq2sti_dn4, var_sq2sti_dn5, var_sq2sti_dn6, var_sq2sti_dn8, var_sq2sti_dn10, var_sq2sti_dn11, var_sq2sti_dn12,)
    }
};
        var_sq2sti = assign9340_e9864;
        var_sq2sti_dn0 = assign9340_e9864_d_n0;
        var_sq2sti_dn2 = assign9340_e9864_d_n2;
        var_sq2sti_dn4 = assign9340_e9864_d_n4;
        var_sq2sti_dn5 = assign9340_e9864_d_n5;
        var_sq2sti_dn6 = assign9340_e9864_d_n6;
        var_sq2sti_dn8 = assign9340_e9864_d_n8;
        var_sq2sti_dn10 = assign9340_e9864_d_n10;
        var_sq2sti_dn11 = assign9340_e9864_d_n11;
        var_sq2sti_dn12 = assign9340_e9864_d_n12;
        var_sq2sti_rv = 0.0;

        let (assign9350_e9874, assign9350_e9874_d_n0, assign9350_e9874_d_n2, assign9350_e9874_d_n4, assign9350_e9874_d_n5, assign9350_e9874_d_n6, assign9350_e9874_d_n8, assign9350_e9874_d_n10, assign9350_e9874_d_n11, assign9350_e9874_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9350_e9871: f64 = (var_sq1sti - var_sq2sti);
        let assign9350_e9872: f64 = (var_costi0 * assign9350_e9871);
        (assign9350_e9872, ((var_costi0_dn0 * assign9350_e9871) + (var_costi0 * (var_sq1sti_dn0 - var_sq2sti_dn0))), ((var_costi0_dn2 * assign9350_e9871) + (var_costi0 * (var_sq1sti_dn2 - var_sq2sti_dn2))), ((var_costi0_dn4 * assign9350_e9871) + (var_costi0 * (var_sq1sti_dn4 - var_sq2sti_dn4))), ((var_costi0_dn5 * assign9350_e9871) + (var_costi0 * (var_sq1sti_dn5 - var_sq2sti_dn5))), ((var_costi0_dn6 * assign9350_e9871) + (var_costi0 * (var_sq1sti_dn6 - var_sq2sti_dn6))), ((var_costi0_dn8 * assign9350_e9871) + (var_costi0 * (var_sq1sti_dn8 - var_sq2sti_dn8))), ((var_costi0_dn10 * assign9350_e9871) + (var_costi0 * (var_sq1sti_dn10 - var_sq2sti_dn10))), ((var_costi0_dn11 * assign9350_e9871) + (var_costi0 * (var_sq1sti_dn11 - var_sq2sti_dn11))), ((var_costi0_dn12 * assign9350_e9871) + (var_costi0 * (var_sq1sti_dn12 - var_sq2sti_dn12))),)
    } else {
        (var_qn0sti, var_qn0sti_dn0, var_qn0sti_dn2, var_qn0sti_dn4, var_qn0sti_dn5, var_qn0sti_dn6, var_qn0sti_dn8, var_qn0sti_dn10, var_qn0sti_dn11, var_qn0sti_dn12,)
    }
};
        var_qn0sti = assign9350_e9874;
        var_qn0sti_dn0 = assign9350_e9874_d_n0;
        var_qn0sti_dn2 = assign9350_e9874_d_n2;
        var_qn0sti_dn4 = assign9350_e9874_d_n4;
        var_qn0sti_dn5 = assign9350_e9874_d_n5;
        var_qn0sti_dn6 = assign9350_e9874_d_n6;
        var_qn0sti_dn8 = assign9350_e9874_d_n8;
        var_qn0sti_dn10 = assign9350_e9874_d_n10;
        var_qn0sti_dn11 = assign9350_e9874_d_n11;
        var_qn0sti_dn12 = assign9350_e9874_d_n12;
        var_qn0sti_rv = 0.0;

        let (assign9360_e9884, assign9360_e9884_d_n0, assign9360_e9884_d_n2, assign9360_e9884_d_n4, assign9360_e9884_d_n5, assign9360_e9884_d_n6, assign9360_e9884_d_n8, assign9360_e9884_d_n10, assign9360_e9884_d_n11, assign9360_e9884_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9360_e9880: f64 = (2.0 * var_weff);
        let assign9360_e9882: f64 = (assign9360_e9880 / var_beta);
        (assign9360_e9882, ((2.0 * var_weff_dn0) / var_beta), ((2.0 * var_weff_dn2) / var_beta), ((((2.0 * var_weff_dn4) * var_beta) - (assign9360_e9880 * var_beta_dn4)) / (var_beta * var_beta)), ((2.0 * var_weff_dn5) / var_beta), ((2.0 * var_weff_dn6) / var_beta), ((2.0 * var_weff_dn8) / var_beta), ((2.0 * var_weff_dn10) / var_beta), ((2.0 * var_weff_dn11) / var_beta), ((2.0 * var_weff_dn12) / var_beta),)
    } else {
        (var_costi7, var_costi7_dn0, var_costi7_dn2, var_costi7_dn4, var_costi7_dn5, var_costi7_dn6, var_costi7_dn8, var_costi7_dn10, var_costi7_dn11, var_costi7_dn12,)
    }
};
        var_costi7 = assign9360_e9884;
        var_costi7_dn0 = assign9360_e9884_d_n0;
        var_costi7_dn2 = assign9360_e9884_d_n2;
        var_costi7_dn4 = assign9360_e9884_d_n4;
        var_costi7_dn5 = assign9360_e9884_d_n5;
        var_costi7_dn6 = assign9360_e9884_d_n6;
        var_costi7_dn8 = assign9360_e9884_d_n8;
        var_costi7_dn10 = assign9360_e9884_d_n10;
        var_costi7_dn11 = assign9360_e9884_d_n11;
        var_costi7_dn12 = assign9360_e9884_d_n12;
        var_costi7_rv = 0.0;

        let (assign9370_e9892, assign9370_e9892_d_n0, assign9370_e9892_d_n2, assign9370_e9892_d_n4, assign9370_e9892_d_n5, assign9370_e9892_d_n6, assign9370_e9892_d_n8, assign9370_e9892_d_n10, assign9370_e9892_d_n11, assign9370_e9892_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9370_e9890: f64 = (300.0 * 0.0001);
        (assign9370_e9890, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mu, var_mu_dn0, var_mu_dn2, var_mu_dn4, var_mu_dn5, var_mu_dn6, var_mu_dn8, var_mu_dn10, var_mu_dn11, var_mu_dn12,)
    }
};
        var_mu = assign9370_e9892;
        var_mu_dn0 = assign9370_e9892_d_n0;
        var_mu_dn2 = assign9370_e9892_d_n2;
        var_mu_dn4 = assign9370_e9892_d_n4;
        var_mu_dn5 = assign9370_e9892_d_n5;
        var_mu_dn6 = assign9370_e9892_d_n6;
        var_mu_dn8 = assign9370_e9892_d_n8;
        var_mu_dn10 = assign9370_e9892_d_n10;
        var_mu_dn11 = assign9370_e9892_d_n11;
        var_mu_dn12 = assign9370_e9892_d_n12;
        var_mu_rv = 0.0;

        let (assign9380_e9898, assign9380_e9898_d_n0, assign9380_e9898_d_n2, assign9380_e9898_d_n4, assign9380_e9898_d_n5, assign9380_e9898_d_n6, assign9380_e9898_d_n8, assign9380_e9898_d_n10, assign9380_e9898_d_n11, assign9380_e9898_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_lred, var_lred_dn0, var_lred_dn2, var_lred_dn4, var_lred_dn5, var_lred_dn6, var_lred_dn8, var_lred_dn10, var_lred_dn11, var_lred_dn12,)
    }
};
        var_lred = assign9380_e9898;
        var_lred_dn0 = assign9380_e9898_d_n0;
        var_lred_dn2 = assign9380_e9898_d_n2;
        var_lred_dn4 = assign9380_e9898_d_n4;
        var_lred_dn5 = assign9380_e9898_d_n5;
        var_lred_dn6 = assign9380_e9898_d_n6;
        var_lred_dn8 = assign9380_e9898_d_n8;
        var_lred_dn10 = assign9380_e9898_d_n10;
        var_lred_dn11 = assign9380_e9898_d_n11;
        var_lred_dn12 = assign9380_e9898_d_n12;
        var_lred_rv = 0.0;

        let (assign9390_e9905, assign9390_e9905_d_n0, assign9390_e9905_d_n2, assign9390_e9905_d_n4, assign9390_e9905_d_n5, assign9390_e9905_d_n6, assign9390_e9905_d_n8, assign9390_e9905_d_n10, assign9390_e9905_d_n11, assign9390_e9905_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9390_e9903: f64 = 0.0;
        (assign9390_e9903, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign9390_e9905;
        var_t1_dn0 = assign9390_e9905_d_n0;
        var_t1_dn2 = assign9390_e9905_d_n2;
        var_t1_dn4 = assign9390_e9905_d_n4;
        var_t1_dn5 = assign9390_e9905_d_n5;
        var_t1_dn6 = assign9390_e9905_d_n6;
        var_t1_dn8 = assign9390_e9905_d_n8;
        var_t1_dn10 = assign9390_e9905_d_n10;
        var_t1_dn11 = assign9390_e9905_d_n11;
        var_t1_dn12 = assign9390_e9905_d_n12;
        var_t1_rv = 0.0;

        let (assign9400_e9921, assign9400_e9921_d_n0, assign9400_e9921_d_n2, assign9400_e9921_d_n4, assign9400_e9921_d_n5, assign9400_e9921_d_n6, assign9400_e9921_d_n8, assign9400_e9921_d_n10, assign9400_e9921_d_n11, assign9400_e9921_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9400_e9911: f64 = (var_costi7 * var_mu);
        let assign9400_e9913: f64 = (assign9400_e9911 * var_qn0sti);
        let assign9400_e9915: f64 = (assign9400_e9913 * var_t1);
        let assign9400_e9918: f64 = (var_leff - var_lred);
        let assign9400_e9919: f64 = (assign9400_e9915 / assign9400_e9918);
        (assign9400_e9919, (((((((((var_costi7_dn0 * var_mu) + (var_costi7 * var_mu_dn0)) * var_qn0sti) + (assign9400_e9911 * var_qn0sti_dn0)) * var_t1) + (assign9400_e9913 * var_t1_dn0)) * assign9400_e9918) - (assign9400_e9915 * (var_leff_dn0 - var_lred_dn0))) / (assign9400_e9918 * assign9400_e9918)), (((((((((var_costi7_dn2 * var_mu) + (var_costi7 * var_mu_dn2)) * var_qn0sti) + (assign9400_e9911 * var_qn0sti_dn2)) * var_t1) + (assign9400_e9913 * var_t1_dn2)) * assign9400_e9918) - (assign9400_e9915 * (var_leff_dn2 - var_lred_dn2))) / (assign9400_e9918 * assign9400_e9918)), (((((((((var_costi7_dn4 * var_mu) + (var_costi7 * var_mu_dn4)) * var_qn0sti) + (assign9400_e9911 * var_qn0sti_dn4)) * var_t1) + (assign9400_e9913 * var_t1_dn4)) * assign9400_e9918) - (assign9400_e9915 * (var_leff_dn4 - var_lred_dn4))) / (assign9400_e9918 * assign9400_e9918)), (((((((((var_costi7_dn5 * var_mu) + (var_costi7 * var_mu_dn5)) * var_qn0sti) + (assign9400_e9911 * var_qn0sti_dn5)) * var_t1) + (assign9400_e9913 * var_t1_dn5)) * assign9400_e9918) - (assign9400_e9915 * (var_leff_dn5 - var_lred_dn5))) / (assign9400_e9918 * assign9400_e9918)), (((((((((var_costi7_dn6 * var_mu) + (var_costi7 * var_mu_dn6)) * var_qn0sti) + (assign9400_e9911 * var_qn0sti_dn6)) * var_t1) + (assign9400_e9913 * var_t1_dn6)) * assign9400_e9918) - (assign9400_e9915 * (var_leff_dn6 - var_lred_dn6))) / (assign9400_e9918 * assign9400_e9918)), (((((((((var_costi7_dn8 * var_mu) + (var_costi7 * var_mu_dn8)) * var_qn0sti) + (assign9400_e9911 * var_qn0sti_dn8)) * var_t1) + (assign9400_e9913 * var_t1_dn8)) * assign9400_e9918) - (assign9400_e9915 * (var_leff_dn8 - var_lred_dn8))) / (assign9400_e9918 * assign9400_e9918)), (((((((((var_costi7_dn10 * var_mu) + (var_costi7 * var_mu_dn10)) * var_qn0sti) + (assign9400_e9911 * var_qn0sti_dn10)) * var_t1) + (assign9400_e9913 * var_t1_dn10)) * assign9400_e9918) - (assign9400_e9915 * (var_leff_dn10 - var_lred_dn10))) / (assign9400_e9918 * assign9400_e9918)), (((((((((var_costi7_dn11 * var_mu) + (var_costi7 * var_mu_dn11)) * var_qn0sti) + (assign9400_e9911 * var_qn0sti_dn11)) * var_t1) + (assign9400_e9913 * var_t1_dn11)) * assign9400_e9918) - (assign9400_e9915 * (var_leff_dn11 - var_lred_dn11))) / (assign9400_e9918 * assign9400_e9918)), (((((((((var_costi7_dn12 * var_mu) + (var_costi7 * var_mu_dn12)) * var_qn0sti) + (assign9400_e9911 * var_qn0sti_dn12)) * var_t1) + (assign9400_e9913 * var_t1_dn12)) * assign9400_e9918) - (assign9400_e9915 * (var_leff_dn12 - var_lred_dn12))) / (assign9400_e9918 * assign9400_e9918)),)
    } else {
        (var_idssti, var_idssti_dn0, var_idssti_dn2, var_idssti_dn4, var_idssti_dn5, var_idssti_dn6, var_idssti_dn8, var_idssti_dn10, var_idssti_dn11, var_idssti_dn12,)
    }
};
        var_idssti = assign9400_e9921;
        var_idssti_dn0 = assign9400_e9921_d_n0;
        var_idssti_dn2 = assign9400_e9921_d_n2;
        var_idssti_dn4 = assign9400_e9921_d_n4;
        var_idssti_dn5 = assign9400_e9921_d_n5;
        var_idssti_dn6 = assign9400_e9921_d_n6;
        var_idssti_dn8 = assign9400_e9921_d_n8;
        var_idssti_dn10 = assign9400_e9921_d_n10;
        var_idssti_dn11 = assign9400_e9921_d_n11;
        var_idssti_dn12 = assign9400_e9921_d_n12;
        var_idssti_rv = 0.0;

        *var_asti_slot = var_asti;
        *var_asti_dn0_slot = var_asti_dn0;
        *var_asti_dn10_slot = var_asti_dn10;
        *var_asti_dn11_slot = var_asti_dn11;
        *var_asti_dn12_slot = var_asti_dn12;
        *var_asti_dn2_slot = var_asti_dn2;
        *var_asti_dn4_slot = var_asti_dn4;
        *var_asti_dn5_slot = var_asti_dn5;
        *var_asti_dn6_slot = var_asti_dn6;
        *var_asti_dn8_slot = var_asti_dn8;
        *var_asti_rv_slot = var_asti_rv;
        *var_costi0_slot = var_costi0;
        *var_costi0_dn0_slot = var_costi0_dn0;
        *var_costi0_dn10_slot = var_costi0_dn10;
        *var_costi0_dn11_slot = var_costi0_dn11;
        *var_costi0_dn12_slot = var_costi0_dn12;
        *var_costi0_dn2_slot = var_costi0_dn2;
        *var_costi0_dn4_slot = var_costi0_dn4;
        *var_costi0_dn5_slot = var_costi0_dn5;
        *var_costi0_dn6_slot = var_costi0_dn6;
        *var_costi0_dn8_slot = var_costi0_dn8;
        *var_costi0_rv_slot = var_costi0_rv;
        *var_costi1_slot = var_costi1;
        *var_costi1_dn0_slot = var_costi1_dn0;
        *var_costi1_dn10_slot = var_costi1_dn10;
        *var_costi1_dn11_slot = var_costi1_dn11;
        *var_costi1_dn12_slot = var_costi1_dn12;
        *var_costi1_dn2_slot = var_costi1_dn2;
        *var_costi1_dn4_slot = var_costi1_dn4;
        *var_costi1_dn5_slot = var_costi1_dn5;
        *var_costi1_dn6_slot = var_costi1_dn6;
        *var_costi1_dn8_slot = var_costi1_dn8;
        *var_costi1_rv_slot = var_costi1_rv;
        *var_costi3_slot = var_costi3;
        *var_costi3_dn0_slot = var_costi3_dn0;
        *var_costi3_dn10_slot = var_costi3_dn10;
        *var_costi3_dn11_slot = var_costi3_dn11;
        *var_costi3_dn12_slot = var_costi3_dn12;
        *var_costi3_dn2_slot = var_costi3_dn2;
        *var_costi3_dn4_slot = var_costi3_dn4;
        *var_costi3_dn5_slot = var_costi3_dn5;
        *var_costi3_dn6_slot = var_costi3_dn6;
        *var_costi3_dn8_slot = var_costi3_dn8;
        *var_costi3_rv_slot = var_costi3_rv;
        *var_costi4_slot = var_costi4;
        *var_costi4_dn0_slot = var_costi4_dn0;
        *var_costi4_dn10_slot = var_costi4_dn10;
        *var_costi4_dn11_slot = var_costi4_dn11;
        *var_costi4_dn12_slot = var_costi4_dn12;
        *var_costi4_dn2_slot = var_costi4_dn2;
        *var_costi4_dn4_slot = var_costi4_dn4;
        *var_costi4_dn5_slot = var_costi4_dn5;
        *var_costi4_dn6_slot = var_costi4_dn6;
        *var_costi4_dn8_slot = var_costi4_dn8;
        *var_costi4_rv_slot = var_costi4_rv;
        *var_costi5_slot = var_costi5;
        *var_costi5_dn0_slot = var_costi5_dn0;
        *var_costi5_dn10_slot = var_costi5_dn10;
        *var_costi5_dn11_slot = var_costi5_dn11;
        *var_costi5_dn12_slot = var_costi5_dn12;
        *var_costi5_dn2_slot = var_costi5_dn2;
        *var_costi5_dn4_slot = var_costi5_dn4;
        *var_costi5_dn5_slot = var_costi5_dn5;
        *var_costi5_dn6_slot = var_costi5_dn6;
        *var_costi5_dn8_slot = var_costi5_dn8;
        *var_costi5_rv_slot = var_costi5_rv;
        *var_costi6_slot = var_costi6;
        *var_costi6_dn0_slot = var_costi6_dn0;
        *var_costi6_dn10_slot = var_costi6_dn10;
        *var_costi6_dn11_slot = var_costi6_dn11;
        *var_costi6_dn12_slot = var_costi6_dn12;
        *var_costi6_dn2_slot = var_costi6_dn2;
        *var_costi6_dn4_slot = var_costi6_dn4;
        *var_costi6_dn5_slot = var_costi6_dn5;
        *var_costi6_dn6_slot = var_costi6_dn6;
        *var_costi6_dn8_slot = var_costi6_dn8;
        *var_costi6_rv_slot = var_costi6_rv;
        *var_costi7_slot = var_costi7;
        *var_costi7_dn0_slot = var_costi7_dn0;
        *var_costi7_dn10_slot = var_costi7_dn10;
        *var_costi7_dn11_slot = var_costi7_dn11;
        *var_costi7_dn12_slot = var_costi7_dn12;
        *var_costi7_dn2_slot = var_costi7_dn2;
        *var_costi7_dn4_slot = var_costi7_dn4;
        *var_costi7_dn5_slot = var_costi7_dn5;
        *var_costi7_dn6_slot = var_costi7_dn6;
        *var_costi7_dn8_slot = var_costi7_dn8;
        *var_costi7_rv_slot = var_costi7_rv;
        *var_expsti_slot = var_expsti;
        *var_expsti_dn0_slot = var_expsti_dn0;
        *var_expsti_dn10_slot = var_expsti_dn10;
        *var_expsti_dn11_slot = var_expsti_dn11;
        *var_expsti_dn12_slot = var_expsti_dn12;
        *var_expsti_dn2_slot = var_expsti_dn2;
        *var_expsti_dn4_slot = var_expsti_dn4;
        *var_expsti_dn5_slot = var_expsti_dn5;
        *var_expsti_dn6_slot = var_expsti_dn6;
        *var_expsti_dn8_slot = var_expsti_dn8;
        *var_expsti_rv_slot = var_expsti_rv;
        *var_guard125_slot = var_guard125;
        *var_guard125_rv_slot = var_guard125_rv;
        *var_idssti_slot = var_idssti;
        *var_idssti_dn0_slot = var_idssti_dn0;
        *var_idssti_dn10_slot = var_idssti_dn10;
        *var_idssti_dn11_slot = var_idssti_dn11;
        *var_idssti_dn12_slot = var_idssti_dn12;
        *var_idssti_dn2_slot = var_idssti_dn2;
        *var_idssti_dn4_slot = var_idssti_dn4;
        *var_idssti_dn5_slot = var_idssti_dn5;
        *var_idssti_dn6_slot = var_idssti_dn6;
        *var_idssti_dn8_slot = var_idssti_dn8;
        *var_idssti_rv_slot = var_idssti_rv;
        *var_lred_slot = var_lred;
        *var_lred_dn0_slot = var_lred_dn0;
        *var_lred_dn10_slot = var_lred_dn10;
        *var_lred_dn11_slot = var_lred_dn11;
        *var_lred_dn12_slot = var_lred_dn12;
        *var_lred_dn2_slot = var_lred_dn2;
        *var_lred_dn4_slot = var_lred_dn4;
        *var_lred_dn5_slot = var_lred_dn5;
        *var_lred_dn6_slot = var_lred_dn6;
        *var_lred_dn8_slot = var_lred_dn8;
        *var_lred_rv_slot = var_lred_rv;
        *var_mu_slot = var_mu;
        *var_mu_dn0_slot = var_mu_dn0;
        *var_mu_dn10_slot = var_mu_dn10;
        *var_mu_dn11_slot = var_mu_dn11;
        *var_mu_dn12_slot = var_mu_dn12;
        *var_mu_dn2_slot = var_mu_dn2;
        *var_mu_dn4_slot = var_mu_dn4;
        *var_mu_dn5_slot = var_mu_dn5;
        *var_mu_dn6_slot = var_mu_dn6;
        *var_mu_dn8_slot = var_mu_dn8;
        *var_mu_rv_slot = var_mu_rv;
        *var_psab_slot = var_psab;
        *var_psab_dn0_slot = var_psab_dn0;
        *var_psab_dn10_slot = var_psab_dn10;
        *var_psab_dn11_slot = var_psab_dn11;
        *var_psab_dn12_slot = var_psab_dn12;
        *var_psab_dn2_slot = var_psab_dn2;
        *var_psab_dn4_slot = var_psab_dn4;
        *var_psab_dn5_slot = var_psab_dn5;
        *var_psab_dn6_slot = var_psab_dn6;
        *var_psab_dn8_slot = var_psab_dn8;
        *var_psab_rv_slot = var_psab_rv;
        *var_psasti_slot = var_psasti;
        *var_psasti_dn0_slot = var_psasti_dn0;
        *var_psasti_dn10_slot = var_psasti_dn10;
        *var_psasti_dn11_slot = var_psasti_dn11;
        *var_psasti_dn12_slot = var_psasti_dn12;
        *var_psasti_dn2_slot = var_psasti_dn2;
        *var_psasti_dn4_slot = var_psasti_dn4;
        *var_psasti_dn5_slot = var_psasti_dn5;
        *var_psasti_dn6_slot = var_psasti_dn6;
        *var_psasti_dn8_slot = var_psasti_dn8;
        *var_psasti_rv_slot = var_psasti_rv;
        *var_psbsti_slot = var_psbsti;
        *var_psbsti_dn0_slot = var_psbsti_dn0;
        *var_psbsti_dn10_slot = var_psbsti_dn10;
        *var_psbsti_dn11_slot = var_psbsti_dn11;
        *var_psbsti_dn12_slot = var_psbsti_dn12;
        *var_psbsti_dn2_slot = var_psbsti_dn2;
        *var_psbsti_dn4_slot = var_psbsti_dn4;
        *var_psbsti_dn5_slot = var_psbsti_dn5;
        *var_psbsti_dn6_slot = var_psbsti_dn6;
        *var_psbsti_dn8_slot = var_psbsti_dn8;
        *var_psbsti_rv_slot = var_psbsti_rv;
        *var_psti_slot = var_psti;
        *var_psti_dn0_slot = var_psti_dn0;
        *var_psti_dn10_slot = var_psti_dn10;
        *var_psti_dn11_slot = var_psti_dn11;
        *var_psti_dn12_slot = var_psti_dn12;
        *var_psti_dn2_slot = var_psti_dn2;
        *var_psti_dn4_slot = var_psti_dn4;
        *var_psti_dn5_slot = var_psti_dn5;
        *var_psti_dn6_slot = var_psti_dn6;
        *var_psti_dn8_slot = var_psti_dn8;
        *var_psti_rv_slot = var_psti_rv;
        *var_qn0sti_slot = var_qn0sti;
        *var_qn0sti_dn0_slot = var_qn0sti_dn0;
        *var_qn0sti_dn10_slot = var_qn0sti_dn10;
        *var_qn0sti_dn11_slot = var_qn0sti_dn11;
        *var_qn0sti_dn12_slot = var_qn0sti_dn12;
        *var_qn0sti_dn2_slot = var_qn0sti_dn2;
        *var_qn0sti_dn4_slot = var_qn0sti_dn4;
        *var_qn0sti_dn5_slot = var_qn0sti_dn5;
        *var_qn0sti_dn6_slot = var_qn0sti_dn6;
        *var_qn0sti_dn8_slot = var_qn0sti_dn8;
        *var_qn0sti_rv_slot = var_qn0sti_rv;
        *var_sq1sti_slot = var_sq1sti;
        *var_sq1sti_dn0_slot = var_sq1sti_dn0;
        *var_sq1sti_dn10_slot = var_sq1sti_dn10;
        *var_sq1sti_dn11_slot = var_sq1sti_dn11;
        *var_sq1sti_dn12_slot = var_sq1sti_dn12;
        *var_sq1sti_dn2_slot = var_sq1sti_dn2;
        *var_sq1sti_dn4_slot = var_sq1sti_dn4;
        *var_sq1sti_dn5_slot = var_sq1sti_dn5;
        *var_sq1sti_dn6_slot = var_sq1sti_dn6;
        *var_sq1sti_dn8_slot = var_sq1sti_dn8;
        *var_sq1sti_rv_slot = var_sq1sti_rv;
        *var_sq2sti_slot = var_sq2sti;
        *var_sq2sti_dn0_slot = var_sq2sti_dn0;
        *var_sq2sti_dn10_slot = var_sq2sti_dn10;
        *var_sq2sti_dn11_slot = var_sq2sti_dn11;
        *var_sq2sti_dn12_slot = var_sq2sti_dn12;
        *var_sq2sti_dn2_slot = var_sq2sti_dn2;
        *var_sq2sti_dn4_slot = var_sq2sti_dn4;
        *var_sq2sti_dn5_slot = var_sq2sti_dn5;
        *var_sq2sti_dn6_slot = var_sq2sti_dn6;
        *var_sq2sti_dn8_slot = var_sq2sti_dn8;
        *var_sq2sti_rv_slot = var_sq2sti_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_rv_slot = var_t1_rv;
        *var_vgssti_slot = var_vgssti;
        *var_vgssti_dn0_slot = var_vgssti_dn0;
        *var_vgssti_dn10_slot = var_vgssti_dn10;
        *var_vgssti_dn11_slot = var_vgssti_dn11;
        *var_vgssti_dn12_slot = var_vgssti_dn12;
        *var_vgssti_dn2_slot = var_vgssti_dn2;
        *var_vgssti_dn4_slot = var_vgssti_dn4;
        *var_vgssti_dn5_slot = var_vgssti_dn5;
        *var_vgssti_dn6_slot = var_vgssti_dn6;
        *var_vgssti_dn8_slot = var_vgssti_dn8;
        *var_vgssti_rv_slot = var_vgssti_rv;
    }

    pub(super) fn stamp_reactive_block_36(
        p: &Parameters,
        var_beta: f64,
        var_beta2: f64,
        var_beta2_dn4: f64,
        var_beta_dn4: f64,
        var_dvthlp: f64,
        var_dvthlp_dn0: f64,
        var_dvthlp_dn10: f64,
        var_dvthlp_dn11: f64,
        var_dvthlp_dn12: f64,
        var_dvthlp_dn2: f64,
        var_dvthlp_dn4: f64,
        var_dvthlp_dn5: f64,
        var_dvthlp_dn6: f64,
        var_dvthlp_dn8: f64,
        var_dvthsc: f64,
        var_dvthsc_dn0: f64,
        var_dvthsc_dn10: f64,
        var_dvthsc_dn11: f64,
        var_dvthsc_dn12: f64,
        var_dvthsc_dn2: f64,
        var_dvthsc_dn4: f64,
        var_dvthsc_dn5: f64,
        var_dvthsc_dn6: f64,
        var_dvthsc_dn8: f64,
        var_eg: f64,
        var_eg_dn0: f64,
        var_eg_dn10: f64,
        var_eg_dn11: f64,
        var_eg_dn12: f64,
        var_eg_dn2: f64,
        var_eg_dn4: f64,
        var_eg_dn5: f64,
        var_eg_dn6: f64,
        var_eg_dn8: f64,
        var_egp12: f64,
        var_egp12_dn0: f64,
        var_egp12_dn10: f64,
        var_egp12_dn11: f64,
        var_egp12_dn12: f64,
        var_egp12_dn2: f64,
        var_egp12_dn4: f64,
        var_egp12_dn5: f64,
        var_egp12_dn6: f64,
        var_egp12_dn8: f64,
        var_fac1p2: f64,
        var_fac1p2_dn0: f64,
        var_fac1p2_dn10: f64,
        var_fac1p2_dn11: f64,
        var_fac1p2_dn12: f64,
        var_fac1p2_dn2: f64,
        var_fac1p2_dn4: f64,
        var_fac1p2_dn5: f64,
        var_fac1p2_dn6: f64,
        var_fac1p2_dn8: f64,
        var_guard124: f64,
        var_guard125: f64,
        var_idssti: f64,
        var_idssti_dn0: f64,
        var_idssti_dn10: f64,
        var_idssti_dn11: f64,
        var_idssti_dn12: f64,
        var_idssti_dn2: f64,
        var_idssti_dn4: f64,
        var_idssti_dn5: f64,
        var_idssti_dn6: f64,
        var_idssti_dn8: f64,
        var_leff: f64,
        var_leff_dn0: f64,
        var_leff_dn10: f64,
        var_leff_dn11: f64,
        var_leff_dn12: f64,
        var_leff_dn2: f64,
        var_leff_dn4: f64,
        var_leff_dn5: f64,
        var_leff_dn6: f64,
        var_leff_dn8: f64,
        var_psti: f64,
        var_psti_dn0: f64,
        var_psti_dn10: f64,
        var_psti_dn11: f64,
        var_psti_dn12: f64,
        var_psti_dn2: f64,
        var_psti_dn4: f64,
        var_psti_dn5: f64,
        var_psti_dn6: f64,
        var_psti_dn8: f64,
        var_tfox0: f64,
        var_vbsz: f64,
        var_vbsz_dn0: f64,
        var_vbsz_dn10: f64,
        var_vbsz_dn11: f64,
        var_vbsz_dn12: f64,
        var_vbsz_dn2: f64,
        var_vbsz_dn4: f64,
        var_vbsz_dn5: f64,
        var_vbsz_dn6: f64,
        var_vbsz_dn8: f64,
        var_vdsz: f64,
        var_vdsz_dn0: f64,
        var_vdsz_dn10: f64,
        var_vdsz_dn11: f64,
        var_vdsz_dn12: f64,
        var_vdsz_dn2: f64,
        var_vdsz_dn4: f64,
        var_vdsz_dn5: f64,
        var_vdsz_dn6: f64,
        var_vdsz_dn8: f64,
        var_vgpz: f64,
        var_vgpz_dn0: f64,
        var_vgpz_dn10: f64,
        var_vgpz_dn11: f64,
        var_vgpz_dn12: f64,
        var_vgpz_dn2: f64,
        var_vgpz_dn4: f64,
        var_vgpz_dn5: f64,
        var_vgpz_dn6: f64,
        var_vgpz_dn8: f64,
        var_weff_nf: f64,
        var_weff_nf_dn0: f64,
        var_weff_nf_dn10: f64,
        var_weff_nf_dn11: f64,
        var_weff_nf_dn12: f64,
        var_weff_nf_dn2: f64,
        var_weff_nf_dn4: f64,
        var_weff_nf_dn5: f64,
        var_weff_nf_dn6: f64,
        var_weff_nf_dn8: f64,
        var_cgs_leff_slot: &mut f64,
        var_cgs_leff_dn0_slot: &mut f64,
        var_cgs_leff_dn10_slot: &mut f64,
        var_cgs_leff_dn11_slot: &mut f64,
        var_cgs_leff_dn12_slot: &mut f64,
        var_cgs_leff_dn2_slot: &mut f64,
        var_cgs_leff_dn4_slot: &mut f64,
        var_cgs_leff_dn5_slot: &mut f64,
        var_cgs_leff_dn6_slot: &mut f64,
        var_cgs_leff_dn8_slot: &mut f64,
        var_cgs_leff_rv_slot: &mut f64,
        var_cgs_tfox0_slot: &mut f64,
        var_cgs_tfox0_rv_slot: &mut f64,
        var_cgs_weff_nf_slot: &mut f64,
        var_cgs_weff_nf_dn0_slot: &mut f64,
        var_cgs_weff_nf_dn10_slot: &mut f64,
        var_cgs_weff_nf_dn11_slot: &mut f64,
        var_cgs_weff_nf_dn12_slot: &mut f64,
        var_cgs_weff_nf_dn2_slot: &mut f64,
        var_cgs_weff_nf_dn4_slot: &mut f64,
        var_cgs_weff_nf_dn5_slot: &mut f64,
        var_cgs_weff_nf_dn6_slot: &mut f64,
        var_cgs_weff_nf_dn8_slot: &mut f64,
        var_cgs_weff_nf_rv_slot: &mut f64,
        var_eevb_wo_vox_slot: &mut f64,
        var_eevb_wo_vox_dn0_slot: &mut f64,
        var_eevb_wo_vox_dn10_slot: &mut f64,
        var_eevb_wo_vox_dn11_slot: &mut f64,
        var_eevb_wo_vox_dn12_slot: &mut f64,
        var_eevb_wo_vox_dn2_slot: &mut f64,
        var_eevb_wo_vox_dn4_slot: &mut f64,
        var_eevb_wo_vox_dn5_slot: &mut f64,
        var_eevb_wo_vox_dn6_slot: &mut f64,
        var_eevb_wo_vox_dn8_slot: &mut f64,
        var_eevb_wo_vox_rv_slot: &mut f64,
        var_evb1_qe_wl_slot: &mut f64,
        var_evb1_qe_wl_dn0_slot: &mut f64,
        var_evb1_qe_wl_dn10_slot: &mut f64,
        var_evb1_qe_wl_dn11_slot: &mut f64,
        var_evb1_qe_wl_dn12_slot: &mut f64,
        var_evb1_qe_wl_dn2_slot: &mut f64,
        var_evb1_qe_wl_dn4_slot: &mut f64,
        var_evb1_qe_wl_dn5_slot: &mut f64,
        var_evb1_qe_wl_dn6_slot: &mut f64,
        var_evb1_qe_wl_dn8_slot: &mut f64,
        var_evb1_qe_wl_p_egp12_slot: &mut f64,
        var_evb1_qe_wl_p_egp12_dn0_slot: &mut f64,
        var_evb1_qe_wl_p_egp12_dn10_slot: &mut f64,
        var_evb1_qe_wl_p_egp12_dn11_slot: &mut f64,
        var_evb1_qe_wl_p_egp12_dn12_slot: &mut f64,
        var_evb1_qe_wl_p_egp12_dn2_slot: &mut f64,
        var_evb1_qe_wl_p_egp12_dn4_slot: &mut f64,
        var_evb1_qe_wl_p_egp12_dn5_slot: &mut f64,
        var_evb1_qe_wl_p_egp12_dn6_slot: &mut f64,
        var_evb1_qe_wl_p_egp12_dn8_slot: &mut f64,
        var_evb1_qe_wl_p_egp12_rv_slot: &mut f64,
        var_evb1_qe_wl_rv_slot: &mut f64,
        var_guard126_slot: &mut f64,
        var_guard126_rv_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_guard127_rv_slot: &mut f64,
        var_guard128_slot: &mut f64,
        var_guard128_rv_slot: &mut f64,
        var_guard148_slot: &mut f64,
        var_guard148_rv_slot: &mut f64,
        var_i_slot: &mut f64,
        var_i_rv_slot: &mut f64,
        var_ids_isub_slot: &mut f64,
        var_ids_isub_dn0_slot: &mut f64,
        var_ids_isub_dn10_slot: &mut f64,
        var_ids_isub_dn11_slot: &mut f64,
        var_ids_isub_dn12_slot: &mut f64,
        var_ids_isub_dn2_slot: &mut f64,
        var_ids_isub_dn4_slot: &mut f64,
        var_ids_isub_dn5_slot: &mut f64,
        var_ids_isub_dn6_slot: &mut f64,
        var_ids_isub_dn8_slot: &mut f64,
        var_ids_isub_rv_slot: &mut f64,
        var_pds_ini_slot: &mut f64,
        var_pds_ini_dn0_slot: &mut f64,
        var_pds_ini_dn10_slot: &mut f64,
        var_pds_ini_dn11_slot: &mut f64,
        var_pds_ini_dn12_slot: &mut f64,
        var_pds_ini_dn2_slot: &mut f64,
        var_pds_ini_dn4_slot: &mut f64,
        var_pds_ini_dn5_slot: &mut f64,
        var_pds_ini_dn6_slot: &mut f64,
        var_pds_ini_dn8_slot: &mut f64,
        var_pds_ini_rv_slot: &mut f64,
        var_pds_max_slot: &mut f64,
        var_pds_max_dn0_slot: &mut f64,
        var_pds_max_dn10_slot: &mut f64,
        var_pds_max_dn11_slot: &mut f64,
        var_pds_max_dn12_slot: &mut f64,
        var_pds_max_dn2_slot: &mut f64,
        var_pds_max_dn4_slot: &mut f64,
        var_pds_max_dn5_slot: &mut f64,
        var_pds_max_dn6_slot: &mut f64,
        var_pds_max_dn8_slot: &mut f64,
        var_pds_max_rv_slot: &mut f64,
        var_pds_qwe_slot: &mut f64,
        var_pds_qwe_dn0_slot: &mut f64,
        var_pds_qwe_dn10_slot: &mut f64,
        var_pds_qwe_dn11_slot: &mut f64,
        var_pds_qwe_dn12_slot: &mut f64,
        var_pds_qwe_dn2_slot: &mut f64,
        var_pds_qwe_dn4_slot: &mut f64,
        var_pds_qwe_dn5_slot: &mut f64,
        var_pds_qwe_dn6_slot: &mut f64,
        var_pds_qwe_dn8_slot: &mut f64,
        var_pds_qwe_rv_slot: &mut f64,
        var_phib_slot: &mut f64,
        var_phib_rv_slot: &mut f64,
        var_ps0_inia_slot: &mut f64,
        var_ps0_inia_dn0_slot: &mut f64,
        var_ps0_inia_dn10_slot: &mut f64,
        var_ps0_inia_dn11_slot: &mut f64,
        var_ps0_inia_dn12_slot: &mut f64,
        var_ps0_inia_dn2_slot: &mut f64,
        var_ps0_inia_dn4_slot: &mut f64,
        var_ps0_inia_dn5_slot: &mut f64,
        var_ps0_inia_dn6_slot: &mut f64,
        var_ps0_inia_dn8_slot: &mut f64,
        var_ps0_inia_rv_slot: &mut f64,
        var_ps0_isub_slot: &mut f64,
        var_ps0_isub_dn0_slot: &mut f64,
        var_ps0_isub_dn10_slot: &mut f64,
        var_ps0_isub_dn11_slot: &mut f64,
        var_ps0_isub_dn12_slot: &mut f64,
        var_ps0_isub_dn2_slot: &mut f64,
        var_ps0_isub_dn4_slot: &mut f64,
        var_ps0_isub_dn5_slot: &mut f64,
        var_ps0_isub_dn6_slot: &mut f64,
        var_ps0_isub_dn8_slot: &mut f64,
        var_ps0_isub_rv_slot: &mut f64,
        var_psl_lim_slot: &mut f64,
        var_psl_lim_dn0_slot: &mut f64,
        var_psl_lim_dn10_slot: &mut f64,
        var_psl_lim_dn11_slot: &mut f64,
        var_psl_lim_dn12_slot: &mut f64,
        var_psl_lim_dn2_slot: &mut f64,
        var_psl_lim_dn4_slot: &mut f64,
        var_psl_lim_dn5_slot: &mut f64,
        var_psl_lim_dn6_slot: &mut f64,
        var_psl_lim_dn8_slot: &mut f64,
        var_psl_lim_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn12_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn12_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
        var_tx_slot: &mut f64,
        var_tx_dn0_slot: &mut f64,
        var_tx_dn10_slot: &mut f64,
        var_tx_dn11_slot: &mut f64,
        var_tx_dn12_slot: &mut f64,
        var_tx_dn2_slot: &mut f64,
        var_tx_dn4_slot: &mut f64,
        var_tx_dn5_slot: &mut f64,
        var_tx_dn6_slot: &mut f64,
        var_tx_dn8_slot: &mut f64,
        var_tx_rv_slot: &mut f64,
    ) {
        let mut var_cgs_leff: f64 = *var_cgs_leff_slot;
        let mut var_cgs_leff_dn0: f64 = *var_cgs_leff_dn0_slot;
        let mut var_cgs_leff_dn10: f64 = *var_cgs_leff_dn10_slot;
        let mut var_cgs_leff_dn11: f64 = *var_cgs_leff_dn11_slot;
        let mut var_cgs_leff_dn12: f64 = *var_cgs_leff_dn12_slot;
        let mut var_cgs_leff_dn2: f64 = *var_cgs_leff_dn2_slot;
        let mut var_cgs_leff_dn4: f64 = *var_cgs_leff_dn4_slot;
        let mut var_cgs_leff_dn5: f64 = *var_cgs_leff_dn5_slot;
        let mut var_cgs_leff_dn6: f64 = *var_cgs_leff_dn6_slot;
        let mut var_cgs_leff_dn8: f64 = *var_cgs_leff_dn8_slot;
        let mut var_cgs_leff_rv: f64 = *var_cgs_leff_rv_slot;
        let mut var_cgs_tfox0: f64 = *var_cgs_tfox0_slot;
        let mut var_cgs_tfox0_rv: f64 = *var_cgs_tfox0_rv_slot;
        let mut var_cgs_weff_nf: f64 = *var_cgs_weff_nf_slot;
        let mut var_cgs_weff_nf_dn0: f64 = *var_cgs_weff_nf_dn0_slot;
        let mut var_cgs_weff_nf_dn10: f64 = *var_cgs_weff_nf_dn10_slot;
        let mut var_cgs_weff_nf_dn11: f64 = *var_cgs_weff_nf_dn11_slot;
        let mut var_cgs_weff_nf_dn12: f64 = *var_cgs_weff_nf_dn12_slot;
        let mut var_cgs_weff_nf_dn2: f64 = *var_cgs_weff_nf_dn2_slot;
        let mut var_cgs_weff_nf_dn4: f64 = *var_cgs_weff_nf_dn4_slot;
        let mut var_cgs_weff_nf_dn5: f64 = *var_cgs_weff_nf_dn5_slot;
        let mut var_cgs_weff_nf_dn6: f64 = *var_cgs_weff_nf_dn6_slot;
        let mut var_cgs_weff_nf_dn8: f64 = *var_cgs_weff_nf_dn8_slot;
        let mut var_cgs_weff_nf_rv: f64 = *var_cgs_weff_nf_rv_slot;
        let mut var_eevb_wo_vox: f64 = *var_eevb_wo_vox_slot;
        let mut var_eevb_wo_vox_dn0: f64 = *var_eevb_wo_vox_dn0_slot;
        let mut var_eevb_wo_vox_dn10: f64 = *var_eevb_wo_vox_dn10_slot;
        let mut var_eevb_wo_vox_dn11: f64 = *var_eevb_wo_vox_dn11_slot;
        let mut var_eevb_wo_vox_dn12: f64 = *var_eevb_wo_vox_dn12_slot;
        let mut var_eevb_wo_vox_dn2: f64 = *var_eevb_wo_vox_dn2_slot;
        let mut var_eevb_wo_vox_dn4: f64 = *var_eevb_wo_vox_dn4_slot;
        let mut var_eevb_wo_vox_dn5: f64 = *var_eevb_wo_vox_dn5_slot;
        let mut var_eevb_wo_vox_dn6: f64 = *var_eevb_wo_vox_dn6_slot;
        let mut var_eevb_wo_vox_dn8: f64 = *var_eevb_wo_vox_dn8_slot;
        let mut var_eevb_wo_vox_rv: f64 = *var_eevb_wo_vox_rv_slot;
        let mut var_evb1_qe_wl: f64 = *var_evb1_qe_wl_slot;
        let mut var_evb1_qe_wl_dn0: f64 = *var_evb1_qe_wl_dn0_slot;
        let mut var_evb1_qe_wl_dn10: f64 = *var_evb1_qe_wl_dn10_slot;
        let mut var_evb1_qe_wl_dn11: f64 = *var_evb1_qe_wl_dn11_slot;
        let mut var_evb1_qe_wl_dn12: f64 = *var_evb1_qe_wl_dn12_slot;
        let mut var_evb1_qe_wl_dn2: f64 = *var_evb1_qe_wl_dn2_slot;
        let mut var_evb1_qe_wl_dn4: f64 = *var_evb1_qe_wl_dn4_slot;
        let mut var_evb1_qe_wl_dn5: f64 = *var_evb1_qe_wl_dn5_slot;
        let mut var_evb1_qe_wl_dn6: f64 = *var_evb1_qe_wl_dn6_slot;
        let mut var_evb1_qe_wl_dn8: f64 = *var_evb1_qe_wl_dn8_slot;
        let mut var_evb1_qe_wl_p_egp12: f64 = *var_evb1_qe_wl_p_egp12_slot;
        let mut var_evb1_qe_wl_p_egp12_dn0: f64 = *var_evb1_qe_wl_p_egp12_dn0_slot;
        let mut var_evb1_qe_wl_p_egp12_dn10: f64 = *var_evb1_qe_wl_p_egp12_dn10_slot;
        let mut var_evb1_qe_wl_p_egp12_dn11: f64 = *var_evb1_qe_wl_p_egp12_dn11_slot;
        let mut var_evb1_qe_wl_p_egp12_dn12: f64 = *var_evb1_qe_wl_p_egp12_dn12_slot;
        let mut var_evb1_qe_wl_p_egp12_dn2: f64 = *var_evb1_qe_wl_p_egp12_dn2_slot;
        let mut var_evb1_qe_wl_p_egp12_dn4: f64 = *var_evb1_qe_wl_p_egp12_dn4_slot;
        let mut var_evb1_qe_wl_p_egp12_dn5: f64 = *var_evb1_qe_wl_p_egp12_dn5_slot;
        let mut var_evb1_qe_wl_p_egp12_dn6: f64 = *var_evb1_qe_wl_p_egp12_dn6_slot;
        let mut var_evb1_qe_wl_p_egp12_dn8: f64 = *var_evb1_qe_wl_p_egp12_dn8_slot;
        let mut var_evb1_qe_wl_p_egp12_rv: f64 = *var_evb1_qe_wl_p_egp12_rv_slot;
        let mut var_evb1_qe_wl_rv: f64 = *var_evb1_qe_wl_rv_slot;
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard126_rv: f64 = *var_guard126_rv_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_guard127_rv: f64 = *var_guard127_rv_slot;
        let mut var_guard128: f64 = *var_guard128_slot;
        let mut var_guard128_rv: f64 = *var_guard128_rv_slot;
        let mut var_guard148: f64 = *var_guard148_slot;
        let mut var_guard148_rv: f64 = *var_guard148_rv_slot;
        let mut var_i: f64 = *var_i_slot;
        let mut var_i_rv: f64 = *var_i_rv_slot;
        let mut var_ids_isub: f64 = *var_ids_isub_slot;
        let mut var_ids_isub_dn0: f64 = *var_ids_isub_dn0_slot;
        let mut var_ids_isub_dn10: f64 = *var_ids_isub_dn10_slot;
        let mut var_ids_isub_dn11: f64 = *var_ids_isub_dn11_slot;
        let mut var_ids_isub_dn12: f64 = *var_ids_isub_dn12_slot;
        let mut var_ids_isub_dn2: f64 = *var_ids_isub_dn2_slot;
        let mut var_ids_isub_dn4: f64 = *var_ids_isub_dn4_slot;
        let mut var_ids_isub_dn5: f64 = *var_ids_isub_dn5_slot;
        let mut var_ids_isub_dn6: f64 = *var_ids_isub_dn6_slot;
        let mut var_ids_isub_dn8: f64 = *var_ids_isub_dn8_slot;
        let mut var_ids_isub_rv: f64 = *var_ids_isub_rv_slot;
        let mut var_pds_ini: f64 = *var_pds_ini_slot;
        let mut var_pds_ini_dn0: f64 = *var_pds_ini_dn0_slot;
        let mut var_pds_ini_dn10: f64 = *var_pds_ini_dn10_slot;
        let mut var_pds_ini_dn11: f64 = *var_pds_ini_dn11_slot;
        let mut var_pds_ini_dn12: f64 = *var_pds_ini_dn12_slot;
        let mut var_pds_ini_dn2: f64 = *var_pds_ini_dn2_slot;
        let mut var_pds_ini_dn4: f64 = *var_pds_ini_dn4_slot;
        let mut var_pds_ini_dn5: f64 = *var_pds_ini_dn5_slot;
        let mut var_pds_ini_dn6: f64 = *var_pds_ini_dn6_slot;
        let mut var_pds_ini_dn8: f64 = *var_pds_ini_dn8_slot;
        let mut var_pds_ini_rv: f64 = *var_pds_ini_rv_slot;
        let mut var_pds_max: f64 = *var_pds_max_slot;
        let mut var_pds_max_dn0: f64 = *var_pds_max_dn0_slot;
        let mut var_pds_max_dn10: f64 = *var_pds_max_dn10_slot;
        let mut var_pds_max_dn11: f64 = *var_pds_max_dn11_slot;
        let mut var_pds_max_dn12: f64 = *var_pds_max_dn12_slot;
        let mut var_pds_max_dn2: f64 = *var_pds_max_dn2_slot;
        let mut var_pds_max_dn4: f64 = *var_pds_max_dn4_slot;
        let mut var_pds_max_dn5: f64 = *var_pds_max_dn5_slot;
        let mut var_pds_max_dn6: f64 = *var_pds_max_dn6_slot;
        let mut var_pds_max_dn8: f64 = *var_pds_max_dn8_slot;
        let mut var_pds_max_rv: f64 = *var_pds_max_rv_slot;
        let mut var_pds_qwe: f64 = *var_pds_qwe_slot;
        let mut var_pds_qwe_dn0: f64 = *var_pds_qwe_dn0_slot;
        let mut var_pds_qwe_dn10: f64 = *var_pds_qwe_dn10_slot;
        let mut var_pds_qwe_dn11: f64 = *var_pds_qwe_dn11_slot;
        let mut var_pds_qwe_dn12: f64 = *var_pds_qwe_dn12_slot;
        let mut var_pds_qwe_dn2: f64 = *var_pds_qwe_dn2_slot;
        let mut var_pds_qwe_dn4: f64 = *var_pds_qwe_dn4_slot;
        let mut var_pds_qwe_dn5: f64 = *var_pds_qwe_dn5_slot;
        let mut var_pds_qwe_dn6: f64 = *var_pds_qwe_dn6_slot;
        let mut var_pds_qwe_dn8: f64 = *var_pds_qwe_dn8_slot;
        let mut var_pds_qwe_rv: f64 = *var_pds_qwe_rv_slot;
        let mut var_phib: f64 = *var_phib_slot;
        let mut var_phib_rv: f64 = *var_phib_rv_slot;
        let mut var_ps0_inia: f64 = *var_ps0_inia_slot;
        let mut var_ps0_inia_dn0: f64 = *var_ps0_inia_dn0_slot;
        let mut var_ps0_inia_dn10: f64 = *var_ps0_inia_dn10_slot;
        let mut var_ps0_inia_dn11: f64 = *var_ps0_inia_dn11_slot;
        let mut var_ps0_inia_dn12: f64 = *var_ps0_inia_dn12_slot;
        let mut var_ps0_inia_dn2: f64 = *var_ps0_inia_dn2_slot;
        let mut var_ps0_inia_dn4: f64 = *var_ps0_inia_dn4_slot;
        let mut var_ps0_inia_dn5: f64 = *var_ps0_inia_dn5_slot;
        let mut var_ps0_inia_dn6: f64 = *var_ps0_inia_dn6_slot;
        let mut var_ps0_inia_dn8: f64 = *var_ps0_inia_dn8_slot;
        let mut var_ps0_inia_rv: f64 = *var_ps0_inia_rv_slot;
        let mut var_ps0_isub: f64 = *var_ps0_isub_slot;
        let mut var_ps0_isub_dn0: f64 = *var_ps0_isub_dn0_slot;
        let mut var_ps0_isub_dn10: f64 = *var_ps0_isub_dn10_slot;
        let mut var_ps0_isub_dn11: f64 = *var_ps0_isub_dn11_slot;
        let mut var_ps0_isub_dn12: f64 = *var_ps0_isub_dn12_slot;
        let mut var_ps0_isub_dn2: f64 = *var_ps0_isub_dn2_slot;
        let mut var_ps0_isub_dn4: f64 = *var_ps0_isub_dn4_slot;
        let mut var_ps0_isub_dn5: f64 = *var_ps0_isub_dn5_slot;
        let mut var_ps0_isub_dn6: f64 = *var_ps0_isub_dn6_slot;
        let mut var_ps0_isub_dn8: f64 = *var_ps0_isub_dn8_slot;
        let mut var_ps0_isub_rv: f64 = *var_ps0_isub_rv_slot;
        let mut var_psl_lim: f64 = *var_psl_lim_slot;
        let mut var_psl_lim_dn0: f64 = *var_psl_lim_dn0_slot;
        let mut var_psl_lim_dn10: f64 = *var_psl_lim_dn10_slot;
        let mut var_psl_lim_dn11: f64 = *var_psl_lim_dn11_slot;
        let mut var_psl_lim_dn12: f64 = *var_psl_lim_dn12_slot;
        let mut var_psl_lim_dn2: f64 = *var_psl_lim_dn2_slot;
        let mut var_psl_lim_dn4: f64 = *var_psl_lim_dn4_slot;
        let mut var_psl_lim_dn5: f64 = *var_psl_lim_dn5_slot;
        let mut var_psl_lim_dn6: f64 = *var_psl_lim_dn6_slot;
        let mut var_psl_lim_dn8: f64 = *var_psl_lim_dn8_slot;
        let mut var_psl_lim_rv: f64 = *var_psl_lim_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn12: f64 = *var_t6_dn12_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn12: f64 = *var_t7_dn12_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;
        let mut var_tx: f64 = *var_tx_slot;
        let mut var_tx_dn0: f64 = *var_tx_dn0_slot;
        let mut var_tx_dn10: f64 = *var_tx_dn10_slot;
        let mut var_tx_dn11: f64 = *var_tx_dn11_slot;
        let mut var_tx_dn12: f64 = *var_tx_dn12_slot;
        let mut var_tx_dn2: f64 = *var_tx_dn2_slot;
        let mut var_tx_dn4: f64 = *var_tx_dn4_slot;
        let mut var_tx_dn5: f64 = *var_tx_dn5_slot;
        let mut var_tx_dn6: f64 = *var_tx_dn6_slot;
        let mut var_tx_dn8: f64 = *var_tx_dn8_slot;
        let mut var_tx_rv: f64 = *var_tx_rv_slot;

        let (assign9410_e9927, assign9410_e9927_d_n0, assign9410_e9927_d_n2, assign9410_e9927_d_n4, assign9410_e9927_d_n5, assign9410_e9927_d_n6, assign9410_e9927_d_n8, assign9410_e9927_d_n10, assign9410_e9927_d_n11, assign9410_e9927_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        (var_idssti, var_idssti_dn0, var_idssti_dn2, var_idssti_dn4, var_idssti_dn5, var_idssti_dn6, var_idssti_dn8, var_idssti_dn10, var_idssti_dn11, var_idssti_dn12,)
    } else {
        (var_ids_isub, var_ids_isub_dn0, var_ids_isub_dn2, var_ids_isub_dn4, var_ids_isub_dn5, var_ids_isub_dn6, var_ids_isub_dn8, var_ids_isub_dn10, var_ids_isub_dn11, var_ids_isub_dn12,)
    }
};
        var_ids_isub = assign9410_e9927;
        var_ids_isub_dn0 = assign9410_e9927_d_n0;
        var_ids_isub_dn2 = assign9410_e9927_d_n2;
        var_ids_isub_dn4 = assign9410_e9927_d_n4;
        var_ids_isub_dn5 = assign9410_e9927_d_n5;
        var_ids_isub_dn6 = assign9410_e9927_d_n6;
        var_ids_isub_dn8 = assign9410_e9927_d_n8;
        var_ids_isub_dn10 = assign9410_e9927_d_n10;
        var_ids_isub_dn11 = assign9410_e9927_d_n11;
        var_ids_isub_dn12 = assign9410_e9927_d_n12;
        var_ids_isub_rv = 0.0;

        let (assign9420_e9933, assign9420_e9933_d_n0, assign9420_e9933_d_n2, assign9420_e9933_d_n4, assign9420_e9933_d_n5, assign9420_e9933_d_n6, assign9420_e9933_d_n8, assign9420_e9933_d_n10, assign9420_e9933_d_n11, assign9420_e9933_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        (var_psti, var_psti_dn0, var_psti_dn2, var_psti_dn4, var_psti_dn5, var_psti_dn6, var_psti_dn8, var_psti_dn10, var_psti_dn11, var_psti_dn12,)
    } else {
        (var_ps0_isub, var_ps0_isub_dn0, var_ps0_isub_dn2, var_ps0_isub_dn4, var_ps0_isub_dn5, var_ps0_isub_dn6, var_ps0_isub_dn8, var_ps0_isub_dn10, var_ps0_isub_dn11, var_ps0_isub_dn12,)
    }
};
        var_ps0_isub = assign9420_e9933;
        var_ps0_isub_dn0 = assign9420_e9933_d_n0;
        var_ps0_isub_dn2 = assign9420_e9933_d_n2;
        var_ps0_isub_dn4 = assign9420_e9933_d_n4;
        var_ps0_isub_dn5 = assign9420_e9933_d_n5;
        var_ps0_isub_dn6 = assign9420_e9933_d_n6;
        var_ps0_isub_dn8 = assign9420_e9933_d_n8;
        var_ps0_isub_dn10 = assign9420_e9933_d_n10;
        var_ps0_isub_dn11 = assign9420_e9933_d_n11;
        var_ps0_isub_dn12 = assign9420_e9933_d_n12;
        var_ps0_isub_rv = 0.0;

        let (assign9430_e9951, assign9430_e9951_d_n0, assign9430_e9951_d_n2, assign9430_e9951_d_n4, assign9430_e9951_d_n5, assign9430_e9951_d_n6, assign9430_e9951_d_n8, assign9430_e9951_d_n10, assign9430_e9951_d_n11, assign9430_e9951_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9430_e9941: f64 = (var_beta * var_vgpz);
        let assign9430_e9943: f64 = (assign9430_e9941 - 1.0);
        let assign9430_e9944: f64 = (4.0 * assign9430_e9943);
        let assign9430_e9947: f64 = (var_fac1p2 * var_beta2);
        let assign9430_e9948: f64 = (assign9430_e9944 / assign9430_e9947);
        let assign9430_e9949: f64 = (1.0 + assign9430_e9948);
        (assign9430_e9949, ((((4.0 * (var_beta * var_vgpz_dn0)) * assign9430_e9947) - (assign9430_e9944 * (var_fac1p2_dn0 * var_beta2))) / (assign9430_e9947 * assign9430_e9947)), ((((4.0 * (var_beta * var_vgpz_dn2)) * assign9430_e9947) - (assign9430_e9944 * (var_fac1p2_dn2 * var_beta2))) / (assign9430_e9947 * assign9430_e9947)), ((((4.0 * ((var_beta_dn4 * var_vgpz) + (var_beta * var_vgpz_dn4))) * assign9430_e9947) - (assign9430_e9944 * ((var_fac1p2_dn4 * var_beta2) + (var_fac1p2 * var_beta2_dn4)))) / (assign9430_e9947 * assign9430_e9947)), ((((4.0 * (var_beta * var_vgpz_dn5)) * assign9430_e9947) - (assign9430_e9944 * (var_fac1p2_dn5 * var_beta2))) / (assign9430_e9947 * assign9430_e9947)), ((((4.0 * (var_beta * var_vgpz_dn6)) * assign9430_e9947) - (assign9430_e9944 * (var_fac1p2_dn6 * var_beta2))) / (assign9430_e9947 * assign9430_e9947)), ((((4.0 * (var_beta * var_vgpz_dn8)) * assign9430_e9947) - (assign9430_e9944 * (var_fac1p2_dn8 * var_beta2))) / (assign9430_e9947 * assign9430_e9947)), ((((4.0 * (var_beta * var_vgpz_dn10)) * assign9430_e9947) - (assign9430_e9944 * (var_fac1p2_dn10 * var_beta2))) / (assign9430_e9947 * assign9430_e9947)), ((((4.0 * (var_beta * var_vgpz_dn11)) * assign9430_e9947) - (assign9430_e9944 * (var_fac1p2_dn11 * var_beta2))) / (assign9430_e9947 * assign9430_e9947)), ((((4.0 * (var_beta * var_vgpz_dn12)) * assign9430_e9947) - (assign9430_e9944 * (var_fac1p2_dn12 * var_beta2))) / (assign9430_e9947 * assign9430_e9947)),)
    } else {
        (var_tx, var_tx_dn0, var_tx_dn2, var_tx_dn4, var_tx_dn5, var_tx_dn6, var_tx_dn8, var_tx_dn10, var_tx_dn11, var_tx_dn12,)
    }
};
        var_tx = assign9430_e9951;
        var_tx_dn0 = assign9430_e9951_d_n0;
        var_tx_dn2 = assign9430_e9951_d_n2;
        var_tx_dn4 = assign9430_e9951_d_n4;
        var_tx_dn5 = assign9430_e9951_d_n5;
        var_tx_dn6 = assign9430_e9951_d_n6;
        var_tx_dn8 = assign9430_e9951_d_n8;
        var_tx_dn10 = assign9430_e9951_d_n10;
        var_tx_dn11 = assign9430_e9951_d_n11;
        var_tx_dn12 = assign9430_e9951_d_n12;
        var_tx_rv = 0.0;

        let assign9440_e9955: f64 = (10.0 * 2.220446049250313e-16);
        let assign9440_e9956: f64 = if var_tx < assign9440_e9955 { 1.0 } else { 0.0 };
        var_guard126 = assign9440_e9956;
        var_guard126_rv = 0.0;

        let (assign9450_e9966, assign9450_e9966_d_n0, assign9450_e9966_d_n2, assign9450_e9966_d_n4, assign9450_e9966_d_n5, assign9450_e9966_d_n6, assign9450_e9966_d_n8, assign9450_e9966_d_n10, assign9450_e9966_d_n11, assign9450_e9966_d_n12,) = {
    if (((var_guard124 != 0.0) && (var_guard125 != 0.0)) && (var_guard126 != 0.0)) {
        let assign9450_e9964: f64 = (10.0 * 2.220446049250313e-16);
        (assign9450_e9964, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tx, var_tx_dn0, var_tx_dn2, var_tx_dn4, var_tx_dn5, var_tx_dn6, var_tx_dn8, var_tx_dn10, var_tx_dn11, var_tx_dn12,)
    }
};
        var_tx = assign9450_e9966;
        var_tx_dn0 = assign9450_e9966_d_n0;
        var_tx_dn2 = assign9450_e9966_d_n2;
        var_tx_dn4 = assign9450_e9966_d_n4;
        var_tx_dn5 = assign9450_e9966_d_n5;
        var_tx_dn6 = assign9450_e9966_d_n6;
        var_tx_dn8 = assign9450_e9966_d_n8;
        var_tx_dn10 = assign9450_e9966_d_n10;
        var_tx_dn11 = assign9450_e9966_d_n11;
        var_tx_dn12 = assign9450_e9966_d_n12;
        var_tx_rv = 0.0;

        let (assign9460_e9983, assign9460_e9983_d_n0, assign9460_e9983_d_n2, assign9460_e9983_d_n4, assign9460_e9983_d_n5, assign9460_e9983_d_n6, assign9460_e9983_d_n8, assign9460_e9983_d_n10, assign9460_e9983_d_n11, assign9460_e9983_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9460_e9973: f64 = (var_fac1p2 * var_beta);
        let assign9460_e9975: f64 = (assign9460_e9973 * 0.5);
        let assign9460_e9978: f64 = (var_tx).sqrt();
        let assign9460_e9979: f64 = (1.0 - assign9460_e9978);
        let assign9460_e9980: f64 = (assign9460_e9975 * assign9460_e9979);
        let assign9460_e9981: f64 = (var_vgpz + assign9460_e9980);
        (assign9460_e9981, (var_vgpz_dn0 + ((((var_fac1p2_dn0 * var_beta) * 0.5) * assign9460_e9979) + (assign9460_e9975 * (-(var_tx_dn0 / (2.0 * assign9460_e9978)))))), (var_vgpz_dn2 + ((((var_fac1p2_dn2 * var_beta) * 0.5) * assign9460_e9979) + (assign9460_e9975 * (-(var_tx_dn2 / (2.0 * assign9460_e9978)))))), (var_vgpz_dn4 + (((((var_fac1p2_dn4 * var_beta) + (var_fac1p2 * var_beta_dn4)) * 0.5) * assign9460_e9979) + (assign9460_e9975 * (-(var_tx_dn4 / (2.0 * assign9460_e9978)))))), (var_vgpz_dn5 + ((((var_fac1p2_dn5 * var_beta) * 0.5) * assign9460_e9979) + (assign9460_e9975 * (-(var_tx_dn5 / (2.0 * assign9460_e9978)))))), (var_vgpz_dn6 + ((((var_fac1p2_dn6 * var_beta) * 0.5) * assign9460_e9979) + (assign9460_e9975 * (-(var_tx_dn6 / (2.0 * assign9460_e9978)))))), (var_vgpz_dn8 + ((((var_fac1p2_dn8 * var_beta) * 0.5) * assign9460_e9979) + (assign9460_e9975 * (-(var_tx_dn8 / (2.0 * assign9460_e9978)))))), (var_vgpz_dn10 + ((((var_fac1p2_dn10 * var_beta) * 0.5) * assign9460_e9979) + (assign9460_e9975 * (-(var_tx_dn10 / (2.0 * assign9460_e9978)))))), (var_vgpz_dn11 + ((((var_fac1p2_dn11 * var_beta) * 0.5) * assign9460_e9979) + (assign9460_e9975 * (-(var_tx_dn11 / (2.0 * assign9460_e9978)))))), (var_vgpz_dn12 + ((((var_fac1p2_dn12 * var_beta) * 0.5) * assign9460_e9979) + (assign9460_e9975 * (-(var_tx_dn12 / (2.0 * assign9460_e9978)))))),)
    } else {
        (var_ps0_inia, var_ps0_inia_dn0, var_ps0_inia_dn2, var_ps0_inia_dn4, var_ps0_inia_dn5, var_ps0_inia_dn6, var_ps0_inia_dn8, var_ps0_inia_dn10, var_ps0_inia_dn11, var_ps0_inia_dn12,)
    }
};
        var_ps0_inia = assign9460_e9983;
        var_ps0_inia_dn0 = assign9460_e9983_d_n0;
        var_ps0_inia_dn2 = assign9460_e9983_d_n2;
        var_ps0_inia_dn4 = assign9460_e9983_d_n4;
        var_ps0_inia_dn5 = assign9460_e9983_d_n5;
        var_ps0_inia_dn6 = assign9460_e9983_d_n6;
        var_ps0_inia_dn8 = assign9460_e9983_d_n8;
        var_ps0_inia_dn10 = assign9460_e9983_d_n10;
        var_ps0_inia_dn11 = assign9460_e9983_d_n11;
        var_ps0_inia_dn12 = assign9460_e9983_d_n12;
        var_ps0_inia_rv = 0.0;

        let (assign9470_e9989, assign9470_e9989_d_n0, assign9470_e9989_d_n2, assign9470_e9989_d_n4, assign9470_e9989_d_n5, assign9470_e9989_d_n6, assign9470_e9989_d_n8, assign9470_e9989_d_n10, assign9470_e9989_d_n11, assign9470_e9989_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        (var_ps0_inia, var_ps0_inia_dn0, var_ps0_inia_dn2, var_ps0_inia_dn4, var_ps0_inia_dn5, var_ps0_inia_dn6, var_ps0_inia_dn8, var_ps0_inia_dn10, var_ps0_inia_dn11, var_ps0_inia_dn12,)
    } else {
        (var_psl_lim, var_psl_lim_dn0, var_psl_lim_dn2, var_psl_lim_dn4, var_psl_lim_dn5, var_psl_lim_dn6, var_psl_lim_dn8, var_psl_lim_dn10, var_psl_lim_dn11, var_psl_lim_dn12,)
    }
};
        var_psl_lim = assign9470_e9989;
        var_psl_lim_dn0 = assign9470_e9989_d_n0;
        var_psl_lim_dn2 = assign9470_e9989_d_n2;
        var_psl_lim_dn4 = assign9470_e9989_d_n4;
        var_psl_lim_dn5 = assign9470_e9989_d_n5;
        var_psl_lim_dn6 = assign9470_e9989_d_n6;
        var_psl_lim_dn8 = assign9470_e9989_d_n8;
        var_psl_lim_dn10 = assign9470_e9989_d_n10;
        var_psl_lim_dn11 = assign9470_e9989_d_n11;
        var_psl_lim_dn12 = assign9470_e9989_d_n12;
        var_psl_lim_rv = 0.0;

        let (assign9480_e9997, assign9480_e9997_d_n0, assign9480_e9997_d_n2, assign9480_e9997_d_n4, assign9480_e9997_d_n5, assign9480_e9997_d_n6, assign9480_e9997_d_n8, assign9480_e9997_d_n10, assign9480_e9997_d_n11, assign9480_e9997_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9480_e9995: f64 = (var_ps0_inia - var_ps0_isub);
        (assign9480_e9995, (var_ps0_inia_dn0 - var_ps0_isub_dn0), (var_ps0_inia_dn2 - var_ps0_isub_dn2), (var_ps0_inia_dn4 - var_ps0_isub_dn4), (var_ps0_inia_dn5 - var_ps0_isub_dn5), (var_ps0_inia_dn6 - var_ps0_isub_dn6), (var_ps0_inia_dn8 - var_ps0_isub_dn8), (var_ps0_inia_dn10 - var_ps0_isub_dn10), (var_ps0_inia_dn11 - var_ps0_isub_dn11), (var_ps0_inia_dn12 - var_ps0_isub_dn12),)
    } else {
        (var_pds_max, var_pds_max_dn0, var_pds_max_dn2, var_pds_max_dn4, var_pds_max_dn5, var_pds_max_dn6, var_pds_max_dn8, var_pds_max_dn10, var_pds_max_dn11, var_pds_max_dn12,)
    }
};
        var_pds_max = assign9480_e9997;
        var_pds_max_dn0 = assign9480_e9997_d_n0;
        var_pds_max_dn2 = assign9480_e9997_d_n2;
        var_pds_max_dn4 = assign9480_e9997_d_n4;
        var_pds_max_dn5 = assign9480_e9997_d_n5;
        var_pds_max_dn6 = assign9480_e9997_d_n6;
        var_pds_max_dn8 = assign9480_e9997_d_n8;
        var_pds_max_dn10 = assign9480_e9997_d_n10;
        var_pds_max_dn11 = assign9480_e9997_d_n11;
        var_pds_max_dn12 = assign9480_e9997_d_n12;
        var_pds_max_rv = 0.0;

        let assign9490_e10000: f64 = if var_pds_max < 0.0 { 1.0 } else { 0.0 };
        var_guard127 = assign9490_e10000;
        var_guard127_rv = 0.0;

        let (assign9500_e10008, assign9500_e10008_d_n0, assign9500_e10008_d_n2, assign9500_e10008_d_n4, assign9500_e10008_d_n5, assign9500_e10008_d_n6, assign9500_e10008_d_n8, assign9500_e10008_d_n10, assign9500_e10008_d_n11, assign9500_e10008_d_n12,) = {
    if (((var_guard124 != 0.0) && (var_guard125 != 0.0)) && (var_guard127 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pds_max, var_pds_max_dn0, var_pds_max_dn2, var_pds_max_dn4, var_pds_max_dn5, var_pds_max_dn6, var_pds_max_dn8, var_pds_max_dn10, var_pds_max_dn11, var_pds_max_dn12,)
    }
};
        var_pds_max = assign9500_e10008;
        var_pds_max_dn0 = assign9500_e10008_d_n0;
        var_pds_max_dn2 = assign9500_e10008_d_n2;
        var_pds_max_dn4 = assign9500_e10008_d_n4;
        var_pds_max_dn5 = assign9500_e10008_d_n5;
        var_pds_max_dn6 = assign9500_e10008_d_n6;
        var_pds_max_dn8 = assign9500_e10008_d_n8;
        var_pds_max_dn10 = assign9500_e10008_d_n10;
        var_pds_max_dn11 = assign9500_e10008_d_n11;
        var_pds_max_dn12 = assign9500_e10008_d_n12;
        var_pds_max_rv = 0.0;

        let (assign9510_e10018, assign9510_e10018_d_n0, assign9510_e10018_d_n2, assign9510_e10018_d_n4, assign9510_e10018_d_n5, assign9510_e10018_d_n6, assign9510_e10018_d_n8, assign9510_e10018_d_n10, assign9510_e10018_d_n11, assign9510_e10018_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9510_e10014: f64 = (1.0 + 0.3);
        let assign9510_e10016: f64 = (assign9510_e10014 * var_pds_max);
        (assign9510_e10016, (assign9510_e10014 * var_pds_max_dn0), (assign9510_e10014 * var_pds_max_dn2), (assign9510_e10014 * var_pds_max_dn4), (assign9510_e10014 * var_pds_max_dn5), (assign9510_e10014 * var_pds_max_dn6), (assign9510_e10014 * var_pds_max_dn8), (assign9510_e10014 * var_pds_max_dn10), (assign9510_e10014 * var_pds_max_dn11), (assign9510_e10014 * var_pds_max_dn12),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
        var_t5 = assign9510_e10018;
        var_t5_dn0 = assign9510_e10018_d_n0;
        var_t5_dn2 = assign9510_e10018_d_n2;
        var_t5_dn4 = assign9510_e10018_d_n4;
        var_t5_dn5 = assign9510_e10018_d_n5;
        var_t5_dn6 = assign9510_e10018_d_n6;
        var_t5_dn8 = assign9510_e10018_d_n8;
        var_t5_dn10 = assign9510_e10018_d_n10;
        var_t5_dn11 = assign9510_e10018_d_n11;
        var_t5_dn12 = assign9510_e10018_d_n12;
        var_t5_rv = 0.0;

        let (assign9520_e10028, assign9520_e10028_d_n0, assign9520_e10028_d_n2, assign9520_e10028_d_n4, assign9520_e10028_d_n5, assign9520_e10028_d_n6, assign9520_e10028_d_n8, assign9520_e10028_d_n10, assign9520_e10028_d_n11, assign9520_e10028_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9520_e10024: f64 = (var_t5 - var_vdsz);
        let assign9520_e10026: f64 = (assign9520_e10024 - 0.03);
        (assign9520_e10026, (var_t5_dn0 - var_vdsz_dn0), (var_t5_dn2 - var_vdsz_dn2), (var_t5_dn4 - var_vdsz_dn4), (var_t5_dn5 - var_vdsz_dn5), (var_t5_dn6 - var_vdsz_dn6), (var_t5_dn8 - var_vdsz_dn8), (var_t5_dn10 - var_vdsz_dn10), (var_t5_dn11 - var_vdsz_dn11), (var_t5_dn12 - var_vdsz_dn12),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
        var_t6 = assign9520_e10028;
        var_t6_dn0 = assign9520_e10028_d_n0;
        var_t6_dn2 = assign9520_e10028_d_n2;
        var_t6_dn4 = assign9520_e10028_d_n4;
        var_t6_dn5 = assign9520_e10028_d_n5;
        var_t6_dn6 = assign9520_e10028_d_n6;
        var_t6_dn8 = assign9520_e10028_d_n8;
        var_t6_dn10 = assign9520_e10028_d_n10;
        var_t6_dn11 = assign9520_e10028_d_n11;
        var_t6_dn12 = assign9520_e10028_d_n12;
        var_t6_rv = 0.0;

        let (assign9530_e10043, assign9530_e10043_d_n0, assign9530_e10043_d_n2, assign9530_e10043_d_n4, assign9530_e10043_d_n5, assign9530_e10043_d_n6, assign9530_e10043_d_n8, assign9530_e10043_d_n10, assign9530_e10043_d_n11, assign9530_e10043_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9530_e10034: f64 = (var_t6 * var_t6);
        let assign9530_e10037: f64 = (4.0 * var_t5);
        let assign9530_e10039: f64 = (assign9530_e10037 * 0.03);
        let assign9530_e10040: f64 = (assign9530_e10034 + assign9530_e10039);
        let assign9530_e10041: f64 = (assign9530_e10040).sqrt();
        (assign9530_e10041, ((((var_t6_dn0 * var_t6) + (var_t6 * var_t6_dn0)) + ((4.0 * var_t5_dn0) * 0.03)) / (2.0 * assign9530_e10041)), ((((var_t6_dn2 * var_t6) + (var_t6 * var_t6_dn2)) + ((4.0 * var_t5_dn2) * 0.03)) / (2.0 * assign9530_e10041)), ((((var_t6_dn4 * var_t6) + (var_t6 * var_t6_dn4)) + ((4.0 * var_t5_dn4) * 0.03)) / (2.0 * assign9530_e10041)), ((((var_t6_dn5 * var_t6) + (var_t6 * var_t6_dn5)) + ((4.0 * var_t5_dn5) * 0.03)) / (2.0 * assign9530_e10041)), ((((var_t6_dn6 * var_t6) + (var_t6 * var_t6_dn6)) + ((4.0 * var_t5_dn6) * 0.03)) / (2.0 * assign9530_e10041)), ((((var_t6_dn8 * var_t6) + (var_t6 * var_t6_dn8)) + ((4.0 * var_t5_dn8) * 0.03)) / (2.0 * assign9530_e10041)), ((((var_t6_dn10 * var_t6) + (var_t6 * var_t6_dn10)) + ((4.0 * var_t5_dn10) * 0.03)) / (2.0 * assign9530_e10041)), ((((var_t6_dn11 * var_t6) + (var_t6 * var_t6_dn11)) + ((4.0 * var_t5_dn11) * 0.03)) / (2.0 * assign9530_e10041)), ((((var_t6_dn12 * var_t6) + (var_t6 * var_t6_dn12)) + ((4.0 * var_t5_dn12) * 0.03)) / (2.0 * assign9530_e10041)),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
        var_t7 = assign9530_e10043;
        var_t7_dn0 = assign9530_e10043_d_n0;
        var_t7_dn2 = assign9530_e10043_d_n2;
        var_t7_dn4 = assign9530_e10043_d_n4;
        var_t7_dn5 = assign9530_e10043_d_n5;
        var_t7_dn6 = assign9530_e10043_d_n6;
        var_t7_dn8 = assign9530_e10043_d_n8;
        var_t7_dn10 = assign9530_e10043_d_n10;
        var_t7_dn11 = assign9530_e10043_d_n11;
        var_t7_dn12 = assign9530_e10043_d_n12;
        var_t7_rv = 0.0;

        let (assign9540_e10055, assign9540_e10055_d_n0, assign9540_e10055_d_n2, assign9540_e10055_d_n4, assign9540_e10055_d_n5, assign9540_e10055_d_n6, assign9540_e10055_d_n8, assign9540_e10055_d_n10, assign9540_e10055_d_n11, assign9540_e10055_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9540_e10051: f64 = (var_t6 + var_t7);
        let assign9540_e10052: f64 = (0.5 * assign9540_e10051);
        let assign9540_e10053: f64 = (var_t5 - assign9540_e10052);
        (assign9540_e10053, (var_t5_dn0 - (0.5 * (var_t6_dn0 + var_t7_dn0))), (var_t5_dn2 - (0.5 * (var_t6_dn2 + var_t7_dn2))), (var_t5_dn4 - (0.5 * (var_t6_dn4 + var_t7_dn4))), (var_t5_dn5 - (0.5 * (var_t6_dn5 + var_t7_dn5))), (var_t5_dn6 - (0.5 * (var_t6_dn6 + var_t7_dn6))), (var_t5_dn8 - (0.5 * (var_t6_dn8 + var_t7_dn8))), (var_t5_dn10 - (0.5 * (var_t6_dn10 + var_t7_dn10))), (var_t5_dn11 - (0.5 * (var_t6_dn11 + var_t7_dn11))), (var_t5_dn12 - (0.5 * (var_t6_dn12 + var_t7_dn12))),)
    } else {
        (var_pds_ini, var_pds_ini_dn0, var_pds_ini_dn2, var_pds_ini_dn4, var_pds_ini_dn5, var_pds_ini_dn6, var_pds_ini_dn8, var_pds_ini_dn10, var_pds_ini_dn11, var_pds_ini_dn12,)
    }
};
        var_pds_ini = assign9540_e10055;
        var_pds_ini_dn0 = assign9540_e10055_d_n0;
        var_pds_ini_dn2 = assign9540_e10055_d_n2;
        var_pds_ini_dn4 = assign9540_e10055_d_n4;
        var_pds_ini_dn5 = assign9540_e10055_d_n5;
        var_pds_ini_dn6 = assign9540_e10055_d_n6;
        var_pds_ini_dn8 = assign9540_e10055_d_n8;
        var_pds_ini_dn10 = assign9540_e10055_d_n10;
        var_pds_ini_dn11 = assign9540_e10055_d_n11;
        var_pds_ini_dn12 = assign9540_e10055_d_n12;
        var_pds_ini_rv = 0.0;

        let assign9550_e10058: f64 = if var_pds_ini > var_pds_max { 1.0 } else { 0.0 };
        var_guard128 = assign9550_e10058;
        var_guard128_rv = 0.0;

        let (assign9560_e10066, assign9560_e10066_d_n0, assign9560_e10066_d_n2, assign9560_e10066_d_n4, assign9560_e10066_d_n5, assign9560_e10066_d_n6, assign9560_e10066_d_n8, assign9560_e10066_d_n10, assign9560_e10066_d_n11, assign9560_e10066_d_n12,) = {
    if (((var_guard124 != 0.0) && (var_guard125 != 0.0)) && (var_guard128 != 0.0)) {
        (var_pds_max, var_pds_max_dn0, var_pds_max_dn2, var_pds_max_dn4, var_pds_max_dn5, var_pds_max_dn6, var_pds_max_dn8, var_pds_max_dn10, var_pds_max_dn11, var_pds_max_dn12,)
    } else {
        (var_pds_ini, var_pds_ini_dn0, var_pds_ini_dn2, var_pds_ini_dn4, var_pds_ini_dn5, var_pds_ini_dn6, var_pds_ini_dn8, var_pds_ini_dn10, var_pds_ini_dn11, var_pds_ini_dn12,)
    }
};
        var_pds_ini = assign9560_e10066;
        var_pds_ini_dn0 = assign9560_e10066_d_n0;
        var_pds_ini_dn2 = assign9560_e10066_d_n2;
        var_pds_ini_dn4 = assign9560_e10066_d_n4;
        var_pds_ini_dn5 = assign9560_e10066_d_n5;
        var_pds_ini_dn6 = assign9560_e10066_d_n6;
        var_pds_ini_dn8 = assign9560_e10066_d_n8;
        var_pds_ini_dn10 = assign9560_e10066_d_n10;
        var_pds_ini_dn11 = assign9560_e10066_d_n11;
        var_pds_ini_dn12 = assign9560_e10066_d_n12;
        var_pds_ini_rv = 0.0;

        let (assign9570_e10072, assign9570_e10072_d_n0, assign9570_e10072_d_n2, assign9570_e10072_d_n4, assign9570_e10072_d_n5, assign9570_e10072_d_n6, assign9570_e10072_d_n8, assign9570_e10072_d_n10, assign9570_e10072_d_n11, assign9570_e10072_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        (var_pds_ini, var_pds_ini_dn0, var_pds_ini_dn2, var_pds_ini_dn4, var_pds_ini_dn5, var_pds_ini_dn6, var_pds_ini_dn8, var_pds_ini_dn10, var_pds_ini_dn11, var_pds_ini_dn12,)
    } else {
        (var_pds_qwe, var_pds_qwe_dn0, var_pds_qwe_dn2, var_pds_qwe_dn4, var_pds_qwe_dn5, var_pds_qwe_dn6, var_pds_qwe_dn8, var_pds_qwe_dn10, var_pds_qwe_dn11, var_pds_qwe_dn12,)
    }
};
        var_pds_qwe = assign9570_e10072;
        var_pds_qwe_dn0 = assign9570_e10072_d_n0;
        var_pds_qwe_dn2 = assign9570_e10072_d_n2;
        var_pds_qwe_dn4 = assign9570_e10072_d_n4;
        var_pds_qwe_dn5 = assign9570_e10072_d_n5;
        var_pds_qwe_dn6 = assign9570_e10072_d_n6;
        var_pds_qwe_dn8 = assign9570_e10072_d_n8;
        var_pds_qwe_dn10 = assign9570_e10072_d_n10;
        var_pds_qwe_dn11 = assign9570_e10072_d_n11;
        var_pds_qwe_dn12 = assign9570_e10072_d_n12;
        var_pds_qwe_rv = 0.0;

        let (assign9580_e10080,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9580_e10078: f64 = (var_tfox0 * 100.0);
        (assign9580_e10078,)
    } else {
        (var_cgs_tfox0,)
    }
};
        var_cgs_tfox0 = assign9580_e10080;
        var_cgs_tfox0_rv = 0.0;

        let (assign9590_e10088, assign9590_e10088_d_n0, assign9590_e10088_d_n2, assign9590_e10088_d_n4, assign9590_e10088_d_n5, assign9590_e10088_d_n6, assign9590_e10088_d_n8, assign9590_e10088_d_n10, assign9590_e10088_d_n11, assign9590_e10088_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9590_e10086: f64 = (var_weff_nf * 100.0);
        (assign9590_e10086, (var_weff_nf_dn0 * 100.0), (var_weff_nf_dn2 * 100.0), (var_weff_nf_dn4 * 100.0), (var_weff_nf_dn5 * 100.0), (var_weff_nf_dn6 * 100.0), (var_weff_nf_dn8 * 100.0), (var_weff_nf_dn10 * 100.0), (var_weff_nf_dn11 * 100.0), (var_weff_nf_dn12 * 100.0),)
    } else {
        (var_cgs_weff_nf, var_cgs_weff_nf_dn0, var_cgs_weff_nf_dn2, var_cgs_weff_nf_dn4, var_cgs_weff_nf_dn5, var_cgs_weff_nf_dn6, var_cgs_weff_nf_dn8, var_cgs_weff_nf_dn10, var_cgs_weff_nf_dn11, var_cgs_weff_nf_dn12,)
    }
};
        var_cgs_weff_nf = assign9590_e10088;
        var_cgs_weff_nf_dn0 = assign9590_e10088_d_n0;
        var_cgs_weff_nf_dn2 = assign9590_e10088_d_n2;
        var_cgs_weff_nf_dn4 = assign9590_e10088_d_n4;
        var_cgs_weff_nf_dn5 = assign9590_e10088_d_n5;
        var_cgs_weff_nf_dn6 = assign9590_e10088_d_n6;
        var_cgs_weff_nf_dn8 = assign9590_e10088_d_n8;
        var_cgs_weff_nf_dn10 = assign9590_e10088_d_n10;
        var_cgs_weff_nf_dn11 = assign9590_e10088_d_n11;
        var_cgs_weff_nf_dn12 = assign9590_e10088_d_n12;
        var_cgs_weff_nf_rv = 0.0;

        let (assign9600_e10096, assign9600_e10096_d_n0, assign9600_e10096_d_n2, assign9600_e10096_d_n4, assign9600_e10096_d_n5, assign9600_e10096_d_n6, assign9600_e10096_d_n8, assign9600_e10096_d_n10, assign9600_e10096_d_n11, assign9600_e10096_d_n12,) = {
    if ((var_guard124 != 0.0) && (var_guard125 != 0.0)) {
        let assign9600_e10094: f64 = (var_leff * 100.0);
        (assign9600_e10094, (var_leff_dn0 * 100.0), (var_leff_dn2 * 100.0), (var_leff_dn4 * 100.0), (var_leff_dn5 * 100.0), (var_leff_dn6 * 100.0), (var_leff_dn8 * 100.0), (var_leff_dn10 * 100.0), (var_leff_dn11 * 100.0), (var_leff_dn12 * 100.0),)
    } else {
        (var_cgs_leff, var_cgs_leff_dn0, var_cgs_leff_dn2, var_cgs_leff_dn4, var_cgs_leff_dn5, var_cgs_leff_dn6, var_cgs_leff_dn8, var_cgs_leff_dn10, var_cgs_leff_dn11, var_cgs_leff_dn12,)
    }
};
        var_cgs_leff = assign9600_e10096;
        var_cgs_leff_dn0 = assign9600_e10096_d_n0;
        var_cgs_leff_dn2 = assign9600_e10096_d_n2;
        var_cgs_leff_dn4 = assign9600_e10096_d_n4;
        var_cgs_leff_dn5 = assign9600_e10096_d_n5;
        var_cgs_leff_dn6 = assign9600_e10096_d_n6;
        var_cgs_leff_dn8 = assign9600_e10096_d_n8;
        var_cgs_leff_dn10 = assign9600_e10096_d_n10;
        var_cgs_leff_dn11 = assign9600_e10096_d_n11;
        var_cgs_leff_dn12 = assign9600_e10096_d_n12;
        var_cgs_leff_rv = 0.0;

        let assign9610_e10099: f64 = if p.p26 == 0.0 { 1.0 } else { 0.0 };
        var_guard148 = assign9610_e10099;
        var_guard148_rv = 0.0;

        let (assign9630_e10116,) = {
    if (((var_guard124 != 0.0) && (var_guard125 != 0.0)) && (var_guard148 == 0.0)) {
        (4.12,)
    } else {
        (var_phib,)
    }
};
        var_phib = assign9630_e10116;
        var_phib_rv = 0.0;

        let (assign9640_e10131, assign9640_e10131_d_n0, assign9640_e10131_d_n2, assign9640_e10131_d_n4, assign9640_e10131_d_n5, assign9640_e10131_d_n6, assign9640_e10131_d_n8, assign9640_e10131_d_n10, assign9640_e10131_d_n11, assign9640_e10131_d_n12,) = {
    if (((var_guard124 != 0.0) && (var_guard125 != 0.0)) && (var_guard148 == 0.0)) {
        let assign9640_e10125: f64 = (p.p141 * 1.6021918e-19);
        let assign9640_e10127: f64 = (assign9640_e10125 * var_cgs_weff_nf);
        let assign9640_e10129: f64 = (assign9640_e10127 * var_cgs_leff);
        (assign9640_e10129, (((assign9640_e10125 * var_cgs_weff_nf_dn0) * var_cgs_leff) + (assign9640_e10127 * var_cgs_leff_dn0)), (((assign9640_e10125 * var_cgs_weff_nf_dn2) * var_cgs_leff) + (assign9640_e10127 * var_cgs_leff_dn2)), (((assign9640_e10125 * var_cgs_weff_nf_dn4) * var_cgs_leff) + (assign9640_e10127 * var_cgs_leff_dn4)), (((assign9640_e10125 * var_cgs_weff_nf_dn5) * var_cgs_leff) + (assign9640_e10127 * var_cgs_leff_dn5)), (((assign9640_e10125 * var_cgs_weff_nf_dn6) * var_cgs_leff) + (assign9640_e10127 * var_cgs_leff_dn6)), (((assign9640_e10125 * var_cgs_weff_nf_dn8) * var_cgs_leff) + (assign9640_e10127 * var_cgs_leff_dn8)), (((assign9640_e10125 * var_cgs_weff_nf_dn10) * var_cgs_leff) + (assign9640_e10127 * var_cgs_leff_dn10)), (((assign9640_e10125 * var_cgs_weff_nf_dn11) * var_cgs_leff) + (assign9640_e10127 * var_cgs_leff_dn11)), (((assign9640_e10125 * var_cgs_weff_nf_dn12) * var_cgs_leff) + (assign9640_e10127 * var_cgs_leff_dn12)),)
    } else {
        (var_evb1_qe_wl, var_evb1_qe_wl_dn0, var_evb1_qe_wl_dn2, var_evb1_qe_wl_dn4, var_evb1_qe_wl_dn5, var_evb1_qe_wl_dn6, var_evb1_qe_wl_dn8, var_evb1_qe_wl_dn10, var_evb1_qe_wl_dn11, var_evb1_qe_wl_dn12,)
    }
};
        var_evb1_qe_wl = assign9640_e10131;
        var_evb1_qe_wl_dn0 = assign9640_e10131_d_n0;
        var_evb1_qe_wl_dn2 = assign9640_e10131_d_n2;
        var_evb1_qe_wl_dn4 = assign9640_e10131_d_n4;
        var_evb1_qe_wl_dn5 = assign9640_e10131_d_n5;
        var_evb1_qe_wl_dn6 = assign9640_e10131_d_n6;
        var_evb1_qe_wl_dn8 = assign9640_e10131_d_n8;
        var_evb1_qe_wl_dn10 = assign9640_e10131_d_n10;
        var_evb1_qe_wl_dn11 = assign9640_e10131_d_n11;
        var_evb1_qe_wl_dn12 = assign9640_e10131_d_n12;
        var_evb1_qe_wl_rv = 0.0;

        let (assign9650_e10142, assign9650_e10142_d_n0, assign9650_e10142_d_n2, assign9650_e10142_d_n4, assign9650_e10142_d_n5, assign9650_e10142_d_n6, assign9650_e10142_d_n8, assign9650_e10142_d_n10, assign9650_e10142_d_n11, assign9650_e10142_d_n12,) = {
    if (((var_guard124 != 0.0) && (var_guard125 != 0.0)) && (var_guard148 == 0.0)) {
        let assign9650_e10140: f64 = (var_evb1_qe_wl / var_egp12);
        (assign9650_e10140, (((var_evb1_qe_wl_dn0 * var_egp12) - (var_evb1_qe_wl * var_egp12_dn0)) / (var_egp12 * var_egp12)), (((var_evb1_qe_wl_dn2 * var_egp12) - (var_evb1_qe_wl * var_egp12_dn2)) / (var_egp12 * var_egp12)), (((var_evb1_qe_wl_dn4 * var_egp12) - (var_evb1_qe_wl * var_egp12_dn4)) / (var_egp12 * var_egp12)), (((var_evb1_qe_wl_dn5 * var_egp12) - (var_evb1_qe_wl * var_egp12_dn5)) / (var_egp12 * var_egp12)), (((var_evb1_qe_wl_dn6 * var_egp12) - (var_evb1_qe_wl * var_egp12_dn6)) / (var_egp12 * var_egp12)), (((var_evb1_qe_wl_dn8 * var_egp12) - (var_evb1_qe_wl * var_egp12_dn8)) / (var_egp12 * var_egp12)), (((var_evb1_qe_wl_dn10 * var_egp12) - (var_evb1_qe_wl * var_egp12_dn10)) / (var_egp12 * var_egp12)), (((var_evb1_qe_wl_dn11 * var_egp12) - (var_evb1_qe_wl * var_egp12_dn11)) / (var_egp12 * var_egp12)), (((var_evb1_qe_wl_dn12 * var_egp12) - (var_evb1_qe_wl * var_egp12_dn12)) / (var_egp12 * var_egp12)),)
    } else {
        (var_evb1_qe_wl_p_egp12, var_evb1_qe_wl_p_egp12_dn0, var_evb1_qe_wl_p_egp12_dn2, var_evb1_qe_wl_p_egp12_dn4, var_evb1_qe_wl_p_egp12_dn5, var_evb1_qe_wl_p_egp12_dn6, var_evb1_qe_wl_p_egp12_dn8, var_evb1_qe_wl_p_egp12_dn10, var_evb1_qe_wl_p_egp12_dn11, var_evb1_qe_wl_p_egp12_dn12,)
    }
};
        var_evb1_qe_wl_p_egp12 = assign9650_e10142;
        var_evb1_qe_wl_p_egp12_dn0 = assign9650_e10142_d_n0;
        var_evb1_qe_wl_p_egp12_dn2 = assign9650_e10142_d_n2;
        var_evb1_qe_wl_p_egp12_dn4 = assign9650_e10142_d_n4;
        var_evb1_qe_wl_p_egp12_dn5 = assign9650_e10142_d_n5;
        var_evb1_qe_wl_p_egp12_dn6 = assign9650_e10142_d_n6;
        var_evb1_qe_wl_p_egp12_dn8 = assign9650_e10142_d_n8;
        var_evb1_qe_wl_p_egp12_dn10 = assign9650_e10142_d_n10;
        var_evb1_qe_wl_p_egp12_dn11 = assign9650_e10142_d_n11;
        var_evb1_qe_wl_p_egp12_dn12 = assign9650_e10142_d_n12;
        var_evb1_qe_wl_p_egp12_rv = 0.0;

        let (assign9660_e10164, assign9660_e10164_d_n0, assign9660_e10164_d_n2, assign9660_e10164_d_n4, assign9660_e10164_d_n5, assign9660_e10164_d_n6, assign9660_e10164_d_n8, assign9660_e10164_d_n10, assign9660_e10164_d_n11, assign9660_e10164_d_n12,) = {
    if (((var_guard124 != 0.0) && (var_guard125 != 0.0)) && (var_guard148 == 0.0)) {
        let assign9660_e10151: f64 = (p.p144 * var_vbsz);
        let assign9660_e10153: f64 = (assign9660_e10151 + var_dvthsc);
        let assign9660_e10155: f64 = (assign9660_e10153 + var_dvthlp);
        let assign9660_e10157: f64 = (assign9660_e10155 + var_eg);
        let assign9660_e10159: f64 = (assign9660_e10157 + p.p143);
        let assign9660_e10160: f64 = (-assign9660_e10159);
        let assign9660_e10162: f64 = (assign9660_e10160 / var_cgs_tfox0);
        (assign9660_e10162, ((-((((p.p144 * var_vbsz_dn0) + var_dvthsc_dn0) + var_dvthlp_dn0) + var_eg_dn0)) / var_cgs_tfox0), ((-((((p.p144 * var_vbsz_dn2) + var_dvthsc_dn2) + var_dvthlp_dn2) + var_eg_dn2)) / var_cgs_tfox0), ((-((((p.p144 * var_vbsz_dn4) + var_dvthsc_dn4) + var_dvthlp_dn4) + var_eg_dn4)) / var_cgs_tfox0), ((-((((p.p144 * var_vbsz_dn5) + var_dvthsc_dn5) + var_dvthlp_dn5) + var_eg_dn5)) / var_cgs_tfox0), ((-((((p.p144 * var_vbsz_dn6) + var_dvthsc_dn6) + var_dvthlp_dn6) + var_eg_dn6)) / var_cgs_tfox0), ((-((((p.p144 * var_vbsz_dn8) + var_dvthsc_dn8) + var_dvthlp_dn8) + var_eg_dn8)) / var_cgs_tfox0), ((-((((p.p144 * var_vbsz_dn10) + var_dvthsc_dn10) + var_dvthlp_dn10) + var_eg_dn10)) / var_cgs_tfox0), ((-((((p.p144 * var_vbsz_dn11) + var_dvthsc_dn11) + var_dvthlp_dn11) + var_eg_dn11)) / var_cgs_tfox0), ((-((((p.p144 * var_vbsz_dn12) + var_dvthsc_dn12) + var_dvthlp_dn12) + var_eg_dn12)) / var_cgs_tfox0),)
    } else {
        (var_eevb_wo_vox, var_eevb_wo_vox_dn0, var_eevb_wo_vox_dn2, var_eevb_wo_vox_dn4, var_eevb_wo_vox_dn5, var_eevb_wo_vox_dn6, var_eevb_wo_vox_dn8, var_eevb_wo_vox_dn10, var_eevb_wo_vox_dn11, var_eevb_wo_vox_dn12,)
    }
};
        var_eevb_wo_vox = assign9660_e10164;
        var_eevb_wo_vox_dn0 = assign9660_e10164_d_n0;
        var_eevb_wo_vox_dn2 = assign9660_e10164_d_n2;
        var_eevb_wo_vox_dn4 = assign9660_e10164_d_n4;
        var_eevb_wo_vox_dn5 = assign9660_e10164_d_n5;
        var_eevb_wo_vox_dn6 = assign9660_e10164_d_n6;
        var_eevb_wo_vox_dn8 = assign9660_e10164_d_n8;
        var_eevb_wo_vox_dn10 = assign9660_e10164_d_n10;
        var_eevb_wo_vox_dn11 = assign9660_e10164_d_n11;
        var_eevb_wo_vox_dn12 = assign9660_e10164_d_n12;
        var_eevb_wo_vox_rv = 0.0;

        let (assign9670_e10173,) = {
    if (((var_guard124 != 0.0) && (var_guard125 != 0.0)) && (var_guard148 == 0.0)) {
        (0.0,)
    } else {
        (var_i,)
    }
};
        var_i = assign9670_e10173;
        var_i_rv = 0.0;

        *var_cgs_leff_slot = var_cgs_leff;
        *var_cgs_leff_dn0_slot = var_cgs_leff_dn0;
        *var_cgs_leff_dn10_slot = var_cgs_leff_dn10;
        *var_cgs_leff_dn11_slot = var_cgs_leff_dn11;
        *var_cgs_leff_dn12_slot = var_cgs_leff_dn12;
        *var_cgs_leff_dn2_slot = var_cgs_leff_dn2;
        *var_cgs_leff_dn4_slot = var_cgs_leff_dn4;
        *var_cgs_leff_dn5_slot = var_cgs_leff_dn5;
        *var_cgs_leff_dn6_slot = var_cgs_leff_dn6;
        *var_cgs_leff_dn8_slot = var_cgs_leff_dn8;
        *var_cgs_leff_rv_slot = var_cgs_leff_rv;
        *var_cgs_tfox0_slot = var_cgs_tfox0;
        *var_cgs_tfox0_rv_slot = var_cgs_tfox0_rv;
        *var_cgs_weff_nf_slot = var_cgs_weff_nf;
        *var_cgs_weff_nf_dn0_slot = var_cgs_weff_nf_dn0;
        *var_cgs_weff_nf_dn10_slot = var_cgs_weff_nf_dn10;
        *var_cgs_weff_nf_dn11_slot = var_cgs_weff_nf_dn11;
        *var_cgs_weff_nf_dn12_slot = var_cgs_weff_nf_dn12;
        *var_cgs_weff_nf_dn2_slot = var_cgs_weff_nf_dn2;
        *var_cgs_weff_nf_dn4_slot = var_cgs_weff_nf_dn4;
        *var_cgs_weff_nf_dn5_slot = var_cgs_weff_nf_dn5;
        *var_cgs_weff_nf_dn6_slot = var_cgs_weff_nf_dn6;
        *var_cgs_weff_nf_dn8_slot = var_cgs_weff_nf_dn8;
        *var_cgs_weff_nf_rv_slot = var_cgs_weff_nf_rv;
        *var_eevb_wo_vox_slot = var_eevb_wo_vox;
        *var_eevb_wo_vox_dn0_slot = var_eevb_wo_vox_dn0;
        *var_eevb_wo_vox_dn10_slot = var_eevb_wo_vox_dn10;
        *var_eevb_wo_vox_dn11_slot = var_eevb_wo_vox_dn11;
        *var_eevb_wo_vox_dn12_slot = var_eevb_wo_vox_dn12;
        *var_eevb_wo_vox_dn2_slot = var_eevb_wo_vox_dn2;
        *var_eevb_wo_vox_dn4_slot = var_eevb_wo_vox_dn4;
        *var_eevb_wo_vox_dn5_slot = var_eevb_wo_vox_dn5;
        *var_eevb_wo_vox_dn6_slot = var_eevb_wo_vox_dn6;
        *var_eevb_wo_vox_dn8_slot = var_eevb_wo_vox_dn8;
        *var_eevb_wo_vox_rv_slot = var_eevb_wo_vox_rv;
        *var_evb1_qe_wl_slot = var_evb1_qe_wl;
        *var_evb1_qe_wl_dn0_slot = var_evb1_qe_wl_dn0;
        *var_evb1_qe_wl_dn10_slot = var_evb1_qe_wl_dn10;
        *var_evb1_qe_wl_dn11_slot = var_evb1_qe_wl_dn11;
        *var_evb1_qe_wl_dn12_slot = var_evb1_qe_wl_dn12;
        *var_evb1_qe_wl_dn2_slot = var_evb1_qe_wl_dn2;
        *var_evb1_qe_wl_dn4_slot = var_evb1_qe_wl_dn4;
        *var_evb1_qe_wl_dn5_slot = var_evb1_qe_wl_dn5;
        *var_evb1_qe_wl_dn6_slot = var_evb1_qe_wl_dn6;
        *var_evb1_qe_wl_dn8_slot = var_evb1_qe_wl_dn8;
        *var_evb1_qe_wl_p_egp12_slot = var_evb1_qe_wl_p_egp12;
        *var_evb1_qe_wl_p_egp12_dn0_slot = var_evb1_qe_wl_p_egp12_dn0;
        *var_evb1_qe_wl_p_egp12_dn10_slot = var_evb1_qe_wl_p_egp12_dn10;
        *var_evb1_qe_wl_p_egp12_dn11_slot = var_evb1_qe_wl_p_egp12_dn11;
        *var_evb1_qe_wl_p_egp12_dn12_slot = var_evb1_qe_wl_p_egp12_dn12;
        *var_evb1_qe_wl_p_egp12_dn2_slot = var_evb1_qe_wl_p_egp12_dn2;
        *var_evb1_qe_wl_p_egp12_dn4_slot = var_evb1_qe_wl_p_egp12_dn4;
        *var_evb1_qe_wl_p_egp12_dn5_slot = var_evb1_qe_wl_p_egp12_dn5;
        *var_evb1_qe_wl_p_egp12_dn6_slot = var_evb1_qe_wl_p_egp12_dn6;
        *var_evb1_qe_wl_p_egp12_dn8_slot = var_evb1_qe_wl_p_egp12_dn8;
        *var_evb1_qe_wl_p_egp12_rv_slot = var_evb1_qe_wl_p_egp12_rv;
        *var_evb1_qe_wl_rv_slot = var_evb1_qe_wl_rv;
        *var_guard126_slot = var_guard126;
        *var_guard126_rv_slot = var_guard126_rv;
        *var_guard127_slot = var_guard127;
        *var_guard127_rv_slot = var_guard127_rv;
        *var_guard128_slot = var_guard128;
        *var_guard128_rv_slot = var_guard128_rv;
        *var_guard148_slot = var_guard148;
        *var_guard148_rv_slot = var_guard148_rv;
        *var_i_slot = var_i;
        *var_i_rv_slot = var_i_rv;
        *var_ids_isub_slot = var_ids_isub;
        *var_ids_isub_dn0_slot = var_ids_isub_dn0;
        *var_ids_isub_dn10_slot = var_ids_isub_dn10;
        *var_ids_isub_dn11_slot = var_ids_isub_dn11;
        *var_ids_isub_dn12_slot = var_ids_isub_dn12;
        *var_ids_isub_dn2_slot = var_ids_isub_dn2;
        *var_ids_isub_dn4_slot = var_ids_isub_dn4;
        *var_ids_isub_dn5_slot = var_ids_isub_dn5;
        *var_ids_isub_dn6_slot = var_ids_isub_dn6;
        *var_ids_isub_dn8_slot = var_ids_isub_dn8;
        *var_ids_isub_rv_slot = var_ids_isub_rv;
        *var_pds_ini_slot = var_pds_ini;
        *var_pds_ini_dn0_slot = var_pds_ini_dn0;
        *var_pds_ini_dn10_slot = var_pds_ini_dn10;
        *var_pds_ini_dn11_slot = var_pds_ini_dn11;
        *var_pds_ini_dn12_slot = var_pds_ini_dn12;
        *var_pds_ini_dn2_slot = var_pds_ini_dn2;
        *var_pds_ini_dn4_slot = var_pds_ini_dn4;
        *var_pds_ini_dn5_slot = var_pds_ini_dn5;
        *var_pds_ini_dn6_slot = var_pds_ini_dn6;
        *var_pds_ini_dn8_slot = var_pds_ini_dn8;
        *var_pds_ini_rv_slot = var_pds_ini_rv;
        *var_pds_max_slot = var_pds_max;
        *var_pds_max_dn0_slot = var_pds_max_dn0;
        *var_pds_max_dn10_slot = var_pds_max_dn10;
        *var_pds_max_dn11_slot = var_pds_max_dn11;
        *var_pds_max_dn12_slot = var_pds_max_dn12;
        *var_pds_max_dn2_slot = var_pds_max_dn2;
        *var_pds_max_dn4_slot = var_pds_max_dn4;
        *var_pds_max_dn5_slot = var_pds_max_dn5;
        *var_pds_max_dn6_slot = var_pds_max_dn6;
        *var_pds_max_dn8_slot = var_pds_max_dn8;
        *var_pds_max_rv_slot = var_pds_max_rv;
        *var_pds_qwe_slot = var_pds_qwe;
        *var_pds_qwe_dn0_slot = var_pds_qwe_dn0;
        *var_pds_qwe_dn10_slot = var_pds_qwe_dn10;
        *var_pds_qwe_dn11_slot = var_pds_qwe_dn11;
        *var_pds_qwe_dn12_slot = var_pds_qwe_dn12;
        *var_pds_qwe_dn2_slot = var_pds_qwe_dn2;
        *var_pds_qwe_dn4_slot = var_pds_qwe_dn4;
        *var_pds_qwe_dn5_slot = var_pds_qwe_dn5;
        *var_pds_qwe_dn6_slot = var_pds_qwe_dn6;
        *var_pds_qwe_dn8_slot = var_pds_qwe_dn8;
        *var_pds_qwe_rv_slot = var_pds_qwe_rv;
        *var_phib_slot = var_phib;
        *var_phib_rv_slot = var_phib_rv;
        *var_ps0_inia_slot = var_ps0_inia;
        *var_ps0_inia_dn0_slot = var_ps0_inia_dn0;
        *var_ps0_inia_dn10_slot = var_ps0_inia_dn10;
        *var_ps0_inia_dn11_slot = var_ps0_inia_dn11;
        *var_ps0_inia_dn12_slot = var_ps0_inia_dn12;
        *var_ps0_inia_dn2_slot = var_ps0_inia_dn2;
        *var_ps0_inia_dn4_slot = var_ps0_inia_dn4;
        *var_ps0_inia_dn5_slot = var_ps0_inia_dn5;
        *var_ps0_inia_dn6_slot = var_ps0_inia_dn6;
        *var_ps0_inia_dn8_slot = var_ps0_inia_dn8;
        *var_ps0_inia_rv_slot = var_ps0_inia_rv;
        *var_ps0_isub_slot = var_ps0_isub;
        *var_ps0_isub_dn0_slot = var_ps0_isub_dn0;
        *var_ps0_isub_dn10_slot = var_ps0_isub_dn10;
        *var_ps0_isub_dn11_slot = var_ps0_isub_dn11;
        *var_ps0_isub_dn12_slot = var_ps0_isub_dn12;
        *var_ps0_isub_dn2_slot = var_ps0_isub_dn2;
        *var_ps0_isub_dn4_slot = var_ps0_isub_dn4;
        *var_ps0_isub_dn5_slot = var_ps0_isub_dn5;
        *var_ps0_isub_dn6_slot = var_ps0_isub_dn6;
        *var_ps0_isub_dn8_slot = var_ps0_isub_dn8;
        *var_ps0_isub_rv_slot = var_ps0_isub_rv;
        *var_psl_lim_slot = var_psl_lim;
        *var_psl_lim_dn0_slot = var_psl_lim_dn0;
        *var_psl_lim_dn10_slot = var_psl_lim_dn10;
        *var_psl_lim_dn11_slot = var_psl_lim_dn11;
        *var_psl_lim_dn12_slot = var_psl_lim_dn12;
        *var_psl_lim_dn2_slot = var_psl_lim_dn2;
        *var_psl_lim_dn4_slot = var_psl_lim_dn4;
        *var_psl_lim_dn5_slot = var_psl_lim_dn5;
        *var_psl_lim_dn6_slot = var_psl_lim_dn6;
        *var_psl_lim_dn8_slot = var_psl_lim_dn8;
        *var_psl_lim_rv_slot = var_psl_lim_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn12_slot = var_t6_dn12;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_rv_slot = var_t6_rv;
        *var_t7_slot = var_t7;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn12_slot = var_t7_dn12;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_rv_slot = var_t7_rv;
        *var_tx_slot = var_tx;
        *var_tx_dn0_slot = var_tx_dn0;
        *var_tx_dn10_slot = var_tx_dn10;
        *var_tx_dn11_slot = var_tx_dn11;
        *var_tx_dn12_slot = var_tx_dn12;
        *var_tx_dn2_slot = var_tx_dn2;
        *var_tx_dn4_slot = var_tx_dn4;
        *var_tx_dn5_slot = var_tx_dn5;
        *var_tx_dn6_slot = var_tx_dn6;
        *var_tx_dn8_slot = var_tx_dn8;
        *var_tx_rv_slot = var_tx_rv;
    }
}
