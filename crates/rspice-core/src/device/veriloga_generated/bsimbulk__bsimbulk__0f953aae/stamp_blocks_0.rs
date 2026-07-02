#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        locals: &mut StampLocals,
    ) {
        locals.var_cdscdr_i = 0.0;
        locals.var_cdscdr_i_dn0 = 0.0;
        locals.var_cdscdr_i_dn2 = 0.0;
        locals.var_cdscdr_i_dn3 = 0.0;
        locals.var_cdscdr_i_dn4 = 0.0;
        locals.var_cdscdr_i_dn5 = 0.0;
        locals.var_cdscdr_i_dn6 = 0.0;
        locals.var_cdscdr_i_dn7 = 0.0;
        locals.var_cdscdr_i_dn8 = 0.0;
        locals.var_cdscdr_i_dn9 = 0.0;
        locals.var_cdscdr_i_dn10 = 0.0;
        locals.var_cdscdr_i_dn11 = 0.0;
        locals.var_cdscdr_i_dn12 = 0.0;
        locals.var_cdscdr_i_dn13 = 0.0;
        locals.var_cdscdr_i_dn14 = 0.0;

        locals.var_l_wln1 = 0.0;

        locals.var_ptwgr_i = 0.0;
        locals.var_ptwgr_i_dn0 = 0.0;
        locals.var_ptwgr_i_dn2 = 0.0;
        locals.var_ptwgr_i_dn3 = 0.0;
        locals.var_ptwgr_i_dn4 = 0.0;
        locals.var_ptwgr_i_dn5 = 0.0;
        locals.var_ptwgr_i_dn6 = 0.0;
        locals.var_ptwgr_i_dn7 = 0.0;
        locals.var_ptwgr_i_dn8 = 0.0;
        locals.var_ptwgr_i_dn9 = 0.0;
        locals.var_ptwgr_i_dn10 = 0.0;
        locals.var_ptwgr_i_dn11 = 0.0;
        locals.var_ptwgr_i_dn12 = 0.0;
        locals.var_ptwgr_i_dn13 = 0.0;
        locals.var_ptwgr_i_dn14 = 0.0;

        locals.var_uar_i = 0.0;
        locals.var_uar_i_dn0 = 0.0;
        locals.var_uar_i_dn2 = 0.0;
        locals.var_uar_i_dn3 = 0.0;
        locals.var_uar_i_dn4 = 0.0;
        locals.var_uar_i_dn5 = 0.0;
        locals.var_uar_i_dn6 = 0.0;
        locals.var_uar_i_dn7 = 0.0;
        locals.var_uar_i_dn8 = 0.0;
        locals.var_uar_i_dn9 = 0.0;
        locals.var_uar_i_dn10 = 0.0;
        locals.var_uar_i_dn11 = 0.0;
        locals.var_uar_i_dn12 = 0.0;
        locals.var_uar_i_dn13 = 0.0;
        locals.var_uar_i_dn14 = 0.0;

        locals.var_ucsr_i = 0.0;

        locals.var_ud_a = 0.0;
        locals.var_ud_a_dn0 = 0.0;
        locals.var_ud_a_dn2 = 0.0;
        locals.var_ud_a_dn3 = 0.0;
        locals.var_ud_a_dn4 = 0.0;
        locals.var_ud_a_dn5 = 0.0;
        locals.var_ud_a_dn6 = 0.0;
        locals.var_ud_a_dn7 = 0.0;
        locals.var_ud_a_dn8 = 0.0;
        locals.var_ud_a_dn9 = 0.0;
        locals.var_ud_a_dn10 = 0.0;
        locals.var_ud_a_dn11 = 0.0;
        locals.var_ud_a_dn12 = 0.0;
        locals.var_ud_a_dn13 = 0.0;
        locals.var_ud_a_dn14 = 0.0;

        locals.var_w_wwn1 = 0.0;

        locals.var_inv_sa = 0.0;
        locals.var_inv_sa_dn0 = 0.0;
        locals.var_inv_sa_dn2 = 0.0;
        locals.var_inv_sa_dn3 = 0.0;
        locals.var_inv_sa_dn4 = 0.0;
        locals.var_inv_sa_dn5 = 0.0;
        locals.var_inv_sa_dn6 = 0.0;
        locals.var_inv_sa_dn7 = 0.0;
        locals.var_inv_sa_dn8 = 0.0;
        locals.var_inv_sa_dn9 = 0.0;
        locals.var_inv_sa_dn10 = 0.0;
        locals.var_inv_sa_dn11 = 0.0;
        locals.var_inv_sa_dn12 = 0.0;
        locals.var_inv_sa_dn13 = 0.0;
        locals.var_inv_sa_dn14 = 0.0;

        locals.var_eta_stress = 0.0;
        locals.var_eta_stress_dn0 = 0.0;
        locals.var_eta_stress_dn2 = 0.0;
        locals.var_eta_stress_dn3 = 0.0;
        locals.var_eta_stress_dn4 = 0.0;
        locals.var_eta_stress_dn5 = 0.0;
        locals.var_eta_stress_dn6 = 0.0;
        locals.var_eta_stress_dn7 = 0.0;
        locals.var_eta_stress_dn8 = 0.0;
        locals.var_eta_stress_dn9 = 0.0;
        locals.var_eta_stress_dn10 = 0.0;
        locals.var_eta_stress_dn11 = 0.0;
        locals.var_eta_stress_dn12 = 0.0;
        locals.var_eta_stress_dn13 = 0.0;
        locals.var_eta_stress_dn14 = 0.0;

        locals.var_local_sca = 0.0;
        locals.var_local_sca_dn0 = 0.0;
        locals.var_local_sca_dn2 = 0.0;
        locals.var_local_sca_dn3 = 0.0;
        locals.var_local_sca_dn4 = 0.0;
        locals.var_local_sca_dn5 = 0.0;
        locals.var_local_sca_dn6 = 0.0;
        locals.var_local_sca_dn7 = 0.0;
        locals.var_local_sca_dn8 = 0.0;
        locals.var_local_sca_dn9 = 0.0;
        locals.var_local_sca_dn10 = 0.0;
        locals.var_local_sca_dn11 = 0.0;
        locals.var_local_sca_dn12 = 0.0;
        locals.var_local_sca_dn13 = 0.0;
        locals.var_local_sca_dn14 = 0.0;

        locals.var_m0_i = 0.0;

        locals.var_m0_t = 0.0;
        locals.var_m0_t_dn4 = 0.0;

        locals.var_eta0edge_i = 0.0;
        locals.var_eta0edge_i_dn0 = 0.0;
        locals.var_eta0edge_i_dn2 = 0.0;
        locals.var_eta0edge_i_dn3 = 0.0;
        locals.var_eta0edge_i_dn4 = 0.0;
        locals.var_eta0edge_i_dn5 = 0.0;
        locals.var_eta0edge_i_dn6 = 0.0;
        locals.var_eta0edge_i_dn7 = 0.0;
        locals.var_eta0edge_i_dn8 = 0.0;
        locals.var_eta0edge_i_dn9 = 0.0;
        locals.var_eta0edge_i_dn10 = 0.0;
        locals.var_eta0edge_i_dn11 = 0.0;
        locals.var_eta0edge_i_dn12 = 0.0;
        locals.var_eta0edge_i_dn13 = 0.0;
        locals.var_eta0edge_i_dn14 = 0.0;

        locals.var_kt2edge_i = 0.0;

        locals.var_k2edge_i = 0.0;
        locals.var_k2edge_i_dn0 = 0.0;
        locals.var_k2edge_i_dn2 = 0.0;
        locals.var_k2edge_i_dn3 = 0.0;
        locals.var_k2edge_i_dn4 = 0.0;
        locals.var_k2edge_i_dn5 = 0.0;
        locals.var_k2edge_i_dn6 = 0.0;
        locals.var_k2edge_i_dn7 = 0.0;
        locals.var_k2edge_i_dn8 = 0.0;
        locals.var_k2edge_i_dn9 = 0.0;
        locals.var_k2edge_i_dn10 = 0.0;
        locals.var_k2edge_i_dn11 = 0.0;
        locals.var_k2edge_i_dn12 = 0.0;
        locals.var_k2edge_i_dn13 = 0.0;
        locals.var_k2edge_i_dn14 = 0.0;

        locals.var_mnud1 = 0.0;
        locals.var_mnud1_dn0 = 0.0;
        locals.var_mnud1_dn2 = 0.0;
        locals.var_mnud1_dn3 = 0.0;
        locals.var_mnud1_dn4 = 0.0;
        locals.var_mnud1_dn5 = 0.0;
        locals.var_mnud1_dn6 = 0.0;
        locals.var_mnud1_dn7 = 0.0;
        locals.var_mnud1_dn8 = 0.0;
        locals.var_mnud1_dn9 = 0.0;
        locals.var_mnud1_dn10 = 0.0;
        locals.var_mnud1_dn11 = 0.0;
        locals.var_mnud1_dn12 = 0.0;
        locals.var_mnud1_dn13 = 0.0;
        locals.var_mnud1_dn14 = 0.0;

        locals.var_c0si_i = 0.0;

        locals.var_c0sisat1_i = 0.0;

        locals.var_isubdr = 0.0;
        locals.var_isubdr_dn0 = 0.0;
        locals.var_isubdr_dn2 = 0.0;
        locals.var_isubdr_dn3 = 0.0;
        locals.var_isubdr_dn4 = 0.0;
        locals.var_isubdr_dn5 = 0.0;
        locals.var_isubdr_dn6 = 0.0;
        locals.var_isubdr_dn7 = 0.0;
        locals.var_isubdr_dn8 = 0.0;
        locals.var_isubdr_dn9 = 0.0;
        locals.var_isubdr_dn10 = 0.0;
        locals.var_isubdr_dn11 = 0.0;
        locals.var_isubdr_dn12 = 0.0;
        locals.var_isubdr_dn13 = 0.0;
        locals.var_isubdr_dn14 = 0.0;

        let assign190_e2015: f64 = 0.0;
        locals.var_gmin = assign190_e2015;

        locals.var_eta0r_i = 0.0;
        locals.var_eta0r_i_dn0 = 0.0;
        locals.var_eta0r_i_dn2 = 0.0;
        locals.var_eta0r_i_dn3 = 0.0;
        locals.var_eta0r_i_dn4 = 0.0;
        locals.var_eta0r_i_dn5 = 0.0;
        locals.var_eta0r_i_dn6 = 0.0;
        locals.var_eta0r_i_dn7 = 0.0;
        locals.var_eta0r_i_dn8 = 0.0;
        locals.var_eta0r_i_dn9 = 0.0;
        locals.var_eta0r_i_dn10 = 0.0;
        locals.var_eta0r_i_dn11 = 0.0;
        locals.var_eta0r_i_dn12 = 0.0;
        locals.var_eta0r_i_dn13 = 0.0;
        locals.var_eta0r_i_dn14 = 0.0;

        locals.var_pclmr_i = 0.0;
        locals.var_pclmr_i_dn0 = 0.0;
        locals.var_pclmr_i_dn2 = 0.0;
        locals.var_pclmr_i_dn3 = 0.0;
        locals.var_pclmr_i_dn4 = 0.0;
        locals.var_pclmr_i_dn5 = 0.0;
        locals.var_pclmr_i_dn6 = 0.0;
        locals.var_pclmr_i_dn7 = 0.0;
        locals.var_pclmr_i_dn8 = 0.0;
        locals.var_pclmr_i_dn9 = 0.0;
        locals.var_pclmr_i_dn10 = 0.0;
        locals.var_pclmr_i_dn11 = 0.0;
        locals.var_pclmr_i_dn12 = 0.0;
        locals.var_pclmr_i_dn13 = 0.0;
        locals.var_pclmr_i_dn14 = 0.0;

        locals.var_ptwgr_t = 0.0;
        locals.var_ptwgr_t_dn0 = 0.0;
        locals.var_ptwgr_t_dn2 = 0.0;
        locals.var_ptwgr_t_dn3 = 0.0;
        locals.var_ptwgr_t_dn4 = 0.0;
        locals.var_ptwgr_t_dn5 = 0.0;
        locals.var_ptwgr_t_dn6 = 0.0;
        locals.var_ptwgr_t_dn7 = 0.0;
        locals.var_ptwgr_t_dn8 = 0.0;
        locals.var_ptwgr_t_dn9 = 0.0;
        locals.var_ptwgr_t_dn10 = 0.0;
        locals.var_ptwgr_t_dn11 = 0.0;
        locals.var_ptwgr_t_dn12 = 0.0;
        locals.var_ptwgr_t_dn13 = 0.0;
        locals.var_ptwgr_t_dn14 = 0.0;

        locals.var_uar_t = 0.0;
        locals.var_uar_t_dn0 = 0.0;
        locals.var_uar_t_dn2 = 0.0;
        locals.var_uar_t_dn3 = 0.0;
        locals.var_uar_t_dn4 = 0.0;
        locals.var_uar_t_dn5 = 0.0;
        locals.var_uar_t_dn6 = 0.0;
        locals.var_uar_t_dn7 = 0.0;
        locals.var_uar_t_dn8 = 0.0;
        locals.var_uar_t_dn9 = 0.0;
        locals.var_uar_t_dn10 = 0.0;
        locals.var_uar_t_dn11 = 0.0;
        locals.var_uar_t_dn12 = 0.0;
        locals.var_uar_t_dn13 = 0.0;
        locals.var_uar_t_dn14 = 0.0;

        locals.var_ucsr_t = 0.0;
        locals.var_ucsr_t_dn4 = 0.0;

        locals.var_vsatr_i = 0.0;
        locals.var_vsatr_i_dn0 = 0.0;
        locals.var_vsatr_i_dn2 = 0.0;
        locals.var_vsatr_i_dn3 = 0.0;
        locals.var_vsatr_i_dn4 = 0.0;
        locals.var_vsatr_i_dn5 = 0.0;
        locals.var_vsatr_i_dn6 = 0.0;
        locals.var_vsatr_i_dn7 = 0.0;
        locals.var_vsatr_i_dn8 = 0.0;
        locals.var_vsatr_i_dn9 = 0.0;
        locals.var_vsatr_i_dn10 = 0.0;
        locals.var_vsatr_i_dn11 = 0.0;
        locals.var_vsatr_i_dn12 = 0.0;
        locals.var_vsatr_i_dn13 = 0.0;
        locals.var_vsatr_i_dn14 = 0.0;

        locals.var_inv_sb = 0.0;
        locals.var_inv_sb_dn0 = 0.0;
        locals.var_inv_sb_dn2 = 0.0;
        locals.var_inv_sb_dn3 = 0.0;
        locals.var_inv_sb_dn4 = 0.0;
        locals.var_inv_sb_dn5 = 0.0;
        locals.var_inv_sb_dn6 = 0.0;
        locals.var_inv_sb_dn7 = 0.0;
        locals.var_inv_sb_dn8 = 0.0;
        locals.var_inv_sb_dn9 = 0.0;
        locals.var_inv_sb_dn10 = 0.0;
        locals.var_inv_sb_dn11 = 0.0;
        locals.var_inv_sb_dn12 = 0.0;
        locals.var_inv_sb_dn13 = 0.0;
        locals.var_inv_sb_dn14 = 0.0;

        locals.var_local_scb = 0.0;
        locals.var_local_scb_dn0 = 0.0;
        locals.var_local_scb_dn2 = 0.0;
        locals.var_local_scb_dn3 = 0.0;
        locals.var_local_scb_dn4 = 0.0;
        locals.var_local_scb_dn5 = 0.0;
        locals.var_local_scb_dn6 = 0.0;
        locals.var_local_scb_dn7 = 0.0;
        locals.var_local_scb_dn8 = 0.0;
        locals.var_local_scb_dn9 = 0.0;
        locals.var_local_scb_dn10 = 0.0;
        locals.var_local_scb_dn11 = 0.0;
        locals.var_local_scb_dn12 = 0.0;
        locals.var_local_scb_dn13 = 0.0;
        locals.var_local_scb_dn14 = 0.0;

        locals.var_k01_i = 0.0;

        locals.var_citedge_i = 0.0;

        locals.var_etabedge_i = 0.0;

        locals.var_kt1expedge_i = 0.0;

        locals.var_kvth0edge_i = 0.0;

        locals.var_c0_i = 0.0;

        locals.var_c0si1_i = 0.0;

        locals.var_c0sisat_t = 0.0;
        locals.var_c0sisat_t_dn4 = 0.0;

        locals.var_rdstemphv = 1.0;
        locals.var_rdstemphv_dn4 = 0.0;

        locals.var_eta0r_t = 0.0;
        locals.var_eta0r_t_dn0 = 0.0;
        locals.var_eta0r_t_dn2 = 0.0;
        locals.var_eta0r_t_dn3 = 0.0;
        locals.var_eta0r_t_dn4 = 0.0;
        locals.var_eta0r_t_dn5 = 0.0;
        locals.var_eta0r_t_dn6 = 0.0;
        locals.var_eta0r_t_dn7 = 0.0;
        locals.var_eta0r_t_dn8 = 0.0;
        locals.var_eta0r_t_dn9 = 0.0;
        locals.var_eta0r_t_dn10 = 0.0;
        locals.var_eta0r_t_dn11 = 0.0;
        locals.var_eta0r_t_dn12 = 0.0;
        locals.var_eta0r_t_dn13 = 0.0;
        locals.var_eta0r_t_dn14 = 0.0;

        locals.var_pdiblcr_i = 0.0;
        locals.var_pdiblcr_i_dn0 = 0.0;
        locals.var_pdiblcr_i_dn2 = 0.0;
        locals.var_pdiblcr_i_dn3 = 0.0;
        locals.var_pdiblcr_i_dn4 = 0.0;
        locals.var_pdiblcr_i_dn5 = 0.0;
        locals.var_pdiblcr_i_dn6 = 0.0;
        locals.var_pdiblcr_i_dn7 = 0.0;
        locals.var_pdiblcr_i_dn8 = 0.0;
        locals.var_pdiblcr_i_dn9 = 0.0;
        locals.var_pdiblcr_i_dn10 = 0.0;
        locals.var_pdiblcr_i_dn11 = 0.0;
        locals.var_pdiblcr_i_dn12 = 0.0;
        locals.var_pdiblcr_i_dn13 = 0.0;
        locals.var_pdiblcr_i_dn14 = 0.0;

        locals.var_u0r_i = 0.0;

        locals.var_ucr_i = 0.0;
        locals.var_ucr_i_dn0 = 0.0;
        locals.var_ucr_i_dn2 = 0.0;
        locals.var_ucr_i_dn3 = 0.0;
        locals.var_ucr_i_dn4 = 0.0;
        locals.var_ucr_i_dn5 = 0.0;
        locals.var_ucr_i_dn6 = 0.0;
        locals.var_ucr_i_dn7 = 0.0;
        locals.var_ucr_i_dn8 = 0.0;
        locals.var_ucr_i_dn9 = 0.0;
        locals.var_ucr_i_dn10 = 0.0;
        locals.var_ucr_i_dn11 = 0.0;
        locals.var_ucr_i_dn12 = 0.0;
        locals.var_ucr_i_dn13 = 0.0;
        locals.var_ucr_i_dn14 = 0.0;

        locals.var_udr_i = 0.0;
        locals.var_udr_i_dn0 = 0.0;
        locals.var_udr_i_dn2 = 0.0;
        locals.var_udr_i_dn3 = 0.0;
        locals.var_udr_i_dn4 = 0.0;
        locals.var_udr_i_dn5 = 0.0;
        locals.var_udr_i_dn6 = 0.0;
        locals.var_udr_i_dn7 = 0.0;
        locals.var_udr_i_dn8 = 0.0;
        locals.var_udr_i_dn9 = 0.0;
        locals.var_udr_i_dn10 = 0.0;
        locals.var_udr_i_dn11 = 0.0;
        locals.var_udr_i_dn12 = 0.0;
        locals.var_udr_i_dn13 = 0.0;
        locals.var_udr_i_dn14 = 0.0;

        locals.var_vsatr_t = 0.0;
        locals.var_vsatr_t_dn0 = 0.0;
        locals.var_vsatr_t_dn2 = 0.0;
        locals.var_vsatr_t_dn3 = 0.0;
        locals.var_vsatr_t_dn4 = 0.0;
        locals.var_vsatr_t_dn5 = 0.0;
        locals.var_vsatr_t_dn6 = 0.0;
        locals.var_vsatr_t_dn7 = 0.0;
        locals.var_vsatr_t_dn8 = 0.0;
        locals.var_vsatr_t_dn9 = 0.0;
        locals.var_vsatr_t_dn10 = 0.0;
        locals.var_vsatr_t_dn11 = 0.0;
        locals.var_vsatr_t_dn12 = 0.0;
        locals.var_vsatr_t_dn13 = 0.0;
        locals.var_vsatr_t_dn14 = 0.0;

        locals.var_vth0_stress_edge = 0.0;
        locals.var_vth0_stress_edge_dn0 = 0.0;
        locals.var_vth0_stress_edge_dn2 = 0.0;
        locals.var_vth0_stress_edge_dn3 = 0.0;
        locals.var_vth0_stress_edge_dn4 = 0.0;
        locals.var_vth0_stress_edge_dn5 = 0.0;
        locals.var_vth0_stress_edge_dn6 = 0.0;
        locals.var_vth0_stress_edge_dn7 = 0.0;
        locals.var_vth0_stress_edge_dn8 = 0.0;
        locals.var_vth0_stress_edge_dn9 = 0.0;
        locals.var_vth0_stress_edge_dn10 = 0.0;
        locals.var_vth0_stress_edge_dn11 = 0.0;
        locals.var_vth0_stress_edge_dn12 = 0.0;
        locals.var_vth0_stress_edge_dn13 = 0.0;
        locals.var_vth0_stress_edge_dn14 = 0.0;

        locals.var_eta_stress_edge = 0.0;
        locals.var_eta_stress_edge_dn0 = 0.0;
        locals.var_eta_stress_edge_dn2 = 0.0;
        locals.var_eta_stress_edge_dn3 = 0.0;
        locals.var_eta_stress_edge_dn4 = 0.0;
        locals.var_eta_stress_edge_dn5 = 0.0;
        locals.var_eta_stress_edge_dn6 = 0.0;
        locals.var_eta_stress_edge_dn7 = 0.0;
        locals.var_eta_stress_edge_dn8 = 0.0;
        locals.var_eta_stress_edge_dn9 = 0.0;
        locals.var_eta_stress_edge_dn10 = 0.0;
        locals.var_eta_stress_edge_dn11 = 0.0;
        locals.var_eta_stress_edge_dn12 = 0.0;
        locals.var_eta_stress_edge_dn13 = 0.0;
        locals.var_eta_stress_edge_dn14 = 0.0;

        locals.var_local_scc = 0.0;
        locals.var_local_scc_dn0 = 0.0;
        locals.var_local_scc_dn2 = 0.0;
        locals.var_local_scc_dn3 = 0.0;
        locals.var_local_scc_dn4 = 0.0;
        locals.var_local_scc_dn5 = 0.0;
        locals.var_local_scc_dn6 = 0.0;
        locals.var_local_scc_dn7 = 0.0;
        locals.var_local_scc_dn8 = 0.0;
        locals.var_local_scc_dn9 = 0.0;
        locals.var_local_scc_dn10 = 0.0;
        locals.var_local_scc_dn11 = 0.0;
        locals.var_local_scc_dn12 = 0.0;
        locals.var_local_scc_dn13 = 0.0;
        locals.var_local_scc_dn14 = 0.0;

        locals.var_m01_i = 0.0;

        locals.var_cdscdedge_i = 0.0;

        locals.var_kt1edge_i = 0.0;

        locals.var_tnfactoredge_i = 0.0;

        locals.var_stk2edge_i = 0.0;

        locals.var_c01_i = 0.0;

        locals.var_c0si_t = 0.0;
        locals.var_c0si_t_dn4 = 0.0;

        locals.var_rdrift_d = 0.0;
        locals.var_rdrift_d_dn0 = 0.0;
        locals.var_rdrift_d_dn2 = 0.0;
        locals.var_rdrift_d_dn3 = 0.0;
        locals.var_rdrift_d_dn4 = 0.0;
        locals.var_rdrift_d_dn5 = 0.0;
        locals.var_rdrift_d_dn6 = 0.0;
        locals.var_rdrift_d_dn7 = 0.0;
        locals.var_rdrift_d_dn8 = 0.0;
        locals.var_rdrift_d_dn9 = 0.0;
        locals.var_rdrift_d_dn10 = 0.0;
        locals.var_rdrift_d_dn11 = 0.0;
        locals.var_rdrift_d_dn12 = 0.0;
        locals.var_rdrift_d_dn13 = 0.0;
        locals.var_rdrift_d_dn14 = 0.0;

        locals.var_vdrift_t = 1.0;
        locals.var_vdrift_t_dn4 = 0.0;

        locals.var_l_lln1 = 0.0;

        locals.var_psatr_i = 0.0;

        locals.var_u0r_t = 0.0;
        locals.var_u0r_t_dn4 = 0.0;

    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        locals.var_ucr_t = 0.0;
        locals.var_ucr_t_dn0 = 0.0;
        locals.var_ucr_t_dn2 = 0.0;
        locals.var_ucr_t_dn3 = 0.0;
        locals.var_ucr_t_dn4 = 0.0;
        locals.var_ucr_t_dn5 = 0.0;
        locals.var_ucr_t_dn6 = 0.0;
        locals.var_ucr_t_dn7 = 0.0;
        locals.var_ucr_t_dn8 = 0.0;
        locals.var_ucr_t_dn9 = 0.0;
        locals.var_ucr_t_dn10 = 0.0;
        locals.var_ucr_t_dn11 = 0.0;
        locals.var_ucr_t_dn12 = 0.0;
        locals.var_ucr_t_dn13 = 0.0;
        locals.var_ucr_t_dn14 = 0.0;

        locals.var_udr_t = 0.0;
        locals.var_udr_t_dn0 = 0.0;
        locals.var_udr_t_dn2 = 0.0;
        locals.var_udr_t_dn3 = 0.0;
        locals.var_udr_t_dn4 = 0.0;
        locals.var_udr_t_dn5 = 0.0;
        locals.var_udr_t_dn6 = 0.0;
        locals.var_udr_t_dn7 = 0.0;
        locals.var_udr_t_dn8 = 0.0;
        locals.var_udr_t_dn9 = 0.0;
        locals.var_udr_t_dn10 = 0.0;
        locals.var_udr_t_dn11 = 0.0;
        locals.var_udr_t_dn12 = 0.0;
        locals.var_udr_t_dn13 = 0.0;
        locals.var_udr_t_dn14 = 0.0;

        locals.var_w_lwn1 = 0.0;

        locals.var_k2_stress_edge = 0.0;
        locals.var_k2_stress_edge_dn0 = 0.0;
        locals.var_k2_stress_edge_dn2 = 0.0;
        locals.var_k2_stress_edge_dn3 = 0.0;
        locals.var_k2_stress_edge_dn4 = 0.0;
        locals.var_k2_stress_edge_dn5 = 0.0;
        locals.var_k2_stress_edge_dn6 = 0.0;
        locals.var_k2_stress_edge_dn7 = 0.0;
        locals.var_k2_stress_edge_dn8 = 0.0;
        locals.var_k2_stress_edge_dn9 = 0.0;
        locals.var_k2_stress_edge_dn10 = 0.0;
        locals.var_k2_stress_edge_dn11 = 0.0;
        locals.var_k2_stress_edge_dn12 = 0.0;
        locals.var_k2_stress_edge_dn13 = 0.0;
        locals.var_k2_stress_edge_dn14 = 0.0;

        locals.var_k0_i = 0.0;

        locals.var_k0_t = 0.0;
        locals.var_k0_t_dn4 = 0.0;

        locals.var_cdscbedge_i = 0.0;

        locals.var_kt1ledge_i = 0.0;

        locals.var_teta0edge_i = 0.0;

        locals.var_steta0edge_i = 0.0;

        locals.var_c0_t = 0.0;
        locals.var_c0_t_dn4 = 0.0;

        locals.var_c0sisat_i = 0.0;

        locals.var_rdrift_s = 0.0;
        locals.var_rdrift_s_dn0 = 0.0;
        locals.var_rdrift_s_dn2 = 0.0;
        locals.var_rdrift_s_dn3 = 0.0;
        locals.var_rdrift_s_dn4 = 0.0;
        locals.var_rdrift_s_dn5 = 0.0;
        locals.var_rdrift_s_dn6 = 0.0;
        locals.var_rdrift_s_dn7 = 0.0;
        locals.var_rdrift_s_dn8 = 0.0;
        locals.var_rdrift_s_dn9 = 0.0;
        locals.var_rdrift_s_dn10 = 0.0;
        locals.var_rdrift_s_dn11 = 0.0;
        locals.var_rdrift_s_dn12 = 0.0;
        locals.var_rdrift_s_dn13 = 0.0;
        locals.var_rdrift_s_dn14 = 0.0;

        locals.var_k2edgewe_i = 0.0;

        locals.var_kvth0edgewe_i = 0.0;

        locals.var_temp_adeff = 0.0;
        locals.var_temp_adeff_dn0 = 0.0;
        locals.var_temp_adeff_dn2 = 0.0;
        locals.var_temp_adeff_dn3 = 0.0;
        locals.var_temp_adeff_dn4 = 0.0;
        locals.var_temp_adeff_dn5 = 0.0;
        locals.var_temp_adeff_dn6 = 0.0;
        locals.var_temp_adeff_dn7 = 0.0;
        locals.var_temp_adeff_dn8 = 0.0;
        locals.var_temp_adeff_dn9 = 0.0;
        locals.var_temp_adeff_dn10 = 0.0;
        locals.var_temp_adeff_dn11 = 0.0;
        locals.var_temp_adeff_dn12 = 0.0;
        locals.var_temp_adeff_dn13 = 0.0;
        locals.var_temp_adeff_dn14 = 0.0;

        locals.var_temp_aseff = 0.0;
        locals.var_temp_aseff_dn0 = 0.0;
        locals.var_temp_aseff_dn2 = 0.0;
        locals.var_temp_aseff_dn3 = 0.0;
        locals.var_temp_aseff_dn4 = 0.0;
        locals.var_temp_aseff_dn5 = 0.0;
        locals.var_temp_aseff_dn6 = 0.0;
        locals.var_temp_aseff_dn7 = 0.0;
        locals.var_temp_aseff_dn8 = 0.0;
        locals.var_temp_aseff_dn9 = 0.0;
        locals.var_temp_aseff_dn10 = 0.0;
        locals.var_temp_aseff_dn11 = 0.0;
        locals.var_temp_aseff_dn12 = 0.0;
        locals.var_temp_aseff_dn13 = 0.0;
        locals.var_temp_aseff_dn14 = 0.0;

        locals.var_temp_pdeff = 0.0;
        locals.var_temp_pdeff_dn0 = 0.0;
        locals.var_temp_pdeff_dn2 = 0.0;
        locals.var_temp_pdeff_dn3 = 0.0;
        locals.var_temp_pdeff_dn4 = 0.0;
        locals.var_temp_pdeff_dn5 = 0.0;
        locals.var_temp_pdeff_dn6 = 0.0;
        locals.var_temp_pdeff_dn7 = 0.0;
        locals.var_temp_pdeff_dn8 = 0.0;
        locals.var_temp_pdeff_dn9 = 0.0;
        locals.var_temp_pdeff_dn10 = 0.0;
        locals.var_temp_pdeff_dn11 = 0.0;
        locals.var_temp_pdeff_dn12 = 0.0;
        locals.var_temp_pdeff_dn13 = 0.0;
        locals.var_temp_pdeff_dn14 = 0.0;

        locals.var_temp_pseff = 0.0;
        locals.var_temp_pseff_dn0 = 0.0;
        locals.var_temp_pseff_dn2 = 0.0;
        locals.var_temp_pseff_dn3 = 0.0;
        locals.var_temp_pseff_dn4 = 0.0;
        locals.var_temp_pseff_dn5 = 0.0;
        locals.var_temp_pseff_dn6 = 0.0;
        locals.var_temp_pseff_dn7 = 0.0;
        locals.var_temp_pseff_dn8 = 0.0;
        locals.var_temp_pseff_dn9 = 0.0;
        locals.var_temp_pseff_dn10 = 0.0;
        locals.var_temp_pseff_dn11 = 0.0;
        locals.var_temp_pseff_dn12 = 0.0;
        locals.var_temp_pseff_dn13 = 0.0;
        locals.var_temp_pseff_dn14 = 0.0;

        locals.var_abulkiv = 1.0;
        locals.var_abulkiv_dn0 = 0.0;
        locals.var_abulkiv_dn2 = 0.0;
        locals.var_abulkiv_dn3 = 0.0;
        locals.var_abulkiv_dn4 = 0.0;
        locals.var_abulkiv_dn5 = 0.0;
        locals.var_abulkiv_dn6 = 0.0;
        locals.var_abulkiv_dn7 = 0.0;
        locals.var_abulkiv_dn8 = 0.0;
        locals.var_abulkiv_dn9 = 0.0;
        locals.var_abulkiv_dn10 = 0.0;
        locals.var_abulkiv_dn11 = 0.0;
        locals.var_abulkiv_dn12 = 0.0;
        locals.var_abulkiv_dn13 = 0.0;
        locals.var_abulkiv_dn14 = 0.0;

        locals.var_abulkcv = 1.0;
        locals.var_abulkcv_dn0 = 0.0;
        locals.var_abulkcv_dn2 = 0.0;
        locals.var_abulkcv_dn3 = 0.0;
        locals.var_abulkcv_dn4 = 0.0;
        locals.var_abulkcv_dn5 = 0.0;
        locals.var_abulkcv_dn6 = 0.0;
        locals.var_abulkcv_dn7 = 0.0;
        locals.var_abulkcv_dn8 = 0.0;
        locals.var_abulkcv_dn9 = 0.0;
        locals.var_abulkcv_dn10 = 0.0;
        locals.var_abulkcv_dn11 = 0.0;
        locals.var_abulkcv_dn12 = 0.0;
        locals.var_abulkcv_dn13 = 0.0;
        locals.var_abulkcv_dn14 = 0.0;

        locals.var_gdpr = 0.0;
        locals.var_gdpr_dn0 = 0.0;
        locals.var_gdpr_dn2 = 0.0;
        locals.var_gdpr_dn3 = 0.0;
        locals.var_gdpr_dn4 = 0.0;
        locals.var_gdpr_dn5 = 0.0;
        locals.var_gdpr_dn6 = 0.0;
        locals.var_gdpr_dn7 = 0.0;
        locals.var_gdpr_dn8 = 0.0;
        locals.var_gdpr_dn9 = 0.0;
        locals.var_gdpr_dn10 = 0.0;
        locals.var_gdpr_dn11 = 0.0;
        locals.var_gdpr_dn12 = 0.0;
        locals.var_gdpr_dn13 = 0.0;
        locals.var_gdpr_dn14 = 0.0;

        locals.var_gspr = 0.0;
        locals.var_gspr_dn0 = 0.0;
        locals.var_gspr_dn2 = 0.0;
        locals.var_gspr_dn3 = 0.0;
        locals.var_gspr_dn4 = 0.0;
        locals.var_gspr_dn5 = 0.0;
        locals.var_gspr_dn6 = 0.0;
        locals.var_gspr_dn7 = 0.0;
        locals.var_gspr_dn8 = 0.0;
        locals.var_gspr_dn9 = 0.0;
        locals.var_gspr_dn10 = 0.0;
        locals.var_gspr_dn11 = 0.0;
        locals.var_gspr_dn12 = 0.0;
        locals.var_gspr_dn13 = 0.0;
        locals.var_gspr_dn14 = 0.0;

        locals.var_gdrift_d = 0.0;
        locals.var_gdrift_d_dn0 = 0.0;
        locals.var_gdrift_d_dn2 = 0.0;
        locals.var_gdrift_d_dn3 = 0.0;
        locals.var_gdrift_d_dn4 = 0.0;
        locals.var_gdrift_d_dn5 = 0.0;
        locals.var_gdrift_d_dn6 = 0.0;
        locals.var_gdrift_d_dn7 = 0.0;
        locals.var_gdrift_d_dn8 = 0.0;
        locals.var_gdrift_d_dn9 = 0.0;
        locals.var_gdrift_d_dn10 = 0.0;
        locals.var_gdrift_d_dn11 = 0.0;
        locals.var_gdrift_d_dn12 = 0.0;
        locals.var_gdrift_d_dn13 = 0.0;
        locals.var_gdrift_d_dn14 = 0.0;

        locals.var_gdrift_s = 0.0;
        locals.var_gdrift_s_dn0 = 0.0;
        locals.var_gdrift_s_dn2 = 0.0;
        locals.var_gdrift_s_dn3 = 0.0;
        locals.var_gdrift_s_dn4 = 0.0;
        locals.var_gdrift_s_dn5 = 0.0;
        locals.var_gdrift_s_dn6 = 0.0;
        locals.var_gdrift_s_dn7 = 0.0;
        locals.var_gdrift_s_dn8 = 0.0;
        locals.var_gdrift_s_dn9 = 0.0;
        locals.var_gdrift_s_dn10 = 0.0;
        locals.var_gdrift_s_dn11 = 0.0;
        locals.var_gdrift_s_dn12 = 0.0;
        locals.var_gdrift_s_dn13 = 0.0;
        locals.var_gdrift_s_dn14 = 0.0;

        locals.var_vd1 = 0.0;
        locals.var_vd1_dn6 = 0.0;
        locals.var_vd1_dn11 = 0.0;

        locals.var_vs1 = 0.0;
        locals.var_vs1_dn8 = 0.0;
        locals.var_vs1_dn11 = 0.0;

        locals.var_idrift_sat_d = 0.0;
        locals.var_idrift_sat_d_dn0 = 0.0;
        locals.var_idrift_sat_d_dn2 = 0.0;
        locals.var_idrift_sat_d_dn3 = 0.0;
        locals.var_idrift_sat_d_dn4 = 0.0;
        locals.var_idrift_sat_d_dn5 = 0.0;
        locals.var_idrift_sat_d_dn6 = 0.0;
        locals.var_idrift_sat_d_dn7 = 0.0;
        locals.var_idrift_sat_d_dn8 = 0.0;
        locals.var_idrift_sat_d_dn9 = 0.0;
        locals.var_idrift_sat_d_dn10 = 0.0;
        locals.var_idrift_sat_d_dn11 = 0.0;
        locals.var_idrift_sat_d_dn12 = 0.0;
        locals.var_idrift_sat_d_dn13 = 0.0;
        locals.var_idrift_sat_d_dn14 = 0.0;

        locals.var_ln_t1_t2 = 0.0;
        locals.var_ln_t1_t2_dn0 = 0.0;
        locals.var_ln_t1_t2_dn2 = 0.0;
        locals.var_ln_t1_t2_dn3 = 0.0;
        locals.var_ln_t1_t2_dn4 = 0.0;
        locals.var_ln_t1_t2_dn5 = 0.0;
        locals.var_ln_t1_t2_dn6 = 0.0;
        locals.var_ln_t1_t2_dn7 = 0.0;
        locals.var_ln_t1_t2_dn8 = 0.0;
        locals.var_ln_t1_t2_dn9 = 0.0;
        locals.var_ln_t1_t2_dn10 = 0.0;
        locals.var_ln_t1_t2_dn11 = 0.0;
        locals.var_ln_t1_t2_dn12 = 0.0;
        locals.var_ln_t1_t2_dn13 = 0.0;
        locals.var_ln_t1_t2_dn14 = 0.0;

        locals.var_iii = 0.0;
        locals.var_iii_dn0 = 0.0;
        locals.var_iii_dn2 = 0.0;
        locals.var_iii_dn3 = 0.0;
        locals.var_iii_dn4 = 0.0;
        locals.var_iii_dn5 = 0.0;
        locals.var_iii_dn6 = 0.0;
        locals.var_iii_dn7 = 0.0;
        locals.var_iii_dn8 = 0.0;
        locals.var_iii_dn9 = 0.0;
        locals.var_iii_dn10 = 0.0;
        locals.var_iii_dn11 = 0.0;
        locals.var_iii_dn12 = 0.0;
        locals.var_iii_dn13 = 0.0;
        locals.var_iii_dn14 = 0.0;

        locals.var_vdseffii = 0.0;
        locals.var_vdseffii_dn0 = 0.0;
        locals.var_vdseffii_dn2 = 0.0;
        locals.var_vdseffii_dn3 = 0.0;
        locals.var_vdseffii_dn4 = 0.0;
        locals.var_vdseffii_dn5 = 0.0;
        locals.var_vdseffii_dn6 = 0.0;
        locals.var_vdseffii_dn7 = 0.0;
        locals.var_vdseffii_dn8 = 0.0;
        locals.var_vdseffii_dn9 = 0.0;
        locals.var_vdseffii_dn10 = 0.0;
        locals.var_vdseffii_dn11 = 0.0;
        locals.var_vdseffii_dn12 = 0.0;
        locals.var_vdseffii_dn13 = 0.0;
        locals.var_vdseffii_dn14 = 0.0;

        locals.var_beta0r_t = 0.0;
        locals.var_beta0r_t_dn4 = 0.0;

        locals.var_alpha0r_i = 0.0;
        locals.var_alpha0r_i_dn0 = 0.0;
        locals.var_alpha0r_i_dn2 = 0.0;
        locals.var_alpha0r_i_dn3 = 0.0;
        locals.var_alpha0r_i_dn4 = 0.0;
        locals.var_alpha0r_i_dn5 = 0.0;
        locals.var_alpha0r_i_dn6 = 0.0;
        locals.var_alpha0r_i_dn7 = 0.0;
        locals.var_alpha0r_i_dn8 = 0.0;
        locals.var_alpha0r_i_dn9 = 0.0;
        locals.var_alpha0r_i_dn10 = 0.0;
        locals.var_alpha0r_i_dn11 = 0.0;
        locals.var_alpha0r_i_dn12 = 0.0;
        locals.var_alpha0r_i_dn13 = 0.0;
        locals.var_alpha0r_i_dn14 = 0.0;

        locals.var_beta0r_i = 0.0;

        locals.var_vb_cm = 0.0;
        locals.var_vb_cm_dn3 = 0.0;
        locals.var_vb_cm_dn11 = 0.0;

        let assign940_e2092: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign940_e2092;

        let (assign950_e2096,) = {
    if (locals.var_guard1 != 0.0) {
        (1.0,)
    } else {
        (locals.var_devsign,)
    }
};
        locals.var_devsign = assign950_e2096;

        let (assign960_e2102,) = {
    if (locals.var_guard1 == 0.0) {
        let assign960_e2100: f64 = (-1.0);
        (assign960_e2100,)
    } else {
        (locals.var_devsign,)
    }
};
        locals.var_devsign = assign960_e2102;

        let assign970_e2105: f64 = (p.p110 * 8.85418e-12);
        locals.var_epssi = assign970_e2105;

        let assign980_e2108: f64 = (p.p111 * 8.85418e-12);
        locals.var_epsox = assign980_e2108;

        let assign990_e2111: f64 = (p.p111 * 8.85418e-12);
        let assign990_e2113: f64 = (assign990_e2111 / p.p77);
        locals.var_cox = assign990_e2113;

        let assign1000_e2116: f64 = (p.p110 / p.p111);
        locals.var_epsratio = assign1000_e2116;

        let assign1010_e2119: f64 = if (!param_given[78]) { 1.0 } else { 0.0 };
        locals.var_guard2 = assign1010_e2119;

        let (assign1020_e2129,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1020_e2123: f64 = (p.p77 * p.p111);
        let assign1020_e2125: f64 = (assign1020_e2123 / 3.9);
        let assign1020_e2127: f64 = (assign1020_e2125 - p.p79);
        (assign1020_e2127,)
    } else {
        (locals.var_bsimbulktoxp,)
    }
};
        locals.var_bsimbulktoxp = assign1020_e2129;

        let (assign1030_e2134,) = {
    if (locals.var_guard2 == 0.0) {
        (p.p78,)
    } else {
        (locals.var_bsimbulktoxp,)
    }
};
        locals.var_bsimbulktoxp = assign1030_e2134;

        let assign1040_e2137: f64 = (p.p0 * p.p52);
        locals.var_l_mult = assign1040_e2137;

        let assign1050_e2140: f64 = (p.p1 * p.p53);
        locals.var_w_mult = assign1050_e2140;

        let assign1060_e2143: f64 = (locals.var_l_mult + p.p54);
        locals.var_lnew = assign1060_e2143;

        let assign1080_e2149: f64 = (locals.var_w_mult / p.p2);
        locals.var_w_by_nf = assign1080_e2149;

        let assign1090_e2152: f64 = (locals.var_w_by_nf + p.p56);
        locals.var_wnew = assign1090_e2152;

        let assign1110_e2158: f64 = (-p.p61);
        let assign1110_e2159: f64 = (locals.var_lnew).powf(assign1110_e2158);
        locals.var_l_lln = assign1110_e2159;

        let assign1120_e2162: f64 = (-p.p62);
        let assign1120_e2163: f64 = (locals.var_wnew).powf(assign1120_e2162);
        locals.var_w_lwn = assign1120_e2163;

        let assign1130_e2166: f64 = (locals.var_l_lln * locals.var_w_lwn);
        locals.var_lw_lln_lwn = assign1130_e2166;

        let assign1140_e2170: f64 = (p.p58 * locals.var_l_lln);
        let assign1140_e2171: f64 = (p.p57 + assign1140_e2170);
        let assign1140_e2174: f64 = (p.p59 * locals.var_w_lwn);
        let assign1140_e2175: f64 = (assign1140_e2171 + assign1140_e2174);
        let assign1140_e2178: f64 = (p.p60 * locals.var_lw_lln_lwn);
        let assign1140_e2179: f64 = (assign1140_e2175 + assign1140_e2178);
        locals.var_dliv = assign1140_e2179;

        let assign1150_e2182: f64 = (-p.p67);
        let assign1150_e2183: f64 = (locals.var_lnew).powf(assign1150_e2182);
        locals.var_l_wln = assign1150_e2183;

        let assign1160_e2186: f64 = (-p.p68);
        let assign1160_e2187: f64 = (locals.var_wnew).powf(assign1160_e2186);
        locals.var_w_wwn = assign1160_e2187;

        let assign1170_e2190: f64 = (locals.var_l_wln * locals.var_w_wwn);
        locals.var_lw_wln_wwn = assign1170_e2190;

        let assign1180_e2194: f64 = (p.p64 * locals.var_l_wln);
        let assign1180_e2195: f64 = (p.p63 + assign1180_e2194);
        let assign1180_e2198: f64 = (p.p65 * locals.var_w_wwn);
        let assign1180_e2199: f64 = (assign1180_e2195 + assign1180_e2198);
        let assign1180_e2202: f64 = (p.p66 * locals.var_lw_wln_wwn);
        let assign1180_e2203: f64 = (assign1180_e2199 + assign1180_e2202);
        locals.var_dwiv = assign1180_e2203;

        let assign1190_e2207: f64 = (2.0 * locals.var_dliv);
        let assign1190_e2208: f64 = (locals.var_lnew - assign1190_e2207);
        locals.var_leff = assign1190_e2208;

        let assign1220_e2218: f64 = (2.0 * locals.var_dwiv);
        let assign1220_e2219: f64 = (locals.var_wnew - assign1220_e2218);
        locals.var_weff = assign1220_e2219;

        let assign1250_e2229: f64 = (p.p70 * locals.var_l_lln);
        let assign1250_e2230: f64 = (p.p69 + assign1250_e2229);
        let assign1250_e2233: f64 = (p.p71 * locals.var_w_lwn);
        let assign1250_e2234: f64 = (assign1250_e2230 + assign1250_e2233);
        let assign1250_e2237: f64 = (p.p72 * locals.var_lw_lln_lwn);
        let assign1250_e2238: f64 = (assign1250_e2234 + assign1250_e2237);
        locals.var_dlcv = assign1250_e2238;

        let assign1260_e2242: f64 = (p.p74 * locals.var_l_wln);
        let assign1260_e2243: f64 = (p.p73 + assign1260_e2242);
        let assign1260_e2246: f64 = (p.p75 * locals.var_w_wwn);
        let assign1260_e2247: f64 = (assign1260_e2243 + assign1260_e2246);
        let assign1260_e2250: f64 = (p.p76 * locals.var_lw_wln_wwn);
        let assign1260_e2251: f64 = (assign1260_e2247 + assign1260_e2250);
        locals.var_dwcv = assign1260_e2251;

        let assign1270_e2255: f64 = (2.0 * locals.var_dlcv);
        let assign1270_e2256: f64 = (locals.var_lnew - assign1270_e2255);
        locals.var_lact = assign1270_e2256;

        let assign1300_e2266: f64 = (2.0 * locals.var_dwcv);
        let assign1300_e2267: f64 = (locals.var_wnew - assign1300_e2266);
        locals.var_wact = assign1300_e2267;

        let assign1330_e2278: f64 = (locals.var_lnew).powf(p.p67);
        let assign1330_e2279: f64 = (p.p74 / assign1330_e2278);
        let assign1330_e2280: f64 = (p.p138 + assign1330_e2279);
        let assign1330_e2284: f64 = (locals.var_wnew).powf(p.p68);
        let assign1330_e2285: f64 = (p.p75 / assign1330_e2284);
        let assign1330_e2286: f64 = (assign1330_e2280 + assign1330_e2285);
        let assign1330_e2290: f64 = (locals.var_lnew).powf(p.p67);
        let assign1330_e2291: f64 = (p.p76 / assign1330_e2290);
        let assign1330_e2294: f64 = (locals.var_wnew).powf(p.p68);
        let assign1330_e2295: f64 = (assign1330_e2291 / assign1330_e2294);
        let assign1330_e2296: f64 = (assign1330_e2286 + assign1330_e2295);
        locals.var_dwj = assign1330_e2296;

        let assign1340_e2300: f64 = (2.0 * locals.var_dwj);
        let assign1340_e2301: f64 = (locals.var_wnew - assign1340_e2300);
        locals.var_weffcj = assign1340_e2301;

        let assign1360_e2307: f64 = (1e-6 / locals.var_leff);
        locals.var_inv_l = assign1360_e2307;

        let assign1370_e2310: f64 = (1e-6 / locals.var_weff);
        locals.var_inv_w = assign1370_e2310;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign1380_e2313: f64 = (1e-6 / locals.var_lact);
        locals.var_inv_lact = assign1380_e2313;

        let assign1390_e2316: f64 = (1e-6 / locals.var_wact);
        locals.var_inv_wact = assign1390_e2316;

        let assign1400_e2319: f64 = (1e-6 / p.p51);
        locals.var_inv_llong = assign1400_e2319;

        let assign1410_e2322: f64 = (1e-6 / p.p55);
        locals.var_inv_wwide = assign1410_e2322;

        let assign1420_e2325: f64 = (locals.var_inv_l * locals.var_inv_w);
        locals.var_inv_wl = assign1420_e2325;

        locals.var_l_lln1 = locals.var_l_lln;

        locals.var_l_wln1 = locals.var_l_wln;

        let assign1450_e2330: f64 = if p.p818 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign1450_e2330;

        let assign1460_e2333: f64 = (-locals.var_lnew);
        let assign1460_e2334: f64 = if p.p818 <= assign1460_e2333 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign1460_e2334;

        let (assign1470_e2346,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 == 0.0)) {
        let assign1470_e2341: f64 = (locals.var_lnew + p.p818);
        let assign1470_e2343: f64 = (-p.p61);
        let assign1470_e2344: f64 = (assign1470_e2341).powf(assign1470_e2343);
        (assign1470_e2344,)
    } else {
        (locals.var_l_lln1,)
    }
};
        locals.var_l_lln1 = assign1470_e2346;

        let (assign1480_e2358,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 == 0.0)) {
        let assign1480_e2353: f64 = (locals.var_lnew + p.p818);
        let assign1480_e2355: f64 = (-p.p67);
        let assign1480_e2356: f64 = (assign1480_e2353).powf(assign1480_e2355);
        (assign1480_e2356,)
    } else {
        (locals.var_l_wln1,)
    }
};
        locals.var_l_wln1 = assign1480_e2358;

        locals.var_w_lwn1 = locals.var_w_lwn;

        locals.var_w_wwn1 = locals.var_w_wwn;

        let assign1510_e2363: f64 = if p.p819 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1510_e2363;

        let assign1520_e2366: f64 = (-locals.var_wnew);
        let assign1520_e2367: f64 = if p.p819 <= assign1520_e2366 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1520_e2367;

        let (assign1530_e2379,) = {
    if ((locals.var_guard16 != 0.0) && (locals.var_guard17 == 0.0)) {
        let assign1530_e2374: f64 = (locals.var_wnew + p.p819);
        let assign1530_e2376: f64 = (-p.p62);
        let assign1530_e2377: f64 = (assign1530_e2374).powf(assign1530_e2376);
        (assign1530_e2377,)
    } else {
        (locals.var_w_lwn1,)
    }
};
        locals.var_w_lwn1 = assign1530_e2379;

        let (assign1540_e2391,) = {
    if ((locals.var_guard16 != 0.0) && (locals.var_guard17 == 0.0)) {
        let assign1540_e2386: f64 = (locals.var_wnew + p.p819);
        let assign1540_e2388: f64 = (-p.p68);
        let assign1540_e2389: f64 = (assign1540_e2386).powf(assign1540_e2388);
        (assign1540_e2389,)
    } else {
        (locals.var_w_wwn1,)
    }
};
        locals.var_w_wwn1 = assign1540_e2391;

        let assign1550_e2394: f64 = (locals.var_l_lln1 * locals.var_w_lwn1);
        locals.var_lw_lln_lwn1 = assign1550_e2394;

        let assign1560_e2398: f64 = (p.p58 * locals.var_l_lln1);
        let assign1560_e2399: f64 = (p.p57 + assign1560_e2398);
        let assign1560_e2402: f64 = (p.p59 * locals.var_w_lwn1);
        let assign1560_e2403: f64 = (assign1560_e2399 + assign1560_e2402);
        let assign1560_e2406: f64 = (p.p60 * locals.var_lw_lln_lwn1);
        let assign1560_e2407: f64 = (assign1560_e2403 + assign1560_e2406);
        locals.var_dlb = assign1560_e2407;

        let assign1570_e2410: f64 = (locals.var_l_wln1 * locals.var_w_wwn1);
        locals.var_lw_wln_wwn1 = assign1570_e2410;

        let assign1580_e2414: f64 = (p.p64 * locals.var_l_wln1);
        let assign1580_e2415: f64 = (p.p63 + assign1580_e2414);
        let assign1580_e2418: f64 = (p.p65 * locals.var_w_wwn1);
        let assign1580_e2419: f64 = (assign1580_e2415 + assign1580_e2418);
        let assign1580_e2422: f64 = (p.p66 * locals.var_lw_wln_wwn1);
        let assign1580_e2423: f64 = (assign1580_e2419 + assign1580_e2422);
        locals.var_dwb = assign1580_e2423;

        let assign1590_e2427: f64 = (2.0 * locals.var_dlb);
        let assign1590_e2428: f64 = (locals.var_lnew - assign1590_e2427);
        let assign1590_e2430: f64 = (assign1590_e2428 + p.p818);
        locals.var_leff1 = assign1590_e2430;

        let assign1610_e2437: f64 = (2.0 * locals.var_dwb);
        let assign1610_e2438: f64 = (locals.var_wnew - assign1610_e2437);
        let assign1610_e2440: f64 = (assign1610_e2438 + p.p819);
        locals.var_weff1 = assign1610_e2440;

        let assign1630_e2446: f64 = if p.p817 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign1630_e2446;

        let (assign1640_e2452,) = {
    if (locals.var_guard20 != 0.0) {
        let assign1640_e2450: f64 = (1e-6 / locals.var_leff1);
        (assign1640_e2450,)
    } else {
        (locals.var_bin_l,)
    }
};
        locals.var_bin_l = assign1640_e2452;

        let (assign1650_e2458,) = {
    if (locals.var_guard20 != 0.0) {
        let assign1650_e2456: f64 = (1e-6 / locals.var_weff1);
        (assign1650_e2456,)
    } else {
        (locals.var_bin_w,)
    }
};
        locals.var_bin_w = assign1650_e2458;

        let (assign1660_e2465,) = {
    if (locals.var_guard20 == 0.0) {
        let assign1660_e2463: f64 = (1.0 / locals.var_leff1);
        (assign1660_e2463,)
    } else {
        (locals.var_bin_l,)
    }
};
        locals.var_bin_l = assign1660_e2465;

        let (assign1670_e2472,) = {
    if (locals.var_guard20 == 0.0) {
        let assign1670_e2470: f64 = (1.0 / locals.var_weff1);
        (assign1670_e2470,)
    } else {
        (locals.var_bin_w,)
    }
};
        locals.var_bin_w = assign1670_e2472;

        let assign1680_e2475: f64 = (locals.var_bin_l * locals.var_bin_w);
        locals.var_bin_wl = assign1680_e2475;

        let assign1690_e2479: f64 = (locals.var_bin_l * p.p117);
        let assign1690_e2480: f64 = (p.p116 + assign1690_e2479);
        let assign1690_e2483: f64 = (locals.var_bin_w * p.p118);
        let assign1690_e2484: f64 = (assign1690_e2480 + assign1690_e2483);
        let assign1690_e2487: f64 = (locals.var_bin_wl * p.p119);
        let assign1690_e2488: f64 = (assign1690_e2484 + assign1690_e2487);
        locals.var_vfb_i = assign1690_e2488;
        locals.var_vfb_i_dn0 = 0.0;
        locals.var_vfb_i_dn2 = 0.0;
        locals.var_vfb_i_dn3 = 0.0;
        locals.var_vfb_i_dn4 = 0.0;
        locals.var_vfb_i_dn5 = 0.0;
        locals.var_vfb_i_dn6 = 0.0;
        locals.var_vfb_i_dn7 = 0.0;
        locals.var_vfb_i_dn8 = 0.0;
        locals.var_vfb_i_dn9 = 0.0;
        locals.var_vfb_i_dn10 = 0.0;
        locals.var_vfb_i_dn11 = 0.0;
        locals.var_vfb_i_dn12 = 0.0;
        locals.var_vfb_i_dn13 = 0.0;
        locals.var_vfb_i_dn14 = 0.0;

        let assign1700_e2492: f64 = (locals.var_bin_l * p.p127);
        let assign1700_e2493: f64 = (p.p126 + assign1700_e2492);
        let assign1700_e2496: f64 = (locals.var_bin_w * p.p128);
        let assign1700_e2497: f64 = (assign1700_e2493 + assign1700_e2496);
        let assign1700_e2500: f64 = (locals.var_bin_wl * p.p129);
        let assign1700_e2501: f64 = (assign1700_e2497 + assign1700_e2500);
        locals.var_vfbcv_i = assign1700_e2501;
        locals.var_vfbcv_i_dn0 = 0.0;
        locals.var_vfbcv_i_dn2 = 0.0;
        locals.var_vfbcv_i_dn3 = 0.0;
        locals.var_vfbcv_i_dn4 = 0.0;
        locals.var_vfbcv_i_dn5 = 0.0;
        locals.var_vfbcv_i_dn6 = 0.0;
        locals.var_vfbcv_i_dn7 = 0.0;
        locals.var_vfbcv_i_dn8 = 0.0;
        locals.var_vfbcv_i_dn9 = 0.0;
        locals.var_vfbcv_i_dn10 = 0.0;
        locals.var_vfbcv_i_dn11 = 0.0;
        locals.var_vfbcv_i_dn12 = 0.0;
        locals.var_vfbcv_i_dn13 = 0.0;
        locals.var_vfbcv_i_dn14 = 0.0;

        let assign1710_e2505: f64 = (locals.var_bin_l * p.p140);
        let assign1710_e2506: f64 = (p.p139 + assign1710_e2505);
        let assign1710_e2509: f64 = (locals.var_bin_w * p.p141);
        let assign1710_e2510: f64 = (assign1710_e2506 + assign1710_e2509);
        let assign1710_e2513: f64 = (locals.var_bin_wl * p.p142);
        let assign1710_e2514: f64 = (assign1710_e2510 + assign1710_e2513);
        locals.var_nsd_i = assign1710_e2514;

        let assign1720_e2518: f64 = (locals.var_bin_l * p.p89);
        let assign1720_e2519: f64 = (p.p80 + assign1720_e2518);
        let assign1720_e2522: f64 = (locals.var_bin_w * p.p90);
        let assign1720_e2523: f64 = (assign1720_e2519 + assign1720_e2522);
        let assign1720_e2526: f64 = (locals.var_bin_wl * p.p91);
        let assign1720_e2527: f64 = (assign1720_e2523 + assign1720_e2526);
        locals.var_ndep_i = assign1720_e2527;
        locals.var_ndep_i_dn0 = 0.0;
        locals.var_ndep_i_dn2 = 0.0;
        locals.var_ndep_i_dn3 = 0.0;
        locals.var_ndep_i_dn4 = 0.0;
        locals.var_ndep_i_dn5 = 0.0;
        locals.var_ndep_i_dn6 = 0.0;
        locals.var_ndep_i_dn7 = 0.0;
        locals.var_ndep_i_dn8 = 0.0;
        locals.var_ndep_i_dn9 = 0.0;
        locals.var_ndep_i_dn10 = 0.0;
        locals.var_ndep_i_dn11 = 0.0;
        locals.var_ndep_i_dn12 = 0.0;
        locals.var_ndep_i_dn13 = 0.0;
        locals.var_ndep_i_dn14 = 0.0;

        let assign1730_e2531: f64 = (locals.var_bin_l * p.p101);
        let assign1730_e2532: f64 = (p.p92 + assign1730_e2531);
        let assign1730_e2535: f64 = (locals.var_bin_w * p.p102);
        let assign1730_e2536: f64 = (assign1730_e2532 + assign1730_e2535);
        let assign1730_e2539: f64 = (locals.var_bin_wl * p.p103);
        let assign1730_e2540: f64 = (assign1730_e2536 + assign1730_e2539);
        locals.var_ndepcv_i = assign1730_e2540;
        locals.var_ndepcv_i_dn0 = 0.0;
        locals.var_ndepcv_i_dn2 = 0.0;
        locals.var_ndepcv_i_dn3 = 0.0;
        locals.var_ndepcv_i_dn4 = 0.0;
        locals.var_ndepcv_i_dn5 = 0.0;
        locals.var_ndepcv_i_dn6 = 0.0;
        locals.var_ndepcv_i_dn7 = 0.0;
        locals.var_ndepcv_i_dn8 = 0.0;
        locals.var_ndepcv_i_dn9 = 0.0;
        locals.var_ndepcv_i_dn10 = 0.0;
        locals.var_ndepcv_i_dn11 = 0.0;
        locals.var_ndepcv_i_dn12 = 0.0;
        locals.var_ndepcv_i_dn13 = 0.0;
        locals.var_ndepcv_i_dn14 = 0.0;

        let assign1740_e2544: f64 = (locals.var_bin_l * p.p105);
        let assign1740_e2545: f64 = (p.p104 + assign1740_e2544);
        let assign1740_e2548: f64 = (locals.var_bin_w * p.p106);
        let assign1740_e2549: f64 = (assign1740_e2545 + assign1740_e2548);
        let assign1740_e2552: f64 = (locals.var_bin_wl * p.p107);
        let assign1740_e2553: f64 = (assign1740_e2549 + assign1740_e2552);
        locals.var_ngate_i = assign1740_e2553;

        let assign1750_e2557: f64 = (locals.var_bin_l * p.p210);
        let assign1750_e2558: f64 = (p.p209 + assign1750_e2557);
        let assign1750_e2561: f64 = (locals.var_bin_w * p.p211);
        let assign1750_e2562: f64 = (assign1750_e2558 + assign1750_e2561);
        let assign1750_e2565: f64 = (locals.var_bin_wl * p.p212);
        let assign1750_e2566: f64 = (assign1750_e2562 + assign1750_e2565);
        locals.var_cit_i = assign1750_e2566;

        let assign1760_e2570: f64 = (locals.var_bin_l * p.p220);
        let assign1760_e2571: f64 = (p.p213 + assign1760_e2570);
        let assign1760_e2574: f64 = (locals.var_bin_w * p.p221);
        let assign1760_e2575: f64 = (assign1760_e2571 + assign1760_e2574);
        let assign1760_e2578: f64 = (locals.var_bin_wl * p.p222);
        let assign1760_e2579: f64 = (assign1760_e2575 + assign1760_e2578);
        locals.var_nfactor_i = assign1760_e2579;
        locals.var_nfactor_i_dn0 = 0.0;
        locals.var_nfactor_i_dn2 = 0.0;
        locals.var_nfactor_i_dn3 = 0.0;
        locals.var_nfactor_i_dn4 = 0.0;
        locals.var_nfactor_i_dn5 = 0.0;
        locals.var_nfactor_i_dn6 = 0.0;
        locals.var_nfactor_i_dn7 = 0.0;
        locals.var_nfactor_i_dn8 = 0.0;
        locals.var_nfactor_i_dn9 = 0.0;
        locals.var_nfactor_i_dn10 = 0.0;
        locals.var_nfactor_i_dn11 = 0.0;
        locals.var_nfactor_i_dn12 = 0.0;
        locals.var_nfactor_i_dn13 = 0.0;
        locals.var_nfactor_i_dn14 = 0.0;

        let assign1770_e2583: f64 = (locals.var_bin_l * p.p226);
        let assign1770_e2584: f64 = (p.p223 + assign1770_e2583);
        let assign1770_e2587: f64 = (locals.var_bin_w * p.p227);
        let assign1770_e2588: f64 = (assign1770_e2584 + assign1770_e2587);
        let assign1770_e2591: f64 = (locals.var_bin_wl * p.p228);
        let assign1770_e2592: f64 = (assign1770_e2588 + assign1770_e2591);
        locals.var_cdscd_i = assign1770_e2592;
        locals.var_cdscd_i_dn0 = 0.0;
        locals.var_cdscd_i_dn2 = 0.0;
        locals.var_cdscd_i_dn3 = 0.0;
        locals.var_cdscd_i_dn4 = 0.0;
        locals.var_cdscd_i_dn5 = 0.0;
        locals.var_cdscd_i_dn6 = 0.0;
        locals.var_cdscd_i_dn7 = 0.0;
        locals.var_cdscd_i_dn8 = 0.0;
        locals.var_cdscd_i_dn9 = 0.0;
        locals.var_cdscd_i_dn10 = 0.0;
        locals.var_cdscd_i_dn11 = 0.0;
        locals.var_cdscd_i_dn12 = 0.0;
        locals.var_cdscd_i_dn13 = 0.0;
        locals.var_cdscd_i_dn14 = 0.0;

        let assign1780_e2596: f64 = (locals.var_bin_l * p.p236);
        let assign1780_e2597: f64 = (p.p233 + assign1780_e2596);
        let assign1780_e2600: f64 = (locals.var_bin_w * p.p237);
        let assign1780_e2601: f64 = (assign1780_e2597 + assign1780_e2600);
        let assign1780_e2604: f64 = (locals.var_bin_wl * p.p238);
        let assign1780_e2605: f64 = (assign1780_e2601 + assign1780_e2604);
        locals.var_cdscb_i = assign1780_e2605;

        let assign1790_e2609: f64 = (locals.var_bin_l * p.p144);
        let assign1790_e2610: f64 = (p.p143 + assign1790_e2609);
        let assign1790_e2613: f64 = (locals.var_bin_w * p.p145);
        let assign1790_e2614: f64 = (assign1790_e2610 + assign1790_e2613);
        let assign1790_e2617: f64 = (locals.var_bin_wl * p.p146);
        let assign1790_e2618: f64 = (assign1790_e2614 + assign1790_e2617);
        locals.var_dvtp0_i = assign1790_e2618;

        let assign1800_e2622: f64 = (locals.var_bin_l * p.p148);
        let assign1800_e2623: f64 = (p.p147 + assign1800_e2622);
        let assign1800_e2626: f64 = (locals.var_bin_w * p.p149);
        let assign1800_e2627: f64 = (assign1800_e2623 + assign1800_e2626);
        let assign1800_e2630: f64 = (locals.var_bin_wl * p.p150);
        let assign1800_e2631: f64 = (assign1800_e2627 + assign1800_e2630);
        locals.var_dvtp1_i = assign1800_e2631;

        let assign1810_e2635: f64 = (locals.var_bin_l * p.p152);
        let assign1810_e2636: f64 = (p.p151 + assign1810_e2635);
        let assign1810_e2639: f64 = (locals.var_bin_w * p.p153);
        let assign1810_e2640: f64 = (assign1810_e2636 + assign1810_e2639);
        let assign1810_e2643: f64 = (locals.var_bin_wl * p.p154);
        let assign1810_e2644: f64 = (assign1810_e2640 + assign1810_e2643);
        locals.var_dvtp2_i = assign1810_e2644;

        let assign1820_e2648: f64 = (locals.var_bin_l * p.p156);
        let assign1820_e2649: f64 = (p.p155 + assign1820_e2648);
        let assign1820_e2652: f64 = (locals.var_bin_w * p.p157);
        let assign1820_e2653: f64 = (assign1820_e2649 + assign1820_e2652);
        let assign1820_e2656: f64 = (locals.var_bin_wl * p.p158);
        let assign1820_e2657: f64 = (assign1820_e2653 + assign1820_e2656);
        locals.var_dvtp3_i = assign1820_e2657;

        let assign1830_e2661: f64 = (locals.var_bin_l * p.p160);
        let assign1830_e2662: f64 = (p.p159 + assign1830_e2661);
        let assign1830_e2665: f64 = (locals.var_bin_w * p.p161);
        let assign1830_e2666: f64 = (assign1830_e2662 + assign1830_e2665);
        let assign1830_e2669: f64 = (locals.var_bin_wl * p.p162);
        let assign1830_e2670: f64 = (assign1830_e2666 + assign1830_e2669);
        locals.var_dvtp4_i = assign1830_e2670;

        let assign1840_e2674: f64 = (locals.var_bin_l * p.p164);
        let assign1840_e2675: f64 = (p.p163 + assign1840_e2674);
        let assign1840_e2678: f64 = (locals.var_bin_w * p.p165);
        let assign1840_e2679: f64 = (assign1840_e2675 + assign1840_e2678);
        let assign1840_e2682: f64 = (locals.var_bin_wl * p.p166);
        let assign1840_e2683: f64 = (assign1840_e2679 + assign1840_e2682);
        locals.var_dvtp5_i = assign1840_e2683;

        let assign1850_e2687: f64 = (locals.var_bin_l * p.p202);
        let assign1850_e2688: f64 = (p.p195 + assign1850_e2687);
        let assign1850_e2691: f64 = (locals.var_bin_w * p.p203);
        let assign1850_e2692: f64 = (assign1850_e2688 + assign1850_e2691);
        let assign1850_e2695: f64 = (locals.var_bin_wl * p.p204);
        let assign1850_e2696: f64 = (assign1850_e2692 + assign1850_e2695);
        locals.var_k2_i = assign1850_e2696;
        locals.var_k2_i_dn0 = 0.0;
        locals.var_k2_i_dn2 = 0.0;
        locals.var_k2_i_dn3 = 0.0;
        locals.var_k2_i_dn4 = 0.0;
        locals.var_k2_i_dn5 = 0.0;
        locals.var_k2_i_dn6 = 0.0;
        locals.var_k2_i_dn7 = 0.0;
        locals.var_k2_i_dn8 = 0.0;
        locals.var_k2_i_dn9 = 0.0;
        locals.var_k2_i_dn10 = 0.0;
        locals.var_k2_i_dn11 = 0.0;
        locals.var_k2_i_dn12 = 0.0;
        locals.var_k2_i_dn13 = 0.0;
        locals.var_k2_i_dn14 = 0.0;

        let assign1860_e2700: f64 = (locals.var_bin_l * p.p192);
        let assign1860_e2701: f64 = (p.p185 + assign1860_e2700);
        let assign1860_e2704: f64 = (locals.var_bin_w * p.p193);
        let assign1860_e2705: f64 = (assign1860_e2701 + assign1860_e2704);
        let assign1860_e2708: f64 = (locals.var_bin_wl * p.p194);
        let assign1860_e2709: f64 = (assign1860_e2705 + assign1860_e2708);
        locals.var_k1_i = assign1860_e2709;
        locals.var_k1_i_dn0 = 0.0;
        locals.var_k1_i_dn2 = 0.0;
        locals.var_k1_i_dn3 = 0.0;
        locals.var_k1_i_dn4 = 0.0;
        locals.var_k1_i_dn5 = 0.0;
        locals.var_k1_i_dn6 = 0.0;
        locals.var_k1_i_dn7 = 0.0;
        locals.var_k1_i_dn8 = 0.0;
        locals.var_k1_i_dn9 = 0.0;
        locals.var_k1_i_dn10 = 0.0;
        locals.var_k1_i_dn11 = 0.0;
        locals.var_k1_i_dn12 = 0.0;
        locals.var_k1_i_dn13 = 0.0;
        locals.var_k1_i_dn14 = 0.0;

        let assign1870_e2713: f64 = (locals.var_bin_l * p.p113);
        let assign1870_e2714: f64 = (p.p112 + assign1870_e2713);
        let assign1870_e2717: f64 = (locals.var_bin_w * p.p114);
        let assign1870_e2718: f64 = (assign1870_e2714 + assign1870_e2717);
        let assign1870_e2721: f64 = (locals.var_bin_wl * p.p115);
        let assign1870_e2722: f64 = (assign1870_e2718 + assign1870_e2721);
        locals.var_xj_i = assign1870_e2722;

        let assign1880_e2726: f64 = (locals.var_bin_l * p.p168);
        let assign1880_e2727: f64 = (p.p167 + assign1880_e2726);
        let assign1880_e2730: f64 = (locals.var_bin_w * p.p169);
        let assign1880_e2731: f64 = (assign1880_e2727 + assign1880_e2730);
        let assign1880_e2734: f64 = (locals.var_bin_wl * p.p170);
        let assign1880_e2735: f64 = (assign1880_e2731 + assign1880_e2734);
        locals.var_phin_i = assign1880_e2735;

        let assign1890_e2739: f64 = (locals.var_bin_l * p.p172);
        let assign1890_e2740: f64 = (p.p171 + assign1890_e2739);
        let assign1890_e2743: f64 = (locals.var_bin_w * p.p173);
        let assign1890_e2744: f64 = (assign1890_e2740 + assign1890_e2743);
        let assign1890_e2747: f64 = (locals.var_bin_wl * p.p174);
        let assign1890_e2748: f64 = (assign1890_e2744 + assign1890_e2747);
        locals.var_eta0_i = assign1890_e2748;
        locals.var_eta0_i_dn0 = 0.0;
        locals.var_eta0_i_dn2 = 0.0;
        locals.var_eta0_i_dn3 = 0.0;
        locals.var_eta0_i_dn4 = 0.0;
        locals.var_eta0_i_dn5 = 0.0;
        locals.var_eta0_i_dn6 = 0.0;
        locals.var_eta0_i_dn7 = 0.0;
        locals.var_eta0_i_dn8 = 0.0;
        locals.var_eta0_i_dn9 = 0.0;
        locals.var_eta0_i_dn10 = 0.0;
        locals.var_eta0_i_dn11 = 0.0;
        locals.var_eta0_i_dn12 = 0.0;
        locals.var_eta0_i_dn13 = 0.0;
        locals.var_eta0_i_dn14 = 0.0;

        let assign1900_e2752: f64 = (locals.var_bin_l * p.p182);
        let assign1900_e2753: f64 = (p.p180 + assign1900_e2752);
        let assign1900_e2756: f64 = (locals.var_bin_w * p.p183);
        let assign1900_e2757: f64 = (assign1900_e2753 + assign1900_e2756);
        let assign1900_e2760: f64 = (locals.var_bin_wl * p.p184);
        let assign1900_e2761: f64 = (assign1900_e2757 + assign1900_e2760);
        locals.var_etab_i = assign1900_e2761;

        let assign1910_e2765: f64 = (locals.var_bin_l * p.p254);
        let assign1910_e2766: f64 = (p.p253 + assign1910_e2765);
        let assign1910_e2769: f64 = (locals.var_bin_w * p.p255);
        let assign1910_e2770: f64 = (assign1910_e2766 + assign1910_e2769);
        let assign1910_e2773: f64 = (locals.var_bin_wl * p.p256);
        let assign1910_e2774: f64 = (assign1910_e2770 + assign1910_e2773);
        locals.var_delta_i = assign1910_e2774;
        locals.var_delta_i_dn0 = 0.0;
        locals.var_delta_i_dn2 = 0.0;
        locals.var_delta_i_dn3 = 0.0;
        locals.var_delta_i_dn4 = 0.0;
        locals.var_delta_i_dn5 = 0.0;
        locals.var_delta_i_dn6 = 0.0;
        locals.var_delta_i_dn7 = 0.0;
        locals.var_delta_i_dn8 = 0.0;
        locals.var_delta_i_dn9 = 0.0;
        locals.var_delta_i_dn10 = 0.0;
        locals.var_delta_i_dn11 = 0.0;
        locals.var_delta_i_dn12 = 0.0;
        locals.var_delta_i_dn13 = 0.0;
        locals.var_delta_i_dn14 = 0.0;

        let assign1920_e2778: f64 = (locals.var_bin_l * p.p276);
        let assign1920_e2779: f64 = (p.p273 + assign1920_e2778);
        let assign1920_e2782: f64 = (locals.var_bin_w * p.p277);
        let assign1920_e2783: f64 = (assign1920_e2779 + assign1920_e2782);
        let assign1920_e2786: f64 = (locals.var_bin_wl * p.p278);
        let assign1920_e2787: f64 = (assign1920_e2783 + assign1920_e2786);
        locals.var_u0_i = assign1920_e2787;

    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign1930_e2791: f64 = (locals.var_bin_l * p.p291);
        let assign1930_e2792: f64 = (p.p284 + assign1930_e2791);
        let assign1930_e2795: f64 = (locals.var_bin_w * p.p292);
        let assign1930_e2796: f64 = (assign1930_e2792 + assign1930_e2795);
        let assign1930_e2799: f64 = (locals.var_bin_wl * p.p293);
        let assign1930_e2800: f64 = (assign1930_e2796 + assign1930_e2799);
        locals.var_ua_i = assign1930_e2800;
        locals.var_ua_i_dn0 = 0.0;
        locals.var_ua_i_dn2 = 0.0;
        locals.var_ua_i_dn3 = 0.0;
        locals.var_ua_i_dn4 = 0.0;
        locals.var_ua_i_dn5 = 0.0;
        locals.var_ua_i_dn6 = 0.0;
        locals.var_ua_i_dn7 = 0.0;
        locals.var_ua_i_dn8 = 0.0;
        locals.var_ua_i_dn9 = 0.0;
        locals.var_ua_i_dn10 = 0.0;
        locals.var_ua_i_dn11 = 0.0;
        locals.var_ua_i_dn12 = 0.0;
        locals.var_ua_i_dn13 = 0.0;
        locals.var_ua_i_dn14 = 0.0;

        let assign1940_e2804: f64 = (locals.var_bin_l * p.p311);
        let assign1940_e2805: f64 = (p.p308 + assign1940_e2804);
        let assign1940_e2808: f64 = (locals.var_bin_w * p.p312);
        let assign1940_e2809: f64 = (assign1940_e2805 + assign1940_e2808);
        let assign1940_e2812: f64 = (locals.var_bin_wl * p.p313);
        let assign1940_e2813: f64 = (assign1940_e2809 + assign1940_e2812);
        locals.var_ud_i = assign1940_e2813;
        locals.var_ud_i_dn0 = 0.0;
        locals.var_ud_i_dn2 = 0.0;
        locals.var_ud_i_dn3 = 0.0;
        locals.var_ud_i_dn4 = 0.0;
        locals.var_ud_i_dn5 = 0.0;
        locals.var_ud_i_dn6 = 0.0;
        locals.var_ud_i_dn7 = 0.0;
        locals.var_ud_i_dn8 = 0.0;
        locals.var_ud_i_dn9 = 0.0;
        locals.var_ud_i_dn10 = 0.0;
        locals.var_ud_i_dn11 = 0.0;
        locals.var_ud_i_dn12 = 0.0;
        locals.var_ud_i_dn13 = 0.0;
        locals.var_ud_i_dn14 = 0.0;

        let assign1950_e2817: f64 = (locals.var_bin_l * p.p299);
        let assign1950_e2818: f64 = (p.p298 + assign1950_e2817);
        let assign1950_e2821: f64 = (locals.var_bin_w * p.p300);
        let assign1950_e2822: f64 = (assign1950_e2818 + assign1950_e2821);
        let assign1950_e2825: f64 = (locals.var_bin_wl * p.p301);
        let assign1950_e2826: f64 = (assign1950_e2822 + assign1950_e2825);
        locals.var_eu_i = assign1950_e2826;
        locals.var_eu_i_dn0 = 0.0;
        locals.var_eu_i_dn2 = 0.0;
        locals.var_eu_i_dn3 = 0.0;
        locals.var_eu_i_dn4 = 0.0;
        locals.var_eu_i_dn5 = 0.0;
        locals.var_eu_i_dn6 = 0.0;
        locals.var_eu_i_dn7 = 0.0;
        locals.var_eu_i_dn8 = 0.0;
        locals.var_eu_i_dn9 = 0.0;
        locals.var_eu_i_dn10 = 0.0;
        locals.var_eu_i_dn11 = 0.0;
        locals.var_eu_i_dn12 = 0.0;
        locals.var_eu_i_dn13 = 0.0;
        locals.var_eu_i_dn14 = 0.0;

        let assign1960_e2830: f64 = (locals.var_bin_l * p.p319);
        let assign1960_e2831: f64 = (p.p318 + assign1960_e2830);
        let assign1960_e2834: f64 = (locals.var_bin_w * p.p320);
        let assign1960_e2835: f64 = (assign1960_e2831 + assign1960_e2834);
        let assign1960_e2838: f64 = (locals.var_bin_wl * p.p321);
        let assign1960_e2839: f64 = (assign1960_e2835 + assign1960_e2838);
        locals.var_ucs_i = assign1960_e2839;

        let assign1970_e2843: f64 = (locals.var_bin_l * p.p333);
        let assign1970_e2844: f64 = (p.p326 + assign1970_e2843);
        let assign1970_e2847: f64 = (locals.var_bin_w * p.p334);
        let assign1970_e2848: f64 = (assign1970_e2844 + assign1970_e2847);
        let assign1970_e2851: f64 = (locals.var_bin_wl * p.p335);
        let assign1970_e2852: f64 = (assign1970_e2848 + assign1970_e2851);
        locals.var_uc_i = assign1970_e2852;
        locals.var_uc_i_dn0 = 0.0;
        locals.var_uc_i_dn2 = 0.0;
        locals.var_uc_i_dn3 = 0.0;
        locals.var_uc_i_dn4 = 0.0;
        locals.var_uc_i_dn5 = 0.0;
        locals.var_uc_i_dn6 = 0.0;
        locals.var_uc_i_dn7 = 0.0;
        locals.var_uc_i_dn8 = 0.0;
        locals.var_uc_i_dn9 = 0.0;
        locals.var_uc_i_dn10 = 0.0;
        locals.var_uc_i_dn11 = 0.0;
        locals.var_uc_i_dn12 = 0.0;
        locals.var_uc_i_dn13 = 0.0;
        locals.var_uc_i_dn14 = 0.0;

        let assign1980_e2856: f64 = (locals.var_bin_l * p.p343);
        let assign1980_e2857: f64 = (p.p340 + assign1980_e2856);
        let assign1980_e2860: f64 = (locals.var_bin_w * p.p344);
        let assign1980_e2861: f64 = (assign1980_e2857 + assign1980_e2860);
        let assign1980_e2864: f64 = (locals.var_bin_wl * p.p345);
        let assign1980_e2865: f64 = (assign1980_e2861 + assign1980_e2864);
        locals.var_pclm_i = assign1980_e2865;
        locals.var_pclm_i_dn0 = 0.0;
        locals.var_pclm_i_dn2 = 0.0;
        locals.var_pclm_i_dn3 = 0.0;
        locals.var_pclm_i_dn4 = 0.0;
        locals.var_pclm_i_dn5 = 0.0;
        locals.var_pclm_i_dn6 = 0.0;
        locals.var_pclm_i_dn7 = 0.0;
        locals.var_pclm_i_dn8 = 0.0;
        locals.var_pclm_i_dn9 = 0.0;
        locals.var_pclm_i_dn10 = 0.0;
        locals.var_pclm_i_dn11 = 0.0;
        locals.var_pclm_i_dn12 = 0.0;
        locals.var_pclm_i_dn13 = 0.0;
        locals.var_pclm_i_dn14 = 0.0;

        let assign1990_e2869: f64 = (locals.var_bin_l * p.p354);
        let assign1990_e2870: f64 = (p.p351 + assign1990_e2869);
        let assign1990_e2873: f64 = (locals.var_bin_w * p.p355);
        let assign1990_e2874: f64 = (assign1990_e2870 + assign1990_e2873);
        let assign1990_e2877: f64 = (locals.var_bin_wl * p.p356);
        let assign1990_e2878: f64 = (assign1990_e2874 + assign1990_e2877);
        locals.var_pclmcv_i = assign1990_e2878;

        let assign2000_e2882: f64 = (locals.var_bin_l * p.p394);
        let assign2000_e2883: f64 = (p.p393 + assign2000_e2882);
        let assign2000_e2886: f64 = (locals.var_bin_w * p.p395);
        let assign2000_e2887: f64 = (assign2000_e2883 + assign2000_e2886);
        let assign2000_e2890: f64 = (locals.var_bin_wl * p.p396);
        let assign2000_e2891: f64 = (assign2000_e2887 + assign2000_e2890);
        locals.var_rsw_i = assign2000_e2891;

        let assign2010_e2895: f64 = (locals.var_bin_l * p.p404);
        let assign2010_e2896: f64 = (p.p403 + assign2010_e2895);
        let assign2010_e2899: f64 = (locals.var_bin_w * p.p405);
        let assign2010_e2900: f64 = (assign2010_e2896 + assign2010_e2899);
        let assign2010_e2903: f64 = (locals.var_bin_wl * p.p406);
        let assign2010_e2904: f64 = (assign2010_e2900 + assign2010_e2903);
        locals.var_rdw_i = assign2010_e2904;

        let assign2020_e2908: f64 = (locals.var_bin_l * p.p376);
        let assign2020_e2909: f64 = (p.p375 + assign2020_e2908);
        let assign2020_e2912: f64 = (locals.var_bin_w * p.p377);
        let assign2020_e2913: f64 = (assign2020_e2909 + assign2020_e2912);
        let assign2020_e2916: f64 = (locals.var_bin_wl * p.p378);
        let assign2020_e2917: f64 = (assign2020_e2913 + assign2020_e2916);
        locals.var_prwg_i = assign2020_e2917;

        let assign2030_e2921: f64 = (locals.var_bin_l * p.p380);
        let assign2030_e2922: f64 = (p.p379 + assign2030_e2921);
        let assign2030_e2925: f64 = (locals.var_bin_w * p.p381);
        let assign2030_e2926: f64 = (assign2030_e2922 + assign2030_e2925);
        let assign2030_e2929: f64 = (locals.var_bin_wl * p.p382);
        let assign2030_e2930: f64 = (assign2030_e2926 + assign2030_e2929);
        locals.var_prwb_i = assign2030_e2930;

        let assign2040_e2934: f64 = (locals.var_bin_l * p.p386);
        let assign2040_e2935: f64 = (p.p385 + assign2040_e2934);
        let assign2040_e2938: f64 = (locals.var_bin_w * p.p387);
        let assign2040_e2939: f64 = (assign2040_e2935 + assign2040_e2938);
        let assign2040_e2942: f64 = (locals.var_bin_wl * p.p388);
        let assign2040_e2943: f64 = (assign2040_e2939 + assign2040_e2942);
        locals.var_wr_i = assign2040_e2943;

        let assign2050_e2947: f64 = (locals.var_bin_l * p.p390);
        let assign2050_e2948: f64 = (p.p389 + assign2050_e2947);
        let assign2050_e2951: f64 = (locals.var_bin_w * p.p391);
        let assign2050_e2952: f64 = (assign2050_e2948 + assign2050_e2951);
        let assign2050_e2955: f64 = (locals.var_bin_wl * p.p392);
        let assign2050_e2956: f64 = (assign2050_e2952 + assign2050_e2955);
        locals.var_rswmin_i = assign2050_e2956;

        let assign2060_e2960: f64 = (locals.var_bin_l * p.p400);
        let assign2060_e2961: f64 = (p.p399 + assign2060_e2960);
        let assign2060_e2964: f64 = (locals.var_bin_w * p.p401);
        let assign2060_e2965: f64 = (assign2060_e2961 + assign2060_e2964);
        let assign2060_e2968: f64 = (locals.var_bin_wl * p.p402);
        let assign2060_e2969: f64 = (assign2060_e2965 + assign2060_e2968);
        locals.var_rdwmin_i = assign2060_e2969;

        let assign2070_e2973: f64 = (locals.var_bin_l * p.p416);
        let assign2070_e2974: f64 = (p.p413 + assign2070_e2973);
        let assign2070_e2977: f64 = (locals.var_bin_w * p.p417);
        let assign2070_e2978: f64 = (assign2070_e2974 + assign2070_e2977);
        let assign2070_e2981: f64 = (locals.var_bin_wl * p.p418);
        let assign2070_e2982: f64 = (assign2070_e2978 + assign2070_e2981);
        locals.var_rdsw_i = assign2070_e2982;

        let assign2080_e2986: f64 = (locals.var_bin_l * p.p410);
        let assign2080_e2987: f64 = (p.p409 + assign2080_e2986);
        let assign2080_e2990: f64 = (locals.var_bin_w * p.p411);
        let assign2080_e2991: f64 = (assign2080_e2987 + assign2080_e2990);
        let assign2080_e2994: f64 = (locals.var_bin_wl * p.p412);
        let assign2080_e2995: f64 = (assign2080_e2991 + assign2080_e2994);
        locals.var_rdswmin_i = assign2080_e2995;

        let assign2090_e2999: f64 = (locals.var_bin_l * p.p435);
        let assign2090_e3000: f64 = (p.p434 + assign2090_e2999);
        let assign2090_e3003: f64 = (locals.var_bin_w * p.p436);
        let assign2090_e3004: f64 = (assign2090_e3000 + assign2090_e3003);
        let assign2090_e3007: f64 = (locals.var_bin_wl * p.p437);
        let assign2090_e3008: f64 = (assign2090_e3004 + assign2090_e3007);
        locals.var_ptwg_i = assign2090_e3008;
        locals.var_ptwg_i_dn0 = 0.0;
        locals.var_ptwg_i_dn2 = 0.0;
        locals.var_ptwg_i_dn3 = 0.0;
        locals.var_ptwg_i_dn4 = 0.0;
        locals.var_ptwg_i_dn5 = 0.0;
        locals.var_ptwg_i_dn6 = 0.0;
        locals.var_ptwg_i_dn7 = 0.0;
        locals.var_ptwg_i_dn8 = 0.0;
        locals.var_ptwg_i_dn9 = 0.0;
        locals.var_ptwg_i_dn10 = 0.0;
        locals.var_ptwg_i_dn11 = 0.0;
        locals.var_ptwg_i_dn12 = 0.0;
        locals.var_ptwg_i_dn13 = 0.0;
        locals.var_ptwg_i_dn14 = 0.0;

        let assign2100_e3012: f64 = (locals.var_bin_l * p.p463);
        let assign2100_e3013: f64 = (p.p460 + assign2100_e3012);
        let assign2100_e3016: f64 = (locals.var_bin_w * p.p464);
        let assign2100_e3017: f64 = (assign2100_e3013 + assign2100_e3016);
        let assign2100_e3020: f64 = (locals.var_bin_wl * p.p465);
        let assign2100_e3021: f64 = (assign2100_e3017 + assign2100_e3020);
        locals.var_pdiblc_i = assign2100_e3021;
        locals.var_pdiblc_i_dn0 = 0.0;
        locals.var_pdiblc_i_dn2 = 0.0;
        locals.var_pdiblc_i_dn3 = 0.0;
        locals.var_pdiblc_i_dn4 = 0.0;
        locals.var_pdiblc_i_dn5 = 0.0;
        locals.var_pdiblc_i_dn6 = 0.0;
        locals.var_pdiblc_i_dn7 = 0.0;
        locals.var_pdiblc_i_dn8 = 0.0;
        locals.var_pdiblc_i_dn9 = 0.0;
        locals.var_pdiblc_i_dn10 = 0.0;
        locals.var_pdiblc_i_dn11 = 0.0;
        locals.var_pdiblc_i_dn12 = 0.0;
        locals.var_pdiblc_i_dn13 = 0.0;
        locals.var_pdiblc_i_dn14 = 0.0;

        let assign2110_e3025: f64 = (locals.var_bin_l * p.p471);
        let assign2110_e3026: f64 = (p.p470 + assign2110_e3025);
        let assign2110_e3029: f64 = (locals.var_bin_w * p.p472);
        let assign2110_e3030: f64 = (assign2110_e3026 + assign2110_e3029);
        let assign2110_e3033: f64 = (locals.var_bin_wl * p.p473);
        let assign2110_e3034: f64 = (assign2110_e3030 + assign2110_e3033);
        locals.var_pdiblcb_i = assign2110_e3034;

        let assign2120_e3038: f64 = (locals.var_bin_l * p.p358);
        let assign2120_e3039: f64 = (p.p357 + assign2120_e3038);
        let assign2120_e3042: f64 = (locals.var_bin_w * p.p359);
        let assign2120_e3043: f64 = (assign2120_e3039 + assign2120_e3042);
        let assign2120_e3046: f64 = (locals.var_bin_wl * p.p360);
        let assign2120_e3047: f64 = (assign2120_e3043 + assign2120_e3046);
        locals.var_pscbe1_i = assign2120_e3047;

        let assign2130_e3051: f64 = (locals.var_bin_l * p.p362);
        let assign2130_e3052: f64 = (p.p361 + assign2130_e3051);
        let assign2130_e3055: f64 = (locals.var_bin_w * p.p363);
        let assign2130_e3056: f64 = (assign2130_e3052 + assign2130_e3055);
        let assign2130_e3059: f64 = (locals.var_bin_wl * p.p364);
        let assign2130_e3060: f64 = (assign2130_e3056 + assign2130_e3059);
        locals.var_pscbe2_i = assign2130_e3060;

        let assign2140_e3064: f64 = (locals.var_bin_l * p.p366);
        let assign2140_e3065: f64 = (p.p365 + assign2140_e3064);
        let assign2140_e3068: f64 = (locals.var_bin_w * p.p367);
        let assign2140_e3069: f64 = (assign2140_e3065 + assign2140_e3068);
        let assign2140_e3072: f64 = (locals.var_bin_wl * p.p368);
        let assign2140_e3073: f64 = (assign2140_e3069 + assign2140_e3072);
        locals.var_pdits_i = assign2140_e3073;

        let assign2150_e3077: f64 = (locals.var_bin_l * p.p371);
        let assign2150_e3078: f64 = (p.p370 + assign2150_e3077);
        let assign2150_e3081: f64 = (locals.var_bin_w * p.p372);
        let assign2150_e3082: f64 = (assign2150_e3078 + assign2150_e3081);
        let assign2150_e3085: f64 = (locals.var_bin_wl * p.p373);
        let assign2150_e3086: f64 = (assign2150_e3082 + assign2150_e3085);
        locals.var_pditsd_i = assign2150_e3086;

        let assign2160_e3090: f64 = (locals.var_bin_l * p.p481);
        let assign2160_e3091: f64 = (p.p478 + assign2160_e3090);
        let assign2160_e3094: f64 = (locals.var_bin_w * p.p482);
        let assign2160_e3095: f64 = (assign2160_e3091 + assign2160_e3094);
        let assign2160_e3098: f64 = (locals.var_bin_wl * p.p483);
        let assign2160_e3099: f64 = (assign2160_e3095 + assign2160_e3098);
        locals.var_fprout_i = assign2160_e3099;

        let assign2170_e3103: f64 = (locals.var_bin_l * p.p475);
        let assign2170_e3104: f64 = (p.p474 + assign2170_e3103);
        let assign2170_e3107: f64 = (locals.var_bin_w * p.p476);
        let assign2170_e3108: f64 = (assign2170_e3104 + assign2170_e3107);
        let assign2170_e3111: f64 = (locals.var_bin_wl * p.p477);
        let assign2170_e3112: f64 = (assign2170_e3108 + assign2170_e3111);
        locals.var_pvag_i = assign2170_e3112;

        let assign2180_e3116: f64 = (locals.var_bin_l * p.p240);
        let assign2180_e3117: f64 = (p.p239 + assign2180_e3116);
        let assign2180_e3120: f64 = (locals.var_bin_w * p.p241);
        let assign2180_e3121: f64 = (assign2180_e3117 + assign2180_e3120);
        let assign2180_e3124: f64 = (locals.var_bin_wl * p.p242);
        let assign2180_e3125: f64 = (assign2180_e3121 + assign2180_e3124);
        locals.var_vsat_i = assign2180_e3125;
        locals.var_vsat_i_dn0 = 0.0;
        locals.var_vsat_i_dn2 = 0.0;
        locals.var_vsat_i_dn3 = 0.0;
        locals.var_vsat_i_dn4 = 0.0;
        locals.var_vsat_i_dn5 = 0.0;
        locals.var_vsat_i_dn6 = 0.0;
        locals.var_vsat_i_dn7 = 0.0;
        locals.var_vsat_i_dn8 = 0.0;
        locals.var_vsat_i_dn9 = 0.0;
        locals.var_vsat_i_dn10 = 0.0;
        locals.var_vsat_i_dn11 = 0.0;
        locals.var_vsat_i_dn12 = 0.0;
        locals.var_vsat_i_dn13 = 0.0;
        locals.var_vsat_i_dn14 = 0.0;

        let assign2190_e3129: f64 = (locals.var_bin_l * p.p420);
        let assign2190_e3130: f64 = (p.p419 + assign2190_e3129);
        let assign2190_e3133: f64 = (locals.var_bin_w * p.p421);
        let assign2190_e3134: f64 = (assign2190_e3130 + assign2190_e3133);
        let assign2190_e3137: f64 = (locals.var_bin_wl * p.p422);
        let assign2190_e3138: f64 = (assign2190_e3134 + assign2190_e3137);
        locals.var_psat_i = assign2190_e3138;

        let assign2200_e3142: f64 = (locals.var_bin_l * p.p260);
        let assign2200_e3143: f64 = (p.p259 + assign2200_e3142);
        let assign2200_e3146: f64 = (locals.var_bin_w * p.p261);
        let assign2200_e3147: f64 = (assign2200_e3143 + assign2200_e3146);
        let assign2200_e3150: f64 = (locals.var_bin_wl * p.p262);
        let assign2200_e3151: f64 = (assign2200_e3147 + assign2200_e3150);
        locals.var_vsatcv_i = assign2200_e3151;
        locals.var_vsatcv_i_dn0 = 0.0;
        locals.var_vsatcv_i_dn2 = 0.0;
        locals.var_vsatcv_i_dn3 = 0.0;
        locals.var_vsatcv_i_dn4 = 0.0;
        locals.var_vsatcv_i_dn5 = 0.0;
        locals.var_vsatcv_i_dn6 = 0.0;
        locals.var_vsatcv_i_dn7 = 0.0;
        locals.var_vsatcv_i_dn8 = 0.0;
        locals.var_vsatcv_i_dn9 = 0.0;
        locals.var_vsatcv_i_dn10 = 0.0;
        locals.var_vsatcv_i_dn11 = 0.0;
        locals.var_vsatcv_i_dn12 = 0.0;
        locals.var_vsatcv_i_dn13 = 0.0;
        locals.var_vsatcv_i_dn14 = 0.0;

        let assign2210_e3155: f64 = (locals.var_bin_l * p.p667);
        let assign2210_e3156: f64 = (p.p666 + assign2210_e3155);
        let assign2210_e3159: f64 = (locals.var_bin_w * p.p668);
        let assign2210_e3160: f64 = (assign2210_e3156 + assign2210_e3159);
        let assign2210_e3163: f64 = (locals.var_bin_wl * p.p669);
        let assign2210_e3164: f64 = (assign2210_e3160 + assign2210_e3163);
        locals.var_cf_i = assign2210_e3164;

        let assign2220_e3168: f64 = (locals.var_bin_l * p.p675);
        let assign2220_e3169: f64 = (p.p674 + assign2220_e3168);
        let assign2220_e3172: f64 = (locals.var_bin_w * p.p676);
        let assign2220_e3173: f64 = (assign2220_e3169 + assign2220_e3172);
        let assign2220_e3176: f64 = (locals.var_bin_wl * p.p677);
        let assign2220_e3177: f64 = (assign2220_e3173 + assign2220_e3176);
        locals.var_cgsl_i = assign2220_e3177;

        let assign2230_e3181: f64 = (locals.var_bin_l * p.p679);
        let assign2230_e3182: f64 = (p.p678 + assign2230_e3181);
        let assign2230_e3185: f64 = (locals.var_bin_w * p.p680);
        let assign2230_e3186: f64 = (assign2230_e3182 + assign2230_e3185);
        let assign2230_e3189: f64 = (locals.var_bin_wl * p.p681);
        let assign2230_e3190: f64 = (assign2230_e3186 + assign2230_e3189);
        locals.var_cgdl_i = assign2230_e3190;

        let assign2240_e3194: f64 = (locals.var_bin_l * p.p683);
        let assign2240_e3195: f64 = (p.p682 + assign2240_e3194);
        let assign2240_e3198: f64 = (locals.var_bin_w * p.p684);
        let assign2240_e3199: f64 = (assign2240_e3195 + assign2240_e3198);
        let assign2240_e3202: f64 = (locals.var_bin_wl * p.p685);
        let assign2240_e3203: f64 = (assign2240_e3199 + assign2240_e3202);
        locals.var_ckappas_i = assign2240_e3203;

        let assign2250_e3207: f64 = (locals.var_bin_l * p.p687);
        let assign2250_e3208: f64 = (p.p686 + assign2250_e3207);
        let assign2250_e3211: f64 = (locals.var_bin_w * p.p688);
        let assign2250_e3212: f64 = (assign2250_e3208 + assign2250_e3211);
        let assign2250_e3215: f64 = (locals.var_bin_wl * p.p689);
        let assign2250_e3216: f64 = (assign2250_e3212 + assign2250_e3215);
        locals.var_ckappad_i = assign2250_e3216;

        let assign2260_e3220: f64 = (locals.var_bin_l * p.p489);
        let assign2260_e3221: f64 = (p.p484 + assign2260_e3220);
        let assign2260_e3224: f64 = (locals.var_bin_w * p.p490);
        let assign2260_e3225: f64 = (assign2260_e3221 + assign2260_e3224);
        let assign2260_e3228: f64 = (locals.var_bin_wl * p.p491);
        let assign2260_e3229: f64 = (assign2260_e3225 + assign2260_e3228);
        locals.var_alpha0_i = assign2260_e3229;
        locals.var_alpha0_i_dn0 = 0.0;
        locals.var_alpha0_i_dn2 = 0.0;
        locals.var_alpha0_i_dn3 = 0.0;
        locals.var_alpha0_i_dn4 = 0.0;
        locals.var_alpha0_i_dn5 = 0.0;
        locals.var_alpha0_i_dn6 = 0.0;
        locals.var_alpha0_i_dn7 = 0.0;
        locals.var_alpha0_i_dn8 = 0.0;
        locals.var_alpha0_i_dn9 = 0.0;
        locals.var_alpha0_i_dn10 = 0.0;
        locals.var_alpha0_i_dn11 = 0.0;
        locals.var_alpha0_i_dn12 = 0.0;
        locals.var_alpha0_i_dn13 = 0.0;
        locals.var_alpha0_i_dn14 = 0.0;

        let assign2270_e3233: f64 = (locals.var_bin_l * p.p497);
        let assign2270_e3234: f64 = (p.p494 + assign2270_e3233);
        let assign2270_e3237: f64 = (locals.var_bin_w * p.p498);
        let assign2270_e3238: f64 = (assign2270_e3234 + assign2270_e3237);
        let assign2270_e3241: f64 = (locals.var_bin_wl * p.p499);
        let assign2270_e3242: f64 = (assign2270_e3238 + assign2270_e3241);
        locals.var_beta0_i = assign2270_e3242;
        locals.var_beta0_i_dn0 = 0.0;
        locals.var_beta0_i_dn2 = 0.0;
        locals.var_beta0_i_dn3 = 0.0;
        locals.var_beta0_i_dn4 = 0.0;
        locals.var_beta0_i_dn5 = 0.0;
        locals.var_beta0_i_dn6 = 0.0;
        locals.var_beta0_i_dn7 = 0.0;
        locals.var_beta0_i_dn8 = 0.0;
        locals.var_beta0_i_dn9 = 0.0;
        locals.var_beta0_i_dn10 = 0.0;
        locals.var_beta0_i_dn11 = 0.0;
        locals.var_beta0_i_dn12 = 0.0;
        locals.var_beta0_i_dn13 = 0.0;
        locals.var_beta0_i_dn14 = 0.0;

        let assign2280_e3246: f64 = (locals.var_bin_l * p.p936);
        let assign2280_e3247: f64 = (p.p935 + assign2280_e3246);
        let assign2280_e3250: f64 = (locals.var_bin_w * p.p937);
        let assign2280_e3251: f64 = (assign2280_e3247 + assign2280_e3250);
        let assign2280_e3254: f64 = (locals.var_bin_wl * p.p938);
        let assign2280_e3255: f64 = (assign2280_e3251 + assign2280_e3254);
        locals.var_kvth0we_i = assign2280_e3255;

        let assign2290_e3259: f64 = (locals.var_bin_l * p.p940);
        let assign2290_e3260: f64 = (p.p939 + assign2290_e3259);
        let assign2290_e3263: f64 = (locals.var_bin_w * p.p941);
        let assign2290_e3264: f64 = (assign2290_e3260 + assign2290_e3263);
        let assign2290_e3267: f64 = (locals.var_bin_wl * p.p942);
        let assign2290_e3268: f64 = (assign2290_e3264 + assign2290_e3267);
        locals.var_k2we_i = assign2290_e3268;

        let assign2300_e3272: f64 = (locals.var_bin_l * p.p944);
        let assign2300_e3273: f64 = (p.p943 + assign2300_e3272);
        let assign2300_e3276: f64 = (locals.var_bin_w * p.p945);
        let assign2300_e3277: f64 = (assign2300_e3273 + assign2300_e3276);
        let assign2300_e3280: f64 = (locals.var_bin_wl * p.p946);
        let assign2300_e3281: f64 = (assign2300_e3277 + assign2300_e3280);
        locals.var_ku0we_i = assign2300_e3281;

        let assign2310_e3285: f64 = (locals.var_bin_l * p.p633);
        let assign2310_e3286: f64 = (p.p630 + assign2310_e3285);
        let assign2310_e3289: f64 = (locals.var_bin_w * p.p634);
        let assign2310_e3290: f64 = (assign2310_e3286 + assign2310_e3289);
        let assign2310_e3293: f64 = (locals.var_bin_wl * p.p635);
        let assign2310_e3294: f64 = (assign2310_e3290 + assign2310_e3293);
        locals.var_agidl_i = assign2310_e3294;

        let assign2320_e3298: f64 = (locals.var_bin_l * p.p637);
        let assign2320_e3299: f64 = (p.p636 + assign2320_e3298);
        let assign2320_e3302: f64 = (locals.var_bin_w * p.p638);
        let assign2320_e3303: f64 = (assign2320_e3299 + assign2320_e3302);
        let assign2320_e3306: f64 = (locals.var_bin_wl * p.p639);
        let assign2320_e3307: f64 = (assign2320_e3303 + assign2320_e3306);
        locals.var_bgidl_i = assign2320_e3307;

        let assign2330_e3311: f64 = (locals.var_bin_l * p.p641);
        let assign2330_e3312: f64 = (p.p640 + assign2330_e3311);
        let assign2330_e3315: f64 = (locals.var_bin_w * p.p642);
        let assign2330_e3316: f64 = (assign2330_e3312 + assign2330_e3315);
        let assign2330_e3319: f64 = (locals.var_bin_wl * p.p643);
        let assign2330_e3320: f64 = (assign2330_e3316 + assign2330_e3319);
        locals.var_cgidl_i = assign2330_e3320;

        let assign2340_e3324: f64 = (locals.var_bin_l * p.p645);
        let assign2340_e3325: f64 = (p.p644 + assign2340_e3324);
        let assign2340_e3328: f64 = (locals.var_bin_w * p.p646);
        let assign2340_e3329: f64 = (assign2340_e3325 + assign2340_e3328);
        let assign2340_e3332: f64 = (locals.var_bin_wl * p.p647);
        let assign2340_e3333: f64 = (assign2340_e3329 + assign2340_e3332);
        locals.var_egidl_i = assign2340_e3333;

        let assign2350_e3337: f64 = (locals.var_bin_l * p.p651);
        let assign2350_e3338: f64 = (p.p648 + assign2350_e3337);
        let assign2350_e3341: f64 = (locals.var_bin_w * p.p652);
        let assign2350_e3342: f64 = (assign2350_e3338 + assign2350_e3341);
        let assign2350_e3345: f64 = (locals.var_bin_wl * p.p653);
        let assign2350_e3346: f64 = (assign2350_e3342 + assign2350_e3345);
        locals.var_agisl_i = assign2350_e3346;

        let assign2360_e3350: f64 = (locals.var_bin_l * p.p655);
        let assign2360_e3351: f64 = (p.p654 + assign2360_e3350);
        let assign2360_e3354: f64 = (locals.var_bin_w * p.p656);
        let assign2360_e3355: f64 = (assign2360_e3351 + assign2360_e3354);
        let assign2360_e3358: f64 = (locals.var_bin_wl * p.p657);
        let assign2360_e3359: f64 = (assign2360_e3355 + assign2360_e3358);
        locals.var_bgisl_i = assign2360_e3359;

    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2370_e3363: f64 = (locals.var_bin_l * p.p659);
        let assign2370_e3364: f64 = (p.p658 + assign2370_e3363);
        let assign2370_e3367: f64 = (locals.var_bin_w * p.p660);
        let assign2370_e3368: f64 = (assign2370_e3364 + assign2370_e3367);
        let assign2370_e3371: f64 = (locals.var_bin_wl * p.p661);
        let assign2370_e3372: f64 = (assign2370_e3368 + assign2370_e3371);
        locals.var_cgisl_i = assign2370_e3372;

        let assign2380_e3376: f64 = (locals.var_bin_l * p.p663);
        let assign2380_e3377: f64 = (p.p662 + assign2380_e3376);
        let assign2380_e3380: f64 = (locals.var_bin_w * p.p664);
        let assign2380_e3381: f64 = (assign2380_e3377 + assign2380_e3380);
        let assign2380_e3384: f64 = (locals.var_bin_wl * p.p665);
        let assign2380_e3385: f64 = (assign2380_e3381 + assign2380_e3384);
        locals.var_egisl_i = assign2380_e3385;

        let assign2390_e3389: f64 = (locals.var_bin_l * p.p825);
        let assign2390_e3390: f64 = (p.p824 + assign2390_e3389);
        let assign2390_e3393: f64 = (locals.var_bin_w * p.p826);
        let assign2390_e3394: f64 = (assign2390_e3390 + assign2390_e3393);
        let assign2390_e3397: f64 = (locals.var_bin_wl * p.p827);
        let assign2390_e3398: f64 = (assign2390_e3394 + assign2390_e3397);
        locals.var_ute_i = assign2390_e3398;

        let assign2400_e3402: f64 = (locals.var_bin_l * p.p830);
        let assign2400_e3403: f64 = (p.p829 + assign2400_e3402);
        let assign2400_e3406: f64 = (locals.var_bin_w * p.p831);
        let assign2400_e3407: f64 = (assign2400_e3403 + assign2400_e3406);
        let assign2400_e3410: f64 = (locals.var_bin_wl * p.p832);
        let assign2400_e3411: f64 = (assign2400_e3407 + assign2400_e3410);
        locals.var_ua1_i = assign2400_e3411;

        let assign2410_e3415: f64 = (locals.var_bin_l * p.p835);
        let assign2410_e3416: f64 = (p.p834 + assign2410_e3415);
        let assign2410_e3419: f64 = (locals.var_bin_w * p.p836);
        let assign2410_e3420: f64 = (assign2410_e3416 + assign2410_e3419);
        let assign2410_e3423: f64 = (locals.var_bin_wl * p.p837);
        let assign2410_e3424: f64 = (assign2410_e3420 + assign2410_e3423);
        locals.var_uc1_i = assign2410_e3424;

        let assign2420_e3428: f64 = (locals.var_bin_l * p.p839);
        let assign2420_e3429: f64 = (p.p838 + assign2420_e3428);
        let assign2420_e3432: f64 = (locals.var_bin_w * p.p840);
        let assign2420_e3433: f64 = (assign2420_e3429 + assign2420_e3432);
        let assign2420_e3436: f64 = (locals.var_bin_wl * p.p841);
        let assign2420_e3437: f64 = (assign2420_e3433 + assign2420_e3436);
        locals.var_ud1_i = assign2420_e3437;

        let assign2430_e3441: f64 = (locals.var_bin_l * p.p844);
        let assign2430_e3442: f64 = (p.p843 + assign2430_e3441);
        let assign2430_e3445: f64 = (locals.var_bin_w * p.p845);
        let assign2430_e3446: f64 = (assign2430_e3442 + assign2430_e3445);
        let assign2430_e3449: f64 = (locals.var_bin_wl * p.p846);
        let assign2430_e3450: f64 = (assign2430_e3446 + assign2430_e3449);
        locals.var_eu1_i = assign2430_e3450;

        let assign2440_e3454: f64 = (locals.var_bin_l * p.p848);
        let assign2440_e3455: f64 = (p.p847 + assign2440_e3454);
        let assign2440_e3458: f64 = (locals.var_bin_w * p.p849);
        let assign2440_e3459: f64 = (assign2440_e3455 + assign2440_e3458);
        let assign2440_e3462: f64 = (locals.var_bin_wl * p.p850);
        let assign2440_e3463: f64 = (assign2440_e3459 + assign2440_e3462);
        locals.var_ucste_i = assign2440_e3463;

        let assign2450_e3467: f64 = (locals.var_bin_l * p.p853);
        let assign2450_e3468: f64 = (p.p852 + assign2450_e3467);
        let assign2450_e3471: f64 = (locals.var_bin_w * p.p854);
        let assign2450_e3472: f64 = (assign2450_e3468 + assign2450_e3471);
        let assign2450_e3475: f64 = (locals.var_bin_wl * p.p855);
        let assign2450_e3476: f64 = (assign2450_e3472 + assign2450_e3475);
        locals.var_prt_i = assign2450_e3476;

        let assign2460_e3480: f64 = (locals.var_bin_l * p.p857);
        let assign2460_e3481: f64 = (p.p856 + assign2460_e3480);
        let assign2460_e3484: f64 = (locals.var_bin_w * p.p858);
        let assign2460_e3485: f64 = (assign2460_e3481 + assign2460_e3484);
        let assign2460_e3488: f64 = (locals.var_bin_wl * p.p859);
        let assign2460_e3489: f64 = (assign2460_e3485 + assign2460_e3488);
        locals.var_at_i = assign2460_e3489;

        let assign2470_e3493: f64 = (locals.var_bin_l * p.p863);
        let assign2470_e3494: f64 = (p.p862 + assign2470_e3493);
        let assign2470_e3497: f64 = (locals.var_bin_w * p.p864);
        let assign2470_e3498: f64 = (assign2470_e3494 + assign2470_e3497);
        let assign2470_e3501: f64 = (locals.var_bin_wl * p.p865);
        let assign2470_e3502: f64 = (assign2470_e3498 + assign2470_e3501);
        locals.var_ptwgt_i = assign2470_e3502;

        let assign2480_e3506: f64 = (locals.var_bin_l * p.p878);
        let assign2480_e3507: f64 = (p.p877 + assign2480_e3506);
        let assign2480_e3510: f64 = (locals.var_bin_w * p.p879);
        let assign2480_e3511: f64 = (assign2480_e3507 + assign2480_e3510);
        let assign2480_e3514: f64 = (locals.var_bin_wl * p.p880);
        let assign2480_e3515: f64 = (assign2480_e3511 + assign2480_e3514);
        locals.var_iit_i = assign2480_e3515;

        let assign2490_e3519: f64 = (locals.var_bin_l * p.p886);
        let assign2490_e3520: f64 = (p.p885 + assign2490_e3519);
        let assign2490_e3523: f64 = (locals.var_bin_w * p.p887);
        let assign2490_e3524: f64 = (assign2490_e3520 + assign2490_e3523);
        let assign2490_e3527: f64 = (locals.var_bin_wl * p.p888);
        let assign2490_e3528: f64 = (assign2490_e3524 + assign2490_e3527);
        locals.var_tgidl_i = assign2490_e3528;

        let assign2500_e3532: f64 = (locals.var_bin_l * p.p882);
        let assign2500_e3533: f64 = (p.p881 + assign2500_e3532);
        let assign2500_e3536: f64 = (locals.var_bin_w * p.p883);
        let assign2500_e3537: f64 = (assign2500_e3533 + assign2500_e3536);
        let assign2500_e3540: f64 = (locals.var_bin_wl * p.p884);
        let assign2500_e3541: f64 = (assign2500_e3537 + assign2500_e3540);
        locals.var_igt_i = assign2500_e3541;

        let assign2510_e3545: f64 = (locals.var_bin_l * p.p564);
        let assign2510_e3546: f64 = (p.p537 + assign2510_e3545);
        let assign2510_e3549: f64 = (locals.var_bin_w * p.p565);
        let assign2510_e3550: f64 = (assign2510_e3546 + assign2510_e3549);
        let assign2510_e3553: f64 = (locals.var_bin_wl * p.p566);
        let assign2510_e3554: f64 = (assign2510_e3550 + assign2510_e3553);
        locals.var_aigbinv_i = assign2510_e3554;

        let assign2520_e3558: f64 = (locals.var_bin_l * p.p567);
        let assign2520_e3559: f64 = (p.p538 + assign2520_e3558);
        let assign2520_e3562: f64 = (locals.var_bin_w * p.p568);
        let assign2520_e3563: f64 = (assign2520_e3559 + assign2520_e3562);
        let assign2520_e3566: f64 = (locals.var_bin_wl * p.p569);
        let assign2520_e3567: f64 = (assign2520_e3563 + assign2520_e3566);
        locals.var_bigbinv_i = assign2520_e3567;

        let assign2530_e3571: f64 = (locals.var_bin_l * p.p570);
        let assign2530_e3572: f64 = (p.p539 + assign2530_e3571);
        let assign2530_e3575: f64 = (locals.var_bin_w * p.p571);
        let assign2530_e3576: f64 = (assign2530_e3572 + assign2530_e3575);
        let assign2530_e3579: f64 = (locals.var_bin_wl * p.p572);
        let assign2530_e3580: f64 = (assign2530_e3576 + assign2530_e3579);
        locals.var_cigbinv_i = assign2530_e3580;

        let assign2540_e3584: f64 = (locals.var_bin_l * p.p573);
        let assign2540_e3585: f64 = (p.p540 + assign2540_e3584);
        let assign2540_e3588: f64 = (locals.var_bin_w * p.p574);
        let assign2540_e3589: f64 = (assign2540_e3585 + assign2540_e3588);
        let assign2540_e3592: f64 = (locals.var_bin_wl * p.p575);
        let assign2540_e3593: f64 = (assign2540_e3589 + assign2540_e3592);
        locals.var_eigbinv_i = assign2540_e3593;

        let assign2550_e3597: f64 = (locals.var_bin_l * p.p576);
        let assign2550_e3598: f64 = (p.p541 + assign2550_e3597);
        let assign2550_e3601: f64 = (locals.var_bin_w * p.p577);
        let assign2550_e3602: f64 = (assign2550_e3598 + assign2550_e3601);
        let assign2550_e3605: f64 = (locals.var_bin_wl * p.p578);
        let assign2550_e3606: f64 = (assign2550_e3602 + assign2550_e3605);
        locals.var_nigbinv_i = assign2550_e3606;

        let assign2560_e3610: f64 = (locals.var_bin_l * p.p579);
        let assign2560_e3611: f64 = (p.p533 + assign2560_e3610);
        let assign2560_e3614: f64 = (locals.var_bin_w * p.p580);
        let assign2560_e3615: f64 = (assign2560_e3611 + assign2560_e3614);
        let assign2560_e3618: f64 = (locals.var_bin_wl * p.p581);
        let assign2560_e3619: f64 = (assign2560_e3615 + assign2560_e3618);
        locals.var_aigbacc_i = assign2560_e3619;

        let assign2570_e3623: f64 = (locals.var_bin_l * p.p582);
        let assign2570_e3624: f64 = (p.p534 + assign2570_e3623);
        let assign2570_e3627: f64 = (locals.var_bin_w * p.p583);
        let assign2570_e3628: f64 = (assign2570_e3624 + assign2570_e3627);
        let assign2570_e3631: f64 = (locals.var_bin_wl * p.p584);
        let assign2570_e3632: f64 = (assign2570_e3628 + assign2570_e3631);
        locals.var_bigbacc_i = assign2570_e3632;

        let assign2580_e3636: f64 = (locals.var_bin_l * p.p585);
        let assign2580_e3637: f64 = (p.p535 + assign2580_e3636);
        let assign2580_e3640: f64 = (locals.var_bin_w * p.p586);
        let assign2580_e3641: f64 = (assign2580_e3637 + assign2580_e3640);
        let assign2580_e3644: f64 = (locals.var_bin_wl * p.p587);
        let assign2580_e3645: f64 = (assign2580_e3641 + assign2580_e3644);
        locals.var_cigbacc_i = assign2580_e3645;

        let assign2590_e3649: f64 = (locals.var_bin_l * p.p588);
        let assign2590_e3650: f64 = (p.p536 + assign2590_e3649);
        let assign2590_e3653: f64 = (locals.var_bin_w * p.p589);
        let assign2590_e3654: f64 = (assign2590_e3650 + assign2590_e3653);
        let assign2590_e3657: f64 = (locals.var_bin_wl * p.p590);
        let assign2590_e3658: f64 = (assign2590_e3654 + assign2590_e3657);
        locals.var_nigbacc_i = assign2590_e3658;

        let assign2600_e3662: f64 = (locals.var_bin_l * p.p591);
        let assign2600_e3663: f64 = (p.p542 + assign2600_e3662);
        let assign2600_e3666: f64 = (locals.var_bin_w * p.p592);
        let assign2600_e3667: f64 = (assign2600_e3663 + assign2600_e3666);
        let assign2600_e3670: f64 = (locals.var_bin_wl * p.p593);
        let assign2600_e3671: f64 = (assign2600_e3667 + assign2600_e3670);
        locals.var_aigc_i = assign2600_e3671;

        let assign2610_e3675: f64 = (locals.var_bin_l * p.p594);
        let assign2610_e3676: f64 = (p.p543 + assign2610_e3675);
        let assign2610_e3679: f64 = (locals.var_bin_w * p.p595);
        let assign2610_e3680: f64 = (assign2610_e3676 + assign2610_e3679);
        let assign2610_e3683: f64 = (locals.var_bin_wl * p.p596);
        let assign2610_e3684: f64 = (assign2610_e3680 + assign2610_e3683);
        locals.var_bigc_i = assign2610_e3684;

        let assign2620_e3688: f64 = (locals.var_bin_l * p.p597);
        let assign2620_e3689: f64 = (p.p544 + assign2620_e3688);
        let assign2620_e3692: f64 = (locals.var_bin_w * p.p598);
        let assign2620_e3693: f64 = (assign2620_e3689 + assign2620_e3692);
        let assign2620_e3696: f64 = (locals.var_bin_wl * p.p599);
        let assign2620_e3697: f64 = (assign2620_e3693 + assign2620_e3696);
        locals.var_cigc_i = assign2620_e3697;

        let assign2630_e3701: f64 = (locals.var_bin_l * p.p600);
        let assign2630_e3702: f64 = (p.p545 + assign2630_e3701);
        let assign2630_e3705: f64 = (locals.var_bin_w * p.p601);
        let assign2630_e3706: f64 = (assign2630_e3702 + assign2630_e3705);
        let assign2630_e3709: f64 = (locals.var_bin_wl * p.p602);
        let assign2630_e3710: f64 = (assign2630_e3706 + assign2630_e3709);
        locals.var_aigs_i = assign2630_e3710;

        let assign2640_e3714: f64 = (locals.var_bin_l * p.p603);
        let assign2640_e3715: f64 = (p.p546 + assign2640_e3714);
        let assign2640_e3718: f64 = (locals.var_bin_w * p.p604);
        let assign2640_e3719: f64 = (assign2640_e3715 + assign2640_e3718);
        let assign2640_e3722: f64 = (locals.var_bin_wl * p.p605);
        let assign2640_e3723: f64 = (assign2640_e3719 + assign2640_e3722);
        locals.var_bigs_i = assign2640_e3723;

        let assign2650_e3727: f64 = (locals.var_bin_l * p.p606);
        let assign2650_e3728: f64 = (p.p547 + assign2650_e3727);
        let assign2650_e3731: f64 = (locals.var_bin_w * p.p607);
        let assign2650_e3732: f64 = (assign2650_e3728 + assign2650_e3731);
        let assign2650_e3735: f64 = (locals.var_bin_wl * p.p608);
        let assign2650_e3736: f64 = (assign2650_e3732 + assign2650_e3735);
        locals.var_cigs_i = assign2650_e3736;

        let assign2660_e3740: f64 = (locals.var_bin_l * p.p609);
        let assign2660_e3741: f64 = (p.p548 + assign2660_e3740);
        let assign2660_e3744: f64 = (locals.var_bin_w * p.p610);
        let assign2660_e3745: f64 = (assign2660_e3741 + assign2660_e3744);
        let assign2660_e3748: f64 = (locals.var_bin_wl * p.p611);
        let assign2660_e3749: f64 = (assign2660_e3745 + assign2660_e3748);
        locals.var_aigd_i = assign2660_e3749;

        let assign2670_e3753: f64 = (locals.var_bin_l * p.p612);
        let assign2670_e3754: f64 = (p.p549 + assign2670_e3753);
        let assign2670_e3757: f64 = (locals.var_bin_w * p.p613);
        let assign2670_e3758: f64 = (assign2670_e3754 + assign2670_e3757);
        let assign2670_e3761: f64 = (locals.var_bin_wl * p.p614);
        let assign2670_e3762: f64 = (assign2670_e3758 + assign2670_e3761);
        locals.var_bigd_i = assign2670_e3762;

        let assign2680_e3766: f64 = (locals.var_bin_l * p.p615);
        let assign2680_e3767: f64 = (p.p550 + assign2680_e3766);
        let assign2680_e3770: f64 = (locals.var_bin_w * p.p616);
        let assign2680_e3771: f64 = (assign2680_e3767 + assign2680_e3770);
        let assign2680_e3774: f64 = (locals.var_bin_wl * p.p617);
        let assign2680_e3775: f64 = (assign2680_e3771 + assign2680_e3774);
        locals.var_cigd_i = assign2680_e3775;

        let assign2690_e3779: f64 = (locals.var_bin_l * p.p618);
        let assign2690_e3780: f64 = (p.p553 + assign2690_e3779);
        let assign2690_e3783: f64 = (locals.var_bin_w * p.p619);
        let assign2690_e3784: f64 = (assign2690_e3780 + assign2690_e3783);
        let assign2690_e3787: f64 = (locals.var_bin_wl * p.p620);
        let assign2690_e3788: f64 = (assign2690_e3784 + assign2690_e3787);
        locals.var_poxedge_i = assign2690_e3788;

        let assign2700_e3792: f64 = (locals.var_bin_l * p.p621);
        let assign2700_e3793: f64 = (p.p551 + assign2700_e3792);
        let assign2700_e3796: f64 = (locals.var_bin_w * p.p622);
        let assign2700_e3797: f64 = (assign2700_e3793 + assign2700_e3796);
        let assign2700_e3800: f64 = (locals.var_bin_wl * p.p623);
        let assign2700_e3801: f64 = (assign2700_e3797 + assign2700_e3800);
        locals.var_dlcig_i = assign2700_e3801;

        let assign2710_e3805: f64 = (locals.var_bin_l * p.p624);
        let assign2710_e3806: f64 = (p.p552 + assign2710_e3805);
        let assign2710_e3809: f64 = (locals.var_bin_w * p.p625);
        let assign2710_e3810: f64 = (assign2710_e3806 + assign2710_e3809);
        let assign2710_e3813: f64 = (locals.var_bin_wl * p.p626);
        let assign2710_e3814: f64 = (assign2710_e3810 + assign2710_e3813);
        locals.var_dlcigd_i = assign2710_e3814;

        let assign2720_e3818: f64 = (locals.var_bin_l * p.p627);
        let assign2720_e3819: f64 = (p.p554 + assign2720_e3818);
        let assign2720_e3822: f64 = (locals.var_bin_w * p.p628);
        let assign2720_e3823: f64 = (assign2720_e3819 + assign2720_e3822);
        let assign2720_e3826: f64 = (locals.var_bin_wl * p.p629);
        let assign2720_e3827: f64 = (assign2720_e3823 + assign2720_e3826);
        locals.var_ntox_i = assign2720_e3827;

        let assign2730_e3831: f64 = (locals.var_bin_l * p.p870);
        let assign2730_e3832: f64 = (p.p867 + assign2730_e3831);
        let assign2730_e3835: f64 = (locals.var_bin_w * p.p871);
        let assign2730_e3836: f64 = (assign2730_e3832 + assign2730_e3835);
        let assign2730_e3839: f64 = (locals.var_bin_wl * p.p872);
        let assign2730_e3840: f64 = (assign2730_e3836 + assign2730_e3839);
        locals.var_kt1_i = assign2730_e3840;

        let assign2740_e3844: f64 = (locals.var_bin_l * p.p874);
        let assign2740_e3845: f64 = (p.p873 + assign2740_e3844);
        let assign2740_e3848: f64 = (locals.var_bin_w * p.p875);
        let assign2740_e3849: f64 = (assign2740_e3845 + assign2740_e3848);
        let assign2740_e3852: f64 = (locals.var_bin_wl * p.p876);
        let assign2740_e3853: f64 = (assign2740_e3849 + assign2740_e3852);
        locals.var_kt2_i = assign2740_e3853;

        let assign2750_e3857: f64 = (locals.var_bin_l * p.p430);
        let assign2750_e3858: f64 = (p.p425 + assign2750_e3857);
        let assign2750_e3861: f64 = (locals.var_bin_w * p.p431);
        let assign2750_e3862: f64 = (assign2750_e3858 + assign2750_e3861);
        let assign2750_e3865: f64 = (locals.var_bin_wl * p.p432);
        let assign2750_e3866: f64 = (assign2750_e3862 + assign2750_e3865);
        locals.var_psatb_i = assign2750_e3866;

        let assign2760_e3870: f64 = (locals.var_bin_l * p.p445);
        let assign2760_e3871: f64 = (p.p444 + assign2760_e3870);
        let assign2760_e3874: f64 = (locals.var_bin_w * p.p446);
        let assign2760_e3875: f64 = (assign2760_e3871 + assign2760_e3874);
        let assign2760_e3878: f64 = (locals.var_bin_wl * p.p447);
        let assign2760_e3879: f64 = (assign2760_e3875 + assign2760_e3878);
        locals.var_a1_i = assign2760_e3879;

        let assign2770_e3883: f64 = (locals.var_bin_l * p.p449);
        let assign2770_e3884: f64 = (p.p448 + assign2770_e3883);
        let assign2770_e3887: f64 = (locals.var_bin_w * p.p450);
        let assign2770_e3888: f64 = (assign2770_e3884 + assign2770_e3887);
        let assign2770_e3891: f64 = (locals.var_bin_wl * p.p451);
        let assign2770_e3892: f64 = (assign2770_e3888 + assign2770_e3891);
        locals.var_a11_i = assign2770_e3892;

        let assign2780_e3896: f64 = (locals.var_bin_l * p.p453);
        let assign2780_e3897: f64 = (p.p452 + assign2780_e3896);
        let assign2780_e3900: f64 = (locals.var_bin_w * p.p454);
        let assign2780_e3901: f64 = (assign2780_e3897 + assign2780_e3900);
        let assign2780_e3904: f64 = (locals.var_bin_wl * p.p455);
        let assign2780_e3905: f64 = (assign2780_e3901 + assign2780_e3904);
        locals.var_a2_i = assign2780_e3905;

        let assign2790_e3909: f64 = (locals.var_bin_l * p.p457);
        let assign2790_e3910: f64 = (p.p456 + assign2790_e3909);
        let assign2790_e3913: f64 = (locals.var_bin_w * p.p458);
        let assign2790_e3914: f64 = (assign2790_e3910 + assign2790_e3913);
        let assign2790_e3917: f64 = (locals.var_bin_wl * p.p459);
        let assign2790_e3918: f64 = (assign2790_e3914 + assign2790_e3917);
        locals.var_a21_i = assign2790_e3918;

        let assign2800_e3922: f64 = (locals.var_bin_l * p.p1047);
        let assign2800_e3923: f64 = (p.p1046 + assign2800_e3922);
        let assign2800_e3926: f64 = (locals.var_bin_w * p.p1048);
        let assign2800_e3927: f64 = (assign2800_e3923 + assign2800_e3926);
        let assign2800_e3930: f64 = (locals.var_bin_wl * p.p1049);
        let assign2800_e3931: f64 = (assign2800_e3927 + assign2800_e3930);
        locals.var_k0_i = assign2800_e3931;

        let assign2810_e3935: f64 = (locals.var_bin_l * p.p1055);
        let assign2810_e3936: f64 = (p.p1054 + assign2810_e3935);
        let assign2810_e3939: f64 = (locals.var_bin_w * p.p1056);
        let assign2810_e3940: f64 = (assign2810_e3936 + assign2810_e3939);
        let assign2810_e3943: f64 = (locals.var_bin_wl * p.p1057);
        let assign2810_e3944: f64 = (assign2810_e3940 + assign2810_e3943);
        locals.var_m0_i = assign2810_e3944;

        let assign2820_e3948: f64 = (locals.var_bin_l * p.p1051);
        let assign2820_e3949: f64 = (p.p1050 + assign2820_e3948);
        let assign2820_e3952: f64 = (locals.var_bin_w * p.p1052);
        let assign2820_e3953: f64 = (assign2820_e3949 + assign2820_e3952);
        let assign2820_e3956: f64 = (locals.var_bin_wl * p.p1053);
        let assign2820_e3957: f64 = (assign2820_e3953 + assign2820_e3956);
        locals.var_k01_i = assign2820_e3957;

        let assign2830_e3961: f64 = (locals.var_bin_l * p.p1059);
        let assign2830_e3962: f64 = (p.p1058 + assign2830_e3961);
        let assign2830_e3965: f64 = (locals.var_bin_w * p.p1060);
        let assign2830_e3966: f64 = (assign2830_e3962 + assign2830_e3965);
        let assign2830_e3969: f64 = (locals.var_bin_wl * p.p1061);
        let assign2830_e3970: f64 = (assign2830_e3966 + assign2830_e3969);
        locals.var_m01_i = assign2830_e3970;

        let assign2840_e3974: f64 = (locals.var_bin_l * p.p967);
        let assign2840_e3975: f64 = (p.p966 + assign2840_e3974);
        let assign2840_e3978: f64 = (locals.var_bin_w * p.p968);
        let assign2840_e3979: f64 = (assign2840_e3975 + assign2840_e3978);
        let assign2840_e3982: f64 = (locals.var_bin_wl * p.p969);
        let assign2840_e3983: f64 = (assign2840_e3979 + assign2840_e3982);
        locals.var_nfactoredge_i = assign2840_e3983;

        let assign2850_e3987: f64 = (locals.var_bin_l * p.p963);
        let assign2850_e3988: f64 = (p.p962 + assign2850_e3987);
        let assign2850_e3991: f64 = (locals.var_bin_w * p.p964);
        let assign2850_e3992: f64 = (assign2850_e3988 + assign2850_e3991);
        let assign2850_e3995: f64 = (locals.var_bin_wl * p.p965);
        let assign2850_e3996: f64 = (assign2850_e3992 + assign2850_e3995);
        locals.var_ndepedge_i = assign2850_e3996;

        let assign2860_e4000: f64 = (locals.var_bin_l * p.p971);
        let assign2860_e4001: f64 = (p.p970 + assign2860_e4000);
        let assign2860_e4004: f64 = (locals.var_bin_w * p.p972);
        let assign2860_e4005: f64 = (assign2860_e4001 + assign2860_e4004);
        let assign2860_e4008: f64 = (locals.var_bin_wl * p.p973);
        let assign2860_e4009: f64 = (assign2860_e4005 + assign2860_e4008);
        locals.var_citedge_i = assign2860_e4009;

        let assign2870_e4013: f64 = (locals.var_bin_l * p.p975);
        let assign2870_e4014: f64 = (p.p974 + assign2870_e4013);
        let assign2870_e4017: f64 = (locals.var_bin_w * p.p976);
        let assign2870_e4018: f64 = (assign2870_e4014 + assign2870_e4017);
        let assign2870_e4021: f64 = (locals.var_bin_wl * p.p977);
        let assign2870_e4022: f64 = (assign2870_e4018 + assign2870_e4021);
        locals.var_cdscdedge_i = assign2870_e4022;

        let assign2880_e4026: f64 = (locals.var_bin_l * p.p979);
        let assign2880_e4027: f64 = (p.p978 + assign2880_e4026);
        let assign2880_e4030: f64 = (locals.var_bin_w * p.p980);
        let assign2880_e4031: f64 = (assign2880_e4027 + assign2880_e4030);
        let assign2880_e4034: f64 = (locals.var_bin_wl * p.p981);
        let assign2880_e4035: f64 = (assign2880_e4031 + assign2880_e4034);
        locals.var_cdscbedge_i = assign2880_e4035;

        let assign2890_e4039: f64 = (locals.var_bin_l * p.p983);
        let assign2890_e4040: f64 = (p.p982 + assign2890_e4039);
        let assign2890_e4043: f64 = (locals.var_bin_w * p.p984);
        let assign2890_e4044: f64 = (assign2890_e4040 + assign2890_e4043);
        let assign2890_e4047: f64 = (locals.var_bin_wl * p.p985);
        let assign2890_e4048: f64 = (assign2890_e4044 + assign2890_e4047);
        locals.var_eta0edge_i = assign2890_e4048;
        locals.var_eta0edge_i_dn0 = 0.0;
        locals.var_eta0edge_i_dn2 = 0.0;
        locals.var_eta0edge_i_dn3 = 0.0;
        locals.var_eta0edge_i_dn4 = 0.0;
        locals.var_eta0edge_i_dn5 = 0.0;
        locals.var_eta0edge_i_dn6 = 0.0;
        locals.var_eta0edge_i_dn7 = 0.0;
        locals.var_eta0edge_i_dn8 = 0.0;
        locals.var_eta0edge_i_dn9 = 0.0;
        locals.var_eta0edge_i_dn10 = 0.0;
        locals.var_eta0edge_i_dn11 = 0.0;
        locals.var_eta0edge_i_dn12 = 0.0;
        locals.var_eta0edge_i_dn13 = 0.0;
        locals.var_eta0edge_i_dn14 = 0.0;

        let assign2900_e4052: f64 = (locals.var_bin_l * p.p987);
        let assign2900_e4053: f64 = (p.p986 + assign2900_e4052);
        let assign2900_e4056: f64 = (locals.var_bin_w * p.p988);
        let assign2900_e4057: f64 = (assign2900_e4053 + assign2900_e4056);
        let assign2900_e4060: f64 = (locals.var_bin_wl * p.p989);
        let assign2900_e4061: f64 = (assign2900_e4057 + assign2900_e4060);
        locals.var_etabedge_i = assign2900_e4061;

        let assign2910_e4065: f64 = (locals.var_bin_l * p.p991);
        let assign2910_e4066: f64 = (p.p990 + assign2910_e4065);
        let assign2910_e4069: f64 = (locals.var_bin_w * p.p992);
        let assign2910_e4070: f64 = (assign2910_e4066 + assign2910_e4069);
        let assign2910_e4073: f64 = (locals.var_bin_wl * p.p993);
        let assign2910_e4074: f64 = (assign2910_e4070 + assign2910_e4073);
        locals.var_kt1edge_i = assign2910_e4074;

        let assign2920_e4078: f64 = (locals.var_bin_l * p.p995);
        let assign2920_e4079: f64 = (p.p994 + assign2920_e4078);
        let assign2920_e4082: f64 = (locals.var_bin_w * p.p996);
        let assign2920_e4083: f64 = (assign2920_e4079 + assign2920_e4082);
        let assign2920_e4086: f64 = (locals.var_bin_wl * p.p997);
        let assign2920_e4087: f64 = (assign2920_e4083 + assign2920_e4086);
        locals.var_kt1ledge_i = assign2920_e4087;

        let assign2930_e4091: f64 = (locals.var_bin_l * p.p999);
        let assign2930_e4092: f64 = (p.p998 + assign2930_e4091);
        let assign2930_e4095: f64 = (locals.var_bin_w * p.p1000);
        let assign2930_e4096: f64 = (assign2930_e4092 + assign2930_e4095);
        let assign2930_e4099: f64 = (locals.var_bin_wl * p.p1001);
        let assign2930_e4100: f64 = (assign2930_e4096 + assign2930_e4099);
        locals.var_kt2edge_i = assign2930_e4100;

        let assign2940_e4104: f64 = (locals.var_bin_l * p.p1003);
        let assign2940_e4105: f64 = (p.p1002 + assign2940_e4104);
        let assign2940_e4108: f64 = (locals.var_bin_w * p.p1004);
        let assign2940_e4109: f64 = (assign2940_e4105 + assign2940_e4108);
        let assign2940_e4112: f64 = (locals.var_bin_wl * p.p1005);
        let assign2940_e4113: f64 = (assign2940_e4109 + assign2940_e4112);
        locals.var_kt1expedge_i = assign2940_e4113;

        let assign2950_e4117: f64 = (locals.var_bin_l * p.p1007);
        let assign2950_e4118: f64 = (p.p1006 + assign2950_e4117);
        let assign2950_e4121: f64 = (locals.var_bin_w * p.p1008);
        let assign2950_e4122: f64 = (assign2950_e4118 + assign2950_e4121);
        let assign2950_e4125: f64 = (locals.var_bin_wl * p.p1009);
        let assign2950_e4126: f64 = (assign2950_e4122 + assign2950_e4125);
        locals.var_tnfactoredge_i = assign2950_e4126;

        let assign2960_e4130: f64 = (locals.var_bin_l * p.p1011);
        let assign2960_e4131: f64 = (p.p1010 + assign2960_e4130);
        let assign2960_e4134: f64 = (locals.var_bin_w * p.p1012);
        let assign2960_e4135: f64 = (assign2960_e4131 + assign2960_e4134);
        let assign2960_e4138: f64 = (locals.var_bin_wl * p.p1013);
        let assign2960_e4139: f64 = (assign2960_e4135 + assign2960_e4138);
        locals.var_teta0edge_i = assign2960_e4139;

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2970_e4143: f64 = (locals.var_bin_l * p.p1018);
        let assign2970_e4144: f64 = (p.p1017 + assign2970_e4143);
        let assign2970_e4147: f64 = (locals.var_bin_w * p.p1019);
        let assign2970_e4148: f64 = (assign2970_e4144 + assign2970_e4147);
        let assign2970_e4151: f64 = (locals.var_bin_wl * p.p1020);
        let assign2970_e4152: f64 = (assign2970_e4148 + assign2970_e4151);
        locals.var_k2edge_i = assign2970_e4152;
        locals.var_k2edge_i_dn0 = 0.0;
        locals.var_k2edge_i_dn2 = 0.0;
        locals.var_k2edge_i_dn3 = 0.0;
        locals.var_k2edge_i_dn4 = 0.0;
        locals.var_k2edge_i_dn5 = 0.0;
        locals.var_k2edge_i_dn6 = 0.0;
        locals.var_k2edge_i_dn7 = 0.0;
        locals.var_k2edge_i_dn8 = 0.0;
        locals.var_k2edge_i_dn9 = 0.0;
        locals.var_k2edge_i_dn10 = 0.0;
        locals.var_k2edge_i_dn11 = 0.0;
        locals.var_k2edge_i_dn12 = 0.0;
        locals.var_k2edge_i_dn13 = 0.0;
        locals.var_k2edge_i_dn14 = 0.0;

        let assign2980_e4156: f64 = (locals.var_bin_l * p.p1022);
        let assign2980_e4157: f64 = (p.p1021 + assign2980_e4156);
        let assign2980_e4160: f64 = (locals.var_bin_w * p.p1023);
        let assign2980_e4161: f64 = (assign2980_e4157 + assign2980_e4160);
        let assign2980_e4164: f64 = (locals.var_bin_wl * p.p1024);
        let assign2980_e4165: f64 = (assign2980_e4161 + assign2980_e4164);
        locals.var_kvth0edge_i = assign2980_e4165;

        let assign2990_e4169: f64 = (locals.var_bin_l * p.p1030);
        let assign2990_e4170: f64 = (p.p1029 + assign2990_e4169);
        let assign2990_e4173: f64 = (locals.var_bin_w * p.p1031);
        let assign2990_e4174: f64 = (assign2990_e4170 + assign2990_e4173);
        let assign2990_e4177: f64 = (locals.var_bin_wl * p.p1032);
        let assign2990_e4178: f64 = (assign2990_e4174 + assign2990_e4177);
        locals.var_k2edgewe_i = assign2990_e4178;

        let assign3000_e4182: f64 = (locals.var_bin_l * p.p1026);
        let assign3000_e4183: f64 = (p.p1025 + assign3000_e4182);
        let assign3000_e4186: f64 = (locals.var_bin_w * p.p1027);
        let assign3000_e4187: f64 = (assign3000_e4183 + assign3000_e4186);
        let assign3000_e4190: f64 = (locals.var_bin_wl * p.p1028);
        let assign3000_e4191: f64 = (assign3000_e4187 + assign3000_e4190);
        locals.var_kvth0edgewe_i = assign3000_e4191;

        let assign3010_e4195: f64 = (locals.var_bin_l * p.p1034);
        let assign3010_e4196: f64 = (p.p1033 + assign3010_e4195);
        let assign3010_e4199: f64 = (locals.var_bin_w * p.p1035);
        let assign3010_e4200: f64 = (assign3010_e4196 + assign3010_e4199);
        let assign3010_e4203: f64 = (locals.var_bin_wl * p.p1036);
        let assign3010_e4204: f64 = (assign3010_e4200 + assign3010_e4203);
        locals.var_stk2edge_i = assign3010_e4204;

        let assign3020_e4208: f64 = (locals.var_bin_l * p.p1038);
        let assign3020_e4209: f64 = (p.p1037 + assign3020_e4208);
        let assign3020_e4212: f64 = (locals.var_bin_w * p.p1039);
        let assign3020_e4213: f64 = (assign3020_e4209 + assign3020_e4212);
        let assign3020_e4216: f64 = (locals.var_bin_wl * p.p1040);
        let assign3020_e4217: f64 = (assign3020_e4213 + assign3020_e4216);
        locals.var_steta0edge_i = assign3020_e4217;

        let assign3030_e4221: f64 = (locals.var_bin_l * p.p1070);
        let assign3030_e4222: f64 = (p.p1069 + assign3030_e4221);
        let assign3030_e4225: f64 = (locals.var_bin_w * p.p1071);
        let assign3030_e4226: f64 = (assign3030_e4222 + assign3030_e4225);
        let assign3030_e4229: f64 = (locals.var_bin_wl * p.p1072);
        let assign3030_e4230: f64 = (assign3030_e4226 + assign3030_e4229);
        locals.var_c0_i = assign3030_e4230;

        let assign3040_e4234: f64 = (locals.var_bin_l * p.p1074);
        let assign3040_e4235: f64 = (p.p1073 + assign3040_e4234);
        let assign3040_e4238: f64 = (locals.var_bin_w * p.p1075);
        let assign3040_e4239: f64 = (assign3040_e4235 + assign3040_e4238);
        let assign3040_e4242: f64 = (locals.var_bin_wl * p.p1076);
        let assign3040_e4243: f64 = (assign3040_e4239 + assign3040_e4242);
        locals.var_c01_i = assign3040_e4243;

        let assign3050_e4247: f64 = (locals.var_bin_l * p.p1078);
        let assign3050_e4248: f64 = (p.p1077 + assign3050_e4247);
        let assign3050_e4251: f64 = (locals.var_bin_w * p.p1079);
        let assign3050_e4252: f64 = (assign3050_e4248 + assign3050_e4251);
        let assign3050_e4255: f64 = (locals.var_bin_wl * p.p1080);
        let assign3050_e4256: f64 = (assign3050_e4252 + assign3050_e4255);
        locals.var_c0si_i = assign3050_e4256;

        let assign3060_e4260: f64 = (locals.var_bin_l * p.p1082);
        let assign3060_e4261: f64 = (p.p1081 + assign3060_e4260);
        let assign3060_e4264: f64 = (locals.var_bin_w * p.p1083);
        let assign3060_e4265: f64 = (assign3060_e4261 + assign3060_e4264);
        let assign3060_e4268: f64 = (locals.var_bin_wl * p.p1084);
        let assign3060_e4269: f64 = (assign3060_e4265 + assign3060_e4268);
        locals.var_c0si1_i = assign3060_e4269;

        let assign3070_e4273: f64 = (locals.var_bin_l * p.p1086);
        let assign3070_e4274: f64 = (p.p1085 + assign3070_e4273);
        let assign3070_e4277: f64 = (locals.var_bin_w * p.p1087);
        let assign3070_e4278: f64 = (assign3070_e4274 + assign3070_e4277);
        let assign3070_e4281: f64 = (locals.var_bin_wl * p.p1088);
        let assign3070_e4282: f64 = (assign3070_e4278 + assign3070_e4281);
        locals.var_c0sisat_i = assign3070_e4282;

        let assign3080_e4286: f64 = (locals.var_bin_l * p.p1090);
        let assign3080_e4287: f64 = (p.p1089 + assign3080_e4286);
        let assign3080_e4290: f64 = (locals.var_bin_w * p.p1091);
        let assign3080_e4291: f64 = (assign3080_e4287 + assign3080_e4290);
        let assign3080_e4294: f64 = (locals.var_bin_wl * p.p1092);
        let assign3080_e4295: f64 = (assign3080_e4291 + assign3080_e4294);
        locals.var_c0sisat1_i = assign3080_e4295;

        let assign3090_e4299: f64 = (locals.var_bin_l * p.p787);
        let assign3090_e4300: f64 = (p.p786 + assign3090_e4299);
        let assign3090_e4303: f64 = (locals.var_bin_w * p.p788);
        let assign3090_e4304: f64 = (assign3090_e4300 + assign3090_e4303);
        let assign3090_e4307: f64 = (locals.var_bin_wl * p.p789);
        let assign3090_e4308: f64 = (assign3090_e4304 + assign3090_e4307);
        locals.var_noia3_i = assign3090_e4308;

        let assign3100_e4312: f64 = (locals.var_bin_l * p.p795);
        let assign3100_e4313: f64 = (p.p794 + assign3100_e4312);
        let assign3100_e4316: f64 = (locals.var_bin_w * p.p796);
        let assign3100_e4317: f64 = (assign3100_e4313 + assign3100_e4316);
        let assign3100_e4320: f64 = (locals.var_bin_wl * p.p797);
        let assign3100_e4321: f64 = (assign3100_e4317 + assign3100_e4320);
        locals.var_qsref_i = assign3100_e4321;

        let assign3110_e4325: f64 = (locals.var_bin_l * p.p791);
        let assign3110_e4326: f64 = (p.p790 + assign3110_e4325);
        let assign3110_e4329: f64 = (locals.var_bin_w * p.p792);
        let assign3110_e4330: f64 = (assign3110_e4326 + assign3110_e4329);
        let assign3110_e4333: f64 = (locals.var_bin_wl * p.p793);
        let assign3110_e4334: f64 = (assign3110_e4330 + assign3110_e4333);
        locals.var_mpower_i = assign3110_e4334;

        let assign3120_e4337: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign3120_e4337;

        let (assign3130_e4353, assign3130_e4353_d_n0, assign3130_e4353_d_n2, assign3130_e4353_d_n3, assign3130_e4353_d_n4, assign3130_e4353_d_n5, assign3130_e4353_d_n6, assign3130_e4353_d_n7, assign3130_e4353_d_n8, assign3130_e4353_d_n9, assign3130_e4353_d_n10, assign3130_e4353_d_n11, assign3130_e4353_d_n12, assign3130_e4353_d_n13, assign3130_e4353_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3130_e4342: f64 = (locals.var_bin_l * p.p230);
        let assign3130_e4343: f64 = (p.p229 + assign3130_e4342);
        let assign3130_e4346: f64 = (locals.var_bin_w * p.p231);
        let assign3130_e4347: f64 = (assign3130_e4343 + assign3130_e4346);
        let assign3130_e4350: f64 = (locals.var_bin_wl * p.p232);
        let assign3130_e4351: f64 = (assign3130_e4347 + assign3130_e4350);
        (assign3130_e4351, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cdscdr_i, locals.var_cdscdr_i_dn0, locals.var_cdscdr_i_dn2, locals.var_cdscdr_i_dn3, locals.var_cdscdr_i_dn4, locals.var_cdscdr_i_dn5, locals.var_cdscdr_i_dn6, locals.var_cdscdr_i_dn7, locals.var_cdscdr_i_dn8, locals.var_cdscdr_i_dn9, locals.var_cdscdr_i_dn10, locals.var_cdscdr_i_dn11, locals.var_cdscdr_i_dn12, locals.var_cdscdr_i_dn13, locals.var_cdscdr_i_dn14,)
    }
};
        locals.var_cdscdr_i = assign3130_e4353;
        locals.var_cdscdr_i_dn0 = assign3130_e4353_d_n0;
        locals.var_cdscdr_i_dn2 = assign3130_e4353_d_n2;
        locals.var_cdscdr_i_dn3 = assign3130_e4353_d_n3;
        locals.var_cdscdr_i_dn4 = assign3130_e4353_d_n4;
        locals.var_cdscdr_i_dn5 = assign3130_e4353_d_n5;
        locals.var_cdscdr_i_dn6 = assign3130_e4353_d_n6;
        locals.var_cdscdr_i_dn7 = assign3130_e4353_d_n7;
        locals.var_cdscdr_i_dn8 = assign3130_e4353_d_n8;
        locals.var_cdscdr_i_dn9 = assign3130_e4353_d_n9;
        locals.var_cdscdr_i_dn10 = assign3130_e4353_d_n10;
        locals.var_cdscdr_i_dn11 = assign3130_e4353_d_n11;
        locals.var_cdscdr_i_dn12 = assign3130_e4353_d_n12;
        locals.var_cdscdr_i_dn13 = assign3130_e4353_d_n13;
        locals.var_cdscdr_i_dn14 = assign3130_e4353_d_n14;

        let (assign3140_e4369, assign3140_e4369_d_n0, assign3140_e4369_d_n2, assign3140_e4369_d_n3, assign3140_e4369_d_n4, assign3140_e4369_d_n5, assign3140_e4369_d_n6, assign3140_e4369_d_n7, assign3140_e4369_d_n8, assign3140_e4369_d_n9, assign3140_e4369_d_n10, assign3140_e4369_d_n11, assign3140_e4369_d_n12, assign3140_e4369_d_n13, assign3140_e4369_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3140_e4358: f64 = (locals.var_bin_l * p.p176);
        let assign3140_e4359: f64 = (p.p175 + assign3140_e4358);
        let assign3140_e4362: f64 = (locals.var_bin_w * p.p177);
        let assign3140_e4363: f64 = (assign3140_e4359 + assign3140_e4362);
        let assign3140_e4366: f64 = (locals.var_bin_wl * p.p178);
        let assign3140_e4367: f64 = (assign3140_e4363 + assign3140_e4366);
        (assign3140_e4367, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eta0r_i, locals.var_eta0r_i_dn0, locals.var_eta0r_i_dn2, locals.var_eta0r_i_dn3, locals.var_eta0r_i_dn4, locals.var_eta0r_i_dn5, locals.var_eta0r_i_dn6, locals.var_eta0r_i_dn7, locals.var_eta0r_i_dn8, locals.var_eta0r_i_dn9, locals.var_eta0r_i_dn10, locals.var_eta0r_i_dn11, locals.var_eta0r_i_dn12, locals.var_eta0r_i_dn13, locals.var_eta0r_i_dn14,)
    }
};
        locals.var_eta0r_i = assign3140_e4369;
        locals.var_eta0r_i_dn0 = assign3140_e4369_d_n0;
        locals.var_eta0r_i_dn2 = assign3140_e4369_d_n2;
        locals.var_eta0r_i_dn3 = assign3140_e4369_d_n3;
        locals.var_eta0r_i_dn4 = assign3140_e4369_d_n4;
        locals.var_eta0r_i_dn5 = assign3140_e4369_d_n5;
        locals.var_eta0r_i_dn6 = assign3140_e4369_d_n6;
        locals.var_eta0r_i_dn7 = assign3140_e4369_d_n7;
        locals.var_eta0r_i_dn8 = assign3140_e4369_d_n8;
        locals.var_eta0r_i_dn9 = assign3140_e4369_d_n9;
        locals.var_eta0r_i_dn10 = assign3140_e4369_d_n10;
        locals.var_eta0r_i_dn11 = assign3140_e4369_d_n11;
        locals.var_eta0r_i_dn12 = assign3140_e4369_d_n12;
        locals.var_eta0r_i_dn13 = assign3140_e4369_d_n13;
        locals.var_eta0r_i_dn14 = assign3140_e4369_d_n14;

        let (assign3150_e4385,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3150_e4374: f64 = (locals.var_bin_l * p.p280);
        let assign3150_e4375: f64 = (p.p279 + assign3150_e4374);
        let assign3150_e4378: f64 = (locals.var_bin_w * p.p281);
        let assign3150_e4379: f64 = (assign3150_e4375 + assign3150_e4378);
        let assign3150_e4382: f64 = (locals.var_bin_wl * p.p282);
        let assign3150_e4383: f64 = (assign3150_e4379 + assign3150_e4382);
        (assign3150_e4383,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign3150_e4385;

        let (assign3160_e4401, assign3160_e4401_d_n0, assign3160_e4401_d_n2, assign3160_e4401_d_n3, assign3160_e4401_d_n4, assign3160_e4401_d_n5, assign3160_e4401_d_n6, assign3160_e4401_d_n7, assign3160_e4401_d_n8, assign3160_e4401_d_n9, assign3160_e4401_d_n10, assign3160_e4401_d_n11, assign3160_e4401_d_n12, assign3160_e4401_d_n13, assign3160_e4401_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3160_e4390: f64 = (locals.var_bin_l * p.p295);
        let assign3160_e4391: f64 = (p.p294 + assign3160_e4390);
        let assign3160_e4394: f64 = (locals.var_bin_w * p.p296);
        let assign3160_e4395: f64 = (assign3160_e4391 + assign3160_e4394);
        let assign3160_e4398: f64 = (locals.var_bin_wl * p.p297);
        let assign3160_e4399: f64 = (assign3160_e4395 + assign3160_e4398);
        (assign3160_e4399, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uar_i, locals.var_uar_i_dn0, locals.var_uar_i_dn2, locals.var_uar_i_dn3, locals.var_uar_i_dn4, locals.var_uar_i_dn5, locals.var_uar_i_dn6, locals.var_uar_i_dn7, locals.var_uar_i_dn8, locals.var_uar_i_dn9, locals.var_uar_i_dn10, locals.var_uar_i_dn11, locals.var_uar_i_dn12, locals.var_uar_i_dn13, locals.var_uar_i_dn14,)
    }
};
        locals.var_uar_i = assign3160_e4401;
        locals.var_uar_i_dn0 = assign3160_e4401_d_n0;
        locals.var_uar_i_dn2 = assign3160_e4401_d_n2;
        locals.var_uar_i_dn3 = assign3160_e4401_d_n3;
        locals.var_uar_i_dn4 = assign3160_e4401_d_n4;
        locals.var_uar_i_dn5 = assign3160_e4401_d_n5;
        locals.var_uar_i_dn6 = assign3160_e4401_d_n6;
        locals.var_uar_i_dn7 = assign3160_e4401_d_n7;
        locals.var_uar_i_dn8 = assign3160_e4401_d_n8;
        locals.var_uar_i_dn9 = assign3160_e4401_d_n9;
        locals.var_uar_i_dn10 = assign3160_e4401_d_n10;
        locals.var_uar_i_dn11 = assign3160_e4401_d_n11;
        locals.var_uar_i_dn12 = assign3160_e4401_d_n12;
        locals.var_uar_i_dn13 = assign3160_e4401_d_n13;
        locals.var_uar_i_dn14 = assign3160_e4401_d_n14;

        let (assign3170_e4417, assign3170_e4417_d_n0, assign3170_e4417_d_n2, assign3170_e4417_d_n3, assign3170_e4417_d_n4, assign3170_e4417_d_n5, assign3170_e4417_d_n6, assign3170_e4417_d_n7, assign3170_e4417_d_n8, assign3170_e4417_d_n9, assign3170_e4417_d_n10, assign3170_e4417_d_n11, assign3170_e4417_d_n12, assign3170_e4417_d_n13, assign3170_e4417_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3170_e4406: f64 = (locals.var_bin_l * p.p315);
        let assign3170_e4407: f64 = (p.p314 + assign3170_e4406);
        let assign3170_e4410: f64 = (locals.var_bin_w * p.p316);
        let assign3170_e4411: f64 = (assign3170_e4407 + assign3170_e4410);
        let assign3170_e4414: f64 = (locals.var_bin_wl * p.p317);
        let assign3170_e4415: f64 = (assign3170_e4411 + assign3170_e4414);
        (assign3170_e4415, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_udr_i, locals.var_udr_i_dn0, locals.var_udr_i_dn2, locals.var_udr_i_dn3, locals.var_udr_i_dn4, locals.var_udr_i_dn5, locals.var_udr_i_dn6, locals.var_udr_i_dn7, locals.var_udr_i_dn8, locals.var_udr_i_dn9, locals.var_udr_i_dn10, locals.var_udr_i_dn11, locals.var_udr_i_dn12, locals.var_udr_i_dn13, locals.var_udr_i_dn14,)
    }
};
        locals.var_udr_i = assign3170_e4417;
        locals.var_udr_i_dn0 = assign3170_e4417_d_n0;
        locals.var_udr_i_dn2 = assign3170_e4417_d_n2;
        locals.var_udr_i_dn3 = assign3170_e4417_d_n3;
        locals.var_udr_i_dn4 = assign3170_e4417_d_n4;
        locals.var_udr_i_dn5 = assign3170_e4417_d_n5;
        locals.var_udr_i_dn6 = assign3170_e4417_d_n6;
        locals.var_udr_i_dn7 = assign3170_e4417_d_n7;
        locals.var_udr_i_dn8 = assign3170_e4417_d_n8;
        locals.var_udr_i_dn9 = assign3170_e4417_d_n9;
        locals.var_udr_i_dn10 = assign3170_e4417_d_n10;
        locals.var_udr_i_dn11 = assign3170_e4417_d_n11;
        locals.var_udr_i_dn12 = assign3170_e4417_d_n12;
        locals.var_udr_i_dn13 = assign3170_e4417_d_n13;
        locals.var_udr_i_dn14 = assign3170_e4417_d_n14;

        let (assign3180_e4433,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3180_e4422: f64 = (locals.var_bin_l * p.p323);
        let assign3180_e4423: f64 = (p.p322 + assign3180_e4422);
        let assign3180_e4426: f64 = (locals.var_bin_w * p.p324);
        let assign3180_e4427: f64 = (assign3180_e4423 + assign3180_e4426);
        let assign3180_e4430: f64 = (locals.var_bin_wl * p.p325);
        let assign3180_e4431: f64 = (assign3180_e4427 + assign3180_e4430);
        (assign3180_e4431,)
    } else {
        (locals.var_ucsr_i,)
    }
};
        locals.var_ucsr_i = assign3180_e4433;

        let (assign3190_e4449, assign3190_e4449_d_n0, assign3190_e4449_d_n2, assign3190_e4449_d_n3, assign3190_e4449_d_n4, assign3190_e4449_d_n5, assign3190_e4449_d_n6, assign3190_e4449_d_n7, assign3190_e4449_d_n8, assign3190_e4449_d_n9, assign3190_e4449_d_n10, assign3190_e4449_d_n11, assign3190_e4449_d_n12, assign3190_e4449_d_n13, assign3190_e4449_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3190_e4438: f64 = (locals.var_bin_l * p.p337);
        let assign3190_e4439: f64 = (p.p336 + assign3190_e4438);
        let assign3190_e4442: f64 = (locals.var_bin_w * p.p338);
        let assign3190_e4443: f64 = (assign3190_e4439 + assign3190_e4442);
        let assign3190_e4446: f64 = (locals.var_bin_wl * p.p339);
        let assign3190_e4447: f64 = (assign3190_e4443 + assign3190_e4446);
        (assign3190_e4447, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ucr_i, locals.var_ucr_i_dn0, locals.var_ucr_i_dn2, locals.var_ucr_i_dn3, locals.var_ucr_i_dn4, locals.var_ucr_i_dn5, locals.var_ucr_i_dn6, locals.var_ucr_i_dn7, locals.var_ucr_i_dn8, locals.var_ucr_i_dn9, locals.var_ucr_i_dn10, locals.var_ucr_i_dn11, locals.var_ucr_i_dn12, locals.var_ucr_i_dn13, locals.var_ucr_i_dn14,)
    }
};
        locals.var_ucr_i = assign3190_e4449;
        locals.var_ucr_i_dn0 = assign3190_e4449_d_n0;
        locals.var_ucr_i_dn2 = assign3190_e4449_d_n2;
        locals.var_ucr_i_dn3 = assign3190_e4449_d_n3;
        locals.var_ucr_i_dn4 = assign3190_e4449_d_n4;
        locals.var_ucr_i_dn5 = assign3190_e4449_d_n5;
        locals.var_ucr_i_dn6 = assign3190_e4449_d_n6;
        locals.var_ucr_i_dn7 = assign3190_e4449_d_n7;
        locals.var_ucr_i_dn8 = assign3190_e4449_d_n8;
        locals.var_ucr_i_dn9 = assign3190_e4449_d_n9;
        locals.var_ucr_i_dn10 = assign3190_e4449_d_n10;
        locals.var_ucr_i_dn11 = assign3190_e4449_d_n11;
        locals.var_ucr_i_dn12 = assign3190_e4449_d_n12;
        locals.var_ucr_i_dn13 = assign3190_e4449_d_n13;
        locals.var_ucr_i_dn14 = assign3190_e4449_d_n14;

        let (assign3200_e4465, assign3200_e4465_d_n0, assign3200_e4465_d_n2, assign3200_e4465_d_n3, assign3200_e4465_d_n4, assign3200_e4465_d_n5, assign3200_e4465_d_n6, assign3200_e4465_d_n7, assign3200_e4465_d_n8, assign3200_e4465_d_n9, assign3200_e4465_d_n10, assign3200_e4465_d_n11, assign3200_e4465_d_n12, assign3200_e4465_d_n13, assign3200_e4465_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3200_e4454: f64 = (locals.var_bin_l * p.p347);
        let assign3200_e4455: f64 = (p.p346 + assign3200_e4454);
        let assign3200_e4458: f64 = (locals.var_bin_w * p.p348);
        let assign3200_e4459: f64 = (assign3200_e4455 + assign3200_e4458);
        let assign3200_e4462: f64 = (locals.var_bin_wl * p.p349);
        let assign3200_e4463: f64 = (assign3200_e4459 + assign3200_e4462);
        (assign3200_e4463, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pclmr_i, locals.var_pclmr_i_dn0, locals.var_pclmr_i_dn2, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11, locals.var_pclmr_i_dn12, locals.var_pclmr_i_dn13, locals.var_pclmr_i_dn14,)
    }
};
        locals.var_pclmr_i = assign3200_e4465;
        locals.var_pclmr_i_dn0 = assign3200_e4465_d_n0;
        locals.var_pclmr_i_dn2 = assign3200_e4465_d_n2;
        locals.var_pclmr_i_dn3 = assign3200_e4465_d_n3;
        locals.var_pclmr_i_dn4 = assign3200_e4465_d_n4;
        locals.var_pclmr_i_dn5 = assign3200_e4465_d_n5;
        locals.var_pclmr_i_dn6 = assign3200_e4465_d_n6;
        locals.var_pclmr_i_dn7 = assign3200_e4465_d_n7;
        locals.var_pclmr_i_dn8 = assign3200_e4465_d_n8;
        locals.var_pclmr_i_dn9 = assign3200_e4465_d_n9;
        locals.var_pclmr_i_dn10 = assign3200_e4465_d_n10;
        locals.var_pclmr_i_dn11 = assign3200_e4465_d_n11;
        locals.var_pclmr_i_dn12 = assign3200_e4465_d_n12;
        locals.var_pclmr_i_dn13 = assign3200_e4465_d_n13;
        locals.var_pclmr_i_dn14 = assign3200_e4465_d_n14;

        let (assign3210_e4481, assign3210_e4481_d_n0, assign3210_e4481_d_n2, assign3210_e4481_d_n3, assign3210_e4481_d_n4, assign3210_e4481_d_n5, assign3210_e4481_d_n6, assign3210_e4481_d_n7, assign3210_e4481_d_n8, assign3210_e4481_d_n9, assign3210_e4481_d_n10, assign3210_e4481_d_n11, assign3210_e4481_d_n12, assign3210_e4481_d_n13, assign3210_e4481_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3210_e4470: f64 = (locals.var_bin_l * p.p467);
        let assign3210_e4471: f64 = (p.p466 + assign3210_e4470);
        let assign3210_e4474: f64 = (locals.var_bin_w * p.p468);
        let assign3210_e4475: f64 = (assign3210_e4471 + assign3210_e4474);
        let assign3210_e4478: f64 = (locals.var_bin_wl * p.p469);
        let assign3210_e4479: f64 = (assign3210_e4475 + assign3210_e4478);
        (assign3210_e4479, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdiblcr_i, locals.var_pdiblcr_i_dn0, locals.var_pdiblcr_i_dn2, locals.var_pdiblcr_i_dn3, locals.var_pdiblcr_i_dn4, locals.var_pdiblcr_i_dn5, locals.var_pdiblcr_i_dn6, locals.var_pdiblcr_i_dn7, locals.var_pdiblcr_i_dn8, locals.var_pdiblcr_i_dn9, locals.var_pdiblcr_i_dn10, locals.var_pdiblcr_i_dn11, locals.var_pdiblcr_i_dn12, locals.var_pdiblcr_i_dn13, locals.var_pdiblcr_i_dn14,)
    }
};
        locals.var_pdiblcr_i = assign3210_e4481;
        locals.var_pdiblcr_i_dn0 = assign3210_e4481_d_n0;
        locals.var_pdiblcr_i_dn2 = assign3210_e4481_d_n2;
        locals.var_pdiblcr_i_dn3 = assign3210_e4481_d_n3;
        locals.var_pdiblcr_i_dn4 = assign3210_e4481_d_n4;
        locals.var_pdiblcr_i_dn5 = assign3210_e4481_d_n5;
        locals.var_pdiblcr_i_dn6 = assign3210_e4481_d_n6;
        locals.var_pdiblcr_i_dn7 = assign3210_e4481_d_n7;
        locals.var_pdiblcr_i_dn8 = assign3210_e4481_d_n8;
        locals.var_pdiblcr_i_dn9 = assign3210_e4481_d_n9;
        locals.var_pdiblcr_i_dn10 = assign3210_e4481_d_n10;
        locals.var_pdiblcr_i_dn11 = assign3210_e4481_d_n11;
        locals.var_pdiblcr_i_dn12 = assign3210_e4481_d_n12;
        locals.var_pdiblcr_i_dn13 = assign3210_e4481_d_n13;
        locals.var_pdiblcr_i_dn14 = assign3210_e4481_d_n14;

        let (assign3220_e4497, assign3220_e4497_d_n0, assign3220_e4497_d_n2, assign3220_e4497_d_n3, assign3220_e4497_d_n4, assign3220_e4497_d_n5, assign3220_e4497_d_n6, assign3220_e4497_d_n7, assign3220_e4497_d_n8, assign3220_e4497_d_n9, assign3220_e4497_d_n10, assign3220_e4497_d_n11, assign3220_e4497_d_n12, assign3220_e4497_d_n13, assign3220_e4497_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3220_e4486: f64 = (locals.var_bin_l * p.p250);
        let assign3220_e4487: f64 = (p.p249 + assign3220_e4486);
        let assign3220_e4490: f64 = (locals.var_bin_w * p.p251);
        let assign3220_e4491: f64 = (assign3220_e4487 + assign3220_e4490);
        let assign3220_e4494: f64 = (locals.var_bin_wl * p.p252);
        let assign3220_e4495: f64 = (assign3220_e4491 + assign3220_e4494);
        (assign3220_e4495, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsatr_i, locals.var_vsatr_i_dn0, locals.var_vsatr_i_dn2, locals.var_vsatr_i_dn3, locals.var_vsatr_i_dn4, locals.var_vsatr_i_dn5, locals.var_vsatr_i_dn6, locals.var_vsatr_i_dn7, locals.var_vsatr_i_dn8, locals.var_vsatr_i_dn9, locals.var_vsatr_i_dn10, locals.var_vsatr_i_dn11, locals.var_vsatr_i_dn12, locals.var_vsatr_i_dn13, locals.var_vsatr_i_dn14,)
    }
};
        locals.var_vsatr_i = assign3220_e4497;
        locals.var_vsatr_i_dn0 = assign3220_e4497_d_n0;
        locals.var_vsatr_i_dn2 = assign3220_e4497_d_n2;
        locals.var_vsatr_i_dn3 = assign3220_e4497_d_n3;
        locals.var_vsatr_i_dn4 = assign3220_e4497_d_n4;
        locals.var_vsatr_i_dn5 = assign3220_e4497_d_n5;
        locals.var_vsatr_i_dn6 = assign3220_e4497_d_n6;
        locals.var_vsatr_i_dn7 = assign3220_e4497_d_n7;
        locals.var_vsatr_i_dn8 = assign3220_e4497_d_n8;
        locals.var_vsatr_i_dn9 = assign3220_e4497_d_n9;
        locals.var_vsatr_i_dn10 = assign3220_e4497_d_n10;
        locals.var_vsatr_i_dn11 = assign3220_e4497_d_n11;
        locals.var_vsatr_i_dn12 = assign3220_e4497_d_n12;
        locals.var_vsatr_i_dn13 = assign3220_e4497_d_n13;
        locals.var_vsatr_i_dn14 = assign3220_e4497_d_n14;

        let (assign3230_e4513,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3230_e4502: f64 = (locals.var_bin_l * p.p427);
        let assign3230_e4503: f64 = (p.p426 + assign3230_e4502);
        let assign3230_e4506: f64 = (locals.var_bin_w * p.p428);
        let assign3230_e4507: f64 = (assign3230_e4503 + assign3230_e4506);
        let assign3230_e4510: f64 = (locals.var_bin_wl * p.p429);
        let assign3230_e4511: f64 = (assign3230_e4507 + assign3230_e4510);
        (assign3230_e4511,)
    } else {
        (locals.var_psatr_i,)
    }
};
        locals.var_psatr_i = assign3230_e4513;

        let (assign3240_e4529, assign3240_e4529_d_n0, assign3240_e4529_d_n2, assign3240_e4529_d_n3, assign3240_e4529_d_n4, assign3240_e4529_d_n5, assign3240_e4529_d_n6, assign3240_e4529_d_n7, assign3240_e4529_d_n8, assign3240_e4529_d_n9, assign3240_e4529_d_n10, assign3240_e4529_d_n11, assign3240_e4529_d_n12, assign3240_e4529_d_n13, assign3240_e4529_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3240_e4518: f64 = (locals.var_bin_l * p.p441);
        let assign3240_e4519: f64 = (p.p440 + assign3240_e4518);
        let assign3240_e4522: f64 = (locals.var_bin_w * p.p442);
        let assign3240_e4523: f64 = (assign3240_e4519 + assign3240_e4522);
        let assign3240_e4526: f64 = (locals.var_bin_wl * p.p443);
        let assign3240_e4527: f64 = (assign3240_e4523 + assign3240_e4526);
        (assign3240_e4527, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ptwgr_i, locals.var_ptwgr_i_dn0, locals.var_ptwgr_i_dn2, locals.var_ptwgr_i_dn3, locals.var_ptwgr_i_dn4, locals.var_ptwgr_i_dn5, locals.var_ptwgr_i_dn6, locals.var_ptwgr_i_dn7, locals.var_ptwgr_i_dn8, locals.var_ptwgr_i_dn9, locals.var_ptwgr_i_dn10, locals.var_ptwgr_i_dn11, locals.var_ptwgr_i_dn12, locals.var_ptwgr_i_dn13, locals.var_ptwgr_i_dn14,)
    }
};
        locals.var_ptwgr_i = assign3240_e4529;
        locals.var_ptwgr_i_dn0 = assign3240_e4529_d_n0;
        locals.var_ptwgr_i_dn2 = assign3240_e4529_d_n2;
        locals.var_ptwgr_i_dn3 = assign3240_e4529_d_n3;
        locals.var_ptwgr_i_dn4 = assign3240_e4529_d_n4;
        locals.var_ptwgr_i_dn5 = assign3240_e4529_d_n5;
        locals.var_ptwgr_i_dn6 = assign3240_e4529_d_n6;
        locals.var_ptwgr_i_dn7 = assign3240_e4529_d_n7;
        locals.var_ptwgr_i_dn8 = assign3240_e4529_d_n8;
        locals.var_ptwgr_i_dn9 = assign3240_e4529_d_n9;
        locals.var_ptwgr_i_dn10 = assign3240_e4529_d_n10;
        locals.var_ptwgr_i_dn11 = assign3240_e4529_d_n11;
        locals.var_ptwgr_i_dn12 = assign3240_e4529_d_n12;
        locals.var_ptwgr_i_dn13 = assign3240_e4529_d_n13;
        locals.var_ptwgr_i_dn14 = assign3240_e4529_d_n14;

        let (assign3250_e4545, assign3250_e4545_d_n0, assign3250_e4545_d_n2, assign3250_e4545_d_n3, assign3250_e4545_d_n4, assign3250_e4545_d_n5, assign3250_e4545_d_n6, assign3250_e4545_d_n7, assign3250_e4545_d_n8, assign3250_e4545_d_n9, assign3250_e4545_d_n10, assign3250_e4545_d_n11, assign3250_e4545_d_n12, assign3250_e4545_d_n13, assign3250_e4545_d_n14,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3250_e4534: f64 = (locals.var_bin_l * p.p526);
        let assign3250_e4535: f64 = (p.p525 + assign3250_e4534);
        let assign3250_e4538: f64 = (locals.var_bin_w * p.p527);
        let assign3250_e4539: f64 = (assign3250_e4535 + assign3250_e4538);
        let assign3250_e4542: f64 = (locals.var_bin_wl * p.p528);
        let assign3250_e4543: f64 = (assign3250_e4539 + assign3250_e4542);
        (assign3250_e4543, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_alpha0r_i, locals.var_alpha0r_i_dn0, locals.var_alpha0r_i_dn2, locals.var_alpha0r_i_dn3, locals.var_alpha0r_i_dn4, locals.var_alpha0r_i_dn5, locals.var_alpha0r_i_dn6, locals.var_alpha0r_i_dn7, locals.var_alpha0r_i_dn8, locals.var_alpha0r_i_dn9, locals.var_alpha0r_i_dn10, locals.var_alpha0r_i_dn11, locals.var_alpha0r_i_dn12, locals.var_alpha0r_i_dn13, locals.var_alpha0r_i_dn14,)
    }
};
        locals.var_alpha0r_i = assign3250_e4545;
        locals.var_alpha0r_i_dn0 = assign3250_e4545_d_n0;
        locals.var_alpha0r_i_dn2 = assign3250_e4545_d_n2;
        locals.var_alpha0r_i_dn3 = assign3250_e4545_d_n3;
        locals.var_alpha0r_i_dn4 = assign3250_e4545_d_n4;
        locals.var_alpha0r_i_dn5 = assign3250_e4545_d_n5;
        locals.var_alpha0r_i_dn6 = assign3250_e4545_d_n6;
        locals.var_alpha0r_i_dn7 = assign3250_e4545_d_n7;
        locals.var_alpha0r_i_dn8 = assign3250_e4545_d_n8;
        locals.var_alpha0r_i_dn9 = assign3250_e4545_d_n9;
        locals.var_alpha0r_i_dn10 = assign3250_e4545_d_n10;
        locals.var_alpha0r_i_dn11 = assign3250_e4545_d_n11;
        locals.var_alpha0r_i_dn12 = assign3250_e4545_d_n12;
        locals.var_alpha0r_i_dn13 = assign3250_e4545_d_n13;
        locals.var_alpha0r_i_dn14 = assign3250_e4545_d_n14;

        let (assign3260_e4561,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3260_e4550: f64 = (locals.var_bin_l * p.p530);
        let assign3260_e4551: f64 = (p.p529 + assign3260_e4550);
        let assign3260_e4554: f64 = (locals.var_bin_w * p.p531);
        let assign3260_e4555: f64 = (assign3260_e4551 + assign3260_e4554);
        let assign3260_e4558: f64 = (locals.var_bin_wl * p.p532);
        let assign3260_e4559: f64 = (assign3260_e4555 + assign3260_e4558);
        (assign3260_e4559,)
    } else {
        (locals.var_beta0r_i,)
    }
};
        locals.var_beta0r_i = assign3260_e4561;

    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3270_e4565: f64 = (locals.var_inv_l).powf(p.p82);
        let assign3270_e4568: f64 = (locals.var_inv_llong).powf(p.p82);
        let assign3270_e4569: f64 = (assign3270_e4565 - assign3270_e4568);
        let assign3270_e4571: f64 = (assign3270_e4569).max(0.0);
        let assign3270_e4572: f64 = (p.p81 * assign3270_e4571);
        let assign3270_e4576: f64 = (locals.var_inv_l).powf(p.p84);
        let assign3270_e4579: f64 = (locals.var_inv_llong).powf(p.p84);
        let assign3270_e4580: f64 = (assign3270_e4576 - assign3270_e4579);
        let assign3270_e4582: f64 = (assign3270_e4580).max(0.0);
        let assign3270_e4583: f64 = (p.p83 * assign3270_e4582);
        let assign3270_e4584: f64 = (assign3270_e4572 + assign3270_e4583);
        locals.var_t0 = assign3270_e4584;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign3280_e4588: f64 = (locals.var_inv_w).powf(p.p86);
        let assign3280_e4591: f64 = (locals.var_inv_wwide).powf(p.p86);
        let assign3280_e4592: f64 = (assign3280_e4588 - assign3280_e4591);
        let assign3280_e4594: f64 = (assign3280_e4592).max(0.0);
        let assign3280_e4595: f64 = (p.p85 * assign3280_e4594);
        let assign3280_e4599: f64 = (locals.var_inv_w * locals.var_inv_l);
        let assign3280_e4601: f64 = (assign3280_e4599).powf(p.p88);
        let assign3280_e4602: f64 = (p.p87 * assign3280_e4601);
        let assign3280_e4603: f64 = (assign3280_e4595 + assign3280_e4602);
        locals.var_t1 = assign3280_e4603;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign3290_e4607: f64 = (1.0 + locals.var_t0);
        let assign3290_e4609: f64 = (assign3290_e4607 + locals.var_t1);
        let assign3290_e4610: f64 = (locals.var_ndep_i * assign3290_e4609);
        locals.var_ndep_i = assign3290_e4610;
        locals.var_ndep_i_dn0 = ((locals.var_ndep_i_dn0 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_ndep_i_dn2 = ((locals.var_ndep_i_dn2 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_ndep_i_dn3 = ((locals.var_ndep_i_dn3 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_ndep_i_dn4 = ((locals.var_ndep_i_dn4 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_ndep_i_dn5 = ((locals.var_ndep_i_dn5 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_ndep_i_dn6 = ((locals.var_ndep_i_dn6 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_ndep_i_dn7 = ((locals.var_ndep_i_dn7 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_ndep_i_dn8 = ((locals.var_ndep_i_dn8 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_ndep_i_dn9 = ((locals.var_ndep_i_dn9 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_ndep_i_dn10 = ((locals.var_ndep_i_dn10 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_ndep_i_dn11 = ((locals.var_ndep_i_dn11 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_ndep_i_dn12 = ((locals.var_ndep_i_dn12 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_ndep_i_dn13 = ((locals.var_ndep_i_dn13 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_ndep_i_dn14 = ((locals.var_ndep_i_dn14 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));

        let assign3300_e4614: f64 = (locals.var_inv_l).powf(p.p215);
        let assign3300_e4617: f64 = (locals.var_inv_llong).powf(p.p215);
        let assign3300_e4618: f64 = (assign3300_e4614 - assign3300_e4617);
        let assign3300_e4620: f64 = (assign3300_e4618).max(0.0);
        let assign3300_e4621: f64 = (p.p214 * assign3300_e4620);
        locals.var_t0 = assign3300_e4621;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign3310_e4625: f64 = (locals.var_inv_w).powf(p.p217);
        let assign3310_e4628: f64 = (locals.var_inv_wwide).powf(p.p217);
        let assign3310_e4629: f64 = (assign3310_e4625 - assign3310_e4628);
        let assign3310_e4631: f64 = (assign3310_e4629).max(0.0);
        let assign3310_e4632: f64 = (p.p216 * assign3310_e4631);
        let assign3310_e4636: f64 = (locals.var_inv_wl).powf(p.p219);
        let assign3310_e4637: f64 = (p.p218 * assign3310_e4636);
        let assign3310_e4638: f64 = (assign3310_e4632 + assign3310_e4637);
        locals.var_t1 = assign3310_e4638;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign3320_e4642: f64 = (1.0 + locals.var_t0);
        let assign3320_e4644: f64 = (assign3320_e4642 + locals.var_t1);
        let assign3320_e4645: f64 = (locals.var_nfactor_i * assign3320_e4644);
        locals.var_nfactor_i = assign3320_e4645;
        locals.var_nfactor_i_dn0 = ((locals.var_nfactor_i_dn0 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_nfactor_i_dn2 = ((locals.var_nfactor_i_dn2 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_nfactor_i_dn3 = ((locals.var_nfactor_i_dn3 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_nfactor_i_dn4 = ((locals.var_nfactor_i_dn4 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_nfactor_i_dn5 = ((locals.var_nfactor_i_dn5 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_nfactor_i_dn6 = ((locals.var_nfactor_i_dn6 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_nfactor_i_dn7 = ((locals.var_nfactor_i_dn7 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_nfactor_i_dn8 = ((locals.var_nfactor_i_dn8 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_nfactor_i_dn9 = ((locals.var_nfactor_i_dn9 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_nfactor_i_dn10 = ((locals.var_nfactor_i_dn10 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_nfactor_i_dn11 = ((locals.var_nfactor_i_dn11 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_nfactor_i_dn12 = ((locals.var_nfactor_i_dn12 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_nfactor_i_dn13 = ((locals.var_nfactor_i_dn13 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_nfactor_i_dn14 = ((locals.var_nfactor_i_dn14 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));

        let assign3330_e4650: f64 = (locals.var_inv_l).powf(p.p225);
        let assign3330_e4653: f64 = (locals.var_inv_llong).powf(p.p225);
        let assign3330_e4654: f64 = (assign3330_e4650 - assign3330_e4653);
        let assign3330_e4656: f64 = (assign3330_e4654).max(0.0);
        let assign3330_e4657: f64 = (p.p224 * assign3330_e4656);
        let assign3330_e4658: f64 = (1.0 + assign3330_e4657);
        locals.var_t0 = assign3330_e4658;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign3340_e4661: f64 = (locals.var_cdscd_i * locals.var_t0);
        locals.var_cdscd_i = assign3340_e4661;
        locals.var_cdscd_i_dn0 = ((locals.var_cdscd_i_dn0 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn0));
        locals.var_cdscd_i_dn2 = ((locals.var_cdscd_i_dn2 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn2));
        locals.var_cdscd_i_dn3 = ((locals.var_cdscd_i_dn3 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn3));
        locals.var_cdscd_i_dn4 = ((locals.var_cdscd_i_dn4 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn4));
        locals.var_cdscd_i_dn5 = ((locals.var_cdscd_i_dn5 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn5));
        locals.var_cdscd_i_dn6 = ((locals.var_cdscd_i_dn6 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn6));
        locals.var_cdscd_i_dn7 = ((locals.var_cdscd_i_dn7 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn7));
        locals.var_cdscd_i_dn8 = ((locals.var_cdscd_i_dn8 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn8));
        locals.var_cdscd_i_dn9 = ((locals.var_cdscd_i_dn9 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn9));
        locals.var_cdscd_i_dn10 = ((locals.var_cdscd_i_dn10 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn10));
        locals.var_cdscd_i_dn11 = ((locals.var_cdscd_i_dn11 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn11));
        locals.var_cdscd_i_dn12 = ((locals.var_cdscd_i_dn12 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn12));
        locals.var_cdscd_i_dn13 = ((locals.var_cdscd_i_dn13 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn13));
        locals.var_cdscd_i_dn14 = ((locals.var_cdscd_i_dn14 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn14));

        let assign3350_e4664: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign3350_e4664;

        let (assign3360_e4670, assign3360_e4670_d_n0, assign3360_e4670_d_n2, assign3360_e4670_d_n3, assign3360_e4670_d_n4, assign3360_e4670_d_n5, assign3360_e4670_d_n6, assign3360_e4670_d_n7, assign3360_e4670_d_n8, assign3360_e4670_d_n9, assign3360_e4670_d_n10, assign3360_e4670_d_n11, assign3360_e4670_d_n12, assign3360_e4670_d_n13, assign3360_e4670_d_n14,) = {
    if (locals.var_guard22 != 0.0) {
        let assign3360_e4668: f64 = (locals.var_cdscdr_i * locals.var_t0);
        (assign3360_e4668, ((locals.var_cdscdr_i_dn0 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn0)), ((locals.var_cdscdr_i_dn2 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn2)), ((locals.var_cdscdr_i_dn3 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn3)), ((locals.var_cdscdr_i_dn4 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn4)), ((locals.var_cdscdr_i_dn5 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn5)), ((locals.var_cdscdr_i_dn6 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn6)), ((locals.var_cdscdr_i_dn7 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn7)), ((locals.var_cdscdr_i_dn8 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn8)), ((locals.var_cdscdr_i_dn9 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn9)), ((locals.var_cdscdr_i_dn10 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn10)), ((locals.var_cdscdr_i_dn11 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn11)), ((locals.var_cdscdr_i_dn12 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn12)), ((locals.var_cdscdr_i_dn13 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn13)), ((locals.var_cdscdr_i_dn14 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn14)),)
    } else {
        (locals.var_cdscdr_i, locals.var_cdscdr_i_dn0, locals.var_cdscdr_i_dn2, locals.var_cdscdr_i_dn3, locals.var_cdscdr_i_dn4, locals.var_cdscdr_i_dn5, locals.var_cdscdr_i_dn6, locals.var_cdscdr_i_dn7, locals.var_cdscdr_i_dn8, locals.var_cdscdr_i_dn9, locals.var_cdscdr_i_dn10, locals.var_cdscdr_i_dn11, locals.var_cdscdr_i_dn12, locals.var_cdscdr_i_dn13, locals.var_cdscdr_i_dn14,)
    }
};
        locals.var_cdscdr_i = assign3360_e4670;
        locals.var_cdscdr_i_dn0 = assign3360_e4670_d_n0;
        locals.var_cdscdr_i_dn2 = assign3360_e4670_d_n2;
        locals.var_cdscdr_i_dn3 = assign3360_e4670_d_n3;
        locals.var_cdscdr_i_dn4 = assign3360_e4670_d_n4;
        locals.var_cdscdr_i_dn5 = assign3360_e4670_d_n5;
        locals.var_cdscdr_i_dn6 = assign3360_e4670_d_n6;
        locals.var_cdscdr_i_dn7 = assign3360_e4670_d_n7;
        locals.var_cdscdr_i_dn8 = assign3360_e4670_d_n8;
        locals.var_cdscdr_i_dn9 = assign3360_e4670_d_n9;
        locals.var_cdscdr_i_dn10 = assign3360_e4670_d_n10;
        locals.var_cdscdr_i_dn11 = assign3360_e4670_d_n11;
        locals.var_cdscdr_i_dn12 = assign3360_e4670_d_n12;
        locals.var_cdscdr_i_dn13 = assign3360_e4670_d_n13;
        locals.var_cdscdr_i_dn14 = assign3360_e4670_d_n14;

        let assign3370_e4676: f64 = (locals.var_inv_l).powf(p.p235);
        let assign3370_e4679: f64 = (locals.var_inv_llong).powf(p.p235);
        let assign3370_e4680: f64 = (assign3370_e4676 - assign3370_e4679);
        let assign3370_e4682: f64 = (assign3370_e4680).max(0.0);
        let assign3370_e4683: f64 = (p.p234 * assign3370_e4682);
        let assign3370_e4684: f64 = (1.0 + assign3370_e4683);
        let assign3370_e4685: f64 = (locals.var_cdscb_i * assign3370_e4684);
        locals.var_cdscb_i = assign3370_e4685;

        let assign3380_e4688: f64 = (p.p34 * locals.var_u0_i);
        locals.var_u0_i = assign3380_e4688;

        let assign3390_e4691: f64 = if p.p50 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard23 = assign3390_e4691;

        let assign3400_e4694: f64 = if p.p275 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard24 = assign3400_e4694;

        let (assign3410_e4714,) = {
    if ((locals.var_guard23 != 0.0) && (locals.var_guard24 != 0.0)) {
        let assign3410_e4703: f64 = (locals.var_inv_l).powf(p.p275);
        let assign3410_e4706: f64 = (locals.var_inv_llong).powf(p.p275);
        let assign3410_e4707: f64 = (assign3410_e4703 - assign3410_e4706);
        let assign3410_e4709: f64 = (assign3410_e4707).max(0.0);
        let assign3410_e4710: f64 = (p.p274 * assign3410_e4709);
        let assign3410_e4711: f64 = (1.0 - assign3410_e4710);
        let assign3410_e4712: f64 = (locals.var_u0_i * assign3410_e4711);
        (assign3410_e4712,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign3410_e4714;

        let assign3420_e4717: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard25 = assign3420_e4717;

        let (assign3430_e4739,) = {
    if (((locals.var_guard23 != 0.0) && (locals.var_guard24 != 0.0)) && (locals.var_guard25 != 0.0)) {
        let assign3430_e4728: f64 = (locals.var_inv_l).powf(p.p275);
        let assign3430_e4731: f64 = (locals.var_inv_llong).powf(p.p275);
        let assign3430_e4732: f64 = (assign3430_e4728 - assign3430_e4731);
        let assign3430_e4734: f64 = (assign3430_e4732).max(0.0);
        let assign3430_e4735: f64 = (p.p274 * assign3430_e4734);
        let assign3430_e4736: f64 = (1.0 - assign3430_e4735);
        let assign3430_e4737: f64 = (locals.var_u0r_i * assign3430_e4736);
        (assign3430_e4737,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign3430_e4739;

        let (assign3440_e4750,) = {
    if ((locals.var_guard23 != 0.0) && (locals.var_guard24 == 0.0)) {
        let assign3440_e4747: f64 = (1.0 - p.p274);
        let assign3440_e4748: f64 = (locals.var_u0_i * assign3440_e4747);
        (assign3440_e4748,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign3440_e4750;

        let assign3450_e4753: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign3450_e4753;

        let (assign3460_e4766,) = {
    if (((locals.var_guard23 != 0.0) && (locals.var_guard24 == 0.0)) && (locals.var_guard26 != 0.0)) {
        let assign3460_e4763: f64 = (1.0 - p.p274);
        let assign3460_e4764: f64 = (locals.var_u0r_i * assign3460_e4763);
        (assign3460_e4764,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign3460_e4766;

        let (assign3470_e4789,) = {
    if (locals.var_guard23 == 0.0) {
        let assign3470_e4773: f64 = (-locals.var_leff);
        let assign3470_e4775: f64 = (assign3470_e4773 / p.p270);
        let assign3470_e4776: f64 = { let limited_exp_arg = assign3470_e4775; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign3470_e4777: f64 = (p.p269 * assign3470_e4776);
        let assign3470_e4778: f64 = (1.0 - assign3470_e4777);
        let assign3470_e4781: f64 = (-locals.var_leff);
        let assign3470_e4783: f64 = (assign3470_e4781 / p.p272);
        let assign3470_e4784: f64 = { let limited_exp_arg = assign3470_e4783; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign3470_e4785: f64 = (p.p271 * assign3470_e4784);
        let assign3470_e4786: f64 = (assign3470_e4778 - assign3470_e4785);
        let assign3470_e4787: f64 = (locals.var_u0_i * assign3470_e4786);
        (assign3470_e4787,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign3470_e4789;

        let assign3480_e4792: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard27 = assign3480_e4792;

        let (assign3490_e4817,) = {
    if ((locals.var_guard23 == 0.0) && (locals.var_guard27 != 0.0)) {
        let assign3490_e4801: f64 = (-locals.var_leff);
        let assign3490_e4803: f64 = (assign3490_e4801 / p.p270);
        let assign3490_e4804: f64 = { let limited_exp_arg = assign3490_e4803; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign3490_e4805: f64 = (p.p269 * assign3490_e4804);
        let assign3490_e4806: f64 = (1.0 - assign3490_e4805);
        let assign3490_e4809: f64 = (-locals.var_leff);
        let assign3490_e4811: f64 = (assign3490_e4809 / p.p272);
        let assign3490_e4812: f64 = { let limited_exp_arg = assign3490_e4811; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign3490_e4813: f64 = (p.p271 * assign3490_e4812);
        let assign3490_e4814: f64 = (assign3490_e4806 - assign3490_e4813);
        let assign3490_e4815: f64 = (locals.var_u0r_i * assign3490_e4814);
        (assign3490_e4815,)
    } else {
        (locals.var_u0r_i,)
    }
};
        locals.var_u0r_i = assign3490_e4817;

        let assign3500_e4821: f64 = (locals.var_inv_l).powf(p.p286);
        let assign3500_e4824: f64 = (locals.var_inv_llong).powf(p.p286);
        let assign3500_e4825: f64 = (assign3500_e4821 - assign3500_e4824);
        let assign3500_e4827: f64 = (assign3500_e4825).max(0.0);
        let assign3500_e4828: f64 = (p.p285 * assign3500_e4827);
        locals.var_t0 = assign3500_e4828;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign3510_e4832: f64 = (locals.var_inv_w).powf(p.p288);
        let assign3510_e4835: f64 = (locals.var_inv_wwide).powf(p.p288);
        let assign3510_e4836: f64 = (assign3510_e4832 - assign3510_e4835);
        let assign3510_e4838: f64 = (assign3510_e4836).max(0.0);
        let assign3510_e4839: f64 = (p.p287 * assign3510_e4838);
        let assign3510_e4843: f64 = (locals.var_inv_wl).powf(p.p290);
        let assign3510_e4844: f64 = (p.p289 * assign3510_e4843);
        let assign3510_e4845: f64 = (assign3510_e4839 + assign3510_e4844);
        locals.var_t1 = assign3510_e4845;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign3520_e4849: f64 = (1.0 + locals.var_t0);
        let assign3520_e4851: f64 = (assign3520_e4849 + locals.var_t1);
        let assign3520_e4852: f64 = (locals.var_ua_i * assign3520_e4851);
        locals.var_ua_i = assign3520_e4852;
        locals.var_ua_i_dn0 = ((locals.var_ua_i_dn0 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_ua_i_dn2 = ((locals.var_ua_i_dn2 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_ua_i_dn3 = ((locals.var_ua_i_dn3 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_ua_i_dn4 = ((locals.var_ua_i_dn4 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_ua_i_dn5 = ((locals.var_ua_i_dn5 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_ua_i_dn6 = ((locals.var_ua_i_dn6 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_ua_i_dn7 = ((locals.var_ua_i_dn7 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_ua_i_dn8 = ((locals.var_ua_i_dn8 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_ua_i_dn9 = ((locals.var_ua_i_dn9 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_ua_i_dn10 = ((locals.var_ua_i_dn10 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_ua_i_dn11 = ((locals.var_ua_i_dn11 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_ua_i_dn12 = ((locals.var_ua_i_dn12 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_ua_i_dn13 = ((locals.var_ua_i_dn13 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_ua_i_dn14 = ((locals.var_ua_i_dn14 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));

        let assign3530_e4855: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard28 = assign3530_e4855;

        let (assign3540_e4865, assign3540_e4865_d_n0, assign3540_e4865_d_n2, assign3540_e4865_d_n3, assign3540_e4865_d_n4, assign3540_e4865_d_n5, assign3540_e4865_d_n6, assign3540_e4865_d_n7, assign3540_e4865_d_n8, assign3540_e4865_d_n9, assign3540_e4865_d_n10, assign3540_e4865_d_n11, assign3540_e4865_d_n12, assign3540_e4865_d_n13, assign3540_e4865_d_n14,) = {
    if (locals.var_guard28 != 0.0) {
        let assign3540_e4860: f64 = (1.0 + locals.var_t0);
        let assign3540_e4862: f64 = (assign3540_e4860 + locals.var_t1);
        let assign3540_e4863: f64 = (locals.var_uar_i * assign3540_e4862);
        (assign3540_e4863, ((locals.var_uar_i_dn0 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_uar_i_dn2 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_uar_i_dn3 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_uar_i_dn4 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_uar_i_dn5 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_uar_i_dn6 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_uar_i_dn7 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_uar_i_dn8 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_uar_i_dn9 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_uar_i_dn10 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_uar_i_dn11 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_uar_i_dn12 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_uar_i_dn13 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_uar_i_dn14 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn14 + locals.var_t1_dn14))),)
    } else {
        (locals.var_uar_i, locals.var_uar_i_dn0, locals.var_uar_i_dn2, locals.var_uar_i_dn3, locals.var_uar_i_dn4, locals.var_uar_i_dn5, locals.var_uar_i_dn6, locals.var_uar_i_dn7, locals.var_uar_i_dn8, locals.var_uar_i_dn9, locals.var_uar_i_dn10, locals.var_uar_i_dn11, locals.var_uar_i_dn12, locals.var_uar_i_dn13, locals.var_uar_i_dn14,)
    }
};
        locals.var_uar_i = assign3540_e4865;
        locals.var_uar_i_dn0 = assign3540_e4865_d_n0;
        locals.var_uar_i_dn2 = assign3540_e4865_d_n2;
        locals.var_uar_i_dn3 = assign3540_e4865_d_n3;
        locals.var_uar_i_dn4 = assign3540_e4865_d_n4;
        locals.var_uar_i_dn5 = assign3540_e4865_d_n5;
        locals.var_uar_i_dn6 = assign3540_e4865_d_n6;
        locals.var_uar_i_dn7 = assign3540_e4865_d_n7;
        locals.var_uar_i_dn8 = assign3540_e4865_d_n8;
        locals.var_uar_i_dn9 = assign3540_e4865_d_n9;
        locals.var_uar_i_dn10 = assign3540_e4865_d_n10;
        locals.var_uar_i_dn11 = assign3540_e4865_d_n11;
        locals.var_uar_i_dn12 = assign3540_e4865_d_n12;
        locals.var_uar_i_dn13 = assign3540_e4865_d_n13;
        locals.var_uar_i_dn14 = assign3540_e4865_d_n14;

        let assign3550_e4869: f64 = (locals.var_inv_l).powf(p.p303);
        let assign3550_e4872: f64 = (locals.var_inv_llong).powf(p.p303);
        let assign3550_e4873: f64 = (assign3550_e4869 - assign3550_e4872);
        let assign3550_e4875: f64 = (assign3550_e4873).max(0.0);
        let assign3550_e4876: f64 = (p.p302 * assign3550_e4875);
        locals.var_t0 = assign3550_e4876;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign3560_e4880: f64 = (locals.var_inv_w).powf(p.p305);
        let assign3560_e4883: f64 = (locals.var_inv_wwide).powf(p.p305);
        let assign3560_e4884: f64 = (assign3560_e4880 - assign3560_e4883);
        let assign3560_e4886: f64 = (assign3560_e4884).max(0.0);
        let assign3560_e4887: f64 = (p.p304 * assign3560_e4886);
        let assign3560_e4891: f64 = (locals.var_inv_wl).powf(p.p307);
        let assign3560_e4892: f64 = (p.p306 * assign3560_e4891);
        let assign3560_e4893: f64 = (assign3560_e4887 + assign3560_e4892);
        locals.var_t1 = assign3560_e4893;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign3570_e4897: f64 = (1.0 + locals.var_t0);
        let assign3570_e4899: f64 = (assign3570_e4897 + locals.var_t1);
        let assign3570_e4900: f64 = (locals.var_eu_i * assign3570_e4899);
        locals.var_eu_i = assign3570_e4900;
        locals.var_eu_i_dn0 = ((locals.var_eu_i_dn0 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_eu_i_dn2 = ((locals.var_eu_i_dn2 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_eu_i_dn3 = ((locals.var_eu_i_dn3 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_eu_i_dn4 = ((locals.var_eu_i_dn4 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_eu_i_dn5 = ((locals.var_eu_i_dn5 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_eu_i_dn6 = ((locals.var_eu_i_dn6 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_eu_i_dn7 = ((locals.var_eu_i_dn7 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_eu_i_dn8 = ((locals.var_eu_i_dn8 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_eu_i_dn9 = ((locals.var_eu_i_dn9 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_eu_i_dn10 = ((locals.var_eu_i_dn10 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_eu_i_dn11 = ((locals.var_eu_i_dn11 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_eu_i_dn12 = ((locals.var_eu_i_dn12 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_eu_i_dn13 = ((locals.var_eu_i_dn13 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_eu_i_dn14 = ((locals.var_eu_i_dn14 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));

        let assign3580_e4905: f64 = (locals.var_inv_l).powf(p.p310);
        let assign3580_e4908: f64 = (locals.var_inv_llong).powf(p.p310);
        let assign3580_e4909: f64 = (assign3580_e4905 - assign3580_e4908);
        let assign3580_e4911: f64 = (assign3580_e4909).max(0.0);
        let assign3580_e4912: f64 = (p.p309 * assign3580_e4911);
        let assign3580_e4913: f64 = (1.0 + assign3580_e4912);
        locals.var_t0 = assign3580_e4913;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3590_e4916: f64 = (locals.var_ud_i * locals.var_t0);
        locals.var_ud_i = assign3590_e4916;
        locals.var_ud_i_dn0 = ((locals.var_ud_i_dn0 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn0));
        locals.var_ud_i_dn2 = ((locals.var_ud_i_dn2 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn2));
        locals.var_ud_i_dn3 = ((locals.var_ud_i_dn3 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn3));
        locals.var_ud_i_dn4 = ((locals.var_ud_i_dn4 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn4));
        locals.var_ud_i_dn5 = ((locals.var_ud_i_dn5 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn5));
        locals.var_ud_i_dn6 = ((locals.var_ud_i_dn6 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn6));
        locals.var_ud_i_dn7 = ((locals.var_ud_i_dn7 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn7));
        locals.var_ud_i_dn8 = ((locals.var_ud_i_dn8 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn8));
        locals.var_ud_i_dn9 = ((locals.var_ud_i_dn9 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn9));
        locals.var_ud_i_dn10 = ((locals.var_ud_i_dn10 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn10));
        locals.var_ud_i_dn11 = ((locals.var_ud_i_dn11 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn11));
        locals.var_ud_i_dn12 = ((locals.var_ud_i_dn12 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn12));
        locals.var_ud_i_dn13 = ((locals.var_ud_i_dn13 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn13));
        locals.var_ud_i_dn14 = ((locals.var_ud_i_dn14 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn14));

        let assign3600_e4919: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign3600_e4919;

        let (assign3610_e4925, assign3610_e4925_d_n0, assign3610_e4925_d_n2, assign3610_e4925_d_n3, assign3610_e4925_d_n4, assign3610_e4925_d_n5, assign3610_e4925_d_n6, assign3610_e4925_d_n7, assign3610_e4925_d_n8, assign3610_e4925_d_n9, assign3610_e4925_d_n10, assign3610_e4925_d_n11, assign3610_e4925_d_n12, assign3610_e4925_d_n13, assign3610_e4925_d_n14,) = {
    if (locals.var_guard29 != 0.0) {
        let assign3610_e4923: f64 = (locals.var_udr_i * locals.var_t0);
        (assign3610_e4923, ((locals.var_udr_i_dn0 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn0)), ((locals.var_udr_i_dn2 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn2)), ((locals.var_udr_i_dn3 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn3)), ((locals.var_udr_i_dn4 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn4)), ((locals.var_udr_i_dn5 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn5)), ((locals.var_udr_i_dn6 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn6)), ((locals.var_udr_i_dn7 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn7)), ((locals.var_udr_i_dn8 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn8)), ((locals.var_udr_i_dn9 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn9)), ((locals.var_udr_i_dn10 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn10)), ((locals.var_udr_i_dn11 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn11)), ((locals.var_udr_i_dn12 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn12)), ((locals.var_udr_i_dn13 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn13)), ((locals.var_udr_i_dn14 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn14)),)
    } else {
        (locals.var_udr_i, locals.var_udr_i_dn0, locals.var_udr_i_dn2, locals.var_udr_i_dn3, locals.var_udr_i_dn4, locals.var_udr_i_dn5, locals.var_udr_i_dn6, locals.var_udr_i_dn7, locals.var_udr_i_dn8, locals.var_udr_i_dn9, locals.var_udr_i_dn10, locals.var_udr_i_dn11, locals.var_udr_i_dn12, locals.var_udr_i_dn13, locals.var_udr_i_dn14,)
    }
};
        locals.var_udr_i = assign3610_e4925;
        locals.var_udr_i_dn0 = assign3610_e4925_d_n0;
        locals.var_udr_i_dn2 = assign3610_e4925_d_n2;
        locals.var_udr_i_dn3 = assign3610_e4925_d_n3;
        locals.var_udr_i_dn4 = assign3610_e4925_d_n4;
        locals.var_udr_i_dn5 = assign3610_e4925_d_n5;
        locals.var_udr_i_dn6 = assign3610_e4925_d_n6;
        locals.var_udr_i_dn7 = assign3610_e4925_d_n7;
        locals.var_udr_i_dn8 = assign3610_e4925_d_n8;
        locals.var_udr_i_dn9 = assign3610_e4925_d_n9;
        locals.var_udr_i_dn10 = assign3610_e4925_d_n10;
        locals.var_udr_i_dn11 = assign3610_e4925_d_n11;
        locals.var_udr_i_dn12 = assign3610_e4925_d_n12;
        locals.var_udr_i_dn13 = assign3610_e4925_d_n13;
        locals.var_udr_i_dn14 = assign3610_e4925_d_n14;

        let assign3620_e4929: f64 = (locals.var_inv_l).powf(p.p328);
        let assign3620_e4932: f64 = (locals.var_inv_llong).powf(p.p328);
        let assign3620_e4933: f64 = (assign3620_e4929 - assign3620_e4932);
        let assign3620_e4935: f64 = (assign3620_e4933).max(0.0);
        let assign3620_e4936: f64 = (p.p327 * assign3620_e4935);
        locals.var_t0 = assign3620_e4936;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign3630_e4940: f64 = (locals.var_inv_w).powf(p.p330);
        let assign3630_e4943: f64 = (locals.var_inv_wwide).powf(p.p330);
        let assign3630_e4944: f64 = (assign3630_e4940 - assign3630_e4943);
        let assign3630_e4946: f64 = (assign3630_e4944).max(0.0);
        let assign3630_e4947: f64 = (p.p329 * assign3630_e4946);
        let assign3630_e4951: f64 = (locals.var_inv_wl).powf(p.p332);
        let assign3630_e4952: f64 = (p.p331 * assign3630_e4951);
        let assign3630_e4953: f64 = (assign3630_e4947 + assign3630_e4952);
        locals.var_t1 = assign3630_e4953;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign3640_e4957: f64 = (1.0 + locals.var_t0);
        let assign3640_e4959: f64 = (assign3640_e4957 + locals.var_t1);
        let assign3640_e4960: f64 = (locals.var_uc_i * assign3640_e4959);
        locals.var_uc_i = assign3640_e4960;
        locals.var_uc_i_dn0 = ((locals.var_uc_i_dn0 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_uc_i_dn2 = ((locals.var_uc_i_dn2 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_uc_i_dn3 = ((locals.var_uc_i_dn3 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_uc_i_dn4 = ((locals.var_uc_i_dn4 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_uc_i_dn5 = ((locals.var_uc_i_dn5 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_uc_i_dn6 = ((locals.var_uc_i_dn6 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_uc_i_dn7 = ((locals.var_uc_i_dn7 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_uc_i_dn8 = ((locals.var_uc_i_dn8 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_uc_i_dn9 = ((locals.var_uc_i_dn9 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_uc_i_dn10 = ((locals.var_uc_i_dn10 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_uc_i_dn11 = ((locals.var_uc_i_dn11 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_uc_i_dn12 = ((locals.var_uc_i_dn12 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_uc_i_dn13 = ((locals.var_uc_i_dn13 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_uc_i_dn14 = ((locals.var_uc_i_dn14 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));

        let assign3650_e4963: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign3650_e4963;

        let (assign3660_e4973, assign3660_e4973_d_n0, assign3660_e4973_d_n2, assign3660_e4973_d_n3, assign3660_e4973_d_n4, assign3660_e4973_d_n5, assign3660_e4973_d_n6, assign3660_e4973_d_n7, assign3660_e4973_d_n8, assign3660_e4973_d_n9, assign3660_e4973_d_n10, assign3660_e4973_d_n11, assign3660_e4973_d_n12, assign3660_e4973_d_n13, assign3660_e4973_d_n14,) = {
    if (locals.var_guard30 != 0.0) {
        let assign3660_e4968: f64 = (1.0 + locals.var_t0);
        let assign3660_e4970: f64 = (assign3660_e4968 + locals.var_t1);
        let assign3660_e4971: f64 = (locals.var_ucr_i * assign3660_e4970);
        (assign3660_e4971, ((locals.var_ucr_i_dn0 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_ucr_i_dn2 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_ucr_i_dn3 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_ucr_i_dn4 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_ucr_i_dn5 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_ucr_i_dn6 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_ucr_i_dn7 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_ucr_i_dn8 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_ucr_i_dn9 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_ucr_i_dn10 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_ucr_i_dn11 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_ucr_i_dn12 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_ucr_i_dn13 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_ucr_i_dn14 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn14 + locals.var_t1_dn14))),)
    } else {
        (locals.var_ucr_i, locals.var_ucr_i_dn0, locals.var_ucr_i_dn2, locals.var_ucr_i_dn3, locals.var_ucr_i_dn4, locals.var_ucr_i_dn5, locals.var_ucr_i_dn6, locals.var_ucr_i_dn7, locals.var_ucr_i_dn8, locals.var_ucr_i_dn9, locals.var_ucr_i_dn10, locals.var_ucr_i_dn11, locals.var_ucr_i_dn12, locals.var_ucr_i_dn13, locals.var_ucr_i_dn14,)
    }
};
        locals.var_ucr_i = assign3660_e4973;
        locals.var_ucr_i_dn0 = assign3660_e4973_d_n0;
        locals.var_ucr_i_dn2 = assign3660_e4973_d_n2;
        locals.var_ucr_i_dn3 = assign3660_e4973_d_n3;
        locals.var_ucr_i_dn4 = assign3660_e4973_d_n4;
        locals.var_ucr_i_dn5 = assign3660_e4973_d_n5;
        locals.var_ucr_i_dn6 = assign3660_e4973_d_n6;
        locals.var_ucr_i_dn7 = assign3660_e4973_d_n7;
        locals.var_ucr_i_dn8 = assign3660_e4973_d_n8;
        locals.var_ucr_i_dn9 = assign3660_e4973_d_n9;
        locals.var_ucr_i_dn10 = assign3660_e4973_d_n10;
        locals.var_ucr_i_dn11 = assign3660_e4973_d_n11;
        locals.var_ucr_i_dn12 = assign3660_e4973_d_n12;
        locals.var_ucr_i_dn13 = assign3660_e4973_d_n13;
        locals.var_ucr_i_dn14 = assign3660_e4973_d_n14;

        let assign3670_e4976: f64 = (locals.var_inv_l).powf(p.p179);
        let assign3670_e4979: f64 = (locals.var_inv_llong).powf(p.p179);
        let assign3670_e4980: f64 = (assign3670_e4976 - assign3670_e4979);
        let assign3670_e4982: f64 = (assign3670_e4980).max(0.0);
        locals.var_t0 = assign3670_e4982;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign3680_e4985: f64 = (locals.var_eta0_i * locals.var_t0);
        locals.var_eta0_i = assign3680_e4985;
        locals.var_eta0_i_dn0 = ((locals.var_eta0_i_dn0 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn0));
        locals.var_eta0_i_dn2 = ((locals.var_eta0_i_dn2 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn2));
        locals.var_eta0_i_dn3 = ((locals.var_eta0_i_dn3 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn3));
        locals.var_eta0_i_dn4 = ((locals.var_eta0_i_dn4 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn4));
        locals.var_eta0_i_dn5 = ((locals.var_eta0_i_dn5 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn5));
        locals.var_eta0_i_dn6 = ((locals.var_eta0_i_dn6 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn6));
        locals.var_eta0_i_dn7 = ((locals.var_eta0_i_dn7 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn7));
        locals.var_eta0_i_dn8 = ((locals.var_eta0_i_dn8 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn8));
        locals.var_eta0_i_dn9 = ((locals.var_eta0_i_dn9 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn9));
        locals.var_eta0_i_dn10 = ((locals.var_eta0_i_dn10 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn10));
        locals.var_eta0_i_dn11 = ((locals.var_eta0_i_dn11 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn11));
        locals.var_eta0_i_dn12 = ((locals.var_eta0_i_dn12 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn12));
        locals.var_eta0_i_dn13 = ((locals.var_eta0_i_dn13 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn13));
        locals.var_eta0_i_dn14 = ((locals.var_eta0_i_dn14 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn14));

        let assign3690_e4988: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign3690_e4988;

        let (assign3700_e4994, assign3700_e4994_d_n0, assign3700_e4994_d_n2, assign3700_e4994_d_n3, assign3700_e4994_d_n4, assign3700_e4994_d_n5, assign3700_e4994_d_n6, assign3700_e4994_d_n7, assign3700_e4994_d_n8, assign3700_e4994_d_n9, assign3700_e4994_d_n10, assign3700_e4994_d_n11, assign3700_e4994_d_n12, assign3700_e4994_d_n13, assign3700_e4994_d_n14,) = {
    if (locals.var_guard31 != 0.0) {
        let assign3700_e4992: f64 = (locals.var_eta0r_i * locals.var_t0);
        (assign3700_e4992, ((locals.var_eta0r_i_dn0 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn0)), ((locals.var_eta0r_i_dn2 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn2)), ((locals.var_eta0r_i_dn3 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn3)), ((locals.var_eta0r_i_dn4 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn4)), ((locals.var_eta0r_i_dn5 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn5)), ((locals.var_eta0r_i_dn6 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn6)), ((locals.var_eta0r_i_dn7 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn7)), ((locals.var_eta0r_i_dn8 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn8)), ((locals.var_eta0r_i_dn9 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn9)), ((locals.var_eta0r_i_dn10 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn10)), ((locals.var_eta0r_i_dn11 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn11)), ((locals.var_eta0r_i_dn12 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn12)), ((locals.var_eta0r_i_dn13 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn13)), ((locals.var_eta0r_i_dn14 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn14)),)
    } else {
        (locals.var_eta0r_i, locals.var_eta0r_i_dn0, locals.var_eta0r_i_dn2, locals.var_eta0r_i_dn3, locals.var_eta0r_i_dn4, locals.var_eta0r_i_dn5, locals.var_eta0r_i_dn6, locals.var_eta0r_i_dn7, locals.var_eta0r_i_dn8, locals.var_eta0r_i_dn9, locals.var_eta0r_i_dn10, locals.var_eta0r_i_dn11, locals.var_eta0r_i_dn12, locals.var_eta0r_i_dn13, locals.var_eta0r_i_dn14,)
    }
};
        locals.var_eta0r_i = assign3700_e4994;
        locals.var_eta0r_i_dn0 = assign3700_e4994_d_n0;
        locals.var_eta0r_i_dn2 = assign3700_e4994_d_n2;
        locals.var_eta0r_i_dn3 = assign3700_e4994_d_n3;
        locals.var_eta0r_i_dn4 = assign3700_e4994_d_n4;
        locals.var_eta0r_i_dn5 = assign3700_e4994_d_n5;
        locals.var_eta0r_i_dn6 = assign3700_e4994_d_n6;
        locals.var_eta0r_i_dn7 = assign3700_e4994_d_n7;
        locals.var_eta0r_i_dn8 = assign3700_e4994_d_n8;
        locals.var_eta0r_i_dn9 = assign3700_e4994_d_n9;
        locals.var_eta0r_i_dn10 = assign3700_e4994_d_n10;
        locals.var_eta0r_i_dn11 = assign3700_e4994_d_n11;
        locals.var_eta0r_i_dn12 = assign3700_e4994_d_n12;
        locals.var_eta0r_i_dn13 = assign3700_e4994_d_n13;
        locals.var_eta0r_i_dn14 = assign3700_e4994_d_n14;

        let assign3710_e4998: f64 = (locals.var_inv_l).powf(p.p181);
        let assign3710_e5001: f64 = (locals.var_inv_llong).powf(p.p181);
        let assign3710_e5002: f64 = (assign3710_e4998 - assign3710_e5001);
        let assign3710_e5004: f64 = (assign3710_e5002).max(0.0);
        let assign3710_e5005: f64 = (locals.var_etab_i * assign3710_e5004);
        locals.var_etab_i = assign3710_e5005;

        let assign3720_e5010: f64 = (locals.var_inv_l).powf(p.p462);
        let assign3720_e5013: f64 = (locals.var_inv_llong).powf(p.p462);
        let assign3720_e5014: f64 = (assign3720_e5010 - assign3720_e5013);
        let assign3720_e5016: f64 = (assign3720_e5014).max(0.0);
        let assign3720_e5017: f64 = (p.p461 * assign3720_e5016);
        let assign3720_e5018: f64 = (1.0 + assign3720_e5017);
        locals.var_t0 = assign3720_e5018;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign3730_e5021: f64 = (locals.var_pdiblc_i * locals.var_t0);
        locals.var_pdiblc_i = assign3730_e5021;
        locals.var_pdiblc_i_dn0 = ((locals.var_pdiblc_i_dn0 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn0));
        locals.var_pdiblc_i_dn2 = ((locals.var_pdiblc_i_dn2 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn2));
        locals.var_pdiblc_i_dn3 = ((locals.var_pdiblc_i_dn3 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn3));
        locals.var_pdiblc_i_dn4 = ((locals.var_pdiblc_i_dn4 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn4));
        locals.var_pdiblc_i_dn5 = ((locals.var_pdiblc_i_dn5 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn5));
        locals.var_pdiblc_i_dn6 = ((locals.var_pdiblc_i_dn6 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn6));
        locals.var_pdiblc_i_dn7 = ((locals.var_pdiblc_i_dn7 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn7));
        locals.var_pdiblc_i_dn8 = ((locals.var_pdiblc_i_dn8 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn8));
        locals.var_pdiblc_i_dn9 = ((locals.var_pdiblc_i_dn9 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn9));
        locals.var_pdiblc_i_dn10 = ((locals.var_pdiblc_i_dn10 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn10));
        locals.var_pdiblc_i_dn11 = ((locals.var_pdiblc_i_dn11 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn11));
        locals.var_pdiblc_i_dn12 = ((locals.var_pdiblc_i_dn12 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn12));
        locals.var_pdiblc_i_dn13 = ((locals.var_pdiblc_i_dn13 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn13));
        locals.var_pdiblc_i_dn14 = ((locals.var_pdiblc_i_dn14 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn14));

        let assign3740_e5024: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign3740_e5024;

        let (assign3750_e5030, assign3750_e5030_d_n0, assign3750_e5030_d_n2, assign3750_e5030_d_n3, assign3750_e5030_d_n4, assign3750_e5030_d_n5, assign3750_e5030_d_n6, assign3750_e5030_d_n7, assign3750_e5030_d_n8, assign3750_e5030_d_n9, assign3750_e5030_d_n10, assign3750_e5030_d_n11, assign3750_e5030_d_n12, assign3750_e5030_d_n13, assign3750_e5030_d_n14,) = {
    if (locals.var_guard32 != 0.0) {
        let assign3750_e5028: f64 = (locals.var_pdiblcr_i * locals.var_t0);
        (assign3750_e5028, ((locals.var_pdiblcr_i_dn0 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn0)), ((locals.var_pdiblcr_i_dn2 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn2)), ((locals.var_pdiblcr_i_dn3 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn3)), ((locals.var_pdiblcr_i_dn4 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn4)), ((locals.var_pdiblcr_i_dn5 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn5)), ((locals.var_pdiblcr_i_dn6 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn6)), ((locals.var_pdiblcr_i_dn7 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn7)), ((locals.var_pdiblcr_i_dn8 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn8)), ((locals.var_pdiblcr_i_dn9 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn9)), ((locals.var_pdiblcr_i_dn10 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn10)), ((locals.var_pdiblcr_i_dn11 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn11)), ((locals.var_pdiblcr_i_dn12 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn12)), ((locals.var_pdiblcr_i_dn13 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn13)), ((locals.var_pdiblcr_i_dn14 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn14)),)
    } else {
        (locals.var_pdiblcr_i, locals.var_pdiblcr_i_dn0, locals.var_pdiblcr_i_dn2, locals.var_pdiblcr_i_dn3, locals.var_pdiblcr_i_dn4, locals.var_pdiblcr_i_dn5, locals.var_pdiblcr_i_dn6, locals.var_pdiblcr_i_dn7, locals.var_pdiblcr_i_dn8, locals.var_pdiblcr_i_dn9, locals.var_pdiblcr_i_dn10, locals.var_pdiblcr_i_dn11, locals.var_pdiblcr_i_dn12, locals.var_pdiblcr_i_dn13, locals.var_pdiblcr_i_dn14,)
    }
};
        locals.var_pdiblcr_i = assign3750_e5030;
        locals.var_pdiblcr_i_dn0 = assign3750_e5030_d_n0;
        locals.var_pdiblcr_i_dn2 = assign3750_e5030_d_n2;
        locals.var_pdiblcr_i_dn3 = assign3750_e5030_d_n3;
        locals.var_pdiblcr_i_dn4 = assign3750_e5030_d_n4;
        locals.var_pdiblcr_i_dn5 = assign3750_e5030_d_n5;
        locals.var_pdiblcr_i_dn6 = assign3750_e5030_d_n6;
        locals.var_pdiblcr_i_dn7 = assign3750_e5030_d_n7;
        locals.var_pdiblcr_i_dn8 = assign3750_e5030_d_n8;
        locals.var_pdiblcr_i_dn9 = assign3750_e5030_d_n9;
        locals.var_pdiblcr_i_dn10 = assign3750_e5030_d_n10;
        locals.var_pdiblcr_i_dn11 = assign3750_e5030_d_n11;
        locals.var_pdiblcr_i_dn12 = assign3750_e5030_d_n12;
        locals.var_pdiblcr_i_dn13 = assign3750_e5030_d_n13;
        locals.var_pdiblcr_i_dn14 = assign3750_e5030_d_n14;

        let assign3760_e5036: f64 = (locals.var_inv_l).powf(p.p258);
        let assign3760_e5039: f64 = (locals.var_inv_llong).powf(p.p258);
        let assign3760_e5040: f64 = (assign3760_e5036 - assign3760_e5039);
        let assign3760_e5042: f64 = (assign3760_e5040).max(0.0);
        let assign3760_e5043: f64 = (p.p257 * assign3760_e5042);
        let assign3760_e5044: f64 = (1.0 + assign3760_e5043);
        let assign3760_e5045: f64 = (locals.var_delta_i * assign3760_e5044);
        locals.var_t0 = assign3760_e5045;
        locals.var_t0_dn0 = (locals.var_delta_i_dn0 * assign3760_e5044);
        locals.var_t0_dn2 = (locals.var_delta_i_dn2 * assign3760_e5044);
        locals.var_t0_dn3 = (locals.var_delta_i_dn3 * assign3760_e5044);
        locals.var_t0_dn4 = (locals.var_delta_i_dn4 * assign3760_e5044);
        locals.var_t0_dn5 = (locals.var_delta_i_dn5 * assign3760_e5044);
        locals.var_t0_dn6 = (locals.var_delta_i_dn6 * assign3760_e5044);
        locals.var_t0_dn7 = (locals.var_delta_i_dn7 * assign3760_e5044);
        locals.var_t0_dn8 = (locals.var_delta_i_dn8 * assign3760_e5044);
        locals.var_t0_dn9 = (locals.var_delta_i_dn9 * assign3760_e5044);
        locals.var_t0_dn10 = (locals.var_delta_i_dn10 * assign3760_e5044);
        locals.var_t0_dn11 = (locals.var_delta_i_dn11 * assign3760_e5044);
        locals.var_t0_dn12 = (locals.var_delta_i_dn12 * assign3760_e5044);
        locals.var_t0_dn13 = (locals.var_delta_i_dn13 * assign3760_e5044);
        locals.var_t0_dn14 = (locals.var_delta_i_dn14 * assign3760_e5044);

        let assign3770_e5048: f64 = (locals.var_t0).min(0.5);
        locals.var_delta_i = assign3770_e5048;
        locals.var_delta_i_dn0 = if locals.var_t0 <= 0.5 { locals.var_t0_dn0 } else { 0.0 };
        locals.var_delta_i_dn2 = if locals.var_t0 <= 0.5 { locals.var_t0_dn2 } else { 0.0 };
        locals.var_delta_i_dn3 = if locals.var_t0 <= 0.5 { locals.var_t0_dn3 } else { 0.0 };
        locals.var_delta_i_dn4 = if locals.var_t0 <= 0.5 { locals.var_t0_dn4 } else { 0.0 };
        locals.var_delta_i_dn5 = if locals.var_t0 <= 0.5 { locals.var_t0_dn5 } else { 0.0 };
        locals.var_delta_i_dn6 = if locals.var_t0 <= 0.5 { locals.var_t0_dn6 } else { 0.0 };
        locals.var_delta_i_dn7 = if locals.var_t0 <= 0.5 { locals.var_t0_dn7 } else { 0.0 };
        locals.var_delta_i_dn8 = if locals.var_t0 <= 0.5 { locals.var_t0_dn8 } else { 0.0 };
        locals.var_delta_i_dn9 = if locals.var_t0 <= 0.5 { locals.var_t0_dn9 } else { 0.0 };
        locals.var_delta_i_dn10 = if locals.var_t0 <= 0.5 { locals.var_t0_dn10 } else { 0.0 };
        locals.var_delta_i_dn11 = if locals.var_t0 <= 0.5 { locals.var_t0_dn11 } else { 0.0 };
        locals.var_delta_i_dn12 = if locals.var_t0 <= 0.5 { locals.var_t0_dn12 } else { 0.0 };
        locals.var_delta_i_dn13 = if locals.var_t0 <= 0.5 { locals.var_t0_dn13 } else { 0.0 };
        locals.var_delta_i_dn14 = if locals.var_t0 <= 0.5 { locals.var_t0_dn14 } else { 0.0 };

        let assign3780_e5054: f64 = (locals.var_inv_l).powf(p.p480);
        let assign3780_e5057: f64 = (locals.var_inv_llong).powf(p.p480);
        let assign3780_e5058: f64 = (assign3780_e5054 - assign3780_e5057);
        let assign3780_e5060: f64 = (assign3780_e5058).max(0.0);
        let assign3780_e5061: f64 = (p.p479 * assign3780_e5060);
        let assign3780_e5062: f64 = (1.0 + assign3780_e5061);
        let assign3780_e5063: f64 = (locals.var_fprout_i * assign3780_e5062);
        locals.var_fprout_i = assign3780_e5063;

        let assign3790_e5068: f64 = (locals.var_inv_l).powf(p.p342);
        let assign3790_e5071: f64 = (locals.var_inv_llong).powf(p.p342);
        let assign3790_e5072: f64 = (assign3790_e5068 - assign3790_e5071);
        let assign3790_e5074: f64 = (assign3790_e5072).max(0.0);
        let assign3790_e5075: f64 = (p.p341 * assign3790_e5074);
        let assign3790_e5076: f64 = (1.0 + assign3790_e5075);
        locals.var_t0 = assign3790_e5076;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign3800_e5079: f64 = (locals.var_pclm_i * locals.var_t0);
        locals.var_pclm_i = assign3800_e5079;
        locals.var_pclm_i_dn0 = ((locals.var_pclm_i_dn0 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn0));
        locals.var_pclm_i_dn2 = ((locals.var_pclm_i_dn2 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn2));
        locals.var_pclm_i_dn3 = ((locals.var_pclm_i_dn3 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn3));
        locals.var_pclm_i_dn4 = ((locals.var_pclm_i_dn4 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn4));
        locals.var_pclm_i_dn5 = ((locals.var_pclm_i_dn5 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn5));
        locals.var_pclm_i_dn6 = ((locals.var_pclm_i_dn6 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn6));
        locals.var_pclm_i_dn7 = ((locals.var_pclm_i_dn7 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn7));
        locals.var_pclm_i_dn8 = ((locals.var_pclm_i_dn8 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn8));
        locals.var_pclm_i_dn9 = ((locals.var_pclm_i_dn9 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn9));
        locals.var_pclm_i_dn10 = ((locals.var_pclm_i_dn10 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn10));
        locals.var_pclm_i_dn11 = ((locals.var_pclm_i_dn11 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn11));
        locals.var_pclm_i_dn12 = ((locals.var_pclm_i_dn12 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn12));
        locals.var_pclm_i_dn13 = ((locals.var_pclm_i_dn13 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn13));
        locals.var_pclm_i_dn14 = ((locals.var_pclm_i_dn14 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn14));

        let assign3810_e5082: f64 = (locals.var_pclm_i).max(0.0);
        locals.var_pclm_i = assign3810_e5082;
        locals.var_pclm_i_dn0 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn0 } else { 0.0 };
        locals.var_pclm_i_dn2 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn2 } else { 0.0 };
        locals.var_pclm_i_dn3 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn3 } else { 0.0 };
        locals.var_pclm_i_dn4 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn4 } else { 0.0 };
        locals.var_pclm_i_dn5 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn5 } else { 0.0 };
        locals.var_pclm_i_dn6 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn6 } else { 0.0 };
        locals.var_pclm_i_dn7 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn7 } else { 0.0 };
        locals.var_pclm_i_dn8 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn8 } else { 0.0 };
        locals.var_pclm_i_dn9 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn9 } else { 0.0 };
        locals.var_pclm_i_dn10 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn10 } else { 0.0 };
        locals.var_pclm_i_dn11 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn11 } else { 0.0 };
        locals.var_pclm_i_dn12 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn12 } else { 0.0 };
        locals.var_pclm_i_dn13 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn13 } else { 0.0 };
        locals.var_pclm_i_dn14 = if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn14 } else { 0.0 };

        let assign3820_e5085: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign3820_e5085;

        let (assign3830_e5091, assign3830_e5091_d_n0, assign3830_e5091_d_n2, assign3830_e5091_d_n3, assign3830_e5091_d_n4, assign3830_e5091_d_n5, assign3830_e5091_d_n6, assign3830_e5091_d_n7, assign3830_e5091_d_n8, assign3830_e5091_d_n9, assign3830_e5091_d_n10, assign3830_e5091_d_n11, assign3830_e5091_d_n12, assign3830_e5091_d_n13, assign3830_e5091_d_n14,) = {
    if (locals.var_guard33 != 0.0) {
        let assign3830_e5089: f64 = (locals.var_pclmr_i * locals.var_t0);
        (assign3830_e5089, ((locals.var_pclmr_i_dn0 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn0)), ((locals.var_pclmr_i_dn2 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn2)), ((locals.var_pclmr_i_dn3 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn3)), ((locals.var_pclmr_i_dn4 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn4)), ((locals.var_pclmr_i_dn5 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn5)), ((locals.var_pclmr_i_dn6 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn6)), ((locals.var_pclmr_i_dn7 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn7)), ((locals.var_pclmr_i_dn8 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn8)), ((locals.var_pclmr_i_dn9 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn9)), ((locals.var_pclmr_i_dn10 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn10)), ((locals.var_pclmr_i_dn11 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn11)), ((locals.var_pclmr_i_dn12 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn12)), ((locals.var_pclmr_i_dn13 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn13)), ((locals.var_pclmr_i_dn14 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn14)),)
    } else {
        (locals.var_pclmr_i, locals.var_pclmr_i_dn0, locals.var_pclmr_i_dn2, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11, locals.var_pclmr_i_dn12, locals.var_pclmr_i_dn13, locals.var_pclmr_i_dn14,)
    }
};
        locals.var_pclmr_i = assign3830_e5091;
        locals.var_pclmr_i_dn0 = assign3830_e5091_d_n0;
        locals.var_pclmr_i_dn2 = assign3830_e5091_d_n2;
        locals.var_pclmr_i_dn3 = assign3830_e5091_d_n3;
        locals.var_pclmr_i_dn4 = assign3830_e5091_d_n4;
        locals.var_pclmr_i_dn5 = assign3830_e5091_d_n5;
        locals.var_pclmr_i_dn6 = assign3830_e5091_d_n6;
        locals.var_pclmr_i_dn7 = assign3830_e5091_d_n7;
        locals.var_pclmr_i_dn8 = assign3830_e5091_d_n8;
        locals.var_pclmr_i_dn9 = assign3830_e5091_d_n9;
        locals.var_pclmr_i_dn10 = assign3830_e5091_d_n10;
        locals.var_pclmr_i_dn11 = assign3830_e5091_d_n11;
        locals.var_pclmr_i_dn12 = assign3830_e5091_d_n12;
        locals.var_pclmr_i_dn13 = assign3830_e5091_d_n13;
        locals.var_pclmr_i_dn14 = assign3830_e5091_d_n14;

        let (assign3840_e5097, assign3840_e5097_d_n0, assign3840_e5097_d_n2, assign3840_e5097_d_n3, assign3840_e5097_d_n4, assign3840_e5097_d_n5, assign3840_e5097_d_n6, assign3840_e5097_d_n7, assign3840_e5097_d_n8, assign3840_e5097_d_n9, assign3840_e5097_d_n10, assign3840_e5097_d_n11, assign3840_e5097_d_n12, assign3840_e5097_d_n13, assign3840_e5097_d_n14,) = {
    if (locals.var_guard33 != 0.0) {
        let assign3840_e5095: f64 = (locals.var_pclmr_i).max(0.0);
        (assign3840_e5095, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn0 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn2 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn3 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn4 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn5 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn6 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn7 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn8 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn9 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn10 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn11 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn12 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn13 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn14 } else { 0.0 },)
    } else {
        (locals.var_pclmr_i, locals.var_pclmr_i_dn0, locals.var_pclmr_i_dn2, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11, locals.var_pclmr_i_dn12, locals.var_pclmr_i_dn13, locals.var_pclmr_i_dn14,)
    }
};
        locals.var_pclmr_i = assign3840_e5097;
        locals.var_pclmr_i_dn0 = assign3840_e5097_d_n0;
        locals.var_pclmr_i_dn2 = assign3840_e5097_d_n2;
        locals.var_pclmr_i_dn3 = assign3840_e5097_d_n3;
        locals.var_pclmr_i_dn4 = assign3840_e5097_d_n4;
        locals.var_pclmr_i_dn5 = assign3840_e5097_d_n5;
        locals.var_pclmr_i_dn6 = assign3840_e5097_d_n6;
        locals.var_pclmr_i_dn7 = assign3840_e5097_d_n7;
        locals.var_pclmr_i_dn8 = assign3840_e5097_d_n8;
        locals.var_pclmr_i_dn9 = assign3840_e5097_d_n9;
        locals.var_pclmr_i_dn10 = assign3840_e5097_d_n10;
        locals.var_pclmr_i_dn11 = assign3840_e5097_d_n11;
        locals.var_pclmr_i_dn12 = assign3840_e5097_d_n12;
        locals.var_pclmr_i_dn13 = assign3840_e5097_d_n13;
        locals.var_pclmr_i_dn14 = assign3840_e5097_d_n14;

        let assign3850_e5101: f64 = (locals.var_inv_l).powf(p.p244);
        let assign3850_e5104: f64 = (locals.var_inv_llong).powf(p.p244);
        let assign3850_e5105: f64 = (assign3850_e5101 - assign3850_e5104);
        let assign3850_e5107: f64 = (assign3850_e5105).max(0.0);
        let assign3850_e5108: f64 = (p.p243 * assign3850_e5107);
        locals.var_t0 = assign3850_e5108;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign3860_e5112: f64 = (locals.var_inv_w).powf(p.p246);
        let assign3860_e5115: f64 = (locals.var_inv_wwide).powf(p.p246);
        let assign3860_e5116: f64 = (assign3860_e5112 - assign3860_e5115);
        let assign3860_e5118: f64 = (assign3860_e5116).max(0.0);
        let assign3860_e5119: f64 = (p.p245 * assign3860_e5118);
        let assign3860_e5123: f64 = (locals.var_inv_wl).powf(p.p248);
        let assign3860_e5124: f64 = (p.p247 * assign3860_e5123);
        let assign3860_e5125: f64 = (assign3860_e5119 + assign3860_e5124);
        locals.var_t1 = assign3860_e5125;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign3870_e5129: f64 = (1.0 + locals.var_t0);
        let assign3870_e5131: f64 = (assign3870_e5129 + locals.var_t1);
        let assign3870_e5132: f64 = (locals.var_vsat_i * assign3870_e5131);
        locals.var_vsat_i = assign3870_e5132;
        locals.var_vsat_i_dn0 = ((locals.var_vsat_i_dn0 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_vsat_i_dn2 = ((locals.var_vsat_i_dn2 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_vsat_i_dn3 = ((locals.var_vsat_i_dn3 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vsat_i_dn4 = ((locals.var_vsat_i_dn4 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vsat_i_dn5 = ((locals.var_vsat_i_dn5 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vsat_i_dn6 = ((locals.var_vsat_i_dn6 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vsat_i_dn7 = ((locals.var_vsat_i_dn7 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vsat_i_dn8 = ((locals.var_vsat_i_dn8 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vsat_i_dn9 = ((locals.var_vsat_i_dn9 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vsat_i_dn10 = ((locals.var_vsat_i_dn10 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vsat_i_dn11 = ((locals.var_vsat_i_dn11 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vsat_i_dn12 = ((locals.var_vsat_i_dn12 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_vsat_i_dn13 = ((locals.var_vsat_i_dn13 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_vsat_i_dn14 = ((locals.var_vsat_i_dn14 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));

        let assign3880_e5135: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign3880_e5135;

    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3890_e5145, assign3890_e5145_d_n0, assign3890_e5145_d_n2, assign3890_e5145_d_n3, assign3890_e5145_d_n4, assign3890_e5145_d_n5, assign3890_e5145_d_n6, assign3890_e5145_d_n7, assign3890_e5145_d_n8, assign3890_e5145_d_n9, assign3890_e5145_d_n10, assign3890_e5145_d_n11, assign3890_e5145_d_n12, assign3890_e5145_d_n13, assign3890_e5145_d_n14,) = {
    if (locals.var_guard34 != 0.0) {
        let assign3890_e5140: f64 = (1.0 + locals.var_t0);
        let assign3890_e5142: f64 = (assign3890_e5140 + locals.var_t1);
        let assign3890_e5143: f64 = (locals.var_vsatr_i * assign3890_e5142);
        (assign3890_e5143, ((locals.var_vsatr_i_dn0 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_vsatr_i_dn2 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_vsatr_i_dn3 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_vsatr_i_dn4 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_vsatr_i_dn5 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_vsatr_i_dn6 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_vsatr_i_dn7 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_vsatr_i_dn8 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_vsatr_i_dn9 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_vsatr_i_dn10 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_vsatr_i_dn11 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_vsatr_i_dn12 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_vsatr_i_dn13 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_vsatr_i_dn14 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn14 + locals.var_t1_dn14))),)
    } else {
        (locals.var_vsatr_i, locals.var_vsatr_i_dn0, locals.var_vsatr_i_dn2, locals.var_vsatr_i_dn3, locals.var_vsatr_i_dn4, locals.var_vsatr_i_dn5, locals.var_vsatr_i_dn6, locals.var_vsatr_i_dn7, locals.var_vsatr_i_dn8, locals.var_vsatr_i_dn9, locals.var_vsatr_i_dn10, locals.var_vsatr_i_dn11, locals.var_vsatr_i_dn12, locals.var_vsatr_i_dn13, locals.var_vsatr_i_dn14,)
    }
};
        locals.var_vsatr_i = assign3890_e5145;
        locals.var_vsatr_i_dn0 = assign3890_e5145_d_n0;
        locals.var_vsatr_i_dn2 = assign3890_e5145_d_n2;
        locals.var_vsatr_i_dn3 = assign3890_e5145_d_n3;
        locals.var_vsatr_i_dn4 = assign3890_e5145_d_n4;
        locals.var_vsatr_i_dn5 = assign3890_e5145_d_n5;
        locals.var_vsatr_i_dn6 = assign3890_e5145_d_n6;
        locals.var_vsatr_i_dn7 = assign3890_e5145_d_n7;
        locals.var_vsatr_i_dn8 = assign3890_e5145_d_n8;
        locals.var_vsatr_i_dn9 = assign3890_e5145_d_n9;
        locals.var_vsatr_i_dn10 = assign3890_e5145_d_n10;
        locals.var_vsatr_i_dn11 = assign3890_e5145_d_n11;
        locals.var_vsatr_i_dn12 = assign3890_e5145_d_n12;
        locals.var_vsatr_i_dn13 = assign3890_e5145_d_n13;
        locals.var_vsatr_i_dn14 = assign3890_e5145_d_n14;

        let assign3900_e5151: f64 = (locals.var_inv_l).powf(p.p424);
        let assign3900_e5154: f64 = (locals.var_inv_llong).powf(p.p424);
        let assign3900_e5155: f64 = (assign3900_e5151 - assign3900_e5154);
        let assign3900_e5157: f64 = (assign3900_e5155).max(0.0);
        let assign3900_e5158: f64 = (p.p423 * assign3900_e5157);
        let assign3900_e5159: f64 = (1.0 + assign3900_e5158);
        let assign3900_e5160: f64 = (locals.var_psat_i * assign3900_e5159);
        let assign3900_e5162: f64 = (assign3900_e5160).max(0.25);
        locals.var_psat_i = assign3900_e5162;

        let assign3910_e5165: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign3910_e5165;

        let (assign3920_e5185,) = {
    if (locals.var_guard35 != 0.0) {
        let assign3920_e5172: f64 = (locals.var_inv_l).powf(p.p424);
        let assign3920_e5175: f64 = (locals.var_inv_llong).powf(p.p424);
        let assign3920_e5176: f64 = (assign3920_e5172 - assign3920_e5175);
        let assign3920_e5178: f64 = (assign3920_e5176).max(0.0);
        let assign3920_e5179: f64 = (p.p423 * assign3920_e5178);
        let assign3920_e5180: f64 = (1.0 + assign3920_e5179);
        let assign3920_e5181: f64 = (locals.var_psatr_i * assign3920_e5180);
        let assign3920_e5183: f64 = (assign3920_e5181).max(0.25);
        (assign3920_e5183,)
    } else {
        (locals.var_psatr_i,)
    }
};
        locals.var_psatr_i = assign3920_e5185;

        let assign3930_e5190: f64 = (locals.var_inv_l).powf(p.p439);
        let assign3930_e5193: f64 = (locals.var_inv_llong).powf(p.p439);
        let assign3930_e5194: f64 = (assign3930_e5190 - assign3930_e5193);
        let assign3930_e5196: f64 = (assign3930_e5194).max(0.0);
        let assign3930_e5197: f64 = (p.p438 * assign3930_e5196);
        let assign3930_e5198: f64 = (1.0 + assign3930_e5197);
        locals.var_t0 = assign3930_e5198;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign3940_e5201: f64 = (locals.var_ptwg_i * locals.var_t0);
        locals.var_ptwg_i = assign3940_e5201;
        locals.var_ptwg_i_dn0 = ((locals.var_ptwg_i_dn0 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn0));
        locals.var_ptwg_i_dn2 = ((locals.var_ptwg_i_dn2 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn2));
        locals.var_ptwg_i_dn3 = ((locals.var_ptwg_i_dn3 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn3));
        locals.var_ptwg_i_dn4 = ((locals.var_ptwg_i_dn4 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn4));
        locals.var_ptwg_i_dn5 = ((locals.var_ptwg_i_dn5 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn5));
        locals.var_ptwg_i_dn6 = ((locals.var_ptwg_i_dn6 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn6));
        locals.var_ptwg_i_dn7 = ((locals.var_ptwg_i_dn7 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn7));
        locals.var_ptwg_i_dn8 = ((locals.var_ptwg_i_dn8 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn8));
        locals.var_ptwg_i_dn9 = ((locals.var_ptwg_i_dn9 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn9));
        locals.var_ptwg_i_dn10 = ((locals.var_ptwg_i_dn10 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn10));
        locals.var_ptwg_i_dn11 = ((locals.var_ptwg_i_dn11 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn11));
        locals.var_ptwg_i_dn12 = ((locals.var_ptwg_i_dn12 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn12));
        locals.var_ptwg_i_dn13 = ((locals.var_ptwg_i_dn13 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn13));
        locals.var_ptwg_i_dn14 = ((locals.var_ptwg_i_dn14 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn14));

        let assign3950_e5204: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign3950_e5204;

        let (assign3960_e5210, assign3960_e5210_d_n0, assign3960_e5210_d_n2, assign3960_e5210_d_n3, assign3960_e5210_d_n4, assign3960_e5210_d_n5, assign3960_e5210_d_n6, assign3960_e5210_d_n7, assign3960_e5210_d_n8, assign3960_e5210_d_n9, assign3960_e5210_d_n10, assign3960_e5210_d_n11, assign3960_e5210_d_n12, assign3960_e5210_d_n13, assign3960_e5210_d_n14,) = {
    if (locals.var_guard36 != 0.0) {
        let assign3960_e5208: f64 = (locals.var_ptwgr_i * locals.var_t0);
        (assign3960_e5208, ((locals.var_ptwgr_i_dn0 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn0)), ((locals.var_ptwgr_i_dn2 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn2)), ((locals.var_ptwgr_i_dn3 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn3)), ((locals.var_ptwgr_i_dn4 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn4)), ((locals.var_ptwgr_i_dn5 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn5)), ((locals.var_ptwgr_i_dn6 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn6)), ((locals.var_ptwgr_i_dn7 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn7)), ((locals.var_ptwgr_i_dn8 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn8)), ((locals.var_ptwgr_i_dn9 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn9)), ((locals.var_ptwgr_i_dn10 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn10)), ((locals.var_ptwgr_i_dn11 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn11)), ((locals.var_ptwgr_i_dn12 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn12)), ((locals.var_ptwgr_i_dn13 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn13)), ((locals.var_ptwgr_i_dn14 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn14)),)
    } else {
        (locals.var_ptwgr_i, locals.var_ptwgr_i_dn0, locals.var_ptwgr_i_dn2, locals.var_ptwgr_i_dn3, locals.var_ptwgr_i_dn4, locals.var_ptwgr_i_dn5, locals.var_ptwgr_i_dn6, locals.var_ptwgr_i_dn7, locals.var_ptwgr_i_dn8, locals.var_ptwgr_i_dn9, locals.var_ptwgr_i_dn10, locals.var_ptwgr_i_dn11, locals.var_ptwgr_i_dn12, locals.var_ptwgr_i_dn13, locals.var_ptwgr_i_dn14,)
    }
};
        locals.var_ptwgr_i = assign3960_e5210;
        locals.var_ptwgr_i_dn0 = assign3960_e5210_d_n0;
        locals.var_ptwgr_i_dn2 = assign3960_e5210_d_n2;
        locals.var_ptwgr_i_dn3 = assign3960_e5210_d_n3;
        locals.var_ptwgr_i_dn4 = assign3960_e5210_d_n4;
        locals.var_ptwgr_i_dn5 = assign3960_e5210_d_n5;
        locals.var_ptwgr_i_dn6 = assign3960_e5210_d_n6;
        locals.var_ptwgr_i_dn7 = assign3960_e5210_d_n7;
        locals.var_ptwgr_i_dn8 = assign3960_e5210_d_n8;
        locals.var_ptwgr_i_dn9 = assign3960_e5210_d_n9;
        locals.var_ptwgr_i_dn10 = assign3960_e5210_d_n10;
        locals.var_ptwgr_i_dn11 = assign3960_e5210_d_n11;
        locals.var_ptwgr_i_dn12 = assign3960_e5210_d_n12;
        locals.var_ptwgr_i_dn13 = assign3960_e5210_d_n13;
        locals.var_ptwgr_i_dn14 = assign3960_e5210_d_n14;

        let assign3970_e5214: f64 = (locals.var_inv_l).powf(p.p486);
        let assign3970_e5217: f64 = (locals.var_inv_llong).powf(p.p486);
        let assign3970_e5218: f64 = (assign3970_e5214 - assign3970_e5217);
        let assign3970_e5220: f64 = (assign3970_e5218).max(0.0);
        let assign3970_e5221: f64 = (p.p485 * assign3970_e5220);
        locals.var_t0 = assign3970_e5221;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign3980_e5225: f64 = (locals.var_inv_w).powf(p.p488);
        let assign3980_e5228: f64 = (locals.var_inv_wwide).powf(p.p488);
        let assign3980_e5229: f64 = (assign3980_e5225 - assign3980_e5228);
        let assign3980_e5231: f64 = (assign3980_e5229).max(0.0);
        let assign3980_e5232: f64 = (p.p487 * assign3980_e5231);
        locals.var_t1 = assign3980_e5232;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign3990_e5236: f64 = (1.0 + locals.var_t0);
        let assign3990_e5238: f64 = (assign3990_e5236 + locals.var_t1);
        let assign3990_e5239: f64 = (locals.var_alpha0_i * assign3990_e5238);
        locals.var_alpha0_i = assign3990_e5239;
        locals.var_alpha0_i_dn0 = ((locals.var_alpha0_i_dn0 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_alpha0_i_dn2 = ((locals.var_alpha0_i_dn2 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_alpha0_i_dn3 = ((locals.var_alpha0_i_dn3 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_alpha0_i_dn4 = ((locals.var_alpha0_i_dn4 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_alpha0_i_dn5 = ((locals.var_alpha0_i_dn5 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_alpha0_i_dn6 = ((locals.var_alpha0_i_dn6 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_alpha0_i_dn7 = ((locals.var_alpha0_i_dn7 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_alpha0_i_dn8 = ((locals.var_alpha0_i_dn8 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_alpha0_i_dn9 = ((locals.var_alpha0_i_dn9 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_alpha0_i_dn10 = ((locals.var_alpha0_i_dn10 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_alpha0_i_dn11 = ((locals.var_alpha0_i_dn11 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_alpha0_i_dn12 = ((locals.var_alpha0_i_dn12 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_alpha0_i_dn13 = ((locals.var_alpha0_i_dn13 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_alpha0_i_dn14 = ((locals.var_alpha0_i_dn14 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));

        let assign4000_e5242: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign4000_e5242;

        let (assign4010_e5252, assign4010_e5252_d_n0, assign4010_e5252_d_n2, assign4010_e5252_d_n3, assign4010_e5252_d_n4, assign4010_e5252_d_n5, assign4010_e5252_d_n6, assign4010_e5252_d_n7, assign4010_e5252_d_n8, assign4010_e5252_d_n9, assign4010_e5252_d_n10, assign4010_e5252_d_n11, assign4010_e5252_d_n12, assign4010_e5252_d_n13, assign4010_e5252_d_n14,) = {
    if (locals.var_guard37 != 0.0) {
        let assign4010_e5247: f64 = (1.0 + locals.var_t0);
        let assign4010_e5249: f64 = (assign4010_e5247 + locals.var_t1);
        let assign4010_e5250: f64 = (locals.var_alpha0r_i * assign4010_e5249);
        (assign4010_e5250, ((locals.var_alpha0r_i_dn0 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_alpha0r_i_dn2 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_alpha0r_i_dn3 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_alpha0r_i_dn4 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_alpha0r_i_dn5 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_alpha0r_i_dn6 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_alpha0r_i_dn7 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_alpha0r_i_dn8 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_alpha0r_i_dn9 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_alpha0r_i_dn10 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_alpha0r_i_dn11 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_alpha0r_i_dn12 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_alpha0r_i_dn13 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_alpha0r_i_dn14 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn14 + locals.var_t1_dn14))),)
    } else {
        (locals.var_alpha0r_i, locals.var_alpha0r_i_dn0, locals.var_alpha0r_i_dn2, locals.var_alpha0r_i_dn3, locals.var_alpha0r_i_dn4, locals.var_alpha0r_i_dn5, locals.var_alpha0r_i_dn6, locals.var_alpha0r_i_dn7, locals.var_alpha0r_i_dn8, locals.var_alpha0r_i_dn9, locals.var_alpha0r_i_dn10, locals.var_alpha0r_i_dn11, locals.var_alpha0r_i_dn12, locals.var_alpha0r_i_dn13, locals.var_alpha0r_i_dn14,)
    }
};
        locals.var_alpha0r_i = assign4010_e5252;
        locals.var_alpha0r_i_dn0 = assign4010_e5252_d_n0;
        locals.var_alpha0r_i_dn2 = assign4010_e5252_d_n2;
        locals.var_alpha0r_i_dn3 = assign4010_e5252_d_n3;
        locals.var_alpha0r_i_dn4 = assign4010_e5252_d_n4;
        locals.var_alpha0r_i_dn5 = assign4010_e5252_d_n5;
        locals.var_alpha0r_i_dn6 = assign4010_e5252_d_n6;
        locals.var_alpha0r_i_dn7 = assign4010_e5252_d_n7;
        locals.var_alpha0r_i_dn8 = assign4010_e5252_d_n8;
        locals.var_alpha0r_i_dn9 = assign4010_e5252_d_n9;
        locals.var_alpha0r_i_dn10 = assign4010_e5252_d_n10;
        locals.var_alpha0r_i_dn11 = assign4010_e5252_d_n11;
        locals.var_alpha0r_i_dn12 = assign4010_e5252_d_n12;
        locals.var_alpha0r_i_dn13 = assign4010_e5252_d_n13;
        locals.var_alpha0r_i_dn14 = assign4010_e5252_d_n14;

        let assign4020_e5256: f64 = (locals.var_inv_w).powf(p.p496);
        let assign4020_e5259: f64 = (locals.var_inv_wwide).powf(p.p496);
        let assign4020_e5260: f64 = (assign4020_e5256 - assign4020_e5259);
        let assign4020_e5262: f64 = (assign4020_e5260).max(0.0);
        let assign4020_e5263: f64 = (p.p495 * assign4020_e5262);
        locals.var_t1 = assign4020_e5263;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign4030_e5267: f64 = (1.0 + locals.var_t1);
        let assign4030_e5268: f64 = (locals.var_beta0_i * assign4030_e5267);
        locals.var_beta0_i = assign4030_e5268;
        locals.var_beta0_i_dn0 = ((locals.var_beta0_i_dn0 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn0));
        locals.var_beta0_i_dn2 = ((locals.var_beta0_i_dn2 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn2));
        locals.var_beta0_i_dn3 = ((locals.var_beta0_i_dn3 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn3));
        locals.var_beta0_i_dn4 = ((locals.var_beta0_i_dn4 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn4));
        locals.var_beta0_i_dn5 = ((locals.var_beta0_i_dn5 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn5));
        locals.var_beta0_i_dn6 = ((locals.var_beta0_i_dn6 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn6));
        locals.var_beta0_i_dn7 = ((locals.var_beta0_i_dn7 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn7));
        locals.var_beta0_i_dn8 = ((locals.var_beta0_i_dn8 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn8));
        locals.var_beta0_i_dn9 = ((locals.var_beta0_i_dn9 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn9));
        locals.var_beta0_i_dn10 = ((locals.var_beta0_i_dn10 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn10));
        locals.var_beta0_i_dn11 = ((locals.var_beta0_i_dn11 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn11));
        locals.var_beta0_i_dn12 = ((locals.var_beta0_i_dn12 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn12));
        locals.var_beta0_i_dn13 = ((locals.var_beta0_i_dn13 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn13));
        locals.var_beta0_i_dn14 = ((locals.var_beta0_i_dn14 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn14));

        let assign4040_e5272: f64 = (locals.var_inv_w).powf(p.p520);
        let assign4040_e5275: f64 = (locals.var_inv_wwide).powf(p.p520);
        let assign4040_e5276: f64 = (assign4040_e5272 - assign4040_e5275);
        let assign4040_e5278: f64 = (assign4040_e5276).max(0.0);
        let assign4040_e5279: f64 = (p.p519 * assign4040_e5278);
        locals.var_t1 = assign4040_e5279;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        locals.var_beta1_i = p.p518;
        locals.var_beta1_i_dn0 = 0.0;
        locals.var_beta1_i_dn2 = 0.0;
        locals.var_beta1_i_dn3 = 0.0;
        locals.var_beta1_i_dn4 = 0.0;
        locals.var_beta1_i_dn5 = 0.0;
        locals.var_beta1_i_dn6 = 0.0;
        locals.var_beta1_i_dn7 = 0.0;
        locals.var_beta1_i_dn8 = 0.0;
        locals.var_beta1_i_dn9 = 0.0;
        locals.var_beta1_i_dn10 = 0.0;
        locals.var_beta1_i_dn11 = 0.0;
        locals.var_beta1_i_dn12 = 0.0;
        locals.var_beta1_i_dn13 = 0.0;
        locals.var_beta1_i_dn14 = 0.0;

        let assign4060_e5284: f64 = (1.0 + locals.var_t1);
        let assign4060_e5285: f64 = (locals.var_beta1_i * assign4060_e5284);
        locals.var_beta1_i = assign4060_e5285;
        locals.var_beta1_i_dn0 = ((locals.var_beta1_i_dn0 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn0));
        locals.var_beta1_i_dn2 = ((locals.var_beta1_i_dn2 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn2));
        locals.var_beta1_i_dn3 = ((locals.var_beta1_i_dn3 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn3));
        locals.var_beta1_i_dn4 = ((locals.var_beta1_i_dn4 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn4));
        locals.var_beta1_i_dn5 = ((locals.var_beta1_i_dn5 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn5));
        locals.var_beta1_i_dn6 = ((locals.var_beta1_i_dn6 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn6));
        locals.var_beta1_i_dn7 = ((locals.var_beta1_i_dn7 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn7));
        locals.var_beta1_i_dn8 = ((locals.var_beta1_i_dn8 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn8));
        locals.var_beta1_i_dn9 = ((locals.var_beta1_i_dn9 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn9));
        locals.var_beta1_i_dn10 = ((locals.var_beta1_i_dn10 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn10));
        locals.var_beta1_i_dn11 = ((locals.var_beta1_i_dn11 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn11));
        locals.var_beta1_i_dn12 = ((locals.var_beta1_i_dn12 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn12));
        locals.var_beta1_i_dn13 = ((locals.var_beta1_i_dn13 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn13));
        locals.var_beta1_i_dn14 = ((locals.var_beta1_i_dn14 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn14));

        let assign4070_e5289: f64 = (locals.var_inv_w).powf(p.p523);
        let assign4070_e5292: f64 = (locals.var_inv_wwide).powf(p.p523);
        let assign4070_e5293: f64 = (assign4070_e5289 - assign4070_e5292);
        let assign4070_e5295: f64 = (assign4070_e5293).max(0.0);
        let assign4070_e5296: f64 = (p.p522 * assign4070_e5295);
        locals.var_t1 = assign4070_e5296;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        locals.var_beta2_i = p.p521;
        locals.var_beta2_i_dn0 = 0.0;
        locals.var_beta2_i_dn2 = 0.0;
        locals.var_beta2_i_dn3 = 0.0;
        locals.var_beta2_i_dn4 = 0.0;
        locals.var_beta2_i_dn5 = 0.0;
        locals.var_beta2_i_dn6 = 0.0;
        locals.var_beta2_i_dn7 = 0.0;
        locals.var_beta2_i_dn8 = 0.0;
        locals.var_beta2_i_dn9 = 0.0;
        locals.var_beta2_i_dn10 = 0.0;
        locals.var_beta2_i_dn11 = 0.0;
        locals.var_beta2_i_dn12 = 0.0;
        locals.var_beta2_i_dn13 = 0.0;
        locals.var_beta2_i_dn14 = 0.0;

        let assign4090_e5301: f64 = (1.0 + locals.var_t1);
        let assign4090_e5302: f64 = (locals.var_beta2_i * assign4090_e5301);
        locals.var_beta2_i = assign4090_e5302;
        locals.var_beta2_i_dn0 = ((locals.var_beta2_i_dn0 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn0));
        locals.var_beta2_i_dn2 = ((locals.var_beta2_i_dn2 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn2));
        locals.var_beta2_i_dn3 = ((locals.var_beta2_i_dn3 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn3));
        locals.var_beta2_i_dn4 = ((locals.var_beta2_i_dn4 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn4));
        locals.var_beta2_i_dn5 = ((locals.var_beta2_i_dn5 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn5));
        locals.var_beta2_i_dn6 = ((locals.var_beta2_i_dn6 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn6));
        locals.var_beta2_i_dn7 = ((locals.var_beta2_i_dn7 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn7));
        locals.var_beta2_i_dn8 = ((locals.var_beta2_i_dn8 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn8));
        locals.var_beta2_i_dn9 = ((locals.var_beta2_i_dn9 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn9));
        locals.var_beta2_i_dn10 = ((locals.var_beta2_i_dn10 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn10));
        locals.var_beta2_i_dn11 = ((locals.var_beta2_i_dn11 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn11));
        locals.var_beta2_i_dn12 = ((locals.var_beta2_i_dn12 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn12));
        locals.var_beta2_i_dn13 = ((locals.var_beta2_i_dn13 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn13));
        locals.var_beta2_i_dn14 = ((locals.var_beta2_i_dn14 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn14));

        let assign4100_e5307: f64 = (p.p631 * locals.var_inv_l);
        let assign4100_e5308: f64 = (1.0 + assign4100_e5307);
        let assign4100_e5311: f64 = (p.p632 * locals.var_inv_w);
        let assign4100_e5312: f64 = (assign4100_e5308 + assign4100_e5311);
        let assign4100_e5313: f64 = (locals.var_agidl_i * assign4100_e5312);
        locals.var_agidl_i = assign4100_e5313;

        let assign4110_e5318: f64 = (p.p649 * locals.var_inv_l);
        let assign4110_e5319: f64 = (1.0 + assign4110_e5318);
        let assign4110_e5322: f64 = (p.p650 * locals.var_inv_w);
        let assign4110_e5323: f64 = (assign4110_e5319 + assign4110_e5322);
        let assign4110_e5324: f64 = (locals.var_agisl_i * assign4110_e5323);
        locals.var_agisl_i = assign4110_e5324;

        let assign4120_e5329: f64 = (p.p557 * locals.var_inv_l);
        let assign4120_e5330: f64 = (1.0 + assign4120_e5329);
        let assign4120_e5333: f64 = (p.p558 * locals.var_inv_w);
        let assign4120_e5334: f64 = (assign4120_e5330 + assign4120_e5333);
        let assign4120_e5335: f64 = (locals.var_aigc_i * assign4120_e5334);
        locals.var_aigc_i = assign4120_e5335;

        let assign4130_e5340: f64 = (p.p559 * locals.var_inv_l);
        let assign4130_e5341: f64 = (1.0 + assign4130_e5340);
        let assign4130_e5344: f64 = (p.p560 * locals.var_inv_w);
        let assign4130_e5345: f64 = (assign4130_e5341 + assign4130_e5344);
        let assign4130_e5346: f64 = (locals.var_aigs_i * assign4130_e5345);
        locals.var_aigs_i = assign4130_e5346;

        let assign4140_e5351: f64 = (p.p561 * locals.var_inv_l);
        let assign4140_e5352: f64 = (1.0 + assign4140_e5351);
        let assign4140_e5355: f64 = (p.p562 * locals.var_inv_w);
        let assign4140_e5356: f64 = (assign4140_e5352 + assign4140_e5355);
        let assign4140_e5357: f64 = (locals.var_aigd_i * assign4140_e5356);
        locals.var_aigd_i = assign4140_e5357;

        let assign4150_e5362: f64 = (p.p563 * locals.var_inv_l);
        let assign4150_e5363: f64 = (1.0 + assign4150_e5362);
        let assign4150_e5364: f64 = (p.p556 * assign4150_e5363);
        locals.var_pigcd_i = assign4150_e5364;

        let assign4160_e5368: f64 = (locals.var_inv_lact).powf(p.p94);
        let assign4160_e5371: f64 = (locals.var_inv_llong).powf(p.p94);
        let assign4160_e5372: f64 = (assign4160_e5368 - assign4160_e5371);
        let assign4160_e5374: f64 = (assign4160_e5372).max(0.0);
        let assign4160_e5375: f64 = (p.p93 * assign4160_e5374);
        let assign4160_e5379: f64 = (locals.var_inv_lact).powf(p.p96);
        let assign4160_e5382: f64 = (locals.var_inv_llong).powf(p.p96);
        let assign4160_e5383: f64 = (assign4160_e5379 - assign4160_e5382);
        let assign4160_e5385: f64 = (assign4160_e5383).max(0.0);
        let assign4160_e5386: f64 = (p.p95 * assign4160_e5385);
        let assign4160_e5387: f64 = (assign4160_e5375 + assign4160_e5386);
        locals.var_t0 = assign4160_e5387;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign4170_e5391: f64 = (locals.var_inv_wact).powf(p.p98);
        let assign4170_e5394: f64 = (locals.var_inv_wwide).powf(p.p98);
        let assign4170_e5395: f64 = (assign4170_e5391 - assign4170_e5394);
        let assign4170_e5397: f64 = (assign4170_e5395).max(0.0);
        let assign4170_e5398: f64 = (p.p97 * assign4170_e5397);
        let assign4170_e5402: f64 = (locals.var_inv_wact * locals.var_inv_lact);
        let assign4170_e5404: f64 = (assign4170_e5402).powf(p.p100);
        let assign4170_e5405: f64 = (p.p99 * assign4170_e5404);
        let assign4170_e5406: f64 = (assign4170_e5398 + assign4170_e5405);
        locals.var_t1 = assign4170_e5406;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign4180_e5410: f64 = (1.0 + locals.var_t0);
        let assign4180_e5412: f64 = (assign4180_e5410 + locals.var_t1);
        let assign4180_e5413: f64 = (locals.var_ndepcv_i * assign4180_e5412);
        locals.var_ndepcv_i = assign4180_e5413;
        locals.var_ndepcv_i_dn0 = ((locals.var_ndepcv_i_dn0 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_ndepcv_i_dn2 = ((locals.var_ndepcv_i_dn2 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_ndepcv_i_dn3 = ((locals.var_ndepcv_i_dn3 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_ndepcv_i_dn4 = ((locals.var_ndepcv_i_dn4 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_ndepcv_i_dn5 = ((locals.var_ndepcv_i_dn5 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_ndepcv_i_dn6 = ((locals.var_ndepcv_i_dn6 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_ndepcv_i_dn7 = ((locals.var_ndepcv_i_dn7 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_ndepcv_i_dn8 = ((locals.var_ndepcv_i_dn8 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_ndepcv_i_dn9 = ((locals.var_ndepcv_i_dn9 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_ndepcv_i_dn10 = ((locals.var_ndepcv_i_dn10 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_ndepcv_i_dn11 = ((locals.var_ndepcv_i_dn11 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_ndepcv_i_dn12 = ((locals.var_ndepcv_i_dn12 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_ndepcv_i_dn13 = ((locals.var_ndepcv_i_dn13 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_ndepcv_i_dn14 = ((locals.var_ndepcv_i_dn14 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));

        let assign4190_e5417: f64 = (locals.var_inv_lact).powf(p.p121);
        let assign4190_e5420: f64 = (locals.var_inv_llong).powf(p.p121);
        let assign4190_e5421: f64 = (assign4190_e5417 - assign4190_e5420);
        let assign4190_e5423: f64 = (assign4190_e5421).max(0.0);
        let assign4190_e5424: f64 = (p.p120 * assign4190_e5423);
        locals.var_t0 = assign4190_e5424;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4200_e5428: f64 = (locals.var_inv_wact).powf(p.p123);
        let assign4200_e5431: f64 = (locals.var_inv_wwide).powf(p.p123);
        let assign4200_e5432: f64 = (assign4200_e5428 - assign4200_e5431);
        let assign4200_e5434: f64 = (assign4200_e5432).max(0.0);
        let assign4200_e5435: f64 = (p.p122 * assign4200_e5434);
        let assign4200_e5439: f64 = (locals.var_inv_wl).powf(p.p125);
        let assign4200_e5440: f64 = (p.p124 * assign4200_e5439);
        let assign4200_e5441: f64 = (assign4200_e5435 + assign4200_e5440);
        locals.var_t1 = assign4200_e5441;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign4210_e5445: f64 = (1.0 + locals.var_t0);
        let assign4210_e5447: f64 = (assign4210_e5445 + locals.var_t1);
        let assign4210_e5448: f64 = (locals.var_vfb_i * assign4210_e5447);
        locals.var_vfb_i = assign4210_e5448;
        locals.var_vfb_i_dn0 = ((locals.var_vfb_i_dn0 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_vfb_i_dn2 = ((locals.var_vfb_i_dn2 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_vfb_i_dn3 = ((locals.var_vfb_i_dn3 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vfb_i_dn4 = ((locals.var_vfb_i_dn4 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vfb_i_dn5 = ((locals.var_vfb_i_dn5 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vfb_i_dn6 = ((locals.var_vfb_i_dn6 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vfb_i_dn7 = ((locals.var_vfb_i_dn7 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vfb_i_dn8 = ((locals.var_vfb_i_dn8 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vfb_i_dn9 = ((locals.var_vfb_i_dn9 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vfb_i_dn10 = ((locals.var_vfb_i_dn10 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vfb_i_dn11 = ((locals.var_vfb_i_dn11 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vfb_i_dn12 = ((locals.var_vfb_i_dn12 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_vfb_i_dn13 = ((locals.var_vfb_i_dn13 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_vfb_i_dn14 = ((locals.var_vfb_i_dn14 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));

        let assign4220_e5452: f64 = (locals.var_inv_lact).powf(p.p131);
        let assign4220_e5455: f64 = (locals.var_inv_llong).powf(p.p131);
        let assign4220_e5456: f64 = (assign4220_e5452 - assign4220_e5455);
        let assign4220_e5458: f64 = (assign4220_e5456).max(0.0);
        let assign4220_e5459: f64 = (p.p130 * assign4220_e5458);
        locals.var_t0 = assign4220_e5459;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign4230_e5463: f64 = (locals.var_inv_wact).powf(p.p133);
        let assign4230_e5466: f64 = (locals.var_inv_wwide).powf(p.p133);
        let assign4230_e5467: f64 = (assign4230_e5463 - assign4230_e5466);
        let assign4230_e5469: f64 = (assign4230_e5467).max(0.0);
        let assign4230_e5470: f64 = (p.p132 * assign4230_e5469);
        let assign4230_e5474: f64 = (locals.var_inv_wl).powf(p.p135);
        let assign4230_e5475: f64 = (p.p134 * assign4230_e5474);
        let assign4230_e5476: f64 = (assign4230_e5470 + assign4230_e5475);
        locals.var_t1 = assign4230_e5476;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign4240_e5480: f64 = (1.0 + locals.var_t0);
        let assign4240_e5482: f64 = (assign4240_e5480 + locals.var_t1);
        let assign4240_e5483: f64 = (locals.var_vfbcv_i * assign4240_e5482);
        locals.var_vfbcv_i = assign4240_e5483;
        locals.var_vfbcv_i_dn0 = ((locals.var_vfbcv_i_dn0 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_vfbcv_i_dn2 = ((locals.var_vfbcv_i_dn2 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_vfbcv_i_dn3 = ((locals.var_vfbcv_i_dn3 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vfbcv_i_dn4 = ((locals.var_vfbcv_i_dn4 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vfbcv_i_dn5 = ((locals.var_vfbcv_i_dn5 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vfbcv_i_dn6 = ((locals.var_vfbcv_i_dn6 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vfbcv_i_dn7 = ((locals.var_vfbcv_i_dn7 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vfbcv_i_dn8 = ((locals.var_vfbcv_i_dn8 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vfbcv_i_dn9 = ((locals.var_vfbcv_i_dn9 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vfbcv_i_dn10 = ((locals.var_vfbcv_i_dn10 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vfbcv_i_dn11 = ((locals.var_vfbcv_i_dn11 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vfbcv_i_dn12 = ((locals.var_vfbcv_i_dn12 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_vfbcv_i_dn13 = ((locals.var_vfbcv_i_dn13 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_vfbcv_i_dn14 = ((locals.var_vfbcv_i_dn14 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));

        let assign4250_e5487: f64 = (locals.var_inv_lact).powf(p.p264);
        let assign4250_e5490: f64 = (locals.var_inv_llong).powf(p.p264);
        let assign4250_e5491: f64 = (assign4250_e5487 - assign4250_e5490);
        let assign4250_e5493: f64 = (assign4250_e5491).max(0.0);
        let assign4250_e5494: f64 = (p.p263 * assign4250_e5493);
        locals.var_t0 = assign4250_e5494;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign4260_e5498: f64 = (locals.var_inv_w).powf(p.p266);
        let assign4260_e5501: f64 = (locals.var_inv_wwide).powf(p.p266);
        let assign4260_e5502: f64 = (assign4260_e5498 - assign4260_e5501);
        let assign4260_e5504: f64 = (assign4260_e5502).max(0.0);
        let assign4260_e5505: f64 = (p.p265 * assign4260_e5504);
        let assign4260_e5509: f64 = (locals.var_inv_wl).powf(p.p268);
        let assign4260_e5510: f64 = (p.p267 * assign4260_e5509);
        let assign4260_e5511: f64 = (assign4260_e5505 + assign4260_e5510);
        locals.var_t1 = assign4260_e5511;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign4270_e5515: f64 = (1.0 + locals.var_t0);
        let assign4270_e5517: f64 = (assign4270_e5515 + locals.var_t1);
        let assign4270_e5518: f64 = (locals.var_vsatcv_i * assign4270_e5517);
        locals.var_vsatcv_i = assign4270_e5518;
        locals.var_vsatcv_i_dn0 = ((locals.var_vsatcv_i_dn0 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_vsatcv_i_dn2 = ((locals.var_vsatcv_i_dn2 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_vsatcv_i_dn3 = ((locals.var_vsatcv_i_dn3 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_vsatcv_i_dn4 = ((locals.var_vsatcv_i_dn4 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_vsatcv_i_dn5 = ((locals.var_vsatcv_i_dn5 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_vsatcv_i_dn6 = ((locals.var_vsatcv_i_dn6 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_vsatcv_i_dn7 = ((locals.var_vsatcv_i_dn7 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_vsatcv_i_dn8 = ((locals.var_vsatcv_i_dn8 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_vsatcv_i_dn9 = ((locals.var_vsatcv_i_dn9 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_vsatcv_i_dn10 = ((locals.var_vsatcv_i_dn10 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_vsatcv_i_dn11 = ((locals.var_vsatcv_i_dn11 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_vsatcv_i_dn12 = ((locals.var_vsatcv_i_dn12 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_vsatcv_i_dn13 = ((locals.var_vsatcv_i_dn13 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_vsatcv_i_dn14 = ((locals.var_vsatcv_i_dn14 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));

        let assign4280_e5524: f64 = (locals.var_inv_lact).powf(p.p353);
        let assign4280_e5527: f64 = (locals.var_inv_llong).powf(p.p353);
        let assign4280_e5528: f64 = (assign4280_e5524 - assign4280_e5527);
        let assign4280_e5530: f64 = (assign4280_e5528).max(0.0);
        let assign4280_e5531: f64 = (p.p352 * assign4280_e5530);
        let assign4280_e5532: f64 = (1.0 + assign4280_e5531);
        let assign4280_e5533: f64 = (locals.var_pclmcv_i * assign4280_e5532);
        locals.var_pclmcv_i = assign4280_e5533;

        let assign4290_e5536: f64 = (locals.var_pclmcv_i).max(0.0);
        locals.var_pclmcv_i = assign4290_e5536;

        let assign4300_e5540: f64 = (locals.var_inv_l).powf(p.p187);
        let assign4300_e5543: f64 = (locals.var_inv_llong).powf(p.p187);
        let assign4300_e5544: f64 = (assign4300_e5540 - assign4300_e5543);
        let assign4300_e5546: f64 = (assign4300_e5544).max(0.0);
        let assign4300_e5547: f64 = (p.p186 * assign4300_e5546);
        locals.var_t0 = assign4300_e5547;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign4310_e5551: f64 = (locals.var_inv_w).powf(p.p189);
        let assign4310_e5554: f64 = (locals.var_inv_wwide).powf(p.p189);
        let assign4310_e5555: f64 = (assign4310_e5551 - assign4310_e5554);
        let assign4310_e5557: f64 = (assign4310_e5555).max(0.0);
        let assign4310_e5558: f64 = (p.p188 * assign4310_e5557);
        let assign4310_e5562: f64 = (locals.var_inv_wl).powf(p.p191);
        let assign4310_e5563: f64 = (p.p190 * assign4310_e5562);
        let assign4310_e5564: f64 = (assign4310_e5558 + assign4310_e5563);
        locals.var_t1 = assign4310_e5564;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign4320_e5568: f64 = (1.0 + locals.var_t0);
        let assign4320_e5570: f64 = (assign4320_e5568 + locals.var_t1);
        let assign4320_e5571: f64 = (locals.var_k1_i * assign4320_e5570);
        locals.var_k1_i = assign4320_e5571;
        locals.var_k1_i_dn0 = ((locals.var_k1_i_dn0 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_k1_i_dn2 = ((locals.var_k1_i_dn2 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_k1_i_dn3 = ((locals.var_k1_i_dn3 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_k1_i_dn4 = ((locals.var_k1_i_dn4 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_k1_i_dn5 = ((locals.var_k1_i_dn5 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_k1_i_dn6 = ((locals.var_k1_i_dn6 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_k1_i_dn7 = ((locals.var_k1_i_dn7 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_k1_i_dn8 = ((locals.var_k1_i_dn8 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_k1_i_dn9 = ((locals.var_k1_i_dn9 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_k1_i_dn10 = ((locals.var_k1_i_dn10 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_k1_i_dn11 = ((locals.var_k1_i_dn11 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_k1_i_dn12 = ((locals.var_k1_i_dn12 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_k1_i_dn13 = ((locals.var_k1_i_dn13 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_k1_i_dn14 = ((locals.var_k1_i_dn14 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));

        let assign4330_e5575: f64 = (locals.var_inv_l).powf(p.p197);
        let assign4330_e5578: f64 = (locals.var_inv_llong).powf(p.p197);
        let assign4330_e5579: f64 = (assign4330_e5575 - assign4330_e5578);
        let assign4330_e5581: f64 = (assign4330_e5579).max(0.0);
        let assign4330_e5582: f64 = (p.p196 * assign4330_e5581);
        locals.var_t0 = assign4330_e5582;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign4340_e5586: f64 = (locals.var_inv_w).powf(p.p199);
        let assign4340_e5589: f64 = (locals.var_inv_wwide).powf(p.p199);
        let assign4340_e5590: f64 = (assign4340_e5586 - assign4340_e5589);
        let assign4340_e5592: f64 = (assign4340_e5590).max(0.0);
        let assign4340_e5593: f64 = (p.p198 * assign4340_e5592);
        let assign4340_e5597: f64 = (locals.var_inv_wl).powf(p.p201);
        let assign4340_e5598: f64 = (p.p200 * assign4340_e5597);
        let assign4340_e5599: f64 = (assign4340_e5593 + assign4340_e5598);
        locals.var_t1 = assign4340_e5599;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign4350_e5603: f64 = (1.0 + locals.var_t0);
        let assign4350_e5605: f64 = (assign4350_e5603 + locals.var_t1);
        let assign4350_e5606: f64 = (locals.var_k2_i * assign4350_e5605);
        locals.var_k2_i = assign4350_e5606;
        locals.var_k2_i_dn0 = ((locals.var_k2_i_dn0 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn0 + locals.var_t1_dn0)));
        locals.var_k2_i_dn2 = ((locals.var_k2_i_dn2 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn2 + locals.var_t1_dn2)));
        locals.var_k2_i_dn3 = ((locals.var_k2_i_dn3 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn3 + locals.var_t1_dn3)));
        locals.var_k2_i_dn4 = ((locals.var_k2_i_dn4 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn4 + locals.var_t1_dn4)));
        locals.var_k2_i_dn5 = ((locals.var_k2_i_dn5 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn5 + locals.var_t1_dn5)));
        locals.var_k2_i_dn6 = ((locals.var_k2_i_dn6 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn6 + locals.var_t1_dn6)));
        locals.var_k2_i_dn7 = ((locals.var_k2_i_dn7 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn7 + locals.var_t1_dn7)));
        locals.var_k2_i_dn8 = ((locals.var_k2_i_dn8 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn8 + locals.var_t1_dn8)));
        locals.var_k2_i_dn9 = ((locals.var_k2_i_dn9 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn9 + locals.var_t1_dn9)));
        locals.var_k2_i_dn10 = ((locals.var_k2_i_dn10 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn10 + locals.var_t1_dn10)));
        locals.var_k2_i_dn11 = ((locals.var_k2_i_dn11 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn11 + locals.var_t1_dn11)));
        locals.var_k2_i_dn12 = ((locals.var_k2_i_dn12 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn12 + locals.var_t1_dn12)));
        locals.var_k2_i_dn13 = ((locals.var_k2_i_dn13 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn13 + locals.var_t1_dn13)));
        locals.var_k2_i_dn14 = ((locals.var_k2_i_dn14 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn14 + locals.var_t1_dn14)));

        let assign4360_e5612: f64 = (locals.var_inv_l).powf(p.p384);
        let assign4360_e5615: f64 = (locals.var_inv_llong).powf(p.p384);
        let assign4360_e5616: f64 = (assign4360_e5612 - assign4360_e5615);
        let assign4360_e5618: f64 = (assign4360_e5616).max(0.0);
        let assign4360_e5619: f64 = (p.p383 * assign4360_e5618);
        let assign4360_e5620: f64 = (1.0 + assign4360_e5619);
        let assign4360_e5621: f64 = (locals.var_prwb_i * assign4360_e5620);
        locals.var_prwb_i = assign4360_e5621;

        let assign4370_e5626: f64 = (locals.var_inv_l * p.p828);
        let assign4370_e5627: f64 = (1.0 + assign4370_e5626);
        let assign4370_e5628: f64 = (locals.var_ute_i * assign4370_e5627);
        locals.var_ute_i = assign4370_e5628;

        let assign4380_e5633: f64 = (locals.var_inv_l * p.p833);
        let assign4380_e5634: f64 = (1.0 + assign4380_e5633);
        let assign4380_e5635: f64 = (locals.var_ua1_i * assign4380_e5634);
        locals.var_ua1_i = assign4380_e5635;

        let assign4390_e5640: f64 = (locals.var_inv_l * p.p842);
        let assign4390_e5641: f64 = (1.0 + assign4390_e5640);
        let assign4390_e5642: f64 = (locals.var_ud1_i * assign4390_e5641);
        locals.var_ud1_i = assign4390_e5642;

        let assign4400_e5647: f64 = (locals.var_inv_l * p.p860);
        let assign4400_e5648: f64 = (1.0 + assign4400_e5647);
        let assign4400_e5649: f64 = (locals.var_at_i * assign4400_e5648);
        locals.var_at_i = assign4400_e5649;

        let assign4410_e5654: f64 = (locals.var_inv_l * p.p866);
        let assign4410_e5655: f64 = (1.0 + assign4410_e5654);
        let assign4410_e5656: f64 = (locals.var_ptwgt_i * assign4410_e5655);
        locals.var_ptwgt_i = assign4410_e5656;

        let assign4440_e5670: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign4440_e5670;

        let (assign4450_e5688,) = {
    if (locals.var_guard40 != 0.0) {
        let assign4450_e5677: f64 = (locals.var_inv_l).powf(p.p398);
        let assign4450_e5680: f64 = (locals.var_inv_llong).powf(p.p398);
        let assign4450_e5681: f64 = (assign4450_e5677 - assign4450_e5680);
        let assign4450_e5683: f64 = (assign4450_e5681).max(0.0);
        let assign4450_e5684: f64 = (p.p397 * assign4450_e5683);
        let assign4450_e5685: f64 = (1.0 + assign4450_e5684);
        let assign4450_e5686: f64 = (locals.var_rsw_i * assign4450_e5685);
        (assign4450_e5686,)
    } else {
        (locals.var_rsw_i,)
    }
};
        locals.var_rsw_i = assign4450_e5688;

        let (assign4460_e5706,) = {
    if (locals.var_guard40 != 0.0) {
        let assign4460_e5695: f64 = (locals.var_inv_l).powf(p.p408);
        let assign4460_e5698: f64 = (locals.var_inv_llong).powf(p.p408);
        let assign4460_e5699: f64 = (assign4460_e5695 - assign4460_e5698);
        let assign4460_e5701: f64 = (assign4460_e5699).max(0.0);
        let assign4460_e5702: f64 = (p.p407 * assign4460_e5701);
        let assign4460_e5703: f64 = (1.0 + assign4460_e5702);
        let assign4460_e5704: f64 = (locals.var_rdw_i * assign4460_e5703);
        (assign4460_e5704,)
    } else {
        (locals.var_rdw_i,)
    }
};
        locals.var_rdw_i = assign4460_e5706;

        let (assign4470_e5725,) = {
    if (locals.var_guard40 == 0.0) {
        let assign4470_e5714: f64 = (locals.var_inv_l).powf(p.p415);
        let assign4470_e5717: f64 = (locals.var_inv_llong).powf(p.p415);
        let assign4470_e5718: f64 = (assign4470_e5714 - assign4470_e5717);
        let assign4470_e5720: f64 = (assign4470_e5718).max(0.0);
        let assign4470_e5721: f64 = (p.p414 * assign4470_e5720);
        let assign4470_e5722: f64 = (1.0 + assign4470_e5721);
        let assign4470_e5723: f64 = (locals.var_rdsw_i * assign4470_e5722);
        (assign4470_e5723,)
    } else {
        (locals.var_rdsw_i,)
    }
};
        locals.var_rdsw_i = assign4470_e5725;

        let assign4480_e5728: f64 = if locals.var_ucs_i < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign4480_e5728;

        let (assign4490_e5732,) = {
    if (locals.var_guard41 != 0.0) {
        (1.0,)
    } else {
        (locals.var_ucs_i,)
    }
};
        locals.var_ucs_i = assign4490_e5732;

        let assign4500_e5735: f64 = if locals.var_ucs_i > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign4500_e5735;

        let (assign4510_e5742,) = {
    if ((locals.var_guard41 == 0.0) && (locals.var_guard42 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_ucs_i,)
    }
};
        locals.var_ucs_i = assign4510_e5742;

        let assign4520_e5745: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign4520_e5745;

        let assign4530_e5748: f64 = if locals.var_ucsr_i < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign4530_e5748;

        let (assign4540_e5754,) = {
    if ((locals.var_guard43 != 0.0) && (locals.var_guard44 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_ucsr_i,)
    }
};
        locals.var_ucsr_i = assign4540_e5754;

        let assign4550_e5757: f64 = if locals.var_ucsr_i > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign4550_e5757;

        let (assign4560_e5766,) = {
    if (((locals.var_guard43 != 0.0) && (locals.var_guard44 == 0.0)) && (locals.var_guard45 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_ucsr_i,)
    }
};
        locals.var_ucsr_i = assign4560_e5766;

        let assign4760_e5826: f64 = if locals.var_dlcig_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard65 = assign4760_e5826;

        let (assign4770_e5830,) = {
    if (locals.var_guard65 != 0.0) {
        (0.0,)
    } else {
        (locals.var_dlcig_i,)
    }
};
        locals.var_dlcig_i = assign4770_e5830;

        let assign4780_e5833: f64 = if locals.var_dlcigd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard66 = assign4780_e5833;

        let (assign4790_e5837,) = {
    if (locals.var_guard66 != 0.0) {
        (0.0,)
    } else {
        (locals.var_dlcigd_i,)
    }
};
        locals.var_dlcigd_i = assign4790_e5837;

        let assign4800_e5840: f64 = if locals.var_m0_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard67 = assign4800_e5840;

        let (assign4810_e5844,) = {
    if (locals.var_guard67 != 0.0) {
        (0.0,)
    } else {
        (locals.var_m0_i,)
    }
};
        locals.var_m0_i = assign4810_e5844;

        let assign4820_e5847: f64 = if locals.var_u0_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard68 = assign4820_e5847;

        let (assign4830_e5851,) = {
    if (locals.var_guard68 != 0.0) {
        (0.067,)
    } else {
        (locals.var_u0_i,)
    }
};
        locals.var_u0_i = assign4830_e5851;

        let assign4840_e5854: f64 = if locals.var_ua_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard69 = assign4840_e5854;

    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign4850_e5858, assign4850_e5858_d_n0, assign4850_e5858_d_n2, assign4850_e5858_d_n3, assign4850_e5858_d_n4, assign4850_e5858_d_n5, assign4850_e5858_d_n6, assign4850_e5858_d_n7, assign4850_e5858_d_n8, assign4850_e5858_d_n9, assign4850_e5858_d_n10, assign4850_e5858_d_n11, assign4850_e5858_d_n12, assign4850_e5858_d_n13, assign4850_e5858_d_n14,) = {
    if (locals.var_guard69 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ua_i, locals.var_ua_i_dn0, locals.var_ua_i_dn2, locals.var_ua_i_dn3, locals.var_ua_i_dn4, locals.var_ua_i_dn5, locals.var_ua_i_dn6, locals.var_ua_i_dn7, locals.var_ua_i_dn8, locals.var_ua_i_dn9, locals.var_ua_i_dn10, locals.var_ua_i_dn11, locals.var_ua_i_dn12, locals.var_ua_i_dn13, locals.var_ua_i_dn14,)
    }
};
        locals.var_ua_i = assign4850_e5858;
        locals.var_ua_i_dn0 = assign4850_e5858_d_n0;
        locals.var_ua_i_dn2 = assign4850_e5858_d_n2;
        locals.var_ua_i_dn3 = assign4850_e5858_d_n3;
        locals.var_ua_i_dn4 = assign4850_e5858_d_n4;
        locals.var_ua_i_dn5 = assign4850_e5858_d_n5;
        locals.var_ua_i_dn6 = assign4850_e5858_d_n6;
        locals.var_ua_i_dn7 = assign4850_e5858_d_n7;
        locals.var_ua_i_dn8 = assign4850_e5858_d_n8;
        locals.var_ua_i_dn9 = assign4850_e5858_d_n9;
        locals.var_ua_i_dn10 = assign4850_e5858_d_n10;
        locals.var_ua_i_dn11 = assign4850_e5858_d_n11;
        locals.var_ua_i_dn12 = assign4850_e5858_d_n12;
        locals.var_ua_i_dn13 = assign4850_e5858_d_n13;
        locals.var_ua_i_dn14 = assign4850_e5858_d_n14;

        let assign4860_e5861: f64 = if locals.var_eu_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign4860_e5861;

        let (assign4870_e5865, assign4870_e5865_d_n0, assign4870_e5865_d_n2, assign4870_e5865_d_n3, assign4870_e5865_d_n4, assign4870_e5865_d_n5, assign4870_e5865_d_n6, assign4870_e5865_d_n7, assign4870_e5865_d_n8, assign4870_e5865_d_n9, assign4870_e5865_d_n10, assign4870_e5865_d_n11, assign4870_e5865_d_n12, assign4870_e5865_d_n13, assign4870_e5865_d_n14,) = {
    if (locals.var_guard70 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eu_i, locals.var_eu_i_dn0, locals.var_eu_i_dn2, locals.var_eu_i_dn3, locals.var_eu_i_dn4, locals.var_eu_i_dn5, locals.var_eu_i_dn6, locals.var_eu_i_dn7, locals.var_eu_i_dn8, locals.var_eu_i_dn9, locals.var_eu_i_dn10, locals.var_eu_i_dn11, locals.var_eu_i_dn12, locals.var_eu_i_dn13, locals.var_eu_i_dn14,)
    }
};
        locals.var_eu_i = assign4870_e5865;
        locals.var_eu_i_dn0 = assign4870_e5865_d_n0;
        locals.var_eu_i_dn2 = assign4870_e5865_d_n2;
        locals.var_eu_i_dn3 = assign4870_e5865_d_n3;
        locals.var_eu_i_dn4 = assign4870_e5865_d_n4;
        locals.var_eu_i_dn5 = assign4870_e5865_d_n5;
        locals.var_eu_i_dn6 = assign4870_e5865_d_n6;
        locals.var_eu_i_dn7 = assign4870_e5865_d_n7;
        locals.var_eu_i_dn8 = assign4870_e5865_d_n8;
        locals.var_eu_i_dn9 = assign4870_e5865_d_n9;
        locals.var_eu_i_dn10 = assign4870_e5865_d_n10;
        locals.var_eu_i_dn11 = assign4870_e5865_d_n11;
        locals.var_eu_i_dn12 = assign4870_e5865_d_n12;
        locals.var_eu_i_dn13 = assign4870_e5865_d_n13;
        locals.var_eu_i_dn14 = assign4870_e5865_d_n14;

        let assign4880_e5868: f64 = if locals.var_ud_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard71 = assign4880_e5868;

        let (assign4890_e5872, assign4890_e5872_d_n0, assign4890_e5872_d_n2, assign4890_e5872_d_n3, assign4890_e5872_d_n4, assign4890_e5872_d_n5, assign4890_e5872_d_n6, assign4890_e5872_d_n7, assign4890_e5872_d_n8, assign4890_e5872_d_n9, assign4890_e5872_d_n10, assign4890_e5872_d_n11, assign4890_e5872_d_n12, assign4890_e5872_d_n13, assign4890_e5872_d_n14,) = {
    if (locals.var_guard71 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ud_i, locals.var_ud_i_dn0, locals.var_ud_i_dn2, locals.var_ud_i_dn3, locals.var_ud_i_dn4, locals.var_ud_i_dn5, locals.var_ud_i_dn6, locals.var_ud_i_dn7, locals.var_ud_i_dn8, locals.var_ud_i_dn9, locals.var_ud_i_dn10, locals.var_ud_i_dn11, locals.var_ud_i_dn12, locals.var_ud_i_dn13, locals.var_ud_i_dn14,)
    }
};
        locals.var_ud_i = assign4890_e5872;
        locals.var_ud_i_dn0 = assign4890_e5872_d_n0;
        locals.var_ud_i_dn2 = assign4890_e5872_d_n2;
        locals.var_ud_i_dn3 = assign4890_e5872_d_n3;
        locals.var_ud_i_dn4 = assign4890_e5872_d_n4;
        locals.var_ud_i_dn5 = assign4890_e5872_d_n5;
        locals.var_ud_i_dn6 = assign4890_e5872_d_n6;
        locals.var_ud_i_dn7 = assign4890_e5872_d_n7;
        locals.var_ud_i_dn8 = assign4890_e5872_d_n8;
        locals.var_ud_i_dn9 = assign4890_e5872_d_n9;
        locals.var_ud_i_dn10 = assign4890_e5872_d_n10;
        locals.var_ud_i_dn11 = assign4890_e5872_d_n11;
        locals.var_ud_i_dn12 = assign4890_e5872_d_n12;
        locals.var_ud_i_dn13 = assign4890_e5872_d_n13;
        locals.var_ud_i_dn14 = assign4890_e5872_d_n14;

        let assign4900_e5875: f64 = if locals.var_ucs_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard72 = assign4900_e5875;

        let (assign4910_e5879,) = {
    if (locals.var_guard72 != 0.0) {
        (0.0,)
    } else {
        (locals.var_ucs_i,)
    }
};
        locals.var_ucs_i = assign4910_e5879;

        let assign4920_e5882: f64 = if locals.var_beta1_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard73 = assign4920_e5882;

        let (assign4930_e5886, assign4930_e5886_d_n0, assign4930_e5886_d_n2, assign4930_e5886_d_n3, assign4930_e5886_d_n4, assign4930_e5886_d_n5, assign4930_e5886_d_n6, assign4930_e5886_d_n7, assign4930_e5886_d_n8, assign4930_e5886_d_n9, assign4930_e5886_d_n10, assign4930_e5886_d_n11, assign4930_e5886_d_n12, assign4930_e5886_d_n13, assign4930_e5886_d_n14,) = {
    if (locals.var_guard73 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_beta1_i, locals.var_beta1_i_dn0, locals.var_beta1_i_dn2, locals.var_beta1_i_dn3, locals.var_beta1_i_dn4, locals.var_beta1_i_dn5, locals.var_beta1_i_dn6, locals.var_beta1_i_dn7, locals.var_beta1_i_dn8, locals.var_beta1_i_dn9, locals.var_beta1_i_dn10, locals.var_beta1_i_dn11, locals.var_beta1_i_dn12, locals.var_beta1_i_dn13, locals.var_beta1_i_dn14,)
    }
};
        locals.var_beta1_i = assign4930_e5886;
        locals.var_beta1_i_dn0 = assign4930_e5886_d_n0;
        locals.var_beta1_i_dn2 = assign4930_e5886_d_n2;
        locals.var_beta1_i_dn3 = assign4930_e5886_d_n3;
        locals.var_beta1_i_dn4 = assign4930_e5886_d_n4;
        locals.var_beta1_i_dn5 = assign4930_e5886_d_n5;
        locals.var_beta1_i_dn6 = assign4930_e5886_d_n6;
        locals.var_beta1_i_dn7 = assign4930_e5886_d_n7;
        locals.var_beta1_i_dn8 = assign4930_e5886_d_n8;
        locals.var_beta1_i_dn9 = assign4930_e5886_d_n9;
        locals.var_beta1_i_dn10 = assign4930_e5886_d_n10;
        locals.var_beta1_i_dn11 = assign4930_e5886_d_n11;
        locals.var_beta1_i_dn12 = assign4930_e5886_d_n12;
        locals.var_beta1_i_dn13 = assign4930_e5886_d_n13;
        locals.var_beta1_i_dn14 = assign4930_e5886_d_n14;

        let assign4940_e5889: f64 = if p.p1065 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard74 = assign4940_e5889;

        let (assign4950_e5893,) = {
    if (locals.var_guard74 != 0.0) {
        (p.p1066,)
    } else {
        (locals.var_lh1,)
    }
};
        locals.var_lh1 = assign4950_e5893;

        let assign4960_e5896: f64 = if locals.var_leff > locals.var_lh1 { 1.0 } else { 0.0 };
        locals.var_guard75 = assign4960_e5896;

        let (assign4970_e5904, assign4970_e5904_d_n0, assign4970_e5904_d_n2, assign4970_e5904_d_n3, assign4970_e5904_d_n4, assign4970_e5904_d_n5, assign4970_e5904_d_n6, assign4970_e5904_d_n7, assign4970_e5904_d_n8, assign4970_e5904_d_n9, assign4970_e5904_d_n10, assign4970_e5904_d_n11, assign4970_e5904_d_n12, assign4970_e5904_d_n13, assign4970_e5904_d_n14,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard75 != 0.0)) {
        let assign4970_e5902: f64 = (locals.var_leff - locals.var_lh1);
        (assign4970_e5902, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign4970_e5904;
        locals.var_t0_dn0 = assign4970_e5904_d_n0;
        locals.var_t0_dn2 = assign4970_e5904_d_n2;
        locals.var_t0_dn3 = assign4970_e5904_d_n3;
        locals.var_t0_dn4 = assign4970_e5904_d_n4;
        locals.var_t0_dn5 = assign4970_e5904_d_n5;
        locals.var_t0_dn6 = assign4970_e5904_d_n6;
        locals.var_t0_dn7 = assign4970_e5904_d_n7;
        locals.var_t0_dn8 = assign4970_e5904_d_n8;
        locals.var_t0_dn9 = assign4970_e5904_d_n9;
        locals.var_t0_dn10 = assign4970_e5904_d_n10;
        locals.var_t0_dn11 = assign4970_e5904_d_n11;
        locals.var_t0_dn12 = assign4970_e5904_d_n12;
        locals.var_t0_dn13 = assign4970_e5904_d_n13;
        locals.var_t0_dn14 = assign4970_e5904_d_n14;

        let (assign4980_e5911,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard75 == 0.0)) {
        (locals.var_leff,)
    } else {
        (locals.var_lh1,)
    }
};
        locals.var_lh1 = assign4980_e5911;

        let (assign4990_e5918, assign4990_e5918_d_n0, assign4990_e5918_d_n2, assign4990_e5918_d_n3, assign4990_e5918_d_n4, assign4990_e5918_d_n5, assign4990_e5918_d_n6, assign4990_e5918_d_n7, assign4990_e5918_d_n8, assign4990_e5918_d_n9, assign4990_e5918_d_n10, assign4990_e5918_d_n11, assign4990_e5918_d_n12, assign4990_e5918_d_n13, assign4990_e5918_d_n14,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard75 == 0.0)) {
        (locals.var_lh1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign4990_e5918;
        locals.var_t0_dn0 = assign4990_e5918_d_n0;
        locals.var_t0_dn2 = assign4990_e5918_d_n2;
        locals.var_t0_dn3 = assign4990_e5918_d_n3;
        locals.var_t0_dn4 = assign4990_e5918_d_n4;
        locals.var_t0_dn5 = assign4990_e5918_d_n5;
        locals.var_t0_dn6 = assign4990_e5918_d_n6;
        locals.var_t0_dn7 = assign4990_e5918_d_n7;
        locals.var_t0_dn8 = assign4990_e5918_d_n8;
        locals.var_t0_dn9 = assign4990_e5918_d_n9;
        locals.var_t0_dn10 = assign4990_e5918_d_n10;
        locals.var_t0_dn11 = assign4990_e5918_d_n11;
        locals.var_t0_dn12 = assign4990_e5918_d_n12;
        locals.var_t0_dn13 = assign4990_e5918_d_n13;
        locals.var_t0_dn14 = assign4990_e5918_d_n14;

        let assign5000_e5922: f64 = (locals.var_t0 / 2.0);
        let assign5000_e5923: f64 = if p.p801 >= assign5000_e5922 { 1.0 } else { 0.0 };
        locals.var_guard76 = assign5000_e5923;

        let (assign5010_e5929,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard76 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_lintnoi_i,)
    }
};
        locals.var_lintnoi_i = assign5010_e5929;

        let (assign5020_e5936,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard76 == 0.0)) {
        (p.p801,)
    } else {
        (locals.var_lintnoi_i,)
    }
};
        locals.var_lintnoi_i = assign5020_e5936;

        locals.var_nuendd = 0.0;

        locals.var_nuends = 0.0;

        locals.var_nuintd = 0.0;

        locals.var_nuints = 0.0;

        locals.var_rend = 0.0;

        locals.var_rint = 0.0;

        let assign5090_e5945: f64 = (p.p695 - p.p698);
        locals.var_dmcgeff = assign5090_e5945;

        locals.var_dmcieff = p.p696;

        let assign5110_e5949: f64 = (p.p697 - p.p698);
        locals.var_dmdgeff = assign5110_e5949;

        let assign5120_e5951: f64 = if param_given[3] { 1.0 } else { 0.0 };
        locals.var_guard77 = assign5120_e5951;

        let (assign5130_e5957,) = {
    if (locals.var_guard77 != 0.0) {
        let assign5130_e5955: f64 = (p.p374 * p.p3);
        (assign5130_e5955,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign5130_e5957;

        let assign5140_e5964: f64 = if ((p.p10 > 0.0) && (p.p374 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard78 = assign5140_e5964;

        let assign5150_e5967: f64 = if p.p9 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard79 = assign5150_e5967;

        let assign5160_e5970: f64 = (p.p2 % 2.0);
        let assign5160_e5972: f64 = if assign5160_e5970 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard80 = assign5160_e5972;

        let (assign5170_e5983,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign5170_e5983;

        let (assign5180_e5994,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign5180_e5994;

        let (assign5190_e6013,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) {
        let assign5190_e6006: f64 = (p.p2 - 1.0);
        let assign5190_e6008: f64 = (assign5190_e6006 / 2.0);
        let assign5190_e6010: f64 = (assign5190_e6008).max(0.0);
        let assign5190_e6011: f64 = (2.0 * assign5190_e6010);
        (assign5190_e6011,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign5190_e6013;

        let (assign5200_e6024,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) {
        (locals.var_nuintd,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign5200_e6024;

        let assign5210_e6027: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard81 = assign5210_e6027;

        let (assign5220_e6041,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign5220_e6041;

        let (assign5230_e6063,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 != 0.0)) {
        let assign5230_e6056: f64 = (p.p2 / 2.0);
        let assign5230_e6058: f64 = (assign5230_e6056 - 1.0);
        let assign5230_e6060: f64 = (assign5230_e6058).max(0.0);
        let assign5230_e6061: f64 = (2.0 * assign5230_e6060);
        (assign5230_e6061,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign5230_e6063;

        let (assign5240_e6077,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign5240_e6077;

        let (assign5250_e6091,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 != 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign5250_e6091;

        let (assign5260_e6106,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign5260_e6106;

        let (assign5270_e6121,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 == 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign5270_e6121;

        let (assign5280_e6136,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign5280_e6136;

        let (assign5290_e6159,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 == 0.0)) {
        let assign5290_e6152: f64 = (p.p2 / 2.0);
        let assign5290_e6154: f64 = (assign5290_e6152 - 1.0);
        let assign5290_e6156: f64 = (assign5290_e6154).max(0.0);
        let assign5290_e6157: f64 = (2.0 * assign5290_e6156);
        (assign5290_e6157,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign5290_e6159;

        let assign5300_e6162: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard82 = assign5300_e6162;

        let assign5310_e6165: f64 = if locals.var_nuints == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign5310_e6165;

        let (assign5320_e6178,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 != 0.0)) && (locals.var_guard83 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign5320_e6178;

        let (assign5330_e6198,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 != 0.0)) && (locals.var_guard83 == 0.0)) {
        let assign5330_e6192: f64 = (p.p374 * locals.var_dmcgeff);
        let assign5330_e6195: f64 = (locals.var_weff * locals.var_nuints);
        let assign5330_e6196: f64 = (assign5330_e6192 / assign5330_e6195);
        (assign5330_e6196,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign5330_e6198;

        let assign5340_e6201: f64 = if locals.var_nuintd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign5340_e6201;

        let (assign5350_e6215,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 == 0.0)) && (locals.var_guard84 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign5350_e6215;

        let (assign5360_e6236,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 == 0.0)) && (locals.var_guard84 == 0.0)) {
        let assign5360_e6230: f64 = (p.p374 * locals.var_dmcgeff);
        let assign5360_e6233: f64 = (locals.var_weff * locals.var_nuintd);
        let assign5360_e6234: f64 = (assign5360_e6230 / assign5360_e6233);
        (assign5360_e6234,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign5360_e6236;

        let assign5370_e6239: f64 = if p.p9 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard85 = assign5370_e6239;

        let assign5380_e6242: f64 = if p.p9 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign5380_e6242;

        let assign5390_e6245: f64 = if p.p9 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign5390_e6245;

        let assign5400_e6248: f64 = if p.p9 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard88 = assign5400_e6248;

        let assign5410_e6251: f64 = if p.p9 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign5410_e6251;

        let assign5420_e6254: f64 = if p.p9 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign5420_e6254;

        let assign5430_e6257: f64 = if p.p9 == 6.0 { 1.0 } else { 0.0 };
        locals.var_guard91 = assign5430_e6257;

        let assign5440_e6260: f64 = if p.p9 == 7.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign5440_e6260;

        let assign5450_e6263: f64 = if p.p9 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign5450_e6263;

        let assign5460_e6266: f64 = if p.p9 == 9.0 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign5460_e6266;

        let assign5470_e6269: f64 = if p.p9 == 10.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign5470_e6269;

        let assign5480_e6272: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign5480_e6272;

        let assign5490_e6275: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign5490_e6275;

        let assign5500_e6286: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard98 = assign5500_e6286;

        let assign5510_e6297: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard99 = assign5510_e6297;

        let assign5520_e6300: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard100 = assign5520_e6300;

        let (assign5530_e6317,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5530_e6317;

        let (assign5540_e6341,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign5540_e6335: f64 = (p.p374 * locals.var_dmcgeff);
        let assign5540_e6338: f64 = (locals.var_weff * locals.var_nuends);
        let assign5540_e6339: f64 = (assign5540_e6335 / assign5540_e6338);
        (assign5540_e6339,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5540_e6341;

        let assign5560_e6352: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5560_e6355: f64 = if ((locals.var_nuends == 0.0) || (assign5560_e6352 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard102 = assign5560_e6355;

        let (assign5570_e6375,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && ((locals.var_guard99 != 0.0) && (locals.var_guard98 == 0.0))) && (locals.var_guard102 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5570_e6375;

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5580_e6406,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && ((locals.var_guard99 != 0.0) && (locals.var_guard98 == 0.0))) && (locals.var_guard102 == 0.0)) {
        let assign5580_e6396: f64 = (p.p374 * locals.var_weff);
        let assign5580_e6399: f64 = (3.0 * locals.var_nuends);
        let assign5580_e6402: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5580_e6403: f64 = (assign5580_e6399 * assign5580_e6402);
        let assign5580_e6404: f64 = (assign5580_e6396 / assign5580_e6403);
        (assign5580_e6404,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5580_e6406;

        let (assign5590_e6424,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && (!((locals.var_guard98 != 0.0) || (locals.var_guard99 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5590_e6424;

        let assign5600_e6435: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard103 = assign5600_e6435;

        let assign5610_e6446: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard104 = assign5610_e6446;

        let assign5620_e6449: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign5620_e6449;

        let (assign5630_e6467,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard105 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5630_e6467;

        let (assign5640_e6492,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard105 == 0.0)) {
        let assign5640_e6486: f64 = (p.p374 * locals.var_dmcgeff);
        let assign5640_e6489: f64 = (locals.var_weff * locals.var_nuends);
        let assign5640_e6490: f64 = (assign5640_e6486 / assign5640_e6489);
        (assign5640_e6490,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5640_e6492;

        let assign5660_e6503: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5660_e6506: f64 = if ((locals.var_nuends == 0.0) || (assign5660_e6503 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard107 = assign5660_e6506;

        let (assign5670_e6527,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && ((locals.var_guard104 != 0.0) && (locals.var_guard103 == 0.0))) && (locals.var_guard107 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5670_e6527;

        let (assign5680_e6559,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && ((locals.var_guard104 != 0.0) && (locals.var_guard103 == 0.0))) && (locals.var_guard107 == 0.0)) {
        let assign5680_e6549: f64 = (p.p374 * locals.var_weff);
        let assign5680_e6552: f64 = (3.0 * locals.var_nuends);
        let assign5680_e6555: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5680_e6556: f64 = (assign5680_e6552 * assign5680_e6555);
        let assign5680_e6557: f64 = (assign5680_e6549 / assign5680_e6556);
        (assign5680_e6557,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5680_e6559;

        let (assign5690_e6578,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (!((locals.var_guard103 != 0.0) || (locals.var_guard104 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5690_e6578;

        let assign5700_e6581: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign5700_e6581;

        let assign5710_e6592: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard109 = assign5710_e6592;

        let assign5720_e6603: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard110 = assign5720_e6603;

        let assign5730_e6606: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign5730_e6606;

        let (assign5740_e6624,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5740_e6624;

        let (assign5750_e6649,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 == 0.0)) {
        let assign5750_e6643: f64 = (p.p374 * locals.var_dmcgeff);
        let assign5750_e6646: f64 = (locals.var_weff * locals.var_nuendd);
        let assign5750_e6647: f64 = (assign5750_e6643 / assign5750_e6646);
        (assign5750_e6647,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5750_e6649;

        let assign5770_e6660: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5770_e6663: f64 = if ((locals.var_nuendd == 0.0) || (assign5770_e6660 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard113 = assign5770_e6663;

        let (assign5780_e6684,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && ((locals.var_guard110 != 0.0) && (locals.var_guard109 == 0.0))) && (locals.var_guard113 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5780_e6684;

        let (assign5790_e6716,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && ((locals.var_guard110 != 0.0) && (locals.var_guard109 == 0.0))) && (locals.var_guard113 == 0.0)) {
        let assign5790_e6706: f64 = (p.p374 * locals.var_weff);
        let assign5790_e6709: f64 = (3.0 * locals.var_nuendd);
        let assign5790_e6712: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5790_e6713: f64 = (assign5790_e6709 * assign5790_e6712);
        let assign5790_e6714: f64 = (assign5790_e6706 / assign5790_e6713);
        (assign5790_e6714,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5790_e6716;

        let (assign5800_e6735,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && (!((locals.var_guard109 != 0.0) || (locals.var_guard110 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5800_e6735;

        let assign5810_e6746: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard114 = assign5810_e6746;

        let assign5820_e6757: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard115 = assign5820_e6757;

        let assign5830_e6760: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign5830_e6760;

        let (assign5840_e6779,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && (locals.var_guard114 != 0.0)) && (locals.var_guard116 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5840_e6779;

        let (assign5850_e6805,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && (locals.var_guard114 != 0.0)) && (locals.var_guard116 == 0.0)) {
        let assign5850_e6799: f64 = (p.p374 * locals.var_dmcgeff);
        let assign5850_e6802: f64 = (locals.var_weff * locals.var_nuendd);
        let assign5850_e6803: f64 = (assign5850_e6799 / assign5850_e6802);
        (assign5850_e6803,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5850_e6805;

        let assign5870_e6816: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5870_e6819: f64 = if ((locals.var_nuendd == 0.0) || (assign5870_e6816 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard118 = assign5870_e6819;

        let (assign5880_e6841,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && ((locals.var_guard115 != 0.0) && (locals.var_guard114 == 0.0))) && (locals.var_guard118 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5880_e6841;

        let (assign5890_e6874,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && ((locals.var_guard115 != 0.0) && (locals.var_guard114 == 0.0))) && (locals.var_guard118 == 0.0)) {
        let assign5890_e6864: f64 = (p.p374 * locals.var_weff);
        let assign5890_e6867: f64 = (3.0 * locals.var_nuendd);
        let assign5890_e6870: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5890_e6871: f64 = (assign5890_e6867 * assign5890_e6870);
        let assign5890_e6872: f64 = (assign5890_e6864 / assign5890_e6871);
        (assign5890_e6872,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5890_e6874;

        let (assign5900_e6894,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && (!((locals.var_guard114 != 0.0) || (locals.var_guard115 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5900_e6894;

        let assign5910_e6897: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard119 = assign5910_e6897;

        let assign5920_e6900: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign5920_e6900;

        let assign5930_e6911: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard121 = assign5930_e6911;

        let assign5940_e6922: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard122 = assign5940_e6922;

        let assign5950_e6925: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard123 = assign5950_e6925;

        let (assign5960_e6945,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && (locals.var_guard123 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5960_e6945;

        let (assign5970_e6972,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && (locals.var_guard123 == 0.0)) {
        let assign5970_e6966: f64 = (p.p374 * locals.var_dmcgeff);
        let assign5970_e6969: f64 = (locals.var_weff * locals.var_nuends);
        let assign5970_e6970: f64 = (assign5970_e6966 / assign5970_e6969);
        (assign5970_e6970,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign5970_e6972;

        let assign5990_e6983: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5990_e6986: f64 = if ((locals.var_nuends == 0.0) || (assign5990_e6983 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard125 = assign5990_e6986;

        let (assign6000_e7009,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && ((locals.var_guard122 != 0.0) && (locals.var_guard121 == 0.0))) && (locals.var_guard125 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6000_e7009;

        let (assign6010_e7043,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && ((locals.var_guard122 != 0.0) && (locals.var_guard121 == 0.0))) && (locals.var_guard125 == 0.0)) {
        let assign6010_e7033: f64 = (p.p374 * locals.var_weff);
        let assign6010_e7036: f64 = (3.0 * locals.var_nuends);
        let assign6010_e7039: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6010_e7040: f64 = (assign6010_e7036 * assign6010_e7039);
        let assign6010_e7041: f64 = (assign6010_e7033 / assign6010_e7040);
        (assign6010_e7041,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6010_e7043;

        let (assign6020_e7064,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && (!((locals.var_guard121 != 0.0) || (locals.var_guard122 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6020_e7064;

        let assign6030_e7075: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard126 = assign6030_e7075;

        let assign6040_e7086: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard127 = assign6040_e7086;

        let assign6050_e7089: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard128 = assign6050_e7089;

        let (assign6060_e7110,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && (locals.var_guard126 != 0.0)) && (locals.var_guard128 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6060_e7110;

        let (assign6070_e7138,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && (locals.var_guard126 != 0.0)) && (locals.var_guard128 == 0.0)) {
        let assign6070_e7132: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6070_e7135: f64 = (locals.var_weff * locals.var_nuends);
        let assign6070_e7136: f64 = (assign6070_e7132 / assign6070_e7135);
        (assign6070_e7136,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6070_e7138;

        let assign6090_e7149: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6090_e7152: f64 = if ((locals.var_nuends == 0.0) || (assign6090_e7149 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard130 = assign6090_e7152;

        let (assign6100_e7176,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && ((locals.var_guard127 != 0.0) && (locals.var_guard126 == 0.0))) && (locals.var_guard130 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6100_e7176;

        let (assign6110_e7211,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && ((locals.var_guard127 != 0.0) && (locals.var_guard126 == 0.0))) && (locals.var_guard130 == 0.0)) {
        let assign6110_e7201: f64 = (p.p374 * locals.var_weff);
        let assign6110_e7204: f64 = (3.0 * locals.var_nuends);
        let assign6110_e7207: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6110_e7208: f64 = (assign6110_e7204 * assign6110_e7207);
        let assign6110_e7209: f64 = (assign6110_e7201 / assign6110_e7208);
        (assign6110_e7209,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6110_e7211;

        let (assign6120_e7233,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && (!((locals.var_guard126 != 0.0) || (locals.var_guard127 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6120_e7233;

        let assign6130_e7236: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard131 = assign6130_e7236;

        let assign6140_e7247: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard132 = assign6140_e7247;

        let assign6150_e7258: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign6150_e7258;

        let assign6160_e7261: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard134 = assign6160_e7261;

        let (assign6170_e7282,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && (locals.var_guard132 != 0.0)) && (locals.var_guard134 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6170_e7282;

        let (assign6180_e7310,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && (locals.var_guard132 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6180_e7304: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6180_e7307: f64 = (locals.var_weff * locals.var_nuendd);
        let assign6180_e7308: f64 = (assign6180_e7304 / assign6180_e7307);
        (assign6180_e7308,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6180_e7310;

        let assign6200_e7320: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard136 = assign6200_e7320;

        let (assign6210_e7344,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && ((locals.var_guard133 != 0.0) && (locals.var_guard132 == 0.0))) && (locals.var_guard136 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6210_e7344;

        let (assign6220_e7377,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && ((locals.var_guard133 != 0.0) && (locals.var_guard132 == 0.0))) && (locals.var_guard136 == 0.0)) {
        let assign6220_e7369: f64 = (p.p374 * locals.var_weff);
        let assign6220_e7372: f64 = (6.0 * locals.var_nuendd);
        let assign6220_e7374: f64 = (assign6220_e7372 * locals.var_dmcgeff);
        let assign6220_e7375: f64 = (assign6220_e7369 / assign6220_e7374);
        (assign6220_e7375,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6220_e7377;

        let (assign6230_e7399,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && (!((locals.var_guard132 != 0.0) || (locals.var_guard133 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6230_e7399;

        let assign6240_e7410: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign6240_e7410;

        let assign6250_e7421: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard138 = assign6250_e7421;

        let assign6260_e7424: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard139 = assign6260_e7424;

        let (assign6270_e7446,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && (locals.var_guard137 != 0.0)) && (locals.var_guard139 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6270_e7446;

        let (assign6280_e7475,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && (locals.var_guard137 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6280_e7469: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6280_e7472: f64 = (locals.var_weff * locals.var_nuendd);
        let assign6280_e7473: f64 = (assign6280_e7469 / assign6280_e7472);
        (assign6280_e7473,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6280_e7475;

        let assign6300_e7485: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard141 = assign6300_e7485;

        let (assign6310_e7510,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && ((locals.var_guard138 != 0.0) && (locals.var_guard137 == 0.0))) && (locals.var_guard141 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6310_e7510;

        let (assign6320_e7544,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && ((locals.var_guard138 != 0.0) && (locals.var_guard137 == 0.0))) && (locals.var_guard141 == 0.0)) {
        let assign6320_e7536: f64 = (p.p374 * locals.var_weff);
        let assign6320_e7539: f64 = (6.0 * locals.var_nuendd);
        let assign6320_e7541: f64 = (assign6320_e7539 * locals.var_dmcgeff);
        let assign6320_e7542: f64 = (assign6320_e7536 / assign6320_e7541);
        (assign6320_e7542,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6320_e7544;

        let (assign6330_e7567,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && (!((locals.var_guard137 != 0.0) || (locals.var_guard138 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6330_e7567;

        let assign6340_e7570: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard142 = assign6340_e7570;

        let assign6350_e7573: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign6350_e7573;

        let assign6360_e7584: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard144 = assign6360_e7584;

        let assign6370_e7595: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard145 = assign6370_e7595;

        let assign6380_e7598: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard146 = assign6380_e7598;

    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6390_e7620,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && (locals.var_guard146 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6390_e7620;

        let (assign6400_e7649,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && (locals.var_guard146 == 0.0)) {
        let assign6400_e7643: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6400_e7646: f64 = (locals.var_weff * locals.var_nuends);
        let assign6400_e7647: f64 = (assign6400_e7643 / assign6400_e7646);
        (assign6400_e7647,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6400_e7649;

        let assign6420_e7659: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard148 = assign6420_e7659;

        let (assign6430_e7684,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && ((locals.var_guard145 != 0.0) && (locals.var_guard144 == 0.0))) && (locals.var_guard148 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6430_e7684;

        let (assign6440_e7718,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && ((locals.var_guard145 != 0.0) && (locals.var_guard144 == 0.0))) && (locals.var_guard148 == 0.0)) {
        let assign6440_e7710: f64 = (p.p374 * locals.var_weff);
        let assign6440_e7713: f64 = (6.0 * locals.var_nuends);
        let assign6440_e7715: f64 = (assign6440_e7713 * locals.var_dmcgeff);
        let assign6440_e7716: f64 = (assign6440_e7710 / assign6440_e7715);
        (assign6440_e7716,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6440_e7718;

        let (assign6450_e7741,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && (!((locals.var_guard144 != 0.0) || (locals.var_guard145 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6450_e7741;

        let assign6460_e7752: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard149 = assign6460_e7752;

        let assign6470_e7763: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard150 = assign6470_e7763;

        let assign6480_e7766: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard151 = assign6480_e7766;

        let (assign6490_e7789,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && (locals.var_guard149 != 0.0)) && (locals.var_guard151 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6490_e7789;

        let (assign6500_e7819,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && (locals.var_guard149 != 0.0)) && (locals.var_guard151 == 0.0)) {
        let assign6500_e7813: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6500_e7816: f64 = (locals.var_weff * locals.var_nuends);
        let assign6500_e7817: f64 = (assign6500_e7813 / assign6500_e7816);
        (assign6500_e7817,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6500_e7819;

        let assign6520_e7829: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard153 = assign6520_e7829;

        let (assign6530_e7855,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && ((locals.var_guard150 != 0.0) && (locals.var_guard149 == 0.0))) && (locals.var_guard153 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6530_e7855;

        let (assign6540_e7890,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && ((locals.var_guard150 != 0.0) && (locals.var_guard149 == 0.0))) && (locals.var_guard153 == 0.0)) {
        let assign6540_e7882: f64 = (p.p374 * locals.var_weff);
        let assign6540_e7885: f64 = (6.0 * locals.var_nuends);
        let assign6540_e7887: f64 = (assign6540_e7885 * locals.var_dmcgeff);
        let assign6540_e7888: f64 = (assign6540_e7882 / assign6540_e7887);
        (assign6540_e7888,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6540_e7890;

        let (assign6550_e7914,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && (!((locals.var_guard149 != 0.0) || (locals.var_guard150 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6550_e7914;

        let assign6560_e7917: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign6560_e7917;

        let assign6570_e7928: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard155 = assign6570_e7928;

        let assign6580_e7939: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard156 = assign6580_e7939;

        let assign6590_e7942: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign6590_e7942;

        let (assign6600_e7965,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard155 != 0.0)) && (locals.var_guard157 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6600_e7965;

        let (assign6610_e7995,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard155 != 0.0)) && (locals.var_guard157 == 0.0)) {
        let assign6610_e7989: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6610_e7992: f64 = (locals.var_weff * locals.var_nuendd);
        let assign6610_e7993: f64 = (assign6610_e7989 / assign6610_e7992);
        (assign6610_e7993,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6610_e7995;

        let assign6630_e8006: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6630_e8009: f64 = if ((locals.var_nuendd == 0.0) || (assign6630_e8006 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard159 = assign6630_e8009;

        let (assign6640_e8035,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && ((locals.var_guard156 != 0.0) && (locals.var_guard155 == 0.0))) && (locals.var_guard159 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6640_e8035;

        let (assign6650_e8072,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && ((locals.var_guard156 != 0.0) && (locals.var_guard155 == 0.0))) && (locals.var_guard159 == 0.0)) {
        let assign6650_e8062: f64 = (p.p374 * locals.var_weff);
        let assign6650_e8065: f64 = (3.0 * locals.var_nuendd);
        let assign6650_e8068: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6650_e8069: f64 = (assign6650_e8065 * assign6650_e8068);
        let assign6650_e8070: f64 = (assign6650_e8062 / assign6650_e8069);
        (assign6650_e8070,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6650_e8072;

        let (assign6660_e8096,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && (!((locals.var_guard155 != 0.0) || (locals.var_guard156 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6660_e8096;

        let assign6670_e8107: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard160 = assign6670_e8107;

        let assign6680_e8118: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard161 = assign6680_e8118;

        let assign6690_e8121: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign6690_e8121;

        let (assign6700_e8145,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard160 != 0.0)) && (locals.var_guard162 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6700_e8145;

        let (assign6710_e8176,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard160 != 0.0)) && (locals.var_guard162 == 0.0)) {
        let assign6710_e8170: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6710_e8173: f64 = (locals.var_weff * locals.var_nuendd);
        let assign6710_e8174: f64 = (assign6710_e8170 / assign6710_e8173);
        (assign6710_e8174,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6710_e8176;

        let assign6730_e8187: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6730_e8190: f64 = if ((locals.var_nuendd == 0.0) || (assign6730_e8187 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard164 = assign6730_e8190;

        let (assign6740_e8217,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && ((locals.var_guard161 != 0.0) && (locals.var_guard160 == 0.0))) && (locals.var_guard164 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6740_e8217;

        let (assign6750_e8255,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && ((locals.var_guard161 != 0.0) && (locals.var_guard160 == 0.0))) && (locals.var_guard164 == 0.0)) {
        let assign6750_e8245: f64 = (p.p374 * locals.var_weff);
        let assign6750_e8248: f64 = (3.0 * locals.var_nuendd);
        let assign6750_e8251: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6750_e8252: f64 = (assign6750_e8248 * assign6750_e8251);
        let assign6750_e8253: f64 = (assign6750_e8245 / assign6750_e8252);
        (assign6750_e8253,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6750_e8255;

        let (assign6760_e8280,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && (!((locals.var_guard160 != 0.0) || (locals.var_guard161 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6760_e8280;

        let assign6770_e8283: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard165 = assign6770_e8283;

        let assign6780_e8286: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard166 = assign6780_e8286;

        let assign6790_e8297: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard167 = assign6790_e8297;

        let assign6800_e8308: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard168 = assign6800_e8308;

        let assign6810_e8311: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard169 = assign6810_e8311;

        let (assign6820_e8335,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard169 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6820_e8335;

        let (assign6830_e8366,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard169 == 0.0)) {
        let assign6830_e8360: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6830_e8363: f64 = (locals.var_weff * locals.var_nuends);
        let assign6830_e8364: f64 = (assign6830_e8360 / assign6830_e8363);
        (assign6830_e8364,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6830_e8366;

        let assign6850_e8376: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard171 = assign6850_e8376;

        let (assign6860_e8403,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && ((locals.var_guard168 != 0.0) && (locals.var_guard167 == 0.0))) && (locals.var_guard171 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6860_e8403;

        let (assign6870_e8439,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && ((locals.var_guard168 != 0.0) && (locals.var_guard167 == 0.0))) && (locals.var_guard171 == 0.0)) {
        let assign6870_e8431: f64 = (p.p374 * locals.var_weff);
        let assign6870_e8434: f64 = (6.0 * locals.var_nuends);
        let assign6870_e8436: f64 = (assign6870_e8434 * locals.var_dmcgeff);
        let assign6870_e8437: f64 = (assign6870_e8431 / assign6870_e8436);
        (assign6870_e8437,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6870_e8439;

        let (assign6880_e8464,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && (!((locals.var_guard167 != 0.0) || (locals.var_guard168 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6880_e8464;

        let assign6890_e8475: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard172 = assign6890_e8475;

        let assign6900_e8486: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard173 = assign6900_e8486;

        let assign6910_e8489: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard174 = assign6910_e8489;

        let (assign6920_e8514,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard172 != 0.0)) && (locals.var_guard174 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6920_e8514;

        let (assign6930_e8546,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard172 != 0.0)) && (locals.var_guard174 == 0.0)) {
        let assign6930_e8540: f64 = (p.p374 * locals.var_dmcgeff);
        let assign6930_e8543: f64 = (locals.var_weff * locals.var_nuends);
        let assign6930_e8544: f64 = (assign6930_e8540 / assign6930_e8543);
        (assign6930_e8544,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6930_e8546;

        let assign6950_e8556: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard176 = assign6950_e8556;

        let (assign6960_e8584,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && ((locals.var_guard173 != 0.0) && (locals.var_guard172 == 0.0))) && (locals.var_guard176 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6960_e8584;

        let (assign6970_e8621,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && ((locals.var_guard173 != 0.0) && (locals.var_guard172 == 0.0))) && (locals.var_guard176 == 0.0)) {
        let assign6970_e8613: f64 = (p.p374 * locals.var_weff);
        let assign6970_e8616: f64 = (6.0 * locals.var_nuends);
        let assign6970_e8618: f64 = (assign6970_e8616 * locals.var_dmcgeff);
        let assign6970_e8619: f64 = (assign6970_e8613 / assign6970_e8618);
        (assign6970_e8619,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6970_e8621;

        let (assign6980_e8647,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && (!((locals.var_guard172 != 0.0) || (locals.var_guard173 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign6980_e8647;

        let assign6990_e8650: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard177 = assign6990_e8650;

        let assign7000_e8661: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard178 = assign7000_e8661;

        let assign7010_e8672: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard179 = assign7010_e8672;

        let assign7020_e8675: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard180 = assign7020_e8675;

        let (assign7030_e8700,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard178 != 0.0)) && (locals.var_guard180 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7030_e8700;

        let (assign7040_e8732,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard178 != 0.0)) && (locals.var_guard180 == 0.0)) {
        let assign7040_e8726: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7040_e8729: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7040_e8730: f64 = (assign7040_e8726 / assign7040_e8729);
        (assign7040_e8730,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7040_e8732;

        let assign7060_e8742: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard182 = assign7060_e8742;

        let (assign7070_e8770,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && ((locals.var_guard179 != 0.0) && (locals.var_guard178 == 0.0))) && (locals.var_guard182 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7070_e8770;

        let (assign7080_e8807,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && ((locals.var_guard179 != 0.0) && (locals.var_guard178 == 0.0))) && (locals.var_guard182 == 0.0)) {
        let assign7080_e8799: f64 = (p.p374 * locals.var_weff);
        let assign7080_e8802: f64 = (6.0 * locals.var_nuendd);
        let assign7080_e8804: f64 = (assign7080_e8802 * locals.var_dmcgeff);
        let assign7080_e8805: f64 = (assign7080_e8799 / assign7080_e8804);
        (assign7080_e8805,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7080_e8807;

        let (assign7090_e8833,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && (!((locals.var_guard178 != 0.0) || (locals.var_guard179 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7090_e8833;

        let assign7100_e8844: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard183 = assign7100_e8844;

        let assign7110_e8855: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard184 = assign7110_e8855;

        let assign7120_e8858: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard185 = assign7120_e8858;

        let (assign7130_e8884,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && (locals.var_guard183 != 0.0)) && (locals.var_guard185 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7130_e8884;

        let (assign7140_e8917,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && (locals.var_guard183 != 0.0)) && (locals.var_guard185 == 0.0)) {
        let assign7140_e8911: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7140_e8914: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7140_e8915: f64 = (assign7140_e8911 / assign7140_e8914);
        (assign7140_e8915,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7140_e8917;

        let assign7160_e8927: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard187 = assign7160_e8927;

        let (assign7170_e8956,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && ((locals.var_guard184 != 0.0) && (locals.var_guard183 == 0.0))) && (locals.var_guard187 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7170_e8956;

        let (assign7180_e8994,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && ((locals.var_guard184 != 0.0) && (locals.var_guard183 == 0.0))) && (locals.var_guard187 == 0.0)) {
        let assign7180_e8986: f64 = (p.p374 * locals.var_weff);
        let assign7180_e8989: f64 = (6.0 * locals.var_nuendd);
        let assign7180_e8991: f64 = (assign7180_e8989 * locals.var_dmcgeff);
        let assign7180_e8992: f64 = (assign7180_e8986 / assign7180_e8991);
        (assign7180_e8992,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7180_e8994;

    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7190_e9021,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && (!((locals.var_guard183 != 0.0) || (locals.var_guard184 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7190_e9021;

        let assign7200_e9024: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard188 = assign7200_e9024;

        let assign7210_e9027: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard189 = assign7210_e9027;

        let assign7220_e9038: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard190 = assign7220_e9038;

        let assign7230_e9049: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard191 = assign7230_e9049;

        let assign7240_e9052: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard192 = assign7240_e9052;

        let (assign7250_e9078,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && (locals.var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7250_e9078;

        let (assign7260_e9111,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && (locals.var_guard192 == 0.0)) {
        let assign7260_e9105: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7260_e9108: f64 = (locals.var_weff * locals.var_nuends);
        let assign7260_e9109: f64 = (assign7260_e9105 / assign7260_e9108);
        (assign7260_e9109,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7260_e9111;

        let assign7280_e9122: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7280_e9125: f64 = if ((locals.var_nuends == 0.0) || (assign7280_e9122 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard194 = assign7280_e9125;

        let (assign7290_e9154,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && ((locals.var_guard191 != 0.0) && (locals.var_guard190 == 0.0))) && (locals.var_guard194 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7290_e9154;

        let (assign7300_e9194,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && ((locals.var_guard191 != 0.0) && (locals.var_guard190 == 0.0))) && (locals.var_guard194 == 0.0)) {
        let assign7300_e9184: f64 = (p.p374 * locals.var_weff);
        let assign7300_e9187: f64 = (3.0 * locals.var_nuends);
        let assign7300_e9190: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7300_e9191: f64 = (assign7300_e9187 * assign7300_e9190);
        let assign7300_e9192: f64 = (assign7300_e9184 / assign7300_e9191);
        (assign7300_e9192,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7300_e9194;

        let (assign7310_e9221,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && (!((locals.var_guard190 != 0.0) || (locals.var_guard191 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7310_e9221;

        let assign7320_e9232: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard195 = assign7320_e9232;

        let assign7330_e9243: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard196 = assign7330_e9243;

        let assign7340_e9246: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard197 = assign7340_e9246;

        let (assign7350_e9273,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && (locals.var_guard195 != 0.0)) && (locals.var_guard197 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7350_e9273;

        let (assign7360_e9307,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && (locals.var_guard195 != 0.0)) && (locals.var_guard197 == 0.0)) {
        let assign7360_e9301: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7360_e9304: f64 = (locals.var_weff * locals.var_nuends);
        let assign7360_e9305: f64 = (assign7360_e9301 / assign7360_e9304);
        (assign7360_e9305,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7360_e9307;

        let assign7380_e9318: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7380_e9321: f64 = if ((locals.var_nuends == 0.0) || (assign7380_e9318 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard199 = assign7380_e9321;

        let (assign7390_e9351,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && ((locals.var_guard196 != 0.0) && (locals.var_guard195 == 0.0))) && (locals.var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7390_e9351;

        let (assign7400_e9392,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && ((locals.var_guard196 != 0.0) && (locals.var_guard195 == 0.0))) && (locals.var_guard199 == 0.0)) {
        let assign7400_e9382: f64 = (p.p374 * locals.var_weff);
        let assign7400_e9385: f64 = (3.0 * locals.var_nuends);
        let assign7400_e9388: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7400_e9389: f64 = (assign7400_e9385 * assign7400_e9388);
        let assign7400_e9390: f64 = (assign7400_e9382 / assign7400_e9389);
        (assign7400_e9390,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7400_e9392;

        let (assign7410_e9420,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && (!((locals.var_guard195 != 0.0) || (locals.var_guard196 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7410_e9420;

        let (assign7420_e9445,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 == 0.0)) {
        let assign7420_e9441: f64 = (p.p374 * locals.var_dmdgeff);
        let assign7420_e9443: f64 = (assign7420_e9441 / locals.var_weff);
        (assign7420_e9443,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7420_e9445;

        let assign7430_e9448: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard200 = assign7430_e9448;

        let assign7440_e9451: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard201 = assign7440_e9451;

        let assign7450_e9462: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard202 = assign7450_e9462;

        let assign7460_e9473: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard203 = assign7460_e9473;

        let assign7470_e9476: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard204 = assign7470_e9476;

        let (assign7480_e9504,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && (locals.var_guard204 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7480_e9504;

        let (assign7490_e9539,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && (locals.var_guard204 == 0.0)) {
        let assign7490_e9533: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7490_e9536: f64 = (locals.var_weff * locals.var_nuends);
        let assign7490_e9537: f64 = (assign7490_e9533 / assign7490_e9536);
        (assign7490_e9537,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7490_e9539;

        let assign7510_e9549: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard206 = assign7510_e9549;

        let (assign7520_e9580,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && ((locals.var_guard203 != 0.0) && (locals.var_guard202 == 0.0))) && (locals.var_guard206 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7520_e9580;

        let (assign7530_e9620,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && ((locals.var_guard203 != 0.0) && (locals.var_guard202 == 0.0))) && (locals.var_guard206 == 0.0)) {
        let assign7530_e9612: f64 = (p.p374 * locals.var_weff);
        let assign7530_e9615: f64 = (6.0 * locals.var_nuends);
        let assign7530_e9617: f64 = (assign7530_e9615 * locals.var_dmcgeff);
        let assign7530_e9618: f64 = (assign7530_e9612 / assign7530_e9617);
        (assign7530_e9618,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7530_e9620;

        let (assign7540_e9649,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && (!((locals.var_guard202 != 0.0) || (locals.var_guard203 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7540_e9649;

        let assign7550_e9660: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard207 = assign7550_e9660;

        let assign7560_e9671: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard208 = assign7560_e9671;

        let assign7570_e9674: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard209 = assign7570_e9674;

        let (assign7580_e9703,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard207 != 0.0)) && (locals.var_guard209 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7580_e9703;

        let (assign7590_e9739,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard207 != 0.0)) && (locals.var_guard209 == 0.0)) {
        let assign7590_e9733: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7590_e9736: f64 = (locals.var_weff * locals.var_nuends);
        let assign7590_e9737: f64 = (assign7590_e9733 / assign7590_e9736);
        (assign7590_e9737,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7590_e9739;

        let assign7610_e9749: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard211 = assign7610_e9749;

        let (assign7620_e9781,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && ((locals.var_guard208 != 0.0) && (locals.var_guard207 == 0.0))) && (locals.var_guard211 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7620_e9781;

        let (assign7630_e9822,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && ((locals.var_guard208 != 0.0) && (locals.var_guard207 == 0.0))) && (locals.var_guard211 == 0.0)) {
        let assign7630_e9814: f64 = (p.p374 * locals.var_weff);
        let assign7630_e9817: f64 = (6.0 * locals.var_nuends);
        let assign7630_e9819: f64 = (assign7630_e9817 * locals.var_dmcgeff);
        let assign7630_e9820: f64 = (assign7630_e9814 / assign7630_e9819);
        (assign7630_e9820,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7630_e9822;

        let (assign7640_e9852,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (!((locals.var_guard207 != 0.0) || (locals.var_guard208 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7640_e9852;

        let assign7650_e9855: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard212 = assign7650_e9855;

        let (assign7660_e9880,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 == 0.0)) && (locals.var_guard212 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7660_e9880;

        let (assign7670_e9912,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 == 0.0)) && (locals.var_guard212 == 0.0)) {
        let assign7670_e9906: f64 = (p.p374 * locals.var_dmdgeff);
        let assign7670_e9909: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7670_e9910: f64 = (assign7670_e9906 / assign7670_e9909);
        (assign7670_e9910,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7670_e9912;

        let assign7680_e9915: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard213 = assign7680_e9915;

        let (assign7690_e9943,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 != 0.0)) {
        let assign7690_e9939: f64 = (p.p374 * locals.var_dmdgeff);
        let assign7690_e9941: f64 = (assign7690_e9939 / locals.var_weff);
        (assign7690_e9941,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7690_e9943;

        let assign7700_e9946: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard214 = assign7700_e9946;

        let assign7710_e9957: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard215 = assign7710_e9957;

        let assign7720_e9968: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard216 = assign7720_e9968;

        let assign7730_e9971: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard217 = assign7730_e9971;

        let (assign7740_e10002,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && (locals.var_guard215 != 0.0)) && (locals.var_guard217 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7740_e10002;

        let (assign7750_e10040,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && (locals.var_guard215 != 0.0)) && (locals.var_guard217 == 0.0)) {
        let assign7750_e10034: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7750_e10037: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7750_e10038: f64 = (assign7750_e10034 / assign7750_e10037);
        (assign7750_e10038,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7750_e10040;

        let assign7770_e10051: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7770_e10054: f64 = if ((locals.var_nuendd == 0.0) || (assign7770_e10051 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard219 = assign7770_e10054;

        let (assign7780_e10088,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && ((locals.var_guard216 != 0.0) && (locals.var_guard215 == 0.0))) && (locals.var_guard219 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7780_e10088;

        let (assign7790_e10133,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && ((locals.var_guard216 != 0.0) && (locals.var_guard215 == 0.0))) && (locals.var_guard219 == 0.0)) {
        let assign7790_e10123: f64 = (p.p374 * locals.var_weff);
        let assign7790_e10126: f64 = (3.0 * locals.var_nuendd);
        let assign7790_e10129: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7790_e10130: f64 = (assign7790_e10126 * assign7790_e10129);
        let assign7790_e10131: f64 = (assign7790_e10123 / assign7790_e10130);
        (assign7790_e10131,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7790_e10133;

        let (assign7800_e10165,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && (!((locals.var_guard215 != 0.0) || (locals.var_guard216 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7800_e10165;

        let assign7810_e10176: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard220 = assign7810_e10176;

        let assign7820_e10187: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard221 = assign7820_e10187;

        let assign7830_e10190: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard222 = assign7830_e10190;

        let (assign7840_e10222,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && (locals.var_guard220 != 0.0)) && (locals.var_guard222 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7840_e10222;

        let (assign7850_e10261,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && (locals.var_guard220 != 0.0)) && (locals.var_guard222 == 0.0)) {
        let assign7850_e10255: f64 = (p.p374 * locals.var_dmcgeff);
        let assign7850_e10258: f64 = (locals.var_weff * locals.var_nuendd);
        let assign7850_e10259: f64 = (assign7850_e10255 / assign7850_e10258);
        (assign7850_e10259,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7850_e10261;

        let assign7870_e10272: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7870_e10275: f64 = if ((locals.var_nuendd == 0.0) || (assign7870_e10272 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard224 = assign7870_e10275;

        let (assign7880_e10310,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && ((locals.var_guard221 != 0.0) && (locals.var_guard220 == 0.0))) && (locals.var_guard224 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7880_e10310;

        let (assign7890_e10356,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && ((locals.var_guard221 != 0.0) && (locals.var_guard220 == 0.0))) && (locals.var_guard224 == 0.0)) {
        let assign7890_e10346: f64 = (p.p374 * locals.var_weff);
        let assign7890_e10349: f64 = (3.0 * locals.var_nuendd);
        let assign7890_e10352: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7890_e10353: f64 = (assign7890_e10349 * assign7890_e10352);
        let assign7890_e10354: f64 = (assign7890_e10346 / assign7890_e10353);
        (assign7890_e10354,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7890_e10356;

        let (assign7900_e10389,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && (!((locals.var_guard220 != 0.0) || (locals.var_guard221 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7900_e10389;

        let assign7910_e10392: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard225 = assign7910_e10392;

        let assign7920_e10395: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard226 = assign7920_e10395;

        let (assign7930_e10423,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 != 0.0)) && (locals.var_guard226 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7930_e10423;

        let (assign7940_e10458,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 != 0.0)) && (locals.var_guard226 == 0.0)) {
        let assign7940_e10452: f64 = (p.p374 * locals.var_dmdgeff);
        let assign7940_e10455: f64 = (locals.var_weff * locals.var_nuends);
        let assign7940_e10456: f64 = (assign7940_e10452 / assign7940_e10455);
        (assign7940_e10456,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7940_e10458;

        let assign7950_e10461: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard227 = assign7950_e10461;

        let assign7960_e10472: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard228 = assign7960_e10472;

        let assign7970_e10483: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard229 = assign7970_e10483;

        let assign7980_e10486: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard230 = assign7980_e10486;

    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign7990_e10519,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && (locals.var_guard228 != 0.0)) && (locals.var_guard230 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign7990_e10519;

        let (assign8000_e10559,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && (locals.var_guard228 != 0.0)) && (locals.var_guard230 == 0.0)) {
        let assign8000_e10553: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8000_e10556: f64 = (locals.var_weff * locals.var_nuendd);
        let assign8000_e10557: f64 = (assign8000_e10553 / assign8000_e10556);
        (assign8000_e10557,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8000_e10559;

        let assign8020_e10569: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard232 = assign8020_e10569;

        let (assign8030_e10605,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && ((locals.var_guard229 != 0.0) && (locals.var_guard228 == 0.0))) && (locals.var_guard232 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8030_e10605;

        let (assign8040_e10650,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && ((locals.var_guard229 != 0.0) && (locals.var_guard228 == 0.0))) && (locals.var_guard232 == 0.0)) {
        let assign8040_e10642: f64 = (p.p374 * locals.var_weff);
        let assign8040_e10645: f64 = (6.0 * locals.var_nuendd);
        let assign8040_e10647: f64 = (assign8040_e10645 * locals.var_dmcgeff);
        let assign8040_e10648: f64 = (assign8040_e10642 / assign8040_e10647);
        (assign8040_e10648,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8040_e10650;

        let (assign8050_e10684,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && (!((locals.var_guard228 != 0.0) || (locals.var_guard229 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8050_e10684;

        let assign8060_e10695: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard233 = assign8060_e10695;

        let assign8070_e10706: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard234 = assign8070_e10706;

        let assign8080_e10709: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard235 = assign8080_e10709;

        let (assign8090_e10743,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && (locals.var_guard233 != 0.0)) && (locals.var_guard235 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8090_e10743;

        let (assign8100_e10784,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && (locals.var_guard233 != 0.0)) && (locals.var_guard235 == 0.0)) {
        let assign8100_e10778: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8100_e10781: f64 = (locals.var_weff * locals.var_nuendd);
        let assign8100_e10782: f64 = (assign8100_e10778 / assign8100_e10781);
        (assign8100_e10782,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8100_e10784;

        let assign8120_e10794: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard237 = assign8120_e10794;

        let (assign8130_e10831,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && ((locals.var_guard234 != 0.0) && (locals.var_guard233 == 0.0))) && (locals.var_guard237 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8130_e10831;

        let (assign8140_e10877,) = {
    if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && ((locals.var_guard234 != 0.0) && (locals.var_guard233 == 0.0))) && (locals.var_guard237 == 0.0)) {
        let assign8140_e10869: f64 = (p.p374 * locals.var_weff);
        let assign8140_e10872: f64 = (6.0 * locals.var_nuendd);
        let assign8140_e10874: f64 = (assign8140_e10872 * locals.var_dmcgeff);
        let assign8140_e10875: f64 = (assign8140_e10869 / assign8140_e10874);
        (assign8140_e10875,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8140_e10877;

        let (assign8150_e10912,) = {
    if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && (!((locals.var_guard233 != 0.0) || (locals.var_guard234 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8150_e10912;

        let (assign8160_e10942,) = {
    if (((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard93 != 0.0) && (!((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) {
        let assign8160_e10938: f64 = (p.p374 * locals.var_dmdgeff);
        let assign8160_e10940: f64 = (assign8160_e10938 / locals.var_weff);
        (assign8160_e10940,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8160_e10942;

        let assign8170_e10945: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard238 = assign8170_e10945;

        let (assign8180_e10981,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 != 0.0)) {
        let assign8180_e10975: f64 = (0.5 * p.p374);
        let assign8180_e10977: f64 = (assign8180_e10975 * locals.var_dmcgeff);
        let assign8180_e10979: f64 = (assign8180_e10977 / locals.var_weff);
        (assign8180_e10979,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8180_e10981;

        let assign8190_e10984: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard239 = assign8190_e10984;

        let (assign8200_e11016,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 != 0.0)) && (locals.var_guard239 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8200_e11016;

        let (assign8210_e11057,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 != 0.0)) && (locals.var_guard239 == 0.0)) {
        let assign8210_e11049: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8210_e11053: f64 = (p.p2 - 2.0);
        let assign8210_e11054: f64 = (locals.var_weff * assign8210_e11053);
        let assign8210_e11055: f64 = (assign8210_e11049 / assign8210_e11054);
        (assign8210_e11055,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8210_e11057;

        let (assign8220_e11088,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8220_e11088;

        let (assign8230_e11125,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 == 0.0)) {
        let assign8230_e11119: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8230_e11122: f64 = (locals.var_weff * p.p2);
        let assign8230_e11123: f64 = (assign8230_e11119 / assign8230_e11122);
        (assign8230_e11123,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8230_e11125;

        let assign8240_e11128: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard240 = assign8240_e11128;

        let (assign8250_e11160,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8250_e11160;

        let (assign8260_e11198,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 != 0.0)) {
        let assign8260_e11192: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8260_e11195: f64 = (locals.var_weff * p.p2);
        let assign8260_e11196: f64 = (assign8260_e11192 / assign8260_e11195);
        (assign8260_e11196,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8260_e11198;

        let (assign8270_e11237,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 == 0.0)) {
        let assign8270_e11231: f64 = (0.5 * p.p374);
        let assign8270_e11233: f64 = (assign8270_e11231 * locals.var_dmcgeff);
        let assign8270_e11235: f64 = (assign8270_e11233 / locals.var_weff);
        (assign8270_e11235,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8270_e11237;

        let assign8280_e11240: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard241 = assign8280_e11240;

        let (assign8290_e11275,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 == 0.0)) && (locals.var_guard241 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8290_e11275;

        let (assign8300_e11319,) = {
    if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 == 0.0)) && (locals.var_guard241 == 0.0)) {
        let assign8300_e11311: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8300_e11315: f64 = (p.p2 - 2.0);
        let assign8300_e11316: f64 = (locals.var_weff * assign8300_e11315);
        let assign8300_e11317: f64 = (assign8300_e11311 / assign8300_e11316);
        (assign8300_e11317,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8300_e11319;

        let (assign8310_e11349,) = {
    if (((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (!(((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8310_e11349;

        let assign8320_e11352: f64 = if locals.var_rint <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard242 = assign8320_e11352;

        let (assign8330_e11361,) = {
    if (((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard242 != 0.0)) {
        (locals.var_rend,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign8330_e11361;

        let assign8340_e11364: f64 = if locals.var_rend <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard243 = assign8340_e11364;

        let (assign8350_e11376,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard242 == 0.0)) && (locals.var_guard243 != 0.0)) {
        (locals.var_rint,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign8350_e11376;

        let (assign8360_e11395,) = {
    if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard242 == 0.0)) && (locals.var_guard243 == 0.0)) {
        let assign8360_e11389: f64 = (locals.var_rint * locals.var_rend);
        let assign8360_e11392: f64 = (locals.var_rint + locals.var_rend);
        let assign8360_e11393: f64 = (assign8360_e11389 / assign8360_e11392);
        (assign8360_e11393,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign8360_e11395;

        let (assign8380_e11406,) = {
    if ((locals.var_guard77 == 0.0) && (locals.var_guard78 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rsourcegeo,)
    }
};
        locals.var_rsourcegeo = assign8380_e11406;

        let assign8390_e11408: f64 = if param_given[4] { 1.0 } else { 0.0 };
        locals.var_guard245 = assign8390_e11408;

        let (assign8400_e11414,) = {
    if (locals.var_guard245 != 0.0) {
        let assign8400_e11412: f64 = (p.p374 * p.p4);
        (assign8400_e11412,)
    } else {
        (locals.var_rdraingeo,)
    }
};
        locals.var_rdraingeo = assign8400_e11414;

        let assign8410_e11421: f64 = if ((p.p10 > 0.0) && (p.p374 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard246 = assign8410_e11421;

        let assign8420_e11424: f64 = if p.p9 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard247 = assign8420_e11424;

        let assign8430_e11427: f64 = (p.p2 % 2.0);
        let assign8430_e11429: f64 = if assign8430_e11427 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard248 = assign8430_e11429;

        let (assign8440_e11440,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign8440_e11440;

        let (assign8450_e11451,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign8450_e11451;

        let (assign8460_e11470,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) {
        let assign8460_e11463: f64 = (p.p2 - 1.0);
        let assign8460_e11465: f64 = (assign8460_e11463 / 2.0);
        let assign8460_e11467: f64 = (assign8460_e11465).max(0.0);
        let assign8460_e11468: f64 = (2.0 * assign8460_e11467);
        (assign8460_e11468,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign8460_e11470;

        let (assign8470_e11481,) = {
    if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) {
        (locals.var_nuintd,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign8470_e11481;

        let assign8480_e11484: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign8480_e11484;

        let (assign8490_e11498,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign8490_e11498;

        let (assign8500_e11520,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 != 0.0)) {
        let assign8500_e11513: f64 = (p.p2 / 2.0);
        let assign8500_e11515: f64 = (assign8500_e11513 - 1.0);
        let assign8500_e11517: f64 = (assign8500_e11515).max(0.0);
        let assign8500_e11518: f64 = (2.0 * assign8500_e11517);
        (assign8500_e11518,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign8500_e11520;

        let (assign8510_e11534,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign8510_e11534;

        let (assign8520_e11548,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 != 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign8520_e11548;

        let (assign8530_e11563,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign8530_e11563;

        let (assign8540_e11578,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 == 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign8540_e11578;

        let (assign8550_e11593,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign8550_e11593;

        let (assign8560_e11616,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 == 0.0)) {
        let assign8560_e11609: f64 = (p.p2 / 2.0);
        let assign8560_e11611: f64 = (assign8560_e11609 - 1.0);
        let assign8560_e11613: f64 = (assign8560_e11611).max(0.0);
        let assign8560_e11614: f64 = (2.0 * assign8560_e11613);
        (assign8560_e11614,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign8560_e11616;

        let assign8570_e11619: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard250 = assign8570_e11619;

        let assign8580_e11622: f64 = if locals.var_nuints == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign8580_e11622;

        let (assign8590_e11635,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 != 0.0)) && (locals.var_guard251 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8590_e11635;

        let (assign8600_e11655,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 != 0.0)) && (locals.var_guard251 == 0.0)) {
        let assign8600_e11649: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8600_e11652: f64 = (locals.var_weff * locals.var_nuints);
        let assign8600_e11653: f64 = (assign8600_e11649 / assign8600_e11652);
        (assign8600_e11653,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8600_e11655;

        let assign8610_e11658: f64 = if locals.var_nuintd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard252 = assign8610_e11658;

        let (assign8620_e11672,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 == 0.0)) && (locals.var_guard252 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8620_e11672;

        let (assign8630_e11693,) = {
    if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 == 0.0)) && (locals.var_guard252 == 0.0)) {
        let assign8630_e11687: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8630_e11690: f64 = (locals.var_weff * locals.var_nuintd);
        let assign8630_e11691: f64 = (assign8630_e11687 / assign8630_e11690);
        (assign8630_e11691,)
    } else {
        (locals.var_rint,)
    }
};
        locals.var_rint = assign8630_e11693;

        let assign8640_e11696: f64 = if p.p9 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard253 = assign8640_e11696;

        let assign8650_e11699: f64 = if p.p9 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard254 = assign8650_e11699;

        let assign8660_e11702: f64 = if p.p9 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard255 = assign8660_e11702;

    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign8670_e11705: f64 = if p.p9 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard256 = assign8670_e11705;

        let assign8680_e11708: f64 = if p.p9 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign8680_e11708;

        let assign8690_e11711: f64 = if p.p9 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard258 = assign8690_e11711;

        let assign8700_e11714: f64 = if p.p9 == 6.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign8700_e11714;

        let assign8710_e11717: f64 = if p.p9 == 7.0 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign8710_e11717;

        let assign8720_e11720: f64 = if p.p9 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard261 = assign8720_e11720;

        let assign8730_e11723: f64 = if p.p9 == 9.0 { 1.0 } else { 0.0 };
        locals.var_guard262 = assign8730_e11723;

        let assign8740_e11726: f64 = if p.p9 == 10.0 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign8740_e11726;

        let assign8750_e11729: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign8750_e11729;

        let assign8760_e11732: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard265 = assign8760_e11732;

        let assign8770_e11743: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard266 = assign8770_e11743;

        let assign8780_e11754: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard267 = assign8780_e11754;

        let assign8790_e11757: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard268 = assign8790_e11757;

        let (assign8800_e11774,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard268 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8800_e11774;

        let (assign8810_e11798,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard268 == 0.0)) {
        let assign8810_e11792: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8810_e11795: f64 = (locals.var_weff * locals.var_nuends);
        let assign8810_e11796: f64 = (assign8810_e11792 / assign8810_e11795);
        (assign8810_e11796,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8810_e11798;

        let assign8830_e11809: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8830_e11812: f64 = if ((locals.var_nuends == 0.0) || (assign8830_e11809 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard270 = assign8830_e11812;

        let (assign8840_e11832,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && ((locals.var_guard267 != 0.0) && (locals.var_guard266 == 0.0))) && (locals.var_guard270 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8840_e11832;

        let (assign8850_e11863,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && ((locals.var_guard267 != 0.0) && (locals.var_guard266 == 0.0))) && (locals.var_guard270 == 0.0)) {
        let assign8850_e11853: f64 = (p.p374 * locals.var_weff);
        let assign8850_e11856: f64 = (3.0 * locals.var_nuends);
        let assign8850_e11859: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8850_e11860: f64 = (assign8850_e11856 * assign8850_e11859);
        let assign8850_e11861: f64 = (assign8850_e11853 / assign8850_e11860);
        (assign8850_e11861,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8850_e11863;

        let (assign8860_e11881,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && (!((locals.var_guard266 != 0.0) || (locals.var_guard267 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8860_e11881;

        let assign8870_e11892: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard271 = assign8870_e11892;

        let assign8880_e11903: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard272 = assign8880_e11903;

        let assign8890_e11906: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign8890_e11906;

        let (assign8900_e11924,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard271 != 0.0)) && (locals.var_guard273 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8900_e11924;

        let (assign8910_e11949,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard271 != 0.0)) && (locals.var_guard273 == 0.0)) {
        let assign8910_e11943: f64 = (p.p374 * locals.var_dmcgeff);
        let assign8910_e11946: f64 = (locals.var_weff * locals.var_nuends);
        let assign8910_e11947: f64 = (assign8910_e11943 / assign8910_e11946);
        (assign8910_e11947,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8910_e11949;

        let assign8930_e11960: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8930_e11963: f64 = if ((locals.var_nuends == 0.0) || (assign8930_e11960 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard275 = assign8930_e11963;

        let (assign8940_e11984,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && ((locals.var_guard272 != 0.0) && (locals.var_guard271 == 0.0))) && (locals.var_guard275 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8940_e11984;

        let (assign8950_e12016,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && ((locals.var_guard272 != 0.0) && (locals.var_guard271 == 0.0))) && (locals.var_guard275 == 0.0)) {
        let assign8950_e12006: f64 = (p.p374 * locals.var_weff);
        let assign8950_e12009: f64 = (3.0 * locals.var_nuends);
        let assign8950_e12012: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8950_e12013: f64 = (assign8950_e12009 * assign8950_e12012);
        let assign8950_e12014: f64 = (assign8950_e12006 / assign8950_e12013);
        (assign8950_e12014,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8950_e12016;

        let (assign8960_e12035,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && (!((locals.var_guard271 != 0.0) || (locals.var_guard272 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign8960_e12035;

        let assign8970_e12038: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard276 = assign8970_e12038;

        let assign8980_e12049: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard277 = assign8980_e12049;

        let assign8990_e12060: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard278 = assign8990_e12060;

        let assign9000_e12063: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign9000_e12063;

        let (assign9010_e12081,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) && (locals.var_guard279 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9010_e12081;

        let (assign9020_e12106,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) && (locals.var_guard279 == 0.0)) {
        let assign9020_e12100: f64 = (p.p374 * locals.var_dmcgeff);
        let assign9020_e12103: f64 = (locals.var_weff * locals.var_nuendd);
        let assign9020_e12104: f64 = (assign9020_e12100 / assign9020_e12103);
        (assign9020_e12104,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9020_e12106;

        let assign9040_e12117: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9040_e12120: f64 = if ((locals.var_nuendd == 0.0) || (assign9040_e12117 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard281 = assign9040_e12120;

        let (assign9050_e12141,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && ((locals.var_guard278 != 0.0) && (locals.var_guard277 == 0.0))) && (locals.var_guard281 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9050_e12141;

        let (assign9060_e12173,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && ((locals.var_guard278 != 0.0) && (locals.var_guard277 == 0.0))) && (locals.var_guard281 == 0.0)) {
        let assign9060_e12163: f64 = (p.p374 * locals.var_weff);
        let assign9060_e12166: f64 = (3.0 * locals.var_nuendd);
        let assign9060_e12169: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9060_e12170: f64 = (assign9060_e12166 * assign9060_e12169);
        let assign9060_e12171: f64 = (assign9060_e12163 / assign9060_e12170);
        (assign9060_e12171,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9060_e12173;

        let (assign9070_e12192,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && (!((locals.var_guard277 != 0.0) || (locals.var_guard278 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9070_e12192;

        let assign9080_e12203: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard282 = assign9080_e12203;

        let assign9090_e12214: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard283 = assign9090_e12214;

        let assign9100_e12217: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard284 = assign9100_e12217;

        let (assign9110_e12236,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && (locals.var_guard282 != 0.0)) && (locals.var_guard284 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9110_e12236;

        let (assign9120_e12262,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && (locals.var_guard282 != 0.0)) && (locals.var_guard284 == 0.0)) {
        let assign9120_e12256: f64 = (p.p374 * locals.var_dmcgeff);
        let assign9120_e12259: f64 = (locals.var_weff * locals.var_nuendd);
        let assign9120_e12260: f64 = (assign9120_e12256 / assign9120_e12259);
        (assign9120_e12260,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9120_e12262;

        let assign9140_e12273: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9140_e12276: f64 = if ((locals.var_nuendd == 0.0) || (assign9140_e12273 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard286 = assign9140_e12276;

        let (assign9150_e12298,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && ((locals.var_guard283 != 0.0) && (locals.var_guard282 == 0.0))) && (locals.var_guard286 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9150_e12298;

        let (assign9160_e12331,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && ((locals.var_guard283 != 0.0) && (locals.var_guard282 == 0.0))) && (locals.var_guard286 == 0.0)) {
        let assign9160_e12321: f64 = (p.p374 * locals.var_weff);
        let assign9160_e12324: f64 = (3.0 * locals.var_nuendd);
        let assign9160_e12327: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9160_e12328: f64 = (assign9160_e12324 * assign9160_e12327);
        let assign9160_e12329: f64 = (assign9160_e12321 / assign9160_e12328);
        (assign9160_e12329,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9160_e12331;

        let (assign9170_e12351,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && (!((locals.var_guard282 != 0.0) || (locals.var_guard283 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9170_e12351;

        let assign9180_e12354: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard287 = assign9180_e12354;

        let assign9190_e12357: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard288 = assign9190_e12357;

        let assign9200_e12368: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard289 = assign9200_e12368;

        let assign9210_e12379: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard290 = assign9210_e12379;

        let assign9220_e12382: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard291 = assign9220_e12382;

        let (assign9230_e12402,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (locals.var_guard291 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9230_e12402;

        let (assign9240_e12429,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (locals.var_guard291 == 0.0)) {
        let assign9240_e12423: f64 = (p.p374 * locals.var_dmcgeff);
        let assign9240_e12426: f64 = (locals.var_weff * locals.var_nuends);
        let assign9240_e12427: f64 = (assign9240_e12423 / assign9240_e12426);
        (assign9240_e12427,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9240_e12429;

        let assign9260_e12440: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9260_e12443: f64 = if ((locals.var_nuends == 0.0) || (assign9260_e12440 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard293 = assign9260_e12443;

        let (assign9270_e12466,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && ((locals.var_guard290 != 0.0) && (locals.var_guard289 == 0.0))) && (locals.var_guard293 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9270_e12466;

        let (assign9280_e12500,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && ((locals.var_guard290 != 0.0) && (locals.var_guard289 == 0.0))) && (locals.var_guard293 == 0.0)) {
        let assign9280_e12490: f64 = (p.p374 * locals.var_weff);
        let assign9280_e12493: f64 = (3.0 * locals.var_nuends);
        let assign9280_e12496: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9280_e12497: f64 = (assign9280_e12493 * assign9280_e12496);
        let assign9280_e12498: f64 = (assign9280_e12490 / assign9280_e12497);
        (assign9280_e12498,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9280_e12500;

        let (assign9290_e12521,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && (!((locals.var_guard289 != 0.0) || (locals.var_guard290 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9290_e12521;

        let assign9300_e12532: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard294 = assign9300_e12532;

        let assign9310_e12543: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard295 = assign9310_e12543;

        let assign9320_e12546: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard296 = assign9320_e12546;

        let (assign9330_e12567,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard294 != 0.0)) && (locals.var_guard296 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9330_e12567;

        let (assign9340_e12595,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard294 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign9340_e12589: f64 = (p.p374 * locals.var_dmcgeff);
        let assign9340_e12592: f64 = (locals.var_weff * locals.var_nuends);
        let assign9340_e12593: f64 = (assign9340_e12589 / assign9340_e12592);
        (assign9340_e12593,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9340_e12595;

        let assign9360_e12606: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9360_e12609: f64 = if ((locals.var_nuends == 0.0) || (assign9360_e12606 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard298 = assign9360_e12609;

        let (assign9370_e12633,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && ((locals.var_guard295 != 0.0) && (locals.var_guard294 == 0.0))) && (locals.var_guard298 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9370_e12633;

        let (assign9380_e12668,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && ((locals.var_guard295 != 0.0) && (locals.var_guard294 == 0.0))) && (locals.var_guard298 == 0.0)) {
        let assign9380_e12658: f64 = (p.p374 * locals.var_weff);
        let assign9380_e12661: f64 = (3.0 * locals.var_nuends);
        let assign9380_e12664: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9380_e12665: f64 = (assign9380_e12661 * assign9380_e12664);
        let assign9380_e12666: f64 = (assign9380_e12658 / assign9380_e12665);
        (assign9380_e12666,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9380_e12668;

        let (assign9390_e12690,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && (!((locals.var_guard294 != 0.0) || (locals.var_guard295 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9390_e12690;

        let assign9400_e12693: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard299 = assign9400_e12693;

        let assign9410_e12704: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard300 = assign9410_e12704;

        let assign9420_e12715: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard301 = assign9420_e12715;

        let assign9430_e12718: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard302 = assign9430_e12718;

        let (assign9440_e12739,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && (locals.var_guard300 != 0.0)) && (locals.var_guard302 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9440_e12739;

        let (assign9450_e12767,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && (locals.var_guard300 != 0.0)) && (locals.var_guard302 == 0.0)) {
        let assign9450_e12761: f64 = (p.p374 * locals.var_dmcgeff);
        let assign9450_e12764: f64 = (locals.var_weff * locals.var_nuendd);
        let assign9450_e12765: f64 = (assign9450_e12761 / assign9450_e12764);
        (assign9450_e12765,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9450_e12767;

        let assign9470_e12777: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard304 = assign9470_e12777;

        let (assign9480_e12801,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && ((locals.var_guard301 != 0.0) && (locals.var_guard300 == 0.0))) && (locals.var_guard304 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9480_e12801;

        let (assign9490_e12834,) = {
    if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && ((locals.var_guard301 != 0.0) && (locals.var_guard300 == 0.0))) && (locals.var_guard304 == 0.0)) {
        let assign9490_e12826: f64 = (p.p374 * locals.var_weff);
        let assign9490_e12829: f64 = (6.0 * locals.var_nuendd);
        let assign9490_e12831: f64 = (assign9490_e12829 * locals.var_dmcgeff);
        let assign9490_e12832: f64 = (assign9490_e12826 / assign9490_e12831);
        (assign9490_e12832,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9490_e12834;

        let (assign9500_e12856,) = {
    if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && (!((locals.var_guard300 != 0.0) || (locals.var_guard301 != 0.0)))) {
        (0.0,)
    } else {
        (locals.var_rend,)
    }
};
        locals.var_rend = assign9500_e12856;

        let assign9510_e12867: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard305 = assign9510_e12867;

        let assign9520_e12878: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard306 = assign9520_e12878;

        let assign9530_e12881: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard307 = assign9530_e12881;

    }
}
