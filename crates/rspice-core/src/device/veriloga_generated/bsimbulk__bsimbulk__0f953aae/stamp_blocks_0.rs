#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        (locals.var_cdscdr_i, locals.var_cdscdr_i_dn0, locals.var_cdscdr_i_dn2, locals.var_cdscdr_i_dn3, locals.var_cdscdr_i_dn4, locals.var_cdscdr_i_dn5, locals.var_cdscdr_i_dn6, locals.var_cdscdr_i_dn7, locals.var_cdscdr_i_dn8, locals.var_cdscdr_i_dn9, locals.var_cdscdr_i_dn10, locals.var_cdscdr_i_dn11, locals.var_cdscdr_i_dn12, locals.var_cdscdr_i_dn13, locals.var_cdscdr_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        locals.var_l_wln1 = 0.0;

        (locals.var_ptwgr_i, locals.var_ptwgr_i_dn0, locals.var_ptwgr_i_dn2, locals.var_ptwgr_i_dn3, locals.var_ptwgr_i_dn4, locals.var_ptwgr_i_dn5, locals.var_ptwgr_i_dn6, locals.var_ptwgr_i_dn7, locals.var_ptwgr_i_dn8, locals.var_ptwgr_i_dn9, locals.var_ptwgr_i_dn10, locals.var_ptwgr_i_dn11, locals.var_ptwgr_i_dn12, locals.var_ptwgr_i_dn13, locals.var_ptwgr_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_uar_i, locals.var_uar_i_dn0, locals.var_uar_i_dn2, locals.var_uar_i_dn3, locals.var_uar_i_dn4, locals.var_uar_i_dn5, locals.var_uar_i_dn6, locals.var_uar_i_dn7, locals.var_uar_i_dn8, locals.var_uar_i_dn9, locals.var_uar_i_dn10, locals.var_uar_i_dn11, locals.var_uar_i_dn12, locals.var_uar_i_dn13, locals.var_uar_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        locals.var_ucsr_i = 0.0;

        (locals.var_ud_a, locals.var_ud_a_dn0, locals.var_ud_a_dn2, locals.var_ud_a_dn3, locals.var_ud_a_dn4, locals.var_ud_a_dn5, locals.var_ud_a_dn6, locals.var_ud_a_dn7, locals.var_ud_a_dn8, locals.var_ud_a_dn9, locals.var_ud_a_dn10, locals.var_ud_a_dn11, locals.var_ud_a_dn12, locals.var_ud_a_dn13, locals.var_ud_a_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        locals.var_w_wwn1 = 0.0;

        (locals.var_inv_sa, locals.var_inv_sa_dn0, locals.var_inv_sa_dn2, locals.var_inv_sa_dn3, locals.var_inv_sa_dn4, locals.var_inv_sa_dn5, locals.var_inv_sa_dn6, locals.var_inv_sa_dn7, locals.var_inv_sa_dn8, locals.var_inv_sa_dn9, locals.var_inv_sa_dn10, locals.var_inv_sa_dn11, locals.var_inv_sa_dn12, locals.var_inv_sa_dn13, locals.var_inv_sa_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_eta_stress, locals.var_eta_stress_dn0, locals.var_eta_stress_dn2, locals.var_eta_stress_dn3, locals.var_eta_stress_dn4, locals.var_eta_stress_dn5, locals.var_eta_stress_dn6, locals.var_eta_stress_dn7, locals.var_eta_stress_dn8, locals.var_eta_stress_dn9, locals.var_eta_stress_dn10, locals.var_eta_stress_dn11, locals.var_eta_stress_dn12, locals.var_eta_stress_dn13, locals.var_eta_stress_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_local_sca, locals.var_local_sca_dn0, locals.var_local_sca_dn2, locals.var_local_sca_dn3, locals.var_local_sca_dn4, locals.var_local_sca_dn5, locals.var_local_sca_dn6, locals.var_local_sca_dn7, locals.var_local_sca_dn8, locals.var_local_sca_dn9, locals.var_local_sca_dn10, locals.var_local_sca_dn11, locals.var_local_sca_dn12, locals.var_local_sca_dn13, locals.var_local_sca_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        locals.var_m0_i = 0.0;

        (locals.var_m0_t, locals.var_m0_t_dn4, ) = (0.0, 0.0, );

        (locals.var_eta0edge_i, locals.var_eta0edge_i_dn0, locals.var_eta0edge_i_dn2, locals.var_eta0edge_i_dn3, locals.var_eta0edge_i_dn4, locals.var_eta0edge_i_dn5, locals.var_eta0edge_i_dn6, locals.var_eta0edge_i_dn7, locals.var_eta0edge_i_dn8, locals.var_eta0edge_i_dn9, locals.var_eta0edge_i_dn10, locals.var_eta0edge_i_dn11, locals.var_eta0edge_i_dn12, locals.var_eta0edge_i_dn13, locals.var_eta0edge_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        locals.var_kt2edge_i = 0.0;

        (locals.var_k2edge_i, locals.var_k2edge_i_dn0, locals.var_k2edge_i_dn2, locals.var_k2edge_i_dn3, locals.var_k2edge_i_dn4, locals.var_k2edge_i_dn5, locals.var_k2edge_i_dn6, locals.var_k2edge_i_dn7, locals.var_k2edge_i_dn8, locals.var_k2edge_i_dn9, locals.var_k2edge_i_dn10, locals.var_k2edge_i_dn11, locals.var_k2edge_i_dn12, locals.var_k2edge_i_dn13, locals.var_k2edge_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_mnud1, locals.var_mnud1_dn0, locals.var_mnud1_dn2, locals.var_mnud1_dn3, locals.var_mnud1_dn4, locals.var_mnud1_dn5, locals.var_mnud1_dn6, locals.var_mnud1_dn7, locals.var_mnud1_dn8, locals.var_mnud1_dn9, locals.var_mnud1_dn10, locals.var_mnud1_dn11, locals.var_mnud1_dn12, locals.var_mnud1_dn13, locals.var_mnud1_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        locals.var_c0si_i = 0.0;

        locals.var_c0sisat1_i = 0.0;

        (locals.var_isubdr, locals.var_isubdr_dn0, locals.var_isubdr_dn2, locals.var_isubdr_dn3, locals.var_isubdr_dn4, locals.var_isubdr_dn5, locals.var_isubdr_dn6, locals.var_isubdr_dn7, locals.var_isubdr_dn8, locals.var_isubdr_dn9, locals.var_isubdr_dn10, locals.var_isubdr_dn11, locals.var_isubdr_dn12, locals.var_isubdr_dn13, locals.var_isubdr_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign190_e2015: f64 = 0.0;
        locals.var_gmin = assign190_e2015;

        (locals.var_eta0r_i, locals.var_eta0r_i_dn0, locals.var_eta0r_i_dn2, locals.var_eta0r_i_dn3, locals.var_eta0r_i_dn4, locals.var_eta0r_i_dn5, locals.var_eta0r_i_dn6, locals.var_eta0r_i_dn7, locals.var_eta0r_i_dn8, locals.var_eta0r_i_dn9, locals.var_eta0r_i_dn10, locals.var_eta0r_i_dn11, locals.var_eta0r_i_dn12, locals.var_eta0r_i_dn13, locals.var_eta0r_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_pclmr_i, locals.var_pclmr_i_dn0, locals.var_pclmr_i_dn2, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11, locals.var_pclmr_i_dn12, locals.var_pclmr_i_dn13, locals.var_pclmr_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_ptwgr_t, locals.var_ptwgr_t_dn0, locals.var_ptwgr_t_dn2, locals.var_ptwgr_t_dn3, locals.var_ptwgr_t_dn4, locals.var_ptwgr_t_dn5, locals.var_ptwgr_t_dn6, locals.var_ptwgr_t_dn7, locals.var_ptwgr_t_dn8, locals.var_ptwgr_t_dn9, locals.var_ptwgr_t_dn10, locals.var_ptwgr_t_dn11, locals.var_ptwgr_t_dn12, locals.var_ptwgr_t_dn13, locals.var_ptwgr_t_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_uar_t, locals.var_uar_t_dn0, locals.var_uar_t_dn2, locals.var_uar_t_dn3, locals.var_uar_t_dn4, locals.var_uar_t_dn5, locals.var_uar_t_dn6, locals.var_uar_t_dn7, locals.var_uar_t_dn8, locals.var_uar_t_dn9, locals.var_uar_t_dn10, locals.var_uar_t_dn11, locals.var_uar_t_dn12, locals.var_uar_t_dn13, locals.var_uar_t_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_ucsr_t, locals.var_ucsr_t_dn4, ) = (0.0, 0.0, );

        (locals.var_vsatr_i, locals.var_vsatr_i_dn0, locals.var_vsatr_i_dn2, locals.var_vsatr_i_dn3, locals.var_vsatr_i_dn4, locals.var_vsatr_i_dn5, locals.var_vsatr_i_dn6, locals.var_vsatr_i_dn7, locals.var_vsatr_i_dn8, locals.var_vsatr_i_dn9, locals.var_vsatr_i_dn10, locals.var_vsatr_i_dn11, locals.var_vsatr_i_dn12, locals.var_vsatr_i_dn13, locals.var_vsatr_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_inv_sb, locals.var_inv_sb_dn0, locals.var_inv_sb_dn2, locals.var_inv_sb_dn3, locals.var_inv_sb_dn4, locals.var_inv_sb_dn5, locals.var_inv_sb_dn6, locals.var_inv_sb_dn7, locals.var_inv_sb_dn8, locals.var_inv_sb_dn9, locals.var_inv_sb_dn10, locals.var_inv_sb_dn11, locals.var_inv_sb_dn12, locals.var_inv_sb_dn13, locals.var_inv_sb_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_local_scb, locals.var_local_scb_dn0, locals.var_local_scb_dn2, locals.var_local_scb_dn3, locals.var_local_scb_dn4, locals.var_local_scb_dn5, locals.var_local_scb_dn6, locals.var_local_scb_dn7, locals.var_local_scb_dn8, locals.var_local_scb_dn9, locals.var_local_scb_dn10, locals.var_local_scb_dn11, locals.var_local_scb_dn12, locals.var_local_scb_dn13, locals.var_local_scb_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        locals.var_k01_i = 0.0;

        locals.var_citedge_i = 0.0;

        locals.var_etabedge_i = 0.0;

        locals.var_kt1expedge_i = 0.0;

        locals.var_kvth0edge_i = 0.0;

        locals.var_c0_i = 0.0;

        locals.var_c0si1_i = 0.0;

        (locals.var_c0sisat_t, locals.var_c0sisat_t_dn4, ) = (0.0, 0.0, );

        (locals.var_rdstemphv, locals.var_rdstemphv_dn4, ) = (1.0, 0.0, );

        (locals.var_eta0r_t, locals.var_eta0r_t_dn0, locals.var_eta0r_t_dn2, locals.var_eta0r_t_dn3, locals.var_eta0r_t_dn4, locals.var_eta0r_t_dn5, locals.var_eta0r_t_dn6, locals.var_eta0r_t_dn7, locals.var_eta0r_t_dn8, locals.var_eta0r_t_dn9, locals.var_eta0r_t_dn10, locals.var_eta0r_t_dn11, locals.var_eta0r_t_dn12, locals.var_eta0r_t_dn13, locals.var_eta0r_t_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_pdiblcr_i, locals.var_pdiblcr_i_dn0, locals.var_pdiblcr_i_dn2, locals.var_pdiblcr_i_dn3, locals.var_pdiblcr_i_dn4, locals.var_pdiblcr_i_dn5, locals.var_pdiblcr_i_dn6, locals.var_pdiblcr_i_dn7, locals.var_pdiblcr_i_dn8, locals.var_pdiblcr_i_dn9, locals.var_pdiblcr_i_dn10, locals.var_pdiblcr_i_dn11, locals.var_pdiblcr_i_dn12, locals.var_pdiblcr_i_dn13, locals.var_pdiblcr_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        locals.var_u0r_i = 0.0;

        (locals.var_ucr_i, locals.var_ucr_i_dn0, locals.var_ucr_i_dn2, locals.var_ucr_i_dn3, locals.var_ucr_i_dn4, locals.var_ucr_i_dn5, locals.var_ucr_i_dn6, locals.var_ucr_i_dn7, locals.var_ucr_i_dn8, locals.var_ucr_i_dn9, locals.var_ucr_i_dn10, locals.var_ucr_i_dn11, locals.var_ucr_i_dn12, locals.var_ucr_i_dn13, locals.var_ucr_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_udr_i, locals.var_udr_i_dn0, locals.var_udr_i_dn2, locals.var_udr_i_dn3, locals.var_udr_i_dn4, locals.var_udr_i_dn5, locals.var_udr_i_dn6, locals.var_udr_i_dn7, locals.var_udr_i_dn8, locals.var_udr_i_dn9, locals.var_udr_i_dn10, locals.var_udr_i_dn11, locals.var_udr_i_dn12, locals.var_udr_i_dn13, locals.var_udr_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_vsatr_t, locals.var_vsatr_t_dn0, locals.var_vsatr_t_dn2, locals.var_vsatr_t_dn3, locals.var_vsatr_t_dn4, locals.var_vsatr_t_dn5, locals.var_vsatr_t_dn6, locals.var_vsatr_t_dn7, locals.var_vsatr_t_dn8, locals.var_vsatr_t_dn9, locals.var_vsatr_t_dn10, locals.var_vsatr_t_dn11, locals.var_vsatr_t_dn12, locals.var_vsatr_t_dn13, locals.var_vsatr_t_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_vth0_stress_edge, locals.var_vth0_stress_edge_dn0, locals.var_vth0_stress_edge_dn2, locals.var_vth0_stress_edge_dn3, locals.var_vth0_stress_edge_dn4, locals.var_vth0_stress_edge_dn5, locals.var_vth0_stress_edge_dn6, locals.var_vth0_stress_edge_dn7, locals.var_vth0_stress_edge_dn8, locals.var_vth0_stress_edge_dn9, locals.var_vth0_stress_edge_dn10, locals.var_vth0_stress_edge_dn11, locals.var_vth0_stress_edge_dn12, locals.var_vth0_stress_edge_dn13, locals.var_vth0_stress_edge_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_eta_stress_edge, locals.var_eta_stress_edge_dn0, locals.var_eta_stress_edge_dn2, locals.var_eta_stress_edge_dn3, locals.var_eta_stress_edge_dn4, locals.var_eta_stress_edge_dn5, locals.var_eta_stress_edge_dn6, locals.var_eta_stress_edge_dn7, locals.var_eta_stress_edge_dn8, locals.var_eta_stress_edge_dn9, locals.var_eta_stress_edge_dn10, locals.var_eta_stress_edge_dn11, locals.var_eta_stress_edge_dn12, locals.var_eta_stress_edge_dn13, locals.var_eta_stress_edge_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_local_scc, locals.var_local_scc_dn0, locals.var_local_scc_dn2, locals.var_local_scc_dn3, locals.var_local_scc_dn4, locals.var_local_scc_dn5, locals.var_local_scc_dn6, locals.var_local_scc_dn7, locals.var_local_scc_dn8, locals.var_local_scc_dn9, locals.var_local_scc_dn10, locals.var_local_scc_dn11, locals.var_local_scc_dn12, locals.var_local_scc_dn13, locals.var_local_scc_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        locals.var_m01_i = 0.0;

        locals.var_cdscdedge_i = 0.0;

        locals.var_kt1edge_i = 0.0;

        locals.var_tnfactoredge_i = 0.0;

        locals.var_stk2edge_i = 0.0;

        locals.var_c01_i = 0.0;

        (locals.var_c0si_t, locals.var_c0si_t_dn4, ) = (0.0, 0.0, );

        (locals.var_rdrift_d, locals.var_rdrift_d_dn0, locals.var_rdrift_d_dn2, locals.var_rdrift_d_dn3, locals.var_rdrift_d_dn4, locals.var_rdrift_d_dn5, locals.var_rdrift_d_dn6, locals.var_rdrift_d_dn7, locals.var_rdrift_d_dn8, locals.var_rdrift_d_dn9, locals.var_rdrift_d_dn10, locals.var_rdrift_d_dn11, locals.var_rdrift_d_dn12, locals.var_rdrift_d_dn13, locals.var_rdrift_d_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_vdrift_t, locals.var_vdrift_t_dn4, ) = (1.0, 0.0, );

        locals.var_l_lln1 = 0.0;

        locals.var_psatr_i = 0.0;

        (locals.var_u0r_t, locals.var_u0r_t_dn4, ) = (0.0, 0.0, );

        (locals.var_ucr_t, locals.var_ucr_t_dn0, locals.var_ucr_t_dn2, locals.var_ucr_t_dn3, locals.var_ucr_t_dn4, locals.var_ucr_t_dn5, locals.var_ucr_t_dn6, locals.var_ucr_t_dn7, locals.var_ucr_t_dn8, locals.var_ucr_t_dn9, locals.var_ucr_t_dn10, locals.var_ucr_t_dn11, locals.var_ucr_t_dn12, locals.var_ucr_t_dn13, locals.var_ucr_t_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_udr_t, locals.var_udr_t_dn0, locals.var_udr_t_dn2, locals.var_udr_t_dn3, locals.var_udr_t_dn4, locals.var_udr_t_dn5, locals.var_udr_t_dn6, locals.var_udr_t_dn7, locals.var_udr_t_dn8, locals.var_udr_t_dn9, locals.var_udr_t_dn10, locals.var_udr_t_dn11, locals.var_udr_t_dn12, locals.var_udr_t_dn13, locals.var_udr_t_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        locals.var_w_lwn1 = 0.0;

        (locals.var_k2_stress_edge, locals.var_k2_stress_edge_dn0, locals.var_k2_stress_edge_dn2, locals.var_k2_stress_edge_dn3, locals.var_k2_stress_edge_dn4, locals.var_k2_stress_edge_dn5, locals.var_k2_stress_edge_dn6, locals.var_k2_stress_edge_dn7, locals.var_k2_stress_edge_dn8, locals.var_k2_stress_edge_dn9, locals.var_k2_stress_edge_dn10, locals.var_k2_stress_edge_dn11, locals.var_k2_stress_edge_dn12, locals.var_k2_stress_edge_dn13, locals.var_k2_stress_edge_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        locals.var_k0_i = 0.0;

        (locals.var_k0_t, locals.var_k0_t_dn4, ) = (0.0, 0.0, );

        locals.var_cdscbedge_i = 0.0;

        locals.var_kt1ledge_i = 0.0;

        locals.var_teta0edge_i = 0.0;

        locals.var_steta0edge_i = 0.0;

        (locals.var_c0_t, locals.var_c0_t_dn4, ) = (0.0, 0.0, );

        locals.var_c0sisat_i = 0.0;

        (locals.var_rdrift_s, locals.var_rdrift_s_dn0, locals.var_rdrift_s_dn2, locals.var_rdrift_s_dn3, locals.var_rdrift_s_dn4, locals.var_rdrift_s_dn5, locals.var_rdrift_s_dn6, locals.var_rdrift_s_dn7, locals.var_rdrift_s_dn8, locals.var_rdrift_s_dn9, locals.var_rdrift_s_dn10, locals.var_rdrift_s_dn11, locals.var_rdrift_s_dn12, locals.var_rdrift_s_dn13, locals.var_rdrift_s_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        locals.var_k2edgewe_i = 0.0;

        locals.var_kvth0edgewe_i = 0.0;

        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_abulkiv, locals.var_abulkiv_dn0, locals.var_abulkiv_dn2, locals.var_abulkiv_dn3, locals.var_abulkiv_dn4, locals.var_abulkiv_dn5, locals.var_abulkiv_dn6, locals.var_abulkiv_dn7, locals.var_abulkiv_dn8, locals.var_abulkiv_dn9, locals.var_abulkiv_dn10, locals.var_abulkiv_dn11, locals.var_abulkiv_dn12, locals.var_abulkiv_dn13, locals.var_abulkiv_dn14, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_abulkcv, locals.var_abulkcv_dn0, locals.var_abulkcv_dn2, locals.var_abulkcv_dn3, locals.var_abulkcv_dn4, locals.var_abulkcv_dn5, locals.var_abulkcv_dn6, locals.var_abulkcv_dn7, locals.var_abulkcv_dn8, locals.var_abulkcv_dn9, locals.var_abulkcv_dn10, locals.var_abulkcv_dn11, locals.var_abulkcv_dn12, locals.var_abulkcv_dn13, locals.var_abulkcv_dn14, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_gdpr, locals.var_gdpr_dn0, locals.var_gdpr_dn2, locals.var_gdpr_dn3, locals.var_gdpr_dn4, locals.var_gdpr_dn5, locals.var_gdpr_dn6, locals.var_gdpr_dn7, locals.var_gdpr_dn8, locals.var_gdpr_dn9, locals.var_gdpr_dn10, locals.var_gdpr_dn11, locals.var_gdpr_dn12, locals.var_gdpr_dn13, locals.var_gdpr_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_gspr, locals.var_gspr_dn0, locals.var_gspr_dn2, locals.var_gspr_dn3, locals.var_gspr_dn4, locals.var_gspr_dn5, locals.var_gspr_dn6, locals.var_gspr_dn7, locals.var_gspr_dn8, locals.var_gspr_dn9, locals.var_gspr_dn10, locals.var_gspr_dn11, locals.var_gspr_dn12, locals.var_gspr_dn13, locals.var_gspr_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_gdrift_d, locals.var_gdrift_d_dn0, locals.var_gdrift_d_dn2, locals.var_gdrift_d_dn3, locals.var_gdrift_d_dn4, locals.var_gdrift_d_dn5, locals.var_gdrift_d_dn6, locals.var_gdrift_d_dn7, locals.var_gdrift_d_dn8, locals.var_gdrift_d_dn9, locals.var_gdrift_d_dn10, locals.var_gdrift_d_dn11, locals.var_gdrift_d_dn12, locals.var_gdrift_d_dn13, locals.var_gdrift_d_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_gdrift_s, locals.var_gdrift_s_dn0, locals.var_gdrift_s_dn2, locals.var_gdrift_s_dn3, locals.var_gdrift_s_dn4, locals.var_gdrift_s_dn5, locals.var_gdrift_s_dn6, locals.var_gdrift_s_dn7, locals.var_gdrift_s_dn8, locals.var_gdrift_s_dn9, locals.var_gdrift_s_dn10, locals.var_gdrift_s_dn11, locals.var_gdrift_s_dn12, locals.var_gdrift_s_dn13, locals.var_gdrift_s_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_vd1, locals.var_vd1_dn6, locals.var_vd1_dn11, ) = (0.0, 0.0, 0.0, );

        (locals.var_vs1, locals.var_vs1_dn8, locals.var_vs1_dn11, ) = (0.0, 0.0, 0.0, );

        (locals.var_idrift_sat_d, locals.var_idrift_sat_d_dn0, locals.var_idrift_sat_d_dn2, locals.var_idrift_sat_d_dn3, locals.var_idrift_sat_d_dn4, locals.var_idrift_sat_d_dn5, locals.var_idrift_sat_d_dn6, locals.var_idrift_sat_d_dn7, locals.var_idrift_sat_d_dn8, locals.var_idrift_sat_d_dn9, locals.var_idrift_sat_d_dn10, locals.var_idrift_sat_d_dn11, locals.var_idrift_sat_d_dn12, locals.var_idrift_sat_d_dn13, locals.var_idrift_sat_d_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_ln_t1_t2, locals.var_ln_t1_t2_dn0, locals.var_ln_t1_t2_dn2, locals.var_ln_t1_t2_dn3, locals.var_ln_t1_t2_dn4, locals.var_ln_t1_t2_dn5, locals.var_ln_t1_t2_dn6, locals.var_ln_t1_t2_dn7, locals.var_ln_t1_t2_dn8, locals.var_ln_t1_t2_dn9, locals.var_ln_t1_t2_dn10, locals.var_ln_t1_t2_dn11, locals.var_ln_t1_t2_dn12, locals.var_ln_t1_t2_dn13, locals.var_ln_t1_t2_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_iii, locals.var_iii_dn0, locals.var_iii_dn2, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11, locals.var_iii_dn12, locals.var_iii_dn13, locals.var_iii_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_vdseffii, locals.var_vdseffii_dn0, locals.var_vdseffii_dn2, locals.var_vdseffii_dn3, locals.var_vdseffii_dn4, locals.var_vdseffii_dn5, locals.var_vdseffii_dn6, locals.var_vdseffii_dn7, locals.var_vdseffii_dn8, locals.var_vdseffii_dn9, locals.var_vdseffii_dn10, locals.var_vdseffii_dn11, locals.var_vdseffii_dn12, locals.var_vdseffii_dn13, locals.var_vdseffii_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_beta0r_t, locals.var_beta0r_t_dn4, ) = (0.0, 0.0, );

        (locals.var_alpha0r_i, locals.var_alpha0r_i_dn0, locals.var_alpha0r_i_dn2, locals.var_alpha0r_i_dn3, locals.var_alpha0r_i_dn4, locals.var_alpha0r_i_dn5, locals.var_alpha0r_i_dn6, locals.var_alpha0r_i_dn7, locals.var_alpha0r_i_dn8, locals.var_alpha0r_i_dn9, locals.var_alpha0r_i_dn10, locals.var_alpha0r_i_dn11, locals.var_alpha0r_i_dn12, locals.var_alpha0r_i_dn13, locals.var_alpha0r_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        locals.var_beta0r_i = 0.0;

        (locals.var_vb_cm, locals.var_vb_cm_dn3, locals.var_vb_cm_dn11, ) = (0.0, 0.0, 0.0, );

        let assign940_e2092: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign940_e2092;

        if (locals.var_guard1 != 0.0) {
            locals.var_devsign = 1.0;
        }

        if (locals.var_guard1 == 0.0) {
            let assign960_e2100: f64 = (-1.0);
            locals.var_devsign = assign960_e2100;
        }

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

        if (locals.var_guard2 != 0.0) {
            let assign1020_e2123: f64 = (p.p77 * p.p111);
            let assign1020_e2125: f64 = (assign1020_e2123 / 3.9);
            let assign1020_e2127: f64 = (assign1020_e2125 - p.p79);
            locals.var_bsimbulktoxp = assign1020_e2127;
        }

        if (locals.var_guard2 == 0.0) {
            locals.var_bsimbulktoxp = p.p78;
        }

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

        if ((locals.var_guard14 != 0.0) && (locals.var_guard15 == 0.0)) {
            let assign1470_e2341: f64 = (locals.var_lnew + p.p818);
            let assign1470_e2343: f64 = (-p.p61);
            let assign1470_e2344: f64 = (assign1470_e2341).powf(assign1470_e2343);
            locals.var_l_lln1 = assign1470_e2344;
        }

        if ((locals.var_guard14 != 0.0) && (locals.var_guard15 == 0.0)) {
            let assign1480_e2353: f64 = (locals.var_lnew + p.p818);
            let assign1480_e2355: f64 = (-p.p67);
            let assign1480_e2356: f64 = (assign1480_e2353).powf(assign1480_e2355);
            locals.var_l_wln1 = assign1480_e2356;
        }

        locals.var_w_lwn1 = locals.var_w_lwn;

        locals.var_w_wwn1 = locals.var_w_wwn;

        let assign1510_e2363: f64 = if p.p819 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1510_e2363;

        let assign1520_e2366: f64 = (-locals.var_wnew);
        let assign1520_e2367: f64 = if p.p819 <= assign1520_e2366 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1520_e2367;

        if ((locals.var_guard16 != 0.0) && (locals.var_guard17 == 0.0)) {
            let assign1530_e2374: f64 = (locals.var_wnew + p.p819);
            let assign1530_e2376: f64 = (-p.p62);
            let assign1530_e2377: f64 = (assign1530_e2374).powf(assign1530_e2376);
            locals.var_w_lwn1 = assign1530_e2377;
        }

        if ((locals.var_guard16 != 0.0) && (locals.var_guard17 == 0.0)) {
            let assign1540_e2386: f64 = (locals.var_wnew + p.p819);
            let assign1540_e2388: f64 = (-p.p68);
            let assign1540_e2389: f64 = (assign1540_e2386).powf(assign1540_e2388);
            locals.var_w_wwn1 = assign1540_e2389;
        }

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

        if (locals.var_guard20 != 0.0) {
            let assign1640_e2450: f64 = (1e-6 / locals.var_leff1);
            locals.var_bin_l = assign1640_e2450;
        }

        if (locals.var_guard20 != 0.0) {
            let assign1650_e2456: f64 = (1e-6 / locals.var_weff1);
            locals.var_bin_w = assign1650_e2456;
        }

        if (locals.var_guard20 == 0.0) {
            let assign1660_e2463: f64 = (1.0 / locals.var_leff1);
            locals.var_bin_l = assign1660_e2463;
        }

        if (locals.var_guard20 == 0.0) {
            let assign1670_e2470: f64 = (1.0 / locals.var_weff1);
            locals.var_bin_w = assign1670_e2470;
        }

        let assign1680_e2475: f64 = (locals.var_bin_l * locals.var_bin_w);
        locals.var_bin_wl = assign1680_e2475;

        let assign1690_e2479: f64 = (locals.var_bin_l * p.p117);
        let assign1690_e2480: f64 = (p.p116 + assign1690_e2479);
        let assign1690_e2483: f64 = (locals.var_bin_w * p.p118);
        let assign1690_e2484: f64 = (assign1690_e2480 + assign1690_e2483);
        let assign1690_e2487: f64 = (locals.var_bin_wl * p.p119);
        let assign1690_e2488: f64 = (assign1690_e2484 + assign1690_e2487);
        (locals.var_vfb_i, locals.var_vfb_i_dn0, locals.var_vfb_i_dn2, locals.var_vfb_i_dn3, locals.var_vfb_i_dn4, locals.var_vfb_i_dn5, locals.var_vfb_i_dn6, locals.var_vfb_i_dn7, locals.var_vfb_i_dn8, locals.var_vfb_i_dn9, locals.var_vfb_i_dn10, locals.var_vfb_i_dn11, locals.var_vfb_i_dn12, locals.var_vfb_i_dn13, locals.var_vfb_i_dn14, ) = (assign1690_e2488, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign1700_e2492: f64 = (locals.var_bin_l * p.p127);
        let assign1700_e2493: f64 = (p.p126 + assign1700_e2492);
        let assign1700_e2496: f64 = (locals.var_bin_w * p.p128);
        let assign1700_e2497: f64 = (assign1700_e2493 + assign1700_e2496);
        let assign1700_e2500: f64 = (locals.var_bin_wl * p.p129);
        let assign1700_e2501: f64 = (assign1700_e2497 + assign1700_e2500);
        (locals.var_vfbcv_i, locals.var_vfbcv_i_dn0, locals.var_vfbcv_i_dn2, locals.var_vfbcv_i_dn3, locals.var_vfbcv_i_dn4, locals.var_vfbcv_i_dn5, locals.var_vfbcv_i_dn6, locals.var_vfbcv_i_dn7, locals.var_vfbcv_i_dn8, locals.var_vfbcv_i_dn9, locals.var_vfbcv_i_dn10, locals.var_vfbcv_i_dn11, locals.var_vfbcv_i_dn12, locals.var_vfbcv_i_dn13, locals.var_vfbcv_i_dn14, ) = (assign1700_e2501, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

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
        (locals.var_ndep_i, locals.var_ndep_i_dn0, locals.var_ndep_i_dn2, locals.var_ndep_i_dn3, locals.var_ndep_i_dn4, locals.var_ndep_i_dn5, locals.var_ndep_i_dn6, locals.var_ndep_i_dn7, locals.var_ndep_i_dn8, locals.var_ndep_i_dn9, locals.var_ndep_i_dn10, locals.var_ndep_i_dn11, locals.var_ndep_i_dn12, locals.var_ndep_i_dn13, locals.var_ndep_i_dn14, ) = (assign1720_e2527, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign1730_e2531: f64 = (locals.var_bin_l * p.p101);
        let assign1730_e2532: f64 = (p.p92 + assign1730_e2531);
        let assign1730_e2535: f64 = (locals.var_bin_w * p.p102);
        let assign1730_e2536: f64 = (assign1730_e2532 + assign1730_e2535);
        let assign1730_e2539: f64 = (locals.var_bin_wl * p.p103);
        let assign1730_e2540: f64 = (assign1730_e2536 + assign1730_e2539);
        (locals.var_ndepcv_i, locals.var_ndepcv_i_dn0, locals.var_ndepcv_i_dn2, locals.var_ndepcv_i_dn3, locals.var_ndepcv_i_dn4, locals.var_ndepcv_i_dn5, locals.var_ndepcv_i_dn6, locals.var_ndepcv_i_dn7, locals.var_ndepcv_i_dn8, locals.var_ndepcv_i_dn9, locals.var_ndepcv_i_dn10, locals.var_ndepcv_i_dn11, locals.var_ndepcv_i_dn12, locals.var_ndepcv_i_dn13, locals.var_ndepcv_i_dn14, ) = (assign1730_e2540, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign1740_e2544: f64 = (locals.var_bin_l * p.p105);
        let assign1740_e2545: f64 = (p.p104 + assign1740_e2544);
        let assign1740_e2548: f64 = (locals.var_bin_w * p.p106);
        let assign1740_e2549: f64 = (assign1740_e2545 + assign1740_e2548);
        let assign1740_e2552: f64 = (locals.var_bin_wl * p.p107);
        let assign1740_e2553: f64 = (assign1740_e2549 + assign1740_e2552);
        locals.var_ngate_i = assign1740_e2553;

    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        (locals.var_nfactor_i, locals.var_nfactor_i_dn0, locals.var_nfactor_i_dn2, locals.var_nfactor_i_dn3, locals.var_nfactor_i_dn4, locals.var_nfactor_i_dn5, locals.var_nfactor_i_dn6, locals.var_nfactor_i_dn7, locals.var_nfactor_i_dn8, locals.var_nfactor_i_dn9, locals.var_nfactor_i_dn10, locals.var_nfactor_i_dn11, locals.var_nfactor_i_dn12, locals.var_nfactor_i_dn13, locals.var_nfactor_i_dn14, ) = (assign1760_e2579, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign1770_e2583: f64 = (locals.var_bin_l * p.p226);
        let assign1770_e2584: f64 = (p.p223 + assign1770_e2583);
        let assign1770_e2587: f64 = (locals.var_bin_w * p.p227);
        let assign1770_e2588: f64 = (assign1770_e2584 + assign1770_e2587);
        let assign1770_e2591: f64 = (locals.var_bin_wl * p.p228);
        let assign1770_e2592: f64 = (assign1770_e2588 + assign1770_e2591);
        (locals.var_cdscd_i, locals.var_cdscd_i_dn0, locals.var_cdscd_i_dn2, locals.var_cdscd_i_dn3, locals.var_cdscd_i_dn4, locals.var_cdscd_i_dn5, locals.var_cdscd_i_dn6, locals.var_cdscd_i_dn7, locals.var_cdscd_i_dn8, locals.var_cdscd_i_dn9, locals.var_cdscd_i_dn10, locals.var_cdscd_i_dn11, locals.var_cdscd_i_dn12, locals.var_cdscd_i_dn13, locals.var_cdscd_i_dn14, ) = (assign1770_e2592, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

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
        (locals.var_k2_i, locals.var_k2_i_dn0, locals.var_k2_i_dn2, locals.var_k2_i_dn3, locals.var_k2_i_dn4, locals.var_k2_i_dn5, locals.var_k2_i_dn6, locals.var_k2_i_dn7, locals.var_k2_i_dn8, locals.var_k2_i_dn9, locals.var_k2_i_dn10, locals.var_k2_i_dn11, locals.var_k2_i_dn12, locals.var_k2_i_dn13, locals.var_k2_i_dn14, ) = (assign1850_e2696, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign1860_e2700: f64 = (locals.var_bin_l * p.p192);
        let assign1860_e2701: f64 = (p.p185 + assign1860_e2700);
        let assign1860_e2704: f64 = (locals.var_bin_w * p.p193);
        let assign1860_e2705: f64 = (assign1860_e2701 + assign1860_e2704);
        let assign1860_e2708: f64 = (locals.var_bin_wl * p.p194);
        let assign1860_e2709: f64 = (assign1860_e2705 + assign1860_e2708);
        (locals.var_k1_i, locals.var_k1_i_dn0, locals.var_k1_i_dn2, locals.var_k1_i_dn3, locals.var_k1_i_dn4, locals.var_k1_i_dn5, locals.var_k1_i_dn6, locals.var_k1_i_dn7, locals.var_k1_i_dn8, locals.var_k1_i_dn9, locals.var_k1_i_dn10, locals.var_k1_i_dn11, locals.var_k1_i_dn12, locals.var_k1_i_dn13, locals.var_k1_i_dn14, ) = (assign1860_e2709, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

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
        (locals.var_eta0_i, locals.var_eta0_i_dn0, locals.var_eta0_i_dn2, locals.var_eta0_i_dn3, locals.var_eta0_i_dn4, locals.var_eta0_i_dn5, locals.var_eta0_i_dn6, locals.var_eta0_i_dn7, locals.var_eta0_i_dn8, locals.var_eta0_i_dn9, locals.var_eta0_i_dn10, locals.var_eta0_i_dn11, locals.var_eta0_i_dn12, locals.var_eta0_i_dn13, locals.var_eta0_i_dn14, ) = (assign1890_e2748, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

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
        (locals.var_delta_i, locals.var_delta_i_dn0, locals.var_delta_i_dn2, locals.var_delta_i_dn3, locals.var_delta_i_dn4, locals.var_delta_i_dn5, locals.var_delta_i_dn6, locals.var_delta_i_dn7, locals.var_delta_i_dn8, locals.var_delta_i_dn9, locals.var_delta_i_dn10, locals.var_delta_i_dn11, locals.var_delta_i_dn12, locals.var_delta_i_dn13, locals.var_delta_i_dn14, ) = (assign1910_e2774, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign1920_e2778: f64 = (locals.var_bin_l * p.p276);
        let assign1920_e2779: f64 = (p.p273 + assign1920_e2778);
        let assign1920_e2782: f64 = (locals.var_bin_w * p.p277);
        let assign1920_e2783: f64 = (assign1920_e2779 + assign1920_e2782);
        let assign1920_e2786: f64 = (locals.var_bin_wl * p.p278);
        let assign1920_e2787: f64 = (assign1920_e2783 + assign1920_e2786);
        locals.var_u0_i = assign1920_e2787;

        let assign1930_e2791: f64 = (locals.var_bin_l * p.p291);
        let assign1930_e2792: f64 = (p.p284 + assign1930_e2791);
        let assign1930_e2795: f64 = (locals.var_bin_w * p.p292);
        let assign1930_e2796: f64 = (assign1930_e2792 + assign1930_e2795);
        let assign1930_e2799: f64 = (locals.var_bin_wl * p.p293);
        let assign1930_e2800: f64 = (assign1930_e2796 + assign1930_e2799);
        (locals.var_ua_i, locals.var_ua_i_dn0, locals.var_ua_i_dn2, locals.var_ua_i_dn3, locals.var_ua_i_dn4, locals.var_ua_i_dn5, locals.var_ua_i_dn6, locals.var_ua_i_dn7, locals.var_ua_i_dn8, locals.var_ua_i_dn9, locals.var_ua_i_dn10, locals.var_ua_i_dn11, locals.var_ua_i_dn12, locals.var_ua_i_dn13, locals.var_ua_i_dn14, ) = (assign1930_e2800, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign1940_e2804: f64 = (locals.var_bin_l * p.p311);
        let assign1940_e2805: f64 = (p.p308 + assign1940_e2804);
        let assign1940_e2808: f64 = (locals.var_bin_w * p.p312);
        let assign1940_e2809: f64 = (assign1940_e2805 + assign1940_e2808);
        let assign1940_e2812: f64 = (locals.var_bin_wl * p.p313);
        let assign1940_e2813: f64 = (assign1940_e2809 + assign1940_e2812);
        (locals.var_ud_i, locals.var_ud_i_dn0, locals.var_ud_i_dn2, locals.var_ud_i_dn3, locals.var_ud_i_dn4, locals.var_ud_i_dn5, locals.var_ud_i_dn6, locals.var_ud_i_dn7, locals.var_ud_i_dn8, locals.var_ud_i_dn9, locals.var_ud_i_dn10, locals.var_ud_i_dn11, locals.var_ud_i_dn12, locals.var_ud_i_dn13, locals.var_ud_i_dn14, ) = (assign1940_e2813, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign1950_e2817: f64 = (locals.var_bin_l * p.p299);
        let assign1950_e2818: f64 = (p.p298 + assign1950_e2817);
        let assign1950_e2821: f64 = (locals.var_bin_w * p.p300);
        let assign1950_e2822: f64 = (assign1950_e2818 + assign1950_e2821);
        let assign1950_e2825: f64 = (locals.var_bin_wl * p.p301);
        let assign1950_e2826: f64 = (assign1950_e2822 + assign1950_e2825);
        (locals.var_eu_i, locals.var_eu_i_dn0, locals.var_eu_i_dn2, locals.var_eu_i_dn3, locals.var_eu_i_dn4, locals.var_eu_i_dn5, locals.var_eu_i_dn6, locals.var_eu_i_dn7, locals.var_eu_i_dn8, locals.var_eu_i_dn9, locals.var_eu_i_dn10, locals.var_eu_i_dn11, locals.var_eu_i_dn12, locals.var_eu_i_dn13, locals.var_eu_i_dn14, ) = (assign1950_e2826, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

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
        (locals.var_uc_i, locals.var_uc_i_dn0, locals.var_uc_i_dn2, locals.var_uc_i_dn3, locals.var_uc_i_dn4, locals.var_uc_i_dn5, locals.var_uc_i_dn6, locals.var_uc_i_dn7, locals.var_uc_i_dn8, locals.var_uc_i_dn9, locals.var_uc_i_dn10, locals.var_uc_i_dn11, locals.var_uc_i_dn12, locals.var_uc_i_dn13, locals.var_uc_i_dn14, ) = (assign1970_e2852, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign1980_e2856: f64 = (locals.var_bin_l * p.p343);
        let assign1980_e2857: f64 = (p.p340 + assign1980_e2856);
        let assign1980_e2860: f64 = (locals.var_bin_w * p.p344);
        let assign1980_e2861: f64 = (assign1980_e2857 + assign1980_e2860);
        let assign1980_e2864: f64 = (locals.var_bin_wl * p.p345);
        let assign1980_e2865: f64 = (assign1980_e2861 + assign1980_e2864);
        (locals.var_pclm_i, locals.var_pclm_i_dn0, locals.var_pclm_i_dn2, locals.var_pclm_i_dn3, locals.var_pclm_i_dn4, locals.var_pclm_i_dn5, locals.var_pclm_i_dn6, locals.var_pclm_i_dn7, locals.var_pclm_i_dn8, locals.var_pclm_i_dn9, locals.var_pclm_i_dn10, locals.var_pclm_i_dn11, locals.var_pclm_i_dn12, locals.var_pclm_i_dn13, locals.var_pclm_i_dn14, ) = (assign1980_e2865, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

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
        (locals.var_ptwg_i, locals.var_ptwg_i_dn0, locals.var_ptwg_i_dn2, locals.var_ptwg_i_dn3, locals.var_ptwg_i_dn4, locals.var_ptwg_i_dn5, locals.var_ptwg_i_dn6, locals.var_ptwg_i_dn7, locals.var_ptwg_i_dn8, locals.var_ptwg_i_dn9, locals.var_ptwg_i_dn10, locals.var_ptwg_i_dn11, locals.var_ptwg_i_dn12, locals.var_ptwg_i_dn13, locals.var_ptwg_i_dn14, ) = (assign2090_e3008, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign2100_e3012: f64 = (locals.var_bin_l * p.p463);
        let assign2100_e3013: f64 = (p.p460 + assign2100_e3012);
        let assign2100_e3016: f64 = (locals.var_bin_w * p.p464);
        let assign2100_e3017: f64 = (assign2100_e3013 + assign2100_e3016);
        let assign2100_e3020: f64 = (locals.var_bin_wl * p.p465);
        let assign2100_e3021: f64 = (assign2100_e3017 + assign2100_e3020);
        (locals.var_pdiblc_i, locals.var_pdiblc_i_dn0, locals.var_pdiblc_i_dn2, locals.var_pdiblc_i_dn3, locals.var_pdiblc_i_dn4, locals.var_pdiblc_i_dn5, locals.var_pdiblc_i_dn6, locals.var_pdiblc_i_dn7, locals.var_pdiblc_i_dn8, locals.var_pdiblc_i_dn9, locals.var_pdiblc_i_dn10, locals.var_pdiblc_i_dn11, locals.var_pdiblc_i_dn12, locals.var_pdiblc_i_dn13, locals.var_pdiblc_i_dn14, ) = (assign2100_e3021, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

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
        (locals.var_vsat_i, locals.var_vsat_i_dn0, locals.var_vsat_i_dn2, locals.var_vsat_i_dn3, locals.var_vsat_i_dn4, locals.var_vsat_i_dn5, locals.var_vsat_i_dn6, locals.var_vsat_i_dn7, locals.var_vsat_i_dn8, locals.var_vsat_i_dn9, locals.var_vsat_i_dn10, locals.var_vsat_i_dn11, locals.var_vsat_i_dn12, locals.var_vsat_i_dn13, locals.var_vsat_i_dn14, ) = (assign2180_e3125, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

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
        (locals.var_vsatcv_i, locals.var_vsatcv_i_dn0, locals.var_vsatcv_i_dn2, locals.var_vsatcv_i_dn3, locals.var_vsatcv_i_dn4, locals.var_vsatcv_i_dn5, locals.var_vsatcv_i_dn6, locals.var_vsatcv_i_dn7, locals.var_vsatcv_i_dn8, locals.var_vsatcv_i_dn9, locals.var_vsatcv_i_dn10, locals.var_vsatcv_i_dn11, locals.var_vsatcv_i_dn12, locals.var_vsatcv_i_dn13, locals.var_vsatcv_i_dn14, ) = (assign2200_e3151, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

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
        (locals.var_alpha0_i, locals.var_alpha0_i_dn0, locals.var_alpha0_i_dn2, locals.var_alpha0_i_dn3, locals.var_alpha0_i_dn4, locals.var_alpha0_i_dn5, locals.var_alpha0_i_dn6, locals.var_alpha0_i_dn7, locals.var_alpha0_i_dn8, locals.var_alpha0_i_dn9, locals.var_alpha0_i_dn10, locals.var_alpha0_i_dn11, locals.var_alpha0_i_dn12, locals.var_alpha0_i_dn13, locals.var_alpha0_i_dn14, ) = (assign2260_e3229, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign2270_e3233: f64 = (locals.var_bin_l * p.p497);
        let assign2270_e3234: f64 = (p.p494 + assign2270_e3233);
        let assign2270_e3237: f64 = (locals.var_bin_w * p.p498);
        let assign2270_e3238: f64 = (assign2270_e3234 + assign2270_e3237);
        let assign2270_e3241: f64 = (locals.var_bin_wl * p.p499);
        let assign2270_e3242: f64 = (assign2270_e3238 + assign2270_e3241);
        (locals.var_beta0_i, locals.var_beta0_i_dn0, locals.var_beta0_i_dn2, locals.var_beta0_i_dn3, locals.var_beta0_i_dn4, locals.var_beta0_i_dn5, locals.var_beta0_i_dn6, locals.var_beta0_i_dn7, locals.var_beta0_i_dn8, locals.var_beta0_i_dn9, locals.var_beta0_i_dn10, locals.var_beta0_i_dn11, locals.var_beta0_i_dn12, locals.var_beta0_i_dn13, locals.var_beta0_i_dn14, ) = (assign2270_e3242, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

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

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        (locals.var_eta0edge_i, locals.var_eta0edge_i_dn0, locals.var_eta0edge_i_dn2, locals.var_eta0edge_i_dn3, locals.var_eta0edge_i_dn4, locals.var_eta0edge_i_dn5, locals.var_eta0edge_i_dn6, locals.var_eta0edge_i_dn7, locals.var_eta0edge_i_dn8, locals.var_eta0edge_i_dn9, locals.var_eta0edge_i_dn10, locals.var_eta0edge_i_dn11, locals.var_eta0edge_i_dn12, locals.var_eta0edge_i_dn13, locals.var_eta0edge_i_dn14, ) = (assign2890_e4048, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

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

        let assign2970_e4143: f64 = (locals.var_bin_l * p.p1018);
        let assign2970_e4144: f64 = (p.p1017 + assign2970_e4143);
        let assign2970_e4147: f64 = (locals.var_bin_w * p.p1019);
        let assign2970_e4148: f64 = (assign2970_e4144 + assign2970_e4147);
        let assign2970_e4151: f64 = (locals.var_bin_wl * p.p1020);
        let assign2970_e4152: f64 = (assign2970_e4148 + assign2970_e4151);
        (locals.var_k2edge_i, locals.var_k2edge_i_dn0, locals.var_k2edge_i_dn2, locals.var_k2edge_i_dn3, locals.var_k2edge_i_dn4, locals.var_k2edge_i_dn5, locals.var_k2edge_i_dn6, locals.var_k2edge_i_dn7, locals.var_k2edge_i_dn8, locals.var_k2edge_i_dn9, locals.var_k2edge_i_dn10, locals.var_k2edge_i_dn11, locals.var_k2edge_i_dn12, locals.var_k2edge_i_dn13, locals.var_k2edge_i_dn14, ) = (assign2970_e4152, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

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

    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        if (locals.var_guard21 != 0.0) {
            let assign3130_e4342: f64 = (locals.var_bin_l * p.p230);
            let assign3130_e4343: f64 = (p.p229 + assign3130_e4342);
            let assign3130_e4346: f64 = (locals.var_bin_w * p.p231);
            let assign3130_e4347: f64 = (assign3130_e4343 + assign3130_e4346);
            let assign3130_e4350: f64 = (locals.var_bin_wl * p.p232);
            let assign3130_e4351: f64 = (assign3130_e4347 + assign3130_e4350);
            (locals.var_cdscdr_i, locals.var_cdscdr_i_dn0, locals.var_cdscdr_i_dn2, locals.var_cdscdr_i_dn3, locals.var_cdscdr_i_dn4, locals.var_cdscdr_i_dn5, locals.var_cdscdr_i_dn6, locals.var_cdscdr_i_dn7, locals.var_cdscdr_i_dn8, locals.var_cdscdr_i_dn9, locals.var_cdscdr_i_dn10, locals.var_cdscdr_i_dn11, locals.var_cdscdr_i_dn12, locals.var_cdscdr_i_dn13, locals.var_cdscdr_i_dn14, ) = (assign3130_e4351, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard21 != 0.0) {
            let assign3140_e4358: f64 = (locals.var_bin_l * p.p176);
            let assign3140_e4359: f64 = (p.p175 + assign3140_e4358);
            let assign3140_e4362: f64 = (locals.var_bin_w * p.p177);
            let assign3140_e4363: f64 = (assign3140_e4359 + assign3140_e4362);
            let assign3140_e4366: f64 = (locals.var_bin_wl * p.p178);
            let assign3140_e4367: f64 = (assign3140_e4363 + assign3140_e4366);
            (locals.var_eta0r_i, locals.var_eta0r_i_dn0, locals.var_eta0r_i_dn2, locals.var_eta0r_i_dn3, locals.var_eta0r_i_dn4, locals.var_eta0r_i_dn5, locals.var_eta0r_i_dn6, locals.var_eta0r_i_dn7, locals.var_eta0r_i_dn8, locals.var_eta0r_i_dn9, locals.var_eta0r_i_dn10, locals.var_eta0r_i_dn11, locals.var_eta0r_i_dn12, locals.var_eta0r_i_dn13, locals.var_eta0r_i_dn14, ) = (assign3140_e4367, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard21 != 0.0) {
            let assign3150_e4374: f64 = (locals.var_bin_l * p.p280);
            let assign3150_e4375: f64 = (p.p279 + assign3150_e4374);
            let assign3150_e4378: f64 = (locals.var_bin_w * p.p281);
            let assign3150_e4379: f64 = (assign3150_e4375 + assign3150_e4378);
            let assign3150_e4382: f64 = (locals.var_bin_wl * p.p282);
            let assign3150_e4383: f64 = (assign3150_e4379 + assign3150_e4382);
            locals.var_u0r_i = assign3150_e4383;
        }

        if (locals.var_guard21 != 0.0) {
            let assign3160_e4390: f64 = (locals.var_bin_l * p.p295);
            let assign3160_e4391: f64 = (p.p294 + assign3160_e4390);
            let assign3160_e4394: f64 = (locals.var_bin_w * p.p296);
            let assign3160_e4395: f64 = (assign3160_e4391 + assign3160_e4394);
            let assign3160_e4398: f64 = (locals.var_bin_wl * p.p297);
            let assign3160_e4399: f64 = (assign3160_e4395 + assign3160_e4398);
            (locals.var_uar_i, locals.var_uar_i_dn0, locals.var_uar_i_dn2, locals.var_uar_i_dn3, locals.var_uar_i_dn4, locals.var_uar_i_dn5, locals.var_uar_i_dn6, locals.var_uar_i_dn7, locals.var_uar_i_dn8, locals.var_uar_i_dn9, locals.var_uar_i_dn10, locals.var_uar_i_dn11, locals.var_uar_i_dn12, locals.var_uar_i_dn13, locals.var_uar_i_dn14, ) = (assign3160_e4399, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard21 != 0.0) {
            let assign3170_e4406: f64 = (locals.var_bin_l * p.p315);
            let assign3170_e4407: f64 = (p.p314 + assign3170_e4406);
            let assign3170_e4410: f64 = (locals.var_bin_w * p.p316);
            let assign3170_e4411: f64 = (assign3170_e4407 + assign3170_e4410);
            let assign3170_e4414: f64 = (locals.var_bin_wl * p.p317);
            let assign3170_e4415: f64 = (assign3170_e4411 + assign3170_e4414);
            (locals.var_udr_i, locals.var_udr_i_dn0, locals.var_udr_i_dn2, locals.var_udr_i_dn3, locals.var_udr_i_dn4, locals.var_udr_i_dn5, locals.var_udr_i_dn6, locals.var_udr_i_dn7, locals.var_udr_i_dn8, locals.var_udr_i_dn9, locals.var_udr_i_dn10, locals.var_udr_i_dn11, locals.var_udr_i_dn12, locals.var_udr_i_dn13, locals.var_udr_i_dn14, ) = (assign3170_e4415, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard21 != 0.0) {
            let assign3180_e4422: f64 = (locals.var_bin_l * p.p323);
            let assign3180_e4423: f64 = (p.p322 + assign3180_e4422);
            let assign3180_e4426: f64 = (locals.var_bin_w * p.p324);
            let assign3180_e4427: f64 = (assign3180_e4423 + assign3180_e4426);
            let assign3180_e4430: f64 = (locals.var_bin_wl * p.p325);
            let assign3180_e4431: f64 = (assign3180_e4427 + assign3180_e4430);
            locals.var_ucsr_i = assign3180_e4431;
        }

        if (locals.var_guard21 != 0.0) {
            let assign3190_e4438: f64 = (locals.var_bin_l * p.p337);
            let assign3190_e4439: f64 = (p.p336 + assign3190_e4438);
            let assign3190_e4442: f64 = (locals.var_bin_w * p.p338);
            let assign3190_e4443: f64 = (assign3190_e4439 + assign3190_e4442);
            let assign3190_e4446: f64 = (locals.var_bin_wl * p.p339);
            let assign3190_e4447: f64 = (assign3190_e4443 + assign3190_e4446);
            (locals.var_ucr_i, locals.var_ucr_i_dn0, locals.var_ucr_i_dn2, locals.var_ucr_i_dn3, locals.var_ucr_i_dn4, locals.var_ucr_i_dn5, locals.var_ucr_i_dn6, locals.var_ucr_i_dn7, locals.var_ucr_i_dn8, locals.var_ucr_i_dn9, locals.var_ucr_i_dn10, locals.var_ucr_i_dn11, locals.var_ucr_i_dn12, locals.var_ucr_i_dn13, locals.var_ucr_i_dn14, ) = (assign3190_e4447, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard21 != 0.0) {
            let assign3200_e4454: f64 = (locals.var_bin_l * p.p347);
            let assign3200_e4455: f64 = (p.p346 + assign3200_e4454);
            let assign3200_e4458: f64 = (locals.var_bin_w * p.p348);
            let assign3200_e4459: f64 = (assign3200_e4455 + assign3200_e4458);
            let assign3200_e4462: f64 = (locals.var_bin_wl * p.p349);
            let assign3200_e4463: f64 = (assign3200_e4459 + assign3200_e4462);
            (locals.var_pclmr_i, locals.var_pclmr_i_dn0, locals.var_pclmr_i_dn2, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11, locals.var_pclmr_i_dn12, locals.var_pclmr_i_dn13, locals.var_pclmr_i_dn14, ) = (assign3200_e4463, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard21 != 0.0) {
            let assign3210_e4470: f64 = (locals.var_bin_l * p.p467);
            let assign3210_e4471: f64 = (p.p466 + assign3210_e4470);
            let assign3210_e4474: f64 = (locals.var_bin_w * p.p468);
            let assign3210_e4475: f64 = (assign3210_e4471 + assign3210_e4474);
            let assign3210_e4478: f64 = (locals.var_bin_wl * p.p469);
            let assign3210_e4479: f64 = (assign3210_e4475 + assign3210_e4478);
            (locals.var_pdiblcr_i, locals.var_pdiblcr_i_dn0, locals.var_pdiblcr_i_dn2, locals.var_pdiblcr_i_dn3, locals.var_pdiblcr_i_dn4, locals.var_pdiblcr_i_dn5, locals.var_pdiblcr_i_dn6, locals.var_pdiblcr_i_dn7, locals.var_pdiblcr_i_dn8, locals.var_pdiblcr_i_dn9, locals.var_pdiblcr_i_dn10, locals.var_pdiblcr_i_dn11, locals.var_pdiblcr_i_dn12, locals.var_pdiblcr_i_dn13, locals.var_pdiblcr_i_dn14, ) = (assign3210_e4479, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard21 != 0.0) {
            let assign3220_e4486: f64 = (locals.var_bin_l * p.p250);
            let assign3220_e4487: f64 = (p.p249 + assign3220_e4486);
            let assign3220_e4490: f64 = (locals.var_bin_w * p.p251);
            let assign3220_e4491: f64 = (assign3220_e4487 + assign3220_e4490);
            let assign3220_e4494: f64 = (locals.var_bin_wl * p.p252);
            let assign3220_e4495: f64 = (assign3220_e4491 + assign3220_e4494);
            (locals.var_vsatr_i, locals.var_vsatr_i_dn0, locals.var_vsatr_i_dn2, locals.var_vsatr_i_dn3, locals.var_vsatr_i_dn4, locals.var_vsatr_i_dn5, locals.var_vsatr_i_dn6, locals.var_vsatr_i_dn7, locals.var_vsatr_i_dn8, locals.var_vsatr_i_dn9, locals.var_vsatr_i_dn10, locals.var_vsatr_i_dn11, locals.var_vsatr_i_dn12, locals.var_vsatr_i_dn13, locals.var_vsatr_i_dn14, ) = (assign3220_e4495, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard21 != 0.0) {
            let assign3230_e4502: f64 = (locals.var_bin_l * p.p427);
            let assign3230_e4503: f64 = (p.p426 + assign3230_e4502);
            let assign3230_e4506: f64 = (locals.var_bin_w * p.p428);
            let assign3230_e4507: f64 = (assign3230_e4503 + assign3230_e4506);
            let assign3230_e4510: f64 = (locals.var_bin_wl * p.p429);
            let assign3230_e4511: f64 = (assign3230_e4507 + assign3230_e4510);
            locals.var_psatr_i = assign3230_e4511;
        }

        if (locals.var_guard21 != 0.0) {
            let assign3240_e4518: f64 = (locals.var_bin_l * p.p441);
            let assign3240_e4519: f64 = (p.p440 + assign3240_e4518);
            let assign3240_e4522: f64 = (locals.var_bin_w * p.p442);
            let assign3240_e4523: f64 = (assign3240_e4519 + assign3240_e4522);
            let assign3240_e4526: f64 = (locals.var_bin_wl * p.p443);
            let assign3240_e4527: f64 = (assign3240_e4523 + assign3240_e4526);
            (locals.var_ptwgr_i, locals.var_ptwgr_i_dn0, locals.var_ptwgr_i_dn2, locals.var_ptwgr_i_dn3, locals.var_ptwgr_i_dn4, locals.var_ptwgr_i_dn5, locals.var_ptwgr_i_dn6, locals.var_ptwgr_i_dn7, locals.var_ptwgr_i_dn8, locals.var_ptwgr_i_dn9, locals.var_ptwgr_i_dn10, locals.var_ptwgr_i_dn11, locals.var_ptwgr_i_dn12, locals.var_ptwgr_i_dn13, locals.var_ptwgr_i_dn14, ) = (assign3240_e4527, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard21 != 0.0) {
            let assign3250_e4534: f64 = (locals.var_bin_l * p.p526);
            let assign3250_e4535: f64 = (p.p525 + assign3250_e4534);
            let assign3250_e4538: f64 = (locals.var_bin_w * p.p527);
            let assign3250_e4539: f64 = (assign3250_e4535 + assign3250_e4538);
            let assign3250_e4542: f64 = (locals.var_bin_wl * p.p528);
            let assign3250_e4543: f64 = (assign3250_e4539 + assign3250_e4542);
            (locals.var_alpha0r_i, locals.var_alpha0r_i_dn0, locals.var_alpha0r_i_dn2, locals.var_alpha0r_i_dn3, locals.var_alpha0r_i_dn4, locals.var_alpha0r_i_dn5, locals.var_alpha0r_i_dn6, locals.var_alpha0r_i_dn7, locals.var_alpha0r_i_dn8, locals.var_alpha0r_i_dn9, locals.var_alpha0r_i_dn10, locals.var_alpha0r_i_dn11, locals.var_alpha0r_i_dn12, locals.var_alpha0r_i_dn13, locals.var_alpha0r_i_dn14, ) = (assign3250_e4543, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard21 != 0.0) {
            let assign3260_e4550: f64 = (locals.var_bin_l * p.p530);
            let assign3260_e4551: f64 = (p.p529 + assign3260_e4550);
            let assign3260_e4554: f64 = (locals.var_bin_w * p.p531);
            let assign3260_e4555: f64 = (assign3260_e4551 + assign3260_e4554);
            let assign3260_e4558: f64 = (locals.var_bin_wl * p.p532);
            let assign3260_e4559: f64 = (assign3260_e4555 + assign3260_e4558);
            locals.var_beta0r_i = assign3260_e4559;
        }

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
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign3270_e4584, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3280_e4588: f64 = (locals.var_inv_w).powf(p.p86);
        let assign3280_e4591: f64 = (locals.var_inv_wwide).powf(p.p86);
        let assign3280_e4592: f64 = (assign3280_e4588 - assign3280_e4591);
        let assign3280_e4594: f64 = (assign3280_e4592).max(0.0);
        let assign3280_e4595: f64 = (p.p85 * assign3280_e4594);
        let assign3280_e4599: f64 = (locals.var_inv_w * locals.var_inv_l);
        let assign3280_e4601: f64 = (assign3280_e4599).powf(p.p88);
        let assign3280_e4602: f64 = (p.p87 * assign3280_e4601);
        let assign3280_e4603: f64 = (assign3280_e4595 + assign3280_e4602);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign3280_e4603, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3290_e4607: f64 = (1.0 + locals.var_t0);
        let assign3290_e4609: f64 = (assign3290_e4607 + locals.var_t1);
        let assign3290_e4610: f64 = (locals.var_ndep_i * assign3290_e4609);
        (locals.var_ndep_i, locals.var_ndep_i_dn0, locals.var_ndep_i_dn2, locals.var_ndep_i_dn3, locals.var_ndep_i_dn4, locals.var_ndep_i_dn5, locals.var_ndep_i_dn6, locals.var_ndep_i_dn7, locals.var_ndep_i_dn8, locals.var_ndep_i_dn9, locals.var_ndep_i_dn10, locals.var_ndep_i_dn11, locals.var_ndep_i_dn12, locals.var_ndep_i_dn13, locals.var_ndep_i_dn14, ) = (assign3290_e4610, ((locals.var_ndep_i_dn0 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_ndep_i_dn2 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_ndep_i_dn3 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_ndep_i_dn4 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_ndep_i_dn5 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_ndep_i_dn6 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_ndep_i_dn7 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_ndep_i_dn8 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_ndep_i_dn9 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_ndep_i_dn10 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_ndep_i_dn11 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_ndep_i_dn12 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_ndep_i_dn13 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_ndep_i_dn14 * assign3290_e4609) + (locals.var_ndep_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );

        let assign3300_e4614: f64 = (locals.var_inv_l).powf(p.p215);
        let assign3300_e4617: f64 = (locals.var_inv_llong).powf(p.p215);
        let assign3300_e4618: f64 = (assign3300_e4614 - assign3300_e4617);
        let assign3300_e4620: f64 = (assign3300_e4618).max(0.0);
        let assign3300_e4621: f64 = (p.p214 * assign3300_e4620);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign3300_e4621, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3310_e4625: f64 = (locals.var_inv_w).powf(p.p217);
        let assign3310_e4628: f64 = (locals.var_inv_wwide).powf(p.p217);
        let assign3310_e4629: f64 = (assign3310_e4625 - assign3310_e4628);
        let assign3310_e4631: f64 = (assign3310_e4629).max(0.0);
        let assign3310_e4632: f64 = (p.p216 * assign3310_e4631);
        let assign3310_e4636: f64 = (locals.var_inv_wl).powf(p.p219);
        let assign3310_e4637: f64 = (p.p218 * assign3310_e4636);
        let assign3310_e4638: f64 = (assign3310_e4632 + assign3310_e4637);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign3310_e4638, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3320_e4642: f64 = (1.0 + locals.var_t0);
        let assign3320_e4644: f64 = (assign3320_e4642 + locals.var_t1);
        let assign3320_e4645: f64 = (locals.var_nfactor_i * assign3320_e4644);
        (locals.var_nfactor_i, locals.var_nfactor_i_dn0, locals.var_nfactor_i_dn2, locals.var_nfactor_i_dn3, locals.var_nfactor_i_dn4, locals.var_nfactor_i_dn5, locals.var_nfactor_i_dn6, locals.var_nfactor_i_dn7, locals.var_nfactor_i_dn8, locals.var_nfactor_i_dn9, locals.var_nfactor_i_dn10, locals.var_nfactor_i_dn11, locals.var_nfactor_i_dn12, locals.var_nfactor_i_dn13, locals.var_nfactor_i_dn14, ) = (assign3320_e4645, ((locals.var_nfactor_i_dn0 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_nfactor_i_dn2 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_nfactor_i_dn3 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_nfactor_i_dn4 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_nfactor_i_dn5 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_nfactor_i_dn6 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_nfactor_i_dn7 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_nfactor_i_dn8 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_nfactor_i_dn9 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_nfactor_i_dn10 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_nfactor_i_dn11 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_nfactor_i_dn12 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_nfactor_i_dn13 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_nfactor_i_dn14 * assign3320_e4644) + (locals.var_nfactor_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );

        let assign3330_e4650: f64 = (locals.var_inv_l).powf(p.p225);
        let assign3330_e4653: f64 = (locals.var_inv_llong).powf(p.p225);
        let assign3330_e4654: f64 = (assign3330_e4650 - assign3330_e4653);
        let assign3330_e4656: f64 = (assign3330_e4654).max(0.0);
        let assign3330_e4657: f64 = (p.p224 * assign3330_e4656);
        let assign3330_e4658: f64 = (1.0 + assign3330_e4657);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign3330_e4658, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3340_e4661: f64 = (locals.var_cdscd_i * locals.var_t0);
        (locals.var_cdscd_i, locals.var_cdscd_i_dn0, locals.var_cdscd_i_dn2, locals.var_cdscd_i_dn3, locals.var_cdscd_i_dn4, locals.var_cdscd_i_dn5, locals.var_cdscd_i_dn6, locals.var_cdscd_i_dn7, locals.var_cdscd_i_dn8, locals.var_cdscd_i_dn9, locals.var_cdscd_i_dn10, locals.var_cdscd_i_dn11, locals.var_cdscd_i_dn12, locals.var_cdscd_i_dn13, locals.var_cdscd_i_dn14, ) = (assign3340_e4661, ((locals.var_cdscd_i_dn0 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn0)), ((locals.var_cdscd_i_dn2 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn2)), ((locals.var_cdscd_i_dn3 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn3)), ((locals.var_cdscd_i_dn4 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn4)), ((locals.var_cdscd_i_dn5 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn5)), ((locals.var_cdscd_i_dn6 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn6)), ((locals.var_cdscd_i_dn7 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn7)), ((locals.var_cdscd_i_dn8 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn8)), ((locals.var_cdscd_i_dn9 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn9)), ((locals.var_cdscd_i_dn10 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn10)), ((locals.var_cdscd_i_dn11 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn11)), ((locals.var_cdscd_i_dn12 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn12)), ((locals.var_cdscd_i_dn13 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn13)), ((locals.var_cdscd_i_dn14 * locals.var_t0) + (locals.var_cdscd_i * locals.var_t0_dn14)), );

        let assign3350_e4664: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign3350_e4664;

        if (locals.var_guard22 != 0.0) {
            let assign3360_e4668: f64 = (locals.var_cdscdr_i * locals.var_t0);
            (locals.var_cdscdr_i, locals.var_cdscdr_i_dn0, locals.var_cdscdr_i_dn2, locals.var_cdscdr_i_dn3, locals.var_cdscdr_i_dn4, locals.var_cdscdr_i_dn5, locals.var_cdscdr_i_dn6, locals.var_cdscdr_i_dn7, locals.var_cdscdr_i_dn8, locals.var_cdscdr_i_dn9, locals.var_cdscdr_i_dn10, locals.var_cdscdr_i_dn11, locals.var_cdscdr_i_dn12, locals.var_cdscdr_i_dn13, locals.var_cdscdr_i_dn14, ) = (assign3360_e4668, ((locals.var_cdscdr_i_dn0 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn0)), ((locals.var_cdscdr_i_dn2 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn2)), ((locals.var_cdscdr_i_dn3 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn3)), ((locals.var_cdscdr_i_dn4 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn4)), ((locals.var_cdscdr_i_dn5 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn5)), ((locals.var_cdscdr_i_dn6 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn6)), ((locals.var_cdscdr_i_dn7 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn7)), ((locals.var_cdscdr_i_dn8 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn8)), ((locals.var_cdscdr_i_dn9 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn9)), ((locals.var_cdscdr_i_dn10 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn10)), ((locals.var_cdscdr_i_dn11 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn11)), ((locals.var_cdscdr_i_dn12 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn12)), ((locals.var_cdscdr_i_dn13 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn13)), ((locals.var_cdscdr_i_dn14 * locals.var_t0) + (locals.var_cdscdr_i * locals.var_t0_dn14)), );
        }

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

        if ((locals.var_guard23 != 0.0) && (locals.var_guard24 != 0.0)) {
            let assign3410_e4703: f64 = (locals.var_inv_l).powf(p.p275);
            let assign3410_e4706: f64 = (locals.var_inv_llong).powf(p.p275);
            let assign3410_e4707: f64 = (assign3410_e4703 - assign3410_e4706);
            let assign3410_e4709: f64 = (assign3410_e4707).max(0.0);
            let assign3410_e4710: f64 = (p.p274 * assign3410_e4709);
            let assign3410_e4711: f64 = (1.0 - assign3410_e4710);
            let assign3410_e4712: f64 = (locals.var_u0_i * assign3410_e4711);
            locals.var_u0_i = assign3410_e4712;
        }

        let assign3420_e4717: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard25 = assign3420_e4717;

        if (((locals.var_guard23 != 0.0) && (locals.var_guard24 != 0.0)) && (locals.var_guard25 != 0.0)) {
            let assign3430_e4728: f64 = (locals.var_inv_l).powf(p.p275);
            let assign3430_e4731: f64 = (locals.var_inv_llong).powf(p.p275);
            let assign3430_e4732: f64 = (assign3430_e4728 - assign3430_e4731);
            let assign3430_e4734: f64 = (assign3430_e4732).max(0.0);
            let assign3430_e4735: f64 = (p.p274 * assign3430_e4734);
            let assign3430_e4736: f64 = (1.0 - assign3430_e4735);
            let assign3430_e4737: f64 = (locals.var_u0r_i * assign3430_e4736);
            locals.var_u0r_i = assign3430_e4737;
        }

        if ((locals.var_guard23 != 0.0) && (locals.var_guard24 == 0.0)) {
            let assign3440_e4747: f64 = (1.0 - p.p274);
            let assign3440_e4748: f64 = (locals.var_u0_i * assign3440_e4747);
            locals.var_u0_i = assign3440_e4748;
        }

        let assign3450_e4753: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign3450_e4753;

        if (((locals.var_guard23 != 0.0) && (locals.var_guard24 == 0.0)) && (locals.var_guard26 != 0.0)) {
            let assign3460_e4763: f64 = (1.0 - p.p274);
            let assign3460_e4764: f64 = (locals.var_u0r_i * assign3460_e4763);
            locals.var_u0r_i = assign3460_e4764;
        }

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
            locals.var_u0_i = assign3470_e4787;
        }

        let assign3480_e4792: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard27 = assign3480_e4792;

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
            locals.var_u0r_i = assign3490_e4815;
        }

        let assign3500_e4821: f64 = (locals.var_inv_l).powf(p.p286);
        let assign3500_e4824: f64 = (locals.var_inv_llong).powf(p.p286);
        let assign3500_e4825: f64 = (assign3500_e4821 - assign3500_e4824);
        let assign3500_e4827: f64 = (assign3500_e4825).max(0.0);
        let assign3500_e4828: f64 = (p.p285 * assign3500_e4827);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign3500_e4828, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3510_e4832: f64 = (locals.var_inv_w).powf(p.p288);
        let assign3510_e4835: f64 = (locals.var_inv_wwide).powf(p.p288);
        let assign3510_e4836: f64 = (assign3510_e4832 - assign3510_e4835);
        let assign3510_e4838: f64 = (assign3510_e4836).max(0.0);
        let assign3510_e4839: f64 = (p.p287 * assign3510_e4838);
        let assign3510_e4843: f64 = (locals.var_inv_wl).powf(p.p290);
        let assign3510_e4844: f64 = (p.p289 * assign3510_e4843);
        let assign3510_e4845: f64 = (assign3510_e4839 + assign3510_e4844);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign3510_e4845, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3520_e4849: f64 = (1.0 + locals.var_t0);
        let assign3520_e4851: f64 = (assign3520_e4849 + locals.var_t1);
        let assign3520_e4852: f64 = (locals.var_ua_i * assign3520_e4851);
        (locals.var_ua_i, locals.var_ua_i_dn0, locals.var_ua_i_dn2, locals.var_ua_i_dn3, locals.var_ua_i_dn4, locals.var_ua_i_dn5, locals.var_ua_i_dn6, locals.var_ua_i_dn7, locals.var_ua_i_dn8, locals.var_ua_i_dn9, locals.var_ua_i_dn10, locals.var_ua_i_dn11, locals.var_ua_i_dn12, locals.var_ua_i_dn13, locals.var_ua_i_dn14, ) = (assign3520_e4852, ((locals.var_ua_i_dn0 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_ua_i_dn2 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_ua_i_dn3 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_ua_i_dn4 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_ua_i_dn5 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_ua_i_dn6 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_ua_i_dn7 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_ua_i_dn8 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_ua_i_dn9 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_ua_i_dn10 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_ua_i_dn11 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_ua_i_dn12 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_ua_i_dn13 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_ua_i_dn14 * assign3520_e4851) + (locals.var_ua_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );

        let assign3530_e4855: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard28 = assign3530_e4855;

        if (locals.var_guard28 != 0.0) {
            let assign3540_e4860: f64 = (1.0 + locals.var_t0);
            let assign3540_e4862: f64 = (assign3540_e4860 + locals.var_t1);
            let assign3540_e4863: f64 = (locals.var_uar_i * assign3540_e4862);
            (locals.var_uar_i, locals.var_uar_i_dn0, locals.var_uar_i_dn2, locals.var_uar_i_dn3, locals.var_uar_i_dn4, locals.var_uar_i_dn5, locals.var_uar_i_dn6, locals.var_uar_i_dn7, locals.var_uar_i_dn8, locals.var_uar_i_dn9, locals.var_uar_i_dn10, locals.var_uar_i_dn11, locals.var_uar_i_dn12, locals.var_uar_i_dn13, locals.var_uar_i_dn14, ) = (assign3540_e4863, ((locals.var_uar_i_dn0 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_uar_i_dn2 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_uar_i_dn3 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_uar_i_dn4 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_uar_i_dn5 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_uar_i_dn6 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_uar_i_dn7 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_uar_i_dn8 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_uar_i_dn9 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_uar_i_dn10 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_uar_i_dn11 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_uar_i_dn12 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_uar_i_dn13 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_uar_i_dn14 * assign3540_e4862) + (locals.var_uar_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );
        }

        let assign3550_e4869: f64 = (locals.var_inv_l).powf(p.p303);
        let assign3550_e4872: f64 = (locals.var_inv_llong).powf(p.p303);
        let assign3550_e4873: f64 = (assign3550_e4869 - assign3550_e4872);
        let assign3550_e4875: f64 = (assign3550_e4873).max(0.0);
        let assign3550_e4876: f64 = (p.p302 * assign3550_e4875);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign3550_e4876, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3560_e4880: f64 = (locals.var_inv_w).powf(p.p305);
        let assign3560_e4883: f64 = (locals.var_inv_wwide).powf(p.p305);
        let assign3560_e4884: f64 = (assign3560_e4880 - assign3560_e4883);
        let assign3560_e4886: f64 = (assign3560_e4884).max(0.0);
        let assign3560_e4887: f64 = (p.p304 * assign3560_e4886);
        let assign3560_e4891: f64 = (locals.var_inv_wl).powf(p.p307);
        let assign3560_e4892: f64 = (p.p306 * assign3560_e4891);
        let assign3560_e4893: f64 = (assign3560_e4887 + assign3560_e4892);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign3560_e4893, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3570_e4897: f64 = (1.0 + locals.var_t0);
        let assign3570_e4899: f64 = (assign3570_e4897 + locals.var_t1);
        let assign3570_e4900: f64 = (locals.var_eu_i * assign3570_e4899);
        (locals.var_eu_i, locals.var_eu_i_dn0, locals.var_eu_i_dn2, locals.var_eu_i_dn3, locals.var_eu_i_dn4, locals.var_eu_i_dn5, locals.var_eu_i_dn6, locals.var_eu_i_dn7, locals.var_eu_i_dn8, locals.var_eu_i_dn9, locals.var_eu_i_dn10, locals.var_eu_i_dn11, locals.var_eu_i_dn12, locals.var_eu_i_dn13, locals.var_eu_i_dn14, ) = (assign3570_e4900, ((locals.var_eu_i_dn0 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_eu_i_dn2 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_eu_i_dn3 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_eu_i_dn4 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_eu_i_dn5 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_eu_i_dn6 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_eu_i_dn7 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_eu_i_dn8 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_eu_i_dn9 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_eu_i_dn10 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_eu_i_dn11 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_eu_i_dn12 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_eu_i_dn13 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_eu_i_dn14 * assign3570_e4899) + (locals.var_eu_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );

        let assign3580_e4905: f64 = (locals.var_inv_l).powf(p.p310);
        let assign3580_e4908: f64 = (locals.var_inv_llong).powf(p.p310);
        let assign3580_e4909: f64 = (assign3580_e4905 - assign3580_e4908);
        let assign3580_e4911: f64 = (assign3580_e4909).max(0.0);
        let assign3580_e4912: f64 = (p.p309 * assign3580_e4911);
        let assign3580_e4913: f64 = (1.0 + assign3580_e4912);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign3580_e4913, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3590_e4916: f64 = (locals.var_ud_i * locals.var_t0);
        (locals.var_ud_i, locals.var_ud_i_dn0, locals.var_ud_i_dn2, locals.var_ud_i_dn3, locals.var_ud_i_dn4, locals.var_ud_i_dn5, locals.var_ud_i_dn6, locals.var_ud_i_dn7, locals.var_ud_i_dn8, locals.var_ud_i_dn9, locals.var_ud_i_dn10, locals.var_ud_i_dn11, locals.var_ud_i_dn12, locals.var_ud_i_dn13, locals.var_ud_i_dn14, ) = (assign3590_e4916, ((locals.var_ud_i_dn0 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn0)), ((locals.var_ud_i_dn2 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn2)), ((locals.var_ud_i_dn3 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn3)), ((locals.var_ud_i_dn4 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn4)), ((locals.var_ud_i_dn5 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn5)), ((locals.var_ud_i_dn6 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn6)), ((locals.var_ud_i_dn7 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn7)), ((locals.var_ud_i_dn8 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn8)), ((locals.var_ud_i_dn9 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn9)), ((locals.var_ud_i_dn10 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn10)), ((locals.var_ud_i_dn11 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn11)), ((locals.var_ud_i_dn12 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn12)), ((locals.var_ud_i_dn13 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn13)), ((locals.var_ud_i_dn14 * locals.var_t0) + (locals.var_ud_i * locals.var_t0_dn14)), );

        let assign3600_e4919: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign3600_e4919;

        if (locals.var_guard29 != 0.0) {
            let assign3610_e4923: f64 = (locals.var_udr_i * locals.var_t0);
            (locals.var_udr_i, locals.var_udr_i_dn0, locals.var_udr_i_dn2, locals.var_udr_i_dn3, locals.var_udr_i_dn4, locals.var_udr_i_dn5, locals.var_udr_i_dn6, locals.var_udr_i_dn7, locals.var_udr_i_dn8, locals.var_udr_i_dn9, locals.var_udr_i_dn10, locals.var_udr_i_dn11, locals.var_udr_i_dn12, locals.var_udr_i_dn13, locals.var_udr_i_dn14, ) = (assign3610_e4923, ((locals.var_udr_i_dn0 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn0)), ((locals.var_udr_i_dn2 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn2)), ((locals.var_udr_i_dn3 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn3)), ((locals.var_udr_i_dn4 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn4)), ((locals.var_udr_i_dn5 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn5)), ((locals.var_udr_i_dn6 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn6)), ((locals.var_udr_i_dn7 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn7)), ((locals.var_udr_i_dn8 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn8)), ((locals.var_udr_i_dn9 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn9)), ((locals.var_udr_i_dn10 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn10)), ((locals.var_udr_i_dn11 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn11)), ((locals.var_udr_i_dn12 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn12)), ((locals.var_udr_i_dn13 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn13)), ((locals.var_udr_i_dn14 * locals.var_t0) + (locals.var_udr_i * locals.var_t0_dn14)), );
        }

        let assign3620_e4929: f64 = (locals.var_inv_l).powf(p.p328);
        let assign3620_e4932: f64 = (locals.var_inv_llong).powf(p.p328);
        let assign3620_e4933: f64 = (assign3620_e4929 - assign3620_e4932);
        let assign3620_e4935: f64 = (assign3620_e4933).max(0.0);
        let assign3620_e4936: f64 = (p.p327 * assign3620_e4935);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign3620_e4936, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3630_e4940: f64 = (locals.var_inv_w).powf(p.p330);
        let assign3630_e4943: f64 = (locals.var_inv_wwide).powf(p.p330);
        let assign3630_e4944: f64 = (assign3630_e4940 - assign3630_e4943);
        let assign3630_e4946: f64 = (assign3630_e4944).max(0.0);
        let assign3630_e4947: f64 = (p.p329 * assign3630_e4946);
        let assign3630_e4951: f64 = (locals.var_inv_wl).powf(p.p332);
        let assign3630_e4952: f64 = (p.p331 * assign3630_e4951);
        let assign3630_e4953: f64 = (assign3630_e4947 + assign3630_e4952);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign3630_e4953, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3640_e4957: f64 = (1.0 + locals.var_t0);
        let assign3640_e4959: f64 = (assign3640_e4957 + locals.var_t1);
        let assign3640_e4960: f64 = (locals.var_uc_i * assign3640_e4959);
        (locals.var_uc_i, locals.var_uc_i_dn0, locals.var_uc_i_dn2, locals.var_uc_i_dn3, locals.var_uc_i_dn4, locals.var_uc_i_dn5, locals.var_uc_i_dn6, locals.var_uc_i_dn7, locals.var_uc_i_dn8, locals.var_uc_i_dn9, locals.var_uc_i_dn10, locals.var_uc_i_dn11, locals.var_uc_i_dn12, locals.var_uc_i_dn13, locals.var_uc_i_dn14, ) = (assign3640_e4960, ((locals.var_uc_i_dn0 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_uc_i_dn2 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_uc_i_dn3 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_uc_i_dn4 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_uc_i_dn5 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_uc_i_dn6 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_uc_i_dn7 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_uc_i_dn8 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_uc_i_dn9 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_uc_i_dn10 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_uc_i_dn11 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_uc_i_dn12 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_uc_i_dn13 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_uc_i_dn14 * assign3640_e4959) + (locals.var_uc_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );

        let assign3650_e4963: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign3650_e4963;

        if (locals.var_guard30 != 0.0) {
            let assign3660_e4968: f64 = (1.0 + locals.var_t0);
            let assign3660_e4970: f64 = (assign3660_e4968 + locals.var_t1);
            let assign3660_e4971: f64 = (locals.var_ucr_i * assign3660_e4970);
            (locals.var_ucr_i, locals.var_ucr_i_dn0, locals.var_ucr_i_dn2, locals.var_ucr_i_dn3, locals.var_ucr_i_dn4, locals.var_ucr_i_dn5, locals.var_ucr_i_dn6, locals.var_ucr_i_dn7, locals.var_ucr_i_dn8, locals.var_ucr_i_dn9, locals.var_ucr_i_dn10, locals.var_ucr_i_dn11, locals.var_ucr_i_dn12, locals.var_ucr_i_dn13, locals.var_ucr_i_dn14, ) = (assign3660_e4971, ((locals.var_ucr_i_dn0 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_ucr_i_dn2 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_ucr_i_dn3 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_ucr_i_dn4 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_ucr_i_dn5 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_ucr_i_dn6 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_ucr_i_dn7 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_ucr_i_dn8 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_ucr_i_dn9 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_ucr_i_dn10 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_ucr_i_dn11 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_ucr_i_dn12 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_ucr_i_dn13 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_ucr_i_dn14 * assign3660_e4970) + (locals.var_ucr_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );
        }

        let assign3670_e4976: f64 = (locals.var_inv_l).powf(p.p179);
        let assign3670_e4979: f64 = (locals.var_inv_llong).powf(p.p179);
        let assign3670_e4980: f64 = (assign3670_e4976 - assign3670_e4979);
        let assign3670_e4982: f64 = (assign3670_e4980).max(0.0);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign3670_e4982, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3680_e4985: f64 = (locals.var_eta0_i * locals.var_t0);
        (locals.var_eta0_i, locals.var_eta0_i_dn0, locals.var_eta0_i_dn2, locals.var_eta0_i_dn3, locals.var_eta0_i_dn4, locals.var_eta0_i_dn5, locals.var_eta0_i_dn6, locals.var_eta0_i_dn7, locals.var_eta0_i_dn8, locals.var_eta0_i_dn9, locals.var_eta0_i_dn10, locals.var_eta0_i_dn11, locals.var_eta0_i_dn12, locals.var_eta0_i_dn13, locals.var_eta0_i_dn14, ) = (assign3680_e4985, ((locals.var_eta0_i_dn0 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn0)), ((locals.var_eta0_i_dn2 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn2)), ((locals.var_eta0_i_dn3 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn3)), ((locals.var_eta0_i_dn4 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn4)), ((locals.var_eta0_i_dn5 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn5)), ((locals.var_eta0_i_dn6 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn6)), ((locals.var_eta0_i_dn7 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn7)), ((locals.var_eta0_i_dn8 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn8)), ((locals.var_eta0_i_dn9 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn9)), ((locals.var_eta0_i_dn10 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn10)), ((locals.var_eta0_i_dn11 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn11)), ((locals.var_eta0_i_dn12 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn12)), ((locals.var_eta0_i_dn13 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn13)), ((locals.var_eta0_i_dn14 * locals.var_t0) + (locals.var_eta0_i * locals.var_t0_dn14)), );

        let assign3690_e4988: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign3690_e4988;

        if (locals.var_guard31 != 0.0) {
            let assign3700_e4992: f64 = (locals.var_eta0r_i * locals.var_t0);
            (locals.var_eta0r_i, locals.var_eta0r_i_dn0, locals.var_eta0r_i_dn2, locals.var_eta0r_i_dn3, locals.var_eta0r_i_dn4, locals.var_eta0r_i_dn5, locals.var_eta0r_i_dn6, locals.var_eta0r_i_dn7, locals.var_eta0r_i_dn8, locals.var_eta0r_i_dn9, locals.var_eta0r_i_dn10, locals.var_eta0r_i_dn11, locals.var_eta0r_i_dn12, locals.var_eta0r_i_dn13, locals.var_eta0r_i_dn14, ) = (assign3700_e4992, ((locals.var_eta0r_i_dn0 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn0)), ((locals.var_eta0r_i_dn2 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn2)), ((locals.var_eta0r_i_dn3 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn3)), ((locals.var_eta0r_i_dn4 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn4)), ((locals.var_eta0r_i_dn5 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn5)), ((locals.var_eta0r_i_dn6 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn6)), ((locals.var_eta0r_i_dn7 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn7)), ((locals.var_eta0r_i_dn8 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn8)), ((locals.var_eta0r_i_dn9 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn9)), ((locals.var_eta0r_i_dn10 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn10)), ((locals.var_eta0r_i_dn11 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn11)), ((locals.var_eta0r_i_dn12 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn12)), ((locals.var_eta0r_i_dn13 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn13)), ((locals.var_eta0r_i_dn14 * locals.var_t0) + (locals.var_eta0r_i * locals.var_t0_dn14)), );
        }

        let assign3710_e4998: f64 = (locals.var_inv_l).powf(p.p181);
        let assign3710_e5001: f64 = (locals.var_inv_llong).powf(p.p181);
        let assign3710_e5002: f64 = (assign3710_e4998 - assign3710_e5001);
        let assign3710_e5004: f64 = (assign3710_e5002).max(0.0);
        let assign3710_e5005: f64 = (locals.var_etab_i * assign3710_e5004);
        locals.var_etab_i = assign3710_e5005;

    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3720_e5010: f64 = (locals.var_inv_l).powf(p.p462);
        let assign3720_e5013: f64 = (locals.var_inv_llong).powf(p.p462);
        let assign3720_e5014: f64 = (assign3720_e5010 - assign3720_e5013);
        let assign3720_e5016: f64 = (assign3720_e5014).max(0.0);
        let assign3720_e5017: f64 = (p.p461 * assign3720_e5016);
        let assign3720_e5018: f64 = (1.0 + assign3720_e5017);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign3720_e5018, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3730_e5021: f64 = (locals.var_pdiblc_i * locals.var_t0);
        (locals.var_pdiblc_i, locals.var_pdiblc_i_dn0, locals.var_pdiblc_i_dn2, locals.var_pdiblc_i_dn3, locals.var_pdiblc_i_dn4, locals.var_pdiblc_i_dn5, locals.var_pdiblc_i_dn6, locals.var_pdiblc_i_dn7, locals.var_pdiblc_i_dn8, locals.var_pdiblc_i_dn9, locals.var_pdiblc_i_dn10, locals.var_pdiblc_i_dn11, locals.var_pdiblc_i_dn12, locals.var_pdiblc_i_dn13, locals.var_pdiblc_i_dn14, ) = (assign3730_e5021, ((locals.var_pdiblc_i_dn0 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn0)), ((locals.var_pdiblc_i_dn2 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn2)), ((locals.var_pdiblc_i_dn3 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn3)), ((locals.var_pdiblc_i_dn4 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn4)), ((locals.var_pdiblc_i_dn5 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn5)), ((locals.var_pdiblc_i_dn6 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn6)), ((locals.var_pdiblc_i_dn7 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn7)), ((locals.var_pdiblc_i_dn8 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn8)), ((locals.var_pdiblc_i_dn9 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn9)), ((locals.var_pdiblc_i_dn10 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn10)), ((locals.var_pdiblc_i_dn11 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn11)), ((locals.var_pdiblc_i_dn12 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn12)), ((locals.var_pdiblc_i_dn13 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn13)), ((locals.var_pdiblc_i_dn14 * locals.var_t0) + (locals.var_pdiblc_i * locals.var_t0_dn14)), );

        let assign3740_e5024: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign3740_e5024;

        if (locals.var_guard32 != 0.0) {
            let assign3750_e5028: f64 = (locals.var_pdiblcr_i * locals.var_t0);
            (locals.var_pdiblcr_i, locals.var_pdiblcr_i_dn0, locals.var_pdiblcr_i_dn2, locals.var_pdiblcr_i_dn3, locals.var_pdiblcr_i_dn4, locals.var_pdiblcr_i_dn5, locals.var_pdiblcr_i_dn6, locals.var_pdiblcr_i_dn7, locals.var_pdiblcr_i_dn8, locals.var_pdiblcr_i_dn9, locals.var_pdiblcr_i_dn10, locals.var_pdiblcr_i_dn11, locals.var_pdiblcr_i_dn12, locals.var_pdiblcr_i_dn13, locals.var_pdiblcr_i_dn14, ) = (assign3750_e5028, ((locals.var_pdiblcr_i_dn0 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn0)), ((locals.var_pdiblcr_i_dn2 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn2)), ((locals.var_pdiblcr_i_dn3 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn3)), ((locals.var_pdiblcr_i_dn4 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn4)), ((locals.var_pdiblcr_i_dn5 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn5)), ((locals.var_pdiblcr_i_dn6 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn6)), ((locals.var_pdiblcr_i_dn7 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn7)), ((locals.var_pdiblcr_i_dn8 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn8)), ((locals.var_pdiblcr_i_dn9 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn9)), ((locals.var_pdiblcr_i_dn10 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn10)), ((locals.var_pdiblcr_i_dn11 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn11)), ((locals.var_pdiblcr_i_dn12 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn12)), ((locals.var_pdiblcr_i_dn13 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn13)), ((locals.var_pdiblcr_i_dn14 * locals.var_t0) + (locals.var_pdiblcr_i * locals.var_t0_dn14)), );
        }

        let assign3760_e5036: f64 = (locals.var_inv_l).powf(p.p258);
        let assign3760_e5039: f64 = (locals.var_inv_llong).powf(p.p258);
        let assign3760_e5040: f64 = (assign3760_e5036 - assign3760_e5039);
        let assign3760_e5042: f64 = (assign3760_e5040).max(0.0);
        let assign3760_e5043: f64 = (p.p257 * assign3760_e5042);
        let assign3760_e5044: f64 = (1.0 + assign3760_e5043);
        let assign3760_e5045: f64 = (locals.var_delta_i * assign3760_e5044);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign3760_e5045, (locals.var_delta_i_dn0 * assign3760_e5044), (locals.var_delta_i_dn2 * assign3760_e5044), (locals.var_delta_i_dn3 * assign3760_e5044), (locals.var_delta_i_dn4 * assign3760_e5044), (locals.var_delta_i_dn5 * assign3760_e5044), (locals.var_delta_i_dn6 * assign3760_e5044), (locals.var_delta_i_dn7 * assign3760_e5044), (locals.var_delta_i_dn8 * assign3760_e5044), (locals.var_delta_i_dn9 * assign3760_e5044), (locals.var_delta_i_dn10 * assign3760_e5044), (locals.var_delta_i_dn11 * assign3760_e5044), (locals.var_delta_i_dn12 * assign3760_e5044), (locals.var_delta_i_dn13 * assign3760_e5044), (locals.var_delta_i_dn14 * assign3760_e5044), );

        let assign3770_e5048: f64 = (locals.var_t0).min(0.5);
        (locals.var_delta_i, locals.var_delta_i_dn0, locals.var_delta_i_dn2, locals.var_delta_i_dn3, locals.var_delta_i_dn4, locals.var_delta_i_dn5, locals.var_delta_i_dn6, locals.var_delta_i_dn7, locals.var_delta_i_dn8, locals.var_delta_i_dn9, locals.var_delta_i_dn10, locals.var_delta_i_dn11, locals.var_delta_i_dn12, locals.var_delta_i_dn13, locals.var_delta_i_dn14, ) = (assign3770_e5048, if locals.var_t0 <= 0.5 { locals.var_t0_dn0 } else { 0.0 }, if locals.var_t0 <= 0.5 { locals.var_t0_dn2 } else { 0.0 }, if locals.var_t0 <= 0.5 { locals.var_t0_dn3 } else { 0.0 }, if locals.var_t0 <= 0.5 { locals.var_t0_dn4 } else { 0.0 }, if locals.var_t0 <= 0.5 { locals.var_t0_dn5 } else { 0.0 }, if locals.var_t0 <= 0.5 { locals.var_t0_dn6 } else { 0.0 }, if locals.var_t0 <= 0.5 { locals.var_t0_dn7 } else { 0.0 }, if locals.var_t0 <= 0.5 { locals.var_t0_dn8 } else { 0.0 }, if locals.var_t0 <= 0.5 { locals.var_t0_dn9 } else { 0.0 }, if locals.var_t0 <= 0.5 { locals.var_t0_dn10 } else { 0.0 }, if locals.var_t0 <= 0.5 { locals.var_t0_dn11 } else { 0.0 }, if locals.var_t0 <= 0.5 { locals.var_t0_dn12 } else { 0.0 }, if locals.var_t0 <= 0.5 { locals.var_t0_dn13 } else { 0.0 }, if locals.var_t0 <= 0.5 { locals.var_t0_dn14 } else { 0.0 }, );

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
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign3790_e5076, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3800_e5079: f64 = (locals.var_pclm_i * locals.var_t0);
        (locals.var_pclm_i, locals.var_pclm_i_dn0, locals.var_pclm_i_dn2, locals.var_pclm_i_dn3, locals.var_pclm_i_dn4, locals.var_pclm_i_dn5, locals.var_pclm_i_dn6, locals.var_pclm_i_dn7, locals.var_pclm_i_dn8, locals.var_pclm_i_dn9, locals.var_pclm_i_dn10, locals.var_pclm_i_dn11, locals.var_pclm_i_dn12, locals.var_pclm_i_dn13, locals.var_pclm_i_dn14, ) = (assign3800_e5079, ((locals.var_pclm_i_dn0 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn0)), ((locals.var_pclm_i_dn2 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn2)), ((locals.var_pclm_i_dn3 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn3)), ((locals.var_pclm_i_dn4 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn4)), ((locals.var_pclm_i_dn5 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn5)), ((locals.var_pclm_i_dn6 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn6)), ((locals.var_pclm_i_dn7 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn7)), ((locals.var_pclm_i_dn8 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn8)), ((locals.var_pclm_i_dn9 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn9)), ((locals.var_pclm_i_dn10 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn10)), ((locals.var_pclm_i_dn11 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn11)), ((locals.var_pclm_i_dn12 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn12)), ((locals.var_pclm_i_dn13 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn13)), ((locals.var_pclm_i_dn14 * locals.var_t0) + (locals.var_pclm_i * locals.var_t0_dn14)), );

        let assign3810_e5082: f64 = (locals.var_pclm_i).max(0.0);
        (locals.var_pclm_i, locals.var_pclm_i_dn0, locals.var_pclm_i_dn2, locals.var_pclm_i_dn3, locals.var_pclm_i_dn4, locals.var_pclm_i_dn5, locals.var_pclm_i_dn6, locals.var_pclm_i_dn7, locals.var_pclm_i_dn8, locals.var_pclm_i_dn9, locals.var_pclm_i_dn10, locals.var_pclm_i_dn11, locals.var_pclm_i_dn12, locals.var_pclm_i_dn13, locals.var_pclm_i_dn14, ) = (assign3810_e5082, if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn0 } else { 0.0 }, if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn2 } else { 0.0 }, if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn3 } else { 0.0 }, if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn4 } else { 0.0 }, if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn5 } else { 0.0 }, if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn6 } else { 0.0 }, if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn7 } else { 0.0 }, if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn8 } else { 0.0 }, if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn9 } else { 0.0 }, if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn10 } else { 0.0 }, if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn11 } else { 0.0 }, if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn12 } else { 0.0 }, if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn13 } else { 0.0 }, if locals.var_pclm_i >= 0.0 { locals.var_pclm_i_dn14 } else { 0.0 }, );

        let assign3820_e5085: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign3820_e5085;

        if (locals.var_guard33 != 0.0) {
            let assign3830_e5089: f64 = (locals.var_pclmr_i * locals.var_t0);
            (locals.var_pclmr_i, locals.var_pclmr_i_dn0, locals.var_pclmr_i_dn2, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11, locals.var_pclmr_i_dn12, locals.var_pclmr_i_dn13, locals.var_pclmr_i_dn14, ) = (assign3830_e5089, ((locals.var_pclmr_i_dn0 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn0)), ((locals.var_pclmr_i_dn2 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn2)), ((locals.var_pclmr_i_dn3 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn3)), ((locals.var_pclmr_i_dn4 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn4)), ((locals.var_pclmr_i_dn5 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn5)), ((locals.var_pclmr_i_dn6 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn6)), ((locals.var_pclmr_i_dn7 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn7)), ((locals.var_pclmr_i_dn8 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn8)), ((locals.var_pclmr_i_dn9 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn9)), ((locals.var_pclmr_i_dn10 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn10)), ((locals.var_pclmr_i_dn11 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn11)), ((locals.var_pclmr_i_dn12 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn12)), ((locals.var_pclmr_i_dn13 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn13)), ((locals.var_pclmr_i_dn14 * locals.var_t0) + (locals.var_pclmr_i * locals.var_t0_dn14)), );
        }

        if (locals.var_guard33 != 0.0) {
            let assign3840_e5095: f64 = (locals.var_pclmr_i).max(0.0);
            (locals.var_pclmr_i, locals.var_pclmr_i_dn0, locals.var_pclmr_i_dn2, locals.var_pclmr_i_dn3, locals.var_pclmr_i_dn4, locals.var_pclmr_i_dn5, locals.var_pclmr_i_dn6, locals.var_pclmr_i_dn7, locals.var_pclmr_i_dn8, locals.var_pclmr_i_dn9, locals.var_pclmr_i_dn10, locals.var_pclmr_i_dn11, locals.var_pclmr_i_dn12, locals.var_pclmr_i_dn13, locals.var_pclmr_i_dn14, ) = (assign3840_e5095, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn0 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn2 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn3 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn4 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn5 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn6 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn7 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn8 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn9 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn10 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn11 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn12 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn13 } else { 0.0 }, if locals.var_pclmr_i >= 0.0 { locals.var_pclmr_i_dn14 } else { 0.0 }, );
        }

        let assign3850_e5101: f64 = (locals.var_inv_l).powf(p.p244);
        let assign3850_e5104: f64 = (locals.var_inv_llong).powf(p.p244);
        let assign3850_e5105: f64 = (assign3850_e5101 - assign3850_e5104);
        let assign3850_e5107: f64 = (assign3850_e5105).max(0.0);
        let assign3850_e5108: f64 = (p.p243 * assign3850_e5107);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign3850_e5108, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3860_e5112: f64 = (locals.var_inv_w).powf(p.p246);
        let assign3860_e5115: f64 = (locals.var_inv_wwide).powf(p.p246);
        let assign3860_e5116: f64 = (assign3860_e5112 - assign3860_e5115);
        let assign3860_e5118: f64 = (assign3860_e5116).max(0.0);
        let assign3860_e5119: f64 = (p.p245 * assign3860_e5118);
        let assign3860_e5123: f64 = (locals.var_inv_wl).powf(p.p248);
        let assign3860_e5124: f64 = (p.p247 * assign3860_e5123);
        let assign3860_e5125: f64 = (assign3860_e5119 + assign3860_e5124);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign3860_e5125, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3870_e5129: f64 = (1.0 + locals.var_t0);
        let assign3870_e5131: f64 = (assign3870_e5129 + locals.var_t1);
        let assign3870_e5132: f64 = (locals.var_vsat_i * assign3870_e5131);
        (locals.var_vsat_i, locals.var_vsat_i_dn0, locals.var_vsat_i_dn2, locals.var_vsat_i_dn3, locals.var_vsat_i_dn4, locals.var_vsat_i_dn5, locals.var_vsat_i_dn6, locals.var_vsat_i_dn7, locals.var_vsat_i_dn8, locals.var_vsat_i_dn9, locals.var_vsat_i_dn10, locals.var_vsat_i_dn11, locals.var_vsat_i_dn12, locals.var_vsat_i_dn13, locals.var_vsat_i_dn14, ) = (assign3870_e5132, ((locals.var_vsat_i_dn0 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_vsat_i_dn2 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_vsat_i_dn3 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_vsat_i_dn4 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_vsat_i_dn5 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_vsat_i_dn6 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_vsat_i_dn7 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_vsat_i_dn8 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_vsat_i_dn9 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_vsat_i_dn10 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_vsat_i_dn11 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_vsat_i_dn12 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_vsat_i_dn13 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_vsat_i_dn14 * assign3870_e5131) + (locals.var_vsat_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );

        let assign3880_e5135: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign3880_e5135;

        if (locals.var_guard34 != 0.0) {
            let assign3890_e5140: f64 = (1.0 + locals.var_t0);
            let assign3890_e5142: f64 = (assign3890_e5140 + locals.var_t1);
            let assign3890_e5143: f64 = (locals.var_vsatr_i * assign3890_e5142);
            (locals.var_vsatr_i, locals.var_vsatr_i_dn0, locals.var_vsatr_i_dn2, locals.var_vsatr_i_dn3, locals.var_vsatr_i_dn4, locals.var_vsatr_i_dn5, locals.var_vsatr_i_dn6, locals.var_vsatr_i_dn7, locals.var_vsatr_i_dn8, locals.var_vsatr_i_dn9, locals.var_vsatr_i_dn10, locals.var_vsatr_i_dn11, locals.var_vsatr_i_dn12, locals.var_vsatr_i_dn13, locals.var_vsatr_i_dn14, ) = (assign3890_e5143, ((locals.var_vsatr_i_dn0 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_vsatr_i_dn2 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_vsatr_i_dn3 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_vsatr_i_dn4 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_vsatr_i_dn5 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_vsatr_i_dn6 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_vsatr_i_dn7 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_vsatr_i_dn8 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_vsatr_i_dn9 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_vsatr_i_dn10 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_vsatr_i_dn11 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_vsatr_i_dn12 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_vsatr_i_dn13 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_vsatr_i_dn14 * assign3890_e5142) + (locals.var_vsatr_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );
        }

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

        if (locals.var_guard35 != 0.0) {
            let assign3920_e5172: f64 = (locals.var_inv_l).powf(p.p424);
            let assign3920_e5175: f64 = (locals.var_inv_llong).powf(p.p424);
            let assign3920_e5176: f64 = (assign3920_e5172 - assign3920_e5175);
            let assign3920_e5178: f64 = (assign3920_e5176).max(0.0);
            let assign3920_e5179: f64 = (p.p423 * assign3920_e5178);
            let assign3920_e5180: f64 = (1.0 + assign3920_e5179);
            let assign3920_e5181: f64 = (locals.var_psatr_i * assign3920_e5180);
            let assign3920_e5183: f64 = (assign3920_e5181).max(0.25);
            locals.var_psatr_i = assign3920_e5183;
        }

        let assign3930_e5190: f64 = (locals.var_inv_l).powf(p.p439);
        let assign3930_e5193: f64 = (locals.var_inv_llong).powf(p.p439);
        let assign3930_e5194: f64 = (assign3930_e5190 - assign3930_e5193);
        let assign3930_e5196: f64 = (assign3930_e5194).max(0.0);
        let assign3930_e5197: f64 = (p.p438 * assign3930_e5196);
        let assign3930_e5198: f64 = (1.0 + assign3930_e5197);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign3930_e5198, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3940_e5201: f64 = (locals.var_ptwg_i * locals.var_t0);
        (locals.var_ptwg_i, locals.var_ptwg_i_dn0, locals.var_ptwg_i_dn2, locals.var_ptwg_i_dn3, locals.var_ptwg_i_dn4, locals.var_ptwg_i_dn5, locals.var_ptwg_i_dn6, locals.var_ptwg_i_dn7, locals.var_ptwg_i_dn8, locals.var_ptwg_i_dn9, locals.var_ptwg_i_dn10, locals.var_ptwg_i_dn11, locals.var_ptwg_i_dn12, locals.var_ptwg_i_dn13, locals.var_ptwg_i_dn14, ) = (assign3940_e5201, ((locals.var_ptwg_i_dn0 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn0)), ((locals.var_ptwg_i_dn2 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn2)), ((locals.var_ptwg_i_dn3 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn3)), ((locals.var_ptwg_i_dn4 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn4)), ((locals.var_ptwg_i_dn5 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn5)), ((locals.var_ptwg_i_dn6 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn6)), ((locals.var_ptwg_i_dn7 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn7)), ((locals.var_ptwg_i_dn8 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn8)), ((locals.var_ptwg_i_dn9 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn9)), ((locals.var_ptwg_i_dn10 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn10)), ((locals.var_ptwg_i_dn11 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn11)), ((locals.var_ptwg_i_dn12 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn12)), ((locals.var_ptwg_i_dn13 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn13)), ((locals.var_ptwg_i_dn14 * locals.var_t0) + (locals.var_ptwg_i * locals.var_t0_dn14)), );

        let assign3950_e5204: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign3950_e5204;

        if (locals.var_guard36 != 0.0) {
            let assign3960_e5208: f64 = (locals.var_ptwgr_i * locals.var_t0);
            (locals.var_ptwgr_i, locals.var_ptwgr_i_dn0, locals.var_ptwgr_i_dn2, locals.var_ptwgr_i_dn3, locals.var_ptwgr_i_dn4, locals.var_ptwgr_i_dn5, locals.var_ptwgr_i_dn6, locals.var_ptwgr_i_dn7, locals.var_ptwgr_i_dn8, locals.var_ptwgr_i_dn9, locals.var_ptwgr_i_dn10, locals.var_ptwgr_i_dn11, locals.var_ptwgr_i_dn12, locals.var_ptwgr_i_dn13, locals.var_ptwgr_i_dn14, ) = (assign3960_e5208, ((locals.var_ptwgr_i_dn0 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn0)), ((locals.var_ptwgr_i_dn2 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn2)), ((locals.var_ptwgr_i_dn3 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn3)), ((locals.var_ptwgr_i_dn4 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn4)), ((locals.var_ptwgr_i_dn5 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn5)), ((locals.var_ptwgr_i_dn6 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn6)), ((locals.var_ptwgr_i_dn7 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn7)), ((locals.var_ptwgr_i_dn8 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn8)), ((locals.var_ptwgr_i_dn9 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn9)), ((locals.var_ptwgr_i_dn10 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn10)), ((locals.var_ptwgr_i_dn11 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn11)), ((locals.var_ptwgr_i_dn12 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn12)), ((locals.var_ptwgr_i_dn13 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn13)), ((locals.var_ptwgr_i_dn14 * locals.var_t0) + (locals.var_ptwgr_i * locals.var_t0_dn14)), );
        }

        let assign3970_e5214: f64 = (locals.var_inv_l).powf(p.p486);
        let assign3970_e5217: f64 = (locals.var_inv_llong).powf(p.p486);
        let assign3970_e5218: f64 = (assign3970_e5214 - assign3970_e5217);
        let assign3970_e5220: f64 = (assign3970_e5218).max(0.0);
        let assign3970_e5221: f64 = (p.p485 * assign3970_e5220);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign3970_e5221, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3980_e5225: f64 = (locals.var_inv_w).powf(p.p488);
        let assign3980_e5228: f64 = (locals.var_inv_wwide).powf(p.p488);
        let assign3980_e5229: f64 = (assign3980_e5225 - assign3980_e5228);
        let assign3980_e5231: f64 = (assign3980_e5229).max(0.0);
        let assign3980_e5232: f64 = (p.p487 * assign3980_e5231);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign3980_e5232, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign3990_e5236: f64 = (1.0 + locals.var_t0);
        let assign3990_e5238: f64 = (assign3990_e5236 + locals.var_t1);
        let assign3990_e5239: f64 = (locals.var_alpha0_i * assign3990_e5238);
        (locals.var_alpha0_i, locals.var_alpha0_i_dn0, locals.var_alpha0_i_dn2, locals.var_alpha0_i_dn3, locals.var_alpha0_i_dn4, locals.var_alpha0_i_dn5, locals.var_alpha0_i_dn6, locals.var_alpha0_i_dn7, locals.var_alpha0_i_dn8, locals.var_alpha0_i_dn9, locals.var_alpha0_i_dn10, locals.var_alpha0_i_dn11, locals.var_alpha0_i_dn12, locals.var_alpha0_i_dn13, locals.var_alpha0_i_dn14, ) = (assign3990_e5239, ((locals.var_alpha0_i_dn0 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_alpha0_i_dn2 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_alpha0_i_dn3 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_alpha0_i_dn4 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_alpha0_i_dn5 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_alpha0_i_dn6 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_alpha0_i_dn7 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_alpha0_i_dn8 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_alpha0_i_dn9 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_alpha0_i_dn10 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_alpha0_i_dn11 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_alpha0_i_dn12 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_alpha0_i_dn13 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_alpha0_i_dn14 * assign3990_e5238) + (locals.var_alpha0_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );

        let assign4000_e5242: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign4000_e5242;

        if (locals.var_guard37 != 0.0) {
            let assign4010_e5247: f64 = (1.0 + locals.var_t0);
            let assign4010_e5249: f64 = (assign4010_e5247 + locals.var_t1);
            let assign4010_e5250: f64 = (locals.var_alpha0r_i * assign4010_e5249);
            (locals.var_alpha0r_i, locals.var_alpha0r_i_dn0, locals.var_alpha0r_i_dn2, locals.var_alpha0r_i_dn3, locals.var_alpha0r_i_dn4, locals.var_alpha0r_i_dn5, locals.var_alpha0r_i_dn6, locals.var_alpha0r_i_dn7, locals.var_alpha0r_i_dn8, locals.var_alpha0r_i_dn9, locals.var_alpha0r_i_dn10, locals.var_alpha0r_i_dn11, locals.var_alpha0r_i_dn12, locals.var_alpha0r_i_dn13, locals.var_alpha0r_i_dn14, ) = (assign4010_e5250, ((locals.var_alpha0r_i_dn0 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_alpha0r_i_dn2 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_alpha0r_i_dn3 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_alpha0r_i_dn4 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_alpha0r_i_dn5 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_alpha0r_i_dn6 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_alpha0r_i_dn7 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_alpha0r_i_dn8 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_alpha0r_i_dn9 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_alpha0r_i_dn10 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_alpha0r_i_dn11 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_alpha0r_i_dn12 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_alpha0r_i_dn13 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_alpha0r_i_dn14 * assign4010_e5249) + (locals.var_alpha0r_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );
        }

        let assign4020_e5256: f64 = (locals.var_inv_w).powf(p.p496);
        let assign4020_e5259: f64 = (locals.var_inv_wwide).powf(p.p496);
        let assign4020_e5260: f64 = (assign4020_e5256 - assign4020_e5259);
        let assign4020_e5262: f64 = (assign4020_e5260).max(0.0);
        let assign4020_e5263: f64 = (p.p495 * assign4020_e5262);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4020_e5263, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign4030_e5267: f64 = (1.0 + locals.var_t1);
        let assign4030_e5268: f64 = (locals.var_beta0_i * assign4030_e5267);
        (locals.var_beta0_i, locals.var_beta0_i_dn0, locals.var_beta0_i_dn2, locals.var_beta0_i_dn3, locals.var_beta0_i_dn4, locals.var_beta0_i_dn5, locals.var_beta0_i_dn6, locals.var_beta0_i_dn7, locals.var_beta0_i_dn8, locals.var_beta0_i_dn9, locals.var_beta0_i_dn10, locals.var_beta0_i_dn11, locals.var_beta0_i_dn12, locals.var_beta0_i_dn13, locals.var_beta0_i_dn14, ) = (assign4030_e5268, ((locals.var_beta0_i_dn0 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn0)), ((locals.var_beta0_i_dn2 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn2)), ((locals.var_beta0_i_dn3 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn3)), ((locals.var_beta0_i_dn4 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn4)), ((locals.var_beta0_i_dn5 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn5)), ((locals.var_beta0_i_dn6 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn6)), ((locals.var_beta0_i_dn7 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn7)), ((locals.var_beta0_i_dn8 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn8)), ((locals.var_beta0_i_dn9 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn9)), ((locals.var_beta0_i_dn10 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn10)), ((locals.var_beta0_i_dn11 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn11)), ((locals.var_beta0_i_dn12 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn12)), ((locals.var_beta0_i_dn13 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn13)), ((locals.var_beta0_i_dn14 * assign4030_e5267) + (locals.var_beta0_i * locals.var_t1_dn14)), );

        let assign4040_e5272: f64 = (locals.var_inv_w).powf(p.p520);
        let assign4040_e5275: f64 = (locals.var_inv_wwide).powf(p.p520);
        let assign4040_e5276: f64 = (assign4040_e5272 - assign4040_e5275);
        let assign4040_e5278: f64 = (assign4040_e5276).max(0.0);
        let assign4040_e5279: f64 = (p.p519 * assign4040_e5278);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4040_e5279, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_beta1_i, locals.var_beta1_i_dn0, locals.var_beta1_i_dn2, locals.var_beta1_i_dn3, locals.var_beta1_i_dn4, locals.var_beta1_i_dn5, locals.var_beta1_i_dn6, locals.var_beta1_i_dn7, locals.var_beta1_i_dn8, locals.var_beta1_i_dn9, locals.var_beta1_i_dn10, locals.var_beta1_i_dn11, locals.var_beta1_i_dn12, locals.var_beta1_i_dn13, locals.var_beta1_i_dn14, ) = (p.p518, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign4060_e5284: f64 = (1.0 + locals.var_t1);
        let assign4060_e5285: f64 = (locals.var_beta1_i * assign4060_e5284);
        (locals.var_beta1_i, locals.var_beta1_i_dn0, locals.var_beta1_i_dn2, locals.var_beta1_i_dn3, locals.var_beta1_i_dn4, locals.var_beta1_i_dn5, locals.var_beta1_i_dn6, locals.var_beta1_i_dn7, locals.var_beta1_i_dn8, locals.var_beta1_i_dn9, locals.var_beta1_i_dn10, locals.var_beta1_i_dn11, locals.var_beta1_i_dn12, locals.var_beta1_i_dn13, locals.var_beta1_i_dn14, ) = (assign4060_e5285, ((locals.var_beta1_i_dn0 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn0)), ((locals.var_beta1_i_dn2 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn2)), ((locals.var_beta1_i_dn3 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn3)), ((locals.var_beta1_i_dn4 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn4)), ((locals.var_beta1_i_dn5 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn5)), ((locals.var_beta1_i_dn6 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn6)), ((locals.var_beta1_i_dn7 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn7)), ((locals.var_beta1_i_dn8 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn8)), ((locals.var_beta1_i_dn9 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn9)), ((locals.var_beta1_i_dn10 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn10)), ((locals.var_beta1_i_dn11 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn11)), ((locals.var_beta1_i_dn12 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn12)), ((locals.var_beta1_i_dn13 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn13)), ((locals.var_beta1_i_dn14 * assign4060_e5284) + (locals.var_beta1_i * locals.var_t1_dn14)), );

        let assign4070_e5289: f64 = (locals.var_inv_w).powf(p.p523);
        let assign4070_e5292: f64 = (locals.var_inv_wwide).powf(p.p523);
        let assign4070_e5293: f64 = (assign4070_e5289 - assign4070_e5292);
        let assign4070_e5295: f64 = (assign4070_e5293).max(0.0);
        let assign4070_e5296: f64 = (p.p522 * assign4070_e5295);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4070_e5296, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_beta2_i, locals.var_beta2_i_dn0, locals.var_beta2_i_dn2, locals.var_beta2_i_dn3, locals.var_beta2_i_dn4, locals.var_beta2_i_dn5, locals.var_beta2_i_dn6, locals.var_beta2_i_dn7, locals.var_beta2_i_dn8, locals.var_beta2_i_dn9, locals.var_beta2_i_dn10, locals.var_beta2_i_dn11, locals.var_beta2_i_dn12, locals.var_beta2_i_dn13, locals.var_beta2_i_dn14, ) = (p.p521, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign4090_e5301: f64 = (1.0 + locals.var_t1);
        let assign4090_e5302: f64 = (locals.var_beta2_i * assign4090_e5301);
        (locals.var_beta2_i, locals.var_beta2_i_dn0, locals.var_beta2_i_dn2, locals.var_beta2_i_dn3, locals.var_beta2_i_dn4, locals.var_beta2_i_dn5, locals.var_beta2_i_dn6, locals.var_beta2_i_dn7, locals.var_beta2_i_dn8, locals.var_beta2_i_dn9, locals.var_beta2_i_dn10, locals.var_beta2_i_dn11, locals.var_beta2_i_dn12, locals.var_beta2_i_dn13, locals.var_beta2_i_dn14, ) = (assign4090_e5302, ((locals.var_beta2_i_dn0 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn0)), ((locals.var_beta2_i_dn2 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn2)), ((locals.var_beta2_i_dn3 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn3)), ((locals.var_beta2_i_dn4 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn4)), ((locals.var_beta2_i_dn5 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn5)), ((locals.var_beta2_i_dn6 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn6)), ((locals.var_beta2_i_dn7 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn7)), ((locals.var_beta2_i_dn8 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn8)), ((locals.var_beta2_i_dn9 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn9)), ((locals.var_beta2_i_dn10 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn10)), ((locals.var_beta2_i_dn11 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn11)), ((locals.var_beta2_i_dn12 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn12)), ((locals.var_beta2_i_dn13 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn13)), ((locals.var_beta2_i_dn14 * assign4090_e5301) + (locals.var_beta2_i * locals.var_t1_dn14)), );

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
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign4160_e5387, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign4170_e5391: f64 = (locals.var_inv_wact).powf(p.p98);
        let assign4170_e5394: f64 = (locals.var_inv_wwide).powf(p.p98);
        let assign4170_e5395: f64 = (assign4170_e5391 - assign4170_e5394);
        let assign4170_e5397: f64 = (assign4170_e5395).max(0.0);
        let assign4170_e5398: f64 = (p.p97 * assign4170_e5397);
        let assign4170_e5402: f64 = (locals.var_inv_wact * locals.var_inv_lact);
        let assign4170_e5404: f64 = (assign4170_e5402).powf(p.p100);
        let assign4170_e5405: f64 = (p.p99 * assign4170_e5404);
        let assign4170_e5406: f64 = (assign4170_e5398 + assign4170_e5405);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4170_e5406, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign4180_e5410: f64 = (1.0 + locals.var_t0);
        let assign4180_e5412: f64 = (assign4180_e5410 + locals.var_t1);
        let assign4180_e5413: f64 = (locals.var_ndepcv_i * assign4180_e5412);
        (locals.var_ndepcv_i, locals.var_ndepcv_i_dn0, locals.var_ndepcv_i_dn2, locals.var_ndepcv_i_dn3, locals.var_ndepcv_i_dn4, locals.var_ndepcv_i_dn5, locals.var_ndepcv_i_dn6, locals.var_ndepcv_i_dn7, locals.var_ndepcv_i_dn8, locals.var_ndepcv_i_dn9, locals.var_ndepcv_i_dn10, locals.var_ndepcv_i_dn11, locals.var_ndepcv_i_dn12, locals.var_ndepcv_i_dn13, locals.var_ndepcv_i_dn14, ) = (assign4180_e5413, ((locals.var_ndepcv_i_dn0 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_ndepcv_i_dn2 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_ndepcv_i_dn3 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_ndepcv_i_dn4 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_ndepcv_i_dn5 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_ndepcv_i_dn6 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_ndepcv_i_dn7 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_ndepcv_i_dn8 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_ndepcv_i_dn9 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_ndepcv_i_dn10 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_ndepcv_i_dn11 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_ndepcv_i_dn12 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_ndepcv_i_dn13 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_ndepcv_i_dn14 * assign4180_e5412) + (locals.var_ndepcv_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );

        let assign4190_e5417: f64 = (locals.var_inv_lact).powf(p.p121);
        let assign4190_e5420: f64 = (locals.var_inv_llong).powf(p.p121);
        let assign4190_e5421: f64 = (assign4190_e5417 - assign4190_e5420);
        let assign4190_e5423: f64 = (assign4190_e5421).max(0.0);
        let assign4190_e5424: f64 = (p.p120 * assign4190_e5423);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign4190_e5424, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign4200_e5428: f64 = (locals.var_inv_wact).powf(p.p123);
        let assign4200_e5431: f64 = (locals.var_inv_wwide).powf(p.p123);
        let assign4200_e5432: f64 = (assign4200_e5428 - assign4200_e5431);
        let assign4200_e5434: f64 = (assign4200_e5432).max(0.0);
        let assign4200_e5435: f64 = (p.p122 * assign4200_e5434);
        let assign4200_e5439: f64 = (locals.var_inv_wl).powf(p.p125);
        let assign4200_e5440: f64 = (p.p124 * assign4200_e5439);
        let assign4200_e5441: f64 = (assign4200_e5435 + assign4200_e5440);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4200_e5441, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign4210_e5445: f64 = (1.0 + locals.var_t0);
        let assign4210_e5447: f64 = (assign4210_e5445 + locals.var_t1);
        let assign4210_e5448: f64 = (locals.var_vfb_i * assign4210_e5447);
        (locals.var_vfb_i, locals.var_vfb_i_dn0, locals.var_vfb_i_dn2, locals.var_vfb_i_dn3, locals.var_vfb_i_dn4, locals.var_vfb_i_dn5, locals.var_vfb_i_dn6, locals.var_vfb_i_dn7, locals.var_vfb_i_dn8, locals.var_vfb_i_dn9, locals.var_vfb_i_dn10, locals.var_vfb_i_dn11, locals.var_vfb_i_dn12, locals.var_vfb_i_dn13, locals.var_vfb_i_dn14, ) = (assign4210_e5448, ((locals.var_vfb_i_dn0 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_vfb_i_dn2 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_vfb_i_dn3 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_vfb_i_dn4 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_vfb_i_dn5 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_vfb_i_dn6 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_vfb_i_dn7 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_vfb_i_dn8 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_vfb_i_dn9 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_vfb_i_dn10 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_vfb_i_dn11 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_vfb_i_dn12 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_vfb_i_dn13 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_vfb_i_dn14 * assign4210_e5447) + (locals.var_vfb_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );

        let assign4220_e5452: f64 = (locals.var_inv_lact).powf(p.p131);
        let assign4220_e5455: f64 = (locals.var_inv_llong).powf(p.p131);
        let assign4220_e5456: f64 = (assign4220_e5452 - assign4220_e5455);
        let assign4220_e5458: f64 = (assign4220_e5456).max(0.0);
        let assign4220_e5459: f64 = (p.p130 * assign4220_e5458);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign4220_e5459, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign4230_e5463: f64 = (locals.var_inv_wact).powf(p.p133);
        let assign4230_e5466: f64 = (locals.var_inv_wwide).powf(p.p133);
        let assign4230_e5467: f64 = (assign4230_e5463 - assign4230_e5466);
        let assign4230_e5469: f64 = (assign4230_e5467).max(0.0);
        let assign4230_e5470: f64 = (p.p132 * assign4230_e5469);
        let assign4230_e5474: f64 = (locals.var_inv_wl).powf(p.p135);
        let assign4230_e5475: f64 = (p.p134 * assign4230_e5474);
        let assign4230_e5476: f64 = (assign4230_e5470 + assign4230_e5475);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4230_e5476, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign4240_e5480: f64 = (1.0 + locals.var_t0);
        let assign4240_e5482: f64 = (assign4240_e5480 + locals.var_t1);
        let assign4240_e5483: f64 = (locals.var_vfbcv_i * assign4240_e5482);
        (locals.var_vfbcv_i, locals.var_vfbcv_i_dn0, locals.var_vfbcv_i_dn2, locals.var_vfbcv_i_dn3, locals.var_vfbcv_i_dn4, locals.var_vfbcv_i_dn5, locals.var_vfbcv_i_dn6, locals.var_vfbcv_i_dn7, locals.var_vfbcv_i_dn8, locals.var_vfbcv_i_dn9, locals.var_vfbcv_i_dn10, locals.var_vfbcv_i_dn11, locals.var_vfbcv_i_dn12, locals.var_vfbcv_i_dn13, locals.var_vfbcv_i_dn14, ) = (assign4240_e5483, ((locals.var_vfbcv_i_dn0 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_vfbcv_i_dn2 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_vfbcv_i_dn3 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_vfbcv_i_dn4 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_vfbcv_i_dn5 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_vfbcv_i_dn6 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_vfbcv_i_dn7 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_vfbcv_i_dn8 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_vfbcv_i_dn9 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_vfbcv_i_dn10 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_vfbcv_i_dn11 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_vfbcv_i_dn12 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_vfbcv_i_dn13 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_vfbcv_i_dn14 * assign4240_e5482) + (locals.var_vfbcv_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );

        let assign4250_e5487: f64 = (locals.var_inv_lact).powf(p.p264);
        let assign4250_e5490: f64 = (locals.var_inv_llong).powf(p.p264);
        let assign4250_e5491: f64 = (assign4250_e5487 - assign4250_e5490);
        let assign4250_e5493: f64 = (assign4250_e5491).max(0.0);
        let assign4250_e5494: f64 = (p.p263 * assign4250_e5493);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign4250_e5494, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign4260_e5498: f64 = (locals.var_inv_w).powf(p.p266);
        let assign4260_e5501: f64 = (locals.var_inv_wwide).powf(p.p266);
        let assign4260_e5502: f64 = (assign4260_e5498 - assign4260_e5501);
        let assign4260_e5504: f64 = (assign4260_e5502).max(0.0);
        let assign4260_e5505: f64 = (p.p265 * assign4260_e5504);
        let assign4260_e5509: f64 = (locals.var_inv_wl).powf(p.p268);
        let assign4260_e5510: f64 = (p.p267 * assign4260_e5509);
        let assign4260_e5511: f64 = (assign4260_e5505 + assign4260_e5510);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4260_e5511, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign4270_e5515: f64 = (1.0 + locals.var_t0);
        let assign4270_e5517: f64 = (assign4270_e5515 + locals.var_t1);
        let assign4270_e5518: f64 = (locals.var_vsatcv_i * assign4270_e5517);
        (locals.var_vsatcv_i, locals.var_vsatcv_i_dn0, locals.var_vsatcv_i_dn2, locals.var_vsatcv_i_dn3, locals.var_vsatcv_i_dn4, locals.var_vsatcv_i_dn5, locals.var_vsatcv_i_dn6, locals.var_vsatcv_i_dn7, locals.var_vsatcv_i_dn8, locals.var_vsatcv_i_dn9, locals.var_vsatcv_i_dn10, locals.var_vsatcv_i_dn11, locals.var_vsatcv_i_dn12, locals.var_vsatcv_i_dn13, locals.var_vsatcv_i_dn14, ) = (assign4270_e5518, ((locals.var_vsatcv_i_dn0 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_vsatcv_i_dn2 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_vsatcv_i_dn3 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_vsatcv_i_dn4 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_vsatcv_i_dn5 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_vsatcv_i_dn6 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_vsatcv_i_dn7 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_vsatcv_i_dn8 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_vsatcv_i_dn9 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_vsatcv_i_dn10 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_vsatcv_i_dn11 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_vsatcv_i_dn12 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_vsatcv_i_dn13 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_vsatcv_i_dn14 * assign4270_e5517) + (locals.var_vsatcv_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );

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
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign4300_e5547, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign4310_e5551: f64 = (locals.var_inv_w).powf(p.p189);
        let assign4310_e5554: f64 = (locals.var_inv_wwide).powf(p.p189);
        let assign4310_e5555: f64 = (assign4310_e5551 - assign4310_e5554);
        let assign4310_e5557: f64 = (assign4310_e5555).max(0.0);
        let assign4310_e5558: f64 = (p.p188 * assign4310_e5557);
        let assign4310_e5562: f64 = (locals.var_inv_wl).powf(p.p191);
        let assign4310_e5563: f64 = (p.p190 * assign4310_e5562);
        let assign4310_e5564: f64 = (assign4310_e5558 + assign4310_e5563);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4310_e5564, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign4320_e5568: f64 = (1.0 + locals.var_t0);
        let assign4320_e5570: f64 = (assign4320_e5568 + locals.var_t1);
        let assign4320_e5571: f64 = (locals.var_k1_i * assign4320_e5570);
        (locals.var_k1_i, locals.var_k1_i_dn0, locals.var_k1_i_dn2, locals.var_k1_i_dn3, locals.var_k1_i_dn4, locals.var_k1_i_dn5, locals.var_k1_i_dn6, locals.var_k1_i_dn7, locals.var_k1_i_dn8, locals.var_k1_i_dn9, locals.var_k1_i_dn10, locals.var_k1_i_dn11, locals.var_k1_i_dn12, locals.var_k1_i_dn13, locals.var_k1_i_dn14, ) = (assign4320_e5571, ((locals.var_k1_i_dn0 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_k1_i_dn2 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_k1_i_dn3 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_k1_i_dn4 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_k1_i_dn5 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_k1_i_dn6 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_k1_i_dn7 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_k1_i_dn8 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_k1_i_dn9 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_k1_i_dn10 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_k1_i_dn11 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_k1_i_dn12 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_k1_i_dn13 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_k1_i_dn14 * assign4320_e5570) + (locals.var_k1_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );

        let assign4330_e5575: f64 = (locals.var_inv_l).powf(p.p197);
        let assign4330_e5578: f64 = (locals.var_inv_llong).powf(p.p197);
        let assign4330_e5579: f64 = (assign4330_e5575 - assign4330_e5578);
        let assign4330_e5581: f64 = (assign4330_e5579).max(0.0);
        let assign4330_e5582: f64 = (p.p196 * assign4330_e5581);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign4330_e5582, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign4340_e5586: f64 = (locals.var_inv_w).powf(p.p199);
        let assign4340_e5589: f64 = (locals.var_inv_wwide).powf(p.p199);
        let assign4340_e5590: f64 = (assign4340_e5586 - assign4340_e5589);
        let assign4340_e5592: f64 = (assign4340_e5590).max(0.0);
        let assign4340_e5593: f64 = (p.p198 * assign4340_e5592);
        let assign4340_e5597: f64 = (locals.var_inv_wl).powf(p.p201);
        let assign4340_e5598: f64 = (p.p200 * assign4340_e5597);
        let assign4340_e5599: f64 = (assign4340_e5593 + assign4340_e5598);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign4340_e5599, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign4350_e5603: f64 = (1.0 + locals.var_t0);
        let assign4350_e5605: f64 = (assign4350_e5603 + locals.var_t1);
        let assign4350_e5606: f64 = (locals.var_k2_i * assign4350_e5605);
        (locals.var_k2_i, locals.var_k2_i_dn0, locals.var_k2_i_dn2, locals.var_k2_i_dn3, locals.var_k2_i_dn4, locals.var_k2_i_dn5, locals.var_k2_i_dn6, locals.var_k2_i_dn7, locals.var_k2_i_dn8, locals.var_k2_i_dn9, locals.var_k2_i_dn10, locals.var_k2_i_dn11, locals.var_k2_i_dn12, locals.var_k2_i_dn13, locals.var_k2_i_dn14, ) = (assign4350_e5606, ((locals.var_k2_i_dn0 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn0 + locals.var_t1_dn0))), ((locals.var_k2_i_dn2 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn2 + locals.var_t1_dn2))), ((locals.var_k2_i_dn3 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn3 + locals.var_t1_dn3))), ((locals.var_k2_i_dn4 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn4 + locals.var_t1_dn4))), ((locals.var_k2_i_dn5 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn5 + locals.var_t1_dn5))), ((locals.var_k2_i_dn6 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn6 + locals.var_t1_dn6))), ((locals.var_k2_i_dn7 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn7 + locals.var_t1_dn7))), ((locals.var_k2_i_dn8 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn8 + locals.var_t1_dn8))), ((locals.var_k2_i_dn9 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn9 + locals.var_t1_dn9))), ((locals.var_k2_i_dn10 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn10 + locals.var_t1_dn10))), ((locals.var_k2_i_dn11 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn11 + locals.var_t1_dn11))), ((locals.var_k2_i_dn12 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn12 + locals.var_t1_dn12))), ((locals.var_k2_i_dn13 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn13 + locals.var_t1_dn13))), ((locals.var_k2_i_dn14 * assign4350_e5605) + (locals.var_k2_i * (locals.var_t0_dn14 + locals.var_t1_dn14))), );

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

        if (locals.var_guard40 != 0.0) {
            let assign4450_e5677: f64 = (locals.var_inv_l).powf(p.p398);
            let assign4450_e5680: f64 = (locals.var_inv_llong).powf(p.p398);
            let assign4450_e5681: f64 = (assign4450_e5677 - assign4450_e5680);
            let assign4450_e5683: f64 = (assign4450_e5681).max(0.0);
            let assign4450_e5684: f64 = (p.p397 * assign4450_e5683);
            let assign4450_e5685: f64 = (1.0 + assign4450_e5684);
            let assign4450_e5686: f64 = (locals.var_rsw_i * assign4450_e5685);
            locals.var_rsw_i = assign4450_e5686;
        }

        if (locals.var_guard40 != 0.0) {
            let assign4460_e5695: f64 = (locals.var_inv_l).powf(p.p408);
            let assign4460_e5698: f64 = (locals.var_inv_llong).powf(p.p408);
            let assign4460_e5699: f64 = (assign4460_e5695 - assign4460_e5698);
            let assign4460_e5701: f64 = (assign4460_e5699).max(0.0);
            let assign4460_e5702: f64 = (p.p407 * assign4460_e5701);
            let assign4460_e5703: f64 = (1.0 + assign4460_e5702);
            let assign4460_e5704: f64 = (locals.var_rdw_i * assign4460_e5703);
            locals.var_rdw_i = assign4460_e5704;
        }

        if (locals.var_guard40 == 0.0) {
            let assign4470_e5714: f64 = (locals.var_inv_l).powf(p.p415);
            let assign4470_e5717: f64 = (locals.var_inv_llong).powf(p.p415);
            let assign4470_e5718: f64 = (assign4470_e5714 - assign4470_e5717);
            let assign4470_e5720: f64 = (assign4470_e5718).max(0.0);
            let assign4470_e5721: f64 = (p.p414 * assign4470_e5720);
            let assign4470_e5722: f64 = (1.0 + assign4470_e5721);
            let assign4470_e5723: f64 = (locals.var_rdsw_i * assign4470_e5722);
            locals.var_rdsw_i = assign4470_e5723;
        }

        let assign4480_e5728: f64 = if locals.var_ucs_i < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign4480_e5728;

        if (locals.var_guard41 != 0.0) {
            locals.var_ucs_i = 1.0;
        }

        let assign4500_e5735: f64 = if locals.var_ucs_i > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign4500_e5735;

        if ((locals.var_guard41 == 0.0) && (locals.var_guard42 != 0.0)) {
            locals.var_ucs_i = 2.0;
        }

        let assign4520_e5745: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign4520_e5745;

        let assign4530_e5748: f64 = if locals.var_ucsr_i < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign4530_e5748;

        if ((locals.var_guard43 != 0.0) && (locals.var_guard44 != 0.0)) {
            locals.var_ucsr_i = 1.0;
        }

        let assign4550_e5757: f64 = if locals.var_ucsr_i > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign4550_e5757;

        if (((locals.var_guard43 != 0.0) && (locals.var_guard44 == 0.0)) && (locals.var_guard45 != 0.0)) {
            locals.var_ucsr_i = 2.0;
        }

        let assign4760_e5826: f64 = if locals.var_dlcig_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard65 = assign4760_e5826;

        if (locals.var_guard65 != 0.0) {
            locals.var_dlcig_i = 0.0;
        }

        let assign4780_e5833: f64 = if locals.var_dlcigd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard66 = assign4780_e5833;

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard66 != 0.0) {
            locals.var_dlcigd_i = 0.0;
        }

        let assign4800_e5840: f64 = if locals.var_m0_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard67 = assign4800_e5840;

        if (locals.var_guard67 != 0.0) {
            locals.var_m0_i = 0.0;
        }

        let assign4820_e5847: f64 = if locals.var_u0_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard68 = assign4820_e5847;

        if (locals.var_guard68 != 0.0) {
            locals.var_u0_i = 0.067;
        }

        let assign4840_e5854: f64 = if locals.var_ua_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard69 = assign4840_e5854;

        if (locals.var_guard69 != 0.0) {
            (locals.var_ua_i, locals.var_ua_i_dn0, locals.var_ua_i_dn2, locals.var_ua_i_dn3, locals.var_ua_i_dn4, locals.var_ua_i_dn5, locals.var_ua_i_dn6, locals.var_ua_i_dn7, locals.var_ua_i_dn8, locals.var_ua_i_dn9, locals.var_ua_i_dn10, locals.var_ua_i_dn11, locals.var_ua_i_dn12, locals.var_ua_i_dn13, locals.var_ua_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign4860_e5861: f64 = if locals.var_eu_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign4860_e5861;

        if (locals.var_guard70 != 0.0) {
            (locals.var_eu_i, locals.var_eu_i_dn0, locals.var_eu_i_dn2, locals.var_eu_i_dn3, locals.var_eu_i_dn4, locals.var_eu_i_dn5, locals.var_eu_i_dn6, locals.var_eu_i_dn7, locals.var_eu_i_dn8, locals.var_eu_i_dn9, locals.var_eu_i_dn10, locals.var_eu_i_dn11, locals.var_eu_i_dn12, locals.var_eu_i_dn13, locals.var_eu_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign4880_e5868: f64 = if locals.var_ud_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard71 = assign4880_e5868;

        if (locals.var_guard71 != 0.0) {
            (locals.var_ud_i, locals.var_ud_i_dn0, locals.var_ud_i_dn2, locals.var_ud_i_dn3, locals.var_ud_i_dn4, locals.var_ud_i_dn5, locals.var_ud_i_dn6, locals.var_ud_i_dn7, locals.var_ud_i_dn8, locals.var_ud_i_dn9, locals.var_ud_i_dn10, locals.var_ud_i_dn11, locals.var_ud_i_dn12, locals.var_ud_i_dn13, locals.var_ud_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign4900_e5875: f64 = if locals.var_ucs_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard72 = assign4900_e5875;

        if (locals.var_guard72 != 0.0) {
            locals.var_ucs_i = 0.0;
        }

        let assign4920_e5882: f64 = if locals.var_beta1_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard73 = assign4920_e5882;

        if (locals.var_guard73 != 0.0) {
            (locals.var_beta1_i, locals.var_beta1_i_dn0, locals.var_beta1_i_dn2, locals.var_beta1_i_dn3, locals.var_beta1_i_dn4, locals.var_beta1_i_dn5, locals.var_beta1_i_dn6, locals.var_beta1_i_dn7, locals.var_beta1_i_dn8, locals.var_beta1_i_dn9, locals.var_beta1_i_dn10, locals.var_beta1_i_dn11, locals.var_beta1_i_dn12, locals.var_beta1_i_dn13, locals.var_beta1_i_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign4940_e5889: f64 = if p.p1065 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard74 = assign4940_e5889;

        if (locals.var_guard74 != 0.0) {
            locals.var_lh1 = p.p1066;
        }

        let assign4960_e5896: f64 = if locals.var_leff > locals.var_lh1 { 1.0 } else { 0.0 };
        locals.var_guard75 = assign4960_e5896;

        if ((locals.var_guard74 != 0.0) && (locals.var_guard75 != 0.0)) {
            let assign4970_e5902: f64 = (locals.var_leff - locals.var_lh1);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign4970_e5902, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard74 != 0.0) && (locals.var_guard75 == 0.0)) {
            locals.var_lh1 = locals.var_leff;
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (locals.var_lh1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign5000_e5922: f64 = (locals.var_t0 / 2.0);
        let assign5000_e5923: f64 = if p.p801 >= assign5000_e5922 { 1.0 } else { 0.0 };
        locals.var_guard76 = assign5000_e5923;

        if ((locals.var_guard74 != 0.0) && (locals.var_guard76 != 0.0)) {
            locals.var_lintnoi_i = 0.0;
        }

        if ((locals.var_guard74 != 0.0) && (locals.var_guard76 == 0.0)) {
            locals.var_lintnoi_i = p.p801;
        }

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

        if (locals.var_guard77 != 0.0) {
            let assign5130_e5955: f64 = (p.p374 * p.p3);
            locals.var_rsourcegeo = assign5130_e5955;
        }

        let assign5140_e5964: f64 = if ((p.p10 > 0.0) && (p.p374 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard78 = assign5140_e5964;

        let assign5150_e5967: f64 = if p.p9 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard79 = assign5150_e5967;

        let assign5160_e5970: f64 = (p.p2 % 2.0);
        let assign5160_e5972: f64 = if assign5160_e5970 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard80 = assign5160_e5972;

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) {
            locals.var_nuendd = 1.0;
            locals.var_nuends = 1.0;
        }

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) {
            let assign5190_e6006: f64 = (p.p2 - 1.0);
            let assign5190_e6008: f64 = (assign5190_e6006 / 2.0);
            let assign5190_e6010: f64 = (assign5190_e6008).max(0.0);
            let assign5190_e6011: f64 = (2.0 * assign5190_e6010);
            locals.var_nuintd = assign5190_e6011;
        }

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 != 0.0)) {
            locals.var_nuints = locals.var_nuintd;
        }

        let assign5210_e6027: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard81 = assign5210_e6027;

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 != 0.0)) {
            locals.var_nuendd = 2.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 != 0.0)) {
            let assign5230_e6056: f64 = (p.p2 / 2.0);
            let assign5230_e6058: f64 = (assign5230_e6056 - 1.0);
            let assign5230_e6060: f64 = (assign5230_e6058).max(0.0);
            let assign5230_e6061: f64 = (2.0 * assign5230_e6060);
            locals.var_nuintd = assign5230_e6061;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 != 0.0)) {
            locals.var_nuends = 0.0;
            locals.var_nuints = p.p2;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 == 0.0)) {
            locals.var_nuendd = 0.0;
            locals.var_nuintd = p.p2;
            locals.var_nuends = 2.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard80 == 0.0)) && (locals.var_guard81 == 0.0)) {
            let assign5290_e6152: f64 = (p.p2 / 2.0);
            let assign5290_e6154: f64 = (assign5290_e6152 - 1.0);
            let assign5290_e6156: f64 = (assign5290_e6154).max(0.0);
            let assign5290_e6157: f64 = (2.0 * assign5290_e6156);
            locals.var_nuints = assign5290_e6157;
        }

        let assign5300_e6162: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard82 = assign5300_e6162;

        let assign5310_e6165: f64 = if locals.var_nuints == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign5310_e6165;

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 != 0.0)) && (locals.var_guard83 != 0.0)) {
            locals.var_rint = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 != 0.0)) && (locals.var_guard83 == 0.0)) {
            let assign5330_e6192: f64 = (p.p374 * locals.var_dmcgeff);
            let assign5330_e6195: f64 = (locals.var_weff * locals.var_nuints);
            let assign5330_e6196: f64 = (assign5330_e6192 / assign5330_e6195);
            locals.var_rint = assign5330_e6196;
        }

        let assign5340_e6201: f64 = if locals.var_nuintd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign5340_e6201;

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 == 0.0)) && (locals.var_guard84 != 0.0)) {
            locals.var_rint = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard79 != 0.0)) && (locals.var_guard82 == 0.0)) && (locals.var_guard84 == 0.0)) {
            let assign5360_e6230: f64 = (p.p374 * locals.var_dmcgeff);
            let assign5360_e6233: f64 = (locals.var_weff * locals.var_nuintd);
            let assign5360_e6234: f64 = (assign5360_e6230 / assign5360_e6233);
            locals.var_rint = assign5360_e6234;
        }

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

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 == 0.0)) {
            let assign5540_e6335: f64 = (p.p374 * locals.var_dmcgeff);
            let assign5540_e6338: f64 = (locals.var_weff * locals.var_nuends);
            let assign5540_e6339: f64 = (assign5540_e6335 / assign5540_e6338);
            locals.var_rend = assign5540_e6339;
        }

        let assign5560_e6352: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5560_e6355: f64 = if ((locals.var_nuends == 0.0) || (assign5560_e6352 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard102 = assign5560_e6355;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && ((locals.var_guard99 != 0.0) && (locals.var_guard98 == 0.0))) && (locals.var_guard102 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && ((locals.var_guard99 != 0.0) && (locals.var_guard98 == 0.0))) && (locals.var_guard102 == 0.0)) {
            let assign5580_e6396: f64 = (p.p374 * locals.var_weff);
            let assign5580_e6399: f64 = (3.0 * locals.var_nuends);
            let assign5580_e6402: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign5580_e6403: f64 = (assign5580_e6399 * assign5580_e6402);
            let assign5580_e6404: f64 = (assign5580_e6396 / assign5580_e6403);
            locals.var_rend = assign5580_e6404;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 != 0.0)) && (!((locals.var_guard98 != 0.0) || (locals.var_guard99 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign5600_e6435: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard103 = assign5600_e6435;

        let assign5610_e6446: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard104 = assign5610_e6446;

        let assign5620_e6449: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign5620_e6449;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard105 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard105 == 0.0)) {
            let assign5640_e6486: f64 = (p.p374 * locals.var_dmcgeff);
            let assign5640_e6489: f64 = (locals.var_weff * locals.var_nuends);
            let assign5640_e6490: f64 = (assign5640_e6486 / assign5640_e6489);
            locals.var_rend = assign5640_e6490;
        }

        let assign5660_e6503: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5660_e6506: f64 = if ((locals.var_nuends == 0.0) || (assign5660_e6503 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard107 = assign5660_e6506;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && ((locals.var_guard104 != 0.0) && (locals.var_guard103 == 0.0))) && (locals.var_guard107 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && ((locals.var_guard104 != 0.0) && (locals.var_guard103 == 0.0))) && (locals.var_guard107 == 0.0)) {
            let assign5680_e6549: f64 = (p.p374 * locals.var_weff);
            let assign5680_e6552: f64 = (3.0 * locals.var_nuends);
            let assign5680_e6555: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign5680_e6556: f64 = (assign5680_e6552 * assign5680_e6555);
            let assign5680_e6557: f64 = (assign5680_e6549 / assign5680_e6556);
            locals.var_rend = assign5680_e6557;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 != 0.0)) && (locals.var_guard97 == 0.0)) && (!((locals.var_guard103 != 0.0) || (locals.var_guard104 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign5700_e6581: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign5700_e6581;

        let assign5710_e6592: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard109 = assign5710_e6592;

        let assign5720_e6603: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard110 = assign5720_e6603;

        let assign5730_e6606: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign5730_e6606;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 == 0.0)) {
            let assign5750_e6643: f64 = (p.p374 * locals.var_dmcgeff);
            let assign5750_e6646: f64 = (locals.var_weff * locals.var_nuendd);
            let assign5750_e6647: f64 = (assign5750_e6643 / assign5750_e6646);
            locals.var_rend = assign5750_e6647;
        }

        let assign5770_e6660: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5770_e6663: f64 = if ((locals.var_nuendd == 0.0) || (assign5770_e6660 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard113 = assign5770_e6663;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && ((locals.var_guard110 != 0.0) && (locals.var_guard109 == 0.0))) && (locals.var_guard113 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && ((locals.var_guard110 != 0.0) && (locals.var_guard109 == 0.0))) && (locals.var_guard113 == 0.0)) {
            let assign5790_e6706: f64 = (p.p374 * locals.var_weff);
            let assign5790_e6709: f64 = (3.0 * locals.var_nuendd);
            let assign5790_e6712: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign5790_e6713: f64 = (assign5790_e6709 * assign5790_e6712);
            let assign5790_e6714: f64 = (assign5790_e6706 / assign5790_e6713);
            locals.var_rend = assign5790_e6714;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 != 0.0)) && (!((locals.var_guard109 != 0.0) || (locals.var_guard110 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign5810_e6746: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard114 = assign5810_e6746;

        let assign5820_e6757: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard115 = assign5820_e6757;

        let assign5830_e6760: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign5830_e6760;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && (locals.var_guard114 != 0.0)) && (locals.var_guard116 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && (locals.var_guard114 != 0.0)) && (locals.var_guard116 == 0.0)) {
            let assign5850_e6799: f64 = (p.p374 * locals.var_dmcgeff);
            let assign5850_e6802: f64 = (locals.var_weff * locals.var_nuendd);
            let assign5850_e6803: f64 = (assign5850_e6799 / assign5850_e6802);
            locals.var_rend = assign5850_e6803;
        }

        let assign5870_e6816: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5870_e6819: f64 = if ((locals.var_nuendd == 0.0) || (assign5870_e6816 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard118 = assign5870_e6819;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && ((locals.var_guard115 != 0.0) && (locals.var_guard114 == 0.0))) && (locals.var_guard118 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && ((locals.var_guard115 != 0.0) && (locals.var_guard114 == 0.0))) && (locals.var_guard118 == 0.0)) {
            let assign5890_e6864: f64 = (p.p374 * locals.var_weff);
            let assign5890_e6867: f64 = (3.0 * locals.var_nuendd);
            let assign5890_e6870: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign5890_e6871: f64 = (assign5890_e6867 * assign5890_e6870);
            let assign5890_e6872: f64 = (assign5890_e6864 / assign5890_e6871);
            locals.var_rend = assign5890_e6872;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard85 != 0.0)) && (locals.var_guard96 == 0.0)) && (locals.var_guard108 == 0.0)) && (!((locals.var_guard114 != 0.0) || (locals.var_guard115 != 0.0)))) {
            locals.var_rend = 0.0;
        }

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

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && (locals.var_guard123 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && (locals.var_guard121 != 0.0)) && (locals.var_guard123 == 0.0)) {
            let assign5970_e6966: f64 = (p.p374 * locals.var_dmcgeff);
            let assign5970_e6969: f64 = (locals.var_weff * locals.var_nuends);
            let assign5970_e6970: f64 = (assign5970_e6966 / assign5970_e6969);
            locals.var_rend = assign5970_e6970;
        }

        let assign5990_e6983: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign5990_e6986: f64 = if ((locals.var_nuends == 0.0) || (assign5990_e6983 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard125 = assign5990_e6986;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && ((locals.var_guard122 != 0.0) && (locals.var_guard121 == 0.0))) && (locals.var_guard125 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && ((locals.var_guard122 != 0.0) && (locals.var_guard121 == 0.0))) && (locals.var_guard125 == 0.0)) {
            let assign6010_e7033: f64 = (p.p374 * locals.var_weff);
            let assign6010_e7036: f64 = (3.0 * locals.var_nuends);
            let assign6010_e7039: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign6010_e7040: f64 = (assign6010_e7036 * assign6010_e7039);
            let assign6010_e7041: f64 = (assign6010_e7033 / assign6010_e7040);
            locals.var_rend = assign6010_e7041;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 != 0.0)) && (!((locals.var_guard121 != 0.0) || (locals.var_guard122 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign6030_e7075: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard126 = assign6030_e7075;

        let assign6040_e7086: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard127 = assign6040_e7086;

        let assign6050_e7089: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard128 = assign6050_e7089;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && (locals.var_guard126 != 0.0)) && (locals.var_guard128 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && (locals.var_guard126 != 0.0)) && (locals.var_guard128 == 0.0)) {
            let assign6070_e7132: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6070_e7135: f64 = (locals.var_weff * locals.var_nuends);
            let assign6070_e7136: f64 = (assign6070_e7132 / assign6070_e7135);
            locals.var_rend = assign6070_e7136;
        }

        let assign6090_e7149: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6090_e7152: f64 = if ((locals.var_nuends == 0.0) || (assign6090_e7149 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard130 = assign6090_e7152;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && ((locals.var_guard127 != 0.0) && (locals.var_guard126 == 0.0))) && (locals.var_guard130 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && ((locals.var_guard127 != 0.0) && (locals.var_guard126 == 0.0))) && (locals.var_guard130 == 0.0)) {
            let assign6110_e7201: f64 = (p.p374 * locals.var_weff);
            let assign6110_e7204: f64 = (3.0 * locals.var_nuends);
            let assign6110_e7207: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign6110_e7208: f64 = (assign6110_e7204 * assign6110_e7207);
            let assign6110_e7209: f64 = (assign6110_e7201 / assign6110_e7208);
            locals.var_rend = assign6110_e7209;
        }

    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 != 0.0)) && (locals.var_guard120 == 0.0)) && (!((locals.var_guard126 != 0.0) || (locals.var_guard127 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign6130_e7236: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard131 = assign6130_e7236;

        let assign6140_e7247: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard132 = assign6140_e7247;

        let assign6150_e7258: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign6150_e7258;

        let assign6160_e7261: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard134 = assign6160_e7261;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && (locals.var_guard132 != 0.0)) && (locals.var_guard134 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && (locals.var_guard132 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6180_e7304: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6180_e7307: f64 = (locals.var_weff * locals.var_nuendd);
            let assign6180_e7308: f64 = (assign6180_e7304 / assign6180_e7307);
            locals.var_rend = assign6180_e7308;
        }

        let assign6200_e7320: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard136 = assign6200_e7320;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && ((locals.var_guard133 != 0.0) && (locals.var_guard132 == 0.0))) && (locals.var_guard136 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && ((locals.var_guard133 != 0.0) && (locals.var_guard132 == 0.0))) && (locals.var_guard136 == 0.0)) {
            let assign6220_e7369: f64 = (p.p374 * locals.var_weff);
            let assign6220_e7372: f64 = (6.0 * locals.var_nuendd);
            let assign6220_e7374: f64 = (assign6220_e7372 * locals.var_dmcgeff);
            let assign6220_e7375: f64 = (assign6220_e7369 / assign6220_e7374);
            locals.var_rend = assign6220_e7375;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 != 0.0)) && (!((locals.var_guard132 != 0.0) || (locals.var_guard133 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign6240_e7410: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign6240_e7410;

        let assign6250_e7421: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard138 = assign6250_e7421;

        let assign6260_e7424: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard139 = assign6260_e7424;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && (locals.var_guard137 != 0.0)) && (locals.var_guard139 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && (locals.var_guard137 != 0.0)) && (locals.var_guard139 == 0.0)) {
            let assign6280_e7469: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6280_e7472: f64 = (locals.var_weff * locals.var_nuendd);
            let assign6280_e7473: f64 = (assign6280_e7469 / assign6280_e7472);
            locals.var_rend = assign6280_e7473;
        }

        let assign6300_e7485: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard141 = assign6300_e7485;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && ((locals.var_guard138 != 0.0) && (locals.var_guard137 == 0.0))) && (locals.var_guard141 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && ((locals.var_guard138 != 0.0) && (locals.var_guard137 == 0.0))) && (locals.var_guard141 == 0.0)) {
            let assign6320_e7536: f64 = (p.p374 * locals.var_weff);
            let assign6320_e7539: f64 = (6.0 * locals.var_nuendd);
            let assign6320_e7541: f64 = (assign6320_e7539 * locals.var_dmcgeff);
            let assign6320_e7542: f64 = (assign6320_e7536 / assign6320_e7541);
            locals.var_rend = assign6320_e7542;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard86 != 0.0) && (locals.var_guard85 == 0.0))) && (locals.var_guard119 == 0.0)) && (locals.var_guard131 == 0.0)) && (!((locals.var_guard137 != 0.0) || (locals.var_guard138 != 0.0)))) {
            locals.var_rend = 0.0;
        }

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

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && (locals.var_guard146 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && (locals.var_guard144 != 0.0)) && (locals.var_guard146 == 0.0)) {
            let assign6400_e7643: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6400_e7646: f64 = (locals.var_weff * locals.var_nuends);
            let assign6400_e7647: f64 = (assign6400_e7643 / assign6400_e7646);
            locals.var_rend = assign6400_e7647;
        }

        let assign6420_e7659: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard148 = assign6420_e7659;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && ((locals.var_guard145 != 0.0) && (locals.var_guard144 == 0.0))) && (locals.var_guard148 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && ((locals.var_guard145 != 0.0) && (locals.var_guard144 == 0.0))) && (locals.var_guard148 == 0.0)) {
            let assign6440_e7710: f64 = (p.p374 * locals.var_weff);
            let assign6440_e7713: f64 = (6.0 * locals.var_nuends);
            let assign6440_e7715: f64 = (assign6440_e7713 * locals.var_dmcgeff);
            let assign6440_e7716: f64 = (assign6440_e7710 / assign6440_e7715);
            locals.var_rend = assign6440_e7716;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 != 0.0)) && (!((locals.var_guard144 != 0.0) || (locals.var_guard145 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign6460_e7752: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard149 = assign6460_e7752;

        let assign6470_e7763: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard150 = assign6470_e7763;

        let assign6480_e7766: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard151 = assign6480_e7766;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && (locals.var_guard149 != 0.0)) && (locals.var_guard151 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && (locals.var_guard149 != 0.0)) && (locals.var_guard151 == 0.0)) {
            let assign6500_e7813: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6500_e7816: f64 = (locals.var_weff * locals.var_nuends);
            let assign6500_e7817: f64 = (assign6500_e7813 / assign6500_e7816);
            locals.var_rend = assign6500_e7817;
        }

        let assign6520_e7829: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard153 = assign6520_e7829;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && ((locals.var_guard150 != 0.0) && (locals.var_guard149 == 0.0))) && (locals.var_guard153 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && ((locals.var_guard150 != 0.0) && (locals.var_guard149 == 0.0))) && (locals.var_guard153 == 0.0)) {
            let assign6540_e7882: f64 = (p.p374 * locals.var_weff);
            let assign6540_e7885: f64 = (6.0 * locals.var_nuends);
            let assign6540_e7887: f64 = (assign6540_e7885 * locals.var_dmcgeff);
            let assign6540_e7888: f64 = (assign6540_e7882 / assign6540_e7887);
            locals.var_rend = assign6540_e7888;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 != 0.0)) && (locals.var_guard143 == 0.0)) && (!((locals.var_guard149 != 0.0) || (locals.var_guard150 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign6560_e7917: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign6560_e7917;

        let assign6570_e7928: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard155 = assign6570_e7928;

        let assign6580_e7939: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard156 = assign6580_e7939;

        let assign6590_e7942: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign6590_e7942;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard155 != 0.0)) && (locals.var_guard157 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && (locals.var_guard155 != 0.0)) && (locals.var_guard157 == 0.0)) {
            let assign6610_e7989: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6610_e7992: f64 = (locals.var_weff * locals.var_nuendd);
            let assign6610_e7993: f64 = (assign6610_e7989 / assign6610_e7992);
            locals.var_rend = assign6610_e7993;
        }

        let assign6630_e8006: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6630_e8009: f64 = if ((locals.var_nuendd == 0.0) || (assign6630_e8006 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard159 = assign6630_e8009;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && ((locals.var_guard156 != 0.0) && (locals.var_guard155 == 0.0))) && (locals.var_guard159 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && ((locals.var_guard156 != 0.0) && (locals.var_guard155 == 0.0))) && (locals.var_guard159 == 0.0)) {
            let assign6650_e8062: f64 = (p.p374 * locals.var_weff);
            let assign6650_e8065: f64 = (3.0 * locals.var_nuendd);
            let assign6650_e8068: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign6650_e8069: f64 = (assign6650_e8065 * assign6650_e8068);
            let assign6650_e8070: f64 = (assign6650_e8062 / assign6650_e8069);
            locals.var_rend = assign6650_e8070;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 != 0.0)) && (!((locals.var_guard155 != 0.0) || (locals.var_guard156 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign6670_e8107: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard160 = assign6670_e8107;

        let assign6680_e8118: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard161 = assign6680_e8118;

        let assign6690_e8121: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign6690_e8121;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard160 != 0.0)) && (locals.var_guard162 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && (locals.var_guard160 != 0.0)) && (locals.var_guard162 == 0.0)) {
            let assign6710_e8170: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6710_e8173: f64 = (locals.var_weff * locals.var_nuendd);
            let assign6710_e8174: f64 = (assign6710_e8170 / assign6710_e8173);
            locals.var_rend = assign6710_e8174;
        }

        let assign6730_e8187: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign6730_e8190: f64 = if ((locals.var_nuendd == 0.0) || (assign6730_e8187 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard164 = assign6730_e8190;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && ((locals.var_guard161 != 0.0) && (locals.var_guard160 == 0.0))) && (locals.var_guard164 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && ((locals.var_guard161 != 0.0) && (locals.var_guard160 == 0.0))) && (locals.var_guard164 == 0.0)) {
            let assign6750_e8245: f64 = (p.p374 * locals.var_weff);
            let assign6750_e8248: f64 = (3.0 * locals.var_nuendd);
            let assign6750_e8251: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign6750_e8252: f64 = (assign6750_e8248 * assign6750_e8251);
            let assign6750_e8253: f64 = (assign6750_e8245 / assign6750_e8252);
            locals.var_rend = assign6750_e8253;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard87 != 0.0) && (!((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0))))) && (locals.var_guard142 == 0.0)) && (locals.var_guard154 == 0.0)) && (!((locals.var_guard160 != 0.0) || (locals.var_guard161 != 0.0)))) {
            locals.var_rend = 0.0;
        }

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

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard169 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && (locals.var_guard167 != 0.0)) && (locals.var_guard169 == 0.0)) {
            let assign6830_e8360: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6830_e8363: f64 = (locals.var_weff * locals.var_nuends);
            let assign6830_e8364: f64 = (assign6830_e8360 / assign6830_e8363);
            locals.var_rend = assign6830_e8364;
        }

        let assign6850_e8376: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard171 = assign6850_e8376;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && ((locals.var_guard168 != 0.0) && (locals.var_guard167 == 0.0))) && (locals.var_guard171 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && ((locals.var_guard168 != 0.0) && (locals.var_guard167 == 0.0))) && (locals.var_guard171 == 0.0)) {
            let assign6870_e8431: f64 = (p.p374 * locals.var_weff);
            let assign6870_e8434: f64 = (6.0 * locals.var_nuends);
            let assign6870_e8436: f64 = (assign6870_e8434 * locals.var_dmcgeff);
            let assign6870_e8437: f64 = (assign6870_e8431 / assign6870_e8436);
            locals.var_rend = assign6870_e8437;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 != 0.0)) && (!((locals.var_guard167 != 0.0) || (locals.var_guard168 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign6890_e8475: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard172 = assign6890_e8475;

        let assign6900_e8486: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard173 = assign6900_e8486;

        let assign6910_e8489: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard174 = assign6910_e8489;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard172 != 0.0)) && (locals.var_guard174 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && (locals.var_guard172 != 0.0)) && (locals.var_guard174 == 0.0)) {
            let assign6930_e8540: f64 = (p.p374 * locals.var_dmcgeff);
            let assign6930_e8543: f64 = (locals.var_weff * locals.var_nuends);
            let assign6930_e8544: f64 = (assign6930_e8540 / assign6930_e8543);
            locals.var_rend = assign6930_e8544;
        }

        let assign6950_e8556: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard176 = assign6950_e8556;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && ((locals.var_guard173 != 0.0) && (locals.var_guard172 == 0.0))) && (locals.var_guard176 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && ((locals.var_guard173 != 0.0) && (locals.var_guard172 == 0.0))) && (locals.var_guard176 == 0.0)) {
            let assign6970_e8613: f64 = (p.p374 * locals.var_weff);
            let assign6970_e8616: f64 = (6.0 * locals.var_nuends);
            let assign6970_e8618: f64 = (assign6970_e8616 * locals.var_dmcgeff);
            let assign6970_e8619: f64 = (assign6970_e8613 / assign6970_e8618);
            locals.var_rend = assign6970_e8619;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 != 0.0)) && (locals.var_guard166 == 0.0)) && (!((locals.var_guard172 != 0.0) || (locals.var_guard173 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign6990_e8650: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard177 = assign6990_e8650;

        let assign7000_e8661: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard178 = assign7000_e8661;

        let assign7010_e8672: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard179 = assign7010_e8672;

        let assign7020_e8675: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard180 = assign7020_e8675;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard178 != 0.0)) && (locals.var_guard180 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && (locals.var_guard178 != 0.0)) && (locals.var_guard180 == 0.0)) {
            let assign7040_e8726: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7040_e8729: f64 = (locals.var_weff * locals.var_nuendd);
            let assign7040_e8730: f64 = (assign7040_e8726 / assign7040_e8729);
            locals.var_rend = assign7040_e8730;
        }

        let assign7060_e8742: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard182 = assign7060_e8742;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && ((locals.var_guard179 != 0.0) && (locals.var_guard178 == 0.0))) && (locals.var_guard182 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && ((locals.var_guard179 != 0.0) && (locals.var_guard178 == 0.0))) && (locals.var_guard182 == 0.0)) {
            let assign7080_e8799: f64 = (p.p374 * locals.var_weff);
            let assign7080_e8802: f64 = (6.0 * locals.var_nuendd);
            let assign7080_e8804: f64 = (assign7080_e8802 * locals.var_dmcgeff);
            let assign7080_e8805: f64 = (assign7080_e8799 / assign7080_e8804);
            locals.var_rend = assign7080_e8805;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 != 0.0)) && (!((locals.var_guard178 != 0.0) || (locals.var_guard179 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign7100_e8844: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard183 = assign7100_e8844;

        let assign7110_e8855: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard184 = assign7110_e8855;

        let assign7120_e8858: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard185 = assign7120_e8858;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && (locals.var_guard183 != 0.0)) && (locals.var_guard185 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && (locals.var_guard183 != 0.0)) && (locals.var_guard185 == 0.0)) {
            let assign7140_e8911: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7140_e8914: f64 = (locals.var_weff * locals.var_nuendd);
            let assign7140_e8915: f64 = (assign7140_e8911 / assign7140_e8914);
            locals.var_rend = assign7140_e8915;
        }

        let assign7160_e8927: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard187 = assign7160_e8927;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && ((locals.var_guard184 != 0.0) && (locals.var_guard183 == 0.0))) && (locals.var_guard187 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && ((locals.var_guard184 != 0.0) && (locals.var_guard183 == 0.0))) && (locals.var_guard187 == 0.0)) {
            let assign7180_e8986: f64 = (p.p374 * locals.var_weff);
            let assign7180_e8989: f64 = (6.0 * locals.var_nuendd);
            let assign7180_e8991: f64 = (assign7180_e8989 * locals.var_dmcgeff);
            let assign7180_e8992: f64 = (assign7180_e8986 / assign7180_e8991);
            locals.var_rend = assign7180_e8992;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard88 != 0.0) && (!(((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0))))) && (locals.var_guard165 == 0.0)) && (locals.var_guard177 == 0.0)) && (!((locals.var_guard183 != 0.0) || (locals.var_guard184 != 0.0)))) {
            locals.var_rend = 0.0;
        }

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

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && (locals.var_guard192 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && (locals.var_guard190 != 0.0)) && (locals.var_guard192 == 0.0)) {
            let assign7260_e9105: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7260_e9108: f64 = (locals.var_weff * locals.var_nuends);
            let assign7260_e9109: f64 = (assign7260_e9105 / assign7260_e9108);
            locals.var_rend = assign7260_e9109;
        }

        let assign7280_e9122: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7280_e9125: f64 = if ((locals.var_nuends == 0.0) || (assign7280_e9122 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard194 = assign7280_e9125;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && ((locals.var_guard191 != 0.0) && (locals.var_guard190 == 0.0))) && (locals.var_guard194 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && ((locals.var_guard191 != 0.0) && (locals.var_guard190 == 0.0))) && (locals.var_guard194 == 0.0)) {
            let assign7300_e9184: f64 = (p.p374 * locals.var_weff);
            let assign7300_e9187: f64 = (3.0 * locals.var_nuends);
            let assign7300_e9190: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign7300_e9191: f64 = (assign7300_e9187 * assign7300_e9190);
            let assign7300_e9192: f64 = (assign7300_e9184 / assign7300_e9191);
            locals.var_rend = assign7300_e9192;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 != 0.0)) && (!((locals.var_guard190 != 0.0) || (locals.var_guard191 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign7320_e9232: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard195 = assign7320_e9232;

        let assign7330_e9243: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard196 = assign7330_e9243;

        let assign7340_e9246: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard197 = assign7340_e9246;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && (locals.var_guard195 != 0.0)) && (locals.var_guard197 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && (locals.var_guard195 != 0.0)) && (locals.var_guard197 == 0.0)) {
            let assign7360_e9301: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7360_e9304: f64 = (locals.var_weff * locals.var_nuends);
            let assign7360_e9305: f64 = (assign7360_e9301 / assign7360_e9304);
            locals.var_rend = assign7360_e9305;
        }

        let assign7380_e9318: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7380_e9321: f64 = if ((locals.var_nuends == 0.0) || (assign7380_e9318 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard199 = assign7380_e9321;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && ((locals.var_guard196 != 0.0) && (locals.var_guard195 == 0.0))) && (locals.var_guard199 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && ((locals.var_guard196 != 0.0) && (locals.var_guard195 == 0.0))) && (locals.var_guard199 == 0.0)) {
            let assign7400_e9382: f64 = (p.p374 * locals.var_weff);
            let assign7400_e9385: f64 = (3.0 * locals.var_nuends);
            let assign7400_e9388: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign7400_e9389: f64 = (assign7400_e9385 * assign7400_e9388);
            let assign7400_e9390: f64 = (assign7400_e9382 / assign7400_e9389);
            locals.var_rend = assign7400_e9390;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 != 0.0)) && (locals.var_guard189 == 0.0)) && (!((locals.var_guard195 != 0.0) || (locals.var_guard196 != 0.0)))) {
            locals.var_rend = 0.0;
        }

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard89 != 0.0) && (!((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0))))) && (locals.var_guard188 == 0.0)) {
            let assign7420_e9441: f64 = (p.p374 * locals.var_dmdgeff);
            let assign7420_e9443: f64 = (assign7420_e9441 / locals.var_weff);
            locals.var_rend = assign7420_e9443;
        }

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

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && (locals.var_guard204 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && (locals.var_guard202 != 0.0)) && (locals.var_guard204 == 0.0)) {
            let assign7490_e9533: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7490_e9536: f64 = (locals.var_weff * locals.var_nuends);
            let assign7490_e9537: f64 = (assign7490_e9533 / assign7490_e9536);
            locals.var_rend = assign7490_e9537;
        }

        let assign7510_e9549: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard206 = assign7510_e9549;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && ((locals.var_guard203 != 0.0) && (locals.var_guard202 == 0.0))) && (locals.var_guard206 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && ((locals.var_guard203 != 0.0) && (locals.var_guard202 == 0.0))) && (locals.var_guard206 == 0.0)) {
            let assign7530_e9612: f64 = (p.p374 * locals.var_weff);
            let assign7530_e9615: f64 = (6.0 * locals.var_nuends);
            let assign7530_e9617: f64 = (assign7530_e9615 * locals.var_dmcgeff);
            let assign7530_e9618: f64 = (assign7530_e9612 / assign7530_e9617);
            locals.var_rend = assign7530_e9618;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 != 0.0)) && (!((locals.var_guard202 != 0.0) || (locals.var_guard203 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign7550_e9660: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard207 = assign7550_e9660;

        let assign7560_e9671: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard208 = assign7560_e9671;

        let assign7570_e9674: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard209 = assign7570_e9674;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard207 != 0.0)) && (locals.var_guard209 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (locals.var_guard207 != 0.0)) && (locals.var_guard209 == 0.0)) {
            let assign7590_e9733: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7590_e9736: f64 = (locals.var_weff * locals.var_nuends);
            let assign7590_e9737: f64 = (assign7590_e9733 / assign7590_e9736);
            locals.var_rend = assign7590_e9737;
        }

        let assign7610_e9749: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard211 = assign7610_e9749;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && ((locals.var_guard208 != 0.0) && (locals.var_guard207 == 0.0))) && (locals.var_guard211 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && ((locals.var_guard208 != 0.0) && (locals.var_guard207 == 0.0))) && (locals.var_guard211 == 0.0)) {
            let assign7630_e9814: f64 = (p.p374 * locals.var_weff);
            let assign7630_e9817: f64 = (6.0 * locals.var_nuends);
            let assign7630_e9819: f64 = (assign7630_e9817 * locals.var_dmcgeff);
            let assign7630_e9820: f64 = (assign7630_e9814 / assign7630_e9819);
            locals.var_rend = assign7630_e9820;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 != 0.0)) && (locals.var_guard201 == 0.0)) && (!((locals.var_guard207 != 0.0) || (locals.var_guard208 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign7650_e9855: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard212 = assign7650_e9855;

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 == 0.0)) && (locals.var_guard212 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard90 != 0.0) && (!(((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0))))) && (locals.var_guard200 == 0.0)) && (locals.var_guard212 == 0.0)) {
            let assign7670_e9906: f64 = (p.p374 * locals.var_dmdgeff);
            let assign7670_e9909: f64 = (locals.var_weff * locals.var_nuendd);
            let assign7670_e9910: f64 = (assign7670_e9906 / assign7670_e9909);
            locals.var_rend = assign7670_e9910;
        }

        let assign7680_e9915: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard213 = assign7680_e9915;

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 != 0.0)) {
            let assign7690_e9939: f64 = (p.p374 * locals.var_dmdgeff);
            let assign7690_e9941: f64 = (assign7690_e9939 / locals.var_weff);
            locals.var_rend = assign7690_e9941;
        }

        let assign7700_e9946: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard214 = assign7700_e9946;

        let assign7710_e9957: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard215 = assign7710_e9957;

        let assign7720_e9968: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard216 = assign7720_e9968;

        let assign7730_e9971: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard217 = assign7730_e9971;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && (locals.var_guard215 != 0.0)) && (locals.var_guard217 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && (locals.var_guard215 != 0.0)) && (locals.var_guard217 == 0.0)) {
            let assign7750_e10034: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7750_e10037: f64 = (locals.var_weff * locals.var_nuendd);
            let assign7750_e10038: f64 = (assign7750_e10034 / assign7750_e10037);
            locals.var_rend = assign7750_e10038;
        }

        let assign7770_e10051: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7770_e10054: f64 = if ((locals.var_nuendd == 0.0) || (assign7770_e10051 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard219 = assign7770_e10054;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && ((locals.var_guard216 != 0.0) && (locals.var_guard215 == 0.0))) && (locals.var_guard219 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && ((locals.var_guard216 != 0.0) && (locals.var_guard215 == 0.0))) && (locals.var_guard219 == 0.0)) {
            let assign7790_e10123: f64 = (p.p374 * locals.var_weff);
            let assign7790_e10126: f64 = (3.0 * locals.var_nuendd);
            let assign7790_e10129: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign7790_e10130: f64 = (assign7790_e10126 * assign7790_e10129);
            let assign7790_e10131: f64 = (assign7790_e10123 / assign7790_e10130);
            locals.var_rend = assign7790_e10131;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 != 0.0)) && (!((locals.var_guard215 != 0.0) || (locals.var_guard216 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign7810_e10176: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard220 = assign7810_e10176;

        let assign7820_e10187: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard221 = assign7820_e10187;

        let assign7830_e10190: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard222 = assign7830_e10190;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && (locals.var_guard220 != 0.0)) && (locals.var_guard222 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && (locals.var_guard220 != 0.0)) && (locals.var_guard222 == 0.0)) {
            let assign7850_e10255: f64 = (p.p374 * locals.var_dmcgeff);
            let assign7850_e10258: f64 = (locals.var_weff * locals.var_nuendd);
            let assign7850_e10259: f64 = (assign7850_e10255 / assign7850_e10258);
            locals.var_rend = assign7850_e10259;
        }

        let assign7870_e10272: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign7870_e10275: f64 = if ((locals.var_nuendd == 0.0) || (assign7870_e10272 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard224 = assign7870_e10275;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && ((locals.var_guard221 != 0.0) && (locals.var_guard220 == 0.0))) && (locals.var_guard224 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && ((locals.var_guard221 != 0.0) && (locals.var_guard220 == 0.0))) && (locals.var_guard224 == 0.0)) {
            let assign7890_e10346: f64 = (p.p374 * locals.var_weff);
            let assign7890_e10349: f64 = (3.0 * locals.var_nuendd);
            let assign7890_e10352: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign7890_e10353: f64 = (assign7890_e10349 * assign7890_e10352);
            let assign7890_e10354: f64 = (assign7890_e10346 / assign7890_e10353);
            locals.var_rend = assign7890_e10354;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard91 != 0.0) && (!((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0))))) && (locals.var_guard213 == 0.0)) && (locals.var_guard214 == 0.0)) && (!((locals.var_guard220 != 0.0) || (locals.var_guard221 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign7910_e10392: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard225 = assign7910_e10392;

        let assign7920_e10395: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard226 = assign7920_e10395;

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 != 0.0)) && (locals.var_guard226 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 != 0.0)) && (locals.var_guard226 == 0.0)) {
            let assign7940_e10452: f64 = (p.p374 * locals.var_dmdgeff);
            let assign7940_e10455: f64 = (locals.var_weff * locals.var_nuends);
            let assign7940_e10456: f64 = (assign7940_e10452 / assign7940_e10455);
            locals.var_rend = assign7940_e10456;
        }

        let assign7950_e10461: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard227 = assign7950_e10461;

        let assign7960_e10472: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard228 = assign7960_e10472;

        let assign7970_e10483: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard229 = assign7970_e10483;

        let assign7980_e10486: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard230 = assign7980_e10486;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && (locals.var_guard228 != 0.0)) && (locals.var_guard230 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && (locals.var_guard228 != 0.0)) && (locals.var_guard230 == 0.0)) {
            let assign8000_e10553: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8000_e10556: f64 = (locals.var_weff * locals.var_nuendd);
            let assign8000_e10557: f64 = (assign8000_e10553 / assign8000_e10556);
            locals.var_rend = assign8000_e10557;
        }

        let assign8020_e10569: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard232 = assign8020_e10569;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && ((locals.var_guard229 != 0.0) && (locals.var_guard228 == 0.0))) && (locals.var_guard232 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && ((locals.var_guard229 != 0.0) && (locals.var_guard228 == 0.0))) && (locals.var_guard232 == 0.0)) {
            let assign8040_e10642: f64 = (p.p374 * locals.var_weff);
            let assign8040_e10645: f64 = (6.0 * locals.var_nuendd);
            let assign8040_e10647: f64 = (assign8040_e10645 * locals.var_dmcgeff);
            let assign8040_e10648: f64 = (assign8040_e10642 / assign8040_e10647);
            locals.var_rend = assign8040_e10648;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 != 0.0)) && (!((locals.var_guard228 != 0.0) || (locals.var_guard229 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign8060_e10695: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard233 = assign8060_e10695;

        let assign8070_e10706: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard234 = assign8070_e10706;

        let assign8080_e10709: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard235 = assign8080_e10709;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && (locals.var_guard233 != 0.0)) && (locals.var_guard235 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && (locals.var_guard233 != 0.0)) && (locals.var_guard235 == 0.0)) {
            let assign8100_e10778: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8100_e10781: f64 = (locals.var_weff * locals.var_nuendd);
            let assign8100_e10782: f64 = (assign8100_e10778 / assign8100_e10781);
            locals.var_rend = assign8100_e10782;
        }

        let assign8120_e10794: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard237 = assign8120_e10794;

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && ((locals.var_guard234 != 0.0) && (locals.var_guard233 == 0.0))) && (locals.var_guard237 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && ((locals.var_guard234 != 0.0) && (locals.var_guard233 == 0.0))) && (locals.var_guard237 == 0.0)) {
            let assign8140_e10869: f64 = (p.p374 * locals.var_weff);
            let assign8140_e10872: f64 = (6.0 * locals.var_nuendd);
            let assign8140_e10874: f64 = (assign8140_e10872 * locals.var_dmcgeff);
            let assign8140_e10875: f64 = (assign8140_e10869 / assign8140_e10874);
            locals.var_rend = assign8140_e10875;
        }

        if ((((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard92 != 0.0) && (!(((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0))))) && (locals.var_guard225 == 0.0)) && (locals.var_guard227 == 0.0)) && (!((locals.var_guard233 != 0.0) || (locals.var_guard234 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        if (((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard93 != 0.0) && (!((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0))))) {
            let assign8160_e10938: f64 = (p.p374 * locals.var_dmdgeff);
            let assign8160_e10940: f64 = (assign8160_e10938 / locals.var_weff);
            locals.var_rend = assign8160_e10940;
        }

        let assign8170_e10945: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard238 = assign8170_e10945;

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 != 0.0)) {
            let assign8180_e10975: f64 = (0.5 * p.p374);
            let assign8180_e10977: f64 = (assign8180_e10975 * locals.var_dmcgeff);
            let assign8180_e10979: f64 = (assign8180_e10977 / locals.var_weff);
            locals.var_rend = assign8180_e10979;
        }

        let assign8190_e10984: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard239 = assign8190_e10984;

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 != 0.0)) && (locals.var_guard239 != 0.0)) {
            locals.var_rint = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 != 0.0)) && (locals.var_guard239 == 0.0)) {
            let assign8210_e11049: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8210_e11053: f64 = (p.p2 - 2.0);
            let assign8210_e11054: f64 = (locals.var_weff * assign8210_e11053);
            let assign8210_e11055: f64 = (assign8210_e11049 / assign8210_e11054);
            locals.var_rint = assign8210_e11055;
        }

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 == 0.0)) {
            locals.var_rend = 0.0;
        }

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard94 != 0.0) && (!(((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0))))) && (locals.var_guard238 == 0.0)) {
            let assign8230_e11119: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8230_e11122: f64 = (locals.var_weff * p.p2);
            let assign8230_e11123: f64 = (assign8230_e11119 / assign8230_e11122);
            locals.var_rint = assign8230_e11123;
        }

        let assign8240_e11128: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard240 = assign8240_e11128;

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 != 0.0)) {
            let assign8260_e11192: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8260_e11195: f64 = (locals.var_weff * p.p2);
            let assign8260_e11196: f64 = (assign8260_e11192 / assign8260_e11195);
            locals.var_rint = assign8260_e11196;
        }

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 == 0.0)) {
            let assign8270_e11231: f64 = (0.5 * p.p374);
            let assign8270_e11233: f64 = (assign8270_e11231 * locals.var_dmcgeff);
            let assign8270_e11235: f64 = (assign8270_e11233 / locals.var_weff);
            locals.var_rend = assign8270_e11235;
        }

        let assign8280_e11240: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard241 = assign8280_e11240;

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 == 0.0)) && (locals.var_guard241 != 0.0)) {
            locals.var_rint = 0.0;
        }

        if (((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && ((locals.var_guard95 != 0.0) && (!((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0))))) && (locals.var_guard240 == 0.0)) && (locals.var_guard241 == 0.0)) {
            let assign8300_e11311: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8300_e11315: f64 = (p.p2 - 2.0);
            let assign8300_e11316: f64 = (locals.var_weff * assign8300_e11315);
            let assign8300_e11317: f64 = (assign8300_e11311 / assign8300_e11316);
            locals.var_rint = assign8300_e11317;
        }

        if (((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (!(((((((((((locals.var_guard85 != 0.0) || (locals.var_guard86 != 0.0)) || (locals.var_guard87 != 0.0)) || (locals.var_guard88 != 0.0)) || (locals.var_guard89 != 0.0)) || (locals.var_guard90 != 0.0)) || (locals.var_guard91 != 0.0)) || (locals.var_guard92 != 0.0)) || (locals.var_guard93 != 0.0)) || (locals.var_guard94 != 0.0)) || (locals.var_guard95 != 0.0)))) {
            locals.var_rint = 0.0;
        }

        let assign8320_e11352: f64 = if locals.var_rint <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard242 = assign8320_e11352;

        if (((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard242 != 0.0)) {
            locals.var_rsourcegeo = locals.var_rend;
        }

        let assign8340_e11364: f64 = if locals.var_rend <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard243 = assign8340_e11364;

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard242 == 0.0)) && (locals.var_guard243 != 0.0)) {
            locals.var_rsourcegeo = locals.var_rint;
        }

        if ((((locals.var_guard77 == 0.0) && (locals.var_guard78 != 0.0)) && (locals.var_guard242 == 0.0)) && (locals.var_guard243 == 0.0)) {
            let assign8360_e11389: f64 = (locals.var_rint * locals.var_rend);
            let assign8360_e11392: f64 = (locals.var_rint + locals.var_rend);
            let assign8360_e11393: f64 = (assign8360_e11389 / assign8360_e11392);
            locals.var_rsourcegeo = assign8360_e11393;
        }

        if ((locals.var_guard77 == 0.0) && (locals.var_guard78 == 0.0)) {
            locals.var_rsourcegeo = 0.0;
        }

        let assign8390_e11408: f64 = if param_given[4] { 1.0 } else { 0.0 };
        locals.var_guard245 = assign8390_e11408;

        if (locals.var_guard245 != 0.0) {
            let assign8400_e11412: f64 = (p.p374 * p.p4);
            locals.var_rdraingeo = assign8400_e11412;
        }

        let assign8410_e11421: f64 = if ((p.p10 > 0.0) && (p.p374 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard246 = assign8410_e11421;

        let assign8420_e11424: f64 = if p.p9 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard247 = assign8420_e11424;

        let assign8430_e11427: f64 = (p.p2 % 2.0);
        let assign8430_e11429: f64 = if assign8430_e11427 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard248 = assign8430_e11429;

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) {
            locals.var_nuendd = 1.0;
            locals.var_nuends = 1.0;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) {
            let assign8460_e11463: f64 = (p.p2 - 1.0);
            let assign8460_e11465: f64 = (assign8460_e11463 / 2.0);
            let assign8460_e11467: f64 = (assign8460_e11465).max(0.0);
            let assign8460_e11468: f64 = (2.0 * assign8460_e11467);
            locals.var_nuintd = assign8460_e11468;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 != 0.0)) {
            locals.var_nuints = locals.var_nuintd;
        }

        let assign8480_e11484: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign8480_e11484;

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 != 0.0)) {
            locals.var_nuendd = 2.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 != 0.0)) {
            let assign8500_e11513: f64 = (p.p2 / 2.0);
            let assign8500_e11515: f64 = (assign8500_e11513 - 1.0);
            let assign8500_e11517: f64 = (assign8500_e11515).max(0.0);
            let assign8500_e11518: f64 = (2.0 * assign8500_e11517);
            locals.var_nuintd = assign8500_e11518;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 != 0.0)) {
            locals.var_nuends = 0.0;
            locals.var_nuints = p.p2;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 == 0.0)) {
            locals.var_nuendd = 0.0;
            locals.var_nuintd = p.p2;
            locals.var_nuends = 2.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard248 == 0.0)) && (locals.var_guard249 == 0.0)) {
            let assign8560_e11609: f64 = (p.p2 / 2.0);
            let assign8560_e11611: f64 = (assign8560_e11609 - 1.0);
            let assign8560_e11613: f64 = (assign8560_e11611).max(0.0);
            let assign8560_e11614: f64 = (2.0 * assign8560_e11613);
            locals.var_nuints = assign8560_e11614;
        }

        let assign8570_e11619: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard250 = assign8570_e11619;

        let assign8580_e11622: f64 = if locals.var_nuints == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign8580_e11622;

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 != 0.0)) && (locals.var_guard251 != 0.0)) {
            locals.var_rint = 0.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 != 0.0)) && (locals.var_guard251 == 0.0)) {
            let assign8600_e11649: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8600_e11652: f64 = (locals.var_weff * locals.var_nuints);
            let assign8600_e11653: f64 = (assign8600_e11649 / assign8600_e11652);
            locals.var_rint = assign8600_e11653;
        }

        let assign8610_e11658: f64 = if locals.var_nuintd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard252 = assign8610_e11658;

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 == 0.0)) && (locals.var_guard252 != 0.0)) {
            locals.var_rint = 0.0;
        }

    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard247 != 0.0)) && (locals.var_guard250 == 0.0)) && (locals.var_guard252 == 0.0)) {
            let assign8630_e11687: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8630_e11690: f64 = (locals.var_weff * locals.var_nuintd);
            let assign8630_e11691: f64 = (assign8630_e11687 / assign8630_e11690);
            locals.var_rint = assign8630_e11691;
        }

        let assign8640_e11696: f64 = if p.p9 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard253 = assign8640_e11696;

        let assign8650_e11699: f64 = if p.p9 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard254 = assign8650_e11699;

        let assign8660_e11702: f64 = if p.p9 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard255 = assign8660_e11702;

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

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard268 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard268 == 0.0)) {
            let assign8810_e11792: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8810_e11795: f64 = (locals.var_weff * locals.var_nuends);
            let assign8810_e11796: f64 = (assign8810_e11792 / assign8810_e11795);
            locals.var_rend = assign8810_e11796;
        }

        let assign8830_e11809: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8830_e11812: f64 = if ((locals.var_nuends == 0.0) || (assign8830_e11809 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard270 = assign8830_e11812;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && ((locals.var_guard267 != 0.0) && (locals.var_guard266 == 0.0))) && (locals.var_guard270 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && ((locals.var_guard267 != 0.0) && (locals.var_guard266 == 0.0))) && (locals.var_guard270 == 0.0)) {
            let assign8850_e11853: f64 = (p.p374 * locals.var_weff);
            let assign8850_e11856: f64 = (3.0 * locals.var_nuends);
            let assign8850_e11859: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign8850_e11860: f64 = (assign8850_e11856 * assign8850_e11859);
            let assign8850_e11861: f64 = (assign8850_e11853 / assign8850_e11860);
            locals.var_rend = assign8850_e11861;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 != 0.0)) && (!((locals.var_guard266 != 0.0) || (locals.var_guard267 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign8870_e11892: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard271 = assign8870_e11892;

        let assign8880_e11903: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard272 = assign8880_e11903;

        let assign8890_e11906: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign8890_e11906;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard271 != 0.0)) && (locals.var_guard273 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard271 != 0.0)) && (locals.var_guard273 == 0.0)) {
            let assign8910_e11943: f64 = (p.p374 * locals.var_dmcgeff);
            let assign8910_e11946: f64 = (locals.var_weff * locals.var_nuends);
            let assign8910_e11947: f64 = (assign8910_e11943 / assign8910_e11946);
            locals.var_rend = assign8910_e11947;
        }

        let assign8930_e11960: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign8930_e11963: f64 = if ((locals.var_nuends == 0.0) || (assign8930_e11960 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard275 = assign8930_e11963;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && ((locals.var_guard272 != 0.0) && (locals.var_guard271 == 0.0))) && (locals.var_guard275 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && ((locals.var_guard272 != 0.0) && (locals.var_guard271 == 0.0))) && (locals.var_guard275 == 0.0)) {
            let assign8950_e12006: f64 = (p.p374 * locals.var_weff);
            let assign8950_e12009: f64 = (3.0 * locals.var_nuends);
            let assign8950_e12012: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign8950_e12013: f64 = (assign8950_e12009 * assign8950_e12012);
            let assign8950_e12014: f64 = (assign8950_e12006 / assign8950_e12013);
            locals.var_rend = assign8950_e12014;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 != 0.0)) && (locals.var_guard265 == 0.0)) && (!((locals.var_guard271 != 0.0) || (locals.var_guard272 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign8970_e12038: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard276 = assign8970_e12038;

        let assign8980_e12049: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard277 = assign8980_e12049;

        let assign8990_e12060: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard278 = assign8990_e12060;

        let assign9000_e12063: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign9000_e12063;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) && (locals.var_guard279 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) && (locals.var_guard279 == 0.0)) {
            let assign9020_e12100: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9020_e12103: f64 = (locals.var_weff * locals.var_nuendd);
            let assign9020_e12104: f64 = (assign9020_e12100 / assign9020_e12103);
            locals.var_rend = assign9020_e12104;
        }

        let assign9040_e12117: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9040_e12120: f64 = if ((locals.var_nuendd == 0.0) || (assign9040_e12117 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard281 = assign9040_e12120;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && ((locals.var_guard278 != 0.0) && (locals.var_guard277 == 0.0))) && (locals.var_guard281 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && ((locals.var_guard278 != 0.0) && (locals.var_guard277 == 0.0))) && (locals.var_guard281 == 0.0)) {
            let assign9060_e12163: f64 = (p.p374 * locals.var_weff);
            let assign9060_e12166: f64 = (3.0 * locals.var_nuendd);
            let assign9060_e12169: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign9060_e12170: f64 = (assign9060_e12166 * assign9060_e12169);
            let assign9060_e12171: f64 = (assign9060_e12163 / assign9060_e12170);
            locals.var_rend = assign9060_e12171;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 != 0.0)) && (!((locals.var_guard277 != 0.0) || (locals.var_guard278 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign9080_e12203: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard282 = assign9080_e12203;

        let assign9090_e12214: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard283 = assign9090_e12214;

        let assign9100_e12217: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard284 = assign9100_e12217;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && (locals.var_guard282 != 0.0)) && (locals.var_guard284 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && (locals.var_guard282 != 0.0)) && (locals.var_guard284 == 0.0)) {
            let assign9120_e12256: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9120_e12259: f64 = (locals.var_weff * locals.var_nuendd);
            let assign9120_e12260: f64 = (assign9120_e12256 / assign9120_e12259);
            locals.var_rend = assign9120_e12260;
        }

        let assign9140_e12273: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9140_e12276: f64 = if ((locals.var_nuendd == 0.0) || (assign9140_e12273 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard286 = assign9140_e12276;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && ((locals.var_guard283 != 0.0) && (locals.var_guard282 == 0.0))) && (locals.var_guard286 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && ((locals.var_guard283 != 0.0) && (locals.var_guard282 == 0.0))) && (locals.var_guard286 == 0.0)) {
            let assign9160_e12321: f64 = (p.p374 * locals.var_weff);
            let assign9160_e12324: f64 = (3.0 * locals.var_nuendd);
            let assign9160_e12327: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign9160_e12328: f64 = (assign9160_e12324 * assign9160_e12327);
            let assign9160_e12329: f64 = (assign9160_e12321 / assign9160_e12328);
            locals.var_rend = assign9160_e12329;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard253 != 0.0)) && (locals.var_guard264 == 0.0)) && (locals.var_guard276 == 0.0)) && (!((locals.var_guard282 != 0.0) || (locals.var_guard283 != 0.0)))) {
            locals.var_rend = 0.0;
        }

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

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (locals.var_guard291 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (locals.var_guard291 == 0.0)) {
            let assign9240_e12423: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9240_e12426: f64 = (locals.var_weff * locals.var_nuends);
            let assign9240_e12427: f64 = (assign9240_e12423 / assign9240_e12426);
            locals.var_rend = assign9240_e12427;
        }

        let assign9260_e12440: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9260_e12443: f64 = if ((locals.var_nuends == 0.0) || (assign9260_e12440 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard293 = assign9260_e12443;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && ((locals.var_guard290 != 0.0) && (locals.var_guard289 == 0.0))) && (locals.var_guard293 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && ((locals.var_guard290 != 0.0) && (locals.var_guard289 == 0.0))) && (locals.var_guard293 == 0.0)) {
            let assign9280_e12490: f64 = (p.p374 * locals.var_weff);
            let assign9280_e12493: f64 = (3.0 * locals.var_nuends);
            let assign9280_e12496: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign9280_e12497: f64 = (assign9280_e12493 * assign9280_e12496);
            let assign9280_e12498: f64 = (assign9280_e12490 / assign9280_e12497);
            locals.var_rend = assign9280_e12498;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 != 0.0)) && (!((locals.var_guard289 != 0.0) || (locals.var_guard290 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign9300_e12532: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard294 = assign9300_e12532;

        let assign9310_e12543: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard295 = assign9310_e12543;

        let assign9320_e12546: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard296 = assign9320_e12546;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard294 != 0.0)) && (locals.var_guard296 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard294 != 0.0)) && (locals.var_guard296 == 0.0)) {
            let assign9340_e12589: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9340_e12592: f64 = (locals.var_weff * locals.var_nuends);
            let assign9340_e12593: f64 = (assign9340_e12589 / assign9340_e12592);
            locals.var_rend = assign9340_e12593;
        }

        let assign9360_e12606: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9360_e12609: f64 = if ((locals.var_nuends == 0.0) || (assign9360_e12606 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard298 = assign9360_e12609;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && ((locals.var_guard295 != 0.0) && (locals.var_guard294 == 0.0))) && (locals.var_guard298 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && ((locals.var_guard295 != 0.0) && (locals.var_guard294 == 0.0))) && (locals.var_guard298 == 0.0)) {
            let assign9380_e12658: f64 = (p.p374 * locals.var_weff);
            let assign9380_e12661: f64 = (3.0 * locals.var_nuends);
            let assign9380_e12664: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign9380_e12665: f64 = (assign9380_e12661 * assign9380_e12664);
            let assign9380_e12666: f64 = (assign9380_e12658 / assign9380_e12665);
            locals.var_rend = assign9380_e12666;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 != 0.0)) && (locals.var_guard288 == 0.0)) && (!((locals.var_guard294 != 0.0) || (locals.var_guard295 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign9400_e12693: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard299 = assign9400_e12693;

        let assign9410_e12704: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard300 = assign9410_e12704;

        let assign9420_e12715: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard301 = assign9420_e12715;

        let assign9430_e12718: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard302 = assign9430_e12718;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && (locals.var_guard300 != 0.0)) && (locals.var_guard302 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && (locals.var_guard300 != 0.0)) && (locals.var_guard302 == 0.0)) {
            let assign9450_e12761: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9450_e12764: f64 = (locals.var_weff * locals.var_nuendd);
            let assign9450_e12765: f64 = (assign9450_e12761 / assign9450_e12764);
            locals.var_rend = assign9450_e12765;
        }

        let assign9470_e12777: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard304 = assign9470_e12777;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && ((locals.var_guard301 != 0.0) && (locals.var_guard300 == 0.0))) && (locals.var_guard304 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && ((locals.var_guard301 != 0.0) && (locals.var_guard300 == 0.0))) && (locals.var_guard304 == 0.0)) {
            let assign9490_e12826: f64 = (p.p374 * locals.var_weff);
            let assign9490_e12829: f64 = (6.0 * locals.var_nuendd);
            let assign9490_e12831: f64 = (assign9490_e12829 * locals.var_dmcgeff);
            let assign9490_e12832: f64 = (assign9490_e12826 / assign9490_e12831);
            locals.var_rend = assign9490_e12832;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 != 0.0)) && (!((locals.var_guard300 != 0.0) || (locals.var_guard301 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign9510_e12867: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard305 = assign9510_e12867;

        let assign9520_e12878: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard306 = assign9520_e12878;

        let assign9530_e12881: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard307 = assign9530_e12881;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 == 0.0)) && (locals.var_guard305 != 0.0)) && (locals.var_guard307 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 == 0.0)) && (locals.var_guard305 != 0.0)) && (locals.var_guard307 == 0.0)) {
            let assign9550_e12926: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9550_e12929: f64 = (locals.var_weff * locals.var_nuendd);
            let assign9550_e12930: f64 = (assign9550_e12926 / assign9550_e12929);
            locals.var_rend = assign9550_e12930;
        }

        let assign9570_e12942: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard309 = assign9570_e12942;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 == 0.0)) && ((locals.var_guard306 != 0.0) && (locals.var_guard305 == 0.0))) && (locals.var_guard309 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 == 0.0)) && ((locals.var_guard306 != 0.0) && (locals.var_guard305 == 0.0))) && (locals.var_guard309 == 0.0)) {
            let assign9590_e12993: f64 = (p.p374 * locals.var_weff);
            let assign9590_e12996: f64 = (6.0 * locals.var_nuendd);
            let assign9590_e12998: f64 = (assign9590_e12996 * locals.var_dmcgeff);
            let assign9590_e12999: f64 = (assign9590_e12993 / assign9590_e12998);
            locals.var_rend = assign9590_e12999;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard254 != 0.0) && (locals.var_guard253 == 0.0))) && (locals.var_guard287 == 0.0)) && (locals.var_guard299 == 0.0)) && (!((locals.var_guard305 != 0.0) || (locals.var_guard306 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign9610_e13027: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard310 = assign9610_e13027;

        let assign9620_e13030: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign9620_e13030;

        let assign9630_e13041: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard312 = assign9630_e13041;

        let assign9640_e13052: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard313 = assign9640_e13052;

        let assign9650_e13055: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard314 = assign9650_e13055;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard314 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) && (locals.var_guard312 != 0.0)) && (locals.var_guard314 == 0.0)) {
            let assign9670_e13100: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9670_e13103: f64 = (locals.var_weff * locals.var_nuends);
            let assign9670_e13104: f64 = (assign9670_e13100 / assign9670_e13103);
            locals.var_rend = assign9670_e13104;
        }

        let assign9690_e13116: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard316 = assign9690_e13116;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) && ((locals.var_guard313 != 0.0) && (locals.var_guard312 == 0.0))) && (locals.var_guard316 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) && ((locals.var_guard313 != 0.0) && (locals.var_guard312 == 0.0))) && (locals.var_guard316 == 0.0)) {
            let assign9710_e13167: f64 = (p.p374 * locals.var_weff);
            let assign9710_e13170: f64 = (6.0 * locals.var_nuends);
            let assign9710_e13172: f64 = (assign9710_e13170 * locals.var_dmcgeff);
            let assign9710_e13173: f64 = (assign9710_e13167 / assign9710_e13172);
            locals.var_rend = assign9710_e13173;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) && (!((locals.var_guard312 != 0.0) || (locals.var_guard313 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign9730_e13209: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard317 = assign9730_e13209;

        let assign9740_e13220: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard318 = assign9740_e13220;

        let assign9750_e13223: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard319 = assign9750_e13223;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && (locals.var_guard317 != 0.0)) && (locals.var_guard319 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && (locals.var_guard317 != 0.0)) && (locals.var_guard319 == 0.0)) {
            let assign9770_e13270: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9770_e13273: f64 = (locals.var_weff * locals.var_nuends);
            let assign9770_e13274: f64 = (assign9770_e13270 / assign9770_e13273);
            locals.var_rend = assign9770_e13274;
        }

        let assign9790_e13286: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard321 = assign9790_e13286;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && ((locals.var_guard318 != 0.0) && (locals.var_guard317 == 0.0))) && (locals.var_guard321 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && ((locals.var_guard318 != 0.0) && (locals.var_guard317 == 0.0))) && (locals.var_guard321 == 0.0)) {
            let assign9810_e13339: f64 = (p.p374 * locals.var_weff);
            let assign9810_e13342: f64 = (6.0 * locals.var_nuends);
            let assign9810_e13344: f64 = (assign9810_e13342 * locals.var_dmcgeff);
            let assign9810_e13345: f64 = (assign9810_e13339 / assign9810_e13344);
            locals.var_rend = assign9810_e13345;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 == 0.0)) && (!((locals.var_guard317 != 0.0) || (locals.var_guard318 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign9830_e13374: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard322 = assign9830_e13374;

        let assign9840_e13385: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard323 = assign9840_e13385;

        let assign9850_e13396: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard324 = assign9850_e13396;

        let assign9860_e13399: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard325 = assign9860_e13399;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 != 0.0)) && (locals.var_guard323 != 0.0)) && (locals.var_guard325 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 != 0.0)) && (locals.var_guard323 != 0.0)) && (locals.var_guard325 == 0.0)) {
            let assign9880_e13446: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9880_e13449: f64 = (locals.var_weff * locals.var_nuendd);
            let assign9880_e13450: f64 = (assign9880_e13446 / assign9880_e13449);
            locals.var_rend = assign9880_e13450;
        }

        let assign9900_e13463: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign9900_e13466: f64 = if ((locals.var_nuendd == 0.0) || (assign9900_e13463 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard327 = assign9900_e13466;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 != 0.0)) && ((locals.var_guard324 != 0.0) && (locals.var_guard323 == 0.0))) && (locals.var_guard327 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 != 0.0)) && ((locals.var_guard324 != 0.0) && (locals.var_guard323 == 0.0))) && (locals.var_guard327 == 0.0)) {
            let assign9920_e13519: f64 = (p.p374 * locals.var_weff);
            let assign9920_e13522: f64 = (3.0 * locals.var_nuendd);
            let assign9920_e13525: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign9920_e13526: f64 = (assign9920_e13522 * assign9920_e13525);
            let assign9920_e13527: f64 = (assign9920_e13519 / assign9920_e13526);
            locals.var_rend = assign9920_e13527;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 != 0.0)) && (!((locals.var_guard323 != 0.0) || (locals.var_guard324 != 0.0)))) {
            locals.var_rend = 0.0;
        }

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign9940_e13564: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard328 = assign9940_e13564;

        let assign9950_e13575: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard329 = assign9950_e13575;

        let assign9960_e13578: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard330 = assign9960_e13578;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 == 0.0)) && (locals.var_guard328 != 0.0)) && (locals.var_guard330 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 == 0.0)) && (locals.var_guard328 != 0.0)) && (locals.var_guard330 == 0.0)) {
            let assign9980_e13627: f64 = (p.p374 * locals.var_dmcgeff);
            let assign9980_e13630: f64 = (locals.var_weff * locals.var_nuendd);
            let assign9980_e13631: f64 = (assign9980_e13627 / assign9980_e13630);
            locals.var_rend = assign9980_e13631;
        }

        let assign10000_e13644: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10000_e13647: f64 = if ((locals.var_nuendd == 0.0) || (assign10000_e13644 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard332 = assign10000_e13647;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 == 0.0)) && ((locals.var_guard329 != 0.0) && (locals.var_guard328 == 0.0))) && (locals.var_guard332 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 == 0.0)) && ((locals.var_guard329 != 0.0) && (locals.var_guard328 == 0.0))) && (locals.var_guard332 == 0.0)) {
            let assign10020_e13702: f64 = (p.p374 * locals.var_weff);
            let assign10020_e13705: f64 = (3.0 * locals.var_nuendd);
            let assign10020_e13708: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign10020_e13709: f64 = (assign10020_e13705 * assign10020_e13708);
            let assign10020_e13710: f64 = (assign10020_e13702 / assign10020_e13709);
            locals.var_rend = assign10020_e13710;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard255 != 0.0) && (!((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0))))) && (locals.var_guard310 == 0.0)) && (locals.var_guard322 == 0.0)) && (!((locals.var_guard328 != 0.0) || (locals.var_guard329 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign10040_e13740: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard333 = assign10040_e13740;

        let assign10050_e13743: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard334 = assign10050_e13743;

        let assign10060_e13754: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard335 = assign10060_e13754;

        let assign10070_e13765: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard336 = assign10070_e13765;

        let assign10080_e13768: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard337 = assign10080_e13768;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && (locals.var_guard337 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) && (locals.var_guard335 != 0.0)) && (locals.var_guard337 == 0.0)) {
            let assign10100_e13817: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10100_e13820: f64 = (locals.var_weff * locals.var_nuends);
            let assign10100_e13821: f64 = (assign10100_e13817 / assign10100_e13820);
            locals.var_rend = assign10100_e13821;
        }

        let assign10120_e13833: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard339 = assign10120_e13833;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) && ((locals.var_guard336 != 0.0) && (locals.var_guard335 == 0.0))) && (locals.var_guard339 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) && ((locals.var_guard336 != 0.0) && (locals.var_guard335 == 0.0))) && (locals.var_guard339 == 0.0)) {
            let assign10140_e13888: f64 = (p.p374 * locals.var_weff);
            let assign10140_e13891: f64 = (6.0 * locals.var_nuends);
            let assign10140_e13893: f64 = (assign10140_e13891 * locals.var_dmcgeff);
            let assign10140_e13894: f64 = (assign10140_e13888 / assign10140_e13893);
            locals.var_rend = assign10140_e13894;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 != 0.0)) && (!((locals.var_guard335 != 0.0) || (locals.var_guard336 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign10160_e13932: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard340 = assign10160_e13932;

        let assign10170_e13943: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard341 = assign10170_e13943;

        let assign10180_e13946: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard342 = assign10180_e13946;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard342 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) && (locals.var_guard340 != 0.0)) && (locals.var_guard342 == 0.0)) {
            let assign10200_e13997: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10200_e14000: f64 = (locals.var_weff * locals.var_nuends);
            let assign10200_e14001: f64 = (assign10200_e13997 / assign10200_e14000);
            locals.var_rend = assign10200_e14001;
        }

        let assign10220_e14013: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard344 = assign10220_e14013;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) && ((locals.var_guard341 != 0.0) && (locals.var_guard340 == 0.0))) && (locals.var_guard344 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) && ((locals.var_guard341 != 0.0) && (locals.var_guard340 == 0.0))) && (locals.var_guard344 == 0.0)) {
            let assign10240_e14070: f64 = (p.p374 * locals.var_weff);
            let assign10240_e14073: f64 = (6.0 * locals.var_nuends);
            let assign10240_e14075: f64 = (assign10240_e14073 * locals.var_dmcgeff);
            let assign10240_e14076: f64 = (assign10240_e14070 / assign10240_e14075);
            locals.var_rend = assign10240_e14076;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 != 0.0)) && (locals.var_guard334 == 0.0)) && (!((locals.var_guard340 != 0.0) || (locals.var_guard341 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign10260_e14107: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard345 = assign10260_e14107;

        let assign10270_e14118: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard346 = assign10270_e14118;

        let assign10280_e14129: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard347 = assign10280_e14129;

        let assign10290_e14132: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard348 = assign10290_e14132;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 != 0.0)) && (locals.var_guard346 != 0.0)) && (locals.var_guard348 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 != 0.0)) && (locals.var_guard346 != 0.0)) && (locals.var_guard348 == 0.0)) {
            let assign10310_e14183: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10310_e14186: f64 = (locals.var_weff * locals.var_nuendd);
            let assign10310_e14187: f64 = (assign10310_e14183 / assign10310_e14186);
            locals.var_rend = assign10310_e14187;
        }

        let assign10330_e14199: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard350 = assign10330_e14199;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 != 0.0)) && ((locals.var_guard347 != 0.0) && (locals.var_guard346 == 0.0))) && (locals.var_guard350 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 != 0.0)) && ((locals.var_guard347 != 0.0) && (locals.var_guard346 == 0.0))) && (locals.var_guard350 == 0.0)) {
            let assign10350_e14256: f64 = (p.p374 * locals.var_weff);
            let assign10350_e14259: f64 = (6.0 * locals.var_nuendd);
            let assign10350_e14261: f64 = (assign10350_e14259 * locals.var_dmcgeff);
            let assign10350_e14262: f64 = (assign10350_e14256 / assign10350_e14261);
            locals.var_rend = assign10350_e14262;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 != 0.0)) && (!((locals.var_guard346 != 0.0) || (locals.var_guard347 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign10370_e14301: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard351 = assign10370_e14301;

        let assign10380_e14312: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard352 = assign10380_e14312;

        let assign10390_e14315: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard353 = assign10390_e14315;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 == 0.0)) && (locals.var_guard351 != 0.0)) && (locals.var_guard353 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 == 0.0)) && (locals.var_guard351 != 0.0)) && (locals.var_guard353 == 0.0)) {
            let assign10410_e14368: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10410_e14371: f64 = (locals.var_weff * locals.var_nuendd);
            let assign10410_e14372: f64 = (assign10410_e14368 / assign10410_e14371);
            locals.var_rend = assign10410_e14372;
        }

        let assign10430_e14384: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard355 = assign10430_e14384;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 == 0.0)) && ((locals.var_guard352 != 0.0) && (locals.var_guard351 == 0.0))) && (locals.var_guard355 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 == 0.0)) && ((locals.var_guard352 != 0.0) && (locals.var_guard351 == 0.0))) && (locals.var_guard355 == 0.0)) {
            let assign10450_e14443: f64 = (p.p374 * locals.var_weff);
            let assign10450_e14446: f64 = (6.0 * locals.var_nuendd);
            let assign10450_e14448: f64 = (assign10450_e14446 * locals.var_dmcgeff);
            let assign10450_e14449: f64 = (assign10450_e14443 / assign10450_e14448);
            locals.var_rend = assign10450_e14449;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard256 != 0.0) && (!(((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0))))) && (locals.var_guard333 == 0.0)) && (locals.var_guard345 == 0.0)) && (!((locals.var_guard351 != 0.0) || (locals.var_guard352 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign10470_e14481: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard356 = assign10470_e14481;

        let assign10480_e14484: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard357 = assign10480_e14484;

        let assign10490_e14495: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard358 = assign10490_e14495;

        let assign10500_e14506: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard359 = assign10500_e14506;

        let assign10510_e14509: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard360 = assign10510_e14509;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard360 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 != 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard360 == 0.0)) {
            let assign10530_e14562: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10530_e14565: f64 = (locals.var_weff * locals.var_nuends);
            let assign10530_e14566: f64 = (assign10530_e14562 / assign10530_e14565);
            locals.var_rend = assign10530_e14566;
        }

        let assign10550_e14579: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10550_e14582: f64 = if ((locals.var_nuends == 0.0) || (assign10550_e14579 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard362 = assign10550_e14582;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 != 0.0)) && ((locals.var_guard359 != 0.0) && (locals.var_guard358 == 0.0))) && (locals.var_guard362 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 != 0.0)) && ((locals.var_guard359 != 0.0) && (locals.var_guard358 == 0.0))) && (locals.var_guard362 == 0.0)) {
            let assign10570_e14641: f64 = (p.p374 * locals.var_weff);
            let assign10570_e14644: f64 = (3.0 * locals.var_nuends);
            let assign10570_e14647: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign10570_e14648: f64 = (assign10570_e14644 * assign10570_e14647);
            let assign10570_e14649: f64 = (assign10570_e14641 / assign10570_e14648);
            locals.var_rend = assign10570_e14649;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 != 0.0)) && (!((locals.var_guard358 != 0.0) || (locals.var_guard359 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign10590_e14689: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard363 = assign10590_e14689;

        let assign10600_e14700: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard364 = assign10600_e14700;

        let assign10610_e14703: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard365 = assign10610_e14703;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && (locals.var_guard363 != 0.0)) && (locals.var_guard365 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && (locals.var_guard363 != 0.0)) && (locals.var_guard365 == 0.0)) {
            let assign10630_e14758: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10630_e14761: f64 = (locals.var_weff * locals.var_nuends);
            let assign10630_e14762: f64 = (assign10630_e14758 / assign10630_e14761);
            locals.var_rend = assign10630_e14762;
        }

        let assign10650_e14775: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign10650_e14778: f64 = if ((locals.var_nuends == 0.0) || (assign10650_e14775 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard367 = assign10650_e14778;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && ((locals.var_guard364 != 0.0) && (locals.var_guard363 == 0.0))) && (locals.var_guard367 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && ((locals.var_guard364 != 0.0) && (locals.var_guard363 == 0.0))) && (locals.var_guard367 == 0.0)) {
            let assign10670_e14839: f64 = (p.p374 * locals.var_weff);
            let assign10670_e14842: f64 = (3.0 * locals.var_nuends);
            let assign10670_e14845: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign10670_e14846: f64 = (assign10670_e14842 * assign10670_e14845);
            let assign10670_e14847: f64 = (assign10670_e14839 / assign10670_e14846);
            locals.var_rend = assign10670_e14847;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 != 0.0)) && (locals.var_guard357 == 0.0)) && (!((locals.var_guard363 != 0.0) || (locals.var_guard364 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard257 != 0.0) && (!((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0))))) && (locals.var_guard356 == 0.0)) {
            let assign10690_e14898: f64 = (p.p374 * locals.var_dmdgeff);
            let assign10690_e14900: f64 = (assign10690_e14898 / locals.var_weff);
            locals.var_rend = assign10690_e14900;
        }

        let assign10700_e14905: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard368 = assign10700_e14905;

        let assign10710_e14908: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard369 = assign10710_e14908;

        let assign10720_e14919: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard370 = assign10720_e14919;

        let assign10730_e14930: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard371 = assign10730_e14930;

        let assign10740_e14933: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard372 = assign10740_e14933;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (locals.var_guard372 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && (locals.var_guard370 != 0.0)) && (locals.var_guard372 == 0.0)) {
            let assign10760_e14990: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10760_e14993: f64 = (locals.var_weff * locals.var_nuends);
            let assign10760_e14994: f64 = (assign10760_e14990 / assign10760_e14993);
            locals.var_rend = assign10760_e14994;
        }

        let assign10780_e15006: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard374 = assign10780_e15006;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && ((locals.var_guard371 != 0.0) && (locals.var_guard370 == 0.0))) && (locals.var_guard374 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && ((locals.var_guard371 != 0.0) && (locals.var_guard370 == 0.0))) && (locals.var_guard374 == 0.0)) {
            let assign10800_e15069: f64 = (p.p374 * locals.var_weff);
            let assign10800_e15072: f64 = (6.0 * locals.var_nuends);
            let assign10800_e15074: f64 = (assign10800_e15072 * locals.var_dmcgeff);
            let assign10800_e15075: f64 = (assign10800_e15069 / assign10800_e15074);
            locals.var_rend = assign10800_e15075;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 != 0.0)) && (!((locals.var_guard370 != 0.0) || (locals.var_guard371 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign10820_e15117: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard375 = assign10820_e15117;

        let assign10830_e15128: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard376 = assign10830_e15128;

        let assign10840_e15131: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard377 = assign10840_e15131;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard375 != 0.0)) && (locals.var_guard377 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && (locals.var_guard375 != 0.0)) && (locals.var_guard377 == 0.0)) {
            let assign10860_e15190: f64 = (p.p374 * locals.var_dmcgeff);
            let assign10860_e15193: f64 = (locals.var_weff * locals.var_nuends);
            let assign10860_e15194: f64 = (assign10860_e15190 / assign10860_e15193);
            locals.var_rend = assign10860_e15194;
        }

        let assign10880_e15206: f64 = if ((locals.var_nuends == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard379 = assign10880_e15206;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && ((locals.var_guard376 != 0.0) && (locals.var_guard375 == 0.0))) && (locals.var_guard379 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && ((locals.var_guard376 != 0.0) && (locals.var_guard375 == 0.0))) && (locals.var_guard379 == 0.0)) {
            let assign10900_e15271: f64 = (p.p374 * locals.var_weff);
            let assign10900_e15274: f64 = (6.0 * locals.var_nuends);
            let assign10900_e15276: f64 = (assign10900_e15274 * locals.var_dmcgeff);
            let assign10900_e15277: f64 = (assign10900_e15271 / assign10900_e15276);
            locals.var_rend = assign10900_e15277;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 != 0.0)) && (locals.var_guard369 == 0.0)) && (!((locals.var_guard375 != 0.0) || (locals.var_guard376 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign10920_e15312: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard380 = assign10920_e15312;

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 == 0.0)) && (locals.var_guard380 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard258 != 0.0) && (!(((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0))))) && (locals.var_guard368 == 0.0)) && (locals.var_guard380 == 0.0)) {
            let assign10940_e15363: f64 = (p.p374 * locals.var_dmdgeff);
            let assign10940_e15366: f64 = (locals.var_weff * locals.var_nuendd);
            let assign10940_e15367: f64 = (assign10940_e15363 / assign10940_e15366);
            locals.var_rend = assign10940_e15367;
        }

        let assign10950_e15372: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard381 = assign10950_e15372;

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 != 0.0)) {
            let assign10960_e15396: f64 = (p.p374 * locals.var_dmdgeff);
            let assign10960_e15398: f64 = (assign10960_e15396 / locals.var_weff);
            locals.var_rend = assign10960_e15398;
        }

        let assign10970_e15403: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard382 = assign10970_e15403;

        let assign10980_e15414: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard383 = assign10980_e15414;

        let assign10990_e15425: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard384 = assign10990_e15425;

        let assign11000_e15428: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard385 = assign11000_e15428;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
            let assign11020_e15491: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11020_e15494: f64 = (locals.var_weff * locals.var_nuendd);
            let assign11020_e15495: f64 = (assign11020_e15491 / assign11020_e15494);
            locals.var_rend = assign11020_e15495;
        }

        let assign11040_e15508: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11040_e15511: f64 = if ((locals.var_nuendd == 0.0) || (assign11040_e15508 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard387 = assign11040_e15511;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && ((locals.var_guard384 != 0.0) && (locals.var_guard383 == 0.0))) && (locals.var_guard387 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && ((locals.var_guard384 != 0.0) && (locals.var_guard383 == 0.0))) && (locals.var_guard387 == 0.0)) {
            let assign11060_e15580: f64 = (p.p374 * locals.var_weff);
            let assign11060_e15583: f64 = (3.0 * locals.var_nuendd);
            let assign11060_e15586: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign11060_e15587: f64 = (assign11060_e15583 * assign11060_e15586);
            let assign11060_e15588: f64 = (assign11060_e15580 / assign11060_e15587);
            locals.var_rend = assign11060_e15588;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 != 0.0)) && (!((locals.var_guard383 != 0.0) || (locals.var_guard384 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign11080_e15633: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard388 = assign11080_e15633;

        let assign11090_e15644: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard389 = assign11090_e15644;

        let assign11100_e15647: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard390 = assign11100_e15647;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard388 != 0.0)) && (locals.var_guard390 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && (locals.var_guard388 != 0.0)) && (locals.var_guard390 == 0.0)) {
            let assign11120_e15712: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11120_e15715: f64 = (locals.var_weff * locals.var_nuendd);
            let assign11120_e15716: f64 = (assign11120_e15712 / assign11120_e15715);
            locals.var_rend = assign11120_e15716;
        }

        let assign11140_e15729: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        let assign11140_e15732: f64 = if ((locals.var_nuendd == 0.0) || (assign11140_e15729 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard392 = assign11140_e15732;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && ((locals.var_guard389 != 0.0) && (locals.var_guard388 == 0.0))) && (locals.var_guard392 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && ((locals.var_guard389 != 0.0) && (locals.var_guard388 == 0.0))) && (locals.var_guard392 == 0.0)) {
            let assign11160_e15803: f64 = (p.p374 * locals.var_weff);
            let assign11160_e15806: f64 = (3.0 * locals.var_nuendd);
            let assign11160_e15809: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
            let assign11160_e15810: f64 = (assign11160_e15806 * assign11160_e15809);
            let assign11160_e15811: f64 = (assign11160_e15803 / assign11160_e15810);
            locals.var_rend = assign11160_e15811;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard259 != 0.0) && (!((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0))))) && (locals.var_guard381 == 0.0)) && (locals.var_guard382 == 0.0)) && (!((locals.var_guard388 != 0.0) || (locals.var_guard389 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign11180_e15849: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard393 = assign11180_e15849;

        let assign11190_e15852: f64 = if locals.var_nuends == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard394 = assign11190_e15852;

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 != 0.0)) && (locals.var_guard394 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 != 0.0)) && (locals.var_guard394 == 0.0)) {
            let assign11210_e15909: f64 = (p.p374 * locals.var_dmdgeff);
            let assign11210_e15912: f64 = (locals.var_weff * locals.var_nuends);
            let assign11210_e15913: f64 = (assign11210_e15909 / assign11210_e15912);
            locals.var_rend = assign11210_e15913;
        }

        let assign11220_e15918: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard395 = assign11220_e15918;

    }

    pub(super) fn stamp_transient_block_10(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let assign11230_e15929: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        locals.var_guard396 = assign11230_e15929;

        let assign11240_e15940: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        locals.var_guard397 = assign11240_e15940;

        let assign11250_e15943: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard398 = assign11250_e15943;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard398 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && (locals.var_guard396 != 0.0)) && (locals.var_guard398 == 0.0)) {
            let assign11270_e16010: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11270_e16013: f64 = (locals.var_weff * locals.var_nuendd);
            let assign11270_e16014: f64 = (assign11270_e16010 / assign11270_e16013);
            locals.var_rend = assign11270_e16014;
        }

        let assign11290_e16026: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard400 = assign11290_e16026;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && ((locals.var_guard397 != 0.0) && (locals.var_guard396 == 0.0))) && (locals.var_guard400 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && ((locals.var_guard397 != 0.0) && (locals.var_guard396 == 0.0))) && (locals.var_guard400 == 0.0)) {
            let assign11310_e16099: f64 = (p.p374 * locals.var_weff);
            let assign11310_e16102: f64 = (6.0 * locals.var_nuendd);
            let assign11310_e16104: f64 = (assign11310_e16102 * locals.var_dmcgeff);
            let assign11310_e16105: f64 = (assign11310_e16099 / assign11310_e16104);
            locals.var_rend = assign11310_e16105;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 != 0.0)) && (!((locals.var_guard396 != 0.0) || (locals.var_guard397 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        let assign11330_e16152: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        locals.var_guard401 = assign11330_e16152;

        let assign11340_e16163: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard402 = assign11340_e16163;

        let assign11350_e16166: f64 = if locals.var_nuendd == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard403 = assign11350_e16166;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && (locals.var_guard401 != 0.0)) && (locals.var_guard403 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && (locals.var_guard401 != 0.0)) && (locals.var_guard403 == 0.0)) {
            let assign11370_e16235: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11370_e16238: f64 = (locals.var_weff * locals.var_nuendd);
            let assign11370_e16239: f64 = (assign11370_e16235 / assign11370_e16238);
            locals.var_rend = assign11370_e16239;
        }

        let assign11390_e16251: f64 = if ((locals.var_nuendd == 0.0) || (locals.var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard405 = assign11390_e16251;

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && ((locals.var_guard402 != 0.0) && (locals.var_guard401 == 0.0))) && (locals.var_guard405 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if (((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && ((locals.var_guard402 != 0.0) && (locals.var_guard401 == 0.0))) && (locals.var_guard405 == 0.0)) {
            let assign11410_e16326: f64 = (p.p374 * locals.var_weff);
            let assign11410_e16329: f64 = (6.0 * locals.var_nuendd);
            let assign11410_e16331: f64 = (assign11410_e16329 * locals.var_dmcgeff);
            let assign11410_e16332: f64 = (assign11410_e16326 / assign11410_e16331);
            locals.var_rend = assign11410_e16332;
        }

        if ((((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard260 != 0.0) && (!(((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0))))) && (locals.var_guard393 == 0.0)) && (locals.var_guard395 == 0.0)) && (!((locals.var_guard401 != 0.0) || (locals.var_guard402 != 0.0)))) {
            locals.var_rend = 0.0;
        }

        if (((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard261 != 0.0) && (!((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0))))) {
            let assign11430_e16395: f64 = (p.p374 * locals.var_dmdgeff);
            let assign11430_e16397: f64 = (assign11430_e16395 / locals.var_weff);
            locals.var_rend = assign11430_e16397;
        }

        let assign11440_e16402: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard406 = assign11440_e16402;

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 != 0.0)) {
            let assign11450_e16432: f64 = (0.5 * p.p374);
            let assign11450_e16434: f64 = (assign11450_e16432 * locals.var_dmcgeff);
            let assign11450_e16436: f64 = (assign11450_e16434 / locals.var_weff);
            locals.var_rend = assign11450_e16436;
        }

        let assign11460_e16441: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign11460_e16441;

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 != 0.0)) && (locals.var_guard407 != 0.0)) {
            locals.var_rint = 0.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 != 0.0)) && (locals.var_guard407 == 0.0)) {
            let assign11480_e16506: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11480_e16510: f64 = (p.p2 - 2.0);
            let assign11480_e16511: f64 = (locals.var_weff * assign11480_e16510);
            let assign11480_e16512: f64 = (assign11480_e16506 / assign11480_e16511);
            locals.var_rint = assign11480_e16512;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 == 0.0)) {
            locals.var_rend = 0.0;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard262 != 0.0) && (!(((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0))))) && (locals.var_guard406 == 0.0)) {
            let assign11500_e16576: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11500_e16579: f64 = (locals.var_weff * p.p2);
            let assign11500_e16580: f64 = (assign11500_e16576 / assign11500_e16579);
            locals.var_rint = assign11500_e16580;
        }

        let assign11510_e16585: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard408 = assign11510_e16585;

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 != 0.0)) {
            locals.var_rend = 0.0;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 != 0.0)) {
            let assign11530_e16649: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11530_e16652: f64 = (locals.var_weff * p.p2);
            let assign11530_e16653: f64 = (assign11530_e16649 / assign11530_e16652);
            locals.var_rint = assign11530_e16653;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 == 0.0)) {
            let assign11540_e16688: f64 = (0.5 * p.p374);
            let assign11540_e16690: f64 = (assign11540_e16688 * locals.var_dmcgeff);
            let assign11540_e16692: f64 = (assign11540_e16690 / locals.var_weff);
            locals.var_rend = assign11540_e16692;
        }

        let assign11550_e16697: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard409 = assign11550_e16697;

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 == 0.0)) && (locals.var_guard409 != 0.0)) {
            locals.var_rint = 0.0;
        }

        if (((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && ((locals.var_guard263 != 0.0) && (!((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0))))) && (locals.var_guard408 == 0.0)) && (locals.var_guard409 == 0.0)) {
            let assign11570_e16768: f64 = (p.p374 * locals.var_dmcgeff);
            let assign11570_e16772: f64 = (p.p2 - 2.0);
            let assign11570_e16773: f64 = (locals.var_weff * assign11570_e16772);
            let assign11570_e16774: f64 = (assign11570_e16768 / assign11570_e16773);
            locals.var_rint = assign11570_e16774;
        }

        if (((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (!(((((((((((locals.var_guard253 != 0.0) || (locals.var_guard254 != 0.0)) || (locals.var_guard255 != 0.0)) || (locals.var_guard256 != 0.0)) || (locals.var_guard257 != 0.0)) || (locals.var_guard258 != 0.0)) || (locals.var_guard259 != 0.0)) || (locals.var_guard260 != 0.0)) || (locals.var_guard261 != 0.0)) || (locals.var_guard262 != 0.0)) || (locals.var_guard263 != 0.0)))) {
            locals.var_rint = 0.0;
        }

        let assign11590_e16809: f64 = if locals.var_rint <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign11590_e16809;

        if (((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard410 != 0.0)) {
            locals.var_rdraingeo = locals.var_rend;
        }

        let assign11610_e16821: f64 = if locals.var_rend <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard411 = assign11610_e16821;

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard410 == 0.0)) && (locals.var_guard411 != 0.0)) {
            locals.var_rdraingeo = locals.var_rint;
        }

        if ((((locals.var_guard245 == 0.0) && (locals.var_guard246 != 0.0)) && (locals.var_guard410 == 0.0)) && (locals.var_guard411 == 0.0)) {
            let assign11630_e16846: f64 = (locals.var_rint * locals.var_rend);
            let assign11630_e16849: f64 = (locals.var_rint + locals.var_rend);
            let assign11630_e16850: f64 = (assign11630_e16846 / assign11630_e16849);
            locals.var_rdraingeo = assign11630_e16850;
        }

        if ((locals.var_guard245 == 0.0) && (locals.var_guard246 == 0.0)) {
            locals.var_rdraingeo = 0.0;
        }

        let assign11660_e16866: f64 = if p.p42 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard413 = assign11660_e16866;

        let assign11670_e16869: f64 = if locals.var_rsourcegeo < p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign11670_e16869;

        if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
            locals.var_rsourcegeo = 0.0;
        }

        let assign11690_e16878: f64 = if locals.var_rdraingeo < p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard415 = assign11690_e16878;

        if ((locals.var_guard413 != 0.0) && (locals.var_guard415 != 0.0)) {
            locals.var_rdraingeo = 0.0;
        }

        let assign11710_e16887: f64 = if locals.var_rsourcegeo <= p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard416 = assign11710_e16887;

        if ((locals.var_guard413 == 0.0) && (locals.var_guard416 != 0.0)) {
            locals.var_rsourcegeo = p.p1093;
        }

        let assign11730_e16897: f64 = if locals.var_rdraingeo <= p.p1093 { 1.0 } else { 0.0 };
        locals.var_guard417 = assign11730_e16897;

        if ((locals.var_guard413 == 0.0) && (locals.var_guard417 != 0.0)) {
            locals.var_rdraingeo = p.p1093;
        }

        let assign11750_e16907: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard418 = assign11750_e16907;

        let assign11760_e16910: f64 = if locals.var_rswmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard419 = assign11760_e16910;

        if ((locals.var_guard418 != 0.0) && (locals.var_guard419 != 0.0)) {
            locals.var_rswmin_i = 0.0;
        }

        let assign11780_e16919: f64 = if locals.var_rdwmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard420 = assign11780_e16919;

        if ((locals.var_guard418 != 0.0) && (locals.var_guard420 != 0.0)) {
            locals.var_rdwmin_i = 0.0;
        }

        let assign11800_e16928: f64 = if locals.var_rsw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard421 = assign11800_e16928;

        if ((locals.var_guard418 != 0.0) && (locals.var_guard421 != 0.0)) {
            locals.var_rsw_i = 0.0;
        }

        let assign11820_e16937: f64 = if locals.var_rdw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard422 = assign11820_e16937;

        if ((locals.var_guard418 != 0.0) && (locals.var_guard422 != 0.0)) {
            locals.var_rdw_i = 0.0;
        }

        let assign11840_e16946: f64 = if locals.var_rdswmin_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard423 = assign11840_e16946;

        if ((locals.var_guard418 == 0.0) && (locals.var_guard423 != 0.0)) {
            locals.var_rdswmin_i = 0.0;
        }

        let assign11860_e16956: f64 = if locals.var_rdsw_i <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard424 = assign11860_e16956;

        if ((locals.var_guard418 == 0.0) && (locals.var_guard424 != 0.0)) {
            locals.var_rdsw_i = 0.0;
        }

        let assign12580_e17615: f64 = if p.p1097 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard443 = assign12580_e17615;

        if (locals.var_guard443 != 0.0) {
            let assign12620_e17639: f64 = (1.0 - p.p1128);
            locals.var_oneminusxpart = assign12620_e17639;
        }

        if (locals.var_guard443 == 0.0) {
            locals.var_oneminusxpart = 1.0;
        }

        let assign12640_e17651: f64 = (locals.var_weffcj / 3.0);
        let assign12640_e17653: f64 = (assign12640_e17651 / p.p32);
        let assign12640_e17654: f64 = (p.p31 + assign12640_e17653);
        let assign12640_e17655: f64 = (p.p700 * assign12640_e17654);
        let assign12640_e17658: f64 = (p.p32 * p.p2);
        let assign12640_e17661: f64 = (locals.var_lnew - p.p699);
        let assign12640_e17662: f64 = (assign12640_e17658 * assign12640_e17661);
        let assign12640_e17663: f64 = (assign12640_e17655 / assign12640_e17662);
        locals.var_grgeltd = assign12640_e17663;

        let assign12650_e17666: f64 = if locals.var_grgeltd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard445 = assign12650_e17666;

        if (locals.var_guard445 != 0.0) {
            let assign12660_e17670: f64 = (1.0 / locals.var_grgeltd);
            locals.var_grgeltd = assign12660_e17670;
        }

        if (locals.var_guard445 == 0.0) {
            locals.var_grgeltd = 1000.0;
        }

        let assign12690_e17683: f64 = (p.p77 * p.p77);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign12690_e17683, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign12700_e17686: f64 = (p.p77 * locals.var_poxedge_i);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign12700_e17686, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign12710_e17689: f64 = (locals.var_t1 * locals.var_t1);
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, ) = (assign12710_e17689, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)), );

        let assign12720_e17693: f64 = (p.p555 / p.p77);
        let assign12720_e17695: f64 = (assign12720_e17693).max(1e-38);
        let assign12720_e17696: f64 = (assign12720_e17695).ln();
        let assign12720_e17697: f64 = (locals.var_ntox_i * assign12720_e17696);
        let assign12720_e17698: f64 = { let limited_exp_arg = assign12720_e17697; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign12720_e17700: f64 = (assign12720_e17698 / locals.var_t0);
        (locals.var_toxratio, locals.var_toxratio_dn0, locals.var_toxratio_dn2, locals.var_toxratio_dn3, locals.var_toxratio_dn4, locals.var_toxratio_dn5, locals.var_toxratio_dn6, locals.var_toxratio_dn7, locals.var_toxratio_dn8, locals.var_toxratio_dn9, locals.var_toxratio_dn10, locals.var_toxratio_dn11, locals.var_toxratio_dn12, locals.var_toxratio_dn13, locals.var_toxratio_dn14, ) = (assign12720_e17700, (-((assign12720_e17698 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((assign12720_e17698 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((assign12720_e17698 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (-((assign12720_e17698 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((assign12720_e17698 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((assign12720_e17698 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((assign12720_e17698 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((assign12720_e17698 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((assign12720_e17698 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((assign12720_e17698 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((assign12720_e17698 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((assign12720_e17698 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((assign12720_e17698 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))), (-((assign12720_e17698 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))), );

        let assign12730_e17704: f64 = (p.p555 / locals.var_t1);
        let assign12730_e17706: f64 = (assign12730_e17704).max(1e-38);
        let assign12730_e17707: f64 = (assign12730_e17706).ln();
        let assign12730_e17708: f64 = (locals.var_ntox_i * assign12730_e17707);
        let assign12730_e17709: f64 = { let limited_exp_arg = assign12730_e17708; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign12730_e17711: f64 = (assign12730_e17709 / locals.var_t2);
        (locals.var_toxratioedge, locals.var_toxratioedge_dn0, locals.var_toxratioedge_dn2, locals.var_toxratioedge_dn3, locals.var_toxratioedge_dn4, locals.var_toxratioedge_dn5, locals.var_toxratioedge_dn6, locals.var_toxratioedge_dn7, locals.var_toxratioedge_dn8, locals.var_toxratioedge_dn9, locals.var_toxratioedge_dn10, locals.var_toxratioedge_dn11, locals.var_toxratioedge_dn12, locals.var_toxratioedge_dn13, locals.var_toxratioedge_dn14, ) = (assign12730_e17711, (((({ let limited_exp_arg = assign12730_e17708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign12730_e17704 >= 1e-38 { (-((p.p555 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign12730_e17706))) * locals.var_t2) - (assign12730_e17709 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((({ let limited_exp_arg = assign12730_e17708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign12730_e17704 >= 1e-38 { (-((p.p555 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign12730_e17706))) * locals.var_t2) - (assign12730_e17709 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((({ let limited_exp_arg = assign12730_e17708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign12730_e17704 >= 1e-38 { (-((p.p555 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign12730_e17706))) * locals.var_t2) - (assign12730_e17709 * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2)), (((({ let limited_exp_arg = assign12730_e17708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign12730_e17704 >= 1e-38 { (-((p.p555 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign12730_e17706))) * locals.var_t2) - (assign12730_e17709 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((({ let limited_exp_arg = assign12730_e17708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign12730_e17704 >= 1e-38 { (-((p.p555 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign12730_e17706))) * locals.var_t2) - (assign12730_e17709 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((({ let limited_exp_arg = assign12730_e17708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign12730_e17704 >= 1e-38 { (-((p.p555 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign12730_e17706))) * locals.var_t2) - (assign12730_e17709 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((({ let limited_exp_arg = assign12730_e17708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign12730_e17704 >= 1e-38 { (-((p.p555 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign12730_e17706))) * locals.var_t2) - (assign12730_e17709 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((({ let limited_exp_arg = assign12730_e17708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign12730_e17704 >= 1e-38 { (-((p.p555 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign12730_e17706))) * locals.var_t2) - (assign12730_e17709 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((({ let limited_exp_arg = assign12730_e17708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign12730_e17704 >= 1e-38 { (-((p.p555 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign12730_e17706))) * locals.var_t2) - (assign12730_e17709 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((({ let limited_exp_arg = assign12730_e17708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign12730_e17704 >= 1e-38 { (-((p.p555 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign12730_e17706))) * locals.var_t2) - (assign12730_e17709 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((({ let limited_exp_arg = assign12730_e17708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign12730_e17704 >= 1e-38 { (-((p.p555 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign12730_e17706))) * locals.var_t2) - (assign12730_e17709 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((({ let limited_exp_arg = assign12730_e17708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign12730_e17704 >= 1e-38 { (-((p.p555 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign12730_e17706))) * locals.var_t2) - (assign12730_e17709 * locals.var_t2_dn12)) / (locals.var_t2 * locals.var_t2)), (((({ let limited_exp_arg = assign12730_e17708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign12730_e17704 >= 1e-38 { (-((p.p555 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign12730_e17706))) * locals.var_t2) - (assign12730_e17709 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)), (((({ let limited_exp_arg = assign12730_e17708; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_ntox_i * (if assign12730_e17704 >= 1e-38 { (-((p.p555 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))) } else { 0.0 } / assign12730_e17706))) * locals.var_t2) - (assign12730_e17709 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)), );

        let (assign12740_e17717,) = {
    if (p.p39 == 1.0) {
        (4.97232e-7,)
    } else {
        (3.42537e-7,)
    }
};
        (locals.var_aechvb, locals.var_aechvb_dn0, locals.var_aechvb_dn2, locals.var_aechvb_dn3, locals.var_aechvb_dn4, locals.var_aechvb_dn5, locals.var_aechvb_dn6, locals.var_aechvb_dn7, locals.var_aechvb_dn8, locals.var_aechvb_dn9, locals.var_aechvb_dn10, locals.var_aechvb_dn11, locals.var_aechvb_dn12, locals.var_aechvb_dn13, locals.var_aechvb_dn14, ) = (assign12740_e17717, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let (assign12750_e17723,) = {
    if (p.p39 == 1.0) {
        (745669000000.0,)
    } else {
        (1166450000000.0,)
    }
};
        locals.var_bechvb = assign12750_e17723;

        let assign12760_e17726: f64 = (locals.var_aechvb * locals.var_weff);
        let assign12760_e17728: f64 = (assign12760_e17726 * locals.var_toxratioedge);
        (locals.var_aechvbedge, locals.var_aechvbedge_dn0, locals.var_aechvbedge_dn2, locals.var_aechvbedge_dn3, locals.var_aechvbedge_dn4, locals.var_aechvbedge_dn5, locals.var_aechvbedge_dn6, locals.var_aechvbedge_dn7, locals.var_aechvbedge_dn8, locals.var_aechvbedge_dn9, locals.var_aechvbedge_dn10, locals.var_aechvbedge_dn11, locals.var_aechvbedge_dn12, locals.var_aechvbedge_dn13, locals.var_aechvbedge_dn14, ) = (assign12760_e17728, (((locals.var_aechvb_dn0 * locals.var_weff) * locals.var_toxratioedge) + (assign12760_e17726 * locals.var_toxratioedge_dn0)), (((locals.var_aechvb_dn2 * locals.var_weff) * locals.var_toxratioedge) + (assign12760_e17726 * locals.var_toxratioedge_dn2)), (((locals.var_aechvb_dn3 * locals.var_weff) * locals.var_toxratioedge) + (assign12760_e17726 * locals.var_toxratioedge_dn3)), (((locals.var_aechvb_dn4 * locals.var_weff) * locals.var_toxratioedge) + (assign12760_e17726 * locals.var_toxratioedge_dn4)), (((locals.var_aechvb_dn5 * locals.var_weff) * locals.var_toxratioedge) + (assign12760_e17726 * locals.var_toxratioedge_dn5)), (((locals.var_aechvb_dn6 * locals.var_weff) * locals.var_toxratioedge) + (assign12760_e17726 * locals.var_toxratioedge_dn6)), (((locals.var_aechvb_dn7 * locals.var_weff) * locals.var_toxratioedge) + (assign12760_e17726 * locals.var_toxratioedge_dn7)), (((locals.var_aechvb_dn8 * locals.var_weff) * locals.var_toxratioedge) + (assign12760_e17726 * locals.var_toxratioedge_dn8)), (((locals.var_aechvb_dn9 * locals.var_weff) * locals.var_toxratioedge) + (assign12760_e17726 * locals.var_toxratioedge_dn9)), (((locals.var_aechvb_dn10 * locals.var_weff) * locals.var_toxratioedge) + (assign12760_e17726 * locals.var_toxratioedge_dn10)), (((locals.var_aechvb_dn11 * locals.var_weff) * locals.var_toxratioedge) + (assign12760_e17726 * locals.var_toxratioedge_dn11)), (((locals.var_aechvb_dn12 * locals.var_weff) * locals.var_toxratioedge) + (assign12760_e17726 * locals.var_toxratioedge_dn12)), (((locals.var_aechvb_dn13 * locals.var_weff) * locals.var_toxratioedge) + (assign12760_e17726 * locals.var_toxratioedge_dn13)), (((locals.var_aechvb_dn14 * locals.var_weff) * locals.var_toxratioedge) + (assign12760_e17726 * locals.var_toxratioedge_dn14)), );

        let assign12770_e17730: f64 = (-locals.var_bechvb);
        let assign12770_e17732: f64 = (assign12770_e17730 * p.p77);
        let assign12770_e17734: f64 = (assign12770_e17732 * locals.var_poxedge_i);
        locals.var_bechvbedge = assign12770_e17734;

        let assign12780_e17738: f64 = (locals.var_weff * locals.var_leff);
        let assign12780_e17740: f64 = (assign12780_e17738 * locals.var_toxratio);
        let assign12780_e17741: f64 = (locals.var_aechvb * assign12780_e17740);
        (locals.var_aechvb, locals.var_aechvb_dn0, locals.var_aechvb_dn2, locals.var_aechvb_dn3, locals.var_aechvb_dn4, locals.var_aechvb_dn5, locals.var_aechvb_dn6, locals.var_aechvb_dn7, locals.var_aechvb_dn8, locals.var_aechvb_dn9, locals.var_aechvb_dn10, locals.var_aechvb_dn11, locals.var_aechvb_dn12, locals.var_aechvb_dn13, locals.var_aechvb_dn14, ) = (assign12780_e17741, ((locals.var_aechvb_dn0 * assign12780_e17740) + (locals.var_aechvb * (assign12780_e17738 * locals.var_toxratio_dn0))), ((locals.var_aechvb_dn2 * assign12780_e17740) + (locals.var_aechvb * (assign12780_e17738 * locals.var_toxratio_dn2))), ((locals.var_aechvb_dn3 * assign12780_e17740) + (locals.var_aechvb * (assign12780_e17738 * locals.var_toxratio_dn3))), ((locals.var_aechvb_dn4 * assign12780_e17740) + (locals.var_aechvb * (assign12780_e17738 * locals.var_toxratio_dn4))), ((locals.var_aechvb_dn5 * assign12780_e17740) + (locals.var_aechvb * (assign12780_e17738 * locals.var_toxratio_dn5))), ((locals.var_aechvb_dn6 * assign12780_e17740) + (locals.var_aechvb * (assign12780_e17738 * locals.var_toxratio_dn6))), ((locals.var_aechvb_dn7 * assign12780_e17740) + (locals.var_aechvb * (assign12780_e17738 * locals.var_toxratio_dn7))), ((locals.var_aechvb_dn8 * assign12780_e17740) + (locals.var_aechvb * (assign12780_e17738 * locals.var_toxratio_dn8))), ((locals.var_aechvb_dn9 * assign12780_e17740) + (locals.var_aechvb * (assign12780_e17738 * locals.var_toxratio_dn9))), ((locals.var_aechvb_dn10 * assign12780_e17740) + (locals.var_aechvb * (assign12780_e17738 * locals.var_toxratio_dn10))), ((locals.var_aechvb_dn11 * assign12780_e17740) + (locals.var_aechvb * (assign12780_e17738 * locals.var_toxratio_dn11))), ((locals.var_aechvb_dn12 * assign12780_e17740) + (locals.var_aechvb * (assign12780_e17738 * locals.var_toxratio_dn12))), ((locals.var_aechvb_dn13 * assign12780_e17740) + (locals.var_aechvb * (assign12780_e17738 * locals.var_toxratio_dn13))), ((locals.var_aechvb_dn14 * assign12780_e17740) + (locals.var_aechvb * (assign12780_e17738 * locals.var_toxratio_dn14))), );

        let assign12790_e17743: f64 = (-locals.var_bechvb);
        let assign12790_e17745: f64 = (assign12790_e17743 * p.p77);
        locals.var_bechvb = assign12790_e17745;

        let assign12800_e17748: f64 = (p.p911 + locals.var_weff);
        locals.var_weff_sh = assign12800_e17748;

        let assign12810_e17759: f64 = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard447 = assign12810_e17759;

        if (locals.var_guard447 != 0.0) {
            let assign12820_e17763: f64 = (locals.var_weff_sh * p.p2);
            let assign12820_e17765: f64 = (assign12820_e17763 / p.p909);
            locals.var_gth = assign12820_e17765;
        }

        if (locals.var_guard447 != 0.0) {
            let assign12830_e17771: f64 = (p.p910 * locals.var_weff_sh);
            let assign12830_e17773: f64 = (assign12830_e17771 * p.p2);
            locals.var_cth = assign12830_e17773;
        }

        if (locals.var_guard447 == 0.0) {
            locals.var_gth = 1.0;
            locals.var_cth = 0.0;
        }

        let assign12860_e17788: f64 = (-273.15);
        let assign12860_e17789: f64 = if p.p820 <= assign12860_e17788 { 1.0 } else { 0.0 };
        locals.var_guard448 = assign12860_e17789;

        if (locals.var_guard448 != 0.0) {
            let assign12870_e17793: f64 = (300.15 - 273.15);
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign12870_e17793, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard448 != 0.0) {
            locals.var_tnom = 300.15;
        }

        if (locals.var_guard448 == 0.0) {
            let assign12890_e17804: f64 = (p.p820 + 273.15);
            locals.var_tnom = assign12890_e17804;
        }

        let assign12900_e17807: f64 = ctx_temp;
        let assign12900_e17809: f64 = (assign12900_e17807 + p.p33);
        (locals.var_devtemp, locals.var_devtemp_dn4, ) = (assign12900_e17809, 0.0, );

        let assign12910_e17820: f64 = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard449 = assign12910_e17820;

        if (locals.var_guard449 != 0.0) {
            (locals.var_deltemp1, locals.var_deltemp1_dn4, ) = ((nv4 - 0.0), 1.0, );
        }

        if (locals.var_guard449 == 0.0) {
            (locals.var_deltemp1, locals.var_deltemp1_dn4, ) = (0.0, 0.0, );
        }

        let assign12940_e17832: f64 = (locals.var_deltemp1 + locals.var_devtemp);
        (locals.var_devtemp, locals.var_devtemp_dn4, ) = (assign12940_e17832, (locals.var_deltemp1_dn4 + locals.var_devtemp_dn4), );

        let assign12980_e17840: f64 = (8.617087e-5 * locals.var_devtemp);
        (locals.var_vt, locals.var_vt_dn4, ) = (assign12980_e17840, (8.617087e-5 * locals.var_devtemp_dn4), );

        let assign12990_e17843: f64 = (1.0 / locals.var_vt);
        (locals.var_inv_vt, locals.var_inv_vt_dn4, ) = (assign12990_e17843, (-(locals.var_vt_dn4 / (locals.var_vt * locals.var_vt))), );

        let assign13000_e17846: f64 = (locals.var_devtemp / locals.var_tnom);
        (locals.var_tratio, locals.var_tratio_dn4, ) = (assign13000_e17846, (locals.var_devtemp_dn4 / locals.var_tnom), );

        let assign13010_e17849: f64 = (locals.var_devtemp - locals.var_tnom);
        (locals.var_deltemp, locals.var_deltemp_dn4, ) = (assign13010_e17849, locals.var_devtemp_dn4, );

        let assign13020_e17852: f64 = (8.617087e-5 * locals.var_devtemp);
        (locals.var_vtm, locals.var_vtm_dn4, ) = (assign13020_e17852, (8.617087e-5 * locals.var_devtemp_dn4), );

        let assign13030_e17855: f64 = (8.617087e-5 * locals.var_tnom);
        locals.var_vtm0 = assign13030_e17855;

        let assign13040_e17859: f64 = (p.p821 * locals.var_devtemp);
        let assign13040_e17861: f64 = (assign13040_e17859 * locals.var_devtemp);
        let assign13040_e17864: f64 = (locals.var_devtemp + p.p822);
        let assign13040_e17865: f64 = (assign13040_e17861 / assign13040_e17864);
        let assign13040_e17866: f64 = (p.p109 - assign13040_e17865);
        (locals.var_eg, locals.var_eg_dn4, ) = (assign13040_e17866, (-((((((p.p821 * locals.var_devtemp_dn4) * locals.var_devtemp) + (assign13040_e17859 * locals.var_devtemp_dn4)) * assign13040_e17864) - (assign13040_e17861 * locals.var_devtemp_dn4)) / (assign13040_e17864 * assign13040_e17864))), );

        let assign13050_e17870: f64 = (p.p821 * locals.var_tnom);
        let assign13050_e17872: f64 = (assign13050_e17870 * locals.var_tnom);
        let assign13050_e17875: f64 = (locals.var_tnom + p.p822);
        let assign13050_e17876: f64 = (assign13050_e17872 / assign13050_e17875);
        let assign13050_e17877: f64 = (p.p109 - assign13050_e17876);
        locals.var_eg0 = assign13050_e17877;

        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_tnom;
        let assign13060_e17880: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign13060_e17883: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign13060_e17884: f64 = (assign13060_e17883).sqrt();
        let assign13060_e17885: f64 = (assign13060_e17880 * assign13060_e17884);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign13060_e17885, 0.0, 0.0, 0.0, (((locals.var_devtemp_dn4 / locals.var_tnom) * assign13060_e17884) + (assign13060_e17880 * ((locals.var_devtemp_dn4 / locals.var_tnom) / (2.0 * assign13060_e17884)))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign13070_e17888: f64 = (p.p108 * locals.var_t1);
        let assign13070_e17892: f64 = (2.0 * locals.var_vtm0);
        let assign13070_e17893: f64 = (locals.var_eg / assign13070_e17892);
        let assign13070_e17897: f64 = (2.0 * locals.var_vtm);
        let assign13070_e17898: f64 = (locals.var_eg / assign13070_e17897);
        let assign13070_e17899: f64 = (assign13070_e17893 - assign13070_e17898);
        let assign13070_e17900: f64 = { let limited_exp_arg = assign13070_e17899; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13070_e17901: f64 = (assign13070_e17888 * assign13070_e17900);
        (locals.var_ni, locals.var_ni_dn0, locals.var_ni_dn2, locals.var_ni_dn3, locals.var_ni_dn4, locals.var_ni_dn5, locals.var_ni_dn6, locals.var_ni_dn7, locals.var_ni_dn8, locals.var_ni_dn9, locals.var_ni_dn10, locals.var_ni_dn11, locals.var_ni_dn12, locals.var_ni_dn13, locals.var_ni_dn14, ) = (assign13070_e17901, ((p.p108 * locals.var_t1_dn0) * assign13070_e17900), ((p.p108 * locals.var_t1_dn2) * assign13070_e17900), ((p.p108 * locals.var_t1_dn3) * assign13070_e17900), (((p.p108 * locals.var_t1_dn4) * assign13070_e17900) + (assign13070_e17888 * ({ let limited_exp_arg = assign13070_e17899; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_eg_dn4 / assign13070_e17892) - (((locals.var_eg_dn4 * assign13070_e17897) - (locals.var_eg * (2.0 * locals.var_vtm_dn4))) / (assign13070_e17897 * assign13070_e17897)))))), ((p.p108 * locals.var_t1_dn5) * assign13070_e17900), ((p.p108 * locals.var_t1_dn6) * assign13070_e17900), ((p.p108 * locals.var_t1_dn7) * assign13070_e17900), ((p.p108 * locals.var_t1_dn8) * assign13070_e17900), ((p.p108 * locals.var_t1_dn9) * assign13070_e17900), ((p.p108 * locals.var_t1_dn10) * assign13070_e17900), ((p.p108 * locals.var_t1_dn11) * assign13070_e17900), ((p.p108 * locals.var_t1_dn12) * assign13070_e17900), ((p.p108 * locals.var_t1_dn13) * assign13070_e17900), ((p.p108 * locals.var_t1_dn14) * assign13070_e17900), );

        let assign13080_e17912: f64 = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard450 = assign13080_e17912;

        if (locals.var_guard450 != 0.0) {
            let assign13090_e17916: f64 = (locals.var_ndep_i / locals.var_ni);
            let assign13090_e17918: f64 = (assign13090_e17916).max(1e-38);
            let assign13090_e17919: f64 = (assign13090_e17918).ln();
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign13090_e17919, (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn0 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn0)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn2 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn2)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn3 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn4 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn5 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn6 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn7 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn8 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn9 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn10 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn11 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn12 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn12)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn13 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn13)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), (if assign13090_e17916 >= 1e-38 { (((locals.var_ndep_i_dn14 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn14)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13090_e17918), );
        }

        if (locals.var_guard450 != 0.0) {
            let assign13100_e17925: f64 = (locals.var_t0 * locals.var_t0);
            let assign13100_e17927: f64 = (assign13100_e17925 + 1e-6);
            let assign13100_e17928: f64 = (assign13100_e17927).sqrt();
            (locals.var_phib, locals.var_phib_dn0, locals.var_phib_dn2, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11, locals.var_phib_dn12, locals.var_phib_dn13, locals.var_phib_dn14, ) = (assign13100_e17928, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign13100_e17928)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign13100_e17928)), );
        }

        if (locals.var_guard450 == 0.0) {
            let assign13110_e17935: f64 = (locals.var_ndep_i / locals.var_ni);
            let assign13110_e17937: f64 = (assign13110_e17935).max(1e-38);
            let assign13110_e17938: f64 = (assign13110_e17937).ln();
            (locals.var_phib, locals.var_phib_dn0, locals.var_phib_dn2, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11, locals.var_phib_dn12, locals.var_phib_dn13, locals.var_phib_dn14, ) = (assign13110_e17938, (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn0 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn0)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn2 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn2)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn3 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn4 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn5 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn6 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn7 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn8 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn9 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn10 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn11 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn12 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn12)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn13 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn13)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), (if assign13110_e17935 >= 1e-38 { (((locals.var_ndep_i_dn14 * locals.var_ni) - (locals.var_ndep_i * locals.var_ni_dn14)) / (locals.var_ni * locals.var_ni)) } else { 0.0 } / assign13110_e17937), );
        }

        let assign13120_e17951: f64 = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (locals.var_weff_sh > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard451 = assign13120_e17951;

        if (locals.var_guard451 != 0.0) {
            let assign13130_e17955: f64 = (locals.var_ndepedge_i * locals.var_nsd_i);
            let assign13130_e17958: f64 = (locals.var_ni * locals.var_ni);
            let assign13130_e17959: f64 = (assign13130_e17955 / assign13130_e17958);
            let assign13130_e17961: f64 = (assign13130_e17959).max(1e-38);
            let assign13130_e17962: f64 = (assign13130_e17961).ln();
            (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign13130_e17962, (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn0 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn0))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn2 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn2))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn3 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn3))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn4 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn4))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn5 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn5))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn7 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn7))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn8 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn8))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn9 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn9))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn10 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn10))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn11 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn11))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn12 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn12))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn13 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn13))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), (if assign13130_e17959 >= 1e-38 { (-((assign13130_e17955 * ((locals.var_ni_dn14 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn14))) / (assign13130_e17958 * assign13130_e17958))) } else { 0.0 } / assign13130_e17961), );
        }

        if (locals.var_guard451 != 0.0) {
            let assign13140_e17968: f64 = (locals.var_t0 * locals.var_t0);
            let assign13140_e17970: f64 = (assign13140_e17968 + 1e-6);
            let assign13140_e17971: f64 = (assign13140_e17970).sqrt();
            (locals.var_vbi_edge, locals.var_vbi_edge_dn0, locals.var_vbi_edge_dn2, locals.var_vbi_edge_dn3, locals.var_vbi_edge_dn4, locals.var_vbi_edge_dn5, locals.var_vbi_edge_dn6, locals.var_vbi_edge_dn7, locals.var_vbi_edge_dn8, locals.var_vbi_edge_dn9, locals.var_vbi_edge_dn10, locals.var_vbi_edge_dn11, locals.var_vbi_edge_dn12, locals.var_vbi_edge_dn13, locals.var_vbi_edge_dn14, ) = (assign13140_e17971, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign13140_e17971)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign13140_e17971)), );
        }

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard451 == 0.0) {
            let assign13150_e17978: f64 = (locals.var_ndepedge_i * locals.var_nsd_i);
            let assign13150_e17981: f64 = (locals.var_ni * locals.var_ni);
            let assign13150_e17982: f64 = (assign13150_e17978 / assign13150_e17981);
            let assign13150_e17984: f64 = (assign13150_e17982).max(1e-38);
            let assign13150_e17985: f64 = (assign13150_e17984).ln();
            (locals.var_vbi_edge, locals.var_vbi_edge_dn0, locals.var_vbi_edge_dn2, locals.var_vbi_edge_dn3, locals.var_vbi_edge_dn4, locals.var_vbi_edge_dn5, locals.var_vbi_edge_dn6, locals.var_vbi_edge_dn7, locals.var_vbi_edge_dn8, locals.var_vbi_edge_dn9, locals.var_vbi_edge_dn10, locals.var_vbi_edge_dn11, locals.var_vbi_edge_dn12, locals.var_vbi_edge_dn13, locals.var_vbi_edge_dn14, ) = (assign13150_e17985, (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn0 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn0))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn2 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn2))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn3 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn3))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn4 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn4))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn5 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn5))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn7 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn7))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn8 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn8))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn9 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn9))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn10 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn10))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn11 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn11))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn12 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn12))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn13 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn13))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), (if assign13150_e17982 >= 1e-38 { (-((assign13150_e17978 * ((locals.var_ni_dn14 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn14))) / (assign13150_e17981 * assign13150_e17981))) } else { 0.0 } / assign13150_e17984), );
        }

        let assign13160_e17990: f64 = if locals.var_ngate_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard452 = assign13160_e17990;

        if (locals.var_guard452 != 0.0) {
            let assign13170_e17993: f64 = (-locals.var_devsign);
            let assign13170_e17995: f64 = (assign13170_e17993 * locals.var_vt);
            let assign13170_e17998: f64 = (locals.var_ngate_i / locals.var_nsd_i);
            let assign13170_e18000: f64 = (assign13170_e17998).max(1e-38);
            let assign13170_e18001: f64 = (assign13170_e18000).ln();
            let assign13170_e18002: f64 = (assign13170_e17995 * assign13170_e18001);
            let assign13170_e18004: f64 = (assign13170_e18002 + p.p5);
            (locals.var_vfbsdr, locals.var_vfbsdr_dn4, ) = (assign13170_e18004, ((assign13170_e17993 * locals.var_vt_dn4) * assign13170_e18001), );
        }

        if (locals.var_guard452 == 0.0) {
            (locals.var_vfbsdr, locals.var_vfbsdr_dn4, ) = (0.0, 0.0, );
        }

        let assign13190_e18015: f64 = (locals.var_vt * locals.var_phib);
        let assign13190_e18016: f64 = (0.4 + assign13190_e18015);
        let assign13190_e18018: f64 = (assign13190_e18016 + locals.var_phin_i);
        let assign13190_e18020: f64 = (assign13190_e18018).max(0.4);
        (locals.var_phist, locals.var_phist_dn0, locals.var_phist_dn2, locals.var_phist_dn3, locals.var_phist_dn4, locals.var_phist_dn5, locals.var_phist_dn6, locals.var_phist_dn7, locals.var_phist_dn8, locals.var_phist_dn9, locals.var_phist_dn10, locals.var_phist_dn11, locals.var_phist_dn12, locals.var_phist_dn13, locals.var_phist_dn14, ) = (assign13190_e18020, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn0) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn2) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn3) } else { 0.0 }, if assign13190_e18018 >= 0.4 { ((locals.var_vt_dn4 * locals.var_phib) + (locals.var_vt * locals.var_phib_dn4)) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn5) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn6) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn7) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn8) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn9) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn10) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn11) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn12) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn13) } else { 0.0 }, if assign13190_e18018 >= 0.4 { (locals.var_vt * locals.var_phib_dn14) } else { 0.0 }, );

        let assign13200_e18022: f64 = (locals.var_phist).sqrt();
        (locals.var_sqrtphist, locals.var_sqrtphist_dn0, locals.var_sqrtphist_dn2, locals.var_sqrtphist_dn3, locals.var_sqrtphist_dn4, locals.var_sqrtphist_dn5, locals.var_sqrtphist_dn6, locals.var_sqrtphist_dn7, locals.var_sqrtphist_dn8, locals.var_sqrtphist_dn9, locals.var_sqrtphist_dn10, locals.var_sqrtphist_dn11, locals.var_sqrtphist_dn12, locals.var_sqrtphist_dn13, locals.var_sqrtphist_dn14, ) = (assign13200_e18022, (locals.var_phist_dn0 / (2.0 * assign13200_e18022)), (locals.var_phist_dn2 / (2.0 * assign13200_e18022)), (locals.var_phist_dn3 / (2.0 * assign13200_e18022)), (locals.var_phist_dn4 / (2.0 * assign13200_e18022)), (locals.var_phist_dn5 / (2.0 * assign13200_e18022)), (locals.var_phist_dn6 / (2.0 * assign13200_e18022)), (locals.var_phist_dn7 / (2.0 * assign13200_e18022)), (locals.var_phist_dn8 / (2.0 * assign13200_e18022)), (locals.var_phist_dn9 / (2.0 * assign13200_e18022)), (locals.var_phist_dn10 / (2.0 * assign13200_e18022)), (locals.var_phist_dn11 / (2.0 * assign13200_e18022)), (locals.var_phist_dn12 / (2.0 * assign13200_e18022)), (locals.var_phist_dn13 / (2.0 * assign13200_e18022)), (locals.var_phist_dn14 / (2.0 * assign13200_e18022)), );

        let assign13210_e18025: f64 = (2.0 * locals.var_epssi);
        let assign13210_e18028: f64 = (1.60219e-19 * locals.var_ndep_i);
        let assign13210_e18029: f64 = (assign13210_e18025 / assign13210_e18028);
        let assign13210_e18030: f64 = (assign13210_e18029).sqrt();
        (locals.var_t1dep, locals.var_t1dep_dn0, locals.var_t1dep_dn2, locals.var_t1dep_dn3, locals.var_t1dep_dn4, locals.var_t1dep_dn5, locals.var_t1dep_dn6, locals.var_t1dep_dn7, locals.var_t1dep_dn8, locals.var_t1dep_dn9, locals.var_t1dep_dn10, locals.var_t1dep_dn11, locals.var_t1dep_dn12, locals.var_t1dep_dn13, locals.var_t1dep_dn14, ) = (assign13210_e18030, ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn0)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn2)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn3)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn4)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn5)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn6)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn7)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn8)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn9)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn10)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn11)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn12)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn13)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), ((-((assign13210_e18025 * (1.60219e-19 * locals.var_ndep_i_dn14)) / (assign13210_e18028 * assign13210_e18028))) / (2.0 * assign13210_e18030)), );

        let assign13220_e18033: f64 = (locals.var_epssi / locals.var_epsox);
        let assign13220_e18035: f64 = (assign13220_e18033 * p.p77);
        let assign13220_e18037: f64 = (assign13220_e18035 * locals.var_xj_i);
        let assign13220_e18038: f64 = (assign13220_e18037).sqrt();
        locals.var_litl = assign13220_e18038;

        let assign13230_e18044: f64 = (locals.var_tratio - 1.0);
        let assign13230_e18045: f64 = (p.p823 * assign13230_e18044);
        let assign13230_e18046: f64 = (1.0 + assign13230_e18045);
        let assign13230_e18048: f64 = (-10000.0);
        let assign13230_e18050: f64 = (assign13230_e18048 * 0.001);
        let (assign13230_e18111, assign13230_e18111_d_n4,) = {
    if (!(assign13230_e18046 < assign13230_e18050)) {
        let assign13230_e18058: f64 = (locals.var_tratio - 1.0);
        let assign13230_e18059: f64 = (p.p823 * assign13230_e18058);
        let assign13230_e18060: f64 = (1.0 + assign13230_e18059);
        let assign13230_e18065: f64 = (locals.var_tratio - 1.0);
        let assign13230_e18066: f64 = (p.p823 * assign13230_e18065);
        let assign13230_e18067: f64 = (1.0 + assign13230_e18066);
        let assign13230_e18072: f64 = (locals.var_tratio - 1.0);
        let assign13230_e18073: f64 = (p.p823 * assign13230_e18072);
        let assign13230_e18074: f64 = (1.0 + assign13230_e18073);
        let assign13230_e18075: f64 = (assign13230_e18067 * assign13230_e18074);
        let assign13230_e18078: f64 = (4.0 * 0.001);
        let assign13230_e18080: f64 = (assign13230_e18078 * 0.001);
        let assign13230_e18081: f64 = (assign13230_e18075 + assign13230_e18080);
        let assign13230_e18082: f64 = (assign13230_e18081).sqrt();
        let assign13230_e18083: f64 = (assign13230_e18060 + assign13230_e18082);
        let assign13230_e18084: f64 = (0.5 * assign13230_e18083);
        (assign13230_e18084, (0.5 * ((p.p823 * locals.var_tratio_dn4) + ((((p.p823 * locals.var_tratio_dn4) * assign13230_e18074) + (assign13230_e18067 * (p.p823 * locals.var_tratio_dn4))) / (2.0 * assign13230_e18082)))),)
    } else {
        let assign13230_e18089: f64 = (locals.var_tratio - 1.0);
        let assign13230_e18090: f64 = (p.p823 * assign13230_e18089);
        let assign13230_e18091: f64 = (1.0 + assign13230_e18090);
        let assign13230_e18093: f64 = (-10000.0);
        let assign13230_e18095: f64 = (assign13230_e18093 * 0.001);
        let (assign13230_e18110, assign13230_e18110_d_n4,) = {
            if (assign13230_e18091 < assign13230_e18095) {
                let assign13230_e18098: f64 = (-0.001);
                let assign13230_e18100: f64 = (assign13230_e18098 * 0.001);
                let assign13230_e18105: f64 = (locals.var_tratio - 1.0);
                let assign13230_e18106: f64 = (p.p823 * assign13230_e18105);
                let assign13230_e18107: f64 = (1.0 + assign13230_e18106);
                let assign13230_e18108: f64 = (assign13230_e18100 / assign13230_e18107);
                (assign13230_e18108, (-((assign13230_e18100 * (p.p823 * locals.var_tratio_dn4)) / (assign13230_e18107 * assign13230_e18107))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13230_e18110, assign13230_e18110_d_n4,)
    }
};
        let assign13230_e18112: f64 = (locals.var_nfactor_i * assign13230_e18111);
        (locals.var_nfactor_t, locals.var_nfactor_t_dn0, locals.var_nfactor_t_dn2, locals.var_nfactor_t_dn3, locals.var_nfactor_t_dn4, locals.var_nfactor_t_dn5, locals.var_nfactor_t_dn6, locals.var_nfactor_t_dn7, locals.var_nfactor_t_dn8, locals.var_nfactor_t_dn9, locals.var_nfactor_t_dn10, locals.var_nfactor_t_dn11, locals.var_nfactor_t_dn12, locals.var_nfactor_t_dn13, locals.var_nfactor_t_dn14, ) = (assign13230_e18112, (locals.var_nfactor_i_dn0 * assign13230_e18111), (locals.var_nfactor_i_dn2 * assign13230_e18111), (locals.var_nfactor_i_dn3 * assign13230_e18111), ((locals.var_nfactor_i_dn4 * assign13230_e18111) + (locals.var_nfactor_i * assign13230_e18111_d_n4)), (locals.var_nfactor_i_dn5 * assign13230_e18111), (locals.var_nfactor_i_dn6 * assign13230_e18111), (locals.var_nfactor_i_dn7 * assign13230_e18111), (locals.var_nfactor_i_dn8 * assign13230_e18111), (locals.var_nfactor_i_dn9 * assign13230_e18111), (locals.var_nfactor_i_dn10 * assign13230_e18111), (locals.var_nfactor_i_dn11 * assign13230_e18111), (locals.var_nfactor_i_dn12 * assign13230_e18111), (locals.var_nfactor_i_dn13 * assign13230_e18111), (locals.var_nfactor_i_dn14 * assign13230_e18111), );

        let assign13240_e18118: f64 = (locals.var_tratio - 1.0);
        let assign13240_e18119: f64 = (p.p851 * assign13240_e18118);
        let assign13240_e18120: f64 = (1.0 + assign13240_e18119);
        let assign13240_e18121: f64 = (locals.var_eta0_i * assign13240_e18120);
        (locals.var_eta0_t, locals.var_eta0_t_dn0, locals.var_eta0_t_dn2, locals.var_eta0_t_dn3, locals.var_eta0_t_dn4, locals.var_eta0_t_dn5, locals.var_eta0_t_dn6, locals.var_eta0_t_dn7, locals.var_eta0_t_dn8, locals.var_eta0_t_dn9, locals.var_eta0_t_dn10, locals.var_eta0_t_dn11, locals.var_eta0_t_dn12, locals.var_eta0_t_dn13, locals.var_eta0_t_dn14, ) = (assign13240_e18121, (locals.var_eta0_i_dn0 * assign13240_e18120), (locals.var_eta0_i_dn2 * assign13240_e18120), (locals.var_eta0_i_dn3 * assign13240_e18120), ((locals.var_eta0_i_dn4 * assign13240_e18120) + (locals.var_eta0_i * (p.p851 * locals.var_tratio_dn4))), (locals.var_eta0_i_dn5 * assign13240_e18120), (locals.var_eta0_i_dn6 * assign13240_e18120), (locals.var_eta0_i_dn7 * assign13240_e18120), (locals.var_eta0_i_dn8 * assign13240_e18120), (locals.var_eta0_i_dn9 * assign13240_e18120), (locals.var_eta0_i_dn10 * assign13240_e18120), (locals.var_eta0_i_dn11 * assign13240_e18120), (locals.var_eta0_i_dn12 * assign13240_e18120), (locals.var_eta0_i_dn13 * assign13240_e18120), (locals.var_eta0_i_dn14 * assign13240_e18120), );

        let assign13250_e18124: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard453 = assign13250_e18124;

        if (locals.var_guard453 != 0.0) {
            let assign13260_e18131: f64 = (locals.var_tratio - 1.0);
            let assign13260_e18132: f64 = (p.p851 * assign13260_e18131);
            let assign13260_e18133: f64 = (1.0 + assign13260_e18132);
            let assign13260_e18134: f64 = (locals.var_eta0r_i * assign13260_e18133);
            (locals.var_eta0r_t, locals.var_eta0r_t_dn0, locals.var_eta0r_t_dn2, locals.var_eta0r_t_dn3, locals.var_eta0r_t_dn4, locals.var_eta0r_t_dn5, locals.var_eta0r_t_dn6, locals.var_eta0r_t_dn7, locals.var_eta0r_t_dn8, locals.var_eta0r_t_dn9, locals.var_eta0r_t_dn10, locals.var_eta0r_t_dn11, locals.var_eta0r_t_dn12, locals.var_eta0r_t_dn13, locals.var_eta0r_t_dn14, ) = (assign13260_e18134, (locals.var_eta0r_i_dn0 * assign13260_e18133), (locals.var_eta0r_i_dn2 * assign13260_e18133), (locals.var_eta0r_i_dn3 * assign13260_e18133), ((locals.var_eta0r_i_dn4 * assign13260_e18133) + (locals.var_eta0r_i * (p.p851 * locals.var_tratio_dn4))), (locals.var_eta0r_i_dn5 * assign13260_e18133), (locals.var_eta0r_i_dn6 * assign13260_e18133), (locals.var_eta0r_i_dn7 * assign13260_e18133), (locals.var_eta0r_i_dn8 * assign13260_e18133), (locals.var_eta0r_i_dn9 * assign13260_e18133), (locals.var_eta0r_i_dn10 * assign13260_e18133), (locals.var_eta0r_i_dn11 * assign13260_e18133), (locals.var_eta0r_i_dn12 * assign13260_e18133), (locals.var_eta0r_i_dn13 * assign13260_e18133), (locals.var_eta0r_i_dn14 * assign13260_e18133), );
        }

        let (assign13270_e18146,) = {
    if (p.p39 != 1.0) {
        let assign13270_e18142: f64 = (0.3333333333333333 * p.p283);
        (assign13270_e18142,)
    } else {
        let assign13270_e18145: f64 = (0.5 * p.p283);
        (assign13270_e18145,)
    }
};
        locals.var_eta_mu = assign13270_e18146;

        let assign13280_e18150: f64 = (locals.var_tratio).powf(locals.var_ute_i);
        let assign13280_e18151: f64 = (locals.var_u0_i * assign13280_e18150);
        (locals.var_u0_t, locals.var_u0_t_dn0, locals.var_u0_t_dn2, locals.var_u0_t_dn3, locals.var_u0_t_dn4, locals.var_u0_t_dn5, locals.var_u0_t_dn6, locals.var_u0_t_dn7, locals.var_u0_t_dn8, locals.var_u0_t_dn9, locals.var_u0_t_dn10, locals.var_u0_t_dn11, locals.var_u0_t_dn12, locals.var_u0_t_dn13, locals.var_u0_t_dn14, ) = (assign13280_e18151, 0.0, 0.0, 0.0, (locals.var_u0_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13280_e18150 * (locals.var_ute_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign13290_e18156: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13290_e18157: f64 = (1.0 + assign13290_e18156);
        let assign13290_e18159: f64 = (assign13290_e18157 - 1e-6);
        let assign13290_e18161: f64 = (-10000.0);
        let assign13290_e18163: f64 = (assign13290_e18161 * 0.001);
        let (assign13290_e18224, assign13290_e18224_d_n4,) = {
    if (!(assign13290_e18159 < assign13290_e18163)) {
        let assign13290_e18170: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13290_e18171: f64 = (1.0 + assign13290_e18170);
        let assign13290_e18173: f64 = (assign13290_e18171 - 1e-6);
        let assign13290_e18177: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13290_e18178: f64 = (1.0 + assign13290_e18177);
        let assign13290_e18180: f64 = (assign13290_e18178 - 1e-6);
        let assign13290_e18184: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13290_e18185: f64 = (1.0 + assign13290_e18184);
        let assign13290_e18187: f64 = (assign13290_e18185 - 1e-6);
        let assign13290_e18188: f64 = (assign13290_e18180 * assign13290_e18187);
        let assign13290_e18191: f64 = (4.0 * 0.001);
        let assign13290_e18193: f64 = (assign13290_e18191 * 0.001);
        let assign13290_e18194: f64 = (assign13290_e18188 + assign13290_e18193);
        let assign13290_e18195: f64 = (assign13290_e18194).sqrt();
        let assign13290_e18196: f64 = (assign13290_e18173 + assign13290_e18195);
        let assign13290_e18197: f64 = (0.5 * assign13290_e18196);
        (assign13290_e18197, (0.5 * ((locals.var_ua1_i * locals.var_deltemp_dn4) + ((((locals.var_ua1_i * locals.var_deltemp_dn4) * assign13290_e18187) + (assign13290_e18180 * (locals.var_ua1_i * locals.var_deltemp_dn4))) / (2.0 * assign13290_e18195)))),)
    } else {
        let assign13290_e18201: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13290_e18202: f64 = (1.0 + assign13290_e18201);
        let assign13290_e18204: f64 = (assign13290_e18202 - 1e-6);
        let assign13290_e18206: f64 = (-10000.0);
        let assign13290_e18208: f64 = (assign13290_e18206 * 0.001);
        let (assign13290_e18223, assign13290_e18223_d_n4,) = {
            if (assign13290_e18204 < assign13290_e18208) {
                let assign13290_e18211: f64 = (-0.001);
                let assign13290_e18213: f64 = (assign13290_e18211 * 0.001);
                let assign13290_e18217: f64 = (locals.var_ua1_i * locals.var_deltemp);
                let assign13290_e18218: f64 = (1.0 + assign13290_e18217);
                let assign13290_e18220: f64 = (assign13290_e18218 - 1e-6);
                let assign13290_e18221: f64 = (assign13290_e18213 / assign13290_e18220);
                (assign13290_e18221, (-((assign13290_e18213 * (locals.var_ua1_i * locals.var_deltemp_dn4)) / (assign13290_e18220 * assign13290_e18220))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13290_e18223, assign13290_e18223_d_n4,)
    }
};
        let assign13290_e18225: f64 = (locals.var_ua_i * assign13290_e18224);
        (locals.var_ua_t, locals.var_ua_t_dn0, locals.var_ua_t_dn2, locals.var_ua_t_dn3, locals.var_ua_t_dn4, locals.var_ua_t_dn5, locals.var_ua_t_dn6, locals.var_ua_t_dn7, locals.var_ua_t_dn8, locals.var_ua_t_dn9, locals.var_ua_t_dn10, locals.var_ua_t_dn11, locals.var_ua_t_dn12, locals.var_ua_t_dn13, locals.var_ua_t_dn14, ) = (assign13290_e18225, (locals.var_ua_i_dn0 * assign13290_e18224), (locals.var_ua_i_dn2 * assign13290_e18224), (locals.var_ua_i_dn3 * assign13290_e18224), ((locals.var_ua_i_dn4 * assign13290_e18224) + (locals.var_ua_i * assign13290_e18224_d_n4)), (locals.var_ua_i_dn5 * assign13290_e18224), (locals.var_ua_i_dn6 * assign13290_e18224), (locals.var_ua_i_dn7 * assign13290_e18224), (locals.var_ua_i_dn8 * assign13290_e18224), (locals.var_ua_i_dn9 * assign13290_e18224), (locals.var_ua_i_dn10 * assign13290_e18224), (locals.var_ua_i_dn11 * assign13290_e18224), (locals.var_ua_i_dn12 * assign13290_e18224), (locals.var_ua_i_dn13 * assign13290_e18224), (locals.var_ua_i_dn14 * assign13290_e18224), );

        let assign13300_e18230: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13300_e18231: f64 = (1.0 + assign13300_e18230);
        let assign13300_e18233: f64 = (assign13300_e18231 - 1e-6);
        let assign13300_e18235: f64 = (-10000.0);
        let assign13300_e18237: f64 = (assign13300_e18235 * 0.001);
        let (assign13300_e18298, assign13300_e18298_d_n4,) = {
    if (!(assign13300_e18233 < assign13300_e18237)) {
        let assign13300_e18244: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13300_e18245: f64 = (1.0 + assign13300_e18244);
        let assign13300_e18247: f64 = (assign13300_e18245 - 1e-6);
        let assign13300_e18251: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13300_e18252: f64 = (1.0 + assign13300_e18251);
        let assign13300_e18254: f64 = (assign13300_e18252 - 1e-6);
        let assign13300_e18258: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13300_e18259: f64 = (1.0 + assign13300_e18258);
        let assign13300_e18261: f64 = (assign13300_e18259 - 1e-6);
        let assign13300_e18262: f64 = (assign13300_e18254 * assign13300_e18261);
        let assign13300_e18265: f64 = (4.0 * 0.001);
        let assign13300_e18267: f64 = (assign13300_e18265 * 0.001);
        let assign13300_e18268: f64 = (assign13300_e18262 + assign13300_e18267);
        let assign13300_e18269: f64 = (assign13300_e18268).sqrt();
        let assign13300_e18270: f64 = (assign13300_e18247 + assign13300_e18269);
        let assign13300_e18271: f64 = (0.5 * assign13300_e18270);
        (assign13300_e18271, (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn4) + ((((locals.var_uc1_i * locals.var_deltemp_dn4) * assign13300_e18261) + (assign13300_e18254 * (locals.var_uc1_i * locals.var_deltemp_dn4))) / (2.0 * assign13300_e18269)))),)
    } else {
        let assign13300_e18275: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13300_e18276: f64 = (1.0 + assign13300_e18275);
        let assign13300_e18278: f64 = (assign13300_e18276 - 1e-6);
        let assign13300_e18280: f64 = (-10000.0);
        let assign13300_e18282: f64 = (assign13300_e18280 * 0.001);
        let (assign13300_e18297, assign13300_e18297_d_n4,) = {
            if (assign13300_e18278 < assign13300_e18282) {
                let assign13300_e18285: f64 = (-0.001);
                let assign13300_e18287: f64 = (assign13300_e18285 * 0.001);
                let assign13300_e18291: f64 = (locals.var_uc1_i * locals.var_deltemp);
                let assign13300_e18292: f64 = (1.0 + assign13300_e18291);
                let assign13300_e18294: f64 = (assign13300_e18292 - 1e-6);
                let assign13300_e18295: f64 = (assign13300_e18287 / assign13300_e18294);
                (assign13300_e18295, (-((assign13300_e18287 * (locals.var_uc1_i * locals.var_deltemp_dn4)) / (assign13300_e18294 * assign13300_e18294))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13300_e18297, assign13300_e18297_d_n4,)
    }
};
        let assign13300_e18299: f64 = (locals.var_uc_i * assign13300_e18298);
        (locals.var_uc_t, locals.var_uc_t_dn0, locals.var_uc_t_dn2, locals.var_uc_t_dn3, locals.var_uc_t_dn4, locals.var_uc_t_dn5, locals.var_uc_t_dn6, locals.var_uc_t_dn7, locals.var_uc_t_dn8, locals.var_uc_t_dn9, locals.var_uc_t_dn10, locals.var_uc_t_dn11, locals.var_uc_t_dn12, locals.var_uc_t_dn13, locals.var_uc_t_dn14, ) = (assign13300_e18299, (locals.var_uc_i_dn0 * assign13300_e18298), (locals.var_uc_i_dn2 * assign13300_e18298), (locals.var_uc_i_dn3 * assign13300_e18298), ((locals.var_uc_i_dn4 * assign13300_e18298) + (locals.var_uc_i * assign13300_e18298_d_n4)), (locals.var_uc_i_dn5 * assign13300_e18298), (locals.var_uc_i_dn6 * assign13300_e18298), (locals.var_uc_i_dn7 * assign13300_e18298), (locals.var_uc_i_dn8 * assign13300_e18298), (locals.var_uc_i_dn9 * assign13300_e18298), (locals.var_uc_i_dn10 * assign13300_e18298), (locals.var_uc_i_dn11 * assign13300_e18298), (locals.var_uc_i_dn12 * assign13300_e18298), (locals.var_uc_i_dn13 * assign13300_e18298), (locals.var_uc_i_dn14 * assign13300_e18298), );

        let assign13310_e18303: f64 = (locals.var_tratio).powf(locals.var_ud1_i);
        let assign13310_e18304: f64 = (locals.var_ud_i * assign13310_e18303);
        (locals.var_ud_t, locals.var_ud_t_dn0, locals.var_ud_t_dn2, locals.var_ud_t_dn3, locals.var_ud_t_dn4, locals.var_ud_t_dn5, locals.var_ud_t_dn6, locals.var_ud_t_dn7, locals.var_ud_t_dn8, locals.var_ud_t_dn9, locals.var_ud_t_dn10, locals.var_ud_t_dn11, locals.var_ud_t_dn12, locals.var_ud_t_dn13, locals.var_ud_t_dn14, ) = (assign13310_e18304, (locals.var_ud_i_dn0 * assign13310_e18303), (locals.var_ud_i_dn2 * assign13310_e18303), (locals.var_ud_i_dn3 * assign13310_e18303), ((locals.var_ud_i_dn4 * assign13310_e18303) + (locals.var_ud_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13310_e18303 * (locals.var_ud1_i * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_ud_i_dn5 * assign13310_e18303), (locals.var_ud_i_dn6 * assign13310_e18303), (locals.var_ud_i_dn7 * assign13310_e18303), (locals.var_ud_i_dn8 * assign13310_e18303), (locals.var_ud_i_dn9 * assign13310_e18303), (locals.var_ud_i_dn10 * assign13310_e18303), (locals.var_ud_i_dn11 * assign13310_e18303), (locals.var_ud_i_dn12 * assign13310_e18303), (locals.var_ud_i_dn13 * assign13310_e18303), (locals.var_ud_i_dn14 * assign13310_e18303), );

        let assign13320_e18308: f64 = (locals.var_tratio).powf(locals.var_ucste_i);
        let assign13320_e18309: f64 = (locals.var_ucs_i * assign13320_e18308);
        (locals.var_ucs_t, locals.var_ucs_t_dn4, ) = (assign13320_e18309, (locals.var_ucs_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13320_e18308 * (locals.var_ucste_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), );

        let assign13330_e18315: f64 = (locals.var_tratio - 1.0);
        let assign13330_e18316: f64 = (locals.var_eu1_i * assign13330_e18315);
        let assign13330_e18317: f64 = (1.0 + assign13330_e18316);
        let assign13330_e18319: f64 = (-10000.0);
        let assign13330_e18321: f64 = (assign13330_e18319 * 0.001);
        let (assign13330_e18382, assign13330_e18382_d_n4,) = {
    if (!(assign13330_e18317 < assign13330_e18321)) {
        let assign13330_e18329: f64 = (locals.var_tratio - 1.0);
        let assign13330_e18330: f64 = (locals.var_eu1_i * assign13330_e18329);
        let assign13330_e18331: f64 = (1.0 + assign13330_e18330);
        let assign13330_e18336: f64 = (locals.var_tratio - 1.0);
        let assign13330_e18337: f64 = (locals.var_eu1_i * assign13330_e18336);
        let assign13330_e18338: f64 = (1.0 + assign13330_e18337);
        let assign13330_e18343: f64 = (locals.var_tratio - 1.0);
        let assign13330_e18344: f64 = (locals.var_eu1_i * assign13330_e18343);
        let assign13330_e18345: f64 = (1.0 + assign13330_e18344);
        let assign13330_e18346: f64 = (assign13330_e18338 * assign13330_e18345);
        let assign13330_e18349: f64 = (4.0 * 0.001);
        let assign13330_e18351: f64 = (assign13330_e18349 * 0.001);
        let assign13330_e18352: f64 = (assign13330_e18346 + assign13330_e18351);
        let assign13330_e18353: f64 = (assign13330_e18352).sqrt();
        let assign13330_e18354: f64 = (assign13330_e18331 + assign13330_e18353);
        let assign13330_e18355: f64 = (0.5 * assign13330_e18354);
        (assign13330_e18355, (0.5 * ((locals.var_eu1_i * locals.var_tratio_dn4) + ((((locals.var_eu1_i * locals.var_tratio_dn4) * assign13330_e18345) + (assign13330_e18338 * (locals.var_eu1_i * locals.var_tratio_dn4))) / (2.0 * assign13330_e18353)))),)
    } else {
        let assign13330_e18360: f64 = (locals.var_tratio - 1.0);
        let assign13330_e18361: f64 = (locals.var_eu1_i * assign13330_e18360);
        let assign13330_e18362: f64 = (1.0 + assign13330_e18361);
        let assign13330_e18364: f64 = (-10000.0);
        let assign13330_e18366: f64 = (assign13330_e18364 * 0.001);
        let (assign13330_e18381, assign13330_e18381_d_n4,) = {
            if (assign13330_e18362 < assign13330_e18366) {
                let assign13330_e18369: f64 = (-0.001);
                let assign13330_e18371: f64 = (assign13330_e18369 * 0.001);
                let assign13330_e18376: f64 = (locals.var_tratio - 1.0);
                let assign13330_e18377: f64 = (locals.var_eu1_i * assign13330_e18376);
                let assign13330_e18378: f64 = (1.0 + assign13330_e18377);
                let assign13330_e18379: f64 = (assign13330_e18371 / assign13330_e18378);
                (assign13330_e18379, (-((assign13330_e18371 * (locals.var_eu1_i * locals.var_tratio_dn4)) / (assign13330_e18378 * assign13330_e18378))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13330_e18381, assign13330_e18381_d_n4,)
    }
};
        let assign13330_e18383: f64 = (locals.var_eu_i * assign13330_e18382);
        (locals.var_eu_t, locals.var_eu_t_dn0, locals.var_eu_t_dn2, locals.var_eu_t_dn3, locals.var_eu_t_dn4, locals.var_eu_t_dn5, locals.var_eu_t_dn6, locals.var_eu_t_dn7, locals.var_eu_t_dn8, locals.var_eu_t_dn9, locals.var_eu_t_dn10, locals.var_eu_t_dn11, locals.var_eu_t_dn12, locals.var_eu_t_dn13, locals.var_eu_t_dn14, ) = (assign13330_e18383, (locals.var_eu_i_dn0 * assign13330_e18382), (locals.var_eu_i_dn2 * assign13330_e18382), (locals.var_eu_i_dn3 * assign13330_e18382), ((locals.var_eu_i_dn4 * assign13330_e18382) + (locals.var_eu_i * assign13330_e18382_d_n4)), (locals.var_eu_i_dn5 * assign13330_e18382), (locals.var_eu_i_dn6 * assign13330_e18382), (locals.var_eu_i_dn7 * assign13330_e18382), (locals.var_eu_i_dn8 * assign13330_e18382), (locals.var_eu_i_dn9 * assign13330_e18382), (locals.var_eu_i_dn10 * assign13330_e18382), (locals.var_eu_i_dn11 * assign13330_e18382), (locals.var_eu_i_dn12 * assign13330_e18382), (locals.var_eu_i_dn13 * assign13330_e18382), (locals.var_eu_i_dn14 * assign13330_e18382), );

        let assign13340_e18386: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard454 = assign13340_e18386;

        if (locals.var_guard454 != 0.0) {
            let assign13350_e18391: f64 = (locals.var_tratio).powf(locals.var_ute_i);
            let assign13350_e18392: f64 = (locals.var_u0r_i * assign13350_e18391);
            (locals.var_u0r_t, locals.var_u0r_t_dn4, ) = (assign13350_e18392, (locals.var_u0r_i * if 0.0 == 0.0 && ((locals.var_ute_i) as f64).is_finite() && ((locals.var_ute_i) as f64).fract() == 0.0 { if locals.var_ute_i == 0.0 { 0.0 } else { (locals.var_ute_i * ((locals.var_tratio).powf(locals.var_ute_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13350_e18391 * (locals.var_ute_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), );
        }

        if (locals.var_guard454 != 0.0) {
            let assign13360_e18400: f64 = (locals.var_ua1_i * locals.var_deltemp);
            let assign13360_e18401: f64 = (1.0 + assign13360_e18400);
            let assign13360_e18403: f64 = (assign13360_e18401 - 1e-6);
            let assign13360_e18405: f64 = (-10000.0);
            let assign13360_e18407: f64 = (assign13360_e18405 * 0.001);
            let (assign13360_e18468, assign13360_e18468_d_n4,) = {
    if (!(assign13360_e18403 < assign13360_e18407)) {
        let assign13360_e18414: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13360_e18415: f64 = (1.0 + assign13360_e18414);
        let assign13360_e18417: f64 = (assign13360_e18415 - 1e-6);
        let assign13360_e18421: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13360_e18422: f64 = (1.0 + assign13360_e18421);
        let assign13360_e18424: f64 = (assign13360_e18422 - 1e-6);
        let assign13360_e18428: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13360_e18429: f64 = (1.0 + assign13360_e18428);
        let assign13360_e18431: f64 = (assign13360_e18429 - 1e-6);
        let assign13360_e18432: f64 = (assign13360_e18424 * assign13360_e18431);
        let assign13360_e18435: f64 = (4.0 * 0.001);
        let assign13360_e18437: f64 = (assign13360_e18435 * 0.001);
        let assign13360_e18438: f64 = (assign13360_e18432 + assign13360_e18437);
        let assign13360_e18439: f64 = (assign13360_e18438).sqrt();
        let assign13360_e18440: f64 = (assign13360_e18417 + assign13360_e18439);
        let assign13360_e18441: f64 = (0.5 * assign13360_e18440);
        (assign13360_e18441, (0.5 * ((locals.var_ua1_i * locals.var_deltemp_dn4) + ((((locals.var_ua1_i * locals.var_deltemp_dn4) * assign13360_e18431) + (assign13360_e18424 * (locals.var_ua1_i * locals.var_deltemp_dn4))) / (2.0 * assign13360_e18439)))),)
    } else {
        let assign13360_e18445: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign13360_e18446: f64 = (1.0 + assign13360_e18445);
        let assign13360_e18448: f64 = (assign13360_e18446 - 1e-6);
        let assign13360_e18450: f64 = (-10000.0);
        let assign13360_e18452: f64 = (assign13360_e18450 * 0.001);
        let (assign13360_e18467, assign13360_e18467_d_n4,) = {
            if (assign13360_e18448 < assign13360_e18452) {
                let assign13360_e18455: f64 = (-0.001);
                let assign13360_e18457: f64 = (assign13360_e18455 * 0.001);
                let assign13360_e18461: f64 = (locals.var_ua1_i * locals.var_deltemp);
                let assign13360_e18462: f64 = (1.0 + assign13360_e18461);
                let assign13360_e18464: f64 = (assign13360_e18462 - 1e-6);
                let assign13360_e18465: f64 = (assign13360_e18457 / assign13360_e18464);
                (assign13360_e18465, (-((assign13360_e18457 * (locals.var_ua1_i * locals.var_deltemp_dn4)) / (assign13360_e18464 * assign13360_e18464))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13360_e18467, assign13360_e18467_d_n4,)
    }
};
            let assign13360_e18469: f64 = (locals.var_uar_i * assign13360_e18468);
            (locals.var_uar_t, locals.var_uar_t_dn0, locals.var_uar_t_dn2, locals.var_uar_t_dn3, locals.var_uar_t_dn4, locals.var_uar_t_dn5, locals.var_uar_t_dn6, locals.var_uar_t_dn7, locals.var_uar_t_dn8, locals.var_uar_t_dn9, locals.var_uar_t_dn10, locals.var_uar_t_dn11, locals.var_uar_t_dn12, locals.var_uar_t_dn13, locals.var_uar_t_dn14, ) = (assign13360_e18469, (locals.var_uar_i_dn0 * assign13360_e18468), (locals.var_uar_i_dn2 * assign13360_e18468), (locals.var_uar_i_dn3 * assign13360_e18468), ((locals.var_uar_i_dn4 * assign13360_e18468) + (locals.var_uar_i * assign13360_e18468_d_n4)), (locals.var_uar_i_dn5 * assign13360_e18468), (locals.var_uar_i_dn6 * assign13360_e18468), (locals.var_uar_i_dn7 * assign13360_e18468), (locals.var_uar_i_dn8 * assign13360_e18468), (locals.var_uar_i_dn9 * assign13360_e18468), (locals.var_uar_i_dn10 * assign13360_e18468), (locals.var_uar_i_dn11 * assign13360_e18468), (locals.var_uar_i_dn12 * assign13360_e18468), (locals.var_uar_i_dn13 * assign13360_e18468), (locals.var_uar_i_dn14 * assign13360_e18468), );
        }

        if (locals.var_guard454 != 0.0) {
            let assign13370_e18477: f64 = (locals.var_uc1_i * locals.var_deltemp);
            let assign13370_e18478: f64 = (1.0 + assign13370_e18477);
            let assign13370_e18480: f64 = (assign13370_e18478 - 1e-6);
            let assign13370_e18482: f64 = (-10000.0);
            let assign13370_e18484: f64 = (assign13370_e18482 * 0.001);
            let (assign13370_e18545, assign13370_e18545_d_n4,) = {
    if (!(assign13370_e18480 < assign13370_e18484)) {
        let assign13370_e18491: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13370_e18492: f64 = (1.0 + assign13370_e18491);
        let assign13370_e18494: f64 = (assign13370_e18492 - 1e-6);
        let assign13370_e18498: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13370_e18499: f64 = (1.0 + assign13370_e18498);
        let assign13370_e18501: f64 = (assign13370_e18499 - 1e-6);
        let assign13370_e18505: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13370_e18506: f64 = (1.0 + assign13370_e18505);
        let assign13370_e18508: f64 = (assign13370_e18506 - 1e-6);
        let assign13370_e18509: f64 = (assign13370_e18501 * assign13370_e18508);
        let assign13370_e18512: f64 = (4.0 * 0.001);
        let assign13370_e18514: f64 = (assign13370_e18512 * 0.001);
        let assign13370_e18515: f64 = (assign13370_e18509 + assign13370_e18514);
        let assign13370_e18516: f64 = (assign13370_e18515).sqrt();
        let assign13370_e18517: f64 = (assign13370_e18494 + assign13370_e18516);
        let assign13370_e18518: f64 = (0.5 * assign13370_e18517);
        (assign13370_e18518, (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn4) + ((((locals.var_uc1_i * locals.var_deltemp_dn4) * assign13370_e18508) + (assign13370_e18501 * (locals.var_uc1_i * locals.var_deltemp_dn4))) / (2.0 * assign13370_e18516)))),)
    } else {
        let assign13370_e18522: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign13370_e18523: f64 = (1.0 + assign13370_e18522);
        let assign13370_e18525: f64 = (assign13370_e18523 - 1e-6);
        let assign13370_e18527: f64 = (-10000.0);
        let assign13370_e18529: f64 = (assign13370_e18527 * 0.001);
        let (assign13370_e18544, assign13370_e18544_d_n4,) = {
            if (assign13370_e18525 < assign13370_e18529) {
                let assign13370_e18532: f64 = (-0.001);
                let assign13370_e18534: f64 = (assign13370_e18532 * 0.001);
                let assign13370_e18538: f64 = (locals.var_uc1_i * locals.var_deltemp);
                let assign13370_e18539: f64 = (1.0 + assign13370_e18538);
                let assign13370_e18541: f64 = (assign13370_e18539 - 1e-6);
                let assign13370_e18542: f64 = (assign13370_e18534 / assign13370_e18541);
                (assign13370_e18542, (-((assign13370_e18534 * (locals.var_uc1_i * locals.var_deltemp_dn4)) / (assign13370_e18541 * assign13370_e18541))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13370_e18544, assign13370_e18544_d_n4,)
    }
};
            let assign13370_e18546: f64 = (locals.var_ucr_i * assign13370_e18545);
            (locals.var_ucr_t, locals.var_ucr_t_dn0, locals.var_ucr_t_dn2, locals.var_ucr_t_dn3, locals.var_ucr_t_dn4, locals.var_ucr_t_dn5, locals.var_ucr_t_dn6, locals.var_ucr_t_dn7, locals.var_ucr_t_dn8, locals.var_ucr_t_dn9, locals.var_ucr_t_dn10, locals.var_ucr_t_dn11, locals.var_ucr_t_dn12, locals.var_ucr_t_dn13, locals.var_ucr_t_dn14, ) = (assign13370_e18546, (locals.var_ucr_i_dn0 * assign13370_e18545), (locals.var_ucr_i_dn2 * assign13370_e18545), (locals.var_ucr_i_dn3 * assign13370_e18545), ((locals.var_ucr_i_dn4 * assign13370_e18545) + (locals.var_ucr_i * assign13370_e18545_d_n4)), (locals.var_ucr_i_dn5 * assign13370_e18545), (locals.var_ucr_i_dn6 * assign13370_e18545), (locals.var_ucr_i_dn7 * assign13370_e18545), (locals.var_ucr_i_dn8 * assign13370_e18545), (locals.var_ucr_i_dn9 * assign13370_e18545), (locals.var_ucr_i_dn10 * assign13370_e18545), (locals.var_ucr_i_dn11 * assign13370_e18545), (locals.var_ucr_i_dn12 * assign13370_e18545), (locals.var_ucr_i_dn13 * assign13370_e18545), (locals.var_ucr_i_dn14 * assign13370_e18545), );
        }

        if (locals.var_guard454 != 0.0) {
            let assign13380_e18553: f64 = (locals.var_tratio).powf(locals.var_ud1_i);
            let assign13380_e18554: f64 = (locals.var_udr_i * assign13380_e18553);
            (locals.var_udr_t, locals.var_udr_t_dn0, locals.var_udr_t_dn2, locals.var_udr_t_dn3, locals.var_udr_t_dn4, locals.var_udr_t_dn5, locals.var_udr_t_dn6, locals.var_udr_t_dn7, locals.var_udr_t_dn8, locals.var_udr_t_dn9, locals.var_udr_t_dn10, locals.var_udr_t_dn11, locals.var_udr_t_dn12, locals.var_udr_t_dn13, locals.var_udr_t_dn14, ) = (assign13380_e18554, (locals.var_udr_i_dn0 * assign13380_e18553), (locals.var_udr_i_dn2 * assign13380_e18553), (locals.var_udr_i_dn3 * assign13380_e18553), ((locals.var_udr_i_dn4 * assign13380_e18553) + (locals.var_udr_i * if 0.0 == 0.0 && ((locals.var_ud1_i) as f64).is_finite() && ((locals.var_ud1_i) as f64).fract() == 0.0 { if locals.var_ud1_i == 0.0 { 0.0 } else { (locals.var_ud1_i * ((locals.var_tratio).powf(locals.var_ud1_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13380_e18553 * (locals.var_ud1_i * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_udr_i_dn5 * assign13380_e18553), (locals.var_udr_i_dn6 * assign13380_e18553), (locals.var_udr_i_dn7 * assign13380_e18553), (locals.var_udr_i_dn8 * assign13380_e18553), (locals.var_udr_i_dn9 * assign13380_e18553), (locals.var_udr_i_dn10 * assign13380_e18553), (locals.var_udr_i_dn11 * assign13380_e18553), (locals.var_udr_i_dn12 * assign13380_e18553), (locals.var_udr_i_dn13 * assign13380_e18553), (locals.var_udr_i_dn14 * assign13380_e18553), );
        }

        if (locals.var_guard454 != 0.0) {
            let assign13390_e18561: f64 = (locals.var_tratio).powf(locals.var_ucste_i);
            let assign13390_e18562: f64 = (locals.var_ucsr_i * assign13390_e18561);
            (locals.var_ucsr_t, locals.var_ucsr_t_dn4, ) = (assign13390_e18562, (locals.var_ucsr_i * if 0.0 == 0.0 && ((locals.var_ucste_i) as f64).is_finite() && ((locals.var_ucste_i) as f64).fract() == 0.0 { if locals.var_ucste_i == 0.0 { 0.0 } else { (locals.var_ucste_i * ((locals.var_tratio).powf(locals.var_ucste_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13390_e18561 * (locals.var_ucste_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), );
        }

        let assign13400_e18567: f64 = (locals.var_tratio).powf(locals.var_prt_i);
        (locals.var_rdstemp, locals.var_rdstemp_dn4, ) = (assign13400_e18567, if 0.0 == 0.0 && ((locals.var_prt_i) as f64).is_finite() && ((locals.var_prt_i) as f64).fract() == 0.0 { if locals.var_prt_i == 0.0 { 0.0 } else { (locals.var_prt_i * ((locals.var_tratio).powf(locals.var_prt_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13400_e18567 * (locals.var_prt_i * (locals.var_tratio_dn4 / locals.var_tratio))) }, );

        let assign13410_e18571: f64 = (-locals.var_at_i);
        let assign13410_e18572: f64 = (locals.var_tratio).powf(assign13410_e18571);
        let assign13410_e18573: f64 = (locals.var_vsat_i * assign13410_e18572);
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn12, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14, ) = (assign13410_e18573, (locals.var_vsat_i_dn0 * assign13410_e18572), (locals.var_vsat_i_dn2 * assign13410_e18572), (locals.var_vsat_i_dn3 * assign13410_e18572), ((locals.var_vsat_i_dn4 * assign13410_e18572) + (locals.var_vsat_i * if 0.0 == 0.0 && ((assign13410_e18571) as f64).is_finite() && ((assign13410_e18571) as f64).fract() == 0.0 { if assign13410_e18571 == 0.0 { 0.0 } else { (assign13410_e18571 * ((locals.var_tratio).powf(assign13410_e18571 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13410_e18572 * (assign13410_e18571 * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_vsat_i_dn5 * assign13410_e18572), (locals.var_vsat_i_dn6 * assign13410_e18572), (locals.var_vsat_i_dn7 * assign13410_e18572), (locals.var_vsat_i_dn8 * assign13410_e18572), (locals.var_vsat_i_dn9 * assign13410_e18572), (locals.var_vsat_i_dn10 * assign13410_e18572), (locals.var_vsat_i_dn11 * assign13410_e18572), (locals.var_vsat_i_dn12 * assign13410_e18572), (locals.var_vsat_i_dn13 * assign13410_e18572), (locals.var_vsat_i_dn14 * assign13410_e18572), );

        let assign13420_e18576: f64 = if locals.var_vsat_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard455 = assign13420_e18576;

        if (locals.var_guard455 != 0.0) {
            (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn12, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14, ) = (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign13440_e18583: f64 = if p.p1094 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard456 = assign13440_e18583;

        if (locals.var_guard456 != 0.0) {
            let assign13450_e18587: f64 = (locals.var_tratio).powf(p.p1120);
            (locals.var_rdstemphv, locals.var_rdstemphv_dn4, ) = (assign13450_e18587, if 0.0 == 0.0 && ((p.p1120) as f64).is_finite() && ((p.p1120) as f64).fract() == 0.0 { if p.p1120 == 0.0 { 0.0 } else { (p.p1120 * ((locals.var_tratio).powf(p.p1120 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13450_e18587 * (p.p1120 * (locals.var_tratio_dn4 / locals.var_tratio))) }, );
        }

        if (locals.var_guard456 != 0.0) {
            let assign13460_e18594: f64 = (-p.p1121);
            let assign13460_e18595: f64 = (locals.var_tratio).powf(assign13460_e18594);
            let assign13460_e18596: f64 = (p.p1100 * assign13460_e18595);
            (locals.var_vdrift_t, locals.var_vdrift_t_dn4, ) = (assign13460_e18596, (p.p1100 * if 0.0 == 0.0 && ((assign13460_e18594) as f64).is_finite() && ((assign13460_e18594) as f64).fract() == 0.0 { if assign13460_e18594 == 0.0 { 0.0 } else { (assign13460_e18594 * ((locals.var_tratio).powf(assign13460_e18594 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13460_e18595 * (assign13460_e18594 * (locals.var_tratio_dn4 / locals.var_tratio))) }), );
        }

        let assign13470_e18601: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard457 = assign13470_e18601;

        if (locals.var_guard457 != 0.0) {
            let assign13480_e18606: f64 = (-locals.var_at_i);
            let assign13480_e18607: f64 = (locals.var_tratio).powf(assign13480_e18606);
            let assign13480_e18608: f64 = (locals.var_vsatr_i * assign13480_e18607);
            (locals.var_vsatr_t, locals.var_vsatr_t_dn0, locals.var_vsatr_t_dn2, locals.var_vsatr_t_dn3, locals.var_vsatr_t_dn4, locals.var_vsatr_t_dn5, locals.var_vsatr_t_dn6, locals.var_vsatr_t_dn7, locals.var_vsatr_t_dn8, locals.var_vsatr_t_dn9, locals.var_vsatr_t_dn10, locals.var_vsatr_t_dn11, locals.var_vsatr_t_dn12, locals.var_vsatr_t_dn13, locals.var_vsatr_t_dn14, ) = (assign13480_e18608, (locals.var_vsatr_i_dn0 * assign13480_e18607), (locals.var_vsatr_i_dn2 * assign13480_e18607), (locals.var_vsatr_i_dn3 * assign13480_e18607), ((locals.var_vsatr_i_dn4 * assign13480_e18607) + (locals.var_vsatr_i * if 0.0 == 0.0 && ((assign13480_e18606) as f64).is_finite() && ((assign13480_e18606) as f64).fract() == 0.0 { if assign13480_e18606 == 0.0 { 0.0 } else { (assign13480_e18606 * ((locals.var_tratio).powf(assign13480_e18606 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13480_e18607 * (assign13480_e18606 * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_vsatr_i_dn5 * assign13480_e18607), (locals.var_vsatr_i_dn6 * assign13480_e18607), (locals.var_vsatr_i_dn7 * assign13480_e18607), (locals.var_vsatr_i_dn8 * assign13480_e18607), (locals.var_vsatr_i_dn9 * assign13480_e18607), (locals.var_vsatr_i_dn10 * assign13480_e18607), (locals.var_vsatr_i_dn11 * assign13480_e18607), (locals.var_vsatr_i_dn12 * assign13480_e18607), (locals.var_vsatr_i_dn13 * assign13480_e18607), (locals.var_vsatr_i_dn14 * assign13480_e18607), );
        }

        let assign13490_e18613: f64 = if locals.var_vsatr_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard458 = assign13490_e18613;

        if ((locals.var_guard457 != 0.0) && (locals.var_guard458 != 0.0)) {
            (locals.var_vsatr_t, locals.var_vsatr_t_dn0, locals.var_vsatr_t_dn2, locals.var_vsatr_t_dn3, locals.var_vsatr_t_dn4, locals.var_vsatr_t_dn5, locals.var_vsatr_t_dn6, locals.var_vsatr_t_dn7, locals.var_vsatr_t_dn8, locals.var_vsatr_t_dn9, locals.var_vsatr_t_dn10, locals.var_vsatr_t_dn11, locals.var_vsatr_t_dn12, locals.var_vsatr_t_dn13, locals.var_vsatr_t_dn14, ) = (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign13510_e18623: f64 = (-locals.var_at_i);
        let assign13510_e18624: f64 = (locals.var_tratio).powf(assign13510_e18623);
        let assign13510_e18625: f64 = (locals.var_vsatcv_i * assign13510_e18624);
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn12, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14, ) = (assign13510_e18625, (locals.var_vsatcv_i_dn0 * assign13510_e18624), (locals.var_vsatcv_i_dn2 * assign13510_e18624), (locals.var_vsatcv_i_dn3 * assign13510_e18624), ((locals.var_vsatcv_i_dn4 * assign13510_e18624) + (locals.var_vsatcv_i * if 0.0 == 0.0 && ((assign13510_e18623) as f64).is_finite() && ((assign13510_e18623) as f64).fract() == 0.0 { if assign13510_e18623 == 0.0 { 0.0 } else { (assign13510_e18623 * ((locals.var_tratio).powf(assign13510_e18623 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13510_e18624 * (assign13510_e18623 * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_vsatcv_i_dn5 * assign13510_e18624), (locals.var_vsatcv_i_dn6 * assign13510_e18624), (locals.var_vsatcv_i_dn7 * assign13510_e18624), (locals.var_vsatcv_i_dn8 * assign13510_e18624), (locals.var_vsatcv_i_dn9 * assign13510_e18624), (locals.var_vsatcv_i_dn10 * assign13510_e18624), (locals.var_vsatcv_i_dn11 * assign13510_e18624), (locals.var_vsatcv_i_dn12 * assign13510_e18624), (locals.var_vsatcv_i_dn13 * assign13510_e18624), (locals.var_vsatcv_i_dn14 * assign13510_e18624), );

        let assign13520_e18628: f64 = if locals.var_vsatcv_t < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard459 = assign13520_e18628;

        if (locals.var_guard459 != 0.0) {
            (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn12, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14, ) = (100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13540_e18636: f64 = (1.0 / locals.var_delta_i);
        let assign13540_e18640: f64 = (p.p861 * locals.var_deltemp);
        let assign13540_e18641: f64 = (1.0 + assign13540_e18640);
        let assign13540_e18642: f64 = (assign13540_e18636 * assign13540_e18641);
        let assign13540_e18644: f64 = (assign13540_e18642 - 2.0);
        let assign13540_e18646: f64 = (-10000.0);
        let assign13540_e18648: f64 = (assign13540_e18646 * 0.001);
        let (assign13540_e18729, assign13540_e18729_d_n0, assign13540_e18729_d_n2, assign13540_e18729_d_n3, assign13540_e18729_d_n4, assign13540_e18729_d_n5, assign13540_e18729_d_n6, assign13540_e18729_d_n7, assign13540_e18729_d_n8, assign13540_e18729_d_n9, assign13540_e18729_d_n10, assign13540_e18729_d_n11, assign13540_e18729_d_n12, assign13540_e18729_d_n13, assign13540_e18729_d_n14,) = {
    if (!(assign13540_e18644 < assign13540_e18648)) {
        let assign13540_e18654: f64 = (1.0 / locals.var_delta_i);
        let assign13540_e18658: f64 = (p.p861 * locals.var_deltemp);
        let assign13540_e18659: f64 = (1.0 + assign13540_e18658);
        let assign13540_e18660: f64 = (assign13540_e18654 * assign13540_e18659);
        let assign13540_e18662: f64 = (assign13540_e18660 - 2.0);
        let assign13540_e18665: f64 = (1.0 / locals.var_delta_i);
        let assign13540_e18669: f64 = (p.p861 * locals.var_deltemp);
        let assign13540_e18670: f64 = (1.0 + assign13540_e18669);
        let assign13540_e18671: f64 = (assign13540_e18665 * assign13540_e18670);
        let assign13540_e18673: f64 = (assign13540_e18671 - 2.0);
        let assign13540_e18676: f64 = (1.0 / locals.var_delta_i);
        let assign13540_e18680: f64 = (p.p861 * locals.var_deltemp);
        let assign13540_e18681: f64 = (1.0 + assign13540_e18680);
        let assign13540_e18682: f64 = (assign13540_e18676 * assign13540_e18681);
        let assign13540_e18684: f64 = (assign13540_e18682 - 2.0);
        let assign13540_e18685: f64 = (assign13540_e18673 * assign13540_e18684);
        let assign13540_e18688: f64 = (4.0 * 0.001);
        let assign13540_e18690: f64 = (assign13540_e18688 * 0.001);
        let assign13540_e18691: f64 = (assign13540_e18685 + assign13540_e18690);
        let assign13540_e18692: f64 = (assign13540_e18691).sqrt();
        let assign13540_e18693: f64 = (assign13540_e18662 + assign13540_e18692);
        let assign13540_e18694: f64 = (0.5 * assign13540_e18693);
        (assign13540_e18694, (0.5 * (((-(locals.var_delta_i_dn0 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn0 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn0 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn2 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn2 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn2 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * ((((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (assign13540_e18654 * (p.p861 * locals.var_deltemp_dn4))) + ((((((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) + (assign13540_e18665 * (p.p861 * locals.var_deltemp_dn4))) * assign13540_e18684) + (assign13540_e18673 * (((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681) + (assign13540_e18676 * (p.p861 * locals.var_deltemp_dn4))))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn12 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn12 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn12 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn13 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn13 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn13 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))), (0.5 * (((-(locals.var_delta_i_dn14 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18659) + (((((-(locals.var_delta_i_dn14 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18670) * assign13540_e18684) + (assign13540_e18673 * ((-(locals.var_delta_i_dn14 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18681))) / (2.0 * assign13540_e18692)))),)
    } else {
        let assign13540_e18697: f64 = (1.0 / locals.var_delta_i);
        let assign13540_e18701: f64 = (p.p861 * locals.var_deltemp);
        let assign13540_e18702: f64 = (1.0 + assign13540_e18701);
        let assign13540_e18703: f64 = (assign13540_e18697 * assign13540_e18702);
        let assign13540_e18705: f64 = (assign13540_e18703 - 2.0);
        let assign13540_e18707: f64 = (-10000.0);
        let assign13540_e18709: f64 = (assign13540_e18707 * 0.001);
        let (assign13540_e18728, assign13540_e18728_d_n0, assign13540_e18728_d_n2, assign13540_e18728_d_n3, assign13540_e18728_d_n4, assign13540_e18728_d_n5, assign13540_e18728_d_n6, assign13540_e18728_d_n7, assign13540_e18728_d_n8, assign13540_e18728_d_n9, assign13540_e18728_d_n10, assign13540_e18728_d_n11, assign13540_e18728_d_n12, assign13540_e18728_d_n13, assign13540_e18728_d_n14,) = {
            if (assign13540_e18705 < assign13540_e18709) {
                let assign13540_e18712: f64 = (-0.001);
                let assign13540_e18714: f64 = (assign13540_e18712 * 0.001);
                let assign13540_e18717: f64 = (1.0 / locals.var_delta_i);
                let assign13540_e18721: f64 = (p.p861 * locals.var_deltemp);
                let assign13540_e18722: f64 = (1.0 + assign13540_e18721);
                let assign13540_e18723: f64 = (assign13540_e18717 * assign13540_e18722);
                let assign13540_e18725: f64 = (assign13540_e18723 - 2.0);
                let assign13540_e18726: f64 = (assign13540_e18714 / assign13540_e18725);
                (assign13540_e18726, (-((assign13540_e18714 * ((-(locals.var_delta_i_dn0 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn2 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn3 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * (((-(locals.var_delta_i_dn4 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722) + (assign13540_e18717 * (p.p861 * locals.var_deltemp_dn4)))) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn5 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn6 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn7 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn8 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn9 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn10 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn11 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn12 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn13 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))), (-((assign13540_e18714 * ((-(locals.var_delta_i_dn14 / (locals.var_delta_i * locals.var_delta_i))) * assign13540_e18722)) / (assign13540_e18725 * assign13540_e18725))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign13540_e18728, assign13540_e18728_d_n0, assign13540_e18728_d_n2, assign13540_e18728_d_n3, assign13540_e18728_d_n4, assign13540_e18728_d_n5, assign13540_e18728_d_n6, assign13540_e18728_d_n7, assign13540_e18728_d_n8, assign13540_e18728_d_n9, assign13540_e18728_d_n10, assign13540_e18728_d_n11, assign13540_e18728_d_n12, assign13540_e18728_d_n13, assign13540_e18728_d_n14,)
    }
};
        let assign13540_e18731: f64 = (assign13540_e18729 + 2.0);
        let assign13540_e18732: f64 = (1.0 / assign13540_e18731);
        (locals.var_delta_t, locals.var_delta_t_dn0, locals.var_delta_t_dn2, locals.var_delta_t_dn3, locals.var_delta_t_dn4, locals.var_delta_t_dn5, locals.var_delta_t_dn6, locals.var_delta_t_dn7, locals.var_delta_t_dn8, locals.var_delta_t_dn9, locals.var_delta_t_dn10, locals.var_delta_t_dn11, locals.var_delta_t_dn12, locals.var_delta_t_dn13, locals.var_delta_t_dn14, ) = (assign13540_e18732, (-(assign13540_e18729_d_n0 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n2 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n3 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n4 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n5 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n6 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n7 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n8 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n9 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n10 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n11 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n12 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n13 / (assign13540_e18731 * assign13540_e18731))), (-(assign13540_e18729_d_n14 / (assign13540_e18731 * assign13540_e18731))), );

        let assign13550_e18737: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13550_e18738: f64 = (1.0 - assign13550_e18737);
        let assign13550_e18740: f64 = (assign13550_e18738 - 1e-6);
        let assign13550_e18742: f64 = (-10000.0);
        let assign13550_e18744: f64 = (assign13550_e18742 * 0.001);
        let (assign13550_e18805, assign13550_e18805_d_n4,) = {
    if (!(assign13550_e18740 < assign13550_e18744)) {
        let assign13550_e18751: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13550_e18752: f64 = (1.0 - assign13550_e18751);
        let assign13550_e18754: f64 = (assign13550_e18752 - 1e-6);
        let assign13550_e18758: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13550_e18759: f64 = (1.0 - assign13550_e18758);
        let assign13550_e18761: f64 = (assign13550_e18759 - 1e-6);
        let assign13550_e18765: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13550_e18766: f64 = (1.0 - assign13550_e18765);
        let assign13550_e18768: f64 = (assign13550_e18766 - 1e-6);
        let assign13550_e18769: f64 = (assign13550_e18761 * assign13550_e18768);
        let assign13550_e18772: f64 = (4.0 * 0.001);
        let assign13550_e18774: f64 = (assign13550_e18772 * 0.001);
        let assign13550_e18775: f64 = (assign13550_e18769 + assign13550_e18774);
        let assign13550_e18776: f64 = (assign13550_e18775).sqrt();
        let assign13550_e18777: f64 = (assign13550_e18754 + assign13550_e18776);
        let assign13550_e18778: f64 = (0.5 * assign13550_e18777);
        (assign13550_e18778, (0.5 * ((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) + ((((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) * assign13550_e18768) + (assign13550_e18761 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4)))) / (2.0 * assign13550_e18776)))),)
    } else {
        let assign13550_e18782: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13550_e18783: f64 = (1.0 - assign13550_e18782);
        let assign13550_e18785: f64 = (assign13550_e18783 - 1e-6);
        let assign13550_e18787: f64 = (-10000.0);
        let assign13550_e18789: f64 = (assign13550_e18787 * 0.001);
        let (assign13550_e18804, assign13550_e18804_d_n4,) = {
            if (assign13550_e18785 < assign13550_e18789) {
                let assign13550_e18792: f64 = (-0.001);
                let assign13550_e18794: f64 = (assign13550_e18792 * 0.001);
                let assign13550_e18798: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
                let assign13550_e18799: f64 = (1.0 - assign13550_e18798);
                let assign13550_e18801: f64 = (assign13550_e18799 - 1e-6);
                let assign13550_e18802: f64 = (assign13550_e18794 / assign13550_e18801);
                (assign13550_e18802, (-((assign13550_e18794 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4))) / (assign13550_e18801 * assign13550_e18801))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13550_e18804, assign13550_e18804_d_n4,)
    }
};
        let assign13550_e18806: f64 = (locals.var_ptwg_i * assign13550_e18805);
        (locals.var_ptwg_t, locals.var_ptwg_t_dn0, locals.var_ptwg_t_dn2, locals.var_ptwg_t_dn3, locals.var_ptwg_t_dn4, locals.var_ptwg_t_dn5, locals.var_ptwg_t_dn6, locals.var_ptwg_t_dn7, locals.var_ptwg_t_dn8, locals.var_ptwg_t_dn9, locals.var_ptwg_t_dn10, locals.var_ptwg_t_dn11, locals.var_ptwg_t_dn12, locals.var_ptwg_t_dn13, locals.var_ptwg_t_dn14, ) = (assign13550_e18806, (locals.var_ptwg_i_dn0 * assign13550_e18805), (locals.var_ptwg_i_dn2 * assign13550_e18805), (locals.var_ptwg_i_dn3 * assign13550_e18805), ((locals.var_ptwg_i_dn4 * assign13550_e18805) + (locals.var_ptwg_i * assign13550_e18805_d_n4)), (locals.var_ptwg_i_dn5 * assign13550_e18805), (locals.var_ptwg_i_dn6 * assign13550_e18805), (locals.var_ptwg_i_dn7 * assign13550_e18805), (locals.var_ptwg_i_dn8 * assign13550_e18805), (locals.var_ptwg_i_dn9 * assign13550_e18805), (locals.var_ptwg_i_dn10 * assign13550_e18805), (locals.var_ptwg_i_dn11 * assign13550_e18805), (locals.var_ptwg_i_dn12 * assign13550_e18805), (locals.var_ptwg_i_dn13 * assign13550_e18805), (locals.var_ptwg_i_dn14 * assign13550_e18805), );

        let assign13560_e18809: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard460 = assign13560_e18809;

        if (locals.var_guard460 != 0.0) {
            let assign13570_e18815: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
            let assign13570_e18816: f64 = (1.0 - assign13570_e18815);
            let assign13570_e18818: f64 = (assign13570_e18816 - 1e-6);
            let assign13570_e18820: f64 = (-10000.0);
            let assign13570_e18822: f64 = (assign13570_e18820 * 0.001);
            let (assign13570_e18883, assign13570_e18883_d_n4,) = {
    if (!(assign13570_e18818 < assign13570_e18822)) {
        let assign13570_e18829: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13570_e18830: f64 = (1.0 - assign13570_e18829);
        let assign13570_e18832: f64 = (assign13570_e18830 - 1e-6);
        let assign13570_e18836: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13570_e18837: f64 = (1.0 - assign13570_e18836);
        let assign13570_e18839: f64 = (assign13570_e18837 - 1e-6);
        let assign13570_e18843: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13570_e18844: f64 = (1.0 - assign13570_e18843);
        let assign13570_e18846: f64 = (assign13570_e18844 - 1e-6);
        let assign13570_e18847: f64 = (assign13570_e18839 * assign13570_e18846);
        let assign13570_e18850: f64 = (4.0 * 0.001);
        let assign13570_e18852: f64 = (assign13570_e18850 * 0.001);
        let assign13570_e18853: f64 = (assign13570_e18847 + assign13570_e18852);
        let assign13570_e18854: f64 = (assign13570_e18853).sqrt();
        let assign13570_e18855: f64 = (assign13570_e18832 + assign13570_e18854);
        let assign13570_e18856: f64 = (0.5 * assign13570_e18855);
        (assign13570_e18856, (0.5 * ((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) + ((((-(locals.var_ptwgt_i * locals.var_deltemp_dn4)) * assign13570_e18846) + (assign13570_e18839 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4)))) / (2.0 * assign13570_e18854)))),)
    } else {
        let assign13570_e18860: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
        let assign13570_e18861: f64 = (1.0 - assign13570_e18860);
        let assign13570_e18863: f64 = (assign13570_e18861 - 1e-6);
        let assign13570_e18865: f64 = (-10000.0);
        let assign13570_e18867: f64 = (assign13570_e18865 * 0.001);
        let (assign13570_e18882, assign13570_e18882_d_n4,) = {
            if (assign13570_e18863 < assign13570_e18867) {
                let assign13570_e18870: f64 = (-0.001);
                let assign13570_e18872: f64 = (assign13570_e18870 * 0.001);
                let assign13570_e18876: f64 = (locals.var_ptwgt_i * locals.var_deltemp);
                let assign13570_e18877: f64 = (1.0 - assign13570_e18876);
                let assign13570_e18879: f64 = (assign13570_e18877 - 1e-6);
                let assign13570_e18880: f64 = (assign13570_e18872 / assign13570_e18879);
                (assign13570_e18880, (-((assign13570_e18872 * (-(locals.var_ptwgt_i * locals.var_deltemp_dn4))) / (assign13570_e18879 * assign13570_e18879))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13570_e18882, assign13570_e18882_d_n4,)
    }
};
            let assign13570_e18884: f64 = (locals.var_ptwgr_i * assign13570_e18883);
            (locals.var_ptwgr_t, locals.var_ptwgr_t_dn0, locals.var_ptwgr_t_dn2, locals.var_ptwgr_t_dn3, locals.var_ptwgr_t_dn4, locals.var_ptwgr_t_dn5, locals.var_ptwgr_t_dn6, locals.var_ptwgr_t_dn7, locals.var_ptwgr_t_dn8, locals.var_ptwgr_t_dn9, locals.var_ptwgr_t_dn10, locals.var_ptwgr_t_dn11, locals.var_ptwgr_t_dn12, locals.var_ptwgr_t_dn13, locals.var_ptwgr_t_dn14, ) = (assign13570_e18884, (locals.var_ptwgr_i_dn0 * assign13570_e18883), (locals.var_ptwgr_i_dn2 * assign13570_e18883), (locals.var_ptwgr_i_dn3 * assign13570_e18883), ((locals.var_ptwgr_i_dn4 * assign13570_e18883) + (locals.var_ptwgr_i * assign13570_e18883_d_n4)), (locals.var_ptwgr_i_dn5 * assign13570_e18883), (locals.var_ptwgr_i_dn6 * assign13570_e18883), (locals.var_ptwgr_i_dn7 * assign13570_e18883), (locals.var_ptwgr_i_dn8 * assign13570_e18883), (locals.var_ptwgr_i_dn9 * assign13570_e18883), (locals.var_ptwgr_i_dn10 * assign13570_e18883), (locals.var_ptwgr_i_dn11 * assign13570_e18883), (locals.var_ptwgr_i_dn12 * assign13570_e18883), (locals.var_ptwgr_i_dn13 * assign13570_e18883), (locals.var_ptwgr_i_dn14 * assign13570_e18883), );
        }

        let assign13580_e18891: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign13580_e18892: f64 = (1.0 + assign13580_e18891);
        let assign13580_e18894: f64 = (assign13580_e18892 - 1e-6);
        let assign13580_e18896: f64 = (-10000.0);
        let assign13580_e18898: f64 = (assign13580_e18896 * 0.001);
        let (assign13580_e18959, assign13580_e18959_d_n4,) = {
    if (!(assign13580_e18894 < assign13580_e18898)) {
        let assign13580_e18905: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign13580_e18906: f64 = (1.0 + assign13580_e18905);
        let assign13580_e18908: f64 = (assign13580_e18906 - 1e-6);
        let assign13580_e18912: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign13580_e18913: f64 = (1.0 + assign13580_e18912);
        let assign13580_e18915: f64 = (assign13580_e18913 - 1e-6);
        let assign13580_e18919: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign13580_e18920: f64 = (1.0 + assign13580_e18919);
        let assign13580_e18922: f64 = (assign13580_e18920 - 1e-6);
        let assign13580_e18923: f64 = (assign13580_e18915 * assign13580_e18922);
        let assign13580_e18926: f64 = (4.0 * 0.001);
        let assign13580_e18928: f64 = (assign13580_e18926 * 0.001);
        let assign13580_e18929: f64 = (assign13580_e18923 + assign13580_e18928);
        let assign13580_e18930: f64 = (assign13580_e18929).sqrt();
        let assign13580_e18931: f64 = (assign13580_e18908 + assign13580_e18930);
        let assign13580_e18932: f64 = (0.5 * assign13580_e18931);
        (assign13580_e18932, (0.5 * ((locals.var_a11_i * locals.var_deltemp_dn4) + ((((locals.var_a11_i * locals.var_deltemp_dn4) * assign13580_e18922) + (assign13580_e18915 * (locals.var_a11_i * locals.var_deltemp_dn4))) / (2.0 * assign13580_e18930)))),)
    } else {
        let assign13580_e18936: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign13580_e18937: f64 = (1.0 + assign13580_e18936);
        let assign13580_e18939: f64 = (assign13580_e18937 - 1e-6);
        let assign13580_e18941: f64 = (-10000.0);
        let assign13580_e18943: f64 = (assign13580_e18941 * 0.001);
        let (assign13580_e18958, assign13580_e18958_d_n4,) = {
            if (assign13580_e18939 < assign13580_e18943) {
                let assign13580_e18946: f64 = (-0.001);
                let assign13580_e18948: f64 = (assign13580_e18946 * 0.001);
                let assign13580_e18952: f64 = (locals.var_a11_i * locals.var_deltemp);
                let assign13580_e18953: f64 = (1.0 + assign13580_e18952);
                let assign13580_e18955: f64 = (assign13580_e18953 - 1e-6);
                let assign13580_e18956: f64 = (assign13580_e18948 / assign13580_e18955);
                (assign13580_e18956, (-((assign13580_e18948 * (locals.var_a11_i * locals.var_deltemp_dn4)) / (assign13580_e18955 * assign13580_e18955))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13580_e18958, assign13580_e18958_d_n4,)
    }
};
        let assign13580_e18960: f64 = (locals.var_a1_i * assign13580_e18959);
        (locals.var_a1_t, locals.var_a1_t_dn4, ) = (assign13580_e18960, (locals.var_a1_i * assign13580_e18959_d_n4), );

        let assign13590_e18965: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign13590_e18966: f64 = (1.0 + assign13590_e18965);
        let assign13590_e18968: f64 = (assign13590_e18966 - 1e-6);
        let assign13590_e18970: f64 = (-10000.0);
        let assign13590_e18972: f64 = (assign13590_e18970 * 0.001);
        let (assign13590_e19033, assign13590_e19033_d_n4,) = {
    if (!(assign13590_e18968 < assign13590_e18972)) {
        let assign13590_e18979: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign13590_e18980: f64 = (1.0 + assign13590_e18979);
        let assign13590_e18982: f64 = (assign13590_e18980 - 1e-6);
        let assign13590_e18986: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign13590_e18987: f64 = (1.0 + assign13590_e18986);
        let assign13590_e18989: f64 = (assign13590_e18987 - 1e-6);
        let assign13590_e18993: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign13590_e18994: f64 = (1.0 + assign13590_e18993);
        let assign13590_e18996: f64 = (assign13590_e18994 - 1e-6);
        let assign13590_e18997: f64 = (assign13590_e18989 * assign13590_e18996);
        let assign13590_e19000: f64 = (4.0 * 0.001);
        let assign13590_e19002: f64 = (assign13590_e19000 * 0.001);
        let assign13590_e19003: f64 = (assign13590_e18997 + assign13590_e19002);
        let assign13590_e19004: f64 = (assign13590_e19003).sqrt();
        let assign13590_e19005: f64 = (assign13590_e18982 + assign13590_e19004);
        let assign13590_e19006: f64 = (0.5 * assign13590_e19005);
        (assign13590_e19006, (0.5 * ((locals.var_a21_i * locals.var_deltemp_dn4) + ((((locals.var_a21_i * locals.var_deltemp_dn4) * assign13590_e18996) + (assign13590_e18989 * (locals.var_a21_i * locals.var_deltemp_dn4))) / (2.0 * assign13590_e19004)))),)
    } else {
        let assign13590_e19010: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign13590_e19011: f64 = (1.0 + assign13590_e19010);
        let assign13590_e19013: f64 = (assign13590_e19011 - 1e-6);
        let assign13590_e19015: f64 = (-10000.0);
        let assign13590_e19017: f64 = (assign13590_e19015 * 0.001);
        let (assign13590_e19032, assign13590_e19032_d_n4,) = {
            if (assign13590_e19013 < assign13590_e19017) {
                let assign13590_e19020: f64 = (-0.001);
                let assign13590_e19022: f64 = (assign13590_e19020 * 0.001);
                let assign13590_e19026: f64 = (locals.var_a21_i * locals.var_deltemp);
                let assign13590_e19027: f64 = (1.0 + assign13590_e19026);
                let assign13590_e19029: f64 = (assign13590_e19027 - 1e-6);
                let assign13590_e19030: f64 = (assign13590_e19022 / assign13590_e19029);
                (assign13590_e19030, (-((assign13590_e19022 * (locals.var_a21_i * locals.var_deltemp_dn4)) / (assign13590_e19029 * assign13590_e19029))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13590_e19032, assign13590_e19032_d_n4,)
    }
};
        let assign13590_e19034: f64 = (locals.var_a2_i * assign13590_e19033);
        (locals.var_a2_t, locals.var_a2_t_dn4, ) = (assign13590_e19034, (locals.var_a2_i * assign13590_e19033_d_n4), );

        let assign13600_e19038: f64 = (locals.var_tratio).powf(locals.var_iit_i);
        let assign13600_e19039: f64 = (locals.var_beta0_i * assign13600_e19038);
        (locals.var_beta0_t, locals.var_beta0_t_dn0, locals.var_beta0_t_dn2, locals.var_beta0_t_dn3, locals.var_beta0_t_dn4, locals.var_beta0_t_dn5, locals.var_beta0_t_dn6, locals.var_beta0_t_dn7, locals.var_beta0_t_dn8, locals.var_beta0_t_dn9, locals.var_beta0_t_dn10, locals.var_beta0_t_dn11, locals.var_beta0_t_dn12, locals.var_beta0_t_dn13, locals.var_beta0_t_dn14, ) = (assign13600_e19039, (locals.var_beta0_i_dn0 * assign13600_e19038), (locals.var_beta0_i_dn2 * assign13600_e19038), (locals.var_beta0_i_dn3 * assign13600_e19038), ((locals.var_beta0_i_dn4 * assign13600_e19038) + (locals.var_beta0_i * if 0.0 == 0.0 && ((locals.var_iit_i) as f64).is_finite() && ((locals.var_iit_i) as f64).fract() == 0.0 { if locals.var_iit_i == 0.0 { 0.0 } else { (locals.var_iit_i * ((locals.var_tratio).powf(locals.var_iit_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13600_e19038 * (locals.var_iit_i * (locals.var_tratio_dn4 / locals.var_tratio))) })), (locals.var_beta0_i_dn5 * assign13600_e19038), (locals.var_beta0_i_dn6 * assign13600_e19038), (locals.var_beta0_i_dn7 * assign13600_e19038), (locals.var_beta0_i_dn8 * assign13600_e19038), (locals.var_beta0_i_dn9 * assign13600_e19038), (locals.var_beta0_i_dn10 * assign13600_e19038), (locals.var_beta0_i_dn11 * assign13600_e19038), (locals.var_beta0_i_dn12 * assign13600_e19038), (locals.var_beta0_i_dn13 * assign13600_e19038), (locals.var_beta0_i_dn14 * assign13600_e19038), );

        let assign13610_e19042: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard461 = assign13610_e19042;

        if (locals.var_guard461 != 0.0) {
            let assign13620_e19047: f64 = (locals.var_tratio).powf(locals.var_iit_i);
            let assign13620_e19048: f64 = (locals.var_beta0r_i * assign13620_e19047);
            (locals.var_beta0r_t, locals.var_beta0r_t_dn4, ) = (assign13620_e19048, (locals.var_beta0r_i * if 0.0 == 0.0 && ((locals.var_iit_i) as f64).is_finite() && ((locals.var_iit_i) as f64).fract() == 0.0 { if locals.var_iit_i == 0.0 { 0.0 } else { (locals.var_iit_i * ((locals.var_tratio).powf(locals.var_iit_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign13620_e19047 * (locals.var_iit_i * (locals.var_tratio_dn4 / locals.var_tratio))) }), );
        }

        let assign13630_e19055: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13630_e19056: f64 = (1.0 + assign13630_e19055);
        let assign13630_e19058: f64 = (assign13630_e19056 - 1e-6);
        let assign13630_e19060: f64 = (-10000.0);
        let assign13630_e19062: f64 = (assign13630_e19060 * 0.001);
        let (assign13630_e19123, assign13630_e19123_d_n4,) = {
    if (!(assign13630_e19058 < assign13630_e19062)) {
        let assign13630_e19069: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13630_e19070: f64 = (1.0 + assign13630_e19069);
        let assign13630_e19072: f64 = (assign13630_e19070 - 1e-6);
        let assign13630_e19076: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13630_e19077: f64 = (1.0 + assign13630_e19076);
        let assign13630_e19079: f64 = (assign13630_e19077 - 1e-6);
        let assign13630_e19083: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13630_e19084: f64 = (1.0 + assign13630_e19083);
        let assign13630_e19086: f64 = (assign13630_e19084 - 1e-6);
        let assign13630_e19087: f64 = (assign13630_e19079 * assign13630_e19086);
        let assign13630_e19090: f64 = (4.0 * 0.001);
        let assign13630_e19092: f64 = (assign13630_e19090 * 0.001);
        let assign13630_e19093: f64 = (assign13630_e19087 + assign13630_e19092);
        let assign13630_e19094: f64 = (assign13630_e19093).sqrt();
        let assign13630_e19095: f64 = (assign13630_e19072 + assign13630_e19094);
        let assign13630_e19096: f64 = (0.5 * assign13630_e19095);
        (assign13630_e19096, (0.5 * ((locals.var_tgidl_i * locals.var_deltemp_dn4) + ((((locals.var_tgidl_i * locals.var_deltemp_dn4) * assign13630_e19086) + (assign13630_e19079 * (locals.var_tgidl_i * locals.var_deltemp_dn4))) / (2.0 * assign13630_e19094)))),)
    } else {
        let assign13630_e19100: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13630_e19101: f64 = (1.0 + assign13630_e19100);
        let assign13630_e19103: f64 = (assign13630_e19101 - 1e-6);
        let assign13630_e19105: f64 = (-10000.0);
        let assign13630_e19107: f64 = (assign13630_e19105 * 0.001);
        let (assign13630_e19122, assign13630_e19122_d_n4,) = {
            if (assign13630_e19103 < assign13630_e19107) {
                let assign13630_e19110: f64 = (-0.001);
                let assign13630_e19112: f64 = (assign13630_e19110 * 0.001);
                let assign13630_e19116: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign13630_e19117: f64 = (1.0 + assign13630_e19116);
                let assign13630_e19119: f64 = (assign13630_e19117 - 1e-6);
                let assign13630_e19120: f64 = (assign13630_e19112 / assign13630_e19119);
                (assign13630_e19120, (-((assign13630_e19112 * (locals.var_tgidl_i * locals.var_deltemp_dn4)) / (assign13630_e19119 * assign13630_e19119))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13630_e19122, assign13630_e19122_d_n4,)
    }
};
        let assign13630_e19124: f64 = (locals.var_bgidl_i * assign13630_e19123);
        (locals.var_bgidl_t, locals.var_bgidl_t_dn4, ) = (assign13630_e19124, (locals.var_bgidl_i * assign13630_e19123_d_n4), );

        let assign13640_e19129: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13640_e19130: f64 = (1.0 + assign13640_e19129);
        let assign13640_e19132: f64 = (assign13640_e19130 - 1e-6);
        let assign13640_e19134: f64 = (-10000.0);
        let assign13640_e19136: f64 = (assign13640_e19134 * 0.001);
        let (assign13640_e19197, assign13640_e19197_d_n4,) = {
    if (!(assign13640_e19132 < assign13640_e19136)) {
        let assign13640_e19143: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13640_e19144: f64 = (1.0 + assign13640_e19143);
        let assign13640_e19146: f64 = (assign13640_e19144 - 1e-6);
        let assign13640_e19150: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13640_e19151: f64 = (1.0 + assign13640_e19150);
        let assign13640_e19153: f64 = (assign13640_e19151 - 1e-6);
        let assign13640_e19157: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13640_e19158: f64 = (1.0 + assign13640_e19157);
        let assign13640_e19160: f64 = (assign13640_e19158 - 1e-6);
        let assign13640_e19161: f64 = (assign13640_e19153 * assign13640_e19160);
        let assign13640_e19164: f64 = (4.0 * 0.001);
        let assign13640_e19166: f64 = (assign13640_e19164 * 0.001);
        let assign13640_e19167: f64 = (assign13640_e19161 + assign13640_e19166);
        let assign13640_e19168: f64 = (assign13640_e19167).sqrt();
        let assign13640_e19169: f64 = (assign13640_e19146 + assign13640_e19168);
        let assign13640_e19170: f64 = (0.5 * assign13640_e19169);
        (assign13640_e19170, (0.5 * ((locals.var_tgidl_i * locals.var_deltemp_dn4) + ((((locals.var_tgidl_i * locals.var_deltemp_dn4) * assign13640_e19160) + (assign13640_e19153 * (locals.var_tgidl_i * locals.var_deltemp_dn4))) / (2.0 * assign13640_e19168)))),)
    } else {
        let assign13640_e19174: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign13640_e19175: f64 = (1.0 + assign13640_e19174);
        let assign13640_e19177: f64 = (assign13640_e19175 - 1e-6);
        let assign13640_e19179: f64 = (-10000.0);
        let assign13640_e19181: f64 = (assign13640_e19179 * 0.001);
        let (assign13640_e19196, assign13640_e19196_d_n4,) = {
            if (assign13640_e19177 < assign13640_e19181) {
                let assign13640_e19184: f64 = (-0.001);
                let assign13640_e19186: f64 = (assign13640_e19184 * 0.001);
                let assign13640_e19190: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign13640_e19191: f64 = (1.0 + assign13640_e19190);
                let assign13640_e19193: f64 = (assign13640_e19191 - 1e-6);
                let assign13640_e19194: f64 = (assign13640_e19186 / assign13640_e19193);
                (assign13640_e19194, (-((assign13640_e19186 * (locals.var_tgidl_i * locals.var_deltemp_dn4)) / (assign13640_e19193 * assign13640_e19193))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13640_e19196, assign13640_e19196_d_n4,)
    }
};
        let assign13640_e19198: f64 = (locals.var_bgisl_i * assign13640_e19197);
        (locals.var_bgisl_t, locals.var_bgisl_t_dn4, ) = (assign13640_e19198, (locals.var_bgisl_i * assign13640_e19197_d_n4), );

        let assign13650_e19202: f64 = (locals.var_tratio).max(1e-38);
        let assign13650_e19203: f64 = (assign13650_e19202).ln();
        let assign13650_e19204: f64 = (locals.var_igt_i * assign13650_e19203);
        let assign13650_e19205: f64 = { let limited_exp_arg = assign13650_e19204; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (locals.var_igtemp, locals.var_igtemp_dn4, ) = (assign13650_e19205, ({ let limited_exp_arg = assign13650_e19204; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_igt_i * (if locals.var_tratio >= 1e-38 { locals.var_tratio_dn4 } else { 0.0 } / assign13650_e19202))), );

        let assign13660_e19210: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign13660_e19211: f64 = (1.0 + assign13660_e19210);
        let assign13660_e19213: f64 = (assign13660_e19211 - 1e-6);
        let assign13660_e19215: f64 = (-10000.0);
        let assign13660_e19217: f64 = (assign13660_e19215 * 0.001);
        let (assign13660_e19278, assign13660_e19278_d_n4,) = {
    if (!(assign13660_e19213 < assign13660_e19217)) {
        let assign13660_e19224: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign13660_e19225: f64 = (1.0 + assign13660_e19224);
        let assign13660_e19227: f64 = (assign13660_e19225 - 1e-6);
        let assign13660_e19231: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign13660_e19232: f64 = (1.0 + assign13660_e19231);
        let assign13660_e19234: f64 = (assign13660_e19232 - 1e-6);
        let assign13660_e19238: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign13660_e19239: f64 = (1.0 + assign13660_e19238);
        let assign13660_e19241: f64 = (assign13660_e19239 - 1e-6);
        let assign13660_e19242: f64 = (assign13660_e19234 * assign13660_e19241);
        let assign13660_e19245: f64 = (4.0 * 0.001);
        let assign13660_e19247: f64 = (assign13660_e19245 * 0.001);
        let assign13660_e19248: f64 = (assign13660_e19242 + assign13660_e19247);
        let assign13660_e19249: f64 = (assign13660_e19248).sqrt();
        let assign13660_e19250: f64 = (assign13660_e19227 + assign13660_e19249);
        let assign13660_e19251: f64 = (0.5 * assign13660_e19250);
        (assign13660_e19251, (0.5 * ((locals.var_k01_i * locals.var_deltemp_dn4) + ((((locals.var_k01_i * locals.var_deltemp_dn4) * assign13660_e19241) + (assign13660_e19234 * (locals.var_k01_i * locals.var_deltemp_dn4))) / (2.0 * assign13660_e19249)))),)
    } else {
        let assign13660_e19255: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign13660_e19256: f64 = (1.0 + assign13660_e19255);
        let assign13660_e19258: f64 = (assign13660_e19256 - 1e-6);
        let assign13660_e19260: f64 = (-10000.0);
        let assign13660_e19262: f64 = (assign13660_e19260 * 0.001);
        let (assign13660_e19277, assign13660_e19277_d_n4,) = {
            if (assign13660_e19258 < assign13660_e19262) {
                let assign13660_e19265: f64 = (-0.001);
                let assign13660_e19267: f64 = (assign13660_e19265 * 0.001);
                let assign13660_e19271: f64 = (locals.var_k01_i * locals.var_deltemp);
                let assign13660_e19272: f64 = (1.0 + assign13660_e19271);
                let assign13660_e19274: f64 = (assign13660_e19272 - 1e-6);
                let assign13660_e19275: f64 = (assign13660_e19267 / assign13660_e19274);
                (assign13660_e19275, (-((assign13660_e19267 * (locals.var_k01_i * locals.var_deltemp_dn4)) / (assign13660_e19274 * assign13660_e19274))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13660_e19277, assign13660_e19277_d_n4,)
    }
};
        let assign13660_e19279: f64 = (locals.var_k0_i * assign13660_e19278);
        (locals.var_k0_t, locals.var_k0_t_dn4, ) = (assign13660_e19279, (locals.var_k0_i * assign13660_e19278_d_n4), );

        let assign13670_e19284: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign13670_e19285: f64 = (1.0 + assign13670_e19284);
        let assign13670_e19287: f64 = (assign13670_e19285 - 1e-6);
        let assign13670_e19289: f64 = (-10000.0);
        let assign13670_e19291: f64 = (assign13670_e19289 * 0.001);
        let (assign13670_e19352, assign13670_e19352_d_n4,) = {
    if (!(assign13670_e19287 < assign13670_e19291)) {
        let assign13670_e19298: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign13670_e19299: f64 = (1.0 + assign13670_e19298);
        let assign13670_e19301: f64 = (assign13670_e19299 - 1e-6);
        let assign13670_e19305: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign13670_e19306: f64 = (1.0 + assign13670_e19305);
        let assign13670_e19308: f64 = (assign13670_e19306 - 1e-6);
        let assign13670_e19312: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign13670_e19313: f64 = (1.0 + assign13670_e19312);
        let assign13670_e19315: f64 = (assign13670_e19313 - 1e-6);
        let assign13670_e19316: f64 = (assign13670_e19308 * assign13670_e19315);
        let assign13670_e19319: f64 = (4.0 * 0.001);
        let assign13670_e19321: f64 = (assign13670_e19319 * 0.001);
        let assign13670_e19322: f64 = (assign13670_e19316 + assign13670_e19321);
        let assign13670_e19323: f64 = (assign13670_e19322).sqrt();
        let assign13670_e19324: f64 = (assign13670_e19301 + assign13670_e19323);
        let assign13670_e19325: f64 = (0.5 * assign13670_e19324);
        (assign13670_e19325, (0.5 * ((locals.var_m01_i * locals.var_deltemp_dn4) + ((((locals.var_m01_i * locals.var_deltemp_dn4) * assign13670_e19315) + (assign13670_e19308 * (locals.var_m01_i * locals.var_deltemp_dn4))) / (2.0 * assign13670_e19323)))),)
    } else {
        let assign13670_e19329: f64 = (locals.var_m01_i * locals.var_deltemp);
        let assign13670_e19330: f64 = (1.0 + assign13670_e19329);
        let assign13670_e19332: f64 = (assign13670_e19330 - 1e-6);
        let assign13670_e19334: f64 = (-10000.0);
        let assign13670_e19336: f64 = (assign13670_e19334 * 0.001);
        let (assign13670_e19351, assign13670_e19351_d_n4,) = {
            if (assign13670_e19332 < assign13670_e19336) {
                let assign13670_e19339: f64 = (-0.001);
                let assign13670_e19341: f64 = (assign13670_e19339 * 0.001);
                let assign13670_e19345: f64 = (locals.var_m01_i * locals.var_deltemp);
                let assign13670_e19346: f64 = (1.0 + assign13670_e19345);
                let assign13670_e19348: f64 = (assign13670_e19346 - 1e-6);
                let assign13670_e19349: f64 = (assign13670_e19341 / assign13670_e19348);
                (assign13670_e19349, (-((assign13670_e19341 * (locals.var_m01_i * locals.var_deltemp_dn4)) / (assign13670_e19348 * assign13670_e19348))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13670_e19351, assign13670_e19351_d_n4,)
    }
};
        let assign13670_e19353: f64 = (locals.var_m0_i * assign13670_e19352);
        (locals.var_m0_t, locals.var_m0_t_dn4, ) = (assign13670_e19353, (locals.var_m0_i * assign13670_e19352_d_n4), );

    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13680_e19358: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign13680_e19359: f64 = (1.0 + assign13680_e19358);
        let assign13680_e19361: f64 = (assign13680_e19359 - 1e-6);
        let assign13680_e19363: f64 = (-10000.0);
        let assign13680_e19365: f64 = (assign13680_e19363 * 0.001);
        let (assign13680_e19426, assign13680_e19426_d_n4,) = {
    if (!(assign13680_e19361 < assign13680_e19365)) {
        let assign13680_e19372: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign13680_e19373: f64 = (1.0 + assign13680_e19372);
        let assign13680_e19375: f64 = (assign13680_e19373 - 1e-6);
        let assign13680_e19379: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign13680_e19380: f64 = (1.0 + assign13680_e19379);
        let assign13680_e19382: f64 = (assign13680_e19380 - 1e-6);
        let assign13680_e19386: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign13680_e19387: f64 = (1.0 + assign13680_e19386);
        let assign13680_e19389: f64 = (assign13680_e19387 - 1e-6);
        let assign13680_e19390: f64 = (assign13680_e19382 * assign13680_e19389);
        let assign13680_e19393: f64 = (4.0 * 0.001);
        let assign13680_e19395: f64 = (assign13680_e19393 * 0.001);
        let assign13680_e19396: f64 = (assign13680_e19390 + assign13680_e19395);
        let assign13680_e19397: f64 = (assign13680_e19396).sqrt();
        let assign13680_e19398: f64 = (assign13680_e19375 + assign13680_e19397);
        let assign13680_e19399: f64 = (0.5 * assign13680_e19398);
        (assign13680_e19399, (0.5 * ((locals.var_c01_i * locals.var_deltemp_dn4) + ((((locals.var_c01_i * locals.var_deltemp_dn4) * assign13680_e19389) + (assign13680_e19382 * (locals.var_c01_i * locals.var_deltemp_dn4))) / (2.0 * assign13680_e19397)))),)
    } else {
        let assign13680_e19403: f64 = (locals.var_c01_i * locals.var_deltemp);
        let assign13680_e19404: f64 = (1.0 + assign13680_e19403);
        let assign13680_e19406: f64 = (assign13680_e19404 - 1e-6);
        let assign13680_e19408: f64 = (-10000.0);
        let assign13680_e19410: f64 = (assign13680_e19408 * 0.001);
        let (assign13680_e19425, assign13680_e19425_d_n4,) = {
            if (assign13680_e19406 < assign13680_e19410) {
                let assign13680_e19413: f64 = (-0.001);
                let assign13680_e19415: f64 = (assign13680_e19413 * 0.001);
                let assign13680_e19419: f64 = (locals.var_c01_i * locals.var_deltemp);
                let assign13680_e19420: f64 = (1.0 + assign13680_e19419);
                let assign13680_e19422: f64 = (assign13680_e19420 - 1e-6);
                let assign13680_e19423: f64 = (assign13680_e19415 / assign13680_e19422);
                (assign13680_e19423, (-((assign13680_e19415 * (locals.var_c01_i * locals.var_deltemp_dn4)) / (assign13680_e19422 * assign13680_e19422))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13680_e19425, assign13680_e19425_d_n4,)
    }
};
        let assign13680_e19427: f64 = (locals.var_c0_i * assign13680_e19426);
        (locals.var_c0_t, locals.var_c0_t_dn4, ) = (assign13680_e19427, (locals.var_c0_i * assign13680_e19426_d_n4), );

        let assign13690_e19432: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19433: f64 = (1.0 + assign13690_e19432);
        let assign13690_e19435: f64 = (assign13690_e19433 - 1e-6);
        let assign13690_e19437: f64 = (-10000.0);
        let assign13690_e19439: f64 = (assign13690_e19437 * 0.001);
        let (assign13690_e19500, assign13690_e19500_d_n4,) = {
    if (!(assign13690_e19435 < assign13690_e19439)) {
        let assign13690_e19446: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19447: f64 = (1.0 + assign13690_e19446);
        let assign13690_e19449: f64 = (assign13690_e19447 - 1e-6);
        let assign13690_e19453: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19454: f64 = (1.0 + assign13690_e19453);
        let assign13690_e19456: f64 = (assign13690_e19454 - 1e-6);
        let assign13690_e19460: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19461: f64 = (1.0 + assign13690_e19460);
        let assign13690_e19463: f64 = (assign13690_e19461 - 1e-6);
        let assign13690_e19464: f64 = (assign13690_e19456 * assign13690_e19463);
        let assign13690_e19467: f64 = (4.0 * 0.001);
        let assign13690_e19469: f64 = (assign13690_e19467 * 0.001);
        let assign13690_e19470: f64 = (assign13690_e19464 + assign13690_e19469);
        let assign13690_e19471: f64 = (assign13690_e19470).sqrt();
        let assign13690_e19472: f64 = (assign13690_e19449 + assign13690_e19471);
        let assign13690_e19473: f64 = (0.5 * assign13690_e19472);
        (assign13690_e19473, (0.5 * ((locals.var_c0si1_i * locals.var_deltemp_dn4) + ((((locals.var_c0si1_i * locals.var_deltemp_dn4) * assign13690_e19463) + (assign13690_e19456 * (locals.var_c0si1_i * locals.var_deltemp_dn4))) / (2.0 * assign13690_e19471)))),)
    } else {
        let assign13690_e19477: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19478: f64 = (1.0 + assign13690_e19477);
        let assign13690_e19480: f64 = (assign13690_e19478 - 1e-6);
        let assign13690_e19482: f64 = (-10000.0);
        let assign13690_e19484: f64 = (assign13690_e19482 * 0.001);
        let (assign13690_e19499, assign13690_e19499_d_n4,) = {
            if (assign13690_e19480 < assign13690_e19484) {
                let assign13690_e19487: f64 = (-0.001);
                let assign13690_e19489: f64 = (assign13690_e19487 * 0.001);
                let assign13690_e19493: f64 = (locals.var_c0si1_i * locals.var_deltemp);
                let assign13690_e19494: f64 = (1.0 + assign13690_e19493);
                let assign13690_e19496: f64 = (assign13690_e19494 - 1e-6);
                let assign13690_e19497: f64 = (assign13690_e19489 / assign13690_e19496);
                (assign13690_e19497, (-((assign13690_e19489 * (locals.var_c0si1_i * locals.var_deltemp_dn4)) / (assign13690_e19496 * assign13690_e19496))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13690_e19499, assign13690_e19499_d_n4,)
    }
};
        let assign13690_e19501: f64 = (locals.var_c0si_i * assign13690_e19500);
        (locals.var_c0si_t, locals.var_c0si_t_dn4, ) = (assign13690_e19501, (locals.var_c0si_i * assign13690_e19500_d_n4), );

        let assign13700_e19506: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19507: f64 = (1.0 + assign13700_e19506);
        let assign13700_e19509: f64 = (assign13700_e19507 - 1e-6);
        let assign13700_e19511: f64 = (-10000.0);
        let assign13700_e19513: f64 = (assign13700_e19511 * 0.001);
        let (assign13700_e19574, assign13700_e19574_d_n4,) = {
    if (!(assign13700_e19509 < assign13700_e19513)) {
        let assign13700_e19520: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19521: f64 = (1.0 + assign13700_e19520);
        let assign13700_e19523: f64 = (assign13700_e19521 - 1e-6);
        let assign13700_e19527: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19528: f64 = (1.0 + assign13700_e19527);
        let assign13700_e19530: f64 = (assign13700_e19528 - 1e-6);
        let assign13700_e19534: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19535: f64 = (1.0 + assign13700_e19534);
        let assign13700_e19537: f64 = (assign13700_e19535 - 1e-6);
        let assign13700_e19538: f64 = (assign13700_e19530 * assign13700_e19537);
        let assign13700_e19541: f64 = (4.0 * 0.001);
        let assign13700_e19543: f64 = (assign13700_e19541 * 0.001);
        let assign13700_e19544: f64 = (assign13700_e19538 + assign13700_e19543);
        let assign13700_e19545: f64 = (assign13700_e19544).sqrt();
        let assign13700_e19546: f64 = (assign13700_e19523 + assign13700_e19545);
        let assign13700_e19547: f64 = (0.5 * assign13700_e19546);
        (assign13700_e19547, (0.5 * ((locals.var_c0sisat1_i * locals.var_deltemp_dn4) + ((((locals.var_c0sisat1_i * locals.var_deltemp_dn4) * assign13700_e19537) + (assign13700_e19530 * (locals.var_c0sisat1_i * locals.var_deltemp_dn4))) / (2.0 * assign13700_e19545)))),)
    } else {
        let assign13700_e19551: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19552: f64 = (1.0 + assign13700_e19551);
        let assign13700_e19554: f64 = (assign13700_e19552 - 1e-6);
        let assign13700_e19556: f64 = (-10000.0);
        let assign13700_e19558: f64 = (assign13700_e19556 * 0.001);
        let (assign13700_e19573, assign13700_e19573_d_n4,) = {
            if (assign13700_e19554 < assign13700_e19558) {
                let assign13700_e19561: f64 = (-0.001);
                let assign13700_e19563: f64 = (assign13700_e19561 * 0.001);
                let assign13700_e19567: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
                let assign13700_e19568: f64 = (1.0 + assign13700_e19567);
                let assign13700_e19570: f64 = (assign13700_e19568 - 1e-6);
                let assign13700_e19571: f64 = (assign13700_e19563 / assign13700_e19570);
                (assign13700_e19571, (-((assign13700_e19563 * (locals.var_c0sisat1_i * locals.var_deltemp_dn4)) / (assign13700_e19570 * assign13700_e19570))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13700_e19573, assign13700_e19573_d_n4,)
    }
};
        let assign13700_e19575: f64 = (locals.var_c0sisat_i * assign13700_e19574);
        (locals.var_c0sisat_t, locals.var_c0sisat_t_dn4, ) = (assign13700_e19575, (locals.var_c0sisat_i * assign13700_e19574_d_n4), );

        let assign13710_e19580: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19581: f64 = (1.0 + assign13710_e19580);
        let assign13710_e19583: f64 = (assign13710_e19581 - 1e-6);
        let assign13710_e19585: f64 = (-10000.0);
        let assign13710_e19587: f64 = (assign13710_e19585 * 0.001);
        let (assign13710_e19648, assign13710_e19648_d_n4,) = {
    if (!(assign13710_e19583 < assign13710_e19587)) {
        let assign13710_e19594: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19595: f64 = (1.0 + assign13710_e19594);
        let assign13710_e19597: f64 = (assign13710_e19595 - 1e-6);
        let assign13710_e19601: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19602: f64 = (1.0 + assign13710_e19601);
        let assign13710_e19604: f64 = (assign13710_e19602 - 1e-6);
        let assign13710_e19608: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19609: f64 = (1.0 + assign13710_e19608);
        let assign13710_e19611: f64 = (assign13710_e19609 - 1e-6);
        let assign13710_e19612: f64 = (assign13710_e19604 * assign13710_e19611);
        let assign13710_e19615: f64 = (4.0 * 0.001);
        let assign13710_e19617: f64 = (assign13710_e19615 * 0.001);
        let assign13710_e19618: f64 = (assign13710_e19612 + assign13710_e19617);
        let assign13710_e19619: f64 = (assign13710_e19618).sqrt();
        let assign13710_e19620: f64 = (assign13710_e19597 + assign13710_e19619);
        let assign13710_e19621: f64 = (0.5 * assign13710_e19620);
        (assign13710_e19621, (0.5 * ((p.p889 * locals.var_deltemp_dn4) + ((((p.p889 * locals.var_deltemp_dn4) * assign13710_e19611) + (assign13710_e19604 * (p.p889 * locals.var_deltemp_dn4))) / (2.0 * assign13710_e19619)))),)
    } else {
        let assign13710_e19625: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19626: f64 = (1.0 + assign13710_e19625);
        let assign13710_e19628: f64 = (assign13710_e19626 - 1e-6);
        let assign13710_e19630: f64 = (-10000.0);
        let assign13710_e19632: f64 = (assign13710_e19630 * 0.001);
        let (assign13710_e19647, assign13710_e19647_d_n4,) = {
            if (assign13710_e19628 < assign13710_e19632) {
                let assign13710_e19635: f64 = (-0.001);
                let assign13710_e19637: f64 = (assign13710_e19635 * 0.001);
                let assign13710_e19641: f64 = (p.p889 * locals.var_deltemp);
                let assign13710_e19642: f64 = (1.0 + assign13710_e19641);
                let assign13710_e19644: f64 = (assign13710_e19642 - 1e-6);
                let assign13710_e19645: f64 = (assign13710_e19637 / assign13710_e19644);
                (assign13710_e19645, (-((assign13710_e19637 * (p.p889 * locals.var_deltemp_dn4)) / (assign13710_e19644 * assign13710_e19644))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13710_e19647, assign13710_e19647_d_n4,)
    }
};
        let assign13710_e19649: f64 = (p.p701 * assign13710_e19648);
        (locals.var_cjs_t, locals.var_cjs_t_dn4, ) = (assign13710_e19649, (p.p701 * assign13710_e19648_d_n4), );

        let assign13720_e19654: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19655: f64 = (1.0 + assign13720_e19654);
        let assign13720_e19657: f64 = (assign13720_e19655 - 1e-6);
        let assign13720_e19659: f64 = (-10000.0);
        let assign13720_e19661: f64 = (assign13720_e19659 * 0.001);
        let (assign13720_e19722, assign13720_e19722_d_n4,) = {
    if (!(assign13720_e19657 < assign13720_e19661)) {
        let assign13720_e19668: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19669: f64 = (1.0 + assign13720_e19668);
        let assign13720_e19671: f64 = (assign13720_e19669 - 1e-6);
        let assign13720_e19675: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19676: f64 = (1.0 + assign13720_e19675);
        let assign13720_e19678: f64 = (assign13720_e19676 - 1e-6);
        let assign13720_e19682: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19683: f64 = (1.0 + assign13720_e19682);
        let assign13720_e19685: f64 = (assign13720_e19683 - 1e-6);
        let assign13720_e19686: f64 = (assign13720_e19678 * assign13720_e19685);
        let assign13720_e19689: f64 = (4.0 * 0.001);
        let assign13720_e19691: f64 = (assign13720_e19689 * 0.001);
        let assign13720_e19692: f64 = (assign13720_e19686 + assign13720_e19691);
        let assign13720_e19693: f64 = (assign13720_e19692).sqrt();
        let assign13720_e19694: f64 = (assign13720_e19671 + assign13720_e19693);
        let assign13720_e19695: f64 = (0.5 * assign13720_e19694);
        (assign13720_e19695, (0.5 * ((p.p889 * locals.var_deltemp_dn4) + ((((p.p889 * locals.var_deltemp_dn4) * assign13720_e19685) + (assign13720_e19678 * (p.p889 * locals.var_deltemp_dn4))) / (2.0 * assign13720_e19693)))),)
    } else {
        let assign13720_e19699: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19700: f64 = (1.0 + assign13720_e19699);
        let assign13720_e19702: f64 = (assign13720_e19700 - 1e-6);
        let assign13720_e19704: f64 = (-10000.0);
        let assign13720_e19706: f64 = (assign13720_e19704 * 0.001);
        let (assign13720_e19721, assign13720_e19721_d_n4,) = {
            if (assign13720_e19702 < assign13720_e19706) {
                let assign13720_e19709: f64 = (-0.001);
                let assign13720_e19711: f64 = (assign13720_e19709 * 0.001);
                let assign13720_e19715: f64 = (p.p889 * locals.var_deltemp);
                let assign13720_e19716: f64 = (1.0 + assign13720_e19715);
                let assign13720_e19718: f64 = (assign13720_e19716 - 1e-6);
                let assign13720_e19719: f64 = (assign13720_e19711 / assign13720_e19718);
                (assign13720_e19719, (-((assign13720_e19711 * (p.p889 * locals.var_deltemp_dn4)) / (assign13720_e19718 * assign13720_e19718))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13720_e19721, assign13720_e19721_d_n4,)
    }
};
        let assign13720_e19723: f64 = (p.p702 * assign13720_e19722);
        (locals.var_cjd_t, locals.var_cjd_t_dn4, ) = (assign13720_e19723, (p.p702 * assign13720_e19722_d_n4), );

        let assign13730_e19728: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19729: f64 = (1.0 + assign13730_e19728);
        let assign13730_e19731: f64 = (assign13730_e19729 - 1e-6);
        let assign13730_e19733: f64 = (-10000.0);
        let assign13730_e19735: f64 = (assign13730_e19733 * 0.001);
        let (assign13730_e19796, assign13730_e19796_d_n4,) = {
    if (!(assign13730_e19731 < assign13730_e19735)) {
        let assign13730_e19742: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19743: f64 = (1.0 + assign13730_e19742);
        let assign13730_e19745: f64 = (assign13730_e19743 - 1e-6);
        let assign13730_e19749: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19750: f64 = (1.0 + assign13730_e19749);
        let assign13730_e19752: f64 = (assign13730_e19750 - 1e-6);
        let assign13730_e19756: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19757: f64 = (1.0 + assign13730_e19756);
        let assign13730_e19759: f64 = (assign13730_e19757 - 1e-6);
        let assign13730_e19760: f64 = (assign13730_e19752 * assign13730_e19759);
        let assign13730_e19763: f64 = (4.0 * 0.001);
        let assign13730_e19765: f64 = (assign13730_e19763 * 0.001);
        let assign13730_e19766: f64 = (assign13730_e19760 + assign13730_e19765);
        let assign13730_e19767: f64 = (assign13730_e19766).sqrt();
        let assign13730_e19768: f64 = (assign13730_e19745 + assign13730_e19767);
        let assign13730_e19769: f64 = (0.5 * assign13730_e19768);
        (assign13730_e19769, (0.5 * ((p.p890 * locals.var_deltemp_dn4) + ((((p.p890 * locals.var_deltemp_dn4) * assign13730_e19759) + (assign13730_e19752 * (p.p890 * locals.var_deltemp_dn4))) / (2.0 * assign13730_e19767)))),)
    } else {
        let assign13730_e19773: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19774: f64 = (1.0 + assign13730_e19773);
        let assign13730_e19776: f64 = (assign13730_e19774 - 1e-6);
        let assign13730_e19778: f64 = (-10000.0);
        let assign13730_e19780: f64 = (assign13730_e19778 * 0.001);
        let (assign13730_e19795, assign13730_e19795_d_n4,) = {
            if (assign13730_e19776 < assign13730_e19780) {
                let assign13730_e19783: f64 = (-0.001);
                let assign13730_e19785: f64 = (assign13730_e19783 * 0.001);
                let assign13730_e19789: f64 = (p.p890 * locals.var_deltemp);
                let assign13730_e19790: f64 = (1.0 + assign13730_e19789);
                let assign13730_e19792: f64 = (assign13730_e19790 - 1e-6);
                let assign13730_e19793: f64 = (assign13730_e19785 / assign13730_e19792);
                (assign13730_e19793, (-((assign13730_e19785 * (p.p890 * locals.var_deltemp_dn4)) / (assign13730_e19792 * assign13730_e19792))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13730_e19795, assign13730_e19795_d_n4,)
    }
};
        let assign13730_e19797: f64 = (p.p703 * assign13730_e19796);
        (locals.var_cjsws_t, locals.var_cjsws_t_dn4, ) = (assign13730_e19797, (p.p703 * assign13730_e19796_d_n4), );

        let assign13740_e19802: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19803: f64 = (1.0 + assign13740_e19802);
        let assign13740_e19805: f64 = (assign13740_e19803 - 1e-6);
        let assign13740_e19807: f64 = (-10000.0);
        let assign13740_e19809: f64 = (assign13740_e19807 * 0.001);
        let (assign13740_e19870, assign13740_e19870_d_n4,) = {
    if (!(assign13740_e19805 < assign13740_e19809)) {
        let assign13740_e19816: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19817: f64 = (1.0 + assign13740_e19816);
        let assign13740_e19819: f64 = (assign13740_e19817 - 1e-6);
        let assign13740_e19823: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19824: f64 = (1.0 + assign13740_e19823);
        let assign13740_e19826: f64 = (assign13740_e19824 - 1e-6);
        let assign13740_e19830: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19831: f64 = (1.0 + assign13740_e19830);
        let assign13740_e19833: f64 = (assign13740_e19831 - 1e-6);
        let assign13740_e19834: f64 = (assign13740_e19826 * assign13740_e19833);
        let assign13740_e19837: f64 = (4.0 * 0.001);
        let assign13740_e19839: f64 = (assign13740_e19837 * 0.001);
        let assign13740_e19840: f64 = (assign13740_e19834 + assign13740_e19839);
        let assign13740_e19841: f64 = (assign13740_e19840).sqrt();
        let assign13740_e19842: f64 = (assign13740_e19819 + assign13740_e19841);
        let assign13740_e19843: f64 = (0.5 * assign13740_e19842);
        (assign13740_e19843, (0.5 * ((p.p890 * locals.var_deltemp_dn4) + ((((p.p890 * locals.var_deltemp_dn4) * assign13740_e19833) + (assign13740_e19826 * (p.p890 * locals.var_deltemp_dn4))) / (2.0 * assign13740_e19841)))),)
    } else {
        let assign13740_e19847: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19848: f64 = (1.0 + assign13740_e19847);
        let assign13740_e19850: f64 = (assign13740_e19848 - 1e-6);
        let assign13740_e19852: f64 = (-10000.0);
        let assign13740_e19854: f64 = (assign13740_e19852 * 0.001);
        let (assign13740_e19869, assign13740_e19869_d_n4,) = {
            if (assign13740_e19850 < assign13740_e19854) {
                let assign13740_e19857: f64 = (-0.001);
                let assign13740_e19859: f64 = (assign13740_e19857 * 0.001);
                let assign13740_e19863: f64 = (p.p890 * locals.var_deltemp);
                let assign13740_e19864: f64 = (1.0 + assign13740_e19863);
                let assign13740_e19866: f64 = (assign13740_e19864 - 1e-6);
                let assign13740_e19867: f64 = (assign13740_e19859 / assign13740_e19866);
                (assign13740_e19867, (-((assign13740_e19859 * (p.p890 * locals.var_deltemp_dn4)) / (assign13740_e19866 * assign13740_e19866))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13740_e19869, assign13740_e19869_d_n4,)
    }
};
        let assign13740_e19871: f64 = (p.p704 * assign13740_e19870);
        (locals.var_cjswd_t, locals.var_cjswd_t_dn4, ) = (assign13740_e19871, (p.p704 * assign13740_e19870_d_n4), );

        let assign13750_e19876: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19877: f64 = (1.0 + assign13750_e19876);
        let assign13750_e19879: f64 = (assign13750_e19877 - 1e-6);
        let assign13750_e19881: f64 = (-10000.0);
        let assign13750_e19883: f64 = (assign13750_e19881 * 0.001);
        let (assign13750_e19944, assign13750_e19944_d_n4,) = {
    if (!(assign13750_e19879 < assign13750_e19883)) {
        let assign13750_e19890: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19891: f64 = (1.0 + assign13750_e19890);
        let assign13750_e19893: f64 = (assign13750_e19891 - 1e-6);
        let assign13750_e19897: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19898: f64 = (1.0 + assign13750_e19897);
        let assign13750_e19900: f64 = (assign13750_e19898 - 1e-6);
        let assign13750_e19904: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19905: f64 = (1.0 + assign13750_e19904);
        let assign13750_e19907: f64 = (assign13750_e19905 - 1e-6);
        let assign13750_e19908: f64 = (assign13750_e19900 * assign13750_e19907);
        let assign13750_e19911: f64 = (4.0 * 0.001);
        let assign13750_e19913: f64 = (assign13750_e19911 * 0.001);
        let assign13750_e19914: f64 = (assign13750_e19908 + assign13750_e19913);
        let assign13750_e19915: f64 = (assign13750_e19914).sqrt();
        let assign13750_e19916: f64 = (assign13750_e19893 + assign13750_e19915);
        let assign13750_e19917: f64 = (0.5 * assign13750_e19916);
        (assign13750_e19917, (0.5 * ((p.p891 * locals.var_deltemp_dn4) + ((((p.p891 * locals.var_deltemp_dn4) * assign13750_e19907) + (assign13750_e19900 * (p.p891 * locals.var_deltemp_dn4))) / (2.0 * assign13750_e19915)))),)
    } else {
        let assign13750_e19921: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19922: f64 = (1.0 + assign13750_e19921);
        let assign13750_e19924: f64 = (assign13750_e19922 - 1e-6);
        let assign13750_e19926: f64 = (-10000.0);
        let assign13750_e19928: f64 = (assign13750_e19926 * 0.001);
        let (assign13750_e19943, assign13750_e19943_d_n4,) = {
            if (assign13750_e19924 < assign13750_e19928) {
                let assign13750_e19931: f64 = (-0.001);
                let assign13750_e19933: f64 = (assign13750_e19931 * 0.001);
                let assign13750_e19937: f64 = (p.p891 * locals.var_deltemp);
                let assign13750_e19938: f64 = (1.0 + assign13750_e19937);
                let assign13750_e19940: f64 = (assign13750_e19938 - 1e-6);
                let assign13750_e19941: f64 = (assign13750_e19933 / assign13750_e19940);
                (assign13750_e19941, (-((assign13750_e19933 * (p.p891 * locals.var_deltemp_dn4)) / (assign13750_e19940 * assign13750_e19940))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13750_e19943, assign13750_e19943_d_n4,)
    }
};
        let assign13750_e19945: f64 = (p.p705 * assign13750_e19944);
        (locals.var_cjswgs_t, locals.var_cjswgs_t_dn4, ) = (assign13750_e19945, (p.p705 * assign13750_e19944_d_n4), );

        let assign13760_e19950: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19951: f64 = (1.0 + assign13760_e19950);
        let assign13760_e19953: f64 = (assign13760_e19951 - 1e-6);
        let assign13760_e19955: f64 = (-10000.0);
        let assign13760_e19957: f64 = (assign13760_e19955 * 0.001);
        let (assign13760_e20018, assign13760_e20018_d_n4,) = {
    if (!(assign13760_e19953 < assign13760_e19957)) {
        let assign13760_e19964: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19965: f64 = (1.0 + assign13760_e19964);
        let assign13760_e19967: f64 = (assign13760_e19965 - 1e-6);
        let assign13760_e19971: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19972: f64 = (1.0 + assign13760_e19971);
        let assign13760_e19974: f64 = (assign13760_e19972 - 1e-6);
        let assign13760_e19978: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19979: f64 = (1.0 + assign13760_e19978);
        let assign13760_e19981: f64 = (assign13760_e19979 - 1e-6);
        let assign13760_e19982: f64 = (assign13760_e19974 * assign13760_e19981);
        let assign13760_e19985: f64 = (4.0 * 0.001);
        let assign13760_e19987: f64 = (assign13760_e19985 * 0.001);
        let assign13760_e19988: f64 = (assign13760_e19982 + assign13760_e19987);
        let assign13760_e19989: f64 = (assign13760_e19988).sqrt();
        let assign13760_e19990: f64 = (assign13760_e19967 + assign13760_e19989);
        let assign13760_e19991: f64 = (0.5 * assign13760_e19990);
        (assign13760_e19991, (0.5 * ((p.p891 * locals.var_deltemp_dn4) + ((((p.p891 * locals.var_deltemp_dn4) * assign13760_e19981) + (assign13760_e19974 * (p.p891 * locals.var_deltemp_dn4))) / (2.0 * assign13760_e19989)))),)
    } else {
        let assign13760_e19995: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19996: f64 = (1.0 + assign13760_e19995);
        let assign13760_e19998: f64 = (assign13760_e19996 - 1e-6);
        let assign13760_e20000: f64 = (-10000.0);
        let assign13760_e20002: f64 = (assign13760_e20000 * 0.001);
        let (assign13760_e20017, assign13760_e20017_d_n4,) = {
            if (assign13760_e19998 < assign13760_e20002) {
                let assign13760_e20005: f64 = (-0.001);
                let assign13760_e20007: f64 = (assign13760_e20005 * 0.001);
                let assign13760_e20011: f64 = (p.p891 * locals.var_deltemp);
                let assign13760_e20012: f64 = (1.0 + assign13760_e20011);
                let assign13760_e20014: f64 = (assign13760_e20012 - 1e-6);
                let assign13760_e20015: f64 = (assign13760_e20007 / assign13760_e20014);
                (assign13760_e20015, (-((assign13760_e20007 * (p.p891 * locals.var_deltemp_dn4)) / (assign13760_e20014 * assign13760_e20014))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13760_e20017, assign13760_e20017_d_n4,)
    }
};
        let assign13760_e20019: f64 = (p.p706 * assign13760_e20018);
        (locals.var_cjswgd_t, locals.var_cjswgd_t_dn4, ) = (assign13760_e20019, (p.p706 * assign13760_e20018_d_n4), );

        let assign13770_e20023: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20024: f64 = (p.p707 - assign13770_e20023);
        let assign13770_e20026: f64 = (assign13770_e20024 - 0.01);
        let assign13770_e20028: f64 = (-10000.0);
        let assign13770_e20030: f64 = (assign13770_e20028 * 0.001);
        let (assign13770_e20091, assign13770_e20091_d_n4,) = {
    if (!(assign13770_e20026 < assign13770_e20030)) {
        let assign13770_e20037: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20038: f64 = (p.p707 - assign13770_e20037);
        let assign13770_e20040: f64 = (assign13770_e20038 - 0.01);
        let assign13770_e20044: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20045: f64 = (p.p707 - assign13770_e20044);
        let assign13770_e20047: f64 = (assign13770_e20045 - 0.01);
        let assign13770_e20051: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20052: f64 = (p.p707 - assign13770_e20051);
        let assign13770_e20054: f64 = (assign13770_e20052 - 0.01);
        let assign13770_e20055: f64 = (assign13770_e20047 * assign13770_e20054);
        let assign13770_e20058: f64 = (4.0 * 0.001);
        let assign13770_e20060: f64 = (assign13770_e20058 * 0.001);
        let assign13770_e20061: f64 = (assign13770_e20055 + assign13770_e20060);
        let assign13770_e20062: f64 = (assign13770_e20061).sqrt();
        let assign13770_e20063: f64 = (assign13770_e20040 + assign13770_e20062);
        let assign13770_e20064: f64 = (0.5 * assign13770_e20063);
        (assign13770_e20064, (0.5 * ((-(p.p892 * locals.var_deltemp_dn4)) + ((((-(p.p892 * locals.var_deltemp_dn4)) * assign13770_e20054) + (assign13770_e20047 * (-(p.p892 * locals.var_deltemp_dn4)))) / (2.0 * assign13770_e20062)))),)
    } else {
        let assign13770_e20068: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20069: f64 = (p.p707 - assign13770_e20068);
        let assign13770_e20071: f64 = (assign13770_e20069 - 0.01);
        let assign13770_e20073: f64 = (-10000.0);
        let assign13770_e20075: f64 = (assign13770_e20073 * 0.001);
        let (assign13770_e20090, assign13770_e20090_d_n4,) = {
            if (assign13770_e20071 < assign13770_e20075) {
                let assign13770_e20078: f64 = (-0.001);
                let assign13770_e20080: f64 = (assign13770_e20078 * 0.001);
                let assign13770_e20084: f64 = (p.p892 * locals.var_deltemp);
                let assign13770_e20085: f64 = (p.p707 - assign13770_e20084);
                let assign13770_e20087: f64 = (assign13770_e20085 - 0.01);
                let assign13770_e20088: f64 = (assign13770_e20080 / assign13770_e20087);
                (assign13770_e20088, (-((assign13770_e20080 * (-(p.p892 * locals.var_deltemp_dn4))) / (assign13770_e20087 * assign13770_e20087))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13770_e20090, assign13770_e20090_d_n4,)
    }
};
        let assign13770_e20093: f64 = (assign13770_e20091 + 0.01);
        (locals.var_pbs_t, locals.var_pbs_t_dn4, ) = (assign13770_e20093, assign13770_e20091_d_n4, );

    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13780_e20097: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20098: f64 = (p.p708 - assign13780_e20097);
        let assign13780_e20100: f64 = (assign13780_e20098 - 0.01);
        let assign13780_e20102: f64 = (-10000.0);
        let assign13780_e20104: f64 = (assign13780_e20102 * 0.001);
        let (assign13780_e20165, assign13780_e20165_d_n4,) = {
    if (!(assign13780_e20100 < assign13780_e20104)) {
        let assign13780_e20111: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20112: f64 = (p.p708 - assign13780_e20111);
        let assign13780_e20114: f64 = (assign13780_e20112 - 0.01);
        let assign13780_e20118: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20119: f64 = (p.p708 - assign13780_e20118);
        let assign13780_e20121: f64 = (assign13780_e20119 - 0.01);
        let assign13780_e20125: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20126: f64 = (p.p708 - assign13780_e20125);
        let assign13780_e20128: f64 = (assign13780_e20126 - 0.01);
        let assign13780_e20129: f64 = (assign13780_e20121 * assign13780_e20128);
        let assign13780_e20132: f64 = (4.0 * 0.001);
        let assign13780_e20134: f64 = (assign13780_e20132 * 0.001);
        let assign13780_e20135: f64 = (assign13780_e20129 + assign13780_e20134);
        let assign13780_e20136: f64 = (assign13780_e20135).sqrt();
        let assign13780_e20137: f64 = (assign13780_e20114 + assign13780_e20136);
        let assign13780_e20138: f64 = (0.5 * assign13780_e20137);
        (assign13780_e20138, (0.5 * ((-(p.p892 * locals.var_deltemp_dn4)) + ((((-(p.p892 * locals.var_deltemp_dn4)) * assign13780_e20128) + (assign13780_e20121 * (-(p.p892 * locals.var_deltemp_dn4)))) / (2.0 * assign13780_e20136)))),)
    } else {
        let assign13780_e20142: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20143: f64 = (p.p708 - assign13780_e20142);
        let assign13780_e20145: f64 = (assign13780_e20143 - 0.01);
        let assign13780_e20147: f64 = (-10000.0);
        let assign13780_e20149: f64 = (assign13780_e20147 * 0.001);
        let (assign13780_e20164, assign13780_e20164_d_n4,) = {
            if (assign13780_e20145 < assign13780_e20149) {
                let assign13780_e20152: f64 = (-0.001);
                let assign13780_e20154: f64 = (assign13780_e20152 * 0.001);
                let assign13780_e20158: f64 = (p.p892 * locals.var_deltemp);
                let assign13780_e20159: f64 = (p.p708 - assign13780_e20158);
                let assign13780_e20161: f64 = (assign13780_e20159 - 0.01);
                let assign13780_e20162: f64 = (assign13780_e20154 / assign13780_e20161);
                (assign13780_e20162, (-((assign13780_e20154 * (-(p.p892 * locals.var_deltemp_dn4))) / (assign13780_e20161 * assign13780_e20161))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13780_e20164, assign13780_e20164_d_n4,)
    }
};
        let assign13780_e20167: f64 = (assign13780_e20165 + 0.01);
        (locals.var_pbd_t, locals.var_pbd_t_dn4, ) = (assign13780_e20167, assign13780_e20165_d_n4, );

        let assign13790_e20171: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20172: f64 = (p.p709 - assign13790_e20171);
        let assign13790_e20174: f64 = (assign13790_e20172 - 0.01);
        let assign13790_e20176: f64 = (-10000.0);
        let assign13790_e20178: f64 = (assign13790_e20176 * 0.001);
        let (assign13790_e20239, assign13790_e20239_d_n4,) = {
    if (!(assign13790_e20174 < assign13790_e20178)) {
        let assign13790_e20185: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20186: f64 = (p.p709 - assign13790_e20185);
        let assign13790_e20188: f64 = (assign13790_e20186 - 0.01);
        let assign13790_e20192: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20193: f64 = (p.p709 - assign13790_e20192);
        let assign13790_e20195: f64 = (assign13790_e20193 - 0.01);
        let assign13790_e20199: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20200: f64 = (p.p709 - assign13790_e20199);
        let assign13790_e20202: f64 = (assign13790_e20200 - 0.01);
        let assign13790_e20203: f64 = (assign13790_e20195 * assign13790_e20202);
        let assign13790_e20206: f64 = (4.0 * 0.001);
        let assign13790_e20208: f64 = (assign13790_e20206 * 0.001);
        let assign13790_e20209: f64 = (assign13790_e20203 + assign13790_e20208);
        let assign13790_e20210: f64 = (assign13790_e20209).sqrt();
        let assign13790_e20211: f64 = (assign13790_e20188 + assign13790_e20210);
        let assign13790_e20212: f64 = (0.5 * assign13790_e20211);
        (assign13790_e20212, (0.5 * ((-(p.p893 * locals.var_deltemp_dn4)) + ((((-(p.p893 * locals.var_deltemp_dn4)) * assign13790_e20202) + (assign13790_e20195 * (-(p.p893 * locals.var_deltemp_dn4)))) / (2.0 * assign13790_e20210)))),)
    } else {
        let assign13790_e20216: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20217: f64 = (p.p709 - assign13790_e20216);
        let assign13790_e20219: f64 = (assign13790_e20217 - 0.01);
        let assign13790_e20221: f64 = (-10000.0);
        let assign13790_e20223: f64 = (assign13790_e20221 * 0.001);
        let (assign13790_e20238, assign13790_e20238_d_n4,) = {
            if (assign13790_e20219 < assign13790_e20223) {
                let assign13790_e20226: f64 = (-0.001);
                let assign13790_e20228: f64 = (assign13790_e20226 * 0.001);
                let assign13790_e20232: f64 = (p.p893 * locals.var_deltemp);
                let assign13790_e20233: f64 = (p.p709 - assign13790_e20232);
                let assign13790_e20235: f64 = (assign13790_e20233 - 0.01);
                let assign13790_e20236: f64 = (assign13790_e20228 / assign13790_e20235);
                (assign13790_e20236, (-((assign13790_e20228 * (-(p.p893 * locals.var_deltemp_dn4))) / (assign13790_e20235 * assign13790_e20235))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13790_e20238, assign13790_e20238_d_n4,)
    }
};
        let assign13790_e20241: f64 = (assign13790_e20239 + 0.01);
        (locals.var_pbsws_t, locals.var_pbsws_t_dn4, ) = (assign13790_e20241, assign13790_e20239_d_n4, );

        let assign13800_e20245: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20246: f64 = (p.p710 - assign13800_e20245);
        let assign13800_e20248: f64 = (assign13800_e20246 - 0.01);
        let assign13800_e20250: f64 = (-10000.0);
        let assign13800_e20252: f64 = (assign13800_e20250 * 0.001);
        let (assign13800_e20313, assign13800_e20313_d_n4,) = {
    if (!(assign13800_e20248 < assign13800_e20252)) {
        let assign13800_e20259: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20260: f64 = (p.p710 - assign13800_e20259);
        let assign13800_e20262: f64 = (assign13800_e20260 - 0.01);
        let assign13800_e20266: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20267: f64 = (p.p710 - assign13800_e20266);
        let assign13800_e20269: f64 = (assign13800_e20267 - 0.01);
        let assign13800_e20273: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20274: f64 = (p.p710 - assign13800_e20273);
        let assign13800_e20276: f64 = (assign13800_e20274 - 0.01);
        let assign13800_e20277: f64 = (assign13800_e20269 * assign13800_e20276);
        let assign13800_e20280: f64 = (4.0 * 0.001);
        let assign13800_e20282: f64 = (assign13800_e20280 * 0.001);
        let assign13800_e20283: f64 = (assign13800_e20277 + assign13800_e20282);
        let assign13800_e20284: f64 = (assign13800_e20283).sqrt();
        let assign13800_e20285: f64 = (assign13800_e20262 + assign13800_e20284);
        let assign13800_e20286: f64 = (0.5 * assign13800_e20285);
        (assign13800_e20286, (0.5 * ((-(p.p893 * locals.var_deltemp_dn4)) + ((((-(p.p893 * locals.var_deltemp_dn4)) * assign13800_e20276) + (assign13800_e20269 * (-(p.p893 * locals.var_deltemp_dn4)))) / (2.0 * assign13800_e20284)))),)
    } else {
        let assign13800_e20290: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20291: f64 = (p.p710 - assign13800_e20290);
        let assign13800_e20293: f64 = (assign13800_e20291 - 0.01);
        let assign13800_e20295: f64 = (-10000.0);
        let assign13800_e20297: f64 = (assign13800_e20295 * 0.001);
        let (assign13800_e20312, assign13800_e20312_d_n4,) = {
            if (assign13800_e20293 < assign13800_e20297) {
                let assign13800_e20300: f64 = (-0.001);
                let assign13800_e20302: f64 = (assign13800_e20300 * 0.001);
                let assign13800_e20306: f64 = (p.p893 * locals.var_deltemp);
                let assign13800_e20307: f64 = (p.p710 - assign13800_e20306);
                let assign13800_e20309: f64 = (assign13800_e20307 - 0.01);
                let assign13800_e20310: f64 = (assign13800_e20302 / assign13800_e20309);
                (assign13800_e20310, (-((assign13800_e20302 * (-(p.p893 * locals.var_deltemp_dn4))) / (assign13800_e20309 * assign13800_e20309))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13800_e20312, assign13800_e20312_d_n4,)
    }
};
        let assign13800_e20315: f64 = (assign13800_e20313 + 0.01);
        (locals.var_pbswd_t, locals.var_pbswd_t_dn4, ) = (assign13800_e20315, assign13800_e20313_d_n4, );

        let assign13810_e20319: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20320: f64 = (p.p711 - assign13810_e20319);
        let assign13810_e20322: f64 = (assign13810_e20320 - 0.01);
        let assign13810_e20324: f64 = (-10000.0);
        let assign13810_e20326: f64 = (assign13810_e20324 * 0.001);
        let (assign13810_e20387, assign13810_e20387_d_n4,) = {
    if (!(assign13810_e20322 < assign13810_e20326)) {
        let assign13810_e20333: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20334: f64 = (p.p711 - assign13810_e20333);
        let assign13810_e20336: f64 = (assign13810_e20334 - 0.01);
        let assign13810_e20340: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20341: f64 = (p.p711 - assign13810_e20340);
        let assign13810_e20343: f64 = (assign13810_e20341 - 0.01);
        let assign13810_e20347: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20348: f64 = (p.p711 - assign13810_e20347);
        let assign13810_e20350: f64 = (assign13810_e20348 - 0.01);
        let assign13810_e20351: f64 = (assign13810_e20343 * assign13810_e20350);
        let assign13810_e20354: f64 = (4.0 * 0.001);
        let assign13810_e20356: f64 = (assign13810_e20354 * 0.001);
        let assign13810_e20357: f64 = (assign13810_e20351 + assign13810_e20356);
        let assign13810_e20358: f64 = (assign13810_e20357).sqrt();
        let assign13810_e20359: f64 = (assign13810_e20336 + assign13810_e20358);
        let assign13810_e20360: f64 = (0.5 * assign13810_e20359);
        (assign13810_e20360, (0.5 * ((-(p.p894 * locals.var_deltemp_dn4)) + ((((-(p.p894 * locals.var_deltemp_dn4)) * assign13810_e20350) + (assign13810_e20343 * (-(p.p894 * locals.var_deltemp_dn4)))) / (2.0 * assign13810_e20358)))),)
    } else {
        let assign13810_e20364: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20365: f64 = (p.p711 - assign13810_e20364);
        let assign13810_e20367: f64 = (assign13810_e20365 - 0.01);
        let assign13810_e20369: f64 = (-10000.0);
        let assign13810_e20371: f64 = (assign13810_e20369 * 0.001);
        let (assign13810_e20386, assign13810_e20386_d_n4,) = {
            if (assign13810_e20367 < assign13810_e20371) {
                let assign13810_e20374: f64 = (-0.001);
                let assign13810_e20376: f64 = (assign13810_e20374 * 0.001);
                let assign13810_e20380: f64 = (p.p894 * locals.var_deltemp);
                let assign13810_e20381: f64 = (p.p711 - assign13810_e20380);
                let assign13810_e20383: f64 = (assign13810_e20381 - 0.01);
                let assign13810_e20384: f64 = (assign13810_e20376 / assign13810_e20383);
                (assign13810_e20384, (-((assign13810_e20376 * (-(p.p894 * locals.var_deltemp_dn4))) / (assign13810_e20383 * assign13810_e20383))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13810_e20386, assign13810_e20386_d_n4,)
    }
};
        let assign13810_e20389: f64 = (assign13810_e20387 + 0.01);
        (locals.var_pbswgs_t, locals.var_pbswgs_t_dn4, ) = (assign13810_e20389, assign13810_e20387_d_n4, );

        let assign13820_e20393: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20394: f64 = (p.p712 - assign13820_e20393);
        let assign13820_e20396: f64 = (assign13820_e20394 - 0.01);
        let assign13820_e20398: f64 = (-10000.0);
        let assign13820_e20400: f64 = (assign13820_e20398 * 0.001);
        let (assign13820_e20461, assign13820_e20461_d_n4,) = {
    if (!(assign13820_e20396 < assign13820_e20400)) {
        let assign13820_e20407: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20408: f64 = (p.p712 - assign13820_e20407);
        let assign13820_e20410: f64 = (assign13820_e20408 - 0.01);
        let assign13820_e20414: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20415: f64 = (p.p712 - assign13820_e20414);
        let assign13820_e20417: f64 = (assign13820_e20415 - 0.01);
        let assign13820_e20421: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20422: f64 = (p.p712 - assign13820_e20421);
        let assign13820_e20424: f64 = (assign13820_e20422 - 0.01);
        let assign13820_e20425: f64 = (assign13820_e20417 * assign13820_e20424);
        let assign13820_e20428: f64 = (4.0 * 0.001);
        let assign13820_e20430: f64 = (assign13820_e20428 * 0.001);
        let assign13820_e20431: f64 = (assign13820_e20425 + assign13820_e20430);
        let assign13820_e20432: f64 = (assign13820_e20431).sqrt();
        let assign13820_e20433: f64 = (assign13820_e20410 + assign13820_e20432);
        let assign13820_e20434: f64 = (0.5 * assign13820_e20433);
        (assign13820_e20434, (0.5 * ((-(p.p894 * locals.var_deltemp_dn4)) + ((((-(p.p894 * locals.var_deltemp_dn4)) * assign13820_e20424) + (assign13820_e20417 * (-(p.p894 * locals.var_deltemp_dn4)))) / (2.0 * assign13820_e20432)))),)
    } else {
        let assign13820_e20438: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20439: f64 = (p.p712 - assign13820_e20438);
        let assign13820_e20441: f64 = (assign13820_e20439 - 0.01);
        let assign13820_e20443: f64 = (-10000.0);
        let assign13820_e20445: f64 = (assign13820_e20443 * 0.001);
        let (assign13820_e20460, assign13820_e20460_d_n4,) = {
            if (assign13820_e20441 < assign13820_e20445) {
                let assign13820_e20448: f64 = (-0.001);
                let assign13820_e20450: f64 = (assign13820_e20448 * 0.001);
                let assign13820_e20454: f64 = (p.p894 * locals.var_deltemp);
                let assign13820_e20455: f64 = (p.p712 - assign13820_e20454);
                let assign13820_e20457: f64 = (assign13820_e20455 - 0.01);
                let assign13820_e20458: f64 = (assign13820_e20450 / assign13820_e20457);
                (assign13820_e20458, (-((assign13820_e20450 * (-(p.p894 * locals.var_deltemp_dn4))) / (assign13820_e20457 * assign13820_e20457))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13820_e20460, assign13820_e20460_d_n4,)
    }
};
        let assign13820_e20463: f64 = (assign13820_e20461 + 0.01);
        (locals.var_pbswgd_t, locals.var_pbswgd_t_dn4, ) = (assign13820_e20463, assign13820_e20461_d_n4, );

        let assign13830_e20466: f64 = (locals.var_eg0 / locals.var_vtm0);
        let assign13830_e20469: f64 = (locals.var_eg / locals.var_vtm);
        let assign13830_e20470: f64 = (assign13830_e20466 - assign13830_e20469);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign13830_e20470, 0.0, 0.0, 0.0, (-(((locals.var_eg_dn4 * locals.var_vtm) - (locals.var_eg * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign13840_e20473: f64 = (locals.var_tratio).max(1e-38);
        let assign13840_e20474: f64 = (assign13840_e20473).ln();
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign13840_e20474, 0.0, 0.0, 0.0, (if locals.var_tratio >= 1e-38 { locals.var_tratio_dn4 } else { 0.0 } / assign13840_e20473), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign13850_e20478: f64 = (p.p895 * locals.var_t1);
        let assign13850_e20479: f64 = (locals.var_t0 + assign13850_e20478);
        let assign13850_e20481: f64 = (assign13850_e20479 / p.p725);
        let assign13850_e20482: f64 = { let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14, ) = (assign13850_e20482, ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 + (p.p895 * locals.var_t1_dn0)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 + (p.p895 * locals.var_t1_dn2)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 + (p.p895 * locals.var_t1_dn3)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 + (p.p895 * locals.var_t1_dn4)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 + (p.p895 * locals.var_t1_dn5)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 + (p.p895 * locals.var_t1_dn6)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 + (p.p895 * locals.var_t1_dn7)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 + (p.p895 * locals.var_t1_dn8)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 + (p.p895 * locals.var_t1_dn9)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 + (p.p895 * locals.var_t1_dn10)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 + (p.p895 * locals.var_t1_dn11)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn12 + (p.p895 * locals.var_t1_dn12)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 + (p.p895 * locals.var_t1_dn13)) / p.p725)), ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 + (p.p895 * locals.var_t1_dn14)) / p.p725)), );

        let assign13860_e20485: f64 = (p.p719 * locals.var_t3);
        (locals.var_jss_t, locals.var_jss_t_dn0, locals.var_jss_t_dn2, locals.var_jss_t_dn3, locals.var_jss_t_dn4, locals.var_jss_t_dn5, locals.var_jss_t_dn6, locals.var_jss_t_dn7, locals.var_jss_t_dn8, locals.var_jss_t_dn9, locals.var_jss_t_dn10, locals.var_jss_t_dn11, locals.var_jss_t_dn12, locals.var_jss_t_dn13, locals.var_jss_t_dn14, ) = (assign13860_e20485, (p.p719 * locals.var_t3_dn0), (p.p719 * locals.var_t3_dn2), (p.p719 * locals.var_t3_dn3), (p.p719 * locals.var_t3_dn4), (p.p719 * locals.var_t3_dn5), (p.p719 * locals.var_t3_dn6), (p.p719 * locals.var_t3_dn7), (p.p719 * locals.var_t3_dn8), (p.p719 * locals.var_t3_dn9), (p.p719 * locals.var_t3_dn10), (p.p719 * locals.var_t3_dn11), (p.p719 * locals.var_t3_dn12), (p.p719 * locals.var_t3_dn13), (p.p719 * locals.var_t3_dn14), );

        let assign13870_e20488: f64 = (p.p721 * locals.var_t3);
        (locals.var_jsws_t, locals.var_jsws_t_dn0, locals.var_jsws_t_dn2, locals.var_jsws_t_dn3, locals.var_jsws_t_dn4, locals.var_jsws_t_dn5, locals.var_jsws_t_dn6, locals.var_jsws_t_dn7, locals.var_jsws_t_dn8, locals.var_jsws_t_dn9, locals.var_jsws_t_dn10, locals.var_jsws_t_dn11, locals.var_jsws_t_dn12, locals.var_jsws_t_dn13, locals.var_jsws_t_dn14, ) = (assign13870_e20488, (p.p721 * locals.var_t3_dn0), (p.p721 * locals.var_t3_dn2), (p.p721 * locals.var_t3_dn3), (p.p721 * locals.var_t3_dn4), (p.p721 * locals.var_t3_dn5), (p.p721 * locals.var_t3_dn6), (p.p721 * locals.var_t3_dn7), (p.p721 * locals.var_t3_dn8), (p.p721 * locals.var_t3_dn9), (p.p721 * locals.var_t3_dn10), (p.p721 * locals.var_t3_dn11), (p.p721 * locals.var_t3_dn12), (p.p721 * locals.var_t3_dn13), (p.p721 * locals.var_t3_dn14), );

        let assign13880_e20491: f64 = (p.p723 * locals.var_t3);
        (locals.var_jswgs_t, locals.var_jswgs_t_dn0, locals.var_jswgs_t_dn2, locals.var_jswgs_t_dn3, locals.var_jswgs_t_dn4, locals.var_jswgs_t_dn5, locals.var_jswgs_t_dn6, locals.var_jswgs_t_dn7, locals.var_jswgs_t_dn8, locals.var_jswgs_t_dn9, locals.var_jswgs_t_dn10, locals.var_jswgs_t_dn11, locals.var_jswgs_t_dn12, locals.var_jswgs_t_dn13, locals.var_jswgs_t_dn14, ) = (assign13880_e20491, (p.p723 * locals.var_t3_dn0), (p.p723 * locals.var_t3_dn2), (p.p723 * locals.var_t3_dn3), (p.p723 * locals.var_t3_dn4), (p.p723 * locals.var_t3_dn5), (p.p723 * locals.var_t3_dn6), (p.p723 * locals.var_t3_dn7), (p.p723 * locals.var_t3_dn8), (p.p723 * locals.var_t3_dn9), (p.p723 * locals.var_t3_dn10), (p.p723 * locals.var_t3_dn11), (p.p723 * locals.var_t3_dn12), (p.p723 * locals.var_t3_dn13), (p.p723 * locals.var_t3_dn14), );

        let assign13890_e20495: f64 = (p.p896 * locals.var_t1);
        let assign13890_e20496: f64 = (locals.var_t0 + assign13890_e20495);
        let assign13890_e20498: f64 = (assign13890_e20496 / p.p726);
        let assign13890_e20499: f64 = { let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14, ) = (assign13890_e20499, ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 + (p.p896 * locals.var_t1_dn0)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 + (p.p896 * locals.var_t1_dn2)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 + (p.p896 * locals.var_t1_dn3)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 + (p.p896 * locals.var_t1_dn4)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 + (p.p896 * locals.var_t1_dn5)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 + (p.p896 * locals.var_t1_dn6)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 + (p.p896 * locals.var_t1_dn7)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 + (p.p896 * locals.var_t1_dn8)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 + (p.p896 * locals.var_t1_dn9)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 + (p.p896 * locals.var_t1_dn10)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 + (p.p896 * locals.var_t1_dn11)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn12 + (p.p896 * locals.var_t1_dn12)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 + (p.p896 * locals.var_t1_dn13)) / p.p726)), ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 + (p.p896 * locals.var_t1_dn14)) / p.p726)), );

        let assign13900_e20502: f64 = (p.p720 * locals.var_t3);
        (locals.var_jsd_t, locals.var_jsd_t_dn0, locals.var_jsd_t_dn2, locals.var_jsd_t_dn3, locals.var_jsd_t_dn4, locals.var_jsd_t_dn5, locals.var_jsd_t_dn6, locals.var_jsd_t_dn7, locals.var_jsd_t_dn8, locals.var_jsd_t_dn9, locals.var_jsd_t_dn10, locals.var_jsd_t_dn11, locals.var_jsd_t_dn12, locals.var_jsd_t_dn13, locals.var_jsd_t_dn14, ) = (assign13900_e20502, (p.p720 * locals.var_t3_dn0), (p.p720 * locals.var_t3_dn2), (p.p720 * locals.var_t3_dn3), (p.p720 * locals.var_t3_dn4), (p.p720 * locals.var_t3_dn5), (p.p720 * locals.var_t3_dn6), (p.p720 * locals.var_t3_dn7), (p.p720 * locals.var_t3_dn8), (p.p720 * locals.var_t3_dn9), (p.p720 * locals.var_t3_dn10), (p.p720 * locals.var_t3_dn11), (p.p720 * locals.var_t3_dn12), (p.p720 * locals.var_t3_dn13), (p.p720 * locals.var_t3_dn14), );

        let assign13910_e20505: f64 = (p.p722 * locals.var_t3);
        (locals.var_jswd_t, locals.var_jswd_t_dn0, locals.var_jswd_t_dn2, locals.var_jswd_t_dn3, locals.var_jswd_t_dn4, locals.var_jswd_t_dn5, locals.var_jswd_t_dn6, locals.var_jswd_t_dn7, locals.var_jswd_t_dn8, locals.var_jswd_t_dn9, locals.var_jswd_t_dn10, locals.var_jswd_t_dn11, locals.var_jswd_t_dn12, locals.var_jswd_t_dn13, locals.var_jswd_t_dn14, ) = (assign13910_e20505, (p.p722 * locals.var_t3_dn0), (p.p722 * locals.var_t3_dn2), (p.p722 * locals.var_t3_dn3), (p.p722 * locals.var_t3_dn4), (p.p722 * locals.var_t3_dn5), (p.p722 * locals.var_t3_dn6), (p.p722 * locals.var_t3_dn7), (p.p722 * locals.var_t3_dn8), (p.p722 * locals.var_t3_dn9), (p.p722 * locals.var_t3_dn10), (p.p722 * locals.var_t3_dn11), (p.p722 * locals.var_t3_dn12), (p.p722 * locals.var_t3_dn13), (p.p722 * locals.var_t3_dn14), );

        let assign13920_e20508: f64 = (p.p724 * locals.var_t3);
        (locals.var_jswgd_t, locals.var_jswgd_t_dn0, locals.var_jswgd_t_dn2, locals.var_jswgd_t_dn3, locals.var_jswgd_t_dn4, locals.var_jswgd_t_dn5, locals.var_jswgd_t_dn6, locals.var_jswgd_t_dn7, locals.var_jswgd_t_dn8, locals.var_jswgd_t_dn9, locals.var_jswgd_t_dn10, locals.var_jswgd_t_dn11, locals.var_jswgd_t_dn12, locals.var_jswgd_t_dn13, locals.var_jswgd_t_dn14, ) = (assign13920_e20508, (p.p724 * locals.var_t3_dn0), (p.p724 * locals.var_t3_dn2), (p.p724 * locals.var_t3_dn3), (p.p724 * locals.var_t3_dn4), (p.p724 * locals.var_t3_dn5), (p.p724 * locals.var_t3_dn6), (p.p724 * locals.var_t3_dn7), (p.p724 * locals.var_t3_dn8), (p.p724 * locals.var_t3_dn9), (p.p724 * locals.var_t3_dn10), (p.p724 * locals.var_t3_dn11), (p.p724 * locals.var_t3_dn12), (p.p724 * locals.var_t3_dn13), (p.p724 * locals.var_t3_dn14), );

        let assign13930_e20512: f64 = (locals.var_eg0 * p.p897);
        let assign13930_e20515: f64 = (locals.var_tratio - 1.0);
        let assign13930_e20516: f64 = (assign13930_e20512 * assign13930_e20515);
        let assign13930_e20518: f64 = (assign13930_e20516 / locals.var_vtm);
        let assign13930_e20519: f64 = { let limited_exp_arg = assign13930_e20518; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13930_e20520: f64 = (p.p735 * assign13930_e20519);
        (locals.var_jtss_t, locals.var_jtss_t_dn4, ) = (assign13930_e20520, (p.p735 * ({ let limited_exp_arg = assign13930_e20518; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13930_e20512 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13930_e20516 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))), );

        let assign13940_e20524: f64 = (locals.var_eg0 * p.p899);
        let assign13940_e20527: f64 = (locals.var_tratio - 1.0);
        let assign13940_e20528: f64 = (assign13940_e20524 * assign13940_e20527);
        let assign13940_e20530: f64 = (assign13940_e20528 / locals.var_vtm);
        let assign13940_e20531: f64 = { let limited_exp_arg = assign13940_e20530; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13940_e20532: f64 = (p.p737 * assign13940_e20531);
        (locals.var_jtssws_t, locals.var_jtssws_t_dn4, ) = (assign13940_e20532, (p.p737 * ({ let limited_exp_arg = assign13940_e20530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13940_e20524 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13940_e20528 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))), );

        let assign13950_e20536: f64 = (p.p741 / locals.var_weffcj);
        let assign13950_e20537: f64 = (assign13950_e20536).sqrt();
        let assign13950_e20539: f64 = (assign13950_e20537 + 1.0);
        let assign13950_e20540: f64 = (p.p739 * assign13950_e20539);
        let assign13950_e20543: f64 = (locals.var_eg0 * p.p901);
        let assign13950_e20546: f64 = (locals.var_tratio - 1.0);
        let assign13950_e20547: f64 = (assign13950_e20543 * assign13950_e20546);
        let assign13950_e20549: f64 = (assign13950_e20547 / locals.var_vtm);
        let assign13950_e20550: f64 = { let limited_exp_arg = assign13950_e20549; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13950_e20551: f64 = (assign13950_e20540 * assign13950_e20550);
        (locals.var_jtsswgs_t, locals.var_jtsswgs_t_dn4, ) = (assign13950_e20551, (assign13950_e20540 * ({ let limited_exp_arg = assign13950_e20549; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13950_e20543 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13950_e20547 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))), );

        let assign13960_e20555: f64 = (locals.var_eg0 * p.p898);
        let assign13960_e20558: f64 = (locals.var_tratio - 1.0);
        let assign13960_e20559: f64 = (assign13960_e20555 * assign13960_e20558);
        let assign13960_e20561: f64 = (assign13960_e20559 / locals.var_vtm);
        let assign13960_e20562: f64 = { let limited_exp_arg = assign13960_e20561; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13960_e20563: f64 = (p.p736 * assign13960_e20562);
        (locals.var_jtsd_t, locals.var_jtsd_t_dn4, ) = (assign13960_e20563, (p.p736 * ({ let limited_exp_arg = assign13960_e20561; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13960_e20555 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13960_e20559 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))), );

        let assign13970_e20567: f64 = (locals.var_eg0 * p.p900);
        let assign13970_e20570: f64 = (locals.var_tratio - 1.0);
        let assign13970_e20571: f64 = (assign13970_e20567 * assign13970_e20570);
        let assign13970_e20573: f64 = (assign13970_e20571 / locals.var_vtm);
        let assign13970_e20574: f64 = { let limited_exp_arg = assign13970_e20573; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13970_e20575: f64 = (p.p738 * assign13970_e20574);
        (locals.var_jtsswd_t, locals.var_jtsswd_t_dn4, ) = (assign13970_e20575, (p.p738 * ({ let limited_exp_arg = assign13970_e20573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13970_e20567 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13970_e20571 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))), );

        let assign13980_e20579: f64 = (p.p741 / locals.var_weffcj);
        let assign13980_e20580: f64 = (assign13980_e20579).sqrt();
        let assign13980_e20582: f64 = (assign13980_e20580 + 1.0);
        let assign13980_e20583: f64 = (p.p740 * assign13980_e20582);
        let assign13980_e20586: f64 = (locals.var_eg0 * p.p902);
        let assign13980_e20589: f64 = (locals.var_tratio - 1.0);
        let assign13980_e20590: f64 = (assign13980_e20586 * assign13980_e20589);
        let assign13980_e20592: f64 = (assign13980_e20590 / locals.var_vtm);
        let assign13980_e20593: f64 = { let limited_exp_arg = assign13980_e20592; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13980_e20594: f64 = (assign13980_e20583 * assign13980_e20593);
        (locals.var_jtsswgd_t, locals.var_jtsswgd_t_dn4, ) = (assign13980_e20594, (assign13980_e20583 * ({ let limited_exp_arg = assign13980_e20592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13980_e20586 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13980_e20590 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))), );

        let assign13990_e20600: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20601: f64 = (p.p903 * assign13990_e20600);
        let assign13990_e20602: f64 = (1.0 + assign13990_e20601);
        let assign13990_e20603: f64 = (p.p742 * assign13990_e20602);
        let assign13990_e20605: f64 = (assign13990_e20603 - 0.01);
        let assign13990_e20607: f64 = (-10000.0);
        let assign13990_e20609: f64 = (assign13990_e20607 * 0.001);
        let (assign13990_e20690, assign13990_e20690_d_n4,) = {
    if (!(assign13990_e20605 < assign13990_e20609)) {
        let assign13990_e20618: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20619: f64 = (p.p903 * assign13990_e20618);
        let assign13990_e20620: f64 = (1.0 + assign13990_e20619);
        let assign13990_e20621: f64 = (p.p742 * assign13990_e20620);
        let assign13990_e20623: f64 = (assign13990_e20621 - 0.01);
        let assign13990_e20629: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20630: f64 = (p.p903 * assign13990_e20629);
        let assign13990_e20631: f64 = (1.0 + assign13990_e20630);
        let assign13990_e20632: f64 = (p.p742 * assign13990_e20631);
        let assign13990_e20634: f64 = (assign13990_e20632 - 0.01);
        let assign13990_e20640: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20641: f64 = (p.p903 * assign13990_e20640);
        let assign13990_e20642: f64 = (1.0 + assign13990_e20641);
        let assign13990_e20643: f64 = (p.p742 * assign13990_e20642);
        let assign13990_e20645: f64 = (assign13990_e20643 - 0.01);
        let assign13990_e20646: f64 = (assign13990_e20634 * assign13990_e20645);
        let assign13990_e20649: f64 = (4.0 * 0.001);
        let assign13990_e20651: f64 = (assign13990_e20649 * 0.001);
        let assign13990_e20652: f64 = (assign13990_e20646 + assign13990_e20651);
        let assign13990_e20653: f64 = (assign13990_e20652).sqrt();
        let assign13990_e20654: f64 = (assign13990_e20623 + assign13990_e20653);
        let assign13990_e20655: f64 = (0.5 * assign13990_e20654);
        (assign13990_e20655, (0.5 * ((p.p742 * (p.p903 * locals.var_tratio_dn4)) + ((((p.p742 * (p.p903 * locals.var_tratio_dn4)) * assign13990_e20645) + (assign13990_e20634 * (p.p742 * (p.p903 * locals.var_tratio_dn4)))) / (2.0 * assign13990_e20653)))),)
    } else {
        let assign13990_e20661: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20662: f64 = (p.p903 * assign13990_e20661);
        let assign13990_e20663: f64 = (1.0 + assign13990_e20662);
        let assign13990_e20664: f64 = (p.p742 * assign13990_e20663);
        let assign13990_e20666: f64 = (assign13990_e20664 - 0.01);
        let assign13990_e20668: f64 = (-10000.0);
        let assign13990_e20670: f64 = (assign13990_e20668 * 0.001);
        let (assign13990_e20689, assign13990_e20689_d_n4,) = {
            if (assign13990_e20666 < assign13990_e20670) {
                let assign13990_e20673: f64 = (-0.001);
                let assign13990_e20675: f64 = (assign13990_e20673 * 0.001);
                let assign13990_e20681: f64 = (locals.var_tratio - 1.0);
                let assign13990_e20682: f64 = (p.p903 * assign13990_e20681);
                let assign13990_e20683: f64 = (1.0 + assign13990_e20682);
                let assign13990_e20684: f64 = (p.p742 * assign13990_e20683);
                let assign13990_e20686: f64 = (assign13990_e20684 - 0.01);
                let assign13990_e20687: f64 = (assign13990_e20675 / assign13990_e20686);
                (assign13990_e20687, (-((assign13990_e20675 * (p.p742 * (p.p903 * locals.var_tratio_dn4))) / (assign13990_e20686 * assign13990_e20686))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13990_e20689, assign13990_e20689_d_n4,)
    }
};
        let assign13990_e20692: f64 = (assign13990_e20690 + 0.01);
        (locals.var_njts_t, locals.var_njts_t_dn4, ) = (assign13990_e20692, assign13990_e20690_d_n4, );

        let assign14000_e20698: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20699: f64 = (p.p905 * assign14000_e20698);
        let assign14000_e20700: f64 = (1.0 + assign14000_e20699);
        let assign14000_e20701: f64 = (p.p744 * assign14000_e20700);
        let assign14000_e20703: f64 = (assign14000_e20701 - 0.01);
        let assign14000_e20705: f64 = (-10000.0);
        let assign14000_e20707: f64 = (assign14000_e20705 * 0.001);
        let (assign14000_e20788, assign14000_e20788_d_n4,) = {
    if (!(assign14000_e20703 < assign14000_e20707)) {
        let assign14000_e20716: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20717: f64 = (p.p905 * assign14000_e20716);
        let assign14000_e20718: f64 = (1.0 + assign14000_e20717);
        let assign14000_e20719: f64 = (p.p744 * assign14000_e20718);
        let assign14000_e20721: f64 = (assign14000_e20719 - 0.01);
        let assign14000_e20727: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20728: f64 = (p.p905 * assign14000_e20727);
        let assign14000_e20729: f64 = (1.0 + assign14000_e20728);
        let assign14000_e20730: f64 = (p.p744 * assign14000_e20729);
        let assign14000_e20732: f64 = (assign14000_e20730 - 0.01);
        let assign14000_e20738: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20739: f64 = (p.p905 * assign14000_e20738);
        let assign14000_e20740: f64 = (1.0 + assign14000_e20739);
        let assign14000_e20741: f64 = (p.p744 * assign14000_e20740);
        let assign14000_e20743: f64 = (assign14000_e20741 - 0.01);
        let assign14000_e20744: f64 = (assign14000_e20732 * assign14000_e20743);
        let assign14000_e20747: f64 = (4.0 * 0.001);
        let assign14000_e20749: f64 = (assign14000_e20747 * 0.001);
        let assign14000_e20750: f64 = (assign14000_e20744 + assign14000_e20749);
        let assign14000_e20751: f64 = (assign14000_e20750).sqrt();
        let assign14000_e20752: f64 = (assign14000_e20721 + assign14000_e20751);
        let assign14000_e20753: f64 = (0.5 * assign14000_e20752);
        (assign14000_e20753, (0.5 * ((p.p744 * (p.p905 * locals.var_tratio_dn4)) + ((((p.p744 * (p.p905 * locals.var_tratio_dn4)) * assign14000_e20743) + (assign14000_e20732 * (p.p744 * (p.p905 * locals.var_tratio_dn4)))) / (2.0 * assign14000_e20751)))),)
    } else {
        let assign14000_e20759: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20760: f64 = (p.p905 * assign14000_e20759);
        let assign14000_e20761: f64 = (1.0 + assign14000_e20760);
        let assign14000_e20762: f64 = (p.p744 * assign14000_e20761);
        let assign14000_e20764: f64 = (assign14000_e20762 - 0.01);
        let assign14000_e20766: f64 = (-10000.0);
        let assign14000_e20768: f64 = (assign14000_e20766 * 0.001);
        let (assign14000_e20787, assign14000_e20787_d_n4,) = {
            if (assign14000_e20764 < assign14000_e20768) {
                let assign14000_e20771: f64 = (-0.001);
                let assign14000_e20773: f64 = (assign14000_e20771 * 0.001);
                let assign14000_e20779: f64 = (locals.var_tratio - 1.0);
                let assign14000_e20780: f64 = (p.p905 * assign14000_e20779);
                let assign14000_e20781: f64 = (1.0 + assign14000_e20780);
                let assign14000_e20782: f64 = (p.p744 * assign14000_e20781);
                let assign14000_e20784: f64 = (assign14000_e20782 - 0.01);
                let assign14000_e20785: f64 = (assign14000_e20773 / assign14000_e20784);
                (assign14000_e20785, (-((assign14000_e20773 * (p.p744 * (p.p905 * locals.var_tratio_dn4))) / (assign14000_e20784 * assign14000_e20784))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14000_e20787, assign14000_e20787_d_n4,)
    }
};
        let assign14000_e20790: f64 = (assign14000_e20788 + 0.01);
        (locals.var_njtssw_t, locals.var_njtssw_t_dn4, ) = (assign14000_e20790, assign14000_e20788_d_n4, );

    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign14010_e20796: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20797: f64 = (p.p907 * assign14010_e20796);
        let assign14010_e20798: f64 = (1.0 + assign14010_e20797);
        let assign14010_e20799: f64 = (p.p746 * assign14010_e20798);
        let assign14010_e20801: f64 = (assign14010_e20799 - 0.01);
        let assign14010_e20803: f64 = (-10000.0);
        let assign14010_e20805: f64 = (assign14010_e20803 * 0.001);
        let (assign14010_e20886, assign14010_e20886_d_n4,) = {
    if (!(assign14010_e20801 < assign14010_e20805)) {
        let assign14010_e20814: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20815: f64 = (p.p907 * assign14010_e20814);
        let assign14010_e20816: f64 = (1.0 + assign14010_e20815);
        let assign14010_e20817: f64 = (p.p746 * assign14010_e20816);
        let assign14010_e20819: f64 = (assign14010_e20817 - 0.01);
        let assign14010_e20825: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20826: f64 = (p.p907 * assign14010_e20825);
        let assign14010_e20827: f64 = (1.0 + assign14010_e20826);
        let assign14010_e20828: f64 = (p.p746 * assign14010_e20827);
        let assign14010_e20830: f64 = (assign14010_e20828 - 0.01);
        let assign14010_e20836: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20837: f64 = (p.p907 * assign14010_e20836);
        let assign14010_e20838: f64 = (1.0 + assign14010_e20837);
        let assign14010_e20839: f64 = (p.p746 * assign14010_e20838);
        let assign14010_e20841: f64 = (assign14010_e20839 - 0.01);
        let assign14010_e20842: f64 = (assign14010_e20830 * assign14010_e20841);
        let assign14010_e20845: f64 = (4.0 * 0.001);
        let assign14010_e20847: f64 = (assign14010_e20845 * 0.001);
        let assign14010_e20848: f64 = (assign14010_e20842 + assign14010_e20847);
        let assign14010_e20849: f64 = (assign14010_e20848).sqrt();
        let assign14010_e20850: f64 = (assign14010_e20819 + assign14010_e20849);
        let assign14010_e20851: f64 = (0.5 * assign14010_e20850);
        (assign14010_e20851, (0.5 * ((p.p746 * (p.p907 * locals.var_tratio_dn4)) + ((((p.p746 * (p.p907 * locals.var_tratio_dn4)) * assign14010_e20841) + (assign14010_e20830 * (p.p746 * (p.p907 * locals.var_tratio_dn4)))) / (2.0 * assign14010_e20849)))),)
    } else {
        let assign14010_e20857: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20858: f64 = (p.p907 * assign14010_e20857);
        let assign14010_e20859: f64 = (1.0 + assign14010_e20858);
        let assign14010_e20860: f64 = (p.p746 * assign14010_e20859);
        let assign14010_e20862: f64 = (assign14010_e20860 - 0.01);
        let assign14010_e20864: f64 = (-10000.0);
        let assign14010_e20866: f64 = (assign14010_e20864 * 0.001);
        let (assign14010_e20885, assign14010_e20885_d_n4,) = {
            if (assign14010_e20862 < assign14010_e20866) {
                let assign14010_e20869: f64 = (-0.001);
                let assign14010_e20871: f64 = (assign14010_e20869 * 0.001);
                let assign14010_e20877: f64 = (locals.var_tratio - 1.0);
                let assign14010_e20878: f64 = (p.p907 * assign14010_e20877);
                let assign14010_e20879: f64 = (1.0 + assign14010_e20878);
                let assign14010_e20880: f64 = (p.p746 * assign14010_e20879);
                let assign14010_e20882: f64 = (assign14010_e20880 - 0.01);
                let assign14010_e20883: f64 = (assign14010_e20871 / assign14010_e20882);
                (assign14010_e20883, (-((assign14010_e20871 * (p.p746 * (p.p907 * locals.var_tratio_dn4))) / (assign14010_e20882 * assign14010_e20882))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14010_e20885, assign14010_e20885_d_n4,)
    }
};
        let assign14010_e20888: f64 = (assign14010_e20886 + 0.01);
        (locals.var_njtsswg_t, locals.var_njtsswg_t_dn4, ) = (assign14010_e20888, assign14010_e20886_d_n4, );

        let assign14020_e20894: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20895: f64 = (p.p904 * assign14020_e20894);
        let assign14020_e20896: f64 = (1.0 + assign14020_e20895);
        let assign14020_e20897: f64 = (p.p743 * assign14020_e20896);
        let assign14020_e20899: f64 = (assign14020_e20897 - 0.01);
        let assign14020_e20901: f64 = (-10000.0);
        let assign14020_e20903: f64 = (assign14020_e20901 * 0.001);
        let (assign14020_e20984, assign14020_e20984_d_n4,) = {
    if (!(assign14020_e20899 < assign14020_e20903)) {
        let assign14020_e20912: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20913: f64 = (p.p904 * assign14020_e20912);
        let assign14020_e20914: f64 = (1.0 + assign14020_e20913);
        let assign14020_e20915: f64 = (p.p743 * assign14020_e20914);
        let assign14020_e20917: f64 = (assign14020_e20915 - 0.01);
        let assign14020_e20923: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20924: f64 = (p.p904 * assign14020_e20923);
        let assign14020_e20925: f64 = (1.0 + assign14020_e20924);
        let assign14020_e20926: f64 = (p.p743 * assign14020_e20925);
        let assign14020_e20928: f64 = (assign14020_e20926 - 0.01);
        let assign14020_e20934: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20935: f64 = (p.p904 * assign14020_e20934);
        let assign14020_e20936: f64 = (1.0 + assign14020_e20935);
        let assign14020_e20937: f64 = (p.p743 * assign14020_e20936);
        let assign14020_e20939: f64 = (assign14020_e20937 - 0.01);
        let assign14020_e20940: f64 = (assign14020_e20928 * assign14020_e20939);
        let assign14020_e20943: f64 = (4.0 * 0.001);
        let assign14020_e20945: f64 = (assign14020_e20943 * 0.001);
        let assign14020_e20946: f64 = (assign14020_e20940 + assign14020_e20945);
        let assign14020_e20947: f64 = (assign14020_e20946).sqrt();
        let assign14020_e20948: f64 = (assign14020_e20917 + assign14020_e20947);
        let assign14020_e20949: f64 = (0.5 * assign14020_e20948);
        (assign14020_e20949, (0.5 * ((p.p743 * (p.p904 * locals.var_tratio_dn4)) + ((((p.p743 * (p.p904 * locals.var_tratio_dn4)) * assign14020_e20939) + (assign14020_e20928 * (p.p743 * (p.p904 * locals.var_tratio_dn4)))) / (2.0 * assign14020_e20947)))),)
    } else {
        let assign14020_e20955: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20956: f64 = (p.p904 * assign14020_e20955);
        let assign14020_e20957: f64 = (1.0 + assign14020_e20956);
        let assign14020_e20958: f64 = (p.p743 * assign14020_e20957);
        let assign14020_e20960: f64 = (assign14020_e20958 - 0.01);
        let assign14020_e20962: f64 = (-10000.0);
        let assign14020_e20964: f64 = (assign14020_e20962 * 0.001);
        let (assign14020_e20983, assign14020_e20983_d_n4,) = {
            if (assign14020_e20960 < assign14020_e20964) {
                let assign14020_e20967: f64 = (-0.001);
                let assign14020_e20969: f64 = (assign14020_e20967 * 0.001);
                let assign14020_e20975: f64 = (locals.var_tratio - 1.0);
                let assign14020_e20976: f64 = (p.p904 * assign14020_e20975);
                let assign14020_e20977: f64 = (1.0 + assign14020_e20976);
                let assign14020_e20978: f64 = (p.p743 * assign14020_e20977);
                let assign14020_e20980: f64 = (assign14020_e20978 - 0.01);
                let assign14020_e20981: f64 = (assign14020_e20969 / assign14020_e20980);
                (assign14020_e20981, (-((assign14020_e20969 * (p.p743 * (p.p904 * locals.var_tratio_dn4))) / (assign14020_e20980 * assign14020_e20980))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14020_e20983, assign14020_e20983_d_n4,)
    }
};
        let assign14020_e20986: f64 = (assign14020_e20984 + 0.01);
        (locals.var_njtsd_t, locals.var_njtsd_t_dn4, ) = (assign14020_e20986, assign14020_e20984_d_n4, );

        let assign14030_e20992: f64 = (locals.var_tratio - 1.0);
        let assign14030_e20993: f64 = (p.p906 * assign14030_e20992);
        let assign14030_e20994: f64 = (1.0 + assign14030_e20993);
        let assign14030_e20995: f64 = (p.p745 * assign14030_e20994);
        let assign14030_e20997: f64 = (assign14030_e20995 - 0.01);
        let assign14030_e20999: f64 = (-10000.0);
        let assign14030_e21001: f64 = (assign14030_e20999 * 0.001);
        let (assign14030_e21082, assign14030_e21082_d_n4,) = {
    if (!(assign14030_e20997 < assign14030_e21001)) {
        let assign14030_e21010: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21011: f64 = (p.p906 * assign14030_e21010);
        let assign14030_e21012: f64 = (1.0 + assign14030_e21011);
        let assign14030_e21013: f64 = (p.p745 * assign14030_e21012);
        let assign14030_e21015: f64 = (assign14030_e21013 - 0.01);
        let assign14030_e21021: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21022: f64 = (p.p906 * assign14030_e21021);
        let assign14030_e21023: f64 = (1.0 + assign14030_e21022);
        let assign14030_e21024: f64 = (p.p745 * assign14030_e21023);
        let assign14030_e21026: f64 = (assign14030_e21024 - 0.01);
        let assign14030_e21032: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21033: f64 = (p.p906 * assign14030_e21032);
        let assign14030_e21034: f64 = (1.0 + assign14030_e21033);
        let assign14030_e21035: f64 = (p.p745 * assign14030_e21034);
        let assign14030_e21037: f64 = (assign14030_e21035 - 0.01);
        let assign14030_e21038: f64 = (assign14030_e21026 * assign14030_e21037);
        let assign14030_e21041: f64 = (4.0 * 0.001);
        let assign14030_e21043: f64 = (assign14030_e21041 * 0.001);
        let assign14030_e21044: f64 = (assign14030_e21038 + assign14030_e21043);
        let assign14030_e21045: f64 = (assign14030_e21044).sqrt();
        let assign14030_e21046: f64 = (assign14030_e21015 + assign14030_e21045);
        let assign14030_e21047: f64 = (0.5 * assign14030_e21046);
        (assign14030_e21047, (0.5 * ((p.p745 * (p.p906 * locals.var_tratio_dn4)) + ((((p.p745 * (p.p906 * locals.var_tratio_dn4)) * assign14030_e21037) + (assign14030_e21026 * (p.p745 * (p.p906 * locals.var_tratio_dn4)))) / (2.0 * assign14030_e21045)))),)
    } else {
        let assign14030_e21053: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21054: f64 = (p.p906 * assign14030_e21053);
        let assign14030_e21055: f64 = (1.0 + assign14030_e21054);
        let assign14030_e21056: f64 = (p.p745 * assign14030_e21055);
        let assign14030_e21058: f64 = (assign14030_e21056 - 0.01);
        let assign14030_e21060: f64 = (-10000.0);
        let assign14030_e21062: f64 = (assign14030_e21060 * 0.001);
        let (assign14030_e21081, assign14030_e21081_d_n4,) = {
            if (assign14030_e21058 < assign14030_e21062) {
                let assign14030_e21065: f64 = (-0.001);
                let assign14030_e21067: f64 = (assign14030_e21065 * 0.001);
                let assign14030_e21073: f64 = (locals.var_tratio - 1.0);
                let assign14030_e21074: f64 = (p.p906 * assign14030_e21073);
                let assign14030_e21075: f64 = (1.0 + assign14030_e21074);
                let assign14030_e21076: f64 = (p.p745 * assign14030_e21075);
                let assign14030_e21078: f64 = (assign14030_e21076 - 0.01);
                let assign14030_e21079: f64 = (assign14030_e21067 / assign14030_e21078);
                (assign14030_e21079, (-((assign14030_e21067 * (p.p745 * (p.p906 * locals.var_tratio_dn4))) / (assign14030_e21078 * assign14030_e21078))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14030_e21081, assign14030_e21081_d_n4,)
    }
};
        let assign14030_e21084: f64 = (assign14030_e21082 + 0.01);
        (locals.var_njtsswd_t, locals.var_njtsswd_t_dn4, ) = (assign14030_e21084, assign14030_e21082_d_n4, );

        let assign14040_e21090: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21091: f64 = (p.p908 * assign14040_e21090);
        let assign14040_e21092: f64 = (1.0 + assign14040_e21091);
        let assign14040_e21093: f64 = (p.p747 * assign14040_e21092);
        let assign14040_e21095: f64 = (assign14040_e21093 - 0.01);
        let assign14040_e21097: f64 = (-10000.0);
        let assign14040_e21099: f64 = (assign14040_e21097 * 0.001);
        let (assign14040_e21180, assign14040_e21180_d_n4,) = {
    if (!(assign14040_e21095 < assign14040_e21099)) {
        let assign14040_e21108: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21109: f64 = (p.p908 * assign14040_e21108);
        let assign14040_e21110: f64 = (1.0 + assign14040_e21109);
        let assign14040_e21111: f64 = (p.p747 * assign14040_e21110);
        let assign14040_e21113: f64 = (assign14040_e21111 - 0.01);
        let assign14040_e21119: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21120: f64 = (p.p908 * assign14040_e21119);
        let assign14040_e21121: f64 = (1.0 + assign14040_e21120);
        let assign14040_e21122: f64 = (p.p747 * assign14040_e21121);
        let assign14040_e21124: f64 = (assign14040_e21122 - 0.01);
        let assign14040_e21130: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21131: f64 = (p.p908 * assign14040_e21130);
        let assign14040_e21132: f64 = (1.0 + assign14040_e21131);
        let assign14040_e21133: f64 = (p.p747 * assign14040_e21132);
        let assign14040_e21135: f64 = (assign14040_e21133 - 0.01);
        let assign14040_e21136: f64 = (assign14040_e21124 * assign14040_e21135);
        let assign14040_e21139: f64 = (4.0 * 0.001);
        let assign14040_e21141: f64 = (assign14040_e21139 * 0.001);
        let assign14040_e21142: f64 = (assign14040_e21136 + assign14040_e21141);
        let assign14040_e21143: f64 = (assign14040_e21142).sqrt();
        let assign14040_e21144: f64 = (assign14040_e21113 + assign14040_e21143);
        let assign14040_e21145: f64 = (0.5 * assign14040_e21144);
        (assign14040_e21145, (0.5 * ((p.p747 * (p.p908 * locals.var_tratio_dn4)) + ((((p.p747 * (p.p908 * locals.var_tratio_dn4)) * assign14040_e21135) + (assign14040_e21124 * (p.p747 * (p.p908 * locals.var_tratio_dn4)))) / (2.0 * assign14040_e21143)))),)
    } else {
        let assign14040_e21151: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21152: f64 = (p.p908 * assign14040_e21151);
        let assign14040_e21153: f64 = (1.0 + assign14040_e21152);
        let assign14040_e21154: f64 = (p.p747 * assign14040_e21153);
        let assign14040_e21156: f64 = (assign14040_e21154 - 0.01);
        let assign14040_e21158: f64 = (-10000.0);
        let assign14040_e21160: f64 = (assign14040_e21158 * 0.001);
        let (assign14040_e21179, assign14040_e21179_d_n4,) = {
            if (assign14040_e21156 < assign14040_e21160) {
                let assign14040_e21163: f64 = (-0.001);
                let assign14040_e21165: f64 = (assign14040_e21163 * 0.001);
                let assign14040_e21171: f64 = (locals.var_tratio - 1.0);
                let assign14040_e21172: f64 = (p.p908 * assign14040_e21171);
                let assign14040_e21173: f64 = (1.0 + assign14040_e21172);
                let assign14040_e21174: f64 = (p.p747 * assign14040_e21173);
                let assign14040_e21176: f64 = (assign14040_e21174 - 0.01);
                let assign14040_e21177: f64 = (assign14040_e21165 / assign14040_e21176);
                (assign14040_e21177, (-((assign14040_e21165 * (p.p747 * (p.p908 * locals.var_tratio_dn4))) / (assign14040_e21176 * assign14040_e21176))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14040_e21179, assign14040_e21179_d_n4,)
    }
};
        let assign14040_e21182: f64 = (assign14040_e21180 + 0.01);
        (locals.var_njtsswgd_t, locals.var_njtsswgd_t_dn4, ) = (assign14040_e21182, assign14040_e21180_d_n4, );

        let assign14050_e21185: f64 = if p.p9 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard462 = assign14050_e21185;

        let assign14060_e21188: f64 = (p.p2 % 2.0);
        let assign14060_e21190: f64 = if assign14060_e21188 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard463 = assign14060_e21190;

        if ((locals.var_guard462 != 0.0) && (locals.var_guard463 != 0.0)) {
            locals.var_nuendd = 1.0;
            locals.var_nuends = 1.0;
        }

        if ((locals.var_guard462 != 0.0) && (locals.var_guard463 != 0.0)) {
            let assign14090_e21209: f64 = (p.p2 - 1.0);
            let assign14090_e21211: f64 = (assign14090_e21209 / 2.0);
            let assign14090_e21213: f64 = (assign14090_e21211).max(0.0);
            let assign14090_e21214: f64 = (2.0 * assign14090_e21213);
            locals.var_nuintd = assign14090_e21214;
        }

        if ((locals.var_guard462 != 0.0) && (locals.var_guard463 != 0.0)) {
            locals.var_nuints = locals.var_nuintd;
        }

        let assign14110_e21225: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard464 = assign14110_e21225;

        if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 != 0.0)) {
            locals.var_nuendd = 2.0;
        }

        if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 != 0.0)) {
            let assign14130_e21244: f64 = (p.p2 / 2.0);
            let assign14130_e21246: f64 = (assign14130_e21244 - 1.0);
            let assign14130_e21248: f64 = (assign14130_e21246).max(0.0);
            let assign14130_e21249: f64 = (2.0 * assign14130_e21248);
            locals.var_nuintd = assign14130_e21249;
        }

        if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 != 0.0)) {
            locals.var_nuends = 0.0;
            locals.var_nuints = p.p2;
        }

        if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 == 0.0)) {
            locals.var_nuendd = 0.0;
            locals.var_nuintd = p.p2;
            locals.var_nuends = 2.0;
        }

        if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 == 0.0)) {
            let assign14190_e21310: f64 = (p.p2 / 2.0);
            let assign14190_e21312: f64 = (assign14190_e21310 - 1.0);
            let assign14190_e21314: f64 = (assign14190_e21312).max(0.0);
            let assign14190_e21315: f64 = (2.0 * assign14190_e21314);
            locals.var_nuints = assign14190_e21315;
        }

        let assign14200_e21320: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14, ) = (assign14200_e21320, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign14210_e21323: f64 = (locals.var_dmcgeff + locals.var_dmcgeff);
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, ) = (assign14210_e21323, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign14220_e21326: f64 = (locals.var_dmdgeff + locals.var_dmdgeff);
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, ) = (assign14220_e21326, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign14230_e21329: f64 = (locals.var_t0 + locals.var_t0);
        let assign14230_e21331: f64 = (assign14230_e21329 + locals.var_weffcj);
        (locals.var_psiso, locals.var_psiso_dn0, locals.var_psiso_dn2, locals.var_psiso_dn3, locals.var_psiso_dn4, locals.var_psiso_dn5, locals.var_psiso_dn6, locals.var_psiso_dn7, locals.var_psiso_dn8, locals.var_psiso_dn9, locals.var_psiso_dn10, locals.var_psiso_dn11, locals.var_psiso_dn12, locals.var_psiso_dn13, locals.var_psiso_dn14, ) = (assign14230_e21331, (locals.var_t0_dn0 + locals.var_t0_dn0), (locals.var_t0_dn2 + locals.var_t0_dn2), (locals.var_t0_dn3 + locals.var_t0_dn3), (locals.var_t0_dn4 + locals.var_t0_dn4), (locals.var_t0_dn5 + locals.var_t0_dn5), (locals.var_t0_dn6 + locals.var_t0_dn6), (locals.var_t0_dn7 + locals.var_t0_dn7), (locals.var_t0_dn8 + locals.var_t0_dn8), (locals.var_t0_dn9 + locals.var_t0_dn9), (locals.var_t0_dn10 + locals.var_t0_dn10), (locals.var_t0_dn11 + locals.var_t0_dn11), (locals.var_t0_dn12 + locals.var_t0_dn12), (locals.var_t0_dn13 + locals.var_t0_dn13), (locals.var_t0_dn14 + locals.var_t0_dn14), );

        let assign14240_e21334: f64 = (locals.var_t0 + locals.var_t0);
        let assign14240_e21336: f64 = (assign14240_e21334 + locals.var_weffcj);
        (locals.var_pdiso, locals.var_pdiso_dn0, locals.var_pdiso_dn2, locals.var_pdiso_dn3, locals.var_pdiso_dn4, locals.var_pdiso_dn5, locals.var_pdiso_dn6, locals.var_pdiso_dn7, locals.var_pdiso_dn8, locals.var_pdiso_dn9, locals.var_pdiso_dn10, locals.var_pdiso_dn11, locals.var_pdiso_dn12, locals.var_pdiso_dn13, locals.var_pdiso_dn14, ) = (assign14240_e21336, (locals.var_t0_dn0 + locals.var_t0_dn0), (locals.var_t0_dn2 + locals.var_t0_dn2), (locals.var_t0_dn3 + locals.var_t0_dn3), (locals.var_t0_dn4 + locals.var_t0_dn4), (locals.var_t0_dn5 + locals.var_t0_dn5), (locals.var_t0_dn6 + locals.var_t0_dn6), (locals.var_t0_dn7 + locals.var_t0_dn7), (locals.var_t0_dn8 + locals.var_t0_dn8), (locals.var_t0_dn9 + locals.var_t0_dn9), (locals.var_t0_dn10 + locals.var_t0_dn10), (locals.var_t0_dn11 + locals.var_t0_dn11), (locals.var_t0_dn12 + locals.var_t0_dn12), (locals.var_t0_dn13 + locals.var_t0_dn13), (locals.var_t0_dn14 + locals.var_t0_dn14), );

        (locals.var_pssha, locals.var_pssha_dn0, locals.var_pssha_dn2, locals.var_pssha_dn3, locals.var_pssha_dn4, locals.var_pssha_dn5, locals.var_pssha_dn6, locals.var_pssha_dn7, locals.var_pssha_dn8, locals.var_pssha_dn9, locals.var_pssha_dn10, locals.var_pssha_dn11, locals.var_pssha_dn12, locals.var_pssha_dn13, locals.var_pssha_dn14, ) = (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, );

        (locals.var_pdsha, locals.var_pdsha_dn0, locals.var_pdsha_dn2, locals.var_pdsha_dn3, locals.var_pdsha_dn4, locals.var_pdsha_dn5, locals.var_pdsha_dn6, locals.var_pdsha_dn7, locals.var_pdsha_dn8, locals.var_pdsha_dn9, locals.var_pdsha_dn10, locals.var_pdsha_dn11, locals.var_pdsha_dn12, locals.var_pdsha_dn13, locals.var_pdsha_dn14, ) = (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14, );

        (locals.var_psmer, locals.var_psmer_dn0, locals.var_psmer_dn2, locals.var_psmer_dn3, locals.var_psmer_dn4, locals.var_psmer_dn5, locals.var_psmer_dn6, locals.var_psmer_dn7, locals.var_psmer_dn8, locals.var_psmer_dn9, locals.var_psmer_dn10, locals.var_psmer_dn11, locals.var_psmer_dn12, locals.var_psmer_dn13, locals.var_psmer_dn14, ) = (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, );

        (locals.var_pdmer, locals.var_pdmer_dn0, locals.var_pdmer_dn2, locals.var_pdmer_dn3, locals.var_pdmer_dn4, locals.var_pdmer_dn5, locals.var_pdmer_dn6, locals.var_pdmer_dn7, locals.var_pdmer_dn8, locals.var_pdmer_dn9, locals.var_pdmer_dn10, locals.var_pdmer_dn11, locals.var_pdmer_dn12, locals.var_pdmer_dn13, locals.var_pdmer_dn14, ) = (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14, );

        let assign14290_e21343: f64 = (locals.var_t0 * locals.var_weffcj);
        (locals.var_asiso, locals.var_asiso_dn0, locals.var_asiso_dn2, locals.var_asiso_dn3, locals.var_asiso_dn4, locals.var_asiso_dn5, locals.var_asiso_dn6, locals.var_asiso_dn7, locals.var_asiso_dn8, locals.var_asiso_dn9, locals.var_asiso_dn10, locals.var_asiso_dn11, locals.var_asiso_dn12, locals.var_asiso_dn13, locals.var_asiso_dn14, ) = (assign14290_e21343, (locals.var_t0_dn0 * locals.var_weffcj), (locals.var_t0_dn2 * locals.var_weffcj), (locals.var_t0_dn3 * locals.var_weffcj), (locals.var_t0_dn4 * locals.var_weffcj), (locals.var_t0_dn5 * locals.var_weffcj), (locals.var_t0_dn6 * locals.var_weffcj), (locals.var_t0_dn7 * locals.var_weffcj), (locals.var_t0_dn8 * locals.var_weffcj), (locals.var_t0_dn9 * locals.var_weffcj), (locals.var_t0_dn10 * locals.var_weffcj), (locals.var_t0_dn11 * locals.var_weffcj), (locals.var_t0_dn12 * locals.var_weffcj), (locals.var_t0_dn13 * locals.var_weffcj), (locals.var_t0_dn14 * locals.var_weffcj), );

        let assign14300_e21346: f64 = (locals.var_t0 * locals.var_weffcj);
        (locals.var_adiso, locals.var_adiso_dn0, locals.var_adiso_dn2, locals.var_adiso_dn3, locals.var_adiso_dn4, locals.var_adiso_dn5, locals.var_adiso_dn6, locals.var_adiso_dn7, locals.var_adiso_dn8, locals.var_adiso_dn9, locals.var_adiso_dn10, locals.var_adiso_dn11, locals.var_adiso_dn12, locals.var_adiso_dn13, locals.var_adiso_dn14, ) = (assign14300_e21346, (locals.var_t0_dn0 * locals.var_weffcj), (locals.var_t0_dn2 * locals.var_weffcj), (locals.var_t0_dn3 * locals.var_weffcj), (locals.var_t0_dn4 * locals.var_weffcj), (locals.var_t0_dn5 * locals.var_weffcj), (locals.var_t0_dn6 * locals.var_weffcj), (locals.var_t0_dn7 * locals.var_weffcj), (locals.var_t0_dn8 * locals.var_weffcj), (locals.var_t0_dn9 * locals.var_weffcj), (locals.var_t0_dn10 * locals.var_weffcj), (locals.var_t0_dn11 * locals.var_weffcj), (locals.var_t0_dn12 * locals.var_weffcj), (locals.var_t0_dn13 * locals.var_weffcj), (locals.var_t0_dn14 * locals.var_weffcj), );

        let assign14310_e21349: f64 = (locals.var_dmcgeff * locals.var_weffcj);
        locals.var_assha = assign14310_e21349;

        let assign14320_e21352: f64 = (locals.var_dmcgeff * locals.var_weffcj);
        locals.var_adsha = assign14320_e21352;

        let assign14330_e21355: f64 = (locals.var_dmdgeff * locals.var_weffcj);
        locals.var_asmer = assign14330_e21355;

        let assign14340_e21358: f64 = (locals.var_dmdgeff * locals.var_weffcj);
        locals.var_admer = assign14340_e21358;

        let assign14350_e21361: f64 = if p.p9 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard465 = assign14350_e21361;

        let assign14360_e21364: f64 = if p.p9 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard466 = assign14360_e21364;

        let assign14370_e21367: f64 = if p.p9 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard467 = assign14370_e21367;

        let assign14380_e21370: f64 = if p.p9 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard468 = assign14380_e21370;

        let assign14390_e21373: f64 = if p.p9 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard469 = assign14390_e21373;

        let assign14400_e21376: f64 = if p.p9 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard470 = assign14400_e21376;

        let assign14410_e21379: f64 = if p.p9 == 6.0 { 1.0 } else { 0.0 };
        locals.var_guard471 = assign14410_e21379;

        let assign14420_e21382: f64 = if p.p9 == 7.0 { 1.0 } else { 0.0 };
        locals.var_guard472 = assign14420_e21382;

        let assign14430_e21385: f64 = if p.p9 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard473 = assign14430_e21385;

        let assign14440_e21388: f64 = if p.p9 == 9.0 { 1.0 } else { 0.0 };
        locals.var_guard474 = assign14440_e21388;

        let assign14450_e21391: f64 = if p.p9 == 10.0 { 1.0 } else { 0.0 };
        locals.var_guard475 = assign14450_e21391;

        if (locals.var_guard465 != 0.0) {
            let assign14460_e21395: f64 = (locals.var_nuends * locals.var_psiso);
            let assign14460_e21398: f64 = (locals.var_nuints * locals.var_pssha);
            let assign14460_e21399: f64 = (assign14460_e21395 + assign14460_e21398);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14460_e21399, ((locals.var_nuends * locals.var_psiso_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psiso_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psiso_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psiso_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psiso_dn14) + (locals.var_nuints * locals.var_pssha_dn14)), );
        }

        if (locals.var_guard465 != 0.0) {
            let assign14470_e21405: f64 = (locals.var_nuendd * locals.var_pdiso);
            let assign14470_e21408: f64 = (locals.var_nuintd * locals.var_pdsha);
            let assign14470_e21409: f64 = (assign14470_e21405 + assign14470_e21408);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14470_e21409, ((locals.var_nuendd * locals.var_pdiso_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdiso_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdiso_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdiso_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdiso_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)), );
        }

        if (locals.var_guard465 != 0.0) {
            let assign14480_e21415: f64 = (locals.var_nuends * locals.var_asiso);
            let assign14480_e21418: f64 = (locals.var_nuints * locals.var_assha);
            let assign14480_e21419: f64 = (assign14480_e21415 + assign14480_e21418);
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (assign14480_e21419, (locals.var_nuends * locals.var_asiso_dn0), (locals.var_nuends * locals.var_asiso_dn2), (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11), (locals.var_nuends * locals.var_asiso_dn12), (locals.var_nuends * locals.var_asiso_dn13), (locals.var_nuends * locals.var_asiso_dn14), );
        }

        if (locals.var_guard465 != 0.0) {
            let assign14490_e21425: f64 = (locals.var_nuendd * locals.var_adiso);
            let assign14490_e21428: f64 = (locals.var_nuintd * locals.var_adsha);
            let assign14490_e21429: f64 = (assign14490_e21425 + assign14490_e21428);
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (assign14490_e21429, (locals.var_nuendd * locals.var_adiso_dn0), (locals.var_nuendd * locals.var_adiso_dn2), (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11), (locals.var_nuendd * locals.var_adiso_dn12), (locals.var_nuendd * locals.var_adiso_dn13), (locals.var_nuendd * locals.var_adiso_dn14), );
        }

        if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
            let assign14500_e21438: f64 = (locals.var_nuends * locals.var_psiso);
            let assign14500_e21441: f64 = (locals.var_nuints * locals.var_pssha);
            let assign14500_e21442: f64 = (assign14500_e21438 + assign14500_e21441);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14500_e21442, ((locals.var_nuends * locals.var_psiso_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psiso_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psiso_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psiso_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psiso_dn14) + (locals.var_nuints * locals.var_pssha_dn14)), );
        }

        if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
            let assign14510_e21451: f64 = (locals.var_nuendd + locals.var_nuintd);
            let assign14510_e21453: f64 = (assign14510_e21451 * locals.var_pdsha);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14510_e21453, (assign14510_e21451 * locals.var_pdsha_dn0), (assign14510_e21451 * locals.var_pdsha_dn2), (assign14510_e21451 * locals.var_pdsha_dn3), (assign14510_e21451 * locals.var_pdsha_dn4), (assign14510_e21451 * locals.var_pdsha_dn5), (assign14510_e21451 * locals.var_pdsha_dn6), (assign14510_e21451 * locals.var_pdsha_dn7), (assign14510_e21451 * locals.var_pdsha_dn8), (assign14510_e21451 * locals.var_pdsha_dn9), (assign14510_e21451 * locals.var_pdsha_dn10), (assign14510_e21451 * locals.var_pdsha_dn11), (assign14510_e21451 * locals.var_pdsha_dn12), (assign14510_e21451 * locals.var_pdsha_dn13), (assign14510_e21451 * locals.var_pdsha_dn14), );
        }

        if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
            let assign14520_e21462: f64 = (locals.var_nuends * locals.var_asiso);
            let assign14520_e21465: f64 = (locals.var_nuints * locals.var_assha);
            let assign14520_e21466: f64 = (assign14520_e21462 + assign14520_e21465);
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (assign14520_e21466, (locals.var_nuends * locals.var_asiso_dn0), (locals.var_nuends * locals.var_asiso_dn2), (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11), (locals.var_nuends * locals.var_asiso_dn12), (locals.var_nuends * locals.var_asiso_dn13), (locals.var_nuends * locals.var_asiso_dn14), );
        }

        if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
            let assign14530_e21475: f64 = (locals.var_nuendd + locals.var_nuintd);
            let assign14530_e21477: f64 = (assign14530_e21475 * locals.var_adsha);
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (assign14530_e21477, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
            let assign14540_e21488: f64 = (locals.var_nuends + locals.var_nuints);
            let assign14540_e21490: f64 = (assign14540_e21488 * locals.var_pssha);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14540_e21490, (assign14540_e21488 * locals.var_pssha_dn0), (assign14540_e21488 * locals.var_pssha_dn2), (assign14540_e21488 * locals.var_pssha_dn3), (assign14540_e21488 * locals.var_pssha_dn4), (assign14540_e21488 * locals.var_pssha_dn5), (assign14540_e21488 * locals.var_pssha_dn6), (assign14540_e21488 * locals.var_pssha_dn7), (assign14540_e21488 * locals.var_pssha_dn8), (assign14540_e21488 * locals.var_pssha_dn9), (assign14540_e21488 * locals.var_pssha_dn10), (assign14540_e21488 * locals.var_pssha_dn11), (assign14540_e21488 * locals.var_pssha_dn12), (assign14540_e21488 * locals.var_pssha_dn13), (assign14540_e21488 * locals.var_pssha_dn14), );
        }

        if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
            let assign14550_e21501: f64 = (locals.var_nuendd * locals.var_pdiso);
            let assign14550_e21504: f64 = (locals.var_nuintd * locals.var_pdsha);
            let assign14550_e21505: f64 = (assign14550_e21501 + assign14550_e21504);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14550_e21505, ((locals.var_nuendd * locals.var_pdiso_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdiso_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdiso_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdiso_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdiso_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)), );
        }

        if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
            let assign14560_e21516: f64 = (locals.var_nuends + locals.var_nuints);
            let assign14560_e21518: f64 = (assign14560_e21516 * locals.var_assha);
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (assign14560_e21518, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
            let assign14570_e21529: f64 = (locals.var_nuendd * locals.var_adiso);
            let assign14570_e21532: f64 = (locals.var_nuintd * locals.var_adsha);
            let assign14570_e21533: f64 = (assign14570_e21529 + assign14570_e21532);
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (assign14570_e21533, (locals.var_nuendd * locals.var_adiso_dn0), (locals.var_nuendd * locals.var_adiso_dn2), (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11), (locals.var_nuendd * locals.var_adiso_dn12), (locals.var_nuendd * locals.var_adiso_dn13), (locals.var_nuendd * locals.var_adiso_dn14), );
        }

        if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
            let assign14580_e21546: f64 = (locals.var_nuends + locals.var_nuints);
            let assign14580_e21548: f64 = (assign14580_e21546 * locals.var_pssha);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14580_e21548, (assign14580_e21546 * locals.var_pssha_dn0), (assign14580_e21546 * locals.var_pssha_dn2), (assign14580_e21546 * locals.var_pssha_dn3), (assign14580_e21546 * locals.var_pssha_dn4), (assign14580_e21546 * locals.var_pssha_dn5), (assign14580_e21546 * locals.var_pssha_dn6), (assign14580_e21546 * locals.var_pssha_dn7), (assign14580_e21546 * locals.var_pssha_dn8), (assign14580_e21546 * locals.var_pssha_dn9), (assign14580_e21546 * locals.var_pssha_dn10), (assign14580_e21546 * locals.var_pssha_dn11), (assign14580_e21546 * locals.var_pssha_dn12), (assign14580_e21546 * locals.var_pssha_dn13), (assign14580_e21546 * locals.var_pssha_dn14), );
        }

        if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
            let assign14590_e21561: f64 = (locals.var_nuendd + locals.var_nuintd);
            let assign14590_e21563: f64 = (assign14590_e21561 * locals.var_pdsha);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14590_e21563, (assign14590_e21561 * locals.var_pdsha_dn0), (assign14590_e21561 * locals.var_pdsha_dn2), (assign14590_e21561 * locals.var_pdsha_dn3), (assign14590_e21561 * locals.var_pdsha_dn4), (assign14590_e21561 * locals.var_pdsha_dn5), (assign14590_e21561 * locals.var_pdsha_dn6), (assign14590_e21561 * locals.var_pdsha_dn7), (assign14590_e21561 * locals.var_pdsha_dn8), (assign14590_e21561 * locals.var_pdsha_dn9), (assign14590_e21561 * locals.var_pdsha_dn10), (assign14590_e21561 * locals.var_pdsha_dn11), (assign14590_e21561 * locals.var_pdsha_dn12), (assign14590_e21561 * locals.var_pdsha_dn13), (assign14590_e21561 * locals.var_pdsha_dn14), );
        }

        if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
            let assign14600_e21576: f64 = (locals.var_nuends + locals.var_nuints);
            let assign14600_e21578: f64 = (assign14600_e21576 * locals.var_assha);
            (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14, ) = (assign14600_e21578, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
            let assign14610_e21591: f64 = (locals.var_nuendd + locals.var_nuintd);
            let assign14610_e21593: f64 = (assign14610_e21591 * locals.var_adsha);
            (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14, ) = (assign14610_e21593, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard469 != 0.0) && (!((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
            let assign14620_e21608: f64 = (locals.var_nuends * locals.var_psiso);
            let assign14620_e21611: f64 = (locals.var_nuints * locals.var_pssha);
            let assign14620_e21612: f64 = (assign14620_e21608 + assign14620_e21611);
            (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14, ) = (assign14620_e21612, ((locals.var_nuends * locals.var_psiso_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psiso_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psiso_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psiso_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psiso_dn14) + (locals.var_nuints * locals.var_pssha_dn14)), );
        }

        if ((locals.var_guard469 != 0.0) && (!((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
            let assign14630_e21627: f64 = (locals.var_nuendd * locals.var_pdmer);
            let assign14630_e21630: f64 = (locals.var_nuintd * locals.var_pdsha);
            let assign14630_e21631: f64 = (assign14630_e21627 + assign14630_e21630);
            (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14, ) = (assign14630_e21631, ((locals.var_nuendd * locals.var_pdmer_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdmer_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdmer_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdmer_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdmer_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)), );
        }

    }
}
