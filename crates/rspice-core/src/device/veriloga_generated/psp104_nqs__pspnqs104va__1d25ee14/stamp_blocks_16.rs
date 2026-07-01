#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
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
        p: &Parameters,
        var_alphaav_slot: &mut f64,
        var_alphaav_rv_slot: &mut f64,
        var_chnl_type_slot: &mut f64,
        var_chnl_type_rv_slot: &mut f64,
        var_cjorbotd_i_slot: &mut f64,
        var_cjorbotd_i_rv_slot: &mut f64,
        var_cjorgat2nd_slot: &mut f64,
        var_cjorgat2nd_rv_slot: &mut f64,
        var_cjorgatd_i_slot: &mut f64,
        var_cjorgatd_i_rv_slot: &mut f64,
        var_cjorstid_i_slot: &mut f64,
        var_cjorstid_i_rv_slot: &mut f64,
        var_deltaphigr_slot: &mut f64,
        var_deltaphigr_rv_slot: &mut f64,
        var_epssi_slot: &mut f64,
        var_epssi_rv_slot: &mut f64,
        var_guard1_slot: &mut f64,
        var_guard10_slot: &mut f64,
        var_guard10_rv_slot: &mut f64,
        var_guard1_rv_slot: &mut f64,
        var_guard2_slot: &mut f64,
        var_guard2_rv_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_guard3_rv_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_guard4_rv_slot: &mut f64,
        var_guard5_slot: &mut f64,
        var_guard5_rv_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_guard6_rv_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard7_rv_slot: &mut f64,
        var_guard8_slot: &mut f64,
        var_guard8_rv_slot: &mut f64,
        var_guard9_slot: &mut f64,
        var_guard9_rv_slot: &mut f64,
        var_kbol_over_qele_slot: &mut f64,
        var_kbol_over_qele_rv_slot: &mut f64,
        var_one_minus_pbot_slot: &mut f64,
        var_one_minus_pbot_rv_slot: &mut f64,
        var_one_minus_pgat_slot: &mut f64,
        var_one_minus_pgat2nd_slot: &mut f64,
        var_one_minus_pgat2nd_rv_slot: &mut f64,
        var_one_minus_pgat_rv_slot: &mut f64,
        var_one_minus_psti_slot: &mut f64,
        var_one_minus_psti_rv_slot: &mut f64,
        var_one_over_one_minus_pbot_slot: &mut f64,
        var_one_over_one_minus_pbot_rv_slot: &mut f64,
        var_one_over_one_minus_pgat_slot: &mut f64,
        var_one_over_one_minus_pgat2nd_slot: &mut f64,
        var_one_over_one_minus_pgat2nd_rv_slot: &mut f64,
        var_one_over_one_minus_pgat_rv_slot: &mut f64,
        var_one_over_one_minus_psti_slot: &mut f64,
        var_one_over_one_minus_psti_rv_slot: &mut f64,
        var_pbotd_i_slot: &mut f64,
        var_pbotd_i_rv_slot: &mut f64,
        var_pgat2nd_slot: &mut f64,
        var_pgat2nd_rv_slot: &mut f64,
        var_pgatd_i_slot: &mut f64,
        var_pgatd_i_rv_slot: &mut f64,
        var_phiggat2nd_slot: &mut f64,
        var_phiggat2nd_rv_slot: &mut f64,
        var_phigrbot_slot: &mut f64,
        var_phigrbot_rv_slot: &mut f64,
        var_phigrgat_slot: &mut f64,
        var_phigrgat2nd_slot: &mut f64,
        var_phigrgat2nd_rv_slot: &mut f64,
        var_phigrgat_rv_slot: &mut f64,
        var_phigrsti_slot: &mut f64,
        var_phigrsti_rv_slot: &mut f64,
        var_phitr_slot: &mut f64,
        var_phitr_rv_slot: &mut f64,
        var_phitrinv_slot: &mut f64,
        var_phitrinv_rv_slot: &mut f64,
        var_pstid_i_slot: &mut f64,
        var_pstid_i_rv_slot: &mut f64,
        var_swgat2nd_slot: &mut f64,
        var_swgat2nd_rv_slot: &mut f64,
        var_swjunexp_i_slot: &mut f64,
        var_swjunexp_i_rv_slot: &mut f64,
        var_swnqs_i_slot: &mut f64,
        var_swnqs_i_rv_slot: &mut f64,
        var_tkr_slot: &mut f64,
        var_tkr_1_slot: &mut f64,
        var_tkr_1_rv_slot: &mut f64,
        var_tkr_rv_slot: &mut f64,
        var_vbirbotd_i_slot: &mut f64,
        var_vbirbotd_i_rv_slot: &mut f64,
        var_vbirbotinv_slot: &mut f64,
        var_vbirbotinv_rv_slot: &mut f64,
        var_vbirgat2nd_slot: &mut f64,
        var_vbirgat2nd_rv_slot: &mut f64,
        var_vbirgatd_i_slot: &mut f64,
        var_vbirgatd_i_rv_slot: &mut f64,
        var_vbirgatinv_slot: &mut f64,
        var_vbirgatinv_rv_slot: &mut f64,
        var_vbirstid_i_slot: &mut f64,
        var_vbirstid_i_rv_slot: &mut f64,
        var_vbirstiinv_slot: &mut f64,
        var_vbirstiinv_rv_slot: &mut f64,
        var_vbrinvbot_slot: &mut f64,
        var_vbrinvbot_rv_slot: &mut f64,
        var_vbrinvgat_slot: &mut f64,
        var_vbrinvgat_dn5_slot: &mut f64,
        var_vbrinvgat_dn6_slot: &mut f64,
        var_vbrinvgat_dn7_slot: &mut f64,
        var_vbrinvgat_dn8_slot: &mut f64,
        var_vbrinvgat_rv_slot: &mut f64,
        var_vbrinvsti_slot: &mut f64,
        var_vbrinvsti_rv_slot: &mut f64,
        var_vnorm_slot: &mut f64,
        var_vnorm_inv_slot: &mut f64,
        var_vnorm_inv_rv_slot: &mut f64,
        var_vnorm_rv_slot: &mut f64,
        var_wdepnulrbot_slot: &mut f64,
        var_wdepnulrbot_rv_slot: &mut f64,
        var_wdepnulrgat_slot: &mut f64,
        var_wdepnulrgat_rv_slot: &mut f64,
        var_wdepnulrinvbot_slot: &mut f64,
        var_wdepnulrinvbot_rv_slot: &mut f64,
        var_wdepnulrinvgat_slot: &mut f64,
        var_wdepnulrinvgat_rv_slot: &mut f64,
        var_wdepnulrinvsti_slot: &mut f64,
        var_wdepnulrinvsti_rv_slot: &mut f64,
        var_wdepnulrsti_slot: &mut f64,
        var_wdepnulrsti_rv_slot: &mut f64,
    ) {
        let mut var_alphaav: f64 = *var_alphaav_slot;
        let mut var_alphaav_rv: f64 = *var_alphaav_rv_slot;
        let mut var_chnl_type: f64 = *var_chnl_type_slot;
        let mut var_chnl_type_rv: f64 = *var_chnl_type_rv_slot;
        let mut var_cjorbotd_i: f64 = *var_cjorbotd_i_slot;
        let mut var_cjorbotd_i_rv: f64 = *var_cjorbotd_i_rv_slot;
        let mut var_cjorgat2nd: f64 = *var_cjorgat2nd_slot;
        let mut var_cjorgat2nd_rv: f64 = *var_cjorgat2nd_rv_slot;
        let mut var_cjorgatd_i: f64 = *var_cjorgatd_i_slot;
        let mut var_cjorgatd_i_rv: f64 = *var_cjorgatd_i_rv_slot;
        let mut var_cjorstid_i: f64 = *var_cjorstid_i_slot;
        let mut var_cjorstid_i_rv: f64 = *var_cjorstid_i_rv_slot;
        let mut var_deltaphigr: f64 = *var_deltaphigr_slot;
        let mut var_deltaphigr_rv: f64 = *var_deltaphigr_rv_slot;
        let mut var_epssi: f64 = *var_epssi_slot;
        let mut var_epssi_rv: f64 = *var_epssi_rv_slot;
        let mut var_guard1: f64 = *var_guard1_slot;
        let mut var_guard10: f64 = *var_guard10_slot;
        let mut var_guard10_rv: f64 = *var_guard10_rv_slot;
        let mut var_guard1_rv: f64 = *var_guard1_rv_slot;
        let mut var_guard2: f64 = *var_guard2_slot;
        let mut var_guard2_rv: f64 = *var_guard2_rv_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard3_rv: f64 = *var_guard3_rv_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_guard4_rv: f64 = *var_guard4_rv_slot;
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_guard5_rv: f64 = *var_guard5_rv_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_guard6_rv: f64 = *var_guard6_rv_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard7_rv: f64 = *var_guard7_rv_slot;
        let mut var_guard8: f64 = *var_guard8_slot;
        let mut var_guard8_rv: f64 = *var_guard8_rv_slot;
        let mut var_guard9: f64 = *var_guard9_slot;
        let mut var_guard9_rv: f64 = *var_guard9_rv_slot;
        let mut var_kbol_over_qele: f64 = *var_kbol_over_qele_slot;
        let mut var_kbol_over_qele_rv: f64 = *var_kbol_over_qele_rv_slot;
        let mut var_one_minus_pbot: f64 = *var_one_minus_pbot_slot;
        let mut var_one_minus_pbot_rv: f64 = *var_one_minus_pbot_rv_slot;
        let mut var_one_minus_pgat: f64 = *var_one_minus_pgat_slot;
        let mut var_one_minus_pgat2nd: f64 = *var_one_minus_pgat2nd_slot;
        let mut var_one_minus_pgat2nd_rv: f64 = *var_one_minus_pgat2nd_rv_slot;
        let mut var_one_minus_pgat_rv: f64 = *var_one_minus_pgat_rv_slot;
        let mut var_one_minus_psti: f64 = *var_one_minus_psti_slot;
        let mut var_one_minus_psti_rv: f64 = *var_one_minus_psti_rv_slot;
        let mut var_one_over_one_minus_pbot: f64 = *var_one_over_one_minus_pbot_slot;
        let mut var_one_over_one_minus_pbot_rv: f64 = *var_one_over_one_minus_pbot_rv_slot;
        let mut var_one_over_one_minus_pgat: f64 = *var_one_over_one_minus_pgat_slot;
        let mut var_one_over_one_minus_pgat2nd: f64 = *var_one_over_one_minus_pgat2nd_slot;
        let mut var_one_over_one_minus_pgat2nd_rv: f64 = *var_one_over_one_minus_pgat2nd_rv_slot;
        let mut var_one_over_one_minus_pgat_rv: f64 = *var_one_over_one_minus_pgat_rv_slot;
        let mut var_one_over_one_minus_psti: f64 = *var_one_over_one_minus_psti_slot;
        let mut var_one_over_one_minus_psti_rv: f64 = *var_one_over_one_minus_psti_rv_slot;
        let mut var_pbotd_i: f64 = *var_pbotd_i_slot;
        let mut var_pbotd_i_rv: f64 = *var_pbotd_i_rv_slot;
        let mut var_pgat2nd: f64 = *var_pgat2nd_slot;
        let mut var_pgat2nd_rv: f64 = *var_pgat2nd_rv_slot;
        let mut var_pgatd_i: f64 = *var_pgatd_i_slot;
        let mut var_pgatd_i_rv: f64 = *var_pgatd_i_rv_slot;
        let mut var_phiggat2nd: f64 = *var_phiggat2nd_slot;
        let mut var_phiggat2nd_rv: f64 = *var_phiggat2nd_rv_slot;
        let mut var_phigrbot: f64 = *var_phigrbot_slot;
        let mut var_phigrbot_rv: f64 = *var_phigrbot_rv_slot;
        let mut var_phigrgat: f64 = *var_phigrgat_slot;
        let mut var_phigrgat2nd: f64 = *var_phigrgat2nd_slot;
        let mut var_phigrgat2nd_rv: f64 = *var_phigrgat2nd_rv_slot;
        let mut var_phigrgat_rv: f64 = *var_phigrgat_rv_slot;
        let mut var_phigrsti: f64 = *var_phigrsti_slot;
        let mut var_phigrsti_rv: f64 = *var_phigrsti_rv_slot;
        let mut var_phitr: f64 = *var_phitr_slot;
        let mut var_phitr_rv: f64 = *var_phitr_rv_slot;
        let mut var_phitrinv: f64 = *var_phitrinv_slot;
        let mut var_phitrinv_rv: f64 = *var_phitrinv_rv_slot;
        let mut var_pstid_i: f64 = *var_pstid_i_slot;
        let mut var_pstid_i_rv: f64 = *var_pstid_i_rv_slot;
        let mut var_swgat2nd: f64 = *var_swgat2nd_slot;
        let mut var_swgat2nd_rv: f64 = *var_swgat2nd_rv_slot;
        let mut var_swjunexp_i: f64 = *var_swjunexp_i_slot;
        let mut var_swjunexp_i_rv: f64 = *var_swjunexp_i_rv_slot;
        let mut var_swnqs_i: f64 = *var_swnqs_i_slot;
        let mut var_swnqs_i_rv: f64 = *var_swnqs_i_rv_slot;
        let mut var_tkr: f64 = *var_tkr_slot;
        let mut var_tkr_1: f64 = *var_tkr_1_slot;
        let mut var_tkr_1_rv: f64 = *var_tkr_1_rv_slot;
        let mut var_tkr_rv: f64 = *var_tkr_rv_slot;
        let mut var_vbirbotd_i: f64 = *var_vbirbotd_i_slot;
        let mut var_vbirbotd_i_rv: f64 = *var_vbirbotd_i_rv_slot;
        let mut var_vbirbotinv: f64 = *var_vbirbotinv_slot;
        let mut var_vbirbotinv_rv: f64 = *var_vbirbotinv_rv_slot;
        let mut var_vbirgat2nd: f64 = *var_vbirgat2nd_slot;
        let mut var_vbirgat2nd_rv: f64 = *var_vbirgat2nd_rv_slot;
        let mut var_vbirgatd_i: f64 = *var_vbirgatd_i_slot;
        let mut var_vbirgatd_i_rv: f64 = *var_vbirgatd_i_rv_slot;
        let mut var_vbirgatinv: f64 = *var_vbirgatinv_slot;
        let mut var_vbirgatinv_rv: f64 = *var_vbirgatinv_rv_slot;
        let mut var_vbirstid_i: f64 = *var_vbirstid_i_slot;
        let mut var_vbirstid_i_rv: f64 = *var_vbirstid_i_rv_slot;
        let mut var_vbirstiinv: f64 = *var_vbirstiinv_slot;
        let mut var_vbirstiinv_rv: f64 = *var_vbirstiinv_rv_slot;
        let mut var_vbrinvbot: f64 = *var_vbrinvbot_slot;
        let mut var_vbrinvbot_rv: f64 = *var_vbrinvbot_rv_slot;
        let mut var_vbrinvgat: f64 = *var_vbrinvgat_slot;
        let mut var_vbrinvgat_dn5: f64 = *var_vbrinvgat_dn5_slot;
        let mut var_vbrinvgat_dn6: f64 = *var_vbrinvgat_dn6_slot;
        let mut var_vbrinvgat_dn7: f64 = *var_vbrinvgat_dn7_slot;
        let mut var_vbrinvgat_dn8: f64 = *var_vbrinvgat_dn8_slot;
        let mut var_vbrinvgat_rv: f64 = *var_vbrinvgat_rv_slot;
        let mut var_vbrinvsti: f64 = *var_vbrinvsti_slot;
        let mut var_vbrinvsti_rv: f64 = *var_vbrinvsti_rv_slot;
        let mut var_vnorm: f64 = *var_vnorm_slot;
        let mut var_vnorm_inv: f64 = *var_vnorm_inv_slot;
        let mut var_vnorm_inv_rv: f64 = *var_vnorm_inv_rv_slot;
        let mut var_vnorm_rv: f64 = *var_vnorm_rv_slot;
        let mut var_wdepnulrbot: f64 = *var_wdepnulrbot_slot;
        let mut var_wdepnulrbot_rv: f64 = *var_wdepnulrbot_rv_slot;
        let mut var_wdepnulrgat: f64 = *var_wdepnulrgat_slot;
        let mut var_wdepnulrgat_rv: f64 = *var_wdepnulrgat_rv_slot;
        let mut var_wdepnulrinvbot: f64 = *var_wdepnulrinvbot_slot;
        let mut var_wdepnulrinvbot_rv: f64 = *var_wdepnulrinvbot_rv_slot;
        let mut var_wdepnulrinvgat: f64 = *var_wdepnulrinvgat_slot;
        let mut var_wdepnulrinvgat_rv: f64 = *var_wdepnulrinvgat_rv_slot;
        let mut var_wdepnulrinvsti: f64 = *var_wdepnulrinvsti_slot;
        let mut var_wdepnulrinvsti_rv: f64 = *var_wdepnulrinvsti_rv_slot;
        let mut var_wdepnulrsti: f64 = *var_wdepnulrsti_slot;
        let mut var_wdepnulrsti_rv: f64 = *var_wdepnulrsti_rv_slot;

        let assign00_e1569: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };
        var_guard1 = assign00_e1569;
        var_guard1_rv = 0.0;

        let (assign10_e1574,) = {
    if (var_guard1 != 0.0) {
        let assign10_e1572: f64 = 1.0;
        (assign10_e1572,)
    } else {
        (var_chnl_type,)
    }
};
        var_chnl_type = assign10_e1574;
        var_chnl_type_rv = 0.0;

        let (assign20_e1580,) = {
    if (var_guard1 == 0.0) {
        let assign20_e1578: f64 = (-1.0);
        (assign20_e1578,)
    } else {
        (var_chnl_type,)
    }
};
        var_chnl_type = assign20_e1580;
        var_chnl_type_rv = 0.0;

        let assign30_e1583: f64 = (8.8541878176e-12 * 11.8);
        var_epssi = assign30_e1583;
        var_epssi_rv = 0.0;

        let assign40_e1586: f64 = if p.p51 < 0.5 { 1.0 } else { 0.0 };
        var_guard2 = assign40_e1586;
        var_guard2_rv = 0.0;

        let (assign50_e1590,) = {
    if (var_guard2 != 0.0) {
        (0.0,)
    } else {
        (var_swnqs_i,)
    }
};
        var_swnqs_i = assign50_e1590;
        var_swnqs_i_rv = 0.0;

        let assign60_e1593: f64 = if p.p51 < 1.5 { 1.0 } else { 0.0 };
        var_guard3 = assign60_e1593;
        var_guard3_rv = 0.0;

        let (assign70_e1600,) = {
    if ((var_guard2 == 0.0) && (var_guard3 != 0.0)) {
        (1.0,)
    } else {
        (var_swnqs_i,)
    }
};
        var_swnqs_i = assign70_e1600;
        var_swnqs_i_rv = 0.0;

        let assign80_e1603: f64 = if p.p51 < 2.5 { 1.0 } else { 0.0 };
        var_guard4 = assign80_e1603;
        var_guard4_rv = 0.0;

        let (assign90_e1613,) = {
    if (((var_guard2 == 0.0) && (var_guard3 == 0.0)) && (var_guard4 != 0.0)) {
        (2.0,)
    } else {
        (var_swnqs_i,)
    }
};
        var_swnqs_i = assign90_e1613;
        var_swnqs_i_rv = 0.0;

        let assign100_e1616: f64 = if p.p51 < 4.0 { 1.0 } else { 0.0 };
        var_guard5 = assign100_e1616;
        var_guard5_rv = 0.0;

        let (assign110_e1629,) = {
    if ((((var_guard2 == 0.0) && (var_guard3 == 0.0)) && (var_guard4 == 0.0)) && (var_guard5 != 0.0)) {
        (3.0,)
    } else {
        (var_swnqs_i,)
    }
};
        var_swnqs_i = assign110_e1629;
        var_swnqs_i_rv = 0.0;

        let assign120_e1632: f64 = if p.p51 < 7.0 { 1.0 } else { 0.0 };
        var_guard6 = assign120_e1632;
        var_guard6_rv = 0.0;

        let (assign130_e1648,) = {
    if (((((var_guard2 == 0.0) && (var_guard3 == 0.0)) && (var_guard4 == 0.0)) && (var_guard5 == 0.0)) && (var_guard6 != 0.0)) {
        (5.0,)
    } else {
        (var_swnqs_i,)
    }
};
        var_swnqs_i = assign130_e1648;
        var_swnqs_i_rv = 0.0;

        let (assign140_e1665,) = {
    if (((((var_guard2 == 0.0) && (var_guard3 == 0.0)) && (var_guard4 == 0.0)) && (var_guard5 == 0.0)) && (var_guard6 == 0.0)) {
        (9.0,)
    } else {
        (var_swnqs_i,)
    }
};
        var_swnqs_i = assign140_e1665;
        var_swnqs_i_rv = 0.0;

        var_vnorm = 10.0;
        var_vnorm_rv = 0.0;

        let assign170_e1670: f64 = (1.0 / var_vnorm);
        var_vnorm_inv = assign170_e1670;
        var_vnorm_inv_rv = 0.0;

        let assign180_e1673: f64 = (273.15 + p.p38);
        var_tkr = assign180_e1673;
        var_tkr_rv = 0.0;

        var_swjunexp_i = 0.0;
        var_swjunexp_i_rv = 0.0;

        let assign200_e1677: f64 = if p.p927 > 0.5 { 1.0 } else { 0.0 };
        var_guard7 = assign200_e1677;
        var_guard7_rv = 0.0;

        let (assign210_e1681,) = {
    if (var_guard7 != 0.0) {
        (1.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign210_e1681;
        var_swjunexp_i_rv = 0.0;

        let (assign220_e1686,) = {
    if (var_guard7 == 0.0) {
        (0.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign220_e1686;
        var_swjunexp_i_rv = 0.0;

        let assign230_e1689: f64 = (273.15 + p.p823);
        var_tkr_1 = assign230_e1689;
        var_tkr_1_rv = 0.0;

        let assign240_e1692: f64 = (1.3806505e-23 / 1.6021918e-19);
        var_kbol_over_qele = assign240_e1692;
        var_kbol_over_qele_rv = 0.0;

        let assign250_e1695: f64 = (var_kbol_over_qele * var_tkr_1);
        var_phitr = assign250_e1695;
        var_phitr_rv = 0.0;

        let assign260_e1698: f64 = (1.0 / var_phitr);
        var_phitrinv = assign260_e1698;
        var_phitrinv_rv = 0.0;

        let assign270_e1701: f64 = (0.000702 * var_tkr_1);
        let assign270_e1703: f64 = (assign270_e1701 * var_tkr_1);
        let assign270_e1704: f64 = (-assign270_e1703);
        let assign270_e1707: f64 = (1108.0 + var_tkr_1);
        let assign270_e1708: f64 = (assign270_e1704 / assign270_e1707);
        var_deltaphigr = assign270_e1708;
        var_deltaphigr_rv = 0.0;

        let assign280_e1711: f64 = (p.p834 + var_deltaphigr);
        var_phigrbot = assign280_e1711;
        var_phigrbot_rv = 0.0;

        let assign290_e1714: f64 = (p.p835 + var_deltaphigr);
        var_phigrsti = assign290_e1714;
        var_phigrsti_rv = 0.0;

        let assign300_e1717: f64 = (p.p836 + var_deltaphigr);
        var_phigrgat = assign300_e1717;
        var_phigrgat_rv = 0.0;

        let assign310_e1720: f64 = (1.0 - p.p831);
        var_one_minus_pbot = assign310_e1720;
        var_one_minus_pbot_rv = 0.0;

        let assign320_e1723: f64 = (1.0 - p.p832);
        var_one_minus_psti = assign320_e1723;
        var_one_minus_psti_rv = 0.0;

        let assign330_e1726: f64 = (1.0 - p.p833);
        var_one_minus_pgat = assign330_e1726;
        var_one_minus_pgat_rv = 0.0;

        let assign340_e1729: f64 = (1.0 / var_one_minus_pbot);
        var_one_over_one_minus_pbot = assign340_e1729;
        var_one_over_one_minus_pbot_rv = 0.0;

        let assign350_e1732: f64 = (1.0 / var_one_minus_psti);
        var_one_over_one_minus_psti = assign350_e1732;
        var_one_over_one_minus_psti_rv = 0.0;

        let assign360_e1735: f64 = (1.0 / var_one_minus_pgat);
        var_one_over_one_minus_pgat = assign360_e1735;
        var_one_over_one_minus_pgat_rv = 0.0;

        let assign370_e1738: f64 = (var_epssi / p.p825);
        var_wdepnulrbot = assign370_e1738;
        var_wdepnulrbot_rv = 0.0;

        let assign380_e1741: f64 = (p.p843 * var_epssi);
        let assign380_e1743: f64 = (assign380_e1741 / p.p826);
        var_wdepnulrsti = assign380_e1743;
        var_wdepnulrsti_rv = 0.0;

        let assign390_e1746: f64 = (p.p844 * var_epssi);
        let assign390_e1748: f64 = (assign390_e1746 / p.p827);
        var_wdepnulrgat = assign390_e1748;
        var_wdepnulrgat_rv = 0.0;

        let assign400_e1751: f64 = (1.0 / var_wdepnulrbot);
        var_wdepnulrinvbot = assign400_e1751;
        var_wdepnulrinvbot_rv = 0.0;

        let assign410_e1754: f64 = (1.0 / var_wdepnulrsti);
        var_wdepnulrinvsti = assign410_e1754;
        var_wdepnulrinvsti_rv = 0.0;

        let assign420_e1757: f64 = (1.0 / var_wdepnulrgat);
        var_wdepnulrinvgat = assign420_e1757;
        var_wdepnulrinvgat_rv = 0.0;

        let assign430_e1760: f64 = (1.0 / p.p828);
        var_vbirbotinv = assign430_e1760;
        var_vbirbotinv_rv = 0.0;

        let assign440_e1763: f64 = (1.0 / p.p829);
        var_vbirstiinv = assign440_e1763;
        var_vbirstiinv_rv = 0.0;

        let assign450_e1766: f64 = (1.0 / p.p830);
        var_vbirgatinv = assign450_e1766;
        var_vbirgatinv_rv = 0.0;

        let assign490_e1791: f64 = (1.0 / p.p824);
        let assign490_e1792: f64 = (1.0 - assign490_e1791);
        var_alphaav = assign490_e1792;
        var_alphaav_rv = 0.0;

        let assign530_e1816: f64 = (1.0 / p.p860);
        var_vbrinvbot = assign530_e1816;
        var_vbrinvbot_rv = 0.0;

        let assign540_e1819: f64 = (1.0 / p.p861);
        var_vbrinvsti = assign540_e1819;
        var_vbrinvsti_rv = 0.0;

        let assign550_e1822: f64 = (1.0 / p.p862);
        var_vbrinvgat = assign550_e1822;
        var_vbrinvgat_dn5 = 0.0;
        var_vbrinvgat_dn6 = 0.0;
        var_vbrinvgat_dn7 = 0.0;
        var_vbrinvgat_dn8 = 0.0;
        var_vbrinvgat_rv = 0.0;

        let assign590_e1879: f64 = if ((((p.p866 != 1.0) || (p.p867 != 1.0)) || (p.p868 != 1.0)) || (p.p869 != 1.0)) { 1.0 } else { 0.0 };
        var_guard8 = assign590_e1879;
        var_guard8_rv = 0.0;

        let (assign600_e1883,) = {
    if (var_guard8 != 0.0) {
        (1.0,)
    } else {
        (var_swgat2nd,)
    }
};
        var_swgat2nd = assign600_e1883;
        var_swgat2nd_rv = 0.0;

        let (assign610_e1888,) = {
    if (var_guard8 == 0.0) {
        (0.0,)
    } else {
        (var_swgat2nd,)
    }
};
        var_swgat2nd = assign610_e1888;
        var_swgat2nd_rv = 0.0;

        let assign620_e1891: f64 = if var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        var_guard9 = assign620_e1891;
        var_guard9_rv = 0.0;

        let (assign630_e1904,) = {
    if (var_guard9 != 0.0) {
        let assign630_e1895: f64 = (p.p827 * p.p866);
        let (assign630_e1902,) = {
            if (assign630_e1895 > 1e-18) {
                let assign630_e1900: f64 = (p.p827 * p.p866);
                (assign630_e1900,)
            } else {
                (1e-18,)
            }
        };
        (assign630_e1902,)
    } else {
        (var_cjorgat2nd,)
    }
};
        var_cjorgat2nd = assign630_e1904;
        var_cjorgat2nd_rv = 0.0;

        let (assign640_e1917,) = {
    if (var_guard9 != 0.0) {
        let assign640_e1908: f64 = (p.p830 * p.p867);
        let (assign640_e1915,) = {
            if (assign640_e1908 > 0.05) {
                let assign640_e1913: f64 = (p.p830 * p.p867);
                (assign640_e1913,)
            } else {
                (0.05,)
            }
        };
        (assign640_e1915,)
    } else {
        (var_vbirgat2nd,)
    }
};
        var_vbirgat2nd = assign640_e1917;
        var_vbirgat2nd_rv = 0.0;

        let (assign650_e1944,) = {
    if (var_guard9 != 0.0) {
        let assign650_e1921: f64 = (p.p833 * p.p868);
        let (assign650_e1928,) = {
            if (assign650_e1921 > 0.05) {
                let assign650_e1926: f64 = (p.p833 * p.p868);
                (assign650_e1926,)
            } else {
                (0.05,)
            }
        };
        let (assign650_e1942,) = {
            if (assign650_e1928 < 0.95) {
                let assign650_e1933: f64 = (p.p833 * p.p868);
                let (assign650_e1940,) = {
                    if (assign650_e1933 > 0.05) {
                        let assign650_e1938: f64 = (p.p833 * p.p868);
                        (assign650_e1938,)
                    } else {
                        (0.05,)
                    }
                };
                (assign650_e1940,)
            } else {
                (0.95,)
            }
        };
        (assign650_e1942,)
    } else {
        (var_pgat2nd,)
    }
};
        var_pgat2nd = assign650_e1944;
        var_pgat2nd_rv = 0.0;

        let (assign660_e1950,) = {
    if (var_guard9 != 0.0) {
        let assign660_e1948: f64 = (p.p836 * p.p869);
        (assign660_e1948,)
    } else {
        (var_phiggat2nd,)
    }
};
        var_phiggat2nd = assign660_e1950;
        var_phiggat2nd_rv = 0.0;

        let (assign670_e1956,) = {
    if (var_guard9 != 0.0) {
        let assign670_e1954: f64 = (var_phiggat2nd + var_deltaphigr);
        (assign670_e1954,)
    } else {
        (var_phigrgat2nd,)
    }
};
        var_phigrgat2nd = assign670_e1956;
        var_phigrgat2nd_rv = 0.0;

        let (assign680_e1962,) = {
    if (var_guard9 != 0.0) {
        let assign680_e1960: f64 = (1.0 - var_pgat2nd);
        (assign680_e1960,)
    } else {
        (var_one_minus_pgat2nd,)
    }
};
        var_one_minus_pgat2nd = assign680_e1962;
        var_one_minus_pgat2nd_rv = 0.0;

        let (assign690_e1968,) = {
    if (var_guard9 != 0.0) {
        let assign690_e1966: f64 = (1.0 / var_one_minus_pgat2nd);
        (assign690_e1966,)
    } else {
        (var_one_over_one_minus_pgat2nd,)
    }
};
        var_one_over_one_minus_pgat2nd = assign690_e1968;
        var_one_over_one_minus_pgat2nd_rv = 0.0;

        let assign700_e1971: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard10 = assign700_e1971;
        var_guard10_rv = 0.0;

        let (assign710_e1975,) = {
    if (var_guard10 != 0.0) {
        (p.p825,)
    } else {
        (var_cjorbotd_i,)
    }
};
        var_cjorbotd_i = assign710_e1975;
        var_cjorbotd_i_rv = 0.0;

        let (assign720_e1979,) = {
    if (var_guard10 != 0.0) {
        (p.p826,)
    } else {
        (var_cjorstid_i,)
    }
};
        var_cjorstid_i = assign720_e1979;
        var_cjorstid_i_rv = 0.0;

        let (assign730_e1983,) = {
    if (var_guard10 != 0.0) {
        (p.p827,)
    } else {
        (var_cjorgatd_i,)
    }
};
        var_cjorgatd_i = assign730_e1983;
        var_cjorgatd_i_rv = 0.0;

        let (assign740_e1987,) = {
    if (var_guard10 != 0.0) {
        (p.p828,)
    } else {
        (var_vbirbotd_i,)
    }
};
        var_vbirbotd_i = assign740_e1987;
        var_vbirbotd_i_rv = 0.0;

        let (assign750_e1991,) = {
    if (var_guard10 != 0.0) {
        (p.p829,)
    } else {
        (var_vbirstid_i,)
    }
};
        var_vbirstid_i = assign750_e1991;
        var_vbirstid_i_rv = 0.0;

        let (assign760_e1995,) = {
    if (var_guard10 != 0.0) {
        (p.p830,)
    } else {
        (var_vbirgatd_i,)
    }
};
        var_vbirgatd_i = assign760_e1995;
        var_vbirgatd_i_rv = 0.0;

        let (assign770_e1999,) = {
    if (var_guard10 != 0.0) {
        (p.p831,)
    } else {
        (var_pbotd_i,)
    }
};
        var_pbotd_i = assign770_e1999;
        var_pbotd_i_rv = 0.0;

        let (assign780_e2003,) = {
    if (var_guard10 != 0.0) {
        (p.p832,)
    } else {
        (var_pstid_i,)
    }
};
        var_pstid_i = assign780_e2003;
        var_pstid_i_rv = 0.0;

        let (assign790_e2007,) = {
    if (var_guard10 != 0.0) {
        (p.p833,)
    } else {
        (var_pgatd_i,)
    }
};
        var_pgatd_i = assign790_e2007;
        var_pgatd_i_rv = 0.0;

        *var_alphaav_slot = var_alphaav;
        *var_alphaav_rv_slot = var_alphaav_rv;
        *var_chnl_type_slot = var_chnl_type;
        *var_chnl_type_rv_slot = var_chnl_type_rv;
        *var_cjorbotd_i_slot = var_cjorbotd_i;
        *var_cjorbotd_i_rv_slot = var_cjorbotd_i_rv;
        *var_cjorgat2nd_slot = var_cjorgat2nd;
        *var_cjorgat2nd_rv_slot = var_cjorgat2nd_rv;
        *var_cjorgatd_i_slot = var_cjorgatd_i;
        *var_cjorgatd_i_rv_slot = var_cjorgatd_i_rv;
        *var_cjorstid_i_slot = var_cjorstid_i;
        *var_cjorstid_i_rv_slot = var_cjorstid_i_rv;
        *var_deltaphigr_slot = var_deltaphigr;
        *var_deltaphigr_rv_slot = var_deltaphigr_rv;
        *var_epssi_slot = var_epssi;
        *var_epssi_rv_slot = var_epssi_rv;
        *var_guard1_slot = var_guard1;
        *var_guard10_slot = var_guard10;
        *var_guard10_rv_slot = var_guard10_rv;
        *var_guard1_rv_slot = var_guard1_rv;
        *var_guard2_slot = var_guard2;
        *var_guard2_rv_slot = var_guard2_rv;
        *var_guard3_slot = var_guard3;
        *var_guard3_rv_slot = var_guard3_rv;
        *var_guard4_slot = var_guard4;
        *var_guard4_rv_slot = var_guard4_rv;
        *var_guard5_slot = var_guard5;
        *var_guard5_rv_slot = var_guard5_rv;
        *var_guard6_slot = var_guard6;
        *var_guard6_rv_slot = var_guard6_rv;
        *var_guard7_slot = var_guard7;
        *var_guard7_rv_slot = var_guard7_rv;
        *var_guard8_slot = var_guard8;
        *var_guard8_rv_slot = var_guard8_rv;
        *var_guard9_slot = var_guard9;
        *var_guard9_rv_slot = var_guard9_rv;
        *var_kbol_over_qele_slot = var_kbol_over_qele;
        *var_kbol_over_qele_rv_slot = var_kbol_over_qele_rv;
        *var_one_minus_pbot_slot = var_one_minus_pbot;
        *var_one_minus_pbot_rv_slot = var_one_minus_pbot_rv;
        *var_one_minus_pgat_slot = var_one_minus_pgat;
        *var_one_minus_pgat2nd_slot = var_one_minus_pgat2nd;
        *var_one_minus_pgat2nd_rv_slot = var_one_minus_pgat2nd_rv;
        *var_one_minus_pgat_rv_slot = var_one_minus_pgat_rv;
        *var_one_minus_psti_slot = var_one_minus_psti;
        *var_one_minus_psti_rv_slot = var_one_minus_psti_rv;
        *var_one_over_one_minus_pbot_slot = var_one_over_one_minus_pbot;
        *var_one_over_one_minus_pbot_rv_slot = var_one_over_one_minus_pbot_rv;
        *var_one_over_one_minus_pgat_slot = var_one_over_one_minus_pgat;
        *var_one_over_one_minus_pgat2nd_slot = var_one_over_one_minus_pgat2nd;
        *var_one_over_one_minus_pgat2nd_rv_slot = var_one_over_one_minus_pgat2nd_rv;
        *var_one_over_one_minus_pgat_rv_slot = var_one_over_one_minus_pgat_rv;
        *var_one_over_one_minus_psti_slot = var_one_over_one_minus_psti;
        *var_one_over_one_minus_psti_rv_slot = var_one_over_one_minus_psti_rv;
        *var_pbotd_i_slot = var_pbotd_i;
        *var_pbotd_i_rv_slot = var_pbotd_i_rv;
        *var_pgat2nd_slot = var_pgat2nd;
        *var_pgat2nd_rv_slot = var_pgat2nd_rv;
        *var_pgatd_i_slot = var_pgatd_i;
        *var_pgatd_i_rv_slot = var_pgatd_i_rv;
        *var_phiggat2nd_slot = var_phiggat2nd;
        *var_phiggat2nd_rv_slot = var_phiggat2nd_rv;
        *var_phigrbot_slot = var_phigrbot;
        *var_phigrbot_rv_slot = var_phigrbot_rv;
        *var_phigrgat_slot = var_phigrgat;
        *var_phigrgat2nd_slot = var_phigrgat2nd;
        *var_phigrgat2nd_rv_slot = var_phigrgat2nd_rv;
        *var_phigrgat_rv_slot = var_phigrgat_rv;
        *var_phigrsti_slot = var_phigrsti;
        *var_phigrsti_rv_slot = var_phigrsti_rv;
        *var_phitr_slot = var_phitr;
        *var_phitr_rv_slot = var_phitr_rv;
        *var_phitrinv_slot = var_phitrinv;
        *var_phitrinv_rv_slot = var_phitrinv_rv;
        *var_pstid_i_slot = var_pstid_i;
        *var_pstid_i_rv_slot = var_pstid_i_rv;
        *var_swgat2nd_slot = var_swgat2nd;
        *var_swgat2nd_rv_slot = var_swgat2nd_rv;
        *var_swjunexp_i_slot = var_swjunexp_i;
        *var_swjunexp_i_rv_slot = var_swjunexp_i_rv;
        *var_swnqs_i_slot = var_swnqs_i;
        *var_swnqs_i_rv_slot = var_swnqs_i_rv;
        *var_tkr_slot = var_tkr;
        *var_tkr_1_slot = var_tkr_1;
        *var_tkr_1_rv_slot = var_tkr_1_rv;
        *var_tkr_rv_slot = var_tkr_rv;
        *var_vbirbotd_i_slot = var_vbirbotd_i;
        *var_vbirbotd_i_rv_slot = var_vbirbotd_i_rv;
        *var_vbirbotinv_slot = var_vbirbotinv;
        *var_vbirbotinv_rv_slot = var_vbirbotinv_rv;
        *var_vbirgat2nd_slot = var_vbirgat2nd;
        *var_vbirgat2nd_rv_slot = var_vbirgat2nd_rv;
        *var_vbirgatd_i_slot = var_vbirgatd_i;
        *var_vbirgatd_i_rv_slot = var_vbirgatd_i_rv;
        *var_vbirgatinv_slot = var_vbirgatinv;
        *var_vbirgatinv_rv_slot = var_vbirgatinv_rv;
        *var_vbirstid_i_slot = var_vbirstid_i;
        *var_vbirstid_i_rv_slot = var_vbirstid_i_rv;
        *var_vbirstiinv_slot = var_vbirstiinv;
        *var_vbirstiinv_rv_slot = var_vbirstiinv_rv;
        *var_vbrinvbot_slot = var_vbrinvbot;
        *var_vbrinvbot_rv_slot = var_vbrinvbot_rv;
        *var_vbrinvgat_slot = var_vbrinvgat;
        *var_vbrinvgat_dn5_slot = var_vbrinvgat_dn5;
        *var_vbrinvgat_dn6_slot = var_vbrinvgat_dn6;
        *var_vbrinvgat_dn7_slot = var_vbrinvgat_dn7;
        *var_vbrinvgat_dn8_slot = var_vbrinvgat_dn8;
        *var_vbrinvgat_rv_slot = var_vbrinvgat_rv;
        *var_vbrinvsti_slot = var_vbrinvsti;
        *var_vbrinvsti_rv_slot = var_vbrinvsti_rv;
        *var_vnorm_slot = var_vnorm;
        *var_vnorm_inv_slot = var_vnorm_inv;
        *var_vnorm_inv_rv_slot = var_vnorm_inv_rv;
        *var_vnorm_rv_slot = var_vnorm_rv;
        *var_wdepnulrbot_slot = var_wdepnulrbot;
        *var_wdepnulrbot_rv_slot = var_wdepnulrbot_rv;
        *var_wdepnulrgat_slot = var_wdepnulrgat;
        *var_wdepnulrgat_rv_slot = var_wdepnulrgat_rv;
        *var_wdepnulrinvbot_slot = var_wdepnulrinvbot;
        *var_wdepnulrinvbot_rv_slot = var_wdepnulrinvbot_rv;
        *var_wdepnulrinvgat_slot = var_wdepnulrinvgat;
        *var_wdepnulrinvgat_rv_slot = var_wdepnulrinvgat_rv;
        *var_wdepnulrinvsti_slot = var_wdepnulrinvsti;
        *var_wdepnulrinvsti_rv_slot = var_wdepnulrinvsti_rv;
        *var_wdepnulrsti_slot = var_wdepnulrsti;
        *var_wdepnulrsti_rv_slot = var_wdepnulrsti_rv;
    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        var_guard10: f64,
        var_adbbtgatd_i_slot: &mut f64,
        var_adbbtgatd_i_rv_slot: &mut f64,
        var_advbrgatd_i_slot: &mut f64,
        var_advbrgatd_i_rv_slot: &mut f64,
        var_anugatd_i_slot: &mut f64,
        var_anugatd_i_rv_slot: &mut f64,
        var_bdbbtgatd_i_slot: &mut f64,
        var_bdbbtgatd_i_rv_slot: &mut f64,
        var_bdvbrgatd_i_slot: &mut f64,
        var_bdvbrgatd_i_rv_slot: &mut f64,
        var_cbbtbotd_i_slot: &mut f64,
        var_cbbtbotd_i_rv_slot: &mut f64,
        var_cbbtgatd_i_slot: &mut f64,
        var_cbbtgatd_i_rv_slot: &mut f64,
        var_cbbtstid_i_slot: &mut f64,
        var_cbbtstid_i_rv_slot: &mut f64,
        var_cjorbotd_i_slot: &mut f64,
        var_cjorbotd_i_rv_slot: &mut f64,
        var_cjorgatd_i_slot: &mut f64,
        var_cjorgatd_i_rv_slot: &mut f64,
        var_cjorstid_i_slot: &mut f64,
        var_cjorstid_i_rv_slot: &mut f64,
        var_csrhbotd_i_slot: &mut f64,
        var_csrhbotd_i_rv_slot: &mut f64,
        var_csrhgatd_i_slot: &mut f64,
        var_csrhgatd_i_rv_slot: &mut f64,
        var_csrhstid_i_slot: &mut f64,
        var_csrhstid_i_rv_slot: &mut f64,
        var_ctatbotd_i_slot: &mut f64,
        var_ctatbotd_i_rv_slot: &mut f64,
        var_ctatgatd_i_slot: &mut f64,
        var_ctatgatd_i_rv_slot: &mut f64,
        var_ctatstid_i_slot: &mut f64,
        var_ctatstid_i_rv_slot: &mut f64,
        var_fbbtrbotd_i_slot: &mut f64,
        var_fbbtrbotd_i_rv_slot: &mut f64,
        var_fbbtrgatd_i_slot: &mut f64,
        var_fbbtrgatd_i_rv_slot: &mut f64,
        var_fbbtrstid_i_slot: &mut f64,
        var_fbbtrstid_i_rv_slot: &mut f64,
        var_fcjorgat2d_i_slot: &mut f64,
        var_fcjorgat2d_i_rv_slot: &mut f64,
        var_fjunqd_i_slot: &mut f64,
        var_fjunqd_i_rv_slot: &mut f64,
        var_fpgat2d_i_slot: &mut f64,
        var_fpgat2d_i_rv_slot: &mut f64,
        var_fphiggat2d_i_slot: &mut f64,
        var_fphiggat2d_i_rv_slot: &mut f64,
        var_fvbirgat2d_i_slot: &mut f64,
        var_fvbirgat2d_i_rv_slot: &mut f64,
        var_idsatrbotd_i_slot: &mut f64,
        var_idsatrbotd_i_rv_slot: &mut f64,
        var_idsatrgatd_i_slot: &mut f64,
        var_idsatrgatd_i_rv_slot: &mut f64,
        var_idsatrstid_i_slot: &mut f64,
        var_idsatrstid_i_rv_slot: &mut f64,
        var_mefftatbotd_i_slot: &mut f64,
        var_mefftatbotd_i_rv_slot: &mut f64,
        var_mefftatgatd_i_slot: &mut f64,
        var_mefftatgatd_i_rv_slot: &mut f64,
        var_mefftatstid_i_slot: &mut f64,
        var_mefftatstid_i_rv_slot: &mut f64,
        var_pbotd_i_slot: &mut f64,
        var_pbotd_i_rv_slot: &mut f64,
        var_pbrbotd_i_slot: &mut f64,
        var_pbrbotd_i_rv_slot: &mut f64,
        var_pbrgatd_i_slot: &mut f64,
        var_pbrgatd_i_rv_slot: &mut f64,
        var_pbrstid_i_slot: &mut f64,
        var_pbrstid_i_rv_slot: &mut f64,
        var_phigbotd_i_slot: &mut f64,
        var_phigbotd_i_rv_slot: &mut f64,
        var_phiggatd_i_slot: &mut f64,
        var_phiggatd_i_rv_slot: &mut f64,
        var_phigstid_i_slot: &mut f64,
        var_phigstid_i_rv_slot: &mut f64,
        var_pstid_i_slot: &mut f64,
        var_pstid_i_rv_slot: &mut f64,
        var_stfbbtbotd_i_slot: &mut f64,
        var_stfbbtbotd_i_rv_slot: &mut f64,
        var_stfbbtgatd_i_slot: &mut f64,
        var_stfbbtgatd_i_rv_slot: &mut f64,
        var_stfbbtstid_i_slot: &mut f64,
        var_stfbbtstid_i_rv_slot: &mut f64,
        var_vbirbotd_i_slot: &mut f64,
        var_vbirbotd_i_rv_slot: &mut f64,
        var_vbirgatd_i_slot: &mut f64,
        var_vbirgatd_i_rv_slot: &mut f64,
        var_vbirstid_i_slot: &mut f64,
        var_vbirstid_i_rv_slot: &mut f64,
        var_vbrbotd_i_slot: &mut f64,
        var_vbrbotd_i_rv_slot: &mut f64,
        var_vbrgatd_i_slot: &mut f64,
        var_vbrgatd_i_rv_slot: &mut f64,
        var_vbrstid_i_slot: &mut f64,
        var_vbrstid_i_rv_slot: &mut f64,
        var_vtrgatd_i_slot: &mut f64,
        var_vtrgatd_i_rv_slot: &mut f64,
        var_xjungatd_i_slot: &mut f64,
        var_xjungatd_i_rv_slot: &mut f64,
        var_xjunstid_i_slot: &mut f64,
        var_xjunstid_i_rv_slot: &mut f64,
    ) {
        let mut var_adbbtgatd_i: f64 = *var_adbbtgatd_i_slot;
        let mut var_adbbtgatd_i_rv: f64 = *var_adbbtgatd_i_rv_slot;
        let mut var_advbrgatd_i: f64 = *var_advbrgatd_i_slot;
        let mut var_advbrgatd_i_rv: f64 = *var_advbrgatd_i_rv_slot;
        let mut var_anugatd_i: f64 = *var_anugatd_i_slot;
        let mut var_anugatd_i_rv: f64 = *var_anugatd_i_rv_slot;
        let mut var_bdbbtgatd_i: f64 = *var_bdbbtgatd_i_slot;
        let mut var_bdbbtgatd_i_rv: f64 = *var_bdbbtgatd_i_rv_slot;
        let mut var_bdvbrgatd_i: f64 = *var_bdvbrgatd_i_slot;
        let mut var_bdvbrgatd_i_rv: f64 = *var_bdvbrgatd_i_rv_slot;
        let mut var_cbbtbotd_i: f64 = *var_cbbtbotd_i_slot;
        let mut var_cbbtbotd_i_rv: f64 = *var_cbbtbotd_i_rv_slot;
        let mut var_cbbtgatd_i: f64 = *var_cbbtgatd_i_slot;
        let mut var_cbbtgatd_i_rv: f64 = *var_cbbtgatd_i_rv_slot;
        let mut var_cbbtstid_i: f64 = *var_cbbtstid_i_slot;
        let mut var_cbbtstid_i_rv: f64 = *var_cbbtstid_i_rv_slot;
        let mut var_cjorbotd_i: f64 = *var_cjorbotd_i_slot;
        let mut var_cjorbotd_i_rv: f64 = *var_cjorbotd_i_rv_slot;
        let mut var_cjorgatd_i: f64 = *var_cjorgatd_i_slot;
        let mut var_cjorgatd_i_rv: f64 = *var_cjorgatd_i_rv_slot;
        let mut var_cjorstid_i: f64 = *var_cjorstid_i_slot;
        let mut var_cjorstid_i_rv: f64 = *var_cjorstid_i_rv_slot;
        let mut var_csrhbotd_i: f64 = *var_csrhbotd_i_slot;
        let mut var_csrhbotd_i_rv: f64 = *var_csrhbotd_i_rv_slot;
        let mut var_csrhgatd_i: f64 = *var_csrhgatd_i_slot;
        let mut var_csrhgatd_i_rv: f64 = *var_csrhgatd_i_rv_slot;
        let mut var_csrhstid_i: f64 = *var_csrhstid_i_slot;
        let mut var_csrhstid_i_rv: f64 = *var_csrhstid_i_rv_slot;
        let mut var_ctatbotd_i: f64 = *var_ctatbotd_i_slot;
        let mut var_ctatbotd_i_rv: f64 = *var_ctatbotd_i_rv_slot;
        let mut var_ctatgatd_i: f64 = *var_ctatgatd_i_slot;
        let mut var_ctatgatd_i_rv: f64 = *var_ctatgatd_i_rv_slot;
        let mut var_ctatstid_i: f64 = *var_ctatstid_i_slot;
        let mut var_ctatstid_i_rv: f64 = *var_ctatstid_i_rv_slot;
        let mut var_fbbtrbotd_i: f64 = *var_fbbtrbotd_i_slot;
        let mut var_fbbtrbotd_i_rv: f64 = *var_fbbtrbotd_i_rv_slot;
        let mut var_fbbtrgatd_i: f64 = *var_fbbtrgatd_i_slot;
        let mut var_fbbtrgatd_i_rv: f64 = *var_fbbtrgatd_i_rv_slot;
        let mut var_fbbtrstid_i: f64 = *var_fbbtrstid_i_slot;
        let mut var_fbbtrstid_i_rv: f64 = *var_fbbtrstid_i_rv_slot;
        let mut var_fcjorgat2d_i: f64 = *var_fcjorgat2d_i_slot;
        let mut var_fcjorgat2d_i_rv: f64 = *var_fcjorgat2d_i_rv_slot;
        let mut var_fjunqd_i: f64 = *var_fjunqd_i_slot;
        let mut var_fjunqd_i_rv: f64 = *var_fjunqd_i_rv_slot;
        let mut var_fpgat2d_i: f64 = *var_fpgat2d_i_slot;
        let mut var_fpgat2d_i_rv: f64 = *var_fpgat2d_i_rv_slot;
        let mut var_fphiggat2d_i: f64 = *var_fphiggat2d_i_slot;
        let mut var_fphiggat2d_i_rv: f64 = *var_fphiggat2d_i_rv_slot;
        let mut var_fvbirgat2d_i: f64 = *var_fvbirgat2d_i_slot;
        let mut var_fvbirgat2d_i_rv: f64 = *var_fvbirgat2d_i_rv_slot;
        let mut var_idsatrbotd_i: f64 = *var_idsatrbotd_i_slot;
        let mut var_idsatrbotd_i_rv: f64 = *var_idsatrbotd_i_rv_slot;
        let mut var_idsatrgatd_i: f64 = *var_idsatrgatd_i_slot;
        let mut var_idsatrgatd_i_rv: f64 = *var_idsatrgatd_i_rv_slot;
        let mut var_idsatrstid_i: f64 = *var_idsatrstid_i_slot;
        let mut var_idsatrstid_i_rv: f64 = *var_idsatrstid_i_rv_slot;
        let mut var_mefftatbotd_i: f64 = *var_mefftatbotd_i_slot;
        let mut var_mefftatbotd_i_rv: f64 = *var_mefftatbotd_i_rv_slot;
        let mut var_mefftatgatd_i: f64 = *var_mefftatgatd_i_slot;
        let mut var_mefftatgatd_i_rv: f64 = *var_mefftatgatd_i_rv_slot;
        let mut var_mefftatstid_i: f64 = *var_mefftatstid_i_slot;
        let mut var_mefftatstid_i_rv: f64 = *var_mefftatstid_i_rv_slot;
        let mut var_pbotd_i: f64 = *var_pbotd_i_slot;
        let mut var_pbotd_i_rv: f64 = *var_pbotd_i_rv_slot;
        let mut var_pbrbotd_i: f64 = *var_pbrbotd_i_slot;
        let mut var_pbrbotd_i_rv: f64 = *var_pbrbotd_i_rv_slot;
        let mut var_pbrgatd_i: f64 = *var_pbrgatd_i_slot;
        let mut var_pbrgatd_i_rv: f64 = *var_pbrgatd_i_rv_slot;
        let mut var_pbrstid_i: f64 = *var_pbrstid_i_slot;
        let mut var_pbrstid_i_rv: f64 = *var_pbrstid_i_rv_slot;
        let mut var_phigbotd_i: f64 = *var_phigbotd_i_slot;
        let mut var_phigbotd_i_rv: f64 = *var_phigbotd_i_rv_slot;
        let mut var_phiggatd_i: f64 = *var_phiggatd_i_slot;
        let mut var_phiggatd_i_rv: f64 = *var_phiggatd_i_rv_slot;
        let mut var_phigstid_i: f64 = *var_phigstid_i_slot;
        let mut var_phigstid_i_rv: f64 = *var_phigstid_i_rv_slot;
        let mut var_pstid_i: f64 = *var_pstid_i_slot;
        let mut var_pstid_i_rv: f64 = *var_pstid_i_rv_slot;
        let mut var_stfbbtbotd_i: f64 = *var_stfbbtbotd_i_slot;
        let mut var_stfbbtbotd_i_rv: f64 = *var_stfbbtbotd_i_rv_slot;
        let mut var_stfbbtgatd_i: f64 = *var_stfbbtgatd_i_slot;
        let mut var_stfbbtgatd_i_rv: f64 = *var_stfbbtgatd_i_rv_slot;
        let mut var_stfbbtstid_i: f64 = *var_stfbbtstid_i_slot;
        let mut var_stfbbtstid_i_rv: f64 = *var_stfbbtstid_i_rv_slot;
        let mut var_vbirbotd_i: f64 = *var_vbirbotd_i_slot;
        let mut var_vbirbotd_i_rv: f64 = *var_vbirbotd_i_rv_slot;
        let mut var_vbirgatd_i: f64 = *var_vbirgatd_i_slot;
        let mut var_vbirgatd_i_rv: f64 = *var_vbirgatd_i_rv_slot;
        let mut var_vbirstid_i: f64 = *var_vbirstid_i_slot;
        let mut var_vbirstid_i_rv: f64 = *var_vbirstid_i_rv_slot;
        let mut var_vbrbotd_i: f64 = *var_vbrbotd_i_slot;
        let mut var_vbrbotd_i_rv: f64 = *var_vbrbotd_i_rv_slot;
        let mut var_vbrgatd_i: f64 = *var_vbrgatd_i_slot;
        let mut var_vbrgatd_i_rv: f64 = *var_vbrgatd_i_rv_slot;
        let mut var_vbrstid_i: f64 = *var_vbrstid_i_slot;
        let mut var_vbrstid_i_rv: f64 = *var_vbrstid_i_rv_slot;
        let mut var_vtrgatd_i: f64 = *var_vtrgatd_i_slot;
        let mut var_vtrgatd_i_rv: f64 = *var_vtrgatd_i_rv_slot;
        let mut var_xjungatd_i: f64 = *var_xjungatd_i_slot;
        let mut var_xjungatd_i_rv: f64 = *var_xjungatd_i_rv_slot;
        let mut var_xjunstid_i: f64 = *var_xjunstid_i_slot;
        let mut var_xjunstid_i_rv: f64 = *var_xjunstid_i_rv_slot;

        let (assign800_e2011,) = {
    if (var_guard10 != 0.0) {
        (p.p834,)
    } else {
        (var_phigbotd_i,)
    }
};
        var_phigbotd_i = assign800_e2011;
        var_phigbotd_i_rv = 0.0;

        let (assign810_e2015,) = {
    if (var_guard10 != 0.0) {
        (p.p835,)
    } else {
        (var_phigstid_i,)
    }
};
        var_phigstid_i = assign810_e2015;
        var_phigstid_i_rv = 0.0;

        let (assign820_e2019,) = {
    if (var_guard10 != 0.0) {
        (p.p836,)
    } else {
        (var_phiggatd_i,)
    }
};
        var_phiggatd_i = assign820_e2019;
        var_phiggatd_i_rv = 0.0;

        let (assign830_e2023,) = {
    if (var_guard10 != 0.0) {
        (p.p837,)
    } else {
        (var_idsatrbotd_i,)
    }
};
        var_idsatrbotd_i = assign830_e2023;
        var_idsatrbotd_i_rv = 0.0;

        let (assign840_e2027,) = {
    if (var_guard10 != 0.0) {
        (p.p838,)
    } else {
        (var_idsatrstid_i,)
    }
};
        var_idsatrstid_i = assign840_e2027;
        var_idsatrstid_i_rv = 0.0;

        let (assign850_e2031,) = {
    if (var_guard10 != 0.0) {
        (p.p839,)
    } else {
        (var_idsatrgatd_i,)
    }
};
        var_idsatrgatd_i = assign850_e2031;
        var_idsatrgatd_i_rv = 0.0;

        let (assign860_e2035,) = {
    if (var_guard10 != 0.0) {
        (p.p840,)
    } else {
        (var_csrhbotd_i,)
    }
};
        var_csrhbotd_i = assign860_e2035;
        var_csrhbotd_i_rv = 0.0;

        let (assign870_e2039,) = {
    if (var_guard10 != 0.0) {
        (p.p841,)
    } else {
        (var_csrhstid_i,)
    }
};
        var_csrhstid_i = assign870_e2039;
        var_csrhstid_i_rv = 0.0;

        let (assign880_e2043,) = {
    if (var_guard10 != 0.0) {
        (p.p842,)
    } else {
        (var_csrhgatd_i,)
    }
};
        var_csrhgatd_i = assign880_e2043;
        var_csrhgatd_i_rv = 0.0;

        let (assign890_e2047,) = {
    if (var_guard10 != 0.0) {
        (p.p843,)
    } else {
        (var_xjunstid_i,)
    }
};
        var_xjunstid_i = assign890_e2047;
        var_xjunstid_i_rv = 0.0;

        let (assign900_e2051,) = {
    if (var_guard10 != 0.0) {
        (p.p844,)
    } else {
        (var_xjungatd_i,)
    }
};
        var_xjungatd_i = assign900_e2051;
        var_xjungatd_i_rv = 0.0;

        let (assign910_e2055,) = {
    if (var_guard10 != 0.0) {
        (p.p845,)
    } else {
        (var_ctatbotd_i,)
    }
};
        var_ctatbotd_i = assign910_e2055;
        var_ctatbotd_i_rv = 0.0;

        let (assign920_e2059,) = {
    if (var_guard10 != 0.0) {
        (p.p846,)
    } else {
        (var_ctatstid_i,)
    }
};
        var_ctatstid_i = assign920_e2059;
        var_ctatstid_i_rv = 0.0;

        let (assign930_e2063,) = {
    if (var_guard10 != 0.0) {
        (p.p847,)
    } else {
        (var_ctatgatd_i,)
    }
};
        var_ctatgatd_i = assign930_e2063;
        var_ctatgatd_i_rv = 0.0;

        let (assign940_e2067,) = {
    if (var_guard10 != 0.0) {
        (p.p848,)
    } else {
        (var_mefftatbotd_i,)
    }
};
        var_mefftatbotd_i = assign940_e2067;
        var_mefftatbotd_i_rv = 0.0;

        let (assign950_e2071,) = {
    if (var_guard10 != 0.0) {
        (p.p849,)
    } else {
        (var_mefftatstid_i,)
    }
};
        var_mefftatstid_i = assign950_e2071;
        var_mefftatstid_i_rv = 0.0;

        let (assign960_e2075,) = {
    if (var_guard10 != 0.0) {
        (p.p850,)
    } else {
        (var_mefftatgatd_i,)
    }
};
        var_mefftatgatd_i = assign960_e2075;
        var_mefftatgatd_i_rv = 0.0;

        let (assign970_e2079,) = {
    if (var_guard10 != 0.0) {
        (p.p851,)
    } else {
        (var_cbbtbotd_i,)
    }
};
        var_cbbtbotd_i = assign970_e2079;
        var_cbbtbotd_i_rv = 0.0;

        let (assign980_e2083,) = {
    if (var_guard10 != 0.0) {
        (p.p852,)
    } else {
        (var_cbbtstid_i,)
    }
};
        var_cbbtstid_i = assign980_e2083;
        var_cbbtstid_i_rv = 0.0;

        let (assign990_e2087,) = {
    if (var_guard10 != 0.0) {
        (p.p853,)
    } else {
        (var_cbbtgatd_i,)
    }
};
        var_cbbtgatd_i = assign990_e2087;
        var_cbbtgatd_i_rv = 0.0;

        let (assign1000_e2091,) = {
    if (var_guard10 != 0.0) {
        (p.p854,)
    } else {
        (var_fbbtrbotd_i,)
    }
};
        var_fbbtrbotd_i = assign1000_e2091;
        var_fbbtrbotd_i_rv = 0.0;

        let (assign1010_e2095,) = {
    if (var_guard10 != 0.0) {
        (p.p855,)
    } else {
        (var_fbbtrstid_i,)
    }
};
        var_fbbtrstid_i = assign1010_e2095;
        var_fbbtrstid_i_rv = 0.0;

        let (assign1020_e2099,) = {
    if (var_guard10 != 0.0) {
        (p.p856,)
    } else {
        (var_fbbtrgatd_i,)
    }
};
        var_fbbtrgatd_i = assign1020_e2099;
        var_fbbtrgatd_i_rv = 0.0;

        let (assign1030_e2103,) = {
    if (var_guard10 != 0.0) {
        (p.p857,)
    } else {
        (var_stfbbtbotd_i,)
    }
};
        var_stfbbtbotd_i = assign1030_e2103;
        var_stfbbtbotd_i_rv = 0.0;

        let (assign1040_e2107,) = {
    if (var_guard10 != 0.0) {
        (p.p858,)
    } else {
        (var_stfbbtstid_i,)
    }
};
        var_stfbbtstid_i = assign1040_e2107;
        var_stfbbtstid_i_rv = 0.0;

        let (assign1050_e2111,) = {
    if (var_guard10 != 0.0) {
        (p.p859,)
    } else {
        (var_stfbbtgatd_i,)
    }
};
        var_stfbbtgatd_i = assign1050_e2111;
        var_stfbbtgatd_i_rv = 0.0;

        let (assign1060_e2115,) = {
    if (var_guard10 != 0.0) {
        (p.p860,)
    } else {
        (var_vbrbotd_i,)
    }
};
        var_vbrbotd_i = assign1060_e2115;
        var_vbrbotd_i_rv = 0.0;

        let (assign1070_e2119,) = {
    if (var_guard10 != 0.0) {
        (p.p861,)
    } else {
        (var_vbrstid_i,)
    }
};
        var_vbrstid_i = assign1070_e2119;
        var_vbrstid_i_rv = 0.0;

        let (assign1080_e2123,) = {
    if (var_guard10 != 0.0) {
        (p.p862,)
    } else {
        (var_vbrgatd_i,)
    }
};
        var_vbrgatd_i = assign1080_e2123;
        var_vbrgatd_i_rv = 0.0;

        let (assign1090_e2127,) = {
    if (var_guard10 != 0.0) {
        (p.p863,)
    } else {
        (var_pbrbotd_i,)
    }
};
        var_pbrbotd_i = assign1090_e2127;
        var_pbrbotd_i_rv = 0.0;

        let (assign1100_e2131,) = {
    if (var_guard10 != 0.0) {
        (p.p864,)
    } else {
        (var_pbrstid_i,)
    }
};
        var_pbrstid_i = assign1100_e2131;
        var_pbrstid_i_rv = 0.0;

        let (assign1110_e2135,) = {
    if (var_guard10 != 0.0) {
        (p.p865,)
    } else {
        (var_pbrgatd_i,)
    }
};
        var_pbrgatd_i = assign1110_e2135;
        var_pbrgatd_i_rv = 0.0;

        let (assign1130_e2143,) = {
    if (var_guard10 != 0.0) {
        (p.p929,)
    } else {
        (var_fjunqd_i,)
    }
};
        var_fjunqd_i = assign1130_e2143;
        var_fjunqd_i_rv = 0.0;

        let (assign1140_e2147,) = {
    if (var_guard10 != 0.0) {
        (p.p872,)
    } else {
        (var_advbrgatd_i,)
    }
};
        var_advbrgatd_i = assign1140_e2147;
        var_advbrgatd_i_rv = 0.0;

        let (assign1150_e2151,) = {
    if (var_guard10 != 0.0) {
        (p.p873,)
    } else {
        (var_bdvbrgatd_i,)
    }
};
        var_bdvbrgatd_i = assign1150_e2151;
        var_bdvbrgatd_i_rv = 0.0;

        let (assign1160_e2155,) = {
    if (var_guard10 != 0.0) {
        (p.p874,)
    } else {
        (var_adbbtgatd_i,)
    }
};
        var_adbbtgatd_i = assign1160_e2155;
        var_adbbtgatd_i_rv = 0.0;

        let (assign1170_e2159,) = {
    if (var_guard10 != 0.0) {
        (p.p875,)
    } else {
        (var_bdbbtgatd_i,)
    }
};
        var_bdbbtgatd_i = assign1170_e2159;
        var_bdbbtgatd_i_rv = 0.0;

        let (assign1180_e2163,) = {
    if (var_guard10 != 0.0) {
        (p.p866,)
    } else {
        (var_fcjorgat2d_i,)
    }
};
        var_fcjorgat2d_i = assign1180_e2163;
        var_fcjorgat2d_i_rv = 0.0;

        let (assign1190_e2167,) = {
    if (var_guard10 != 0.0) {
        (p.p867,)
    } else {
        (var_fvbirgat2d_i,)
    }
};
        var_fvbirgat2d_i = assign1190_e2167;
        var_fvbirgat2d_i_rv = 0.0;

        let (assign1200_e2171,) = {
    if (var_guard10 != 0.0) {
        (p.p868,)
    } else {
        (var_fpgat2d_i,)
    }
};
        var_fpgat2d_i = assign1200_e2171;
        var_fpgat2d_i_rv = 0.0;

        let (assign1210_e2175,) = {
    if (var_guard10 != 0.0) {
        (p.p869,)
    } else {
        (var_fphiggat2d_i,)
    }
};
        var_fphiggat2d_i = assign1210_e2175;
        var_fphiggat2d_i_rv = 0.0;

        let (assign1220_e2179,) = {
    if (var_guard10 != 0.0) {
        (p.p870,)
    } else {
        (var_vtrgatd_i,)
    }
};
        var_vtrgatd_i = assign1220_e2179;
        var_vtrgatd_i_rv = 0.0;

        let (assign1230_e2183,) = {
    if (var_guard10 != 0.0) {
        (p.p871,)
    } else {
        (var_anugatd_i,)
    }
};
        var_anugatd_i = assign1230_e2183;
        var_anugatd_i_rv = 0.0;

        let (assign1240_e2188,) = {
    if (var_guard10 == 0.0) {
        (p.p876,)
    } else {
        (var_cjorbotd_i,)
    }
};
        var_cjorbotd_i = assign1240_e2188;
        var_cjorbotd_i_rv = 0.0;

        let (assign1250_e2193,) = {
    if (var_guard10 == 0.0) {
        (p.p877,)
    } else {
        (var_cjorstid_i,)
    }
};
        var_cjorstid_i = assign1250_e2193;
        var_cjorstid_i_rv = 0.0;

        let (assign1260_e2198,) = {
    if (var_guard10 == 0.0) {
        (p.p878,)
    } else {
        (var_cjorgatd_i,)
    }
};
        var_cjorgatd_i = assign1260_e2198;
        var_cjorgatd_i_rv = 0.0;

        let (assign1270_e2203,) = {
    if (var_guard10 == 0.0) {
        (p.p879,)
    } else {
        (var_vbirbotd_i,)
    }
};
        var_vbirbotd_i = assign1270_e2203;
        var_vbirbotd_i_rv = 0.0;

        let (assign1280_e2208,) = {
    if (var_guard10 == 0.0) {
        (p.p880,)
    } else {
        (var_vbirstid_i,)
    }
};
        var_vbirstid_i = assign1280_e2208;
        var_vbirstid_i_rv = 0.0;

        let (assign1290_e2213,) = {
    if (var_guard10 == 0.0) {
        (p.p881,)
    } else {
        (var_vbirgatd_i,)
    }
};
        var_vbirgatd_i = assign1290_e2213;
        var_vbirgatd_i_rv = 0.0;

        let (assign1300_e2218,) = {
    if (var_guard10 == 0.0) {
        (p.p882,)
    } else {
        (var_pbotd_i,)
    }
};
        var_pbotd_i = assign1300_e2218;
        var_pbotd_i_rv = 0.0;

        let (assign1310_e2223,) = {
    if (var_guard10 == 0.0) {
        (p.p883,)
    } else {
        (var_pstid_i,)
    }
};
        var_pstid_i = assign1310_e2223;
        var_pstid_i_rv = 0.0;

        *var_adbbtgatd_i_slot = var_adbbtgatd_i;
        *var_adbbtgatd_i_rv_slot = var_adbbtgatd_i_rv;
        *var_advbrgatd_i_slot = var_advbrgatd_i;
        *var_advbrgatd_i_rv_slot = var_advbrgatd_i_rv;
        *var_anugatd_i_slot = var_anugatd_i;
        *var_anugatd_i_rv_slot = var_anugatd_i_rv;
        *var_bdbbtgatd_i_slot = var_bdbbtgatd_i;
        *var_bdbbtgatd_i_rv_slot = var_bdbbtgatd_i_rv;
        *var_bdvbrgatd_i_slot = var_bdvbrgatd_i;
        *var_bdvbrgatd_i_rv_slot = var_bdvbrgatd_i_rv;
        *var_cbbtbotd_i_slot = var_cbbtbotd_i;
        *var_cbbtbotd_i_rv_slot = var_cbbtbotd_i_rv;
        *var_cbbtgatd_i_slot = var_cbbtgatd_i;
        *var_cbbtgatd_i_rv_slot = var_cbbtgatd_i_rv;
        *var_cbbtstid_i_slot = var_cbbtstid_i;
        *var_cbbtstid_i_rv_slot = var_cbbtstid_i_rv;
        *var_cjorbotd_i_slot = var_cjorbotd_i;
        *var_cjorbotd_i_rv_slot = var_cjorbotd_i_rv;
        *var_cjorgatd_i_slot = var_cjorgatd_i;
        *var_cjorgatd_i_rv_slot = var_cjorgatd_i_rv;
        *var_cjorstid_i_slot = var_cjorstid_i;
        *var_cjorstid_i_rv_slot = var_cjorstid_i_rv;
        *var_csrhbotd_i_slot = var_csrhbotd_i;
        *var_csrhbotd_i_rv_slot = var_csrhbotd_i_rv;
        *var_csrhgatd_i_slot = var_csrhgatd_i;
        *var_csrhgatd_i_rv_slot = var_csrhgatd_i_rv;
        *var_csrhstid_i_slot = var_csrhstid_i;
        *var_csrhstid_i_rv_slot = var_csrhstid_i_rv;
        *var_ctatbotd_i_slot = var_ctatbotd_i;
        *var_ctatbotd_i_rv_slot = var_ctatbotd_i_rv;
        *var_ctatgatd_i_slot = var_ctatgatd_i;
        *var_ctatgatd_i_rv_slot = var_ctatgatd_i_rv;
        *var_ctatstid_i_slot = var_ctatstid_i;
        *var_ctatstid_i_rv_slot = var_ctatstid_i_rv;
        *var_fbbtrbotd_i_slot = var_fbbtrbotd_i;
        *var_fbbtrbotd_i_rv_slot = var_fbbtrbotd_i_rv;
        *var_fbbtrgatd_i_slot = var_fbbtrgatd_i;
        *var_fbbtrgatd_i_rv_slot = var_fbbtrgatd_i_rv;
        *var_fbbtrstid_i_slot = var_fbbtrstid_i;
        *var_fbbtrstid_i_rv_slot = var_fbbtrstid_i_rv;
        *var_fcjorgat2d_i_slot = var_fcjorgat2d_i;
        *var_fcjorgat2d_i_rv_slot = var_fcjorgat2d_i_rv;
        *var_fjunqd_i_slot = var_fjunqd_i;
        *var_fjunqd_i_rv_slot = var_fjunqd_i_rv;
        *var_fpgat2d_i_slot = var_fpgat2d_i;
        *var_fpgat2d_i_rv_slot = var_fpgat2d_i_rv;
        *var_fphiggat2d_i_slot = var_fphiggat2d_i;
        *var_fphiggat2d_i_rv_slot = var_fphiggat2d_i_rv;
        *var_fvbirgat2d_i_slot = var_fvbirgat2d_i;
        *var_fvbirgat2d_i_rv_slot = var_fvbirgat2d_i_rv;
        *var_idsatrbotd_i_slot = var_idsatrbotd_i;
        *var_idsatrbotd_i_rv_slot = var_idsatrbotd_i_rv;
        *var_idsatrgatd_i_slot = var_idsatrgatd_i;
        *var_idsatrgatd_i_rv_slot = var_idsatrgatd_i_rv;
        *var_idsatrstid_i_slot = var_idsatrstid_i;
        *var_idsatrstid_i_rv_slot = var_idsatrstid_i_rv;
        *var_mefftatbotd_i_slot = var_mefftatbotd_i;
        *var_mefftatbotd_i_rv_slot = var_mefftatbotd_i_rv;
        *var_mefftatgatd_i_slot = var_mefftatgatd_i;
        *var_mefftatgatd_i_rv_slot = var_mefftatgatd_i_rv;
        *var_mefftatstid_i_slot = var_mefftatstid_i;
        *var_mefftatstid_i_rv_slot = var_mefftatstid_i_rv;
        *var_pbotd_i_slot = var_pbotd_i;
        *var_pbotd_i_rv_slot = var_pbotd_i_rv;
        *var_pbrbotd_i_slot = var_pbrbotd_i;
        *var_pbrbotd_i_rv_slot = var_pbrbotd_i_rv;
        *var_pbrgatd_i_slot = var_pbrgatd_i;
        *var_pbrgatd_i_rv_slot = var_pbrgatd_i_rv;
        *var_pbrstid_i_slot = var_pbrstid_i;
        *var_pbrstid_i_rv_slot = var_pbrstid_i_rv;
        *var_phigbotd_i_slot = var_phigbotd_i;
        *var_phigbotd_i_rv_slot = var_phigbotd_i_rv;
        *var_phiggatd_i_slot = var_phiggatd_i;
        *var_phiggatd_i_rv_slot = var_phiggatd_i_rv;
        *var_phigstid_i_slot = var_phigstid_i;
        *var_phigstid_i_rv_slot = var_phigstid_i_rv;
        *var_pstid_i_slot = var_pstid_i;
        *var_pstid_i_rv_slot = var_pstid_i_rv;
        *var_stfbbtbotd_i_slot = var_stfbbtbotd_i;
        *var_stfbbtbotd_i_rv_slot = var_stfbbtbotd_i_rv;
        *var_stfbbtgatd_i_slot = var_stfbbtgatd_i;
        *var_stfbbtgatd_i_rv_slot = var_stfbbtgatd_i_rv;
        *var_stfbbtstid_i_slot = var_stfbbtstid_i;
        *var_stfbbtstid_i_rv_slot = var_stfbbtstid_i_rv;
        *var_vbirbotd_i_slot = var_vbirbotd_i;
        *var_vbirbotd_i_rv_slot = var_vbirbotd_i_rv;
        *var_vbirgatd_i_slot = var_vbirgatd_i;
        *var_vbirgatd_i_rv_slot = var_vbirgatd_i_rv;
        *var_vbirstid_i_slot = var_vbirstid_i;
        *var_vbirstid_i_rv_slot = var_vbirstid_i_rv;
        *var_vbrbotd_i_slot = var_vbrbotd_i;
        *var_vbrbotd_i_rv_slot = var_vbrbotd_i_rv;
        *var_vbrgatd_i_slot = var_vbrgatd_i;
        *var_vbrgatd_i_rv_slot = var_vbrgatd_i_rv;
        *var_vbrstid_i_slot = var_vbrstid_i;
        *var_vbrstid_i_rv_slot = var_vbrstid_i_rv;
        *var_vtrgatd_i_slot = var_vtrgatd_i;
        *var_vtrgatd_i_rv_slot = var_vtrgatd_i_rv;
        *var_xjungatd_i_slot = var_xjungatd_i;
        *var_xjungatd_i_rv_slot = var_xjungatd_i_rv;
        *var_xjunstid_i_slot = var_xjunstid_i;
        *var_xjunstid_i_rv_slot = var_xjunstid_i_rv;
    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        var_cjorbotd_i: f64,
        var_cjorgatd_i: f64,
        var_cjorstid_i: f64,
        var_deltaphigr: f64,
        var_epssi: f64,
        var_guard10: f64,
        var_pbotd_i: f64,
        var_pstid_i: f64,
        var_vbirbotd_i: f64,
        var_vbirstid_i: f64,
        var_adbbtgatd_i_slot: &mut f64,
        var_adbbtgatd_i_rv_slot: &mut f64,
        var_advbrgatd_i_slot: &mut f64,
        var_advbrgatd_i_rv_slot: &mut f64,
        var_anugatd_i_slot: &mut f64,
        var_anugatd_i_rv_slot: &mut f64,
        var_bdbbtgatd_i_slot: &mut f64,
        var_bdbbtgatd_i_rv_slot: &mut f64,
        var_bdvbrgatd_i_slot: &mut f64,
        var_bdvbrgatd_i_rv_slot: &mut f64,
        var_cbbtbotd_i_slot: &mut f64,
        var_cbbtbotd_i_rv_slot: &mut f64,
        var_cbbtgatd_i_slot: &mut f64,
        var_cbbtgatd_i_rv_slot: &mut f64,
        var_cbbtstid_i_slot: &mut f64,
        var_cbbtstid_i_rv_slot: &mut f64,
        var_csrhbotd_i_slot: &mut f64,
        var_csrhbotd_i_rv_slot: &mut f64,
        var_csrhgatd_i_slot: &mut f64,
        var_csrhgatd_i_rv_slot: &mut f64,
        var_csrhstid_i_slot: &mut f64,
        var_csrhstid_i_rv_slot: &mut f64,
        var_ctatbotd_i_slot: &mut f64,
        var_ctatbotd_i_rv_slot: &mut f64,
        var_ctatgatd_i_slot: &mut f64,
        var_ctatgatd_i_rv_slot: &mut f64,
        var_ctatstid_i_slot: &mut f64,
        var_ctatstid_i_rv_slot: &mut f64,
        var_fbbtrbotd_i_slot: &mut f64,
        var_fbbtrbotd_i_rv_slot: &mut f64,
        var_fbbtrgatd_i_slot: &mut f64,
        var_fbbtrgatd_i_rv_slot: &mut f64,
        var_fbbtrstid_i_slot: &mut f64,
        var_fbbtrstid_i_rv_slot: &mut f64,
        var_fcjorgat2d_i_slot: &mut f64,
        var_fcjorgat2d_i_rv_slot: &mut f64,
        var_fjunqd_i_slot: &mut f64,
        var_fjunqd_i_rv_slot: &mut f64,
        var_fpgat2d_i_slot: &mut f64,
        var_fpgat2d_i_rv_slot: &mut f64,
        var_fphiggat2d_i_slot: &mut f64,
        var_fphiggat2d_i_rv_slot: &mut f64,
        var_fvbirgat2d_i_slot: &mut f64,
        var_fvbirgat2d_i_rv_slot: &mut f64,
        var_idsatrbotd_i_slot: &mut f64,
        var_idsatrbotd_i_rv_slot: &mut f64,
        var_idsatrgatd_i_slot: &mut f64,
        var_idsatrgatd_i_rv_slot: &mut f64,
        var_idsatrstid_i_slot: &mut f64,
        var_idsatrstid_i_rv_slot: &mut f64,
        var_mefftatbotd_i_slot: &mut f64,
        var_mefftatbotd_i_rv_slot: &mut f64,
        var_mefftatgatd_i_slot: &mut f64,
        var_mefftatgatd_i_rv_slot: &mut f64,
        var_mefftatstid_i_slot: &mut f64,
        var_mefftatstid_i_rv_slot: &mut f64,
        var_one_minus_pbot_d_slot: &mut f64,
        var_one_minus_pbot_d_rv_slot: &mut f64,
        var_one_minus_pgat_d_slot: &mut f64,
        var_one_minus_pgat_d_rv_slot: &mut f64,
        var_one_minus_psti_d_slot: &mut f64,
        var_one_minus_psti_d_rv_slot: &mut f64,
        var_one_over_one_minus_pbot_d_slot: &mut f64,
        var_one_over_one_minus_pbot_d_rv_slot: &mut f64,
        var_one_over_one_minus_pgat_d_slot: &mut f64,
        var_one_over_one_minus_pgat_d_rv_slot: &mut f64,
        var_one_over_one_minus_psti_d_slot: &mut f64,
        var_one_over_one_minus_psti_d_rv_slot: &mut f64,
        var_pbrbotd_i_slot: &mut f64,
        var_pbrbotd_i_rv_slot: &mut f64,
        var_pbrgatd_i_slot: &mut f64,
        var_pbrgatd_i_rv_slot: &mut f64,
        var_pbrstid_i_slot: &mut f64,
        var_pbrstid_i_rv_slot: &mut f64,
        var_pgatd_i_slot: &mut f64,
        var_pgatd_i_rv_slot: &mut f64,
        var_phigbotd_i_slot: &mut f64,
        var_phigbotd_i_rv_slot: &mut f64,
        var_phiggatd_i_slot: &mut f64,
        var_phiggatd_i_rv_slot: &mut f64,
        var_phigrbot_d_slot: &mut f64,
        var_phigrbot_d_rv_slot: &mut f64,
        var_phigrgat_d_slot: &mut f64,
        var_phigrgat_d_rv_slot: &mut f64,
        var_phigrsti_d_slot: &mut f64,
        var_phigrsti_d_rv_slot: &mut f64,
        var_phigstid_i_slot: &mut f64,
        var_phigstid_i_rv_slot: &mut f64,
        var_stfbbtbotd_i_slot: &mut f64,
        var_stfbbtbotd_i_rv_slot: &mut f64,
        var_stfbbtgatd_i_slot: &mut f64,
        var_stfbbtgatd_i_rv_slot: &mut f64,
        var_stfbbtstid_i_slot: &mut f64,
        var_stfbbtstid_i_rv_slot: &mut f64,
        var_vbirbotinv_d_slot: &mut f64,
        var_vbirbotinv_d_rv_slot: &mut f64,
        var_vbirstiinv_d_slot: &mut f64,
        var_vbirstiinv_d_rv_slot: &mut f64,
        var_vbrbotd_i_slot: &mut f64,
        var_vbrbotd_i_rv_slot: &mut f64,
        var_vbrgatd_i_slot: &mut f64,
        var_vbrgatd_i_rv_slot: &mut f64,
        var_vbrstid_i_slot: &mut f64,
        var_vbrstid_i_rv_slot: &mut f64,
        var_vtrgatd_i_slot: &mut f64,
        var_vtrgatd_i_rv_slot: &mut f64,
        var_wdepnulrbot_d_slot: &mut f64,
        var_wdepnulrbot_d_rv_slot: &mut f64,
        var_wdepnulrgat_d_slot: &mut f64,
        var_wdepnulrgat_d_rv_slot: &mut f64,
        var_wdepnulrinvbot_d_slot: &mut f64,
        var_wdepnulrinvbot_d_rv_slot: &mut f64,
        var_wdepnulrinvgat_d_slot: &mut f64,
        var_wdepnulrinvgat_d_rv_slot: &mut f64,
        var_wdepnulrinvsti_d_slot: &mut f64,
        var_wdepnulrinvsti_d_rv_slot: &mut f64,
        var_wdepnulrsti_d_slot: &mut f64,
        var_wdepnulrsti_d_rv_slot: &mut f64,
        var_xjungatd_i_slot: &mut f64,
        var_xjungatd_i_rv_slot: &mut f64,
        var_xjunstid_i_slot: &mut f64,
        var_xjunstid_i_rv_slot: &mut f64,
    ) {
        let mut var_adbbtgatd_i: f64 = *var_adbbtgatd_i_slot;
        let mut var_adbbtgatd_i_rv: f64 = *var_adbbtgatd_i_rv_slot;
        let mut var_advbrgatd_i: f64 = *var_advbrgatd_i_slot;
        let mut var_advbrgatd_i_rv: f64 = *var_advbrgatd_i_rv_slot;
        let mut var_anugatd_i: f64 = *var_anugatd_i_slot;
        let mut var_anugatd_i_rv: f64 = *var_anugatd_i_rv_slot;
        let mut var_bdbbtgatd_i: f64 = *var_bdbbtgatd_i_slot;
        let mut var_bdbbtgatd_i_rv: f64 = *var_bdbbtgatd_i_rv_slot;
        let mut var_bdvbrgatd_i: f64 = *var_bdvbrgatd_i_slot;
        let mut var_bdvbrgatd_i_rv: f64 = *var_bdvbrgatd_i_rv_slot;
        let mut var_cbbtbotd_i: f64 = *var_cbbtbotd_i_slot;
        let mut var_cbbtbotd_i_rv: f64 = *var_cbbtbotd_i_rv_slot;
        let mut var_cbbtgatd_i: f64 = *var_cbbtgatd_i_slot;
        let mut var_cbbtgatd_i_rv: f64 = *var_cbbtgatd_i_rv_slot;
        let mut var_cbbtstid_i: f64 = *var_cbbtstid_i_slot;
        let mut var_cbbtstid_i_rv: f64 = *var_cbbtstid_i_rv_slot;
        let mut var_csrhbotd_i: f64 = *var_csrhbotd_i_slot;
        let mut var_csrhbotd_i_rv: f64 = *var_csrhbotd_i_rv_slot;
        let mut var_csrhgatd_i: f64 = *var_csrhgatd_i_slot;
        let mut var_csrhgatd_i_rv: f64 = *var_csrhgatd_i_rv_slot;
        let mut var_csrhstid_i: f64 = *var_csrhstid_i_slot;
        let mut var_csrhstid_i_rv: f64 = *var_csrhstid_i_rv_slot;
        let mut var_ctatbotd_i: f64 = *var_ctatbotd_i_slot;
        let mut var_ctatbotd_i_rv: f64 = *var_ctatbotd_i_rv_slot;
        let mut var_ctatgatd_i: f64 = *var_ctatgatd_i_slot;
        let mut var_ctatgatd_i_rv: f64 = *var_ctatgatd_i_rv_slot;
        let mut var_ctatstid_i: f64 = *var_ctatstid_i_slot;
        let mut var_ctatstid_i_rv: f64 = *var_ctatstid_i_rv_slot;
        let mut var_fbbtrbotd_i: f64 = *var_fbbtrbotd_i_slot;
        let mut var_fbbtrbotd_i_rv: f64 = *var_fbbtrbotd_i_rv_slot;
        let mut var_fbbtrgatd_i: f64 = *var_fbbtrgatd_i_slot;
        let mut var_fbbtrgatd_i_rv: f64 = *var_fbbtrgatd_i_rv_slot;
        let mut var_fbbtrstid_i: f64 = *var_fbbtrstid_i_slot;
        let mut var_fbbtrstid_i_rv: f64 = *var_fbbtrstid_i_rv_slot;
        let mut var_fcjorgat2d_i: f64 = *var_fcjorgat2d_i_slot;
        let mut var_fcjorgat2d_i_rv: f64 = *var_fcjorgat2d_i_rv_slot;
        let mut var_fjunqd_i: f64 = *var_fjunqd_i_slot;
        let mut var_fjunqd_i_rv: f64 = *var_fjunqd_i_rv_slot;
        let mut var_fpgat2d_i: f64 = *var_fpgat2d_i_slot;
        let mut var_fpgat2d_i_rv: f64 = *var_fpgat2d_i_rv_slot;
        let mut var_fphiggat2d_i: f64 = *var_fphiggat2d_i_slot;
        let mut var_fphiggat2d_i_rv: f64 = *var_fphiggat2d_i_rv_slot;
        let mut var_fvbirgat2d_i: f64 = *var_fvbirgat2d_i_slot;
        let mut var_fvbirgat2d_i_rv: f64 = *var_fvbirgat2d_i_rv_slot;
        let mut var_idsatrbotd_i: f64 = *var_idsatrbotd_i_slot;
        let mut var_idsatrbotd_i_rv: f64 = *var_idsatrbotd_i_rv_slot;
        let mut var_idsatrgatd_i: f64 = *var_idsatrgatd_i_slot;
        let mut var_idsatrgatd_i_rv: f64 = *var_idsatrgatd_i_rv_slot;
        let mut var_idsatrstid_i: f64 = *var_idsatrstid_i_slot;
        let mut var_idsatrstid_i_rv: f64 = *var_idsatrstid_i_rv_slot;
        let mut var_mefftatbotd_i: f64 = *var_mefftatbotd_i_slot;
        let mut var_mefftatbotd_i_rv: f64 = *var_mefftatbotd_i_rv_slot;
        let mut var_mefftatgatd_i: f64 = *var_mefftatgatd_i_slot;
        let mut var_mefftatgatd_i_rv: f64 = *var_mefftatgatd_i_rv_slot;
        let mut var_mefftatstid_i: f64 = *var_mefftatstid_i_slot;
        let mut var_mefftatstid_i_rv: f64 = *var_mefftatstid_i_rv_slot;
        let mut var_one_minus_pbot_d: f64 = *var_one_minus_pbot_d_slot;
        let mut var_one_minus_pbot_d_rv: f64 = *var_one_minus_pbot_d_rv_slot;
        let mut var_one_minus_pgat_d: f64 = *var_one_minus_pgat_d_slot;
        let mut var_one_minus_pgat_d_rv: f64 = *var_one_minus_pgat_d_rv_slot;
        let mut var_one_minus_psti_d: f64 = *var_one_minus_psti_d_slot;
        let mut var_one_minus_psti_d_rv: f64 = *var_one_minus_psti_d_rv_slot;
        let mut var_one_over_one_minus_pbot_d: f64 = *var_one_over_one_minus_pbot_d_slot;
        let mut var_one_over_one_minus_pbot_d_rv: f64 = *var_one_over_one_minus_pbot_d_rv_slot;
        let mut var_one_over_one_minus_pgat_d: f64 = *var_one_over_one_minus_pgat_d_slot;
        let mut var_one_over_one_minus_pgat_d_rv: f64 = *var_one_over_one_minus_pgat_d_rv_slot;
        let mut var_one_over_one_minus_psti_d: f64 = *var_one_over_one_minus_psti_d_slot;
        let mut var_one_over_one_minus_psti_d_rv: f64 = *var_one_over_one_minus_psti_d_rv_slot;
        let mut var_pbrbotd_i: f64 = *var_pbrbotd_i_slot;
        let mut var_pbrbotd_i_rv: f64 = *var_pbrbotd_i_rv_slot;
        let mut var_pbrgatd_i: f64 = *var_pbrgatd_i_slot;
        let mut var_pbrgatd_i_rv: f64 = *var_pbrgatd_i_rv_slot;
        let mut var_pbrstid_i: f64 = *var_pbrstid_i_slot;
        let mut var_pbrstid_i_rv: f64 = *var_pbrstid_i_rv_slot;
        let mut var_pgatd_i: f64 = *var_pgatd_i_slot;
        let mut var_pgatd_i_rv: f64 = *var_pgatd_i_rv_slot;
        let mut var_phigbotd_i: f64 = *var_phigbotd_i_slot;
        let mut var_phigbotd_i_rv: f64 = *var_phigbotd_i_rv_slot;
        let mut var_phiggatd_i: f64 = *var_phiggatd_i_slot;
        let mut var_phiggatd_i_rv: f64 = *var_phiggatd_i_rv_slot;
        let mut var_phigrbot_d: f64 = *var_phigrbot_d_slot;
        let mut var_phigrbot_d_rv: f64 = *var_phigrbot_d_rv_slot;
        let mut var_phigrgat_d: f64 = *var_phigrgat_d_slot;
        let mut var_phigrgat_d_rv: f64 = *var_phigrgat_d_rv_slot;
        let mut var_phigrsti_d: f64 = *var_phigrsti_d_slot;
        let mut var_phigrsti_d_rv: f64 = *var_phigrsti_d_rv_slot;
        let mut var_phigstid_i: f64 = *var_phigstid_i_slot;
        let mut var_phigstid_i_rv: f64 = *var_phigstid_i_rv_slot;
        let mut var_stfbbtbotd_i: f64 = *var_stfbbtbotd_i_slot;
        let mut var_stfbbtbotd_i_rv: f64 = *var_stfbbtbotd_i_rv_slot;
        let mut var_stfbbtgatd_i: f64 = *var_stfbbtgatd_i_slot;
        let mut var_stfbbtgatd_i_rv: f64 = *var_stfbbtgatd_i_rv_slot;
        let mut var_stfbbtstid_i: f64 = *var_stfbbtstid_i_slot;
        let mut var_stfbbtstid_i_rv: f64 = *var_stfbbtstid_i_rv_slot;
        let mut var_vbirbotinv_d: f64 = *var_vbirbotinv_d_slot;
        let mut var_vbirbotinv_d_rv: f64 = *var_vbirbotinv_d_rv_slot;
        let mut var_vbirstiinv_d: f64 = *var_vbirstiinv_d_slot;
        let mut var_vbirstiinv_d_rv: f64 = *var_vbirstiinv_d_rv_slot;
        let mut var_vbrbotd_i: f64 = *var_vbrbotd_i_slot;
        let mut var_vbrbotd_i_rv: f64 = *var_vbrbotd_i_rv_slot;
        let mut var_vbrgatd_i: f64 = *var_vbrgatd_i_slot;
        let mut var_vbrgatd_i_rv: f64 = *var_vbrgatd_i_rv_slot;
        let mut var_vbrstid_i: f64 = *var_vbrstid_i_slot;
        let mut var_vbrstid_i_rv: f64 = *var_vbrstid_i_rv_slot;
        let mut var_vtrgatd_i: f64 = *var_vtrgatd_i_slot;
        let mut var_vtrgatd_i_rv: f64 = *var_vtrgatd_i_rv_slot;
        let mut var_wdepnulrbot_d: f64 = *var_wdepnulrbot_d_slot;
        let mut var_wdepnulrbot_d_rv: f64 = *var_wdepnulrbot_d_rv_slot;
        let mut var_wdepnulrgat_d: f64 = *var_wdepnulrgat_d_slot;
        let mut var_wdepnulrgat_d_rv: f64 = *var_wdepnulrgat_d_rv_slot;
        let mut var_wdepnulrinvbot_d: f64 = *var_wdepnulrinvbot_d_slot;
        let mut var_wdepnulrinvbot_d_rv: f64 = *var_wdepnulrinvbot_d_rv_slot;
        let mut var_wdepnulrinvgat_d: f64 = *var_wdepnulrinvgat_d_slot;
        let mut var_wdepnulrinvgat_d_rv: f64 = *var_wdepnulrinvgat_d_rv_slot;
        let mut var_wdepnulrinvsti_d: f64 = *var_wdepnulrinvsti_d_slot;
        let mut var_wdepnulrinvsti_d_rv: f64 = *var_wdepnulrinvsti_d_rv_slot;
        let mut var_wdepnulrsti_d: f64 = *var_wdepnulrsti_d_slot;
        let mut var_wdepnulrsti_d_rv: f64 = *var_wdepnulrsti_d_rv_slot;
        let mut var_xjungatd_i: f64 = *var_xjungatd_i_slot;
        let mut var_xjungatd_i_rv: f64 = *var_xjungatd_i_rv_slot;
        let mut var_xjunstid_i: f64 = *var_xjunstid_i_slot;
        let mut var_xjunstid_i_rv: f64 = *var_xjunstid_i_rv_slot;

        let (assign1320_e2228,) = {
    if (var_guard10 == 0.0) {
        (p.p884,)
    } else {
        (var_pgatd_i,)
    }
};
        var_pgatd_i = assign1320_e2228;
        var_pgatd_i_rv = 0.0;

        let (assign1330_e2233,) = {
    if (var_guard10 == 0.0) {
        (p.p885,)
    } else {
        (var_phigbotd_i,)
    }
};
        var_phigbotd_i = assign1330_e2233;
        var_phigbotd_i_rv = 0.0;

        let (assign1340_e2238,) = {
    if (var_guard10 == 0.0) {
        (p.p886,)
    } else {
        (var_phigstid_i,)
    }
};
        var_phigstid_i = assign1340_e2238;
        var_phigstid_i_rv = 0.0;

        let (assign1350_e2243,) = {
    if (var_guard10 == 0.0) {
        (p.p887,)
    } else {
        (var_phiggatd_i,)
    }
};
        var_phiggatd_i = assign1350_e2243;
        var_phiggatd_i_rv = 0.0;

        let (assign1360_e2248,) = {
    if (var_guard10 == 0.0) {
        (p.p888,)
    } else {
        (var_idsatrbotd_i,)
    }
};
        var_idsatrbotd_i = assign1360_e2248;
        var_idsatrbotd_i_rv = 0.0;

        let (assign1370_e2253,) = {
    if (var_guard10 == 0.0) {
        (p.p889,)
    } else {
        (var_idsatrstid_i,)
    }
};
        var_idsatrstid_i = assign1370_e2253;
        var_idsatrstid_i_rv = 0.0;

        let (assign1380_e2258,) = {
    if (var_guard10 == 0.0) {
        (p.p890,)
    } else {
        (var_idsatrgatd_i,)
    }
};
        var_idsatrgatd_i = assign1380_e2258;
        var_idsatrgatd_i_rv = 0.0;

        let (assign1390_e2263,) = {
    if (var_guard10 == 0.0) {
        (p.p891,)
    } else {
        (var_csrhbotd_i,)
    }
};
        var_csrhbotd_i = assign1390_e2263;
        var_csrhbotd_i_rv = 0.0;

        let (assign1400_e2268,) = {
    if (var_guard10 == 0.0) {
        (p.p892,)
    } else {
        (var_csrhstid_i,)
    }
};
        var_csrhstid_i = assign1400_e2268;
        var_csrhstid_i_rv = 0.0;

        let (assign1410_e2273,) = {
    if (var_guard10 == 0.0) {
        (p.p893,)
    } else {
        (var_csrhgatd_i,)
    }
};
        var_csrhgatd_i = assign1410_e2273;
        var_csrhgatd_i_rv = 0.0;

        let (assign1420_e2278,) = {
    if (var_guard10 == 0.0) {
        (p.p894,)
    } else {
        (var_xjunstid_i,)
    }
};
        var_xjunstid_i = assign1420_e2278;
        var_xjunstid_i_rv = 0.0;

        let (assign1430_e2283,) = {
    if (var_guard10 == 0.0) {
        (p.p895,)
    } else {
        (var_xjungatd_i,)
    }
};
        var_xjungatd_i = assign1430_e2283;
        var_xjungatd_i_rv = 0.0;

        let (assign1440_e2288,) = {
    if (var_guard10 == 0.0) {
        (p.p896,)
    } else {
        (var_ctatbotd_i,)
    }
};
        var_ctatbotd_i = assign1440_e2288;
        var_ctatbotd_i_rv = 0.0;

        let (assign1450_e2293,) = {
    if (var_guard10 == 0.0) {
        (p.p897,)
    } else {
        (var_ctatstid_i,)
    }
};
        var_ctatstid_i = assign1450_e2293;
        var_ctatstid_i_rv = 0.0;

        let (assign1460_e2298,) = {
    if (var_guard10 == 0.0) {
        (p.p898,)
    } else {
        (var_ctatgatd_i,)
    }
};
        var_ctatgatd_i = assign1460_e2298;
        var_ctatgatd_i_rv = 0.0;

        let (assign1470_e2303,) = {
    if (var_guard10 == 0.0) {
        (p.p899,)
    } else {
        (var_mefftatbotd_i,)
    }
};
        var_mefftatbotd_i = assign1470_e2303;
        var_mefftatbotd_i_rv = 0.0;

        let (assign1480_e2308,) = {
    if (var_guard10 == 0.0) {
        (p.p900,)
    } else {
        (var_mefftatstid_i,)
    }
};
        var_mefftatstid_i = assign1480_e2308;
        var_mefftatstid_i_rv = 0.0;

        let (assign1490_e2313,) = {
    if (var_guard10 == 0.0) {
        (p.p901,)
    } else {
        (var_mefftatgatd_i,)
    }
};
        var_mefftatgatd_i = assign1490_e2313;
        var_mefftatgatd_i_rv = 0.0;

        let (assign1500_e2318,) = {
    if (var_guard10 == 0.0) {
        (p.p902,)
    } else {
        (var_cbbtbotd_i,)
    }
};
        var_cbbtbotd_i = assign1500_e2318;
        var_cbbtbotd_i_rv = 0.0;

        let (assign1510_e2323,) = {
    if (var_guard10 == 0.0) {
        (p.p903,)
    } else {
        (var_cbbtstid_i,)
    }
};
        var_cbbtstid_i = assign1510_e2323;
        var_cbbtstid_i_rv = 0.0;

        let (assign1520_e2328,) = {
    if (var_guard10 == 0.0) {
        (p.p904,)
    } else {
        (var_cbbtgatd_i,)
    }
};
        var_cbbtgatd_i = assign1520_e2328;
        var_cbbtgatd_i_rv = 0.0;

        let (assign1530_e2333,) = {
    if (var_guard10 == 0.0) {
        (p.p905,)
    } else {
        (var_fbbtrbotd_i,)
    }
};
        var_fbbtrbotd_i = assign1530_e2333;
        var_fbbtrbotd_i_rv = 0.0;

        let (assign1540_e2338,) = {
    if (var_guard10 == 0.0) {
        (p.p906,)
    } else {
        (var_fbbtrstid_i,)
    }
};
        var_fbbtrstid_i = assign1540_e2338;
        var_fbbtrstid_i_rv = 0.0;

        let (assign1550_e2343,) = {
    if (var_guard10 == 0.0) {
        (p.p907,)
    } else {
        (var_fbbtrgatd_i,)
    }
};
        var_fbbtrgatd_i = assign1550_e2343;
        var_fbbtrgatd_i_rv = 0.0;

        let (assign1560_e2348,) = {
    if (var_guard10 == 0.0) {
        (p.p908,)
    } else {
        (var_stfbbtbotd_i,)
    }
};
        var_stfbbtbotd_i = assign1560_e2348;
        var_stfbbtbotd_i_rv = 0.0;

        let (assign1570_e2353,) = {
    if (var_guard10 == 0.0) {
        (p.p909,)
    } else {
        (var_stfbbtstid_i,)
    }
};
        var_stfbbtstid_i = assign1570_e2353;
        var_stfbbtstid_i_rv = 0.0;

        let (assign1580_e2358,) = {
    if (var_guard10 == 0.0) {
        (p.p910,)
    } else {
        (var_stfbbtgatd_i,)
    }
};
        var_stfbbtgatd_i = assign1580_e2358;
        var_stfbbtgatd_i_rv = 0.0;

        let (assign1590_e2363,) = {
    if (var_guard10 == 0.0) {
        (p.p911,)
    } else {
        (var_vbrbotd_i,)
    }
};
        var_vbrbotd_i = assign1590_e2363;
        var_vbrbotd_i_rv = 0.0;

        let (assign1600_e2368,) = {
    if (var_guard10 == 0.0) {
        (p.p912,)
    } else {
        (var_vbrstid_i,)
    }
};
        var_vbrstid_i = assign1600_e2368;
        var_vbrstid_i_rv = 0.0;

        let (assign1610_e2373,) = {
    if (var_guard10 == 0.0) {
        (p.p913,)
    } else {
        (var_vbrgatd_i,)
    }
};
        var_vbrgatd_i = assign1610_e2373;
        var_vbrgatd_i_rv = 0.0;

        let (assign1620_e2378,) = {
    if (var_guard10 == 0.0) {
        (p.p914,)
    } else {
        (var_pbrbotd_i,)
    }
};
        var_pbrbotd_i = assign1620_e2378;
        var_pbrbotd_i_rv = 0.0;

        let (assign1630_e2383,) = {
    if (var_guard10 == 0.0) {
        (p.p915,)
    } else {
        (var_pbrstid_i,)
    }
};
        var_pbrstid_i = assign1630_e2383;
        var_pbrstid_i_rv = 0.0;

        let (assign1640_e2388,) = {
    if (var_guard10 == 0.0) {
        (p.p916,)
    } else {
        (var_pbrgatd_i,)
    }
};
        var_pbrgatd_i = assign1640_e2388;
        var_pbrgatd_i_rv = 0.0;

        let (assign1660_e2398,) = {
    if (var_guard10 == 0.0) {
        (p.p931,)
    } else {
        (var_fjunqd_i,)
    }
};
        var_fjunqd_i = assign1660_e2398;
        var_fjunqd_i_rv = 0.0;

        let (assign1670_e2403,) = {
    if (var_guard10 == 0.0) {
        (p.p923,)
    } else {
        (var_advbrgatd_i,)
    }
};
        var_advbrgatd_i = assign1670_e2403;
        var_advbrgatd_i_rv = 0.0;

        let (assign1680_e2408,) = {
    if (var_guard10 == 0.0) {
        (p.p924,)
    } else {
        (var_bdvbrgatd_i,)
    }
};
        var_bdvbrgatd_i = assign1680_e2408;
        var_bdvbrgatd_i_rv = 0.0;

        let (assign1690_e2413,) = {
    if (var_guard10 == 0.0) {
        (p.p925,)
    } else {
        (var_adbbtgatd_i,)
    }
};
        var_adbbtgatd_i = assign1690_e2413;
        var_adbbtgatd_i_rv = 0.0;

        let (assign1700_e2418,) = {
    if (var_guard10 == 0.0) {
        (p.p926,)
    } else {
        (var_bdbbtgatd_i,)
    }
};
        var_bdbbtgatd_i = assign1700_e2418;
        var_bdbbtgatd_i_rv = 0.0;

        let (assign1710_e2423,) = {
    if (var_guard10 == 0.0) {
        (p.p917,)
    } else {
        (var_fcjorgat2d_i,)
    }
};
        var_fcjorgat2d_i = assign1710_e2423;
        var_fcjorgat2d_i_rv = 0.0;

        let (assign1720_e2428,) = {
    if (var_guard10 == 0.0) {
        (p.p918,)
    } else {
        (var_fvbirgat2d_i,)
    }
};
        var_fvbirgat2d_i = assign1720_e2428;
        var_fvbirgat2d_i_rv = 0.0;

        let (assign1730_e2433,) = {
    if (var_guard10 == 0.0) {
        (p.p919,)
    } else {
        (var_fpgat2d_i,)
    }
};
        var_fpgat2d_i = assign1730_e2433;
        var_fpgat2d_i_rv = 0.0;

        let (assign1740_e2438,) = {
    if (var_guard10 == 0.0) {
        (p.p920,)
    } else {
        (var_fphiggat2d_i,)
    }
};
        var_fphiggat2d_i = assign1740_e2438;
        var_fphiggat2d_i_rv = 0.0;

        let (assign1750_e2443,) = {
    if (var_guard10 == 0.0) {
        (p.p921,)
    } else {
        (var_vtrgatd_i,)
    }
};
        var_vtrgatd_i = assign1750_e2443;
        var_vtrgatd_i_rv = 0.0;

        let (assign1760_e2448,) = {
    if (var_guard10 == 0.0) {
        (p.p922,)
    } else {
        (var_anugatd_i,)
    }
};
        var_anugatd_i = assign1760_e2448;
        var_anugatd_i_rv = 0.0;

        let assign1770_e2451: f64 = (var_phigbotd_i + var_deltaphigr);
        var_phigrbot_d = assign1770_e2451;
        var_phigrbot_d_rv = 0.0;

        let assign1780_e2454: f64 = (var_phigstid_i + var_deltaphigr);
        var_phigrsti_d = assign1780_e2454;
        var_phigrsti_d_rv = 0.0;

        let assign1790_e2457: f64 = (var_phiggatd_i + var_deltaphigr);
        var_phigrgat_d = assign1790_e2457;
        var_phigrgat_d_rv = 0.0;

        let assign1800_e2460: f64 = (1.0 - var_pbotd_i);
        var_one_minus_pbot_d = assign1800_e2460;
        var_one_minus_pbot_d_rv = 0.0;

        let assign1810_e2463: f64 = (1.0 - var_pstid_i);
        var_one_minus_psti_d = assign1810_e2463;
        var_one_minus_psti_d_rv = 0.0;

        let assign1820_e2466: f64 = (1.0 - var_pgatd_i);
        var_one_minus_pgat_d = assign1820_e2466;
        var_one_minus_pgat_d_rv = 0.0;

        let assign1830_e2469: f64 = (1.0 / var_one_minus_pbot_d);
        var_one_over_one_minus_pbot_d = assign1830_e2469;
        var_one_over_one_minus_pbot_d_rv = 0.0;

        let assign1840_e2472: f64 = (1.0 / var_one_minus_psti_d);
        var_one_over_one_minus_psti_d = assign1840_e2472;
        var_one_over_one_minus_psti_d_rv = 0.0;

        let assign1850_e2475: f64 = (1.0 / var_one_minus_pgat_d);
        var_one_over_one_minus_pgat_d = assign1850_e2475;
        var_one_over_one_minus_pgat_d_rv = 0.0;

        let assign1860_e2478: f64 = (var_epssi / var_cjorbotd_i);
        var_wdepnulrbot_d = assign1860_e2478;
        var_wdepnulrbot_d_rv = 0.0;

        let assign1870_e2481: f64 = (var_xjunstid_i * var_epssi);
        let assign1870_e2483: f64 = (assign1870_e2481 / var_cjorstid_i);
        var_wdepnulrsti_d = assign1870_e2483;
        var_wdepnulrsti_d_rv = 0.0;

        let assign1880_e2486: f64 = (var_xjungatd_i * var_epssi);
        let assign1880_e2488: f64 = (assign1880_e2486 / var_cjorgatd_i);
        var_wdepnulrgat_d = assign1880_e2488;
        var_wdepnulrgat_d_rv = 0.0;

        let assign1890_e2491: f64 = (1.0 / var_wdepnulrbot_d);
        var_wdepnulrinvbot_d = assign1890_e2491;
        var_wdepnulrinvbot_d_rv = 0.0;

        let assign1900_e2494: f64 = (1.0 / var_wdepnulrsti_d);
        var_wdepnulrinvsti_d = assign1900_e2494;
        var_wdepnulrinvsti_d_rv = 0.0;

        let assign1910_e2497: f64 = (1.0 / var_wdepnulrgat_d);
        var_wdepnulrinvgat_d = assign1910_e2497;
        var_wdepnulrinvgat_d_rv = 0.0;

        let assign1920_e2500: f64 = (1.0 / var_vbirbotd_i);
        var_vbirbotinv_d = assign1920_e2500;
        var_vbirbotinv_d_rv = 0.0;

        let assign1930_e2503: f64 = (1.0 / var_vbirstid_i);
        var_vbirstiinv_d = assign1930_e2503;
        var_vbirstiinv_d_rv = 0.0;

        *var_adbbtgatd_i_slot = var_adbbtgatd_i;
        *var_adbbtgatd_i_rv_slot = var_adbbtgatd_i_rv;
        *var_advbrgatd_i_slot = var_advbrgatd_i;
        *var_advbrgatd_i_rv_slot = var_advbrgatd_i_rv;
        *var_anugatd_i_slot = var_anugatd_i;
        *var_anugatd_i_rv_slot = var_anugatd_i_rv;
        *var_bdbbtgatd_i_slot = var_bdbbtgatd_i;
        *var_bdbbtgatd_i_rv_slot = var_bdbbtgatd_i_rv;
        *var_bdvbrgatd_i_slot = var_bdvbrgatd_i;
        *var_bdvbrgatd_i_rv_slot = var_bdvbrgatd_i_rv;
        *var_cbbtbotd_i_slot = var_cbbtbotd_i;
        *var_cbbtbotd_i_rv_slot = var_cbbtbotd_i_rv;
        *var_cbbtgatd_i_slot = var_cbbtgatd_i;
        *var_cbbtgatd_i_rv_slot = var_cbbtgatd_i_rv;
        *var_cbbtstid_i_slot = var_cbbtstid_i;
        *var_cbbtstid_i_rv_slot = var_cbbtstid_i_rv;
        *var_csrhbotd_i_slot = var_csrhbotd_i;
        *var_csrhbotd_i_rv_slot = var_csrhbotd_i_rv;
        *var_csrhgatd_i_slot = var_csrhgatd_i;
        *var_csrhgatd_i_rv_slot = var_csrhgatd_i_rv;
        *var_csrhstid_i_slot = var_csrhstid_i;
        *var_csrhstid_i_rv_slot = var_csrhstid_i_rv;
        *var_ctatbotd_i_slot = var_ctatbotd_i;
        *var_ctatbotd_i_rv_slot = var_ctatbotd_i_rv;
        *var_ctatgatd_i_slot = var_ctatgatd_i;
        *var_ctatgatd_i_rv_slot = var_ctatgatd_i_rv;
        *var_ctatstid_i_slot = var_ctatstid_i;
        *var_ctatstid_i_rv_slot = var_ctatstid_i_rv;
        *var_fbbtrbotd_i_slot = var_fbbtrbotd_i;
        *var_fbbtrbotd_i_rv_slot = var_fbbtrbotd_i_rv;
        *var_fbbtrgatd_i_slot = var_fbbtrgatd_i;
        *var_fbbtrgatd_i_rv_slot = var_fbbtrgatd_i_rv;
        *var_fbbtrstid_i_slot = var_fbbtrstid_i;
        *var_fbbtrstid_i_rv_slot = var_fbbtrstid_i_rv;
        *var_fcjorgat2d_i_slot = var_fcjorgat2d_i;
        *var_fcjorgat2d_i_rv_slot = var_fcjorgat2d_i_rv;
        *var_fjunqd_i_slot = var_fjunqd_i;
        *var_fjunqd_i_rv_slot = var_fjunqd_i_rv;
        *var_fpgat2d_i_slot = var_fpgat2d_i;
        *var_fpgat2d_i_rv_slot = var_fpgat2d_i_rv;
        *var_fphiggat2d_i_slot = var_fphiggat2d_i;
        *var_fphiggat2d_i_rv_slot = var_fphiggat2d_i_rv;
        *var_fvbirgat2d_i_slot = var_fvbirgat2d_i;
        *var_fvbirgat2d_i_rv_slot = var_fvbirgat2d_i_rv;
        *var_idsatrbotd_i_slot = var_idsatrbotd_i;
        *var_idsatrbotd_i_rv_slot = var_idsatrbotd_i_rv;
        *var_idsatrgatd_i_slot = var_idsatrgatd_i;
        *var_idsatrgatd_i_rv_slot = var_idsatrgatd_i_rv;
        *var_idsatrstid_i_slot = var_idsatrstid_i;
        *var_idsatrstid_i_rv_slot = var_idsatrstid_i_rv;
        *var_mefftatbotd_i_slot = var_mefftatbotd_i;
        *var_mefftatbotd_i_rv_slot = var_mefftatbotd_i_rv;
        *var_mefftatgatd_i_slot = var_mefftatgatd_i;
        *var_mefftatgatd_i_rv_slot = var_mefftatgatd_i_rv;
        *var_mefftatstid_i_slot = var_mefftatstid_i;
        *var_mefftatstid_i_rv_slot = var_mefftatstid_i_rv;
        *var_one_minus_pbot_d_slot = var_one_minus_pbot_d;
        *var_one_minus_pbot_d_rv_slot = var_one_minus_pbot_d_rv;
        *var_one_minus_pgat_d_slot = var_one_minus_pgat_d;
        *var_one_minus_pgat_d_rv_slot = var_one_minus_pgat_d_rv;
        *var_one_minus_psti_d_slot = var_one_minus_psti_d;
        *var_one_minus_psti_d_rv_slot = var_one_minus_psti_d_rv;
        *var_one_over_one_minus_pbot_d_slot = var_one_over_one_minus_pbot_d;
        *var_one_over_one_minus_pbot_d_rv_slot = var_one_over_one_minus_pbot_d_rv;
        *var_one_over_one_minus_pgat_d_slot = var_one_over_one_minus_pgat_d;
        *var_one_over_one_minus_pgat_d_rv_slot = var_one_over_one_minus_pgat_d_rv;
        *var_one_over_one_minus_psti_d_slot = var_one_over_one_minus_psti_d;
        *var_one_over_one_minus_psti_d_rv_slot = var_one_over_one_minus_psti_d_rv;
        *var_pbrbotd_i_slot = var_pbrbotd_i;
        *var_pbrbotd_i_rv_slot = var_pbrbotd_i_rv;
        *var_pbrgatd_i_slot = var_pbrgatd_i;
        *var_pbrgatd_i_rv_slot = var_pbrgatd_i_rv;
        *var_pbrstid_i_slot = var_pbrstid_i;
        *var_pbrstid_i_rv_slot = var_pbrstid_i_rv;
        *var_pgatd_i_slot = var_pgatd_i;
        *var_pgatd_i_rv_slot = var_pgatd_i_rv;
        *var_phigbotd_i_slot = var_phigbotd_i;
        *var_phigbotd_i_rv_slot = var_phigbotd_i_rv;
        *var_phiggatd_i_slot = var_phiggatd_i;
        *var_phiggatd_i_rv_slot = var_phiggatd_i_rv;
        *var_phigrbot_d_slot = var_phigrbot_d;
        *var_phigrbot_d_rv_slot = var_phigrbot_d_rv;
        *var_phigrgat_d_slot = var_phigrgat_d;
        *var_phigrgat_d_rv_slot = var_phigrgat_d_rv;
        *var_phigrsti_d_slot = var_phigrsti_d;
        *var_phigrsti_d_rv_slot = var_phigrsti_d_rv;
        *var_phigstid_i_slot = var_phigstid_i;
        *var_phigstid_i_rv_slot = var_phigstid_i_rv;
        *var_stfbbtbotd_i_slot = var_stfbbtbotd_i;
        *var_stfbbtbotd_i_rv_slot = var_stfbbtbotd_i_rv;
        *var_stfbbtgatd_i_slot = var_stfbbtgatd_i;
        *var_stfbbtgatd_i_rv_slot = var_stfbbtgatd_i_rv;
        *var_stfbbtstid_i_slot = var_stfbbtstid_i;
        *var_stfbbtstid_i_rv_slot = var_stfbbtstid_i_rv;
        *var_vbirbotinv_d_slot = var_vbirbotinv_d;
        *var_vbirbotinv_d_rv_slot = var_vbirbotinv_d_rv;
        *var_vbirstiinv_d_slot = var_vbirstiinv_d;
        *var_vbirstiinv_d_rv_slot = var_vbirstiinv_d_rv;
        *var_vbrbotd_i_slot = var_vbrbotd_i;
        *var_vbrbotd_i_rv_slot = var_vbrbotd_i_rv;
        *var_vbrgatd_i_slot = var_vbrgatd_i;
        *var_vbrgatd_i_rv_slot = var_vbrgatd_i_rv;
        *var_vbrstid_i_slot = var_vbrstid_i;
        *var_vbrstid_i_rv_slot = var_vbrstid_i_rv;
        *var_vtrgatd_i_slot = var_vtrgatd_i;
        *var_vtrgatd_i_rv_slot = var_vtrgatd_i_rv;
        *var_wdepnulrbot_d_slot = var_wdepnulrbot_d;
        *var_wdepnulrbot_d_rv_slot = var_wdepnulrbot_d_rv;
        *var_wdepnulrgat_d_slot = var_wdepnulrgat_d;
        *var_wdepnulrgat_d_rv_slot = var_wdepnulrgat_d_rv;
        *var_wdepnulrinvbot_d_slot = var_wdepnulrinvbot_d;
        *var_wdepnulrinvbot_d_rv_slot = var_wdepnulrinvbot_d_rv;
        *var_wdepnulrinvgat_d_slot = var_wdepnulrinvgat_d;
        *var_wdepnulrinvgat_d_rv_slot = var_wdepnulrinvgat_d_rv;
        *var_wdepnulrinvsti_d_slot = var_wdepnulrinvsti_d;
        *var_wdepnulrinvsti_d_rv_slot = var_wdepnulrinvsti_d_rv;
        *var_wdepnulrsti_d_slot = var_wdepnulrsti_d;
        *var_wdepnulrsti_d_rv_slot = var_wdepnulrsti_d_rv;
        *var_xjungatd_i_slot = var_xjungatd_i;
        *var_xjungatd_i_rv_slot = var_xjungatd_i_rv;
        *var_xjunstid_i_slot = var_xjunstid_i;
        *var_xjunstid_i_rv_slot = var_xjunstid_i_rv;
    }

    pub(super) fn stamp_reactive_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        var_cjorgatd_i: f64,
        var_deltaphigr: f64,
        var_fcjorgat2d_i: f64,
        var_fpgat2d_i: f64,
        var_fphiggat2d_i: f64,
        var_fvbirgat2d_i: f64,
        var_kbol_over_qele: f64,
        var_one_over_one_minus_pbot: f64,
        var_one_over_one_minus_pgat: f64,
        var_one_over_one_minus_psti: f64,
        var_pgatd_i: f64,
        var_phiggatd_i: f64,
        var_phigrbot: f64,
        var_phigrgat: f64,
        var_phigrsti: f64,
        var_phitrinv: f64,
        var_tkr: f64,
        var_tkr_1: f64,
        var_vbirgatd_i: f64,
        var_vbrbotd_i: f64,
        var_vbrgatd_i: f64,
        var_vbrstid_i: f64,
        var_atatbot_slot: &mut f64,
        var_atatbot_rv_slot: &mut f64,
        var_atatgat_slot: &mut f64,
        var_atatgat_rv_slot: &mut f64,
        var_atatsti_slot: &mut f64,
        var_atatsti_rv_slot: &mut f64,
        var_auxt_slot: &mut f64,
        var_auxt_rv_slot: &mut f64,
        var_btatpartbot_slot: &mut f64,
        var_btatpartbot_rv_slot: &mut f64,
        var_btatpartgat_slot: &mut f64,
        var_btatpartgat_rv_slot: &mut f64,
        var_btatpartsti_slot: &mut f64,
        var_btatpartsti_rv_slot: &mut f64,
        var_cjobot_slot: &mut f64,
        var_cjobot_rv_slot: &mut f64,
        var_cjogat_slot: &mut f64,
        var_cjogat_rv_slot: &mut f64,
        var_cjorgat2nd_d_slot: &mut f64,
        var_cjorgat2nd_d_rv_slot: &mut f64,
        var_cjosti_slot: &mut f64,
        var_cjosti_rv_slot: &mut f64,
        var_delt_slot: &mut f64,
        var_delt_rv_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_delta_rv_slot: &mut f64,
        var_deltaebot_slot: &mut f64,
        var_deltaebot_rv_slot: &mut f64,
        var_deltaegat_slot: &mut f64,
        var_deltaegat_rv_slot: &mut f64,
        var_deltaesti_slot: &mut f64,
        var_deltaesti_rv_slot: &mut f64,
        var_deltaphigd_slot: &mut f64,
        var_deltaphigd_rv_slot: &mut f64,
        var_eg_slot: &mut f64,
        var_eg_rv_slot: &mut f64,
        var_fbbtbot_slot: &mut f64,
        var_fbbtbot_rv_slot: &mut f64,
        var_ftdbot_slot: &mut f64,
        var_ftdbot_rv_slot: &mut f64,
        var_ftdgat_slot: &mut f64,
        var_ftdgat_rv_slot: &mut f64,
        var_ftdsti_slot: &mut f64,
        var_ftdsti_rv_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard11_rv_slot: &mut f64,
        var_guard12_slot: &mut f64,
        var_guard12_rv_slot: &mut f64,
        var_idsatbot_slot: &mut f64,
        var_idsatbot_rv_slot: &mut f64,
        var_idsatgat_slot: &mut f64,
        var_idsatgat_rv_slot: &mut f64,
        var_idsatsti_slot: &mut f64,
        var_idsatsti_rv_slot: &mut f64,
        var_inv_phit_slot: &mut f64,
        var_inv_phit_rv_slot: &mut f64,
        var_inv_phita_slot: &mut f64,
        var_inv_phita_rv_slot: &mut f64,
        var_ln_rtn_slot: &mut f64,
        var_ln_rtn_rv_slot: &mut f64,
        var_one_minus_pgat2nd_d_slot: &mut f64,
        var_one_minus_pgat2nd_d_rv_slot: &mut f64,
        var_one_over_one_minus_pgat2nd_d_slot: &mut f64,
        var_one_over_one_minus_pgat2nd_d_rv_slot: &mut f64,
        var_pgat2nd_d_slot: &mut f64,
        var_pgat2nd_d_rv_slot: &mut f64,
        var_phibfac_slot: &mut f64,
        var_phibfac_rv_slot: &mut f64,
        var_phigdbot_slot: &mut f64,
        var_phigdbot_rv_slot: &mut f64,
        var_phigdgat_slot: &mut f64,
        var_phigdgat_rv_slot: &mut f64,
        var_phigdsti_slot: &mut f64,
        var_phigdsti_rv_slot: &mut f64,
        var_phiggat2nd_d_slot: &mut f64,
        var_phiggat2nd_d_rv_slot: &mut f64,
        var_phigrgat2nd_d_slot: &mut f64,
        var_phigrgat2nd_d_rv_slot: &mut f64,
        var_phit_slot: &mut f64,
        var_phit_rv_slot: &mut f64,
        var_phita_slot: &mut f64,
        var_phita_rv_slot: &mut f64,
        var_phitd_slot: &mut f64,
        var_phitd_rv_slot: &mut f64,
        var_phitdinv_slot: &mut f64,
        var_phitdinv_rv_slot: &mut f64,
        var_qpref2bot_slot: &mut f64,
        var_qpref2bot_rv_slot: &mut f64,
        var_qpref2gat_slot: &mut f64,
        var_qpref2gat_rv_slot: &mut f64,
        var_qpref2sti_slot: &mut f64,
        var_qpref2sti_rv_slot: &mut f64,
        var_qprefbot_slot: &mut f64,
        var_qprefbot_rv_slot: &mut f64,
        var_qprefgat_slot: &mut f64,
        var_qprefgat_rv_slot: &mut f64,
        var_qprefsti_slot: &mut f64,
        var_qprefsti_rv_slot: &mut f64,
        var_rta_slot: &mut f64,
        var_rta_rv_slot: &mut f64,
        var_rtn_slot: &mut f64,
        var_rtn_rv_slot: &mut f64,
        var_swgat2nd_d_slot: &mut f64,
        var_swgat2nd_d_rv_slot: &mut f64,
        var_tka_slot: &mut f64,
        var_tka_rv_slot: &mut f64,
        var_tkd_slot: &mut f64,
        var_tkd_1_slot: &mut f64,
        var_tkd_1_rv_slot: &mut f64,
        var_tkd_rv_slot: &mut f64,
        var_tkd_sq_slot: &mut f64,
        var_tkd_sq_rv_slot: &mut f64,
        var_ubibot_slot: &mut f64,
        var_ubibot_rv_slot: &mut f64,
        var_ubigat_slot: &mut f64,
        var_ubigat_rv_slot: &mut f64,
        var_ubisti_slot: &mut f64,
        var_ubisti_rv_slot: &mut f64,
        var_vbibot_slot: &mut f64,
        var_vbibot_rv_slot: &mut f64,
        var_vbigat_slot: &mut f64,
        var_vbigat_rv_slot: &mut f64,
        var_vbiinvbot_slot: &mut f64,
        var_vbiinvbot_rv_slot: &mut f64,
        var_vbiinvgat_slot: &mut f64,
        var_vbiinvgat_rv_slot: &mut f64,
        var_vbiinvsti_slot: &mut f64,
        var_vbiinvsti_rv_slot: &mut f64,
        var_vbirgat2nd_d_slot: &mut f64,
        var_vbirgat2nd_d_rv_slot: &mut f64,
        var_vbirgatinv_d_slot: &mut f64,
        var_vbirgatinv_d_rv_slot: &mut f64,
        var_vbisti_slot: &mut f64,
        var_vbisti_rv_slot: &mut f64,
        var_vbrinvbot_d_slot: &mut f64,
        var_vbrinvbot_d_rv_slot: &mut f64,
        var_vbrinvgat_d_slot: &mut f64,
        var_vbrinvgat_d_dn5_slot: &mut f64,
        var_vbrinvgat_d_dn6_slot: &mut f64,
        var_vbrinvgat_d_dn7_slot: &mut f64,
        var_vbrinvgat_d_dn8_slot: &mut f64,
        var_vbrinvgat_d_rv_slot: &mut f64,
        var_vbrinvsti_d_slot: &mut f64,
        var_vbrinvsti_d_rv_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let mut var_atatbot: f64 = *var_atatbot_slot;
        let mut var_atatbot_rv: f64 = *var_atatbot_rv_slot;
        let mut var_atatgat: f64 = *var_atatgat_slot;
        let mut var_atatgat_rv: f64 = *var_atatgat_rv_slot;
        let mut var_atatsti: f64 = *var_atatsti_slot;
        let mut var_atatsti_rv: f64 = *var_atatsti_rv_slot;
        let mut var_auxt: f64 = *var_auxt_slot;
        let mut var_auxt_rv: f64 = *var_auxt_rv_slot;
        let mut var_btatpartbot: f64 = *var_btatpartbot_slot;
        let mut var_btatpartbot_rv: f64 = *var_btatpartbot_rv_slot;
        let mut var_btatpartgat: f64 = *var_btatpartgat_slot;
        let mut var_btatpartgat_rv: f64 = *var_btatpartgat_rv_slot;
        let mut var_btatpartsti: f64 = *var_btatpartsti_slot;
        let mut var_btatpartsti_rv: f64 = *var_btatpartsti_rv_slot;
        let mut var_cjobot: f64 = *var_cjobot_slot;
        let mut var_cjobot_rv: f64 = *var_cjobot_rv_slot;
        let mut var_cjogat: f64 = *var_cjogat_slot;
        let mut var_cjogat_rv: f64 = *var_cjogat_rv_slot;
        let mut var_cjorgat2nd_d: f64 = *var_cjorgat2nd_d_slot;
        let mut var_cjorgat2nd_d_rv: f64 = *var_cjorgat2nd_d_rv_slot;
        let mut var_cjosti: f64 = *var_cjosti_slot;
        let mut var_cjosti_rv: f64 = *var_cjosti_rv_slot;
        let mut var_delt: f64 = *var_delt_slot;
        let mut var_delt_rv: f64 = *var_delt_rv_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_rv: f64 = *var_delta_rv_slot;
        let mut var_deltaebot: f64 = *var_deltaebot_slot;
        let mut var_deltaebot_rv: f64 = *var_deltaebot_rv_slot;
        let mut var_deltaegat: f64 = *var_deltaegat_slot;
        let mut var_deltaegat_rv: f64 = *var_deltaegat_rv_slot;
        let mut var_deltaesti: f64 = *var_deltaesti_slot;
        let mut var_deltaesti_rv: f64 = *var_deltaesti_rv_slot;
        let mut var_deltaphigd: f64 = *var_deltaphigd_slot;
        let mut var_deltaphigd_rv: f64 = *var_deltaphigd_rv_slot;
        let mut var_eg: f64 = *var_eg_slot;
        let mut var_eg_rv: f64 = *var_eg_rv_slot;
        let mut var_fbbtbot: f64 = *var_fbbtbot_slot;
        let mut var_fbbtbot_rv: f64 = *var_fbbtbot_rv_slot;
        let mut var_ftdbot: f64 = *var_ftdbot_slot;
        let mut var_ftdbot_rv: f64 = *var_ftdbot_rv_slot;
        let mut var_ftdgat: f64 = *var_ftdgat_slot;
        let mut var_ftdgat_rv: f64 = *var_ftdgat_rv_slot;
        let mut var_ftdsti: f64 = *var_ftdsti_slot;
        let mut var_ftdsti_rv: f64 = *var_ftdsti_rv_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard11_rv: f64 = *var_guard11_rv_slot;
        let mut var_guard12: f64 = *var_guard12_slot;
        let mut var_guard12_rv: f64 = *var_guard12_rv_slot;
        let mut var_idsatbot: f64 = *var_idsatbot_slot;
        let mut var_idsatbot_rv: f64 = *var_idsatbot_rv_slot;
        let mut var_idsatgat: f64 = *var_idsatgat_slot;
        let mut var_idsatgat_rv: f64 = *var_idsatgat_rv_slot;
        let mut var_idsatsti: f64 = *var_idsatsti_slot;
        let mut var_idsatsti_rv: f64 = *var_idsatsti_rv_slot;
        let mut var_inv_phit: f64 = *var_inv_phit_slot;
        let mut var_inv_phit_rv: f64 = *var_inv_phit_rv_slot;
        let mut var_inv_phita: f64 = *var_inv_phita_slot;
        let mut var_inv_phita_rv: f64 = *var_inv_phita_rv_slot;
        let mut var_ln_rtn: f64 = *var_ln_rtn_slot;
        let mut var_ln_rtn_rv: f64 = *var_ln_rtn_rv_slot;
        let mut var_one_minus_pgat2nd_d: f64 = *var_one_minus_pgat2nd_d_slot;
        let mut var_one_minus_pgat2nd_d_rv: f64 = *var_one_minus_pgat2nd_d_rv_slot;
        let mut var_one_over_one_minus_pgat2nd_d: f64 = *var_one_over_one_minus_pgat2nd_d_slot;
        let mut var_one_over_one_minus_pgat2nd_d_rv: f64 = *var_one_over_one_minus_pgat2nd_d_rv_slot;
        let mut var_pgat2nd_d: f64 = *var_pgat2nd_d_slot;
        let mut var_pgat2nd_d_rv: f64 = *var_pgat2nd_d_rv_slot;
        let mut var_phibfac: f64 = *var_phibfac_slot;
        let mut var_phibfac_rv: f64 = *var_phibfac_rv_slot;
        let mut var_phigdbot: f64 = *var_phigdbot_slot;
        let mut var_phigdbot_rv: f64 = *var_phigdbot_rv_slot;
        let mut var_phigdgat: f64 = *var_phigdgat_slot;
        let mut var_phigdgat_rv: f64 = *var_phigdgat_rv_slot;
        let mut var_phigdsti: f64 = *var_phigdsti_slot;
        let mut var_phigdsti_rv: f64 = *var_phigdsti_rv_slot;
        let mut var_phiggat2nd_d: f64 = *var_phiggat2nd_d_slot;
        let mut var_phiggat2nd_d_rv: f64 = *var_phiggat2nd_d_rv_slot;
        let mut var_phigrgat2nd_d: f64 = *var_phigrgat2nd_d_slot;
        let mut var_phigrgat2nd_d_rv: f64 = *var_phigrgat2nd_d_rv_slot;
        let mut var_phit: f64 = *var_phit_slot;
        let mut var_phit_rv: f64 = *var_phit_rv_slot;
        let mut var_phita: f64 = *var_phita_slot;
        let mut var_phita_rv: f64 = *var_phita_rv_slot;
        let mut var_phitd: f64 = *var_phitd_slot;
        let mut var_phitd_rv: f64 = *var_phitd_rv_slot;
        let mut var_phitdinv: f64 = *var_phitdinv_slot;
        let mut var_phitdinv_rv: f64 = *var_phitdinv_rv_slot;
        let mut var_qpref2bot: f64 = *var_qpref2bot_slot;
        let mut var_qpref2bot_rv: f64 = *var_qpref2bot_rv_slot;
        let mut var_qpref2gat: f64 = *var_qpref2gat_slot;
        let mut var_qpref2gat_rv: f64 = *var_qpref2gat_rv_slot;
        let mut var_qpref2sti: f64 = *var_qpref2sti_slot;
        let mut var_qpref2sti_rv: f64 = *var_qpref2sti_rv_slot;
        let mut var_qprefbot: f64 = *var_qprefbot_slot;
        let mut var_qprefbot_rv: f64 = *var_qprefbot_rv_slot;
        let mut var_qprefgat: f64 = *var_qprefgat_slot;
        let mut var_qprefgat_rv: f64 = *var_qprefgat_rv_slot;
        let mut var_qprefsti: f64 = *var_qprefsti_slot;
        let mut var_qprefsti_rv: f64 = *var_qprefsti_rv_slot;
        let mut var_rta: f64 = *var_rta_slot;
        let mut var_rta_rv: f64 = *var_rta_rv_slot;
        let mut var_rtn: f64 = *var_rtn_slot;
        let mut var_rtn_rv: f64 = *var_rtn_rv_slot;
        let mut var_swgat2nd_d: f64 = *var_swgat2nd_d_slot;
        let mut var_swgat2nd_d_rv: f64 = *var_swgat2nd_d_rv_slot;
        let mut var_tka: f64 = *var_tka_slot;
        let mut var_tka_rv: f64 = *var_tka_rv_slot;
        let mut var_tkd: f64 = *var_tkd_slot;
        let mut var_tkd_1: f64 = *var_tkd_1_slot;
        let mut var_tkd_1_rv: f64 = *var_tkd_1_rv_slot;
        let mut var_tkd_rv: f64 = *var_tkd_rv_slot;
        let mut var_tkd_sq: f64 = *var_tkd_sq_slot;
        let mut var_tkd_sq_rv: f64 = *var_tkd_sq_rv_slot;
        let mut var_ubibot: f64 = *var_ubibot_slot;
        let mut var_ubibot_rv: f64 = *var_ubibot_rv_slot;
        let mut var_ubigat: f64 = *var_ubigat_slot;
        let mut var_ubigat_rv: f64 = *var_ubigat_rv_slot;
        let mut var_ubisti: f64 = *var_ubisti_slot;
        let mut var_ubisti_rv: f64 = *var_ubisti_rv_slot;
        let mut var_vbibot: f64 = *var_vbibot_slot;
        let mut var_vbibot_rv: f64 = *var_vbibot_rv_slot;
        let mut var_vbigat: f64 = *var_vbigat_slot;
        let mut var_vbigat_rv: f64 = *var_vbigat_rv_slot;
        let mut var_vbiinvbot: f64 = *var_vbiinvbot_slot;
        let mut var_vbiinvbot_rv: f64 = *var_vbiinvbot_rv_slot;
        let mut var_vbiinvgat: f64 = *var_vbiinvgat_slot;
        let mut var_vbiinvgat_rv: f64 = *var_vbiinvgat_rv_slot;
        let mut var_vbiinvsti: f64 = *var_vbiinvsti_slot;
        let mut var_vbiinvsti_rv: f64 = *var_vbiinvsti_rv_slot;
        let mut var_vbirgat2nd_d: f64 = *var_vbirgat2nd_d_slot;
        let mut var_vbirgat2nd_d_rv: f64 = *var_vbirgat2nd_d_rv_slot;
        let mut var_vbirgatinv_d: f64 = *var_vbirgatinv_d_slot;
        let mut var_vbirgatinv_d_rv: f64 = *var_vbirgatinv_d_rv_slot;
        let mut var_vbisti: f64 = *var_vbisti_slot;
        let mut var_vbisti_rv: f64 = *var_vbisti_rv_slot;
        let mut var_vbrinvbot_d: f64 = *var_vbrinvbot_d_slot;
        let mut var_vbrinvbot_d_rv: f64 = *var_vbrinvbot_d_rv_slot;
        let mut var_vbrinvgat_d: f64 = *var_vbrinvgat_d_slot;
        let mut var_vbrinvgat_d_dn5: f64 = *var_vbrinvgat_d_dn5_slot;
        let mut var_vbrinvgat_d_dn6: f64 = *var_vbrinvgat_d_dn6_slot;
        let mut var_vbrinvgat_d_dn7: f64 = *var_vbrinvgat_d_dn7_slot;
        let mut var_vbrinvgat_d_dn8: f64 = *var_vbrinvgat_d_dn8_slot;
        let mut var_vbrinvgat_d_rv: f64 = *var_vbrinvgat_d_rv_slot;
        let mut var_vbrinvsti_d: f64 = *var_vbrinvsti_d_slot;
        let mut var_vbrinvsti_d_rv: f64 = *var_vbrinvsti_d_rv_slot;

        let assign1940_e2506: f64 = (1.0 / var_vbirgatd_i);
        var_vbirgatinv_d = assign1940_e2506;
        var_vbirgatinv_d_rv = 0.0;

        let assign1980_e2530: f64 = (1.0 / var_vbrbotd_i);
        var_vbrinvbot_d = assign1980_e2530;
        var_vbrinvbot_d_rv = 0.0;

        let assign1990_e2533: f64 = (1.0 / var_vbrstid_i);
        var_vbrinvsti_d = assign1990_e2533;
        var_vbrinvsti_d_rv = 0.0;

        let assign2000_e2536: f64 = (1.0 / var_vbrgatd_i);
        var_vbrinvgat_d = assign2000_e2536;
        var_vbrinvgat_d_dn5 = 0.0;
        var_vbrinvgat_d_dn6 = 0.0;
        var_vbrinvgat_d_dn7 = 0.0;
        var_vbrinvgat_d_dn8 = 0.0;
        var_vbrinvgat_d_rv = 0.0;

        let assign2040_e2593: f64 = if ((((var_fcjorgat2d_i != 1.0) || (var_fvbirgat2d_i != 1.0)) || (var_fpgat2d_i != 1.0)) || (var_fphiggat2d_i != 1.0)) { 1.0 } else { 0.0 };
        var_guard11 = assign2040_e2593;
        var_guard11_rv = 0.0;

        let (assign2050_e2597,) = {
    if (var_guard11 != 0.0) {
        (1.0,)
    } else {
        (var_swgat2nd_d,)
    }
};
        var_swgat2nd_d = assign2050_e2597;
        var_swgat2nd_d_rv = 0.0;

        let (assign2060_e2602,) = {
    if (var_guard11 == 0.0) {
        (0.0,)
    } else {
        (var_swgat2nd_d,)
    }
};
        var_swgat2nd_d = assign2060_e2602;
        var_swgat2nd_d_rv = 0.0;

        let assign2070_e2605: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard12 = assign2070_e2605;
        var_guard12_rv = 0.0;

        let (assign2080_e2618,) = {
    if (var_guard12 != 0.0) {
        let assign2080_e2609: f64 = (var_cjorgatd_i * var_fcjorgat2d_i);
        let (assign2080_e2616,) = {
            if (assign2080_e2609 > 1e-18) {
                let assign2080_e2614: f64 = (var_cjorgatd_i * var_fcjorgat2d_i);
                (assign2080_e2614,)
            } else {
                (1e-18,)
            }
        };
        (assign2080_e2616,)
    } else {
        (var_cjorgat2nd_d,)
    }
};
        var_cjorgat2nd_d = assign2080_e2618;
        var_cjorgat2nd_d_rv = 0.0;

        let (assign2090_e2631,) = {
    if (var_guard12 != 0.0) {
        let assign2090_e2622: f64 = (var_vbirgatd_i * var_fvbirgat2d_i);
        let (assign2090_e2629,) = {
            if (assign2090_e2622 > 0.05) {
                let assign2090_e2627: f64 = (var_vbirgatd_i * var_fvbirgat2d_i);
                (assign2090_e2627,)
            } else {
                (0.05,)
            }
        };
        (assign2090_e2629,)
    } else {
        (var_vbirgat2nd_d,)
    }
};
        var_vbirgat2nd_d = assign2090_e2631;
        var_vbirgat2nd_d_rv = 0.0;

        let (assign2100_e2658,) = {
    if (var_guard12 != 0.0) {
        let assign2100_e2635: f64 = (var_pgatd_i * var_fpgat2d_i);
        let (assign2100_e2642,) = {
            if (assign2100_e2635 > 0.05) {
                let assign2100_e2640: f64 = (var_pgatd_i * var_fpgat2d_i);
                (assign2100_e2640,)
            } else {
                (0.05,)
            }
        };
        let (assign2100_e2656,) = {
            if (assign2100_e2642 < 0.95) {
                let assign2100_e2647: f64 = (var_pgatd_i * var_fpgat2d_i);
                let (assign2100_e2654,) = {
                    if (assign2100_e2647 > 0.05) {
                        let assign2100_e2652: f64 = (var_pgatd_i * var_fpgat2d_i);
                        (assign2100_e2652,)
                    } else {
                        (0.05,)
                    }
                };
                (assign2100_e2654,)
            } else {
                (0.95,)
            }
        };
        (assign2100_e2656,)
    } else {
        (var_pgat2nd_d,)
    }
};
        var_pgat2nd_d = assign2100_e2658;
        var_pgat2nd_d_rv = 0.0;

        let (assign2110_e2664,) = {
    if (var_guard12 != 0.0) {
        let assign2110_e2662: f64 = (var_phiggatd_i * var_fphiggat2d_i);
        (assign2110_e2662,)
    } else {
        (var_phiggat2nd_d,)
    }
};
        var_phiggat2nd_d = assign2110_e2664;
        var_phiggat2nd_d_rv = 0.0;

        let (assign2120_e2670,) = {
    if (var_guard12 != 0.0) {
        let assign2120_e2668: f64 = (var_phiggat2nd_d + var_deltaphigr);
        (assign2120_e2668,)
    } else {
        (var_phigrgat2nd_d,)
    }
};
        var_phigrgat2nd_d = assign2120_e2670;
        var_phigrgat2nd_d_rv = 0.0;

        let (assign2130_e2676,) = {
    if (var_guard12 != 0.0) {
        let assign2130_e2674: f64 = (1.0 - var_pgat2nd_d);
        (assign2130_e2674,)
    } else {
        (var_one_minus_pgat2nd_d,)
    }
};
        var_one_minus_pgat2nd_d = assign2130_e2676;
        var_one_minus_pgat2nd_d_rv = 0.0;

        let (assign2140_e2682,) = {
    if (var_guard12 != 0.0) {
        let assign2140_e2680: f64 = (1.0 / var_one_minus_pgat2nd_d);
        (assign2140_e2680,)
    } else {
        (var_one_over_one_minus_pgat2nd_d,)
    }
};
        var_one_over_one_minus_pgat2nd_d = assign2140_e2682;
        var_one_over_one_minus_pgat2nd_d_rv = 0.0;

        let assign2190_e2704: f64 = ctx_temp;
        let assign2190_e2706: f64 = (assign2190_e2704 + p.p56);
        let assign2190_e2708: f64 = (assign2190_e2706 + p.p35);
        var_tka = assign2190_e2708;
        var_tka_rv = 0.0;

        let assign2200_e2711: f64 = (var_tka / var_tkr);
        var_rta = assign2200_e2711;
        var_rta_rv = 0.0;

        let assign2210_e2714: f64 = (var_tka - var_tkr);
        var_delta = assign2210_e2714;
        var_delta_rv = 0.0;

        let assign2220_e2717: f64 = (var_tka * 1.3806505e-23);
        let assign2220_e2719: f64 = (assign2220_e2717 / 1.6021918e-19);
        var_phita = assign2220_e2719;
        var_phita_rv = 0.0;

        let assign2230_e2722: f64 = (1.0 / var_phita);
        var_inv_phita = assign2230_e2722;
        var_inv_phita_rv = 0.0;

        var_tkd = var_tka;
        var_tkd_rv = 0.0;

        let assign2250_e2726: f64 = (var_tkd * var_tkd);
        var_tkd_sq = assign2250_e2726;
        var_tkd_sq_rv = 0.0;

        let assign2260_e2729: f64 = (var_tkd - var_tkr);
        var_delt = assign2260_e2729;
        var_delt_rv = 0.0;

        let assign2270_e2732: f64 = (var_tkr / var_tkd);
        var_rtn = assign2270_e2732;
        var_rtn_rv = 0.0;

        let assign2280_e2734: f64 = (var_rtn).ln();
        var_ln_rtn = assign2280_e2734;
        var_ln_rtn_rv = 0.0;

        let assign2290_e2737: f64 = (var_tkd * 1.3806505e-23);
        let assign2290_e2739: f64 = (assign2290_e2737 / 1.6021918e-19);
        var_phit = assign2290_e2739;
        var_phit_rv = 0.0;

        let assign2300_e2742: f64 = (1.0 / var_phit);
        var_inv_phit = assign2300_e2742;
        var_inv_phit_rv = 0.0;

        let assign2310_e2746: f64 = (9.025e-5 * var_tkd);
        let assign2310_e2747: f64 = (1.179 - assign2310_e2746);
        let assign2310_e2750: f64 = (3.05e-7 * var_tkd_sq);
        let assign2310_e2751: f64 = (assign2310_e2747 - assign2310_e2750);
        var_eg = assign2310_e2751;
        var_eg_rv = 0.0;

        let assign2320_e2755: f64 = (0.00045 * var_tkd);
        let assign2320_e2756: f64 = (1.045 + assign2320_e2755);
        let assign2320_e2760: f64 = (0.0014 * var_tkd);
        let assign2320_e2761: f64 = (0.523 + assign2320_e2760);
        let assign2320_e2764: f64 = (1.48e-6 * var_tkd_sq);
        let assign2320_e2765: f64 = (assign2320_e2761 - assign2320_e2764);
        let assign2320_e2766: f64 = (assign2320_e2756 * assign2320_e2765);
        let assign2320_e2768: f64 = (assign2320_e2766 * var_tkd_sq);
        let assign2320_e2770: f64 = (assign2320_e2768 / 90000.0);
        var_phibfac = assign2320_e2770;
        var_phibfac_rv = 0.0;

        let (assign2330_e2776,) = {
    if (var_phibfac > 0.001) {
        (var_phibfac,)
    } else {
        (0.001,)
    }
};
        var_phibfac = assign2330_e2776;
        var_phibfac_rv = 0.0;

        let assign2350_e2782: f64 = ctx_temp;
        let assign2350_e2784: f64 = (assign2350_e2782 + p.p56);
        let assign2350_e2786: f64 = (assign2350_e2784 + p.p35);
        let assign2350_e2789: f64 = (-250.0);
        let assign2350_e2790: f64 = (273.15 + assign2350_e2789);
        let assign2350_e2791: f64 = (assign2350_e2786).max(assign2350_e2790);
        var_tkd_1 = assign2350_e2791;
        var_tkd_1_rv = 0.0;

        let assign2360_e2794: f64 = (var_tkd_1 / var_tkr_1);
        var_auxt = assign2360_e2794;
        var_auxt_rv = 0.0;

        let assign2370_e2797: f64 = (var_kbol_over_qele * var_tkd_1);
        var_phitd = assign2370_e2797;
        var_phitd_rv = 0.0;

        let assign2380_e2800: f64 = (1.0 / var_phitd);
        var_phitdinv = assign2380_e2800;
        var_phitdinv_rv = 0.0;

        let assign2390_e2803: f64 = (0.000702 * var_tkd_1);
        let assign2390_e2805: f64 = (assign2390_e2803 * var_tkd_1);
        let assign2390_e2806: f64 = (-assign2390_e2805);
        let assign2390_e2809: f64 = (1108.0 + var_tkd_1);
        let assign2390_e2810: f64 = (assign2390_e2806 / assign2390_e2809);
        var_deltaphigd = assign2390_e2810;
        var_deltaphigd_rv = 0.0;

        let assign2400_e2813: f64 = (p.p834 + var_deltaphigd);
        var_phigdbot = assign2400_e2813;
        var_phigdbot_rv = 0.0;

        let assign2410_e2816: f64 = (p.p835 + var_deltaphigd);
        var_phigdsti = assign2410_e2816;
        var_phigdsti_rv = 0.0;

        let assign2420_e2819: f64 = (p.p836 + var_deltaphigd);
        var_phigdgat = assign2420_e2819;
        var_phigdgat_rv = 0.0;

        let assign2430_e2822: f64 = (var_auxt).powf(1.5);
        let assign2430_e2826: f64 = (var_phigrbot * var_phitrinv);
        let assign2430_e2829: f64 = (var_phigdbot * var_phitdinv);
        let assign2430_e2830: f64 = (assign2430_e2826 - assign2430_e2829);
        let assign2430_e2831: f64 = (0.5 * assign2430_e2830);
        let assign2430_e2832: f64 = (assign2430_e2831).exp();
        let assign2430_e2833: f64 = (assign2430_e2822 * assign2430_e2832);
        var_ftdbot = assign2430_e2833;
        var_ftdbot_rv = 0.0;

        let assign2440_e2836: f64 = (var_auxt).powf(1.5);
        let assign2440_e2840: f64 = (var_phigrsti * var_phitrinv);
        let assign2440_e2843: f64 = (var_phigdsti * var_phitdinv);
        let assign2440_e2844: f64 = (assign2440_e2840 - assign2440_e2843);
        let assign2440_e2845: f64 = (0.5 * assign2440_e2844);
        let assign2440_e2846: f64 = (assign2440_e2845).exp();
        let assign2440_e2847: f64 = (assign2440_e2836 * assign2440_e2846);
        var_ftdsti = assign2440_e2847;
        var_ftdsti_rv = 0.0;

        let assign2450_e2850: f64 = (var_auxt).powf(1.5);
        let assign2450_e2854: f64 = (var_phigrgat * var_phitrinv);
        let assign2450_e2857: f64 = (var_phigdgat * var_phitdinv);
        let assign2450_e2858: f64 = (assign2450_e2854 - assign2450_e2857);
        let assign2450_e2859: f64 = (0.5 * assign2450_e2858);
        let assign2450_e2860: f64 = (assign2450_e2859).exp();
        let assign2450_e2861: f64 = (assign2450_e2850 * assign2450_e2860);
        var_ftdgat = assign2450_e2861;
        var_ftdgat_rv = 0.0;

        let assign2460_e2864: f64 = (p.p837 * var_ftdbot);
        let assign2460_e2866: f64 = (assign2460_e2864 * var_ftdbot);
        var_idsatbot = assign2460_e2866;
        var_idsatbot_rv = 0.0;

        let assign2470_e2869: f64 = (p.p838 * var_ftdsti);
        let assign2470_e2871: f64 = (assign2470_e2869 * var_ftdsti);
        var_idsatsti = assign2470_e2871;
        var_idsatsti_rv = 0.0;

        let assign2480_e2874: f64 = (p.p839 * var_ftdgat);
        let assign2480_e2876: f64 = (assign2480_e2874 * var_ftdgat);
        var_idsatgat = assign2480_e2876;
        var_idsatgat_rv = 0.0;

        let assign2490_e2879: f64 = (p.p828 * var_auxt);
        let assign2490_e2882: f64 = (2.0 * var_phitd);
        let assign2490_e2884: f64 = (var_ftdbot).ln();
        let assign2490_e2885: f64 = (assign2490_e2882 * assign2490_e2884);
        let assign2490_e2886: f64 = (assign2490_e2879 - assign2490_e2885);
        var_ubibot = assign2490_e2886;
        var_ubibot_rv = 0.0;

        let assign2500_e2889: f64 = (p.p829 * var_auxt);
        let assign2500_e2892: f64 = (2.0 * var_phitd);
        let assign2500_e2894: f64 = (var_ftdsti).ln();
        let assign2500_e2895: f64 = (assign2500_e2892 * assign2500_e2894);
        let assign2500_e2896: f64 = (assign2500_e2889 - assign2500_e2895);
        var_ubisti = assign2500_e2896;
        var_ubisti_rv = 0.0;

        let assign2510_e2899: f64 = (p.p830 * var_auxt);
        let assign2510_e2902: f64 = (2.0 * var_phitd);
        let assign2510_e2904: f64 = (var_ftdgat).ln();
        let assign2510_e2905: f64 = (assign2510_e2902 * assign2510_e2904);
        let assign2510_e2906: f64 = (assign2510_e2899 - assign2510_e2905);
        var_ubigat = assign2510_e2906;
        var_ubigat_rv = 0.0;

        let assign2520_e2912: f64 = (0.05 - var_ubibot);
        let assign2520_e2914: f64 = (assign2520_e2912 * var_phitdinv);
        let assign2520_e2915: f64 = (assign2520_e2914).exp();
        let assign2520_e2916: f64 = (1.0 + assign2520_e2915);
        let assign2520_e2917: f64 = (assign2520_e2916).ln();
        let assign2520_e2918: f64 = (var_phitd * assign2520_e2917);
        let assign2520_e2919: f64 = (var_ubibot + assign2520_e2918);
        var_vbibot = assign2520_e2919;
        var_vbibot_rv = 0.0;

        let assign2530_e2925: f64 = (0.05 - var_ubisti);
        let assign2530_e2927: f64 = (assign2530_e2925 * var_phitdinv);
        let assign2530_e2928: f64 = (assign2530_e2927).exp();
        let assign2530_e2929: f64 = (1.0 + assign2530_e2928);
        let assign2530_e2930: f64 = (assign2530_e2929).ln();
        let assign2530_e2931: f64 = (var_phitd * assign2530_e2930);
        let assign2530_e2932: f64 = (var_ubisti + assign2530_e2931);
        var_vbisti = assign2530_e2932;
        var_vbisti_rv = 0.0;

        let assign2540_e2938: f64 = (0.05 - var_ubigat);
        let assign2540_e2940: f64 = (assign2540_e2938 * var_phitdinv);
        let assign2540_e2941: f64 = (assign2540_e2940).exp();
        let assign2540_e2942: f64 = (1.0 + assign2540_e2941);
        let assign2540_e2943: f64 = (assign2540_e2942).ln();
        let assign2540_e2944: f64 = (var_phitd * assign2540_e2943);
        let assign2540_e2945: f64 = (var_ubigat + assign2540_e2944);
        var_vbigat = assign2540_e2945;
        var_vbigat_rv = 0.0;

        let assign2550_e2948: f64 = (1.0 / var_vbibot);
        var_vbiinvbot = assign2550_e2948;
        var_vbiinvbot_rv = 0.0;

        let assign2560_e2951: f64 = (1.0 / var_vbisti);
        var_vbiinvsti = assign2560_e2951;
        var_vbiinvsti_rv = 0.0;

        let assign2570_e2954: f64 = (1.0 / var_vbigat);
        var_vbiinvgat = assign2570_e2954;
        var_vbiinvgat_rv = 0.0;

        let assign2580_e2958: f64 = (p.p828 * var_vbiinvbot);
        let assign2580_e2960: f64 = (assign2580_e2958).powf(p.p831);
        let assign2580_e2961: f64 = (p.p825 * assign2580_e2960);
        var_cjobot = assign2580_e2961;
        var_cjobot_rv = 0.0;

        let assign2590_e2965: f64 = (p.p829 * var_vbiinvsti);
        let assign2590_e2967: f64 = (assign2590_e2965).powf(p.p832);
        let assign2590_e2968: f64 = (p.p826 * assign2590_e2967);
        var_cjosti = assign2590_e2968;
        var_cjosti_rv = 0.0;

        let assign2600_e2972: f64 = (p.p830 * var_vbiinvgat);
        let assign2600_e2974: f64 = (assign2600_e2972).powf(p.p833);
        let assign2600_e2975: f64 = (p.p827 * assign2600_e2974);
        var_cjogat = assign2600_e2975;
        var_cjogat_rv = 0.0;

        let assign2610_e2978: f64 = (var_cjobot * var_vbibot);
        let assign2610_e2980: f64 = (assign2610_e2978 * var_one_over_one_minus_pbot);
        var_qprefbot = assign2610_e2980;
        var_qprefbot_rv = 0.0;

        let assign2620_e2983: f64 = (var_cjosti * var_vbisti);
        let assign2620_e2985: f64 = (assign2620_e2983 * var_one_over_one_minus_psti);
        var_qprefsti = assign2620_e2985;
        var_qprefsti_rv = 0.0;

        let assign2630_e2988: f64 = (var_cjogat * var_vbigat);
        let assign2630_e2990: f64 = (assign2630_e2988 * var_one_over_one_minus_pgat);
        var_qprefgat = assign2630_e2990;
        var_qprefgat_rv = 0.0;

        let assign2640_e2993: f64 = (2.0 * var_cjobot);
        var_qpref2bot = assign2640_e2993;
        var_qpref2bot_rv = 0.0;

        let assign2650_e2996: f64 = (2.0 * var_cjosti);
        var_qpref2sti = assign2650_e2996;
        var_qpref2sti_rv = 0.0;

        let assign2660_e2999: f64 = (2.0 * var_cjogat);
        var_qpref2gat = assign2660_e2999;
        var_qpref2gat_rv = 0.0;

        let assign2670_e3002: f64 = (0.5 * var_phigdbot);
        let assign2670_e3004: f64 = (assign2670_e3002).max(var_phitd);
        var_deltaebot = assign2670_e3004;
        var_deltaebot_rv = 0.0;

        let assign2680_e3007: f64 = (0.5 * var_phigdsti);
        let assign2680_e3009: f64 = (assign2680_e3007).max(var_phitd);
        var_deltaesti = assign2680_e3009;
        var_deltaesti_rv = 0.0;

        let assign2690_e3012: f64 = (0.5 * var_phigdgat);
        let assign2690_e3014: f64 = (assign2690_e3012).max(var_phitd);
        var_deltaegat = assign2690_e3014;
        var_deltaegat_rv = 0.0;

        let assign2700_e3017: f64 = (var_deltaebot * var_phitdinv);
        var_atatbot = assign2700_e3017;
        var_atatbot_rv = 0.0;

        let assign2710_e3020: f64 = (var_deltaesti * var_phitdinv);
        var_atatsti = assign2710_e3020;
        var_atatsti_rv = 0.0;

        let assign2720_e3023: f64 = (var_deltaegat * var_phitdinv);
        var_atatgat = assign2720_e3023;
        var_atatgat_rv = 0.0;

        let assign2730_e3026: f64 = (32.0 * p.p848);
        let assign2730_e3028: f64 = (assign2730_e3026 * 9.1093826e-31);
        let assign2730_e3030: f64 = (assign2730_e3028 * 1.6021918e-19);
        let assign2730_e3033: f64 = (var_deltaebot * var_deltaebot);
        let assign2730_e3035: f64 = (assign2730_e3033 * var_deltaebot);
        let assign2730_e3036: f64 = (assign2730_e3030 * assign2730_e3035);
        let assign2730_e3037: f64 = (assign2730_e3036).sqrt();
        let assign2730_e3040: f64 = (3.0 * 1.05457168e-34);
        let assign2730_e3041: f64 = (assign2730_e3037 / assign2730_e3040);
        var_btatpartbot = assign2730_e3041;
        var_btatpartbot_rv = 0.0;

        let assign2740_e3044: f64 = (32.0 * p.p849);
        let assign2740_e3046: f64 = (assign2740_e3044 * 9.1093826e-31);
        let assign2740_e3048: f64 = (assign2740_e3046 * 1.6021918e-19);
        let assign2740_e3051: f64 = (var_deltaesti * var_deltaesti);
        let assign2740_e3053: f64 = (assign2740_e3051 * var_deltaesti);
        let assign2740_e3054: f64 = (assign2740_e3048 * assign2740_e3053);
        let assign2740_e3055: f64 = (assign2740_e3054).sqrt();
        let assign2740_e3058: f64 = (3.0 * 1.05457168e-34);
        let assign2740_e3059: f64 = (assign2740_e3055 / assign2740_e3058);
        var_btatpartsti = assign2740_e3059;
        var_btatpartsti_rv = 0.0;

        let assign2750_e3062: f64 = (32.0 * p.p850);
        let assign2750_e3064: f64 = (assign2750_e3062 * 9.1093826e-31);
        let assign2750_e3066: f64 = (assign2750_e3064 * 1.6021918e-19);
        let assign2750_e3069: f64 = (var_deltaegat * var_deltaegat);
        let assign2750_e3071: f64 = (assign2750_e3069 * var_deltaegat);
        let assign2750_e3072: f64 = (assign2750_e3066 * assign2750_e3071);
        let assign2750_e3073: f64 = (assign2750_e3072).sqrt();
        let assign2750_e3076: f64 = (3.0 * 1.05457168e-34);
        let assign2750_e3077: f64 = (assign2750_e3073 / assign2750_e3076);
        var_btatpartgat = assign2750_e3077;
        var_btatpartgat_rv = 0.0;

        let assign2760_e3083: f64 = (var_tkd_1 - var_tkr_1);
        let assign2760_e3084: f64 = (p.p857 * assign2760_e3083);
        let assign2760_e3085: f64 = (1.0 + assign2760_e3084);
        let assign2760_e3086: f64 = (p.p854 * assign2760_e3085);
        var_fbbtbot = assign2760_e3086;
        var_fbbtbot_rv = 0.0;

        *var_atatbot_slot = var_atatbot;
        *var_atatbot_rv_slot = var_atatbot_rv;
        *var_atatgat_slot = var_atatgat;
        *var_atatgat_rv_slot = var_atatgat_rv;
        *var_atatsti_slot = var_atatsti;
        *var_atatsti_rv_slot = var_atatsti_rv;
        *var_auxt_slot = var_auxt;
        *var_auxt_rv_slot = var_auxt_rv;
        *var_btatpartbot_slot = var_btatpartbot;
        *var_btatpartbot_rv_slot = var_btatpartbot_rv;
        *var_btatpartgat_slot = var_btatpartgat;
        *var_btatpartgat_rv_slot = var_btatpartgat_rv;
        *var_btatpartsti_slot = var_btatpartsti;
        *var_btatpartsti_rv_slot = var_btatpartsti_rv;
        *var_cjobot_slot = var_cjobot;
        *var_cjobot_rv_slot = var_cjobot_rv;
        *var_cjogat_slot = var_cjogat;
        *var_cjogat_rv_slot = var_cjogat_rv;
        *var_cjorgat2nd_d_slot = var_cjorgat2nd_d;
        *var_cjorgat2nd_d_rv_slot = var_cjorgat2nd_d_rv;
        *var_cjosti_slot = var_cjosti;
        *var_cjosti_rv_slot = var_cjosti_rv;
        *var_delt_slot = var_delt;
        *var_delt_rv_slot = var_delt_rv;
        *var_delta_slot = var_delta;
        *var_delta_rv_slot = var_delta_rv;
        *var_deltaebot_slot = var_deltaebot;
        *var_deltaebot_rv_slot = var_deltaebot_rv;
        *var_deltaegat_slot = var_deltaegat;
        *var_deltaegat_rv_slot = var_deltaegat_rv;
        *var_deltaesti_slot = var_deltaesti;
        *var_deltaesti_rv_slot = var_deltaesti_rv;
        *var_deltaphigd_slot = var_deltaphigd;
        *var_deltaphigd_rv_slot = var_deltaphigd_rv;
        *var_eg_slot = var_eg;
        *var_eg_rv_slot = var_eg_rv;
        *var_fbbtbot_slot = var_fbbtbot;
        *var_fbbtbot_rv_slot = var_fbbtbot_rv;
        *var_ftdbot_slot = var_ftdbot;
        *var_ftdbot_rv_slot = var_ftdbot_rv;
        *var_ftdgat_slot = var_ftdgat;
        *var_ftdgat_rv_slot = var_ftdgat_rv;
        *var_ftdsti_slot = var_ftdsti;
        *var_ftdsti_rv_slot = var_ftdsti_rv;
        *var_guard11_slot = var_guard11;
        *var_guard11_rv_slot = var_guard11_rv;
        *var_guard12_slot = var_guard12;
        *var_guard12_rv_slot = var_guard12_rv;
        *var_idsatbot_slot = var_idsatbot;
        *var_idsatbot_rv_slot = var_idsatbot_rv;
        *var_idsatgat_slot = var_idsatgat;
        *var_idsatgat_rv_slot = var_idsatgat_rv;
        *var_idsatsti_slot = var_idsatsti;
        *var_idsatsti_rv_slot = var_idsatsti_rv;
        *var_inv_phit_slot = var_inv_phit;
        *var_inv_phit_rv_slot = var_inv_phit_rv;
        *var_inv_phita_slot = var_inv_phita;
        *var_inv_phita_rv_slot = var_inv_phita_rv;
        *var_ln_rtn_slot = var_ln_rtn;
        *var_ln_rtn_rv_slot = var_ln_rtn_rv;
        *var_one_minus_pgat2nd_d_slot = var_one_minus_pgat2nd_d;
        *var_one_minus_pgat2nd_d_rv_slot = var_one_minus_pgat2nd_d_rv;
        *var_one_over_one_minus_pgat2nd_d_slot = var_one_over_one_minus_pgat2nd_d;
        *var_one_over_one_minus_pgat2nd_d_rv_slot = var_one_over_one_minus_pgat2nd_d_rv;
        *var_pgat2nd_d_slot = var_pgat2nd_d;
        *var_pgat2nd_d_rv_slot = var_pgat2nd_d_rv;
        *var_phibfac_slot = var_phibfac;
        *var_phibfac_rv_slot = var_phibfac_rv;
        *var_phigdbot_slot = var_phigdbot;
        *var_phigdbot_rv_slot = var_phigdbot_rv;
        *var_phigdgat_slot = var_phigdgat;
        *var_phigdgat_rv_slot = var_phigdgat_rv;
        *var_phigdsti_slot = var_phigdsti;
        *var_phigdsti_rv_slot = var_phigdsti_rv;
        *var_phiggat2nd_d_slot = var_phiggat2nd_d;
        *var_phiggat2nd_d_rv_slot = var_phiggat2nd_d_rv;
        *var_phigrgat2nd_d_slot = var_phigrgat2nd_d;
        *var_phigrgat2nd_d_rv_slot = var_phigrgat2nd_d_rv;
        *var_phit_slot = var_phit;
        *var_phit_rv_slot = var_phit_rv;
        *var_phita_slot = var_phita;
        *var_phita_rv_slot = var_phita_rv;
        *var_phitd_slot = var_phitd;
        *var_phitd_rv_slot = var_phitd_rv;
        *var_phitdinv_slot = var_phitdinv;
        *var_phitdinv_rv_slot = var_phitdinv_rv;
        *var_qpref2bot_slot = var_qpref2bot;
        *var_qpref2bot_rv_slot = var_qpref2bot_rv;
        *var_qpref2gat_slot = var_qpref2gat;
        *var_qpref2gat_rv_slot = var_qpref2gat_rv;
        *var_qpref2sti_slot = var_qpref2sti;
        *var_qpref2sti_rv_slot = var_qpref2sti_rv;
        *var_qprefbot_slot = var_qprefbot;
        *var_qprefbot_rv_slot = var_qprefbot_rv;
        *var_qprefgat_slot = var_qprefgat;
        *var_qprefgat_rv_slot = var_qprefgat_rv;
        *var_qprefsti_slot = var_qprefsti;
        *var_qprefsti_rv_slot = var_qprefsti_rv;
        *var_rta_slot = var_rta;
        *var_rta_rv_slot = var_rta_rv;
        *var_rtn_slot = var_rtn;
        *var_rtn_rv_slot = var_rtn_rv;
        *var_swgat2nd_d_slot = var_swgat2nd_d;
        *var_swgat2nd_d_rv_slot = var_swgat2nd_d_rv;
        *var_tka_slot = var_tka;
        *var_tka_rv_slot = var_tka_rv;
        *var_tkd_slot = var_tkd;
        *var_tkd_1_slot = var_tkd_1;
        *var_tkd_1_rv_slot = var_tkd_1_rv;
        *var_tkd_rv_slot = var_tkd_rv;
        *var_tkd_sq_slot = var_tkd_sq;
        *var_tkd_sq_rv_slot = var_tkd_sq_rv;
        *var_ubibot_slot = var_ubibot;
        *var_ubibot_rv_slot = var_ubibot_rv;
        *var_ubigat_slot = var_ubigat;
        *var_ubigat_rv_slot = var_ubigat_rv;
        *var_ubisti_slot = var_ubisti;
        *var_ubisti_rv_slot = var_ubisti_rv;
        *var_vbibot_slot = var_vbibot;
        *var_vbibot_rv_slot = var_vbibot_rv;
        *var_vbigat_slot = var_vbigat;
        *var_vbigat_rv_slot = var_vbigat_rv;
        *var_vbiinvbot_slot = var_vbiinvbot;
        *var_vbiinvbot_rv_slot = var_vbiinvbot_rv;
        *var_vbiinvgat_slot = var_vbiinvgat;
        *var_vbiinvgat_rv_slot = var_vbiinvgat_rv;
        *var_vbiinvsti_slot = var_vbiinvsti;
        *var_vbiinvsti_rv_slot = var_vbiinvsti_rv;
        *var_vbirgat2nd_d_slot = var_vbirgat2nd_d;
        *var_vbirgat2nd_d_rv_slot = var_vbirgat2nd_d_rv;
        *var_vbirgatinv_d_slot = var_vbirgatinv_d;
        *var_vbirgatinv_d_rv_slot = var_vbirgatinv_d_rv;
        *var_vbisti_slot = var_vbisti;
        *var_vbisti_rv_slot = var_vbisti_rv;
        *var_vbrinvbot_d_slot = var_vbrinvbot_d;
        *var_vbrinvbot_d_rv_slot = var_vbrinvbot_d_rv;
        *var_vbrinvgat_d_slot = var_vbrinvgat_d;
        *var_vbrinvgat_d_dn5_slot = var_vbrinvgat_d_dn5;
        *var_vbrinvgat_d_dn6_slot = var_vbrinvgat_d_dn6;
        *var_vbrinvgat_d_dn7_slot = var_vbrinvgat_d_dn7;
        *var_vbrinvgat_d_dn8_slot = var_vbrinvgat_d_dn8;
        *var_vbrinvgat_d_rv_slot = var_vbrinvgat_d_rv;
        *var_vbrinvsti_d_slot = var_vbrinvsti_d;
        *var_vbrinvsti_d_rv_slot = var_vbrinvsti_d_rv;
    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        var_auxt: f64,
        var_cjorbotd_i: f64,
        var_cjorgat2nd: f64,
        var_cjorgatd_i: f64,
        var_cjorstid_i: f64,
        var_deltaphigd: f64,
        var_fbbtrbotd_i: f64,
        var_fbbtrgatd_i: f64,
        var_fbbtrstid_i: f64,
        var_idsatrbotd_i: f64,
        var_idsatrgatd_i: f64,
        var_idsatrstid_i: f64,
        var_mefftatbotd_i: f64,
        var_mefftatgatd_i: f64,
        var_mefftatstid_i: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_one_over_one_minus_pgat2nd: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbotd_i: f64,
        var_pgat2nd: f64,
        var_pgatd_i: f64,
        var_phigbotd_i: f64,
        var_phiggat2nd: f64,
        var_phiggat2nd_d: f64,
        var_phiggatd_i: f64,
        var_phigrbot_d: f64,
        var_phigrgat2nd: f64,
        var_phigrgat2nd_d: f64,
        var_phigrgat_d: f64,
        var_phigrsti_d: f64,
        var_phigstid_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitrinv: f64,
        var_pstid_i: f64,
        var_stfbbtbotd_i: f64,
        var_stfbbtgatd_i: f64,
        var_stfbbtstid_i: f64,
        var_swgat2nd: f64,
        var_swgat2nd_d: f64,
        var_tkd_1: f64,
        var_tkr_1: f64,
        var_vbirbotd_i: f64,
        var_vbirgat2nd: f64,
        var_vbirgat2nd_d: f64,
        var_vbirgatd_i: f64,
        var_vbirstid_i: f64,
        var_atatbot_d_slot: &mut f64,
        var_atatbot_d_rv_slot: &mut f64,
        var_atatgat_d_slot: &mut f64,
        var_atatgat_d_rv_slot: &mut f64,
        var_atatsti_d_slot: &mut f64,
        var_atatsti_d_rv_slot: &mut f64,
        var_btatpartbot_d_slot: &mut f64,
        var_btatpartbot_d_rv_slot: &mut f64,
        var_btatpartgat_d_slot: &mut f64,
        var_btatpartgat_d_rv_slot: &mut f64,
        var_btatpartsti_d_slot: &mut f64,
        var_btatpartsti_d_rv_slot: &mut f64,
        var_cjobot_d_slot: &mut f64,
        var_cjobot_d_rv_slot: &mut f64,
        var_cjogat2nd_slot: &mut f64,
        var_cjogat2nd_rv_slot: &mut f64,
        var_cjogat_d_slot: &mut f64,
        var_cjogat_d_rv_slot: &mut f64,
        var_cjosti_d_slot: &mut f64,
        var_cjosti_d_rv_slot: &mut f64,
        var_deltaebot_d_slot: &mut f64,
        var_deltaebot_d_rv_slot: &mut f64,
        var_deltaegat_d_slot: &mut f64,
        var_deltaegat_d_rv_slot: &mut f64,
        var_deltaesti_d_slot: &mut f64,
        var_deltaesti_d_rv_slot: &mut f64,
        var_fbbtbot_slot: &mut f64,
        var_fbbtbot_d_slot: &mut f64,
        var_fbbtbot_d_rv_slot: &mut f64,
        var_fbbtbot_rv_slot: &mut f64,
        var_fbbtgat_slot: &mut f64,
        var_fbbtgat_d_slot: &mut f64,
        var_fbbtgat_d_dn5_slot: &mut f64,
        var_fbbtgat_d_dn6_slot: &mut f64,
        var_fbbtgat_d_dn7_slot: &mut f64,
        var_fbbtgat_d_dn8_slot: &mut f64,
        var_fbbtgat_d_rv_slot: &mut f64,
        var_fbbtgat_dn5_slot: &mut f64,
        var_fbbtgat_dn6_slot: &mut f64,
        var_fbbtgat_dn7_slot: &mut f64,
        var_fbbtgat_dn8_slot: &mut f64,
        var_fbbtgat_rv_slot: &mut f64,
        var_fbbtsti_slot: &mut f64,
        var_fbbtsti_d_slot: &mut f64,
        var_fbbtsti_d_rv_slot: &mut f64,
        var_fbbtsti_rv_slot: &mut f64,
        var_ftdbot_d_slot: &mut f64,
        var_ftdbot_d_rv_slot: &mut f64,
        var_ftdgat2nd_slot: &mut f64,
        var_ftdgat2nd_d_slot: &mut f64,
        var_ftdgat2nd_d_rv_slot: &mut f64,
        var_ftdgat2nd_rv_slot: &mut f64,
        var_ftdgat_d_slot: &mut f64,
        var_ftdgat_d_rv_slot: &mut f64,
        var_ftdsti_d_slot: &mut f64,
        var_ftdsti_d_rv_slot: &mut f64,
        var_guard32_slot: &mut f64,
        var_guard32_rv_slot: &mut f64,
        var_guard33_slot: &mut f64,
        var_guard33_rv_slot: &mut f64,
        var_idsatbot_d_slot: &mut f64,
        var_idsatbot_d_rv_slot: &mut f64,
        var_idsatgat_d_slot: &mut f64,
        var_idsatgat_d_rv_slot: &mut f64,
        var_idsatsti_d_slot: &mut f64,
        var_idsatsti_d_rv_slot: &mut f64,
        var_phigdbot_d_slot: &mut f64,
        var_phigdbot_d_rv_slot: &mut f64,
        var_phigdgat2nd_slot: &mut f64,
        var_phigdgat2nd_d_slot: &mut f64,
        var_phigdgat2nd_d_rv_slot: &mut f64,
        var_phigdgat2nd_rv_slot: &mut f64,
        var_phigdgat_d_slot: &mut f64,
        var_phigdgat_d_rv_slot: &mut f64,
        var_phigdsti_d_slot: &mut f64,
        var_phigdsti_d_rv_slot: &mut f64,
        var_qpref2bot_d_slot: &mut f64,
        var_qpref2bot_d_rv_slot: &mut f64,
        var_qpref2gat2nd_slot: &mut f64,
        var_qpref2gat2nd_rv_slot: &mut f64,
        var_qpref2gat_d_slot: &mut f64,
        var_qpref2gat_d_rv_slot: &mut f64,
        var_qpref2sti_d_slot: &mut f64,
        var_qpref2sti_d_rv_slot: &mut f64,
        var_qprefbot_d_slot: &mut f64,
        var_qprefbot_d_rv_slot: &mut f64,
        var_qprefgat2nd_slot: &mut f64,
        var_qprefgat2nd_rv_slot: &mut f64,
        var_qprefgat_d_slot: &mut f64,
        var_qprefgat_d_rv_slot: &mut f64,
        var_qprefsti_d_slot: &mut f64,
        var_qprefsti_d_rv_slot: &mut f64,
        var_ubibot_d_slot: &mut f64,
        var_ubibot_d_rv_slot: &mut f64,
        var_ubigat2nd_slot: &mut f64,
        var_ubigat2nd_d_slot: &mut f64,
        var_ubigat2nd_d_rv_slot: &mut f64,
        var_ubigat2nd_rv_slot: &mut f64,
        var_ubigat_d_slot: &mut f64,
        var_ubigat_d_rv_slot: &mut f64,
        var_ubisti_d_slot: &mut f64,
        var_ubisti_d_rv_slot: &mut f64,
        var_vbibot_d_slot: &mut f64,
        var_vbibot_d_rv_slot: &mut f64,
        var_vbigat2nd_slot: &mut f64,
        var_vbigat2nd_rv_slot: &mut f64,
        var_vbigat_d_slot: &mut f64,
        var_vbigat_d_rv_slot: &mut f64,
        var_vbiinvbot_d_slot: &mut f64,
        var_vbiinvbot_d_rv_slot: &mut f64,
        var_vbiinvgat2nd_slot: &mut f64,
        var_vbiinvgat2nd_rv_slot: &mut f64,
        var_vbiinvgat_d_slot: &mut f64,
        var_vbiinvgat_d_rv_slot: &mut f64,
        var_vbiinvsti_d_slot: &mut f64,
        var_vbiinvsti_d_rv_slot: &mut f64,
        var_vbisti_d_slot: &mut f64,
        var_vbisti_d_rv_slot: &mut f64,
    ) {
        let mut var_atatbot_d: f64 = *var_atatbot_d_slot;
        let mut var_atatbot_d_rv: f64 = *var_atatbot_d_rv_slot;
        let mut var_atatgat_d: f64 = *var_atatgat_d_slot;
        let mut var_atatgat_d_rv: f64 = *var_atatgat_d_rv_slot;
        let mut var_atatsti_d: f64 = *var_atatsti_d_slot;
        let mut var_atatsti_d_rv: f64 = *var_atatsti_d_rv_slot;
        let mut var_btatpartbot_d: f64 = *var_btatpartbot_d_slot;
        let mut var_btatpartbot_d_rv: f64 = *var_btatpartbot_d_rv_slot;
        let mut var_btatpartgat_d: f64 = *var_btatpartgat_d_slot;
        let mut var_btatpartgat_d_rv: f64 = *var_btatpartgat_d_rv_slot;
        let mut var_btatpartsti_d: f64 = *var_btatpartsti_d_slot;
        let mut var_btatpartsti_d_rv: f64 = *var_btatpartsti_d_rv_slot;
        let mut var_cjobot_d: f64 = *var_cjobot_d_slot;
        let mut var_cjobot_d_rv: f64 = *var_cjobot_d_rv_slot;
        let mut var_cjogat2nd: f64 = *var_cjogat2nd_slot;
        let mut var_cjogat2nd_rv: f64 = *var_cjogat2nd_rv_slot;
        let mut var_cjogat_d: f64 = *var_cjogat_d_slot;
        let mut var_cjogat_d_rv: f64 = *var_cjogat_d_rv_slot;
        let mut var_cjosti_d: f64 = *var_cjosti_d_slot;
        let mut var_cjosti_d_rv: f64 = *var_cjosti_d_rv_slot;
        let mut var_deltaebot_d: f64 = *var_deltaebot_d_slot;
        let mut var_deltaebot_d_rv: f64 = *var_deltaebot_d_rv_slot;
        let mut var_deltaegat_d: f64 = *var_deltaegat_d_slot;
        let mut var_deltaegat_d_rv: f64 = *var_deltaegat_d_rv_slot;
        let mut var_deltaesti_d: f64 = *var_deltaesti_d_slot;
        let mut var_deltaesti_d_rv: f64 = *var_deltaesti_d_rv_slot;
        let mut var_fbbtbot: f64 = *var_fbbtbot_slot;
        let mut var_fbbtbot_d: f64 = *var_fbbtbot_d_slot;
        let mut var_fbbtbot_d_rv: f64 = *var_fbbtbot_d_rv_slot;
        let mut var_fbbtbot_rv: f64 = *var_fbbtbot_rv_slot;
        let mut var_fbbtgat: f64 = *var_fbbtgat_slot;
        let mut var_fbbtgat_d: f64 = *var_fbbtgat_d_slot;
        let mut var_fbbtgat_d_dn5: f64 = *var_fbbtgat_d_dn5_slot;
        let mut var_fbbtgat_d_dn6: f64 = *var_fbbtgat_d_dn6_slot;
        let mut var_fbbtgat_d_dn7: f64 = *var_fbbtgat_d_dn7_slot;
        let mut var_fbbtgat_d_dn8: f64 = *var_fbbtgat_d_dn8_slot;
        let mut var_fbbtgat_d_rv: f64 = *var_fbbtgat_d_rv_slot;
        let mut var_fbbtgat_dn5: f64 = *var_fbbtgat_dn5_slot;
        let mut var_fbbtgat_dn6: f64 = *var_fbbtgat_dn6_slot;
        let mut var_fbbtgat_dn7: f64 = *var_fbbtgat_dn7_slot;
        let mut var_fbbtgat_dn8: f64 = *var_fbbtgat_dn8_slot;
        let mut var_fbbtgat_rv: f64 = *var_fbbtgat_rv_slot;
        let mut var_fbbtsti: f64 = *var_fbbtsti_slot;
        let mut var_fbbtsti_d: f64 = *var_fbbtsti_d_slot;
        let mut var_fbbtsti_d_rv: f64 = *var_fbbtsti_d_rv_slot;
        let mut var_fbbtsti_rv: f64 = *var_fbbtsti_rv_slot;
        let mut var_ftdbot_d: f64 = *var_ftdbot_d_slot;
        let mut var_ftdbot_d_rv: f64 = *var_ftdbot_d_rv_slot;
        let mut var_ftdgat2nd: f64 = *var_ftdgat2nd_slot;
        let mut var_ftdgat2nd_d: f64 = *var_ftdgat2nd_d_slot;
        let mut var_ftdgat2nd_d_rv: f64 = *var_ftdgat2nd_d_rv_slot;
        let mut var_ftdgat2nd_rv: f64 = *var_ftdgat2nd_rv_slot;
        let mut var_ftdgat_d: f64 = *var_ftdgat_d_slot;
        let mut var_ftdgat_d_rv: f64 = *var_ftdgat_d_rv_slot;
        let mut var_ftdsti_d: f64 = *var_ftdsti_d_slot;
        let mut var_ftdsti_d_rv: f64 = *var_ftdsti_d_rv_slot;
        let mut var_guard32: f64 = *var_guard32_slot;
        let mut var_guard32_rv: f64 = *var_guard32_rv_slot;
        let mut var_guard33: f64 = *var_guard33_slot;
        let mut var_guard33_rv: f64 = *var_guard33_rv_slot;
        let mut var_idsatbot_d: f64 = *var_idsatbot_d_slot;
        let mut var_idsatbot_d_rv: f64 = *var_idsatbot_d_rv_slot;
        let mut var_idsatgat_d: f64 = *var_idsatgat_d_slot;
        let mut var_idsatgat_d_rv: f64 = *var_idsatgat_d_rv_slot;
        let mut var_idsatsti_d: f64 = *var_idsatsti_d_slot;
        let mut var_idsatsti_d_rv: f64 = *var_idsatsti_d_rv_slot;
        let mut var_phigdbot_d: f64 = *var_phigdbot_d_slot;
        let mut var_phigdbot_d_rv: f64 = *var_phigdbot_d_rv_slot;
        let mut var_phigdgat2nd: f64 = *var_phigdgat2nd_slot;
        let mut var_phigdgat2nd_d: f64 = *var_phigdgat2nd_d_slot;
        let mut var_phigdgat2nd_d_rv: f64 = *var_phigdgat2nd_d_rv_slot;
        let mut var_phigdgat2nd_rv: f64 = *var_phigdgat2nd_rv_slot;
        let mut var_phigdgat_d: f64 = *var_phigdgat_d_slot;
        let mut var_phigdgat_d_rv: f64 = *var_phigdgat_d_rv_slot;
        let mut var_phigdsti_d: f64 = *var_phigdsti_d_slot;
        let mut var_phigdsti_d_rv: f64 = *var_phigdsti_d_rv_slot;
        let mut var_qpref2bot_d: f64 = *var_qpref2bot_d_slot;
        let mut var_qpref2bot_d_rv: f64 = *var_qpref2bot_d_rv_slot;
        let mut var_qpref2gat2nd: f64 = *var_qpref2gat2nd_slot;
        let mut var_qpref2gat2nd_rv: f64 = *var_qpref2gat2nd_rv_slot;
        let mut var_qpref2gat_d: f64 = *var_qpref2gat_d_slot;
        let mut var_qpref2gat_d_rv: f64 = *var_qpref2gat_d_rv_slot;
        let mut var_qpref2sti_d: f64 = *var_qpref2sti_d_slot;
        let mut var_qpref2sti_d_rv: f64 = *var_qpref2sti_d_rv_slot;
        let mut var_qprefbot_d: f64 = *var_qprefbot_d_slot;
        let mut var_qprefbot_d_rv: f64 = *var_qprefbot_d_rv_slot;
        let mut var_qprefgat2nd: f64 = *var_qprefgat2nd_slot;
        let mut var_qprefgat2nd_rv: f64 = *var_qprefgat2nd_rv_slot;
        let mut var_qprefgat_d: f64 = *var_qprefgat_d_slot;
        let mut var_qprefgat_d_rv: f64 = *var_qprefgat_d_rv_slot;
        let mut var_qprefsti_d: f64 = *var_qprefsti_d_slot;
        let mut var_qprefsti_d_rv: f64 = *var_qprefsti_d_rv_slot;
        let mut var_ubibot_d: f64 = *var_ubibot_d_slot;
        let mut var_ubibot_d_rv: f64 = *var_ubibot_d_rv_slot;
        let mut var_ubigat2nd: f64 = *var_ubigat2nd_slot;
        let mut var_ubigat2nd_d: f64 = *var_ubigat2nd_d_slot;
        let mut var_ubigat2nd_d_rv: f64 = *var_ubigat2nd_d_rv_slot;
        let mut var_ubigat2nd_rv: f64 = *var_ubigat2nd_rv_slot;
        let mut var_ubigat_d: f64 = *var_ubigat_d_slot;
        let mut var_ubigat_d_rv: f64 = *var_ubigat_d_rv_slot;
        let mut var_ubisti_d: f64 = *var_ubisti_d_slot;
        let mut var_ubisti_d_rv: f64 = *var_ubisti_d_rv_slot;
        let mut var_vbibot_d: f64 = *var_vbibot_d_slot;
        let mut var_vbibot_d_rv: f64 = *var_vbibot_d_rv_slot;
        let mut var_vbigat2nd: f64 = *var_vbigat2nd_slot;
        let mut var_vbigat2nd_rv: f64 = *var_vbigat2nd_rv_slot;
        let mut var_vbigat_d: f64 = *var_vbigat_d_slot;
        let mut var_vbigat_d_rv: f64 = *var_vbigat_d_rv_slot;
        let mut var_vbiinvbot_d: f64 = *var_vbiinvbot_d_slot;
        let mut var_vbiinvbot_d_rv: f64 = *var_vbiinvbot_d_rv_slot;
        let mut var_vbiinvgat2nd: f64 = *var_vbiinvgat2nd_slot;
        let mut var_vbiinvgat2nd_rv: f64 = *var_vbiinvgat2nd_rv_slot;
        let mut var_vbiinvgat_d: f64 = *var_vbiinvgat_d_slot;
        let mut var_vbiinvgat_d_rv: f64 = *var_vbiinvgat_d_rv_slot;
        let mut var_vbiinvsti_d: f64 = *var_vbiinvsti_d_slot;
        let mut var_vbiinvsti_d_rv: f64 = *var_vbiinvsti_d_rv_slot;
        let mut var_vbisti_d: f64 = *var_vbisti_d_slot;
        let mut var_vbisti_d_rv: f64 = *var_vbisti_d_rv_slot;

        let assign2770_e3092: f64 = (var_tkd_1 - var_tkr_1);
        let assign2770_e3093: f64 = (p.p858 * assign2770_e3092);
        let assign2770_e3094: f64 = (1.0 + assign2770_e3093);
        let assign2770_e3095: f64 = (p.p855 * assign2770_e3094);
        var_fbbtsti = assign2770_e3095;
        var_fbbtsti_rv = 0.0;

        let assign2780_e3101: f64 = (var_tkd_1 - var_tkr_1);
        let assign2780_e3102: f64 = (p.p859 * assign2780_e3101);
        let assign2780_e3103: f64 = (1.0 + assign2780_e3102);
        let assign2780_e3104: f64 = (p.p856 * assign2780_e3103);
        var_fbbtgat = assign2780_e3104;
        var_fbbtgat_dn5 = 0.0;
        var_fbbtgat_dn6 = 0.0;
        var_fbbtgat_dn7 = 0.0;
        var_fbbtgat_dn8 = 0.0;
        var_fbbtgat_rv = 0.0;

        let (assign2790_e3110,) = {
    if (var_fbbtbot > 0.0) {
        (var_fbbtbot,)
    } else {
        (0.0,)
    }
};
        var_fbbtbot = assign2790_e3110;
        var_fbbtbot_rv = 0.0;

        let (assign2800_e3116,) = {
    if (var_fbbtsti > 0.0) {
        (var_fbbtsti,)
    } else {
        (0.0,)
    }
};
        var_fbbtsti = assign2800_e3116;
        var_fbbtsti_rv = 0.0;

        let (assign2810_e3122, assign2810_e3122_d_n5, assign2810_e3122_d_n6, assign2810_e3122_d_n7, assign2810_e3122_d_n8,) = {
    if (var_fbbtgat > 0.0) {
        (var_fbbtgat, var_fbbtgat_dn5, var_fbbtgat_dn6, var_fbbtgat_dn7, var_fbbtgat_dn8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_fbbtgat = assign2810_e3122;
        var_fbbtgat_dn5 = assign2810_e3122_d_n5;
        var_fbbtgat_dn6 = assign2810_e3122_d_n6;
        var_fbbtgat_dn7 = assign2810_e3122_d_n7;
        var_fbbtgat_dn8 = assign2810_e3122_d_n8;
        var_fbbtgat_rv = 0.0;

        let assign2820_e3125: f64 = if var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        var_guard32 = assign2820_e3125;
        var_guard32_rv = 0.0;

        let (assign2830_e3131,) = {
    if (var_guard32 != 0.0) {
        let assign2830_e3129: f64 = (var_phiggat2nd + var_deltaphigd);
        (assign2830_e3129,)
    } else {
        (var_phigdgat2nd,)
    }
};
        var_phigdgat2nd = assign2830_e3131;
        var_phigdgat2nd_rv = 0.0;

        let (assign2840_e3148,) = {
    if (var_guard32 != 0.0) {
        let assign2840_e3135: f64 = (var_auxt).powf(1.5);
        let assign2840_e3139: f64 = (var_phigrgat2nd * var_phitrinv);
        let assign2840_e3142: f64 = (var_phigdgat2nd * var_phitdinv);
        let assign2840_e3143: f64 = (assign2840_e3139 - assign2840_e3142);
        let assign2840_e3144: f64 = (0.5 * assign2840_e3143);
        let assign2840_e3145: f64 = (assign2840_e3144).exp();
        let assign2840_e3146: f64 = (assign2840_e3135 * assign2840_e3145);
        (assign2840_e3146,)
    } else {
        (var_ftdgat2nd,)
    }
};
        var_ftdgat2nd = assign2840_e3148;
        var_ftdgat2nd_rv = 0.0;

        let (assign2850_e3161,) = {
    if (var_guard32 != 0.0) {
        let assign2850_e3152: f64 = (var_vbirgat2nd * var_auxt);
        let assign2850_e3155: f64 = (2.0 * var_phitd);
        let assign2850_e3157: f64 = (var_ftdgat2nd).ln();
        let assign2850_e3158: f64 = (assign2850_e3155 * assign2850_e3157);
        let assign2850_e3159: f64 = (assign2850_e3152 - assign2850_e3158);
        (assign2850_e3159,)
    } else {
        (var_ubigat2nd,)
    }
};
        var_ubigat2nd = assign2850_e3161;
        var_ubigat2nd_rv = 0.0;

        let (assign2860_e3177,) = {
    if (var_guard32 != 0.0) {
        let assign2860_e3168: f64 = (0.05 - var_ubigat2nd);
        let assign2860_e3170: f64 = (assign2860_e3168 * var_phitdinv);
        let assign2860_e3171: f64 = (assign2860_e3170).exp();
        let assign2860_e3172: f64 = (1.0 + assign2860_e3171);
        let assign2860_e3173: f64 = (assign2860_e3172).ln();
        let assign2860_e3174: f64 = (var_phitd * assign2860_e3173);
        let assign2860_e3175: f64 = (var_ubigat2nd + assign2860_e3174);
        (assign2860_e3175,)
    } else {
        (var_vbigat2nd,)
    }
};
        var_vbigat2nd = assign2860_e3177;
        var_vbigat2nd_rv = 0.0;

        let (assign2870_e3183,) = {
    if (var_guard32 != 0.0) {
        let assign2870_e3181: f64 = (1.0 / var_vbigat2nd);
        (assign2870_e3181,)
    } else {
        (var_vbiinvgat2nd,)
    }
};
        var_vbiinvgat2nd = assign2870_e3183;
        var_vbiinvgat2nd_rv = 0.0;

        let (assign2880_e3193,) = {
    if (var_guard32 != 0.0) {
        let assign2880_e3188: f64 = (var_vbirgat2nd * var_vbiinvgat2nd);
        let assign2880_e3190: f64 = (assign2880_e3188).powf(var_pgat2nd);
        let assign2880_e3191: f64 = (var_cjorgat2nd * assign2880_e3190);
        (assign2880_e3191,)
    } else {
        (var_cjogat2nd,)
    }
};
        var_cjogat2nd = assign2880_e3193;
        var_cjogat2nd_rv = 0.0;

        let (assign2890_e3201,) = {
    if (var_guard32 != 0.0) {
        let assign2890_e3197: f64 = (var_cjogat2nd * var_vbigat2nd);
        let assign2890_e3199: f64 = (assign2890_e3197 * var_one_over_one_minus_pgat2nd);
        (assign2890_e3199,)
    } else {
        (var_qprefgat2nd,)
    }
};
        var_qprefgat2nd = assign2890_e3201;
        var_qprefgat2nd_rv = 0.0;

        let (assign2900_e3207,) = {
    if (var_guard32 != 0.0) {
        let assign2900_e3205: f64 = (2.0 * var_cjogat2nd);
        (assign2900_e3205,)
    } else {
        (var_qpref2gat2nd,)
    }
};
        var_qpref2gat2nd = assign2900_e3207;
        var_qpref2gat2nd_rv = 0.0;

        let assign2910_e3210: f64 = (var_phigbotd_i + var_deltaphigd);
        var_phigdbot_d = assign2910_e3210;
        var_phigdbot_d_rv = 0.0;

        let assign2920_e3213: f64 = (var_phigstid_i + var_deltaphigd);
        var_phigdsti_d = assign2920_e3213;
        var_phigdsti_d_rv = 0.0;

        let assign2930_e3216: f64 = (var_phiggatd_i + var_deltaphigd);
        var_phigdgat_d = assign2930_e3216;
        var_phigdgat_d_rv = 0.0;

        let assign2940_e3219: f64 = (var_auxt).powf(1.5);
        let assign2940_e3223: f64 = (var_phigrbot_d * var_phitrinv);
        let assign2940_e3226: f64 = (var_phigdbot_d * var_phitdinv);
        let assign2940_e3227: f64 = (assign2940_e3223 - assign2940_e3226);
        let assign2940_e3228: f64 = (0.5 * assign2940_e3227);
        let assign2940_e3229: f64 = (assign2940_e3228).exp();
        let assign2940_e3230: f64 = (assign2940_e3219 * assign2940_e3229);
        var_ftdbot_d = assign2940_e3230;
        var_ftdbot_d_rv = 0.0;

        let assign2950_e3233: f64 = (var_auxt).powf(1.5);
        let assign2950_e3237: f64 = (var_phigrsti_d * var_phitrinv);
        let assign2950_e3240: f64 = (var_phigdsti_d * var_phitdinv);
        let assign2950_e3241: f64 = (assign2950_e3237 - assign2950_e3240);
        let assign2950_e3242: f64 = (0.5 * assign2950_e3241);
        let assign2950_e3243: f64 = (assign2950_e3242).exp();
        let assign2950_e3244: f64 = (assign2950_e3233 * assign2950_e3243);
        var_ftdsti_d = assign2950_e3244;
        var_ftdsti_d_rv = 0.0;

        let assign2960_e3247: f64 = (var_auxt).powf(1.5);
        let assign2960_e3251: f64 = (var_phigrgat_d * var_phitrinv);
        let assign2960_e3254: f64 = (var_phigdgat_d * var_phitdinv);
        let assign2960_e3255: f64 = (assign2960_e3251 - assign2960_e3254);
        let assign2960_e3256: f64 = (0.5 * assign2960_e3255);
        let assign2960_e3257: f64 = (assign2960_e3256).exp();
        let assign2960_e3258: f64 = (assign2960_e3247 * assign2960_e3257);
        var_ftdgat_d = assign2960_e3258;
        var_ftdgat_d_rv = 0.0;

        let assign2970_e3261: f64 = (var_idsatrbotd_i * var_ftdbot_d);
        let assign2970_e3263: f64 = (assign2970_e3261 * var_ftdbot_d);
        var_idsatbot_d = assign2970_e3263;
        var_idsatbot_d_rv = 0.0;

        let assign2980_e3266: f64 = (var_idsatrstid_i * var_ftdsti_d);
        let assign2980_e3268: f64 = (assign2980_e3266 * var_ftdsti_d);
        var_idsatsti_d = assign2980_e3268;
        var_idsatsti_d_rv = 0.0;

        let assign2990_e3271: f64 = (var_idsatrgatd_i * var_ftdgat_d);
        let assign2990_e3273: f64 = (assign2990_e3271 * var_ftdgat_d);
        var_idsatgat_d = assign2990_e3273;
        var_idsatgat_d_rv = 0.0;

        let assign3000_e3276: f64 = (var_vbirbotd_i * var_auxt);
        let assign3000_e3279: f64 = (2.0 * var_phitd);
        let assign3000_e3281: f64 = (var_ftdbot_d).ln();
        let assign3000_e3282: f64 = (assign3000_e3279 * assign3000_e3281);
        let assign3000_e3283: f64 = (assign3000_e3276 - assign3000_e3282);
        var_ubibot_d = assign3000_e3283;
        var_ubibot_d_rv = 0.0;

        let assign3010_e3286: f64 = (var_vbirstid_i * var_auxt);
        let assign3010_e3289: f64 = (2.0 * var_phitd);
        let assign3010_e3291: f64 = (var_ftdsti_d).ln();
        let assign3010_e3292: f64 = (assign3010_e3289 * assign3010_e3291);
        let assign3010_e3293: f64 = (assign3010_e3286 - assign3010_e3292);
        var_ubisti_d = assign3010_e3293;
        var_ubisti_d_rv = 0.0;

        let assign3020_e3296: f64 = (var_vbirgatd_i * var_auxt);
        let assign3020_e3299: f64 = (2.0 * var_phitd);
        let assign3020_e3301: f64 = (var_ftdgat_d).ln();
        let assign3020_e3302: f64 = (assign3020_e3299 * assign3020_e3301);
        let assign3020_e3303: f64 = (assign3020_e3296 - assign3020_e3302);
        var_ubigat_d = assign3020_e3303;
        var_ubigat_d_rv = 0.0;

        let assign3030_e3309: f64 = (0.05 - var_ubibot_d);
        let assign3030_e3311: f64 = (assign3030_e3309 * var_phitdinv);
        let assign3030_e3312: f64 = (assign3030_e3311).exp();
        let assign3030_e3313: f64 = (1.0 + assign3030_e3312);
        let assign3030_e3314: f64 = (assign3030_e3313).ln();
        let assign3030_e3315: f64 = (var_phitd * assign3030_e3314);
        let assign3030_e3316: f64 = (var_ubibot_d + assign3030_e3315);
        var_vbibot_d = assign3030_e3316;
        var_vbibot_d_rv = 0.0;

        let assign3040_e3322: f64 = (0.05 - var_ubisti_d);
        let assign3040_e3324: f64 = (assign3040_e3322 * var_phitdinv);
        let assign3040_e3325: f64 = (assign3040_e3324).exp();
        let assign3040_e3326: f64 = (1.0 + assign3040_e3325);
        let assign3040_e3327: f64 = (assign3040_e3326).ln();
        let assign3040_e3328: f64 = (var_phitd * assign3040_e3327);
        let assign3040_e3329: f64 = (var_ubisti_d + assign3040_e3328);
        var_vbisti_d = assign3040_e3329;
        var_vbisti_d_rv = 0.0;

        let assign3050_e3335: f64 = (0.05 - var_ubigat_d);
        let assign3050_e3337: f64 = (assign3050_e3335 * var_phitdinv);
        let assign3050_e3338: f64 = (assign3050_e3337).exp();
        let assign3050_e3339: f64 = (1.0 + assign3050_e3338);
        let assign3050_e3340: f64 = (assign3050_e3339).ln();
        let assign3050_e3341: f64 = (var_phitd * assign3050_e3340);
        let assign3050_e3342: f64 = (var_ubigat_d + assign3050_e3341);
        var_vbigat_d = assign3050_e3342;
        var_vbigat_d_rv = 0.0;

        let assign3060_e3345: f64 = (1.0 / var_vbibot_d);
        var_vbiinvbot_d = assign3060_e3345;
        var_vbiinvbot_d_rv = 0.0;

        let assign3070_e3348: f64 = (1.0 / var_vbisti_d);
        var_vbiinvsti_d = assign3070_e3348;
        var_vbiinvsti_d_rv = 0.0;

        let assign3080_e3351: f64 = (1.0 / var_vbigat_d);
        var_vbiinvgat_d = assign3080_e3351;
        var_vbiinvgat_d_rv = 0.0;

        let assign3090_e3355: f64 = (var_vbirbotd_i * var_vbiinvbot_d);
        let assign3090_e3357: f64 = (assign3090_e3355).powf(var_pbotd_i);
        let assign3090_e3358: f64 = (var_cjorbotd_i * assign3090_e3357);
        var_cjobot_d = assign3090_e3358;
        var_cjobot_d_rv = 0.0;

        let assign3100_e3362: f64 = (var_vbirstid_i * var_vbiinvsti_d);
        let assign3100_e3364: f64 = (assign3100_e3362).powf(var_pstid_i);
        let assign3100_e3365: f64 = (var_cjorstid_i * assign3100_e3364);
        var_cjosti_d = assign3100_e3365;
        var_cjosti_d_rv = 0.0;

        let assign3110_e3369: f64 = (var_vbirgatd_i * var_vbiinvgat_d);
        let assign3110_e3371: f64 = (assign3110_e3369).powf(var_pgatd_i);
        let assign3110_e3372: f64 = (var_cjorgatd_i * assign3110_e3371);
        var_cjogat_d = assign3110_e3372;
        var_cjogat_d_rv = 0.0;

        let assign3120_e3375: f64 = (var_cjobot_d * var_vbibot_d);
        let assign3120_e3377: f64 = (assign3120_e3375 * var_one_over_one_minus_pbot_d);
        var_qprefbot_d = assign3120_e3377;
        var_qprefbot_d_rv = 0.0;

        let assign3130_e3380: f64 = (var_cjosti_d * var_vbisti_d);
        let assign3130_e3382: f64 = (assign3130_e3380 * var_one_over_one_minus_psti_d);
        var_qprefsti_d = assign3130_e3382;
        var_qprefsti_d_rv = 0.0;

        let assign3140_e3385: f64 = (var_cjogat_d * var_vbigat_d);
        let assign3140_e3387: f64 = (assign3140_e3385 * var_one_over_one_minus_pgat_d);
        var_qprefgat_d = assign3140_e3387;
        var_qprefgat_d_rv = 0.0;

        let assign3150_e3390: f64 = (2.0 * var_cjobot_d);
        var_qpref2bot_d = assign3150_e3390;
        var_qpref2bot_d_rv = 0.0;

        let assign3160_e3393: f64 = (2.0 * var_cjosti_d);
        var_qpref2sti_d = assign3160_e3393;
        var_qpref2sti_d_rv = 0.0;

        let assign3170_e3396: f64 = (2.0 * var_cjogat_d);
        var_qpref2gat_d = assign3170_e3396;
        var_qpref2gat_d_rv = 0.0;

        let assign3180_e3399: f64 = (0.5 * var_phigdbot_d);
        let assign3180_e3401: f64 = (assign3180_e3399).max(var_phitd);
        var_deltaebot_d = assign3180_e3401;
        var_deltaebot_d_rv = 0.0;

        let assign3190_e3404: f64 = (0.5 * var_phigdsti_d);
        let assign3190_e3406: f64 = (assign3190_e3404).max(var_phitd);
        var_deltaesti_d = assign3190_e3406;
        var_deltaesti_d_rv = 0.0;

        let assign3200_e3409: f64 = (0.5 * var_phigdgat_d);
        let assign3200_e3411: f64 = (assign3200_e3409).max(var_phitd);
        var_deltaegat_d = assign3200_e3411;
        var_deltaegat_d_rv = 0.0;

        let assign3210_e3414: f64 = (var_deltaebot_d * var_phitdinv);
        var_atatbot_d = assign3210_e3414;
        var_atatbot_d_rv = 0.0;

        let assign3220_e3417: f64 = (var_deltaesti_d * var_phitdinv);
        var_atatsti_d = assign3220_e3417;
        var_atatsti_d_rv = 0.0;

        let assign3230_e3420: f64 = (var_deltaegat_d * var_phitdinv);
        var_atatgat_d = assign3230_e3420;
        var_atatgat_d_rv = 0.0;

        let assign3240_e3423: f64 = (32.0 * var_mefftatbotd_i);
        let assign3240_e3425: f64 = (assign3240_e3423 * 9.1093826e-31);
        let assign3240_e3427: f64 = (assign3240_e3425 * 1.6021918e-19);
        let assign3240_e3430: f64 = (var_deltaebot_d * var_deltaebot_d);
        let assign3240_e3432: f64 = (assign3240_e3430 * var_deltaebot_d);
        let assign3240_e3433: f64 = (assign3240_e3427 * assign3240_e3432);
        let assign3240_e3434: f64 = (assign3240_e3433).sqrt();
        let assign3240_e3437: f64 = (3.0 * 1.05457168e-34);
        let assign3240_e3438: f64 = (assign3240_e3434 / assign3240_e3437);
        var_btatpartbot_d = assign3240_e3438;
        var_btatpartbot_d_rv = 0.0;

        let assign3250_e3441: f64 = (32.0 * var_mefftatstid_i);
        let assign3250_e3443: f64 = (assign3250_e3441 * 9.1093826e-31);
        let assign3250_e3445: f64 = (assign3250_e3443 * 1.6021918e-19);
        let assign3250_e3448: f64 = (var_deltaesti_d * var_deltaesti_d);
        let assign3250_e3450: f64 = (assign3250_e3448 * var_deltaesti_d);
        let assign3250_e3451: f64 = (assign3250_e3445 * assign3250_e3450);
        let assign3250_e3452: f64 = (assign3250_e3451).sqrt();
        let assign3250_e3455: f64 = (3.0 * 1.05457168e-34);
        let assign3250_e3456: f64 = (assign3250_e3452 / assign3250_e3455);
        var_btatpartsti_d = assign3250_e3456;
        var_btatpartsti_d_rv = 0.0;

        let assign3260_e3459: f64 = (32.0 * var_mefftatgatd_i);
        let assign3260_e3461: f64 = (assign3260_e3459 * 9.1093826e-31);
        let assign3260_e3463: f64 = (assign3260_e3461 * 1.6021918e-19);
        let assign3260_e3466: f64 = (var_deltaegat_d * var_deltaegat_d);
        let assign3260_e3468: f64 = (assign3260_e3466 * var_deltaegat_d);
        let assign3260_e3469: f64 = (assign3260_e3463 * assign3260_e3468);
        let assign3260_e3470: f64 = (assign3260_e3469).sqrt();
        let assign3260_e3473: f64 = (3.0 * 1.05457168e-34);
        let assign3260_e3474: f64 = (assign3260_e3470 / assign3260_e3473);
        var_btatpartgat_d = assign3260_e3474;
        var_btatpartgat_d_rv = 0.0;

        let assign3270_e3480: f64 = (var_tkd_1 - var_tkr_1);
        let assign3270_e3481: f64 = (var_stfbbtbotd_i * assign3270_e3480);
        let assign3270_e3482: f64 = (1.0 + assign3270_e3481);
        let assign3270_e3483: f64 = (var_fbbtrbotd_i * assign3270_e3482);
        var_fbbtbot_d = assign3270_e3483;
        var_fbbtbot_d_rv = 0.0;

        let assign3280_e3489: f64 = (var_tkd_1 - var_tkr_1);
        let assign3280_e3490: f64 = (var_stfbbtstid_i * assign3280_e3489);
        let assign3280_e3491: f64 = (1.0 + assign3280_e3490);
        let assign3280_e3492: f64 = (var_fbbtrstid_i * assign3280_e3491);
        var_fbbtsti_d = assign3280_e3492;
        var_fbbtsti_d_rv = 0.0;

        let assign3290_e3498: f64 = (var_tkd_1 - var_tkr_1);
        let assign3290_e3499: f64 = (var_stfbbtgatd_i * assign3290_e3498);
        let assign3290_e3500: f64 = (1.0 + assign3290_e3499);
        let assign3290_e3501: f64 = (var_fbbtrgatd_i * assign3290_e3500);
        var_fbbtgat_d = assign3290_e3501;
        var_fbbtgat_d_dn5 = 0.0;
        var_fbbtgat_d_dn6 = 0.0;
        var_fbbtgat_d_dn7 = 0.0;
        var_fbbtgat_d_dn8 = 0.0;
        var_fbbtgat_d_rv = 0.0;

        let (assign3300_e3507,) = {
    if (var_fbbtbot_d > 0.0) {
        (var_fbbtbot_d,)
    } else {
        (0.0,)
    }
};
        var_fbbtbot_d = assign3300_e3507;
        var_fbbtbot_d_rv = 0.0;

        let (assign3310_e3513,) = {
    if (var_fbbtsti_d > 0.0) {
        (var_fbbtsti_d,)
    } else {
        (0.0,)
    }
};
        var_fbbtsti_d = assign3310_e3513;
        var_fbbtsti_d_rv = 0.0;

        let (assign3320_e3519, assign3320_e3519_d_n5, assign3320_e3519_d_n6, assign3320_e3519_d_n7, assign3320_e3519_d_n8,) = {
    if (var_fbbtgat_d > 0.0) {
        (var_fbbtgat_d, var_fbbtgat_d_dn5, var_fbbtgat_d_dn6, var_fbbtgat_d_dn7, var_fbbtgat_d_dn8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_fbbtgat_d = assign3320_e3519;
        var_fbbtgat_d_dn5 = assign3320_e3519_d_n5;
        var_fbbtgat_d_dn6 = assign3320_e3519_d_n6;
        var_fbbtgat_d_dn7 = assign3320_e3519_d_n7;
        var_fbbtgat_d_dn8 = assign3320_e3519_d_n8;
        var_fbbtgat_d_rv = 0.0;

        let assign3330_e3522: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard33 = assign3330_e3522;
        var_guard33_rv = 0.0;

        let (assign3340_e3528,) = {
    if (var_guard33 != 0.0) {
        let assign3340_e3526: f64 = (var_phiggat2nd_d + var_deltaphigd);
        (assign3340_e3526,)
    } else {
        (var_phigdgat2nd_d,)
    }
};
        var_phigdgat2nd_d = assign3340_e3528;
        var_phigdgat2nd_d_rv = 0.0;

        let (assign3350_e3545,) = {
    if (var_guard33 != 0.0) {
        let assign3350_e3532: f64 = (var_auxt).powf(1.5);
        let assign3350_e3536: f64 = (var_phigrgat2nd_d * var_phitrinv);
        let assign3350_e3539: f64 = (var_phigdgat2nd_d * var_phitdinv);
        let assign3350_e3540: f64 = (assign3350_e3536 - assign3350_e3539);
        let assign3350_e3541: f64 = (0.5 * assign3350_e3540);
        let assign3350_e3542: f64 = (assign3350_e3541).exp();
        let assign3350_e3543: f64 = (assign3350_e3532 * assign3350_e3542);
        (assign3350_e3543,)
    } else {
        (var_ftdgat2nd_d,)
    }
};
        var_ftdgat2nd_d = assign3350_e3545;
        var_ftdgat2nd_d_rv = 0.0;

        let (assign3360_e3558,) = {
    if (var_guard33 != 0.0) {
        let assign3360_e3549: f64 = (var_vbirgat2nd_d * var_auxt);
        let assign3360_e3552: f64 = (2.0 * var_phitd);
        let assign3360_e3554: f64 = (var_ftdgat2nd_d).ln();
        let assign3360_e3555: f64 = (assign3360_e3552 * assign3360_e3554);
        let assign3360_e3556: f64 = (assign3360_e3549 - assign3360_e3555);
        (assign3360_e3556,)
    } else {
        (var_ubigat2nd_d,)
    }
};
        var_ubigat2nd_d = assign3360_e3558;
        var_ubigat2nd_d_rv = 0.0;

        *var_atatbot_d_slot = var_atatbot_d;
        *var_atatbot_d_rv_slot = var_atatbot_d_rv;
        *var_atatgat_d_slot = var_atatgat_d;
        *var_atatgat_d_rv_slot = var_atatgat_d_rv;
        *var_atatsti_d_slot = var_atatsti_d;
        *var_atatsti_d_rv_slot = var_atatsti_d_rv;
        *var_btatpartbot_d_slot = var_btatpartbot_d;
        *var_btatpartbot_d_rv_slot = var_btatpartbot_d_rv;
        *var_btatpartgat_d_slot = var_btatpartgat_d;
        *var_btatpartgat_d_rv_slot = var_btatpartgat_d_rv;
        *var_btatpartsti_d_slot = var_btatpartsti_d;
        *var_btatpartsti_d_rv_slot = var_btatpartsti_d_rv;
        *var_cjobot_d_slot = var_cjobot_d;
        *var_cjobot_d_rv_slot = var_cjobot_d_rv;
        *var_cjogat2nd_slot = var_cjogat2nd;
        *var_cjogat2nd_rv_slot = var_cjogat2nd_rv;
        *var_cjogat_d_slot = var_cjogat_d;
        *var_cjogat_d_rv_slot = var_cjogat_d_rv;
        *var_cjosti_d_slot = var_cjosti_d;
        *var_cjosti_d_rv_slot = var_cjosti_d_rv;
        *var_deltaebot_d_slot = var_deltaebot_d;
        *var_deltaebot_d_rv_slot = var_deltaebot_d_rv;
        *var_deltaegat_d_slot = var_deltaegat_d;
        *var_deltaegat_d_rv_slot = var_deltaegat_d_rv;
        *var_deltaesti_d_slot = var_deltaesti_d;
        *var_deltaesti_d_rv_slot = var_deltaesti_d_rv;
        *var_fbbtbot_slot = var_fbbtbot;
        *var_fbbtbot_d_slot = var_fbbtbot_d;
        *var_fbbtbot_d_rv_slot = var_fbbtbot_d_rv;
        *var_fbbtbot_rv_slot = var_fbbtbot_rv;
        *var_fbbtgat_slot = var_fbbtgat;
        *var_fbbtgat_d_slot = var_fbbtgat_d;
        *var_fbbtgat_d_dn5_slot = var_fbbtgat_d_dn5;
        *var_fbbtgat_d_dn6_slot = var_fbbtgat_d_dn6;
        *var_fbbtgat_d_dn7_slot = var_fbbtgat_d_dn7;
        *var_fbbtgat_d_dn8_slot = var_fbbtgat_d_dn8;
        *var_fbbtgat_d_rv_slot = var_fbbtgat_d_rv;
        *var_fbbtgat_dn5_slot = var_fbbtgat_dn5;
        *var_fbbtgat_dn6_slot = var_fbbtgat_dn6;
        *var_fbbtgat_dn7_slot = var_fbbtgat_dn7;
        *var_fbbtgat_dn8_slot = var_fbbtgat_dn8;
        *var_fbbtgat_rv_slot = var_fbbtgat_rv;
        *var_fbbtsti_slot = var_fbbtsti;
        *var_fbbtsti_d_slot = var_fbbtsti_d;
        *var_fbbtsti_d_rv_slot = var_fbbtsti_d_rv;
        *var_fbbtsti_rv_slot = var_fbbtsti_rv;
        *var_ftdbot_d_slot = var_ftdbot_d;
        *var_ftdbot_d_rv_slot = var_ftdbot_d_rv;
        *var_ftdgat2nd_slot = var_ftdgat2nd;
        *var_ftdgat2nd_d_slot = var_ftdgat2nd_d;
        *var_ftdgat2nd_d_rv_slot = var_ftdgat2nd_d_rv;
        *var_ftdgat2nd_rv_slot = var_ftdgat2nd_rv;
        *var_ftdgat_d_slot = var_ftdgat_d;
        *var_ftdgat_d_rv_slot = var_ftdgat_d_rv;
        *var_ftdsti_d_slot = var_ftdsti_d;
        *var_ftdsti_d_rv_slot = var_ftdsti_d_rv;
        *var_guard32_slot = var_guard32;
        *var_guard32_rv_slot = var_guard32_rv;
        *var_guard33_slot = var_guard33;
        *var_guard33_rv_slot = var_guard33_rv;
        *var_idsatbot_d_slot = var_idsatbot_d;
        *var_idsatbot_d_rv_slot = var_idsatbot_d_rv;
        *var_idsatgat_d_slot = var_idsatgat_d;
        *var_idsatgat_d_rv_slot = var_idsatgat_d_rv;
        *var_idsatsti_d_slot = var_idsatsti_d;
        *var_idsatsti_d_rv_slot = var_idsatsti_d_rv;
        *var_phigdbot_d_slot = var_phigdbot_d;
        *var_phigdbot_d_rv_slot = var_phigdbot_d_rv;
        *var_phigdgat2nd_slot = var_phigdgat2nd;
        *var_phigdgat2nd_d_slot = var_phigdgat2nd_d;
        *var_phigdgat2nd_d_rv_slot = var_phigdgat2nd_d_rv;
        *var_phigdgat2nd_rv_slot = var_phigdgat2nd_rv;
        *var_phigdgat_d_slot = var_phigdgat_d;
        *var_phigdgat_d_rv_slot = var_phigdgat_d_rv;
        *var_phigdsti_d_slot = var_phigdsti_d;
        *var_phigdsti_d_rv_slot = var_phigdsti_d_rv;
        *var_qpref2bot_d_slot = var_qpref2bot_d;
        *var_qpref2bot_d_rv_slot = var_qpref2bot_d_rv;
        *var_qpref2gat2nd_slot = var_qpref2gat2nd;
        *var_qpref2gat2nd_rv_slot = var_qpref2gat2nd_rv;
        *var_qpref2gat_d_slot = var_qpref2gat_d;
        *var_qpref2gat_d_rv_slot = var_qpref2gat_d_rv;
        *var_qpref2sti_d_slot = var_qpref2sti_d;
        *var_qpref2sti_d_rv_slot = var_qpref2sti_d_rv;
        *var_qprefbot_d_slot = var_qprefbot_d;
        *var_qprefbot_d_rv_slot = var_qprefbot_d_rv;
        *var_qprefgat2nd_slot = var_qprefgat2nd;
        *var_qprefgat2nd_rv_slot = var_qprefgat2nd_rv;
        *var_qprefgat_d_slot = var_qprefgat_d;
        *var_qprefgat_d_rv_slot = var_qprefgat_d_rv;
        *var_qprefsti_d_slot = var_qprefsti_d;
        *var_qprefsti_d_rv_slot = var_qprefsti_d_rv;
        *var_ubibot_d_slot = var_ubibot_d;
        *var_ubibot_d_rv_slot = var_ubibot_d_rv;
        *var_ubigat2nd_slot = var_ubigat2nd;
        *var_ubigat2nd_d_slot = var_ubigat2nd_d;
        *var_ubigat2nd_d_rv_slot = var_ubigat2nd_d_rv;
        *var_ubigat2nd_rv_slot = var_ubigat2nd_rv;
        *var_ubigat_d_slot = var_ubigat_d;
        *var_ubigat_d_rv_slot = var_ubigat_d_rv;
        *var_ubisti_d_slot = var_ubisti_d;
        *var_ubisti_d_rv_slot = var_ubisti_d_rv;
        *var_vbibot_d_slot = var_vbibot_d;
        *var_vbibot_d_rv_slot = var_vbibot_d_rv;
        *var_vbigat2nd_slot = var_vbigat2nd;
        *var_vbigat2nd_rv_slot = var_vbigat2nd_rv;
        *var_vbigat_d_slot = var_vbigat_d;
        *var_vbigat_d_rv_slot = var_vbigat_d_rv;
        *var_vbiinvbot_d_slot = var_vbiinvbot_d;
        *var_vbiinvbot_d_rv_slot = var_vbiinvbot_d_rv;
        *var_vbiinvgat2nd_slot = var_vbiinvgat2nd;
        *var_vbiinvgat2nd_rv_slot = var_vbiinvgat2nd_rv;
        *var_vbiinvgat_d_slot = var_vbiinvgat_d;
        *var_vbiinvgat_d_rv_slot = var_vbiinvgat_d_rv;
        *var_vbiinvsti_d_slot = var_vbiinvsti_d;
        *var_vbiinvsti_d_rv_slot = var_vbiinvsti_d_rv;
        *var_vbisti_d_slot = var_vbisti_d;
        *var_vbisti_d_rv_slot = var_vbisti_d_rv;
    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        var_cjorgat2nd_d: f64,
        var_guard33: f64,
        var_one_over_one_minus_pgat2nd_d: f64,
        var_pgat2nd_d: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_ubigat2nd_d: f64,
        var_vbirgat2nd_d: f64,
        var_a1_p_slot: &mut f64,
        var_a1_p_rv_slot: &mut f64,
        var_a2_p_slot: &mut f64,
        var_a2_p_rv_slot: &mut f64,
        var_abdrain_i_slot: &mut f64,
        var_abdrain_i_rv_slot: &mut f64,
        var_absource_i_slot: &mut f64,
        var_absource_i_rv_slot: &mut f64,
        var_ad_i_slot: &mut f64,
        var_ad_i_rv_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp1_p_rv_slot: &mut f64,
        var_alp2_p_slot: &mut f64,
        var_alp2_p_rv_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_alp_p_rv_slot: &mut f64,
        var_as_i_slot: &mut f64,
        var_as_i_rv_slot: &mut f64,
        var_ax_p_slot: &mut f64,
        var_ax_p_rv_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_rv_slot: &mut f64,
        var_cfb_p_slot: &mut f64,
        var_cfb_p_rv_slot: &mut f64,
        var_cfd_p_slot: &mut f64,
        var_cfd_p_rv_slot: &mut f64,
        var_cjogat2nd_d_slot: &mut f64,
        var_cjogat2nd_d_rv_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_cs_p_rv_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ct_p_rv_slot: &mut f64,
        var_ctb_p_slot: &mut f64,
        var_ctb_p_rv_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_ctg_p_rv_slot: &mut f64,
        var_dellps_slot: &mut f64,
        var_dellps_rv_slot: &mut f64,
        var_delwod_slot: &mut f64,
        var_delwod_rv_slot: &mut f64,
        var_dphib_p_slot: &mut f64,
        var_dphib_p_rv_slot: &mut f64,
        var_dvsbnud_p_slot: &mut f64,
        var_dvsbnud_p_rv_slot: &mut f64,
        var_epsrox_p_slot: &mut f64,
        var_epsrox_p_rv_slot: &mut f64,
        var_feta_p_slot: &mut f64,
        var_feta_p_rv_slot: &mut f64,
        var_gfacnud_p_slot: &mut f64,
        var_gfacnud_p_rv_slot: &mut f64,
        var_guard34_slot: &mut f64,
        var_guard34_rv_slot: &mut f64,
        var_iae_slot: &mut f64,
        var_iae_rv_slot: &mut f64,
        var_iiae_slot: &mut f64,
        var_iiae_rv_slot: &mut f64,
        var_iilcv_slot: &mut f64,
        var_iilcv_rv_slot: &mut f64,
        var_iiwcv_slot: &mut f64,
        var_iiwcv_rv_slot: &mut f64,
        var_iiwe_slot: &mut f64,
        var_iiwe_rv_slot: &mut f64,
        var_iiwecv_slot: &mut f64,
        var_iiwecv_rv_slot: &mut f64,
        var_il_slot: &mut f64,
        var_il_rv_slot: &mut f64,
        var_ile_slot: &mut f64,
        var_ile2_slot: &mut f64,
        var_ile2_rv_slot: &mut f64,
        var_ile_rv_slot: &mut f64,
        var_invnf_slot: &mut f64,
        var_invnf_rv_slot: &mut f64,
        var_iw_slot: &mut f64,
        var_iw_rv_slot: &mut f64,
        var_iwe_slot: &mut f64,
        var_iwe_rv_slot: &mut f64,
        var_jw_i_slot: &mut f64,
        var_jw_i_rv_slot: &mut f64,
        var_l_i_slot: &mut f64,
        var_l_i_rv_slot: &mut f64,
        var_lcv_slot: &mut f64,
        var_lcv_rv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_rv_slot: &mut f64,
        var_lecv_slot: &mut f64,
        var_lecv_rv_slot: &mut f64,
        var_lgdrain_i_slot: &mut f64,
        var_lgdrain_i_rv_slot: &mut f64,
        var_lgsource_i_slot: &mut f64,
        var_lgsource_i_rv_slot: &mut f64,
        var_lsdrain_i_slot: &mut f64,
        var_lsdrain_i_rv_slot: &mut f64,
        var_lssource_i_slot: &mut f64,
        var_lssource_i_rv_slot: &mut f64,
        var_mue_p_slot: &mut f64,
        var_mue_p_rv_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_neff_p_rv_slot: &mut f64,
        var_nf_i_slot: &mut f64,
        var_nf_i_rv_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_nov_p_rv_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_novd_p_rv_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_np_p_rv_slot: &mut f64,
        var_pd_i_slot: &mut f64,
        var_pd_i_rv_slot: &mut f64,
        var_ps_i_slot: &mut f64,
        var_ps_i_rv_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psce_p_rv_slot: &mut f64,
        var_psceb_p_slot: &mut f64,
        var_psceb_p_rv_slot: &mut f64,
        var_psced_p_slot: &mut f64,
        var_psced_p_rv_slot: &mut f64,
        var_qpref2gat2nd_d_slot: &mut f64,
        var_qpref2gat2nd_d_rv_slot: &mut f64,
        var_qprefgat2nd_d_slot: &mut f64,
        var_qprefgat2nd_d_rv_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rs_p_rv_slot: &mut f64,
        var_rsb_p_slot: &mut f64,
        var_rsb_p_rv_slot: &mut f64,
        var_rsg_p_slot: &mut f64,
        var_rsg_p_rv_slot: &mut f64,
        var_sa_i_slot: &mut f64,
        var_sa_i_rv_slot: &mut f64,
        var_sb_i_slot: &mut f64,
        var_sb_i_rv_slot: &mut f64,
        var_sc_i_slot: &mut f64,
        var_sc_i_rv_slot: &mut f64,
        var_sca_i_slot: &mut f64,
        var_sca_i_rv_slot: &mut f64,
        var_scb_i_slot: &mut f64,
        var_scb_i_rv_slot: &mut f64,
        var_scc_i_slot: &mut f64,
        var_scc_i_rv_slot: &mut f64,
        var_sd_i_slot: &mut f64,
        var_sd_i_rv_slot: &mut f64,
        var_st2vfb_p_slot: &mut f64,
        var_st2vfb_p_rv_slot: &mut f64,
        var_stbet_p_slot: &mut f64,
        var_stbet_p_rv_slot: &mut f64,
        var_stcs_p_slot: &mut f64,
        var_stcs_p_rv_slot: &mut f64,
        var_stct_p_slot: &mut f64,
        var_stct_p_rv_slot: &mut f64,
        var_stmue_p_slot: &mut f64,
        var_stmue_p_rv_slot: &mut f64,
        var_strs_p_slot: &mut f64,
        var_strs_p_rv_slot: &mut f64,
        var_stthecs_p_slot: &mut f64,
        var_stthecs_p_rv_slot: &mut f64,
        var_stthemu_p_slot: &mut f64,
        var_stthemu_p_rv_slot: &mut f64,
        var_stthesat_p_slot: &mut f64,
        var_stthesat_p_rv_slot: &mut f64,
        var_stvfb_p_slot: &mut f64,
        var_stvfb_p_rv_slot: &mut f64,
        var_stxcor_p_slot: &mut f64,
        var_stxcor_p_rv_slot: &mut f64,
        var_thecs_p_slot: &mut f64,
        var_thecs_p_rv_slot: &mut f64,
        var_themu_p_slot: &mut f64,
        var_themu_p_rv_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_rv_slot: &mut f64,
        var_thesatb_p_slot: &mut f64,
        var_thesatb_p_rv_slot: &mut f64,
        var_thesatg_p_slot: &mut f64,
        var_thesatg_p_rv_slot: &mut f64,
        var_thesatt_p_slot: &mut f64,
        var_thesatt_p_rv_slot: &mut f64,
        var_tox_p_slot: &mut f64,
        var_tox_p_rv_slot: &mut f64,
        var_toxov_p_slot: &mut f64,
        var_toxov_p_rv_slot: &mut f64,
        var_toxovd_p_slot: &mut f64,
        var_toxovd_p_rv_slot: &mut f64,
        var_vbigat2nd_d_slot: &mut f64,
        var_vbigat2nd_d_rv_slot: &mut f64,
        var_vbiinvgat2nd_d_slot: &mut f64,
        var_vbiinvgat2nd_d_rv_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfb_p_rv_slot: &mut f64,
        var_vp_p_slot: &mut f64,
        var_vp_p_rv_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
        var_vsbnud_p_rv_slot: &mut f64,
        var_w_i_slot: &mut f64,
        var_w_i_rv_slot: &mut f64,
        var_wcv_slot: &mut f64,
        var_wcv_rv_slot: &mut f64,
        var_we_slot: &mut f64,
        var_we_rv_slot: &mut f64,
        var_wecv_slot: &mut f64,
        var_wecv_rv_slot: &mut f64,
        var_xcor_p_slot: &mut f64,
        var_xcor_p_rv_slot: &mut f64,
    ) {
        let mut var_a1_p: f64 = *var_a1_p_slot;
        let mut var_a1_p_rv: f64 = *var_a1_p_rv_slot;
        let mut var_a2_p: f64 = *var_a2_p_slot;
        let mut var_a2_p_rv: f64 = *var_a2_p_rv_slot;
        let mut var_abdrain_i: f64 = *var_abdrain_i_slot;
        let mut var_abdrain_i_rv: f64 = *var_abdrain_i_rv_slot;
        let mut var_absource_i: f64 = *var_absource_i_slot;
        let mut var_absource_i_rv: f64 = *var_absource_i_rv_slot;
        let mut var_ad_i: f64 = *var_ad_i_slot;
        let mut var_ad_i_rv: f64 = *var_ad_i_rv_slot;
        let mut var_alp1_p: f64 = *var_alp1_p_slot;
        let mut var_alp1_p_rv: f64 = *var_alp1_p_rv_slot;
        let mut var_alp2_p: f64 = *var_alp2_p_slot;
        let mut var_alp2_p_rv: f64 = *var_alp2_p_rv_slot;
        let mut var_alp_p: f64 = *var_alp_p_slot;
        let mut var_alp_p_rv: f64 = *var_alp_p_rv_slot;
        let mut var_as_i: f64 = *var_as_i_slot;
        let mut var_as_i_rv: f64 = *var_as_i_rv_slot;
        let mut var_ax_p: f64 = *var_ax_p_slot;
        let mut var_ax_p_rv: f64 = *var_ax_p_rv_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_rv: f64 = *var_cf_p_rv_slot;
        let mut var_cfb_p: f64 = *var_cfb_p_slot;
        let mut var_cfb_p_rv: f64 = *var_cfb_p_rv_slot;
        let mut var_cfd_p: f64 = *var_cfd_p_slot;
        let mut var_cfd_p_rv: f64 = *var_cfd_p_rv_slot;
        let mut var_cjogat2nd_d: f64 = *var_cjogat2nd_d_slot;
        let mut var_cjogat2nd_d_rv: f64 = *var_cjogat2nd_d_rv_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_cs_p_rv: f64 = *var_cs_p_rv_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ct_p_rv: f64 = *var_ct_p_rv_slot;
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctb_p_rv: f64 = *var_ctb_p_rv_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_ctg_p_rv: f64 = *var_ctg_p_rv_slot;
        let mut var_dellps: f64 = *var_dellps_slot;
        let mut var_dellps_rv: f64 = *var_dellps_rv_slot;
        let mut var_delwod: f64 = *var_delwod_slot;
        let mut var_delwod_rv: f64 = *var_delwod_rv_slot;
        let mut var_dphib_p: f64 = *var_dphib_p_slot;
        let mut var_dphib_p_rv: f64 = *var_dphib_p_rv_slot;
        let mut var_dvsbnud_p: f64 = *var_dvsbnud_p_slot;
        let mut var_dvsbnud_p_rv: f64 = *var_dvsbnud_p_rv_slot;
        let mut var_epsrox_p: f64 = *var_epsrox_p_slot;
        let mut var_epsrox_p_rv: f64 = *var_epsrox_p_rv_slot;
        let mut var_feta_p: f64 = *var_feta_p_slot;
        let mut var_feta_p_rv: f64 = *var_feta_p_rv_slot;
        let mut var_gfacnud_p: f64 = *var_gfacnud_p_slot;
        let mut var_gfacnud_p_rv: f64 = *var_gfacnud_p_rv_slot;
        let mut var_guard34: f64 = *var_guard34_slot;
        let mut var_guard34_rv: f64 = *var_guard34_rv_slot;
        let mut var_iae: f64 = *var_iae_slot;
        let mut var_iae_rv: f64 = *var_iae_rv_slot;
        let mut var_iiae: f64 = *var_iiae_slot;
        let mut var_iiae_rv: f64 = *var_iiae_rv_slot;
        let mut var_iilcv: f64 = *var_iilcv_slot;
        let mut var_iilcv_rv: f64 = *var_iilcv_rv_slot;
        let mut var_iiwcv: f64 = *var_iiwcv_slot;
        let mut var_iiwcv_rv: f64 = *var_iiwcv_rv_slot;
        let mut var_iiwe: f64 = *var_iiwe_slot;
        let mut var_iiwe_rv: f64 = *var_iiwe_rv_slot;
        let mut var_iiwecv: f64 = *var_iiwecv_slot;
        let mut var_iiwecv_rv: f64 = *var_iiwecv_rv_slot;
        let mut var_il: f64 = *var_il_slot;
        let mut var_il_rv: f64 = *var_il_rv_slot;
        let mut var_ile: f64 = *var_ile_slot;
        let mut var_ile2: f64 = *var_ile2_slot;
        let mut var_ile2_rv: f64 = *var_ile2_rv_slot;
        let mut var_ile_rv: f64 = *var_ile_rv_slot;
        let mut var_invnf: f64 = *var_invnf_slot;
        let mut var_invnf_rv: f64 = *var_invnf_rv_slot;
        let mut var_iw: f64 = *var_iw_slot;
        let mut var_iw_rv: f64 = *var_iw_rv_slot;
        let mut var_iwe: f64 = *var_iwe_slot;
        let mut var_iwe_rv: f64 = *var_iwe_rv_slot;
        let mut var_jw_i: f64 = *var_jw_i_slot;
        let mut var_jw_i_rv: f64 = *var_jw_i_rv_slot;
        let mut var_l_i: f64 = *var_l_i_slot;
        let mut var_l_i_rv: f64 = *var_l_i_rv_slot;
        let mut var_lcv: f64 = *var_lcv_slot;
        let mut var_lcv_rv: f64 = *var_lcv_rv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_rv: f64 = *var_le_rv_slot;
        let mut var_lecv: f64 = *var_lecv_slot;
        let mut var_lecv_rv: f64 = *var_lecv_rv_slot;
        let mut var_lgdrain_i: f64 = *var_lgdrain_i_slot;
        let mut var_lgdrain_i_rv: f64 = *var_lgdrain_i_rv_slot;
        let mut var_lgsource_i: f64 = *var_lgsource_i_slot;
        let mut var_lgsource_i_rv: f64 = *var_lgsource_i_rv_slot;
        let mut var_lsdrain_i: f64 = *var_lsdrain_i_slot;
        let mut var_lsdrain_i_rv: f64 = *var_lsdrain_i_rv_slot;
        let mut var_lssource_i: f64 = *var_lssource_i_slot;
        let mut var_lssource_i_rv: f64 = *var_lssource_i_rv_slot;
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_mue_p_rv: f64 = *var_mue_p_rv_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_neff_p_rv: f64 = *var_neff_p_rv_slot;
        let mut var_nf_i: f64 = *var_nf_i_slot;
        let mut var_nf_i_rv: f64 = *var_nf_i_rv_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_nov_p_rv: f64 = *var_nov_p_rv_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_novd_p_rv: f64 = *var_novd_p_rv_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_np_p_rv: f64 = *var_np_p_rv_slot;
        let mut var_pd_i: f64 = *var_pd_i_slot;
        let mut var_pd_i_rv: f64 = *var_pd_i_rv_slot;
        let mut var_ps_i: f64 = *var_ps_i_slot;
        let mut var_ps_i_rv: f64 = *var_ps_i_rv_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psce_p_rv: f64 = *var_psce_p_rv_slot;
        let mut var_psceb_p: f64 = *var_psceb_p_slot;
        let mut var_psceb_p_rv: f64 = *var_psceb_p_rv_slot;
        let mut var_psced_p: f64 = *var_psced_p_slot;
        let mut var_psced_p_rv: f64 = *var_psced_p_rv_slot;
        let mut var_qpref2gat2nd_d: f64 = *var_qpref2gat2nd_d_slot;
        let mut var_qpref2gat2nd_d_rv: f64 = *var_qpref2gat2nd_d_rv_slot;
        let mut var_qprefgat2nd_d: f64 = *var_qprefgat2nd_d_slot;
        let mut var_qprefgat2nd_d_rv: f64 = *var_qprefgat2nd_d_rv_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rs_p_rv: f64 = *var_rs_p_rv_slot;
        let mut var_rsb_p: f64 = *var_rsb_p_slot;
        let mut var_rsb_p_rv: f64 = *var_rsb_p_rv_slot;
        let mut var_rsg_p: f64 = *var_rsg_p_slot;
        let mut var_rsg_p_rv: f64 = *var_rsg_p_rv_slot;
        let mut var_sa_i: f64 = *var_sa_i_slot;
        let mut var_sa_i_rv: f64 = *var_sa_i_rv_slot;
        let mut var_sb_i: f64 = *var_sb_i_slot;
        let mut var_sb_i_rv: f64 = *var_sb_i_rv_slot;
        let mut var_sc_i: f64 = *var_sc_i_slot;
        let mut var_sc_i_rv: f64 = *var_sc_i_rv_slot;
        let mut var_sca_i: f64 = *var_sca_i_slot;
        let mut var_sca_i_rv: f64 = *var_sca_i_rv_slot;
        let mut var_scb_i: f64 = *var_scb_i_slot;
        let mut var_scb_i_rv: f64 = *var_scb_i_rv_slot;
        let mut var_scc_i: f64 = *var_scc_i_slot;
        let mut var_scc_i_rv: f64 = *var_scc_i_rv_slot;
        let mut var_sd_i: f64 = *var_sd_i_slot;
        let mut var_sd_i_rv: f64 = *var_sd_i_rv_slot;
        let mut var_st2vfb_p: f64 = *var_st2vfb_p_slot;
        let mut var_st2vfb_p_rv: f64 = *var_st2vfb_p_rv_slot;
        let mut var_stbet_p: f64 = *var_stbet_p_slot;
        let mut var_stbet_p_rv: f64 = *var_stbet_p_rv_slot;
        let mut var_stcs_p: f64 = *var_stcs_p_slot;
        let mut var_stcs_p_rv: f64 = *var_stcs_p_rv_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stct_p_rv: f64 = *var_stct_p_rv_slot;
        let mut var_stmue_p: f64 = *var_stmue_p_slot;
        let mut var_stmue_p_rv: f64 = *var_stmue_p_rv_slot;
        let mut var_strs_p: f64 = *var_strs_p_slot;
        let mut var_strs_p_rv: f64 = *var_strs_p_rv_slot;
        let mut var_stthecs_p: f64 = *var_stthecs_p_slot;
        let mut var_stthecs_p_rv: f64 = *var_stthecs_p_rv_slot;
        let mut var_stthemu_p: f64 = *var_stthemu_p_slot;
        let mut var_stthemu_p_rv: f64 = *var_stthemu_p_rv_slot;
        let mut var_stthesat_p: f64 = *var_stthesat_p_slot;
        let mut var_stthesat_p_rv: f64 = *var_stthesat_p_rv_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
        let mut var_stvfb_p_rv: f64 = *var_stvfb_p_rv_slot;
        let mut var_stxcor_p: f64 = *var_stxcor_p_slot;
        let mut var_stxcor_p_rv: f64 = *var_stxcor_p_rv_slot;
        let mut var_thecs_p: f64 = *var_thecs_p_slot;
        let mut var_thecs_p_rv: f64 = *var_thecs_p_rv_slot;
        let mut var_themu_p: f64 = *var_themu_p_slot;
        let mut var_themu_p_rv: f64 = *var_themu_p_rv_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_rv: f64 = *var_thesat_p_rv_slot;
        let mut var_thesatb_p: f64 = *var_thesatb_p_slot;
        let mut var_thesatb_p_rv: f64 = *var_thesatb_p_rv_slot;
        let mut var_thesatg_p: f64 = *var_thesatg_p_slot;
        let mut var_thesatg_p_rv: f64 = *var_thesatg_p_rv_slot;
        let mut var_thesatt_p: f64 = *var_thesatt_p_slot;
        let mut var_thesatt_p_rv: f64 = *var_thesatt_p_rv_slot;
        let mut var_tox_p: f64 = *var_tox_p_slot;
        let mut var_tox_p_rv: f64 = *var_tox_p_rv_slot;
        let mut var_toxov_p: f64 = *var_toxov_p_slot;
        let mut var_toxov_p_rv: f64 = *var_toxov_p_rv_slot;
        let mut var_toxovd_p: f64 = *var_toxovd_p_slot;
        let mut var_toxovd_p_rv: f64 = *var_toxovd_p_rv_slot;
        let mut var_vbigat2nd_d: f64 = *var_vbigat2nd_d_slot;
        let mut var_vbigat2nd_d_rv: f64 = *var_vbigat2nd_d_rv_slot;
        let mut var_vbiinvgat2nd_d: f64 = *var_vbiinvgat2nd_d_slot;
        let mut var_vbiinvgat2nd_d_rv: f64 = *var_vbiinvgat2nd_d_rv_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfb_p_rv: f64 = *var_vfb_p_rv_slot;
        let mut var_vp_p: f64 = *var_vp_p_slot;
        let mut var_vp_p_rv: f64 = *var_vp_p_rv_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;
        let mut var_vsbnud_p_rv: f64 = *var_vsbnud_p_rv_slot;
        let mut var_w_i: f64 = *var_w_i_slot;
        let mut var_w_i_rv: f64 = *var_w_i_rv_slot;
        let mut var_wcv: f64 = *var_wcv_slot;
        let mut var_wcv_rv: f64 = *var_wcv_rv_slot;
        let mut var_we: f64 = *var_we_slot;
        let mut var_we_rv: f64 = *var_we_rv_slot;
        let mut var_wecv: f64 = *var_wecv_slot;
        let mut var_wecv_rv: f64 = *var_wecv_rv_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;
        let mut var_xcor_p_rv: f64 = *var_xcor_p_rv_slot;

        let (assign3370_e3574,) = {
    if (var_guard33 != 0.0) {
        let assign3370_e3565: f64 = (0.05 - var_ubigat2nd_d);
        let assign3370_e3567: f64 = (assign3370_e3565 * var_phitdinv);
        let assign3370_e3568: f64 = (assign3370_e3567).exp();
        let assign3370_e3569: f64 = (1.0 + assign3370_e3568);
        let assign3370_e3570: f64 = (assign3370_e3569).ln();
        let assign3370_e3571: f64 = (var_phitd * assign3370_e3570);
        let assign3370_e3572: f64 = (var_ubigat2nd_d + assign3370_e3571);
        (assign3370_e3572,)
    } else {
        (var_vbigat2nd_d,)
    }
};
        var_vbigat2nd_d = assign3370_e3574;
        var_vbigat2nd_d_rv = 0.0;

        let (assign3380_e3580,) = {
    if (var_guard33 != 0.0) {
        let assign3380_e3578: f64 = (1.0 / var_vbigat2nd_d);
        (assign3380_e3578,)
    } else {
        (var_vbiinvgat2nd_d,)
    }
};
        var_vbiinvgat2nd_d = assign3380_e3580;
        var_vbiinvgat2nd_d_rv = 0.0;

        let (assign3390_e3590,) = {
    if (var_guard33 != 0.0) {
        let assign3390_e3585: f64 = (var_vbirgat2nd_d * var_vbiinvgat2nd_d);
        let assign3390_e3587: f64 = (assign3390_e3585).powf(var_pgat2nd_d);
        let assign3390_e3588: f64 = (var_cjorgat2nd_d * assign3390_e3587);
        (assign3390_e3588,)
    } else {
        (var_cjogat2nd_d,)
    }
};
        var_cjogat2nd_d = assign3390_e3590;
        var_cjogat2nd_d_rv = 0.0;

        let (assign3400_e3598,) = {
    if (var_guard33 != 0.0) {
        let assign3400_e3594: f64 = (var_cjogat2nd_d * var_vbigat2nd_d);
        let assign3400_e3596: f64 = (assign3400_e3594 * var_one_over_one_minus_pgat2nd_d);
        (assign3400_e3596,)
    } else {
        (var_qprefgat2nd_d,)
    }
};
        var_qprefgat2nd_d = assign3400_e3598;
        var_qprefgat2nd_d_rv = 0.0;

        let (assign3410_e3604,) = {
    if (var_guard33 != 0.0) {
        let assign3410_e3602: f64 = (2.0 * var_cjogat2nd_d);
        (assign3410_e3602,)
    } else {
        (var_qpref2gat2nd_d,)
    }
};
        var_qpref2gat2nd_d = assign3410_e3604;
        var_qpref2gat2nd_d_rv = 0.0;

        var_nf_i = 1.0;
        var_nf_i_rv = 0.0;

        var_invnf = 1.0;
        var_invnf_rv = 0.0;

        var_le = 0.0;
        var_le_rv = 0.0;

        var_we = 0.0;
        var_we_rv = 0.0;

        var_l_i = p.p0;
        var_l_i_rv = 0.0;

        var_w_i = p.p1;
        var_w_i_rv = 0.0;

        var_sa_i = p.p2;
        var_sa_i_rv = 0.0;

        var_sb_i = p.p3;
        var_sb_i_rv = 0.0;

        var_sd_i = p.p4;
        var_sd_i_rv = 0.0;

        var_sc_i = p.p8;
        var_sc_i_rv = 0.0;

        var_absource_i = p.p19;
        var_absource_i_rv = 0.0;

        var_lssource_i = p.p20;
        var_lssource_i_rv = 0.0;

        var_lgsource_i = p.p21;
        var_lgsource_i_rv = 0.0;

        var_abdrain_i = p.p22;
        var_abdrain_i_rv = 0.0;

        var_lsdrain_i = p.p23;
        var_lsdrain_i_rv = 0.0;

        var_lgdrain_i = p.p24;
        var_lgdrain_i_rv = 0.0;

        var_as_i = p.p25;
        var_as_i_rv = 0.0;

        var_ps_i = p.p26;
        var_ps_i_rv = 0.0;

        var_ad_i = p.p27;
        var_ad_i_rv = 0.0;

        var_pd_i = p.p28;
        var_pd_i_rv = 0.0;

        var_jw_i = p.p14;
        var_jw_i_rv = 0.0;

        let assign3640_e3629: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        var_guard34 = assign3640_e3629;
        var_guard34_rv = 0.0;

        let (assign3650_e3638,) = {
    if (var_guard34 != 0.0) {
        let (assign3650_e3636,) = {
            if (p.p9 > 1.0) {
                (p.p9,)
            } else {
                (1.0,)
            }
        };
        (assign3650_e3636,)
    } else {
        (var_nf_i,)
    }
};
        var_nf_i = assign3650_e3638;
        var_nf_i_rv = 0.0;

        let (assign3660_e3645,) = {
    if (var_guard34 != 0.0) {
        let assign3660_e3642: f64 = (var_nf_i + 0.5);
        let assign3660_e3643: f64 = (assign3660_e3642).floor();
        (assign3660_e3643,)
    } else {
        (var_nf_i,)
    }
};
        var_nf_i = assign3660_e3645;
        var_nf_i_rv = 0.0;

        let (assign3670_e3651,) = {
    if (var_guard34 != 0.0) {
        let assign3670_e3649: f64 = (1.0 / var_nf_i);
        (assign3670_e3649,)
    } else {
        (var_invnf,)
    }
};
        var_invnf = assign3670_e3651;
        var_invnf_rv = 0.0;

        let assign3680_e3654: f64 = (var_w_i * var_invnf);
        let (assign3680_e3661,) = {
    if (assign3680_e3654 > 1e-9) {
        let assign3680_e3659: f64 = (var_w_i * var_invnf);
        (assign3680_e3659,)
    } else {
        (1e-9,)
    }
};
        var_w_i = assign3680_e3661;
        var_w_i_rv = 0.0;

        var_sca_i = p.p5;
        var_sca_i_rv = 0.0;

        var_scb_i = p.p6;
        var_scb_i_rv = 0.0;

        var_scc_i = p.p7;
        var_scc_i_rv = 0.0;

        let assign3730_e3673: f64 = (1e-6 / var_l_i);
        var_il = assign3730_e3673;
        var_il_rv = 0.0;

        let assign3740_e3676: f64 = (1e-6 / var_w_i);
        var_iw = assign3740_e3676;
        var_iw_rv = 0.0;

        let assign3750_e3681: f64 = (p.p189 * var_il);
        let assign3750_e3682: f64 = (1.0 + assign3750_e3681);
        let assign3750_e3683: f64 = (p.p188 * assign3750_e3682);
        let assign3750_e3687: f64 = (p.p190 * var_iw);
        let assign3750_e3688: f64 = (1.0 + assign3750_e3687);
        let assign3750_e3689: f64 = (assign3750_e3683 * assign3750_e3688);
        var_dellps = assign3750_e3689;
        var_dellps_rv = 0.0;

        let assign3760_e3694: f64 = (p.p193 * var_il);
        let assign3760_e3695: f64 = (1.0 + assign3760_e3694);
        let assign3760_e3696: f64 = (p.p192 * assign3760_e3695);
        let assign3760_e3700: f64 = (p.p194 * var_iw);
        let assign3760_e3701: f64 = (1.0 + assign3760_e3700);
        let assign3760_e3702: f64 = (assign3760_e3696 * assign3760_e3701);
        var_delwod = assign3760_e3702;
        var_delwod_rv = 0.0;

        let assign3770_e3705: f64 = (var_l_i + var_dellps);
        let assign3770_e3708: f64 = (2.0 * p.p191);
        let assign3770_e3709: f64 = (assign3770_e3705 - assign3770_e3708);
        let (assign3770_e3720,) = {
    if (assign3770_e3709 > 1e-9) {
        let assign3770_e3714: f64 = (var_l_i + var_dellps);
        let assign3770_e3717: f64 = (2.0 * p.p191);
        let assign3770_e3718: f64 = (assign3770_e3714 - assign3770_e3717);
        (assign3770_e3718,)
    } else {
        (1e-9,)
    }
};
        var_le = assign3770_e3720;
        var_le_rv = 0.0;

        let assign3780_e3723: f64 = (var_w_i + var_delwod);
        let assign3780_e3726: f64 = (2.0 * p.p195);
        let assign3780_e3727: f64 = (assign3780_e3723 - assign3780_e3726);
        let (assign3780_e3738,) = {
    if (assign3780_e3727 > 1e-9) {
        let assign3780_e3732: f64 = (var_w_i + var_delwod);
        let assign3780_e3735: f64 = (2.0 * p.p195);
        let assign3780_e3736: f64 = (assign3780_e3732 - assign3780_e3735);
        (assign3780_e3736,)
    } else {
        (1e-9,)
    }
};
        var_we = assign3780_e3738;
        var_we_rv = 0.0;

        let assign3790_e3741: f64 = (1e-6 / var_le);
        var_ile = assign3790_e3741;
        var_ile_rv = 0.0;

        let assign3800_e3744: f64 = (var_ile * var_ile);
        var_ile2 = assign3800_e3744;
        var_ile2_rv = 0.0;

        let assign3810_e3747: f64 = (1e-6 / var_we);
        var_iwe = assign3810_e3747;
        var_iwe_rv = 0.0;

        let assign3820_e3750: f64 = (1.0 / var_iwe);
        var_iiwe = assign3820_e3750;
        var_iiwe_rv = 0.0;

        let assign3830_e3753: f64 = (var_ile * var_iwe);
        var_iae = assign3830_e3753;
        var_iae_rv = 0.0;

        let assign3840_e3756: f64 = (1.0 / var_iae);
        var_iiae = assign3840_e3756;
        var_iiae_rv = 0.0;

        let assign3850_e3759: f64 = (var_l_i + var_dellps);
        let assign3850_e3762: f64 = (2.0 * p.p191);
        let assign3850_e3763: f64 = (assign3850_e3759 - assign3850_e3762);
        let assign3850_e3765: f64 = (assign3850_e3763 + p.p196);
        let (assign3850_e3778,) = {
    if (assign3850_e3765 > 1e-9) {
        let assign3850_e3770: f64 = (var_l_i + var_dellps);
        let assign3850_e3773: f64 = (2.0 * p.p191);
        let assign3850_e3774: f64 = (assign3850_e3770 - assign3850_e3773);
        let assign3850_e3776: f64 = (assign3850_e3774 + p.p196);
        (assign3850_e3776,)
    } else {
        (1e-9,)
    }
};
        var_lecv = assign3850_e3778;
        var_lecv_rv = 0.0;

        let assign3860_e3781: f64 = (var_w_i + var_delwod);
        let assign3860_e3784: f64 = (2.0 * p.p195);
        let assign3860_e3785: f64 = (assign3860_e3781 - assign3860_e3784);
        let assign3860_e3787: f64 = (assign3860_e3785 + p.p197);
        let (assign3860_e3800,) = {
    if (assign3860_e3787 > 1e-9) {
        let assign3860_e3792: f64 = (var_w_i + var_delwod);
        let assign3860_e3795: f64 = (2.0 * p.p195);
        let assign3860_e3796: f64 = (assign3860_e3792 - assign3860_e3795);
        let assign3860_e3798: f64 = (assign3860_e3796 + p.p197);
        (assign3860_e3798,)
    } else {
        (1e-9,)
    }
};
        var_wecv = assign3860_e3800;
        var_wecv_rv = 0.0;

        let assign3870_e3803: f64 = (var_wecv / 1e-6);
        var_iiwecv = assign3870_e3803;
        var_iiwecv_rv = 0.0;

        let assign3880_e3806: f64 = (var_l_i + var_dellps);
        let assign3880_e3808: f64 = (assign3880_e3806 + p.p196);
        let (assign3880_e3817,) = {
    if (assign3880_e3808 > 1e-9) {
        let assign3880_e3813: f64 = (var_l_i + var_dellps);
        let assign3880_e3815: f64 = (assign3880_e3813 + p.p196);
        (assign3880_e3815,)
    } else {
        (1e-9,)
    }
};
        var_lcv = assign3880_e3817;
        var_lcv_rv = 0.0;

        let assign3890_e3820: f64 = (var_w_i + var_delwod);
        let assign3890_e3822: f64 = (assign3890_e3820 + p.p197);
        let (assign3890_e3831,) = {
    if (assign3890_e3822 > 1e-9) {
        let assign3890_e3827: f64 = (var_w_i + var_delwod);
        let assign3890_e3829: f64 = (assign3890_e3827 + p.p197);
        (assign3890_e3829,)
    } else {
        (1e-9,)
    }
};
        var_wcv = assign3890_e3831;
        var_wcv_rv = 0.0;

        let assign3900_e3834: f64 = (var_lcv / 1e-6);
        var_iilcv = assign3900_e3834;
        var_iilcv_rv = 0.0;

        let assign3910_e3837: f64 = (var_wcv / 1e-6);
        var_iiwcv = assign3910_e3837;
        var_iiwcv_rv = 0.0;

        var_vfb_p = p.p57;
        var_vfb_p_rv = 0.0;

        var_stvfb_p = p.p58;
        var_stvfb_p_rv = 0.0;

        var_st2vfb_p = p.p59;
        var_st2vfb_p_rv = 0.0;

        var_tox_p = p.p60;
        var_tox_p_rv = 0.0;

        var_epsrox_p = p.p61;
        var_epsrox_p_rv = 0.0;

        var_neff_p = p.p62;
        var_neff_p_rv = 0.0;

        var_gfacnud_p = p.p63;
        var_gfacnud_p_rv = 0.0;

        var_vsbnud_p = p.p64;
        var_vsbnud_p_rv = 0.0;

        var_dvsbnud_p = p.p65;
        var_dvsbnud_p_rv = 0.0;

        var_dphib_p = p.p66;
        var_dphib_p_rv = 0.0;

        var_np_p = p.p67;
        var_np_p_rv = 0.0;

        var_toxov_p = p.p68;
        var_toxov_p_rv = 0.0;

        var_toxovd_p = p.p69;
        var_toxovd_p_rv = 0.0;

        var_nov_p = p.p70;
        var_nov_p_rv = 0.0;

        var_novd_p = p.p71;
        var_novd_p_rv = 0.0;

        var_ct_p = p.p72;
        var_ct_p_rv = 0.0;

        var_ctg_p = p.p74;
        var_ctg_p_rv = 0.0;

        var_ctb_p = p.p73;
        var_ctb_p_rv = 0.0;

        var_stct_p = p.p75;
        var_stct_p_rv = 0.0;

        var_psce_p = p.p79;
        var_psce_p_rv = 0.0;

        var_psced_p = p.p81;
        var_psced_p_rv = 0.0;

        var_psceb_p = p.p80;
        var_psceb_p_rv = 0.0;

        var_cf_p = p.p76;
        var_cf_p_rv = 0.0;

        var_cfd_p = p.p78;
        var_cfd_p_rv = 0.0;

        var_cfb_p = p.p77;
        var_cfb_p_rv = 0.0;

        var_betn_p = p.p82;
        var_betn_p_rv = 0.0;

        var_stbet_p = p.p83;
        var_stbet_p_rv = 0.0;

        var_mue_p = p.p84;
        var_mue_p_rv = 0.0;

        var_stmue_p = p.p85;
        var_stmue_p_rv = 0.0;

        var_themu_p = p.p86;
        var_themu_p_rv = 0.0;

        var_stthemu_p = p.p87;
        var_stthemu_p_rv = 0.0;

        var_cs_p = p.p88;
        var_cs_p_rv = 0.0;

        var_stcs_p = p.p89;
        var_stcs_p_rv = 0.0;

        var_thecs_p = p.p90;
        var_thecs_p_rv = 0.0;

        var_stthecs_p = p.p91;
        var_stthecs_p_rv = 0.0;

        var_xcor_p = p.p92;
        var_xcor_p_rv = 0.0;

        var_stxcor_p = p.p93;
        var_stxcor_p_rv = 0.0;

        var_feta_p = p.p94;
        var_feta_p_rv = 0.0;

        var_rs_p = p.p95;
        var_rs_p_rv = 0.0;

        var_strs_p = p.p96;
        var_strs_p_rv = 0.0;

        var_rsb_p = p.p97;
        var_rsb_p_rv = 0.0;

        var_rsg_p = p.p98;
        var_rsg_p_rv = 0.0;

        var_thesat_p = p.p99;
        var_thesat_p_rv = 0.0;

        var_stthesat_p = p.p100;
        var_stthesat_p_rv = 0.0;

        var_thesatb_p = p.p101;
        var_thesatb_p_rv = 0.0;

        var_thesatg_p = p.p102;
        var_thesatg_p_rv = 0.0;

        var_thesatt_p = p.p103;
        var_thesatt_p_rv = 0.0;

        var_ax_p = p.p104;
        var_ax_p_rv = 0.0;

        var_alp_p = p.p105;
        var_alp_p_rv = 0.0;

        var_alp1_p = p.p106;
        var_alp1_p_rv = 0.0;

        var_alp2_p = p.p107;
        var_alp2_p_rv = 0.0;

        var_vp_p = p.p108;
        var_vp_p_rv = 0.0;

        var_a1_p = p.p109;
        var_a1_p_rv = 0.0;

        var_a2_p = p.p110;
        var_a2_p_rv = 0.0;

        *var_a1_p_slot = var_a1_p;
        *var_a1_p_rv_slot = var_a1_p_rv;
        *var_a2_p_slot = var_a2_p;
        *var_a2_p_rv_slot = var_a2_p_rv;
        *var_abdrain_i_slot = var_abdrain_i;
        *var_abdrain_i_rv_slot = var_abdrain_i_rv;
        *var_absource_i_slot = var_absource_i;
        *var_absource_i_rv_slot = var_absource_i_rv;
        *var_ad_i_slot = var_ad_i;
        *var_ad_i_rv_slot = var_ad_i_rv;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp1_p_rv_slot = var_alp1_p_rv;
        *var_alp2_p_slot = var_alp2_p;
        *var_alp2_p_rv_slot = var_alp2_p_rv;
        *var_alp_p_slot = var_alp_p;
        *var_alp_p_rv_slot = var_alp_p_rv;
        *var_as_i_slot = var_as_i;
        *var_as_i_rv_slot = var_as_i_rv;
        *var_ax_p_slot = var_ax_p;
        *var_ax_p_rv_slot = var_ax_p_rv;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_rv_slot = var_cf_p_rv;
        *var_cfb_p_slot = var_cfb_p;
        *var_cfb_p_rv_slot = var_cfb_p_rv;
        *var_cfd_p_slot = var_cfd_p;
        *var_cfd_p_rv_slot = var_cfd_p_rv;
        *var_cjogat2nd_d_slot = var_cjogat2nd_d;
        *var_cjogat2nd_d_rv_slot = var_cjogat2nd_d_rv;
        *var_cs_p_slot = var_cs_p;
        *var_cs_p_rv_slot = var_cs_p_rv;
        *var_ct_p_slot = var_ct_p;
        *var_ct_p_rv_slot = var_ct_p_rv;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctb_p_rv_slot = var_ctb_p_rv;
        *var_ctg_p_slot = var_ctg_p;
        *var_ctg_p_rv_slot = var_ctg_p_rv;
        *var_dellps_slot = var_dellps;
        *var_dellps_rv_slot = var_dellps_rv;
        *var_delwod_slot = var_delwod;
        *var_delwod_rv_slot = var_delwod_rv;
        *var_dphib_p_slot = var_dphib_p;
        *var_dphib_p_rv_slot = var_dphib_p_rv;
        *var_dvsbnud_p_slot = var_dvsbnud_p;
        *var_dvsbnud_p_rv_slot = var_dvsbnud_p_rv;
        *var_epsrox_p_slot = var_epsrox_p;
        *var_epsrox_p_rv_slot = var_epsrox_p_rv;
        *var_feta_p_slot = var_feta_p;
        *var_feta_p_rv_slot = var_feta_p_rv;
        *var_gfacnud_p_slot = var_gfacnud_p;
        *var_gfacnud_p_rv_slot = var_gfacnud_p_rv;
        *var_guard34_slot = var_guard34;
        *var_guard34_rv_slot = var_guard34_rv;
        *var_iae_slot = var_iae;
        *var_iae_rv_slot = var_iae_rv;
        *var_iiae_slot = var_iiae;
        *var_iiae_rv_slot = var_iiae_rv;
        *var_iilcv_slot = var_iilcv;
        *var_iilcv_rv_slot = var_iilcv_rv;
        *var_iiwcv_slot = var_iiwcv;
        *var_iiwcv_rv_slot = var_iiwcv_rv;
        *var_iiwe_slot = var_iiwe;
        *var_iiwe_rv_slot = var_iiwe_rv;
        *var_iiwecv_slot = var_iiwecv;
        *var_iiwecv_rv_slot = var_iiwecv_rv;
        *var_il_slot = var_il;
        *var_il_rv_slot = var_il_rv;
        *var_ile_slot = var_ile;
        *var_ile2_slot = var_ile2;
        *var_ile2_rv_slot = var_ile2_rv;
        *var_ile_rv_slot = var_ile_rv;
        *var_invnf_slot = var_invnf;
        *var_invnf_rv_slot = var_invnf_rv;
        *var_iw_slot = var_iw;
        *var_iw_rv_slot = var_iw_rv;
        *var_iwe_slot = var_iwe;
        *var_iwe_rv_slot = var_iwe_rv;
        *var_jw_i_slot = var_jw_i;
        *var_jw_i_rv_slot = var_jw_i_rv;
        *var_l_i_slot = var_l_i;
        *var_l_i_rv_slot = var_l_i_rv;
        *var_lcv_slot = var_lcv;
        *var_lcv_rv_slot = var_lcv_rv;
        *var_le_slot = var_le;
        *var_le_rv_slot = var_le_rv;
        *var_lecv_slot = var_lecv;
        *var_lecv_rv_slot = var_lecv_rv;
        *var_lgdrain_i_slot = var_lgdrain_i;
        *var_lgdrain_i_rv_slot = var_lgdrain_i_rv;
        *var_lgsource_i_slot = var_lgsource_i;
        *var_lgsource_i_rv_slot = var_lgsource_i_rv;
        *var_lsdrain_i_slot = var_lsdrain_i;
        *var_lsdrain_i_rv_slot = var_lsdrain_i_rv;
        *var_lssource_i_slot = var_lssource_i;
        *var_lssource_i_rv_slot = var_lssource_i_rv;
        *var_mue_p_slot = var_mue_p;
        *var_mue_p_rv_slot = var_mue_p_rv;
        *var_neff_p_slot = var_neff_p;
        *var_neff_p_rv_slot = var_neff_p_rv;
        *var_nf_i_slot = var_nf_i;
        *var_nf_i_rv_slot = var_nf_i_rv;
        *var_nov_p_slot = var_nov_p;
        *var_nov_p_rv_slot = var_nov_p_rv;
        *var_novd_p_slot = var_novd_p;
        *var_novd_p_rv_slot = var_novd_p_rv;
        *var_np_p_slot = var_np_p;
        *var_np_p_rv_slot = var_np_p_rv;
        *var_pd_i_slot = var_pd_i;
        *var_pd_i_rv_slot = var_pd_i_rv;
        *var_ps_i_slot = var_ps_i;
        *var_ps_i_rv_slot = var_ps_i_rv;
        *var_psce_p_slot = var_psce_p;
        *var_psce_p_rv_slot = var_psce_p_rv;
        *var_psceb_p_slot = var_psceb_p;
        *var_psceb_p_rv_slot = var_psceb_p_rv;
        *var_psced_p_slot = var_psced_p;
        *var_psced_p_rv_slot = var_psced_p_rv;
        *var_qpref2gat2nd_d_slot = var_qpref2gat2nd_d;
        *var_qpref2gat2nd_d_rv_slot = var_qpref2gat2nd_d_rv;
        *var_qprefgat2nd_d_slot = var_qprefgat2nd_d;
        *var_qprefgat2nd_d_rv_slot = var_qprefgat2nd_d_rv;
        *var_rs_p_slot = var_rs_p;
        *var_rs_p_rv_slot = var_rs_p_rv;
        *var_rsb_p_slot = var_rsb_p;
        *var_rsb_p_rv_slot = var_rsb_p_rv;
        *var_rsg_p_slot = var_rsg_p;
        *var_rsg_p_rv_slot = var_rsg_p_rv;
        *var_sa_i_slot = var_sa_i;
        *var_sa_i_rv_slot = var_sa_i_rv;
        *var_sb_i_slot = var_sb_i;
        *var_sb_i_rv_slot = var_sb_i_rv;
        *var_sc_i_slot = var_sc_i;
        *var_sc_i_rv_slot = var_sc_i_rv;
        *var_sca_i_slot = var_sca_i;
        *var_sca_i_rv_slot = var_sca_i_rv;
        *var_scb_i_slot = var_scb_i;
        *var_scb_i_rv_slot = var_scb_i_rv;
        *var_scc_i_slot = var_scc_i;
        *var_scc_i_rv_slot = var_scc_i_rv;
        *var_sd_i_slot = var_sd_i;
        *var_sd_i_rv_slot = var_sd_i_rv;
        *var_st2vfb_p_slot = var_st2vfb_p;
        *var_st2vfb_p_rv_slot = var_st2vfb_p_rv;
        *var_stbet_p_slot = var_stbet_p;
        *var_stbet_p_rv_slot = var_stbet_p_rv;
        *var_stcs_p_slot = var_stcs_p;
        *var_stcs_p_rv_slot = var_stcs_p_rv;
        *var_stct_p_slot = var_stct_p;
        *var_stct_p_rv_slot = var_stct_p_rv;
        *var_stmue_p_slot = var_stmue_p;
        *var_stmue_p_rv_slot = var_stmue_p_rv;
        *var_strs_p_slot = var_strs_p;
        *var_strs_p_rv_slot = var_strs_p_rv;
        *var_stthecs_p_slot = var_stthecs_p;
        *var_stthecs_p_rv_slot = var_stthecs_p_rv;
        *var_stthemu_p_slot = var_stthemu_p;
        *var_stthemu_p_rv_slot = var_stthemu_p_rv;
        *var_stthesat_p_slot = var_stthesat_p;
        *var_stthesat_p_rv_slot = var_stthesat_p_rv;
        *var_stvfb_p_slot = var_stvfb_p;
        *var_stvfb_p_rv_slot = var_stvfb_p_rv;
        *var_stxcor_p_slot = var_stxcor_p;
        *var_stxcor_p_rv_slot = var_stxcor_p_rv;
        *var_thecs_p_slot = var_thecs_p;
        *var_thecs_p_rv_slot = var_thecs_p_rv;
        *var_themu_p_slot = var_themu_p;
        *var_themu_p_rv_slot = var_themu_p_rv;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_rv_slot = var_thesat_p_rv;
        *var_thesatb_p_slot = var_thesatb_p;
        *var_thesatb_p_rv_slot = var_thesatb_p_rv;
        *var_thesatg_p_slot = var_thesatg_p;
        *var_thesatg_p_rv_slot = var_thesatg_p_rv;
        *var_thesatt_p_slot = var_thesatt_p;
        *var_thesatt_p_rv_slot = var_thesatt_p_rv;
        *var_tox_p_slot = var_tox_p;
        *var_tox_p_rv_slot = var_tox_p_rv;
        *var_toxov_p_slot = var_toxov_p;
        *var_toxov_p_rv_slot = var_toxov_p_rv;
        *var_toxovd_p_slot = var_toxovd_p;
        *var_toxovd_p_rv_slot = var_toxovd_p_rv;
        *var_vbigat2nd_d_slot = var_vbigat2nd_d;
        *var_vbigat2nd_d_rv_slot = var_vbigat2nd_d_rv;
        *var_vbiinvgat2nd_d_slot = var_vbiinvgat2nd_d;
        *var_vbiinvgat2nd_d_rv_slot = var_vbiinvgat2nd_d_rv;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfb_p_rv_slot = var_vfb_p_rv;
        *var_vp_p_slot = var_vp_p;
        *var_vp_p_rv_slot = var_vp_p_rv;
        *var_vsbnud_p_slot = var_vsbnud_p;
        *var_vsbnud_p_rv_slot = var_vsbnud_p_rv;
        *var_w_i_slot = var_w_i;
        *var_w_i_rv_slot = var_w_i_rv;
        *var_wcv_slot = var_wcv;
        *var_wcv_rv_slot = var_wcv_rv;
        *var_we_slot = var_we;
        *var_we_rv_slot = var_we_rv;
        *var_wecv_slot = var_wecv;
        *var_wecv_rv_slot = var_wecv_rv;
        *var_xcor_p_slot = var_xcor_p;
        *var_xcor_p_rv_slot = var_xcor_p_rv;
    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_a3_p_slot: &mut f64,
        var_a3_p_rv_slot: &mut f64,
        var_a4_p_slot: &mut f64,
        var_a4_p_rv_slot: &mut f64,
        var_aa_slot: &mut f64,
        var_aa_rv_slot: &mut f64,
        var_agidl_p_slot: &mut f64,
        var_agidl_p_rv_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_agidld_p_rv_slot: &mut f64,
        var_alp1ac_p_slot: &mut f64,
        var_alp1ac_p_rv_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_alpac_p_rv_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_axac_p_rv_slot: &mut f64,
        var_axinr_p_slot: &mut f64,
        var_axinr_p_rv_slot: &mut f64,
        var_bb_slot: &mut f64,
        var_bb_rv_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_betnedge_p_rv_slot: &mut f64,
        var_bgidl_p_slot: &mut f64,
        var_bgidl_p_rv_slot: &mut f64,
        var_bgidld_p_slot: &mut f64,
        var_bgidld_p_rv_slot: &mut f64,
        var_cfbedge_p_slot: &mut f64,
        var_cfbedge_p_rv_slot: &mut f64,
        var_cfdedge_p_slot: &mut f64,
        var_cfdedge_p_rv_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cfedge_p_rv_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfr_p_rv_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cfrd_p_rv_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgbov_p_rv_slot: &mut f64,
        var_cgidl_p_slot: &mut f64,
        var_cgidl_p_rv_slot: &mut f64,
        var_cgidld_p_slot: &mut f64,
        var_cgidld_p_rv_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgov_p_rv_slot: &mut f64,
        var_cgovaccg_p_slot: &mut f64,
        var_cgovaccg_p_rv_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_cgovd_p_rv_slot: &mut f64,
        var_chib_p_slot: &mut f64,
        var_chib_p_rv_slot: &mut f64,
        var_cinr_p_slot: &mut f64,
        var_cinr_p_rv_slot: &mut f64,
        var_cinrd_p_slot: &mut f64,
        var_cinrd_p_rv_slot: &mut f64,
        var_cox_p_slot: &mut f64,
        var_cox_p_rv_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_ctedge_p_rv_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_delvtac_p_rv_slot: &mut f64,
        var_dphibedge_p_slot: &mut f64,
        var_dphibedge_p_rv_slot: &mut f64,
        var_dvfbinr_p_slot: &mut f64,
        var_dvfbinr_p_rv_slot: &mut f64,
        var_epsrox_p_slot: &mut f64,
        var_epsrox_p_rv_slot: &mut f64,
        var_facneffac_p_slot: &mut f64,
        var_facneffac_p_rv_slot: &mut f64,
        var_fcgovacc_p_slot: &mut f64,
        var_fcgovacc_p_rv_slot: &mut f64,
        var_fcgovaccd_p_slot: &mut f64,
        var_fcgovaccd_p_rv_slot: &mut f64,
        var_fcinracc_p_slot: &mut f64,
        var_fcinracc_p_rv_slot: &mut f64,
        var_fcinrdep_p_slot: &mut f64,
        var_fcinrdep_p_rv_slot: &mut f64,
        var_fnt_p_slot: &mut f64,
        var_fnt_p_rv_slot: &mut f64,
        var_gc2_p_slot: &mut f64,
        var_gc2_p_rv_slot: &mut f64,
        var_gc2ov_p_slot: &mut f64,
        var_gc2ov_p_rv_slot: &mut f64,
        var_gc2ovd_p_slot: &mut f64,
        var_gc2ovd_p_rv_slot: &mut f64,
        var_gc3_p_slot: &mut f64,
        var_gc3_p_rv_slot: &mut f64,
        var_gc3ov_p_slot: &mut f64,
        var_gc3ov_p_rv_slot: &mut f64,
        var_gc3ovd_p_slot: &mut f64,
        var_gc3ovd_p_rv_slot: &mut f64,
        var_gco_p_slot: &mut f64,
        var_gco_p_rv_slot: &mut f64,
        var_guard35_slot: &mut f64,
        var_guard35_rv_slot: &mut f64,
        var_guard36_slot: &mut f64,
        var_guard36_rv_slot: &mut f64,
        var_guard37_slot: &mut f64,
        var_guard37_rv_slot: &mut f64,
        var_guard38_slot: &mut f64,
        var_guard38_rv_slot: &mut f64,
        var_guard39_slot: &mut f64,
        var_guard39_rv_slot: &mut f64,
        var_guard40_slot: &mut f64,
        var_guard40_rv_slot: &mut f64,
        var_guard41_slot: &mut f64,
        var_guard41_rv_slot: &mut f64,
        var_guard42_slot: &mut f64,
        var_guard42_rv_slot: &mut f64,
        var_guard43_slot: &mut f64,
        var_guard43_rv_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_iginv_p_rv_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igov_p_rv_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_igovd_p_rv_slot: &mut f64,
        var_imaxii_p_slot: &mut f64,
        var_imaxii_p_rv_slot: &mut f64,
        var_lpcke_slot: &mut f64,
        var_lpcke_rv_slot: &mut f64,
        var_munqs_p_slot: &mut f64,
        var_munqs_p_rv_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_neffedge_p_rv_slot: &mut f64,
        var_npcke_slot: &mut f64,
        var_npcke_rv_slot: &mut f64,
        var_nsub_slot: &mut f64,
        var_nsub0e_slot: &mut f64,
        var_nsub0e_rv_slot: &mut f64,
        var_nsub_rv_slot: &mut f64,
        var_pscebedge_p_slot: &mut f64,
        var_pscebedge_p_rv_slot: &mut f64,
        var_pscededge_p_slot: &mut f64,
        var_pscededge_p_rv_slot: &mut f64,
        var_psceedge_p_slot: &mut f64,
        var_psceedge_p_rv_slot: &mut f64,
        var_st2vfb_p_slot: &mut f64,
        var_st2vfb_p_rv_slot: &mut f64,
        var_sta2_p_slot: &mut f64,
        var_sta2_p_rv_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_stbetedge_p_rv_slot: &mut f64,
        var_stbgidl_p_slot: &mut f64,
        var_stbgidl_p_rv_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_stbgidld_p_rv_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_stig_p_rv_slot: &mut f64,
        var_stvfb_p_slot: &mut f64,
        var_stvfb_p_rv_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_stvfbedge_p_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
        var_tox_p_slot: &mut f64,
        var_tox_p_rv_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfb_p_rv_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vfbedge_p_rv_slot: &mut f64,
    ) {
        let mut var_a3_p: f64 = *var_a3_p_slot;
        let mut var_a3_p_rv: f64 = *var_a3_p_rv_slot;
        let mut var_a4_p: f64 = *var_a4_p_slot;
        let mut var_a4_p_rv: f64 = *var_a4_p_rv_slot;
        let mut var_aa: f64 = *var_aa_slot;
        let mut var_aa_rv: f64 = *var_aa_rv_slot;
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidl_p_rv: f64 = *var_agidl_p_rv_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_agidld_p_rv: f64 = *var_agidld_p_rv_slot;
        let mut var_alp1ac_p: f64 = *var_alp1ac_p_slot;
        let mut var_alp1ac_p_rv: f64 = *var_alp1ac_p_rv_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_alpac_p_rv: f64 = *var_alpac_p_rv_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_axac_p_rv: f64 = *var_axac_p_rv_slot;
        let mut var_axinr_p: f64 = *var_axinr_p_slot;
        let mut var_axinr_p_rv: f64 = *var_axinr_p_rv_slot;
        let mut var_bb: f64 = *var_bb_slot;
        let mut var_bb_rv: f64 = *var_bb_rv_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_betnedge_p_rv: f64 = *var_betnedge_p_rv_slot;
        let mut var_bgidl_p: f64 = *var_bgidl_p_slot;
        let mut var_bgidl_p_rv: f64 = *var_bgidl_p_rv_slot;
        let mut var_bgidld_p: f64 = *var_bgidld_p_slot;
        let mut var_bgidld_p_rv: f64 = *var_bgidld_p_rv_slot;
        let mut var_cfbedge_p: f64 = *var_cfbedge_p_slot;
        let mut var_cfbedge_p_rv: f64 = *var_cfbedge_p_rv_slot;
        let mut var_cfdedge_p: f64 = *var_cfdedge_p_slot;
        let mut var_cfdedge_p_rv: f64 = *var_cfdedge_p_rv_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cfedge_p_rv: f64 = *var_cfedge_p_rv_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfr_p_rv: f64 = *var_cfr_p_rv_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cfrd_p_rv: f64 = *var_cfrd_p_rv_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgbov_p_rv: f64 = *var_cgbov_p_rv_slot;
        let mut var_cgidl_p: f64 = *var_cgidl_p_slot;
        let mut var_cgidl_p_rv: f64 = *var_cgidl_p_rv_slot;
        let mut var_cgidld_p: f64 = *var_cgidld_p_slot;
        let mut var_cgidld_p_rv: f64 = *var_cgidld_p_rv_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgov_p_rv: f64 = *var_cgov_p_rv_slot;
        let mut var_cgovaccg_p: f64 = *var_cgovaccg_p_slot;
        let mut var_cgovaccg_p_rv: f64 = *var_cgovaccg_p_rv_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_cgovd_p_rv: f64 = *var_cgovd_p_rv_slot;
        let mut var_chib_p: f64 = *var_chib_p_slot;
        let mut var_chib_p_rv: f64 = *var_chib_p_rv_slot;
        let mut var_cinr_p: f64 = *var_cinr_p_slot;
        let mut var_cinr_p_rv: f64 = *var_cinr_p_rv_slot;
        let mut var_cinrd_p: f64 = *var_cinrd_p_slot;
        let mut var_cinrd_p_rv: f64 = *var_cinrd_p_rv_slot;
        let mut var_cox_p: f64 = *var_cox_p_slot;
        let mut var_cox_p_rv: f64 = *var_cox_p_rv_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_ctedge_p_rv: f64 = *var_ctedge_p_rv_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_delvtac_p_rv: f64 = *var_delvtac_p_rv_slot;
        let mut var_dphibedge_p: f64 = *var_dphibedge_p_slot;
        let mut var_dphibedge_p_rv: f64 = *var_dphibedge_p_rv_slot;
        let mut var_dvfbinr_p: f64 = *var_dvfbinr_p_slot;
        let mut var_dvfbinr_p_rv: f64 = *var_dvfbinr_p_rv_slot;
        let mut var_epsrox_p: f64 = *var_epsrox_p_slot;
        let mut var_epsrox_p_rv: f64 = *var_epsrox_p_rv_slot;
        let mut var_facneffac_p: f64 = *var_facneffac_p_slot;
        let mut var_facneffac_p_rv: f64 = *var_facneffac_p_rv_slot;
        let mut var_fcgovacc_p: f64 = *var_fcgovacc_p_slot;
        let mut var_fcgovacc_p_rv: f64 = *var_fcgovacc_p_rv_slot;
        let mut var_fcgovaccd_p: f64 = *var_fcgovaccd_p_slot;
        let mut var_fcgovaccd_p_rv: f64 = *var_fcgovaccd_p_rv_slot;
        let mut var_fcinracc_p: f64 = *var_fcinracc_p_slot;
        let mut var_fcinracc_p_rv: f64 = *var_fcinracc_p_rv_slot;
        let mut var_fcinrdep_p: f64 = *var_fcinrdep_p_slot;
        let mut var_fcinrdep_p_rv: f64 = *var_fcinrdep_p_rv_slot;
        let mut var_fnt_p: f64 = *var_fnt_p_slot;
        let mut var_fnt_p_rv: f64 = *var_fnt_p_rv_slot;
        let mut var_gc2_p: f64 = *var_gc2_p_slot;
        let mut var_gc2_p_rv: f64 = *var_gc2_p_rv_slot;
        let mut var_gc2ov_p: f64 = *var_gc2ov_p_slot;
        let mut var_gc2ov_p_rv: f64 = *var_gc2ov_p_rv_slot;
        let mut var_gc2ovd_p: f64 = *var_gc2ovd_p_slot;
        let mut var_gc2ovd_p_rv: f64 = *var_gc2ovd_p_rv_slot;
        let mut var_gc3_p: f64 = *var_gc3_p_slot;
        let mut var_gc3_p_rv: f64 = *var_gc3_p_rv_slot;
        let mut var_gc3ov_p: f64 = *var_gc3ov_p_slot;
        let mut var_gc3ov_p_rv: f64 = *var_gc3ov_p_rv_slot;
        let mut var_gc3ovd_p: f64 = *var_gc3ovd_p_slot;
        let mut var_gc3ovd_p_rv: f64 = *var_gc3ovd_p_rv_slot;
        let mut var_gco_p: f64 = *var_gco_p_slot;
        let mut var_gco_p_rv: f64 = *var_gco_p_rv_slot;
        let mut var_guard35: f64 = *var_guard35_slot;
        let mut var_guard35_rv: f64 = *var_guard35_rv_slot;
        let mut var_guard36: f64 = *var_guard36_slot;
        let mut var_guard36_rv: f64 = *var_guard36_rv_slot;
        let mut var_guard37: f64 = *var_guard37_slot;
        let mut var_guard37_rv: f64 = *var_guard37_rv_slot;
        let mut var_guard38: f64 = *var_guard38_slot;
        let mut var_guard38_rv: f64 = *var_guard38_rv_slot;
        let mut var_guard39: f64 = *var_guard39_slot;
        let mut var_guard39_rv: f64 = *var_guard39_rv_slot;
        let mut var_guard40: f64 = *var_guard40_slot;
        let mut var_guard40_rv: f64 = *var_guard40_rv_slot;
        let mut var_guard41: f64 = *var_guard41_slot;
        let mut var_guard41_rv: f64 = *var_guard41_rv_slot;
        let mut var_guard42: f64 = *var_guard42_slot;
        let mut var_guard42_rv: f64 = *var_guard42_rv_slot;
        let mut var_guard43: f64 = *var_guard43_slot;
        let mut var_guard43_rv: f64 = *var_guard43_rv_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_iginv_p_rv: f64 = *var_iginv_p_rv_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igov_p_rv: f64 = *var_igov_p_rv_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_igovd_p_rv: f64 = *var_igovd_p_rv_slot;
        let mut var_imaxii_p: f64 = *var_imaxii_p_slot;
        let mut var_imaxii_p_rv: f64 = *var_imaxii_p_rv_slot;
        let mut var_lpcke: f64 = *var_lpcke_slot;
        let mut var_lpcke_rv: f64 = *var_lpcke_rv_slot;
        let mut var_munqs_p: f64 = *var_munqs_p_slot;
        let mut var_munqs_p_rv: f64 = *var_munqs_p_rv_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_neffedge_p_rv: f64 = *var_neffedge_p_rv_slot;
        let mut var_npcke: f64 = *var_npcke_slot;
        let mut var_npcke_rv: f64 = *var_npcke_rv_slot;
        let mut var_nsub: f64 = *var_nsub_slot;
        let mut var_nsub0e: f64 = *var_nsub0e_slot;
        let mut var_nsub0e_rv: f64 = *var_nsub0e_rv_slot;
        let mut var_nsub_rv: f64 = *var_nsub_rv_slot;
        let mut var_pscebedge_p: f64 = *var_pscebedge_p_slot;
        let mut var_pscebedge_p_rv: f64 = *var_pscebedge_p_rv_slot;
        let mut var_pscededge_p: f64 = *var_pscededge_p_slot;
        let mut var_pscededge_p_rv: f64 = *var_pscededge_p_rv_slot;
        let mut var_psceedge_p: f64 = *var_psceedge_p_slot;
        let mut var_psceedge_p_rv: f64 = *var_psceedge_p_rv_slot;
        let mut var_st2vfb_p: f64 = *var_st2vfb_p_slot;
        let mut var_st2vfb_p_rv: f64 = *var_st2vfb_p_rv_slot;
        let mut var_sta2_p: f64 = *var_sta2_p_slot;
        let mut var_sta2_p_rv: f64 = *var_sta2_p_rv_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_stbetedge_p_rv: f64 = *var_stbetedge_p_rv_slot;
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidl_p_rv: f64 = *var_stbgidl_p_rv_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_stbgidld_p_rv: f64 = *var_stbgidld_p_rv_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_stig_p_rv: f64 = *var_stig_p_rv_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
        let mut var_stvfb_p_rv: f64 = *var_stvfb_p_rv_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_stvfbedge_p_rv: f64 = *var_stvfbedge_p_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;
        let mut var_tox_p: f64 = *var_tox_p_slot;
        let mut var_tox_p_rv: f64 = *var_tox_p_rv_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfb_p_rv: f64 = *var_vfb_p_rv_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vfbedge_p_rv: f64 = *var_vfbedge_p_rv_slot;

        var_sta2_p = p.p111;
        var_sta2_p_rv = 0.0;

        var_a3_p = p.p112;
        var_a3_p_rv = 0.0;

        var_a4_p = p.p113;
        var_a4_p_rv = 0.0;

        var_imaxii_p = p.p114;
        var_imaxii_p_rv = 0.0;

        var_gco_p = p.p115;
        var_gco_p_rv = 0.0;

        var_iginv_p = p.p116;
        var_iginv_p_rv = 0.0;

        var_igov_p = p.p117;
        var_igov_p_rv = 0.0;

        var_igovd_p = p.p118;
        var_igovd_p_rv = 0.0;

        var_stig_p = p.p119;
        var_stig_p_rv = 0.0;

        var_gc2_p = p.p120;
        var_gc2_p_rv = 0.0;

        var_gc3_p = p.p121;
        var_gc3_p_rv = 0.0;

        var_gc2ov_p = p.p120;
        var_gc2ov_p_rv = 0.0;

        let assign4620_e3949: f64 = if param_given[122] { 1.0 } else { 0.0 };
        let assign4620_e3951: f64 = if assign4620_e3949 == 1.0 { 1.0 } else { 0.0 };
        var_guard35 = assign4620_e3951;
        var_guard35_rv = 0.0;

        let (assign4630_e3955,) = {
    if (var_guard35 != 0.0) {
        (p.p122,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign4630_e3955;
        var_gc2ov_p_rv = 0.0;

        var_gc3ov_p = p.p121;
        var_gc3ov_p_rv = 0.0;

        let assign4650_e3958: f64 = if param_given[123] { 1.0 } else { 0.0 };
        let assign4650_e3960: f64 = if assign4650_e3958 == 1.0 { 1.0 } else { 0.0 };
        var_guard36 = assign4650_e3960;
        var_guard36_rv = 0.0;

        let (assign4660_e3964,) = {
    if (var_guard36 != 0.0) {
        (p.p123,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign4660_e3964;
        var_gc3ov_p_rv = 0.0;

        var_gc2ovd_p = var_gc2ov_p;
        var_gc2ovd_p_rv = 0.0;

        let assign4680_e3967: f64 = if param_given[124] { 1.0 } else { 0.0 };
        let assign4680_e3969: f64 = if assign4680_e3967 == 1.0 { 1.0 } else { 0.0 };
        var_guard37 = assign4680_e3969;
        var_guard37_rv = 0.0;

        let (assign4690_e3973,) = {
    if (var_guard37 != 0.0) {
        (p.p124,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign4690_e3973;
        var_gc2ovd_p_rv = 0.0;

        var_gc3ovd_p = var_gc3ov_p;
        var_gc3ovd_p_rv = 0.0;

        let assign4710_e3976: f64 = if param_given[125] { 1.0 } else { 0.0 };
        let assign4710_e3978: f64 = if assign4710_e3976 == 1.0 { 1.0 } else { 0.0 };
        var_guard38 = assign4710_e3978;
        var_guard38_rv = 0.0;

        let (assign4720_e3982,) = {
    if (var_guard38 != 0.0) {
        (p.p125,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign4720_e3982;
        var_gc3ovd_p_rv = 0.0;

        var_chib_p = p.p126;
        var_chib_p_rv = 0.0;

        var_agidl_p = p.p127;
        var_agidl_p_rv = 0.0;

        var_agidld_p = p.p128;
        var_agidld_p_rv = 0.0;

        var_bgidl_p = p.p129;
        var_bgidl_p_rv = 0.0;

        var_bgidld_p = p.p130;
        var_bgidld_p_rv = 0.0;

        var_stbgidl_p = p.p131;
        var_stbgidl_p_rv = 0.0;

        var_stbgidld_p = p.p132;
        var_stbgidld_p_rv = 0.0;

        var_cgidl_p = p.p133;
        var_cgidl_p_rv = 0.0;

        var_cgidld_p = p.p134;
        var_cgidld_p_rv = 0.0;

        var_cox_p = p.p135;
        var_cox_p_rv = 0.0;

        var_delvtac_p = p.p136;
        var_delvtac_p_rv = 0.0;

        var_facneffac_p = p.p137;
        var_facneffac_p_rv = 0.0;

        var_thesatac_p = p.p99;
        var_thesatac_p_rv = 0.0;

        let assign4860_e3997: f64 = if param_given[138] { 1.0 } else { 0.0 };
        let assign4860_e3999: f64 = if assign4860_e3997 == 1.0 { 1.0 } else { 0.0 };
        var_guard39 = assign4860_e3999;
        var_guard39_rv = 0.0;

        let (assign4870_e4003,) = {
    if (var_guard39 != 0.0) {
        (p.p138,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign4870_e4003;
        var_thesatac_p_rv = 0.0;

        var_axac_p = p.p104;
        var_axac_p_rv = 0.0;

        let assign4890_e4006: f64 = if param_given[139] { 1.0 } else { 0.0 };
        let assign4890_e4008: f64 = if assign4890_e4006 == 1.0 { 1.0 } else { 0.0 };
        var_guard40 = assign4890_e4008;
        var_guard40_rv = 0.0;

        let (assign4900_e4012,) = {
    if (var_guard40 != 0.0) {
        (p.p139,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign4900_e4012;
        var_axac_p_rv = 0.0;

        var_alpac_p = p.p140;
        var_alpac_p_rv = 0.0;

        var_alp1ac_p = p.p141;
        var_alp1ac_p_rv = 0.0;

        var_cgov_p = p.p142;
        var_cgov_p_rv = 0.0;

        var_cgovd_p = p.p143;
        var_cgovd_p_rv = 0.0;

        var_fcgovacc_p = p.p144;
        var_fcgovacc_p_rv = 0.0;

        var_fcgovaccd_p = p.p145;
        var_fcgovaccd_p_rv = 0.0;

        var_cgovaccg_p = p.p146;
        var_cgovaccg_p_rv = 0.0;

        var_cgbov_p = p.p147;
        var_cgbov_p_rv = 0.0;

        var_cinr_p = p.p148;
        var_cinr_p_rv = 0.0;

        var_cinrd_p = p.p149;
        var_cinrd_p_rv = 0.0;

        var_dvfbinr_p = p.p150;
        var_dvfbinr_p_rv = 0.0;

        var_fcinrdep_p = p.p151;
        var_fcinrdep_p_rv = 0.0;

        var_fcinracc_p = p.p152;
        var_fcinracc_p_rv = 0.0;

        var_axinr_p = p.p153;
        var_axinr_p_rv = 0.0;

        var_cfr_p = p.p154;
        var_cfr_p_rv = 0.0;

        var_cfrd_p = p.p155;
        var_cfrd_p_rv = 0.0;

        var_fnt_p = p.p156;
        var_fnt_p_rv = 0.0;

        var_vfbedge_p = p.p162;
        var_vfbedge_p_rv = 0.0;

        var_stvfbedge_p = p.p163;
        var_stvfbedge_p_rv = 0.0;

        var_dphibedge_p = p.p164;
        var_dphibedge_p_rv = 0.0;

        var_neffedge_p = p.p165;
        var_neffedge_p_rv = 0.0;

        var_ctedge_p = p.p166;
        var_ctedge_p_rv = 0.0;

        var_betnedge_p = p.p167;
        var_betnedge_p_rv = 0.0;

        var_stbetedge_p = p.p168;
        var_stbetedge_p_rv = 0.0;

        var_psceedge_p = p.p169;
        var_psceedge_p_rv = 0.0;

        var_pscebedge_p = p.p170;
        var_pscebedge_p_rv = 0.0;

        var_pscededge_p = p.p171;
        var_pscededge_p_rv = 0.0;

        var_cfedge_p = p.p172;
        var_cfedge_p_rv = 0.0;

        var_cfdedge_p = p.p174;
        var_cfdedge_p_rv = 0.0;

        var_cfbedge_p = p.p173;
        var_cfbedge_p_rv = 0.0;

        var_munqs_p = p.p187;
        var_munqs_p_rv = 0.0;

        let assign5390_e4063: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        var_guard41 = assign5390_e4063;
        var_guard41_rv = 0.0;

        let (assign5400_e4081,) = {
    if (var_guard41 != 0.0) {
        let assign5400_e4069: f64 = (var_ile).powf(p.p200);
        let assign5400_e4070: f64 = (p.p199 * assign5400_e4069);
        let assign5400_e4071: f64 = (p.p198 + assign5400_e4070);
        let assign5400_e4074: f64 = (p.p201 * var_iwe);
        let assign5400_e4075: f64 = (assign5400_e4071 + assign5400_e4074);
        let assign5400_e4078: f64 = (p.p202 * var_iae);
        let assign5400_e4079: f64 = (assign5400_e4075 + assign5400_e4078);
        (assign5400_e4079,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign5400_e4081;
        var_vfb_p_rv = 0.0;

        let (assign5410_e4097,) = {
    if (var_guard41 != 0.0) {
        let assign5410_e4086: f64 = (p.p204 * var_ile);
        let assign5410_e4087: f64 = (p.p203 + assign5410_e4086);
        let assign5410_e4090: f64 = (p.p205 * var_iwe);
        let assign5410_e4091: f64 = (assign5410_e4087 + assign5410_e4090);
        let assign5410_e4094: f64 = (p.p206 * var_iae);
        let assign5410_e4095: f64 = (assign5410_e4091 + assign5410_e4094);
        (assign5410_e4095,)
    } else {
        (var_stvfb_p,)
    }
};
        var_stvfb_p = assign5410_e4097;
        var_stvfb_p_rv = 0.0;

        let (assign5420_e4101,) = {
    if (var_guard41 != 0.0) {
        (p.p207,)
    } else {
        (var_st2vfb_p,)
    }
};
        var_st2vfb_p = assign5420_e4101;
        var_st2vfb_p_rv = 0.0;

        let (assign5430_e4105,) = {
    if (var_guard41 != 0.0) {
        (p.p208,)
    } else {
        (var_tox_p,)
    }
};
        var_tox_p = assign5430_e4105;
        var_tox_p_rv = 0.0;

        let (assign5440_e4109,) = {
    if (var_guard41 != 0.0) {
        (p.p209,)
    } else {
        (var_epsrox_p,)
    }
};
        var_epsrox_p = assign5440_e4109;
        var_epsrox_p_rv = 0.0;

        let (assign5450_e4142,) = {
    if (var_guard41 != 0.0) {
        let assign5450_e4115: f64 = (p.p211 * var_iwe);
        let assign5450_e4119: f64 = (var_we / p.p212);
        let assign5450_e4120: f64 = (1.0 + assign5450_e4119);
        let assign5450_e4121: f64 = (assign5450_e4120).ln();
        let assign5450_e4122: f64 = (assign5450_e4115 * assign5450_e4121);
        let assign5450_e4123: f64 = (1.0 + assign5450_e4122);
        let (assign5450_e4139,) = {
            if (assign5450_e4123 > 0.001) {
                let assign5450_e4129: f64 = (p.p211 * var_iwe);
                let assign5450_e4133: f64 = (var_we / p.p212);
                let assign5450_e4134: f64 = (1.0 + assign5450_e4133);
                let assign5450_e4135: f64 = (assign5450_e4134).ln();
                let assign5450_e4136: f64 = (assign5450_e4129 * assign5450_e4135);
                let assign5450_e4137: f64 = (1.0 + assign5450_e4136);
                (assign5450_e4137,)
            } else {
                (0.001,)
            }
        };
        let assign5450_e4140: f64 = (p.p210 * assign5450_e4139);
        (assign5450_e4140,)
    } else {
        (var_nsub0e,)
    }
};
        var_nsub0e = assign5450_e4142;
        var_nsub0e_rv = 0.0;

        let (assign5460_e4175,) = {
    if (var_guard41 != 0.0) {
        let assign5460_e4148: f64 = (p.p214 * var_iwe);
        let assign5460_e4152: f64 = (var_we / p.p215);
        let assign5460_e4153: f64 = (1.0 + assign5460_e4152);
        let assign5460_e4154: f64 = (assign5460_e4153).ln();
        let assign5460_e4155: f64 = (assign5460_e4148 * assign5460_e4154);
        let assign5460_e4156: f64 = (1.0 + assign5460_e4155);
        let (assign5460_e4172,) = {
            if (assign5460_e4156 > 0.001) {
                let assign5460_e4162: f64 = (p.p214 * var_iwe);
                let assign5460_e4166: f64 = (var_we / p.p215);
                let assign5460_e4167: f64 = (1.0 + assign5460_e4166);
                let assign5460_e4168: f64 = (assign5460_e4167).ln();
                let assign5460_e4169: f64 = (assign5460_e4162 * assign5460_e4168);
                let assign5460_e4170: f64 = (1.0 + assign5460_e4169);
                (assign5460_e4170,)
            } else {
                (0.001,)
            }
        };
        let assign5460_e4173: f64 = (p.p213 * assign5460_e4172);
        (assign5460_e4173,)
    } else {
        (var_npcke,)
    }
};
        var_npcke = assign5460_e4175;
        var_npcke_rv = 0.0;

        let (assign5470_e4208,) = {
    if (var_guard41 != 0.0) {
        let assign5470_e4181: f64 = (p.p217 * var_iwe);
        let assign5470_e4185: f64 = (var_we / p.p215);
        let assign5470_e4186: f64 = (1.0 + assign5470_e4185);
        let assign5470_e4187: f64 = (assign5470_e4186).ln();
        let assign5470_e4188: f64 = (assign5470_e4181 * assign5470_e4187);
        let assign5470_e4189: f64 = (1.0 + assign5470_e4188);
        let (assign5470_e4205,) = {
            if (assign5470_e4189 > 0.001) {
                let assign5470_e4195: f64 = (p.p217 * var_iwe);
                let assign5470_e4199: f64 = (var_we / p.p215);
                let assign5470_e4200: f64 = (1.0 + assign5470_e4199);
                let assign5470_e4201: f64 = (assign5470_e4200).ln();
                let assign5470_e4202: f64 = (assign5470_e4195 * assign5470_e4201);
                let assign5470_e4203: f64 = (1.0 + assign5470_e4202);
                (assign5470_e4203,)
            } else {
                (0.001,)
            }
        };
        let assign5470_e4206: f64 = (p.p216 * assign5470_e4205);
        (assign5470_e4206,)
    } else {
        (var_lpcke,)
    }
};
        var_lpcke = assign5470_e4208;
        var_lpcke_rv = 0.0;

        let assign5480_e4212: f64 = (2.0 * var_lpcke);
        let assign5480_e4213: f64 = if var_le > assign5480_e4212 { 1.0 } else { 0.0 };
        var_guard42 = assign5480_e4213;
        var_guard42_rv = 0.0;

        let (assign5490_e4219,) = {
    if ((var_guard41 != 0.0) && (var_guard42 != 0.0)) {
        (75000000000.0,)
    } else {
        (var_aa,)
    }
};
        var_aa = assign5490_e4219;
        var_aa_rv = 0.0;

        let (assign5500_e4233,) = {
    if ((var_guard41 != 0.0) && (var_guard42 != 0.0)) {
        let assign5500_e4226: f64 = (0.5 * var_npcke);
        let assign5500_e4227: f64 = (var_nsub0e + assign5500_e4226);
        let assign5500_e4228: f64 = (assign5500_e4227).sqrt();
        let assign5500_e4230: f64 = (var_nsub0e).sqrt();
        let assign5500_e4231: f64 = (assign5500_e4228 - assign5500_e4230);
        (assign5500_e4231,)
    } else {
        (var_bb,)
    }
};
        var_bb = assign5500_e4233;
        var_bb_rv = 0.0;

        let (assign5510_e4258,) = {
    if ((var_guard41 != 0.0) && (var_guard42 != 0.0)) {
        let assign5510_e4238: f64 = (var_nsub0e).sqrt();
        let assign5510_e4243: f64 = (2.0 * var_lpcke);
        let assign5510_e4245: f64 = (assign5510_e4243 / var_le);
        let assign5510_e4248: f64 = (var_bb / var_aa);
        let assign5510_e4249: f64 = (assign5510_e4248).exp();
        let assign5510_e4251: f64 = (assign5510_e4249 - 1.0);
        let assign5510_e4252: f64 = (assign5510_e4245 * assign5510_e4251);
        let assign5510_e4253: f64 = (1.0 + assign5510_e4252);
        let assign5510_e4254: f64 = (assign5510_e4253).ln();
        let assign5510_e4255: f64 = (var_aa * assign5510_e4254);
        let assign5510_e4256: f64 = (assign5510_e4238 + assign5510_e4255);
        (assign5510_e4256,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5510_e4258;
        var_nsub_rv = 0.0;

        let (assign5520_e4266,) = {
    if ((var_guard41 != 0.0) && (var_guard42 != 0.0)) {
        let assign5520_e4264: f64 = (var_nsub * var_nsub);
        (assign5520_e4264,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5520_e4266;
        var_nsub_rv = 0.0;

        let assign5530_e4269: f64 = if var_le >= var_lpcke { 1.0 } else { 0.0 };
        var_guard43 = assign5530_e4269;
        var_guard43_rv = 0.0;

        let (assign5540_e4284,) = {
    if (((var_guard41 != 0.0) && (var_guard42 == 0.0)) && (var_guard43 != 0.0)) {
        let assign5540_e4279: f64 = (var_npcke * var_lpcke);
        let assign5540_e4281: f64 = (assign5540_e4279 / var_le);
        let assign5540_e4282: f64 = (var_nsub0e + assign5540_e4281);
        (assign5540_e4282,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5540_e4284;
        var_nsub_rv = 0.0;

        *var_a3_p_slot = var_a3_p;
        *var_a3_p_rv_slot = var_a3_p_rv;
        *var_a4_p_slot = var_a4_p;
        *var_a4_p_rv_slot = var_a4_p_rv;
        *var_aa_slot = var_aa;
        *var_aa_rv_slot = var_aa_rv;
        *var_agidl_p_slot = var_agidl_p;
        *var_agidl_p_rv_slot = var_agidl_p_rv;
        *var_agidld_p_slot = var_agidld_p;
        *var_agidld_p_rv_slot = var_agidld_p_rv;
        *var_alp1ac_p_slot = var_alp1ac_p;
        *var_alp1ac_p_rv_slot = var_alp1ac_p_rv;
        *var_alpac_p_slot = var_alpac_p;
        *var_alpac_p_rv_slot = var_alpac_p_rv;
        *var_axac_p_slot = var_axac_p;
        *var_axac_p_rv_slot = var_axac_p_rv;
        *var_axinr_p_slot = var_axinr_p;
        *var_axinr_p_rv_slot = var_axinr_p_rv;
        *var_bb_slot = var_bb;
        *var_bb_rv_slot = var_bb_rv;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_betnedge_p_rv_slot = var_betnedge_p_rv;
        *var_bgidl_p_slot = var_bgidl_p;
        *var_bgidl_p_rv_slot = var_bgidl_p_rv;
        *var_bgidld_p_slot = var_bgidld_p;
        *var_bgidld_p_rv_slot = var_bgidld_p_rv;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfbedge_p_rv_slot = var_cfbedge_p_rv;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfdedge_p_rv_slot = var_cfdedge_p_rv;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cfedge_p_rv_slot = var_cfedge_p_rv;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfr_p_rv_slot = var_cfr_p_rv;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cfrd_p_rv_slot = var_cfrd_p_rv;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgbov_p_rv_slot = var_cgbov_p_rv;
        *var_cgidl_p_slot = var_cgidl_p;
        *var_cgidl_p_rv_slot = var_cgidl_p_rv;
        *var_cgidld_p_slot = var_cgidld_p;
        *var_cgidld_p_rv_slot = var_cgidld_p_rv;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgov_p_rv_slot = var_cgov_p_rv;
        *var_cgovaccg_p_slot = var_cgovaccg_p;
        *var_cgovaccg_p_rv_slot = var_cgovaccg_p_rv;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_cgovd_p_rv_slot = var_cgovd_p_rv;
        *var_chib_p_slot = var_chib_p;
        *var_chib_p_rv_slot = var_chib_p_rv;
        *var_cinr_p_slot = var_cinr_p;
        *var_cinr_p_rv_slot = var_cinr_p_rv;
        *var_cinrd_p_slot = var_cinrd_p;
        *var_cinrd_p_rv_slot = var_cinrd_p_rv;
        *var_cox_p_slot = var_cox_p;
        *var_cox_p_rv_slot = var_cox_p_rv;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_ctedge_p_rv_slot = var_ctedge_p_rv;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_delvtac_p_rv_slot = var_delvtac_p_rv;
        *var_dphibedge_p_slot = var_dphibedge_p;
        *var_dphibedge_p_rv_slot = var_dphibedge_p_rv;
        *var_dvfbinr_p_slot = var_dvfbinr_p;
        *var_dvfbinr_p_rv_slot = var_dvfbinr_p_rv;
        *var_epsrox_p_slot = var_epsrox_p;
        *var_epsrox_p_rv_slot = var_epsrox_p_rv;
        *var_facneffac_p_slot = var_facneffac_p;
        *var_facneffac_p_rv_slot = var_facneffac_p_rv;
        *var_fcgovacc_p_slot = var_fcgovacc_p;
        *var_fcgovacc_p_rv_slot = var_fcgovacc_p_rv;
        *var_fcgovaccd_p_slot = var_fcgovaccd_p;
        *var_fcgovaccd_p_rv_slot = var_fcgovaccd_p_rv;
        *var_fcinracc_p_slot = var_fcinracc_p;
        *var_fcinracc_p_rv_slot = var_fcinracc_p_rv;
        *var_fcinrdep_p_slot = var_fcinrdep_p;
        *var_fcinrdep_p_rv_slot = var_fcinrdep_p_rv;
        *var_fnt_p_slot = var_fnt_p;
        *var_fnt_p_rv_slot = var_fnt_p_rv;
        *var_gc2_p_slot = var_gc2_p;
        *var_gc2_p_rv_slot = var_gc2_p_rv;
        *var_gc2ov_p_slot = var_gc2ov_p;
        *var_gc2ov_p_rv_slot = var_gc2ov_p_rv;
        *var_gc2ovd_p_slot = var_gc2ovd_p;
        *var_gc2ovd_p_rv_slot = var_gc2ovd_p_rv;
        *var_gc3_p_slot = var_gc3_p;
        *var_gc3_p_rv_slot = var_gc3_p_rv;
        *var_gc3ov_p_slot = var_gc3ov_p;
        *var_gc3ov_p_rv_slot = var_gc3ov_p_rv;
        *var_gc3ovd_p_slot = var_gc3ovd_p;
        *var_gc3ovd_p_rv_slot = var_gc3ovd_p_rv;
        *var_gco_p_slot = var_gco_p;
        *var_gco_p_rv_slot = var_gco_p_rv;
        *var_guard35_slot = var_guard35;
        *var_guard35_rv_slot = var_guard35_rv;
        *var_guard36_slot = var_guard36;
        *var_guard36_rv_slot = var_guard36_rv;
        *var_guard37_slot = var_guard37;
        *var_guard37_rv_slot = var_guard37_rv;
        *var_guard38_slot = var_guard38;
        *var_guard38_rv_slot = var_guard38_rv;
        *var_guard39_slot = var_guard39;
        *var_guard39_rv_slot = var_guard39_rv;
        *var_guard40_slot = var_guard40;
        *var_guard40_rv_slot = var_guard40_rv;
        *var_guard41_slot = var_guard41;
        *var_guard41_rv_slot = var_guard41_rv;
        *var_guard42_slot = var_guard42;
        *var_guard42_rv_slot = var_guard42_rv;
        *var_guard43_slot = var_guard43;
        *var_guard43_rv_slot = var_guard43_rv;
        *var_iginv_p_slot = var_iginv_p;
        *var_iginv_p_rv_slot = var_iginv_p_rv;
        *var_igov_p_slot = var_igov_p;
        *var_igov_p_rv_slot = var_igov_p_rv;
        *var_igovd_p_slot = var_igovd_p;
        *var_igovd_p_rv_slot = var_igovd_p_rv;
        *var_imaxii_p_slot = var_imaxii_p;
        *var_imaxii_p_rv_slot = var_imaxii_p_rv;
        *var_lpcke_slot = var_lpcke;
        *var_lpcke_rv_slot = var_lpcke_rv;
        *var_munqs_p_slot = var_munqs_p;
        *var_munqs_p_rv_slot = var_munqs_p_rv;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_neffedge_p_rv_slot = var_neffedge_p_rv;
        *var_npcke_slot = var_npcke;
        *var_npcke_rv_slot = var_npcke_rv;
        *var_nsub_slot = var_nsub;
        *var_nsub0e_slot = var_nsub0e;
        *var_nsub0e_rv_slot = var_nsub0e_rv;
        *var_nsub_rv_slot = var_nsub_rv;
        *var_pscebedge_p_slot = var_pscebedge_p;
        *var_pscebedge_p_rv_slot = var_pscebedge_p_rv;
        *var_pscededge_p_slot = var_pscededge_p;
        *var_pscededge_p_rv_slot = var_pscededge_p_rv;
        *var_psceedge_p_slot = var_psceedge_p;
        *var_psceedge_p_rv_slot = var_psceedge_p_rv;
        *var_st2vfb_p_slot = var_st2vfb_p;
        *var_st2vfb_p_rv_slot = var_st2vfb_p_rv;
        *var_sta2_p_slot = var_sta2_p;
        *var_sta2_p_rv_slot = var_sta2_p_rv;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_stbetedge_p_rv_slot = var_stbetedge_p_rv;
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidl_p_rv_slot = var_stbgidl_p_rv;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_stbgidld_p_rv_slot = var_stbgidld_p_rv;
        *var_stig_p_slot = var_stig_p;
        *var_stig_p_rv_slot = var_stig_p_rv;
        *var_stvfb_p_slot = var_stvfb_p;
        *var_stvfb_p_rv_slot = var_stvfb_p_rv;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_stvfbedge_p_rv_slot = var_stvfbedge_p_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
        *var_tox_p_slot = var_tox_p;
        *var_tox_p_rv_slot = var_tox_p_rv;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfb_p_rv_slot = var_vfb_p_rv;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vfbedge_p_rv_slot = var_vfbedge_p_rv;
    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        var_guard41: f64,
        var_guard42: f64,
        var_guard43: f64,
        var_iae: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_lpcke: f64,
        var_npcke: f64,
        var_nsub0e: f64,
        var_we: f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_rv_slot: &mut f64,
        var_cfb_p_slot: &mut f64,
        var_cfb_p_rv_slot: &mut f64,
        var_cfd_p_slot: &mut f64,
        var_cfd_p_rv_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_cs_p_rv_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ct_p_rv_slot: &mut f64,
        var_ctb_p_slot: &mut f64,
        var_ctb_p_rv_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_ctg_p_rv_slot: &mut f64,
        var_dphib_p_slot: &mut f64,
        var_dphib_p_rv_slot: &mut f64,
        var_dvsbnud_p_slot: &mut f64,
        var_dvsbnud_p_rv_slot: &mut f64,
        var_fbet1e_slot: &mut f64,
        var_fbet1e_rv_slot: &mut f64,
        var_gfacnud_p_slot: &mut f64,
        var_gfacnud_p_rv_slot: &mut f64,
        var_gpe_slot: &mut f64,
        var_gpe_rv_slot: &mut f64,
        var_gwe_slot: &mut f64,
        var_gwe_rv_slot: &mut f64,
        var_lp1e_slot: &mut f64,
        var_lp1e_rv_slot: &mut f64,
        var_mue_p_slot: &mut f64,
        var_mue_p_rv_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_neff_p_rv_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_nov_p_rv_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_novd_p_rv_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_np_p_rv_slot: &mut f64,
        var_nsub_slot: &mut f64,
        var_nsub_rv_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psce_p_rv_slot: &mut f64,
        var_psceb_p_slot: &mut f64,
        var_psceb_p_rv_slot: &mut f64,
        var_psced_p_slot: &mut f64,
        var_psced_p_rv_slot: &mut f64,
        var_stbet_p_slot: &mut f64,
        var_stbet_p_rv_slot: &mut f64,
        var_stcs_p_slot: &mut f64,
        var_stcs_p_rv_slot: &mut f64,
        var_stct_p_slot: &mut f64,
        var_stct_p_rv_slot: &mut f64,
        var_stmue_p_slot: &mut f64,
        var_stmue_p_rv_slot: &mut f64,
        var_stthecs_p_slot: &mut f64,
        var_stthecs_p_rv_slot: &mut f64,
        var_stthemu_p_slot: &mut f64,
        var_stthemu_p_rv_slot: &mut f64,
        var_stxcor_p_slot: &mut f64,
        var_stxcor_p_rv_slot: &mut f64,
        var_thecs_p_slot: &mut f64,
        var_thecs_p_rv_slot: &mut f64,
        var_themu_p_slot: &mut f64,
        var_themu_p_rv_slot: &mut f64,
        var_toxov_p_slot: &mut f64,
        var_toxov_p_rv_slot: &mut f64,
        var_toxovd_p_slot: &mut f64,
        var_toxovd_p_rv_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
        var_vsbnud_p_rv_slot: &mut f64,
        var_xcor_p_slot: &mut f64,
        var_xcor_p_rv_slot: &mut f64,
    ) {
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_rv: f64 = *var_cf_p_rv_slot;
        let mut var_cfb_p: f64 = *var_cfb_p_slot;
        let mut var_cfb_p_rv: f64 = *var_cfb_p_rv_slot;
        let mut var_cfd_p: f64 = *var_cfd_p_slot;
        let mut var_cfd_p_rv: f64 = *var_cfd_p_rv_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_cs_p_rv: f64 = *var_cs_p_rv_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ct_p_rv: f64 = *var_ct_p_rv_slot;
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctb_p_rv: f64 = *var_ctb_p_rv_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_ctg_p_rv: f64 = *var_ctg_p_rv_slot;
        let mut var_dphib_p: f64 = *var_dphib_p_slot;
        let mut var_dphib_p_rv: f64 = *var_dphib_p_rv_slot;
        let mut var_dvsbnud_p: f64 = *var_dvsbnud_p_slot;
        let mut var_dvsbnud_p_rv: f64 = *var_dvsbnud_p_rv_slot;
        let mut var_fbet1e: f64 = *var_fbet1e_slot;
        let mut var_fbet1e_rv: f64 = *var_fbet1e_rv_slot;
        let mut var_gfacnud_p: f64 = *var_gfacnud_p_slot;
        let mut var_gfacnud_p_rv: f64 = *var_gfacnud_p_rv_slot;
        let mut var_gpe: f64 = *var_gpe_slot;
        let mut var_gpe_rv: f64 = *var_gpe_rv_slot;
        let mut var_gwe: f64 = *var_gwe_slot;
        let mut var_gwe_rv: f64 = *var_gwe_rv_slot;
        let mut var_lp1e: f64 = *var_lp1e_slot;
        let mut var_lp1e_rv: f64 = *var_lp1e_rv_slot;
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_mue_p_rv: f64 = *var_mue_p_rv_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_neff_p_rv: f64 = *var_neff_p_rv_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_nov_p_rv: f64 = *var_nov_p_rv_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_novd_p_rv: f64 = *var_novd_p_rv_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_np_p_rv: f64 = *var_np_p_rv_slot;
        let mut var_nsub: f64 = *var_nsub_slot;
        let mut var_nsub_rv: f64 = *var_nsub_rv_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psce_p_rv: f64 = *var_psce_p_rv_slot;
        let mut var_psceb_p: f64 = *var_psceb_p_slot;
        let mut var_psceb_p_rv: f64 = *var_psceb_p_rv_slot;
        let mut var_psced_p: f64 = *var_psced_p_slot;
        let mut var_psced_p_rv: f64 = *var_psced_p_rv_slot;
        let mut var_stbet_p: f64 = *var_stbet_p_slot;
        let mut var_stbet_p_rv: f64 = *var_stbet_p_rv_slot;
        let mut var_stcs_p: f64 = *var_stcs_p_slot;
        let mut var_stcs_p_rv: f64 = *var_stcs_p_rv_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stct_p_rv: f64 = *var_stct_p_rv_slot;
        let mut var_stmue_p: f64 = *var_stmue_p_slot;
        let mut var_stmue_p_rv: f64 = *var_stmue_p_rv_slot;
        let mut var_stthecs_p: f64 = *var_stthecs_p_slot;
        let mut var_stthecs_p_rv: f64 = *var_stthecs_p_rv_slot;
        let mut var_stthemu_p: f64 = *var_stthemu_p_slot;
        let mut var_stthemu_p_rv: f64 = *var_stthemu_p_rv_slot;
        let mut var_stxcor_p: f64 = *var_stxcor_p_slot;
        let mut var_stxcor_p_rv: f64 = *var_stxcor_p_rv_slot;
        let mut var_thecs_p: f64 = *var_thecs_p_slot;
        let mut var_thecs_p_rv: f64 = *var_thecs_p_rv_slot;
        let mut var_themu_p: f64 = *var_themu_p_slot;
        let mut var_themu_p_rv: f64 = *var_themu_p_rv_slot;
        let mut var_toxov_p: f64 = *var_toxov_p_slot;
        let mut var_toxov_p_rv: f64 = *var_toxov_p_rv_slot;
        let mut var_toxovd_p: f64 = *var_toxovd_p_slot;
        let mut var_toxovd_p_rv: f64 = *var_toxovd_p_rv_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;
        let mut var_vsbnud_p_rv: f64 = *var_vsbnud_p_rv_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;
        let mut var_xcor_p_rv: f64 = *var_xcor_p_rv_slot;

        let (assign5550_e4302,) = {
    if (((var_guard41 != 0.0) && (var_guard42 == 0.0)) && (var_guard43 == 0.0)) {
        let assign5550_e4297: f64 = (var_le / var_lpcke);
        let assign5550_e4298: f64 = (2.0 - assign5550_e4297);
        let assign5550_e4299: f64 = (var_npcke * assign5550_e4298);
        let assign5550_e4300: f64 = (var_nsub0e + assign5550_e4299);
        (assign5550_e4300,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5550_e4302;
        var_nsub_rv = 0.0;

        let (assign5560_e4316,) = {
    if (var_guard41 != 0.0) {
        let assign5560_e4308: f64 = (p.p218 * var_ile);
        let assign5560_e4309: f64 = (1.0 - assign5560_e4308);
        let assign5560_e4312: f64 = (p.p219 * var_ile2);
        let assign5560_e4313: f64 = (assign5560_e4309 - assign5560_e4312);
        let assign5560_e4314: f64 = (var_nsub * assign5560_e4313);
        (assign5560_e4314,)
    } else {
        (var_neff_p,)
    }
};
        var_neff_p = assign5560_e4316;
        var_neff_p_rv = 0.0;

        let (assign5570_e4334,) = {
    if (var_guard41 != 0.0) {
        let assign5570_e4322: f64 = (var_ile).powf(p.p222);
        let assign5570_e4323: f64 = (p.p221 * assign5570_e4322);
        let assign5570_e4324: f64 = (p.p220 + assign5570_e4323);
        let assign5570_e4327: f64 = (p.p223 * var_iwe);
        let assign5570_e4328: f64 = (assign5570_e4324 + assign5570_e4327);
        let assign5570_e4331: f64 = (p.p224 * var_iae);
        let assign5570_e4332: f64 = (assign5570_e4328 + assign5570_e4331);
        (assign5570_e4332,)
    } else {
        (var_gfacnud_p,)
    }
};
        var_gfacnud_p = assign5570_e4334;
        var_gfacnud_p_rv = 0.0;

        let (assign5580_e4338,) = {
    if (var_guard41 != 0.0) {
        (p.p225,)
    } else {
        (var_vsbnud_p,)
    }
};
        var_vsbnud_p = assign5580_e4338;
        var_vsbnud_p_rv = 0.0;

        let (assign5590_e4342,) = {
    if (var_guard41 != 0.0) {
        (p.p226,)
    } else {
        (var_dvsbnud_p,)
    }
};
        var_dvsbnud_p = assign5590_e4342;
        var_dvsbnud_p_rv = 0.0;

        let (assign5600_e4360,) = {
    if (var_guard41 != 0.0) {
        let assign5600_e4348: f64 = (var_ile).powf(p.p229);
        let assign5600_e4349: f64 = (p.p228 * assign5600_e4348);
        let assign5600_e4350: f64 = (p.p227 + assign5600_e4349);
        let assign5600_e4353: f64 = (p.p230 * var_iwe);
        let assign5600_e4354: f64 = (assign5600_e4350 + assign5600_e4353);
        let assign5600_e4357: f64 = (p.p231 * var_iae);
        let assign5600_e4358: f64 = (assign5600_e4354 + assign5600_e4357);
        (assign5600_e4358,)
    } else {
        (var_dphib_p,)
    }
};
        var_dphib_p = assign5600_e4360;
        var_dphib_p_rv = 0.0;

        let (assign5610_e4379,) = {
    if (var_guard41 != 0.0) {
        let assign5610_e4367: f64 = (p.p233 * var_ile);
        let assign5610_e4368: f64 = (1.0 + assign5610_e4367);
        let (assign5610_e4376,) = {
            if (1e-6 > assign5610_e4368) {
                (1e-6,)
            } else {
                let assign5610_e4374: f64 = (p.p233 * var_ile);
                let assign5610_e4375: f64 = (1.0 + assign5610_e4374);
                (assign5610_e4375,)
            }
        };
        let assign5610_e4377: f64 = (p.p232 * assign5610_e4376);
        (assign5610_e4377,)
    } else {
        (var_np_p,)
    }
};
        var_np_p = assign5610_e4379;
        var_np_p_rv = 0.0;

        let (assign5620_e4383,) = {
    if (var_guard41 != 0.0) {
        (p.p234,)
    } else {
        (var_toxov_p,)
    }
};
        var_toxov_p = assign5620_e4383;
        var_toxov_p_rv = 0.0;

        let (assign5630_e4387,) = {
    if (var_guard41 != 0.0) {
        (p.p235,)
    } else {
        (var_toxovd_p,)
    }
};
        var_toxovd_p = assign5630_e4387;
        var_toxovd_p_rv = 0.0;

        let (assign5640_e4391,) = {
    if (var_guard41 != 0.0) {
        (p.p238,)
    } else {
        (var_nov_p,)
    }
};
        var_nov_p = assign5640_e4391;
        var_nov_p_rv = 0.0;

        let (assign5650_e4395,) = {
    if (var_guard41 != 0.0) {
        (p.p239,)
    } else {
        (var_novd_p,)
    }
};
        var_novd_p = assign5650_e4395;
        var_novd_p_rv = 0.0;

        let (assign5660_e4417,) = {
    if (var_guard41 != 0.0) {
        let assign5660_e4401: f64 = (var_ile).powf(p.p242);
        let assign5660_e4402: f64 = (p.p241 * assign5660_e4401);
        let assign5660_e4403: f64 = (p.p240 + assign5660_e4402);
        let assign5660_e4407: f64 = (p.p243 * var_iwe);
        let assign5660_e4408: f64 = (1.0 + assign5660_e4407);
        let assign5660_e4409: f64 = (assign5660_e4403 * assign5660_e4408);
        let assign5660_e4413: f64 = (p.p244 * var_iae);
        let assign5660_e4414: f64 = (1.0 + assign5660_e4413);
        let assign5660_e4415: f64 = (assign5660_e4409 * assign5660_e4414);
        (assign5660_e4415,)
    } else {
        (var_ct_p,)
    }
};
        var_ct_p = assign5660_e4417;
        var_ct_p_rv = 0.0;

        let (assign5670_e4421,) = {
    if (var_guard41 != 0.0) {
        (p.p246,)
    } else {
        (var_ctg_p,)
    }
};
        var_ctg_p = assign5670_e4421;
        var_ctg_p_rv = 0.0;

        let (assign5680_e4425,) = {
    if (var_guard41 != 0.0) {
        (p.p245,)
    } else {
        (var_ctb_p,)
    }
};
        var_ctb_p = assign5680_e4425;
        var_ctb_p_rv = 0.0;

        let (assign5690_e4429,) = {
    if (var_guard41 != 0.0) {
        (p.p247,)
    } else {
        (var_stct_p,)
    }
};
        var_stct_p = assign5690_e4429;
        var_stct_p_rv = 0.0;

        let (assign5700_e4443,) = {
    if (var_guard41 != 0.0) {
        let assign5700_e4434: f64 = (var_ile).powf(p.p249);
        let assign5700_e4435: f64 = (p.p248 * assign5700_e4434);
        let assign5700_e4439: f64 = (p.p250 * var_iwe);
        let assign5700_e4440: f64 = (1.0 + assign5700_e4439);
        let assign5700_e4441: f64 = (assign5700_e4435 * assign5700_e4440);
        (assign5700_e4441,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign5700_e4443;
        var_cf_p_rv = 0.0;

        let (assign5710_e4447,) = {
    if (var_guard41 != 0.0) {
        (p.p252,)
    } else {
        (var_cfd_p,)
    }
};
        var_cfd_p = assign5710_e4447;
        var_cfd_p_rv = 0.0;

        let (assign5720_e4451,) = {
    if (var_guard41 != 0.0) {
        (p.p251,)
    } else {
        (var_cfb_p,)
    }
};
        var_cfb_p = assign5720_e4451;
        var_cfb_p_rv = 0.0;

        let (assign5730_e4465,) = {
    if (var_guard41 != 0.0) {
        let assign5730_e4456: f64 = (var_ile).powf(p.p254);
        let assign5730_e4457: f64 = (p.p253 * assign5730_e4456);
        let assign5730_e4461: f64 = (p.p255 * var_iwe);
        let assign5730_e4462: f64 = (1.0 + assign5730_e4461);
        let assign5730_e4463: f64 = (assign5730_e4457 * assign5730_e4462);
        (assign5730_e4463,)
    } else {
        (var_psce_p,)
    }
};
        var_psce_p = assign5730_e4465;
        var_psce_p_rv = 0.0;

        let (assign5740_e4469,) = {
    if (var_guard41 != 0.0) {
        (p.p257,)
    } else {
        (var_psced_p,)
    }
};
        var_psced_p = assign5740_e4469;
        var_psced_p_rv = 0.0;

        let (assign5750_e4473,) = {
    if (var_guard41 != 0.0) {
        (p.p256,)
    } else {
        (var_psceb_p,)
    }
};
        var_psceb_p = assign5750_e4473;
        var_psceb_p_rv = 0.0;

        let (assign5760_e4483,) = {
    if (var_guard41 != 0.0) {
        let assign5760_e4479: f64 = (p.p260 * var_iwe);
        let assign5760_e4480: f64 = (1.0 + assign5760_e4479);
        let assign5760_e4481: f64 = (p.p259 * assign5760_e4480);
        (assign5760_e4481,)
    } else {
        (var_fbet1e,)
    }
};
        var_fbet1e = assign5760_e4483;
        var_fbet1e_rv = 0.0;

        let (assign5770_e4502,) = {
    if (var_guard41 != 0.0) {
        let assign5770_e4489: f64 = (p.p262 * var_iwe);
        let assign5770_e4490: f64 = (1.0 + assign5770_e4489);
        let (assign5770_e4499,) = {
            if (assign5770_e4490 > 0.001) {
                let assign5770_e4496: f64 = (p.p262 * var_iwe);
                let assign5770_e4497: f64 = (1.0 + assign5770_e4496);
                (assign5770_e4497,)
            } else {
                (0.001,)
            }
        };
        let assign5770_e4500: f64 = (p.p261 * assign5770_e4499);
        (assign5770_e4500,)
    } else {
        (var_lp1e,)
    }
};
        var_lp1e = assign5770_e4502;
        var_lp1e_rv = 0.0;

        let (assign5780_e4534,) = {
    if (var_guard41 != 0.0) {
        let assign5780_e4507: f64 = (var_fbet1e * var_lp1e);
        let assign5780_e4509: f64 = (assign5780_e4507 / var_le);
        let assign5780_e4512: f64 = (-var_le);
        let assign5780_e4514: f64 = (assign5780_e4512 / var_lp1e);
        let assign5780_e4515: f64 = (assign5780_e4514).exp();
        let assign5780_e4516: f64 = (1.0 - assign5780_e4515);
        let assign5780_e4517: f64 = (assign5780_e4509 * assign5780_e4516);
        let assign5780_e4518: f64 = (1.0 + assign5780_e4517);
        let assign5780_e4521: f64 = (p.p263 * p.p264);
        let assign5780_e4523: f64 = (assign5780_e4521 / var_le);
        let assign5780_e4526: f64 = (-var_le);
        let assign5780_e4528: f64 = (assign5780_e4526 / p.p264);
        let assign5780_e4529: f64 = (assign5780_e4528).exp();
        let assign5780_e4530: f64 = (1.0 - assign5780_e4529);
        let assign5780_e4531: f64 = (assign5780_e4523 * assign5780_e4530);
        let assign5780_e4532: f64 = (assign5780_e4518 + assign5780_e4531);
        (assign5780_e4532,)
    } else {
        (var_gpe,)
    }
};
        var_gpe = assign5780_e4534;
        var_gpe_rv = 0.0;

        let (assign5790_e4543,) = {
    if (var_guard41 != 0.0) {
        let (assign5790_e4541,) = {
            if (var_gpe > 1e-15) {
                (var_gpe,)
            } else {
                (1e-15,)
            }
        };
        (assign5790_e4541,)
    } else {
        (var_gpe,)
    }
};
        var_gpe = assign5790_e4543;
        var_gpe_rv = 0.0;

        let (assign5800_e4562,) = {
    if (var_guard41 != 0.0) {
        let assign5800_e4548: f64 = (p.p265 * var_iwe);
        let assign5800_e4549: f64 = (1.0 + assign5800_e4548);
        let assign5800_e4552: f64 = (p.p266 * var_iwe);
        let assign5800_e4556: f64 = (var_we / p.p267);
        let assign5800_e4557: f64 = (1.0 + assign5800_e4556);
        let assign5800_e4558: f64 = (assign5800_e4557).ln();
        let assign5800_e4559: f64 = (assign5800_e4552 * assign5800_e4558);
        let assign5800_e4560: f64 = (assign5800_e4549 + assign5800_e4559);
        (assign5800_e4560,)
    } else {
        (var_gwe,)
    }
};
        var_gwe = assign5800_e4562;
        var_gwe_rv = 0.0;

        let (assign5810_e4574,) = {
    if (var_guard41 != 0.0) {
        let assign5810_e4566: f64 = (p.p258 * var_we);
        let assign5810_e4569: f64 = (var_gpe * var_le);
        let assign5810_e4570: f64 = (assign5810_e4566 / assign5810_e4569);
        let assign5810_e4572: f64 = (assign5810_e4570 * var_gwe);
        (assign5810_e4572,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign5810_e4574;
        var_betn_p_rv = 0.0;

        let (assign5820_e4590,) = {
    if (var_guard41 != 0.0) {
        let assign5820_e4579: f64 = (p.p269 * var_ile);
        let assign5820_e4580: f64 = (p.p268 + assign5820_e4579);
        let assign5820_e4583: f64 = (p.p270 * var_iwe);
        let assign5820_e4584: f64 = (assign5820_e4580 + assign5820_e4583);
        let assign5820_e4587: f64 = (p.p271 * var_iae);
        let assign5820_e4588: f64 = (assign5820_e4584 + assign5820_e4587);
        (assign5820_e4588,)
    } else {
        (var_stbet_p,)
    }
};
        var_stbet_p = assign5820_e4590;
        var_stbet_p_rv = 0.0;

        let (assign5830_e4600,) = {
    if (var_guard41 != 0.0) {
        let assign5830_e4596: f64 = (p.p273 * var_iwe);
        let assign5830_e4597: f64 = (1.0 + assign5830_e4596);
        let assign5830_e4598: f64 = (p.p272 * assign5830_e4597);
        (assign5830_e4598,)
    } else {
        (var_mue_p,)
    }
};
        var_mue_p = assign5830_e4600;
        var_mue_p_rv = 0.0;

        let (assign5840_e4604,) = {
    if (var_guard41 != 0.0) {
        (p.p274,)
    } else {
        (var_stmue_p,)
    }
};
        var_stmue_p = assign5840_e4604;
        var_stmue_p_rv = 0.0;

        let (assign5850_e4608,) = {
    if (var_guard41 != 0.0) {
        (p.p275,)
    } else {
        (var_themu_p,)
    }
};
        var_themu_p = assign5850_e4608;
        var_themu_p_rv = 0.0;

        let (assign5860_e4612,) = {
    if (var_guard41 != 0.0) {
        (p.p276,)
    } else {
        (var_stthemu_p,)
    }
};
        var_stthemu_p = assign5860_e4612;
        var_stthemu_p_rv = 0.0;

        let (assign5870_e4634,) = {
    if (var_guard41 != 0.0) {
        let assign5870_e4618: f64 = (var_ile).powf(p.p279);
        let assign5870_e4619: f64 = (p.p278 * assign5870_e4618);
        let assign5870_e4620: f64 = (p.p277 + assign5870_e4619);
        let assign5870_e4624: f64 = (p.p280 * var_iwe);
        let assign5870_e4625: f64 = (1.0 + assign5870_e4624);
        let assign5870_e4626: f64 = (assign5870_e4620 * assign5870_e4625);
        let assign5870_e4630: f64 = (p.p281 * var_iae);
        let assign5870_e4631: f64 = (1.0 + assign5870_e4630);
        let assign5870_e4632: f64 = (assign5870_e4626 * assign5870_e4631);
        (assign5870_e4632,)
    } else {
        (var_cs_p,)
    }
};
        var_cs_p = assign5870_e4634;
        var_cs_p_rv = 0.0;

        let (assign5880_e4638,) = {
    if (var_guard41 != 0.0) {
        (p.p282,)
    } else {
        (var_stcs_p,)
    }
};
        var_stcs_p = assign5880_e4638;
        var_stcs_p_rv = 0.0;

        let (assign5890_e4642,) = {
    if (var_guard41 != 0.0) {
        (p.p283,)
    } else {
        (var_thecs_p,)
    }
};
        var_thecs_p = assign5890_e4642;
        var_thecs_p_rv = 0.0;

        let (assign5900_e4646,) = {
    if (var_guard41 != 0.0) {
        (p.p284,)
    } else {
        (var_stthecs_p,)
    }
};
        var_stthecs_p = assign5900_e4646;
        var_stthecs_p_rv = 0.0;

        let (assign5910_e4668,) = {
    if (var_guard41 != 0.0) {
        let assign5910_e4652: f64 = (p.p286 * var_ile);
        let assign5910_e4653: f64 = (1.0 + assign5910_e4652);
        let assign5910_e4654: f64 = (p.p285 * assign5910_e4653);
        let assign5910_e4658: f64 = (p.p287 * var_iwe);
        let assign5910_e4659: f64 = (1.0 + assign5910_e4658);
        let assign5910_e4660: f64 = (assign5910_e4654 * assign5910_e4659);
        let assign5910_e4664: f64 = (p.p288 * var_iae);
        let assign5910_e4665: f64 = (1.0 + assign5910_e4664);
        let assign5910_e4666: f64 = (assign5910_e4660 * assign5910_e4665);
        (assign5910_e4666,)
    } else {
        (var_xcor_p,)
    }
};
        var_xcor_p = assign5910_e4668;
        var_xcor_p_rv = 0.0;

        let (assign5920_e4672,) = {
    if (var_guard41 != 0.0) {
        (p.p289,)
    } else {
        (var_stxcor_p,)
    }
};
        var_stxcor_p = assign5920_e4672;
        var_stxcor_p_rv = 0.0;

        *var_betn_p_slot = var_betn_p;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_rv_slot = var_cf_p_rv;
        *var_cfb_p_slot = var_cfb_p;
        *var_cfb_p_rv_slot = var_cfb_p_rv;
        *var_cfd_p_slot = var_cfd_p;
        *var_cfd_p_rv_slot = var_cfd_p_rv;
        *var_cs_p_slot = var_cs_p;
        *var_cs_p_rv_slot = var_cs_p_rv;
        *var_ct_p_slot = var_ct_p;
        *var_ct_p_rv_slot = var_ct_p_rv;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctb_p_rv_slot = var_ctb_p_rv;
        *var_ctg_p_slot = var_ctg_p;
        *var_ctg_p_rv_slot = var_ctg_p_rv;
        *var_dphib_p_slot = var_dphib_p;
        *var_dphib_p_rv_slot = var_dphib_p_rv;
        *var_dvsbnud_p_slot = var_dvsbnud_p;
        *var_dvsbnud_p_rv_slot = var_dvsbnud_p_rv;
        *var_fbet1e_slot = var_fbet1e;
        *var_fbet1e_rv_slot = var_fbet1e_rv;
        *var_gfacnud_p_slot = var_gfacnud_p;
        *var_gfacnud_p_rv_slot = var_gfacnud_p_rv;
        *var_gpe_slot = var_gpe;
        *var_gpe_rv_slot = var_gpe_rv;
        *var_gwe_slot = var_gwe;
        *var_gwe_rv_slot = var_gwe_rv;
        *var_lp1e_slot = var_lp1e;
        *var_lp1e_rv_slot = var_lp1e_rv;
        *var_mue_p_slot = var_mue_p;
        *var_mue_p_rv_slot = var_mue_p_rv;
        *var_neff_p_slot = var_neff_p;
        *var_neff_p_rv_slot = var_neff_p_rv;
        *var_nov_p_slot = var_nov_p;
        *var_nov_p_rv_slot = var_nov_p_rv;
        *var_novd_p_slot = var_novd_p;
        *var_novd_p_rv_slot = var_novd_p_rv;
        *var_np_p_slot = var_np_p;
        *var_np_p_rv_slot = var_np_p_rv;
        *var_nsub_slot = var_nsub;
        *var_nsub_rv_slot = var_nsub_rv;
        *var_psce_p_slot = var_psce_p;
        *var_psce_p_rv_slot = var_psce_p_rv;
        *var_psceb_p_slot = var_psceb_p;
        *var_psceb_p_rv_slot = var_psceb_p_rv;
        *var_psced_p_slot = var_psced_p;
        *var_psced_p_rv_slot = var_psced_p_rv;
        *var_stbet_p_slot = var_stbet_p;
        *var_stbet_p_rv_slot = var_stbet_p_rv;
        *var_stcs_p_slot = var_stcs_p;
        *var_stcs_p_rv_slot = var_stcs_p_rv;
        *var_stct_p_slot = var_stct_p;
        *var_stct_p_rv_slot = var_stct_p_rv;
        *var_stmue_p_slot = var_stmue_p;
        *var_stmue_p_rv_slot = var_stmue_p_rv;
        *var_stthecs_p_slot = var_stthecs_p;
        *var_stthecs_p_rv_slot = var_stthecs_p_rv;
        *var_stthemu_p_slot = var_stthemu_p;
        *var_stthemu_p_rv_slot = var_stthemu_p_rv;
        *var_stxcor_p_slot = var_stxcor_p;
        *var_stxcor_p_rv_slot = var_stxcor_p_rv;
        *var_thecs_p_slot = var_thecs_p;
        *var_thecs_p_rv_slot = var_thecs_p_rv;
        *var_themu_p_slot = var_themu_p;
        *var_themu_p_rv_slot = var_themu_p_rv;
        *var_toxov_p_slot = var_toxov_p;
        *var_toxov_p_rv_slot = var_toxov_p_rv;
        *var_toxovd_p_slot = var_toxovd_p;
        *var_toxovd_p_rv_slot = var_toxovd_p_rv;
        *var_vsbnud_p_slot = var_vsbnud_p;
        *var_vsbnud_p_rv_slot = var_vsbnud_p_rv;
        *var_xcor_p_slot = var_xcor_p;
        *var_xcor_p_rv_slot = var_xcor_p_rv;
    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_gpe: f64,
        var_guard41: f64,
        var_gwe: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_a1_p_slot: &mut f64,
        var_a1_p_rv_slot: &mut f64,
        var_a2_p_slot: &mut f64,
        var_a2_p_rv_slot: &mut f64,
        var_a3_p_slot: &mut f64,
        var_a3_p_rv_slot: &mut f64,
        var_a4_p_slot: &mut f64,
        var_a4_p_rv_slot: &mut f64,
        var_agidl_p_slot: &mut f64,
        var_agidl_p_rv_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_agidld_p_rv_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp1_p_rv_slot: &mut f64,
        var_alp2_p_slot: &mut f64,
        var_alp2_p_rv_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_alp_p_rv_slot: &mut f64,
        var_ax_p_slot: &mut f64,
        var_ax_p_rv_slot: &mut f64,
        var_chib_p_slot: &mut f64,
        var_chib_p_rv_slot: &mut f64,
        var_feta_p_slot: &mut f64,
        var_feta_p_rv_slot: &mut f64,
        var_gc2_p_slot: &mut f64,
        var_gc2_p_rv_slot: &mut f64,
        var_gc2ov_p_slot: &mut f64,
        var_gc2ov_p_rv_slot: &mut f64,
        var_gc2ovd_p_slot: &mut f64,
        var_gc2ovd_p_rv_slot: &mut f64,
        var_gc3_p_slot: &mut f64,
        var_gc3_p_rv_slot: &mut f64,
        var_gc3ov_p_slot: &mut f64,
        var_gc3ov_p_rv_slot: &mut f64,
        var_gc3ovd_p_slot: &mut f64,
        var_gc3ovd_p_rv_slot: &mut f64,
        var_gco_p_slot: &mut f64,
        var_gco_p_rv_slot: &mut f64,
        var_guard44_slot: &mut f64,
        var_guard44_rv_slot: &mut f64,
        var_guard45_slot: &mut f64,
        var_guard45_rv_slot: &mut f64,
        var_guard46_slot: &mut f64,
        var_guard46_rv_slot: &mut f64,
        var_guard47_slot: &mut f64,
        var_guard47_rv_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_iginv_p_rv_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igov_p_rv_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_igovd_p_rv_slot: &mut f64,
        var_imaxii_p_slot: &mut f64,
        var_imaxii_p_rv_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rs_p_rv_slot: &mut f64,
        var_rsb_p_slot: &mut f64,
        var_rsb_p_rv_slot: &mut f64,
        var_rsg_p_slot: &mut f64,
        var_rsg_p_rv_slot: &mut f64,
        var_sta2_p_slot: &mut f64,
        var_sta2_p_rv_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_stig_p_rv_slot: &mut f64,
        var_strs_p_slot: &mut f64,
        var_strs_p_rv_slot: &mut f64,
        var_stthesat_p_slot: &mut f64,
        var_stthesat_p_rv_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_rv_slot: &mut f64,
        var_thesatb_p_slot: &mut f64,
        var_thesatb_p_rv_slot: &mut f64,
        var_thesatg_p_slot: &mut f64,
        var_thesatg_p_rv_slot: &mut f64,
        var_thesatt_p_slot: &mut f64,
        var_thesatt_p_rv_slot: &mut f64,
        var_tmpx_slot: &mut f64,
        var_tmpx_rv_slot: &mut f64,
        var_vp_p_slot: &mut f64,
        var_vp_p_rv_slot: &mut f64,
    ) {
        let mut var_a1_p: f64 = *var_a1_p_slot;
        let mut var_a1_p_rv: f64 = *var_a1_p_rv_slot;
        let mut var_a2_p: f64 = *var_a2_p_slot;
        let mut var_a2_p_rv: f64 = *var_a2_p_rv_slot;
        let mut var_a3_p: f64 = *var_a3_p_slot;
        let mut var_a3_p_rv: f64 = *var_a3_p_rv_slot;
        let mut var_a4_p: f64 = *var_a4_p_slot;
        let mut var_a4_p_rv: f64 = *var_a4_p_rv_slot;
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidl_p_rv: f64 = *var_agidl_p_rv_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_agidld_p_rv: f64 = *var_agidld_p_rv_slot;
        let mut var_alp1_p: f64 = *var_alp1_p_slot;
        let mut var_alp1_p_rv: f64 = *var_alp1_p_rv_slot;
        let mut var_alp2_p: f64 = *var_alp2_p_slot;
        let mut var_alp2_p_rv: f64 = *var_alp2_p_rv_slot;
        let mut var_alp_p: f64 = *var_alp_p_slot;
        let mut var_alp_p_rv: f64 = *var_alp_p_rv_slot;
        let mut var_ax_p: f64 = *var_ax_p_slot;
        let mut var_ax_p_rv: f64 = *var_ax_p_rv_slot;
        let mut var_chib_p: f64 = *var_chib_p_slot;
        let mut var_chib_p_rv: f64 = *var_chib_p_rv_slot;
        let mut var_feta_p: f64 = *var_feta_p_slot;
        let mut var_feta_p_rv: f64 = *var_feta_p_rv_slot;
        let mut var_gc2_p: f64 = *var_gc2_p_slot;
        let mut var_gc2_p_rv: f64 = *var_gc2_p_rv_slot;
        let mut var_gc2ov_p: f64 = *var_gc2ov_p_slot;
        let mut var_gc2ov_p_rv: f64 = *var_gc2ov_p_rv_slot;
        let mut var_gc2ovd_p: f64 = *var_gc2ovd_p_slot;
        let mut var_gc2ovd_p_rv: f64 = *var_gc2ovd_p_rv_slot;
        let mut var_gc3_p: f64 = *var_gc3_p_slot;
        let mut var_gc3_p_rv: f64 = *var_gc3_p_rv_slot;
        let mut var_gc3ov_p: f64 = *var_gc3ov_p_slot;
        let mut var_gc3ov_p_rv: f64 = *var_gc3ov_p_rv_slot;
        let mut var_gc3ovd_p: f64 = *var_gc3ovd_p_slot;
        let mut var_gc3ovd_p_rv: f64 = *var_gc3ovd_p_rv_slot;
        let mut var_gco_p: f64 = *var_gco_p_slot;
        let mut var_gco_p_rv: f64 = *var_gco_p_rv_slot;
        let mut var_guard44: f64 = *var_guard44_slot;
        let mut var_guard44_rv: f64 = *var_guard44_rv_slot;
        let mut var_guard45: f64 = *var_guard45_slot;
        let mut var_guard45_rv: f64 = *var_guard45_rv_slot;
        let mut var_guard46: f64 = *var_guard46_slot;
        let mut var_guard46_rv: f64 = *var_guard46_rv_slot;
        let mut var_guard47: f64 = *var_guard47_slot;
        let mut var_guard47_rv: f64 = *var_guard47_rv_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_iginv_p_rv: f64 = *var_iginv_p_rv_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igov_p_rv: f64 = *var_igov_p_rv_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_igovd_p_rv: f64 = *var_igovd_p_rv_slot;
        let mut var_imaxii_p: f64 = *var_imaxii_p_slot;
        let mut var_imaxii_p_rv: f64 = *var_imaxii_p_rv_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rs_p_rv: f64 = *var_rs_p_rv_slot;
        let mut var_rsb_p: f64 = *var_rsb_p_slot;
        let mut var_rsb_p_rv: f64 = *var_rsb_p_rv_slot;
        let mut var_rsg_p: f64 = *var_rsg_p_slot;
        let mut var_rsg_p_rv: f64 = *var_rsg_p_rv_slot;
        let mut var_sta2_p: f64 = *var_sta2_p_slot;
        let mut var_sta2_p_rv: f64 = *var_sta2_p_rv_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_stig_p_rv: f64 = *var_stig_p_rv_slot;
        let mut var_strs_p: f64 = *var_strs_p_slot;
        let mut var_strs_p_rv: f64 = *var_strs_p_rv_slot;
        let mut var_stthesat_p: f64 = *var_stthesat_p_slot;
        let mut var_stthesat_p_rv: f64 = *var_stthesat_p_rv_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_rv: f64 = *var_thesat_p_rv_slot;
        let mut var_thesatb_p: f64 = *var_thesatb_p_slot;
        let mut var_thesatb_p_rv: f64 = *var_thesatb_p_rv_slot;
        let mut var_thesatg_p: f64 = *var_thesatg_p_slot;
        let mut var_thesatg_p_rv: f64 = *var_thesatg_p_rv_slot;
        let mut var_thesatt_p: f64 = *var_thesatt_p_slot;
        let mut var_thesatt_p_rv: f64 = *var_thesatt_p_rv_slot;
        let mut var_tmpx: f64 = *var_tmpx_slot;
        let mut var_tmpx_rv: f64 = *var_tmpx_rv_slot;
        let mut var_vp_p: f64 = *var_vp_p_slot;
        let mut var_vp_p_rv: f64 = *var_vp_p_rv_slot;

        let (assign5930_e4676,) = {
    if (var_guard41 != 0.0) {
        (p.p290,)
    } else {
        (var_feta_p,)
    }
};
        var_feta_p = assign5930_e4676;
        var_feta_p_rv = 0.0;

        let (assign5940_e4688,) = {
    if (var_guard41 != 0.0) {
        let assign5940_e4680: f64 = (p.p291 * var_iwe);
        let assign5940_e4684: f64 = (p.p292 * var_iwe);
        let assign5940_e4685: f64 = (1.0 + assign5940_e4684);
        let assign5940_e4686: f64 = (assign5940_e4680 * assign5940_e4685);
        (assign5940_e4686,)
    } else {
        (var_rs_p,)
    }
};
        var_rs_p = assign5940_e4688;
        var_rs_p_rv = 0.0;

        let (assign5950_e4692,) = {
    if (var_guard41 != 0.0) {
        (p.p293,)
    } else {
        (var_strs_p,)
    }
};
        var_strs_p = assign5950_e4692;
        var_strs_p_rv = 0.0;

        let (assign5960_e4696,) = {
    if (var_guard41 != 0.0) {
        (p.p294,)
    } else {
        (var_rsb_p,)
    }
};
        var_rsb_p = assign5960_e4696;
        var_rsb_p_rv = 0.0;

        let (assign5970_e4700,) = {
    if (var_guard41 != 0.0) {
        (p.p295,)
    } else {
        (var_rsg_p,)
    }
};
        var_rsg_p = assign5970_e4700;
        var_rsg_p_rv = 0.0;

        let (assign5980_e4726,) = {
    if (var_guard41 != 0.0) {
        let assign5980_e4705: f64 = (p.p297 * var_gwe);
        let assign5980_e4707: f64 = (assign5980_e4705 / var_gpe);
        let assign5980_e4710: f64 = (var_ile).powf(p.p298);
        let assign5980_e4711: f64 = (assign5980_e4707 * assign5980_e4710);
        let assign5980_e4712: f64 = (p.p296 + assign5980_e4711);
        let assign5980_e4716: f64 = (p.p299 * var_iwe);
        let assign5980_e4717: f64 = (1.0 + assign5980_e4716);
        let assign5980_e4718: f64 = (assign5980_e4712 * assign5980_e4717);
        let assign5980_e4722: f64 = (p.p300 * var_iae);
        let assign5980_e4723: f64 = (1.0 + assign5980_e4722);
        let assign5980_e4724: f64 = (assign5980_e4718 * assign5980_e4723);
        (assign5980_e4724,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign5980_e4726;
        var_thesat_p_rv = 0.0;

        let (assign5990_e4742,) = {
    if (var_guard41 != 0.0) {
        let assign5990_e4731: f64 = (p.p302 * var_ile);
        let assign5990_e4732: f64 = (p.p301 + assign5990_e4731);
        let assign5990_e4735: f64 = (p.p303 * var_iwe);
        let assign5990_e4736: f64 = (assign5990_e4732 + assign5990_e4735);
        let assign5990_e4739: f64 = (p.p304 * var_iae);
        let assign5990_e4740: f64 = (assign5990_e4736 + assign5990_e4739);
        (assign5990_e4740,)
    } else {
        (var_stthesat_p,)
    }
};
        var_stthesat_p = assign5990_e4742;
        var_stthesat_p_rv = 0.0;

        let (assign6000_e4746,) = {
    if (var_guard41 != 0.0) {
        (p.p305,)
    } else {
        (var_thesatb_p,)
    }
};
        var_thesatb_p = assign6000_e4746;
        var_thesatb_p_rv = 0.0;

        let (assign6010_e4750,) = {
    if (var_guard41 != 0.0) {
        (p.p306,)
    } else {
        (var_thesatg_p,)
    }
};
        var_thesatg_p = assign6010_e4750;
        var_thesatg_p_rv = 0.0;

        let (assign6020_e4754,) = {
    if (var_guard41 != 0.0) {
        (p.p307,)
    } else {
        (var_thesatt_p,)
    }
};
        var_thesatt_p = assign6020_e4754;
        var_thesatt_p_rv = 0.0;

        let (assign6030_e4764,) = {
    if (var_guard41 != 0.0) {
        let assign6030_e4760: f64 = (p.p309 * var_ile);
        let assign6030_e4761: f64 = (1.0 + assign6030_e4760);
        let assign6030_e4762: f64 = (p.p308 / assign6030_e4761);
        (assign6030_e4762,)
    } else {
        (var_ax_p,)
    }
};
        var_ax_p = assign6030_e4764;
        var_ax_p_rv = 0.0;

        let (assign6040_e4778,) = {
    if (var_guard41 != 0.0) {
        let assign6040_e4769: f64 = (var_ile).powf(p.p311);
        let assign6040_e4770: f64 = (p.p310 * assign6040_e4769);
        let assign6040_e4774: f64 = (p.p312 * var_iwe);
        let assign6040_e4775: f64 = (1.0 + assign6040_e4774);
        let assign6040_e4776: f64 = (assign6040_e4770 * assign6040_e4775);
        (assign6040_e4776,)
    } else {
        (var_alp_p,)
    }
};
        var_alp_p = assign6040_e4778;
        var_alp_p_rv = 0.0;

        let (assign6050_e4784,) = {
    if (var_guard41 != 0.0) {
        let assign6050_e4782: f64 = (var_ile).powf(p.p314);
        (assign6050_e4782,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign6050_e4784;
        var_tmpx_rv = 0.0;

        let (assign6060_e4804,) = {
    if (var_guard41 != 0.0) {
        let assign6060_e4788: f64 = (p.p313 * var_tmpx);
        let assign6060_e4792: f64 = (p.p316 * var_iwe);
        let assign6060_e4793: f64 = (1.0 + assign6060_e4792);
        let assign6060_e4794: f64 = (assign6060_e4788 * assign6060_e4793);
        let assign6060_e4798: f64 = (p.p315 * var_ile);
        let assign6060_e4800: f64 = (assign6060_e4798 * var_tmpx);
        let assign6060_e4801: f64 = (1.0 + assign6060_e4800);
        let assign6060_e4802: f64 = (assign6060_e4794 / assign6060_e4801);
        (assign6060_e4802,)
    } else {
        (var_alp1_p,)
    }
};
        var_alp1_p = assign6060_e4804;
        var_alp1_p_rv = 0.0;

        let (assign6070_e4810,) = {
    if (var_guard41 != 0.0) {
        let assign6070_e4808: f64 = (var_ile).powf(p.p318);
        (assign6070_e4808,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign6070_e4810;
        var_tmpx_rv = 0.0;

        let (assign6080_e4830,) = {
    if (var_guard41 != 0.0) {
        let assign6080_e4814: f64 = (p.p317 * var_tmpx);
        let assign6080_e4818: f64 = (p.p320 * var_iwe);
        let assign6080_e4819: f64 = (1.0 + assign6080_e4818);
        let assign6080_e4820: f64 = (assign6080_e4814 * assign6080_e4819);
        let assign6080_e4824: f64 = (p.p319 * var_ile);
        let assign6080_e4826: f64 = (assign6080_e4824 * var_tmpx);
        let assign6080_e4827: f64 = (1.0 + assign6080_e4826);
        let assign6080_e4828: f64 = (assign6080_e4820 / assign6080_e4827);
        (assign6080_e4828,)
    } else {
        (var_alp2_p,)
    }
};
        var_alp2_p = assign6080_e4830;
        var_alp2_p_rv = 0.0;

        let (assign6090_e4834,) = {
    if (var_guard41 != 0.0) {
        (p.p321,)
    } else {
        (var_vp_p,)
    }
};
        var_vp_p = assign6090_e4834;
        var_vp_p_rv = 0.0;

        let (assign6100_e4850,) = {
    if (var_guard41 != 0.0) {
        let assign6100_e4840: f64 = (p.p323 * var_ile);
        let assign6100_e4841: f64 = (1.0 + assign6100_e4840);
        let assign6100_e4842: f64 = (p.p322 * assign6100_e4841);
        let assign6100_e4846: f64 = (p.p324 * var_iwe);
        let assign6100_e4847: f64 = (1.0 + assign6100_e4846);
        let assign6100_e4848: f64 = (assign6100_e4842 * assign6100_e4847);
        (assign6100_e4848,)
    } else {
        (var_a1_p,)
    }
};
        var_a1_p = assign6100_e4850;
        var_a1_p_rv = 0.0;

        let (assign6110_e4854,) = {
    if (var_guard41 != 0.0) {
        (p.p325,)
    } else {
        (var_a2_p,)
    }
};
        var_a2_p = assign6110_e4854;
        var_a2_p_rv = 0.0;

        let (assign6120_e4858,) = {
    if (var_guard41 != 0.0) {
        (p.p326,)
    } else {
        (var_sta2_p,)
    }
};
        var_sta2_p = assign6120_e4858;
        var_sta2_p_rv = 0.0;

        let (assign6130_e4874,) = {
    if (var_guard41 != 0.0) {
        let assign6130_e4864: f64 = (p.p328 * var_ile);
        let assign6130_e4865: f64 = (1.0 + assign6130_e4864);
        let assign6130_e4866: f64 = (p.p327 * assign6130_e4865);
        let assign6130_e4870: f64 = (p.p329 * var_iwe);
        let assign6130_e4871: f64 = (1.0 + assign6130_e4870);
        let assign6130_e4872: f64 = (assign6130_e4866 * assign6130_e4871);
        (assign6130_e4872,)
    } else {
        (var_a3_p,)
    }
};
        var_a3_p = assign6130_e4874;
        var_a3_p_rv = 0.0;

        let (assign6140_e4890,) = {
    if (var_guard41 != 0.0) {
        let assign6140_e4880: f64 = (p.p331 * var_ile);
        let assign6140_e4881: f64 = (1.0 + assign6140_e4880);
        let assign6140_e4882: f64 = (p.p330 * assign6140_e4881);
        let assign6140_e4886: f64 = (p.p332 * var_iwe);
        let assign6140_e4887: f64 = (1.0 + assign6140_e4886);
        let assign6140_e4888: f64 = (assign6140_e4882 * assign6140_e4887);
        (assign6140_e4888,)
    } else {
        (var_a4_p,)
    }
};
        var_a4_p = assign6140_e4890;
        var_a4_p_rv = 0.0;

        let (assign6150_e4894,) = {
    if (var_guard41 != 0.0) {
        (p.p333,)
    } else {
        (var_imaxii_p,)
    }
};
        var_imaxii_p = assign6150_e4894;
        var_imaxii_p_rv = 0.0;

        let (assign6160_e4898,) = {
    if (var_guard41 != 0.0) {
        (p.p334,)
    } else {
        (var_gco_p,)
    }
};
        var_gco_p = assign6160_e4898;
        var_gco_p_rv = 0.0;

        let (assign6170_e4904,) = {
    if (var_guard41 != 0.0) {
        let assign6170_e4902: f64 = (p.p335 / var_iae);
        (assign6170_e4902,)
    } else {
        (var_iginv_p,)
    }
};
        var_iginv_p = assign6170_e4904;
        var_iginv_p_rv = 0.0;

        let (assign6180_e4914,) = {
    if (var_guard41 != 0.0) {
        let assign6180_e4908: f64 = (p.p336 * p.p236);
        let assign6180_e4911: f64 = (1e-6 * var_iwe);
        let assign6180_e4912: f64 = (assign6180_e4908 / assign6180_e4911);
        (assign6180_e4912,)
    } else {
        (var_igov_p,)
    }
};
        var_igov_p = assign6180_e4914;
        var_igov_p_rv = 0.0;

        let (assign6190_e4924,) = {
    if (var_guard41 != 0.0) {
        let assign6190_e4918: f64 = (p.p337 * p.p237);
        let assign6190_e4921: f64 = (1e-6 * var_iwe);
        let assign6190_e4922: f64 = (assign6190_e4918 / assign6190_e4921);
        (assign6190_e4922,)
    } else {
        (var_igovd_p,)
    }
};
        var_igovd_p = assign6190_e4924;
        var_igovd_p_rv = 0.0;

        let (assign6200_e4928,) = {
    if (var_guard41 != 0.0) {
        (p.p338,)
    } else {
        (var_stig_p,)
    }
};
        var_stig_p = assign6200_e4928;
        var_stig_p_rv = 0.0;

        let (assign6210_e4932,) = {
    if (var_guard41 != 0.0) {
        (p.p339,)
    } else {
        (var_gc2_p,)
    }
};
        var_gc2_p = assign6210_e4932;
        var_gc2_p_rv = 0.0;

        let (assign6220_e4936,) = {
    if (var_guard41 != 0.0) {
        (p.p340,)
    } else {
        (var_gc3_p,)
    }
};
        var_gc3_p = assign6220_e4936;
        var_gc3_p_rv = 0.0;

        let (assign6230_e4940,) = {
    if (var_guard41 != 0.0) {
        (p.p339,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign6230_e4940;
        var_gc2ov_p_rv = 0.0;

        let assign6240_e4942: f64 = if param_given[341] { 1.0 } else { 0.0 };
        let assign6240_e4944: f64 = if assign6240_e4942 == 1.0 { 1.0 } else { 0.0 };
        var_guard44 = assign6240_e4944;
        var_guard44_rv = 0.0;

        let (assign6250_e4950,) = {
    if ((var_guard41 != 0.0) && (var_guard44 != 0.0)) {
        (p.p341,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign6250_e4950;
        var_gc2ov_p_rv = 0.0;

        let (assign6260_e4954,) = {
    if (var_guard41 != 0.0) {
        (p.p340,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign6260_e4954;
        var_gc3ov_p_rv = 0.0;

        let assign6270_e4956: f64 = if param_given[342] { 1.0 } else { 0.0 };
        let assign6270_e4958: f64 = if assign6270_e4956 == 1.0 { 1.0 } else { 0.0 };
        var_guard45 = assign6270_e4958;
        var_guard45_rv = 0.0;

        let (assign6280_e4964,) = {
    if ((var_guard41 != 0.0) && (var_guard45 != 0.0)) {
        (p.p342,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign6280_e4964;
        var_gc3ov_p_rv = 0.0;

        let (assign6290_e4968,) = {
    if (var_guard41 != 0.0) {
        (var_gc2ov_p,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign6290_e4968;
        var_gc2ovd_p_rv = 0.0;

        let assign6300_e4970: f64 = if param_given[343] { 1.0 } else { 0.0 };
        let assign6300_e4972: f64 = if assign6300_e4970 == 1.0 { 1.0 } else { 0.0 };
        var_guard46 = assign6300_e4972;
        var_guard46_rv = 0.0;

        let (assign6310_e4978,) = {
    if ((var_guard41 != 0.0) && (var_guard46 != 0.0)) {
        (p.p343,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign6310_e4978;
        var_gc2ovd_p_rv = 0.0;

        let (assign6320_e4982,) = {
    if (var_guard41 != 0.0) {
        (var_gc3ov_p,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign6320_e4982;
        var_gc3ovd_p_rv = 0.0;

        let assign6330_e4984: f64 = if param_given[344] { 1.0 } else { 0.0 };
        let assign6330_e4986: f64 = if assign6330_e4984 == 1.0 { 1.0 } else { 0.0 };
        var_guard47 = assign6330_e4986;
        var_guard47_rv = 0.0;

        let (assign6340_e4992,) = {
    if ((var_guard41 != 0.0) && (var_guard47 != 0.0)) {
        (p.p344,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign6340_e4992;
        var_gc3ovd_p_rv = 0.0;

        let (assign6350_e4996,) = {
    if (var_guard41 != 0.0) {
        (p.p345,)
    } else {
        (var_chib_p,)
    }
};
        var_chib_p = assign6350_e4996;
        var_chib_p_rv = 0.0;

        let (assign6360_e5006,) = {
    if (var_guard41 != 0.0) {
        let assign6360_e5000: f64 = (p.p346 * p.p236);
        let assign6360_e5003: f64 = (1e-6 * var_iwe);
        let assign6360_e5004: f64 = (assign6360_e5000 / assign6360_e5003);
        (assign6360_e5004,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign6360_e5006;
        var_agidl_p_rv = 0.0;

        let (assign6370_e5016,) = {
    if (var_guard41 != 0.0) {
        let assign6370_e5010: f64 = (p.p347 * p.p237);
        let assign6370_e5013: f64 = (1e-6 * var_iwe);
        let assign6370_e5014: f64 = (assign6370_e5010 / assign6370_e5013);
        (assign6370_e5014,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign6370_e5016;
        var_agidld_p_rv = 0.0;

        *var_a1_p_slot = var_a1_p;
        *var_a1_p_rv_slot = var_a1_p_rv;
        *var_a2_p_slot = var_a2_p;
        *var_a2_p_rv_slot = var_a2_p_rv;
        *var_a3_p_slot = var_a3_p;
        *var_a3_p_rv_slot = var_a3_p_rv;
        *var_a4_p_slot = var_a4_p;
        *var_a4_p_rv_slot = var_a4_p_rv;
        *var_agidl_p_slot = var_agidl_p;
        *var_agidl_p_rv_slot = var_agidl_p_rv;
        *var_agidld_p_slot = var_agidld_p;
        *var_agidld_p_rv_slot = var_agidld_p_rv;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp1_p_rv_slot = var_alp1_p_rv;
        *var_alp2_p_slot = var_alp2_p;
        *var_alp2_p_rv_slot = var_alp2_p_rv;
        *var_alp_p_slot = var_alp_p;
        *var_alp_p_rv_slot = var_alp_p_rv;
        *var_ax_p_slot = var_ax_p;
        *var_ax_p_rv_slot = var_ax_p_rv;
        *var_chib_p_slot = var_chib_p;
        *var_chib_p_rv_slot = var_chib_p_rv;
        *var_feta_p_slot = var_feta_p;
        *var_feta_p_rv_slot = var_feta_p_rv;
        *var_gc2_p_slot = var_gc2_p;
        *var_gc2_p_rv_slot = var_gc2_p_rv;
        *var_gc2ov_p_slot = var_gc2ov_p;
        *var_gc2ov_p_rv_slot = var_gc2ov_p_rv;
        *var_gc2ovd_p_slot = var_gc2ovd_p;
        *var_gc2ovd_p_rv_slot = var_gc2ovd_p_rv;
        *var_gc3_p_slot = var_gc3_p;
        *var_gc3_p_rv_slot = var_gc3_p_rv;
        *var_gc3ov_p_slot = var_gc3ov_p;
        *var_gc3ov_p_rv_slot = var_gc3ov_p_rv;
        *var_gc3ovd_p_slot = var_gc3ovd_p;
        *var_gc3ovd_p_rv_slot = var_gc3ovd_p_rv;
        *var_gco_p_slot = var_gco_p;
        *var_gco_p_rv_slot = var_gco_p_rv;
        *var_guard44_slot = var_guard44;
        *var_guard44_rv_slot = var_guard44_rv;
        *var_guard45_slot = var_guard45;
        *var_guard45_rv_slot = var_guard45_rv;
        *var_guard46_slot = var_guard46;
        *var_guard46_rv_slot = var_guard46_rv;
        *var_guard47_slot = var_guard47;
        *var_guard47_rv_slot = var_guard47_rv;
        *var_iginv_p_slot = var_iginv_p;
        *var_iginv_p_rv_slot = var_iginv_p_rv;
        *var_igov_p_slot = var_igov_p;
        *var_igov_p_rv_slot = var_igov_p_rv;
        *var_igovd_p_slot = var_igovd_p;
        *var_igovd_p_rv_slot = var_igovd_p_rv;
        *var_imaxii_p_slot = var_imaxii_p;
        *var_imaxii_p_rv_slot = var_imaxii_p_rv;
        *var_rs_p_slot = var_rs_p;
        *var_rs_p_rv_slot = var_rs_p_rv;
        *var_rsb_p_slot = var_rsb_p;
        *var_rsb_p_rv_slot = var_rsb_p_rv;
        *var_rsg_p_slot = var_rsg_p;
        *var_rsg_p_rv_slot = var_rsg_p_rv;
        *var_sta2_p_slot = var_sta2_p;
        *var_sta2_p_rv_slot = var_sta2_p_rv;
        *var_stig_p_slot = var_stig_p;
        *var_stig_p_rv_slot = var_stig_p_rv;
        *var_strs_p_slot = var_strs_p;
        *var_strs_p_rv_slot = var_strs_p_rv;
        *var_stthesat_p_slot = var_stthesat_p;
        *var_stthesat_p_rv_slot = var_stthesat_p_rv;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_rv_slot = var_thesat_p_rv;
        *var_thesatb_p_slot = var_thesatb_p;
        *var_thesatb_p_rv_slot = var_thesatb_p_rv;
        *var_thesatg_p_slot = var_thesatg_p;
        *var_thesatg_p_rv_slot = var_thesatg_p_rv;
        *var_thesatt_p_slot = var_thesatt_p;
        *var_thesatt_p_rv_slot = var_thesatt_p_rv;
        *var_tmpx_slot = var_tmpx;
        *var_tmpx_rv_slot = var_tmpx_rv;
        *var_vp_p_slot = var_vp_p;
        *var_vp_p_rv_slot = var_vp_p_rv;
    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_gpe: f64,
        var_guard41: f64,
        var_gwe: f64,
        var_iae: f64,
        var_iilcv: f64,
        var_iiwcv: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_iwe: f64,
        var_lecv: f64,
        var_wecv: f64,
        var_alp1ac_p_slot: &mut f64,
        var_alp1ac_p_rv_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_alpac_p_rv_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_axac_p_rv_slot: &mut f64,
        var_axacl_i_slot: &mut f64,
        var_axacl_i_rv_slot: &mut f64,
        var_axaco_i_slot: &mut f64,
        var_axaco_i_rv_slot: &mut f64,
        var_axinr_p_slot: &mut f64,
        var_axinr_p_rv_slot: &mut f64,
        var_bgidl_p_slot: &mut f64,
        var_bgidl_p_rv_slot: &mut f64,
        var_bgidld_p_slot: &mut f64,
        var_bgidld_p_rv_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfr_p_rv_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgbov_p_rv_slot: &mut f64,
        var_cgidl_p_slot: &mut f64,
        var_cgidl_p_rv_slot: &mut f64,
        var_cgidld_p_slot: &mut f64,
        var_cgidld_p_rv_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgov_p_rv_slot: &mut f64,
        var_cgovaccg_p_slot: &mut f64,
        var_cgovaccg_p_rv_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_cgovd_p_rv_slot: &mut f64,
        var_cinr_p_slot: &mut f64,
        var_cinr_p_rv_slot: &mut f64,
        var_cinrd_p_slot: &mut f64,
        var_cinrd_p_rv_slot: &mut f64,
        var_cox_p_slot: &mut f64,
        var_cox_p_rv_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_delvtac_p_rv_slot: &mut f64,
        var_dvfbinr_p_slot: &mut f64,
        var_dvfbinr_p_rv_slot: &mut f64,
        var_facneffac_p_slot: &mut f64,
        var_facneffac_p_rv_slot: &mut f64,
        var_fcgovacc_p_slot: &mut f64,
        var_fcgovacc_p_rv_slot: &mut f64,
        var_fcgovaccd_p_slot: &mut f64,
        var_fcgovaccd_p_rv_slot: &mut f64,
        var_fcinracc_p_slot: &mut f64,
        var_fcinracc_p_rv_slot: &mut f64,
        var_fcinrdep_p_slot: &mut f64,
        var_fcinrdep_p_rv_slot: &mut f64,
        var_guard48_slot: &mut f64,
        var_guard48_rv_slot: &mut f64,
        var_guard49_slot: &mut f64,
        var_guard49_rv_slot: &mut f64,
        var_guard50_slot: &mut f64,
        var_guard50_rv_slot: &mut f64,
        var_guard51_slot: &mut f64,
        var_guard51_rv_slot: &mut f64,
        var_guard52_slot: &mut f64,
        var_guard52_rv_slot: &mut f64,
        var_guard53_slot: &mut f64,
        var_guard53_rv_slot: &mut f64,
        var_guard54_slot: &mut f64,
        var_guard54_rv_slot: &mut f64,
        var_stbgidl_p_slot: &mut f64,
        var_stbgidl_p_rv_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_stbgidld_p_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
        var_thesatacl_i_slot: &mut f64,
        var_thesatacl_i_rv_slot: &mut f64,
        var_thesataclexp_i_slot: &mut f64,
        var_thesataclexp_i_rv_slot: &mut f64,
        var_thesataclw_i_slot: &mut f64,
        var_thesataclw_i_rv_slot: &mut f64,
        var_thesataco_i_slot: &mut f64,
        var_thesataco_i_rv_slot: &mut f64,
        var_thesatacw_i_slot: &mut f64,
        var_thesatacw_i_rv_slot: &mut f64,
        var_tmpx_slot: &mut f64,
        var_tmpx_rv_slot: &mut f64,
    ) {
        let mut var_alp1ac_p: f64 = *var_alp1ac_p_slot;
        let mut var_alp1ac_p_rv: f64 = *var_alp1ac_p_rv_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_alpac_p_rv: f64 = *var_alpac_p_rv_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_axac_p_rv: f64 = *var_axac_p_rv_slot;
        let mut var_axacl_i: f64 = *var_axacl_i_slot;
        let mut var_axacl_i_rv: f64 = *var_axacl_i_rv_slot;
        let mut var_axaco_i: f64 = *var_axaco_i_slot;
        let mut var_axaco_i_rv: f64 = *var_axaco_i_rv_slot;
        let mut var_axinr_p: f64 = *var_axinr_p_slot;
        let mut var_axinr_p_rv: f64 = *var_axinr_p_rv_slot;
        let mut var_bgidl_p: f64 = *var_bgidl_p_slot;
        let mut var_bgidl_p_rv: f64 = *var_bgidl_p_rv_slot;
        let mut var_bgidld_p: f64 = *var_bgidld_p_slot;
        let mut var_bgidld_p_rv: f64 = *var_bgidld_p_rv_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfr_p_rv: f64 = *var_cfr_p_rv_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgbov_p_rv: f64 = *var_cgbov_p_rv_slot;
        let mut var_cgidl_p: f64 = *var_cgidl_p_slot;
        let mut var_cgidl_p_rv: f64 = *var_cgidl_p_rv_slot;
        let mut var_cgidld_p: f64 = *var_cgidld_p_slot;
        let mut var_cgidld_p_rv: f64 = *var_cgidld_p_rv_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgov_p_rv: f64 = *var_cgov_p_rv_slot;
        let mut var_cgovaccg_p: f64 = *var_cgovaccg_p_slot;
        let mut var_cgovaccg_p_rv: f64 = *var_cgovaccg_p_rv_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_cgovd_p_rv: f64 = *var_cgovd_p_rv_slot;
        let mut var_cinr_p: f64 = *var_cinr_p_slot;
        let mut var_cinr_p_rv: f64 = *var_cinr_p_rv_slot;
        let mut var_cinrd_p: f64 = *var_cinrd_p_slot;
        let mut var_cinrd_p_rv: f64 = *var_cinrd_p_rv_slot;
        let mut var_cox_p: f64 = *var_cox_p_slot;
        let mut var_cox_p_rv: f64 = *var_cox_p_rv_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_delvtac_p_rv: f64 = *var_delvtac_p_rv_slot;
        let mut var_dvfbinr_p: f64 = *var_dvfbinr_p_slot;
        let mut var_dvfbinr_p_rv: f64 = *var_dvfbinr_p_rv_slot;
        let mut var_facneffac_p: f64 = *var_facneffac_p_slot;
        let mut var_facneffac_p_rv: f64 = *var_facneffac_p_rv_slot;
        let mut var_fcgovacc_p: f64 = *var_fcgovacc_p_slot;
        let mut var_fcgovacc_p_rv: f64 = *var_fcgovacc_p_rv_slot;
        let mut var_fcgovaccd_p: f64 = *var_fcgovaccd_p_slot;
        let mut var_fcgovaccd_p_rv: f64 = *var_fcgovaccd_p_rv_slot;
        let mut var_fcinracc_p: f64 = *var_fcinracc_p_slot;
        let mut var_fcinracc_p_rv: f64 = *var_fcinracc_p_rv_slot;
        let mut var_fcinrdep_p: f64 = *var_fcinrdep_p_slot;
        let mut var_fcinrdep_p_rv: f64 = *var_fcinrdep_p_rv_slot;
        let mut var_guard48: f64 = *var_guard48_slot;
        let mut var_guard48_rv: f64 = *var_guard48_rv_slot;
        let mut var_guard49: f64 = *var_guard49_slot;
        let mut var_guard49_rv: f64 = *var_guard49_rv_slot;
        let mut var_guard50: f64 = *var_guard50_slot;
        let mut var_guard50_rv: f64 = *var_guard50_rv_slot;
        let mut var_guard51: f64 = *var_guard51_slot;
        let mut var_guard51_rv: f64 = *var_guard51_rv_slot;
        let mut var_guard52: f64 = *var_guard52_slot;
        let mut var_guard52_rv: f64 = *var_guard52_rv_slot;
        let mut var_guard53: f64 = *var_guard53_slot;
        let mut var_guard53_rv: f64 = *var_guard53_rv_slot;
        let mut var_guard54: f64 = *var_guard54_slot;
        let mut var_guard54_rv: f64 = *var_guard54_rv_slot;
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidl_p_rv: f64 = *var_stbgidl_p_rv_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_stbgidld_p_rv: f64 = *var_stbgidld_p_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;
        let mut var_thesatacl_i: f64 = *var_thesatacl_i_slot;
        let mut var_thesatacl_i_rv: f64 = *var_thesatacl_i_rv_slot;
        let mut var_thesataclexp_i: f64 = *var_thesataclexp_i_slot;
        let mut var_thesataclexp_i_rv: f64 = *var_thesataclexp_i_rv_slot;
        let mut var_thesataclw_i: f64 = *var_thesataclw_i_slot;
        let mut var_thesataclw_i_rv: f64 = *var_thesataclw_i_rv_slot;
        let mut var_thesataco_i: f64 = *var_thesataco_i_slot;
        let mut var_thesataco_i_rv: f64 = *var_thesataco_i_rv_slot;
        let mut var_thesatacw_i: f64 = *var_thesatacw_i_slot;
        let mut var_thesatacw_i_rv: f64 = *var_thesatacw_i_rv_slot;
        let mut var_tmpx: f64 = *var_tmpx_slot;
        let mut var_tmpx_rv: f64 = *var_tmpx_rv_slot;

        let (assign6380_e5020,) = {
    if (var_guard41 != 0.0) {
        (p.p348,)
    } else {
        (var_bgidl_p,)
    }
};
        var_bgidl_p = assign6380_e5020;
        var_bgidl_p_rv = 0.0;

        let (assign6390_e5024,) = {
    if (var_guard41 != 0.0) {
        (p.p349,)
    } else {
        (var_bgidld_p,)
    }
};
        var_bgidld_p = assign6390_e5024;
        var_bgidld_p_rv = 0.0;

        let (assign6400_e5028,) = {
    if (var_guard41 != 0.0) {
        (p.p350,)
    } else {
        (var_stbgidl_p,)
    }
};
        var_stbgidl_p = assign6400_e5028;
        var_stbgidl_p_rv = 0.0;

        let (assign6410_e5032,) = {
    if (var_guard41 != 0.0) {
        (p.p351,)
    } else {
        (var_stbgidld_p,)
    }
};
        var_stbgidld_p = assign6410_e5032;
        var_stbgidld_p_rv = 0.0;

        let (assign6420_e5036,) = {
    if (var_guard41 != 0.0) {
        (p.p352,)
    } else {
        (var_cgidl_p,)
    }
};
        var_cgidl_p = assign6420_e5036;
        var_cgidl_p_rv = 0.0;

        let (assign6430_e5040,) = {
    if (var_guard41 != 0.0) {
        (p.p353,)
    } else {
        (var_cgidld_p,)
    }
};
        var_cgidld_p = assign6430_e5040;
        var_cgidld_p_rv = 0.0;

        let (assign6440_e5052,) = {
    if (var_guard41 != 0.0) {
        let assign6440_e5044: f64 = (8.8541878176e-12 * p.p209);
        let assign6440_e5046: f64 = (assign6440_e5044 * var_wecv);
        let assign6440_e5048: f64 = (assign6440_e5046 * var_lecv);
        let assign6440_e5050: f64 = (assign6440_e5048 / p.p208);
        (assign6440_e5050,)
    } else {
        (var_cox_p,)
    }
};
        var_cox_p = assign6440_e5052;
        var_cox_p_rv = 0.0;

        let (assign6450_e5064,) = {
    if (var_guard41 != 0.0) {
        let assign6450_e5056: f64 = (8.8541878176e-12 * p.p209);
        let assign6450_e5058: f64 = (assign6450_e5056 * var_wecv);
        let assign6450_e5060: f64 = (assign6450_e5058 * p.p236);
        let assign6450_e5062: f64 = (assign6450_e5060 / p.p234);
        (assign6450_e5062,)
    } else {
        (var_cgov_p,)
    }
};
        var_cgov_p = assign6450_e5064;
        var_cgov_p_rv = 0.0;

        let (assign6460_e5076,) = {
    if (var_guard41 != 0.0) {
        let assign6460_e5068: f64 = (8.8541878176e-12 * p.p209);
        let assign6460_e5070: f64 = (assign6460_e5068 * var_wecv);
        let assign6460_e5072: f64 = (assign6460_e5070 * p.p237);
        let assign6460_e5074: f64 = (assign6460_e5072 / p.p235);
        (assign6460_e5074,)
    } else {
        (var_cgovd_p,)
    }
};
        var_cgovd_p = assign6460_e5076;
        var_cgovd_p_rv = 0.0;

        let (assign6470_e5094,) = {
    if (var_guard41 != 0.0) {
        let assign6470_e5082: f64 = (var_ile).powf(p.p356);
        let assign6470_e5083: f64 = (p.p355 * assign6470_e5082);
        let assign6470_e5084: f64 = (p.p354 + assign6470_e5083);
        let assign6470_e5087: f64 = (p.p357 * var_iwe);
        let assign6470_e5088: f64 = (assign6470_e5084 + assign6470_e5087);
        let assign6470_e5091: f64 = (p.p358 * var_iae);
        let assign6470_e5092: f64 = (assign6470_e5088 + assign6470_e5091);
        (assign6470_e5092,)
    } else {
        (var_delvtac_p,)
    }
};
        var_delvtac_p = assign6470_e5094;
        var_delvtac_p_rv = 0.0;

        let (assign6480_e5110,) = {
    if (var_guard41 != 0.0) {
        let assign6480_e5099: f64 = (p.p360 * var_ile);
        let assign6480_e5100: f64 = (p.p359 + assign6480_e5099);
        let assign6480_e5103: f64 = (p.p361 * var_iwe);
        let assign6480_e5104: f64 = (assign6480_e5100 + assign6480_e5103);
        let assign6480_e5107: f64 = (p.p362 * var_iae);
        let assign6480_e5108: f64 = (assign6480_e5104 + assign6480_e5107);
        (assign6480_e5108,)
    } else {
        (var_facneffac_p,)
    }
};
        var_facneffac_p = assign6480_e5110;
        var_facneffac_p_rv = 0.0;

        let (assign6490_e5114,) = {
    if (var_guard41 != 0.0) {
        (p.p296,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign6490_e5114;
        var_thesataco_i_rv = 0.0;

        let assign6500_e5116: f64 = if param_given[363] { 1.0 } else { 0.0 };
        let assign6500_e5118: f64 = if assign6500_e5116 == 1.0 { 1.0 } else { 0.0 };
        var_guard48 = assign6500_e5118;
        var_guard48_rv = 0.0;

        let (assign6510_e5124,) = {
    if ((var_guard41 != 0.0) && (var_guard48 != 0.0)) {
        (p.p363,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign6510_e5124;
        var_thesataco_i_rv = 0.0;

        let (assign6520_e5128,) = {
    if (var_guard41 != 0.0) {
        (p.p297,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign6520_e5128;
        var_thesatacl_i_rv = 0.0;

        let assign6530_e5130: f64 = if param_given[364] { 1.0 } else { 0.0 };
        let assign6530_e5132: f64 = if assign6530_e5130 == 1.0 { 1.0 } else { 0.0 };
        var_guard49 = assign6530_e5132;
        var_guard49_rv = 0.0;

        let (assign6540_e5138,) = {
    if ((var_guard41 != 0.0) && (var_guard49 != 0.0)) {
        (p.p364,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign6540_e5138;
        var_thesatacl_i_rv = 0.0;

        let (assign6550_e5142,) = {
    if (var_guard41 != 0.0) {
        (p.p298,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign6550_e5142;
        var_thesataclexp_i_rv = 0.0;

        let assign6560_e5144: f64 = if param_given[365] { 1.0 } else { 0.0 };
        let assign6560_e5146: f64 = if assign6560_e5144 == 1.0 { 1.0 } else { 0.0 };
        var_guard50 = assign6560_e5146;
        var_guard50_rv = 0.0;

        let (assign6570_e5152,) = {
    if ((var_guard41 != 0.0) && (var_guard50 != 0.0)) {
        (p.p365,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign6570_e5152;
        var_thesataclexp_i_rv = 0.0;

        let (assign6580_e5156,) = {
    if (var_guard41 != 0.0) {
        (p.p299,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign6580_e5156;
        var_thesatacw_i_rv = 0.0;

        let assign6590_e5158: f64 = if param_given[366] { 1.0 } else { 0.0 };
        let assign6590_e5160: f64 = if assign6590_e5158 == 1.0 { 1.0 } else { 0.0 };
        var_guard51 = assign6590_e5160;
        var_guard51_rv = 0.0;

        let (assign6600_e5166,) = {
    if ((var_guard41 != 0.0) && (var_guard51 != 0.0)) {
        (p.p366,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign6600_e5166;
        var_thesatacw_i_rv = 0.0;

        let (assign6610_e5170,) = {
    if (var_guard41 != 0.0) {
        (p.p300,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign6610_e5170;
        var_thesataclw_i_rv = 0.0;

        let assign6620_e5172: f64 = if param_given[367] { 1.0 } else { 0.0 };
        let assign6620_e5174: f64 = if assign6620_e5172 == 1.0 { 1.0 } else { 0.0 };
        var_guard52 = assign6620_e5174;
        var_guard52_rv = 0.0;

        let (assign6630_e5180,) = {
    if ((var_guard41 != 0.0) && (var_guard52 != 0.0)) {
        (p.p367,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign6630_e5180;
        var_thesataclw_i_rv = 0.0;

        let (assign6640_e5206,) = {
    if (var_guard41 != 0.0) {
        let assign6640_e5185: f64 = (var_thesatacl_i * var_gwe);
        let assign6640_e5187: f64 = (assign6640_e5185 / var_gpe);
        let assign6640_e5190: f64 = (var_ile).powf(var_thesataclexp_i);
        let assign6640_e5191: f64 = (assign6640_e5187 * assign6640_e5190);
        let assign6640_e5192: f64 = (var_thesataco_i + assign6640_e5191);
        let assign6640_e5196: f64 = (var_thesatacw_i * var_iwe);
        let assign6640_e5197: f64 = (1.0 + assign6640_e5196);
        let assign6640_e5198: f64 = (assign6640_e5192 * assign6640_e5197);
        let assign6640_e5202: f64 = (var_thesataclw_i * var_iae);
        let assign6640_e5203: f64 = (1.0 + assign6640_e5202);
        let assign6640_e5204: f64 = (assign6640_e5198 * assign6640_e5203);
        (assign6640_e5204,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign6640_e5206;
        var_thesatac_p_rv = 0.0;

        let (assign6650_e5210,) = {
    if (var_guard41 != 0.0) {
        (p.p308,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign6650_e5210;
        var_axaco_i_rv = 0.0;

        let assign6660_e5212: f64 = if param_given[368] { 1.0 } else { 0.0 };
        let assign6660_e5214: f64 = if assign6660_e5212 == 1.0 { 1.0 } else { 0.0 };
        var_guard53 = assign6660_e5214;
        var_guard53_rv = 0.0;

        let (assign6670_e5220,) = {
    if ((var_guard41 != 0.0) && (var_guard53 != 0.0)) {
        (p.p368,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign6670_e5220;
        var_axaco_i_rv = 0.0;

        let (assign6680_e5224,) = {
    if (var_guard41 != 0.0) {
        (p.p309,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign6680_e5224;
        var_axacl_i_rv = 0.0;

        let assign6690_e5226: f64 = if param_given[369] { 1.0 } else { 0.0 };
        let assign6690_e5228: f64 = if assign6690_e5226 == 1.0 { 1.0 } else { 0.0 };
        var_guard54 = assign6690_e5228;
        var_guard54_rv = 0.0;

        let (assign6700_e5234,) = {
    if ((var_guard41 != 0.0) && (var_guard54 != 0.0)) {
        (p.p369,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign6700_e5234;
        var_axacl_i_rv = 0.0;

        let (assign6710_e5244,) = {
    if (var_guard41 != 0.0) {
        let assign6710_e5240: f64 = (var_axacl_i * var_ile);
        let assign6710_e5241: f64 = (1.0 + assign6710_e5240);
        let assign6710_e5242: f64 = (var_axaco_i / assign6710_e5241);
        (assign6710_e5242,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign6710_e5244;
        var_axac_p_rv = 0.0;

        let (assign6720_e5258,) = {
    if (var_guard41 != 0.0) {
        let assign6720_e5249: f64 = (var_ile).powf(p.p371);
        let assign6720_e5250: f64 = (p.p370 * assign6720_e5249);
        let assign6720_e5254: f64 = (p.p372 * var_iwe);
        let assign6720_e5255: f64 = (1.0 + assign6720_e5254);
        let assign6720_e5256: f64 = (assign6720_e5250 * assign6720_e5255);
        (assign6720_e5256,)
    } else {
        (var_alpac_p,)
    }
};
        var_alpac_p = assign6720_e5258;
        var_alpac_p_rv = 0.0;

        let (assign6730_e5264,) = {
    if (var_guard41 != 0.0) {
        let assign6730_e5262: f64 = (var_ile).powf(p.p374);
        (assign6730_e5262,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign6730_e5264;
        var_tmpx_rv = 0.0;

        let (assign6740_e5284,) = {
    if (var_guard41 != 0.0) {
        let assign6740_e5268: f64 = (p.p373 * var_tmpx);
        let assign6740_e5272: f64 = (p.p376 * var_iwe);
        let assign6740_e5273: f64 = (1.0 + assign6740_e5272);
        let assign6740_e5274: f64 = (assign6740_e5268 * assign6740_e5273);
        let assign6740_e5278: f64 = (p.p375 * var_ile);
        let assign6740_e5280: f64 = (assign6740_e5278 * var_tmpx);
        let assign6740_e5281: f64 = (1.0 + assign6740_e5280);
        let assign6740_e5282: f64 = (assign6740_e5274 / assign6740_e5281);
        (assign6740_e5282,)
    } else {
        (var_alp1ac_p,)
    }
};
        var_alp1ac_p = assign6740_e5284;
        var_alp1ac_p_rv = 0.0;

        let (assign6750_e5288,) = {
    if (var_guard41 != 0.0) {
        (p.p377,)
    } else {
        (var_fcgovacc_p,)
    }
};
        var_fcgovacc_p = assign6750_e5288;
        var_fcgovacc_p_rv = 0.0;

        let (assign6760_e5292,) = {
    if (var_guard41 != 0.0) {
        (p.p378,)
    } else {
        (var_fcgovaccd_p,)
    }
};
        var_fcgovaccd_p = assign6760_e5292;
        var_fcgovaccd_p_rv = 0.0;

        let (assign6770_e5296,) = {
    if (var_guard41 != 0.0) {
        (p.p379,)
    } else {
        (var_cgovaccg_p,)
    }
};
        var_cgovaccg_p = assign6770_e5296;
        var_cgovaccg_p_rv = 0.0;

        let (assign6780_e5302,) = {
    if (var_guard41 != 0.0) {
        let assign6780_e5300: f64 = (p.p380 * var_iilcv);
        (assign6780_e5300,)
    } else {
        (var_cgbov_p,)
    }
};
        var_cgbov_p = assign6780_e5302;
        var_cgbov_p_rv = 0.0;

        let (assign6790_e5308,) = {
    if (var_guard41 != 0.0) {
        let assign6790_e5306: f64 = (p.p381 * var_iiwecv);
        (assign6790_e5306,)
    } else {
        (var_cinr_p,)
    }
};
        var_cinr_p = assign6790_e5308;
        var_cinr_p_rv = 0.0;

        let (assign6800_e5314,) = {
    if (var_guard41 != 0.0) {
        let assign6800_e5312: f64 = (p.p382 * var_iiwecv);
        (assign6800_e5312,)
    } else {
        (var_cinrd_p,)
    }
};
        var_cinrd_p = assign6800_e5314;
        var_cinrd_p_rv = 0.0;

        let (assign6810_e5318,) = {
    if (var_guard41 != 0.0) {
        (p.p383,)
    } else {
        (var_dvfbinr_p,)
    }
};
        var_dvfbinr_p = assign6810_e5318;
        var_dvfbinr_p_rv = 0.0;

        let (assign6820_e5322,) = {
    if (var_guard41 != 0.0) {
        (p.p384,)
    } else {
        (var_fcinrdep_p,)
    }
};
        var_fcinrdep_p = assign6820_e5322;
        var_fcinrdep_p_rv = 0.0;

        let (assign6830_e5326,) = {
    if (var_guard41 != 0.0) {
        (p.p385,)
    } else {
        (var_fcinracc_p,)
    }
};
        var_fcinracc_p = assign6830_e5326;
        var_fcinracc_p_rv = 0.0;

        let (assign6840_e5330,) = {
    if (var_guard41 != 0.0) {
        (p.p386,)
    } else {
        (var_axinr_p,)
    }
};
        var_axinr_p = assign6840_e5330;
        var_axinr_p_rv = 0.0;

        let (assign6850_e5336,) = {
    if (var_guard41 != 0.0) {
        let assign6850_e5334: f64 = (p.p387 * var_iiwcv);
        (assign6850_e5334,)
    } else {
        (var_cfr_p,)
    }
};
        var_cfr_p = assign6850_e5336;
        var_cfr_p_rv = 0.0;

        *var_alp1ac_p_slot = var_alp1ac_p;
        *var_alp1ac_p_rv_slot = var_alp1ac_p_rv;
        *var_alpac_p_slot = var_alpac_p;
        *var_alpac_p_rv_slot = var_alpac_p_rv;
        *var_axac_p_slot = var_axac_p;
        *var_axac_p_rv_slot = var_axac_p_rv;
        *var_axacl_i_slot = var_axacl_i;
        *var_axacl_i_rv_slot = var_axacl_i_rv;
        *var_axaco_i_slot = var_axaco_i;
        *var_axaco_i_rv_slot = var_axaco_i_rv;
        *var_axinr_p_slot = var_axinr_p;
        *var_axinr_p_rv_slot = var_axinr_p_rv;
        *var_bgidl_p_slot = var_bgidl_p;
        *var_bgidl_p_rv_slot = var_bgidl_p_rv;
        *var_bgidld_p_slot = var_bgidld_p;
        *var_bgidld_p_rv_slot = var_bgidld_p_rv;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfr_p_rv_slot = var_cfr_p_rv;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgbov_p_rv_slot = var_cgbov_p_rv;
        *var_cgidl_p_slot = var_cgidl_p;
        *var_cgidl_p_rv_slot = var_cgidl_p_rv;
        *var_cgidld_p_slot = var_cgidld_p;
        *var_cgidld_p_rv_slot = var_cgidld_p_rv;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgov_p_rv_slot = var_cgov_p_rv;
        *var_cgovaccg_p_slot = var_cgovaccg_p;
        *var_cgovaccg_p_rv_slot = var_cgovaccg_p_rv;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_cgovd_p_rv_slot = var_cgovd_p_rv;
        *var_cinr_p_slot = var_cinr_p;
        *var_cinr_p_rv_slot = var_cinr_p_rv;
        *var_cinrd_p_slot = var_cinrd_p;
        *var_cinrd_p_rv_slot = var_cinrd_p_rv;
        *var_cox_p_slot = var_cox_p;
        *var_cox_p_rv_slot = var_cox_p_rv;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_delvtac_p_rv_slot = var_delvtac_p_rv;
        *var_dvfbinr_p_slot = var_dvfbinr_p;
        *var_dvfbinr_p_rv_slot = var_dvfbinr_p_rv;
        *var_facneffac_p_slot = var_facneffac_p;
        *var_facneffac_p_rv_slot = var_facneffac_p_rv;
        *var_fcgovacc_p_slot = var_fcgovacc_p;
        *var_fcgovacc_p_rv_slot = var_fcgovacc_p_rv;
        *var_fcgovaccd_p_slot = var_fcgovaccd_p;
        *var_fcgovaccd_p_rv_slot = var_fcgovaccd_p_rv;
        *var_fcinracc_p_slot = var_fcinracc_p;
        *var_fcinracc_p_rv_slot = var_fcinracc_p_rv;
        *var_fcinrdep_p_slot = var_fcinrdep_p;
        *var_fcinrdep_p_rv_slot = var_fcinrdep_p_rv;
        *var_guard48_slot = var_guard48;
        *var_guard48_rv_slot = var_guard48_rv;
        *var_guard49_slot = var_guard49;
        *var_guard49_rv_slot = var_guard49_rv;
        *var_guard50_slot = var_guard50;
        *var_guard50_rv_slot = var_guard50_rv;
        *var_guard51_slot = var_guard51;
        *var_guard51_rv_slot = var_guard51_rv;
        *var_guard52_slot = var_guard52;
        *var_guard52_rv_slot = var_guard52_rv;
        *var_guard53_slot = var_guard53;
        *var_guard53_rv_slot = var_guard53_rv;
        *var_guard54_slot = var_guard54;
        *var_guard54_rv_slot = var_guard54_rv;
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidl_p_rv_slot = var_stbgidl_p_rv;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_stbgidld_p_rv_slot = var_stbgidld_p_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
        *var_thesatacl_i_slot = var_thesatacl_i;
        *var_thesatacl_i_rv_slot = var_thesatacl_i_rv;
        *var_thesataclexp_i_slot = var_thesataclexp_i;
        *var_thesataclexp_i_rv_slot = var_thesataclexp_i_rv;
        *var_thesataclw_i_slot = var_thesataclw_i;
        *var_thesataclw_i_rv_slot = var_thesataclw_i_rv;
        *var_thesataco_i_slot = var_thesataco_i;
        *var_thesataco_i_rv_slot = var_thesataco_i_rv;
        *var_thesatacw_i_slot = var_thesatacw_i;
        *var_thesatacw_i_rv_slot = var_thesatacw_i_rv;
        *var_tmpx_slot = var_tmpx;
        *var_tmpx_rv_slot = var_tmpx_rv;
    }
}
