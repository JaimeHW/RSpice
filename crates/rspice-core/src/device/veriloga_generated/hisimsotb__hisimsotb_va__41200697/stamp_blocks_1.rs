#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn4: f64,
        var_beta_inv: f64,
        var_beta_inv_dn4: f64,
        var_c_box_fd_inv: f64,
        var_c_box_inv: f64,
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
        var_cnst0soi: f64,
        var_cnst0soi_dn0: f64,
        var_cnst0soi_dn10: f64,
        var_cnst0soi_dn11: f64,
        var_cnst0soi_dn12: f64,
        var_cnst0soi_dn2: f64,
        var_cnst0soi_dn4: f64,
        var_cnst0soi_dn5: f64,
        var_cnst0soi_dn6: f64,
        var_cnst0soi_dn8: f64,
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
        var_nin: f64,
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
        var_vbi_soi: f64,
        var_vbi_soi_dn0: f64,
        var_vbi_soi_dn10: f64,
        var_vbi_soi_dn11: f64,
        var_vbi_soi_dn12: f64,
        var_vbi_soi_dn2: f64,
        var_vbi_soi_dn4: f64,
        var_vbi_soi_dn5: f64,
        var_vbi_soi_dn6: f64,
        var_vbi_soi_dn8: f64,
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
        var_guard67_slot: &mut f64,
        var_pb2_bulk_slot: &mut f64,
        var_phi_s0_bulk_0_slot: &mut f64,
        var_phi_s0_bulk_0_dn0_slot: &mut f64,
        var_phi_s0_bulk_0_dn10_slot: &mut f64,
        var_phi_s0_bulk_0_dn11_slot: &mut f64,
        var_phi_s0_bulk_0_dn12_slot: &mut f64,
        var_phi_s0_bulk_0_dn2_slot: &mut f64,
        var_phi_s0_bulk_0_dn4_slot: &mut f64,
        var_phi_s0_bulk_0_dn5_slot: &mut f64,
        var_phi_s0_bulk_0_dn6_slot: &mut f64,
        var_phi_s0_bulk_0_dn8_slot: &mut f64,
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
        var_vbsbiz_slot: &mut f64,
        var_vbsbiz_dn0_slot: &mut f64,
        var_vbsbiz_dn10_slot: &mut f64,
        var_vbsbiz_dn11_slot: &mut f64,
        var_vbsbiz_dn12_slot: &mut f64,
        var_vbsbiz_dn2_slot: &mut f64,
        var_vbsbiz_dn4_slot: &mut f64,
        var_vbsbiz_dn5_slot: &mut f64,
        var_vbsbiz_dn6_slot: &mut f64,
        var_vbsbiz_dn8_slot: &mut f64,
    ) {
        let mut var_guard67: f64 = *var_guard67_slot;
        let mut var_pb2_bulk: f64 = *var_pb2_bulk_slot;
        let mut var_phi_s0_bulk_0: f64 = *var_phi_s0_bulk_0_slot;
        let mut var_phi_s0_bulk_0_dn0: f64 = *var_phi_s0_bulk_0_dn0_slot;
        let mut var_phi_s0_bulk_0_dn10: f64 = *var_phi_s0_bulk_0_dn10_slot;
        let mut var_phi_s0_bulk_0_dn11: f64 = *var_phi_s0_bulk_0_dn11_slot;
        let mut var_phi_s0_bulk_0_dn12: f64 = *var_phi_s0_bulk_0_dn12_slot;
        let mut var_phi_s0_bulk_0_dn2: f64 = *var_phi_s0_bulk_0_dn2_slot;
        let mut var_phi_s0_bulk_0_dn4: f64 = *var_phi_s0_bulk_0_dn4_slot;
        let mut var_phi_s0_bulk_0_dn5: f64 = *var_phi_s0_bulk_0_dn5_slot;
        let mut var_phi_s0_bulk_0_dn6: f64 = *var_phi_s0_bulk_0_dn6_slot;
        let mut var_phi_s0_bulk_0_dn8: f64 = *var_phi_s0_bulk_0_dn8_slot;
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
        let mut var_vbsbiz: f64 = *var_vbsbiz_slot;
        let mut var_vbsbiz_dn0: f64 = *var_vbsbiz_dn0_slot;
        let mut var_vbsbiz_dn10: f64 = *var_vbsbiz_dn10_slot;
        let mut var_vbsbiz_dn11: f64 = *var_vbsbiz_dn11_slot;
        let mut var_vbsbiz_dn12: f64 = *var_vbsbiz_dn12_slot;
        let mut var_vbsbiz_dn2: f64 = *var_vbsbiz_dn2_slot;
        let mut var_vbsbiz_dn4: f64 = *var_vbsbiz_dn4_slot;
        let mut var_vbsbiz_dn5: f64 = *var_vbsbiz_dn5_slot;
        let mut var_vbsbiz_dn6: f64 = *var_vbsbiz_dn6_slot;
        let mut var_vbsbiz_dn8: f64 = *var_vbsbiz_dn8_slot;

        let (assign6570_e4544, assign6570_e4544_d_n0, assign6570_e4544_d_n2, assign6570_e4544_d_n4, assign6570_e4544_d_n5, assign6570_e4544_d_n6, assign6570_e4544_d_n8, assign6570_e4544_d_n10, assign6570_e4544_d_n11, assign6570_e4544_d_n12,) = {
    if (p.p29 == 0.0) {
        let assign6570_e4542: f64 = (var_vbs + var_vbi_soi);
        (assign6570_e4542, (var_vbs_dn0 + var_vbi_soi_dn0), (var_vbs_dn2 + var_vbi_soi_dn2), (var_vbs_dn4 + var_vbi_soi_dn4), (var_vbs_dn5 + var_vbi_soi_dn5), (var_vbs_dn6 + var_vbi_soi_dn6), (var_vbs_dn8 + var_vbi_soi_dn8), (var_vbs_dn10 + var_vbi_soi_dn10), (var_vbs_dn11 + var_vbi_soi_dn11), (var_vbs_dn12 + var_vbi_soi_dn12),)
    } else {
        (var_vbsbiz, var_vbsbiz_dn0, var_vbsbiz_dn2, var_vbsbiz_dn4, var_vbsbiz_dn5, var_vbsbiz_dn6, var_vbsbiz_dn8, var_vbsbiz_dn10, var_vbsbiz_dn11, var_vbsbiz_dn12,)
    }
};
        var_vbsbiz = assign6570_e4544;
        var_vbsbiz_dn0 = assign6570_e4544_d_n0;
        var_vbsbiz_dn2 = assign6570_e4544_d_n2;
        var_vbsbiz_dn4 = assign6570_e4544_d_n4;
        var_vbsbiz_dn5 = assign6570_e4544_d_n5;
        var_vbsbiz_dn6 = assign6570_e4544_d_n6;
        var_vbsbiz_dn8 = assign6570_e4544_d_n8;
        var_vbsbiz_dn10 = assign6570_e4544_d_n10;
        var_vbsbiz_dn11 = assign6570_e4544_d_n11;
        var_vbsbiz_dn12 = assign6570_e4544_d_n12;

        let assign6580_e4547: f64 = if var_vbsbiz < 0.0 { 1.0 } else { 0.0 };
        var_guard67 = assign6580_e4547;

        let (assign6590_e4553, assign6590_e4553_d_n0, assign6590_e4553_d_n2, assign6590_e4553_d_n4, assign6590_e4553_d_n5, assign6590_e4553_d_n6, assign6590_e4553_d_n8, assign6590_e4553_d_n10, assign6590_e4553_d_n11, assign6590_e4553_d_n12,) = {
    if (var_guard67 != 0.0) {
        let assign6590_e4551: f64 = (var_n_subbl / var_uc_nsubs);
        (assign6590_e4551, (((var_n_subbl_dn0 * var_uc_nsubs) - (var_n_subbl * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)), (((var_n_subbl_dn2 * var_uc_nsubs) - (var_n_subbl * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)), (((var_n_subbl_dn4 * var_uc_nsubs) - (var_n_subbl * var_uc_nsubs_dn4)) / (var_uc_nsubs * var_uc_nsubs)), (((var_n_subbl_dn5 * var_uc_nsubs) - (var_n_subbl * var_uc_nsubs_dn5)) / (var_uc_nsubs * var_uc_nsubs)), (((var_n_subbl_dn6 * var_uc_nsubs) - (var_n_subbl * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)), (((var_n_subbl_dn8 * var_uc_nsubs) - (var_n_subbl * var_uc_nsubs_dn8)) / (var_uc_nsubs * var_uc_nsubs)), (((var_n_subbl_dn10 * var_uc_nsubs) - (var_n_subbl * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)), (((var_n_subbl_dn11 * var_uc_nsubs) - (var_n_subbl * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)), (((var_n_subbl_dn12 * var_uc_nsubs) - (var_n_subbl * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
        var_t0 = assign6590_e4553;
        var_t0_dn0 = assign6590_e4553_d_n0;
        var_t0_dn2 = assign6590_e4553_d_n2;
        var_t0_dn4 = assign6590_e4553_d_n4;
        var_t0_dn5 = assign6590_e4553_d_n5;
        var_t0_dn6 = assign6590_e4553_d_n6;
        var_t0_dn8 = assign6590_e4553_d_n8;
        var_t0_dn10 = assign6590_e4553_d_n10;
        var_t0_dn11 = assign6590_e4553_d_n11;
        var_t0_dn12 = assign6590_e4553_d_n12;

        let (assign6600_e4559, assign6600_e4559_d_n0, assign6600_e4559_d_n2, assign6600_e4559_d_n4, assign6600_e4559_d_n5, assign6600_e4559_d_n6, assign6600_e4559_d_n8, assign6600_e4559_d_n10, assign6600_e4559_d_n11, assign6600_e4559_d_n12,) = {
    if (var_guard67 != 0.0) {
        let assign6600_e4557: f64 = (var_t0 + 1.0);
        (assign6600_e4557, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign6600_e4559;
        var_t1_dn0 = assign6600_e4559_d_n0;
        var_t1_dn2 = assign6600_e4559_d_n2;
        var_t1_dn4 = assign6600_e4559_d_n4;
        var_t1_dn5 = assign6600_e4559_d_n5;
        var_t1_dn6 = assign6600_e4559_d_n6;
        var_t1_dn8 = assign6600_e4559_d_n8;
        var_t1_dn10 = assign6600_e4559_d_n10;
        var_t1_dn11 = assign6600_e4559_d_n11;
        var_t1_dn12 = assign6600_e4559_d_n12;

        let (assign6610_e4571, assign6610_e4571_d_n0, assign6610_e4571_d_n2, assign6610_e4571_d_n4, assign6610_e4571_d_n5, assign6610_e4571_d_n6, assign6610_e4571_d_n8, assign6610_e4571_d_n10, assign6610_e4571_d_n11, assign6610_e4571_d_n12,) = {
    if (var_guard67 != 0.0) {
        let assign6610_e4563: f64 = (var_beta_inv - var_vbsbiz);
        let assign6610_e4567: f64 = (var_beta_inv + var_vbsbiz);
        let assign6610_e4568: f64 = (var_t0 * assign6610_e4567);
        let assign6610_e4569: f64 = (assign6610_e4563 + assign6610_e4568);
        (assign6610_e4569, ((-var_vbsbiz_dn0) + ((var_t0_dn0 * assign6610_e4567) + (var_t0 * var_vbsbiz_dn0))), ((-var_vbsbiz_dn2) + ((var_t0_dn2 * assign6610_e4567) + (var_t0 * var_vbsbiz_dn2))), ((var_beta_inv_dn4 - var_vbsbiz_dn4) + ((var_t0_dn4 * assign6610_e4567) + (var_t0 * (var_beta_inv_dn4 + var_vbsbiz_dn4)))), ((-var_vbsbiz_dn5) + ((var_t0_dn5 * assign6610_e4567) + (var_t0 * var_vbsbiz_dn5))), ((-var_vbsbiz_dn6) + ((var_t0_dn6 * assign6610_e4567) + (var_t0 * var_vbsbiz_dn6))), ((-var_vbsbiz_dn8) + ((var_t0_dn8 * assign6610_e4567) + (var_t0 * var_vbsbiz_dn8))), ((-var_vbsbiz_dn10) + ((var_t0_dn10 * assign6610_e4567) + (var_t0 * var_vbsbiz_dn10))), ((-var_vbsbiz_dn11) + ((var_t0_dn11 * assign6610_e4567) + (var_t0 * var_vbsbiz_dn11))), ((-var_vbsbiz_dn12) + ((var_t0_dn12 * assign6610_e4567) + (var_t0 * var_vbsbiz_dn12))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign6610_e4571;
        var_t2_dn0 = assign6610_e4571_d_n0;
        var_t2_dn2 = assign6610_e4571_d_n2;
        var_t2_dn4 = assign6610_e4571_d_n4;
        var_t2_dn5 = assign6610_e4571_d_n5;
        var_t2_dn6 = assign6610_e4571_d_n6;
        var_t2_dn8 = assign6610_e4571_d_n8;
        var_t2_dn10 = assign6610_e4571_d_n10;
        var_t2_dn11 = assign6610_e4571_d_n11;
        var_t2_dn12 = assign6610_e4571_d_n12;

        let (assign6620_e4581, assign6620_e4581_d_n0, assign6620_e4581_d_n2, assign6620_e4581_d_n4, assign6620_e4581_d_n5, assign6620_e4581_d_n6, assign6620_e4581_d_n8, assign6620_e4581_d_n10, assign6620_e4581_d_n11, assign6620_e4581_d_n12,) = {
    if (var_guard67 != 0.0) {
        let assign6620_e4575: f64 = (var_cnst0bulk * var_cnst0bulk);
        let assign6620_e4577: f64 = (assign6620_e4575 * var_c_box_inv);
        let assign6620_e4579: f64 = (assign6620_e4577 * var_c_box_inv);
        (assign6620_e4579, ((((var_cnst0bulk_dn0 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn0)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0bulk_dn2 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn2)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0bulk_dn4 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn4)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0bulk_dn5 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn5)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0bulk_dn6 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn6)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0bulk_dn8 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn8)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0bulk_dn10 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn10)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0bulk_dn11 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn11)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0bulk_dn12 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn12)) * var_c_box_inv) * var_c_box_inv),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign6620_e4581;
        var_t3_dn0 = assign6620_e4581_d_n0;
        var_t3_dn2 = assign6620_e4581_d_n2;
        var_t3_dn4 = assign6620_e4581_d_n4;
        var_t3_dn5 = assign6620_e4581_d_n5;
        var_t3_dn6 = assign6620_e4581_d_n6;
        var_t3_dn8 = assign6620_e4581_d_n8;
        var_t3_dn10 = assign6620_e4581_d_n10;
        var_t3_dn11 = assign6620_e4581_d_n11;
        var_t3_dn12 = assign6620_e4581_d_n12;

        let (assign6630_e4593, assign6630_e4593_d_n0, assign6630_e4593_d_n2, assign6630_e4593_d_n4, assign6630_e4593_d_n5, assign6630_e4593_d_n6, assign6630_e4593_d_n8, assign6630_e4593_d_n10, assign6630_e4593_d_n11, assign6630_e4593_d_n12,) = {
    if (var_guard67 != 0.0) {
        let assign6630_e4585: f64 = (2.0 * var_t2);
        let assign6630_e4587: f64 = (assign6630_e4585 * var_t1);
        let assign6630_e4590: f64 = (var_t3 * var_beta);
        let assign6630_e4591: f64 = (assign6630_e4587 - assign6630_e4590);
        (assign6630_e4591, ((((2.0 * var_t2_dn0) * var_t1) + (assign6630_e4585 * var_t1_dn0)) - (var_t3_dn0 * var_beta)), ((((2.0 * var_t2_dn2) * var_t1) + (assign6630_e4585 * var_t1_dn2)) - (var_t3_dn2 * var_beta)), ((((2.0 * var_t2_dn4) * var_t1) + (assign6630_e4585 * var_t1_dn4)) - ((var_t3_dn4 * var_beta) + (var_t3 * var_beta_dn4))), ((((2.0 * var_t2_dn5) * var_t1) + (assign6630_e4585 * var_t1_dn5)) - (var_t3_dn5 * var_beta)), ((((2.0 * var_t2_dn6) * var_t1) + (assign6630_e4585 * var_t1_dn6)) - (var_t3_dn6 * var_beta)), ((((2.0 * var_t2_dn8) * var_t1) + (assign6630_e4585 * var_t1_dn8)) - (var_t3_dn8 * var_beta)), ((((2.0 * var_t2_dn10) * var_t1) + (assign6630_e4585 * var_t1_dn10)) - (var_t3_dn10 * var_beta)), ((((2.0 * var_t2_dn11) * var_t1) + (assign6630_e4585 * var_t1_dn11)) - (var_t3_dn11 * var_beta)), ((((2.0 * var_t2_dn12) * var_t1) + (assign6630_e4585 * var_t1_dn12)) - (var_t3_dn12 * var_beta)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
        var_t4 = assign6630_e4593;
        var_t4_dn0 = assign6630_e4593_d_n0;
        var_t4_dn2 = assign6630_e4593_d_n2;
        var_t4_dn4 = assign6630_e4593_d_n4;
        var_t4_dn5 = assign6630_e4593_d_n5;
        var_t4_dn6 = assign6630_e4593_d_n6;
        var_t4_dn8 = assign6630_e4593_d_n8;
        var_t4_dn10 = assign6630_e4593_d_n10;
        var_t4_dn11 = assign6630_e4593_d_n11;
        var_t4_dn12 = assign6630_e4593_d_n12;

        let (assign6640_e4607, assign6640_e4607_d_n0, assign6640_e4607_d_n2, assign6640_e4607_d_n4, assign6640_e4607_d_n5, assign6640_e4607_d_n6, assign6640_e4607_d_n8, assign6640_e4607_d_n10, assign6640_e4607_d_n11, assign6640_e4607_d_n12,) = {
    if (var_guard67 != 0.0) {
        let assign6640_e4597: f64 = (var_t2 * var_t2);
        let assign6640_e4600: f64 = (var_t3 * var_beta);
        let assign6640_e4602: f64 = (assign6640_e4600 * var_vbsbiz);
        let assign6640_e4603: f64 = (assign6640_e4597 + assign6640_e4602);
        let assign6640_e4605: f64 = (assign6640_e4603 + var_t3);
        (assign6640_e4605, ((((var_t2_dn0 * var_t2) + (var_t2 * var_t2_dn0)) + (((var_t3_dn0 * var_beta) * var_vbsbiz) + (assign6640_e4600 * var_vbsbiz_dn0))) + var_t3_dn0), ((((var_t2_dn2 * var_t2) + (var_t2 * var_t2_dn2)) + (((var_t3_dn2 * var_beta) * var_vbsbiz) + (assign6640_e4600 * var_vbsbiz_dn2))) + var_t3_dn2), ((((var_t2_dn4 * var_t2) + (var_t2 * var_t2_dn4)) + ((((var_t3_dn4 * var_beta) + (var_t3 * var_beta_dn4)) * var_vbsbiz) + (assign6640_e4600 * var_vbsbiz_dn4))) + var_t3_dn4), ((((var_t2_dn5 * var_t2) + (var_t2 * var_t2_dn5)) + (((var_t3_dn5 * var_beta) * var_vbsbiz) + (assign6640_e4600 * var_vbsbiz_dn5))) + var_t3_dn5), ((((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6)) + (((var_t3_dn6 * var_beta) * var_vbsbiz) + (assign6640_e4600 * var_vbsbiz_dn6))) + var_t3_dn6), ((((var_t2_dn8 * var_t2) + (var_t2 * var_t2_dn8)) + (((var_t3_dn8 * var_beta) * var_vbsbiz) + (assign6640_e4600 * var_vbsbiz_dn8))) + var_t3_dn8), ((((var_t2_dn10 * var_t2) + (var_t2 * var_t2_dn10)) + (((var_t3_dn10 * var_beta) * var_vbsbiz) + (assign6640_e4600 * var_vbsbiz_dn10))) + var_t3_dn10), ((((var_t2_dn11 * var_t2) + (var_t2 * var_t2_dn11)) + (((var_t3_dn11 * var_beta) * var_vbsbiz) + (assign6640_e4600 * var_vbsbiz_dn11))) + var_t3_dn11), ((((var_t2_dn12 * var_t2) + (var_t2 * var_t2_dn12)) + (((var_t3_dn12 * var_beta) * var_vbsbiz) + (assign6640_e4600 * var_vbsbiz_dn12))) + var_t3_dn12),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
        var_t5 = assign6640_e4607;
        var_t5_dn0 = assign6640_e4607_d_n0;
        var_t5_dn2 = assign6640_e4607_d_n2;
        var_t5_dn4 = assign6640_e4607_d_n4;
        var_t5_dn5 = assign6640_e4607_d_n5;
        var_t5_dn6 = assign6640_e4607_d_n6;
        var_t5_dn8 = assign6640_e4607_d_n8;
        var_t5_dn10 = assign6640_e4607_d_n10;
        var_t5_dn11 = assign6640_e4607_d_n11;
        var_t5_dn12 = assign6640_e4607_d_n12;

        let (assign6650_e4636, assign6650_e4636_d_n0, assign6650_e4636_d_n2, assign6650_e4636_d_n4, assign6650_e4636_d_n5, assign6650_e4636_d_n6, assign6650_e4636_d_n8, assign6650_e4636_d_n10, assign6650_e4636_d_n11, assign6650_e4636_d_n12,) = {
    if (var_guard67 != 0.0) {
        let assign6650_e4611: f64 = (var_t4 * var_t4);
        let assign6650_e4614: f64 = (4.0 * var_t1);
        let assign6650_e4616: f64 = (assign6650_e4614 * var_t1);
        let assign6650_e4618: f64 = (assign6650_e4616 * var_t5);
        let assign6650_e4619: f64 = (assign6650_e4611 - assign6650_e4618);
        let (assign6650_e4634, assign6650_e4634_d_n0, assign6650_e4634_d_n2, assign6650_e4634_d_n4, assign6650_e4634_d_n5, assign6650_e4634_d_n6, assign6650_e4634_d_n8, assign6650_e4634_d_n10, assign6650_e4634_d_n11, assign6650_e4634_d_n12,) = {
            if (assign6650_e4619 >= 1e-50) {
                let assign6650_e4624: f64 = (var_t4 * var_t4);
                let assign6650_e4627: f64 = (4.0 * var_t1);
                let assign6650_e4629: f64 = (assign6650_e4627 * var_t1);
                let assign6650_e4631: f64 = (assign6650_e4629 * var_t5);
                let assign6650_e4632: f64 = (assign6650_e4624 - assign6650_e4631);
                (assign6650_e4632, (((var_t4_dn0 * var_t4) + (var_t4 * var_t4_dn0)) - (((((4.0 * var_t1_dn0) * var_t1) + (assign6650_e4627 * var_t1_dn0)) * var_t5) + (assign6650_e4629 * var_t5_dn0))), (((var_t4_dn2 * var_t4) + (var_t4 * var_t4_dn2)) - (((((4.0 * var_t1_dn2) * var_t1) + (assign6650_e4627 * var_t1_dn2)) * var_t5) + (assign6650_e4629 * var_t5_dn2))), (((var_t4_dn4 * var_t4) + (var_t4 * var_t4_dn4)) - (((((4.0 * var_t1_dn4) * var_t1) + (assign6650_e4627 * var_t1_dn4)) * var_t5) + (assign6650_e4629 * var_t5_dn4))), (((var_t4_dn5 * var_t4) + (var_t4 * var_t4_dn5)) - (((((4.0 * var_t1_dn5) * var_t1) + (assign6650_e4627 * var_t1_dn5)) * var_t5) + (assign6650_e4629 * var_t5_dn5))), (((var_t4_dn6 * var_t4) + (var_t4 * var_t4_dn6)) - (((((4.0 * var_t1_dn6) * var_t1) + (assign6650_e4627 * var_t1_dn6)) * var_t5) + (assign6650_e4629 * var_t5_dn6))), (((var_t4_dn8 * var_t4) + (var_t4 * var_t4_dn8)) - (((((4.0 * var_t1_dn8) * var_t1) + (assign6650_e4627 * var_t1_dn8)) * var_t5) + (assign6650_e4629 * var_t5_dn8))), (((var_t4_dn10 * var_t4) + (var_t4 * var_t4_dn10)) - (((((4.0 * var_t1_dn10) * var_t1) + (assign6650_e4627 * var_t1_dn10)) * var_t5) + (assign6650_e4629 * var_t5_dn10))), (((var_t4_dn11 * var_t4) + (var_t4 * var_t4_dn11)) - (((((4.0 * var_t1_dn11) * var_t1) + (assign6650_e4627 * var_t1_dn11)) * var_t5) + (assign6650_e4629 * var_t5_dn11))), (((var_t4_dn12 * var_t4) + (var_t4 * var_t4_dn12)) - (((((4.0 * var_t1_dn12) * var_t1) + (assign6650_e4627 * var_t1_dn12)) * var_t5) + (assign6650_e4629 * var_t5_dn12))),)
            } else {
                (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign6650_e4634, assign6650_e4634_d_n0, assign6650_e4634_d_n2, assign6650_e4634_d_n4, assign6650_e4634_d_n5, assign6650_e4634_d_n6, assign6650_e4634_d_n8, assign6650_e4634_d_n10, assign6650_e4634_d_n11, assign6650_e4634_d_n12,)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
        var_t7 = assign6650_e4636;
        var_t7_dn0 = assign6650_e4636_d_n0;
        var_t7_dn2 = assign6650_e4636_d_n2;
        var_t7_dn4 = assign6650_e4636_d_n4;
        var_t7_dn5 = assign6650_e4636_d_n5;
        var_t7_dn6 = assign6650_e4636_d_n6;
        var_t7_dn8 = assign6650_e4636_d_n8;
        var_t7_dn10 = assign6650_e4636_d_n10;
        var_t7_dn11 = assign6650_e4636_d_n11;
        var_t7_dn12 = assign6650_e4636_d_n12;

        let (assign6660_e4649, assign6660_e4649_d_n0, assign6660_e4649_d_n2, assign6660_e4649_d_n4, assign6660_e4649_d_n5, assign6660_e4649_d_n6, assign6660_e4649_d_n8, assign6660_e4649_d_n10, assign6660_e4649_d_n11, assign6660_e4649_d_n12,) = {
    if (var_guard67 != 0.0) {
        let assign6660_e4640: f64 = (var_t7).sqrt();
        let assign6660_e4641: f64 = (var_t4 + assign6660_e4640);
        let assign6660_e4645: f64 = (var_t1 * var_t1);
        let assign6660_e4646: f64 = (2.0 + assign6660_e4645);
        let assign6660_e4647: f64 = (assign6660_e4641 / assign6660_e4646);
        (assign6660_e4647, ((((var_t4_dn0 + (var_t7_dn0 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)))) / (assign6660_e4646 * assign6660_e4646)), ((((var_t4_dn2 + (var_t7_dn2 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)))) / (assign6660_e4646 * assign6660_e4646)), ((((var_t4_dn4 + (var_t7_dn4 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)))) / (assign6660_e4646 * assign6660_e4646)), ((((var_t4_dn5 + (var_t7_dn5 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)))) / (assign6660_e4646 * assign6660_e4646)), ((((var_t4_dn6 + (var_t7_dn6 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)))) / (assign6660_e4646 * assign6660_e4646)), ((((var_t4_dn8 + (var_t7_dn8 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)))) / (assign6660_e4646 * assign6660_e4646)), ((((var_t4_dn10 + (var_t7_dn10 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)))) / (assign6660_e4646 * assign6660_e4646)), ((((var_t4_dn11 + (var_t7_dn11 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)))) / (assign6660_e4646 * assign6660_e4646)), ((((var_t4_dn12 + (var_t7_dn12 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)))) / (assign6660_e4646 * assign6660_e4646)),)
    } else {
        (var_phi_s0_bulk_0, var_phi_s0_bulk_0_dn0, var_phi_s0_bulk_0_dn2, var_phi_s0_bulk_0_dn4, var_phi_s0_bulk_0_dn5, var_phi_s0_bulk_0_dn6, var_phi_s0_bulk_0_dn8, var_phi_s0_bulk_0_dn10, var_phi_s0_bulk_0_dn11, var_phi_s0_bulk_0_dn12,)
    }
};
        var_phi_s0_bulk_0 = assign6660_e4649;
        var_phi_s0_bulk_0_dn0 = assign6660_e4649_d_n0;
        var_phi_s0_bulk_0_dn2 = assign6660_e4649_d_n2;
        var_phi_s0_bulk_0_dn4 = assign6660_e4649_d_n4;
        var_phi_s0_bulk_0_dn5 = assign6660_e4649_d_n5;
        var_phi_s0_bulk_0_dn6 = assign6660_e4649_d_n6;
        var_phi_s0_bulk_0_dn8 = assign6660_e4649_d_n8;
        var_phi_s0_bulk_0_dn10 = assign6660_e4649_d_n10;
        var_phi_s0_bulk_0_dn11 = assign6660_e4649_d_n11;
        var_phi_s0_bulk_0_dn12 = assign6660_e4649_d_n12;

        let (assign6670_e4658, assign6670_e4658_d_n0, assign6670_e4658_d_n2, assign6670_e4658_d_n4, assign6670_e4658_d_n5, assign6670_e4658_d_n6, assign6670_e4658_d_n8, assign6670_e4658_d_n10, assign6670_e4658_d_n11, assign6670_e4658_d_n12,) = {
    if (var_guard67 == 0.0) {
        let assign6670_e4654: f64 = (var_cnst0bulk * var_cnst0bulk);
        let assign6670_e4656: f64 = (assign6670_e4654 * var_beta);
        (assign6670_e4656, (((var_cnst0bulk_dn0 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn0)) * var_beta), (((var_cnst0bulk_dn2 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn2)) * var_beta), ((((var_cnst0bulk_dn4 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn4)) * var_beta) + (assign6670_e4654 * var_beta_dn4)), (((var_cnst0bulk_dn5 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn5)) * var_beta), (((var_cnst0bulk_dn6 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn6)) * var_beta), (((var_cnst0bulk_dn8 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn8)) * var_beta), (((var_cnst0bulk_dn10 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn10)) * var_beta), (((var_cnst0bulk_dn11 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn11)) * var_beta), (((var_cnst0bulk_dn12 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn12)) * var_beta),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign6670_e4658;
        var_t1_dn0 = assign6670_e4658_d_n0;
        var_t1_dn2 = assign6670_e4658_d_n2;
        var_t1_dn4 = assign6670_e4658_d_n4;
        var_t1_dn5 = assign6670_e4658_d_n5;
        var_t1_dn6 = assign6670_e4658_d_n6;
        var_t1_dn8 = assign6670_e4658_d_n8;
        var_t1_dn10 = assign6670_e4658_d_n10;
        var_t1_dn11 = assign6670_e4658_d_n11;
        var_t1_dn12 = assign6670_e4658_d_n12;

        let (assign6680_e4667, assign6680_e4667_d_n0, assign6680_e4667_d_n2, assign6680_e4667_d_n4, assign6680_e4667_d_n5, assign6680_e4667_d_n6, assign6680_e4667_d_n8, assign6680_e4667_d_n10, assign6680_e4667_d_n11, assign6680_e4667_d_n12,) = {
    if (var_guard67 == 0.0) {
        let assign6680_e4663: f64 = (var_cnst0soi * var_cnst0soi);
        let assign6680_e4665: f64 = (assign6680_e4663 * var_beta);
        (assign6680_e4665, (((var_cnst0soi_dn0 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn0)) * var_beta), (((var_cnst0soi_dn2 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn2)) * var_beta), ((((var_cnst0soi_dn4 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn4)) * var_beta) + (assign6680_e4663 * var_beta_dn4)), (((var_cnst0soi_dn5 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn5)) * var_beta), (((var_cnst0soi_dn6 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn6)) * var_beta), (((var_cnst0soi_dn8 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn8)) * var_beta), (((var_cnst0soi_dn10 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn10)) * var_beta), (((var_cnst0soi_dn11 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn11)) * var_beta), (((var_cnst0soi_dn12 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn12)) * var_beta),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign6680_e4667;
        var_t2_dn0 = assign6680_e4667_d_n0;
        var_t2_dn2 = assign6680_e4667_d_n2;
        var_t2_dn4 = assign6680_e4667_d_n4;
        var_t2_dn5 = assign6680_e4667_d_n5;
        var_t2_dn6 = assign6680_e4667_d_n6;
        var_t2_dn8 = assign6680_e4667_d_n8;
        var_t2_dn10 = assign6680_e4667_d_n10;
        var_t2_dn11 = assign6680_e4667_d_n11;
        var_t2_dn12 = assign6680_e4667_d_n12;

        let (assign6690_e4677, assign6690_e4677_d_n0, assign6690_e4677_d_n2, assign6690_e4677_d_n4, assign6690_e4677_d_n5, assign6690_e4677_d_n6, assign6690_e4677_d_n8, assign6690_e4677_d_n10, assign6690_e4677_d_n11, assign6690_e4677_d_n12,) = {
    if (var_guard67 == 0.0) {
        let assign6690_e4673: f64 = (2.0 * var_vbsbiz);
        let assign6690_e4674: f64 = (var_beta_inv + assign6690_e4673);
        let assign6690_e4675: f64 = (-assign6690_e4674);
        (assign6690_e4675, (-(2.0 * var_vbsbiz_dn0)), (-(2.0 * var_vbsbiz_dn2)), (-(var_beta_inv_dn4 + (2.0 * var_vbsbiz_dn4))), (-(2.0 * var_vbsbiz_dn5)), (-(2.0 * var_vbsbiz_dn6)), (-(2.0 * var_vbsbiz_dn8)), (-(2.0 * var_vbsbiz_dn10)), (-(2.0 * var_vbsbiz_dn11)), (-(2.0 * var_vbsbiz_dn12)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign6690_e4677;
        var_t3_dn0 = assign6690_e4677_d_n0;
        var_t3_dn2 = assign6690_e4677_d_n2;
        var_t3_dn4 = assign6690_e4677_d_n4;
        var_t3_dn5 = assign6690_e4677_d_n5;
        var_t3_dn6 = assign6690_e4677_d_n6;
        var_t3_dn8 = assign6690_e4677_d_n8;
        var_t3_dn10 = assign6690_e4677_d_n10;
        var_t3_dn11 = assign6690_e4677_d_n11;
        var_t3_dn12 = assign6690_e4677_d_n12;

        let (assign6700_e4686, assign6700_e4686_d_n0, assign6700_e4686_d_n2, assign6700_e4686_d_n4, assign6700_e4686_d_n5, assign6700_e4686_d_n6, assign6700_e4686_d_n8, assign6700_e4686_d_n10, assign6700_e4686_d_n11, assign6700_e4686_d_n12,) = {
    if (var_guard67 == 0.0) {
        let assign6700_e4683: f64 = (var_t2 / var_t1);
        let assign6700_e4684: f64 = (1.0 + assign6700_e4683);
        (assign6700_e4684, (((var_t2_dn0 * var_t1) - (var_t2 * var_t1_dn0)) / (var_t1 * var_t1)), (((var_t2_dn2 * var_t1) - (var_t2 * var_t1_dn2)) / (var_t1 * var_t1)), (((var_t2_dn4 * var_t1) - (var_t2 * var_t1_dn4)) / (var_t1 * var_t1)), (((var_t2_dn5 * var_t1) - (var_t2 * var_t1_dn5)) / (var_t1 * var_t1)), (((var_t2_dn6 * var_t1) - (var_t2 * var_t1_dn6)) / (var_t1 * var_t1)), (((var_t2_dn8 * var_t1) - (var_t2 * var_t1_dn8)) / (var_t1 * var_t1)), (((var_t2_dn10 * var_t1) - (var_t2 * var_t1_dn10)) / (var_t1 * var_t1)), (((var_t2_dn11 * var_t1) - (var_t2 * var_t1_dn11)) / (var_t1 * var_t1)), (((var_t2_dn12 * var_t1) - (var_t2 * var_t1_dn12)) / (var_t1 * var_t1)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
        var_t4 = assign6700_e4686;
        var_t4_dn0 = assign6700_e4686_d_n0;
        var_t4_dn2 = assign6700_e4686_d_n2;
        var_t4_dn4 = assign6700_e4686_d_n4;
        var_t4_dn5 = assign6700_e4686_d_n5;
        var_t4_dn6 = assign6700_e4686_d_n6;
        var_t4_dn8 = assign6700_e4686_d_n8;
        var_t4_dn10 = assign6700_e4686_d_n10;
        var_t4_dn11 = assign6700_e4686_d_n11;
        var_t4_dn12 = assign6700_e4686_d_n12;

        let (assign6710_e4697, assign6710_e4697_d_n0, assign6710_e4697_d_n2, assign6710_e4697_d_n4, assign6710_e4697_d_n5, assign6710_e4697_d_n6, assign6710_e4697_d_n8, assign6710_e4697_d_n10, assign6710_e4697_d_n11, assign6710_e4697_d_n12,) = {
    if (var_guard67 == 0.0) {
        let assign6710_e4691: f64 = (var_cnst0soi * var_cnst0soi);
        let assign6710_e4693: f64 = (assign6710_e4691 * var_c_box_inv);
        let assign6710_e4695: f64 = (assign6710_e4693 * var_c_box_inv);
        (assign6710_e4695, ((((var_cnst0soi_dn0 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn0)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0soi_dn2 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn2)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0soi_dn4 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn4)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0soi_dn5 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn5)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0soi_dn6 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn6)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0soi_dn8 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn8)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0soi_dn10 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn10)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0soi_dn11 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn11)) * var_c_box_inv) * var_c_box_inv), ((((var_cnst0soi_dn12 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn12)) * var_c_box_inv) * var_c_box_inv),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
        var_t5 = assign6710_e4697;
        var_t5_dn0 = assign6710_e4697_d_n0;
        var_t5_dn2 = assign6710_e4697_d_n2;
        var_t5_dn4 = assign6710_e4697_d_n4;
        var_t5_dn5 = assign6710_e4697_d_n5;
        var_t5_dn6 = assign6710_e4697_d_n6;
        var_t5_dn8 = assign6710_e4697_d_n8;
        var_t5_dn10 = assign6710_e4697_d_n10;
        var_t5_dn11 = assign6710_e4697_d_n11;
        var_t5_dn12 = assign6710_e4697_d_n12;

        let (assign6720_e4710, assign6720_e4710_d_n0, assign6720_e4710_d_n2, assign6720_e4710_d_n4, assign6720_e4710_d_n5, assign6720_e4710_d_n6, assign6720_e4710_d_n8, assign6720_e4710_d_n10, assign6720_e4710_d_n11, assign6720_e4710_d_n12,) = {
    if (var_guard67 == 0.0) {
        let assign6720_e4702: f64 = (var_t5 * var_beta);
        let assign6720_e4705: f64 = (2.0 * var_t3);
        let assign6720_e4707: f64 = (assign6720_e4705 * var_t4);
        let assign6720_e4708: f64 = (assign6720_e4702 - assign6720_e4707);
        (assign6720_e4708, ((var_t5_dn0 * var_beta) - (((2.0 * var_t3_dn0) * var_t4) + (assign6720_e4705 * var_t4_dn0))), ((var_t5_dn2 * var_beta) - (((2.0 * var_t3_dn2) * var_t4) + (assign6720_e4705 * var_t4_dn2))), (((var_t5_dn4 * var_beta) + (var_t5 * var_beta_dn4)) - (((2.0 * var_t3_dn4) * var_t4) + (assign6720_e4705 * var_t4_dn4))), ((var_t5_dn5 * var_beta) - (((2.0 * var_t3_dn5) * var_t4) + (assign6720_e4705 * var_t4_dn5))), ((var_t5_dn6 * var_beta) - (((2.0 * var_t3_dn6) * var_t4) + (assign6720_e4705 * var_t4_dn6))), ((var_t5_dn8 * var_beta) - (((2.0 * var_t3_dn8) * var_t4) + (assign6720_e4705 * var_t4_dn8))), ((var_t5_dn10 * var_beta) - (((2.0 * var_t3_dn10) * var_t4) + (assign6720_e4705 * var_t4_dn10))), ((var_t5_dn11 * var_beta) - (((2.0 * var_t3_dn11) * var_t4) + (assign6720_e4705 * var_t4_dn11))), ((var_t5_dn12 * var_beta) - (((2.0 * var_t3_dn12) * var_t4) + (assign6720_e4705 * var_t4_dn12))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
        var_t6 = assign6720_e4710;
        var_t6_dn0 = assign6720_e4710_d_n0;
        var_t6_dn2 = assign6720_e4710_d_n2;
        var_t6_dn4 = assign6720_e4710_d_n4;
        var_t6_dn5 = assign6720_e4710_d_n5;
        var_t6_dn6 = assign6720_e4710_d_n6;
        var_t6_dn8 = assign6720_e4710_d_n8;
        var_t6_dn10 = assign6720_e4710_d_n10;
        var_t6_dn11 = assign6720_e4710_d_n11;
        var_t6_dn12 = assign6720_e4710_d_n12;

        let (assign6730_e4744, assign6730_e4744_d_n0, assign6730_e4744_d_n2, assign6730_e4744_d_n4, assign6730_e4744_d_n5, assign6730_e4744_d_n6, assign6730_e4744_d_n8, assign6730_e4744_d_n10, assign6730_e4744_d_n11, assign6730_e4744_d_n12,) = {
    if (var_guard67 == 0.0) {
        let assign6730_e4715: f64 = (var_t6 * var_t6);
        let assign6730_e4718: f64 = (4.0 * var_t4);
        let assign6730_e4720: f64 = (assign6730_e4718 * var_t4);
        let assign6730_e4722: f64 = (assign6730_e4720 * var_t3);
        let assign6730_e4724: f64 = (assign6730_e4722 * var_t3);
        let assign6730_e4725: f64 = (assign6730_e4715 - assign6730_e4724);
        let (assign6730_e4742, assign6730_e4742_d_n0, assign6730_e4742_d_n2, assign6730_e4742_d_n4, assign6730_e4742_d_n5, assign6730_e4742_d_n6, assign6730_e4742_d_n8, assign6730_e4742_d_n10, assign6730_e4742_d_n11, assign6730_e4742_d_n12,) = {
            if (assign6730_e4725 >= 1e-50) {
                let assign6730_e4730: f64 = (var_t6 * var_t6);
                let assign6730_e4733: f64 = (4.0 * var_t4);
                let assign6730_e4735: f64 = (assign6730_e4733 * var_t4);
                let assign6730_e4737: f64 = (assign6730_e4735 * var_t3);
                let assign6730_e4739: f64 = (assign6730_e4737 * var_t3);
                let assign6730_e4740: f64 = (assign6730_e4730 - assign6730_e4739);
                (assign6730_e4740, (((var_t6_dn0 * var_t6) + (var_t6 * var_t6_dn0)) - (((((((4.0 * var_t4_dn0) * var_t4) + (assign6730_e4733 * var_t4_dn0)) * var_t3) + (assign6730_e4735 * var_t3_dn0)) * var_t3) + (assign6730_e4737 * var_t3_dn0))), (((var_t6_dn2 * var_t6) + (var_t6 * var_t6_dn2)) - (((((((4.0 * var_t4_dn2) * var_t4) + (assign6730_e4733 * var_t4_dn2)) * var_t3) + (assign6730_e4735 * var_t3_dn2)) * var_t3) + (assign6730_e4737 * var_t3_dn2))), (((var_t6_dn4 * var_t6) + (var_t6 * var_t6_dn4)) - (((((((4.0 * var_t4_dn4) * var_t4) + (assign6730_e4733 * var_t4_dn4)) * var_t3) + (assign6730_e4735 * var_t3_dn4)) * var_t3) + (assign6730_e4737 * var_t3_dn4))), (((var_t6_dn5 * var_t6) + (var_t6 * var_t6_dn5)) - (((((((4.0 * var_t4_dn5) * var_t4) + (assign6730_e4733 * var_t4_dn5)) * var_t3) + (assign6730_e4735 * var_t3_dn5)) * var_t3) + (assign6730_e4737 * var_t3_dn5))), (((var_t6_dn6 * var_t6) + (var_t6 * var_t6_dn6)) - (((((((4.0 * var_t4_dn6) * var_t4) + (assign6730_e4733 * var_t4_dn6)) * var_t3) + (assign6730_e4735 * var_t3_dn6)) * var_t3) + (assign6730_e4737 * var_t3_dn6))), (((var_t6_dn8 * var_t6) + (var_t6 * var_t6_dn8)) - (((((((4.0 * var_t4_dn8) * var_t4) + (assign6730_e4733 * var_t4_dn8)) * var_t3) + (assign6730_e4735 * var_t3_dn8)) * var_t3) + (assign6730_e4737 * var_t3_dn8))), (((var_t6_dn10 * var_t6) + (var_t6 * var_t6_dn10)) - (((((((4.0 * var_t4_dn10) * var_t4) + (assign6730_e4733 * var_t4_dn10)) * var_t3) + (assign6730_e4735 * var_t3_dn10)) * var_t3) + (assign6730_e4737 * var_t3_dn10))), (((var_t6_dn11 * var_t6) + (var_t6 * var_t6_dn11)) - (((((((4.0 * var_t4_dn11) * var_t4) + (assign6730_e4733 * var_t4_dn11)) * var_t3) + (assign6730_e4735 * var_t3_dn11)) * var_t3) + (assign6730_e4737 * var_t3_dn11))), (((var_t6_dn12 * var_t6) + (var_t6 * var_t6_dn12)) - (((((((4.0 * var_t4_dn12) * var_t4) + (assign6730_e4733 * var_t4_dn12)) * var_t3) + (assign6730_e4735 * var_t3_dn12)) * var_t3) + (assign6730_e4737 * var_t3_dn12))),)
            } else {
                (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign6730_e4742, assign6730_e4742_d_n0, assign6730_e4742_d_n2, assign6730_e4742_d_n4, assign6730_e4742_d_n5, assign6730_e4742_d_n6, assign6730_e4742_d_n8, assign6730_e4742_d_n10, assign6730_e4742_d_n11, assign6730_e4742_d_n12,)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
        var_t7 = assign6730_e4744;
        var_t7_dn0 = assign6730_e4744_d_n0;
        var_t7_dn2 = assign6730_e4744_d_n2;
        var_t7_dn4 = assign6730_e4744_d_n4;
        var_t7_dn5 = assign6730_e4744_d_n5;
        var_t7_dn6 = assign6730_e4744_d_n6;
        var_t7_dn8 = assign6730_e4744_d_n8;
        var_t7_dn10 = assign6730_e4744_d_n10;
        var_t7_dn11 = assign6730_e4744_d_n11;
        var_t7_dn12 = assign6730_e4744_d_n12;

        let (assign6740_e4758, assign6740_e4758_d_n0, assign6740_e4758_d_n2, assign6740_e4758_d_n4, assign6740_e4758_d_n5, assign6740_e4758_d_n6, assign6740_e4758_d_n8, assign6740_e4758_d_n10, assign6740_e4758_d_n11, assign6740_e4758_d_n12,) = {
    if (var_guard67 == 0.0) {
        let assign6740_e4749: f64 = (var_t7).sqrt();
        let assign6740_e4750: f64 = (var_t6 + assign6740_e4749);
        let assign6740_e4753: f64 = (2.0 * var_t4);
        let assign6740_e4755: f64 = (assign6740_e4753 * var_t4);
        let assign6740_e4756: f64 = (assign6740_e4750 / assign6740_e4755);
        (assign6740_e4756, ((((var_t6_dn0 + (var_t7_dn0 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * var_t4_dn0) * var_t4) + (assign6740_e4753 * var_t4_dn0)))) / (assign6740_e4755 * assign6740_e4755)), ((((var_t6_dn2 + (var_t7_dn2 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * var_t4_dn2) * var_t4) + (assign6740_e4753 * var_t4_dn2)))) / (assign6740_e4755 * assign6740_e4755)), ((((var_t6_dn4 + (var_t7_dn4 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * var_t4_dn4) * var_t4) + (assign6740_e4753 * var_t4_dn4)))) / (assign6740_e4755 * assign6740_e4755)), ((((var_t6_dn5 + (var_t7_dn5 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * var_t4_dn5) * var_t4) + (assign6740_e4753 * var_t4_dn5)))) / (assign6740_e4755 * assign6740_e4755)), ((((var_t6_dn6 + (var_t7_dn6 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * var_t4_dn6) * var_t4) + (assign6740_e4753 * var_t4_dn6)))) / (assign6740_e4755 * assign6740_e4755)), ((((var_t6_dn8 + (var_t7_dn8 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * var_t4_dn8) * var_t4) + (assign6740_e4753 * var_t4_dn8)))) / (assign6740_e4755 * assign6740_e4755)), ((((var_t6_dn10 + (var_t7_dn10 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * var_t4_dn10) * var_t4) + (assign6740_e4753 * var_t4_dn10)))) / (assign6740_e4755 * assign6740_e4755)), ((((var_t6_dn11 + (var_t7_dn11 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * var_t4_dn11) * var_t4) + (assign6740_e4753 * var_t4_dn11)))) / (assign6740_e4755 * assign6740_e4755)), ((((var_t6_dn12 + (var_t7_dn12 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * var_t4_dn12) * var_t4) + (assign6740_e4753 * var_t4_dn12)))) / (assign6740_e4755 * assign6740_e4755)),)
    } else {
        (var_phi_s0_bulk_0, var_phi_s0_bulk_0_dn0, var_phi_s0_bulk_0_dn2, var_phi_s0_bulk_0_dn4, var_phi_s0_bulk_0_dn5, var_phi_s0_bulk_0_dn6, var_phi_s0_bulk_0_dn8, var_phi_s0_bulk_0_dn10, var_phi_s0_bulk_0_dn11, var_phi_s0_bulk_0_dn12,)
    }
};
        var_phi_s0_bulk_0 = assign6740_e4758;
        var_phi_s0_bulk_0_dn0 = assign6740_e4758_d_n0;
        var_phi_s0_bulk_0_dn2 = assign6740_e4758_d_n2;
        var_phi_s0_bulk_0_dn4 = assign6740_e4758_d_n4;
        var_phi_s0_bulk_0_dn5 = assign6740_e4758_d_n5;
        var_phi_s0_bulk_0_dn6 = assign6740_e4758_d_n6;
        var_phi_s0_bulk_0_dn8 = assign6740_e4758_d_n8;
        var_phi_s0_bulk_0_dn10 = assign6740_e4758_d_n10;
        var_phi_s0_bulk_0_dn11 = assign6740_e4758_d_n11;
        var_phi_s0_bulk_0_dn12 = assign6740_e4758_d_n12;

        let assign6750_e4761: f64 = (2.0 / var_beta);
        let assign6750_e4764: f64 = (var_n_subbl / var_nin);
        let assign6750_e4765: f64 = (assign6750_e4764).ln();
        let assign6750_e4766: f64 = (assign6750_e4761 * assign6750_e4765);
        var_pb2_bulk = assign6750_e4766;

        let assign6760_e4769: f64 = (var_cnst0bulk * var_cnst0bulk);
        let assign6760_e4771: f64 = (assign6760_e4769 * var_c_box_fd_inv);
        let assign6760_e4773: f64 = (assign6760_e4771 * var_c_box_fd_inv);
        var_t0 = assign6760_e4773;
        var_t0_dn0 = ((((var_cnst0bulk_dn0 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn0)) * var_c_box_fd_inv) * var_c_box_fd_inv);
        var_t0_dn2 = ((((var_cnst0bulk_dn2 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn2)) * var_c_box_fd_inv) * var_c_box_fd_inv);
        var_t0_dn4 = ((((var_cnst0bulk_dn4 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn4)) * var_c_box_fd_inv) * var_c_box_fd_inv);
        var_t0_dn5 = ((((var_cnst0bulk_dn5 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn5)) * var_c_box_fd_inv) * var_c_box_fd_inv);
        var_t0_dn6 = ((((var_cnst0bulk_dn6 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn6)) * var_c_box_fd_inv) * var_c_box_fd_inv);
        var_t0_dn8 = ((((var_cnst0bulk_dn8 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn8)) * var_c_box_fd_inv) * var_c_box_fd_inv);
        var_t0_dn10 = ((((var_cnst0bulk_dn10 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn10)) * var_c_box_fd_inv) * var_c_box_fd_inv);
        var_t0_dn11 = ((((var_cnst0bulk_dn11 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn11)) * var_c_box_fd_inv) * var_c_box_fd_inv);
        var_t0_dn12 = ((((var_cnst0bulk_dn12 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn12)) * var_c_box_fd_inv) * var_c_box_fd_inv);

        let assign6770_e4775: f64 = (-var_vbsbiz);
        var_t1 = assign6770_e4775;
        var_t1_dn0 = (-var_vbsbiz_dn0);
        var_t1_dn2 = (-var_vbsbiz_dn2);
        var_t1_dn4 = (-var_vbsbiz_dn4);
        var_t1_dn5 = (-var_vbsbiz_dn5);
        var_t1_dn6 = (-var_vbsbiz_dn6);
        var_t1_dn8 = (-var_vbsbiz_dn8);
        var_t1_dn10 = (-var_vbsbiz_dn10);
        var_t1_dn11 = (-var_vbsbiz_dn11);
        var_t1_dn12 = (-var_vbsbiz_dn12);

        let assign6780_e4778: f64 = (2.0 * var_t1);
        let assign6780_e4781: f64 = (var_t0 * var_beta);
        let assign6780_e4782: f64 = (assign6780_e4778 + assign6780_e4781);
        let assign6780_e4785: f64 = (2.0 * var_t1);
        let assign6780_e4788: f64 = (var_t0 * var_beta);
        let assign6780_e4789: f64 = (assign6780_e4785 + assign6780_e4788);
        let assign6780_e4790: f64 = (assign6780_e4782 * assign6780_e4789);
        let assign6780_e4794: f64 = (var_t1 * var_t1);
        let assign6780_e4796: f64 = (assign6780_e4794 + var_t0);
        let assign6780_e4797: f64 = (4.0 * assign6780_e4796);
        let assign6780_e4798: f64 = (assign6780_e4790 - assign6780_e4797);
        var_t2 = assign6780_e4798;
        var_t2_dn0 = (((((2.0 * var_t1_dn0) + (var_t0_dn0 * var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * var_t1_dn0) + (var_t0_dn0 * var_beta)))) - (4.0 * (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) + var_t0_dn0)));
        var_t2_dn2 = (((((2.0 * var_t1_dn2) + (var_t0_dn2 * var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * var_t1_dn2) + (var_t0_dn2 * var_beta)))) - (4.0 * (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) + var_t0_dn2)));
        var_t2_dn4 = (((((2.0 * var_t1_dn4) + ((var_t0_dn4 * var_beta) + (var_t0 * var_beta_dn4))) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * var_t1_dn4) + ((var_t0_dn4 * var_beta) + (var_t0 * var_beta_dn4))))) - (4.0 * (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) + var_t0_dn4)));
        var_t2_dn5 = (((((2.0 * var_t1_dn5) + (var_t0_dn5 * var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * var_t1_dn5) + (var_t0_dn5 * var_beta)))) - (4.0 * (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) + var_t0_dn5)));
        var_t2_dn6 = (((((2.0 * var_t1_dn6) + (var_t0_dn6 * var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * var_t1_dn6) + (var_t0_dn6 * var_beta)))) - (4.0 * (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) + var_t0_dn6)));
        var_t2_dn8 = (((((2.0 * var_t1_dn8) + (var_t0_dn8 * var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * var_t1_dn8) + (var_t0_dn8 * var_beta)))) - (4.0 * (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) + var_t0_dn8)));
        var_t2_dn10 = (((((2.0 * var_t1_dn10) + (var_t0_dn10 * var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * var_t1_dn10) + (var_t0_dn10 * var_beta)))) - (4.0 * (((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) + var_t0_dn10)));
        var_t2_dn11 = (((((2.0 * var_t1_dn11) + (var_t0_dn11 * var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * var_t1_dn11) + (var_t0_dn11 * var_beta)))) - (4.0 * (((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) + var_t0_dn11)));
        var_t2_dn12 = (((((2.0 * var_t1_dn12) + (var_t0_dn12 * var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * var_t1_dn12) + (var_t0_dn12 * var_beta)))) - (4.0 * (((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)) + var_t0_dn12)));

        let assign6790_e4802: f64 = (10.0 * 2.220446049250313e-16);
        let (assign6790_e4808, assign6790_e4808_d_n0, assign6790_e4808_d_n2, assign6790_e4808_d_n4, assign6790_e4808_d_n5, assign6790_e4808_d_n6, assign6790_e4808_d_n8, assign6790_e4808_d_n10, assign6790_e4808_d_n11, assign6790_e4808_d_n12,) = {
    if (var_t2 >= assign6790_e4802) {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    } else {
        let assign6790_e4807: f64 = (10.0 * 2.220446049250313e-16);
        (assign6790_e4807, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_t2 = assign6790_e4808;
        var_t2_dn0 = assign6790_e4808_d_n0;
        var_t2_dn2 = assign6790_e4808_d_n2;
        var_t2_dn4 = assign6790_e4808_d_n4;
        var_t2_dn5 = assign6790_e4808_d_n5;
        var_t2_dn6 = assign6790_e4808_d_n6;
        var_t2_dn8 = assign6790_e4808_d_n8;
        var_t2_dn10 = assign6790_e4808_d_n10;
        var_t2_dn11 = assign6790_e4808_d_n11;
        var_t2_dn12 = assign6790_e4808_d_n12;

        let assign6800_e4810: f64 = (var_t2).sqrt();
        var_t2 = assign6800_e4810;
        var_t2_dn0 = (var_t2_dn0 / (2.0 * assign6800_e4810));
        var_t2_dn2 = (var_t2_dn2 / (2.0 * assign6800_e4810));
        var_t2_dn4 = (var_t2_dn4 / (2.0 * assign6800_e4810));
        var_t2_dn5 = (var_t2_dn5 / (2.0 * assign6800_e4810));
        var_t2_dn6 = (var_t2_dn6 / (2.0 * assign6800_e4810));
        var_t2_dn8 = (var_t2_dn8 / (2.0 * assign6800_e4810));
        var_t2_dn10 = (var_t2_dn10 / (2.0 * assign6800_e4810));
        var_t2_dn11 = (var_t2_dn11 / (2.0 * assign6800_e4810));
        var_t2_dn12 = (var_t2_dn12 / (2.0 * assign6800_e4810));

        let assign6810_e4813: f64 = (2.0 * var_t1);
        let assign6810_e4816: f64 = (var_t0 * var_beta);
        let assign6810_e4817: f64 = (assign6810_e4813 + assign6810_e4816);
        var_t3 = assign6810_e4817;
        var_t3_dn0 = ((2.0 * var_t1_dn0) + (var_t0_dn0 * var_beta));
        var_t3_dn2 = ((2.0 * var_t1_dn2) + (var_t0_dn2 * var_beta));
        var_t3_dn4 = ((2.0 * var_t1_dn4) + ((var_t0_dn4 * var_beta) + (var_t0 * var_beta_dn4)));
        var_t3_dn5 = ((2.0 * var_t1_dn5) + (var_t0_dn5 * var_beta));
        var_t3_dn6 = ((2.0 * var_t1_dn6) + (var_t0_dn6 * var_beta));
        var_t3_dn8 = ((2.0 * var_t1_dn8) + (var_t0_dn8 * var_beta));
        var_t3_dn10 = ((2.0 * var_t1_dn10) + (var_t0_dn10 * var_beta));
        var_t3_dn11 = ((2.0 * var_t1_dn11) + (var_t0_dn11 * var_beta));
        var_t3_dn12 = ((2.0 * var_t1_dn12) + (var_t0_dn12 * var_beta));

        let assign6820_e4820: f64 = (var_t3 - var_t2);
        let assign6820_e4822: f64 = (assign6820_e4820 / 2.0);
        var_psb_inia = assign6820_e4822;
        var_psb_inia_dn0 = ((var_t3_dn0 - var_t2_dn0) / 2.0);
        var_psb_inia_dn2 = ((var_t3_dn2 - var_t2_dn2) / 2.0);
        var_psb_inia_dn4 = ((var_t3_dn4 - var_t2_dn4) / 2.0);
        var_psb_inia_dn5 = ((var_t3_dn5 - var_t2_dn5) / 2.0);
        var_psb_inia_dn6 = ((var_t3_dn6 - var_t2_dn6) / 2.0);
        var_psb_inia_dn8 = ((var_t3_dn8 - var_t2_dn8) / 2.0);
        var_psb_inia_dn10 = ((var_t3_dn10 - var_t2_dn10) / 2.0);
        var_psb_inia_dn11 = ((var_t3_dn11 - var_t2_dn11) / 2.0);
        var_psb_inia_dn12 = ((var_t3_dn12 - var_t2_dn12) / 2.0);

        *var_guard67_slot = var_guard67;
        *var_pb2_bulk_slot = var_pb2_bulk;
        *var_phi_s0_bulk_0_slot = var_phi_s0_bulk_0;
        *var_phi_s0_bulk_0_dn0_slot = var_phi_s0_bulk_0_dn0;
        *var_phi_s0_bulk_0_dn10_slot = var_phi_s0_bulk_0_dn10;
        *var_phi_s0_bulk_0_dn11_slot = var_phi_s0_bulk_0_dn11;
        *var_phi_s0_bulk_0_dn12_slot = var_phi_s0_bulk_0_dn12;
        *var_phi_s0_bulk_0_dn2_slot = var_phi_s0_bulk_0_dn2;
        *var_phi_s0_bulk_0_dn4_slot = var_phi_s0_bulk_0_dn4;
        *var_phi_s0_bulk_0_dn5_slot = var_phi_s0_bulk_0_dn5;
        *var_phi_s0_bulk_0_dn6_slot = var_phi_s0_bulk_0_dn6;
        *var_phi_s0_bulk_0_dn8_slot = var_phi_s0_bulk_0_dn8;
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
        *var_vbsbiz_slot = var_vbsbiz;
        *var_vbsbiz_dn0_slot = var_vbsbiz_dn0;
        *var_vbsbiz_dn10_slot = var_vbsbiz_dn10;
        *var_vbsbiz_dn11_slot = var_vbsbiz_dn11;
        *var_vbsbiz_dn12_slot = var_vbsbiz_dn12;
        *var_vbsbiz_dn2_slot = var_vbsbiz_dn2;
        *var_vbsbiz_dn4_slot = var_vbsbiz_dn4;
        *var_vbsbiz_dn5_slot = var_vbsbiz_dn5;
        *var_vbsbiz_dn6_slot = var_vbsbiz_dn6;
        *var_vbsbiz_dn8_slot = var_vbsbiz_dn8;
    }

    pub(super) fn stamp_transient_block_17(
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
        var_pb2_bulk: f64,
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
        var_guard68_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
        var_phi_s0_bulk_0_slot: &mut f64,
        var_phi_s0_bulk_0_dn0_slot: &mut f64,
        var_phi_s0_bulk_0_dn10_slot: &mut f64,
        var_phi_s0_bulk_0_dn11_slot: &mut f64,
        var_phi_s0_bulk_0_dn12_slot: &mut f64,
        var_phi_s0_bulk_0_dn2_slot: &mut f64,
        var_phi_s0_bulk_0_dn4_slot: &mut f64,
        var_phi_s0_bulk_0_dn5_slot: &mut f64,
        var_phi_s0_bulk_0_dn6_slot: &mut f64,
        var_phi_s0_bulk_0_dn8_slot: &mut f64,
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
    ) {
        let mut var_guard68: f64 = *var_guard68_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
        let mut var_phi_s0_bulk_0: f64 = *var_phi_s0_bulk_0_slot;
        let mut var_phi_s0_bulk_0_dn0: f64 = *var_phi_s0_bulk_0_dn0_slot;
        let mut var_phi_s0_bulk_0_dn10: f64 = *var_phi_s0_bulk_0_dn10_slot;
        let mut var_phi_s0_bulk_0_dn11: f64 = *var_phi_s0_bulk_0_dn11_slot;
        let mut var_phi_s0_bulk_0_dn12: f64 = *var_phi_s0_bulk_0_dn12_slot;
        let mut var_phi_s0_bulk_0_dn2: f64 = *var_phi_s0_bulk_0_dn2_slot;
        let mut var_phi_s0_bulk_0_dn4: f64 = *var_phi_s0_bulk_0_dn4_slot;
        let mut var_phi_s0_bulk_0_dn5: f64 = *var_phi_s0_bulk_0_dn5_slot;
        let mut var_phi_s0_bulk_0_dn6: f64 = *var_phi_s0_bulk_0_dn6_slot;
        let mut var_phi_s0_bulk_0_dn8: f64 = *var_phi_s0_bulk_0_dn8_slot;
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

        let assign6830_e4825: f64 = (var_t1 * var_t1);
        let assign6830_e4827: f64 = (assign6830_e4825 / var_t0);
        let assign6830_e4829: f64 = (assign6830_e4827 / var_cnst1bulk);
        let assign6830_e4830: f64 = (assign6830_e4829).ln();
        let assign6830_e4834: f64 = (2.0 / var_t1);
        let assign6830_e4835: f64 = (var_beta + assign6830_e4834);
        let assign6830_e4836: f64 = (assign6830_e4830 / assign6830_e4835);
        var_psb_inib = assign6830_e4836;
        var_psb_inib_dn0 = ((((((((((((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) * var_t0) - (assign6830_e4825 * var_t0_dn0)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign6830_e4827 * var_cnst1bulk_dn0)) / (var_cnst1bulk * var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * var_t1_dn0) / (var_t1 * var_t1))))) / (assign6830_e4835 * assign6830_e4835));
        var_psb_inib_dn2 = ((((((((((((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) * var_t0) - (assign6830_e4825 * var_t0_dn2)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign6830_e4827 * var_cnst1bulk_dn2)) / (var_cnst1bulk * var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * var_t1_dn2) / (var_t1 * var_t1))))) / (assign6830_e4835 * assign6830_e4835));
        var_psb_inib_dn4 = ((((((((((((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) * var_t0) - (assign6830_e4825 * var_t0_dn4)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign6830_e4827 * var_cnst1bulk_dn4)) / (var_cnst1bulk * var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (var_beta_dn4 + (-((2.0 * var_t1_dn4) / (var_t1 * var_t1)))))) / (assign6830_e4835 * assign6830_e4835));
        var_psb_inib_dn5 = ((((((((((((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) * var_t0) - (assign6830_e4825 * var_t0_dn5)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign6830_e4827 * var_cnst1bulk_dn5)) / (var_cnst1bulk * var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * var_t1_dn5) / (var_t1 * var_t1))))) / (assign6830_e4835 * assign6830_e4835));
        var_psb_inib_dn6 = ((((((((((((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) * var_t0) - (assign6830_e4825 * var_t0_dn6)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign6830_e4827 * var_cnst1bulk_dn6)) / (var_cnst1bulk * var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * var_t1_dn6) / (var_t1 * var_t1))))) / (assign6830_e4835 * assign6830_e4835));
        var_psb_inib_dn8 = ((((((((((((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) * var_t0) - (assign6830_e4825 * var_t0_dn8)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign6830_e4827 * var_cnst1bulk_dn8)) / (var_cnst1bulk * var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * var_t1_dn8) / (var_t1 * var_t1))))) / (assign6830_e4835 * assign6830_e4835));
        var_psb_inib_dn10 = ((((((((((((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) * var_t0) - (assign6830_e4825 * var_t0_dn10)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign6830_e4827 * var_cnst1bulk_dn10)) / (var_cnst1bulk * var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * var_t1_dn10) / (var_t1 * var_t1))))) / (assign6830_e4835 * assign6830_e4835));
        var_psb_inib_dn11 = ((((((((((((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) * var_t0) - (assign6830_e4825 * var_t0_dn11)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign6830_e4827 * var_cnst1bulk_dn11)) / (var_cnst1bulk * var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * var_t1_dn11) / (var_t1 * var_t1))))) / (assign6830_e4835 * assign6830_e4835));
        var_psb_inib_dn12 = ((((((((((((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)) * var_t0) - (assign6830_e4825 * var_t0_dn12)) / (var_t0 * var_t0)) * var_cnst1bulk) - (assign6830_e4827 * var_cnst1bulk_dn12)) / (var_cnst1bulk * var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * var_t1_dn12) / (var_t1 * var_t1))))) / (assign6830_e4835 * assign6830_e4835));

        let assign6840_e4839: f64 = if var_psb_inia < var_pb2_bulk { 1.0 } else { 0.0 };
        var_guard68 = assign6840_e4839;

        let (assign6850_e4843, assign6850_e4843_d_n0, assign6850_e4843_d_n2, assign6850_e4843_d_n4, assign6850_e4843_d_n5, assign6850_e4843_d_n6, assign6850_e4843_d_n8, assign6850_e4843_d_n10, assign6850_e4843_d_n11, assign6850_e4843_d_n12,) = {
    if (var_guard68 != 0.0) {
        (var_psb_inia, var_psb_inia_dn0, var_psb_inia_dn2, var_psb_inia_dn4, var_psb_inia_dn5, var_psb_inia_dn6, var_psb_inia_dn8, var_psb_inia_dn10, var_psb_inia_dn11, var_psb_inia_dn12,)
    } else {
        (var_phi_s0_bulk_0, var_phi_s0_bulk_0_dn0, var_phi_s0_bulk_0_dn2, var_phi_s0_bulk_0_dn4, var_phi_s0_bulk_0_dn5, var_phi_s0_bulk_0_dn6, var_phi_s0_bulk_0_dn8, var_phi_s0_bulk_0_dn10, var_phi_s0_bulk_0_dn11, var_phi_s0_bulk_0_dn12,)
    }
};
        var_phi_s0_bulk_0 = assign6850_e4843;
        var_phi_s0_bulk_0_dn0 = assign6850_e4843_d_n0;
        var_phi_s0_bulk_0_dn2 = assign6850_e4843_d_n2;
        var_phi_s0_bulk_0_dn4 = assign6850_e4843_d_n4;
        var_phi_s0_bulk_0_dn5 = assign6850_e4843_d_n5;
        var_phi_s0_bulk_0_dn6 = assign6850_e4843_d_n6;
        var_phi_s0_bulk_0_dn8 = assign6850_e4843_d_n8;
        var_phi_s0_bulk_0_dn10 = assign6850_e4843_d_n10;
        var_phi_s0_bulk_0_dn11 = assign6850_e4843_d_n11;
        var_phi_s0_bulk_0_dn12 = assign6850_e4843_d_n12;

        let (assign6860_e4852, assign6860_e4852_d_n0, assign6860_e4852_d_n2, assign6860_e4852_d_n4, assign6860_e4852_d_n5, assign6860_e4852_d_n6, assign6860_e4852_d_n8, assign6860_e4852_d_n10, assign6860_e4852_d_n11, assign6860_e4852_d_n12,) = {
    if (var_guard68 == 0.0) {
        let assign6860_e4848: f64 = (var_psb_inib - var_psb_inia);
        let assign6860_e4850: f64 = (assign6860_e4848 - 0.0008);
        (assign6860_e4850, (var_psb_inib_dn0 - var_psb_inia_dn0), (var_psb_inib_dn2 - var_psb_inia_dn2), (var_psb_inib_dn4 - var_psb_inia_dn4), (var_psb_inib_dn5 - var_psb_inia_dn5), (var_psb_inib_dn6 - var_psb_inia_dn6), (var_psb_inib_dn8 - var_psb_inia_dn8), (var_psb_inib_dn10 - var_psb_inia_dn10), (var_psb_inib_dn11 - var_psb_inia_dn11), (var_psb_inib_dn12 - var_psb_inia_dn12),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn8, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12,)
    }
};
        var_tmf1 = assign6860_e4852;
        var_tmf1_dn0 = assign6860_e4852_d_n0;
        var_tmf1_dn2 = assign6860_e4852_d_n2;
        var_tmf1_dn4 = assign6860_e4852_d_n4;
        var_tmf1_dn5 = assign6860_e4852_d_n5;
        var_tmf1_dn6 = assign6860_e4852_d_n6;
        var_tmf1_dn8 = assign6860_e4852_d_n8;
        var_tmf1_dn10 = assign6860_e4852_d_n10;
        var_tmf1_dn11 = assign6860_e4852_d_n11;
        var_tmf1_dn12 = assign6860_e4852_d_n12;

        let (assign6870_e4861, assign6870_e4861_d_n0, assign6870_e4861_d_n2, assign6870_e4861_d_n4, assign6870_e4861_d_n5, assign6870_e4861_d_n6, assign6870_e4861_d_n8, assign6870_e4861_d_n10, assign6870_e4861_d_n11, assign6870_e4861_d_n12,) = {
    if (var_guard68 == 0.0) {
        let assign6870_e4857: f64 = (4.0 * var_psb_inib);
        let assign6870_e4859: f64 = (assign6870_e4857 * 0.0008);
        (assign6870_e4859, ((4.0 * var_psb_inib_dn0) * 0.0008), ((4.0 * var_psb_inib_dn2) * 0.0008), ((4.0 * var_psb_inib_dn4) * 0.0008), ((4.0 * var_psb_inib_dn5) * 0.0008), ((4.0 * var_psb_inib_dn6) * 0.0008), ((4.0 * var_psb_inib_dn8) * 0.0008), ((4.0 * var_psb_inib_dn10) * 0.0008), ((4.0 * var_psb_inib_dn11) * 0.0008), ((4.0 * var_psb_inib_dn12) * 0.0008),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
        var_tmf2 = assign6870_e4861;
        var_tmf2_dn0 = assign6870_e4861_d_n0;
        var_tmf2_dn2 = assign6870_e4861_d_n2;
        var_tmf2_dn4 = assign6870_e4861_d_n4;
        var_tmf2_dn5 = assign6870_e4861_d_n5;
        var_tmf2_dn6 = assign6870_e4861_d_n6;
        var_tmf2_dn8 = assign6870_e4861_d_n8;
        var_tmf2_dn10 = assign6870_e4861_d_n10;
        var_tmf2_dn11 = assign6870_e4861_d_n11;
        var_tmf2_dn12 = assign6870_e4861_d_n12;

        let (assign6880_e4872, assign6880_e4872_d_n0, assign6880_e4872_d_n2, assign6880_e4872_d_n4, assign6880_e4872_d_n5, assign6880_e4872_d_n6, assign6880_e4872_d_n8, assign6880_e4872_d_n10, assign6880_e4872_d_n11, assign6880_e4872_d_n12,) = {
    if (var_guard68 == 0.0) {
        let (assign6880_e4870, assign6880_e4870_d_n0, assign6880_e4870_d_n2, assign6880_e4870_d_n4, assign6880_e4870_d_n5, assign6880_e4870_d_n6, assign6880_e4870_d_n8, assign6880_e4870_d_n10, assign6880_e4870_d_n11, assign6880_e4870_d_n12,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
            } else {
                let assign6880_e4869: f64 = (-var_tmf2);
                (assign6880_e4869, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn8), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12),)
            }
        };
        (assign6880_e4870, assign6880_e4870_d_n0, assign6880_e4870_d_n2, assign6880_e4870_d_n4, assign6880_e4870_d_n5, assign6880_e4870_d_n6, assign6880_e4870_d_n8, assign6880_e4870_d_n10, assign6880_e4870_d_n11, assign6880_e4870_d_n12,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
        var_tmf2 = assign6880_e4872;
        var_tmf2_dn0 = assign6880_e4872_d_n0;
        var_tmf2_dn2 = assign6880_e4872_d_n2;
        var_tmf2_dn4 = assign6880_e4872_d_n4;
        var_tmf2_dn5 = assign6880_e4872_d_n5;
        var_tmf2_dn6 = assign6880_e4872_d_n6;
        var_tmf2_dn8 = assign6880_e4872_d_n8;
        var_tmf2_dn10 = assign6880_e4872_d_n10;
        var_tmf2_dn11 = assign6880_e4872_d_n11;
        var_tmf2_dn12 = assign6880_e4872_d_n12;

        let (assign6890_e4882, assign6890_e4882_d_n0, assign6890_e4882_d_n2, assign6890_e4882_d_n4, assign6890_e4882_d_n5, assign6890_e4882_d_n6, assign6890_e4882_d_n8, assign6890_e4882_d_n10, assign6890_e4882_d_n11, assign6890_e4882_d_n12,) = {
    if (var_guard68 == 0.0) {
        let assign6890_e4877: f64 = (var_tmf1 * var_tmf1);
        let assign6890_e4879: f64 = (assign6890_e4877 + var_tmf2);
        let assign6890_e4880: f64 = (assign6890_e4879).sqrt();
        (assign6890_e4880, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign6890_e4880)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign6890_e4880)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign6890_e4880)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign6890_e4880)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign6890_e4880)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign6890_e4880)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign6890_e4880)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign6890_e4880)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign6890_e4880)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    }
};
        var_tmf2 = assign6890_e4882;
        var_tmf2_dn0 = assign6890_e4882_d_n0;
        var_tmf2_dn2 = assign6890_e4882_d_n2;
        var_tmf2_dn4 = assign6890_e4882_d_n4;
        var_tmf2_dn5 = assign6890_e4882_d_n5;
        var_tmf2_dn6 = assign6890_e4882_d_n6;
        var_tmf2_dn8 = assign6890_e4882_d_n8;
        var_tmf2_dn10 = assign6890_e4882_d_n10;
        var_tmf2_dn11 = assign6890_e4882_d_n11;
        var_tmf2_dn12 = assign6890_e4882_d_n12;

        let (assign6900_e4893, assign6900_e4893_d_n0, assign6900_e4893_d_n2, assign6900_e4893_d_n4, assign6900_e4893_d_n5, assign6900_e4893_d_n6, assign6900_e4893_d_n8, assign6900_e4893_d_n10, assign6900_e4893_d_n11, assign6900_e4893_d_n12,) = {
    if (var_guard68 == 0.0) {
        let assign6900_e4889: f64 = (var_tmf1 / var_tmf2);
        let assign6900_e4890: f64 = (1.0 + assign6900_e4889);
        let assign6900_e4891: f64 = (0.5 * assign6900_e4890);
        (assign6900_e4891, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn12 * var_tmf2) - (var_tmf1 * var_tmf2_dn12)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn8, var_t1_dn10, var_t1_dn11, var_t1_dn12,)
    }
};
        var_t1 = assign6900_e4893;
        var_t1_dn0 = assign6900_e4893_d_n0;
        var_t1_dn2 = assign6900_e4893_d_n2;
        var_t1_dn4 = assign6900_e4893_d_n4;
        var_t1_dn5 = assign6900_e4893_d_n5;
        var_t1_dn6 = assign6900_e4893_d_n6;
        var_t1_dn8 = assign6900_e4893_d_n8;
        var_t1_dn10 = assign6900_e4893_d_n10;
        var_t1_dn11 = assign6900_e4893_d_n11;
        var_t1_dn12 = assign6900_e4893_d_n12;

        let (assign6910_e4904, assign6910_e4904_d_n0, assign6910_e4904_d_n2, assign6910_e4904_d_n4, assign6910_e4904_d_n5, assign6910_e4904_d_n6, assign6910_e4904_d_n8, assign6910_e4904_d_n10, assign6910_e4904_d_n11, assign6910_e4904_d_n12,) = {
    if (var_guard68 == 0.0) {
        let assign6910_e4900: f64 = (var_tmf1 + var_tmf2);
        let assign6910_e4901: f64 = (0.5 * assign6910_e4900);
        let assign6910_e4902: f64 = (var_psb_inib - assign6910_e4901);
        (assign6910_e4902, (var_psb_inib_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_psb_inib_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_psb_inib_dn4 - (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_psb_inib_dn5 - (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_psb_inib_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_psb_inib_dn8 - (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), (var_psb_inib_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_psb_inib_dn11 - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (var_psb_inib_dn12 - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))),)
    } else {
        (var_phi_s0_bulk_0, var_phi_s0_bulk_0_dn0, var_phi_s0_bulk_0_dn2, var_phi_s0_bulk_0_dn4, var_phi_s0_bulk_0_dn5, var_phi_s0_bulk_0_dn6, var_phi_s0_bulk_0_dn8, var_phi_s0_bulk_0_dn10, var_phi_s0_bulk_0_dn11, var_phi_s0_bulk_0_dn12,)
    }
};
        var_phi_s0_bulk_0 = assign6910_e4904;
        var_phi_s0_bulk_0_dn0 = assign6910_e4904_d_n0;
        var_phi_s0_bulk_0_dn2 = assign6910_e4904_d_n2;
        var_phi_s0_bulk_0_dn4 = assign6910_e4904_d_n4;
        var_phi_s0_bulk_0_dn5 = assign6910_e4904_d_n5;
        var_phi_s0_bulk_0_dn6 = assign6910_e4904_d_n6;
        var_phi_s0_bulk_0_dn8 = assign6910_e4904_d_n8;
        var_phi_s0_bulk_0_dn10 = assign6910_e4904_d_n10;
        var_phi_s0_bulk_0_dn11 = assign6910_e4904_d_n11;
        var_phi_s0_bulk_0_dn12 = assign6910_e4904_d_n12;

        var_lp_s0 = 0.0;

        *var_guard68_slot = var_guard68;
        *var_lp_s0_slot = var_lp_s0;
        *var_phi_s0_bulk_0_slot = var_phi_s0_bulk_0;
        *var_phi_s0_bulk_0_dn0_slot = var_phi_s0_bulk_0_dn0;
        *var_phi_s0_bulk_0_dn10_slot = var_phi_s0_bulk_0_dn10;
        *var_phi_s0_bulk_0_dn11_slot = var_phi_s0_bulk_0_dn11;
        *var_phi_s0_bulk_0_dn12_slot = var_phi_s0_bulk_0_dn12;
        *var_phi_s0_bulk_0_dn2_slot = var_phi_s0_bulk_0_dn2;
        *var_phi_s0_bulk_0_dn4_slot = var_phi_s0_bulk_0_dn4;
        *var_phi_s0_bulk_0_dn5_slot = var_phi_s0_bulk_0_dn5;
        *var_phi_s0_bulk_0_dn6_slot = var_phi_s0_bulk_0_dn6;
        *var_phi_s0_bulk_0_dn8_slot = var_phi_s0_bulk_0_dn8;
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
    }

    pub(super) fn stamp_transient_block_18(
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
        var_guard69_slot: &mut f64,
        var_guard70_slot: &mut f64,
        var_guard71_slot: &mut f64,
        var_guard72_slot: &mut f64,
        var_guard73_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
        var_phi_b_dep_slot: &mut f64,
        var_phi_b_dep0_slot: &mut f64,
        var_phi_b_dep0_dn0_slot: &mut f64,
        var_phi_b_dep0_dn10_slot: &mut f64,
        var_phi_b_dep0_dn11_slot: &mut f64,
        var_phi_b_dep0_dn12_slot: &mut f64,
        var_phi_b_dep0_dn2_slot: &mut f64,
        var_phi_b_dep0_dn4_slot: &mut f64,
        var_phi_b_dep0_dn5_slot: &mut f64,
        var_phi_b_dep0_dn6_slot: &mut f64,
        var_phi_b_dep0_dn8_slot: &mut f64,
        var_phi_b_dep0_dpsb_slot: &mut f64,
        var_phi_b_dep0_dpsb_dn0_slot: &mut f64,
        var_phi_b_dep0_dpsb_dn10_slot: &mut f64,
        var_phi_b_dep0_dpsb_dn11_slot: &mut f64,
        var_phi_b_dep0_dpsb_dn12_slot: &mut f64,
        var_phi_b_dep0_dpsb_dn2_slot: &mut f64,
        var_phi_b_dep0_dpsb_dn4_slot: &mut f64,
        var_phi_b_dep0_dpsb_dn5_slot: &mut f64,
        var_phi_b_dep0_dpsb_dn6_slot: &mut f64,
        var_phi_b_dep0_dpsb_dn8_slot: &mut f64,
        var_phi_b_dep_dn0_slot: &mut f64,
        var_phi_b_dep_dn10_slot: &mut f64,
        var_phi_b_dep_dn11_slot: &mut f64,
        var_phi_b_dep_dn12_slot: &mut f64,
        var_phi_b_dep_dn2_slot: &mut f64,
        var_phi_b_dep_dn4_slot: &mut f64,
        var_phi_b_dep_dn5_slot: &mut f64,
        var_phi_b_dep_dn6_slot: &mut f64,
        var_phi_b_dep_dn8_slot: &mut f64,
        var_phi_s0_bulk_0_slot: &mut f64,
        var_phi_s0_bulk_0_dn0_slot: &mut f64,
        var_phi_s0_bulk_0_dn10_slot: &mut f64,
        var_phi_s0_bulk_0_dn11_slot: &mut f64,
        var_phi_s0_bulk_0_dn12_slot: &mut f64,
        var_phi_s0_bulk_0_dn2_slot: &mut f64,
        var_phi_s0_bulk_0_dn4_slot: &mut f64,
        var_phi_s0_bulk_0_dn5_slot: &mut f64,
        var_phi_s0_bulk_0_dn6_slot: &mut f64,
        var_phi_s0_bulk_0_dn8_slot: &mut f64,
        var_q_s0_bulk_0_slot: &mut f64,
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
    ) {
        let mut var_guard69: f64 = *var_guard69_slot;
        let mut var_guard70: f64 = *var_guard70_slot;
        let mut var_guard71: f64 = *var_guard71_slot;
        let mut var_guard72: f64 = *var_guard72_slot;
        let mut var_guard73: f64 = *var_guard73_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
        let mut var_phi_b_dep: f64 = *var_phi_b_dep_slot;
        let mut var_phi_b_dep0: f64 = *var_phi_b_dep0_slot;
        let mut var_phi_b_dep0_dn0: f64 = *var_phi_b_dep0_dn0_slot;
        let mut var_phi_b_dep0_dn10: f64 = *var_phi_b_dep0_dn10_slot;
        let mut var_phi_b_dep0_dn11: f64 = *var_phi_b_dep0_dn11_slot;
        let mut var_phi_b_dep0_dn12: f64 = *var_phi_b_dep0_dn12_slot;
        let mut var_phi_b_dep0_dn2: f64 = *var_phi_b_dep0_dn2_slot;
        let mut var_phi_b_dep0_dn4: f64 = *var_phi_b_dep0_dn4_slot;
        let mut var_phi_b_dep0_dn5: f64 = *var_phi_b_dep0_dn5_slot;
        let mut var_phi_b_dep0_dn6: f64 = *var_phi_b_dep0_dn6_slot;
        let mut var_phi_b_dep0_dn8: f64 = *var_phi_b_dep0_dn8_slot;
        let mut var_phi_b_dep0_dpsb: f64 = *var_phi_b_dep0_dpsb_slot;
        let mut var_phi_b_dep0_dpsb_dn0: f64 = *var_phi_b_dep0_dpsb_dn0_slot;
        let mut var_phi_b_dep0_dpsb_dn10: f64 = *var_phi_b_dep0_dpsb_dn10_slot;
        let mut var_phi_b_dep0_dpsb_dn11: f64 = *var_phi_b_dep0_dpsb_dn11_slot;
        let mut var_phi_b_dep0_dpsb_dn12: f64 = *var_phi_b_dep0_dpsb_dn12_slot;
        let mut var_phi_b_dep0_dpsb_dn2: f64 = *var_phi_b_dep0_dpsb_dn2_slot;
        let mut var_phi_b_dep0_dpsb_dn4: f64 = *var_phi_b_dep0_dpsb_dn4_slot;
        let mut var_phi_b_dep0_dpsb_dn5: f64 = *var_phi_b_dep0_dpsb_dn5_slot;
        let mut var_phi_b_dep0_dpsb_dn6: f64 = *var_phi_b_dep0_dpsb_dn6_slot;
        let mut var_phi_b_dep0_dpsb_dn8: f64 = *var_phi_b_dep0_dpsb_dn8_slot;
        let mut var_phi_b_dep_dn0: f64 = *var_phi_b_dep_dn0_slot;
        let mut var_phi_b_dep_dn10: f64 = *var_phi_b_dep_dn10_slot;
        let mut var_phi_b_dep_dn11: f64 = *var_phi_b_dep_dn11_slot;
        let mut var_phi_b_dep_dn12: f64 = *var_phi_b_dep_dn12_slot;
        let mut var_phi_b_dep_dn2: f64 = *var_phi_b_dep_dn2_slot;
        let mut var_phi_b_dep_dn4: f64 = *var_phi_b_dep_dn4_slot;
        let mut var_phi_b_dep_dn5: f64 = *var_phi_b_dep_dn5_slot;
        let mut var_phi_b_dep_dn6: f64 = *var_phi_b_dep_dn6_slot;
        let mut var_phi_b_dep_dn8: f64 = *var_phi_b_dep_dn8_slot;
        let mut var_phi_s0_bulk_0: f64 = *var_phi_s0_bulk_0_slot;
        let mut var_phi_s0_bulk_0_dn0: f64 = *var_phi_s0_bulk_0_dn0_slot;
        let mut var_phi_s0_bulk_0_dn10: f64 = *var_phi_s0_bulk_0_dn10_slot;
        let mut var_phi_s0_bulk_0_dn11: f64 = *var_phi_s0_bulk_0_dn11_slot;
        let mut var_phi_s0_bulk_0_dn12: f64 = *var_phi_s0_bulk_0_dn12_slot;
        let mut var_phi_s0_bulk_0_dn2: f64 = *var_phi_s0_bulk_0_dn2_slot;
        let mut var_phi_s0_bulk_0_dn4: f64 = *var_phi_s0_bulk_0_dn4_slot;
        let mut var_phi_s0_bulk_0_dn5: f64 = *var_phi_s0_bulk_0_dn5_slot;
        let mut var_phi_s0_bulk_0_dn6: f64 = *var_phi_s0_bulk_0_dn6_slot;
        let mut var_phi_s0_bulk_0_dn8: f64 = *var_phi_s0_bulk_0_dn8_slot;
        let mut var_q_s0_bulk_0: f64 = *var_q_s0_bulk_0_slot;
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

        let mut assign6930_loop_guard: usize = 0;
        while {
            let assign6930_cond_e4908: f64 = if var_lp_s0 < var_lp_s0_max { 1.0 } else { 0.0 };
            assign6930_cond_e4908 != 0.0
        } {
            assign6930_loop_guard += 1;
            assert!(assign6930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            var_t1 = var_cnst0bulk;
            var_t1_dn0 = var_cnst0bulk_dn0;
            var_t1_dn2 = var_cnst0bulk_dn2;
            var_t1_dn4 = var_cnst0bulk_dn4;
            var_t1_dn5 = var_cnst0bulk_dn5;
            var_t1_dn6 = var_cnst0bulk_dn6;
            var_t1_dn8 = var_cnst0bulk_dn8;
            var_t1_dn10 = var_cnst0bulk_dn10;
            var_t1_dn11 = var_cnst0bulk_dn11;
            var_t1_dn12 = var_cnst0bulk_dn12;
            let assign6930_body1_e4912: f64 = (var_beta * var_phi_s0_bulk_0);
            var_t2 = assign6930_body1_e4912;
            var_t2_dn0 = (var_beta * var_phi_s0_bulk_0_dn0);
            var_t2_dn2 = (var_beta * var_phi_s0_bulk_0_dn2);
            var_t2_dn4 = ((var_beta_dn4 * var_phi_s0_bulk_0) + (var_beta * var_phi_s0_bulk_0_dn4));
            var_t2_dn5 = (var_beta * var_phi_s0_bulk_0_dn5);
            var_t2_dn6 = (var_beta * var_phi_s0_bulk_0_dn6);
            var_t2_dn8 = (var_beta * var_phi_s0_bulk_0_dn8);
            var_t2_dn10 = (var_beta * var_phi_s0_bulk_0_dn10);
            var_t2_dn11 = (var_beta * var_phi_s0_bulk_0_dn11);
            var_t2_dn12 = (var_beta * var_phi_s0_bulk_0_dn12);
            let assign6930_body2_e4914: f64 = (-var_t2);
            let assign6930_body2_e4915: f64 = (assign6930_body2_e4914).exp();
            var_t3 = assign6930_body2_e4915;
            var_t3_dn0 = (assign6930_body2_e4915 * (-var_t2_dn0));
            var_t3_dn2 = (assign6930_body2_e4915 * (-var_t2_dn2));
            var_t3_dn4 = (assign6930_body2_e4915 * (-var_t2_dn4));
            var_t3_dn5 = (assign6930_body2_e4915 * (-var_t2_dn5));
            var_t3_dn6 = (assign6930_body2_e4915 * (-var_t2_dn6));
            var_t3_dn8 = (assign6930_body2_e4915 * (-var_t2_dn8));
            var_t3_dn10 = (assign6930_body2_e4915 * (-var_t2_dn10));
            var_t3_dn11 = (assign6930_body2_e4915 * (-var_t2_dn11));
            var_t3_dn12 = (assign6930_body2_e4915 * (-var_t2_dn12));
            let assign6930_body3_e4918: f64 = if var_phi_s0_bulk_0 > 1e-8 { 1.0 } else { 0.0 };
            var_guard69 = assign6930_body3_e4918;
            let (assign6930_body4_e4925, assign6930_body4_e4925_d_n0, assign6930_body4_e4925_d_n2, assign6930_body4_e4925_d_n4, assign6930_body4_e4925_d_n5, assign6930_body4_e4925_d_n6, assign6930_body4_e4925_d_n8, assign6930_body4_e4925_d_n10, assign6930_body4_e4925_d_n11, assign6930_body4_e4925_d_n12,) = {
    if (var_guard69 != 0.0) {
        let assign6930_body4_e4922: f64 = (var_beta * var_phi_s0_bulk_0);
        let assign6930_body4_e4923: f64 = (assign6930_body4_e4922).exp();
        (assign6930_body4_e4923, (assign6930_body4_e4923 * (var_beta * var_phi_s0_bulk_0_dn0)), (assign6930_body4_e4923 * (var_beta * var_phi_s0_bulk_0_dn2)), (assign6930_body4_e4923 * ((var_beta_dn4 * var_phi_s0_bulk_0) + (var_beta * var_phi_s0_bulk_0_dn4))), (assign6930_body4_e4923 * (var_beta * var_phi_s0_bulk_0_dn5)), (assign6930_body4_e4923 * (var_beta * var_phi_s0_bulk_0_dn6)), (assign6930_body4_e4923 * (var_beta * var_phi_s0_bulk_0_dn8)), (assign6930_body4_e4923 * (var_beta * var_phi_s0_bulk_0_dn10)), (assign6930_body4_e4923 * (var_beta * var_phi_s0_bulk_0_dn11)), (assign6930_body4_e4923 * (var_beta * var_phi_s0_bulk_0_dn12)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
            var_t0 = assign6930_body4_e4925;
            var_t0_dn0 = assign6930_body4_e4925_d_n0;
            var_t0_dn2 = assign6930_body4_e4925_d_n2;
            var_t0_dn4 = assign6930_body4_e4925_d_n4;
            var_t0_dn5 = assign6930_body4_e4925_d_n5;
            var_t0_dn6 = assign6930_body4_e4925_d_n6;
            var_t0_dn8 = assign6930_body4_e4925_d_n8;
            var_t0_dn10 = assign6930_body4_e4925_d_n10;
            var_t0_dn11 = assign6930_body4_e4925_d_n11;
            var_t0_dn12 = assign6930_body4_e4925_d_n12;
            let (assign6930_body5_e4943, assign6930_body5_e4943_d_n0, assign6930_body5_e4943_d_n2, assign6930_body5_e4943_d_n4, assign6930_body5_e4943_d_n5, assign6930_body5_e4943_d_n6, assign6930_body5_e4943_d_n8, assign6930_body5_e4943_d_n10, assign6930_body5_e4943_d_n11, assign6930_body5_e4943_d_n12,) = {
    if (var_guard69 != 0.0) {
        let assign6930_body5_e4928: f64 = (-var_t1);
        let assign6930_body5_e4931: f64 = (var_t3 + var_t2);
        let assign6930_body5_e4933: f64 = (assign6930_body5_e4931 - 1.0);
        let assign6930_body5_e4937: f64 = (var_t0 - 1.0);
        let assign6930_body5_e4938: f64 = (var_cnst1bulk * assign6930_body5_e4937);
        let assign6930_body5_e4939: f64 = (assign6930_body5_e4933 + assign6930_body5_e4938);
        let assign6930_body5_e4940: f64 = (assign6930_body5_e4939).sqrt();
        let assign6930_body5_e4941: f64 = (assign6930_body5_e4928 * assign6930_body5_e4940);
        (assign6930_body5_e4941, (((-var_t1_dn0) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((var_t3_dn0 + var_t2_dn0) + ((var_cnst1bulk_dn0 * assign6930_body5_e4937) + (var_cnst1bulk * var_t0_dn0))) / (2.0 * assign6930_body5_e4940)))), (((-var_t1_dn2) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((var_t3_dn2 + var_t2_dn2) + ((var_cnst1bulk_dn2 * assign6930_body5_e4937) + (var_cnst1bulk * var_t0_dn2))) / (2.0 * assign6930_body5_e4940)))), (((-var_t1_dn4) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((var_t3_dn4 + var_t2_dn4) + ((var_cnst1bulk_dn4 * assign6930_body5_e4937) + (var_cnst1bulk * var_t0_dn4))) / (2.0 * assign6930_body5_e4940)))), (((-var_t1_dn5) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((var_t3_dn5 + var_t2_dn5) + ((var_cnst1bulk_dn5 * assign6930_body5_e4937) + (var_cnst1bulk * var_t0_dn5))) / (2.0 * assign6930_body5_e4940)))), (((-var_t1_dn6) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((var_t3_dn6 + var_t2_dn6) + ((var_cnst1bulk_dn6 * assign6930_body5_e4937) + (var_cnst1bulk * var_t0_dn6))) / (2.0 * assign6930_body5_e4940)))), (((-var_t1_dn8) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((var_t3_dn8 + var_t2_dn8) + ((var_cnst1bulk_dn8 * assign6930_body5_e4937) + (var_cnst1bulk * var_t0_dn8))) / (2.0 * assign6930_body5_e4940)))), (((-var_t1_dn10) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((var_t3_dn10 + var_t2_dn10) + ((var_cnst1bulk_dn10 * assign6930_body5_e4937) + (var_cnst1bulk * var_t0_dn10))) / (2.0 * assign6930_body5_e4940)))), (((-var_t1_dn11) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((var_t3_dn11 + var_t2_dn11) + ((var_cnst1bulk_dn11 * assign6930_body5_e4937) + (var_cnst1bulk * var_t0_dn11))) / (2.0 * assign6930_body5_e4940)))), (((-var_t1_dn12) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((var_t3_dn12 + var_t2_dn12) + ((var_cnst1bulk_dn12 * assign6930_body5_e4937) + (var_cnst1bulk * var_t0_dn12))) / (2.0 * assign6930_body5_e4940)))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign6930_body5_e4943;
            var_t4_dn0 = assign6930_body5_e4943_d_n0;
            var_t4_dn2 = assign6930_body5_e4943_d_n2;
            var_t4_dn4 = assign6930_body5_e4943_d_n4;
            var_t4_dn5 = assign6930_body5_e4943_d_n5;
            var_t4_dn6 = assign6930_body5_e4943_d_n6;
            var_t4_dn8 = assign6930_body5_e4943_d_n8;
            var_t4_dn10 = assign6930_body5_e4943_d_n10;
            var_t4_dn11 = assign6930_body5_e4943_d_n11;
            var_t4_dn12 = assign6930_body5_e4943_d_n12;
            let (assign6930_body6_e4958, assign6930_body6_e4958_d_n0, assign6930_body6_e4958_d_n2, assign6930_body6_e4958_d_n4, assign6930_body6_e4958_d_n5, assign6930_body6_e4958_d_n6, assign6930_body6_e4958_d_n8, assign6930_body6_e4958_d_n10, assign6930_body6_e4958_d_n11, assign6930_body6_e4958_d_n12,) = {
    if (var_guard69 != 0.0) {
        let assign6930_body6_e4947: f64 = (var_c0bulk / var_t4);
        let assign6930_body6_e4949: f64 = (-var_t3);
        let assign6930_body6_e4951: f64 = (assign6930_body6_e4949 + 1.0);
        let assign6930_body6_e4954: f64 = (var_cnst1bulk * var_t0);
        let assign6930_body6_e4955: f64 = (assign6930_body6_e4951 + assign6930_body6_e4954);
        let assign6930_body6_e4956: f64 = (assign6930_body6_e4947 * assign6930_body6_e4955);
        (assign6930_body6_e4956, (((((var_c0bulk_dn0 * var_t4) - (var_c0bulk * var_t4_dn0)) / (var_t4 * var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-var_t3_dn0) + ((var_cnst1bulk_dn0 * var_t0) + (var_cnst1bulk * var_t0_dn0))))), (((((var_c0bulk_dn2 * var_t4) - (var_c0bulk * var_t4_dn2)) / (var_t4 * var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-var_t3_dn2) + ((var_cnst1bulk_dn2 * var_t0) + (var_cnst1bulk * var_t0_dn2))))), (((((var_c0bulk_dn4 * var_t4) - (var_c0bulk * var_t4_dn4)) / (var_t4 * var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-var_t3_dn4) + ((var_cnst1bulk_dn4 * var_t0) + (var_cnst1bulk * var_t0_dn4))))), (((((var_c0bulk_dn5 * var_t4) - (var_c0bulk * var_t4_dn5)) / (var_t4 * var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-var_t3_dn5) + ((var_cnst1bulk_dn5 * var_t0) + (var_cnst1bulk * var_t0_dn5))))), (((((var_c0bulk_dn6 * var_t4) - (var_c0bulk * var_t4_dn6)) / (var_t4 * var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-var_t3_dn6) + ((var_cnst1bulk_dn6 * var_t0) + (var_cnst1bulk * var_t0_dn6))))), (((((var_c0bulk_dn8 * var_t4) - (var_c0bulk * var_t4_dn8)) / (var_t4 * var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-var_t3_dn8) + ((var_cnst1bulk_dn8 * var_t0) + (var_cnst1bulk * var_t0_dn8))))), (((((var_c0bulk_dn10 * var_t4) - (var_c0bulk * var_t4_dn10)) / (var_t4 * var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-var_t3_dn10) + ((var_cnst1bulk_dn10 * var_t0) + (var_cnst1bulk * var_t0_dn10))))), (((((var_c0bulk_dn11 * var_t4) - (var_c0bulk * var_t4_dn11)) / (var_t4 * var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-var_t3_dn11) + ((var_cnst1bulk_dn11 * var_t0) + (var_cnst1bulk * var_t0_dn11))))), (((((var_c0bulk_dn12 * var_t4) - (var_c0bulk * var_t4_dn12)) / (var_t4 * var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-var_t3_dn12) + ((var_cnst1bulk_dn12 * var_t0) + (var_cnst1bulk * var_t0_dn12))))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign6930_body6_e4958;
            var_t5_dn0 = assign6930_body6_e4958_d_n0;
            var_t5_dn2 = assign6930_body6_e4958_d_n2;
            var_t5_dn4 = assign6930_body6_e4958_d_n4;
            var_t5_dn5 = assign6930_body6_e4958_d_n5;
            var_t5_dn6 = assign6930_body6_e4958_d_n6;
            var_t5_dn8 = assign6930_body6_e4958_d_n8;
            var_t5_dn10 = assign6930_body6_e4958_d_n10;
            var_t5_dn11 = assign6930_body6_e4958_d_n11;
            var_t5_dn12 = assign6930_body6_e4958_d_n12;
            let assign6930_body7_e4961: f64 = (-1e-8);
            let assign6930_body7_e4962: f64 = if var_phi_s0_bulk_0 < assign6930_body7_e4961 { 1.0 } else { 0.0 };
            var_guard70 = assign6930_body7_e4962;
            let (assign6930_body8_e4976, assign6930_body8_e4976_d_n0, assign6930_body8_e4976_d_n2, assign6930_body8_e4976_d_n4, assign6930_body8_e4976_d_n5, assign6930_body8_e4976_d_n6, assign6930_body8_e4976_d_n8, assign6930_body8_e4976_d_n10, assign6930_body8_e4976_d_n11, assign6930_body8_e4976_d_n12,) = {
    if ((var_guard69 == 0.0) && (var_guard70 != 0.0)) {
        let assign6930_body8_e4970: f64 = (var_t3 + var_t2);
        let assign6930_body8_e4972: f64 = (assign6930_body8_e4970 - 1.0);
        let assign6930_body8_e4973: f64 = (assign6930_body8_e4972).sqrt();
        let assign6930_body8_e4974: f64 = (var_t1 * assign6930_body8_e4973);
        (assign6930_body8_e4974, ((var_t1_dn0 * assign6930_body8_e4973) + (var_t1 * ((var_t3_dn0 + var_t2_dn0) / (2.0 * assign6930_body8_e4973)))), ((var_t1_dn2 * assign6930_body8_e4973) + (var_t1 * ((var_t3_dn2 + var_t2_dn2) / (2.0 * assign6930_body8_e4973)))), ((var_t1_dn4 * assign6930_body8_e4973) + (var_t1 * ((var_t3_dn4 + var_t2_dn4) / (2.0 * assign6930_body8_e4973)))), ((var_t1_dn5 * assign6930_body8_e4973) + (var_t1 * ((var_t3_dn5 + var_t2_dn5) / (2.0 * assign6930_body8_e4973)))), ((var_t1_dn6 * assign6930_body8_e4973) + (var_t1 * ((var_t3_dn6 + var_t2_dn6) / (2.0 * assign6930_body8_e4973)))), ((var_t1_dn8 * assign6930_body8_e4973) + (var_t1 * ((var_t3_dn8 + var_t2_dn8) / (2.0 * assign6930_body8_e4973)))), ((var_t1_dn10 * assign6930_body8_e4973) + (var_t1 * ((var_t3_dn10 + var_t2_dn10) / (2.0 * assign6930_body8_e4973)))), ((var_t1_dn11 * assign6930_body8_e4973) + (var_t1 * ((var_t3_dn11 + var_t2_dn11) / (2.0 * assign6930_body8_e4973)))), ((var_t1_dn12 * assign6930_body8_e4973) + (var_t1 * ((var_t3_dn12 + var_t2_dn12) / (2.0 * assign6930_body8_e4973)))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign6930_body8_e4976;
            var_t4_dn0 = assign6930_body8_e4976_d_n0;
            var_t4_dn2 = assign6930_body8_e4976_d_n2;
            var_t4_dn4 = assign6930_body8_e4976_d_n4;
            var_t4_dn5 = assign6930_body8_e4976_d_n5;
            var_t4_dn6 = assign6930_body8_e4976_d_n6;
            var_t4_dn8 = assign6930_body8_e4976_d_n8;
            var_t4_dn10 = assign6930_body8_e4976_d_n10;
            var_t4_dn11 = assign6930_body8_e4976_d_n11;
            var_t4_dn12 = assign6930_body8_e4976_d_n12;
            let (assign6930_body9_e4990, assign6930_body9_e4990_d_n0, assign6930_body9_e4990_d_n2, assign6930_body9_e4990_d_n4, assign6930_body9_e4990_d_n5, assign6930_body9_e4990_d_n6, assign6930_body9_e4990_d_n8, assign6930_body9_e4990_d_n10, assign6930_body9_e4990_d_n11, assign6930_body9_e4990_d_n12,) = {
    if ((var_guard69 == 0.0) && (var_guard70 != 0.0)) {
        let assign6930_body9_e4983: f64 = (var_c0bulk / var_t4);
        let assign6930_body9_e4985: f64 = (-var_t3);
        let assign6930_body9_e4987: f64 = (assign6930_body9_e4985 + 1.0);
        let assign6930_body9_e4988: f64 = (assign6930_body9_e4983 * assign6930_body9_e4987);
        (assign6930_body9_e4988, (((((var_c0bulk_dn0 * var_t4) - (var_c0bulk * var_t4_dn0)) / (var_t4 * var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-var_t3_dn0))), (((((var_c0bulk_dn2 * var_t4) - (var_c0bulk * var_t4_dn2)) / (var_t4 * var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-var_t3_dn2))), (((((var_c0bulk_dn4 * var_t4) - (var_c0bulk * var_t4_dn4)) / (var_t4 * var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-var_t3_dn4))), (((((var_c0bulk_dn5 * var_t4) - (var_c0bulk * var_t4_dn5)) / (var_t4 * var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-var_t3_dn5))), (((((var_c0bulk_dn6 * var_t4) - (var_c0bulk * var_t4_dn6)) / (var_t4 * var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-var_t3_dn6))), (((((var_c0bulk_dn8 * var_t4) - (var_c0bulk * var_t4_dn8)) / (var_t4 * var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-var_t3_dn8))), (((((var_c0bulk_dn10 * var_t4) - (var_c0bulk * var_t4_dn10)) / (var_t4 * var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-var_t3_dn10))), (((((var_c0bulk_dn11 * var_t4) - (var_c0bulk * var_t4_dn11)) / (var_t4 * var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-var_t3_dn11))), (((((var_c0bulk_dn12 * var_t4) - (var_c0bulk * var_t4_dn12)) / (var_t4 * var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-var_t3_dn12))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign6930_body9_e4990;
            var_t5_dn0 = assign6930_body9_e4990_d_n0;
            var_t5_dn2 = assign6930_body9_e4990_d_n2;
            var_t5_dn4 = assign6930_body9_e4990_d_n4;
            var_t5_dn5 = assign6930_body9_e4990_d_n5;
            var_t5_dn6 = assign6930_body9_e4990_d_n6;
            var_t5_dn8 = assign6930_body9_e4990_d_n8;
            var_t5_dn10 = assign6930_body9_e4990_d_n10;
            var_t5_dn11 = assign6930_body9_e4990_d_n11;
            var_t5_dn12 = assign6930_body9_e4990_d_n12;
            let (assign6930_body10_e5006, assign6930_body10_e5006_d_n0, assign6930_body10_e5006_d_n2, assign6930_body10_e5006_d_n4, assign6930_body10_e5006_d_n5, assign6930_body10_e5006_d_n6, assign6930_body10_e5006_d_n8, assign6930_body10_e5006_d_n10, assign6930_body10_e5006_d_n11, assign6930_body10_e5006_d_n12,) = {
    if ((var_guard69 == 0.0) && (var_guard70 == 0.0)) {
        let assign6930_body10_e4998: f64 = (var_c0bulk / var_beta);
        let assign6930_body10_e4999: f64 = (assign6930_body10_e4998).sqrt();
        let assign6930_body10_e5000: f64 = (-assign6930_body10_e4999);
        let assign6930_body10_e5002: f64 = (assign6930_body10_e5000 * var_beta);
        let assign6930_body10_e5004: f64 = (assign6930_body10_e5002 * var_phi_s0_bulk_0);
        (assign6930_body10_e5004, ((((-((var_c0bulk_dn0 / var_beta) / (2.0 * assign6930_body10_e4999))) * var_beta) * var_phi_s0_bulk_0) + (assign6930_body10_e5002 * var_phi_s0_bulk_0_dn0)), ((((-((var_c0bulk_dn2 / var_beta) / (2.0 * assign6930_body10_e4999))) * var_beta) * var_phi_s0_bulk_0) + (assign6930_body10_e5002 * var_phi_s0_bulk_0_dn2)), (((((-((((var_c0bulk_dn4 * var_beta) - (var_c0bulk * var_beta_dn4)) / (var_beta * var_beta)) / (2.0 * assign6930_body10_e4999))) * var_beta) + (assign6930_body10_e5000 * var_beta_dn4)) * var_phi_s0_bulk_0) + (assign6930_body10_e5002 * var_phi_s0_bulk_0_dn4)), ((((-((var_c0bulk_dn5 / var_beta) / (2.0 * assign6930_body10_e4999))) * var_beta) * var_phi_s0_bulk_0) + (assign6930_body10_e5002 * var_phi_s0_bulk_0_dn5)), ((((-((var_c0bulk_dn6 / var_beta) / (2.0 * assign6930_body10_e4999))) * var_beta) * var_phi_s0_bulk_0) + (assign6930_body10_e5002 * var_phi_s0_bulk_0_dn6)), ((((-((var_c0bulk_dn8 / var_beta) / (2.0 * assign6930_body10_e4999))) * var_beta) * var_phi_s0_bulk_0) + (assign6930_body10_e5002 * var_phi_s0_bulk_0_dn8)), ((((-((var_c0bulk_dn10 / var_beta) / (2.0 * assign6930_body10_e4999))) * var_beta) * var_phi_s0_bulk_0) + (assign6930_body10_e5002 * var_phi_s0_bulk_0_dn10)), ((((-((var_c0bulk_dn11 / var_beta) / (2.0 * assign6930_body10_e4999))) * var_beta) * var_phi_s0_bulk_0) + (assign6930_body10_e5002 * var_phi_s0_bulk_0_dn11)), ((((-((var_c0bulk_dn12 / var_beta) / (2.0 * assign6930_body10_e4999))) * var_beta) * var_phi_s0_bulk_0) + (assign6930_body10_e5002 * var_phi_s0_bulk_0_dn12)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
            var_t4 = assign6930_body10_e5006;
            var_t4_dn0 = assign6930_body10_e5006_d_n0;
            var_t4_dn2 = assign6930_body10_e5006_d_n2;
            var_t4_dn4 = assign6930_body10_e5006_d_n4;
            var_t4_dn5 = assign6930_body10_e5006_d_n5;
            var_t4_dn6 = assign6930_body10_e5006_d_n6;
            var_t4_dn8 = assign6930_body10_e5006_d_n8;
            var_t4_dn10 = assign6930_body10_e5006_d_n10;
            var_t4_dn11 = assign6930_body10_e5006_d_n11;
            var_t4_dn12 = assign6930_body10_e5006_d_n12;
            let (assign6930_body11_e5018, assign6930_body11_e5018_d_n0, assign6930_body11_e5018_d_n2, assign6930_body11_e5018_d_n4, assign6930_body11_e5018_d_n5, assign6930_body11_e5018_d_n6, assign6930_body11_e5018_d_n8, assign6930_body11_e5018_d_n10, assign6930_body11_e5018_d_n11, assign6930_body11_e5018_d_n12,) = {
    if ((var_guard69 == 0.0) && (var_guard70 == 0.0)) {
        let assign6930_body11_e5014: f64 = (var_c0bulk * var_beta);
        let assign6930_body11_e5015: f64 = (assign6930_body11_e5014).sqrt();
        let assign6930_body11_e5016: f64 = (-assign6930_body11_e5015);
        (assign6930_body11_e5016, (-((var_c0bulk_dn0 * var_beta) / (2.0 * assign6930_body11_e5015))), (-((var_c0bulk_dn2 * var_beta) / (2.0 * assign6930_body11_e5015))), (-(((var_c0bulk_dn4 * var_beta) + (var_c0bulk * var_beta_dn4)) / (2.0 * assign6930_body11_e5015))), (-((var_c0bulk_dn5 * var_beta) / (2.0 * assign6930_body11_e5015))), (-((var_c0bulk_dn6 * var_beta) / (2.0 * assign6930_body11_e5015))), (-((var_c0bulk_dn8 * var_beta) / (2.0 * assign6930_body11_e5015))), (-((var_c0bulk_dn10 * var_beta) / (2.0 * assign6930_body11_e5015))), (-((var_c0bulk_dn11 * var_beta) / (2.0 * assign6930_body11_e5015))), (-((var_c0bulk_dn12 * var_beta) / (2.0 * assign6930_body11_e5015))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
            var_t5 = assign6930_body11_e5018;
            var_t5_dn0 = assign6930_body11_e5018_d_n0;
            var_t5_dn2 = assign6930_body11_e5018_d_n2;
            var_t5_dn4 = assign6930_body11_e5018_d_n4;
            var_t5_dn5 = assign6930_body11_e5018_d_n5;
            var_t5_dn6 = assign6930_body11_e5018_d_n6;
            var_t5_dn8 = assign6930_body11_e5018_d_n8;
            var_t5_dn10 = assign6930_body11_e5018_d_n10;
            var_t5_dn11 = assign6930_body11_e5018_d_n11;
            var_t5_dn12 = assign6930_body11_e5018_d_n12;
            let assign6930_body12_e5021: f64 = (var_t4 * var_t4);
            let assign6930_body12_e5024: f64 = (4.0 * 1e-6);
            let assign6930_body12_e5026: f64 = (assign6930_body12_e5024 * 1e-6);
            let assign6930_body12_e5027: f64 = (assign6930_body12_e5021 + assign6930_body12_e5026);
            let assign6930_body12_e5028: f64 = (assign6930_body12_e5027).sqrt();
            var_tmf2 = assign6930_body12_e5028;
            var_tmf2_dn0 = (((var_t4_dn0 * var_t4) + (var_t4 * var_t4_dn0)) / (2.0 * assign6930_body12_e5028));
            var_tmf2_dn2 = (((var_t4_dn2 * var_t4) + (var_t4 * var_t4_dn2)) / (2.0 * assign6930_body12_e5028));
            var_tmf2_dn4 = (((var_t4_dn4 * var_t4) + (var_t4 * var_t4_dn4)) / (2.0 * assign6930_body12_e5028));
            var_tmf2_dn5 = (((var_t4_dn5 * var_t4) + (var_t4 * var_t4_dn5)) / (2.0 * assign6930_body12_e5028));
            var_tmf2_dn6 = (((var_t4_dn6 * var_t4) + (var_t4 * var_t4_dn6)) / (2.0 * assign6930_body12_e5028));
            var_tmf2_dn8 = (((var_t4_dn8 * var_t4) + (var_t4 * var_t4_dn8)) / (2.0 * assign6930_body12_e5028));
            var_tmf2_dn10 = (((var_t4_dn10 * var_t4) + (var_t4 * var_t4_dn10)) / (2.0 * assign6930_body12_e5028));
            var_tmf2_dn11 = (((var_t4_dn11 * var_t4) + (var_t4 * var_t4_dn11)) / (2.0 * assign6930_body12_e5028));
            var_tmf2_dn12 = (((var_t4_dn12 * var_t4) + (var_t4 * var_t4_dn12)) / (2.0 * assign6930_body12_e5028));
            let assign6930_body13_e5033: f64 = (var_t4 / var_tmf2);
            let assign6930_body13_e5034: f64 = (1.0 + assign6930_body13_e5033);
            let assign6930_body13_e5035: f64 = (0.5 * assign6930_body13_e5034);
            var_t7 = assign6930_body13_e5035;
            var_t7_dn0 = (0.5 * (((var_t4_dn0 * var_tmf2) - (var_t4 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2)));
            var_t7_dn2 = (0.5 * (((var_t4_dn2 * var_tmf2) - (var_t4 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2)));
            var_t7_dn4 = (0.5 * (((var_t4_dn4 * var_tmf2) - (var_t4 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2)));
            var_t7_dn5 = (0.5 * (((var_t4_dn5 * var_tmf2) - (var_t4 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2)));
            var_t7_dn6 = (0.5 * (((var_t4_dn6 * var_tmf2) - (var_t4 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2)));
            var_t7_dn8 = (0.5 * (((var_t4_dn8 * var_tmf2) - (var_t4 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2)));
            var_t7_dn10 = (0.5 * (((var_t4_dn10 * var_tmf2) - (var_t4 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2)));
            var_t7_dn11 = (0.5 * (((var_t4_dn11 * var_tmf2) - (var_t4 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2)));
            var_t7_dn12 = (0.5 * (((var_t4_dn12 * var_tmf2) - (var_t4 * var_tmf2_dn12)) / (var_tmf2 * var_tmf2)));
            let assign6930_body14_e5039: f64 = (var_t4 + var_tmf2);
            let assign6930_body14_e5040: f64 = (0.5 * assign6930_body14_e5039);
            let assign6930_body14_e5043: f64 = (1e-10 * 1e-6);
            let assign6930_body14_e5044: f64 = (assign6930_body14_e5040 + assign6930_body14_e5043);
            var_t6 = assign6930_body14_e5044;
            var_t6_dn0 = (0.5 * (var_t4_dn0 + var_tmf2_dn0));
            var_t6_dn2 = (0.5 * (var_t4_dn2 + var_tmf2_dn2));
            var_t6_dn4 = (0.5 * (var_t4_dn4 + var_tmf2_dn4));
            var_t6_dn5 = (0.5 * (var_t4_dn5 + var_tmf2_dn5));
            var_t6_dn6 = (0.5 * (var_t4_dn6 + var_tmf2_dn6));
            var_t6_dn8 = (0.5 * (var_t4_dn8 + var_tmf2_dn8));
            var_t6_dn10 = (0.5 * (var_t4_dn10 + var_tmf2_dn10));
            var_t6_dn11 = (0.5 * (var_t4_dn11 + var_tmf2_dn11));
            var_t6_dn12 = (0.5 * (var_t4_dn12 + var_tmf2_dn12));
            let assign6930_body15_e5047: f64 = if var_t6 < 0.0 { 1.0 } else { 0.0 };
            var_guard71 = assign6930_body15_e5047;
            let (assign6930_body16_e5051, assign6930_body16_e5051_d_n0, assign6930_body16_e5051_d_n2, assign6930_body16_e5051_d_n4, assign6930_body16_e5051_d_n5, assign6930_body16_e5051_d_n6, assign6930_body16_e5051_d_n8, assign6930_body16_e5051_d_n10, assign6930_body16_e5051_d_n11, assign6930_body16_e5051_d_n12,) = {
    if (var_guard71 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn8, var_t6_dn10, var_t6_dn11, var_t6_dn12,)
    }
};
            var_t6 = assign6930_body16_e5051;
            var_t6_dn0 = assign6930_body16_e5051_d_n0;
            var_t6_dn2 = assign6930_body16_e5051_d_n2;
            var_t6_dn4 = assign6930_body16_e5051_d_n4;
            var_t6_dn5 = assign6930_body16_e5051_d_n5;
            var_t6_dn6 = assign6930_body16_e5051_d_n6;
            var_t6_dn8 = assign6930_body16_e5051_d_n8;
            var_t6_dn10 = assign6930_body16_e5051_d_n10;
            var_t6_dn11 = assign6930_body16_e5051_d_n11;
            var_t6_dn12 = assign6930_body16_e5051_d_n12;
            let (assign6930_body17_e5055, assign6930_body17_e5055_d_n0, assign6930_body17_e5055_d_n2, assign6930_body17_e5055_d_n4, assign6930_body17_e5055_d_n5, assign6930_body17_e5055_d_n6, assign6930_body17_e5055_d_n8, assign6930_body17_e5055_d_n10, assign6930_body17_e5055_d_n11, assign6930_body17_e5055_d_n12,) = {
    if (var_guard71 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn8, var_t7_dn10, var_t7_dn11, var_t7_dn12,)
    }
};
            var_t7 = assign6930_body17_e5055;
            var_t7_dn0 = assign6930_body17_e5055_d_n0;
            var_t7_dn2 = assign6930_body17_e5055_d_n2;
            var_t7_dn4 = assign6930_body17_e5055_d_n4;
            var_t7_dn5 = assign6930_body17_e5055_d_n5;
            var_t7_dn6 = assign6930_body17_e5055_d_n6;
            var_t7_dn8 = assign6930_body17_e5055_d_n8;
            var_t7_dn10 = assign6930_body17_e5055_d_n10;
            var_t7_dn11 = assign6930_body17_e5055_d_n11;
            var_t7_dn12 = assign6930_body17_e5055_d_n12;
            let assign6930_body18_e5057: f64 = (-var_q_fd_soi);
            let assign6930_body18_e5059: f64 = (assign6930_body18_e5057 - var_t6);
            let assign6930_body18_e5061: f64 = (assign6930_body18_e5059 - 1e-9);
            var_tmf1 = assign6930_body18_e5061;
            var_tmf1_dn0 = ((-var_q_fd_soi_dn0) - var_t6_dn0);
            var_tmf1_dn2 = ((-var_q_fd_soi_dn2) - var_t6_dn2);
            var_tmf1_dn4 = ((-var_q_fd_soi_dn4) - var_t6_dn4);
            var_tmf1_dn5 = ((-var_q_fd_soi_dn5) - var_t6_dn5);
            var_tmf1_dn6 = ((-var_q_fd_soi_dn6) - var_t6_dn6);
            var_tmf1_dn8 = ((-var_q_fd_soi_dn8) - var_t6_dn8);
            var_tmf1_dn10 = ((-var_q_fd_soi_dn10) - var_t6_dn10);
            var_tmf1_dn11 = ((-var_q_fd_soi_dn11) - var_t6_dn11);
            var_tmf1_dn12 = ((-var_q_fd_soi_dn12) - var_t6_dn12);
            let assign6930_body19_e5064: f64 = (-var_q_fd_soi);
            let assign6930_body19_e5065: f64 = (4.0 * assign6930_body19_e5064);
            let assign6930_body19_e5067: f64 = (assign6930_body19_e5065 * 1e-9);
            var_tmf2 = assign6930_body19_e5067;
            var_tmf2_dn0 = ((4.0 * (-var_q_fd_soi_dn0)) * 1e-9);
            var_tmf2_dn2 = ((4.0 * (-var_q_fd_soi_dn2)) * 1e-9);
            var_tmf2_dn4 = ((4.0 * (-var_q_fd_soi_dn4)) * 1e-9);
            var_tmf2_dn5 = ((4.0 * (-var_q_fd_soi_dn5)) * 1e-9);
            var_tmf2_dn6 = ((4.0 * (-var_q_fd_soi_dn6)) * 1e-9);
            var_tmf2_dn8 = ((4.0 * (-var_q_fd_soi_dn8)) * 1e-9);
            var_tmf2_dn10 = ((4.0 * (-var_q_fd_soi_dn10)) * 1e-9);
            var_tmf2_dn11 = ((4.0 * (-var_q_fd_soi_dn11)) * 1e-9);
            var_tmf2_dn12 = ((4.0 * (-var_q_fd_soi_dn12)) * 1e-9);
            let (assign6930_body20_e5074, assign6930_body20_e5074_d_n0, assign6930_body20_e5074_d_n2, assign6930_body20_e5074_d_n4, assign6930_body20_e5074_d_n5, assign6930_body20_e5074_d_n6, assign6930_body20_e5074_d_n8, assign6930_body20_e5074_d_n10, assign6930_body20_e5074_d_n11, assign6930_body20_e5074_d_n12,) = {
    if (var_tmf2 > 0.0) {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn8, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12,)
    } else {
        let assign6930_body20_e5073: f64 = (-var_tmf2);
        (assign6930_body20_e5073, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn8), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12),)
    }
};
            var_tmf2 = assign6930_body20_e5074;
            var_tmf2_dn0 = assign6930_body20_e5074_d_n0;
            var_tmf2_dn2 = assign6930_body20_e5074_d_n2;
            var_tmf2_dn4 = assign6930_body20_e5074_d_n4;
            var_tmf2_dn5 = assign6930_body20_e5074_d_n5;
            var_tmf2_dn6 = assign6930_body20_e5074_d_n6;
            var_tmf2_dn8 = assign6930_body20_e5074_d_n8;
            var_tmf2_dn10 = assign6930_body20_e5074_d_n10;
            var_tmf2_dn11 = assign6930_body20_e5074_d_n11;
            var_tmf2_dn12 = assign6930_body20_e5074_d_n12;
            let assign6930_body21_e5077: f64 = (var_tmf1 * var_tmf1);
            let assign6930_body21_e5079: f64 = (assign6930_body21_e5077 + var_tmf2);
            let assign6930_body21_e5080: f64 = (assign6930_body21_e5079).sqrt();
            var_tmf2 = assign6930_body21_e5080;
            var_tmf2_dn0 = ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign6930_body21_e5080));
            var_tmf2_dn2 = ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign6930_body21_e5080));
            var_tmf2_dn4 = ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign6930_body21_e5080));
            var_tmf2_dn5 = ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign6930_body21_e5080));
            var_tmf2_dn6 = ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign6930_body21_e5080));
            var_tmf2_dn8 = ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign6930_body21_e5080));
            var_tmf2_dn10 = ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign6930_body21_e5080));
            var_tmf2_dn11 = ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign6930_body21_e5080));
            var_tmf2_dn12 = ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign6930_body21_e5080));
            let assign6930_body22_e5085: f64 = (var_tmf1 / var_tmf2);
            let assign6930_body22_e5086: f64 = (1.0 + assign6930_body22_e5085);
            let assign6930_body22_e5087: f64 = (0.5 * assign6930_body22_e5086);
            var_t8 = assign6930_body22_e5087;
            var_t8_dn0 = (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2)));
            var_t8_dn2 = (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2)));
            var_t8_dn4 = (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2)));
            var_t8_dn5 = (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2)));
            var_t8_dn6 = (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2)));
            var_t8_dn8 = (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2)));
            var_t8_dn10 = (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2)));
            var_t8_dn11 = (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2)));
            var_t8_dn12 = (0.5 * (((var_tmf1_dn12 * var_tmf2) - (var_tmf1 * var_tmf2_dn12)) / (var_tmf2 * var_tmf2)));
            let assign6930_body23_e5089: f64 = (-var_q_fd_soi);
            let assign6930_body23_e5093: f64 = (var_tmf1 + var_tmf2);
            let assign6930_body23_e5094: f64 = (0.5 * assign6930_body23_e5093);
            let assign6930_body23_e5095: f64 = (assign6930_body23_e5089 - assign6930_body23_e5094);
            var_t6 = assign6930_body23_e5095;
            var_t6_dn0 = ((-var_q_fd_soi_dn0) - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)));
            var_t6_dn2 = ((-var_q_fd_soi_dn2) - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)));
            var_t6_dn4 = ((-var_q_fd_soi_dn4) - (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)));
            var_t6_dn5 = ((-var_q_fd_soi_dn5) - (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)));
            var_t6_dn6 = ((-var_q_fd_soi_dn6) - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)));
            var_t6_dn8 = ((-var_q_fd_soi_dn8) - (0.5 * (var_tmf1_dn8 + var_tmf2_dn8)));
            var_t6_dn10 = ((-var_q_fd_soi_dn10) - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)));
            var_t6_dn11 = ((-var_q_fd_soi_dn11) - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11)));
            var_t6_dn12 = ((-var_q_fd_soi_dn12) - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12)));
            let assign6930_body24_e5099: f64 = (var_t5 * var_t8);
            let assign6930_body24_e5100: f64 = (var_t7 * assign6930_body24_e5099);
            var_t7 = assign6930_body24_e5100;
            var_t7_dn0 = ((var_t7_dn0 * assign6930_body24_e5099) + (var_t7 * ((var_t5_dn0 * var_t8) + (var_t5 * var_t8_dn0))));
            var_t7_dn2 = ((var_t7_dn2 * assign6930_body24_e5099) + (var_t7 * ((var_t5_dn2 * var_t8) + (var_t5 * var_t8_dn2))));
            var_t7_dn4 = ((var_t7_dn4 * assign6930_body24_e5099) + (var_t7 * ((var_t5_dn4 * var_t8) + (var_t5 * var_t8_dn4))));
            var_t7_dn5 = ((var_t7_dn5 * assign6930_body24_e5099) + (var_t7 * ((var_t5_dn5 * var_t8) + (var_t5 * var_t8_dn5))));
            var_t7_dn6 = ((var_t7_dn6 * assign6930_body24_e5099) + (var_t7 * ((var_t5_dn6 * var_t8) + (var_t5 * var_t8_dn6))));
            var_t7_dn8 = ((var_t7_dn8 * assign6930_body24_e5099) + (var_t7 * ((var_t5_dn8 * var_t8) + (var_t5 * var_t8_dn8))));
            var_t7_dn10 = ((var_t7_dn10 * assign6930_body24_e5099) + (var_t7 * ((var_t5_dn10 * var_t8) + (var_t5 * var_t8_dn10))));
            var_t7_dn11 = ((var_t7_dn11 * assign6930_body24_e5099) + (var_t7 * ((var_t5_dn11 * var_t8) + (var_t5 * var_t8_dn11))));
            var_t7_dn12 = ((var_t7_dn12 * assign6930_body24_e5099) + (var_t7 * ((var_t5_dn12 * var_t8) + (var_t5 * var_t8_dn12))));
            let assign6930_body25_e5103: f64 = (var_t6 * var_t6);
            let assign6930_body25_e5105: f64 = (assign6930_body25_e5103 / 2.0);
            let assign6930_body25_e5107: f64 = (assign6930_body25_e5105 / 1.034943e-10);
            let assign6930_body25_e5109: f64 = (assign6930_body25_e5107 / 1.6021918e-19);
            let assign6930_body25_e5111: f64 = (assign6930_body25_e5109 / var_uc_nsubs);
            var_phi_b_dep0 = assign6930_body25_e5111;
            var_phi_b_dep0_dn0 = ((((((((var_t6_dn0 * var_t6) + (var_t6 * var_t6_dn0)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign6930_body25_e5109 * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs));
            var_phi_b_dep0_dn2 = ((((((((var_t6_dn2 * var_t6) + (var_t6 * var_t6_dn2)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign6930_body25_e5109 * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs));
            var_phi_b_dep0_dn4 = ((((((((var_t6_dn4 * var_t6) + (var_t6 * var_t6_dn4)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign6930_body25_e5109 * var_uc_nsubs_dn4)) / (var_uc_nsubs * var_uc_nsubs));
            var_phi_b_dep0_dn5 = ((((((((var_t6_dn5 * var_t6) + (var_t6 * var_t6_dn5)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign6930_body25_e5109 * var_uc_nsubs_dn5)) / (var_uc_nsubs * var_uc_nsubs));
            var_phi_b_dep0_dn6 = ((((((((var_t6_dn6 * var_t6) + (var_t6 * var_t6_dn6)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign6930_body25_e5109 * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs));
            var_phi_b_dep0_dn8 = ((((((((var_t6_dn8 * var_t6) + (var_t6 * var_t6_dn8)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign6930_body25_e5109 * var_uc_nsubs_dn8)) / (var_uc_nsubs * var_uc_nsubs));
            var_phi_b_dep0_dn10 = ((((((((var_t6_dn10 * var_t6) + (var_t6 * var_t6_dn10)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign6930_body25_e5109 * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs));
            var_phi_b_dep0_dn11 = ((((((((var_t6_dn11 * var_t6) + (var_t6 * var_t6_dn11)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign6930_body25_e5109 * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs));
            var_phi_b_dep0_dn12 = ((((((((var_t6_dn12 * var_t6) + (var_t6 * var_t6_dn12)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * var_uc_nsubs) - (assign6930_body25_e5109 * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs));
            let assign6930_body26_e5114: f64 = (2.0 * var_phi_b_dep0);
            let assign6930_body26_e5116: f64 = (assign6930_body26_e5114 * var_t7);
            let assign6930_body26_e5118: f64 = (assign6930_body26_e5116 / var_t6);
            var_phi_b_dep0_dpsb = assign6930_body26_e5118;
            var_phi_b_dep0_dpsb_dn0 = ((((((2.0 * var_phi_b_dep0_dn0) * var_t7) + (assign6930_body26_e5114 * var_t7_dn0)) * var_t6) - (assign6930_body26_e5116 * var_t6_dn0)) / (var_t6 * var_t6));
            var_phi_b_dep0_dpsb_dn2 = ((((((2.0 * var_phi_b_dep0_dn2) * var_t7) + (assign6930_body26_e5114 * var_t7_dn2)) * var_t6) - (assign6930_body26_e5116 * var_t6_dn2)) / (var_t6 * var_t6));
            var_phi_b_dep0_dpsb_dn4 = ((((((2.0 * var_phi_b_dep0_dn4) * var_t7) + (assign6930_body26_e5114 * var_t7_dn4)) * var_t6) - (assign6930_body26_e5116 * var_t6_dn4)) / (var_t6 * var_t6));
            var_phi_b_dep0_dpsb_dn5 = ((((((2.0 * var_phi_b_dep0_dn5) * var_t7) + (assign6930_body26_e5114 * var_t7_dn5)) * var_t6) - (assign6930_body26_e5116 * var_t6_dn5)) / (var_t6 * var_t6));
            var_phi_b_dep0_dpsb_dn6 = ((((((2.0 * var_phi_b_dep0_dn6) * var_t7) + (assign6930_body26_e5114 * var_t7_dn6)) * var_t6) - (assign6930_body26_e5116 * var_t6_dn6)) / (var_t6 * var_t6));
            var_phi_b_dep0_dpsb_dn8 = ((((((2.0 * var_phi_b_dep0_dn8) * var_t7) + (assign6930_body26_e5114 * var_t7_dn8)) * var_t6) - (assign6930_body26_e5116 * var_t6_dn8)) / (var_t6 * var_t6));
            var_phi_b_dep0_dpsb_dn10 = ((((((2.0 * var_phi_b_dep0_dn10) * var_t7) + (assign6930_body26_e5114 * var_t7_dn10)) * var_t6) - (assign6930_body26_e5116 * var_t6_dn10)) / (var_t6 * var_t6));
            var_phi_b_dep0_dpsb_dn11 = ((((((2.0 * var_phi_b_dep0_dn11) * var_t7) + (assign6930_body26_e5114 * var_t7_dn11)) * var_t6) - (assign6930_body26_e5116 * var_t6_dn11)) / (var_t6 * var_t6));
            var_phi_b_dep0_dpsb_dn12 = ((((((2.0 * var_phi_b_dep0_dn12) * var_t7) + (assign6930_body26_e5114 * var_t7_dn12)) * var_t6) - (assign6930_body26_e5116 * var_t6_dn12)) / (var_t6 * var_t6));
            let assign6930_body27_e5121: f64 = (-var_phi_s0_bulk_0);
            let assign6930_body27_e5124: f64 = (var_t4 / var_c_box);
            let assign6930_body27_e5125: f64 = (assign6930_body27_e5121 + assign6930_body27_e5124);
            let assign6930_body27_e5127: f64 = (assign6930_body27_e5125 - var_vbsbiz);
            let assign6930_body27_e5129: f64 = (assign6930_body27_e5127 + var_phi_b_dep0);
            let assign6930_body27_e5131: f64 = (-1.0);
            let assign6930_body27_e5134: f64 = (var_t5 / var_c_box);
            let assign6930_body27_e5135: f64 = (assign6930_body27_e5131 + assign6930_body27_e5134);
            let assign6930_body27_e5137: f64 = (assign6930_body27_e5135 + var_phi_b_dep0_dpsb);
            let assign6930_body27_e5138: f64 = (assign6930_body27_e5129 / assign6930_body27_e5137);
            let assign6930_body27_e5139: f64 = (var_phi_s0_bulk_0 - assign6930_body27_e5138);
            var_t6 = assign6930_body27_e5139;
            var_t6_dn0 = (var_phi_s0_bulk_0_dn0 - (((((((-var_phi_s0_bulk_0_dn0) + (var_t4_dn0 / var_c_box)) - var_vbsbiz_dn0) + var_phi_b_dep0_dn0) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((var_t5_dn0 / var_c_box) + var_phi_b_dep0_dpsb_dn0))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            var_t6_dn2 = (var_phi_s0_bulk_0_dn2 - (((((((-var_phi_s0_bulk_0_dn2) + (var_t4_dn2 / var_c_box)) - var_vbsbiz_dn2) + var_phi_b_dep0_dn2) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((var_t5_dn2 / var_c_box) + var_phi_b_dep0_dpsb_dn2))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            var_t6_dn4 = (var_phi_s0_bulk_0_dn4 - (((((((-var_phi_s0_bulk_0_dn4) + (var_t4_dn4 / var_c_box)) - var_vbsbiz_dn4) + var_phi_b_dep0_dn4) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((var_t5_dn4 / var_c_box) + var_phi_b_dep0_dpsb_dn4))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            var_t6_dn5 = (var_phi_s0_bulk_0_dn5 - (((((((-var_phi_s0_bulk_0_dn5) + (var_t4_dn5 / var_c_box)) - var_vbsbiz_dn5) + var_phi_b_dep0_dn5) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((var_t5_dn5 / var_c_box) + var_phi_b_dep0_dpsb_dn5))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            var_t6_dn6 = (var_phi_s0_bulk_0_dn6 - (((((((-var_phi_s0_bulk_0_dn6) + (var_t4_dn6 / var_c_box)) - var_vbsbiz_dn6) + var_phi_b_dep0_dn6) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((var_t5_dn6 / var_c_box) + var_phi_b_dep0_dpsb_dn6))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            var_t6_dn8 = (var_phi_s0_bulk_0_dn8 - (((((((-var_phi_s0_bulk_0_dn8) + (var_t4_dn8 / var_c_box)) - var_vbsbiz_dn8) + var_phi_b_dep0_dn8) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((var_t5_dn8 / var_c_box) + var_phi_b_dep0_dpsb_dn8))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            var_t6_dn10 = (var_phi_s0_bulk_0_dn10 - (((((((-var_phi_s0_bulk_0_dn10) + (var_t4_dn10 / var_c_box)) - var_vbsbiz_dn10) + var_phi_b_dep0_dn10) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((var_t5_dn10 / var_c_box) + var_phi_b_dep0_dpsb_dn10))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            var_t6_dn11 = (var_phi_s0_bulk_0_dn11 - (((((((-var_phi_s0_bulk_0_dn11) + (var_t4_dn11 / var_c_box)) - var_vbsbiz_dn11) + var_phi_b_dep0_dn11) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((var_t5_dn11 / var_c_box) + var_phi_b_dep0_dpsb_dn11))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            var_t6_dn12 = (var_phi_s0_bulk_0_dn12 - (((((((-var_phi_s0_bulk_0_dn12) + (var_t4_dn12 / var_c_box)) - var_vbsbiz_dn12) + var_phi_b_dep0_dn12) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((var_t5_dn12 / var_c_box) + var_phi_b_dep0_dpsb_dn12))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            let assign6930_body28_e5142: f64 = (var_t6 - var_phi_s0_bulk_0);
            let assign6930_body28_e5143: f64 = (assign6930_body28_e5142).abs();
            let assign6930_body28_e5145: f64 = if assign6930_body28_e5143 < 0.001 { 1.0 } else { 0.0 };
            var_guard72 = assign6930_body28_e5145;
            let (assign6930_body29_e5149,) = {
    if (var_guard72 != 0.0) {
        (var_lp_s0_max,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign6930_body29_e5149;
            var_phi_s0_bulk_0 = var_t6;
            var_phi_s0_bulk_0_dn0 = var_t6_dn0;
            var_phi_s0_bulk_0_dn2 = var_t6_dn2;
            var_phi_s0_bulk_0_dn4 = var_t6_dn4;
            var_phi_s0_bulk_0_dn5 = var_t6_dn5;
            var_phi_s0_bulk_0_dn6 = var_t6_dn6;
            var_phi_s0_bulk_0_dn8 = var_t6_dn8;
            var_phi_s0_bulk_0_dn10 = var_t6_dn10;
            var_phi_s0_bulk_0_dn11 = var_t6_dn11;
            var_phi_s0_bulk_0_dn12 = var_t6_dn12;
            var_q_s0_bulk_0 = var_t4;
            let assign6930_body32_e5154: f64 = (var_lp_s0 + 1.0);
            var_lp_s0 = assign6930_body32_e5154;
        }

        var_phi_b_dep = var_phi_b_dep0;
        var_phi_b_dep_dn0 = var_phi_b_dep0_dn0;
        var_phi_b_dep_dn2 = var_phi_b_dep0_dn2;
        var_phi_b_dep_dn4 = var_phi_b_dep0_dn4;
        var_phi_b_dep_dn5 = var_phi_b_dep0_dn5;
        var_phi_b_dep_dn6 = var_phi_b_dep0_dn6;
        var_phi_b_dep_dn8 = var_phi_b_dep0_dn8;
        var_phi_b_dep_dn10 = var_phi_b_dep0_dn10;
        var_phi_b_dep_dn11 = var_phi_b_dep0_dn11;
        var_phi_b_dep_dn12 = var_phi_b_dep0_dn12;

        let assign6950_e5158: f64 = (2.0 * 1.034943e-10);
        let assign6950_e5160: f64 = (assign6950_e5158 / 1.6021918e-19);
        let assign6950_e5162: f64 = (assign6950_e5160 * var_phi_b_dep);
        let assign6950_e5164: f64 = (assign6950_e5162 / var_uc_nsubs);
        let assign6950_e5165: f64 = (assign6950_e5164).sqrt();
        var_t1 = assign6950_e5165;
        var_t1_dn0 = (((((assign6950_e5160 * var_phi_b_dep_dn0) * var_uc_nsubs) - (assign6950_e5162 * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign6950_e5165));
        var_t1_dn2 = (((((assign6950_e5160 * var_phi_b_dep_dn2) * var_uc_nsubs) - (assign6950_e5162 * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign6950_e5165));
        var_t1_dn4 = (((((assign6950_e5160 * var_phi_b_dep_dn4) * var_uc_nsubs) - (assign6950_e5162 * var_uc_nsubs_dn4)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign6950_e5165));
        var_t1_dn5 = (((((assign6950_e5160 * var_phi_b_dep_dn5) * var_uc_nsubs) - (assign6950_e5162 * var_uc_nsubs_dn5)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign6950_e5165));
        var_t1_dn6 = (((((assign6950_e5160 * var_phi_b_dep_dn6) * var_uc_nsubs) - (assign6950_e5162 * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign6950_e5165));
        var_t1_dn8 = (((((assign6950_e5160 * var_phi_b_dep_dn8) * var_uc_nsubs) - (assign6950_e5162 * var_uc_nsubs_dn8)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign6950_e5165));
        var_t1_dn10 = (((((assign6950_e5160 * var_phi_b_dep_dn10) * var_uc_nsubs) - (assign6950_e5162 * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign6950_e5165));
        var_t1_dn11 = (((((assign6950_e5160 * var_phi_b_dep_dn11) * var_uc_nsubs) - (assign6950_e5162 * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign6950_e5165));
        var_t1_dn12 = (((((assign6950_e5160 * var_phi_b_dep_dn12) * var_uc_nsubs) - (assign6950_e5162 * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)) / (2.0 * assign6950_e5165));

        let assign6960_e5169: f64 = (0.99 * p.p227);
        let assign6960_e5170: f64 = if var_t1 > assign6960_e5169 { 1.0 } else { 0.0 };
        var_guard73 = assign6960_e5170;

        let (assign6970_e5176, assign6970_e5176_d_n0, assign6970_e5176_d_n2, assign6970_e5176_d_n4, assign6970_e5176_d_n5, assign6970_e5176_d_n6, assign6970_e5176_d_n8, assign6970_e5176_d_n10, assign6970_e5176_d_n11, assign6970_e5176_d_n12,) = {
    if (var_guard73 != 0.0) {
        let assign6970_e5174: f64 = (1.0 / var_c_fox);
        (assign6970_e5174, (-(var_c_fox_dn0 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn2 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn4 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn5 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn6 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn8 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn10 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn11 / (var_c_fox * var_c_fox))), (-(var_c_fox_dn12 / (var_c_fox * var_c_fox))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn8, var_t0_dn10, var_t0_dn11, var_t0_dn12,)
    }
};
        var_t0 = assign6970_e5176;
        var_t0_dn0 = assign6970_e5176_d_n0;
        var_t0_dn2 = assign6970_e5176_d_n2;
        var_t0_dn4 = assign6970_e5176_d_n4;
        var_t0_dn5 = assign6970_e5176_d_n5;
        var_t0_dn6 = assign6970_e5176_d_n6;
        var_t0_dn8 = assign6970_e5176_d_n8;
        var_t0_dn10 = assign6970_e5176_d_n10;
        var_t0_dn11 = assign6970_e5176_d_n11;
        var_t0_dn12 = assign6970_e5176_d_n12;

        *var_guard69_slot = var_guard69;
        *var_guard70_slot = var_guard70;
        *var_guard71_slot = var_guard71;
        *var_guard72_slot = var_guard72;
        *var_guard73_slot = var_guard73;
        *var_lp_s0_slot = var_lp_s0;
        *var_phi_b_dep_slot = var_phi_b_dep;
        *var_phi_b_dep0_slot = var_phi_b_dep0;
        *var_phi_b_dep0_dn0_slot = var_phi_b_dep0_dn0;
        *var_phi_b_dep0_dn10_slot = var_phi_b_dep0_dn10;
        *var_phi_b_dep0_dn11_slot = var_phi_b_dep0_dn11;
        *var_phi_b_dep0_dn12_slot = var_phi_b_dep0_dn12;
        *var_phi_b_dep0_dn2_slot = var_phi_b_dep0_dn2;
        *var_phi_b_dep0_dn4_slot = var_phi_b_dep0_dn4;
        *var_phi_b_dep0_dn5_slot = var_phi_b_dep0_dn5;
        *var_phi_b_dep0_dn6_slot = var_phi_b_dep0_dn6;
        *var_phi_b_dep0_dn8_slot = var_phi_b_dep0_dn8;
        *var_phi_b_dep0_dpsb_slot = var_phi_b_dep0_dpsb;
        *var_phi_b_dep0_dpsb_dn0_slot = var_phi_b_dep0_dpsb_dn0;
        *var_phi_b_dep0_dpsb_dn10_slot = var_phi_b_dep0_dpsb_dn10;
        *var_phi_b_dep0_dpsb_dn11_slot = var_phi_b_dep0_dpsb_dn11;
        *var_phi_b_dep0_dpsb_dn12_slot = var_phi_b_dep0_dpsb_dn12;
        *var_phi_b_dep0_dpsb_dn2_slot = var_phi_b_dep0_dpsb_dn2;
        *var_phi_b_dep0_dpsb_dn4_slot = var_phi_b_dep0_dpsb_dn4;
        *var_phi_b_dep0_dpsb_dn5_slot = var_phi_b_dep0_dpsb_dn5;
        *var_phi_b_dep0_dpsb_dn6_slot = var_phi_b_dep0_dpsb_dn6;
        *var_phi_b_dep0_dpsb_dn8_slot = var_phi_b_dep0_dpsb_dn8;
        *var_phi_b_dep_dn0_slot = var_phi_b_dep_dn0;
        *var_phi_b_dep_dn10_slot = var_phi_b_dep_dn10;
        *var_phi_b_dep_dn11_slot = var_phi_b_dep_dn11;
        *var_phi_b_dep_dn12_slot = var_phi_b_dep_dn12;
        *var_phi_b_dep_dn2_slot = var_phi_b_dep_dn2;
        *var_phi_b_dep_dn4_slot = var_phi_b_dep_dn4;
        *var_phi_b_dep_dn5_slot = var_phi_b_dep_dn5;
        *var_phi_b_dep_dn6_slot = var_phi_b_dep_dn6;
        *var_phi_b_dep_dn8_slot = var_phi_b_dep_dn8;
        *var_phi_s0_bulk_0_slot = var_phi_s0_bulk_0;
        *var_phi_s0_bulk_0_dn0_slot = var_phi_s0_bulk_0_dn0;
        *var_phi_s0_bulk_0_dn10_slot = var_phi_s0_bulk_0_dn10;
        *var_phi_s0_bulk_0_dn11_slot = var_phi_s0_bulk_0_dn11;
        *var_phi_s0_bulk_0_dn12_slot = var_phi_s0_bulk_0_dn12;
        *var_phi_s0_bulk_0_dn2_slot = var_phi_s0_bulk_0_dn2;
        *var_phi_s0_bulk_0_dn4_slot = var_phi_s0_bulk_0_dn4;
        *var_phi_s0_bulk_0_dn5_slot = var_phi_s0_bulk_0_dn5;
        *var_phi_s0_bulk_0_dn6_slot = var_phi_s0_bulk_0_dn6;
        *var_phi_s0_bulk_0_dn8_slot = var_phi_s0_bulk_0_dn8;
        *var_q_s0_bulk_0_slot = var_q_s0_bulk_0;
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
    }

    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        var_beta_inv: f64,
        var_c_box: f64,
        var_c_soi_inv: f64,
        var_flg_pprv: f64,
        var_guard73: f64,
        var_pbs0_ini: f64,
        var_psb0_ini: f64,
        var_pss0_ini: f64,
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
        var_q_s0_bulk_0: f64,
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
        var_fd_end_slot: &mut f64,
        var_fd_start_slot: &mut f64,
        var_guard74_slot: &mut f64,
        var_guard75_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
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
        var_shift_slot: &mut f64,
        var_shift_dn0_slot: &mut f64,
        var_shift_dn10_slot: &mut f64,
        var_shift_dn11_slot: &mut f64,
        var_shift_dn12_slot: &mut f64,
        var_shift_dn2_slot: &mut f64,
        var_shift_dn4_slot: &mut f64,
        var_shift_dn5_slot: &mut f64,
        var_shift_dn6_slot: &mut f64,
        var_shift_dn8_slot: &mut f64,
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
        var_vgp_slot: &mut f64,
        var_vgp_dn0_slot: &mut f64,
        var_vgp_dn10_slot: &mut f64,
        var_vgp_dn11_slot: &mut f64,
        var_vgp_dn12_slot: &mut f64,
        var_vgp_dn2_slot: &mut f64,
        var_vgp_dn4_slot: &mut f64,
        var_vgp_dn5_slot: &mut f64,
        var_vgp_dn6_slot: &mut f64,
        var_vgp_dn8_slot: &mut f64,
        var_vgpz_slot: &mut f64,
        var_vgpz_dn0_slot: &mut f64,
        var_vgpz_dn10_slot: &mut f64,
        var_vgpz_dn11_slot: &mut f64,
        var_vgpz_dn12_slot: &mut f64,
        var_vgpz_dn2_slot: &mut f64,
        var_vgpz_dn4_slot: &mut f64,
        var_vgpz_dn5_slot: &mut f64,
        var_vgpz_dn6_slot: &mut f64,
        var_vgpz_dn8_slot: &mut f64,
        var_vgs_fb_slot: &mut f64,
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
    ) {
        let mut var_fd_end: f64 = *var_fd_end_slot;
        let mut var_fd_start: f64 = *var_fd_start_slot;
        let mut var_guard74: f64 = *var_guard74_slot;
        let mut var_guard75: f64 = *var_guard75_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
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
        let mut var_shift: f64 = *var_shift_slot;
        let mut var_shift_dn0: f64 = *var_shift_dn0_slot;
        let mut var_shift_dn10: f64 = *var_shift_dn10_slot;
        let mut var_shift_dn11: f64 = *var_shift_dn11_slot;
        let mut var_shift_dn12: f64 = *var_shift_dn12_slot;
        let mut var_shift_dn2: f64 = *var_shift_dn2_slot;
        let mut var_shift_dn4: f64 = *var_shift_dn4_slot;
        let mut var_shift_dn5: f64 = *var_shift_dn5_slot;
        let mut var_shift_dn6: f64 = *var_shift_dn6_slot;
        let mut var_shift_dn8: f64 = *var_shift_dn8_slot;
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
        let mut var_vgp: f64 = *var_vgp_slot;
        let mut var_vgp_dn0: f64 = *var_vgp_dn0_slot;
        let mut var_vgp_dn10: f64 = *var_vgp_dn10_slot;
        let mut var_vgp_dn11: f64 = *var_vgp_dn11_slot;
        let mut var_vgp_dn12: f64 = *var_vgp_dn12_slot;
        let mut var_vgp_dn2: f64 = *var_vgp_dn2_slot;
        let mut var_vgp_dn4: f64 = *var_vgp_dn4_slot;
        let mut var_vgp_dn5: f64 = *var_vgp_dn5_slot;
        let mut var_vgp_dn6: f64 = *var_vgp_dn6_slot;
        let mut var_vgp_dn8: f64 = *var_vgp_dn8_slot;
        let mut var_vgpz: f64 = *var_vgpz_slot;
        let mut var_vgpz_dn0: f64 = *var_vgpz_dn0_slot;
        let mut var_vgpz_dn10: f64 = *var_vgpz_dn10_slot;
        let mut var_vgpz_dn11: f64 = *var_vgpz_dn11_slot;
        let mut var_vgpz_dn12: f64 = *var_vgpz_dn12_slot;
        let mut var_vgpz_dn2: f64 = *var_vgpz_dn2_slot;
        let mut var_vgpz_dn4: f64 = *var_vgpz_dn4_slot;
        let mut var_vgpz_dn5: f64 = *var_vgpz_dn5_slot;
        let mut var_vgpz_dn6: f64 = *var_vgpz_dn6_slot;
        let mut var_vgpz_dn8: f64 = *var_vgpz_dn8_slot;
        let mut var_vgs_fb: f64 = *var_vgs_fb_slot;
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

        let (assign6980_e5182, assign6980_e5182_d_n0, assign6980_e5182_d_n2, assign6980_e5182_d_n4, assign6980_e5182_d_n5, assign6980_e5182_d_n6, assign6980_e5182_d_n8, assign6980_e5182_d_n10, assign6980_e5182_d_n11, assign6980_e5182_d_n12,) = {
    if (var_guard73 != 0.0) {
        let assign6980_e5180: f64 = (1.0 / var_c_box);
        (assign6980_e5180, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn8, var_t2_dn10, var_t2_dn11, var_t2_dn12,)
    }
};
        var_t2 = assign6980_e5182;
        var_t2_dn0 = assign6980_e5182_d_n0;
        var_t2_dn2 = assign6980_e5182_d_n2;
        var_t2_dn4 = assign6980_e5182_d_n4;
        var_t2_dn5 = assign6980_e5182_d_n5;
        var_t2_dn6 = assign6980_e5182_d_n6;
        var_t2_dn8 = assign6980_e5182_d_n8;
        var_t2_dn10 = assign6980_e5182_d_n10;
        var_t2_dn11 = assign6980_e5182_d_n11;
        var_t2_dn12 = assign6980_e5182_d_n12;

        let (assign6990_e5192, assign6990_e5192_d_n0, assign6990_e5192_d_n2, assign6990_e5192_d_n4, assign6990_e5192_d_n5, assign6990_e5192_d_n6, assign6990_e5192_d_n8, assign6990_e5192_d_n10, assign6990_e5192_d_n11, assign6990_e5192_d_n12,) = {
    if (var_guard73 != 0.0) {
        let assign6990_e5187: f64 = (var_t0 + var_c_soi_inv);
        let assign6990_e5189: f64 = (assign6990_e5187 + var_t2);
        let assign6990_e5190: f64 = (1.0 / assign6990_e5189);
        (assign6990_e5190, (-((var_t0_dn0 + var_t2_dn0) / (assign6990_e5189 * assign6990_e5189))), (-((var_t0_dn2 + var_t2_dn2) / (assign6990_e5189 * assign6990_e5189))), (-((var_t0_dn4 + var_t2_dn4) / (assign6990_e5189 * assign6990_e5189))), (-((var_t0_dn5 + var_t2_dn5) / (assign6990_e5189 * assign6990_e5189))), (-((var_t0_dn6 + var_t2_dn6) / (assign6990_e5189 * assign6990_e5189))), (-((var_t0_dn8 + var_t2_dn8) / (assign6990_e5189 * assign6990_e5189))), (-((var_t0_dn10 + var_t2_dn10) / (assign6990_e5189 * assign6990_e5189))), (-((var_t0_dn11 + var_t2_dn11) / (assign6990_e5189 * assign6990_e5189))), (-((var_t0_dn12 + var_t2_dn12) / (assign6990_e5189 * assign6990_e5189))),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn8, var_t3_dn10, var_t3_dn11, var_t3_dn12,)
    }
};
        var_t3 = assign6990_e5192;
        var_t3_dn0 = assign6990_e5192_d_n0;
        var_t3_dn2 = assign6990_e5192_d_n2;
        var_t3_dn4 = assign6990_e5192_d_n4;
        var_t3_dn5 = assign6990_e5192_d_n5;
        var_t3_dn6 = assign6990_e5192_d_n6;
        var_t3_dn8 = assign6990_e5192_d_n8;
        var_t3_dn10 = assign6990_e5192_d_n10;
        var_t3_dn11 = assign6990_e5192_d_n11;
        var_t3_dn12 = assign6990_e5192_d_n12;

        let (assign7000_e5200, assign7000_e5200_d_n0, assign7000_e5200_d_n2, assign7000_e5200_d_n4, assign7000_e5200_d_n5, assign7000_e5200_d_n6, assign7000_e5200_d_n8, assign7000_e5200_d_n10, assign7000_e5200_d_n11, assign7000_e5200_d_n12,) = {
    if (var_guard73 != 0.0) {
        let assign7000_e5197: f64 = (var_t3 * var_t0);
        let assign7000_e5198: f64 = (1.0 - assign7000_e5197);
        (assign7000_e5198, (-((var_t3_dn0 * var_t0) + (var_t3 * var_t0_dn0))), (-((var_t3_dn2 * var_t0) + (var_t3 * var_t0_dn2))), (-((var_t3_dn4 * var_t0) + (var_t3 * var_t0_dn4))), (-((var_t3_dn5 * var_t0) + (var_t3 * var_t0_dn5))), (-((var_t3_dn6 * var_t0) + (var_t3 * var_t0_dn6))), (-((var_t3_dn8 * var_t0) + (var_t3 * var_t0_dn8))), (-((var_t3_dn10 * var_t0) + (var_t3 * var_t0_dn10))), (-((var_t3_dn11 * var_t0) + (var_t3 * var_t0_dn11))), (-((var_t3_dn12 * var_t0) + (var_t3 * var_t0_dn12))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn8, var_t4_dn10, var_t4_dn11, var_t4_dn12,)
    }
};
        var_t4 = assign7000_e5200;
        var_t4_dn0 = assign7000_e5200_d_n0;
        var_t4_dn2 = assign7000_e5200_d_n2;
        var_t4_dn4 = assign7000_e5200_d_n4;
        var_t4_dn5 = assign7000_e5200_d_n5;
        var_t4_dn6 = assign7000_e5200_d_n6;
        var_t4_dn8 = assign7000_e5200_d_n8;
        var_t4_dn10 = assign7000_e5200_d_n10;
        var_t4_dn11 = assign7000_e5200_d_n11;
        var_t4_dn12 = assign7000_e5200_d_n12;

        let (assign7010_e5218, assign7010_e5218_d_n0, assign7010_e5218_d_n2, assign7010_e5218_d_n4, assign7010_e5218_d_n5, assign7010_e5218_d_n6, assign7010_e5218_d_n8, assign7010_e5218_d_n10, assign7010_e5218_d_n11, assign7010_e5218_d_n12,) = {
    if (var_guard73 != 0.0) {
        let assign7010_e5205: f64 = (-var_vbsbiz);
        let assign7010_e5209: f64 = (0.5 * var_c_soi_inv);
        let assign7010_e5210: f64 = (var_t2 + assign7010_e5209);
        let assign7010_e5212: f64 = (-var_q_fd_soi);
        let assign7010_e5213: f64 = (assign7010_e5210 * assign7010_e5212);
        let assign7010_e5214: f64 = (assign7010_e5205 + assign7010_e5213);
        let assign7010_e5215: f64 = (var_t3 * assign7010_e5214);
        let assign7010_e5216: f64 = (var_t0 * assign7010_e5215);
        (assign7010_e5216, ((var_t0_dn0 * assign7010_e5215) + (var_t0 * ((var_t3_dn0 * assign7010_e5214) + (var_t3 * ((-var_vbsbiz_dn0) + ((var_t2_dn0 * assign7010_e5212) + (assign7010_e5210 * (-var_q_fd_soi_dn0)))))))), ((var_t0_dn2 * assign7010_e5215) + (var_t0 * ((var_t3_dn2 * assign7010_e5214) + (var_t3 * ((-var_vbsbiz_dn2) + ((var_t2_dn2 * assign7010_e5212) + (assign7010_e5210 * (-var_q_fd_soi_dn2)))))))), ((var_t0_dn4 * assign7010_e5215) + (var_t0 * ((var_t3_dn4 * assign7010_e5214) + (var_t3 * ((-var_vbsbiz_dn4) + ((var_t2_dn4 * assign7010_e5212) + (assign7010_e5210 * (-var_q_fd_soi_dn4)))))))), ((var_t0_dn5 * assign7010_e5215) + (var_t0 * ((var_t3_dn5 * assign7010_e5214) + (var_t3 * ((-var_vbsbiz_dn5) + ((var_t2_dn5 * assign7010_e5212) + (assign7010_e5210 * (-var_q_fd_soi_dn5)))))))), ((var_t0_dn6 * assign7010_e5215) + (var_t0 * ((var_t3_dn6 * assign7010_e5214) + (var_t3 * ((-var_vbsbiz_dn6) + ((var_t2_dn6 * assign7010_e5212) + (assign7010_e5210 * (-var_q_fd_soi_dn6)))))))), ((var_t0_dn8 * assign7010_e5215) + (var_t0 * ((var_t3_dn8 * assign7010_e5214) + (var_t3 * ((-var_vbsbiz_dn8) + ((var_t2_dn8 * assign7010_e5212) + (assign7010_e5210 * (-var_q_fd_soi_dn8)))))))), ((var_t0_dn10 * assign7010_e5215) + (var_t0 * ((var_t3_dn10 * assign7010_e5214) + (var_t3 * ((-var_vbsbiz_dn10) + ((var_t2_dn10 * assign7010_e5212) + (assign7010_e5210 * (-var_q_fd_soi_dn10)))))))), ((var_t0_dn11 * assign7010_e5215) + (var_t0 * ((var_t3_dn11 * assign7010_e5214) + (var_t3 * ((-var_vbsbiz_dn11) + ((var_t2_dn11 * assign7010_e5212) + (assign7010_e5210 * (-var_q_fd_soi_dn11)))))))), ((var_t0_dn12 * assign7010_e5215) + (var_t0 * ((var_t3_dn12 * assign7010_e5214) + (var_t3 * ((-var_vbsbiz_dn12) + ((var_t2_dn12 * assign7010_e5212) + (assign7010_e5210 * (-var_q_fd_soi_dn12)))))))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn8, var_t5_dn10, var_t5_dn11, var_t5_dn12,)
    }
};
        var_t5 = assign7010_e5218;
        var_t5_dn0 = assign7010_e5218_d_n0;
        var_t5_dn2 = assign7010_e5218_d_n2;
        var_t5_dn4 = assign7010_e5218_d_n4;
        var_t5_dn5 = assign7010_e5218_d_n5;
        var_t5_dn6 = assign7010_e5218_d_n6;
        var_t5_dn8 = assign7010_e5218_d_n8;
        var_t5_dn10 = assign7010_e5218_d_n10;
        var_t5_dn11 = assign7010_e5218_d_n11;
        var_t5_dn12 = assign7010_e5218_d_n12;

        let (assign7020_e5224, assign7020_e5224_d_n0, assign7020_e5224_d_n2, assign7020_e5224_d_n4, assign7020_e5224_d_n5, assign7020_e5224_d_n6, assign7020_e5224_d_n8, assign7020_e5224_d_n10, assign7020_e5224_d_n11, assign7020_e5224_d_n12,) = {
    if (var_guard73 != 0.0) {
        let assign7020_e5222: f64 = (var_t5 / var_t4);
        (assign7020_e5222, (((var_t5_dn0 * var_t4) - (var_t5 * var_t4_dn0)) / (var_t4 * var_t4)), (((var_t5_dn2 * var_t4) - (var_t5 * var_t4_dn2)) / (var_t4 * var_t4)), (((var_t5_dn4 * var_t4) - (var_t5 * var_t4_dn4)) / (var_t4 * var_t4)), (((var_t5_dn5 * var_t4) - (var_t5 * var_t4_dn5)) / (var_t4 * var_t4)), (((var_t5_dn6 * var_t4) - (var_t5 * var_t4_dn6)) / (var_t4 * var_t4)), (((var_t5_dn8 * var_t4) - (var_t5 * var_t4_dn8)) / (var_t4 * var_t4)), (((var_t5_dn10 * var_t4) - (var_t5 * var_t4_dn10)) / (var_t4 * var_t4)), (((var_t5_dn11 * var_t4) - (var_t5 * var_t4_dn11)) / (var_t4 * var_t4)), (((var_t5_dn12 * var_t4) - (var_t5 * var_t4_dn12)) / (var_t4 * var_t4)),)
    } else {
        (var_shift, var_shift_dn0, var_shift_dn2, var_shift_dn4, var_shift_dn5, var_shift_dn6, var_shift_dn8, var_shift_dn10, var_shift_dn11, var_shift_dn12,)
    }
};
        var_shift = assign7020_e5224;
        var_shift_dn0 = assign7020_e5224_d_n0;
        var_shift_dn2 = assign7020_e5224_d_n2;
        var_shift_dn4 = assign7020_e5224_d_n4;
        var_shift_dn5 = assign7020_e5224_d_n5;
        var_shift_dn6 = assign7020_e5224_d_n6;
        var_shift_dn8 = assign7020_e5224_d_n8;
        var_shift_dn10 = assign7020_e5224_d_n10;
        var_shift_dn11 = assign7020_e5224_d_n11;
        var_shift_dn12 = assign7020_e5224_d_n12;

        let (assign7030_e5230,) = {
    if (var_guard73 != 0.0) {
        let assign7030_e5228: f64 = (var_vgs_fb + var_shift);
        (assign7030_e5228,)
    } else {
        (var_vgs_fb,)
    }
};
        var_vgs_fb = assign7030_e5230;

        let (assign7040_e5238, assign7040_e5238_d_n0, assign7040_e5238_d_n2, assign7040_e5238_d_n4, assign7040_e5238_d_n5, assign7040_e5238_d_n6, assign7040_e5238_d_n8, assign7040_e5238_d_n10, assign7040_e5238_d_n11, assign7040_e5238_d_n12,) = {
    if (var_guard73 != 0.0) {
        let assign7040_e5235: f64 = (p.p298 * var_shift);
        let assign7040_e5236: f64 = (var_vgp - assign7040_e5235);
        (assign7040_e5236, (var_vgp_dn0 - (p.p298 * var_shift_dn0)), (var_vgp_dn2 - (p.p298 * var_shift_dn2)), (var_vgp_dn4 - (p.p298 * var_shift_dn4)), (var_vgp_dn5 - (p.p298 * var_shift_dn5)), (var_vgp_dn6 - (p.p298 * var_shift_dn6)), (var_vgp_dn8 - (p.p298 * var_shift_dn8)), (var_vgp_dn10 - (p.p298 * var_shift_dn10)), (var_vgp_dn11 - (p.p298 * var_shift_dn11)), (var_vgp_dn12 - (p.p298 * var_shift_dn12)),)
    } else {
        (var_vgp, var_vgp_dn0, var_vgp_dn2, var_vgp_dn4, var_vgp_dn5, var_vgp_dn6, var_vgp_dn8, var_vgp_dn10, var_vgp_dn11, var_vgp_dn12,)
    }
};
        var_vgp = assign7040_e5238;
        var_vgp_dn0 = assign7040_e5238_d_n0;
        var_vgp_dn2 = assign7040_e5238_d_n2;
        var_vgp_dn4 = assign7040_e5238_d_n4;
        var_vgp_dn5 = assign7040_e5238_d_n5;
        var_vgp_dn6 = assign7040_e5238_d_n6;
        var_vgp_dn8 = assign7040_e5238_d_n8;
        var_vgp_dn10 = assign7040_e5238_d_n10;
        var_vgp_dn11 = assign7040_e5238_d_n11;
        var_vgp_dn12 = assign7040_e5238_d_n12;

        let (assign7050_e5242, assign7050_e5242_d_n0, assign7050_e5242_d_n2, assign7050_e5242_d_n4, assign7050_e5242_d_n5, assign7050_e5242_d_n6, assign7050_e5242_d_n8, assign7050_e5242_d_n10, assign7050_e5242_d_n11, assign7050_e5242_d_n12,) = {
    if (var_guard73 != 0.0) {
        (var_vgp, var_vgp_dn0, var_vgp_dn2, var_vgp_dn4, var_vgp_dn5, var_vgp_dn6, var_vgp_dn8, var_vgp_dn10, var_vgp_dn11, var_vgp_dn12,)
    } else {
        (var_vgpz, var_vgpz_dn0, var_vgpz_dn2, var_vgpz_dn4, var_vgpz_dn5, var_vgpz_dn6, var_vgpz_dn8, var_vgpz_dn10, var_vgpz_dn11, var_vgpz_dn12,)
    }
};
        var_vgpz = assign7050_e5242;
        var_vgpz_dn0 = assign7050_e5242_d_n0;
        var_vgpz_dn2 = assign7050_e5242_d_n2;
        var_vgpz_dn4 = assign7050_e5242_d_n4;
        var_vgpz_dn5 = assign7050_e5242_d_n5;
        var_vgpz_dn6 = assign7050_e5242_d_n6;
        var_vgpz_dn8 = assign7050_e5242_d_n8;
        var_vgpz_dn10 = assign7050_e5242_d_n10;
        var_vgpz_dn11 = assign7050_e5242_d_n11;
        var_vgpz_dn12 = assign7050_e5242_d_n12;

        let assign7060_e5245: f64 = if var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        var_guard74 = assign7060_e5245;

        let (assign7070_e5249, assign7070_e5249_d_n0, assign7070_e5249_d_n2, assign7070_e5249_d_n4, assign7070_e5249_d_n5, assign7070_e5249_d_n6, assign7070_e5249_d_n8, assign7070_e5249_d_n10, assign7070_e5249_d_n11, assign7070_e5249_d_n12,) = {
    if (var_guard74 != 0.0) {
        (var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_phi_s0_soi, var_phi_s0_soi_dn0, var_phi_s0_soi_dn2, var_phi_s0_soi_dn4, var_phi_s0_soi_dn5, var_phi_s0_soi_dn6, var_phi_s0_soi_dn8, var_phi_s0_soi_dn10, var_phi_s0_soi_dn11, var_phi_s0_soi_dn12,)
    }
};
        var_phi_s0_soi = assign7070_e5249;
        var_phi_s0_soi_dn0 = assign7070_e5249_d_n0;
        var_phi_s0_soi_dn2 = assign7070_e5249_d_n2;
        var_phi_s0_soi_dn4 = assign7070_e5249_d_n4;
        var_phi_s0_soi_dn5 = assign7070_e5249_d_n5;
        var_phi_s0_soi_dn6 = assign7070_e5249_d_n6;
        var_phi_s0_soi_dn8 = assign7070_e5249_d_n8;
        var_phi_s0_soi_dn10 = assign7070_e5249_d_n10;
        var_phi_s0_soi_dn11 = assign7070_e5249_d_n11;
        var_phi_s0_soi_dn12 = assign7070_e5249_d_n12;

        let (assign7080_e5253, assign7080_e5253_d_n0, assign7080_e5253_d_n2, assign7080_e5253_d_n4, assign7080_e5253_d_n5, assign7080_e5253_d_n6, assign7080_e5253_d_n8, assign7080_e5253_d_n10, assign7080_e5253_d_n11, assign7080_e5253_d_n12,) = {
    if (var_guard74 != 0.0) {
        (var_pbs0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_phi_b0_soi, var_phi_b0_soi_dn0, var_phi_b0_soi_dn2, var_phi_b0_soi_dn4, var_phi_b0_soi_dn5, var_phi_b0_soi_dn6, var_phi_b0_soi_dn8, var_phi_b0_soi_dn10, var_phi_b0_soi_dn11, var_phi_b0_soi_dn12,)
    }
};
        var_phi_b0_soi = assign7080_e5253;
        var_phi_b0_soi_dn0 = assign7080_e5253_d_n0;
        var_phi_b0_soi_dn2 = assign7080_e5253_d_n2;
        var_phi_b0_soi_dn4 = assign7080_e5253_d_n4;
        var_phi_b0_soi_dn5 = assign7080_e5253_d_n5;
        var_phi_b0_soi_dn6 = assign7080_e5253_d_n6;
        var_phi_b0_soi_dn8 = assign7080_e5253_d_n8;
        var_phi_b0_soi_dn10 = assign7080_e5253_d_n10;
        var_phi_b0_soi_dn11 = assign7080_e5253_d_n11;
        var_phi_b0_soi_dn12 = assign7080_e5253_d_n12;

        let (assign7090_e5259, assign7090_e5259_d_n0, assign7090_e5259_d_n2, assign7090_e5259_d_n4, assign7090_e5259_d_n5, assign7090_e5259_d_n6, assign7090_e5259_d_n8, assign7090_e5259_d_n10, assign7090_e5259_d_n11, assign7090_e5259_d_n12,) = {
    if (var_guard74 != 0.0) {
        let assign7090_e5257: f64 = (var_psb0_ini + var_vbsbiz);
        (assign7090_e5257, var_vbsbiz_dn0, var_vbsbiz_dn2, var_vbsbiz_dn4, var_vbsbiz_dn5, var_vbsbiz_dn6, var_vbsbiz_dn8, var_vbsbiz_dn10, var_vbsbiz_dn11, var_vbsbiz_dn12,)
    } else {
        (var_phi_s0_bulk, var_phi_s0_bulk_dn0, var_phi_s0_bulk_dn2, var_phi_s0_bulk_dn4, var_phi_s0_bulk_dn5, var_phi_s0_bulk_dn6, var_phi_s0_bulk_dn8, var_phi_s0_bulk_dn10, var_phi_s0_bulk_dn11, var_phi_s0_bulk_dn12,)
    }
};
        var_phi_s0_bulk = assign7090_e5259;
        var_phi_s0_bulk_dn0 = assign7090_e5259_d_n0;
        var_phi_s0_bulk_dn2 = assign7090_e5259_d_n2;
        var_phi_s0_bulk_dn4 = assign7090_e5259_d_n4;
        var_phi_s0_bulk_dn5 = assign7090_e5259_d_n5;
        var_phi_s0_bulk_dn6 = assign7090_e5259_d_n6;
        var_phi_s0_bulk_dn8 = assign7090_e5259_d_n8;
        var_phi_s0_bulk_dn10 = assign7090_e5259_d_n10;
        var_phi_s0_bulk_dn11 = assign7090_e5259_d_n11;
        var_phi_s0_bulk_dn12 = assign7090_e5259_d_n12;

        let (assign7100_e5270,) = {
    if (var_guard74 != 0.0) {
        let assign7100_e5262: f64 = (-var_q_fd_soi);
        let assign7100_e5264: f64 = (assign7100_e5262 * var_c_soi_inv);
        let assign7100_e5266: f64 = (assign7100_e5264 / 2.0);
        let assign7100_e5268: f64 = (assign7100_e5266 + var_beta_inv);
        (assign7100_e5268,)
    } else {
        (var_fd_start,)
    }
};
        var_fd_start = assign7100_e5270;

        let (assign7110_e5278,) = {
    if (var_guard74 != 0.0) {
        let assign7110_e5275: f64 = (var_q_s0_bulk_0 * var_c_soi_inv);
        let assign7110_e5276: f64 = (var_fd_start - assign7110_e5275);
        (assign7110_e5276,)
    } else {
        (var_fd_end,)
    }
};
        var_fd_end = assign7110_e5278;

        let assign7120_e5281: f64 = if var_vbsbiz < 0.0 { 1.0 } else { 0.0 };
        var_guard75 = assign7120_e5281;

        let (assign7130_e5288, assign7130_e5288_d_n0, assign7130_e5288_d_n2, assign7130_e5288_d_n4, assign7130_e5288_d_n5, assign7130_e5288_d_n6, assign7130_e5288_d_n8, assign7130_e5288_d_n10, assign7130_e5288_d_n11, assign7130_e5288_d_n12,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vgs_shift, var_vgs_shift_dn0, var_vgs_shift_dn2, var_vgs_shift_dn4, var_vgs_shift_dn5, var_vgs_shift_dn6, var_vgs_shift_dn8, var_vgs_shift_dn10, var_vgs_shift_dn11, var_vgs_shift_dn12,)
    }
};
        var_vgs_shift = assign7130_e5288;
        var_vgs_shift_dn0 = assign7130_e5288_d_n0;
        var_vgs_shift_dn2 = assign7130_e5288_d_n2;
        var_vgs_shift_dn4 = assign7130_e5288_d_n4;
        var_vgs_shift_dn5 = assign7130_e5288_d_n5;
        var_vgs_shift_dn6 = assign7130_e5288_d_n6;
        var_vgs_shift_dn8 = assign7130_e5288_d_n8;
        var_vgs_shift_dn10 = assign7130_e5288_d_n10;
        var_vgs_shift_dn11 = assign7130_e5288_d_n11;
        var_vgs_shift_dn12 = assign7130_e5288_d_n12;

        let (assign7140_e5295,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        (1.0,)
    } else {
        (var_lp_s0,)
    }
};
        var_lp_s0 = assign7140_e5295;

        *var_fd_end_slot = var_fd_end;
        *var_fd_start_slot = var_fd_start;
        *var_guard74_slot = var_guard74;
        *var_guard75_slot = var_guard75;
        *var_lp_s0_slot = var_lp_s0;
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
        *var_shift_slot = var_shift;
        *var_shift_dn0_slot = var_shift_dn0;
        *var_shift_dn10_slot = var_shift_dn10;
        *var_shift_dn11_slot = var_shift_dn11;
        *var_shift_dn12_slot = var_shift_dn12;
        *var_shift_dn2_slot = var_shift_dn2;
        *var_shift_dn4_slot = var_shift_dn4;
        *var_shift_dn5_slot = var_shift_dn5;
        *var_shift_dn6_slot = var_shift_dn6;
        *var_shift_dn8_slot = var_shift_dn8;
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
        *var_vgp_slot = var_vgp;
        *var_vgp_dn0_slot = var_vgp_dn0;
        *var_vgp_dn10_slot = var_vgp_dn10;
        *var_vgp_dn11_slot = var_vgp_dn11;
        *var_vgp_dn12_slot = var_vgp_dn12;
        *var_vgp_dn2_slot = var_vgp_dn2;
        *var_vgp_dn4_slot = var_vgp_dn4;
        *var_vgp_dn5_slot = var_vgp_dn5;
        *var_vgp_dn6_slot = var_vgp_dn6;
        *var_vgp_dn8_slot = var_vgp_dn8;
        *var_vgpz_slot = var_vgpz;
        *var_vgpz_dn0_slot = var_vgpz_dn0;
        *var_vgpz_dn10_slot = var_vgpz_dn10;
        *var_vgpz_dn11_slot = var_vgpz_dn11;
        *var_vgpz_dn12_slot = var_vgpz_dn12;
        *var_vgpz_dn2_slot = var_vgpz_dn2;
        *var_vgpz_dn4_slot = var_vgpz_dn4;
        *var_vgpz_dn5_slot = var_vgpz_dn5;
        *var_vgpz_dn6_slot = var_vgpz_dn6;
        *var_vgpz_dn8_slot = var_vgpz_dn8;
        *var_vgs_fb_slot = var_vgs_fb;
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
    }

    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn4: f64,
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
        var_flg_depmode_slot: &mut f64,
        var_flg_zone_slot: &mut f64,
        var_guard76_slot: &mut f64,
        var_guard77_slot: &mut f64,
        var_guard78_slot: &mut f64,
        var_guard79_slot: &mut f64,
        var_guard80_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
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
        var_pf1_dn0_slot: &mut f64,
        var_pf1_dn10_slot: &mut f64,
        var_pf1_dn11_slot: &mut f64,
        var_pf1_dn12_slot: &mut f64,
        var_pf1_dn2_slot: &mut f64,
        var_pf1_dn4_slot: &mut f64,
        var_pf1_dn5_slot: &mut f64,
        var_pf1_dn6_slot: &mut f64,
        var_pf1_dn8_slot: &mut f64,
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
        let mut var_flg_depmode: f64 = *var_flg_depmode_slot;
        let mut var_flg_zone: f64 = *var_flg_zone_slot;
        let mut var_guard76: f64 = *var_guard76_slot;
        let mut var_guard77: f64 = *var_guard77_slot;
        let mut var_guard78: f64 = *var_guard78_slot;
        let mut var_guard79: f64 = *var_guard79_slot;
        let mut var_guard80: f64 = *var_guard80_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
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
        let mut var_pf1_dn0: f64 = *var_pf1_dn0_slot;
        let mut var_pf1_dn10: f64 = *var_pf1_dn10_slot;
        let mut var_pf1_dn11: f64 = *var_pf1_dn11_slot;
        let mut var_pf1_dn12: f64 = *var_pf1_dn12_slot;
        let mut var_pf1_dn2: f64 = *var_pf1_dn2_slot;
        let mut var_pf1_dn4: f64 = *var_pf1_dn4_slot;
        let mut var_pf1_dn5: f64 = *var_pf1_dn5_slot;
        let mut var_pf1_dn6: f64 = *var_pf1_dn6_slot;
        let mut var_pf1_dn8: f64 = *var_pf1_dn8_slot;
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
            let assign7150_body13_e5509: f64 = (var_dpsb).abs();
            let assign7150_body13_e5511: f64 = if assign7150_body13_e5509 < 1e-12 { 1.0 } else { 0.0 };
            var_guard76 = assign7150_body13_e5511;
            let (assign7150_body14_e5520,) = {
    if (((var_guard74 == 0.0) && (var_guard75 != 0.0)) && (var_guard76 != 0.0)) {
        (var_lp_s0_max,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign7150_body14_e5520;
            let assign7150_body15_e5523: f64 = if var_dpsb > 0.1 { 1.0 } else { 0.0 };
            var_guard77 = assign7150_body15_e5523;
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
            let assign7150_body17_e5538: f64 = (-0.1);
            let assign7150_body17_e5539: f64 = if var_dpsb < assign7150_body17_e5538 { 1.0 } else { 0.0 };
            var_guard78 = assign7150_body17_e5539;
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
            let (assign7150_body20_e5573,) = {
    if ((var_guard74 == 0.0) && (var_guard75 != 0.0)) {
        let assign7150_body20_e5571: f64 = (var_lp_s0 + 1.0);
        (assign7150_body20_e5571,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign7150_body20_e5573;
        }

        let assign7160_e5577: f64 = (var_vgs_fb + var_vgs_shift);
        let assign7160_e5578: f64 = if var_vgs < assign7160_e5577 { 1.0 } else { 0.0 };
        var_guard79 = assign7160_e5578;

        let (assign7170_e5585,) = {
    if ((var_guard74 == 0.0) && (var_guard79 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_depmode,)
    }
};
        var_flg_depmode = assign7170_e5585;

        let (assign7180_e5593,) = {
    if ((var_guard74 == 0.0) && (var_guard79 != 0.0)) {
        let assign7180_e5591: f64 = (-1.0);
        (assign7180_e5591,)
    } else {
        (var_flg_zone,)
    }
};
        var_flg_zone = assign7180_e5593;

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

        let assign7220_e5632: f64 = (var_wdsoi + var_t1);
        let assign7220_e5634: f64 = if assign7220_e5632 < p.p227 { 1.0 } else { 0.0 };
        var_guard80 = assign7220_e5634;

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
        *var_flg_depmode_slot = var_flg_depmode;
        *var_flg_zone_slot = var_flg_zone;
        *var_guard76_slot = var_guard76;
        *var_guard77_slot = var_guard77;
        *var_guard78_slot = var_guard78;
        *var_guard79_slot = var_guard79;
        *var_guard80_slot = var_guard80;
        *var_lp_s0_slot = var_lp_s0;
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
        *var_pf1_dn0_slot = var_pf1_dn0;
        *var_pf1_dn10_slot = var_pf1_dn10;
        *var_pf1_dn11_slot = var_pf1_dn11;
        *var_pf1_dn12_slot = var_pf1_dn12;
        *var_pf1_dn2_slot = var_pf1_dn2;
        *var_pf1_dn4_slot = var_pf1_dn4;
        *var_pf1_dn5_slot = var_pf1_dn5;
        *var_pf1_dn6_slot = var_pf1_dn6;
        *var_pf1_dn8_slot = var_pf1_dn8;
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
    }

    pub(super) fn stamp_transient_block_21(
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
        var_guard82_slot: &mut f64,
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
    ) {
        let mut var_guard81: f64 = *var_guard81_slot;
        let mut var_guard82: f64 = *var_guard82_slot;
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

        let assign7300_e5760: f64 = if var_psb_inia < var_pb2_bulk { 1.0 } else { 0.0 };
        var_guard81 = assign7300_e5760;

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

        let assign7450_e6013: f64 = if var_psb_inia < var_pb2_bulk { 1.0 } else { 0.0 };
        var_guard82 = assign7450_e6013;

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

        *var_guard81_slot = var_guard81;
        *var_guard82_slot = var_guard82;
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
    }

    pub(super) fn stamp_transient_block_22(
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
        var_wdsoi: f64,
        var_guard83_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
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
    ) {
        let mut var_guard83: f64 = *var_guard83_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
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

        let assign7540_e6153: f64 = (var_wdsoi + var_t1);
        let assign7540_e6155: f64 = if assign7540_e6153 < p.p227 { 1.0 } else { 0.0 };
        var_guard83 = assign7540_e6155;

        let (assign7550_e6164,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        (0.0,)
    } else {
        (var_lp_s0,)
    }
};
        var_lp_s0 = assign7550_e6164;

        *var_guard83_slot = var_guard83;
        *var_lp_s0_slot = var_lp_s0;
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
    }

    pub(super) fn stamp_transient_block_23(
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
        var_guard85_slot: &mut f64,
        var_guard86_slot: &mut f64,
        var_guard87_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
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
    ) {
        let mut var_guard84: f64 = *var_guard84_slot;
        let mut var_guard85: f64 = *var_guard85_slot;
        let mut var_guard86: f64 = *var_guard86_slot;
        let mut var_guard87: f64 = *var_guard87_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
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
            let assign7560_body3_e6208: f64 = if var_phi_s0_bulk > 1e-8 { 1.0 } else { 0.0 };
            var_guard84 = assign7560_body3_e6208;
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
            let assign7560_body7_e6272: f64 = (-1e-8);
            let assign7560_body7_e6273: f64 = if var_phi_s0_bulk < assign7560_body7_e6272 { 1.0 } else { 0.0 };
            var_guard85 = assign7560_body7_e6273;
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
            let assign7560_body15_e6410: f64 = if var_t6 < 0.0 { 1.0 } else { 0.0 };
            var_guard86 = assign7560_body15_e6410;
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
            let assign7560_body28_e6599: f64 = (var_t6 - var_phi_s0_bulk);
            let assign7560_body28_e6600: f64 = (assign7560_body28_e6599).abs();
            let assign7560_body28_e6602: f64 = if assign7560_body28_e6600 < 0.001 { 1.0 } else { 0.0 };
            var_guard87 = assign7560_body28_e6602;
            let (assign7560_body29_e6613,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) && (var_guard87 != 0.0)) {
        (var_lp_s0_max,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign7560_body29_e6613;
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
            let (assign7560_body32_e6642,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 != 0.0)) {
        let assign7560_body32_e6640: f64 = (var_lp_s0 + 1.0);
        (assign7560_body32_e6640,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign7560_body32_e6642;
        }

        *var_guard84_slot = var_guard84;
        *var_guard85_slot = var_guard85;
        *var_guard86_slot = var_guard86;
        *var_guard87_slot = var_guard87;
        *var_lp_s0_slot = var_lp_s0;
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
    }

    pub(super) fn stamp_transient_block_24(
        var_guard74: f64,
        var_guard79: f64,
        var_guard83: f64,
        var_lp_s0_slot: &mut f64,
    ) {
        let mut var_lp_s0: f64 = *var_lp_s0_slot;

        let (assign7570_e6652,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        (0.0,)
    } else {
        (var_lp_s0,)
    }
};
        var_lp_s0 = assign7570_e6652;

        *var_lp_s0_slot = var_lp_s0;
    }

    pub(super) fn stamp_transient_block_25(
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
        var_guard89_slot: &mut f64,
        var_guard90_slot: &mut f64,
        var_guard91_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
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
    ) {
        let mut var_guard88: f64 = *var_guard88_slot;
        let mut var_guard89: f64 = *var_guard89_slot;
        let mut var_guard90: f64 = *var_guard90_slot;
        let mut var_guard91: f64 = *var_guard91_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
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
            let assign7580_body3_e6700: f64 = if var_phi_s0_bulk > 1e-8 { 1.0 } else { 0.0 };
            var_guard88 = assign7580_body3_e6700;
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
            let assign7580_body7_e6767: f64 = (-1e-8);
            let assign7580_body7_e6768: f64 = if var_phi_s0_bulk < assign7580_body7_e6767 { 1.0 } else { 0.0 };
            var_guard89 = assign7580_body7_e6768;
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
            let assign7580_body15_e6912: f64 = if var_t6 < 0.0 { 1.0 } else { 0.0 };
            var_guard90 = assign7580_body15_e6912;
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
            let assign7580_body28_e7130: f64 = (var_t6 - var_phi_s0_bulk);
            let assign7580_body28_e7131: f64 = (assign7580_body28_e7130).abs();
            let assign7580_body28_e7133: f64 = if assign7580_body28_e7131 < 0.001 { 1.0 } else { 0.0 };
            var_guard91 = assign7580_body28_e7133;
            let (assign7580_body29_e7145,) = {
    if ((((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) && (var_guard91 != 0.0)) {
        (var_lp_s0_max,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign7580_body29_e7145;
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
            let (assign7580_body32_e7177,) = {
    if (((var_guard74 == 0.0) && (var_guard79 != 0.0)) && (var_guard83 == 0.0)) {
        let assign7580_body32_e7175: f64 = (var_lp_s0 + 1.0);
        (assign7580_body32_e7175,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign7580_body32_e7177;
        }

        *var_guard88_slot = var_guard88;
        *var_guard89_slot = var_guard89;
        *var_guard90_slot = var_guard90;
        *var_guard91_slot = var_guard91;
        *var_lp_s0_slot = var_lp_s0;
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
    }

    pub(super) fn stamp_transient_block_26(
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
        var_guard93_slot: &mut f64,
        var_guard94_slot: &mut f64,
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
    ) {
        let mut var_guard92: f64 = *var_guard92_slot;
        let mut var_guard93: f64 = *var_guard93_slot;
        let mut var_guard94: f64 = *var_guard94_slot;
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

        let assign7680_e7281: f64 = (var_vgs - var_shift);
        let assign7680_e7283: f64 = if assign7680_e7281 <= var_vth { 1.0 } else { 0.0 };
        var_guard92 = assign7680_e7283;

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

        let assign7750_e7374: f64 = (var_vgs - var_shift);
        let assign7750_e7376: f64 = if assign7750_e7374 > var_vth { 1.0 } else { 0.0 };
        var_guard93 = assign7750_e7376;

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

        let assign7800_e7429: f64 = (var_ps0_inib - 0.15);
        let assign7800_e7434: f64 = if ((var_ps0_inia > assign7800_e7429) && (0.15 >= 0.0)) { 1.0 } else { 0.0 };
        var_guard94 = assign7800_e7434;

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

        *var_guard92_slot = var_guard92;
        *var_guard93_slot = var_guard93;
        *var_guard94_slot = var_guard94;
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
    }

    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
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
        var_flg_depmode_slot: &mut f64,
        var_guard100_slot: &mut f64,
        var_guard101_slot: &mut f64,
        var_guard95_slot: &mut f64,
        var_guard96_slot: &mut f64,
        var_guard97_slot: &mut f64,
        var_guard98_slot: &mut f64,
        var_guard99_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_mm_slot: &mut f64,
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
        var_wdsoi_slot: &mut f64,
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
        let mut var_flg_depmode: f64 = *var_flg_depmode_slot;
        let mut var_guard100: f64 = *var_guard100_slot;
        let mut var_guard101: f64 = *var_guard101_slot;
        let mut var_guard95: f64 = *var_guard95_slot;
        let mut var_guard96: f64 = *var_guard96_slot;
        let mut var_guard97: f64 = *var_guard97_slot;
        let mut var_guard98: f64 = *var_guard98_slot;
        let mut var_guard99: f64 = *var_guard99_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_mm: f64 = *var_mm_slot;
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
        let mut var_wdsoi: f64 = *var_wdsoi_slot;
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

        let (assign7860_e7496,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign7860_e7496;

        let (assign7870_e7505,) = {
    if (((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign7870_e7505;

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

        let assign7940_e7580: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard95 = assign7940_e7580;

        let assign7950_e7583: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard96 = assign7950_e7583;

        let (assign7960_e7596,) = {
    if (((((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) && (var_guard95 != 0.0)) && (var_guard96 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign7960_e7596;

        let assign7970_e7599: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard97 = assign7970_e7599;

        let (assign7980_e7615,) = {
    if ((((((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) && (var_guard95 != 0.0)) && (var_guard96 == 0.0)) && (var_guard97 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign7980_e7615;

        let assign7990_e7618: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard98 = assign7990_e7618;

        let (assign8000_e7637,) = {
    if (((((((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) && (var_guard95 != 0.0)) && (var_guard96 == 0.0)) && (var_guard97 == 0.0)) && (var_guard98 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign8000_e7637;

        let assign8010_e7640: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard99 = assign8010_e7640;

        let (assign8020_e7662,) = {
    if ((((((((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) && (var_guard95 != 0.0)) && (var_guard96 == 0.0)) && (var_guard97 == 0.0)) && (var_guard98 == 0.0)) && (var_guard99 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign8020_e7662;

        let (assign8030_e7673,) = {
    if ((((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) && (var_guard95 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign8030_e7673;

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
            let (assign8040_body1_e7710,) = {
    if ((((var_guard74 == 0.0) && (var_guard93 != 0.0)) && (var_guard94 != 0.0)) && (var_guard95 != 0.0)) {
        let assign8040_body1_e7708: f64 = (var_m0 + 1.0);
        (assign8040_body1_e7708,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign8040_body1_e7710;
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

        let (assign8130_e7832,) = {
    if (var_guard74 == 0.0) {
        let (assign8130_e7830,) = {
            if (var_ps0_ini > 0.0) {
                let assign8130_e7821: f64 = (2.0 * 1.034943e-10);
                let assign8130_e7823: f64 = (assign8130_e7821 / 1.6021918e-19);
                let assign8130_e7825: f64 = (assign8130_e7823 * var_ps0_ini);
                let assign8130_e7827: f64 = (assign8130_e7825 / var_uc_nsubs);
                let assign8130_e7828: f64 = (assign8130_e7827).sqrt();
                (assign8130_e7828,)
            } else {
                (0.0,)
            }
        };
        (assign8130_e7830,)
    } else {
        (var_wdsoi,)
    }
};
        var_wdsoi = assign8130_e7832;

        let assign8140_e7835: f64 = if var_wdsoi < p.p227 { 1.0 } else { 0.0 };
        var_guard100 = assign8140_e7835;

        let (assign8150_e7842,) = {
    if ((var_guard74 == 0.0) && (var_guard100 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_depmode,)
    }
};
        var_flg_depmode = assign8150_e7842;

        let (assign8160_e7850,) = {
    if ((var_guard74 == 0.0) && (var_guard100 == 0.0)) {
        (2.0,)
    } else {
        (var_flg_depmode,)
    }
};
        var_flg_depmode = assign8160_e7850;

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

        let assign8200_e7874: f64 = if var_flg_depmode == 1.0 { 1.0 } else { 0.0 };
        var_guard101 = assign8200_e7874;

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
        *var_flg_depmode_slot = var_flg_depmode;
        *var_guard100_slot = var_guard100;
        *var_guard101_slot = var_guard101;
        *var_guard95_slot = var_guard95;
        *var_guard96_slot = var_guard96;
        *var_guard97_slot = var_guard97;
        *var_guard98_slot = var_guard98;
        *var_guard99_slot = var_guard99;
        *var_m0_slot = var_m0;
        *var_mm_slot = var_mm;
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
        *var_wdsoi_slot = var_wdsoi;
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
    }

    pub(super) fn stamp_transient_block_28(
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
        var_guard101: f64,
        var_guard74: f64,
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
        var_guard102_slot: &mut f64,
        var_guard103_slot: &mut f64,
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
    ) {
        let mut var_guard102: f64 = *var_guard102_slot;
        let mut var_guard103: f64 = *var_guard103_slot;
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

        let assign8280_e7982: f64 = if var_psb_inia < var_pb2_bulk { 1.0 } else { 0.0 };
        var_guard102 = assign8280_e7982;

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

        let assign8430_e8207: f64 = if var_psb_inia < var_pb2_bulk { 1.0 } else { 0.0 };
        var_guard103 = assign8430_e8207;

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

        *var_guard102_slot = var_guard102;
        *var_guard103_slot = var_guard103;
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
    }

    pub(super) fn stamp_transient_block_29(
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
        var_guard101: f64,
        var_guard103: f64,
        var_guard74: f64,
        var_lp_s0_max: f64,
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
        var_guard104_slot: &mut f64,
        var_guard105_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
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
    ) {
        let mut var_flg_depmode: f64 = *var_flg_depmode_slot;
        let mut var_guard104: f64 = *var_guard104_slot;
        let mut var_guard105: f64 = *var_guard105_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
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

        let assign8510_e8319: f64 = if ((var_flg_depmode == 1.0) && (0.0 != 0.0)) { 1.0 } else { 0.0 };
        var_guard104 = assign8510_e8319;

        let (assign8520_e8326,) = {
    if ((var_guard74 == 0.0) && (var_guard104 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_depmode,)
    }
};
        var_flg_depmode = assign8520_e8326;

        let (assign8530_e8333,) = {
    if ((var_guard74 == 0.0) && (var_guard104 != 0.0)) {
        (0.0,)
    } else {
        (var_lp_s0,)
    }
};
        var_lp_s0 = assign8530_e8333;

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
            let assign8540_body3_e8369: f64 = if var_phi_s0_bulk > 1e-8 { 1.0 } else { 0.0 };
            var_guard105 = assign8540_body3_e8369;
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
            let assign8540_body7_e8427: f64 = (-1e-8);
            let assign8540_body7_e8428: f64 = if var_phi_s0_bulk < assign8540_body7_e8427 { 1.0 } else { 0.0 };
            var_guard106 = assign8540_body7_e8428;
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
            let assign8540_body13_e8530: f64 = (var_t6 - var_phi_s0_bulk);
            let assign8540_body13_e8531: f64 = (assign8540_body13_e8530).abs();
            let assign8540_body13_e8533: f64 = if assign8540_body13_e8531 < 0.001 { 1.0 } else { 0.0 };
            var_guard107 = assign8540_body13_e8533;
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
            let (assign8540_body15_e8551,) = {
    if (((var_guard74 == 0.0) && (var_guard104 != 0.0)) && (var_guard107 != 0.0)) {
        (var_lp_s0_max,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign8540_body15_e8551;
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
            let (assign8540_body18_e8574,) = {
    if ((var_guard74 == 0.0) && (var_guard104 != 0.0)) {
        let assign8540_body18_e8572: f64 = (var_lp_s0 + 1.0);
        (assign8540_body18_e8572,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign8540_body18_e8574;
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

        *var_flg_depmode_slot = var_flg_depmode;
        *var_guard104_slot = var_guard104;
        *var_guard105_slot = var_guard105;
        *var_guard106_slot = var_guard106;
        *var_guard107_slot = var_guard107;
        *var_lp_s0_slot = var_lp_s0;
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
    }

    pub(super) fn stamp_transient_block_30(
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
        var_flg_depmode_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard110_slot: &mut f64,
        var_guard111_slot: &mut f64,
        var_guard112_slot: &mut f64,
        var_guard113_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
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
        var_ps_conv_ini_slot: &mut f64,
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
        var_q_s0_bulk_dn0_slot: &mut f64,
        var_q_s0_bulk_dn10_slot: &mut f64,
        var_q_s0_bulk_dn11_slot: &mut f64,
        var_q_s0_bulk_dn12_slot: &mut f64,
        var_q_s0_bulk_dn2_slot: &mut f64,
        var_q_s0_bulk_dn4_slot: &mut f64,
        var_q_s0_bulk_dn5_slot: &mut f64,
        var_q_s0_bulk_dn6_slot: &mut f64,
        var_q_s0_bulk_dn8_slot: &mut f64,
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
    ) {
        let mut var_flg_depmode: f64 = *var_flg_depmode_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard111: f64 = *var_guard111_slot;
        let mut var_guard112: f64 = *var_guard112_slot;
        let mut var_guard113: f64 = *var_guard113_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
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
        let mut var_ps_conv_ini: f64 = *var_ps_conv_ini_slot;
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
        let mut var_q_s0_bulk_dn0: f64 = *var_q_s0_bulk_dn0_slot;
        let mut var_q_s0_bulk_dn10: f64 = *var_q_s0_bulk_dn10_slot;
        let mut var_q_s0_bulk_dn11: f64 = *var_q_s0_bulk_dn11_slot;
        let mut var_q_s0_bulk_dn12: f64 = *var_q_s0_bulk_dn12_slot;
        let mut var_q_s0_bulk_dn2: f64 = *var_q_s0_bulk_dn2_slot;
        let mut var_q_s0_bulk_dn4: f64 = *var_q_s0_bulk_dn4_slot;
        let mut var_q_s0_bulk_dn5: f64 = *var_q_s0_bulk_dn5_slot;
        let mut var_q_s0_bulk_dn6: f64 = *var_q_s0_bulk_dn6_slot;
        let mut var_q_s0_bulk_dn8: f64 = *var_q_s0_bulk_dn8_slot;
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

        let (assign8570_e8602,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        (2.0,)
    } else {
        (var_flg_depmode,)
    }
};
        var_flg_depmode = assign8570_e8602;

        let assign8580_e8605: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        var_guard108 = assign8580_e8605;

        let (assign8590_e8617,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard108 != 0.0)) {
        let assign8590_e8615: f64 = (1e-12 * 100.0);
        (assign8590_e8615,)
    } else {
        (var_ps_conv_ini,)
    }
};
        var_ps_conv_ini = assign8590_e8617;

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

        let (assign8610_e8638,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard108 == 0.0)) {
        (0.001,)
    } else {
        (var_ps_conv_ini,)
    }
};
        var_ps_conv_ini = assign8610_e8638;

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

        let (assign8630_e8657,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        (0.0,)
    } else {
        (var_lp_s0,)
    }
};
        var_lp_s0 = assign8630_e8657;

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
            let assign8640_body3_e8697: f64 = if var_phi_s0_bulk > 1e-8 { 1.0 } else { 0.0 };
            var_guard109 = assign8640_body3_e8697;
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
            let assign8640_body7_e8758: f64 = (-1e-8);
            let assign8640_body7_e8759: f64 = if var_phi_s0_bulk < assign8640_body7_e8758 { 1.0 } else { 0.0 };
            var_guard110 = assign8640_body7_e8759;
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
            let assign8640_body13_e8883: f64 = (var_t6 - var_phi_s0_bulk);
            let assign8640_body13_e8884: f64 = (assign8640_body13_e8883).abs();
            let assign8640_body13_e8886: f64 = if assign8640_body13_e8884 < var_ps_conv_ini { 1.0 } else { 0.0 };
            var_guard111 = assign8640_body13_e8886;
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
            let (assign8640_body15_e8906,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard111 != 0.0)) {
        (var_lp_s0_max,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign8640_body15_e8906;
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
            let (assign8640_body18_e8932,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        let assign8640_body18_e8930: f64 = (var_lp_s0 + 1.0);
        (assign8640_body18_e8930,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign8640_body18_e8932;
        }

        let assign8650_e8935: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        var_guard112 = assign8650_e8935;

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

        let assign8670_e8948: f64 = if 1.0 == 0.0 { 1.0 } else { 0.0 };
        var_guard113 = assign8670_e8948;

        let (assign8680_e8960,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard113 != 0.0)) {
        let assign8680_e8958: f64 = (1e-12 * 100.0);
        (assign8680_e8958,)
    } else {
        (var_ps_conv_ini,)
    }
};
        var_ps_conv_ini = assign8680_e8960;

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

        let (assign8700_e8981,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard113 == 0.0)) {
        (0.001,)
    } else {
        (var_ps_conv_ini,)
    }
};
        var_ps_conv_ini = assign8700_e8981;

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

        let (assign8720_e9000,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        (0.0,)
    } else {
        (var_lp_s0,)
    }
};
        var_lp_s0 = assign8720_e9000;

        *var_flg_depmode_slot = var_flg_depmode;
        *var_guard108_slot = var_guard108;
        *var_guard109_slot = var_guard109;
        *var_guard110_slot = var_guard110;
        *var_guard111_slot = var_guard111;
        *var_guard112_slot = var_guard112;
        *var_guard113_slot = var_guard113;
        *var_lp_s0_slot = var_lp_s0;
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
        *var_ps_conv_ini_slot = var_ps_conv_ini;
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
        *var_q_s0_bulk_dn0_slot = var_q_s0_bulk_dn0;
        *var_q_s0_bulk_dn10_slot = var_q_s0_bulk_dn10;
        *var_q_s0_bulk_dn11_slot = var_q_s0_bulk_dn11;
        *var_q_s0_bulk_dn12_slot = var_q_s0_bulk_dn12;
        *var_q_s0_bulk_dn2_slot = var_q_s0_bulk_dn2;
        *var_q_s0_bulk_dn4_slot = var_q_s0_bulk_dn4;
        *var_q_s0_bulk_dn5_slot = var_q_s0_bulk_dn5;
        *var_q_s0_bulk_dn6_slot = var_q_s0_bulk_dn6;
        *var_q_s0_bulk_dn8_slot = var_q_s0_bulk_dn8;
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
    }

    pub(super) fn stamp_transient_block_31(
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
        var_guard115_slot: &mut f64,
        var_guard116_slot: &mut f64,
        var_guard117_slot: &mut f64,
        var_guard118_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
        var_lp_sl_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_mm_slot: &mut f64,
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
        var_q_s0_bulk_dn0_slot: &mut f64,
        var_q_s0_bulk_dn10_slot: &mut f64,
        var_q_s0_bulk_dn11_slot: &mut f64,
        var_q_s0_bulk_dn12_slot: &mut f64,
        var_q_s0_bulk_dn2_slot: &mut f64,
        var_q_s0_bulk_dn4_slot: &mut f64,
        var_q_s0_bulk_dn5_slot: &mut f64,
        var_q_s0_bulk_dn6_slot: &mut f64,
        var_q_s0_bulk_dn8_slot: &mut f64,
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
    ) {
        let mut var_guard114: f64 = *var_guard114_slot;
        let mut var_guard115: f64 = *var_guard115_slot;
        let mut var_guard116: f64 = *var_guard116_slot;
        let mut var_guard117: f64 = *var_guard117_slot;
        let mut var_guard118: f64 = *var_guard118_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
        let mut var_lp_sl: f64 = *var_lp_sl_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_mm: f64 = *var_mm_slot;
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
        let mut var_q_s0_bulk_dn0: f64 = *var_q_s0_bulk_dn0_slot;
        let mut var_q_s0_bulk_dn10: f64 = *var_q_s0_bulk_dn10_slot;
        let mut var_q_s0_bulk_dn11: f64 = *var_q_s0_bulk_dn11_slot;
        let mut var_q_s0_bulk_dn12: f64 = *var_q_s0_bulk_dn12_slot;
        let mut var_q_s0_bulk_dn2: f64 = *var_q_s0_bulk_dn2_slot;
        let mut var_q_s0_bulk_dn4: f64 = *var_q_s0_bulk_dn4_slot;
        let mut var_q_s0_bulk_dn5: f64 = *var_q_s0_bulk_dn5_slot;
        let mut var_q_s0_bulk_dn6: f64 = *var_q_s0_bulk_dn6_slot;
        let mut var_q_s0_bulk_dn8: f64 = *var_q_s0_bulk_dn8_slot;
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
            let assign8730_body3_e9040: f64 = if var_phi_s0_bulk > 1e-8 { 1.0 } else { 0.0 };
            var_guard114 = assign8730_body3_e9040;
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
            let assign8730_body7_e9101: f64 = (-1e-8);
            let assign8730_body7_e9102: f64 = if var_phi_s0_bulk < assign8730_body7_e9101 { 1.0 } else { 0.0 };
            var_guard115 = assign8730_body7_e9102;
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
            let assign8730_body13_e9226: f64 = (var_t6 - var_phi_s0_bulk);
            let assign8730_body13_e9227: f64 = (assign8730_body13_e9226).abs();
            let assign8730_body13_e9229: f64 = if assign8730_body13_e9227 < var_ps_conv_ini { 1.0 } else { 0.0 };
            var_guard116 = assign8730_body13_e9229;
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
            let (assign8730_body15_e9249,) = {
    if (((var_guard74 == 0.0) && (var_guard104 == 0.0)) && (var_guard116 != 0.0)) {
        (var_lp_s0_max,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign8730_body15_e9249;
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
            let (assign8730_body18_e9275,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        let assign8730_body18_e9273: f64 = (var_lp_s0 + 1.0);
        (assign8730_body18_e9273,)
    } else {
        (var_lp_s0,)
    }
};
            var_lp_s0 = assign8730_body18_e9275;
        }

        let assign8740_e9278: f64 = if 1.0 == 0.0 { 1.0 } else { 0.0 };
        var_guard117 = assign8740_e9278;

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

        let (assign8760_e9296,) = {
    if ((var_guard74 == 0.0) && (var_guard104 == 0.0)) {
        (0.0,)
    } else {
        (var_lp_sl,)
    }
};
        var_lp_sl = assign8760_e9296;

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

        let assign8790_e9318: f64 = (var_phi_s0_soi - 0.15);
        let assign8790_e9323: f64 = if ((var_phi_b0_soi > assign8790_e9318) && (0.15 >= 0.0)) { 1.0 } else { 0.0 };
        var_guard118 = assign8790_e9323;

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

        let (assign8850_e9373,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign8850_e9373;

        let (assign8860_e9380,) = {
    if ((var_guard74 == 0.0) && (var_guard118 != 0.0)) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign8860_e9380;

        *var_guard114_slot = var_guard114;
        *var_guard115_slot = var_guard115;
        *var_guard116_slot = var_guard116;
        *var_guard117_slot = var_guard117;
        *var_guard118_slot = var_guard118;
        *var_lp_s0_slot = var_lp_s0;
        *var_lp_sl_slot = var_lp_sl;
        *var_m0_slot = var_m0;
        *var_mm_slot = var_mm;
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
        *var_q_s0_bulk_dn0_slot = var_q_s0_bulk_dn0;
        *var_q_s0_bulk_dn10_slot = var_q_s0_bulk_dn10;
        *var_q_s0_bulk_dn11_slot = var_q_s0_bulk_dn11;
        *var_q_s0_bulk_dn12_slot = var_q_s0_bulk_dn12;
        *var_q_s0_bulk_dn2_slot = var_q_s0_bulk_dn2;
        *var_q_s0_bulk_dn4_slot = var_q_s0_bulk_dn4;
        *var_q_s0_bulk_dn5_slot = var_q_s0_bulk_dn5;
        *var_q_s0_bulk_dn6_slot = var_q_s0_bulk_dn6;
        *var_q_s0_bulk_dn8_slot = var_q_s0_bulk_dn8;
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
    }
}
