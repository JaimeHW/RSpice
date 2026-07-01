#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_256(
        var_a_factrp: f64,
        var_a_factrp_dn12: f64,
        var_a_factrp_dn13: f64,
        var_a_factrp_dn14: f64,
        var_a_factrp_dn15: f64,
        var_a_factrp_dn16: f64,
        var_a_factrp_dn17: f64,
        var_a_factrp_dn18: f64,
        var_a_factrp_dn19: f64,
        var_a_factrp_dn20: f64,
        var_a_factrp_dn5: f64,
        var_a_factrp_dn6: f64,
        var_a_factrp_dn7: f64,
        var_a_factrp_dn8: f64,
        var_gp2: f64,
        var_gp2_dn12: f64,
        var_gp2_dn13: f64,
        var_gp2_dn14: f64,
        var_gp2_dn15: f64,
        var_gp2_dn16: f64,
        var_gp2_dn17: f64,
        var_gp2_dn18: f64,
        var_gp2_dn19: f64,
        var_gp2_dn20: f64,
        var_gp2_dn5: f64,
        var_gp2_dn6: f64,
        var_gp2_dn7: f64,
        var_gp2_dn8: f64,
        var_guard2078: f64,
        var_guard2079: f64,
        var_guard2088: f64,
        var_guard2105: f64,
        var_guard2130: f64,
        var_guard2171: f64,
        var_guard2228: f64,
        var_guard2229: f64,
        var_guard2234: f64,
        var_guard2235: f64,
        var_marginp: f64,
        var_nqs_x0: f64,
        var_nqs_x0_dn12: f64,
        var_nqs_x0_dn13: f64,
        var_nqs_x0_dn14: f64,
        var_nqs_x0_dn15: f64,
        var_nqs_x0_dn16: f64,
        var_nqs_x0_dn17: f64,
        var_nqs_x0_dn18: f64,
        var_nqs_x0_dn19: f64,
        var_nqs_x0_dn20: f64,
        var_nqs_x0_dn5: f64,
        var_nqs_x0_dn6: f64,
        var_nqs_x0_dn7: f64,
        var_nqs_x0_dn8: f64,
        var_pd: f64,
        var_pd_dn12: f64,
        var_pd_dn13: f64,
        var_pd_dn14: f64,
        var_pd_dn15: f64,
        var_pd_dn16: f64,
        var_pd_dn17: f64,
        var_pd_dn18: f64,
        var_pd_dn19: f64,
        var_pd_dn20: f64,
        var_pd_dn5: f64,
        var_pd_dn6: f64,
        var_pd_dn7: f64,
        var_pd_dn8: f64,
        var_qp9: f64,
        var_qp9_dn20: f64,
        var_xg_ac: f64,
        var_xg_ac_dn12: f64,
        var_xg_ac_dn13: f64,
        var_xg_ac_dn14: f64,
        var_xg_ac_dn15: f64,
        var_xg_ac_dn16: f64,
        var_xg_ac_dn17: f64,
        var_xg_ac_dn18: f64,
        var_xg_ac_dn19: f64,
        var_xg_ac_dn20: f64,
        var_xg_ac_dn5: f64,
        var_xg_ac_dn6: f64,
        var_xg_ac_dn7: f64,
        var_xg_ac_dn8: f64,
        var_guard2236_slot: &mut f64,
        var_guard2237_slot: &mut f64,
        var_guard2238_slot: &mut f64,
        var_mutau_slot: &mut f64,
        var_mutau_dn12_slot: &mut f64,
        var_mutau_dn13_slot: &mut f64,
        var_mutau_dn14_slot: &mut f64,
        var_mutau_dn15_slot: &mut f64,
        var_mutau_dn16_slot: &mut f64,
        var_mutau_dn17_slot: &mut f64,
        var_mutau_dn18_slot: &mut f64,
        var_mutau_dn19_slot: &mut f64,
        var_mutau_dn20_slot: &mut f64,
        var_mutau_dn5_slot: &mut f64,
        var_mutau_dn6_slot: &mut f64,
        var_mutau_dn7_slot: &mut f64,
        var_mutau_dn8_slot: &mut f64,
        var_nqs_a_slot: &mut f64,
        var_nqs_a_dn12_slot: &mut f64,
        var_nqs_a_dn13_slot: &mut f64,
        var_nqs_a_dn14_slot: &mut f64,
        var_nqs_a_dn15_slot: &mut f64,
        var_nqs_a_dn16_slot: &mut f64,
        var_nqs_a_dn17_slot: &mut f64,
        var_nqs_a_dn18_slot: &mut f64,
        var_nqs_a_dn19_slot: &mut f64,
        var_nqs_a_dn20_slot: &mut f64,
        var_nqs_a_dn5_slot: &mut f64,
        var_nqs_a_dn6_slot: &mut f64,
        var_nqs_a_dn7_slot: &mut f64,
        var_nqs_a_dn8_slot: &mut f64,
        var_nqs_c_slot: &mut f64,
        var_nqs_c_dn12_slot: &mut f64,
        var_nqs_c_dn13_slot: &mut f64,
        var_nqs_c_dn14_slot: &mut f64,
        var_nqs_c_dn15_slot: &mut f64,
        var_nqs_c_dn16_slot: &mut f64,
        var_nqs_c_dn17_slot: &mut f64,
        var_nqs_c_dn18_slot: &mut f64,
        var_nqs_c_dn19_slot: &mut f64,
        var_nqs_c_dn20_slot: &mut f64,
        var_nqs_c_dn5_slot: &mut f64,
        var_nqs_c_dn6_slot: &mut f64,
        var_nqs_c_dn7_slot: &mut f64,
        var_nqs_c_dn8_slot: &mut f64,
        var_nqs_d0_slot: &mut f64,
        var_nqs_d0_dn12_slot: &mut f64,
        var_nqs_d0_dn13_slot: &mut f64,
        var_nqs_d0_dn14_slot: &mut f64,
        var_nqs_d0_dn15_slot: &mut f64,
        var_nqs_d0_dn16_slot: &mut f64,
        var_nqs_d0_dn17_slot: &mut f64,
        var_nqs_d0_dn18_slot: &mut f64,
        var_nqs_d0_dn19_slot: &mut f64,
        var_nqs_d0_dn20_slot: &mut f64,
        var_nqs_d0_dn5_slot: &mut f64,
        var_nqs_d0_dn6_slot: &mut f64,
        var_nqs_d0_dn7_slot: &mut f64,
        var_nqs_d0_dn8_slot: &mut f64,
        var_nqs_eta_slot: &mut f64,
        var_nqs_eta_dn12_slot: &mut f64,
        var_nqs_eta_dn13_slot: &mut f64,
        var_nqs_eta_dn14_slot: &mut f64,
        var_nqs_eta_dn15_slot: &mut f64,
        var_nqs_eta_dn16_slot: &mut f64,
        var_nqs_eta_dn17_slot: &mut f64,
        var_nqs_eta_dn18_slot: &mut f64,
        var_nqs_eta_dn19_slot: &mut f64,
        var_nqs_eta_dn20_slot: &mut f64,
        var_nqs_eta_dn5_slot: &mut f64,
        var_nqs_eta_dn6_slot: &mut f64,
        var_nqs_eta_dn7_slot: &mut f64,
        var_nqs_eta_dn8_slot: &mut f64,
        var_nqs_p_slot: &mut f64,
        var_nqs_p_dn12_slot: &mut f64,
        var_nqs_p_dn13_slot: &mut f64,
        var_nqs_p_dn14_slot: &mut f64,
        var_nqs_p_dn15_slot: &mut f64,
        var_nqs_p_dn16_slot: &mut f64,
        var_nqs_p_dn17_slot: &mut f64,
        var_nqs_p_dn18_slot: &mut f64,
        var_nqs_p_dn19_slot: &mut f64,
        var_nqs_p_dn20_slot: &mut f64,
        var_nqs_p_dn5_slot: &mut f64,
        var_nqs_p_dn6_slot: &mut f64,
        var_nqs_p_dn7_slot: &mut f64,
        var_nqs_p_dn8_slot: &mut f64,
        var_nqs_q_slot: &mut f64,
        var_nqs_q_dn12_slot: &mut f64,
        var_nqs_q_dn13_slot: &mut f64,
        var_nqs_q_dn14_slot: &mut f64,
        var_nqs_q_dn15_slot: &mut f64,
        var_nqs_q_dn16_slot: &mut f64,
        var_nqs_q_dn17_slot: &mut f64,
        var_nqs_q_dn18_slot: &mut f64,
        var_nqs_q_dn19_slot: &mut f64,
        var_nqs_q_dn20_slot: &mut f64,
        var_nqs_q_dn5_slot: &mut f64,
        var_nqs_q_dn6_slot: &mut f64,
        var_nqs_q_dn7_slot: &mut f64,
        var_nqs_q_dn8_slot: &mut f64,
        var_nqs_tau_slot: &mut f64,
        var_nqs_tau_dn12_slot: &mut f64,
        var_nqs_tau_dn13_slot: &mut f64,
        var_nqs_tau_dn14_slot: &mut f64,
        var_nqs_tau_dn15_slot: &mut f64,
        var_nqs_tau_dn16_slot: &mut f64,
        var_nqs_tau_dn17_slot: &mut f64,
        var_nqs_tau_dn18_slot: &mut f64,
        var_nqs_tau_dn19_slot: &mut f64,
        var_nqs_tau_dn20_slot: &mut f64,
        var_nqs_tau_dn5_slot: &mut f64,
        var_nqs_tau_dn6_slot: &mut f64,
        var_nqs_tau_dn7_slot: &mut f64,
        var_nqs_tau_dn8_slot: &mut f64,
        var_nqs_temp_slot: &mut f64,
        var_nqs_temp_dn12_slot: &mut f64,
        var_nqs_temp_dn13_slot: &mut f64,
        var_nqs_temp_dn14_slot: &mut f64,
        var_nqs_temp_dn15_slot: &mut f64,
        var_nqs_temp_dn16_slot: &mut f64,
        var_nqs_temp_dn17_slot: &mut f64,
        var_nqs_temp_dn18_slot: &mut f64,
        var_nqs_temp_dn19_slot: &mut f64,
        var_nqs_temp_dn20_slot: &mut f64,
        var_nqs_temp_dn5_slot: &mut f64,
        var_nqs_temp_dn6_slot: &mut f64,
        var_nqs_temp_dn7_slot: &mut f64,
        var_nqs_temp_dn8_slot: &mut f64,
        var_nqs_u_slot: &mut f64,
        var_nqs_u_dn12_slot: &mut f64,
        var_nqs_u_dn13_slot: &mut f64,
        var_nqs_u_dn14_slot: &mut f64,
        var_nqs_u_dn15_slot: &mut f64,
        var_nqs_u_dn16_slot: &mut f64,
        var_nqs_u_dn17_slot: &mut f64,
        var_nqs_u_dn18_slot: &mut f64,
        var_nqs_u_dn19_slot: &mut f64,
        var_nqs_u_dn20_slot: &mut f64,
        var_nqs_u_dn5_slot: &mut f64,
        var_nqs_u_dn6_slot: &mut f64,
        var_nqs_u_dn7_slot: &mut f64,
        var_nqs_u_dn8_slot: &mut f64,
        var_nqs_xi_slot: &mut f64,
        var_nqs_xi_dn12_slot: &mut f64,
        var_nqs_xi_dn13_slot: &mut f64,
        var_nqs_xi_dn14_slot: &mut f64,
        var_nqs_xi_dn15_slot: &mut f64,
        var_nqs_xi_dn16_slot: &mut f64,
        var_nqs_xi_dn17_slot: &mut f64,
        var_nqs_xi_dn18_slot: &mut f64,
        var_nqs_xi_dn19_slot: &mut f64,
        var_nqs_xi_dn20_slot: &mut f64,
        var_nqs_xi_dn5_slot: &mut f64,
        var_nqs_xi_dn6_slot: &mut f64,
        var_nqs_xi_dn7_slot: &mut f64,
        var_nqs_xi_dn8_slot: &mut f64,
        var_nqs_y0_slot: &mut f64,
        var_nqs_y0_dn12_slot: &mut f64,
        var_nqs_y0_dn13_slot: &mut f64,
        var_nqs_y0_dn14_slot: &mut f64,
        var_nqs_y0_dn15_slot: &mut f64,
        var_nqs_y0_dn16_slot: &mut f64,
        var_nqs_y0_dn17_slot: &mut f64,
        var_nqs_y0_dn18_slot: &mut f64,
        var_nqs_y0_dn19_slot: &mut f64,
        var_nqs_y0_dn20_slot: &mut f64,
        var_nqs_y0_dn5_slot: &mut f64,
        var_nqs_y0_dn6_slot: &mut f64,
        var_nqs_y0_dn7_slot: &mut f64,
        var_nqs_y0_dn8_slot: &mut f64,
        var_nqs_yg_slot: &mut f64,
        var_nqs_yg_dn12_slot: &mut f64,
        var_nqs_yg_dn13_slot: &mut f64,
        var_nqs_yg_dn14_slot: &mut f64,
        var_nqs_yg_dn15_slot: &mut f64,
        var_nqs_yg_dn16_slot: &mut f64,
        var_nqs_yg_dn17_slot: &mut f64,
        var_nqs_yg_dn18_slot: &mut f64,
        var_nqs_yg_dn19_slot: &mut f64,
        var_nqs_yg_dn20_slot: &mut f64,
        var_nqs_yg_dn5_slot: &mut f64,
        var_nqs_yg_dn6_slot: &mut f64,
        var_nqs_yg_dn7_slot: &mut f64,
        var_nqs_yg_dn8_slot: &mut f64,
        var_nqs_z_slot: &mut f64,
        var_nqs_z_dn12_slot: &mut f64,
        var_nqs_z_dn13_slot: &mut f64,
        var_nqs_z_dn14_slot: &mut f64,
        var_nqs_z_dn15_slot: &mut f64,
        var_nqs_z_dn16_slot: &mut f64,
        var_nqs_z_dn17_slot: &mut f64,
        var_nqs_z_dn18_slot: &mut f64,
        var_nqs_z_dn19_slot: &mut f64,
        var_nqs_z_dn20_slot: &mut f64,
        var_nqs_z_dn5_slot: &mut f64,
        var_nqs_z_dn6_slot: &mut f64,
        var_nqs_z_dn7_slot: &mut f64,
        var_nqs_z_dn8_slot: &mut f64,
        var_nu_slot: &mut f64,
        var_nu_dn12_slot: &mut f64,
        var_nu_dn13_slot: &mut f64,
        var_nu_dn14_slot: &mut f64,
        var_nu_dn15_slot: &mut f64,
        var_nu_dn16_slot: &mut f64,
        var_nu_dn17_slot: &mut f64,
        var_nu_dn18_slot: &mut f64,
        var_nu_dn19_slot: &mut f64,
        var_nu_dn20_slot: &mut f64,
        var_nu_dn5_slot: &mut f64,
        var_nu_dn6_slot: &mut f64,
        var_nu_dn7_slot: &mut f64,
        var_nu_dn8_slot: &mut f64,
        var_temp8_slot: &mut f64,
        var_temp8_dn12_slot: &mut f64,
        var_temp8_dn13_slot: &mut f64,
        var_temp8_dn14_slot: &mut f64,
        var_temp8_dn15_slot: &mut f64,
        var_temp8_dn16_slot: &mut f64,
        var_temp8_dn17_slot: &mut f64,
        var_temp8_dn18_slot: &mut f64,
        var_temp8_dn19_slot: &mut f64,
        var_temp8_dn20_slot: &mut f64,
        var_temp8_dn5_slot: &mut f64,
        var_temp8_dn6_slot: &mut f64,
        var_temp8_dn7_slot: &mut f64,
        var_temp8_dn8_slot: &mut f64,
        var_temp9_slot: &mut f64,
        var_temp9_dn12_slot: &mut f64,
        var_temp9_dn13_slot: &mut f64,
        var_temp9_dn14_slot: &mut f64,
        var_temp9_dn15_slot: &mut f64,
        var_temp9_dn16_slot: &mut f64,
        var_temp9_dn17_slot: &mut f64,
        var_temp9_dn18_slot: &mut f64,
        var_temp9_dn19_slot: &mut f64,
        var_temp9_dn20_slot: &mut f64,
        var_temp9_dn5_slot: &mut f64,
        var_temp9_dn6_slot: &mut f64,
        var_temp9_dn7_slot: &mut f64,
        var_temp9_dn8_slot: &mut f64,
        var_temp__blk1038_slot: &mut f64,
        var_temp__blk1038_dn12_slot: &mut f64,
        var_temp__blk1038_dn13_slot: &mut f64,
        var_temp__blk1038_dn14_slot: &mut f64,
        var_temp__blk1038_dn15_slot: &mut f64,
        var_temp__blk1038_dn16_slot: &mut f64,
        var_temp__blk1038_dn17_slot: &mut f64,
        var_temp__blk1038_dn18_slot: &mut f64,
        var_temp__blk1038_dn19_slot: &mut f64,
        var_temp__blk1038_dn20_slot: &mut f64,
        var_temp__blk1038_dn5_slot: &mut f64,
        var_temp__blk1038_dn6_slot: &mut f64,
        var_temp__blk1038_dn7_slot: &mut f64,
        var_temp__blk1038_dn8_slot: &mut f64,
    ) {
        let mut var_guard2236: f64 = *var_guard2236_slot;
        let mut var_guard2237: f64 = *var_guard2237_slot;
        let mut var_guard2238: f64 = *var_guard2238_slot;
        let mut var_mutau: f64 = *var_mutau_slot;
        let mut var_mutau_dn12: f64 = *var_mutau_dn12_slot;
        let mut var_mutau_dn13: f64 = *var_mutau_dn13_slot;
        let mut var_mutau_dn14: f64 = *var_mutau_dn14_slot;
        let mut var_mutau_dn15: f64 = *var_mutau_dn15_slot;
        let mut var_mutau_dn16: f64 = *var_mutau_dn16_slot;
        let mut var_mutau_dn17: f64 = *var_mutau_dn17_slot;
        let mut var_mutau_dn18: f64 = *var_mutau_dn18_slot;
        let mut var_mutau_dn19: f64 = *var_mutau_dn19_slot;
        let mut var_mutau_dn20: f64 = *var_mutau_dn20_slot;
        let mut var_mutau_dn5: f64 = *var_mutau_dn5_slot;
        let mut var_mutau_dn6: f64 = *var_mutau_dn6_slot;
        let mut var_mutau_dn7: f64 = *var_mutau_dn7_slot;
        let mut var_mutau_dn8: f64 = *var_mutau_dn8_slot;
        let mut var_nqs_a: f64 = *var_nqs_a_slot;
        let mut var_nqs_a_dn12: f64 = *var_nqs_a_dn12_slot;
        let mut var_nqs_a_dn13: f64 = *var_nqs_a_dn13_slot;
        let mut var_nqs_a_dn14: f64 = *var_nqs_a_dn14_slot;
        let mut var_nqs_a_dn15: f64 = *var_nqs_a_dn15_slot;
        let mut var_nqs_a_dn16: f64 = *var_nqs_a_dn16_slot;
        let mut var_nqs_a_dn17: f64 = *var_nqs_a_dn17_slot;
        let mut var_nqs_a_dn18: f64 = *var_nqs_a_dn18_slot;
        let mut var_nqs_a_dn19: f64 = *var_nqs_a_dn19_slot;
        let mut var_nqs_a_dn20: f64 = *var_nqs_a_dn20_slot;
        let mut var_nqs_a_dn5: f64 = *var_nqs_a_dn5_slot;
        let mut var_nqs_a_dn6: f64 = *var_nqs_a_dn6_slot;
        let mut var_nqs_a_dn7: f64 = *var_nqs_a_dn7_slot;
        let mut var_nqs_a_dn8: f64 = *var_nqs_a_dn8_slot;
        let mut var_nqs_c: f64 = *var_nqs_c_slot;
        let mut var_nqs_c_dn12: f64 = *var_nqs_c_dn12_slot;
        let mut var_nqs_c_dn13: f64 = *var_nqs_c_dn13_slot;
        let mut var_nqs_c_dn14: f64 = *var_nqs_c_dn14_slot;
        let mut var_nqs_c_dn15: f64 = *var_nqs_c_dn15_slot;
        let mut var_nqs_c_dn16: f64 = *var_nqs_c_dn16_slot;
        let mut var_nqs_c_dn17: f64 = *var_nqs_c_dn17_slot;
        let mut var_nqs_c_dn18: f64 = *var_nqs_c_dn18_slot;
        let mut var_nqs_c_dn19: f64 = *var_nqs_c_dn19_slot;
        let mut var_nqs_c_dn20: f64 = *var_nqs_c_dn20_slot;
        let mut var_nqs_c_dn5: f64 = *var_nqs_c_dn5_slot;
        let mut var_nqs_c_dn6: f64 = *var_nqs_c_dn6_slot;
        let mut var_nqs_c_dn7: f64 = *var_nqs_c_dn7_slot;
        let mut var_nqs_c_dn8: f64 = *var_nqs_c_dn8_slot;
        let mut var_nqs_d0: f64 = *var_nqs_d0_slot;
        let mut var_nqs_d0_dn12: f64 = *var_nqs_d0_dn12_slot;
        let mut var_nqs_d0_dn13: f64 = *var_nqs_d0_dn13_slot;
        let mut var_nqs_d0_dn14: f64 = *var_nqs_d0_dn14_slot;
        let mut var_nqs_d0_dn15: f64 = *var_nqs_d0_dn15_slot;
        let mut var_nqs_d0_dn16: f64 = *var_nqs_d0_dn16_slot;
        let mut var_nqs_d0_dn17: f64 = *var_nqs_d0_dn17_slot;
        let mut var_nqs_d0_dn18: f64 = *var_nqs_d0_dn18_slot;
        let mut var_nqs_d0_dn19: f64 = *var_nqs_d0_dn19_slot;
        let mut var_nqs_d0_dn20: f64 = *var_nqs_d0_dn20_slot;
        let mut var_nqs_d0_dn5: f64 = *var_nqs_d0_dn5_slot;
        let mut var_nqs_d0_dn6: f64 = *var_nqs_d0_dn6_slot;
        let mut var_nqs_d0_dn7: f64 = *var_nqs_d0_dn7_slot;
        let mut var_nqs_d0_dn8: f64 = *var_nqs_d0_dn8_slot;
        let mut var_nqs_eta: f64 = *var_nqs_eta_slot;
        let mut var_nqs_eta_dn12: f64 = *var_nqs_eta_dn12_slot;
        let mut var_nqs_eta_dn13: f64 = *var_nqs_eta_dn13_slot;
        let mut var_nqs_eta_dn14: f64 = *var_nqs_eta_dn14_slot;
        let mut var_nqs_eta_dn15: f64 = *var_nqs_eta_dn15_slot;
        let mut var_nqs_eta_dn16: f64 = *var_nqs_eta_dn16_slot;
        let mut var_nqs_eta_dn17: f64 = *var_nqs_eta_dn17_slot;
        let mut var_nqs_eta_dn18: f64 = *var_nqs_eta_dn18_slot;
        let mut var_nqs_eta_dn19: f64 = *var_nqs_eta_dn19_slot;
        let mut var_nqs_eta_dn20: f64 = *var_nqs_eta_dn20_slot;
        let mut var_nqs_eta_dn5: f64 = *var_nqs_eta_dn5_slot;
        let mut var_nqs_eta_dn6: f64 = *var_nqs_eta_dn6_slot;
        let mut var_nqs_eta_dn7: f64 = *var_nqs_eta_dn7_slot;
        let mut var_nqs_eta_dn8: f64 = *var_nqs_eta_dn8_slot;
        let mut var_nqs_p: f64 = *var_nqs_p_slot;
        let mut var_nqs_p_dn12: f64 = *var_nqs_p_dn12_slot;
        let mut var_nqs_p_dn13: f64 = *var_nqs_p_dn13_slot;
        let mut var_nqs_p_dn14: f64 = *var_nqs_p_dn14_slot;
        let mut var_nqs_p_dn15: f64 = *var_nqs_p_dn15_slot;
        let mut var_nqs_p_dn16: f64 = *var_nqs_p_dn16_slot;
        let mut var_nqs_p_dn17: f64 = *var_nqs_p_dn17_slot;
        let mut var_nqs_p_dn18: f64 = *var_nqs_p_dn18_slot;
        let mut var_nqs_p_dn19: f64 = *var_nqs_p_dn19_slot;
        let mut var_nqs_p_dn20: f64 = *var_nqs_p_dn20_slot;
        let mut var_nqs_p_dn5: f64 = *var_nqs_p_dn5_slot;
        let mut var_nqs_p_dn6: f64 = *var_nqs_p_dn6_slot;
        let mut var_nqs_p_dn7: f64 = *var_nqs_p_dn7_slot;
        let mut var_nqs_p_dn8: f64 = *var_nqs_p_dn8_slot;
        let mut var_nqs_q: f64 = *var_nqs_q_slot;
        let mut var_nqs_q_dn12: f64 = *var_nqs_q_dn12_slot;
        let mut var_nqs_q_dn13: f64 = *var_nqs_q_dn13_slot;
        let mut var_nqs_q_dn14: f64 = *var_nqs_q_dn14_slot;
        let mut var_nqs_q_dn15: f64 = *var_nqs_q_dn15_slot;
        let mut var_nqs_q_dn16: f64 = *var_nqs_q_dn16_slot;
        let mut var_nqs_q_dn17: f64 = *var_nqs_q_dn17_slot;
        let mut var_nqs_q_dn18: f64 = *var_nqs_q_dn18_slot;
        let mut var_nqs_q_dn19: f64 = *var_nqs_q_dn19_slot;
        let mut var_nqs_q_dn20: f64 = *var_nqs_q_dn20_slot;
        let mut var_nqs_q_dn5: f64 = *var_nqs_q_dn5_slot;
        let mut var_nqs_q_dn6: f64 = *var_nqs_q_dn6_slot;
        let mut var_nqs_q_dn7: f64 = *var_nqs_q_dn7_slot;
        let mut var_nqs_q_dn8: f64 = *var_nqs_q_dn8_slot;
        let mut var_nqs_tau: f64 = *var_nqs_tau_slot;
        let mut var_nqs_tau_dn12: f64 = *var_nqs_tau_dn12_slot;
        let mut var_nqs_tau_dn13: f64 = *var_nqs_tau_dn13_slot;
        let mut var_nqs_tau_dn14: f64 = *var_nqs_tau_dn14_slot;
        let mut var_nqs_tau_dn15: f64 = *var_nqs_tau_dn15_slot;
        let mut var_nqs_tau_dn16: f64 = *var_nqs_tau_dn16_slot;
        let mut var_nqs_tau_dn17: f64 = *var_nqs_tau_dn17_slot;
        let mut var_nqs_tau_dn18: f64 = *var_nqs_tau_dn18_slot;
        let mut var_nqs_tau_dn19: f64 = *var_nqs_tau_dn19_slot;
        let mut var_nqs_tau_dn20: f64 = *var_nqs_tau_dn20_slot;
        let mut var_nqs_tau_dn5: f64 = *var_nqs_tau_dn5_slot;
        let mut var_nqs_tau_dn6: f64 = *var_nqs_tau_dn6_slot;
        let mut var_nqs_tau_dn7: f64 = *var_nqs_tau_dn7_slot;
        let mut var_nqs_tau_dn8: f64 = *var_nqs_tau_dn8_slot;
        let mut var_nqs_temp: f64 = *var_nqs_temp_slot;
        let mut var_nqs_temp_dn12: f64 = *var_nqs_temp_dn12_slot;
        let mut var_nqs_temp_dn13: f64 = *var_nqs_temp_dn13_slot;
        let mut var_nqs_temp_dn14: f64 = *var_nqs_temp_dn14_slot;
        let mut var_nqs_temp_dn15: f64 = *var_nqs_temp_dn15_slot;
        let mut var_nqs_temp_dn16: f64 = *var_nqs_temp_dn16_slot;
        let mut var_nqs_temp_dn17: f64 = *var_nqs_temp_dn17_slot;
        let mut var_nqs_temp_dn18: f64 = *var_nqs_temp_dn18_slot;
        let mut var_nqs_temp_dn19: f64 = *var_nqs_temp_dn19_slot;
        let mut var_nqs_temp_dn20: f64 = *var_nqs_temp_dn20_slot;
        let mut var_nqs_temp_dn5: f64 = *var_nqs_temp_dn5_slot;
        let mut var_nqs_temp_dn6: f64 = *var_nqs_temp_dn6_slot;
        let mut var_nqs_temp_dn7: f64 = *var_nqs_temp_dn7_slot;
        let mut var_nqs_temp_dn8: f64 = *var_nqs_temp_dn8_slot;
        let mut var_nqs_u: f64 = *var_nqs_u_slot;
        let mut var_nqs_u_dn12: f64 = *var_nqs_u_dn12_slot;
        let mut var_nqs_u_dn13: f64 = *var_nqs_u_dn13_slot;
        let mut var_nqs_u_dn14: f64 = *var_nqs_u_dn14_slot;
        let mut var_nqs_u_dn15: f64 = *var_nqs_u_dn15_slot;
        let mut var_nqs_u_dn16: f64 = *var_nqs_u_dn16_slot;
        let mut var_nqs_u_dn17: f64 = *var_nqs_u_dn17_slot;
        let mut var_nqs_u_dn18: f64 = *var_nqs_u_dn18_slot;
        let mut var_nqs_u_dn19: f64 = *var_nqs_u_dn19_slot;
        let mut var_nqs_u_dn20: f64 = *var_nqs_u_dn20_slot;
        let mut var_nqs_u_dn5: f64 = *var_nqs_u_dn5_slot;
        let mut var_nqs_u_dn6: f64 = *var_nqs_u_dn6_slot;
        let mut var_nqs_u_dn7: f64 = *var_nqs_u_dn7_slot;
        let mut var_nqs_u_dn8: f64 = *var_nqs_u_dn8_slot;
        let mut var_nqs_xi: f64 = *var_nqs_xi_slot;
        let mut var_nqs_xi_dn12: f64 = *var_nqs_xi_dn12_slot;
        let mut var_nqs_xi_dn13: f64 = *var_nqs_xi_dn13_slot;
        let mut var_nqs_xi_dn14: f64 = *var_nqs_xi_dn14_slot;
        let mut var_nqs_xi_dn15: f64 = *var_nqs_xi_dn15_slot;
        let mut var_nqs_xi_dn16: f64 = *var_nqs_xi_dn16_slot;
        let mut var_nqs_xi_dn17: f64 = *var_nqs_xi_dn17_slot;
        let mut var_nqs_xi_dn18: f64 = *var_nqs_xi_dn18_slot;
        let mut var_nqs_xi_dn19: f64 = *var_nqs_xi_dn19_slot;
        let mut var_nqs_xi_dn20: f64 = *var_nqs_xi_dn20_slot;
        let mut var_nqs_xi_dn5: f64 = *var_nqs_xi_dn5_slot;
        let mut var_nqs_xi_dn6: f64 = *var_nqs_xi_dn6_slot;
        let mut var_nqs_xi_dn7: f64 = *var_nqs_xi_dn7_slot;
        let mut var_nqs_xi_dn8: f64 = *var_nqs_xi_dn8_slot;
        let mut var_nqs_y0: f64 = *var_nqs_y0_slot;
        let mut var_nqs_y0_dn12: f64 = *var_nqs_y0_dn12_slot;
        let mut var_nqs_y0_dn13: f64 = *var_nqs_y0_dn13_slot;
        let mut var_nqs_y0_dn14: f64 = *var_nqs_y0_dn14_slot;
        let mut var_nqs_y0_dn15: f64 = *var_nqs_y0_dn15_slot;
        let mut var_nqs_y0_dn16: f64 = *var_nqs_y0_dn16_slot;
        let mut var_nqs_y0_dn17: f64 = *var_nqs_y0_dn17_slot;
        let mut var_nqs_y0_dn18: f64 = *var_nqs_y0_dn18_slot;
        let mut var_nqs_y0_dn19: f64 = *var_nqs_y0_dn19_slot;
        let mut var_nqs_y0_dn20: f64 = *var_nqs_y0_dn20_slot;
        let mut var_nqs_y0_dn5: f64 = *var_nqs_y0_dn5_slot;
        let mut var_nqs_y0_dn6: f64 = *var_nqs_y0_dn6_slot;
        let mut var_nqs_y0_dn7: f64 = *var_nqs_y0_dn7_slot;
        let mut var_nqs_y0_dn8: f64 = *var_nqs_y0_dn8_slot;
        let mut var_nqs_yg: f64 = *var_nqs_yg_slot;
        let mut var_nqs_yg_dn12: f64 = *var_nqs_yg_dn12_slot;
        let mut var_nqs_yg_dn13: f64 = *var_nqs_yg_dn13_slot;
        let mut var_nqs_yg_dn14: f64 = *var_nqs_yg_dn14_slot;
        let mut var_nqs_yg_dn15: f64 = *var_nqs_yg_dn15_slot;
        let mut var_nqs_yg_dn16: f64 = *var_nqs_yg_dn16_slot;
        let mut var_nqs_yg_dn17: f64 = *var_nqs_yg_dn17_slot;
        let mut var_nqs_yg_dn18: f64 = *var_nqs_yg_dn18_slot;
        let mut var_nqs_yg_dn19: f64 = *var_nqs_yg_dn19_slot;
        let mut var_nqs_yg_dn20: f64 = *var_nqs_yg_dn20_slot;
        let mut var_nqs_yg_dn5: f64 = *var_nqs_yg_dn5_slot;
        let mut var_nqs_yg_dn6: f64 = *var_nqs_yg_dn6_slot;
        let mut var_nqs_yg_dn7: f64 = *var_nqs_yg_dn7_slot;
        let mut var_nqs_yg_dn8: f64 = *var_nqs_yg_dn8_slot;
        let mut var_nqs_z: f64 = *var_nqs_z_slot;
        let mut var_nqs_z_dn12: f64 = *var_nqs_z_dn12_slot;
        let mut var_nqs_z_dn13: f64 = *var_nqs_z_dn13_slot;
        let mut var_nqs_z_dn14: f64 = *var_nqs_z_dn14_slot;
        let mut var_nqs_z_dn15: f64 = *var_nqs_z_dn15_slot;
        let mut var_nqs_z_dn16: f64 = *var_nqs_z_dn16_slot;
        let mut var_nqs_z_dn17: f64 = *var_nqs_z_dn17_slot;
        let mut var_nqs_z_dn18: f64 = *var_nqs_z_dn18_slot;
        let mut var_nqs_z_dn19: f64 = *var_nqs_z_dn19_slot;
        let mut var_nqs_z_dn20: f64 = *var_nqs_z_dn20_slot;
        let mut var_nqs_z_dn5: f64 = *var_nqs_z_dn5_slot;
        let mut var_nqs_z_dn6: f64 = *var_nqs_z_dn6_slot;
        let mut var_nqs_z_dn7: f64 = *var_nqs_z_dn7_slot;
        let mut var_nqs_z_dn8: f64 = *var_nqs_z_dn8_slot;
        let mut var_nu: f64 = *var_nu_slot;
        let mut var_nu_dn12: f64 = *var_nu_dn12_slot;
        let mut var_nu_dn13: f64 = *var_nu_dn13_slot;
        let mut var_nu_dn14: f64 = *var_nu_dn14_slot;
        let mut var_nu_dn15: f64 = *var_nu_dn15_slot;
        let mut var_nu_dn16: f64 = *var_nu_dn16_slot;
        let mut var_nu_dn17: f64 = *var_nu_dn17_slot;
        let mut var_nu_dn18: f64 = *var_nu_dn18_slot;
        let mut var_nu_dn19: f64 = *var_nu_dn19_slot;
        let mut var_nu_dn20: f64 = *var_nu_dn20_slot;
        let mut var_nu_dn5: f64 = *var_nu_dn5_slot;
        let mut var_nu_dn6: f64 = *var_nu_dn6_slot;
        let mut var_nu_dn7: f64 = *var_nu_dn7_slot;
        let mut var_nu_dn8: f64 = *var_nu_dn8_slot;
        let mut var_temp8: f64 = *var_temp8_slot;
        let mut var_temp8_dn12: f64 = *var_temp8_dn12_slot;
        let mut var_temp8_dn13: f64 = *var_temp8_dn13_slot;
        let mut var_temp8_dn14: f64 = *var_temp8_dn14_slot;
        let mut var_temp8_dn15: f64 = *var_temp8_dn15_slot;
        let mut var_temp8_dn16: f64 = *var_temp8_dn16_slot;
        let mut var_temp8_dn17: f64 = *var_temp8_dn17_slot;
        let mut var_temp8_dn18: f64 = *var_temp8_dn18_slot;
        let mut var_temp8_dn19: f64 = *var_temp8_dn19_slot;
        let mut var_temp8_dn20: f64 = *var_temp8_dn20_slot;
        let mut var_temp8_dn5: f64 = *var_temp8_dn5_slot;
        let mut var_temp8_dn6: f64 = *var_temp8_dn6_slot;
        let mut var_temp8_dn7: f64 = *var_temp8_dn7_slot;
        let mut var_temp8_dn8: f64 = *var_temp8_dn8_slot;
        let mut var_temp9: f64 = *var_temp9_slot;
        let mut var_temp9_dn12: f64 = *var_temp9_dn12_slot;
        let mut var_temp9_dn13: f64 = *var_temp9_dn13_slot;
        let mut var_temp9_dn14: f64 = *var_temp9_dn14_slot;
        let mut var_temp9_dn15: f64 = *var_temp9_dn15_slot;
        let mut var_temp9_dn16: f64 = *var_temp9_dn16_slot;
        let mut var_temp9_dn17: f64 = *var_temp9_dn17_slot;
        let mut var_temp9_dn18: f64 = *var_temp9_dn18_slot;
        let mut var_temp9_dn19: f64 = *var_temp9_dn19_slot;
        let mut var_temp9_dn20: f64 = *var_temp9_dn20_slot;
        let mut var_temp9_dn5: f64 = *var_temp9_dn5_slot;
        let mut var_temp9_dn6: f64 = *var_temp9_dn6_slot;
        let mut var_temp9_dn7: f64 = *var_temp9_dn7_slot;
        let mut var_temp9_dn8: f64 = *var_temp9_dn8_slot;
        let mut var_temp__blk1038: f64 = *var_temp__blk1038_slot;
        let mut var_temp__blk1038_dn12: f64 = *var_temp__blk1038_dn12_slot;
        let mut var_temp__blk1038_dn13: f64 = *var_temp__blk1038_dn13_slot;
        let mut var_temp__blk1038_dn14: f64 = *var_temp__blk1038_dn14_slot;
        let mut var_temp__blk1038_dn15: f64 = *var_temp__blk1038_dn15_slot;
        let mut var_temp__blk1038_dn16: f64 = *var_temp__blk1038_dn16_slot;
        let mut var_temp__blk1038_dn17: f64 = *var_temp__blk1038_dn17_slot;
        let mut var_temp__blk1038_dn18: f64 = *var_temp__blk1038_dn18_slot;
        let mut var_temp__blk1038_dn19: f64 = *var_temp__blk1038_dn19_slot;
        let mut var_temp__blk1038_dn20: f64 = *var_temp__blk1038_dn20_slot;
        let mut var_temp__blk1038_dn5: f64 = *var_temp__blk1038_dn5_slot;
        let mut var_temp__blk1038_dn6: f64 = *var_temp__blk1038_dn6_slot;
        let mut var_temp__blk1038_dn7: f64 = *var_temp__blk1038_dn7_slot;
        let mut var_temp__blk1038_dn8: f64 = *var_temp__blk1038_dn8_slot;

        let (assign82100_e122840, assign82100_e122840_d_n5, assign82100_e122840_d_n6, assign82100_e122840_d_n7, assign82100_e122840_d_n8, assign82100_e122840_d_n12, assign82100_e122840_d_n13, assign82100_e122840_d_n14, assign82100_e122840_d_n15, assign82100_e122840_d_n16, assign82100_e122840_d_n17, assign82100_e122840_d_n18, assign82100_e122840_d_n19, assign82100_e122840_d_n20,) = {
    if ((((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2228 == 0.0)) && (var_guard2229 == 0.0)) && (var_guard2234 == 0.0)) && (var_guard2235 == 0.0)) {
        let assign82100_e122816: f64 = (-var_nqs_x0);
        let assign82100_e122818: f64 = (assign82100_e122816 - 230.25850929940458);
        let assign82100_e122822: f64 = (-var_nqs_x0);
        let assign82100_e122824: f64 = (assign82100_e122822 - 230.25850929940458);
        let assign82100_e122827: f64 = (-var_nqs_x0);
        let assign82100_e122829: f64 = (assign82100_e122827 - 230.25850929940458);
        let assign82100_e122831: f64 = (assign82100_e122829 * 0.3333333333333333);
        let assign82100_e122832: f64 = (1.0 + assign82100_e122831);
        let assign82100_e122833: f64 = (assign82100_e122824 * assign82100_e122832);
        let assign82100_e122834: f64 = (0.5 * assign82100_e122833);
        let assign82100_e122835: f64 = (1.0 + assign82100_e122834);
        let assign82100_e122836: f64 = (assign82100_e122818 * assign82100_e122835);
        let assign82100_e122837: f64 = (1.0 + assign82100_e122836);
        let assign82100_e122838: f64 = (1e100 * assign82100_e122837);
        (assign82100_e122838, (1e100 * (((-var_nqs_x0_dn5) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-var_nqs_x0_dn5) * assign82100_e122832) + (assign82100_e122824 * ((-var_nqs_x0_dn5) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn6) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-var_nqs_x0_dn6) * assign82100_e122832) + (assign82100_e122824 * ((-var_nqs_x0_dn6) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn7) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-var_nqs_x0_dn7) * assign82100_e122832) + (assign82100_e122824 * ((-var_nqs_x0_dn7) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn8) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-var_nqs_x0_dn8) * assign82100_e122832) + (assign82100_e122824 * ((-var_nqs_x0_dn8) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn12) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-var_nqs_x0_dn12) * assign82100_e122832) + (assign82100_e122824 * ((-var_nqs_x0_dn12) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn13) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-var_nqs_x0_dn13) * assign82100_e122832) + (assign82100_e122824 * ((-var_nqs_x0_dn13) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn14) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-var_nqs_x0_dn14) * assign82100_e122832) + (assign82100_e122824 * ((-var_nqs_x0_dn14) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn15) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-var_nqs_x0_dn15) * assign82100_e122832) + (assign82100_e122824 * ((-var_nqs_x0_dn15) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn16) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-var_nqs_x0_dn16) * assign82100_e122832) + (assign82100_e122824 * ((-var_nqs_x0_dn16) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn17) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-var_nqs_x0_dn17) * assign82100_e122832) + (assign82100_e122824 * ((-var_nqs_x0_dn17) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn18) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-var_nqs_x0_dn18) * assign82100_e122832) + (assign82100_e122824 * ((-var_nqs_x0_dn18) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn19) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-var_nqs_x0_dn19) * assign82100_e122832) + (assign82100_e122824 * ((-var_nqs_x0_dn19) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn20) * assign82100_e122835) + (assign82100_e122818 * (0.5 * (((-var_nqs_x0_dn20) * assign82100_e122832) + (assign82100_e122824 * ((-var_nqs_x0_dn20) * 0.3333333333333333))))))),)
    } else {
        (var_nqs_d0, var_nqs_d0_dn5, var_nqs_d0_dn6, var_nqs_d0_dn7, var_nqs_d0_dn8, var_nqs_d0_dn12, var_nqs_d0_dn13, var_nqs_d0_dn14, var_nqs_d0_dn15, var_nqs_d0_dn16, var_nqs_d0_dn17, var_nqs_d0_dn18, var_nqs_d0_dn19, var_nqs_d0_dn20,)
    }
};
        var_nqs_d0 = assign82100_e122840;
        var_nqs_d0_dn5 = assign82100_e122840_d_n5;
        var_nqs_d0_dn6 = assign82100_e122840_d_n6;
        var_nqs_d0_dn7 = assign82100_e122840_d_n7;
        var_nqs_d0_dn8 = assign82100_e122840_d_n8;
        var_nqs_d0_dn12 = assign82100_e122840_d_n12;
        var_nqs_d0_dn13 = assign82100_e122840_d_n13;
        var_nqs_d0_dn14 = assign82100_e122840_d_n14;
        var_nqs_d0_dn15 = assign82100_e122840_d_n15;
        var_nqs_d0_dn16 = assign82100_e122840_d_n16;
        var_nqs_d0_dn17 = assign82100_e122840_d_n17;
        var_nqs_d0_dn18 = assign82100_e122840_d_n18;
        var_nqs_d0_dn19 = assign82100_e122840_d_n19;
        var_nqs_d0_dn20 = assign82100_e122840_d_n20;

        let (assign82110_e122870, assign82110_e122870_d_n5, assign82110_e122870_d_n6, assign82110_e122870_d_n7, assign82110_e122870_d_n8, assign82110_e122870_d_n12, assign82110_e122870_d_n13, assign82110_e122870_d_n14, assign82110_e122870_d_n15, assign82110_e122870_d_n16, assign82110_e122870_d_n17, assign82110_e122870_d_n18, assign82110_e122870_d_n19, assign82110_e122870_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2228 == 0.0)) && (var_guard2229 == 0.0)) {
        let assign82110_e122865: f64 = (var_gp2 * 0.5);
        let assign82110_e122867: f64 = (assign82110_e122865 * var_nqs_d0);
        let assign82110_e122868: f64 = (1.0 - assign82110_e122867);
        (assign82110_e122868, (-(((var_gp2_dn5 * 0.5) * var_nqs_d0) + (assign82110_e122865 * var_nqs_d0_dn5))), (-(((var_gp2_dn6 * 0.5) * var_nqs_d0) + (assign82110_e122865 * var_nqs_d0_dn6))), (-(((var_gp2_dn7 * 0.5) * var_nqs_d0) + (assign82110_e122865 * var_nqs_d0_dn7))), (-(((var_gp2_dn8 * 0.5) * var_nqs_d0) + (assign82110_e122865 * var_nqs_d0_dn8))), (-(((var_gp2_dn12 * 0.5) * var_nqs_d0) + (assign82110_e122865 * var_nqs_d0_dn12))), (-(((var_gp2_dn13 * 0.5) * var_nqs_d0) + (assign82110_e122865 * var_nqs_d0_dn13))), (-(((var_gp2_dn14 * 0.5) * var_nqs_d0) + (assign82110_e122865 * var_nqs_d0_dn14))), (-(((var_gp2_dn15 * 0.5) * var_nqs_d0) + (assign82110_e122865 * var_nqs_d0_dn15))), (-(((var_gp2_dn16 * 0.5) * var_nqs_d0) + (assign82110_e122865 * var_nqs_d0_dn16))), (-(((var_gp2_dn17 * 0.5) * var_nqs_d0) + (assign82110_e122865 * var_nqs_d0_dn17))), (-(((var_gp2_dn18 * 0.5) * var_nqs_d0) + (assign82110_e122865 * var_nqs_d0_dn18))), (-(((var_gp2_dn19 * 0.5) * var_nqs_d0) + (assign82110_e122865 * var_nqs_d0_dn19))), (-(((var_gp2_dn20 * 0.5) * var_nqs_d0) + (assign82110_e122865 * var_nqs_d0_dn20))),)
    } else {
        (var_nqs_xi, var_nqs_xi_dn5, var_nqs_xi_dn6, var_nqs_xi_dn7, var_nqs_xi_dn8, var_nqs_xi_dn12, var_nqs_xi_dn13, var_nqs_xi_dn14, var_nqs_xi_dn15, var_nqs_xi_dn16, var_nqs_xi_dn17, var_nqs_xi_dn18, var_nqs_xi_dn19, var_nqs_xi_dn20,)
    }
};
        var_nqs_xi = assign82110_e122870;
        var_nqs_xi_dn5 = assign82110_e122870_d_n5;
        var_nqs_xi_dn6 = assign82110_e122870_d_n6;
        var_nqs_xi_dn7 = assign82110_e122870_d_n7;
        var_nqs_xi_dn8 = assign82110_e122870_d_n8;
        var_nqs_xi_dn12 = assign82110_e122870_d_n12;
        var_nqs_xi_dn13 = assign82110_e122870_d_n13;
        var_nqs_xi_dn14 = assign82110_e122870_d_n14;
        var_nqs_xi_dn15 = assign82110_e122870_d_n15;
        var_nqs_xi_dn16 = assign82110_e122870_d_n16;
        var_nqs_xi_dn17 = assign82110_e122870_d_n17;
        var_nqs_xi_dn18 = assign82110_e122870_d_n18;
        var_nqs_xi_dn19 = assign82110_e122870_d_n19;
        var_nqs_xi_dn20 = assign82110_e122870_d_n20;

        let (assign82120_e122904, assign82120_e122904_d_n5, assign82120_e122904_d_n6, assign82120_e122904_d_n7, assign82120_e122904_d_n8, assign82120_e122904_d_n12, assign82120_e122904_d_n13, assign82120_e122904_d_n14, assign82120_e122904_d_n15, assign82120_e122904_d_n16, assign82120_e122904_d_n17, assign82120_e122904_d_n18, assign82120_e122904_d_n19, assign82120_e122904_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2228 == 0.0)) && (var_guard2229 == 0.0)) {
        let assign82120_e122895: f64 = (var_temp__blk1038 - var_nqs_x0);
        let assign82120_e122896: f64 = (2.0 * assign82120_e122895);
        let assign82120_e122900: f64 = (1.0 - var_nqs_d0);
        let assign82120_e122901: f64 = (var_gp2 * assign82120_e122900);
        let assign82120_e122902: f64 = (assign82120_e122896 + assign82120_e122901);
        (assign82120_e122902, ((2.0 * (var_temp__blk1038_dn5 - var_nqs_x0_dn5)) + ((var_gp2_dn5 * assign82120_e122900) + (var_gp2 * (-var_nqs_d0_dn5)))), ((2.0 * (var_temp__blk1038_dn6 - var_nqs_x0_dn6)) + ((var_gp2_dn6 * assign82120_e122900) + (var_gp2 * (-var_nqs_d0_dn6)))), ((2.0 * (var_temp__blk1038_dn7 - var_nqs_x0_dn7)) + ((var_gp2_dn7 * assign82120_e122900) + (var_gp2 * (-var_nqs_d0_dn7)))), ((2.0 * (var_temp__blk1038_dn8 - var_nqs_x0_dn8)) + ((var_gp2_dn8 * assign82120_e122900) + (var_gp2 * (-var_nqs_d0_dn8)))), ((2.0 * (var_temp__blk1038_dn12 - var_nqs_x0_dn12)) + ((var_gp2_dn12 * assign82120_e122900) + (var_gp2 * (-var_nqs_d0_dn12)))), ((2.0 * (var_temp__blk1038_dn13 - var_nqs_x0_dn13)) + ((var_gp2_dn13 * assign82120_e122900) + (var_gp2 * (-var_nqs_d0_dn13)))), ((2.0 * (var_temp__blk1038_dn14 - var_nqs_x0_dn14)) + ((var_gp2_dn14 * assign82120_e122900) + (var_gp2 * (-var_nqs_d0_dn14)))), ((2.0 * (var_temp__blk1038_dn15 - var_nqs_x0_dn15)) + ((var_gp2_dn15 * assign82120_e122900) + (var_gp2 * (-var_nqs_d0_dn15)))), ((2.0 * (var_temp__blk1038_dn16 - var_nqs_x0_dn16)) + ((var_gp2_dn16 * assign82120_e122900) + (var_gp2 * (-var_nqs_d0_dn16)))), ((2.0 * (var_temp__blk1038_dn17 - var_nqs_x0_dn17)) + ((var_gp2_dn17 * assign82120_e122900) + (var_gp2 * (-var_nqs_d0_dn17)))), ((2.0 * (var_temp__blk1038_dn18 - var_nqs_x0_dn18)) + ((var_gp2_dn18 * assign82120_e122900) + (var_gp2 * (-var_nqs_d0_dn18)))), ((2.0 * (var_temp__blk1038_dn19 - var_nqs_x0_dn19)) + ((var_gp2_dn19 * assign82120_e122900) + (var_gp2 * (-var_nqs_d0_dn19)))), ((2.0 * (var_temp__blk1038_dn20 - var_nqs_x0_dn20)) + ((var_gp2_dn20 * assign82120_e122900) + (var_gp2 * (-var_nqs_d0_dn20)))),)
    } else {
        (var_nqs_p, var_nqs_p_dn5, var_nqs_p_dn6, var_nqs_p_dn7, var_nqs_p_dn8, var_nqs_p_dn12, var_nqs_p_dn13, var_nqs_p_dn14, var_nqs_p_dn15, var_nqs_p_dn16, var_nqs_p_dn17, var_nqs_p_dn18, var_nqs_p_dn19, var_nqs_p_dn20,)
    }
};
        var_nqs_p = assign82120_e122904;
        var_nqs_p_dn5 = assign82120_e122904_d_n5;
        var_nqs_p_dn6 = assign82120_e122904_d_n6;
        var_nqs_p_dn7 = assign82120_e122904_d_n7;
        var_nqs_p_dn8 = assign82120_e122904_d_n8;
        var_nqs_p_dn12 = assign82120_e122904_d_n12;
        var_nqs_p_dn13 = assign82120_e122904_d_n13;
        var_nqs_p_dn14 = assign82120_e122904_d_n14;
        var_nqs_p_dn15 = assign82120_e122904_d_n15;
        var_nqs_p_dn16 = assign82120_e122904_d_n16;
        var_nqs_p_dn17 = assign82120_e122904_d_n17;
        var_nqs_p_dn18 = assign82120_e122904_d_n18;
        var_nqs_p_dn19 = assign82120_e122904_d_n19;
        var_nqs_p_dn20 = assign82120_e122904_d_n20;

        let (assign82130_e122942, assign82130_e122942_d_n5, assign82130_e122942_d_n6, assign82130_e122942_d_n7, assign82130_e122942_d_n8, assign82130_e122942_d_n12, assign82130_e122942_d_n13, assign82130_e122942_d_n14, assign82130_e122942_d_n15, assign82130_e122942_d_n16, assign82130_e122942_d_n17, assign82130_e122942_d_n18, assign82130_e122942_d_n19, assign82130_e122942_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2228 == 0.0)) && (var_guard2229 == 0.0)) {
        let assign82130_e122928: f64 = (var_temp__blk1038 - var_nqs_x0);
        let assign82130_e122931: f64 = (var_temp__blk1038 - var_nqs_x0);
        let assign82130_e122932: f64 = (assign82130_e122928 * assign82130_e122931);
        let assign82130_e122936: f64 = (var_nqs_x0 - 1.0);
        let assign82130_e122938: f64 = (assign82130_e122936 + var_nqs_d0);
        let assign82130_e122939: f64 = (var_gp2 * assign82130_e122938);
        let assign82130_e122940: f64 = (assign82130_e122932 - assign82130_e122939);
        (assign82130_e122940, ((((var_temp__blk1038_dn5 - var_nqs_x0_dn5) * assign82130_e122931) + (assign82130_e122928 * (var_temp__blk1038_dn5 - var_nqs_x0_dn5))) - ((var_gp2_dn5 * assign82130_e122938) + (var_gp2 * (var_nqs_x0_dn5 + var_nqs_d0_dn5)))), ((((var_temp__blk1038_dn6 - var_nqs_x0_dn6) * assign82130_e122931) + (assign82130_e122928 * (var_temp__blk1038_dn6 - var_nqs_x0_dn6))) - ((var_gp2_dn6 * assign82130_e122938) + (var_gp2 * (var_nqs_x0_dn6 + var_nqs_d0_dn6)))), ((((var_temp__blk1038_dn7 - var_nqs_x0_dn7) * assign82130_e122931) + (assign82130_e122928 * (var_temp__blk1038_dn7 - var_nqs_x0_dn7))) - ((var_gp2_dn7 * assign82130_e122938) + (var_gp2 * (var_nqs_x0_dn7 + var_nqs_d0_dn7)))), ((((var_temp__blk1038_dn8 - var_nqs_x0_dn8) * assign82130_e122931) + (assign82130_e122928 * (var_temp__blk1038_dn8 - var_nqs_x0_dn8))) - ((var_gp2_dn8 * assign82130_e122938) + (var_gp2 * (var_nqs_x0_dn8 + var_nqs_d0_dn8)))), ((((var_temp__blk1038_dn12 - var_nqs_x0_dn12) * assign82130_e122931) + (assign82130_e122928 * (var_temp__blk1038_dn12 - var_nqs_x0_dn12))) - ((var_gp2_dn12 * assign82130_e122938) + (var_gp2 * (var_nqs_x0_dn12 + var_nqs_d0_dn12)))), ((((var_temp__blk1038_dn13 - var_nqs_x0_dn13) * assign82130_e122931) + (assign82130_e122928 * (var_temp__blk1038_dn13 - var_nqs_x0_dn13))) - ((var_gp2_dn13 * assign82130_e122938) + (var_gp2 * (var_nqs_x0_dn13 + var_nqs_d0_dn13)))), ((((var_temp__blk1038_dn14 - var_nqs_x0_dn14) * assign82130_e122931) + (assign82130_e122928 * (var_temp__blk1038_dn14 - var_nqs_x0_dn14))) - ((var_gp2_dn14 * assign82130_e122938) + (var_gp2 * (var_nqs_x0_dn14 + var_nqs_d0_dn14)))), ((((var_temp__blk1038_dn15 - var_nqs_x0_dn15) * assign82130_e122931) + (assign82130_e122928 * (var_temp__blk1038_dn15 - var_nqs_x0_dn15))) - ((var_gp2_dn15 * assign82130_e122938) + (var_gp2 * (var_nqs_x0_dn15 + var_nqs_d0_dn15)))), ((((var_temp__blk1038_dn16 - var_nqs_x0_dn16) * assign82130_e122931) + (assign82130_e122928 * (var_temp__blk1038_dn16 - var_nqs_x0_dn16))) - ((var_gp2_dn16 * assign82130_e122938) + (var_gp2 * (var_nqs_x0_dn16 + var_nqs_d0_dn16)))), ((((var_temp__blk1038_dn17 - var_nqs_x0_dn17) * assign82130_e122931) + (assign82130_e122928 * (var_temp__blk1038_dn17 - var_nqs_x0_dn17))) - ((var_gp2_dn17 * assign82130_e122938) + (var_gp2 * (var_nqs_x0_dn17 + var_nqs_d0_dn17)))), ((((var_temp__blk1038_dn18 - var_nqs_x0_dn18) * assign82130_e122931) + (assign82130_e122928 * (var_temp__blk1038_dn18 - var_nqs_x0_dn18))) - ((var_gp2_dn18 * assign82130_e122938) + (var_gp2 * (var_nqs_x0_dn18 + var_nqs_d0_dn18)))), ((((var_temp__blk1038_dn19 - var_nqs_x0_dn19) * assign82130_e122931) + (assign82130_e122928 * (var_temp__blk1038_dn19 - var_nqs_x0_dn19))) - ((var_gp2_dn19 * assign82130_e122938) + (var_gp2 * (var_nqs_x0_dn19 + var_nqs_d0_dn19)))), ((((var_temp__blk1038_dn20 - var_nqs_x0_dn20) * assign82130_e122931) + (assign82130_e122928 * (var_temp__blk1038_dn20 - var_nqs_x0_dn20))) - ((var_gp2_dn20 * assign82130_e122938) + (var_gp2 * (var_nqs_x0_dn20 + var_nqs_d0_dn20)))),)
    } else {
        (var_nqs_q, var_nqs_q_dn5, var_nqs_q_dn6, var_nqs_q_dn7, var_nqs_q_dn8, var_nqs_q_dn12, var_nqs_q_dn13, var_nqs_q_dn14, var_nqs_q_dn15, var_nqs_q_dn16, var_nqs_q_dn17, var_nqs_q_dn18, var_nqs_q_dn19, var_nqs_q_dn20,)
    }
};
        var_nqs_q = assign82130_e122942;
        var_nqs_q_dn5 = assign82130_e122942_d_n5;
        var_nqs_q_dn6 = assign82130_e122942_d_n6;
        var_nqs_q_dn7 = assign82130_e122942_d_n7;
        var_nqs_q_dn8 = assign82130_e122942_d_n8;
        var_nqs_q_dn12 = assign82130_e122942_d_n12;
        var_nqs_q_dn13 = assign82130_e122942_d_n13;
        var_nqs_q_dn14 = assign82130_e122942_d_n14;
        var_nqs_q_dn15 = assign82130_e122942_d_n15;
        var_nqs_q_dn16 = assign82130_e122942_d_n16;
        var_nqs_q_dn17 = assign82130_e122942_d_n17;
        var_nqs_q_dn18 = assign82130_e122942_d_n18;
        var_nqs_q_dn19 = assign82130_e122942_d_n19;
        var_nqs_q_dn20 = assign82130_e122942_d_n20;

        let (assign82140_e122974, assign82140_e122974_d_n5, assign82140_e122974_d_n6, assign82140_e122974_d_n7, assign82140_e122974_d_n8, assign82140_e122974_d_n12, assign82140_e122974_d_n13, assign82140_e122974_d_n14, assign82140_e122974_d_n15, assign82140_e122974_d_n16, assign82140_e122974_d_n17, assign82140_e122974_d_n18, assign82140_e122974_d_n19, assign82140_e122974_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2228 == 0.0)) && (var_guard2229 == 0.0)) {
        let assign82140_e122966: f64 = (var_nqs_p * var_nqs_p);
        let assign82140_e122969: f64 = (4.0 * var_nqs_xi);
        let assign82140_e122971: f64 = (assign82140_e122969 * var_nqs_q);
        let assign82140_e122972: f64 = (assign82140_e122966 - assign82140_e122971);
        (assign82140_e122972, (((var_nqs_p_dn5 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn5)) - (((4.0 * var_nqs_xi_dn5) * var_nqs_q) + (assign82140_e122969 * var_nqs_q_dn5))), (((var_nqs_p_dn6 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn6)) - (((4.0 * var_nqs_xi_dn6) * var_nqs_q) + (assign82140_e122969 * var_nqs_q_dn6))), (((var_nqs_p_dn7 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn7)) - (((4.0 * var_nqs_xi_dn7) * var_nqs_q) + (assign82140_e122969 * var_nqs_q_dn7))), (((var_nqs_p_dn8 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn8)) - (((4.0 * var_nqs_xi_dn8) * var_nqs_q) + (assign82140_e122969 * var_nqs_q_dn8))), (((var_nqs_p_dn12 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn12)) - (((4.0 * var_nqs_xi_dn12) * var_nqs_q) + (assign82140_e122969 * var_nqs_q_dn12))), (((var_nqs_p_dn13 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn13)) - (((4.0 * var_nqs_xi_dn13) * var_nqs_q) + (assign82140_e122969 * var_nqs_q_dn13))), (((var_nqs_p_dn14 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn14)) - (((4.0 * var_nqs_xi_dn14) * var_nqs_q) + (assign82140_e122969 * var_nqs_q_dn14))), (((var_nqs_p_dn15 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn15)) - (((4.0 * var_nqs_xi_dn15) * var_nqs_q) + (assign82140_e122969 * var_nqs_q_dn15))), (((var_nqs_p_dn16 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn16)) - (((4.0 * var_nqs_xi_dn16) * var_nqs_q) + (assign82140_e122969 * var_nqs_q_dn16))), (((var_nqs_p_dn17 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn17)) - (((4.0 * var_nqs_xi_dn17) * var_nqs_q) + (assign82140_e122969 * var_nqs_q_dn17))), (((var_nqs_p_dn18 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn18)) - (((4.0 * var_nqs_xi_dn18) * var_nqs_q) + (assign82140_e122969 * var_nqs_q_dn18))), (((var_nqs_p_dn19 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn19)) - (((4.0 * var_nqs_xi_dn19) * var_nqs_q) + (assign82140_e122969 * var_nqs_q_dn19))), (((var_nqs_p_dn20 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn20)) - (((4.0 * var_nqs_xi_dn20) * var_nqs_q) + (assign82140_e122969 * var_nqs_q_dn20))),)
    } else {
        (var_nqs_temp, var_nqs_temp_dn5, var_nqs_temp_dn6, var_nqs_temp_dn7, var_nqs_temp_dn8, var_nqs_temp_dn12, var_nqs_temp_dn13, var_nqs_temp_dn14, var_nqs_temp_dn15, var_nqs_temp_dn16, var_nqs_temp_dn17, var_nqs_temp_dn18, var_nqs_temp_dn19, var_nqs_temp_dn20,)
    }
};
        var_nqs_temp = assign82140_e122974;
        var_nqs_temp_dn5 = assign82140_e122974_d_n5;
        var_nqs_temp_dn6 = assign82140_e122974_d_n6;
        var_nqs_temp_dn7 = assign82140_e122974_d_n7;
        var_nqs_temp_dn8 = assign82140_e122974_d_n8;
        var_nqs_temp_dn12 = assign82140_e122974_d_n12;
        var_nqs_temp_dn13 = assign82140_e122974_d_n13;
        var_nqs_temp_dn14 = assign82140_e122974_d_n14;
        var_nqs_temp_dn15 = assign82140_e122974_d_n15;
        var_nqs_temp_dn16 = assign82140_e122974_d_n16;
        var_nqs_temp_dn17 = assign82140_e122974_d_n17;
        var_nqs_temp_dn18 = assign82140_e122974_d_n18;
        var_nqs_temp_dn19 = assign82140_e122974_d_n19;
        var_nqs_temp_dn20 = assign82140_e122974_d_n20;

        let (assign82150_e123005, assign82150_e123005_d_n5, assign82150_e123005_d_n6, assign82150_e123005_d_n7, assign82150_e123005_d_n8, assign82150_e123005_d_n12, assign82150_e123005_d_n13, assign82150_e123005_d_n14, assign82150_e123005_d_n15, assign82150_e123005_d_n16, assign82150_e123005_d_n17, assign82150_e123005_d_n18, assign82150_e123005_d_n19, assign82150_e123005_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2228 == 0.0)) && (var_guard2229 == 0.0)) {
        let assign82150_e122998: f64 = (2.0 * var_nqs_q);
        let assign82150_e123001: f64 = (var_nqs_temp).sqrt();
        let assign82150_e123002: f64 = (var_nqs_p + assign82150_e123001);
        let assign82150_e123003: f64 = (assign82150_e122998 / assign82150_e123002);
        (assign82150_e123003, ((((2.0 * var_nqs_q_dn5) * assign82150_e123002) - (assign82150_e122998 * (var_nqs_p_dn5 + (var_nqs_temp_dn5 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * var_nqs_q_dn6) * assign82150_e123002) - (assign82150_e122998 * (var_nqs_p_dn6 + (var_nqs_temp_dn6 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * var_nqs_q_dn7) * assign82150_e123002) - (assign82150_e122998 * (var_nqs_p_dn7 + (var_nqs_temp_dn7 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * var_nqs_q_dn8) * assign82150_e123002) - (assign82150_e122998 * (var_nqs_p_dn8 + (var_nqs_temp_dn8 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * var_nqs_q_dn12) * assign82150_e123002) - (assign82150_e122998 * (var_nqs_p_dn12 + (var_nqs_temp_dn12 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * var_nqs_q_dn13) * assign82150_e123002) - (assign82150_e122998 * (var_nqs_p_dn13 + (var_nqs_temp_dn13 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * var_nqs_q_dn14) * assign82150_e123002) - (assign82150_e122998 * (var_nqs_p_dn14 + (var_nqs_temp_dn14 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * var_nqs_q_dn15) * assign82150_e123002) - (assign82150_e122998 * (var_nqs_p_dn15 + (var_nqs_temp_dn15 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * var_nqs_q_dn16) * assign82150_e123002) - (assign82150_e122998 * (var_nqs_p_dn16 + (var_nqs_temp_dn16 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * var_nqs_q_dn17) * assign82150_e123002) - (assign82150_e122998 * (var_nqs_p_dn17 + (var_nqs_temp_dn17 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * var_nqs_q_dn18) * assign82150_e123002) - (assign82150_e122998 * (var_nqs_p_dn18 + (var_nqs_temp_dn18 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * var_nqs_q_dn19) * assign82150_e123002) - (assign82150_e122998 * (var_nqs_p_dn19 + (var_nqs_temp_dn19 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)), ((((2.0 * var_nqs_q_dn20) * assign82150_e123002) - (assign82150_e122998 * (var_nqs_p_dn20 + (var_nqs_temp_dn20 / (2.0 * assign82150_e123001))))) / (assign82150_e123002 * assign82150_e123002)),)
    } else {
        (var_nqs_u, var_nqs_u_dn5, var_nqs_u_dn6, var_nqs_u_dn7, var_nqs_u_dn8, var_nqs_u_dn12, var_nqs_u_dn13, var_nqs_u_dn14, var_nqs_u_dn15, var_nqs_u_dn16, var_nqs_u_dn17, var_nqs_u_dn18, var_nqs_u_dn19, var_nqs_u_dn20,)
    }
};
        var_nqs_u = assign82150_e123005;
        var_nqs_u_dn5 = assign82150_e123005_d_n5;
        var_nqs_u_dn6 = assign82150_e123005_d_n6;
        var_nqs_u_dn7 = assign82150_e123005_d_n7;
        var_nqs_u_dn8 = assign82150_e123005_d_n8;
        var_nqs_u_dn12 = assign82150_e123005_d_n12;
        var_nqs_u_dn13 = assign82150_e123005_d_n13;
        var_nqs_u_dn14 = assign82150_e123005_d_n14;
        var_nqs_u_dn15 = assign82150_e123005_d_n15;
        var_nqs_u_dn16 = assign82150_e123005_d_n16;
        var_nqs_u_dn17 = assign82150_e123005_d_n17;
        var_nqs_u_dn18 = assign82150_e123005_d_n18;
        var_nqs_u_dn19 = assign82150_e123005_d_n19;
        var_nqs_u_dn20 = assign82150_e123005_d_n20;

        let (assign82160_e123031, assign82160_e123031_d_n5, assign82160_e123031_d_n6, assign82160_e123031_d_n7, assign82160_e123031_d_n8, assign82160_e123031_d_n12, assign82160_e123031_d_n13, assign82160_e123031_d_n14, assign82160_e123031_d_n15, assign82160_e123031_d_n16, assign82160_e123031_d_n17, assign82160_e123031_d_n18, assign82160_e123031_d_n19, assign82160_e123031_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2228 == 0.0)) && (var_guard2229 == 0.0)) {
        let assign82160_e123029: f64 = (var_nqs_x0 + var_nqs_u);
        (assign82160_e123029, (var_nqs_x0_dn5 + var_nqs_u_dn5), (var_nqs_x0_dn6 + var_nqs_u_dn6), (var_nqs_x0_dn7 + var_nqs_u_dn7), (var_nqs_x0_dn8 + var_nqs_u_dn8), (var_nqs_x0_dn12 + var_nqs_u_dn12), (var_nqs_x0_dn13 + var_nqs_u_dn13), (var_nqs_x0_dn14 + var_nqs_u_dn14), (var_nqs_x0_dn15 + var_nqs_u_dn15), (var_nqs_x0_dn16 + var_nqs_u_dn16), (var_nqs_x0_dn17 + var_nqs_u_dn17), (var_nqs_x0_dn18 + var_nqs_u_dn18), (var_nqs_x0_dn19 + var_nqs_u_dn19), (var_nqs_x0_dn20 + var_nqs_u_dn20),)
    } else {
        (var_temp8, var_temp8_dn5, var_temp8_dn6, var_temp8_dn7, var_temp8_dn8, var_temp8_dn12, var_temp8_dn13, var_temp8_dn14, var_temp8_dn15, var_temp8_dn16, var_temp8_dn17, var_temp8_dn18, var_temp8_dn19, var_temp8_dn20,)
    }
};
        var_temp8 = assign82160_e123031;
        var_temp8_dn5 = assign82160_e123031_d_n5;
        var_temp8_dn6 = assign82160_e123031_d_n6;
        var_temp8_dn7 = assign82160_e123031_d_n7;
        var_temp8_dn8 = assign82160_e123031_d_n8;
        var_temp8_dn12 = assign82160_e123031_d_n12;
        var_temp8_dn13 = assign82160_e123031_d_n13;
        var_temp8_dn14 = assign82160_e123031_d_n14;
        var_temp8_dn15 = assign82160_e123031_d_n15;
        var_temp8_dn16 = assign82160_e123031_d_n16;
        var_temp8_dn17 = assign82160_e123031_d_n17;
        var_temp8_dn18 = assign82160_e123031_d_n18;
        var_temp8_dn19 = assign82160_e123031_d_n19;
        var_temp8_dn20 = assign82160_e123031_d_n20;

        let (assign82170_e123053, assign82170_e123053_d_n5, assign82170_e123053_d_n6, assign82170_e123053_d_n7, assign82170_e123053_d_n8, assign82170_e123053_d_n12, assign82170_e123053_d_n13, assign82170_e123053_d_n14, assign82170_e123053_d_n15, assign82170_e123053_d_n16, assign82170_e123053_d_n17, assign82170_e123053_d_n18, assign82170_e123053_d_n19, assign82170_e123053_d_n20,) = {
    if ((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) {
        let assign82170_e123049: f64 = (var_qp9 / var_pd);
        let assign82170_e123051: f64 = (assign82170_e123049 + var_xg_ac);
        (assign82170_e123051, ((-((var_qp9 * var_pd_dn5) / (var_pd * var_pd))) + var_xg_ac_dn5), ((-((var_qp9 * var_pd_dn6) / (var_pd * var_pd))) + var_xg_ac_dn6), ((-((var_qp9 * var_pd_dn7) / (var_pd * var_pd))) + var_xg_ac_dn7), ((-((var_qp9 * var_pd_dn8) / (var_pd * var_pd))) + var_xg_ac_dn8), ((-((var_qp9 * var_pd_dn12) / (var_pd * var_pd))) + var_xg_ac_dn12), ((-((var_qp9 * var_pd_dn13) / (var_pd * var_pd))) + var_xg_ac_dn13), ((-((var_qp9 * var_pd_dn14) / (var_pd * var_pd))) + var_xg_ac_dn14), ((-((var_qp9 * var_pd_dn15) / (var_pd * var_pd))) + var_xg_ac_dn15), ((-((var_qp9 * var_pd_dn16) / (var_pd * var_pd))) + var_xg_ac_dn16), ((-((var_qp9 * var_pd_dn17) / (var_pd * var_pd))) + var_xg_ac_dn17), ((-((var_qp9 * var_pd_dn18) / (var_pd * var_pd))) + var_xg_ac_dn18), ((-((var_qp9 * var_pd_dn19) / (var_pd * var_pd))) + var_xg_ac_dn19), ((((var_qp9_dn20 * var_pd) - (var_qp9 * var_pd_dn20)) / (var_pd * var_pd)) + var_xg_ac_dn20),)
    } else {
        (var_temp__blk1038, var_temp__blk1038_dn5, var_temp__blk1038_dn6, var_temp__blk1038_dn7, var_temp__blk1038_dn8, var_temp__blk1038_dn12, var_temp__blk1038_dn13, var_temp__blk1038_dn14, var_temp__blk1038_dn15, var_temp__blk1038_dn16, var_temp__blk1038_dn17, var_temp__blk1038_dn18, var_temp__blk1038_dn19, var_temp__blk1038_dn20,)
    }
};
        var_temp__blk1038 = assign82170_e123053;
        var_temp__blk1038_dn5 = assign82170_e123053_d_n5;
        var_temp__blk1038_dn6 = assign82170_e123053_d_n6;
        var_temp__blk1038_dn7 = assign82170_e123053_d_n7;
        var_temp__blk1038_dn8 = assign82170_e123053_d_n8;
        var_temp__blk1038_dn12 = assign82170_e123053_d_n12;
        var_temp__blk1038_dn13 = assign82170_e123053_d_n13;
        var_temp__blk1038_dn14 = assign82170_e123053_d_n14;
        var_temp__blk1038_dn15 = assign82170_e123053_d_n15;
        var_temp__blk1038_dn16 = assign82170_e123053_d_n16;
        var_temp__blk1038_dn17 = assign82170_e123053_d_n17;
        var_temp__blk1038_dn18 = assign82170_e123053_d_n18;
        var_temp__blk1038_dn19 = assign82170_e123053_d_n19;
        var_temp__blk1038_dn20 = assign82170_e123053_d_n20;

        let assign82180_e123055: f64 = (var_temp__blk1038).abs();
        let assign82180_e123057: f64 = if assign82180_e123055 <= var_marginp { 1.0 } else { 0.0 };
        var_guard2236 = assign82180_e123057;

        let (assign82190_e123079, assign82190_e123079_d_n5, assign82190_e123079_d_n6, assign82190_e123079_d_n7, assign82190_e123079_d_n8, assign82190_e123079_d_n12, assign82190_e123079_d_n13, assign82190_e123079_d_n14, assign82190_e123079_d_n15, assign82190_e123079_d_n16, assign82190_e123079_d_n17, assign82190_e123079_d_n18, assign82190_e123079_d_n19, assign82190_e123079_d_n20,) = {
    if (((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 != 0.0)) {
        let assign82190_e123077: f64 = (var_temp__blk1038 / var_a_factrp);
        (assign82190_e123077, (((var_temp__blk1038_dn5 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn5)) / (var_a_factrp * var_a_factrp)), (((var_temp__blk1038_dn6 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn6)) / (var_a_factrp * var_a_factrp)), (((var_temp__blk1038_dn7 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn7)) / (var_a_factrp * var_a_factrp)), (((var_temp__blk1038_dn8 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn8)) / (var_a_factrp * var_a_factrp)), (((var_temp__blk1038_dn12 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn12)) / (var_a_factrp * var_a_factrp)), (((var_temp__blk1038_dn13 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn13)) / (var_a_factrp * var_a_factrp)), (((var_temp__blk1038_dn14 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn14)) / (var_a_factrp * var_a_factrp)), (((var_temp__blk1038_dn15 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn15)) / (var_a_factrp * var_a_factrp)), (((var_temp__blk1038_dn16 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn16)) / (var_a_factrp * var_a_factrp)), (((var_temp__blk1038_dn17 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn17)) / (var_a_factrp * var_a_factrp)), (((var_temp__blk1038_dn18 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn18)) / (var_a_factrp * var_a_factrp)), (((var_temp__blk1038_dn19 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn19)) / (var_a_factrp * var_a_factrp)), (((var_temp__blk1038_dn20 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn20)) / (var_a_factrp * var_a_factrp)),)
    } else {
        (var_temp9, var_temp9_dn5, var_temp9_dn6, var_temp9_dn7, var_temp9_dn8, var_temp9_dn12, var_temp9_dn13, var_temp9_dn14, var_temp9_dn15, var_temp9_dn16, var_temp9_dn17, var_temp9_dn18, var_temp9_dn19, var_temp9_dn20,)
    }
};
        var_temp9 = assign82190_e123079;
        var_temp9_dn5 = assign82190_e123079_d_n5;
        var_temp9_dn6 = assign82190_e123079_d_n6;
        var_temp9_dn7 = assign82190_e123079_d_n7;
        var_temp9_dn8 = assign82190_e123079_d_n8;
        var_temp9_dn12 = assign82190_e123079_d_n12;
        var_temp9_dn13 = assign82190_e123079_d_n13;
        var_temp9_dn14 = assign82190_e123079_d_n14;
        var_temp9_dn15 = assign82190_e123079_d_n15;
        var_temp9_dn16 = assign82190_e123079_d_n16;
        var_temp9_dn17 = assign82190_e123079_d_n17;
        var_temp9_dn18 = assign82190_e123079_d_n18;
        var_temp9_dn19 = assign82190_e123079_d_n19;
        var_temp9_dn20 = assign82190_e123079_d_n20;

        let assign82200_e123082: f64 = (-var_marginp);
        let assign82200_e123083: f64 = if var_temp__blk1038 < assign82200_e123082 { 1.0 } else { 0.0 };
        var_guard2237 = assign82200_e123083;

        let (assign82210_e123107, assign82210_e123107_d_n5, assign82210_e123107_d_n6, assign82210_e123107_d_n7, assign82210_e123107_d_n8, assign82210_e123107_d_n12, assign82210_e123107_d_n13, assign82210_e123107_d_n14, assign82210_e123107_d_n15, assign82210_e123107_d_n16, assign82210_e123107_d_n17, assign82210_e123107_d_n18, assign82210_e123107_d_n19, assign82210_e123107_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) {
        let assign82210_e123105: f64 = (-var_temp__blk1038);
        (assign82210_e123105, (-var_temp__blk1038_dn5), (-var_temp__blk1038_dn6), (-var_temp__blk1038_dn7), (-var_temp__blk1038_dn8), (-var_temp__blk1038_dn12), (-var_temp__blk1038_dn13), (-var_temp__blk1038_dn14), (-var_temp__blk1038_dn15), (-var_temp__blk1038_dn16), (-var_temp__blk1038_dn17), (-var_temp__blk1038_dn18), (-var_temp__blk1038_dn19), (-var_temp__blk1038_dn20),)
    } else {
        (var_nqs_yg, var_nqs_yg_dn5, var_nqs_yg_dn6, var_nqs_yg_dn7, var_nqs_yg_dn8, var_nqs_yg_dn12, var_nqs_yg_dn13, var_nqs_yg_dn14, var_nqs_yg_dn15, var_nqs_yg_dn16, var_nqs_yg_dn17, var_nqs_yg_dn18, var_nqs_yg_dn19, var_nqs_yg_dn20,)
    }
};
        var_nqs_yg = assign82210_e123107;
        var_nqs_yg_dn5 = assign82210_e123107_d_n5;
        var_nqs_yg_dn6 = assign82210_e123107_d_n6;
        var_nqs_yg_dn7 = assign82210_e123107_d_n7;
        var_nqs_yg_dn8 = assign82210_e123107_d_n8;
        var_nqs_yg_dn12 = assign82210_e123107_d_n12;
        var_nqs_yg_dn13 = assign82210_e123107_d_n13;
        var_nqs_yg_dn14 = assign82210_e123107_d_n14;
        var_nqs_yg_dn15 = assign82210_e123107_d_n15;
        var_nqs_yg_dn16 = assign82210_e123107_d_n16;
        var_nqs_yg_dn17 = assign82210_e123107_d_n17;
        var_nqs_yg_dn18 = assign82210_e123107_d_n18;
        var_nqs_yg_dn19 = assign82210_e123107_d_n19;
        var_nqs_yg_dn20 = assign82210_e123107_d_n20;

        let (assign82220_e123134, assign82220_e123134_d_n5, assign82220_e123134_d_n6, assign82220_e123134_d_n7, assign82220_e123134_d_n8, assign82220_e123134_d_n12, assign82220_e123134_d_n13, assign82220_e123134_d_n14, assign82220_e123134_d_n15, assign82220_e123134_d_n16, assign82220_e123134_d_n17, assign82220_e123134_d_n18, assign82220_e123134_d_n19, assign82220_e123134_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) {
        let assign82220_e123130: f64 = (1.25 * var_nqs_yg);
        let assign82220_e123132: f64 = (assign82220_e123130 / var_a_factrp);
        (assign82220_e123132, ((((1.25 * var_nqs_yg_dn5) * var_a_factrp) - (assign82220_e123130 * var_a_factrp_dn5)) / (var_a_factrp * var_a_factrp)), ((((1.25 * var_nqs_yg_dn6) * var_a_factrp) - (assign82220_e123130 * var_a_factrp_dn6)) / (var_a_factrp * var_a_factrp)), ((((1.25 * var_nqs_yg_dn7) * var_a_factrp) - (assign82220_e123130 * var_a_factrp_dn7)) / (var_a_factrp * var_a_factrp)), ((((1.25 * var_nqs_yg_dn8) * var_a_factrp) - (assign82220_e123130 * var_a_factrp_dn8)) / (var_a_factrp * var_a_factrp)), ((((1.25 * var_nqs_yg_dn12) * var_a_factrp) - (assign82220_e123130 * var_a_factrp_dn12)) / (var_a_factrp * var_a_factrp)), ((((1.25 * var_nqs_yg_dn13) * var_a_factrp) - (assign82220_e123130 * var_a_factrp_dn13)) / (var_a_factrp * var_a_factrp)), ((((1.25 * var_nqs_yg_dn14) * var_a_factrp) - (assign82220_e123130 * var_a_factrp_dn14)) / (var_a_factrp * var_a_factrp)), ((((1.25 * var_nqs_yg_dn15) * var_a_factrp) - (assign82220_e123130 * var_a_factrp_dn15)) / (var_a_factrp * var_a_factrp)), ((((1.25 * var_nqs_yg_dn16) * var_a_factrp) - (assign82220_e123130 * var_a_factrp_dn16)) / (var_a_factrp * var_a_factrp)), ((((1.25 * var_nqs_yg_dn17) * var_a_factrp) - (assign82220_e123130 * var_a_factrp_dn17)) / (var_a_factrp * var_a_factrp)), ((((1.25 * var_nqs_yg_dn18) * var_a_factrp) - (assign82220_e123130 * var_a_factrp_dn18)) / (var_a_factrp * var_a_factrp)), ((((1.25 * var_nqs_yg_dn19) * var_a_factrp) - (assign82220_e123130 * var_a_factrp_dn19)) / (var_a_factrp * var_a_factrp)), ((((1.25 * var_nqs_yg_dn20) * var_a_factrp) - (assign82220_e123130 * var_a_factrp_dn20)) / (var_a_factrp * var_a_factrp)),)
    } else {
        (var_nqs_z, var_nqs_z_dn5, var_nqs_z_dn6, var_nqs_z_dn7, var_nqs_z_dn8, var_nqs_z_dn12, var_nqs_z_dn13, var_nqs_z_dn14, var_nqs_z_dn15, var_nqs_z_dn16, var_nqs_z_dn17, var_nqs_z_dn18, var_nqs_z_dn19, var_nqs_z_dn20,)
    }
};
        var_nqs_z = assign82220_e123134;
        var_nqs_z_dn5 = assign82220_e123134_d_n5;
        var_nqs_z_dn6 = assign82220_e123134_d_n6;
        var_nqs_z_dn7 = assign82220_e123134_d_n7;
        var_nqs_z_dn8 = assign82220_e123134_d_n8;
        var_nqs_z_dn12 = assign82220_e123134_d_n12;
        var_nqs_z_dn13 = assign82220_e123134_d_n13;
        var_nqs_z_dn14 = assign82220_e123134_d_n14;
        var_nqs_z_dn15 = assign82220_e123134_d_n15;
        var_nqs_z_dn16 = assign82220_e123134_d_n16;
        var_nqs_z_dn17 = assign82220_e123134_d_n17;
        var_nqs_z_dn18 = assign82220_e123134_d_n18;
        var_nqs_z_dn19 = assign82220_e123134_d_n19;
        var_nqs_z_dn20 = assign82220_e123134_d_n20;

        let (assign82230_e123172, assign82230_e123172_d_n5, assign82230_e123172_d_n6, assign82230_e123172_d_n7, assign82230_e123172_d_n8, assign82230_e123172_d_n12, assign82230_e123172_d_n13, assign82230_e123172_d_n14, assign82230_e123172_d_n15, assign82230_e123172_d_n16, assign82230_e123172_d_n17, assign82230_e123172_d_n18, assign82230_e123172_d_n19, assign82230_e123172_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) {
        let assign82230_e123157: f64 = (var_nqs_z + 10.0);
        let assign82230_e123160: f64 = (var_nqs_z - 6.0);
        let assign82230_e123163: f64 = (var_nqs_z - 6.0);
        let assign82230_e123164: f64 = (assign82230_e123160 * assign82230_e123163);
        let assign82230_e123166: f64 = (assign82230_e123164 + 64.0);
        let assign82230_e123167: f64 = (assign82230_e123166).sqrt();
        let assign82230_e123168: f64 = (assign82230_e123157 - assign82230_e123167);
        let assign82230_e123170: f64 = (assign82230_e123168 * 0.5);
        (assign82230_e123170, ((var_nqs_z_dn5 - (((var_nqs_z_dn5 * assign82230_e123163) + (assign82230_e123160 * var_nqs_z_dn5)) / (2.0 * assign82230_e123167))) * 0.5), ((var_nqs_z_dn6 - (((var_nqs_z_dn6 * assign82230_e123163) + (assign82230_e123160 * var_nqs_z_dn6)) / (2.0 * assign82230_e123167))) * 0.5), ((var_nqs_z_dn7 - (((var_nqs_z_dn7 * assign82230_e123163) + (assign82230_e123160 * var_nqs_z_dn7)) / (2.0 * assign82230_e123167))) * 0.5), ((var_nqs_z_dn8 - (((var_nqs_z_dn8 * assign82230_e123163) + (assign82230_e123160 * var_nqs_z_dn8)) / (2.0 * assign82230_e123167))) * 0.5), ((var_nqs_z_dn12 - (((var_nqs_z_dn12 * assign82230_e123163) + (assign82230_e123160 * var_nqs_z_dn12)) / (2.0 * assign82230_e123167))) * 0.5), ((var_nqs_z_dn13 - (((var_nqs_z_dn13 * assign82230_e123163) + (assign82230_e123160 * var_nqs_z_dn13)) / (2.0 * assign82230_e123167))) * 0.5), ((var_nqs_z_dn14 - (((var_nqs_z_dn14 * assign82230_e123163) + (assign82230_e123160 * var_nqs_z_dn14)) / (2.0 * assign82230_e123167))) * 0.5), ((var_nqs_z_dn15 - (((var_nqs_z_dn15 * assign82230_e123163) + (assign82230_e123160 * var_nqs_z_dn15)) / (2.0 * assign82230_e123167))) * 0.5), ((var_nqs_z_dn16 - (((var_nqs_z_dn16 * assign82230_e123163) + (assign82230_e123160 * var_nqs_z_dn16)) / (2.0 * assign82230_e123167))) * 0.5), ((var_nqs_z_dn17 - (((var_nqs_z_dn17 * assign82230_e123163) + (assign82230_e123160 * var_nqs_z_dn17)) / (2.0 * assign82230_e123167))) * 0.5), ((var_nqs_z_dn18 - (((var_nqs_z_dn18 * assign82230_e123163) + (assign82230_e123160 * var_nqs_z_dn18)) / (2.0 * assign82230_e123167))) * 0.5), ((var_nqs_z_dn19 - (((var_nqs_z_dn19 * assign82230_e123163) + (assign82230_e123160 * var_nqs_z_dn19)) / (2.0 * assign82230_e123167))) * 0.5), ((var_nqs_z_dn20 - (((var_nqs_z_dn20 * assign82230_e123163) + (assign82230_e123160 * var_nqs_z_dn20)) / (2.0 * assign82230_e123167))) * 0.5),)
    } else {
        (var_nqs_eta, var_nqs_eta_dn5, var_nqs_eta_dn6, var_nqs_eta_dn7, var_nqs_eta_dn8, var_nqs_eta_dn12, var_nqs_eta_dn13, var_nqs_eta_dn14, var_nqs_eta_dn15, var_nqs_eta_dn16, var_nqs_eta_dn17, var_nqs_eta_dn18, var_nqs_eta_dn19, var_nqs_eta_dn20,)
    }
};
        var_nqs_eta = assign82230_e123172;
        var_nqs_eta_dn5 = assign82230_e123172_d_n5;
        var_nqs_eta_dn6 = assign82230_e123172_d_n6;
        var_nqs_eta_dn7 = assign82230_e123172_d_n7;
        var_nqs_eta_dn8 = assign82230_e123172_d_n8;
        var_nqs_eta_dn12 = assign82230_e123172_d_n12;
        var_nqs_eta_dn13 = assign82230_e123172_d_n13;
        var_nqs_eta_dn14 = assign82230_e123172_d_n14;
        var_nqs_eta_dn15 = assign82230_e123172_d_n15;
        var_nqs_eta_dn16 = assign82230_e123172_d_n16;
        var_nqs_eta_dn17 = assign82230_e123172_d_n17;
        var_nqs_eta_dn18 = assign82230_e123172_d_n18;
        var_nqs_eta_dn19 = assign82230_e123172_d_n19;
        var_nqs_eta_dn20 = assign82230_e123172_d_n20;

        let (assign82240_e123207, assign82240_e123207_d_n5, assign82240_e123207_d_n6, assign82240_e123207_d_n7, assign82240_e123207_d_n8, assign82240_e123207_d_n12, assign82240_e123207_d_n13, assign82240_e123207_d_n14, assign82240_e123207_d_n15, assign82240_e123207_d_n16, assign82240_e123207_d_n17, assign82240_e123207_d_n18, assign82240_e123207_d_n19, assign82240_e123207_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) {
        let assign82240_e123195: f64 = (var_nqs_yg - var_nqs_eta);
        let assign82240_e123198: f64 = (var_nqs_yg - var_nqs_eta);
        let assign82240_e123199: f64 = (assign82240_e123195 * assign82240_e123198);
        let assign82240_e123203: f64 = (var_nqs_eta + 1.0);
        let assign82240_e123204: f64 = (var_gp2 * assign82240_e123203);
        let assign82240_e123205: f64 = (assign82240_e123199 + assign82240_e123204);
        (assign82240_e123205, ((((var_nqs_yg_dn5 - var_nqs_eta_dn5) * assign82240_e123198) + (assign82240_e123195 * (var_nqs_yg_dn5 - var_nqs_eta_dn5))) + ((var_gp2_dn5 * assign82240_e123203) + (var_gp2 * var_nqs_eta_dn5))), ((((var_nqs_yg_dn6 - var_nqs_eta_dn6) * assign82240_e123198) + (assign82240_e123195 * (var_nqs_yg_dn6 - var_nqs_eta_dn6))) + ((var_gp2_dn6 * assign82240_e123203) + (var_gp2 * var_nqs_eta_dn6))), ((((var_nqs_yg_dn7 - var_nqs_eta_dn7) * assign82240_e123198) + (assign82240_e123195 * (var_nqs_yg_dn7 - var_nqs_eta_dn7))) + ((var_gp2_dn7 * assign82240_e123203) + (var_gp2 * var_nqs_eta_dn7))), ((((var_nqs_yg_dn8 - var_nqs_eta_dn8) * assign82240_e123198) + (assign82240_e123195 * (var_nqs_yg_dn8 - var_nqs_eta_dn8))) + ((var_gp2_dn8 * assign82240_e123203) + (var_gp2 * var_nqs_eta_dn8))), ((((var_nqs_yg_dn12 - var_nqs_eta_dn12) * assign82240_e123198) + (assign82240_e123195 * (var_nqs_yg_dn12 - var_nqs_eta_dn12))) + ((var_gp2_dn12 * assign82240_e123203) + (var_gp2 * var_nqs_eta_dn12))), ((((var_nqs_yg_dn13 - var_nqs_eta_dn13) * assign82240_e123198) + (assign82240_e123195 * (var_nqs_yg_dn13 - var_nqs_eta_dn13))) + ((var_gp2_dn13 * assign82240_e123203) + (var_gp2 * var_nqs_eta_dn13))), ((((var_nqs_yg_dn14 - var_nqs_eta_dn14) * assign82240_e123198) + (assign82240_e123195 * (var_nqs_yg_dn14 - var_nqs_eta_dn14))) + ((var_gp2_dn14 * assign82240_e123203) + (var_gp2 * var_nqs_eta_dn14))), ((((var_nqs_yg_dn15 - var_nqs_eta_dn15) * assign82240_e123198) + (assign82240_e123195 * (var_nqs_yg_dn15 - var_nqs_eta_dn15))) + ((var_gp2_dn15 * assign82240_e123203) + (var_gp2 * var_nqs_eta_dn15))), ((((var_nqs_yg_dn16 - var_nqs_eta_dn16) * assign82240_e123198) + (assign82240_e123195 * (var_nqs_yg_dn16 - var_nqs_eta_dn16))) + ((var_gp2_dn16 * assign82240_e123203) + (var_gp2 * var_nqs_eta_dn16))), ((((var_nqs_yg_dn17 - var_nqs_eta_dn17) * assign82240_e123198) + (assign82240_e123195 * (var_nqs_yg_dn17 - var_nqs_eta_dn17))) + ((var_gp2_dn17 * assign82240_e123203) + (var_gp2 * var_nqs_eta_dn17))), ((((var_nqs_yg_dn18 - var_nqs_eta_dn18) * assign82240_e123198) + (assign82240_e123195 * (var_nqs_yg_dn18 - var_nqs_eta_dn18))) + ((var_gp2_dn18 * assign82240_e123203) + (var_gp2 * var_nqs_eta_dn18))), ((((var_nqs_yg_dn19 - var_nqs_eta_dn19) * assign82240_e123198) + (assign82240_e123195 * (var_nqs_yg_dn19 - var_nqs_eta_dn19))) + ((var_gp2_dn19 * assign82240_e123203) + (var_gp2 * var_nqs_eta_dn19))), ((((var_nqs_yg_dn20 - var_nqs_eta_dn20) * assign82240_e123198) + (assign82240_e123195 * (var_nqs_yg_dn20 - var_nqs_eta_dn20))) + ((var_gp2_dn20 * assign82240_e123203) + (var_gp2 * var_nqs_eta_dn20))),)
    } else {
        (var_nqs_a, var_nqs_a_dn5, var_nqs_a_dn6, var_nqs_a_dn7, var_nqs_a_dn8, var_nqs_a_dn12, var_nqs_a_dn13, var_nqs_a_dn14, var_nqs_a_dn15, var_nqs_a_dn16, var_nqs_a_dn17, var_nqs_a_dn18, var_nqs_a_dn19, var_nqs_a_dn20,)
    }
};
        var_nqs_a = assign82240_e123207;
        var_nqs_a_dn5 = assign82240_e123207_d_n5;
        var_nqs_a_dn6 = assign82240_e123207_d_n6;
        var_nqs_a_dn7 = assign82240_e123207_d_n7;
        var_nqs_a_dn8 = assign82240_e123207_d_n8;
        var_nqs_a_dn12 = assign82240_e123207_d_n12;
        var_nqs_a_dn13 = assign82240_e123207_d_n13;
        var_nqs_a_dn14 = assign82240_e123207_d_n14;
        var_nqs_a_dn15 = assign82240_e123207_d_n15;
        var_nqs_a_dn16 = assign82240_e123207_d_n16;
        var_nqs_a_dn17 = assign82240_e123207_d_n17;
        var_nqs_a_dn18 = assign82240_e123207_d_n18;
        var_nqs_a_dn19 = assign82240_e123207_d_n19;
        var_nqs_a_dn20 = assign82240_e123207_d_n20;

        let (assign82250_e123236, assign82250_e123236_d_n5, assign82250_e123236_d_n6, assign82250_e123236_d_n7, assign82250_e123236_d_n8, assign82250_e123236_d_n12, assign82250_e123236_d_n13, assign82250_e123236_d_n14, assign82250_e123236_d_n15, assign82250_e123236_d_n16, assign82250_e123236_d_n17, assign82250_e123236_d_n18, assign82250_e123236_d_n19, assign82250_e123236_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) {
        let assign82250_e123231: f64 = (var_nqs_yg - var_nqs_eta);
        let assign82250_e123232: f64 = (2.0 * assign82250_e123231);
        let assign82250_e123234: f64 = (assign82250_e123232 - var_gp2);
        (assign82250_e123234, ((2.0 * (var_nqs_yg_dn5 - var_nqs_eta_dn5)) - var_gp2_dn5), ((2.0 * (var_nqs_yg_dn6 - var_nqs_eta_dn6)) - var_gp2_dn6), ((2.0 * (var_nqs_yg_dn7 - var_nqs_eta_dn7)) - var_gp2_dn7), ((2.0 * (var_nqs_yg_dn8 - var_nqs_eta_dn8)) - var_gp2_dn8), ((2.0 * (var_nqs_yg_dn12 - var_nqs_eta_dn12)) - var_gp2_dn12), ((2.0 * (var_nqs_yg_dn13 - var_nqs_eta_dn13)) - var_gp2_dn13), ((2.0 * (var_nqs_yg_dn14 - var_nqs_eta_dn14)) - var_gp2_dn14), ((2.0 * (var_nqs_yg_dn15 - var_nqs_eta_dn15)) - var_gp2_dn15), ((2.0 * (var_nqs_yg_dn16 - var_nqs_eta_dn16)) - var_gp2_dn16), ((2.0 * (var_nqs_yg_dn17 - var_nqs_eta_dn17)) - var_gp2_dn17), ((2.0 * (var_nqs_yg_dn18 - var_nqs_eta_dn18)) - var_gp2_dn18), ((2.0 * (var_nqs_yg_dn19 - var_nqs_eta_dn19)) - var_gp2_dn19), ((2.0 * (var_nqs_yg_dn20 - var_nqs_eta_dn20)) - var_gp2_dn20),)
    } else {
        (var_nqs_c, var_nqs_c_dn5, var_nqs_c_dn6, var_nqs_c_dn7, var_nqs_c_dn8, var_nqs_c_dn12, var_nqs_c_dn13, var_nqs_c_dn14, var_nqs_c_dn15, var_nqs_c_dn16, var_nqs_c_dn17, var_nqs_c_dn18, var_nqs_c_dn19, var_nqs_c_dn20,)
    }
};
        var_nqs_c = assign82250_e123236;
        var_nqs_c_dn5 = assign82250_e123236_d_n5;
        var_nqs_c_dn6 = assign82250_e123236_d_n6;
        var_nqs_c_dn7 = assign82250_e123236_d_n7;
        var_nqs_c_dn8 = assign82250_e123236_d_n8;
        var_nqs_c_dn12 = assign82250_e123236_d_n12;
        var_nqs_c_dn13 = assign82250_e123236_d_n13;
        var_nqs_c_dn14 = assign82250_e123236_d_n14;
        var_nqs_c_dn15 = assign82250_e123236_d_n15;
        var_nqs_c_dn16 = assign82250_e123236_d_n16;
        var_nqs_c_dn17 = assign82250_e123236_d_n17;
        var_nqs_c_dn18 = assign82250_e123236_d_n18;
        var_nqs_c_dn19 = assign82250_e123236_d_n19;
        var_nqs_c_dn20 = assign82250_e123236_d_n20;

        let (assign82260_e123264, assign82260_e123264_d_n5, assign82260_e123264_d_n6, assign82260_e123264_d_n7, assign82260_e123264_d_n8, assign82260_e123264_d_n12, assign82260_e123264_d_n13, assign82260_e123264_d_n14, assign82260_e123264_d_n15, assign82260_e123264_d_n16, assign82260_e123264_d_n17, assign82260_e123264_d_n18, assign82260_e123264_d_n19, assign82260_e123264_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) {
        let assign82260_e123259: f64 = (var_nqs_a / var_gp2);
        let assign82260_e123260: f64 = (assign82260_e123259).ln();
        let assign82260_e123262: f64 = (assign82260_e123260 - var_nqs_eta);
        (assign82260_e123262, (((((var_nqs_a_dn5 * var_gp2) - (var_nqs_a * var_gp2_dn5)) / (var_gp2 * var_gp2)) / assign82260_e123259) - var_nqs_eta_dn5), (((((var_nqs_a_dn6 * var_gp2) - (var_nqs_a * var_gp2_dn6)) / (var_gp2 * var_gp2)) / assign82260_e123259) - var_nqs_eta_dn6), (((((var_nqs_a_dn7 * var_gp2) - (var_nqs_a * var_gp2_dn7)) / (var_gp2 * var_gp2)) / assign82260_e123259) - var_nqs_eta_dn7), (((((var_nqs_a_dn8 * var_gp2) - (var_nqs_a * var_gp2_dn8)) / (var_gp2 * var_gp2)) / assign82260_e123259) - var_nqs_eta_dn8), (((((var_nqs_a_dn12 * var_gp2) - (var_nqs_a * var_gp2_dn12)) / (var_gp2 * var_gp2)) / assign82260_e123259) - var_nqs_eta_dn12), (((((var_nqs_a_dn13 * var_gp2) - (var_nqs_a * var_gp2_dn13)) / (var_gp2 * var_gp2)) / assign82260_e123259) - var_nqs_eta_dn13), (((((var_nqs_a_dn14 * var_gp2) - (var_nqs_a * var_gp2_dn14)) / (var_gp2 * var_gp2)) / assign82260_e123259) - var_nqs_eta_dn14), (((((var_nqs_a_dn15 * var_gp2) - (var_nqs_a * var_gp2_dn15)) / (var_gp2 * var_gp2)) / assign82260_e123259) - var_nqs_eta_dn15), (((((var_nqs_a_dn16 * var_gp2) - (var_nqs_a * var_gp2_dn16)) / (var_gp2 * var_gp2)) / assign82260_e123259) - var_nqs_eta_dn16), (((((var_nqs_a_dn17 * var_gp2) - (var_nqs_a * var_gp2_dn17)) / (var_gp2 * var_gp2)) / assign82260_e123259) - var_nqs_eta_dn17), (((((var_nqs_a_dn18 * var_gp2) - (var_nqs_a * var_gp2_dn18)) / (var_gp2 * var_gp2)) / assign82260_e123259) - var_nqs_eta_dn18), (((((var_nqs_a_dn19 * var_gp2) - (var_nqs_a * var_gp2_dn19)) / (var_gp2 * var_gp2)) / assign82260_e123259) - var_nqs_eta_dn19), (((((var_nqs_a_dn20 * var_gp2) - (var_nqs_a * var_gp2_dn20)) / (var_gp2 * var_gp2)) / assign82260_e123259) - var_nqs_eta_dn20),)
    } else {
        (var_nqs_tau, var_nqs_tau_dn5, var_nqs_tau_dn6, var_nqs_tau_dn7, var_nqs_tau_dn8, var_nqs_tau_dn12, var_nqs_tau_dn13, var_nqs_tau_dn14, var_nqs_tau_dn15, var_nqs_tau_dn16, var_nqs_tau_dn17, var_nqs_tau_dn18, var_nqs_tau_dn19, var_nqs_tau_dn20,)
    }
};
        var_nqs_tau = assign82260_e123264;
        var_nqs_tau_dn5 = assign82260_e123264_d_n5;
        var_nqs_tau_dn6 = assign82260_e123264_d_n6;
        var_nqs_tau_dn7 = assign82260_e123264_d_n7;
        var_nqs_tau_dn8 = assign82260_e123264_d_n8;
        var_nqs_tau_dn12 = assign82260_e123264_d_n12;
        var_nqs_tau_dn13 = assign82260_e123264_d_n13;
        var_nqs_tau_dn14 = assign82260_e123264_d_n14;
        var_nqs_tau_dn15 = assign82260_e123264_d_n15;
        var_nqs_tau_dn16 = assign82260_e123264_d_n16;
        var_nqs_tau_dn17 = assign82260_e123264_d_n17;
        var_nqs_tau_dn18 = assign82260_e123264_d_n18;
        var_nqs_tau_dn19 = assign82260_e123264_d_n19;
        var_nqs_tau_dn20 = assign82260_e123264_d_n20;

        let (assign82270_e123289, assign82270_e123289_d_n5, assign82270_e123289_d_n6, assign82270_e123289_d_n7, assign82270_e123289_d_n8, assign82270_e123289_d_n12, assign82270_e123289_d_n13, assign82270_e123289_d_n14, assign82270_e123289_d_n15, assign82270_e123289_d_n16, assign82270_e123289_d_n17, assign82270_e123289_d_n18, assign82270_e123289_d_n19, assign82270_e123289_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) {
        let assign82270_e123287: f64 = (var_nqs_a + var_nqs_c);
        (assign82270_e123287, (var_nqs_a_dn5 + var_nqs_c_dn5), (var_nqs_a_dn6 + var_nqs_c_dn6), (var_nqs_a_dn7 + var_nqs_c_dn7), (var_nqs_a_dn8 + var_nqs_c_dn8), (var_nqs_a_dn12 + var_nqs_c_dn12), (var_nqs_a_dn13 + var_nqs_c_dn13), (var_nqs_a_dn14 + var_nqs_c_dn14), (var_nqs_a_dn15 + var_nqs_c_dn15), (var_nqs_a_dn16 + var_nqs_c_dn16), (var_nqs_a_dn17 + var_nqs_c_dn17), (var_nqs_a_dn18 + var_nqs_c_dn18), (var_nqs_a_dn19 + var_nqs_c_dn19), (var_nqs_a_dn20 + var_nqs_c_dn20),)
    } else {
        (var_nu, var_nu_dn5, var_nu_dn6, var_nu_dn7, var_nu_dn8, var_nu_dn12, var_nu_dn13, var_nu_dn14, var_nu_dn15, var_nu_dn16, var_nu_dn17, var_nu_dn18, var_nu_dn19, var_nu_dn20,)
    }
};
        var_nu = assign82270_e123289;
        var_nu_dn5 = assign82270_e123289_d_n5;
        var_nu_dn6 = assign82270_e123289_d_n6;
        var_nu_dn7 = assign82270_e123289_d_n7;
        var_nu_dn8 = assign82270_e123289_d_n8;
        var_nu_dn12 = assign82270_e123289_d_n12;
        var_nu_dn13 = assign82270_e123289_d_n13;
        var_nu_dn14 = assign82270_e123289_d_n14;
        var_nu_dn15 = assign82270_e123289_d_n15;
        var_nu_dn16 = assign82270_e123289_d_n16;
        var_nu_dn17 = assign82270_e123289_d_n17;
        var_nu_dn18 = assign82270_e123289_d_n18;
        var_nu_dn19 = assign82270_e123289_d_n19;
        var_nu_dn20 = assign82270_e123289_d_n20;

        let (assign82280_e123324, assign82280_e123324_d_n5, assign82280_e123324_d_n6, assign82280_e123324_d_n7, assign82280_e123324_d_n8, assign82280_e123324_d_n12, assign82280_e123324_d_n13, assign82280_e123324_d_n14, assign82280_e123324_d_n15, assign82280_e123324_d_n16, assign82280_e123324_d_n17, assign82280_e123324_d_n18, assign82280_e123324_d_n19, assign82280_e123324_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) {
        let assign82280_e123312: f64 = (var_nu * var_nu);
        let assign82280_e123317: f64 = (var_nqs_c * var_nqs_c);
        let assign82280_e123318: f64 = (0.5 * assign82280_e123317);
        let assign82280_e123320: f64 = (assign82280_e123318 - var_nqs_a);
        let assign82280_e123321: f64 = (var_nqs_tau * assign82280_e123320);
        let assign82280_e123322: f64 = (assign82280_e123312 + assign82280_e123321);
        (assign82280_e123322, (((var_nu_dn5 * var_nu) + (var_nu * var_nu_dn5)) + ((var_nqs_tau_dn5 * assign82280_e123320) + (var_nqs_tau * ((0.5 * ((var_nqs_c_dn5 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn5))) - var_nqs_a_dn5)))), (((var_nu_dn6 * var_nu) + (var_nu * var_nu_dn6)) + ((var_nqs_tau_dn6 * assign82280_e123320) + (var_nqs_tau * ((0.5 * ((var_nqs_c_dn6 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn6))) - var_nqs_a_dn6)))), (((var_nu_dn7 * var_nu) + (var_nu * var_nu_dn7)) + ((var_nqs_tau_dn7 * assign82280_e123320) + (var_nqs_tau * ((0.5 * ((var_nqs_c_dn7 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn7))) - var_nqs_a_dn7)))), (((var_nu_dn8 * var_nu) + (var_nu * var_nu_dn8)) + ((var_nqs_tau_dn8 * assign82280_e123320) + (var_nqs_tau * ((0.5 * ((var_nqs_c_dn8 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn8))) - var_nqs_a_dn8)))), (((var_nu_dn12 * var_nu) + (var_nu * var_nu_dn12)) + ((var_nqs_tau_dn12 * assign82280_e123320) + (var_nqs_tau * ((0.5 * ((var_nqs_c_dn12 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn12))) - var_nqs_a_dn12)))), (((var_nu_dn13 * var_nu) + (var_nu * var_nu_dn13)) + ((var_nqs_tau_dn13 * assign82280_e123320) + (var_nqs_tau * ((0.5 * ((var_nqs_c_dn13 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn13))) - var_nqs_a_dn13)))), (((var_nu_dn14 * var_nu) + (var_nu * var_nu_dn14)) + ((var_nqs_tau_dn14 * assign82280_e123320) + (var_nqs_tau * ((0.5 * ((var_nqs_c_dn14 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn14))) - var_nqs_a_dn14)))), (((var_nu_dn15 * var_nu) + (var_nu * var_nu_dn15)) + ((var_nqs_tau_dn15 * assign82280_e123320) + (var_nqs_tau * ((0.5 * ((var_nqs_c_dn15 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn15))) - var_nqs_a_dn15)))), (((var_nu_dn16 * var_nu) + (var_nu * var_nu_dn16)) + ((var_nqs_tau_dn16 * assign82280_e123320) + (var_nqs_tau * ((0.5 * ((var_nqs_c_dn16 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn16))) - var_nqs_a_dn16)))), (((var_nu_dn17 * var_nu) + (var_nu * var_nu_dn17)) + ((var_nqs_tau_dn17 * assign82280_e123320) + (var_nqs_tau * ((0.5 * ((var_nqs_c_dn17 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn17))) - var_nqs_a_dn17)))), (((var_nu_dn18 * var_nu) + (var_nu * var_nu_dn18)) + ((var_nqs_tau_dn18 * assign82280_e123320) + (var_nqs_tau * ((0.5 * ((var_nqs_c_dn18 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn18))) - var_nqs_a_dn18)))), (((var_nu_dn19 * var_nu) + (var_nu * var_nu_dn19)) + ((var_nqs_tau_dn19 * assign82280_e123320) + (var_nqs_tau * ((0.5 * ((var_nqs_c_dn19 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn19))) - var_nqs_a_dn19)))), (((var_nu_dn20 * var_nu) + (var_nu * var_nu_dn20)) + ((var_nqs_tau_dn20 * assign82280_e123320) + (var_nqs_tau * ((0.5 * ((var_nqs_c_dn20 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn20))) - var_nqs_a_dn20)))),)
    } else {
        (var_mutau, var_mutau_dn5, var_mutau_dn6, var_mutau_dn7, var_mutau_dn8, var_mutau_dn12, var_mutau_dn13, var_mutau_dn14, var_mutau_dn15, var_mutau_dn16, var_mutau_dn17, var_mutau_dn18, var_mutau_dn19, var_mutau_dn20,)
    }
};
        var_mutau = assign82280_e123324;
        var_mutau_dn5 = assign82280_e123324_d_n5;
        var_mutau_dn6 = assign82280_e123324_d_n6;
        var_mutau_dn7 = assign82280_e123324_d_n7;
        var_mutau_dn8 = assign82280_e123324_d_n8;
        var_mutau_dn12 = assign82280_e123324_d_n12;
        var_mutau_dn13 = assign82280_e123324_d_n13;
        var_mutau_dn14 = assign82280_e123324_d_n14;
        var_mutau_dn15 = assign82280_e123324_d_n15;
        var_mutau_dn16 = assign82280_e123324_d_n16;
        var_mutau_dn17 = assign82280_e123324_d_n17;
        var_mutau_dn18 = assign82280_e123324_d_n18;
        var_mutau_dn19 = assign82280_e123324_d_n19;
        var_mutau_dn20 = assign82280_e123324_d_n20;

        let (assign82290_e123373, assign82290_e123373_d_n5, assign82290_e123373_d_n6, assign82290_e123373_d_n7, assign82290_e123373_d_n8, assign82290_e123373_d_n12, assign82290_e123373_d_n13, assign82290_e123373_d_n14, assign82290_e123373_d_n15, assign82290_e123373_d_n16, assign82290_e123373_d_n17, assign82290_e123373_d_n18, assign82290_e123373_d_n19, assign82290_e123373_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) {
        let assign82290_e123348: f64 = (var_nqs_a * var_nu);
        let assign82290_e123350: f64 = (assign82290_e123348 * var_nqs_tau);
        let assign82290_e123354: f64 = (var_nu / var_mutau);
        let assign82290_e123356: f64 = (assign82290_e123354 * var_nqs_tau);
        let assign82290_e123358: f64 = (assign82290_e123356 * var_nqs_tau);
        let assign82290_e123360: f64 = (assign82290_e123358 * var_nqs_c);
        let assign82290_e123363: f64 = (var_nqs_c * var_nqs_c);
        let assign82290_e123365: f64 = (assign82290_e123363 * 0.3333333333333333);
        let assign82290_e123367: f64 = (assign82290_e123365 - var_nqs_a);
        let assign82290_e123368: f64 = (assign82290_e123360 * assign82290_e123367);
        let assign82290_e123369: f64 = (var_mutau + assign82290_e123368);
        let assign82290_e123370: f64 = (assign82290_e123350 / assign82290_e123369);
        let assign82290_e123371: f64 = (var_nqs_eta + assign82290_e123370);
        (assign82290_e123371, (var_nqs_eta_dn5 + (((((((var_nqs_a_dn5 * var_nu) + (var_nqs_a * var_nu_dn5)) * var_nqs_tau) + (assign82290_e123348 * var_nqs_tau_dn5)) * assign82290_e123369) - (assign82290_e123350 * (var_mutau_dn5 + (((((((((((var_nu_dn5 * var_mutau) - (var_nu * var_mutau_dn5)) / (var_mutau * var_mutau)) * var_nqs_tau) + (assign82290_e123354 * var_nqs_tau_dn5)) * var_nqs_tau) + (assign82290_e123356 * var_nqs_tau_dn5)) * var_nqs_c) + (assign82290_e123358 * var_nqs_c_dn5)) * assign82290_e123367) + (assign82290_e123360 * ((((var_nqs_c_dn5 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn5)) * 0.3333333333333333) - var_nqs_a_dn5)))))) / (assign82290_e123369 * assign82290_e123369))), (var_nqs_eta_dn6 + (((((((var_nqs_a_dn6 * var_nu) + (var_nqs_a * var_nu_dn6)) * var_nqs_tau) + (assign82290_e123348 * var_nqs_tau_dn6)) * assign82290_e123369) - (assign82290_e123350 * (var_mutau_dn6 + (((((((((((var_nu_dn6 * var_mutau) - (var_nu * var_mutau_dn6)) / (var_mutau * var_mutau)) * var_nqs_tau) + (assign82290_e123354 * var_nqs_tau_dn6)) * var_nqs_tau) + (assign82290_e123356 * var_nqs_tau_dn6)) * var_nqs_c) + (assign82290_e123358 * var_nqs_c_dn6)) * assign82290_e123367) + (assign82290_e123360 * ((((var_nqs_c_dn6 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn6)) * 0.3333333333333333) - var_nqs_a_dn6)))))) / (assign82290_e123369 * assign82290_e123369))), (var_nqs_eta_dn7 + (((((((var_nqs_a_dn7 * var_nu) + (var_nqs_a * var_nu_dn7)) * var_nqs_tau) + (assign82290_e123348 * var_nqs_tau_dn7)) * assign82290_e123369) - (assign82290_e123350 * (var_mutau_dn7 + (((((((((((var_nu_dn7 * var_mutau) - (var_nu * var_mutau_dn7)) / (var_mutau * var_mutau)) * var_nqs_tau) + (assign82290_e123354 * var_nqs_tau_dn7)) * var_nqs_tau) + (assign82290_e123356 * var_nqs_tau_dn7)) * var_nqs_c) + (assign82290_e123358 * var_nqs_c_dn7)) * assign82290_e123367) + (assign82290_e123360 * ((((var_nqs_c_dn7 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn7)) * 0.3333333333333333) - var_nqs_a_dn7)))))) / (assign82290_e123369 * assign82290_e123369))), (var_nqs_eta_dn8 + (((((((var_nqs_a_dn8 * var_nu) + (var_nqs_a * var_nu_dn8)) * var_nqs_tau) + (assign82290_e123348 * var_nqs_tau_dn8)) * assign82290_e123369) - (assign82290_e123350 * (var_mutau_dn8 + (((((((((((var_nu_dn8 * var_mutau) - (var_nu * var_mutau_dn8)) / (var_mutau * var_mutau)) * var_nqs_tau) + (assign82290_e123354 * var_nqs_tau_dn8)) * var_nqs_tau) + (assign82290_e123356 * var_nqs_tau_dn8)) * var_nqs_c) + (assign82290_e123358 * var_nqs_c_dn8)) * assign82290_e123367) + (assign82290_e123360 * ((((var_nqs_c_dn8 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn8)) * 0.3333333333333333) - var_nqs_a_dn8)))))) / (assign82290_e123369 * assign82290_e123369))), (var_nqs_eta_dn12 + (((((((var_nqs_a_dn12 * var_nu) + (var_nqs_a * var_nu_dn12)) * var_nqs_tau) + (assign82290_e123348 * var_nqs_tau_dn12)) * assign82290_e123369) - (assign82290_e123350 * (var_mutau_dn12 + (((((((((((var_nu_dn12 * var_mutau) - (var_nu * var_mutau_dn12)) / (var_mutau * var_mutau)) * var_nqs_tau) + (assign82290_e123354 * var_nqs_tau_dn12)) * var_nqs_tau) + (assign82290_e123356 * var_nqs_tau_dn12)) * var_nqs_c) + (assign82290_e123358 * var_nqs_c_dn12)) * assign82290_e123367) + (assign82290_e123360 * ((((var_nqs_c_dn12 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn12)) * 0.3333333333333333) - var_nqs_a_dn12)))))) / (assign82290_e123369 * assign82290_e123369))), (var_nqs_eta_dn13 + (((((((var_nqs_a_dn13 * var_nu) + (var_nqs_a * var_nu_dn13)) * var_nqs_tau) + (assign82290_e123348 * var_nqs_tau_dn13)) * assign82290_e123369) - (assign82290_e123350 * (var_mutau_dn13 + (((((((((((var_nu_dn13 * var_mutau) - (var_nu * var_mutau_dn13)) / (var_mutau * var_mutau)) * var_nqs_tau) + (assign82290_e123354 * var_nqs_tau_dn13)) * var_nqs_tau) + (assign82290_e123356 * var_nqs_tau_dn13)) * var_nqs_c) + (assign82290_e123358 * var_nqs_c_dn13)) * assign82290_e123367) + (assign82290_e123360 * ((((var_nqs_c_dn13 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn13)) * 0.3333333333333333) - var_nqs_a_dn13)))))) / (assign82290_e123369 * assign82290_e123369))), (var_nqs_eta_dn14 + (((((((var_nqs_a_dn14 * var_nu) + (var_nqs_a * var_nu_dn14)) * var_nqs_tau) + (assign82290_e123348 * var_nqs_tau_dn14)) * assign82290_e123369) - (assign82290_e123350 * (var_mutau_dn14 + (((((((((((var_nu_dn14 * var_mutau) - (var_nu * var_mutau_dn14)) / (var_mutau * var_mutau)) * var_nqs_tau) + (assign82290_e123354 * var_nqs_tau_dn14)) * var_nqs_tau) + (assign82290_e123356 * var_nqs_tau_dn14)) * var_nqs_c) + (assign82290_e123358 * var_nqs_c_dn14)) * assign82290_e123367) + (assign82290_e123360 * ((((var_nqs_c_dn14 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn14)) * 0.3333333333333333) - var_nqs_a_dn14)))))) / (assign82290_e123369 * assign82290_e123369))), (var_nqs_eta_dn15 + (((((((var_nqs_a_dn15 * var_nu) + (var_nqs_a * var_nu_dn15)) * var_nqs_tau) + (assign82290_e123348 * var_nqs_tau_dn15)) * assign82290_e123369) - (assign82290_e123350 * (var_mutau_dn15 + (((((((((((var_nu_dn15 * var_mutau) - (var_nu * var_mutau_dn15)) / (var_mutau * var_mutau)) * var_nqs_tau) + (assign82290_e123354 * var_nqs_tau_dn15)) * var_nqs_tau) + (assign82290_e123356 * var_nqs_tau_dn15)) * var_nqs_c) + (assign82290_e123358 * var_nqs_c_dn15)) * assign82290_e123367) + (assign82290_e123360 * ((((var_nqs_c_dn15 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn15)) * 0.3333333333333333) - var_nqs_a_dn15)))))) / (assign82290_e123369 * assign82290_e123369))), (var_nqs_eta_dn16 + (((((((var_nqs_a_dn16 * var_nu) + (var_nqs_a * var_nu_dn16)) * var_nqs_tau) + (assign82290_e123348 * var_nqs_tau_dn16)) * assign82290_e123369) - (assign82290_e123350 * (var_mutau_dn16 + (((((((((((var_nu_dn16 * var_mutau) - (var_nu * var_mutau_dn16)) / (var_mutau * var_mutau)) * var_nqs_tau) + (assign82290_e123354 * var_nqs_tau_dn16)) * var_nqs_tau) + (assign82290_e123356 * var_nqs_tau_dn16)) * var_nqs_c) + (assign82290_e123358 * var_nqs_c_dn16)) * assign82290_e123367) + (assign82290_e123360 * ((((var_nqs_c_dn16 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn16)) * 0.3333333333333333) - var_nqs_a_dn16)))))) / (assign82290_e123369 * assign82290_e123369))), (var_nqs_eta_dn17 + (((((((var_nqs_a_dn17 * var_nu) + (var_nqs_a * var_nu_dn17)) * var_nqs_tau) + (assign82290_e123348 * var_nqs_tau_dn17)) * assign82290_e123369) - (assign82290_e123350 * (var_mutau_dn17 + (((((((((((var_nu_dn17 * var_mutau) - (var_nu * var_mutau_dn17)) / (var_mutau * var_mutau)) * var_nqs_tau) + (assign82290_e123354 * var_nqs_tau_dn17)) * var_nqs_tau) + (assign82290_e123356 * var_nqs_tau_dn17)) * var_nqs_c) + (assign82290_e123358 * var_nqs_c_dn17)) * assign82290_e123367) + (assign82290_e123360 * ((((var_nqs_c_dn17 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn17)) * 0.3333333333333333) - var_nqs_a_dn17)))))) / (assign82290_e123369 * assign82290_e123369))), (var_nqs_eta_dn18 + (((((((var_nqs_a_dn18 * var_nu) + (var_nqs_a * var_nu_dn18)) * var_nqs_tau) + (assign82290_e123348 * var_nqs_tau_dn18)) * assign82290_e123369) - (assign82290_e123350 * (var_mutau_dn18 + (((((((((((var_nu_dn18 * var_mutau) - (var_nu * var_mutau_dn18)) / (var_mutau * var_mutau)) * var_nqs_tau) + (assign82290_e123354 * var_nqs_tau_dn18)) * var_nqs_tau) + (assign82290_e123356 * var_nqs_tau_dn18)) * var_nqs_c) + (assign82290_e123358 * var_nqs_c_dn18)) * assign82290_e123367) + (assign82290_e123360 * ((((var_nqs_c_dn18 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn18)) * 0.3333333333333333) - var_nqs_a_dn18)))))) / (assign82290_e123369 * assign82290_e123369))), (var_nqs_eta_dn19 + (((((((var_nqs_a_dn19 * var_nu) + (var_nqs_a * var_nu_dn19)) * var_nqs_tau) + (assign82290_e123348 * var_nqs_tau_dn19)) * assign82290_e123369) - (assign82290_e123350 * (var_mutau_dn19 + (((((((((((var_nu_dn19 * var_mutau) - (var_nu * var_mutau_dn19)) / (var_mutau * var_mutau)) * var_nqs_tau) + (assign82290_e123354 * var_nqs_tau_dn19)) * var_nqs_tau) + (assign82290_e123356 * var_nqs_tau_dn19)) * var_nqs_c) + (assign82290_e123358 * var_nqs_c_dn19)) * assign82290_e123367) + (assign82290_e123360 * ((((var_nqs_c_dn19 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn19)) * 0.3333333333333333) - var_nqs_a_dn19)))))) / (assign82290_e123369 * assign82290_e123369))), (var_nqs_eta_dn20 + (((((((var_nqs_a_dn20 * var_nu) + (var_nqs_a * var_nu_dn20)) * var_nqs_tau) + (assign82290_e123348 * var_nqs_tau_dn20)) * assign82290_e123369) - (assign82290_e123350 * (var_mutau_dn20 + (((((((((((var_nu_dn20 * var_mutau) - (var_nu * var_mutau_dn20)) / (var_mutau * var_mutau)) * var_nqs_tau) + (assign82290_e123354 * var_nqs_tau_dn20)) * var_nqs_tau) + (assign82290_e123356 * var_nqs_tau_dn20)) * var_nqs_c) + (assign82290_e123358 * var_nqs_c_dn20)) * assign82290_e123367) + (assign82290_e123360 * ((((var_nqs_c_dn20 * var_nqs_c) + (var_nqs_c * var_nqs_c_dn20)) * 0.3333333333333333) - var_nqs_a_dn20)))))) / (assign82290_e123369 * assign82290_e123369))),)
    } else {
        (var_nqs_y0, var_nqs_y0_dn5, var_nqs_y0_dn6, var_nqs_y0_dn7, var_nqs_y0_dn8, var_nqs_y0_dn12, var_nqs_y0_dn13, var_nqs_y0_dn14, var_nqs_y0_dn15, var_nqs_y0_dn16, var_nqs_y0_dn17, var_nqs_y0_dn18, var_nqs_y0_dn19, var_nqs_y0_dn20,)
    }
};
        var_nqs_y0 = assign82290_e123373;
        var_nqs_y0_dn5 = assign82290_e123373_d_n5;
        var_nqs_y0_dn6 = assign82290_e123373_d_n6;
        var_nqs_y0_dn7 = assign82290_e123373_d_n7;
        var_nqs_y0_dn8 = assign82290_e123373_d_n8;
        var_nqs_y0_dn12 = assign82290_e123373_d_n12;
        var_nqs_y0_dn13 = assign82290_e123373_d_n13;
        var_nqs_y0_dn14 = assign82290_e123373_d_n14;
        var_nqs_y0_dn15 = assign82290_e123373_d_n15;
        var_nqs_y0_dn16 = assign82290_e123373_d_n16;
        var_nqs_y0_dn17 = assign82290_e123373_d_n17;
        var_nqs_y0_dn18 = assign82290_e123373_d_n18;
        var_nqs_y0_dn19 = assign82290_e123373_d_n19;
        var_nqs_y0_dn20 = assign82290_e123373_d_n20;

        let assign82300_e123375: f64 = (var_nqs_y0).abs();
        let assign82300_e123377: f64 = if assign82300_e123375 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard2238 = assign82300_e123377;

        *var_guard2236_slot = var_guard2236;
        *var_guard2237_slot = var_guard2237;
        *var_guard2238_slot = var_guard2238;
        *var_mutau_slot = var_mutau;
        *var_mutau_dn12_slot = var_mutau_dn12;
        *var_mutau_dn13_slot = var_mutau_dn13;
        *var_mutau_dn14_slot = var_mutau_dn14;
        *var_mutau_dn15_slot = var_mutau_dn15;
        *var_mutau_dn16_slot = var_mutau_dn16;
        *var_mutau_dn17_slot = var_mutau_dn17;
        *var_mutau_dn18_slot = var_mutau_dn18;
        *var_mutau_dn19_slot = var_mutau_dn19;
        *var_mutau_dn20_slot = var_mutau_dn20;
        *var_mutau_dn5_slot = var_mutau_dn5;
        *var_mutau_dn6_slot = var_mutau_dn6;
        *var_mutau_dn7_slot = var_mutau_dn7;
        *var_mutau_dn8_slot = var_mutau_dn8;
        *var_nqs_a_slot = var_nqs_a;
        *var_nqs_a_dn12_slot = var_nqs_a_dn12;
        *var_nqs_a_dn13_slot = var_nqs_a_dn13;
        *var_nqs_a_dn14_slot = var_nqs_a_dn14;
        *var_nqs_a_dn15_slot = var_nqs_a_dn15;
        *var_nqs_a_dn16_slot = var_nqs_a_dn16;
        *var_nqs_a_dn17_slot = var_nqs_a_dn17;
        *var_nqs_a_dn18_slot = var_nqs_a_dn18;
        *var_nqs_a_dn19_slot = var_nqs_a_dn19;
        *var_nqs_a_dn20_slot = var_nqs_a_dn20;
        *var_nqs_a_dn5_slot = var_nqs_a_dn5;
        *var_nqs_a_dn6_slot = var_nqs_a_dn6;
        *var_nqs_a_dn7_slot = var_nqs_a_dn7;
        *var_nqs_a_dn8_slot = var_nqs_a_dn8;
        *var_nqs_c_slot = var_nqs_c;
        *var_nqs_c_dn12_slot = var_nqs_c_dn12;
        *var_nqs_c_dn13_slot = var_nqs_c_dn13;
        *var_nqs_c_dn14_slot = var_nqs_c_dn14;
        *var_nqs_c_dn15_slot = var_nqs_c_dn15;
        *var_nqs_c_dn16_slot = var_nqs_c_dn16;
        *var_nqs_c_dn17_slot = var_nqs_c_dn17;
        *var_nqs_c_dn18_slot = var_nqs_c_dn18;
        *var_nqs_c_dn19_slot = var_nqs_c_dn19;
        *var_nqs_c_dn20_slot = var_nqs_c_dn20;
        *var_nqs_c_dn5_slot = var_nqs_c_dn5;
        *var_nqs_c_dn6_slot = var_nqs_c_dn6;
        *var_nqs_c_dn7_slot = var_nqs_c_dn7;
        *var_nqs_c_dn8_slot = var_nqs_c_dn8;
        *var_nqs_d0_slot = var_nqs_d0;
        *var_nqs_d0_dn12_slot = var_nqs_d0_dn12;
        *var_nqs_d0_dn13_slot = var_nqs_d0_dn13;
        *var_nqs_d0_dn14_slot = var_nqs_d0_dn14;
        *var_nqs_d0_dn15_slot = var_nqs_d0_dn15;
        *var_nqs_d0_dn16_slot = var_nqs_d0_dn16;
        *var_nqs_d0_dn17_slot = var_nqs_d0_dn17;
        *var_nqs_d0_dn18_slot = var_nqs_d0_dn18;
        *var_nqs_d0_dn19_slot = var_nqs_d0_dn19;
        *var_nqs_d0_dn20_slot = var_nqs_d0_dn20;
        *var_nqs_d0_dn5_slot = var_nqs_d0_dn5;
        *var_nqs_d0_dn6_slot = var_nqs_d0_dn6;
        *var_nqs_d0_dn7_slot = var_nqs_d0_dn7;
        *var_nqs_d0_dn8_slot = var_nqs_d0_dn8;
        *var_nqs_eta_slot = var_nqs_eta;
        *var_nqs_eta_dn12_slot = var_nqs_eta_dn12;
        *var_nqs_eta_dn13_slot = var_nqs_eta_dn13;
        *var_nqs_eta_dn14_slot = var_nqs_eta_dn14;
        *var_nqs_eta_dn15_slot = var_nqs_eta_dn15;
        *var_nqs_eta_dn16_slot = var_nqs_eta_dn16;
        *var_nqs_eta_dn17_slot = var_nqs_eta_dn17;
        *var_nqs_eta_dn18_slot = var_nqs_eta_dn18;
        *var_nqs_eta_dn19_slot = var_nqs_eta_dn19;
        *var_nqs_eta_dn20_slot = var_nqs_eta_dn20;
        *var_nqs_eta_dn5_slot = var_nqs_eta_dn5;
        *var_nqs_eta_dn6_slot = var_nqs_eta_dn6;
        *var_nqs_eta_dn7_slot = var_nqs_eta_dn7;
        *var_nqs_eta_dn8_slot = var_nqs_eta_dn8;
        *var_nqs_p_slot = var_nqs_p;
        *var_nqs_p_dn12_slot = var_nqs_p_dn12;
        *var_nqs_p_dn13_slot = var_nqs_p_dn13;
        *var_nqs_p_dn14_slot = var_nqs_p_dn14;
        *var_nqs_p_dn15_slot = var_nqs_p_dn15;
        *var_nqs_p_dn16_slot = var_nqs_p_dn16;
        *var_nqs_p_dn17_slot = var_nqs_p_dn17;
        *var_nqs_p_dn18_slot = var_nqs_p_dn18;
        *var_nqs_p_dn19_slot = var_nqs_p_dn19;
        *var_nqs_p_dn20_slot = var_nqs_p_dn20;
        *var_nqs_p_dn5_slot = var_nqs_p_dn5;
        *var_nqs_p_dn6_slot = var_nqs_p_dn6;
        *var_nqs_p_dn7_slot = var_nqs_p_dn7;
        *var_nqs_p_dn8_slot = var_nqs_p_dn8;
        *var_nqs_q_slot = var_nqs_q;
        *var_nqs_q_dn12_slot = var_nqs_q_dn12;
        *var_nqs_q_dn13_slot = var_nqs_q_dn13;
        *var_nqs_q_dn14_slot = var_nqs_q_dn14;
        *var_nqs_q_dn15_slot = var_nqs_q_dn15;
        *var_nqs_q_dn16_slot = var_nqs_q_dn16;
        *var_nqs_q_dn17_slot = var_nqs_q_dn17;
        *var_nqs_q_dn18_slot = var_nqs_q_dn18;
        *var_nqs_q_dn19_slot = var_nqs_q_dn19;
        *var_nqs_q_dn20_slot = var_nqs_q_dn20;
        *var_nqs_q_dn5_slot = var_nqs_q_dn5;
        *var_nqs_q_dn6_slot = var_nqs_q_dn6;
        *var_nqs_q_dn7_slot = var_nqs_q_dn7;
        *var_nqs_q_dn8_slot = var_nqs_q_dn8;
        *var_nqs_tau_slot = var_nqs_tau;
        *var_nqs_tau_dn12_slot = var_nqs_tau_dn12;
        *var_nqs_tau_dn13_slot = var_nqs_tau_dn13;
        *var_nqs_tau_dn14_slot = var_nqs_tau_dn14;
        *var_nqs_tau_dn15_slot = var_nqs_tau_dn15;
        *var_nqs_tau_dn16_slot = var_nqs_tau_dn16;
        *var_nqs_tau_dn17_slot = var_nqs_tau_dn17;
        *var_nqs_tau_dn18_slot = var_nqs_tau_dn18;
        *var_nqs_tau_dn19_slot = var_nqs_tau_dn19;
        *var_nqs_tau_dn20_slot = var_nqs_tau_dn20;
        *var_nqs_tau_dn5_slot = var_nqs_tau_dn5;
        *var_nqs_tau_dn6_slot = var_nqs_tau_dn6;
        *var_nqs_tau_dn7_slot = var_nqs_tau_dn7;
        *var_nqs_tau_dn8_slot = var_nqs_tau_dn8;
        *var_nqs_temp_slot = var_nqs_temp;
        *var_nqs_temp_dn12_slot = var_nqs_temp_dn12;
        *var_nqs_temp_dn13_slot = var_nqs_temp_dn13;
        *var_nqs_temp_dn14_slot = var_nqs_temp_dn14;
        *var_nqs_temp_dn15_slot = var_nqs_temp_dn15;
        *var_nqs_temp_dn16_slot = var_nqs_temp_dn16;
        *var_nqs_temp_dn17_slot = var_nqs_temp_dn17;
        *var_nqs_temp_dn18_slot = var_nqs_temp_dn18;
        *var_nqs_temp_dn19_slot = var_nqs_temp_dn19;
        *var_nqs_temp_dn20_slot = var_nqs_temp_dn20;
        *var_nqs_temp_dn5_slot = var_nqs_temp_dn5;
        *var_nqs_temp_dn6_slot = var_nqs_temp_dn6;
        *var_nqs_temp_dn7_slot = var_nqs_temp_dn7;
        *var_nqs_temp_dn8_slot = var_nqs_temp_dn8;
        *var_nqs_u_slot = var_nqs_u;
        *var_nqs_u_dn12_slot = var_nqs_u_dn12;
        *var_nqs_u_dn13_slot = var_nqs_u_dn13;
        *var_nqs_u_dn14_slot = var_nqs_u_dn14;
        *var_nqs_u_dn15_slot = var_nqs_u_dn15;
        *var_nqs_u_dn16_slot = var_nqs_u_dn16;
        *var_nqs_u_dn17_slot = var_nqs_u_dn17;
        *var_nqs_u_dn18_slot = var_nqs_u_dn18;
        *var_nqs_u_dn19_slot = var_nqs_u_dn19;
        *var_nqs_u_dn20_slot = var_nqs_u_dn20;
        *var_nqs_u_dn5_slot = var_nqs_u_dn5;
        *var_nqs_u_dn6_slot = var_nqs_u_dn6;
        *var_nqs_u_dn7_slot = var_nqs_u_dn7;
        *var_nqs_u_dn8_slot = var_nqs_u_dn8;
        *var_nqs_xi_slot = var_nqs_xi;
        *var_nqs_xi_dn12_slot = var_nqs_xi_dn12;
        *var_nqs_xi_dn13_slot = var_nqs_xi_dn13;
        *var_nqs_xi_dn14_slot = var_nqs_xi_dn14;
        *var_nqs_xi_dn15_slot = var_nqs_xi_dn15;
        *var_nqs_xi_dn16_slot = var_nqs_xi_dn16;
        *var_nqs_xi_dn17_slot = var_nqs_xi_dn17;
        *var_nqs_xi_dn18_slot = var_nqs_xi_dn18;
        *var_nqs_xi_dn19_slot = var_nqs_xi_dn19;
        *var_nqs_xi_dn20_slot = var_nqs_xi_dn20;
        *var_nqs_xi_dn5_slot = var_nqs_xi_dn5;
        *var_nqs_xi_dn6_slot = var_nqs_xi_dn6;
        *var_nqs_xi_dn7_slot = var_nqs_xi_dn7;
        *var_nqs_xi_dn8_slot = var_nqs_xi_dn8;
        *var_nqs_y0_slot = var_nqs_y0;
        *var_nqs_y0_dn12_slot = var_nqs_y0_dn12;
        *var_nqs_y0_dn13_slot = var_nqs_y0_dn13;
        *var_nqs_y0_dn14_slot = var_nqs_y0_dn14;
        *var_nqs_y0_dn15_slot = var_nqs_y0_dn15;
        *var_nqs_y0_dn16_slot = var_nqs_y0_dn16;
        *var_nqs_y0_dn17_slot = var_nqs_y0_dn17;
        *var_nqs_y0_dn18_slot = var_nqs_y0_dn18;
        *var_nqs_y0_dn19_slot = var_nqs_y0_dn19;
        *var_nqs_y0_dn20_slot = var_nqs_y0_dn20;
        *var_nqs_y0_dn5_slot = var_nqs_y0_dn5;
        *var_nqs_y0_dn6_slot = var_nqs_y0_dn6;
        *var_nqs_y0_dn7_slot = var_nqs_y0_dn7;
        *var_nqs_y0_dn8_slot = var_nqs_y0_dn8;
        *var_nqs_yg_slot = var_nqs_yg;
        *var_nqs_yg_dn12_slot = var_nqs_yg_dn12;
        *var_nqs_yg_dn13_slot = var_nqs_yg_dn13;
        *var_nqs_yg_dn14_slot = var_nqs_yg_dn14;
        *var_nqs_yg_dn15_slot = var_nqs_yg_dn15;
        *var_nqs_yg_dn16_slot = var_nqs_yg_dn16;
        *var_nqs_yg_dn17_slot = var_nqs_yg_dn17;
        *var_nqs_yg_dn18_slot = var_nqs_yg_dn18;
        *var_nqs_yg_dn19_slot = var_nqs_yg_dn19;
        *var_nqs_yg_dn20_slot = var_nqs_yg_dn20;
        *var_nqs_yg_dn5_slot = var_nqs_yg_dn5;
        *var_nqs_yg_dn6_slot = var_nqs_yg_dn6;
        *var_nqs_yg_dn7_slot = var_nqs_yg_dn7;
        *var_nqs_yg_dn8_slot = var_nqs_yg_dn8;
        *var_nqs_z_slot = var_nqs_z;
        *var_nqs_z_dn12_slot = var_nqs_z_dn12;
        *var_nqs_z_dn13_slot = var_nqs_z_dn13;
        *var_nqs_z_dn14_slot = var_nqs_z_dn14;
        *var_nqs_z_dn15_slot = var_nqs_z_dn15;
        *var_nqs_z_dn16_slot = var_nqs_z_dn16;
        *var_nqs_z_dn17_slot = var_nqs_z_dn17;
        *var_nqs_z_dn18_slot = var_nqs_z_dn18;
        *var_nqs_z_dn19_slot = var_nqs_z_dn19;
        *var_nqs_z_dn20_slot = var_nqs_z_dn20;
        *var_nqs_z_dn5_slot = var_nqs_z_dn5;
        *var_nqs_z_dn6_slot = var_nqs_z_dn6;
        *var_nqs_z_dn7_slot = var_nqs_z_dn7;
        *var_nqs_z_dn8_slot = var_nqs_z_dn8;
        *var_nu_slot = var_nu;
        *var_nu_dn12_slot = var_nu_dn12;
        *var_nu_dn13_slot = var_nu_dn13;
        *var_nu_dn14_slot = var_nu_dn14;
        *var_nu_dn15_slot = var_nu_dn15;
        *var_nu_dn16_slot = var_nu_dn16;
        *var_nu_dn17_slot = var_nu_dn17;
        *var_nu_dn18_slot = var_nu_dn18;
        *var_nu_dn19_slot = var_nu_dn19;
        *var_nu_dn20_slot = var_nu_dn20;
        *var_nu_dn5_slot = var_nu_dn5;
        *var_nu_dn6_slot = var_nu_dn6;
        *var_nu_dn7_slot = var_nu_dn7;
        *var_nu_dn8_slot = var_nu_dn8;
        *var_temp8_slot = var_temp8;
        *var_temp8_dn12_slot = var_temp8_dn12;
        *var_temp8_dn13_slot = var_temp8_dn13;
        *var_temp8_dn14_slot = var_temp8_dn14;
        *var_temp8_dn15_slot = var_temp8_dn15;
        *var_temp8_dn16_slot = var_temp8_dn16;
        *var_temp8_dn17_slot = var_temp8_dn17;
        *var_temp8_dn18_slot = var_temp8_dn18;
        *var_temp8_dn19_slot = var_temp8_dn19;
        *var_temp8_dn20_slot = var_temp8_dn20;
        *var_temp8_dn5_slot = var_temp8_dn5;
        *var_temp8_dn6_slot = var_temp8_dn6;
        *var_temp8_dn7_slot = var_temp8_dn7;
        *var_temp8_dn8_slot = var_temp8_dn8;
        *var_temp9_slot = var_temp9;
        *var_temp9_dn12_slot = var_temp9_dn12;
        *var_temp9_dn13_slot = var_temp9_dn13;
        *var_temp9_dn14_slot = var_temp9_dn14;
        *var_temp9_dn15_slot = var_temp9_dn15;
        *var_temp9_dn16_slot = var_temp9_dn16;
        *var_temp9_dn17_slot = var_temp9_dn17;
        *var_temp9_dn18_slot = var_temp9_dn18;
        *var_temp9_dn19_slot = var_temp9_dn19;
        *var_temp9_dn20_slot = var_temp9_dn20;
        *var_temp9_dn5_slot = var_temp9_dn5;
        *var_temp9_dn6_slot = var_temp9_dn6;
        *var_temp9_dn7_slot = var_temp9_dn7;
        *var_temp9_dn8_slot = var_temp9_dn8;
        *var_temp__blk1038_slot = var_temp__blk1038;
        *var_temp__blk1038_dn12_slot = var_temp__blk1038_dn12;
        *var_temp__blk1038_dn13_slot = var_temp__blk1038_dn13;
        *var_temp__blk1038_dn14_slot = var_temp__blk1038_dn14;
        *var_temp__blk1038_dn15_slot = var_temp__blk1038_dn15;
        *var_temp__blk1038_dn16_slot = var_temp__blk1038_dn16;
        *var_temp__blk1038_dn17_slot = var_temp__blk1038_dn17;
        *var_temp__blk1038_dn18_slot = var_temp__blk1038_dn18;
        *var_temp__blk1038_dn19_slot = var_temp__blk1038_dn19;
        *var_temp__blk1038_dn20_slot = var_temp__blk1038_dn20;
        *var_temp__blk1038_dn5_slot = var_temp__blk1038_dn5;
        *var_temp__blk1038_dn6_slot = var_temp__blk1038_dn6;
        *var_temp__blk1038_dn7_slot = var_temp__blk1038_dn7;
        *var_temp__blk1038_dn8_slot = var_temp__blk1038_dn8;
    }

    pub(super) fn stamp_transient_block_257(
        var_a_factrp: f64,
        var_a_factrp_dn12: f64,
        var_a_factrp_dn13: f64,
        var_a_factrp_dn14: f64,
        var_a_factrp_dn15: f64,
        var_a_factrp_dn16: f64,
        var_a_factrp_dn17: f64,
        var_a_factrp_dn18: f64,
        var_a_factrp_dn19: f64,
        var_a_factrp_dn20: f64,
        var_a_factrp_dn5: f64,
        var_a_factrp_dn6: f64,
        var_a_factrp_dn7: f64,
        var_a_factrp_dn8: f64,
        var_gp: f64,
        var_gp2: f64,
        var_gp2_dn12: f64,
        var_gp2_dn13: f64,
        var_gp2_dn14: f64,
        var_gp2_dn15: f64,
        var_gp2_dn16: f64,
        var_gp2_dn17: f64,
        var_gp2_dn18: f64,
        var_gp2_dn19: f64,
        var_gp2_dn20: f64,
        var_gp2_dn5: f64,
        var_gp2_dn6: f64,
        var_gp2_dn7: f64,
        var_gp2_dn8: f64,
        var_gp_dn12: f64,
        var_gp_dn13: f64,
        var_gp_dn14: f64,
        var_gp_dn15: f64,
        var_gp_dn16: f64,
        var_gp_dn17: f64,
        var_gp_dn18: f64,
        var_gp_dn19: f64,
        var_gp_dn20: f64,
        var_gp_dn5: f64,
        var_gp_dn6: f64,
        var_gp_dn7: f64,
        var_gp_dn8: f64,
        var_guard2078: f64,
        var_guard2079: f64,
        var_guard2088: f64,
        var_guard2105: f64,
        var_guard2130: f64,
        var_guard2171: f64,
        var_guard2236: f64,
        var_guard2237: f64,
        var_guard2238: f64,
        var_nqs_y0: f64,
        var_nqs_y0_dn12: f64,
        var_nqs_y0_dn13: f64,
        var_nqs_y0_dn14: f64,
        var_nqs_y0_dn15: f64,
        var_nqs_y0_dn16: f64,
        var_nqs_y0_dn17: f64,
        var_nqs_y0_dn18: f64,
        var_nqs_y0_dn19: f64,
        var_nqs_y0_dn20: f64,
        var_nqs_y0_dn5: f64,
        var_nqs_y0_dn6: f64,
        var_nqs_y0_dn7: f64,
        var_nqs_y0_dn8: f64,
        var_nqs_yg: f64,
        var_nqs_yg_dn12: f64,
        var_nqs_yg_dn13: f64,
        var_nqs_yg_dn14: f64,
        var_nqs_yg_dn15: f64,
        var_nqs_yg_dn16: f64,
        var_nqs_yg_dn17: f64,
        var_nqs_yg_dn18: f64,
        var_nqs_yg_dn19: f64,
        var_nqs_yg_dn20: f64,
        var_nqs_yg_dn5: f64,
        var_nqs_yg_dn6: f64,
        var_nqs_yg_dn7: f64,
        var_nqs_yg_dn8: f64,
        var_temp__blk1038: f64,
        var_temp__blk1038_dn12: f64,
        var_temp__blk1038_dn13: f64,
        var_temp__blk1038_dn14: f64,
        var_temp__blk1038_dn15: f64,
        var_temp__blk1038_dn16: f64,
        var_temp__blk1038_dn17: f64,
        var_temp__blk1038_dn18: f64,
        var_temp__blk1038_dn19: f64,
        var_temp__blk1038_dn20: f64,
        var_temp__blk1038_dn5: f64,
        var_temp__blk1038_dn6: f64,
        var_temp__blk1038_dn7: f64,
        var_temp__blk1038_dn8: f64,
        var_guard2239_slot: &mut f64,
        var_guard2240_slot: &mut f64,
        var_guard2241_slot: &mut f64,
        var_guard2242_slot: &mut f64,
        var_nqs_a_fac_slot: &mut f64,
        var_nqs_a_fac_dn12_slot: &mut f64,
        var_nqs_a_fac_dn13_slot: &mut f64,
        var_nqs_a_fac_dn14_slot: &mut f64,
        var_nqs_a_fac_dn15_slot: &mut f64,
        var_nqs_a_fac_dn16_slot: &mut f64,
        var_nqs_a_fac_dn17_slot: &mut f64,
        var_nqs_a_fac_dn18_slot: &mut f64,
        var_nqs_a_fac_dn19_slot: &mut f64,
        var_nqs_a_fac_dn20_slot: &mut f64,
        var_nqs_a_fac_dn5_slot: &mut f64,
        var_nqs_a_fac_dn6_slot: &mut f64,
        var_nqs_a_fac_dn7_slot: &mut f64,
        var_nqs_a_fac_dn8_slot: &mut f64,
        var_nqs_d0_slot: &mut f64,
        var_nqs_d0_dn12_slot: &mut f64,
        var_nqs_d0_dn13_slot: &mut f64,
        var_nqs_d0_dn14_slot: &mut f64,
        var_nqs_d0_dn15_slot: &mut f64,
        var_nqs_d0_dn16_slot: &mut f64,
        var_nqs_d0_dn17_slot: &mut f64,
        var_nqs_d0_dn18_slot: &mut f64,
        var_nqs_d0_dn19_slot: &mut f64,
        var_nqs_d0_dn20_slot: &mut f64,
        var_nqs_d0_dn5_slot: &mut f64,
        var_nqs_d0_dn6_slot: &mut f64,
        var_nqs_d0_dn7_slot: &mut f64,
        var_nqs_d0_dn8_slot: &mut f64,
        var_nqs_p_slot: &mut f64,
        var_nqs_p_dn12_slot: &mut f64,
        var_nqs_p_dn13_slot: &mut f64,
        var_nqs_p_dn14_slot: &mut f64,
        var_nqs_p_dn15_slot: &mut f64,
        var_nqs_p_dn16_slot: &mut f64,
        var_nqs_p_dn17_slot: &mut f64,
        var_nqs_p_dn18_slot: &mut f64,
        var_nqs_p_dn19_slot: &mut f64,
        var_nqs_p_dn20_slot: &mut f64,
        var_nqs_p_dn5_slot: &mut f64,
        var_nqs_p_dn6_slot: &mut f64,
        var_nqs_p_dn7_slot: &mut f64,
        var_nqs_p_dn8_slot: &mut f64,
        var_nqs_q_slot: &mut f64,
        var_nqs_q_dn12_slot: &mut f64,
        var_nqs_q_dn13_slot: &mut f64,
        var_nqs_q_dn14_slot: &mut f64,
        var_nqs_q_dn15_slot: &mut f64,
        var_nqs_q_dn16_slot: &mut f64,
        var_nqs_q_dn17_slot: &mut f64,
        var_nqs_q_dn18_slot: &mut f64,
        var_nqs_q_dn19_slot: &mut f64,
        var_nqs_q_dn20_slot: &mut f64,
        var_nqs_q_dn5_slot: &mut f64,
        var_nqs_q_dn6_slot: &mut f64,
        var_nqs_q_dn7_slot: &mut f64,
        var_nqs_q_dn8_slot: &mut f64,
        var_nqs_temp_slot: &mut f64,
        var_nqs_temp_dn12_slot: &mut f64,
        var_nqs_temp_dn13_slot: &mut f64,
        var_nqs_temp_dn14_slot: &mut f64,
        var_nqs_temp_dn15_slot: &mut f64,
        var_nqs_temp_dn16_slot: &mut f64,
        var_nqs_temp_dn17_slot: &mut f64,
        var_nqs_temp_dn18_slot: &mut f64,
        var_nqs_temp_dn19_slot: &mut f64,
        var_nqs_temp_dn20_slot: &mut f64,
        var_nqs_temp_dn5_slot: &mut f64,
        var_nqs_temp_dn6_slot: &mut f64,
        var_nqs_temp_dn7_slot: &mut f64,
        var_nqs_temp_dn8_slot: &mut f64,
        var_nqs_w_slot: &mut f64,
        var_nqs_w_dn12_slot: &mut f64,
        var_nqs_w_dn13_slot: &mut f64,
        var_nqs_w_dn14_slot: &mut f64,
        var_nqs_w_dn15_slot: &mut f64,
        var_nqs_w_dn16_slot: &mut f64,
        var_nqs_w_dn17_slot: &mut f64,
        var_nqs_w_dn18_slot: &mut f64,
        var_nqs_w_dn19_slot: &mut f64,
        var_nqs_w_dn20_slot: &mut f64,
        var_nqs_w_dn5_slot: &mut f64,
        var_nqs_w_dn6_slot: &mut f64,
        var_nqs_w_dn7_slot: &mut f64,
        var_nqs_w_dn8_slot: &mut f64,
        var_nqs_x0_slot: &mut f64,
        var_nqs_x0_dn12_slot: &mut f64,
        var_nqs_x0_dn13_slot: &mut f64,
        var_nqs_x0_dn14_slot: &mut f64,
        var_nqs_x0_dn15_slot: &mut f64,
        var_nqs_x0_dn16_slot: &mut f64,
        var_nqs_x0_dn17_slot: &mut f64,
        var_nqs_x0_dn18_slot: &mut f64,
        var_nqs_x0_dn19_slot: &mut f64,
        var_nqs_x0_dn20_slot: &mut f64,
        var_nqs_x0_dn5_slot: &mut f64,
        var_nqs_x0_dn6_slot: &mut f64,
        var_nqs_x0_dn7_slot: &mut f64,
        var_nqs_x0_dn8_slot: &mut f64,
        var_nqs_xbar_slot: &mut f64,
        var_nqs_xbar_dn12_slot: &mut f64,
        var_nqs_xbar_dn13_slot: &mut f64,
        var_nqs_xbar_dn14_slot: &mut f64,
        var_nqs_xbar_dn15_slot: &mut f64,
        var_nqs_xbar_dn16_slot: &mut f64,
        var_nqs_xbar_dn17_slot: &mut f64,
        var_nqs_xbar_dn18_slot: &mut f64,
        var_nqs_xbar_dn19_slot: &mut f64,
        var_nqs_xbar_dn20_slot: &mut f64,
        var_nqs_xbar_dn5_slot: &mut f64,
        var_nqs_xbar_dn6_slot: &mut f64,
        var_nqs_xbar_dn7_slot: &mut f64,
        var_nqs_xbar_dn8_slot: &mut f64,
        var_nqs_xg1_slot: &mut f64,
        var_nqs_xg1_dn12_slot: &mut f64,
        var_nqs_xg1_dn13_slot: &mut f64,
        var_nqs_xg1_dn14_slot: &mut f64,
        var_nqs_xg1_dn15_slot: &mut f64,
        var_nqs_xg1_dn16_slot: &mut f64,
        var_nqs_xg1_dn17_slot: &mut f64,
        var_nqs_xg1_dn18_slot: &mut f64,
        var_nqs_xg1_dn19_slot: &mut f64,
        var_nqs_xg1_dn20_slot: &mut f64,
        var_nqs_xg1_dn5_slot: &mut f64,
        var_nqs_xg1_dn6_slot: &mut f64,
        var_nqs_xg1_dn7_slot: &mut f64,
        var_nqs_xg1_dn8_slot: &mut f64,
        var_nqs_xi_slot: &mut f64,
        var_nqs_xi_dn12_slot: &mut f64,
        var_nqs_xi_dn13_slot: &mut f64,
        var_nqs_xi_dn14_slot: &mut f64,
        var_nqs_xi_dn15_slot: &mut f64,
        var_nqs_xi_dn16_slot: &mut f64,
        var_nqs_xi_dn17_slot: &mut f64,
        var_nqs_xi_dn18_slot: &mut f64,
        var_nqs_xi_dn19_slot: &mut f64,
        var_nqs_xi_dn20_slot: &mut f64,
        var_nqs_xi_dn5_slot: &mut f64,
        var_nqs_xi_dn6_slot: &mut f64,
        var_nqs_xi_dn7_slot: &mut f64,
        var_nqs_xi_dn8_slot: &mut f64,
        var_temp9_slot: &mut f64,
        var_temp9_dn12_slot: &mut f64,
        var_temp9_dn13_slot: &mut f64,
        var_temp9_dn14_slot: &mut f64,
        var_temp9_dn15_slot: &mut f64,
        var_temp9_dn16_slot: &mut f64,
        var_temp9_dn17_slot: &mut f64,
        var_temp9_dn18_slot: &mut f64,
        var_temp9_dn19_slot: &mut f64,
        var_temp9_dn20_slot: &mut f64,
        var_temp9_dn5_slot: &mut f64,
        var_temp9_dn6_slot: &mut f64,
        var_temp9_dn7_slot: &mut f64,
        var_temp9_dn8_slot: &mut f64,
    ) {
        let mut var_guard2239: f64 = *var_guard2239_slot;
        let mut var_guard2240: f64 = *var_guard2240_slot;
        let mut var_guard2241: f64 = *var_guard2241_slot;
        let mut var_guard2242: f64 = *var_guard2242_slot;
        let mut var_nqs_a_fac: f64 = *var_nqs_a_fac_slot;
        let mut var_nqs_a_fac_dn12: f64 = *var_nqs_a_fac_dn12_slot;
        let mut var_nqs_a_fac_dn13: f64 = *var_nqs_a_fac_dn13_slot;
        let mut var_nqs_a_fac_dn14: f64 = *var_nqs_a_fac_dn14_slot;
        let mut var_nqs_a_fac_dn15: f64 = *var_nqs_a_fac_dn15_slot;
        let mut var_nqs_a_fac_dn16: f64 = *var_nqs_a_fac_dn16_slot;
        let mut var_nqs_a_fac_dn17: f64 = *var_nqs_a_fac_dn17_slot;
        let mut var_nqs_a_fac_dn18: f64 = *var_nqs_a_fac_dn18_slot;
        let mut var_nqs_a_fac_dn19: f64 = *var_nqs_a_fac_dn19_slot;
        let mut var_nqs_a_fac_dn20: f64 = *var_nqs_a_fac_dn20_slot;
        let mut var_nqs_a_fac_dn5: f64 = *var_nqs_a_fac_dn5_slot;
        let mut var_nqs_a_fac_dn6: f64 = *var_nqs_a_fac_dn6_slot;
        let mut var_nqs_a_fac_dn7: f64 = *var_nqs_a_fac_dn7_slot;
        let mut var_nqs_a_fac_dn8: f64 = *var_nqs_a_fac_dn8_slot;
        let mut var_nqs_d0: f64 = *var_nqs_d0_slot;
        let mut var_nqs_d0_dn12: f64 = *var_nqs_d0_dn12_slot;
        let mut var_nqs_d0_dn13: f64 = *var_nqs_d0_dn13_slot;
        let mut var_nqs_d0_dn14: f64 = *var_nqs_d0_dn14_slot;
        let mut var_nqs_d0_dn15: f64 = *var_nqs_d0_dn15_slot;
        let mut var_nqs_d0_dn16: f64 = *var_nqs_d0_dn16_slot;
        let mut var_nqs_d0_dn17: f64 = *var_nqs_d0_dn17_slot;
        let mut var_nqs_d0_dn18: f64 = *var_nqs_d0_dn18_slot;
        let mut var_nqs_d0_dn19: f64 = *var_nqs_d0_dn19_slot;
        let mut var_nqs_d0_dn20: f64 = *var_nqs_d0_dn20_slot;
        let mut var_nqs_d0_dn5: f64 = *var_nqs_d0_dn5_slot;
        let mut var_nqs_d0_dn6: f64 = *var_nqs_d0_dn6_slot;
        let mut var_nqs_d0_dn7: f64 = *var_nqs_d0_dn7_slot;
        let mut var_nqs_d0_dn8: f64 = *var_nqs_d0_dn8_slot;
        let mut var_nqs_p: f64 = *var_nqs_p_slot;
        let mut var_nqs_p_dn12: f64 = *var_nqs_p_dn12_slot;
        let mut var_nqs_p_dn13: f64 = *var_nqs_p_dn13_slot;
        let mut var_nqs_p_dn14: f64 = *var_nqs_p_dn14_slot;
        let mut var_nqs_p_dn15: f64 = *var_nqs_p_dn15_slot;
        let mut var_nqs_p_dn16: f64 = *var_nqs_p_dn16_slot;
        let mut var_nqs_p_dn17: f64 = *var_nqs_p_dn17_slot;
        let mut var_nqs_p_dn18: f64 = *var_nqs_p_dn18_slot;
        let mut var_nqs_p_dn19: f64 = *var_nqs_p_dn19_slot;
        let mut var_nqs_p_dn20: f64 = *var_nqs_p_dn20_slot;
        let mut var_nqs_p_dn5: f64 = *var_nqs_p_dn5_slot;
        let mut var_nqs_p_dn6: f64 = *var_nqs_p_dn6_slot;
        let mut var_nqs_p_dn7: f64 = *var_nqs_p_dn7_slot;
        let mut var_nqs_p_dn8: f64 = *var_nqs_p_dn8_slot;
        let mut var_nqs_q: f64 = *var_nqs_q_slot;
        let mut var_nqs_q_dn12: f64 = *var_nqs_q_dn12_slot;
        let mut var_nqs_q_dn13: f64 = *var_nqs_q_dn13_slot;
        let mut var_nqs_q_dn14: f64 = *var_nqs_q_dn14_slot;
        let mut var_nqs_q_dn15: f64 = *var_nqs_q_dn15_slot;
        let mut var_nqs_q_dn16: f64 = *var_nqs_q_dn16_slot;
        let mut var_nqs_q_dn17: f64 = *var_nqs_q_dn17_slot;
        let mut var_nqs_q_dn18: f64 = *var_nqs_q_dn18_slot;
        let mut var_nqs_q_dn19: f64 = *var_nqs_q_dn19_slot;
        let mut var_nqs_q_dn20: f64 = *var_nqs_q_dn20_slot;
        let mut var_nqs_q_dn5: f64 = *var_nqs_q_dn5_slot;
        let mut var_nqs_q_dn6: f64 = *var_nqs_q_dn6_slot;
        let mut var_nqs_q_dn7: f64 = *var_nqs_q_dn7_slot;
        let mut var_nqs_q_dn8: f64 = *var_nqs_q_dn8_slot;
        let mut var_nqs_temp: f64 = *var_nqs_temp_slot;
        let mut var_nqs_temp_dn12: f64 = *var_nqs_temp_dn12_slot;
        let mut var_nqs_temp_dn13: f64 = *var_nqs_temp_dn13_slot;
        let mut var_nqs_temp_dn14: f64 = *var_nqs_temp_dn14_slot;
        let mut var_nqs_temp_dn15: f64 = *var_nqs_temp_dn15_slot;
        let mut var_nqs_temp_dn16: f64 = *var_nqs_temp_dn16_slot;
        let mut var_nqs_temp_dn17: f64 = *var_nqs_temp_dn17_slot;
        let mut var_nqs_temp_dn18: f64 = *var_nqs_temp_dn18_slot;
        let mut var_nqs_temp_dn19: f64 = *var_nqs_temp_dn19_slot;
        let mut var_nqs_temp_dn20: f64 = *var_nqs_temp_dn20_slot;
        let mut var_nqs_temp_dn5: f64 = *var_nqs_temp_dn5_slot;
        let mut var_nqs_temp_dn6: f64 = *var_nqs_temp_dn6_slot;
        let mut var_nqs_temp_dn7: f64 = *var_nqs_temp_dn7_slot;
        let mut var_nqs_temp_dn8: f64 = *var_nqs_temp_dn8_slot;
        let mut var_nqs_w: f64 = *var_nqs_w_slot;
        let mut var_nqs_w_dn12: f64 = *var_nqs_w_dn12_slot;
        let mut var_nqs_w_dn13: f64 = *var_nqs_w_dn13_slot;
        let mut var_nqs_w_dn14: f64 = *var_nqs_w_dn14_slot;
        let mut var_nqs_w_dn15: f64 = *var_nqs_w_dn15_slot;
        let mut var_nqs_w_dn16: f64 = *var_nqs_w_dn16_slot;
        let mut var_nqs_w_dn17: f64 = *var_nqs_w_dn17_slot;
        let mut var_nqs_w_dn18: f64 = *var_nqs_w_dn18_slot;
        let mut var_nqs_w_dn19: f64 = *var_nqs_w_dn19_slot;
        let mut var_nqs_w_dn20: f64 = *var_nqs_w_dn20_slot;
        let mut var_nqs_w_dn5: f64 = *var_nqs_w_dn5_slot;
        let mut var_nqs_w_dn6: f64 = *var_nqs_w_dn6_slot;
        let mut var_nqs_w_dn7: f64 = *var_nqs_w_dn7_slot;
        let mut var_nqs_w_dn8: f64 = *var_nqs_w_dn8_slot;
        let mut var_nqs_x0: f64 = *var_nqs_x0_slot;
        let mut var_nqs_x0_dn12: f64 = *var_nqs_x0_dn12_slot;
        let mut var_nqs_x0_dn13: f64 = *var_nqs_x0_dn13_slot;
        let mut var_nqs_x0_dn14: f64 = *var_nqs_x0_dn14_slot;
        let mut var_nqs_x0_dn15: f64 = *var_nqs_x0_dn15_slot;
        let mut var_nqs_x0_dn16: f64 = *var_nqs_x0_dn16_slot;
        let mut var_nqs_x0_dn17: f64 = *var_nqs_x0_dn17_slot;
        let mut var_nqs_x0_dn18: f64 = *var_nqs_x0_dn18_slot;
        let mut var_nqs_x0_dn19: f64 = *var_nqs_x0_dn19_slot;
        let mut var_nqs_x0_dn20: f64 = *var_nqs_x0_dn20_slot;
        let mut var_nqs_x0_dn5: f64 = *var_nqs_x0_dn5_slot;
        let mut var_nqs_x0_dn6: f64 = *var_nqs_x0_dn6_slot;
        let mut var_nqs_x0_dn7: f64 = *var_nqs_x0_dn7_slot;
        let mut var_nqs_x0_dn8: f64 = *var_nqs_x0_dn8_slot;
        let mut var_nqs_xbar: f64 = *var_nqs_xbar_slot;
        let mut var_nqs_xbar_dn12: f64 = *var_nqs_xbar_dn12_slot;
        let mut var_nqs_xbar_dn13: f64 = *var_nqs_xbar_dn13_slot;
        let mut var_nqs_xbar_dn14: f64 = *var_nqs_xbar_dn14_slot;
        let mut var_nqs_xbar_dn15: f64 = *var_nqs_xbar_dn15_slot;
        let mut var_nqs_xbar_dn16: f64 = *var_nqs_xbar_dn16_slot;
        let mut var_nqs_xbar_dn17: f64 = *var_nqs_xbar_dn17_slot;
        let mut var_nqs_xbar_dn18: f64 = *var_nqs_xbar_dn18_slot;
        let mut var_nqs_xbar_dn19: f64 = *var_nqs_xbar_dn19_slot;
        let mut var_nqs_xbar_dn20: f64 = *var_nqs_xbar_dn20_slot;
        let mut var_nqs_xbar_dn5: f64 = *var_nqs_xbar_dn5_slot;
        let mut var_nqs_xbar_dn6: f64 = *var_nqs_xbar_dn6_slot;
        let mut var_nqs_xbar_dn7: f64 = *var_nqs_xbar_dn7_slot;
        let mut var_nqs_xbar_dn8: f64 = *var_nqs_xbar_dn8_slot;
        let mut var_nqs_xg1: f64 = *var_nqs_xg1_slot;
        let mut var_nqs_xg1_dn12: f64 = *var_nqs_xg1_dn12_slot;
        let mut var_nqs_xg1_dn13: f64 = *var_nqs_xg1_dn13_slot;
        let mut var_nqs_xg1_dn14: f64 = *var_nqs_xg1_dn14_slot;
        let mut var_nqs_xg1_dn15: f64 = *var_nqs_xg1_dn15_slot;
        let mut var_nqs_xg1_dn16: f64 = *var_nqs_xg1_dn16_slot;
        let mut var_nqs_xg1_dn17: f64 = *var_nqs_xg1_dn17_slot;
        let mut var_nqs_xg1_dn18: f64 = *var_nqs_xg1_dn18_slot;
        let mut var_nqs_xg1_dn19: f64 = *var_nqs_xg1_dn19_slot;
        let mut var_nqs_xg1_dn20: f64 = *var_nqs_xg1_dn20_slot;
        let mut var_nqs_xg1_dn5: f64 = *var_nqs_xg1_dn5_slot;
        let mut var_nqs_xg1_dn6: f64 = *var_nqs_xg1_dn6_slot;
        let mut var_nqs_xg1_dn7: f64 = *var_nqs_xg1_dn7_slot;
        let mut var_nqs_xg1_dn8: f64 = *var_nqs_xg1_dn8_slot;
        let mut var_nqs_xi: f64 = *var_nqs_xi_slot;
        let mut var_nqs_xi_dn12: f64 = *var_nqs_xi_dn12_slot;
        let mut var_nqs_xi_dn13: f64 = *var_nqs_xi_dn13_slot;
        let mut var_nqs_xi_dn14: f64 = *var_nqs_xi_dn14_slot;
        let mut var_nqs_xi_dn15: f64 = *var_nqs_xi_dn15_slot;
        let mut var_nqs_xi_dn16: f64 = *var_nqs_xi_dn16_slot;
        let mut var_nqs_xi_dn17: f64 = *var_nqs_xi_dn17_slot;
        let mut var_nqs_xi_dn18: f64 = *var_nqs_xi_dn18_slot;
        let mut var_nqs_xi_dn19: f64 = *var_nqs_xi_dn19_slot;
        let mut var_nqs_xi_dn20: f64 = *var_nqs_xi_dn20_slot;
        let mut var_nqs_xi_dn5: f64 = *var_nqs_xi_dn5_slot;
        let mut var_nqs_xi_dn6: f64 = *var_nqs_xi_dn6_slot;
        let mut var_nqs_xi_dn7: f64 = *var_nqs_xi_dn7_slot;
        let mut var_nqs_xi_dn8: f64 = *var_nqs_xi_dn8_slot;
        let mut var_temp9: f64 = *var_temp9_slot;
        let mut var_temp9_dn12: f64 = *var_temp9_dn12_slot;
        let mut var_temp9_dn13: f64 = *var_temp9_dn13_slot;
        let mut var_temp9_dn14: f64 = *var_temp9_dn14_slot;
        let mut var_temp9_dn15: f64 = *var_temp9_dn15_slot;
        let mut var_temp9_dn16: f64 = *var_temp9_dn16_slot;
        let mut var_temp9_dn17: f64 = *var_temp9_dn17_slot;
        let mut var_temp9_dn18: f64 = *var_temp9_dn18_slot;
        let mut var_temp9_dn19: f64 = *var_temp9_dn19_slot;
        let mut var_temp9_dn20: f64 = *var_temp9_dn20_slot;
        let mut var_temp9_dn5: f64 = *var_temp9_dn5_slot;
        let mut var_temp9_dn6: f64 = *var_temp9_dn6_slot;
        let mut var_temp9_dn7: f64 = *var_temp9_dn7_slot;
        let mut var_temp9_dn8: f64 = *var_temp9_dn8_slot;

        let (assign82310_e123403, assign82310_e123403_d_n5, assign82310_e123403_d_n6, assign82310_e123403_d_n7, assign82310_e123403_d_n8, assign82310_e123403_d_n12, assign82310_e123403_d_n13, assign82310_e123403_d_n14, assign82310_e123403_d_n15, assign82310_e123403_d_n16, assign82310_e123403_d_n17, assign82310_e123403_d_n18, assign82310_e123403_d_n19, assign82310_e123403_d_n20,) = {
    if (((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) && (var_guard2238 != 0.0)) {
        let assign82310_e123401: f64 = (var_nqs_y0).exp();
        (assign82310_e123401, (assign82310_e123401 * var_nqs_y0_dn5), (assign82310_e123401 * var_nqs_y0_dn6), (assign82310_e123401 * var_nqs_y0_dn7), (assign82310_e123401 * var_nqs_y0_dn8), (assign82310_e123401 * var_nqs_y0_dn12), (assign82310_e123401 * var_nqs_y0_dn13), (assign82310_e123401 * var_nqs_y0_dn14), (assign82310_e123401 * var_nqs_y0_dn15), (assign82310_e123401 * var_nqs_y0_dn16), (assign82310_e123401 * var_nqs_y0_dn17), (assign82310_e123401 * var_nqs_y0_dn18), (assign82310_e123401 * var_nqs_y0_dn19), (assign82310_e123401 * var_nqs_y0_dn20),)
    } else {
        (var_nqs_d0, var_nqs_d0_dn5, var_nqs_d0_dn6, var_nqs_d0_dn7, var_nqs_d0_dn8, var_nqs_d0_dn12, var_nqs_d0_dn13, var_nqs_d0_dn14, var_nqs_d0_dn15, var_nqs_d0_dn16, var_nqs_d0_dn17, var_nqs_d0_dn18, var_nqs_d0_dn19, var_nqs_d0_dn20,)
    }
};
        var_nqs_d0 = assign82310_e123403;
        var_nqs_d0_dn5 = assign82310_e123403_d_n5;
        var_nqs_d0_dn6 = assign82310_e123403_d_n6;
        var_nqs_d0_dn7 = assign82310_e123403_d_n7;
        var_nqs_d0_dn8 = assign82310_e123403_d_n8;
        var_nqs_d0_dn12 = assign82310_e123403_d_n12;
        var_nqs_d0_dn13 = assign82310_e123403_d_n13;
        var_nqs_d0_dn14 = assign82310_e123403_d_n14;
        var_nqs_d0_dn15 = assign82310_e123403_d_n15;
        var_nqs_d0_dn16 = assign82310_e123403_d_n16;
        var_nqs_d0_dn17 = assign82310_e123403_d_n17;
        var_nqs_d0_dn18 = assign82310_e123403_d_n18;
        var_nqs_d0_dn19 = assign82310_e123403_d_n19;
        var_nqs_d0_dn20 = assign82310_e123403_d_n20;

        let assign82320_e123406: f64 = if var_nqs_y0 < 0.0 { 1.0 } else { 0.0 };
        var_guard2239 = assign82320_e123406;

        let (assign82330_e123459, assign82330_e123459_d_n5, assign82330_e123459_d_n6, assign82330_e123459_d_n7, assign82330_e123459_d_n8, assign82330_e123459_d_n12, assign82330_e123459_d_n13, assign82330_e123459_d_n14, assign82330_e123459_d_n15, assign82330_e123459_d_n16, assign82330_e123459_d_n17, assign82330_e123459_d_n18, assign82330_e123459_d_n19, assign82330_e123459_d_n20,) = {
    if ((((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) && (var_guard2238 == 0.0)) && (var_guard2239 != 0.0)) {
        let assign82330_e123435: f64 = (-230.25850929940458);
        let assign82330_e123437: f64 = (assign82330_e123435 - var_nqs_y0);
        let assign82330_e123441: f64 = (-230.25850929940458);
        let assign82330_e123443: f64 = (assign82330_e123441 - var_nqs_y0);
        let assign82330_e123446: f64 = (-230.25850929940458);
        let assign82330_e123448: f64 = (assign82330_e123446 - var_nqs_y0);
        let assign82330_e123450: f64 = (assign82330_e123448 * 0.3333333333333333);
        let assign82330_e123451: f64 = (1.0 + assign82330_e123450);
        let assign82330_e123452: f64 = (assign82330_e123443 * assign82330_e123451);
        let assign82330_e123453: f64 = (0.5 * assign82330_e123452);
        let assign82330_e123454: f64 = (1.0 + assign82330_e123453);
        let assign82330_e123455: f64 = (assign82330_e123437 * assign82330_e123454);
        let assign82330_e123456: f64 = (1.0 + assign82330_e123455);
        let assign82330_e123457: f64 = (1e-100 / assign82330_e123456);
        (assign82330_e123457, (-((1e-100 * (((-var_nqs_y0_dn5) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-var_nqs_y0_dn5) * assign82330_e123451) + (assign82330_e123443 * ((-var_nqs_y0_dn5) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-var_nqs_y0_dn6) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-var_nqs_y0_dn6) * assign82330_e123451) + (assign82330_e123443 * ((-var_nqs_y0_dn6) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-var_nqs_y0_dn7) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-var_nqs_y0_dn7) * assign82330_e123451) + (assign82330_e123443 * ((-var_nqs_y0_dn7) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-var_nqs_y0_dn8) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-var_nqs_y0_dn8) * assign82330_e123451) + (assign82330_e123443 * ((-var_nqs_y0_dn8) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-var_nqs_y0_dn12) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-var_nqs_y0_dn12) * assign82330_e123451) + (assign82330_e123443 * ((-var_nqs_y0_dn12) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-var_nqs_y0_dn13) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-var_nqs_y0_dn13) * assign82330_e123451) + (assign82330_e123443 * ((-var_nqs_y0_dn13) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-var_nqs_y0_dn14) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-var_nqs_y0_dn14) * assign82330_e123451) + (assign82330_e123443 * ((-var_nqs_y0_dn14) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-var_nqs_y0_dn15) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-var_nqs_y0_dn15) * assign82330_e123451) + (assign82330_e123443 * ((-var_nqs_y0_dn15) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-var_nqs_y0_dn16) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-var_nqs_y0_dn16) * assign82330_e123451) + (assign82330_e123443 * ((-var_nqs_y0_dn16) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-var_nqs_y0_dn17) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-var_nqs_y0_dn17) * assign82330_e123451) + (assign82330_e123443 * ((-var_nqs_y0_dn17) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-var_nqs_y0_dn18) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-var_nqs_y0_dn18) * assign82330_e123451) + (assign82330_e123443 * ((-var_nqs_y0_dn18) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-var_nqs_y0_dn19) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-var_nqs_y0_dn19) * assign82330_e123451) + (assign82330_e123443 * ((-var_nqs_y0_dn19) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))), (-((1e-100 * (((-var_nqs_y0_dn20) * assign82330_e123454) + (assign82330_e123437 * (0.5 * (((-var_nqs_y0_dn20) * assign82330_e123451) + (assign82330_e123443 * ((-var_nqs_y0_dn20) * 0.3333333333333333))))))) / (assign82330_e123456 * assign82330_e123456))),)
    } else {
        (var_nqs_d0, var_nqs_d0_dn5, var_nqs_d0_dn6, var_nqs_d0_dn7, var_nqs_d0_dn8, var_nqs_d0_dn12, var_nqs_d0_dn13, var_nqs_d0_dn14, var_nqs_d0_dn15, var_nqs_d0_dn16, var_nqs_d0_dn17, var_nqs_d0_dn18, var_nqs_d0_dn19, var_nqs_d0_dn20,)
    }
};
        var_nqs_d0 = assign82330_e123459;
        var_nqs_d0_dn5 = assign82330_e123459_d_n5;
        var_nqs_d0_dn6 = assign82330_e123459_d_n6;
        var_nqs_d0_dn7 = assign82330_e123459_d_n7;
        var_nqs_d0_dn8 = assign82330_e123459_d_n8;
        var_nqs_d0_dn12 = assign82330_e123459_d_n12;
        var_nqs_d0_dn13 = assign82330_e123459_d_n13;
        var_nqs_d0_dn14 = assign82330_e123459_d_n14;
        var_nqs_d0_dn15 = assign82330_e123459_d_n15;
        var_nqs_d0_dn16 = assign82330_e123459_d_n16;
        var_nqs_d0_dn17 = assign82330_e123459_d_n17;
        var_nqs_d0_dn18 = assign82330_e123459_d_n18;
        var_nqs_d0_dn19 = assign82330_e123459_d_n19;
        var_nqs_d0_dn20 = assign82330_e123459_d_n20;

        let (assign82340_e123510, assign82340_e123510_d_n5, assign82340_e123510_d_n6, assign82340_e123510_d_n7, assign82340_e123510_d_n8, assign82340_e123510_d_n12, assign82340_e123510_d_n13, assign82340_e123510_d_n14, assign82340_e123510_d_n15, assign82340_e123510_d_n16, assign82340_e123510_d_n17, assign82340_e123510_d_n18, assign82340_e123510_d_n19, assign82340_e123510_d_n20,) = {
    if ((((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) && (var_guard2238 == 0.0)) && (var_guard2239 == 0.0)) {
        let assign82340_e123490: f64 = (var_nqs_y0 - 230.25850929940458);
        let assign82340_e123495: f64 = (var_nqs_y0 - 230.25850929940458);
        let assign82340_e123499: f64 = (var_nqs_y0 - 230.25850929940458);
        let assign82340_e123501: f64 = (assign82340_e123499 * 0.3333333333333333);
        let assign82340_e123502: f64 = (1.0 + assign82340_e123501);
        let assign82340_e123503: f64 = (assign82340_e123495 * assign82340_e123502);
        let assign82340_e123504: f64 = (0.5 * assign82340_e123503);
        let assign82340_e123505: f64 = (1.0 + assign82340_e123504);
        let assign82340_e123506: f64 = (assign82340_e123490 * assign82340_e123505);
        let assign82340_e123507: f64 = (1.0 + assign82340_e123506);
        let assign82340_e123508: f64 = (1e100 * assign82340_e123507);
        (assign82340_e123508, (1e100 * ((var_nqs_y0_dn5 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((var_nqs_y0_dn5 * assign82340_e123502) + (assign82340_e123495 * (var_nqs_y0_dn5 * 0.3333333333333333))))))), (1e100 * ((var_nqs_y0_dn6 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((var_nqs_y0_dn6 * assign82340_e123502) + (assign82340_e123495 * (var_nqs_y0_dn6 * 0.3333333333333333))))))), (1e100 * ((var_nqs_y0_dn7 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((var_nqs_y0_dn7 * assign82340_e123502) + (assign82340_e123495 * (var_nqs_y0_dn7 * 0.3333333333333333))))))), (1e100 * ((var_nqs_y0_dn8 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((var_nqs_y0_dn8 * assign82340_e123502) + (assign82340_e123495 * (var_nqs_y0_dn8 * 0.3333333333333333))))))), (1e100 * ((var_nqs_y0_dn12 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((var_nqs_y0_dn12 * assign82340_e123502) + (assign82340_e123495 * (var_nqs_y0_dn12 * 0.3333333333333333))))))), (1e100 * ((var_nqs_y0_dn13 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((var_nqs_y0_dn13 * assign82340_e123502) + (assign82340_e123495 * (var_nqs_y0_dn13 * 0.3333333333333333))))))), (1e100 * ((var_nqs_y0_dn14 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((var_nqs_y0_dn14 * assign82340_e123502) + (assign82340_e123495 * (var_nqs_y0_dn14 * 0.3333333333333333))))))), (1e100 * ((var_nqs_y0_dn15 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((var_nqs_y0_dn15 * assign82340_e123502) + (assign82340_e123495 * (var_nqs_y0_dn15 * 0.3333333333333333))))))), (1e100 * ((var_nqs_y0_dn16 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((var_nqs_y0_dn16 * assign82340_e123502) + (assign82340_e123495 * (var_nqs_y0_dn16 * 0.3333333333333333))))))), (1e100 * ((var_nqs_y0_dn17 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((var_nqs_y0_dn17 * assign82340_e123502) + (assign82340_e123495 * (var_nqs_y0_dn17 * 0.3333333333333333))))))), (1e100 * ((var_nqs_y0_dn18 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((var_nqs_y0_dn18 * assign82340_e123502) + (assign82340_e123495 * (var_nqs_y0_dn18 * 0.3333333333333333))))))), (1e100 * ((var_nqs_y0_dn19 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((var_nqs_y0_dn19 * assign82340_e123502) + (assign82340_e123495 * (var_nqs_y0_dn19 * 0.3333333333333333))))))), (1e100 * ((var_nqs_y0_dn20 * assign82340_e123505) + (assign82340_e123490 * (0.5 * ((var_nqs_y0_dn20 * assign82340_e123502) + (assign82340_e123495 * (var_nqs_y0_dn20 * 0.3333333333333333))))))),)
    } else {
        (var_nqs_d0, var_nqs_d0_dn5, var_nqs_d0_dn6, var_nqs_d0_dn7, var_nqs_d0_dn8, var_nqs_d0_dn12, var_nqs_d0_dn13, var_nqs_d0_dn14, var_nqs_d0_dn15, var_nqs_d0_dn16, var_nqs_d0_dn17, var_nqs_d0_dn18, var_nqs_d0_dn19, var_nqs_d0_dn20,)
    }
};
        var_nqs_d0 = assign82340_e123510;
        var_nqs_d0_dn5 = assign82340_e123510_d_n5;
        var_nqs_d0_dn6 = assign82340_e123510_d_n6;
        var_nqs_d0_dn7 = assign82340_e123510_d_n7;
        var_nqs_d0_dn8 = assign82340_e123510_d_n8;
        var_nqs_d0_dn12 = assign82340_e123510_d_n12;
        var_nqs_d0_dn13 = assign82340_e123510_d_n13;
        var_nqs_d0_dn14 = assign82340_e123510_d_n14;
        var_nqs_d0_dn15 = assign82340_e123510_d_n15;
        var_nqs_d0_dn16 = assign82340_e123510_d_n16;
        var_nqs_d0_dn17 = assign82340_e123510_d_n17;
        var_nqs_d0_dn18 = assign82340_e123510_d_n18;
        var_nqs_d0_dn19 = assign82340_e123510_d_n19;
        var_nqs_d0_dn20 = assign82340_e123510_d_n20;

        let (assign82350_e123539, assign82350_e123539_d_n5, assign82350_e123539_d_n6, assign82350_e123539_d_n7, assign82350_e123539_d_n8, assign82350_e123539_d_n12, assign82350_e123539_d_n13, assign82350_e123539_d_n14, assign82350_e123539_d_n15, assign82350_e123539_d_n16, assign82350_e123539_d_n17, assign82350_e123539_d_n18, assign82350_e123539_d_n19, assign82350_e123539_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) {
        let assign82350_e123534: f64 = (var_gp2 * var_nqs_d0);
        let assign82350_e123536: f64 = (assign82350_e123534 * 0.5);
        let assign82350_e123537: f64 = (1.0 - assign82350_e123536);
        (assign82350_e123537, (-(((var_gp2_dn5 * var_nqs_d0) + (var_gp2 * var_nqs_d0_dn5)) * 0.5)), (-(((var_gp2_dn6 * var_nqs_d0) + (var_gp2 * var_nqs_d0_dn6)) * 0.5)), (-(((var_gp2_dn7 * var_nqs_d0) + (var_gp2 * var_nqs_d0_dn7)) * 0.5)), (-(((var_gp2_dn8 * var_nqs_d0) + (var_gp2 * var_nqs_d0_dn8)) * 0.5)), (-(((var_gp2_dn12 * var_nqs_d0) + (var_gp2 * var_nqs_d0_dn12)) * 0.5)), (-(((var_gp2_dn13 * var_nqs_d0) + (var_gp2 * var_nqs_d0_dn13)) * 0.5)), (-(((var_gp2_dn14 * var_nqs_d0) + (var_gp2 * var_nqs_d0_dn14)) * 0.5)), (-(((var_gp2_dn15 * var_nqs_d0) + (var_gp2 * var_nqs_d0_dn15)) * 0.5)), (-(((var_gp2_dn16 * var_nqs_d0) + (var_gp2 * var_nqs_d0_dn16)) * 0.5)), (-(((var_gp2_dn17 * var_nqs_d0) + (var_gp2 * var_nqs_d0_dn17)) * 0.5)), (-(((var_gp2_dn18 * var_nqs_d0) + (var_gp2 * var_nqs_d0_dn18)) * 0.5)), (-(((var_gp2_dn19 * var_nqs_d0) + (var_gp2 * var_nqs_d0_dn19)) * 0.5)), (-(((var_gp2_dn20 * var_nqs_d0) + (var_gp2 * var_nqs_d0_dn20)) * 0.5)),)
    } else {
        (var_nqs_xi, var_nqs_xi_dn5, var_nqs_xi_dn6, var_nqs_xi_dn7, var_nqs_xi_dn8, var_nqs_xi_dn12, var_nqs_xi_dn13, var_nqs_xi_dn14, var_nqs_xi_dn15, var_nqs_xi_dn16, var_nqs_xi_dn17, var_nqs_xi_dn18, var_nqs_xi_dn19, var_nqs_xi_dn20,)
    }
};
        var_nqs_xi = assign82350_e123539;
        var_nqs_xi_dn5 = assign82350_e123539_d_n5;
        var_nqs_xi_dn6 = assign82350_e123539_d_n6;
        var_nqs_xi_dn7 = assign82350_e123539_d_n7;
        var_nqs_xi_dn8 = assign82350_e123539_d_n8;
        var_nqs_xi_dn12 = assign82350_e123539_d_n12;
        var_nqs_xi_dn13 = assign82350_e123539_d_n13;
        var_nqs_xi_dn14 = assign82350_e123539_d_n14;
        var_nqs_xi_dn15 = assign82350_e123539_d_n15;
        var_nqs_xi_dn16 = assign82350_e123539_d_n16;
        var_nqs_xi_dn17 = assign82350_e123539_d_n17;
        var_nqs_xi_dn18 = assign82350_e123539_d_n18;
        var_nqs_xi_dn19 = assign82350_e123539_d_n19;
        var_nqs_xi_dn20 = assign82350_e123539_d_n20;

        let (assign82360_e123572, assign82360_e123572_d_n5, assign82360_e123572_d_n6, assign82360_e123572_d_n7, assign82360_e123572_d_n8, assign82360_e123572_d_n12, assign82360_e123572_d_n13, assign82360_e123572_d_n14, assign82360_e123572_d_n15, assign82360_e123572_d_n16, assign82360_e123572_d_n17, assign82360_e123572_d_n18, assign82360_e123572_d_n19, assign82360_e123572_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) {
        let assign82360_e123563: f64 = (var_nqs_yg - var_nqs_y0);
        let assign82360_e123564: f64 = (2.0 * assign82360_e123563);
        let assign82360_e123568: f64 = (var_nqs_d0 - 1.0);
        let assign82360_e123569: f64 = (var_gp2 * assign82360_e123568);
        let assign82360_e123570: f64 = (assign82360_e123564 + assign82360_e123569);
        (assign82360_e123570, ((2.0 * (var_nqs_yg_dn5 - var_nqs_y0_dn5)) + ((var_gp2_dn5 * assign82360_e123568) + (var_gp2 * var_nqs_d0_dn5))), ((2.0 * (var_nqs_yg_dn6 - var_nqs_y0_dn6)) + ((var_gp2_dn6 * assign82360_e123568) + (var_gp2 * var_nqs_d0_dn6))), ((2.0 * (var_nqs_yg_dn7 - var_nqs_y0_dn7)) + ((var_gp2_dn7 * assign82360_e123568) + (var_gp2 * var_nqs_d0_dn7))), ((2.0 * (var_nqs_yg_dn8 - var_nqs_y0_dn8)) + ((var_gp2_dn8 * assign82360_e123568) + (var_gp2 * var_nqs_d0_dn8))), ((2.0 * (var_nqs_yg_dn12 - var_nqs_y0_dn12)) + ((var_gp2_dn12 * assign82360_e123568) + (var_gp2 * var_nqs_d0_dn12))), ((2.0 * (var_nqs_yg_dn13 - var_nqs_y0_dn13)) + ((var_gp2_dn13 * assign82360_e123568) + (var_gp2 * var_nqs_d0_dn13))), ((2.0 * (var_nqs_yg_dn14 - var_nqs_y0_dn14)) + ((var_gp2_dn14 * assign82360_e123568) + (var_gp2 * var_nqs_d0_dn14))), ((2.0 * (var_nqs_yg_dn15 - var_nqs_y0_dn15)) + ((var_gp2_dn15 * assign82360_e123568) + (var_gp2 * var_nqs_d0_dn15))), ((2.0 * (var_nqs_yg_dn16 - var_nqs_y0_dn16)) + ((var_gp2_dn16 * assign82360_e123568) + (var_gp2 * var_nqs_d0_dn16))), ((2.0 * (var_nqs_yg_dn17 - var_nqs_y0_dn17)) + ((var_gp2_dn17 * assign82360_e123568) + (var_gp2 * var_nqs_d0_dn17))), ((2.0 * (var_nqs_yg_dn18 - var_nqs_y0_dn18)) + ((var_gp2_dn18 * assign82360_e123568) + (var_gp2 * var_nqs_d0_dn18))), ((2.0 * (var_nqs_yg_dn19 - var_nqs_y0_dn19)) + ((var_gp2_dn19 * assign82360_e123568) + (var_gp2 * var_nqs_d0_dn19))), ((2.0 * (var_nqs_yg_dn20 - var_nqs_y0_dn20)) + ((var_gp2_dn20 * assign82360_e123568) + (var_gp2 * var_nqs_d0_dn20))),)
    } else {
        (var_nqs_p, var_nqs_p_dn5, var_nqs_p_dn6, var_nqs_p_dn7, var_nqs_p_dn8, var_nqs_p_dn12, var_nqs_p_dn13, var_nqs_p_dn14, var_nqs_p_dn15, var_nqs_p_dn16, var_nqs_p_dn17, var_nqs_p_dn18, var_nqs_p_dn19, var_nqs_p_dn20,)
    }
};
        var_nqs_p = assign82360_e123572;
        var_nqs_p_dn5 = assign82360_e123572_d_n5;
        var_nqs_p_dn6 = assign82360_e123572_d_n6;
        var_nqs_p_dn7 = assign82360_e123572_d_n7;
        var_nqs_p_dn8 = assign82360_e123572_d_n8;
        var_nqs_p_dn12 = assign82360_e123572_d_n12;
        var_nqs_p_dn13 = assign82360_e123572_d_n13;
        var_nqs_p_dn14 = assign82360_e123572_d_n14;
        var_nqs_p_dn15 = assign82360_e123572_d_n15;
        var_nqs_p_dn16 = assign82360_e123572_d_n16;
        var_nqs_p_dn17 = assign82360_e123572_d_n17;
        var_nqs_p_dn18 = assign82360_e123572_d_n18;
        var_nqs_p_dn19 = assign82360_e123572_d_n19;
        var_nqs_p_dn20 = assign82360_e123572_d_n20;

        let (assign82370_e123609, assign82370_e123609_d_n5, assign82370_e123609_d_n6, assign82370_e123609_d_n7, assign82370_e123609_d_n8, assign82370_e123609_d_n12, assign82370_e123609_d_n13, assign82370_e123609_d_n14, assign82370_e123609_d_n15, assign82370_e123609_d_n16, assign82370_e123609_d_n17, assign82370_e123609_d_n18, assign82370_e123609_d_n19, assign82370_e123609_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) {
        let assign82370_e123595: f64 = (var_nqs_yg - var_nqs_y0);
        let assign82370_e123598: f64 = (var_nqs_yg - var_nqs_y0);
        let assign82370_e123599: f64 = (assign82370_e123595 * assign82370_e123598);
        let assign82370_e123603: f64 = (var_nqs_y0 + 1.0);
        let assign82370_e123605: f64 = (assign82370_e123603 - var_nqs_d0);
        let assign82370_e123606: f64 = (var_gp2 * assign82370_e123605);
        let assign82370_e123607: f64 = (assign82370_e123599 + assign82370_e123606);
        (assign82370_e123607, ((((var_nqs_yg_dn5 - var_nqs_y0_dn5) * assign82370_e123598) + (assign82370_e123595 * (var_nqs_yg_dn5 - var_nqs_y0_dn5))) + ((var_gp2_dn5 * assign82370_e123605) + (var_gp2 * (var_nqs_y0_dn5 - var_nqs_d0_dn5)))), ((((var_nqs_yg_dn6 - var_nqs_y0_dn6) * assign82370_e123598) + (assign82370_e123595 * (var_nqs_yg_dn6 - var_nqs_y0_dn6))) + ((var_gp2_dn6 * assign82370_e123605) + (var_gp2 * (var_nqs_y0_dn6 - var_nqs_d0_dn6)))), ((((var_nqs_yg_dn7 - var_nqs_y0_dn7) * assign82370_e123598) + (assign82370_e123595 * (var_nqs_yg_dn7 - var_nqs_y0_dn7))) + ((var_gp2_dn7 * assign82370_e123605) + (var_gp2 * (var_nqs_y0_dn7 - var_nqs_d0_dn7)))), ((((var_nqs_yg_dn8 - var_nqs_y0_dn8) * assign82370_e123598) + (assign82370_e123595 * (var_nqs_yg_dn8 - var_nqs_y0_dn8))) + ((var_gp2_dn8 * assign82370_e123605) + (var_gp2 * (var_nqs_y0_dn8 - var_nqs_d0_dn8)))), ((((var_nqs_yg_dn12 - var_nqs_y0_dn12) * assign82370_e123598) + (assign82370_e123595 * (var_nqs_yg_dn12 - var_nqs_y0_dn12))) + ((var_gp2_dn12 * assign82370_e123605) + (var_gp2 * (var_nqs_y0_dn12 - var_nqs_d0_dn12)))), ((((var_nqs_yg_dn13 - var_nqs_y0_dn13) * assign82370_e123598) + (assign82370_e123595 * (var_nqs_yg_dn13 - var_nqs_y0_dn13))) + ((var_gp2_dn13 * assign82370_e123605) + (var_gp2 * (var_nqs_y0_dn13 - var_nqs_d0_dn13)))), ((((var_nqs_yg_dn14 - var_nqs_y0_dn14) * assign82370_e123598) + (assign82370_e123595 * (var_nqs_yg_dn14 - var_nqs_y0_dn14))) + ((var_gp2_dn14 * assign82370_e123605) + (var_gp2 * (var_nqs_y0_dn14 - var_nqs_d0_dn14)))), ((((var_nqs_yg_dn15 - var_nqs_y0_dn15) * assign82370_e123598) + (assign82370_e123595 * (var_nqs_yg_dn15 - var_nqs_y0_dn15))) + ((var_gp2_dn15 * assign82370_e123605) + (var_gp2 * (var_nqs_y0_dn15 - var_nqs_d0_dn15)))), ((((var_nqs_yg_dn16 - var_nqs_y0_dn16) * assign82370_e123598) + (assign82370_e123595 * (var_nqs_yg_dn16 - var_nqs_y0_dn16))) + ((var_gp2_dn16 * assign82370_e123605) + (var_gp2 * (var_nqs_y0_dn16 - var_nqs_d0_dn16)))), ((((var_nqs_yg_dn17 - var_nqs_y0_dn17) * assign82370_e123598) + (assign82370_e123595 * (var_nqs_yg_dn17 - var_nqs_y0_dn17))) + ((var_gp2_dn17 * assign82370_e123605) + (var_gp2 * (var_nqs_y0_dn17 - var_nqs_d0_dn17)))), ((((var_nqs_yg_dn18 - var_nqs_y0_dn18) * assign82370_e123598) + (assign82370_e123595 * (var_nqs_yg_dn18 - var_nqs_y0_dn18))) + ((var_gp2_dn18 * assign82370_e123605) + (var_gp2 * (var_nqs_y0_dn18 - var_nqs_d0_dn18)))), ((((var_nqs_yg_dn19 - var_nqs_y0_dn19) * assign82370_e123598) + (assign82370_e123595 * (var_nqs_yg_dn19 - var_nqs_y0_dn19))) + ((var_gp2_dn19 * assign82370_e123605) + (var_gp2 * (var_nqs_y0_dn19 - var_nqs_d0_dn19)))), ((((var_nqs_yg_dn20 - var_nqs_y0_dn20) * assign82370_e123598) + (assign82370_e123595 * (var_nqs_yg_dn20 - var_nqs_y0_dn20))) + ((var_gp2_dn20 * assign82370_e123605) + (var_gp2 * (var_nqs_y0_dn20 - var_nqs_d0_dn20)))),)
    } else {
        (var_nqs_q, var_nqs_q_dn5, var_nqs_q_dn6, var_nqs_q_dn7, var_nqs_q_dn8, var_nqs_q_dn12, var_nqs_q_dn13, var_nqs_q_dn14, var_nqs_q_dn15, var_nqs_q_dn16, var_nqs_q_dn17, var_nqs_q_dn18, var_nqs_q_dn19, var_nqs_q_dn20,)
    }
};
        var_nqs_q = assign82370_e123609;
        var_nqs_q_dn5 = assign82370_e123609_d_n5;
        var_nqs_q_dn6 = assign82370_e123609_d_n6;
        var_nqs_q_dn7 = assign82370_e123609_d_n7;
        var_nqs_q_dn8 = assign82370_e123609_d_n8;
        var_nqs_q_dn12 = assign82370_e123609_d_n12;
        var_nqs_q_dn13 = assign82370_e123609_d_n13;
        var_nqs_q_dn14 = assign82370_e123609_d_n14;
        var_nqs_q_dn15 = assign82370_e123609_d_n15;
        var_nqs_q_dn16 = assign82370_e123609_d_n16;
        var_nqs_q_dn17 = assign82370_e123609_d_n17;
        var_nqs_q_dn18 = assign82370_e123609_d_n18;
        var_nqs_q_dn19 = assign82370_e123609_d_n19;
        var_nqs_q_dn20 = assign82370_e123609_d_n20;

        let (assign82380_e123640, assign82380_e123640_d_n5, assign82380_e123640_d_n6, assign82380_e123640_d_n7, assign82380_e123640_d_n8, assign82380_e123640_d_n12, assign82380_e123640_d_n13, assign82380_e123640_d_n14, assign82380_e123640_d_n15, assign82380_e123640_d_n16, assign82380_e123640_d_n17, assign82380_e123640_d_n18, assign82380_e123640_d_n19, assign82380_e123640_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) {
        let assign82380_e123632: f64 = (var_nqs_p * var_nqs_p);
        let assign82380_e123635: f64 = (4.0 * var_nqs_xi);
        let assign82380_e123637: f64 = (assign82380_e123635 * var_nqs_q);
        let assign82380_e123638: f64 = (assign82380_e123632 - assign82380_e123637);
        (assign82380_e123638, (((var_nqs_p_dn5 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn5)) - (((4.0 * var_nqs_xi_dn5) * var_nqs_q) + (assign82380_e123635 * var_nqs_q_dn5))), (((var_nqs_p_dn6 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn6)) - (((4.0 * var_nqs_xi_dn6) * var_nqs_q) + (assign82380_e123635 * var_nqs_q_dn6))), (((var_nqs_p_dn7 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn7)) - (((4.0 * var_nqs_xi_dn7) * var_nqs_q) + (assign82380_e123635 * var_nqs_q_dn7))), (((var_nqs_p_dn8 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn8)) - (((4.0 * var_nqs_xi_dn8) * var_nqs_q) + (assign82380_e123635 * var_nqs_q_dn8))), (((var_nqs_p_dn12 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn12)) - (((4.0 * var_nqs_xi_dn12) * var_nqs_q) + (assign82380_e123635 * var_nqs_q_dn12))), (((var_nqs_p_dn13 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn13)) - (((4.0 * var_nqs_xi_dn13) * var_nqs_q) + (assign82380_e123635 * var_nqs_q_dn13))), (((var_nqs_p_dn14 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn14)) - (((4.0 * var_nqs_xi_dn14) * var_nqs_q) + (assign82380_e123635 * var_nqs_q_dn14))), (((var_nqs_p_dn15 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn15)) - (((4.0 * var_nqs_xi_dn15) * var_nqs_q) + (assign82380_e123635 * var_nqs_q_dn15))), (((var_nqs_p_dn16 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn16)) - (((4.0 * var_nqs_xi_dn16) * var_nqs_q) + (assign82380_e123635 * var_nqs_q_dn16))), (((var_nqs_p_dn17 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn17)) - (((4.0 * var_nqs_xi_dn17) * var_nqs_q) + (assign82380_e123635 * var_nqs_q_dn17))), (((var_nqs_p_dn18 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn18)) - (((4.0 * var_nqs_xi_dn18) * var_nqs_q) + (assign82380_e123635 * var_nqs_q_dn18))), (((var_nqs_p_dn19 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn19)) - (((4.0 * var_nqs_xi_dn19) * var_nqs_q) + (assign82380_e123635 * var_nqs_q_dn19))), (((var_nqs_p_dn20 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn20)) - (((4.0 * var_nqs_xi_dn20) * var_nqs_q) + (assign82380_e123635 * var_nqs_q_dn20))),)
    } else {
        (var_nqs_temp, var_nqs_temp_dn5, var_nqs_temp_dn6, var_nqs_temp_dn7, var_nqs_temp_dn8, var_nqs_temp_dn12, var_nqs_temp_dn13, var_nqs_temp_dn14, var_nqs_temp_dn15, var_nqs_temp_dn16, var_nqs_temp_dn17, var_nqs_temp_dn18, var_nqs_temp_dn19, var_nqs_temp_dn20,)
    }
};
        var_nqs_temp = assign82380_e123640;
        var_nqs_temp_dn5 = assign82380_e123640_d_n5;
        var_nqs_temp_dn6 = assign82380_e123640_d_n6;
        var_nqs_temp_dn7 = assign82380_e123640_d_n7;
        var_nqs_temp_dn8 = assign82380_e123640_d_n8;
        var_nqs_temp_dn12 = assign82380_e123640_d_n12;
        var_nqs_temp_dn13 = assign82380_e123640_d_n13;
        var_nqs_temp_dn14 = assign82380_e123640_d_n14;
        var_nqs_temp_dn15 = assign82380_e123640_d_n15;
        var_nqs_temp_dn16 = assign82380_e123640_d_n16;
        var_nqs_temp_dn17 = assign82380_e123640_d_n17;
        var_nqs_temp_dn18 = assign82380_e123640_d_n18;
        var_nqs_temp_dn19 = assign82380_e123640_d_n19;
        var_nqs_temp_dn20 = assign82380_e123640_d_n20;

        let (assign82390_e123670, assign82390_e123670_d_n5, assign82390_e123670_d_n6, assign82390_e123670_d_n7, assign82390_e123670_d_n8, assign82390_e123670_d_n12, assign82390_e123670_d_n13, assign82390_e123670_d_n14, assign82390_e123670_d_n15, assign82390_e123670_d_n16, assign82390_e123670_d_n17, assign82390_e123670_d_n18, assign82390_e123670_d_n19, assign82390_e123670_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) {
        let assign82390_e123663: f64 = (2.0 * var_nqs_q);
        let assign82390_e123666: f64 = (var_nqs_temp).sqrt();
        let assign82390_e123667: f64 = (var_nqs_p + assign82390_e123666);
        let assign82390_e123668: f64 = (assign82390_e123663 / assign82390_e123667);
        (assign82390_e123668, ((((2.0 * var_nqs_q_dn5) * assign82390_e123667) - (assign82390_e123663 * (var_nqs_p_dn5 + (var_nqs_temp_dn5 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * var_nqs_q_dn6) * assign82390_e123667) - (assign82390_e123663 * (var_nqs_p_dn6 + (var_nqs_temp_dn6 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * var_nqs_q_dn7) * assign82390_e123667) - (assign82390_e123663 * (var_nqs_p_dn7 + (var_nqs_temp_dn7 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * var_nqs_q_dn8) * assign82390_e123667) - (assign82390_e123663 * (var_nqs_p_dn8 + (var_nqs_temp_dn8 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * var_nqs_q_dn12) * assign82390_e123667) - (assign82390_e123663 * (var_nqs_p_dn12 + (var_nqs_temp_dn12 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * var_nqs_q_dn13) * assign82390_e123667) - (assign82390_e123663 * (var_nqs_p_dn13 + (var_nqs_temp_dn13 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * var_nqs_q_dn14) * assign82390_e123667) - (assign82390_e123663 * (var_nqs_p_dn14 + (var_nqs_temp_dn14 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * var_nqs_q_dn15) * assign82390_e123667) - (assign82390_e123663 * (var_nqs_p_dn15 + (var_nqs_temp_dn15 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * var_nqs_q_dn16) * assign82390_e123667) - (assign82390_e123663 * (var_nqs_p_dn16 + (var_nqs_temp_dn16 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * var_nqs_q_dn17) * assign82390_e123667) - (assign82390_e123663 * (var_nqs_p_dn17 + (var_nqs_temp_dn17 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * var_nqs_q_dn18) * assign82390_e123667) - (assign82390_e123663 * (var_nqs_p_dn18 + (var_nqs_temp_dn18 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * var_nqs_q_dn19) * assign82390_e123667) - (assign82390_e123663 * (var_nqs_p_dn19 + (var_nqs_temp_dn19 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)), ((((2.0 * var_nqs_q_dn20) * assign82390_e123667) - (assign82390_e123663 * (var_nqs_p_dn20 + (var_nqs_temp_dn20 / (2.0 * assign82390_e123666))))) / (assign82390_e123667 * assign82390_e123667)),)
    } else {
        (var_nqs_w, var_nqs_w_dn5, var_nqs_w_dn6, var_nqs_w_dn7, var_nqs_w_dn8, var_nqs_w_dn12, var_nqs_w_dn13, var_nqs_w_dn14, var_nqs_w_dn15, var_nqs_w_dn16, var_nqs_w_dn17, var_nqs_w_dn18, var_nqs_w_dn19, var_nqs_w_dn20,)
    }
};
        var_nqs_w = assign82390_e123670;
        var_nqs_w_dn5 = assign82390_e123670_d_n5;
        var_nqs_w_dn6 = assign82390_e123670_d_n6;
        var_nqs_w_dn7 = assign82390_e123670_d_n7;
        var_nqs_w_dn8 = assign82390_e123670_d_n8;
        var_nqs_w_dn12 = assign82390_e123670_d_n12;
        var_nqs_w_dn13 = assign82390_e123670_d_n13;
        var_nqs_w_dn14 = assign82390_e123670_d_n14;
        var_nqs_w_dn15 = assign82390_e123670_d_n15;
        var_nqs_w_dn16 = assign82390_e123670_d_n16;
        var_nqs_w_dn17 = assign82390_e123670_d_n17;
        var_nqs_w_dn18 = assign82390_e123670_d_n18;
        var_nqs_w_dn19 = assign82390_e123670_d_n19;
        var_nqs_w_dn20 = assign82390_e123670_d_n20;

        let (assign82400_e123696, assign82400_e123696_d_n5, assign82400_e123696_d_n6, assign82400_e123696_d_n7, assign82400_e123696_d_n8, assign82400_e123696_d_n12, assign82400_e123696_d_n13, assign82400_e123696_d_n14, assign82400_e123696_d_n15, assign82400_e123696_d_n16, assign82400_e123696_d_n17, assign82400_e123696_d_n18, assign82400_e123696_d_n19, assign82400_e123696_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 != 0.0)) {
        let assign82400_e123693: f64 = (var_nqs_y0 + var_nqs_w);
        let assign82400_e123694: f64 = (-assign82400_e123693);
        (assign82400_e123694, (-(var_nqs_y0_dn5 + var_nqs_w_dn5)), (-(var_nqs_y0_dn6 + var_nqs_w_dn6)), (-(var_nqs_y0_dn7 + var_nqs_w_dn7)), (-(var_nqs_y0_dn8 + var_nqs_w_dn8)), (-(var_nqs_y0_dn12 + var_nqs_w_dn12)), (-(var_nqs_y0_dn13 + var_nqs_w_dn13)), (-(var_nqs_y0_dn14 + var_nqs_w_dn14)), (-(var_nqs_y0_dn15 + var_nqs_w_dn15)), (-(var_nqs_y0_dn16 + var_nqs_w_dn16)), (-(var_nqs_y0_dn17 + var_nqs_w_dn17)), (-(var_nqs_y0_dn18 + var_nqs_w_dn18)), (-(var_nqs_y0_dn19 + var_nqs_w_dn19)), (-(var_nqs_y0_dn20 + var_nqs_w_dn20)),)
    } else {
        (var_temp9, var_temp9_dn5, var_temp9_dn6, var_temp9_dn7, var_temp9_dn8, var_temp9_dn12, var_temp9_dn13, var_temp9_dn14, var_temp9_dn15, var_temp9_dn16, var_temp9_dn17, var_temp9_dn18, var_temp9_dn19, var_temp9_dn20,)
    }
};
        var_temp9 = assign82400_e123696;
        var_temp9_dn5 = assign82400_e123696_d_n5;
        var_temp9_dn6 = assign82400_e123696_d_n6;
        var_temp9_dn7 = assign82400_e123696_d_n7;
        var_temp9_dn8 = assign82400_e123696_d_n8;
        var_temp9_dn12 = assign82400_e123696_d_n12;
        var_temp9_dn13 = assign82400_e123696_d_n13;
        var_temp9_dn14 = assign82400_e123696_d_n14;
        var_temp9_dn15 = assign82400_e123696_d_n15;
        var_temp9_dn16 = assign82400_e123696_d_n16;
        var_temp9_dn17 = assign82400_e123696_d_n17;
        var_temp9_dn18 = assign82400_e123696_d_n18;
        var_temp9_dn19 = assign82400_e123696_d_n19;
        var_temp9_dn20 = assign82400_e123696_d_n20;

        let (assign82410_e123726, assign82410_e123726_d_n5, assign82410_e123726_d_n6, assign82410_e123726_d_n7, assign82410_e123726_d_n8, assign82410_e123726_d_n12, assign82410_e123726_d_n13, assign82410_e123726_d_n14, assign82410_e123726_d_n15, assign82410_e123726_d_n16, assign82410_e123726_d_n17, assign82410_e123726_d_n18, assign82410_e123726_d_n19, assign82410_e123726_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) {
        let assign82410_e123722: f64 = (0.732464877560822 * var_gp);
        let assign82410_e123723: f64 = (1.25 + assign82410_e123722);
        let assign82410_e123724: f64 = (1.0 / assign82410_e123723);
        (assign82410_e123724, (-((0.732464877560822 * var_gp_dn5) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * var_gp_dn6) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * var_gp_dn7) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * var_gp_dn8) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * var_gp_dn12) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * var_gp_dn13) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * var_gp_dn14) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * var_gp_dn15) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * var_gp_dn16) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * var_gp_dn17) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * var_gp_dn18) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * var_gp_dn19) / (assign82410_e123723 * assign82410_e123723))), (-((0.732464877560822 * var_gp_dn20) / (assign82410_e123723 * assign82410_e123723))),)
    } else {
        (var_nqs_xg1, var_nqs_xg1_dn5, var_nqs_xg1_dn6, var_nqs_xg1_dn7, var_nqs_xg1_dn8, var_nqs_xg1_dn12, var_nqs_xg1_dn13, var_nqs_xg1_dn14, var_nqs_xg1_dn15, var_nqs_xg1_dn16, var_nqs_xg1_dn17, var_nqs_xg1_dn18, var_nqs_xg1_dn19, var_nqs_xg1_dn20,)
    }
};
        var_nqs_xg1 = assign82410_e123726;
        var_nqs_xg1_dn5 = assign82410_e123726_d_n5;
        var_nqs_xg1_dn6 = assign82410_e123726_d_n6;
        var_nqs_xg1_dn7 = assign82410_e123726_d_n7;
        var_nqs_xg1_dn8 = assign82410_e123726_d_n8;
        var_nqs_xg1_dn12 = assign82410_e123726_d_n12;
        var_nqs_xg1_dn13 = assign82410_e123726_d_n13;
        var_nqs_xg1_dn14 = assign82410_e123726_d_n14;
        var_nqs_xg1_dn15 = assign82410_e123726_d_n15;
        var_nqs_xg1_dn16 = assign82410_e123726_d_n16;
        var_nqs_xg1_dn17 = assign82410_e123726_d_n17;
        var_nqs_xg1_dn18 = assign82410_e123726_d_n18;
        var_nqs_xg1_dn19 = assign82410_e123726_d_n19;
        var_nqs_xg1_dn20 = assign82410_e123726_d_n20;

        let (assign82420_e123758, assign82420_e123758_d_n5, assign82420_e123758_d_n6, assign82420_e123758_d_n7, assign82420_e123758_d_n8, assign82420_e123758_d_n12, assign82420_e123758_d_n13, assign82420_e123758_d_n14, assign82420_e123758_d_n15, assign82420_e123758_d_n16, assign82420_e123758_d_n17, assign82420_e123758_d_n18, assign82420_e123758_d_n19, assign82420_e123758_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) {
        let assign82420_e123750: f64 = (1.25 * var_a_factrp);
        let assign82420_e123752: f64 = (assign82420_e123750 * var_nqs_xg1);
        let assign82420_e123754: f64 = (assign82420_e123752 - 1.0);
        let assign82420_e123756: f64 = (assign82420_e123754 * var_nqs_xg1);
        (assign82420_e123756, (((((1.25 * var_a_factrp_dn5) * var_nqs_xg1) + (assign82420_e123750 * var_nqs_xg1_dn5)) * var_nqs_xg1) + (assign82420_e123754 * var_nqs_xg1_dn5)), (((((1.25 * var_a_factrp_dn6) * var_nqs_xg1) + (assign82420_e123750 * var_nqs_xg1_dn6)) * var_nqs_xg1) + (assign82420_e123754 * var_nqs_xg1_dn6)), (((((1.25 * var_a_factrp_dn7) * var_nqs_xg1) + (assign82420_e123750 * var_nqs_xg1_dn7)) * var_nqs_xg1) + (assign82420_e123754 * var_nqs_xg1_dn7)), (((((1.25 * var_a_factrp_dn8) * var_nqs_xg1) + (assign82420_e123750 * var_nqs_xg1_dn8)) * var_nqs_xg1) + (assign82420_e123754 * var_nqs_xg1_dn8)), (((((1.25 * var_a_factrp_dn12) * var_nqs_xg1) + (assign82420_e123750 * var_nqs_xg1_dn12)) * var_nqs_xg1) + (assign82420_e123754 * var_nqs_xg1_dn12)), (((((1.25 * var_a_factrp_dn13) * var_nqs_xg1) + (assign82420_e123750 * var_nqs_xg1_dn13)) * var_nqs_xg1) + (assign82420_e123754 * var_nqs_xg1_dn13)), (((((1.25 * var_a_factrp_dn14) * var_nqs_xg1) + (assign82420_e123750 * var_nqs_xg1_dn14)) * var_nqs_xg1) + (assign82420_e123754 * var_nqs_xg1_dn14)), (((((1.25 * var_a_factrp_dn15) * var_nqs_xg1) + (assign82420_e123750 * var_nqs_xg1_dn15)) * var_nqs_xg1) + (assign82420_e123754 * var_nqs_xg1_dn15)), (((((1.25 * var_a_factrp_dn16) * var_nqs_xg1) + (assign82420_e123750 * var_nqs_xg1_dn16)) * var_nqs_xg1) + (assign82420_e123754 * var_nqs_xg1_dn16)), (((((1.25 * var_a_factrp_dn17) * var_nqs_xg1) + (assign82420_e123750 * var_nqs_xg1_dn17)) * var_nqs_xg1) + (assign82420_e123754 * var_nqs_xg1_dn17)), (((((1.25 * var_a_factrp_dn18) * var_nqs_xg1) + (assign82420_e123750 * var_nqs_xg1_dn18)) * var_nqs_xg1) + (assign82420_e123754 * var_nqs_xg1_dn18)), (((((1.25 * var_a_factrp_dn19) * var_nqs_xg1) + (assign82420_e123750 * var_nqs_xg1_dn19)) * var_nqs_xg1) + (assign82420_e123754 * var_nqs_xg1_dn19)), (((((1.25 * var_a_factrp_dn20) * var_nqs_xg1) + (assign82420_e123750 * var_nqs_xg1_dn20)) * var_nqs_xg1) + (assign82420_e123754 * var_nqs_xg1_dn20)),)
    } else {
        (var_nqs_a_fac, var_nqs_a_fac_dn5, var_nqs_a_fac_dn6, var_nqs_a_fac_dn7, var_nqs_a_fac_dn8, var_nqs_a_fac_dn12, var_nqs_a_fac_dn13, var_nqs_a_fac_dn14, var_nqs_a_fac_dn15, var_nqs_a_fac_dn16, var_nqs_a_fac_dn17, var_nqs_a_fac_dn18, var_nqs_a_fac_dn19, var_nqs_a_fac_dn20,)
    }
};
        var_nqs_a_fac = assign82420_e123758;
        var_nqs_a_fac_dn5 = assign82420_e123758_d_n5;
        var_nqs_a_fac_dn6 = assign82420_e123758_d_n6;
        var_nqs_a_fac_dn7 = assign82420_e123758_d_n7;
        var_nqs_a_fac_dn8 = assign82420_e123758_d_n8;
        var_nqs_a_fac_dn12 = assign82420_e123758_d_n12;
        var_nqs_a_fac_dn13 = assign82420_e123758_d_n13;
        var_nqs_a_fac_dn14 = assign82420_e123758_d_n14;
        var_nqs_a_fac_dn15 = assign82420_e123758_d_n15;
        var_nqs_a_fac_dn16 = assign82420_e123758_d_n16;
        var_nqs_a_fac_dn17 = assign82420_e123758_d_n17;
        var_nqs_a_fac_dn18 = assign82420_e123758_d_n18;
        var_nqs_a_fac_dn19 = assign82420_e123758_d_n19;
        var_nqs_a_fac_dn20 = assign82420_e123758_d_n20;

        let (assign82430_e123790, assign82430_e123790_d_n5, assign82430_e123790_d_n6, assign82430_e123790_d_n7, assign82430_e123790_d_n8, assign82430_e123790_d_n12, assign82430_e123790_d_n13, assign82430_e123790_d_n14, assign82430_e123790_d_n15, assign82430_e123790_d_n16, assign82430_e123790_d_n17, assign82430_e123790_d_n18, assign82430_e123790_d_n19, assign82430_e123790_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) {
        let assign82430_e123782: f64 = (var_temp__blk1038 / var_a_factrp);
        let assign82430_e123786: f64 = (var_nqs_a_fac * var_temp__blk1038);
        let assign82430_e123787: f64 = (1.0 + assign82430_e123786);
        let assign82430_e123788: f64 = (assign82430_e123782 * assign82430_e123787);
        (assign82430_e123788, (((((var_temp__blk1038_dn5 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn5)) / (var_a_factrp * var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((var_nqs_a_fac_dn5 * var_temp__blk1038) + (var_nqs_a_fac * var_temp__blk1038_dn5)))), (((((var_temp__blk1038_dn6 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn6)) / (var_a_factrp * var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((var_nqs_a_fac_dn6 * var_temp__blk1038) + (var_nqs_a_fac * var_temp__blk1038_dn6)))), (((((var_temp__blk1038_dn7 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn7)) / (var_a_factrp * var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((var_nqs_a_fac_dn7 * var_temp__blk1038) + (var_nqs_a_fac * var_temp__blk1038_dn7)))), (((((var_temp__blk1038_dn8 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn8)) / (var_a_factrp * var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((var_nqs_a_fac_dn8 * var_temp__blk1038) + (var_nqs_a_fac * var_temp__blk1038_dn8)))), (((((var_temp__blk1038_dn12 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn12)) / (var_a_factrp * var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((var_nqs_a_fac_dn12 * var_temp__blk1038) + (var_nqs_a_fac * var_temp__blk1038_dn12)))), (((((var_temp__blk1038_dn13 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn13)) / (var_a_factrp * var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((var_nqs_a_fac_dn13 * var_temp__blk1038) + (var_nqs_a_fac * var_temp__blk1038_dn13)))), (((((var_temp__blk1038_dn14 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn14)) / (var_a_factrp * var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((var_nqs_a_fac_dn14 * var_temp__blk1038) + (var_nqs_a_fac * var_temp__blk1038_dn14)))), (((((var_temp__blk1038_dn15 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn15)) / (var_a_factrp * var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((var_nqs_a_fac_dn15 * var_temp__blk1038) + (var_nqs_a_fac * var_temp__blk1038_dn15)))), (((((var_temp__blk1038_dn16 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn16)) / (var_a_factrp * var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((var_nqs_a_fac_dn16 * var_temp__blk1038) + (var_nqs_a_fac * var_temp__blk1038_dn16)))), (((((var_temp__blk1038_dn17 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn17)) / (var_a_factrp * var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((var_nqs_a_fac_dn17 * var_temp__blk1038) + (var_nqs_a_fac * var_temp__blk1038_dn17)))), (((((var_temp__blk1038_dn18 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn18)) / (var_a_factrp * var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((var_nqs_a_fac_dn18 * var_temp__blk1038) + (var_nqs_a_fac * var_temp__blk1038_dn18)))), (((((var_temp__blk1038_dn19 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn19)) / (var_a_factrp * var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((var_nqs_a_fac_dn19 * var_temp__blk1038) + (var_nqs_a_fac * var_temp__blk1038_dn19)))), (((((var_temp__blk1038_dn20 * var_a_factrp) - (var_temp__blk1038 * var_a_factrp_dn20)) / (var_a_factrp * var_a_factrp)) * assign82430_e123787) + (assign82430_e123782 * ((var_nqs_a_fac_dn20 * var_temp__blk1038) + (var_nqs_a_fac * var_temp__blk1038_dn20)))),)
    } else {
        (var_nqs_xbar, var_nqs_xbar_dn5, var_nqs_xbar_dn6, var_nqs_xbar_dn7, var_nqs_xbar_dn8, var_nqs_xbar_dn12, var_nqs_xbar_dn13, var_nqs_xbar_dn14, var_nqs_xbar_dn15, var_nqs_xbar_dn16, var_nqs_xbar_dn17, var_nqs_xbar_dn18, var_nqs_xbar_dn19, var_nqs_xbar_dn20,)
    }
};
        var_nqs_xbar = assign82430_e123790;
        var_nqs_xbar_dn5 = assign82430_e123790_d_n5;
        var_nqs_xbar_dn6 = assign82430_e123790_d_n6;
        var_nqs_xbar_dn7 = assign82430_e123790_d_n7;
        var_nqs_xbar_dn8 = assign82430_e123790_d_n8;
        var_nqs_xbar_dn12 = assign82430_e123790_d_n12;
        var_nqs_xbar_dn13 = assign82430_e123790_d_n13;
        var_nqs_xbar_dn14 = assign82430_e123790_d_n14;
        var_nqs_xbar_dn15 = assign82430_e123790_d_n15;
        var_nqs_xbar_dn16 = assign82430_e123790_d_n16;
        var_nqs_xbar_dn17 = assign82430_e123790_d_n17;
        var_nqs_xbar_dn18 = assign82430_e123790_d_n18;
        var_nqs_xbar_dn19 = assign82430_e123790_d_n19;
        var_nqs_xbar_dn20 = assign82430_e123790_d_n20;

        let assign82440_e123792: f64 = (-var_nqs_xbar);
        let assign82440_e123793: f64 = (assign82440_e123792).abs();
        let assign82440_e123795: f64 = if assign82440_e123793 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard2240 = assign82440_e123795;

        let (assign82450_e123823, assign82450_e123823_d_n5, assign82450_e123823_d_n6, assign82450_e123823_d_n7, assign82450_e123823_d_n8, assign82450_e123823_d_n12, assign82450_e123823_d_n13, assign82450_e123823_d_n14, assign82450_e123823_d_n15, assign82450_e123823_d_n16, assign82450_e123823_d_n17, assign82450_e123823_d_n18, assign82450_e123823_d_n19, assign82450_e123823_d_n20,) = {
    if (((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) && (var_guard2240 != 0.0)) {
        let assign82450_e123820: f64 = (-var_nqs_xbar);
        let assign82450_e123821: f64 = (assign82450_e123820).exp();
        (assign82450_e123821, (assign82450_e123821 * (-var_nqs_xbar_dn5)), (assign82450_e123821 * (-var_nqs_xbar_dn6)), (assign82450_e123821 * (-var_nqs_xbar_dn7)), (assign82450_e123821 * (-var_nqs_xbar_dn8)), (assign82450_e123821 * (-var_nqs_xbar_dn12)), (assign82450_e123821 * (-var_nqs_xbar_dn13)), (assign82450_e123821 * (-var_nqs_xbar_dn14)), (assign82450_e123821 * (-var_nqs_xbar_dn15)), (assign82450_e123821 * (-var_nqs_xbar_dn16)), (assign82450_e123821 * (-var_nqs_xbar_dn17)), (assign82450_e123821 * (-var_nqs_xbar_dn18)), (assign82450_e123821 * (-var_nqs_xbar_dn19)), (assign82450_e123821 * (-var_nqs_xbar_dn20)),)
    } else {
        (var_nqs_temp, var_nqs_temp_dn5, var_nqs_temp_dn6, var_nqs_temp_dn7, var_nqs_temp_dn8, var_nqs_temp_dn12, var_nqs_temp_dn13, var_nqs_temp_dn14, var_nqs_temp_dn15, var_nqs_temp_dn16, var_nqs_temp_dn17, var_nqs_temp_dn18, var_nqs_temp_dn19, var_nqs_temp_dn20,)
    }
};
        var_nqs_temp = assign82450_e123823;
        var_nqs_temp_dn5 = assign82450_e123823_d_n5;
        var_nqs_temp_dn6 = assign82450_e123823_d_n6;
        var_nqs_temp_dn7 = assign82450_e123823_d_n7;
        var_nqs_temp_dn8 = assign82450_e123823_d_n8;
        var_nqs_temp_dn12 = assign82450_e123823_d_n12;
        var_nqs_temp_dn13 = assign82450_e123823_d_n13;
        var_nqs_temp_dn14 = assign82450_e123823_d_n14;
        var_nqs_temp_dn15 = assign82450_e123823_d_n15;
        var_nqs_temp_dn16 = assign82450_e123823_d_n16;
        var_nqs_temp_dn17 = assign82450_e123823_d_n17;
        var_nqs_temp_dn18 = assign82450_e123823_d_n18;
        var_nqs_temp_dn19 = assign82450_e123823_d_n19;
        var_nqs_temp_dn20 = assign82450_e123823_d_n20;

        let assign82460_e123825: f64 = (-var_nqs_xbar);
        let assign82460_e123827: f64 = if assign82460_e123825 < 0.0 { 1.0 } else { 0.0 };
        var_guard2241 = assign82460_e123827;

        let (assign82470_e123884, assign82470_e123884_d_n5, assign82470_e123884_d_n6, assign82470_e123884_d_n7, assign82470_e123884_d_n8, assign82470_e123884_d_n12, assign82470_e123884_d_n13, assign82470_e123884_d_n14, assign82470_e123884_d_n15, assign82470_e123884_d_n16, assign82470_e123884_d_n17, assign82470_e123884_d_n18, assign82470_e123884_d_n19, assign82470_e123884_d_n20,) = {
    if ((((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) && (var_guard2240 == 0.0)) && (var_guard2241 != 0.0)) {
        let assign82470_e123857: f64 = (-230.25850929940458);
        let assign82470_e123859: f64 = (-var_nqs_xbar);
        let assign82470_e123860: f64 = (assign82470_e123857 - assign82470_e123859);
        let assign82470_e123864: f64 = (-230.25850929940458);
        let assign82470_e123866: f64 = (-var_nqs_xbar);
        let assign82470_e123867: f64 = (assign82470_e123864 - assign82470_e123866);
        let assign82470_e123870: f64 = (-230.25850929940458);
        let assign82470_e123872: f64 = (-var_nqs_xbar);
        let assign82470_e123873: f64 = (assign82470_e123870 - assign82470_e123872);
        let assign82470_e123875: f64 = (assign82470_e123873 * 0.3333333333333333);
        let assign82470_e123876: f64 = (1.0 + assign82470_e123875);
        let assign82470_e123877: f64 = (assign82470_e123867 * assign82470_e123876);
        let assign82470_e123878: f64 = (0.5 * assign82470_e123877);
        let assign82470_e123879: f64 = (1.0 + assign82470_e123878);
        let assign82470_e123880: f64 = (assign82470_e123860 * assign82470_e123879);
        let assign82470_e123881: f64 = (1.0 + assign82470_e123880);
        let assign82470_e123882: f64 = (1e-100 / assign82470_e123881);
        (assign82470_e123882, (-((1e-100 * (((-(-var_nqs_xbar_dn5)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-var_nqs_xbar_dn5)) * assign82470_e123876) + (assign82470_e123867 * ((-(-var_nqs_xbar_dn5)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-var_nqs_xbar_dn6)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-var_nqs_xbar_dn6)) * assign82470_e123876) + (assign82470_e123867 * ((-(-var_nqs_xbar_dn6)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-var_nqs_xbar_dn7)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-var_nqs_xbar_dn7)) * assign82470_e123876) + (assign82470_e123867 * ((-(-var_nqs_xbar_dn7)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-var_nqs_xbar_dn8)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-var_nqs_xbar_dn8)) * assign82470_e123876) + (assign82470_e123867 * ((-(-var_nqs_xbar_dn8)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-var_nqs_xbar_dn12)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-var_nqs_xbar_dn12)) * assign82470_e123876) + (assign82470_e123867 * ((-(-var_nqs_xbar_dn12)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-var_nqs_xbar_dn13)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-var_nqs_xbar_dn13)) * assign82470_e123876) + (assign82470_e123867 * ((-(-var_nqs_xbar_dn13)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-var_nqs_xbar_dn14)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-var_nqs_xbar_dn14)) * assign82470_e123876) + (assign82470_e123867 * ((-(-var_nqs_xbar_dn14)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-var_nqs_xbar_dn15)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-var_nqs_xbar_dn15)) * assign82470_e123876) + (assign82470_e123867 * ((-(-var_nqs_xbar_dn15)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-var_nqs_xbar_dn16)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-var_nqs_xbar_dn16)) * assign82470_e123876) + (assign82470_e123867 * ((-(-var_nqs_xbar_dn16)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-var_nqs_xbar_dn17)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-var_nqs_xbar_dn17)) * assign82470_e123876) + (assign82470_e123867 * ((-(-var_nqs_xbar_dn17)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-var_nqs_xbar_dn18)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-var_nqs_xbar_dn18)) * assign82470_e123876) + (assign82470_e123867 * ((-(-var_nqs_xbar_dn18)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-var_nqs_xbar_dn19)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-var_nqs_xbar_dn19)) * assign82470_e123876) + (assign82470_e123867 * ((-(-var_nqs_xbar_dn19)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))), (-((1e-100 * (((-(-var_nqs_xbar_dn20)) * assign82470_e123879) + (assign82470_e123860 * (0.5 * (((-(-var_nqs_xbar_dn20)) * assign82470_e123876) + (assign82470_e123867 * ((-(-var_nqs_xbar_dn20)) * 0.3333333333333333))))))) / (assign82470_e123881 * assign82470_e123881))),)
    } else {
        (var_nqs_temp, var_nqs_temp_dn5, var_nqs_temp_dn6, var_nqs_temp_dn7, var_nqs_temp_dn8, var_nqs_temp_dn12, var_nqs_temp_dn13, var_nqs_temp_dn14, var_nqs_temp_dn15, var_nqs_temp_dn16, var_nqs_temp_dn17, var_nqs_temp_dn18, var_nqs_temp_dn19, var_nqs_temp_dn20,)
    }
};
        var_nqs_temp = assign82470_e123884;
        var_nqs_temp_dn5 = assign82470_e123884_d_n5;
        var_nqs_temp_dn6 = assign82470_e123884_d_n6;
        var_nqs_temp_dn7 = assign82470_e123884_d_n7;
        var_nqs_temp_dn8 = assign82470_e123884_d_n8;
        var_nqs_temp_dn12 = assign82470_e123884_d_n12;
        var_nqs_temp_dn13 = assign82470_e123884_d_n13;
        var_nqs_temp_dn14 = assign82470_e123884_d_n14;
        var_nqs_temp_dn15 = assign82470_e123884_d_n15;
        var_nqs_temp_dn16 = assign82470_e123884_d_n16;
        var_nqs_temp_dn17 = assign82470_e123884_d_n17;
        var_nqs_temp_dn18 = assign82470_e123884_d_n18;
        var_nqs_temp_dn19 = assign82470_e123884_d_n19;
        var_nqs_temp_dn20 = assign82470_e123884_d_n20;

        let (assign82480_e123939, assign82480_e123939_d_n5, assign82480_e123939_d_n6, assign82480_e123939_d_n7, assign82480_e123939_d_n8, assign82480_e123939_d_n12, assign82480_e123939_d_n13, assign82480_e123939_d_n14, assign82480_e123939_d_n15, assign82480_e123939_d_n16, assign82480_e123939_d_n17, assign82480_e123939_d_n18, assign82480_e123939_d_n19, assign82480_e123939_d_n20,) = {
    if ((((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) && (var_guard2240 == 0.0)) && (var_guard2241 == 0.0)) {
        let assign82480_e123915: f64 = (-var_nqs_xbar);
        let assign82480_e123917: f64 = (assign82480_e123915 - 230.25850929940458);
        let assign82480_e123921: f64 = (-var_nqs_xbar);
        let assign82480_e123923: f64 = (assign82480_e123921 - 230.25850929940458);
        let assign82480_e123926: f64 = (-var_nqs_xbar);
        let assign82480_e123928: f64 = (assign82480_e123926 - 230.25850929940458);
        let assign82480_e123930: f64 = (assign82480_e123928 * 0.3333333333333333);
        let assign82480_e123931: f64 = (1.0 + assign82480_e123930);
        let assign82480_e123932: f64 = (assign82480_e123923 * assign82480_e123931);
        let assign82480_e123933: f64 = (0.5 * assign82480_e123932);
        let assign82480_e123934: f64 = (1.0 + assign82480_e123933);
        let assign82480_e123935: f64 = (assign82480_e123917 * assign82480_e123934);
        let assign82480_e123936: f64 = (1.0 + assign82480_e123935);
        let assign82480_e123937: f64 = (1e100 * assign82480_e123936);
        (assign82480_e123937, (1e100 * (((-var_nqs_xbar_dn5) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-var_nqs_xbar_dn5) * assign82480_e123931) + (assign82480_e123923 * ((-var_nqs_xbar_dn5) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_xbar_dn6) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-var_nqs_xbar_dn6) * assign82480_e123931) + (assign82480_e123923 * ((-var_nqs_xbar_dn6) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_xbar_dn7) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-var_nqs_xbar_dn7) * assign82480_e123931) + (assign82480_e123923 * ((-var_nqs_xbar_dn7) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_xbar_dn8) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-var_nqs_xbar_dn8) * assign82480_e123931) + (assign82480_e123923 * ((-var_nqs_xbar_dn8) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_xbar_dn12) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-var_nqs_xbar_dn12) * assign82480_e123931) + (assign82480_e123923 * ((-var_nqs_xbar_dn12) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_xbar_dn13) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-var_nqs_xbar_dn13) * assign82480_e123931) + (assign82480_e123923 * ((-var_nqs_xbar_dn13) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_xbar_dn14) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-var_nqs_xbar_dn14) * assign82480_e123931) + (assign82480_e123923 * ((-var_nqs_xbar_dn14) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_xbar_dn15) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-var_nqs_xbar_dn15) * assign82480_e123931) + (assign82480_e123923 * ((-var_nqs_xbar_dn15) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_xbar_dn16) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-var_nqs_xbar_dn16) * assign82480_e123931) + (assign82480_e123923 * ((-var_nqs_xbar_dn16) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_xbar_dn17) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-var_nqs_xbar_dn17) * assign82480_e123931) + (assign82480_e123923 * ((-var_nqs_xbar_dn17) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_xbar_dn18) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-var_nqs_xbar_dn18) * assign82480_e123931) + (assign82480_e123923 * ((-var_nqs_xbar_dn18) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_xbar_dn19) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-var_nqs_xbar_dn19) * assign82480_e123931) + (assign82480_e123923 * ((-var_nqs_xbar_dn19) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_xbar_dn20) * assign82480_e123934) + (assign82480_e123917 * (0.5 * (((-var_nqs_xbar_dn20) * assign82480_e123931) + (assign82480_e123923 * ((-var_nqs_xbar_dn20) * 0.3333333333333333))))))),)
    } else {
        (var_nqs_temp, var_nqs_temp_dn5, var_nqs_temp_dn6, var_nqs_temp_dn7, var_nqs_temp_dn8, var_nqs_temp_dn12, var_nqs_temp_dn13, var_nqs_temp_dn14, var_nqs_temp_dn15, var_nqs_temp_dn16, var_nqs_temp_dn17, var_nqs_temp_dn18, var_nqs_temp_dn19, var_nqs_temp_dn20,)
    }
};
        var_nqs_temp = assign82480_e123939;
        var_nqs_temp_dn5 = assign82480_e123939_d_n5;
        var_nqs_temp_dn6 = assign82480_e123939_d_n6;
        var_nqs_temp_dn7 = assign82480_e123939_d_n7;
        var_nqs_temp_dn8 = assign82480_e123939_d_n8;
        var_nqs_temp_dn12 = assign82480_e123939_d_n12;
        var_nqs_temp_dn13 = assign82480_e123939_d_n13;
        var_nqs_temp_dn14 = assign82480_e123939_d_n14;
        var_nqs_temp_dn15 = assign82480_e123939_d_n15;
        var_nqs_temp_dn16 = assign82480_e123939_d_n16;
        var_nqs_temp_dn17 = assign82480_e123939_d_n17;
        var_nqs_temp_dn18 = assign82480_e123939_d_n18;
        var_nqs_temp_dn19 = assign82480_e123939_d_n19;
        var_nqs_temp_dn20 = assign82480_e123939_d_n20;

        let (assign82490_e123965, assign82490_e123965_d_n5, assign82490_e123965_d_n6, assign82490_e123965_d_n7, assign82490_e123965_d_n8, assign82490_e123965_d_n12, assign82490_e123965_d_n13, assign82490_e123965_d_n14, assign82490_e123965_d_n15, assign82490_e123965_d_n16, assign82490_e123965_d_n17, assign82490_e123965_d_n18, assign82490_e123965_d_n19, assign82490_e123965_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) {
        let assign82490_e123963: f64 = (1.0 - var_nqs_temp);
        (assign82490_e123963, (-var_nqs_temp_dn5), (-var_nqs_temp_dn6), (-var_nqs_temp_dn7), (-var_nqs_temp_dn8), (-var_nqs_temp_dn12), (-var_nqs_temp_dn13), (-var_nqs_temp_dn14), (-var_nqs_temp_dn15), (-var_nqs_temp_dn16), (-var_nqs_temp_dn17), (-var_nqs_temp_dn18), (-var_nqs_temp_dn19), (-var_nqs_temp_dn20),)
    } else {
        (var_nqs_w, var_nqs_w_dn5, var_nqs_w_dn6, var_nqs_w_dn7, var_nqs_w_dn8, var_nqs_w_dn12, var_nqs_w_dn13, var_nqs_w_dn14, var_nqs_w_dn15, var_nqs_w_dn16, var_nqs_w_dn17, var_nqs_w_dn18, var_nqs_w_dn19, var_nqs_w_dn20,)
    }
};
        var_nqs_w = assign82490_e123965;
        var_nqs_w_dn5 = assign82490_e123965_d_n5;
        var_nqs_w_dn6 = assign82490_e123965_d_n6;
        var_nqs_w_dn7 = assign82490_e123965_d_n7;
        var_nqs_w_dn8 = assign82490_e123965_d_n8;
        var_nqs_w_dn12 = assign82490_e123965_d_n12;
        var_nqs_w_dn13 = assign82490_e123965_d_n13;
        var_nqs_w_dn14 = assign82490_e123965_d_n14;
        var_nqs_w_dn15 = assign82490_e123965_d_n15;
        var_nqs_w_dn16 = assign82490_e123965_d_n16;
        var_nqs_w_dn17 = assign82490_e123965_d_n17;
        var_nqs_w_dn18 = assign82490_e123965_d_n18;
        var_nqs_w_dn19 = assign82490_e123965_d_n19;
        var_nqs_w_dn20 = assign82490_e123965_d_n20;

        let (assign82500_e124004, assign82500_e124004_d_n5, assign82500_e124004_d_n6, assign82500_e124004_d_n7, assign82500_e124004_d_n8, assign82500_e124004_d_n12, assign82500_e124004_d_n13, assign82500_e124004_d_n14, assign82500_e124004_d_n15, assign82500_e124004_d_n16, assign82500_e124004_d_n17, assign82500_e124004_d_n18, assign82500_e124004_d_n19, assign82500_e124004_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) {
        let assign82500_e123990: f64 = (var_gp2 * 0.5);
        let assign82500_e123991: f64 = (var_temp__blk1038 + assign82500_e123990);
        let assign82500_e123996: f64 = (var_gp2 * 0.25);
        let assign82500_e123997: f64 = (var_temp__blk1038 + assign82500_e123996);
        let assign82500_e123999: f64 = (assign82500_e123997 - var_nqs_w);
        let assign82500_e124000: f64 = (assign82500_e123999).sqrt();
        let assign82500_e124001: f64 = (var_gp * assign82500_e124000);
        let assign82500_e124002: f64 = (assign82500_e123991 - assign82500_e124001);
        (assign82500_e124002, ((var_temp__blk1038_dn5 + (var_gp2_dn5 * 0.5)) - ((var_gp_dn5 * assign82500_e124000) + (var_gp * (((var_temp__blk1038_dn5 + (var_gp2_dn5 * 0.25)) - var_nqs_w_dn5) / (2.0 * assign82500_e124000))))), ((var_temp__blk1038_dn6 + (var_gp2_dn6 * 0.5)) - ((var_gp_dn6 * assign82500_e124000) + (var_gp * (((var_temp__blk1038_dn6 + (var_gp2_dn6 * 0.25)) - var_nqs_w_dn6) / (2.0 * assign82500_e124000))))), ((var_temp__blk1038_dn7 + (var_gp2_dn7 * 0.5)) - ((var_gp_dn7 * assign82500_e124000) + (var_gp * (((var_temp__blk1038_dn7 + (var_gp2_dn7 * 0.25)) - var_nqs_w_dn7) / (2.0 * assign82500_e124000))))), ((var_temp__blk1038_dn8 + (var_gp2_dn8 * 0.5)) - ((var_gp_dn8 * assign82500_e124000) + (var_gp * (((var_temp__blk1038_dn8 + (var_gp2_dn8 * 0.25)) - var_nqs_w_dn8) / (2.0 * assign82500_e124000))))), ((var_temp__blk1038_dn12 + (var_gp2_dn12 * 0.5)) - ((var_gp_dn12 * assign82500_e124000) + (var_gp * (((var_temp__blk1038_dn12 + (var_gp2_dn12 * 0.25)) - var_nqs_w_dn12) / (2.0 * assign82500_e124000))))), ((var_temp__blk1038_dn13 + (var_gp2_dn13 * 0.5)) - ((var_gp_dn13 * assign82500_e124000) + (var_gp * (((var_temp__blk1038_dn13 + (var_gp2_dn13 * 0.25)) - var_nqs_w_dn13) / (2.0 * assign82500_e124000))))), ((var_temp__blk1038_dn14 + (var_gp2_dn14 * 0.5)) - ((var_gp_dn14 * assign82500_e124000) + (var_gp * (((var_temp__blk1038_dn14 + (var_gp2_dn14 * 0.25)) - var_nqs_w_dn14) / (2.0 * assign82500_e124000))))), ((var_temp__blk1038_dn15 + (var_gp2_dn15 * 0.5)) - ((var_gp_dn15 * assign82500_e124000) + (var_gp * (((var_temp__blk1038_dn15 + (var_gp2_dn15 * 0.25)) - var_nqs_w_dn15) / (2.0 * assign82500_e124000))))), ((var_temp__blk1038_dn16 + (var_gp2_dn16 * 0.5)) - ((var_gp_dn16 * assign82500_e124000) + (var_gp * (((var_temp__blk1038_dn16 + (var_gp2_dn16 * 0.25)) - var_nqs_w_dn16) / (2.0 * assign82500_e124000))))), ((var_temp__blk1038_dn17 + (var_gp2_dn17 * 0.5)) - ((var_gp_dn17 * assign82500_e124000) + (var_gp * (((var_temp__blk1038_dn17 + (var_gp2_dn17 * 0.25)) - var_nqs_w_dn17) / (2.0 * assign82500_e124000))))), ((var_temp__blk1038_dn18 + (var_gp2_dn18 * 0.5)) - ((var_gp_dn18 * assign82500_e124000) + (var_gp * (((var_temp__blk1038_dn18 + (var_gp2_dn18 * 0.25)) - var_nqs_w_dn18) / (2.0 * assign82500_e124000))))), ((var_temp__blk1038_dn19 + (var_gp2_dn19 * 0.5)) - ((var_gp_dn19 * assign82500_e124000) + (var_gp * (((var_temp__blk1038_dn19 + (var_gp2_dn19 * 0.25)) - var_nqs_w_dn19) / (2.0 * assign82500_e124000))))), ((var_temp__blk1038_dn20 + (var_gp2_dn20 * 0.5)) - ((var_gp_dn20 * assign82500_e124000) + (var_gp * (((var_temp__blk1038_dn20 + (var_gp2_dn20 * 0.25)) - var_nqs_w_dn20) / (2.0 * assign82500_e124000))))),)
    } else {
        (var_nqs_x0, var_nqs_x0_dn5, var_nqs_x0_dn6, var_nqs_x0_dn7, var_nqs_x0_dn8, var_nqs_x0_dn12, var_nqs_x0_dn13, var_nqs_x0_dn14, var_nqs_x0_dn15, var_nqs_x0_dn16, var_nqs_x0_dn17, var_nqs_x0_dn18, var_nqs_x0_dn19, var_nqs_x0_dn20,)
    }
};
        var_nqs_x0 = assign82500_e124004;
        var_nqs_x0_dn5 = assign82500_e124004_d_n5;
        var_nqs_x0_dn6 = assign82500_e124004_d_n6;
        var_nqs_x0_dn7 = assign82500_e124004_d_n7;
        var_nqs_x0_dn8 = assign82500_e124004_d_n8;
        var_nqs_x0_dn12 = assign82500_e124004_d_n12;
        var_nqs_x0_dn13 = assign82500_e124004_d_n13;
        var_nqs_x0_dn14 = assign82500_e124004_d_n14;
        var_nqs_x0_dn15 = assign82500_e124004_d_n15;
        var_nqs_x0_dn16 = assign82500_e124004_d_n16;
        var_nqs_x0_dn17 = assign82500_e124004_d_n17;
        var_nqs_x0_dn18 = assign82500_e124004_d_n18;
        var_nqs_x0_dn19 = assign82500_e124004_d_n19;
        var_nqs_x0_dn20 = assign82500_e124004_d_n20;

        let assign82510_e124006: f64 = (-var_nqs_x0);
        let assign82510_e124007: f64 = (assign82510_e124006).abs();
        let assign82510_e124009: f64 = if assign82510_e124007 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard2242 = assign82510_e124009;

        *var_guard2239_slot = var_guard2239;
        *var_guard2240_slot = var_guard2240;
        *var_guard2241_slot = var_guard2241;
        *var_guard2242_slot = var_guard2242;
        *var_nqs_a_fac_slot = var_nqs_a_fac;
        *var_nqs_a_fac_dn12_slot = var_nqs_a_fac_dn12;
        *var_nqs_a_fac_dn13_slot = var_nqs_a_fac_dn13;
        *var_nqs_a_fac_dn14_slot = var_nqs_a_fac_dn14;
        *var_nqs_a_fac_dn15_slot = var_nqs_a_fac_dn15;
        *var_nqs_a_fac_dn16_slot = var_nqs_a_fac_dn16;
        *var_nqs_a_fac_dn17_slot = var_nqs_a_fac_dn17;
        *var_nqs_a_fac_dn18_slot = var_nqs_a_fac_dn18;
        *var_nqs_a_fac_dn19_slot = var_nqs_a_fac_dn19;
        *var_nqs_a_fac_dn20_slot = var_nqs_a_fac_dn20;
        *var_nqs_a_fac_dn5_slot = var_nqs_a_fac_dn5;
        *var_nqs_a_fac_dn6_slot = var_nqs_a_fac_dn6;
        *var_nqs_a_fac_dn7_slot = var_nqs_a_fac_dn7;
        *var_nqs_a_fac_dn8_slot = var_nqs_a_fac_dn8;
        *var_nqs_d0_slot = var_nqs_d0;
        *var_nqs_d0_dn12_slot = var_nqs_d0_dn12;
        *var_nqs_d0_dn13_slot = var_nqs_d0_dn13;
        *var_nqs_d0_dn14_slot = var_nqs_d0_dn14;
        *var_nqs_d0_dn15_slot = var_nqs_d0_dn15;
        *var_nqs_d0_dn16_slot = var_nqs_d0_dn16;
        *var_nqs_d0_dn17_slot = var_nqs_d0_dn17;
        *var_nqs_d0_dn18_slot = var_nqs_d0_dn18;
        *var_nqs_d0_dn19_slot = var_nqs_d0_dn19;
        *var_nqs_d0_dn20_slot = var_nqs_d0_dn20;
        *var_nqs_d0_dn5_slot = var_nqs_d0_dn5;
        *var_nqs_d0_dn6_slot = var_nqs_d0_dn6;
        *var_nqs_d0_dn7_slot = var_nqs_d0_dn7;
        *var_nqs_d0_dn8_slot = var_nqs_d0_dn8;
        *var_nqs_p_slot = var_nqs_p;
        *var_nqs_p_dn12_slot = var_nqs_p_dn12;
        *var_nqs_p_dn13_slot = var_nqs_p_dn13;
        *var_nqs_p_dn14_slot = var_nqs_p_dn14;
        *var_nqs_p_dn15_slot = var_nqs_p_dn15;
        *var_nqs_p_dn16_slot = var_nqs_p_dn16;
        *var_nqs_p_dn17_slot = var_nqs_p_dn17;
        *var_nqs_p_dn18_slot = var_nqs_p_dn18;
        *var_nqs_p_dn19_slot = var_nqs_p_dn19;
        *var_nqs_p_dn20_slot = var_nqs_p_dn20;
        *var_nqs_p_dn5_slot = var_nqs_p_dn5;
        *var_nqs_p_dn6_slot = var_nqs_p_dn6;
        *var_nqs_p_dn7_slot = var_nqs_p_dn7;
        *var_nqs_p_dn8_slot = var_nqs_p_dn8;
        *var_nqs_q_slot = var_nqs_q;
        *var_nqs_q_dn12_slot = var_nqs_q_dn12;
        *var_nqs_q_dn13_slot = var_nqs_q_dn13;
        *var_nqs_q_dn14_slot = var_nqs_q_dn14;
        *var_nqs_q_dn15_slot = var_nqs_q_dn15;
        *var_nqs_q_dn16_slot = var_nqs_q_dn16;
        *var_nqs_q_dn17_slot = var_nqs_q_dn17;
        *var_nqs_q_dn18_slot = var_nqs_q_dn18;
        *var_nqs_q_dn19_slot = var_nqs_q_dn19;
        *var_nqs_q_dn20_slot = var_nqs_q_dn20;
        *var_nqs_q_dn5_slot = var_nqs_q_dn5;
        *var_nqs_q_dn6_slot = var_nqs_q_dn6;
        *var_nqs_q_dn7_slot = var_nqs_q_dn7;
        *var_nqs_q_dn8_slot = var_nqs_q_dn8;
        *var_nqs_temp_slot = var_nqs_temp;
        *var_nqs_temp_dn12_slot = var_nqs_temp_dn12;
        *var_nqs_temp_dn13_slot = var_nqs_temp_dn13;
        *var_nqs_temp_dn14_slot = var_nqs_temp_dn14;
        *var_nqs_temp_dn15_slot = var_nqs_temp_dn15;
        *var_nqs_temp_dn16_slot = var_nqs_temp_dn16;
        *var_nqs_temp_dn17_slot = var_nqs_temp_dn17;
        *var_nqs_temp_dn18_slot = var_nqs_temp_dn18;
        *var_nqs_temp_dn19_slot = var_nqs_temp_dn19;
        *var_nqs_temp_dn20_slot = var_nqs_temp_dn20;
        *var_nqs_temp_dn5_slot = var_nqs_temp_dn5;
        *var_nqs_temp_dn6_slot = var_nqs_temp_dn6;
        *var_nqs_temp_dn7_slot = var_nqs_temp_dn7;
        *var_nqs_temp_dn8_slot = var_nqs_temp_dn8;
        *var_nqs_w_slot = var_nqs_w;
        *var_nqs_w_dn12_slot = var_nqs_w_dn12;
        *var_nqs_w_dn13_slot = var_nqs_w_dn13;
        *var_nqs_w_dn14_slot = var_nqs_w_dn14;
        *var_nqs_w_dn15_slot = var_nqs_w_dn15;
        *var_nqs_w_dn16_slot = var_nqs_w_dn16;
        *var_nqs_w_dn17_slot = var_nqs_w_dn17;
        *var_nqs_w_dn18_slot = var_nqs_w_dn18;
        *var_nqs_w_dn19_slot = var_nqs_w_dn19;
        *var_nqs_w_dn20_slot = var_nqs_w_dn20;
        *var_nqs_w_dn5_slot = var_nqs_w_dn5;
        *var_nqs_w_dn6_slot = var_nqs_w_dn6;
        *var_nqs_w_dn7_slot = var_nqs_w_dn7;
        *var_nqs_w_dn8_slot = var_nqs_w_dn8;
        *var_nqs_x0_slot = var_nqs_x0;
        *var_nqs_x0_dn12_slot = var_nqs_x0_dn12;
        *var_nqs_x0_dn13_slot = var_nqs_x0_dn13;
        *var_nqs_x0_dn14_slot = var_nqs_x0_dn14;
        *var_nqs_x0_dn15_slot = var_nqs_x0_dn15;
        *var_nqs_x0_dn16_slot = var_nqs_x0_dn16;
        *var_nqs_x0_dn17_slot = var_nqs_x0_dn17;
        *var_nqs_x0_dn18_slot = var_nqs_x0_dn18;
        *var_nqs_x0_dn19_slot = var_nqs_x0_dn19;
        *var_nqs_x0_dn20_slot = var_nqs_x0_dn20;
        *var_nqs_x0_dn5_slot = var_nqs_x0_dn5;
        *var_nqs_x0_dn6_slot = var_nqs_x0_dn6;
        *var_nqs_x0_dn7_slot = var_nqs_x0_dn7;
        *var_nqs_x0_dn8_slot = var_nqs_x0_dn8;
        *var_nqs_xbar_slot = var_nqs_xbar;
        *var_nqs_xbar_dn12_slot = var_nqs_xbar_dn12;
        *var_nqs_xbar_dn13_slot = var_nqs_xbar_dn13;
        *var_nqs_xbar_dn14_slot = var_nqs_xbar_dn14;
        *var_nqs_xbar_dn15_slot = var_nqs_xbar_dn15;
        *var_nqs_xbar_dn16_slot = var_nqs_xbar_dn16;
        *var_nqs_xbar_dn17_slot = var_nqs_xbar_dn17;
        *var_nqs_xbar_dn18_slot = var_nqs_xbar_dn18;
        *var_nqs_xbar_dn19_slot = var_nqs_xbar_dn19;
        *var_nqs_xbar_dn20_slot = var_nqs_xbar_dn20;
        *var_nqs_xbar_dn5_slot = var_nqs_xbar_dn5;
        *var_nqs_xbar_dn6_slot = var_nqs_xbar_dn6;
        *var_nqs_xbar_dn7_slot = var_nqs_xbar_dn7;
        *var_nqs_xbar_dn8_slot = var_nqs_xbar_dn8;
        *var_nqs_xg1_slot = var_nqs_xg1;
        *var_nqs_xg1_dn12_slot = var_nqs_xg1_dn12;
        *var_nqs_xg1_dn13_slot = var_nqs_xg1_dn13;
        *var_nqs_xg1_dn14_slot = var_nqs_xg1_dn14;
        *var_nqs_xg1_dn15_slot = var_nqs_xg1_dn15;
        *var_nqs_xg1_dn16_slot = var_nqs_xg1_dn16;
        *var_nqs_xg1_dn17_slot = var_nqs_xg1_dn17;
        *var_nqs_xg1_dn18_slot = var_nqs_xg1_dn18;
        *var_nqs_xg1_dn19_slot = var_nqs_xg1_dn19;
        *var_nqs_xg1_dn20_slot = var_nqs_xg1_dn20;
        *var_nqs_xg1_dn5_slot = var_nqs_xg1_dn5;
        *var_nqs_xg1_dn6_slot = var_nqs_xg1_dn6;
        *var_nqs_xg1_dn7_slot = var_nqs_xg1_dn7;
        *var_nqs_xg1_dn8_slot = var_nqs_xg1_dn8;
        *var_nqs_xi_slot = var_nqs_xi;
        *var_nqs_xi_dn12_slot = var_nqs_xi_dn12;
        *var_nqs_xi_dn13_slot = var_nqs_xi_dn13;
        *var_nqs_xi_dn14_slot = var_nqs_xi_dn14;
        *var_nqs_xi_dn15_slot = var_nqs_xi_dn15;
        *var_nqs_xi_dn16_slot = var_nqs_xi_dn16;
        *var_nqs_xi_dn17_slot = var_nqs_xi_dn17;
        *var_nqs_xi_dn18_slot = var_nqs_xi_dn18;
        *var_nqs_xi_dn19_slot = var_nqs_xi_dn19;
        *var_nqs_xi_dn20_slot = var_nqs_xi_dn20;
        *var_nqs_xi_dn5_slot = var_nqs_xi_dn5;
        *var_nqs_xi_dn6_slot = var_nqs_xi_dn6;
        *var_nqs_xi_dn7_slot = var_nqs_xi_dn7;
        *var_nqs_xi_dn8_slot = var_nqs_xi_dn8;
        *var_temp9_slot = var_temp9;
        *var_temp9_dn12_slot = var_temp9_dn12;
        *var_temp9_dn13_slot = var_temp9_dn13;
        *var_temp9_dn14_slot = var_temp9_dn14;
        *var_temp9_dn15_slot = var_temp9_dn15;
        *var_temp9_dn16_slot = var_temp9_dn16;
        *var_temp9_dn17_slot = var_temp9_dn17;
        *var_temp9_dn18_slot = var_temp9_dn18;
        *var_temp9_dn19_slot = var_temp9_dn19;
        *var_temp9_dn20_slot = var_temp9_dn20;
        *var_temp9_dn5_slot = var_temp9_dn5;
        *var_temp9_dn6_slot = var_temp9_dn6;
        *var_temp9_dn7_slot = var_temp9_dn7;
        *var_temp9_dn8_slot = var_temp9_dn8;
    }

    pub(super) fn stamp_transient_block_258(
        var_absource_i: f64,
        var_cox_qm: f64,
        var_cox_qm_dn12: f64,
        var_cox_qm_dn13: f64,
        var_cox_qm_dn14: f64,
        var_cox_qm_dn15: f64,
        var_cox_qm_dn16: f64,
        var_cox_qm_dn17: f64,
        var_cox_qm_dn18: f64,
        var_cox_qm_dn19: f64,
        var_cox_qm_dn20: f64,
        var_cox_qm_dn5: f64,
        var_cox_qm_dn6: f64,
        var_cox_qm_dn7: f64,
        var_cox_qm_dn8: f64,
        var_gp2: f64,
        var_gp2_dn12: f64,
        var_gp2_dn13: f64,
        var_gp2_dn14: f64,
        var_gp2_dn15: f64,
        var_gp2_dn16: f64,
        var_gp2_dn17: f64,
        var_gp2_dn18: f64,
        var_gp2_dn19: f64,
        var_gp2_dn20: f64,
        var_gp2_dn5: f64,
        var_gp2_dn6: f64,
        var_gp2_dn7: f64,
        var_gp2_dn8: f64,
        var_guard2078: f64,
        var_guard2079: f64,
        var_guard2088: f64,
        var_guard2105: f64,
        var_guard2130: f64,
        var_guard2171: f64,
        var_guard2236: f64,
        var_guard2237: f64,
        var_guard2242: f64,
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_nqs_x0: f64,
        var_nqs_x0_dn12: f64,
        var_nqs_x0_dn13: f64,
        var_nqs_x0_dn14: f64,
        var_nqs_x0_dn15: f64,
        var_nqs_x0_dn16: f64,
        var_nqs_x0_dn17: f64,
        var_nqs_x0_dn18: f64,
        var_nqs_x0_dn19: f64,
        var_nqs_x0_dn20: f64,
        var_nqs_x0_dn5: f64,
        var_nqs_x0_dn6: f64,
        var_nqs_x0_dn7: f64,
        var_nqs_x0_dn8: f64,
        var_pd: f64,
        var_pd_dn12: f64,
        var_pd_dn13: f64,
        var_pd_dn14: f64,
        var_pd_dn15: f64,
        var_pd_dn16: f64,
        var_pd_dn17: f64,
        var_pd_dn18: f64,
        var_pd_dn19: f64,
        var_pd_dn20: f64,
        var_pd_dn5: f64,
        var_pd_dn6: f64,
        var_pd_dn7: f64,
        var_pd_dn8: f64,
        var_phit1_ac: f64,
        var_phit1_ac_dn12: f64,
        var_phit1_ac_dn13: f64,
        var_phit1_ac_dn14: f64,
        var_phit1_ac_dn15: f64,
        var_phit1_ac_dn16: f64,
        var_phit1_ac_dn17: f64,
        var_phit1_ac_dn18: f64,
        var_phit1_ac_dn19: f64,
        var_phit1_ac_dn20: f64,
        var_phit1_ac_dn5: f64,
        var_phit1_ac_dn6: f64,
        var_phit1_ac_dn7: f64,
        var_phit1_ac_dn8: f64,
        var_qd_nqs: f64,
        var_qd_nqs_dn12: f64,
        var_qd_nqs_dn13: f64,
        var_qd_nqs_dn14: f64,
        var_qd_nqs_dn15: f64,
        var_qd_nqs_dn16: f64,
        var_qd_nqs_dn17: f64,
        var_qd_nqs_dn18: f64,
        var_qd_nqs_dn19: f64,
        var_qd_nqs_dn20: f64,
        var_qd_nqs_dn5: f64,
        var_qd_nqs_dn6: f64,
        var_qd_nqs_dn7: f64,
        var_qd_nqs_dn8: f64,
        var_qgd_ov: f64,
        var_qgd_ov_dn5: f64,
        var_qgd_ov_dn6: f64,
        var_qgd_ov_dn7: f64,
        var_qgs_ov: f64,
        var_qgs_ov_dn5: f64,
        var_qgs_ov_dn6: f64,
        var_qgs_ov_dn7: f64,
        var_qjunbot_s: f64,
        var_qjunbot_s_dn10: f64,
        var_qjunbot_s_dn11: f64,
        var_qjunbot_s_dn5: f64,
        var_qjunbot_s_dn6: f64,
        var_qjunbot_s_dn7: f64,
        var_qjunbot_s_dn8: f64,
        var_qjungat_s: f64,
        var_qjungat_s_dn10: f64,
        var_qjungat_s_dn11: f64,
        var_qjungat_s_dn5: f64,
        var_qjungat_s_dn6: f64,
        var_qjungat_s_dn7: f64,
        var_qjungat_s_dn8: f64,
        var_qjunsti_s: f64,
        var_qjunsti_s_dn10: f64,
        var_qjunsti_s_dn11: f64,
        var_qjunsti_s_dn5: f64,
        var_qjunsti_s_dn6: f64,
        var_qjunsti_s_dn7: f64,
        var_qjunsti_s_dn8: f64,
        var_qs_nqs: f64,
        var_qs_nqs_dn12: f64,
        var_qs_nqs_dn13: f64,
        var_qs_nqs_dn14: f64,
        var_qs_nqs_dn15: f64,
        var_qs_nqs_dn16: f64,
        var_qs_nqs_dn17: f64,
        var_qs_nqs_dn18: f64,
        var_qs_nqs_dn19: f64,
        var_qs_nqs_dn20: f64,
        var_qs_nqs_dn5: f64,
        var_qs_nqs_dn6: f64,
        var_qs_nqs_dn7: f64,
        var_qs_nqs_dn8: f64,
        var_sigvds: f64,
        var_temp1: f64,
        var_temp1_dn12: f64,
        var_temp1_dn13: f64,
        var_temp1_dn14: f64,
        var_temp1_dn15: f64,
        var_temp1_dn16: f64,
        var_temp1_dn17: f64,
        var_temp1_dn18: f64,
        var_temp1_dn19: f64,
        var_temp1_dn20: f64,
        var_temp1_dn5: f64,
        var_temp1_dn6: f64,
        var_temp1_dn7: f64,
        var_temp1_dn8: f64,
        var_temp2: f64,
        var_temp2_dn12: f64,
        var_temp2_dn13: f64,
        var_temp2_dn14: f64,
        var_temp2_dn15: f64,
        var_temp2_dn16: f64,
        var_temp2_dn17: f64,
        var_temp2_dn18: f64,
        var_temp2_dn19: f64,
        var_temp2_dn20: f64,
        var_temp2_dn5: f64,
        var_temp2_dn6: f64,
        var_temp2_dn7: f64,
        var_temp2_dn8: f64,
        var_temp3: f64,
        var_temp3_dn12: f64,
        var_temp3_dn13: f64,
        var_temp3_dn14: f64,
        var_temp3_dn15: f64,
        var_temp3_dn16: f64,
        var_temp3_dn17: f64,
        var_temp3_dn18: f64,
        var_temp3_dn19: f64,
        var_temp3_dn20: f64,
        var_temp3_dn5: f64,
        var_temp3_dn6: f64,
        var_temp3_dn7: f64,
        var_temp3_dn8: f64,
        var_temp4: f64,
        var_temp4_dn12: f64,
        var_temp4_dn13: f64,
        var_temp4_dn14: f64,
        var_temp4_dn15: f64,
        var_temp4_dn16: f64,
        var_temp4_dn17: f64,
        var_temp4_dn18: f64,
        var_temp4_dn19: f64,
        var_temp4_dn20: f64,
        var_temp4_dn5: f64,
        var_temp4_dn6: f64,
        var_temp4_dn7: f64,
        var_temp4_dn8: f64,
        var_temp5: f64,
        var_temp5_dn12: f64,
        var_temp5_dn13: f64,
        var_temp5_dn14: f64,
        var_temp5_dn15: f64,
        var_temp5_dn16: f64,
        var_temp5_dn17: f64,
        var_temp5_dn18: f64,
        var_temp5_dn19: f64,
        var_temp5_dn20: f64,
        var_temp5_dn5: f64,
        var_temp5_dn6: f64,
        var_temp5_dn7: f64,
        var_temp5_dn8: f64,
        var_temp6: f64,
        var_temp6_dn12: f64,
        var_temp6_dn13: f64,
        var_temp6_dn14: f64,
        var_temp6_dn15: f64,
        var_temp6_dn16: f64,
        var_temp6_dn17: f64,
        var_temp6_dn18: f64,
        var_temp6_dn19: f64,
        var_temp6_dn20: f64,
        var_temp6_dn5: f64,
        var_temp6_dn6: f64,
        var_temp6_dn7: f64,
        var_temp6_dn8: f64,
        var_temp7: f64,
        var_temp7_dn12: f64,
        var_temp7_dn13: f64,
        var_temp7_dn14: f64,
        var_temp7_dn15: f64,
        var_temp7_dn16: f64,
        var_temp7_dn17: f64,
        var_temp7_dn18: f64,
        var_temp7_dn19: f64,
        var_temp7_dn20: f64,
        var_temp7_dn5: f64,
        var_temp7_dn6: f64,
        var_temp7_dn7: f64,
        var_temp7_dn8: f64,
        var_temp8: f64,
        var_temp8_dn12: f64,
        var_temp8_dn13: f64,
        var_temp8_dn14: f64,
        var_temp8_dn15: f64,
        var_temp8_dn16: f64,
        var_temp8_dn17: f64,
        var_temp8_dn18: f64,
        var_temp8_dn19: f64,
        var_temp8_dn20: f64,
        var_temp8_dn5: f64,
        var_temp8_dn6: f64,
        var_temp8_dn7: f64,
        var_temp8_dn8: f64,
        var_temp__blk1038: f64,
        var_temp__blk1038_dn12: f64,
        var_temp__blk1038_dn13: f64,
        var_temp__blk1038_dn14: f64,
        var_temp__blk1038_dn15: f64,
        var_temp__blk1038_dn16: f64,
        var_temp__blk1038_dn17: f64,
        var_temp__blk1038_dn18: f64,
        var_temp__blk1038_dn19: f64,
        var_temp__blk1038_dn20: f64,
        var_temp__blk1038_dn5: f64,
        var_temp__blk1038_dn6: f64,
        var_temp__blk1038_dn7: f64,
        var_temp__blk1038_dn8: f64,
        var_x_dp: f64,
        var_x_dp_dn12: f64,
        var_x_dp_dn13: f64,
        var_x_dp_dn14: f64,
        var_x_dp_dn15: f64,
        var_x_dp_dn16: f64,
        var_x_dp_dn17: f64,
        var_x_dp_dn18: f64,
        var_x_dp_dn19: f64,
        var_x_dp_dn20: f64,
        var_x_dp_dn5: f64,
        var_x_dp_dn6: f64,
        var_x_dp_dn7: f64,
        var_x_dp_dn8: f64,
        var_x_sp: f64,
        var_x_sp_dn12: f64,
        var_x_sp_dn13: f64,
        var_x_sp_dn14: f64,
        var_x_sp_dn15: f64,
        var_x_sp_dn16: f64,
        var_x_sp_dn17: f64,
        var_x_sp_dn18: f64,
        var_x_sp_dn19: f64,
        var_x_sp_dn20: f64,
        var_x_sp_dn5: f64,
        var_x_sp_dn6: f64,
        var_x_sp_dn7: f64,
        var_x_sp_dn8: f64,
        var_xg_ac: f64,
        var_xg_ac_dn12: f64,
        var_xg_ac_dn13: f64,
        var_xg_ac_dn14: f64,
        var_xg_ac_dn15: f64,
        var_xg_ac_dn16: f64,
        var_xg_ac_dn17: f64,
        var_xg_ac_dn18: f64,
        var_xg_ac_dn19: f64,
        var_xg_ac_dn20: f64,
        var_xg_ac_dn5: f64,
        var_xg_ac_dn6: f64,
        var_xg_ac_dn7: f64,
        var_xg_ac_dn8: f64,
        var_guard2243_slot: &mut f64,
        var_guard2244_slot: &mut f64,
        var_nqs_d0_slot: &mut f64,
        var_nqs_d0_dn12_slot: &mut f64,
        var_nqs_d0_dn13_slot: &mut f64,
        var_nqs_d0_dn14_slot: &mut f64,
        var_nqs_d0_dn15_slot: &mut f64,
        var_nqs_d0_dn16_slot: &mut f64,
        var_nqs_d0_dn17_slot: &mut f64,
        var_nqs_d0_dn18_slot: &mut f64,
        var_nqs_d0_dn19_slot: &mut f64,
        var_nqs_d0_dn20_slot: &mut f64,
        var_nqs_d0_dn5_slot: &mut f64,
        var_nqs_d0_dn6_slot: &mut f64,
        var_nqs_d0_dn7_slot: &mut f64,
        var_nqs_d0_dn8_slot: &mut f64,
        var_nqs_p_slot: &mut f64,
        var_nqs_p_dn12_slot: &mut f64,
        var_nqs_p_dn13_slot: &mut f64,
        var_nqs_p_dn14_slot: &mut f64,
        var_nqs_p_dn15_slot: &mut f64,
        var_nqs_p_dn16_slot: &mut f64,
        var_nqs_p_dn17_slot: &mut f64,
        var_nqs_p_dn18_slot: &mut f64,
        var_nqs_p_dn19_slot: &mut f64,
        var_nqs_p_dn20_slot: &mut f64,
        var_nqs_p_dn5_slot: &mut f64,
        var_nqs_p_dn6_slot: &mut f64,
        var_nqs_p_dn7_slot: &mut f64,
        var_nqs_p_dn8_slot: &mut f64,
        var_nqs_q_slot: &mut f64,
        var_nqs_q_dn12_slot: &mut f64,
        var_nqs_q_dn13_slot: &mut f64,
        var_nqs_q_dn14_slot: &mut f64,
        var_nqs_q_dn15_slot: &mut f64,
        var_nqs_q_dn16_slot: &mut f64,
        var_nqs_q_dn17_slot: &mut f64,
        var_nqs_q_dn18_slot: &mut f64,
        var_nqs_q_dn19_slot: &mut f64,
        var_nqs_q_dn20_slot: &mut f64,
        var_nqs_q_dn5_slot: &mut f64,
        var_nqs_q_dn6_slot: &mut f64,
        var_nqs_q_dn7_slot: &mut f64,
        var_nqs_q_dn8_slot: &mut f64,
        var_nqs_temp_slot: &mut f64,
        var_nqs_temp_dn12_slot: &mut f64,
        var_nqs_temp_dn13_slot: &mut f64,
        var_nqs_temp_dn14_slot: &mut f64,
        var_nqs_temp_dn15_slot: &mut f64,
        var_nqs_temp_dn16_slot: &mut f64,
        var_nqs_temp_dn17_slot: &mut f64,
        var_nqs_temp_dn18_slot: &mut f64,
        var_nqs_temp_dn19_slot: &mut f64,
        var_nqs_temp_dn20_slot: &mut f64,
        var_nqs_temp_dn5_slot: &mut f64,
        var_nqs_temp_dn6_slot: &mut f64,
        var_nqs_temp_dn7_slot: &mut f64,
        var_nqs_temp_dn8_slot: &mut f64,
        var_nqs_u_slot: &mut f64,
        var_nqs_u_dn12_slot: &mut f64,
        var_nqs_u_dn13_slot: &mut f64,
        var_nqs_u_dn14_slot: &mut f64,
        var_nqs_u_dn15_slot: &mut f64,
        var_nqs_u_dn16_slot: &mut f64,
        var_nqs_u_dn17_slot: &mut f64,
        var_nqs_u_dn18_slot: &mut f64,
        var_nqs_u_dn19_slot: &mut f64,
        var_nqs_u_dn20_slot: &mut f64,
        var_nqs_u_dn5_slot: &mut f64,
        var_nqs_u_dn6_slot: &mut f64,
        var_nqs_u_dn7_slot: &mut f64,
        var_nqs_u_dn8_slot: &mut f64,
        var_nqs_xi_slot: &mut f64,
        var_nqs_xi_dn12_slot: &mut f64,
        var_nqs_xi_dn13_slot: &mut f64,
        var_nqs_xi_dn14_slot: &mut f64,
        var_nqs_xi_dn15_slot: &mut f64,
        var_nqs_xi_dn16_slot: &mut f64,
        var_nqs_xi_dn17_slot: &mut f64,
        var_nqs_xi_dn18_slot: &mut f64,
        var_nqs_xi_dn19_slot: &mut f64,
        var_nqs_xi_dn20_slot: &mut f64,
        var_nqs_xi_dn5_slot: &mut f64,
        var_nqs_xi_dn6_slot: &mut f64,
        var_nqs_xi_dn7_slot: &mut f64,
        var_nqs_xi_dn8_slot: &mut f64,
        var_qb_slot: &mut f64,
        var_qb_dn12_slot: &mut f64,
        var_qb_dn13_slot: &mut f64,
        var_qb_dn14_slot: &mut f64,
        var_qb_dn15_slot: &mut f64,
        var_qb_dn16_slot: &mut f64,
        var_qb_dn17_slot: &mut f64,
        var_qb_dn18_slot: &mut f64,
        var_qb_dn19_slot: &mut f64,
        var_qb_dn20_slot: &mut f64,
        var_qb_dn5_slot: &mut f64,
        var_qb_dn6_slot: &mut f64,
        var_qb_dn7_slot: &mut f64,
        var_qb_dn8_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn12_slot: &mut f64,
        var_qd_dn13_slot: &mut f64,
        var_qd_dn14_slot: &mut f64,
        var_qd_dn15_slot: &mut f64,
        var_qd_dn16_slot: &mut f64,
        var_qd_dn17_slot: &mut f64,
        var_qd_dn18_slot: &mut f64,
        var_qd_dn19_slot: &mut f64,
        var_qd_dn20_slot: &mut f64,
        var_qd_dn5_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qfgd_slot: &mut f64,
        var_qfgd_dn5_slot: &mut f64,
        var_qfgd_dn6_slot: &mut f64,
        var_qfgd_dn7_slot: &mut f64,
        var_qfgs_slot: &mut f64,
        var_qfgs_dn5_slot: &mut f64,
        var_qfgs_dn6_slot: &mut f64,
        var_qfgs_dn7_slot: &mut f64,
        var_qg_slot: &mut f64,
        var_qg_dn12_slot: &mut f64,
        var_qg_dn13_slot: &mut f64,
        var_qg_dn14_slot: &mut f64,
        var_qg_dn15_slot: &mut f64,
        var_qg_dn16_slot: &mut f64,
        var_qg_dn17_slot: &mut f64,
        var_qg_dn18_slot: &mut f64,
        var_qg_dn19_slot: &mut f64,
        var_qg_dn20_slot: &mut f64,
        var_qg_dn5_slot: &mut f64,
        var_qg_dn6_slot: &mut f64,
        var_qg_dn7_slot: &mut f64,
        var_qg_dn8_slot: &mut f64,
        var_qg_nqs_slot: &mut f64,
        var_qg_nqs_dn12_slot: &mut f64,
        var_qg_nqs_dn13_slot: &mut f64,
        var_qg_nqs_dn14_slot: &mut f64,
        var_qg_nqs_dn15_slot: &mut f64,
        var_qg_nqs_dn16_slot: &mut f64,
        var_qg_nqs_dn17_slot: &mut f64,
        var_qg_nqs_dn18_slot: &mut f64,
        var_qg_nqs_dn19_slot: &mut f64,
        var_qg_nqs_dn20_slot: &mut f64,
        var_qg_nqs_dn5_slot: &mut f64,
        var_qg_nqs_dn6_slot: &mut f64,
        var_qg_nqs_dn7_slot: &mut f64,
        var_qg_nqs_dn8_slot: &mut f64,
        var_qjun_s_slot: &mut f64,
        var_qjun_s_dn10_slot: &mut f64,
        var_qjun_s_dn11_slot: &mut f64,
        var_qjun_s_dn5_slot: &mut f64,
        var_qjun_s_dn6_slot: &mut f64,
        var_qjun_s_dn7_slot: &mut f64,
        var_qjun_s_dn8_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_dn12_slot: &mut f64,
        var_qs_dn13_slot: &mut f64,
        var_qs_dn14_slot: &mut f64,
        var_qs_dn15_slot: &mut f64,
        var_qs_dn16_slot: &mut f64,
        var_qs_dn17_slot: &mut f64,
        var_qs_dn18_slot: &mut f64,
        var_qs_dn19_slot: &mut f64,
        var_qs_dn20_slot: &mut f64,
        var_qs_dn5_slot: &mut f64,
        var_qs_dn6_slot: &mut f64,
        var_qs_dn7_slot: &mut f64,
        var_qs_dn8_slot: &mut f64,
        var_temp9_slot: &mut f64,
        var_temp9_dn12_slot: &mut f64,
        var_temp9_dn13_slot: &mut f64,
        var_temp9_dn14_slot: &mut f64,
        var_temp9_dn15_slot: &mut f64,
        var_temp9_dn16_slot: &mut f64,
        var_temp9_dn17_slot: &mut f64,
        var_temp9_dn18_slot: &mut f64,
        var_temp9_dn19_slot: &mut f64,
        var_temp9_dn20_slot: &mut f64,
        var_temp9_dn5_slot: &mut f64,
        var_temp9_dn6_slot: &mut f64,
        var_temp9_dn7_slot: &mut f64,
        var_temp9_dn8_slot: &mut f64,
    ) {
        let mut var_guard2243: f64 = *var_guard2243_slot;
        let mut var_guard2244: f64 = *var_guard2244_slot;
        let mut var_nqs_d0: f64 = *var_nqs_d0_slot;
        let mut var_nqs_d0_dn12: f64 = *var_nqs_d0_dn12_slot;
        let mut var_nqs_d0_dn13: f64 = *var_nqs_d0_dn13_slot;
        let mut var_nqs_d0_dn14: f64 = *var_nqs_d0_dn14_slot;
        let mut var_nqs_d0_dn15: f64 = *var_nqs_d0_dn15_slot;
        let mut var_nqs_d0_dn16: f64 = *var_nqs_d0_dn16_slot;
        let mut var_nqs_d0_dn17: f64 = *var_nqs_d0_dn17_slot;
        let mut var_nqs_d0_dn18: f64 = *var_nqs_d0_dn18_slot;
        let mut var_nqs_d0_dn19: f64 = *var_nqs_d0_dn19_slot;
        let mut var_nqs_d0_dn20: f64 = *var_nqs_d0_dn20_slot;
        let mut var_nqs_d0_dn5: f64 = *var_nqs_d0_dn5_slot;
        let mut var_nqs_d0_dn6: f64 = *var_nqs_d0_dn6_slot;
        let mut var_nqs_d0_dn7: f64 = *var_nqs_d0_dn7_slot;
        let mut var_nqs_d0_dn8: f64 = *var_nqs_d0_dn8_slot;
        let mut var_nqs_p: f64 = *var_nqs_p_slot;
        let mut var_nqs_p_dn12: f64 = *var_nqs_p_dn12_slot;
        let mut var_nqs_p_dn13: f64 = *var_nqs_p_dn13_slot;
        let mut var_nqs_p_dn14: f64 = *var_nqs_p_dn14_slot;
        let mut var_nqs_p_dn15: f64 = *var_nqs_p_dn15_slot;
        let mut var_nqs_p_dn16: f64 = *var_nqs_p_dn16_slot;
        let mut var_nqs_p_dn17: f64 = *var_nqs_p_dn17_slot;
        let mut var_nqs_p_dn18: f64 = *var_nqs_p_dn18_slot;
        let mut var_nqs_p_dn19: f64 = *var_nqs_p_dn19_slot;
        let mut var_nqs_p_dn20: f64 = *var_nqs_p_dn20_slot;
        let mut var_nqs_p_dn5: f64 = *var_nqs_p_dn5_slot;
        let mut var_nqs_p_dn6: f64 = *var_nqs_p_dn6_slot;
        let mut var_nqs_p_dn7: f64 = *var_nqs_p_dn7_slot;
        let mut var_nqs_p_dn8: f64 = *var_nqs_p_dn8_slot;
        let mut var_nqs_q: f64 = *var_nqs_q_slot;
        let mut var_nqs_q_dn12: f64 = *var_nqs_q_dn12_slot;
        let mut var_nqs_q_dn13: f64 = *var_nqs_q_dn13_slot;
        let mut var_nqs_q_dn14: f64 = *var_nqs_q_dn14_slot;
        let mut var_nqs_q_dn15: f64 = *var_nqs_q_dn15_slot;
        let mut var_nqs_q_dn16: f64 = *var_nqs_q_dn16_slot;
        let mut var_nqs_q_dn17: f64 = *var_nqs_q_dn17_slot;
        let mut var_nqs_q_dn18: f64 = *var_nqs_q_dn18_slot;
        let mut var_nqs_q_dn19: f64 = *var_nqs_q_dn19_slot;
        let mut var_nqs_q_dn20: f64 = *var_nqs_q_dn20_slot;
        let mut var_nqs_q_dn5: f64 = *var_nqs_q_dn5_slot;
        let mut var_nqs_q_dn6: f64 = *var_nqs_q_dn6_slot;
        let mut var_nqs_q_dn7: f64 = *var_nqs_q_dn7_slot;
        let mut var_nqs_q_dn8: f64 = *var_nqs_q_dn8_slot;
        let mut var_nqs_temp: f64 = *var_nqs_temp_slot;
        let mut var_nqs_temp_dn12: f64 = *var_nqs_temp_dn12_slot;
        let mut var_nqs_temp_dn13: f64 = *var_nqs_temp_dn13_slot;
        let mut var_nqs_temp_dn14: f64 = *var_nqs_temp_dn14_slot;
        let mut var_nqs_temp_dn15: f64 = *var_nqs_temp_dn15_slot;
        let mut var_nqs_temp_dn16: f64 = *var_nqs_temp_dn16_slot;
        let mut var_nqs_temp_dn17: f64 = *var_nqs_temp_dn17_slot;
        let mut var_nqs_temp_dn18: f64 = *var_nqs_temp_dn18_slot;
        let mut var_nqs_temp_dn19: f64 = *var_nqs_temp_dn19_slot;
        let mut var_nqs_temp_dn20: f64 = *var_nqs_temp_dn20_slot;
        let mut var_nqs_temp_dn5: f64 = *var_nqs_temp_dn5_slot;
        let mut var_nqs_temp_dn6: f64 = *var_nqs_temp_dn6_slot;
        let mut var_nqs_temp_dn7: f64 = *var_nqs_temp_dn7_slot;
        let mut var_nqs_temp_dn8: f64 = *var_nqs_temp_dn8_slot;
        let mut var_nqs_u: f64 = *var_nqs_u_slot;
        let mut var_nqs_u_dn12: f64 = *var_nqs_u_dn12_slot;
        let mut var_nqs_u_dn13: f64 = *var_nqs_u_dn13_slot;
        let mut var_nqs_u_dn14: f64 = *var_nqs_u_dn14_slot;
        let mut var_nqs_u_dn15: f64 = *var_nqs_u_dn15_slot;
        let mut var_nqs_u_dn16: f64 = *var_nqs_u_dn16_slot;
        let mut var_nqs_u_dn17: f64 = *var_nqs_u_dn17_slot;
        let mut var_nqs_u_dn18: f64 = *var_nqs_u_dn18_slot;
        let mut var_nqs_u_dn19: f64 = *var_nqs_u_dn19_slot;
        let mut var_nqs_u_dn20: f64 = *var_nqs_u_dn20_slot;
        let mut var_nqs_u_dn5: f64 = *var_nqs_u_dn5_slot;
        let mut var_nqs_u_dn6: f64 = *var_nqs_u_dn6_slot;
        let mut var_nqs_u_dn7: f64 = *var_nqs_u_dn7_slot;
        let mut var_nqs_u_dn8: f64 = *var_nqs_u_dn8_slot;
        let mut var_nqs_xi: f64 = *var_nqs_xi_slot;
        let mut var_nqs_xi_dn12: f64 = *var_nqs_xi_dn12_slot;
        let mut var_nqs_xi_dn13: f64 = *var_nqs_xi_dn13_slot;
        let mut var_nqs_xi_dn14: f64 = *var_nqs_xi_dn14_slot;
        let mut var_nqs_xi_dn15: f64 = *var_nqs_xi_dn15_slot;
        let mut var_nqs_xi_dn16: f64 = *var_nqs_xi_dn16_slot;
        let mut var_nqs_xi_dn17: f64 = *var_nqs_xi_dn17_slot;
        let mut var_nqs_xi_dn18: f64 = *var_nqs_xi_dn18_slot;
        let mut var_nqs_xi_dn19: f64 = *var_nqs_xi_dn19_slot;
        let mut var_nqs_xi_dn20: f64 = *var_nqs_xi_dn20_slot;
        let mut var_nqs_xi_dn5: f64 = *var_nqs_xi_dn5_slot;
        let mut var_nqs_xi_dn6: f64 = *var_nqs_xi_dn6_slot;
        let mut var_nqs_xi_dn7: f64 = *var_nqs_xi_dn7_slot;
        let mut var_nqs_xi_dn8: f64 = *var_nqs_xi_dn8_slot;
        let mut var_qb: f64 = *var_qb_slot;
        let mut var_qb_dn12: f64 = *var_qb_dn12_slot;
        let mut var_qb_dn13: f64 = *var_qb_dn13_slot;
        let mut var_qb_dn14: f64 = *var_qb_dn14_slot;
        let mut var_qb_dn15: f64 = *var_qb_dn15_slot;
        let mut var_qb_dn16: f64 = *var_qb_dn16_slot;
        let mut var_qb_dn17: f64 = *var_qb_dn17_slot;
        let mut var_qb_dn18: f64 = *var_qb_dn18_slot;
        let mut var_qb_dn19: f64 = *var_qb_dn19_slot;
        let mut var_qb_dn20: f64 = *var_qb_dn20_slot;
        let mut var_qb_dn5: f64 = *var_qb_dn5_slot;
        let mut var_qb_dn6: f64 = *var_qb_dn6_slot;
        let mut var_qb_dn7: f64 = *var_qb_dn7_slot;
        let mut var_qb_dn8: f64 = *var_qb_dn8_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn12: f64 = *var_qd_dn12_slot;
        let mut var_qd_dn13: f64 = *var_qd_dn13_slot;
        let mut var_qd_dn14: f64 = *var_qd_dn14_slot;
        let mut var_qd_dn15: f64 = *var_qd_dn15_slot;
        let mut var_qd_dn16: f64 = *var_qd_dn16_slot;
        let mut var_qd_dn17: f64 = *var_qd_dn17_slot;
        let mut var_qd_dn18: f64 = *var_qd_dn18_slot;
        let mut var_qd_dn19: f64 = *var_qd_dn19_slot;
        let mut var_qd_dn20: f64 = *var_qd_dn20_slot;
        let mut var_qd_dn5: f64 = *var_qd_dn5_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qfgd: f64 = *var_qfgd_slot;
        let mut var_qfgd_dn5: f64 = *var_qfgd_dn5_slot;
        let mut var_qfgd_dn6: f64 = *var_qfgd_dn6_slot;
        let mut var_qfgd_dn7: f64 = *var_qfgd_dn7_slot;
        let mut var_qfgs: f64 = *var_qfgs_slot;
        let mut var_qfgs_dn5: f64 = *var_qfgs_dn5_slot;
        let mut var_qfgs_dn6: f64 = *var_qfgs_dn6_slot;
        let mut var_qfgs_dn7: f64 = *var_qfgs_dn7_slot;
        let mut var_qg: f64 = *var_qg_slot;
        let mut var_qg_dn12: f64 = *var_qg_dn12_slot;
        let mut var_qg_dn13: f64 = *var_qg_dn13_slot;
        let mut var_qg_dn14: f64 = *var_qg_dn14_slot;
        let mut var_qg_dn15: f64 = *var_qg_dn15_slot;
        let mut var_qg_dn16: f64 = *var_qg_dn16_slot;
        let mut var_qg_dn17: f64 = *var_qg_dn17_slot;
        let mut var_qg_dn18: f64 = *var_qg_dn18_slot;
        let mut var_qg_dn19: f64 = *var_qg_dn19_slot;
        let mut var_qg_dn20: f64 = *var_qg_dn20_slot;
        let mut var_qg_dn5: f64 = *var_qg_dn5_slot;
        let mut var_qg_dn6: f64 = *var_qg_dn6_slot;
        let mut var_qg_dn7: f64 = *var_qg_dn7_slot;
        let mut var_qg_dn8: f64 = *var_qg_dn8_slot;
        let mut var_qg_nqs: f64 = *var_qg_nqs_slot;
        let mut var_qg_nqs_dn12: f64 = *var_qg_nqs_dn12_slot;
        let mut var_qg_nqs_dn13: f64 = *var_qg_nqs_dn13_slot;
        let mut var_qg_nqs_dn14: f64 = *var_qg_nqs_dn14_slot;
        let mut var_qg_nqs_dn15: f64 = *var_qg_nqs_dn15_slot;
        let mut var_qg_nqs_dn16: f64 = *var_qg_nqs_dn16_slot;
        let mut var_qg_nqs_dn17: f64 = *var_qg_nqs_dn17_slot;
        let mut var_qg_nqs_dn18: f64 = *var_qg_nqs_dn18_slot;
        let mut var_qg_nqs_dn19: f64 = *var_qg_nqs_dn19_slot;
        let mut var_qg_nqs_dn20: f64 = *var_qg_nqs_dn20_slot;
        let mut var_qg_nqs_dn5: f64 = *var_qg_nqs_dn5_slot;
        let mut var_qg_nqs_dn6: f64 = *var_qg_nqs_dn6_slot;
        let mut var_qg_nqs_dn7: f64 = *var_qg_nqs_dn7_slot;
        let mut var_qg_nqs_dn8: f64 = *var_qg_nqs_dn8_slot;
        let mut var_qjun_s: f64 = *var_qjun_s_slot;
        let mut var_qjun_s_dn10: f64 = *var_qjun_s_dn10_slot;
        let mut var_qjun_s_dn11: f64 = *var_qjun_s_dn11_slot;
        let mut var_qjun_s_dn5: f64 = *var_qjun_s_dn5_slot;
        let mut var_qjun_s_dn6: f64 = *var_qjun_s_dn6_slot;
        let mut var_qjun_s_dn7: f64 = *var_qjun_s_dn7_slot;
        let mut var_qjun_s_dn8: f64 = *var_qjun_s_dn8_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_dn12: f64 = *var_qs_dn12_slot;
        let mut var_qs_dn13: f64 = *var_qs_dn13_slot;
        let mut var_qs_dn14: f64 = *var_qs_dn14_slot;
        let mut var_qs_dn15: f64 = *var_qs_dn15_slot;
        let mut var_qs_dn16: f64 = *var_qs_dn16_slot;
        let mut var_qs_dn17: f64 = *var_qs_dn17_slot;
        let mut var_qs_dn18: f64 = *var_qs_dn18_slot;
        let mut var_qs_dn19: f64 = *var_qs_dn19_slot;
        let mut var_qs_dn20: f64 = *var_qs_dn20_slot;
        let mut var_qs_dn5: f64 = *var_qs_dn5_slot;
        let mut var_qs_dn6: f64 = *var_qs_dn6_slot;
        let mut var_qs_dn7: f64 = *var_qs_dn7_slot;
        let mut var_qs_dn8: f64 = *var_qs_dn8_slot;
        let mut var_temp9: f64 = *var_temp9_slot;
        let mut var_temp9_dn12: f64 = *var_temp9_dn12_slot;
        let mut var_temp9_dn13: f64 = *var_temp9_dn13_slot;
        let mut var_temp9_dn14: f64 = *var_temp9_dn14_slot;
        let mut var_temp9_dn15: f64 = *var_temp9_dn15_slot;
        let mut var_temp9_dn16: f64 = *var_temp9_dn16_slot;
        let mut var_temp9_dn17: f64 = *var_temp9_dn17_slot;
        let mut var_temp9_dn18: f64 = *var_temp9_dn18_slot;
        let mut var_temp9_dn19: f64 = *var_temp9_dn19_slot;
        let mut var_temp9_dn20: f64 = *var_temp9_dn20_slot;
        let mut var_temp9_dn5: f64 = *var_temp9_dn5_slot;
        let mut var_temp9_dn6: f64 = *var_temp9_dn6_slot;
        let mut var_temp9_dn7: f64 = *var_temp9_dn7_slot;
        let mut var_temp9_dn8: f64 = *var_temp9_dn8_slot;

        let (assign82520_e124037, assign82520_e124037_d_n5, assign82520_e124037_d_n6, assign82520_e124037_d_n7, assign82520_e124037_d_n8, assign82520_e124037_d_n12, assign82520_e124037_d_n13, assign82520_e124037_d_n14, assign82520_e124037_d_n15, assign82520_e124037_d_n16, assign82520_e124037_d_n17, assign82520_e124037_d_n18, assign82520_e124037_d_n19, assign82520_e124037_d_n20,) = {
    if (((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) && (var_guard2242 != 0.0)) {
        let assign82520_e124034: f64 = (-var_nqs_x0);
        let assign82520_e124035: f64 = (assign82520_e124034).exp();
        (assign82520_e124035, (assign82520_e124035 * (-var_nqs_x0_dn5)), (assign82520_e124035 * (-var_nqs_x0_dn6)), (assign82520_e124035 * (-var_nqs_x0_dn7)), (assign82520_e124035 * (-var_nqs_x0_dn8)), (assign82520_e124035 * (-var_nqs_x0_dn12)), (assign82520_e124035 * (-var_nqs_x0_dn13)), (assign82520_e124035 * (-var_nqs_x0_dn14)), (assign82520_e124035 * (-var_nqs_x0_dn15)), (assign82520_e124035 * (-var_nqs_x0_dn16)), (assign82520_e124035 * (-var_nqs_x0_dn17)), (assign82520_e124035 * (-var_nqs_x0_dn18)), (assign82520_e124035 * (-var_nqs_x0_dn19)), (assign82520_e124035 * (-var_nqs_x0_dn20)),)
    } else {
        (var_nqs_d0, var_nqs_d0_dn5, var_nqs_d0_dn6, var_nqs_d0_dn7, var_nqs_d0_dn8, var_nqs_d0_dn12, var_nqs_d0_dn13, var_nqs_d0_dn14, var_nqs_d0_dn15, var_nqs_d0_dn16, var_nqs_d0_dn17, var_nqs_d0_dn18, var_nqs_d0_dn19, var_nqs_d0_dn20,)
    }
};
        var_nqs_d0 = assign82520_e124037;
        var_nqs_d0_dn5 = assign82520_e124037_d_n5;
        var_nqs_d0_dn6 = assign82520_e124037_d_n6;
        var_nqs_d0_dn7 = assign82520_e124037_d_n7;
        var_nqs_d0_dn8 = assign82520_e124037_d_n8;
        var_nqs_d0_dn12 = assign82520_e124037_d_n12;
        var_nqs_d0_dn13 = assign82520_e124037_d_n13;
        var_nqs_d0_dn14 = assign82520_e124037_d_n14;
        var_nqs_d0_dn15 = assign82520_e124037_d_n15;
        var_nqs_d0_dn16 = assign82520_e124037_d_n16;
        var_nqs_d0_dn17 = assign82520_e124037_d_n17;
        var_nqs_d0_dn18 = assign82520_e124037_d_n18;
        var_nqs_d0_dn19 = assign82520_e124037_d_n19;
        var_nqs_d0_dn20 = assign82520_e124037_d_n20;

        let assign82530_e124039: f64 = (-var_nqs_x0);
        let assign82530_e124041: f64 = if assign82530_e124039 < 0.0 { 1.0 } else { 0.0 };
        var_guard2243 = assign82530_e124041;

        let (assign82540_e124098, assign82540_e124098_d_n5, assign82540_e124098_d_n6, assign82540_e124098_d_n7, assign82540_e124098_d_n8, assign82540_e124098_d_n12, assign82540_e124098_d_n13, assign82540_e124098_d_n14, assign82540_e124098_d_n15, assign82540_e124098_d_n16, assign82540_e124098_d_n17, assign82540_e124098_d_n18, assign82540_e124098_d_n19, assign82540_e124098_d_n20,) = {
    if ((((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) && (var_guard2242 == 0.0)) && (var_guard2243 != 0.0)) {
        let assign82540_e124071: f64 = (-230.25850929940458);
        let assign82540_e124073: f64 = (-var_nqs_x0);
        let assign82540_e124074: f64 = (assign82540_e124071 - assign82540_e124073);
        let assign82540_e124078: f64 = (-230.25850929940458);
        let assign82540_e124080: f64 = (-var_nqs_x0);
        let assign82540_e124081: f64 = (assign82540_e124078 - assign82540_e124080);
        let assign82540_e124084: f64 = (-230.25850929940458);
        let assign82540_e124086: f64 = (-var_nqs_x0);
        let assign82540_e124087: f64 = (assign82540_e124084 - assign82540_e124086);
        let assign82540_e124089: f64 = (assign82540_e124087 * 0.3333333333333333);
        let assign82540_e124090: f64 = (1.0 + assign82540_e124089);
        let assign82540_e124091: f64 = (assign82540_e124081 * assign82540_e124090);
        let assign82540_e124092: f64 = (0.5 * assign82540_e124091);
        let assign82540_e124093: f64 = (1.0 + assign82540_e124092);
        let assign82540_e124094: f64 = (assign82540_e124074 * assign82540_e124093);
        let assign82540_e124095: f64 = (1.0 + assign82540_e124094);
        let assign82540_e124096: f64 = (1e-100 / assign82540_e124095);
        (assign82540_e124096, (-((1e-100 * (((-(-var_nqs_x0_dn5)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-var_nqs_x0_dn5)) * assign82540_e124090) + (assign82540_e124081 * ((-(-var_nqs_x0_dn5)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-var_nqs_x0_dn6)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-var_nqs_x0_dn6)) * assign82540_e124090) + (assign82540_e124081 * ((-(-var_nqs_x0_dn6)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-var_nqs_x0_dn7)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-var_nqs_x0_dn7)) * assign82540_e124090) + (assign82540_e124081 * ((-(-var_nqs_x0_dn7)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-var_nqs_x0_dn8)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-var_nqs_x0_dn8)) * assign82540_e124090) + (assign82540_e124081 * ((-(-var_nqs_x0_dn8)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-var_nqs_x0_dn12)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-var_nqs_x0_dn12)) * assign82540_e124090) + (assign82540_e124081 * ((-(-var_nqs_x0_dn12)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-var_nqs_x0_dn13)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-var_nqs_x0_dn13)) * assign82540_e124090) + (assign82540_e124081 * ((-(-var_nqs_x0_dn13)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-var_nqs_x0_dn14)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-var_nqs_x0_dn14)) * assign82540_e124090) + (assign82540_e124081 * ((-(-var_nqs_x0_dn14)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-var_nqs_x0_dn15)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-var_nqs_x0_dn15)) * assign82540_e124090) + (assign82540_e124081 * ((-(-var_nqs_x0_dn15)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-var_nqs_x0_dn16)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-var_nqs_x0_dn16)) * assign82540_e124090) + (assign82540_e124081 * ((-(-var_nqs_x0_dn16)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-var_nqs_x0_dn17)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-var_nqs_x0_dn17)) * assign82540_e124090) + (assign82540_e124081 * ((-(-var_nqs_x0_dn17)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-var_nqs_x0_dn18)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-var_nqs_x0_dn18)) * assign82540_e124090) + (assign82540_e124081 * ((-(-var_nqs_x0_dn18)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-var_nqs_x0_dn19)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-var_nqs_x0_dn19)) * assign82540_e124090) + (assign82540_e124081 * ((-(-var_nqs_x0_dn19)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))), (-((1e-100 * (((-(-var_nqs_x0_dn20)) * assign82540_e124093) + (assign82540_e124074 * (0.5 * (((-(-var_nqs_x0_dn20)) * assign82540_e124090) + (assign82540_e124081 * ((-(-var_nqs_x0_dn20)) * 0.3333333333333333))))))) / (assign82540_e124095 * assign82540_e124095))),)
    } else {
        (var_nqs_d0, var_nqs_d0_dn5, var_nqs_d0_dn6, var_nqs_d0_dn7, var_nqs_d0_dn8, var_nqs_d0_dn12, var_nqs_d0_dn13, var_nqs_d0_dn14, var_nqs_d0_dn15, var_nqs_d0_dn16, var_nqs_d0_dn17, var_nqs_d0_dn18, var_nqs_d0_dn19, var_nqs_d0_dn20,)
    }
};
        var_nqs_d0 = assign82540_e124098;
        var_nqs_d0_dn5 = assign82540_e124098_d_n5;
        var_nqs_d0_dn6 = assign82540_e124098_d_n6;
        var_nqs_d0_dn7 = assign82540_e124098_d_n7;
        var_nqs_d0_dn8 = assign82540_e124098_d_n8;
        var_nqs_d0_dn12 = assign82540_e124098_d_n12;
        var_nqs_d0_dn13 = assign82540_e124098_d_n13;
        var_nqs_d0_dn14 = assign82540_e124098_d_n14;
        var_nqs_d0_dn15 = assign82540_e124098_d_n15;
        var_nqs_d0_dn16 = assign82540_e124098_d_n16;
        var_nqs_d0_dn17 = assign82540_e124098_d_n17;
        var_nqs_d0_dn18 = assign82540_e124098_d_n18;
        var_nqs_d0_dn19 = assign82540_e124098_d_n19;
        var_nqs_d0_dn20 = assign82540_e124098_d_n20;

        let (assign82550_e124153, assign82550_e124153_d_n5, assign82550_e124153_d_n6, assign82550_e124153_d_n7, assign82550_e124153_d_n8, assign82550_e124153_d_n12, assign82550_e124153_d_n13, assign82550_e124153_d_n14, assign82550_e124153_d_n15, assign82550_e124153_d_n16, assign82550_e124153_d_n17, assign82550_e124153_d_n18, assign82550_e124153_d_n19, assign82550_e124153_d_n20,) = {
    if ((((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) && (var_guard2242 == 0.0)) && (var_guard2243 == 0.0)) {
        let assign82550_e124129: f64 = (-var_nqs_x0);
        let assign82550_e124131: f64 = (assign82550_e124129 - 230.25850929940458);
        let assign82550_e124135: f64 = (-var_nqs_x0);
        let assign82550_e124137: f64 = (assign82550_e124135 - 230.25850929940458);
        let assign82550_e124140: f64 = (-var_nqs_x0);
        let assign82550_e124142: f64 = (assign82550_e124140 - 230.25850929940458);
        let assign82550_e124144: f64 = (assign82550_e124142 * 0.3333333333333333);
        let assign82550_e124145: f64 = (1.0 + assign82550_e124144);
        let assign82550_e124146: f64 = (assign82550_e124137 * assign82550_e124145);
        let assign82550_e124147: f64 = (0.5 * assign82550_e124146);
        let assign82550_e124148: f64 = (1.0 + assign82550_e124147);
        let assign82550_e124149: f64 = (assign82550_e124131 * assign82550_e124148);
        let assign82550_e124150: f64 = (1.0 + assign82550_e124149);
        let assign82550_e124151: f64 = (1e100 * assign82550_e124150);
        (assign82550_e124151, (1e100 * (((-var_nqs_x0_dn5) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-var_nqs_x0_dn5) * assign82550_e124145) + (assign82550_e124137 * ((-var_nqs_x0_dn5) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn6) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-var_nqs_x0_dn6) * assign82550_e124145) + (assign82550_e124137 * ((-var_nqs_x0_dn6) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn7) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-var_nqs_x0_dn7) * assign82550_e124145) + (assign82550_e124137 * ((-var_nqs_x0_dn7) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn8) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-var_nqs_x0_dn8) * assign82550_e124145) + (assign82550_e124137 * ((-var_nqs_x0_dn8) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn12) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-var_nqs_x0_dn12) * assign82550_e124145) + (assign82550_e124137 * ((-var_nqs_x0_dn12) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn13) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-var_nqs_x0_dn13) * assign82550_e124145) + (assign82550_e124137 * ((-var_nqs_x0_dn13) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn14) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-var_nqs_x0_dn14) * assign82550_e124145) + (assign82550_e124137 * ((-var_nqs_x0_dn14) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn15) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-var_nqs_x0_dn15) * assign82550_e124145) + (assign82550_e124137 * ((-var_nqs_x0_dn15) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn16) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-var_nqs_x0_dn16) * assign82550_e124145) + (assign82550_e124137 * ((-var_nqs_x0_dn16) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn17) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-var_nqs_x0_dn17) * assign82550_e124145) + (assign82550_e124137 * ((-var_nqs_x0_dn17) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn18) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-var_nqs_x0_dn18) * assign82550_e124145) + (assign82550_e124137 * ((-var_nqs_x0_dn18) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn19) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-var_nqs_x0_dn19) * assign82550_e124145) + (assign82550_e124137 * ((-var_nqs_x0_dn19) * 0.3333333333333333))))))), (1e100 * (((-var_nqs_x0_dn20) * assign82550_e124148) + (assign82550_e124131 * (0.5 * (((-var_nqs_x0_dn20) * assign82550_e124145) + (assign82550_e124137 * ((-var_nqs_x0_dn20) * 0.3333333333333333))))))),)
    } else {
        (var_nqs_d0, var_nqs_d0_dn5, var_nqs_d0_dn6, var_nqs_d0_dn7, var_nqs_d0_dn8, var_nqs_d0_dn12, var_nqs_d0_dn13, var_nqs_d0_dn14, var_nqs_d0_dn15, var_nqs_d0_dn16, var_nqs_d0_dn17, var_nqs_d0_dn18, var_nqs_d0_dn19, var_nqs_d0_dn20,)
    }
};
        var_nqs_d0 = assign82550_e124153;
        var_nqs_d0_dn5 = assign82550_e124153_d_n5;
        var_nqs_d0_dn6 = assign82550_e124153_d_n6;
        var_nqs_d0_dn7 = assign82550_e124153_d_n7;
        var_nqs_d0_dn8 = assign82550_e124153_d_n8;
        var_nqs_d0_dn12 = assign82550_e124153_d_n12;
        var_nqs_d0_dn13 = assign82550_e124153_d_n13;
        var_nqs_d0_dn14 = assign82550_e124153_d_n14;
        var_nqs_d0_dn15 = assign82550_e124153_d_n15;
        var_nqs_d0_dn16 = assign82550_e124153_d_n16;
        var_nqs_d0_dn17 = assign82550_e124153_d_n17;
        var_nqs_d0_dn18 = assign82550_e124153_d_n18;
        var_nqs_d0_dn19 = assign82550_e124153_d_n19;
        var_nqs_d0_dn20 = assign82550_e124153_d_n20;

        let (assign82560_e124183, assign82560_e124183_d_n5, assign82560_e124183_d_n6, assign82560_e124183_d_n7, assign82560_e124183_d_n8, assign82560_e124183_d_n12, assign82560_e124183_d_n13, assign82560_e124183_d_n14, assign82560_e124183_d_n15, assign82560_e124183_d_n16, assign82560_e124183_d_n17, assign82560_e124183_d_n18, assign82560_e124183_d_n19, assign82560_e124183_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) {
        let assign82560_e124178: f64 = (var_gp2 * 0.5);
        let assign82560_e124180: f64 = (assign82560_e124178 * var_nqs_d0);
        let assign82560_e124181: f64 = (1.0 - assign82560_e124180);
        (assign82560_e124181, (-(((var_gp2_dn5 * 0.5) * var_nqs_d0) + (assign82560_e124178 * var_nqs_d0_dn5))), (-(((var_gp2_dn6 * 0.5) * var_nqs_d0) + (assign82560_e124178 * var_nqs_d0_dn6))), (-(((var_gp2_dn7 * 0.5) * var_nqs_d0) + (assign82560_e124178 * var_nqs_d0_dn7))), (-(((var_gp2_dn8 * 0.5) * var_nqs_d0) + (assign82560_e124178 * var_nqs_d0_dn8))), (-(((var_gp2_dn12 * 0.5) * var_nqs_d0) + (assign82560_e124178 * var_nqs_d0_dn12))), (-(((var_gp2_dn13 * 0.5) * var_nqs_d0) + (assign82560_e124178 * var_nqs_d0_dn13))), (-(((var_gp2_dn14 * 0.5) * var_nqs_d0) + (assign82560_e124178 * var_nqs_d0_dn14))), (-(((var_gp2_dn15 * 0.5) * var_nqs_d0) + (assign82560_e124178 * var_nqs_d0_dn15))), (-(((var_gp2_dn16 * 0.5) * var_nqs_d0) + (assign82560_e124178 * var_nqs_d0_dn16))), (-(((var_gp2_dn17 * 0.5) * var_nqs_d0) + (assign82560_e124178 * var_nqs_d0_dn17))), (-(((var_gp2_dn18 * 0.5) * var_nqs_d0) + (assign82560_e124178 * var_nqs_d0_dn18))), (-(((var_gp2_dn19 * 0.5) * var_nqs_d0) + (assign82560_e124178 * var_nqs_d0_dn19))), (-(((var_gp2_dn20 * 0.5) * var_nqs_d0) + (assign82560_e124178 * var_nqs_d0_dn20))),)
    } else {
        (var_nqs_xi, var_nqs_xi_dn5, var_nqs_xi_dn6, var_nqs_xi_dn7, var_nqs_xi_dn8, var_nqs_xi_dn12, var_nqs_xi_dn13, var_nqs_xi_dn14, var_nqs_xi_dn15, var_nqs_xi_dn16, var_nqs_xi_dn17, var_nqs_xi_dn18, var_nqs_xi_dn19, var_nqs_xi_dn20,)
    }
};
        var_nqs_xi = assign82560_e124183;
        var_nqs_xi_dn5 = assign82560_e124183_d_n5;
        var_nqs_xi_dn6 = assign82560_e124183_d_n6;
        var_nqs_xi_dn7 = assign82560_e124183_d_n7;
        var_nqs_xi_dn8 = assign82560_e124183_d_n8;
        var_nqs_xi_dn12 = assign82560_e124183_d_n12;
        var_nqs_xi_dn13 = assign82560_e124183_d_n13;
        var_nqs_xi_dn14 = assign82560_e124183_d_n14;
        var_nqs_xi_dn15 = assign82560_e124183_d_n15;
        var_nqs_xi_dn16 = assign82560_e124183_d_n16;
        var_nqs_xi_dn17 = assign82560_e124183_d_n17;
        var_nqs_xi_dn18 = assign82560_e124183_d_n18;
        var_nqs_xi_dn19 = assign82560_e124183_d_n19;
        var_nqs_xi_dn20 = assign82560_e124183_d_n20;

        let (assign82570_e124217, assign82570_e124217_d_n5, assign82570_e124217_d_n6, assign82570_e124217_d_n7, assign82570_e124217_d_n8, assign82570_e124217_d_n12, assign82570_e124217_d_n13, assign82570_e124217_d_n14, assign82570_e124217_d_n15, assign82570_e124217_d_n16, assign82570_e124217_d_n17, assign82570_e124217_d_n18, assign82570_e124217_d_n19, assign82570_e124217_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) {
        let assign82570_e124208: f64 = (var_temp__blk1038 - var_nqs_x0);
        let assign82570_e124209: f64 = (2.0 * assign82570_e124208);
        let assign82570_e124213: f64 = (1.0 - var_nqs_d0);
        let assign82570_e124214: f64 = (var_gp2 * assign82570_e124213);
        let assign82570_e124215: f64 = (assign82570_e124209 + assign82570_e124214);
        (assign82570_e124215, ((2.0 * (var_temp__blk1038_dn5 - var_nqs_x0_dn5)) + ((var_gp2_dn5 * assign82570_e124213) + (var_gp2 * (-var_nqs_d0_dn5)))), ((2.0 * (var_temp__blk1038_dn6 - var_nqs_x0_dn6)) + ((var_gp2_dn6 * assign82570_e124213) + (var_gp2 * (-var_nqs_d0_dn6)))), ((2.0 * (var_temp__blk1038_dn7 - var_nqs_x0_dn7)) + ((var_gp2_dn7 * assign82570_e124213) + (var_gp2 * (-var_nqs_d0_dn7)))), ((2.0 * (var_temp__blk1038_dn8 - var_nqs_x0_dn8)) + ((var_gp2_dn8 * assign82570_e124213) + (var_gp2 * (-var_nqs_d0_dn8)))), ((2.0 * (var_temp__blk1038_dn12 - var_nqs_x0_dn12)) + ((var_gp2_dn12 * assign82570_e124213) + (var_gp2 * (-var_nqs_d0_dn12)))), ((2.0 * (var_temp__blk1038_dn13 - var_nqs_x0_dn13)) + ((var_gp2_dn13 * assign82570_e124213) + (var_gp2 * (-var_nqs_d0_dn13)))), ((2.0 * (var_temp__blk1038_dn14 - var_nqs_x0_dn14)) + ((var_gp2_dn14 * assign82570_e124213) + (var_gp2 * (-var_nqs_d0_dn14)))), ((2.0 * (var_temp__blk1038_dn15 - var_nqs_x0_dn15)) + ((var_gp2_dn15 * assign82570_e124213) + (var_gp2 * (-var_nqs_d0_dn15)))), ((2.0 * (var_temp__blk1038_dn16 - var_nqs_x0_dn16)) + ((var_gp2_dn16 * assign82570_e124213) + (var_gp2 * (-var_nqs_d0_dn16)))), ((2.0 * (var_temp__blk1038_dn17 - var_nqs_x0_dn17)) + ((var_gp2_dn17 * assign82570_e124213) + (var_gp2 * (-var_nqs_d0_dn17)))), ((2.0 * (var_temp__blk1038_dn18 - var_nqs_x0_dn18)) + ((var_gp2_dn18 * assign82570_e124213) + (var_gp2 * (-var_nqs_d0_dn18)))), ((2.0 * (var_temp__blk1038_dn19 - var_nqs_x0_dn19)) + ((var_gp2_dn19 * assign82570_e124213) + (var_gp2 * (-var_nqs_d0_dn19)))), ((2.0 * (var_temp__blk1038_dn20 - var_nqs_x0_dn20)) + ((var_gp2_dn20 * assign82570_e124213) + (var_gp2 * (-var_nqs_d0_dn20)))),)
    } else {
        (var_nqs_p, var_nqs_p_dn5, var_nqs_p_dn6, var_nqs_p_dn7, var_nqs_p_dn8, var_nqs_p_dn12, var_nqs_p_dn13, var_nqs_p_dn14, var_nqs_p_dn15, var_nqs_p_dn16, var_nqs_p_dn17, var_nqs_p_dn18, var_nqs_p_dn19, var_nqs_p_dn20,)
    }
};
        var_nqs_p = assign82570_e124217;
        var_nqs_p_dn5 = assign82570_e124217_d_n5;
        var_nqs_p_dn6 = assign82570_e124217_d_n6;
        var_nqs_p_dn7 = assign82570_e124217_d_n7;
        var_nqs_p_dn8 = assign82570_e124217_d_n8;
        var_nqs_p_dn12 = assign82570_e124217_d_n12;
        var_nqs_p_dn13 = assign82570_e124217_d_n13;
        var_nqs_p_dn14 = assign82570_e124217_d_n14;
        var_nqs_p_dn15 = assign82570_e124217_d_n15;
        var_nqs_p_dn16 = assign82570_e124217_d_n16;
        var_nqs_p_dn17 = assign82570_e124217_d_n17;
        var_nqs_p_dn18 = assign82570_e124217_d_n18;
        var_nqs_p_dn19 = assign82570_e124217_d_n19;
        var_nqs_p_dn20 = assign82570_e124217_d_n20;

        let (assign82580_e124255, assign82580_e124255_d_n5, assign82580_e124255_d_n6, assign82580_e124255_d_n7, assign82580_e124255_d_n8, assign82580_e124255_d_n12, assign82580_e124255_d_n13, assign82580_e124255_d_n14, assign82580_e124255_d_n15, assign82580_e124255_d_n16, assign82580_e124255_d_n17, assign82580_e124255_d_n18, assign82580_e124255_d_n19, assign82580_e124255_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) {
        let assign82580_e124241: f64 = (var_temp__blk1038 - var_nqs_x0);
        let assign82580_e124244: f64 = (var_temp__blk1038 - var_nqs_x0);
        let assign82580_e124245: f64 = (assign82580_e124241 * assign82580_e124244);
        let assign82580_e124249: f64 = (var_nqs_x0 - 1.0);
        let assign82580_e124251: f64 = (assign82580_e124249 + var_nqs_d0);
        let assign82580_e124252: f64 = (var_gp2 * assign82580_e124251);
        let assign82580_e124253: f64 = (assign82580_e124245 - assign82580_e124252);
        (assign82580_e124253, ((((var_temp__blk1038_dn5 - var_nqs_x0_dn5) * assign82580_e124244) + (assign82580_e124241 * (var_temp__blk1038_dn5 - var_nqs_x0_dn5))) - ((var_gp2_dn5 * assign82580_e124251) + (var_gp2 * (var_nqs_x0_dn5 + var_nqs_d0_dn5)))), ((((var_temp__blk1038_dn6 - var_nqs_x0_dn6) * assign82580_e124244) + (assign82580_e124241 * (var_temp__blk1038_dn6 - var_nqs_x0_dn6))) - ((var_gp2_dn6 * assign82580_e124251) + (var_gp2 * (var_nqs_x0_dn6 + var_nqs_d0_dn6)))), ((((var_temp__blk1038_dn7 - var_nqs_x0_dn7) * assign82580_e124244) + (assign82580_e124241 * (var_temp__blk1038_dn7 - var_nqs_x0_dn7))) - ((var_gp2_dn7 * assign82580_e124251) + (var_gp2 * (var_nqs_x0_dn7 + var_nqs_d0_dn7)))), ((((var_temp__blk1038_dn8 - var_nqs_x0_dn8) * assign82580_e124244) + (assign82580_e124241 * (var_temp__blk1038_dn8 - var_nqs_x0_dn8))) - ((var_gp2_dn8 * assign82580_e124251) + (var_gp2 * (var_nqs_x0_dn8 + var_nqs_d0_dn8)))), ((((var_temp__blk1038_dn12 - var_nqs_x0_dn12) * assign82580_e124244) + (assign82580_e124241 * (var_temp__blk1038_dn12 - var_nqs_x0_dn12))) - ((var_gp2_dn12 * assign82580_e124251) + (var_gp2 * (var_nqs_x0_dn12 + var_nqs_d0_dn12)))), ((((var_temp__blk1038_dn13 - var_nqs_x0_dn13) * assign82580_e124244) + (assign82580_e124241 * (var_temp__blk1038_dn13 - var_nqs_x0_dn13))) - ((var_gp2_dn13 * assign82580_e124251) + (var_gp2 * (var_nqs_x0_dn13 + var_nqs_d0_dn13)))), ((((var_temp__blk1038_dn14 - var_nqs_x0_dn14) * assign82580_e124244) + (assign82580_e124241 * (var_temp__blk1038_dn14 - var_nqs_x0_dn14))) - ((var_gp2_dn14 * assign82580_e124251) + (var_gp2 * (var_nqs_x0_dn14 + var_nqs_d0_dn14)))), ((((var_temp__blk1038_dn15 - var_nqs_x0_dn15) * assign82580_e124244) + (assign82580_e124241 * (var_temp__blk1038_dn15 - var_nqs_x0_dn15))) - ((var_gp2_dn15 * assign82580_e124251) + (var_gp2 * (var_nqs_x0_dn15 + var_nqs_d0_dn15)))), ((((var_temp__blk1038_dn16 - var_nqs_x0_dn16) * assign82580_e124244) + (assign82580_e124241 * (var_temp__blk1038_dn16 - var_nqs_x0_dn16))) - ((var_gp2_dn16 * assign82580_e124251) + (var_gp2 * (var_nqs_x0_dn16 + var_nqs_d0_dn16)))), ((((var_temp__blk1038_dn17 - var_nqs_x0_dn17) * assign82580_e124244) + (assign82580_e124241 * (var_temp__blk1038_dn17 - var_nqs_x0_dn17))) - ((var_gp2_dn17 * assign82580_e124251) + (var_gp2 * (var_nqs_x0_dn17 + var_nqs_d0_dn17)))), ((((var_temp__blk1038_dn18 - var_nqs_x0_dn18) * assign82580_e124244) + (assign82580_e124241 * (var_temp__blk1038_dn18 - var_nqs_x0_dn18))) - ((var_gp2_dn18 * assign82580_e124251) + (var_gp2 * (var_nqs_x0_dn18 + var_nqs_d0_dn18)))), ((((var_temp__blk1038_dn19 - var_nqs_x0_dn19) * assign82580_e124244) + (assign82580_e124241 * (var_temp__blk1038_dn19 - var_nqs_x0_dn19))) - ((var_gp2_dn19 * assign82580_e124251) + (var_gp2 * (var_nqs_x0_dn19 + var_nqs_d0_dn19)))), ((((var_temp__blk1038_dn20 - var_nqs_x0_dn20) * assign82580_e124244) + (assign82580_e124241 * (var_temp__blk1038_dn20 - var_nqs_x0_dn20))) - ((var_gp2_dn20 * assign82580_e124251) + (var_gp2 * (var_nqs_x0_dn20 + var_nqs_d0_dn20)))),)
    } else {
        (var_nqs_q, var_nqs_q_dn5, var_nqs_q_dn6, var_nqs_q_dn7, var_nqs_q_dn8, var_nqs_q_dn12, var_nqs_q_dn13, var_nqs_q_dn14, var_nqs_q_dn15, var_nqs_q_dn16, var_nqs_q_dn17, var_nqs_q_dn18, var_nqs_q_dn19, var_nqs_q_dn20,)
    }
};
        var_nqs_q = assign82580_e124255;
        var_nqs_q_dn5 = assign82580_e124255_d_n5;
        var_nqs_q_dn6 = assign82580_e124255_d_n6;
        var_nqs_q_dn7 = assign82580_e124255_d_n7;
        var_nqs_q_dn8 = assign82580_e124255_d_n8;
        var_nqs_q_dn12 = assign82580_e124255_d_n12;
        var_nqs_q_dn13 = assign82580_e124255_d_n13;
        var_nqs_q_dn14 = assign82580_e124255_d_n14;
        var_nqs_q_dn15 = assign82580_e124255_d_n15;
        var_nqs_q_dn16 = assign82580_e124255_d_n16;
        var_nqs_q_dn17 = assign82580_e124255_d_n17;
        var_nqs_q_dn18 = assign82580_e124255_d_n18;
        var_nqs_q_dn19 = assign82580_e124255_d_n19;
        var_nqs_q_dn20 = assign82580_e124255_d_n20;

        let (assign82590_e124287, assign82590_e124287_d_n5, assign82590_e124287_d_n6, assign82590_e124287_d_n7, assign82590_e124287_d_n8, assign82590_e124287_d_n12, assign82590_e124287_d_n13, assign82590_e124287_d_n14, assign82590_e124287_d_n15, assign82590_e124287_d_n16, assign82590_e124287_d_n17, assign82590_e124287_d_n18, assign82590_e124287_d_n19, assign82590_e124287_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) {
        let assign82590_e124279: f64 = (var_nqs_p * var_nqs_p);
        let assign82590_e124282: f64 = (4.0 * var_nqs_xi);
        let assign82590_e124284: f64 = (assign82590_e124282 * var_nqs_q);
        let assign82590_e124285: f64 = (assign82590_e124279 - assign82590_e124284);
        (assign82590_e124285, (((var_nqs_p_dn5 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn5)) - (((4.0 * var_nqs_xi_dn5) * var_nqs_q) + (assign82590_e124282 * var_nqs_q_dn5))), (((var_nqs_p_dn6 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn6)) - (((4.0 * var_nqs_xi_dn6) * var_nqs_q) + (assign82590_e124282 * var_nqs_q_dn6))), (((var_nqs_p_dn7 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn7)) - (((4.0 * var_nqs_xi_dn7) * var_nqs_q) + (assign82590_e124282 * var_nqs_q_dn7))), (((var_nqs_p_dn8 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn8)) - (((4.0 * var_nqs_xi_dn8) * var_nqs_q) + (assign82590_e124282 * var_nqs_q_dn8))), (((var_nqs_p_dn12 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn12)) - (((4.0 * var_nqs_xi_dn12) * var_nqs_q) + (assign82590_e124282 * var_nqs_q_dn12))), (((var_nqs_p_dn13 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn13)) - (((4.0 * var_nqs_xi_dn13) * var_nqs_q) + (assign82590_e124282 * var_nqs_q_dn13))), (((var_nqs_p_dn14 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn14)) - (((4.0 * var_nqs_xi_dn14) * var_nqs_q) + (assign82590_e124282 * var_nqs_q_dn14))), (((var_nqs_p_dn15 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn15)) - (((4.0 * var_nqs_xi_dn15) * var_nqs_q) + (assign82590_e124282 * var_nqs_q_dn15))), (((var_nqs_p_dn16 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn16)) - (((4.0 * var_nqs_xi_dn16) * var_nqs_q) + (assign82590_e124282 * var_nqs_q_dn16))), (((var_nqs_p_dn17 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn17)) - (((4.0 * var_nqs_xi_dn17) * var_nqs_q) + (assign82590_e124282 * var_nqs_q_dn17))), (((var_nqs_p_dn18 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn18)) - (((4.0 * var_nqs_xi_dn18) * var_nqs_q) + (assign82590_e124282 * var_nqs_q_dn18))), (((var_nqs_p_dn19 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn19)) - (((4.0 * var_nqs_xi_dn19) * var_nqs_q) + (assign82590_e124282 * var_nqs_q_dn19))), (((var_nqs_p_dn20 * var_nqs_p) + (var_nqs_p * var_nqs_p_dn20)) - (((4.0 * var_nqs_xi_dn20) * var_nqs_q) + (assign82590_e124282 * var_nqs_q_dn20))),)
    } else {
        (var_nqs_temp, var_nqs_temp_dn5, var_nqs_temp_dn6, var_nqs_temp_dn7, var_nqs_temp_dn8, var_nqs_temp_dn12, var_nqs_temp_dn13, var_nqs_temp_dn14, var_nqs_temp_dn15, var_nqs_temp_dn16, var_nqs_temp_dn17, var_nqs_temp_dn18, var_nqs_temp_dn19, var_nqs_temp_dn20,)
    }
};
        var_nqs_temp = assign82590_e124287;
        var_nqs_temp_dn5 = assign82590_e124287_d_n5;
        var_nqs_temp_dn6 = assign82590_e124287_d_n6;
        var_nqs_temp_dn7 = assign82590_e124287_d_n7;
        var_nqs_temp_dn8 = assign82590_e124287_d_n8;
        var_nqs_temp_dn12 = assign82590_e124287_d_n12;
        var_nqs_temp_dn13 = assign82590_e124287_d_n13;
        var_nqs_temp_dn14 = assign82590_e124287_d_n14;
        var_nqs_temp_dn15 = assign82590_e124287_d_n15;
        var_nqs_temp_dn16 = assign82590_e124287_d_n16;
        var_nqs_temp_dn17 = assign82590_e124287_d_n17;
        var_nqs_temp_dn18 = assign82590_e124287_d_n18;
        var_nqs_temp_dn19 = assign82590_e124287_d_n19;
        var_nqs_temp_dn20 = assign82590_e124287_d_n20;

        let (assign82600_e124318, assign82600_e124318_d_n5, assign82600_e124318_d_n6, assign82600_e124318_d_n7, assign82600_e124318_d_n8, assign82600_e124318_d_n12, assign82600_e124318_d_n13, assign82600_e124318_d_n14, assign82600_e124318_d_n15, assign82600_e124318_d_n16, assign82600_e124318_d_n17, assign82600_e124318_d_n18, assign82600_e124318_d_n19, assign82600_e124318_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) {
        let assign82600_e124311: f64 = (2.0 * var_nqs_q);
        let assign82600_e124314: f64 = (var_nqs_temp).sqrt();
        let assign82600_e124315: f64 = (var_nqs_p + assign82600_e124314);
        let assign82600_e124316: f64 = (assign82600_e124311 / assign82600_e124315);
        (assign82600_e124316, ((((2.0 * var_nqs_q_dn5) * assign82600_e124315) - (assign82600_e124311 * (var_nqs_p_dn5 + (var_nqs_temp_dn5 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * var_nqs_q_dn6) * assign82600_e124315) - (assign82600_e124311 * (var_nqs_p_dn6 + (var_nqs_temp_dn6 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * var_nqs_q_dn7) * assign82600_e124315) - (assign82600_e124311 * (var_nqs_p_dn7 + (var_nqs_temp_dn7 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * var_nqs_q_dn8) * assign82600_e124315) - (assign82600_e124311 * (var_nqs_p_dn8 + (var_nqs_temp_dn8 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * var_nqs_q_dn12) * assign82600_e124315) - (assign82600_e124311 * (var_nqs_p_dn12 + (var_nqs_temp_dn12 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * var_nqs_q_dn13) * assign82600_e124315) - (assign82600_e124311 * (var_nqs_p_dn13 + (var_nqs_temp_dn13 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * var_nqs_q_dn14) * assign82600_e124315) - (assign82600_e124311 * (var_nqs_p_dn14 + (var_nqs_temp_dn14 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * var_nqs_q_dn15) * assign82600_e124315) - (assign82600_e124311 * (var_nqs_p_dn15 + (var_nqs_temp_dn15 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * var_nqs_q_dn16) * assign82600_e124315) - (assign82600_e124311 * (var_nqs_p_dn16 + (var_nqs_temp_dn16 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * var_nqs_q_dn17) * assign82600_e124315) - (assign82600_e124311 * (var_nqs_p_dn17 + (var_nqs_temp_dn17 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * var_nqs_q_dn18) * assign82600_e124315) - (assign82600_e124311 * (var_nqs_p_dn18 + (var_nqs_temp_dn18 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * var_nqs_q_dn19) * assign82600_e124315) - (assign82600_e124311 * (var_nqs_p_dn19 + (var_nqs_temp_dn19 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)), ((((2.0 * var_nqs_q_dn20) * assign82600_e124315) - (assign82600_e124311 * (var_nqs_p_dn20 + (var_nqs_temp_dn20 / (2.0 * assign82600_e124314))))) / (assign82600_e124315 * assign82600_e124315)),)
    } else {
        (var_nqs_u, var_nqs_u_dn5, var_nqs_u_dn6, var_nqs_u_dn7, var_nqs_u_dn8, var_nqs_u_dn12, var_nqs_u_dn13, var_nqs_u_dn14, var_nqs_u_dn15, var_nqs_u_dn16, var_nqs_u_dn17, var_nqs_u_dn18, var_nqs_u_dn19, var_nqs_u_dn20,)
    }
};
        var_nqs_u = assign82600_e124318;
        var_nqs_u_dn5 = assign82600_e124318_d_n5;
        var_nqs_u_dn6 = assign82600_e124318_d_n6;
        var_nqs_u_dn7 = assign82600_e124318_d_n7;
        var_nqs_u_dn8 = assign82600_e124318_d_n8;
        var_nqs_u_dn12 = assign82600_e124318_d_n12;
        var_nqs_u_dn13 = assign82600_e124318_d_n13;
        var_nqs_u_dn14 = assign82600_e124318_d_n14;
        var_nqs_u_dn15 = assign82600_e124318_d_n15;
        var_nqs_u_dn16 = assign82600_e124318_d_n16;
        var_nqs_u_dn17 = assign82600_e124318_d_n17;
        var_nqs_u_dn18 = assign82600_e124318_d_n18;
        var_nqs_u_dn19 = assign82600_e124318_d_n19;
        var_nqs_u_dn20 = assign82600_e124318_d_n20;

        let (assign82610_e124344, assign82610_e124344_d_n5, assign82610_e124344_d_n6, assign82610_e124344_d_n7, assign82610_e124344_d_n8, assign82610_e124344_d_n12, assign82610_e124344_d_n13, assign82610_e124344_d_n14, assign82610_e124344_d_n15, assign82610_e124344_d_n16, assign82610_e124344_d_n17, assign82610_e124344_d_n18, assign82610_e124344_d_n19, assign82610_e124344_d_n20,) = {
    if ((((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) && (var_guard2236 == 0.0)) && (var_guard2237 == 0.0)) {
        let assign82610_e124342: f64 = (var_nqs_x0 + var_nqs_u);
        (assign82610_e124342, (var_nqs_x0_dn5 + var_nqs_u_dn5), (var_nqs_x0_dn6 + var_nqs_u_dn6), (var_nqs_x0_dn7 + var_nqs_u_dn7), (var_nqs_x0_dn8 + var_nqs_u_dn8), (var_nqs_x0_dn12 + var_nqs_u_dn12), (var_nqs_x0_dn13 + var_nqs_u_dn13), (var_nqs_x0_dn14 + var_nqs_u_dn14), (var_nqs_x0_dn15 + var_nqs_u_dn15), (var_nqs_x0_dn16 + var_nqs_u_dn16), (var_nqs_x0_dn17 + var_nqs_u_dn17), (var_nqs_x0_dn18 + var_nqs_u_dn18), (var_nqs_x0_dn19 + var_nqs_u_dn19), (var_nqs_x0_dn20 + var_nqs_u_dn20),)
    } else {
        (var_temp9, var_temp9_dn5, var_temp9_dn6, var_temp9_dn7, var_temp9_dn8, var_temp9_dn12, var_temp9_dn13, var_temp9_dn14, var_temp9_dn15, var_temp9_dn16, var_temp9_dn17, var_temp9_dn18, var_temp9_dn19, var_temp9_dn20,)
    }
};
        var_temp9 = assign82610_e124344;
        var_temp9_dn5 = assign82610_e124344_d_n5;
        var_temp9_dn6 = assign82610_e124344_d_n6;
        var_temp9_dn7 = assign82610_e124344_d_n7;
        var_temp9_dn8 = assign82610_e124344_d_n8;
        var_temp9_dn12 = assign82610_e124344_d_n12;
        var_temp9_dn13 = assign82610_e124344_d_n13;
        var_temp9_dn14 = assign82610_e124344_d_n14;
        var_temp9_dn15 = assign82610_e124344_d_n15;
        var_temp9_dn16 = assign82610_e124344_d_n16;
        var_temp9_dn17 = assign82610_e124344_d_n17;
        var_temp9_dn18 = assign82610_e124344_d_n18;
        var_temp9_dn19 = assign82610_e124344_d_n19;
        var_temp9_dn20 = assign82610_e124344_d_n20;

        let (assign82620_e124390, assign82620_e124390_d_n5, assign82620_e124390_d_n6, assign82620_e124390_d_n7, assign82620_e124390_d_n8, assign82620_e124390_d_n12, assign82620_e124390_d_n13, assign82620_e124390_d_n14, assign82620_e124390_d_n15, assign82620_e124390_d_n16, assign82620_e124390_d_n17, assign82620_e124390_d_n18, assign82620_e124390_d_n19, assign82620_e124390_d_n20,) = {
    if ((((((var_guard2078 != 0.0) && (var_guard2079 == 0.0)) && (var_guard2088 == 0.0)) && (var_guard2105 == 0.0)) && (var_guard2130 == 0.0)) && (var_guard2171 != 0.0)) {
        let assign82620_e124365: f64 = (var_temp1 + var_temp3);
        let assign82620_e124367: f64 = (assign82620_e124365 + var_temp5);
        let assign82620_e124369: f64 = (assign82620_e124367 + var_temp7);
        let assign82620_e124371: f64 = (assign82620_e124369 + var_temp9);
        let assign82620_e124372: f64 = (4.0 * assign82620_e124371);
        let assign82620_e124373: f64 = (var_x_sp + assign82620_e124372);
        let assign82620_e124377: f64 = (var_temp2 + var_temp4);
        let assign82620_e124379: f64 = (assign82620_e124377 + var_temp6);
        let assign82620_e124381: f64 = (assign82620_e124379 + var_temp8);
        let assign82620_e124382: f64 = (2.0 * assign82620_e124381);
        let assign82620_e124383: f64 = (assign82620_e124373 + assign82620_e124382);
        let assign82620_e124385: f64 = (assign82620_e124383 + var_x_dp);
        let assign82620_e124387: f64 = (assign82620_e124385 / 30.0);
        let assign82620_e124388: f64 = (var_xg_ac - assign82620_e124387);
        (assign82620_e124388, (var_xg_ac_dn5 - ((((var_x_sp_dn5 + (4.0 * ((((var_temp1_dn5 + var_temp3_dn5) + var_temp5_dn5) + var_temp7_dn5) + var_temp9_dn5))) + (2.0 * (((var_temp2_dn5 + var_temp4_dn5) + var_temp6_dn5) + var_temp8_dn5))) + var_x_dp_dn5) / 30.0)), (var_xg_ac_dn6 - ((((var_x_sp_dn6 + (4.0 * ((((var_temp1_dn6 + var_temp3_dn6) + var_temp5_dn6) + var_temp7_dn6) + var_temp9_dn6))) + (2.0 * (((var_temp2_dn6 + var_temp4_dn6) + var_temp6_dn6) + var_temp8_dn6))) + var_x_dp_dn6) / 30.0)), (var_xg_ac_dn7 - ((((var_x_sp_dn7 + (4.0 * ((((var_temp1_dn7 + var_temp3_dn7) + var_temp5_dn7) + var_temp7_dn7) + var_temp9_dn7))) + (2.0 * (((var_temp2_dn7 + var_temp4_dn7) + var_temp6_dn7) + var_temp8_dn7))) + var_x_dp_dn7) / 30.0)), (var_xg_ac_dn8 - ((((var_x_sp_dn8 + (4.0 * ((((var_temp1_dn8 + var_temp3_dn8) + var_temp5_dn8) + var_temp7_dn8) + var_temp9_dn8))) + (2.0 * (((var_temp2_dn8 + var_temp4_dn8) + var_temp6_dn8) + var_temp8_dn8))) + var_x_dp_dn8) / 30.0)), (var_xg_ac_dn12 - ((((var_x_sp_dn12 + (4.0 * ((((var_temp1_dn12 + var_temp3_dn12) + var_temp5_dn12) + var_temp7_dn12) + var_temp9_dn12))) + (2.0 * (((var_temp2_dn12 + var_temp4_dn12) + var_temp6_dn12) + var_temp8_dn12))) + var_x_dp_dn12) / 30.0)), (var_xg_ac_dn13 - ((((var_x_sp_dn13 + (4.0 * ((((var_temp1_dn13 + var_temp3_dn13) + var_temp5_dn13) + var_temp7_dn13) + var_temp9_dn13))) + (2.0 * (((var_temp2_dn13 + var_temp4_dn13) + var_temp6_dn13) + var_temp8_dn13))) + var_x_dp_dn13) / 30.0)), (var_xg_ac_dn14 - ((((var_x_sp_dn14 + (4.0 * ((((var_temp1_dn14 + var_temp3_dn14) + var_temp5_dn14) + var_temp7_dn14) + var_temp9_dn14))) + (2.0 * (((var_temp2_dn14 + var_temp4_dn14) + var_temp6_dn14) + var_temp8_dn14))) + var_x_dp_dn14) / 30.0)), (var_xg_ac_dn15 - ((((var_x_sp_dn15 + (4.0 * ((((var_temp1_dn15 + var_temp3_dn15) + var_temp5_dn15) + var_temp7_dn15) + var_temp9_dn15))) + (2.0 * (((var_temp2_dn15 + var_temp4_dn15) + var_temp6_dn15) + var_temp8_dn15))) + var_x_dp_dn15) / 30.0)), (var_xg_ac_dn16 - ((((var_x_sp_dn16 + (4.0 * ((((var_temp1_dn16 + var_temp3_dn16) + var_temp5_dn16) + var_temp7_dn16) + var_temp9_dn16))) + (2.0 * (((var_temp2_dn16 + var_temp4_dn16) + var_temp6_dn16) + var_temp8_dn16))) + var_x_dp_dn16) / 30.0)), (var_xg_ac_dn17 - ((((var_x_sp_dn17 + (4.0 * ((((var_temp1_dn17 + var_temp3_dn17) + var_temp5_dn17) + var_temp7_dn17) + var_temp9_dn17))) + (2.0 * (((var_temp2_dn17 + var_temp4_dn17) + var_temp6_dn17) + var_temp8_dn17))) + var_x_dp_dn17) / 30.0)), (var_xg_ac_dn18 - ((((var_x_sp_dn18 + (4.0 * ((((var_temp1_dn18 + var_temp3_dn18) + var_temp5_dn18) + var_temp7_dn18) + var_temp9_dn18))) + (2.0 * (((var_temp2_dn18 + var_temp4_dn18) + var_temp6_dn18) + var_temp8_dn18))) + var_x_dp_dn18) / 30.0)), (var_xg_ac_dn19 - ((((var_x_sp_dn19 + (4.0 * ((((var_temp1_dn19 + var_temp3_dn19) + var_temp5_dn19) + var_temp7_dn19) + var_temp9_dn19))) + (2.0 * (((var_temp2_dn19 + var_temp4_dn19) + var_temp6_dn19) + var_temp8_dn19))) + var_x_dp_dn19) / 30.0)), (var_xg_ac_dn20 - ((((var_x_sp_dn20 + (4.0 * ((((var_temp1_dn20 + var_temp3_dn20) + var_temp5_dn20) + var_temp7_dn20) + var_temp9_dn20))) + (2.0 * (((var_temp2_dn20 + var_temp4_dn20) + var_temp6_dn20) + var_temp8_dn20))) + var_x_dp_dn20) / 30.0)),)
    } else {
        (var_qg_nqs, var_qg_nqs_dn5, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn8, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn14, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18, var_qg_nqs_dn19, var_qg_nqs_dn20,)
    }
};
        var_qg_nqs = assign82620_e124390;
        var_qg_nqs_dn5 = assign82620_e124390_d_n5;
        var_qg_nqs_dn6 = assign82620_e124390_d_n6;
        var_qg_nqs_dn7 = assign82620_e124390_d_n7;
        var_qg_nqs_dn8 = assign82620_e124390_d_n8;
        var_qg_nqs_dn12 = assign82620_e124390_d_n12;
        var_qg_nqs_dn13 = assign82620_e124390_d_n13;
        var_qg_nqs_dn14 = assign82620_e124390_d_n14;
        var_qg_nqs_dn15 = assign82620_e124390_d_n15;
        var_qg_nqs_dn16 = assign82620_e124390_d_n16;
        var_qg_nqs_dn17 = assign82620_e124390_d_n17;
        var_qg_nqs_dn18 = assign82620_e124390_d_n18;
        var_qg_nqs_dn19 = assign82620_e124390_d_n19;
        var_qg_nqs_dn20 = assign82620_e124390_d_n20;

        let (assign82630_e124396, assign82630_e124396_d_n5, assign82630_e124396_d_n6, assign82630_e124396_d_n7, assign82630_e124396_d_n8, assign82630_e124396_d_n12, assign82630_e124396_d_n13, assign82630_e124396_d_n14, assign82630_e124396_d_n15, assign82630_e124396_d_n16, assign82630_e124396_d_n17, assign82630_e124396_d_n18, assign82630_e124396_d_n19, assign82630_e124396_d_n20,) = {
    if (var_guard2078 != 0.0) {
        let assign82630_e124394: f64 = (var_pd * var_qg_nqs);
        (assign82630_e124394, ((var_pd_dn5 * var_qg_nqs) + (var_pd * var_qg_nqs_dn5)), ((var_pd_dn6 * var_qg_nqs) + (var_pd * var_qg_nqs_dn6)), ((var_pd_dn7 * var_qg_nqs) + (var_pd * var_qg_nqs_dn7)), ((var_pd_dn8 * var_qg_nqs) + (var_pd * var_qg_nqs_dn8)), ((var_pd_dn12 * var_qg_nqs) + (var_pd * var_qg_nqs_dn12)), ((var_pd_dn13 * var_qg_nqs) + (var_pd * var_qg_nqs_dn13)), ((var_pd_dn14 * var_qg_nqs) + (var_pd * var_qg_nqs_dn14)), ((var_pd_dn15 * var_qg_nqs) + (var_pd * var_qg_nqs_dn15)), ((var_pd_dn16 * var_qg_nqs) + (var_pd * var_qg_nqs_dn16)), ((var_pd_dn17 * var_qg_nqs) + (var_pd * var_qg_nqs_dn17)), ((var_pd_dn18 * var_qg_nqs) + (var_pd * var_qg_nqs_dn18)), ((var_pd_dn19 * var_qg_nqs) + (var_pd * var_qg_nqs_dn19)), ((var_pd_dn20 * var_qg_nqs) + (var_pd * var_qg_nqs_dn20)),)
    } else {
        (var_qg_nqs, var_qg_nqs_dn5, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn8, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn14, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18, var_qg_nqs_dn19, var_qg_nqs_dn20,)
    }
};
        var_qg_nqs = assign82630_e124396;
        var_qg_nqs_dn5 = assign82630_e124396_d_n5;
        var_qg_nqs_dn6 = assign82630_e124396_d_n6;
        var_qg_nqs_dn7 = assign82630_e124396_d_n7;
        var_qg_nqs_dn8 = assign82630_e124396_d_n8;
        var_qg_nqs_dn12 = assign82630_e124396_d_n12;
        var_qg_nqs_dn13 = assign82630_e124396_d_n13;
        var_qg_nqs_dn14 = assign82630_e124396_d_n14;
        var_qg_nqs_dn15 = assign82630_e124396_d_n15;
        var_qg_nqs_dn16 = assign82630_e124396_d_n16;
        var_qg_nqs_dn17 = assign82630_e124396_d_n17;
        var_qg_nqs_dn18 = assign82630_e124396_d_n18;
        var_qg_nqs_dn19 = assign82630_e124396_d_n19;
        var_qg_nqs_dn20 = assign82630_e124396_d_n20;

        let assign82640_e124399: f64 = if var_sigvds > 0.0 { 1.0 } else { 0.0 };
        var_guard2244 = assign82640_e124399;

        let (assign82650_e124409, assign82650_e124409_d_n5, assign82650_e124409_d_n6, assign82650_e124409_d_n7, assign82650_e124409_d_n8, assign82650_e124409_d_n12, assign82650_e124409_d_n13, assign82650_e124409_d_n14, assign82650_e124409_d_n15, assign82650_e124409_d_n16, assign82650_e124409_d_n17, assign82650_e124409_d_n18, assign82650_e124409_d_n19, assign82650_e124409_d_n20,) = {
    if ((var_guard2078 != 0.0) && (var_guard2244 != 0.0)) {
        let assign82650_e124405: f64 = (var_cox_qm * var_phit1_ac);
        let assign82650_e124407: f64 = (assign82650_e124405 * var_qs_nqs);
        (assign82650_e124407, ((((var_cox_qm_dn5 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn5)) * var_qs_nqs) + (assign82650_e124405 * var_qs_nqs_dn5)), ((((var_cox_qm_dn6 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn6)) * var_qs_nqs) + (assign82650_e124405 * var_qs_nqs_dn6)), ((((var_cox_qm_dn7 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn7)) * var_qs_nqs) + (assign82650_e124405 * var_qs_nqs_dn7)), ((((var_cox_qm_dn8 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn8)) * var_qs_nqs) + (assign82650_e124405 * var_qs_nqs_dn8)), ((((var_cox_qm_dn12 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn12)) * var_qs_nqs) + (assign82650_e124405 * var_qs_nqs_dn12)), ((((var_cox_qm_dn13 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn13)) * var_qs_nqs) + (assign82650_e124405 * var_qs_nqs_dn13)), ((((var_cox_qm_dn14 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn14)) * var_qs_nqs) + (assign82650_e124405 * var_qs_nqs_dn14)), ((((var_cox_qm_dn15 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn15)) * var_qs_nqs) + (assign82650_e124405 * var_qs_nqs_dn15)), ((((var_cox_qm_dn16 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn16)) * var_qs_nqs) + (assign82650_e124405 * var_qs_nqs_dn16)), ((((var_cox_qm_dn17 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn17)) * var_qs_nqs) + (assign82650_e124405 * var_qs_nqs_dn17)), ((((var_cox_qm_dn18 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn18)) * var_qs_nqs) + (assign82650_e124405 * var_qs_nqs_dn18)), ((((var_cox_qm_dn19 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn19)) * var_qs_nqs) + (assign82650_e124405 * var_qs_nqs_dn19)), ((((var_cox_qm_dn20 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn20)) * var_qs_nqs) + (assign82650_e124405 * var_qs_nqs_dn20)),)
    } else {
        (var_qs, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_dn12, var_qs_dn13, var_qs_dn14, var_qs_dn15, var_qs_dn16, var_qs_dn17, var_qs_dn18, var_qs_dn19, var_qs_dn20,)
    }
};
        var_qs = assign82650_e124409;
        var_qs_dn5 = assign82650_e124409_d_n5;
        var_qs_dn6 = assign82650_e124409_d_n6;
        var_qs_dn7 = assign82650_e124409_d_n7;
        var_qs_dn8 = assign82650_e124409_d_n8;
        var_qs_dn12 = assign82650_e124409_d_n12;
        var_qs_dn13 = assign82650_e124409_d_n13;
        var_qs_dn14 = assign82650_e124409_d_n14;
        var_qs_dn15 = assign82650_e124409_d_n15;
        var_qs_dn16 = assign82650_e124409_d_n16;
        var_qs_dn17 = assign82650_e124409_d_n17;
        var_qs_dn18 = assign82650_e124409_d_n18;
        var_qs_dn19 = assign82650_e124409_d_n19;
        var_qs_dn20 = assign82650_e124409_d_n20;

        let (assign82660_e124419, assign82660_e124419_d_n5, assign82660_e124419_d_n6, assign82660_e124419_d_n7, assign82660_e124419_d_n8, assign82660_e124419_d_n12, assign82660_e124419_d_n13, assign82660_e124419_d_n14, assign82660_e124419_d_n15, assign82660_e124419_d_n16, assign82660_e124419_d_n17, assign82660_e124419_d_n18, assign82660_e124419_d_n19, assign82660_e124419_d_n20,) = {
    if ((var_guard2078 != 0.0) && (var_guard2244 != 0.0)) {
        let assign82660_e124415: f64 = (var_cox_qm * var_phit1_ac);
        let assign82660_e124417: f64 = (assign82660_e124415 * var_qd_nqs);
        (assign82660_e124417, ((((var_cox_qm_dn5 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn5)) * var_qd_nqs) + (assign82660_e124415 * var_qd_nqs_dn5)), ((((var_cox_qm_dn6 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn6)) * var_qd_nqs) + (assign82660_e124415 * var_qd_nqs_dn6)), ((((var_cox_qm_dn7 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn7)) * var_qd_nqs) + (assign82660_e124415 * var_qd_nqs_dn7)), ((((var_cox_qm_dn8 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn8)) * var_qd_nqs) + (assign82660_e124415 * var_qd_nqs_dn8)), ((((var_cox_qm_dn12 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn12)) * var_qd_nqs) + (assign82660_e124415 * var_qd_nqs_dn12)), ((((var_cox_qm_dn13 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn13)) * var_qd_nqs) + (assign82660_e124415 * var_qd_nqs_dn13)), ((((var_cox_qm_dn14 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn14)) * var_qd_nqs) + (assign82660_e124415 * var_qd_nqs_dn14)), ((((var_cox_qm_dn15 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn15)) * var_qd_nqs) + (assign82660_e124415 * var_qd_nqs_dn15)), ((((var_cox_qm_dn16 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn16)) * var_qd_nqs) + (assign82660_e124415 * var_qd_nqs_dn16)), ((((var_cox_qm_dn17 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn17)) * var_qd_nqs) + (assign82660_e124415 * var_qd_nqs_dn17)), ((((var_cox_qm_dn18 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn18)) * var_qd_nqs) + (assign82660_e124415 * var_qd_nqs_dn18)), ((((var_cox_qm_dn19 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn19)) * var_qd_nqs) + (assign82660_e124415 * var_qd_nqs_dn19)), ((((var_cox_qm_dn20 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn20)) * var_qd_nqs) + (assign82660_e124415 * var_qd_nqs_dn20)),)
    } else {
        (var_qd, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_dn12, var_qd_dn13, var_qd_dn14, var_qd_dn15, var_qd_dn16, var_qd_dn17, var_qd_dn18, var_qd_dn19, var_qd_dn20,)
    }
};
        var_qd = assign82660_e124419;
        var_qd_dn5 = assign82660_e124419_d_n5;
        var_qd_dn6 = assign82660_e124419_d_n6;
        var_qd_dn7 = assign82660_e124419_d_n7;
        var_qd_dn8 = assign82660_e124419_d_n8;
        var_qd_dn12 = assign82660_e124419_d_n12;
        var_qd_dn13 = assign82660_e124419_d_n13;
        var_qd_dn14 = assign82660_e124419_d_n14;
        var_qd_dn15 = assign82660_e124419_d_n15;
        var_qd_dn16 = assign82660_e124419_d_n16;
        var_qd_dn17 = assign82660_e124419_d_n17;
        var_qd_dn18 = assign82660_e124419_d_n18;
        var_qd_dn19 = assign82660_e124419_d_n19;
        var_qd_dn20 = assign82660_e124419_d_n20;

        let (assign82670_e124430, assign82670_e124430_d_n5, assign82670_e124430_d_n6, assign82670_e124430_d_n7, assign82670_e124430_d_n8, assign82670_e124430_d_n12, assign82670_e124430_d_n13, assign82670_e124430_d_n14, assign82670_e124430_d_n15, assign82670_e124430_d_n16, assign82670_e124430_d_n17, assign82670_e124430_d_n18, assign82670_e124430_d_n19, assign82670_e124430_d_n20,) = {
    if ((var_guard2078 != 0.0) && (var_guard2244 == 0.0)) {
        let assign82670_e124426: f64 = (var_cox_qm * var_phit1_ac);
        let assign82670_e124428: f64 = (assign82670_e124426 * var_qd_nqs);
        (assign82670_e124428, ((((var_cox_qm_dn5 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn5)) * var_qd_nqs) + (assign82670_e124426 * var_qd_nqs_dn5)), ((((var_cox_qm_dn6 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn6)) * var_qd_nqs) + (assign82670_e124426 * var_qd_nqs_dn6)), ((((var_cox_qm_dn7 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn7)) * var_qd_nqs) + (assign82670_e124426 * var_qd_nqs_dn7)), ((((var_cox_qm_dn8 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn8)) * var_qd_nqs) + (assign82670_e124426 * var_qd_nqs_dn8)), ((((var_cox_qm_dn12 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn12)) * var_qd_nqs) + (assign82670_e124426 * var_qd_nqs_dn12)), ((((var_cox_qm_dn13 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn13)) * var_qd_nqs) + (assign82670_e124426 * var_qd_nqs_dn13)), ((((var_cox_qm_dn14 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn14)) * var_qd_nqs) + (assign82670_e124426 * var_qd_nqs_dn14)), ((((var_cox_qm_dn15 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn15)) * var_qd_nqs) + (assign82670_e124426 * var_qd_nqs_dn15)), ((((var_cox_qm_dn16 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn16)) * var_qd_nqs) + (assign82670_e124426 * var_qd_nqs_dn16)), ((((var_cox_qm_dn17 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn17)) * var_qd_nqs) + (assign82670_e124426 * var_qd_nqs_dn17)), ((((var_cox_qm_dn18 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn18)) * var_qd_nqs) + (assign82670_e124426 * var_qd_nqs_dn18)), ((((var_cox_qm_dn19 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn19)) * var_qd_nqs) + (assign82670_e124426 * var_qd_nqs_dn19)), ((((var_cox_qm_dn20 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn20)) * var_qd_nqs) + (assign82670_e124426 * var_qd_nqs_dn20)),)
    } else {
        (var_qs, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_dn12, var_qs_dn13, var_qs_dn14, var_qs_dn15, var_qs_dn16, var_qs_dn17, var_qs_dn18, var_qs_dn19, var_qs_dn20,)
    }
};
        var_qs = assign82670_e124430;
        var_qs_dn5 = assign82670_e124430_d_n5;
        var_qs_dn6 = assign82670_e124430_d_n6;
        var_qs_dn7 = assign82670_e124430_d_n7;
        var_qs_dn8 = assign82670_e124430_d_n8;
        var_qs_dn12 = assign82670_e124430_d_n12;
        var_qs_dn13 = assign82670_e124430_d_n13;
        var_qs_dn14 = assign82670_e124430_d_n14;
        var_qs_dn15 = assign82670_e124430_d_n15;
        var_qs_dn16 = assign82670_e124430_d_n16;
        var_qs_dn17 = assign82670_e124430_d_n17;
        var_qs_dn18 = assign82670_e124430_d_n18;
        var_qs_dn19 = assign82670_e124430_d_n19;
        var_qs_dn20 = assign82670_e124430_d_n20;

        let (assign82680_e124441, assign82680_e124441_d_n5, assign82680_e124441_d_n6, assign82680_e124441_d_n7, assign82680_e124441_d_n8, assign82680_e124441_d_n12, assign82680_e124441_d_n13, assign82680_e124441_d_n14, assign82680_e124441_d_n15, assign82680_e124441_d_n16, assign82680_e124441_d_n17, assign82680_e124441_d_n18, assign82680_e124441_d_n19, assign82680_e124441_d_n20,) = {
    if ((var_guard2078 != 0.0) && (var_guard2244 == 0.0)) {
        let assign82680_e124437: f64 = (var_cox_qm * var_phit1_ac);
        let assign82680_e124439: f64 = (assign82680_e124437 * var_qs_nqs);
        (assign82680_e124439, ((((var_cox_qm_dn5 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn5)) * var_qs_nqs) + (assign82680_e124437 * var_qs_nqs_dn5)), ((((var_cox_qm_dn6 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn6)) * var_qs_nqs) + (assign82680_e124437 * var_qs_nqs_dn6)), ((((var_cox_qm_dn7 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn7)) * var_qs_nqs) + (assign82680_e124437 * var_qs_nqs_dn7)), ((((var_cox_qm_dn8 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn8)) * var_qs_nqs) + (assign82680_e124437 * var_qs_nqs_dn8)), ((((var_cox_qm_dn12 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn12)) * var_qs_nqs) + (assign82680_e124437 * var_qs_nqs_dn12)), ((((var_cox_qm_dn13 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn13)) * var_qs_nqs) + (assign82680_e124437 * var_qs_nqs_dn13)), ((((var_cox_qm_dn14 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn14)) * var_qs_nqs) + (assign82680_e124437 * var_qs_nqs_dn14)), ((((var_cox_qm_dn15 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn15)) * var_qs_nqs) + (assign82680_e124437 * var_qs_nqs_dn15)), ((((var_cox_qm_dn16 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn16)) * var_qs_nqs) + (assign82680_e124437 * var_qs_nqs_dn16)), ((((var_cox_qm_dn17 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn17)) * var_qs_nqs) + (assign82680_e124437 * var_qs_nqs_dn17)), ((((var_cox_qm_dn18 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn18)) * var_qs_nqs) + (assign82680_e124437 * var_qs_nqs_dn18)), ((((var_cox_qm_dn19 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn19)) * var_qs_nqs) + (assign82680_e124437 * var_qs_nqs_dn19)), ((((var_cox_qm_dn20 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn20)) * var_qs_nqs) + (assign82680_e124437 * var_qs_nqs_dn20)),)
    } else {
        (var_qd, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_dn12, var_qd_dn13, var_qd_dn14, var_qd_dn15, var_qd_dn16, var_qd_dn17, var_qd_dn18, var_qd_dn19, var_qd_dn20,)
    }
};
        var_qd = assign82680_e124441;
        var_qd_dn5 = assign82680_e124441_d_n5;
        var_qd_dn6 = assign82680_e124441_d_n6;
        var_qd_dn7 = assign82680_e124441_d_n7;
        var_qd_dn8 = assign82680_e124441_d_n8;
        var_qd_dn12 = assign82680_e124441_d_n12;
        var_qd_dn13 = assign82680_e124441_d_n13;
        var_qd_dn14 = assign82680_e124441_d_n14;
        var_qd_dn15 = assign82680_e124441_d_n15;
        var_qd_dn16 = assign82680_e124441_d_n16;
        var_qd_dn17 = assign82680_e124441_d_n17;
        var_qd_dn18 = assign82680_e124441_d_n18;
        var_qd_dn19 = assign82680_e124441_d_n19;
        var_qd_dn20 = assign82680_e124441_d_n20;

        let (assign82690_e124449, assign82690_e124449_d_n5, assign82690_e124449_d_n6, assign82690_e124449_d_n7, assign82690_e124449_d_n8, assign82690_e124449_d_n12, assign82690_e124449_d_n13, assign82690_e124449_d_n14, assign82690_e124449_d_n15, assign82690_e124449_d_n16, assign82690_e124449_d_n17, assign82690_e124449_d_n18, assign82690_e124449_d_n19, assign82690_e124449_d_n20,) = {
    if (var_guard2078 != 0.0) {
        let assign82690_e124445: f64 = (var_cox_qm * var_phit1_ac);
        let assign82690_e124447: f64 = (assign82690_e124445 * var_qg_nqs);
        (assign82690_e124447, ((((var_cox_qm_dn5 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn5)) * var_qg_nqs) + (assign82690_e124445 * var_qg_nqs_dn5)), ((((var_cox_qm_dn6 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn6)) * var_qg_nqs) + (assign82690_e124445 * var_qg_nqs_dn6)), ((((var_cox_qm_dn7 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn7)) * var_qg_nqs) + (assign82690_e124445 * var_qg_nqs_dn7)), ((((var_cox_qm_dn8 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn8)) * var_qg_nqs) + (assign82690_e124445 * var_qg_nqs_dn8)), ((((var_cox_qm_dn12 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn12)) * var_qg_nqs) + (assign82690_e124445 * var_qg_nqs_dn12)), ((((var_cox_qm_dn13 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn13)) * var_qg_nqs) + (assign82690_e124445 * var_qg_nqs_dn13)), ((((var_cox_qm_dn14 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn14)) * var_qg_nqs) + (assign82690_e124445 * var_qg_nqs_dn14)), ((((var_cox_qm_dn15 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn15)) * var_qg_nqs) + (assign82690_e124445 * var_qg_nqs_dn15)), ((((var_cox_qm_dn16 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn16)) * var_qg_nqs) + (assign82690_e124445 * var_qg_nqs_dn16)), ((((var_cox_qm_dn17 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn17)) * var_qg_nqs) + (assign82690_e124445 * var_qg_nqs_dn17)), ((((var_cox_qm_dn18 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn18)) * var_qg_nqs) + (assign82690_e124445 * var_qg_nqs_dn18)), ((((var_cox_qm_dn19 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn19)) * var_qg_nqs) + (assign82690_e124445 * var_qg_nqs_dn19)), ((((var_cox_qm_dn20 * var_phit1_ac) + (var_cox_qm * var_phit1_ac_dn20)) * var_qg_nqs) + (assign82690_e124445 * var_qg_nqs_dn20)),)
    } else {
        (var_qg, var_qg_dn5, var_qg_dn6, var_qg_dn7, var_qg_dn8, var_qg_dn12, var_qg_dn13, var_qg_dn14, var_qg_dn15, var_qg_dn16, var_qg_dn17, var_qg_dn18, var_qg_dn19, var_qg_dn20,)
    }
};
        var_qg = assign82690_e124449;
        var_qg_dn5 = assign82690_e124449_d_n5;
        var_qg_dn6 = assign82690_e124449_d_n6;
        var_qg_dn7 = assign82690_e124449_d_n7;
        var_qg_dn8 = assign82690_e124449_d_n8;
        var_qg_dn12 = assign82690_e124449_d_n12;
        var_qg_dn13 = assign82690_e124449_d_n13;
        var_qg_dn14 = assign82690_e124449_d_n14;
        var_qg_dn15 = assign82690_e124449_d_n15;
        var_qg_dn16 = assign82690_e124449_d_n16;
        var_qg_dn17 = assign82690_e124449_d_n17;
        var_qg_dn18 = assign82690_e124449_d_n18;
        var_qg_dn19 = assign82690_e124449_d_n19;
        var_qg_dn20 = assign82690_e124449_d_n20;

        let (assign82700_e124458, assign82700_e124458_d_n5, assign82700_e124458_d_n6, assign82700_e124458_d_n7, assign82700_e124458_d_n8, assign82700_e124458_d_n12, assign82700_e124458_d_n13, assign82700_e124458_d_n14, assign82700_e124458_d_n15, assign82700_e124458_d_n16, assign82700_e124458_d_n17, assign82700_e124458_d_n18, assign82700_e124458_d_n19, assign82700_e124458_d_n20,) = {
    if (var_guard2078 != 0.0) {
        let assign82700_e124452: f64 = (-var_qg);
        let assign82700_e124454: f64 = (assign82700_e124452 - var_qs);
        let assign82700_e124456: f64 = (assign82700_e124454 - var_qd);
        (assign82700_e124456, (((-var_qg_dn5) - var_qs_dn5) - var_qd_dn5), (((-var_qg_dn6) - var_qs_dn6) - var_qd_dn6), (((-var_qg_dn7) - var_qs_dn7) - var_qd_dn7), (((-var_qg_dn8) - var_qs_dn8) - var_qd_dn8), (((-var_qg_dn12) - var_qs_dn12) - var_qd_dn12), (((-var_qg_dn13) - var_qs_dn13) - var_qd_dn13), (((-var_qg_dn14) - var_qs_dn14) - var_qd_dn14), (((-var_qg_dn15) - var_qs_dn15) - var_qd_dn15), (((-var_qg_dn16) - var_qs_dn16) - var_qd_dn16), (((-var_qg_dn17) - var_qs_dn17) - var_qd_dn17), (((-var_qg_dn18) - var_qs_dn18) - var_qd_dn18), (((-var_qg_dn19) - var_qs_dn19) - var_qd_dn19), (((-var_qg_dn20) - var_qs_dn20) - var_qd_dn20),)
    } else {
        (var_qb, var_qb_dn5, var_qb_dn6, var_qb_dn7, var_qb_dn8, var_qb_dn12, var_qb_dn13, var_qb_dn14, var_qb_dn15, var_qb_dn16, var_qb_dn17, var_qb_dn18, var_qb_dn19, var_qb_dn20,)
    }
};
        var_qb = assign82700_e124458;
        var_qb_dn5 = assign82700_e124458_d_n5;
        var_qb_dn6 = assign82700_e124458_d_n6;
        var_qb_dn7 = assign82700_e124458_d_n7;
        var_qb_dn8 = assign82700_e124458_d_n8;
        var_qb_dn12 = assign82700_e124458_d_n12;
        var_qb_dn13 = assign82700_e124458_d_n13;
        var_qb_dn14 = assign82700_e124458_d_n14;
        var_qb_dn15 = assign82700_e124458_d_n15;
        var_qb_dn16 = assign82700_e124458_d_n16;
        var_qb_dn17 = assign82700_e124458_d_n17;
        var_qb_dn18 = assign82700_e124458_d_n18;
        var_qb_dn19 = assign82700_e124458_d_n19;
        var_qb_dn20 = assign82700_e124458_d_n20;

        let assign82710_e124461: f64 = (var_qg + var_qb);
        let assign82710_e124463: f64 = (assign82710_e124461 + var_qd);
        let assign82710_e124464: f64 = (-assign82710_e124463);
        var_qs = assign82710_e124464;
        var_qs_dn5 = (-((var_qg_dn5 + var_qb_dn5) + var_qd_dn5));
        var_qs_dn6 = (-((var_qg_dn6 + var_qb_dn6) + var_qd_dn6));
        var_qs_dn7 = (-((var_qg_dn7 + var_qb_dn7) + var_qd_dn7));
        var_qs_dn8 = (-((var_qg_dn8 + var_qb_dn8) + var_qd_dn8));
        var_qs_dn12 = (-((var_qg_dn12 + var_qb_dn12) + var_qd_dn12));
        var_qs_dn13 = (-((var_qg_dn13 + var_qb_dn13) + var_qd_dn13));
        var_qs_dn14 = (-((var_qg_dn14 + var_qb_dn14) + var_qd_dn14));
        var_qs_dn15 = (-((var_qg_dn15 + var_qb_dn15) + var_qd_dn15));
        var_qs_dn16 = (-((var_qg_dn16 + var_qb_dn16) + var_qd_dn16));
        var_qs_dn17 = (-((var_qg_dn17 + var_qb_dn17) + var_qd_dn17));
        var_qs_dn18 = (-((var_qg_dn18 + var_qb_dn18) + var_qd_dn18));
        var_qs_dn19 = (-((var_qg_dn19 + var_qb_dn19) + var_qd_dn19));
        var_qs_dn20 = (-((var_qg_dn20 + var_qb_dn20) + var_qd_dn20));

        let assign82720_e124467: f64 = (var_qfgs + var_qgs_ov);
        var_qfgs = assign82720_e124467;
        var_qfgs_dn5 = (var_qfgs_dn5 + var_qgs_ov_dn5);
        var_qfgs_dn6 = (var_qfgs_dn6 + var_qgs_ov_dn6);
        var_qfgs_dn7 = (var_qfgs_dn7 + var_qgs_ov_dn7);

        let assign82730_e124470: f64 = (var_qfgd + var_qgd_ov);
        var_qfgd = assign82730_e124470;
        var_qfgd_dn5 = (var_qfgd_dn5 + var_qgd_ov_dn5);
        var_qfgd_dn6 = (var_qfgd_dn6 + var_qgd_ov_dn6);
        var_qfgd_dn7 = (var_qfgd_dn7 + var_qgd_ov_dn7);

        let assign82740_e124473: f64 = (var_absource_i * var_qjunbot_s);
        let assign82740_e124476: f64 = (var_lssource_i * var_qjunsti_s);
        let assign82740_e124477: f64 = (assign82740_e124473 + assign82740_e124476);
        let assign82740_e124480: f64 = (var_lgsource_i * var_qjungat_s);
        let assign82740_e124481: f64 = (assign82740_e124477 + assign82740_e124480);
        var_qjun_s = assign82740_e124481;
        var_qjun_s_dn5 = (((var_absource_i * var_qjunbot_s_dn5) + (var_lssource_i * var_qjunsti_s_dn5)) + (var_lgsource_i * var_qjungat_s_dn5));
        var_qjun_s_dn6 = (((var_absource_i * var_qjunbot_s_dn6) + (var_lssource_i * var_qjunsti_s_dn6)) + (var_lgsource_i * var_qjungat_s_dn6));
        var_qjun_s_dn7 = (((var_absource_i * var_qjunbot_s_dn7) + (var_lssource_i * var_qjunsti_s_dn7)) + (var_lgsource_i * var_qjungat_s_dn7));
        var_qjun_s_dn8 = (((var_absource_i * var_qjunbot_s_dn8) + (var_lssource_i * var_qjunsti_s_dn8)) + (var_lgsource_i * var_qjungat_s_dn8));
        var_qjun_s_dn10 = (((var_absource_i * var_qjunbot_s_dn10) + (var_lssource_i * var_qjunsti_s_dn10)) + (var_lgsource_i * var_qjungat_s_dn10));
        var_qjun_s_dn11 = (((var_absource_i * var_qjunbot_s_dn11) + (var_lssource_i * var_qjunsti_s_dn11)) + (var_lgsource_i * var_qjungat_s_dn11));

        *var_guard2243_slot = var_guard2243;
        *var_guard2244_slot = var_guard2244;
        *var_nqs_d0_slot = var_nqs_d0;
        *var_nqs_d0_dn12_slot = var_nqs_d0_dn12;
        *var_nqs_d0_dn13_slot = var_nqs_d0_dn13;
        *var_nqs_d0_dn14_slot = var_nqs_d0_dn14;
        *var_nqs_d0_dn15_slot = var_nqs_d0_dn15;
        *var_nqs_d0_dn16_slot = var_nqs_d0_dn16;
        *var_nqs_d0_dn17_slot = var_nqs_d0_dn17;
        *var_nqs_d0_dn18_slot = var_nqs_d0_dn18;
        *var_nqs_d0_dn19_slot = var_nqs_d0_dn19;
        *var_nqs_d0_dn20_slot = var_nqs_d0_dn20;
        *var_nqs_d0_dn5_slot = var_nqs_d0_dn5;
        *var_nqs_d0_dn6_slot = var_nqs_d0_dn6;
        *var_nqs_d0_dn7_slot = var_nqs_d0_dn7;
        *var_nqs_d0_dn8_slot = var_nqs_d0_dn8;
        *var_nqs_p_slot = var_nqs_p;
        *var_nqs_p_dn12_slot = var_nqs_p_dn12;
        *var_nqs_p_dn13_slot = var_nqs_p_dn13;
        *var_nqs_p_dn14_slot = var_nqs_p_dn14;
        *var_nqs_p_dn15_slot = var_nqs_p_dn15;
        *var_nqs_p_dn16_slot = var_nqs_p_dn16;
        *var_nqs_p_dn17_slot = var_nqs_p_dn17;
        *var_nqs_p_dn18_slot = var_nqs_p_dn18;
        *var_nqs_p_dn19_slot = var_nqs_p_dn19;
        *var_nqs_p_dn20_slot = var_nqs_p_dn20;
        *var_nqs_p_dn5_slot = var_nqs_p_dn5;
        *var_nqs_p_dn6_slot = var_nqs_p_dn6;
        *var_nqs_p_dn7_slot = var_nqs_p_dn7;
        *var_nqs_p_dn8_slot = var_nqs_p_dn8;
        *var_nqs_q_slot = var_nqs_q;
        *var_nqs_q_dn12_slot = var_nqs_q_dn12;
        *var_nqs_q_dn13_slot = var_nqs_q_dn13;
        *var_nqs_q_dn14_slot = var_nqs_q_dn14;
        *var_nqs_q_dn15_slot = var_nqs_q_dn15;
        *var_nqs_q_dn16_slot = var_nqs_q_dn16;
        *var_nqs_q_dn17_slot = var_nqs_q_dn17;
        *var_nqs_q_dn18_slot = var_nqs_q_dn18;
        *var_nqs_q_dn19_slot = var_nqs_q_dn19;
        *var_nqs_q_dn20_slot = var_nqs_q_dn20;
        *var_nqs_q_dn5_slot = var_nqs_q_dn5;
        *var_nqs_q_dn6_slot = var_nqs_q_dn6;
        *var_nqs_q_dn7_slot = var_nqs_q_dn7;
        *var_nqs_q_dn8_slot = var_nqs_q_dn8;
        *var_nqs_temp_slot = var_nqs_temp;
        *var_nqs_temp_dn12_slot = var_nqs_temp_dn12;
        *var_nqs_temp_dn13_slot = var_nqs_temp_dn13;
        *var_nqs_temp_dn14_slot = var_nqs_temp_dn14;
        *var_nqs_temp_dn15_slot = var_nqs_temp_dn15;
        *var_nqs_temp_dn16_slot = var_nqs_temp_dn16;
        *var_nqs_temp_dn17_slot = var_nqs_temp_dn17;
        *var_nqs_temp_dn18_slot = var_nqs_temp_dn18;
        *var_nqs_temp_dn19_slot = var_nqs_temp_dn19;
        *var_nqs_temp_dn20_slot = var_nqs_temp_dn20;
        *var_nqs_temp_dn5_slot = var_nqs_temp_dn5;
        *var_nqs_temp_dn6_slot = var_nqs_temp_dn6;
        *var_nqs_temp_dn7_slot = var_nqs_temp_dn7;
        *var_nqs_temp_dn8_slot = var_nqs_temp_dn8;
        *var_nqs_u_slot = var_nqs_u;
        *var_nqs_u_dn12_slot = var_nqs_u_dn12;
        *var_nqs_u_dn13_slot = var_nqs_u_dn13;
        *var_nqs_u_dn14_slot = var_nqs_u_dn14;
        *var_nqs_u_dn15_slot = var_nqs_u_dn15;
        *var_nqs_u_dn16_slot = var_nqs_u_dn16;
        *var_nqs_u_dn17_slot = var_nqs_u_dn17;
        *var_nqs_u_dn18_slot = var_nqs_u_dn18;
        *var_nqs_u_dn19_slot = var_nqs_u_dn19;
        *var_nqs_u_dn20_slot = var_nqs_u_dn20;
        *var_nqs_u_dn5_slot = var_nqs_u_dn5;
        *var_nqs_u_dn6_slot = var_nqs_u_dn6;
        *var_nqs_u_dn7_slot = var_nqs_u_dn7;
        *var_nqs_u_dn8_slot = var_nqs_u_dn8;
        *var_nqs_xi_slot = var_nqs_xi;
        *var_nqs_xi_dn12_slot = var_nqs_xi_dn12;
        *var_nqs_xi_dn13_slot = var_nqs_xi_dn13;
        *var_nqs_xi_dn14_slot = var_nqs_xi_dn14;
        *var_nqs_xi_dn15_slot = var_nqs_xi_dn15;
        *var_nqs_xi_dn16_slot = var_nqs_xi_dn16;
        *var_nqs_xi_dn17_slot = var_nqs_xi_dn17;
        *var_nqs_xi_dn18_slot = var_nqs_xi_dn18;
        *var_nqs_xi_dn19_slot = var_nqs_xi_dn19;
        *var_nqs_xi_dn20_slot = var_nqs_xi_dn20;
        *var_nqs_xi_dn5_slot = var_nqs_xi_dn5;
        *var_nqs_xi_dn6_slot = var_nqs_xi_dn6;
        *var_nqs_xi_dn7_slot = var_nqs_xi_dn7;
        *var_nqs_xi_dn8_slot = var_nqs_xi_dn8;
        *var_qb_slot = var_qb;
        *var_qb_dn12_slot = var_qb_dn12;
        *var_qb_dn13_slot = var_qb_dn13;
        *var_qb_dn14_slot = var_qb_dn14;
        *var_qb_dn15_slot = var_qb_dn15;
        *var_qb_dn16_slot = var_qb_dn16;
        *var_qb_dn17_slot = var_qb_dn17;
        *var_qb_dn18_slot = var_qb_dn18;
        *var_qb_dn19_slot = var_qb_dn19;
        *var_qb_dn20_slot = var_qb_dn20;
        *var_qb_dn5_slot = var_qb_dn5;
        *var_qb_dn6_slot = var_qb_dn6;
        *var_qb_dn7_slot = var_qb_dn7;
        *var_qb_dn8_slot = var_qb_dn8;
        *var_qd_slot = var_qd;
        *var_qd_dn12_slot = var_qd_dn12;
        *var_qd_dn13_slot = var_qd_dn13;
        *var_qd_dn14_slot = var_qd_dn14;
        *var_qd_dn15_slot = var_qd_dn15;
        *var_qd_dn16_slot = var_qd_dn16;
        *var_qd_dn17_slot = var_qd_dn17;
        *var_qd_dn18_slot = var_qd_dn18;
        *var_qd_dn19_slot = var_qd_dn19;
        *var_qd_dn20_slot = var_qd_dn20;
        *var_qd_dn5_slot = var_qd_dn5;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qfgd_slot = var_qfgd;
        *var_qfgd_dn5_slot = var_qfgd_dn5;
        *var_qfgd_dn6_slot = var_qfgd_dn6;
        *var_qfgd_dn7_slot = var_qfgd_dn7;
        *var_qfgs_slot = var_qfgs;
        *var_qfgs_dn5_slot = var_qfgs_dn5;
        *var_qfgs_dn6_slot = var_qfgs_dn6;
        *var_qfgs_dn7_slot = var_qfgs_dn7;
        *var_qg_slot = var_qg;
        *var_qg_dn12_slot = var_qg_dn12;
        *var_qg_dn13_slot = var_qg_dn13;
        *var_qg_dn14_slot = var_qg_dn14;
        *var_qg_dn15_slot = var_qg_dn15;
        *var_qg_dn16_slot = var_qg_dn16;
        *var_qg_dn17_slot = var_qg_dn17;
        *var_qg_dn18_slot = var_qg_dn18;
        *var_qg_dn19_slot = var_qg_dn19;
        *var_qg_dn20_slot = var_qg_dn20;
        *var_qg_dn5_slot = var_qg_dn5;
        *var_qg_dn6_slot = var_qg_dn6;
        *var_qg_dn7_slot = var_qg_dn7;
        *var_qg_dn8_slot = var_qg_dn8;
        *var_qg_nqs_slot = var_qg_nqs;
        *var_qg_nqs_dn12_slot = var_qg_nqs_dn12;
        *var_qg_nqs_dn13_slot = var_qg_nqs_dn13;
        *var_qg_nqs_dn14_slot = var_qg_nqs_dn14;
        *var_qg_nqs_dn15_slot = var_qg_nqs_dn15;
        *var_qg_nqs_dn16_slot = var_qg_nqs_dn16;
        *var_qg_nqs_dn17_slot = var_qg_nqs_dn17;
        *var_qg_nqs_dn18_slot = var_qg_nqs_dn18;
        *var_qg_nqs_dn19_slot = var_qg_nqs_dn19;
        *var_qg_nqs_dn20_slot = var_qg_nqs_dn20;
        *var_qg_nqs_dn5_slot = var_qg_nqs_dn5;
        *var_qg_nqs_dn6_slot = var_qg_nqs_dn6;
        *var_qg_nqs_dn7_slot = var_qg_nqs_dn7;
        *var_qg_nqs_dn8_slot = var_qg_nqs_dn8;
        *var_qjun_s_slot = var_qjun_s;
        *var_qjun_s_dn10_slot = var_qjun_s_dn10;
        *var_qjun_s_dn11_slot = var_qjun_s_dn11;
        *var_qjun_s_dn5_slot = var_qjun_s_dn5;
        *var_qjun_s_dn6_slot = var_qjun_s_dn6;
        *var_qjun_s_dn7_slot = var_qjun_s_dn7;
        *var_qjun_s_dn8_slot = var_qjun_s_dn8;
        *var_qs_slot = var_qs;
        *var_qs_dn12_slot = var_qs_dn12;
        *var_qs_dn13_slot = var_qs_dn13;
        *var_qs_dn14_slot = var_qs_dn14;
        *var_qs_dn15_slot = var_qs_dn15;
        *var_qs_dn16_slot = var_qs_dn16;
        *var_qs_dn17_slot = var_qs_dn17;
        *var_qs_dn18_slot = var_qs_dn18;
        *var_qs_dn19_slot = var_qs_dn19;
        *var_qs_dn20_slot = var_qs_dn20;
        *var_qs_dn5_slot = var_qs_dn5;
        *var_qs_dn6_slot = var_qs_dn6;
        *var_qs_dn7_slot = var_qs_dn7;
        *var_qs_dn8_slot = var_qs_dn8;
        *var_temp9_slot = var_temp9;
        *var_temp9_dn12_slot = var_temp9_dn12;
        *var_temp9_dn13_slot = var_temp9_dn13;
        *var_temp9_dn14_slot = var_temp9_dn14;
        *var_temp9_dn15_slot = var_temp9_dn15;
        *var_temp9_dn16_slot = var_temp9_dn16;
        *var_temp9_dn17_slot = var_temp9_dn17;
        *var_temp9_dn18_slot = var_temp9_dn18;
        *var_temp9_dn19_slot = var_temp9_dn19;
        *var_temp9_dn20_slot = var_temp9_dn20;
        *var_temp9_dn5_slot = var_temp9_dn5;
        *var_temp9_dn6_slot = var_temp9_dn6;
        *var_temp9_dn7_slot = var_temp9_dn7;
        *var_temp9_dn8_slot = var_temp9_dn8;
    }

    pub(super) fn stamp_transient_block_259(
        p: &Parameters,
        var_abdrain_i: f64,
        var_alpha_dc: f64,
        var_alpha_dc_dn12: f64,
        var_alpha_dc_dn13: f64,
        var_alpha_dc_dn14: f64,
        var_alpha_dc_dn15: f64,
        var_alpha_dc_dn16: f64,
        var_alpha_dc_dn17: f64,
        var_alpha_dc_dn18: f64,
        var_alpha_dc_dn19: f64,
        var_alpha_dc_dn20: f64,
        var_alpha_dc_dn5: f64,
        var_alpha_dc_dn6: f64,
        var_alpha_dc_dn7: f64,
        var_alpha_dc_dn8: f64,
        var_bet_i: f64,
        var_cox_qm: f64,
        var_cox_qm_dn12: f64,
        var_cox_qm_dn13: f64,
        var_cox_qm_dn14: f64,
        var_cox_qm_dn15: f64,
        var_cox_qm_dn16: f64,
        var_cox_qm_dn17: f64,
        var_cox_qm_dn18: f64,
        var_cox_qm_dn19: f64,
        var_cox_qm_dn20: f64,
        var_cox_qm_dn5: f64,
        var_cox_qm_dn6: f64,
        var_cox_qm_dn7: f64,
        var_cox_qm_dn8: f64,
        var_dps_dc: f64,
        var_dps_dc_dn12: f64,
        var_dps_dc_dn13: f64,
        var_dps_dc_dn14: f64,
        var_dps_dc_dn15: f64,
        var_dps_dc_dn16: f64,
        var_dps_dc_dn17: f64,
        var_dps_dc_dn18: f64,
        var_dps_dc_dn19: f64,
        var_dps_dc_dn20: f64,
        var_dps_dc_dn5: f64,
        var_dps_dc_dn6: f64,
        var_dps_dc_dn7: f64,
        var_dps_dc_dn8: f64,
        var_eta_p_ac: f64,
        var_eta_p_ac_dn12: f64,
        var_eta_p_ac_dn13: f64,
        var_eta_p_ac_dn14: f64,
        var_eta_p_ac_dn15: f64,
        var_eta_p_ac_dn16: f64,
        var_eta_p_ac_dn17: f64,
        var_eta_p_ac_dn18: f64,
        var_eta_p_ac_dn19: f64,
        var_eta_p_ac_dn20: f64,
        var_eta_p_ac_dn5: f64,
        var_eta_p_ac_dn6: f64,
        var_eta_p_ac_dn7: f64,
        var_eta_p_ac_dn8: f64,
        var_fntexc_i: f64,
        var_gvsatinv_dc: f64,
        var_gvsatinv_dc_dn12: f64,
        var_gvsatinv_dc_dn13: f64,
        var_gvsatinv_dc_dn14: f64,
        var_gvsatinv_dc_dn15: f64,
        var_gvsatinv_dc_dn16: f64,
        var_gvsatinv_dc_dn17: f64,
        var_gvsatinv_dc_dn18: f64,
        var_gvsatinv_dc_dn19: f64,
        var_gvsatinv_dc_dn20: f64,
        var_gvsatinv_dc_dn5: f64,
        var_gvsatinv_dc_dn6: f64,
        var_gvsatinv_dc_dn7: f64,
        var_gvsatinv_dc_dn8: f64,
        var_h_dc: f64,
        var_h_dc_dn12: f64,
        var_h_dc_dn13: f64,
        var_h_dc_dn14: f64,
        var_h_dc_dn15: f64,
        var_h_dc_dn16: f64,
        var_h_dc_dn17: f64,
        var_h_dc_dn18: f64,
        var_h_dc_dn19: f64,
        var_h_dc_dn20: f64,
        var_h_dc_dn5: f64,
        var_h_dc_dn6: f64,
        var_h_dc_dn7: f64,
        var_h_dc_dn8: f64,
        var_lgdrain_i: f64,
        var_lsdrain_i: f64,
        var_qim1_dc: f64,
        var_qim1_dc_dn12: f64,
        var_qim1_dc_dn13: f64,
        var_qim1_dc_dn14: f64,
        var_qim1_dc_dn15: f64,
        var_qim1_dc_dn16: f64,
        var_qim1_dc_dn17: f64,
        var_qim1_dc_dn18: f64,
        var_qim1_dc_dn19: f64,
        var_qim1_dc_dn20: f64,
        var_qim1_dc_dn5: f64,
        var_qim1_dc_dn6: f64,
        var_qim1_dc_dn7: f64,
        var_qim1_dc_dn8: f64,
        var_qim_dc: f64,
        var_qim_dc_dn12: f64,
        var_qim_dc_dn13: f64,
        var_qim_dc_dn14: f64,
        var_qim_dc_dn15: f64,
        var_qim_dc_dn16: f64,
        var_qim_dc_dn17: f64,
        var_qim_dc_dn18: f64,
        var_qim_dc_dn19: f64,
        var_qim_dc_dn20: f64,
        var_qim_dc_dn5: f64,
        var_qim_dc_dn6: f64,
        var_qim_dc_dn7: f64,
        var_qim_dc_dn8: f64,
        var_qjunbot_d: f64,
        var_qjunbot_d_dn10: f64,
        var_qjunbot_d_dn11: f64,
        var_qjunbot_d_dn5: f64,
        var_qjunbot_d_dn6: f64,
        var_qjunbot_d_dn7: f64,
        var_qjunbot_d_dn8: f64,
        var_qjungat_d: f64,
        var_qjungat_d_dn10: f64,
        var_qjungat_d_dn11: f64,
        var_qjungat_d_dn5: f64,
        var_qjungat_d_dn6: f64,
        var_qjungat_d_dn7: f64,
        var_qjungat_d_dn8: f64,
        var_qjunsti_d: f64,
        var_qjunsti_d_dn10: f64,
        var_qjunsti_d_dn11: f64,
        var_qjunsti_d_dn5: f64,
        var_qjunsti_d_dn6: f64,
        var_qjunsti_d_dn7: f64,
        var_qjunsti_d_dn8: f64,
        var_sigvds: f64,
        var_xg_dc: f64,
        var_c_igid_slot: &mut f64,
        var_c_igid_dn12_slot: &mut f64,
        var_c_igid_dn13_slot: &mut f64,
        var_c_igid_dn14_slot: &mut f64,
        var_c_igid_dn15_slot: &mut f64,
        var_c_igid_dn16_slot: &mut f64,
        var_c_igid_dn17_slot: &mut f64,
        var_c_igid_dn18_slot: &mut f64,
        var_c_igid_dn19_slot: &mut f64,
        var_c_igid_dn20_slot: &mut f64,
        var_c_igid_dn5_slot: &mut f64,
        var_c_igid_dn6_slot: &mut f64,
        var_c_igid_dn7_slot: &mut f64,
        var_c_igid_dn8_slot: &mut f64,
        var_cgeff_slot: &mut f64,
        var_cgeff_dn12_slot: &mut f64,
        var_cgeff_dn13_slot: &mut f64,
        var_cgeff_dn14_slot: &mut f64,
        var_cgeff_dn15_slot: &mut f64,
        var_cgeff_dn16_slot: &mut f64,
        var_cgeff_dn17_slot: &mut f64,
        var_cgeff_dn18_slot: &mut f64,
        var_cgeff_dn19_slot: &mut f64,
        var_cgeff_dn20_slot: &mut f64,
        var_cgeff_dn5_slot: &mut f64,
        var_cgeff_dn6_slot: &mut f64,
        var_cgeff_dn7_slot: &mut f64,
        var_cgeff_dn8_slot: &mut f64,
        var_g_ideal_slot: &mut f64,
        var_g_ideal_dn12_slot: &mut f64,
        var_g_ideal_dn13_slot: &mut f64,
        var_g_ideal_dn14_slot: &mut f64,
        var_g_ideal_dn15_slot: &mut f64,
        var_g_ideal_dn16_slot: &mut f64,
        var_g_ideal_dn17_slot: &mut f64,
        var_g_ideal_dn18_slot: &mut f64,
        var_g_ideal_dn19_slot: &mut f64,
        var_g_ideal_dn20_slot: &mut f64,
        var_g_ideal_dn5_slot: &mut f64,
        var_g_ideal_dn6_slot: &mut f64,
        var_g_ideal_dn7_slot: &mut f64,
        var_g_ideal_dn8_slot: &mut f64,
        var_guard2246_slot: &mut f64,
        var_guard2279_slot: &mut f64,
        var_guard2281_slot: &mut f64,
        var_guard2282_slot: &mut f64,
        var_h0_slot: &mut f64,
        var_h0_dn12_slot: &mut f64,
        var_h0_dn13_slot: &mut f64,
        var_h0_dn14_slot: &mut f64,
        var_h0_dn15_slot: &mut f64,
        var_h0_dn16_slot: &mut f64,
        var_h0_dn17_slot: &mut f64,
        var_h0_dn18_slot: &mut f64,
        var_h0_dn19_slot: &mut f64,
        var_h0_dn20_slot: &mut f64,
        var_h0_dn5_slot: &mut f64,
        var_h0_dn6_slot: &mut f64,
        var_h0_dn7_slot: &mut f64,
        var_h0_dn8_slot: &mut f64,
        var_lc_slot: &mut f64,
        var_lc_dn12_slot: &mut f64,
        var_lc_dn13_slot: &mut f64,
        var_lc_dn14_slot: &mut f64,
        var_lc_dn15_slot: &mut f64,
        var_lc_dn16_slot: &mut f64,
        var_lc_dn17_slot: &mut f64,
        var_lc_dn18_slot: &mut f64,
        var_lc_dn19_slot: &mut f64,
        var_lc_dn20_slot: &mut f64,
        var_lc_dn5_slot: &mut f64,
        var_lc_dn6_slot: &mut f64,
        var_lc_dn7_slot: &mut f64,
        var_lc_dn8_slot: &mut f64,
        var_lcinv2_slot: &mut f64,
        var_lcinv2_dn12_slot: &mut f64,
        var_lcinv2_dn13_slot: &mut f64,
        var_lcinv2_dn14_slot: &mut f64,
        var_lcinv2_dn15_slot: &mut f64,
        var_lcinv2_dn16_slot: &mut f64,
        var_lcinv2_dn17_slot: &mut f64,
        var_lcinv2_dn18_slot: &mut f64,
        var_lcinv2_dn19_slot: &mut f64,
        var_lcinv2_dn20_slot: &mut f64,
        var_lcinv2_dn5_slot: &mut f64,
        var_lcinv2_dn6_slot: &mut f64,
        var_lcinv2_dn7_slot: &mut f64,
        var_lcinv2_dn8_slot: &mut f64,
        var_mid_slot: &mut f64,
        var_mid_dn12_slot: &mut f64,
        var_mid_dn13_slot: &mut f64,
        var_mid_dn14_slot: &mut f64,
        var_mid_dn15_slot: &mut f64,
        var_mid_dn16_slot: &mut f64,
        var_mid_dn17_slot: &mut f64,
        var_mid_dn18_slot: &mut f64,
        var_mid_dn19_slot: &mut f64,
        var_mid_dn20_slot: &mut f64,
        var_mid_dn5_slot: &mut f64,
        var_mid_dn6_slot: &mut f64,
        var_mid_dn7_slot: &mut f64,
        var_mid_dn8_slot: &mut f64,
        var_mig_slot: &mut f64,
        var_mig_dn12_slot: &mut f64,
        var_mig_dn13_slot: &mut f64,
        var_mig_dn14_slot: &mut f64,
        var_mig_dn15_slot: &mut f64,
        var_mig_dn16_slot: &mut f64,
        var_mig_dn17_slot: &mut f64,
        var_mig_dn18_slot: &mut f64,
        var_mig_dn19_slot: &mut f64,
        var_mig_dn20_slot: &mut f64,
        var_mig_dn5_slot: &mut f64,
        var_mig_dn6_slot: &mut f64,
        var_mig_dn7_slot: &mut f64,
        var_mig_dn8_slot: &mut f64,
        var_migid_slot: &mut f64,
        var_migid_dn12_slot: &mut f64,
        var_migid_dn13_slot: &mut f64,
        var_migid_dn14_slot: &mut f64,
        var_migid_dn15_slot: &mut f64,
        var_migid_dn16_slot: &mut f64,
        var_migid_dn17_slot: &mut f64,
        var_migid_dn18_slot: &mut f64,
        var_migid_dn19_slot: &mut f64,
        var_migid_dn20_slot: &mut f64,
        var_migid_dn5_slot: &mut f64,
        var_migid_dn6_slot: &mut f64,
        var_migid_dn7_slot: &mut f64,
        var_migid_dn8_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn12_slot: &mut f64,
        var_qd_dn13_slot: &mut f64,
        var_qd_dn14_slot: &mut f64,
        var_qd_dn15_slot: &mut f64,
        var_qd_dn16_slot: &mut f64,
        var_qd_dn17_slot: &mut f64,
        var_qd_dn18_slot: &mut f64,
        var_qd_dn19_slot: &mut f64,
        var_qd_dn20_slot: &mut f64,
        var_qd_dn5_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qjun_d_slot: &mut f64,
        var_qjun_d_dn10_slot: &mut f64,
        var_qjun_d_dn11_slot: &mut f64,
        var_qjun_d_dn5_slot: &mut f64,
        var_qjun_d_dn6_slot: &mut f64,
        var_qjun_d_dn7_slot: &mut f64,
        var_qjun_d_dn8_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_dn12_slot: &mut f64,
        var_qs_dn13_slot: &mut f64,
        var_qs_dn14_slot: &mut f64,
        var_qs_dn15_slot: &mut f64,
        var_qs_dn16_slot: &mut f64,
        var_qs_dn17_slot: &mut f64,
        var_qs_dn18_slot: &mut f64,
        var_qs_dn19_slot: &mut f64,
        var_qs_dn20_slot: &mut f64,
        var_qs_dn5_slot: &mut f64,
        var_qs_dn6_slot: &mut f64,
        var_qs_dn7_slot: &mut f64,
        var_qs_dn8_slot: &mut f64,
        var_r_slot: &mut f64,
        var_r_dn12_slot: &mut f64,
        var_r_dn13_slot: &mut f64,
        var_r_dn14_slot: &mut f64,
        var_r_dn15_slot: &mut f64,
        var_r_dn16_slot: &mut f64,
        var_r_dn17_slot: &mut f64,
        var_r_dn18_slot: &mut f64,
        var_r_dn19_slot: &mut f64,
        var_r_dn20_slot: &mut f64,
        var_r_dn5_slot: &mut f64,
        var_r_dn6_slot: &mut f64,
        var_r_dn7_slot: &mut f64,
        var_r_dn8_slot: &mut f64,
        var_sidexc_slot: &mut f64,
        var_sidexc_dn12_slot: &mut f64,
        var_sidexc_dn13_slot: &mut f64,
        var_sidexc_dn14_slot: &mut f64,
        var_sidexc_dn15_slot: &mut f64,
        var_sidexc_dn16_slot: &mut f64,
        var_sidexc_dn17_slot: &mut f64,
        var_sidexc_dn18_slot: &mut f64,
        var_sidexc_dn19_slot: &mut f64,
        var_sidexc_dn20_slot: &mut f64,
        var_sidexc_dn5_slot: &mut f64,
        var_sidexc_dn6_slot: &mut f64,
        var_sidexc_dn7_slot: &mut f64,
        var_sidexc_dn8_slot: &mut f64,
        var_sqid_slot: &mut f64,
        var_sqid_dn12_slot: &mut f64,
        var_sqid_dn13_slot: &mut f64,
        var_sqid_dn14_slot: &mut f64,
        var_sqid_dn15_slot: &mut f64,
        var_sqid_dn16_slot: &mut f64,
        var_sqid_dn17_slot: &mut f64,
        var_sqid_dn18_slot: &mut f64,
        var_sqid_dn19_slot: &mut f64,
        var_sqid_dn20_slot: &mut f64,
        var_sqid_dn5_slot: &mut f64,
        var_sqid_dn6_slot: &mut f64,
        var_sqid_dn7_slot: &mut f64,
        var_sqid_dn8_slot: &mut f64,
        var_sqig_slot: &mut f64,
        var_sqig_dn12_slot: &mut f64,
        var_sqig_dn13_slot: &mut f64,
        var_sqig_dn14_slot: &mut f64,
        var_sqig_dn15_slot: &mut f64,
        var_sqig_dn16_slot: &mut f64,
        var_sqig_dn17_slot: &mut f64,
        var_sqig_dn18_slot: &mut f64,
        var_sqig_dn19_slot: &mut f64,
        var_sqig_dn20_slot: &mut f64,
        var_sqig_dn5_slot: &mut f64,
        var_sqig_dn6_slot: &mut f64,
        var_sqig_dn7_slot: &mut f64,
        var_sqig_dn8_slot: &mut f64,
        var_sqt2_slot: &mut f64,
        var_sqt2_dn12_slot: &mut f64,
        var_sqt2_dn13_slot: &mut f64,
        var_sqt2_dn14_slot: &mut f64,
        var_sqt2_dn15_slot: &mut f64,
        var_sqt2_dn16_slot: &mut f64,
        var_sqt2_dn17_slot: &mut f64,
        var_sqt2_dn18_slot: &mut f64,
        var_sqt2_dn19_slot: &mut f64,
        var_sqt2_dn20_slot: &mut f64,
        var_sqt2_dn5_slot: &mut f64,
        var_sqt2_dn6_slot: &mut f64,
        var_sqt2_dn7_slot: &mut f64,
        var_sqt2_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn15_slot: &mut f64,
        var_t1_dn16_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn18_slot: &mut f64,
        var_t1_dn19_slot: &mut f64,
        var_t1_dn20_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn15_slot: &mut f64,
        var_t2_dn16_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn18_slot: &mut f64,
        var_t2_dn19_slot: &mut f64,
        var_t2_dn20_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_temp__blk2245_slot: &mut f64,
        var_temp__blk2245_dn12_slot: &mut f64,
        var_temp__blk2245_dn13_slot: &mut f64,
        var_temp__blk2245_dn14_slot: &mut f64,
        var_temp__blk2245_dn15_slot: &mut f64,
        var_temp__blk2245_dn16_slot: &mut f64,
        var_temp__blk2245_dn17_slot: &mut f64,
        var_temp__blk2245_dn18_slot: &mut f64,
        var_temp__blk2245_dn19_slot: &mut f64,
        var_temp__blk2245_dn20_slot: &mut f64,
        var_temp__blk2245_dn5_slot: &mut f64,
        var_temp__blk2245_dn6_slot: &mut f64,
        var_temp__blk2245_dn7_slot: &mut f64,
        var_temp__blk2245_dn8_slot: &mut f64,
    ) {
        let mut var_c_igid: f64 = *var_c_igid_slot;
        let mut var_c_igid_dn12: f64 = *var_c_igid_dn12_slot;
        let mut var_c_igid_dn13: f64 = *var_c_igid_dn13_slot;
        let mut var_c_igid_dn14: f64 = *var_c_igid_dn14_slot;
        let mut var_c_igid_dn15: f64 = *var_c_igid_dn15_slot;
        let mut var_c_igid_dn16: f64 = *var_c_igid_dn16_slot;
        let mut var_c_igid_dn17: f64 = *var_c_igid_dn17_slot;
        let mut var_c_igid_dn18: f64 = *var_c_igid_dn18_slot;
        let mut var_c_igid_dn19: f64 = *var_c_igid_dn19_slot;
        let mut var_c_igid_dn20: f64 = *var_c_igid_dn20_slot;
        let mut var_c_igid_dn5: f64 = *var_c_igid_dn5_slot;
        let mut var_c_igid_dn6: f64 = *var_c_igid_dn6_slot;
        let mut var_c_igid_dn7: f64 = *var_c_igid_dn7_slot;
        let mut var_c_igid_dn8: f64 = *var_c_igid_dn8_slot;
        let mut var_cgeff: f64 = *var_cgeff_slot;
        let mut var_cgeff_dn12: f64 = *var_cgeff_dn12_slot;
        let mut var_cgeff_dn13: f64 = *var_cgeff_dn13_slot;
        let mut var_cgeff_dn14: f64 = *var_cgeff_dn14_slot;
        let mut var_cgeff_dn15: f64 = *var_cgeff_dn15_slot;
        let mut var_cgeff_dn16: f64 = *var_cgeff_dn16_slot;
        let mut var_cgeff_dn17: f64 = *var_cgeff_dn17_slot;
        let mut var_cgeff_dn18: f64 = *var_cgeff_dn18_slot;
        let mut var_cgeff_dn19: f64 = *var_cgeff_dn19_slot;
        let mut var_cgeff_dn20: f64 = *var_cgeff_dn20_slot;
        let mut var_cgeff_dn5: f64 = *var_cgeff_dn5_slot;
        let mut var_cgeff_dn6: f64 = *var_cgeff_dn6_slot;
        let mut var_cgeff_dn7: f64 = *var_cgeff_dn7_slot;
        let mut var_cgeff_dn8: f64 = *var_cgeff_dn8_slot;
        let mut var_g_ideal: f64 = *var_g_ideal_slot;
        let mut var_g_ideal_dn12: f64 = *var_g_ideal_dn12_slot;
        let mut var_g_ideal_dn13: f64 = *var_g_ideal_dn13_slot;
        let mut var_g_ideal_dn14: f64 = *var_g_ideal_dn14_slot;
        let mut var_g_ideal_dn15: f64 = *var_g_ideal_dn15_slot;
        let mut var_g_ideal_dn16: f64 = *var_g_ideal_dn16_slot;
        let mut var_g_ideal_dn17: f64 = *var_g_ideal_dn17_slot;
        let mut var_g_ideal_dn18: f64 = *var_g_ideal_dn18_slot;
        let mut var_g_ideal_dn19: f64 = *var_g_ideal_dn19_slot;
        let mut var_g_ideal_dn20: f64 = *var_g_ideal_dn20_slot;
        let mut var_g_ideal_dn5: f64 = *var_g_ideal_dn5_slot;
        let mut var_g_ideal_dn6: f64 = *var_g_ideal_dn6_slot;
        let mut var_g_ideal_dn7: f64 = *var_g_ideal_dn7_slot;
        let mut var_g_ideal_dn8: f64 = *var_g_ideal_dn8_slot;
        let mut var_guard2246: f64 = *var_guard2246_slot;
        let mut var_guard2279: f64 = *var_guard2279_slot;
        let mut var_guard2281: f64 = *var_guard2281_slot;
        let mut var_guard2282: f64 = *var_guard2282_slot;
        let mut var_h0: f64 = *var_h0_slot;
        let mut var_h0_dn12: f64 = *var_h0_dn12_slot;
        let mut var_h0_dn13: f64 = *var_h0_dn13_slot;
        let mut var_h0_dn14: f64 = *var_h0_dn14_slot;
        let mut var_h0_dn15: f64 = *var_h0_dn15_slot;
        let mut var_h0_dn16: f64 = *var_h0_dn16_slot;
        let mut var_h0_dn17: f64 = *var_h0_dn17_slot;
        let mut var_h0_dn18: f64 = *var_h0_dn18_slot;
        let mut var_h0_dn19: f64 = *var_h0_dn19_slot;
        let mut var_h0_dn20: f64 = *var_h0_dn20_slot;
        let mut var_h0_dn5: f64 = *var_h0_dn5_slot;
        let mut var_h0_dn6: f64 = *var_h0_dn6_slot;
        let mut var_h0_dn7: f64 = *var_h0_dn7_slot;
        let mut var_h0_dn8: f64 = *var_h0_dn8_slot;
        let mut var_lc: f64 = *var_lc_slot;
        let mut var_lc_dn12: f64 = *var_lc_dn12_slot;
        let mut var_lc_dn13: f64 = *var_lc_dn13_slot;
        let mut var_lc_dn14: f64 = *var_lc_dn14_slot;
        let mut var_lc_dn15: f64 = *var_lc_dn15_slot;
        let mut var_lc_dn16: f64 = *var_lc_dn16_slot;
        let mut var_lc_dn17: f64 = *var_lc_dn17_slot;
        let mut var_lc_dn18: f64 = *var_lc_dn18_slot;
        let mut var_lc_dn19: f64 = *var_lc_dn19_slot;
        let mut var_lc_dn20: f64 = *var_lc_dn20_slot;
        let mut var_lc_dn5: f64 = *var_lc_dn5_slot;
        let mut var_lc_dn6: f64 = *var_lc_dn6_slot;
        let mut var_lc_dn7: f64 = *var_lc_dn7_slot;
        let mut var_lc_dn8: f64 = *var_lc_dn8_slot;
        let mut var_lcinv2: f64 = *var_lcinv2_slot;
        let mut var_lcinv2_dn12: f64 = *var_lcinv2_dn12_slot;
        let mut var_lcinv2_dn13: f64 = *var_lcinv2_dn13_slot;
        let mut var_lcinv2_dn14: f64 = *var_lcinv2_dn14_slot;
        let mut var_lcinv2_dn15: f64 = *var_lcinv2_dn15_slot;
        let mut var_lcinv2_dn16: f64 = *var_lcinv2_dn16_slot;
        let mut var_lcinv2_dn17: f64 = *var_lcinv2_dn17_slot;
        let mut var_lcinv2_dn18: f64 = *var_lcinv2_dn18_slot;
        let mut var_lcinv2_dn19: f64 = *var_lcinv2_dn19_slot;
        let mut var_lcinv2_dn20: f64 = *var_lcinv2_dn20_slot;
        let mut var_lcinv2_dn5: f64 = *var_lcinv2_dn5_slot;
        let mut var_lcinv2_dn6: f64 = *var_lcinv2_dn6_slot;
        let mut var_lcinv2_dn7: f64 = *var_lcinv2_dn7_slot;
        let mut var_lcinv2_dn8: f64 = *var_lcinv2_dn8_slot;
        let mut var_mid: f64 = *var_mid_slot;
        let mut var_mid_dn12: f64 = *var_mid_dn12_slot;
        let mut var_mid_dn13: f64 = *var_mid_dn13_slot;
        let mut var_mid_dn14: f64 = *var_mid_dn14_slot;
        let mut var_mid_dn15: f64 = *var_mid_dn15_slot;
        let mut var_mid_dn16: f64 = *var_mid_dn16_slot;
        let mut var_mid_dn17: f64 = *var_mid_dn17_slot;
        let mut var_mid_dn18: f64 = *var_mid_dn18_slot;
        let mut var_mid_dn19: f64 = *var_mid_dn19_slot;
        let mut var_mid_dn20: f64 = *var_mid_dn20_slot;
        let mut var_mid_dn5: f64 = *var_mid_dn5_slot;
        let mut var_mid_dn6: f64 = *var_mid_dn6_slot;
        let mut var_mid_dn7: f64 = *var_mid_dn7_slot;
        let mut var_mid_dn8: f64 = *var_mid_dn8_slot;
        let mut var_mig: f64 = *var_mig_slot;
        let mut var_mig_dn12: f64 = *var_mig_dn12_slot;
        let mut var_mig_dn13: f64 = *var_mig_dn13_slot;
        let mut var_mig_dn14: f64 = *var_mig_dn14_slot;
        let mut var_mig_dn15: f64 = *var_mig_dn15_slot;
        let mut var_mig_dn16: f64 = *var_mig_dn16_slot;
        let mut var_mig_dn17: f64 = *var_mig_dn17_slot;
        let mut var_mig_dn18: f64 = *var_mig_dn18_slot;
        let mut var_mig_dn19: f64 = *var_mig_dn19_slot;
        let mut var_mig_dn20: f64 = *var_mig_dn20_slot;
        let mut var_mig_dn5: f64 = *var_mig_dn5_slot;
        let mut var_mig_dn6: f64 = *var_mig_dn6_slot;
        let mut var_mig_dn7: f64 = *var_mig_dn7_slot;
        let mut var_mig_dn8: f64 = *var_mig_dn8_slot;
        let mut var_migid: f64 = *var_migid_slot;
        let mut var_migid_dn12: f64 = *var_migid_dn12_slot;
        let mut var_migid_dn13: f64 = *var_migid_dn13_slot;
        let mut var_migid_dn14: f64 = *var_migid_dn14_slot;
        let mut var_migid_dn15: f64 = *var_migid_dn15_slot;
        let mut var_migid_dn16: f64 = *var_migid_dn16_slot;
        let mut var_migid_dn17: f64 = *var_migid_dn17_slot;
        let mut var_migid_dn18: f64 = *var_migid_dn18_slot;
        let mut var_migid_dn19: f64 = *var_migid_dn19_slot;
        let mut var_migid_dn20: f64 = *var_migid_dn20_slot;
        let mut var_migid_dn5: f64 = *var_migid_dn5_slot;
        let mut var_migid_dn6: f64 = *var_migid_dn6_slot;
        let mut var_migid_dn7: f64 = *var_migid_dn7_slot;
        let mut var_migid_dn8: f64 = *var_migid_dn8_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn12: f64 = *var_qd_dn12_slot;
        let mut var_qd_dn13: f64 = *var_qd_dn13_slot;
        let mut var_qd_dn14: f64 = *var_qd_dn14_slot;
        let mut var_qd_dn15: f64 = *var_qd_dn15_slot;
        let mut var_qd_dn16: f64 = *var_qd_dn16_slot;
        let mut var_qd_dn17: f64 = *var_qd_dn17_slot;
        let mut var_qd_dn18: f64 = *var_qd_dn18_slot;
        let mut var_qd_dn19: f64 = *var_qd_dn19_slot;
        let mut var_qd_dn20: f64 = *var_qd_dn20_slot;
        let mut var_qd_dn5: f64 = *var_qd_dn5_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qjun_d: f64 = *var_qjun_d_slot;
        let mut var_qjun_d_dn10: f64 = *var_qjun_d_dn10_slot;
        let mut var_qjun_d_dn11: f64 = *var_qjun_d_dn11_slot;
        let mut var_qjun_d_dn5: f64 = *var_qjun_d_dn5_slot;
        let mut var_qjun_d_dn6: f64 = *var_qjun_d_dn6_slot;
        let mut var_qjun_d_dn7: f64 = *var_qjun_d_dn7_slot;
        let mut var_qjun_d_dn8: f64 = *var_qjun_d_dn8_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_dn12: f64 = *var_qs_dn12_slot;
        let mut var_qs_dn13: f64 = *var_qs_dn13_slot;
        let mut var_qs_dn14: f64 = *var_qs_dn14_slot;
        let mut var_qs_dn15: f64 = *var_qs_dn15_slot;
        let mut var_qs_dn16: f64 = *var_qs_dn16_slot;
        let mut var_qs_dn17: f64 = *var_qs_dn17_slot;
        let mut var_qs_dn18: f64 = *var_qs_dn18_slot;
        let mut var_qs_dn19: f64 = *var_qs_dn19_slot;
        let mut var_qs_dn20: f64 = *var_qs_dn20_slot;
        let mut var_qs_dn5: f64 = *var_qs_dn5_slot;
        let mut var_qs_dn6: f64 = *var_qs_dn6_slot;
        let mut var_qs_dn7: f64 = *var_qs_dn7_slot;
        let mut var_qs_dn8: f64 = *var_qs_dn8_slot;
        let mut var_r: f64 = *var_r_slot;
        let mut var_r_dn12: f64 = *var_r_dn12_slot;
        let mut var_r_dn13: f64 = *var_r_dn13_slot;
        let mut var_r_dn14: f64 = *var_r_dn14_slot;
        let mut var_r_dn15: f64 = *var_r_dn15_slot;
        let mut var_r_dn16: f64 = *var_r_dn16_slot;
        let mut var_r_dn17: f64 = *var_r_dn17_slot;
        let mut var_r_dn18: f64 = *var_r_dn18_slot;
        let mut var_r_dn19: f64 = *var_r_dn19_slot;
        let mut var_r_dn20: f64 = *var_r_dn20_slot;
        let mut var_r_dn5: f64 = *var_r_dn5_slot;
        let mut var_r_dn6: f64 = *var_r_dn6_slot;
        let mut var_r_dn7: f64 = *var_r_dn7_slot;
        let mut var_r_dn8: f64 = *var_r_dn8_slot;
        let mut var_sidexc: f64 = *var_sidexc_slot;
        let mut var_sidexc_dn12: f64 = *var_sidexc_dn12_slot;
        let mut var_sidexc_dn13: f64 = *var_sidexc_dn13_slot;
        let mut var_sidexc_dn14: f64 = *var_sidexc_dn14_slot;
        let mut var_sidexc_dn15: f64 = *var_sidexc_dn15_slot;
        let mut var_sidexc_dn16: f64 = *var_sidexc_dn16_slot;
        let mut var_sidexc_dn17: f64 = *var_sidexc_dn17_slot;
        let mut var_sidexc_dn18: f64 = *var_sidexc_dn18_slot;
        let mut var_sidexc_dn19: f64 = *var_sidexc_dn19_slot;
        let mut var_sidexc_dn20: f64 = *var_sidexc_dn20_slot;
        let mut var_sidexc_dn5: f64 = *var_sidexc_dn5_slot;
        let mut var_sidexc_dn6: f64 = *var_sidexc_dn6_slot;
        let mut var_sidexc_dn7: f64 = *var_sidexc_dn7_slot;
        let mut var_sidexc_dn8: f64 = *var_sidexc_dn8_slot;
        let mut var_sqid: f64 = *var_sqid_slot;
        let mut var_sqid_dn12: f64 = *var_sqid_dn12_slot;
        let mut var_sqid_dn13: f64 = *var_sqid_dn13_slot;
        let mut var_sqid_dn14: f64 = *var_sqid_dn14_slot;
        let mut var_sqid_dn15: f64 = *var_sqid_dn15_slot;
        let mut var_sqid_dn16: f64 = *var_sqid_dn16_slot;
        let mut var_sqid_dn17: f64 = *var_sqid_dn17_slot;
        let mut var_sqid_dn18: f64 = *var_sqid_dn18_slot;
        let mut var_sqid_dn19: f64 = *var_sqid_dn19_slot;
        let mut var_sqid_dn20: f64 = *var_sqid_dn20_slot;
        let mut var_sqid_dn5: f64 = *var_sqid_dn5_slot;
        let mut var_sqid_dn6: f64 = *var_sqid_dn6_slot;
        let mut var_sqid_dn7: f64 = *var_sqid_dn7_slot;
        let mut var_sqid_dn8: f64 = *var_sqid_dn8_slot;
        let mut var_sqig: f64 = *var_sqig_slot;
        let mut var_sqig_dn12: f64 = *var_sqig_dn12_slot;
        let mut var_sqig_dn13: f64 = *var_sqig_dn13_slot;
        let mut var_sqig_dn14: f64 = *var_sqig_dn14_slot;
        let mut var_sqig_dn15: f64 = *var_sqig_dn15_slot;
        let mut var_sqig_dn16: f64 = *var_sqig_dn16_slot;
        let mut var_sqig_dn17: f64 = *var_sqig_dn17_slot;
        let mut var_sqig_dn18: f64 = *var_sqig_dn18_slot;
        let mut var_sqig_dn19: f64 = *var_sqig_dn19_slot;
        let mut var_sqig_dn20: f64 = *var_sqig_dn20_slot;
        let mut var_sqig_dn5: f64 = *var_sqig_dn5_slot;
        let mut var_sqig_dn6: f64 = *var_sqig_dn6_slot;
        let mut var_sqig_dn7: f64 = *var_sqig_dn7_slot;
        let mut var_sqig_dn8: f64 = *var_sqig_dn8_slot;
        let mut var_sqt2: f64 = *var_sqt2_slot;
        let mut var_sqt2_dn12: f64 = *var_sqt2_dn12_slot;
        let mut var_sqt2_dn13: f64 = *var_sqt2_dn13_slot;
        let mut var_sqt2_dn14: f64 = *var_sqt2_dn14_slot;
        let mut var_sqt2_dn15: f64 = *var_sqt2_dn15_slot;
        let mut var_sqt2_dn16: f64 = *var_sqt2_dn16_slot;
        let mut var_sqt2_dn17: f64 = *var_sqt2_dn17_slot;
        let mut var_sqt2_dn18: f64 = *var_sqt2_dn18_slot;
        let mut var_sqt2_dn19: f64 = *var_sqt2_dn19_slot;
        let mut var_sqt2_dn20: f64 = *var_sqt2_dn20_slot;
        let mut var_sqt2_dn5: f64 = *var_sqt2_dn5_slot;
        let mut var_sqt2_dn6: f64 = *var_sqt2_dn6_slot;
        let mut var_sqt2_dn7: f64 = *var_sqt2_dn7_slot;
        let mut var_sqt2_dn8: f64 = *var_sqt2_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn15: f64 = *var_t1_dn15_slot;
        let mut var_t1_dn16: f64 = *var_t1_dn16_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn18: f64 = *var_t1_dn18_slot;
        let mut var_t1_dn19: f64 = *var_t1_dn19_slot;
        let mut var_t1_dn20: f64 = *var_t1_dn20_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn15: f64 = *var_t2_dn15_slot;
        let mut var_t2_dn16: f64 = *var_t2_dn16_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn18: f64 = *var_t2_dn18_slot;
        let mut var_t2_dn19: f64 = *var_t2_dn19_slot;
        let mut var_t2_dn20: f64 = *var_t2_dn20_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_temp__blk2245: f64 = *var_temp__blk2245_slot;
        let mut var_temp__blk2245_dn12: f64 = *var_temp__blk2245_dn12_slot;
        let mut var_temp__blk2245_dn13: f64 = *var_temp__blk2245_dn13_slot;
        let mut var_temp__blk2245_dn14: f64 = *var_temp__blk2245_dn14_slot;
        let mut var_temp__blk2245_dn15: f64 = *var_temp__blk2245_dn15_slot;
        let mut var_temp__blk2245_dn16: f64 = *var_temp__blk2245_dn16_slot;
        let mut var_temp__blk2245_dn17: f64 = *var_temp__blk2245_dn17_slot;
        let mut var_temp__blk2245_dn18: f64 = *var_temp__blk2245_dn18_slot;
        let mut var_temp__blk2245_dn19: f64 = *var_temp__blk2245_dn19_slot;
        let mut var_temp__blk2245_dn20: f64 = *var_temp__blk2245_dn20_slot;
        let mut var_temp__blk2245_dn5: f64 = *var_temp__blk2245_dn5_slot;
        let mut var_temp__blk2245_dn6: f64 = *var_temp__blk2245_dn6_slot;
        let mut var_temp__blk2245_dn7: f64 = *var_temp__blk2245_dn7_slot;
        let mut var_temp__blk2245_dn8: f64 = *var_temp__blk2245_dn8_slot;

        let assign82750_e124484: f64 = (var_abdrain_i * var_qjunbot_d);
        let assign82750_e124487: f64 = (var_lsdrain_i * var_qjunsti_d);
        let assign82750_e124488: f64 = (assign82750_e124484 + assign82750_e124487);
        let assign82750_e124491: f64 = (var_lgdrain_i * var_qjungat_d);
        let assign82750_e124492: f64 = (assign82750_e124488 + assign82750_e124491);
        var_qjun_d = assign82750_e124492;
        var_qjun_d_dn5 = (((var_abdrain_i * var_qjunbot_d_dn5) + (var_lsdrain_i * var_qjunsti_d_dn5)) + (var_lgdrain_i * var_qjungat_d_dn5));
        var_qjun_d_dn6 = (((var_abdrain_i * var_qjunbot_d_dn6) + (var_lsdrain_i * var_qjunsti_d_dn6)) + (var_lgdrain_i * var_qjungat_d_dn6));
        var_qjun_d_dn7 = (((var_abdrain_i * var_qjunbot_d_dn7) + (var_lsdrain_i * var_qjunsti_d_dn7)) + (var_lgdrain_i * var_qjungat_d_dn7));
        var_qjun_d_dn8 = (((var_abdrain_i * var_qjunbot_d_dn8) + (var_lsdrain_i * var_qjunsti_d_dn8)) + (var_lgdrain_i * var_qjungat_d_dn8));
        var_qjun_d_dn10 = (((var_abdrain_i * var_qjunbot_d_dn10) + (var_lsdrain_i * var_qjunsti_d_dn10)) + (var_lgdrain_i * var_qjungat_d_dn10));
        var_qjun_d_dn11 = (((var_abdrain_i * var_qjunbot_d_dn11) + (var_lsdrain_i * var_qjunsti_d_dn11)) + (var_lgdrain_i * var_qjungat_d_dn11));

        let assign82760_e124495: f64 = if var_sigvds < 0.0 { 1.0 } else { 0.0 };
        var_guard2246 = assign82760_e124495;

        let (assign82770_e124499, assign82770_e124499_d_n5, assign82770_e124499_d_n6, assign82770_e124499_d_n7, assign82770_e124499_d_n8, assign82770_e124499_d_n12, assign82770_e124499_d_n13, assign82770_e124499_d_n14, assign82770_e124499_d_n15, assign82770_e124499_d_n16, assign82770_e124499_d_n17, assign82770_e124499_d_n18, assign82770_e124499_d_n19, assign82770_e124499_d_n20,) = {
    if (var_guard2246 != 0.0) {
        (var_qd, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_dn12, var_qd_dn13, var_qd_dn14, var_qd_dn15, var_qd_dn16, var_qd_dn17, var_qd_dn18, var_qd_dn19, var_qd_dn20,)
    } else {
        (var_temp__blk2245, var_temp__blk2245_dn5, var_temp__blk2245_dn6, var_temp__blk2245_dn7, var_temp__blk2245_dn8, var_temp__blk2245_dn12, var_temp__blk2245_dn13, var_temp__blk2245_dn14, var_temp__blk2245_dn15, var_temp__blk2245_dn16, var_temp__blk2245_dn17, var_temp__blk2245_dn18, var_temp__blk2245_dn19, var_temp__blk2245_dn20,)
    }
};
        var_temp__blk2245 = assign82770_e124499;
        var_temp__blk2245_dn5 = assign82770_e124499_d_n5;
        var_temp__blk2245_dn6 = assign82770_e124499_d_n6;
        var_temp__blk2245_dn7 = assign82770_e124499_d_n7;
        var_temp__blk2245_dn8 = assign82770_e124499_d_n8;
        var_temp__blk2245_dn12 = assign82770_e124499_d_n12;
        var_temp__blk2245_dn13 = assign82770_e124499_d_n13;
        var_temp__blk2245_dn14 = assign82770_e124499_d_n14;
        var_temp__blk2245_dn15 = assign82770_e124499_d_n15;
        var_temp__blk2245_dn16 = assign82770_e124499_d_n16;
        var_temp__blk2245_dn17 = assign82770_e124499_d_n17;
        var_temp__blk2245_dn18 = assign82770_e124499_d_n18;
        var_temp__blk2245_dn19 = assign82770_e124499_d_n19;
        var_temp__blk2245_dn20 = assign82770_e124499_d_n20;

        let (assign82780_e124503, assign82780_e124503_d_n5, assign82780_e124503_d_n6, assign82780_e124503_d_n7, assign82780_e124503_d_n8, assign82780_e124503_d_n12, assign82780_e124503_d_n13, assign82780_e124503_d_n14, assign82780_e124503_d_n15, assign82780_e124503_d_n16, assign82780_e124503_d_n17, assign82780_e124503_d_n18, assign82780_e124503_d_n19, assign82780_e124503_d_n20,) = {
    if (var_guard2246 != 0.0) {
        (var_qs, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_dn12, var_qs_dn13, var_qs_dn14, var_qs_dn15, var_qs_dn16, var_qs_dn17, var_qs_dn18, var_qs_dn19, var_qs_dn20,)
    } else {
        (var_qd, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_dn12, var_qd_dn13, var_qd_dn14, var_qd_dn15, var_qd_dn16, var_qd_dn17, var_qd_dn18, var_qd_dn19, var_qd_dn20,)
    }
};
        var_qd = assign82780_e124503;
        var_qd_dn5 = assign82780_e124503_d_n5;
        var_qd_dn6 = assign82780_e124503_d_n6;
        var_qd_dn7 = assign82780_e124503_d_n7;
        var_qd_dn8 = assign82780_e124503_d_n8;
        var_qd_dn12 = assign82780_e124503_d_n12;
        var_qd_dn13 = assign82780_e124503_d_n13;
        var_qd_dn14 = assign82780_e124503_d_n14;
        var_qd_dn15 = assign82780_e124503_d_n15;
        var_qd_dn16 = assign82780_e124503_d_n16;
        var_qd_dn17 = assign82780_e124503_d_n17;
        var_qd_dn18 = assign82780_e124503_d_n18;
        var_qd_dn19 = assign82780_e124503_d_n19;
        var_qd_dn20 = assign82780_e124503_d_n20;

        let (assign82790_e124507, assign82790_e124507_d_n5, assign82790_e124507_d_n6, assign82790_e124507_d_n7, assign82790_e124507_d_n8, assign82790_e124507_d_n12, assign82790_e124507_d_n13, assign82790_e124507_d_n14, assign82790_e124507_d_n15, assign82790_e124507_d_n16, assign82790_e124507_d_n17, assign82790_e124507_d_n18, assign82790_e124507_d_n19, assign82790_e124507_d_n20,) = {
    if (var_guard2246 != 0.0) {
        (var_temp__blk2245, var_temp__blk2245_dn5, var_temp__blk2245_dn6, var_temp__blk2245_dn7, var_temp__blk2245_dn8, var_temp__blk2245_dn12, var_temp__blk2245_dn13, var_temp__blk2245_dn14, var_temp__blk2245_dn15, var_temp__blk2245_dn16, var_temp__blk2245_dn17, var_temp__blk2245_dn18, var_temp__blk2245_dn19, var_temp__blk2245_dn20,)
    } else {
        (var_qs, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_dn12, var_qs_dn13, var_qs_dn14, var_qs_dn15, var_qs_dn16, var_qs_dn17, var_qs_dn18, var_qs_dn19, var_qs_dn20,)
    }
};
        var_qs = assign82790_e124507;
        var_qs_dn5 = assign82790_e124507_d_n5;
        var_qs_dn6 = assign82790_e124507_d_n6;
        var_qs_dn7 = assign82790_e124507_d_n7;
        var_qs_dn8 = assign82790_e124507_d_n8;
        var_qs_dn12 = assign82790_e124507_d_n12;
        var_qs_dn13 = assign82790_e124507_d_n13;
        var_qs_dn14 = assign82790_e124507_d_n14;
        var_qs_dn15 = assign82790_e124507_d_n15;
        var_qs_dn16 = assign82790_e124507_d_n16;
        var_qs_dn17 = assign82790_e124507_d_n17;
        var_qs_dn18 = assign82790_e124507_d_n18;
        var_qs_dn19 = assign82790_e124507_d_n19;
        var_qs_dn20 = assign82790_e124507_d_n20;

        var_sidexc = 0.0;
        var_sidexc_dn5 = 0.0;
        var_sidexc_dn6 = 0.0;
        var_sidexc_dn7 = 0.0;
        var_sidexc_dn8 = 0.0;
        var_sidexc_dn12 = 0.0;
        var_sidexc_dn13 = 0.0;
        var_sidexc_dn14 = 0.0;
        var_sidexc_dn15 = 0.0;
        var_sidexc_dn16 = 0.0;
        var_sidexc_dn17 = 0.0;
        var_sidexc_dn18 = 0.0;
        var_sidexc_dn19 = 0.0;
        var_sidexc_dn20 = 0.0;

        var_mid = 0.0;
        var_mid_dn5 = 0.0;
        var_mid_dn6 = 0.0;
        var_mid_dn7 = 0.0;
        var_mid_dn8 = 0.0;
        var_mid_dn12 = 0.0;
        var_mid_dn13 = 0.0;
        var_mid_dn14 = 0.0;
        var_mid_dn15 = 0.0;
        var_mid_dn16 = 0.0;
        var_mid_dn17 = 0.0;
        var_mid_dn18 = 0.0;
        var_mid_dn19 = 0.0;
        var_mid_dn20 = 0.0;

        var_mig = 1e-40;
        var_mig_dn5 = 0.0;
        var_mig_dn6 = 0.0;
        var_mig_dn7 = 0.0;
        var_mig_dn8 = 0.0;
        var_mig_dn12 = 0.0;
        var_mig_dn13 = 0.0;
        var_mig_dn14 = 0.0;
        var_mig_dn15 = 0.0;
        var_mig_dn16 = 0.0;
        var_mig_dn17 = 0.0;
        var_mig_dn18 = 0.0;
        var_mig_dn19 = 0.0;
        var_mig_dn20 = 0.0;

        var_migid = 0.0;
        var_migid_dn5 = 0.0;
        var_migid_dn6 = 0.0;
        var_migid_dn7 = 0.0;
        var_migid_dn8 = 0.0;
        var_migid_dn12 = 0.0;
        var_migid_dn13 = 0.0;
        var_migid_dn14 = 0.0;
        var_migid_dn15 = 0.0;
        var_migid_dn16 = 0.0;
        var_migid_dn17 = 0.0;
        var_migid_dn18 = 0.0;
        var_migid_dn19 = 0.0;
        var_migid_dn20 = 0.0;

        var_c_igid = 0.0;
        var_c_igid_dn5 = 0.0;
        var_c_igid_dn6 = 0.0;
        var_c_igid_dn7 = 0.0;
        var_c_igid_dn8 = 0.0;
        var_c_igid_dn12 = 0.0;
        var_c_igid_dn13 = 0.0;
        var_c_igid_dn14 = 0.0;
        var_c_igid_dn15 = 0.0;
        var_c_igid_dn16 = 0.0;
        var_c_igid_dn17 = 0.0;
        var_c_igid_dn18 = 0.0;
        var_c_igid_dn19 = 0.0;
        var_c_igid_dn20 = 0.0;

        let assign82860_e124516: f64 = (var_cox_qm * var_eta_p_ac);
        var_cgeff = assign82860_e124516;
        var_cgeff_dn5 = ((var_cox_qm_dn5 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn5));
        var_cgeff_dn6 = ((var_cox_qm_dn6 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn6));
        var_cgeff_dn7 = ((var_cox_qm_dn7 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn7));
        var_cgeff_dn8 = ((var_cox_qm_dn8 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn8));
        var_cgeff_dn12 = ((var_cox_qm_dn12 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn12));
        var_cgeff_dn13 = ((var_cox_qm_dn13 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn13));
        var_cgeff_dn14 = ((var_cox_qm_dn14 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn14));
        var_cgeff_dn15 = ((var_cox_qm_dn15 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn15));
        var_cgeff_dn16 = ((var_cox_qm_dn16 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn16));
        var_cgeff_dn17 = ((var_cox_qm_dn17 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn17));
        var_cgeff_dn18 = ((var_cox_qm_dn18 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn18));
        var_cgeff_dn19 = ((var_cox_qm_dn19 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn19));
        var_cgeff_dn20 = ((var_cox_qm_dn20 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn20));

        var_sqid = 0.0;
        var_sqid_dn5 = 0.0;
        var_sqid_dn6 = 0.0;
        var_sqid_dn7 = 0.0;
        var_sqid_dn8 = 0.0;
        var_sqid_dn12 = 0.0;
        var_sqid_dn13 = 0.0;
        var_sqid_dn14 = 0.0;
        var_sqid_dn15 = 0.0;
        var_sqid_dn16 = 0.0;
        var_sqid_dn17 = 0.0;
        var_sqid_dn18 = 0.0;
        var_sqid_dn19 = 0.0;
        var_sqid_dn20 = 0.0;

        var_sqig = 0.0;
        var_sqig_dn5 = 0.0;
        var_sqig_dn6 = 0.0;
        var_sqig_dn7 = 0.0;
        var_sqig_dn8 = 0.0;
        var_sqig_dn12 = 0.0;
        var_sqig_dn13 = 0.0;
        var_sqig_dn14 = 0.0;
        var_sqig_dn15 = 0.0;
        var_sqig_dn16 = 0.0;
        var_sqig_dn17 = 0.0;
        var_sqig_dn18 = 0.0;
        var_sqig_dn19 = 0.0;
        var_sqig_dn20 = 0.0;

        let assign82920_e124528: f64 = if ((var_xg_dc > 0.0) && (var_bet_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard2279 = assign82920_e124528;

        let assign83010_e124634: f64 = if p.p32 > 0.0 { 1.0 } else { 0.0 };
        var_guard2281 = assign83010_e124634;

        let (assign83020_e124642, assign83020_e124642_d_n5, assign83020_e124642_d_n6, assign83020_e124642_d_n7, assign83020_e124642_d_n8, assign83020_e124642_d_n12, assign83020_e124642_d_n13, assign83020_e124642_d_n14, assign83020_e124642_d_n15, assign83020_e124642_d_n16, assign83020_e124642_d_n17, assign83020_e124642_d_n18, assign83020_e124642_d_n19, assign83020_e124642_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) {
        let assign83020_e124640: f64 = (var_qim1_dc / var_alpha_dc);
        (assign83020_e124640, (((var_qim1_dc_dn5 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn5)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn6 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn6)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn7 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn7)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn8 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn8)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn12 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn12)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn13 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn13)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn14 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn14)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn15 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn15)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn16 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn16)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn17 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn17)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn18 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn18)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn19 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn19)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn20 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn20)) / (var_alpha_dc * var_alpha_dc)),)
    } else {
        (var_h0, var_h0_dn5, var_h0_dn6, var_h0_dn7, var_h0_dn8, var_h0_dn12, var_h0_dn13, var_h0_dn14, var_h0_dn15, var_h0_dn16, var_h0_dn17, var_h0_dn18, var_h0_dn19, var_h0_dn20,)
    }
};
        var_h0 = assign83020_e124642;
        var_h0_dn5 = assign83020_e124642_d_n5;
        var_h0_dn6 = assign83020_e124642_d_n6;
        var_h0_dn7 = assign83020_e124642_d_n7;
        var_h0_dn8 = assign83020_e124642_d_n8;
        var_h0_dn12 = assign83020_e124642_d_n12;
        var_h0_dn13 = assign83020_e124642_d_n13;
        var_h0_dn14 = assign83020_e124642_d_n14;
        var_h0_dn15 = assign83020_e124642_d_n15;
        var_h0_dn16 = assign83020_e124642_d_n16;
        var_h0_dn17 = assign83020_e124642_d_n17;
        var_h0_dn18 = assign83020_e124642_d_n18;
        var_h0_dn19 = assign83020_e124642_d_n19;
        var_h0_dn20 = assign83020_e124642_d_n20;

        let (assign83030_e124650, assign83030_e124650_d_n5, assign83030_e124650_d_n6, assign83030_e124650_d_n7, assign83030_e124650_d_n8, assign83030_e124650_d_n12, assign83030_e124650_d_n13, assign83030_e124650_d_n14, assign83030_e124650_d_n15, assign83030_e124650_d_n16, assign83030_e124650_d_n17, assign83030_e124650_d_n18, assign83030_e124650_d_n19, assign83030_e124650_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) {
        let assign83030_e124648: f64 = (var_qim_dc / var_qim1_dc);
        (assign83030_e124648, (((var_qim_dc_dn5 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn5)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn6 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn6)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn7 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn7)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn8 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn8)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn12 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn12)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn13 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn13)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn14 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn14)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn15 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn15)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn16 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn16)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn17 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn17)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn18 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn18)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn19 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn19)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn20 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn20)) / (var_qim1_dc * var_qim1_dc)),)
    } else {
        (var_t1, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn12, var_t1_dn13, var_t1_dn14, var_t1_dn15, var_t1_dn16, var_t1_dn17, var_t1_dn18, var_t1_dn19, var_t1_dn20,)
    }
};
        var_t1 = assign83030_e124650;
        var_t1_dn5 = assign83030_e124650_d_n5;
        var_t1_dn6 = assign83030_e124650_d_n6;
        var_t1_dn7 = assign83030_e124650_d_n7;
        var_t1_dn8 = assign83030_e124650_d_n8;
        var_t1_dn12 = assign83030_e124650_d_n12;
        var_t1_dn13 = assign83030_e124650_d_n13;
        var_t1_dn14 = assign83030_e124650_d_n14;
        var_t1_dn15 = assign83030_e124650_d_n15;
        var_t1_dn16 = assign83030_e124650_d_n16;
        var_t1_dn17 = assign83030_e124650_d_n17;
        var_t1_dn18 = assign83030_e124650_d_n18;
        var_t1_dn19 = assign83030_e124650_d_n19;
        var_t1_dn20 = assign83030_e124650_d_n20;

        let (assign83040_e124662, assign83040_e124662_d_n5, assign83040_e124662_d_n6, assign83040_e124662_d_n7, assign83040_e124662_d_n8, assign83040_e124662_d_n12, assign83040_e124662_d_n13, assign83040_e124662_d_n14, assign83040_e124662_d_n15, assign83040_e124662_d_n16, assign83040_e124662_d_n17, assign83040_e124662_d_n18, assign83040_e124662_d_n19, assign83040_e124662_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) {
        let assign83040_e124656: f64 = (0.5 * 0.16666666666666666);
        let assign83040_e124659: f64 = (var_dps_dc / var_h0);
        let assign83040_e124660: f64 = (assign83040_e124656 * assign83040_e124659);
        (assign83040_e124660, (assign83040_e124656 * (((var_dps_dc_dn5 * var_h0) - (var_dps_dc * var_h0_dn5)) / (var_h0 * var_h0))), (assign83040_e124656 * (((var_dps_dc_dn6 * var_h0) - (var_dps_dc * var_h0_dn6)) / (var_h0 * var_h0))), (assign83040_e124656 * (((var_dps_dc_dn7 * var_h0) - (var_dps_dc * var_h0_dn7)) / (var_h0 * var_h0))), (assign83040_e124656 * (((var_dps_dc_dn8 * var_h0) - (var_dps_dc * var_h0_dn8)) / (var_h0 * var_h0))), (assign83040_e124656 * (((var_dps_dc_dn12 * var_h0) - (var_dps_dc * var_h0_dn12)) / (var_h0 * var_h0))), (assign83040_e124656 * (((var_dps_dc_dn13 * var_h0) - (var_dps_dc * var_h0_dn13)) / (var_h0 * var_h0))), (assign83040_e124656 * (((var_dps_dc_dn14 * var_h0) - (var_dps_dc * var_h0_dn14)) / (var_h0 * var_h0))), (assign83040_e124656 * (((var_dps_dc_dn15 * var_h0) - (var_dps_dc * var_h0_dn15)) / (var_h0 * var_h0))), (assign83040_e124656 * (((var_dps_dc_dn16 * var_h0) - (var_dps_dc * var_h0_dn16)) / (var_h0 * var_h0))), (assign83040_e124656 * (((var_dps_dc_dn17 * var_h0) - (var_dps_dc * var_h0_dn17)) / (var_h0 * var_h0))), (assign83040_e124656 * (((var_dps_dc_dn18 * var_h0) - (var_dps_dc * var_h0_dn18)) / (var_h0 * var_h0))), (assign83040_e124656 * (((var_dps_dc_dn19 * var_h0) - (var_dps_dc * var_h0_dn19)) / (var_h0 * var_h0))), (assign83040_e124656 * (((var_dps_dc_dn20 * var_h0) - (var_dps_dc * var_h0_dn20)) / (var_h0 * var_h0))),)
    } else {
        (var_sqt2, var_sqt2_dn5, var_sqt2_dn6, var_sqt2_dn7, var_sqt2_dn8, var_sqt2_dn12, var_sqt2_dn13, var_sqt2_dn14, var_sqt2_dn15, var_sqt2_dn16, var_sqt2_dn17, var_sqt2_dn18, var_sqt2_dn19, var_sqt2_dn20,)
    }
};
        var_sqt2 = assign83040_e124662;
        var_sqt2_dn5 = assign83040_e124662_d_n5;
        var_sqt2_dn6 = assign83040_e124662_d_n6;
        var_sqt2_dn7 = assign83040_e124662_d_n7;
        var_sqt2_dn8 = assign83040_e124662_d_n8;
        var_sqt2_dn12 = assign83040_e124662_d_n12;
        var_sqt2_dn13 = assign83040_e124662_d_n13;
        var_sqt2_dn14 = assign83040_e124662_d_n14;
        var_sqt2_dn15 = assign83040_e124662_d_n15;
        var_sqt2_dn16 = assign83040_e124662_d_n16;
        var_sqt2_dn17 = assign83040_e124662_d_n17;
        var_sqt2_dn18 = assign83040_e124662_d_n18;
        var_sqt2_dn19 = assign83040_e124662_d_n19;
        var_sqt2_dn20 = assign83040_e124662_d_n20;

        let (assign83050_e124670, assign83050_e124670_d_n5, assign83050_e124670_d_n6, assign83050_e124670_d_n7, assign83050_e124670_d_n8, assign83050_e124670_d_n12, assign83050_e124670_d_n13, assign83050_e124670_d_n14, assign83050_e124670_d_n15, assign83050_e124670_d_n16, assign83050_e124670_d_n17, assign83050_e124670_d_n18, assign83050_e124670_d_n19, assign83050_e124670_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) {
        let assign83050_e124668: f64 = (var_sqt2 * var_sqt2);
        (assign83050_e124668, ((var_sqt2_dn5 * var_sqt2) + (var_sqt2 * var_sqt2_dn5)), ((var_sqt2_dn6 * var_sqt2) + (var_sqt2 * var_sqt2_dn6)), ((var_sqt2_dn7 * var_sqt2) + (var_sqt2 * var_sqt2_dn7)), ((var_sqt2_dn8 * var_sqt2) + (var_sqt2 * var_sqt2_dn8)), ((var_sqt2_dn12 * var_sqt2) + (var_sqt2 * var_sqt2_dn12)), ((var_sqt2_dn13 * var_sqt2) + (var_sqt2 * var_sqt2_dn13)), ((var_sqt2_dn14 * var_sqt2) + (var_sqt2 * var_sqt2_dn14)), ((var_sqt2_dn15 * var_sqt2) + (var_sqt2 * var_sqt2_dn15)), ((var_sqt2_dn16 * var_sqt2) + (var_sqt2 * var_sqt2_dn16)), ((var_sqt2_dn17 * var_sqt2) + (var_sqt2 * var_sqt2_dn17)), ((var_sqt2_dn18 * var_sqt2) + (var_sqt2 * var_sqt2_dn18)), ((var_sqt2_dn19 * var_sqt2) + (var_sqt2 * var_sqt2_dn19)), ((var_sqt2_dn20 * var_sqt2) + (var_sqt2 * var_sqt2_dn20)),)
    } else {
        (var_t2, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn12, var_t2_dn13, var_t2_dn14, var_t2_dn15, var_t2_dn16, var_t2_dn17, var_t2_dn18, var_t2_dn19, var_t2_dn20,)
    }
};
        var_t2 = assign83050_e124670;
        var_t2_dn5 = assign83050_e124670_d_n5;
        var_t2_dn6 = assign83050_e124670_d_n6;
        var_t2_dn7 = assign83050_e124670_d_n7;
        var_t2_dn8 = assign83050_e124670_d_n8;
        var_t2_dn12 = assign83050_e124670_d_n12;
        var_t2_dn13 = assign83050_e124670_d_n13;
        var_t2_dn14 = assign83050_e124670_d_n14;
        var_t2_dn15 = assign83050_e124670_d_n15;
        var_t2_dn16 = assign83050_e124670_d_n16;
        var_t2_dn17 = assign83050_e124670_d_n17;
        var_t2_dn18 = assign83050_e124670_d_n18;
        var_t2_dn19 = assign83050_e124670_d_n19;
        var_t2_dn20 = assign83050_e124670_d_n20;

        let (assign83060_e124680, assign83060_e124680_d_n5, assign83060_e124680_d_n6, assign83060_e124680_d_n7, assign83060_e124680_d_n8, assign83060_e124680_d_n12, assign83060_e124680_d_n13, assign83060_e124680_d_n14, assign83060_e124680_d_n15, assign83060_e124680_d_n16, assign83060_e124680_d_n17, assign83060_e124680_d_n18, assign83060_e124680_d_n19, assign83060_e124680_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) {
        let assign83060_e124676: f64 = (var_h0 / var_h_dc);
        let assign83060_e124678: f64 = (assign83060_e124676 - 1.0);
        (assign83060_e124678, (((var_h0_dn5 * var_h_dc) - (var_h0 * var_h_dc_dn5)) / (var_h_dc * var_h_dc)), (((var_h0_dn6 * var_h_dc) - (var_h0 * var_h_dc_dn6)) / (var_h_dc * var_h_dc)), (((var_h0_dn7 * var_h_dc) - (var_h0 * var_h_dc_dn7)) / (var_h_dc * var_h_dc)), (((var_h0_dn8 * var_h_dc) - (var_h0 * var_h_dc_dn8)) / (var_h_dc * var_h_dc)), (((var_h0_dn12 * var_h_dc) - (var_h0 * var_h_dc_dn12)) / (var_h_dc * var_h_dc)), (((var_h0_dn13 * var_h_dc) - (var_h0 * var_h_dc_dn13)) / (var_h_dc * var_h_dc)), (((var_h0_dn14 * var_h_dc) - (var_h0 * var_h_dc_dn14)) / (var_h_dc * var_h_dc)), (((var_h0_dn15 * var_h_dc) - (var_h0 * var_h_dc_dn15)) / (var_h_dc * var_h_dc)), (((var_h0_dn16 * var_h_dc) - (var_h0 * var_h_dc_dn16)) / (var_h_dc * var_h_dc)), (((var_h0_dn17 * var_h_dc) - (var_h0 * var_h_dc_dn17)) / (var_h_dc * var_h_dc)), (((var_h0_dn18 * var_h_dc) - (var_h0 * var_h_dc_dn18)) / (var_h_dc * var_h_dc)), (((var_h0_dn19 * var_h_dc) - (var_h0 * var_h_dc_dn19)) / (var_h_dc * var_h_dc)), (((var_h0_dn20 * var_h_dc) - (var_h0 * var_h_dc_dn20)) / (var_h_dc * var_h_dc)),)
    } else {
        (var_r, var_r_dn5, var_r_dn6, var_r_dn7, var_r_dn8, var_r_dn12, var_r_dn13, var_r_dn14, var_r_dn15, var_r_dn16, var_r_dn17, var_r_dn18, var_r_dn19, var_r_dn20,)
    }
};
        var_r = assign83060_e124680;
        var_r_dn5 = assign83060_e124680_d_n5;
        var_r_dn6 = assign83060_e124680_d_n6;
        var_r_dn7 = assign83060_e124680_d_n7;
        var_r_dn8 = assign83060_e124680_d_n8;
        var_r_dn12 = assign83060_e124680_d_n12;
        var_r_dn13 = assign83060_e124680_d_n13;
        var_r_dn14 = assign83060_e124680_d_n14;
        var_r_dn15 = assign83060_e124680_d_n15;
        var_r_dn16 = assign83060_e124680_d_n16;
        var_r_dn17 = assign83060_e124680_d_n17;
        var_r_dn18 = assign83060_e124680_d_n18;
        var_r_dn19 = assign83060_e124680_d_n19;
        var_r_dn20 = assign83060_e124680_d_n20;

        let (assign83070_e124703, assign83070_e124703_d_n5, assign83070_e124703_d_n6, assign83070_e124703_d_n7, assign83070_e124703_d_n8, assign83070_e124703_d_n12, assign83070_e124703_d_n13, assign83070_e124703_d_n14, assign83070_e124703_d_n15, assign83070_e124703_d_n16, assign83070_e124703_d_n17, assign83070_e124703_d_n18, assign83070_e124703_d_n19, assign83070_e124703_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) {
        let assign83070_e124688: f64 = (var_r * var_t2);
        let assign83070_e124689: f64 = (12.0 * assign83070_e124688);
        let assign83070_e124690: f64 = (1.0 - assign83070_e124689);
        let (assign83070_e124701, assign83070_e124701_d_n5, assign83070_e124701_d_n6, assign83070_e124701_d_n7, assign83070_e124701_d_n8, assign83070_e124701_d_n12, assign83070_e124701_d_n13, assign83070_e124701_d_n14, assign83070_e124701_d_n15, assign83070_e124701_d_n16, assign83070_e124701_d_n17, assign83070_e124701_d_n18, assign83070_e124701_d_n19, assign83070_e124701_d_n20,) = {
            if (assign83070_e124690 > 1e-20) {
                let assign83070_e124697: f64 = (var_r * var_t2);
                let assign83070_e124698: f64 = (12.0 * assign83070_e124697);
                let assign83070_e124699: f64 = (1.0 - assign83070_e124698);
                (assign83070_e124699, (-(12.0 * ((var_r_dn5 * var_t2) + (var_r * var_t2_dn5)))), (-(12.0 * ((var_r_dn6 * var_t2) + (var_r * var_t2_dn6)))), (-(12.0 * ((var_r_dn7 * var_t2) + (var_r * var_t2_dn7)))), (-(12.0 * ((var_r_dn8 * var_t2) + (var_r * var_t2_dn8)))), (-(12.0 * ((var_r_dn12 * var_t2) + (var_r * var_t2_dn12)))), (-(12.0 * ((var_r_dn13 * var_t2) + (var_r * var_t2_dn13)))), (-(12.0 * ((var_r_dn14 * var_t2) + (var_r * var_t2_dn14)))), (-(12.0 * ((var_r_dn15 * var_t2) + (var_r * var_t2_dn15)))), (-(12.0 * ((var_r_dn16 * var_t2) + (var_r * var_t2_dn16)))), (-(12.0 * ((var_r_dn17 * var_t2) + (var_r * var_t2_dn17)))), (-(12.0 * ((var_r_dn18 * var_t2) + (var_r * var_t2_dn18)))), (-(12.0 * ((var_r_dn19 * var_t2) + (var_r * var_t2_dn19)))), (-(12.0 * ((var_r_dn20 * var_t2) + (var_r * var_t2_dn20)))),)
            } else {
                (1e-20, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign83070_e124701, assign83070_e124701_d_n5, assign83070_e124701_d_n6, assign83070_e124701_d_n7, assign83070_e124701_d_n8, assign83070_e124701_d_n12, assign83070_e124701_d_n13, assign83070_e124701_d_n14, assign83070_e124701_d_n15, assign83070_e124701_d_n16, assign83070_e124701_d_n17, assign83070_e124701_d_n18, assign83070_e124701_d_n19, assign83070_e124701_d_n20,)
    } else {
        (var_lc, var_lc_dn5, var_lc_dn6, var_lc_dn7, var_lc_dn8, var_lc_dn12, var_lc_dn13, var_lc_dn14, var_lc_dn15, var_lc_dn16, var_lc_dn17, var_lc_dn18, var_lc_dn19, var_lc_dn20,)
    }
};
        var_lc = assign83070_e124703;
        var_lc_dn5 = assign83070_e124703_d_n5;
        var_lc_dn6 = assign83070_e124703_d_n6;
        var_lc_dn7 = assign83070_e124703_d_n7;
        var_lc_dn8 = assign83070_e124703_d_n8;
        var_lc_dn12 = assign83070_e124703_d_n12;
        var_lc_dn13 = assign83070_e124703_d_n13;
        var_lc_dn14 = assign83070_e124703_d_n14;
        var_lc_dn15 = assign83070_e124703_d_n15;
        var_lc_dn16 = assign83070_e124703_d_n16;
        var_lc_dn17 = assign83070_e124703_d_n17;
        var_lc_dn18 = assign83070_e124703_d_n18;
        var_lc_dn19 = assign83070_e124703_d_n19;
        var_lc_dn20 = assign83070_e124703_d_n20;

        let (assign83080_e124713, assign83080_e124713_d_n5, assign83080_e124713_d_n6, assign83080_e124713_d_n7, assign83080_e124713_d_n8, assign83080_e124713_d_n12, assign83080_e124713_d_n13, assign83080_e124713_d_n14, assign83080_e124713_d_n15, assign83080_e124713_d_n16, assign83080_e124713_d_n17, assign83080_e124713_d_n18, assign83080_e124713_d_n19, assign83080_e124713_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) {
        let assign83080_e124710: f64 = (var_lc * var_lc);
        let assign83080_e124711: f64 = (1.0 / assign83080_e124710);
        (assign83080_e124711, (-(((var_lc_dn5 * var_lc) + (var_lc * var_lc_dn5)) / (assign83080_e124710 * assign83080_e124710))), (-(((var_lc_dn6 * var_lc) + (var_lc * var_lc_dn6)) / (assign83080_e124710 * assign83080_e124710))), (-(((var_lc_dn7 * var_lc) + (var_lc * var_lc_dn7)) / (assign83080_e124710 * assign83080_e124710))), (-(((var_lc_dn8 * var_lc) + (var_lc * var_lc_dn8)) / (assign83080_e124710 * assign83080_e124710))), (-(((var_lc_dn12 * var_lc) + (var_lc * var_lc_dn12)) / (assign83080_e124710 * assign83080_e124710))), (-(((var_lc_dn13 * var_lc) + (var_lc * var_lc_dn13)) / (assign83080_e124710 * assign83080_e124710))), (-(((var_lc_dn14 * var_lc) + (var_lc * var_lc_dn14)) / (assign83080_e124710 * assign83080_e124710))), (-(((var_lc_dn15 * var_lc) + (var_lc * var_lc_dn15)) / (assign83080_e124710 * assign83080_e124710))), (-(((var_lc_dn16 * var_lc) + (var_lc * var_lc_dn16)) / (assign83080_e124710 * assign83080_e124710))), (-(((var_lc_dn17 * var_lc) + (var_lc * var_lc_dn17)) / (assign83080_e124710 * assign83080_e124710))), (-(((var_lc_dn18 * var_lc) + (var_lc * var_lc_dn18)) / (assign83080_e124710 * assign83080_e124710))), (-(((var_lc_dn19 * var_lc) + (var_lc * var_lc_dn19)) / (assign83080_e124710 * assign83080_e124710))), (-(((var_lc_dn20 * var_lc) + (var_lc * var_lc_dn20)) / (assign83080_e124710 * assign83080_e124710))),)
    } else {
        (var_lcinv2, var_lcinv2_dn5, var_lcinv2_dn6, var_lcinv2_dn7, var_lcinv2_dn8, var_lcinv2_dn12, var_lcinv2_dn13, var_lcinv2_dn14, var_lcinv2_dn15, var_lcinv2_dn16, var_lcinv2_dn17, var_lcinv2_dn18, var_lcinv2_dn19, var_lcinv2_dn20,)
    }
};
        var_lcinv2 = assign83080_e124713;
        var_lcinv2_dn5 = assign83080_e124713_d_n5;
        var_lcinv2_dn6 = assign83080_e124713_d_n6;
        var_lcinv2_dn7 = assign83080_e124713_d_n7;
        var_lcinv2_dn8 = assign83080_e124713_d_n8;
        var_lcinv2_dn12 = assign83080_e124713_d_n12;
        var_lcinv2_dn13 = assign83080_e124713_d_n13;
        var_lcinv2_dn14 = assign83080_e124713_d_n14;
        var_lcinv2_dn15 = assign83080_e124713_d_n15;
        var_lcinv2_dn16 = assign83080_e124713_d_n16;
        var_lcinv2_dn17 = assign83080_e124713_d_n17;
        var_lcinv2_dn18 = assign83080_e124713_d_n18;
        var_lcinv2_dn19 = assign83080_e124713_d_n19;
        var_lcinv2_dn20 = assign83080_e124713_d_n20;

        let (assign83090_e124723, assign83090_e124723_d_n5, assign83090_e124723_d_n6, assign83090_e124723_d_n7, assign83090_e124723_d_n8, assign83090_e124723_d_n12, assign83090_e124723_d_n13, assign83090_e124723_d_n14, assign83090_e124723_d_n15, assign83090_e124723_d_n16, assign83090_e124723_d_n17, assign83090_e124723_d_n18, assign83090_e124723_d_n19, assign83090_e124723_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) {
        let assign83090_e124719: f64 = (var_bet_i * var_qim1_dc);
        let assign83090_e124721: f64 = (assign83090_e124719 * var_gvsatinv_dc);
        (assign83090_e124721, (((var_bet_i * var_qim1_dc_dn5) * var_gvsatinv_dc) + (assign83090_e124719 * var_gvsatinv_dc_dn5)), (((var_bet_i * var_qim1_dc_dn6) * var_gvsatinv_dc) + (assign83090_e124719 * var_gvsatinv_dc_dn6)), (((var_bet_i * var_qim1_dc_dn7) * var_gvsatinv_dc) + (assign83090_e124719 * var_gvsatinv_dc_dn7)), (((var_bet_i * var_qim1_dc_dn8) * var_gvsatinv_dc) + (assign83090_e124719 * var_gvsatinv_dc_dn8)), (((var_bet_i * var_qim1_dc_dn12) * var_gvsatinv_dc) + (assign83090_e124719 * var_gvsatinv_dc_dn12)), (((var_bet_i * var_qim1_dc_dn13) * var_gvsatinv_dc) + (assign83090_e124719 * var_gvsatinv_dc_dn13)), (((var_bet_i * var_qim1_dc_dn14) * var_gvsatinv_dc) + (assign83090_e124719 * var_gvsatinv_dc_dn14)), (((var_bet_i * var_qim1_dc_dn15) * var_gvsatinv_dc) + (assign83090_e124719 * var_gvsatinv_dc_dn15)), (((var_bet_i * var_qim1_dc_dn16) * var_gvsatinv_dc) + (assign83090_e124719 * var_gvsatinv_dc_dn16)), (((var_bet_i * var_qim1_dc_dn17) * var_gvsatinv_dc) + (assign83090_e124719 * var_gvsatinv_dc_dn17)), (((var_bet_i * var_qim1_dc_dn18) * var_gvsatinv_dc) + (assign83090_e124719 * var_gvsatinv_dc_dn18)), (((var_bet_i * var_qim1_dc_dn19) * var_gvsatinv_dc) + (assign83090_e124719 * var_gvsatinv_dc_dn19)), (((var_bet_i * var_qim1_dc_dn20) * var_gvsatinv_dc) + (assign83090_e124719 * var_gvsatinv_dc_dn20)),)
    } else {
        (var_g_ideal, var_g_ideal_dn5, var_g_ideal_dn6, var_g_ideal_dn7, var_g_ideal_dn8, var_g_ideal_dn12, var_g_ideal_dn13, var_g_ideal_dn14, var_g_ideal_dn15, var_g_ideal_dn16, var_g_ideal_dn17, var_g_ideal_dn18, var_g_ideal_dn19, var_g_ideal_dn20,)
    }
};
        var_g_ideal = assign83090_e124723;
        var_g_ideal_dn5 = assign83090_e124723_d_n5;
        var_g_ideal_dn6 = assign83090_e124723_d_n6;
        var_g_ideal_dn7 = assign83090_e124723_d_n7;
        var_g_ideal_dn8 = assign83090_e124723_d_n8;
        var_g_ideal_dn12 = assign83090_e124723_d_n12;
        var_g_ideal_dn13 = assign83090_e124723_d_n13;
        var_g_ideal_dn14 = assign83090_e124723_d_n14;
        var_g_ideal_dn15 = assign83090_e124723_d_n15;
        var_g_ideal_dn16 = assign83090_e124723_d_n16;
        var_g_ideal_dn17 = assign83090_e124723_d_n17;
        var_g_ideal_dn18 = assign83090_e124723_d_n18;
        var_g_ideal_dn19 = assign83090_e124723_d_n19;
        var_g_ideal_dn20 = assign83090_e124723_d_n20;

        let (assign83100_e124743, assign83100_e124743_d_n5, assign83100_e124743_d_n6, assign83100_e124743_d_n7, assign83100_e124743_d_n8, assign83100_e124743_d_n12, assign83100_e124743_d_n13, assign83100_e124743_d_n14, assign83100_e124743_d_n15, assign83100_e124743_d_n16, assign83100_e124743_d_n17, assign83100_e124743_d_n18, assign83100_e124743_d_n19, assign83100_e124743_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) {
        let assign83100_e124730: f64 = (12.0 * var_t2);
        let assign83100_e124731: f64 = (var_t1 + assign83100_e124730);
        let assign83100_e124735: f64 = (1.0 + var_t1);
        let assign83100_e124737: f64 = (assign83100_e124735 * var_t2);
        let assign83100_e124739: f64 = (assign83100_e124737 * var_r);
        let assign83100_e124740: f64 = (24.0 * assign83100_e124739);
        let assign83100_e124741: f64 = (assign83100_e124731 - assign83100_e124740);
        (assign83100_e124741, ((var_t1_dn5 + (12.0 * var_t2_dn5)) - (24.0 * ((((var_t1_dn5 * var_t2) + (assign83100_e124735 * var_t2_dn5)) * var_r) + (assign83100_e124737 * var_r_dn5)))), ((var_t1_dn6 + (12.0 * var_t2_dn6)) - (24.0 * ((((var_t1_dn6 * var_t2) + (assign83100_e124735 * var_t2_dn6)) * var_r) + (assign83100_e124737 * var_r_dn6)))), ((var_t1_dn7 + (12.0 * var_t2_dn7)) - (24.0 * ((((var_t1_dn7 * var_t2) + (assign83100_e124735 * var_t2_dn7)) * var_r) + (assign83100_e124737 * var_r_dn7)))), ((var_t1_dn8 + (12.0 * var_t2_dn8)) - (24.0 * ((((var_t1_dn8 * var_t2) + (assign83100_e124735 * var_t2_dn8)) * var_r) + (assign83100_e124737 * var_r_dn8)))), ((var_t1_dn12 + (12.0 * var_t2_dn12)) - (24.0 * ((((var_t1_dn12 * var_t2) + (assign83100_e124735 * var_t2_dn12)) * var_r) + (assign83100_e124737 * var_r_dn12)))), ((var_t1_dn13 + (12.0 * var_t2_dn13)) - (24.0 * ((((var_t1_dn13 * var_t2) + (assign83100_e124735 * var_t2_dn13)) * var_r) + (assign83100_e124737 * var_r_dn13)))), ((var_t1_dn14 + (12.0 * var_t2_dn14)) - (24.0 * ((((var_t1_dn14 * var_t2) + (assign83100_e124735 * var_t2_dn14)) * var_r) + (assign83100_e124737 * var_r_dn14)))), ((var_t1_dn15 + (12.0 * var_t2_dn15)) - (24.0 * ((((var_t1_dn15 * var_t2) + (assign83100_e124735 * var_t2_dn15)) * var_r) + (assign83100_e124737 * var_r_dn15)))), ((var_t1_dn16 + (12.0 * var_t2_dn16)) - (24.0 * ((((var_t1_dn16 * var_t2) + (assign83100_e124735 * var_t2_dn16)) * var_r) + (assign83100_e124737 * var_r_dn16)))), ((var_t1_dn17 + (12.0 * var_t2_dn17)) - (24.0 * ((((var_t1_dn17 * var_t2) + (assign83100_e124735 * var_t2_dn17)) * var_r) + (assign83100_e124737 * var_r_dn17)))), ((var_t1_dn18 + (12.0 * var_t2_dn18)) - (24.0 * ((((var_t1_dn18 * var_t2) + (assign83100_e124735 * var_t2_dn18)) * var_r) + (assign83100_e124737 * var_r_dn18)))), ((var_t1_dn19 + (12.0 * var_t2_dn19)) - (24.0 * ((((var_t1_dn19 * var_t2) + (assign83100_e124735 * var_t2_dn19)) * var_r) + (assign83100_e124737 * var_r_dn19)))), ((var_t1_dn20 + (12.0 * var_t2_dn20)) - (24.0 * ((((var_t1_dn20 * var_t2) + (assign83100_e124735 * var_t2_dn20)) * var_r) + (assign83100_e124737 * var_r_dn20)))),)
    } else {
        (var_mid, var_mid_dn5, var_mid_dn6, var_mid_dn7, var_mid_dn8, var_mid_dn12, var_mid_dn13, var_mid_dn14, var_mid_dn15, var_mid_dn16, var_mid_dn17, var_mid_dn18, var_mid_dn19, var_mid_dn20,)
    }
};
        var_mid = assign83100_e124743;
        var_mid_dn5 = assign83100_e124743_d_n5;
        var_mid_dn6 = assign83100_e124743_d_n6;
        var_mid_dn7 = assign83100_e124743_d_n7;
        var_mid_dn8 = assign83100_e124743_d_n8;
        var_mid_dn12 = assign83100_e124743_d_n12;
        var_mid_dn13 = assign83100_e124743_d_n13;
        var_mid_dn14 = assign83100_e124743_d_n14;
        var_mid_dn15 = assign83100_e124743_d_n15;
        var_mid_dn16 = assign83100_e124743_d_n16;
        var_mid_dn17 = assign83100_e124743_d_n17;
        var_mid_dn18 = assign83100_e124743_d_n18;
        var_mid_dn19 = assign83100_e124743_d_n19;
        var_mid_dn20 = assign83100_e124743_d_n20;

        let (assign83110_e124754, assign83110_e124754_d_n5, assign83110_e124754_d_n6, assign83110_e124754_d_n7, assign83110_e124754_d_n8, assign83110_e124754_d_n12, assign83110_e124754_d_n13, assign83110_e124754_d_n14, assign83110_e124754_d_n15, assign83110_e124754_d_n16, assign83110_e124754_d_n17, assign83110_e124754_d_n18, assign83110_e124754_d_n19, assign83110_e124754_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) {
        let (assign83110_e124752, assign83110_e124752_d_n5, assign83110_e124752_d_n6, assign83110_e124752_d_n7, assign83110_e124752_d_n8, assign83110_e124752_d_n12, assign83110_e124752_d_n13, assign83110_e124752_d_n14, assign83110_e124752_d_n15, assign83110_e124752_d_n16, assign83110_e124752_d_n17, assign83110_e124752_d_n18, assign83110_e124752_d_n19, assign83110_e124752_d_n20,) = {
            if (var_mid > 1e-40) {
                (var_mid, var_mid_dn5, var_mid_dn6, var_mid_dn7, var_mid_dn8, var_mid_dn12, var_mid_dn13, var_mid_dn14, var_mid_dn15, var_mid_dn16, var_mid_dn17, var_mid_dn18, var_mid_dn19, var_mid_dn20,)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign83110_e124752, assign83110_e124752_d_n5, assign83110_e124752_d_n6, assign83110_e124752_d_n7, assign83110_e124752_d_n8, assign83110_e124752_d_n12, assign83110_e124752_d_n13, assign83110_e124752_d_n14, assign83110_e124752_d_n15, assign83110_e124752_d_n16, assign83110_e124752_d_n17, assign83110_e124752_d_n18, assign83110_e124752_d_n19, assign83110_e124752_d_n20,)
    } else {
        (var_mid, var_mid_dn5, var_mid_dn6, var_mid_dn7, var_mid_dn8, var_mid_dn12, var_mid_dn13, var_mid_dn14, var_mid_dn15, var_mid_dn16, var_mid_dn17, var_mid_dn18, var_mid_dn19, var_mid_dn20,)
    }
};
        var_mid = assign83110_e124754;
        var_mid_dn5 = assign83110_e124754_d_n5;
        var_mid_dn6 = assign83110_e124754_d_n6;
        var_mid_dn7 = assign83110_e124754_d_n7;
        var_mid_dn8 = assign83110_e124754_d_n8;
        var_mid_dn12 = assign83110_e124754_d_n12;
        var_mid_dn13 = assign83110_e124754_d_n13;
        var_mid_dn14 = assign83110_e124754_d_n14;
        var_mid_dn15 = assign83110_e124754_d_n15;
        var_mid_dn16 = assign83110_e124754_d_n16;
        var_mid_dn17 = assign83110_e124754_d_n17;
        var_mid_dn18 = assign83110_e124754_d_n18;
        var_mid_dn19 = assign83110_e124754_d_n19;
        var_mid_dn20 = assign83110_e124754_d_n20;

        let (assign83120_e124764, assign83120_e124764_d_n5, assign83120_e124764_d_n6, assign83120_e124764_d_n7, assign83120_e124764_d_n8, assign83120_e124764_d_n12, assign83120_e124764_d_n13, assign83120_e124764_d_n14, assign83120_e124764_d_n15, assign83120_e124764_d_n16, assign83120_e124764_d_n17, assign83120_e124764_d_n18, assign83120_e124764_d_n19, assign83120_e124764_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) {
        let assign83120_e124760: f64 = (var_g_ideal * var_lcinv2);
        let assign83120_e124762: f64 = (assign83120_e124760 * var_mid);
        (assign83120_e124762, ((((var_g_ideal_dn5 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn5)) * var_mid) + (assign83120_e124760 * var_mid_dn5)), ((((var_g_ideal_dn6 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn6)) * var_mid) + (assign83120_e124760 * var_mid_dn6)), ((((var_g_ideal_dn7 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn7)) * var_mid) + (assign83120_e124760 * var_mid_dn7)), ((((var_g_ideal_dn8 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn8)) * var_mid) + (assign83120_e124760 * var_mid_dn8)), ((((var_g_ideal_dn12 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn12)) * var_mid) + (assign83120_e124760 * var_mid_dn12)), ((((var_g_ideal_dn13 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn13)) * var_mid) + (assign83120_e124760 * var_mid_dn13)), ((((var_g_ideal_dn14 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn14)) * var_mid) + (assign83120_e124760 * var_mid_dn14)), ((((var_g_ideal_dn15 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn15)) * var_mid) + (assign83120_e124760 * var_mid_dn15)), ((((var_g_ideal_dn16 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn16)) * var_mid) + (assign83120_e124760 * var_mid_dn16)), ((((var_g_ideal_dn17 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn17)) * var_mid) + (assign83120_e124760 * var_mid_dn17)), ((((var_g_ideal_dn18 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn18)) * var_mid) + (assign83120_e124760 * var_mid_dn18)), ((((var_g_ideal_dn19 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn19)) * var_mid) + (assign83120_e124760 * var_mid_dn19)), ((((var_g_ideal_dn20 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn20)) * var_mid) + (assign83120_e124760 * var_mid_dn20)),)
    } else {
        (var_mid, var_mid_dn5, var_mid_dn6, var_mid_dn7, var_mid_dn8, var_mid_dn12, var_mid_dn13, var_mid_dn14, var_mid_dn15, var_mid_dn16, var_mid_dn17, var_mid_dn18, var_mid_dn19, var_mid_dn20,)
    }
};
        var_mid = assign83120_e124764;
        var_mid_dn5 = assign83120_e124764_d_n5;
        var_mid_dn6 = assign83120_e124764_d_n6;
        var_mid_dn7 = assign83120_e124764_d_n7;
        var_mid_dn8 = assign83120_e124764_d_n8;
        var_mid_dn12 = assign83120_e124764_d_n12;
        var_mid_dn13 = assign83120_e124764_d_n13;
        var_mid_dn14 = assign83120_e124764_d_n14;
        var_mid_dn15 = assign83120_e124764_d_n15;
        var_mid_dn16 = assign83120_e124764_d_n16;
        var_mid_dn17 = assign83120_e124764_d_n17;
        var_mid_dn18 = assign83120_e124764_d_n18;
        var_mid_dn19 = assign83120_e124764_d_n19;
        var_mid_dn20 = assign83120_e124764_d_n20;

        let assign83130_e124767: f64 = if var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        var_guard2282 = assign83130_e124767;

        *var_c_igid_slot = var_c_igid;
        *var_c_igid_dn12_slot = var_c_igid_dn12;
        *var_c_igid_dn13_slot = var_c_igid_dn13;
        *var_c_igid_dn14_slot = var_c_igid_dn14;
        *var_c_igid_dn15_slot = var_c_igid_dn15;
        *var_c_igid_dn16_slot = var_c_igid_dn16;
        *var_c_igid_dn17_slot = var_c_igid_dn17;
        *var_c_igid_dn18_slot = var_c_igid_dn18;
        *var_c_igid_dn19_slot = var_c_igid_dn19;
        *var_c_igid_dn20_slot = var_c_igid_dn20;
        *var_c_igid_dn5_slot = var_c_igid_dn5;
        *var_c_igid_dn6_slot = var_c_igid_dn6;
        *var_c_igid_dn7_slot = var_c_igid_dn7;
        *var_c_igid_dn8_slot = var_c_igid_dn8;
        *var_cgeff_slot = var_cgeff;
        *var_cgeff_dn12_slot = var_cgeff_dn12;
        *var_cgeff_dn13_slot = var_cgeff_dn13;
        *var_cgeff_dn14_slot = var_cgeff_dn14;
        *var_cgeff_dn15_slot = var_cgeff_dn15;
        *var_cgeff_dn16_slot = var_cgeff_dn16;
        *var_cgeff_dn17_slot = var_cgeff_dn17;
        *var_cgeff_dn18_slot = var_cgeff_dn18;
        *var_cgeff_dn19_slot = var_cgeff_dn19;
        *var_cgeff_dn20_slot = var_cgeff_dn20;
        *var_cgeff_dn5_slot = var_cgeff_dn5;
        *var_cgeff_dn6_slot = var_cgeff_dn6;
        *var_cgeff_dn7_slot = var_cgeff_dn7;
        *var_cgeff_dn8_slot = var_cgeff_dn8;
        *var_g_ideal_slot = var_g_ideal;
        *var_g_ideal_dn12_slot = var_g_ideal_dn12;
        *var_g_ideal_dn13_slot = var_g_ideal_dn13;
        *var_g_ideal_dn14_slot = var_g_ideal_dn14;
        *var_g_ideal_dn15_slot = var_g_ideal_dn15;
        *var_g_ideal_dn16_slot = var_g_ideal_dn16;
        *var_g_ideal_dn17_slot = var_g_ideal_dn17;
        *var_g_ideal_dn18_slot = var_g_ideal_dn18;
        *var_g_ideal_dn19_slot = var_g_ideal_dn19;
        *var_g_ideal_dn20_slot = var_g_ideal_dn20;
        *var_g_ideal_dn5_slot = var_g_ideal_dn5;
        *var_g_ideal_dn6_slot = var_g_ideal_dn6;
        *var_g_ideal_dn7_slot = var_g_ideal_dn7;
        *var_g_ideal_dn8_slot = var_g_ideal_dn8;
        *var_guard2246_slot = var_guard2246;
        *var_guard2279_slot = var_guard2279;
        *var_guard2281_slot = var_guard2281;
        *var_guard2282_slot = var_guard2282;
        *var_h0_slot = var_h0;
        *var_h0_dn12_slot = var_h0_dn12;
        *var_h0_dn13_slot = var_h0_dn13;
        *var_h0_dn14_slot = var_h0_dn14;
        *var_h0_dn15_slot = var_h0_dn15;
        *var_h0_dn16_slot = var_h0_dn16;
        *var_h0_dn17_slot = var_h0_dn17;
        *var_h0_dn18_slot = var_h0_dn18;
        *var_h0_dn19_slot = var_h0_dn19;
        *var_h0_dn20_slot = var_h0_dn20;
        *var_h0_dn5_slot = var_h0_dn5;
        *var_h0_dn6_slot = var_h0_dn6;
        *var_h0_dn7_slot = var_h0_dn7;
        *var_h0_dn8_slot = var_h0_dn8;
        *var_lc_slot = var_lc;
        *var_lc_dn12_slot = var_lc_dn12;
        *var_lc_dn13_slot = var_lc_dn13;
        *var_lc_dn14_slot = var_lc_dn14;
        *var_lc_dn15_slot = var_lc_dn15;
        *var_lc_dn16_slot = var_lc_dn16;
        *var_lc_dn17_slot = var_lc_dn17;
        *var_lc_dn18_slot = var_lc_dn18;
        *var_lc_dn19_slot = var_lc_dn19;
        *var_lc_dn20_slot = var_lc_dn20;
        *var_lc_dn5_slot = var_lc_dn5;
        *var_lc_dn6_slot = var_lc_dn6;
        *var_lc_dn7_slot = var_lc_dn7;
        *var_lc_dn8_slot = var_lc_dn8;
        *var_lcinv2_slot = var_lcinv2;
        *var_lcinv2_dn12_slot = var_lcinv2_dn12;
        *var_lcinv2_dn13_slot = var_lcinv2_dn13;
        *var_lcinv2_dn14_slot = var_lcinv2_dn14;
        *var_lcinv2_dn15_slot = var_lcinv2_dn15;
        *var_lcinv2_dn16_slot = var_lcinv2_dn16;
        *var_lcinv2_dn17_slot = var_lcinv2_dn17;
        *var_lcinv2_dn18_slot = var_lcinv2_dn18;
        *var_lcinv2_dn19_slot = var_lcinv2_dn19;
        *var_lcinv2_dn20_slot = var_lcinv2_dn20;
        *var_lcinv2_dn5_slot = var_lcinv2_dn5;
        *var_lcinv2_dn6_slot = var_lcinv2_dn6;
        *var_lcinv2_dn7_slot = var_lcinv2_dn7;
        *var_lcinv2_dn8_slot = var_lcinv2_dn8;
        *var_mid_slot = var_mid;
        *var_mid_dn12_slot = var_mid_dn12;
        *var_mid_dn13_slot = var_mid_dn13;
        *var_mid_dn14_slot = var_mid_dn14;
        *var_mid_dn15_slot = var_mid_dn15;
        *var_mid_dn16_slot = var_mid_dn16;
        *var_mid_dn17_slot = var_mid_dn17;
        *var_mid_dn18_slot = var_mid_dn18;
        *var_mid_dn19_slot = var_mid_dn19;
        *var_mid_dn20_slot = var_mid_dn20;
        *var_mid_dn5_slot = var_mid_dn5;
        *var_mid_dn6_slot = var_mid_dn6;
        *var_mid_dn7_slot = var_mid_dn7;
        *var_mid_dn8_slot = var_mid_dn8;
        *var_mig_slot = var_mig;
        *var_mig_dn12_slot = var_mig_dn12;
        *var_mig_dn13_slot = var_mig_dn13;
        *var_mig_dn14_slot = var_mig_dn14;
        *var_mig_dn15_slot = var_mig_dn15;
        *var_mig_dn16_slot = var_mig_dn16;
        *var_mig_dn17_slot = var_mig_dn17;
        *var_mig_dn18_slot = var_mig_dn18;
        *var_mig_dn19_slot = var_mig_dn19;
        *var_mig_dn20_slot = var_mig_dn20;
        *var_mig_dn5_slot = var_mig_dn5;
        *var_mig_dn6_slot = var_mig_dn6;
        *var_mig_dn7_slot = var_mig_dn7;
        *var_mig_dn8_slot = var_mig_dn8;
        *var_migid_slot = var_migid;
        *var_migid_dn12_slot = var_migid_dn12;
        *var_migid_dn13_slot = var_migid_dn13;
        *var_migid_dn14_slot = var_migid_dn14;
        *var_migid_dn15_slot = var_migid_dn15;
        *var_migid_dn16_slot = var_migid_dn16;
        *var_migid_dn17_slot = var_migid_dn17;
        *var_migid_dn18_slot = var_migid_dn18;
        *var_migid_dn19_slot = var_migid_dn19;
        *var_migid_dn20_slot = var_migid_dn20;
        *var_migid_dn5_slot = var_migid_dn5;
        *var_migid_dn6_slot = var_migid_dn6;
        *var_migid_dn7_slot = var_migid_dn7;
        *var_migid_dn8_slot = var_migid_dn8;
        *var_qd_slot = var_qd;
        *var_qd_dn12_slot = var_qd_dn12;
        *var_qd_dn13_slot = var_qd_dn13;
        *var_qd_dn14_slot = var_qd_dn14;
        *var_qd_dn15_slot = var_qd_dn15;
        *var_qd_dn16_slot = var_qd_dn16;
        *var_qd_dn17_slot = var_qd_dn17;
        *var_qd_dn18_slot = var_qd_dn18;
        *var_qd_dn19_slot = var_qd_dn19;
        *var_qd_dn20_slot = var_qd_dn20;
        *var_qd_dn5_slot = var_qd_dn5;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qjun_d_slot = var_qjun_d;
        *var_qjun_d_dn10_slot = var_qjun_d_dn10;
        *var_qjun_d_dn11_slot = var_qjun_d_dn11;
        *var_qjun_d_dn5_slot = var_qjun_d_dn5;
        *var_qjun_d_dn6_slot = var_qjun_d_dn6;
        *var_qjun_d_dn7_slot = var_qjun_d_dn7;
        *var_qjun_d_dn8_slot = var_qjun_d_dn8;
        *var_qs_slot = var_qs;
        *var_qs_dn12_slot = var_qs_dn12;
        *var_qs_dn13_slot = var_qs_dn13;
        *var_qs_dn14_slot = var_qs_dn14;
        *var_qs_dn15_slot = var_qs_dn15;
        *var_qs_dn16_slot = var_qs_dn16;
        *var_qs_dn17_slot = var_qs_dn17;
        *var_qs_dn18_slot = var_qs_dn18;
        *var_qs_dn19_slot = var_qs_dn19;
        *var_qs_dn20_slot = var_qs_dn20;
        *var_qs_dn5_slot = var_qs_dn5;
        *var_qs_dn6_slot = var_qs_dn6;
        *var_qs_dn7_slot = var_qs_dn7;
        *var_qs_dn8_slot = var_qs_dn8;
        *var_r_slot = var_r;
        *var_r_dn12_slot = var_r_dn12;
        *var_r_dn13_slot = var_r_dn13;
        *var_r_dn14_slot = var_r_dn14;
        *var_r_dn15_slot = var_r_dn15;
        *var_r_dn16_slot = var_r_dn16;
        *var_r_dn17_slot = var_r_dn17;
        *var_r_dn18_slot = var_r_dn18;
        *var_r_dn19_slot = var_r_dn19;
        *var_r_dn20_slot = var_r_dn20;
        *var_r_dn5_slot = var_r_dn5;
        *var_r_dn6_slot = var_r_dn6;
        *var_r_dn7_slot = var_r_dn7;
        *var_r_dn8_slot = var_r_dn8;
        *var_sidexc_slot = var_sidexc;
        *var_sidexc_dn12_slot = var_sidexc_dn12;
        *var_sidexc_dn13_slot = var_sidexc_dn13;
        *var_sidexc_dn14_slot = var_sidexc_dn14;
        *var_sidexc_dn15_slot = var_sidexc_dn15;
        *var_sidexc_dn16_slot = var_sidexc_dn16;
        *var_sidexc_dn17_slot = var_sidexc_dn17;
        *var_sidexc_dn18_slot = var_sidexc_dn18;
        *var_sidexc_dn19_slot = var_sidexc_dn19;
        *var_sidexc_dn20_slot = var_sidexc_dn20;
        *var_sidexc_dn5_slot = var_sidexc_dn5;
        *var_sidexc_dn6_slot = var_sidexc_dn6;
        *var_sidexc_dn7_slot = var_sidexc_dn7;
        *var_sidexc_dn8_slot = var_sidexc_dn8;
        *var_sqid_slot = var_sqid;
        *var_sqid_dn12_slot = var_sqid_dn12;
        *var_sqid_dn13_slot = var_sqid_dn13;
        *var_sqid_dn14_slot = var_sqid_dn14;
        *var_sqid_dn15_slot = var_sqid_dn15;
        *var_sqid_dn16_slot = var_sqid_dn16;
        *var_sqid_dn17_slot = var_sqid_dn17;
        *var_sqid_dn18_slot = var_sqid_dn18;
        *var_sqid_dn19_slot = var_sqid_dn19;
        *var_sqid_dn20_slot = var_sqid_dn20;
        *var_sqid_dn5_slot = var_sqid_dn5;
        *var_sqid_dn6_slot = var_sqid_dn6;
        *var_sqid_dn7_slot = var_sqid_dn7;
        *var_sqid_dn8_slot = var_sqid_dn8;
        *var_sqig_slot = var_sqig;
        *var_sqig_dn12_slot = var_sqig_dn12;
        *var_sqig_dn13_slot = var_sqig_dn13;
        *var_sqig_dn14_slot = var_sqig_dn14;
        *var_sqig_dn15_slot = var_sqig_dn15;
        *var_sqig_dn16_slot = var_sqig_dn16;
        *var_sqig_dn17_slot = var_sqig_dn17;
        *var_sqig_dn18_slot = var_sqig_dn18;
        *var_sqig_dn19_slot = var_sqig_dn19;
        *var_sqig_dn20_slot = var_sqig_dn20;
        *var_sqig_dn5_slot = var_sqig_dn5;
        *var_sqig_dn6_slot = var_sqig_dn6;
        *var_sqig_dn7_slot = var_sqig_dn7;
        *var_sqig_dn8_slot = var_sqig_dn8;
        *var_sqt2_slot = var_sqt2;
        *var_sqt2_dn12_slot = var_sqt2_dn12;
        *var_sqt2_dn13_slot = var_sqt2_dn13;
        *var_sqt2_dn14_slot = var_sqt2_dn14;
        *var_sqt2_dn15_slot = var_sqt2_dn15;
        *var_sqt2_dn16_slot = var_sqt2_dn16;
        *var_sqt2_dn17_slot = var_sqt2_dn17;
        *var_sqt2_dn18_slot = var_sqt2_dn18;
        *var_sqt2_dn19_slot = var_sqt2_dn19;
        *var_sqt2_dn20_slot = var_sqt2_dn20;
        *var_sqt2_dn5_slot = var_sqt2_dn5;
        *var_sqt2_dn6_slot = var_sqt2_dn6;
        *var_sqt2_dn7_slot = var_sqt2_dn7;
        *var_sqt2_dn8_slot = var_sqt2_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn15_slot = var_t1_dn15;
        *var_t1_dn16_slot = var_t1_dn16;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn18_slot = var_t1_dn18;
        *var_t1_dn19_slot = var_t1_dn19;
        *var_t1_dn20_slot = var_t1_dn20;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn15_slot = var_t2_dn15;
        *var_t2_dn16_slot = var_t2_dn16;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn18_slot = var_t2_dn18;
        *var_t2_dn19_slot = var_t2_dn19;
        *var_t2_dn20_slot = var_t2_dn20;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_temp__blk2245_slot = var_temp__blk2245;
        *var_temp__blk2245_dn12_slot = var_temp__blk2245_dn12;
        *var_temp__blk2245_dn13_slot = var_temp__blk2245_dn13;
        *var_temp__blk2245_dn14_slot = var_temp__blk2245_dn14;
        *var_temp__blk2245_dn15_slot = var_temp__blk2245_dn15;
        *var_temp__blk2245_dn16_slot = var_temp__blk2245_dn16;
        *var_temp__blk2245_dn17_slot = var_temp__blk2245_dn17;
        *var_temp__blk2245_dn18_slot = var_temp__blk2245_dn18;
        *var_temp__blk2245_dn19_slot = var_temp__blk2245_dn19;
        *var_temp__blk2245_dn20_slot = var_temp__blk2245_dn20;
        *var_temp__blk2245_dn5_slot = var_temp__blk2245_dn5;
        *var_temp__blk2245_dn6_slot = var_temp__blk2245_dn6;
        *var_temp__blk2245_dn7_slot = var_temp__blk2245_dn7;
        *var_temp__blk2245_dn8_slot = var_temp__blk2245_dn8;
    }

    pub(super) fn stamp_transient_block_260(
        p: &Parameters,
        var_chnl_type: f64,
        var_cox_qm: f64,
        var_cox_qm_dn12: f64,
        var_cox_qm_dn13: f64,
        var_cox_qm_dn14: f64,
        var_cox_qm_dn15: f64,
        var_cox_qm_dn16: f64,
        var_cox_qm_dn17: f64,
        var_cox_qm_dn18: f64,
        var_cox_qm_dn19: f64,
        var_cox_qm_dn20: f64,
        var_cox_qm_dn5: f64,
        var_cox_qm_dn6: f64,
        var_cox_qm_dn7: f64,
        var_cox_qm_dn8: f64,
        var_dps_dc: f64,
        var_dps_dc_dn12: f64,
        var_dps_dc_dn13: f64,
        var_dps_dc_dn14: f64,
        var_dps_dc_dn15: f64,
        var_dps_dc_dn16: f64,
        var_dps_dc_dn17: f64,
        var_dps_dc_dn18: f64,
        var_dps_dc_dn19: f64,
        var_dps_dc_dn20: f64,
        var_dps_dc_dn5: f64,
        var_dps_dc_dn6: f64,
        var_dps_dc_dn7: f64,
        var_dps_dc_dn8: f64,
        var_eta_p_ac: f64,
        var_eta_p_ac_dn12: f64,
        var_eta_p_ac_dn13: f64,
        var_eta_p_ac_dn14: f64,
        var_eta_p_ac_dn15: f64,
        var_eta_p_ac_dn16: f64,
        var_eta_p_ac_dn17: f64,
        var_eta_p_ac_dn18: f64,
        var_eta_p_ac_dn19: f64,
        var_eta_p_ac_dn20: f64,
        var_eta_p_ac_dn5: f64,
        var_eta_p_ac_dn6: f64,
        var_eta_p_ac_dn7: f64,
        var_eta_p_ac_dn8: f64,
        var_fac_exc: f64,
        var_fntexc_i: f64,
        var_g_ideal: f64,
        var_g_ideal_dn12: f64,
        var_g_ideal_dn13: f64,
        var_g_ideal_dn14: f64,
        var_g_ideal_dn15: f64,
        var_g_ideal_dn16: f64,
        var_g_ideal_dn17: f64,
        var_g_ideal_dn18: f64,
        var_g_ideal_dn19: f64,
        var_g_ideal_dn20: f64,
        var_g_ideal_dn5: f64,
        var_g_ideal_dn6: f64,
        var_g_ideal_dn7: f64,
        var_g_ideal_dn8: f64,
        var_gmob_dc: f64,
        var_gmob_dc_dn12: f64,
        var_gmob_dc_dn13: f64,
        var_gmob_dc_dn14: f64,
        var_gmob_dc_dn15: f64,
        var_gmob_dc_dn16: f64,
        var_gmob_dc_dn17: f64,
        var_gmob_dc_dn18: f64,
        var_gmob_dc_dn19: f64,
        var_gmob_dc_dn20: f64,
        var_gmob_dc_dn5: f64,
        var_gmob_dc_dn6: f64,
        var_gmob_dc_dn7: f64,
        var_gmob_dc_dn8: f64,
        var_gmob_dl_ac: f64,
        var_gmob_dl_ac_dn12: f64,
        var_gmob_dl_ac_dn13: f64,
        var_gmob_dl_ac_dn14: f64,
        var_gmob_dl_ac_dn15: f64,
        var_gmob_dl_ac_dn16: f64,
        var_gmob_dl_ac_dn17: f64,
        var_gmob_dl_ac_dn18: f64,
        var_gmob_dl_ac_dn19: f64,
        var_gmob_dl_ac_dn20: f64,
        var_gmob_dl_ac_dn5: f64,
        var_gmob_dl_ac_dn6: f64,
        var_gmob_dl_ac_dn7: f64,
        var_gmob_dl_ac_dn8: f64,
        var_guard2279: f64,
        var_guard2281: f64,
        var_guard2282: f64,
        var_gvsat_ac: f64,
        var_gvsat_ac_dn12: f64,
        var_gvsat_ac_dn13: f64,
        var_gvsat_ac_dn14: f64,
        var_gvsat_ac_dn15: f64,
        var_gvsat_ac_dn16: f64,
        var_gvsat_ac_dn17: f64,
        var_gvsat_ac_dn18: f64,
        var_gvsat_ac_dn19: f64,
        var_gvsat_ac_dn20: f64,
        var_gvsat_ac_dn5: f64,
        var_gvsat_ac_dn6: f64,
        var_gvsat_ac_dn7: f64,
        var_gvsat_ac_dn8: f64,
        var_i_ds: f64,
        var_i_ds_dn12: f64,
        var_i_ds_dn13: f64,
        var_i_ds_dn14: f64,
        var_i_ds_dn15: f64,
        var_i_ds_dn16: f64,
        var_i_ds_dn17: f64,
        var_i_ds_dn18: f64,
        var_i_ds_dn19: f64,
        var_i_ds_dn20: f64,
        var_i_ds_dn5: f64,
        var_i_ds_dn6: f64,
        var_i_ds_dn7: f64,
        var_i_ds_dn8: f64,
        var_lc: f64,
        var_lc_dn12: f64,
        var_lc_dn13: f64,
        var_lc_dn14: f64,
        var_lc_dn15: f64,
        var_lc_dn16: f64,
        var_lc_dn17: f64,
        var_lc_dn18: f64,
        var_lc_dn19: f64,
        var_lc_dn20: f64,
        var_lc_dn5: f64,
        var_lc_dn6: f64,
        var_lc_dn7: f64,
        var_lc_dn8: f64,
        var_lcinv2: f64,
        var_lcinv2_dn12: f64,
        var_lcinv2_dn13: f64,
        var_lcinv2_dn14: f64,
        var_lcinv2_dn15: f64,
        var_lcinv2_dn16: f64,
        var_lcinv2_dn17: f64,
        var_lcinv2_dn18: f64,
        var_lcinv2_dn19: f64,
        var_lcinv2_dn20: f64,
        var_lcinv2_dn5: f64,
        var_lcinv2_dn6: f64,
        var_lcinv2_dn7: f64,
        var_lcinv2_dn8: f64,
        var_nt: f64,
        var_nt0: f64,
        var_r: f64,
        var_r_dn12: f64,
        var_r_dn13: f64,
        var_r_dn14: f64,
        var_r_dn15: f64,
        var_r_dn16: f64,
        var_r_dn17: f64,
        var_r_dn18: f64,
        var_r_dn19: f64,
        var_r_dn20: f64,
        var_r_dn5: f64,
        var_r_dn6: f64,
        var_r_dn7: f64,
        var_r_dn8: f64,
        var_sqt2: f64,
        var_sqt2_dn12: f64,
        var_sqt2_dn13: f64,
        var_sqt2_dn14: f64,
        var_sqt2_dn15: f64,
        var_sqt2_dn16: f64,
        var_sqt2_dn17: f64,
        var_sqt2_dn18: f64,
        var_sqt2_dn19: f64,
        var_sqt2_dn20: f64,
        var_sqt2_dn5: f64,
        var_sqt2_dn6: f64,
        var_sqt2_dn7: f64,
        var_sqt2_dn8: f64,
        var_t1: f64,
        var_t1_dn12: f64,
        var_t1_dn13: f64,
        var_t1_dn14: f64,
        var_t1_dn15: f64,
        var_t1_dn16: f64,
        var_t1_dn17: f64,
        var_t1_dn18: f64,
        var_t1_dn19: f64,
        var_t1_dn20: f64,
        var_t1_dn5: f64,
        var_t1_dn6: f64,
        var_t1_dn7: f64,
        var_t1_dn8: f64,
        var_t2: f64,
        var_t2_dn12: f64,
        var_t2_dn13: f64,
        var_t2_dn14: f64,
        var_t2_dn15: f64,
        var_t2_dn16: f64,
        var_t2_dn17: f64,
        var_t2_dn18: f64,
        var_t2_dn19: f64,
        var_t2_dn20: f64,
        var_t2_dn5: f64,
        var_t2_dn6: f64,
        var_t2_dn7: f64,
        var_t2_dn8: f64,
        var_thesateff_dc: f64,
        var_thesateff_dc_dn12: f64,
        var_thesateff_dc_dn13: f64,
        var_thesateff_dc_dn14: f64,
        var_thesateff_dc_dn15: f64,
        var_thesateff_dc_dn16: f64,
        var_thesateff_dc_dn17: f64,
        var_thesateff_dc_dn18: f64,
        var_thesateff_dc_dn19: f64,
        var_thesateff_dc_dn20: f64,
        var_thesateff_dc_dn5: f64,
        var_thesateff_dc_dn6: f64,
        var_thesateff_dc_dn7: f64,
        var_thesateff_dc_dn8: f64,
        var_vdse_dc: f64,
        var_vdse_dc_dn12: f64,
        var_vdse_dc_dn13: f64,
        var_vdse_dc_dn14: f64,
        var_vdse_dc_dn15: f64,
        var_vdse_dc_dn16: f64,
        var_vdse_dc_dn17: f64,
        var_vdse_dc_dn18: f64,
        var_vdse_dc_dn19: f64,
        var_vdse_dc_dn20: f64,
        var_vdse_dc_dn5: f64,
        var_vdse_dc_dn6: f64,
        var_vdse_dc_dn7: f64,
        var_vdse_dc_dn8: f64,
        var_c_igid_slot: &mut f64,
        var_c_igid_dn12_slot: &mut f64,
        var_c_igid_dn13_slot: &mut f64,
        var_c_igid_dn14_slot: &mut f64,
        var_c_igid_dn15_slot: &mut f64,
        var_c_igid_dn16_slot: &mut f64,
        var_c_igid_dn17_slot: &mut f64,
        var_c_igid_dn18_slot: &mut f64,
        var_c_igid_dn19_slot: &mut f64,
        var_c_igid_dn20_slot: &mut f64,
        var_c_igid_dn5_slot: &mut f64,
        var_c_igid_dn6_slot: &mut f64,
        var_c_igid_dn7_slot: &mut f64,
        var_c_igid_dn8_slot: &mut f64,
        var_cgeff_slot: &mut f64,
        var_cgeff_dn12_slot: &mut f64,
        var_cgeff_dn13_slot: &mut f64,
        var_cgeff_dn14_slot: &mut f64,
        var_cgeff_dn15_slot: &mut f64,
        var_cgeff_dn16_slot: &mut f64,
        var_cgeff_dn17_slot: &mut f64,
        var_cgeff_dn18_slot: &mut f64,
        var_cgeff_dn19_slot: &mut f64,
        var_cgeff_dn20_slot: &mut f64,
        var_cgeff_dn5_slot: &mut f64,
        var_cgeff_dn6_slot: &mut f64,
        var_cgeff_dn7_slot: &mut f64,
        var_cgeff_dn8_slot: &mut f64,
        var_gfac_slot: &mut f64,
        var_gfac_dn12_slot: &mut f64,
        var_gfac_dn13_slot: &mut f64,
        var_gfac_dn14_slot: &mut f64,
        var_gfac_dn15_slot: &mut f64,
        var_gfac_dn16_slot: &mut f64,
        var_gfac_dn17_slot: &mut f64,
        var_gfac_dn18_slot: &mut f64,
        var_gfac_dn19_slot: &mut f64,
        var_gfac_dn20_slot: &mut f64,
        var_gfac_dn5_slot: &mut f64,
        var_gfac_dn6_slot: &mut f64,
        var_gfac_dn7_slot: &mut f64,
        var_gfac_dn8_slot: &mut f64,
        var_guard2283_slot: &mut f64,
        var_guard2284_slot: &mut f64,
        var_guard2285_slot: &mut f64,
        var_guard2286_slot: &mut f64,
        var_gvsat_exc_slot: &mut f64,
        var_gvsat_exc_dn12_slot: &mut f64,
        var_gvsat_exc_dn13_slot: &mut f64,
        var_gvsat_exc_dn14_slot: &mut f64,
        var_gvsat_exc_dn15_slot: &mut f64,
        var_gvsat_exc_dn16_slot: &mut f64,
        var_gvsat_exc_dn17_slot: &mut f64,
        var_gvsat_exc_dn18_slot: &mut f64,
        var_gvsat_exc_dn19_slot: &mut f64,
        var_gvsat_exc_dn20_slot: &mut f64,
        var_gvsat_exc_dn5_slot: &mut f64,
        var_gvsat_exc_dn6_slot: &mut f64,
        var_gvsat_exc_dn7_slot: &mut f64,
        var_gvsat_exc_dn8_slot: &mut f64,
        var_mid_slot: &mut f64,
        var_mid_dn12_slot: &mut f64,
        var_mid_dn13_slot: &mut f64,
        var_mid_dn14_slot: &mut f64,
        var_mid_dn15_slot: &mut f64,
        var_mid_dn16_slot: &mut f64,
        var_mid_dn17_slot: &mut f64,
        var_mid_dn18_slot: &mut f64,
        var_mid_dn19_slot: &mut f64,
        var_mid_dn20_slot: &mut f64,
        var_mid_dn5_slot: &mut f64,
        var_mid_dn6_slot: &mut f64,
        var_mid_dn7_slot: &mut f64,
        var_mid_dn8_slot: &mut f64,
        var_mig_slot: &mut f64,
        var_mig_dn12_slot: &mut f64,
        var_mig_dn13_slot: &mut f64,
        var_mig_dn14_slot: &mut f64,
        var_mig_dn15_slot: &mut f64,
        var_mig_dn16_slot: &mut f64,
        var_mig_dn17_slot: &mut f64,
        var_mig_dn18_slot: &mut f64,
        var_mig_dn19_slot: &mut f64,
        var_mig_dn20_slot: &mut f64,
        var_mig_dn5_slot: &mut f64,
        var_mig_dn6_slot: &mut f64,
        var_mig_dn7_slot: &mut f64,
        var_mig_dn8_slot: &mut f64,
        var_migid0_slot: &mut f64,
        var_migid0_dn12_slot: &mut f64,
        var_migid0_dn13_slot: &mut f64,
        var_migid0_dn14_slot: &mut f64,
        var_migid0_dn15_slot: &mut f64,
        var_migid0_dn16_slot: &mut f64,
        var_migid0_dn17_slot: &mut f64,
        var_migid0_dn18_slot: &mut f64,
        var_migid0_dn19_slot: &mut f64,
        var_migid0_dn20_slot: &mut f64,
        var_migid0_dn5_slot: &mut f64,
        var_migid0_dn6_slot: &mut f64,
        var_migid0_dn7_slot: &mut f64,
        var_migid0_dn8_slot: &mut f64,
        var_sidexc_slot: &mut f64,
        var_sidexc_dn12_slot: &mut f64,
        var_sidexc_dn13_slot: &mut f64,
        var_sidexc_dn14_slot: &mut f64,
        var_sidexc_dn15_slot: &mut f64,
        var_sidexc_dn16_slot: &mut f64,
        var_sidexc_dn17_slot: &mut f64,
        var_sidexc_dn18_slot: &mut f64,
        var_sidexc_dn19_slot: &mut f64,
        var_sidexc_dn20_slot: &mut f64,
        var_sidexc_dn5_slot: &mut f64,
        var_sidexc_dn6_slot: &mut f64,
        var_sidexc_dn7_slot: &mut f64,
        var_sidexc_dn8_slot: &mut f64,
        var_sqid_slot: &mut f64,
        var_sqid_dn12_slot: &mut f64,
        var_sqid_dn13_slot: &mut f64,
        var_sqid_dn14_slot: &mut f64,
        var_sqid_dn15_slot: &mut f64,
        var_sqid_dn16_slot: &mut f64,
        var_sqid_dn17_slot: &mut f64,
        var_sqid_dn18_slot: &mut f64,
        var_sqid_dn19_slot: &mut f64,
        var_sqid_dn20_slot: &mut f64,
        var_sqid_dn5_slot: &mut f64,
        var_sqid_dn6_slot: &mut f64,
        var_sqid_dn7_slot: &mut f64,
        var_sqid_dn8_slot: &mut f64,
        var_sqig_slot: &mut f64,
        var_sqig_dn12_slot: &mut f64,
        var_sqig_dn13_slot: &mut f64,
        var_sqig_dn14_slot: &mut f64,
        var_sqig_dn15_slot: &mut f64,
        var_sqig_dn16_slot: &mut f64,
        var_sqig_dn17_slot: &mut f64,
        var_sqig_dn18_slot: &mut f64,
        var_sqig_dn19_slot: &mut f64,
        var_sqig_dn20_slot: &mut f64,
        var_sqig_dn5_slot: &mut f64,
        var_sqig_dn6_slot: &mut f64,
        var_sqig_dn7_slot: &mut f64,
        var_sqig_dn8_slot: &mut f64,
        var_thesat1_exc_slot: &mut f64,
        var_thesat1_exc_dn12_slot: &mut f64,
        var_thesat1_exc_dn13_slot: &mut f64,
        var_thesat1_exc_dn14_slot: &mut f64,
        var_thesat1_exc_dn15_slot: &mut f64,
        var_thesat1_exc_dn16_slot: &mut f64,
        var_thesat1_exc_dn17_slot: &mut f64,
        var_thesat1_exc_dn18_slot: &mut f64,
        var_thesat1_exc_dn19_slot: &mut f64,
        var_thesat1_exc_dn20_slot: &mut f64,
        var_thesat1_exc_dn5_slot: &mut f64,
        var_thesat1_exc_dn6_slot: &mut f64,
        var_thesat1_exc_dn7_slot: &mut f64,
        var_thesat1_exc_dn8_slot: &mut f64,
        var_zsat_exc_slot: &mut f64,
        var_zsat_exc_dn12_slot: &mut f64,
        var_zsat_exc_dn13_slot: &mut f64,
        var_zsat_exc_dn14_slot: &mut f64,
        var_zsat_exc_dn15_slot: &mut f64,
        var_zsat_exc_dn16_slot: &mut f64,
        var_zsat_exc_dn17_slot: &mut f64,
        var_zsat_exc_dn18_slot: &mut f64,
        var_zsat_exc_dn19_slot: &mut f64,
        var_zsat_exc_dn20_slot: &mut f64,
        var_zsat_exc_dn5_slot: &mut f64,
        var_zsat_exc_dn6_slot: &mut f64,
        var_zsat_exc_dn7_slot: &mut f64,
        var_zsat_exc_dn8_slot: &mut f64,
    ) {
        let mut var_c_igid: f64 = *var_c_igid_slot;
        let mut var_c_igid_dn12: f64 = *var_c_igid_dn12_slot;
        let mut var_c_igid_dn13: f64 = *var_c_igid_dn13_slot;
        let mut var_c_igid_dn14: f64 = *var_c_igid_dn14_slot;
        let mut var_c_igid_dn15: f64 = *var_c_igid_dn15_slot;
        let mut var_c_igid_dn16: f64 = *var_c_igid_dn16_slot;
        let mut var_c_igid_dn17: f64 = *var_c_igid_dn17_slot;
        let mut var_c_igid_dn18: f64 = *var_c_igid_dn18_slot;
        let mut var_c_igid_dn19: f64 = *var_c_igid_dn19_slot;
        let mut var_c_igid_dn20: f64 = *var_c_igid_dn20_slot;
        let mut var_c_igid_dn5: f64 = *var_c_igid_dn5_slot;
        let mut var_c_igid_dn6: f64 = *var_c_igid_dn6_slot;
        let mut var_c_igid_dn7: f64 = *var_c_igid_dn7_slot;
        let mut var_c_igid_dn8: f64 = *var_c_igid_dn8_slot;
        let mut var_cgeff: f64 = *var_cgeff_slot;
        let mut var_cgeff_dn12: f64 = *var_cgeff_dn12_slot;
        let mut var_cgeff_dn13: f64 = *var_cgeff_dn13_slot;
        let mut var_cgeff_dn14: f64 = *var_cgeff_dn14_slot;
        let mut var_cgeff_dn15: f64 = *var_cgeff_dn15_slot;
        let mut var_cgeff_dn16: f64 = *var_cgeff_dn16_slot;
        let mut var_cgeff_dn17: f64 = *var_cgeff_dn17_slot;
        let mut var_cgeff_dn18: f64 = *var_cgeff_dn18_slot;
        let mut var_cgeff_dn19: f64 = *var_cgeff_dn19_slot;
        let mut var_cgeff_dn20: f64 = *var_cgeff_dn20_slot;
        let mut var_cgeff_dn5: f64 = *var_cgeff_dn5_slot;
        let mut var_cgeff_dn6: f64 = *var_cgeff_dn6_slot;
        let mut var_cgeff_dn7: f64 = *var_cgeff_dn7_slot;
        let mut var_cgeff_dn8: f64 = *var_cgeff_dn8_slot;
        let mut var_gfac: f64 = *var_gfac_slot;
        let mut var_gfac_dn12: f64 = *var_gfac_dn12_slot;
        let mut var_gfac_dn13: f64 = *var_gfac_dn13_slot;
        let mut var_gfac_dn14: f64 = *var_gfac_dn14_slot;
        let mut var_gfac_dn15: f64 = *var_gfac_dn15_slot;
        let mut var_gfac_dn16: f64 = *var_gfac_dn16_slot;
        let mut var_gfac_dn17: f64 = *var_gfac_dn17_slot;
        let mut var_gfac_dn18: f64 = *var_gfac_dn18_slot;
        let mut var_gfac_dn19: f64 = *var_gfac_dn19_slot;
        let mut var_gfac_dn20: f64 = *var_gfac_dn20_slot;
        let mut var_gfac_dn5: f64 = *var_gfac_dn5_slot;
        let mut var_gfac_dn6: f64 = *var_gfac_dn6_slot;
        let mut var_gfac_dn7: f64 = *var_gfac_dn7_slot;
        let mut var_gfac_dn8: f64 = *var_gfac_dn8_slot;
        let mut var_guard2283: f64 = *var_guard2283_slot;
        let mut var_guard2284: f64 = *var_guard2284_slot;
        let mut var_guard2285: f64 = *var_guard2285_slot;
        let mut var_guard2286: f64 = *var_guard2286_slot;
        let mut var_gvsat_exc: f64 = *var_gvsat_exc_slot;
        let mut var_gvsat_exc_dn12: f64 = *var_gvsat_exc_dn12_slot;
        let mut var_gvsat_exc_dn13: f64 = *var_gvsat_exc_dn13_slot;
        let mut var_gvsat_exc_dn14: f64 = *var_gvsat_exc_dn14_slot;
        let mut var_gvsat_exc_dn15: f64 = *var_gvsat_exc_dn15_slot;
        let mut var_gvsat_exc_dn16: f64 = *var_gvsat_exc_dn16_slot;
        let mut var_gvsat_exc_dn17: f64 = *var_gvsat_exc_dn17_slot;
        let mut var_gvsat_exc_dn18: f64 = *var_gvsat_exc_dn18_slot;
        let mut var_gvsat_exc_dn19: f64 = *var_gvsat_exc_dn19_slot;
        let mut var_gvsat_exc_dn20: f64 = *var_gvsat_exc_dn20_slot;
        let mut var_gvsat_exc_dn5: f64 = *var_gvsat_exc_dn5_slot;
        let mut var_gvsat_exc_dn6: f64 = *var_gvsat_exc_dn6_slot;
        let mut var_gvsat_exc_dn7: f64 = *var_gvsat_exc_dn7_slot;
        let mut var_gvsat_exc_dn8: f64 = *var_gvsat_exc_dn8_slot;
        let mut var_mid: f64 = *var_mid_slot;
        let mut var_mid_dn12: f64 = *var_mid_dn12_slot;
        let mut var_mid_dn13: f64 = *var_mid_dn13_slot;
        let mut var_mid_dn14: f64 = *var_mid_dn14_slot;
        let mut var_mid_dn15: f64 = *var_mid_dn15_slot;
        let mut var_mid_dn16: f64 = *var_mid_dn16_slot;
        let mut var_mid_dn17: f64 = *var_mid_dn17_slot;
        let mut var_mid_dn18: f64 = *var_mid_dn18_slot;
        let mut var_mid_dn19: f64 = *var_mid_dn19_slot;
        let mut var_mid_dn20: f64 = *var_mid_dn20_slot;
        let mut var_mid_dn5: f64 = *var_mid_dn5_slot;
        let mut var_mid_dn6: f64 = *var_mid_dn6_slot;
        let mut var_mid_dn7: f64 = *var_mid_dn7_slot;
        let mut var_mid_dn8: f64 = *var_mid_dn8_slot;
        let mut var_mig: f64 = *var_mig_slot;
        let mut var_mig_dn12: f64 = *var_mig_dn12_slot;
        let mut var_mig_dn13: f64 = *var_mig_dn13_slot;
        let mut var_mig_dn14: f64 = *var_mig_dn14_slot;
        let mut var_mig_dn15: f64 = *var_mig_dn15_slot;
        let mut var_mig_dn16: f64 = *var_mig_dn16_slot;
        let mut var_mig_dn17: f64 = *var_mig_dn17_slot;
        let mut var_mig_dn18: f64 = *var_mig_dn18_slot;
        let mut var_mig_dn19: f64 = *var_mig_dn19_slot;
        let mut var_mig_dn20: f64 = *var_mig_dn20_slot;
        let mut var_mig_dn5: f64 = *var_mig_dn5_slot;
        let mut var_mig_dn6: f64 = *var_mig_dn6_slot;
        let mut var_mig_dn7: f64 = *var_mig_dn7_slot;
        let mut var_mig_dn8: f64 = *var_mig_dn8_slot;
        let mut var_migid0: f64 = *var_migid0_slot;
        let mut var_migid0_dn12: f64 = *var_migid0_dn12_slot;
        let mut var_migid0_dn13: f64 = *var_migid0_dn13_slot;
        let mut var_migid0_dn14: f64 = *var_migid0_dn14_slot;
        let mut var_migid0_dn15: f64 = *var_migid0_dn15_slot;
        let mut var_migid0_dn16: f64 = *var_migid0_dn16_slot;
        let mut var_migid0_dn17: f64 = *var_migid0_dn17_slot;
        let mut var_migid0_dn18: f64 = *var_migid0_dn18_slot;
        let mut var_migid0_dn19: f64 = *var_migid0_dn19_slot;
        let mut var_migid0_dn20: f64 = *var_migid0_dn20_slot;
        let mut var_migid0_dn5: f64 = *var_migid0_dn5_slot;
        let mut var_migid0_dn6: f64 = *var_migid0_dn6_slot;
        let mut var_migid0_dn7: f64 = *var_migid0_dn7_slot;
        let mut var_migid0_dn8: f64 = *var_migid0_dn8_slot;
        let mut var_sidexc: f64 = *var_sidexc_slot;
        let mut var_sidexc_dn12: f64 = *var_sidexc_dn12_slot;
        let mut var_sidexc_dn13: f64 = *var_sidexc_dn13_slot;
        let mut var_sidexc_dn14: f64 = *var_sidexc_dn14_slot;
        let mut var_sidexc_dn15: f64 = *var_sidexc_dn15_slot;
        let mut var_sidexc_dn16: f64 = *var_sidexc_dn16_slot;
        let mut var_sidexc_dn17: f64 = *var_sidexc_dn17_slot;
        let mut var_sidexc_dn18: f64 = *var_sidexc_dn18_slot;
        let mut var_sidexc_dn19: f64 = *var_sidexc_dn19_slot;
        let mut var_sidexc_dn20: f64 = *var_sidexc_dn20_slot;
        let mut var_sidexc_dn5: f64 = *var_sidexc_dn5_slot;
        let mut var_sidexc_dn6: f64 = *var_sidexc_dn6_slot;
        let mut var_sidexc_dn7: f64 = *var_sidexc_dn7_slot;
        let mut var_sidexc_dn8: f64 = *var_sidexc_dn8_slot;
        let mut var_sqid: f64 = *var_sqid_slot;
        let mut var_sqid_dn12: f64 = *var_sqid_dn12_slot;
        let mut var_sqid_dn13: f64 = *var_sqid_dn13_slot;
        let mut var_sqid_dn14: f64 = *var_sqid_dn14_slot;
        let mut var_sqid_dn15: f64 = *var_sqid_dn15_slot;
        let mut var_sqid_dn16: f64 = *var_sqid_dn16_slot;
        let mut var_sqid_dn17: f64 = *var_sqid_dn17_slot;
        let mut var_sqid_dn18: f64 = *var_sqid_dn18_slot;
        let mut var_sqid_dn19: f64 = *var_sqid_dn19_slot;
        let mut var_sqid_dn20: f64 = *var_sqid_dn20_slot;
        let mut var_sqid_dn5: f64 = *var_sqid_dn5_slot;
        let mut var_sqid_dn6: f64 = *var_sqid_dn6_slot;
        let mut var_sqid_dn7: f64 = *var_sqid_dn7_slot;
        let mut var_sqid_dn8: f64 = *var_sqid_dn8_slot;
        let mut var_sqig: f64 = *var_sqig_slot;
        let mut var_sqig_dn12: f64 = *var_sqig_dn12_slot;
        let mut var_sqig_dn13: f64 = *var_sqig_dn13_slot;
        let mut var_sqig_dn14: f64 = *var_sqig_dn14_slot;
        let mut var_sqig_dn15: f64 = *var_sqig_dn15_slot;
        let mut var_sqig_dn16: f64 = *var_sqig_dn16_slot;
        let mut var_sqig_dn17: f64 = *var_sqig_dn17_slot;
        let mut var_sqig_dn18: f64 = *var_sqig_dn18_slot;
        let mut var_sqig_dn19: f64 = *var_sqig_dn19_slot;
        let mut var_sqig_dn20: f64 = *var_sqig_dn20_slot;
        let mut var_sqig_dn5: f64 = *var_sqig_dn5_slot;
        let mut var_sqig_dn6: f64 = *var_sqig_dn6_slot;
        let mut var_sqig_dn7: f64 = *var_sqig_dn7_slot;
        let mut var_sqig_dn8: f64 = *var_sqig_dn8_slot;
        let mut var_thesat1_exc: f64 = *var_thesat1_exc_slot;
        let mut var_thesat1_exc_dn12: f64 = *var_thesat1_exc_dn12_slot;
        let mut var_thesat1_exc_dn13: f64 = *var_thesat1_exc_dn13_slot;
        let mut var_thesat1_exc_dn14: f64 = *var_thesat1_exc_dn14_slot;
        let mut var_thesat1_exc_dn15: f64 = *var_thesat1_exc_dn15_slot;
        let mut var_thesat1_exc_dn16: f64 = *var_thesat1_exc_dn16_slot;
        let mut var_thesat1_exc_dn17: f64 = *var_thesat1_exc_dn17_slot;
        let mut var_thesat1_exc_dn18: f64 = *var_thesat1_exc_dn18_slot;
        let mut var_thesat1_exc_dn19: f64 = *var_thesat1_exc_dn19_slot;
        let mut var_thesat1_exc_dn20: f64 = *var_thesat1_exc_dn20_slot;
        let mut var_thesat1_exc_dn5: f64 = *var_thesat1_exc_dn5_slot;
        let mut var_thesat1_exc_dn6: f64 = *var_thesat1_exc_dn6_slot;
        let mut var_thesat1_exc_dn7: f64 = *var_thesat1_exc_dn7_slot;
        let mut var_thesat1_exc_dn8: f64 = *var_thesat1_exc_dn8_slot;
        let mut var_zsat_exc: f64 = *var_zsat_exc_slot;
        let mut var_zsat_exc_dn12: f64 = *var_zsat_exc_dn12_slot;
        let mut var_zsat_exc_dn13: f64 = *var_zsat_exc_dn13_slot;
        let mut var_zsat_exc_dn14: f64 = *var_zsat_exc_dn14_slot;
        let mut var_zsat_exc_dn15: f64 = *var_zsat_exc_dn15_slot;
        let mut var_zsat_exc_dn16: f64 = *var_zsat_exc_dn16_slot;
        let mut var_zsat_exc_dn17: f64 = *var_zsat_exc_dn17_slot;
        let mut var_zsat_exc_dn18: f64 = *var_zsat_exc_dn18_slot;
        let mut var_zsat_exc_dn19: f64 = *var_zsat_exc_dn19_slot;
        let mut var_zsat_exc_dn20: f64 = *var_zsat_exc_dn20_slot;
        let mut var_zsat_exc_dn5: f64 = *var_zsat_exc_dn5_slot;
        let mut var_zsat_exc_dn6: f64 = *var_zsat_exc_dn6_slot;
        let mut var_zsat_exc_dn7: f64 = *var_zsat_exc_dn7_slot;
        let mut var_zsat_exc_dn8: f64 = *var_zsat_exc_dn8_slot;

        let (assign83140_e124777, assign83140_e124777_d_n5, assign83140_e124777_d_n6, assign83140_e124777_d_n7, assign83140_e124777_d_n8, assign83140_e124777_d_n12, assign83140_e124777_d_n13, assign83140_e124777_d_n14, assign83140_e124777_d_n15, assign83140_e124777_d_n16, assign83140_e124777_d_n17, assign83140_e124777_d_n18, assign83140_e124777_d_n19, assign83140_e124777_d_n20,) = {
    if (((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) && (var_guard2282 != 0.0)) {
        let assign83140_e124775: f64 = (var_thesateff_dc / var_gmob_dc);
        (assign83140_e124775, (((var_thesateff_dc_dn5 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn5)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn6 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn6)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn7 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn7)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn8 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn8)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn12 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn12)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn13 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn13)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn14 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn14)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn15 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn15)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn16 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn16)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn17 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn17)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn18 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn18)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn19 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn19)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn20 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn20)) / (var_gmob_dc * var_gmob_dc)),)
    } else {
        (var_thesat1_exc, var_thesat1_exc_dn5, var_thesat1_exc_dn6, var_thesat1_exc_dn7, var_thesat1_exc_dn8, var_thesat1_exc_dn12, var_thesat1_exc_dn13, var_thesat1_exc_dn14, var_thesat1_exc_dn15, var_thesat1_exc_dn16, var_thesat1_exc_dn17, var_thesat1_exc_dn18, var_thesat1_exc_dn19, var_thesat1_exc_dn20,)
    }
};
        var_thesat1_exc = assign83140_e124777;
        var_thesat1_exc_dn5 = assign83140_e124777_d_n5;
        var_thesat1_exc_dn6 = assign83140_e124777_d_n6;
        var_thesat1_exc_dn7 = assign83140_e124777_d_n7;
        var_thesat1_exc_dn8 = assign83140_e124777_d_n8;
        var_thesat1_exc_dn12 = assign83140_e124777_d_n12;
        var_thesat1_exc_dn13 = assign83140_e124777_d_n13;
        var_thesat1_exc_dn14 = assign83140_e124777_d_n14;
        var_thesat1_exc_dn15 = assign83140_e124777_d_n15;
        var_thesat1_exc_dn16 = assign83140_e124777_d_n16;
        var_thesat1_exc_dn17 = assign83140_e124777_d_n17;
        var_thesat1_exc_dn18 = assign83140_e124777_d_n18;
        var_thesat1_exc_dn19 = assign83140_e124777_d_n19;
        var_thesat1_exc_dn20 = assign83140_e124777_d_n20;

        let (assign83150_e124791, assign83150_e124791_d_n5, assign83150_e124791_d_n6, assign83150_e124791_d_n7, assign83150_e124791_d_n8, assign83150_e124791_d_n12, assign83150_e124791_d_n13, assign83150_e124791_d_n14, assign83150_e124791_d_n15, assign83150_e124791_d_n16, assign83150_e124791_d_n17, assign83150_e124791_d_n18, assign83150_e124791_d_n19, assign83150_e124791_d_n20,) = {
    if (((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) && (var_guard2282 != 0.0)) {
        let assign83150_e124785: f64 = (var_thesat1_exc * var_thesat1_exc);
        let assign83150_e124787: f64 = (assign83150_e124785 * var_dps_dc);
        let assign83150_e124789: f64 = (assign83150_e124787 * var_dps_dc);
        (assign83150_e124789, ((((((var_thesat1_exc_dn5 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn5)) * var_dps_dc) + (assign83150_e124785 * var_dps_dc_dn5)) * var_dps_dc) + (assign83150_e124787 * var_dps_dc_dn5)), ((((((var_thesat1_exc_dn6 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn6)) * var_dps_dc) + (assign83150_e124785 * var_dps_dc_dn6)) * var_dps_dc) + (assign83150_e124787 * var_dps_dc_dn6)), ((((((var_thesat1_exc_dn7 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn7)) * var_dps_dc) + (assign83150_e124785 * var_dps_dc_dn7)) * var_dps_dc) + (assign83150_e124787 * var_dps_dc_dn7)), ((((((var_thesat1_exc_dn8 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn8)) * var_dps_dc) + (assign83150_e124785 * var_dps_dc_dn8)) * var_dps_dc) + (assign83150_e124787 * var_dps_dc_dn8)), ((((((var_thesat1_exc_dn12 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn12)) * var_dps_dc) + (assign83150_e124785 * var_dps_dc_dn12)) * var_dps_dc) + (assign83150_e124787 * var_dps_dc_dn12)), ((((((var_thesat1_exc_dn13 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn13)) * var_dps_dc) + (assign83150_e124785 * var_dps_dc_dn13)) * var_dps_dc) + (assign83150_e124787 * var_dps_dc_dn13)), ((((((var_thesat1_exc_dn14 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn14)) * var_dps_dc) + (assign83150_e124785 * var_dps_dc_dn14)) * var_dps_dc) + (assign83150_e124787 * var_dps_dc_dn14)), ((((((var_thesat1_exc_dn15 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn15)) * var_dps_dc) + (assign83150_e124785 * var_dps_dc_dn15)) * var_dps_dc) + (assign83150_e124787 * var_dps_dc_dn15)), ((((((var_thesat1_exc_dn16 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn16)) * var_dps_dc) + (assign83150_e124785 * var_dps_dc_dn16)) * var_dps_dc) + (assign83150_e124787 * var_dps_dc_dn16)), ((((((var_thesat1_exc_dn17 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn17)) * var_dps_dc) + (assign83150_e124785 * var_dps_dc_dn17)) * var_dps_dc) + (assign83150_e124787 * var_dps_dc_dn17)), ((((((var_thesat1_exc_dn18 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn18)) * var_dps_dc) + (assign83150_e124785 * var_dps_dc_dn18)) * var_dps_dc) + (assign83150_e124787 * var_dps_dc_dn18)), ((((((var_thesat1_exc_dn19 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn19)) * var_dps_dc) + (assign83150_e124785 * var_dps_dc_dn19)) * var_dps_dc) + (assign83150_e124787 * var_dps_dc_dn19)), ((((((var_thesat1_exc_dn20 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn20)) * var_dps_dc) + (assign83150_e124785 * var_dps_dc_dn20)) * var_dps_dc) + (assign83150_e124787 * var_dps_dc_dn20)),)
    } else {
        (var_zsat_exc, var_zsat_exc_dn5, var_zsat_exc_dn6, var_zsat_exc_dn7, var_zsat_exc_dn8, var_zsat_exc_dn12, var_zsat_exc_dn13, var_zsat_exc_dn14, var_zsat_exc_dn15, var_zsat_exc_dn16, var_zsat_exc_dn17, var_zsat_exc_dn18, var_zsat_exc_dn19, var_zsat_exc_dn20,)
    }
};
        var_zsat_exc = assign83150_e124791;
        var_zsat_exc_dn5 = assign83150_e124791_d_n5;
        var_zsat_exc_dn6 = assign83150_e124791_d_n6;
        var_zsat_exc_dn7 = assign83150_e124791_d_n7;
        var_zsat_exc_dn8 = assign83150_e124791_d_n8;
        var_zsat_exc_dn12 = assign83150_e124791_d_n12;
        var_zsat_exc_dn13 = assign83150_e124791_d_n13;
        var_zsat_exc_dn14 = assign83150_e124791_d_n14;
        var_zsat_exc_dn15 = assign83150_e124791_d_n15;
        var_zsat_exc_dn16 = assign83150_e124791_d_n16;
        var_zsat_exc_dn17 = assign83150_e124791_d_n17;
        var_zsat_exc_dn18 = assign83150_e124791_d_n18;
        var_zsat_exc_dn19 = assign83150_e124791_d_n19;
        var_zsat_exc_dn20 = assign83150_e124791_d_n20;

        let assign83160_e124794: f64 = (-1.0);
        let assign83160_e124795: f64 = if var_chnl_type == assign83160_e124794 { 1.0 } else { 0.0 };
        var_guard2283 = assign83160_e124795;

        let (assign83170_e124811, assign83170_e124811_d_n5, assign83170_e124811_d_n6, assign83170_e124811_d_n7, assign83170_e124811_d_n8, assign83170_e124811_d_n12, assign83170_e124811_d_n13, assign83170_e124811_d_n14, assign83170_e124811_d_n15, assign83170_e124811_d_n16, assign83170_e124811_d_n17, assign83170_e124811_d_n18, assign83170_e124811_d_n19, assign83170_e124811_d_n20,) = {
    if ((((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) && (var_guard2282 != 0.0)) && (var_guard2283 != 0.0)) {
        let assign83170_e124807: f64 = (var_thesat1_exc * var_dps_dc);
        let assign83170_e124808: f64 = (1.0 + assign83170_e124807);
        let assign83170_e124809: f64 = (var_zsat_exc / assign83170_e124808);
        (assign83170_e124809, (((var_zsat_exc_dn5 * assign83170_e124808) - (var_zsat_exc * ((var_thesat1_exc_dn5 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn5)))) / (assign83170_e124808 * assign83170_e124808)), (((var_zsat_exc_dn6 * assign83170_e124808) - (var_zsat_exc * ((var_thesat1_exc_dn6 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn6)))) / (assign83170_e124808 * assign83170_e124808)), (((var_zsat_exc_dn7 * assign83170_e124808) - (var_zsat_exc * ((var_thesat1_exc_dn7 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn7)))) / (assign83170_e124808 * assign83170_e124808)), (((var_zsat_exc_dn8 * assign83170_e124808) - (var_zsat_exc * ((var_thesat1_exc_dn8 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn8)))) / (assign83170_e124808 * assign83170_e124808)), (((var_zsat_exc_dn12 * assign83170_e124808) - (var_zsat_exc * ((var_thesat1_exc_dn12 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn12)))) / (assign83170_e124808 * assign83170_e124808)), (((var_zsat_exc_dn13 * assign83170_e124808) - (var_zsat_exc * ((var_thesat1_exc_dn13 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn13)))) / (assign83170_e124808 * assign83170_e124808)), (((var_zsat_exc_dn14 * assign83170_e124808) - (var_zsat_exc * ((var_thesat1_exc_dn14 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn14)))) / (assign83170_e124808 * assign83170_e124808)), (((var_zsat_exc_dn15 * assign83170_e124808) - (var_zsat_exc * ((var_thesat1_exc_dn15 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn15)))) / (assign83170_e124808 * assign83170_e124808)), (((var_zsat_exc_dn16 * assign83170_e124808) - (var_zsat_exc * ((var_thesat1_exc_dn16 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn16)))) / (assign83170_e124808 * assign83170_e124808)), (((var_zsat_exc_dn17 * assign83170_e124808) - (var_zsat_exc * ((var_thesat1_exc_dn17 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn17)))) / (assign83170_e124808 * assign83170_e124808)), (((var_zsat_exc_dn18 * assign83170_e124808) - (var_zsat_exc * ((var_thesat1_exc_dn18 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn18)))) / (assign83170_e124808 * assign83170_e124808)), (((var_zsat_exc_dn19 * assign83170_e124808) - (var_zsat_exc * ((var_thesat1_exc_dn19 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn19)))) / (assign83170_e124808 * assign83170_e124808)), (((var_zsat_exc_dn20 * assign83170_e124808) - (var_zsat_exc * ((var_thesat1_exc_dn20 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn20)))) / (assign83170_e124808 * assign83170_e124808)),)
    } else {
        (var_zsat_exc, var_zsat_exc_dn5, var_zsat_exc_dn6, var_zsat_exc_dn7, var_zsat_exc_dn8, var_zsat_exc_dn12, var_zsat_exc_dn13, var_zsat_exc_dn14, var_zsat_exc_dn15, var_zsat_exc_dn16, var_zsat_exc_dn17, var_zsat_exc_dn18, var_zsat_exc_dn19, var_zsat_exc_dn20,)
    }
};
        var_zsat_exc = assign83170_e124811;
        var_zsat_exc_dn5 = assign83170_e124811_d_n5;
        var_zsat_exc_dn6 = assign83170_e124811_d_n6;
        var_zsat_exc_dn7 = assign83170_e124811_d_n7;
        var_zsat_exc_dn8 = assign83170_e124811_d_n8;
        var_zsat_exc_dn12 = assign83170_e124811_d_n12;
        var_zsat_exc_dn13 = assign83170_e124811_d_n13;
        var_zsat_exc_dn14 = assign83170_e124811_d_n14;
        var_zsat_exc_dn15 = assign83170_e124811_d_n15;
        var_zsat_exc_dn16 = assign83170_e124811_d_n16;
        var_zsat_exc_dn17 = assign83170_e124811_d_n17;
        var_zsat_exc_dn18 = assign83170_e124811_d_n18;
        var_zsat_exc_dn19 = assign83170_e124811_d_n19;
        var_zsat_exc_dn20 = assign83170_e124811_d_n20;

        let (assign83180_e124830, assign83180_e124830_d_n5, assign83180_e124830_d_n6, assign83180_e124830_d_n7, assign83180_e124830_d_n8, assign83180_e124830_d_n12, assign83180_e124830_d_n13, assign83180_e124830_d_n14, assign83180_e124830_d_n15, assign83180_e124830_d_n16, assign83180_e124830_d_n17, assign83180_e124830_d_n18, assign83180_e124830_d_n19, assign83180_e124830_d_n20,) = {
    if (((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) && (var_guard2282 != 0.0)) {
        let assign83180_e124823: f64 = (2.0 * var_zsat_exc);
        let assign83180_e124824: f64 = (1.0 + assign83180_e124823);
        let assign83180_e124825: f64 = (assign83180_e124824).sqrt();
        let assign83180_e124826: f64 = (1.0 + assign83180_e124825);
        let assign83180_e124827: f64 = (var_gmob_dc * assign83180_e124826);
        let assign83180_e124828: f64 = (0.5 * assign83180_e124827);
        (assign83180_e124828, (0.5 * ((var_gmob_dc_dn5 * assign83180_e124826) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn5) / (2.0 * assign83180_e124825))))), (0.5 * ((var_gmob_dc_dn6 * assign83180_e124826) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn6) / (2.0 * assign83180_e124825))))), (0.5 * ((var_gmob_dc_dn7 * assign83180_e124826) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn7) / (2.0 * assign83180_e124825))))), (0.5 * ((var_gmob_dc_dn8 * assign83180_e124826) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn8) / (2.0 * assign83180_e124825))))), (0.5 * ((var_gmob_dc_dn12 * assign83180_e124826) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn12) / (2.0 * assign83180_e124825))))), (0.5 * ((var_gmob_dc_dn13 * assign83180_e124826) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn13) / (2.0 * assign83180_e124825))))), (0.5 * ((var_gmob_dc_dn14 * assign83180_e124826) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn14) / (2.0 * assign83180_e124825))))), (0.5 * ((var_gmob_dc_dn15 * assign83180_e124826) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn15) / (2.0 * assign83180_e124825))))), (0.5 * ((var_gmob_dc_dn16 * assign83180_e124826) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn16) / (2.0 * assign83180_e124825))))), (0.5 * ((var_gmob_dc_dn17 * assign83180_e124826) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn17) / (2.0 * assign83180_e124825))))), (0.5 * ((var_gmob_dc_dn18 * assign83180_e124826) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn18) / (2.0 * assign83180_e124825))))), (0.5 * ((var_gmob_dc_dn19 * assign83180_e124826) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn19) / (2.0 * assign83180_e124825))))), (0.5 * ((var_gmob_dc_dn20 * assign83180_e124826) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn20) / (2.0 * assign83180_e124825))))),)
    } else {
        (var_gvsat_exc, var_gvsat_exc_dn5, var_gvsat_exc_dn6, var_gvsat_exc_dn7, var_gvsat_exc_dn8, var_gvsat_exc_dn12, var_gvsat_exc_dn13, var_gvsat_exc_dn14, var_gvsat_exc_dn15, var_gvsat_exc_dn16, var_gvsat_exc_dn17, var_gvsat_exc_dn18, var_gvsat_exc_dn19, var_gvsat_exc_dn20,)
    }
};
        var_gvsat_exc = assign83180_e124830;
        var_gvsat_exc_dn5 = assign83180_e124830_d_n5;
        var_gvsat_exc_dn6 = assign83180_e124830_d_n6;
        var_gvsat_exc_dn7 = assign83180_e124830_d_n7;
        var_gvsat_exc_dn8 = assign83180_e124830_d_n8;
        var_gvsat_exc_dn12 = assign83180_e124830_d_n12;
        var_gvsat_exc_dn13 = assign83180_e124830_d_n13;
        var_gvsat_exc_dn14 = assign83180_e124830_d_n14;
        var_gvsat_exc_dn15 = assign83180_e124830_d_n15;
        var_gvsat_exc_dn16 = assign83180_e124830_d_n16;
        var_gvsat_exc_dn17 = assign83180_e124830_d_n17;
        var_gvsat_exc_dn18 = assign83180_e124830_d_n18;
        var_gvsat_exc_dn19 = assign83180_e124830_d_n19;
        var_gvsat_exc_dn20 = assign83180_e124830_d_n20;

        let (assign83190_e124842, assign83190_e124842_d_n5, assign83190_e124842_d_n6, assign83190_e124842_d_n7, assign83190_e124842_d_n8, assign83190_e124842_d_n12, assign83190_e124842_d_n13, assign83190_e124842_d_n14, assign83190_e124842_d_n15, assign83190_e124842_d_n16, assign83190_e124842_d_n17, assign83190_e124842_d_n18, assign83190_e124842_d_n19, assign83190_e124842_d_n20,) = {
    if (((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) && (var_guard2282 != 0.0)) {
        let assign83190_e124839: f64 = (var_gvsat_exc * var_lc);
        let assign83190_e124840: f64 = (var_gmob_dc / assign83190_e124839);
        (assign83190_e124840, (((var_gmob_dc_dn5 * assign83190_e124839) - (var_gmob_dc * ((var_gvsat_exc_dn5 * var_lc) + (var_gvsat_exc * var_lc_dn5)))) / (assign83190_e124839 * assign83190_e124839)), (((var_gmob_dc_dn6 * assign83190_e124839) - (var_gmob_dc * ((var_gvsat_exc_dn6 * var_lc) + (var_gvsat_exc * var_lc_dn6)))) / (assign83190_e124839 * assign83190_e124839)), (((var_gmob_dc_dn7 * assign83190_e124839) - (var_gmob_dc * ((var_gvsat_exc_dn7 * var_lc) + (var_gvsat_exc * var_lc_dn7)))) / (assign83190_e124839 * assign83190_e124839)), (((var_gmob_dc_dn8 * assign83190_e124839) - (var_gmob_dc * ((var_gvsat_exc_dn8 * var_lc) + (var_gvsat_exc * var_lc_dn8)))) / (assign83190_e124839 * assign83190_e124839)), (((var_gmob_dc_dn12 * assign83190_e124839) - (var_gmob_dc * ((var_gvsat_exc_dn12 * var_lc) + (var_gvsat_exc * var_lc_dn12)))) / (assign83190_e124839 * assign83190_e124839)), (((var_gmob_dc_dn13 * assign83190_e124839) - (var_gmob_dc * ((var_gvsat_exc_dn13 * var_lc) + (var_gvsat_exc * var_lc_dn13)))) / (assign83190_e124839 * assign83190_e124839)), (((var_gmob_dc_dn14 * assign83190_e124839) - (var_gmob_dc * ((var_gvsat_exc_dn14 * var_lc) + (var_gvsat_exc * var_lc_dn14)))) / (assign83190_e124839 * assign83190_e124839)), (((var_gmob_dc_dn15 * assign83190_e124839) - (var_gmob_dc * ((var_gvsat_exc_dn15 * var_lc) + (var_gvsat_exc * var_lc_dn15)))) / (assign83190_e124839 * assign83190_e124839)), (((var_gmob_dc_dn16 * assign83190_e124839) - (var_gmob_dc * ((var_gvsat_exc_dn16 * var_lc) + (var_gvsat_exc * var_lc_dn16)))) / (assign83190_e124839 * assign83190_e124839)), (((var_gmob_dc_dn17 * assign83190_e124839) - (var_gmob_dc * ((var_gvsat_exc_dn17 * var_lc) + (var_gvsat_exc * var_lc_dn17)))) / (assign83190_e124839 * assign83190_e124839)), (((var_gmob_dc_dn18 * assign83190_e124839) - (var_gmob_dc * ((var_gvsat_exc_dn18 * var_lc) + (var_gvsat_exc * var_lc_dn18)))) / (assign83190_e124839 * assign83190_e124839)), (((var_gmob_dc_dn19 * assign83190_e124839) - (var_gmob_dc * ((var_gvsat_exc_dn19 * var_lc) + (var_gvsat_exc * var_lc_dn19)))) / (assign83190_e124839 * assign83190_e124839)), (((var_gmob_dc_dn20 * assign83190_e124839) - (var_gmob_dc * ((var_gvsat_exc_dn20 * var_lc) + (var_gvsat_exc * var_lc_dn20)))) / (assign83190_e124839 * assign83190_e124839)),)
    } else {
        (var_gfac, var_gfac_dn5, var_gfac_dn6, var_gfac_dn7, var_gfac_dn8, var_gfac_dn12, var_gfac_dn13, var_gfac_dn14, var_gfac_dn15, var_gfac_dn16, var_gfac_dn17, var_gfac_dn18, var_gfac_dn19, var_gfac_dn20,)
    }
};
        var_gfac = assign83190_e124842;
        var_gfac_dn5 = assign83190_e124842_d_n5;
        var_gfac_dn6 = assign83190_e124842_d_n6;
        var_gfac_dn7 = assign83190_e124842_d_n7;
        var_gfac_dn8 = assign83190_e124842_d_n8;
        var_gfac_dn12 = assign83190_e124842_d_n12;
        var_gfac_dn13 = assign83190_e124842_d_n13;
        var_gfac_dn14 = assign83190_e124842_d_n14;
        var_gfac_dn15 = assign83190_e124842_d_n15;
        var_gfac_dn16 = assign83190_e124842_d_n16;
        var_gfac_dn17 = assign83190_e124842_d_n17;
        var_gfac_dn18 = assign83190_e124842_d_n18;
        var_gfac_dn19 = assign83190_e124842_d_n19;
        var_gfac_dn20 = assign83190_e124842_d_n20;

        let (assign83200_e124858, assign83200_e124858_d_n5, assign83200_e124858_d_n6, assign83200_e124858_d_n7, assign83200_e124858_d_n8, assign83200_e124858_d_n12, assign83200_e124858_d_n13, assign83200_e124858_d_n14, assign83200_e124858_d_n15, assign83200_e124858_d_n16, assign83200_e124858_d_n17, assign83200_e124858_d_n18, assign83200_e124858_d_n19, assign83200_e124858_d_n20,) = {
    if (((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) && (var_guard2282 != 0.0)) {
        let assign83200_e124850: f64 = (var_fac_exc * var_i_ds);
        let assign83200_e124852: f64 = (assign83200_e124850 * var_vdse_dc);
        let assign83200_e124854: f64 = (assign83200_e124852 * var_gfac);
        let assign83200_e124856: f64 = (assign83200_e124854 * var_gfac);
        (assign83200_e124856, (((((((var_fac_exc * var_i_ds_dn5) * var_vdse_dc) + (assign83200_e124850 * var_vdse_dc_dn5)) * var_gfac) + (assign83200_e124852 * var_gfac_dn5)) * var_gfac) + (assign83200_e124854 * var_gfac_dn5)), (((((((var_fac_exc * var_i_ds_dn6) * var_vdse_dc) + (assign83200_e124850 * var_vdse_dc_dn6)) * var_gfac) + (assign83200_e124852 * var_gfac_dn6)) * var_gfac) + (assign83200_e124854 * var_gfac_dn6)), (((((((var_fac_exc * var_i_ds_dn7) * var_vdse_dc) + (assign83200_e124850 * var_vdse_dc_dn7)) * var_gfac) + (assign83200_e124852 * var_gfac_dn7)) * var_gfac) + (assign83200_e124854 * var_gfac_dn7)), (((((((var_fac_exc * var_i_ds_dn8) * var_vdse_dc) + (assign83200_e124850 * var_vdse_dc_dn8)) * var_gfac) + (assign83200_e124852 * var_gfac_dn8)) * var_gfac) + (assign83200_e124854 * var_gfac_dn8)), (((((((var_fac_exc * var_i_ds_dn12) * var_vdse_dc) + (assign83200_e124850 * var_vdse_dc_dn12)) * var_gfac) + (assign83200_e124852 * var_gfac_dn12)) * var_gfac) + (assign83200_e124854 * var_gfac_dn12)), (((((((var_fac_exc * var_i_ds_dn13) * var_vdse_dc) + (assign83200_e124850 * var_vdse_dc_dn13)) * var_gfac) + (assign83200_e124852 * var_gfac_dn13)) * var_gfac) + (assign83200_e124854 * var_gfac_dn13)), (((((((var_fac_exc * var_i_ds_dn14) * var_vdse_dc) + (assign83200_e124850 * var_vdse_dc_dn14)) * var_gfac) + (assign83200_e124852 * var_gfac_dn14)) * var_gfac) + (assign83200_e124854 * var_gfac_dn14)), (((((((var_fac_exc * var_i_ds_dn15) * var_vdse_dc) + (assign83200_e124850 * var_vdse_dc_dn15)) * var_gfac) + (assign83200_e124852 * var_gfac_dn15)) * var_gfac) + (assign83200_e124854 * var_gfac_dn15)), (((((((var_fac_exc * var_i_ds_dn16) * var_vdse_dc) + (assign83200_e124850 * var_vdse_dc_dn16)) * var_gfac) + (assign83200_e124852 * var_gfac_dn16)) * var_gfac) + (assign83200_e124854 * var_gfac_dn16)), (((((((var_fac_exc * var_i_ds_dn17) * var_vdse_dc) + (assign83200_e124850 * var_vdse_dc_dn17)) * var_gfac) + (assign83200_e124852 * var_gfac_dn17)) * var_gfac) + (assign83200_e124854 * var_gfac_dn17)), (((((((var_fac_exc * var_i_ds_dn18) * var_vdse_dc) + (assign83200_e124850 * var_vdse_dc_dn18)) * var_gfac) + (assign83200_e124852 * var_gfac_dn18)) * var_gfac) + (assign83200_e124854 * var_gfac_dn18)), (((((((var_fac_exc * var_i_ds_dn19) * var_vdse_dc) + (assign83200_e124850 * var_vdse_dc_dn19)) * var_gfac) + (assign83200_e124852 * var_gfac_dn19)) * var_gfac) + (assign83200_e124854 * var_gfac_dn19)), (((((((var_fac_exc * var_i_ds_dn20) * var_vdse_dc) + (assign83200_e124850 * var_vdse_dc_dn20)) * var_gfac) + (assign83200_e124852 * var_gfac_dn20)) * var_gfac) + (assign83200_e124854 * var_gfac_dn20)),)
    } else {
        (var_sidexc, var_sidexc_dn5, var_sidexc_dn6, var_sidexc_dn7, var_sidexc_dn8, var_sidexc_dn12, var_sidexc_dn13, var_sidexc_dn14, var_sidexc_dn15, var_sidexc_dn16, var_sidexc_dn17, var_sidexc_dn18, var_sidexc_dn19, var_sidexc_dn20,)
    }
};
        var_sidexc = assign83200_e124858;
        var_sidexc_dn5 = assign83200_e124858_d_n5;
        var_sidexc_dn6 = assign83200_e124858_d_n6;
        var_sidexc_dn7 = assign83200_e124858_d_n7;
        var_sidexc_dn8 = assign83200_e124858_d_n8;
        var_sidexc_dn12 = assign83200_e124858_d_n12;
        var_sidexc_dn13 = assign83200_e124858_d_n13;
        var_sidexc_dn14 = assign83200_e124858_d_n14;
        var_sidexc_dn15 = assign83200_e124858_d_n15;
        var_sidexc_dn16 = assign83200_e124858_d_n16;
        var_sidexc_dn17 = assign83200_e124858_d_n17;
        var_sidexc_dn18 = assign83200_e124858_d_n18;
        var_sidexc_dn19 = assign83200_e124858_d_n19;
        var_sidexc_dn20 = assign83200_e124858_d_n20;

        let (assign83210_e124870, assign83210_e124870_d_n5, assign83210_e124870_d_n6, assign83210_e124870_d_n7, assign83210_e124870_d_n8, assign83210_e124870_d_n12, assign83210_e124870_d_n13, assign83210_e124870_d_n14, assign83210_e124870_d_n15, assign83210_e124870_d_n16, assign83210_e124870_d_n17, assign83210_e124870_d_n18, assign83210_e124870_d_n19, assign83210_e124870_d_n20,) = {
    if (((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) && (var_guard2282 != 0.0)) {
        let assign83210_e124867: f64 = (var_sidexc / var_nt0);
        let assign83210_e124868: f64 = (var_mid + assign83210_e124867);
        (assign83210_e124868, (var_mid_dn5 + (var_sidexc_dn5 / var_nt0)), (var_mid_dn6 + (var_sidexc_dn6 / var_nt0)), (var_mid_dn7 + (var_sidexc_dn7 / var_nt0)), (var_mid_dn8 + (var_sidexc_dn8 / var_nt0)), (var_mid_dn12 + (var_sidexc_dn12 / var_nt0)), (var_mid_dn13 + (var_sidexc_dn13 / var_nt0)), (var_mid_dn14 + (var_sidexc_dn14 / var_nt0)), (var_mid_dn15 + (var_sidexc_dn15 / var_nt0)), (var_mid_dn16 + (var_sidexc_dn16 / var_nt0)), (var_mid_dn17 + (var_sidexc_dn17 / var_nt0)), (var_mid_dn18 + (var_sidexc_dn18 / var_nt0)), (var_mid_dn19 + (var_sidexc_dn19 / var_nt0)), (var_mid_dn20 + (var_sidexc_dn20 / var_nt0)),)
    } else {
        (var_mid, var_mid_dn5, var_mid_dn6, var_mid_dn7, var_mid_dn8, var_mid_dn12, var_mid_dn13, var_mid_dn14, var_mid_dn15, var_mid_dn16, var_mid_dn17, var_mid_dn18, var_mid_dn19, var_mid_dn20,)
    }
};
        var_mid = assign83210_e124870;
        var_mid_dn5 = assign83210_e124870_d_n5;
        var_mid_dn6 = assign83210_e124870_d_n6;
        var_mid_dn7 = assign83210_e124870_d_n7;
        var_mid_dn8 = assign83210_e124870_d_n8;
        var_mid_dn12 = assign83210_e124870_d_n12;
        var_mid_dn13 = assign83210_e124870_d_n13;
        var_mid_dn14 = assign83210_e124870_d_n14;
        var_mid_dn15 = assign83210_e124870_d_n15;
        var_mid_dn16 = assign83210_e124870_d_n16;
        var_mid_dn17 = assign83210_e124870_d_n17;
        var_mid_dn18 = assign83210_e124870_d_n18;
        var_mid_dn19 = assign83210_e124870_d_n19;
        var_mid_dn20 = assign83210_e124870_d_n20;

        let (assign83220_e124879, assign83220_e124879_d_n5, assign83220_e124879_d_n6, assign83220_e124879_d_n7, assign83220_e124879_d_n8, assign83220_e124879_d_n12, assign83220_e124879_d_n13, assign83220_e124879_d_n14, assign83220_e124879_d_n15, assign83220_e124879_d_n16, assign83220_e124879_d_n17, assign83220_e124879_d_n18, assign83220_e124879_d_n19, assign83220_e124879_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2281 != 0.0)) {
        let assign83220_e124876: f64 = (var_nt * var_mid);
        let assign83220_e124877: f64 = (assign83220_e124876).sqrt();
        (assign83220_e124877, ((var_nt * var_mid_dn5) / (2.0 * assign83220_e124877)), ((var_nt * var_mid_dn6) / (2.0 * assign83220_e124877)), ((var_nt * var_mid_dn7) / (2.0 * assign83220_e124877)), ((var_nt * var_mid_dn8) / (2.0 * assign83220_e124877)), ((var_nt * var_mid_dn12) / (2.0 * assign83220_e124877)), ((var_nt * var_mid_dn13) / (2.0 * assign83220_e124877)), ((var_nt * var_mid_dn14) / (2.0 * assign83220_e124877)), ((var_nt * var_mid_dn15) / (2.0 * assign83220_e124877)), ((var_nt * var_mid_dn16) / (2.0 * assign83220_e124877)), ((var_nt * var_mid_dn17) / (2.0 * assign83220_e124877)), ((var_nt * var_mid_dn18) / (2.0 * assign83220_e124877)), ((var_nt * var_mid_dn19) / (2.0 * assign83220_e124877)), ((var_nt * var_mid_dn20) / (2.0 * assign83220_e124877)),)
    } else {
        (var_sqid, var_sqid_dn5, var_sqid_dn6, var_sqid_dn7, var_sqid_dn8, var_sqid_dn12, var_sqid_dn13, var_sqid_dn14, var_sqid_dn15, var_sqid_dn16, var_sqid_dn17, var_sqid_dn18, var_sqid_dn19, var_sqid_dn20,)
    }
};
        var_sqid = assign83220_e124879;
        var_sqid_dn5 = assign83220_e124879_d_n5;
        var_sqid_dn6 = assign83220_e124879_d_n6;
        var_sqid_dn7 = assign83220_e124879_d_n7;
        var_sqid_dn8 = assign83220_e124879_d_n8;
        var_sqid_dn12 = assign83220_e124879_d_n12;
        var_sqid_dn13 = assign83220_e124879_d_n13;
        var_sqid_dn14 = assign83220_e124879_d_n14;
        var_sqid_dn15 = assign83220_e124879_d_n15;
        var_sqid_dn16 = assign83220_e124879_d_n16;
        var_sqid_dn17 = assign83220_e124879_d_n17;
        var_sqid_dn18 = assign83220_e124879_d_n18;
        var_sqid_dn19 = assign83220_e124879_d_n19;
        var_sqid_dn20 = assign83220_e124879_d_n20;

        let assign83230_e124894: f64 = if ((((p.p50 == 1.0) && (var_nt > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        var_guard2284 = assign83230_e124894;

        let (assign83240_e124926, assign83240_e124926_d_n5, assign83240_e124926_d_n6, assign83240_e124926_d_n7, assign83240_e124926_d_n8, assign83240_e124926_d_n12, assign83240_e124926_d_n13, assign83240_e124926_d_n14, assign83240_e124926_d_n15, assign83240_e124926_d_n16, assign83240_e124926_d_n17, assign83240_e124926_d_n18, assign83240_e124926_d_n19, assign83240_e124926_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2284 != 0.0)) {
        let assign83240_e124900: f64 = (var_t1 / 12.0);
        let assign83240_e124904: f64 = (var_t1 + 0.2);
        let assign83240_e124907: f64 = (12.0 * var_t2);
        let assign83240_e124908: f64 = (assign83240_e124904 - assign83240_e124907);
        let assign83240_e124909: f64 = (var_t2 * assign83240_e124908);
        let assign83240_e124910: f64 = (assign83240_e124900 - assign83240_e124909);
        let assign83240_e124915: f64 = (var_t1 + 1.0);
        let assign83240_e124918: f64 = (12.0 * var_t2);
        let assign83240_e124919: f64 = (assign83240_e124915 - assign83240_e124918);
        let assign83240_e124920: f64 = (var_t2 * assign83240_e124919);
        let assign83240_e124922: f64 = (assign83240_e124920 * var_r);
        let assign83240_e124923: f64 = (1.6 * assign83240_e124922);
        let assign83240_e124924: f64 = (assign83240_e124910 - assign83240_e124923);
        (assign83240_e124924, (((var_t1_dn5 / 12.0) - ((var_t2_dn5 * assign83240_e124908) + (var_t2 * (var_t1_dn5 - (12.0 * var_t2_dn5))))) - (1.6 * ((((var_t2_dn5 * assign83240_e124919) + (var_t2 * (var_t1_dn5 - (12.0 * var_t2_dn5)))) * var_r) + (assign83240_e124920 * var_r_dn5)))), (((var_t1_dn6 / 12.0) - ((var_t2_dn6 * assign83240_e124908) + (var_t2 * (var_t1_dn6 - (12.0 * var_t2_dn6))))) - (1.6 * ((((var_t2_dn6 * assign83240_e124919) + (var_t2 * (var_t1_dn6 - (12.0 * var_t2_dn6)))) * var_r) + (assign83240_e124920 * var_r_dn6)))), (((var_t1_dn7 / 12.0) - ((var_t2_dn7 * assign83240_e124908) + (var_t2 * (var_t1_dn7 - (12.0 * var_t2_dn7))))) - (1.6 * ((((var_t2_dn7 * assign83240_e124919) + (var_t2 * (var_t1_dn7 - (12.0 * var_t2_dn7)))) * var_r) + (assign83240_e124920 * var_r_dn7)))), (((var_t1_dn8 / 12.0) - ((var_t2_dn8 * assign83240_e124908) + (var_t2 * (var_t1_dn8 - (12.0 * var_t2_dn8))))) - (1.6 * ((((var_t2_dn8 * assign83240_e124919) + (var_t2 * (var_t1_dn8 - (12.0 * var_t2_dn8)))) * var_r) + (assign83240_e124920 * var_r_dn8)))), (((var_t1_dn12 / 12.0) - ((var_t2_dn12 * assign83240_e124908) + (var_t2 * (var_t1_dn12 - (12.0 * var_t2_dn12))))) - (1.6 * ((((var_t2_dn12 * assign83240_e124919) + (var_t2 * (var_t1_dn12 - (12.0 * var_t2_dn12)))) * var_r) + (assign83240_e124920 * var_r_dn12)))), (((var_t1_dn13 / 12.0) - ((var_t2_dn13 * assign83240_e124908) + (var_t2 * (var_t1_dn13 - (12.0 * var_t2_dn13))))) - (1.6 * ((((var_t2_dn13 * assign83240_e124919) + (var_t2 * (var_t1_dn13 - (12.0 * var_t2_dn13)))) * var_r) + (assign83240_e124920 * var_r_dn13)))), (((var_t1_dn14 / 12.0) - ((var_t2_dn14 * assign83240_e124908) + (var_t2 * (var_t1_dn14 - (12.0 * var_t2_dn14))))) - (1.6 * ((((var_t2_dn14 * assign83240_e124919) + (var_t2 * (var_t1_dn14 - (12.0 * var_t2_dn14)))) * var_r) + (assign83240_e124920 * var_r_dn14)))), (((var_t1_dn15 / 12.0) - ((var_t2_dn15 * assign83240_e124908) + (var_t2 * (var_t1_dn15 - (12.0 * var_t2_dn15))))) - (1.6 * ((((var_t2_dn15 * assign83240_e124919) + (var_t2 * (var_t1_dn15 - (12.0 * var_t2_dn15)))) * var_r) + (assign83240_e124920 * var_r_dn15)))), (((var_t1_dn16 / 12.0) - ((var_t2_dn16 * assign83240_e124908) + (var_t2 * (var_t1_dn16 - (12.0 * var_t2_dn16))))) - (1.6 * ((((var_t2_dn16 * assign83240_e124919) + (var_t2 * (var_t1_dn16 - (12.0 * var_t2_dn16)))) * var_r) + (assign83240_e124920 * var_r_dn16)))), (((var_t1_dn17 / 12.0) - ((var_t2_dn17 * assign83240_e124908) + (var_t2 * (var_t1_dn17 - (12.0 * var_t2_dn17))))) - (1.6 * ((((var_t2_dn17 * assign83240_e124919) + (var_t2 * (var_t1_dn17 - (12.0 * var_t2_dn17)))) * var_r) + (assign83240_e124920 * var_r_dn17)))), (((var_t1_dn18 / 12.0) - ((var_t2_dn18 * assign83240_e124908) + (var_t2 * (var_t1_dn18 - (12.0 * var_t2_dn18))))) - (1.6 * ((((var_t2_dn18 * assign83240_e124919) + (var_t2 * (var_t1_dn18 - (12.0 * var_t2_dn18)))) * var_r) + (assign83240_e124920 * var_r_dn18)))), (((var_t1_dn19 / 12.0) - ((var_t2_dn19 * assign83240_e124908) + (var_t2 * (var_t1_dn19 - (12.0 * var_t2_dn19))))) - (1.6 * ((((var_t2_dn19 * assign83240_e124919) + (var_t2 * (var_t1_dn19 - (12.0 * var_t2_dn19)))) * var_r) + (assign83240_e124920 * var_r_dn19)))), (((var_t1_dn20 / 12.0) - ((var_t2_dn20 * assign83240_e124908) + (var_t2 * (var_t1_dn20 - (12.0 * var_t2_dn20))))) - (1.6 * ((((var_t2_dn20 * assign83240_e124919) + (var_t2 * (var_t1_dn20 - (12.0 * var_t2_dn20)))) * var_r) + (assign83240_e124920 * var_r_dn20)))),)
    } else {
        (var_mig, var_mig_dn5, var_mig_dn6, var_mig_dn7, var_mig_dn8, var_mig_dn12, var_mig_dn13, var_mig_dn14, var_mig_dn15, var_mig_dn16, var_mig_dn17, var_mig_dn18, var_mig_dn19, var_mig_dn20,)
    }
};
        var_mig = assign83240_e124926;
        var_mig_dn5 = assign83240_e124926_d_n5;
        var_mig_dn6 = assign83240_e124926_d_n6;
        var_mig_dn7 = assign83240_e124926_d_n7;
        var_mig_dn8 = assign83240_e124926_d_n8;
        var_mig_dn12 = assign83240_e124926_d_n12;
        var_mig_dn13 = assign83240_e124926_d_n13;
        var_mig_dn14 = assign83240_e124926_d_n14;
        var_mig_dn15 = assign83240_e124926_d_n15;
        var_mig_dn16 = assign83240_e124926_d_n16;
        var_mig_dn17 = assign83240_e124926_d_n17;
        var_mig_dn18 = assign83240_e124926_d_n18;
        var_mig_dn19 = assign83240_e124926_d_n19;
        var_mig_dn20 = assign83240_e124926_d_n20;

        let (assign83250_e124937, assign83250_e124937_d_n5, assign83250_e124937_d_n6, assign83250_e124937_d_n7, assign83250_e124937_d_n8, assign83250_e124937_d_n12, assign83250_e124937_d_n13, assign83250_e124937_d_n14, assign83250_e124937_d_n15, assign83250_e124937_d_n16, assign83250_e124937_d_n17, assign83250_e124937_d_n18, assign83250_e124937_d_n19, assign83250_e124937_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2284 != 0.0)) {
        let (assign83250_e124935, assign83250_e124935_d_n5, assign83250_e124935_d_n6, assign83250_e124935_d_n7, assign83250_e124935_d_n8, assign83250_e124935_d_n12, assign83250_e124935_d_n13, assign83250_e124935_d_n14, assign83250_e124935_d_n15, assign83250_e124935_d_n16, assign83250_e124935_d_n17, assign83250_e124935_d_n18, assign83250_e124935_d_n19, assign83250_e124935_d_n20,) = {
            if (var_mig > 1e-40) {
                (var_mig, var_mig_dn5, var_mig_dn6, var_mig_dn7, var_mig_dn8, var_mig_dn12, var_mig_dn13, var_mig_dn14, var_mig_dn15, var_mig_dn16, var_mig_dn17, var_mig_dn18, var_mig_dn19, var_mig_dn20,)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign83250_e124935, assign83250_e124935_d_n5, assign83250_e124935_d_n6, assign83250_e124935_d_n7, assign83250_e124935_d_n8, assign83250_e124935_d_n12, assign83250_e124935_d_n13, assign83250_e124935_d_n14, assign83250_e124935_d_n15, assign83250_e124935_d_n16, assign83250_e124935_d_n17, assign83250_e124935_d_n18, assign83250_e124935_d_n19, assign83250_e124935_d_n20,)
    } else {
        (var_mig, var_mig_dn5, var_mig_dn6, var_mig_dn7, var_mig_dn8, var_mig_dn12, var_mig_dn13, var_mig_dn14, var_mig_dn15, var_mig_dn16, var_mig_dn17, var_mig_dn18, var_mig_dn19, var_mig_dn20,)
    }
};
        var_mig = assign83250_e124937;
        var_mig_dn5 = assign83250_e124937_d_n5;
        var_mig_dn6 = assign83250_e124937_d_n6;
        var_mig_dn7 = assign83250_e124937_d_n7;
        var_mig_dn8 = assign83250_e124937_d_n8;
        var_mig_dn12 = assign83250_e124937_d_n12;
        var_mig_dn13 = assign83250_e124937_d_n13;
        var_mig_dn14 = assign83250_e124937_d_n14;
        var_mig_dn15 = assign83250_e124937_d_n15;
        var_mig_dn16 = assign83250_e124937_d_n16;
        var_mig_dn17 = assign83250_e124937_d_n17;
        var_mig_dn18 = assign83250_e124937_d_n18;
        var_mig_dn19 = assign83250_e124937_d_n19;
        var_mig_dn20 = assign83250_e124937_d_n20;

        let (assign83260_e124947, assign83260_e124947_d_n5, assign83260_e124947_d_n6, assign83260_e124947_d_n7, assign83260_e124947_d_n8, assign83260_e124947_d_n12, assign83260_e124947_d_n13, assign83260_e124947_d_n14, assign83260_e124947_d_n15, assign83260_e124947_d_n16, assign83260_e124947_d_n17, assign83260_e124947_d_n18, assign83260_e124947_d_n19, assign83260_e124947_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2284 != 0.0)) {
        let assign83260_e124943: f64 = (var_lcinv2 / var_g_ideal);
        let assign83260_e124945: f64 = (assign83260_e124943 * var_mig);
        (assign83260_e124945, (((((var_lcinv2_dn5 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn5)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign83260_e124943 * var_mig_dn5)), (((((var_lcinv2_dn6 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn6)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign83260_e124943 * var_mig_dn6)), (((((var_lcinv2_dn7 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn7)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign83260_e124943 * var_mig_dn7)), (((((var_lcinv2_dn8 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn8)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign83260_e124943 * var_mig_dn8)), (((((var_lcinv2_dn12 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn12)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign83260_e124943 * var_mig_dn12)), (((((var_lcinv2_dn13 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn13)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign83260_e124943 * var_mig_dn13)), (((((var_lcinv2_dn14 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn14)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign83260_e124943 * var_mig_dn14)), (((((var_lcinv2_dn15 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn15)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign83260_e124943 * var_mig_dn15)), (((((var_lcinv2_dn16 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn16)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign83260_e124943 * var_mig_dn16)), (((((var_lcinv2_dn17 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn17)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign83260_e124943 * var_mig_dn17)), (((((var_lcinv2_dn18 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn18)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign83260_e124943 * var_mig_dn18)), (((((var_lcinv2_dn19 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn19)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign83260_e124943 * var_mig_dn19)), (((((var_lcinv2_dn20 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn20)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign83260_e124943 * var_mig_dn20)),)
    } else {
        (var_mig, var_mig_dn5, var_mig_dn6, var_mig_dn7, var_mig_dn8, var_mig_dn12, var_mig_dn13, var_mig_dn14, var_mig_dn15, var_mig_dn16, var_mig_dn17, var_mig_dn18, var_mig_dn19, var_mig_dn20,)
    }
};
        var_mig = assign83260_e124947;
        var_mig_dn5 = assign83260_e124947_d_n5;
        var_mig_dn6 = assign83260_e124947_d_n6;
        var_mig_dn7 = assign83260_e124947_d_n7;
        var_mig_dn8 = assign83260_e124947_d_n8;
        var_mig_dn12 = assign83260_e124947_d_n12;
        var_mig_dn13 = assign83260_e124947_d_n13;
        var_mig_dn14 = assign83260_e124947_d_n14;
        var_mig_dn15 = assign83260_e124947_d_n15;
        var_mig_dn16 = assign83260_e124947_d_n16;
        var_mig_dn17 = assign83260_e124947_d_n17;
        var_mig_dn18 = assign83260_e124947_d_n18;
        var_mig_dn19 = assign83260_e124947_d_n19;
        var_mig_dn20 = assign83260_e124947_d_n20;

        let (assign83270_e124975, assign83270_e124975_d_n5, assign83270_e124975_d_n6, assign83270_e124975_d_n7, assign83270_e124975_d_n8, assign83270_e124975_d_n12, assign83270_e124975_d_n13, assign83270_e124975_d_n14, assign83270_e124975_d_n15, assign83270_e124975_d_n16, assign83270_e124975_d_n17, assign83270_e124975_d_n18, assign83270_e124975_d_n19, assign83270_e124975_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2284 != 0.0)) {
        let assign83270_e124953: f64 = (var_lcinv2 * var_sqt2);
        let assign83270_e124957: f64 = (12.0 * var_t2);
        let assign83270_e124958: f64 = (1.0 - assign83270_e124957);
        let assign83270_e124962: f64 = (19.2 * var_t2);
        let assign83270_e124963: f64 = (var_t1 + assign83270_e124962);
        let assign83270_e124967: f64 = (var_t1 * var_t2);
        let assign83270_e124968: f64 = (12.0 * assign83270_e124967);
        let assign83270_e124969: f64 = (assign83270_e124963 - assign83270_e124968);
        let assign83270_e124971: f64 = (assign83270_e124969 * var_r);
        let assign83270_e124972: f64 = (assign83270_e124958 - assign83270_e124971);
        let assign83270_e124973: f64 = (assign83270_e124953 * assign83270_e124972);
        (assign83270_e124973, ((((var_lcinv2_dn5 * var_sqt2) + (var_lcinv2 * var_sqt2_dn5)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * var_t2_dn5)) - ((((var_t1_dn5 + (19.2 * var_t2_dn5)) - (12.0 * ((var_t1_dn5 * var_t2) + (var_t1 * var_t2_dn5)))) * var_r) + (assign83270_e124969 * var_r_dn5))))), ((((var_lcinv2_dn6 * var_sqt2) + (var_lcinv2 * var_sqt2_dn6)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * var_t2_dn6)) - ((((var_t1_dn6 + (19.2 * var_t2_dn6)) - (12.0 * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6)))) * var_r) + (assign83270_e124969 * var_r_dn6))))), ((((var_lcinv2_dn7 * var_sqt2) + (var_lcinv2 * var_sqt2_dn7)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * var_t2_dn7)) - ((((var_t1_dn7 + (19.2 * var_t2_dn7)) - (12.0 * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7)))) * var_r) + (assign83270_e124969 * var_r_dn7))))), ((((var_lcinv2_dn8 * var_sqt2) + (var_lcinv2 * var_sqt2_dn8)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * var_t2_dn8)) - ((((var_t1_dn8 + (19.2 * var_t2_dn8)) - (12.0 * ((var_t1_dn8 * var_t2) + (var_t1 * var_t2_dn8)))) * var_r) + (assign83270_e124969 * var_r_dn8))))), ((((var_lcinv2_dn12 * var_sqt2) + (var_lcinv2 * var_sqt2_dn12)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * var_t2_dn12)) - ((((var_t1_dn12 + (19.2 * var_t2_dn12)) - (12.0 * ((var_t1_dn12 * var_t2) + (var_t1 * var_t2_dn12)))) * var_r) + (assign83270_e124969 * var_r_dn12))))), ((((var_lcinv2_dn13 * var_sqt2) + (var_lcinv2 * var_sqt2_dn13)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * var_t2_dn13)) - ((((var_t1_dn13 + (19.2 * var_t2_dn13)) - (12.0 * ((var_t1_dn13 * var_t2) + (var_t1 * var_t2_dn13)))) * var_r) + (assign83270_e124969 * var_r_dn13))))), ((((var_lcinv2_dn14 * var_sqt2) + (var_lcinv2 * var_sqt2_dn14)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * var_t2_dn14)) - ((((var_t1_dn14 + (19.2 * var_t2_dn14)) - (12.0 * ((var_t1_dn14 * var_t2) + (var_t1 * var_t2_dn14)))) * var_r) + (assign83270_e124969 * var_r_dn14))))), ((((var_lcinv2_dn15 * var_sqt2) + (var_lcinv2 * var_sqt2_dn15)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * var_t2_dn15)) - ((((var_t1_dn15 + (19.2 * var_t2_dn15)) - (12.0 * ((var_t1_dn15 * var_t2) + (var_t1 * var_t2_dn15)))) * var_r) + (assign83270_e124969 * var_r_dn15))))), ((((var_lcinv2_dn16 * var_sqt2) + (var_lcinv2 * var_sqt2_dn16)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * var_t2_dn16)) - ((((var_t1_dn16 + (19.2 * var_t2_dn16)) - (12.0 * ((var_t1_dn16 * var_t2) + (var_t1 * var_t2_dn16)))) * var_r) + (assign83270_e124969 * var_r_dn16))))), ((((var_lcinv2_dn17 * var_sqt2) + (var_lcinv2 * var_sqt2_dn17)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * var_t2_dn17)) - ((((var_t1_dn17 + (19.2 * var_t2_dn17)) - (12.0 * ((var_t1_dn17 * var_t2) + (var_t1 * var_t2_dn17)))) * var_r) + (assign83270_e124969 * var_r_dn17))))), ((((var_lcinv2_dn18 * var_sqt2) + (var_lcinv2 * var_sqt2_dn18)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * var_t2_dn18)) - ((((var_t1_dn18 + (19.2 * var_t2_dn18)) - (12.0 * ((var_t1_dn18 * var_t2) + (var_t1 * var_t2_dn18)))) * var_r) + (assign83270_e124969 * var_r_dn18))))), ((((var_lcinv2_dn19 * var_sqt2) + (var_lcinv2 * var_sqt2_dn19)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * var_t2_dn19)) - ((((var_t1_dn19 + (19.2 * var_t2_dn19)) - (12.0 * ((var_t1_dn19 * var_t2) + (var_t1 * var_t2_dn19)))) * var_r) + (assign83270_e124969 * var_r_dn19))))), ((((var_lcinv2_dn20 * var_sqt2) + (var_lcinv2 * var_sqt2_dn20)) * assign83270_e124972) + (assign83270_e124953 * ((-(12.0 * var_t2_dn20)) - ((((var_t1_dn20 + (19.2 * var_t2_dn20)) - (12.0 * ((var_t1_dn20 * var_t2) + (var_t1 * var_t2_dn20)))) * var_r) + (assign83270_e124969 * var_r_dn20))))),)
    } else {
        (var_migid0, var_migid0_dn5, var_migid0_dn6, var_migid0_dn7, var_migid0_dn8, var_migid0_dn12, var_migid0_dn13, var_migid0_dn14, var_migid0_dn15, var_migid0_dn16, var_migid0_dn17, var_migid0_dn18, var_migid0_dn19, var_migid0_dn20,)
    }
};
        var_migid0 = assign83270_e124975;
        var_migid0_dn5 = assign83270_e124975_d_n5;
        var_migid0_dn6 = assign83270_e124975_d_n6;
        var_migid0_dn7 = assign83270_e124975_d_n7;
        var_migid0_dn8 = assign83270_e124975_d_n8;
        var_migid0_dn12 = assign83270_e124975_d_n12;
        var_migid0_dn13 = assign83270_e124975_d_n13;
        var_migid0_dn14 = assign83270_e124975_d_n14;
        var_migid0_dn15 = assign83270_e124975_d_n15;
        var_migid0_dn16 = assign83270_e124975_d_n16;
        var_migid0_dn17 = assign83270_e124975_d_n17;
        var_migid0_dn18 = assign83270_e124975_d_n18;
        var_migid0_dn19 = assign83270_e124975_d_n19;
        var_migid0_dn20 = assign83270_e124975_d_n20;

        let (assign83280_e124991, assign83280_e124991_d_n5, assign83280_e124991_d_n6, assign83280_e124991_d_n7, assign83280_e124991_d_n8, assign83280_e124991_d_n12, assign83280_e124991_d_n13, assign83280_e124991_d_n14, assign83280_e124991_d_n15, assign83280_e124991_d_n16, assign83280_e124991_d_n17, assign83280_e124991_d_n18, assign83280_e124991_d_n19, assign83280_e124991_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2284 != 0.0)) {
        let assign83280_e124981: f64 = (var_gvsat_ac * var_gvsat_ac);
        let assign83280_e124983: f64 = (assign83280_e124981 * var_cox_qm);
        let assign83280_e124985: f64 = (assign83280_e124983 * var_eta_p_ac);
        let assign83280_e124988: f64 = (var_gmob_dl_ac * var_gmob_dl_ac);
        let assign83280_e124989: f64 = (assign83280_e124985 / assign83280_e124988);
        (assign83280_e124989, (((((((((var_gvsat_ac_dn5 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn5)) * var_cox_qm) + (assign83280_e124981 * var_cox_qm_dn5)) * var_eta_p_ac) + (assign83280_e124983 * var_eta_p_ac_dn5)) * assign83280_e124988) - (assign83280_e124985 * ((var_gmob_dl_ac_dn5 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn5)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((var_gvsat_ac_dn6 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn6)) * var_cox_qm) + (assign83280_e124981 * var_cox_qm_dn6)) * var_eta_p_ac) + (assign83280_e124983 * var_eta_p_ac_dn6)) * assign83280_e124988) - (assign83280_e124985 * ((var_gmob_dl_ac_dn6 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn6)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((var_gvsat_ac_dn7 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn7)) * var_cox_qm) + (assign83280_e124981 * var_cox_qm_dn7)) * var_eta_p_ac) + (assign83280_e124983 * var_eta_p_ac_dn7)) * assign83280_e124988) - (assign83280_e124985 * ((var_gmob_dl_ac_dn7 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn7)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((var_gvsat_ac_dn8 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn8)) * var_cox_qm) + (assign83280_e124981 * var_cox_qm_dn8)) * var_eta_p_ac) + (assign83280_e124983 * var_eta_p_ac_dn8)) * assign83280_e124988) - (assign83280_e124985 * ((var_gmob_dl_ac_dn8 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn8)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((var_gvsat_ac_dn12 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn12)) * var_cox_qm) + (assign83280_e124981 * var_cox_qm_dn12)) * var_eta_p_ac) + (assign83280_e124983 * var_eta_p_ac_dn12)) * assign83280_e124988) - (assign83280_e124985 * ((var_gmob_dl_ac_dn12 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn12)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((var_gvsat_ac_dn13 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn13)) * var_cox_qm) + (assign83280_e124981 * var_cox_qm_dn13)) * var_eta_p_ac) + (assign83280_e124983 * var_eta_p_ac_dn13)) * assign83280_e124988) - (assign83280_e124985 * ((var_gmob_dl_ac_dn13 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn13)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((var_gvsat_ac_dn14 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn14)) * var_cox_qm) + (assign83280_e124981 * var_cox_qm_dn14)) * var_eta_p_ac) + (assign83280_e124983 * var_eta_p_ac_dn14)) * assign83280_e124988) - (assign83280_e124985 * ((var_gmob_dl_ac_dn14 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn14)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((var_gvsat_ac_dn15 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn15)) * var_cox_qm) + (assign83280_e124981 * var_cox_qm_dn15)) * var_eta_p_ac) + (assign83280_e124983 * var_eta_p_ac_dn15)) * assign83280_e124988) - (assign83280_e124985 * ((var_gmob_dl_ac_dn15 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn15)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((var_gvsat_ac_dn16 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn16)) * var_cox_qm) + (assign83280_e124981 * var_cox_qm_dn16)) * var_eta_p_ac) + (assign83280_e124983 * var_eta_p_ac_dn16)) * assign83280_e124988) - (assign83280_e124985 * ((var_gmob_dl_ac_dn16 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn16)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((var_gvsat_ac_dn17 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn17)) * var_cox_qm) + (assign83280_e124981 * var_cox_qm_dn17)) * var_eta_p_ac) + (assign83280_e124983 * var_eta_p_ac_dn17)) * assign83280_e124988) - (assign83280_e124985 * ((var_gmob_dl_ac_dn17 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn17)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((var_gvsat_ac_dn18 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn18)) * var_cox_qm) + (assign83280_e124981 * var_cox_qm_dn18)) * var_eta_p_ac) + (assign83280_e124983 * var_eta_p_ac_dn18)) * assign83280_e124988) - (assign83280_e124985 * ((var_gmob_dl_ac_dn18 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn18)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((var_gvsat_ac_dn19 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn19)) * var_cox_qm) + (assign83280_e124981 * var_cox_qm_dn19)) * var_eta_p_ac) + (assign83280_e124983 * var_eta_p_ac_dn19)) * assign83280_e124988) - (assign83280_e124985 * ((var_gmob_dl_ac_dn19 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn19)))) / (assign83280_e124988 * assign83280_e124988)), (((((((((var_gvsat_ac_dn20 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn20)) * var_cox_qm) + (assign83280_e124981 * var_cox_qm_dn20)) * var_eta_p_ac) + (assign83280_e124983 * var_eta_p_ac_dn20)) * assign83280_e124988) - (assign83280_e124985 * ((var_gmob_dl_ac_dn20 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn20)))) / (assign83280_e124988 * assign83280_e124988)),)
    } else {
        (var_cgeff, var_cgeff_dn5, var_cgeff_dn6, var_cgeff_dn7, var_cgeff_dn8, var_cgeff_dn12, var_cgeff_dn13, var_cgeff_dn14, var_cgeff_dn15, var_cgeff_dn16, var_cgeff_dn17, var_cgeff_dn18, var_cgeff_dn19, var_cgeff_dn20,)
    }
};
        var_cgeff = assign83280_e124991;
        var_cgeff_dn5 = assign83280_e124991_d_n5;
        var_cgeff_dn6 = assign83280_e124991_d_n6;
        var_cgeff_dn7 = assign83280_e124991_d_n7;
        var_cgeff_dn8 = assign83280_e124991_d_n8;
        var_cgeff_dn12 = assign83280_e124991_d_n12;
        var_cgeff_dn13 = assign83280_e124991_d_n13;
        var_cgeff_dn14 = assign83280_e124991_d_n14;
        var_cgeff_dn15 = assign83280_e124991_d_n15;
        var_cgeff_dn16 = assign83280_e124991_d_n16;
        var_cgeff_dn17 = assign83280_e124991_d_n17;
        var_cgeff_dn18 = assign83280_e124991_d_n18;
        var_cgeff_dn19 = assign83280_e124991_d_n19;
        var_cgeff_dn20 = assign83280_e124991_d_n20;

        let assign83290_e124994: f64 = if var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        var_guard2285 = assign83290_e124994;

        let (assign83300_e125018, assign83300_e125018_d_n5, assign83300_e125018_d_n6, assign83300_e125018_d_n7, assign83300_e125018_d_n8, assign83300_e125018_d_n12, assign83300_e125018_d_n13, assign83300_e125018_d_n14, assign83300_e125018_d_n15, assign83300_e125018_d_n16, assign83300_e125018_d_n17, assign83300_e125018_d_n18, assign83300_e125018_d_n19, assign83300_e125018_d_n20,) = {
    if (((var_guard2279 != 0.0) && (var_guard2284 != 0.0)) && (var_guard2285 != 0.0)) {
        let assign83300_e125005: f64 = (12.0 * var_t2);
        let assign83300_e125006: f64 = (1.0 + assign83300_e125005);
        let assign83300_e125007: f64 = (var_sidexc * assign83300_e125006);
        let assign83300_e125010: f64 = (12.0 * var_g_ideal);
        let assign83300_e125012: f64 = (assign83300_e125010 * var_g_ideal);
        let assign83300_e125014: f64 = (assign83300_e125012 * var_nt0);
        let assign83300_e125015: f64 = (assign83300_e125007 / assign83300_e125014);
        let assign83300_e125016: f64 = (var_mig + assign83300_e125015);
        (assign83300_e125016, (var_mig_dn5 + (((((var_sidexc_dn5 * assign83300_e125006) + (var_sidexc * (12.0 * var_t2_dn5))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * var_g_ideal_dn5) * var_g_ideal) + (assign83300_e125010 * var_g_ideal_dn5)) * var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (var_mig_dn6 + (((((var_sidexc_dn6 * assign83300_e125006) + (var_sidexc * (12.0 * var_t2_dn6))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * var_g_ideal_dn6) * var_g_ideal) + (assign83300_e125010 * var_g_ideal_dn6)) * var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (var_mig_dn7 + (((((var_sidexc_dn7 * assign83300_e125006) + (var_sidexc * (12.0 * var_t2_dn7))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * var_g_ideal_dn7) * var_g_ideal) + (assign83300_e125010 * var_g_ideal_dn7)) * var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (var_mig_dn8 + (((((var_sidexc_dn8 * assign83300_e125006) + (var_sidexc * (12.0 * var_t2_dn8))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * var_g_ideal_dn8) * var_g_ideal) + (assign83300_e125010 * var_g_ideal_dn8)) * var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (var_mig_dn12 + (((((var_sidexc_dn12 * assign83300_e125006) + (var_sidexc * (12.0 * var_t2_dn12))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * var_g_ideal_dn12) * var_g_ideal) + (assign83300_e125010 * var_g_ideal_dn12)) * var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (var_mig_dn13 + (((((var_sidexc_dn13 * assign83300_e125006) + (var_sidexc * (12.0 * var_t2_dn13))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * var_g_ideal_dn13) * var_g_ideal) + (assign83300_e125010 * var_g_ideal_dn13)) * var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (var_mig_dn14 + (((((var_sidexc_dn14 * assign83300_e125006) + (var_sidexc * (12.0 * var_t2_dn14))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * var_g_ideal_dn14) * var_g_ideal) + (assign83300_e125010 * var_g_ideal_dn14)) * var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (var_mig_dn15 + (((((var_sidexc_dn15 * assign83300_e125006) + (var_sidexc * (12.0 * var_t2_dn15))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * var_g_ideal_dn15) * var_g_ideal) + (assign83300_e125010 * var_g_ideal_dn15)) * var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (var_mig_dn16 + (((((var_sidexc_dn16 * assign83300_e125006) + (var_sidexc * (12.0 * var_t2_dn16))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * var_g_ideal_dn16) * var_g_ideal) + (assign83300_e125010 * var_g_ideal_dn16)) * var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (var_mig_dn17 + (((((var_sidexc_dn17 * assign83300_e125006) + (var_sidexc * (12.0 * var_t2_dn17))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * var_g_ideal_dn17) * var_g_ideal) + (assign83300_e125010 * var_g_ideal_dn17)) * var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (var_mig_dn18 + (((((var_sidexc_dn18 * assign83300_e125006) + (var_sidexc * (12.0 * var_t2_dn18))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * var_g_ideal_dn18) * var_g_ideal) + (assign83300_e125010 * var_g_ideal_dn18)) * var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (var_mig_dn19 + (((((var_sidexc_dn19 * assign83300_e125006) + (var_sidexc * (12.0 * var_t2_dn19))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * var_g_ideal_dn19) * var_g_ideal) + (assign83300_e125010 * var_g_ideal_dn19)) * var_nt0))) / (assign83300_e125014 * assign83300_e125014))), (var_mig_dn20 + (((((var_sidexc_dn20 * assign83300_e125006) + (var_sidexc * (12.0 * var_t2_dn20))) * assign83300_e125014) - (assign83300_e125007 * ((((12.0 * var_g_ideal_dn20) * var_g_ideal) + (assign83300_e125010 * var_g_ideal_dn20)) * var_nt0))) / (assign83300_e125014 * assign83300_e125014))),)
    } else {
        (var_mig, var_mig_dn5, var_mig_dn6, var_mig_dn7, var_mig_dn8, var_mig_dn12, var_mig_dn13, var_mig_dn14, var_mig_dn15, var_mig_dn16, var_mig_dn17, var_mig_dn18, var_mig_dn19, var_mig_dn20,)
    }
};
        var_mig = assign83300_e125018;
        var_mig_dn5 = assign83300_e125018_d_n5;
        var_mig_dn6 = assign83300_e125018_d_n6;
        var_mig_dn7 = assign83300_e125018_d_n7;
        var_mig_dn8 = assign83300_e125018_d_n8;
        var_mig_dn12 = assign83300_e125018_d_n12;
        var_mig_dn13 = assign83300_e125018_d_n13;
        var_mig_dn14 = assign83300_e125018_d_n14;
        var_mig_dn15 = assign83300_e125018_d_n15;
        var_mig_dn16 = assign83300_e125018_d_n16;
        var_mig_dn17 = assign83300_e125018_d_n17;
        var_mig_dn18 = assign83300_e125018_d_n18;
        var_mig_dn19 = assign83300_e125018_d_n19;
        var_mig_dn20 = assign83300_e125018_d_n20;

        let (assign83310_e125038, assign83310_e125038_d_n5, assign83310_e125038_d_n6, assign83310_e125038_d_n7, assign83310_e125038_d_n8, assign83310_e125038_d_n12, assign83310_e125038_d_n13, assign83310_e125038_d_n14, assign83310_e125038_d_n15, assign83310_e125038_d_n16, assign83310_e125038_d_n17, assign83310_e125038_d_n18, assign83310_e125038_d_n19, assign83310_e125038_d_n20,) = {
    if (((var_guard2279 != 0.0) && (var_guard2284 != 0.0)) && (var_guard2285 != 0.0)) {
        let assign83310_e125027: f64 = (var_sidexc * var_sqt2);
        let assign83310_e125030: f64 = (1.0 + var_r);
        let assign83310_e125031: f64 = (assign83310_e125027 * assign83310_e125030);
        let assign83310_e125034: f64 = (var_g_ideal * var_nt0);
        let assign83310_e125035: f64 = (assign83310_e125031 / assign83310_e125034);
        let assign83310_e125036: f64 = (var_migid0 - assign83310_e125035);
        (assign83310_e125036, (var_migid0_dn5 - (((((((var_sidexc_dn5 * var_sqt2) + (var_sidexc * var_sqt2_dn5)) * assign83310_e125030) + (assign83310_e125027 * var_r_dn5)) * assign83310_e125034) - (assign83310_e125031 * (var_g_ideal_dn5 * var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (var_migid0_dn6 - (((((((var_sidexc_dn6 * var_sqt2) + (var_sidexc * var_sqt2_dn6)) * assign83310_e125030) + (assign83310_e125027 * var_r_dn6)) * assign83310_e125034) - (assign83310_e125031 * (var_g_ideal_dn6 * var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (var_migid0_dn7 - (((((((var_sidexc_dn7 * var_sqt2) + (var_sidexc * var_sqt2_dn7)) * assign83310_e125030) + (assign83310_e125027 * var_r_dn7)) * assign83310_e125034) - (assign83310_e125031 * (var_g_ideal_dn7 * var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (var_migid0_dn8 - (((((((var_sidexc_dn8 * var_sqt2) + (var_sidexc * var_sqt2_dn8)) * assign83310_e125030) + (assign83310_e125027 * var_r_dn8)) * assign83310_e125034) - (assign83310_e125031 * (var_g_ideal_dn8 * var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (var_migid0_dn12 - (((((((var_sidexc_dn12 * var_sqt2) + (var_sidexc * var_sqt2_dn12)) * assign83310_e125030) + (assign83310_e125027 * var_r_dn12)) * assign83310_e125034) - (assign83310_e125031 * (var_g_ideal_dn12 * var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (var_migid0_dn13 - (((((((var_sidexc_dn13 * var_sqt2) + (var_sidexc * var_sqt2_dn13)) * assign83310_e125030) + (assign83310_e125027 * var_r_dn13)) * assign83310_e125034) - (assign83310_e125031 * (var_g_ideal_dn13 * var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (var_migid0_dn14 - (((((((var_sidexc_dn14 * var_sqt2) + (var_sidexc * var_sqt2_dn14)) * assign83310_e125030) + (assign83310_e125027 * var_r_dn14)) * assign83310_e125034) - (assign83310_e125031 * (var_g_ideal_dn14 * var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (var_migid0_dn15 - (((((((var_sidexc_dn15 * var_sqt2) + (var_sidexc * var_sqt2_dn15)) * assign83310_e125030) + (assign83310_e125027 * var_r_dn15)) * assign83310_e125034) - (assign83310_e125031 * (var_g_ideal_dn15 * var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (var_migid0_dn16 - (((((((var_sidexc_dn16 * var_sqt2) + (var_sidexc * var_sqt2_dn16)) * assign83310_e125030) + (assign83310_e125027 * var_r_dn16)) * assign83310_e125034) - (assign83310_e125031 * (var_g_ideal_dn16 * var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (var_migid0_dn17 - (((((((var_sidexc_dn17 * var_sqt2) + (var_sidexc * var_sqt2_dn17)) * assign83310_e125030) + (assign83310_e125027 * var_r_dn17)) * assign83310_e125034) - (assign83310_e125031 * (var_g_ideal_dn17 * var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (var_migid0_dn18 - (((((((var_sidexc_dn18 * var_sqt2) + (var_sidexc * var_sqt2_dn18)) * assign83310_e125030) + (assign83310_e125027 * var_r_dn18)) * assign83310_e125034) - (assign83310_e125031 * (var_g_ideal_dn18 * var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (var_migid0_dn19 - (((((((var_sidexc_dn19 * var_sqt2) + (var_sidexc * var_sqt2_dn19)) * assign83310_e125030) + (assign83310_e125027 * var_r_dn19)) * assign83310_e125034) - (assign83310_e125031 * (var_g_ideal_dn19 * var_nt0))) / (assign83310_e125034 * assign83310_e125034))), (var_migid0_dn20 - (((((((var_sidexc_dn20 * var_sqt2) + (var_sidexc * var_sqt2_dn20)) * assign83310_e125030) + (assign83310_e125027 * var_r_dn20)) * assign83310_e125034) - (assign83310_e125031 * (var_g_ideal_dn20 * var_nt0))) / (assign83310_e125034 * assign83310_e125034))),)
    } else {
        (var_migid0, var_migid0_dn5, var_migid0_dn6, var_migid0_dn7, var_migid0_dn8, var_migid0_dn12, var_migid0_dn13, var_migid0_dn14, var_migid0_dn15, var_migid0_dn16, var_migid0_dn17, var_migid0_dn18, var_migid0_dn19, var_migid0_dn20,)
    }
};
        var_migid0 = assign83310_e125038;
        var_migid0_dn5 = assign83310_e125038_d_n5;
        var_migid0_dn6 = assign83310_e125038_d_n6;
        var_migid0_dn7 = assign83310_e125038_d_n7;
        var_migid0_dn8 = assign83310_e125038_d_n8;
        var_migid0_dn12 = assign83310_e125038_d_n12;
        var_migid0_dn13 = assign83310_e125038_d_n13;
        var_migid0_dn14 = assign83310_e125038_d_n14;
        var_migid0_dn15 = assign83310_e125038_d_n15;
        var_migid0_dn16 = assign83310_e125038_d_n16;
        var_migid0_dn17 = assign83310_e125038_d_n17;
        var_migid0_dn18 = assign83310_e125038_d_n18;
        var_migid0_dn19 = assign83310_e125038_d_n19;
        var_migid0_dn20 = assign83310_e125038_d_n20;

        let (assign83320_e125047, assign83320_e125047_d_n5, assign83320_e125047_d_n6, assign83320_e125047_d_n7, assign83320_e125047_d_n8, assign83320_e125047_d_n12, assign83320_e125047_d_n13, assign83320_e125047_d_n14, assign83320_e125047_d_n15, assign83320_e125047_d_n16, assign83320_e125047_d_n17, assign83320_e125047_d_n18, assign83320_e125047_d_n19, assign83320_e125047_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2284 != 0.0)) {
        let assign83320_e125044: f64 = (var_nt / var_mig);
        let assign83320_e125045: f64 = (assign83320_e125044).sqrt();
        (assign83320_e125045, ((-((var_nt * var_mig_dn5) / (var_mig * var_mig))) / (2.0 * assign83320_e125045)), ((-((var_nt * var_mig_dn6) / (var_mig * var_mig))) / (2.0 * assign83320_e125045)), ((-((var_nt * var_mig_dn7) / (var_mig * var_mig))) / (2.0 * assign83320_e125045)), ((-((var_nt * var_mig_dn8) / (var_mig * var_mig))) / (2.0 * assign83320_e125045)), ((-((var_nt * var_mig_dn12) / (var_mig * var_mig))) / (2.0 * assign83320_e125045)), ((-((var_nt * var_mig_dn13) / (var_mig * var_mig))) / (2.0 * assign83320_e125045)), ((-((var_nt * var_mig_dn14) / (var_mig * var_mig))) / (2.0 * assign83320_e125045)), ((-((var_nt * var_mig_dn15) / (var_mig * var_mig))) / (2.0 * assign83320_e125045)), ((-((var_nt * var_mig_dn16) / (var_mig * var_mig))) / (2.0 * assign83320_e125045)), ((-((var_nt * var_mig_dn17) / (var_mig * var_mig))) / (2.0 * assign83320_e125045)), ((-((var_nt * var_mig_dn18) / (var_mig * var_mig))) / (2.0 * assign83320_e125045)), ((-((var_nt * var_mig_dn19) / (var_mig * var_mig))) / (2.0 * assign83320_e125045)), ((-((var_nt * var_mig_dn20) / (var_mig * var_mig))) / (2.0 * assign83320_e125045)),)
    } else {
        (var_sqig, var_sqig_dn5, var_sqig_dn6, var_sqig_dn7, var_sqig_dn8, var_sqig_dn12, var_sqig_dn13, var_sqig_dn14, var_sqig_dn15, var_sqig_dn16, var_sqig_dn17, var_sqig_dn18, var_sqig_dn19, var_sqig_dn20,)
    }
};
        var_sqig = assign83320_e125047;
        var_sqig_dn5 = assign83320_e125047_d_n5;
        var_sqig_dn6 = assign83320_e125047_d_n6;
        var_sqig_dn7 = assign83320_e125047_d_n7;
        var_sqig_dn8 = assign83320_e125047_d_n8;
        var_sqig_dn12 = assign83320_e125047_d_n12;
        var_sqig_dn13 = assign83320_e125047_d_n13;
        var_sqig_dn14 = assign83320_e125047_d_n14;
        var_sqig_dn15 = assign83320_e125047_d_n15;
        var_sqig_dn16 = assign83320_e125047_d_n16;
        var_sqig_dn17 = assign83320_e125047_d_n17;
        var_sqig_dn18 = assign83320_e125047_d_n18;
        var_sqig_dn19 = assign83320_e125047_d_n19;
        var_sqig_dn20 = assign83320_e125047_d_n20;

        let assign83330_e125050: f64 = if var_sqid <= 0.0 { 1.0 } else { 0.0 };
        var_guard2286 = assign83330_e125050;

        let (assign83340_e125058, assign83340_e125058_d_n5, assign83340_e125058_d_n6, assign83340_e125058_d_n7, assign83340_e125058_d_n8, assign83340_e125058_d_n12, assign83340_e125058_d_n13, assign83340_e125058_d_n14, assign83340_e125058_d_n15, assign83340_e125058_d_n16, assign83340_e125058_d_n17, assign83340_e125058_d_n18, assign83340_e125058_d_n19, assign83340_e125058_d_n20,) = {
    if (((var_guard2279 != 0.0) && (var_guard2284 != 0.0)) && (var_guard2286 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_c_igid, var_c_igid_dn5, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8, var_c_igid_dn12, var_c_igid_dn13, var_c_igid_dn14, var_c_igid_dn15, var_c_igid_dn16, var_c_igid_dn17, var_c_igid_dn18, var_c_igid_dn19, var_c_igid_dn20,)
    }
};
        var_c_igid = assign83340_e125058;
        var_c_igid_dn5 = assign83340_e125058_d_n5;
        var_c_igid_dn6 = assign83340_e125058_d_n6;
        var_c_igid_dn7 = assign83340_e125058_d_n7;
        var_c_igid_dn8 = assign83340_e125058_d_n8;
        var_c_igid_dn12 = assign83340_e125058_d_n12;
        var_c_igid_dn13 = assign83340_e125058_d_n13;
        var_c_igid_dn14 = assign83340_e125058_d_n14;
        var_c_igid_dn15 = assign83340_e125058_d_n15;
        var_c_igid_dn16 = assign83340_e125058_d_n16;
        var_c_igid_dn17 = assign83340_e125058_d_n17;
        var_c_igid_dn18 = assign83340_e125058_d_n18;
        var_c_igid_dn19 = assign83340_e125058_d_n19;
        var_c_igid_dn20 = assign83340_e125058_d_n20;

        let (assign83350_e125071, assign83350_e125071_d_n5, assign83350_e125071_d_n6, assign83350_e125071_d_n7, assign83350_e125071_d_n8, assign83350_e125071_d_n12, assign83350_e125071_d_n13, assign83350_e125071_d_n14, assign83350_e125071_d_n15, assign83350_e125071_d_n16, assign83350_e125071_d_n17, assign83350_e125071_d_n18, assign83350_e125071_d_n19, assign83350_e125071_d_n20,) = {
    if (((var_guard2279 != 0.0) && (var_guard2284 != 0.0)) && (var_guard2286 == 0.0)) {
        let assign83350_e125067: f64 = (var_migid0 * var_sqig);
        let assign83350_e125069: f64 = (assign83350_e125067 / var_sqid);
        (assign83350_e125069, (((((var_migid0_dn5 * var_sqig) + (var_migid0 * var_sqig_dn5)) * var_sqid) - (assign83350_e125067 * var_sqid_dn5)) / (var_sqid * var_sqid)), (((((var_migid0_dn6 * var_sqig) + (var_migid0 * var_sqig_dn6)) * var_sqid) - (assign83350_e125067 * var_sqid_dn6)) / (var_sqid * var_sqid)), (((((var_migid0_dn7 * var_sqig) + (var_migid0 * var_sqig_dn7)) * var_sqid) - (assign83350_e125067 * var_sqid_dn7)) / (var_sqid * var_sqid)), (((((var_migid0_dn8 * var_sqig) + (var_migid0 * var_sqig_dn8)) * var_sqid) - (assign83350_e125067 * var_sqid_dn8)) / (var_sqid * var_sqid)), (((((var_migid0_dn12 * var_sqig) + (var_migid0 * var_sqig_dn12)) * var_sqid) - (assign83350_e125067 * var_sqid_dn12)) / (var_sqid * var_sqid)), (((((var_migid0_dn13 * var_sqig) + (var_migid0 * var_sqig_dn13)) * var_sqid) - (assign83350_e125067 * var_sqid_dn13)) / (var_sqid * var_sqid)), (((((var_migid0_dn14 * var_sqig) + (var_migid0 * var_sqig_dn14)) * var_sqid) - (assign83350_e125067 * var_sqid_dn14)) / (var_sqid * var_sqid)), (((((var_migid0_dn15 * var_sqig) + (var_migid0 * var_sqig_dn15)) * var_sqid) - (assign83350_e125067 * var_sqid_dn15)) / (var_sqid * var_sqid)), (((((var_migid0_dn16 * var_sqig) + (var_migid0 * var_sqig_dn16)) * var_sqid) - (assign83350_e125067 * var_sqid_dn16)) / (var_sqid * var_sqid)), (((((var_migid0_dn17 * var_sqig) + (var_migid0 * var_sqig_dn17)) * var_sqid) - (assign83350_e125067 * var_sqid_dn17)) / (var_sqid * var_sqid)), (((((var_migid0_dn18 * var_sqig) + (var_migid0 * var_sqig_dn18)) * var_sqid) - (assign83350_e125067 * var_sqid_dn18)) / (var_sqid * var_sqid)), (((((var_migid0_dn19 * var_sqig) + (var_migid0 * var_sqig_dn19)) * var_sqid) - (assign83350_e125067 * var_sqid_dn19)) / (var_sqid * var_sqid)), (((((var_migid0_dn20 * var_sqig) + (var_migid0 * var_sqig_dn20)) * var_sqid) - (assign83350_e125067 * var_sqid_dn20)) / (var_sqid * var_sqid)),)
    } else {
        (var_c_igid, var_c_igid_dn5, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8, var_c_igid_dn12, var_c_igid_dn13, var_c_igid_dn14, var_c_igid_dn15, var_c_igid_dn16, var_c_igid_dn17, var_c_igid_dn18, var_c_igid_dn19, var_c_igid_dn20,)
    }
};
        var_c_igid = assign83350_e125071;
        var_c_igid_dn5 = assign83350_e125071_d_n5;
        var_c_igid_dn6 = assign83350_e125071_d_n6;
        var_c_igid_dn7 = assign83350_e125071_d_n7;
        var_c_igid_dn8 = assign83350_e125071_d_n8;
        var_c_igid_dn12 = assign83350_e125071_d_n12;
        var_c_igid_dn13 = assign83350_e125071_d_n13;
        var_c_igid_dn14 = assign83350_e125071_d_n14;
        var_c_igid_dn15 = assign83350_e125071_d_n15;
        var_c_igid_dn16 = assign83350_e125071_d_n16;
        var_c_igid_dn17 = assign83350_e125071_d_n17;
        var_c_igid_dn18 = assign83350_e125071_d_n18;
        var_c_igid_dn19 = assign83350_e125071_d_n19;
        var_c_igid_dn20 = assign83350_e125071_d_n20;

        *var_c_igid_slot = var_c_igid;
        *var_c_igid_dn12_slot = var_c_igid_dn12;
        *var_c_igid_dn13_slot = var_c_igid_dn13;
        *var_c_igid_dn14_slot = var_c_igid_dn14;
        *var_c_igid_dn15_slot = var_c_igid_dn15;
        *var_c_igid_dn16_slot = var_c_igid_dn16;
        *var_c_igid_dn17_slot = var_c_igid_dn17;
        *var_c_igid_dn18_slot = var_c_igid_dn18;
        *var_c_igid_dn19_slot = var_c_igid_dn19;
        *var_c_igid_dn20_slot = var_c_igid_dn20;
        *var_c_igid_dn5_slot = var_c_igid_dn5;
        *var_c_igid_dn6_slot = var_c_igid_dn6;
        *var_c_igid_dn7_slot = var_c_igid_dn7;
        *var_c_igid_dn8_slot = var_c_igid_dn8;
        *var_cgeff_slot = var_cgeff;
        *var_cgeff_dn12_slot = var_cgeff_dn12;
        *var_cgeff_dn13_slot = var_cgeff_dn13;
        *var_cgeff_dn14_slot = var_cgeff_dn14;
        *var_cgeff_dn15_slot = var_cgeff_dn15;
        *var_cgeff_dn16_slot = var_cgeff_dn16;
        *var_cgeff_dn17_slot = var_cgeff_dn17;
        *var_cgeff_dn18_slot = var_cgeff_dn18;
        *var_cgeff_dn19_slot = var_cgeff_dn19;
        *var_cgeff_dn20_slot = var_cgeff_dn20;
        *var_cgeff_dn5_slot = var_cgeff_dn5;
        *var_cgeff_dn6_slot = var_cgeff_dn6;
        *var_cgeff_dn7_slot = var_cgeff_dn7;
        *var_cgeff_dn8_slot = var_cgeff_dn8;
        *var_gfac_slot = var_gfac;
        *var_gfac_dn12_slot = var_gfac_dn12;
        *var_gfac_dn13_slot = var_gfac_dn13;
        *var_gfac_dn14_slot = var_gfac_dn14;
        *var_gfac_dn15_slot = var_gfac_dn15;
        *var_gfac_dn16_slot = var_gfac_dn16;
        *var_gfac_dn17_slot = var_gfac_dn17;
        *var_gfac_dn18_slot = var_gfac_dn18;
        *var_gfac_dn19_slot = var_gfac_dn19;
        *var_gfac_dn20_slot = var_gfac_dn20;
        *var_gfac_dn5_slot = var_gfac_dn5;
        *var_gfac_dn6_slot = var_gfac_dn6;
        *var_gfac_dn7_slot = var_gfac_dn7;
        *var_gfac_dn8_slot = var_gfac_dn8;
        *var_guard2283_slot = var_guard2283;
        *var_guard2284_slot = var_guard2284;
        *var_guard2285_slot = var_guard2285;
        *var_guard2286_slot = var_guard2286;
        *var_gvsat_exc_slot = var_gvsat_exc;
        *var_gvsat_exc_dn12_slot = var_gvsat_exc_dn12;
        *var_gvsat_exc_dn13_slot = var_gvsat_exc_dn13;
        *var_gvsat_exc_dn14_slot = var_gvsat_exc_dn14;
        *var_gvsat_exc_dn15_slot = var_gvsat_exc_dn15;
        *var_gvsat_exc_dn16_slot = var_gvsat_exc_dn16;
        *var_gvsat_exc_dn17_slot = var_gvsat_exc_dn17;
        *var_gvsat_exc_dn18_slot = var_gvsat_exc_dn18;
        *var_gvsat_exc_dn19_slot = var_gvsat_exc_dn19;
        *var_gvsat_exc_dn20_slot = var_gvsat_exc_dn20;
        *var_gvsat_exc_dn5_slot = var_gvsat_exc_dn5;
        *var_gvsat_exc_dn6_slot = var_gvsat_exc_dn6;
        *var_gvsat_exc_dn7_slot = var_gvsat_exc_dn7;
        *var_gvsat_exc_dn8_slot = var_gvsat_exc_dn8;
        *var_mid_slot = var_mid;
        *var_mid_dn12_slot = var_mid_dn12;
        *var_mid_dn13_slot = var_mid_dn13;
        *var_mid_dn14_slot = var_mid_dn14;
        *var_mid_dn15_slot = var_mid_dn15;
        *var_mid_dn16_slot = var_mid_dn16;
        *var_mid_dn17_slot = var_mid_dn17;
        *var_mid_dn18_slot = var_mid_dn18;
        *var_mid_dn19_slot = var_mid_dn19;
        *var_mid_dn20_slot = var_mid_dn20;
        *var_mid_dn5_slot = var_mid_dn5;
        *var_mid_dn6_slot = var_mid_dn6;
        *var_mid_dn7_slot = var_mid_dn7;
        *var_mid_dn8_slot = var_mid_dn8;
        *var_mig_slot = var_mig;
        *var_mig_dn12_slot = var_mig_dn12;
        *var_mig_dn13_slot = var_mig_dn13;
        *var_mig_dn14_slot = var_mig_dn14;
        *var_mig_dn15_slot = var_mig_dn15;
        *var_mig_dn16_slot = var_mig_dn16;
        *var_mig_dn17_slot = var_mig_dn17;
        *var_mig_dn18_slot = var_mig_dn18;
        *var_mig_dn19_slot = var_mig_dn19;
        *var_mig_dn20_slot = var_mig_dn20;
        *var_mig_dn5_slot = var_mig_dn5;
        *var_mig_dn6_slot = var_mig_dn6;
        *var_mig_dn7_slot = var_mig_dn7;
        *var_mig_dn8_slot = var_mig_dn8;
        *var_migid0_slot = var_migid0;
        *var_migid0_dn12_slot = var_migid0_dn12;
        *var_migid0_dn13_slot = var_migid0_dn13;
        *var_migid0_dn14_slot = var_migid0_dn14;
        *var_migid0_dn15_slot = var_migid0_dn15;
        *var_migid0_dn16_slot = var_migid0_dn16;
        *var_migid0_dn17_slot = var_migid0_dn17;
        *var_migid0_dn18_slot = var_migid0_dn18;
        *var_migid0_dn19_slot = var_migid0_dn19;
        *var_migid0_dn20_slot = var_migid0_dn20;
        *var_migid0_dn5_slot = var_migid0_dn5;
        *var_migid0_dn6_slot = var_migid0_dn6;
        *var_migid0_dn7_slot = var_migid0_dn7;
        *var_migid0_dn8_slot = var_migid0_dn8;
        *var_sidexc_slot = var_sidexc;
        *var_sidexc_dn12_slot = var_sidexc_dn12;
        *var_sidexc_dn13_slot = var_sidexc_dn13;
        *var_sidexc_dn14_slot = var_sidexc_dn14;
        *var_sidexc_dn15_slot = var_sidexc_dn15;
        *var_sidexc_dn16_slot = var_sidexc_dn16;
        *var_sidexc_dn17_slot = var_sidexc_dn17;
        *var_sidexc_dn18_slot = var_sidexc_dn18;
        *var_sidexc_dn19_slot = var_sidexc_dn19;
        *var_sidexc_dn20_slot = var_sidexc_dn20;
        *var_sidexc_dn5_slot = var_sidexc_dn5;
        *var_sidexc_dn6_slot = var_sidexc_dn6;
        *var_sidexc_dn7_slot = var_sidexc_dn7;
        *var_sidexc_dn8_slot = var_sidexc_dn8;
        *var_sqid_slot = var_sqid;
        *var_sqid_dn12_slot = var_sqid_dn12;
        *var_sqid_dn13_slot = var_sqid_dn13;
        *var_sqid_dn14_slot = var_sqid_dn14;
        *var_sqid_dn15_slot = var_sqid_dn15;
        *var_sqid_dn16_slot = var_sqid_dn16;
        *var_sqid_dn17_slot = var_sqid_dn17;
        *var_sqid_dn18_slot = var_sqid_dn18;
        *var_sqid_dn19_slot = var_sqid_dn19;
        *var_sqid_dn20_slot = var_sqid_dn20;
        *var_sqid_dn5_slot = var_sqid_dn5;
        *var_sqid_dn6_slot = var_sqid_dn6;
        *var_sqid_dn7_slot = var_sqid_dn7;
        *var_sqid_dn8_slot = var_sqid_dn8;
        *var_sqig_slot = var_sqig;
        *var_sqig_dn12_slot = var_sqig_dn12;
        *var_sqig_dn13_slot = var_sqig_dn13;
        *var_sqig_dn14_slot = var_sqig_dn14;
        *var_sqig_dn15_slot = var_sqig_dn15;
        *var_sqig_dn16_slot = var_sqig_dn16;
        *var_sqig_dn17_slot = var_sqig_dn17;
        *var_sqig_dn18_slot = var_sqig_dn18;
        *var_sqig_dn19_slot = var_sqig_dn19;
        *var_sqig_dn20_slot = var_sqig_dn20;
        *var_sqig_dn5_slot = var_sqig_dn5;
        *var_sqig_dn6_slot = var_sqig_dn6;
        *var_sqig_dn7_slot = var_sqig_dn7;
        *var_sqig_dn8_slot = var_sqig_dn8;
        *var_thesat1_exc_slot = var_thesat1_exc;
        *var_thesat1_exc_dn12_slot = var_thesat1_exc_dn12;
        *var_thesat1_exc_dn13_slot = var_thesat1_exc_dn13;
        *var_thesat1_exc_dn14_slot = var_thesat1_exc_dn14;
        *var_thesat1_exc_dn15_slot = var_thesat1_exc_dn15;
        *var_thesat1_exc_dn16_slot = var_thesat1_exc_dn16;
        *var_thesat1_exc_dn17_slot = var_thesat1_exc_dn17;
        *var_thesat1_exc_dn18_slot = var_thesat1_exc_dn18;
        *var_thesat1_exc_dn19_slot = var_thesat1_exc_dn19;
        *var_thesat1_exc_dn20_slot = var_thesat1_exc_dn20;
        *var_thesat1_exc_dn5_slot = var_thesat1_exc_dn5;
        *var_thesat1_exc_dn6_slot = var_thesat1_exc_dn6;
        *var_thesat1_exc_dn7_slot = var_thesat1_exc_dn7;
        *var_thesat1_exc_dn8_slot = var_thesat1_exc_dn8;
        *var_zsat_exc_slot = var_zsat_exc;
        *var_zsat_exc_dn12_slot = var_zsat_exc_dn12;
        *var_zsat_exc_dn13_slot = var_zsat_exc_dn13;
        *var_zsat_exc_dn14_slot = var_zsat_exc_dn14;
        *var_zsat_exc_dn15_slot = var_zsat_exc_dn15;
        *var_zsat_exc_dn16_slot = var_zsat_exc_dn16;
        *var_zsat_exc_dn17_slot = var_zsat_exc_dn17;
        *var_zsat_exc_dn18_slot = var_zsat_exc_dn18;
        *var_zsat_exc_dn19_slot = var_zsat_exc_dn19;
        *var_zsat_exc_dn20_slot = var_zsat_exc_dn20;
        *var_zsat_exc_dn5_slot = var_zsat_exc_dn5;
        *var_zsat_exc_dn6_slot = var_zsat_exc_dn6;
        *var_zsat_exc_dn7_slot = var_zsat_exc_dn7;
        *var_zsat_exc_dn8_slot = var_zsat_exc_dn8;
    }

    pub(super) fn stamp_transient_block_261(
        p: &Parameters,
        var_alpha_dc: f64,
        var_alpha_dc_dn12: f64,
        var_alpha_dc_dn13: f64,
        var_alpha_dc_dn14: f64,
        var_alpha_dc_dn15: f64,
        var_alpha_dc_dn16: f64,
        var_alpha_dc_dn17: f64,
        var_alpha_dc_dn18: f64,
        var_alpha_dc_dn19: f64,
        var_alpha_dc_dn20: f64,
        var_alpha_dc_dn5: f64,
        var_alpha_dc_dn6: f64,
        var_alpha_dc_dn7: f64,
        var_alpha_dc_dn8: f64,
        var_betnedge_i: f64,
        var_cox_over_q: f64,
        var_dsqredge: f64,
        var_dsqredge_dn12: f64,
        var_dsqredge_dn13: f64,
        var_dsqredge_dn14: f64,
        var_dsqredge_dn15: f64,
        var_dsqredge_dn16: f64,
        var_dsqredge_dn17: f64,
        var_dsqredge_dn18: f64,
        var_dsqredge_dn19: f64,
        var_dsqredge_dn20: f64,
        var_dsqredge_dn5: f64,
        var_dsqredge_dn6: f64,
        var_dsqredge_dn7: f64,
        var_dsqredge_dn8: f64,
        var_gfedge2: f64,
        var_guard2279: f64,
        var_guard2284: f64,
        var_h_dc: f64,
        var_h_dc_dn12: f64,
        var_h_dc_dn13: f64,
        var_h_dc_dn14: f64,
        var_h_dc_dn15: f64,
        var_h_dc_dn16: f64,
        var_h_dc_dn17: f64,
        var_h_dc_dn18: f64,
        var_h_dc_dn19: f64,
        var_h_dc_dn20: f64,
        var_h_dc_dn5: f64,
        var_h_dc_dn6: f64,
        var_h_dc_dn7: f64,
        var_h_dc_dn8: f64,
        var_phit: f64,
        var_sqid: f64,
        var_sqid_dn12: f64,
        var_sqid_dn13: f64,
        var_sqid_dn14: f64,
        var_sqid_dn15: f64,
        var_sqid_dn16: f64,
        var_sqid_dn17: f64,
        var_sqid_dn18: f64,
        var_sqid_dn19: f64,
        var_sqid_dn20: f64,
        var_sqid_dn5: f64,
        var_sqid_dn6: f64,
        var_sqid_dn7: f64,
        var_sqid_dn8: f64,
        var_sqig: f64,
        var_sqig_dn12: f64,
        var_sqig_dn13: f64,
        var_sqig_dn14: f64,
        var_sqig_dn15: f64,
        var_sqig_dn16: f64,
        var_sqig_dn17: f64,
        var_sqig_dn18: f64,
        var_sqig_dn19: f64,
        var_sqig_dn20: f64,
        var_sqig_dn5: f64,
        var_sqig_dn6: f64,
        var_sqig_dn7: f64,
        var_sqig_dn8: f64,
        var_xgedge: f64,
        var_c_igid_slot: &mut f64,
        var_c_igid_dn12_slot: &mut f64,
        var_c_igid_dn13_slot: &mut f64,
        var_c_igid_dn14_slot: &mut f64,
        var_c_igid_dn15_slot: &mut f64,
        var_c_igid_dn16_slot: &mut f64,
        var_c_igid_dn17_slot: &mut f64,
        var_c_igid_dn18_slot: &mut f64,
        var_c_igid_dn19_slot: &mut f64,
        var_c_igid_dn20_slot: &mut f64,
        var_c_igid_dn5_slot: &mut f64,
        var_c_igid_dn6_slot: &mut f64,
        var_c_igid_dn7_slot: &mut f64,
        var_c_igid_dn8_slot: &mut f64,
        var_guard2288_slot: &mut f64,
        var_migid_slot: &mut f64,
        var_migid_dn12_slot: &mut f64,
        var_migid_dn13_slot: &mut f64,
        var_migid_dn14_slot: &mut f64,
        var_migid_dn15_slot: &mut f64,
        var_migid_dn16_slot: &mut f64,
        var_migid_dn17_slot: &mut f64,
        var_migid_dn18_slot: &mut f64,
        var_migid_dn19_slot: &mut f64,
        var_migid_dn20_slot: &mut f64,
        var_migid_dn5_slot: &mut f64,
        var_migid_dn6_slot: &mut f64,
        var_migid_dn7_slot: &mut f64,
        var_migid_dn8_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn12_slot: &mut f64,
        var_temp1_dn13_slot: &mut f64,
        var_temp1_dn14_slot: &mut f64,
        var_temp1_dn15_slot: &mut f64,
        var_temp1_dn16_slot: &mut f64,
        var_temp1_dn17_slot: &mut f64,
        var_temp1_dn18_slot: &mut f64,
        var_temp1_dn19_slot: &mut f64,
        var_temp1_dn20_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
    ) {
        let mut var_c_igid: f64 = *var_c_igid_slot;
        let mut var_c_igid_dn12: f64 = *var_c_igid_dn12_slot;
        let mut var_c_igid_dn13: f64 = *var_c_igid_dn13_slot;
        let mut var_c_igid_dn14: f64 = *var_c_igid_dn14_slot;
        let mut var_c_igid_dn15: f64 = *var_c_igid_dn15_slot;
        let mut var_c_igid_dn16: f64 = *var_c_igid_dn16_slot;
        let mut var_c_igid_dn17: f64 = *var_c_igid_dn17_slot;
        let mut var_c_igid_dn18: f64 = *var_c_igid_dn18_slot;
        let mut var_c_igid_dn19: f64 = *var_c_igid_dn19_slot;
        let mut var_c_igid_dn20: f64 = *var_c_igid_dn20_slot;
        let mut var_c_igid_dn5: f64 = *var_c_igid_dn5_slot;
        let mut var_c_igid_dn6: f64 = *var_c_igid_dn6_slot;
        let mut var_c_igid_dn7: f64 = *var_c_igid_dn7_slot;
        let mut var_c_igid_dn8: f64 = *var_c_igid_dn8_slot;
        let mut var_guard2288: f64 = *var_guard2288_slot;
        let mut var_migid: f64 = *var_migid_slot;
        let mut var_migid_dn12: f64 = *var_migid_dn12_slot;
        let mut var_migid_dn13: f64 = *var_migid_dn13_slot;
        let mut var_migid_dn14: f64 = *var_migid_dn14_slot;
        let mut var_migid_dn15: f64 = *var_migid_dn15_slot;
        let mut var_migid_dn16: f64 = *var_migid_dn16_slot;
        let mut var_migid_dn17: f64 = *var_migid_dn17_slot;
        let mut var_migid_dn18: f64 = *var_migid_dn18_slot;
        let mut var_migid_dn19: f64 = *var_migid_dn19_slot;
        let mut var_migid_dn20: f64 = *var_migid_dn20_slot;
        let mut var_migid_dn5: f64 = *var_migid_dn5_slot;
        let mut var_migid_dn6: f64 = *var_migid_dn6_slot;
        let mut var_migid_dn7: f64 = *var_migid_dn7_slot;
        let mut var_migid_dn8: f64 = *var_migid_dn8_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn12: f64 = *var_temp1_dn12_slot;
        let mut var_temp1_dn13: f64 = *var_temp1_dn13_slot;
        let mut var_temp1_dn14: f64 = *var_temp1_dn14_slot;
        let mut var_temp1_dn15: f64 = *var_temp1_dn15_slot;
        let mut var_temp1_dn16: f64 = *var_temp1_dn16_slot;
        let mut var_temp1_dn17: f64 = *var_temp1_dn17_slot;
        let mut var_temp1_dn18: f64 = *var_temp1_dn18_slot;
        let mut var_temp1_dn19: f64 = *var_temp1_dn19_slot;
        let mut var_temp1_dn20: f64 = *var_temp1_dn20_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;

        let (assign83360_e125087, assign83360_e125087_d_n5, assign83360_e125087_d_n6, assign83360_e125087_d_n7, assign83360_e125087_d_n8, assign83360_e125087_d_n12, assign83360_e125087_d_n13, assign83360_e125087_d_n14, assign83360_e125087_d_n15, assign83360_e125087_d_n16, assign83360_e125087_d_n17, assign83360_e125087_d_n18, assign83360_e125087_d_n19, assign83360_e125087_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2284 != 0.0)) {
        let (assign83360_e125085, assign83360_e125085_d_n5, assign83360_e125085_d_n6, assign83360_e125085_d_n7, assign83360_e125085_d_n8, assign83360_e125085_d_n12, assign83360_e125085_d_n13, assign83360_e125085_d_n14, assign83360_e125085_d_n15, assign83360_e125085_d_n16, assign83360_e125085_d_n17, assign83360_e125085_d_n18, assign83360_e125085_d_n19, assign83360_e125085_d_n20,) = {
            if (var_c_igid > 0.0) {
                let (assign83360_e125083, assign83360_e125083_d_n5, assign83360_e125083_d_n6, assign83360_e125083_d_n7, assign83360_e125083_d_n8, assign83360_e125083_d_n12, assign83360_e125083_d_n13, assign83360_e125083_d_n14, assign83360_e125083_d_n15, assign83360_e125083_d_n16, assign83360_e125083_d_n17, assign83360_e125083_d_n18, assign83360_e125083_d_n19, assign83360_e125083_d_n20,) = {
                    if (var_c_igid < 1.0) {
                        (var_c_igid, var_c_igid_dn5, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8, var_c_igid_dn12, var_c_igid_dn13, var_c_igid_dn14, var_c_igid_dn15, var_c_igid_dn16, var_c_igid_dn17, var_c_igid_dn18, var_c_igid_dn19, var_c_igid_dn20,)
                    } else {
                        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign83360_e125083, assign83360_e125083_d_n5, assign83360_e125083_d_n6, assign83360_e125083_d_n7, assign83360_e125083_d_n8, assign83360_e125083_d_n12, assign83360_e125083_d_n13, assign83360_e125083_d_n14, assign83360_e125083_d_n15, assign83360_e125083_d_n16, assign83360_e125083_d_n17, assign83360_e125083_d_n18, assign83360_e125083_d_n19, assign83360_e125083_d_n20,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign83360_e125085, assign83360_e125085_d_n5, assign83360_e125085_d_n6, assign83360_e125085_d_n7, assign83360_e125085_d_n8, assign83360_e125085_d_n12, assign83360_e125085_d_n13, assign83360_e125085_d_n14, assign83360_e125085_d_n15, assign83360_e125085_d_n16, assign83360_e125085_d_n17, assign83360_e125085_d_n18, assign83360_e125085_d_n19, assign83360_e125085_d_n20,)
    } else {
        (var_c_igid, var_c_igid_dn5, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8, var_c_igid_dn12, var_c_igid_dn13, var_c_igid_dn14, var_c_igid_dn15, var_c_igid_dn16, var_c_igid_dn17, var_c_igid_dn18, var_c_igid_dn19, var_c_igid_dn20,)
    }
};
        var_c_igid = assign83360_e125087;
        var_c_igid_dn5 = assign83360_e125087_d_n5;
        var_c_igid_dn6 = assign83360_e125087_d_n6;
        var_c_igid_dn7 = assign83360_e125087_d_n7;
        var_c_igid_dn8 = assign83360_e125087_d_n8;
        var_c_igid_dn12 = assign83360_e125087_d_n12;
        var_c_igid_dn13 = assign83360_e125087_d_n13;
        var_c_igid_dn14 = assign83360_e125087_d_n14;
        var_c_igid_dn15 = assign83360_e125087_d_n15;
        var_c_igid_dn16 = assign83360_e125087_d_n16;
        var_c_igid_dn17 = assign83360_e125087_d_n17;
        var_c_igid_dn18 = assign83360_e125087_d_n18;
        var_c_igid_dn19 = assign83360_e125087_d_n19;
        var_c_igid_dn20 = assign83360_e125087_d_n20;

        let (assign83370_e125097, assign83370_e125097_d_n5, assign83370_e125097_d_n6, assign83370_e125097_d_n7, assign83370_e125097_d_n8, assign83370_e125097_d_n12, assign83370_e125097_d_n13, assign83370_e125097_d_n14, assign83370_e125097_d_n15, assign83370_e125097_d_n16, assign83370_e125097_d_n17, assign83370_e125097_d_n18, assign83370_e125097_d_n19, assign83370_e125097_d_n20,) = {
    if ((var_guard2279 != 0.0) && (var_guard2284 != 0.0)) {
        let assign83370_e125093: f64 = (var_c_igid * var_sqid);
        let assign83370_e125095: f64 = (assign83370_e125093 / var_sqig);
        (assign83370_e125095, (((((var_c_igid_dn5 * var_sqid) + (var_c_igid * var_sqid_dn5)) * var_sqig) - (assign83370_e125093 * var_sqig_dn5)) / (var_sqig * var_sqig)), (((((var_c_igid_dn6 * var_sqid) + (var_c_igid * var_sqid_dn6)) * var_sqig) - (assign83370_e125093 * var_sqig_dn6)) / (var_sqig * var_sqig)), (((((var_c_igid_dn7 * var_sqid) + (var_c_igid * var_sqid_dn7)) * var_sqig) - (assign83370_e125093 * var_sqig_dn7)) / (var_sqig * var_sqig)), (((((var_c_igid_dn8 * var_sqid) + (var_c_igid * var_sqid_dn8)) * var_sqig) - (assign83370_e125093 * var_sqig_dn8)) / (var_sqig * var_sqig)), (((((var_c_igid_dn12 * var_sqid) + (var_c_igid * var_sqid_dn12)) * var_sqig) - (assign83370_e125093 * var_sqig_dn12)) / (var_sqig * var_sqig)), (((((var_c_igid_dn13 * var_sqid) + (var_c_igid * var_sqid_dn13)) * var_sqig) - (assign83370_e125093 * var_sqig_dn13)) / (var_sqig * var_sqig)), (((((var_c_igid_dn14 * var_sqid) + (var_c_igid * var_sqid_dn14)) * var_sqig) - (assign83370_e125093 * var_sqig_dn14)) / (var_sqig * var_sqig)), (((((var_c_igid_dn15 * var_sqid) + (var_c_igid * var_sqid_dn15)) * var_sqig) - (assign83370_e125093 * var_sqig_dn15)) / (var_sqig * var_sqig)), (((((var_c_igid_dn16 * var_sqid) + (var_c_igid * var_sqid_dn16)) * var_sqig) - (assign83370_e125093 * var_sqig_dn16)) / (var_sqig * var_sqig)), (((((var_c_igid_dn17 * var_sqid) + (var_c_igid * var_sqid_dn17)) * var_sqig) - (assign83370_e125093 * var_sqig_dn17)) / (var_sqig * var_sqig)), (((((var_c_igid_dn18 * var_sqid) + (var_c_igid * var_sqid_dn18)) * var_sqig) - (assign83370_e125093 * var_sqig_dn18)) / (var_sqig * var_sqig)), (((((var_c_igid_dn19 * var_sqid) + (var_c_igid * var_sqid_dn19)) * var_sqig) - (assign83370_e125093 * var_sqig_dn19)) / (var_sqig * var_sqig)), (((((var_c_igid_dn20 * var_sqid) + (var_c_igid * var_sqid_dn20)) * var_sqig) - (assign83370_e125093 * var_sqig_dn20)) / (var_sqig * var_sqig)),)
    } else {
        (var_migid, var_migid_dn5, var_migid_dn6, var_migid_dn7, var_migid_dn8, var_migid_dn12, var_migid_dn13, var_migid_dn14, var_migid_dn15, var_migid_dn16, var_migid_dn17, var_migid_dn18, var_migid_dn19, var_migid_dn20,)
    }
};
        var_migid = assign83370_e125097;
        var_migid_dn5 = assign83370_e125097_d_n5;
        var_migid_dn6 = assign83370_e125097_d_n6;
        var_migid_dn7 = assign83370_e125097_d_n7;
        var_migid_dn8 = assign83370_e125097_d_n8;
        var_migid_dn12 = assign83370_e125097_d_n12;
        var_migid_dn13 = assign83370_e125097_d_n13;
        var_migid_dn14 = assign83370_e125097_d_n14;
        var_migid_dn15 = assign83370_e125097_d_n15;
        var_migid_dn16 = assign83370_e125097_d_n16;
        var_migid_dn17 = assign83370_e125097_d_n17;
        var_migid_dn18 = assign83370_e125097_d_n18;
        var_migid_dn19 = assign83370_e125097_d_n19;
        var_migid_dn20 = assign83370_e125097_d_n20;

        let assign83540_e125205: f64 = if (((p.p46 != 0.0) && (var_betnedge_i > 0.0)) && (var_xgedge > 0.0)) { 1.0 } else { 0.0 };
        var_guard2288 = assign83540_e125205;

        let (assign83550_e125213, assign83550_e125213_d_n5, assign83550_e125213_d_n6, assign83550_e125213_d_n7, assign83550_e125213_d_n8, assign83550_e125213_d_n12, assign83550_e125213_d_n13, assign83550_e125213_d_n14, assign83550_e125213_d_n15, assign83550_e125213_d_n16, assign83550_e125213_d_n17, assign83550_e125213_d_n18, assign83550_e125213_d_n19, assign83550_e125213_d_n20,) = {
    if (var_guard2288 != 0.0) {
        let assign83550_e125209: f64 = (4.0 * var_dsqredge);
        let assign83550_e125211: f64 = (assign83550_e125209 / var_gfedge2);
        (assign83550_e125211, ((4.0 * var_dsqredge_dn5) / var_gfedge2), ((4.0 * var_dsqredge_dn6) / var_gfedge2), ((4.0 * var_dsqredge_dn7) / var_gfedge2), ((4.0 * var_dsqredge_dn8) / var_gfedge2), ((4.0 * var_dsqredge_dn12) / var_gfedge2), ((4.0 * var_dsqredge_dn13) / var_gfedge2), ((4.0 * var_dsqredge_dn14) / var_gfedge2), ((4.0 * var_dsqredge_dn15) / var_gfedge2), ((4.0 * var_dsqredge_dn16) / var_gfedge2), ((4.0 * var_dsqredge_dn17) / var_gfedge2), ((4.0 * var_dsqredge_dn18) / var_gfedge2), ((4.0 * var_dsqredge_dn19) / var_gfedge2), ((4.0 * var_dsqredge_dn20) / var_gfedge2),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn12, var_temp1_dn13, var_temp1_dn14, var_temp1_dn15, var_temp1_dn16, var_temp1_dn17, var_temp1_dn18, var_temp1_dn19, var_temp1_dn20,)
    }
};
        var_temp1 = assign83550_e125213;
        var_temp1_dn5 = assign83550_e125213_d_n5;
        var_temp1_dn6 = assign83550_e125213_d_n6;
        var_temp1_dn7 = assign83550_e125213_d_n7;
        var_temp1_dn8 = assign83550_e125213_d_n8;
        var_temp1_dn12 = assign83550_e125213_d_n12;
        var_temp1_dn13 = assign83550_e125213_d_n13;
        var_temp1_dn14 = assign83550_e125213_d_n14;
        var_temp1_dn15 = assign83550_e125213_d_n15;
        var_temp1_dn16 = assign83550_e125213_d_n16;
        var_temp1_dn17 = assign83550_e125213_d_n17;
        var_temp1_dn18 = assign83550_e125213_d_n18;
        var_temp1_dn19 = assign83550_e125213_d_n19;
        var_temp1_dn20 = assign83550_e125213_d_n20;

        let (assign83570_e125233, assign83570_e125233_d_n5, assign83570_e125233_d_n6, assign83570_e125233_d_n7, assign83570_e125233_d_n8, assign83570_e125233_d_n12, assign83570_e125233_d_n13, assign83570_e125233_d_n14, assign83570_e125233_d_n15, assign83570_e125233_d_n16, assign83570_e125233_d_n17, assign83570_e125233_d_n18, assign83570_e125233_d_n19, assign83570_e125233_d_n20,) = {
    if (var_guard2288 != 0.0) {
        let assign83570_e125231: f64 = (var_cox_over_q * var_phit);
        (assign83570_e125231, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn12, var_temp1_dn13, var_temp1_dn14, var_temp1_dn15, var_temp1_dn16, var_temp1_dn17, var_temp1_dn18, var_temp1_dn19, var_temp1_dn20,)
    }
};
        var_temp1 = assign83570_e125233;
        var_temp1_dn5 = assign83570_e125233_d_n5;
        var_temp1_dn6 = assign83570_e125233_d_n6;
        var_temp1_dn7 = assign83570_e125233_d_n7;
        var_temp1_dn8 = assign83570_e125233_d_n8;
        var_temp1_dn12 = assign83570_e125233_d_n12;
        var_temp1_dn13 = assign83570_e125233_d_n13;
        var_temp1_dn14 = assign83570_e125233_d_n14;
        var_temp1_dn15 = assign83570_e125233_d_n15;
        var_temp1_dn16 = assign83570_e125233_d_n16;
        var_temp1_dn17 = assign83570_e125233_d_n17;
        var_temp1_dn18 = assign83570_e125233_d_n18;
        var_temp1_dn19 = assign83570_e125233_d_n19;
        var_temp1_dn20 = assign83570_e125233_d_n20;

        let (assign83700_e125373, assign83700_e125373_d_n5, assign83700_e125373_d_n6, assign83700_e125373_d_n7, assign83700_e125373_d_n8, assign83700_e125373_d_n12, assign83700_e125373_d_n13, assign83700_e125373_d_n14, assign83700_e125373_d_n15, assign83700_e125373_d_n16, assign83700_e125373_d_n17, assign83700_e125373_d_n18, assign83700_e125373_d_n19, assign83700_e125373_d_n20,) = {
    if (var_guard2288 != 0.0) {
        let assign83700_e125371: f64 = (var_alpha_dc * var_h_dc);
        (assign83700_e125371, ((var_alpha_dc_dn5 * var_h_dc) + (var_alpha_dc * var_h_dc_dn5)), ((var_alpha_dc_dn6 * var_h_dc) + (var_alpha_dc * var_h_dc_dn6)), ((var_alpha_dc_dn7 * var_h_dc) + (var_alpha_dc * var_h_dc_dn7)), ((var_alpha_dc_dn8 * var_h_dc) + (var_alpha_dc * var_h_dc_dn8)), ((var_alpha_dc_dn12 * var_h_dc) + (var_alpha_dc * var_h_dc_dn12)), ((var_alpha_dc_dn13 * var_h_dc) + (var_alpha_dc * var_h_dc_dn13)), ((var_alpha_dc_dn14 * var_h_dc) + (var_alpha_dc * var_h_dc_dn14)), ((var_alpha_dc_dn15 * var_h_dc) + (var_alpha_dc * var_h_dc_dn15)), ((var_alpha_dc_dn16 * var_h_dc) + (var_alpha_dc * var_h_dc_dn16)), ((var_alpha_dc_dn17 * var_h_dc) + (var_alpha_dc * var_h_dc_dn17)), ((var_alpha_dc_dn18 * var_h_dc) + (var_alpha_dc * var_h_dc_dn18)), ((var_alpha_dc_dn19 * var_h_dc) + (var_alpha_dc * var_h_dc_dn19)), ((var_alpha_dc_dn20 * var_h_dc) + (var_alpha_dc * var_h_dc_dn20)),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn12, var_temp1_dn13, var_temp1_dn14, var_temp1_dn15, var_temp1_dn16, var_temp1_dn17, var_temp1_dn18, var_temp1_dn19, var_temp1_dn20,)
    }
};
        var_temp1 = assign83700_e125373;
        var_temp1_dn5 = assign83700_e125373_d_n5;
        var_temp1_dn6 = assign83700_e125373_d_n6;
        var_temp1_dn7 = assign83700_e125373_d_n7;
        var_temp1_dn8 = assign83700_e125373_d_n8;
        var_temp1_dn12 = assign83700_e125373_d_n12;
        var_temp1_dn13 = assign83700_e125373_d_n13;
        var_temp1_dn14 = assign83700_e125373_d_n14;
        var_temp1_dn15 = assign83700_e125373_d_n15;
        var_temp1_dn16 = assign83700_e125373_d_n16;
        var_temp1_dn17 = assign83700_e125373_d_n17;
        var_temp1_dn18 = assign83700_e125373_d_n18;
        var_temp1_dn19 = assign83700_e125373_d_n19;
        var_temp1_dn20 = assign83700_e125373_d_n20;

        *var_c_igid_slot = var_c_igid;
        *var_c_igid_dn12_slot = var_c_igid_dn12;
        *var_c_igid_dn13_slot = var_c_igid_dn13;
        *var_c_igid_dn14_slot = var_c_igid_dn14;
        *var_c_igid_dn15_slot = var_c_igid_dn15;
        *var_c_igid_dn16_slot = var_c_igid_dn16;
        *var_c_igid_dn17_slot = var_c_igid_dn17;
        *var_c_igid_dn18_slot = var_c_igid_dn18;
        *var_c_igid_dn19_slot = var_c_igid_dn19;
        *var_c_igid_dn20_slot = var_c_igid_dn20;
        *var_c_igid_dn5_slot = var_c_igid_dn5;
        *var_c_igid_dn6_slot = var_c_igid_dn6;
        *var_c_igid_dn7_slot = var_c_igid_dn7;
        *var_c_igid_dn8_slot = var_c_igid_dn8;
        *var_guard2288_slot = var_guard2288;
        *var_migid_slot = var_migid;
        *var_migid_dn12_slot = var_migid_dn12;
        *var_migid_dn13_slot = var_migid_dn13;
        *var_migid_dn14_slot = var_migid_dn14;
        *var_migid_dn15_slot = var_migid_dn15;
        *var_migid_dn16_slot = var_migid_dn16;
        *var_migid_dn17_slot = var_migid_dn17;
        *var_migid_dn18_slot = var_migid_dn18;
        *var_migid_dn19_slot = var_migid_dn19;
        *var_migid_dn20_slot = var_migid_dn20;
        *var_migid_dn5_slot = var_migid_dn5;
        *var_migid_dn6_slot = var_migid_dn6;
        *var_migid_dn7_slot = var_migid_dn7;
        *var_migid_dn8_slot = var_migid_dn8;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn12_slot = var_temp1_dn12;
        *var_temp1_dn13_slot = var_temp1_dn13;
        *var_temp1_dn14_slot = var_temp1_dn14;
        *var_temp1_dn15_slot = var_temp1_dn15;
        *var_temp1_dn16_slot = var_temp1_dn16;
        *var_temp1_dn17_slot = var_temp1_dn17;
        *var_temp1_dn18_slot = var_temp1_dn18;
        *var_temp1_dn19_slot = var_temp1_dn19;
        *var_temp1_dn20_slot = var_temp1_dn20;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[990] = (p.p37 >= 0.0);
        s.store_scalar(990, if s.b[990] { 1.0 } else { 0.0 });

        if s.b[990] {
            s.store_scalar(0, 1.0);
        }

        if (!s.b[990]) {
            s.store_scalar(0, (-1.0));
        }

        s.store_scalar(767, (8.8541878176e-12 * 11.8));

        s.b[991] = (p.p51 < 0.5);
        s.store_scalar(991, if s.b[991] { 1.0 } else { 0.0 });

        if s.b[991] {
            s.store_scalar(1, 0.0);
        }

        s.b[992] = (p.p51 < 1.5);
        s.store_scalar(992, if s.b[992] { 1.0 } else { 0.0 });

        if ((!s.b[991]) && s.b[992]) {
            s.store_scalar(1, 1.0);
        }

        s.b[993] = (p.p51 < 2.5);
        s.store_scalar(993, if s.b[993] { 1.0 } else { 0.0 });

        if (((!s.b[991]) && (!s.b[992])) && s.b[993]) {
            s.store_scalar(1, 2.0);
        }

        s.b[994] = (p.p51 < 4.0);
        s.store_scalar(994, if s.b[994] { 1.0 } else { 0.0 });

        if ((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && s.b[994]) {
            s.store_scalar(1, 3.0);
        }

        s.b[995] = (p.p51 < 7.0);
        s.store_scalar(995, if s.b[995] { 1.0 } else { 0.0 });

        if (((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && (!s.b[994])) && s.b[995]) {
            s.store_scalar(1, 5.0);
        }

        if (((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && (!s.b[994])) && (!s.b[995])) {
            s.store_scalar(1, 9.0);
        }

        s.store_scalar(3, 10.0);

        s.store_scalar(4, (1.0 / s.v[3]));

        s.store_scalar(350, (273.15 + p.p38));

        s.store_scalar(474, 0.0);

        s.b[996] = (p.p927 > 0.5);
        s.store_scalar(996, if s.b[996] { 1.0 } else { 0.0 });

        if s.b[996] {
            s.store_scalar(474, 1.0);
        }

        if (!s.b[996]) {
            s.store_scalar(474, 0.0);
        }

        s.store_scalar(364, (273.15 + p.p823));

        s.store_scalar(367, (1.3806505e-23 / 1.6021918e-19));

        s.store_scalar(368, (s.v[367] * s.v[364]));

        s.store_scalar(369, (1.0 / s.v[368]));

        s.store_scalar(375, ((-((0.000702 * s.v[364]) * s.v[364])) / (1108.0 + s.v[364])));

        s.store_scalar(378, (p.p834 + s.v[375]));

        s.store_scalar(379, (p.p835 + s.v[375]));

        s.store_scalar(380, (p.p836 + s.v[375]));

        s.store_scalar(408, (1.0 - p.p831));

        s.store_scalar(409, (1.0 - p.p832));

        s.store_scalar(410, (1.0 - p.p833));

        s.store_scalar(411, (1.0 / s.v[408]));

        s.store_scalar(412, (1.0 / s.v[409]));

        s.store_scalar(413, (1.0 / s.v[410]));

        s.store_scalar(423, (s.v[767] / p.p825));

        s.store_scalar(424, ((p.p843 * s.v[767]) / p.p826));

        s.store_scalar(425, ((p.p844 * s.v[767]) / p.p827));

        s.store_scalar(426, (1.0 / s.v[423]));

        s.store_scalar(427, (1.0 / s.v[424]));

        s.store_scalar(428, (1.0 / s.v[425]));

        s.store_scalar(429, (1.0 / p.p828));

        s.store_scalar(430, (1.0 / p.p829));

        s.store_scalar(431, (1.0 / p.p830));

        s.store_scalar(444, (1.0 - (1.0 / p.p824)));

        s.store_scalar(448, (1.0 / p.p860));

        s.store_scalar(449, (1.0 / p.p861));

        s.store_scalar(450, (1.0 / p.p862));

        s.b[997] = ((((p.p866 != 1.0) || (p.p867 != 1.0)) || (p.p868 != 1.0)) || (p.p869 != 1.0));
        s.store_scalar(997, if s.b[997] { 1.0 } else { 0.0 });

        if s.b[997] {
            s.store_scalar(473, 1.0);
        }

        if (!s.b[997]) {
            s.store_scalar(473, 0.0);
        }

        s.b[998] = (s.v[473] == 1.0);
        s.store_scalar(998, if s.b[998] { 1.0 } else { 0.0 });

        if s.b[998] {
            s.store_scalar(457, (if ((p.p827 * p.p866) > 1e-18) { (p.p827 * p.p866) } else { 1e-18 }));
        }

        if s.b[998] {
            s.store_scalar(458, (if ((p.p830 * p.p867) > 0.05) { (p.p830 * p.p867) } else { 0.05 }));
        }

        if s.b[998] {
            s.store_scalar(459, (if ((if ((p.p833 * p.p868) > 0.05) { (p.p833 * p.p868) } else { 0.05 }) < 0.95) { (if ((p.p833 * p.p868) > 0.05) { (p.p833 * p.p868) } else { 0.05 }) } else { 0.95 }));
        }

        if s.b[998] {
            s.store_scalar(460, (p.p836 * p.p869));
            s.store_offset(462, 460, s.v[375]);
            s.store_sub_from_scalar(467, 1.0, 459);
            s.store_div_from_scalar(468, 1.0, 467);
        }

        s.b[999] = (p.p44 == 0.0);
        s.store_scalar(999, if s.b[999] { 1.0 } else { 0.0 });

        if s.b[999] {
            s.store_scalar(505, p.p825);
            s.store_scalar(506, p.p826);
            s.store_scalar(507, p.p827);
            s.store_scalar(508, p.p828);
            s.store_scalar(509, p.p829);
            s.store_scalar(510, p.p830);
            s.store_scalar(511, p.p831);
            s.store_scalar(512, p.p832);
            s.store_scalar(513, p.p833);
            s.store_scalar(514, p.p834);
            s.store_scalar(515, p.p835);
            s.store_scalar(516, p.p836);
            s.store_scalar(517, p.p837);
            s.store_scalar(518, p.p838);
            s.store_scalar(519, p.p839);
            s.store_scalar(522, p.p840);
            s.store_scalar(523, p.p841);
            s.store_scalar(524, p.p842);
            s.store_scalar(520, p.p843);
            s.store_scalar(521, p.p844);
            s.store_scalar(525, p.p845);
            s.store_scalar(526, p.p846);
            s.store_scalar(527, p.p847);
            s.store_scalar(528, p.p848);
            s.store_scalar(529, p.p849);
            s.store_scalar(530, p.p850);
            s.store_scalar(531, p.p851);
            s.store_scalar(532, p.p852);
            s.store_scalar(533, p.p853);
            s.store_scalar(534, p.p854);
            s.store_scalar(535, p.p855);
            s.store_scalar(536, p.p856);
            s.store_scalar(537, p.p857);
            s.store_scalar(538, p.p858);
            s.store_scalar(539, p.p859);
            s.store_scalar(540, p.p860);
            s.store_scalar(541, p.p861);
            s.store_scalar(542, p.p862);
            s.store_scalar(543, p.p863);
            s.store_scalar(544, p.p864);
            s.store_scalar(545, p.p865);
            s.store_scalar(553, p.p929);
            s.store_scalar(636, p.p872);
            s.store_scalar(637, p.p873);
            s.store_scalar(638, p.p874);
            s.store_scalar(639, p.p875);
            s.store_scalar(546, p.p866);
            s.store_scalar(547, p.p867);
            s.store_scalar(548, p.p868);
            s.store_scalar(549, p.p869);
            s.store_scalar(550, p.p870);
            s.store_scalar(551, p.p871);
        }

        if (!s.b[999]) {
            s.store_scalar(505, p.p876);
            s.store_scalar(506, p.p877);
            s.store_scalar(507, p.p878);
            s.store_scalar(508, p.p879);
            s.store_scalar(509, p.p880);
            s.store_scalar(510, p.p881);
            s.store_scalar(511, p.p882);
            s.store_scalar(512, p.p883);
            s.store_scalar(513, p.p884);
            s.store_scalar(514, p.p885);
            s.store_scalar(515, p.p886);
            s.store_scalar(516, p.p887);
            s.store_scalar(517, p.p888);
            s.store_scalar(518, p.p889);
            s.store_scalar(519, p.p890);
            s.store_scalar(522, p.p891);
            s.store_scalar(523, p.p892);
            s.store_scalar(524, p.p893);
            s.store_scalar(520, p.p894);
            s.store_scalar(521, p.p895);
            s.store_scalar(525, p.p896);
            s.store_scalar(526, p.p897);
            s.store_scalar(527, p.p898);
            s.store_scalar(528, p.p899);
            s.store_scalar(529, p.p900);
            s.store_scalar(530, p.p901);
            s.store_scalar(531, p.p902);
            s.store_scalar(532, p.p903);
            s.store_scalar(533, p.p904);
            s.store_scalar(534, p.p905);
            s.store_scalar(535, p.p906);
            s.store_scalar(536, p.p907);
            s.store_scalar(537, p.p908);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (!s.b[999]) {
            s.store_scalar(538, p.p909);
            s.store_scalar(539, p.p910);
            s.store_scalar(540, p.p911);
            s.store_scalar(541, p.p912);
            s.store_scalar(542, p.p913);
            s.store_scalar(543, p.p914);
            s.store_scalar(544, p.p915);
            s.store_scalar(545, p.p916);
            s.store_scalar(553, p.p931);
            s.store_scalar(636, p.p923);
            s.store_scalar(637, p.p924);
            s.store_scalar(638, p.p925);
            s.store_scalar(639, p.p926);
            s.store_scalar(546, p.p917);
            s.store_scalar(547, p.p918);
            s.store_scalar(548, p.p919);
            s.store_scalar(549, p.p920);
            s.store_scalar(550, p.p921);
            s.store_scalar(551, p.p922);
        }

        s.store_offset(554, 514, s.v[375]);

        s.store_offset(555, 515, s.v[375]);

        s.store_offset(556, 516, s.v[375]);

        s.store_sub_from_scalar(575, 1.0, 511);

        s.store_sub_from_scalar(576, 1.0, 512);

        s.store_sub_from_scalar(577, 1.0, 513);

        s.store_div_from_scalar(578, 1.0, 575);

        s.store_div_from_scalar(579, 1.0, 576);

        s.store_div_from_scalar(580, 1.0, 577);

        s.store_div_from_scalar(590, s.v[767], 505);

        s.store_div_scaled_inputs_indices(591, 520, s.v[767], 506, 1.0);

        s.store_div_scaled_inputs_indices(592, 521, s.v[767], 507, 1.0);

        s.store_div_from_scalar(593, 1.0, 590);

        s.store_div_from_scalar(594, 1.0, 591);

        s.store_div_from_scalar(595, 1.0, 592);

        s.store_div_from_scalar(596, 1.0, 508);

        s.store_div_from_scalar(597, 1.0, 509);

        s.store_div_from_scalar(598, 1.0, 510);

        s.store_div_from_scalar(614, 1.0, 540);

        s.store_div_from_scalar(615, 1.0, 541);

        s.store_div_from_scalar(616, 1.0, 542);

        s.b[1000] = ((((s.v[546] != 1.0) || (s.v[547] != 1.0)) || (s.v[548] != 1.0)) || (s.v[549] != 1.0));
        s.store_scalar(1000, if s.b[1000] { 1.0 } else { 0.0 });

        if s.b[1000] {
            s.store_scalar(635, 1.0);
        }

        if (!s.b[1000]) {
            s.store_scalar(635, 0.0);
        }

        s.b[1001] = (s.v[635] == 1.0);
        s.store_scalar(1001, if s.b[1001] { 1.0 } else { 0.0 });

        if s.b[1001] {
            if ((s.v[507] * s.v[546]) > 1e-18) {
                s.store_mul(620, 507, 546);
            } else {
                s.store_scalar(620, 1e-18);
            }
        }

        if s.b[1001] {
            if ((s.v[510] * s.v[547]) > 0.05) {
                s.store_mul(621, 510, 547);
            } else {
                s.store_scalar(621, 0.05);
            }
        }

        if s.b[1001] {
            if ((if ((s.v[513] * s.v[548]) > 0.05) { (s.v[513] * s.v[548]) } else { 0.05 }) < 0.95) {
                if ((s.v[513] * s.v[548]) > 0.05) {
                    s.store_mul(622, 513, 548);
                } else {
                    s.store_scalar(622, 0.05);
                }
            } else {
                s.store_scalar(622, 0.95);
            }
        }

        if s.b[1001] {
            s.store_mul(623, 516, 549);
            s.store_offset(625, 623, s.v[375]);
            s.store_sub_from_scalar(630, 1.0, 622);
            s.store_div_from_scalar(631, 1.0, 630);
        }

        s.store_scalar(351, ((ctx_temp + p.p56) + p.p35));

        s.store_scalar(352, (s.v[351] / s.v[350]));

        s.store_scalar(353, (s.v[351] - s.v[350]));

        s.store_scalar(354, ((s.v[351] * 1.3806505e-23) / 1.6021918e-19));

        s.store_scalar(355, (1.0 / s.v[354]));

        s.store_scalar(356, s.v[351]);

        s.store_scalar(357, (s.v[356] * s.v[356]));

        s.store_scalar(358, (s.v[356] - s.v[350]));

        s.store_scalar(359, (s.v[350] / s.v[356]));

        s.store_scalar(360, ((s.v[359]) as f64).ln());

        s.store_scalar(715, ((s.v[356] * 1.3806505e-23) / 1.6021918e-19));

        s.store_scalar(361, (1.0 / s.v[715]));

        s.store_scalar(362, ((1.179 - (9.025e-5 * s.v[356])) - (3.05e-7 * s.v[357])));

        s.store_scalar(363, ((((1.045 + (0.00045 * s.v[356])) * ((0.523 + (0.0014 * s.v[356])) - (1.48e-6 * s.v[357]))) * s.v[357]) / 90000.0));

        if (!(s.v[363] > 0.001)) {
            s.store_scalar(363, 0.001);
        }

        s.store_scalar(365, (((ctx_temp + p.p56) + p.p35)).max((273.15 + (-250.0))));

        s.store_scalar(366, (s.v[365] / s.v[364]));

        s.store_scalar(370, (s.v[367] * s.v[365]));

        s.store_scalar(371, (1.0 / s.v[370]));

        s.store_scalar(376, ((-((0.000702 * s.v[365]) * s.v[365])) / (1108.0 + s.v[365])));

        s.store_scalar(381, (p.p834 + s.v[376]));

        s.store_scalar(382, (p.p835 + s.v[376]));

        s.store_scalar(383, (p.p836 + s.v[376]));

        s.store_scalar(384, (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[378] * s.v[369]) - (s.v[381] * s.v[371])))) as f64).exp()));

        s.store_scalar(385, (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[379] * s.v[369]) - (s.v[382] * s.v[371])))) as f64).exp()));

        s.store_scalar(386, (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[380] * s.v[369]) - (s.v[383] * s.v[371])))) as f64).exp()));

        s.store_scalar(387, ((p.p837 * s.v[384]) * s.v[384]));

        s.store_scalar(388, ((p.p838 * s.v[385]) * s.v[385]));

        s.store_scalar(389, ((p.p839 * s.v[386]) * s.v[386]));

        s.store_scalar(390, ((p.p828 * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[384]) as f64).ln())));

        s.store_scalar(391, ((p.p829 * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[385]) as f64).ln())));

        s.store_scalar(392, ((p.p830 * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[386]) as f64).ln())));

        s.store_scalar(393, (s.v[390] + (s.v[370] * (((1.0 + ((((0.05 - s.v[390]) * s.v[371])) as f64).exp())) as f64).ln())));

        s.store_scalar(394, (s.v[391] + (s.v[370] * (((1.0 + ((((0.05 - s.v[391]) * s.v[371])) as f64).exp())) as f64).ln())));

        s.store_scalar(395, (s.v[392] + (s.v[370] * (((1.0 + ((((0.05 - s.v[392]) * s.v[371])) as f64).exp())) as f64).ln())));

        s.store_scalar(405, (1.0 / s.v[393]));

        s.store_scalar(406, (1.0 / s.v[394]));

        s.store_scalar(407, (1.0 / s.v[395]));

        s.store_scalar(414, (p.p825 * (((p.p828 * s.v[405])) as f64).powf(p.p831)));

        s.store_scalar(415, (p.p826 * (((p.p829 * s.v[406])) as f64).powf(p.p832)));

        s.store_scalar(416, (p.p827 * (((p.p830 * s.v[407])) as f64).powf(p.p833)));

        s.store_scalar(417, ((s.v[414] * s.v[393]) * s.v[411]));

        s.store_scalar(418, ((s.v[415] * s.v[394]) * s.v[412]));

        s.store_scalar(419, ((s.v[416] * s.v[395]) * s.v[413]));

        s.store_scalar(420, (2.0 * s.v[414]));

        s.store_scalar(421, (2.0 * s.v[415]));

        s.store_scalar(422, (2.0 * s.v[416]));

        s.store_scalar(432, ((0.5 * s.v[381])).max(s.v[370]));

        s.store_scalar(433, ((0.5 * s.v[382])).max(s.v[370]));

        s.store_scalar(434, ((0.5 * s.v[383])).max(s.v[370]));

        s.store_scalar(435, (s.v[432] * s.v[371]));

        s.store_scalar(436, (s.v[433] * s.v[371]));

        s.store_scalar(437, (s.v[434] * s.v[371]));

        s.store_scalar(438, (((((((32.0 * p.p848) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[432] * s.v[432]) * s.v[432]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));

        s.store_scalar(439, (((((((32.0 * p.p849) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[433] * s.v[433]) * s.v[433]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));

        s.store_scalar(440, (((((((32.0 * p.p850) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[434] * s.v[434]) * s.v[434]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));

        s.store_scalar(441, (p.p854 * (1.0 + (p.p857 * (s.v[365] - s.v[364])))));

        s.store_scalar(442, (p.p855 * (1.0 + (p.p858 * (s.v[365] - s.v[364])))));

        s.store_scalar(443, (p.p856 * (1.0 + (p.p859 * (s.v[365] - s.v[364])))));

        if (!(s.v[441] > 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if (!(s.v[442] > 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (!(s.v[443] > 0.0)) {
            s.store_scalar(443, 0.0);
        }

        s.b[1021] = (s.v[473] == 1.0);
        s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });

        if s.b[1021] {
            s.store_offset(461, 460, s.v[376]);
            s.store_scale_ad(463, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(462), s.v[369], s.ad_value(461), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(464, 458, s.v[366], A::ln(s.ad_value(463)), (2.0 * s.v[370]));
            s.store_add_scaled_inputs_ad_rhs(465, 464, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(464), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);
            s.store_div_from_scalar(466, 1.0, 465);
            s.store_mul_pow_ad_rhs(469, 457, A::mul(s.ad_value(458), s.ad_value(466)), s.ad_value(459));
            s.store_mul3_lhs(470, 469, 465, 468);
            s.store_scale(471, 469, 2.0);
        }

        s.store_offset(557, 514, s.v[376]);

        s.store_offset(558, 515, s.v[376]);

        s.store_offset(559, 516, s.v[376]);

        s.store_scale_ad(560, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(554), s.v[369], s.ad_value(557), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));

        s.store_scale_ad(561, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(555), s.v[369], s.ad_value(558), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));

        s.store_scale_ad(562, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(556), s.v[369], s.ad_value(559), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));

        s.store_mul3_lhs(563, 517, 560, 560);

        s.store_mul3_lhs(564, 518, 561, 561);

        s.store_mul3_lhs(565, 519, 562, 562);

        s.store_sub_scaled_inputs_ad_rhs(566, 508, s.v[366], A::ln(s.ad_value(560)), (2.0 * s.v[370]));

        s.store_sub_scaled_inputs_ad_rhs(567, 509, s.v[366], A::ln(s.ad_value(561)), (2.0 * s.v[370]));

        s.store_sub_scaled_inputs_ad_rhs(568, 510, s.v[366], A::ln(s.ad_value(562)), (2.0 * s.v[370]));

        s.store_add_scaled_inputs_ad_rhs(569, 566, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(566), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);

        s.store_add_scaled_inputs_ad_rhs(570, 567, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(567), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);

        s.store_add_scaled_inputs_ad_rhs(571, 568, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(568), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);

        s.store_div_from_scalar(572, 1.0, 569);

        s.store_div_from_scalar(573, 1.0, 570);

        s.store_div_from_scalar(574, 1.0, 571);

        s.store_mul_pow_ad_rhs(581, 505, A::mul(s.ad_value(508), s.ad_value(572)), s.ad_value(511));

        s.store_mul_pow_ad_rhs(582, 506, A::mul(s.ad_value(509), s.ad_value(573)), s.ad_value(512));

        s.store_mul_pow_ad_rhs(583, 507, A::mul(s.ad_value(510), s.ad_value(574)), s.ad_value(513));

        s.store_mul3_lhs(584, 581, 569, 578);

        s.store_mul3_lhs(585, 582, 570, 579);

        s.store_mul3_lhs(586, 583, 571, 580);

        s.store_scale(587, 581, 2.0);

        s.store_scale(588, 582, 2.0);

        s.store_scale(589, 583, 2.0);

        s.store_max_with_scalar_ad(599, A::scale(s.ad_value(557), 0.5), s.v[370]);

        s.store_max_with_scalar_ad(600, A::scale(s.ad_value(558), 0.5), s.v[370]);

        s.store_max_with_scalar_ad(601, A::scale(s.ad_value(559), 0.5), s.v[370]);

        s.store_scale(602, 599, s.v[371]);

        s.store_scale(603, 600, s.v[371]);

        s.store_scale(604, 601, s.v[371]);

        s.store_scaled_sqrt_ad(605, A::mul3_scaled_output(s.ad_value(528), A::square(s.ad_value(599)), s.ad_value(599), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(606, A::mul3_scaled_output(s.ad_value(529), A::square(s.ad_value(600)), s.ad_value(600), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(607, A::mul3_scaled_output(s.ad_value(530), A::square(s.ad_value(601)), s.ad_value(601), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_mul_scale_offset_rhs(608, 534, 537, (s.v[365] - s.v[364]), 1.0);

        s.store_mul_scale_offset_rhs(609, 535, 538, (s.v[365] - s.v[364]), 1.0);

        s.store_mul_scale_offset_rhs(610, 536, 539, (s.v[365] - s.v[364]), 1.0);

        if (!(s.v[608] > 0.0)) {
            s.store_scalar(608, 0.0);
        }

        if (!(s.v[609] > 0.0)) {
            s.store_scalar(609, 0.0);
        }

        if (!(s.v[610] > 0.0)) {
            s.store_scalar(610, 0.0);
        }

        s.b[1022] = (s.v[635] == 1.0);
        s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });

        if s.b[1022] {
            s.store_offset(624, 623, s.v[376]);
            s.store_scale_ad(626, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(625), s.v[369], s.ad_value(624), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(627, 621, s.v[366], A::ln(s.ad_value(626)), (2.0 * s.v[370]));
            s.store_add_scaled_inputs_ad_rhs(628, 627, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(627), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);
            s.store_div_from_scalar(629, 1.0, 628);
            s.store_mul_pow_ad_rhs(632, 620, A::mul(s.ad_value(621), s.ad_value(629)), s.ad_value(622));
            s.store_mul3_lhs(633, 632, 628, 631);
            s.store_scale(634, 632, 2.0);
        }

        s.store_scalar(5, 1.0);

        s.store_scalar(6, 1.0);

        s.store_scalar(312, 0.0);

        s.store_scalar(313, 0.0);

        s.store_scalar(7, p.p0);

        s.store_scalar(8, p.p1);

        s.store_scalar(9, p.p2);

        s.store_scalar(10, p.p3);

        s.store_scalar(11, p.p4);

        s.store_scalar(12, p.p8);

        s.store_scalar(646, p.p19);

        s.store_scalar(647, p.p20);

        s.store_scalar(648, p.p21);

        s.store_scalar(673, p.p22);

        s.store_scalar(674, p.p23);

        s.store_scalar(675, p.p24);

        s.store_scalar(649, p.p25);

        s.store_scalar(650, p.p26);

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_scalar(676, p.p27);

        s.store_scalar(677, p.p28);

        s.store_scalar(14, p.p14);

        s.b[1023] = (p.p39 > 0.0);
        s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });

        if s.b[1023] {
            s.store_scalar(5, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));
        }

        if s.b[1023] {
            s.store_floor_ad(5, A::offset(s.ad_value(5), 0.5));
            s.store_div_from_scalar(6, 1.0, 5);
        }

        if ((s.v[8] * s.v[6]) > 1e-9) {
            s.store_scale(8, 6, s.v[8]);
        } else {
            s.store_scalar(8, 1e-9);
        }

        s.store_scalar(15, p.p5);

        s.store_scalar(16, p.p6);

        s.store_scalar(17, p.p7);

        s.store_scalar(308, (1e-6 / s.v[7]));

        s.store_div_from_scalar(309, 1e-6, 8);

        s.store_offset_scaled(310, 309, ((p.p190) * ((p.p188 * (1.0 + (p.p189 * s.v[308]))))), (p.p188 * (1.0 + (p.p189 * s.v[308]))));

        s.store_offset_scaled(311, 309, ((p.p194) * ((p.p192 * (1.0 + (p.p193 * s.v[308]))))), (p.p192 * (1.0 + (p.p193 * s.v[308]))));

        if (((s.v[7] + s.v[310]) - (2.0 * p.p191)) > 1e-9) {
            s.store_offset(312, 310, ((s.v[7]) + ((-(2.0 * p.p191)))));
        } else {
            s.store_scalar(312, 1e-9);
        }

        if (((s.v[8] + s.v[311]) - (2.0 * p.p195)) > 1e-9) {
            s.store_offset_add(313, 8, 311, (-(2.0 * p.p195)));
        } else {
            s.store_scalar(313, 1e-9);
        }

        s.store_div_from_scalar(314, 1e-6, 312);

        s.store_square(315, 314);

        s.store_div_from_scalar(316, 1e-6, 313);

        s.store_div_from_scalar(317, 1.0, 316);

        s.store_mul(318, 314, 316);

        s.store_div_from_scalar(319, 1.0, 318);

        if ((((s.v[7] + s.v[310]) - (2.0 * p.p191)) + p.p196) > 1e-9) {
            s.store_offset(320, 310, ((((s.v[7]) + ((-(2.0 * p.p191))))) + (p.p196)));
        } else {
            s.store_scalar(320, 1e-9);
        }

        if ((((s.v[8] + s.v[311]) - (2.0 * p.p195)) + p.p197) > 1e-9) {
            s.store_offset_add(321, 8, 311, (((-(2.0 * p.p195))) + (p.p197)));
        } else {
            s.store_scalar(321, 1e-9);
        }

        s.store_scale(322, 321, 1000000.0);

        if (((s.v[7] + s.v[310]) + p.p196) > 1e-9) {
            s.store_offset(323, 310, ((s.v[7]) + (p.p196)));
        } else {
            s.store_scalar(323, 1e-9);
        }

        if (((s.v[8] + s.v[311]) + p.p197) > 1e-9) {
            s.store_offset_add(324, 8, 311, p.p197);
        } else {
            s.store_scalar(324, 1e-9);
        }

        s.store_scale(325, 323, 1000000.0);

        s.store_scale(326, 324, 1000000.0);

        s.store_scalar(44, p.p57);

        s.store_scalar(45, p.p58);

        s.store_scalar(46, p.p59);

        s.store_scalar(47, p.p60);

        s.store_scalar(48, p.p61);

        s.store_scalar(49, p.p62);

        s.store_scalar(50, p.p63);

        s.store_scalar(51, p.p64);

        s.store_scalar(52, p.p65);

        s.store_scalar(53, p.p66);

        s.store_scalar(54, p.p67);

        s.store_scalar(59, p.p68);

        s.store_scalar(60, p.p69);

        s.store_scalar(61, p.p70);

        s.store_scalar(62, p.p71);

        s.store_scalar(55, p.p72);

        s.store_scalar(56, p.p74);

        s.store_scalar(57, p.p73);

        s.store_scalar(58, p.p75);

        s.store_scalar(63, p.p79);

        s.store_scalar(64, p.p81);

        s.store_scalar(65, p.p80);

        s.store_scalar(66, p.p76);

        s.store_scalar(67, p.p78);

        s.store_scalar(68, p.p77);

        s.store_scalar(69, p.p82);

        s.store_scalar(70, p.p83);

        s.store_scalar(71, p.p84);

        s.store_scalar(72, p.p85);

        s.store_scalar(73, p.p86);

        s.store_scalar(74, p.p87);

        s.store_scalar(75, p.p88);

        s.store_scalar(76, p.p89);

        s.store_scalar(77, p.p90);

        s.store_scalar(78, p.p91);

        s.store_scalar(79, p.p92);

        s.store_scalar(80, p.p93);

        s.store_scalar(81, p.p94);

        s.store_scalar(82, p.p95);

        s.store_scalar(83, p.p96);

        s.store_scalar(84, p.p97);

        s.store_scalar(85, p.p98);

        s.store_scalar(86, p.p99);

        s.store_scalar(87, p.p100);

        s.store_scalar(88, p.p101);

        s.store_scalar(89, p.p102);

        s.store_scalar(90, p.p103);

        s.store_scalar(91, p.p104);

        s.store_scalar(92, p.p105);

        s.store_scalar(93, p.p106);

        s.store_scalar(94, p.p107);

        s.store_scalar(95, p.p108);

        s.store_scalar(96, p.p109);

        s.store_scalar(97, p.p110);

        s.store_scalar(98, p.p111);

        s.store_scalar(99, p.p112);

        s.store_scalar(100, p.p113);

        s.store_scalar(101, p.p114);

        s.store_scalar(102, p.p115);

        s.store_scalar(103, p.p116);

        s.store_scalar(104, p.p117);

        s.store_scalar(105, p.p118);

        s.store_scalar(106, p.p119);

        s.store_scalar(107, p.p120);

        s.store_scalar(108, p.p121);

        s.store_scalar(109, p.p120);

        s.b[1024] = param_given[122];
        s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });

        if s.b[1024] {
            s.store_scalar(109, p.p122);
        }

        s.store_scalar(110, p.p121);

        s.b[1025] = param_given[123];
        s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });

        if s.b[1025] {
            s.store_scalar(110, p.p123);
        }

        s.copy_ad(111, 109);

        s.b[1026] = param_given[124];
        s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });

        if s.b[1026] {
            s.store_scalar(111, p.p124);
        }

        s.copy_ad(112, 110);

        s.b[1027] = param_given[125];
        s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });

        if s.b[1027] {
            s.store_scalar(112, p.p125);
        }

        s.store_scalar(113, p.p126);

        s.store_scalar(114, p.p127);

        s.store_scalar(115, p.p128);

        s.store_scalar(116, p.p129);

        s.store_scalar(117, p.p130);

        s.store_scalar(118, p.p131);

        s.store_scalar(119, p.p132);

        s.store_scalar(120, p.p133);

        s.store_scalar(121, p.p134);

        s.store_scalar(122, p.p135);

        s.store_scalar(123, p.p136);

        s.store_scalar(124, p.p137);

        s.store_scalar(125, p.p99);

        s.b[1028] = param_given[138];
        s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });

        if s.b[1028] {
            s.store_scalar(125, p.p138);
        }

        s.store_scalar(126, p.p104);

        s.b[1029] = param_given[139];
        s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });

        if s.b[1029] {
            s.store_scalar(126, p.p139);
        }

        s.store_scalar(127, p.p140);

        s.store_scalar(128, p.p141);

        s.store_scalar(129, p.p142);

        s.store_scalar(130, p.p143);

        s.store_scalar(131, p.p144);

        s.store_scalar(132, p.p145);

        s.store_scalar(133, p.p146);

        s.store_scalar(134, p.p147);

        s.store_scalar(135, p.p148);

        s.store_scalar(136, p.p149);

        s.store_scalar(137, p.p150);

        s.store_scalar(138, p.p151);

        s.store_scalar(139, p.p152);

        s.store_scalar(140, p.p153);

        s.store_scalar(141, p.p154);

        s.store_scalar(142, p.p155);

        s.store_scalar(143, p.p156);

        s.store_scalar(149, p.p162);

        s.store_scalar(150, p.p163);

        s.store_scalar(151, p.p164);

        s.store_scalar(152, p.p165);

        s.store_scalar(153, p.p166);

        s.store_scalar(154, p.p167);

        s.store_scalar(155, p.p168);

        s.store_scalar(156, p.p169);

        s.store_scalar(157, p.p170);

        s.store_scalar(158, p.p171);

        s.store_scalar(159, p.p172);

        s.store_scalar(160, p.p174);

        s.store_scalar(161, p.p173);

        s.store_scalar(176, p.p187);

        s.b[1030] = (p.p39 > 0.0);
        s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });

        if s.b[1030] {
            s.store_add_scaled_inputs3_offset_mixed_aii(44, A::powf(s.ad_value(314), p.p200), p.p199, 316, p.p201, 318, p.p202, p.p198);
            s.store_add_scaled_inputs3_offset_indices(45, 314, p.p204, 316, p.p205, 318, p.p206, p.p203);
            s.store_scalar(46, p.p207);
            s.store_scalar(47, p.p208);
            s.store_scalar(48, p.p209);
        }

        if s.b[1030] {
            s.store_scale_ad(331, {
                if ((1.0 + ((p.p211 * s.v[316]) * (((1.0 + (s.v[313] / p.p212))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p211, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p212), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p210);
        }

        if s.b[1030] {
            s.store_scale_ad(332, {
                if ((1.0 + ((p.p214 * s.v[316]) * (((1.0 + (s.v[313] / p.p215))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p214, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p215), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p213);
        }

        if s.b[1030] {
            s.store_scale_ad(333, {
                if ((1.0 + ((p.p217 * s.v[316]) * (((1.0 + (s.v[313] / p.p215))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p217, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p215), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p216);
        }

        s.b[1031] = (s.v[312] > (2.0 * s.v[333]));
        s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1031]) {
            s.store_scalar(334, 75000000000.0);
            s.store_sub_ad(335, A::sqrt(A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(332), 0.5)), A::sqrt(s.ad_value(331)));
            s.store_add_scaled_product_mixed_aia(336, A::sqrt(s.ad_value(331)), 1.0, 334, A::ln(A::offset(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(333), 2.0, s.ad_value(312), 1.0), A::exp(A::div(s.ad_value(335), s.ad_value(334))), (-1.0)), 1.0)), 1.0);
            s.store_square(336, 336);
        }

        s.b[1032] = (s.v[312] >= s.v[333]);
        s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });

        if ((s.b[1030] && (!s.b[1031])) && s.b[1032]) {
            s.store_add_ad_rhs(336, 331, A::div_scaled_product(s.ad_value(332), s.ad_value(333), 1.0, s.ad_value(312), 1.0));
        }

        if ((s.b[1030] && (!s.b[1031])) && (!s.b[1032])) {
            s.store_add_ad_rhs(336, 331, A::mul_sub_from_scalar_rhs(s.ad_value(332), 2.0, A::div(s.ad_value(312), s.ad_value(333))));
        }

        if s.b[1030] {
            s.store_mul_sub_scaled_inputs_rhs(49, 336, A::sub_from_scalar(1.0, A::scale(s.ad_value(314), p.p218)), 1.0, s.ad_value(315), p.p219);
            s.store_add_scaled_inputs3_offset_mixed_aii(50, A::powf(s.ad_value(314), p.p222), p.p221, 316, p.p223, 318, p.p224, p.p220);
            s.store_scalar(51, p.p225);
            s.store_scalar(52, p.p226);
            s.store_add_scaled_inputs3_offset_mixed_aii(53, A::powf(s.ad_value(314), p.p229), p.p228, 316, p.p230, 318, p.p231, p.p227);
        }

        if s.b[1030] {
            s.store_scale_ad(54, {
                if (1e-6 > (1.0 + (p.p233 * s.v[314]))) {
                    A::constant(1e-6)
                } else {
                    A::scale_offset(s.ad_value(314), p.p233, 1.0)
                }
            }, p.p232);
        }

        if s.b[1030] {
            s.store_scalar(59, p.p234);
            s.store_scalar(60, p.p235);
            s.store_scalar(61, p.p238);
            s.store_scalar(62, p.p239);
            s.store_mul3_ad(55, A::scale_offset(A::powf(s.ad_value(314), p.p242), p.p241, p.p240), A::scale_offset(s.ad_value(316), p.p243, 1.0), A::scale_offset(s.ad_value(318), p.p244, 1.0));
            s.store_scalar(56, p.p246);
            s.store_scalar(57, p.p245);
            s.store_scalar(58, p.p247);
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1030] {
            s.store_scaled_mul_scale_offset_rhs_ad(66, A::powf(s.ad_value(314), p.p249), 316, p.p250, 1.0, p.p248);
            s.store_scalar(67, p.p252);
            s.store_scalar(68, p.p251);
            s.store_scaled_mul_scale_offset_rhs_ad(63, A::powf(s.ad_value(314), p.p254), 316, p.p255, 1.0, p.p253);
            s.store_scalar(64, p.p257);
            s.store_scalar(65, p.p256);
            s.store_offset_scaled(337, 316, ((p.p260) * (p.p259)), p.p259);
        }

        if s.b[1030] {
            s.store_scale_ad(338, {
                if ((1.0 + (p.p262 * s.v[316])) > 0.001) {
                    A::scale_offset(s.ad_value(316), p.p262, 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p261);
        }

        if s.b[1030] {
            s.store_add_ad(339, A::offset(A::mul_sub_from_scalar_rhs(A::div_scaled_product(s.ad_value(337), s.ad_value(338), 1.0, s.ad_value(312), 1.0), 1.0, A::exp_div_scaled_inputs(s.ad_value(312), -1.0, s.ad_value(338), 1.0)), 1.0), A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p263 * p.p264), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p264)))));
        }

        if s.b[1030] {
            if (s.v[339] > 1e-15) {
            } else {
                s.store_scalar(339, 1e-15);
            }
        }

        if s.b[1030] {
            s.store_add_scaled_product_mixed_aia(340, A::scale_offset(s.ad_value(316), p.p265, 1.0), 1.0, 316, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p267), 1.0)), p.p266);
            s.store_mul_div_scaled_inputs_mixed_iia(69, 340, 313, p.p258, A::mul(s.ad_value(339), s.ad_value(312)), 1.0);
            s.store_add_scaled_inputs3_offset_indices(70, 314, p.p269, 316, p.p270, 318, p.p271, p.p268);
            s.store_offset_scaled(71, 316, ((p.p273) * (p.p272)), p.p272);
            s.store_scalar(72, p.p274);
            s.store_scalar(73, p.p275);
            s.store_scalar(74, p.p276);
            s.store_mul3_ad(75, A::scale_offset(A::powf(s.ad_value(314), p.p279), p.p278, p.p277), A::scale_offset(s.ad_value(316), p.p280, 1.0), A::scale_offset(s.ad_value(318), p.p281, 1.0));
            s.store_scalar(76, p.p282);
            s.store_scalar(77, p.p283);
            s.store_scalar(78, p.p284);
            s.store_mul3_ad_scaled_output(79, A::scale_offset(s.ad_value(314), p.p286, 1.0), A::scale_offset(s.ad_value(316), p.p287, 1.0), A::scale_offset(s.ad_value(318), p.p288, 1.0), p.p285);
            s.store_scalar(80, p.p289);
            s.store_scalar(81, p.p290);
            s.store_mul_scale_offset_rhs(82, 316, 316, ((p.p292) * (p.p291)), p.p291);
            s.store_scalar(83, p.p293);
            s.store_scalar(84, p.p294);
            s.store_scalar(85, p.p295);
            s.store_mul3_ad(86, A::offset(A::mul(A::div_scaled_inputs(s.ad_value(340), p.p297, s.ad_value(339), 1.0), A::powf(s.ad_value(314), p.p298)), p.p296), A::scale_offset(s.ad_value(316), p.p299, 1.0), A::scale_offset(s.ad_value(318), p.p300, 1.0));
            s.store_add_scaled_inputs3_offset_indices(87, 314, p.p302, 316, p.p303, 318, p.p304, p.p301);
            s.store_scalar(88, p.p305);
            s.store_scalar(89, p.p306);
            s.store_scalar(90, p.p307);
            s.store_div_from_scalar_offset_scaled_input(91, p.p308, 314, p.p309, 1.0);
            s.store_scaled_mul_scale_offset_rhs_ad(92, A::powf(s.ad_value(314), p.p311), 316, p.p312, 1.0, p.p310);
            s.store_powf(341, 314, p.p314);
            s.store_div_scaled_product_offset_denominator(93, s.ad_value(341), A::scale_offset(s.ad_value(316), p.p316, 1.0), p.p313, A::mul_scaled_lhs(s.ad_value(314), p.p315, s.ad_value(341)), 1.0, 1.0);
            s.store_powf(341, 314, p.p318);
            s.store_div_scaled_product_offset_denominator(94, s.ad_value(341), A::scale_offset(s.ad_value(316), p.p320, 1.0), p.p317, A::mul_scaled_lhs(s.ad_value(314), p.p319, s.ad_value(341)), 1.0, 1.0);
            s.store_scalar(95, p.p321);
            s.store_scaled_mul_scale_offset_inputs(96, 314, p.p323, 1.0, 316, p.p324, 1.0, p.p322);
            s.store_scalar(97, p.p325);
            s.store_scalar(98, p.p326);
            s.store_scaled_mul_scale_offset_inputs(99, 314, p.p328, 1.0, 316, p.p329, 1.0, p.p327);
            s.store_scaled_mul_scale_offset_inputs(100, 314, p.p331, 1.0, 316, p.p332, 1.0, p.p330);
            s.store_scalar(101, p.p333);
            s.store_scalar(102, p.p334);
            s.store_div_from_scalar(103, p.p335, 318);
            s.store_div_from_scalar_scaled_input(104, (p.p336 * p.p236), 316, 1e-6);
            s.store_div_from_scalar_scaled_input(105, (p.p337 * p.p237), 316, 1e-6);
            s.store_scalar(106, p.p338);
            s.store_scalar(107, p.p339);
            s.store_scalar(108, p.p340);
            s.store_scalar(109, p.p339);
        }

        s.b[1033] = param_given[341];
        s.store_scalar(1033, if s.b[1033] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1033]) {
            s.store_scalar(109, p.p341);
        }

        if s.b[1030] {
            s.store_scalar(110, p.p340);
        }

        s.b[1034] = param_given[342];
        s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1034]) {
            s.store_scalar(110, p.p342);
        }

        if s.b[1030] {
            s.copy_ad(111, 109);
        }

        s.b[1035] = param_given[343];
        s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1035]) {
            s.store_scalar(111, p.p343);
        }

        if s.b[1030] {
            s.copy_ad(112, 110);
        }

        s.b[1036] = param_given[344];
        s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1036]) {
            s.store_scalar(112, p.p344);
        }

        if s.b[1030] {
            s.store_scalar(113, p.p345);
            s.store_div_from_scalar_scaled_input(114, (p.p346 * p.p236), 316, 1e-6);
            s.store_div_from_scalar_scaled_input(115, (p.p347 * p.p237), 316, 1e-6);
            s.store_scalar(116, p.p348);
            s.store_scalar(117, p.p349);
            s.store_scalar(118, p.p350);
            s.store_scalar(119, p.p351);
            s.store_scalar(120, p.p352);
            s.store_scalar(121, p.p353);
            s.store_scaled_mul(122, 321, 320, ((8.8541878176e-12 * p.p209) * 1.0 / (p.p208)));
            s.store_scale(129, 321, ((8.8541878176e-12 * p.p209) * (p.p236 * 1.0 / (p.p234))));
            s.store_scale(130, 321, ((8.8541878176e-12 * p.p209) * (p.p237 * 1.0 / (p.p235))));
            s.store_add_scaled_inputs3_offset_mixed_aii(123, A::powf(s.ad_value(314), p.p356), p.p355, 316, p.p357, 318, p.p358, p.p354);
            s.store_add_scaled_inputs3_offset_indices(124, 314, p.p360, 316, p.p361, 318, p.p362, p.p359);
            s.store_scalar(36, p.p296);
        }

        s.b[1037] = param_given[363];
        s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1037]) {
            s.store_scalar(36, p.p363);
        }

        if s.b[1030] {
            s.store_scalar(37, p.p297);
        }

        s.b[1038] = param_given[364];
        s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1038]) {
            s.store_scalar(37, p.p364);
        }

        if s.b[1030] {
            s.store_scalar(38, p.p298);
        }

        s.b[1039] = param_given[365];
        s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1039]) {
            s.store_scalar(38, p.p365);
        }

        if s.b[1030] {
            s.store_scalar(39, p.p299);
        }

        s.b[1040] = param_given[366];
        s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1040]) {
            s.store_scalar(39, p.p366);
        }

        if s.b[1030] {
            s.store_scalar(40, p.p300);
        }

        s.b[1041] = param_given[367];
        s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1041]) {
            s.store_scalar(40, p.p367);
        }

        if s.b[1030] {
            s.store_mul3_ad(125, A::add_scaled_product(s.ad_value(36), 1.0, A::div_scaled_product(s.ad_value(37), s.ad_value(340), 1.0, s.ad_value(339), 1.0), A::pow(s.ad_value(314), s.ad_value(38)), 1.0), A::offset(A::mul(s.ad_value(39), s.ad_value(316)), 1.0), A::offset(A::mul(s.ad_value(40), s.ad_value(318)), 1.0));
            s.store_scalar(41, p.p308);
        }

        s.b[1042] = param_given[368];
        s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1042]) {
            s.store_scalar(41, p.p368);
        }

        if s.b[1030] {
            s.store_scalar(42, p.p309);
        }

        s.b[1043] = param_given[369];
        s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1043]) {
            s.store_scalar(42, p.p369);
        }

        if s.b[1030] {
            s.store_div_scaled_value_offset_denominator(126, s.ad_value(41), 1.0, A::mul(s.ad_value(42), s.ad_value(314)), 1.0, 1.0);
            s.store_scaled_mul_scale_offset_rhs_ad(127, A::powf(s.ad_value(314), p.p371), 316, p.p372, 1.0, p.p370);
            s.store_powf(341, 314, p.p374);
            s.store_div_scaled_product_offset_denominator(128, s.ad_value(341), A::scale_offset(s.ad_value(316), p.p376, 1.0), p.p373, A::mul_scaled_lhs(s.ad_value(314), p.p375, s.ad_value(341)), 1.0, 1.0);
            s.store_scalar(131, p.p377);
            s.store_scalar(132, p.p378);
            s.store_scalar(133, p.p379);
            s.store_scale(134, 325, p.p380);
            s.store_scale(135, 322, p.p381);
            s.store_scale(136, 322, p.p382);
            s.store_scalar(137, p.p383);
            s.store_scalar(138, p.p384);
            s.store_scalar(139, p.p385);
            s.store_scalar(140, p.p386);
            s.store_scale(141, 326, p.p387);
            s.store_scale(142, 326, p.p388);
            s.store_sub_from_scalar_ad(1012, 1.0, A::div_from_scalar((2.0 * p.p395), s.ad_value(312)));
            s.store_scalar(143, p.p389);
            s.store_offset_scaled(344, 313, p.p398, (2.0 * p.p397));
            s.store_scalar(149, p.p399);
            s.store_add_scaled_inputs3_offset_indices(150, 314, p.p401, 316, p.p402, 318, p.p403, p.p400);
            s.store_add_scaled_inputs3_offset_mixed_aii(151, A::powf(s.ad_value(314), p.p406), p.p405, 316, p.p407, 318, p.p408, p.p404);
            s.store_mul3_ad_scaled_output(152, A::scale_offset(A::powf(s.ad_value(314), p.p411), p.p410, 1.0), A::scale_offset(s.ad_value(316), p.p412, 1.0), A::scale_offset(s.ad_value(318), p.p413, 1.0), p.p409);
            s.store_offset_scaled_ad(153, A::powf(s.ad_value(314), p.p416), p.p415, p.p414);
            s.store_offset_ad(347, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p417 * p.p418), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p418)))), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1030] {
            if (s.v[347] > 1e-15) {
            } else {
                s.store_scalar(347, 1e-15);
            }
        }

        if s.b[1030] {
            s.store_mul_div_scaled_inputs_mixed_aia(154, A::scale_offset(s.ad_value(316), p.p419, 1.0), 344, p.p258, A::mul(s.ad_value(347), s.ad_value(312)), 1.0);
            s.store_add_scaled_inputs3_offset_indices(155, 314, p.p421, 316, p.p422, 318, p.p423, p.p420);
            s.store_scaled_mul_scale_offset_rhs_ad(156, A::powf(s.ad_value(314), p.p425), 316, p.p426, 1.0, p.p424);
            s.store_scalar(157, p.p427);
            s.store_scalar(158, p.p428);
            s.store_scaled_mul_scale_offset_rhs_ad(159, A::powf(s.ad_value(314), p.p430), 316, p.p431, 1.0, p.p429);
            s.store_scalar(160, p.p433);
            s.store_scalar(161, p.p432);
            s.store_add_scaled_inputs3_offset_indices(348, 314, p.p815, 316, p.p816, 318, p.p817, p.p814);
            s.store_add_scaled_inputs3_offset_indices(349, 314, p.p819, 316, p.p820, 318, p.p821, p.p818);
            s.store_scalar(176, p.p450);
        }

        s.b[1045] = (((param_given[451] || param_given[452]) || param_given[453]) || param_given[454]);
        s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1045]) {
            s.store_add_scaled_inputs3_offset_indices(44, 314, p.p452, 316, p.p453, 318, p.p454, p.p451);
        }

        s.b[1046] = (((param_given[455] || param_given[456]) || param_given[457]) || param_given[458]);
        s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1046]) {
            s.store_add_scaled_inputs3_offset_indices(45, 314, p.p456, 316, p.p457, 318, p.p458, p.p455);
        }

        s.b[1047] = (((param_given[459] || param_given[460]) || param_given[461]) || param_given[462]);
        s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1047]) {
            s.store_add_scaled_inputs3_offset_indices(49, 314, p.p460, 316, p.p461, 318, p.p462, p.p459);
        }

        s.b[1048] = (((param_given[463] || param_given[464]) || param_given[465]) || param_given[466]);
        s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1048]) {
            s.store_add_scaled_inputs3_offset_indices(50, 314, p.p464, 316, p.p465, 318, p.p466, p.p463);
        }

        s.b[1049] = (((param_given[467] || param_given[468]) || param_given[469]) || param_given[470]);
        s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1049]) {
            s.store_add_scaled_inputs3_offset_indices(51, 314, p.p468, 316, p.p469, 318, p.p470, p.p467);
        }

        s.b[1050] = (((param_given[471] || param_given[472]) || param_given[473]) || param_given[474]);
        s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1050]) {
            s.store_add_scaled_inputs3_offset_indices(53, 314, p.p472, 316, p.p473, 318, p.p474, p.p471);
        }

        s.b[1051] = (((param_given[475] || param_given[476]) || param_given[477]) || param_given[478]);
        s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1051]) {
            s.store_add_scaled_inputs3_offset_indices(54, 314, p.p476, 316, p.p477, 318, p.p478, p.p475);
        }

        s.b[1052] = (((param_given[479] || param_given[480]) || param_given[481]) || param_given[482]);
        s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1052]) {
            s.store_add_scaled_inputs3_offset_indices(61, 314, p.p480, 316, p.p481, 318, p.p482, p.p479);
        }

        s.b[1053] = (((param_given[483] || param_given[484]) || param_given[485]) || param_given[486]);
        s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1053]) {
            s.store_add_scaled_inputs3_offset_indices(62, 314, p.p484, 316, p.p485, 318, p.p486, p.p483);
        }

        s.b[1054] = (((param_given[487] || param_given[488]) || param_given[489]) || param_given[490]);
        s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1054]) {
            s.store_add_scaled_inputs3_offset_indices(55, 314, p.p488, 316, p.p489, 318, p.p490, p.p487);
        }

        s.b[1055] = (((param_given[495] || param_given[496]) || param_given[497]) || param_given[498]);
        s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1055]) {
            s.store_add_scaled_inputs3_offset_indices(56, 314, p.p496, 316, p.p497, 318, p.p498, p.p495);
        }

        s.b[1056] = (((param_given[491] || param_given[492]) || param_given[493]) || param_given[494]);
        s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1056]) {
            s.store_add_scaled_inputs3_offset_indices(57, 314, p.p492, 316, p.p493, 318, p.p494, p.p491);
        }

        s.b[1057] = (((param_given[499] || param_given[500]) || param_given[501]) || param_given[502]);
        s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1057]) {
            s.store_add_scaled_inputs3_offset_indices(58, 314, p.p500, 316, p.p501, 318, p.p502, p.p499);
        }

        s.b[1058] = (((param_given[503] || param_given[504]) || param_given[505]) || param_given[506]);
        s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1058]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(66, 315, s.ad_value(314), p.p504, s.ad_value(316), p.p505, s.ad_value(318), p.p506, p.p503);
        }

        s.b[1059] = (((param_given[511] || param_given[512]) || param_given[513]) || param_given[514]);
        s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1059]) {
            s.store_add_scaled_inputs3_offset_indices(67, 314, p.p512, 316, p.p513, 318, p.p514, p.p511);
        }

        s.b[1060] = (((param_given[507] || param_given[508]) || param_given[509]) || param_given[510]);
        s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1060]) {
            s.store_add_scaled_inputs3_offset_indices(68, 314, p.p508, 316, p.p509, 318, p.p510, p.p507);
        }

        s.b[1061] = (((param_given[515] || param_given[516]) || param_given[517]) || param_given[518]);
        s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1061]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(63, 315, s.ad_value(314), p.p516, s.ad_value(316), p.p517, s.ad_value(318), p.p518, p.p515);
        }

        s.b[1062] = (((param_given[523] || param_given[524]) || param_given[525]) || param_given[526]);
        s.store_scalar(1062, if s.b[1062] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1062]) {
            s.store_add_scaled_inputs3_offset_indices(64, 314, p.p524, 316, p.p525, 318, p.p526, p.p523);
        }

        s.b[1063] = (((param_given[519] || param_given[520]) || param_given[521]) || param_given[522]);
        s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1063]) {
            s.store_add_scaled_inputs3_offset_indices(65, 314, p.p520, 316, p.p521, 318, p.p522, p.p519);
        }

        s.b[1064] = (((param_given[527] || param_given[528]) || param_given[529]) || param_given[530]);
        s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1064]) {
            s.store_mul_div_scaled_inputs_mixed_aii(69, A::add_scaled_inputs3_offset(s.ad_value(314), p.p528, s.ad_value(316), p.p529, s.ad_value(318), p.p530, p.p527), 313, 1.0, 312, 1.0);
        }

        s.b[1065] = (((param_given[531] || param_given[532]) || param_given[533]) || param_given[534]);
        s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1065]) {
            s.store_add_scaled_inputs3_offset_indices(70, 314, p.p532, 316, p.p533, 318, p.p534, p.p531);
        }

        s.b[1066] = (((param_given[535] || param_given[536]) || param_given[537]) || param_given[538]);
        s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1066]) {
            s.store_add_scaled_inputs3_offset_indices(71, 314, p.p536, 316, p.p537, 318, p.p538, p.p535);
        }

        s.b[1067] = (((param_given[539] || param_given[540]) || param_given[541]) || param_given[542]);
        s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1067]) {
            s.store_add_scaled_inputs3_offset_indices(73, 314, p.p540, 316, p.p541, 318, p.p542, p.p539);
        }

        s.b[1068] = (((param_given[543] || param_given[544]) || param_given[545]) || param_given[546]);
        s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1068]) {
            s.store_add_scaled_inputs3_offset_indices(75, 314, p.p544, 316, p.p545, 318, p.p546, p.p543);
        }

        s.b[1069] = (((param_given[547] || param_given[548]) || param_given[549]) || param_given[550]);
        s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1069]) {
            s.store_add_scaled_inputs3_offset_indices(77, 314, p.p548, 316, p.p549, 318, p.p550, p.p547);
        }

        s.b[1070] = (((param_given[551] || param_given[552]) || param_given[553]) || param_given[554]);
        s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1070]) {
            s.store_add_scaled_inputs3_offset_indices(79, 314, p.p552, 316, p.p553, 318, p.p554, p.p551);
        }

        s.b[1071] = (((param_given[555] || param_given[556]) || param_given[557]) || param_given[558]);
        s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1071]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(82, 316, s.ad_value(314), p.p556, s.ad_value(316), p.p557, s.ad_value(318), p.p558, p.p555);
        }

        s.b[1072] = (((param_given[559] || param_given[560]) || param_given[561]) || param_given[562]);
        s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1072]) {
            s.store_add_scaled_inputs3_offset_indices(83, 314, p.p560, 316, p.p561, 318, p.p562, p.p559);
        }

        s.b[1073] = (((param_given[563] || param_given[564]) || param_given[565]) || param_given[566]);
        s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1073]) {
            s.store_add_scaled_inputs3_offset_indices(84, 314, p.p564, 316, p.p565, 318, p.p566, p.p563);
        }

        s.b[1074] = (((param_given[567] || param_given[568]) || param_given[569]) || param_given[570]);
        s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1074]) {
            s.store_add_scaled_inputs3_offset_indices(85, 314, p.p568, 316, p.p569, 318, p.p570, p.p567);
        }

        s.b[1075] = (((param_given[571] || param_given[572]) || param_given[573]) || param_given[574]);
        s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1075]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(86, 314, s.ad_value(314), p.p572, s.ad_value(316), p.p573, s.ad_value(318), p.p574, p.p571);
        }

        s.b[1076] = (((param_given[575] || param_given[576]) || param_given[577]) || param_given[578]);
        s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1076]) {
            s.store_add_scaled_inputs3_offset_indices(87, 314, p.p576, 316, p.p577, 318, p.p578, p.p575);
        }

        s.b[1077] = (((param_given[579] || param_given[580]) || param_given[581]) || param_given[582]);
        s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1077]) {
            s.store_add_scaled_inputs3_offset_indices(88, 314, p.p580, 316, p.p581, 318, p.p582, p.p579);
        }

        s.b[1078] = (((param_given[583] || param_given[584]) || param_given[585]) || param_given[586]);
        s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1078]) {
            s.store_add_scaled_inputs3_offset_indices(89, 314, p.p584, 316, p.p585, 318, p.p586, p.p583);
        }

        s.b[1079] = (((param_given[587] || param_given[588]) || param_given[589]) || param_given[590]);
        s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1079]) {
            s.store_add_scaled_inputs3_offset_indices(91, 314, p.p588, 316, p.p589, 318, p.p590, p.p587);
        }

        s.b[1080] = (((param_given[591] || param_given[592]) || param_given[593]) || param_given[594]);
        s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1080]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(92, 314, s.ad_value(314), p.p592, s.ad_value(316), p.p593, s.ad_value(318), p.p594, p.p591);
        }

        s.b[1081] = (((param_given[595] || param_given[596]) || param_given[597]) || param_given[598]);
        s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1081]) {
            s.store_add_scaled_inputs3_offset_indices(93, 314, p.p596, 316, p.p597, 318, p.p598, p.p595);
        }

        s.b[1082] = (((param_given[599] || param_given[600]) || param_given[601]) || param_given[602]);
        s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1082]) {
            s.store_add_scaled_inputs3_offset_indices(94, 314, p.p600, 316, p.p601, 318, p.p602, p.p599);
        }

        s.b[1083] = (((param_given[603] || param_given[604]) || param_given[605]) || param_given[606]);
        s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1083]) {
            s.store_add_scaled_inputs3_offset_indices(96, 314, p.p604, 316, p.p605, 318, p.p606, p.p603);
        }

        s.b[1084] = (((param_given[607] || param_given[608]) || param_given[609]) || param_given[610]);
        s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1084]) {
            s.store_add_scaled_inputs3_offset_indices(98, 314, p.p608, 316, p.p609, 318, p.p610, p.p607);
        }

        s.b[1085] = (((param_given[611] || param_given[612]) || param_given[613]) || param_given[614]);
        s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1085]) {
            s.store_add_scaled_inputs3_offset_indices(99, 314, p.p612, 316, p.p613, 318, p.p614, p.p611);
        }

        s.b[1086] = (((param_given[615] || param_given[616]) || param_given[617]) || param_given[618]);
        s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1086]) {
            s.store_add_scaled_inputs3_offset_indices(100, 314, p.p616, 316, p.p617, 318, p.p618, p.p615);
        }

        s.b[1087] = (((param_given[619] || param_given[620]) || param_given[621]) || param_given[622]);
        s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1087]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(103, 319, s.ad_value(314), p.p620, s.ad_value(316), p.p621, s.ad_value(318), p.p622, p.p619);
        }

        s.b[1088] = (((param_given[623] || param_given[624]) || param_given[625]) || param_given[626]);
        s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1088]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(104, 317, s.ad_value(314), p.p624, s.ad_value(316), p.p625, s.ad_value(318), p.p626, p.p623);
        }

        s.b[1089] = (((param_given[627] || param_given[628]) || param_given[629]) || param_given[630]);
        s.store_scalar(1089, if s.b[1089] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1089]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(105, 317, s.ad_value(314), p.p628, s.ad_value(316), p.p629, s.ad_value(318), p.p630, p.p627);
        }

        s.b[1090] = (((param_given[631] || param_given[632]) || param_given[633]) || param_given[634]);
        s.store_scalar(1090, if s.b[1090] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1090]) {
            s.store_add_scaled_inputs3_offset_indices(106, 314, p.p632, 316, p.p633, 318, p.p634, p.p631);
        }

        s.b[1091] = (((param_given[635] || param_given[636]) || param_given[637]) || param_given[638]);
        s.store_scalar(1091, if s.b[1091] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1091]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(114, 317, s.ad_value(314), p.p636, s.ad_value(316), p.p637, s.ad_value(318), p.p638, p.p635);
        }

        s.b[1092] = (((param_given[639] || param_given[640]) || param_given[641]) || param_given[642]);
        s.store_scalar(1092, if s.b[1092] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1092]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(115, 317, s.ad_value(314), p.p640, s.ad_value(316), p.p641, s.ad_value(318), p.p642, p.p639);
        }

        s.b[1093] = (((param_given[643] || param_given[644]) || param_given[645]) || param_given[646]);
        s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1093]) {
            s.store_add_scaled_inputs3_offset_indices(118, 314, p.p644, 316, p.p645, 318, p.p646, p.p643);
        }

        s.b[1094] = (((param_given[647] || param_given[648]) || param_given[649]) || param_given[650]);
        s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1094]) {
            s.store_add_scaled_inputs3_offset_indices(119, 314, p.p648, 316, p.p649, 318, p.p650, p.p647);
        }

        s.b[1095] = (((param_given[651] || param_given[652]) || param_given[653]) || param_given[654]);
        s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1095]) {
            s.store_mul_ad_affine_product_rhs(122, 322, s.ad_value(320), A::add_scaled_inputs3_offset(s.ad_value(314), p.p652, s.ad_value(316), p.p653, s.ad_value(318), p.p654, p.p651), 1.0 / (1e-6), 0.0);
        }

        s.b[1096] = (((param_given[655] || param_given[656]) || param_given[657]) || param_given[658]);
        s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1096]) {
            s.store_add_scaled_inputs3_offset_indices(123, 314, p.p656, 316, p.p657, 318, p.p658, p.p655);
        }

        s.b[1097] = (((param_given[659] || param_given[660]) || param_given[661]) || param_given[662]);
        s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1097]) {
            s.store_add_scaled_inputs3_offset_indices(124, 314, p.p660, 316, p.p661, 318, p.p662, p.p659);
        }

        s.b[1098] = (((((((param_given[663] || param_given[664]) || param_given[665]) || param_given[666]) || param_given[571]) || param_given[572]) || param_given[573]) || param_given[574]);
        s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(32, p.p571);
        }

        s.b[1099] = param_given[663];
        s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1098]) && s.b[1099]) {
            s.store_scalar(32, p.p663);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(33, p.p572);
        }

        s.b[1100] = param_given[664];
        s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1098]) && s.b[1100]) {
            s.store_scalar(33, p.p664);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(34, p.p573);
        }

        s.b[1101] = param_given[665];
        s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1098]) && s.b[1101]) {
            s.store_scalar(34, p.p665);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(35, p.p574);
        }

        s.b[1102] = param_given[666];
        s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1098]) && s.b[1102]) {
            s.store_scalar(35, p.p666);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_mul_ad_rhs(125, 314, A::add_scaled_value_products3(s.ad_value(32), 1.0, s.ad_value(33), s.ad_value(314), 1.0, s.ad_value(34), s.ad_value(316), 1.0, s.ad_value(35), s.ad_value(318), 1.0));
        }

        s.b[1103] = (((((((param_given[667] || param_given[668]) || param_given[669]) || param_given[670]) || param_given[587]) || param_given[588]) || param_given[589]) || param_given[590]);
        s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(32, p.p587);
        }

        s.b[1104] = param_given[667];
        s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1103]) && s.b[1104]) {
            s.store_scalar(32, p.p667);
        }

        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(33, p.p588);
        }

        s.b[1105] = param_given[668];
        s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1103]) && s.b[1105]) {
            s.store_scalar(33, p.p668);
        }

        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(34, p.p589);
        }

        s.b[1106] = param_given[669];
        s.store_scalar(1106, if s.b[1106] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1103]) && s.b[1106]) {
            s.store_scalar(34, p.p669);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(35, p.p590);
        }

        s.b[1107] = param_given[670];
        s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1103]) && s.b[1107]) {
            s.store_scalar(35, p.p670);
        }

        if (s.b[1030] && s.b[1103]) {
            s.store_add_scaled_value_products3_indices(126, 32, 1.0, 33, 314, 1.0, 34, 316, 1.0, 35, 318, 1.0);
        }

        s.b[1108] = (((param_given[671] || param_given[672]) || param_given[673]) || param_given[674]);
        s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1108]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(127, 314, s.ad_value(314), p.p672, s.ad_value(316), p.p673, s.ad_value(318), p.p674, p.p671);
        }

        s.b[1109] = (((param_given[675] || param_given[676]) || param_given[677]) || param_given[678]);
        s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1109]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(128, 314, s.ad_value(314), p.p676, s.ad_value(316), p.p677, s.ad_value(318), p.p678, p.p675);
        }

        s.b[1110] = (((param_given[679] || param_given[680]) || param_given[681]) || param_given[682]);
        s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1110]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(129, 322, s.ad_value(314), p.p680, s.ad_value(316), p.p681, s.ad_value(318), p.p682, p.p679);
        }

        s.b[1111] = (((param_given[683] || param_given[684]) || param_given[685]) || param_given[686]);
        s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1111]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(130, 322, s.ad_value(314), p.p684, s.ad_value(316), p.p685, s.ad_value(318), p.p686, p.p683);
        }

        s.b[1112] = (((param_given[687] || param_given[688]) || param_given[689]) || param_given[690]);
        s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1112]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(134, 325, s.ad_value(314), p.p688, s.ad_value(316), p.p689, s.ad_value(318), p.p690, p.p687);
        }

        s.b[1113] = (((param_given[691] || param_given[692]) || param_given[693]) || param_given[694]);
        s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1113]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 322, s.ad_value(314), p.p692, s.ad_value(316), p.p693, s.ad_value(318), p.p694, p.p691);
        }

        s.b[1114] = (((param_given[695] || param_given[696]) || param_given[697]) || param_given[698]);
        s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1114]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(136, 322, s.ad_value(314), p.p696, s.ad_value(316), p.p697, s.ad_value(318), p.p698, p.p695);
        }

        s.b[1115] = (((param_given[699] || param_given[700]) || param_given[701]) || param_given[702]);
        s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1115]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(141, 326, s.ad_value(314), p.p700, s.ad_value(316), p.p701, s.ad_value(318), p.p702, p.p699);
        }

        s.b[1116] = (((param_given[703] || param_given[704]) || param_given[705]) || param_given[706]);
        s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1116]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(142, 326, s.ad_value(314), p.p704, s.ad_value(316), p.p705, s.ad_value(318), p.p706, p.p703);
        }

        s.b[1121] = (((param_given[723] || param_given[724]) || param_given[725]) || param_given[726]);
        s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1121]) {
            s.store_add_scaled_inputs3_offset_indices(149, 314, p.p724, 316, p.p725, 318, p.p726, p.p723);
        }

        s.b[1122] = (((param_given[727] || param_given[728]) || param_given[729]) || param_given[730]);
        s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1122]) {
            s.store_add_scaled_inputs3_offset_indices(150, 314, p.p728, 316, p.p729, 318, p.p730, p.p727);
        }

        s.b[1123] = (((param_given[731] || param_given[732]) || param_given[733]) || param_given[734]);
        s.store_scalar(1123, if s.b[1123] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1123]) {
            s.store_add_scaled_inputs3_offset_indices(151, 314, p.p732, 316, p.p733, 318, p.p734, p.p731);
        }

        s.b[1124] = (((param_given[735] || param_given[736]) || param_given[737]) || param_given[738]);
        s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1124]) {
            s.store_add_scaled_inputs3_offset_indices(152, 314, p.p736, 316, p.p737, 318, p.p738, p.p735);
        }

        s.b[1125] = (((param_given[739] || param_given[740]) || param_given[741]) || param_given[742]);
        s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1125]) {
            s.store_add_scaled_inputs3_offset_indices(153, 314, p.p740, 316, p.p741, 318, p.p742, p.p739);
        }

        s.b[1126] = (((param_given[743] || param_given[744]) || param_given[745]) || param_given[746]);
        s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1126]) {
            s.store_mul_div_scaled_inputs_mixed_aii(154, A::add_scaled_inputs3_offset(s.ad_value(314), p.p744, s.ad_value(316), p.p745, s.ad_value(318), p.p746, p.p743), 344, 1.0, 312, 1.0);
        }

        s.b[1127] = (((param_given[747] || param_given[748]) || param_given[749]) || param_given[750]);
        s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1127]) {
            s.store_add_scaled_inputs3_offset_indices(155, 314, p.p748, 316, p.p749, 318, p.p750, p.p747);
        }

        s.b[1128] = (((param_given[751] || param_given[752]) || param_given[753]) || param_given[754]);
        s.store_scalar(1128, if s.b[1128] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1128]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(156, 315, s.ad_value(314), p.p752, s.ad_value(316), p.p753, s.ad_value(318), p.p754, p.p751);
        }

        s.b[1129] = (((param_given[755] || param_given[756]) || param_given[757]) || param_given[758]);
        s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1129]) {
            s.store_add_scaled_inputs3_offset_indices(157, 314, p.p756, 316, p.p757, 318, p.p758, p.p755);
        }

        s.b[1130] = (((param_given[759] || param_given[760]) || param_given[761]) || param_given[762]);
        s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1130]) {
            s.store_add_scaled_inputs3_offset_indices(158, 314, p.p760, 316, p.p761, 318, p.p762, p.p759);
        }

        s.b[1131] = (((param_given[763] || param_given[764]) || param_given[765]) || param_given[766]);
        s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1131]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(159, 315, s.ad_value(314), p.p764, s.ad_value(316), p.p765, s.ad_value(318), p.p766, p.p763);
        }

        s.b[1132] = (((param_given[771] || param_given[772]) || param_given[773]) || param_given[774]);
        s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1132]) {
            s.store_add_scaled_inputs3_offset_indices(160, 314, p.p772, 316, p.p773, 318, p.p774, p.p771);
        }

        s.b[1133] = (((param_given[767] || param_given[768]) || param_given[769]) || param_given[770]);
        s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1133]) {
            s.store_add_scaled_inputs3_offset_indices(161, 314, p.p768, 316, p.p769, 318, p.p770, p.p767);
        }

        s.b[1137] = (((param_given[787] || param_given[788]) || param_given[789]) || param_given[790]);
        s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1137]) {
            s.store_add_scaled_inputs3_offset_indices(176, 314, p.p788, 316, p.p789, 318, p.p790, p.p787);
        }

        if s.b[1030] {
            s.store_scalar(1019, 0.0);
            s.store_scalar(1020, 0.0);
            s.store_scalar(1018, 0.0);
            s.store_scalar(43, p.p795);
        }

        s.b[1138] = param_given[796];
        s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1138]) {
            s.store_scalar(43, p.p796);
        }

        s.b[1139] = (((s.v[9] > 0.0) && (s.v[10] > 0.0)) && ((s.v[5] == 1.0) || ((s.v[5] > 1.0) && (s.v[11] > 0.0))));
        s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });

        let mut assign9340_loop_guard: usize = 0;
        while {
            let assign9340_cond_e9222: f64 = (s.v[5] - 0.5);
            let assign9340_cond_e9224: f64 = if ((s.b[1030] && s.b[1139]) && (s.v[1018] < assign9340_cond_e9222)) { 1.0 } else { 0.0 };
            assign9340_cond_e9224 != 0.0
        } {
            assign9340_loop_guard += 1;
            assert!(assign9340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1030] && s.b[1139]) {
                s.store_add_ad_rhs(1019, 1019, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + s.v[7]), (s.v[9] + (0.5 * s.v[7])))));
                s.store_add_ad_rhs(1020, 1020, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + s.v[7]), (s.v[10] + (0.5 * s.v[7])))));
                s.store_offset(1018, 1018, 1.0);
            }
        }

        if (s.b[1030] && s.b[1139]) {
            s.store_mul(1003, 1019, 6);
            s.store_mul(1004, 1020, 6);
            s.store_scalar(1005, (1.0 / (p.p791 + (0.5 * s.v[7]))));
            s.store_scalar(1006, (1.0 / (p.p792 + (0.5 * s.v[7]))));
        }

        if (s.b[1030] && s.b[1139]) {
            if ((s.v[7] + s.v[310]) > 1e-9) {
                s.store_offset(1016, 310, s.v[7]);
            } else {
                s.store_scalar(1016, 1e-9);
            }
        }

        if (s.b[1030] && s.b[1139]) {
            if (((s.v[8] + s.v[311]) + p.p793) > 1e-9) {
                s.store_offset_add(1017, 8, 311, p.p793);
            } else {
                s.store_scalar(1017, 1e-9);
            }
        }

        if (s.b[1030] && s.b[1139]) {
            s.store_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p.p801);
            s.store_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p.p802);
            s.store_add_scaled_inputs_product_first_ad(1007, A::scale_offset(s.ad_value(1014), p.p798, 1.0), (1.0 + (p.p797 * (s.v[352] - 1.0))), 1015, (p.p799 * (1.0 + (p.p797 * (s.v[352] - 1.0)))), 1014, 1015, (p.p800 * (1.0 + (p.p797 * (s.v[352] - 1.0)))));
            s.store_div_scaled_inputs2_indices(1008, 1003, p.p794, 1004, p.p794, 1007, 1.0);
            s.store_div_scaled_inputs2_indices(1009, 1005, p.p794, 1006, p.p794, 1007, 1.0);
            s.store_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p.p807);
            s.store_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p.p808);
            s.store_add_scaled_inputs_product_first_ad(1010, A::scale_offset(s.ad_value(1014), p.p804, 1.0), 1.0, 1015, p.p805, 1014, 1015, p.p806);
            s.store_add_scaled_inputs4_indices(1012, 1003, 1.0, 1004, 1.0, 1005, -1.0, 1006, -1.0);
            s.store_div_scaled_offset_numerator(1013, s.ad_value(1008), 1.0, 1.0, A::offset(s.ad_value(1009), 1.0), 1.0);
            s.store_mul(69, 69, 1013);
            s.store_div_scaled_product3_mixed_iiaa(86, 86, 1013, A::scale_offset(s.ad_value(1009), p.p795, 1.0), 1.0, A::scale_offset(s.ad_value(1008), p.p795, 1.0), 1.0);
            s.store_div_scaled_product3_mixed_iiaa(125, 125, 1013, A::offset(A::mul(s.ad_value(43), s.ad_value(1009)), 1.0), 1.0, A::offset(A::mul(s.ad_value(43), s.ad_value(1008)), 1.0), 1.0);
            s.store_mul(154, 154, 1013);
            s.store_div_scaled_inputs_indices(1013, 1012, p.p803, 1010, 1.0);
            s.store_add(44, 44, 1013);
            s.store_add(149, 149, 1013);
            s.store_div_scaled_inputs_mixed_ia(1013, 1012, p.p809, A::powf(s.ad_value(1010), p.p810), 1.0);
            s.store_add(66, 66, 1013);
            s.store_add(159, 159, 1013);
        }

        s.b[1140] = ((((s.v[15] > 0.0) || (s.v[16] > 0.0)) || (s.v[17] > 0.0)) || (s.v[12] > 0.0));
        s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });

        s.b[1141] = (((s.v[15] == 0.0) && (s.v[16] == 0.0)) && (s.v[17] == 0.0));
        s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1140]) && s.b[1141]) {
            s.store_offset(1012, 8, s.v[12]);
            s.store_scalar(1013, (1.0 / p.p811));
            s.store_div_from_scalar_scaled_input(15, (p.p811 * p.p811), 1012, s.v[12]);
            s.store_div_scaled_add_product(16, A::exp_scaled_input(s.ad_value(1013), ((-10.0) * s.v[12])), ((0.1 * s.v[12]) + (0.01 * p.p811)), A::scale_offset(s.ad_value(1012), 0.1, (0.01 * p.p811)), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-10.0), s.ad_value(1013))), (-1.0), s.ad_value(8), 1.0);
            s.store_div_scaled_add_product(17, A::exp_scaled_input(s.ad_value(1013), ((-20.0) * s.v[12])), ((0.05 * s.v[12]) + (0.0025 * p.p811)), A::scale_offset(s.ad_value(1012), 0.05, (0.0025 * p.p811)), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-20.0), s.ad_value(1013))), (-1.0), s.ad_value(8), 1.0);
        }

        if (s.b[1030] && s.b[1140]) {
            s.store_add_scaled_inputs3_indices(1012, 15, 1.0, 16, p.p812, 17, p.p813);
            s.store_add_scaled_product_indices(44, 44, 1.0, 348, 1012, 1.0);
            s.store_mul_offset_ad_rhs(69, 69, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0);
            s.store_add_scaled_product_indices(149, 149, 1.0, 348, 1012, 1.0);
            s.store_mul_offset_ad_rhs(154, 154, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0);
        }

        s.copy_ad(177, 44);

        s.copy_ad(178, 45);

        s.copy_ad(179, 46);

        s.copy_ad(181, 47);

        s.copy_ad(182, 48);

        if (s.v[49] > 1e20) {
            if (s.v[49] < 1e26) {
                s.copy_ad(183, 49);
            } else {
                s.store_scalar(183, 1e26);
            }
        } else {
            s.store_scalar(183, 1e20);
        }

        if (s.v[50] > 0.01) {
            s.copy_ad(184, 50);
        } else {
            s.store_scalar(184, 0.01);
        }

        if (s.v[51] > 0.0) {
            s.copy_ad(185, 51);
        } else {
            s.store_scalar(185, 0.0);
        }

        s.copy_ad(186, 52);

        s.copy_ad(187, 53);

        if (s.v[54] > 0.0) {
            s.copy_ad(188, 54);
        } else {
            s.store_scalar(188, 0.0);
        }

        s.copy_ad(192, 59);

        s.copy_ad(193, 60);

        if (s.v[61] > 1e23) {
            if (s.v[61] < 1e27) {
                s.copy_ad(194, 61);
            } else {
                s.store_scalar(194, 1e27);
            }
        } else {
            s.store_scalar(194, 1e23);
        }

        if (s.v[62] > 1e23) {
            if (s.v[62] < 1e27) {
                s.copy_ad(195, 62);
            } else {
                s.store_scalar(195, 1e27);
            }
        } else {
            s.store_scalar(195, 1e23);
        }

        if (s.v[55] > 0.0) {
            s.copy_ad(189, 55);
        } else {
            s.store_scalar(189, 0.0);
        }

        if (s.v[57] > 0.0) {
            if (s.v[57] < 0.5) {
                s.copy_ad(191, 57);
            } else {
                s.store_scalar(191, 0.5);
            }
        } else {
            s.store_scalar(191, 0.0);
        }

        if (s.v[56] > 0.0) {
            if (s.v[56] < 1.0) {
                s.copy_ad(190, 56);
            } else {
                s.store_scalar(190, 1.0);
            }
        } else {
            s.store_scalar(190, 0.0);
        }

        s.copy_ad(180, 58);

        if (s.v[66] > 0.0) {
            s.copy_ad(196, 66);
        } else {
            s.store_scalar(196, 0.0);
        }

        if (s.v[68] > 0.0) {
            if (s.v[68] < 1.0) {
                s.copy_ad(198, 68);
            } else {
                s.store_scalar(198, 1.0);
            }
        } else {
            s.store_scalar(198, 0.0);
        }

        if (s.v[67] > 0.0) {
            s.copy_ad(197, 67);
        } else {
            s.store_scalar(197, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.v[63] > 0.0) {
            s.copy_ad(199, 63);
        } else {
            s.store_scalar(199, 0.0);
        }

        if (s.v[65] > 0.0) {
            if (s.v[65] < 1.0) {
                s.copy_ad(200, 65);
            } else {
                s.store_scalar(200, 1.0);
            }
        } else {
            s.store_scalar(200, 0.0);
        }

        if (s.v[64] > 0.0) {
            s.copy_ad(201, 64);
        } else {
            s.store_scalar(201, 0.0);
        }

        if (s.v[69] > 0.0) {
            s.copy_ad(202, 69);
        } else {
            s.store_scalar(202, 0.0);
        }

        s.copy_ad(203, 70);

        if (s.v[71] > 0.0) {
            s.copy_ad(204, 71);
        } else {
            s.store_scalar(204, 0.0);
        }

        s.copy_ad(205, 72);

        if (s.v[73] > 0.0) {
            s.copy_ad(206, 73);
        } else {
            s.store_scalar(206, 0.0);
        }

        s.copy_ad(207, 74);

        if (s.v[75] > 0.0) {
            s.copy_ad(208, 75);
        } else {
            s.store_scalar(208, 0.0);
        }

        s.copy_ad(209, 76);

        if (s.v[77] > 0.0) {
            s.copy_ad(210, 77);
        } else {
            s.store_scalar(210, 0.0);
        }

        s.copy_ad(211, 78);

        if (s.v[79] > 0.0) {
            s.copy_ad(212, 79);
        } else {
            s.store_scalar(212, 0.0);
        }

        s.copy_ad(213, 80);

        s.copy_ad(214, 81);

        if (s.v[82] > 0.0) {
            s.copy_ad(215, 82);
        } else {
            s.store_scalar(215, 0.0);
        }

        s.copy_ad(216, 83);

        if (s.v[84] > (-0.5)) {
            if (s.v[84] < 1.0) {
                s.copy_ad(217, 84);
            } else {
                s.store_scalar(217, 1.0);
            }
        } else {
            s.store_scalar(217, (-0.5));
        }

        if (s.v[85] > (-0.5)) {
            s.copy_ad(218, 85);
        } else {
            s.store_scalar(218, (-0.5));
        }

        if (s.v[86] > 0.0) {
            s.copy_ad(219, 86);
        } else {
            s.store_scalar(219, 0.0);
        }

        s.copy_ad(220, 87);

        if (s.v[88] > (-0.5)) {
            if (s.v[88] < 1.0) {
                s.copy_ad(221, 88);
            } else {
                s.store_scalar(221, 1.0);
            }
        } else {
            s.store_scalar(221, (-0.5));
        }

        if (s.v[89] > (-0.5)) {
            s.copy_ad(222, 89);
        } else {
            s.store_scalar(222, (-0.5));
        }

        if (s.v[90] > 0.01) {
            s.copy_ad(223, 90);
        } else {
            s.store_scalar(223, 0.01);
        }

        if (s.v[91] > 2.0) {
            s.copy_ad(224, 91);
        } else {
            s.store_scalar(224, 2.0);
        }

        if (s.v[92] > 0.0) {
            s.copy_ad(225, 92);
        } else {
            s.store_scalar(225, 0.0);
        }

        if (s.v[93] > 0.0) {
            s.copy_ad(226, 93);
        } else {
            s.store_scalar(226, 0.0);
        }

        if (s.v[94] > 0.0) {
            s.copy_ad(227, 94);
        } else {
            s.store_scalar(227, 0.0);
        }

        s.copy_ad(228, 95);

        if (s.v[96] > 0.0) {
            s.copy_ad(229, 96);
        } else {
            s.store_scalar(229, 0.0);
        }

        s.copy_ad(230, 97);

        s.copy_ad(231, 98);

        if (s.v[99] > 0.0) {
            s.copy_ad(232, 99);
        } else {
            s.store_scalar(232, 0.0);
        }

        if (s.v[100] > 0.0) {
            s.copy_ad(233, 100);
        } else {
            s.store_scalar(233, 0.0);
        }

        if (s.v[101] > 1e-12) {
            s.copy_ad(234, 101);
        } else {
            s.store_scalar(234, 1e-12);
        }

        s.copy_ad(235, 102);

        if (s.v[103] > 0.0) {
            s.copy_ad(236, 103);
        } else {
            s.store_scalar(236, 0.0);
        }

        if (s.v[104] > 0.0) {
            s.copy_ad(237, 104);
        } else {
            s.store_scalar(237, 0.0);
        }

        if (s.v[105] > 0.0) {
            s.copy_ad(238, 105);
        } else {
            s.store_scalar(238, 0.0);
        }

        s.copy_ad(239, 106);

        s.copy_ad(240, 107);

        s.copy_ad(241, 108);

        s.copy_ad(242, 109);

        s.copy_ad(243, 110);

        s.copy_ad(244, 111);

        s.copy_ad(245, 112);

        s.copy_ad(246, 113);

        if (s.v[114] > 0.0) {
            s.copy_ad(247, 114);
        } else {
            s.store_scalar(247, 0.0);
        }

        if (s.v[115] > 0.0) {
            s.copy_ad(248, 115);
        } else {
            s.store_scalar(248, 0.0);
        }

        s.copy_ad(249, 116);

        s.copy_ad(250, 117);

        s.copy_ad(251, 118);

        s.copy_ad(252, 119);

        s.copy_ad(253, 120);

        s.copy_ad(254, 121);

        if (s.v[122] > 0.0) {
            s.copy_ad(255, 122);
        } else {
            s.store_scalar(255, 0.0);
        }

        s.copy_ad(256, 123);

        if (s.v[124] > 0.0) {
            s.copy_ad(257, 124);
        } else {
            s.store_scalar(257, 0.0);
        }

        if (s.v[125] > 0.0) {
            s.copy_ad(258, 125);
        } else {
            s.store_scalar(258, 0.0);
        }

        if (s.v[126] > 2.0) {
            s.copy_ad(259, 126);
        } else {
            s.store_scalar(259, 2.0);
        }

        s.copy_ad(260, 127);

        if (s.v[128] > 0.0) {
            s.copy_ad(261, 128);
        } else {
            s.store_scalar(261, 0.0);
        }

        if (s.v[129] > 0.0) {
            s.copy_ad(262, 129);
        } else {
            s.store_scalar(262, 0.0);
        }

        if (s.v[130] > 0.0) {
            s.copy_ad(263, 130);
        } else {
            s.store_scalar(263, 0.0);
        }

        s.copy_ad(264, 131);

        s.copy_ad(265, 132);

        s.copy_ad(266, 133);

        if (s.v[134] > 0.0) {
            s.copy_ad(267, 134);
        } else {
            s.store_scalar(267, 0.0);
        }

        if (s.v[135] > 0.0) {
            s.copy_ad(268, 135);
        } else {
            s.store_scalar(268, 0.0);
        }

        if (s.v[136] > 0.0) {
            s.copy_ad(269, 136);
        } else {
            s.store_scalar(269, 0.0);
        }

        s.copy_ad(270, 137);

        s.copy_ad(271, 138);

        s.copy_ad(272, 139);

        s.copy_ad(273, 140);

        if (s.v[141] > 0.0) {
            s.copy_ad(274, 141);
        } else {
            s.store_scalar(274, 0.0);
        }

        if (s.v[142] > 0.0) {
            s.copy_ad(275, 142);
        } else {
            s.store_scalar(275, 0.0);
        }

        s.copy_ad(276, 143);

        s.copy_ad(282, 149);

        s.copy_ad(283, 150);

        s.copy_ad(284, 151);

        if (s.v[152] > 1e20) {
            if (s.v[152] < 1e26) {
                s.copy_ad(285, 152);
            } else {
                s.store_scalar(285, 1e26);
            }
        } else {
            s.store_scalar(285, 1e20);
        }

        if (s.v[153] > 0.0) {
            s.copy_ad(286, 153);
        } else {
            s.store_scalar(286, 0.0);
        }

        if (s.v[154] > 0.0) {
            s.copy_ad(287, 154);
        } else {
            s.store_scalar(287, 0.0);
        }

        s.copy_ad(288, 155);

        if (s.v[156] > 0.0) {
            s.copy_ad(289, 156);
        } else {
            s.store_scalar(289, 0.0);
        }

        if (s.v[157] > 0.0) {
            if (s.v[157] < 1.0) {
                s.copy_ad(290, 157);
            } else {
                s.store_scalar(290, 1.0);
            }
        } else {
            s.store_scalar(290, 0.0);
        }

        if (s.v[158] > 0.0) {
            s.copy_ad(291, 158);
        } else {
            s.store_scalar(291, 0.0);
        }

        if (s.v[159] > 0.0) {
            s.copy_ad(292, 159);
        } else {
            s.store_scalar(292, 0.0);
        }

        if (s.v[161] > 0.0) {
            if (s.v[161] < 1.0) {
                s.copy_ad(294, 161);
            } else {
                s.store_scalar(294, 1.0);
            }
        } else {
            s.store_scalar(294, 0.0);
        }

        if (s.v[160] > 0.0) {
            s.copy_ad(293, 160);
        } else {
            s.store_scalar(293, 0.0);
        }

        if ((p.p31 * s.v[5]) > 0.0) {
            s.store_scale(19, 5, p.p31);
        } else {
            s.store_scalar(19, 0.0);
        }

        s.store_scalar(20, p.p16);

        s.store_scalar(21, p.p15);

        s.store_scalar(22, p.p18);

        s.store_scalar(23, p.p17);

        if (s.v[176] > 0.0) {
            s.copy_ad(307, 176);
        } else {
            s.store_scalar(307, 0.0);
        }

        s.b[1142] = (p.p44 == 0.0);
        s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });

        if s.b[1142] {
            s.copy_ad(193, 192);
            s.copy_ad(195, 194);
            s.copy_ad(248, 247);
            s.copy_ad(250, 249);
            s.copy_ad(252, 251);
            s.copy_ad(254, 253);
            s.copy_ad(238, 237);
            s.copy_ad(244, 242);
            s.copy_ad(245, 243);
            s.copy_ad(263, 262);
            s.copy_ad(265, 264);
            s.copy_ad(269, 268);
            s.copy_ad(275, 274);
        }

        s.store_scale(768, 182, 8.8541878176e-12);

        s.store_div(769, 768, 181);

        s.store_square(770, 181);

        s.store_scale(771, 769, 6.241449993689894e18);

        s.store_mul(772, 257, 183);

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.v[772] > 1e20) {
            if (s.v[772] < 1e26) {
            } else {
                s.store_scalar(772, 1e26);
            }
        } else {
            s.store_scalar(772, 1e20);
        }

        s.store_scalar(773, 0.0);

        s.b[1143] = (p.p52 > 0.0);
        s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });

        if s.b[1143] {
            s.store_scale_ad(773, A::powf(s.ad_value(769), 0.6666666666666666), ((0.4 * 5.951993) * p.p52));
        }

        s.b[1144] = (s.v[0] == (-1.0));
        s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });

        if (s.b[1143] && s.b[1144]) {
            s.store_scale(773, 773, (7.448711 / 5.951993));
        }

        s.store_scale(774, 769, (1e-8 * 1.0 / (s.v[767])));

        s.store_scale(775, 214, 0.5);

        s.store_scalar(776, 0.5);

        s.b[1145] = (s.v[0] == (-1.0));
        s.store_scalar(1145, if s.b[1145] { 1.0 } else { 0.0 });

        if s.b[1145] {
            s.store_scale(775, 214, 0.3333333333333333);
            s.store_scalar(776, 0.3333333333333333);
        }

        s.store_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(224)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(777, s.ad_value(1011), (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(259)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(778, s.ad_value(1011), (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_div_from_scalar(779, 1.0, 228);

        s.store_div(780, 768, 192);

        s.store_div(781, 768, 193);

        s.store_div_ad_lhs(782, A::sqrt_scaled_input(s.ad_value(194), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[355])), 780);

        s.store_div_ad_lhs(783, A::sqrt_scaled_input(s.ad_value(195), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[355])), 781);

        s.store_square(784, 782);

        s.store_square(785, 783);

        s.store_offset_div_ad(786, A::ln(A::offset(A::exp_scaled_input(s.ad_value(266), (0.005 * s.v[355])), (-1.0))), s.ad_value(266), (-((((((0.005 * s.v[355])) as f64).exp() - 1.0)) as f64).ln()));

        s.store_add_ad_lhs(787, A::ln_scaled_input(s.ad_value(782), 0.5), 786);

        s.store_add_ad_lhs(788, A::ln_scaled_input(s.ad_value(783), 0.5), 786);

        s.store_div_from_scalar(820, 1.0, 782);

        s.store_offset_scaled(821, 782, 3.1, 8.5);

        s.store_square(789, 821);

        s.store_scale(822, 821, 0.5);

        s.b[1146] = (s.v[820] < 0.06);
        s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });

        if s.b[1146] {
            s.store_scale(790, 820, 64.0);
        }

        s.b[1147] = (s.v[820] <= 0.45);
        s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });

        if ((!s.b[1146]) && s.b[1147]) {
            s.store_offset_scaled(790, 820, 22.0, 3.0);
        }

        s.b[1148] = (s.v[820] <= 1.6);
        s.store_scalar(1148, if s.b[1148] { 1.0 } else { 0.0 });

        if (((!s.b[1146]) && (!s.b[1147])) && s.b[1148]) {
            s.store_offset_scaled(790, 820, (-7.2), 15.5);
        }

        if (((!s.b[1146]) && (!s.b[1147])) && (!s.b[1148])) {
            s.copy_ad(790, 782);
        }

        s.store_add_scaled_inputs_product_right_ad(791, 822, 1.0, 784, 0.5, 782, A::sqrt(A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(784), 0.25, s.ad_value(790), 1.0)), (-1.0));

        s.store_div_from_scalar(820, 1.0, 783);

        s.store_offset_scaled(821, 783, 3.1, 8.5);

        s.store_square(792, 821);

        s.store_scale(822, 821, 0.5);

        s.b[1149] = (s.v[820] < 0.06);
        s.store_scalar(1149, if s.b[1149] { 1.0 } else { 0.0 });

        if s.b[1149] {
            s.store_scale(793, 820, 64.0);
        }

        s.b[1150] = (s.v[820] <= 0.45);
        s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });

        if ((!s.b[1149]) && s.b[1150]) {
            s.store_offset_scaled(793, 820, 22.0, 3.0);
        }

        s.b[1151] = (s.v[820] <= 1.6);
        s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });

        if (((!s.b[1149]) && (!s.b[1150])) && s.b[1151]) {
            s.store_offset_scaled(793, 820, (-7.2), 15.5);
        }

        if (((!s.b[1149]) && (!s.b[1150])) && (!s.b[1151])) {
            s.copy_ad(793, 783);
        }

        s.store_add_scaled_inputs_product_right_ad(794, 822, 1.0, 785, 0.5, 783, A::sqrt(A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(785), 0.25, s.ad_value(793), 1.0)), (-1.0));

        s.store_add_scaled_inputs_ad(728, A::offset(s.ad_value(187), s.v[362]), 1.0, A::ln_scaled_input(A::mul(s.ad_value(183), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]));

        if (!(s.v[728] > 0.05)) {
            s.store_scalar(728, 0.05);
        }

        s.store_div_ad_lhs(729, A::sqrt_scaled_input(s.ad_value(183), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);

        s.store_scalar(730, 0.0);

        s.store_scalar(731, 0.0);

        s.b[1152] = (s.v[188] > 0.0);
        s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });

        if s.b[1152] {
            s.store_div_from_scalar(732, 80000000.0, 770);
        }

        if s.b[1152] {
            if (s.v[188] > s.v[732]) {
                s.copy_ad(731, 188);
            } else {
                s.copy_ad(731, 732);
            }
        }

        if s.b[1152] {
            if (5e24 > s.v[731]) {
                s.store_scalar(731, 5e24);
            } else {
            }
        }

        if s.b[1152] {
            s.store_div_scaled_product_indices(730, 769, 769, (2.0 * s.v[715]), 731, (1.6021918e-19 * s.v[767]));
        }

        s.store_scalar(733, ((100.0 * s.v[715]) * s.v[715]));

        s.b[1153] = (p.p52 > 0.0);
        s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });

        if s.b[1153] {
            s.store_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(729), s.ad_value(729), s.ad_value(728), s.v[715]));
            s.store_mul_scaled_powf_rhs(735, 773, 0.75, 734, 0.6666666666666666);
            s.store_add(728, 728, 735);
            s.store_mul_offset_ad_rhs(729, 729, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0);
        }

        s.store_sqrt(736, 728);

        s.store_scale(737, 728, 0.95);

        s.store_scaled_mul(738, 728, 728, 0.0025);

        s.copy_ad(739, 738);

        s.store_scaled_sqrt(740, 739, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(741, 737, 0.5, 740, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(737), s.ad_value(740))), s.ad_value(738)), (-0.5));

        s.store_scaled_offset(742, 728, s.v[362], 0.5);

        s.store_sub_ad_lhs(743, A::sqrt(A::add(s.ad_value(185), s.ad_value(728))), 736);

        s.store_add_scaled_inputs3_sqrt_first_mixed_aii(744, A::add_scaled_inputs3(s.ad_value(185), 1.0, s.ad_value(186), 1.0, s.ad_value(728), 1.0), 1.0, 736, (-1.0), 743, -1.0);

        s.store_add_scaled_inputs3_offset_mixed_iia(745, 187, 1.0, 256, 1.0, A::ln_scaled_input(A::mul(s.ad_value(772), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]), s.v[362]);

        if (!(s.v[745] > 0.05)) {
            s.store_scalar(745, 0.05);
        }

        s.store_div_ad_lhs(746, A::sqrt_scaled_input(s.ad_value(772), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);

        s.b[1154] = (p.p52 > 0.0);
        s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });

        if s.b[1154] {
            s.store_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(746), s.ad_value(746), s.ad_value(745), s.v[715]));
            s.store_mul_scaled_powf_rhs(735, 773, 0.75, 734, 0.6666666666666666);
            s.store_add(745, 745, 735);
            s.store_mul_offset_ad_rhs(746, 746, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0);
        }

        s.store_scale(747, 745, 0.95);

        s.store_scaled_mul(748, 745, 745, 0.0025);

        s.copy_ad(749, 748);

        s.store_scaled_sqrt(740, 749, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(750, 747, 0.5, 740, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(747), s.ad_value(740))), s.ad_value(748)), (-0.5));

        s.store_offset_add_scaled_product(700, s.ad_value(177), 1.0, s.ad_value(178), A::scale_offset(s.ad_value(179), s.v[358], 1.0), s.v[358], s.v[21]);

        s.store_exp_scaled_input(751, 180, s.v[360]);

        s.store_mul(701, 189, 751);

        s.store_scale(702, 190, 1.0 / (s.v[359]));

        s.store_exp_scaled_input(752, 203, s.v[360]);

        s.store_mul(703, 202, 752);

        s.store_scaled_mul(716, 703, 769, s.v[20]);

        s.store_mul_ad_rhs(705, 206, A::exp_scaled_input(s.ad_value(207), s.v[360]));

        s.store_exp_scaled_input(753, 205, s.v[360]);

        s.store_mul(704, 204, 753);

        s.store_mul_ad_rhs(707, 210, A::exp_scaled_input(s.ad_value(211), s.v[360]));

        s.store_exp_scaled_input(754, 209, s.v[360]);

        s.store_mul(706, 208, 754);

        s.store_exp_scaled_input(755, 213, s.v[360]);

        s.store_mul(708, 212, 755);

        s.store_exp_scaled_input(756, 216, s.v[360]);

        s.store_mul(709, 215, 756);

        s.store_scaled_mul(757, 716, 709, 2.0);

        s.store_exp_scaled_input(758, 220, s.v[360]);

        s.store_mul(720, 219, 758);

        s.store_mul(721, 258, 758);

        s.store_mul_ad_rhs(712, 230, A::exp_scaled_input(s.ad_value(231), (-s.v[360])));

        s.store_scale(719, 276, (4.0 * (1.3806505e-23 * s.v[356])));

        s.b[1155] = ((p.p46 != 0.0) && (s.v[287] > 0.0));
        s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });

        if s.b[1155] {
            s.store_offset_add_scaled_inputs_indices(713, 282, 1.0, 283, s.v[358], s.v[23]);
            s.store_exp_scaled_input(759, 288, s.v[360]);
            s.store_mul(714, 287, 759);
            s.store_scaled_mul(717, 714, 769, s.v[22]);
            s.store_offset_scaled(723, 286, ((s.v[359]) * (s.v[715])), s.v[715]);
            s.store_add_scaled_product_mixed_aia(760, A::offset(s.ad_value(284), s.v[362]), 1.0, 723, A::ln_scaled_input(A::mul(s.ad_value(285), A::powf(s.ad_value(363), (-0.75))), 4e-26), 2.0);
        }

        if s.b[1155] {
            if (s.v[760] > 0.05) {
            } else {
                s.store_scalar(760, 0.05);
            }
        }

        if s.b[1155] {
            s.store_div_ad_lhs(761, A::sqrt_scaled_input(s.ad_value(285), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);
            s.store_square(724, 761);
            s.store_ln(725, 724);
            s.store_scale(762, 760, 0.95);
            s.store_scaled_mul(763, 760, 760, 0.0025);
            s.copy_ad(764, 763);
            s.store_scaled_sqrt(765, 764, 0.5);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(766, 762, 0.5, 765, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(762), s.ad_value(765))), s.ad_value(763)), (-0.5));
        }

        if (!s.b[1155]) {
            s.store_scalar(713, 0.0);
            s.store_scalar(759, 1.0);
            s.store_scalar(714, 0.0);
            s.store_scalar(717, 0.0);
            s.store_scalar(723, s.v[715]);
            s.store_scalar(760, 0.0);
            s.store_scalar(761, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 0.0);
            s.store_scalar(762, 0.0);
            s.store_scalar(763, 0.0);
            s.store_scalar(764, 0.0);
            s.store_scalar(765, 0.0);
            s.store_scalar(766, 0.0);
        }

        s.store_div_from_scalar(795, 1.0, 246);

        s.store_scaled_sqrt_scaled_input(796, 246, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(797, 796, 181);

        s.store_mul(798, 796, 192);

        s.store_mul(799, 796, 193);

        s.store_scalar(800, 0.0);

        s.b[1156] = (s.v[241] < 0.0);
        s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });

        if s.b[1156] {
            s.store_div_scaled_inputs_indices(800, 240, (-0.495), 241, 1.0);
        }

        s.store_scalar(801, 0.0);

        s.b[1157] = (s.v[243] < 0.0);
        s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });

        if s.b[1157] {
            s.store_div_scaled_inputs_indices(801, 242, (-0.495), 243, 1.0);
        }

        s.b[1158] = (s.v[245] < 0.0);
        s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });

        if s.b[1158] {
            s.store_div_scaled_inputs_indices(802, 244, (-0.495), 245, 1.0);
        }

        s.store_pow_from_scalar_ad(803, s.v[352], s.ad_value(239));

        s.store_mul(236, 236, 803);

        s.store_mul(237, 237, 803);

        s.store_mul(238, 238, 803);

        if ((1.0 + (s.v[251] * s.v[353])) > 0.0) {
            s.store_offset_scaled(796, 251, s.v[353], 1.0);
        } else {
            s.store_scalar(796, 0.0);
        }

        s.store_mul(710, 249, 796);

        s.store_scaled_mul(806, 710, 192, 500000000.0);

        if ((1.0 + (s.v[252] * s.v[353])) > 0.0) {
            s.store_offset_scaled(796, 252, s.v[353], 1.0);
        } else {
            s.store_scalar(796, 0.0);
        }

        s.store_mul(711, 250, 796);

        s.store_scaled_mul(807, 711, 193, 500000000.0);

        s.store_scalar(808, 0.0);

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1159] = (s.v[272] > 1e-10);
        s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });

        if s.b[1159] {
            s.store_div_from_scalar(808, 0.75, 272);
        }

        s.store_square(809, 273);

        s.store_scale(24, 6, s.v[646]);

        s.store_scale(25, 6, s.v[647]);

        s.store_scale(26, 6, s.v[648]);

        s.store_scale(27, 6, s.v[673]);

        s.store_scale(28, 6, s.v[674]);

        s.store_scale(29, 6, s.v[675]);

        s.store_scalar(30, 0.0);

        s.b[1167] = (p.p43 == 3.0);
        s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });

        if s.b[1167] {
            s.store_scalar(30, 1.0);
        }

        s.copy_ad(31, 313);

        s.b[1168] = (p.p39 == 0.0);
        s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });

        if s.b[1168] {
            s.store_scalar(31, (if (s.v[14] > 0.0) { s.v[14] } else { 0.0 }));
        }

        s.b[1169] = ((p.p43 == 2.0) || (p.p43 == 3.0));
        s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });

        if s.b[1169] {
            s.store_scale(24, 6, s.v[649]);
            s.store_add_scaled_product_indices(25, 6, s.v[650], 30, 31, (-1.0));
            s.copy_ad(26, 31);
            s.store_scale(27, 6, s.v[676]);
            s.store_add_scaled_product_indices(28, 6, s.v[677], 30, 31, (-1.0));
            s.copy_ad(29, 31);
        }

        s.b[1170] = (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0));
        s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });

        if s.b[1170] {
            if (s.v[24] > 0.0) {
                s.copy_ad(646, 24);
            } else {
                s.store_scalar(646, 0.0);
            }
        }

        if s.b[1170] {
            if (s.v[25] > 0.0) {
                s.copy_ad(647, 25);
            } else {
                s.store_scalar(647, 0.0);
            }
        }

        if s.b[1170] {
            if (s.v[26] > 0.0) {
                s.copy_ad(648, 26);
            } else {
                s.store_scalar(648, 0.0);
            }
        }

        if s.b[1170] {
            if (s.v[27] > 0.0) {
                s.copy_ad(673, 27);
            } else {
                s.store_scalar(673, 0.0);
            }
        }

        if s.b[1170] {
            if (s.v[28] > 0.0) {
                s.copy_ad(674, 28);
            } else {
                s.store_scalar(674, 0.0);
            }
        }

        if s.b[1170] {
            if (s.v[29] > 0.0) {
                s.copy_ad(675, 29);
            } else {
                s.store_scalar(675, 0.0);
            }
        }

        if (!s.b[1170]) {
            s.store_scalar(646, 0.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(673, 0.0);
            s.store_scalar(674, 0.0);
            s.store_scalar(675, 0.0);
        }

        s.store_scalar(656, 0.0);

        s.store_scalar(683, 0.0);

        s.store_scalar(658, 0.0);

        s.store_scalar(685, 0.0);

        s.store_scalar(657, 0.0);

        s.store_scalar(684, 0.0);

        s.store_scalar(659, 0.0);

        s.store_scalar(686, 0.0);

        s.store_scalar(654, 0.0);

        s.store_scalar(681, 0.0);

        s.store_scalar(655, 0.0);

        s.store_scalar(682, 0.0);

        s.store_scalar(651, 1.0);

        s.store_scalar(678, 1.0);

        s.store_scalar(652, 1.0);

        s.store_scalar(679, 1.0);

        s.store_scalar(653, 1.0);

        s.store_scalar(680, 1.0);

        s.store_scalar(501, 0.0);

        s.b[1171] = (p.p43 > 0.0);
        s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });

        s.b[1172] = ((s.v[387] * s.v[646]) > 0.0);
        s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1172]) {
            s.store_scaled_ln_ad(454, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(646), s.v[387])), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1172])) {
            s.store_scalar(454, 100000000.0);
        }

        s.b[1173] = ((s.v[388] * s.v[647]) > 0.0);
        s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1173]) {
            s.store_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(647), s.v[388])), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1173])) {
            s.store_scalar(455, 100000000.0);
        }

        s.b[1174] = ((s.v[389] * s.v[648]) > 0.0);
        s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1174]) {
            s.store_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(648), s.v[389])), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1174])) {
            s.store_scalar(456, 100000000.0);
        }

        if s.b[1171] {
            s.store_min3(654, 454, 455, 456);
        }

        s.b[1175] = ((((s.v[654] * s.v[371])) as f64).abs() < 230.25850929940458);
        s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1175]) {
            s.store_exp_scaled_input(655, 654, s.v[371]);
        }

        s.b[1176] = ((s.v[654] * s.v[371]) < 0.0);
        s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });

        if ((s.b[1171] && (!s.b[1175])) && s.b[1176]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(655, 1e-100, (-230.25850929940458), A::scale(s.ad_value(654), s.v[371]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((s.b[1171] && (!s.b[1175])) && (!s.b[1176])) {
            s.store_scaled_offset_ad(655, A::mul_offset_rhs(A::scale_offset(s.ad_value(654), s.v[371], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(654), s.v[371], (-230.25850929940458)), A::scale_offset(s.ad_value(654), ((s.v[371]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1171] {
            s.store_scalar(396, s.v[393]);
            s.store_scalar(397, s.v[394]);
            s.store_scalar(398, s.v[395]);
            s.store_scalar(399, p.p831);
            s.store_scalar(400, p.p832);
            s.store_scalar(401, p.p833);
            s.store_scalar(402, p.p828);
            s.store_scalar(403, p.p829);
            s.store_scalar(404, p.p830);
        }

        s.b[1177] = (s.v[646] == 0.0);
        s.store_scalar(1177, if s.b[1177] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1177]) {
            s.store_scalar(396, (s.v[394] + s.v[395]));
            s.store_scalar(399, (0.9 * (p.p832).min(p.p833)));
            s.store_scalar(402, (p.p829 + p.p830));
        }

        s.b[1178] = (s.v[647] == 0.0);
        s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1178]) {
            s.store_scalar(397, (s.v[393] + s.v[395]));
            s.store_scalar(400, (0.9 * (p.p831).min(p.p833)));
            s.store_scalar(403, (p.p828 + p.p830));
        }

        s.b[1179] = (s.v[648] == 0.0);
        s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1179]) {
            s.store_scalar(398, (s.v[393] + s.v[394]));
            s.store_scalar(401, (0.9 * (p.p831).min(p.p832)));
            s.store_scalar(404, (p.p828 + p.p829));
        }

        if s.b[1171] {
            s.store_min3(656, 396, 397, 398);
            s.store_scale(657, 656, 0.1);
            s.store_max3(377, 399, 400, 401);
            s.store_mul_sub_from_scalar_ad_rhs(658, 656, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377))));
            s.store_offset_min_ad(659, A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404), (-0.05));
        }

        s.b[1180] = ((s.v[563] * s.v[673]) > 0.0);
        s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1180]) {
            s.store_scaled_ln_ad(454, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(563), s.ad_value(673))), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1180])) {
            s.store_scalar(454, 100000000.0);
        }

        s.b[1181] = ((s.v[564] * s.v[674]) > 0.0);
        s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1181]) {
            s.store_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(564), s.ad_value(674))), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1181])) {
            s.store_scalar(455, 100000000.0);
        }

        s.b[1182] = ((s.v[565] * s.v[675]) > 0.0);
        s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1182]) {
            s.store_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(565), s.ad_value(675))), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1182])) {
            s.store_scalar(456, 100000000.0);
        }

        if s.b[1171] {
            s.store_min3(681, 454, 455, 456);
        }

        s.b[1183] = ((((s.v[681] * s.v[371])) as f64).abs() < 230.25850929940458);
        s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1183]) {
            s.store_exp_scaled_input(682, 681, s.v[371]);
        }

        s.b[1184] = ((s.v[681] * s.v[371]) < 0.0);
        s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });

        if ((s.b[1171] && (!s.b[1183])) && s.b[1184]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(682, 1e-100, (-230.25850929940458), A::scale(s.ad_value(681), s.v[371]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((s.b[1171] && (!s.b[1183])) && (!s.b[1184])) {
            s.store_scaled_offset_ad(682, A::mul_offset_rhs(A::scale_offset(s.ad_value(681), s.v[371], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(681), s.v[371], (-230.25850929940458)), A::scale_offset(s.ad_value(681), ((s.v[371]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1171] {
            s.copy_ad(396, 569);
            s.copy_ad(397, 570);
            s.copy_ad(398, 571);
            s.copy_ad(399, 511);
            s.copy_ad(400, 512);
            s.copy_ad(401, 513);
            s.copy_ad(402, 508);
            s.copy_ad(403, 509);
            s.copy_ad(404, 510);
        }

        s.b[1185] = (s.v[673] == 0.0);
        s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1185]) {
            s.store_add(396, 570, 571);
            s.store_scale_ad(399, A::min(s.ad_value(512), s.ad_value(513)), 0.9);
            s.store_add(402, 509, 510);
        }

        s.b[1186] = (s.v[674] == 0.0);
        s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1186]) {
            s.store_add(397, 569, 571);
            s.store_scale_ad(400, A::min(s.ad_value(511), s.ad_value(513)), 0.9);
            s.store_add(403, 508, 510);
        }

        s.b[1187] = (s.v[675] == 0.0);
        s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });

        if (s.b[1171] && s.b[1187]) {
            s.store_add(398, 569, 570);
            s.store_scale_ad(401, A::min(s.ad_value(511), s.ad_value(512)), 0.9);
            s.store_add(404, 508, 509);
        }

        if s.b[1171] {
            s.store_min3(683, 396, 397, 398);
            s.store_scale(684, 683, 0.1);
            s.store_max3(377, 399, 400, 401);
            s.store_mul_sub_from_scalar_ad_rhs(685, 683, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377))));
            s.store_offset_min_ad(686, A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404), (-0.05));
        }

        s.b[1188] = (s.v[474] == 1.0);
        s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1171] && s.b[1188]) {
            s.store_add_scaled_inputs3_indices(501, 646, (s.v[414] * p.p929), 647, (s.v[415] * p.p929), 648, (s.v[416] * p.p929));
        }

        s.b[1523] = ((s.v[646] * s.v[414]) <= s.v[501]);
        s.store_scalar(1523, if s.b[1523] { 1.0 } else { 0.0 });

        if ((s.b[1171] && s.b[1188]) && s.b[1523]) {
            s.store_scalar(651, 0.0);
        }

        s.b[1524] = ((s.v[647] * s.v[415]) <= s.v[501]);
        s.store_scalar(1524, if s.b[1524] { 1.0 } else { 0.0 });

        if ((s.b[1171] && s.b[1188]) && s.b[1524]) {
            s.store_scalar(652, 0.0);
        }

        s.b[1525] = ((s.v[648] * s.v[416]) <= s.v[501]);
        s.store_scalar(1525, if s.b[1525] { 1.0 } else { 0.0 });

        if ((s.b[1171] && s.b[1188]) && s.b[1525]) {
            s.store_scalar(653, 0.0);
        }

        if (s.b[1171] && s.b[1188]) {
            s.store_mul_ad_rhs(501, 553, A::add_scaled_products3(s.ad_value(673), s.ad_value(581), 1.0, s.ad_value(674), s.ad_value(582), 1.0, s.ad_value(675), s.ad_value(583), 1.0));
        }

        s.b[1813] = ((s.v[673] * s.v[581]) <= s.v[501]);
        s.store_scalar(1813, if s.b[1813] { 1.0 } else { 0.0 });

        if ((s.b[1171] && s.b[1188]) && s.b[1813]) {
            s.store_scalar(678, 0.0);
        }

        s.b[1814] = ((s.v[674] * s.v[582]) <= s.v[501]);
        s.store_scalar(1814, if s.b[1814] { 1.0 } else { 0.0 });

        if ((s.b[1171] && s.b[1188]) && s.b[1814]) {
            s.store_scalar(679, 0.0);
        }

        s.b[1815] = ((s.v[675] * s.v[583]) <= s.v[501]);
        s.store_scalar(1815, if s.b[1815] { 1.0 } else { 0.0 });

        if ((s.b[1171] && s.b[1188]) && s.b[1815]) {
            s.store_scalar(680, 0.0);
        }

        s.store_scalar(2027, 0.0);

        s.store_scalar(2028, 0.0);

        s.store_scalar(2029, 0.0);

        s.store_scalar(1937, 1.0);

        s.store_scalar(1936, 0.0);

        s.b[2102] = (s.v[0] == 1.0);
        s.store_scalar(2102, if s.b[2102] { 1.0 } else { 0.0 });

        if s.b[2102] {
            s.store_voltage(825, ctx, nodes, Some(5), Some(6));
            s.store_voltage(826, ctx, nodes, Some(7), Some(6));
            s.store_voltage(827, ctx, nodes, Some(6), Some(8));
            s.store_scaled_voltage(832, ctx, nodes, Some(6), Some(10), -1.0);
            s.store_scaled_voltage(833, ctx, nodes, Some(7), Some(11), -1.0);
        }

        if (!s.b[2102]) {
            s.store_scaled_voltage(825, ctx, nodes, Some(5), Some(6), -1.0);
            s.store_scaled_voltage(826, ctx, nodes, Some(7), Some(6), -1.0);
            s.store_scaled_voltage(827, ctx, nodes, Some(6), Some(8), -1.0);
            s.store_voltage(832, ctx, nodes, Some(6), Some(10));
            s.store_voltage(833, ctx, nodes, Some(7), Some(11));
        }

        s.store_add(829, 825, 827);

        s.copy_ad(834, 825);

        s.copy_ad(835, 827);

        s.store_add(836, 826, 827);

        s.store_sub(837, 825, 826);

        s.store_scale(1817, 834, (-s.v[355]));

        s.store_scale(1818, 837, (-s.v[355]));

        s.store_scaled_sub(1819, 829, 700, (-s.v[355]));

        s.store_scalar(831, 1.0);

        s.b[2103] = (s.v[826] < 0.0);
        s.store_scalar(2103, if s.b[2103] { 1.0 } else { 0.0 });

        if s.b[2103] {
            s.store_scalar(831, (-1.0));
            s.store_sub(825, 825, 826);
            s.store_add(827, 827, 826);
            s.store_neg(826, 826);
        }

        s.store_add(828, 826, 827);

        s.store_div_scaled_product_offset_denominator(830, s.ad_value(826), s.ad_value(826), 1.0, A::sqrt_square_offset(s.ad_value(826), 0.01), 0.1, 1.0);

        s.store_add_scaled_inputs4_mixed_iiai(2107, 828, 0.5, 827, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(828), s.ad_value(827))), s.ad_value(739))), (-0.5), 737, 1.0);

        s.copy_ad(1820, 2107);

        s.store_add_scaled_inputs4_mixed_iiai(2030, 827, 1.0, 2107, (-0.5), A::sqrt(A::add(A::square(s.ad_value(2107)), s.ad_value(738))), (-(-0.5)), 741, 1.0);

        s.copy_ad(1821, 2030);

        s.store_scalar(2031, 0.0);

        s.b[2263] = ((p.p45 != 0.0) && (s.v[184] != 1.0));
        s.store_scalar(2263, if s.b[2263] { 1.0 } else { 0.0 });

        if s.b[2263] {
            s.store_add_scaled_inputs3_indices(2032, 2030, 1.0, 826, 0.5, 830, (-0.5));
            s.store_sub_ad_lhs(2033, A::sqrt(A::add(s.ad_value(2032), s.ad_value(728))), 736);
            s.store_offset_div_scaled_inputs2_indices(2027, 2033, 2.0, 743, (-2.0), 744, 1.0, (-1.0));
            s.store_add_scaled_product_mixed_iaa(2034, 2033, 1.0, A::mul_sub_from_scalar_lhs_scaled_output(1.0, s.ad_value(184), s.ad_value(744), 0.25), A::add(s.ad_value(2027), A::sqrt_square_offset(s.ad_value(2027), 0.4804530139182)), (-1.0));
            s.store_add_scaled_square_product_indices(2035, 2034, 1.0, 736, 2034, 2.0);
            s.store_add_scaled_inputs3_indices(2030, 2035, 1.0, 826, (-0.5), 830, (-(-0.5)));
            s.store_sub(2031, 1821, 2030);
        }

        s.copy_ad(2104, 728);

        s.copy_ad(2105, 738);

        s.copy_ad(2106, 729);

        s.copy_ad(2108, 2030);

        s.copy_ad(2112, 2031);

        s.copy_ad(2109, 720);

        s.copy_ad(2110, 777);

        s.store_add_scaled_inputs3_indices(2111, 829, 1.0, 2112, (-1.0), 700, -1.0);

        s.store_add_scaled_inputs3_indices(2113, 2108, 1.0, 826, 0.5, 830, (-0.5));

        s.store_scalar(2125, 1.0);

        s.b[2264] = (s.v[190] > 0.0);
        s.store_scalar(2264, if s.b[2264] { 1.0 } else { 0.0 });

        if s.b[2264] {
            s.store_scale(2116, 2104, s.v[361]);
            s.store_scale(2117, 2113, s.v[361]);
            s.store_scale(2118, 2111, s.v[361]);
            s.store_offset_div_scaled_inputs_mixed_ia(2028, 2106, 0.5, A::sqrt(s.ad_value(2116)), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(2029, 2116, 1.0, 2106, A::sqrt(s.ad_value(2116)), 1.0);
            s.store_add_scaled_inputs_product_mixed_aiai(2119, A::div_scaled_inputs2(s.ad_value(2118), 1.0, s.ad_value(2029), (-1.0), s.ad_value(2028), 1.0), 1.0, 2116, 0.5, A::offset(s.ad_value(191), 1.0), 2117, (-1.0));
            s.store_offset_scaled(2120, 2116, 0.5, 2.0);
            s.store_add(2121, 2116, 2117);
            s.store_sub_scaled_inputs_ad(2028, A::add_scaled_inputs_product(s.ad_value(2118), 1.0, s.ad_value(2121), (-1.0), s.ad_value(2106), A::sqrt(s.ad_value(2121)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2116), s.ad_value(2106)), A::sqrt(s.ad_value(2116)))), 2.0);
            s.store_add_scaled_inputs(2122, 2028, 2.0, 2120, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2119, 0.5, 2122, 0.5, 2119, 2122, 20.0, 0.5);
            s.store_add_scaled_inputs3_indices(2029, 2118, 2.0, 2117, (-2.0), 2120, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2123, 2028, 0.5, 2029, 0.5, 2028, 2029, 20.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2123, 0.5, 2120, 0.5, 2123, 2120, 5.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2124, 2028, 0.5, 2120, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2120), -1.0)), 20.0), 0.5);
            s.store_mul_offset_ad_rhs(2029, 702, A::div(s.ad_value(2124), s.ad_value(2120)), 1.0);
        }

        s.b[2265] = (s.v[2029] > (-230.25850929940458));
        s.store_scalar(2265, if s.b[2265] { 1.0 } else { 0.0 });

        if (s.b[2264] && s.b[2265]) {
            s.store_exp(2125, 2029);
        }

        if (s.b[2264] && (!s.b[2265])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2125, 1e-100, (-230.25850929940458), 2029, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.store_offset_mul(2126, 701, 2125, 1.0);

        s.store_scale(2127, 2126, s.v[715]);

        s.store_mul_ad_product_rhs(2128, 199, A::offset(A::mul(s.ad_value(201), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(200), s.ad_value(2113)), 1.0));

        s.store_mul_offset_rhs(2129, 2127, 2128, 1.0);

        s.store_div_from_scalar(2130, 1.0, 2129);

        s.store_mul_ad_rhs(2114, 2106, A::sqrt_scaled_input(s.ad_value(2130), s.v[715]));

        s.store_square(2115, 2114);

        s.store_div_from_scalar(2131, 1.0, 2115);

        s.store_mul(2132, 2108, 2130);

        s.store_mul(2133, 2111, 2130);

        s.store_div_scaled_value_offset_denominator(2134, s.ad_value(830), 2.0, A::sqrt_product_offset(s.ad_value(197), s.ad_value(830), 1.0), 1.0, 1.0);

        s.store_mul_ad_product_rhs_mixed_ia(2135, 196, 2134, A::offset(A::mul(s.ad_value(198), s.ad_value(2113)), 1.0));

        s.store_mul(2136, 2104, 2130);

        s.store_sqrt_square_add(2028, 2107, 2105);

        s.store_sqrt_add_ad(2029, A::square(A::sub(s.ad_value(2107), s.ad_value(2135))), s.ad_value(2105));

        s.store_mul_add_scaled_inputs3_offset_rhs(2137, 2130, s.ad_value(2135), 0.5, s.ad_value(2028), 0.5, s.ad_value(2029), ((-1.0) * (0.5)), 0.0);

        s.store_add(2138, 2136, 2132);

        s.store_sub(2139, 2138, 2137);

        s.b[2266] = (p.p45 > 0.0);
        s.store_scalar(2266, if s.b[2266] { 1.0 } else { 0.0 });

        s.b[2267] = (((s.v[2139]) as f64).abs() < 1e-5);
        s.store_scalar(2267, if s.b[2267] { 1.0 } else { 0.0 });

        if (s.b[2266] && s.b[2267]) {
            s.store_offset_ad(2140, A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2139), 1.0, A::scale(s.ad_value(2139), 0.3125), 0.5)), 1.0);
        }

        s.b[2268] = (s.v[2139] < 460.51701859880916);
        s.store_scalar(2268, if s.b[2268] { 1.0 } else { 0.0 });

        if ((s.b[2266] && (!s.b[2267])) && s.b[2268]) {
            s.store_exp_neg_input(2154, 2139);
        }

        if ((s.b[2266] && (!s.b[2267])) && (!s.b[2268])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2154, 1e-200, 2139, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (s.b[2266] && (!s.b[2267])) {
            s.store_scalar(2027, (if (s.v[2139] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[2266] && (!s.b[2267])) {
            s.store_offset_ad(2140, A::div_scaled_product3(s.ad_value(2027), s.ad_value(2114), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2154), 1.0, s.ad_value(2139))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2139), 1.0, s.ad_value(2154))), 2.0), 1.0);
        }

        if (!s.b[2266]) {
            s.store_offset_div_scaled_inputs_mixed_ia(2140, 2114, 0.5, A::sqrt(s.ad_value(2139)), 1.0, 1.0);
        }

        s.store_add_scaled_value_products(2141, s.ad_value(2139), 1.0, s.ad_value(2114), A::sqrt(s.ad_value(2139)), 1.0, s.ad_value(2140), A::ln(A::offset(s.ad_value(2140), (-1.0))), (-1.0));

        s.store_div_scaled_inputs2_indices(2142, 2133, 1.0, 2141, (-1.0), 2140, 1.0);

        s.store_mul_scaled_offset_ad_rhs(2148, 2115, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2115)), 1.0)), (-1.0));

        s.store_scalar(2147, 0.0);

        s.store_scalar(2149, 1.0);

        s.b[2269] = (s.v[2142] > (-30.0));
        s.store_scalar(2269, if s.b[2269] { 1.0 } else { 0.0 });

        if s.b[2269] {
            s.store_offset_mul(2143, 2140, 2142, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2027, 2143, 2143, 10.0, 0.5);
            s.store_sub_ad_rhs(2144, 2142, A::ln(s.ad_value(2027)));
            s.store_scaled_add_sqrt_square_offset_rhs(2145, 2144, 2144, 2.0, 0.5);
        }

        s.b[2270] = ((s.v[2142] - s.v[2145]) < 230.25850929940458);
        s.store_scalar(2270, if s.b[2270] { 1.0 } else { 0.0 });

        if (s.b[2269] && s.b[2270]) {
            s.store_exp_sub(2027, 2142, 2145);
        }

        if (s.b[2269] && (!s.b[2270])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::sub(s.ad_value(2142), s.ad_value(2145)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if s.b[2269] {
            s.store_div(2146, 2027, 2140);
            s.store_sub_ad_lhs(2027, A::scaled_offset(s.ad_value(2145), 1.0, 2.0), 2146);
        }

        s.b[2271] = (s.v[2146] > 1e-6);
        s.store_scalar(2271, if s.b[2271] { 1.0 } else { 0.0 });

        if (s.b[2269] && s.b[2271]) {
            s.store_mul_offset_ad_rhs(2147, 2140, A::sub(s.ad_value(2145), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2146), s.ad_value(2027), 1.0), 1.0, (-1.0), s.ad_value(2146), 1.0)), 1.0);
        }

        if (s.b[2269] && (!s.b[2271])) {
            s.store_mul_ad_affine_product_rhs(2147, 2140, s.ad_value(2146), A::offset(A::mul_scaled_lhs(s.ad_value(2027), 0.25, s.ad_value(2027)), 1.0), 0.5, 0.0);
        }

        if s.b[2269] {
            s.store_add_scaled_inputs3_offset_mixed_iia(2027, 2133, 0.5, 2147, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2133), s.ad_value(2147)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_offset_ad_rhs(2148, 2115, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2115)), s.ad_value(2027), 1.0), (-1.0));
            s.store_div_add_scaled_inputs_rhs_indices(2149, 2148, 2148, 1.0, 2147, 1.0);
            s.store_add_scaled_product_indices(2139, 2138, 1.0, 2149, 2137, (-1.0));
        }

        s.store_offset_scaled(2150, 2114, 0.7071067811865475, 1.0);

        s.store_scale(2151, 2150, 1e-5);

        s.store_div_from_scalar(2152, 1.0, 2150);

        s.store_scalar(2259, 0.0);

        s.store_scalar(2153, 0.0);

        s.b[2272] = (s.v[2139] < 460.51701859880916);
        s.store_scalar(2272, if s.b[2272] { 1.0 } else { 0.0 });

        if s.b[2272] {
            s.store_exp_neg_input(2154, 2139);
        }

        if (!s.b[2272]) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2154, 1e-200, 2139, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        s.b[2273] = (((s.v[2133]) as f64).abs() <= s.v[2151]);
        s.store_scalar(2273, if s.b[2273] { 1.0 } else { 0.0 });

        if s.b[2273] {
            s.store_scaled_square(2239, 2152, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(2153, 2133, 2152, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2133), 1.0, s.ad_value(2154)), s.ad_value(2114), s.ad_value(2239)), 1.0));
        }

        s.b[2274] = (s.v[2133] < (-s.v[2151]));
        s.store_scalar(2274, if s.b[2274] { 1.0 } else { 0.0 });

        if ((!s.b[2273]) && s.b[2274]) {
            s.store_neg(2241, 2133);
            s.store_scaled_mul(2242, 2241, 2152, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(2243, 2242, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(2238, 2241, 2243);
            s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::offset(s.ad_value(2243), 1.0), 1.0);
            s.store_sub_scaled_inputs(2245, 2238, 2.0, 2115, 1.0);
            s.store_sub_ln_mul_lhs(2246, 2244, 2131, 2243);
            s.store_add(824, 2244, 2245);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::sub_scaled_inputs(A::square(s.ad_value(2245)), 0.5, s.ad_value(2244), 1.0), 1.0);
            s.store_add_ad_rhs(2247, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::sub_scaled_inputs(A::square(s.ad_value(2245)), 0.3333333333333333, s.ad_value(2244), 1.0))), 1.0));
        }

        s.b[2275] = (s.v[2247] < 230.25850929940458);
        s.store_scalar(2275, if s.b[2275] { 1.0 } else { 0.0 });

        if (((!s.b[2273]) && s.b[2274]) && s.b[2275]) {
            s.store_exp(2248, 2247);
        }

    }
}
