#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        p: &Parameters,
        var_alpha_slot: &mut f64,
        var_alpha_dn0_slot: &mut f64,
        var_alpha_dn10_slot: &mut f64,
        var_alpha_dn11_slot: &mut f64,
        var_alpha_dn12_slot: &mut f64,
        var_alpha_dn17_slot: &mut f64,
        var_alpha_dn2_slot: &mut f64,
        var_alpha_dn6_slot: &mut f64,
        var_alpha_dn7_slot: &mut f64,
        var_betawl_slot: &mut f64,
        var_betawl_dn0_slot: &mut f64,
        var_betawl_dn10_slot: &mut f64,
        var_betawl_dn11_slot: &mut f64,
        var_betawl_dn12_slot: &mut f64,
        var_betawl_dn17_slot: &mut f64,
        var_betawl_dn2_slot: &mut f64,
        var_betawl_dn6_slot: &mut f64,
        var_betawl_dn7_slot: &mut f64,
        var_end_of_part_1_slot: &mut f64,
        var_ey_slot: &mut f64,
        var_ey_dn0_slot: &mut f64,
        var_ey_dn10_slot: &mut f64,
        var_ey_dn11_slot: &mut f64,
        var_ey_dn12_slot: &mut f64,
        var_ey_dn17_slot: &mut f64,
        var_ey_dn2_slot: &mut f64,
        var_ey_dn6_slot: &mut f64,
        var_ey_dn7_slot: &mut f64,
        var_fb_slot: &mut f64,
        var_fb_dn0_slot: &mut f64,
        var_fb_dn10_slot: &mut f64,
        var_fb_dn11_slot: &mut f64,
        var_fb_dn12_slot: &mut f64,
        var_fb_dn17_slot: &mut f64,
        var_fb_dn2_slot: &mut f64,
        var_fb_dn6_slot: &mut f64,
        var_fb_dn7_slot: &mut f64,
        var_flg_ign_slot: &mut f64,
        var_flg_noqi_slot: &mut f64,
        var_flg_zone_slot: &mut f64,
        var_gds0_ign_slot: &mut f64,
        var_gds0_ign_dn0_slot: &mut f64,
        var_gds0_ign_dn10_slot: &mut f64,
        var_gds0_ign_dn11_slot: &mut f64,
        var_gds0_ign_dn12_slot: &mut f64,
        var_gds0_ign_dn17_slot: &mut f64,
        var_gds0_ign_dn2_slot: &mut f64,
        var_gds0_ign_dn6_slot: &mut f64,
        var_gds0_ign_dn7_slot: &mut f64,
        var_ibd_slot: &mut f64,
        var_ibd_dn0_slot: &mut f64,
        var_ibd_dn10_slot: &mut f64,
        var_ibd_dn11_slot: &mut f64,
        var_ibd_dn12_slot: &mut f64,
        var_ibd_dn17_slot: &mut f64,
        var_ibd_dn2_slot: &mut f64,
        var_ibd_dn6_slot: &mut f64,
        var_ibd_dn7_slot: &mut f64,
        var_ibs_slot: &mut f64,
        var_ibs_dn0_slot: &mut f64,
        var_ibs_dn10_slot: &mut f64,
        var_ibs_dn11_slot: &mut f64,
        var_ibs_dn12_slot: &mut f64,
        var_ibs_dn17_slot: &mut f64,
        var_ibs_dn2_slot: &mut f64,
        var_ibs_dn6_slot: &mut f64,
        var_ibs_dn7_slot: &mut f64,
        var_idd_slot: &mut f64,
        var_idd_dn0_slot: &mut f64,
        var_idd_dn10_slot: &mut f64,
        var_idd_dn11_slot: &mut f64,
        var_idd_dn12_slot: &mut f64,
        var_idd_dn17_slot: &mut f64,
        var_idd_dn2_slot: &mut f64,
        var_idd_dn6_slot: &mut f64,
        var_idd_dn7_slot: &mut f64,
        var_ids_slot: &mut f64,
        var_ids_dn0_slot: &mut f64,
        var_ids_dn10_slot: &mut f64,
        var_ids_dn11_slot: &mut f64,
        var_ids_dn12_slot: &mut f64,
        var_ids_dn17_slot: &mut f64,
        var_ids_dn2_slot: &mut f64,
        var_ids_dn6_slot: &mut f64,
        var_ids_dn7_slot: &mut f64,
        var_idsibpc_slot: &mut f64,
        var_idsibpc_dn0_slot: &mut f64,
        var_idsibpc_dn10_slot: &mut f64,
        var_idsibpc_dn11_slot: &mut f64,
        var_idsibpc_dn12_slot: &mut f64,
        var_idsibpc_dn17_slot: &mut f64,
        var_idsibpc_dn2_slot: &mut f64,
        var_idsibpc_dn6_slot: &mut f64,
        var_idsibpc_dn7_slot: &mut f64,
        var_isub_slot: &mut f64,
        var_isub_dn0_slot: &mut f64,
        var_isub_dn10_slot: &mut f64,
        var_isub_dn11_slot: &mut f64,
        var_isub_dn12_slot: &mut f64,
        var_isub_dn17_slot: &mut f64,
        var_isub_dn2_slot: &mut f64,
        var_isub_dn6_slot: &mut f64,
        var_isub_dn7_slot: &mut f64,
        var_mu_slot: &mut f64,
        var_mu_dn0_slot: &mut f64,
        var_mu_dn10_slot: &mut f64,
        var_mu_dn11_slot: &mut f64,
        var_mu_dn12_slot: &mut f64,
        var_mu_dn17_slot: &mut f64,
        var_mu_dn2_slot: &mut f64,
        var_mu_dn6_slot: &mut f64,
        var_mu_dn7_slot: &mut f64,
        var_muun_slot: &mut f64,
        var_muun_dn0_slot: &mut f64,
        var_muun_dn10_slot: &mut f64,
        var_muun_dn11_slot: &mut f64,
        var_muun_dn12_slot: &mut f64,
        var_muun_dn17_slot: &mut f64,
        var_muun_dn2_slot: &mut f64,
        var_muun_dn6_slot: &mut f64,
        var_muun_dn7_slot: &mut f64,
        var_pds_slot: &mut f64,
        var_pds_dn0_slot: &mut f64,
        var_pds_dn10_slot: &mut f64,
        var_pds_dn11_slot: &mut f64,
        var_pds_dn12_slot: &mut f64,
        var_pds_dn17_slot: &mut f64,
        var_pds_dn2_slot: &mut f64,
        var_pds_dn6_slot: &mut f64,
        var_pds_dn7_slot: &mut f64,
        var_pds_ini_slot: &mut f64,
        var_pds_ini_dn0_slot: &mut f64,
        var_pds_ini_dn10_slot: &mut f64,
        var_pds_ini_dn11_slot: &mut f64,
        var_pds_ini_dn12_slot: &mut f64,
        var_pds_ini_dn17_slot: &mut f64,
        var_pds_ini_dn2_slot: &mut f64,
        var_pds_ini_dn6_slot: &mut f64,
        var_pds_ini_dn7_slot: &mut f64,
        var_ps0z_slot: &mut f64,
        var_ps0z_dn0_slot: &mut f64,
        var_ps0z_dn10_slot: &mut f64,
        var_ps0z_dn11_slot: &mut f64,
        var_ps0z_dn12_slot: &mut f64,
        var_ps0z_dn17_slot: &mut f64,
        var_ps0z_dn2_slot: &mut f64,
        var_ps0z_dn6_slot: &mut f64,
        var_ps0z_dn7_slot: &mut f64,
        var_psl_slot: &mut f64,
        var_psl_dn0_slot: &mut f64,
        var_psl_dn10_slot: &mut f64,
        var_psl_dn11_slot: &mut f64,
        var_psl_dn12_slot: &mut f64,
        var_psl_dn17_slot: &mut f64,
        var_psl_dn2_slot: &mut f64,
        var_psl_dn6_slot: &mut f64,
        var_psl_dn7_slot: &mut f64,
        var_psl_lim_slot: &mut f64,
        var_psl_lim_dn0_slot: &mut f64,
        var_psl_lim_dn10_slot: &mut f64,
        var_psl_lim_dn11_slot: &mut f64,
        var_psl_lim_dn12_slot: &mut f64,
        var_psl_lim_dn17_slot: &mut f64,
        var_psl_lim_dn2_slot: &mut f64,
        var_psl_lim_dn6_slot: &mut f64,
        var_psl_lim_dn7_slot: &mut f64,
        var_q_bt_ge_slot: &mut f64,
        var_q_bt_ge_dn0_slot: &mut f64,
        var_q_bt_ge_dn10_slot: &mut f64,
        var_q_bt_ge_dn11_slot: &mut f64,
        var_q_bt_ge_dn12_slot: &mut f64,
        var_q_bt_ge_dn17_slot: &mut f64,
        var_q_bt_ge_dn2_slot: &mut f64,
        var_q_bt_ge_dn6_slot: &mut f64,
        var_q_bt_ge_dn7_slot: &mut f64,
        var_q_bt_se_slot: &mut f64,
        var_q_bt_se_dn0_slot: &mut f64,
        var_q_bt_se_dn10_slot: &mut f64,
        var_q_bt_se_dn11_slot: &mut f64,
        var_q_bt_se_dn12_slot: &mut f64,
        var_q_bt_se_dn17_slot: &mut f64,
        var_q_bt_se_dn2_slot: &mut f64,
        var_q_bt_se_dn6_slot: &mut f64,
        var_q_bt_se_dn7_slot: &mut f64,
        var_qb_slot: &mut f64,
        var_qb_dn0_slot: &mut f64,
        var_qb_dn10_slot: &mut f64,
        var_qb_dn11_slot: &mut f64,
        var_qb_dn12_slot: &mut f64,
        var_qb_dn13_slot: &mut f64,
        var_qb_dn15_slot: &mut f64,
        var_qb_dn16_slot: &mut f64,
        var_qb_dn17_slot: &mut f64,
        var_qb_dn18_slot: &mut f64,
        var_qb_dn2_slot: &mut f64,
        var_qb_dn6_slot: &mut f64,
        var_qb_dn7_slot: &mut f64,
        var_qbd_slot: &mut f64,
        var_qbd_dn0_slot: &mut f64,
        var_qbd_dn10_slot: &mut f64,
        var_qbd_dn11_slot: &mut f64,
        var_qbd_dn12_slot: &mut f64,
        var_qbd_dn17_slot: &mut f64,
        var_qbd_dn2_slot: &mut f64,
        var_qbd_dn6_slot: &mut f64,
        var_qbd_dn7_slot: &mut f64,
        var_qbdld_slot: &mut f64,
        var_qbdld_dn0_slot: &mut f64,
        var_qbdld_dn10_slot: &mut f64,
        var_qbdld_dn11_slot: &mut f64,
        var_qbdld_dn12_slot: &mut f64,
        var_qbdld_dn17_slot: &mut f64,
        var_qbdld_dn2_slot: &mut f64,
        var_qbdld_dn6_slot: &mut f64,
        var_qbdld_dn7_slot: &mut f64,
        var_qbody_bt_n_iud_slot: &mut f64,
        var_qbody_bt_n_iud_dn0_slot: &mut f64,
        var_qbody_bt_n_iud_dn10_slot: &mut f64,
        var_qbody_bt_n_iud_dn11_slot: &mut f64,
        var_qbody_bt_n_iud_dn12_slot: &mut f64,
        var_qbody_bt_n_iud_dn17_slot: &mut f64,
        var_qbody_bt_n_iud_dn2_slot: &mut f64,
        var_qbody_bt_n_iud_dn6_slot: &mut f64,
        var_qbody_bt_n_iud_dn7_slot: &mut f64,
        var_qbody_bt_n_ius_slot: &mut f64,
        var_qbody_bt_n_ius_dn0_slot: &mut f64,
        var_qbody_bt_n_ius_dn10_slot: &mut f64,
        var_qbody_bt_n_ius_dn11_slot: &mut f64,
        var_qbody_bt_n_ius_dn12_slot: &mut f64,
        var_qbody_bt_n_ius_dn17_slot: &mut f64,
        var_qbody_bt_n_ius_dn2_slot: &mut f64,
        var_qbody_bt_n_ius_dn6_slot: &mut f64,
        var_qbody_bt_n_ius_dn7_slot: &mut f64,
        var_qbody_bt_n_sud_slot: &mut f64,
        var_qbody_bt_n_sud_dn0_slot: &mut f64,
        var_qbody_bt_n_sud_dn10_slot: &mut f64,
        var_qbody_bt_n_sud_dn11_slot: &mut f64,
        var_qbody_bt_n_sud_dn12_slot: &mut f64,
        var_qbody_bt_n_sud_dn17_slot: &mut f64,
        var_qbody_bt_n_sud_dn2_slot: &mut f64,
        var_qbody_bt_n_sud_dn6_slot: &mut f64,
        var_qbody_bt_n_sud_dn7_slot: &mut f64,
        var_qbody_bt_n_sus_slot: &mut f64,
        var_qbody_bt_n_sus_dn0_slot: &mut f64,
        var_qbody_bt_n_sus_dn10_slot: &mut f64,
        var_qbody_bt_n_sus_dn11_slot: &mut f64,
        var_qbody_bt_n_sus_dn12_slot: &mut f64,
        var_qbody_bt_n_sus_dn17_slot: &mut f64,
        var_qbody_bt_n_sus_dn2_slot: &mut f64,
        var_qbody_bt_n_sus_dn6_slot: &mut f64,
        var_qbody_bt_n_sus_dn7_slot: &mut f64,
        var_qbody_bt_p_iud_slot: &mut f64,
        var_qbody_bt_p_iud_dn0_slot: &mut f64,
        var_qbody_bt_p_iud_dn10_slot: &mut f64,
        var_qbody_bt_p_iud_dn11_slot: &mut f64,
        var_qbody_bt_p_iud_dn12_slot: &mut f64,
        var_qbody_bt_p_iud_dn17_slot: &mut f64,
        var_qbody_bt_p_iud_dn2_slot: &mut f64,
        var_qbody_bt_p_iud_dn6_slot: &mut f64,
        var_qbody_bt_p_iud_dn7_slot: &mut f64,
        var_qbody_bt_p_ius_slot: &mut f64,
        var_qbody_bt_p_ius_dn0_slot: &mut f64,
        var_qbody_bt_p_ius_dn10_slot: &mut f64,
        var_qbody_bt_p_ius_dn11_slot: &mut f64,
        var_qbody_bt_p_ius_dn12_slot: &mut f64,
        var_qbody_bt_p_ius_dn17_slot: &mut f64,
        var_qbody_bt_p_ius_dn2_slot: &mut f64,
        var_qbody_bt_p_ius_dn6_slot: &mut f64,
        var_qbody_bt_p_ius_dn7_slot: &mut f64,
        var_qbody_bt_p_sud_slot: &mut f64,
        var_qbody_bt_p_sud_dn0_slot: &mut f64,
        var_qbody_bt_p_sud_dn10_slot: &mut f64,
        var_qbody_bt_p_sud_dn11_slot: &mut f64,
        var_qbody_bt_p_sud_dn12_slot: &mut f64,
        var_qbody_bt_p_sud_dn17_slot: &mut f64,
        var_qbody_bt_p_sud_dn2_slot: &mut f64,
        var_qbody_bt_p_sud_dn6_slot: &mut f64,
        var_qbody_bt_p_sud_dn7_slot: &mut f64,
        var_qbody_bt_p_sus_slot: &mut f64,
        var_qbody_bt_p_sus_dn0_slot: &mut f64,
        var_qbody_bt_p_sus_dn10_slot: &mut f64,
        var_qbody_bt_p_sus_dn11_slot: &mut f64,
        var_qbody_bt_p_sus_dn12_slot: &mut f64,
        var_qbody_bt_p_sus_dn17_slot: &mut f64,
        var_qbody_bt_p_sus_dn2_slot: &mut f64,
        var_qbody_bt_p_sus_dn6_slot: &mut f64,
        var_qbody_bt_p_sus_dn7_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qbs_dn0_slot: &mut f64,
        var_qbs_dn10_slot: &mut f64,
        var_qbs_dn11_slot: &mut f64,
        var_qbs_dn12_slot: &mut f64,
        var_qbs_dn17_slot: &mut f64,
        var_qbs_dn2_slot: &mut f64,
        var_qbs_dn6_slot: &mut f64,
        var_qbs_dn7_slot: &mut f64,
        var_qbsld_slot: &mut f64,
        var_qbsld_dn0_slot: &mut f64,
        var_qbsld_dn10_slot: &mut f64,
        var_qbsld_dn11_slot: &mut f64,
        var_qbsld_dn12_slot: &mut f64,
        var_qbsld_dn17_slot: &mut f64,
        var_qbsld_dn2_slot: &mut f64,
        var_qbsld_dn6_slot: &mut f64,
        var_qbsld_dn7_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn0_slot: &mut f64,
        var_qd_dn10_slot: &mut f64,
        var_qd_dn11_slot: &mut f64,
        var_qd_dn12_slot: &mut f64,
        var_qd_dn13_slot: &mut f64,
        var_qd_dn15_slot: &mut f64,
        var_qd_dn16_slot: &mut f64,
        var_qd_dn17_slot: &mut f64,
        var_qd_dn18_slot: &mut f64,
        var_qd_dn2_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qgob_slot: &mut f64,
        var_qgob_dn0_slot: &mut f64,
        var_qgob_dn10_slot: &mut f64,
        var_qgob_dn11_slot: &mut f64,
        var_qgob_dn12_slot: &mut f64,
        var_qgob_dn17_slot: &mut f64,
        var_qgob_dn2_slot: &mut f64,
        var_qgob_dn6_slot: &mut f64,
        var_qgob_dn7_slot: &mut f64,
        var_qgod_slot: &mut f64,
        var_qgod_dn0_slot: &mut f64,
        var_qgod_dn10_slot: &mut f64,
        var_qgod_dn11_slot: &mut f64,
        var_qgod_dn12_slot: &mut f64,
        var_qgod_dn17_slot: &mut f64,
        var_qgod_dn2_slot: &mut f64,
        var_qgod_dn6_slot: &mut f64,
        var_qgod_dn7_slot: &mut f64,
        var_qgos_slot: &mut f64,
        var_qgos_dn0_slot: &mut f64,
        var_qgos_dn10_slot: &mut f64,
        var_qgos_dn11_slot: &mut f64,
        var_qgos_dn12_slot: &mut f64,
        var_qgos_dn17_slot: &mut f64,
        var_qgos_dn2_slot: &mut f64,
        var_qgos_dn6_slot: &mut f64,
        var_qgos_dn7_slot: &mut f64,
        var_qi_slot: &mut f64,
        var_qi_dn0_slot: &mut f64,
        var_qi_dn10_slot: &mut f64,
        var_qi_dn11_slot: &mut f64,
        var_qi_dn12_slot: &mut f64,
        var_qi_dn17_slot: &mut f64,
        var_qi_dn2_slot: &mut f64,
        var_qi_dn6_slot: &mut f64,
        var_qi_dn7_slot: &mut f64,
        var_qidn_slot: &mut f64,
        var_qidn_dn0_slot: &mut f64,
        var_qidn_dn10_slot: &mut f64,
        var_qidn_dn11_slot: &mut f64,
        var_qidn_dn12_slot: &mut f64,
        var_qidn_dn17_slot: &mut f64,
        var_qidn_dn2_slot: &mut f64,
        var_qidn_dn6_slot: &mut f64,
        var_qidn_dn7_slot: &mut f64,
        var_qinm_slot: &mut f64,
        var_qinm_dn0_slot: &mut f64,
        var_qinm_dn10_slot: &mut f64,
        var_qinm_dn11_slot: &mut f64,
        var_qinm_dn12_slot: &mut f64,
        var_qinm_dn17_slot: &mut f64,
        var_qinm_dn2_slot: &mut f64,
        var_qinm_dn6_slot: &mut f64,
        var_qinm_dn7_slot: &mut f64,
        var_qn0_slot: &mut f64,
        var_qn0_dn0_slot: &mut f64,
        var_qn0_dn10_slot: &mut f64,
        var_qn0_dn11_slot: &mut f64,
        var_qn0_dn12_slot: &mut f64,
        var_qn0_dn17_slot: &mut f64,
        var_qn0_dn2_slot: &mut f64,
        var_qn0_dn6_slot: &mut f64,
        var_qn0_dn7_slot: &mut f64,
        var_qovd_slot: &mut f64,
        var_qovd_dn0_slot: &mut f64,
        var_qovd_dn10_slot: &mut f64,
        var_qovd_dn11_slot: &mut f64,
        var_qovd_dn12_slot: &mut f64,
        var_qovd_dn17_slot: &mut f64,
        var_qovd_dn2_slot: &mut f64,
        var_qovd_dn6_slot: &mut f64,
        var_qovd_dn7_slot: &mut f64,
        var_qovs_slot: &mut f64,
        var_qovs_dn0_slot: &mut f64,
        var_qovs_dn10_slot: &mut f64,
        var_qovs_dn11_slot: &mut f64,
        var_qovs_dn12_slot: &mut f64,
        var_qovs_dn17_slot: &mut f64,
        var_qovs_dn2_slot: &mut f64,
        var_qovs_dn6_slot: &mut f64,
        var_qovs_dn7_slot: &mut f64,
        var_qse_slot: &mut f64,
        var_qse_dn0_slot: &mut f64,
        var_qse_dn10_slot: &mut f64,
        var_qse_dn11_slot: &mut f64,
        var_qse_dn12_slot: &mut f64,
        var_qse_dn13_slot: &mut f64,
        var_qse_dn15_slot: &mut f64,
        var_qse_dn16_slot: &mut f64,
        var_qse_dn17_slot: &mut f64,
        var_qse_dn18_slot: &mut f64,
        var_qse_dn2_slot: &mut f64,
        var_qse_dn6_slot: &mut f64,
        var_qse_dn7_slot: &mut f64,
        var_tau_slot: &mut f64,
        var_tau_dn0_slot: &mut f64,
        var_tau_dn10_slot: &mut f64,
        var_tau_dn11_slot: &mut f64,
        var_tau_dn12_slot: &mut f64,
        var_tau_dn17_slot: &mut f64,
        var_tau_dn2_slot: &mut f64,
        var_tau_dn6_slot: &mut f64,
        var_tau_dn7_slot: &mut f64,
        var_taub_slot: &mut f64,
        var_taub_dn0_slot: &mut f64,
        var_taub_dn10_slot: &mut f64,
        var_taub_dn11_slot: &mut f64,
        var_taub_dn12_slot: &mut f64,
        var_taub_dn17_slot: &mut f64,
        var_taub_dn2_slot: &mut f64,
        var_taub_dn6_slot: &mut f64,
        var_taub_dn7_slot: &mut f64,
        var_uc_areabt_slot: &mut f64,
        var_uc_vfbbt_slot: &mut f64,
        var_vgvt_slot: &mut f64,
        var_vgvt_dn0_slot: &mut f64,
        var_vgvt_dn10_slot: &mut f64,
        var_vgvt_dn11_slot: &mut f64,
        var_vgvt_dn12_slot: &mut f64,
        var_vgvt_dn17_slot: &mut f64,
        var_vgvt_dn2_slot: &mut f64,
        var_vgvt_dn6_slot: &mut f64,
        var_vgvt_dn7_slot: &mut f64,
        var_wdsoi_0_slot: &mut f64,
        var_xd_slot: &mut f64,
        var_xd_dn0_slot: &mut f64,
        var_xd_dn10_slot: &mut f64,
        var_xd_dn11_slot: &mut f64,
        var_xd_dn12_slot: &mut f64,
        var_xd_dn17_slot: &mut f64,
        var_xd_dn2_slot: &mut f64,
        var_xd_dn6_slot: &mut f64,
        var_xd_dn7_slot: &mut f64,
    ) {
        let mut var_alpha: f64 = *var_alpha_slot;
        let mut var_alpha_dn0: f64 = *var_alpha_dn0_slot;
        let mut var_alpha_dn10: f64 = *var_alpha_dn10_slot;
        let mut var_alpha_dn11: f64 = *var_alpha_dn11_slot;
        let mut var_alpha_dn12: f64 = *var_alpha_dn12_slot;
        let mut var_alpha_dn17: f64 = *var_alpha_dn17_slot;
        let mut var_alpha_dn2: f64 = *var_alpha_dn2_slot;
        let mut var_alpha_dn6: f64 = *var_alpha_dn6_slot;
        let mut var_alpha_dn7: f64 = *var_alpha_dn7_slot;
        let mut var_betawl: f64 = *var_betawl_slot;
        let mut var_betawl_dn0: f64 = *var_betawl_dn0_slot;
        let mut var_betawl_dn10: f64 = *var_betawl_dn10_slot;
        let mut var_betawl_dn11: f64 = *var_betawl_dn11_slot;
        let mut var_betawl_dn12: f64 = *var_betawl_dn12_slot;
        let mut var_betawl_dn17: f64 = *var_betawl_dn17_slot;
        let mut var_betawl_dn2: f64 = *var_betawl_dn2_slot;
        let mut var_betawl_dn6: f64 = *var_betawl_dn6_slot;
        let mut var_betawl_dn7: f64 = *var_betawl_dn7_slot;
        let mut var_end_of_part_1: f64 = *var_end_of_part_1_slot;
        let mut var_ey: f64 = *var_ey_slot;
        let mut var_ey_dn0: f64 = *var_ey_dn0_slot;
        let mut var_ey_dn10: f64 = *var_ey_dn10_slot;
        let mut var_ey_dn11: f64 = *var_ey_dn11_slot;
        let mut var_ey_dn12: f64 = *var_ey_dn12_slot;
        let mut var_ey_dn17: f64 = *var_ey_dn17_slot;
        let mut var_ey_dn2: f64 = *var_ey_dn2_slot;
        let mut var_ey_dn6: f64 = *var_ey_dn6_slot;
        let mut var_ey_dn7: f64 = *var_ey_dn7_slot;
        let mut var_fb: f64 = *var_fb_slot;
        let mut var_fb_dn0: f64 = *var_fb_dn0_slot;
        let mut var_fb_dn10: f64 = *var_fb_dn10_slot;
        let mut var_fb_dn11: f64 = *var_fb_dn11_slot;
        let mut var_fb_dn12: f64 = *var_fb_dn12_slot;
        let mut var_fb_dn17: f64 = *var_fb_dn17_slot;
        let mut var_fb_dn2: f64 = *var_fb_dn2_slot;
        let mut var_fb_dn6: f64 = *var_fb_dn6_slot;
        let mut var_fb_dn7: f64 = *var_fb_dn7_slot;
        let mut var_flg_ign: f64 = *var_flg_ign_slot;
        let mut var_flg_noqi: f64 = *var_flg_noqi_slot;
        let mut var_flg_zone: f64 = *var_flg_zone_slot;
        let mut var_gds0_ign: f64 = *var_gds0_ign_slot;
        let mut var_gds0_ign_dn0: f64 = *var_gds0_ign_dn0_slot;
        let mut var_gds0_ign_dn10: f64 = *var_gds0_ign_dn10_slot;
        let mut var_gds0_ign_dn11: f64 = *var_gds0_ign_dn11_slot;
        let mut var_gds0_ign_dn12: f64 = *var_gds0_ign_dn12_slot;
        let mut var_gds0_ign_dn17: f64 = *var_gds0_ign_dn17_slot;
        let mut var_gds0_ign_dn2: f64 = *var_gds0_ign_dn2_slot;
        let mut var_gds0_ign_dn6: f64 = *var_gds0_ign_dn6_slot;
        let mut var_gds0_ign_dn7: f64 = *var_gds0_ign_dn7_slot;
        let mut var_ibd: f64 = *var_ibd_slot;
        let mut var_ibd_dn0: f64 = *var_ibd_dn0_slot;
        let mut var_ibd_dn10: f64 = *var_ibd_dn10_slot;
        let mut var_ibd_dn11: f64 = *var_ibd_dn11_slot;
        let mut var_ibd_dn12: f64 = *var_ibd_dn12_slot;
        let mut var_ibd_dn17: f64 = *var_ibd_dn17_slot;
        let mut var_ibd_dn2: f64 = *var_ibd_dn2_slot;
        let mut var_ibd_dn6: f64 = *var_ibd_dn6_slot;
        let mut var_ibd_dn7: f64 = *var_ibd_dn7_slot;
        let mut var_ibs: f64 = *var_ibs_slot;
        let mut var_ibs_dn0: f64 = *var_ibs_dn0_slot;
        let mut var_ibs_dn10: f64 = *var_ibs_dn10_slot;
        let mut var_ibs_dn11: f64 = *var_ibs_dn11_slot;
        let mut var_ibs_dn12: f64 = *var_ibs_dn12_slot;
        let mut var_ibs_dn17: f64 = *var_ibs_dn17_slot;
        let mut var_ibs_dn2: f64 = *var_ibs_dn2_slot;
        let mut var_ibs_dn6: f64 = *var_ibs_dn6_slot;
        let mut var_ibs_dn7: f64 = *var_ibs_dn7_slot;
        let mut var_idd: f64 = *var_idd_slot;
        let mut var_idd_dn0: f64 = *var_idd_dn0_slot;
        let mut var_idd_dn10: f64 = *var_idd_dn10_slot;
        let mut var_idd_dn11: f64 = *var_idd_dn11_slot;
        let mut var_idd_dn12: f64 = *var_idd_dn12_slot;
        let mut var_idd_dn17: f64 = *var_idd_dn17_slot;
        let mut var_idd_dn2: f64 = *var_idd_dn2_slot;
        let mut var_idd_dn6: f64 = *var_idd_dn6_slot;
        let mut var_idd_dn7: f64 = *var_idd_dn7_slot;
        let mut var_ids: f64 = *var_ids_slot;
        let mut var_ids_dn0: f64 = *var_ids_dn0_slot;
        let mut var_ids_dn10: f64 = *var_ids_dn10_slot;
        let mut var_ids_dn11: f64 = *var_ids_dn11_slot;
        let mut var_ids_dn12: f64 = *var_ids_dn12_slot;
        let mut var_ids_dn17: f64 = *var_ids_dn17_slot;
        let mut var_ids_dn2: f64 = *var_ids_dn2_slot;
        let mut var_ids_dn6: f64 = *var_ids_dn6_slot;
        let mut var_ids_dn7: f64 = *var_ids_dn7_slot;
        let mut var_idsibpc: f64 = *var_idsibpc_slot;
        let mut var_idsibpc_dn0: f64 = *var_idsibpc_dn0_slot;
        let mut var_idsibpc_dn10: f64 = *var_idsibpc_dn10_slot;
        let mut var_idsibpc_dn11: f64 = *var_idsibpc_dn11_slot;
        let mut var_idsibpc_dn12: f64 = *var_idsibpc_dn12_slot;
        let mut var_idsibpc_dn17: f64 = *var_idsibpc_dn17_slot;
        let mut var_idsibpc_dn2: f64 = *var_idsibpc_dn2_slot;
        let mut var_idsibpc_dn6: f64 = *var_idsibpc_dn6_slot;
        let mut var_idsibpc_dn7: f64 = *var_idsibpc_dn7_slot;
        let mut var_isub: f64 = *var_isub_slot;
        let mut var_isub_dn0: f64 = *var_isub_dn0_slot;
        let mut var_isub_dn10: f64 = *var_isub_dn10_slot;
        let mut var_isub_dn11: f64 = *var_isub_dn11_slot;
        let mut var_isub_dn12: f64 = *var_isub_dn12_slot;
        let mut var_isub_dn17: f64 = *var_isub_dn17_slot;
        let mut var_isub_dn2: f64 = *var_isub_dn2_slot;
        let mut var_isub_dn6: f64 = *var_isub_dn6_slot;
        let mut var_isub_dn7: f64 = *var_isub_dn7_slot;
        let mut var_mu: f64 = *var_mu_slot;
        let mut var_mu_dn0: f64 = *var_mu_dn0_slot;
        let mut var_mu_dn10: f64 = *var_mu_dn10_slot;
        let mut var_mu_dn11: f64 = *var_mu_dn11_slot;
        let mut var_mu_dn12: f64 = *var_mu_dn12_slot;
        let mut var_mu_dn17: f64 = *var_mu_dn17_slot;
        let mut var_mu_dn2: f64 = *var_mu_dn2_slot;
        let mut var_mu_dn6: f64 = *var_mu_dn6_slot;
        let mut var_mu_dn7: f64 = *var_mu_dn7_slot;
        let mut var_muun: f64 = *var_muun_slot;
        let mut var_muun_dn0: f64 = *var_muun_dn0_slot;
        let mut var_muun_dn10: f64 = *var_muun_dn10_slot;
        let mut var_muun_dn11: f64 = *var_muun_dn11_slot;
        let mut var_muun_dn12: f64 = *var_muun_dn12_slot;
        let mut var_muun_dn17: f64 = *var_muun_dn17_slot;
        let mut var_muun_dn2: f64 = *var_muun_dn2_slot;
        let mut var_muun_dn6: f64 = *var_muun_dn6_slot;
        let mut var_muun_dn7: f64 = *var_muun_dn7_slot;
        let mut var_pds: f64 = *var_pds_slot;
        let mut var_pds_dn0: f64 = *var_pds_dn0_slot;
        let mut var_pds_dn10: f64 = *var_pds_dn10_slot;
        let mut var_pds_dn11: f64 = *var_pds_dn11_slot;
        let mut var_pds_dn12: f64 = *var_pds_dn12_slot;
        let mut var_pds_dn17: f64 = *var_pds_dn17_slot;
        let mut var_pds_dn2: f64 = *var_pds_dn2_slot;
        let mut var_pds_dn6: f64 = *var_pds_dn6_slot;
        let mut var_pds_dn7: f64 = *var_pds_dn7_slot;
        let mut var_pds_ini: f64 = *var_pds_ini_slot;
        let mut var_pds_ini_dn0: f64 = *var_pds_ini_dn0_slot;
        let mut var_pds_ini_dn10: f64 = *var_pds_ini_dn10_slot;
        let mut var_pds_ini_dn11: f64 = *var_pds_ini_dn11_slot;
        let mut var_pds_ini_dn12: f64 = *var_pds_ini_dn12_slot;
        let mut var_pds_ini_dn17: f64 = *var_pds_ini_dn17_slot;
        let mut var_pds_ini_dn2: f64 = *var_pds_ini_dn2_slot;
        let mut var_pds_ini_dn6: f64 = *var_pds_ini_dn6_slot;
        let mut var_pds_ini_dn7: f64 = *var_pds_ini_dn7_slot;
        let mut var_ps0z: f64 = *var_ps0z_slot;
        let mut var_ps0z_dn0: f64 = *var_ps0z_dn0_slot;
        let mut var_ps0z_dn10: f64 = *var_ps0z_dn10_slot;
        let mut var_ps0z_dn11: f64 = *var_ps0z_dn11_slot;
        let mut var_ps0z_dn12: f64 = *var_ps0z_dn12_slot;
        let mut var_ps0z_dn17: f64 = *var_ps0z_dn17_slot;
        let mut var_ps0z_dn2: f64 = *var_ps0z_dn2_slot;
        let mut var_ps0z_dn6: f64 = *var_ps0z_dn6_slot;
        let mut var_ps0z_dn7: f64 = *var_ps0z_dn7_slot;
        let mut var_psl: f64 = *var_psl_slot;
        let mut var_psl_dn0: f64 = *var_psl_dn0_slot;
        let mut var_psl_dn10: f64 = *var_psl_dn10_slot;
        let mut var_psl_dn11: f64 = *var_psl_dn11_slot;
        let mut var_psl_dn12: f64 = *var_psl_dn12_slot;
        let mut var_psl_dn17: f64 = *var_psl_dn17_slot;
        let mut var_psl_dn2: f64 = *var_psl_dn2_slot;
        let mut var_psl_dn6: f64 = *var_psl_dn6_slot;
        let mut var_psl_dn7: f64 = *var_psl_dn7_slot;
        let mut var_psl_lim: f64 = *var_psl_lim_slot;
        let mut var_psl_lim_dn0: f64 = *var_psl_lim_dn0_slot;
        let mut var_psl_lim_dn10: f64 = *var_psl_lim_dn10_slot;
        let mut var_psl_lim_dn11: f64 = *var_psl_lim_dn11_slot;
        let mut var_psl_lim_dn12: f64 = *var_psl_lim_dn12_slot;
        let mut var_psl_lim_dn17: f64 = *var_psl_lim_dn17_slot;
        let mut var_psl_lim_dn2: f64 = *var_psl_lim_dn2_slot;
        let mut var_psl_lim_dn6: f64 = *var_psl_lim_dn6_slot;
        let mut var_psl_lim_dn7: f64 = *var_psl_lim_dn7_slot;
        let mut var_q_bt_ge: f64 = *var_q_bt_ge_slot;
        let mut var_q_bt_ge_dn0: f64 = *var_q_bt_ge_dn0_slot;
        let mut var_q_bt_ge_dn10: f64 = *var_q_bt_ge_dn10_slot;
        let mut var_q_bt_ge_dn11: f64 = *var_q_bt_ge_dn11_slot;
        let mut var_q_bt_ge_dn12: f64 = *var_q_bt_ge_dn12_slot;
        let mut var_q_bt_ge_dn17: f64 = *var_q_bt_ge_dn17_slot;
        let mut var_q_bt_ge_dn2: f64 = *var_q_bt_ge_dn2_slot;
        let mut var_q_bt_ge_dn6: f64 = *var_q_bt_ge_dn6_slot;
        let mut var_q_bt_ge_dn7: f64 = *var_q_bt_ge_dn7_slot;
        let mut var_q_bt_se: f64 = *var_q_bt_se_slot;
        let mut var_q_bt_se_dn0: f64 = *var_q_bt_se_dn0_slot;
        let mut var_q_bt_se_dn10: f64 = *var_q_bt_se_dn10_slot;
        let mut var_q_bt_se_dn11: f64 = *var_q_bt_se_dn11_slot;
        let mut var_q_bt_se_dn12: f64 = *var_q_bt_se_dn12_slot;
        let mut var_q_bt_se_dn17: f64 = *var_q_bt_se_dn17_slot;
        let mut var_q_bt_se_dn2: f64 = *var_q_bt_se_dn2_slot;
        let mut var_q_bt_se_dn6: f64 = *var_q_bt_se_dn6_slot;
        let mut var_q_bt_se_dn7: f64 = *var_q_bt_se_dn7_slot;
        let mut var_qb: f64 = *var_qb_slot;
        let mut var_qb_dn0: f64 = *var_qb_dn0_slot;
        let mut var_qb_dn10: f64 = *var_qb_dn10_slot;
        let mut var_qb_dn11: f64 = *var_qb_dn11_slot;
        let mut var_qb_dn12: f64 = *var_qb_dn12_slot;
        let mut var_qb_dn13: f64 = *var_qb_dn13_slot;
        let mut var_qb_dn15: f64 = *var_qb_dn15_slot;
        let mut var_qb_dn16: f64 = *var_qb_dn16_slot;
        let mut var_qb_dn17: f64 = *var_qb_dn17_slot;
        let mut var_qb_dn18: f64 = *var_qb_dn18_slot;
        let mut var_qb_dn2: f64 = *var_qb_dn2_slot;
        let mut var_qb_dn6: f64 = *var_qb_dn6_slot;
        let mut var_qb_dn7: f64 = *var_qb_dn7_slot;
        let mut var_qbd: f64 = *var_qbd_slot;
        let mut var_qbd_dn0: f64 = *var_qbd_dn0_slot;
        let mut var_qbd_dn10: f64 = *var_qbd_dn10_slot;
        let mut var_qbd_dn11: f64 = *var_qbd_dn11_slot;
        let mut var_qbd_dn12: f64 = *var_qbd_dn12_slot;
        let mut var_qbd_dn17: f64 = *var_qbd_dn17_slot;
        let mut var_qbd_dn2: f64 = *var_qbd_dn2_slot;
        let mut var_qbd_dn6: f64 = *var_qbd_dn6_slot;
        let mut var_qbd_dn7: f64 = *var_qbd_dn7_slot;
        let mut var_qbdld: f64 = *var_qbdld_slot;
        let mut var_qbdld_dn0: f64 = *var_qbdld_dn0_slot;
        let mut var_qbdld_dn10: f64 = *var_qbdld_dn10_slot;
        let mut var_qbdld_dn11: f64 = *var_qbdld_dn11_slot;
        let mut var_qbdld_dn12: f64 = *var_qbdld_dn12_slot;
        let mut var_qbdld_dn17: f64 = *var_qbdld_dn17_slot;
        let mut var_qbdld_dn2: f64 = *var_qbdld_dn2_slot;
        let mut var_qbdld_dn6: f64 = *var_qbdld_dn6_slot;
        let mut var_qbdld_dn7: f64 = *var_qbdld_dn7_slot;
        let mut var_qbody_bt_n_iud: f64 = *var_qbody_bt_n_iud_slot;
        let mut var_qbody_bt_n_iud_dn0: f64 = *var_qbody_bt_n_iud_dn0_slot;
        let mut var_qbody_bt_n_iud_dn10: f64 = *var_qbody_bt_n_iud_dn10_slot;
        let mut var_qbody_bt_n_iud_dn11: f64 = *var_qbody_bt_n_iud_dn11_slot;
        let mut var_qbody_bt_n_iud_dn12: f64 = *var_qbody_bt_n_iud_dn12_slot;
        let mut var_qbody_bt_n_iud_dn17: f64 = *var_qbody_bt_n_iud_dn17_slot;
        let mut var_qbody_bt_n_iud_dn2: f64 = *var_qbody_bt_n_iud_dn2_slot;
        let mut var_qbody_bt_n_iud_dn6: f64 = *var_qbody_bt_n_iud_dn6_slot;
        let mut var_qbody_bt_n_iud_dn7: f64 = *var_qbody_bt_n_iud_dn7_slot;
        let mut var_qbody_bt_n_ius: f64 = *var_qbody_bt_n_ius_slot;
        let mut var_qbody_bt_n_ius_dn0: f64 = *var_qbody_bt_n_ius_dn0_slot;
        let mut var_qbody_bt_n_ius_dn10: f64 = *var_qbody_bt_n_ius_dn10_slot;
        let mut var_qbody_bt_n_ius_dn11: f64 = *var_qbody_bt_n_ius_dn11_slot;
        let mut var_qbody_bt_n_ius_dn12: f64 = *var_qbody_bt_n_ius_dn12_slot;
        let mut var_qbody_bt_n_ius_dn17: f64 = *var_qbody_bt_n_ius_dn17_slot;
        let mut var_qbody_bt_n_ius_dn2: f64 = *var_qbody_bt_n_ius_dn2_slot;
        let mut var_qbody_bt_n_ius_dn6: f64 = *var_qbody_bt_n_ius_dn6_slot;
        let mut var_qbody_bt_n_ius_dn7: f64 = *var_qbody_bt_n_ius_dn7_slot;
        let mut var_qbody_bt_n_sud: f64 = *var_qbody_bt_n_sud_slot;
        let mut var_qbody_bt_n_sud_dn0: f64 = *var_qbody_bt_n_sud_dn0_slot;
        let mut var_qbody_bt_n_sud_dn10: f64 = *var_qbody_bt_n_sud_dn10_slot;
        let mut var_qbody_bt_n_sud_dn11: f64 = *var_qbody_bt_n_sud_dn11_slot;
        let mut var_qbody_bt_n_sud_dn12: f64 = *var_qbody_bt_n_sud_dn12_slot;
        let mut var_qbody_bt_n_sud_dn17: f64 = *var_qbody_bt_n_sud_dn17_slot;
        let mut var_qbody_bt_n_sud_dn2: f64 = *var_qbody_bt_n_sud_dn2_slot;
        let mut var_qbody_bt_n_sud_dn6: f64 = *var_qbody_bt_n_sud_dn6_slot;
        let mut var_qbody_bt_n_sud_dn7: f64 = *var_qbody_bt_n_sud_dn7_slot;
        let mut var_qbody_bt_n_sus: f64 = *var_qbody_bt_n_sus_slot;
        let mut var_qbody_bt_n_sus_dn0: f64 = *var_qbody_bt_n_sus_dn0_slot;
        let mut var_qbody_bt_n_sus_dn10: f64 = *var_qbody_bt_n_sus_dn10_slot;
        let mut var_qbody_bt_n_sus_dn11: f64 = *var_qbody_bt_n_sus_dn11_slot;
        let mut var_qbody_bt_n_sus_dn12: f64 = *var_qbody_bt_n_sus_dn12_slot;
        let mut var_qbody_bt_n_sus_dn17: f64 = *var_qbody_bt_n_sus_dn17_slot;
        let mut var_qbody_bt_n_sus_dn2: f64 = *var_qbody_bt_n_sus_dn2_slot;
        let mut var_qbody_bt_n_sus_dn6: f64 = *var_qbody_bt_n_sus_dn6_slot;
        let mut var_qbody_bt_n_sus_dn7: f64 = *var_qbody_bt_n_sus_dn7_slot;
        let mut var_qbody_bt_p_iud: f64 = *var_qbody_bt_p_iud_slot;
        let mut var_qbody_bt_p_iud_dn0: f64 = *var_qbody_bt_p_iud_dn0_slot;
        let mut var_qbody_bt_p_iud_dn10: f64 = *var_qbody_bt_p_iud_dn10_slot;
        let mut var_qbody_bt_p_iud_dn11: f64 = *var_qbody_bt_p_iud_dn11_slot;
        let mut var_qbody_bt_p_iud_dn12: f64 = *var_qbody_bt_p_iud_dn12_slot;
        let mut var_qbody_bt_p_iud_dn17: f64 = *var_qbody_bt_p_iud_dn17_slot;
        let mut var_qbody_bt_p_iud_dn2: f64 = *var_qbody_bt_p_iud_dn2_slot;
        let mut var_qbody_bt_p_iud_dn6: f64 = *var_qbody_bt_p_iud_dn6_slot;
        let mut var_qbody_bt_p_iud_dn7: f64 = *var_qbody_bt_p_iud_dn7_slot;
        let mut var_qbody_bt_p_ius: f64 = *var_qbody_bt_p_ius_slot;
        let mut var_qbody_bt_p_ius_dn0: f64 = *var_qbody_bt_p_ius_dn0_slot;
        let mut var_qbody_bt_p_ius_dn10: f64 = *var_qbody_bt_p_ius_dn10_slot;
        let mut var_qbody_bt_p_ius_dn11: f64 = *var_qbody_bt_p_ius_dn11_slot;
        let mut var_qbody_bt_p_ius_dn12: f64 = *var_qbody_bt_p_ius_dn12_slot;
        let mut var_qbody_bt_p_ius_dn17: f64 = *var_qbody_bt_p_ius_dn17_slot;
        let mut var_qbody_bt_p_ius_dn2: f64 = *var_qbody_bt_p_ius_dn2_slot;
        let mut var_qbody_bt_p_ius_dn6: f64 = *var_qbody_bt_p_ius_dn6_slot;
        let mut var_qbody_bt_p_ius_dn7: f64 = *var_qbody_bt_p_ius_dn7_slot;
        let mut var_qbody_bt_p_sud: f64 = *var_qbody_bt_p_sud_slot;
        let mut var_qbody_bt_p_sud_dn0: f64 = *var_qbody_bt_p_sud_dn0_slot;
        let mut var_qbody_bt_p_sud_dn10: f64 = *var_qbody_bt_p_sud_dn10_slot;
        let mut var_qbody_bt_p_sud_dn11: f64 = *var_qbody_bt_p_sud_dn11_slot;
        let mut var_qbody_bt_p_sud_dn12: f64 = *var_qbody_bt_p_sud_dn12_slot;
        let mut var_qbody_bt_p_sud_dn17: f64 = *var_qbody_bt_p_sud_dn17_slot;
        let mut var_qbody_bt_p_sud_dn2: f64 = *var_qbody_bt_p_sud_dn2_slot;
        let mut var_qbody_bt_p_sud_dn6: f64 = *var_qbody_bt_p_sud_dn6_slot;
        let mut var_qbody_bt_p_sud_dn7: f64 = *var_qbody_bt_p_sud_dn7_slot;
        let mut var_qbody_bt_p_sus: f64 = *var_qbody_bt_p_sus_slot;
        let mut var_qbody_bt_p_sus_dn0: f64 = *var_qbody_bt_p_sus_dn0_slot;
        let mut var_qbody_bt_p_sus_dn10: f64 = *var_qbody_bt_p_sus_dn10_slot;
        let mut var_qbody_bt_p_sus_dn11: f64 = *var_qbody_bt_p_sus_dn11_slot;
        let mut var_qbody_bt_p_sus_dn12: f64 = *var_qbody_bt_p_sus_dn12_slot;
        let mut var_qbody_bt_p_sus_dn17: f64 = *var_qbody_bt_p_sus_dn17_slot;
        let mut var_qbody_bt_p_sus_dn2: f64 = *var_qbody_bt_p_sus_dn2_slot;
        let mut var_qbody_bt_p_sus_dn6: f64 = *var_qbody_bt_p_sus_dn6_slot;
        let mut var_qbody_bt_p_sus_dn7: f64 = *var_qbody_bt_p_sus_dn7_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qbs_dn0: f64 = *var_qbs_dn0_slot;
        let mut var_qbs_dn10: f64 = *var_qbs_dn10_slot;
        let mut var_qbs_dn11: f64 = *var_qbs_dn11_slot;
        let mut var_qbs_dn12: f64 = *var_qbs_dn12_slot;
        let mut var_qbs_dn17: f64 = *var_qbs_dn17_slot;
        let mut var_qbs_dn2: f64 = *var_qbs_dn2_slot;
        let mut var_qbs_dn6: f64 = *var_qbs_dn6_slot;
        let mut var_qbs_dn7: f64 = *var_qbs_dn7_slot;
        let mut var_qbsld: f64 = *var_qbsld_slot;
        let mut var_qbsld_dn0: f64 = *var_qbsld_dn0_slot;
        let mut var_qbsld_dn10: f64 = *var_qbsld_dn10_slot;
        let mut var_qbsld_dn11: f64 = *var_qbsld_dn11_slot;
        let mut var_qbsld_dn12: f64 = *var_qbsld_dn12_slot;
        let mut var_qbsld_dn17: f64 = *var_qbsld_dn17_slot;
        let mut var_qbsld_dn2: f64 = *var_qbsld_dn2_slot;
        let mut var_qbsld_dn6: f64 = *var_qbsld_dn6_slot;
        let mut var_qbsld_dn7: f64 = *var_qbsld_dn7_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn0: f64 = *var_qd_dn0_slot;
        let mut var_qd_dn10: f64 = *var_qd_dn10_slot;
        let mut var_qd_dn11: f64 = *var_qd_dn11_slot;
        let mut var_qd_dn12: f64 = *var_qd_dn12_slot;
        let mut var_qd_dn13: f64 = *var_qd_dn13_slot;
        let mut var_qd_dn15: f64 = *var_qd_dn15_slot;
        let mut var_qd_dn16: f64 = *var_qd_dn16_slot;
        let mut var_qd_dn17: f64 = *var_qd_dn17_slot;
        let mut var_qd_dn18: f64 = *var_qd_dn18_slot;
        let mut var_qd_dn2: f64 = *var_qd_dn2_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qgob: f64 = *var_qgob_slot;
        let mut var_qgob_dn0: f64 = *var_qgob_dn0_slot;
        let mut var_qgob_dn10: f64 = *var_qgob_dn10_slot;
        let mut var_qgob_dn11: f64 = *var_qgob_dn11_slot;
        let mut var_qgob_dn12: f64 = *var_qgob_dn12_slot;
        let mut var_qgob_dn17: f64 = *var_qgob_dn17_slot;
        let mut var_qgob_dn2: f64 = *var_qgob_dn2_slot;
        let mut var_qgob_dn6: f64 = *var_qgob_dn6_slot;
        let mut var_qgob_dn7: f64 = *var_qgob_dn7_slot;
        let mut var_qgod: f64 = *var_qgod_slot;
        let mut var_qgod_dn0: f64 = *var_qgod_dn0_slot;
        let mut var_qgod_dn10: f64 = *var_qgod_dn10_slot;
        let mut var_qgod_dn11: f64 = *var_qgod_dn11_slot;
        let mut var_qgod_dn12: f64 = *var_qgod_dn12_slot;
        let mut var_qgod_dn17: f64 = *var_qgod_dn17_slot;
        let mut var_qgod_dn2: f64 = *var_qgod_dn2_slot;
        let mut var_qgod_dn6: f64 = *var_qgod_dn6_slot;
        let mut var_qgod_dn7: f64 = *var_qgod_dn7_slot;
        let mut var_qgos: f64 = *var_qgos_slot;
        let mut var_qgos_dn0: f64 = *var_qgos_dn0_slot;
        let mut var_qgos_dn10: f64 = *var_qgos_dn10_slot;
        let mut var_qgos_dn11: f64 = *var_qgos_dn11_slot;
        let mut var_qgos_dn12: f64 = *var_qgos_dn12_slot;
        let mut var_qgos_dn17: f64 = *var_qgos_dn17_slot;
        let mut var_qgos_dn2: f64 = *var_qgos_dn2_slot;
        let mut var_qgos_dn6: f64 = *var_qgos_dn6_slot;
        let mut var_qgos_dn7: f64 = *var_qgos_dn7_slot;
        let mut var_qi: f64 = *var_qi_slot;
        let mut var_qi_dn0: f64 = *var_qi_dn0_slot;
        let mut var_qi_dn10: f64 = *var_qi_dn10_slot;
        let mut var_qi_dn11: f64 = *var_qi_dn11_slot;
        let mut var_qi_dn12: f64 = *var_qi_dn12_slot;
        let mut var_qi_dn17: f64 = *var_qi_dn17_slot;
        let mut var_qi_dn2: f64 = *var_qi_dn2_slot;
        let mut var_qi_dn6: f64 = *var_qi_dn6_slot;
        let mut var_qi_dn7: f64 = *var_qi_dn7_slot;
        let mut var_qidn: f64 = *var_qidn_slot;
        let mut var_qidn_dn0: f64 = *var_qidn_dn0_slot;
        let mut var_qidn_dn10: f64 = *var_qidn_dn10_slot;
        let mut var_qidn_dn11: f64 = *var_qidn_dn11_slot;
        let mut var_qidn_dn12: f64 = *var_qidn_dn12_slot;
        let mut var_qidn_dn17: f64 = *var_qidn_dn17_slot;
        let mut var_qidn_dn2: f64 = *var_qidn_dn2_slot;
        let mut var_qidn_dn6: f64 = *var_qidn_dn6_slot;
        let mut var_qidn_dn7: f64 = *var_qidn_dn7_slot;
        let mut var_qinm: f64 = *var_qinm_slot;
        let mut var_qinm_dn0: f64 = *var_qinm_dn0_slot;
        let mut var_qinm_dn10: f64 = *var_qinm_dn10_slot;
        let mut var_qinm_dn11: f64 = *var_qinm_dn11_slot;
        let mut var_qinm_dn12: f64 = *var_qinm_dn12_slot;
        let mut var_qinm_dn17: f64 = *var_qinm_dn17_slot;
        let mut var_qinm_dn2: f64 = *var_qinm_dn2_slot;
        let mut var_qinm_dn6: f64 = *var_qinm_dn6_slot;
        let mut var_qinm_dn7: f64 = *var_qinm_dn7_slot;
        let mut var_qn0: f64 = *var_qn0_slot;
        let mut var_qn0_dn0: f64 = *var_qn0_dn0_slot;
        let mut var_qn0_dn10: f64 = *var_qn0_dn10_slot;
        let mut var_qn0_dn11: f64 = *var_qn0_dn11_slot;
        let mut var_qn0_dn12: f64 = *var_qn0_dn12_slot;
        let mut var_qn0_dn17: f64 = *var_qn0_dn17_slot;
        let mut var_qn0_dn2: f64 = *var_qn0_dn2_slot;
        let mut var_qn0_dn6: f64 = *var_qn0_dn6_slot;
        let mut var_qn0_dn7: f64 = *var_qn0_dn7_slot;
        let mut var_qovd: f64 = *var_qovd_slot;
        let mut var_qovd_dn0: f64 = *var_qovd_dn0_slot;
        let mut var_qovd_dn10: f64 = *var_qovd_dn10_slot;
        let mut var_qovd_dn11: f64 = *var_qovd_dn11_slot;
        let mut var_qovd_dn12: f64 = *var_qovd_dn12_slot;
        let mut var_qovd_dn17: f64 = *var_qovd_dn17_slot;
        let mut var_qovd_dn2: f64 = *var_qovd_dn2_slot;
        let mut var_qovd_dn6: f64 = *var_qovd_dn6_slot;
        let mut var_qovd_dn7: f64 = *var_qovd_dn7_slot;
        let mut var_qovs: f64 = *var_qovs_slot;
        let mut var_qovs_dn0: f64 = *var_qovs_dn0_slot;
        let mut var_qovs_dn10: f64 = *var_qovs_dn10_slot;
        let mut var_qovs_dn11: f64 = *var_qovs_dn11_slot;
        let mut var_qovs_dn12: f64 = *var_qovs_dn12_slot;
        let mut var_qovs_dn17: f64 = *var_qovs_dn17_slot;
        let mut var_qovs_dn2: f64 = *var_qovs_dn2_slot;
        let mut var_qovs_dn6: f64 = *var_qovs_dn6_slot;
        let mut var_qovs_dn7: f64 = *var_qovs_dn7_slot;
        let mut var_qse: f64 = *var_qse_slot;
        let mut var_qse_dn0: f64 = *var_qse_dn0_slot;
        let mut var_qse_dn10: f64 = *var_qse_dn10_slot;
        let mut var_qse_dn11: f64 = *var_qse_dn11_slot;
        let mut var_qse_dn12: f64 = *var_qse_dn12_slot;
        let mut var_qse_dn13: f64 = *var_qse_dn13_slot;
        let mut var_qse_dn15: f64 = *var_qse_dn15_slot;
        let mut var_qse_dn16: f64 = *var_qse_dn16_slot;
        let mut var_qse_dn17: f64 = *var_qse_dn17_slot;
        let mut var_qse_dn18: f64 = *var_qse_dn18_slot;
        let mut var_qse_dn2: f64 = *var_qse_dn2_slot;
        let mut var_qse_dn6: f64 = *var_qse_dn6_slot;
        let mut var_qse_dn7: f64 = *var_qse_dn7_slot;
        let mut var_tau: f64 = *var_tau_slot;
        let mut var_tau_dn0: f64 = *var_tau_dn0_slot;
        let mut var_tau_dn10: f64 = *var_tau_dn10_slot;
        let mut var_tau_dn11: f64 = *var_tau_dn11_slot;
        let mut var_tau_dn12: f64 = *var_tau_dn12_slot;
        let mut var_tau_dn17: f64 = *var_tau_dn17_slot;
        let mut var_tau_dn2: f64 = *var_tau_dn2_slot;
        let mut var_tau_dn6: f64 = *var_tau_dn6_slot;
        let mut var_tau_dn7: f64 = *var_tau_dn7_slot;
        let mut var_taub: f64 = *var_taub_slot;
        let mut var_taub_dn0: f64 = *var_taub_dn0_slot;
        let mut var_taub_dn10: f64 = *var_taub_dn10_slot;
        let mut var_taub_dn11: f64 = *var_taub_dn11_slot;
        let mut var_taub_dn12: f64 = *var_taub_dn12_slot;
        let mut var_taub_dn17: f64 = *var_taub_dn17_slot;
        let mut var_taub_dn2: f64 = *var_taub_dn2_slot;
        let mut var_taub_dn6: f64 = *var_taub_dn6_slot;
        let mut var_taub_dn7: f64 = *var_taub_dn7_slot;
        let mut var_uc_areabt: f64 = *var_uc_areabt_slot;
        let mut var_uc_vfbbt: f64 = *var_uc_vfbbt_slot;
        let mut var_vgvt: f64 = *var_vgvt_slot;
        let mut var_vgvt_dn0: f64 = *var_vgvt_dn0_slot;
        let mut var_vgvt_dn10: f64 = *var_vgvt_dn10_slot;
        let mut var_vgvt_dn11: f64 = *var_vgvt_dn11_slot;
        let mut var_vgvt_dn12: f64 = *var_vgvt_dn12_slot;
        let mut var_vgvt_dn17: f64 = *var_vgvt_dn17_slot;
        let mut var_vgvt_dn2: f64 = *var_vgvt_dn2_slot;
        let mut var_vgvt_dn6: f64 = *var_vgvt_dn6_slot;
        let mut var_vgvt_dn7: f64 = *var_vgvt_dn7_slot;
        let mut var_wdsoi_0: f64 = *var_wdsoi_0_slot;
        let mut var_xd: f64 = *var_xd_slot;
        let mut var_xd_dn0: f64 = *var_xd_dn0_slot;
        let mut var_xd_dn10: f64 = *var_xd_dn10_slot;
        let mut var_xd_dn11: f64 = *var_xd_dn11_slot;
        let mut var_xd_dn12: f64 = *var_xd_dn12_slot;
        let mut var_xd_dn17: f64 = *var_xd_dn17_slot;
        let mut var_xd_dn2: f64 = *var_xd_dn2_slot;
        let mut var_xd_dn6: f64 = *var_xd_dn6_slot;
        let mut var_xd_dn7: f64 = *var_xd_dn7_slot;

        var_idd = 0.0;
        var_idd_dn0 = 0.0;
        var_idd_dn2 = 0.0;
        var_idd_dn6 = 0.0;
        var_idd_dn7 = 0.0;
        var_idd_dn10 = 0.0;
        var_idd_dn11 = 0.0;
        var_idd_dn12 = 0.0;
        var_idd_dn17 = 0.0;

        var_gds0_ign = 1e-12;
        var_gds0_ign_dn0 = 0.0;
        var_gds0_ign_dn2 = 0.0;
        var_gds0_ign_dn6 = 0.0;
        var_gds0_ign_dn7 = 0.0;
        var_gds0_ign_dn10 = 0.0;
        var_gds0_ign_dn11 = 0.0;
        var_gds0_ign_dn12 = 0.0;
        var_gds0_ign_dn17 = 0.0;

        var_qse = 0.0;
        var_qse_dn0 = 0.0;
        var_qse_dn2 = 0.0;
        var_qse_dn6 = 0.0;
        var_qse_dn7 = 0.0;
        var_qse_dn10 = 0.0;
        var_qse_dn11 = 0.0;
        var_qse_dn12 = 0.0;
        var_qse_dn13 = 0.0;
        var_qse_dn15 = 0.0;
        var_qse_dn16 = 0.0;
        var_qse_dn17 = 0.0;
        var_qse_dn18 = 0.0;

        var_flg_ign = 0.0;

        var_end_of_part_1 = 0.0;

        var_xd = 0.0;
        var_xd_dn0 = 0.0;
        var_xd_dn2 = 0.0;
        var_xd_dn6 = 0.0;
        var_xd_dn7 = 0.0;
        var_xd_dn10 = 0.0;
        var_xd_dn11 = 0.0;
        var_xd_dn12 = 0.0;
        var_xd_dn17 = 0.0;

        var_flg_noqi = 0.0;

        var_flg_zone = 0.0;

        var_psl = 0.0;
        var_psl_dn0 = 0.0;
        var_psl_dn2 = 0.0;
        var_psl_dn6 = 0.0;
        var_psl_dn7 = 0.0;
        var_psl_dn10 = 0.0;
        var_psl_dn11 = 0.0;
        var_psl_dn12 = 0.0;
        var_psl_dn17 = 0.0;

        var_psl_lim = 0.0;
        var_psl_lim_dn0 = 0.0;
        var_psl_lim_dn2 = 0.0;
        var_psl_lim_dn6 = 0.0;
        var_psl_lim_dn7 = 0.0;
        var_psl_lim_dn10 = 0.0;
        var_psl_lim_dn11 = 0.0;
        var_psl_lim_dn12 = 0.0;
        var_psl_lim_dn17 = 0.0;

        var_pds = 0.0;
        var_pds_dn0 = 0.0;
        var_pds_dn2 = 0.0;
        var_pds_dn6 = 0.0;
        var_pds_dn7 = 0.0;
        var_pds_dn10 = 0.0;
        var_pds_dn11 = 0.0;
        var_pds_dn12 = 0.0;
        var_pds_dn17 = 0.0;

        var_pds_ini = 0.0;
        var_pds_ini_dn0 = 0.0;
        var_pds_ini_dn2 = 0.0;
        var_pds_ini_dn6 = 0.0;
        var_pds_ini_dn7 = 0.0;
        var_pds_ini_dn10 = 0.0;
        var_pds_ini_dn11 = 0.0;
        var_pds_ini_dn12 = 0.0;
        var_pds_ini_dn17 = 0.0;

        var_ps0z = 1.0;
        var_ps0z_dn0 = 0.0;
        var_ps0z_dn2 = 0.0;
        var_ps0z_dn6 = 0.0;
        var_ps0z_dn7 = 0.0;
        var_ps0z_dn10 = 0.0;
        var_ps0z_dn11 = 0.0;
        var_ps0z_dn12 = 0.0;
        var_ps0z_dn17 = 0.0;

        var_alpha = 0.0;
        var_alpha_dn0 = 0.0;
        var_alpha_dn2 = 0.0;
        var_alpha_dn6 = 0.0;
        var_alpha_dn7 = 0.0;
        var_alpha_dn10 = 0.0;
        var_alpha_dn11 = 0.0;
        var_alpha_dn12 = 0.0;
        var_alpha_dn17 = 0.0;

        var_vgvt = 0.0;
        var_vgvt_dn0 = 0.0;
        var_vgvt_dn2 = 0.0;
        var_vgvt_dn6 = 0.0;
        var_vgvt_dn7 = 0.0;
        var_vgvt_dn10 = 0.0;
        var_vgvt_dn11 = 0.0;
        var_vgvt_dn12 = 0.0;
        var_vgvt_dn17 = 0.0;

        var_qb = 0.0;
        var_qb_dn0 = 0.0;
        var_qb_dn2 = 0.0;
        var_qb_dn6 = 0.0;
        var_qb_dn7 = 0.0;
        var_qb_dn10 = 0.0;
        var_qb_dn11 = 0.0;
        var_qb_dn12 = 0.0;
        var_qb_dn13 = 0.0;
        var_qb_dn15 = 0.0;
        var_qb_dn16 = 0.0;
        var_qb_dn17 = 0.0;
        var_qb_dn18 = 0.0;

        var_qi = 0.0;
        var_qi_dn0 = 0.0;
        var_qi_dn2 = 0.0;
        var_qi_dn6 = 0.0;
        var_qi_dn7 = 0.0;
        var_qi_dn10 = 0.0;
        var_qi_dn11 = 0.0;
        var_qi_dn12 = 0.0;
        var_qi_dn17 = 0.0;

        var_qd = 0.0;
        var_qd_dn0 = 0.0;
        var_qd_dn2 = 0.0;
        var_qd_dn6 = 0.0;
        var_qd_dn7 = 0.0;
        var_qd_dn10 = 0.0;
        var_qd_dn11 = 0.0;
        var_qd_dn12 = 0.0;
        var_qd_dn13 = 0.0;
        var_qd_dn15 = 0.0;
        var_qd_dn16 = 0.0;
        var_qd_dn17 = 0.0;
        var_qd_dn18 = 0.0;

        var_ids = 0.0;
        var_ids_dn0 = 0.0;
        var_ids_dn2 = 0.0;
        var_ids_dn6 = 0.0;
        var_ids_dn7 = 0.0;
        var_ids_dn10 = 0.0;
        var_ids_dn11 = 0.0;
        var_ids_dn12 = 0.0;
        var_ids_dn17 = 0.0;

        var_fb = 0.0;
        var_fb_dn0 = 0.0;
        var_fb_dn2 = 0.0;
        var_fb_dn6 = 0.0;
        var_fb_dn7 = 0.0;
        var_fb_dn10 = 0.0;
        var_fb_dn11 = 0.0;
        var_fb_dn12 = 0.0;
        var_fb_dn17 = 0.0;

        var_qn0 = 0.0;
        var_qn0_dn0 = 0.0;
        var_qn0_dn2 = 0.0;
        var_qn0_dn6 = 0.0;
        var_qn0_dn7 = 0.0;
        var_qn0_dn10 = 0.0;
        var_qn0_dn11 = 0.0;
        var_qn0_dn12 = 0.0;
        var_qn0_dn17 = 0.0;

        var_mu = 0.0;
        var_mu_dn0 = 0.0;
        var_mu_dn2 = 0.0;
        var_mu_dn6 = 0.0;
        var_mu_dn7 = 0.0;
        var_mu_dn10 = 0.0;
        var_mu_dn11 = 0.0;
        var_mu_dn12 = 0.0;
        var_mu_dn17 = 0.0;

        var_muun = 0.0;
        var_muun_dn0 = 0.0;
        var_muun_dn2 = 0.0;
        var_muun_dn6 = 0.0;
        var_muun_dn7 = 0.0;
        var_muun_dn10 = 0.0;
        var_muun_dn11 = 0.0;
        var_muun_dn12 = 0.0;
        var_muun_dn17 = 0.0;

        var_ey = 0.0;
        var_ey_dn0 = 0.0;
        var_ey_dn2 = 0.0;
        var_ey_dn6 = 0.0;
        var_ey_dn7 = 0.0;
        var_ey_dn10 = 0.0;
        var_ey_dn11 = 0.0;
        var_ey_dn12 = 0.0;
        var_ey_dn17 = 0.0;

        var_isub = 0.0;
        var_isub_dn0 = 0.0;
        var_isub_dn2 = 0.0;
        var_isub_dn6 = 0.0;
        var_isub_dn7 = 0.0;
        var_isub_dn10 = 0.0;
        var_isub_dn11 = 0.0;
        var_isub_dn12 = 0.0;
        var_isub_dn17 = 0.0;

        var_betawl = 1.0;
        var_betawl_dn0 = 0.0;
        var_betawl_dn2 = 0.0;
        var_betawl_dn6 = 0.0;
        var_betawl_dn7 = 0.0;
        var_betawl_dn10 = 0.0;
        var_betawl_dn11 = 0.0;
        var_betawl_dn12 = 0.0;
        var_betawl_dn17 = 0.0;

        var_idsibpc = 0.0;
        var_idsibpc_dn0 = 0.0;
        var_idsibpc_dn2 = 0.0;
        var_idsibpc_dn6 = 0.0;
        var_idsibpc_dn7 = 0.0;
        var_idsibpc_dn10 = 0.0;
        var_idsibpc_dn11 = 0.0;
        var_idsibpc_dn12 = 0.0;
        var_idsibpc_dn17 = 0.0;

        var_qgos = 0.0;
        var_qgos_dn0 = 0.0;
        var_qgos_dn2 = 0.0;
        var_qgos_dn6 = 0.0;
        var_qgos_dn7 = 0.0;
        var_qgos_dn10 = 0.0;
        var_qgos_dn11 = 0.0;
        var_qgos_dn12 = 0.0;
        var_qgos_dn17 = 0.0;

        var_qgod = 0.0;
        var_qgod_dn0 = 0.0;
        var_qgod_dn2 = 0.0;
        var_qgod_dn6 = 0.0;
        var_qgod_dn7 = 0.0;
        var_qgod_dn10 = 0.0;
        var_qgod_dn11 = 0.0;
        var_qgod_dn12 = 0.0;
        var_qgod_dn17 = 0.0;

        var_qgob = 0.0;
        var_qgob_dn0 = 0.0;
        var_qgob_dn2 = 0.0;
        var_qgob_dn6 = 0.0;
        var_qgob_dn7 = 0.0;
        var_qgob_dn10 = 0.0;
        var_qgob_dn11 = 0.0;
        var_qgob_dn12 = 0.0;
        var_qgob_dn17 = 0.0;

        var_qovd = 0.0;
        var_qovd_dn0 = 0.0;
        var_qovd_dn2 = 0.0;
        var_qovd_dn6 = 0.0;
        var_qovd_dn7 = 0.0;
        var_qovd_dn10 = 0.0;
        var_qovd_dn11 = 0.0;
        var_qovd_dn12 = 0.0;
        var_qovd_dn17 = 0.0;

        var_qovs = 0.0;
        var_qovs_dn0 = 0.0;
        var_qovs_dn2 = 0.0;
        var_qovs_dn6 = 0.0;
        var_qovs_dn7 = 0.0;
        var_qovs_dn10 = 0.0;
        var_qovs_dn11 = 0.0;
        var_qovs_dn12 = 0.0;
        var_qovs_dn17 = 0.0;

        var_qbdld = 0.0;
        var_qbdld_dn0 = 0.0;
        var_qbdld_dn2 = 0.0;
        var_qbdld_dn6 = 0.0;
        var_qbdld_dn7 = 0.0;
        var_qbdld_dn10 = 0.0;
        var_qbdld_dn11 = 0.0;
        var_qbdld_dn12 = 0.0;
        var_qbdld_dn17 = 0.0;

        var_qbsld = 0.0;
        var_qbsld_dn0 = 0.0;
        var_qbsld_dn2 = 0.0;
        var_qbsld_dn6 = 0.0;
        var_qbsld_dn7 = 0.0;
        var_qbsld_dn10 = 0.0;
        var_qbsld_dn11 = 0.0;
        var_qbsld_dn12 = 0.0;
        var_qbsld_dn17 = 0.0;

        var_ibd = 0.0;
        var_ibd_dn0 = 0.0;
        var_ibd_dn2 = 0.0;
        var_ibd_dn6 = 0.0;
        var_ibd_dn7 = 0.0;
        var_ibd_dn10 = 0.0;
        var_ibd_dn11 = 0.0;
        var_ibd_dn12 = 0.0;
        var_ibd_dn17 = 0.0;

        var_ibs = 0.0;
        var_ibs_dn0 = 0.0;
        var_ibs_dn2 = 0.0;
        var_ibs_dn6 = 0.0;
        var_ibs_dn7 = 0.0;
        var_ibs_dn10 = 0.0;
        var_ibs_dn11 = 0.0;
        var_ibs_dn12 = 0.0;
        var_ibs_dn17 = 0.0;

        var_qbd = 0.0;
        var_qbd_dn0 = 0.0;
        var_qbd_dn2 = 0.0;
        var_qbd_dn6 = 0.0;
        var_qbd_dn7 = 0.0;
        var_qbd_dn10 = 0.0;
        var_qbd_dn11 = 0.0;
        var_qbd_dn12 = 0.0;
        var_qbd_dn17 = 0.0;

        var_qbs = 0.0;
        var_qbs_dn0 = 0.0;
        var_qbs_dn2 = 0.0;
        var_qbs_dn6 = 0.0;
        var_qbs_dn7 = 0.0;
        var_qbs_dn10 = 0.0;
        var_qbs_dn11 = 0.0;
        var_qbs_dn12 = 0.0;
        var_qbs_dn17 = 0.0;

        var_qinm = 0.0;
        var_qinm_dn0 = 0.0;
        var_qinm_dn2 = 0.0;
        var_qinm_dn6 = 0.0;
        var_qinm_dn7 = 0.0;
        var_qinm_dn10 = 0.0;
        var_qinm_dn11 = 0.0;
        var_qinm_dn12 = 0.0;
        var_qinm_dn17 = 0.0;

        var_qidn = 0.0;
        var_qidn_dn0 = 0.0;
        var_qidn_dn2 = 0.0;
        var_qidn_dn6 = 0.0;
        var_qidn_dn7 = 0.0;
        var_qidn_dn10 = 0.0;
        var_qidn_dn11 = 0.0;
        var_qidn_dn12 = 0.0;
        var_qidn_dn17 = 0.0;

        var_wdsoi_0 = p.p237;

        var_qbody_bt_p_sus = 0.0;
        var_qbody_bt_p_sus_dn0 = 0.0;
        var_qbody_bt_p_sus_dn2 = 0.0;
        var_qbody_bt_p_sus_dn6 = 0.0;
        var_qbody_bt_p_sus_dn7 = 0.0;
        var_qbody_bt_p_sus_dn10 = 0.0;
        var_qbody_bt_p_sus_dn11 = 0.0;
        var_qbody_bt_p_sus_dn12 = 0.0;
        var_qbody_bt_p_sus_dn17 = 0.0;

        var_qbody_bt_p_sud = 0.0;
        var_qbody_bt_p_sud_dn0 = 0.0;
        var_qbody_bt_p_sud_dn2 = 0.0;
        var_qbody_bt_p_sud_dn6 = 0.0;
        var_qbody_bt_p_sud_dn7 = 0.0;
        var_qbody_bt_p_sud_dn10 = 0.0;
        var_qbody_bt_p_sud_dn11 = 0.0;
        var_qbody_bt_p_sud_dn12 = 0.0;
        var_qbody_bt_p_sud_dn17 = 0.0;

        var_qbody_bt_p_iud = 0.0;
        var_qbody_bt_p_iud_dn0 = 0.0;
        var_qbody_bt_p_iud_dn2 = 0.0;
        var_qbody_bt_p_iud_dn6 = 0.0;
        var_qbody_bt_p_iud_dn7 = 0.0;
        var_qbody_bt_p_iud_dn10 = 0.0;
        var_qbody_bt_p_iud_dn11 = 0.0;
        var_qbody_bt_p_iud_dn12 = 0.0;
        var_qbody_bt_p_iud_dn17 = 0.0;

        var_qbody_bt_p_ius = 0.0;
        var_qbody_bt_p_ius_dn0 = 0.0;
        var_qbody_bt_p_ius_dn2 = 0.0;
        var_qbody_bt_p_ius_dn6 = 0.0;
        var_qbody_bt_p_ius_dn7 = 0.0;
        var_qbody_bt_p_ius_dn10 = 0.0;
        var_qbody_bt_p_ius_dn11 = 0.0;
        var_qbody_bt_p_ius_dn12 = 0.0;
        var_qbody_bt_p_ius_dn17 = 0.0;

        var_qbody_bt_n_sus = 0.0;
        var_qbody_bt_n_sus_dn0 = 0.0;
        var_qbody_bt_n_sus_dn2 = 0.0;
        var_qbody_bt_n_sus_dn6 = 0.0;
        var_qbody_bt_n_sus_dn7 = 0.0;
        var_qbody_bt_n_sus_dn10 = 0.0;
        var_qbody_bt_n_sus_dn11 = 0.0;
        var_qbody_bt_n_sus_dn12 = 0.0;
        var_qbody_bt_n_sus_dn17 = 0.0;

        var_qbody_bt_n_sud = 0.0;
        var_qbody_bt_n_sud_dn0 = 0.0;
        var_qbody_bt_n_sud_dn2 = 0.0;
        var_qbody_bt_n_sud_dn6 = 0.0;
        var_qbody_bt_n_sud_dn7 = 0.0;
        var_qbody_bt_n_sud_dn10 = 0.0;
        var_qbody_bt_n_sud_dn11 = 0.0;
        var_qbody_bt_n_sud_dn12 = 0.0;
        var_qbody_bt_n_sud_dn17 = 0.0;

        var_qbody_bt_n_iud = 0.0;
        var_qbody_bt_n_iud_dn0 = 0.0;
        var_qbody_bt_n_iud_dn2 = 0.0;
        var_qbody_bt_n_iud_dn6 = 0.0;
        var_qbody_bt_n_iud_dn7 = 0.0;
        var_qbody_bt_n_iud_dn10 = 0.0;
        var_qbody_bt_n_iud_dn11 = 0.0;
        var_qbody_bt_n_iud_dn12 = 0.0;
        var_qbody_bt_n_iud_dn17 = 0.0;

        var_qbody_bt_n_ius = 0.0;
        var_qbody_bt_n_ius_dn0 = 0.0;
        var_qbody_bt_n_ius_dn2 = 0.0;
        var_qbody_bt_n_ius_dn6 = 0.0;
        var_qbody_bt_n_ius_dn7 = 0.0;
        var_qbody_bt_n_ius_dn10 = 0.0;
        var_qbody_bt_n_ius_dn11 = 0.0;
        var_qbody_bt_n_ius_dn12 = 0.0;
        var_qbody_bt_n_ius_dn17 = 0.0;

        var_uc_areabt = 0.0;

        var_uc_vfbbt = 0.0;

        var_q_bt_ge = 0.0;
        var_q_bt_ge_dn0 = 0.0;
        var_q_bt_ge_dn2 = 0.0;
        var_q_bt_ge_dn6 = 0.0;
        var_q_bt_ge_dn7 = 0.0;
        var_q_bt_ge_dn10 = 0.0;
        var_q_bt_ge_dn11 = 0.0;
        var_q_bt_ge_dn12 = 0.0;
        var_q_bt_ge_dn17 = 0.0;

        var_q_bt_se = 0.0;
        var_q_bt_se_dn0 = 0.0;
        var_q_bt_se_dn2 = 0.0;
        var_q_bt_se_dn6 = 0.0;
        var_q_bt_se_dn7 = 0.0;
        var_q_bt_se_dn10 = 0.0;
        var_q_bt_se_dn11 = 0.0;
        var_q_bt_se_dn12 = 0.0;
        var_q_bt_se_dn17 = 0.0;

        var_tau = 0.0;
        var_tau_dn0 = 0.0;
        var_tau_dn2 = 0.0;
        var_tau_dn6 = 0.0;
        var_tau_dn7 = 0.0;
        var_tau_dn10 = 0.0;
        var_tau_dn11 = 0.0;
        var_tau_dn12 = 0.0;
        var_tau_dn17 = 0.0;

        var_taub = 0.0;
        var_taub_dn0 = 0.0;
        var_taub_dn2 = 0.0;
        var_taub_dn6 = 0.0;
        var_taub_dn7 = 0.0;
        var_taub_dn10 = 0.0;
        var_taub_dn11 = 0.0;
        var_taub_dn12 = 0.0;
        var_taub_dn17 = 0.0;

        *var_alpha_slot = var_alpha;
        *var_alpha_dn0_slot = var_alpha_dn0;
        *var_alpha_dn10_slot = var_alpha_dn10;
        *var_alpha_dn11_slot = var_alpha_dn11;
        *var_alpha_dn12_slot = var_alpha_dn12;
        *var_alpha_dn17_slot = var_alpha_dn17;
        *var_alpha_dn2_slot = var_alpha_dn2;
        *var_alpha_dn6_slot = var_alpha_dn6;
        *var_alpha_dn7_slot = var_alpha_dn7;
        *var_betawl_slot = var_betawl;
        *var_betawl_dn0_slot = var_betawl_dn0;
        *var_betawl_dn10_slot = var_betawl_dn10;
        *var_betawl_dn11_slot = var_betawl_dn11;
        *var_betawl_dn12_slot = var_betawl_dn12;
        *var_betawl_dn17_slot = var_betawl_dn17;
        *var_betawl_dn2_slot = var_betawl_dn2;
        *var_betawl_dn6_slot = var_betawl_dn6;
        *var_betawl_dn7_slot = var_betawl_dn7;
        *var_end_of_part_1_slot = var_end_of_part_1;
        *var_ey_slot = var_ey;
        *var_ey_dn0_slot = var_ey_dn0;
        *var_ey_dn10_slot = var_ey_dn10;
        *var_ey_dn11_slot = var_ey_dn11;
        *var_ey_dn12_slot = var_ey_dn12;
        *var_ey_dn17_slot = var_ey_dn17;
        *var_ey_dn2_slot = var_ey_dn2;
        *var_ey_dn6_slot = var_ey_dn6;
        *var_ey_dn7_slot = var_ey_dn7;
        *var_fb_slot = var_fb;
        *var_fb_dn0_slot = var_fb_dn0;
        *var_fb_dn10_slot = var_fb_dn10;
        *var_fb_dn11_slot = var_fb_dn11;
        *var_fb_dn12_slot = var_fb_dn12;
        *var_fb_dn17_slot = var_fb_dn17;
        *var_fb_dn2_slot = var_fb_dn2;
        *var_fb_dn6_slot = var_fb_dn6;
        *var_fb_dn7_slot = var_fb_dn7;
        *var_flg_ign_slot = var_flg_ign;
        *var_flg_noqi_slot = var_flg_noqi;
        *var_flg_zone_slot = var_flg_zone;
        *var_gds0_ign_slot = var_gds0_ign;
        *var_gds0_ign_dn0_slot = var_gds0_ign_dn0;
        *var_gds0_ign_dn10_slot = var_gds0_ign_dn10;
        *var_gds0_ign_dn11_slot = var_gds0_ign_dn11;
        *var_gds0_ign_dn12_slot = var_gds0_ign_dn12;
        *var_gds0_ign_dn17_slot = var_gds0_ign_dn17;
        *var_gds0_ign_dn2_slot = var_gds0_ign_dn2;
        *var_gds0_ign_dn6_slot = var_gds0_ign_dn6;
        *var_gds0_ign_dn7_slot = var_gds0_ign_dn7;
        *var_ibd_slot = var_ibd;
        *var_ibd_dn0_slot = var_ibd_dn0;
        *var_ibd_dn10_slot = var_ibd_dn10;
        *var_ibd_dn11_slot = var_ibd_dn11;
        *var_ibd_dn12_slot = var_ibd_dn12;
        *var_ibd_dn17_slot = var_ibd_dn17;
        *var_ibd_dn2_slot = var_ibd_dn2;
        *var_ibd_dn6_slot = var_ibd_dn6;
        *var_ibd_dn7_slot = var_ibd_dn7;
        *var_ibs_slot = var_ibs;
        *var_ibs_dn0_slot = var_ibs_dn0;
        *var_ibs_dn10_slot = var_ibs_dn10;
        *var_ibs_dn11_slot = var_ibs_dn11;
        *var_ibs_dn12_slot = var_ibs_dn12;
        *var_ibs_dn17_slot = var_ibs_dn17;
        *var_ibs_dn2_slot = var_ibs_dn2;
        *var_ibs_dn6_slot = var_ibs_dn6;
        *var_ibs_dn7_slot = var_ibs_dn7;
        *var_idd_slot = var_idd;
        *var_idd_dn0_slot = var_idd_dn0;
        *var_idd_dn10_slot = var_idd_dn10;
        *var_idd_dn11_slot = var_idd_dn11;
        *var_idd_dn12_slot = var_idd_dn12;
        *var_idd_dn17_slot = var_idd_dn17;
        *var_idd_dn2_slot = var_idd_dn2;
        *var_idd_dn6_slot = var_idd_dn6;
        *var_idd_dn7_slot = var_idd_dn7;
        *var_ids_slot = var_ids;
        *var_ids_dn0_slot = var_ids_dn0;
        *var_ids_dn10_slot = var_ids_dn10;
        *var_ids_dn11_slot = var_ids_dn11;
        *var_ids_dn12_slot = var_ids_dn12;
        *var_ids_dn17_slot = var_ids_dn17;
        *var_ids_dn2_slot = var_ids_dn2;
        *var_ids_dn6_slot = var_ids_dn6;
        *var_ids_dn7_slot = var_ids_dn7;
        *var_idsibpc_slot = var_idsibpc;
        *var_idsibpc_dn0_slot = var_idsibpc_dn0;
        *var_idsibpc_dn10_slot = var_idsibpc_dn10;
        *var_idsibpc_dn11_slot = var_idsibpc_dn11;
        *var_idsibpc_dn12_slot = var_idsibpc_dn12;
        *var_idsibpc_dn17_slot = var_idsibpc_dn17;
        *var_idsibpc_dn2_slot = var_idsibpc_dn2;
        *var_idsibpc_dn6_slot = var_idsibpc_dn6;
        *var_idsibpc_dn7_slot = var_idsibpc_dn7;
        *var_isub_slot = var_isub;
        *var_isub_dn0_slot = var_isub_dn0;
        *var_isub_dn10_slot = var_isub_dn10;
        *var_isub_dn11_slot = var_isub_dn11;
        *var_isub_dn12_slot = var_isub_dn12;
        *var_isub_dn17_slot = var_isub_dn17;
        *var_isub_dn2_slot = var_isub_dn2;
        *var_isub_dn6_slot = var_isub_dn6;
        *var_isub_dn7_slot = var_isub_dn7;
        *var_mu_slot = var_mu;
        *var_mu_dn0_slot = var_mu_dn0;
        *var_mu_dn10_slot = var_mu_dn10;
        *var_mu_dn11_slot = var_mu_dn11;
        *var_mu_dn12_slot = var_mu_dn12;
        *var_mu_dn17_slot = var_mu_dn17;
        *var_mu_dn2_slot = var_mu_dn2;
        *var_mu_dn6_slot = var_mu_dn6;
        *var_mu_dn7_slot = var_mu_dn7;
        *var_muun_slot = var_muun;
        *var_muun_dn0_slot = var_muun_dn0;
        *var_muun_dn10_slot = var_muun_dn10;
        *var_muun_dn11_slot = var_muun_dn11;
        *var_muun_dn12_slot = var_muun_dn12;
        *var_muun_dn17_slot = var_muun_dn17;
        *var_muun_dn2_slot = var_muun_dn2;
        *var_muun_dn6_slot = var_muun_dn6;
        *var_muun_dn7_slot = var_muun_dn7;
        *var_pds_slot = var_pds;
        *var_pds_dn0_slot = var_pds_dn0;
        *var_pds_dn10_slot = var_pds_dn10;
        *var_pds_dn11_slot = var_pds_dn11;
        *var_pds_dn12_slot = var_pds_dn12;
        *var_pds_dn17_slot = var_pds_dn17;
        *var_pds_dn2_slot = var_pds_dn2;
        *var_pds_dn6_slot = var_pds_dn6;
        *var_pds_dn7_slot = var_pds_dn7;
        *var_pds_ini_slot = var_pds_ini;
        *var_pds_ini_dn0_slot = var_pds_ini_dn0;
        *var_pds_ini_dn10_slot = var_pds_ini_dn10;
        *var_pds_ini_dn11_slot = var_pds_ini_dn11;
        *var_pds_ini_dn12_slot = var_pds_ini_dn12;
        *var_pds_ini_dn17_slot = var_pds_ini_dn17;
        *var_pds_ini_dn2_slot = var_pds_ini_dn2;
        *var_pds_ini_dn6_slot = var_pds_ini_dn6;
        *var_pds_ini_dn7_slot = var_pds_ini_dn7;
        *var_ps0z_slot = var_ps0z;
        *var_ps0z_dn0_slot = var_ps0z_dn0;
        *var_ps0z_dn10_slot = var_ps0z_dn10;
        *var_ps0z_dn11_slot = var_ps0z_dn11;
        *var_ps0z_dn12_slot = var_ps0z_dn12;
        *var_ps0z_dn17_slot = var_ps0z_dn17;
        *var_ps0z_dn2_slot = var_ps0z_dn2;
        *var_ps0z_dn6_slot = var_ps0z_dn6;
        *var_ps0z_dn7_slot = var_ps0z_dn7;
        *var_psl_slot = var_psl;
        *var_psl_dn0_slot = var_psl_dn0;
        *var_psl_dn10_slot = var_psl_dn10;
        *var_psl_dn11_slot = var_psl_dn11;
        *var_psl_dn12_slot = var_psl_dn12;
        *var_psl_dn17_slot = var_psl_dn17;
        *var_psl_dn2_slot = var_psl_dn2;
        *var_psl_dn6_slot = var_psl_dn6;
        *var_psl_dn7_slot = var_psl_dn7;
        *var_psl_lim_slot = var_psl_lim;
        *var_psl_lim_dn0_slot = var_psl_lim_dn0;
        *var_psl_lim_dn10_slot = var_psl_lim_dn10;
        *var_psl_lim_dn11_slot = var_psl_lim_dn11;
        *var_psl_lim_dn12_slot = var_psl_lim_dn12;
        *var_psl_lim_dn17_slot = var_psl_lim_dn17;
        *var_psl_lim_dn2_slot = var_psl_lim_dn2;
        *var_psl_lim_dn6_slot = var_psl_lim_dn6;
        *var_psl_lim_dn7_slot = var_psl_lim_dn7;
        *var_q_bt_ge_slot = var_q_bt_ge;
        *var_q_bt_ge_dn0_slot = var_q_bt_ge_dn0;
        *var_q_bt_ge_dn10_slot = var_q_bt_ge_dn10;
        *var_q_bt_ge_dn11_slot = var_q_bt_ge_dn11;
        *var_q_bt_ge_dn12_slot = var_q_bt_ge_dn12;
        *var_q_bt_ge_dn17_slot = var_q_bt_ge_dn17;
        *var_q_bt_ge_dn2_slot = var_q_bt_ge_dn2;
        *var_q_bt_ge_dn6_slot = var_q_bt_ge_dn6;
        *var_q_bt_ge_dn7_slot = var_q_bt_ge_dn7;
        *var_q_bt_se_slot = var_q_bt_se;
        *var_q_bt_se_dn0_slot = var_q_bt_se_dn0;
        *var_q_bt_se_dn10_slot = var_q_bt_se_dn10;
        *var_q_bt_se_dn11_slot = var_q_bt_se_dn11;
        *var_q_bt_se_dn12_slot = var_q_bt_se_dn12;
        *var_q_bt_se_dn17_slot = var_q_bt_se_dn17;
        *var_q_bt_se_dn2_slot = var_q_bt_se_dn2;
        *var_q_bt_se_dn6_slot = var_q_bt_se_dn6;
        *var_q_bt_se_dn7_slot = var_q_bt_se_dn7;
        *var_qb_slot = var_qb;
        *var_qb_dn0_slot = var_qb_dn0;
        *var_qb_dn10_slot = var_qb_dn10;
        *var_qb_dn11_slot = var_qb_dn11;
        *var_qb_dn12_slot = var_qb_dn12;
        *var_qb_dn13_slot = var_qb_dn13;
        *var_qb_dn15_slot = var_qb_dn15;
        *var_qb_dn16_slot = var_qb_dn16;
        *var_qb_dn17_slot = var_qb_dn17;
        *var_qb_dn18_slot = var_qb_dn18;
        *var_qb_dn2_slot = var_qb_dn2;
        *var_qb_dn6_slot = var_qb_dn6;
        *var_qb_dn7_slot = var_qb_dn7;
        *var_qbd_slot = var_qbd;
        *var_qbd_dn0_slot = var_qbd_dn0;
        *var_qbd_dn10_slot = var_qbd_dn10;
        *var_qbd_dn11_slot = var_qbd_dn11;
        *var_qbd_dn12_slot = var_qbd_dn12;
        *var_qbd_dn17_slot = var_qbd_dn17;
        *var_qbd_dn2_slot = var_qbd_dn2;
        *var_qbd_dn6_slot = var_qbd_dn6;
        *var_qbd_dn7_slot = var_qbd_dn7;
        *var_qbdld_slot = var_qbdld;
        *var_qbdld_dn0_slot = var_qbdld_dn0;
        *var_qbdld_dn10_slot = var_qbdld_dn10;
        *var_qbdld_dn11_slot = var_qbdld_dn11;
        *var_qbdld_dn12_slot = var_qbdld_dn12;
        *var_qbdld_dn17_slot = var_qbdld_dn17;
        *var_qbdld_dn2_slot = var_qbdld_dn2;
        *var_qbdld_dn6_slot = var_qbdld_dn6;
        *var_qbdld_dn7_slot = var_qbdld_dn7;
        *var_qbody_bt_n_iud_slot = var_qbody_bt_n_iud;
        *var_qbody_bt_n_iud_dn0_slot = var_qbody_bt_n_iud_dn0;
        *var_qbody_bt_n_iud_dn10_slot = var_qbody_bt_n_iud_dn10;
        *var_qbody_bt_n_iud_dn11_slot = var_qbody_bt_n_iud_dn11;
        *var_qbody_bt_n_iud_dn12_slot = var_qbody_bt_n_iud_dn12;
        *var_qbody_bt_n_iud_dn17_slot = var_qbody_bt_n_iud_dn17;
        *var_qbody_bt_n_iud_dn2_slot = var_qbody_bt_n_iud_dn2;
        *var_qbody_bt_n_iud_dn6_slot = var_qbody_bt_n_iud_dn6;
        *var_qbody_bt_n_iud_dn7_slot = var_qbody_bt_n_iud_dn7;
        *var_qbody_bt_n_ius_slot = var_qbody_bt_n_ius;
        *var_qbody_bt_n_ius_dn0_slot = var_qbody_bt_n_ius_dn0;
        *var_qbody_bt_n_ius_dn10_slot = var_qbody_bt_n_ius_dn10;
        *var_qbody_bt_n_ius_dn11_slot = var_qbody_bt_n_ius_dn11;
        *var_qbody_bt_n_ius_dn12_slot = var_qbody_bt_n_ius_dn12;
        *var_qbody_bt_n_ius_dn17_slot = var_qbody_bt_n_ius_dn17;
        *var_qbody_bt_n_ius_dn2_slot = var_qbody_bt_n_ius_dn2;
        *var_qbody_bt_n_ius_dn6_slot = var_qbody_bt_n_ius_dn6;
        *var_qbody_bt_n_ius_dn7_slot = var_qbody_bt_n_ius_dn7;
        *var_qbody_bt_n_sud_slot = var_qbody_bt_n_sud;
        *var_qbody_bt_n_sud_dn0_slot = var_qbody_bt_n_sud_dn0;
        *var_qbody_bt_n_sud_dn10_slot = var_qbody_bt_n_sud_dn10;
        *var_qbody_bt_n_sud_dn11_slot = var_qbody_bt_n_sud_dn11;
        *var_qbody_bt_n_sud_dn12_slot = var_qbody_bt_n_sud_dn12;
        *var_qbody_bt_n_sud_dn17_slot = var_qbody_bt_n_sud_dn17;
        *var_qbody_bt_n_sud_dn2_slot = var_qbody_bt_n_sud_dn2;
        *var_qbody_bt_n_sud_dn6_slot = var_qbody_bt_n_sud_dn6;
        *var_qbody_bt_n_sud_dn7_slot = var_qbody_bt_n_sud_dn7;
        *var_qbody_bt_n_sus_slot = var_qbody_bt_n_sus;
        *var_qbody_bt_n_sus_dn0_slot = var_qbody_bt_n_sus_dn0;
        *var_qbody_bt_n_sus_dn10_slot = var_qbody_bt_n_sus_dn10;
        *var_qbody_bt_n_sus_dn11_slot = var_qbody_bt_n_sus_dn11;
        *var_qbody_bt_n_sus_dn12_slot = var_qbody_bt_n_sus_dn12;
        *var_qbody_bt_n_sus_dn17_slot = var_qbody_bt_n_sus_dn17;
        *var_qbody_bt_n_sus_dn2_slot = var_qbody_bt_n_sus_dn2;
        *var_qbody_bt_n_sus_dn6_slot = var_qbody_bt_n_sus_dn6;
        *var_qbody_bt_n_sus_dn7_slot = var_qbody_bt_n_sus_dn7;
        *var_qbody_bt_p_iud_slot = var_qbody_bt_p_iud;
        *var_qbody_bt_p_iud_dn0_slot = var_qbody_bt_p_iud_dn0;
        *var_qbody_bt_p_iud_dn10_slot = var_qbody_bt_p_iud_dn10;
        *var_qbody_bt_p_iud_dn11_slot = var_qbody_bt_p_iud_dn11;
        *var_qbody_bt_p_iud_dn12_slot = var_qbody_bt_p_iud_dn12;
        *var_qbody_bt_p_iud_dn17_slot = var_qbody_bt_p_iud_dn17;
        *var_qbody_bt_p_iud_dn2_slot = var_qbody_bt_p_iud_dn2;
        *var_qbody_bt_p_iud_dn6_slot = var_qbody_bt_p_iud_dn6;
        *var_qbody_bt_p_iud_dn7_slot = var_qbody_bt_p_iud_dn7;
        *var_qbody_bt_p_ius_slot = var_qbody_bt_p_ius;
        *var_qbody_bt_p_ius_dn0_slot = var_qbody_bt_p_ius_dn0;
        *var_qbody_bt_p_ius_dn10_slot = var_qbody_bt_p_ius_dn10;
        *var_qbody_bt_p_ius_dn11_slot = var_qbody_bt_p_ius_dn11;
        *var_qbody_bt_p_ius_dn12_slot = var_qbody_bt_p_ius_dn12;
        *var_qbody_bt_p_ius_dn17_slot = var_qbody_bt_p_ius_dn17;
        *var_qbody_bt_p_ius_dn2_slot = var_qbody_bt_p_ius_dn2;
        *var_qbody_bt_p_ius_dn6_slot = var_qbody_bt_p_ius_dn6;
        *var_qbody_bt_p_ius_dn7_slot = var_qbody_bt_p_ius_dn7;
        *var_qbody_bt_p_sud_slot = var_qbody_bt_p_sud;
        *var_qbody_bt_p_sud_dn0_slot = var_qbody_bt_p_sud_dn0;
        *var_qbody_bt_p_sud_dn10_slot = var_qbody_bt_p_sud_dn10;
        *var_qbody_bt_p_sud_dn11_slot = var_qbody_bt_p_sud_dn11;
        *var_qbody_bt_p_sud_dn12_slot = var_qbody_bt_p_sud_dn12;
        *var_qbody_bt_p_sud_dn17_slot = var_qbody_bt_p_sud_dn17;
        *var_qbody_bt_p_sud_dn2_slot = var_qbody_bt_p_sud_dn2;
        *var_qbody_bt_p_sud_dn6_slot = var_qbody_bt_p_sud_dn6;
        *var_qbody_bt_p_sud_dn7_slot = var_qbody_bt_p_sud_dn7;
        *var_qbody_bt_p_sus_slot = var_qbody_bt_p_sus;
        *var_qbody_bt_p_sus_dn0_slot = var_qbody_bt_p_sus_dn0;
        *var_qbody_bt_p_sus_dn10_slot = var_qbody_bt_p_sus_dn10;
        *var_qbody_bt_p_sus_dn11_slot = var_qbody_bt_p_sus_dn11;
        *var_qbody_bt_p_sus_dn12_slot = var_qbody_bt_p_sus_dn12;
        *var_qbody_bt_p_sus_dn17_slot = var_qbody_bt_p_sus_dn17;
        *var_qbody_bt_p_sus_dn2_slot = var_qbody_bt_p_sus_dn2;
        *var_qbody_bt_p_sus_dn6_slot = var_qbody_bt_p_sus_dn6;
        *var_qbody_bt_p_sus_dn7_slot = var_qbody_bt_p_sus_dn7;
        *var_qbs_slot = var_qbs;
        *var_qbs_dn0_slot = var_qbs_dn0;
        *var_qbs_dn10_slot = var_qbs_dn10;
        *var_qbs_dn11_slot = var_qbs_dn11;
        *var_qbs_dn12_slot = var_qbs_dn12;
        *var_qbs_dn17_slot = var_qbs_dn17;
        *var_qbs_dn2_slot = var_qbs_dn2;
        *var_qbs_dn6_slot = var_qbs_dn6;
        *var_qbs_dn7_slot = var_qbs_dn7;
        *var_qbsld_slot = var_qbsld;
        *var_qbsld_dn0_slot = var_qbsld_dn0;
        *var_qbsld_dn10_slot = var_qbsld_dn10;
        *var_qbsld_dn11_slot = var_qbsld_dn11;
        *var_qbsld_dn12_slot = var_qbsld_dn12;
        *var_qbsld_dn17_slot = var_qbsld_dn17;
        *var_qbsld_dn2_slot = var_qbsld_dn2;
        *var_qbsld_dn6_slot = var_qbsld_dn6;
        *var_qbsld_dn7_slot = var_qbsld_dn7;
        *var_qd_slot = var_qd;
        *var_qd_dn0_slot = var_qd_dn0;
        *var_qd_dn10_slot = var_qd_dn10;
        *var_qd_dn11_slot = var_qd_dn11;
        *var_qd_dn12_slot = var_qd_dn12;
        *var_qd_dn13_slot = var_qd_dn13;
        *var_qd_dn15_slot = var_qd_dn15;
        *var_qd_dn16_slot = var_qd_dn16;
        *var_qd_dn17_slot = var_qd_dn17;
        *var_qd_dn18_slot = var_qd_dn18;
        *var_qd_dn2_slot = var_qd_dn2;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qgob_slot = var_qgob;
        *var_qgob_dn0_slot = var_qgob_dn0;
        *var_qgob_dn10_slot = var_qgob_dn10;
        *var_qgob_dn11_slot = var_qgob_dn11;
        *var_qgob_dn12_slot = var_qgob_dn12;
        *var_qgob_dn17_slot = var_qgob_dn17;
        *var_qgob_dn2_slot = var_qgob_dn2;
        *var_qgob_dn6_slot = var_qgob_dn6;
        *var_qgob_dn7_slot = var_qgob_dn7;
        *var_qgod_slot = var_qgod;
        *var_qgod_dn0_slot = var_qgod_dn0;
        *var_qgod_dn10_slot = var_qgod_dn10;
        *var_qgod_dn11_slot = var_qgod_dn11;
        *var_qgod_dn12_slot = var_qgod_dn12;
        *var_qgod_dn17_slot = var_qgod_dn17;
        *var_qgod_dn2_slot = var_qgod_dn2;
        *var_qgod_dn6_slot = var_qgod_dn6;
        *var_qgod_dn7_slot = var_qgod_dn7;
        *var_qgos_slot = var_qgos;
        *var_qgos_dn0_slot = var_qgos_dn0;
        *var_qgos_dn10_slot = var_qgos_dn10;
        *var_qgos_dn11_slot = var_qgos_dn11;
        *var_qgos_dn12_slot = var_qgos_dn12;
        *var_qgos_dn17_slot = var_qgos_dn17;
        *var_qgos_dn2_slot = var_qgos_dn2;
        *var_qgos_dn6_slot = var_qgos_dn6;
        *var_qgos_dn7_slot = var_qgos_dn7;
        *var_qi_slot = var_qi;
        *var_qi_dn0_slot = var_qi_dn0;
        *var_qi_dn10_slot = var_qi_dn10;
        *var_qi_dn11_slot = var_qi_dn11;
        *var_qi_dn12_slot = var_qi_dn12;
        *var_qi_dn17_slot = var_qi_dn17;
        *var_qi_dn2_slot = var_qi_dn2;
        *var_qi_dn6_slot = var_qi_dn6;
        *var_qi_dn7_slot = var_qi_dn7;
        *var_qidn_slot = var_qidn;
        *var_qidn_dn0_slot = var_qidn_dn0;
        *var_qidn_dn10_slot = var_qidn_dn10;
        *var_qidn_dn11_slot = var_qidn_dn11;
        *var_qidn_dn12_slot = var_qidn_dn12;
        *var_qidn_dn17_slot = var_qidn_dn17;
        *var_qidn_dn2_slot = var_qidn_dn2;
        *var_qidn_dn6_slot = var_qidn_dn6;
        *var_qidn_dn7_slot = var_qidn_dn7;
        *var_qinm_slot = var_qinm;
        *var_qinm_dn0_slot = var_qinm_dn0;
        *var_qinm_dn10_slot = var_qinm_dn10;
        *var_qinm_dn11_slot = var_qinm_dn11;
        *var_qinm_dn12_slot = var_qinm_dn12;
        *var_qinm_dn17_slot = var_qinm_dn17;
        *var_qinm_dn2_slot = var_qinm_dn2;
        *var_qinm_dn6_slot = var_qinm_dn6;
        *var_qinm_dn7_slot = var_qinm_dn7;
        *var_qn0_slot = var_qn0;
        *var_qn0_dn0_slot = var_qn0_dn0;
        *var_qn0_dn10_slot = var_qn0_dn10;
        *var_qn0_dn11_slot = var_qn0_dn11;
        *var_qn0_dn12_slot = var_qn0_dn12;
        *var_qn0_dn17_slot = var_qn0_dn17;
        *var_qn0_dn2_slot = var_qn0_dn2;
        *var_qn0_dn6_slot = var_qn0_dn6;
        *var_qn0_dn7_slot = var_qn0_dn7;
        *var_qovd_slot = var_qovd;
        *var_qovd_dn0_slot = var_qovd_dn0;
        *var_qovd_dn10_slot = var_qovd_dn10;
        *var_qovd_dn11_slot = var_qovd_dn11;
        *var_qovd_dn12_slot = var_qovd_dn12;
        *var_qovd_dn17_slot = var_qovd_dn17;
        *var_qovd_dn2_slot = var_qovd_dn2;
        *var_qovd_dn6_slot = var_qovd_dn6;
        *var_qovd_dn7_slot = var_qovd_dn7;
        *var_qovs_slot = var_qovs;
        *var_qovs_dn0_slot = var_qovs_dn0;
        *var_qovs_dn10_slot = var_qovs_dn10;
        *var_qovs_dn11_slot = var_qovs_dn11;
        *var_qovs_dn12_slot = var_qovs_dn12;
        *var_qovs_dn17_slot = var_qovs_dn17;
        *var_qovs_dn2_slot = var_qovs_dn2;
        *var_qovs_dn6_slot = var_qovs_dn6;
        *var_qovs_dn7_slot = var_qovs_dn7;
        *var_qse_slot = var_qse;
        *var_qse_dn0_slot = var_qse_dn0;
        *var_qse_dn10_slot = var_qse_dn10;
        *var_qse_dn11_slot = var_qse_dn11;
        *var_qse_dn12_slot = var_qse_dn12;
        *var_qse_dn13_slot = var_qse_dn13;
        *var_qse_dn15_slot = var_qse_dn15;
        *var_qse_dn16_slot = var_qse_dn16;
        *var_qse_dn17_slot = var_qse_dn17;
        *var_qse_dn18_slot = var_qse_dn18;
        *var_qse_dn2_slot = var_qse_dn2;
        *var_qse_dn6_slot = var_qse_dn6;
        *var_qse_dn7_slot = var_qse_dn7;
        *var_tau_slot = var_tau;
        *var_tau_dn0_slot = var_tau_dn0;
        *var_tau_dn10_slot = var_tau_dn10;
        *var_tau_dn11_slot = var_tau_dn11;
        *var_tau_dn12_slot = var_tau_dn12;
        *var_tau_dn17_slot = var_tau_dn17;
        *var_tau_dn2_slot = var_tau_dn2;
        *var_tau_dn6_slot = var_tau_dn6;
        *var_tau_dn7_slot = var_tau_dn7;
        *var_taub_slot = var_taub;
        *var_taub_dn0_slot = var_taub_dn0;
        *var_taub_dn10_slot = var_taub_dn10;
        *var_taub_dn11_slot = var_taub_dn11;
        *var_taub_dn12_slot = var_taub_dn12;
        *var_taub_dn17_slot = var_taub_dn17;
        *var_taub_dn2_slot = var_taub_dn2;
        *var_taub_dn6_slot = var_taub_dn6;
        *var_taub_dn7_slot = var_taub_dn7;
        *var_uc_areabt_slot = var_uc_areabt;
        *var_uc_vfbbt_slot = var_uc_vfbbt;
        *var_vgvt_slot = var_vgvt;
        *var_vgvt_dn0_slot = var_vgvt_dn0;
        *var_vgvt_dn10_slot = var_vgvt_dn10;
        *var_vgvt_dn11_slot = var_vgvt_dn11;
        *var_vgvt_dn12_slot = var_vgvt_dn12;
        *var_vgvt_dn17_slot = var_vgvt_dn17;
        *var_vgvt_dn2_slot = var_vgvt_dn2;
        *var_vgvt_dn6_slot = var_vgvt_dn6;
        *var_vgvt_dn7_slot = var_vgvt_dn7;
        *var_wdsoi_0_slot = var_wdsoi_0;
        *var_xd_slot = var_xd;
        *var_xd_dn0_slot = var_xd_dn0;
        *var_xd_dn10_slot = var_xd_dn10;
        *var_xd_dn11_slot = var_xd_dn11;
        *var_xd_dn12_slot = var_xd_dn12;
        *var_xd_dn17_slot = var_xd_dn17;
        *var_xd_dn2_slot = var_xd_dn2;
        *var_xd_dn6_slot = var_xd_dn6;
        *var_xd_dn7_slot = var_xd_dn7;
    }

    pub(super) fn stamp_transient_block_1(
        var_crl_f_slot: &mut f64,
        var_crl_f_dn0_slot: &mut f64,
        var_crl_f_dn10_slot: &mut f64,
        var_crl_f_dn11_slot: &mut f64,
        var_crl_f_dn12_slot: &mut f64,
        var_crl_f_dn17_slot: &mut f64,
        var_crl_f_dn2_slot: &mut f64,
        var_crl_f_dn6_slot: &mut f64,
        var_crl_f_dn7_slot: &mut f64,
        var_ec_slot: &mut f64,
        var_ec_dn0_slot: &mut f64,
        var_ec_dn10_slot: &mut f64,
        var_ec_dn11_slot: &mut f64,
        var_ec_dn12_slot: &mut f64,
        var_ec_dn17_slot: &mut f64,
        var_ec_dn2_slot: &mut f64,
        var_ec_dn6_slot: &mut f64,
        var_ec_dn7_slot: &mut f64,
        var_flg_depmode_slot: &mut f64,
        var_iqd_nqs_slot: &mut f64,
        var_iqd_nqs_dn0_slot: &mut f64,
        var_iqd_nqs_dn10_slot: &mut f64,
        var_iqd_nqs_dn11_slot: &mut f64,
        var_iqd_nqs_dn12_slot: &mut f64,
        var_iqd_nqs_dn13_slot: &mut f64,
        var_iqd_nqs_dn15_slot: &mut f64,
        var_iqd_nqs_dn16_slot: &mut f64,
        var_iqd_nqs_dn17_slot: &mut f64,
        var_iqd_nqs_dn18_slot: &mut f64,
        var_iqd_nqs_dn2_slot: &mut f64,
        var_iqd_nqs_dn6_slot: &mut f64,
        var_iqd_nqs_dn7_slot: &mut f64,
        var_iqh_nqs_slot: &mut f64,
        var_iqh_nqs_dn0_slot: &mut f64,
        var_iqh_nqs_dn10_slot: &mut f64,
        var_iqh_nqs_dn11_slot: &mut f64,
        var_iqh_nqs_dn12_slot: &mut f64,
        var_iqh_nqs_dn17_slot: &mut f64,
        var_iqh_nqs_dn2_slot: &mut f64,
        var_iqh_nqs_dn6_slot: &mut f64,
        var_iqh_nqs_dn7_slot: &mut f64,
        var_iqi_nqs_slot: &mut f64,
        var_iqi_nqs_dn0_slot: &mut f64,
        var_iqi_nqs_dn10_slot: &mut f64,
        var_iqi_nqs_dn11_slot: &mut f64,
        var_iqi_nqs_dn12_slot: &mut f64,
        var_iqi_nqs_dn17_slot: &mut f64,
        var_iqi_nqs_dn18_slot: &mut f64,
        var_iqi_nqs_dn2_slot: &mut f64,
        var_iqi_nqs_dn6_slot: &mut f64,
        var_iqi_nqs_dn7_slot: &mut f64,
        var_iqs_nqs_slot: &mut f64,
        var_iqs_nqs_dn0_slot: &mut f64,
        var_iqs_nqs_dn10_slot: &mut f64,
        var_iqs_nqs_dn11_slot: &mut f64,
        var_iqs_nqs_dn12_slot: &mut f64,
        var_iqs_nqs_dn13_slot: &mut f64,
        var_iqs_nqs_dn15_slot: &mut f64,
        var_iqs_nqs_dn16_slot: &mut f64,
        var_iqs_nqs_dn17_slot: &mut f64,
        var_iqs_nqs_dn18_slot: &mut f64,
        var_iqs_nqs_dn2_slot: &mut f64,
        var_iqs_nqs_dn6_slot: &mut f64,
        var_iqs_nqs_dn7_slot: &mut f64,
        var_kusai00_slot: &mut f64,
        var_kusai00_dn0_slot: &mut f64,
        var_kusai00_dn10_slot: &mut f64,
        var_kusai00_dn11_slot: &mut f64,
        var_kusai00_dn12_slot: &mut f64,
        var_kusai00_dn17_slot: &mut f64,
        var_kusai00_dn2_slot: &mut f64,
        var_kusai00_dn6_slot: &mut f64,
        var_kusai00_dn7_slot: &mut f64,
        var_kusai00l_slot: &mut f64,
        var_kusai00l_dn0_slot: &mut f64,
        var_kusai00l_dn10_slot: &mut f64,
        var_kusai00l_dn11_slot: &mut f64,
        var_kusai00l_dn12_slot: &mut f64,
        var_kusai00l_dn17_slot: &mut f64,
        var_kusai00l_dn2_slot: &mut f64,
        var_kusai00l_dn6_slot: &mut f64,
        var_kusai00l_dn7_slot: &mut f64,
        var_kusai_ig_slot: &mut f64,
        var_kusai_ig_dn0_slot: &mut f64,
        var_kusai_ig_dn10_slot: &mut f64,
        var_kusai_ig_dn11_slot: &mut f64,
        var_kusai_ig_dn12_slot: &mut f64,
        var_kusai_ig_dn17_slot: &mut f64,
        var_kusai_ig_dn2_slot: &mut f64,
        var_kusai_ig_dn6_slot: &mut f64,
        var_kusai_ig_dn7_slot: &mut f64,
        var_kusail_slot: &mut f64,
        var_kusail_dn0_slot: &mut f64,
        var_kusail_dn10_slot: &mut f64,
        var_kusail_dn11_slot: &mut f64,
        var_kusail_dn12_slot: &mut f64,
        var_kusail_dn17_slot: &mut f64,
        var_kusail_dn2_slot: &mut f64,
        var_kusail_dn6_slot: &mut f64,
        var_kusail_dn7_slot: &mut f64,
        var_lred_slot: &mut f64,
        var_lred_dn0_slot: &mut f64,
        var_lred_dn10_slot: &mut f64,
        var_lred_dn11_slot: &mut f64,
        var_lred_dn12_slot: &mut f64,
        var_lred_dn17_slot: &mut f64,
        var_lred_dn2_slot: &mut f64,
        var_lred_dn6_slot: &mut f64,
        var_lred_dn7_slot: &mut f64,
        var_mud_hoso_slot: &mut f64,
        var_mud_hoso_dn0_slot: &mut f64,
        var_mud_hoso_dn10_slot: &mut f64,
        var_mud_hoso_dn11_slot: &mut f64,
        var_mud_hoso_dn12_slot: &mut f64,
        var_mud_hoso_dn17_slot: &mut f64,
        var_mud_hoso_dn2_slot: &mut f64,
        var_mud_hoso_dn6_slot: &mut f64,
        var_mud_hoso_dn7_slot: &mut f64,
        var_phi_b0_soi_slot: &mut f64,
        var_phi_b0_soi_dn0_slot: &mut f64,
        var_phi_b0_soi_dn10_slot: &mut f64,
        var_phi_b0_soi_dn11_slot: &mut f64,
        var_phi_b0_soi_dn12_slot: &mut f64,
        var_phi_b0_soi_dn17_slot: &mut f64,
        var_phi_b0_soi_dn2_slot: &mut f64,
        var_phi_b0_soi_dn6_slot: &mut f64,
        var_phi_b0_soi_dn7_slot: &mut f64,
        var_phi_b_dep0_slot: &mut f64,
        var_phi_b_dep0_dn0_slot: &mut f64,
        var_phi_b_dep0_dn10_slot: &mut f64,
        var_phi_b_dep0_dn11_slot: &mut f64,
        var_phi_b_dep0_dn12_slot: &mut f64,
        var_phi_b_dep0_dn17_slot: &mut f64,
        var_phi_b_dep0_dn2_slot: &mut f64,
        var_phi_b_dep0_dn6_slot: &mut f64,
        var_phi_b_dep0_dn7_slot: &mut f64,
        var_phi_bl_soi_slot: &mut f64,
        var_phi_bl_soi_dn0_slot: &mut f64,
        var_phi_bl_soi_dn10_slot: &mut f64,
        var_phi_bl_soi_dn11_slot: &mut f64,
        var_phi_bl_soi_dn12_slot: &mut f64,
        var_phi_bl_soi_dn17_slot: &mut f64,
        var_phi_bl_soi_dn2_slot: &mut f64,
        var_phi_bl_soi_dn6_slot: &mut f64,
        var_phi_bl_soi_dn7_slot: &mut f64,
        var_phi_bl_soi_ini_slot: &mut f64,
        var_phi_bl_soi_ini_dn0_slot: &mut f64,
        var_phi_bl_soi_ini_dn10_slot: &mut f64,
        var_phi_bl_soi_ini_dn11_slot: &mut f64,
        var_phi_bl_soi_ini_dn12_slot: &mut f64,
        var_phi_bl_soi_ini_dn17_slot: &mut f64,
        var_phi_bl_soi_ini_dn2_slot: &mut f64,
        var_phi_bl_soi_ini_dn6_slot: &mut f64,
        var_phi_bl_soi_ini_dn7_slot: &mut f64,
        var_phi_s0_bulk_slot: &mut f64,
        var_phi_s0_bulk_dn0_slot: &mut f64,
        var_phi_s0_bulk_dn10_slot: &mut f64,
        var_phi_s0_bulk_dn11_slot: &mut f64,
        var_phi_s0_bulk_dn12_slot: &mut f64,
        var_phi_s0_bulk_dn17_slot: &mut f64,
        var_phi_s0_bulk_dn2_slot: &mut f64,
        var_phi_s0_bulk_dn6_slot: &mut f64,
        var_phi_s0_bulk_dn7_slot: &mut f64,
        var_phi_s0_soi_slot: &mut f64,
        var_phi_s0_soi_dn0_slot: &mut f64,
        var_phi_s0_soi_dn10_slot: &mut f64,
        var_phi_s0_soi_dn11_slot: &mut f64,
        var_phi_s0_soi_dn12_slot: &mut f64,
        var_phi_s0_soi_dn17_slot: &mut f64,
        var_phi_s0_soi_dn2_slot: &mut f64,
        var_phi_s0_soi_dn6_slot: &mut f64,
        var_phi_s0_soi_dn7_slot: &mut f64,
        var_phi_sl_bulk_slot: &mut f64,
        var_phi_sl_bulk_dn0_slot: &mut f64,
        var_phi_sl_bulk_dn10_slot: &mut f64,
        var_phi_sl_bulk_dn11_slot: &mut f64,
        var_phi_sl_bulk_dn12_slot: &mut f64,
        var_phi_sl_bulk_dn17_slot: &mut f64,
        var_phi_sl_bulk_dn2_slot: &mut f64,
        var_phi_sl_bulk_dn6_slot: &mut f64,
        var_phi_sl_bulk_dn7_slot: &mut f64,
        var_phi_sl_bulk_ini_slot: &mut f64,
        var_phi_sl_bulk_ini_dn0_slot: &mut f64,
        var_phi_sl_bulk_ini_dn10_slot: &mut f64,
        var_phi_sl_bulk_ini_dn11_slot: &mut f64,
        var_phi_sl_bulk_ini_dn12_slot: &mut f64,
        var_phi_sl_bulk_ini_dn17_slot: &mut f64,
        var_phi_sl_bulk_ini_dn2_slot: &mut f64,
        var_phi_sl_bulk_ini_dn6_slot: &mut f64,
        var_phi_sl_bulk_ini_dn7_slot: &mut f64,
        var_phi_sl_soi_slot: &mut f64,
        var_phi_sl_soi_dn0_slot: &mut f64,
        var_phi_sl_soi_dn10_slot: &mut f64,
        var_phi_sl_soi_dn11_slot: &mut f64,
        var_phi_sl_soi_dn12_slot: &mut f64,
        var_phi_sl_soi_dn17_slot: &mut f64,
        var_phi_sl_soi_dn2_slot: &mut f64,
        var_phi_sl_soi_dn6_slot: &mut f64,
        var_phi_sl_soi_dn7_slot: &mut f64,
        var_phi_sl_soi_ini_slot: &mut f64,
        var_phi_sl_soi_ini_dn0_slot: &mut f64,
        var_phi_sl_soi_ini_dn10_slot: &mut f64,
        var_phi_sl_soi_ini_dn11_slot: &mut f64,
        var_phi_sl_soi_ini_dn12_slot: &mut f64,
        var_phi_sl_soi_ini_dn17_slot: &mut f64,
        var_phi_sl_soi_ini_dn2_slot: &mut f64,
        var_phi_sl_soi_ini_dn6_slot: &mut f64,
        var_phi_sl_soi_ini_dn7_slot: &mut f64,
        var_ps0_inia_slot: &mut f64,
        var_ps0_inia_dn0_slot: &mut f64,
        var_ps0_inia_dn10_slot: &mut f64,
        var_ps0_inia_dn11_slot: &mut f64,
        var_ps0_inia_dn12_slot: &mut f64,
        var_ps0_inia_dn17_slot: &mut f64,
        var_ps0_inia_dn2_slot: &mut f64,
        var_ps0_inia_dn6_slot: &mut f64,
        var_ps0_inia_dn7_slot: &mut f64,
        var_psdl_slot: &mut f64,
        var_psdl_dn0_slot: &mut f64,
        var_psdl_dn10_slot: &mut f64,
        var_psdl_dn11_slot: &mut f64,
        var_psdl_dn12_slot: &mut f64,
        var_psdl_dn17_slot: &mut f64,
        var_psdl_dn2_slot: &mut f64,
        var_psdl_dn6_slot: &mut f64,
        var_psdl_dn7_slot: &mut f64,
        var_q_b0_dep_slot: &mut f64,
        var_q_b0_dep_dn0_slot: &mut f64,
        var_q_b0_dep_dn10_slot: &mut f64,
        var_q_b0_dep_dn11_slot: &mut f64,
        var_q_b0_dep_dn12_slot: &mut f64,
        var_q_b0_dep_dn17_slot: &mut f64,
        var_q_b0_dep_dn2_slot: &mut f64,
        var_q_b0_dep_dn6_slot: &mut f64,
        var_q_b0_dep_dn7_slot: &mut f64,
        var_q_bl_dep_slot: &mut f64,
        var_q_bl_dep_dn0_slot: &mut f64,
        var_q_bl_dep_dn10_slot: &mut f64,
        var_q_bl_dep_dn11_slot: &mut f64,
        var_q_bl_dep_dn12_slot: &mut f64,
        var_q_bl_dep_dn17_slot: &mut f64,
        var_q_bl_dep_dn2_slot: &mut f64,
        var_q_bl_dep_dn6_slot: &mut f64,
        var_q_bl_dep_dn7_slot: &mut f64,
        var_q_dep0_slot: &mut f64,
        var_q_dep0_dn0_slot: &mut f64,
        var_q_dep0_dn10_slot: &mut f64,
        var_q_dep0_dn11_slot: &mut f64,
        var_q_dep0_dn12_slot: &mut f64,
        var_q_dep0_dn17_slot: &mut f64,
        var_q_dep0_dn2_slot: &mut f64,
        var_q_dep0_dn6_slot: &mut f64,
        var_q_dep0_dn7_slot: &mut f64,
        var_q_dep_soi_slot: &mut f64,
        var_q_dep_soi_dn0_slot: &mut f64,
        var_q_dep_soi_dn10_slot: &mut f64,
        var_q_dep_soi_dn11_slot: &mut f64,
        var_q_dep_soi_dn12_slot: &mut f64,
        var_q_dep_soi_dn17_slot: &mut f64,
        var_q_dep_soi_dn2_slot: &mut f64,
        var_q_dep_soi_dn6_slot: &mut f64,
        var_q_dep_soi_dn7_slot: &mut f64,
        var_q_depl_slot: &mut f64,
        var_q_depl_dn0_slot: &mut f64,
        var_q_depl_dn10_slot: &mut f64,
        var_q_depl_dn11_slot: &mut f64,
        var_q_depl_dn12_slot: &mut f64,
        var_q_depl_dn17_slot: &mut f64,
        var_q_depl_dn2_slot: &mut f64,
        var_q_depl_dn6_slot: &mut f64,
        var_q_depl_dn7_slot: &mut f64,
        var_q_n0_slot: &mut f64,
        var_q_n0_dn0_slot: &mut f64,
        var_q_n0_dn10_slot: &mut f64,
        var_q_n0_dn11_slot: &mut f64,
        var_q_n0_dn12_slot: &mut f64,
        var_q_n0_dn17_slot: &mut f64,
        var_q_n0_dn2_slot: &mut f64,
        var_q_n0_dn6_slot: &mut f64,
        var_q_n0_dn7_slot: &mut f64,
        var_q_nl_slot: &mut f64,
        var_q_nl_dn0_slot: &mut f64,
        var_q_nl_dn10_slot: &mut f64,
        var_q_nl_dn11_slot: &mut f64,
        var_q_nl_dn12_slot: &mut f64,
        var_q_nl_dn17_slot: &mut f64,
        var_q_nl_dn2_slot: &mut f64,
        var_q_nl_dn6_slot: &mut f64,
        var_q_nl_dn7_slot: &mut f64,
        var_q_s0_bulk_slot: &mut f64,
        var_q_s0_bulk_0_slot: &mut f64,
        var_q_s0_bulk_dn0_slot: &mut f64,
        var_q_s0_bulk_dn10_slot: &mut f64,
        var_q_s0_bulk_dn11_slot: &mut f64,
        var_q_s0_bulk_dn12_slot: &mut f64,
        var_q_s0_bulk_dn17_slot: &mut f64,
        var_q_s0_bulk_dn2_slot: &mut f64,
        var_q_s0_bulk_dn6_slot: &mut f64,
        var_q_s0_bulk_dn7_slot: &mut f64,
        var_q_sl_bulk_slot: &mut f64,
        var_q_sl_bulk_dn0_slot: &mut f64,
        var_q_sl_bulk_dn10_slot: &mut f64,
        var_q_sl_bulk_dn11_slot: &mut f64,
        var_q_sl_bulk_dn12_slot: &mut f64,
        var_q_sl_bulk_dn17_slot: &mut f64,
        var_q_sl_bulk_dn2_slot: &mut f64,
        var_q_sl_bulk_dn6_slot: &mut f64,
        var_q_sl_bulk_dn7_slot: &mut f64,
        var_qbu_slot: &mut f64,
        var_qbu_dn0_slot: &mut f64,
        var_qbu_dn10_slot: &mut f64,
        var_qbu_dn11_slot: &mut f64,
        var_qbu_dn12_slot: &mut f64,
        var_qbu_dn17_slot: &mut f64,
        var_qbu_dn2_slot: &mut f64,
        var_qbu_dn6_slot: &mut f64,
        var_qbu_dn7_slot: &mut f64,
        var_qd_fb_slot: &mut f64,
        var_qd_fb_dn0_slot: &mut f64,
        var_qd_fb_dn10_slot: &mut f64,
        var_qd_fb_dn11_slot: &mut f64,
        var_qd_fb_dn12_slot: &mut f64,
        var_qd_fb_dn13_slot: &mut f64,
        var_qd_fb_dn15_slot: &mut f64,
        var_qd_fb_dn16_slot: &mut f64,
        var_qd_fb_dn17_slot: &mut f64,
        var_qd_fb_dn18_slot: &mut f64,
        var_qd_fb_dn2_slot: &mut f64,
        var_qd_fb_dn6_slot: &mut f64,
        var_qd_fb_dn7_slot: &mut f64,
        var_qd_nqs_slot: &mut f64,
        var_qd_nqs_dn0_slot: &mut f64,
        var_qd_nqs_dn10_slot: &mut f64,
        var_qd_nqs_dn11_slot: &mut f64,
        var_qd_nqs_dn12_slot: &mut f64,
        var_qd_nqs_dn15_slot: &mut f64,
        var_qd_nqs_dn17_slot: &mut f64,
        var_qd_nqs_dn18_slot: &mut f64,
        var_qd_nqs_dn2_slot: &mut f64,
        var_qd_nqs_dn6_slot: &mut f64,
        var_qd_nqs_dn7_slot: &mut f64,
        var_qd_qs_slot: &mut f64,
        var_qd_qs_dn0_slot: &mut f64,
        var_qd_qs_dn10_slot: &mut f64,
        var_qd_qs_dn11_slot: &mut f64,
        var_qd_qs_dn12_slot: &mut f64,
        var_qd_qs_dn13_slot: &mut f64,
        var_qd_qs_dn15_slot: &mut f64,
        var_qd_qs_dn16_slot: &mut f64,
        var_qd_qs_dn17_slot: &mut f64,
        var_qd_qs_dn18_slot: &mut f64,
        var_qd_qs_dn2_slot: &mut f64,
        var_qd_qs_dn6_slot: &mut f64,
        var_qd_qs_dn7_slot: &mut f64,
        var_qdrat_slot: &mut f64,
        var_qdrat_dn0_slot: &mut f64,
        var_qdrat_dn10_slot: &mut f64,
        var_qdrat_dn11_slot: &mut f64,
        var_qdrat_dn12_slot: &mut f64,
        var_qdrat_dn17_slot: &mut f64,
        var_qdrat_dn2_slot: &mut f64,
        var_qdrat_dn6_slot: &mut f64,
        var_qdrat_dn7_slot: &mut f64,
        var_qdrat_noi_slot: &mut f64,
        var_qdrat_noi_dn0_slot: &mut f64,
        var_qdrat_noi_dn10_slot: &mut f64,
        var_qdrat_noi_dn11_slot: &mut f64,
        var_qdrat_noi_dn12_slot: &mut f64,
        var_qdrat_noi_dn17_slot: &mut f64,
        var_qdrat_noi_dn2_slot: &mut f64,
        var_qdrat_noi_dn6_slot: &mut f64,
        var_qdrat_noi_dn7_slot: &mut f64,
        var_qhs_slot: &mut f64,
        var_qhs_dn0_slot: &mut f64,
        var_qhs_dn10_slot: &mut f64,
        var_qhs_dn11_slot: &mut f64,
        var_qhs_dn12_slot: &mut f64,
        var_qhs_dn17_slot: &mut f64,
        var_qhs_dn2_slot: &mut f64,
        var_qhs_dn6_slot: &mut f64,
        var_qhs_dn7_slot: &mut f64,
        var_qi_nqs_slot: &mut f64,
        var_qi_nqs_dn18_slot: &mut f64,
        var_qiu_slot: &mut f64,
        var_qiu_dn0_slot: &mut f64,
        var_qiu_dn10_slot: &mut f64,
        var_qiu_dn11_slot: &mut f64,
        var_qiu_dn12_slot: &mut f64,
        var_qiu_dn17_slot: &mut f64,
        var_qiu_dn2_slot: &mut f64,
        var_qiu_dn6_slot: &mut f64,
        var_qiu_dn7_slot: &mut f64,
        var_qs_fb_slot: &mut f64,
        var_qs_fb_dn0_slot: &mut f64,
        var_qs_fb_dn10_slot: &mut f64,
        var_qs_fb_dn11_slot: &mut f64,
        var_qs_fb_dn12_slot: &mut f64,
        var_qs_fb_dn13_slot: &mut f64,
        var_qs_fb_dn15_slot: &mut f64,
        var_qs_fb_dn16_slot: &mut f64,
        var_qs_fb_dn17_slot: &mut f64,
        var_qs_fb_dn18_slot: &mut f64,
        var_qs_fb_dn2_slot: &mut f64,
        var_qs_fb_dn6_slot: &mut f64,
        var_qs_fb_dn7_slot: &mut f64,
        var_qs_nqs_slot: &mut f64,
        var_qs_nqs_dn0_slot: &mut f64,
        var_qs_nqs_dn10_slot: &mut f64,
        var_qs_nqs_dn11_slot: &mut f64,
        var_qs_nqs_dn12_slot: &mut f64,
        var_qs_nqs_dn16_slot: &mut f64,
        var_qs_nqs_dn17_slot: &mut f64,
        var_qs_nqs_dn18_slot: &mut f64,
        var_qs_nqs_dn2_slot: &mut f64,
        var_qs_nqs_dn6_slot: &mut f64,
        var_qs_nqs_dn7_slot: &mut f64,
        var_qsub_slot: &mut f64,
        var_qsub_dn0_slot: &mut f64,
        var_qsub_dn10_slot: &mut f64,
        var_qsub_dn11_slot: &mut f64,
        var_qsub_dn12_slot: &mut f64,
        var_qsub_dn17_slot: &mut f64,
        var_qsub_dn2_slot: &mut f64,
        var_qsub_dn6_slot: &mut f64,
        var_qsub_dn7_slot: &mut f64,
        var_shift_slot: &mut f64,
        var_shift_dn0_slot: &mut f64,
        var_shift_dn10_slot: &mut f64,
        var_shift_dn11_slot: &mut f64,
        var_shift_dn12_slot: &mut f64,
        var_shift_dn17_slot: &mut f64,
        var_shift_dn2_slot: &mut f64,
        var_shift_dn6_slot: &mut f64,
        var_shift_dn7_slot: &mut f64,
        var_sqrtkusail_slot: &mut f64,
        var_sqrtkusail_dn0_slot: &mut f64,
        var_sqrtkusail_dn10_slot: &mut f64,
        var_sqrtkusail_dn11_slot: &mut f64,
        var_sqrtkusail_dn12_slot: &mut f64,
        var_sqrtkusail_dn17_slot: &mut f64,
        var_sqrtkusail_dn2_slot: &mut f64,
        var_sqrtkusail_dn6_slot: &mut f64,
        var_sqrtkusail_dn7_slot: &mut f64,
        var_wdsoi_slot: &mut f64,
        var_wdsoi_dn0_slot: &mut f64,
        var_wdsoi_dn10_slot: &mut f64,
        var_wdsoi_dn11_slot: &mut f64,
        var_wdsoi_dn12_slot: &mut f64,
        var_wdsoi_dn17_slot: &mut f64,
        var_wdsoi_dn2_slot: &mut f64,
        var_wdsoi_dn6_slot: &mut f64,
        var_wdsoi_dn7_slot: &mut f64,
    ) {
        let mut var_crl_f: f64 = *var_crl_f_slot;
        let mut var_crl_f_dn0: f64 = *var_crl_f_dn0_slot;
        let mut var_crl_f_dn10: f64 = *var_crl_f_dn10_slot;
        let mut var_crl_f_dn11: f64 = *var_crl_f_dn11_slot;
        let mut var_crl_f_dn12: f64 = *var_crl_f_dn12_slot;
        let mut var_crl_f_dn17: f64 = *var_crl_f_dn17_slot;
        let mut var_crl_f_dn2: f64 = *var_crl_f_dn2_slot;
        let mut var_crl_f_dn6: f64 = *var_crl_f_dn6_slot;
        let mut var_crl_f_dn7: f64 = *var_crl_f_dn7_slot;
        let mut var_ec: f64 = *var_ec_slot;
        let mut var_ec_dn0: f64 = *var_ec_dn0_slot;
        let mut var_ec_dn10: f64 = *var_ec_dn10_slot;
        let mut var_ec_dn11: f64 = *var_ec_dn11_slot;
        let mut var_ec_dn12: f64 = *var_ec_dn12_slot;
        let mut var_ec_dn17: f64 = *var_ec_dn17_slot;
        let mut var_ec_dn2: f64 = *var_ec_dn2_slot;
        let mut var_ec_dn6: f64 = *var_ec_dn6_slot;
        let mut var_ec_dn7: f64 = *var_ec_dn7_slot;
        let mut var_flg_depmode: f64 = *var_flg_depmode_slot;
        let mut var_iqd_nqs: f64 = *var_iqd_nqs_slot;
        let mut var_iqd_nqs_dn0: f64 = *var_iqd_nqs_dn0_slot;
        let mut var_iqd_nqs_dn10: f64 = *var_iqd_nqs_dn10_slot;
        let mut var_iqd_nqs_dn11: f64 = *var_iqd_nqs_dn11_slot;
        let mut var_iqd_nqs_dn12: f64 = *var_iqd_nqs_dn12_slot;
        let mut var_iqd_nqs_dn13: f64 = *var_iqd_nqs_dn13_slot;
        let mut var_iqd_nqs_dn15: f64 = *var_iqd_nqs_dn15_slot;
        let mut var_iqd_nqs_dn16: f64 = *var_iqd_nqs_dn16_slot;
        let mut var_iqd_nqs_dn17: f64 = *var_iqd_nqs_dn17_slot;
        let mut var_iqd_nqs_dn18: f64 = *var_iqd_nqs_dn18_slot;
        let mut var_iqd_nqs_dn2: f64 = *var_iqd_nqs_dn2_slot;
        let mut var_iqd_nqs_dn6: f64 = *var_iqd_nqs_dn6_slot;
        let mut var_iqd_nqs_dn7: f64 = *var_iqd_nqs_dn7_slot;
        let mut var_iqh_nqs: f64 = *var_iqh_nqs_slot;
        let mut var_iqh_nqs_dn0: f64 = *var_iqh_nqs_dn0_slot;
        let mut var_iqh_nqs_dn10: f64 = *var_iqh_nqs_dn10_slot;
        let mut var_iqh_nqs_dn11: f64 = *var_iqh_nqs_dn11_slot;
        let mut var_iqh_nqs_dn12: f64 = *var_iqh_nqs_dn12_slot;
        let mut var_iqh_nqs_dn17: f64 = *var_iqh_nqs_dn17_slot;
        let mut var_iqh_nqs_dn2: f64 = *var_iqh_nqs_dn2_slot;
        let mut var_iqh_nqs_dn6: f64 = *var_iqh_nqs_dn6_slot;
        let mut var_iqh_nqs_dn7: f64 = *var_iqh_nqs_dn7_slot;
        let mut var_iqi_nqs: f64 = *var_iqi_nqs_slot;
        let mut var_iqi_nqs_dn0: f64 = *var_iqi_nqs_dn0_slot;
        let mut var_iqi_nqs_dn10: f64 = *var_iqi_nqs_dn10_slot;
        let mut var_iqi_nqs_dn11: f64 = *var_iqi_nqs_dn11_slot;
        let mut var_iqi_nqs_dn12: f64 = *var_iqi_nqs_dn12_slot;
        let mut var_iqi_nqs_dn17: f64 = *var_iqi_nqs_dn17_slot;
        let mut var_iqi_nqs_dn18: f64 = *var_iqi_nqs_dn18_slot;
        let mut var_iqi_nqs_dn2: f64 = *var_iqi_nqs_dn2_slot;
        let mut var_iqi_nqs_dn6: f64 = *var_iqi_nqs_dn6_slot;
        let mut var_iqi_nqs_dn7: f64 = *var_iqi_nqs_dn7_slot;
        let mut var_iqs_nqs: f64 = *var_iqs_nqs_slot;
        let mut var_iqs_nqs_dn0: f64 = *var_iqs_nqs_dn0_slot;
        let mut var_iqs_nqs_dn10: f64 = *var_iqs_nqs_dn10_slot;
        let mut var_iqs_nqs_dn11: f64 = *var_iqs_nqs_dn11_slot;
        let mut var_iqs_nqs_dn12: f64 = *var_iqs_nqs_dn12_slot;
        let mut var_iqs_nqs_dn13: f64 = *var_iqs_nqs_dn13_slot;
        let mut var_iqs_nqs_dn15: f64 = *var_iqs_nqs_dn15_slot;
        let mut var_iqs_nqs_dn16: f64 = *var_iqs_nqs_dn16_slot;
        let mut var_iqs_nqs_dn17: f64 = *var_iqs_nqs_dn17_slot;
        let mut var_iqs_nqs_dn18: f64 = *var_iqs_nqs_dn18_slot;
        let mut var_iqs_nqs_dn2: f64 = *var_iqs_nqs_dn2_slot;
        let mut var_iqs_nqs_dn6: f64 = *var_iqs_nqs_dn6_slot;
        let mut var_iqs_nqs_dn7: f64 = *var_iqs_nqs_dn7_slot;
        let mut var_kusai00: f64 = *var_kusai00_slot;
        let mut var_kusai00_dn0: f64 = *var_kusai00_dn0_slot;
        let mut var_kusai00_dn10: f64 = *var_kusai00_dn10_slot;
        let mut var_kusai00_dn11: f64 = *var_kusai00_dn11_slot;
        let mut var_kusai00_dn12: f64 = *var_kusai00_dn12_slot;
        let mut var_kusai00_dn17: f64 = *var_kusai00_dn17_slot;
        let mut var_kusai00_dn2: f64 = *var_kusai00_dn2_slot;
        let mut var_kusai00_dn6: f64 = *var_kusai00_dn6_slot;
        let mut var_kusai00_dn7: f64 = *var_kusai00_dn7_slot;
        let mut var_kusai00l: f64 = *var_kusai00l_slot;
        let mut var_kusai00l_dn0: f64 = *var_kusai00l_dn0_slot;
        let mut var_kusai00l_dn10: f64 = *var_kusai00l_dn10_slot;
        let mut var_kusai00l_dn11: f64 = *var_kusai00l_dn11_slot;
        let mut var_kusai00l_dn12: f64 = *var_kusai00l_dn12_slot;
        let mut var_kusai00l_dn17: f64 = *var_kusai00l_dn17_slot;
        let mut var_kusai00l_dn2: f64 = *var_kusai00l_dn2_slot;
        let mut var_kusai00l_dn6: f64 = *var_kusai00l_dn6_slot;
        let mut var_kusai00l_dn7: f64 = *var_kusai00l_dn7_slot;
        let mut var_kusai_ig: f64 = *var_kusai_ig_slot;
        let mut var_kusai_ig_dn0: f64 = *var_kusai_ig_dn0_slot;
        let mut var_kusai_ig_dn10: f64 = *var_kusai_ig_dn10_slot;
        let mut var_kusai_ig_dn11: f64 = *var_kusai_ig_dn11_slot;
        let mut var_kusai_ig_dn12: f64 = *var_kusai_ig_dn12_slot;
        let mut var_kusai_ig_dn17: f64 = *var_kusai_ig_dn17_slot;
        let mut var_kusai_ig_dn2: f64 = *var_kusai_ig_dn2_slot;
        let mut var_kusai_ig_dn6: f64 = *var_kusai_ig_dn6_slot;
        let mut var_kusai_ig_dn7: f64 = *var_kusai_ig_dn7_slot;
        let mut var_kusail: f64 = *var_kusail_slot;
        let mut var_kusail_dn0: f64 = *var_kusail_dn0_slot;
        let mut var_kusail_dn10: f64 = *var_kusail_dn10_slot;
        let mut var_kusail_dn11: f64 = *var_kusail_dn11_slot;
        let mut var_kusail_dn12: f64 = *var_kusail_dn12_slot;
        let mut var_kusail_dn17: f64 = *var_kusail_dn17_slot;
        let mut var_kusail_dn2: f64 = *var_kusail_dn2_slot;
        let mut var_kusail_dn6: f64 = *var_kusail_dn6_slot;
        let mut var_kusail_dn7: f64 = *var_kusail_dn7_slot;
        let mut var_lred: f64 = *var_lred_slot;
        let mut var_lred_dn0: f64 = *var_lred_dn0_slot;
        let mut var_lred_dn10: f64 = *var_lred_dn10_slot;
        let mut var_lred_dn11: f64 = *var_lred_dn11_slot;
        let mut var_lred_dn12: f64 = *var_lred_dn12_slot;
        let mut var_lred_dn17: f64 = *var_lred_dn17_slot;
        let mut var_lred_dn2: f64 = *var_lred_dn2_slot;
        let mut var_lred_dn6: f64 = *var_lred_dn6_slot;
        let mut var_lred_dn7: f64 = *var_lred_dn7_slot;
        let mut var_mud_hoso: f64 = *var_mud_hoso_slot;
        let mut var_mud_hoso_dn0: f64 = *var_mud_hoso_dn0_slot;
        let mut var_mud_hoso_dn10: f64 = *var_mud_hoso_dn10_slot;
        let mut var_mud_hoso_dn11: f64 = *var_mud_hoso_dn11_slot;
        let mut var_mud_hoso_dn12: f64 = *var_mud_hoso_dn12_slot;
        let mut var_mud_hoso_dn17: f64 = *var_mud_hoso_dn17_slot;
        let mut var_mud_hoso_dn2: f64 = *var_mud_hoso_dn2_slot;
        let mut var_mud_hoso_dn6: f64 = *var_mud_hoso_dn6_slot;
        let mut var_mud_hoso_dn7: f64 = *var_mud_hoso_dn7_slot;
        let mut var_phi_b0_soi: f64 = *var_phi_b0_soi_slot;
        let mut var_phi_b0_soi_dn0: f64 = *var_phi_b0_soi_dn0_slot;
        let mut var_phi_b0_soi_dn10: f64 = *var_phi_b0_soi_dn10_slot;
        let mut var_phi_b0_soi_dn11: f64 = *var_phi_b0_soi_dn11_slot;
        let mut var_phi_b0_soi_dn12: f64 = *var_phi_b0_soi_dn12_slot;
        let mut var_phi_b0_soi_dn17: f64 = *var_phi_b0_soi_dn17_slot;
        let mut var_phi_b0_soi_dn2: f64 = *var_phi_b0_soi_dn2_slot;
        let mut var_phi_b0_soi_dn6: f64 = *var_phi_b0_soi_dn6_slot;
        let mut var_phi_b0_soi_dn7: f64 = *var_phi_b0_soi_dn7_slot;
        let mut var_phi_b_dep0: f64 = *var_phi_b_dep0_slot;
        let mut var_phi_b_dep0_dn0: f64 = *var_phi_b_dep0_dn0_slot;
        let mut var_phi_b_dep0_dn10: f64 = *var_phi_b_dep0_dn10_slot;
        let mut var_phi_b_dep0_dn11: f64 = *var_phi_b_dep0_dn11_slot;
        let mut var_phi_b_dep0_dn12: f64 = *var_phi_b_dep0_dn12_slot;
        let mut var_phi_b_dep0_dn17: f64 = *var_phi_b_dep0_dn17_slot;
        let mut var_phi_b_dep0_dn2: f64 = *var_phi_b_dep0_dn2_slot;
        let mut var_phi_b_dep0_dn6: f64 = *var_phi_b_dep0_dn6_slot;
        let mut var_phi_b_dep0_dn7: f64 = *var_phi_b_dep0_dn7_slot;
        let mut var_phi_bl_soi: f64 = *var_phi_bl_soi_slot;
        let mut var_phi_bl_soi_dn0: f64 = *var_phi_bl_soi_dn0_slot;
        let mut var_phi_bl_soi_dn10: f64 = *var_phi_bl_soi_dn10_slot;
        let mut var_phi_bl_soi_dn11: f64 = *var_phi_bl_soi_dn11_slot;
        let mut var_phi_bl_soi_dn12: f64 = *var_phi_bl_soi_dn12_slot;
        let mut var_phi_bl_soi_dn17: f64 = *var_phi_bl_soi_dn17_slot;
        let mut var_phi_bl_soi_dn2: f64 = *var_phi_bl_soi_dn2_slot;
        let mut var_phi_bl_soi_dn6: f64 = *var_phi_bl_soi_dn6_slot;
        let mut var_phi_bl_soi_dn7: f64 = *var_phi_bl_soi_dn7_slot;
        let mut var_phi_bl_soi_ini: f64 = *var_phi_bl_soi_ini_slot;
        let mut var_phi_bl_soi_ini_dn0: f64 = *var_phi_bl_soi_ini_dn0_slot;
        let mut var_phi_bl_soi_ini_dn10: f64 = *var_phi_bl_soi_ini_dn10_slot;
        let mut var_phi_bl_soi_ini_dn11: f64 = *var_phi_bl_soi_ini_dn11_slot;
        let mut var_phi_bl_soi_ini_dn12: f64 = *var_phi_bl_soi_ini_dn12_slot;
        let mut var_phi_bl_soi_ini_dn17: f64 = *var_phi_bl_soi_ini_dn17_slot;
        let mut var_phi_bl_soi_ini_dn2: f64 = *var_phi_bl_soi_ini_dn2_slot;
        let mut var_phi_bl_soi_ini_dn6: f64 = *var_phi_bl_soi_ini_dn6_slot;
        let mut var_phi_bl_soi_ini_dn7: f64 = *var_phi_bl_soi_ini_dn7_slot;
        let mut var_phi_s0_bulk: f64 = *var_phi_s0_bulk_slot;
        let mut var_phi_s0_bulk_dn0: f64 = *var_phi_s0_bulk_dn0_slot;
        let mut var_phi_s0_bulk_dn10: f64 = *var_phi_s0_bulk_dn10_slot;
        let mut var_phi_s0_bulk_dn11: f64 = *var_phi_s0_bulk_dn11_slot;
        let mut var_phi_s0_bulk_dn12: f64 = *var_phi_s0_bulk_dn12_slot;
        let mut var_phi_s0_bulk_dn17: f64 = *var_phi_s0_bulk_dn17_slot;
        let mut var_phi_s0_bulk_dn2: f64 = *var_phi_s0_bulk_dn2_slot;
        let mut var_phi_s0_bulk_dn6: f64 = *var_phi_s0_bulk_dn6_slot;
        let mut var_phi_s0_bulk_dn7: f64 = *var_phi_s0_bulk_dn7_slot;
        let mut var_phi_s0_soi: f64 = *var_phi_s0_soi_slot;
        let mut var_phi_s0_soi_dn0: f64 = *var_phi_s0_soi_dn0_slot;
        let mut var_phi_s0_soi_dn10: f64 = *var_phi_s0_soi_dn10_slot;
        let mut var_phi_s0_soi_dn11: f64 = *var_phi_s0_soi_dn11_slot;
        let mut var_phi_s0_soi_dn12: f64 = *var_phi_s0_soi_dn12_slot;
        let mut var_phi_s0_soi_dn17: f64 = *var_phi_s0_soi_dn17_slot;
        let mut var_phi_s0_soi_dn2: f64 = *var_phi_s0_soi_dn2_slot;
        let mut var_phi_s0_soi_dn6: f64 = *var_phi_s0_soi_dn6_slot;
        let mut var_phi_s0_soi_dn7: f64 = *var_phi_s0_soi_dn7_slot;
        let mut var_phi_sl_bulk: f64 = *var_phi_sl_bulk_slot;
        let mut var_phi_sl_bulk_dn0: f64 = *var_phi_sl_bulk_dn0_slot;
        let mut var_phi_sl_bulk_dn10: f64 = *var_phi_sl_bulk_dn10_slot;
        let mut var_phi_sl_bulk_dn11: f64 = *var_phi_sl_bulk_dn11_slot;
        let mut var_phi_sl_bulk_dn12: f64 = *var_phi_sl_bulk_dn12_slot;
        let mut var_phi_sl_bulk_dn17: f64 = *var_phi_sl_bulk_dn17_slot;
        let mut var_phi_sl_bulk_dn2: f64 = *var_phi_sl_bulk_dn2_slot;
        let mut var_phi_sl_bulk_dn6: f64 = *var_phi_sl_bulk_dn6_slot;
        let mut var_phi_sl_bulk_dn7: f64 = *var_phi_sl_bulk_dn7_slot;
        let mut var_phi_sl_bulk_ini: f64 = *var_phi_sl_bulk_ini_slot;
        let mut var_phi_sl_bulk_ini_dn0: f64 = *var_phi_sl_bulk_ini_dn0_slot;
        let mut var_phi_sl_bulk_ini_dn10: f64 = *var_phi_sl_bulk_ini_dn10_slot;
        let mut var_phi_sl_bulk_ini_dn11: f64 = *var_phi_sl_bulk_ini_dn11_slot;
        let mut var_phi_sl_bulk_ini_dn12: f64 = *var_phi_sl_bulk_ini_dn12_slot;
        let mut var_phi_sl_bulk_ini_dn17: f64 = *var_phi_sl_bulk_ini_dn17_slot;
        let mut var_phi_sl_bulk_ini_dn2: f64 = *var_phi_sl_bulk_ini_dn2_slot;
        let mut var_phi_sl_bulk_ini_dn6: f64 = *var_phi_sl_bulk_ini_dn6_slot;
        let mut var_phi_sl_bulk_ini_dn7: f64 = *var_phi_sl_bulk_ini_dn7_slot;
        let mut var_phi_sl_soi: f64 = *var_phi_sl_soi_slot;
        let mut var_phi_sl_soi_dn0: f64 = *var_phi_sl_soi_dn0_slot;
        let mut var_phi_sl_soi_dn10: f64 = *var_phi_sl_soi_dn10_slot;
        let mut var_phi_sl_soi_dn11: f64 = *var_phi_sl_soi_dn11_slot;
        let mut var_phi_sl_soi_dn12: f64 = *var_phi_sl_soi_dn12_slot;
        let mut var_phi_sl_soi_dn17: f64 = *var_phi_sl_soi_dn17_slot;
        let mut var_phi_sl_soi_dn2: f64 = *var_phi_sl_soi_dn2_slot;
        let mut var_phi_sl_soi_dn6: f64 = *var_phi_sl_soi_dn6_slot;
        let mut var_phi_sl_soi_dn7: f64 = *var_phi_sl_soi_dn7_slot;
        let mut var_phi_sl_soi_ini: f64 = *var_phi_sl_soi_ini_slot;
        let mut var_phi_sl_soi_ini_dn0: f64 = *var_phi_sl_soi_ini_dn0_slot;
        let mut var_phi_sl_soi_ini_dn10: f64 = *var_phi_sl_soi_ini_dn10_slot;
        let mut var_phi_sl_soi_ini_dn11: f64 = *var_phi_sl_soi_ini_dn11_slot;
        let mut var_phi_sl_soi_ini_dn12: f64 = *var_phi_sl_soi_ini_dn12_slot;
        let mut var_phi_sl_soi_ini_dn17: f64 = *var_phi_sl_soi_ini_dn17_slot;
        let mut var_phi_sl_soi_ini_dn2: f64 = *var_phi_sl_soi_ini_dn2_slot;
        let mut var_phi_sl_soi_ini_dn6: f64 = *var_phi_sl_soi_ini_dn6_slot;
        let mut var_phi_sl_soi_ini_dn7: f64 = *var_phi_sl_soi_ini_dn7_slot;
        let mut var_ps0_inia: f64 = *var_ps0_inia_slot;
        let mut var_ps0_inia_dn0: f64 = *var_ps0_inia_dn0_slot;
        let mut var_ps0_inia_dn10: f64 = *var_ps0_inia_dn10_slot;
        let mut var_ps0_inia_dn11: f64 = *var_ps0_inia_dn11_slot;
        let mut var_ps0_inia_dn12: f64 = *var_ps0_inia_dn12_slot;
        let mut var_ps0_inia_dn17: f64 = *var_ps0_inia_dn17_slot;
        let mut var_ps0_inia_dn2: f64 = *var_ps0_inia_dn2_slot;
        let mut var_ps0_inia_dn6: f64 = *var_ps0_inia_dn6_slot;
        let mut var_ps0_inia_dn7: f64 = *var_ps0_inia_dn7_slot;
        let mut var_psdl: f64 = *var_psdl_slot;
        let mut var_psdl_dn0: f64 = *var_psdl_dn0_slot;
        let mut var_psdl_dn10: f64 = *var_psdl_dn10_slot;
        let mut var_psdl_dn11: f64 = *var_psdl_dn11_slot;
        let mut var_psdl_dn12: f64 = *var_psdl_dn12_slot;
        let mut var_psdl_dn17: f64 = *var_psdl_dn17_slot;
        let mut var_psdl_dn2: f64 = *var_psdl_dn2_slot;
        let mut var_psdl_dn6: f64 = *var_psdl_dn6_slot;
        let mut var_psdl_dn7: f64 = *var_psdl_dn7_slot;
        let mut var_q_b0_dep: f64 = *var_q_b0_dep_slot;
        let mut var_q_b0_dep_dn0: f64 = *var_q_b0_dep_dn0_slot;
        let mut var_q_b0_dep_dn10: f64 = *var_q_b0_dep_dn10_slot;
        let mut var_q_b0_dep_dn11: f64 = *var_q_b0_dep_dn11_slot;
        let mut var_q_b0_dep_dn12: f64 = *var_q_b0_dep_dn12_slot;
        let mut var_q_b0_dep_dn17: f64 = *var_q_b0_dep_dn17_slot;
        let mut var_q_b0_dep_dn2: f64 = *var_q_b0_dep_dn2_slot;
        let mut var_q_b0_dep_dn6: f64 = *var_q_b0_dep_dn6_slot;
        let mut var_q_b0_dep_dn7: f64 = *var_q_b0_dep_dn7_slot;
        let mut var_q_bl_dep: f64 = *var_q_bl_dep_slot;
        let mut var_q_bl_dep_dn0: f64 = *var_q_bl_dep_dn0_slot;
        let mut var_q_bl_dep_dn10: f64 = *var_q_bl_dep_dn10_slot;
        let mut var_q_bl_dep_dn11: f64 = *var_q_bl_dep_dn11_slot;
        let mut var_q_bl_dep_dn12: f64 = *var_q_bl_dep_dn12_slot;
        let mut var_q_bl_dep_dn17: f64 = *var_q_bl_dep_dn17_slot;
        let mut var_q_bl_dep_dn2: f64 = *var_q_bl_dep_dn2_slot;
        let mut var_q_bl_dep_dn6: f64 = *var_q_bl_dep_dn6_slot;
        let mut var_q_bl_dep_dn7: f64 = *var_q_bl_dep_dn7_slot;
        let mut var_q_dep0: f64 = *var_q_dep0_slot;
        let mut var_q_dep0_dn0: f64 = *var_q_dep0_dn0_slot;
        let mut var_q_dep0_dn10: f64 = *var_q_dep0_dn10_slot;
        let mut var_q_dep0_dn11: f64 = *var_q_dep0_dn11_slot;
        let mut var_q_dep0_dn12: f64 = *var_q_dep0_dn12_slot;
        let mut var_q_dep0_dn17: f64 = *var_q_dep0_dn17_slot;
        let mut var_q_dep0_dn2: f64 = *var_q_dep0_dn2_slot;
        let mut var_q_dep0_dn6: f64 = *var_q_dep0_dn6_slot;
        let mut var_q_dep0_dn7: f64 = *var_q_dep0_dn7_slot;
        let mut var_q_dep_soi: f64 = *var_q_dep_soi_slot;
        let mut var_q_dep_soi_dn0: f64 = *var_q_dep_soi_dn0_slot;
        let mut var_q_dep_soi_dn10: f64 = *var_q_dep_soi_dn10_slot;
        let mut var_q_dep_soi_dn11: f64 = *var_q_dep_soi_dn11_slot;
        let mut var_q_dep_soi_dn12: f64 = *var_q_dep_soi_dn12_slot;
        let mut var_q_dep_soi_dn17: f64 = *var_q_dep_soi_dn17_slot;
        let mut var_q_dep_soi_dn2: f64 = *var_q_dep_soi_dn2_slot;
        let mut var_q_dep_soi_dn6: f64 = *var_q_dep_soi_dn6_slot;
        let mut var_q_dep_soi_dn7: f64 = *var_q_dep_soi_dn7_slot;
        let mut var_q_depl: f64 = *var_q_depl_slot;
        let mut var_q_depl_dn0: f64 = *var_q_depl_dn0_slot;
        let mut var_q_depl_dn10: f64 = *var_q_depl_dn10_slot;
        let mut var_q_depl_dn11: f64 = *var_q_depl_dn11_slot;
        let mut var_q_depl_dn12: f64 = *var_q_depl_dn12_slot;
        let mut var_q_depl_dn17: f64 = *var_q_depl_dn17_slot;
        let mut var_q_depl_dn2: f64 = *var_q_depl_dn2_slot;
        let mut var_q_depl_dn6: f64 = *var_q_depl_dn6_slot;
        let mut var_q_depl_dn7: f64 = *var_q_depl_dn7_slot;
        let mut var_q_n0: f64 = *var_q_n0_slot;
        let mut var_q_n0_dn0: f64 = *var_q_n0_dn0_slot;
        let mut var_q_n0_dn10: f64 = *var_q_n0_dn10_slot;
        let mut var_q_n0_dn11: f64 = *var_q_n0_dn11_slot;
        let mut var_q_n0_dn12: f64 = *var_q_n0_dn12_slot;
        let mut var_q_n0_dn17: f64 = *var_q_n0_dn17_slot;
        let mut var_q_n0_dn2: f64 = *var_q_n0_dn2_slot;
        let mut var_q_n0_dn6: f64 = *var_q_n0_dn6_slot;
        let mut var_q_n0_dn7: f64 = *var_q_n0_dn7_slot;
        let mut var_q_nl: f64 = *var_q_nl_slot;
        let mut var_q_nl_dn0: f64 = *var_q_nl_dn0_slot;
        let mut var_q_nl_dn10: f64 = *var_q_nl_dn10_slot;
        let mut var_q_nl_dn11: f64 = *var_q_nl_dn11_slot;
        let mut var_q_nl_dn12: f64 = *var_q_nl_dn12_slot;
        let mut var_q_nl_dn17: f64 = *var_q_nl_dn17_slot;
        let mut var_q_nl_dn2: f64 = *var_q_nl_dn2_slot;
        let mut var_q_nl_dn6: f64 = *var_q_nl_dn6_slot;
        let mut var_q_nl_dn7: f64 = *var_q_nl_dn7_slot;
        let mut var_q_s0_bulk: f64 = *var_q_s0_bulk_slot;
        let mut var_q_s0_bulk_0: f64 = *var_q_s0_bulk_0_slot;
        let mut var_q_s0_bulk_dn0: f64 = *var_q_s0_bulk_dn0_slot;
        let mut var_q_s0_bulk_dn10: f64 = *var_q_s0_bulk_dn10_slot;
        let mut var_q_s0_bulk_dn11: f64 = *var_q_s0_bulk_dn11_slot;
        let mut var_q_s0_bulk_dn12: f64 = *var_q_s0_bulk_dn12_slot;
        let mut var_q_s0_bulk_dn17: f64 = *var_q_s0_bulk_dn17_slot;
        let mut var_q_s0_bulk_dn2: f64 = *var_q_s0_bulk_dn2_slot;
        let mut var_q_s0_bulk_dn6: f64 = *var_q_s0_bulk_dn6_slot;
        let mut var_q_s0_bulk_dn7: f64 = *var_q_s0_bulk_dn7_slot;
        let mut var_q_sl_bulk: f64 = *var_q_sl_bulk_slot;
        let mut var_q_sl_bulk_dn0: f64 = *var_q_sl_bulk_dn0_slot;
        let mut var_q_sl_bulk_dn10: f64 = *var_q_sl_bulk_dn10_slot;
        let mut var_q_sl_bulk_dn11: f64 = *var_q_sl_bulk_dn11_slot;
        let mut var_q_sl_bulk_dn12: f64 = *var_q_sl_bulk_dn12_slot;
        let mut var_q_sl_bulk_dn17: f64 = *var_q_sl_bulk_dn17_slot;
        let mut var_q_sl_bulk_dn2: f64 = *var_q_sl_bulk_dn2_slot;
        let mut var_q_sl_bulk_dn6: f64 = *var_q_sl_bulk_dn6_slot;
        let mut var_q_sl_bulk_dn7: f64 = *var_q_sl_bulk_dn7_slot;
        let mut var_qbu: f64 = *var_qbu_slot;
        let mut var_qbu_dn0: f64 = *var_qbu_dn0_slot;
        let mut var_qbu_dn10: f64 = *var_qbu_dn10_slot;
        let mut var_qbu_dn11: f64 = *var_qbu_dn11_slot;
        let mut var_qbu_dn12: f64 = *var_qbu_dn12_slot;
        let mut var_qbu_dn17: f64 = *var_qbu_dn17_slot;
        let mut var_qbu_dn2: f64 = *var_qbu_dn2_slot;
        let mut var_qbu_dn6: f64 = *var_qbu_dn6_slot;
        let mut var_qbu_dn7: f64 = *var_qbu_dn7_slot;
        let mut var_qd_fb: f64 = *var_qd_fb_slot;
        let mut var_qd_fb_dn0: f64 = *var_qd_fb_dn0_slot;
        let mut var_qd_fb_dn10: f64 = *var_qd_fb_dn10_slot;
        let mut var_qd_fb_dn11: f64 = *var_qd_fb_dn11_slot;
        let mut var_qd_fb_dn12: f64 = *var_qd_fb_dn12_slot;
        let mut var_qd_fb_dn13: f64 = *var_qd_fb_dn13_slot;
        let mut var_qd_fb_dn15: f64 = *var_qd_fb_dn15_slot;
        let mut var_qd_fb_dn16: f64 = *var_qd_fb_dn16_slot;
        let mut var_qd_fb_dn17: f64 = *var_qd_fb_dn17_slot;
        let mut var_qd_fb_dn18: f64 = *var_qd_fb_dn18_slot;
        let mut var_qd_fb_dn2: f64 = *var_qd_fb_dn2_slot;
        let mut var_qd_fb_dn6: f64 = *var_qd_fb_dn6_slot;
        let mut var_qd_fb_dn7: f64 = *var_qd_fb_dn7_slot;
        let mut var_qd_nqs: f64 = *var_qd_nqs_slot;
        let mut var_qd_nqs_dn0: f64 = *var_qd_nqs_dn0_slot;
        let mut var_qd_nqs_dn10: f64 = *var_qd_nqs_dn10_slot;
        let mut var_qd_nqs_dn11: f64 = *var_qd_nqs_dn11_slot;
        let mut var_qd_nqs_dn12: f64 = *var_qd_nqs_dn12_slot;
        let mut var_qd_nqs_dn15: f64 = *var_qd_nqs_dn15_slot;
        let mut var_qd_nqs_dn17: f64 = *var_qd_nqs_dn17_slot;
        let mut var_qd_nqs_dn18: f64 = *var_qd_nqs_dn18_slot;
        let mut var_qd_nqs_dn2: f64 = *var_qd_nqs_dn2_slot;
        let mut var_qd_nqs_dn6: f64 = *var_qd_nqs_dn6_slot;
        let mut var_qd_nqs_dn7: f64 = *var_qd_nqs_dn7_slot;
        let mut var_qd_qs: f64 = *var_qd_qs_slot;
        let mut var_qd_qs_dn0: f64 = *var_qd_qs_dn0_slot;
        let mut var_qd_qs_dn10: f64 = *var_qd_qs_dn10_slot;
        let mut var_qd_qs_dn11: f64 = *var_qd_qs_dn11_slot;
        let mut var_qd_qs_dn12: f64 = *var_qd_qs_dn12_slot;
        let mut var_qd_qs_dn13: f64 = *var_qd_qs_dn13_slot;
        let mut var_qd_qs_dn15: f64 = *var_qd_qs_dn15_slot;
        let mut var_qd_qs_dn16: f64 = *var_qd_qs_dn16_slot;
        let mut var_qd_qs_dn17: f64 = *var_qd_qs_dn17_slot;
        let mut var_qd_qs_dn18: f64 = *var_qd_qs_dn18_slot;
        let mut var_qd_qs_dn2: f64 = *var_qd_qs_dn2_slot;
        let mut var_qd_qs_dn6: f64 = *var_qd_qs_dn6_slot;
        let mut var_qd_qs_dn7: f64 = *var_qd_qs_dn7_slot;
        let mut var_qdrat: f64 = *var_qdrat_slot;
        let mut var_qdrat_dn0: f64 = *var_qdrat_dn0_slot;
        let mut var_qdrat_dn10: f64 = *var_qdrat_dn10_slot;
        let mut var_qdrat_dn11: f64 = *var_qdrat_dn11_slot;
        let mut var_qdrat_dn12: f64 = *var_qdrat_dn12_slot;
        let mut var_qdrat_dn17: f64 = *var_qdrat_dn17_slot;
        let mut var_qdrat_dn2: f64 = *var_qdrat_dn2_slot;
        let mut var_qdrat_dn6: f64 = *var_qdrat_dn6_slot;
        let mut var_qdrat_dn7: f64 = *var_qdrat_dn7_slot;
        let mut var_qdrat_noi: f64 = *var_qdrat_noi_slot;
        let mut var_qdrat_noi_dn0: f64 = *var_qdrat_noi_dn0_slot;
        let mut var_qdrat_noi_dn10: f64 = *var_qdrat_noi_dn10_slot;
        let mut var_qdrat_noi_dn11: f64 = *var_qdrat_noi_dn11_slot;
        let mut var_qdrat_noi_dn12: f64 = *var_qdrat_noi_dn12_slot;
        let mut var_qdrat_noi_dn17: f64 = *var_qdrat_noi_dn17_slot;
        let mut var_qdrat_noi_dn2: f64 = *var_qdrat_noi_dn2_slot;
        let mut var_qdrat_noi_dn6: f64 = *var_qdrat_noi_dn6_slot;
        let mut var_qdrat_noi_dn7: f64 = *var_qdrat_noi_dn7_slot;
        let mut var_qhs: f64 = *var_qhs_slot;
        let mut var_qhs_dn0: f64 = *var_qhs_dn0_slot;
        let mut var_qhs_dn10: f64 = *var_qhs_dn10_slot;
        let mut var_qhs_dn11: f64 = *var_qhs_dn11_slot;
        let mut var_qhs_dn12: f64 = *var_qhs_dn12_slot;
        let mut var_qhs_dn17: f64 = *var_qhs_dn17_slot;
        let mut var_qhs_dn2: f64 = *var_qhs_dn2_slot;
        let mut var_qhs_dn6: f64 = *var_qhs_dn6_slot;
        let mut var_qhs_dn7: f64 = *var_qhs_dn7_slot;
        let mut var_qi_nqs: f64 = *var_qi_nqs_slot;
        let mut var_qi_nqs_dn18: f64 = *var_qi_nqs_dn18_slot;
        let mut var_qiu: f64 = *var_qiu_slot;
        let mut var_qiu_dn0: f64 = *var_qiu_dn0_slot;
        let mut var_qiu_dn10: f64 = *var_qiu_dn10_slot;
        let mut var_qiu_dn11: f64 = *var_qiu_dn11_slot;
        let mut var_qiu_dn12: f64 = *var_qiu_dn12_slot;
        let mut var_qiu_dn17: f64 = *var_qiu_dn17_slot;
        let mut var_qiu_dn2: f64 = *var_qiu_dn2_slot;
        let mut var_qiu_dn6: f64 = *var_qiu_dn6_slot;
        let mut var_qiu_dn7: f64 = *var_qiu_dn7_slot;
        let mut var_qs_fb: f64 = *var_qs_fb_slot;
        let mut var_qs_fb_dn0: f64 = *var_qs_fb_dn0_slot;
        let mut var_qs_fb_dn10: f64 = *var_qs_fb_dn10_slot;
        let mut var_qs_fb_dn11: f64 = *var_qs_fb_dn11_slot;
        let mut var_qs_fb_dn12: f64 = *var_qs_fb_dn12_slot;
        let mut var_qs_fb_dn13: f64 = *var_qs_fb_dn13_slot;
        let mut var_qs_fb_dn15: f64 = *var_qs_fb_dn15_slot;
        let mut var_qs_fb_dn16: f64 = *var_qs_fb_dn16_slot;
        let mut var_qs_fb_dn17: f64 = *var_qs_fb_dn17_slot;
        let mut var_qs_fb_dn18: f64 = *var_qs_fb_dn18_slot;
        let mut var_qs_fb_dn2: f64 = *var_qs_fb_dn2_slot;
        let mut var_qs_fb_dn6: f64 = *var_qs_fb_dn6_slot;
        let mut var_qs_fb_dn7: f64 = *var_qs_fb_dn7_slot;
        let mut var_qs_nqs: f64 = *var_qs_nqs_slot;
        let mut var_qs_nqs_dn0: f64 = *var_qs_nqs_dn0_slot;
        let mut var_qs_nqs_dn10: f64 = *var_qs_nqs_dn10_slot;
        let mut var_qs_nqs_dn11: f64 = *var_qs_nqs_dn11_slot;
        let mut var_qs_nqs_dn12: f64 = *var_qs_nqs_dn12_slot;
        let mut var_qs_nqs_dn16: f64 = *var_qs_nqs_dn16_slot;
        let mut var_qs_nqs_dn17: f64 = *var_qs_nqs_dn17_slot;
        let mut var_qs_nqs_dn18: f64 = *var_qs_nqs_dn18_slot;
        let mut var_qs_nqs_dn2: f64 = *var_qs_nqs_dn2_slot;
        let mut var_qs_nqs_dn6: f64 = *var_qs_nqs_dn6_slot;
        let mut var_qs_nqs_dn7: f64 = *var_qs_nqs_dn7_slot;
        let mut var_qsub: f64 = *var_qsub_slot;
        let mut var_qsub_dn0: f64 = *var_qsub_dn0_slot;
        let mut var_qsub_dn10: f64 = *var_qsub_dn10_slot;
        let mut var_qsub_dn11: f64 = *var_qsub_dn11_slot;
        let mut var_qsub_dn12: f64 = *var_qsub_dn12_slot;
        let mut var_qsub_dn17: f64 = *var_qsub_dn17_slot;
        let mut var_qsub_dn2: f64 = *var_qsub_dn2_slot;
        let mut var_qsub_dn6: f64 = *var_qsub_dn6_slot;
        let mut var_qsub_dn7: f64 = *var_qsub_dn7_slot;
        let mut var_shift: f64 = *var_shift_slot;
        let mut var_shift_dn0: f64 = *var_shift_dn0_slot;
        let mut var_shift_dn10: f64 = *var_shift_dn10_slot;
        let mut var_shift_dn11: f64 = *var_shift_dn11_slot;
        let mut var_shift_dn12: f64 = *var_shift_dn12_slot;
        let mut var_shift_dn17: f64 = *var_shift_dn17_slot;
        let mut var_shift_dn2: f64 = *var_shift_dn2_slot;
        let mut var_shift_dn6: f64 = *var_shift_dn6_slot;
        let mut var_shift_dn7: f64 = *var_shift_dn7_slot;
        let mut var_sqrtkusail: f64 = *var_sqrtkusail_slot;
        let mut var_sqrtkusail_dn0: f64 = *var_sqrtkusail_dn0_slot;
        let mut var_sqrtkusail_dn10: f64 = *var_sqrtkusail_dn10_slot;
        let mut var_sqrtkusail_dn11: f64 = *var_sqrtkusail_dn11_slot;
        let mut var_sqrtkusail_dn12: f64 = *var_sqrtkusail_dn12_slot;
        let mut var_sqrtkusail_dn17: f64 = *var_sqrtkusail_dn17_slot;
        let mut var_sqrtkusail_dn2: f64 = *var_sqrtkusail_dn2_slot;
        let mut var_sqrtkusail_dn6: f64 = *var_sqrtkusail_dn6_slot;
        let mut var_sqrtkusail_dn7: f64 = *var_sqrtkusail_dn7_slot;
        let mut var_wdsoi: f64 = *var_wdsoi_slot;
        let mut var_wdsoi_dn0: f64 = *var_wdsoi_dn0_slot;
        let mut var_wdsoi_dn10: f64 = *var_wdsoi_dn10_slot;
        let mut var_wdsoi_dn11: f64 = *var_wdsoi_dn11_slot;
        let mut var_wdsoi_dn12: f64 = *var_wdsoi_dn12_slot;
        let mut var_wdsoi_dn17: f64 = *var_wdsoi_dn17_slot;
        let mut var_wdsoi_dn2: f64 = *var_wdsoi_dn2_slot;
        let mut var_wdsoi_dn6: f64 = *var_wdsoi_dn6_slot;
        let mut var_wdsoi_dn7: f64 = *var_wdsoi_dn7_slot;

        var_mud_hoso = 0.0;
        var_mud_hoso_dn0 = 0.0;
        var_mud_hoso_dn2 = 0.0;
        var_mud_hoso_dn6 = 0.0;
        var_mud_hoso_dn7 = 0.0;
        var_mud_hoso_dn10 = 0.0;
        var_mud_hoso_dn11 = 0.0;
        var_mud_hoso_dn12 = 0.0;
        var_mud_hoso_dn17 = 0.0;

        var_kusai00 = 0.0;
        var_kusai00_dn0 = 0.0;
        var_kusai00_dn2 = 0.0;
        var_kusai00_dn6 = 0.0;
        var_kusai00_dn7 = 0.0;
        var_kusai00_dn10 = 0.0;
        var_kusai00_dn11 = 0.0;
        var_kusai00_dn12 = 0.0;
        var_kusai00_dn17 = 0.0;

        var_kusail = 0.0;
        var_kusail_dn0 = 0.0;
        var_kusail_dn2 = 0.0;
        var_kusail_dn6 = 0.0;
        var_kusail_dn7 = 0.0;
        var_kusail_dn10 = 0.0;
        var_kusail_dn11 = 0.0;
        var_kusail_dn12 = 0.0;
        var_kusail_dn17 = 0.0;

        var_kusai00l = 0.0;
        var_kusai00l_dn0 = 0.0;
        var_kusai00l_dn2 = 0.0;
        var_kusai00l_dn6 = 0.0;
        var_kusai00l_dn7 = 0.0;
        var_kusai00l_dn10 = 0.0;
        var_kusai00l_dn11 = 0.0;
        var_kusai00l_dn12 = 0.0;
        var_kusai00l_dn17 = 0.0;

        var_sqrtkusail = 0.0;
        var_sqrtkusail_dn0 = 0.0;
        var_sqrtkusail_dn2 = 0.0;
        var_sqrtkusail_dn6 = 0.0;
        var_sqrtkusail_dn7 = 0.0;
        var_sqrtkusail_dn10 = 0.0;
        var_sqrtkusail_dn11 = 0.0;
        var_sqrtkusail_dn12 = 0.0;
        var_sqrtkusail_dn17 = 0.0;

        var_kusai_ig = 0.0;
        var_kusai_ig_dn0 = 0.0;
        var_kusai_ig_dn2 = 0.0;
        var_kusai_ig_dn6 = 0.0;
        var_kusai_ig_dn7 = 0.0;
        var_kusai_ig_dn10 = 0.0;
        var_kusai_ig_dn11 = 0.0;
        var_kusai_ig_dn12 = 0.0;
        var_kusai_ig_dn17 = 0.0;

        var_crl_f = 0.0;
        var_crl_f_dn0 = 0.0;
        var_crl_f_dn2 = 0.0;
        var_crl_f_dn6 = 0.0;
        var_crl_f_dn7 = 0.0;
        var_crl_f_dn10 = 0.0;
        var_crl_f_dn11 = 0.0;
        var_crl_f_dn12 = 0.0;
        var_crl_f_dn17 = 0.0;

        var_psdl = 0.0;
        var_psdl_dn0 = 0.0;
        var_psdl_dn2 = 0.0;
        var_psdl_dn6 = 0.0;
        var_psdl_dn7 = 0.0;
        var_psdl_dn10 = 0.0;
        var_psdl_dn11 = 0.0;
        var_psdl_dn12 = 0.0;
        var_psdl_dn17 = 0.0;

        var_ec = 0.0;
        var_ec_dn0 = 0.0;
        var_ec_dn2 = 0.0;
        var_ec_dn6 = 0.0;
        var_ec_dn7 = 0.0;
        var_ec_dn10 = 0.0;
        var_ec_dn11 = 0.0;
        var_ec_dn12 = 0.0;
        var_ec_dn17 = 0.0;

        var_lred = 0.0;
        var_lred_dn0 = 0.0;
        var_lred_dn2 = 0.0;
        var_lred_dn6 = 0.0;
        var_lred_dn7 = 0.0;
        var_lred_dn10 = 0.0;
        var_lred_dn11 = 0.0;
        var_lred_dn12 = 0.0;
        var_lred_dn17 = 0.0;

        var_flg_depmode = 0.0;

        var_phi_sl_soi_ini = 0.0;
        var_phi_sl_soi_ini_dn0 = 0.0;
        var_phi_sl_soi_ini_dn2 = 0.0;
        var_phi_sl_soi_ini_dn6 = 0.0;
        var_phi_sl_soi_ini_dn7 = 0.0;
        var_phi_sl_soi_ini_dn10 = 0.0;
        var_phi_sl_soi_ini_dn11 = 0.0;
        var_phi_sl_soi_ini_dn12 = 0.0;
        var_phi_sl_soi_ini_dn17 = 0.0;

        var_phi_bl_soi_ini = 0.0;
        var_phi_bl_soi_ini_dn0 = 0.0;
        var_phi_bl_soi_ini_dn2 = 0.0;
        var_phi_bl_soi_ini_dn6 = 0.0;
        var_phi_bl_soi_ini_dn7 = 0.0;
        var_phi_bl_soi_ini_dn10 = 0.0;
        var_phi_bl_soi_ini_dn11 = 0.0;
        var_phi_bl_soi_ini_dn12 = 0.0;
        var_phi_bl_soi_ini_dn17 = 0.0;

        var_phi_sl_bulk_ini = 0.0;
        var_phi_sl_bulk_ini_dn0 = 0.0;
        var_phi_sl_bulk_ini_dn2 = 0.0;
        var_phi_sl_bulk_ini_dn6 = 0.0;
        var_phi_sl_bulk_ini_dn7 = 0.0;
        var_phi_sl_bulk_ini_dn10 = 0.0;
        var_phi_sl_bulk_ini_dn11 = 0.0;
        var_phi_sl_bulk_ini_dn12 = 0.0;
        var_phi_sl_bulk_ini_dn17 = 0.0;

        var_phi_s0_soi = 0.0;
        var_phi_s0_soi_dn0 = 0.0;
        var_phi_s0_soi_dn2 = 0.0;
        var_phi_s0_soi_dn6 = 0.0;
        var_phi_s0_soi_dn7 = 0.0;
        var_phi_s0_soi_dn10 = 0.0;
        var_phi_s0_soi_dn11 = 0.0;
        var_phi_s0_soi_dn12 = 0.0;
        var_phi_s0_soi_dn17 = 0.0;

        var_phi_b0_soi = 0.0;
        var_phi_b0_soi_dn0 = 0.0;
        var_phi_b0_soi_dn2 = 0.0;
        var_phi_b0_soi_dn6 = 0.0;
        var_phi_b0_soi_dn7 = 0.0;
        var_phi_b0_soi_dn10 = 0.0;
        var_phi_b0_soi_dn11 = 0.0;
        var_phi_b0_soi_dn12 = 0.0;
        var_phi_b0_soi_dn17 = 0.0;

        var_phi_s0_bulk = 0.0;
        var_phi_s0_bulk_dn0 = 0.0;
        var_phi_s0_bulk_dn2 = 0.0;
        var_phi_s0_bulk_dn6 = 0.0;
        var_phi_s0_bulk_dn7 = 0.0;
        var_phi_s0_bulk_dn10 = 0.0;
        var_phi_s0_bulk_dn11 = 0.0;
        var_phi_s0_bulk_dn12 = 0.0;
        var_phi_s0_bulk_dn17 = 0.0;

        var_phi_sl_soi = 0.0;
        var_phi_sl_soi_dn0 = 0.0;
        var_phi_sl_soi_dn2 = 0.0;
        var_phi_sl_soi_dn6 = 0.0;
        var_phi_sl_soi_dn7 = 0.0;
        var_phi_sl_soi_dn10 = 0.0;
        var_phi_sl_soi_dn11 = 0.0;
        var_phi_sl_soi_dn12 = 0.0;
        var_phi_sl_soi_dn17 = 0.0;

        var_phi_bl_soi = 0.0;
        var_phi_bl_soi_dn0 = 0.0;
        var_phi_bl_soi_dn2 = 0.0;
        var_phi_bl_soi_dn6 = 0.0;
        var_phi_bl_soi_dn7 = 0.0;
        var_phi_bl_soi_dn10 = 0.0;
        var_phi_bl_soi_dn11 = 0.0;
        var_phi_bl_soi_dn12 = 0.0;
        var_phi_bl_soi_dn17 = 0.0;

        var_phi_sl_bulk = 0.0;
        var_phi_sl_bulk_dn0 = 0.0;
        var_phi_sl_bulk_dn2 = 0.0;
        var_phi_sl_bulk_dn6 = 0.0;
        var_phi_sl_bulk_dn7 = 0.0;
        var_phi_sl_bulk_dn10 = 0.0;
        var_phi_sl_bulk_dn11 = 0.0;
        var_phi_sl_bulk_dn12 = 0.0;
        var_phi_sl_bulk_dn17 = 0.0;

        var_q_dep_soi = 0.0;
        var_q_dep_soi_dn0 = 0.0;
        var_q_dep_soi_dn2 = 0.0;
        var_q_dep_soi_dn6 = 0.0;
        var_q_dep_soi_dn7 = 0.0;
        var_q_dep_soi_dn10 = 0.0;
        var_q_dep_soi_dn11 = 0.0;
        var_q_dep_soi_dn12 = 0.0;
        var_q_dep_soi_dn17 = 0.0;

        var_q_n0 = 0.0;
        var_q_n0_dn0 = 0.0;
        var_q_n0_dn2 = 0.0;
        var_q_n0_dn6 = 0.0;
        var_q_n0_dn7 = 0.0;
        var_q_n0_dn10 = 0.0;
        var_q_n0_dn11 = 0.0;
        var_q_n0_dn12 = 0.0;
        var_q_n0_dn17 = 0.0;

        var_q_b0_dep = 0.0;
        var_q_b0_dep_dn0 = 0.0;
        var_q_b0_dep_dn2 = 0.0;
        var_q_b0_dep_dn6 = 0.0;
        var_q_b0_dep_dn7 = 0.0;
        var_q_b0_dep_dn10 = 0.0;
        var_q_b0_dep_dn11 = 0.0;
        var_q_b0_dep_dn12 = 0.0;
        var_q_b0_dep_dn17 = 0.0;

        var_q_bl_dep = 0.0;
        var_q_bl_dep_dn0 = 0.0;
        var_q_bl_dep_dn2 = 0.0;
        var_q_bl_dep_dn6 = 0.0;
        var_q_bl_dep_dn7 = 0.0;
        var_q_bl_dep_dn10 = 0.0;
        var_q_bl_dep_dn11 = 0.0;
        var_q_bl_dep_dn12 = 0.0;
        var_q_bl_dep_dn17 = 0.0;

        var_q_dep0 = 0.0;
        var_q_dep0_dn0 = 0.0;
        var_q_dep0_dn2 = 0.0;
        var_q_dep0_dn6 = 0.0;
        var_q_dep0_dn7 = 0.0;
        var_q_dep0_dn10 = 0.0;
        var_q_dep0_dn11 = 0.0;
        var_q_dep0_dn12 = 0.0;
        var_q_dep0_dn17 = 0.0;

        var_q_s0_bulk = 0.0;
        var_q_s0_bulk_dn0 = 0.0;
        var_q_s0_bulk_dn2 = 0.0;
        var_q_s0_bulk_dn6 = 0.0;
        var_q_s0_bulk_dn7 = 0.0;
        var_q_s0_bulk_dn10 = 0.0;
        var_q_s0_bulk_dn11 = 0.0;
        var_q_s0_bulk_dn12 = 0.0;
        var_q_s0_bulk_dn17 = 0.0;

        var_q_nl = 0.0;
        var_q_nl_dn0 = 0.0;
        var_q_nl_dn2 = 0.0;
        var_q_nl_dn6 = 0.0;
        var_q_nl_dn7 = 0.0;
        var_q_nl_dn10 = 0.0;
        var_q_nl_dn11 = 0.0;
        var_q_nl_dn12 = 0.0;
        var_q_nl_dn17 = 0.0;

        var_q_depl = 0.0;
        var_q_depl_dn0 = 0.0;
        var_q_depl_dn2 = 0.0;
        var_q_depl_dn6 = 0.0;
        var_q_depl_dn7 = 0.0;
        var_q_depl_dn10 = 0.0;
        var_q_depl_dn11 = 0.0;
        var_q_depl_dn12 = 0.0;
        var_q_depl_dn17 = 0.0;

        var_q_sl_bulk = 0.0;
        var_q_sl_bulk_dn0 = 0.0;
        var_q_sl_bulk_dn2 = 0.0;
        var_q_sl_bulk_dn6 = 0.0;
        var_q_sl_bulk_dn7 = 0.0;
        var_q_sl_bulk_dn10 = 0.0;
        var_q_sl_bulk_dn11 = 0.0;
        var_q_sl_bulk_dn12 = 0.0;
        var_q_sl_bulk_dn17 = 0.0;

        var_shift = 0.0;
        var_shift_dn0 = 0.0;
        var_shift_dn2 = 0.0;
        var_shift_dn6 = 0.0;
        var_shift_dn7 = 0.0;
        var_shift_dn10 = 0.0;
        var_shift_dn11 = 0.0;
        var_shift_dn12 = 0.0;
        var_shift_dn17 = 0.0;

        var_q_s0_bulk_0 = 0.0;

        var_iqd_nqs = 0.0;
        var_iqd_nqs_dn0 = 0.0;
        var_iqd_nqs_dn2 = 0.0;
        var_iqd_nqs_dn6 = 0.0;
        var_iqd_nqs_dn7 = 0.0;
        var_iqd_nqs_dn10 = 0.0;
        var_iqd_nqs_dn11 = 0.0;
        var_iqd_nqs_dn12 = 0.0;
        var_iqd_nqs_dn13 = 0.0;
        var_iqd_nqs_dn15 = 0.0;
        var_iqd_nqs_dn16 = 0.0;
        var_iqd_nqs_dn17 = 0.0;
        var_iqd_nqs_dn18 = 0.0;

        var_iqs_nqs = 0.0;
        var_iqs_nqs_dn0 = 0.0;
        var_iqs_nqs_dn2 = 0.0;
        var_iqs_nqs_dn6 = 0.0;
        var_iqs_nqs_dn7 = 0.0;
        var_iqs_nqs_dn10 = 0.0;
        var_iqs_nqs_dn11 = 0.0;
        var_iqs_nqs_dn12 = 0.0;
        var_iqs_nqs_dn13 = 0.0;
        var_iqs_nqs_dn15 = 0.0;
        var_iqs_nqs_dn16 = 0.0;
        var_iqs_nqs_dn17 = 0.0;
        var_iqs_nqs_dn18 = 0.0;

        var_iqi_nqs = 0.0;
        var_iqi_nqs_dn0 = 0.0;
        var_iqi_nqs_dn2 = 0.0;
        var_iqi_nqs_dn6 = 0.0;
        var_iqi_nqs_dn7 = 0.0;
        var_iqi_nqs_dn10 = 0.0;
        var_iqi_nqs_dn11 = 0.0;
        var_iqi_nqs_dn12 = 0.0;
        var_iqi_nqs_dn17 = 0.0;
        var_iqi_nqs_dn18 = 0.0;

        var_qi_nqs = 0.0;
        var_qi_nqs_dn18 = 0.0;

        var_qd_nqs = 0.0;
        var_qd_nqs_dn0 = 0.0;
        var_qd_nqs_dn2 = 0.0;
        var_qd_nqs_dn6 = 0.0;
        var_qd_nqs_dn7 = 0.0;
        var_qd_nqs_dn10 = 0.0;
        var_qd_nqs_dn11 = 0.0;
        var_qd_nqs_dn12 = 0.0;
        var_qd_nqs_dn15 = 0.0;
        var_qd_nqs_dn17 = 0.0;
        var_qd_nqs_dn18 = 0.0;

        var_qs_nqs = 0.0;
        var_qs_nqs_dn0 = 0.0;
        var_qs_nqs_dn2 = 0.0;
        var_qs_nqs_dn6 = 0.0;
        var_qs_nqs_dn7 = 0.0;
        var_qs_nqs_dn10 = 0.0;
        var_qs_nqs_dn11 = 0.0;
        var_qs_nqs_dn12 = 0.0;
        var_qs_nqs_dn16 = 0.0;
        var_qs_nqs_dn17 = 0.0;
        var_qs_nqs_dn18 = 0.0;

        var_phi_b_dep0 = 0.0;
        var_phi_b_dep0_dn0 = 0.0;
        var_phi_b_dep0_dn2 = 0.0;
        var_phi_b_dep0_dn6 = 0.0;
        var_phi_b_dep0_dn7 = 0.0;
        var_phi_b_dep0_dn10 = 0.0;
        var_phi_b_dep0_dn11 = 0.0;
        var_phi_b_dep0_dn12 = 0.0;
        var_phi_b_dep0_dn17 = 0.0;

        var_qsub = 0.0;
        var_qsub_dn0 = 0.0;
        var_qsub_dn2 = 0.0;
        var_qsub_dn6 = 0.0;
        var_qsub_dn7 = 0.0;
        var_qsub_dn10 = 0.0;
        var_qsub_dn11 = 0.0;
        var_qsub_dn12 = 0.0;
        var_qsub_dn17 = 0.0;

        var_qhs = 0.0;
        var_qhs_dn0 = 0.0;
        var_qhs_dn2 = 0.0;
        var_qhs_dn6 = 0.0;
        var_qhs_dn7 = 0.0;
        var_qhs_dn10 = 0.0;
        var_qhs_dn11 = 0.0;
        var_qhs_dn12 = 0.0;
        var_qhs_dn17 = 0.0;

        var_wdsoi = 0.0;
        var_wdsoi_dn0 = 0.0;
        var_wdsoi_dn2 = 0.0;
        var_wdsoi_dn6 = 0.0;
        var_wdsoi_dn7 = 0.0;
        var_wdsoi_dn10 = 0.0;
        var_wdsoi_dn11 = 0.0;
        var_wdsoi_dn12 = 0.0;
        var_wdsoi_dn17 = 0.0;

        var_ps0_inia = 0.0;
        var_ps0_inia_dn0 = 0.0;
        var_ps0_inia_dn2 = 0.0;
        var_ps0_inia_dn6 = 0.0;
        var_ps0_inia_dn7 = 0.0;
        var_ps0_inia_dn10 = 0.0;
        var_ps0_inia_dn11 = 0.0;
        var_ps0_inia_dn12 = 0.0;
        var_ps0_inia_dn17 = 0.0;

        var_qiu = 0.0;
        var_qiu_dn0 = 0.0;
        var_qiu_dn2 = 0.0;
        var_qiu_dn6 = 0.0;
        var_qiu_dn7 = 0.0;
        var_qiu_dn10 = 0.0;
        var_qiu_dn11 = 0.0;
        var_qiu_dn12 = 0.0;
        var_qiu_dn17 = 0.0;

        var_qbu = 0.0;
        var_qbu_dn0 = 0.0;
        var_qbu_dn2 = 0.0;
        var_qbu_dn6 = 0.0;
        var_qbu_dn7 = 0.0;
        var_qbu_dn10 = 0.0;
        var_qbu_dn11 = 0.0;
        var_qbu_dn12 = 0.0;
        var_qbu_dn17 = 0.0;

        var_qdrat = 0.5;
        var_qdrat_dn0 = 0.0;
        var_qdrat_dn2 = 0.0;
        var_qdrat_dn6 = 0.0;
        var_qdrat_dn7 = 0.0;
        var_qdrat_dn10 = 0.0;
        var_qdrat_dn11 = 0.0;
        var_qdrat_dn12 = 0.0;
        var_qdrat_dn17 = 0.0;

        var_qdrat_noi = 0.5;
        var_qdrat_noi_dn0 = 0.0;
        var_qdrat_noi_dn2 = 0.0;
        var_qdrat_noi_dn6 = 0.0;
        var_qdrat_noi_dn7 = 0.0;
        var_qdrat_noi_dn10 = 0.0;
        var_qdrat_noi_dn11 = 0.0;
        var_qdrat_noi_dn12 = 0.0;
        var_qdrat_noi_dn17 = 0.0;

        var_qs_fb = 0.0;
        var_qs_fb_dn0 = 0.0;
        var_qs_fb_dn2 = 0.0;
        var_qs_fb_dn6 = 0.0;
        var_qs_fb_dn7 = 0.0;
        var_qs_fb_dn10 = 0.0;
        var_qs_fb_dn11 = 0.0;
        var_qs_fb_dn12 = 0.0;
        var_qs_fb_dn13 = 0.0;
        var_qs_fb_dn15 = 0.0;
        var_qs_fb_dn16 = 0.0;
        var_qs_fb_dn17 = 0.0;
        var_qs_fb_dn18 = 0.0;

        var_qd_fb = 0.0;
        var_qd_fb_dn0 = 0.0;
        var_qd_fb_dn2 = 0.0;
        var_qd_fb_dn6 = 0.0;
        var_qd_fb_dn7 = 0.0;
        var_qd_fb_dn10 = 0.0;
        var_qd_fb_dn11 = 0.0;
        var_qd_fb_dn12 = 0.0;
        var_qd_fb_dn13 = 0.0;
        var_qd_fb_dn15 = 0.0;
        var_qd_fb_dn16 = 0.0;
        var_qd_fb_dn17 = 0.0;
        var_qd_fb_dn18 = 0.0;

        var_iqh_nqs = 0.0;
        var_iqh_nqs_dn0 = 0.0;
        var_iqh_nqs_dn2 = 0.0;
        var_iqh_nqs_dn6 = 0.0;
        var_iqh_nqs_dn7 = 0.0;
        var_iqh_nqs_dn10 = 0.0;
        var_iqh_nqs_dn11 = 0.0;
        var_iqh_nqs_dn12 = 0.0;
        var_iqh_nqs_dn17 = 0.0;

        var_qd_qs = 0.0;
        var_qd_qs_dn0 = 0.0;
        var_qd_qs_dn2 = 0.0;
        var_qd_qs_dn6 = 0.0;
        var_qd_qs_dn7 = 0.0;
        var_qd_qs_dn10 = 0.0;
        var_qd_qs_dn11 = 0.0;
        var_qd_qs_dn12 = 0.0;
        var_qd_qs_dn13 = 0.0;
        var_qd_qs_dn15 = 0.0;
        var_qd_qs_dn16 = 0.0;
        var_qd_qs_dn17 = 0.0;
        var_qd_qs_dn18 = 0.0;

        *var_crl_f_slot = var_crl_f;
        *var_crl_f_dn0_slot = var_crl_f_dn0;
        *var_crl_f_dn10_slot = var_crl_f_dn10;
        *var_crl_f_dn11_slot = var_crl_f_dn11;
        *var_crl_f_dn12_slot = var_crl_f_dn12;
        *var_crl_f_dn17_slot = var_crl_f_dn17;
        *var_crl_f_dn2_slot = var_crl_f_dn2;
        *var_crl_f_dn6_slot = var_crl_f_dn6;
        *var_crl_f_dn7_slot = var_crl_f_dn7;
        *var_ec_slot = var_ec;
        *var_ec_dn0_slot = var_ec_dn0;
        *var_ec_dn10_slot = var_ec_dn10;
        *var_ec_dn11_slot = var_ec_dn11;
        *var_ec_dn12_slot = var_ec_dn12;
        *var_ec_dn17_slot = var_ec_dn17;
        *var_ec_dn2_slot = var_ec_dn2;
        *var_ec_dn6_slot = var_ec_dn6;
        *var_ec_dn7_slot = var_ec_dn7;
        *var_flg_depmode_slot = var_flg_depmode;
        *var_iqd_nqs_slot = var_iqd_nqs;
        *var_iqd_nqs_dn0_slot = var_iqd_nqs_dn0;
        *var_iqd_nqs_dn10_slot = var_iqd_nqs_dn10;
        *var_iqd_nqs_dn11_slot = var_iqd_nqs_dn11;
        *var_iqd_nqs_dn12_slot = var_iqd_nqs_dn12;
        *var_iqd_nqs_dn13_slot = var_iqd_nqs_dn13;
        *var_iqd_nqs_dn15_slot = var_iqd_nqs_dn15;
        *var_iqd_nqs_dn16_slot = var_iqd_nqs_dn16;
        *var_iqd_nqs_dn17_slot = var_iqd_nqs_dn17;
        *var_iqd_nqs_dn18_slot = var_iqd_nqs_dn18;
        *var_iqd_nqs_dn2_slot = var_iqd_nqs_dn2;
        *var_iqd_nqs_dn6_slot = var_iqd_nqs_dn6;
        *var_iqd_nqs_dn7_slot = var_iqd_nqs_dn7;
        *var_iqh_nqs_slot = var_iqh_nqs;
        *var_iqh_nqs_dn0_slot = var_iqh_nqs_dn0;
        *var_iqh_nqs_dn10_slot = var_iqh_nqs_dn10;
        *var_iqh_nqs_dn11_slot = var_iqh_nqs_dn11;
        *var_iqh_nqs_dn12_slot = var_iqh_nqs_dn12;
        *var_iqh_nqs_dn17_slot = var_iqh_nqs_dn17;
        *var_iqh_nqs_dn2_slot = var_iqh_nqs_dn2;
        *var_iqh_nqs_dn6_slot = var_iqh_nqs_dn6;
        *var_iqh_nqs_dn7_slot = var_iqh_nqs_dn7;
        *var_iqi_nqs_slot = var_iqi_nqs;
        *var_iqi_nqs_dn0_slot = var_iqi_nqs_dn0;
        *var_iqi_nqs_dn10_slot = var_iqi_nqs_dn10;
        *var_iqi_nqs_dn11_slot = var_iqi_nqs_dn11;
        *var_iqi_nqs_dn12_slot = var_iqi_nqs_dn12;
        *var_iqi_nqs_dn17_slot = var_iqi_nqs_dn17;
        *var_iqi_nqs_dn18_slot = var_iqi_nqs_dn18;
        *var_iqi_nqs_dn2_slot = var_iqi_nqs_dn2;
        *var_iqi_nqs_dn6_slot = var_iqi_nqs_dn6;
        *var_iqi_nqs_dn7_slot = var_iqi_nqs_dn7;
        *var_iqs_nqs_slot = var_iqs_nqs;
        *var_iqs_nqs_dn0_slot = var_iqs_nqs_dn0;
        *var_iqs_nqs_dn10_slot = var_iqs_nqs_dn10;
        *var_iqs_nqs_dn11_slot = var_iqs_nqs_dn11;
        *var_iqs_nqs_dn12_slot = var_iqs_nqs_dn12;
        *var_iqs_nqs_dn13_slot = var_iqs_nqs_dn13;
        *var_iqs_nqs_dn15_slot = var_iqs_nqs_dn15;
        *var_iqs_nqs_dn16_slot = var_iqs_nqs_dn16;
        *var_iqs_nqs_dn17_slot = var_iqs_nqs_dn17;
        *var_iqs_nqs_dn18_slot = var_iqs_nqs_dn18;
        *var_iqs_nqs_dn2_slot = var_iqs_nqs_dn2;
        *var_iqs_nqs_dn6_slot = var_iqs_nqs_dn6;
        *var_iqs_nqs_dn7_slot = var_iqs_nqs_dn7;
        *var_kusai00_slot = var_kusai00;
        *var_kusai00_dn0_slot = var_kusai00_dn0;
        *var_kusai00_dn10_slot = var_kusai00_dn10;
        *var_kusai00_dn11_slot = var_kusai00_dn11;
        *var_kusai00_dn12_slot = var_kusai00_dn12;
        *var_kusai00_dn17_slot = var_kusai00_dn17;
        *var_kusai00_dn2_slot = var_kusai00_dn2;
        *var_kusai00_dn6_slot = var_kusai00_dn6;
        *var_kusai00_dn7_slot = var_kusai00_dn7;
        *var_kusai00l_slot = var_kusai00l;
        *var_kusai00l_dn0_slot = var_kusai00l_dn0;
        *var_kusai00l_dn10_slot = var_kusai00l_dn10;
        *var_kusai00l_dn11_slot = var_kusai00l_dn11;
        *var_kusai00l_dn12_slot = var_kusai00l_dn12;
        *var_kusai00l_dn17_slot = var_kusai00l_dn17;
        *var_kusai00l_dn2_slot = var_kusai00l_dn2;
        *var_kusai00l_dn6_slot = var_kusai00l_dn6;
        *var_kusai00l_dn7_slot = var_kusai00l_dn7;
        *var_kusai_ig_slot = var_kusai_ig;
        *var_kusai_ig_dn0_slot = var_kusai_ig_dn0;
        *var_kusai_ig_dn10_slot = var_kusai_ig_dn10;
        *var_kusai_ig_dn11_slot = var_kusai_ig_dn11;
        *var_kusai_ig_dn12_slot = var_kusai_ig_dn12;
        *var_kusai_ig_dn17_slot = var_kusai_ig_dn17;
        *var_kusai_ig_dn2_slot = var_kusai_ig_dn2;
        *var_kusai_ig_dn6_slot = var_kusai_ig_dn6;
        *var_kusai_ig_dn7_slot = var_kusai_ig_dn7;
        *var_kusail_slot = var_kusail;
        *var_kusail_dn0_slot = var_kusail_dn0;
        *var_kusail_dn10_slot = var_kusail_dn10;
        *var_kusail_dn11_slot = var_kusail_dn11;
        *var_kusail_dn12_slot = var_kusail_dn12;
        *var_kusail_dn17_slot = var_kusail_dn17;
        *var_kusail_dn2_slot = var_kusail_dn2;
        *var_kusail_dn6_slot = var_kusail_dn6;
        *var_kusail_dn7_slot = var_kusail_dn7;
        *var_lred_slot = var_lred;
        *var_lred_dn0_slot = var_lred_dn0;
        *var_lred_dn10_slot = var_lred_dn10;
        *var_lred_dn11_slot = var_lred_dn11;
        *var_lred_dn12_slot = var_lred_dn12;
        *var_lred_dn17_slot = var_lred_dn17;
        *var_lred_dn2_slot = var_lred_dn2;
        *var_lred_dn6_slot = var_lred_dn6;
        *var_lred_dn7_slot = var_lred_dn7;
        *var_mud_hoso_slot = var_mud_hoso;
        *var_mud_hoso_dn0_slot = var_mud_hoso_dn0;
        *var_mud_hoso_dn10_slot = var_mud_hoso_dn10;
        *var_mud_hoso_dn11_slot = var_mud_hoso_dn11;
        *var_mud_hoso_dn12_slot = var_mud_hoso_dn12;
        *var_mud_hoso_dn17_slot = var_mud_hoso_dn17;
        *var_mud_hoso_dn2_slot = var_mud_hoso_dn2;
        *var_mud_hoso_dn6_slot = var_mud_hoso_dn6;
        *var_mud_hoso_dn7_slot = var_mud_hoso_dn7;
        *var_phi_b0_soi_slot = var_phi_b0_soi;
        *var_phi_b0_soi_dn0_slot = var_phi_b0_soi_dn0;
        *var_phi_b0_soi_dn10_slot = var_phi_b0_soi_dn10;
        *var_phi_b0_soi_dn11_slot = var_phi_b0_soi_dn11;
        *var_phi_b0_soi_dn12_slot = var_phi_b0_soi_dn12;
        *var_phi_b0_soi_dn17_slot = var_phi_b0_soi_dn17;
        *var_phi_b0_soi_dn2_slot = var_phi_b0_soi_dn2;
        *var_phi_b0_soi_dn6_slot = var_phi_b0_soi_dn6;
        *var_phi_b0_soi_dn7_slot = var_phi_b0_soi_dn7;
        *var_phi_b_dep0_slot = var_phi_b_dep0;
        *var_phi_b_dep0_dn0_slot = var_phi_b_dep0_dn0;
        *var_phi_b_dep0_dn10_slot = var_phi_b_dep0_dn10;
        *var_phi_b_dep0_dn11_slot = var_phi_b_dep0_dn11;
        *var_phi_b_dep0_dn12_slot = var_phi_b_dep0_dn12;
        *var_phi_b_dep0_dn17_slot = var_phi_b_dep0_dn17;
        *var_phi_b_dep0_dn2_slot = var_phi_b_dep0_dn2;
        *var_phi_b_dep0_dn6_slot = var_phi_b_dep0_dn6;
        *var_phi_b_dep0_dn7_slot = var_phi_b_dep0_dn7;
        *var_phi_bl_soi_slot = var_phi_bl_soi;
        *var_phi_bl_soi_dn0_slot = var_phi_bl_soi_dn0;
        *var_phi_bl_soi_dn10_slot = var_phi_bl_soi_dn10;
        *var_phi_bl_soi_dn11_slot = var_phi_bl_soi_dn11;
        *var_phi_bl_soi_dn12_slot = var_phi_bl_soi_dn12;
        *var_phi_bl_soi_dn17_slot = var_phi_bl_soi_dn17;
        *var_phi_bl_soi_dn2_slot = var_phi_bl_soi_dn2;
        *var_phi_bl_soi_dn6_slot = var_phi_bl_soi_dn6;
        *var_phi_bl_soi_dn7_slot = var_phi_bl_soi_dn7;
        *var_phi_bl_soi_ini_slot = var_phi_bl_soi_ini;
        *var_phi_bl_soi_ini_dn0_slot = var_phi_bl_soi_ini_dn0;
        *var_phi_bl_soi_ini_dn10_slot = var_phi_bl_soi_ini_dn10;
        *var_phi_bl_soi_ini_dn11_slot = var_phi_bl_soi_ini_dn11;
        *var_phi_bl_soi_ini_dn12_slot = var_phi_bl_soi_ini_dn12;
        *var_phi_bl_soi_ini_dn17_slot = var_phi_bl_soi_ini_dn17;
        *var_phi_bl_soi_ini_dn2_slot = var_phi_bl_soi_ini_dn2;
        *var_phi_bl_soi_ini_dn6_slot = var_phi_bl_soi_ini_dn6;
        *var_phi_bl_soi_ini_dn7_slot = var_phi_bl_soi_ini_dn7;
        *var_phi_s0_bulk_slot = var_phi_s0_bulk;
        *var_phi_s0_bulk_dn0_slot = var_phi_s0_bulk_dn0;
        *var_phi_s0_bulk_dn10_slot = var_phi_s0_bulk_dn10;
        *var_phi_s0_bulk_dn11_slot = var_phi_s0_bulk_dn11;
        *var_phi_s0_bulk_dn12_slot = var_phi_s0_bulk_dn12;
        *var_phi_s0_bulk_dn17_slot = var_phi_s0_bulk_dn17;
        *var_phi_s0_bulk_dn2_slot = var_phi_s0_bulk_dn2;
        *var_phi_s0_bulk_dn6_slot = var_phi_s0_bulk_dn6;
        *var_phi_s0_bulk_dn7_slot = var_phi_s0_bulk_dn7;
        *var_phi_s0_soi_slot = var_phi_s0_soi;
        *var_phi_s0_soi_dn0_slot = var_phi_s0_soi_dn0;
        *var_phi_s0_soi_dn10_slot = var_phi_s0_soi_dn10;
        *var_phi_s0_soi_dn11_slot = var_phi_s0_soi_dn11;
        *var_phi_s0_soi_dn12_slot = var_phi_s0_soi_dn12;
        *var_phi_s0_soi_dn17_slot = var_phi_s0_soi_dn17;
        *var_phi_s0_soi_dn2_slot = var_phi_s0_soi_dn2;
        *var_phi_s0_soi_dn6_slot = var_phi_s0_soi_dn6;
        *var_phi_s0_soi_dn7_slot = var_phi_s0_soi_dn7;
        *var_phi_sl_bulk_slot = var_phi_sl_bulk;
        *var_phi_sl_bulk_dn0_slot = var_phi_sl_bulk_dn0;
        *var_phi_sl_bulk_dn10_slot = var_phi_sl_bulk_dn10;
        *var_phi_sl_bulk_dn11_slot = var_phi_sl_bulk_dn11;
        *var_phi_sl_bulk_dn12_slot = var_phi_sl_bulk_dn12;
        *var_phi_sl_bulk_dn17_slot = var_phi_sl_bulk_dn17;
        *var_phi_sl_bulk_dn2_slot = var_phi_sl_bulk_dn2;
        *var_phi_sl_bulk_dn6_slot = var_phi_sl_bulk_dn6;
        *var_phi_sl_bulk_dn7_slot = var_phi_sl_bulk_dn7;
        *var_phi_sl_bulk_ini_slot = var_phi_sl_bulk_ini;
        *var_phi_sl_bulk_ini_dn0_slot = var_phi_sl_bulk_ini_dn0;
        *var_phi_sl_bulk_ini_dn10_slot = var_phi_sl_bulk_ini_dn10;
        *var_phi_sl_bulk_ini_dn11_slot = var_phi_sl_bulk_ini_dn11;
        *var_phi_sl_bulk_ini_dn12_slot = var_phi_sl_bulk_ini_dn12;
        *var_phi_sl_bulk_ini_dn17_slot = var_phi_sl_bulk_ini_dn17;
        *var_phi_sl_bulk_ini_dn2_slot = var_phi_sl_bulk_ini_dn2;
        *var_phi_sl_bulk_ini_dn6_slot = var_phi_sl_bulk_ini_dn6;
        *var_phi_sl_bulk_ini_dn7_slot = var_phi_sl_bulk_ini_dn7;
        *var_phi_sl_soi_slot = var_phi_sl_soi;
        *var_phi_sl_soi_dn0_slot = var_phi_sl_soi_dn0;
        *var_phi_sl_soi_dn10_slot = var_phi_sl_soi_dn10;
        *var_phi_sl_soi_dn11_slot = var_phi_sl_soi_dn11;
        *var_phi_sl_soi_dn12_slot = var_phi_sl_soi_dn12;
        *var_phi_sl_soi_dn17_slot = var_phi_sl_soi_dn17;
        *var_phi_sl_soi_dn2_slot = var_phi_sl_soi_dn2;
        *var_phi_sl_soi_dn6_slot = var_phi_sl_soi_dn6;
        *var_phi_sl_soi_dn7_slot = var_phi_sl_soi_dn7;
        *var_phi_sl_soi_ini_slot = var_phi_sl_soi_ini;
        *var_phi_sl_soi_ini_dn0_slot = var_phi_sl_soi_ini_dn0;
        *var_phi_sl_soi_ini_dn10_slot = var_phi_sl_soi_ini_dn10;
        *var_phi_sl_soi_ini_dn11_slot = var_phi_sl_soi_ini_dn11;
        *var_phi_sl_soi_ini_dn12_slot = var_phi_sl_soi_ini_dn12;
        *var_phi_sl_soi_ini_dn17_slot = var_phi_sl_soi_ini_dn17;
        *var_phi_sl_soi_ini_dn2_slot = var_phi_sl_soi_ini_dn2;
        *var_phi_sl_soi_ini_dn6_slot = var_phi_sl_soi_ini_dn6;
        *var_phi_sl_soi_ini_dn7_slot = var_phi_sl_soi_ini_dn7;
        *var_ps0_inia_slot = var_ps0_inia;
        *var_ps0_inia_dn0_slot = var_ps0_inia_dn0;
        *var_ps0_inia_dn10_slot = var_ps0_inia_dn10;
        *var_ps0_inia_dn11_slot = var_ps0_inia_dn11;
        *var_ps0_inia_dn12_slot = var_ps0_inia_dn12;
        *var_ps0_inia_dn17_slot = var_ps0_inia_dn17;
        *var_ps0_inia_dn2_slot = var_ps0_inia_dn2;
        *var_ps0_inia_dn6_slot = var_ps0_inia_dn6;
        *var_ps0_inia_dn7_slot = var_ps0_inia_dn7;
        *var_psdl_slot = var_psdl;
        *var_psdl_dn0_slot = var_psdl_dn0;
        *var_psdl_dn10_slot = var_psdl_dn10;
        *var_psdl_dn11_slot = var_psdl_dn11;
        *var_psdl_dn12_slot = var_psdl_dn12;
        *var_psdl_dn17_slot = var_psdl_dn17;
        *var_psdl_dn2_slot = var_psdl_dn2;
        *var_psdl_dn6_slot = var_psdl_dn6;
        *var_psdl_dn7_slot = var_psdl_dn7;
        *var_q_b0_dep_slot = var_q_b0_dep;
        *var_q_b0_dep_dn0_slot = var_q_b0_dep_dn0;
        *var_q_b0_dep_dn10_slot = var_q_b0_dep_dn10;
        *var_q_b0_dep_dn11_slot = var_q_b0_dep_dn11;
        *var_q_b0_dep_dn12_slot = var_q_b0_dep_dn12;
        *var_q_b0_dep_dn17_slot = var_q_b0_dep_dn17;
        *var_q_b0_dep_dn2_slot = var_q_b0_dep_dn2;
        *var_q_b0_dep_dn6_slot = var_q_b0_dep_dn6;
        *var_q_b0_dep_dn7_slot = var_q_b0_dep_dn7;
        *var_q_bl_dep_slot = var_q_bl_dep;
        *var_q_bl_dep_dn0_slot = var_q_bl_dep_dn0;
        *var_q_bl_dep_dn10_slot = var_q_bl_dep_dn10;
        *var_q_bl_dep_dn11_slot = var_q_bl_dep_dn11;
        *var_q_bl_dep_dn12_slot = var_q_bl_dep_dn12;
        *var_q_bl_dep_dn17_slot = var_q_bl_dep_dn17;
        *var_q_bl_dep_dn2_slot = var_q_bl_dep_dn2;
        *var_q_bl_dep_dn6_slot = var_q_bl_dep_dn6;
        *var_q_bl_dep_dn7_slot = var_q_bl_dep_dn7;
        *var_q_dep0_slot = var_q_dep0;
        *var_q_dep0_dn0_slot = var_q_dep0_dn0;
        *var_q_dep0_dn10_slot = var_q_dep0_dn10;
        *var_q_dep0_dn11_slot = var_q_dep0_dn11;
        *var_q_dep0_dn12_slot = var_q_dep0_dn12;
        *var_q_dep0_dn17_slot = var_q_dep0_dn17;
        *var_q_dep0_dn2_slot = var_q_dep0_dn2;
        *var_q_dep0_dn6_slot = var_q_dep0_dn6;
        *var_q_dep0_dn7_slot = var_q_dep0_dn7;
        *var_q_dep_soi_slot = var_q_dep_soi;
        *var_q_dep_soi_dn0_slot = var_q_dep_soi_dn0;
        *var_q_dep_soi_dn10_slot = var_q_dep_soi_dn10;
        *var_q_dep_soi_dn11_slot = var_q_dep_soi_dn11;
        *var_q_dep_soi_dn12_slot = var_q_dep_soi_dn12;
        *var_q_dep_soi_dn17_slot = var_q_dep_soi_dn17;
        *var_q_dep_soi_dn2_slot = var_q_dep_soi_dn2;
        *var_q_dep_soi_dn6_slot = var_q_dep_soi_dn6;
        *var_q_dep_soi_dn7_slot = var_q_dep_soi_dn7;
        *var_q_depl_slot = var_q_depl;
        *var_q_depl_dn0_slot = var_q_depl_dn0;
        *var_q_depl_dn10_slot = var_q_depl_dn10;
        *var_q_depl_dn11_slot = var_q_depl_dn11;
        *var_q_depl_dn12_slot = var_q_depl_dn12;
        *var_q_depl_dn17_slot = var_q_depl_dn17;
        *var_q_depl_dn2_slot = var_q_depl_dn2;
        *var_q_depl_dn6_slot = var_q_depl_dn6;
        *var_q_depl_dn7_slot = var_q_depl_dn7;
        *var_q_n0_slot = var_q_n0;
        *var_q_n0_dn0_slot = var_q_n0_dn0;
        *var_q_n0_dn10_slot = var_q_n0_dn10;
        *var_q_n0_dn11_slot = var_q_n0_dn11;
        *var_q_n0_dn12_slot = var_q_n0_dn12;
        *var_q_n0_dn17_slot = var_q_n0_dn17;
        *var_q_n0_dn2_slot = var_q_n0_dn2;
        *var_q_n0_dn6_slot = var_q_n0_dn6;
        *var_q_n0_dn7_slot = var_q_n0_dn7;
        *var_q_nl_slot = var_q_nl;
        *var_q_nl_dn0_slot = var_q_nl_dn0;
        *var_q_nl_dn10_slot = var_q_nl_dn10;
        *var_q_nl_dn11_slot = var_q_nl_dn11;
        *var_q_nl_dn12_slot = var_q_nl_dn12;
        *var_q_nl_dn17_slot = var_q_nl_dn17;
        *var_q_nl_dn2_slot = var_q_nl_dn2;
        *var_q_nl_dn6_slot = var_q_nl_dn6;
        *var_q_nl_dn7_slot = var_q_nl_dn7;
        *var_q_s0_bulk_slot = var_q_s0_bulk;
        *var_q_s0_bulk_0_slot = var_q_s0_bulk_0;
        *var_q_s0_bulk_dn0_slot = var_q_s0_bulk_dn0;
        *var_q_s0_bulk_dn10_slot = var_q_s0_bulk_dn10;
        *var_q_s0_bulk_dn11_slot = var_q_s0_bulk_dn11;
        *var_q_s0_bulk_dn12_slot = var_q_s0_bulk_dn12;
        *var_q_s0_bulk_dn17_slot = var_q_s0_bulk_dn17;
        *var_q_s0_bulk_dn2_slot = var_q_s0_bulk_dn2;
        *var_q_s0_bulk_dn6_slot = var_q_s0_bulk_dn6;
        *var_q_s0_bulk_dn7_slot = var_q_s0_bulk_dn7;
        *var_q_sl_bulk_slot = var_q_sl_bulk;
        *var_q_sl_bulk_dn0_slot = var_q_sl_bulk_dn0;
        *var_q_sl_bulk_dn10_slot = var_q_sl_bulk_dn10;
        *var_q_sl_bulk_dn11_slot = var_q_sl_bulk_dn11;
        *var_q_sl_bulk_dn12_slot = var_q_sl_bulk_dn12;
        *var_q_sl_bulk_dn17_slot = var_q_sl_bulk_dn17;
        *var_q_sl_bulk_dn2_slot = var_q_sl_bulk_dn2;
        *var_q_sl_bulk_dn6_slot = var_q_sl_bulk_dn6;
        *var_q_sl_bulk_dn7_slot = var_q_sl_bulk_dn7;
        *var_qbu_slot = var_qbu;
        *var_qbu_dn0_slot = var_qbu_dn0;
        *var_qbu_dn10_slot = var_qbu_dn10;
        *var_qbu_dn11_slot = var_qbu_dn11;
        *var_qbu_dn12_slot = var_qbu_dn12;
        *var_qbu_dn17_slot = var_qbu_dn17;
        *var_qbu_dn2_slot = var_qbu_dn2;
        *var_qbu_dn6_slot = var_qbu_dn6;
        *var_qbu_dn7_slot = var_qbu_dn7;
        *var_qd_fb_slot = var_qd_fb;
        *var_qd_fb_dn0_slot = var_qd_fb_dn0;
        *var_qd_fb_dn10_slot = var_qd_fb_dn10;
        *var_qd_fb_dn11_slot = var_qd_fb_dn11;
        *var_qd_fb_dn12_slot = var_qd_fb_dn12;
        *var_qd_fb_dn13_slot = var_qd_fb_dn13;
        *var_qd_fb_dn15_slot = var_qd_fb_dn15;
        *var_qd_fb_dn16_slot = var_qd_fb_dn16;
        *var_qd_fb_dn17_slot = var_qd_fb_dn17;
        *var_qd_fb_dn18_slot = var_qd_fb_dn18;
        *var_qd_fb_dn2_slot = var_qd_fb_dn2;
        *var_qd_fb_dn6_slot = var_qd_fb_dn6;
        *var_qd_fb_dn7_slot = var_qd_fb_dn7;
        *var_qd_nqs_slot = var_qd_nqs;
        *var_qd_nqs_dn0_slot = var_qd_nqs_dn0;
        *var_qd_nqs_dn10_slot = var_qd_nqs_dn10;
        *var_qd_nqs_dn11_slot = var_qd_nqs_dn11;
        *var_qd_nqs_dn12_slot = var_qd_nqs_dn12;
        *var_qd_nqs_dn15_slot = var_qd_nqs_dn15;
        *var_qd_nqs_dn17_slot = var_qd_nqs_dn17;
        *var_qd_nqs_dn18_slot = var_qd_nqs_dn18;
        *var_qd_nqs_dn2_slot = var_qd_nqs_dn2;
        *var_qd_nqs_dn6_slot = var_qd_nqs_dn6;
        *var_qd_nqs_dn7_slot = var_qd_nqs_dn7;
        *var_qd_qs_slot = var_qd_qs;
        *var_qd_qs_dn0_slot = var_qd_qs_dn0;
        *var_qd_qs_dn10_slot = var_qd_qs_dn10;
        *var_qd_qs_dn11_slot = var_qd_qs_dn11;
        *var_qd_qs_dn12_slot = var_qd_qs_dn12;
        *var_qd_qs_dn13_slot = var_qd_qs_dn13;
        *var_qd_qs_dn15_slot = var_qd_qs_dn15;
        *var_qd_qs_dn16_slot = var_qd_qs_dn16;
        *var_qd_qs_dn17_slot = var_qd_qs_dn17;
        *var_qd_qs_dn18_slot = var_qd_qs_dn18;
        *var_qd_qs_dn2_slot = var_qd_qs_dn2;
        *var_qd_qs_dn6_slot = var_qd_qs_dn6;
        *var_qd_qs_dn7_slot = var_qd_qs_dn7;
        *var_qdrat_slot = var_qdrat;
        *var_qdrat_dn0_slot = var_qdrat_dn0;
        *var_qdrat_dn10_slot = var_qdrat_dn10;
        *var_qdrat_dn11_slot = var_qdrat_dn11;
        *var_qdrat_dn12_slot = var_qdrat_dn12;
        *var_qdrat_dn17_slot = var_qdrat_dn17;
        *var_qdrat_dn2_slot = var_qdrat_dn2;
        *var_qdrat_dn6_slot = var_qdrat_dn6;
        *var_qdrat_dn7_slot = var_qdrat_dn7;
        *var_qdrat_noi_slot = var_qdrat_noi;
        *var_qdrat_noi_dn0_slot = var_qdrat_noi_dn0;
        *var_qdrat_noi_dn10_slot = var_qdrat_noi_dn10;
        *var_qdrat_noi_dn11_slot = var_qdrat_noi_dn11;
        *var_qdrat_noi_dn12_slot = var_qdrat_noi_dn12;
        *var_qdrat_noi_dn17_slot = var_qdrat_noi_dn17;
        *var_qdrat_noi_dn2_slot = var_qdrat_noi_dn2;
        *var_qdrat_noi_dn6_slot = var_qdrat_noi_dn6;
        *var_qdrat_noi_dn7_slot = var_qdrat_noi_dn7;
        *var_qhs_slot = var_qhs;
        *var_qhs_dn0_slot = var_qhs_dn0;
        *var_qhs_dn10_slot = var_qhs_dn10;
        *var_qhs_dn11_slot = var_qhs_dn11;
        *var_qhs_dn12_slot = var_qhs_dn12;
        *var_qhs_dn17_slot = var_qhs_dn17;
        *var_qhs_dn2_slot = var_qhs_dn2;
        *var_qhs_dn6_slot = var_qhs_dn6;
        *var_qhs_dn7_slot = var_qhs_dn7;
        *var_qi_nqs_slot = var_qi_nqs;
        *var_qi_nqs_dn18_slot = var_qi_nqs_dn18;
        *var_qiu_slot = var_qiu;
        *var_qiu_dn0_slot = var_qiu_dn0;
        *var_qiu_dn10_slot = var_qiu_dn10;
        *var_qiu_dn11_slot = var_qiu_dn11;
        *var_qiu_dn12_slot = var_qiu_dn12;
        *var_qiu_dn17_slot = var_qiu_dn17;
        *var_qiu_dn2_slot = var_qiu_dn2;
        *var_qiu_dn6_slot = var_qiu_dn6;
        *var_qiu_dn7_slot = var_qiu_dn7;
        *var_qs_fb_slot = var_qs_fb;
        *var_qs_fb_dn0_slot = var_qs_fb_dn0;
        *var_qs_fb_dn10_slot = var_qs_fb_dn10;
        *var_qs_fb_dn11_slot = var_qs_fb_dn11;
        *var_qs_fb_dn12_slot = var_qs_fb_dn12;
        *var_qs_fb_dn13_slot = var_qs_fb_dn13;
        *var_qs_fb_dn15_slot = var_qs_fb_dn15;
        *var_qs_fb_dn16_slot = var_qs_fb_dn16;
        *var_qs_fb_dn17_slot = var_qs_fb_dn17;
        *var_qs_fb_dn18_slot = var_qs_fb_dn18;
        *var_qs_fb_dn2_slot = var_qs_fb_dn2;
        *var_qs_fb_dn6_slot = var_qs_fb_dn6;
        *var_qs_fb_dn7_slot = var_qs_fb_dn7;
        *var_qs_nqs_slot = var_qs_nqs;
        *var_qs_nqs_dn0_slot = var_qs_nqs_dn0;
        *var_qs_nqs_dn10_slot = var_qs_nqs_dn10;
        *var_qs_nqs_dn11_slot = var_qs_nqs_dn11;
        *var_qs_nqs_dn12_slot = var_qs_nqs_dn12;
        *var_qs_nqs_dn16_slot = var_qs_nqs_dn16;
        *var_qs_nqs_dn17_slot = var_qs_nqs_dn17;
        *var_qs_nqs_dn18_slot = var_qs_nqs_dn18;
        *var_qs_nqs_dn2_slot = var_qs_nqs_dn2;
        *var_qs_nqs_dn6_slot = var_qs_nqs_dn6;
        *var_qs_nqs_dn7_slot = var_qs_nqs_dn7;
        *var_qsub_slot = var_qsub;
        *var_qsub_dn0_slot = var_qsub_dn0;
        *var_qsub_dn10_slot = var_qsub_dn10;
        *var_qsub_dn11_slot = var_qsub_dn11;
        *var_qsub_dn12_slot = var_qsub_dn12;
        *var_qsub_dn17_slot = var_qsub_dn17;
        *var_qsub_dn2_slot = var_qsub_dn2;
        *var_qsub_dn6_slot = var_qsub_dn6;
        *var_qsub_dn7_slot = var_qsub_dn7;
        *var_shift_slot = var_shift;
        *var_shift_dn0_slot = var_shift_dn0;
        *var_shift_dn10_slot = var_shift_dn10;
        *var_shift_dn11_slot = var_shift_dn11;
        *var_shift_dn12_slot = var_shift_dn12;
        *var_shift_dn17_slot = var_shift_dn17;
        *var_shift_dn2_slot = var_shift_dn2;
        *var_shift_dn6_slot = var_shift_dn6;
        *var_shift_dn7_slot = var_shift_dn7;
        *var_sqrtkusail_slot = var_sqrtkusail;
        *var_sqrtkusail_dn0_slot = var_sqrtkusail_dn0;
        *var_sqrtkusail_dn10_slot = var_sqrtkusail_dn10;
        *var_sqrtkusail_dn11_slot = var_sqrtkusail_dn11;
        *var_sqrtkusail_dn12_slot = var_sqrtkusail_dn12;
        *var_sqrtkusail_dn17_slot = var_sqrtkusail_dn17;
        *var_sqrtkusail_dn2_slot = var_sqrtkusail_dn2;
        *var_sqrtkusail_dn6_slot = var_sqrtkusail_dn6;
        *var_sqrtkusail_dn7_slot = var_sqrtkusail_dn7;
        *var_wdsoi_slot = var_wdsoi;
        *var_wdsoi_dn0_slot = var_wdsoi_dn0;
        *var_wdsoi_dn10_slot = var_wdsoi_dn10;
        *var_wdsoi_dn11_slot = var_wdsoi_dn11;
        *var_wdsoi_dn12_slot = var_wdsoi_dn12;
        *var_wdsoi_dn17_slot = var_wdsoi_dn17;
        *var_wdsoi_dn2_slot = var_wdsoi_dn2;
        *var_wdsoi_dn6_slot = var_wdsoi_dn6;
        *var_wdsoi_dn7_slot = var_wdsoi_dn7;
    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_arg_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn10_slot: &mut f64,
        var_arg_dn11_slot: &mut f64,
        var_arg_dn12_slot: &mut f64,
        var_arg_dn17_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn11_slot: &mut f64,
        var_dnm_dn12_slot: &mut f64,
        var_dnm_dn17_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_flg_info_slot: &mut f64,
        var_flg_nqs_slot: &mut f64,
        var_flg_skipacc_slot: &mut f64,
        var_fs01_slot: &mut f64,
        var_fs01_dn0_slot: &mut f64,
        var_fs01_dn10_slot: &mut f64,
        var_fs01_dn11_slot: &mut f64,
        var_fs01_dn12_slot: &mut f64,
        var_fs01_dn17_slot: &mut f64,
        var_fs01_dn2_slot: &mut f64,
        var_fs01_dn6_slot: &mut f64,
        var_fs01_dn7_slot: &mut f64,
        var_fs02_slot: &mut f64,
        var_fs02_dn0_slot: &mut f64,
        var_fs02_dn10_slot: &mut f64,
        var_fs02_dn11_slot: &mut f64,
        var_fs02_dn12_slot: &mut f64,
        var_fs02_dn17_slot: &mut f64,
        var_fs02_dn2_slot: &mut f64,
        var_fs02_dn6_slot: &mut f64,
        var_fs02_dn7_slot: &mut f64,
        var_fsl1_slot: &mut f64,
        var_fsl1_dn0_slot: &mut f64,
        var_fsl1_dn10_slot: &mut f64,
        var_fsl1_dn11_slot: &mut f64,
        var_fsl1_dn12_slot: &mut f64,
        var_fsl1_dn17_slot: &mut f64,
        var_fsl1_dn2_slot: &mut f64,
        var_fsl1_dn6_slot: &mut f64,
        var_fsl1_dn7_slot: &mut f64,
        var_fsl2_slot: &mut f64,
        var_fsl2_dn0_slot: &mut f64,
        var_fsl2_dn10_slot: &mut f64,
        var_fsl2_dn11_slot: &mut f64,
        var_fsl2_dn12_slot: &mut f64,
        var_fsl2_dn17_slot: &mut f64,
        var_fsl2_dn2_slot: &mut f64,
        var_fsl2_dn6_slot: &mut f64,
        var_fsl2_dn7_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_guard5_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_idspt0_slot: &mut f64,
        var_idspt0_dn0_slot: &mut f64,
        var_idspt0_dn10_slot: &mut f64,
        var_idspt0_dn11_slot: &mut f64,
        var_idspt0_dn12_slot: &mut f64,
        var_idspt0_dn17_slot: &mut f64,
        var_idspt0_dn2_slot: &mut f64,
        var_idspt0_dn6_slot: &mut f64,
        var_idspt0_dn7_slot: &mut f64,
        var_lp_s0_max_slot: &mut f64,
        var_lp_sl_max_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_mks_cth0_slot: &mut f64,
        var_mks_njunc_slot: &mut f64,
        var_mks_nover_slot: &mut f64,
        var_mks_nsti_slot: &mut f64,
        var_mks_nsubb_slot: &mut f64,
        var_mks_nsubcmax_slot: &mut f64,
        var_mks_nsubp_slot: &mut f64,
        var_mks_nsubs_slot: &mut f64,
        var_mks_parl1_slot: &mut f64,
        var_mks_rth0_slot: &mut f64,
        var_mks_vmax_slot: &mut f64,
        var_mks_vtmp_slot: &mut f64,
        var_mks_wfc_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_ps0_slot: &mut f64,
        var_ps0_dn0_slot: &mut f64,
        var_ps0_dn10_slot: &mut f64,
        var_ps0_dn11_slot: &mut f64,
        var_ps0_dn12_slot: &mut f64,
        var_ps0_dn17_slot: &mut f64,
        var_ps0_dn2_slot: &mut f64,
        var_ps0_dn6_slot: &mut f64,
        var_ps0_dn7_slot: &mut f64,
        var_ps0_ini_slot: &mut f64,
        var_ps0_ini_dn0_slot: &mut f64,
        var_ps0_ini_dn10_slot: &mut f64,
        var_ps0_ini_dn11_slot: &mut f64,
        var_ps0_ini_dn12_slot: &mut f64,
        var_ps0_ini_dn17_slot: &mut f64,
        var_ps0_ini_dn2_slot: &mut f64,
        var_ps0_ini_dn6_slot: &mut f64,
        var_ps0_ini_dn7_slot: &mut f64,
        var_q_s0_dep_ini_slot: &mut f64,
        var_q_s0_dep_ini_dn0_slot: &mut f64,
        var_q_s0_dep_ini_dn10_slot: &mut f64,
        var_q_s0_dep_ini_dn11_slot: &mut f64,
        var_q_s0_dep_ini_dn12_slot: &mut f64,
        var_q_s0_dep_ini_dn17_slot: &mut f64,
        var_q_s0_dep_ini_dn2_slot: &mut f64,
        var_q_s0_dep_ini_dn6_slot: &mut f64,
        var_q_s0_dep_ini_dn7_slot: &mut f64,
        var_qb_qs_slot: &mut f64,
        var_qb_qs_dn0_slot: &mut f64,
        var_qb_qs_dn10_slot: &mut f64,
        var_qb_qs_dn11_slot: &mut f64,
        var_qb_qs_dn12_slot: &mut f64,
        var_qb_qs_dn13_slot: &mut f64,
        var_qb_qs_dn15_slot: &mut f64,
        var_qb_qs_dn16_slot: &mut f64,
        var_qb_qs_dn17_slot: &mut f64,
        var_qb_qs_dn18_slot: &mut f64,
        var_qb_qs_dn2_slot: &mut f64,
        var_qb_qs_dn6_slot: &mut f64,
        var_qb_qs_dn7_slot: &mut f64,
        var_qi_qs_slot: &mut f64,
        var_qi_qs_dn0_slot: &mut f64,
        var_qi_qs_dn10_slot: &mut f64,
        var_qi_qs_dn11_slot: &mut f64,
        var_qi_qs_dn12_slot: &mut f64,
        var_qi_qs_dn17_slot: &mut f64,
        var_qi_qs_dn2_slot: &mut f64,
        var_qi_qs_dn6_slot: &mut f64,
        var_qi_qs_dn7_slot: &mut f64,
        var_qs_qs_slot: &mut f64,
        var_qs_qs_dn0_slot: &mut f64,
        var_qs_qs_dn10_slot: &mut f64,
        var_qs_qs_dn11_slot: &mut f64,
        var_qs_qs_dn12_slot: &mut f64,
        var_qs_qs_dn13_slot: &mut f64,
        var_qs_qs_dn15_slot: &mut f64,
        var_qs_qs_dn16_slot: &mut f64,
        var_qs_qs_dn17_slot: &mut f64,
        var_qs_qs_dn18_slot: &mut f64,
        var_qs_qs_dn2_slot: &mut f64,
        var_qs_qs_dn6_slot: &mut f64,
        var_qs_qs_dn7_slot: &mut f64,
        var_subversion_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_uc_clm2_slot: &mut f64,
        var_uc_clm2_dn0_slot: &mut f64,
        var_uc_clm2_dn10_slot: &mut f64,
        var_uc_clm2_dn11_slot: &mut f64,
        var_uc_clm2_dn12_slot: &mut f64,
        var_uc_clm2_dn17_slot: &mut f64,
        var_uc_clm2_dn2_slot: &mut f64,
        var_uc_clm2_dn6_slot: &mut f64,
        var_uc_clm2_dn7_slot: &mut f64,
        var_uc_gdld_slot: &mut f64,
        var_uc_sc2_slot: &mut f64,
        var_uc_sc3_slot: &mut f64,
        var_uc_scp2_slot: &mut f64,
        var_uc_scp3_slot: &mut f64,
        var_uc_tnom_slot: &mut f64,
        var_uc_vfbover_slot: &mut f64,
        var_vbcs_cl_slot: &mut f64,
        var_vbcs_cl_dn0_slot: &mut f64,
        var_vbcs_cl_dn10_slot: &mut f64,
        var_vbcs_cl_dn11_slot: &mut f64,
        var_vbcs_cl_dn12_slot: &mut f64,
        var_vbcs_cl_dn17_slot: &mut f64,
        var_vbcs_cl_dn2_slot: &mut f64,
        var_vbcs_cl_dn6_slot: &mut f64,
        var_vbcs_cl_dn7_slot: &mut f64,
        var_vbsbiz_slot: &mut f64,
        var_vbsbiz_dn0_slot: &mut f64,
        var_vbsbiz_dn10_slot: &mut f64,
        var_vbsbiz_dn11_slot: &mut f64,
        var_vbsbiz_dn12_slot: &mut f64,
        var_vbsbiz_dn17_slot: &mut f64,
        var_vbsbiz_dn2_slot: &mut f64,
        var_vbsbiz_dn6_slot: &mut f64,
        var_vbsbiz_dn7_slot: &mut f64,
        var_x2_slot: &mut f64,
        var_x2_dn0_slot: &mut f64,
        var_x2_dn10_slot: &mut f64,
        var_x2_dn11_slot: &mut f64,
        var_x2_dn12_slot: &mut f64,
        var_x2_dn17_slot: &mut f64,
        var_x2_dn2_slot: &mut f64,
        var_x2_dn6_slot: &mut f64,
        var_x2_dn7_slot: &mut f64,
        var_xmax2_slot: &mut f64,
        var_xmax2_dn0_slot: &mut f64,
        var_xmax2_dn10_slot: &mut f64,
        var_xmax2_dn11_slot: &mut f64,
        var_xmax2_dn12_slot: &mut f64,
        var_xmax2_dn17_slot: &mut f64,
        var_xmax2_dn2_slot: &mut f64,
        var_xmax2_dn6_slot: &mut f64,
        var_xmax2_dn7_slot: &mut f64,
        var_xmp_slot: &mut f64,
        var_xmp_dn0_slot: &mut f64,
        var_xmp_dn10_slot: &mut f64,
        var_xmp_dn11_slot: &mut f64,
        var_xmp_dn12_slot: &mut f64,
        var_xmp_dn17_slot: &mut f64,
        var_xmp_dn2_slot: &mut f64,
        var_xmp_dn6_slot: &mut f64,
        var_xmp_dn7_slot: &mut f64,
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn11_slot: &mut f64,
        var_xp_dn12_slot: &mut f64,
        var_xp_dn17_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn7_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn10: f64 = *var_arg_dn10_slot;
        let mut var_arg_dn11: f64 = *var_arg_dn11_slot;
        let mut var_arg_dn12: f64 = *var_arg_dn12_slot;
        let mut var_arg_dn17: f64 = *var_arg_dn17_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn11: f64 = *var_dnm_dn11_slot;
        let mut var_dnm_dn12: f64 = *var_dnm_dn12_slot;
        let mut var_dnm_dn17: f64 = *var_dnm_dn17_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_flg_info: f64 = *var_flg_info_slot;
        let mut var_flg_nqs: f64 = *var_flg_nqs_slot;
        let mut var_flg_skipacc: f64 = *var_flg_skipacc_slot;
        let mut var_fs01: f64 = *var_fs01_slot;
        let mut var_fs01_dn0: f64 = *var_fs01_dn0_slot;
        let mut var_fs01_dn10: f64 = *var_fs01_dn10_slot;
        let mut var_fs01_dn11: f64 = *var_fs01_dn11_slot;
        let mut var_fs01_dn12: f64 = *var_fs01_dn12_slot;
        let mut var_fs01_dn17: f64 = *var_fs01_dn17_slot;
        let mut var_fs01_dn2: f64 = *var_fs01_dn2_slot;
        let mut var_fs01_dn6: f64 = *var_fs01_dn6_slot;
        let mut var_fs01_dn7: f64 = *var_fs01_dn7_slot;
        let mut var_fs02: f64 = *var_fs02_slot;
        let mut var_fs02_dn0: f64 = *var_fs02_dn0_slot;
        let mut var_fs02_dn10: f64 = *var_fs02_dn10_slot;
        let mut var_fs02_dn11: f64 = *var_fs02_dn11_slot;
        let mut var_fs02_dn12: f64 = *var_fs02_dn12_slot;
        let mut var_fs02_dn17: f64 = *var_fs02_dn17_slot;
        let mut var_fs02_dn2: f64 = *var_fs02_dn2_slot;
        let mut var_fs02_dn6: f64 = *var_fs02_dn6_slot;
        let mut var_fs02_dn7: f64 = *var_fs02_dn7_slot;
        let mut var_fsl1: f64 = *var_fsl1_slot;
        let mut var_fsl1_dn0: f64 = *var_fsl1_dn0_slot;
        let mut var_fsl1_dn10: f64 = *var_fsl1_dn10_slot;
        let mut var_fsl1_dn11: f64 = *var_fsl1_dn11_slot;
        let mut var_fsl1_dn12: f64 = *var_fsl1_dn12_slot;
        let mut var_fsl1_dn17: f64 = *var_fsl1_dn17_slot;
        let mut var_fsl1_dn2: f64 = *var_fsl1_dn2_slot;
        let mut var_fsl1_dn6: f64 = *var_fsl1_dn6_slot;
        let mut var_fsl1_dn7: f64 = *var_fsl1_dn7_slot;
        let mut var_fsl2: f64 = *var_fsl2_slot;
        let mut var_fsl2_dn0: f64 = *var_fsl2_dn0_slot;
        let mut var_fsl2_dn10: f64 = *var_fsl2_dn10_slot;
        let mut var_fsl2_dn11: f64 = *var_fsl2_dn11_slot;
        let mut var_fsl2_dn12: f64 = *var_fsl2_dn12_slot;
        let mut var_fsl2_dn17: f64 = *var_fsl2_dn17_slot;
        let mut var_fsl2_dn2: f64 = *var_fsl2_dn2_slot;
        let mut var_fsl2_dn6: f64 = *var_fsl2_dn6_slot;
        let mut var_fsl2_dn7: f64 = *var_fsl2_dn7_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_idspt0: f64 = *var_idspt0_slot;
        let mut var_idspt0_dn0: f64 = *var_idspt0_dn0_slot;
        let mut var_idspt0_dn10: f64 = *var_idspt0_dn10_slot;
        let mut var_idspt0_dn11: f64 = *var_idspt0_dn11_slot;
        let mut var_idspt0_dn12: f64 = *var_idspt0_dn12_slot;
        let mut var_idspt0_dn17: f64 = *var_idspt0_dn17_slot;
        let mut var_idspt0_dn2: f64 = *var_idspt0_dn2_slot;
        let mut var_idspt0_dn6: f64 = *var_idspt0_dn6_slot;
        let mut var_idspt0_dn7: f64 = *var_idspt0_dn7_slot;
        let mut var_lp_s0_max: f64 = *var_lp_s0_max_slot;
        let mut var_lp_sl_max: f64 = *var_lp_sl_max_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_mks_cth0: f64 = *var_mks_cth0_slot;
        let mut var_mks_njunc: f64 = *var_mks_njunc_slot;
        let mut var_mks_nover: f64 = *var_mks_nover_slot;
        let mut var_mks_nsti: f64 = *var_mks_nsti_slot;
        let mut var_mks_nsubb: f64 = *var_mks_nsubb_slot;
        let mut var_mks_nsubcmax: f64 = *var_mks_nsubcmax_slot;
        let mut var_mks_nsubp: f64 = *var_mks_nsubp_slot;
        let mut var_mks_nsubs: f64 = *var_mks_nsubs_slot;
        let mut var_mks_parl1: f64 = *var_mks_parl1_slot;
        let mut var_mks_rth0: f64 = *var_mks_rth0_slot;
        let mut var_mks_vmax: f64 = *var_mks_vmax_slot;
        let mut var_mks_vtmp: f64 = *var_mks_vtmp_slot;
        let mut var_mks_wfc: f64 = *var_mks_wfc_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_ps0: f64 = *var_ps0_slot;
        let mut var_ps0_dn0: f64 = *var_ps0_dn0_slot;
        let mut var_ps0_dn10: f64 = *var_ps0_dn10_slot;
        let mut var_ps0_dn11: f64 = *var_ps0_dn11_slot;
        let mut var_ps0_dn12: f64 = *var_ps0_dn12_slot;
        let mut var_ps0_dn17: f64 = *var_ps0_dn17_slot;
        let mut var_ps0_dn2: f64 = *var_ps0_dn2_slot;
        let mut var_ps0_dn6: f64 = *var_ps0_dn6_slot;
        let mut var_ps0_dn7: f64 = *var_ps0_dn7_slot;
        let mut var_ps0_ini: f64 = *var_ps0_ini_slot;
        let mut var_ps0_ini_dn0: f64 = *var_ps0_ini_dn0_slot;
        let mut var_ps0_ini_dn10: f64 = *var_ps0_ini_dn10_slot;
        let mut var_ps0_ini_dn11: f64 = *var_ps0_ini_dn11_slot;
        let mut var_ps0_ini_dn12: f64 = *var_ps0_ini_dn12_slot;
        let mut var_ps0_ini_dn17: f64 = *var_ps0_ini_dn17_slot;
        let mut var_ps0_ini_dn2: f64 = *var_ps0_ini_dn2_slot;
        let mut var_ps0_ini_dn6: f64 = *var_ps0_ini_dn6_slot;
        let mut var_ps0_ini_dn7: f64 = *var_ps0_ini_dn7_slot;
        let mut var_q_s0_dep_ini: f64 = *var_q_s0_dep_ini_slot;
        let mut var_q_s0_dep_ini_dn0: f64 = *var_q_s0_dep_ini_dn0_slot;
        let mut var_q_s0_dep_ini_dn10: f64 = *var_q_s0_dep_ini_dn10_slot;
        let mut var_q_s0_dep_ini_dn11: f64 = *var_q_s0_dep_ini_dn11_slot;
        let mut var_q_s0_dep_ini_dn12: f64 = *var_q_s0_dep_ini_dn12_slot;
        let mut var_q_s0_dep_ini_dn17: f64 = *var_q_s0_dep_ini_dn17_slot;
        let mut var_q_s0_dep_ini_dn2: f64 = *var_q_s0_dep_ini_dn2_slot;
        let mut var_q_s0_dep_ini_dn6: f64 = *var_q_s0_dep_ini_dn6_slot;
        let mut var_q_s0_dep_ini_dn7: f64 = *var_q_s0_dep_ini_dn7_slot;
        let mut var_qb_qs: f64 = *var_qb_qs_slot;
        let mut var_qb_qs_dn0: f64 = *var_qb_qs_dn0_slot;
        let mut var_qb_qs_dn10: f64 = *var_qb_qs_dn10_slot;
        let mut var_qb_qs_dn11: f64 = *var_qb_qs_dn11_slot;
        let mut var_qb_qs_dn12: f64 = *var_qb_qs_dn12_slot;
        let mut var_qb_qs_dn13: f64 = *var_qb_qs_dn13_slot;
        let mut var_qb_qs_dn15: f64 = *var_qb_qs_dn15_slot;
        let mut var_qb_qs_dn16: f64 = *var_qb_qs_dn16_slot;
        let mut var_qb_qs_dn17: f64 = *var_qb_qs_dn17_slot;
        let mut var_qb_qs_dn18: f64 = *var_qb_qs_dn18_slot;
        let mut var_qb_qs_dn2: f64 = *var_qb_qs_dn2_slot;
        let mut var_qb_qs_dn6: f64 = *var_qb_qs_dn6_slot;
        let mut var_qb_qs_dn7: f64 = *var_qb_qs_dn7_slot;
        let mut var_qi_qs: f64 = *var_qi_qs_slot;
        let mut var_qi_qs_dn0: f64 = *var_qi_qs_dn0_slot;
        let mut var_qi_qs_dn10: f64 = *var_qi_qs_dn10_slot;
        let mut var_qi_qs_dn11: f64 = *var_qi_qs_dn11_slot;
        let mut var_qi_qs_dn12: f64 = *var_qi_qs_dn12_slot;
        let mut var_qi_qs_dn17: f64 = *var_qi_qs_dn17_slot;
        let mut var_qi_qs_dn2: f64 = *var_qi_qs_dn2_slot;
        let mut var_qi_qs_dn6: f64 = *var_qi_qs_dn6_slot;
        let mut var_qi_qs_dn7: f64 = *var_qi_qs_dn7_slot;
        let mut var_qs_qs: f64 = *var_qs_qs_slot;
        let mut var_qs_qs_dn0: f64 = *var_qs_qs_dn0_slot;
        let mut var_qs_qs_dn10: f64 = *var_qs_qs_dn10_slot;
        let mut var_qs_qs_dn11: f64 = *var_qs_qs_dn11_slot;
        let mut var_qs_qs_dn12: f64 = *var_qs_qs_dn12_slot;
        let mut var_qs_qs_dn13: f64 = *var_qs_qs_dn13_slot;
        let mut var_qs_qs_dn15: f64 = *var_qs_qs_dn15_slot;
        let mut var_qs_qs_dn16: f64 = *var_qs_qs_dn16_slot;
        let mut var_qs_qs_dn17: f64 = *var_qs_qs_dn17_slot;
        let mut var_qs_qs_dn18: f64 = *var_qs_qs_dn18_slot;
        let mut var_qs_qs_dn2: f64 = *var_qs_qs_dn2_slot;
        let mut var_qs_qs_dn6: f64 = *var_qs_qs_dn6_slot;
        let mut var_qs_qs_dn7: f64 = *var_qs_qs_dn7_slot;
        let mut var_subversion: f64 = *var_subversion_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_uc_clm2: f64 = *var_uc_clm2_slot;
        let mut var_uc_clm2_dn0: f64 = *var_uc_clm2_dn0_slot;
        let mut var_uc_clm2_dn10: f64 = *var_uc_clm2_dn10_slot;
        let mut var_uc_clm2_dn11: f64 = *var_uc_clm2_dn11_slot;
        let mut var_uc_clm2_dn12: f64 = *var_uc_clm2_dn12_slot;
        let mut var_uc_clm2_dn17: f64 = *var_uc_clm2_dn17_slot;
        let mut var_uc_clm2_dn2: f64 = *var_uc_clm2_dn2_slot;
        let mut var_uc_clm2_dn6: f64 = *var_uc_clm2_dn6_slot;
        let mut var_uc_clm2_dn7: f64 = *var_uc_clm2_dn7_slot;
        let mut var_uc_gdld: f64 = *var_uc_gdld_slot;
        let mut var_uc_sc2: f64 = *var_uc_sc2_slot;
        let mut var_uc_sc3: f64 = *var_uc_sc3_slot;
        let mut var_uc_scp2: f64 = *var_uc_scp2_slot;
        let mut var_uc_scp3: f64 = *var_uc_scp3_slot;
        let mut var_uc_tnom: f64 = *var_uc_tnom_slot;
        let mut var_uc_vfbover: f64 = *var_uc_vfbover_slot;
        let mut var_vbcs_cl: f64 = *var_vbcs_cl_slot;
        let mut var_vbcs_cl_dn0: f64 = *var_vbcs_cl_dn0_slot;
        let mut var_vbcs_cl_dn10: f64 = *var_vbcs_cl_dn10_slot;
        let mut var_vbcs_cl_dn11: f64 = *var_vbcs_cl_dn11_slot;
        let mut var_vbcs_cl_dn12: f64 = *var_vbcs_cl_dn12_slot;
        let mut var_vbcs_cl_dn17: f64 = *var_vbcs_cl_dn17_slot;
        let mut var_vbcs_cl_dn2: f64 = *var_vbcs_cl_dn2_slot;
        let mut var_vbcs_cl_dn6: f64 = *var_vbcs_cl_dn6_slot;
        let mut var_vbcs_cl_dn7: f64 = *var_vbcs_cl_dn7_slot;
        let mut var_vbsbiz: f64 = *var_vbsbiz_slot;
        let mut var_vbsbiz_dn0: f64 = *var_vbsbiz_dn0_slot;
        let mut var_vbsbiz_dn10: f64 = *var_vbsbiz_dn10_slot;
        let mut var_vbsbiz_dn11: f64 = *var_vbsbiz_dn11_slot;
        let mut var_vbsbiz_dn12: f64 = *var_vbsbiz_dn12_slot;
        let mut var_vbsbiz_dn17: f64 = *var_vbsbiz_dn17_slot;
        let mut var_vbsbiz_dn2: f64 = *var_vbsbiz_dn2_slot;
        let mut var_vbsbiz_dn6: f64 = *var_vbsbiz_dn6_slot;
        let mut var_vbsbiz_dn7: f64 = *var_vbsbiz_dn7_slot;
        let mut var_x2: f64 = *var_x2_slot;
        let mut var_x2_dn0: f64 = *var_x2_dn0_slot;
        let mut var_x2_dn10: f64 = *var_x2_dn10_slot;
        let mut var_x2_dn11: f64 = *var_x2_dn11_slot;
        let mut var_x2_dn12: f64 = *var_x2_dn12_slot;
        let mut var_x2_dn17: f64 = *var_x2_dn17_slot;
        let mut var_x2_dn2: f64 = *var_x2_dn2_slot;
        let mut var_x2_dn6: f64 = *var_x2_dn6_slot;
        let mut var_x2_dn7: f64 = *var_x2_dn7_slot;
        let mut var_xmax2: f64 = *var_xmax2_slot;
        let mut var_xmax2_dn0: f64 = *var_xmax2_dn0_slot;
        let mut var_xmax2_dn10: f64 = *var_xmax2_dn10_slot;
        let mut var_xmax2_dn11: f64 = *var_xmax2_dn11_slot;
        let mut var_xmax2_dn12: f64 = *var_xmax2_dn12_slot;
        let mut var_xmax2_dn17: f64 = *var_xmax2_dn17_slot;
        let mut var_xmax2_dn2: f64 = *var_xmax2_dn2_slot;
        let mut var_xmax2_dn6: f64 = *var_xmax2_dn6_slot;
        let mut var_xmax2_dn7: f64 = *var_xmax2_dn7_slot;
        let mut var_xmp: f64 = *var_xmp_slot;
        let mut var_xmp_dn0: f64 = *var_xmp_dn0_slot;
        let mut var_xmp_dn10: f64 = *var_xmp_dn10_slot;
        let mut var_xmp_dn11: f64 = *var_xmp_dn11_slot;
        let mut var_xmp_dn12: f64 = *var_xmp_dn12_slot;
        let mut var_xmp_dn17: f64 = *var_xmp_dn17_slot;
        let mut var_xmp_dn2: f64 = *var_xmp_dn2_slot;
        let mut var_xmp_dn6: f64 = *var_xmp_dn6_slot;
        let mut var_xmp_dn7: f64 = *var_xmp_dn7_slot;
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn11: f64 = *var_xp_dn11_slot;
        let mut var_xp_dn12: f64 = *var_xp_dn12_slot;
        let mut var_xp_dn17: f64 = *var_xp_dn17_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn7: f64 = *var_xp_dn7_slot;

        var_qs_qs = 0.0;
        var_qs_qs_dn0 = 0.0;
        var_qs_qs_dn2 = 0.0;
        var_qs_qs_dn6 = 0.0;
        var_qs_qs_dn7 = 0.0;
        var_qs_qs_dn10 = 0.0;
        var_qs_qs_dn11 = 0.0;
        var_qs_qs_dn12 = 0.0;
        var_qs_qs_dn13 = 0.0;
        var_qs_qs_dn15 = 0.0;
        var_qs_qs_dn16 = 0.0;
        var_qs_qs_dn17 = 0.0;
        var_qs_qs_dn18 = 0.0;

        var_qi_qs = 0.0;
        var_qi_qs_dn0 = 0.0;
        var_qi_qs_dn2 = 0.0;
        var_qi_qs_dn6 = 0.0;
        var_qi_qs_dn7 = 0.0;
        var_qi_qs_dn10 = 0.0;
        var_qi_qs_dn11 = 0.0;
        var_qi_qs_dn12 = 0.0;
        var_qi_qs_dn17 = 0.0;

        var_qb_qs = 0.0;
        var_qb_qs_dn0 = 0.0;
        var_qb_qs_dn2 = 0.0;
        var_qb_qs_dn6 = 0.0;
        var_qb_qs_dn7 = 0.0;
        var_qb_qs_dn10 = 0.0;
        var_qb_qs_dn11 = 0.0;
        var_qb_qs_dn12 = 0.0;
        var_qb_qs_dn13 = 0.0;
        var_qb_qs_dn15 = 0.0;
        var_qb_qs_dn16 = 0.0;
        var_qb_qs_dn17 = 0.0;
        var_qb_qs_dn18 = 0.0;

        var_fs01 = 0.0;
        var_fs01_dn0 = 0.0;
        var_fs01_dn2 = 0.0;
        var_fs01_dn6 = 0.0;
        var_fs01_dn7 = 0.0;
        var_fs01_dn10 = 0.0;
        var_fs01_dn11 = 0.0;
        var_fs01_dn12 = 0.0;
        var_fs01_dn17 = 0.0;

        var_fs02 = 0.0;
        var_fs02_dn0 = 0.0;
        var_fs02_dn2 = 0.0;
        var_fs02_dn6 = 0.0;
        var_fs02_dn7 = 0.0;
        var_fs02_dn10 = 0.0;
        var_fs02_dn11 = 0.0;
        var_fs02_dn12 = 0.0;
        var_fs02_dn17 = 0.0;

        var_fsl1 = 0.0;
        var_fsl1_dn0 = 0.0;
        var_fsl1_dn2 = 0.0;
        var_fsl1_dn6 = 0.0;
        var_fsl1_dn7 = 0.0;
        var_fsl1_dn10 = 0.0;
        var_fsl1_dn11 = 0.0;
        var_fsl1_dn12 = 0.0;
        var_fsl1_dn17 = 0.0;

        var_fsl2 = 0.0;
        var_fsl2_dn0 = 0.0;
        var_fsl2_dn2 = 0.0;
        var_fsl2_dn6 = 0.0;
        var_fsl2_dn7 = 0.0;
        var_fsl2_dn10 = 0.0;
        var_fsl2_dn11 = 0.0;
        var_fsl2_dn12 = 0.0;
        var_fsl2_dn17 = 0.0;

        let assign1220_e978: f64 = (p.p51 * 10.0);
        let assign1220_e980: f64 = (assign1220_e978 % 10.0);
        var_subversion = assign1220_e980;

        var_lp_s0_max = 200.0;

        var_lp_sl_max = 200.0;

        var_flg_skipacc = 0.0;

        var_vbsbiz = 0.0;
        var_vbsbiz_dn0 = 0.0;
        var_vbsbiz_dn2 = 0.0;
        var_vbsbiz_dn6 = 0.0;
        var_vbsbiz_dn7 = 0.0;
        var_vbsbiz_dn10 = 0.0;
        var_vbsbiz_dn11 = 0.0;
        var_vbsbiz_dn12 = 0.0;
        var_vbsbiz_dn17 = 0.0;

        var_ps0_ini = 0.0;
        var_ps0_ini_dn0 = 0.0;
        var_ps0_ini_dn2 = 0.0;
        var_ps0_ini_dn6 = 0.0;
        var_ps0_ini_dn7 = 0.0;
        var_ps0_ini_dn10 = 0.0;
        var_ps0_ini_dn11 = 0.0;
        var_ps0_ini_dn12 = 0.0;
        var_ps0_ini_dn17 = 0.0;

        var_q_s0_dep_ini = 0.0;
        var_q_s0_dep_ini_dn0 = 0.0;
        var_q_s0_dep_ini_dn2 = 0.0;
        var_q_s0_dep_ini_dn6 = 0.0;
        var_q_s0_dep_ini_dn7 = 0.0;
        var_q_s0_dep_ini_dn10 = 0.0;
        var_q_s0_dep_ini_dn11 = 0.0;
        var_q_s0_dep_ini_dn12 = 0.0;
        var_q_s0_dep_ini_dn17 = 0.0;

        var_idspt0 = 0.0;
        var_idspt0_dn0 = 0.0;
        var_idspt0_dn2 = 0.0;
        var_idspt0_dn6 = 0.0;
        var_idspt0_dn7 = 0.0;
        var_idspt0_dn10 = 0.0;
        var_idspt0_dn11 = 0.0;
        var_idspt0_dn12 = 0.0;
        var_idspt0_dn17 = 0.0;

        var_ps0 = 0.0;
        var_ps0_dn0 = 0.0;
        var_ps0_dn2 = 0.0;
        var_ps0_dn6 = 0.0;
        var_ps0_dn7 = 0.0;
        var_ps0_dn10 = 0.0;
        var_ps0_dn11 = 0.0;
        var_ps0_dn12 = 0.0;
        var_ps0_dn17 = 0.0;

        var_vbcs_cl = 0.0;
        var_vbcs_cl_dn0 = 0.0;
        var_vbcs_cl_dn2 = 0.0;
        var_vbcs_cl_dn6 = 0.0;
        var_vbcs_cl_dn7 = 0.0;
        var_vbcs_cl_dn10 = 0.0;
        var_vbcs_cl_dn11 = 0.0;
        var_vbcs_cl_dn12 = 0.0;
        var_vbcs_cl_dn17 = 0.0;

        let assign1330_e993: f64 = (p.p52 * 0.01);
        var_mks_vmax = assign1330_e993;

        let assign1340_e996: f64 = (p.p73 / 1e-6);
        var_mks_nsubp = assign1340_e996;

        let assign1350_e999: f64 = (p.p104 * 0.01);
        var_mks_vtmp = assign1350_e999;

        let assign1360_e1002: f64 = (p.p201 / 1e-6);
        var_mks_nsubcmax = assign1360_e1002;

        let assign1400_e1014: f64 = (p.p240 / 1e-6);
        var_mks_nsubs = assign1400_e1014;

        let assign1410_e1017: f64 = (p.p241 / 1e-6);
        var_mks_nsubb = assign1410_e1017;

        let assign1420_e1020: f64 = (p.p242 * 0.01);
        var_mks_rth0 = assign1420_e1020;

        let assign1430_e1023: f64 = (p.p243 / 0.01);
        var_mks_cth0 = assign1430_e1023;

        let assign1440_e1026: f64 = (p.p59 / 1e-6);
        var_mks_nover = assign1440_e1026;

        let assign1450_e1029: f64 = (p.p284 / 1e-6);
        var_mks_njunc = assign1450_e1029;

        let assign1460_e1032: f64 = (p.p148 / 1e-6);
        var_mks_nsti = assign1460_e1032;

        let assign1470_e1035: f64 = (p.p198 / 0.0001);
        var_mks_wfc = assign1470_e1035;

        let assign1480_e1038: f64 = (p.p70 * 0.01);
        var_mks_parl1 = assign1480_e1038;

        let (assign1490_e1044,) = {
    if (p.p83 == 0.0) {
        (0.0,)
    } else {
        (p.p84,)
    }
};
        var_uc_sc2 = assign1490_e1044;

        let (assign1500_e1050,) = {
    if (p.p83 == 0.0) {
        (0.0,)
    } else {
        (p.p85,)
    }
};
        var_uc_sc3 = assign1500_e1050;

        let (assign1510_e1056,) = {
    if (p.p80 == 0.0) {
        (0.0,)
    } else {
        (p.p81,)
    }
};
        var_uc_scp2 = assign1510_e1056;

        let (assign1520_e1062,) = {
    if (p.p83 == 0.0) {
        (0.0,)
    } else {
        (p.p82,)
    }
};
        var_uc_scp3 = assign1520_e1062;

        let assign1530_e1065: f64 = (p.p250 * 1000000.0);
        var_uc_gdld = assign1530_e1065;

        let assign1540_e1068: f64 = (p.p232 + 273.15);
        var_uc_tnom = assign1540_e1068;

        var_uc_vfbover = p.p58;

        var_flg_info = p.p46;

        var_flg_nqs = p.p34;

        let (assign1590_e1083,) = {
    if param_given[190] {
        (p.p190,)
    } else {
        let assign1590_e1081: f64 = (p.p237 * p.p240);
        let assign1590_e1082: f64 = (5000000000.0 / assign1590_e1081);
        (assign1590_e1082,)
    }
};
        var_uc_clm2 = assign1590_e1083;
        var_uc_clm2_dn0 = 0.0;
        var_uc_clm2_dn2 = 0.0;
        var_uc_clm2_dn6 = 0.0;
        var_uc_clm2_dn7 = 0.0;
        var_uc_clm2_dn10 = 0.0;
        var_uc_clm2_dn11 = 0.0;
        var_uc_clm2_dn12 = 0.0;
        var_uc_clm2_dn17 = 0.0;

        let assign1600_e1087: f64 = (2.0 + 0.1);
        let assign1600_e1092: f64 = if ((var_uc_clm2 < assign1600_e1087) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        var_guard4 = assign1600_e1092;

        let (assign1610_e1100, assign1610_e1100_d_n0, assign1610_e1100_d_n2, assign1610_e1100_d_n6, assign1610_e1100_d_n7, assign1610_e1100_d_n10, assign1610_e1100_d_n11, assign1610_e1100_d_n12, assign1610_e1100_d_n17,) = {
    if (var_guard4 != 0.0) {
        let assign1610_e1096: f64 = (2.0 + 0.1);
        let assign1610_e1098: f64 = (assign1610_e1096 - var_uc_clm2);
        (assign1610_e1098, (-var_uc_clm2_dn0), (-var_uc_clm2_dn2), (-var_uc_clm2_dn6), (-var_uc_clm2_dn7), (-var_uc_clm2_dn10), (-var_uc_clm2_dn11), (-var_uc_clm2_dn12), (-var_uc_clm2_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign1610_e1100;
        var_tmf1_dn0 = assign1610_e1100_d_n0;
        var_tmf1_dn2 = assign1610_e1100_d_n2;
        var_tmf1_dn6 = assign1610_e1100_d_n6;
        var_tmf1_dn7 = assign1610_e1100_d_n7;
        var_tmf1_dn10 = assign1610_e1100_d_n10;
        var_tmf1_dn11 = assign1610_e1100_d_n11;
        var_tmf1_dn12 = assign1610_e1100_d_n12;
        var_tmf1_dn17 = assign1610_e1100_d_n17;

        let (assign1620_e1106, assign1620_e1106_d_n0, assign1620_e1106_d_n2, assign1620_e1106_d_n6, assign1620_e1106_d_n7, assign1620_e1106_d_n10, assign1620_e1106_d_n11, assign1620_e1106_d_n12, assign1620_e1106_d_n17,) = {
    if (var_guard4 != 0.0) {
        let assign1620_e1104: f64 = (var_tmf1 * var_tmf1);
        (assign1620_e1104, ((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)), ((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)), ((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)), ((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)), ((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)), ((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)), ((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)), ((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn6, var_x2_dn7, var_x2_dn10, var_x2_dn11, var_x2_dn12, var_x2_dn17,)
    }
};
        var_x2 = assign1620_e1106;
        var_x2_dn0 = assign1620_e1106_d_n0;
        var_x2_dn2 = assign1620_e1106_d_n2;
        var_x2_dn6 = assign1620_e1106_d_n6;
        var_x2_dn7 = assign1620_e1106_d_n7;
        var_x2_dn10 = assign1620_e1106_d_n10;
        var_x2_dn11 = assign1620_e1106_d_n11;
        var_x2_dn12 = assign1620_e1106_d_n12;
        var_x2_dn17 = assign1620_e1106_d_n17;

        let (assign1630_e1112, assign1630_e1112_d_n0, assign1630_e1112_d_n2, assign1630_e1112_d_n6, assign1630_e1112_d_n7, assign1630_e1112_d_n10, assign1630_e1112_d_n11, assign1630_e1112_d_n12, assign1630_e1112_d_n17,) = {
    if (var_guard4 != 0.0) {
        let assign1630_e1110: f64 = (0.1 * 0.1);
        (assign1630_e1110, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn6, var_xmax2_dn7, var_xmax2_dn10, var_xmax2_dn11, var_xmax2_dn12, var_xmax2_dn17,)
    }
};
        var_xmax2 = assign1630_e1112;
        var_xmax2_dn0 = assign1630_e1112_d_n0;
        var_xmax2_dn2 = assign1630_e1112_d_n2;
        var_xmax2_dn6 = assign1630_e1112_d_n6;
        var_xmax2_dn7 = assign1630_e1112_d_n7;
        var_xmax2_dn10 = assign1630_e1112_d_n10;
        var_xmax2_dn11 = assign1630_e1112_d_n11;
        var_xmax2_dn12 = assign1630_e1112_d_n12;
        var_xmax2_dn17 = assign1630_e1112_d_n17;

        let (assign1640_e1116, assign1640_e1116_d_n0, assign1640_e1116_d_n2, assign1640_e1116_d_n6, assign1640_e1116_d_n7, assign1640_e1116_d_n10, assign1640_e1116_d_n11, assign1640_e1116_d_n12, assign1640_e1116_d_n17,) = {
    if (var_guard4 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign1640_e1116;
        var_xp_dn0 = assign1640_e1116_d_n0;
        var_xp_dn2 = assign1640_e1116_d_n2;
        var_xp_dn6 = assign1640_e1116_d_n6;
        var_xp_dn7 = assign1640_e1116_d_n7;
        var_xp_dn10 = assign1640_e1116_d_n10;
        var_xp_dn11 = assign1640_e1116_d_n11;
        var_xp_dn12 = assign1640_e1116_d_n12;
        var_xp_dn17 = assign1640_e1116_d_n17;

        let (assign1650_e1120, assign1650_e1120_d_n0, assign1650_e1120_d_n2, assign1650_e1120_d_n6, assign1650_e1120_d_n7, assign1650_e1120_d_n10, assign1650_e1120_d_n11, assign1650_e1120_d_n12, assign1650_e1120_d_n17,) = {
    if (var_guard4 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign1650_e1120;
        var_xmp_dn0 = assign1650_e1120_d_n0;
        var_xmp_dn2 = assign1650_e1120_d_n2;
        var_xmp_dn6 = assign1650_e1120_d_n6;
        var_xmp_dn7 = assign1650_e1120_d_n7;
        var_xmp_dn10 = assign1650_e1120_d_n10;
        var_xmp_dn11 = assign1650_e1120_d_n11;
        var_xmp_dn12 = assign1650_e1120_d_n12;
        var_xmp_dn17 = assign1650_e1120_d_n17;

        let (assign1660_e1124,) = {
    if (var_guard4 != 0.0) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign1660_e1124;

        let (assign1670_e1128,) = {
    if (var_guard4 != 0.0) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign1670_e1128;

        let (assign1680_e1132, assign1680_e1132_d_n0, assign1680_e1132_d_n2, assign1680_e1132_d_n6, assign1680_e1132_d_n7, assign1680_e1132_d_n10, assign1680_e1132_d_n11, assign1680_e1132_d_n12, assign1680_e1132_d_n17,) = {
    if (var_guard4 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    }
};
        var_arg = assign1680_e1132;
        var_arg_dn0 = assign1680_e1132_d_n0;
        var_arg_dn2 = assign1680_e1132_d_n2;
        var_arg_dn6 = assign1680_e1132_d_n6;
        var_arg_dn7 = assign1680_e1132_d_n7;
        var_arg_dn10 = assign1680_e1132_d_n10;
        var_arg_dn11 = assign1680_e1132_d_n11;
        var_arg_dn12 = assign1680_e1132_d_n12;
        var_arg_dn17 = assign1680_e1132_d_n17;

        let (assign1690_e1136, assign1690_e1136_d_n0, assign1690_e1136_d_n2, assign1690_e1136_d_n6, assign1690_e1136_d_n7, assign1690_e1136_d_n10, assign1690_e1136_d_n11, assign1690_e1136_d_n12, assign1690_e1136_d_n17,) = {
    if (var_guard4 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign1690_e1136;
        var_dnm_dn0 = assign1690_e1136_d_n0;
        var_dnm_dn2 = assign1690_e1136_d_n2;
        var_dnm_dn6 = assign1690_e1136_d_n6;
        var_dnm_dn7 = assign1690_e1136_d_n7;
        var_dnm_dn10 = assign1690_e1136_d_n10;
        var_dnm_dn11 = assign1690_e1136_d_n11;
        var_dnm_dn12 = assign1690_e1136_d_n12;
        var_dnm_dn17 = assign1690_e1136_d_n17;

        let (assign1700_e1142, assign1700_e1142_d_n0, assign1700_e1142_d_n2, assign1700_e1142_d_n6, assign1700_e1142_d_n7, assign1700_e1142_d_n10, assign1700_e1142_d_n11, assign1700_e1142_d_n12, assign1700_e1142_d_n17,) = {
    if (var_guard4 != 0.0) {
        let assign1700_e1140: f64 = (var_xp * var_x2);
        (assign1700_e1140, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign1700_e1142;
        var_xp_dn0 = assign1700_e1142_d_n0;
        var_xp_dn2 = assign1700_e1142_d_n2;
        var_xp_dn6 = assign1700_e1142_d_n6;
        var_xp_dn7 = assign1700_e1142_d_n7;
        var_xp_dn10 = assign1700_e1142_d_n10;
        var_xp_dn11 = assign1700_e1142_d_n11;
        var_xp_dn12 = assign1700_e1142_d_n12;
        var_xp_dn17 = assign1700_e1142_d_n17;

        let (assign1710_e1148, assign1710_e1148_d_n0, assign1710_e1148_d_n2, assign1710_e1148_d_n6, assign1710_e1148_d_n7, assign1710_e1148_d_n10, assign1710_e1148_d_n11, assign1710_e1148_d_n12, assign1710_e1148_d_n17,) = {
    if (var_guard4 != 0.0) {
        let assign1710_e1146: f64 = (var_xmp * var_xmax2);
        (assign1710_e1146, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign1710_e1148;
        var_xmp_dn0 = assign1710_e1148_d_n0;
        var_xmp_dn2 = assign1710_e1148_d_n2;
        var_xmp_dn6 = assign1710_e1148_d_n6;
        var_xmp_dn7 = assign1710_e1148_d_n7;
        var_xmp_dn10 = assign1710_e1148_d_n10;
        var_xmp_dn11 = assign1710_e1148_d_n11;
        var_xmp_dn12 = assign1710_e1148_d_n12;
        var_xmp_dn17 = assign1710_e1148_d_n17;

        let (assign1720_e1154, assign1720_e1154_d_n0, assign1720_e1154_d_n2, assign1720_e1154_d_n6, assign1720_e1154_d_n7, assign1720_e1154_d_n10, assign1720_e1154_d_n11, assign1720_e1154_d_n12, assign1720_e1154_d_n17,) = {
    if (var_guard4 != 0.0) {
        let assign1720_e1152: f64 = (var_xp * var_x2);
        (assign1720_e1152, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign1720_e1154;
        var_xp_dn0 = assign1720_e1154_d_n0;
        var_xp_dn2 = assign1720_e1154_d_n2;
        var_xp_dn6 = assign1720_e1154_d_n6;
        var_xp_dn7 = assign1720_e1154_d_n7;
        var_xp_dn10 = assign1720_e1154_d_n10;
        var_xp_dn11 = assign1720_e1154_d_n11;
        var_xp_dn12 = assign1720_e1154_d_n12;
        var_xp_dn17 = assign1720_e1154_d_n17;

        let (assign1730_e1160, assign1730_e1160_d_n0, assign1730_e1160_d_n2, assign1730_e1160_d_n6, assign1730_e1160_d_n7, assign1730_e1160_d_n10, assign1730_e1160_d_n11, assign1730_e1160_d_n12, assign1730_e1160_d_n17,) = {
    if (var_guard4 != 0.0) {
        let assign1730_e1158: f64 = (var_xmp * var_xmax2);
        (assign1730_e1158, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign1730_e1160;
        var_xmp_dn0 = assign1730_e1160_d_n0;
        var_xmp_dn2 = assign1730_e1160_d_n2;
        var_xmp_dn6 = assign1730_e1160_d_n6;
        var_xmp_dn7 = assign1730_e1160_d_n7;
        var_xmp_dn10 = assign1730_e1160_d_n10;
        var_xmp_dn11 = assign1730_e1160_d_n11;
        var_xmp_dn12 = assign1730_e1160_d_n12;
        var_xmp_dn17 = assign1730_e1160_d_n17;

        let (assign1740_e1166, assign1740_e1166_d_n0, assign1740_e1166_d_n2, assign1740_e1166_d_n6, assign1740_e1166_d_n7, assign1740_e1166_d_n10, assign1740_e1166_d_n11, assign1740_e1166_d_n12, assign1740_e1166_d_n17,) = {
    if (var_guard4 != 0.0) {
        let assign1740_e1164: f64 = (var_xp + var_xmp);
        (assign1740_e1164, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn7 + var_xmp_dn7), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn11 + var_xmp_dn11), (var_xp_dn12 + var_xmp_dn12), (var_xp_dn17 + var_xmp_dn17),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    }
};
        var_arg = assign1740_e1166;
        var_arg_dn0 = assign1740_e1166_d_n0;
        var_arg_dn2 = assign1740_e1166_d_n2;
        var_arg_dn6 = assign1740_e1166_d_n6;
        var_arg_dn7 = assign1740_e1166_d_n7;
        var_arg_dn10 = assign1740_e1166_d_n10;
        var_arg_dn11 = assign1740_e1166_d_n11;
        var_arg_dn12 = assign1740_e1166_d_n12;
        var_arg_dn17 = assign1740_e1166_d_n17;

        let (assign1750_e1170, assign1750_e1170_d_n0, assign1750_e1170_d_n2, assign1750_e1170_d_n6, assign1750_e1170_d_n7, assign1750_e1170_d_n10, assign1750_e1170_d_n11, assign1750_e1170_d_n12, assign1750_e1170_d_n17,) = {
    if (var_guard4 != 0.0) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign1750_e1170;
        var_dnm_dn0 = assign1750_e1170_d_n0;
        var_dnm_dn2 = assign1750_e1170_d_n2;
        var_dnm_dn6 = assign1750_e1170_d_n6;
        var_dnm_dn7 = assign1750_e1170_d_n7;
        var_dnm_dn10 = assign1750_e1170_d_n10;
        var_dnm_dn11 = assign1750_e1170_d_n11;
        var_dnm_dn12 = assign1750_e1170_d_n12;
        var_dnm_dn17 = assign1750_e1170_d_n17;

        let assign1760_e1185: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard5 = assign1760_e1185;

        let assign1770_e1188: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard6 = assign1770_e1188;

        *var_arg_slot = var_arg;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn10_slot = var_arg_dn10;
        *var_arg_dn11_slot = var_arg_dn11;
        *var_arg_dn12_slot = var_arg_dn12;
        *var_arg_dn17_slot = var_arg_dn17;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn11_slot = var_dnm_dn11;
        *var_dnm_dn12_slot = var_dnm_dn12;
        *var_dnm_dn17_slot = var_dnm_dn17;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_flg_info_slot = var_flg_info;
        *var_flg_nqs_slot = var_flg_nqs;
        *var_flg_skipacc_slot = var_flg_skipacc;
        *var_fs01_slot = var_fs01;
        *var_fs01_dn0_slot = var_fs01_dn0;
        *var_fs01_dn10_slot = var_fs01_dn10;
        *var_fs01_dn11_slot = var_fs01_dn11;
        *var_fs01_dn12_slot = var_fs01_dn12;
        *var_fs01_dn17_slot = var_fs01_dn17;
        *var_fs01_dn2_slot = var_fs01_dn2;
        *var_fs01_dn6_slot = var_fs01_dn6;
        *var_fs01_dn7_slot = var_fs01_dn7;
        *var_fs02_slot = var_fs02;
        *var_fs02_dn0_slot = var_fs02_dn0;
        *var_fs02_dn10_slot = var_fs02_dn10;
        *var_fs02_dn11_slot = var_fs02_dn11;
        *var_fs02_dn12_slot = var_fs02_dn12;
        *var_fs02_dn17_slot = var_fs02_dn17;
        *var_fs02_dn2_slot = var_fs02_dn2;
        *var_fs02_dn6_slot = var_fs02_dn6;
        *var_fs02_dn7_slot = var_fs02_dn7;
        *var_fsl1_slot = var_fsl1;
        *var_fsl1_dn0_slot = var_fsl1_dn0;
        *var_fsl1_dn10_slot = var_fsl1_dn10;
        *var_fsl1_dn11_slot = var_fsl1_dn11;
        *var_fsl1_dn12_slot = var_fsl1_dn12;
        *var_fsl1_dn17_slot = var_fsl1_dn17;
        *var_fsl1_dn2_slot = var_fsl1_dn2;
        *var_fsl1_dn6_slot = var_fsl1_dn6;
        *var_fsl1_dn7_slot = var_fsl1_dn7;
        *var_fsl2_slot = var_fsl2;
        *var_fsl2_dn0_slot = var_fsl2_dn0;
        *var_fsl2_dn10_slot = var_fsl2_dn10;
        *var_fsl2_dn11_slot = var_fsl2_dn11;
        *var_fsl2_dn12_slot = var_fsl2_dn12;
        *var_fsl2_dn17_slot = var_fsl2_dn17;
        *var_fsl2_dn2_slot = var_fsl2_dn2;
        *var_fsl2_dn6_slot = var_fsl2_dn6;
        *var_fsl2_dn7_slot = var_fsl2_dn7;
        *var_guard4_slot = var_guard4;
        *var_guard5_slot = var_guard5;
        *var_guard6_slot = var_guard6;
        *var_idspt0_slot = var_idspt0;
        *var_idspt0_dn0_slot = var_idspt0_dn0;
        *var_idspt0_dn10_slot = var_idspt0_dn10;
        *var_idspt0_dn11_slot = var_idspt0_dn11;
        *var_idspt0_dn12_slot = var_idspt0_dn12;
        *var_idspt0_dn17_slot = var_idspt0_dn17;
        *var_idspt0_dn2_slot = var_idspt0_dn2;
        *var_idspt0_dn6_slot = var_idspt0_dn6;
        *var_idspt0_dn7_slot = var_idspt0_dn7;
        *var_lp_s0_max_slot = var_lp_s0_max;
        *var_lp_sl_max_slot = var_lp_sl_max;
        *var_m0_slot = var_m0;
        *var_mks_cth0_slot = var_mks_cth0;
        *var_mks_njunc_slot = var_mks_njunc;
        *var_mks_nover_slot = var_mks_nover;
        *var_mks_nsti_slot = var_mks_nsti;
        *var_mks_nsubb_slot = var_mks_nsubb;
        *var_mks_nsubcmax_slot = var_mks_nsubcmax;
        *var_mks_nsubp_slot = var_mks_nsubp;
        *var_mks_nsubs_slot = var_mks_nsubs;
        *var_mks_parl1_slot = var_mks_parl1;
        *var_mks_rth0_slot = var_mks_rth0;
        *var_mks_vmax_slot = var_mks_vmax;
        *var_mks_vtmp_slot = var_mks_vtmp;
        *var_mks_wfc_slot = var_mks_wfc;
        *var_mm_slot = var_mm;
        *var_ps0_slot = var_ps0;
        *var_ps0_dn0_slot = var_ps0_dn0;
        *var_ps0_dn10_slot = var_ps0_dn10;
        *var_ps0_dn11_slot = var_ps0_dn11;
        *var_ps0_dn12_slot = var_ps0_dn12;
        *var_ps0_dn17_slot = var_ps0_dn17;
        *var_ps0_dn2_slot = var_ps0_dn2;
        *var_ps0_dn6_slot = var_ps0_dn6;
        *var_ps0_dn7_slot = var_ps0_dn7;
        *var_ps0_ini_slot = var_ps0_ini;
        *var_ps0_ini_dn0_slot = var_ps0_ini_dn0;
        *var_ps0_ini_dn10_slot = var_ps0_ini_dn10;
        *var_ps0_ini_dn11_slot = var_ps0_ini_dn11;
        *var_ps0_ini_dn12_slot = var_ps0_ini_dn12;
        *var_ps0_ini_dn17_slot = var_ps0_ini_dn17;
        *var_ps0_ini_dn2_slot = var_ps0_ini_dn2;
        *var_ps0_ini_dn6_slot = var_ps0_ini_dn6;
        *var_ps0_ini_dn7_slot = var_ps0_ini_dn7;
        *var_q_s0_dep_ini_slot = var_q_s0_dep_ini;
        *var_q_s0_dep_ini_dn0_slot = var_q_s0_dep_ini_dn0;
        *var_q_s0_dep_ini_dn10_slot = var_q_s0_dep_ini_dn10;
        *var_q_s0_dep_ini_dn11_slot = var_q_s0_dep_ini_dn11;
        *var_q_s0_dep_ini_dn12_slot = var_q_s0_dep_ini_dn12;
        *var_q_s0_dep_ini_dn17_slot = var_q_s0_dep_ini_dn17;
        *var_q_s0_dep_ini_dn2_slot = var_q_s0_dep_ini_dn2;
        *var_q_s0_dep_ini_dn6_slot = var_q_s0_dep_ini_dn6;
        *var_q_s0_dep_ini_dn7_slot = var_q_s0_dep_ini_dn7;
        *var_qb_qs_slot = var_qb_qs;
        *var_qb_qs_dn0_slot = var_qb_qs_dn0;
        *var_qb_qs_dn10_slot = var_qb_qs_dn10;
        *var_qb_qs_dn11_slot = var_qb_qs_dn11;
        *var_qb_qs_dn12_slot = var_qb_qs_dn12;
        *var_qb_qs_dn13_slot = var_qb_qs_dn13;
        *var_qb_qs_dn15_slot = var_qb_qs_dn15;
        *var_qb_qs_dn16_slot = var_qb_qs_dn16;
        *var_qb_qs_dn17_slot = var_qb_qs_dn17;
        *var_qb_qs_dn18_slot = var_qb_qs_dn18;
        *var_qb_qs_dn2_slot = var_qb_qs_dn2;
        *var_qb_qs_dn6_slot = var_qb_qs_dn6;
        *var_qb_qs_dn7_slot = var_qb_qs_dn7;
        *var_qi_qs_slot = var_qi_qs;
        *var_qi_qs_dn0_slot = var_qi_qs_dn0;
        *var_qi_qs_dn10_slot = var_qi_qs_dn10;
        *var_qi_qs_dn11_slot = var_qi_qs_dn11;
        *var_qi_qs_dn12_slot = var_qi_qs_dn12;
        *var_qi_qs_dn17_slot = var_qi_qs_dn17;
        *var_qi_qs_dn2_slot = var_qi_qs_dn2;
        *var_qi_qs_dn6_slot = var_qi_qs_dn6;
        *var_qi_qs_dn7_slot = var_qi_qs_dn7;
        *var_qs_qs_slot = var_qs_qs;
        *var_qs_qs_dn0_slot = var_qs_qs_dn0;
        *var_qs_qs_dn10_slot = var_qs_qs_dn10;
        *var_qs_qs_dn11_slot = var_qs_qs_dn11;
        *var_qs_qs_dn12_slot = var_qs_qs_dn12;
        *var_qs_qs_dn13_slot = var_qs_qs_dn13;
        *var_qs_qs_dn15_slot = var_qs_qs_dn15;
        *var_qs_qs_dn16_slot = var_qs_qs_dn16;
        *var_qs_qs_dn17_slot = var_qs_qs_dn17;
        *var_qs_qs_dn18_slot = var_qs_qs_dn18;
        *var_qs_qs_dn2_slot = var_qs_qs_dn2;
        *var_qs_qs_dn6_slot = var_qs_qs_dn6;
        *var_qs_qs_dn7_slot = var_qs_qs_dn7;
        *var_subversion_slot = var_subversion;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_uc_clm2_slot = var_uc_clm2;
        *var_uc_clm2_dn0_slot = var_uc_clm2_dn0;
        *var_uc_clm2_dn10_slot = var_uc_clm2_dn10;
        *var_uc_clm2_dn11_slot = var_uc_clm2_dn11;
        *var_uc_clm2_dn12_slot = var_uc_clm2_dn12;
        *var_uc_clm2_dn17_slot = var_uc_clm2_dn17;
        *var_uc_clm2_dn2_slot = var_uc_clm2_dn2;
        *var_uc_clm2_dn6_slot = var_uc_clm2_dn6;
        *var_uc_clm2_dn7_slot = var_uc_clm2_dn7;
        *var_uc_gdld_slot = var_uc_gdld;
        *var_uc_sc2_slot = var_uc_sc2;
        *var_uc_sc3_slot = var_uc_sc3;
        *var_uc_scp2_slot = var_uc_scp2;
        *var_uc_scp3_slot = var_uc_scp3;
        *var_uc_tnom_slot = var_uc_tnom;
        *var_uc_vfbover_slot = var_uc_vfbover;
        *var_vbcs_cl_slot = var_vbcs_cl;
        *var_vbcs_cl_dn0_slot = var_vbcs_cl_dn0;
        *var_vbcs_cl_dn10_slot = var_vbcs_cl_dn10;
        *var_vbcs_cl_dn11_slot = var_vbcs_cl_dn11;
        *var_vbcs_cl_dn12_slot = var_vbcs_cl_dn12;
        *var_vbcs_cl_dn17_slot = var_vbcs_cl_dn17;
        *var_vbcs_cl_dn2_slot = var_vbcs_cl_dn2;
        *var_vbcs_cl_dn6_slot = var_vbcs_cl_dn6;
        *var_vbcs_cl_dn7_slot = var_vbcs_cl_dn7;
        *var_vbsbiz_slot = var_vbsbiz;
        *var_vbsbiz_dn0_slot = var_vbsbiz_dn0;
        *var_vbsbiz_dn10_slot = var_vbsbiz_dn10;
        *var_vbsbiz_dn11_slot = var_vbsbiz_dn11;
        *var_vbsbiz_dn12_slot = var_vbsbiz_dn12;
        *var_vbsbiz_dn17_slot = var_vbsbiz_dn17;
        *var_vbsbiz_dn2_slot = var_vbsbiz_dn2;
        *var_vbsbiz_dn6_slot = var_vbsbiz_dn6;
        *var_vbsbiz_dn7_slot = var_vbsbiz_dn7;
        *var_x2_slot = var_x2;
        *var_x2_dn0_slot = var_x2_dn0;
        *var_x2_dn10_slot = var_x2_dn10;
        *var_x2_dn11_slot = var_x2_dn11;
        *var_x2_dn12_slot = var_x2_dn12;
        *var_x2_dn17_slot = var_x2_dn17;
        *var_x2_dn2_slot = var_x2_dn2;
        *var_x2_dn6_slot = var_x2_dn6;
        *var_x2_dn7_slot = var_x2_dn7;
        *var_xmax2_slot = var_xmax2;
        *var_xmax2_dn0_slot = var_xmax2_dn0;
        *var_xmax2_dn10_slot = var_xmax2_dn10;
        *var_xmax2_dn11_slot = var_xmax2_dn11;
        *var_xmax2_dn12_slot = var_xmax2_dn12;
        *var_xmax2_dn17_slot = var_xmax2_dn17;
        *var_xmax2_dn2_slot = var_xmax2_dn2;
        *var_xmax2_dn6_slot = var_xmax2_dn6;
        *var_xmax2_dn7_slot = var_xmax2_dn7;
        *var_xmp_slot = var_xmp;
        *var_xmp_dn0_slot = var_xmp_dn0;
        *var_xmp_dn10_slot = var_xmp_dn10;
        *var_xmp_dn11_slot = var_xmp_dn11;
        *var_xmp_dn12_slot = var_xmp_dn12;
        *var_xmp_dn17_slot = var_xmp_dn17;
        *var_xmp_dn2_slot = var_xmp_dn2;
        *var_xmp_dn6_slot = var_xmp_dn6;
        *var_xmp_dn7_slot = var_xmp_dn7;
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn11_slot = var_xp_dn11;
        *var_xp_dn12_slot = var_xp_dn12;
        *var_xp_dn17_slot = var_xp_dn17;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn7_slot = var_xp_dn7;
    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        var_guard4: f64,
        var_guard5: f64,
        var_guard6: f64,
        var_mks_nsti: f64,
        var_mks_nsubb: f64,
        var_mks_nsubs: f64,
        var_subversion: f64,
        var_tmf1: f64,
        var_tmf1_dn0: f64,
        var_tmf1_dn10: f64,
        var_tmf1_dn11: f64,
        var_tmf1_dn12: f64,
        var_tmf1_dn17: f64,
        var_tmf1_dn2: f64,
        var_tmf1_dn6: f64,
        var_tmf1_dn7: f64,
        var_uc_gdld: f64,
        var_uc_tnom: f64,
        var_betatnom_slot: &mut f64,
        var_c0bulk_slot: &mut f64,
        var_c_box_slot: &mut f64,
        var_c_box_fd_inv_slot: &mut f64,
        var_c_box_inv_slot: &mut f64,
        var_c_fox0_slot: &mut f64,
        var_c_fox0_inv_slot: &mut f64,
        var_c_soi_slot: &mut f64,
        var_c_soi_inv_slot: &mut f64,
        var_clmmod_slot: &mut f64,
        var_cnstpgd_slot: &mut f64,
        var_costi00_slot: &mut f64,
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn11_slot: &mut f64,
        var_dnm_dn12_slot: &mut f64,
        var_dnm_dn17_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_dvthsm_slot: &mut f64,
        var_dw_slot: &mut f64,
        var_dwbt_slot: &mut f64,
        var_dwcv_slot: &mut f64,
        var_egtnom_slot: &mut f64,
        var_gdl0_slot: &mut f64,
        var_grg_cnst_slot: &mut f64,
        var_guard10_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard12_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard8_slot: &mut f64,
        var_guard9_slot: &mut f64,
        var_leff_slot: &mut f64,
        var_leff_cv_slot: &mut f64,
        var_lgate_slot: &mut f64,
        var_lgatesm_slot: &mut f64,
        var_lgle_slot: &mut f64,
        var_lgleff_slot: &mut f64,
        var_lod_half_ref_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_mks_nsubp_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_muesr_slot: &mut f64,
        var_nsti_p2_slot: &mut f64,
        var_nsubpp_slot: &mut f64,
        var_pt40_slot: &mut f64,
        var_ptl0_slot: &mut f64,
        var_qnbulk_esi_slot: &mut f64,
        var_tfox0_slot: &mut f64,
        var_tmf0_slot: &mut f64,
        var_tmf0_dn0_slot: &mut f64,
        var_tmf0_dn10_slot: &mut f64,
        var_tmf0_dn11_slot: &mut f64,
        var_tmf0_dn12_slot: &mut f64,
        var_tmf0_dn17_slot: &mut f64,
        var_tmf0_dn2_slot: &mut f64,
        var_tmf0_dn6_slot: &mut f64,
        var_tmf0_dn7_slot: &mut f64,
        var_uc_clm2_slot: &mut f64,
        var_uc_clm2_dn0_slot: &mut f64,
        var_uc_clm2_dn10_slot: &mut f64,
        var_uc_clm2_dn11_slot: &mut f64,
        var_uc_clm2_dn12_slot: &mut f64,
        var_uc_clm2_dn17_slot: &mut f64,
        var_uc_clm2_dn2_slot: &mut f64,
        var_uc_clm2_dn6_slot: &mut f64,
        var_uc_clm2_dn7_slot: &mut f64,
        var_uc_wsti_slot: &mut f64,
        var_vfb_slot: &mut f64,
        var_weff_slot: &mut f64,
        var_weff_cv_slot: &mut f64,
        var_weff_nf_slot: &mut f64,
        var_weffcv_nf_slot: &mut f64,
        var_wg_slot: &mut f64,
        var_wgate_slot: &mut f64,
        var_wl_slot: &mut f64,
        var_xgate_slot: &mut f64,
        var_xsub1_slot: &mut f64,
        var_xsub2_slot: &mut f64,
        var_xvbs_slot: &mut f64,
        var_zvgs_slot: &mut f64,
    ) {
        let mut var_betatnom: f64 = *var_betatnom_slot;
        let mut var_c0bulk: f64 = *var_c0bulk_slot;
        let mut var_c_box: f64 = *var_c_box_slot;
        let mut var_c_box_fd_inv: f64 = *var_c_box_fd_inv_slot;
        let mut var_c_box_inv: f64 = *var_c_box_inv_slot;
        let mut var_c_fox0: f64 = *var_c_fox0_slot;
        let mut var_c_fox0_inv: f64 = *var_c_fox0_inv_slot;
        let mut var_c_soi: f64 = *var_c_soi_slot;
        let mut var_c_soi_inv: f64 = *var_c_soi_inv_slot;
        let mut var_clmmod: f64 = *var_clmmod_slot;
        let mut var_cnstpgd: f64 = *var_cnstpgd_slot;
        let mut var_costi00: f64 = *var_costi00_slot;
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn11: f64 = *var_dnm_dn11_slot;
        let mut var_dnm_dn12: f64 = *var_dnm_dn12_slot;
        let mut var_dnm_dn17: f64 = *var_dnm_dn17_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_dvthsm: f64 = *var_dvthsm_slot;
        let mut var_dw: f64 = *var_dw_slot;
        let mut var_dwbt: f64 = *var_dwbt_slot;
        let mut var_dwcv: f64 = *var_dwcv_slot;
        let mut var_egtnom: f64 = *var_egtnom_slot;
        let mut var_gdl0: f64 = *var_gdl0_slot;
        let mut var_grg_cnst: f64 = *var_grg_cnst_slot;
        let mut var_guard10: f64 = *var_guard10_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard12: f64 = *var_guard12_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard8: f64 = *var_guard8_slot;
        let mut var_guard9: f64 = *var_guard9_slot;
        let mut var_leff: f64 = *var_leff_slot;
        let mut var_leff_cv: f64 = *var_leff_cv_slot;
        let mut var_lgate: f64 = *var_lgate_slot;
        let mut var_lgatesm: f64 = *var_lgatesm_slot;
        let mut var_lgle: f64 = *var_lgle_slot;
        let mut var_lgleff: f64 = *var_lgleff_slot;
        let mut var_lod_half_ref: f64 = *var_lod_half_ref_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_mks_nsubp: f64 = *var_mks_nsubp_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_muesr: f64 = *var_muesr_slot;
        let mut var_nsti_p2: f64 = *var_nsti_p2_slot;
        let mut var_nsubpp: f64 = *var_nsubpp_slot;
        let mut var_pt40: f64 = *var_pt40_slot;
        let mut var_ptl0: f64 = *var_ptl0_slot;
        let mut var_qnbulk_esi: f64 = *var_qnbulk_esi_slot;
        let mut var_tfox0: f64 = *var_tfox0_slot;
        let mut var_tmf0: f64 = *var_tmf0_slot;
        let mut var_tmf0_dn0: f64 = *var_tmf0_dn0_slot;
        let mut var_tmf0_dn10: f64 = *var_tmf0_dn10_slot;
        let mut var_tmf0_dn11: f64 = *var_tmf0_dn11_slot;
        let mut var_tmf0_dn12: f64 = *var_tmf0_dn12_slot;
        let mut var_tmf0_dn17: f64 = *var_tmf0_dn17_slot;
        let mut var_tmf0_dn2: f64 = *var_tmf0_dn2_slot;
        let mut var_tmf0_dn6: f64 = *var_tmf0_dn6_slot;
        let mut var_tmf0_dn7: f64 = *var_tmf0_dn7_slot;
        let mut var_uc_clm2: f64 = *var_uc_clm2_slot;
        let mut var_uc_clm2_dn0: f64 = *var_uc_clm2_dn0_slot;
        let mut var_uc_clm2_dn10: f64 = *var_uc_clm2_dn10_slot;
        let mut var_uc_clm2_dn11: f64 = *var_uc_clm2_dn11_slot;
        let mut var_uc_clm2_dn12: f64 = *var_uc_clm2_dn12_slot;
        let mut var_uc_clm2_dn17: f64 = *var_uc_clm2_dn17_slot;
        let mut var_uc_clm2_dn2: f64 = *var_uc_clm2_dn2_slot;
        let mut var_uc_clm2_dn6: f64 = *var_uc_clm2_dn6_slot;
        let mut var_uc_clm2_dn7: f64 = *var_uc_clm2_dn7_slot;
        let mut var_uc_wsti: f64 = *var_uc_wsti_slot;
        let mut var_vfb: f64 = *var_vfb_slot;
        let mut var_weff: f64 = *var_weff_slot;
        let mut var_weff_cv: f64 = *var_weff_cv_slot;
        let mut var_weff_nf: f64 = *var_weff_nf_slot;
        let mut var_weffcv_nf: f64 = *var_weffcv_nf_slot;
        let mut var_wg: f64 = *var_wg_slot;
        let mut var_wgate: f64 = *var_wgate_slot;
        let mut var_wl: f64 = *var_wl_slot;
        let mut var_xgate: f64 = *var_xgate_slot;
        let mut var_xsub1: f64 = *var_xsub1_slot;
        let mut var_xsub2: f64 = *var_xsub2_slot;
        let mut var_xvbs: f64 = *var_xvbs_slot;
        let mut var_zvgs: f64 = *var_zvgs_slot;

        let (assign1780_e1196,) = {
    if (((var_guard4 != 0.0) && (var_guard5 != 0.0)) && (var_guard6 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign1780_e1196;

        let assign1790_e1199: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard7 = assign1790_e1199;

        let (assign1800_e1210,) = {
    if ((((var_guard4 != 0.0) && (var_guard5 != 0.0)) && (var_guard6 == 0.0)) && (var_guard7 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign1800_e1210;

        let assign1810_e1213: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard8 = assign1810_e1213;

        let (assign1820_e1227,) = {
    if (((((var_guard4 != 0.0) && (var_guard5 != 0.0)) && (var_guard6 == 0.0)) && (var_guard7 == 0.0)) && (var_guard8 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign1820_e1227;

        let assign1830_e1230: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard9 = assign1830_e1230;

        let (assign1840_e1247,) = {
    if ((((((var_guard4 != 0.0) && (var_guard5 != 0.0)) && (var_guard6 == 0.0)) && (var_guard7 == 0.0)) && (var_guard8 == 0.0)) && (var_guard9 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign1840_e1247;

        let (assign1850_e1253,) = {
    if ((var_guard4 != 0.0) && (var_guard5 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign1850_e1253;

        let mut assign1860_loop_guard: usize = 0;
        while {
            let assign1860_cond_e1260: f64 = if (((var_guard4 != 0.0) && (var_guard5 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign1860_cond_e1260 != 0.0
        } {
            assign1860_loop_guard += 1;
            assert!(assign1860_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign1860_body0_e1267, assign1860_body0_e1267_d_n0, assign1860_body0_e1267_d_n2, assign1860_body0_e1267_d_n6, assign1860_body0_e1267_d_n7, assign1860_body0_e1267_d_n10, assign1860_body0_e1267_d_n11, assign1860_body0_e1267_d_n12, assign1860_body0_e1267_d_n17,) = {
    if ((var_guard4 != 0.0) && (var_guard5 != 0.0)) {
        let assign1860_body0_e1265: f64 = (var_dnm).sqrt();
        (assign1860_body0_e1265, (var_dnm_dn0 / (2.0 * assign1860_body0_e1265)), (var_dnm_dn2 / (2.0 * assign1860_body0_e1265)), (var_dnm_dn6 / (2.0 * assign1860_body0_e1265)), (var_dnm_dn7 / (2.0 * assign1860_body0_e1265)), (var_dnm_dn10 / (2.0 * assign1860_body0_e1265)), (var_dnm_dn11 / (2.0 * assign1860_body0_e1265)), (var_dnm_dn12 / (2.0 * assign1860_body0_e1265)), (var_dnm_dn17 / (2.0 * assign1860_body0_e1265)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
            var_dnm = assign1860_body0_e1267;
            var_dnm_dn0 = assign1860_body0_e1267_d_n0;
            var_dnm_dn2 = assign1860_body0_e1267_d_n2;
            var_dnm_dn6 = assign1860_body0_e1267_d_n6;
            var_dnm_dn7 = assign1860_body0_e1267_d_n7;
            var_dnm_dn10 = assign1860_body0_e1267_d_n10;
            var_dnm_dn11 = assign1860_body0_e1267_d_n11;
            var_dnm_dn12 = assign1860_body0_e1267_d_n12;
            var_dnm_dn17 = assign1860_body0_e1267_d_n17;
            let (assign1860_body1_e1275,) = {
    if ((var_guard4 != 0.0) && (var_guard5 != 0.0)) {
        let assign1860_body1_e1273: f64 = (var_m0 + 1.0);
        (assign1860_body1_e1273,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign1860_body1_e1275;
        }

        let (assign1870_e1288, assign1870_e1288_d_n0, assign1870_e1288_d_n2, assign1870_e1288_d_n6, assign1870_e1288_d_n7, assign1870_e1288_d_n10, assign1870_e1288_d_n11, assign1870_e1288_d_n12, assign1870_e1288_d_n17,) = {
    if ((var_guard4 != 0.0) && (var_guard5 == 0.0)) {
        let assign1870_e1284: f64 = (2.0 * 2.0);
        let assign1870_e1285: f64 = (1.0 / assign1870_e1284);
        let assign1870_e1286: f64 = (var_dnm).powf(assign1870_e1285);
        (assign1870_e1286, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((var_dnm).powf(assign1870_e1285 - 1.0) * var_dnm_dn0)) } } else { (assign1870_e1286 * (assign1870_e1285 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((var_dnm).powf(assign1870_e1285 - 1.0) * var_dnm_dn2)) } } else { (assign1870_e1286 * (assign1870_e1285 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((var_dnm).powf(assign1870_e1285 - 1.0) * var_dnm_dn6)) } } else { (assign1870_e1286 * (assign1870_e1285 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((var_dnm).powf(assign1870_e1285 - 1.0) * var_dnm_dn7)) } } else { (assign1870_e1286 * (assign1870_e1285 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((var_dnm).powf(assign1870_e1285 - 1.0) * var_dnm_dn10)) } } else { (assign1870_e1286 * (assign1870_e1285 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((var_dnm).powf(assign1870_e1285 - 1.0) * var_dnm_dn11)) } } else { (assign1870_e1286 * (assign1870_e1285 * (var_dnm_dn11 / var_dnm))) }, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((var_dnm).powf(assign1870_e1285 - 1.0) * var_dnm_dn12)) } } else { (assign1870_e1286 * (assign1870_e1285 * (var_dnm_dn12 / var_dnm))) }, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((var_dnm).powf(assign1870_e1285 - 1.0) * var_dnm_dn17)) } } else { (assign1870_e1286 * (assign1870_e1285 * (var_dnm_dn17 / var_dnm))) },)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign1870_e1288;
        var_dnm_dn0 = assign1870_e1288_d_n0;
        var_dnm_dn2 = assign1870_e1288_d_n2;
        var_dnm_dn6 = assign1870_e1288_d_n6;
        var_dnm_dn7 = assign1870_e1288_d_n7;
        var_dnm_dn10 = assign1870_e1288_d_n10;
        var_dnm_dn11 = assign1870_e1288_d_n11;
        var_dnm_dn12 = assign1870_e1288_d_n12;
        var_dnm_dn17 = assign1870_e1288_d_n17;

        let (assign1880_e1294, assign1880_e1294_d_n0, assign1880_e1294_d_n2, assign1880_e1294_d_n6, assign1880_e1294_d_n7, assign1880_e1294_d_n10, assign1880_e1294_d_n11, assign1880_e1294_d_n12, assign1880_e1294_d_n17,) = {
    if (var_guard4 != 0.0) {
        let assign1880_e1292: f64 = (1.0 / var_dnm);
        (assign1880_e1292, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn11 / (var_dnm * var_dnm))), (-(var_dnm_dn12 / (var_dnm * var_dnm))), (-(var_dnm_dn17 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign1880_e1294;
        var_dnm_dn0 = assign1880_e1294_d_n0;
        var_dnm_dn2 = assign1880_e1294_d_n2;
        var_dnm_dn6 = assign1880_e1294_d_n6;
        var_dnm_dn7 = assign1880_e1294_d_n7;
        var_dnm_dn10 = assign1880_e1294_d_n10;
        var_dnm_dn11 = assign1880_e1294_d_n11;
        var_dnm_dn12 = assign1880_e1294_d_n12;
        var_dnm_dn17 = assign1880_e1294_d_n17;

        let (assign1890_e1302, assign1890_e1302_d_n0, assign1890_e1302_d_n2, assign1890_e1302_d_n6, assign1890_e1302_d_n7, assign1890_e1302_d_n10, assign1890_e1302_d_n11, assign1890_e1302_d_n12, assign1890_e1302_d_n17,) = {
    if (var_guard4 != 0.0) {
        let assign1890_e1298: f64 = (var_tmf1 * 0.1);
        let assign1890_e1300: f64 = (assign1890_e1298 * var_dnm);
        (assign1890_e1300, (((var_tmf1_dn0 * 0.1) * var_dnm) + (assign1890_e1298 * var_dnm_dn0)), (((var_tmf1_dn2 * 0.1) * var_dnm) + (assign1890_e1298 * var_dnm_dn2)), (((var_tmf1_dn6 * 0.1) * var_dnm) + (assign1890_e1298 * var_dnm_dn6)), (((var_tmf1_dn7 * 0.1) * var_dnm) + (assign1890_e1298 * var_dnm_dn7)), (((var_tmf1_dn10 * 0.1) * var_dnm) + (assign1890_e1298 * var_dnm_dn10)), (((var_tmf1_dn11 * 0.1) * var_dnm) + (assign1890_e1298 * var_dnm_dn11)), (((var_tmf1_dn12 * 0.1) * var_dnm) + (assign1890_e1298 * var_dnm_dn12)), (((var_tmf1_dn17 * 0.1) * var_dnm) + (assign1890_e1298 * var_dnm_dn17)),)
    } else {
        (var_tmf0, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn10, var_tmf0_dn11, var_tmf0_dn12, var_tmf0_dn17,)
    }
};
        var_tmf0 = assign1890_e1302;
        var_tmf0_dn0 = assign1890_e1302_d_n0;
        var_tmf0_dn2 = assign1890_e1302_d_n2;
        var_tmf0_dn6 = assign1890_e1302_d_n6;
        var_tmf0_dn7 = assign1890_e1302_d_n7;
        var_tmf0_dn10 = assign1890_e1302_d_n10;
        var_tmf0_dn11 = assign1890_e1302_d_n11;
        var_tmf0_dn12 = assign1890_e1302_d_n12;
        var_tmf0_dn17 = assign1890_e1302_d_n17;

        let (assign1900_e1310, assign1900_e1310_d_n0, assign1900_e1310_d_n2, assign1900_e1310_d_n6, assign1900_e1310_d_n7, assign1900_e1310_d_n10, assign1900_e1310_d_n11, assign1900_e1310_d_n12, assign1900_e1310_d_n17,) = {
    if (var_guard4 != 0.0) {
        let assign1900_e1306: f64 = (2.0 + 0.1);
        let assign1900_e1308: f64 = (assign1900_e1306 - var_tmf0);
        (assign1900_e1308, (-var_tmf0_dn0), (-var_tmf0_dn2), (-var_tmf0_dn6), (-var_tmf0_dn7), (-var_tmf0_dn10), (-var_tmf0_dn11), (-var_tmf0_dn12), (-var_tmf0_dn17),)
    } else {
        (var_uc_clm2, var_uc_clm2_dn0, var_uc_clm2_dn2, var_uc_clm2_dn6, var_uc_clm2_dn7, var_uc_clm2_dn10, var_uc_clm2_dn11, var_uc_clm2_dn12, var_uc_clm2_dn17,)
    }
};
        var_uc_clm2 = assign1900_e1310;
        var_uc_clm2_dn0 = assign1900_e1310_d_n0;
        var_uc_clm2_dn2 = assign1900_e1310_d_n2;
        var_uc_clm2_dn6 = assign1900_e1310_d_n6;
        var_uc_clm2_dn7 = assign1900_e1310_d_n7;
        var_uc_clm2_dn10 = assign1900_e1310_d_n10;
        var_uc_clm2_dn11 = assign1900_e1310_d_n11;
        var_uc_clm2_dn12 = assign1900_e1310_d_n12;
        var_uc_clm2_dn17 = assign1900_e1310_d_n17;

        let (assign1910_e1315, assign1910_e1315_d_n0, assign1910_e1315_d_n2, assign1910_e1315_d_n6, assign1910_e1315_d_n7, assign1910_e1315_d_n10, assign1910_e1315_d_n11, assign1910_e1315_d_n12, assign1910_e1315_d_n17,) = {
    if (var_guard4 == 0.0) {
        (var_uc_clm2, var_uc_clm2_dn0, var_uc_clm2_dn2, var_uc_clm2_dn6, var_uc_clm2_dn7, var_uc_clm2_dn10, var_uc_clm2_dn11, var_uc_clm2_dn12, var_uc_clm2_dn17,)
    } else {
        (var_uc_clm2, var_uc_clm2_dn0, var_uc_clm2_dn2, var_uc_clm2_dn6, var_uc_clm2_dn7, var_uc_clm2_dn10, var_uc_clm2_dn11, var_uc_clm2_dn12, var_uc_clm2_dn17,)
    }
};
        var_uc_clm2 = assign1910_e1315;
        var_uc_clm2_dn0 = assign1910_e1315_d_n0;
        var_uc_clm2_dn2 = assign1910_e1315_d_n2;
        var_uc_clm2_dn6 = assign1910_e1315_d_n6;
        var_uc_clm2_dn7 = assign1910_e1315_d_n7;
        var_uc_clm2_dn10 = assign1910_e1315_d_n10;
        var_uc_clm2_dn11 = assign1910_e1315_d_n11;
        var_uc_clm2_dn12 = assign1910_e1315_d_n12;
        var_uc_clm2_dn17 = assign1910_e1315_d_n17;

        let assign1920_e1321: f64 = (var_uc_tnom * 1e-7);
        let assign1920_e1322: f64 = (9.025e-5 + assign1920_e1321);
        let assign1920_e1323: f64 = (var_uc_tnom * assign1920_e1322);
        let assign1920_e1324: f64 = (p.p55 - assign1920_e1323);
        var_egtnom = assign1920_e1324;

        var_tfox0 = p.p236;

        let assign1940_e1328: f64 = (1.034943e-10 / p.p237);
        var_c_soi = assign1940_e1328;

        let assign1950_e1331: f64 = (1.0 / var_c_soi);
        var_c_soi_inv = assign1950_e1331;

        let assign1960_e1334: f64 = (3.453133e-11 / var_tfox0);
        var_c_fox0 = assign1960_e1334;

        let assign1970_e1337: f64 = (var_tfox0 / 3.453133e-11);
        var_c_fox0_inv = assign1970_e1337;

        let assign1980_e1340: f64 = (3.453133e-11 / p.p239);
        var_c_box = assign1980_e1340;

        let assign1990_e1343: f64 = (p.p239 / 3.453133e-11);
        var_c_box_inv = assign1990_e1343;

        let assign2000_e1346: f64 = (var_c_box_inv + var_c_soi_inv);
        var_c_box_fd_inv = assign2000_e1346;

        var_lgate = p.p0;

        let assign2020_e1351: f64 = (2.0 * p.p56);
        let assign2020_e1352: f64 = (var_lgate - assign2020_e1351);
        var_leff = assign2020_e1352;

        let assign2030_e1356: f64 = (2.0 * p.p57);
        let assign2030_e1357: f64 = (var_lgate - assign2030_e1356);
        var_leff_cv = assign2030_e1357;

        let (assign2040_e1363,) = {
    if (p.p40 == 0.0) {
        (var_lgate,)
    } else {
        (var_leff,)
    }
};
        var_lgleff = assign2040_e1363;

        let assign2050_e1366: f64 = (var_lgleff * 1000000.0);
        var_lgle = assign2050_e1366;

        let assign2060_e1369: f64 = (p.p1 / p.p9);
        var_wgate = assign2060_e1369;

        var_dw = p.p60;

        let (assign2080_e1376,) = {
    if (var_subversion < 1.0) {
        (0.0,)
    } else {
        (p.p295,)
    }
};
        var_dwbt = assign2080_e1376;

        let (assign2090_e1382,) = {
    if (var_subversion < 1.0) {
        (p.p60,)
    } else {
        (p.p61,)
    }
};
        var_dwcv = assign2090_e1382;

        let assign2100_e1385: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        var_guard10 = assign2100_e1385;

        let (assign2110_e1393,) = {
    if (var_guard10 != 0.0) {
        let assign2110_e1390: f64 = (2.0 * var_dw);
        let assign2110_e1391: f64 = (var_wgate - assign2110_e1390);
        (assign2110_e1391,)
    } else {
        (var_weff,)
    }
};
        var_weff = assign2110_e1393;

        let (assign2120_e1401,) = {
    if (var_guard10 != 0.0) {
        let assign2120_e1398: f64 = (2.0 * var_dwcv);
        let assign2120_e1399: f64 = (var_wgate - assign2120_e1398);
        (assign2120_e1399,)
    } else {
        (var_weff_cv,)
    }
};
        var_weff_cv = assign2120_e1401;

        let (assign2130_e1416,) = {
    if (var_guard10 == 0.0) {
        let assign2130_e1407: f64 = (p.p18 * var_dwbt);
        let assign2130_e1408: f64 = (var_wgate - assign2130_e1407);
        let assign2130_e1411: f64 = (2.0 - p.p18);
        let assign2130_e1413: f64 = (assign2130_e1411 * var_dw);
        let assign2130_e1414: f64 = (assign2130_e1408 - assign2130_e1413);
        (assign2130_e1414,)
    } else {
        (var_weff,)
    }
};
        var_weff = assign2130_e1416;

        let (assign2140_e1431,) = {
    if (var_guard10 == 0.0) {
        let assign2140_e1422: f64 = (p.p18 * var_dwbt);
        let assign2140_e1423: f64 = (var_wgate - assign2140_e1422);
        let assign2140_e1426: f64 = (2.0 - p.p18);
        let assign2140_e1428: f64 = (assign2140_e1426 * var_dwcv);
        let assign2140_e1429: f64 = (assign2140_e1423 - assign2140_e1428);
        (assign2140_e1429,)
    } else {
        (var_weff_cv,)
    }
};
        var_weff_cv = assign2140_e1431;

        let assign2150_e1434: f64 = (var_weff * p.p9);
        var_weff_nf = assign2150_e1434;

        let assign2160_e1437: f64 = (var_weff_cv * p.p9);
        var_weffcv_nf = assign2160_e1437;

        let assign2170_e1440: f64 = (var_wgate * 1000000.0);
        var_wg = assign2170_e1440;

        let assign2180_e1443: f64 = (var_wg * var_lgle);
        var_wl = assign2180_e1443;

        let assign2190_e1449: f64 = (var_lgle).powf(p.p111);
        let assign2190_e1450: f64 = (p.p108 / assign2190_e1449);
        let assign2190_e1451: f64 = (1.0 + assign2190_e1450);
        let assign2190_e1452: f64 = (p.p107 * assign2190_e1451);
        let assign2190_e1457: f64 = (var_wg).powf(p.p110);
        let assign2190_e1458: f64 = (p.p109 / assign2190_e1457);
        let assign2190_e1459: f64 = (1.0 + assign2190_e1458);
        let assign2190_e1460: f64 = (assign2190_e1452 * assign2190_e1459);
        var_muesr = assign2190_e1460;

        let assign2200_e1471: f64 = if (((var_subversion > 3.0) && (var_mks_nsubp < var_mks_nsubs)) && (p.p72 > 0.0)) { 1.0 } else { 0.0 };
        var_guard11 = assign2200_e1471;

        let (assign2210_e1475,) = {
    if (var_guard11 != 0.0) {
        (var_mks_nsubs,)
    } else {
        (var_mks_nsubp,)
    }
};
        var_mks_nsubp = assign2210_e1475;

        let assign2220_e1481: f64 = (var_wg).powf(p.p75);
        let assign2220_e1482: f64 = (p.p74 / assign2220_e1481);
        let assign2220_e1483: f64 = (1.0 + assign2220_e1482);
        let assign2220_e1484: f64 = (var_mks_nsubp * assign2220_e1483);
        var_nsubpp = assign2220_e1484;

        let assign2230_e1490: f64 = (0.5 * var_lgate);
        let assign2230_e1491: f64 = (p.p62 + assign2230_e1490);
        let assign2230_e1492: f64 = (1.0 / assign2230_e1491);
        let assign2230_e1497: f64 = (0.5 * var_lgate);
        let assign2230_e1498: f64 = (p.p63 + assign2230_e1497);
        let assign2230_e1499: f64 = (1.0 / assign2230_e1498);
        let assign2230_e1500: f64 = (assign2230_e1492 + assign2230_e1499);
        let assign2230_e1501: f64 = (2.0 / assign2230_e1500);
        var_lod_half_ref = assign2230_e1501;

        let assign2240_e1505: f64 = (1.3806226e-23 * var_uc_tnom);
        let assign2240_e1506: f64 = (1.6021918e-19 / assign2240_e1505);
        var_betatnom = assign2240_e1506;

        let assign2250_e1509: f64 = (1.6021918e-19 * var_mks_nsubb);
        let assign2250_e1511: f64 = (assign2250_e1509 * 1.034943e-10);
        var_qnbulk_esi = assign2250_e1511;

        let assign2260_e1515: f64 = (-p.p247);
        let assign2260_e1516: f64 = (var_lgle).powf(assign2260_e1515);
        let assign2260_e1517: f64 = (p.p244 * assign2260_e1516);
        var_ptl0 = assign2260_e1517;

        let assign2270_e1521: f64 = (-p.p252);
        let assign2270_e1522: f64 = (var_lgle).powf(assign2270_e1521);
        let assign2270_e1523: f64 = (p.p251 * assign2270_e1522);
        var_pt40 = assign2270_e1523;

        let assign2280_e1527: f64 = (var_lgle + var_uc_gdld);
        let assign2280_e1529: f64 = (-p.p249);
        let assign2280_e1530: f64 = (assign2280_e1527).powf(assign2280_e1529);
        let assign2280_e1531: f64 = (p.p248 * assign2280_e1530);
        var_gdl0 = assign2280_e1531;

        let assign2290_e1534: f64 = (2.0 * 1.6021918e-19);
        let assign2290_e1536: f64 = (assign2290_e1534 * var_mks_nsti);
        let assign2290_e1538: f64 = (assign2290_e1536 * 1.034943e-10);
        let assign2290_e1539: f64 = (assign2290_e1538).sqrt();
        var_costi00 = assign2290_e1539;

        let assign2300_e1543: f64 = (var_mks_nsti * var_mks_nsti);
        let assign2300_e1544: f64 = (1.0 / assign2300_e1543);
        var_nsti_p2 = assign2300_e1544;

        let assign2310_e1548: f64 = (1.0 / var_lgle);
        let assign2310_e1549: f64 = (1.0 + assign2310_e1548);
        let assign2310_e1551: f64 = (assign2310_e1549).powf(p.p91);
        let assign2310_e1553: f64 = (assign2310_e1551 * p.p89);
        var_cnstpgd = assign2310_e1553;

        var_c0bulk = var_qnbulk_esi;

        var_vfb = p.p68;

        let assign2340_e1560: f64 = (var_wl).powf(p.p77);
        let assign2340_e1561: f64 = (p.p76 / assign2340_e1560);
        let assign2340_e1562: f64 = (var_lgleff + assign2340_e1561);
        var_lgatesm = assign2340_e1562;

        let assign2350_e1566: f64 = (var_wl).powf(p.p79);
        let assign2350_e1567: f64 = (p.p78 / assign2350_e1566);
        var_dvthsm = assign2350_e1567;

        let assign2360_e1573: f64 = (var_lgatesm * 1000000.0);
        let assign2360_e1575: f64 = (assign2360_e1573).powf(p.p151);
        let assign2360_e1576: f64 = (p.p150 / assign2360_e1575);
        let assign2360_e1577: f64 = (1.0 + assign2360_e1576);
        let assign2360_e1578: f64 = (p.p149 * assign2360_e1577);
        let assign2360_e1580: f64 = assign2360_e1578;
        let assign2360_e1584: f64 = (var_wg).powf(p.p153);
        let assign2360_e1585: f64 = (p.p152 / assign2360_e1584);
        let assign2360_e1586: f64 = (assign2360_e1580 + assign2360_e1585);
        var_uc_wsti = assign2360_e1586;

        let assign2370_e1590: f64 = (var_lgle).powf(p.p192);
        let assign2370_e1592: f64 = (assign2370_e1590 * p.p193);
        let assign2370_e1593: f64 = (1.0 + assign2370_e1592);
        var_clmmod = assign2370_e1593;

        let assign2380_e1599: f64 = (3.0 * p.p6);
        let assign2380_e1600: f64 = (var_weff / assign2380_e1599);
        let assign2380_e1601: f64 = (p.p7 + assign2380_e1600);
        let assign2380_e1602: f64 = (p.p67 * assign2380_e1601);
        let assign2380_e1606: f64 = (var_lgate - p.p8);
        let assign2380_e1607: f64 = (p.p6 * assign2380_e1606);
        let assign2380_e1609: f64 = (assign2380_e1607 * p.p9);
        let assign2380_e1610: f64 = (assign2380_e1602 / assign2380_e1609);
        var_grg_cnst = assign2380_e1610;

        let assign2390_e1613: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        var_guard12 = assign2390_e1613;

        let (assign2400_e1623,) = {
    if (var_guard12 != 0.0) {
        let assign2400_e1619: f64 = (var_wg).powf(p.p131);
        let assign2400_e1620: f64 = (p.p130 / assign2400_e1619);
        let assign2400_e1621: f64 = (1.0 + assign2400_e1620);
        (assign2400_e1621,)
    } else {
        (var_zvgs,)
    }
};
        var_zvgs = assign2400_e1623;

        let (assign2410_e1635,) = {
    if (var_guard12 != 0.0) {
        let assign2410_e1630: f64 = (var_lgle).powf(p.p126);
        let assign2410_e1631: f64 = (p.p125 / assign2410_e1630);
        let assign2410_e1632: f64 = (1.0 + assign2410_e1631);
        let assign2410_e1633: f64 = (p.p124 * assign2410_e1632);
        (assign2410_e1633,)
    } else {
        (var_xvbs,)
    }
};
        var_xvbs = assign2410_e1635;

        let (assign2420_e1643,) = {
    if (var_guard12 != 0.0) {
        let assign2420_e1640: f64 = (var_lgle + p.p123);
        let assign2420_e1641: f64 = (var_lgle / assign2420_e1640);
        (assign2420_e1641,)
    } else {
        (var_xgate,)
    }
};
        var_xgate = assign2420_e1643;

        let (assign2430_e1655,) = {
    if (var_guard12 != 0.0) {
        let assign2430_e1650: f64 = (var_lgle).powf(p.p120);
        let assign2430_e1651: f64 = (p.p119 / assign2430_e1650);
        let assign2430_e1652: f64 = (1.0 + assign2430_e1651);
        let assign2430_e1653: f64 = (p.p117 * assign2430_e1652);
        (assign2430_e1653,)
    } else {
        (var_xsub1,)
    }
};
        var_xsub1 = assign2430_e1655;

        let (assign2440_e1665,) = {
    if (var_guard12 != 0.0) {
        let assign2440_e1661: f64 = (p.p121 / var_lgle);
        let assign2440_e1662: f64 = (1.0 + assign2440_e1661);
        let assign2440_e1663: f64 = (p.p118 * assign2440_e1662);
        (assign2440_e1663,)
    } else {
        (var_xsub2,)
    }
};
        var_xsub2 = assign2440_e1665;

        *var_betatnom_slot = var_betatnom;
        *var_c0bulk_slot = var_c0bulk;
        *var_c_box_slot = var_c_box;
        *var_c_box_fd_inv_slot = var_c_box_fd_inv;
        *var_c_box_inv_slot = var_c_box_inv;
        *var_c_fox0_slot = var_c_fox0;
        *var_c_fox0_inv_slot = var_c_fox0_inv;
        *var_c_soi_slot = var_c_soi;
        *var_c_soi_inv_slot = var_c_soi_inv;
        *var_clmmod_slot = var_clmmod;
        *var_cnstpgd_slot = var_cnstpgd;
        *var_costi00_slot = var_costi00;
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn11_slot = var_dnm_dn11;
        *var_dnm_dn12_slot = var_dnm_dn12;
        *var_dnm_dn17_slot = var_dnm_dn17;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_dvthsm_slot = var_dvthsm;
        *var_dw_slot = var_dw;
        *var_dwbt_slot = var_dwbt;
        *var_dwcv_slot = var_dwcv;
        *var_egtnom_slot = var_egtnom;
        *var_gdl0_slot = var_gdl0;
        *var_grg_cnst_slot = var_grg_cnst;
        *var_guard10_slot = var_guard10;
        *var_guard11_slot = var_guard11;
        *var_guard12_slot = var_guard12;
        *var_guard7_slot = var_guard7;
        *var_guard8_slot = var_guard8;
        *var_guard9_slot = var_guard9;
        *var_leff_slot = var_leff;
        *var_leff_cv_slot = var_leff_cv;
        *var_lgate_slot = var_lgate;
        *var_lgatesm_slot = var_lgatesm;
        *var_lgle_slot = var_lgle;
        *var_lgleff_slot = var_lgleff;
        *var_lod_half_ref_slot = var_lod_half_ref;
        *var_m0_slot = var_m0;
        *var_mks_nsubp_slot = var_mks_nsubp;
        *var_mm_slot = var_mm;
        *var_muesr_slot = var_muesr;
        *var_nsti_p2_slot = var_nsti_p2;
        *var_nsubpp_slot = var_nsubpp;
        *var_pt40_slot = var_pt40;
        *var_ptl0_slot = var_ptl0;
        *var_qnbulk_esi_slot = var_qnbulk_esi;
        *var_tfox0_slot = var_tfox0;
        *var_tmf0_slot = var_tmf0;
        *var_tmf0_dn0_slot = var_tmf0_dn0;
        *var_tmf0_dn10_slot = var_tmf0_dn10;
        *var_tmf0_dn11_slot = var_tmf0_dn11;
        *var_tmf0_dn12_slot = var_tmf0_dn12;
        *var_tmf0_dn17_slot = var_tmf0_dn17;
        *var_tmf0_dn2_slot = var_tmf0_dn2;
        *var_tmf0_dn6_slot = var_tmf0_dn6;
        *var_tmf0_dn7_slot = var_tmf0_dn7;
        *var_uc_clm2_slot = var_uc_clm2;
        *var_uc_clm2_dn0_slot = var_uc_clm2_dn0;
        *var_uc_clm2_dn10_slot = var_uc_clm2_dn10;
        *var_uc_clm2_dn11_slot = var_uc_clm2_dn11;
        *var_uc_clm2_dn12_slot = var_uc_clm2_dn12;
        *var_uc_clm2_dn17_slot = var_uc_clm2_dn17;
        *var_uc_clm2_dn2_slot = var_uc_clm2_dn2;
        *var_uc_clm2_dn6_slot = var_uc_clm2_dn6;
        *var_uc_clm2_dn7_slot = var_uc_clm2_dn7;
        *var_uc_wsti_slot = var_uc_wsti;
        *var_vfb_slot = var_vfb;
        *var_weff_slot = var_weff;
        *var_weff_cv_slot = var_weff_cv;
        *var_weff_nf_slot = var_weff_nf;
        *var_weffcv_nf_slot = var_weffcv_nf;
        *var_wg_slot = var_wg;
        *var_wgate_slot = var_wgate;
        *var_wl_slot = var_wl;
        *var_xgate_slot = var_xgate;
        *var_xsub1_slot = var_xsub1;
        *var_xsub2_slot = var_xsub2;
        *var_xvbs_slot = var_xvbs;
        *var_zvgs_slot = var_zvgs;
    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard12: f64,
        var_lgate: f64,
        var_lgle: f64,
        var_lod_half_ref: f64,
        var_mks_cth0: f64,
        var_mks_nsubcmax: f64,
        var_mks_nsubs: f64,
        var_mks_rth0: f64,
        var_nsubpp: f64,
        var_weff_nf: f64,
        var_weffcv_nf: f64,
        var_wg: f64,
        var_abtn_given_slot: &mut f64,
        var_abtp_given_slot: &mut f64,
        var_cbtbn_given_slot: &mut f64,
        var_cbtbp_given_slot: &mut f64,
        var_cgbo_given_slot: &mut f64,
        var_cgdo_given_slot: &mut f64,
        var_cgso_given_slot: &mut f64,
        var_cqyb0_slot: &mut f64,
        var_cth_slot: &mut f64,
        var_ddlte_slot: &mut f64,
        var_dtemp_given_slot: &mut f64,
        var_gjmin_slot: &mut f64,
        var_guard13_slot: &mut f64,
        var_guard14_slot: &mut f64,
        var_guard15_slot: &mut f64,
        var_guard16_slot: &mut f64,
        var_i_slot: &mut f64,
        var_lod_half_slot: &mut f64,
        var_lod_half_dn0_slot: &mut f64,
        var_lod_half_dn10_slot: &mut f64,
        var_lod_half_dn11_slot: &mut f64,
        var_lod_half_dn12_slot: &mut f64,
        var_lod_half_dn17_slot: &mut f64,
        var_lod_half_dn2_slot: &mut f64,
        var_lod_half_dn6_slot: &mut f64,
        var_lod_half_dn7_slot: &mut f64,
        var_mfactor_slot: &mut f64,
        var_nsubps_slot: &mut f64,
        var_nsubps_dn0_slot: &mut f64,
        var_nsubps_dn10_slot: &mut f64,
        var_nsubps_dn11_slot: &mut f64,
        var_nsubps_dn12_slot: &mut f64,
        var_nsubps_dn17_slot: &mut f64,
        var_nsubps_dn2_slot: &mut f64,
        var_nsubps_dn6_slot: &mut f64,
        var_nsubps_dn7_slot: &mut f64,
        var_pdbcp_given_slot: &mut f64,
        var_psbcp_given_slot: &mut f64,
        var_rth_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn17_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_temp_given_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn17_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_uc_pdbcp_slot: &mut f64,
        var_uc_psbcp_slot: &mut f64,
        var_uc_svgs_slot: &mut f64,
        var_uc_temp_slot: &mut f64,
        var_vfbsub0_slot: &mut f64,
        var_vg2const_slot: &mut f64,
        var_vg2const_dn0_slot: &mut f64,
        var_vg2const_dn10_slot: &mut f64,
        var_vg2const_dn11_slot: &mut f64,
        var_vg2const_dn12_slot: &mut f64,
        var_vg2const_dn17_slot: &mut f64,
        var_vg2const_dn2_slot: &mut f64,
        var_vg2const_dn6_slot: &mut f64,
        var_vg2const_dn7_slot: &mut f64,
        var_vgs_min_slot: &mut f64,
        var_xgate_slot: &mut f64,
        var_xsub1_slot: &mut f64,
        var_xsub2_slot: &mut f64,
        var_xvbs_slot: &mut f64,
    ) {
        let mut var_abtn_given: f64 = *var_abtn_given_slot;
        let mut var_abtp_given: f64 = *var_abtp_given_slot;
        let mut var_cbtbn_given: f64 = *var_cbtbn_given_slot;
        let mut var_cbtbp_given: f64 = *var_cbtbp_given_slot;
        let mut var_cgbo_given: f64 = *var_cgbo_given_slot;
        let mut var_cgdo_given: f64 = *var_cgdo_given_slot;
        let mut var_cgso_given: f64 = *var_cgso_given_slot;
        let mut var_cqyb0: f64 = *var_cqyb0_slot;
        let mut var_cth: f64 = *var_cth_slot;
        let mut var_ddlte: f64 = *var_ddlte_slot;
        let mut var_dtemp_given: f64 = *var_dtemp_given_slot;
        let mut var_gjmin: f64 = *var_gjmin_slot;
        let mut var_guard13: f64 = *var_guard13_slot;
        let mut var_guard14: f64 = *var_guard14_slot;
        let mut var_guard15: f64 = *var_guard15_slot;
        let mut var_guard16: f64 = *var_guard16_slot;
        let mut var_i: f64 = *var_i_slot;
        let mut var_lod_half: f64 = *var_lod_half_slot;
        let mut var_lod_half_dn0: f64 = *var_lod_half_dn0_slot;
        let mut var_lod_half_dn10: f64 = *var_lod_half_dn10_slot;
        let mut var_lod_half_dn11: f64 = *var_lod_half_dn11_slot;
        let mut var_lod_half_dn12: f64 = *var_lod_half_dn12_slot;
        let mut var_lod_half_dn17: f64 = *var_lod_half_dn17_slot;
        let mut var_lod_half_dn2: f64 = *var_lod_half_dn2_slot;
        let mut var_lod_half_dn6: f64 = *var_lod_half_dn6_slot;
        let mut var_lod_half_dn7: f64 = *var_lod_half_dn7_slot;
        let mut var_mfactor: f64 = *var_mfactor_slot;
        let mut var_nsubps: f64 = *var_nsubps_slot;
        let mut var_nsubps_dn0: f64 = *var_nsubps_dn0_slot;
        let mut var_nsubps_dn10: f64 = *var_nsubps_dn10_slot;
        let mut var_nsubps_dn11: f64 = *var_nsubps_dn11_slot;
        let mut var_nsubps_dn12: f64 = *var_nsubps_dn12_slot;
        let mut var_nsubps_dn17: f64 = *var_nsubps_dn17_slot;
        let mut var_nsubps_dn2: f64 = *var_nsubps_dn2_slot;
        let mut var_nsubps_dn6: f64 = *var_nsubps_dn6_slot;
        let mut var_nsubps_dn7: f64 = *var_nsubps_dn7_slot;
        let mut var_pdbcp_given: f64 = *var_pdbcp_given_slot;
        let mut var_psbcp_given: f64 = *var_psbcp_given_slot;
        let mut var_rth: f64 = *var_rth_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn17: f64 = *var_t3_dn17_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_temp_given: f64 = *var_temp_given_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn17: f64 = *var_tmf2_dn17_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_uc_pdbcp: f64 = *var_uc_pdbcp_slot;
        let mut var_uc_psbcp: f64 = *var_uc_psbcp_slot;
        let mut var_uc_svgs: f64 = *var_uc_svgs_slot;
        let mut var_uc_temp: f64 = *var_uc_temp_slot;
        let mut var_vfbsub0: f64 = *var_vfbsub0_slot;
        let mut var_vg2const: f64 = *var_vg2const_slot;
        let mut var_vg2const_dn0: f64 = *var_vg2const_dn0_slot;
        let mut var_vg2const_dn10: f64 = *var_vg2const_dn10_slot;
        let mut var_vg2const_dn11: f64 = *var_vg2const_dn11_slot;
        let mut var_vg2const_dn12: f64 = *var_vg2const_dn12_slot;
        let mut var_vg2const_dn17: f64 = *var_vg2const_dn17_slot;
        let mut var_vg2const_dn2: f64 = *var_vg2const_dn2_slot;
        let mut var_vg2const_dn6: f64 = *var_vg2const_dn6_slot;
        let mut var_vg2const_dn7: f64 = *var_vg2const_dn7_slot;
        let mut var_vgs_min: f64 = *var_vgs_min_slot;
        let mut var_xgate: f64 = *var_xgate_slot;
        let mut var_xsub1: f64 = *var_xsub1_slot;
        let mut var_xsub2: f64 = *var_xsub2_slot;
        let mut var_xvbs: f64 = *var_xvbs_slot;

        let (assign2450_e1672, assign2450_e1672_d_n0, assign2450_e1672_d_n2, assign2450_e1672_d_n6, assign2450_e1672_d_n7, assign2450_e1672_d_n10, assign2450_e1672_d_n11, assign2450_e1672_d_n12, assign2450_e1672_d_n17,) = {
    if (var_guard12 == 0.0) {
        let assign2450_e1670: f64 = (var_wg).powf(p.p131);
        (assign2450_e1670, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign2450_e1672;
        var_t2_dn0 = assign2450_e1672_d_n0;
        var_t2_dn2 = assign2450_e1672_d_n2;
        var_t2_dn6 = assign2450_e1672_d_n6;
        var_t2_dn7 = assign2450_e1672_d_n7;
        var_t2_dn10 = assign2450_e1672_d_n10;
        var_t2_dn11 = assign2450_e1672_d_n11;
        var_t2_dn12 = assign2450_e1672_d_n12;
        var_t2_dn17 = assign2450_e1672_d_n17;

        let (assign2460_e1691, assign2460_e1691_d_n0, assign2460_e1691_d_n2, assign2460_e1691_d_n6, assign2460_e1691_d_n7, assign2460_e1691_d_n10, assign2460_e1691_d_n11, assign2460_e1691_d_n12, assign2460_e1691_d_n17,) = {
    if (var_guard12 == 0.0) {
        let assign2460_e1680: f64 = (var_lgle).powf(p.p129);
        let assign2460_e1681: f64 = (p.p128 / assign2460_e1680);
        let assign2460_e1682: f64 = (1.0 + assign2460_e1681);
        let assign2460_e1683: f64 = (p.p127 * assign2460_e1682);
        let assign2460_e1687: f64 = (var_t2 + p.p130);
        let assign2460_e1688: f64 = (var_t2 / assign2460_e1687);
        let assign2460_e1689: f64 = (assign2460_e1683 * assign2460_e1688);
        (assign2460_e1689, (assign2460_e1683 * (((var_t2_dn0 * assign2460_e1687) - (var_t2 * var_t2_dn0)) / (assign2460_e1687 * assign2460_e1687))), (assign2460_e1683 * (((var_t2_dn2 * assign2460_e1687) - (var_t2 * var_t2_dn2)) / (assign2460_e1687 * assign2460_e1687))), (assign2460_e1683 * (((var_t2_dn6 * assign2460_e1687) - (var_t2 * var_t2_dn6)) / (assign2460_e1687 * assign2460_e1687))), (assign2460_e1683 * (((var_t2_dn7 * assign2460_e1687) - (var_t2 * var_t2_dn7)) / (assign2460_e1687 * assign2460_e1687))), (assign2460_e1683 * (((var_t2_dn10 * assign2460_e1687) - (var_t2 * var_t2_dn10)) / (assign2460_e1687 * assign2460_e1687))), (assign2460_e1683 * (((var_t2_dn11 * assign2460_e1687) - (var_t2 * var_t2_dn11)) / (assign2460_e1687 * assign2460_e1687))), (assign2460_e1683 * (((var_t2_dn12 * assign2460_e1687) - (var_t2 * var_t2_dn12)) / (assign2460_e1687 * assign2460_e1687))), (assign2460_e1683 * (((var_t2_dn17 * assign2460_e1687) - (var_t2 * var_t2_dn17)) / (assign2460_e1687 * assign2460_e1687))),)
    } else {
        (var_vg2const, var_vg2const_dn0, var_vg2const_dn2, var_vg2const_dn6, var_vg2const_dn7, var_vg2const_dn10, var_vg2const_dn11, var_vg2const_dn12, var_vg2const_dn17,)
    }
};
        var_vg2const = assign2460_e1691;
        var_vg2const_dn0 = assign2460_e1691_d_n0;
        var_vg2const_dn2 = assign2460_e1691_d_n2;
        var_vg2const_dn6 = assign2460_e1691_d_n6;
        var_vg2const_dn7 = assign2460_e1691_d_n7;
        var_vg2const_dn10 = assign2460_e1691_d_n10;
        var_vg2const_dn11 = assign2460_e1691_d_n11;
        var_vg2const_dn12 = assign2460_e1691_d_n12;
        var_vg2const_dn17 = assign2460_e1691_d_n17;

        let (assign2470_e1704,) = {
    if (var_guard12 == 0.0) {
        let assign2470_e1699: f64 = (var_lgle).powf(p.p126);
        let assign2470_e1700: f64 = (p.p125 / assign2470_e1699);
        let assign2470_e1701: f64 = (1.0 + assign2470_e1700);
        let assign2470_e1702: f64 = (p.p124 * assign2470_e1701);
        (assign2470_e1702,)
    } else {
        (var_xvbs,)
    }
};
        var_xvbs = assign2470_e1704;

        let (assign2480_e1717,) = {
    if (var_guard12 == 0.0) {
        let assign2480_e1712: f64 = (var_lgle).powf(p.p133);
        let assign2480_e1713: f64 = (p.p132 / assign2480_e1712);
        let assign2480_e1714: f64 = (1.0 + assign2480_e1713);
        let assign2480_e1715: f64 = (p.p123 * assign2480_e1714);
        (assign2480_e1715,)
    } else {
        (var_xgate,)
    }
};
        var_xgate = assign2480_e1717;

        let (assign2490_e1730,) = {
    if (var_guard12 == 0.0) {
        let assign2490_e1725: f64 = (var_lgle).powf(p.p120);
        let assign2490_e1726: f64 = (p.p119 / assign2490_e1725);
        let assign2490_e1727: f64 = (1.0 + assign2490_e1726);
        let assign2490_e1728: f64 = (p.p117 * assign2490_e1727);
        (assign2490_e1728,)
    } else {
        (var_xsub1,)
    }
};
        var_xsub1 = assign2490_e1730;

        let (assign2500_e1741,) = {
    if (var_guard12 == 0.0) {
        let assign2500_e1737: f64 = (p.p121 / var_lgle);
        let assign2500_e1738: f64 = (1.0 + assign2500_e1737);
        let assign2500_e1739: f64 = (p.p118 * assign2500_e1738);
        (assign2500_e1739,)
    } else {
        (var_xsub2,)
    }
};
        var_xsub2 = assign2500_e1741;

        let assign2510_e1744: f64 = (1000000.0 * var_weffcv_nf);
        let assign2510_e1746: f64 = (assign2510_e1744 * p.p65);
        let assign2510_e1749: f64 = (var_lgle).powf(p.p66);
        let assign2510_e1750: f64 = (assign2510_e1746 / assign2510_e1749);
        var_cqyb0 = assign2510_e1750;

        let assign2520_e1756: f64 = (var_lgle).powf(p.p136);
        let assign2520_e1757: f64 = (p.p135 / assign2520_e1756);
        let assign2520_e1758: f64 = (1.0 + assign2520_e1757);
        let assign2520_e1759: f64 = (p.p134 * assign2520_e1758);
        var_vfbsub0 = assign2520_e1759;

        let assign2530_e1762: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        var_guard13 = assign2530_e1762;

        let (assign2540_e1774,) = {
    if (var_guard13 != 0.0) {
        let assign2540_e1769: f64 = (var_lgle).powf(p.p129);
        let assign2540_e1770: f64 = (p.p128 / assign2540_e1769);
        let assign2540_e1771: f64 = (1.0 + assign2540_e1770);
        let assign2540_e1772: f64 = (p.p127 * assign2540_e1771);
        (assign2540_e1772,)
    } else {
        (var_uc_svgs,)
    }
};
        var_uc_svgs = assign2540_e1774;

        let assign2550_e1777: f64 = (p.p115 * var_lgle);
        let assign2550_e1779: f64 = (assign2550_e1777 * p.p114);
        let assign2550_e1782: f64 = (p.p115 * var_lgle);
        let assign2550_e1784: f64 = (assign2550_e1782 + p.p114);
        let assign2550_e1785: f64 = (assign2550_e1779 / assign2550_e1784);
        let assign2550_e1787: f64 = (assign2550_e1785 + p.p116);
        let assign2550_e1789: f64 = (assign2550_e1787 + 1e-50);
        var_ddlte = assign2550_e1789;

        let assign2560_e1792: f64 = if var_ddlte < 3.0 { 1.0 } else { 0.0 };
        var_guard14 = assign2560_e1792;

        let (assign2570_e1796,) = {
    if (var_guard14 != 0.0) {
        (3.0,)
    } else {
        (var_ddlte,)
    }
};
        var_ddlte = assign2570_e1796;

        let assign2580_e1799: f64 = (p.p50 * p.p253);
        var_vgs_min = assign2580_e1799;

        let assign2590_e1801: f64 = if param_given[168] { 1.0 } else { 0.0 };
        var_cgbo_given = assign2590_e1801;

        let assign2600_e1803: f64 = if param_given[169] { 1.0 } else { 0.0 };
        var_cgdo_given = assign2600_e1803;

        let assign2610_e1805: f64 = if param_given[170] { 1.0 } else { 0.0 };
        var_cgso_given = assign2610_e1805;

        let assign2620_e1807: f64 = if param_given[294] { 1.0 } else { 0.0 };
        var_cbtbp_given = assign2620_e1807;

        let assign2630_e1809: f64 = if param_given[293] { 1.0 } else { 0.0 };
        var_cbtbn_given = assign2630_e1809;

        let assign2640_e1811: f64 = if param_given[13] { 1.0 } else { 0.0 };
        var_pdbcp_given = assign2640_e1811;

        let assign2650_e1813: f64 = if param_given[14] { 1.0 } else { 0.0 };
        var_psbcp_given = assign2650_e1813;

        let assign2660_e1815: f64 = if param_given[23] { 1.0 } else { 0.0 };
        var_abtp_given = assign2660_e1815;

        let assign2670_e1817: f64 = if param_given[22] { 1.0 } else { 0.0 };
        var_abtn_given = assign2670_e1817;

        let assign2680_e1819: f64 = if param_given[16] { 1.0 } else { 0.0 };
        var_temp_given = assign2680_e1819;

        let (assign2690_e1825,) = {
    if (p.p17 == 0.0) {
        (0.0,)
    } else {
        (1.0,)
    }
};
        var_dtemp_given = assign2690_e1825;

        var_mfactor = 1.0;

        let assign2710_e1829: f64 = 0.0;
        var_gjmin = assign2710_e1829;

        var_uc_pdbcp = p.p13;

        var_uc_psbcp = p.p14;

        let assign2740_e1834: f64 = (p.p16 + 273.15);
        var_uc_temp = assign2740_e1834;

        let assign2750_e1838: f64 = (var_mfactor * var_weff_nf);
        let assign2750_e1839: f64 = (var_mks_rth0 / assign2750_e1838);
        var_rth = assign2750_e1839;

        let assign2760_e1843: f64 = (var_mfactor * var_weffcv_nf);
        let assign2760_e1844: f64 = (var_mks_cth0 * assign2760_e1843);
        var_cth = assign2760_e1844;

        let assign2770_e1863: f64 = if (((p.p10 > 0.0) && (p.p11 > 0.0)) && ((p.p9 == 1.0) || ((p.p9 > 1.0) && (p.p12 > 0.0)))) { 1.0 } else { 0.0 };
        var_guard15 = assign2770_e1863;

        let (assign2780_e1867, assign2780_e1867_d_n0, assign2780_e1867_d_n2, assign2780_e1867_d_n6, assign2780_e1867_d_n7, assign2780_e1867_d_n10, assign2780_e1867_d_n11, assign2780_e1867_d_n12, assign2780_e1867_d_n17,) = {
    if (var_guard15 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign2780_e1867;
        var_t1_dn0 = assign2780_e1867_d_n0;
        var_t1_dn2 = assign2780_e1867_d_n2;
        var_t1_dn6 = assign2780_e1867_d_n6;
        var_t1_dn7 = assign2780_e1867_d_n7;
        var_t1_dn10 = assign2780_e1867_d_n10;
        var_t1_dn11 = assign2780_e1867_d_n11;
        var_t1_dn12 = assign2780_e1867_d_n12;
        var_t1_dn17 = assign2780_e1867_d_n17;

        let (assign2790_e1871,) = {
    if (var_guard15 != 0.0) {
        (0.0,)
    } else {
        (var_i,)
    }
};
        var_i = assign2790_e1871;

        let mut assign2800_loop_guard: usize = 0;
        while {
            let assign2800_cond_e1876: f64 = if ((var_guard15 != 0.0) && (var_i < p.p9)) { 1.0 } else { 0.0 };
            assign2800_cond_e1876 != 0.0
        } {
            assign2800_loop_guard += 1;
            assert!(assign2800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign2800_body0_e1908, assign2800_body0_e1908_d_n0, assign2800_body0_e1908_d_n2, assign2800_body0_e1908_d_n6, assign2800_body0_e1908_d_n7, assign2800_body0_e1908_d_n10, assign2800_body0_e1908_d_n11, assign2800_body0_e1908_d_n12, assign2800_body0_e1908_d_n17,) = {
    if (var_guard15 != 0.0) {
        let assign2800_body0_e1883: f64 = (0.5 * var_lgate);
        let assign2800_body0_e1884: f64 = (p.p10 + assign2800_body0_e1883);
        let assign2800_body0_e1888: f64 = (p.p12 + var_lgate);
        let assign2800_body0_e1889: f64 = (var_i * assign2800_body0_e1888);
        let assign2800_body0_e1890: f64 = (assign2800_body0_e1884 + assign2800_body0_e1889);
        let assign2800_body0_e1891: f64 = (1.0 / assign2800_body0_e1890);
        let assign2800_body0_e1892: f64 = (var_t1 + assign2800_body0_e1891);
        let assign2800_body0_e1897: f64 = (0.5 * var_lgate);
        let assign2800_body0_e1898: f64 = (p.p11 + assign2800_body0_e1897);
        let assign2800_body0_e1902: f64 = (p.p12 + var_lgate);
        let assign2800_body0_e1903: f64 = (var_i * assign2800_body0_e1902);
        let assign2800_body0_e1904: f64 = (assign2800_body0_e1898 + assign2800_body0_e1903);
        let assign2800_body0_e1905: f64 = (1.0 / assign2800_body0_e1904);
        let assign2800_body0_e1906: f64 = (assign2800_body0_e1892 + assign2800_body0_e1905);
        (assign2800_body0_e1906, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
            var_t1 = assign2800_body0_e1908;
            var_t1_dn0 = assign2800_body0_e1908_d_n0;
            var_t1_dn2 = assign2800_body0_e1908_d_n2;
            var_t1_dn6 = assign2800_body0_e1908_d_n6;
            var_t1_dn7 = assign2800_body0_e1908_d_n7;
            var_t1_dn10 = assign2800_body0_e1908_d_n10;
            var_t1_dn11 = assign2800_body0_e1908_d_n11;
            var_t1_dn12 = assign2800_body0_e1908_d_n12;
            var_t1_dn17 = assign2800_body0_e1908_d_n17;
            let (assign2800_body1_e1914,) = {
    if (var_guard15 != 0.0) {
        let assign2800_body1_e1912: f64 = (var_i + 1.0);
        (assign2800_body1_e1912,)
    } else {
        (var_i,)
    }
};
            var_i = assign2800_body1_e1914;
        }

        let (assign2810_e1922, assign2810_e1922_d_n0, assign2810_e1922_d_n2, assign2810_e1922_d_n6, assign2810_e1922_d_n7, assign2810_e1922_d_n10, assign2810_e1922_d_n11, assign2810_e1922_d_n12, assign2810_e1922_d_n17,) = {
    if (var_guard15 != 0.0) {
        let assign2810_e1918: f64 = (2.0 * p.p9);
        let assign2810_e1920: f64 = (assign2810_e1918 / var_t1);
        (assign2810_e1920, (-((assign2810_e1918 * var_t1_dn0) / (var_t1 * var_t1))), (-((assign2810_e1918 * var_t1_dn2) / (var_t1 * var_t1))), (-((assign2810_e1918 * var_t1_dn6) / (var_t1 * var_t1))), (-((assign2810_e1918 * var_t1_dn7) / (var_t1 * var_t1))), (-((assign2810_e1918 * var_t1_dn10) / (var_t1 * var_t1))), (-((assign2810_e1918 * var_t1_dn11) / (var_t1 * var_t1))), (-((assign2810_e1918 * var_t1_dn12) / (var_t1 * var_t1))), (-((assign2810_e1918 * var_t1_dn17) / (var_t1 * var_t1))),)
    } else {
        (var_lod_half, var_lod_half_dn0, var_lod_half_dn2, var_lod_half_dn6, var_lod_half_dn7, var_lod_half_dn10, var_lod_half_dn11, var_lod_half_dn12, var_lod_half_dn17,)
    }
};
        var_lod_half = assign2810_e1922;
        var_lod_half_dn0 = assign2810_e1922_d_n0;
        var_lod_half_dn2 = assign2810_e1922_d_n2;
        var_lod_half_dn6 = assign2810_e1922_d_n6;
        var_lod_half_dn7 = assign2810_e1922_d_n7;
        var_lod_half_dn10 = assign2810_e1922_d_n10;
        var_lod_half_dn11 = assign2810_e1922_d_n11;
        var_lod_half_dn12 = assign2810_e1922_d_n12;
        var_lod_half_dn17 = assign2810_e1922_d_n17;

        let (assign2820_e1927, assign2820_e1927_d_n0, assign2820_e1927_d_n2, assign2820_e1927_d_n6, assign2820_e1927_d_n7, assign2820_e1927_d_n10, assign2820_e1927_d_n11, assign2820_e1927_d_n12, assign2820_e1927_d_n17,) = {
    if (var_guard15 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_lod_half, var_lod_half_dn0, var_lod_half_dn2, var_lod_half_dn6, var_lod_half_dn7, var_lod_half_dn10, var_lod_half_dn11, var_lod_half_dn12, var_lod_half_dn17,)
    }
};
        var_lod_half = assign2820_e1927;
        var_lod_half_dn0 = assign2820_e1927_d_n0;
        var_lod_half_dn2 = assign2820_e1927_d_n2;
        var_lod_half_dn6 = assign2820_e1927_d_n6;
        var_lod_half_dn7 = assign2820_e1927_d_n7;
        var_lod_half_dn10 = assign2820_e1927_d_n10;
        var_lod_half_dn11 = assign2820_e1927_d_n11;
        var_lod_half_dn12 = assign2820_e1927_d_n12;
        var_lod_half_dn17 = assign2820_e1927_d_n17;

        let assign2830_e1930: f64 = if var_lod_half > 0.0 { 1.0 } else { 0.0 };
        var_guard16 = assign2830_e1930;

        let (assign2840_e1938, assign2840_e1938_d_n0, assign2840_e1938_d_n2, assign2840_e1938_d_n6, assign2840_e1938_d_n7, assign2840_e1938_d_n10, assign2840_e1938_d_n11, assign2840_e1938_d_n12, assign2840_e1938_d_n17,) = {
    if (var_guard16 != 0.0) {
        let assign2840_e1935: f64 = (1.0 + p.p162);
        let assign2840_e1936: f64 = (1.0 / assign2840_e1935);
        (assign2840_e1936, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign2840_e1938;
        var_t1_dn0 = assign2840_e1938_d_n0;
        var_t1_dn2 = assign2840_e1938_d_n2;
        var_t1_dn6 = assign2840_e1938_d_n6;
        var_t1_dn7 = assign2840_e1938_d_n7;
        var_t1_dn10 = assign2840_e1938_d_n10;
        var_t1_dn11 = assign2840_e1938_d_n11;
        var_t1_dn12 = assign2840_e1938_d_n12;
        var_t1_dn17 = assign2840_e1938_d_n17;

        let (assign2850_e1946, assign2850_e1946_d_n0, assign2850_e1946_d_n2, assign2850_e1946_d_n6, assign2850_e1946_d_n7, assign2850_e1946_d_n10, assign2850_e1946_d_n11, assign2850_e1946_d_n12, assign2850_e1946_d_n17,) = {
    if (var_guard16 != 0.0) {
        let assign2850_e1942: f64 = (p.p161 / var_lod_half);
        let assign2850_e1944: f64 = (assign2850_e1942).powf(p.p163);
        (assign2850_e1944, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn0) / (var_lod_half * var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * var_lod_half_dn0) / (var_lod_half * var_lod_half))) / assign2850_e1942))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn2) / (var_lod_half * var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * var_lod_half_dn2) / (var_lod_half * var_lod_half))) / assign2850_e1942))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn6) / (var_lod_half * var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * var_lod_half_dn6) / (var_lod_half * var_lod_half))) / assign2850_e1942))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn7) / (var_lod_half * var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * var_lod_half_dn7) / (var_lod_half * var_lod_half))) / assign2850_e1942))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn10) / (var_lod_half * var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * var_lod_half_dn10) / (var_lod_half * var_lod_half))) / assign2850_e1942))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn11) / (var_lod_half * var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * var_lod_half_dn11) / (var_lod_half * var_lod_half))) / assign2850_e1942))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn12) / (var_lod_half * var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * var_lod_half_dn12) / (var_lod_half * var_lod_half))) / assign2850_e1942))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * var_lod_half_dn17) / (var_lod_half * var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * var_lod_half_dn17) / (var_lod_half * var_lod_half))) / assign2850_e1942))) },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign2850_e1946;
        var_t2_dn0 = assign2850_e1946_d_n0;
        var_t2_dn2 = assign2850_e1946_d_n2;
        var_t2_dn6 = assign2850_e1946_d_n6;
        var_t2_dn7 = assign2850_e1946_d_n7;
        var_t2_dn10 = assign2850_e1946_d_n10;
        var_t2_dn11 = assign2850_e1946_d_n11;
        var_t2_dn12 = assign2850_e1946_d_n12;
        var_t2_dn17 = assign2850_e1946_d_n17;

        let (assign2860_e1954, assign2860_e1954_d_n0, assign2860_e1954_d_n2, assign2860_e1954_d_n6, assign2860_e1954_d_n7, assign2860_e1954_d_n10, assign2860_e1954_d_n11, assign2860_e1954_d_n12, assign2860_e1954_d_n17,) = {
    if (var_guard16 != 0.0) {
        let assign2860_e1950: f64 = (p.p161 / var_lod_half_ref);
        let assign2860_e1952: f64 = (assign2860_e1950).powf(p.p163);
        (assign2860_e1952, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign2860_e1954;
        var_t3_dn0 = assign2860_e1954_d_n0;
        var_t3_dn2 = assign2860_e1954_d_n2;
        var_t3_dn6 = assign2860_e1954_d_n6;
        var_t3_dn7 = assign2860_e1954_d_n7;
        var_t3_dn10 = assign2860_e1954_d_n10;
        var_t3_dn11 = assign2860_e1954_d_n11;
        var_t3_dn12 = assign2860_e1954_d_n12;
        var_t3_dn17 = assign2860_e1954_d_n17;

        let (assign2870_e1970, assign2870_e1970_d_n0, assign2870_e1970_d_n2, assign2870_e1970_d_n6, assign2870_e1970_d_n7, assign2870_e1970_d_n10, assign2870_e1970_d_n11, assign2870_e1970_d_n12, assign2870_e1970_d_n17,) = {
    if (var_guard16 != 0.0) {
        let assign2870_e1960: f64 = (var_t1 * var_t2);
        let assign2870_e1961: f64 = (1.0 + assign2870_e1960);
        let assign2870_e1962: f64 = (var_nsubpp * assign2870_e1961);
        let assign2870_e1966: f64 = (var_t1 * var_t3);
        let assign2870_e1967: f64 = (1.0 + assign2870_e1966);
        let assign2870_e1968: f64 = (assign2870_e1962 / assign2870_e1967);
        (assign2870_e1968, ((((var_nsubpp * ((var_t1_dn0 * var_t2) + (var_t1 * var_t2_dn0))) * assign2870_e1967) - (assign2870_e1962 * ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)))) / (assign2870_e1967 * assign2870_e1967)), ((((var_nsubpp * ((var_t1_dn2 * var_t2) + (var_t1 * var_t2_dn2))) * assign2870_e1967) - (assign2870_e1962 * ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)))) / (assign2870_e1967 * assign2870_e1967)), ((((var_nsubpp * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6))) * assign2870_e1967) - (assign2870_e1962 * ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)))) / (assign2870_e1967 * assign2870_e1967)), ((((var_nsubpp * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7))) * assign2870_e1967) - (assign2870_e1962 * ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)))) / (assign2870_e1967 * assign2870_e1967)), ((((var_nsubpp * ((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10))) * assign2870_e1967) - (assign2870_e1962 * ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)))) / (assign2870_e1967 * assign2870_e1967)), ((((var_nsubpp * ((var_t1_dn11 * var_t2) + (var_t1 * var_t2_dn11))) * assign2870_e1967) - (assign2870_e1962 * ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)))) / (assign2870_e1967 * assign2870_e1967)), ((((var_nsubpp * ((var_t1_dn12 * var_t2) + (var_t1 * var_t2_dn12))) * assign2870_e1967) - (assign2870_e1962 * ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)))) / (assign2870_e1967 * assign2870_e1967)), ((((var_nsubpp * ((var_t1_dn17 * var_t2) + (var_t1 * var_t2_dn17))) * assign2870_e1967) - (assign2870_e1962 * ((var_t1_dn17 * var_t3) + (var_t1 * var_t3_dn17)))) / (assign2870_e1967 * assign2870_e1967)),)
    } else {
        (var_nsubps, var_nsubps_dn0, var_nsubps_dn2, var_nsubps_dn6, var_nsubps_dn7, var_nsubps_dn10, var_nsubps_dn11, var_nsubps_dn12, var_nsubps_dn17,)
    }
};
        var_nsubps = assign2870_e1970;
        var_nsubps_dn0 = assign2870_e1970_d_n0;
        var_nsubps_dn2 = assign2870_e1970_d_n2;
        var_nsubps_dn6 = assign2870_e1970_d_n6;
        var_nsubps_dn7 = assign2870_e1970_d_n7;
        var_nsubps_dn10 = assign2870_e1970_d_n10;
        var_nsubps_dn11 = assign2870_e1970_d_n11;
        var_nsubps_dn12 = assign2870_e1970_d_n12;
        var_nsubps_dn17 = assign2870_e1970_d_n17;

        let (assign2880_e1975, assign2880_e1975_d_n0, assign2880_e1975_d_n2, assign2880_e1975_d_n6, assign2880_e1975_d_n7, assign2880_e1975_d_n10, assign2880_e1975_d_n11, assign2880_e1975_d_n12, assign2880_e1975_d_n17,) = {
    if (var_guard16 == 0.0) {
        (var_nsubpp, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_nsubps, var_nsubps_dn0, var_nsubps_dn2, var_nsubps_dn6, var_nsubps_dn7, var_nsubps_dn10, var_nsubps_dn11, var_nsubps_dn12, var_nsubps_dn17,)
    }
};
        var_nsubps = assign2880_e1975;
        var_nsubps_dn0 = assign2880_e1975_d_n0;
        var_nsubps_dn2 = assign2880_e1975_d_n2;
        var_nsubps_dn6 = assign2880_e1975_d_n6;
        var_nsubps_dn7 = assign2880_e1975_d_n7;
        var_nsubps_dn10 = assign2880_e1975_d_n10;
        var_nsubps_dn11 = assign2880_e1975_d_n11;
        var_nsubps_dn12 = assign2880_e1975_d_n12;
        var_nsubps_dn17 = assign2880_e1975_d_n17;

        let assign2890_e1980: f64 = (var_wg).powf(p.p200);
        let assign2890_e1981: f64 = (p.p199 / assign2890_e1980);
        let assign2890_e1982: f64 = (1.0 + assign2890_e1981);
        let assign2890_e1987: f64 = (var_lgle).powf(p.p203);
        let assign2890_e1988: f64 = (p.p202 / assign2890_e1987);
        let assign2890_e1989: f64 = (1.0 + assign2890_e1988);
        let assign2890_e1990: f64 = (assign2890_e1982 * assign2890_e1989);
        var_t2 = assign2890_e1990;
        var_t2_dn0 = 0.0;
        var_t2_dn2 = 0.0;
        var_t2_dn6 = 0.0;
        var_t2_dn7 = 0.0;
        var_t2_dn10 = 0.0;
        var_t2_dn11 = 0.0;
        var_t2_dn12 = 0.0;
        var_t2_dn17 = 0.0;

        let assign2900_e1993: f64 = (var_mks_nsubcmax / var_mks_nsubs);
        var_t3 = assign2900_e1993;
        var_t3_dn0 = 0.0;
        var_t3_dn2 = 0.0;
        var_t3_dn6 = 0.0;
        var_t3_dn7 = 0.0;
        var_t3_dn10 = 0.0;
        var_t3_dn11 = 0.0;
        var_t3_dn12 = 0.0;
        var_t3_dn17 = 0.0;

        let assign2910_e1996: f64 = (var_t3 - var_t2);
        let assign2910_e1998: f64 = (assign2910_e1996 - 0.01);
        var_tmf1 = assign2910_e1998;
        var_tmf1_dn0 = (var_t3_dn0 - var_t2_dn0);
        var_tmf1_dn2 = (var_t3_dn2 - var_t2_dn2);
        var_tmf1_dn6 = (var_t3_dn6 - var_t2_dn6);
        var_tmf1_dn7 = (var_t3_dn7 - var_t2_dn7);
        var_tmf1_dn10 = (var_t3_dn10 - var_t2_dn10);
        var_tmf1_dn11 = (var_t3_dn11 - var_t2_dn11);
        var_tmf1_dn12 = (var_t3_dn12 - var_t2_dn12);
        var_tmf1_dn17 = (var_t3_dn17 - var_t2_dn17);

        let assign2920_e2001: f64 = (4.0 * var_t3);
        let assign2920_e2003: f64 = (assign2920_e2001 * 0.01);
        var_tmf2 = assign2920_e2003;
        var_tmf2_dn0 = ((4.0 * var_t3_dn0) * 0.01);
        var_tmf2_dn2 = ((4.0 * var_t3_dn2) * 0.01);
        var_tmf2_dn6 = ((4.0 * var_t3_dn6) * 0.01);
        var_tmf2_dn7 = ((4.0 * var_t3_dn7) * 0.01);
        var_tmf2_dn10 = ((4.0 * var_t3_dn10) * 0.01);
        var_tmf2_dn11 = ((4.0 * var_t3_dn11) * 0.01);
        var_tmf2_dn12 = ((4.0 * var_t3_dn12) * 0.01);
        var_tmf2_dn17 = ((4.0 * var_t3_dn17) * 0.01);

        let (assign2930_e2010, assign2930_e2010_d_n0, assign2930_e2010_d_n2, assign2930_e2010_d_n6, assign2930_e2010_d_n7, assign2930_e2010_d_n10, assign2930_e2010_d_n11, assign2930_e2010_d_n12, assign2930_e2010_d_n17,) = {
    if (var_tmf2 > 0.0) {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    } else {
        let assign2930_e2009: f64 = (-var_tmf2);
        (assign2930_e2009, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
    }
};
        var_tmf2 = assign2930_e2010;
        var_tmf2_dn0 = assign2930_e2010_d_n0;
        var_tmf2_dn2 = assign2930_e2010_d_n2;
        var_tmf2_dn6 = assign2930_e2010_d_n6;
        var_tmf2_dn7 = assign2930_e2010_d_n7;
        var_tmf2_dn10 = assign2930_e2010_d_n10;
        var_tmf2_dn11 = assign2930_e2010_d_n11;
        var_tmf2_dn12 = assign2930_e2010_d_n12;
        var_tmf2_dn17 = assign2930_e2010_d_n17;

        let assign2940_e2013: f64 = (var_tmf1 * var_tmf1);
        let assign2940_e2015: f64 = (assign2940_e2013 + var_tmf2);
        let assign2940_e2016: f64 = (assign2940_e2015).sqrt();
        var_tmf2 = assign2940_e2016;
        var_tmf2_dn0 = ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign2940_e2016));
        var_tmf2_dn2 = ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign2940_e2016));
        var_tmf2_dn6 = ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign2940_e2016));
        var_tmf2_dn7 = ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign2940_e2016));
        var_tmf2_dn10 = ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign2940_e2016));
        var_tmf2_dn11 = ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign2940_e2016));
        var_tmf2_dn12 = ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign2940_e2016));
        var_tmf2_dn17 = ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign2940_e2016));

        *var_abtn_given_slot = var_abtn_given;
        *var_abtp_given_slot = var_abtp_given;
        *var_cbtbn_given_slot = var_cbtbn_given;
        *var_cbtbp_given_slot = var_cbtbp_given;
        *var_cgbo_given_slot = var_cgbo_given;
        *var_cgdo_given_slot = var_cgdo_given;
        *var_cgso_given_slot = var_cgso_given;
        *var_cqyb0_slot = var_cqyb0;
        *var_cth_slot = var_cth;
        *var_ddlte_slot = var_ddlte;
        *var_dtemp_given_slot = var_dtemp_given;
        *var_gjmin_slot = var_gjmin;
        *var_guard13_slot = var_guard13;
        *var_guard14_slot = var_guard14;
        *var_guard15_slot = var_guard15;
        *var_guard16_slot = var_guard16;
        *var_i_slot = var_i;
        *var_lod_half_slot = var_lod_half;
        *var_lod_half_dn0_slot = var_lod_half_dn0;
        *var_lod_half_dn10_slot = var_lod_half_dn10;
        *var_lod_half_dn11_slot = var_lod_half_dn11;
        *var_lod_half_dn12_slot = var_lod_half_dn12;
        *var_lod_half_dn17_slot = var_lod_half_dn17;
        *var_lod_half_dn2_slot = var_lod_half_dn2;
        *var_lod_half_dn6_slot = var_lod_half_dn6;
        *var_lod_half_dn7_slot = var_lod_half_dn7;
        *var_mfactor_slot = var_mfactor;
        *var_nsubps_slot = var_nsubps;
        *var_nsubps_dn0_slot = var_nsubps_dn0;
        *var_nsubps_dn10_slot = var_nsubps_dn10;
        *var_nsubps_dn11_slot = var_nsubps_dn11;
        *var_nsubps_dn12_slot = var_nsubps_dn12;
        *var_nsubps_dn17_slot = var_nsubps_dn17;
        *var_nsubps_dn2_slot = var_nsubps_dn2;
        *var_nsubps_dn6_slot = var_nsubps_dn6;
        *var_nsubps_dn7_slot = var_nsubps_dn7;
        *var_pdbcp_given_slot = var_pdbcp_given;
        *var_psbcp_given_slot = var_psbcp_given;
        *var_rth_slot = var_rth;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn17_slot = var_t3_dn17;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_temp_given_slot = var_temp_given;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn17_slot = var_tmf2_dn17;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_uc_pdbcp_slot = var_uc_pdbcp;
        *var_uc_psbcp_slot = var_uc_psbcp;
        *var_uc_svgs_slot = var_uc_svgs;
        *var_uc_temp_slot = var_uc_temp;
        *var_vfbsub0_slot = var_vfbsub0;
        *var_vg2const_slot = var_vg2const;
        *var_vg2const_dn0_slot = var_vg2const_dn0;
        *var_vg2const_dn10_slot = var_vg2const_dn10;
        *var_vg2const_dn11_slot = var_vg2const_dn11;
        *var_vg2const_dn12_slot = var_vg2const_dn12;
        *var_vg2const_dn17_slot = var_vg2const_dn17;
        *var_vg2const_dn2_slot = var_vg2const_dn2;
        *var_vg2const_dn6_slot = var_vg2const_dn6;
        *var_vg2const_dn7_slot = var_vg2const_dn7;
        *var_vgs_min_slot = var_vgs_min;
        *var_xgate_slot = var_xgate;
        *var_xsub1_slot = var_xsub1;
        *var_xsub2_slot = var_xsub2;
        *var_xvbs_slot = var_xvbs;
    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        var_abtn_given: f64,
        var_abtp_given: f64,
        var_grg_cnst: f64,
        var_lgle: f64,
        var_lgleff: f64,
        var_lod_half: f64,
        var_lod_half_dn0: f64,
        var_lod_half_dn10: f64,
        var_lod_half_dn11: f64,
        var_lod_half_dn12: f64,
        var_lod_half_dn17: f64,
        var_lod_half_dn2: f64,
        var_lod_half_dn6: f64,
        var_lod_half_dn7: f64,
        var_lod_half_ref: f64,
        var_mfactor: f64,
        var_mks_nsubs: f64,
        var_nsubps: f64,
        var_nsubps_dn0: f64,
        var_nsubps_dn10: f64,
        var_nsubps_dn11: f64,
        var_nsubps_dn12: f64,
        var_nsubps_dn17: f64,
        var_nsubps_dn2: f64,
        var_nsubps_dn6: f64,
        var_nsubps_dn7: f64,
        var_tmf2: f64,
        var_tmf2_dn0: f64,
        var_tmf2_dn10: f64,
        var_tmf2_dn11: f64,
        var_tmf2_dn12: f64,
        var_tmf2_dn17: f64,
        var_tmf2_dn2: f64,
        var_tmf2_dn6: f64,
        var_tmf2_dn7: f64,
        var_weff_nf: f64,
        var_wl: f64,
        var_area_bt_n_slot: &mut f64,
        var_area_bt_p_slot: &mut f64,
        var_grg_slot: &mut f64,
        var_guard17_slot: &mut f64,
        var_guard18_slot: &mut f64,
        var_guard19_slot: &mut f64,
        var_guard20_slot: &mut f64,
        var_guard21_slot: &mut f64,
        var_guard22_slot: &mut f64,
        var_guard23_slot: &mut f64,
        var_guard24_slot: &mut f64,
        var_guard28_slot: &mut f64,
        var_nsub_slot: &mut f64,
        var_nsub_dn0_slot: &mut f64,
        var_nsub_dn10_slot: &mut f64,
        var_nsub_dn11_slot: &mut f64,
        var_nsub_dn12_slot: &mut f64,
        var_nsub_dn17_slot: &mut f64,
        var_nsub_dn2_slot: &mut f64,
        var_nsub_dn6_slot: &mut f64,
        var_nsub_dn7_slot: &mut f64,
        var_nsubb0_slot: &mut f64,
        var_nsubb0_dn0_slot: &mut f64,
        var_nsubb0_dn10_slot: &mut f64,
        var_nsubb0_dn11_slot: &mut f64,
        var_nsubb0_dn12_slot: &mut f64,
        var_nsubb0_dn17_slot: &mut f64,
        var_nsubb0_dn2_slot: &mut f64,
        var_nsubb0_dn6_slot: &mut f64,
        var_nsubb0_dn7_slot: &mut f64,
        var_pb20_slot: &mut f64,
        var_pb20_dn0_slot: &mut f64,
        var_pb20_dn10_slot: &mut f64,
        var_pb20_dn11_slot: &mut f64,
        var_pb20_dn12_slot: &mut f64,
        var_pb20_dn17_slot: &mut f64,
        var_pb20_dn2_slot: &mut f64,
        var_pb20_dn6_slot: &mut f64,
        var_pb20_dn7_slot: &mut f64,
        var_pb2c_slot: &mut f64,
        var_pb2c_dn0_slot: &mut f64,
        var_pb2c_dn10_slot: &mut f64,
        var_pb2c_dn11_slot: &mut f64,
        var_pb2c_dn12_slot: &mut f64,
        var_pb2c_dn17_slot: &mut f64,
        var_pb2c_dn2_slot: &mut f64,
        var_pb2c_dn6_slot: &mut f64,
        var_pb2c_dn7_slot: &mut f64,
        var_ptovr0_slot: &mut f64,
        var_ptovr0_dn0_slot: &mut f64,
        var_ptovr0_dn10_slot: &mut f64,
        var_ptovr0_dn11_slot: &mut f64,
        var_ptovr0_dn12_slot: &mut f64,
        var_ptovr0_dn17_slot: &mut f64,
        var_ptovr0_dn2_slot: &mut f64,
        var_ptovr0_dn6_slot: &mut f64,
        var_ptovr0_dn7_slot: &mut f64,
        var_q_nsub_slot: &mut f64,
        var_q_nsub_dn0_slot: &mut f64,
        var_q_nsub_dn10_slot: &mut f64,
        var_q_nsub_dn11_slot: &mut f64,
        var_q_nsub_dn12_slot: &mut f64,
        var_q_nsub_dn17_slot: &mut f64,
        var_q_nsub_dn2_slot: &mut f64,
        var_q_nsub_dn6_slot: &mut f64,
        var_q_nsub_dn7_slot: &mut f64,
        var_qnsub_esi_slot: &mut f64,
        var_qnsub_esi2_slot: &mut f64,
        var_qnsub_esi2_dn0_slot: &mut f64,
        var_qnsub_esi2_dn10_slot: &mut f64,
        var_qnsub_esi2_dn11_slot: &mut f64,
        var_qnsub_esi2_dn12_slot: &mut f64,
        var_qnsub_esi2_dn17_slot: &mut f64,
        var_qnsub_esi2_dn2_slot: &mut f64,
        var_qnsub_esi2_dn6_slot: &mut f64,
        var_qnsub_esi2_dn7_slot: &mut f64,
        var_qnsub_esi_dn0_slot: &mut f64,
        var_qnsub_esi_dn10_slot: &mut f64,
        var_qnsub_esi_dn11_slot: &mut f64,
        var_qnsub_esi_dn12_slot: &mut f64,
        var_qnsub_esi_dn17_slot: &mut f64,
        var_qnsub_esi_dn2_slot: &mut f64,
        var_qnsub_esi_dn6_slot: &mut f64,
        var_qnsub_esi_dn7_slot: &mut f64,
        var_rbulk_slot: &mut f64,
        var_rbulk_dn0_slot: &mut f64,
        var_rbulk_dn10_slot: &mut f64,
        var_rbulk_dn11_slot: &mut f64,
        var_rbulk_dn12_slot: &mut f64,
        var_rbulk_dn17_slot: &mut f64,
        var_rbulk_dn2_slot: &mut f64,
        var_rbulk_dn6_slot: &mut f64,
        var_rbulk_dn7_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn17_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn17_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_uc_nsubs_slot: &mut f64,
        var_uc_nsubs_dn0_slot: &mut f64,
        var_uc_nsubs_dn10_slot: &mut f64,
        var_uc_nsubs_dn11_slot: &mut f64,
        var_uc_nsubs_dn12_slot: &mut f64,
        var_uc_nsubs_dn17_slot: &mut f64,
        var_uc_nsubs_dn2_slot: &mut f64,
        var_uc_nsubs_dn6_slot: &mut f64,
        var_uc_nsubs_dn7_slot: &mut f64,
        var_vmax0_slot: &mut f64,
        var_vmax0_dn0_slot: &mut f64,
        var_vmax0_dn10_slot: &mut f64,
        var_vmax0_dn11_slot: &mut f64,
        var_vmax0_dn12_slot: &mut f64,
        var_vmax0_dn17_slot: &mut f64,
        var_vmax0_dn2_slot: &mut f64,
        var_vmax0_dn6_slot: &mut f64,
        var_vmax0_dn7_slot: &mut f64,
        var_wdpl_slot: &mut f64,
        var_wdpl_dn0_slot: &mut f64,
        var_wdpl_dn10_slot: &mut f64,
        var_wdpl_dn11_slot: &mut f64,
        var_wdpl_dn12_slot: &mut f64,
        var_wdpl_dn17_slot: &mut f64,
        var_wdpl_dn2_slot: &mut f64,
        var_wdpl_dn6_slot: &mut f64,
        var_wdpl_dn7_slot: &mut f64,
    ) {
        let mut var_area_bt_n: f64 = *var_area_bt_n_slot;
        let mut var_area_bt_p: f64 = *var_area_bt_p_slot;
        let mut var_grg: f64 = *var_grg_slot;
        let mut var_guard17: f64 = *var_guard17_slot;
        let mut var_guard18: f64 = *var_guard18_slot;
        let mut var_guard19: f64 = *var_guard19_slot;
        let mut var_guard20: f64 = *var_guard20_slot;
        let mut var_guard21: f64 = *var_guard21_slot;
        let mut var_guard22: f64 = *var_guard22_slot;
        let mut var_guard23: f64 = *var_guard23_slot;
        let mut var_guard24: f64 = *var_guard24_slot;
        let mut var_guard28: f64 = *var_guard28_slot;
        let mut var_nsub: f64 = *var_nsub_slot;
        let mut var_nsub_dn0: f64 = *var_nsub_dn0_slot;
        let mut var_nsub_dn10: f64 = *var_nsub_dn10_slot;
        let mut var_nsub_dn11: f64 = *var_nsub_dn11_slot;
        let mut var_nsub_dn12: f64 = *var_nsub_dn12_slot;
        let mut var_nsub_dn17: f64 = *var_nsub_dn17_slot;
        let mut var_nsub_dn2: f64 = *var_nsub_dn2_slot;
        let mut var_nsub_dn6: f64 = *var_nsub_dn6_slot;
        let mut var_nsub_dn7: f64 = *var_nsub_dn7_slot;
        let mut var_nsubb0: f64 = *var_nsubb0_slot;
        let mut var_nsubb0_dn0: f64 = *var_nsubb0_dn0_slot;
        let mut var_nsubb0_dn10: f64 = *var_nsubb0_dn10_slot;
        let mut var_nsubb0_dn11: f64 = *var_nsubb0_dn11_slot;
        let mut var_nsubb0_dn12: f64 = *var_nsubb0_dn12_slot;
        let mut var_nsubb0_dn17: f64 = *var_nsubb0_dn17_slot;
        let mut var_nsubb0_dn2: f64 = *var_nsubb0_dn2_slot;
        let mut var_nsubb0_dn6: f64 = *var_nsubb0_dn6_slot;
        let mut var_nsubb0_dn7: f64 = *var_nsubb0_dn7_slot;
        let mut var_pb20: f64 = *var_pb20_slot;
        let mut var_pb20_dn0: f64 = *var_pb20_dn0_slot;
        let mut var_pb20_dn10: f64 = *var_pb20_dn10_slot;
        let mut var_pb20_dn11: f64 = *var_pb20_dn11_slot;
        let mut var_pb20_dn12: f64 = *var_pb20_dn12_slot;
        let mut var_pb20_dn17: f64 = *var_pb20_dn17_slot;
        let mut var_pb20_dn2: f64 = *var_pb20_dn2_slot;
        let mut var_pb20_dn6: f64 = *var_pb20_dn6_slot;
        let mut var_pb20_dn7: f64 = *var_pb20_dn7_slot;
        let mut var_pb2c: f64 = *var_pb2c_slot;
        let mut var_pb2c_dn0: f64 = *var_pb2c_dn0_slot;
        let mut var_pb2c_dn10: f64 = *var_pb2c_dn10_slot;
        let mut var_pb2c_dn11: f64 = *var_pb2c_dn11_slot;
        let mut var_pb2c_dn12: f64 = *var_pb2c_dn12_slot;
        let mut var_pb2c_dn17: f64 = *var_pb2c_dn17_slot;
        let mut var_pb2c_dn2: f64 = *var_pb2c_dn2_slot;
        let mut var_pb2c_dn6: f64 = *var_pb2c_dn6_slot;
        let mut var_pb2c_dn7: f64 = *var_pb2c_dn7_slot;
        let mut var_ptovr0: f64 = *var_ptovr0_slot;
        let mut var_ptovr0_dn0: f64 = *var_ptovr0_dn0_slot;
        let mut var_ptovr0_dn10: f64 = *var_ptovr0_dn10_slot;
        let mut var_ptovr0_dn11: f64 = *var_ptovr0_dn11_slot;
        let mut var_ptovr0_dn12: f64 = *var_ptovr0_dn12_slot;
        let mut var_ptovr0_dn17: f64 = *var_ptovr0_dn17_slot;
        let mut var_ptovr0_dn2: f64 = *var_ptovr0_dn2_slot;
        let mut var_ptovr0_dn6: f64 = *var_ptovr0_dn6_slot;
        let mut var_ptovr0_dn7: f64 = *var_ptovr0_dn7_slot;
        let mut var_q_nsub: f64 = *var_q_nsub_slot;
        let mut var_q_nsub_dn0: f64 = *var_q_nsub_dn0_slot;
        let mut var_q_nsub_dn10: f64 = *var_q_nsub_dn10_slot;
        let mut var_q_nsub_dn11: f64 = *var_q_nsub_dn11_slot;
        let mut var_q_nsub_dn12: f64 = *var_q_nsub_dn12_slot;
        let mut var_q_nsub_dn17: f64 = *var_q_nsub_dn17_slot;
        let mut var_q_nsub_dn2: f64 = *var_q_nsub_dn2_slot;
        let mut var_q_nsub_dn6: f64 = *var_q_nsub_dn6_slot;
        let mut var_q_nsub_dn7: f64 = *var_q_nsub_dn7_slot;
        let mut var_qnsub_esi: f64 = *var_qnsub_esi_slot;
        let mut var_qnsub_esi2: f64 = *var_qnsub_esi2_slot;
        let mut var_qnsub_esi2_dn0: f64 = *var_qnsub_esi2_dn0_slot;
        let mut var_qnsub_esi2_dn10: f64 = *var_qnsub_esi2_dn10_slot;
        let mut var_qnsub_esi2_dn11: f64 = *var_qnsub_esi2_dn11_slot;
        let mut var_qnsub_esi2_dn12: f64 = *var_qnsub_esi2_dn12_slot;
        let mut var_qnsub_esi2_dn17: f64 = *var_qnsub_esi2_dn17_slot;
        let mut var_qnsub_esi2_dn2: f64 = *var_qnsub_esi2_dn2_slot;
        let mut var_qnsub_esi2_dn6: f64 = *var_qnsub_esi2_dn6_slot;
        let mut var_qnsub_esi2_dn7: f64 = *var_qnsub_esi2_dn7_slot;
        let mut var_qnsub_esi_dn0: f64 = *var_qnsub_esi_dn0_slot;
        let mut var_qnsub_esi_dn10: f64 = *var_qnsub_esi_dn10_slot;
        let mut var_qnsub_esi_dn11: f64 = *var_qnsub_esi_dn11_slot;
        let mut var_qnsub_esi_dn12: f64 = *var_qnsub_esi_dn12_slot;
        let mut var_qnsub_esi_dn17: f64 = *var_qnsub_esi_dn17_slot;
        let mut var_qnsub_esi_dn2: f64 = *var_qnsub_esi_dn2_slot;
        let mut var_qnsub_esi_dn6: f64 = *var_qnsub_esi_dn6_slot;
        let mut var_qnsub_esi_dn7: f64 = *var_qnsub_esi_dn7_slot;
        let mut var_rbulk: f64 = *var_rbulk_slot;
        let mut var_rbulk_dn0: f64 = *var_rbulk_dn0_slot;
        let mut var_rbulk_dn10: f64 = *var_rbulk_dn10_slot;
        let mut var_rbulk_dn11: f64 = *var_rbulk_dn11_slot;
        let mut var_rbulk_dn12: f64 = *var_rbulk_dn12_slot;
        let mut var_rbulk_dn17: f64 = *var_rbulk_dn17_slot;
        let mut var_rbulk_dn2: f64 = *var_rbulk_dn2_slot;
        let mut var_rbulk_dn6: f64 = *var_rbulk_dn6_slot;
        let mut var_rbulk_dn7: f64 = *var_rbulk_dn7_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn17: f64 = *var_t0_dn17_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn17: f64 = *var_t3_dn17_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_uc_nsubs: f64 = *var_uc_nsubs_slot;
        let mut var_uc_nsubs_dn0: f64 = *var_uc_nsubs_dn0_slot;
        let mut var_uc_nsubs_dn10: f64 = *var_uc_nsubs_dn10_slot;
        let mut var_uc_nsubs_dn11: f64 = *var_uc_nsubs_dn11_slot;
        let mut var_uc_nsubs_dn12: f64 = *var_uc_nsubs_dn12_slot;
        let mut var_uc_nsubs_dn17: f64 = *var_uc_nsubs_dn17_slot;
        let mut var_uc_nsubs_dn2: f64 = *var_uc_nsubs_dn2_slot;
        let mut var_uc_nsubs_dn6: f64 = *var_uc_nsubs_dn6_slot;
        let mut var_uc_nsubs_dn7: f64 = *var_uc_nsubs_dn7_slot;
        let mut var_vmax0: f64 = *var_vmax0_slot;
        let mut var_vmax0_dn0: f64 = *var_vmax0_dn0_slot;
        let mut var_vmax0_dn10: f64 = *var_vmax0_dn10_slot;
        let mut var_vmax0_dn11: f64 = *var_vmax0_dn11_slot;
        let mut var_vmax0_dn12: f64 = *var_vmax0_dn12_slot;
        let mut var_vmax0_dn17: f64 = *var_vmax0_dn17_slot;
        let mut var_vmax0_dn2: f64 = *var_vmax0_dn2_slot;
        let mut var_vmax0_dn6: f64 = *var_vmax0_dn6_slot;
        let mut var_vmax0_dn7: f64 = *var_vmax0_dn7_slot;
        let mut var_wdpl: f64 = *var_wdpl_slot;
        let mut var_wdpl_dn0: f64 = *var_wdpl_dn0_slot;
        let mut var_wdpl_dn10: f64 = *var_wdpl_dn10_slot;
        let mut var_wdpl_dn11: f64 = *var_wdpl_dn11_slot;
        let mut var_wdpl_dn12: f64 = *var_wdpl_dn12_slot;
        let mut var_wdpl_dn17: f64 = *var_wdpl_dn17_slot;
        let mut var_wdpl_dn2: f64 = *var_wdpl_dn2_slot;
        let mut var_wdpl_dn6: f64 = *var_wdpl_dn6_slot;
        let mut var_wdpl_dn7: f64 = *var_wdpl_dn7_slot;

        let assign2950_e2021: f64 = (var_tmf1 + var_tmf2);
        let assign2950_e2022: f64 = (0.5 * assign2950_e2021);
        let assign2950_e2023: f64 = (var_t3 - assign2950_e2022);
        var_t1 = assign2950_e2023;
        var_t1_dn0 = (var_t3_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)));
        var_t1_dn2 = (var_t3_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)));
        var_t1_dn6 = (var_t3_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)));
        var_t1_dn7 = (var_t3_dn7 - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7)));
        var_t1_dn10 = (var_t3_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)));
        var_t1_dn11 = (var_t3_dn11 - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11)));
        var_t1_dn12 = (var_t3_dn12 - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12)));
        var_t1_dn17 = (var_t3_dn17 - (0.5 * (var_tmf1_dn17 + var_tmf2_dn17)));

        let assign2960_e2026: f64 = (var_mks_nsubs * var_t1);
        var_uc_nsubs = assign2960_e2026;
        var_uc_nsubs_dn0 = (var_mks_nsubs * var_t1_dn0);
        var_uc_nsubs_dn2 = (var_mks_nsubs * var_t1_dn2);
        var_uc_nsubs_dn6 = (var_mks_nsubs * var_t1_dn6);
        var_uc_nsubs_dn7 = (var_mks_nsubs * var_t1_dn7);
        var_uc_nsubs_dn10 = (var_mks_nsubs * var_t1_dn10);
        var_uc_nsubs_dn11 = (var_mks_nsubs * var_t1_dn11);
        var_uc_nsubs_dn12 = (var_mks_nsubs * var_t1_dn12);
        var_uc_nsubs_dn17 = (var_mks_nsubs * var_t1_dn17);

        let assign2970_e2029: f64 = if var_lod_half > 0.0 { 1.0 } else { 0.0 };
        var_guard17 = assign2970_e2029;

        let (assign2980_e2037, assign2980_e2037_d_n0, assign2980_e2037_d_n2, assign2980_e2037_d_n6, assign2980_e2037_d_n7, assign2980_e2037_d_n10, assign2980_e2037_d_n11, assign2980_e2037_d_n12, assign2980_e2037_d_n17,) = {
    if (var_guard17 != 0.0) {
        let assign2980_e2034: f64 = (1.0 + p.p165);
        let assign2980_e2035: f64 = (1.0 / assign2980_e2034);
        (assign2980_e2035, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign2980_e2037;
        var_t1_dn0 = assign2980_e2037_d_n0;
        var_t1_dn2 = assign2980_e2037_d_n2;
        var_t1_dn6 = assign2980_e2037_d_n6;
        var_t1_dn7 = assign2980_e2037_d_n7;
        var_t1_dn10 = assign2980_e2037_d_n10;
        var_t1_dn11 = assign2980_e2037_d_n11;
        var_t1_dn12 = assign2980_e2037_d_n12;
        var_t1_dn17 = assign2980_e2037_d_n17;

        let (assign2990_e2045, assign2990_e2045_d_n0, assign2990_e2045_d_n2, assign2990_e2045_d_n6, assign2990_e2045_d_n7, assign2990_e2045_d_n10, assign2990_e2045_d_n11, assign2990_e2045_d_n12, assign2990_e2045_d_n17,) = {
    if (var_guard17 != 0.0) {
        let assign2990_e2041: f64 = (p.p164 / var_lod_half);
        let assign2990_e2043: f64 = (assign2990_e2041).powf(p.p166);
        (assign2990_e2043, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn0) / (var_lod_half * var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * var_lod_half_dn0) / (var_lod_half * var_lod_half))) / assign2990_e2041))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn2) / (var_lod_half * var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * var_lod_half_dn2) / (var_lod_half * var_lod_half))) / assign2990_e2041))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn6) / (var_lod_half * var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * var_lod_half_dn6) / (var_lod_half * var_lod_half))) / assign2990_e2041))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn7) / (var_lod_half * var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * var_lod_half_dn7) / (var_lod_half * var_lod_half))) / assign2990_e2041))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn10) / (var_lod_half * var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * var_lod_half_dn10) / (var_lod_half * var_lod_half))) / assign2990_e2041))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn11) / (var_lod_half * var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * var_lod_half_dn11) / (var_lod_half * var_lod_half))) / assign2990_e2041))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn12) / (var_lod_half * var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * var_lod_half_dn12) / (var_lod_half * var_lod_half))) / assign2990_e2041))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * var_lod_half_dn17) / (var_lod_half * var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * var_lod_half_dn17) / (var_lod_half * var_lod_half))) / assign2990_e2041))) },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign2990_e2045;
        var_t2_dn0 = assign2990_e2045_d_n0;
        var_t2_dn2 = assign2990_e2045_d_n2;
        var_t2_dn6 = assign2990_e2045_d_n6;
        var_t2_dn7 = assign2990_e2045_d_n7;
        var_t2_dn10 = assign2990_e2045_d_n10;
        var_t2_dn11 = assign2990_e2045_d_n11;
        var_t2_dn12 = assign2990_e2045_d_n12;
        var_t2_dn17 = assign2990_e2045_d_n17;

        let (assign3000_e2053, assign3000_e2053_d_n0, assign3000_e2053_d_n2, assign3000_e2053_d_n6, assign3000_e2053_d_n7, assign3000_e2053_d_n10, assign3000_e2053_d_n11, assign3000_e2053_d_n12, assign3000_e2053_d_n17,) = {
    if (var_guard17 != 0.0) {
        let assign3000_e2049: f64 = (p.p164 / var_lod_half_ref);
        let assign3000_e2051: f64 = (assign3000_e2049).powf(p.p166);
        (assign3000_e2051, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign3000_e2053;
        var_t3_dn0 = assign3000_e2053_d_n0;
        var_t3_dn2 = assign3000_e2053_d_n2;
        var_t3_dn6 = assign3000_e2053_d_n6;
        var_t3_dn7 = assign3000_e2053_d_n7;
        var_t3_dn10 = assign3000_e2053_d_n10;
        var_t3_dn11 = assign3000_e2053_d_n11;
        var_t3_dn12 = assign3000_e2053_d_n12;
        var_t3_dn17 = assign3000_e2053_d_n17;

        let (assign3010_e2069, assign3010_e2069_d_n0, assign3010_e2069_d_n2, assign3010_e2069_d_n6, assign3010_e2069_d_n7, assign3010_e2069_d_n10, assign3010_e2069_d_n11, assign3010_e2069_d_n12, assign3010_e2069_d_n17,) = {
    if (var_guard17 != 0.0) {
        let assign3010_e2059: f64 = (var_t1 * var_t2);
        let assign3010_e2060: f64 = (1.0 + assign3010_e2059);
        let assign3010_e2061: f64 = (var_uc_nsubs * assign3010_e2060);
        let assign3010_e2065: f64 = (var_t1 * var_t3);
        let assign3010_e2066: f64 = (1.0 + assign3010_e2065);
        let assign3010_e2067: f64 = (assign3010_e2061 / assign3010_e2066);
        (assign3010_e2067, (((((var_uc_nsubs_dn0 * assign3010_e2060) + (var_uc_nsubs * ((var_t1_dn0 * var_t2) + (var_t1 * var_t2_dn0)))) * assign3010_e2066) - (assign3010_e2061 * ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)))) / (assign3010_e2066 * assign3010_e2066)), (((((var_uc_nsubs_dn2 * assign3010_e2060) + (var_uc_nsubs * ((var_t1_dn2 * var_t2) + (var_t1 * var_t2_dn2)))) * assign3010_e2066) - (assign3010_e2061 * ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)))) / (assign3010_e2066 * assign3010_e2066)), (((((var_uc_nsubs_dn6 * assign3010_e2060) + (var_uc_nsubs * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6)))) * assign3010_e2066) - (assign3010_e2061 * ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)))) / (assign3010_e2066 * assign3010_e2066)), (((((var_uc_nsubs_dn7 * assign3010_e2060) + (var_uc_nsubs * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7)))) * assign3010_e2066) - (assign3010_e2061 * ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)))) / (assign3010_e2066 * assign3010_e2066)), (((((var_uc_nsubs_dn10 * assign3010_e2060) + (var_uc_nsubs * ((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10)))) * assign3010_e2066) - (assign3010_e2061 * ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)))) / (assign3010_e2066 * assign3010_e2066)), (((((var_uc_nsubs_dn11 * assign3010_e2060) + (var_uc_nsubs * ((var_t1_dn11 * var_t2) + (var_t1 * var_t2_dn11)))) * assign3010_e2066) - (assign3010_e2061 * ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)))) / (assign3010_e2066 * assign3010_e2066)), (((((var_uc_nsubs_dn12 * assign3010_e2060) + (var_uc_nsubs * ((var_t1_dn12 * var_t2) + (var_t1 * var_t2_dn12)))) * assign3010_e2066) - (assign3010_e2061 * ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)))) / (assign3010_e2066 * assign3010_e2066)), (((((var_uc_nsubs_dn17 * assign3010_e2060) + (var_uc_nsubs * ((var_t1_dn17 * var_t2) + (var_t1 * var_t2_dn17)))) * assign3010_e2066) - (assign3010_e2061 * ((var_t1_dn17 * var_t3) + (var_t1 * var_t3_dn17)))) / (assign3010_e2066 * assign3010_e2066)),)
    } else {
        (var_uc_nsubs, var_uc_nsubs_dn0, var_uc_nsubs_dn2, var_uc_nsubs_dn6, var_uc_nsubs_dn7, var_uc_nsubs_dn10, var_uc_nsubs_dn11, var_uc_nsubs_dn12, var_uc_nsubs_dn17,)
    }
};
        var_uc_nsubs = assign3010_e2069;
        var_uc_nsubs_dn0 = assign3010_e2069_d_n0;
        var_uc_nsubs_dn2 = assign3010_e2069_d_n2;
        var_uc_nsubs_dn6 = assign3010_e2069_d_n6;
        var_uc_nsubs_dn7 = assign3010_e2069_d_n7;
        var_uc_nsubs_dn10 = assign3010_e2069_d_n10;
        var_uc_nsubs_dn11 = assign3010_e2069_d_n11;
        var_uc_nsubs_dn12 = assign3010_e2069_d_n12;
        var_uc_nsubs_dn17 = assign3010_e2069_d_n17;

        let assign3020_e2076: f64 = if ((var_lgleff > p.p72) || (p.p72 <= 0.0)) { 1.0 } else { 0.0 };
        var_guard18 = assign3020_e2076;

        let (assign3030_e2090, assign3030_e2090_d_n0, assign3030_e2090_d_n2, assign3030_e2090_d_n6, assign3030_e2090_d_n7, assign3030_e2090_d_n10, assign3030_e2090_d_n11, assign3030_e2090_d_n12, assign3030_e2090_d_n17,) = {
    if (var_guard18 != 0.0) {
        let assign3030_e2081: f64 = (var_lgleff - p.p72);
        let assign3030_e2082: f64 = (var_uc_nsubs * assign3030_e2081);
        let assign3030_e2085: f64 = (var_nsubps * p.p72);
        let assign3030_e2086: f64 = (assign3030_e2082 + assign3030_e2085);
        let assign3030_e2088: f64 = (assign3030_e2086 / var_lgleff);
        (assign3030_e2088, (((var_uc_nsubs_dn0 * assign3030_e2081) + (var_nsubps_dn0 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn2 * assign3030_e2081) + (var_nsubps_dn2 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn6 * assign3030_e2081) + (var_nsubps_dn6 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn7 * assign3030_e2081) + (var_nsubps_dn7 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn10 * assign3030_e2081) + (var_nsubps_dn10 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn11 * assign3030_e2081) + (var_nsubps_dn11 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn12 * assign3030_e2081) + (var_nsubps_dn12 * p.p72)) / var_lgleff), (((var_uc_nsubs_dn17 * assign3030_e2081) + (var_nsubps_dn17 * p.p72)) / var_lgleff),)
    } else {
        (var_nsub, var_nsub_dn0, var_nsub_dn2, var_nsub_dn6, var_nsub_dn7, var_nsub_dn10, var_nsub_dn11, var_nsub_dn12, var_nsub_dn17,)
    }
};
        var_nsub = assign3030_e2090;
        var_nsub_dn0 = assign3030_e2090_d_n0;
        var_nsub_dn2 = assign3030_e2090_d_n2;
        var_nsub_dn6 = assign3030_e2090_d_n6;
        var_nsub_dn7 = assign3030_e2090_d_n7;
        var_nsub_dn10 = assign3030_e2090_d_n10;
        var_nsub_dn11 = assign3030_e2090_d_n11;
        var_nsub_dn12 = assign3030_e2090_d_n12;
        var_nsub_dn17 = assign3030_e2090_d_n17;

        let (assign3040_e2105, assign3040_e2105_d_n0, assign3040_e2105_d_n2, assign3040_e2105_d_n6, assign3040_e2105_d_n7, assign3040_e2105_d_n10, assign3040_e2105_d_n11, assign3040_e2105_d_n12, assign3040_e2105_d_n17,) = {
    if (var_guard18 == 0.0) {
        let assign3040_e2096: f64 = (var_nsubps - var_uc_nsubs);
        let assign3040_e2099: f64 = (p.p72 - var_lgleff);
        let assign3040_e2100: f64 = (assign3040_e2096 * assign3040_e2099);
        let assign3040_e2102: f64 = (assign3040_e2100 / p.p72);
        let assign3040_e2103: f64 = (var_nsubps + assign3040_e2102);
        (assign3040_e2103, (var_nsubps_dn0 + (((var_nsubps_dn0 - var_uc_nsubs_dn0) * assign3040_e2099) / p.p72)), (var_nsubps_dn2 + (((var_nsubps_dn2 - var_uc_nsubs_dn2) * assign3040_e2099) / p.p72)), (var_nsubps_dn6 + (((var_nsubps_dn6 - var_uc_nsubs_dn6) * assign3040_e2099) / p.p72)), (var_nsubps_dn7 + (((var_nsubps_dn7 - var_uc_nsubs_dn7) * assign3040_e2099) / p.p72)), (var_nsubps_dn10 + (((var_nsubps_dn10 - var_uc_nsubs_dn10) * assign3040_e2099) / p.p72)), (var_nsubps_dn11 + (((var_nsubps_dn11 - var_uc_nsubs_dn11) * assign3040_e2099) / p.p72)), (var_nsubps_dn12 + (((var_nsubps_dn12 - var_uc_nsubs_dn12) * assign3040_e2099) / p.p72)), (var_nsubps_dn17 + (((var_nsubps_dn17 - var_uc_nsubs_dn17) * assign3040_e2099) / p.p72)),)
    } else {
        (var_nsub, var_nsub_dn0, var_nsub_dn2, var_nsub_dn6, var_nsub_dn7, var_nsub_dn10, var_nsub_dn11, var_nsub_dn12, var_nsub_dn17,)
    }
};
        var_nsub = assign3040_e2105;
        var_nsub_dn0 = assign3040_e2105_d_n0;
        var_nsub_dn2 = assign3040_e2105_d_n2;
        var_nsub_dn6 = assign3040_e2105_d_n6;
        var_nsub_dn7 = assign3040_e2105_d_n7;
        var_nsub_dn10 = assign3040_e2105_d_n10;
        var_nsub_dn11 = assign3040_e2105_d_n11;
        var_nsub_dn12 = assign3040_e2105_d_n12;
        var_nsub_dn17 = assign3040_e2105_d_n17;

        let assign3050_e2108: f64 = (1.6021918e-19 * var_nsub);
        var_q_nsub = assign3050_e2108;
        var_q_nsub_dn0 = (1.6021918e-19 * var_nsub_dn0);
        var_q_nsub_dn2 = (1.6021918e-19 * var_nsub_dn2);
        var_q_nsub_dn6 = (1.6021918e-19 * var_nsub_dn6);
        var_q_nsub_dn7 = (1.6021918e-19 * var_nsub_dn7);
        var_q_nsub_dn10 = (1.6021918e-19 * var_nsub_dn10);
        var_q_nsub_dn11 = (1.6021918e-19 * var_nsub_dn11);
        var_q_nsub_dn12 = (1.6021918e-19 * var_nsub_dn12);
        var_q_nsub_dn17 = (1.6021918e-19 * var_nsub_dn17);

        let assign3060_e2111: f64 = (var_q_nsub * 1.034943e-10);
        var_qnsub_esi = assign3060_e2111;
        var_qnsub_esi_dn0 = (var_q_nsub_dn0 * 1.034943e-10);
        var_qnsub_esi_dn2 = (var_q_nsub_dn2 * 1.034943e-10);
        var_qnsub_esi_dn6 = (var_q_nsub_dn6 * 1.034943e-10);
        var_qnsub_esi_dn7 = (var_q_nsub_dn7 * 1.034943e-10);
        var_qnsub_esi_dn10 = (var_q_nsub_dn10 * 1.034943e-10);
        var_qnsub_esi_dn11 = (var_q_nsub_dn11 * 1.034943e-10);
        var_qnsub_esi_dn12 = (var_q_nsub_dn12 * 1.034943e-10);
        var_qnsub_esi_dn17 = (var_q_nsub_dn17 * 1.034943e-10);

        let assign3070_e2114: f64 = (2.0 * var_qnsub_esi);
        var_qnsub_esi2 = assign3070_e2114;
        var_qnsub_esi2_dn0 = (2.0 * var_qnsub_esi_dn0);
        var_qnsub_esi2_dn2 = (2.0 * var_qnsub_esi_dn2);
        var_qnsub_esi2_dn6 = (2.0 * var_qnsub_esi_dn6);
        var_qnsub_esi2_dn7 = (2.0 * var_qnsub_esi_dn7);
        var_qnsub_esi2_dn10 = (2.0 * var_qnsub_esi_dn10);
        var_qnsub_esi2_dn11 = (2.0 * var_qnsub_esi_dn11);
        var_qnsub_esi2_dn12 = (2.0 * var_qnsub_esi_dn12);
        var_qnsub_esi2_dn17 = (2.0 * var_qnsub_esi_dn17);

        let assign3080_e2118: f64 = (2.0 * p.p72);
        let assign3080_e2123: f64 = if ((var_lgleff <= assign3080_e2118) && (p.p72 > 0.0)) { 1.0 } else { 0.0 };
        var_guard19 = assign3080_e2123;

        let (assign3090_e2139, assign3090_e2139_d_n0, assign3090_e2139_d_n2, assign3090_e2139_d_n6, assign3090_e2139_d_n7, assign3090_e2139_d_n10, assign3090_e2139_d_n11, assign3090_e2139_d_n12, assign3090_e2139_d_n17,) = {
    if (var_guard19 != 0.0) {
        let assign3090_e2127: f64 = (2.0 * var_nsubps);
        let assign3090_e2130: f64 = (var_nsubps - var_uc_nsubs);
        let assign3090_e2132: f64 = (assign3090_e2130 * var_lgleff);
        let assign3090_e2134: f64 = (assign3090_e2132 / p.p72);
        let assign3090_e2135: f64 = (assign3090_e2127 - assign3090_e2134);
        let assign3090_e2137: f64 = (assign3090_e2135 - var_uc_nsubs);
        (assign3090_e2137, (((2.0 * var_nsubps_dn0) - (((var_nsubps_dn0 - var_uc_nsubs_dn0) * var_lgleff) / p.p72)) - var_uc_nsubs_dn0), (((2.0 * var_nsubps_dn2) - (((var_nsubps_dn2 - var_uc_nsubs_dn2) * var_lgleff) / p.p72)) - var_uc_nsubs_dn2), (((2.0 * var_nsubps_dn6) - (((var_nsubps_dn6 - var_uc_nsubs_dn6) * var_lgleff) / p.p72)) - var_uc_nsubs_dn6), (((2.0 * var_nsubps_dn7) - (((var_nsubps_dn7 - var_uc_nsubs_dn7) * var_lgleff) / p.p72)) - var_uc_nsubs_dn7), (((2.0 * var_nsubps_dn10) - (((var_nsubps_dn10 - var_uc_nsubs_dn10) * var_lgleff) / p.p72)) - var_uc_nsubs_dn10), (((2.0 * var_nsubps_dn11) - (((var_nsubps_dn11 - var_uc_nsubs_dn11) * var_lgleff) / p.p72)) - var_uc_nsubs_dn11), (((2.0 * var_nsubps_dn12) - (((var_nsubps_dn12 - var_uc_nsubs_dn12) * var_lgleff) / p.p72)) - var_uc_nsubs_dn12), (((2.0 * var_nsubps_dn17) - (((var_nsubps_dn17 - var_uc_nsubs_dn17) * var_lgleff) / p.p72)) - var_uc_nsubs_dn17),)
    } else {
        (var_nsubb0, var_nsubb0_dn0, var_nsubb0_dn2, var_nsubb0_dn6, var_nsubb0_dn7, var_nsubb0_dn10, var_nsubb0_dn11, var_nsubb0_dn12, var_nsubb0_dn17,)
    }
};
        var_nsubb0 = assign3090_e2139;
        var_nsubb0_dn0 = assign3090_e2139_d_n0;
        var_nsubb0_dn2 = assign3090_e2139_d_n2;
        var_nsubb0_dn6 = assign3090_e2139_d_n6;
        var_nsubb0_dn7 = assign3090_e2139_d_n7;
        var_nsubb0_dn10 = assign3090_e2139_d_n10;
        var_nsubb0_dn11 = assign3090_e2139_d_n11;
        var_nsubb0_dn12 = assign3090_e2139_d_n12;
        var_nsubb0_dn17 = assign3090_e2139_d_n17;

        let (assign3100_e2146, assign3100_e2146_d_n0, assign3100_e2146_d_n2, assign3100_e2146_d_n6, assign3100_e2146_d_n7, assign3100_e2146_d_n10, assign3100_e2146_d_n11, assign3100_e2146_d_n12, assign3100_e2146_d_n17,) = {
    if (var_guard19 != 0.0) {
        let assign3100_e2143: f64 = (var_nsubb0 / var_uc_nsubs);
        let assign3100_e2144: f64 = (assign3100_e2143).ln();
        (assign3100_e2144, ((((var_nsubb0_dn0 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)) / assign3100_e2143), ((((var_nsubb0_dn2 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)) / assign3100_e2143), ((((var_nsubb0_dn6 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)) / assign3100_e2143), ((((var_nsubb0_dn7 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn7)) / (var_uc_nsubs * var_uc_nsubs)) / assign3100_e2143), ((((var_nsubb0_dn10 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)) / assign3100_e2143), ((((var_nsubb0_dn11 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)) / assign3100_e2143), ((((var_nsubb0_dn12 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)) / assign3100_e2143), ((((var_nsubb0_dn17 * var_uc_nsubs) - (var_nsubb0 * var_uc_nsubs_dn17)) / (var_uc_nsubs * var_uc_nsubs)) / assign3100_e2143),)
    } else {
        (var_ptovr0, var_ptovr0_dn0, var_ptovr0_dn2, var_ptovr0_dn6, var_ptovr0_dn7, var_ptovr0_dn10, var_ptovr0_dn11, var_ptovr0_dn12, var_ptovr0_dn17,)
    }
};
        var_ptovr0 = assign3100_e2146;
        var_ptovr0_dn0 = assign3100_e2146_d_n0;
        var_ptovr0_dn2 = assign3100_e2146_d_n2;
        var_ptovr0_dn6 = assign3100_e2146_d_n6;
        var_ptovr0_dn7 = assign3100_e2146_d_n7;
        var_ptovr0_dn10 = assign3100_e2146_d_n10;
        var_ptovr0_dn11 = assign3100_e2146_d_n11;
        var_ptovr0_dn12 = assign3100_e2146_d_n12;
        var_ptovr0_dn17 = assign3100_e2146_d_n17;

        let (assign3110_e2151, assign3110_e2151_d_n0, assign3110_e2151_d_n2, assign3110_e2151_d_n6, assign3110_e2151_d_n7, assign3110_e2151_d_n10, assign3110_e2151_d_n11, assign3110_e2151_d_n12, assign3110_e2151_d_n17,) = {
    if (var_guard19 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ptovr0, var_ptovr0_dn0, var_ptovr0_dn2, var_ptovr0_dn6, var_ptovr0_dn7, var_ptovr0_dn10, var_ptovr0_dn11, var_ptovr0_dn12, var_ptovr0_dn17,)
    }
};
        var_ptovr0 = assign3110_e2151;
        var_ptovr0_dn0 = assign3110_e2151_d_n0;
        var_ptovr0_dn2 = assign3110_e2151_d_n2;
        var_ptovr0_dn6 = assign3110_e2151_d_n6;
        var_ptovr0_dn7 = assign3110_e2151_d_n7;
        var_ptovr0_dn10 = assign3110_e2151_d_n10;
        var_ptovr0_dn11 = assign3110_e2151_d_n11;
        var_ptovr0_dn12 = assign3110_e2151_d_n12;
        var_ptovr0_dn17 = assign3110_e2151_d_n17;

        let assign3120_e2154: f64 = (2.0 / 38.68283);
        let assign3120_e2158: f64 = (10400000000.0 / 1e-6);
        let assign3120_e2159: f64 = (var_nsub / assign3120_e2158);
        let assign3120_e2160: f64 = (assign3120_e2159).ln();
        let assign3120_e2161: f64 = (assign3120_e2154 * assign3120_e2160);
        var_pb20 = assign3120_e2161;
        var_pb20_dn0 = (assign3120_e2154 * ((var_nsub_dn0 / assign3120_e2158) / assign3120_e2159));
        var_pb20_dn2 = (assign3120_e2154 * ((var_nsub_dn2 / assign3120_e2158) / assign3120_e2159));
        var_pb20_dn6 = (assign3120_e2154 * ((var_nsub_dn6 / assign3120_e2158) / assign3120_e2159));
        var_pb20_dn7 = (assign3120_e2154 * ((var_nsub_dn7 / assign3120_e2158) / assign3120_e2159));
        var_pb20_dn10 = (assign3120_e2154 * ((var_nsub_dn10 / assign3120_e2158) / assign3120_e2159));
        var_pb20_dn11 = (assign3120_e2154 * ((var_nsub_dn11 / assign3120_e2158) / assign3120_e2159));
        var_pb20_dn12 = (assign3120_e2154 * ((var_nsub_dn12 / assign3120_e2158) / assign3120_e2159));
        var_pb20_dn17 = (assign3120_e2154 * ((var_nsub_dn17 / assign3120_e2158) / assign3120_e2159));

        let assign3130_e2164: f64 = (2.0 / 38.68283);
        let assign3130_e2168: f64 = (10400000000.0 / 1e-6);
        let assign3130_e2169: f64 = (var_uc_nsubs / assign3130_e2168);
        let assign3130_e2170: f64 = (assign3130_e2169).ln();
        let assign3130_e2171: f64 = (assign3130_e2164 * assign3130_e2170);
        var_pb2c = assign3130_e2171;
        var_pb2c_dn0 = (assign3130_e2164 * ((var_uc_nsubs_dn0 / assign3130_e2168) / assign3130_e2169));
        var_pb2c_dn2 = (assign3130_e2164 * ((var_uc_nsubs_dn2 / assign3130_e2168) / assign3130_e2169));
        var_pb2c_dn6 = (assign3130_e2164 * ((var_uc_nsubs_dn6 / assign3130_e2168) / assign3130_e2169));
        var_pb2c_dn7 = (assign3130_e2164 * ((var_uc_nsubs_dn7 / assign3130_e2168) / assign3130_e2169));
        var_pb2c_dn10 = (assign3130_e2164 * ((var_uc_nsubs_dn10 / assign3130_e2168) / assign3130_e2169));
        var_pb2c_dn11 = (assign3130_e2164 * ((var_uc_nsubs_dn11 / assign3130_e2168) / assign3130_e2169));
        var_pb2c_dn12 = (assign3130_e2164 * ((var_uc_nsubs_dn12 / assign3130_e2168) / assign3130_e2169));
        var_pb2c_dn17 = (assign3130_e2164 * ((var_uc_nsubs_dn17 / assign3130_e2168) / assign3130_e2169));

        let assign3140_e2174: f64 = (2.0 * 1.034943e-10);
        let assign3140_e2176: f64 = (assign3140_e2174 / 1.6021918e-19);
        let assign3140_e2178: f64 = (assign3140_e2176 / var_nsub);
        let assign3140_e2179: f64 = (assign3140_e2178).sqrt();
        var_wdpl = assign3140_e2179;
        var_wdpl_dn0 = ((-((assign3140_e2176 * var_nsub_dn0) / (var_nsub * var_nsub))) / (2.0 * assign3140_e2179));
        var_wdpl_dn2 = ((-((assign3140_e2176 * var_nsub_dn2) / (var_nsub * var_nsub))) / (2.0 * assign3140_e2179));
        var_wdpl_dn6 = ((-((assign3140_e2176 * var_nsub_dn6) / (var_nsub * var_nsub))) / (2.0 * assign3140_e2179));
        var_wdpl_dn7 = ((-((assign3140_e2176 * var_nsub_dn7) / (var_nsub * var_nsub))) / (2.0 * assign3140_e2179));
        var_wdpl_dn10 = ((-((assign3140_e2176 * var_nsub_dn10) / (var_nsub * var_nsub))) / (2.0 * assign3140_e2179));
        var_wdpl_dn11 = ((-((assign3140_e2176 * var_nsub_dn11) / (var_nsub * var_nsub))) / (2.0 * assign3140_e2179));
        var_wdpl_dn12 = ((-((assign3140_e2176 * var_nsub_dn12) / (var_nsub * var_nsub))) / (2.0 * assign3140_e2179));
        var_wdpl_dn17 = ((-((assign3140_e2176 * var_nsub_dn17) / (var_nsub * var_nsub))) / (2.0 * assign3140_e2179));

        let assign3150_e2184: f64 = (var_lgle).powf(p.p195);
        let assign3150_e2185: f64 = (p.p194 / assign3150_e2184);
        let assign3150_e2186: f64 = (1.0 + assign3150_e2185);
        let assign3150_e2191: f64 = (var_wl).powf(p.p197);
        let assign3150_e2192: f64 = (p.p196 / assign3150_e2191);
        let assign3150_e2193: f64 = (1.0 + assign3150_e2192);
        let assign3150_e2194: f64 = (assign3150_e2186 * assign3150_e2193);
        var_t1 = assign3150_e2194;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn17 = 0.0;

        let assign3160_e2197: f64 = (var_t1 * var_t1);
        let assign3160_e2200: f64 = (4.0 * 0.001);
        let assign3160_e2202: f64 = (assign3160_e2200 * 0.001);
        let assign3160_e2203: f64 = (assign3160_e2197 + assign3160_e2202);
        let assign3160_e2204: f64 = (assign3160_e2203).sqrt();
        var_tmf1 = assign3160_e2204;
        var_tmf1_dn0 = (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) / (2.0 * assign3160_e2204));
        var_tmf1_dn2 = (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) / (2.0 * assign3160_e2204));
        var_tmf1_dn6 = (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) / (2.0 * assign3160_e2204));
        var_tmf1_dn7 = (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) / (2.0 * assign3160_e2204));
        var_tmf1_dn10 = (((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) / (2.0 * assign3160_e2204));
        var_tmf1_dn11 = (((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) / (2.0 * assign3160_e2204));
        var_tmf1_dn12 = (((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)) / (2.0 * assign3160_e2204));
        var_tmf1_dn17 = (((var_t1_dn17 * var_t1) + (var_t1 * var_t1_dn17)) / (2.0 * assign3160_e2204));

        let assign3170_e2208: f64 = (var_t1 + var_tmf1);
        let assign3170_e2209: f64 = (0.5 * assign3170_e2208);
        let assign3170_e2212: f64 = (1e-10 * 0.001);
        let assign3170_e2213: f64 = (assign3170_e2209 + assign3170_e2212);
        var_vmax0 = assign3170_e2213;
        var_vmax0_dn0 = (0.5 * (var_t1_dn0 + var_tmf1_dn0));
        var_vmax0_dn2 = (0.5 * (var_t1_dn2 + var_tmf1_dn2));
        var_vmax0_dn6 = (0.5 * (var_t1_dn6 + var_tmf1_dn6));
        var_vmax0_dn7 = (0.5 * (var_t1_dn7 + var_tmf1_dn7));
        var_vmax0_dn10 = (0.5 * (var_t1_dn10 + var_tmf1_dn10));
        var_vmax0_dn11 = (0.5 * (var_t1_dn11 + var_tmf1_dn11));
        var_vmax0_dn12 = (0.5 * (var_t1_dn12 + var_tmf1_dn12));
        var_vmax0_dn17 = (0.5 * (var_t1_dn17 + var_tmf1_dn17));

        let assign3180_e2216: f64 = if var_vmax0 < 0.0 { 1.0 } else { 0.0 };
        var_guard20 = assign3180_e2216;

        let (assign3190_e2220, assign3190_e2220_d_n0, assign3190_e2220_d_n2, assign3190_e2220_d_n6, assign3190_e2220_d_n7, assign3190_e2220_d_n10, assign3190_e2220_d_n11, assign3190_e2220_d_n12, assign3190_e2220_d_n17,) = {
    if (var_guard20 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vmax0, var_vmax0_dn0, var_vmax0_dn2, var_vmax0_dn6, var_vmax0_dn7, var_vmax0_dn10, var_vmax0_dn11, var_vmax0_dn12, var_vmax0_dn17,)
    }
};
        var_vmax0 = assign3190_e2220;
        var_vmax0_dn0 = assign3190_e2220_d_n0;
        var_vmax0_dn2 = assign3190_e2220_d_n2;
        var_vmax0_dn6 = assign3190_e2220_d_n6;
        var_vmax0_dn7 = assign3190_e2220_d_n7;
        var_vmax0_dn10 = assign3190_e2220_d_n10;
        var_vmax0_dn11 = assign3190_e2220_d_n11;
        var_vmax0_dn12 = assign3190_e2220_d_n12;
        var_vmax0_dn17 = assign3190_e2220_d_n17;

        let assign3200_e2223: f64 = if p.p35 == 1.0 { 1.0 } else { 0.0 };
        var_guard21 = assign3200_e2223;

        let assign3210_e2226: f64 = if var_grg_cnst > 0.001 { 1.0 } else { 0.0 };
        var_guard22 = assign3210_e2226;

        let (assign3220_e2234,) = {
    if ((var_guard21 != 0.0) && (var_guard22 != 0.0)) {
        let assign3220_e2232: f64 = (var_mfactor / var_grg_cnst);
        (assign3220_e2232,)
    } else {
        (var_grg,)
    }
};
        var_grg = assign3220_e2234;

        let (assign3230_e2243,) = {
    if ((var_guard21 != 0.0) && (var_guard22 == 0.0)) {
        let assign3230_e2241: f64 = (var_mfactor * 1000.0);
        (assign3230_e2241,)
    } else {
        (var_grg,)
    }
};
        var_grg = assign3230_e2243;

        let (assign3240_e2250,) = {
    if (var_guard21 == 0.0) {
        let assign3240_e2248: f64 = (var_mfactor * 1000.0);
        (assign3240_e2248,)
    } else {
        (var_grg,)
    }
};
        var_grg = assign3240_e2250;

        let assign3250_e2253: f64 = if p.p261 == 1.0 { 1.0 } else { 0.0 };
        var_guard23 = assign3250_e2253;

        let (assign3260_e2261, assign3260_e2261_d_n0, assign3260_e2261_d_n2, assign3260_e2261_d_n6, assign3260_e2261_d_n7, assign3260_e2261_d_n10, assign3260_e2261_d_n11, assign3260_e2261_d_n12, assign3260_e2261_d_n17,) = {
    if (var_guard23 != 0.0) {
        let assign3260_e2257: f64 = (p.p289 * var_weff_nf);
        let assign3260_e2259: f64 = (assign3260_e2257 + p.p288);
        (assign3260_e2259, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn6, var_t0_dn7, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn17,)
    }
};
        var_t0 = assign3260_e2261;
        var_t0_dn0 = assign3260_e2261_d_n0;
        var_t0_dn2 = assign3260_e2261_d_n2;
        var_t0_dn6 = assign3260_e2261_d_n6;
        var_t0_dn7 = assign3260_e2261_d_n7;
        var_t0_dn10 = assign3260_e2261_d_n10;
        var_t0_dn11 = assign3260_e2261_d_n11;
        var_t0_dn12 = assign3260_e2261_d_n12;
        var_t0_dn17 = assign3260_e2261_d_n17;

        let (assign3270_e2267, assign3270_e2267_d_n0, assign3270_e2267_d_n2, assign3270_e2267_d_n6, assign3270_e2267_d_n7, assign3270_e2267_d_n10, assign3270_e2267_d_n11, assign3270_e2267_d_n12, assign3270_e2267_d_n17,) = {
    if (var_guard23 != 0.0) {
        let assign3270_e2265: f64 = (var_t0 / var_mfactor);
        (assign3270_e2265, (var_t0_dn0 / var_mfactor), (var_t0_dn2 / var_mfactor), (var_t0_dn6 / var_mfactor), (var_t0_dn7 / var_mfactor), (var_t0_dn10 / var_mfactor), (var_t0_dn11 / var_mfactor), (var_t0_dn12 / var_mfactor), (var_t0_dn17 / var_mfactor),)
    } else {
        (var_rbulk, var_rbulk_dn0, var_rbulk_dn2, var_rbulk_dn6, var_rbulk_dn7, var_rbulk_dn10, var_rbulk_dn11, var_rbulk_dn12, var_rbulk_dn17,)
    }
};
        var_rbulk = assign3270_e2267;
        var_rbulk_dn0 = assign3270_e2267_d_n0;
        var_rbulk_dn2 = assign3270_e2267_d_n2;
        var_rbulk_dn6 = assign3270_e2267_d_n6;
        var_rbulk_dn7 = assign3270_e2267_d_n7;
        var_rbulk_dn10 = assign3270_e2267_d_n10;
        var_rbulk_dn11 = assign3270_e2267_d_n11;
        var_rbulk_dn12 = assign3270_e2267_d_n12;
        var_rbulk_dn17 = assign3270_e2267_d_n17;

        let assign3280_e2270: f64 = if var_rbulk < 0.0001 { 1.0 } else { 0.0 };
        var_guard24 = assign3280_e2270;

        let (assign3290_e2276, assign3290_e2276_d_n0, assign3290_e2276_d_n2, assign3290_e2276_d_n6, assign3290_e2276_d_n7, assign3290_e2276_d_n10, assign3290_e2276_d_n11, assign3290_e2276_d_n12, assign3290_e2276_d_n17,) = {
    if ((var_guard23 != 0.0) && (var_guard24 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rbulk, var_rbulk_dn0, var_rbulk_dn2, var_rbulk_dn6, var_rbulk_dn7, var_rbulk_dn10, var_rbulk_dn11, var_rbulk_dn12, var_rbulk_dn17,)
    }
};
        var_rbulk = assign3290_e2276;
        var_rbulk_dn0 = assign3290_e2276_d_n0;
        var_rbulk_dn2 = assign3290_e2276_d_n2;
        var_rbulk_dn6 = assign3290_e2276_d_n6;
        var_rbulk_dn7 = assign3290_e2276_d_n7;
        var_rbulk_dn10 = assign3290_e2276_d_n10;
        var_rbulk_dn11 = assign3290_e2276_d_n11;
        var_rbulk_dn12 = assign3290_e2276_d_n12;
        var_rbulk_dn17 = assign3290_e2276_d_n17;

        let (assign3300_e2281, assign3300_e2281_d_n0, assign3300_e2281_d_n2, assign3300_e2281_d_n6, assign3300_e2281_d_n7, assign3300_e2281_d_n10, assign3300_e2281_d_n11, assign3300_e2281_d_n12, assign3300_e2281_d_n17,) = {
    if (var_guard23 == 0.0) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rbulk, var_rbulk_dn0, var_rbulk_dn2, var_rbulk_dn6, var_rbulk_dn7, var_rbulk_dn10, var_rbulk_dn11, var_rbulk_dn12, var_rbulk_dn17,)
    }
};
        var_rbulk = assign3300_e2281;
        var_rbulk_dn0 = assign3300_e2281_d_n0;
        var_rbulk_dn2 = assign3300_e2281_d_n2;
        var_rbulk_dn6 = assign3300_e2281_d_n6;
        var_rbulk_dn7 = assign3300_e2281_d_n7;
        var_rbulk_dn10 = assign3300_e2281_d_n10;
        var_rbulk_dn11 = assign3300_e2281_d_n11;
        var_rbulk_dn12 = assign3300_e2281_d_n12;
        var_rbulk_dn17 = assign3300_e2281_d_n17;

        let assign3400_e2337: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard28 = assign3400_e2337;

        let (assign3410_e2350,) = {
    if ((var_guard28 != 0.0) && (p.p24 != 0.0)) {
        let (assign3410_e2348,) = {
            if (var_abtp_given != 0.0) {
                (p.p23,)
            } else {
                let assign3410_e2345: f64 = (p.p20 * p.p9);
                let assign3410_e2347: f64 = (assign3410_e2345 * p.p19);
                (assign3410_e2347,)
            }
        };
        (assign3410_e2348,)
    } else {
        (var_area_bt_p,)
    }
};
        var_area_bt_p = assign3410_e2350;

        let (assign3420_e2363,) = {
    if ((var_guard28 != 0.0) && (p.p24 != 0.0)) {
        let (assign3420_e2361,) = {
            if (var_abtn_given != 0.0) {
                (p.p22,)
            } else {
                let assign3420_e2358: f64 = (p.p21 * p.p9);
                let assign3420_e2360: f64 = (assign3420_e2358 * p.p19);
                (assign3420_e2360,)
            }
        };
        (assign3420_e2361,)
    } else {
        (var_area_bt_n,)
    }
};
        var_area_bt_n = assign3420_e2363;

        *var_area_bt_n_slot = var_area_bt_n;
        *var_area_bt_p_slot = var_area_bt_p;
        *var_grg_slot = var_grg;
        *var_guard17_slot = var_guard17;
        *var_guard18_slot = var_guard18;
        *var_guard19_slot = var_guard19;
        *var_guard20_slot = var_guard20;
        *var_guard21_slot = var_guard21;
        *var_guard22_slot = var_guard22;
        *var_guard23_slot = var_guard23;
        *var_guard24_slot = var_guard24;
        *var_guard28_slot = var_guard28;
        *var_nsub_slot = var_nsub;
        *var_nsub_dn0_slot = var_nsub_dn0;
        *var_nsub_dn10_slot = var_nsub_dn10;
        *var_nsub_dn11_slot = var_nsub_dn11;
        *var_nsub_dn12_slot = var_nsub_dn12;
        *var_nsub_dn17_slot = var_nsub_dn17;
        *var_nsub_dn2_slot = var_nsub_dn2;
        *var_nsub_dn6_slot = var_nsub_dn6;
        *var_nsub_dn7_slot = var_nsub_dn7;
        *var_nsubb0_slot = var_nsubb0;
        *var_nsubb0_dn0_slot = var_nsubb0_dn0;
        *var_nsubb0_dn10_slot = var_nsubb0_dn10;
        *var_nsubb0_dn11_slot = var_nsubb0_dn11;
        *var_nsubb0_dn12_slot = var_nsubb0_dn12;
        *var_nsubb0_dn17_slot = var_nsubb0_dn17;
        *var_nsubb0_dn2_slot = var_nsubb0_dn2;
        *var_nsubb0_dn6_slot = var_nsubb0_dn6;
        *var_nsubb0_dn7_slot = var_nsubb0_dn7;
        *var_pb20_slot = var_pb20;
        *var_pb20_dn0_slot = var_pb20_dn0;
        *var_pb20_dn10_slot = var_pb20_dn10;
        *var_pb20_dn11_slot = var_pb20_dn11;
        *var_pb20_dn12_slot = var_pb20_dn12;
        *var_pb20_dn17_slot = var_pb20_dn17;
        *var_pb20_dn2_slot = var_pb20_dn2;
        *var_pb20_dn6_slot = var_pb20_dn6;
        *var_pb20_dn7_slot = var_pb20_dn7;
        *var_pb2c_slot = var_pb2c;
        *var_pb2c_dn0_slot = var_pb2c_dn0;
        *var_pb2c_dn10_slot = var_pb2c_dn10;
        *var_pb2c_dn11_slot = var_pb2c_dn11;
        *var_pb2c_dn12_slot = var_pb2c_dn12;
        *var_pb2c_dn17_slot = var_pb2c_dn17;
        *var_pb2c_dn2_slot = var_pb2c_dn2;
        *var_pb2c_dn6_slot = var_pb2c_dn6;
        *var_pb2c_dn7_slot = var_pb2c_dn7;
        *var_ptovr0_slot = var_ptovr0;
        *var_ptovr0_dn0_slot = var_ptovr0_dn0;
        *var_ptovr0_dn10_slot = var_ptovr0_dn10;
        *var_ptovr0_dn11_slot = var_ptovr0_dn11;
        *var_ptovr0_dn12_slot = var_ptovr0_dn12;
        *var_ptovr0_dn17_slot = var_ptovr0_dn17;
        *var_ptovr0_dn2_slot = var_ptovr0_dn2;
        *var_ptovr0_dn6_slot = var_ptovr0_dn6;
        *var_ptovr0_dn7_slot = var_ptovr0_dn7;
        *var_q_nsub_slot = var_q_nsub;
        *var_q_nsub_dn0_slot = var_q_nsub_dn0;
        *var_q_nsub_dn10_slot = var_q_nsub_dn10;
        *var_q_nsub_dn11_slot = var_q_nsub_dn11;
        *var_q_nsub_dn12_slot = var_q_nsub_dn12;
        *var_q_nsub_dn17_slot = var_q_nsub_dn17;
        *var_q_nsub_dn2_slot = var_q_nsub_dn2;
        *var_q_nsub_dn6_slot = var_q_nsub_dn6;
        *var_q_nsub_dn7_slot = var_q_nsub_dn7;
        *var_qnsub_esi_slot = var_qnsub_esi;
        *var_qnsub_esi2_slot = var_qnsub_esi2;
        *var_qnsub_esi2_dn0_slot = var_qnsub_esi2_dn0;
        *var_qnsub_esi2_dn10_slot = var_qnsub_esi2_dn10;
        *var_qnsub_esi2_dn11_slot = var_qnsub_esi2_dn11;
        *var_qnsub_esi2_dn12_slot = var_qnsub_esi2_dn12;
        *var_qnsub_esi2_dn17_slot = var_qnsub_esi2_dn17;
        *var_qnsub_esi2_dn2_slot = var_qnsub_esi2_dn2;
        *var_qnsub_esi2_dn6_slot = var_qnsub_esi2_dn6;
        *var_qnsub_esi2_dn7_slot = var_qnsub_esi2_dn7;
        *var_qnsub_esi_dn0_slot = var_qnsub_esi_dn0;
        *var_qnsub_esi_dn10_slot = var_qnsub_esi_dn10;
        *var_qnsub_esi_dn11_slot = var_qnsub_esi_dn11;
        *var_qnsub_esi_dn12_slot = var_qnsub_esi_dn12;
        *var_qnsub_esi_dn17_slot = var_qnsub_esi_dn17;
        *var_qnsub_esi_dn2_slot = var_qnsub_esi_dn2;
        *var_qnsub_esi_dn6_slot = var_qnsub_esi_dn6;
        *var_qnsub_esi_dn7_slot = var_qnsub_esi_dn7;
        *var_rbulk_slot = var_rbulk;
        *var_rbulk_dn0_slot = var_rbulk_dn0;
        *var_rbulk_dn10_slot = var_rbulk_dn10;
        *var_rbulk_dn11_slot = var_rbulk_dn11;
        *var_rbulk_dn12_slot = var_rbulk_dn12;
        *var_rbulk_dn17_slot = var_rbulk_dn17;
        *var_rbulk_dn2_slot = var_rbulk_dn2;
        *var_rbulk_dn6_slot = var_rbulk_dn6;
        *var_rbulk_dn7_slot = var_rbulk_dn7;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn17_slot = var_t0_dn17;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn17_slot = var_t3_dn17;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_uc_nsubs_slot = var_uc_nsubs;
        *var_uc_nsubs_dn0_slot = var_uc_nsubs_dn0;
        *var_uc_nsubs_dn10_slot = var_uc_nsubs_dn10;
        *var_uc_nsubs_dn11_slot = var_uc_nsubs_dn11;
        *var_uc_nsubs_dn12_slot = var_uc_nsubs_dn12;
        *var_uc_nsubs_dn17_slot = var_uc_nsubs_dn17;
        *var_uc_nsubs_dn2_slot = var_uc_nsubs_dn2;
        *var_uc_nsubs_dn6_slot = var_uc_nsubs_dn6;
        *var_uc_nsubs_dn7_slot = var_uc_nsubs_dn7;
        *var_vmax0_slot = var_vmax0;
        *var_vmax0_dn0_slot = var_vmax0_dn0;
        *var_vmax0_dn10_slot = var_vmax0_dn10;
        *var_vmax0_dn11_slot = var_vmax0_dn11;
        *var_vmax0_dn12_slot = var_vmax0_dn12;
        *var_vmax0_dn17_slot = var_vmax0_dn17;
        *var_vmax0_dn2_slot = var_vmax0_dn2;
        *var_vmax0_dn6_slot = var_vmax0_dn6;
        *var_vmax0_dn7_slot = var_vmax0_dn7;
        *var_wdpl_slot = var_wdpl;
        *var_wdpl_dn0_slot = var_wdpl_dn0;
        *var_wdpl_dn10_slot = var_wdpl_dn10;
        *var_wdpl_dn11_slot = var_wdpl_dn11;
        *var_wdpl_dn12_slot = var_wdpl_dn12;
        *var_wdpl_dn17_slot = var_wdpl_dn17;
        *var_wdpl_dn2_slot = var_wdpl_dn2;
        *var_wdpl_dn6_slot = var_wdpl_dn6;
        *var_wdpl_dn7_slot = var_wdpl_dn7;
    }

    pub(super) fn stamp_transient_block_6(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_cbtbn_given: f64,
        var_cbtbp_given: f64,
        var_flg_nqs: f64,
        var_guard28: f64,
        var_lgate: f64,
        var_mks_rth0: f64,
        var_pdbcp_given: f64,
        var_psbcp_given: f64,
        var_weff_nf: f64,
        var_weffcv_nf: f64,
        var_area_bt_n_slot: &mut f64,
        var_area_bt_p_slot: &mut f64,
        var_cbtn_slot: &mut f64,
        var_cbtp_slot: &mut f64,
        var_deltemp_slot: &mut f64,
        var_deltemp_dn10_slot: &mut f64,
        var_guard29_slot: &mut f64,
        var_guard30_slot: &mut f64,
        var_guard31_slot: &mut f64,
        var_guard32_slot: &mut f64,
        var_guard33_slot: &mut f64,
        var_guard34_slot: &mut f64,
        var_mode_slot: &mut f64,
        var_modenml_slot: &mut f64,
        var_peri_hhi_slot: &mut f64,
        var_qb_nqs_slot: &mut f64,
        var_qb_nqs_dn13_slot: &mut f64,
        var_qd_nqs_slot: &mut f64,
        var_qd_nqs_dn0_slot: &mut f64,
        var_qd_nqs_dn10_slot: &mut f64,
        var_qd_nqs_dn11_slot: &mut f64,
        var_qd_nqs_dn12_slot: &mut f64,
        var_qd_nqs_dn15_slot: &mut f64,
        var_qd_nqs_dn17_slot: &mut f64,
        var_qd_nqs_dn18_slot: &mut f64,
        var_qd_nqs_dn2_slot: &mut f64,
        var_qd_nqs_dn6_slot: &mut f64,
        var_qd_nqs_dn7_slot: &mut f64,
        var_qi_nqs_slot: &mut f64,
        var_qi_nqs_dn18_slot: &mut f64,
        var_qs_nqs_slot: &mut f64,
        var_qs_nqs_dn0_slot: &mut f64,
        var_qs_nqs_dn10_slot: &mut f64,
        var_qs_nqs_dn11_slot: &mut f64,
        var_qs_nqs_dn12_slot: &mut f64,
        var_qs_nqs_dn16_slot: &mut f64,
        var_qs_nqs_dn17_slot: &mut f64,
        var_qs_nqs_dn18_slot: &mut f64,
        var_qs_nqs_dn2_slot: &mut f64,
        var_qs_nqs_dn6_slot: &mut f64,
        var_qs_nqs_dn7_slot: &mut f64,
        var_uc_pdbcp_slot: &mut f64,
        var_uc_psbcp_slot: &mut f64,
        var_vbcd_slot: &mut f64,
        var_vbcd_dn12_slot: &mut f64,
        var_vbcd_dn6_slot: &mut f64,
        var_vbcs_slot: &mut f64,
        var_vbcs_dn12_slot: &mut f64,
        var_vbcs_dn7_slot: &mut f64,
        var_vbsi_slot: &mut f64,
        var_vbsi_dn12_slot: &mut f64,
        var_vbsi_dn7_slot: &mut f64,
        var_vdsi_slot: &mut f64,
        var_vdsi_dn6_slot: &mut f64,
        var_vdsi_dn7_slot: &mut f64,
        var_vgsi_slot: &mut f64,
        var_vgsi_dn11_slot: &mut f64,
        var_vgsi_dn7_slot: &mut f64,
        var_w_diod_slot: &mut f64,
        var_w_diodcv_slot: &mut f64,
        var_w_dios_slot: &mut f64,
        var_w_dioscv_slot: &mut f64,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let mut var_area_bt_n: f64 = *var_area_bt_n_slot;
        let mut var_area_bt_p: f64 = *var_area_bt_p_slot;
        let mut var_cbtn: f64 = *var_cbtn_slot;
        let mut var_cbtp: f64 = *var_cbtp_slot;
        let mut var_deltemp: f64 = *var_deltemp_slot;
        let mut var_deltemp_dn10: f64 = *var_deltemp_dn10_slot;
        let mut var_guard29: f64 = *var_guard29_slot;
        let mut var_guard30: f64 = *var_guard30_slot;
        let mut var_guard31: f64 = *var_guard31_slot;
        let mut var_guard32: f64 = *var_guard32_slot;
        let mut var_guard33: f64 = *var_guard33_slot;
        let mut var_guard34: f64 = *var_guard34_slot;
        let mut var_mode: f64 = *var_mode_slot;
        let mut var_modenml: f64 = *var_modenml_slot;
        let mut var_peri_hhi: f64 = *var_peri_hhi_slot;
        let mut var_qb_nqs: f64 = *var_qb_nqs_slot;
        let mut var_qb_nqs_dn13: f64 = *var_qb_nqs_dn13_slot;
        let mut var_qd_nqs: f64 = *var_qd_nqs_slot;
        let mut var_qd_nqs_dn0: f64 = *var_qd_nqs_dn0_slot;
        let mut var_qd_nqs_dn10: f64 = *var_qd_nqs_dn10_slot;
        let mut var_qd_nqs_dn11: f64 = *var_qd_nqs_dn11_slot;
        let mut var_qd_nqs_dn12: f64 = *var_qd_nqs_dn12_slot;
        let mut var_qd_nqs_dn15: f64 = *var_qd_nqs_dn15_slot;
        let mut var_qd_nqs_dn17: f64 = *var_qd_nqs_dn17_slot;
        let mut var_qd_nqs_dn18: f64 = *var_qd_nqs_dn18_slot;
        let mut var_qd_nqs_dn2: f64 = *var_qd_nqs_dn2_slot;
        let mut var_qd_nqs_dn6: f64 = *var_qd_nqs_dn6_slot;
        let mut var_qd_nqs_dn7: f64 = *var_qd_nqs_dn7_slot;
        let mut var_qi_nqs: f64 = *var_qi_nqs_slot;
        let mut var_qi_nqs_dn18: f64 = *var_qi_nqs_dn18_slot;
        let mut var_qs_nqs: f64 = *var_qs_nqs_slot;
        let mut var_qs_nqs_dn0: f64 = *var_qs_nqs_dn0_slot;
        let mut var_qs_nqs_dn10: f64 = *var_qs_nqs_dn10_slot;
        let mut var_qs_nqs_dn11: f64 = *var_qs_nqs_dn11_slot;
        let mut var_qs_nqs_dn12: f64 = *var_qs_nqs_dn12_slot;
        let mut var_qs_nqs_dn16: f64 = *var_qs_nqs_dn16_slot;
        let mut var_qs_nqs_dn17: f64 = *var_qs_nqs_dn17_slot;
        let mut var_qs_nqs_dn18: f64 = *var_qs_nqs_dn18_slot;
        let mut var_qs_nqs_dn2: f64 = *var_qs_nqs_dn2_slot;
        let mut var_qs_nqs_dn6: f64 = *var_qs_nqs_dn6_slot;
        let mut var_qs_nqs_dn7: f64 = *var_qs_nqs_dn7_slot;
        let mut var_uc_pdbcp: f64 = *var_uc_pdbcp_slot;
        let mut var_uc_psbcp: f64 = *var_uc_psbcp_slot;
        let mut var_vbcd: f64 = *var_vbcd_slot;
        let mut var_vbcd_dn12: f64 = *var_vbcd_dn12_slot;
        let mut var_vbcd_dn6: f64 = *var_vbcd_dn6_slot;
        let mut var_vbcs: f64 = *var_vbcs_slot;
        let mut var_vbcs_dn12: f64 = *var_vbcs_dn12_slot;
        let mut var_vbcs_dn7: f64 = *var_vbcs_dn7_slot;
        let mut var_vbsi: f64 = *var_vbsi_slot;
        let mut var_vbsi_dn12: f64 = *var_vbsi_dn12_slot;
        let mut var_vbsi_dn7: f64 = *var_vbsi_dn7_slot;
        let mut var_vdsi: f64 = *var_vdsi_slot;
        let mut var_vdsi_dn6: f64 = *var_vdsi_dn6_slot;
        let mut var_vdsi_dn7: f64 = *var_vdsi_dn7_slot;
        let mut var_vgsi: f64 = *var_vgsi_slot;
        let mut var_vgsi_dn11: f64 = *var_vgsi_dn11_slot;
        let mut var_vgsi_dn7: f64 = *var_vgsi_dn7_slot;
        let mut var_w_diod: f64 = *var_w_diod_slot;
        let mut var_w_diodcv: f64 = *var_w_diodcv_slot;
        let mut var_w_dios: f64 = *var_w_dios_slot;
        let mut var_w_dioscv: f64 = *var_w_dioscv_slot;

        let (assign3430_e2369,) = {
    if ((var_guard28 != 0.0) && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (var_cbtp,)
    }
};
        var_cbtp = assign3430_e2369;

        let (assign3440_e2375,) = {
    if ((var_guard28 != 0.0) && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (var_cbtn,)
    }
};
        var_cbtn = assign3440_e2375;

        let assign3450_e2380: f64 = if ((var_area_bt_p > 0.0) && (var_cbtbp_given != 0.0)) { 1.0 } else { 0.0 };
        var_guard29 = assign3450_e2380;

        let (assign3460_e2391,) = {
    if (((var_guard28 != 0.0) && (p.p24 != 0.0)) && (var_guard29 != 0.0)) {
        let assign3460_e2387: f64 = (-var_area_bt_p);
        let assign3460_e2389: f64 = (assign3460_e2387 * p.p294);
        (assign3460_e2389,)
    } else {
        (var_cbtp,)
    }
};
        var_cbtp = assign3460_e2391;

        let (assign3470_e2400,) = {
    if (((var_guard28 != 0.0) && (p.p24 != 0.0)) && (var_guard29 == 0.0)) {
        (0.0,)
    } else {
        (var_cbtp,)
    }
};
        var_cbtp = assign3470_e2400;

        let assign3480_e2405: f64 = if ((var_area_bt_n > 0.0) && (var_cbtbn_given != 0.0)) { 1.0 } else { 0.0 };
        var_guard30 = assign3480_e2405;

        let (assign3490_e2416,) = {
    if (((var_guard28 != 0.0) && (p.p24 != 0.0)) && (var_guard30 != 0.0)) {
        let assign3490_e2412: f64 = (-var_area_bt_n);
        let assign3490_e2414: f64 = (assign3490_e2412 * p.p293);
        (assign3490_e2414,)
    } else {
        (var_cbtn,)
    }
};
        var_cbtn = assign3490_e2416;

        let (assign3500_e2424,) = {
    if (((var_guard28 != 0.0) && (p.p24 != 0.0)) && (var_guard30 != 0.0)) {
        (0.0,)
    } else {
        (var_area_bt_n,)
    }
};
        var_area_bt_n = assign3500_e2424;

        let (assign3510_e2431,) = {
    if ((var_guard28 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (var_area_bt_n,)
    }
};
        var_area_bt_n = assign3510_e2431;

        let (assign3520_e2438,) = {
    if ((var_guard28 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (var_cbtn,)
    }
};
        var_cbtn = assign3520_e2438;

        let (assign3530_e2445,) = {
    if ((var_guard28 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (var_area_bt_p,)
    }
};
        var_area_bt_p = assign3530_e2445;

        let (assign3540_e2452,) = {
    if ((var_guard28 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (var_cbtp,)
    }
};
        var_cbtp = assign3540_e2452;

        let (assign3550_e2465,) = {
    if (var_guard28 != 0.0) {
        let (assign3550_e2463,) = {
            if (p.p19 > var_lgate) {
                let assign3550_e2460: f64 = (p.p19 - var_lgate);
                let assign3550_e2461: f64 = (0.5 * assign3550_e2460);
                (assign3550_e2461,)
            } else {
                (0.0,)
            }
        };
        (assign3550_e2463,)
    } else {
        (var_peri_hhi,)
    }
};
        var_peri_hhi = assign3550_e2465;

        let assign3560_e2468: f64 = if var_pdbcp_given == 0.0 { 1.0 } else { 0.0 };
        var_guard31 = assign3560_e2468;

        let (assign3570_e2474,) = {
    if ((var_guard28 != 0.0) && (var_guard31 != 0.0)) {
        (var_peri_hhi,)
    } else {
        (var_uc_pdbcp,)
    }
};
        var_uc_pdbcp = assign3570_e2474;

        let assign3580_e2477: f64 = if var_psbcp_given == 0.0 { 1.0 } else { 0.0 };
        var_guard32 = assign3580_e2477;

        let (assign3590_e2483,) = {
    if ((var_guard28 != 0.0) && (var_guard32 != 0.0)) {
        (var_peri_hhi,)
    } else {
        (var_uc_psbcp,)
    }
};
        var_uc_psbcp = assign3590_e2483;

        let (assign3600_e2491,) = {
    if (var_guard28 != 0.0) {
        let assign3600_e2488: f64 = (p.p9 * var_uc_pdbcp);
        let assign3600_e2489: f64 = (var_weff_nf + assign3600_e2488);
        (assign3600_e2489,)
    } else {
        (var_w_diod,)
    }
};
        var_w_diod = assign3600_e2491;

        let (assign3610_e2499,) = {
    if (var_guard28 != 0.0) {
        let assign3610_e2496: f64 = (p.p9 * var_uc_psbcp);
        let assign3610_e2497: f64 = (var_weff_nf + assign3610_e2496);
        (assign3610_e2497,)
    } else {
        (var_w_dios,)
    }
};
        var_w_dios = assign3610_e2499;

        let (assign3620_e2507,) = {
    if (var_guard28 != 0.0) {
        let assign3620_e2504: f64 = (p.p9 * var_uc_pdbcp);
        let assign3620_e2505: f64 = (var_weffcv_nf + assign3620_e2504);
        (assign3620_e2505,)
    } else {
        (var_w_diodcv,)
    }
};
        var_w_diodcv = assign3620_e2507;

        let (assign3630_e2515,) = {
    if (var_guard28 != 0.0) {
        let assign3630_e2512: f64 = (p.p9 * var_uc_psbcp);
        let assign3630_e2513: f64 = (var_weffcv_nf + assign3630_e2512);
        (assign3630_e2513,)
    } else {
        (var_w_dioscv,)
    }
};
        var_w_dioscv = assign3630_e2515;

        let (assign3640_e2520,) = {
    if (var_guard28 == 0.0) {
        (0.0,)
    } else {
        (var_area_bt_n,)
    }
};
        var_area_bt_n = assign3640_e2520;

        let (assign3650_e2525,) = {
    if (var_guard28 == 0.0) {
        (0.0,)
    } else {
        (var_cbtn,)
    }
};
        var_cbtn = assign3650_e2525;

        let (assign3660_e2530,) = {
    if (var_guard28 == 0.0) {
        (0.0,)
    } else {
        (var_area_bt_p,)
    }
};
        var_area_bt_p = assign3660_e2530;

        let (assign3670_e2535,) = {
    if (var_guard28 == 0.0) {
        (0.0,)
    } else {
        (var_cbtp,)
    }
};
        var_cbtp = assign3670_e2535;

        let (assign3680_e2540,) = {
    if (var_guard28 == 0.0) {
        (0.0,)
    } else {
        (var_w_diod,)
    }
};
        var_w_diod = assign3680_e2540;

        let (assign3690_e2545,) = {
    if (var_guard28 == 0.0) {
        (0.0,)
    } else {
        (var_w_dios,)
    }
};
        var_w_dios = assign3690_e2545;

        let (assign3700_e2550,) = {
    if (var_guard28 == 0.0) {
        (0.0,)
    } else {
        (var_w_diodcv,)
    }
};
        var_w_diodcv = assign3700_e2550;

        let (assign3710_e2555,) = {
    if (var_guard28 == 0.0) {
        (0.0,)
    } else {
        (var_w_dioscv,)
    }
};
        var_w_dioscv = assign3710_e2555;

        let assign3720_e2558: f64 = (p.p50 * (nv6 - nv7));
        var_vdsi = assign3720_e2558;
        var_vdsi_dn6 = p.p50;
        var_vdsi_dn7 = (-p.p50);

        let assign3730_e2561: f64 = (p.p50 * (nv11 - nv7));
        var_vgsi = assign3730_e2561;
        var_vgsi_dn7 = (-p.p50);
        var_vgsi_dn11 = p.p50;

        let assign3740_e2564: f64 = (p.p50 * (nv12 - nv7));
        var_vbsi = assign3740_e2564;
        var_vbsi_dn7 = (-p.p50);
        var_vbsi_dn12 = p.p50;

        let assign3780_e2576: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard33 = assign3780_e2576;

        let (assign3790_e2582, assign3790_e2582_d_n6, assign3790_e2582_d_n12,) = {
    if (var_guard33 != 0.0) {
        let assign3790_e2580: f64 = (p.p50 * (nv12 - nv6));
        (assign3790_e2580, (-p.p50), p.p50,)
    } else {
        (var_vbcd, var_vbcd_dn6, var_vbcd_dn12,)
    }
};
        var_vbcd = assign3790_e2582;
        var_vbcd_dn6 = assign3790_e2582_d_n6;
        var_vbcd_dn12 = assign3790_e2582_d_n12;

        let (assign3800_e2588, assign3800_e2588_d_n7, assign3800_e2588_d_n12,) = {
    if (var_guard33 != 0.0) {
        let assign3800_e2586: f64 = (p.p50 * (nv12 - nv7));
        (assign3800_e2586, (-p.p50), p.p50,)
    } else {
        (var_vbcs, var_vbcs_dn7, var_vbcs_dn12,)
    }
};
        var_vbcs = assign3800_e2588;
        var_vbcs_dn7 = assign3800_e2588_d_n7;
        var_vbcs_dn12 = assign3800_e2588_d_n12;

        let (assign3810_e2598, assign3810_e2598_d_n18,) = {
    if ((var_guard33 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign3810_e2594: f64 = (1e-9 / 0.0001);
        let assign3810_e2596: f64 = (assign3810_e2594 * (nv18 - 0.0));
        (assign3810_e2596, assign3810_e2594,)
    } else {
        (var_qi_nqs, var_qi_nqs_dn18,)
    }
};
        var_qi_nqs = assign3810_e2598;
        var_qi_nqs_dn18 = assign3810_e2598_d_n18;

        let (assign3820_e2608, assign3820_e2608_d_n13,) = {
    if ((var_guard33 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign3820_e2604: f64 = (1e-9 / 0.0001);
        let assign3820_e2606: f64 = (assign3820_e2604 * (nv13 - 0.0));
        (assign3820_e2606, assign3820_e2604,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign3820_e2608;
        var_qb_nqs_dn13 = assign3820_e2608_d_n13;

        let (assign3830_e2615, assign3830_e2615_d_n18,) = {
    if ((var_guard33 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qi_nqs, var_qi_nqs_dn18,)
    }
};
        var_qi_nqs = assign3830_e2615;
        var_qi_nqs_dn18 = assign3830_e2615_d_n18;

        let (assign3840_e2622, assign3840_e2622_d_n13,) = {
    if ((var_guard33 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign3840_e2622;
        var_qb_nqs_dn13 = assign3840_e2622_d_n13;

        let (assign3850_e2627, assign3850_e2627_d_n6, assign3850_e2627_d_n12,) = {
    if (var_guard33 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_vbcd, var_vbcd_dn6, var_vbcd_dn12,)
    }
};
        var_vbcd = assign3850_e2627;
        var_vbcd_dn6 = assign3850_e2627_d_n6;
        var_vbcd_dn12 = assign3850_e2627_d_n12;

        let (assign3860_e2632, assign3860_e2632_d_n7, assign3860_e2632_d_n12,) = {
    if (var_guard33 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_vbcs, var_vbcs_dn7, var_vbcs_dn12,)
    }
};
        var_vbcs = assign3860_e2632;
        var_vbcs_dn7 = assign3860_e2632_d_n7;
        var_vbcs_dn12 = assign3860_e2632_d_n12;

        let (assign3870_e2643, assign3870_e2643_d_n0, assign3870_e2643_d_n2, assign3870_e2643_d_n6, assign3870_e2643_d_n7, assign3870_e2643_d_n10, assign3870_e2643_d_n11, assign3870_e2643_d_n12, assign3870_e2643_d_n15, assign3870_e2643_d_n17, assign3870_e2643_d_n18,) = {
    if ((var_guard33 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign3870_e2639: f64 = (1e-9 / 0.0001);
        let assign3870_e2641: f64 = (assign3870_e2639 * (nv15 - 0.0));
        (assign3870_e2641, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign3870_e2639, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign3870_e2643;
        var_qd_nqs_dn0 = assign3870_e2643_d_n0;
        var_qd_nqs_dn2 = assign3870_e2643_d_n2;
        var_qd_nqs_dn6 = assign3870_e2643_d_n6;
        var_qd_nqs_dn7 = assign3870_e2643_d_n7;
        var_qd_nqs_dn10 = assign3870_e2643_d_n10;
        var_qd_nqs_dn11 = assign3870_e2643_d_n11;
        var_qd_nqs_dn12 = assign3870_e2643_d_n12;
        var_qd_nqs_dn15 = assign3870_e2643_d_n15;
        var_qd_nqs_dn17 = assign3870_e2643_d_n17;
        var_qd_nqs_dn18 = assign3870_e2643_d_n18;

        let (assign3880_e2654, assign3880_e2654_d_n0, assign3880_e2654_d_n2, assign3880_e2654_d_n6, assign3880_e2654_d_n7, assign3880_e2654_d_n10, assign3880_e2654_d_n11, assign3880_e2654_d_n12, assign3880_e2654_d_n16, assign3880_e2654_d_n17, assign3880_e2654_d_n18,) = {
    if ((var_guard33 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign3880_e2650: f64 = (1e-9 / 0.0001);
        let assign3880_e2652: f64 = (assign3880_e2650 * (nv16 - 0.0));
        (assign3880_e2652, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign3880_e2650, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign3880_e2654;
        var_qs_nqs_dn0 = assign3880_e2654_d_n0;
        var_qs_nqs_dn2 = assign3880_e2654_d_n2;
        var_qs_nqs_dn6 = assign3880_e2654_d_n6;
        var_qs_nqs_dn7 = assign3880_e2654_d_n7;
        var_qs_nqs_dn10 = assign3880_e2654_d_n10;
        var_qs_nqs_dn11 = assign3880_e2654_d_n11;
        var_qs_nqs_dn12 = assign3880_e2654_d_n12;
        var_qs_nqs_dn16 = assign3880_e2654_d_n16;
        var_qs_nqs_dn17 = assign3880_e2654_d_n17;
        var_qs_nqs_dn18 = assign3880_e2654_d_n18;

        let (assign3890_e2665, assign3890_e2665_d_n13,) = {
    if ((var_guard33 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign3890_e2661: f64 = (1e-9 / 0.0001);
        let assign3890_e2663: f64 = (assign3890_e2661 * (nv13 - 0.0));
        (assign3890_e2663, assign3890_e2661,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign3890_e2665;
        var_qb_nqs_dn13 = assign3890_e2665_d_n13;

        let (assign3900_e2673, assign3900_e2673_d_n0, assign3900_e2673_d_n2, assign3900_e2673_d_n6, assign3900_e2673_d_n7, assign3900_e2673_d_n10, assign3900_e2673_d_n11, assign3900_e2673_d_n12, assign3900_e2673_d_n15, assign3900_e2673_d_n17, assign3900_e2673_d_n18,) = {
    if ((var_guard33 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign3900_e2673;
        var_qd_nqs_dn0 = assign3900_e2673_d_n0;
        var_qd_nqs_dn2 = assign3900_e2673_d_n2;
        var_qd_nqs_dn6 = assign3900_e2673_d_n6;
        var_qd_nqs_dn7 = assign3900_e2673_d_n7;
        var_qd_nqs_dn10 = assign3900_e2673_d_n10;
        var_qd_nqs_dn11 = assign3900_e2673_d_n11;
        var_qd_nqs_dn12 = assign3900_e2673_d_n12;
        var_qd_nqs_dn15 = assign3900_e2673_d_n15;
        var_qd_nqs_dn17 = assign3900_e2673_d_n17;
        var_qd_nqs_dn18 = assign3900_e2673_d_n18;

        let (assign3910_e2681, assign3910_e2681_d_n0, assign3910_e2681_d_n2, assign3910_e2681_d_n6, assign3910_e2681_d_n7, assign3910_e2681_d_n10, assign3910_e2681_d_n11, assign3910_e2681_d_n12, assign3910_e2681_d_n16, assign3910_e2681_d_n17, assign3910_e2681_d_n18,) = {
    if ((var_guard33 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign3910_e2681;
        var_qs_nqs_dn0 = assign3910_e2681_d_n0;
        var_qs_nqs_dn2 = assign3910_e2681_d_n2;
        var_qs_nqs_dn6 = assign3910_e2681_d_n6;
        var_qs_nqs_dn7 = assign3910_e2681_d_n7;
        var_qs_nqs_dn10 = assign3910_e2681_d_n10;
        var_qs_nqs_dn11 = assign3910_e2681_d_n11;
        var_qs_nqs_dn12 = assign3910_e2681_d_n12;
        var_qs_nqs_dn16 = assign3910_e2681_d_n16;
        var_qs_nqs_dn17 = assign3910_e2681_d_n17;
        var_qs_nqs_dn18 = assign3910_e2681_d_n18;

        let (assign3920_e2689, assign3920_e2689_d_n13,) = {
    if ((var_guard33 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign3920_e2689;
        var_qb_nqs_dn13 = assign3920_e2689_d_n13;

        let (assign3930_e2704, assign3930_e2704_d_n10,) = {
    if ((p.p38 > 0.0) && (var_mks_rth0 > 0.0)) {
        let (assign3930_e2702, assign3930_e2702_d_n10,) = {
            if ((nv10 - 0.0) > 0.0) {
                ((nv10 - 0.0), 1.0,)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign3930_e2702, assign3930_e2702_d_n10,)
    } else {
        (0.0, 0.0,)
    }
};
        var_deltemp = assign3930_e2704;
        var_deltemp_dn10 = assign3930_e2704_d_n10;

        let assign3940_e2707: f64 = if var_vdsi >= 0.0 { 1.0 } else { 0.0 };
        var_guard34 = assign3940_e2707;

        let (assign3950_e2711,) = {
    if (var_guard34 != 0.0) {
        (1.0,)
    } else {
        (var_mode,)
    }
};
        var_mode = assign3950_e2711;

        let (assign3960_e2715,) = {
    if (var_guard34 != 0.0) {
        (1.0,)
    } else {
        (var_modenml,)
    }
};
        var_modenml = assign3960_e2715;

        *var_area_bt_n_slot = var_area_bt_n;
        *var_area_bt_p_slot = var_area_bt_p;
        *var_cbtn_slot = var_cbtn;
        *var_cbtp_slot = var_cbtp;
        *var_deltemp_slot = var_deltemp;
        *var_deltemp_dn10_slot = var_deltemp_dn10;
        *var_guard29_slot = var_guard29;
        *var_guard30_slot = var_guard30;
        *var_guard31_slot = var_guard31;
        *var_guard32_slot = var_guard32;
        *var_guard33_slot = var_guard33;
        *var_guard34_slot = var_guard34;
        *var_mode_slot = var_mode;
        *var_modenml_slot = var_modenml;
        *var_peri_hhi_slot = var_peri_hhi;
        *var_qb_nqs_slot = var_qb_nqs;
        *var_qb_nqs_dn13_slot = var_qb_nqs_dn13;
        *var_qd_nqs_slot = var_qd_nqs;
        *var_qd_nqs_dn0_slot = var_qd_nqs_dn0;
        *var_qd_nqs_dn10_slot = var_qd_nqs_dn10;
        *var_qd_nqs_dn11_slot = var_qd_nqs_dn11;
        *var_qd_nqs_dn12_slot = var_qd_nqs_dn12;
        *var_qd_nqs_dn15_slot = var_qd_nqs_dn15;
        *var_qd_nqs_dn17_slot = var_qd_nqs_dn17;
        *var_qd_nqs_dn18_slot = var_qd_nqs_dn18;
        *var_qd_nqs_dn2_slot = var_qd_nqs_dn2;
        *var_qd_nqs_dn6_slot = var_qd_nqs_dn6;
        *var_qd_nqs_dn7_slot = var_qd_nqs_dn7;
        *var_qi_nqs_slot = var_qi_nqs;
        *var_qi_nqs_dn18_slot = var_qi_nqs_dn18;
        *var_qs_nqs_slot = var_qs_nqs;
        *var_qs_nqs_dn0_slot = var_qs_nqs_dn0;
        *var_qs_nqs_dn10_slot = var_qs_nqs_dn10;
        *var_qs_nqs_dn11_slot = var_qs_nqs_dn11;
        *var_qs_nqs_dn12_slot = var_qs_nqs_dn12;
        *var_qs_nqs_dn16_slot = var_qs_nqs_dn16;
        *var_qs_nqs_dn17_slot = var_qs_nqs_dn17;
        *var_qs_nqs_dn18_slot = var_qs_nqs_dn18;
        *var_qs_nqs_dn2_slot = var_qs_nqs_dn2;
        *var_qs_nqs_dn6_slot = var_qs_nqs_dn6;
        *var_qs_nqs_dn7_slot = var_qs_nqs_dn7;
        *var_uc_pdbcp_slot = var_uc_pdbcp;
        *var_uc_psbcp_slot = var_uc_psbcp;
        *var_vbcd_slot = var_vbcd;
        *var_vbcd_dn12_slot = var_vbcd_dn12;
        *var_vbcd_dn6_slot = var_vbcd_dn6;
        *var_vbcs_slot = var_vbcs;
        *var_vbcs_dn12_slot = var_vbcs_dn12;
        *var_vbcs_dn7_slot = var_vbcs_dn7;
        *var_vbsi_slot = var_vbsi;
        *var_vbsi_dn12_slot = var_vbsi_dn12;
        *var_vbsi_dn7_slot = var_vbsi_dn7;
        *var_vdsi_slot = var_vdsi;
        *var_vdsi_dn6_slot = var_vdsi_dn6;
        *var_vdsi_dn7_slot = var_vdsi_dn7;
        *var_vgsi_slot = var_vgsi;
        *var_vgsi_dn11_slot = var_vgsi_dn11;
        *var_vgsi_dn7_slot = var_vgsi_dn7;
        *var_w_diod_slot = var_w_diod;
        *var_w_diodcv_slot = var_w_diodcv;
        *var_w_dios_slot = var_w_dios;
        *var_w_dioscv_slot = var_w_dioscv;
    }

    pub(super) fn stamp_transient_block_7(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        var_betatnom: f64,
        var_c0bulk: f64,
        var_costi00: f64,
        var_deltemp: f64,
        var_deltemp_dn10: f64,
        var_dtemp_given: f64,
        var_egtnom: f64,
        var_guard34: f64,
        var_lgate: f64,
        var_lgle: f64,
        var_mks_vmax: f64,
        var_mks_vtmp: f64,
        var_nsti_p2: f64,
        var_nsub: f64,
        var_nsub_dn0: f64,
        var_nsub_dn10: f64,
        var_nsub_dn11: f64,
        var_nsub_dn12: f64,
        var_nsub_dn17: f64,
        var_nsub_dn2: f64,
        var_nsub_dn6: f64,
        var_nsub_dn7: f64,
        var_ptovr0: f64,
        var_ptovr0_dn0: f64,
        var_ptovr0_dn10: f64,
        var_ptovr0_dn11: f64,
        var_ptovr0_dn12: f64,
        var_ptovr0_dn17: f64,
        var_ptovr0_dn2: f64,
        var_ptovr0_dn6: f64,
        var_ptovr0_dn7: f64,
        var_q_nsub: f64,
        var_q_nsub_dn0: f64,
        var_q_nsub_dn10: f64,
        var_q_nsub_dn11: f64,
        var_q_nsub_dn12: f64,
        var_q_nsub_dn17: f64,
        var_q_nsub_dn2: f64,
        var_q_nsub_dn6: f64,
        var_q_nsub_dn7: f64,
        var_subversion: f64,
        var_temp_given: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn17: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn7: f64,
        var_uc_temp: f64,
        var_uc_tnom: f64,
        var_vbsi: f64,
        var_vbsi_dn12: f64,
        var_vbsi_dn7: f64,
        var_vdsi: f64,
        var_vdsi_dn6: f64,
        var_vdsi_dn7: f64,
        var_vgsi: f64,
        var_vgsi_dn11: f64,
        var_vgsi_dn7: f64,
        var_vmax0: f64,
        var_vmax0_dn0: f64,
        var_vmax0_dn10: f64,
        var_vmax0_dn11: f64,
        var_vmax0_dn12: f64,
        var_vmax0_dn17: f64,
        var_vmax0_dn2: f64,
        var_vmax0_dn6: f64,
        var_vmax0_dn7: f64,
        var_wg: f64,
        var_wl: f64,
        var_beta_slot: &mut f64,
        var_beta2_slot: &mut f64,
        var_beta2_dn10_slot: &mut f64,
        var_beta_dn10_slot: &mut f64,
        var_beta_inv_slot: &mut f64,
        var_beta_inv_dn10_slot: &mut f64,
        var_cgs_mphn0_slot: &mut f64,
        var_cgs_mphn0_dn10_slot: &mut f64,
        var_cgs_mueph_slot: &mut f64,
        var_cgs_wmueph_slot: &mut f64,
        var_cnst0bulk_slot: &mut f64,
        var_cnst0bulk_dn10_slot: &mut f64,
        var_cnst0soi_slot: &mut f64,
        var_cnst0soi_dn0_slot: &mut f64,
        var_cnst0soi_dn10_slot: &mut f64,
        var_cnst0soi_dn11_slot: &mut f64,
        var_cnst0soi_dn12_slot: &mut f64,
        var_cnst0soi_dn17_slot: &mut f64,
        var_cnst0soi_dn2_slot: &mut f64,
        var_cnst0soi_dn6_slot: &mut f64,
        var_cnst0soi_dn7_slot: &mut f64,
        var_cnst1bulk_slot: &mut f64,
        var_cnst1bulk_dn0_slot: &mut f64,
        var_cnst1bulk_dn10_slot: &mut f64,
        var_cnst1bulk_dn11_slot: &mut f64,
        var_cnst1bulk_dn12_slot: &mut f64,
        var_cnst1bulk_dn17_slot: &mut f64,
        var_cnst1bulk_dn2_slot: &mut f64,
        var_cnst1bulk_dn6_slot: &mut f64,
        var_cnst1bulk_dn7_slot: &mut f64,
        var_costi0_slot: &mut f64,
        var_costi0_dn0_slot: &mut f64,
        var_costi0_dn10_slot: &mut f64,
        var_costi0_dn11_slot: &mut f64,
        var_costi0_dn12_slot: &mut f64,
        var_costi0_dn17_slot: &mut f64,
        var_costi0_dn2_slot: &mut f64,
        var_costi0_dn6_slot: &mut f64,
        var_costi0_dn7_slot: &mut f64,
        var_costi0_p2_slot: &mut f64,
        var_costi0_p2_dn0_slot: &mut f64,
        var_costi0_p2_dn10_slot: &mut f64,
        var_costi0_p2_dn11_slot: &mut f64,
        var_costi0_p2_dn12_slot: &mut f64,
        var_costi0_p2_dn17_slot: &mut f64,
        var_costi0_p2_dn2_slot: &mut f64,
        var_costi0_p2_dn6_slot: &mut f64,
        var_costi0_p2_dn7_slot: &mut f64,
        var_costi1_slot: &mut f64,
        var_costi1_dn0_slot: &mut f64,
        var_costi1_dn10_slot: &mut f64,
        var_costi1_dn11_slot: &mut f64,
        var_costi1_dn12_slot: &mut f64,
        var_costi1_dn17_slot: &mut f64,
        var_costi1_dn2_slot: &mut f64,
        var_costi1_dn6_slot: &mut f64,
        var_costi1_dn7_slot: &mut f64,
        var_eg_slot: &mut f64,
        var_eg_dn0_slot: &mut f64,
        var_eg_dn10_slot: &mut f64,
        var_eg_dn11_slot: &mut f64,
        var_eg_dn12_slot: &mut f64,
        var_eg_dn17_slot: &mut f64,
        var_eg_dn2_slot: &mut f64,
        var_eg_dn6_slot: &mut f64,
        var_eg_dn7_slot: &mut f64,
        var_egp12_slot: &mut f64,
        var_egp12_dn0_slot: &mut f64,
        var_egp12_dn10_slot: &mut f64,
        var_egp12_dn11_slot: &mut f64,
        var_egp12_dn12_slot: &mut f64,
        var_egp12_dn17_slot: &mut f64,
        var_egp12_dn2_slot: &mut f64,
        var_egp12_dn6_slot: &mut f64,
        var_egp12_dn7_slot: &mut f64,
        var_egp32_slot: &mut f64,
        var_egp32_dn0_slot: &mut f64,
        var_egp32_dn10_slot: &mut f64,
        var_egp32_dn11_slot: &mut f64,
        var_egp32_dn12_slot: &mut f64,
        var_egp32_dn17_slot: &mut f64,
        var_egp32_dn2_slot: &mut f64,
        var_egp32_dn6_slot: &mut f64,
        var_egp32_dn7_slot: &mut f64,
        var_guard42_slot: &mut f64,
        var_guard43_slot: &mut f64,
        var_lch_slot: &mut f64,
        var_lch_dn0_slot: &mut f64,
        var_lch_dn10_slot: &mut f64,
        var_lch_dn11_slot: &mut f64,
        var_lch_dn12_slot: &mut f64,
        var_lch_dn17_slot: &mut f64,
        var_lch_dn2_slot: &mut f64,
        var_lch_dn6_slot: &mut f64,
        var_lch_dn7_slot: &mut f64,
        var_ldby_slot: &mut f64,
        var_ldby_dn0_slot: &mut f64,
        var_ldby_dn10_slot: &mut f64,
        var_ldby_dn11_slot: &mut f64,
        var_ldby_dn12_slot: &mut f64,
        var_ldby_dn17_slot: &mut f64,
        var_ldby_dn2_slot: &mut f64,
        var_ldby_dn6_slot: &mut f64,
        var_ldby_dn7_slot: &mut f64,
        var_mode_slot: &mut f64,
        var_modenml_slot: &mut f64,
        var_modervs_slot: &mut f64,
        var_nin_slot: &mut f64,
        var_nin_dn0_slot: &mut f64,
        var_nin_dn10_slot: &mut f64,
        var_nin_dn11_slot: &mut f64,
        var_nin_dn12_slot: &mut f64,
        var_nin_dn17_slot: &mut f64,
        var_nin_dn2_slot: &mut f64,
        var_nin_dn6_slot: &mut f64,
        var_nin_dn7_slot: &mut f64,
        var_pb2_slot: &mut f64,
        var_pb2_dn0_slot: &mut f64,
        var_pb2_dn10_slot: &mut f64,
        var_pb2_dn11_slot: &mut f64,
        var_pb2_dn12_slot: &mut f64,
        var_pb2_dn17_slot: &mut f64,
        var_pb2_dn2_slot: &mut f64,
        var_pb2_dn6_slot: &mut f64,
        var_pb2_dn7_slot: &mut f64,
        var_ptovr_slot: &mut f64,
        var_ptovr_dn0_slot: &mut f64,
        var_ptovr_dn10_slot: &mut f64,
        var_ptovr_dn11_slot: &mut f64,
        var_ptovr_dn12_slot: &mut f64,
        var_ptovr_dn17_slot: &mut f64,
        var_ptovr_dn2_slot: &mut f64,
        var_ptovr_dn6_slot: &mut f64,
        var_ptovr_dn7_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1__blk39_slot: &mut f64,
        var_t1__blk39_dn10_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2__blk40_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t3__blk41_slot: &mut f64,
        var_ttemp_slot: &mut f64,
        var_ttemp_dn10_slot: &mut f64,
        var_vbs_slot: &mut f64,
        var_vbs_dn0_slot: &mut f64,
        var_vbs_dn10_slot: &mut f64,
        var_vbs_dn11_slot: &mut f64,
        var_vbs_dn12_slot: &mut f64,
        var_vbs_dn17_slot: &mut f64,
        var_vbs_dn2_slot: &mut f64,
        var_vbs_dn6_slot: &mut f64,
        var_vbs_dn7_slot: &mut f64,
        var_vds_slot: &mut f64,
        var_vds_dn0_slot: &mut f64,
        var_vds_dn10_slot: &mut f64,
        var_vds_dn11_slot: &mut f64,
        var_vds_dn12_slot: &mut f64,
        var_vds_dn17_slot: &mut f64,
        var_vds_dn2_slot: &mut f64,
        var_vds_dn6_slot: &mut f64,
        var_vds_dn7_slot: &mut f64,
        var_vgs_slot: &mut f64,
        var_vgs_dn11_slot: &mut f64,
        var_vgs_dn6_slot: &mut f64,
        var_vgs_dn7_slot: &mut f64,
        var_vmaxe_slot: &mut f64,
        var_vmaxe_dn0_slot: &mut f64,
        var_vmaxe_dn10_slot: &mut f64,
        var_vmaxe_dn11_slot: &mut f64,
        var_vmaxe_dn12_slot: &mut f64,
        var_vmaxe_dn17_slot: &mut f64,
        var_vmaxe_dn2_slot: &mut f64,
        var_vmaxe_dn6_slot: &mut f64,
        var_vmaxe_dn7_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let mut var_beta: f64 = *var_beta_slot;
        let mut var_beta2: f64 = *var_beta2_slot;
        let mut var_beta2_dn10: f64 = *var_beta2_dn10_slot;
        let mut var_beta_dn10: f64 = *var_beta_dn10_slot;
        let mut var_beta_inv: f64 = *var_beta_inv_slot;
        let mut var_beta_inv_dn10: f64 = *var_beta_inv_dn10_slot;
        let mut var_cgs_mphn0: f64 = *var_cgs_mphn0_slot;
        let mut var_cgs_mphn0_dn10: f64 = *var_cgs_mphn0_dn10_slot;
        let mut var_cgs_mueph: f64 = *var_cgs_mueph_slot;
        let mut var_cgs_wmueph: f64 = *var_cgs_wmueph_slot;
        let mut var_cnst0bulk: f64 = *var_cnst0bulk_slot;
        let mut var_cnst0bulk_dn10: f64 = *var_cnst0bulk_dn10_slot;
        let mut var_cnst0soi: f64 = *var_cnst0soi_slot;
        let mut var_cnst0soi_dn0: f64 = *var_cnst0soi_dn0_slot;
        let mut var_cnst0soi_dn10: f64 = *var_cnst0soi_dn10_slot;
        let mut var_cnst0soi_dn11: f64 = *var_cnst0soi_dn11_slot;
        let mut var_cnst0soi_dn12: f64 = *var_cnst0soi_dn12_slot;
        let mut var_cnst0soi_dn17: f64 = *var_cnst0soi_dn17_slot;
        let mut var_cnst0soi_dn2: f64 = *var_cnst0soi_dn2_slot;
        let mut var_cnst0soi_dn6: f64 = *var_cnst0soi_dn6_slot;
        let mut var_cnst0soi_dn7: f64 = *var_cnst0soi_dn7_slot;
        let mut var_cnst1bulk: f64 = *var_cnst1bulk_slot;
        let mut var_cnst1bulk_dn0: f64 = *var_cnst1bulk_dn0_slot;
        let mut var_cnst1bulk_dn10: f64 = *var_cnst1bulk_dn10_slot;
        let mut var_cnst1bulk_dn11: f64 = *var_cnst1bulk_dn11_slot;
        let mut var_cnst1bulk_dn12: f64 = *var_cnst1bulk_dn12_slot;
        let mut var_cnst1bulk_dn17: f64 = *var_cnst1bulk_dn17_slot;
        let mut var_cnst1bulk_dn2: f64 = *var_cnst1bulk_dn2_slot;
        let mut var_cnst1bulk_dn6: f64 = *var_cnst1bulk_dn6_slot;
        let mut var_cnst1bulk_dn7: f64 = *var_cnst1bulk_dn7_slot;
        let mut var_costi0: f64 = *var_costi0_slot;
        let mut var_costi0_dn0: f64 = *var_costi0_dn0_slot;
        let mut var_costi0_dn10: f64 = *var_costi0_dn10_slot;
        let mut var_costi0_dn11: f64 = *var_costi0_dn11_slot;
        let mut var_costi0_dn12: f64 = *var_costi0_dn12_slot;
        let mut var_costi0_dn17: f64 = *var_costi0_dn17_slot;
        let mut var_costi0_dn2: f64 = *var_costi0_dn2_slot;
        let mut var_costi0_dn6: f64 = *var_costi0_dn6_slot;
        let mut var_costi0_dn7: f64 = *var_costi0_dn7_slot;
        let mut var_costi0_p2: f64 = *var_costi0_p2_slot;
        let mut var_costi0_p2_dn0: f64 = *var_costi0_p2_dn0_slot;
        let mut var_costi0_p2_dn10: f64 = *var_costi0_p2_dn10_slot;
        let mut var_costi0_p2_dn11: f64 = *var_costi0_p2_dn11_slot;
        let mut var_costi0_p2_dn12: f64 = *var_costi0_p2_dn12_slot;
        let mut var_costi0_p2_dn17: f64 = *var_costi0_p2_dn17_slot;
        let mut var_costi0_p2_dn2: f64 = *var_costi0_p2_dn2_slot;
        let mut var_costi0_p2_dn6: f64 = *var_costi0_p2_dn6_slot;
        let mut var_costi0_p2_dn7: f64 = *var_costi0_p2_dn7_slot;
        let mut var_costi1: f64 = *var_costi1_slot;
        let mut var_costi1_dn0: f64 = *var_costi1_dn0_slot;
        let mut var_costi1_dn10: f64 = *var_costi1_dn10_slot;
        let mut var_costi1_dn11: f64 = *var_costi1_dn11_slot;
        let mut var_costi1_dn12: f64 = *var_costi1_dn12_slot;
        let mut var_costi1_dn17: f64 = *var_costi1_dn17_slot;
        let mut var_costi1_dn2: f64 = *var_costi1_dn2_slot;
        let mut var_costi1_dn6: f64 = *var_costi1_dn6_slot;
        let mut var_costi1_dn7: f64 = *var_costi1_dn7_slot;
        let mut var_eg: f64 = *var_eg_slot;
        let mut var_eg_dn0: f64 = *var_eg_dn0_slot;
        let mut var_eg_dn10: f64 = *var_eg_dn10_slot;
        let mut var_eg_dn11: f64 = *var_eg_dn11_slot;
        let mut var_eg_dn12: f64 = *var_eg_dn12_slot;
        let mut var_eg_dn17: f64 = *var_eg_dn17_slot;
        let mut var_eg_dn2: f64 = *var_eg_dn2_slot;
        let mut var_eg_dn6: f64 = *var_eg_dn6_slot;
        let mut var_eg_dn7: f64 = *var_eg_dn7_slot;
        let mut var_egp12: f64 = *var_egp12_slot;
        let mut var_egp12_dn0: f64 = *var_egp12_dn0_slot;
        let mut var_egp12_dn10: f64 = *var_egp12_dn10_slot;
        let mut var_egp12_dn11: f64 = *var_egp12_dn11_slot;
        let mut var_egp12_dn12: f64 = *var_egp12_dn12_slot;
        let mut var_egp12_dn17: f64 = *var_egp12_dn17_slot;
        let mut var_egp12_dn2: f64 = *var_egp12_dn2_slot;
        let mut var_egp12_dn6: f64 = *var_egp12_dn6_slot;
        let mut var_egp12_dn7: f64 = *var_egp12_dn7_slot;
        let mut var_egp32: f64 = *var_egp32_slot;
        let mut var_egp32_dn0: f64 = *var_egp32_dn0_slot;
        let mut var_egp32_dn10: f64 = *var_egp32_dn10_slot;
        let mut var_egp32_dn11: f64 = *var_egp32_dn11_slot;
        let mut var_egp32_dn12: f64 = *var_egp32_dn12_slot;
        let mut var_egp32_dn17: f64 = *var_egp32_dn17_slot;
        let mut var_egp32_dn2: f64 = *var_egp32_dn2_slot;
        let mut var_egp32_dn6: f64 = *var_egp32_dn6_slot;
        let mut var_egp32_dn7: f64 = *var_egp32_dn7_slot;
        let mut var_guard42: f64 = *var_guard42_slot;
        let mut var_guard43: f64 = *var_guard43_slot;
        let mut var_lch: f64 = *var_lch_slot;
        let mut var_lch_dn0: f64 = *var_lch_dn0_slot;
        let mut var_lch_dn10: f64 = *var_lch_dn10_slot;
        let mut var_lch_dn11: f64 = *var_lch_dn11_slot;
        let mut var_lch_dn12: f64 = *var_lch_dn12_slot;
        let mut var_lch_dn17: f64 = *var_lch_dn17_slot;
        let mut var_lch_dn2: f64 = *var_lch_dn2_slot;
        let mut var_lch_dn6: f64 = *var_lch_dn6_slot;
        let mut var_lch_dn7: f64 = *var_lch_dn7_slot;
        let mut var_ldby: f64 = *var_ldby_slot;
        let mut var_ldby_dn0: f64 = *var_ldby_dn0_slot;
        let mut var_ldby_dn10: f64 = *var_ldby_dn10_slot;
        let mut var_ldby_dn11: f64 = *var_ldby_dn11_slot;
        let mut var_ldby_dn12: f64 = *var_ldby_dn12_slot;
        let mut var_ldby_dn17: f64 = *var_ldby_dn17_slot;
        let mut var_ldby_dn2: f64 = *var_ldby_dn2_slot;
        let mut var_ldby_dn6: f64 = *var_ldby_dn6_slot;
        let mut var_ldby_dn7: f64 = *var_ldby_dn7_slot;
        let mut var_mode: f64 = *var_mode_slot;
        let mut var_modenml: f64 = *var_modenml_slot;
        let mut var_modervs: f64 = *var_modervs_slot;
        let mut var_nin: f64 = *var_nin_slot;
        let mut var_nin_dn0: f64 = *var_nin_dn0_slot;
        let mut var_nin_dn10: f64 = *var_nin_dn10_slot;
        let mut var_nin_dn11: f64 = *var_nin_dn11_slot;
        let mut var_nin_dn12: f64 = *var_nin_dn12_slot;
        let mut var_nin_dn17: f64 = *var_nin_dn17_slot;
        let mut var_nin_dn2: f64 = *var_nin_dn2_slot;
        let mut var_nin_dn6: f64 = *var_nin_dn6_slot;
        let mut var_nin_dn7: f64 = *var_nin_dn7_slot;
        let mut var_pb2: f64 = *var_pb2_slot;
        let mut var_pb2_dn0: f64 = *var_pb2_dn0_slot;
        let mut var_pb2_dn10: f64 = *var_pb2_dn10_slot;
        let mut var_pb2_dn11: f64 = *var_pb2_dn11_slot;
        let mut var_pb2_dn12: f64 = *var_pb2_dn12_slot;
        let mut var_pb2_dn17: f64 = *var_pb2_dn17_slot;
        let mut var_pb2_dn2: f64 = *var_pb2_dn2_slot;
        let mut var_pb2_dn6: f64 = *var_pb2_dn6_slot;
        let mut var_pb2_dn7: f64 = *var_pb2_dn7_slot;
        let mut var_ptovr: f64 = *var_ptovr_slot;
        let mut var_ptovr_dn0: f64 = *var_ptovr_dn0_slot;
        let mut var_ptovr_dn10: f64 = *var_ptovr_dn10_slot;
        let mut var_ptovr_dn11: f64 = *var_ptovr_dn11_slot;
        let mut var_ptovr_dn12: f64 = *var_ptovr_dn12_slot;
        let mut var_ptovr_dn17: f64 = *var_ptovr_dn17_slot;
        let mut var_ptovr_dn2: f64 = *var_ptovr_dn2_slot;
        let mut var_ptovr_dn6: f64 = *var_ptovr_dn6_slot;
        let mut var_ptovr_dn7: f64 = *var_ptovr_dn7_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1__blk39: f64 = *var_t1__blk39_slot;
        let mut var_t1__blk39_dn10: f64 = *var_t1__blk39_dn10_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2__blk40: f64 = *var_t2__blk40_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t3__blk41: f64 = *var_t3__blk41_slot;
        let mut var_ttemp: f64 = *var_ttemp_slot;
        let mut var_ttemp_dn10: f64 = *var_ttemp_dn10_slot;
        let mut var_vbs: f64 = *var_vbs_slot;
        let mut var_vbs_dn0: f64 = *var_vbs_dn0_slot;
        let mut var_vbs_dn10: f64 = *var_vbs_dn10_slot;
        let mut var_vbs_dn11: f64 = *var_vbs_dn11_slot;
        let mut var_vbs_dn12: f64 = *var_vbs_dn12_slot;
        let mut var_vbs_dn17: f64 = *var_vbs_dn17_slot;
        let mut var_vbs_dn2: f64 = *var_vbs_dn2_slot;
        let mut var_vbs_dn6: f64 = *var_vbs_dn6_slot;
        let mut var_vbs_dn7: f64 = *var_vbs_dn7_slot;
        let mut var_vds: f64 = *var_vds_slot;
        let mut var_vds_dn0: f64 = *var_vds_dn0_slot;
        let mut var_vds_dn10: f64 = *var_vds_dn10_slot;
        let mut var_vds_dn11: f64 = *var_vds_dn11_slot;
        let mut var_vds_dn12: f64 = *var_vds_dn12_slot;
        let mut var_vds_dn17: f64 = *var_vds_dn17_slot;
        let mut var_vds_dn2: f64 = *var_vds_dn2_slot;
        let mut var_vds_dn6: f64 = *var_vds_dn6_slot;
        let mut var_vds_dn7: f64 = *var_vds_dn7_slot;
        let mut var_vgs: f64 = *var_vgs_slot;
        let mut var_vgs_dn11: f64 = *var_vgs_dn11_slot;
        let mut var_vgs_dn6: f64 = *var_vgs_dn6_slot;
        let mut var_vgs_dn7: f64 = *var_vgs_dn7_slot;
        let mut var_vmaxe: f64 = *var_vmaxe_slot;
        let mut var_vmaxe_dn0: f64 = *var_vmaxe_dn0_slot;
        let mut var_vmaxe_dn10: f64 = *var_vmaxe_dn10_slot;
        let mut var_vmaxe_dn11: f64 = *var_vmaxe_dn11_slot;
        let mut var_vmaxe_dn12: f64 = *var_vmaxe_dn12_slot;
        let mut var_vmaxe_dn17: f64 = *var_vmaxe_dn17_slot;
        let mut var_vmaxe_dn2: f64 = *var_vmaxe_dn2_slot;
        let mut var_vmaxe_dn6: f64 = *var_vmaxe_dn6_slot;
        let mut var_vmaxe_dn7: f64 = *var_vmaxe_dn7_slot;

        let (assign3970_e2719,) = {
    if (var_guard34 != 0.0) {
        (0.0,)
    } else {
        (var_modervs,)
    }
};
        var_modervs = assign3970_e2719;

        let (assign3980_e2723, assign3980_e2723_d_n0, assign3980_e2723_d_n2, assign3980_e2723_d_n6, assign3980_e2723_d_n7, assign3980_e2723_d_n10, assign3980_e2723_d_n11, assign3980_e2723_d_n12, assign3980_e2723_d_n17,) = {
    if (var_guard34 != 0.0) {
        (var_vdsi, 0.0, 0.0, var_vdsi_dn6, var_vdsi_dn7, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vds, var_vds_dn0, var_vds_dn2, var_vds_dn6, var_vds_dn7, var_vds_dn10, var_vds_dn11, var_vds_dn12, var_vds_dn17,)
    }
};
        var_vds = assign3980_e2723;
        var_vds_dn0 = assign3980_e2723_d_n0;
        var_vds_dn2 = assign3980_e2723_d_n2;
        var_vds_dn6 = assign3980_e2723_d_n6;
        var_vds_dn7 = assign3980_e2723_d_n7;
        var_vds_dn10 = assign3980_e2723_d_n10;
        var_vds_dn11 = assign3980_e2723_d_n11;
        var_vds_dn12 = assign3980_e2723_d_n12;
        var_vds_dn17 = assign3980_e2723_d_n17;

        let (assign3990_e2727, assign3990_e2727_d_n6, assign3990_e2727_d_n7, assign3990_e2727_d_n11,) = {
    if (var_guard34 != 0.0) {
        (var_vgsi, 0.0, var_vgsi_dn7, var_vgsi_dn11,)
    } else {
        (var_vgs, var_vgs_dn6, var_vgs_dn7, var_vgs_dn11,)
    }
};
        var_vgs = assign3990_e2727;
        var_vgs_dn6 = assign3990_e2727_d_n6;
        var_vgs_dn7 = assign3990_e2727_d_n7;
        var_vgs_dn11 = assign3990_e2727_d_n11;

        let (assign4000_e2731, assign4000_e2731_d_n0, assign4000_e2731_d_n2, assign4000_e2731_d_n6, assign4000_e2731_d_n7, assign4000_e2731_d_n10, assign4000_e2731_d_n11, assign4000_e2731_d_n12, assign4000_e2731_d_n17,) = {
    if (var_guard34 != 0.0) {
        (var_vbsi, 0.0, 0.0, 0.0, var_vbsi_dn7, 0.0, 0.0, var_vbsi_dn12, 0.0,)
    } else {
        (var_vbs, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
    }
};
        var_vbs = assign4000_e2731;
        var_vbs_dn0 = assign4000_e2731_d_n0;
        var_vbs_dn2 = assign4000_e2731_d_n2;
        var_vbs_dn6 = assign4000_e2731_d_n6;
        var_vbs_dn7 = assign4000_e2731_d_n7;
        var_vbs_dn10 = assign4000_e2731_d_n10;
        var_vbs_dn11 = assign4000_e2731_d_n11;
        var_vbs_dn12 = assign4000_e2731_d_n12;
        var_vbs_dn17 = assign4000_e2731_d_n17;

        let (assign4040_e2749,) = {
    if (var_guard34 == 0.0) {
        let assign4040_e2747: f64 = (-1.0);
        (assign4040_e2747,)
    } else {
        (var_mode,)
    }
};
        var_mode = assign4040_e2749;

        let (assign4050_e2754,) = {
    if (var_guard34 == 0.0) {
        (0.0,)
    } else {
        (var_modenml,)
    }
};
        var_modenml = assign4050_e2754;

        let (assign4060_e2759,) = {
    if (var_guard34 == 0.0) {
        (1.0,)
    } else {
        (var_modervs,)
    }
};
        var_modervs = assign4060_e2759;

        let (assign4070_e2765, assign4070_e2765_d_n0, assign4070_e2765_d_n2, assign4070_e2765_d_n6, assign4070_e2765_d_n7, assign4070_e2765_d_n10, assign4070_e2765_d_n11, assign4070_e2765_d_n12, assign4070_e2765_d_n17,) = {
    if (var_guard34 == 0.0) {
        let assign4070_e2763: f64 = (-var_vdsi);
        (assign4070_e2763, 0.0, 0.0, (-var_vdsi_dn6), (-var_vdsi_dn7), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vds, var_vds_dn0, var_vds_dn2, var_vds_dn6, var_vds_dn7, var_vds_dn10, var_vds_dn11, var_vds_dn12, var_vds_dn17,)
    }
};
        var_vds = assign4070_e2765;
        var_vds_dn0 = assign4070_e2765_d_n0;
        var_vds_dn2 = assign4070_e2765_d_n2;
        var_vds_dn6 = assign4070_e2765_d_n6;
        var_vds_dn7 = assign4070_e2765_d_n7;
        var_vds_dn10 = assign4070_e2765_d_n10;
        var_vds_dn11 = assign4070_e2765_d_n11;
        var_vds_dn12 = assign4070_e2765_d_n12;
        var_vds_dn17 = assign4070_e2765_d_n17;

        let (assign4080_e2772, assign4080_e2772_d_n6, assign4080_e2772_d_n7, assign4080_e2772_d_n11,) = {
    if (var_guard34 == 0.0) {
        let assign4080_e2770: f64 = (var_vgsi - var_vdsi);
        (assign4080_e2770, (-var_vdsi_dn6), (var_vgsi_dn7 - var_vdsi_dn7), var_vgsi_dn11,)
    } else {
        (var_vgs, var_vgs_dn6, var_vgs_dn7, var_vgs_dn11,)
    }
};
        var_vgs = assign4080_e2772;
        var_vgs_dn6 = assign4080_e2772_d_n6;
        var_vgs_dn7 = assign4080_e2772_d_n7;
        var_vgs_dn11 = assign4080_e2772_d_n11;

        let (assign4090_e2779, assign4090_e2779_d_n0, assign4090_e2779_d_n2, assign4090_e2779_d_n6, assign4090_e2779_d_n7, assign4090_e2779_d_n10, assign4090_e2779_d_n11, assign4090_e2779_d_n12, assign4090_e2779_d_n17,) = {
    if (var_guard34 == 0.0) {
        let assign4090_e2777: f64 = (var_vbsi - var_vdsi);
        (assign4090_e2777, 0.0, 0.0, (-var_vdsi_dn6), (var_vbsi_dn7 - var_vdsi_dn7), 0.0, 0.0, var_vbsi_dn12, 0.0,)
    } else {
        (var_vbs, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
    }
};
        var_vbs = assign4090_e2779;
        var_vbs_dn0 = assign4090_e2779_d_n0;
        var_vbs_dn2 = assign4090_e2779_d_n2;
        var_vbs_dn6 = assign4090_e2779_d_n6;
        var_vbs_dn7 = assign4090_e2779_d_n7;
        var_vbs_dn10 = assign4090_e2779_d_n10;
        var_vbs_dn11 = assign4090_e2779_d_n11;
        var_vbs_dn12 = assign4090_e2779_d_n12;
        var_vbs_dn17 = assign4090_e2779_d_n17;

        let assign4150_e2806: f64 = ctx_temp;
        var_ttemp = assign4150_e2806;
        var_ttemp_dn10 = 0.0;

        let (assign4160_e2810, assign4160_e2810_d_n10,) = {
    if (var_temp_given != 0.0) {
        (var_uc_temp, 0.0,)
    } else {
        (var_ttemp, var_ttemp_dn10,)
    }
};
        var_ttemp = assign4160_e2810;
        var_ttemp_dn10 = assign4160_e2810_d_n10;

        let (assign4170_e2816, assign4170_e2816_d_n10,) = {
    if (var_dtemp_given != 0.0) {
        let assign4170_e2814: f64 = (var_ttemp + p.p17);
        (assign4170_e2814, var_ttemp_dn10,)
    } else {
        (var_ttemp, var_ttemp_dn10,)
    }
};
        var_ttemp = assign4170_e2816;
        var_ttemp_dn10 = assign4170_e2816_d_n10;

        let assign4180_e2819: f64 = (var_ttemp + var_deltemp);
        var_ttemp = assign4180_e2819;
        var_ttemp_dn10 = (var_ttemp_dn10 + var_deltemp_dn10);

        let assign4190_e2822: f64 = (var_ttemp - var_uc_tnom);
        var_t1 = assign4190_e2822;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn10 = var_ttemp_dn10;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn17 = 0.0;

        let assign4200_e2826: f64 = (var_ttemp + var_uc_tnom);
        let assign4200_e2827: f64 = (var_t1 * assign4200_e2826);
        var_t2 = assign4200_e2827;
        var_t2_dn0 = (var_t1_dn0 * assign4200_e2826);
        var_t2_dn2 = (var_t1_dn2 * assign4200_e2826);
        var_t2_dn6 = (var_t1_dn6 * assign4200_e2826);
        var_t2_dn7 = (var_t1_dn7 * assign4200_e2826);
        var_t2_dn10 = ((var_t1_dn10 * assign4200_e2826) + (var_t1 * var_ttemp_dn10));
        var_t2_dn11 = (var_t1_dn11 * assign4200_e2826);
        var_t2_dn12 = (var_t1_dn12 * assign4200_e2826);
        var_t2_dn17 = (var_t1_dn17 * assign4200_e2826);

        let assign4210_e2831: f64 = (p.p53 * var_t1);
        let assign4210_e2832: f64 = (var_egtnom - assign4210_e2831);
        let assign4210_e2835: f64 = (p.p54 * var_t2);
        let assign4210_e2836: f64 = (assign4210_e2832 - assign4210_e2835);
        var_eg = assign4210_e2836;
        var_eg_dn0 = ((-(p.p53 * var_t1_dn0)) - (p.p54 * var_t2_dn0));
        var_eg_dn2 = ((-(p.p53 * var_t1_dn2)) - (p.p54 * var_t2_dn2));
        var_eg_dn6 = ((-(p.p53 * var_t1_dn6)) - (p.p54 * var_t2_dn6));
        var_eg_dn7 = ((-(p.p53 * var_t1_dn7)) - (p.p54 * var_t2_dn7));
        var_eg_dn10 = ((-(p.p53 * var_t1_dn10)) - (p.p54 * var_t2_dn10));
        var_eg_dn11 = ((-(p.p53 * var_t1_dn11)) - (p.p54 * var_t2_dn11));
        var_eg_dn12 = ((-(p.p53 * var_t1_dn12)) - (p.p54 * var_t2_dn12));
        var_eg_dn17 = ((-(p.p53 * var_t1_dn17)) - (p.p54 * var_t2_dn17));

        let assign4220_e2840: f64 = (1.3806226e-23 * var_ttemp);
        let assign4220_e2841: f64 = (1.6021918e-19 / assign4220_e2840);
        var_beta = assign4220_e2841;
        var_beta_dn10 = (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn10)) / (assign4220_e2840 * assign4220_e2840)));

        let assign4230_e2844: f64 = (var_beta * var_beta);
        var_beta2 = assign4230_e2844;
        var_beta2_dn10 = ((var_beta_dn10 * var_beta) + (var_beta * var_beta_dn10));

        let assign4240_e2847: f64 = (1.0 / var_beta);
        var_beta_inv = assign4240_e2847;
        var_beta_inv_dn10 = (-(var_beta_dn10 / (var_beta * var_beta)));

        let assign4250_e2853: f64 = (var_wg).powf(p.p99);
        let assign4250_e2854: f64 = (p.p98 / assign4250_e2853);
        let assign4250_e2855: f64 = (1.0 + assign4250_e2854);
        let assign4250_e2856: f64 = (p.p254 * assign4250_e2855);
        let assign4250_e2861: f64 = (var_lgle).powf(p.p101);
        let assign4250_e2862: f64 = (p.p100 / assign4250_e2861);
        let assign4250_e2863: f64 = (1.0 + assign4250_e2862);
        let assign4250_e2864: f64 = (assign4250_e2856 * assign4250_e2863);
        let assign4250_e2869: f64 = (var_wl).powf(p.p103);
        let assign4250_e2870: f64 = (p.p102 / assign4250_e2869);
        let assign4250_e2871: f64 = (1.0 + assign4250_e2870);
        let assign4250_e2872: f64 = (assign4250_e2864 * assign4250_e2871);
        var_cgs_mueph = assign4250_e2872;

        let assign4260_e2876: f64 = (1.0 + p.p159);
        let assign4260_e2877: f64 = (1.0 / assign4260_e2876);
        var_t2__blk40 = assign4260_e2877;

        var_t3__blk41 = 0.0;

        let assign4280_e2883: f64 = (var_t2__blk40 * var_t3__blk41);
        let assign4280_e2884: f64 = (1.0 + assign4280_e2883);
        let assign4280_e2885: f64 = (var_cgs_mueph * assign4280_e2884);
        var_cgs_wmueph = assign4280_e2885;

        let assign4290_e2888: f64 = (var_ttemp / var_uc_tnom);
        let assign4290_e2890: f64 = (assign4290_e2888).powf(p.p112);
        var_t1__blk39 = assign4290_e2890;
        var_t1__blk39_dn10 = if 0.0 == 0.0 && ((p.p112) as f64).is_finite() && ((p.p112) as f64).fract() == 0.0 { if p.p112 == 0.0 { 0.0 } else { (p.p112 * ((assign4290_e2888).powf(p.p112 - 1.0) * (var_ttemp_dn10 / var_uc_tnom))) } } else { (assign4290_e2890 * (p.p112 * ((var_ttemp_dn10 / var_uc_tnom) / assign4290_e2888))) };

        let assign4300_e2893: f64 = (var_t1__blk39 / var_cgs_wmueph);
        var_cgs_mphn0 = assign4300_e2893;
        var_cgs_mphn0_dn10 = (var_t1__blk39_dn10 / var_cgs_wmueph);

        let assign4310_e2896: f64 = (var_ptovr0 * var_beta_inv);
        var_ptovr = assign4310_e2896;
        var_ptovr_dn0 = (var_ptovr0_dn0 * var_beta_inv);
        var_ptovr_dn2 = (var_ptovr0_dn2 * var_beta_inv);
        var_ptovr_dn6 = (var_ptovr0_dn6 * var_beta_inv);
        var_ptovr_dn7 = (var_ptovr0_dn7 * var_beta_inv);
        var_ptovr_dn10 = ((var_ptovr0_dn10 * var_beta_inv) + (var_ptovr0 * var_beta_inv_dn10));
        var_ptovr_dn11 = (var_ptovr0_dn11 * var_beta_inv);
        var_ptovr_dn12 = (var_ptovr0_dn12 * var_beta_inv);
        var_ptovr_dn17 = (var_ptovr0_dn17 * var_beta_inv);

        let assign4320_e2899: f64 = (var_ttemp / var_uc_tnom);
        var_t1 = assign4320_e2899;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn10 = (var_ttemp_dn10 / var_uc_tnom);
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn17 = 0.0;

        let assign4330_e2902: f64 = (var_vmax0 * var_mks_vmax);
        let assign4330_e2906: f64 = (0.4 * var_t1);
        let assign4330_e2907: f64 = (1.8 + assign4330_e2906);
        let assign4330_e2910: f64 = (0.1 * var_t1);
        let assign4330_e2912: f64 = (assign4330_e2910 * var_t1);
        let assign4330_e2913: f64 = (assign4330_e2907 + assign4330_e2912);
        let assign4330_e2917: f64 = (1.0 - var_t1);
        let assign4330_e2918: f64 = (var_mks_vtmp * assign4330_e2917);
        let assign4330_e2919: f64 = (assign4330_e2913 - assign4330_e2918);
        let assign4330_e2920: f64 = (assign4330_e2902 / assign4330_e2919);
        var_vmaxe = assign4330_e2920;
        var_vmaxe_dn0 = ((((var_vmax0_dn0 * var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * var_t1_dn0) + (((0.1 * var_t1_dn0) * var_t1) + (assign4330_e2910 * var_t1_dn0))) - (var_mks_vtmp * (-var_t1_dn0))))) / (assign4330_e2919 * assign4330_e2919));
        var_vmaxe_dn2 = ((((var_vmax0_dn2 * var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * var_t1_dn2) + (((0.1 * var_t1_dn2) * var_t1) + (assign4330_e2910 * var_t1_dn2))) - (var_mks_vtmp * (-var_t1_dn2))))) / (assign4330_e2919 * assign4330_e2919));
        var_vmaxe_dn6 = ((((var_vmax0_dn6 * var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * var_t1_dn6) + (((0.1 * var_t1_dn6) * var_t1) + (assign4330_e2910 * var_t1_dn6))) - (var_mks_vtmp * (-var_t1_dn6))))) / (assign4330_e2919 * assign4330_e2919));
        var_vmaxe_dn7 = ((((var_vmax0_dn7 * var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * var_t1_dn7) + (((0.1 * var_t1_dn7) * var_t1) + (assign4330_e2910 * var_t1_dn7))) - (var_mks_vtmp * (-var_t1_dn7))))) / (assign4330_e2919 * assign4330_e2919));
        var_vmaxe_dn10 = ((((var_vmax0_dn10 * var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * var_t1_dn10) + (((0.1 * var_t1_dn10) * var_t1) + (assign4330_e2910 * var_t1_dn10))) - (var_mks_vtmp * (-var_t1_dn10))))) / (assign4330_e2919 * assign4330_e2919));
        var_vmaxe_dn11 = ((((var_vmax0_dn11 * var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * var_t1_dn11) + (((0.1 * var_t1_dn11) * var_t1) + (assign4330_e2910 * var_t1_dn11))) - (var_mks_vtmp * (-var_t1_dn11))))) / (assign4330_e2919 * assign4330_e2919));
        var_vmaxe_dn12 = ((((var_vmax0_dn12 * var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * var_t1_dn12) + (((0.1 * var_t1_dn12) * var_t1) + (assign4330_e2910 * var_t1_dn12))) - (var_mks_vtmp * (-var_t1_dn12))))) / (assign4330_e2919 * assign4330_e2919));
        var_vmaxe_dn17 = ((((var_vmax0_dn17 * var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * var_t1_dn17) + (((0.1 * var_t1_dn17) * var_t1) + (assign4330_e2910 * var_t1_dn17))) - (var_mks_vtmp * (-var_t1_dn17))))) / (assign4330_e2919 * assign4330_e2919));

        let assign4340_e2922: f64 = (var_eg).sqrt();
        var_egp12 = assign4340_e2922;
        var_egp12_dn0 = (var_eg_dn0 / (2.0 * assign4340_e2922));
        var_egp12_dn2 = (var_eg_dn2 / (2.0 * assign4340_e2922));
        var_egp12_dn6 = (var_eg_dn6 / (2.0 * assign4340_e2922));
        var_egp12_dn7 = (var_eg_dn7 / (2.0 * assign4340_e2922));
        var_egp12_dn10 = (var_eg_dn10 / (2.0 * assign4340_e2922));
        var_egp12_dn11 = (var_eg_dn11 / (2.0 * assign4340_e2922));
        var_egp12_dn12 = (var_eg_dn12 / (2.0 * assign4340_e2922));
        var_egp12_dn17 = (var_eg_dn17 / (2.0 * assign4340_e2922));

        let assign4350_e2925: f64 = (var_eg * var_egp12);
        var_egp32 = assign4350_e2925;
        var_egp32_dn0 = ((var_eg_dn0 * var_egp12) + (var_eg * var_egp12_dn0));
        var_egp32_dn2 = ((var_eg_dn2 * var_egp12) + (var_eg * var_egp12_dn2));
        var_egp32_dn6 = ((var_eg_dn6 * var_egp12) + (var_eg * var_egp12_dn6));
        var_egp32_dn7 = ((var_eg_dn7 * var_egp12) + (var_eg * var_egp12_dn7));
        var_egp32_dn10 = ((var_eg_dn10 * var_egp12) + (var_eg * var_egp12_dn10));
        var_egp32_dn11 = ((var_eg_dn11 * var_egp12) + (var_eg * var_egp12_dn11));
        var_egp32_dn12 = ((var_eg_dn12 * var_egp12) + (var_eg * var_egp12_dn12));
        var_egp32_dn17 = ((var_eg_dn17 * var_egp12) + (var_eg * var_egp12_dn17));

        let assign4360_e2928: f64 = (10400000000.0 / 1e-6);
        let assign4360_e2931: f64 = (var_ttemp / var_uc_tnom);
        let assign4360_e2933: f64 = (assign4360_e2931).powf(1.5);
        let assign4360_e2934: f64 = (assign4360_e2928 * assign4360_e2933);
        let assign4360_e2936: f64 = (-var_eg);
        let assign4360_e2938: f64 = (assign4360_e2936 / 2.0);
        let assign4360_e2940: f64 = (assign4360_e2938 * var_beta);
        let assign4360_e2943: f64 = (var_egtnom / 2.0);
        let assign4360_e2945: f64 = (assign4360_e2943 * var_betatnom);
        let assign4360_e2946: f64 = (assign4360_e2940 + assign4360_e2945);
        let assign4360_e2947: f64 = (assign4360_e2946).exp();
        let assign4360_e2948: f64 = (assign4360_e2934 * assign4360_e2947);
        var_nin = assign4360_e2948;
        var_nin_dn0 = (assign4360_e2934 * (assign4360_e2947 * (((-var_eg_dn0) / 2.0) * var_beta)));
        var_nin_dn2 = (assign4360_e2934 * (assign4360_e2947 * (((-var_eg_dn2) / 2.0) * var_beta)));
        var_nin_dn6 = (assign4360_e2934 * (assign4360_e2947 * (((-var_eg_dn6) / 2.0) * var_beta)));
        var_nin_dn7 = (assign4360_e2934 * (assign4360_e2947 * (((-var_eg_dn7) / 2.0) * var_beta)));
        var_nin_dn10 = (((assign4360_e2928 * if 0.0 == 0.0 && ((1.5) as f64).is_finite() && ((1.5) as f64).fract() == 0.0 { if 1.5 == 0.0 { 0.0 } else { (1.5 * ((assign4360_e2931).powf(1.5 - 1.0) * (var_ttemp_dn10 / var_uc_tnom))) } } else { (assign4360_e2933 * (1.5 * ((var_ttemp_dn10 / var_uc_tnom) / assign4360_e2931))) }) * assign4360_e2947) + (assign4360_e2934 * (assign4360_e2947 * ((((-var_eg_dn10) / 2.0) * var_beta) + (assign4360_e2938 * var_beta_dn10)))));
        var_nin_dn11 = (assign4360_e2934 * (assign4360_e2947 * (((-var_eg_dn11) / 2.0) * var_beta)));
        var_nin_dn12 = (assign4360_e2934 * (assign4360_e2947 * (((-var_eg_dn12) / 2.0) * var_beta)));
        var_nin_dn17 = (assign4360_e2934 * (assign4360_e2947 * (((-var_eg_dn17) / 2.0) * var_beta)));

        let assign4370_e2951: f64 = (var_beta_inv).sqrt();
        let assign4370_e2952: f64 = (var_costi00 * assign4370_e2951);
        var_costi0 = assign4370_e2952;
        var_costi0_dn0 = 0.0;
        var_costi0_dn2 = 0.0;
        var_costi0_dn6 = 0.0;
        var_costi0_dn7 = 0.0;
        var_costi0_dn10 = (var_costi00 * (var_beta_inv_dn10 / (2.0 * assign4370_e2951)));
        var_costi0_dn11 = 0.0;
        var_costi0_dn12 = 0.0;
        var_costi0_dn17 = 0.0;

        let assign4380_e2955: f64 = (var_costi0 * var_costi0);
        var_costi0_p2 = assign4380_e2955;
        var_costi0_p2_dn0 = ((var_costi0_dn0 * var_costi0) + (var_costi0 * var_costi0_dn0));
        var_costi0_p2_dn2 = ((var_costi0_dn2 * var_costi0) + (var_costi0 * var_costi0_dn2));
        var_costi0_p2_dn6 = ((var_costi0_dn6 * var_costi0) + (var_costi0 * var_costi0_dn6));
        var_costi0_p2_dn7 = ((var_costi0_dn7 * var_costi0) + (var_costi0 * var_costi0_dn7));
        var_costi0_p2_dn10 = ((var_costi0_dn10 * var_costi0) + (var_costi0 * var_costi0_dn10));
        var_costi0_p2_dn11 = ((var_costi0_dn11 * var_costi0) + (var_costi0 * var_costi0_dn11));
        var_costi0_p2_dn12 = ((var_costi0_dn12 * var_costi0) + (var_costi0 * var_costi0_dn12));
        var_costi0_p2_dn17 = ((var_costi0_dn17 * var_costi0) + (var_costi0 * var_costi0_dn17));

        let assign4390_e2958: f64 = (var_nin * var_nin);
        let assign4390_e2960: f64 = (assign4390_e2958 * var_nsti_p2);
        var_costi1 = assign4390_e2960;
        var_costi1_dn0 = (((var_nin_dn0 * var_nin) + (var_nin * var_nin_dn0)) * var_nsti_p2);
        var_costi1_dn2 = (((var_nin_dn2 * var_nin) + (var_nin * var_nin_dn2)) * var_nsti_p2);
        var_costi1_dn6 = (((var_nin_dn6 * var_nin) + (var_nin * var_nin_dn6)) * var_nsti_p2);
        var_costi1_dn7 = (((var_nin_dn7 * var_nin) + (var_nin * var_nin_dn7)) * var_nsti_p2);
        var_costi1_dn10 = (((var_nin_dn10 * var_nin) + (var_nin * var_nin_dn10)) * var_nsti_p2);
        var_costi1_dn11 = (((var_nin_dn11 * var_nin) + (var_nin * var_nin_dn11)) * var_nsti_p2);
        var_costi1_dn12 = (((var_nin_dn12 * var_nin) + (var_nin * var_nin_dn12)) * var_nsti_p2);
        var_costi1_dn17 = (((var_nin_dn17 * var_nin) + (var_nin * var_nin_dn17)) * var_nsti_p2);

        let assign4400_e2964: f64 = (2.0 * p.p56);
        let assign4400_e2965: f64 = (var_lgate - assign4400_e2964);
        var_lch = assign4400_e2965;
        var_lch_dn0 = 0.0;
        var_lch_dn2 = 0.0;
        var_lch_dn6 = 0.0;
        var_lch_dn7 = 0.0;
        var_lch_dn10 = 0.0;
        var_lch_dn11 = 0.0;
        var_lch_dn12 = 0.0;
        var_lch_dn17 = 0.0;

        let assign4410_e2968: f64 = if var_subversion > 3.0 { 1.0 } else { 0.0 };
        var_guard42 = assign4410_e2968;

        let (assign4420_e2979, assign4420_e2979_d_n0, assign4420_e2979_d_n2, assign4420_e2979_d_n6, assign4420_e2979_d_n7, assign4420_e2979_d_n10, assign4420_e2979_d_n11, assign4420_e2979_d_n12, assign4420_e2979_d_n17,) = {
    if (var_guard42 != 0.0) {
        let assign4420_e2972: f64 = (2.0 * var_beta_inv);
        let assign4420_e2975: f64 = (var_nsub / var_nin);
        let assign4420_e2976: f64 = (assign4420_e2975).ln();
        let assign4420_e2977: f64 = (assign4420_e2972 * assign4420_e2976);
        (assign4420_e2977, (assign4420_e2972 * ((((var_nsub_dn0 * var_nin) - (var_nsub * var_nin_dn0)) / (var_nin * var_nin)) / assign4420_e2975)), (assign4420_e2972 * ((((var_nsub_dn2 * var_nin) - (var_nsub * var_nin_dn2)) / (var_nin * var_nin)) / assign4420_e2975)), (assign4420_e2972 * ((((var_nsub_dn6 * var_nin) - (var_nsub * var_nin_dn6)) / (var_nin * var_nin)) / assign4420_e2975)), (assign4420_e2972 * ((((var_nsub_dn7 * var_nin) - (var_nsub * var_nin_dn7)) / (var_nin * var_nin)) / assign4420_e2975)), (((2.0 * var_beta_inv_dn10) * assign4420_e2976) + (assign4420_e2972 * ((((var_nsub_dn10 * var_nin) - (var_nsub * var_nin_dn10)) / (var_nin * var_nin)) / assign4420_e2975))), (assign4420_e2972 * ((((var_nsub_dn11 * var_nin) - (var_nsub * var_nin_dn11)) / (var_nin * var_nin)) / assign4420_e2975)), (assign4420_e2972 * ((((var_nsub_dn12 * var_nin) - (var_nsub * var_nin_dn12)) / (var_nin * var_nin)) / assign4420_e2975)), (assign4420_e2972 * ((((var_nsub_dn17 * var_nin) - (var_nsub * var_nin_dn17)) / (var_nin * var_nin)) / assign4420_e2975)),)
    } else {
        (var_pb2, var_pb2_dn0, var_pb2_dn2, var_pb2_dn6, var_pb2_dn7, var_pb2_dn10, var_pb2_dn11, var_pb2_dn12, var_pb2_dn17,)
    }
};
        var_pb2 = assign4420_e2979;
        var_pb2_dn0 = assign4420_e2979_d_n0;
        var_pb2_dn2 = assign4420_e2979_d_n2;
        var_pb2_dn6 = assign4420_e2979_d_n6;
        var_pb2_dn7 = assign4420_e2979_d_n7;
        var_pb2_dn10 = assign4420_e2979_d_n10;
        var_pb2_dn11 = assign4420_e2979_d_n11;
        var_pb2_dn12 = assign4420_e2979_d_n12;
        var_pb2_dn17 = assign4420_e2979_d_n17;

        let (assign4430_e2991, assign4430_e2991_d_n0, assign4430_e2991_d_n2, assign4430_e2991_d_n6, assign4430_e2991_d_n7, assign4430_e2991_d_n10, assign4430_e2991_d_n11, assign4430_e2991_d_n12, assign4430_e2991_d_n17,) = {
    if (var_guard42 == 0.0) {
        let assign4430_e2984: f64 = (2.0 * var_beta_inv);
        let assign4430_e2987: f64 = (var_uc_nsubs / var_nin);
        let assign4430_e2988: f64 = (assign4430_e2987).ln();
        let assign4430_e2989: f64 = (assign4430_e2984 * assign4430_e2988);
        (assign4430_e2989, (assign4430_e2984 * ((((var_uc_nsubs_dn0 * var_nin) - (var_uc_nsubs * var_nin_dn0)) / (var_nin * var_nin)) / assign4430_e2987)), (assign4430_e2984 * ((((var_uc_nsubs_dn2 * var_nin) - (var_uc_nsubs * var_nin_dn2)) / (var_nin * var_nin)) / assign4430_e2987)), (assign4430_e2984 * ((((var_uc_nsubs_dn6 * var_nin) - (var_uc_nsubs * var_nin_dn6)) / (var_nin * var_nin)) / assign4430_e2987)), (assign4430_e2984 * ((((var_uc_nsubs_dn7 * var_nin) - (var_uc_nsubs * var_nin_dn7)) / (var_nin * var_nin)) / assign4430_e2987)), (((2.0 * var_beta_inv_dn10) * assign4430_e2988) + (assign4430_e2984 * ((((var_uc_nsubs_dn10 * var_nin) - (var_uc_nsubs * var_nin_dn10)) / (var_nin * var_nin)) / assign4430_e2987))), (assign4430_e2984 * ((((var_uc_nsubs_dn11 * var_nin) - (var_uc_nsubs * var_nin_dn11)) / (var_nin * var_nin)) / assign4430_e2987)), (assign4430_e2984 * ((((var_uc_nsubs_dn12 * var_nin) - (var_uc_nsubs * var_nin_dn12)) / (var_nin * var_nin)) / assign4430_e2987)), (assign4430_e2984 * ((((var_uc_nsubs_dn17 * var_nin) - (var_uc_nsubs * var_nin_dn17)) / (var_nin * var_nin)) / assign4430_e2987)),)
    } else {
        (var_pb2, var_pb2_dn0, var_pb2_dn2, var_pb2_dn6, var_pb2_dn7, var_pb2_dn10, var_pb2_dn11, var_pb2_dn12, var_pb2_dn17,)
    }
};
        var_pb2 = assign4430_e2991;
        var_pb2_dn0 = assign4430_e2991_d_n0;
        var_pb2_dn2 = assign4430_e2991_d_n2;
        var_pb2_dn6 = assign4430_e2991_d_n6;
        var_pb2_dn7 = assign4430_e2991_d_n7;
        var_pb2_dn10 = assign4430_e2991_d_n10;
        var_pb2_dn11 = assign4430_e2991_d_n11;
        var_pb2_dn12 = assign4430_e2991_d_n12;
        var_pb2_dn17 = assign4430_e2991_d_n17;

        let assign4440_e2994: f64 = (1.034943e-10 / var_q_nsub);
        let assign4440_e2996: f64 = (assign4440_e2994 * var_beta_inv);
        let assign4440_e2997: f64 = (assign4440_e2996).sqrt();
        var_ldby = assign4440_e2997;
        var_ldby_dn0 = (((-((1.034943e-10 * var_q_nsub_dn0) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4440_e2997));
        var_ldby_dn2 = (((-((1.034943e-10 * var_q_nsub_dn2) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4440_e2997));
        var_ldby_dn6 = (((-((1.034943e-10 * var_q_nsub_dn6) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4440_e2997));
        var_ldby_dn7 = (((-((1.034943e-10 * var_q_nsub_dn7) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4440_e2997));
        var_ldby_dn10 = ((((-((1.034943e-10 * var_q_nsub_dn10) / (var_q_nsub * var_q_nsub))) * var_beta_inv) + (assign4440_e2994 * var_beta_inv_dn10)) / (2.0 * assign4440_e2997));
        var_ldby_dn11 = (((-((1.034943e-10 * var_q_nsub_dn11) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4440_e2997));
        var_ldby_dn12 = (((-((1.034943e-10 * var_q_nsub_dn12) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4440_e2997));
        var_ldby_dn17 = (((-((1.034943e-10 * var_q_nsub_dn17) / (var_q_nsub * var_q_nsub))) * var_beta_inv) / (2.0 * assign4440_e2997));

        let assign4450_e3000: f64 = (var_q_nsub * 1.414213562373095);
        let assign4450_e3002: f64 = (assign4450_e3000 * var_ldby);
        var_cnst0soi = assign4450_e3002;
        var_cnst0soi_dn0 = (((var_q_nsub_dn0 * 1.414213562373095) * var_ldby) + (assign4450_e3000 * var_ldby_dn0));
        var_cnst0soi_dn2 = (((var_q_nsub_dn2 * 1.414213562373095) * var_ldby) + (assign4450_e3000 * var_ldby_dn2));
        var_cnst0soi_dn6 = (((var_q_nsub_dn6 * 1.414213562373095) * var_ldby) + (assign4450_e3000 * var_ldby_dn6));
        var_cnst0soi_dn7 = (((var_q_nsub_dn7 * 1.414213562373095) * var_ldby) + (assign4450_e3000 * var_ldby_dn7));
        var_cnst0soi_dn10 = (((var_q_nsub_dn10 * 1.414213562373095) * var_ldby) + (assign4450_e3000 * var_ldby_dn10));
        var_cnst0soi_dn11 = (((var_q_nsub_dn11 * 1.414213562373095) * var_ldby) + (assign4450_e3000 * var_ldby_dn11));
        var_cnst0soi_dn12 = (((var_q_nsub_dn12 * 1.414213562373095) * var_ldby) + (assign4450_e3000 * var_ldby_dn12));
        var_cnst0soi_dn17 = (((var_q_nsub_dn17 * 1.414213562373095) * var_ldby) + (assign4450_e3000 * var_ldby_dn17));

        let assign4460_e3005: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard43 = assign4460_e3005;

        let (assign4470_e3009, assign4470_e3009_d_n10,) = {
    if (var_guard43 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_cnst0bulk, var_cnst0bulk_dn10,)
    }
};
        var_cnst0bulk = assign4470_e3009;
        var_cnst0bulk_dn10 = assign4470_e3009_d_n10;

        let (assign4480_e3013, assign4480_e3013_d_n0, assign4480_e3013_d_n2, assign4480_e3013_d_n6, assign4480_e3013_d_n7, assign4480_e3013_d_n10, assign4480_e3013_d_n11, assign4480_e3013_d_n12, assign4480_e3013_d_n17,) = {
    if (var_guard43 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cnst1bulk, var_cnst1bulk_dn0, var_cnst1bulk_dn2, var_cnst1bulk_dn6, var_cnst1bulk_dn7, var_cnst1bulk_dn10, var_cnst1bulk_dn11, var_cnst1bulk_dn12, var_cnst1bulk_dn17,)
    }
};
        var_cnst1bulk = assign4480_e3013;
        var_cnst1bulk_dn0 = assign4480_e3013_d_n0;
        var_cnst1bulk_dn2 = assign4480_e3013_d_n2;
        var_cnst1bulk_dn6 = assign4480_e3013_d_n6;
        var_cnst1bulk_dn7 = assign4480_e3013_d_n7;
        var_cnst1bulk_dn10 = assign4480_e3013_d_n10;
        var_cnst1bulk_dn11 = assign4480_e3013_d_n11;
        var_cnst1bulk_dn12 = assign4480_e3013_d_n12;
        var_cnst1bulk_dn17 = assign4480_e3013_d_n17;

        let (assign4490_e3019, assign4490_e3019_d_n0, assign4490_e3019_d_n2, assign4490_e3019_d_n6, assign4490_e3019_d_n7, assign4490_e3019_d_n10, assign4490_e3019_d_n11, assign4490_e3019_d_n12, assign4490_e3019_d_n17,) = {
    if (var_guard43 != 0.0) {
        let assign4490_e3017: f64 = (var_nin / var_nsub);
        (assign4490_e3017, (((var_nin_dn0 * var_nsub) - (var_nin * var_nsub_dn0)) / (var_nsub * var_nsub)), (((var_nin_dn2 * var_nsub) - (var_nin * var_nsub_dn2)) / (var_nsub * var_nsub)), (((var_nin_dn6 * var_nsub) - (var_nin * var_nsub_dn6)) / (var_nsub * var_nsub)), (((var_nin_dn7 * var_nsub) - (var_nin * var_nsub_dn7)) / (var_nsub * var_nsub)), (((var_nin_dn10 * var_nsub) - (var_nin * var_nsub_dn10)) / (var_nsub * var_nsub)), (((var_nin_dn11 * var_nsub) - (var_nin * var_nsub_dn11)) / (var_nsub * var_nsub)), (((var_nin_dn12 * var_nsub) - (var_nin * var_nsub_dn12)) / (var_nsub * var_nsub)), (((var_nin_dn17 * var_nsub) - (var_nin * var_nsub_dn17)) / (var_nsub * var_nsub)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign4490_e3019;
        var_t1_dn0 = assign4490_e3019_d_n0;
        var_t1_dn2 = assign4490_e3019_d_n2;
        var_t1_dn6 = assign4490_e3019_d_n6;
        var_t1_dn7 = assign4490_e3019_d_n7;
        var_t1_dn10 = assign4490_e3019_d_n10;
        var_t1_dn11 = assign4490_e3019_d_n11;
        var_t1_dn12 = assign4490_e3019_d_n12;
        var_t1_dn17 = assign4490_e3019_d_n17;

        let (assign4500_e3029, assign4500_e3029_d_n10,) = {
    if (var_guard43 == 0.0) {
        let assign4500_e3024: f64 = (2.0 * var_c0bulk);
        let assign4500_e3026: f64 = (assign4500_e3024 * var_beta_inv);
        let assign4500_e3027: f64 = (assign4500_e3026).sqrt();
        (assign4500_e3027, ((assign4500_e3024 * var_beta_inv_dn10) / (2.0 * assign4500_e3027)),)
    } else {
        (var_cnst0bulk, var_cnst0bulk_dn10,)
    }
};
        var_cnst0bulk = assign4500_e3029;
        var_cnst0bulk_dn10 = assign4500_e3029_d_n10;

        *var_beta_slot = var_beta;
        *var_beta2_slot = var_beta2;
        *var_beta2_dn10_slot = var_beta2_dn10;
        *var_beta_dn10_slot = var_beta_dn10;
        *var_beta_inv_slot = var_beta_inv;
        *var_beta_inv_dn10_slot = var_beta_inv_dn10;
        *var_cgs_mphn0_slot = var_cgs_mphn0;
        *var_cgs_mphn0_dn10_slot = var_cgs_mphn0_dn10;
        *var_cgs_mueph_slot = var_cgs_mueph;
        *var_cgs_wmueph_slot = var_cgs_wmueph;
        *var_cnst0bulk_slot = var_cnst0bulk;
        *var_cnst0bulk_dn10_slot = var_cnst0bulk_dn10;
        *var_cnst0soi_slot = var_cnst0soi;
        *var_cnst0soi_dn0_slot = var_cnst0soi_dn0;
        *var_cnst0soi_dn10_slot = var_cnst0soi_dn10;
        *var_cnst0soi_dn11_slot = var_cnst0soi_dn11;
        *var_cnst0soi_dn12_slot = var_cnst0soi_dn12;
        *var_cnst0soi_dn17_slot = var_cnst0soi_dn17;
        *var_cnst0soi_dn2_slot = var_cnst0soi_dn2;
        *var_cnst0soi_dn6_slot = var_cnst0soi_dn6;
        *var_cnst0soi_dn7_slot = var_cnst0soi_dn7;
        *var_cnst1bulk_slot = var_cnst1bulk;
        *var_cnst1bulk_dn0_slot = var_cnst1bulk_dn0;
        *var_cnst1bulk_dn10_slot = var_cnst1bulk_dn10;
        *var_cnst1bulk_dn11_slot = var_cnst1bulk_dn11;
        *var_cnst1bulk_dn12_slot = var_cnst1bulk_dn12;
        *var_cnst1bulk_dn17_slot = var_cnst1bulk_dn17;
        *var_cnst1bulk_dn2_slot = var_cnst1bulk_dn2;
        *var_cnst1bulk_dn6_slot = var_cnst1bulk_dn6;
        *var_cnst1bulk_dn7_slot = var_cnst1bulk_dn7;
        *var_costi0_slot = var_costi0;
        *var_costi0_dn0_slot = var_costi0_dn0;
        *var_costi0_dn10_slot = var_costi0_dn10;
        *var_costi0_dn11_slot = var_costi0_dn11;
        *var_costi0_dn12_slot = var_costi0_dn12;
        *var_costi0_dn17_slot = var_costi0_dn17;
        *var_costi0_dn2_slot = var_costi0_dn2;
        *var_costi0_dn6_slot = var_costi0_dn6;
        *var_costi0_dn7_slot = var_costi0_dn7;
        *var_costi0_p2_slot = var_costi0_p2;
        *var_costi0_p2_dn0_slot = var_costi0_p2_dn0;
        *var_costi0_p2_dn10_slot = var_costi0_p2_dn10;
        *var_costi0_p2_dn11_slot = var_costi0_p2_dn11;
        *var_costi0_p2_dn12_slot = var_costi0_p2_dn12;
        *var_costi0_p2_dn17_slot = var_costi0_p2_dn17;
        *var_costi0_p2_dn2_slot = var_costi0_p2_dn2;
        *var_costi0_p2_dn6_slot = var_costi0_p2_dn6;
        *var_costi0_p2_dn7_slot = var_costi0_p2_dn7;
        *var_costi1_slot = var_costi1;
        *var_costi1_dn0_slot = var_costi1_dn0;
        *var_costi1_dn10_slot = var_costi1_dn10;
        *var_costi1_dn11_slot = var_costi1_dn11;
        *var_costi1_dn12_slot = var_costi1_dn12;
        *var_costi1_dn17_slot = var_costi1_dn17;
        *var_costi1_dn2_slot = var_costi1_dn2;
        *var_costi1_dn6_slot = var_costi1_dn6;
        *var_costi1_dn7_slot = var_costi1_dn7;
        *var_eg_slot = var_eg;
        *var_eg_dn0_slot = var_eg_dn0;
        *var_eg_dn10_slot = var_eg_dn10;
        *var_eg_dn11_slot = var_eg_dn11;
        *var_eg_dn12_slot = var_eg_dn12;
        *var_eg_dn17_slot = var_eg_dn17;
        *var_eg_dn2_slot = var_eg_dn2;
        *var_eg_dn6_slot = var_eg_dn6;
        *var_eg_dn7_slot = var_eg_dn7;
        *var_egp12_slot = var_egp12;
        *var_egp12_dn0_slot = var_egp12_dn0;
        *var_egp12_dn10_slot = var_egp12_dn10;
        *var_egp12_dn11_slot = var_egp12_dn11;
        *var_egp12_dn12_slot = var_egp12_dn12;
        *var_egp12_dn17_slot = var_egp12_dn17;
        *var_egp12_dn2_slot = var_egp12_dn2;
        *var_egp12_dn6_slot = var_egp12_dn6;
        *var_egp12_dn7_slot = var_egp12_dn7;
        *var_egp32_slot = var_egp32;
        *var_egp32_dn0_slot = var_egp32_dn0;
        *var_egp32_dn10_slot = var_egp32_dn10;
        *var_egp32_dn11_slot = var_egp32_dn11;
        *var_egp32_dn12_slot = var_egp32_dn12;
        *var_egp32_dn17_slot = var_egp32_dn17;
        *var_egp32_dn2_slot = var_egp32_dn2;
        *var_egp32_dn6_slot = var_egp32_dn6;
        *var_egp32_dn7_slot = var_egp32_dn7;
        *var_guard42_slot = var_guard42;
        *var_guard43_slot = var_guard43;
        *var_lch_slot = var_lch;
        *var_lch_dn0_slot = var_lch_dn0;
        *var_lch_dn10_slot = var_lch_dn10;
        *var_lch_dn11_slot = var_lch_dn11;
        *var_lch_dn12_slot = var_lch_dn12;
        *var_lch_dn17_slot = var_lch_dn17;
        *var_lch_dn2_slot = var_lch_dn2;
        *var_lch_dn6_slot = var_lch_dn6;
        *var_lch_dn7_slot = var_lch_dn7;
        *var_ldby_slot = var_ldby;
        *var_ldby_dn0_slot = var_ldby_dn0;
        *var_ldby_dn10_slot = var_ldby_dn10;
        *var_ldby_dn11_slot = var_ldby_dn11;
        *var_ldby_dn12_slot = var_ldby_dn12;
        *var_ldby_dn17_slot = var_ldby_dn17;
        *var_ldby_dn2_slot = var_ldby_dn2;
        *var_ldby_dn6_slot = var_ldby_dn6;
        *var_ldby_dn7_slot = var_ldby_dn7;
        *var_mode_slot = var_mode;
        *var_modenml_slot = var_modenml;
        *var_modervs_slot = var_modervs;
        *var_nin_slot = var_nin;
        *var_nin_dn0_slot = var_nin_dn0;
        *var_nin_dn10_slot = var_nin_dn10;
        *var_nin_dn11_slot = var_nin_dn11;
        *var_nin_dn12_slot = var_nin_dn12;
        *var_nin_dn17_slot = var_nin_dn17;
        *var_nin_dn2_slot = var_nin_dn2;
        *var_nin_dn6_slot = var_nin_dn6;
        *var_nin_dn7_slot = var_nin_dn7;
        *var_pb2_slot = var_pb2;
        *var_pb2_dn0_slot = var_pb2_dn0;
        *var_pb2_dn10_slot = var_pb2_dn10;
        *var_pb2_dn11_slot = var_pb2_dn11;
        *var_pb2_dn12_slot = var_pb2_dn12;
        *var_pb2_dn17_slot = var_pb2_dn17;
        *var_pb2_dn2_slot = var_pb2_dn2;
        *var_pb2_dn6_slot = var_pb2_dn6;
        *var_pb2_dn7_slot = var_pb2_dn7;
        *var_ptovr_slot = var_ptovr;
        *var_ptovr_dn0_slot = var_ptovr_dn0;
        *var_ptovr_dn10_slot = var_ptovr_dn10;
        *var_ptovr_dn11_slot = var_ptovr_dn11;
        *var_ptovr_dn12_slot = var_ptovr_dn12;
        *var_ptovr_dn17_slot = var_ptovr_dn17;
        *var_ptovr_dn2_slot = var_ptovr_dn2;
        *var_ptovr_dn6_slot = var_ptovr_dn6;
        *var_ptovr_dn7_slot = var_ptovr_dn7;
        *var_t1_slot = var_t1;
        *var_t1__blk39_slot = var_t1__blk39;
        *var_t1__blk39_dn10_slot = var_t1__blk39_dn10;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t2_slot = var_t2;
        *var_t2__blk40_slot = var_t2__blk40;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t3__blk41_slot = var_t3__blk41;
        *var_ttemp_slot = var_ttemp;
        *var_ttemp_dn10_slot = var_ttemp_dn10;
        *var_vbs_slot = var_vbs;
        *var_vbs_dn0_slot = var_vbs_dn0;
        *var_vbs_dn10_slot = var_vbs_dn10;
        *var_vbs_dn11_slot = var_vbs_dn11;
        *var_vbs_dn12_slot = var_vbs_dn12;
        *var_vbs_dn17_slot = var_vbs_dn17;
        *var_vbs_dn2_slot = var_vbs_dn2;
        *var_vbs_dn6_slot = var_vbs_dn6;
        *var_vbs_dn7_slot = var_vbs_dn7;
        *var_vds_slot = var_vds;
        *var_vds_dn0_slot = var_vds_dn0;
        *var_vds_dn10_slot = var_vds_dn10;
        *var_vds_dn11_slot = var_vds_dn11;
        *var_vds_dn12_slot = var_vds_dn12;
        *var_vds_dn17_slot = var_vds_dn17;
        *var_vds_dn2_slot = var_vds_dn2;
        *var_vds_dn6_slot = var_vds_dn6;
        *var_vds_dn7_slot = var_vds_dn7;
        *var_vgs_slot = var_vgs;
        *var_vgs_dn11_slot = var_vgs_dn11;
        *var_vgs_dn6_slot = var_vgs_dn6;
        *var_vgs_dn7_slot = var_vgs_dn7;
        *var_vmaxe_slot = var_vmaxe;
        *var_vmaxe_dn0_slot = var_vmaxe_dn0;
        *var_vmaxe_dn10_slot = var_vmaxe_dn10;
        *var_vmaxe_dn11_slot = var_vmaxe_dn11;
        *var_vmaxe_dn12_slot = var_vmaxe_dn12;
        *var_vmaxe_dn17_slot = var_vmaxe_dn17;
        *var_vmaxe_dn2_slot = var_vmaxe_dn2;
        *var_vmaxe_dn6_slot = var_vmaxe_dn6;
        *var_vmaxe_dn7_slot = var_vmaxe_dn7;
    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn10: f64,
        var_guard43: f64,
        var_mks_nsubb: f64,
        var_nin: f64,
        var_nin_dn0: f64,
        var_nin_dn10: f64,
        var_nin_dn11: f64,
        var_nin_dn12: f64,
        var_nin_dn17: f64,
        var_nin_dn2: f64,
        var_nin_dn6: f64,
        var_nin_dn7: f64,
        var_pb2: f64,
        var_q_nsub: f64,
        var_q_nsub_dn0: f64,
        var_q_nsub_dn10: f64,
        var_q_nsub_dn11: f64,
        var_q_nsub_dn12: f64,
        var_q_nsub_dn17: f64,
        var_q_nsub_dn2: f64,
        var_q_nsub_dn6: f64,
        var_q_nsub_dn7: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn17: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn7: f64,
        var_vbs: f64,
        var_vbs_dn0: f64,
        var_vbs_dn10: f64,
        var_vbs_dn11: f64,
        var_vbs_dn12: f64,
        var_vbs_dn17: f64,
        var_vbs_dn2: f64,
        var_vbs_dn6: f64,
        var_vbs_dn7: f64,
        var_arg_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn10_slot: &mut f64,
        var_arg_dn11_slot: &mut f64,
        var_arg_dn12_slot: &mut f64,
        var_arg_dn17_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_c_w_soi_slot: &mut f64,
        var_c_w_soi_dn0_slot: &mut f64,
        var_c_w_soi_dn10_slot: &mut f64,
        var_c_w_soi_dn11_slot: &mut f64,
        var_c_w_soi_dn12_slot: &mut f64,
        var_c_w_soi_dn17_slot: &mut f64,
        var_c_w_soi_dn2_slot: &mut f64,
        var_c_w_soi_dn6_slot: &mut f64,
        var_c_w_soi_dn7_slot: &mut f64,
        var_cnst1bulk_slot: &mut f64,
        var_cnst1bulk_dn0_slot: &mut f64,
        var_cnst1bulk_dn10_slot: &mut f64,
        var_cnst1bulk_dn11_slot: &mut f64,
        var_cnst1bulk_dn12_slot: &mut f64,
        var_cnst1bulk_dn17_slot: &mut f64,
        var_cnst1bulk_dn2_slot: &mut f64,
        var_cnst1bulk_dn6_slot: &mut f64,
        var_cnst1bulk_dn7_slot: &mut f64,
        var_cnst1soi_slot: &mut f64,
        var_cnst1soi_dn0_slot: &mut f64,
        var_cnst1soi_dn10_slot: &mut f64,
        var_cnst1soi_dn11_slot: &mut f64,
        var_cnst1soi_dn12_slot: &mut f64,
        var_cnst1soi_dn17_slot: &mut f64,
        var_cnst1soi_dn2_slot: &mut f64,
        var_cnst1soi_dn6_slot: &mut f64,
        var_cnst1soi_dn7_slot: &mut f64,
        var_cnst_2esi_q_nsubs_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn0_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn10_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn11_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn12_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn17_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn2_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn6_slot: &mut f64,
        var_cnst_2esi_q_nsubs_dn7_slot: &mut f64,
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn11_slot: &mut f64,
        var_dnm_dn12_slot: &mut f64,
        var_dnm_dn17_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_guard48_slot: &mut f64,
        var_guard49_slot: &mut f64,
        var_guard50_slot: &mut f64,
        var_guard51_slot: &mut f64,
        var_guard52_slot: &mut f64,
        var_guard53_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn17_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_vbs_bnd_slot: &mut f64,
        var_vbs_max_slot: &mut f64,
        var_wdsoi_ini_slot: &mut f64,
        var_x2_slot: &mut f64,
        var_x2_dn0_slot: &mut f64,
        var_x2_dn10_slot: &mut f64,
        var_x2_dn11_slot: &mut f64,
        var_x2_dn12_slot: &mut f64,
        var_x2_dn17_slot: &mut f64,
        var_x2_dn2_slot: &mut f64,
        var_x2_dn6_slot: &mut f64,
        var_x2_dn7_slot: &mut f64,
        var_xmax2_slot: &mut f64,
        var_xmax2_dn0_slot: &mut f64,
        var_xmax2_dn10_slot: &mut f64,
        var_xmax2_dn11_slot: &mut f64,
        var_xmax2_dn12_slot: &mut f64,
        var_xmax2_dn17_slot: &mut f64,
        var_xmax2_dn2_slot: &mut f64,
        var_xmax2_dn6_slot: &mut f64,
        var_xmax2_dn7_slot: &mut f64,
        var_xmp_slot: &mut f64,
        var_xmp_dn0_slot: &mut f64,
        var_xmp_dn10_slot: &mut f64,
        var_xmp_dn11_slot: &mut f64,
        var_xmp_dn12_slot: &mut f64,
        var_xmp_dn17_slot: &mut f64,
        var_xmp_dn2_slot: &mut f64,
        var_xmp_dn6_slot: &mut f64,
        var_xmp_dn7_slot: &mut f64,
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn11_slot: &mut f64,
        var_xp_dn12_slot: &mut f64,
        var_xp_dn17_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn7_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn10: f64 = *var_arg_dn10_slot;
        let mut var_arg_dn11: f64 = *var_arg_dn11_slot;
        let mut var_arg_dn12: f64 = *var_arg_dn12_slot;
        let mut var_arg_dn17: f64 = *var_arg_dn17_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_c_w_soi: f64 = *var_c_w_soi_slot;
        let mut var_c_w_soi_dn0: f64 = *var_c_w_soi_dn0_slot;
        let mut var_c_w_soi_dn10: f64 = *var_c_w_soi_dn10_slot;
        let mut var_c_w_soi_dn11: f64 = *var_c_w_soi_dn11_slot;
        let mut var_c_w_soi_dn12: f64 = *var_c_w_soi_dn12_slot;
        let mut var_c_w_soi_dn17: f64 = *var_c_w_soi_dn17_slot;
        let mut var_c_w_soi_dn2: f64 = *var_c_w_soi_dn2_slot;
        let mut var_c_w_soi_dn6: f64 = *var_c_w_soi_dn6_slot;
        let mut var_c_w_soi_dn7: f64 = *var_c_w_soi_dn7_slot;
        let mut var_cnst1bulk: f64 = *var_cnst1bulk_slot;
        let mut var_cnst1bulk_dn0: f64 = *var_cnst1bulk_dn0_slot;
        let mut var_cnst1bulk_dn10: f64 = *var_cnst1bulk_dn10_slot;
        let mut var_cnst1bulk_dn11: f64 = *var_cnst1bulk_dn11_slot;
        let mut var_cnst1bulk_dn12: f64 = *var_cnst1bulk_dn12_slot;
        let mut var_cnst1bulk_dn17: f64 = *var_cnst1bulk_dn17_slot;
        let mut var_cnst1bulk_dn2: f64 = *var_cnst1bulk_dn2_slot;
        let mut var_cnst1bulk_dn6: f64 = *var_cnst1bulk_dn6_slot;
        let mut var_cnst1bulk_dn7: f64 = *var_cnst1bulk_dn7_slot;
        let mut var_cnst1soi: f64 = *var_cnst1soi_slot;
        let mut var_cnst1soi_dn0: f64 = *var_cnst1soi_dn0_slot;
        let mut var_cnst1soi_dn10: f64 = *var_cnst1soi_dn10_slot;
        let mut var_cnst1soi_dn11: f64 = *var_cnst1soi_dn11_slot;
        let mut var_cnst1soi_dn12: f64 = *var_cnst1soi_dn12_slot;
        let mut var_cnst1soi_dn17: f64 = *var_cnst1soi_dn17_slot;
        let mut var_cnst1soi_dn2: f64 = *var_cnst1soi_dn2_slot;
        let mut var_cnst1soi_dn6: f64 = *var_cnst1soi_dn6_slot;
        let mut var_cnst1soi_dn7: f64 = *var_cnst1soi_dn7_slot;
        let mut var_cnst_2esi_q_nsubs: f64 = *var_cnst_2esi_q_nsubs_slot;
        let mut var_cnst_2esi_q_nsubs_dn0: f64 = *var_cnst_2esi_q_nsubs_dn0_slot;
        let mut var_cnst_2esi_q_nsubs_dn10: f64 = *var_cnst_2esi_q_nsubs_dn10_slot;
        let mut var_cnst_2esi_q_nsubs_dn11: f64 = *var_cnst_2esi_q_nsubs_dn11_slot;
        let mut var_cnst_2esi_q_nsubs_dn12: f64 = *var_cnst_2esi_q_nsubs_dn12_slot;
        let mut var_cnst_2esi_q_nsubs_dn17: f64 = *var_cnst_2esi_q_nsubs_dn17_slot;
        let mut var_cnst_2esi_q_nsubs_dn2: f64 = *var_cnst_2esi_q_nsubs_dn2_slot;
        let mut var_cnst_2esi_q_nsubs_dn6: f64 = *var_cnst_2esi_q_nsubs_dn6_slot;
        let mut var_cnst_2esi_q_nsubs_dn7: f64 = *var_cnst_2esi_q_nsubs_dn7_slot;
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn11: f64 = *var_dnm_dn11_slot;
        let mut var_dnm_dn12: f64 = *var_dnm_dn12_slot;
        let mut var_dnm_dn17: f64 = *var_dnm_dn17_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_guard48: f64 = *var_guard48_slot;
        let mut var_guard49: f64 = *var_guard49_slot;
        let mut var_guard50: f64 = *var_guard50_slot;
        let mut var_guard51: f64 = *var_guard51_slot;
        let mut var_guard52: f64 = *var_guard52_slot;
        let mut var_guard53: f64 = *var_guard53_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn17: f64 = *var_t3_dn17_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_vbs_bnd: f64 = *var_vbs_bnd_slot;
        let mut var_vbs_max: f64 = *var_vbs_max_slot;
        let mut var_wdsoi_ini: f64 = *var_wdsoi_ini_slot;
        let mut var_x2: f64 = *var_x2_slot;
        let mut var_x2_dn0: f64 = *var_x2_dn0_slot;
        let mut var_x2_dn10: f64 = *var_x2_dn10_slot;
        let mut var_x2_dn11: f64 = *var_x2_dn11_slot;
        let mut var_x2_dn12: f64 = *var_x2_dn12_slot;
        let mut var_x2_dn17: f64 = *var_x2_dn17_slot;
        let mut var_x2_dn2: f64 = *var_x2_dn2_slot;
        let mut var_x2_dn6: f64 = *var_x2_dn6_slot;
        let mut var_x2_dn7: f64 = *var_x2_dn7_slot;
        let mut var_xmax2: f64 = *var_xmax2_slot;
        let mut var_xmax2_dn0: f64 = *var_xmax2_dn0_slot;
        let mut var_xmax2_dn10: f64 = *var_xmax2_dn10_slot;
        let mut var_xmax2_dn11: f64 = *var_xmax2_dn11_slot;
        let mut var_xmax2_dn12: f64 = *var_xmax2_dn12_slot;
        let mut var_xmax2_dn17: f64 = *var_xmax2_dn17_slot;
        let mut var_xmax2_dn2: f64 = *var_xmax2_dn2_slot;
        let mut var_xmax2_dn6: f64 = *var_xmax2_dn6_slot;
        let mut var_xmax2_dn7: f64 = *var_xmax2_dn7_slot;
        let mut var_xmp: f64 = *var_xmp_slot;
        let mut var_xmp_dn0: f64 = *var_xmp_dn0_slot;
        let mut var_xmp_dn10: f64 = *var_xmp_dn10_slot;
        let mut var_xmp_dn11: f64 = *var_xmp_dn11_slot;
        let mut var_xmp_dn12: f64 = *var_xmp_dn12_slot;
        let mut var_xmp_dn17: f64 = *var_xmp_dn17_slot;
        let mut var_xmp_dn2: f64 = *var_xmp_dn2_slot;
        let mut var_xmp_dn6: f64 = *var_xmp_dn6_slot;
        let mut var_xmp_dn7: f64 = *var_xmp_dn7_slot;
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn11: f64 = *var_xp_dn11_slot;
        let mut var_xp_dn12: f64 = *var_xp_dn12_slot;
        let mut var_xp_dn17: f64 = *var_xp_dn17_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn7: f64 = *var_xp_dn7_slot;

        let (assign4510_e3036, assign4510_e3036_d_n0, assign4510_e3036_d_n2, assign4510_e3036_d_n6, assign4510_e3036_d_n7, assign4510_e3036_d_n10, assign4510_e3036_d_n11, assign4510_e3036_d_n12, assign4510_e3036_d_n17,) = {
    if (var_guard43 == 0.0) {
        let assign4510_e3034: f64 = (var_nin / var_mks_nsubb);
        (assign4510_e3034, (var_nin_dn0 / var_mks_nsubb), (var_nin_dn2 / var_mks_nsubb), (var_nin_dn6 / var_mks_nsubb), (var_nin_dn7 / var_mks_nsubb), (var_nin_dn10 / var_mks_nsubb), (var_nin_dn11 / var_mks_nsubb), (var_nin_dn12 / var_mks_nsubb), (var_nin_dn17 / var_mks_nsubb),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign4510_e3036;
        var_t1_dn0 = assign4510_e3036_d_n0;
        var_t1_dn2 = assign4510_e3036_d_n2;
        var_t1_dn6 = assign4510_e3036_d_n6;
        var_t1_dn7 = assign4510_e3036_d_n7;
        var_t1_dn10 = assign4510_e3036_d_n10;
        var_t1_dn11 = assign4510_e3036_d_n11;
        var_t1_dn12 = assign4510_e3036_d_n12;
        var_t1_dn17 = assign4510_e3036_d_n17;

        let (assign4520_e3043, assign4520_e3043_d_n0, assign4520_e3043_d_n2, assign4520_e3043_d_n6, assign4520_e3043_d_n7, assign4520_e3043_d_n10, assign4520_e3043_d_n11, assign4520_e3043_d_n12, assign4520_e3043_d_n17,) = {
    if (var_guard43 == 0.0) {
        let assign4520_e3041: f64 = (var_t1 * var_t1);
        (assign4520_e3041, ((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)), ((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)), ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)), ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)), ((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)), ((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)), ((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12)), ((var_t1_dn17 * var_t1) + (var_t1 * var_t1_dn17)),)
    } else {
        (var_cnst1bulk, var_cnst1bulk_dn0, var_cnst1bulk_dn2, var_cnst1bulk_dn6, var_cnst1bulk_dn7, var_cnst1bulk_dn10, var_cnst1bulk_dn11, var_cnst1bulk_dn12, var_cnst1bulk_dn17,)
    }
};
        var_cnst1bulk = assign4520_e3043;
        var_cnst1bulk_dn0 = assign4520_e3043_d_n0;
        var_cnst1bulk_dn2 = assign4520_e3043_d_n2;
        var_cnst1bulk_dn6 = assign4520_e3043_d_n6;
        var_cnst1bulk_dn7 = assign4520_e3043_d_n7;
        var_cnst1bulk_dn10 = assign4520_e3043_d_n10;
        var_cnst1bulk_dn11 = assign4520_e3043_d_n11;
        var_cnst1bulk_dn12 = assign4520_e3043_d_n12;
        var_cnst1bulk_dn17 = assign4520_e3043_d_n17;

        let (assign4530_e3050, assign4530_e3050_d_n0, assign4530_e3050_d_n2, assign4530_e3050_d_n6, assign4530_e3050_d_n7, assign4530_e3050_d_n10, assign4530_e3050_d_n11, assign4530_e3050_d_n12, assign4530_e3050_d_n17,) = {
    if (var_guard43 == 0.0) {
        let assign4530_e3048: f64 = (var_nin / var_uc_nsubs);
        (assign4530_e3048, (((var_nin_dn0 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn0)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn2 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn2)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn6 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn6)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn7 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn7)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn10 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn10)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn11 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn11)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn12 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn12)) / (var_uc_nsubs * var_uc_nsubs)), (((var_nin_dn17 * var_uc_nsubs) - (var_nin * var_uc_nsubs_dn17)) / (var_uc_nsubs * var_uc_nsubs)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign4530_e3050;
        var_t1_dn0 = assign4530_e3050_d_n0;
        var_t1_dn2 = assign4530_e3050_d_n2;
        var_t1_dn6 = assign4530_e3050_d_n6;
        var_t1_dn7 = assign4530_e3050_d_n7;
        var_t1_dn10 = assign4530_e3050_d_n10;
        var_t1_dn11 = assign4530_e3050_d_n11;
        var_t1_dn12 = assign4530_e3050_d_n12;
        var_t1_dn17 = assign4530_e3050_d_n17;

        let assign4540_e3053: f64 = (var_t1 * var_t1);
        var_cnst1soi = assign4540_e3053;
        var_cnst1soi_dn0 = ((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0));
        var_cnst1soi_dn2 = ((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2));
        var_cnst1soi_dn6 = ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6));
        var_cnst1soi_dn7 = ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7));
        var_cnst1soi_dn10 = ((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10));
        var_cnst1soi_dn11 = ((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11));
        var_cnst1soi_dn12 = ((var_t1_dn12 * var_t1) + (var_t1 * var_t1_dn12));
        var_cnst1soi_dn17 = ((var_t1_dn17 * var_t1) + (var_t1 * var_t1_dn17));

        let assign4550_e3057: f64 = (1.034943e-10 / var_q_nsub);
        let assign4550_e3059: f64 = (assign4550_e3057 / var_beta);
        let assign4550_e3060: f64 = (2.0 * assign4550_e3059);
        let assign4550_e3061: f64 = (assign4550_e3060).sqrt();
        var_c_w_soi = assign4550_e3061;
        var_c_w_soi_dn0 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn0) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4550_e3061));
        var_c_w_soi_dn2 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn2) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4550_e3061));
        var_c_w_soi_dn6 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn6) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4550_e3061));
        var_c_w_soi_dn7 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn7) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4550_e3061));
        var_c_w_soi_dn10 = ((2.0 * ((((-((1.034943e-10 * var_q_nsub_dn10) / (var_q_nsub * var_q_nsub))) * var_beta) - (assign4550_e3057 * var_beta_dn10)) / (var_beta * var_beta))) / (2.0 * assign4550_e3061));
        var_c_w_soi_dn11 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn11) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4550_e3061));
        var_c_w_soi_dn12 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn12) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4550_e3061));
        var_c_w_soi_dn17 = ((2.0 * ((-((1.034943e-10 * var_q_nsub_dn17) / (var_q_nsub * var_q_nsub))) / var_beta)) / (2.0 * assign4550_e3061));

        let assign4560_e3064: f64 = (2.0 * 1.034943e-10);
        let assign4560_e3066: f64 = (assign4560_e3064 / 1.6021918e-19);
        let assign4560_e3068: f64 = (assign4560_e3066 / var_uc_nsubs);
        var_cnst_2esi_q_nsubs = assign4560_e3068;
        var_cnst_2esi_q_nsubs_dn0 = (-((assign4560_e3066 * var_uc_nsubs_dn0) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn2 = (-((assign4560_e3066 * var_uc_nsubs_dn2) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn6 = (-((assign4560_e3066 * var_uc_nsubs_dn6) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn7 = (-((assign4560_e3066 * var_uc_nsubs_dn7) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn10 = (-((assign4560_e3066 * var_uc_nsubs_dn10) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn11 = (-((assign4560_e3066 * var_uc_nsubs_dn11) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn12 = (-((assign4560_e3066 * var_uc_nsubs_dn12) / (var_uc_nsubs * var_uc_nsubs)));
        var_cnst_2esi_q_nsubs_dn17 = (-((assign4560_e3066 * var_uc_nsubs_dn17) / (var_uc_nsubs * var_uc_nsubs)));

        let assign4570_e3071: f64 = (2.0 * 1.034943e-10);
        let assign4570_e3073: f64 = (assign4570_e3071 / 1.6021918e-19);
        let assign4570_e3075: f64 = (assign4570_e3073 * var_pb2);
        let assign4570_e3077: f64 = (assign4570_e3075 / var_uc_nsubs);
        let assign4570_e3078: f64 = (assign4570_e3077).sqrt();
        var_wdsoi_ini = assign4570_e3078;

        let assign4650_e3103: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard48 = assign4650_e3103;

        let (assign4660_e3107,) = {
    if (var_guard48 != 0.0) {
        (0.4,)
    } else {
        (var_vbs_bnd,)
    }
};
        var_vbs_bnd = assign4660_e3107;

        let (assign4670_e3111,) = {
    if (var_guard48 != 0.0) {
        (0.8,)
    } else {
        (var_vbs_max,)
    }
};
        var_vbs_max = assign4670_e3111;

        let (assign4680_e3116,) = {
    if (var_guard48 == 0.0) {
        (0.8,)
    } else {
        (var_vbs_bnd,)
    }
};
        var_vbs_bnd = assign4680_e3116;

        let (assign4690_e3121,) = {
    if (var_guard48 == 0.0) {
        (1.2,)
    } else {
        (var_vbs_max,)
    }
};
        var_vbs_max = assign4690_e3121;

        let assign4700_e3125: f64 = (var_vbs_max * 0.5);
        let assign4700_e3126: f64 = if var_vbs_bnd > assign4700_e3125 { 1.0 } else { 0.0 };
        var_guard49 = assign4700_e3126;

        let (assign4710_e3132,) = {
    if (var_guard49 != 0.0) {
        let assign4710_e3130: f64 = (0.5 * var_vbs_max);
        (assign4710_e3130,)
    } else {
        (var_vbs_bnd,)
    }
};
        var_vbs_bnd = assign4710_e3132;

        let assign4720_e3135: f64 = if var_vbs > var_vbs_bnd { 1.0 } else { 0.0 };
        var_guard50 = assign4720_e3135;

        let (assign4730_e3141, assign4730_e3141_d_n0, assign4730_e3141_d_n2, assign4730_e3141_d_n6, assign4730_e3141_d_n7, assign4730_e3141_d_n10, assign4730_e3141_d_n11, assign4730_e3141_d_n12, assign4730_e3141_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign4730_e3139: f64 = (var_vbs - var_vbs_bnd);
        (assign4730_e3139, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign4730_e3141;
        var_t2_dn0 = assign4730_e3141_d_n0;
        var_t2_dn2 = assign4730_e3141_d_n2;
        var_t2_dn6 = assign4730_e3141_d_n6;
        var_t2_dn7 = assign4730_e3141_d_n7;
        var_t2_dn10 = assign4730_e3141_d_n10;
        var_t2_dn11 = assign4730_e3141_d_n11;
        var_t2_dn12 = assign4730_e3141_d_n12;
        var_t2_dn17 = assign4730_e3141_d_n17;

        let (assign4740_e3147, assign4740_e3147_d_n0, assign4740_e3147_d_n2, assign4740_e3147_d_n6, assign4740_e3147_d_n7, assign4740_e3147_d_n10, assign4740_e3147_d_n11, assign4740_e3147_d_n12, assign4740_e3147_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign4740_e3145: f64 = (var_vbs_max - var_vbs_bnd);
        (assign4740_e3145, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign4740_e3147;
        var_t3_dn0 = assign4740_e3147_d_n0;
        var_t3_dn2 = assign4740_e3147_d_n2;
        var_t3_dn6 = assign4740_e3147_d_n6;
        var_t3_dn7 = assign4740_e3147_d_n7;
        var_t3_dn10 = assign4740_e3147_d_n10;
        var_t3_dn11 = assign4740_e3147_d_n11;
        var_t3_dn12 = assign4740_e3147_d_n12;
        var_t3_dn17 = assign4740_e3147_d_n17;

        let (assign4750_e3153, assign4750_e3153_d_n0, assign4750_e3153_d_n2, assign4750_e3153_d_n6, assign4750_e3153_d_n7, assign4750_e3153_d_n10, assign4750_e3153_d_n11, assign4750_e3153_d_n12, assign4750_e3153_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign4750_e3151: f64 = (var_t2 * var_t2);
        (assign4750_e3151, ((var_t2_dn0 * var_t2) + (var_t2 * var_t2_dn0)), ((var_t2_dn2 * var_t2) + (var_t2 * var_t2_dn2)), ((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6)), ((var_t2_dn7 * var_t2) + (var_t2 * var_t2_dn7)), ((var_t2_dn10 * var_t2) + (var_t2 * var_t2_dn10)), ((var_t2_dn11 * var_t2) + (var_t2 * var_t2_dn11)), ((var_t2_dn12 * var_t2) + (var_t2 * var_t2_dn12)), ((var_t2_dn17 * var_t2) + (var_t2 * var_t2_dn17)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn6, var_x2_dn7, var_x2_dn10, var_x2_dn11, var_x2_dn12, var_x2_dn17,)
    }
};
        var_x2 = assign4750_e3153;
        var_x2_dn0 = assign4750_e3153_d_n0;
        var_x2_dn2 = assign4750_e3153_d_n2;
        var_x2_dn6 = assign4750_e3153_d_n6;
        var_x2_dn7 = assign4750_e3153_d_n7;
        var_x2_dn10 = assign4750_e3153_d_n10;
        var_x2_dn11 = assign4750_e3153_d_n11;
        var_x2_dn12 = assign4750_e3153_d_n12;
        var_x2_dn17 = assign4750_e3153_d_n17;

        let (assign4760_e3159, assign4760_e3159_d_n0, assign4760_e3159_d_n2, assign4760_e3159_d_n6, assign4760_e3159_d_n7, assign4760_e3159_d_n10, assign4760_e3159_d_n11, assign4760_e3159_d_n12, assign4760_e3159_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign4760_e3157: f64 = (var_t3 * var_t3);
        (assign4760_e3157, ((var_t3_dn0 * var_t3) + (var_t3 * var_t3_dn0)), ((var_t3_dn2 * var_t3) + (var_t3 * var_t3_dn2)), ((var_t3_dn6 * var_t3) + (var_t3 * var_t3_dn6)), ((var_t3_dn7 * var_t3) + (var_t3 * var_t3_dn7)), ((var_t3_dn10 * var_t3) + (var_t3 * var_t3_dn10)), ((var_t3_dn11 * var_t3) + (var_t3 * var_t3_dn11)), ((var_t3_dn12 * var_t3) + (var_t3 * var_t3_dn12)), ((var_t3_dn17 * var_t3) + (var_t3 * var_t3_dn17)),)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn6, var_xmax2_dn7, var_xmax2_dn10, var_xmax2_dn11, var_xmax2_dn12, var_xmax2_dn17,)
    }
};
        var_xmax2 = assign4760_e3159;
        var_xmax2_dn0 = assign4760_e3159_d_n0;
        var_xmax2_dn2 = assign4760_e3159_d_n2;
        var_xmax2_dn6 = assign4760_e3159_d_n6;
        var_xmax2_dn7 = assign4760_e3159_d_n7;
        var_xmax2_dn10 = assign4760_e3159_d_n10;
        var_xmax2_dn11 = assign4760_e3159_d_n11;
        var_xmax2_dn12 = assign4760_e3159_d_n12;
        var_xmax2_dn17 = assign4760_e3159_d_n17;

        let (assign4770_e3163, assign4770_e3163_d_n0, assign4770_e3163_d_n2, assign4770_e3163_d_n6, assign4770_e3163_d_n7, assign4770_e3163_d_n10, assign4770_e3163_d_n11, assign4770_e3163_d_n12, assign4770_e3163_d_n17,) = {
    if (var_guard50 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign4770_e3163;
        var_xp_dn0 = assign4770_e3163_d_n0;
        var_xp_dn2 = assign4770_e3163_d_n2;
        var_xp_dn6 = assign4770_e3163_d_n6;
        var_xp_dn7 = assign4770_e3163_d_n7;
        var_xp_dn10 = assign4770_e3163_d_n10;
        var_xp_dn11 = assign4770_e3163_d_n11;
        var_xp_dn12 = assign4770_e3163_d_n12;
        var_xp_dn17 = assign4770_e3163_d_n17;

        let (assign4780_e3167, assign4780_e3167_d_n0, assign4780_e3167_d_n2, assign4780_e3167_d_n6, assign4780_e3167_d_n7, assign4780_e3167_d_n10, assign4780_e3167_d_n11, assign4780_e3167_d_n12, assign4780_e3167_d_n17,) = {
    if (var_guard50 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign4780_e3167;
        var_xmp_dn0 = assign4780_e3167_d_n0;
        var_xmp_dn2 = assign4780_e3167_d_n2;
        var_xmp_dn6 = assign4780_e3167_d_n6;
        var_xmp_dn7 = assign4780_e3167_d_n7;
        var_xmp_dn10 = assign4780_e3167_d_n10;
        var_xmp_dn11 = assign4780_e3167_d_n11;
        var_xmp_dn12 = assign4780_e3167_d_n12;
        var_xmp_dn17 = assign4780_e3167_d_n17;

        let (assign4790_e3171,) = {
    if (var_guard50 != 0.0) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign4790_e3171;

        let (assign4800_e3175,) = {
    if (var_guard50 != 0.0) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign4800_e3175;

        let (assign4810_e3179, assign4810_e3179_d_n0, assign4810_e3179_d_n2, assign4810_e3179_d_n6, assign4810_e3179_d_n7, assign4810_e3179_d_n10, assign4810_e3179_d_n11, assign4810_e3179_d_n12, assign4810_e3179_d_n17,) = {
    if (var_guard50 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    }
};
        var_arg = assign4810_e3179;
        var_arg_dn0 = assign4810_e3179_d_n0;
        var_arg_dn2 = assign4810_e3179_d_n2;
        var_arg_dn6 = assign4810_e3179_d_n6;
        var_arg_dn7 = assign4810_e3179_d_n7;
        var_arg_dn10 = assign4810_e3179_d_n10;
        var_arg_dn11 = assign4810_e3179_d_n11;
        var_arg_dn12 = assign4810_e3179_d_n12;
        var_arg_dn17 = assign4810_e3179_d_n17;

        let (assign4820_e3183, assign4820_e3183_d_n0, assign4820_e3183_d_n2, assign4820_e3183_d_n6, assign4820_e3183_d_n7, assign4820_e3183_d_n10, assign4820_e3183_d_n11, assign4820_e3183_d_n12, assign4820_e3183_d_n17,) = {
    if (var_guard50 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign4820_e3183;
        var_dnm_dn0 = assign4820_e3183_d_n0;
        var_dnm_dn2 = assign4820_e3183_d_n2;
        var_dnm_dn6 = assign4820_e3183_d_n6;
        var_dnm_dn7 = assign4820_e3183_d_n7;
        var_dnm_dn10 = assign4820_e3183_d_n10;
        var_dnm_dn11 = assign4820_e3183_d_n11;
        var_dnm_dn12 = assign4820_e3183_d_n12;
        var_dnm_dn17 = assign4820_e3183_d_n17;

        let (assign4830_e3189, assign4830_e3189_d_n0, assign4830_e3189_d_n2, assign4830_e3189_d_n6, assign4830_e3189_d_n7, assign4830_e3189_d_n10, assign4830_e3189_d_n11, assign4830_e3189_d_n12, assign4830_e3189_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign4830_e3187: f64 = (var_xp * var_x2);
        (assign4830_e3187, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign4830_e3189;
        var_xp_dn0 = assign4830_e3189_d_n0;
        var_xp_dn2 = assign4830_e3189_d_n2;
        var_xp_dn6 = assign4830_e3189_d_n6;
        var_xp_dn7 = assign4830_e3189_d_n7;
        var_xp_dn10 = assign4830_e3189_d_n10;
        var_xp_dn11 = assign4830_e3189_d_n11;
        var_xp_dn12 = assign4830_e3189_d_n12;
        var_xp_dn17 = assign4830_e3189_d_n17;

        let (assign4840_e3195, assign4840_e3195_d_n0, assign4840_e3195_d_n2, assign4840_e3195_d_n6, assign4840_e3195_d_n7, assign4840_e3195_d_n10, assign4840_e3195_d_n11, assign4840_e3195_d_n12, assign4840_e3195_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign4840_e3193: f64 = (var_xmp * var_xmax2);
        (assign4840_e3193, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign4840_e3195;
        var_xmp_dn0 = assign4840_e3195_d_n0;
        var_xmp_dn2 = assign4840_e3195_d_n2;
        var_xmp_dn6 = assign4840_e3195_d_n6;
        var_xmp_dn7 = assign4840_e3195_d_n7;
        var_xmp_dn10 = assign4840_e3195_d_n10;
        var_xmp_dn11 = assign4840_e3195_d_n11;
        var_xmp_dn12 = assign4840_e3195_d_n12;
        var_xmp_dn17 = assign4840_e3195_d_n17;

        let (assign4850_e3201, assign4850_e3201_d_n0, assign4850_e3201_d_n2, assign4850_e3201_d_n6, assign4850_e3201_d_n7, assign4850_e3201_d_n10, assign4850_e3201_d_n11, assign4850_e3201_d_n12, assign4850_e3201_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign4850_e3199: f64 = (var_xp * var_x2);
        (assign4850_e3199, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign4850_e3201;
        var_xp_dn0 = assign4850_e3201_d_n0;
        var_xp_dn2 = assign4850_e3201_d_n2;
        var_xp_dn6 = assign4850_e3201_d_n6;
        var_xp_dn7 = assign4850_e3201_d_n7;
        var_xp_dn10 = assign4850_e3201_d_n10;
        var_xp_dn11 = assign4850_e3201_d_n11;
        var_xp_dn12 = assign4850_e3201_d_n12;
        var_xp_dn17 = assign4850_e3201_d_n17;

        let (assign4860_e3207, assign4860_e3207_d_n0, assign4860_e3207_d_n2, assign4860_e3207_d_n6, assign4860_e3207_d_n7, assign4860_e3207_d_n10, assign4860_e3207_d_n11, assign4860_e3207_d_n12, assign4860_e3207_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign4860_e3205: f64 = (var_xmp * var_xmax2);
        (assign4860_e3205, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign4860_e3207;
        var_xmp_dn0 = assign4860_e3207_d_n0;
        var_xmp_dn2 = assign4860_e3207_d_n2;
        var_xmp_dn6 = assign4860_e3207_d_n6;
        var_xmp_dn7 = assign4860_e3207_d_n7;
        var_xmp_dn10 = assign4860_e3207_d_n10;
        var_xmp_dn11 = assign4860_e3207_d_n11;
        var_xmp_dn12 = assign4860_e3207_d_n12;
        var_xmp_dn17 = assign4860_e3207_d_n17;

        let (assign4870_e3213, assign4870_e3213_d_n0, assign4870_e3213_d_n2, assign4870_e3213_d_n6, assign4870_e3213_d_n7, assign4870_e3213_d_n10, assign4870_e3213_d_n11, assign4870_e3213_d_n12, assign4870_e3213_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign4870_e3211: f64 = (var_xp * var_x2);
        (assign4870_e3211, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign4870_e3213;
        var_xp_dn0 = assign4870_e3213_d_n0;
        var_xp_dn2 = assign4870_e3213_d_n2;
        var_xp_dn6 = assign4870_e3213_d_n6;
        var_xp_dn7 = assign4870_e3213_d_n7;
        var_xp_dn10 = assign4870_e3213_d_n10;
        var_xp_dn11 = assign4870_e3213_d_n11;
        var_xp_dn12 = assign4870_e3213_d_n12;
        var_xp_dn17 = assign4870_e3213_d_n17;

        let (assign4880_e3219, assign4880_e3219_d_n0, assign4880_e3219_d_n2, assign4880_e3219_d_n6, assign4880_e3219_d_n7, assign4880_e3219_d_n10, assign4880_e3219_d_n11, assign4880_e3219_d_n12, assign4880_e3219_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign4880_e3217: f64 = (var_xmp * var_xmax2);
        (assign4880_e3217, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign4880_e3219;
        var_xmp_dn0 = assign4880_e3219_d_n0;
        var_xmp_dn2 = assign4880_e3219_d_n2;
        var_xmp_dn6 = assign4880_e3219_d_n6;
        var_xmp_dn7 = assign4880_e3219_d_n7;
        var_xmp_dn10 = assign4880_e3219_d_n10;
        var_xmp_dn11 = assign4880_e3219_d_n11;
        var_xmp_dn12 = assign4880_e3219_d_n12;
        var_xmp_dn17 = assign4880_e3219_d_n17;

        let (assign4890_e3225, assign4890_e3225_d_n0, assign4890_e3225_d_n2, assign4890_e3225_d_n6, assign4890_e3225_d_n7, assign4890_e3225_d_n10, assign4890_e3225_d_n11, assign4890_e3225_d_n12, assign4890_e3225_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign4890_e3223: f64 = (var_xp * var_x2);
        (assign4890_e3223, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn11 * var_x2) + (var_xp * var_x2_dn11)), ((var_xp_dn12 * var_x2) + (var_xp * var_x2_dn12)), ((var_xp_dn17 * var_x2) + (var_xp * var_x2_dn17)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn6, var_xp_dn7, var_xp_dn10, var_xp_dn11, var_xp_dn12, var_xp_dn17,)
    }
};
        var_xp = assign4890_e3225;
        var_xp_dn0 = assign4890_e3225_d_n0;
        var_xp_dn2 = assign4890_e3225_d_n2;
        var_xp_dn6 = assign4890_e3225_d_n6;
        var_xp_dn7 = assign4890_e3225_d_n7;
        var_xp_dn10 = assign4890_e3225_d_n10;
        var_xp_dn11 = assign4890_e3225_d_n11;
        var_xp_dn12 = assign4890_e3225_d_n12;
        var_xp_dn17 = assign4890_e3225_d_n17;

        let (assign4900_e3231, assign4900_e3231_d_n0, assign4900_e3231_d_n2, assign4900_e3231_d_n6, assign4900_e3231_d_n7, assign4900_e3231_d_n10, assign4900_e3231_d_n11, assign4900_e3231_d_n12, assign4900_e3231_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign4900_e3229: f64 = (var_xmp * var_xmax2);
        (assign4900_e3229, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn11 * var_xmax2) + (var_xmp * var_xmax2_dn11)), ((var_xmp_dn12 * var_xmax2) + (var_xmp * var_xmax2_dn12)), ((var_xmp_dn17 * var_xmax2) + (var_xmp * var_xmax2_dn17)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn6, var_xmp_dn7, var_xmp_dn10, var_xmp_dn11, var_xmp_dn12, var_xmp_dn17,)
    }
};
        var_xmp = assign4900_e3231;
        var_xmp_dn0 = assign4900_e3231_d_n0;
        var_xmp_dn2 = assign4900_e3231_d_n2;
        var_xmp_dn6 = assign4900_e3231_d_n6;
        var_xmp_dn7 = assign4900_e3231_d_n7;
        var_xmp_dn10 = assign4900_e3231_d_n10;
        var_xmp_dn11 = assign4900_e3231_d_n11;
        var_xmp_dn12 = assign4900_e3231_d_n12;
        var_xmp_dn17 = assign4900_e3231_d_n17;

        let (assign4910_e3237, assign4910_e3237_d_n0, assign4910_e3237_d_n2, assign4910_e3237_d_n6, assign4910_e3237_d_n7, assign4910_e3237_d_n10, assign4910_e3237_d_n11, assign4910_e3237_d_n12, assign4910_e3237_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign4910_e3235: f64 = (var_xp + var_xmp);
        (assign4910_e3235, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn7 + var_xmp_dn7), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn11 + var_xmp_dn11), (var_xp_dn12 + var_xmp_dn12), (var_xp_dn17 + var_xmp_dn17),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    }
};
        var_arg = assign4910_e3237;
        var_arg_dn0 = assign4910_e3237_d_n0;
        var_arg_dn2 = assign4910_e3237_d_n2;
        var_arg_dn6 = assign4910_e3237_d_n6;
        var_arg_dn7 = assign4910_e3237_d_n7;
        var_arg_dn10 = assign4910_e3237_d_n10;
        var_arg_dn11 = assign4910_e3237_d_n11;
        var_arg_dn12 = assign4910_e3237_d_n12;
        var_arg_dn17 = assign4910_e3237_d_n17;

        let (assign4920_e3241, assign4920_e3241_d_n0, assign4920_e3241_d_n2, assign4920_e3241_d_n6, assign4920_e3241_d_n7, assign4920_e3241_d_n10, assign4920_e3241_d_n11, assign4920_e3241_d_n12, assign4920_e3241_d_n17,) = {
    if (var_guard50 != 0.0) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn6, var_arg_dn7, var_arg_dn10, var_arg_dn11, var_arg_dn12, var_arg_dn17,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign4920_e3241;
        var_dnm_dn0 = assign4920_e3241_d_n0;
        var_dnm_dn2 = assign4920_e3241_d_n2;
        var_dnm_dn6 = assign4920_e3241_d_n6;
        var_dnm_dn7 = assign4920_e3241_d_n7;
        var_dnm_dn10 = assign4920_e3241_d_n10;
        var_dnm_dn11 = assign4920_e3241_d_n11;
        var_dnm_dn12 = assign4920_e3241_d_n12;
        var_dnm_dn17 = assign4920_e3241_d_n17;

        let assign4930_e3256: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard51 = assign4930_e3256;

        let assign4940_e3259: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard52 = assign4940_e3259;

        let (assign4950_e3267,) = {
    if (((var_guard50 != 0.0) && (var_guard51 != 0.0)) && (var_guard52 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign4950_e3267;

        let assign4960_e3270: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard53 = assign4960_e3270;

        *var_arg_slot = var_arg;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn10_slot = var_arg_dn10;
        *var_arg_dn11_slot = var_arg_dn11;
        *var_arg_dn12_slot = var_arg_dn12;
        *var_arg_dn17_slot = var_arg_dn17;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_c_w_soi_slot = var_c_w_soi;
        *var_c_w_soi_dn0_slot = var_c_w_soi_dn0;
        *var_c_w_soi_dn10_slot = var_c_w_soi_dn10;
        *var_c_w_soi_dn11_slot = var_c_w_soi_dn11;
        *var_c_w_soi_dn12_slot = var_c_w_soi_dn12;
        *var_c_w_soi_dn17_slot = var_c_w_soi_dn17;
        *var_c_w_soi_dn2_slot = var_c_w_soi_dn2;
        *var_c_w_soi_dn6_slot = var_c_w_soi_dn6;
        *var_c_w_soi_dn7_slot = var_c_w_soi_dn7;
        *var_cnst1bulk_slot = var_cnst1bulk;
        *var_cnst1bulk_dn0_slot = var_cnst1bulk_dn0;
        *var_cnst1bulk_dn10_slot = var_cnst1bulk_dn10;
        *var_cnst1bulk_dn11_slot = var_cnst1bulk_dn11;
        *var_cnst1bulk_dn12_slot = var_cnst1bulk_dn12;
        *var_cnst1bulk_dn17_slot = var_cnst1bulk_dn17;
        *var_cnst1bulk_dn2_slot = var_cnst1bulk_dn2;
        *var_cnst1bulk_dn6_slot = var_cnst1bulk_dn6;
        *var_cnst1bulk_dn7_slot = var_cnst1bulk_dn7;
        *var_cnst1soi_slot = var_cnst1soi;
        *var_cnst1soi_dn0_slot = var_cnst1soi_dn0;
        *var_cnst1soi_dn10_slot = var_cnst1soi_dn10;
        *var_cnst1soi_dn11_slot = var_cnst1soi_dn11;
        *var_cnst1soi_dn12_slot = var_cnst1soi_dn12;
        *var_cnst1soi_dn17_slot = var_cnst1soi_dn17;
        *var_cnst1soi_dn2_slot = var_cnst1soi_dn2;
        *var_cnst1soi_dn6_slot = var_cnst1soi_dn6;
        *var_cnst1soi_dn7_slot = var_cnst1soi_dn7;
        *var_cnst_2esi_q_nsubs_slot = var_cnst_2esi_q_nsubs;
        *var_cnst_2esi_q_nsubs_dn0_slot = var_cnst_2esi_q_nsubs_dn0;
        *var_cnst_2esi_q_nsubs_dn10_slot = var_cnst_2esi_q_nsubs_dn10;
        *var_cnst_2esi_q_nsubs_dn11_slot = var_cnst_2esi_q_nsubs_dn11;
        *var_cnst_2esi_q_nsubs_dn12_slot = var_cnst_2esi_q_nsubs_dn12;
        *var_cnst_2esi_q_nsubs_dn17_slot = var_cnst_2esi_q_nsubs_dn17;
        *var_cnst_2esi_q_nsubs_dn2_slot = var_cnst_2esi_q_nsubs_dn2;
        *var_cnst_2esi_q_nsubs_dn6_slot = var_cnst_2esi_q_nsubs_dn6;
        *var_cnst_2esi_q_nsubs_dn7_slot = var_cnst_2esi_q_nsubs_dn7;
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn11_slot = var_dnm_dn11;
        *var_dnm_dn12_slot = var_dnm_dn12;
        *var_dnm_dn17_slot = var_dnm_dn17;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_guard48_slot = var_guard48;
        *var_guard49_slot = var_guard49;
        *var_guard50_slot = var_guard50;
        *var_guard51_slot = var_guard51;
        *var_guard52_slot = var_guard52;
        *var_guard53_slot = var_guard53;
        *var_m0_slot = var_m0;
        *var_mm_slot = var_mm;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn17_slot = var_t3_dn17;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_vbs_bnd_slot = var_vbs_bnd;
        *var_vbs_max_slot = var_vbs_max;
        *var_wdsoi_ini_slot = var_wdsoi_ini;
        *var_x2_slot = var_x2;
        *var_x2_dn0_slot = var_x2_dn0;
        *var_x2_dn10_slot = var_x2_dn10;
        *var_x2_dn11_slot = var_x2_dn11;
        *var_x2_dn12_slot = var_x2_dn12;
        *var_x2_dn17_slot = var_x2_dn17;
        *var_x2_dn2_slot = var_x2_dn2;
        *var_x2_dn6_slot = var_x2_dn6;
        *var_x2_dn7_slot = var_x2_dn7;
        *var_xmax2_slot = var_xmax2;
        *var_xmax2_dn0_slot = var_xmax2_dn0;
        *var_xmax2_dn10_slot = var_xmax2_dn10;
        *var_xmax2_dn11_slot = var_xmax2_dn11;
        *var_xmax2_dn12_slot = var_xmax2_dn12;
        *var_xmax2_dn17_slot = var_xmax2_dn17;
        *var_xmax2_dn2_slot = var_xmax2_dn2;
        *var_xmax2_dn6_slot = var_xmax2_dn6;
        *var_xmax2_dn7_slot = var_xmax2_dn7;
        *var_xmp_slot = var_xmp;
        *var_xmp_dn0_slot = var_xmp_dn0;
        *var_xmp_dn10_slot = var_xmp_dn10;
        *var_xmp_dn11_slot = var_xmp_dn11;
        *var_xmp_dn12_slot = var_xmp_dn12;
        *var_xmp_dn17_slot = var_xmp_dn17;
        *var_xmp_dn2_slot = var_xmp_dn2;
        *var_xmp_dn6_slot = var_xmp_dn6;
        *var_xmp_dn7_slot = var_xmp_dn7;
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn11_slot = var_xp_dn11;
        *var_xp_dn12_slot = var_xp_dn12;
        *var_xp_dn17_slot = var_xp_dn17;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn7_slot = var_xp_dn7;
    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        var_arg: f64,
        var_arg_dn0: f64,
        var_arg_dn10: f64,
        var_arg_dn11: f64,
        var_arg_dn12: f64,
        var_arg_dn17: f64,
        var_arg_dn2: f64,
        var_arg_dn6: f64,
        var_arg_dn7: f64,
        var_guard50: f64,
        var_guard51: f64,
        var_guard52: f64,
        var_guard53: f64,
        var_t2: f64,
        var_t2_dn0: f64,
        var_t2_dn10: f64,
        var_t2_dn11: f64,
        var_t2_dn12: f64,
        var_t2_dn17: f64,
        var_t2_dn2: f64,
        var_t2_dn6: f64,
        var_t2_dn7: f64,
        var_t3: f64,
        var_t3_dn0: f64,
        var_t3_dn10: f64,
        var_t3_dn11: f64,
        var_t3_dn12: f64,
        var_t3_dn17: f64,
        var_t3_dn2: f64,
        var_t3_dn6: f64,
        var_t3_dn7: f64,
        var_vbs_bnd: f64,
        var_xmp: f64,
        var_xmp_dn0: f64,
        var_xmp_dn10: f64,
        var_xmp_dn11: f64,
        var_xmp_dn12: f64,
        var_xmp_dn17: f64,
        var_xmp_dn2: f64,
        var_xmp_dn6: f64,
        var_xmp_dn7: f64,
        var_ai_slot: &mut f64,
        var_ai_dn0_slot: &mut f64,
        var_ai_dn10_slot: &mut f64,
        var_ai_dn11_slot: &mut f64,
        var_ai_dn12_slot: &mut f64,
        var_ai_dn17_slot: &mut f64,
        var_ai_dn2_slot: &mut f64,
        var_ai_dn6_slot: &mut f64,
        var_ai_dn7_slot: &mut f64,
        var_c2_slot: &mut f64,
        var_c2_dn0_slot: &mut f64,
        var_c2_dn10_slot: &mut f64,
        var_c2_dn11_slot: &mut f64,
        var_c2_dn12_slot: &mut f64,
        var_c2_dn17_slot: &mut f64,
        var_c2_dn2_slot: &mut f64,
        var_c2_dn6_slot: &mut f64,
        var_c2_dn7_slot: &mut f64,
        var_db_slot: &mut f64,
        var_db_dn0_slot: &mut f64,
        var_db_dn10_slot: &mut f64,
        var_db_dn11_slot: &mut f64,
        var_db_dn12_slot: &mut f64,
        var_db_dn17_slot: &mut f64,
        var_db_dn2_slot: &mut f64,
        var_db_dn6_slot: &mut f64,
        var_db_dn7_slot: &mut f64,
        var_di_slot: &mut f64,
        var_di_dn0_slot: &mut f64,
        var_di_dn10_slot: &mut f64,
        var_di_dn11_slot: &mut f64,
        var_di_dn12_slot: &mut f64,
        var_di_dn17_slot: &mut f64,
        var_di_dn2_slot: &mut f64,
        var_di_dn6_slot: &mut f64,
        var_di_dn7_slot: &mut f64,
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn11_slot: &mut f64,
        var_dnm_dn12_slot: &mut f64,
        var_dnm_dn17_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_flg_pprv_slot: &mut f64,
        var_guard54_slot: &mut f64,
        var_guard55_slot: &mut f64,
        var_guard57_slot: &mut f64,
        var_guard58_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
        var_lp_sl_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_pbs0_ini_slot: &mut f64,
        var_pbsl_ini_slot: &mut f64,
        var_psb0_ini_slot: &mut f64,
        var_psbl_ini_slot: &mut f64,
        var_pss0_ini_slot: &mut f64,
        var_pssl_ini_slot: &mut f64,
        var_t1__blk56_slot: &mut f64,
        var_t1__blk56_dn0_slot: &mut f64,
        var_t1__blk56_dn10_slot: &mut f64,
        var_t1__blk56_dn11_slot: &mut f64,
        var_t1__blk56_dn12_slot: &mut f64,
        var_t1__blk56_dn17_slot: &mut f64,
        var_t1__blk56_dn2_slot: &mut f64,
        var_t1__blk56_dn6_slot: &mut f64,
        var_t1__blk56_dn7_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn17_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn0_slot: &mut f64,
        var_t8_dn10_slot: &mut f64,
        var_t8_dn11_slot: &mut f64,
        var_t8_dn12_slot: &mut f64,
        var_t8_dn17_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn17_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_vbs_slot: &mut f64,
        var_vbs_dn0_slot: &mut f64,
        var_vbs_dn10_slot: &mut f64,
        var_vbs_dn11_slot: &mut f64,
        var_vbs_dn12_slot: &mut f64,
        var_vbs_dn17_slot: &mut f64,
        var_vbs_dn2_slot: &mut f64,
        var_vbs_dn6_slot: &mut f64,
        var_vbs_dn7_slot: &mut f64,
        var_vbsc_slot: &mut f64,
        var_vbsc_dn0_slot: &mut f64,
        var_vbsc_dn10_slot: &mut f64,
        var_vbsc_dn11_slot: &mut f64,
        var_vbsc_dn12_slot: &mut f64,
        var_vbsc_dn17_slot: &mut f64,
        var_vbsc_dn2_slot: &mut f64,
        var_vbsc_dn6_slot: &mut f64,
        var_vbsc_dn7_slot: &mut f64,
        var_vbsc_dvbse_slot: &mut f64,
        var_vbsc_dvbse_dn0_slot: &mut f64,
        var_vbsc_dvbse_dn10_slot: &mut f64,
        var_vbsc_dvbse_dn11_slot: &mut f64,
        var_vbsc_dvbse_dn12_slot: &mut f64,
        var_vbsc_dvbse_dn17_slot: &mut f64,
        var_vbsc_dvbse_dn2_slot: &mut f64,
        var_vbsc_dvbse_dn6_slot: &mut f64,
        var_vbsc_dvbse_dn7_slot: &mut f64,
        var_vbsp_slot: &mut f64,
        var_vbsp_dn0_slot: &mut f64,
        var_vbsp_dn10_slot: &mut f64,
        var_vbsp_dn11_slot: &mut f64,
        var_vbsp_dn12_slot: &mut f64,
        var_vbsp_dn17_slot: &mut f64,
        var_vbsp_dn2_slot: &mut f64,
        var_vbsp_dn6_slot: &mut f64,
        var_vbsp_dn7_slot: &mut f64,
        var_vbsz_slot: &mut f64,
        var_vbsz_dn0_slot: &mut f64,
        var_vbsz_dn10_slot: &mut f64,
        var_vbsz_dn11_slot: &mut f64,
        var_vbsz_dn12_slot: &mut f64,
        var_vbsz_dn17_slot: &mut f64,
        var_vbsz_dn2_slot: &mut f64,
        var_vbsz_dn6_slot: &mut f64,
        var_vbsz_dn7_slot: &mut f64,
        var_vds_slot: &mut f64,
        var_vds_dn0_slot: &mut f64,
        var_vds_dn10_slot: &mut f64,
        var_vds_dn11_slot: &mut f64,
        var_vds_dn12_slot: &mut f64,
        var_vds_dn17_slot: &mut f64,
        var_vds_dn2_slot: &mut f64,
        var_vds_dn6_slot: &mut f64,
        var_vds_dn7_slot: &mut f64,
        var_vdsc_slot: &mut f64,
        var_vdsc_dn0_slot: &mut f64,
        var_vdsc_dn10_slot: &mut f64,
        var_vdsc_dn11_slot: &mut f64,
        var_vdsc_dn12_slot: &mut f64,
        var_vdsc_dn17_slot: &mut f64,
        var_vdsc_dn2_slot: &mut f64,
        var_vdsc_dn6_slot: &mut f64,
        var_vdsc_dn7_slot: &mut f64,
        var_vdsz_slot: &mut f64,
        var_vdsz_dn0_slot: &mut f64,
        var_vdsz_dn10_slot: &mut f64,
        var_vdsz_dn11_slot: &mut f64,
        var_vdsz_dn12_slot: &mut f64,
        var_vdsz_dn17_slot: &mut f64,
        var_vdsz_dn2_slot: &mut f64,
        var_vdsz_dn6_slot: &mut f64,
        var_vdsz_dn7_slot: &mut f64,
        var_vgs_slot: &mut f64,
        var_vgs_dn11_slot: &mut f64,
        var_vgs_dn6_slot: &mut f64,
        var_vgs_dn7_slot: &mut f64,
        var_vgsc_slot: &mut f64,
        var_vgsc_dn11_slot: &mut f64,
        var_vgsc_dn6_slot: &mut f64,
        var_vgsc_dn7_slot: &mut f64,
        var_vgsz_slot: &mut f64,
        var_vgsz_dn0_slot: &mut f64,
        var_vgsz_dn10_slot: &mut f64,
        var_vgsz_dn11_slot: &mut f64,
        var_vgsz_dn12_slot: &mut f64,
        var_vgsz_dn17_slot: &mut f64,
        var_vgsz_dn2_slot: &mut f64,
        var_vgsz_dn6_slot: &mut f64,
        var_vgsz_dn7_slot: &mut f64,
        var_vzadd_slot: &mut f64,
        var_vzadd_dn0_slot: &mut f64,
        var_vzadd_dn10_slot: &mut f64,
        var_vzadd_dn11_slot: &mut f64,
        var_vzadd_dn12_slot: &mut f64,
        var_vzadd_dn17_slot: &mut f64,
        var_vzadd_dn2_slot: &mut f64,
        var_vzadd_dn6_slot: &mut f64,
        var_vzadd_dn7_slot: &mut f64,
    ) {
        let mut var_ai: f64 = *var_ai_slot;
        let mut var_ai_dn0: f64 = *var_ai_dn0_slot;
        let mut var_ai_dn10: f64 = *var_ai_dn10_slot;
        let mut var_ai_dn11: f64 = *var_ai_dn11_slot;
        let mut var_ai_dn12: f64 = *var_ai_dn12_slot;
        let mut var_ai_dn17: f64 = *var_ai_dn17_slot;
        let mut var_ai_dn2: f64 = *var_ai_dn2_slot;
        let mut var_ai_dn6: f64 = *var_ai_dn6_slot;
        let mut var_ai_dn7: f64 = *var_ai_dn7_slot;
        let mut var_c2: f64 = *var_c2_slot;
        let mut var_c2_dn0: f64 = *var_c2_dn0_slot;
        let mut var_c2_dn10: f64 = *var_c2_dn10_slot;
        let mut var_c2_dn11: f64 = *var_c2_dn11_slot;
        let mut var_c2_dn12: f64 = *var_c2_dn12_slot;
        let mut var_c2_dn17: f64 = *var_c2_dn17_slot;
        let mut var_c2_dn2: f64 = *var_c2_dn2_slot;
        let mut var_c2_dn6: f64 = *var_c2_dn6_slot;
        let mut var_c2_dn7: f64 = *var_c2_dn7_slot;
        let mut var_db: f64 = *var_db_slot;
        let mut var_db_dn0: f64 = *var_db_dn0_slot;
        let mut var_db_dn10: f64 = *var_db_dn10_slot;
        let mut var_db_dn11: f64 = *var_db_dn11_slot;
        let mut var_db_dn12: f64 = *var_db_dn12_slot;
        let mut var_db_dn17: f64 = *var_db_dn17_slot;
        let mut var_db_dn2: f64 = *var_db_dn2_slot;
        let mut var_db_dn6: f64 = *var_db_dn6_slot;
        let mut var_db_dn7: f64 = *var_db_dn7_slot;
        let mut var_di: f64 = *var_di_slot;
        let mut var_di_dn0: f64 = *var_di_dn0_slot;
        let mut var_di_dn10: f64 = *var_di_dn10_slot;
        let mut var_di_dn11: f64 = *var_di_dn11_slot;
        let mut var_di_dn12: f64 = *var_di_dn12_slot;
        let mut var_di_dn17: f64 = *var_di_dn17_slot;
        let mut var_di_dn2: f64 = *var_di_dn2_slot;
        let mut var_di_dn6: f64 = *var_di_dn6_slot;
        let mut var_di_dn7: f64 = *var_di_dn7_slot;
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn11: f64 = *var_dnm_dn11_slot;
        let mut var_dnm_dn12: f64 = *var_dnm_dn12_slot;
        let mut var_dnm_dn17: f64 = *var_dnm_dn17_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_flg_pprv: f64 = *var_flg_pprv_slot;
        let mut var_guard54: f64 = *var_guard54_slot;
        let mut var_guard55: f64 = *var_guard55_slot;
        let mut var_guard57: f64 = *var_guard57_slot;
        let mut var_guard58: f64 = *var_guard58_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
        let mut var_lp_sl: f64 = *var_lp_sl_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_pbs0_ini: f64 = *var_pbs0_ini_slot;
        let mut var_pbsl_ini: f64 = *var_pbsl_ini_slot;
        let mut var_psb0_ini: f64 = *var_psb0_ini_slot;
        let mut var_psbl_ini: f64 = *var_psbl_ini_slot;
        let mut var_pss0_ini: f64 = *var_pss0_ini_slot;
        let mut var_pssl_ini: f64 = *var_pssl_ini_slot;
        let mut var_t1__blk56: f64 = *var_t1__blk56_slot;
        let mut var_t1__blk56_dn0: f64 = *var_t1__blk56_dn0_slot;
        let mut var_t1__blk56_dn10: f64 = *var_t1__blk56_dn10_slot;
        let mut var_t1__blk56_dn11: f64 = *var_t1__blk56_dn11_slot;
        let mut var_t1__blk56_dn12: f64 = *var_t1__blk56_dn12_slot;
        let mut var_t1__blk56_dn17: f64 = *var_t1__blk56_dn17_slot;
        let mut var_t1__blk56_dn2: f64 = *var_t1__blk56_dn2_slot;
        let mut var_t1__blk56_dn6: f64 = *var_t1__blk56_dn6_slot;
        let mut var_t1__blk56_dn7: f64 = *var_t1__blk56_dn7_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn17: f64 = *var_t4_dn17_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn0: f64 = *var_t8_dn0_slot;
        let mut var_t8_dn10: f64 = *var_t8_dn10_slot;
        let mut var_t8_dn11: f64 = *var_t8_dn11_slot;
        let mut var_t8_dn12: f64 = *var_t8_dn12_slot;
        let mut var_t8_dn17: f64 = *var_t8_dn17_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn17: f64 = *var_tmf2_dn17_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_vbs: f64 = *var_vbs_slot;
        let mut var_vbs_dn0: f64 = *var_vbs_dn0_slot;
        let mut var_vbs_dn10: f64 = *var_vbs_dn10_slot;
        let mut var_vbs_dn11: f64 = *var_vbs_dn11_slot;
        let mut var_vbs_dn12: f64 = *var_vbs_dn12_slot;
        let mut var_vbs_dn17: f64 = *var_vbs_dn17_slot;
        let mut var_vbs_dn2: f64 = *var_vbs_dn2_slot;
        let mut var_vbs_dn6: f64 = *var_vbs_dn6_slot;
        let mut var_vbs_dn7: f64 = *var_vbs_dn7_slot;
        let mut var_vbsc: f64 = *var_vbsc_slot;
        let mut var_vbsc_dn0: f64 = *var_vbsc_dn0_slot;
        let mut var_vbsc_dn10: f64 = *var_vbsc_dn10_slot;
        let mut var_vbsc_dn11: f64 = *var_vbsc_dn11_slot;
        let mut var_vbsc_dn12: f64 = *var_vbsc_dn12_slot;
        let mut var_vbsc_dn17: f64 = *var_vbsc_dn17_slot;
        let mut var_vbsc_dn2: f64 = *var_vbsc_dn2_slot;
        let mut var_vbsc_dn6: f64 = *var_vbsc_dn6_slot;
        let mut var_vbsc_dn7: f64 = *var_vbsc_dn7_slot;
        let mut var_vbsc_dvbse: f64 = *var_vbsc_dvbse_slot;
        let mut var_vbsc_dvbse_dn0: f64 = *var_vbsc_dvbse_dn0_slot;
        let mut var_vbsc_dvbse_dn10: f64 = *var_vbsc_dvbse_dn10_slot;
        let mut var_vbsc_dvbse_dn11: f64 = *var_vbsc_dvbse_dn11_slot;
        let mut var_vbsc_dvbse_dn12: f64 = *var_vbsc_dvbse_dn12_slot;
        let mut var_vbsc_dvbse_dn17: f64 = *var_vbsc_dvbse_dn17_slot;
        let mut var_vbsc_dvbse_dn2: f64 = *var_vbsc_dvbse_dn2_slot;
        let mut var_vbsc_dvbse_dn6: f64 = *var_vbsc_dvbse_dn6_slot;
        let mut var_vbsc_dvbse_dn7: f64 = *var_vbsc_dvbse_dn7_slot;
        let mut var_vbsp: f64 = *var_vbsp_slot;
        let mut var_vbsp_dn0: f64 = *var_vbsp_dn0_slot;
        let mut var_vbsp_dn10: f64 = *var_vbsp_dn10_slot;
        let mut var_vbsp_dn11: f64 = *var_vbsp_dn11_slot;
        let mut var_vbsp_dn12: f64 = *var_vbsp_dn12_slot;
        let mut var_vbsp_dn17: f64 = *var_vbsp_dn17_slot;
        let mut var_vbsp_dn2: f64 = *var_vbsp_dn2_slot;
        let mut var_vbsp_dn6: f64 = *var_vbsp_dn6_slot;
        let mut var_vbsp_dn7: f64 = *var_vbsp_dn7_slot;
        let mut var_vbsz: f64 = *var_vbsz_slot;
        let mut var_vbsz_dn0: f64 = *var_vbsz_dn0_slot;
        let mut var_vbsz_dn10: f64 = *var_vbsz_dn10_slot;
        let mut var_vbsz_dn11: f64 = *var_vbsz_dn11_slot;
        let mut var_vbsz_dn12: f64 = *var_vbsz_dn12_slot;
        let mut var_vbsz_dn17: f64 = *var_vbsz_dn17_slot;
        let mut var_vbsz_dn2: f64 = *var_vbsz_dn2_slot;
        let mut var_vbsz_dn6: f64 = *var_vbsz_dn6_slot;
        let mut var_vbsz_dn7: f64 = *var_vbsz_dn7_slot;
        let mut var_vds: f64 = *var_vds_slot;
        let mut var_vds_dn0: f64 = *var_vds_dn0_slot;
        let mut var_vds_dn10: f64 = *var_vds_dn10_slot;
        let mut var_vds_dn11: f64 = *var_vds_dn11_slot;
        let mut var_vds_dn12: f64 = *var_vds_dn12_slot;
        let mut var_vds_dn17: f64 = *var_vds_dn17_slot;
        let mut var_vds_dn2: f64 = *var_vds_dn2_slot;
        let mut var_vds_dn6: f64 = *var_vds_dn6_slot;
        let mut var_vds_dn7: f64 = *var_vds_dn7_slot;
        let mut var_vdsc: f64 = *var_vdsc_slot;
        let mut var_vdsc_dn0: f64 = *var_vdsc_dn0_slot;
        let mut var_vdsc_dn10: f64 = *var_vdsc_dn10_slot;
        let mut var_vdsc_dn11: f64 = *var_vdsc_dn11_slot;
        let mut var_vdsc_dn12: f64 = *var_vdsc_dn12_slot;
        let mut var_vdsc_dn17: f64 = *var_vdsc_dn17_slot;
        let mut var_vdsc_dn2: f64 = *var_vdsc_dn2_slot;
        let mut var_vdsc_dn6: f64 = *var_vdsc_dn6_slot;
        let mut var_vdsc_dn7: f64 = *var_vdsc_dn7_slot;
        let mut var_vdsz: f64 = *var_vdsz_slot;
        let mut var_vdsz_dn0: f64 = *var_vdsz_dn0_slot;
        let mut var_vdsz_dn10: f64 = *var_vdsz_dn10_slot;
        let mut var_vdsz_dn11: f64 = *var_vdsz_dn11_slot;
        let mut var_vdsz_dn12: f64 = *var_vdsz_dn12_slot;
        let mut var_vdsz_dn17: f64 = *var_vdsz_dn17_slot;
        let mut var_vdsz_dn2: f64 = *var_vdsz_dn2_slot;
        let mut var_vdsz_dn6: f64 = *var_vdsz_dn6_slot;
        let mut var_vdsz_dn7: f64 = *var_vdsz_dn7_slot;
        let mut var_vgs: f64 = *var_vgs_slot;
        let mut var_vgs_dn11: f64 = *var_vgs_dn11_slot;
        let mut var_vgs_dn6: f64 = *var_vgs_dn6_slot;
        let mut var_vgs_dn7: f64 = *var_vgs_dn7_slot;
        let mut var_vgsc: f64 = *var_vgsc_slot;
        let mut var_vgsc_dn11: f64 = *var_vgsc_dn11_slot;
        let mut var_vgsc_dn6: f64 = *var_vgsc_dn6_slot;
        let mut var_vgsc_dn7: f64 = *var_vgsc_dn7_slot;
        let mut var_vgsz: f64 = *var_vgsz_slot;
        let mut var_vgsz_dn0: f64 = *var_vgsz_dn0_slot;
        let mut var_vgsz_dn10: f64 = *var_vgsz_dn10_slot;
        let mut var_vgsz_dn11: f64 = *var_vgsz_dn11_slot;
        let mut var_vgsz_dn12: f64 = *var_vgsz_dn12_slot;
        let mut var_vgsz_dn17: f64 = *var_vgsz_dn17_slot;
        let mut var_vgsz_dn2: f64 = *var_vgsz_dn2_slot;
        let mut var_vgsz_dn6: f64 = *var_vgsz_dn6_slot;
        let mut var_vgsz_dn7: f64 = *var_vgsz_dn7_slot;
        let mut var_vzadd: f64 = *var_vzadd_slot;
        let mut var_vzadd_dn0: f64 = *var_vzadd_dn0_slot;
        let mut var_vzadd_dn10: f64 = *var_vzadd_dn10_slot;
        let mut var_vzadd_dn11: f64 = *var_vzadd_dn11_slot;
        let mut var_vzadd_dn12: f64 = *var_vzadd_dn12_slot;
        let mut var_vzadd_dn17: f64 = *var_vzadd_dn17_slot;
        let mut var_vzadd_dn2: f64 = *var_vzadd_dn2_slot;
        let mut var_vzadd_dn6: f64 = *var_vzadd_dn6_slot;
        let mut var_vzadd_dn7: f64 = *var_vzadd_dn7_slot;

        let (assign4970_e3281,) = {
    if ((((var_guard50 != 0.0) && (var_guard51 != 0.0)) && (var_guard52 == 0.0)) && (var_guard53 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign4970_e3281;

        let assign4980_e3284: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard54 = assign4980_e3284;

        let (assign4990_e3298,) = {
    if (((((var_guard50 != 0.0) && (var_guard51 != 0.0)) && (var_guard52 == 0.0)) && (var_guard53 == 0.0)) && (var_guard54 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign4990_e3298;

        let assign5000_e3301: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard55 = assign5000_e3301;

        let (assign5010_e3318,) = {
    if ((((((var_guard50 != 0.0) && (var_guard51 != 0.0)) && (var_guard52 == 0.0)) && (var_guard53 == 0.0)) && (var_guard54 == 0.0)) && (var_guard55 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign5010_e3318;

        let (assign5020_e3324,) = {
    if ((var_guard50 != 0.0) && (var_guard51 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign5020_e3324;

        let mut assign5030_loop_guard: usize = 0;
        while {
            let assign5030_cond_e3331: f64 = if (((var_guard50 != 0.0) && (var_guard51 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign5030_cond_e3331 != 0.0
        } {
            assign5030_loop_guard += 1;
            assert!(assign5030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign5030_body0_e3338, assign5030_body0_e3338_d_n0, assign5030_body0_e3338_d_n2, assign5030_body0_e3338_d_n6, assign5030_body0_e3338_d_n7, assign5030_body0_e3338_d_n10, assign5030_body0_e3338_d_n11, assign5030_body0_e3338_d_n12, assign5030_body0_e3338_d_n17,) = {
    if ((var_guard50 != 0.0) && (var_guard51 != 0.0)) {
        let assign5030_body0_e3336: f64 = (var_dnm).sqrt();
        (assign5030_body0_e3336, (var_dnm_dn0 / (2.0 * assign5030_body0_e3336)), (var_dnm_dn2 / (2.0 * assign5030_body0_e3336)), (var_dnm_dn6 / (2.0 * assign5030_body0_e3336)), (var_dnm_dn7 / (2.0 * assign5030_body0_e3336)), (var_dnm_dn10 / (2.0 * assign5030_body0_e3336)), (var_dnm_dn11 / (2.0 * assign5030_body0_e3336)), (var_dnm_dn12 / (2.0 * assign5030_body0_e3336)), (var_dnm_dn17 / (2.0 * assign5030_body0_e3336)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
            var_dnm = assign5030_body0_e3338;
            var_dnm_dn0 = assign5030_body0_e3338_d_n0;
            var_dnm_dn2 = assign5030_body0_e3338_d_n2;
            var_dnm_dn6 = assign5030_body0_e3338_d_n6;
            var_dnm_dn7 = assign5030_body0_e3338_d_n7;
            var_dnm_dn10 = assign5030_body0_e3338_d_n10;
            var_dnm_dn11 = assign5030_body0_e3338_d_n11;
            var_dnm_dn12 = assign5030_body0_e3338_d_n12;
            var_dnm_dn17 = assign5030_body0_e3338_d_n17;
            let (assign5030_body1_e3346,) = {
    if ((var_guard50 != 0.0) && (var_guard51 != 0.0)) {
        let assign5030_body1_e3344: f64 = (var_m0 + 1.0);
        (assign5030_body1_e3344,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign5030_body1_e3346;
        }

        let (assign5040_e3359, assign5040_e3359_d_n0, assign5040_e3359_d_n2, assign5040_e3359_d_n6, assign5040_e3359_d_n7, assign5040_e3359_d_n10, assign5040_e3359_d_n11, assign5040_e3359_d_n12, assign5040_e3359_d_n17,) = {
    if ((var_guard50 != 0.0) && (var_guard51 == 0.0)) {
        let assign5040_e3355: f64 = (2.0 * 4.0);
        let assign5040_e3356: f64 = (1.0 / assign5040_e3355);
        let assign5040_e3357: f64 = (var_dnm).powf(assign5040_e3356);
        (assign5040_e3357, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((var_dnm).powf(assign5040_e3356 - 1.0) * var_dnm_dn0)) } } else { (assign5040_e3357 * (assign5040_e3356 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((var_dnm).powf(assign5040_e3356 - 1.0) * var_dnm_dn2)) } } else { (assign5040_e3357 * (assign5040_e3356 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((var_dnm).powf(assign5040_e3356 - 1.0) * var_dnm_dn6)) } } else { (assign5040_e3357 * (assign5040_e3356 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((var_dnm).powf(assign5040_e3356 - 1.0) * var_dnm_dn7)) } } else { (assign5040_e3357 * (assign5040_e3356 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((var_dnm).powf(assign5040_e3356 - 1.0) * var_dnm_dn10)) } } else { (assign5040_e3357 * (assign5040_e3356 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((var_dnm).powf(assign5040_e3356 - 1.0) * var_dnm_dn11)) } } else { (assign5040_e3357 * (assign5040_e3356 * (var_dnm_dn11 / var_dnm))) }, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((var_dnm).powf(assign5040_e3356 - 1.0) * var_dnm_dn12)) } } else { (assign5040_e3357 * (assign5040_e3356 * (var_dnm_dn12 / var_dnm))) }, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((var_dnm).powf(assign5040_e3356 - 1.0) * var_dnm_dn17)) } } else { (assign5040_e3357 * (assign5040_e3356 * (var_dnm_dn17 / var_dnm))) },)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign5040_e3359;
        var_dnm_dn0 = assign5040_e3359_d_n0;
        var_dnm_dn2 = assign5040_e3359_d_n2;
        var_dnm_dn6 = assign5040_e3359_d_n6;
        var_dnm_dn7 = assign5040_e3359_d_n7;
        var_dnm_dn10 = assign5040_e3359_d_n10;
        var_dnm_dn11 = assign5040_e3359_d_n11;
        var_dnm_dn12 = assign5040_e3359_d_n12;
        var_dnm_dn17 = assign5040_e3359_d_n17;

        let (assign5050_e3365, assign5050_e3365_d_n0, assign5050_e3365_d_n2, assign5050_e3365_d_n6, assign5050_e3365_d_n7, assign5050_e3365_d_n10, assign5050_e3365_d_n11, assign5050_e3365_d_n12, assign5050_e3365_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign5050_e3363: f64 = (1.0 / var_dnm);
        (assign5050_e3363, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn11 / (var_dnm * var_dnm))), (-(var_dnm_dn12 / (var_dnm * var_dnm))), (-(var_dnm_dn17 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn6, var_dnm_dn7, var_dnm_dn10, var_dnm_dn11, var_dnm_dn12, var_dnm_dn17,)
    }
};
        var_dnm = assign5050_e3365;
        var_dnm_dn0 = assign5050_e3365_d_n0;
        var_dnm_dn2 = assign5050_e3365_d_n2;
        var_dnm_dn6 = assign5050_e3365_d_n6;
        var_dnm_dn7 = assign5050_e3365_d_n7;
        var_dnm_dn10 = assign5050_e3365_d_n10;
        var_dnm_dn11 = assign5050_e3365_d_n11;
        var_dnm_dn12 = assign5050_e3365_d_n12;
        var_dnm_dn17 = assign5050_e3365_d_n17;

        let (assign5060_e3373, assign5060_e3373_d_n0, assign5060_e3373_d_n2, assign5060_e3373_d_n6, assign5060_e3373_d_n7, assign5060_e3373_d_n10, assign5060_e3373_d_n11, assign5060_e3373_d_n12, assign5060_e3373_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign5060_e3369: f64 = (var_t2 * var_t3);
        let assign5060_e3371: f64 = (assign5060_e3369 * var_dnm);
        (assign5060_e3371, ((((var_t2_dn0 * var_t3) + (var_t2 * var_t3_dn0)) * var_dnm) + (assign5060_e3369 * var_dnm_dn0)), ((((var_t2_dn2 * var_t3) + (var_t2 * var_t3_dn2)) * var_dnm) + (assign5060_e3369 * var_dnm_dn2)), ((((var_t2_dn6 * var_t3) + (var_t2 * var_t3_dn6)) * var_dnm) + (assign5060_e3369 * var_dnm_dn6)), ((((var_t2_dn7 * var_t3) + (var_t2 * var_t3_dn7)) * var_dnm) + (assign5060_e3369 * var_dnm_dn7)), ((((var_t2_dn10 * var_t3) + (var_t2 * var_t3_dn10)) * var_dnm) + (assign5060_e3369 * var_dnm_dn10)), ((((var_t2_dn11 * var_t3) + (var_t2 * var_t3_dn11)) * var_dnm) + (assign5060_e3369 * var_dnm_dn11)), ((((var_t2_dn12 * var_t3) + (var_t2 * var_t3_dn12)) * var_dnm) + (assign5060_e3369 * var_dnm_dn12)), ((((var_t2_dn17 * var_t3) + (var_t2 * var_t3_dn17)) * var_dnm) + (assign5060_e3369 * var_dnm_dn17)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    }
};
        var_t4 = assign5060_e3373;
        var_t4_dn0 = assign5060_e3373_d_n0;
        var_t4_dn2 = assign5060_e3373_d_n2;
        var_t4_dn6 = assign5060_e3373_d_n6;
        var_t4_dn7 = assign5060_e3373_d_n7;
        var_t4_dn10 = assign5060_e3373_d_n10;
        var_t4_dn11 = assign5060_e3373_d_n11;
        var_t4_dn12 = assign5060_e3373_d_n12;
        var_t4_dn17 = assign5060_e3373_d_n17;

        let (assign5070_e3383, assign5070_e3383_d_n0, assign5070_e3383_d_n2, assign5070_e3383_d_n6, assign5070_e3383_d_n7, assign5070_e3383_d_n10, assign5070_e3383_d_n11, assign5070_e3383_d_n12, assign5070_e3383_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign5070_e3377: f64 = (var_t3 * var_xmp);
        let assign5070_e3379: f64 = (assign5070_e3377 * var_dnm);
        let assign5070_e3381: f64 = (assign5070_e3379 / var_arg);
        (assign5070_e3381, (((((((var_t3_dn0 * var_xmp) + (var_t3 * var_xmp_dn0)) * var_dnm) + (assign5070_e3377 * var_dnm_dn0)) * var_arg) - (assign5070_e3379 * var_arg_dn0)) / (var_arg * var_arg)), (((((((var_t3_dn2 * var_xmp) + (var_t3 * var_xmp_dn2)) * var_dnm) + (assign5070_e3377 * var_dnm_dn2)) * var_arg) - (assign5070_e3379 * var_arg_dn2)) / (var_arg * var_arg)), (((((((var_t3_dn6 * var_xmp) + (var_t3 * var_xmp_dn6)) * var_dnm) + (assign5070_e3377 * var_dnm_dn6)) * var_arg) - (assign5070_e3379 * var_arg_dn6)) / (var_arg * var_arg)), (((((((var_t3_dn7 * var_xmp) + (var_t3 * var_xmp_dn7)) * var_dnm) + (assign5070_e3377 * var_dnm_dn7)) * var_arg) - (assign5070_e3379 * var_arg_dn7)) / (var_arg * var_arg)), (((((((var_t3_dn10 * var_xmp) + (var_t3 * var_xmp_dn10)) * var_dnm) + (assign5070_e3377 * var_dnm_dn10)) * var_arg) - (assign5070_e3379 * var_arg_dn10)) / (var_arg * var_arg)), (((((((var_t3_dn11 * var_xmp) + (var_t3 * var_xmp_dn11)) * var_dnm) + (assign5070_e3377 * var_dnm_dn11)) * var_arg) - (assign5070_e3379 * var_arg_dn11)) / (var_arg * var_arg)), (((((((var_t3_dn12 * var_xmp) + (var_t3 * var_xmp_dn12)) * var_dnm) + (assign5070_e3377 * var_dnm_dn12)) * var_arg) - (assign5070_e3379 * var_arg_dn12)) / (var_arg * var_arg)), (((((((var_t3_dn17 * var_xmp) + (var_t3 * var_xmp_dn17)) * var_dnm) + (assign5070_e3377 * var_dnm_dn17)) * var_arg) - (assign5070_e3379 * var_arg_dn17)) / (var_arg * var_arg)),)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn6, var_t8_dn7, var_t8_dn10, var_t8_dn11, var_t8_dn12, var_t8_dn17,)
    }
};
        var_t8 = assign5070_e3383;
        var_t8_dn0 = assign5070_e3383_d_n0;
        var_t8_dn2 = assign5070_e3383_d_n2;
        var_t8_dn6 = assign5070_e3383_d_n6;
        var_t8_dn7 = assign5070_e3383_d_n7;
        var_t8_dn10 = assign5070_e3383_d_n10;
        var_t8_dn11 = assign5070_e3383_d_n11;
        var_t8_dn12 = assign5070_e3383_d_n12;
        var_t8_dn17 = assign5070_e3383_d_n17;

        let (assign5080_e3389, assign5080_e3389_d_n0, assign5080_e3389_d_n2, assign5080_e3389_d_n6, assign5080_e3389_d_n7, assign5080_e3389_d_n10, assign5080_e3389_d_n11, assign5080_e3389_d_n12, assign5080_e3389_d_n17,) = {
    if (var_guard50 != 0.0) {
        let assign5080_e3387: f64 = (var_vbs_bnd + var_t4);
        (assign5080_e3387, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    } else {
        (var_vbsc, var_vbsc_dn0, var_vbsc_dn2, var_vbsc_dn6, var_vbsc_dn7, var_vbsc_dn10, var_vbsc_dn11, var_vbsc_dn12, var_vbsc_dn17,)
    }
};
        var_vbsc = assign5080_e3389;
        var_vbsc_dn0 = assign5080_e3389_d_n0;
        var_vbsc_dn2 = assign5080_e3389_d_n2;
        var_vbsc_dn6 = assign5080_e3389_d_n6;
        var_vbsc_dn7 = assign5080_e3389_d_n7;
        var_vbsc_dn10 = assign5080_e3389_d_n10;
        var_vbsc_dn11 = assign5080_e3389_d_n11;
        var_vbsc_dn12 = assign5080_e3389_d_n12;
        var_vbsc_dn17 = assign5080_e3389_d_n17;

        let (assign5090_e3393, assign5090_e3393_d_n0, assign5090_e3393_d_n2, assign5090_e3393_d_n6, assign5090_e3393_d_n7, assign5090_e3393_d_n10, assign5090_e3393_d_n11, assign5090_e3393_d_n12, assign5090_e3393_d_n17,) = {
    if (var_guard50 != 0.0) {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn6, var_t8_dn7, var_t8_dn10, var_t8_dn11, var_t8_dn12, var_t8_dn17,)
    } else {
        (var_vbsc_dvbse, var_vbsc_dvbse_dn0, var_vbsc_dvbse_dn2, var_vbsc_dvbse_dn6, var_vbsc_dvbse_dn7, var_vbsc_dvbse_dn10, var_vbsc_dvbse_dn11, var_vbsc_dvbse_dn12, var_vbsc_dvbse_dn17,)
    }
};
        var_vbsc_dvbse = assign5090_e3393;
        var_vbsc_dvbse_dn0 = assign5090_e3393_d_n0;
        var_vbsc_dvbse_dn2 = assign5090_e3393_d_n2;
        var_vbsc_dvbse_dn6 = assign5090_e3393_d_n6;
        var_vbsc_dvbse_dn7 = assign5090_e3393_d_n7;
        var_vbsc_dvbse_dn10 = assign5090_e3393_d_n10;
        var_vbsc_dvbse_dn11 = assign5090_e3393_d_n11;
        var_vbsc_dvbse_dn12 = assign5090_e3393_d_n12;
        var_vbsc_dvbse_dn17 = assign5090_e3393_d_n17;

        let (assign5100_e3398, assign5100_e3398_d_n0, assign5100_e3398_d_n2, assign5100_e3398_d_n6, assign5100_e3398_d_n7, assign5100_e3398_d_n10, assign5100_e3398_d_n11, assign5100_e3398_d_n12, assign5100_e3398_d_n17,) = {
    if (var_guard50 == 0.0) {
        (var_vbs, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
    } else {
        (var_vbsc, var_vbsc_dn0, var_vbsc_dn2, var_vbsc_dn6, var_vbsc_dn7, var_vbsc_dn10, var_vbsc_dn11, var_vbsc_dn12, var_vbsc_dn17,)
    }
};
        var_vbsc = assign5100_e3398;
        var_vbsc_dn0 = assign5100_e3398_d_n0;
        var_vbsc_dn2 = assign5100_e3398_d_n2;
        var_vbsc_dn6 = assign5100_e3398_d_n6;
        var_vbsc_dn7 = assign5100_e3398_d_n7;
        var_vbsc_dn10 = assign5100_e3398_d_n10;
        var_vbsc_dn11 = assign5100_e3398_d_n11;
        var_vbsc_dn12 = assign5100_e3398_d_n12;
        var_vbsc_dn17 = assign5100_e3398_d_n17;

        let (assign5110_e3403, assign5110_e3403_d_n0, assign5110_e3403_d_n2, assign5110_e3403_d_n6, assign5110_e3403_d_n7, assign5110_e3403_d_n10, assign5110_e3403_d_n11, assign5110_e3403_d_n12, assign5110_e3403_d_n17,) = {
    if (var_guard50 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vbsc_dvbse, var_vbsc_dvbse_dn0, var_vbsc_dvbse_dn2, var_vbsc_dvbse_dn6, var_vbsc_dvbse_dn7, var_vbsc_dvbse_dn10, var_vbsc_dvbse_dn11, var_vbsc_dvbse_dn12, var_vbsc_dvbse_dn17,)
    }
};
        var_vbsc_dvbse = assign5110_e3403;
        var_vbsc_dvbse_dn0 = assign5110_e3403_d_n0;
        var_vbsc_dvbse_dn2 = assign5110_e3403_d_n2;
        var_vbsc_dvbse_dn6 = assign5110_e3403_d_n6;
        var_vbsc_dvbse_dn7 = assign5110_e3403_d_n7;
        var_vbsc_dvbse_dn10 = assign5110_e3403_d_n10;
        var_vbsc_dvbse_dn11 = assign5110_e3403_d_n11;
        var_vbsc_dvbse_dn12 = assign5110_e3403_d_n12;
        var_vbsc_dvbse_dn17 = assign5110_e3403_d_n17;

        let (assign5120_e3409, assign5120_e3409_d_n0, assign5120_e3409_d_n2, assign5120_e3409_d_n6, assign5120_e3409_d_n7, assign5120_e3409_d_n10, assign5120_e3409_d_n11, assign5120_e3409_d_n12, assign5120_e3409_d_n17,) = {
    if (var_vds > 20.0) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vds, var_vds_dn0, var_vds_dn2, var_vds_dn6, var_vds_dn7, var_vds_dn10, var_vds_dn11, var_vds_dn12, var_vds_dn17,)
    }
};
        var_vdsc = assign5120_e3409;
        var_vdsc_dn0 = assign5120_e3409_d_n0;
        var_vdsc_dn2 = assign5120_e3409_d_n2;
        var_vdsc_dn6 = assign5120_e3409_d_n6;
        var_vdsc_dn7 = assign5120_e3409_d_n7;
        var_vdsc_dn10 = assign5120_e3409_d_n10;
        var_vdsc_dn11 = assign5120_e3409_d_n11;
        var_vdsc_dn12 = assign5120_e3409_d_n12;
        var_vdsc_dn17 = assign5120_e3409_d_n17;

        let (assign5130_e3415, assign5130_e3415_d_n6, assign5130_e3415_d_n7, assign5130_e3415_d_n11,) = {
    if (var_vgs > 20.0) {
        (20.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vgs, var_vgs_dn6, var_vgs_dn7, var_vgs_dn11,)
    }
};
        var_vgsc = assign5130_e3415;
        var_vgsc_dn6 = assign5130_e3415_d_n6;
        var_vgsc_dn7 = assign5130_e3415_d_n7;
        var_vgsc_dn11 = assign5130_e3415_d_n11;

        let assign5140_e3418: f64 = (-20.0);
        let (assign5140_e3423, assign5140_e3423_d_n6, assign5140_e3423_d_n7, assign5140_e3423_d_n11,) = {
    if (var_vgs < assign5140_e3418) {
        let assign5140_e3421: f64 = (-20.0);
        (assign5140_e3421, 0.0, 0.0, 0.0,)
    } else {
        (var_vgsc, var_vgsc_dn6, var_vgsc_dn7, var_vgsc_dn11,)
    }
};
        var_vgsc = assign5140_e3423;
        var_vgsc_dn6 = assign5140_e3423_d_n6;
        var_vgsc_dn7 = assign5140_e3423_d_n7;
        var_vgsc_dn11 = assign5140_e3423_d_n11;

        let assign5150_e3426: f64 = (-20.0);
        let (assign5150_e3431, assign5150_e3431_d_n0, assign5150_e3431_d_n2, assign5150_e3431_d_n6, assign5150_e3431_d_n7, assign5150_e3431_d_n10, assign5150_e3431_d_n11, assign5150_e3431_d_n12, assign5150_e3431_d_n17,) = {
    if (var_vbsc < assign5150_e3426) {
        let assign5150_e3429: f64 = (-20.0);
        (assign5150_e3429, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vbsc, var_vbsc_dn0, var_vbsc_dn2, var_vbsc_dn6, var_vbsc_dn7, var_vbsc_dn10, var_vbsc_dn11, var_vbsc_dn12, var_vbsc_dn17,)
    }
};
        var_vbsc = assign5150_e3431;
        var_vbsc_dn0 = assign5150_e3431_d_n0;
        var_vbsc_dn2 = assign5150_e3431_d_n2;
        var_vbsc_dn6 = assign5150_e3431_d_n6;
        var_vbsc_dn7 = assign5150_e3431_d_n7;
        var_vbsc_dn10 = assign5150_e3431_d_n10;
        var_vbsc_dn11 = assign5150_e3431_d_n11;
        var_vbsc_dn12 = assign5150_e3431_d_n12;
        var_vbsc_dn17 = assign5150_e3431_d_n17;

        var_vds = var_vdsc;
        var_vds_dn0 = var_vdsc_dn0;
        var_vds_dn2 = var_vdsc_dn2;
        var_vds_dn6 = var_vdsc_dn6;
        var_vds_dn7 = var_vdsc_dn7;
        var_vds_dn10 = var_vdsc_dn10;
        var_vds_dn11 = var_vdsc_dn11;
        var_vds_dn12 = var_vdsc_dn12;
        var_vds_dn17 = var_vdsc_dn17;

        var_vgs = var_vgsc;
        var_vgs_dn6 = var_vgsc_dn6;
        var_vgs_dn7 = var_vgsc_dn7;
        var_vgs_dn11 = var_vgsc_dn11;

        var_vbs = var_vbsc;
        var_vbs_dn0 = var_vbsc_dn0;
        var_vbs_dn2 = var_vbsc_dn2;
        var_vbs_dn6 = var_vbsc_dn6;
        var_vbs_dn7 = var_vbsc_dn7;
        var_vbs_dn10 = var_vbsc_dn10;
        var_vbs_dn11 = var_vbsc_dn11;
        var_vbs_dn12 = var_vbsc_dn12;
        var_vbs_dn17 = var_vbsc_dn17;

        var_flg_pprv = 0.0;

        var_pss0_ini = 0.0;

        var_pbs0_ini = 0.0;

        var_psb0_ini = 0.0;

        var_pssl_ini = 0.0;

        var_pbsl_ini = 0.0;

        var_psbl_ini = 0.0;

        var_ai = 0.0;
        var_ai_dn0 = 0.0;
        var_ai_dn2 = 0.0;
        var_ai_dn6 = 0.0;
        var_ai_dn7 = 0.0;
        var_ai_dn10 = 0.0;
        var_ai_dn11 = 0.0;
        var_ai_dn12 = 0.0;
        var_ai_dn17 = 0.0;

        var_db = 0.0;
        var_db_dn0 = 0.0;
        var_db_dn2 = 0.0;
        var_db_dn6 = 0.0;
        var_db_dn7 = 0.0;
        var_db_dn10 = 0.0;
        var_db_dn11 = 0.0;
        var_db_dn12 = 0.0;
        var_db_dn17 = 0.0;

        var_di = 0.0;
        var_di_dn0 = 0.0;
        var_di_dn2 = 0.0;
        var_di_dn6 = 0.0;
        var_di_dn7 = 0.0;
        var_di_dn10 = 0.0;
        var_di_dn11 = 0.0;
        var_di_dn12 = 0.0;
        var_di_dn17 = 0.0;

        var_c2 = 0.0;
        var_c2_dn0 = 0.0;
        var_c2_dn2 = 0.0;
        var_c2_dn6 = 0.0;
        var_c2_dn7 = 0.0;
        var_c2_dn10 = 0.0;
        var_c2_dn11 = 0.0;
        var_c2_dn12 = 0.0;
        var_c2_dn17 = 0.0;

        var_lp_s0 = 0.0;

        var_lp_sl = 0.0;

        let assign5320_e3450: f64 = (var_vbsc_dvbse * var_vds);
        let assign5320_e3452: f64 = (assign5320_e3450 / 2.0);
        var_t1__blk56 = assign5320_e3452;
        var_t1__blk56_dn0 = (((var_vbsc_dvbse_dn0 * var_vds) + (var_vbsc_dvbse * var_vds_dn0)) / 2.0);
        var_t1__blk56_dn2 = (((var_vbsc_dvbse_dn2 * var_vds) + (var_vbsc_dvbse * var_vds_dn2)) / 2.0);
        var_t1__blk56_dn6 = (((var_vbsc_dvbse_dn6 * var_vds) + (var_vbsc_dvbse * var_vds_dn6)) / 2.0);
        var_t1__blk56_dn7 = (((var_vbsc_dvbse_dn7 * var_vds) + (var_vbsc_dvbse * var_vds_dn7)) / 2.0);
        var_t1__blk56_dn10 = (((var_vbsc_dvbse_dn10 * var_vds) + (var_vbsc_dvbse * var_vds_dn10)) / 2.0);
        var_t1__blk56_dn11 = (((var_vbsc_dvbse_dn11 * var_vds) + (var_vbsc_dvbse * var_vds_dn11)) / 2.0);
        var_t1__blk56_dn12 = (((var_vbsc_dvbse_dn12 * var_vds) + (var_vbsc_dvbse * var_vds_dn12)) / 2.0);
        var_t1__blk56_dn17 = (((var_vbsc_dvbse_dn17 * var_vds) + (var_vbsc_dvbse * var_vds_dn17)) / 2.0);

        let assign5330_e3455: f64 = (2.0 * var_t1__blk56);
        let assign5330_e3457: f64 = (assign5330_e3455 / p.p226);
        var_tmf1 = assign5330_e3457;
        var_tmf1_dn0 = ((2.0 * var_t1__blk56_dn0) / p.p226);
        var_tmf1_dn2 = ((2.0 * var_t1__blk56_dn2) / p.p226);
        var_tmf1_dn6 = ((2.0 * var_t1__blk56_dn6) / p.p226);
        var_tmf1_dn7 = ((2.0 * var_t1__blk56_dn7) / p.p226);
        var_tmf1_dn10 = ((2.0 * var_t1__blk56_dn10) / p.p226);
        var_tmf1_dn11 = ((2.0 * var_t1__blk56_dn11) / p.p226);
        var_tmf1_dn12 = ((2.0 * var_t1__blk56_dn12) / p.p226);
        var_tmf1_dn17 = ((2.0 * var_t1__blk56_dn17) / p.p226);

        let assign5340_e3462: f64 = (1.0 / 2.0);
        let assign5340_e3466: f64 = (1.0 / 6.0);
        let assign5340_e3470: f64 = (1.0 / 24.0);
        let assign5340_e3474: f64 = (1.0 / 120.0);
        let assign5340_e3478: f64 = (1.0 / 720.0);
        let assign5340_e3482: f64 = (1.0 / 5040.0);
        let assign5340_e3483: f64 = (var_tmf1 * assign5340_e3482);
        let assign5340_e3484: f64 = (assign5340_e3478 + assign5340_e3483);
        let assign5340_e3485: f64 = (var_tmf1 * assign5340_e3484);
        let assign5340_e3486: f64 = (assign5340_e3474 + assign5340_e3485);
        let assign5340_e3487: f64 = (var_tmf1 * assign5340_e3486);
        let assign5340_e3488: f64 = (assign5340_e3470 + assign5340_e3487);
        let assign5340_e3489: f64 = (var_tmf1 * assign5340_e3488);
        let assign5340_e3490: f64 = (assign5340_e3466 + assign5340_e3489);
        let assign5340_e3491: f64 = (var_tmf1 * assign5340_e3490);
        let assign5340_e3492: f64 = (assign5340_e3462 + assign5340_e3491);
        let assign5340_e3493: f64 = (var_tmf1 * assign5340_e3492);
        let assign5340_e3494: f64 = (1.0 + assign5340_e3493);
        var_tmf2 = assign5340_e3494;
        var_tmf2_dn0 = ((var_tmf1_dn0 * assign5340_e3492) + (var_tmf1 * ((var_tmf1_dn0 * assign5340_e3490) + (var_tmf1 * ((var_tmf1_dn0 * assign5340_e3488) + (var_tmf1 * ((var_tmf1_dn0 * assign5340_e3486) + (var_tmf1 * ((var_tmf1_dn0 * assign5340_e3484) + (var_tmf1 * (var_tmf1_dn0 * assign5340_e3482)))))))))));
        var_tmf2_dn2 = ((var_tmf1_dn2 * assign5340_e3492) + (var_tmf1 * ((var_tmf1_dn2 * assign5340_e3490) + (var_tmf1 * ((var_tmf1_dn2 * assign5340_e3488) + (var_tmf1 * ((var_tmf1_dn2 * assign5340_e3486) + (var_tmf1 * ((var_tmf1_dn2 * assign5340_e3484) + (var_tmf1 * (var_tmf1_dn2 * assign5340_e3482)))))))))));
        var_tmf2_dn6 = ((var_tmf1_dn6 * assign5340_e3492) + (var_tmf1 * ((var_tmf1_dn6 * assign5340_e3490) + (var_tmf1 * ((var_tmf1_dn6 * assign5340_e3488) + (var_tmf1 * ((var_tmf1_dn6 * assign5340_e3486) + (var_tmf1 * ((var_tmf1_dn6 * assign5340_e3484) + (var_tmf1 * (var_tmf1_dn6 * assign5340_e3482)))))))))));
        var_tmf2_dn7 = ((var_tmf1_dn7 * assign5340_e3492) + (var_tmf1 * ((var_tmf1_dn7 * assign5340_e3490) + (var_tmf1 * ((var_tmf1_dn7 * assign5340_e3488) + (var_tmf1 * ((var_tmf1_dn7 * assign5340_e3486) + (var_tmf1 * ((var_tmf1_dn7 * assign5340_e3484) + (var_tmf1 * (var_tmf1_dn7 * assign5340_e3482)))))))))));
        var_tmf2_dn10 = ((var_tmf1_dn10 * assign5340_e3492) + (var_tmf1 * ((var_tmf1_dn10 * assign5340_e3490) + (var_tmf1 * ((var_tmf1_dn10 * assign5340_e3488) + (var_tmf1 * ((var_tmf1_dn10 * assign5340_e3486) + (var_tmf1 * ((var_tmf1_dn10 * assign5340_e3484) + (var_tmf1 * (var_tmf1_dn10 * assign5340_e3482)))))))))));
        var_tmf2_dn11 = ((var_tmf1_dn11 * assign5340_e3492) + (var_tmf1 * ((var_tmf1_dn11 * assign5340_e3490) + (var_tmf1 * ((var_tmf1_dn11 * assign5340_e3488) + (var_tmf1 * ((var_tmf1_dn11 * assign5340_e3486) + (var_tmf1 * ((var_tmf1_dn11 * assign5340_e3484) + (var_tmf1 * (var_tmf1_dn11 * assign5340_e3482)))))))))));
        var_tmf2_dn12 = ((var_tmf1_dn12 * assign5340_e3492) + (var_tmf1 * ((var_tmf1_dn12 * assign5340_e3490) + (var_tmf1 * ((var_tmf1_dn12 * assign5340_e3488) + (var_tmf1 * ((var_tmf1_dn12 * assign5340_e3486) + (var_tmf1 * ((var_tmf1_dn12 * assign5340_e3484) + (var_tmf1 * (var_tmf1_dn12 * assign5340_e3482)))))))))));
        var_tmf2_dn17 = ((var_tmf1_dn17 * assign5340_e3492) + (var_tmf1 * ((var_tmf1_dn17 * assign5340_e3490) + (var_tmf1 * ((var_tmf1_dn17 * assign5340_e3488) + (var_tmf1 * ((var_tmf1_dn17 * assign5340_e3486) + (var_tmf1 * ((var_tmf1_dn17 * assign5340_e3484) + (var_tmf1 * (var_tmf1_dn17 * assign5340_e3482)))))))))));

        let assign5350_e3497: f64 = (p.p226 / var_tmf2);
        var_vzadd = assign5350_e3497;
        var_vzadd_dn0 = (-((p.p226 * var_tmf2_dn0) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn2 = (-((p.p226 * var_tmf2_dn2) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn6 = (-((p.p226 * var_tmf2_dn6) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn7 = (-((p.p226 * var_tmf2_dn7) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn10 = (-((p.p226 * var_tmf2_dn10) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn11 = (-((p.p226 * var_tmf2_dn11) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn12 = (-((p.p226 * var_tmf2_dn12) / (var_tmf2 * var_tmf2)));
        var_vzadd_dn17 = (-((p.p226 * var_tmf2_dn17) / (var_tmf2 * var_tmf2)));

        let assign5360_e3500: f64 = if var_vzadd < 5e-12 { 1.0 } else { 0.0 };
        var_guard57 = assign5360_e3500;

        let (assign5370_e3504, assign5370_e3504_d_n0, assign5370_e3504_d_n2, assign5370_e3504_d_n6, assign5370_e3504_d_n7, assign5370_e3504_d_n10, assign5370_e3504_d_n11, assign5370_e3504_d_n12, assign5370_e3504_d_n17,) = {
    if (var_guard57 != 0.0) {
        (5e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vzadd, var_vzadd_dn0, var_vzadd_dn2, var_vzadd_dn6, var_vzadd_dn7, var_vzadd_dn10, var_vzadd_dn11, var_vzadd_dn12, var_vzadd_dn17,)
    }
};
        var_vzadd = assign5370_e3504;
        var_vzadd_dn0 = assign5370_e3504_d_n0;
        var_vzadd_dn2 = assign5370_e3504_d_n2;
        var_vzadd_dn6 = assign5370_e3504_d_n6;
        var_vzadd_dn7 = assign5370_e3504_d_n7;
        var_vzadd_dn10 = assign5370_e3504_d_n10;
        var_vzadd_dn11 = assign5370_e3504_d_n11;
        var_vzadd_dn12 = assign5370_e3504_d_n12;
        var_vzadd_dn17 = assign5370_e3504_d_n17;

        let assign5380_e3507: f64 = (var_vbs + var_vzadd);
        var_vbsz = assign5380_e3507;
        var_vbsz_dn0 = (var_vbs_dn0 + var_vzadd_dn0);
        var_vbsz_dn2 = (var_vbs_dn2 + var_vzadd_dn2);
        var_vbsz_dn6 = (var_vbs_dn6 + var_vzadd_dn6);
        var_vbsz_dn7 = (var_vbs_dn7 + var_vzadd_dn7);
        var_vbsz_dn10 = (var_vbs_dn10 + var_vzadd_dn10);
        var_vbsz_dn11 = (var_vbs_dn11 + var_vzadd_dn11);
        var_vbsz_dn12 = (var_vbs_dn12 + var_vzadd_dn12);
        var_vbsz_dn17 = (var_vbs_dn17 + var_vzadd_dn17);

        let assign5390_e3511: f64 = (2.0 * var_vzadd);
        let assign5390_e3512: f64 = (var_vds + assign5390_e3511);
        var_vdsz = assign5390_e3512;
        var_vdsz_dn0 = (var_vds_dn0 + (2.0 * var_vzadd_dn0));
        var_vdsz_dn2 = (var_vds_dn2 + (2.0 * var_vzadd_dn2));
        var_vdsz_dn6 = (var_vds_dn6 + (2.0 * var_vzadd_dn6));
        var_vdsz_dn7 = (var_vds_dn7 + (2.0 * var_vzadd_dn7));
        var_vdsz_dn10 = (var_vds_dn10 + (2.0 * var_vzadd_dn10));
        var_vdsz_dn11 = (var_vds_dn11 + (2.0 * var_vzadd_dn11));
        var_vdsz_dn12 = (var_vds_dn12 + (2.0 * var_vzadd_dn12));
        var_vdsz_dn17 = (var_vds_dn17 + (2.0 * var_vzadd_dn17));

        let assign5400_e3515: f64 = (var_vgs + var_vzadd);
        var_vgsz = assign5400_e3515;
        var_vgsz_dn0 = var_vzadd_dn0;
        var_vgsz_dn2 = var_vzadd_dn2;
        var_vgsz_dn6 = (var_vgs_dn6 + var_vzadd_dn6);
        var_vgsz_dn7 = (var_vgs_dn7 + var_vzadd_dn7);
        var_vgsz_dn10 = var_vzadd_dn10;
        var_vgsz_dn11 = (var_vgs_dn11 + var_vzadd_dn11);
        var_vgsz_dn12 = var_vzadd_dn12;
        var_vgsz_dn17 = var_vzadd_dn17;

        let assign5410_e3518: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard58 = assign5410_e3518;

        let (assign5420_e3522, assign5420_e3522_d_n0, assign5420_e3522_d_n2, assign5420_e3522_d_n6, assign5420_e3522_d_n7, assign5420_e3522_d_n10, assign5420_e3522_d_n11, assign5420_e3522_d_n12, assign5420_e3522_d_n17,) = {
    if (var_guard58 != 0.0) {
        (var_vbs, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
    } else {
        (var_vbsp, var_vbsp_dn0, var_vbsp_dn2, var_vbsp_dn6, var_vbsp_dn7, var_vbsp_dn10, var_vbsp_dn11, var_vbsp_dn12, var_vbsp_dn17,)
    }
};
        var_vbsp = assign5420_e3522;
        var_vbsp_dn0 = assign5420_e3522_d_n0;
        var_vbsp_dn2 = assign5420_e3522_d_n2;
        var_vbsp_dn6 = assign5420_e3522_d_n6;
        var_vbsp_dn7 = assign5420_e3522_d_n7;
        var_vbsp_dn10 = assign5420_e3522_d_n10;
        var_vbsp_dn11 = assign5420_e3522_d_n11;
        var_vbsp_dn12 = assign5420_e3522_d_n12;
        var_vbsp_dn17 = assign5420_e3522_d_n17;

        *var_ai_slot = var_ai;
        *var_ai_dn0_slot = var_ai_dn0;
        *var_ai_dn10_slot = var_ai_dn10;
        *var_ai_dn11_slot = var_ai_dn11;
        *var_ai_dn12_slot = var_ai_dn12;
        *var_ai_dn17_slot = var_ai_dn17;
        *var_ai_dn2_slot = var_ai_dn2;
        *var_ai_dn6_slot = var_ai_dn6;
        *var_ai_dn7_slot = var_ai_dn7;
        *var_c2_slot = var_c2;
        *var_c2_dn0_slot = var_c2_dn0;
        *var_c2_dn10_slot = var_c2_dn10;
        *var_c2_dn11_slot = var_c2_dn11;
        *var_c2_dn12_slot = var_c2_dn12;
        *var_c2_dn17_slot = var_c2_dn17;
        *var_c2_dn2_slot = var_c2_dn2;
        *var_c2_dn6_slot = var_c2_dn6;
        *var_c2_dn7_slot = var_c2_dn7;
        *var_db_slot = var_db;
        *var_db_dn0_slot = var_db_dn0;
        *var_db_dn10_slot = var_db_dn10;
        *var_db_dn11_slot = var_db_dn11;
        *var_db_dn12_slot = var_db_dn12;
        *var_db_dn17_slot = var_db_dn17;
        *var_db_dn2_slot = var_db_dn2;
        *var_db_dn6_slot = var_db_dn6;
        *var_db_dn7_slot = var_db_dn7;
        *var_di_slot = var_di;
        *var_di_dn0_slot = var_di_dn0;
        *var_di_dn10_slot = var_di_dn10;
        *var_di_dn11_slot = var_di_dn11;
        *var_di_dn12_slot = var_di_dn12;
        *var_di_dn17_slot = var_di_dn17;
        *var_di_dn2_slot = var_di_dn2;
        *var_di_dn6_slot = var_di_dn6;
        *var_di_dn7_slot = var_di_dn7;
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn11_slot = var_dnm_dn11;
        *var_dnm_dn12_slot = var_dnm_dn12;
        *var_dnm_dn17_slot = var_dnm_dn17;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_flg_pprv_slot = var_flg_pprv;
        *var_guard54_slot = var_guard54;
        *var_guard55_slot = var_guard55;
        *var_guard57_slot = var_guard57;
        *var_guard58_slot = var_guard58;
        *var_lp_s0_slot = var_lp_s0;
        *var_lp_sl_slot = var_lp_sl;
        *var_m0_slot = var_m0;
        *var_mm_slot = var_mm;
        *var_pbs0_ini_slot = var_pbs0_ini;
        *var_pbsl_ini_slot = var_pbsl_ini;
        *var_psb0_ini_slot = var_psb0_ini;
        *var_psbl_ini_slot = var_psbl_ini;
        *var_pss0_ini_slot = var_pss0_ini;
        *var_pssl_ini_slot = var_pssl_ini;
        *var_t1__blk56_slot = var_t1__blk56;
        *var_t1__blk56_dn0_slot = var_t1__blk56_dn0;
        *var_t1__blk56_dn10_slot = var_t1__blk56_dn10;
        *var_t1__blk56_dn11_slot = var_t1__blk56_dn11;
        *var_t1__blk56_dn12_slot = var_t1__blk56_dn12;
        *var_t1__blk56_dn17_slot = var_t1__blk56_dn17;
        *var_t1__blk56_dn2_slot = var_t1__blk56_dn2;
        *var_t1__blk56_dn6_slot = var_t1__blk56_dn6;
        *var_t1__blk56_dn7_slot = var_t1__blk56_dn7;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn17_slot = var_t4_dn17;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t8_slot = var_t8;
        *var_t8_dn0_slot = var_t8_dn0;
        *var_t8_dn10_slot = var_t8_dn10;
        *var_t8_dn11_slot = var_t8_dn11;
        *var_t8_dn12_slot = var_t8_dn12;
        *var_t8_dn17_slot = var_t8_dn17;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn17_slot = var_tmf2_dn17;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_vbs_slot = var_vbs;
        *var_vbs_dn0_slot = var_vbs_dn0;
        *var_vbs_dn10_slot = var_vbs_dn10;
        *var_vbs_dn11_slot = var_vbs_dn11;
        *var_vbs_dn12_slot = var_vbs_dn12;
        *var_vbs_dn17_slot = var_vbs_dn17;
        *var_vbs_dn2_slot = var_vbs_dn2;
        *var_vbs_dn6_slot = var_vbs_dn6;
        *var_vbs_dn7_slot = var_vbs_dn7;
        *var_vbsc_slot = var_vbsc;
        *var_vbsc_dn0_slot = var_vbsc_dn0;
        *var_vbsc_dn10_slot = var_vbsc_dn10;
        *var_vbsc_dn11_slot = var_vbsc_dn11;
        *var_vbsc_dn12_slot = var_vbsc_dn12;
        *var_vbsc_dn17_slot = var_vbsc_dn17;
        *var_vbsc_dn2_slot = var_vbsc_dn2;
        *var_vbsc_dn6_slot = var_vbsc_dn6;
        *var_vbsc_dn7_slot = var_vbsc_dn7;
        *var_vbsc_dvbse_slot = var_vbsc_dvbse;
        *var_vbsc_dvbse_dn0_slot = var_vbsc_dvbse_dn0;
        *var_vbsc_dvbse_dn10_slot = var_vbsc_dvbse_dn10;
        *var_vbsc_dvbse_dn11_slot = var_vbsc_dvbse_dn11;
        *var_vbsc_dvbse_dn12_slot = var_vbsc_dvbse_dn12;
        *var_vbsc_dvbse_dn17_slot = var_vbsc_dvbse_dn17;
        *var_vbsc_dvbse_dn2_slot = var_vbsc_dvbse_dn2;
        *var_vbsc_dvbse_dn6_slot = var_vbsc_dvbse_dn6;
        *var_vbsc_dvbse_dn7_slot = var_vbsc_dvbse_dn7;
        *var_vbsp_slot = var_vbsp;
        *var_vbsp_dn0_slot = var_vbsp_dn0;
        *var_vbsp_dn10_slot = var_vbsp_dn10;
        *var_vbsp_dn11_slot = var_vbsp_dn11;
        *var_vbsp_dn12_slot = var_vbsp_dn12;
        *var_vbsp_dn17_slot = var_vbsp_dn17;
        *var_vbsp_dn2_slot = var_vbsp_dn2;
        *var_vbsp_dn6_slot = var_vbsp_dn6;
        *var_vbsp_dn7_slot = var_vbsp_dn7;
        *var_vbsz_slot = var_vbsz;
        *var_vbsz_dn0_slot = var_vbsz_dn0;
        *var_vbsz_dn10_slot = var_vbsz_dn10;
        *var_vbsz_dn11_slot = var_vbsz_dn11;
        *var_vbsz_dn12_slot = var_vbsz_dn12;
        *var_vbsz_dn17_slot = var_vbsz_dn17;
        *var_vbsz_dn2_slot = var_vbsz_dn2;
        *var_vbsz_dn6_slot = var_vbsz_dn6;
        *var_vbsz_dn7_slot = var_vbsz_dn7;
        *var_vds_slot = var_vds;
        *var_vds_dn0_slot = var_vds_dn0;
        *var_vds_dn10_slot = var_vds_dn10;
        *var_vds_dn11_slot = var_vds_dn11;
        *var_vds_dn12_slot = var_vds_dn12;
        *var_vds_dn17_slot = var_vds_dn17;
        *var_vds_dn2_slot = var_vds_dn2;
        *var_vds_dn6_slot = var_vds_dn6;
        *var_vds_dn7_slot = var_vds_dn7;
        *var_vdsc_slot = var_vdsc;
        *var_vdsc_dn0_slot = var_vdsc_dn0;
        *var_vdsc_dn10_slot = var_vdsc_dn10;
        *var_vdsc_dn11_slot = var_vdsc_dn11;
        *var_vdsc_dn12_slot = var_vdsc_dn12;
        *var_vdsc_dn17_slot = var_vdsc_dn17;
        *var_vdsc_dn2_slot = var_vdsc_dn2;
        *var_vdsc_dn6_slot = var_vdsc_dn6;
        *var_vdsc_dn7_slot = var_vdsc_dn7;
        *var_vdsz_slot = var_vdsz;
        *var_vdsz_dn0_slot = var_vdsz_dn0;
        *var_vdsz_dn10_slot = var_vdsz_dn10;
        *var_vdsz_dn11_slot = var_vdsz_dn11;
        *var_vdsz_dn12_slot = var_vdsz_dn12;
        *var_vdsz_dn17_slot = var_vdsz_dn17;
        *var_vdsz_dn2_slot = var_vdsz_dn2;
        *var_vdsz_dn6_slot = var_vdsz_dn6;
        *var_vdsz_dn7_slot = var_vdsz_dn7;
        *var_vgs_slot = var_vgs;
        *var_vgs_dn11_slot = var_vgs_dn11;
        *var_vgs_dn6_slot = var_vgs_dn6;
        *var_vgs_dn7_slot = var_vgs_dn7;
        *var_vgsc_slot = var_vgsc;
        *var_vgsc_dn11_slot = var_vgsc_dn11;
        *var_vgsc_dn6_slot = var_vgsc_dn6;
        *var_vgsc_dn7_slot = var_vgsc_dn7;
        *var_vgsz_slot = var_vgsz;
        *var_vgsz_dn0_slot = var_vgsz_dn0;
        *var_vgsz_dn10_slot = var_vgsz_dn10;
        *var_vgsz_dn11_slot = var_vgsz_dn11;
        *var_vgsz_dn12_slot = var_vgsz_dn12;
        *var_vgsz_dn17_slot = var_vgsz_dn17;
        *var_vgsz_dn2_slot = var_vgsz_dn2;
        *var_vgsz_dn6_slot = var_vgsz_dn6;
        *var_vgsz_dn7_slot = var_vgsz_dn7;
        *var_vzadd_slot = var_vzadd;
        *var_vzadd_dn0_slot = var_vzadd_dn0;
        *var_vzadd_dn10_slot = var_vzadd_dn10;
        *var_vzadd_dn11_slot = var_vzadd_dn11;
        *var_vzadd_dn12_slot = var_vzadd_dn12;
        *var_vzadd_dn17_slot = var_vzadd_dn17;
        *var_vzadd_dn2_slot = var_vzadd_dn2;
        *var_vzadd_dn6_slot = var_vzadd_dn6;
        *var_vzadd_dn7_slot = var_vzadd_dn7;
    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        var_beta_inv: f64,
        var_beta_inv_dn10: f64,
        var_c_fox0: f64,
        var_c_fox0_inv: f64,
        var_cnst0soi: f64,
        var_cnst0soi_dn0: f64,
        var_cnst0soi_dn10: f64,
        var_cnst0soi_dn11: f64,
        var_cnst0soi_dn12: f64,
        var_cnst0soi_dn17: f64,
        var_cnst0soi_dn2: f64,
        var_cnst0soi_dn6: f64,
        var_cnst0soi_dn7: f64,
        var_guard58: f64,
        var_pb2: f64,
        var_pb20: f64,
        var_pb20_dn0: f64,
        var_pb20_dn10: f64,
        var_pb20_dn11: f64,
        var_pb20_dn12: f64,
        var_pb20_dn17: f64,
        var_pb20_dn2: f64,
        var_pb20_dn6: f64,
        var_pb20_dn7: f64,
        var_pb2_dn0: f64,
        var_pb2_dn10: f64,
        var_pb2_dn11: f64,
        var_pb2_dn12: f64,
        var_pb2_dn17: f64,
        var_pb2_dn2: f64,
        var_pb2_dn6: f64,
        var_pb2_dn7: f64,
        var_q_nsub: f64,
        var_q_nsub_dn0: f64,
        var_q_nsub_dn10: f64,
        var_q_nsub_dn11: f64,
        var_q_nsub_dn12: f64,
        var_q_nsub_dn17: f64,
        var_q_nsub_dn2: f64,
        var_q_nsub_dn6: f64,
        var_q_nsub_dn7: f64,
        var_subversion: f64,
        var_tfox0: f64,
        var_vbs: f64,
        var_vbs_dn0: f64,
        var_vbs_dn10: f64,
        var_vbs_dn11: f64,
        var_vbs_dn12: f64,
        var_vbs_dn17: f64,
        var_vbs_dn2: f64,
        var_vbs_dn6: f64,
        var_vbs_dn7: f64,
        var_vbsz: f64,
        var_vbsz_dn0: f64,
        var_vbsz_dn10: f64,
        var_vbsz_dn11: f64,
        var_vbsz_dn12: f64,
        var_vbsz_dn17: f64,
        var_vbsz_dn2: f64,
        var_vbsz_dn6: f64,
        var_vbsz_dn7: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn11: f64,
        var_vds_dn12: f64,
        var_vds_dn17: f64,
        var_vds_dn2: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vfb: f64,
        var_vgs: f64,
        var_vgs_dn11: f64,
        var_vgs_dn6: f64,
        var_vgs_dn7: f64,
        var_c_fox_slot: &mut f64,
        var_c_fox_dn0_slot: &mut f64,
        var_c_fox_dn10_slot: &mut f64,
        var_c_fox_dn11_slot: &mut f64,
        var_c_fox_dn12_slot: &mut f64,
        var_c_fox_dn17_slot: &mut f64,
        var_c_fox_dn2_slot: &mut f64,
        var_c_fox_dn6_slot: &mut f64,
        var_c_fox_dn7_slot: &mut f64,
        var_c_fox_inv_slot: &mut f64,
        var_c_fox_inv_dn0_slot: &mut f64,
        var_c_fox_inv_dn10_slot: &mut f64,
        var_c_fox_inv_dn11_slot: &mut f64,
        var_c_fox_inv_dn12_slot: &mut f64,
        var_c_fox_inv_dn17_slot: &mut f64,
        var_c_fox_inv_dn2_slot: &mut f64,
        var_c_fox_inv_dn6_slot: &mut f64,
        var_c_fox_inv_dn7_slot: &mut f64,
        var_cnstc_foxi_slot: &mut f64,
        var_cnstc_foxi_dn0_slot: &mut f64,
        var_cnstc_foxi_dn10_slot: &mut f64,
        var_cnstc_foxi_dn11_slot: &mut f64,
        var_cnstc_foxi_dn12_slot: &mut f64,
        var_cnstc_foxi_dn17_slot: &mut f64,
        var_cnstc_foxi_dn2_slot: &mut f64,
        var_cnstc_foxi_dn6_slot: &mut f64,
        var_cnstc_foxi_dn7_slot: &mut f64,
        var_flg_qme_slot: &mut f64,
        var_fmdvds_slot: &mut f64,
        var_fmdvds_dn0_slot: &mut f64,
        var_fmdvds_dn10_slot: &mut f64,
        var_fmdvds_dn11_slot: &mut f64,
        var_fmdvds_dn12_slot: &mut f64,
        var_fmdvds_dn17_slot: &mut f64,
        var_fmdvds_dn2_slot: &mut f64,
        var_fmdvds_dn6_slot: &mut f64,
        var_fmdvds_dn7_slot: &mut f64,
        var_guard63_slot: &mut f64,
        var_guard70_slot: &mut f64,
        var_guard71_slot: &mut f64,
        var_pslsat_slot: &mut f64,
        var_pslsat_dn0_slot: &mut f64,
        var_pslsat_dn10_slot: &mut f64,
        var_pslsat_dn11_slot: &mut f64,
        var_pslsat_dn12_slot: &mut f64,
        var_pslsat_dn17_slot: &mut f64,
        var_pslsat_dn2_slot: &mut f64,
        var_pslsat_dn6_slot: &mut f64,
        var_pslsat_dn7_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn17_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t1__blk59_slot: &mut f64,
        var_t1__blk59_dn0_slot: &mut f64,
        var_t1__blk59_dn10_slot: &mut f64,
        var_t1__blk59_dn11_slot: &mut f64,
        var_t1__blk59_dn12_slot: &mut f64,
        var_t1__blk59_dn17_slot: &mut f64,
        var_t1__blk59_dn2_slot: &mut f64,
        var_t1__blk59_dn6_slot: &mut f64,
        var_t1__blk59_dn7_slot: &mut f64,
        var_t2__blk60_slot: &mut f64,
        var_t2__blk60_dn11_slot: &mut f64,
        var_t2__blk60_dn6_slot: &mut f64,
        var_t2__blk60_dn7_slot: &mut f64,
        var_t2__blk64_slot: &mut f64,
        var_t2__blk64_dn0_slot: &mut f64,
        var_t2__blk64_dn10_slot: &mut f64,
        var_t2__blk64_dn11_slot: &mut f64,
        var_t2__blk64_dn12_slot: &mut f64,
        var_t2__blk64_dn17_slot: &mut f64,
        var_t2__blk64_dn2_slot: &mut f64,
        var_t2__blk64_dn6_slot: &mut f64,
        var_t2__blk64_dn7_slot: &mut f64,
        var_t3__blk61_slot: &mut f64,
        var_t3__blk61_dn0_slot: &mut f64,
        var_t3__blk61_dn10_slot: &mut f64,
        var_t3__blk61_dn11_slot: &mut f64,
        var_t3__blk61_dn12_slot: &mut f64,
        var_t3__blk61_dn17_slot: &mut f64,
        var_t3__blk61_dn2_slot: &mut f64,
        var_t3__blk61_dn6_slot: &mut f64,
        var_t3__blk61_dn7_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn17_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_tfoxe_slot: &mut f64,
        var_tfoxe_dn0_slot: &mut f64,
        var_tfoxe_dn10_slot: &mut f64,
        var_tfoxe_dn11_slot: &mut f64,
        var_tfoxe_dn12_slot: &mut f64,
        var_tfoxe_dn17_slot: &mut f64,
        var_tfoxe_dn2_slot: &mut f64,
        var_tfoxe_dn6_slot: &mut f64,
        var_tfoxe_dn7_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn17_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf3_slot: &mut f64,
        var_tmf3_dn0_slot: &mut f64,
        var_tmf3_dn10_slot: &mut f64,
        var_tmf3_dn11_slot: &mut f64,
        var_tmf3_dn12_slot: &mut f64,
        var_tmf3_dn17_slot: &mut f64,
        var_tmf3_dn2_slot: &mut f64,
        var_tmf3_dn6_slot: &mut f64,
        var_tmf3_dn7_slot: &mut f64,
        var_tmf4_slot: &mut f64,
        var_tmf4_dn0_slot: &mut f64,
        var_tmf4_dn10_slot: &mut f64,
        var_tmf4_dn11_slot: &mut f64,
        var_tmf4_dn12_slot: &mut f64,
        var_tmf4_dn17_slot: &mut f64,
        var_tmf4_dn2_slot: &mut f64,
        var_tmf4_dn6_slot: &mut f64,
        var_tmf4_dn7_slot: &mut f64,
        var_tx__blk62_slot: &mut f64,
        var_tx__blk62_dn0_slot: &mut f64,
        var_tx__blk62_dn10_slot: &mut f64,
        var_tx__blk62_dn11_slot: &mut f64,
        var_tx__blk62_dn12_slot: &mut f64,
        var_tx__blk62_dn17_slot: &mut f64,
        var_tx__blk62_dn2_slot: &mut f64,
        var_tx__blk62_dn6_slot: &mut f64,
        var_tx__blk62_dn7_slot: &mut f64,
        var_vbsp_slot: &mut f64,
        var_vbsp_dn0_slot: &mut f64,
        var_vbsp_dn10_slot: &mut f64,
        var_vbsp_dn11_slot: &mut f64,
        var_vbsp_dn12_slot: &mut f64,
        var_vbsp_dn17_slot: &mut f64,
        var_vbsp_dn2_slot: &mut f64,
        var_vbsp_dn6_slot: &mut f64,
        var_vbsp_dn7_slot: &mut f64,
        var_vbspz_slot: &mut f64,
        var_vbspz_dn0_slot: &mut f64,
        var_vbspz_dn10_slot: &mut f64,
        var_vbspz_dn11_slot: &mut f64,
        var_vbspz_dn12_slot: &mut f64,
        var_vbspz_dn17_slot: &mut f64,
        var_vbspz_dn2_slot: &mut f64,
        var_vbspz_dn6_slot: &mut f64,
        var_vbspz_dn7_slot: &mut f64,
        var_vdsats_slot: &mut f64,
        var_vdsats_dn0_slot: &mut f64,
        var_vdsats_dn10_slot: &mut f64,
        var_vdsats_dn11_slot: &mut f64,
        var_vdsats_dn12_slot: &mut f64,
        var_vdsats_dn17_slot: &mut f64,
        var_vdsats_dn2_slot: &mut f64,
        var_vdsats_dn6_slot: &mut f64,
        var_vdsats_dn7_slot: &mut f64,
        var_vthq_slot: &mut f64,
        var_vthq_dn0_slot: &mut f64,
        var_vthq_dn10_slot: &mut f64,
        var_vthq_dn11_slot: &mut f64,
        var_vthq_dn12_slot: &mut f64,
        var_vthq_dn17_slot: &mut f64,
        var_vthq_dn2_slot: &mut f64,
        var_vthq_dn6_slot: &mut f64,
        var_vthq_dn7_slot: &mut f64,
    ) {
        let mut var_c_fox: f64 = *var_c_fox_slot;
        let mut var_c_fox_dn0: f64 = *var_c_fox_dn0_slot;
        let mut var_c_fox_dn10: f64 = *var_c_fox_dn10_slot;
        let mut var_c_fox_dn11: f64 = *var_c_fox_dn11_slot;
        let mut var_c_fox_dn12: f64 = *var_c_fox_dn12_slot;
        let mut var_c_fox_dn17: f64 = *var_c_fox_dn17_slot;
        let mut var_c_fox_dn2: f64 = *var_c_fox_dn2_slot;
        let mut var_c_fox_dn6: f64 = *var_c_fox_dn6_slot;
        let mut var_c_fox_dn7: f64 = *var_c_fox_dn7_slot;
        let mut var_c_fox_inv: f64 = *var_c_fox_inv_slot;
        let mut var_c_fox_inv_dn0: f64 = *var_c_fox_inv_dn0_slot;
        let mut var_c_fox_inv_dn10: f64 = *var_c_fox_inv_dn10_slot;
        let mut var_c_fox_inv_dn11: f64 = *var_c_fox_inv_dn11_slot;
        let mut var_c_fox_inv_dn12: f64 = *var_c_fox_inv_dn12_slot;
        let mut var_c_fox_inv_dn17: f64 = *var_c_fox_inv_dn17_slot;
        let mut var_c_fox_inv_dn2: f64 = *var_c_fox_inv_dn2_slot;
        let mut var_c_fox_inv_dn6: f64 = *var_c_fox_inv_dn6_slot;
        let mut var_c_fox_inv_dn7: f64 = *var_c_fox_inv_dn7_slot;
        let mut var_cnstc_foxi: f64 = *var_cnstc_foxi_slot;
        let mut var_cnstc_foxi_dn0: f64 = *var_cnstc_foxi_dn0_slot;
        let mut var_cnstc_foxi_dn10: f64 = *var_cnstc_foxi_dn10_slot;
        let mut var_cnstc_foxi_dn11: f64 = *var_cnstc_foxi_dn11_slot;
        let mut var_cnstc_foxi_dn12: f64 = *var_cnstc_foxi_dn12_slot;
        let mut var_cnstc_foxi_dn17: f64 = *var_cnstc_foxi_dn17_slot;
        let mut var_cnstc_foxi_dn2: f64 = *var_cnstc_foxi_dn2_slot;
        let mut var_cnstc_foxi_dn6: f64 = *var_cnstc_foxi_dn6_slot;
        let mut var_cnstc_foxi_dn7: f64 = *var_cnstc_foxi_dn7_slot;
        let mut var_flg_qme: f64 = *var_flg_qme_slot;
        let mut var_fmdvds: f64 = *var_fmdvds_slot;
        let mut var_fmdvds_dn0: f64 = *var_fmdvds_dn0_slot;
        let mut var_fmdvds_dn10: f64 = *var_fmdvds_dn10_slot;
        let mut var_fmdvds_dn11: f64 = *var_fmdvds_dn11_slot;
        let mut var_fmdvds_dn12: f64 = *var_fmdvds_dn12_slot;
        let mut var_fmdvds_dn17: f64 = *var_fmdvds_dn17_slot;
        let mut var_fmdvds_dn2: f64 = *var_fmdvds_dn2_slot;
        let mut var_fmdvds_dn6: f64 = *var_fmdvds_dn6_slot;
        let mut var_fmdvds_dn7: f64 = *var_fmdvds_dn7_slot;
        let mut var_guard63: f64 = *var_guard63_slot;
        let mut var_guard70: f64 = *var_guard70_slot;
        let mut var_guard71: f64 = *var_guard71_slot;
        let mut var_pslsat: f64 = *var_pslsat_slot;
        let mut var_pslsat_dn0: f64 = *var_pslsat_dn0_slot;
        let mut var_pslsat_dn10: f64 = *var_pslsat_dn10_slot;
        let mut var_pslsat_dn11: f64 = *var_pslsat_dn11_slot;
        let mut var_pslsat_dn12: f64 = *var_pslsat_dn12_slot;
        let mut var_pslsat_dn17: f64 = *var_pslsat_dn17_slot;
        let mut var_pslsat_dn2: f64 = *var_pslsat_dn2_slot;
        let mut var_pslsat_dn6: f64 = *var_pslsat_dn6_slot;
        let mut var_pslsat_dn7: f64 = *var_pslsat_dn7_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn17: f64 = *var_t0_dn17_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t1__blk59: f64 = *var_t1__blk59_slot;
        let mut var_t1__blk59_dn0: f64 = *var_t1__blk59_dn0_slot;
        let mut var_t1__blk59_dn10: f64 = *var_t1__blk59_dn10_slot;
        let mut var_t1__blk59_dn11: f64 = *var_t1__blk59_dn11_slot;
        let mut var_t1__blk59_dn12: f64 = *var_t1__blk59_dn12_slot;
        let mut var_t1__blk59_dn17: f64 = *var_t1__blk59_dn17_slot;
        let mut var_t1__blk59_dn2: f64 = *var_t1__blk59_dn2_slot;
        let mut var_t1__blk59_dn6: f64 = *var_t1__blk59_dn6_slot;
        let mut var_t1__blk59_dn7: f64 = *var_t1__blk59_dn7_slot;
        let mut var_t2__blk60: f64 = *var_t2__blk60_slot;
        let mut var_t2__blk60_dn11: f64 = *var_t2__blk60_dn11_slot;
        let mut var_t2__blk60_dn6: f64 = *var_t2__blk60_dn6_slot;
        let mut var_t2__blk60_dn7: f64 = *var_t2__blk60_dn7_slot;
        let mut var_t2__blk64: f64 = *var_t2__blk64_slot;
        let mut var_t2__blk64_dn0: f64 = *var_t2__blk64_dn0_slot;
        let mut var_t2__blk64_dn10: f64 = *var_t2__blk64_dn10_slot;
        let mut var_t2__blk64_dn11: f64 = *var_t2__blk64_dn11_slot;
        let mut var_t2__blk64_dn12: f64 = *var_t2__blk64_dn12_slot;
        let mut var_t2__blk64_dn17: f64 = *var_t2__blk64_dn17_slot;
        let mut var_t2__blk64_dn2: f64 = *var_t2__blk64_dn2_slot;
        let mut var_t2__blk64_dn6: f64 = *var_t2__blk64_dn6_slot;
        let mut var_t2__blk64_dn7: f64 = *var_t2__blk64_dn7_slot;
        let mut var_t3__blk61: f64 = *var_t3__blk61_slot;
        let mut var_t3__blk61_dn0: f64 = *var_t3__blk61_dn0_slot;
        let mut var_t3__blk61_dn10: f64 = *var_t3__blk61_dn10_slot;
        let mut var_t3__blk61_dn11: f64 = *var_t3__blk61_dn11_slot;
        let mut var_t3__blk61_dn12: f64 = *var_t3__blk61_dn12_slot;
        let mut var_t3__blk61_dn17: f64 = *var_t3__blk61_dn17_slot;
        let mut var_t3__blk61_dn2: f64 = *var_t3__blk61_dn2_slot;
        let mut var_t3__blk61_dn6: f64 = *var_t3__blk61_dn6_slot;
        let mut var_t3__blk61_dn7: f64 = *var_t3__blk61_dn7_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn17: f64 = *var_t4_dn17_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_tfoxe: f64 = *var_tfoxe_slot;
        let mut var_tfoxe_dn0: f64 = *var_tfoxe_dn0_slot;
        let mut var_tfoxe_dn10: f64 = *var_tfoxe_dn10_slot;
        let mut var_tfoxe_dn11: f64 = *var_tfoxe_dn11_slot;
        let mut var_tfoxe_dn12: f64 = *var_tfoxe_dn12_slot;
        let mut var_tfoxe_dn17: f64 = *var_tfoxe_dn17_slot;
        let mut var_tfoxe_dn2: f64 = *var_tfoxe_dn2_slot;
        let mut var_tfoxe_dn6: f64 = *var_tfoxe_dn6_slot;
        let mut var_tfoxe_dn7: f64 = *var_tfoxe_dn7_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn17: f64 = *var_tmf2_dn17_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf3: f64 = *var_tmf3_slot;
        let mut var_tmf3_dn0: f64 = *var_tmf3_dn0_slot;
        let mut var_tmf3_dn10: f64 = *var_tmf3_dn10_slot;
        let mut var_tmf3_dn11: f64 = *var_tmf3_dn11_slot;
        let mut var_tmf3_dn12: f64 = *var_tmf3_dn12_slot;
        let mut var_tmf3_dn17: f64 = *var_tmf3_dn17_slot;
        let mut var_tmf3_dn2: f64 = *var_tmf3_dn2_slot;
        let mut var_tmf3_dn6: f64 = *var_tmf3_dn6_slot;
        let mut var_tmf3_dn7: f64 = *var_tmf3_dn7_slot;
        let mut var_tmf4: f64 = *var_tmf4_slot;
        let mut var_tmf4_dn0: f64 = *var_tmf4_dn0_slot;
        let mut var_tmf4_dn10: f64 = *var_tmf4_dn10_slot;
        let mut var_tmf4_dn11: f64 = *var_tmf4_dn11_slot;
        let mut var_tmf4_dn12: f64 = *var_tmf4_dn12_slot;
        let mut var_tmf4_dn17: f64 = *var_tmf4_dn17_slot;
        let mut var_tmf4_dn2: f64 = *var_tmf4_dn2_slot;
        let mut var_tmf4_dn6: f64 = *var_tmf4_dn6_slot;
        let mut var_tmf4_dn7: f64 = *var_tmf4_dn7_slot;
        let mut var_tx__blk62: f64 = *var_tx__blk62_slot;
        let mut var_tx__blk62_dn0: f64 = *var_tx__blk62_dn0_slot;
        let mut var_tx__blk62_dn10: f64 = *var_tx__blk62_dn10_slot;
        let mut var_tx__blk62_dn11: f64 = *var_tx__blk62_dn11_slot;
        let mut var_tx__blk62_dn12: f64 = *var_tx__blk62_dn12_slot;
        let mut var_tx__blk62_dn17: f64 = *var_tx__blk62_dn17_slot;
        let mut var_tx__blk62_dn2: f64 = *var_tx__blk62_dn2_slot;
        let mut var_tx__blk62_dn6: f64 = *var_tx__blk62_dn6_slot;
        let mut var_tx__blk62_dn7: f64 = *var_tx__blk62_dn7_slot;
        let mut var_vbsp: f64 = *var_vbsp_slot;
        let mut var_vbsp_dn0: f64 = *var_vbsp_dn0_slot;
        let mut var_vbsp_dn10: f64 = *var_vbsp_dn10_slot;
        let mut var_vbsp_dn11: f64 = *var_vbsp_dn11_slot;
        let mut var_vbsp_dn12: f64 = *var_vbsp_dn12_slot;
        let mut var_vbsp_dn17: f64 = *var_vbsp_dn17_slot;
        let mut var_vbsp_dn2: f64 = *var_vbsp_dn2_slot;
        let mut var_vbsp_dn6: f64 = *var_vbsp_dn6_slot;
        let mut var_vbsp_dn7: f64 = *var_vbsp_dn7_slot;
        let mut var_vbspz: f64 = *var_vbspz_slot;
        let mut var_vbspz_dn0: f64 = *var_vbspz_dn0_slot;
        let mut var_vbspz_dn10: f64 = *var_vbspz_dn10_slot;
        let mut var_vbspz_dn11: f64 = *var_vbspz_dn11_slot;
        let mut var_vbspz_dn12: f64 = *var_vbspz_dn12_slot;
        let mut var_vbspz_dn17: f64 = *var_vbspz_dn17_slot;
        let mut var_vbspz_dn2: f64 = *var_vbspz_dn2_slot;
        let mut var_vbspz_dn6: f64 = *var_vbspz_dn6_slot;
        let mut var_vbspz_dn7: f64 = *var_vbspz_dn7_slot;
        let mut var_vdsats: f64 = *var_vdsats_slot;
        let mut var_vdsats_dn0: f64 = *var_vdsats_dn0_slot;
        let mut var_vdsats_dn10: f64 = *var_vdsats_dn10_slot;
        let mut var_vdsats_dn11: f64 = *var_vdsats_dn11_slot;
        let mut var_vdsats_dn12: f64 = *var_vdsats_dn12_slot;
        let mut var_vdsats_dn17: f64 = *var_vdsats_dn17_slot;
        let mut var_vdsats_dn2: f64 = *var_vdsats_dn2_slot;
        let mut var_vdsats_dn6: f64 = *var_vdsats_dn6_slot;
        let mut var_vdsats_dn7: f64 = *var_vdsats_dn7_slot;
        let mut var_vthq: f64 = *var_vthq_slot;
        let mut var_vthq_dn0: f64 = *var_vthq_dn0_slot;
        let mut var_vthq_dn10: f64 = *var_vthq_dn10_slot;
        let mut var_vthq_dn11: f64 = *var_vthq_dn11_slot;
        let mut var_vthq_dn12: f64 = *var_vthq_dn12_slot;
        let mut var_vthq_dn17: f64 = *var_vthq_dn17_slot;
        let mut var_vthq_dn2: f64 = *var_vthq_dn2_slot;
        let mut var_vthq_dn6: f64 = *var_vthq_dn6_slot;
        let mut var_vthq_dn7: f64 = *var_vthq_dn7_slot;

        let (assign5430_e3526, assign5430_e3526_d_n0, assign5430_e3526_d_n2, assign5430_e3526_d_n6, assign5430_e3526_d_n7, assign5430_e3526_d_n10, assign5430_e3526_d_n11, assign5430_e3526_d_n12, assign5430_e3526_d_n17,) = {
    if (var_guard58 != 0.0) {
        (var_vbsz, var_vbsz_dn0, var_vbsz_dn2, var_vbsz_dn6, var_vbsz_dn7, var_vbsz_dn10, var_vbsz_dn11, var_vbsz_dn12, var_vbsz_dn17,)
    } else {
        (var_vbspz, var_vbspz_dn0, var_vbspz_dn2, var_vbspz_dn6, var_vbspz_dn7, var_vbspz_dn10, var_vbspz_dn11, var_vbspz_dn12, var_vbspz_dn17,)
    }
};
        var_vbspz = assign5430_e3526;
        var_vbspz_dn0 = assign5430_e3526_d_n0;
        var_vbspz_dn2 = assign5430_e3526_d_n2;
        var_vbspz_dn6 = assign5430_e3526_d_n6;
        var_vbspz_dn7 = assign5430_e3526_d_n7;
        var_vbspz_dn10 = assign5430_e3526_d_n10;
        var_vbspz_dn11 = assign5430_e3526_d_n11;
        var_vbspz_dn12 = assign5430_e3526_d_n12;
        var_vbspz_dn17 = assign5430_e3526_d_n17;

        let (assign5440_e3536, assign5440_e3536_d_n0, assign5440_e3536_d_n2, assign5440_e3536_d_n6, assign5440_e3536_d_n7, assign5440_e3536_d_n10, assign5440_e3536_d_n11, assign5440_e3536_d_n12, assign5440_e3536_d_n17,) = {
    if (var_guard58 == 0.0) {
        let (assign5440_e3534, assign5440_e3534_d_n0, assign5440_e3534_d_n2, assign5440_e3534_d_n6, assign5440_e3534_d_n7, assign5440_e3534_d_n10, assign5440_e3534_d_n11, assign5440_e3534_d_n12, assign5440_e3534_d_n17,) = {
            if (var_subversion < 3.0) {
                (var_vbs, var_vbs_dn0, var_vbs_dn2, var_vbs_dn6, var_vbs_dn7, var_vbs_dn10, var_vbs_dn11, var_vbs_dn12, var_vbs_dn17,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign5440_e3534, assign5440_e3534_d_n0, assign5440_e3534_d_n2, assign5440_e3534_d_n6, assign5440_e3534_d_n7, assign5440_e3534_d_n10, assign5440_e3534_d_n11, assign5440_e3534_d_n12, assign5440_e3534_d_n17,)
    } else {
        (var_vbsp, var_vbsp_dn0, var_vbsp_dn2, var_vbsp_dn6, var_vbsp_dn7, var_vbsp_dn10, var_vbsp_dn11, var_vbsp_dn12, var_vbsp_dn17,)
    }
};
        var_vbsp = assign5440_e3536;
        var_vbsp_dn0 = assign5440_e3536_d_n0;
        var_vbsp_dn2 = assign5440_e3536_d_n2;
        var_vbsp_dn6 = assign5440_e3536_d_n6;
        var_vbsp_dn7 = assign5440_e3536_d_n7;
        var_vbsp_dn10 = assign5440_e3536_d_n10;
        var_vbsp_dn11 = assign5440_e3536_d_n11;
        var_vbsp_dn12 = assign5440_e3536_d_n12;
        var_vbsp_dn17 = assign5440_e3536_d_n17;

        let (assign5450_e3546, assign5450_e3546_d_n0, assign5450_e3546_d_n2, assign5450_e3546_d_n6, assign5450_e3546_d_n7, assign5450_e3546_d_n10, assign5450_e3546_d_n11, assign5450_e3546_d_n12, assign5450_e3546_d_n17,) = {
    if (var_guard58 == 0.0) {
        let (assign5450_e3544, assign5450_e3544_d_n0, assign5450_e3544_d_n2, assign5450_e3544_d_n6, assign5450_e3544_d_n7, assign5450_e3544_d_n10, assign5450_e3544_d_n11, assign5450_e3544_d_n12, assign5450_e3544_d_n17,) = {
            if (var_subversion < 3.0) {
                (var_vbsz, var_vbsz_dn0, var_vbsz_dn2, var_vbsz_dn6, var_vbsz_dn7, var_vbsz_dn10, var_vbsz_dn11, var_vbsz_dn12, var_vbsz_dn17,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign5450_e3544, assign5450_e3544_d_n0, assign5450_e3544_d_n2, assign5450_e3544_d_n6, assign5450_e3544_d_n7, assign5450_e3544_d_n10, assign5450_e3544_d_n11, assign5450_e3544_d_n12, assign5450_e3544_d_n17,)
    } else {
        (var_vbspz, var_vbspz_dn0, var_vbspz_dn2, var_vbspz_dn6, var_vbspz_dn7, var_vbspz_dn10, var_vbspz_dn11, var_vbspz_dn12, var_vbspz_dn17,)
    }
};
        var_vbspz = assign5450_e3546;
        var_vbspz_dn0 = assign5450_e3546_d_n0;
        var_vbspz_dn2 = assign5450_e3546_d_n2;
        var_vbspz_dn6 = assign5450_e3546_d_n6;
        var_vbspz_dn7 = assign5450_e3546_d_n7;
        var_vbspz_dn10 = assign5450_e3546_d_n10;
        var_vbspz_dn11 = assign5450_e3546_d_n11;
        var_vbspz_dn12 = assign5450_e3546_d_n12;
        var_vbspz_dn17 = assign5450_e3546_d_n17;

        let assign5460_e3549: f64 = (2.0 * var_q_nsub);
        let assign5460_e3551: f64 = (assign5460_e3549 * 1.034943e-10);
        let assign5460_e3553: f64 = (assign5460_e3551 * var_c_fox0_inv);
        let assign5460_e3555: f64 = (assign5460_e3553 * var_c_fox0_inv);
        var_t1__blk59 = assign5460_e3555;
        var_t1__blk59_dn0 = ((((2.0 * var_q_nsub_dn0) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk59_dn2 = ((((2.0 * var_q_nsub_dn2) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk59_dn6 = ((((2.0 * var_q_nsub_dn6) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk59_dn7 = ((((2.0 * var_q_nsub_dn7) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk59_dn10 = ((((2.0 * var_q_nsub_dn10) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk59_dn11 = ((((2.0 * var_q_nsub_dn11) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk59_dn12 = ((((2.0 * var_q_nsub_dn12) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);
        var_t1__blk59_dn17 = ((((2.0 * var_q_nsub_dn17) * 1.034943e-10) * var_c_fox0_inv) * var_c_fox0_inv);

        let assign5470_e3558: f64 = (var_vgs - var_vfb);
        var_t2__blk60 = assign5470_e3558;
        var_t2__blk60_dn6 = var_vgs_dn6;
        var_t2__blk60_dn7 = var_vgs_dn7;
        var_t2__blk60_dn11 = var_vgs_dn11;

        let assign5480_e3562: f64 = (2.0 / var_t1__blk59);
        let assign5480_e3565: f64 = (var_t2__blk60 - var_beta_inv);
        let assign5480_e3567: f64 = (assign5480_e3565 - var_vbsp);
        let assign5480_e3568: f64 = (assign5480_e3562 * assign5480_e3567);
        let assign5480_e3569: f64 = (1.0 + assign5480_e3568);
        var_t3__blk61 = assign5480_e3569;
        var_t3__blk61_dn0 = (((-((2.0 * var_t1__blk59_dn0) / (var_t1__blk59 * var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * (-var_vbsp_dn0)));
        var_t3__blk61_dn2 = (((-((2.0 * var_t1__blk59_dn2) / (var_t1__blk59 * var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * (-var_vbsp_dn2)));
        var_t3__blk61_dn6 = (((-((2.0 * var_t1__blk59_dn6) / (var_t1__blk59 * var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * (var_t2__blk60_dn6 - var_vbsp_dn6)));
        var_t3__blk61_dn7 = (((-((2.0 * var_t1__blk59_dn7) / (var_t1__blk59 * var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * (var_t2__blk60_dn7 - var_vbsp_dn7)));
        var_t3__blk61_dn10 = (((-((2.0 * var_t1__blk59_dn10) / (var_t1__blk59 * var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * ((-var_beta_inv_dn10) - var_vbsp_dn10)));
        var_t3__blk61_dn11 = (((-((2.0 * var_t1__blk59_dn11) / (var_t1__blk59 * var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * (var_t2__blk60_dn11 - var_vbsp_dn11)));
        var_t3__blk61_dn12 = (((-((2.0 * var_t1__blk59_dn12) / (var_t1__blk59 * var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * (-var_vbsp_dn12)));
        var_t3__blk61_dn17 = (((-((2.0 * var_t1__blk59_dn17) / (var_t1__blk59 * var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * (-var_vbsp_dn17)));

        let assign5490_e3572: f64 = (var_t3__blk61 * var_t3__blk61);
        let assign5490_e3575: f64 = (4.0 * 0.001);
        let assign5490_e3577: f64 = (assign5490_e3575 * 0.001);
        let assign5490_e3578: f64 = (assign5490_e3572 + assign5490_e3577);
        let assign5490_e3579: f64 = (assign5490_e3578).sqrt();
        var_tmf1 = assign5490_e3579;
        var_tmf1_dn0 = (((var_t3__blk61_dn0 * var_t3__blk61) + (var_t3__blk61 * var_t3__blk61_dn0)) / (2.0 * assign5490_e3579));
        var_tmf1_dn2 = (((var_t3__blk61_dn2 * var_t3__blk61) + (var_t3__blk61 * var_t3__blk61_dn2)) / (2.0 * assign5490_e3579));
        var_tmf1_dn6 = (((var_t3__blk61_dn6 * var_t3__blk61) + (var_t3__blk61 * var_t3__blk61_dn6)) / (2.0 * assign5490_e3579));
        var_tmf1_dn7 = (((var_t3__blk61_dn7 * var_t3__blk61) + (var_t3__blk61 * var_t3__blk61_dn7)) / (2.0 * assign5490_e3579));
        var_tmf1_dn10 = (((var_t3__blk61_dn10 * var_t3__blk61) + (var_t3__blk61 * var_t3__blk61_dn10)) / (2.0 * assign5490_e3579));
        var_tmf1_dn11 = (((var_t3__blk61_dn11 * var_t3__blk61) + (var_t3__blk61 * var_t3__blk61_dn11)) / (2.0 * assign5490_e3579));
        var_tmf1_dn12 = (((var_t3__blk61_dn12 * var_t3__blk61) + (var_t3__blk61 * var_t3__blk61_dn12)) / (2.0 * assign5490_e3579));
        var_tmf1_dn17 = (((var_t3__blk61_dn17 * var_t3__blk61) + (var_t3__blk61 * var_t3__blk61_dn17)) / (2.0 * assign5490_e3579));

        let assign5500_e3583: f64 = (var_t3__blk61 + var_tmf1);
        let assign5500_e3584: f64 = (0.5 * assign5500_e3583);
        let assign5500_e3587: f64 = (1e-10 * 0.001);
        let assign5500_e3588: f64 = (assign5500_e3584 + assign5500_e3587);
        var_t4 = assign5500_e3588;
        var_t4_dn0 = (0.5 * (var_t3__blk61_dn0 + var_tmf1_dn0));
        var_t4_dn2 = (0.5 * (var_t3__blk61_dn2 + var_tmf1_dn2));
        var_t4_dn6 = (0.5 * (var_t3__blk61_dn6 + var_tmf1_dn6));
        var_t4_dn7 = (0.5 * (var_t3__blk61_dn7 + var_tmf1_dn7));
        var_t4_dn10 = (0.5 * (var_t3__blk61_dn10 + var_tmf1_dn10));
        var_t4_dn11 = (0.5 * (var_t3__blk61_dn11 + var_tmf1_dn11));
        var_t4_dn12 = (0.5 * (var_t3__blk61_dn12 + var_tmf1_dn12));
        var_t4_dn17 = (0.5 * (var_t3__blk61_dn17 + var_tmf1_dn17));

        let assign5510_e3591: f64 = if var_t4 < 0.0 { 1.0 } else { 0.0 };
        var_guard63 = assign5510_e3591;

        let (assign5520_e3595, assign5520_e3595_d_n0, assign5520_e3595_d_n2, assign5520_e3595_d_n6, assign5520_e3595_d_n7, assign5520_e3595_d_n10, assign5520_e3595_d_n11, assign5520_e3595_d_n12, assign5520_e3595_d_n17,) = {
    if (var_guard63 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    }
};
        var_t4 = assign5520_e3595;
        var_t4_dn0 = assign5520_e3595_d_n0;
        var_t4_dn2 = assign5520_e3595_d_n2;
        var_t4_dn6 = assign5520_e3595_d_n6;
        var_t4_dn7 = assign5520_e3595_d_n7;
        var_t4_dn10 = assign5520_e3595_d_n10;
        var_t4_dn11 = assign5520_e3595_d_n11;
        var_t4_dn12 = assign5520_e3595_d_n12;
        var_t4_dn17 = assign5520_e3595_d_n17;

        let assign5530_e3598: f64 = (var_t4 + 1e-50);
        let assign5530_e3599: f64 = (assign5530_e3598).sqrt();
        var_tx__blk62 = assign5530_e3599;
        var_tx__blk62_dn0 = (var_t4_dn0 / (2.0 * assign5530_e3599));
        var_tx__blk62_dn2 = (var_t4_dn2 / (2.0 * assign5530_e3599));
        var_tx__blk62_dn6 = (var_t4_dn6 / (2.0 * assign5530_e3599));
        var_tx__blk62_dn7 = (var_t4_dn7 / (2.0 * assign5530_e3599));
        var_tx__blk62_dn10 = (var_t4_dn10 / (2.0 * assign5530_e3599));
        var_tx__blk62_dn11 = (var_t4_dn11 / (2.0 * assign5530_e3599));
        var_tx__blk62_dn12 = (var_t4_dn12 / (2.0 * assign5530_e3599));
        var_tx__blk62_dn17 = (var_t4_dn17 / (2.0 * assign5530_e3599));

        let assign5540_e3604: f64 = (1.0 - var_tx__blk62);
        let assign5540_e3605: f64 = (var_t1__blk59 * assign5540_e3604);
        let assign5540_e3606: f64 = (var_t2__blk60 + assign5540_e3605);
        var_pslsat = assign5540_e3606;
        var_pslsat_dn0 = ((var_t1__blk59_dn0 * assign5540_e3604) + (var_t1__blk59 * (-var_tx__blk62_dn0)));
        var_pslsat_dn2 = ((var_t1__blk59_dn2 * assign5540_e3604) + (var_t1__blk59 * (-var_tx__blk62_dn2)));
        var_pslsat_dn6 = (var_t2__blk60_dn6 + ((var_t1__blk59_dn6 * assign5540_e3604) + (var_t1__blk59 * (-var_tx__blk62_dn6))));
        var_pslsat_dn7 = (var_t2__blk60_dn7 + ((var_t1__blk59_dn7 * assign5540_e3604) + (var_t1__blk59 * (-var_tx__blk62_dn7))));
        var_pslsat_dn10 = ((var_t1__blk59_dn10 * assign5540_e3604) + (var_t1__blk59 * (-var_tx__blk62_dn10)));
        var_pslsat_dn11 = (var_t2__blk60_dn11 + ((var_t1__blk59_dn11 * assign5540_e3604) + (var_t1__blk59 * (-var_tx__blk62_dn11))));
        var_pslsat_dn12 = ((var_t1__blk59_dn12 * assign5540_e3604) + (var_t1__blk59 * (-var_tx__blk62_dn12)));
        var_pslsat_dn17 = ((var_t1__blk59_dn17 * assign5540_e3604) + (var_t1__blk59 * (-var_tx__blk62_dn17)));

        let assign5550_e3609: f64 = (var_pslsat - var_pb2);
        var_vdsats = assign5550_e3609;
        var_vdsats_dn0 = (var_pslsat_dn0 - var_pb2_dn0);
        var_vdsats_dn2 = (var_pslsat_dn2 - var_pb2_dn2);
        var_vdsats_dn6 = (var_pslsat_dn6 - var_pb2_dn6);
        var_vdsats_dn7 = (var_pslsat_dn7 - var_pb2_dn7);
        var_vdsats_dn10 = (var_pslsat_dn10 - var_pb2_dn10);
        var_vdsats_dn11 = (var_pslsat_dn11 - var_pb2_dn11);
        var_vdsats_dn12 = (var_pslsat_dn12 - var_pb2_dn12);
        var_vdsats_dn17 = (var_pslsat_dn17 - var_pb2_dn17);

        let assign5560_e3612: f64 = (var_vdsats - 0.1);
        let assign5560_e3614: f64 = (assign5560_e3612 - 0.05);
        var_tmf1 = assign5560_e3614;
        var_tmf1_dn0 = var_vdsats_dn0;
        var_tmf1_dn2 = var_vdsats_dn2;
        var_tmf1_dn6 = var_vdsats_dn6;
        var_tmf1_dn7 = var_vdsats_dn7;
        var_tmf1_dn10 = var_vdsats_dn10;
        var_tmf1_dn11 = var_vdsats_dn11;
        var_tmf1_dn12 = var_vdsats_dn12;
        var_tmf1_dn17 = var_vdsats_dn17;

        let assign5570_e3617: f64 = (4.0 * 0.1);
        let assign5570_e3619: f64 = (assign5570_e3617 * 0.05);
        var_tmf2 = assign5570_e3619;
        var_tmf2_dn0 = 0.0;
        var_tmf2_dn2 = 0.0;
        var_tmf2_dn6 = 0.0;
        var_tmf2_dn7 = 0.0;
        var_tmf2_dn10 = 0.0;
        var_tmf2_dn11 = 0.0;
        var_tmf2_dn12 = 0.0;
        var_tmf2_dn17 = 0.0;

        let (assign5580_e3626, assign5580_e3626_d_n0, assign5580_e3626_d_n2, assign5580_e3626_d_n6, assign5580_e3626_d_n7, assign5580_e3626_d_n10, assign5580_e3626_d_n11, assign5580_e3626_d_n12, assign5580_e3626_d_n17,) = {
    if (var_tmf2 > 0.0) {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    } else {
        let assign5580_e3625: f64 = (-var_tmf2);
        (assign5580_e3625, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
    }
};
        var_tmf2 = assign5580_e3626;
        var_tmf2_dn0 = assign5580_e3626_d_n0;
        var_tmf2_dn2 = assign5580_e3626_d_n2;
        var_tmf2_dn6 = assign5580_e3626_d_n6;
        var_tmf2_dn7 = assign5580_e3626_d_n7;
        var_tmf2_dn10 = assign5580_e3626_d_n10;
        var_tmf2_dn11 = assign5580_e3626_d_n11;
        var_tmf2_dn12 = assign5580_e3626_d_n12;
        var_tmf2_dn17 = assign5580_e3626_d_n17;

        let assign5590_e3629: f64 = (var_tmf1 * var_tmf1);
        let assign5590_e3631: f64 = (assign5590_e3629 + var_tmf2);
        let assign5590_e3632: f64 = (assign5590_e3631).sqrt();
        var_tmf2 = assign5590_e3632;
        var_tmf2_dn0 = ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5590_e3632));
        var_tmf2_dn2 = ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5590_e3632));
        var_tmf2_dn6 = ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign5590_e3632));
        var_tmf2_dn7 = ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign5590_e3632));
        var_tmf2_dn10 = ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign5590_e3632));
        var_tmf2_dn11 = ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign5590_e3632));
        var_tmf2_dn12 = ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign5590_e3632));
        var_tmf2_dn17 = ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign5590_e3632));

        let assign5600_e3637: f64 = (var_tmf1 + var_tmf2);
        let assign5600_e3638: f64 = (0.5 * assign5600_e3637);
        let assign5600_e3639: f64 = (0.1 + assign5600_e3638);
        var_vdsats = assign5600_e3639;
        var_vdsats_dn0 = (0.5 * (var_tmf1_dn0 + var_tmf2_dn0));
        var_vdsats_dn2 = (0.5 * (var_tmf1_dn2 + var_tmf2_dn2));
        var_vdsats_dn6 = (0.5 * (var_tmf1_dn6 + var_tmf2_dn6));
        var_vdsats_dn7 = (0.5 * (var_tmf1_dn7 + var_tmf2_dn7));
        var_vdsats_dn10 = (0.5 * (var_tmf1_dn10 + var_tmf2_dn10));
        var_vdsats_dn11 = (0.5 * (var_tmf1_dn11 + var_tmf2_dn11));
        var_vdsats_dn12 = (0.5 * (var_tmf1_dn12 + var_tmf2_dn12));
        var_vdsats_dn17 = (0.5 * (var_tmf1_dn17 + var_tmf2_dn17));

        let assign5610_e3642: f64 = (var_vds / var_vdsats);
        var_t1__blk59 = assign5610_e3642;
        var_t1__blk59_dn0 = (((var_vds_dn0 * var_vdsats) - (var_vds * var_vdsats_dn0)) / (var_vdsats * var_vdsats));
        var_t1__blk59_dn2 = (((var_vds_dn2 * var_vdsats) - (var_vds * var_vdsats_dn2)) / (var_vdsats * var_vdsats));
        var_t1__blk59_dn6 = (((var_vds_dn6 * var_vdsats) - (var_vds * var_vdsats_dn6)) / (var_vdsats * var_vdsats));
        var_t1__blk59_dn7 = (((var_vds_dn7 * var_vdsats) - (var_vds * var_vdsats_dn7)) / (var_vdsats * var_vdsats));
        var_t1__blk59_dn10 = (((var_vds_dn10 * var_vdsats) - (var_vds * var_vdsats_dn10)) / (var_vdsats * var_vdsats));
        var_t1__blk59_dn11 = (((var_vds_dn11 * var_vdsats) - (var_vds * var_vdsats_dn11)) / (var_vdsats * var_vdsats));
        var_t1__blk59_dn12 = (((var_vds_dn12 * var_vdsats) - (var_vds * var_vdsats_dn12)) / (var_vdsats * var_vdsats));
        var_t1__blk59_dn17 = (((var_vds_dn17 * var_vdsats) - (var_vds * var_vdsats_dn17)) / (var_vdsats * var_vdsats));

        let assign5620_e3645: f64 = var_t1__blk59;
        var_tmf1 = assign5620_e3645;
        var_tmf1_dn0 = var_t1__blk59_dn0;
        var_tmf1_dn2 = var_t1__blk59_dn2;
        var_tmf1_dn6 = var_t1__blk59_dn6;
        var_tmf1_dn7 = var_t1__blk59_dn7;
        var_tmf1_dn10 = var_t1__blk59_dn10;
        var_tmf1_dn11 = var_t1__blk59_dn11;
        var_tmf1_dn12 = var_t1__blk59_dn12;
        var_tmf1_dn17 = var_t1__blk59_dn17;

        let assign5630_e3648: f64 = (var_tmf1 * var_tmf1);
        var_tmf2 = assign5630_e3648;
        var_tmf2_dn0 = ((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0));
        var_tmf2_dn2 = ((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2));
        var_tmf2_dn6 = ((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6));
        var_tmf2_dn7 = ((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7));
        var_tmf2_dn10 = ((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10));
        var_tmf2_dn11 = ((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11));
        var_tmf2_dn12 = ((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12));
        var_tmf2_dn17 = ((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17));

        let assign5640_e3651: f64 = (var_tmf2 * var_tmf1);
        var_tmf3 = assign5640_e3651;
        var_tmf3_dn0 = ((var_tmf2_dn0 * var_tmf1) + (var_tmf2 * var_tmf1_dn0));
        var_tmf3_dn2 = ((var_tmf2_dn2 * var_tmf1) + (var_tmf2 * var_tmf1_dn2));
        var_tmf3_dn6 = ((var_tmf2_dn6 * var_tmf1) + (var_tmf2 * var_tmf1_dn6));
        var_tmf3_dn7 = ((var_tmf2_dn7 * var_tmf1) + (var_tmf2 * var_tmf1_dn7));
        var_tmf3_dn10 = ((var_tmf2_dn10 * var_tmf1) + (var_tmf2 * var_tmf1_dn10));
        var_tmf3_dn11 = ((var_tmf2_dn11 * var_tmf1) + (var_tmf2 * var_tmf1_dn11));
        var_tmf3_dn12 = ((var_tmf2_dn12 * var_tmf1) + (var_tmf2 * var_tmf1_dn12));
        var_tmf3_dn17 = ((var_tmf2_dn17 * var_tmf1) + (var_tmf2 * var_tmf1_dn17));

        let assign5650_e3654: f64 = (var_tmf2 * var_tmf2);
        var_tmf4 = assign5650_e3654;
        var_tmf4_dn0 = ((var_tmf2_dn0 * var_tmf2) + (var_tmf2 * var_tmf2_dn0));
        var_tmf4_dn2 = ((var_tmf2_dn2 * var_tmf2) + (var_tmf2 * var_tmf2_dn2));
        var_tmf4_dn6 = ((var_tmf2_dn6 * var_tmf2) + (var_tmf2 * var_tmf2_dn6));
        var_tmf4_dn7 = ((var_tmf2_dn7 * var_tmf2) + (var_tmf2 * var_tmf2_dn7));
        var_tmf4_dn10 = ((var_tmf2_dn10 * var_tmf2) + (var_tmf2 * var_tmf2_dn10));
        var_tmf4_dn11 = ((var_tmf2_dn11 * var_tmf2) + (var_tmf2 * var_tmf2_dn11));
        var_tmf4_dn12 = ((var_tmf2_dn12 * var_tmf2) + (var_tmf2 * var_tmf2_dn12));
        var_tmf4_dn17 = ((var_tmf2_dn17 * var_tmf2) + (var_tmf2 * var_tmf2_dn17));

        let assign5660_e3658: f64 = (1.0 + var_tmf1);
        let assign5660_e3660: f64 = (assign5660_e3658 + var_tmf2);
        let assign5660_e3662: f64 = (assign5660_e3660 + var_tmf3);
        let assign5660_e3664: f64 = (assign5660_e3662 + var_tmf4);
        let assign5660_e3665: f64 = (1.0 / assign5660_e3664);
        var_tx__blk62 = assign5660_e3665;
        var_tx__blk62_dn0 = (-((((var_tmf1_dn0 + var_tmf2_dn0) + var_tmf3_dn0) + var_tmf4_dn0) / (assign5660_e3664 * assign5660_e3664)));
        var_tx__blk62_dn2 = (-((((var_tmf1_dn2 + var_tmf2_dn2) + var_tmf3_dn2) + var_tmf4_dn2) / (assign5660_e3664 * assign5660_e3664)));
        var_tx__blk62_dn6 = (-((((var_tmf1_dn6 + var_tmf2_dn6) + var_tmf3_dn6) + var_tmf4_dn6) / (assign5660_e3664 * assign5660_e3664)));
        var_tx__blk62_dn7 = (-((((var_tmf1_dn7 + var_tmf2_dn7) + var_tmf3_dn7) + var_tmf4_dn7) / (assign5660_e3664 * assign5660_e3664)));
        var_tx__blk62_dn10 = (-((((var_tmf1_dn10 + var_tmf2_dn10) + var_tmf3_dn10) + var_tmf4_dn10) / (assign5660_e3664 * assign5660_e3664)));
        var_tx__blk62_dn11 = (-((((var_tmf1_dn11 + var_tmf2_dn11) + var_tmf3_dn11) + var_tmf4_dn11) / (assign5660_e3664 * assign5660_e3664)));
        var_tx__blk62_dn12 = (-((((var_tmf1_dn12 + var_tmf2_dn12) + var_tmf3_dn12) + var_tmf4_dn12) / (assign5660_e3664 * assign5660_e3664)));
        var_tx__blk62_dn17 = (-((((var_tmf1_dn17 + var_tmf2_dn17) + var_tmf3_dn17) + var_tmf4_dn17) / (assign5660_e3664 * assign5660_e3664)));

        let assign5670_e3669: f64 = (2.0 * var_tmf1);
        let assign5670_e3670: f64 = (1.0 + assign5670_e3669);
        let assign5670_e3673: f64 = (3.0 * var_tmf2);
        let assign5670_e3674: f64 = (assign5670_e3670 + assign5670_e3673);
        let assign5670_e3677: f64 = (4.0 * var_tmf3);
        let assign5670_e3678: f64 = (assign5670_e3674 + assign5670_e3677);
        let assign5670_e3679: f64 = (-assign5670_e3678);
        let assign5670_e3681: f64 = (assign5670_e3679 * var_tx__blk62);
        let assign5670_e3683: f64 = (assign5670_e3681 * var_tx__blk62);
        var_t0 = assign5670_e3683;
        var_t0_dn0 = (((((-(((2.0 * var_tmf1_dn0) + (3.0 * var_tmf2_dn0)) + (4.0 * var_tmf3_dn0))) * var_tx__blk62) + (assign5670_e3679 * var_tx__blk62_dn0)) * var_tx__blk62) + (assign5670_e3681 * var_tx__blk62_dn0));
        var_t0_dn2 = (((((-(((2.0 * var_tmf1_dn2) + (3.0 * var_tmf2_dn2)) + (4.0 * var_tmf3_dn2))) * var_tx__blk62) + (assign5670_e3679 * var_tx__blk62_dn2)) * var_tx__blk62) + (assign5670_e3681 * var_tx__blk62_dn2));
        var_t0_dn6 = (((((-(((2.0 * var_tmf1_dn6) + (3.0 * var_tmf2_dn6)) + (4.0 * var_tmf3_dn6))) * var_tx__blk62) + (assign5670_e3679 * var_tx__blk62_dn6)) * var_tx__blk62) + (assign5670_e3681 * var_tx__blk62_dn6));
        var_t0_dn7 = (((((-(((2.0 * var_tmf1_dn7) + (3.0 * var_tmf2_dn7)) + (4.0 * var_tmf3_dn7))) * var_tx__blk62) + (assign5670_e3679 * var_tx__blk62_dn7)) * var_tx__blk62) + (assign5670_e3681 * var_tx__blk62_dn7));
        var_t0_dn10 = (((((-(((2.0 * var_tmf1_dn10) + (3.0 * var_tmf2_dn10)) + (4.0 * var_tmf3_dn10))) * var_tx__blk62) + (assign5670_e3679 * var_tx__blk62_dn10)) * var_tx__blk62) + (assign5670_e3681 * var_tx__blk62_dn10));
        var_t0_dn11 = (((((-(((2.0 * var_tmf1_dn11) + (3.0 * var_tmf2_dn11)) + (4.0 * var_tmf3_dn11))) * var_tx__blk62) + (assign5670_e3679 * var_tx__blk62_dn11)) * var_tx__blk62) + (assign5670_e3681 * var_tx__blk62_dn11));
        var_t0_dn12 = (((((-(((2.0 * var_tmf1_dn12) + (3.0 * var_tmf2_dn12)) + (4.0 * var_tmf3_dn12))) * var_tx__blk62) + (assign5670_e3679 * var_tx__blk62_dn12)) * var_tx__blk62) + (assign5670_e3681 * var_tx__blk62_dn12));
        var_t0_dn17 = (((((-(((2.0 * var_tmf1_dn17) + (3.0 * var_tmf2_dn17)) + (4.0 * var_tmf3_dn17))) * var_tx__blk62) + (assign5670_e3679 * var_tx__blk62_dn17)) * var_tx__blk62) + (assign5670_e3681 * var_tx__blk62_dn17));

        let assign5680_e3687: f64 = (1.0 - var_tx__blk62);
        let assign5680_e3688: f64 = assign5680_e3687;
        var_tx__blk62 = assign5680_e3688;
        var_tx__blk62_dn0 = (-var_tx__blk62_dn0);
        var_tx__blk62_dn2 = (-var_tx__blk62_dn2);
        var_tx__blk62_dn6 = (-var_tx__blk62_dn6);
        var_tx__blk62_dn7 = (-var_tx__blk62_dn7);
        var_tx__blk62_dn10 = (-var_tx__blk62_dn10);
        var_tx__blk62_dn11 = (-var_tx__blk62_dn11);
        var_tx__blk62_dn12 = (-var_tx__blk62_dn12);
        var_tx__blk62_dn17 = (-var_tx__blk62_dn17);

        let assign5690_e3690: f64 = (-var_t0);
        var_t0 = assign5690_e3690;
        var_t0_dn0 = (-var_t0_dn0);
        var_t0_dn2 = (-var_t0_dn2);
        var_t0_dn6 = (-var_t0_dn6);
        var_t0_dn7 = (-var_t0_dn7);
        var_t0_dn10 = (-var_t0_dn10);
        var_t0_dn11 = (-var_t0_dn11);
        var_t0_dn12 = (-var_t0_dn12);
        var_t0_dn17 = (-var_t0_dn17);

        let assign5700_e3693: f64 = (var_tx__blk62 * var_tx__blk62);
        var_fmdvds = assign5700_e3693;
        var_fmdvds_dn0 = ((var_tx__blk62_dn0 * var_tx__blk62) + (var_tx__blk62 * var_tx__blk62_dn0));
        var_fmdvds_dn2 = ((var_tx__blk62_dn2 * var_tx__blk62) + (var_tx__blk62 * var_tx__blk62_dn2));
        var_fmdvds_dn6 = ((var_tx__blk62_dn6 * var_tx__blk62) + (var_tx__blk62 * var_tx__blk62_dn6));
        var_fmdvds_dn7 = ((var_tx__blk62_dn7 * var_tx__blk62) + (var_tx__blk62 * var_tx__blk62_dn7));
        var_fmdvds_dn10 = ((var_tx__blk62_dn10 * var_tx__blk62) + (var_tx__blk62 * var_tx__blk62_dn10));
        var_fmdvds_dn11 = ((var_tx__blk62_dn11 * var_tx__blk62) + (var_tx__blk62 * var_tx__blk62_dn11));
        var_fmdvds_dn12 = ((var_tx__blk62_dn12 * var_tx__blk62) + (var_tx__blk62 * var_tx__blk62_dn12));
        var_fmdvds_dn17 = ((var_tx__blk62_dn17 * var_tx__blk62) + (var_tx__blk62 * var_tx__blk62_dn17));

        let assign5710_e3704: f64 = if (((p.p204 == 0.0) && (p.p206 == 0.0)) || (p.p205 == 0.0)) { 1.0 } else { 0.0 };
        var_guard70 = assign5710_e3704;

        let (assign5720_e3708,) = {
    if (var_guard70 != 0.0) {
        (0.0,)
    } else {
        (var_flg_qme,)
    }
};
        var_flg_qme = assign5720_e3708;

        let (assign5730_e3713,) = {
    if (var_guard70 == 0.0) {
        (1.0,)
    } else {
        (var_flg_qme,)
    }
};
        var_flg_qme = assign5730_e3713;

        let assign5740_e3716: f64 = (2.0 * var_q_nsub);
        let assign5740_e3718: f64 = (assign5740_e3716 * 1.034943e-10);
        let assign5740_e3720: f64 = (assign5740_e3718 * var_pb20);
        let assign5740_e3721: f64 = (assign5740_e3720).sqrt();
        var_t2__blk64 = assign5740_e3721;
        var_t2__blk64_dn0 = (((((2.0 * var_q_nsub_dn0) * 1.034943e-10) * var_pb20) + (assign5740_e3718 * var_pb20_dn0)) / (2.0 * assign5740_e3721));
        var_t2__blk64_dn2 = (((((2.0 * var_q_nsub_dn2) * 1.034943e-10) * var_pb20) + (assign5740_e3718 * var_pb20_dn2)) / (2.0 * assign5740_e3721));
        var_t2__blk64_dn6 = (((((2.0 * var_q_nsub_dn6) * 1.034943e-10) * var_pb20) + (assign5740_e3718 * var_pb20_dn6)) / (2.0 * assign5740_e3721));
        var_t2__blk64_dn7 = (((((2.0 * var_q_nsub_dn7) * 1.034943e-10) * var_pb20) + (assign5740_e3718 * var_pb20_dn7)) / (2.0 * assign5740_e3721));
        var_t2__blk64_dn10 = (((((2.0 * var_q_nsub_dn10) * 1.034943e-10) * var_pb20) + (assign5740_e3718 * var_pb20_dn10)) / (2.0 * assign5740_e3721));
        var_t2__blk64_dn11 = (((((2.0 * var_q_nsub_dn11) * 1.034943e-10) * var_pb20) + (assign5740_e3718 * var_pb20_dn11)) / (2.0 * assign5740_e3721));
        var_t2__blk64_dn12 = (((((2.0 * var_q_nsub_dn12) * 1.034943e-10) * var_pb20) + (assign5740_e3718 * var_pb20_dn12)) / (2.0 * assign5740_e3721));
        var_t2__blk64_dn17 = (((((2.0 * var_q_nsub_dn17) * 1.034943e-10) * var_pb20) + (assign5740_e3718 * var_pb20_dn17)) / (2.0 * assign5740_e3721));

        let assign5750_e3724: f64 = (var_pb20 + var_vfb);
        let assign5750_e3727: f64 = (var_t2__blk64 / var_c_fox0);
        let assign5750_e3728: f64 = (assign5750_e3724 + assign5750_e3727);
        var_vthq = assign5750_e3728;
        var_vthq_dn0 = (var_pb20_dn0 + (var_t2__blk64_dn0 / var_c_fox0));
        var_vthq_dn2 = (var_pb20_dn2 + (var_t2__blk64_dn2 / var_c_fox0));
        var_vthq_dn6 = (var_pb20_dn6 + (var_t2__blk64_dn6 / var_c_fox0));
        var_vthq_dn7 = (var_pb20_dn7 + (var_t2__blk64_dn7 / var_c_fox0));
        var_vthq_dn10 = (var_pb20_dn10 + (var_t2__blk64_dn10 / var_c_fox0));
        var_vthq_dn11 = (var_pb20_dn11 + (var_t2__blk64_dn11 / var_c_fox0));
        var_vthq_dn12 = (var_pb20_dn12 + (var_t2__blk64_dn12 / var_c_fox0));
        var_vthq_dn17 = (var_pb20_dn17 + (var_t2__blk64_dn17 / var_c_fox0));

        let assign5760_e3731: f64 = if var_flg_qme == 0.0 { 1.0 } else { 0.0 };
        var_guard71 = assign5760_e3731;

        let (assign5770_e3735, assign5770_e3735_d_n0, assign5770_e3735_d_n2, assign5770_e3735_d_n6, assign5770_e3735_d_n7, assign5770_e3735_d_n10, assign5770_e3735_d_n11, assign5770_e3735_d_n12, assign5770_e3735_d_n17,) = {
    if (var_guard71 != 0.0) {
        (var_tfox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tfoxe, var_tfoxe_dn0, var_tfoxe_dn2, var_tfoxe_dn6, var_tfoxe_dn7, var_tfoxe_dn10, var_tfoxe_dn11, var_tfoxe_dn12, var_tfoxe_dn17,)
    }
};
        var_tfoxe = assign5770_e3735;
        var_tfoxe_dn0 = assign5770_e3735_d_n0;
        var_tfoxe_dn2 = assign5770_e3735_d_n2;
        var_tfoxe_dn6 = assign5770_e3735_d_n6;
        var_tfoxe_dn7 = assign5770_e3735_d_n7;
        var_tfoxe_dn10 = assign5770_e3735_d_n10;
        var_tfoxe_dn11 = assign5770_e3735_d_n11;
        var_tfoxe_dn12 = assign5770_e3735_d_n12;
        var_tfoxe_dn17 = assign5770_e3735_d_n17;

        let (assign5780_e3739, assign5780_e3739_d_n0, assign5780_e3739_d_n2, assign5780_e3739_d_n6, assign5780_e3739_d_n7, assign5780_e3739_d_n10, assign5780_e3739_d_n11, assign5780_e3739_d_n12, assign5780_e3739_d_n17,) = {
    if (var_guard71 != 0.0) {
        (var_c_fox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_c_fox, var_c_fox_dn0, var_c_fox_dn2, var_c_fox_dn6, var_c_fox_dn7, var_c_fox_dn10, var_c_fox_dn11, var_c_fox_dn12, var_c_fox_dn17,)
    }
};
        var_c_fox = assign5780_e3739;
        var_c_fox_dn0 = assign5780_e3739_d_n0;
        var_c_fox_dn2 = assign5780_e3739_d_n2;
        var_c_fox_dn6 = assign5780_e3739_d_n6;
        var_c_fox_dn7 = assign5780_e3739_d_n7;
        var_c_fox_dn10 = assign5780_e3739_d_n10;
        var_c_fox_dn11 = assign5780_e3739_d_n11;
        var_c_fox_dn12 = assign5780_e3739_d_n12;
        var_c_fox_dn17 = assign5780_e3739_d_n17;

        let (assign5790_e3743, assign5790_e3743_d_n0, assign5790_e3743_d_n2, assign5790_e3743_d_n6, assign5790_e3743_d_n7, assign5790_e3743_d_n10, assign5790_e3743_d_n11, assign5790_e3743_d_n12, assign5790_e3743_d_n17,) = {
    if (var_guard71 != 0.0) {
        (var_c_fox0_inv, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_c_fox_inv, var_c_fox_inv_dn0, var_c_fox_inv_dn2, var_c_fox_inv_dn6, var_c_fox_inv_dn7, var_c_fox_inv_dn10, var_c_fox_inv_dn11, var_c_fox_inv_dn12, var_c_fox_inv_dn17,)
    }
};
        var_c_fox_inv = assign5790_e3743;
        var_c_fox_inv_dn0 = assign5790_e3743_d_n0;
        var_c_fox_inv_dn2 = assign5790_e3743_d_n2;
        var_c_fox_inv_dn6 = assign5790_e3743_d_n6;
        var_c_fox_inv_dn7 = assign5790_e3743_d_n7;
        var_c_fox_inv_dn10 = assign5790_e3743_d_n10;
        var_c_fox_inv_dn11 = assign5790_e3743_d_n11;
        var_c_fox_inv_dn12 = assign5790_e3743_d_n12;
        var_c_fox_inv_dn17 = assign5790_e3743_d_n17;

        let (assign5800_e3753, assign5800_e3753_d_n0, assign5800_e3753_d_n2, assign5800_e3753_d_n6, assign5800_e3753_d_n7, assign5800_e3753_d_n10, assign5800_e3753_d_n11, assign5800_e3753_d_n12, assign5800_e3753_d_n17,) = {
    if (var_guard71 != 0.0) {
        let assign5800_e3747: f64 = (var_cnst0soi * var_c_fox0_inv);
        let assign5800_e3749: f64 = (assign5800_e3747 * var_c_fox0_inv);
        let assign5800_e3751: f64 = (assign5800_e3749 * var_cnst0soi);
        (assign5800_e3751, ((((var_cnst0soi_dn0 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5800_e3749 * var_cnst0soi_dn0)), ((((var_cnst0soi_dn2 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5800_e3749 * var_cnst0soi_dn2)), ((((var_cnst0soi_dn6 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5800_e3749 * var_cnst0soi_dn6)), ((((var_cnst0soi_dn7 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5800_e3749 * var_cnst0soi_dn7)), ((((var_cnst0soi_dn10 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5800_e3749 * var_cnst0soi_dn10)), ((((var_cnst0soi_dn11 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5800_e3749 * var_cnst0soi_dn11)), ((((var_cnst0soi_dn12 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5800_e3749 * var_cnst0soi_dn12)), ((((var_cnst0soi_dn17 * var_c_fox0_inv) * var_c_fox0_inv) * var_cnst0soi) + (assign5800_e3749 * var_cnst0soi_dn17)),)
    } else {
        (var_cnstc_foxi, var_cnstc_foxi_dn0, var_cnstc_foxi_dn2, var_cnstc_foxi_dn6, var_cnstc_foxi_dn7, var_cnstc_foxi_dn10, var_cnstc_foxi_dn11, var_cnstc_foxi_dn12, var_cnstc_foxi_dn17,)
    }
};
        var_cnstc_foxi = assign5800_e3753;
        var_cnstc_foxi_dn0 = assign5800_e3753_d_n0;
        var_cnstc_foxi_dn2 = assign5800_e3753_d_n2;
        var_cnstc_foxi_dn6 = assign5800_e3753_d_n6;
        var_cnstc_foxi_dn7 = assign5800_e3753_d_n7;
        var_cnstc_foxi_dn10 = assign5800_e3753_d_n10;
        var_cnstc_foxi_dn11 = assign5800_e3753_d_n11;
        var_cnstc_foxi_dn12 = assign5800_e3753_d_n12;
        var_cnstc_foxi_dn17 = assign5800_e3753_d_n17;

        *var_c_fox_slot = var_c_fox;
        *var_c_fox_dn0_slot = var_c_fox_dn0;
        *var_c_fox_dn10_slot = var_c_fox_dn10;
        *var_c_fox_dn11_slot = var_c_fox_dn11;
        *var_c_fox_dn12_slot = var_c_fox_dn12;
        *var_c_fox_dn17_slot = var_c_fox_dn17;
        *var_c_fox_dn2_slot = var_c_fox_dn2;
        *var_c_fox_dn6_slot = var_c_fox_dn6;
        *var_c_fox_dn7_slot = var_c_fox_dn7;
        *var_c_fox_inv_slot = var_c_fox_inv;
        *var_c_fox_inv_dn0_slot = var_c_fox_inv_dn0;
        *var_c_fox_inv_dn10_slot = var_c_fox_inv_dn10;
        *var_c_fox_inv_dn11_slot = var_c_fox_inv_dn11;
        *var_c_fox_inv_dn12_slot = var_c_fox_inv_dn12;
        *var_c_fox_inv_dn17_slot = var_c_fox_inv_dn17;
        *var_c_fox_inv_dn2_slot = var_c_fox_inv_dn2;
        *var_c_fox_inv_dn6_slot = var_c_fox_inv_dn6;
        *var_c_fox_inv_dn7_slot = var_c_fox_inv_dn7;
        *var_cnstc_foxi_slot = var_cnstc_foxi;
        *var_cnstc_foxi_dn0_slot = var_cnstc_foxi_dn0;
        *var_cnstc_foxi_dn10_slot = var_cnstc_foxi_dn10;
        *var_cnstc_foxi_dn11_slot = var_cnstc_foxi_dn11;
        *var_cnstc_foxi_dn12_slot = var_cnstc_foxi_dn12;
        *var_cnstc_foxi_dn17_slot = var_cnstc_foxi_dn17;
        *var_cnstc_foxi_dn2_slot = var_cnstc_foxi_dn2;
        *var_cnstc_foxi_dn6_slot = var_cnstc_foxi_dn6;
        *var_cnstc_foxi_dn7_slot = var_cnstc_foxi_dn7;
        *var_flg_qme_slot = var_flg_qme;
        *var_fmdvds_slot = var_fmdvds;
        *var_fmdvds_dn0_slot = var_fmdvds_dn0;
        *var_fmdvds_dn10_slot = var_fmdvds_dn10;
        *var_fmdvds_dn11_slot = var_fmdvds_dn11;
        *var_fmdvds_dn12_slot = var_fmdvds_dn12;
        *var_fmdvds_dn17_slot = var_fmdvds_dn17;
        *var_fmdvds_dn2_slot = var_fmdvds_dn2;
        *var_fmdvds_dn6_slot = var_fmdvds_dn6;
        *var_fmdvds_dn7_slot = var_fmdvds_dn7;
        *var_guard63_slot = var_guard63;
        *var_guard70_slot = var_guard70;
        *var_guard71_slot = var_guard71;
        *var_pslsat_slot = var_pslsat;
        *var_pslsat_dn0_slot = var_pslsat_dn0;
        *var_pslsat_dn10_slot = var_pslsat_dn10;
        *var_pslsat_dn11_slot = var_pslsat_dn11;
        *var_pslsat_dn12_slot = var_pslsat_dn12;
        *var_pslsat_dn17_slot = var_pslsat_dn17;
        *var_pslsat_dn2_slot = var_pslsat_dn2;
        *var_pslsat_dn6_slot = var_pslsat_dn6;
        *var_pslsat_dn7_slot = var_pslsat_dn7;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn17_slot = var_t0_dn17;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t1__blk59_slot = var_t1__blk59;
        *var_t1__blk59_dn0_slot = var_t1__blk59_dn0;
        *var_t1__blk59_dn10_slot = var_t1__blk59_dn10;
        *var_t1__blk59_dn11_slot = var_t1__blk59_dn11;
        *var_t1__blk59_dn12_slot = var_t1__blk59_dn12;
        *var_t1__blk59_dn17_slot = var_t1__blk59_dn17;
        *var_t1__blk59_dn2_slot = var_t1__blk59_dn2;
        *var_t1__blk59_dn6_slot = var_t1__blk59_dn6;
        *var_t1__blk59_dn7_slot = var_t1__blk59_dn7;
        *var_t2__blk60_slot = var_t2__blk60;
        *var_t2__blk60_dn11_slot = var_t2__blk60_dn11;
        *var_t2__blk60_dn6_slot = var_t2__blk60_dn6;
        *var_t2__blk60_dn7_slot = var_t2__blk60_dn7;
        *var_t2__blk64_slot = var_t2__blk64;
        *var_t2__blk64_dn0_slot = var_t2__blk64_dn0;
        *var_t2__blk64_dn10_slot = var_t2__blk64_dn10;
        *var_t2__blk64_dn11_slot = var_t2__blk64_dn11;
        *var_t2__blk64_dn12_slot = var_t2__blk64_dn12;
        *var_t2__blk64_dn17_slot = var_t2__blk64_dn17;
        *var_t2__blk64_dn2_slot = var_t2__blk64_dn2;
        *var_t2__blk64_dn6_slot = var_t2__blk64_dn6;
        *var_t2__blk64_dn7_slot = var_t2__blk64_dn7;
        *var_t3__blk61_slot = var_t3__blk61;
        *var_t3__blk61_dn0_slot = var_t3__blk61_dn0;
        *var_t3__blk61_dn10_slot = var_t3__blk61_dn10;
        *var_t3__blk61_dn11_slot = var_t3__blk61_dn11;
        *var_t3__blk61_dn12_slot = var_t3__blk61_dn12;
        *var_t3__blk61_dn17_slot = var_t3__blk61_dn17;
        *var_t3__blk61_dn2_slot = var_t3__blk61_dn2;
        *var_t3__blk61_dn6_slot = var_t3__blk61_dn6;
        *var_t3__blk61_dn7_slot = var_t3__blk61_dn7;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn17_slot = var_t4_dn17;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_tfoxe_slot = var_tfoxe;
        *var_tfoxe_dn0_slot = var_tfoxe_dn0;
        *var_tfoxe_dn10_slot = var_tfoxe_dn10;
        *var_tfoxe_dn11_slot = var_tfoxe_dn11;
        *var_tfoxe_dn12_slot = var_tfoxe_dn12;
        *var_tfoxe_dn17_slot = var_tfoxe_dn17;
        *var_tfoxe_dn2_slot = var_tfoxe_dn2;
        *var_tfoxe_dn6_slot = var_tfoxe_dn6;
        *var_tfoxe_dn7_slot = var_tfoxe_dn7;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn17_slot = var_tmf2_dn17;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf3_slot = var_tmf3;
        *var_tmf3_dn0_slot = var_tmf3_dn0;
        *var_tmf3_dn10_slot = var_tmf3_dn10;
        *var_tmf3_dn11_slot = var_tmf3_dn11;
        *var_tmf3_dn12_slot = var_tmf3_dn12;
        *var_tmf3_dn17_slot = var_tmf3_dn17;
        *var_tmf3_dn2_slot = var_tmf3_dn2;
        *var_tmf3_dn6_slot = var_tmf3_dn6;
        *var_tmf3_dn7_slot = var_tmf3_dn7;
        *var_tmf4_slot = var_tmf4;
        *var_tmf4_dn0_slot = var_tmf4_dn0;
        *var_tmf4_dn10_slot = var_tmf4_dn10;
        *var_tmf4_dn11_slot = var_tmf4_dn11;
        *var_tmf4_dn12_slot = var_tmf4_dn12;
        *var_tmf4_dn17_slot = var_tmf4_dn17;
        *var_tmf4_dn2_slot = var_tmf4_dn2;
        *var_tmf4_dn6_slot = var_tmf4_dn6;
        *var_tmf4_dn7_slot = var_tmf4_dn7;
        *var_tx__blk62_slot = var_tx__blk62;
        *var_tx__blk62_dn0_slot = var_tx__blk62_dn0;
        *var_tx__blk62_dn10_slot = var_tx__blk62_dn10;
        *var_tx__blk62_dn11_slot = var_tx__blk62_dn11;
        *var_tx__blk62_dn12_slot = var_tx__blk62_dn12;
        *var_tx__blk62_dn17_slot = var_tx__blk62_dn17;
        *var_tx__blk62_dn2_slot = var_tx__blk62_dn2;
        *var_tx__blk62_dn6_slot = var_tx__blk62_dn6;
        *var_tx__blk62_dn7_slot = var_tx__blk62_dn7;
        *var_vbsp_slot = var_vbsp;
        *var_vbsp_dn0_slot = var_vbsp_dn0;
        *var_vbsp_dn10_slot = var_vbsp_dn10;
        *var_vbsp_dn11_slot = var_vbsp_dn11;
        *var_vbsp_dn12_slot = var_vbsp_dn12;
        *var_vbsp_dn17_slot = var_vbsp_dn17;
        *var_vbsp_dn2_slot = var_vbsp_dn2;
        *var_vbsp_dn6_slot = var_vbsp_dn6;
        *var_vbsp_dn7_slot = var_vbsp_dn7;
        *var_vbspz_slot = var_vbspz;
        *var_vbspz_dn0_slot = var_vbspz_dn0;
        *var_vbspz_dn10_slot = var_vbspz_dn10;
        *var_vbspz_dn11_slot = var_vbspz_dn11;
        *var_vbspz_dn12_slot = var_vbspz_dn12;
        *var_vbspz_dn17_slot = var_vbspz_dn17;
        *var_vbspz_dn2_slot = var_vbspz_dn2;
        *var_vbspz_dn6_slot = var_vbspz_dn6;
        *var_vbspz_dn7_slot = var_vbspz_dn7;
        *var_vdsats_slot = var_vdsats;
        *var_vdsats_dn0_slot = var_vdsats_dn0;
        *var_vdsats_dn10_slot = var_vdsats_dn10;
        *var_vdsats_dn11_slot = var_vdsats_dn11;
        *var_vdsats_dn12_slot = var_vdsats_dn12;
        *var_vdsats_dn17_slot = var_vdsats_dn17;
        *var_vdsats_dn2_slot = var_vdsats_dn2;
        *var_vdsats_dn6_slot = var_vdsats_dn6;
        *var_vdsats_dn7_slot = var_vdsats_dn7;
        *var_vthq_slot = var_vthq;
        *var_vthq_dn0_slot = var_vthq_dn0;
        *var_vthq_dn10_slot = var_vthq_dn10;
        *var_vthq_dn11_slot = var_vthq_dn11;
        *var_vthq_dn12_slot = var_vthq_dn12;
        *var_vthq_dn17_slot = var_vthq_dn17;
        *var_vthq_dn2_slot = var_vthq_dn2;
        *var_vthq_dn6_slot = var_vthq_dn6;
        *var_vthq_dn7_slot = var_vthq_dn7;
    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        var_cnst0soi: f64,
        var_cnst0soi_dn0: f64,
        var_cnst0soi_dn10: f64,
        var_cnst0soi_dn11: f64,
        var_cnst0soi_dn12: f64,
        var_cnst0soi_dn17: f64,
        var_cnst0soi_dn2: f64,
        var_cnst0soi_dn6: f64,
        var_cnst0soi_dn7: f64,
        var_guard71: f64,
        var_subversion: f64,
        var_tfox0: f64,
        var_vbsp: f64,
        var_vbsp_dn0: f64,
        var_vbsp_dn10: f64,
        var_vbsp_dn11: f64,
        var_vbsp_dn12: f64,
        var_vbsp_dn17: f64,
        var_vbsp_dn2: f64,
        var_vbsp_dn6: f64,
        var_vbsp_dn7: f64,
        var_vbspz: f64,
        var_vbspz_dn0: f64,
        var_vbspz_dn10: f64,
        var_vbspz_dn11: f64,
        var_vbspz_dn12: f64,
        var_vbspz_dn17: f64,
        var_vbspz_dn2: f64,
        var_vbspz_dn6: f64,
        var_vbspz_dn7: f64,
        var_vfb: f64,
        var_vgs: f64,
        var_vgs_dn11: f64,
        var_vgs_dn6: f64,
        var_vgs_dn7: f64,
        var_vthq: f64,
        var_vthq_dn0: f64,
        var_vthq_dn10: f64,
        var_vthq_dn11: f64,
        var_vthq_dn12: f64,
        var_vthq_dn17: f64,
        var_vthq_dn2: f64,
        var_vthq_dn6: f64,
        var_vthq_dn7: f64,
        var_c_fox_slot: &mut f64,
        var_c_fox_dn0_slot: &mut f64,
        var_c_fox_dn10_slot: &mut f64,
        var_c_fox_dn11_slot: &mut f64,
        var_c_fox_dn12_slot: &mut f64,
        var_c_fox_dn17_slot: &mut f64,
        var_c_fox_dn2_slot: &mut f64,
        var_c_fox_dn6_slot: &mut f64,
        var_c_fox_dn7_slot: &mut f64,
        var_c_fox_inv_slot: &mut f64,
        var_c_fox_inv_dn0_slot: &mut f64,
        var_c_fox_inv_dn10_slot: &mut f64,
        var_c_fox_inv_dn11_slot: &mut f64,
        var_c_fox_inv_dn12_slot: &mut f64,
        var_c_fox_inv_dn17_slot: &mut f64,
        var_c_fox_inv_dn2_slot: &mut f64,
        var_c_fox_inv_dn6_slot: &mut f64,
        var_c_fox_inv_dn7_slot: &mut f64,
        var_cnstc_foxi_slot: &mut f64,
        var_cnstc_foxi_dn0_slot: &mut f64,
        var_cnstc_foxi_dn10_slot: &mut f64,
        var_cnstc_foxi_dn11_slot: &mut f64,
        var_cnstc_foxi_dn12_slot: &mut f64,
        var_cnstc_foxi_dn17_slot: &mut f64,
        var_cnstc_foxi_dn2_slot: &mut f64,
        var_cnstc_foxi_dn6_slot: &mut f64,
        var_cnstc_foxi_dn7_slot: &mut f64,
        var_dtfox_slot: &mut f64,
        var_dtfox_dn0_slot: &mut f64,
        var_dtfox_dn10_slot: &mut f64,
        var_dtfox_dn11_slot: &mut f64,
        var_dtfox_dn12_slot: &mut f64,
        var_dtfox_dn17_slot: &mut f64,
        var_dtfox_dn2_slot: &mut f64,
        var_dtfox_dn6_slot: &mut f64,
        var_dtfox_dn7_slot: &mut f64,
        var_flg_qme_slot: &mut f64,
        var_guard72_slot: &mut f64,
        var_guard73_slot: &mut f64,
        var_guard74_slot: &mut f64,
        var_t2__blk64_slot: &mut f64,
        var_t2__blk64_dn0_slot: &mut f64,
        var_t2__blk64_dn10_slot: &mut f64,
        var_t2__blk64_dn11_slot: &mut f64,
        var_t2__blk64_dn12_slot: &mut f64,
        var_t2__blk64_dn17_slot: &mut f64,
        var_t2__blk64_dn2_slot: &mut f64,
        var_t2__blk64_dn6_slot: &mut f64,
        var_t2__blk64_dn7_slot: &mut f64,
        var_t3__blk65_slot: &mut f64,
        var_t3__blk65_dn0_slot: &mut f64,
        var_t3__blk65_dn10_slot: &mut f64,
        var_t3__blk65_dn11_slot: &mut f64,
        var_t3__blk65_dn12_slot: &mut f64,
        var_t3__blk65_dn17_slot: &mut f64,
        var_t3__blk65_dn2_slot: &mut f64,
        var_t3__blk65_dn6_slot: &mut f64,
        var_t3__blk65_dn7_slot: &mut f64,
        var_t4__blk66_slot: &mut f64,
        var_t4__blk66_dn0_slot: &mut f64,
        var_t4__blk66_dn10_slot: &mut f64,
        var_t4__blk66_dn11_slot: &mut f64,
        var_t4__blk66_dn12_slot: &mut f64,
        var_t4__blk66_dn17_slot: &mut f64,
        var_t4__blk66_dn2_slot: &mut f64,
        var_t4__blk66_dn6_slot: &mut f64,
        var_t4__blk66_dn7_slot: &mut f64,
        var_t4w_slot: &mut f64,
        var_t4w_dn0_slot: &mut f64,
        var_t4w_dn10_slot: &mut f64,
        var_t4w_dn11_slot: &mut f64,
        var_t4w_dn12_slot: &mut f64,
        var_t4w_dn17_slot: &mut f64,
        var_t4w_dn2_slot: &mut f64,
        var_t4w_dn6_slot: &mut f64,
        var_t4w_dn7_slot: &mut f64,
        var_t5__blk68_slot: &mut f64,
        var_t5__blk68_dn0_slot: &mut f64,
        var_t5__blk68_dn10_slot: &mut f64,
        var_t5__blk68_dn11_slot: &mut f64,
        var_t5__blk68_dn12_slot: &mut f64,
        var_t5__blk68_dn17_slot: &mut f64,
        var_t5__blk68_dn2_slot: &mut f64,
        var_t5__blk68_dn6_slot: &mut f64,
        var_t5__blk68_dn7_slot: &mut f64,
        var_t6__blk69_slot: &mut f64,
        var_t6__blk69_dn0_slot: &mut f64,
        var_t6__blk69_dn10_slot: &mut f64,
        var_t6__blk69_dn11_slot: &mut f64,
        var_t6__blk69_dn12_slot: &mut f64,
        var_t6__blk69_dn17_slot: &mut f64,
        var_t6__blk69_dn2_slot: &mut f64,
        var_t6__blk69_dn6_slot: &mut f64,
        var_t6__blk69_dn7_slot: &mut f64,
        var_tfoxe_slot: &mut f64,
        var_tfoxe_dn0_slot: &mut f64,
        var_tfoxe_dn10_slot: &mut f64,
        var_tfoxe_dn11_slot: &mut f64,
        var_tfoxe_dn12_slot: &mut f64,
        var_tfoxe_dn17_slot: &mut f64,
        var_tfoxe_dn2_slot: &mut f64,
        var_tfoxe_dn6_slot: &mut f64,
        var_tfoxe_dn7_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn17_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_vbsz2_slot: &mut f64,
        var_vbsz2_dn0_slot: &mut f64,
        var_vbsz2_dn10_slot: &mut f64,
        var_vbsz2_dn11_slot: &mut f64,
        var_vbsz2_dn12_slot: &mut f64,
        var_vbsz2_dn17_slot: &mut f64,
        var_vbsz2_dn2_slot: &mut f64,
        var_vbsz2_dn6_slot: &mut f64,
        var_vbsz2_dn7_slot: &mut f64,
    ) {
        let mut var_c_fox: f64 = *var_c_fox_slot;
        let mut var_c_fox_dn0: f64 = *var_c_fox_dn0_slot;
        let mut var_c_fox_dn10: f64 = *var_c_fox_dn10_slot;
        let mut var_c_fox_dn11: f64 = *var_c_fox_dn11_slot;
        let mut var_c_fox_dn12: f64 = *var_c_fox_dn12_slot;
        let mut var_c_fox_dn17: f64 = *var_c_fox_dn17_slot;
        let mut var_c_fox_dn2: f64 = *var_c_fox_dn2_slot;
        let mut var_c_fox_dn6: f64 = *var_c_fox_dn6_slot;
        let mut var_c_fox_dn7: f64 = *var_c_fox_dn7_slot;
        let mut var_c_fox_inv: f64 = *var_c_fox_inv_slot;
        let mut var_c_fox_inv_dn0: f64 = *var_c_fox_inv_dn0_slot;
        let mut var_c_fox_inv_dn10: f64 = *var_c_fox_inv_dn10_slot;
        let mut var_c_fox_inv_dn11: f64 = *var_c_fox_inv_dn11_slot;
        let mut var_c_fox_inv_dn12: f64 = *var_c_fox_inv_dn12_slot;
        let mut var_c_fox_inv_dn17: f64 = *var_c_fox_inv_dn17_slot;
        let mut var_c_fox_inv_dn2: f64 = *var_c_fox_inv_dn2_slot;
        let mut var_c_fox_inv_dn6: f64 = *var_c_fox_inv_dn6_slot;
        let mut var_c_fox_inv_dn7: f64 = *var_c_fox_inv_dn7_slot;
        let mut var_cnstc_foxi: f64 = *var_cnstc_foxi_slot;
        let mut var_cnstc_foxi_dn0: f64 = *var_cnstc_foxi_dn0_slot;
        let mut var_cnstc_foxi_dn10: f64 = *var_cnstc_foxi_dn10_slot;
        let mut var_cnstc_foxi_dn11: f64 = *var_cnstc_foxi_dn11_slot;
        let mut var_cnstc_foxi_dn12: f64 = *var_cnstc_foxi_dn12_slot;
        let mut var_cnstc_foxi_dn17: f64 = *var_cnstc_foxi_dn17_slot;
        let mut var_cnstc_foxi_dn2: f64 = *var_cnstc_foxi_dn2_slot;
        let mut var_cnstc_foxi_dn6: f64 = *var_cnstc_foxi_dn6_slot;
        let mut var_cnstc_foxi_dn7: f64 = *var_cnstc_foxi_dn7_slot;
        let mut var_dtfox: f64 = *var_dtfox_slot;
        let mut var_dtfox_dn0: f64 = *var_dtfox_dn0_slot;
        let mut var_dtfox_dn10: f64 = *var_dtfox_dn10_slot;
        let mut var_dtfox_dn11: f64 = *var_dtfox_dn11_slot;
        let mut var_dtfox_dn12: f64 = *var_dtfox_dn12_slot;
        let mut var_dtfox_dn17: f64 = *var_dtfox_dn17_slot;
        let mut var_dtfox_dn2: f64 = *var_dtfox_dn2_slot;
        let mut var_dtfox_dn6: f64 = *var_dtfox_dn6_slot;
        let mut var_dtfox_dn7: f64 = *var_dtfox_dn7_slot;
        let mut var_flg_qme: f64 = *var_flg_qme_slot;
        let mut var_guard72: f64 = *var_guard72_slot;
        let mut var_guard73: f64 = *var_guard73_slot;
        let mut var_guard74: f64 = *var_guard74_slot;
        let mut var_t2__blk64: f64 = *var_t2__blk64_slot;
        let mut var_t2__blk64_dn0: f64 = *var_t2__blk64_dn0_slot;
        let mut var_t2__blk64_dn10: f64 = *var_t2__blk64_dn10_slot;
        let mut var_t2__blk64_dn11: f64 = *var_t2__blk64_dn11_slot;
        let mut var_t2__blk64_dn12: f64 = *var_t2__blk64_dn12_slot;
        let mut var_t2__blk64_dn17: f64 = *var_t2__blk64_dn17_slot;
        let mut var_t2__blk64_dn2: f64 = *var_t2__blk64_dn2_slot;
        let mut var_t2__blk64_dn6: f64 = *var_t2__blk64_dn6_slot;
        let mut var_t2__blk64_dn7: f64 = *var_t2__blk64_dn7_slot;
        let mut var_t3__blk65: f64 = *var_t3__blk65_slot;
        let mut var_t3__blk65_dn0: f64 = *var_t3__blk65_dn0_slot;
        let mut var_t3__blk65_dn10: f64 = *var_t3__blk65_dn10_slot;
        let mut var_t3__blk65_dn11: f64 = *var_t3__blk65_dn11_slot;
        let mut var_t3__blk65_dn12: f64 = *var_t3__blk65_dn12_slot;
        let mut var_t3__blk65_dn17: f64 = *var_t3__blk65_dn17_slot;
        let mut var_t3__blk65_dn2: f64 = *var_t3__blk65_dn2_slot;
        let mut var_t3__blk65_dn6: f64 = *var_t3__blk65_dn6_slot;
        let mut var_t3__blk65_dn7: f64 = *var_t3__blk65_dn7_slot;
        let mut var_t4__blk66: f64 = *var_t4__blk66_slot;
        let mut var_t4__blk66_dn0: f64 = *var_t4__blk66_dn0_slot;
        let mut var_t4__blk66_dn10: f64 = *var_t4__blk66_dn10_slot;
        let mut var_t4__blk66_dn11: f64 = *var_t4__blk66_dn11_slot;
        let mut var_t4__blk66_dn12: f64 = *var_t4__blk66_dn12_slot;
        let mut var_t4__blk66_dn17: f64 = *var_t4__blk66_dn17_slot;
        let mut var_t4__blk66_dn2: f64 = *var_t4__blk66_dn2_slot;
        let mut var_t4__blk66_dn6: f64 = *var_t4__blk66_dn6_slot;
        let mut var_t4__blk66_dn7: f64 = *var_t4__blk66_dn7_slot;
        let mut var_t4w: f64 = *var_t4w_slot;
        let mut var_t4w_dn0: f64 = *var_t4w_dn0_slot;
        let mut var_t4w_dn10: f64 = *var_t4w_dn10_slot;
        let mut var_t4w_dn11: f64 = *var_t4w_dn11_slot;
        let mut var_t4w_dn12: f64 = *var_t4w_dn12_slot;
        let mut var_t4w_dn17: f64 = *var_t4w_dn17_slot;
        let mut var_t4w_dn2: f64 = *var_t4w_dn2_slot;
        let mut var_t4w_dn6: f64 = *var_t4w_dn6_slot;
        let mut var_t4w_dn7: f64 = *var_t4w_dn7_slot;
        let mut var_t5__blk68: f64 = *var_t5__blk68_slot;
        let mut var_t5__blk68_dn0: f64 = *var_t5__blk68_dn0_slot;
        let mut var_t5__blk68_dn10: f64 = *var_t5__blk68_dn10_slot;
        let mut var_t5__blk68_dn11: f64 = *var_t5__blk68_dn11_slot;
        let mut var_t5__blk68_dn12: f64 = *var_t5__blk68_dn12_slot;
        let mut var_t5__blk68_dn17: f64 = *var_t5__blk68_dn17_slot;
        let mut var_t5__blk68_dn2: f64 = *var_t5__blk68_dn2_slot;
        let mut var_t5__blk68_dn6: f64 = *var_t5__blk68_dn6_slot;
        let mut var_t5__blk68_dn7: f64 = *var_t5__blk68_dn7_slot;
        let mut var_t6__blk69: f64 = *var_t6__blk69_slot;
        let mut var_t6__blk69_dn0: f64 = *var_t6__blk69_dn0_slot;
        let mut var_t6__blk69_dn10: f64 = *var_t6__blk69_dn10_slot;
        let mut var_t6__blk69_dn11: f64 = *var_t6__blk69_dn11_slot;
        let mut var_t6__blk69_dn12: f64 = *var_t6__blk69_dn12_slot;
        let mut var_t6__blk69_dn17: f64 = *var_t6__blk69_dn17_slot;
        let mut var_t6__blk69_dn2: f64 = *var_t6__blk69_dn2_slot;
        let mut var_t6__blk69_dn6: f64 = *var_t6__blk69_dn6_slot;
        let mut var_t6__blk69_dn7: f64 = *var_t6__blk69_dn7_slot;
        let mut var_tfoxe: f64 = *var_tfoxe_slot;
        let mut var_tfoxe_dn0: f64 = *var_tfoxe_dn0_slot;
        let mut var_tfoxe_dn10: f64 = *var_tfoxe_dn10_slot;
        let mut var_tfoxe_dn11: f64 = *var_tfoxe_dn11_slot;
        let mut var_tfoxe_dn12: f64 = *var_tfoxe_dn12_slot;
        let mut var_tfoxe_dn17: f64 = *var_tfoxe_dn17_slot;
        let mut var_tfoxe_dn2: f64 = *var_tfoxe_dn2_slot;
        let mut var_tfoxe_dn6: f64 = *var_tfoxe_dn6_slot;
        let mut var_tfoxe_dn7: f64 = *var_tfoxe_dn7_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn17: f64 = *var_tmf2_dn17_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_vbsz2: f64 = *var_vbsz2_slot;
        let mut var_vbsz2_dn0: f64 = *var_vbsz2_dn0_slot;
        let mut var_vbsz2_dn10: f64 = *var_vbsz2_dn10_slot;
        let mut var_vbsz2_dn11: f64 = *var_vbsz2_dn11_slot;
        let mut var_vbsz2_dn12: f64 = *var_vbsz2_dn12_slot;
        let mut var_vbsz2_dn17: f64 = *var_vbsz2_dn17_slot;
        let mut var_vbsz2_dn2: f64 = *var_vbsz2_dn2_slot;
        let mut var_vbsz2_dn6: f64 = *var_vbsz2_dn6_slot;
        let mut var_vbsz2_dn7: f64 = *var_vbsz2_dn7_slot;

        let (assign5810_e3764, assign5810_e3764_d_n0, assign5810_e3764_d_n2, assign5810_e3764_d_n6, assign5810_e3764_d_n7, assign5810_e3764_d_n10, assign5810_e3764_d_n11, assign5810_e3764_d_n12, assign5810_e3764_d_n17,) = {
    if (var_guard71 == 0.0) {
        let assign5810_e3758: f64 = (var_vgs - var_vbsp);
        let assign5810_e3760: f64 = (assign5810_e3758 - var_vthq);
        let assign5810_e3762: f64 = (assign5810_e3760 + p.p205);
        (assign5810_e3762, ((-var_vbsp_dn0) - var_vthq_dn0), ((-var_vbsp_dn2) - var_vthq_dn2), ((var_vgs_dn6 - var_vbsp_dn6) - var_vthq_dn6), ((var_vgs_dn7 - var_vbsp_dn7) - var_vthq_dn7), ((-var_vbsp_dn10) - var_vthq_dn10), ((var_vgs_dn11 - var_vbsp_dn11) - var_vthq_dn11), ((-var_vbsp_dn12) - var_vthq_dn12), ((-var_vbsp_dn17) - var_vthq_dn17),)
    } else {
        (var_t5__blk68, var_t5__blk68_dn0, var_t5__blk68_dn2, var_t5__blk68_dn6, var_t5__blk68_dn7, var_t5__blk68_dn10, var_t5__blk68_dn11, var_t5__blk68_dn12, var_t5__blk68_dn17,)
    }
};
        var_t5__blk68 = assign5810_e3764;
        var_t5__blk68_dn0 = assign5810_e3764_d_n0;
        var_t5__blk68_dn2 = assign5810_e3764_d_n2;
        var_t5__blk68_dn6 = assign5810_e3764_d_n6;
        var_t5__blk68_dn7 = assign5810_e3764_d_n7;
        var_t5__blk68_dn10 = assign5810_e3764_d_n10;
        var_t5__blk68_dn11 = assign5810_e3764_d_n11;
        var_t5__blk68_dn12 = assign5810_e3764_d_n12;
        var_t5__blk68_dn17 = assign5810_e3764_d_n17;

        let (assign5820_e3778, assign5820_e3778_d_n0, assign5820_e3778_d_n2, assign5820_e3778_d_n6, assign5820_e3778_d_n7, assign5820_e3778_d_n10, assign5820_e3778_d_n11, assign5820_e3778_d_n12, assign5820_e3778_d_n17,) = {
    if (var_guard71 == 0.0) {
        let assign5820_e3769: f64 = (var_t5__blk68 * var_t5__blk68);
        let assign5820_e3772: f64 = (4.0 * 0.0001);
        let assign5820_e3774: f64 = (assign5820_e3772 * 0.0001);
        let assign5820_e3775: f64 = (assign5820_e3769 + assign5820_e3774);
        let assign5820_e3776: f64 = (assign5820_e3775).sqrt();
        (assign5820_e3776, (((var_t5__blk68_dn0 * var_t5__blk68) + (var_t5__blk68 * var_t5__blk68_dn0)) / (2.0 * assign5820_e3776)), (((var_t5__blk68_dn2 * var_t5__blk68) + (var_t5__blk68 * var_t5__blk68_dn2)) / (2.0 * assign5820_e3776)), (((var_t5__blk68_dn6 * var_t5__blk68) + (var_t5__blk68 * var_t5__blk68_dn6)) / (2.0 * assign5820_e3776)), (((var_t5__blk68_dn7 * var_t5__blk68) + (var_t5__blk68 * var_t5__blk68_dn7)) / (2.0 * assign5820_e3776)), (((var_t5__blk68_dn10 * var_t5__blk68) + (var_t5__blk68 * var_t5__blk68_dn10)) / (2.0 * assign5820_e3776)), (((var_t5__blk68_dn11 * var_t5__blk68) + (var_t5__blk68 * var_t5__blk68_dn11)) / (2.0 * assign5820_e3776)), (((var_t5__blk68_dn12 * var_t5__blk68) + (var_t5__blk68 * var_t5__blk68_dn12)) / (2.0 * assign5820_e3776)), (((var_t5__blk68_dn17 * var_t5__blk68) + (var_t5__blk68 * var_t5__blk68_dn17)) / (2.0 * assign5820_e3776)),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign5820_e3778;
        var_tmf1_dn0 = assign5820_e3778_d_n0;
        var_tmf1_dn2 = assign5820_e3778_d_n2;
        var_tmf1_dn6 = assign5820_e3778_d_n6;
        var_tmf1_dn7 = assign5820_e3778_d_n7;
        var_tmf1_dn10 = assign5820_e3778_d_n10;
        var_tmf1_dn11 = assign5820_e3778_d_n11;
        var_tmf1_dn12 = assign5820_e3778_d_n12;
        var_tmf1_dn17 = assign5820_e3778_d_n17;

        let (assign5830_e3791, assign5830_e3791_d_n0, assign5830_e3791_d_n2, assign5830_e3791_d_n6, assign5830_e3791_d_n7, assign5830_e3791_d_n10, assign5830_e3791_d_n11, assign5830_e3791_d_n12, assign5830_e3791_d_n17,) = {
    if (var_guard71 == 0.0) {
        let assign5830_e3784: f64 = (var_t5__blk68 + var_tmf1);
        let assign5830_e3785: f64 = (0.5 * assign5830_e3784);
        let assign5830_e3788: f64 = (1e-10 * 0.0001);
        let assign5830_e3789: f64 = (assign5830_e3785 + assign5830_e3788);
        (assign5830_e3789, (0.5 * (var_t5__blk68_dn0 + var_tmf1_dn0)), (0.5 * (var_t5__blk68_dn2 + var_tmf1_dn2)), (0.5 * (var_t5__blk68_dn6 + var_tmf1_dn6)), (0.5 * (var_t5__blk68_dn7 + var_tmf1_dn7)), (0.5 * (var_t5__blk68_dn10 + var_tmf1_dn10)), (0.5 * (var_t5__blk68_dn11 + var_tmf1_dn11)), (0.5 * (var_t5__blk68_dn12 + var_tmf1_dn12)), (0.5 * (var_t5__blk68_dn17 + var_tmf1_dn17)),)
    } else {
        (var_t2__blk64, var_t2__blk64_dn0, var_t2__blk64_dn2, var_t2__blk64_dn6, var_t2__blk64_dn7, var_t2__blk64_dn10, var_t2__blk64_dn11, var_t2__blk64_dn12, var_t2__blk64_dn17,)
    }
};
        var_t2__blk64 = assign5830_e3791;
        var_t2__blk64_dn0 = assign5830_e3791_d_n0;
        var_t2__blk64_dn2 = assign5830_e3791_d_n2;
        var_t2__blk64_dn6 = assign5830_e3791_d_n6;
        var_t2__blk64_dn7 = assign5830_e3791_d_n7;
        var_t2__blk64_dn10 = assign5830_e3791_d_n10;
        var_t2__blk64_dn11 = assign5830_e3791_d_n11;
        var_t2__blk64_dn12 = assign5830_e3791_d_n12;
        var_t2__blk64_dn17 = assign5830_e3791_d_n17;

        let assign5840_e3794: f64 = if var_t2__blk64 < 0.0 { 1.0 } else { 0.0 };
        var_guard72 = assign5840_e3794;

        let (assign5850_e3801, assign5850_e3801_d_n0, assign5850_e3801_d_n2, assign5850_e3801_d_n6, assign5850_e3801_d_n7, assign5850_e3801_d_n10, assign5850_e3801_d_n11, assign5850_e3801_d_n12, assign5850_e3801_d_n17,) = {
    if ((var_guard71 == 0.0) && (var_guard72 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2__blk64, var_t2__blk64_dn0, var_t2__blk64_dn2, var_t2__blk64_dn6, var_t2__blk64_dn7, var_t2__blk64_dn10, var_t2__blk64_dn11, var_t2__blk64_dn12, var_t2__blk64_dn17,)
    }
};
        var_t2__blk64 = assign5850_e3801;
        var_t2__blk64_dn0 = assign5850_e3801_d_n0;
        var_t2__blk64_dn2 = assign5850_e3801_d_n2;
        var_t2__blk64_dn6 = assign5850_e3801_d_n6;
        var_t2__blk64_dn7 = assign5850_e3801_d_n7;
        var_t2__blk64_dn10 = assign5850_e3801_d_n10;
        var_t2__blk64_dn11 = assign5850_e3801_d_n11;
        var_t2__blk64_dn12 = assign5850_e3801_d_n12;
        var_t2__blk64_dn17 = assign5850_e3801_d_n17;

        let (assign5860_e3808, assign5860_e3808_d_n0, assign5860_e3808_d_n2, assign5860_e3808_d_n6, assign5860_e3808_d_n7, assign5860_e3808_d_n10, assign5860_e3808_d_n11, assign5860_e3808_d_n12, assign5860_e3808_d_n17,) = {
    if (var_guard71 == 0.0) {
        let assign5860_e3806: f64 = (1.0 / var_t2__blk64);
        (assign5860_e3806, (-(var_t2__blk64_dn0 / (var_t2__blk64 * var_t2__blk64))), (-(var_t2__blk64_dn2 / (var_t2__blk64 * var_t2__blk64))), (-(var_t2__blk64_dn6 / (var_t2__blk64 * var_t2__blk64))), (-(var_t2__blk64_dn7 / (var_t2__blk64 * var_t2__blk64))), (-(var_t2__blk64_dn10 / (var_t2__blk64 * var_t2__blk64))), (-(var_t2__blk64_dn11 / (var_t2__blk64 * var_t2__blk64))), (-(var_t2__blk64_dn12 / (var_t2__blk64 * var_t2__blk64))), (-(var_t2__blk64_dn17 / (var_t2__blk64 * var_t2__blk64))),)
    } else {
        (var_t3__blk65, var_t3__blk65_dn0, var_t3__blk65_dn2, var_t3__blk65_dn6, var_t3__blk65_dn7, var_t3__blk65_dn10, var_t3__blk65_dn11, var_t3__blk65_dn12, var_t3__blk65_dn17,)
    }
};
        var_t3__blk65 = assign5860_e3808;
        var_t3__blk65_dn0 = assign5860_e3808_d_n0;
        var_t3__blk65_dn2 = assign5860_e3808_d_n2;
        var_t3__blk65_dn6 = assign5860_e3808_d_n6;
        var_t3__blk65_dn7 = assign5860_e3808_d_n7;
        var_t3__blk65_dn10 = assign5860_e3808_d_n10;
        var_t3__blk65_dn11 = assign5860_e3808_d_n11;
        var_t3__blk65_dn12 = assign5860_e3808_d_n12;
        var_t3__blk65_dn17 = assign5860_e3808_d_n17;

        let (assign5870_e3816, assign5870_e3816_d_n0, assign5870_e3816_d_n2, assign5870_e3816_d_n6, assign5870_e3816_d_n7, assign5870_e3816_d_n10, assign5870_e3816_d_n11, assign5870_e3816_d_n12, assign5870_e3816_d_n17,) = {
    if (var_guard71 == 0.0) {
        let assign5870_e3813: f64 = (var_vthq).abs();
        let assign5870_e3814: f64 = (2.0 * assign5870_e3813);
        (assign5870_e3814, (2.0 * if var_vthq >= 0.0 { var_vthq_dn0 } else { (-var_vthq_dn0) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn2 } else { (-var_vthq_dn2) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn6 } else { (-var_vthq_dn6) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn7 } else { (-var_vthq_dn7) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn10 } else { (-var_vthq_dn10) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn11 } else { (-var_vthq_dn11) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn12 } else { (-var_vthq_dn12) }), (2.0 * if var_vthq >= 0.0 { var_vthq_dn17 } else { (-var_vthq_dn17) }),)
    } else {
        (var_t4w, var_t4w_dn0, var_t4w_dn2, var_t4w_dn6, var_t4w_dn7, var_t4w_dn10, var_t4w_dn11, var_t4w_dn12, var_t4w_dn17,)
    }
};
        var_t4w = assign5870_e3816;
        var_t4w_dn0 = assign5870_e3816_d_n0;
        var_t4w_dn2 = assign5870_e3816_d_n2;
        var_t4w_dn6 = assign5870_e3816_d_n6;
        var_t4w_dn7 = assign5870_e3816_d_n7;
        var_t4w_dn10 = assign5870_e3816_d_n10;
        var_t4w_dn11 = assign5870_e3816_d_n11;
        var_t4w_dn12 = assign5870_e3816_d_n12;
        var_t4w_dn17 = assign5870_e3816_d_n17;

        let (assign5880_e3825, assign5880_e3825_d_n0, assign5880_e3825_d_n2, assign5880_e3825_d_n6, assign5880_e3825_d_n7, assign5880_e3825_d_n10, assign5880_e3825_d_n11, assign5880_e3825_d_n12, assign5880_e3825_d_n17,) = {
    if (var_guard71 == 0.0) {
        let assign5880_e3821: f64 = (var_vfb - var_vthq);
        let assign5880_e3823: f64 = (assign5880_e3821 + p.p205);
        (assign5880_e3823, (-var_vthq_dn0), (-var_vthq_dn2), (-var_vthq_dn6), (-var_vthq_dn7), (-var_vthq_dn10), (-var_vthq_dn11), (-var_vthq_dn12), (-var_vthq_dn17),)
    } else {
        (var_t6__blk69, var_t6__blk69_dn0, var_t6__blk69_dn2, var_t6__blk69_dn6, var_t6__blk69_dn7, var_t6__blk69_dn10, var_t6__blk69_dn11, var_t6__blk69_dn12, var_t6__blk69_dn17,)
    }
};
        var_t6__blk69 = assign5880_e3825;
        var_t6__blk69_dn0 = assign5880_e3825_d_n0;
        var_t6__blk69_dn2 = assign5880_e3825_d_n2;
        var_t6__blk69_dn6 = assign5880_e3825_d_n6;
        var_t6__blk69_dn7 = assign5880_e3825_d_n7;
        var_t6__blk69_dn10 = assign5880_e3825_d_n10;
        var_t6__blk69_dn11 = assign5880_e3825_d_n11;
        var_t6__blk69_dn12 = assign5880_e3825_d_n12;
        var_t6__blk69_dn17 = assign5880_e3825_d_n17;

        let (assign5890_e3835, assign5890_e3835_d_n0, assign5890_e3835_d_n2, assign5890_e3835_d_n6, assign5890_e3835_d_n7, assign5890_e3835_d_n10, assign5890_e3835_d_n11, assign5890_e3835_d_n12, assign5890_e3835_d_n17,) = {
    if (var_guard71 == 0.0) {
        let (assign5890_e3833, assign5890_e3833_d_n0, assign5890_e3833_d_n2, assign5890_e3833_d_n6, assign5890_e3833_d_n7, assign5890_e3833_d_n10, assign5890_e3833_d_n11, assign5890_e3833_d_n12, assign5890_e3833_d_n17,) = {
            if (var_t6__blk69 > var_t4w) {
                (var_t6__blk69, var_t6__blk69_dn0, var_t6__blk69_dn2, var_t6__blk69_dn6, var_t6__blk69_dn7, var_t6__blk69_dn10, var_t6__blk69_dn11, var_t6__blk69_dn12, var_t6__blk69_dn17,)
            } else {
                (var_t4w, var_t4w_dn0, var_t4w_dn2, var_t4w_dn6, var_t4w_dn7, var_t4w_dn10, var_t4w_dn11, var_t4w_dn12, var_t4w_dn17,)
            }
        };
        (assign5890_e3833, assign5890_e3833_d_n0, assign5890_e3833_d_n2, assign5890_e3833_d_n6, assign5890_e3833_d_n7, assign5890_e3833_d_n10, assign5890_e3833_d_n11, assign5890_e3833_d_n12, assign5890_e3833_d_n17,)
    } else {
        (var_t4__blk66, var_t4__blk66_dn0, var_t4__blk66_dn2, var_t4__blk66_dn6, var_t4__blk66_dn7, var_t4__blk66_dn10, var_t4__blk66_dn11, var_t4__blk66_dn12, var_t4__blk66_dn17,)
    }
};
        var_t4__blk66 = assign5890_e3835;
        var_t4__blk66_dn0 = assign5890_e3835_d_n0;
        var_t4__blk66_dn2 = assign5890_e3835_d_n2;
        var_t4__blk66_dn6 = assign5890_e3835_d_n6;
        var_t4__blk66_dn7 = assign5890_e3835_d_n7;
        var_t4__blk66_dn10 = assign5890_e3835_d_n10;
        var_t4__blk66_dn11 = assign5890_e3835_d_n11;
        var_t4__blk66_dn12 = assign5890_e3835_d_n12;
        var_t4__blk66_dn17 = assign5890_e3835_d_n17;

        let (assign5900_e3846, assign5900_e3846_d_n0, assign5900_e3846_d_n2, assign5900_e3846_d_n6, assign5900_e3846_d_n7, assign5900_e3846_d_n10, assign5900_e3846_d_n11, assign5900_e3846_d_n12, assign5900_e3846_d_n17,) = {
    if (var_guard71 == 0.0) {
        let assign5900_e3840: f64 = (1.0 / var_t4__blk66);
        let assign5900_e3842: f64 = (assign5900_e3840 - var_t3__blk65);
        let assign5900_e3844: f64 = (assign5900_e3842 - 0.0001);
        (assign5900_e3844, ((-(var_t4__blk66_dn0 / (var_t4__blk66 * var_t4__blk66))) - var_t3__blk65_dn0), ((-(var_t4__blk66_dn2 / (var_t4__blk66 * var_t4__blk66))) - var_t3__blk65_dn2), ((-(var_t4__blk66_dn6 / (var_t4__blk66 * var_t4__blk66))) - var_t3__blk65_dn6), ((-(var_t4__blk66_dn7 / (var_t4__blk66 * var_t4__blk66))) - var_t3__blk65_dn7), ((-(var_t4__blk66_dn10 / (var_t4__blk66 * var_t4__blk66))) - var_t3__blk65_dn10), ((-(var_t4__blk66_dn11 / (var_t4__blk66 * var_t4__blk66))) - var_t3__blk65_dn11), ((-(var_t4__blk66_dn12 / (var_t4__blk66 * var_t4__blk66))) - var_t3__blk65_dn12), ((-(var_t4__blk66_dn17 / (var_t4__blk66 * var_t4__blk66))) - var_t3__blk65_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign5900_e3846;
        var_tmf1_dn0 = assign5900_e3846_d_n0;
        var_tmf1_dn2 = assign5900_e3846_d_n2;
        var_tmf1_dn6 = assign5900_e3846_d_n6;
        var_tmf1_dn7 = assign5900_e3846_d_n7;
        var_tmf1_dn10 = assign5900_e3846_d_n10;
        var_tmf1_dn11 = assign5900_e3846_d_n11;
        var_tmf1_dn12 = assign5900_e3846_d_n12;
        var_tmf1_dn17 = assign5900_e3846_d_n17;

        let (assign5910_e3857, assign5910_e3857_d_n0, assign5910_e3857_d_n2, assign5910_e3857_d_n6, assign5910_e3857_d_n7, assign5910_e3857_d_n10, assign5910_e3857_d_n11, assign5910_e3857_d_n12, assign5910_e3857_d_n17,) = {
    if (var_guard71 == 0.0) {
        let assign5910_e3852: f64 = (1.0 / var_t4__blk66);
        let assign5910_e3853: f64 = (4.0 * assign5910_e3852);
        let assign5910_e3855: f64 = (assign5910_e3853 * 0.0001);
        (assign5910_e3855, ((4.0 * (-(var_t4__blk66_dn0 / (var_t4__blk66 * var_t4__blk66)))) * 0.0001), ((4.0 * (-(var_t4__blk66_dn2 / (var_t4__blk66 * var_t4__blk66)))) * 0.0001), ((4.0 * (-(var_t4__blk66_dn6 / (var_t4__blk66 * var_t4__blk66)))) * 0.0001), ((4.0 * (-(var_t4__blk66_dn7 / (var_t4__blk66 * var_t4__blk66)))) * 0.0001), ((4.0 * (-(var_t4__blk66_dn10 / (var_t4__blk66 * var_t4__blk66)))) * 0.0001), ((4.0 * (-(var_t4__blk66_dn11 / (var_t4__blk66 * var_t4__blk66)))) * 0.0001), ((4.0 * (-(var_t4__blk66_dn12 / (var_t4__blk66 * var_t4__blk66)))) * 0.0001), ((4.0 * (-(var_t4__blk66_dn17 / (var_t4__blk66 * var_t4__blk66)))) * 0.0001),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign5910_e3857;
        var_tmf2_dn0 = assign5910_e3857_d_n0;
        var_tmf2_dn2 = assign5910_e3857_d_n2;
        var_tmf2_dn6 = assign5910_e3857_d_n6;
        var_tmf2_dn7 = assign5910_e3857_d_n7;
        var_tmf2_dn10 = assign5910_e3857_d_n10;
        var_tmf2_dn11 = assign5910_e3857_d_n11;
        var_tmf2_dn12 = assign5910_e3857_d_n12;
        var_tmf2_dn17 = assign5910_e3857_d_n17;

        let (assign5920_e3868, assign5920_e3868_d_n0, assign5920_e3868_d_n2, assign5920_e3868_d_n6, assign5920_e3868_d_n7, assign5920_e3868_d_n10, assign5920_e3868_d_n11, assign5920_e3868_d_n12, assign5920_e3868_d_n17,) = {
    if (var_guard71 == 0.0) {
        let (assign5920_e3866, assign5920_e3866_d_n0, assign5920_e3866_d_n2, assign5920_e3866_d_n6, assign5920_e3866_d_n7, assign5920_e3866_d_n10, assign5920_e3866_d_n11, assign5920_e3866_d_n12, assign5920_e3866_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign5920_e3865: f64 = (-var_tmf2);
                (assign5920_e3865, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign5920_e3866, assign5920_e3866_d_n0, assign5920_e3866_d_n2, assign5920_e3866_d_n6, assign5920_e3866_d_n7, assign5920_e3866_d_n10, assign5920_e3866_d_n11, assign5920_e3866_d_n12, assign5920_e3866_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign5920_e3868;
        var_tmf2_dn0 = assign5920_e3868_d_n0;
        var_tmf2_dn2 = assign5920_e3868_d_n2;
        var_tmf2_dn6 = assign5920_e3868_d_n6;
        var_tmf2_dn7 = assign5920_e3868_d_n7;
        var_tmf2_dn10 = assign5920_e3868_d_n10;
        var_tmf2_dn11 = assign5920_e3868_d_n11;
        var_tmf2_dn12 = assign5920_e3868_d_n12;
        var_tmf2_dn17 = assign5920_e3868_d_n17;

        let (assign5930_e3878, assign5930_e3878_d_n0, assign5930_e3878_d_n2, assign5930_e3878_d_n6, assign5930_e3878_d_n7, assign5930_e3878_d_n10, assign5930_e3878_d_n11, assign5930_e3878_d_n12, assign5930_e3878_d_n17,) = {
    if (var_guard71 == 0.0) {
        let assign5930_e3873: f64 = (var_tmf1 * var_tmf1);
        let assign5930_e3875: f64 = (assign5930_e3873 + var_tmf2);
        let assign5930_e3876: f64 = (assign5930_e3875).sqrt();
        (assign5930_e3876, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign5930_e3876)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign5930_e3876)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign5930_e3876)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign5930_e3876)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign5930_e3876)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign5930_e3876)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign5930_e3876)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign5930_e3876)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign5930_e3878;
        var_tmf2_dn0 = assign5930_e3878_d_n0;
        var_tmf2_dn2 = assign5930_e3878_d_n2;
        var_tmf2_dn6 = assign5930_e3878_d_n6;
        var_tmf2_dn7 = assign5930_e3878_d_n7;
        var_tmf2_dn10 = assign5930_e3878_d_n10;
        var_tmf2_dn11 = assign5930_e3878_d_n11;
        var_tmf2_dn12 = assign5930_e3878_d_n12;
        var_tmf2_dn17 = assign5930_e3878_d_n17;

        let (assign5940_e3891, assign5940_e3891_d_n0, assign5940_e3891_d_n2, assign5940_e3891_d_n6, assign5940_e3891_d_n7, assign5940_e3891_d_n10, assign5940_e3891_d_n11, assign5940_e3891_d_n12, assign5940_e3891_d_n17,) = {
    if (var_guard71 == 0.0) {
        let assign5940_e3883: f64 = (1.0 / var_t4__blk66);
        let assign5940_e3887: f64 = (var_tmf1 + var_tmf2);
        let assign5940_e3888: f64 = (0.5 * assign5940_e3887);
        let assign5940_e3889: f64 = (assign5940_e3883 - assign5940_e3888);
        (assign5940_e3889, ((-(var_t4__blk66_dn0 / (var_t4__blk66 * var_t4__blk66))) - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), ((-(var_t4__blk66_dn2 / (var_t4__blk66 * var_t4__blk66))) - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), ((-(var_t4__blk66_dn6 / (var_t4__blk66 * var_t4__blk66))) - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), ((-(var_t4__blk66_dn7 / (var_t4__blk66 * var_t4__blk66))) - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), ((-(var_t4__blk66_dn10 / (var_t4__blk66 * var_t4__blk66))) - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), ((-(var_t4__blk66_dn11 / (var_t4__blk66 * var_t4__blk66))) - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), ((-(var_t4__blk66_dn12 / (var_t4__blk66 * var_t4__blk66))) - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), ((-(var_t4__blk66_dn17 / (var_t4__blk66 * var_t4__blk66))) - (0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_t2__blk64, var_t2__blk64_dn0, var_t2__blk64_dn2, var_t2__blk64_dn6, var_t2__blk64_dn7, var_t2__blk64_dn10, var_t2__blk64_dn11, var_t2__blk64_dn12, var_t2__blk64_dn17,)
    }
};
        var_t2__blk64 = assign5940_e3891;
        var_t2__blk64_dn0 = assign5940_e3891_d_n0;
        var_t2__blk64_dn2 = assign5940_e3891_d_n2;
        var_t2__blk64_dn6 = assign5940_e3891_d_n6;
        var_t2__blk64_dn7 = assign5940_e3891_d_n7;
        var_t2__blk64_dn10 = assign5940_e3891_d_n10;
        var_t2__blk64_dn11 = assign5940_e3891_d_n11;
        var_t2__blk64_dn12 = assign5940_e3891_d_n12;
        var_t2__blk64_dn17 = assign5940_e3891_d_n17;

        let (assign5950_e3900, assign5950_e3900_d_n0, assign5950_e3900_d_n2, assign5950_e3900_d_n6, assign5950_e3900_d_n7, assign5950_e3900_d_n10, assign5950_e3900_d_n11, assign5950_e3900_d_n12, assign5950_e3900_d_n17,) = {
    if (var_guard71 == 0.0) {
        let assign5950_e3896: f64 = (p.p204 * var_t2__blk64);
        let assign5950_e3898: f64 = (assign5950_e3896 + p.p206);
        (assign5950_e3898, (p.p204 * var_t2__blk64_dn0), (p.p204 * var_t2__blk64_dn2), (p.p204 * var_t2__blk64_dn6), (p.p204 * var_t2__blk64_dn7), (p.p204 * var_t2__blk64_dn10), (p.p204 * var_t2__blk64_dn11), (p.p204 * var_t2__blk64_dn12), (p.p204 * var_t2__blk64_dn17),)
    } else {
        (var_dtfox, var_dtfox_dn0, var_dtfox_dn2, var_dtfox_dn6, var_dtfox_dn7, var_dtfox_dn10, var_dtfox_dn11, var_dtfox_dn12, var_dtfox_dn17,)
    }
};
        var_dtfox = assign5950_e3900;
        var_dtfox_dn0 = assign5950_e3900_d_n0;
        var_dtfox_dn2 = assign5950_e3900_d_n2;
        var_dtfox_dn6 = assign5950_e3900_d_n6;
        var_dtfox_dn7 = assign5950_e3900_d_n7;
        var_dtfox_dn10 = assign5950_e3900_d_n10;
        var_dtfox_dn11 = assign5950_e3900_d_n11;
        var_dtfox_dn12 = assign5950_e3900_d_n12;
        var_dtfox_dn17 = assign5950_e3900_d_n17;

        let assign5960_e3903: f64 = (var_dtfox * 1000000000000.0);
        let assign5960_e3905: f64 = if assign5960_e3903 < var_tfox0 { 1.0 } else { 0.0 };
        var_guard73 = assign5960_e3905;

        let (assign5970_e3912, assign5970_e3912_d_n0, assign5970_e3912_d_n2, assign5970_e3912_d_n6, assign5970_e3912_d_n7, assign5970_e3912_d_n10, assign5970_e3912_d_n11, assign5970_e3912_d_n12, assign5970_e3912_d_n17,) = {
    if ((var_guard71 == 0.0) && (var_guard73 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dtfox, var_dtfox_dn0, var_dtfox_dn2, var_dtfox_dn6, var_dtfox_dn7, var_dtfox_dn10, var_dtfox_dn11, var_dtfox_dn12, var_dtfox_dn17,)
    }
};
        var_dtfox = assign5970_e3912;
        var_dtfox_dn0 = assign5970_e3912_d_n0;
        var_dtfox_dn2 = assign5970_e3912_d_n2;
        var_dtfox_dn6 = assign5970_e3912_d_n6;
        var_dtfox_dn7 = assign5970_e3912_d_n7;
        var_dtfox_dn10 = assign5970_e3912_d_n10;
        var_dtfox_dn11 = assign5970_e3912_d_n11;
        var_dtfox_dn12 = assign5970_e3912_d_n12;
        var_dtfox_dn17 = assign5970_e3912_d_n17;

        let (assign5980_e3919,) = {
    if ((var_guard71 == 0.0) && (var_guard73 != 0.0)) {
        (0.0,)
    } else {
        (var_flg_qme,)
    }
};
        var_flg_qme = assign5980_e3919;

        let (assign5990_e3926, assign5990_e3926_d_n0, assign5990_e3926_d_n2, assign5990_e3926_d_n6, assign5990_e3926_d_n7, assign5990_e3926_d_n10, assign5990_e3926_d_n11, assign5990_e3926_d_n12, assign5990_e3926_d_n17,) = {
    if (var_guard71 == 0.0) {
        let assign5990_e3924: f64 = (var_tfox0 + var_dtfox);
        (assign5990_e3924, var_dtfox_dn0, var_dtfox_dn2, var_dtfox_dn6, var_dtfox_dn7, var_dtfox_dn10, var_dtfox_dn11, var_dtfox_dn12, var_dtfox_dn17,)
    } else {
        (var_tfoxe, var_tfoxe_dn0, var_tfoxe_dn2, var_tfoxe_dn6, var_tfoxe_dn7, var_tfoxe_dn10, var_tfoxe_dn11, var_tfoxe_dn12, var_tfoxe_dn17,)
    }
};
        var_tfoxe = assign5990_e3926;
        var_tfoxe_dn0 = assign5990_e3926_d_n0;
        var_tfoxe_dn2 = assign5990_e3926_d_n2;
        var_tfoxe_dn6 = assign5990_e3926_d_n6;
        var_tfoxe_dn7 = assign5990_e3926_d_n7;
        var_tfoxe_dn10 = assign5990_e3926_d_n10;
        var_tfoxe_dn11 = assign5990_e3926_d_n11;
        var_tfoxe_dn12 = assign5990_e3926_d_n12;
        var_tfoxe_dn17 = assign5990_e3926_d_n17;

        let (assign6000_e3933, assign6000_e3933_d_n0, assign6000_e3933_d_n2, assign6000_e3933_d_n6, assign6000_e3933_d_n7, assign6000_e3933_d_n10, assign6000_e3933_d_n11, assign6000_e3933_d_n12, assign6000_e3933_d_n17,) = {
    if (var_guard71 == 0.0) {
        let assign6000_e3931: f64 = (3.453133e-11 / var_tfoxe);
        (assign6000_e3931, (-((3.453133e-11 * var_tfoxe_dn0) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn2) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn6) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn7) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn10) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn11) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn12) / (var_tfoxe * var_tfoxe))), (-((3.453133e-11 * var_tfoxe_dn17) / (var_tfoxe * var_tfoxe))),)
    } else {
        (var_c_fox, var_c_fox_dn0, var_c_fox_dn2, var_c_fox_dn6, var_c_fox_dn7, var_c_fox_dn10, var_c_fox_dn11, var_c_fox_dn12, var_c_fox_dn17,)
    }
};
        var_c_fox = assign6000_e3933;
        var_c_fox_dn0 = assign6000_e3933_d_n0;
        var_c_fox_dn2 = assign6000_e3933_d_n2;
        var_c_fox_dn6 = assign6000_e3933_d_n6;
        var_c_fox_dn7 = assign6000_e3933_d_n7;
        var_c_fox_dn10 = assign6000_e3933_d_n10;
        var_c_fox_dn11 = assign6000_e3933_d_n11;
        var_c_fox_dn12 = assign6000_e3933_d_n12;
        var_c_fox_dn17 = assign6000_e3933_d_n17;

        let (assign6010_e3940, assign6010_e3940_d_n0, assign6010_e3940_d_n2, assign6010_e3940_d_n6, assign6010_e3940_d_n7, assign6010_e3940_d_n10, assign6010_e3940_d_n11, assign6010_e3940_d_n12, assign6010_e3940_d_n17,) = {
    if (var_guard71 == 0.0) {
        let assign6010_e3938: f64 = (var_tfoxe / 3.453133e-11);
        (assign6010_e3938, (var_tfoxe_dn0 / 3.453133e-11), (var_tfoxe_dn2 / 3.453133e-11), (var_tfoxe_dn6 / 3.453133e-11), (var_tfoxe_dn7 / 3.453133e-11), (var_tfoxe_dn10 / 3.453133e-11), (var_tfoxe_dn11 / 3.453133e-11), (var_tfoxe_dn12 / 3.453133e-11), (var_tfoxe_dn17 / 3.453133e-11),)
    } else {
        (var_c_fox_inv, var_c_fox_inv_dn0, var_c_fox_inv_dn2, var_c_fox_inv_dn6, var_c_fox_inv_dn7, var_c_fox_inv_dn10, var_c_fox_inv_dn11, var_c_fox_inv_dn12, var_c_fox_inv_dn17,)
    }
};
        var_c_fox_inv = assign6010_e3940;
        var_c_fox_inv_dn0 = assign6010_e3940_d_n0;
        var_c_fox_inv_dn2 = assign6010_e3940_d_n2;
        var_c_fox_inv_dn6 = assign6010_e3940_d_n6;
        var_c_fox_inv_dn7 = assign6010_e3940_d_n7;
        var_c_fox_inv_dn10 = assign6010_e3940_d_n10;
        var_c_fox_inv_dn11 = assign6010_e3940_d_n11;
        var_c_fox_inv_dn12 = assign6010_e3940_d_n12;
        var_c_fox_inv_dn17 = assign6010_e3940_d_n17;

        let (assign6020_e3951, assign6020_e3951_d_n0, assign6020_e3951_d_n2, assign6020_e3951_d_n6, assign6020_e3951_d_n7, assign6020_e3951_d_n10, assign6020_e3951_d_n11, assign6020_e3951_d_n12, assign6020_e3951_d_n17,) = {
    if (var_guard71 == 0.0) {
        let assign6020_e3945: f64 = (var_cnst0soi * var_cnst0soi);
        let assign6020_e3947: f64 = (assign6020_e3945 * var_c_fox_inv);
        let assign6020_e3949: f64 = (assign6020_e3947 * var_c_fox_inv);
        (assign6020_e3949, ((((((var_cnst0soi_dn0 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn0)) * var_c_fox_inv) + (assign6020_e3945 * var_c_fox_inv_dn0)) * var_c_fox_inv) + (assign6020_e3947 * var_c_fox_inv_dn0)), ((((((var_cnst0soi_dn2 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn2)) * var_c_fox_inv) + (assign6020_e3945 * var_c_fox_inv_dn2)) * var_c_fox_inv) + (assign6020_e3947 * var_c_fox_inv_dn2)), ((((((var_cnst0soi_dn6 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn6)) * var_c_fox_inv) + (assign6020_e3945 * var_c_fox_inv_dn6)) * var_c_fox_inv) + (assign6020_e3947 * var_c_fox_inv_dn6)), ((((((var_cnst0soi_dn7 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn7)) * var_c_fox_inv) + (assign6020_e3945 * var_c_fox_inv_dn7)) * var_c_fox_inv) + (assign6020_e3947 * var_c_fox_inv_dn7)), ((((((var_cnst0soi_dn10 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn10)) * var_c_fox_inv) + (assign6020_e3945 * var_c_fox_inv_dn10)) * var_c_fox_inv) + (assign6020_e3947 * var_c_fox_inv_dn10)), ((((((var_cnst0soi_dn11 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn11)) * var_c_fox_inv) + (assign6020_e3945 * var_c_fox_inv_dn11)) * var_c_fox_inv) + (assign6020_e3947 * var_c_fox_inv_dn11)), ((((((var_cnst0soi_dn12 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn12)) * var_c_fox_inv) + (assign6020_e3945 * var_c_fox_inv_dn12)) * var_c_fox_inv) + (assign6020_e3947 * var_c_fox_inv_dn12)), ((((((var_cnst0soi_dn17 * var_cnst0soi) + (var_cnst0soi * var_cnst0soi_dn17)) * var_c_fox_inv) + (assign6020_e3945 * var_c_fox_inv_dn17)) * var_c_fox_inv) + (assign6020_e3947 * var_c_fox_inv_dn17)),)
    } else {
        (var_cnstc_foxi, var_cnstc_foxi_dn0, var_cnstc_foxi_dn2, var_cnstc_foxi_dn6, var_cnstc_foxi_dn7, var_cnstc_foxi_dn10, var_cnstc_foxi_dn11, var_cnstc_foxi_dn12, var_cnstc_foxi_dn17,)
    }
};
        var_cnstc_foxi = assign6020_e3951;
        var_cnstc_foxi_dn0 = assign6020_e3951_d_n0;
        var_cnstc_foxi_dn2 = assign6020_e3951_d_n2;
        var_cnstc_foxi_dn6 = assign6020_e3951_d_n6;
        var_cnstc_foxi_dn7 = assign6020_e3951_d_n7;
        var_cnstc_foxi_dn10 = assign6020_e3951_d_n10;
        var_cnstc_foxi_dn11 = assign6020_e3951_d_n11;
        var_cnstc_foxi_dn12 = assign6020_e3951_d_n12;
        var_cnstc_foxi_dn17 = assign6020_e3951_d_n17;

        let assign6030_e3958: f64 = if ((p.p43 == 1.0) || (var_subversion < 3.0)) { 1.0 } else { 0.0 };
        var_guard74 = assign6030_e3958;

        let (assign6040_e3966, assign6040_e3966_d_n0, assign6040_e3966_d_n2, assign6040_e3966_d_n6, assign6040_e3966_d_n7, assign6040_e3966_d_n10, assign6040_e3966_d_n11, assign6040_e3966_d_n12, assign6040_e3966_d_n17,) = {
    if (var_guard74 != 0.0) {
        let assign6040_e3962: f64 = (0.5 - var_vbspz);
        let assign6040_e3964: f64 = (assign6040_e3962 - 0.001);
        (assign6040_e3964, (-var_vbspz_dn0), (-var_vbspz_dn2), (-var_vbspz_dn6), (-var_vbspz_dn7), (-var_vbspz_dn10), (-var_vbspz_dn11), (-var_vbspz_dn12), (-var_vbspz_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign6040_e3966;
        var_tmf1_dn0 = assign6040_e3966_d_n0;
        var_tmf1_dn2 = assign6040_e3966_d_n2;
        var_tmf1_dn6 = assign6040_e3966_d_n6;
        var_tmf1_dn7 = assign6040_e3966_d_n7;
        var_tmf1_dn10 = assign6040_e3966_d_n10;
        var_tmf1_dn11 = assign6040_e3966_d_n11;
        var_tmf1_dn12 = assign6040_e3966_d_n12;
        var_tmf1_dn17 = assign6040_e3966_d_n17;

        let (assign6050_e3974, assign6050_e3974_d_n0, assign6050_e3974_d_n2, assign6050_e3974_d_n6, assign6050_e3974_d_n7, assign6050_e3974_d_n10, assign6050_e3974_d_n11, assign6050_e3974_d_n12, assign6050_e3974_d_n17,) = {
    if (var_guard74 != 0.0) {
        let assign6050_e3970: f64 = (4.0 * 0.5);
        let assign6050_e3972: f64 = (assign6050_e3970 * 0.001);
        (assign6050_e3972, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6050_e3974;
        var_tmf2_dn0 = assign6050_e3974_d_n0;
        var_tmf2_dn2 = assign6050_e3974_d_n2;
        var_tmf2_dn6 = assign6050_e3974_d_n6;
        var_tmf2_dn7 = assign6050_e3974_d_n7;
        var_tmf2_dn10 = assign6050_e3974_d_n10;
        var_tmf2_dn11 = assign6050_e3974_d_n11;
        var_tmf2_dn12 = assign6050_e3974_d_n12;
        var_tmf2_dn17 = assign6050_e3974_d_n17;

        let (assign6060_e3984, assign6060_e3984_d_n0, assign6060_e3984_d_n2, assign6060_e3984_d_n6, assign6060_e3984_d_n7, assign6060_e3984_d_n10, assign6060_e3984_d_n11, assign6060_e3984_d_n12, assign6060_e3984_d_n17,) = {
    if (var_guard74 != 0.0) {
        let (assign6060_e3982, assign6060_e3982_d_n0, assign6060_e3982_d_n2, assign6060_e3982_d_n6, assign6060_e3982_d_n7, assign6060_e3982_d_n10, assign6060_e3982_d_n11, assign6060_e3982_d_n12, assign6060_e3982_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign6060_e3981: f64 = (-var_tmf2);
                (assign6060_e3981, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign6060_e3982, assign6060_e3982_d_n0, assign6060_e3982_d_n2, assign6060_e3982_d_n6, assign6060_e3982_d_n7, assign6060_e3982_d_n10, assign6060_e3982_d_n11, assign6060_e3982_d_n12, assign6060_e3982_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6060_e3984;
        var_tmf2_dn0 = assign6060_e3984_d_n0;
        var_tmf2_dn2 = assign6060_e3984_d_n2;
        var_tmf2_dn6 = assign6060_e3984_d_n6;
        var_tmf2_dn7 = assign6060_e3984_d_n7;
        var_tmf2_dn10 = assign6060_e3984_d_n10;
        var_tmf2_dn11 = assign6060_e3984_d_n11;
        var_tmf2_dn12 = assign6060_e3984_d_n12;
        var_tmf2_dn17 = assign6060_e3984_d_n17;

        let (assign6070_e3993, assign6070_e3993_d_n0, assign6070_e3993_d_n2, assign6070_e3993_d_n6, assign6070_e3993_d_n7, assign6070_e3993_d_n10, assign6070_e3993_d_n11, assign6070_e3993_d_n12, assign6070_e3993_d_n17,) = {
    if (var_guard74 != 0.0) {
        let assign6070_e3988: f64 = (var_tmf1 * var_tmf1);
        let assign6070_e3990: f64 = (assign6070_e3988 + var_tmf2);
        let assign6070_e3991: f64 = (assign6070_e3990).sqrt();
        (assign6070_e3991, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign6070_e3991)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign6070_e3991)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign6070_e3991)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign6070_e3991)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign6070_e3991)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign6070_e3991)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign6070_e3991)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign6070_e3991)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6070_e3993;
        var_tmf2_dn0 = assign6070_e3993_d_n0;
        var_tmf2_dn2 = assign6070_e3993_d_n2;
        var_tmf2_dn6 = assign6070_e3993_d_n6;
        var_tmf2_dn7 = assign6070_e3993_d_n7;
        var_tmf2_dn10 = assign6070_e3993_d_n10;
        var_tmf2_dn11 = assign6070_e3993_d_n11;
        var_tmf2_dn12 = assign6070_e3993_d_n12;
        var_tmf2_dn17 = assign6070_e3993_d_n17;

        let (assign6080_e4003, assign6080_e4003_d_n0, assign6080_e4003_d_n2, assign6080_e4003_d_n6, assign6080_e4003_d_n7, assign6080_e4003_d_n10, assign6080_e4003_d_n11, assign6080_e4003_d_n12, assign6080_e4003_d_n17,) = {
    if (var_guard74 != 0.0) {
        let assign6080_e3999: f64 = (var_tmf1 + var_tmf2);
        let assign6080_e4000: f64 = (0.5 * assign6080_e3999);
        let assign6080_e4001: f64 = (0.5 - assign6080_e4000);
        (assign6080_e4001, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (-(0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (-(0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (-(0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (-(0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), (-(0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_vbsz2, var_vbsz2_dn0, var_vbsz2_dn2, var_vbsz2_dn6, var_vbsz2_dn7, var_vbsz2_dn10, var_vbsz2_dn11, var_vbsz2_dn12, var_vbsz2_dn17,)
    }
};
        var_vbsz2 = assign6080_e4003;
        var_vbsz2_dn0 = assign6080_e4003_d_n0;
        var_vbsz2_dn2 = assign6080_e4003_d_n2;
        var_vbsz2_dn6 = assign6080_e4003_d_n6;
        var_vbsz2_dn7 = assign6080_e4003_d_n7;
        var_vbsz2_dn10 = assign6080_e4003_d_n10;
        var_vbsz2_dn11 = assign6080_e4003_d_n11;
        var_vbsz2_dn12 = assign6080_e4003_d_n12;
        var_vbsz2_dn17 = assign6080_e4003_d_n17;

        *var_c_fox_slot = var_c_fox;
        *var_c_fox_dn0_slot = var_c_fox_dn0;
        *var_c_fox_dn10_slot = var_c_fox_dn10;
        *var_c_fox_dn11_slot = var_c_fox_dn11;
        *var_c_fox_dn12_slot = var_c_fox_dn12;
        *var_c_fox_dn17_slot = var_c_fox_dn17;
        *var_c_fox_dn2_slot = var_c_fox_dn2;
        *var_c_fox_dn6_slot = var_c_fox_dn6;
        *var_c_fox_dn7_slot = var_c_fox_dn7;
        *var_c_fox_inv_slot = var_c_fox_inv;
        *var_c_fox_inv_dn0_slot = var_c_fox_inv_dn0;
        *var_c_fox_inv_dn10_slot = var_c_fox_inv_dn10;
        *var_c_fox_inv_dn11_slot = var_c_fox_inv_dn11;
        *var_c_fox_inv_dn12_slot = var_c_fox_inv_dn12;
        *var_c_fox_inv_dn17_slot = var_c_fox_inv_dn17;
        *var_c_fox_inv_dn2_slot = var_c_fox_inv_dn2;
        *var_c_fox_inv_dn6_slot = var_c_fox_inv_dn6;
        *var_c_fox_inv_dn7_slot = var_c_fox_inv_dn7;
        *var_cnstc_foxi_slot = var_cnstc_foxi;
        *var_cnstc_foxi_dn0_slot = var_cnstc_foxi_dn0;
        *var_cnstc_foxi_dn10_slot = var_cnstc_foxi_dn10;
        *var_cnstc_foxi_dn11_slot = var_cnstc_foxi_dn11;
        *var_cnstc_foxi_dn12_slot = var_cnstc_foxi_dn12;
        *var_cnstc_foxi_dn17_slot = var_cnstc_foxi_dn17;
        *var_cnstc_foxi_dn2_slot = var_cnstc_foxi_dn2;
        *var_cnstc_foxi_dn6_slot = var_cnstc_foxi_dn6;
        *var_cnstc_foxi_dn7_slot = var_cnstc_foxi_dn7;
        *var_dtfox_slot = var_dtfox;
        *var_dtfox_dn0_slot = var_dtfox_dn0;
        *var_dtfox_dn10_slot = var_dtfox_dn10;
        *var_dtfox_dn11_slot = var_dtfox_dn11;
        *var_dtfox_dn12_slot = var_dtfox_dn12;
        *var_dtfox_dn17_slot = var_dtfox_dn17;
        *var_dtfox_dn2_slot = var_dtfox_dn2;
        *var_dtfox_dn6_slot = var_dtfox_dn6;
        *var_dtfox_dn7_slot = var_dtfox_dn7;
        *var_flg_qme_slot = var_flg_qme;
        *var_guard72_slot = var_guard72;
        *var_guard73_slot = var_guard73;
        *var_guard74_slot = var_guard74;
        *var_t2__blk64_slot = var_t2__blk64;
        *var_t2__blk64_dn0_slot = var_t2__blk64_dn0;
        *var_t2__blk64_dn10_slot = var_t2__blk64_dn10;
        *var_t2__blk64_dn11_slot = var_t2__blk64_dn11;
        *var_t2__blk64_dn12_slot = var_t2__blk64_dn12;
        *var_t2__blk64_dn17_slot = var_t2__blk64_dn17;
        *var_t2__blk64_dn2_slot = var_t2__blk64_dn2;
        *var_t2__blk64_dn6_slot = var_t2__blk64_dn6;
        *var_t2__blk64_dn7_slot = var_t2__blk64_dn7;
        *var_t3__blk65_slot = var_t3__blk65;
        *var_t3__blk65_dn0_slot = var_t3__blk65_dn0;
        *var_t3__blk65_dn10_slot = var_t3__blk65_dn10;
        *var_t3__blk65_dn11_slot = var_t3__blk65_dn11;
        *var_t3__blk65_dn12_slot = var_t3__blk65_dn12;
        *var_t3__blk65_dn17_slot = var_t3__blk65_dn17;
        *var_t3__blk65_dn2_slot = var_t3__blk65_dn2;
        *var_t3__blk65_dn6_slot = var_t3__blk65_dn6;
        *var_t3__blk65_dn7_slot = var_t3__blk65_dn7;
        *var_t4__blk66_slot = var_t4__blk66;
        *var_t4__blk66_dn0_slot = var_t4__blk66_dn0;
        *var_t4__blk66_dn10_slot = var_t4__blk66_dn10;
        *var_t4__blk66_dn11_slot = var_t4__blk66_dn11;
        *var_t4__blk66_dn12_slot = var_t4__blk66_dn12;
        *var_t4__blk66_dn17_slot = var_t4__blk66_dn17;
        *var_t4__blk66_dn2_slot = var_t4__blk66_dn2;
        *var_t4__blk66_dn6_slot = var_t4__blk66_dn6;
        *var_t4__blk66_dn7_slot = var_t4__blk66_dn7;
        *var_t4w_slot = var_t4w;
        *var_t4w_dn0_slot = var_t4w_dn0;
        *var_t4w_dn10_slot = var_t4w_dn10;
        *var_t4w_dn11_slot = var_t4w_dn11;
        *var_t4w_dn12_slot = var_t4w_dn12;
        *var_t4w_dn17_slot = var_t4w_dn17;
        *var_t4w_dn2_slot = var_t4w_dn2;
        *var_t4w_dn6_slot = var_t4w_dn6;
        *var_t4w_dn7_slot = var_t4w_dn7;
        *var_t5__blk68_slot = var_t5__blk68;
        *var_t5__blk68_dn0_slot = var_t5__blk68_dn0;
        *var_t5__blk68_dn10_slot = var_t5__blk68_dn10;
        *var_t5__blk68_dn11_slot = var_t5__blk68_dn11;
        *var_t5__blk68_dn12_slot = var_t5__blk68_dn12;
        *var_t5__blk68_dn17_slot = var_t5__blk68_dn17;
        *var_t5__blk68_dn2_slot = var_t5__blk68_dn2;
        *var_t5__blk68_dn6_slot = var_t5__blk68_dn6;
        *var_t5__blk68_dn7_slot = var_t5__blk68_dn7;
        *var_t6__blk69_slot = var_t6__blk69;
        *var_t6__blk69_dn0_slot = var_t6__blk69_dn0;
        *var_t6__blk69_dn10_slot = var_t6__blk69_dn10;
        *var_t6__blk69_dn11_slot = var_t6__blk69_dn11;
        *var_t6__blk69_dn12_slot = var_t6__blk69_dn12;
        *var_t6__blk69_dn17_slot = var_t6__blk69_dn17;
        *var_t6__blk69_dn2_slot = var_t6__blk69_dn2;
        *var_t6__blk69_dn6_slot = var_t6__blk69_dn6;
        *var_t6__blk69_dn7_slot = var_t6__blk69_dn7;
        *var_tfoxe_slot = var_tfoxe;
        *var_tfoxe_dn0_slot = var_tfoxe_dn0;
        *var_tfoxe_dn10_slot = var_tfoxe_dn10;
        *var_tfoxe_dn11_slot = var_tfoxe_dn11;
        *var_tfoxe_dn12_slot = var_tfoxe_dn12;
        *var_tfoxe_dn17_slot = var_tfoxe_dn17;
        *var_tfoxe_dn2_slot = var_tfoxe_dn2;
        *var_tfoxe_dn6_slot = var_tfoxe_dn6;
        *var_tfoxe_dn7_slot = var_tfoxe_dn7;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn17_slot = var_tmf2_dn17;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_vbsz2_slot = var_vbsz2;
        *var_vbsz2_dn0_slot = var_vbsz2_dn0;
        *var_vbsz2_dn10_slot = var_vbsz2_dn10;
        *var_vbsz2_dn11_slot = var_vbsz2_dn11;
        *var_vbsz2_dn12_slot = var_vbsz2_dn12;
        *var_vbsz2_dn17_slot = var_vbsz2_dn17;
        *var_vbsz2_dn2_slot = var_vbsz2_dn2;
        *var_vbsz2_dn6_slot = var_vbsz2_dn6;
        *var_vbsz2_dn7_slot = var_vbsz2_dn7;
    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        var_beta_inv: f64,
        var_beta_inv_dn10: f64,
        var_c_fox_inv: f64,
        var_c_fox_inv_dn0: f64,
        var_c_fox_inv_dn10: f64,
        var_c_fox_inv_dn11: f64,
        var_c_fox_inv_dn12: f64,
        var_c_fox_inv_dn17: f64,
        var_c_fox_inv_dn2: f64,
        var_c_fox_inv_dn6: f64,
        var_c_fox_inv_dn7: f64,
        var_guard74: f64,
        var_pb2: f64,
        var_pb20: f64,
        var_pb20_dn0: f64,
        var_pb20_dn10: f64,
        var_pb20_dn11: f64,
        var_pb20_dn12: f64,
        var_pb20_dn17: f64,
        var_pb20_dn2: f64,
        var_pb20_dn6: f64,
        var_pb20_dn7: f64,
        var_pb2_dn0: f64,
        var_pb2_dn10: f64,
        var_pb2_dn11: f64,
        var_pb2_dn12: f64,
        var_pb2_dn17: f64,
        var_pb2_dn2: f64,
        var_pb2_dn6: f64,
        var_pb2_dn7: f64,
        var_pb2c: f64,
        var_pb2c_dn0: f64,
        var_pb2c_dn10: f64,
        var_pb2c_dn11: f64,
        var_pb2c_dn12: f64,
        var_pb2c_dn17: f64,
        var_pb2c_dn2: f64,
        var_pb2c_dn6: f64,
        var_pb2c_dn7: f64,
        var_ptovr: f64,
        var_ptovr_dn0: f64,
        var_ptovr_dn10: f64,
        var_ptovr_dn11: f64,
        var_ptovr_dn12: f64,
        var_ptovr_dn17: f64,
        var_ptovr_dn2: f64,
        var_ptovr_dn6: f64,
        var_ptovr_dn7: f64,
        var_q_nsub: f64,
        var_q_nsub_dn0: f64,
        var_q_nsub_dn10: f64,
        var_q_nsub_dn11: f64,
        var_q_nsub_dn12: f64,
        var_q_nsub_dn17: f64,
        var_q_nsub_dn2: f64,
        var_q_nsub_dn6: f64,
        var_q_nsub_dn7: f64,
        var_qnsub_esi2: f64,
        var_qnsub_esi2_dn0: f64,
        var_qnsub_esi2_dn10: f64,
        var_qnsub_esi2_dn11: f64,
        var_qnsub_esi2_dn12: f64,
        var_qnsub_esi2_dn17: f64,
        var_qnsub_esi2_dn2: f64,
        var_qnsub_esi2_dn6: f64,
        var_qnsub_esi2_dn7: f64,
        var_subversion: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn17: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn7: f64,
        var_vfb: f64,
        var_guard75_slot: &mut f64,
        var_guard76_slot: &mut f64,
        var_guard88_slot: &mut f64,
        var_pb20b_slot: &mut f64,
        var_pb20b_dn0_slot: &mut f64,
        var_pb20b_dn10_slot: &mut f64,
        var_pb20b_dn11_slot: &mut f64,
        var_pb20b_dn12_slot: &mut f64,
        var_pb20b_dn17_slot: &mut f64,
        var_pb20b_dn2_slot: &mut f64,
        var_pb20b_dn6_slot: &mut f64,
        var_pb20b_dn7_slot: &mut f64,
        var_pbsum_slot: &mut f64,
        var_pbsum_dn0_slot: &mut f64,
        var_pbsum_dn10_slot: &mut f64,
        var_pbsum_dn11_slot: &mut f64,
        var_pbsum_dn12_slot: &mut f64,
        var_pbsum_dn17_slot: &mut f64,
        var_pbsum_dn2_slot: &mut f64,
        var_pbsum_dn6_slot: &mut f64,
        var_pbsum_dn7_slot: &mut f64,
        var_qb0_slot: &mut f64,
        var_qb0_dn0_slot: &mut f64,
        var_qb0_dn10_slot: &mut f64,
        var_qb0_dn11_slot: &mut f64,
        var_qb0_dn12_slot: &mut f64,
        var_qb0_dn17_slot: &mut f64,
        var_qb0_dn2_slot: &mut f64,
        var_qb0_dn6_slot: &mut f64,
        var_qb0_dn7_slot: &mut f64,
        var_sqrt_pbsum_slot: &mut f64,
        var_sqrt_pbsum_dn0_slot: &mut f64,
        var_sqrt_pbsum_dn10_slot: &mut f64,
        var_sqrt_pbsum_dn11_slot: &mut f64,
        var_sqrt_pbsum_dn12_slot: &mut f64,
        var_sqrt_pbsum_dn17_slot: &mut f64,
        var_sqrt_pbsum_dn2_slot: &mut f64,
        var_sqrt_pbsum_dn6_slot: &mut f64,
        var_sqrt_pbsum_dn7_slot: &mut f64,
        var_t0__blk78_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1__blk77_slot: &mut f64,
        var_t1__blk77_dn0_slot: &mut f64,
        var_t1__blk77_dn10_slot: &mut f64,
        var_t1__blk77_dn11_slot: &mut f64,
        var_t1__blk77_dn12_slot: &mut f64,
        var_t1__blk77_dn17_slot: &mut f64,
        var_t1__blk77_dn2_slot: &mut f64,
        var_t1__blk77_dn6_slot: &mut f64,
        var_t1__blk77_dn7_slot: &mut f64,
        var_t1__blk82_slot: &mut f64,
        var_t1__blk82_dn0_slot: &mut f64,
        var_t1__blk82_dn10_slot: &mut f64,
        var_t1__blk82_dn11_slot: &mut f64,
        var_t1__blk82_dn12_slot: &mut f64,
        var_t1__blk82_dn17_slot: &mut f64,
        var_t1__blk82_dn2_slot: &mut f64,
        var_t1__blk82_dn6_slot: &mut f64,
        var_t1__blk82_dn7_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t2__blk79_slot: &mut f64,
        var_t2__blk79_dn0_slot: &mut f64,
        var_t2__blk79_dn10_slot: &mut f64,
        var_t2__blk79_dn11_slot: &mut f64,
        var_t2__blk79_dn12_slot: &mut f64,
        var_t2__blk79_dn17_slot: &mut f64,
        var_t2__blk79_dn2_slot: &mut f64,
        var_t2__blk79_dn6_slot: &mut f64,
        var_t2__blk79_dn7_slot: &mut f64,
        var_t2__blk83_slot: &mut f64,
        var_t2__blk83_dn0_slot: &mut f64,
        var_t2__blk83_dn10_slot: &mut f64,
        var_t2__blk83_dn11_slot: &mut f64,
        var_t2__blk83_dn12_slot: &mut f64,
        var_t2__blk83_dn17_slot: &mut f64,
        var_t2__blk83_dn2_slot: &mut f64,
        var_t2__blk83_dn6_slot: &mut f64,
        var_t2__blk83_dn7_slot: &mut f64,
        var_t3__blk80_slot: &mut f64,
        var_t3__blk80_dn0_slot: &mut f64,
        var_t3__blk80_dn10_slot: &mut f64,
        var_t3__blk80_dn11_slot: &mut f64,
        var_t3__blk80_dn12_slot: &mut f64,
        var_t3__blk80_dn17_slot: &mut f64,
        var_t3__blk80_dn2_slot: &mut f64,
        var_t3__blk80_dn6_slot: &mut f64,
        var_t3__blk80_dn7_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn17_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_vbslim_slot: &mut f64,
        var_vbslim_dn0_slot: &mut f64,
        var_vbslim_dn10_slot: &mut f64,
        var_vbslim_dn11_slot: &mut f64,
        var_vbslim_dn12_slot: &mut f64,
        var_vbslim_dn17_slot: &mut f64,
        var_vbslim_dn2_slot: &mut f64,
        var_vbslim_dn6_slot: &mut f64,
        var_vbslim_dn7_slot: &mut f64,
        var_vbsz2_slot: &mut f64,
        var_vbsz2_dn0_slot: &mut f64,
        var_vbsz2_dn10_slot: &mut f64,
        var_vbsz2_dn11_slot: &mut f64,
        var_vbsz2_dn12_slot: &mut f64,
        var_vbsz2_dn17_slot: &mut f64,
        var_vbsz2_dn2_slot: &mut f64,
        var_vbsz2_dn6_slot: &mut f64,
        var_vbsz2_dn7_slot: &mut f64,
        var_vth0_slot: &mut f64,
        var_vth0_dn0_slot: &mut f64,
        var_vth0_dn10_slot: &mut f64,
        var_vth0_dn11_slot: &mut f64,
        var_vth0_dn12_slot: &mut f64,
        var_vth0_dn17_slot: &mut f64,
        var_vth0_dn2_slot: &mut f64,
        var_vth0_dn6_slot: &mut f64,
        var_vth0_dn7_slot: &mut f64,
        var_vthp_slot: &mut f64,
        var_vthp_dn0_slot: &mut f64,
        var_vthp_dn10_slot: &mut f64,
        var_vthp_dn11_slot: &mut f64,
        var_vthp_dn12_slot: &mut f64,
        var_vthp_dn17_slot: &mut f64,
        var_vthp_dn2_slot: &mut f64,
        var_vthp_dn6_slot: &mut f64,
        var_vthp_dn7_slot: &mut f64,
        var_wd0_slot: &mut f64,
        var_wd0_dn0_slot: &mut f64,
        var_wd0_dn10_slot: &mut f64,
        var_wd0_dn11_slot: &mut f64,
        var_wd0_dn12_slot: &mut f64,
        var_wd0_dn17_slot: &mut f64,
        var_wd0_dn2_slot: &mut f64,
        var_wd0_dn6_slot: &mut f64,
        var_wd0_dn7_slot: &mut f64,
    ) {
        let mut var_guard75: f64 = *var_guard75_slot;
        let mut var_guard76: f64 = *var_guard76_slot;
        let mut var_guard88: f64 = *var_guard88_slot;
        let mut var_pb20b: f64 = *var_pb20b_slot;
        let mut var_pb20b_dn0: f64 = *var_pb20b_dn0_slot;
        let mut var_pb20b_dn10: f64 = *var_pb20b_dn10_slot;
        let mut var_pb20b_dn11: f64 = *var_pb20b_dn11_slot;
        let mut var_pb20b_dn12: f64 = *var_pb20b_dn12_slot;
        let mut var_pb20b_dn17: f64 = *var_pb20b_dn17_slot;
        let mut var_pb20b_dn2: f64 = *var_pb20b_dn2_slot;
        let mut var_pb20b_dn6: f64 = *var_pb20b_dn6_slot;
        let mut var_pb20b_dn7: f64 = *var_pb20b_dn7_slot;
        let mut var_pbsum: f64 = *var_pbsum_slot;
        let mut var_pbsum_dn0: f64 = *var_pbsum_dn0_slot;
        let mut var_pbsum_dn10: f64 = *var_pbsum_dn10_slot;
        let mut var_pbsum_dn11: f64 = *var_pbsum_dn11_slot;
        let mut var_pbsum_dn12: f64 = *var_pbsum_dn12_slot;
        let mut var_pbsum_dn17: f64 = *var_pbsum_dn17_slot;
        let mut var_pbsum_dn2: f64 = *var_pbsum_dn2_slot;
        let mut var_pbsum_dn6: f64 = *var_pbsum_dn6_slot;
        let mut var_pbsum_dn7: f64 = *var_pbsum_dn7_slot;
        let mut var_qb0: f64 = *var_qb0_slot;
        let mut var_qb0_dn0: f64 = *var_qb0_dn0_slot;
        let mut var_qb0_dn10: f64 = *var_qb0_dn10_slot;
        let mut var_qb0_dn11: f64 = *var_qb0_dn11_slot;
        let mut var_qb0_dn12: f64 = *var_qb0_dn12_slot;
        let mut var_qb0_dn17: f64 = *var_qb0_dn17_slot;
        let mut var_qb0_dn2: f64 = *var_qb0_dn2_slot;
        let mut var_qb0_dn6: f64 = *var_qb0_dn6_slot;
        let mut var_qb0_dn7: f64 = *var_qb0_dn7_slot;
        let mut var_sqrt_pbsum: f64 = *var_sqrt_pbsum_slot;
        let mut var_sqrt_pbsum_dn0: f64 = *var_sqrt_pbsum_dn0_slot;
        let mut var_sqrt_pbsum_dn10: f64 = *var_sqrt_pbsum_dn10_slot;
        let mut var_sqrt_pbsum_dn11: f64 = *var_sqrt_pbsum_dn11_slot;
        let mut var_sqrt_pbsum_dn12: f64 = *var_sqrt_pbsum_dn12_slot;
        let mut var_sqrt_pbsum_dn17: f64 = *var_sqrt_pbsum_dn17_slot;
        let mut var_sqrt_pbsum_dn2: f64 = *var_sqrt_pbsum_dn2_slot;
        let mut var_sqrt_pbsum_dn6: f64 = *var_sqrt_pbsum_dn6_slot;
        let mut var_sqrt_pbsum_dn7: f64 = *var_sqrt_pbsum_dn7_slot;
        let mut var_t0__blk78: f64 = *var_t0__blk78_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1__blk77: f64 = *var_t1__blk77_slot;
        let mut var_t1__blk77_dn0: f64 = *var_t1__blk77_dn0_slot;
        let mut var_t1__blk77_dn10: f64 = *var_t1__blk77_dn10_slot;
        let mut var_t1__blk77_dn11: f64 = *var_t1__blk77_dn11_slot;
        let mut var_t1__blk77_dn12: f64 = *var_t1__blk77_dn12_slot;
        let mut var_t1__blk77_dn17: f64 = *var_t1__blk77_dn17_slot;
        let mut var_t1__blk77_dn2: f64 = *var_t1__blk77_dn2_slot;
        let mut var_t1__blk77_dn6: f64 = *var_t1__blk77_dn6_slot;
        let mut var_t1__blk77_dn7: f64 = *var_t1__blk77_dn7_slot;
        let mut var_t1__blk82: f64 = *var_t1__blk82_slot;
        let mut var_t1__blk82_dn0: f64 = *var_t1__blk82_dn0_slot;
        let mut var_t1__blk82_dn10: f64 = *var_t1__blk82_dn10_slot;
        let mut var_t1__blk82_dn11: f64 = *var_t1__blk82_dn11_slot;
        let mut var_t1__blk82_dn12: f64 = *var_t1__blk82_dn12_slot;
        let mut var_t1__blk82_dn17: f64 = *var_t1__blk82_dn17_slot;
        let mut var_t1__blk82_dn2: f64 = *var_t1__blk82_dn2_slot;
        let mut var_t1__blk82_dn6: f64 = *var_t1__blk82_dn6_slot;
        let mut var_t1__blk82_dn7: f64 = *var_t1__blk82_dn7_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t2__blk79: f64 = *var_t2__blk79_slot;
        let mut var_t2__blk79_dn0: f64 = *var_t2__blk79_dn0_slot;
        let mut var_t2__blk79_dn10: f64 = *var_t2__blk79_dn10_slot;
        let mut var_t2__blk79_dn11: f64 = *var_t2__blk79_dn11_slot;
        let mut var_t2__blk79_dn12: f64 = *var_t2__blk79_dn12_slot;
        let mut var_t2__blk79_dn17: f64 = *var_t2__blk79_dn17_slot;
        let mut var_t2__blk79_dn2: f64 = *var_t2__blk79_dn2_slot;
        let mut var_t2__blk79_dn6: f64 = *var_t2__blk79_dn6_slot;
        let mut var_t2__blk79_dn7: f64 = *var_t2__blk79_dn7_slot;
        let mut var_t2__blk83: f64 = *var_t2__blk83_slot;
        let mut var_t2__blk83_dn0: f64 = *var_t2__blk83_dn0_slot;
        let mut var_t2__blk83_dn10: f64 = *var_t2__blk83_dn10_slot;
        let mut var_t2__blk83_dn11: f64 = *var_t2__blk83_dn11_slot;
        let mut var_t2__blk83_dn12: f64 = *var_t2__blk83_dn12_slot;
        let mut var_t2__blk83_dn17: f64 = *var_t2__blk83_dn17_slot;
        let mut var_t2__blk83_dn2: f64 = *var_t2__blk83_dn2_slot;
        let mut var_t2__blk83_dn6: f64 = *var_t2__blk83_dn6_slot;
        let mut var_t2__blk83_dn7: f64 = *var_t2__blk83_dn7_slot;
        let mut var_t3__blk80: f64 = *var_t3__blk80_slot;
        let mut var_t3__blk80_dn0: f64 = *var_t3__blk80_dn0_slot;
        let mut var_t3__blk80_dn10: f64 = *var_t3__blk80_dn10_slot;
        let mut var_t3__blk80_dn11: f64 = *var_t3__blk80_dn11_slot;
        let mut var_t3__blk80_dn12: f64 = *var_t3__blk80_dn12_slot;
        let mut var_t3__blk80_dn17: f64 = *var_t3__blk80_dn17_slot;
        let mut var_t3__blk80_dn2: f64 = *var_t3__blk80_dn2_slot;
        let mut var_t3__blk80_dn6: f64 = *var_t3__blk80_dn6_slot;
        let mut var_t3__blk80_dn7: f64 = *var_t3__blk80_dn7_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn17: f64 = *var_tmf2_dn17_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_vbslim: f64 = *var_vbslim_slot;
        let mut var_vbslim_dn0: f64 = *var_vbslim_dn0_slot;
        let mut var_vbslim_dn10: f64 = *var_vbslim_dn10_slot;
        let mut var_vbslim_dn11: f64 = *var_vbslim_dn11_slot;
        let mut var_vbslim_dn12: f64 = *var_vbslim_dn12_slot;
        let mut var_vbslim_dn17: f64 = *var_vbslim_dn17_slot;
        let mut var_vbslim_dn2: f64 = *var_vbslim_dn2_slot;
        let mut var_vbslim_dn6: f64 = *var_vbslim_dn6_slot;
        let mut var_vbslim_dn7: f64 = *var_vbslim_dn7_slot;
        let mut var_vbsz2: f64 = *var_vbsz2_slot;
        let mut var_vbsz2_dn0: f64 = *var_vbsz2_dn0_slot;
        let mut var_vbsz2_dn10: f64 = *var_vbsz2_dn10_slot;
        let mut var_vbsz2_dn11: f64 = *var_vbsz2_dn11_slot;
        let mut var_vbsz2_dn12: f64 = *var_vbsz2_dn12_slot;
        let mut var_vbsz2_dn17: f64 = *var_vbsz2_dn17_slot;
        let mut var_vbsz2_dn2: f64 = *var_vbsz2_dn2_slot;
        let mut var_vbsz2_dn6: f64 = *var_vbsz2_dn6_slot;
        let mut var_vbsz2_dn7: f64 = *var_vbsz2_dn7_slot;
        let mut var_vth0: f64 = *var_vth0_slot;
        let mut var_vth0_dn0: f64 = *var_vth0_dn0_slot;
        let mut var_vth0_dn10: f64 = *var_vth0_dn10_slot;
        let mut var_vth0_dn11: f64 = *var_vth0_dn11_slot;
        let mut var_vth0_dn12: f64 = *var_vth0_dn12_slot;
        let mut var_vth0_dn17: f64 = *var_vth0_dn17_slot;
        let mut var_vth0_dn2: f64 = *var_vth0_dn2_slot;
        let mut var_vth0_dn6: f64 = *var_vth0_dn6_slot;
        let mut var_vth0_dn7: f64 = *var_vth0_dn7_slot;
        let mut var_vthp: f64 = *var_vthp_slot;
        let mut var_vthp_dn0: f64 = *var_vthp_dn0_slot;
        let mut var_vthp_dn10: f64 = *var_vthp_dn10_slot;
        let mut var_vthp_dn11: f64 = *var_vthp_dn11_slot;
        let mut var_vthp_dn12: f64 = *var_vthp_dn12_slot;
        let mut var_vthp_dn17: f64 = *var_vthp_dn17_slot;
        let mut var_vthp_dn2: f64 = *var_vthp_dn2_slot;
        let mut var_vthp_dn6: f64 = *var_vthp_dn6_slot;
        let mut var_vthp_dn7: f64 = *var_vthp_dn7_slot;
        let mut var_wd0: f64 = *var_wd0_slot;
        let mut var_wd0_dn0: f64 = *var_wd0_dn0_slot;
        let mut var_wd0_dn10: f64 = *var_wd0_dn10_slot;
        let mut var_wd0_dn11: f64 = *var_wd0_dn11_slot;
        let mut var_wd0_dn12: f64 = *var_wd0_dn12_slot;
        let mut var_wd0_dn17: f64 = *var_wd0_dn17_slot;
        let mut var_wd0_dn2: f64 = *var_wd0_dn2_slot;
        let mut var_wd0_dn6: f64 = *var_wd0_dn6_slot;
        let mut var_wd0_dn7: f64 = *var_wd0_dn7_slot;

        let (assign6090_e4020, assign6090_e4020_d_n0, assign6090_e4020_d_n2, assign6090_e4020_d_n6, assign6090_e4020_d_n7, assign6090_e4020_d_n10, assign6090_e4020_d_n11, assign6090_e4020_d_n12, assign6090_e4020_d_n17,) = {
    if (var_guard74 != 0.0) {
        let assign6090_e4006: f64 = (-p.p237);
        let assign6090_e4008: f64 = (assign6090_e4006 * p.p237);
        let assign6090_e4010: f64 = (assign6090_e4008 * var_q_nsub);
        let assign6090_e4013: f64 = (2.0 * 1.034943e-10);
        let assign6090_e4014: f64 = (assign6090_e4010 / assign6090_e4013);
        let assign6090_e4016: f64 = (assign6090_e4014 + var_pb2);
        let assign6090_e4018: f64 = (assign6090_e4016 - var_beta_inv);
        (assign6090_e4018, (((assign6090_e4008 * var_q_nsub_dn0) / assign6090_e4013) + var_pb2_dn0), (((assign6090_e4008 * var_q_nsub_dn2) / assign6090_e4013) + var_pb2_dn2), (((assign6090_e4008 * var_q_nsub_dn6) / assign6090_e4013) + var_pb2_dn6), (((assign6090_e4008 * var_q_nsub_dn7) / assign6090_e4013) + var_pb2_dn7), ((((assign6090_e4008 * var_q_nsub_dn10) / assign6090_e4013) + var_pb2_dn10) - var_beta_inv_dn10), (((assign6090_e4008 * var_q_nsub_dn11) / assign6090_e4013) + var_pb2_dn11), (((assign6090_e4008 * var_q_nsub_dn12) / assign6090_e4013) + var_pb2_dn12), (((assign6090_e4008 * var_q_nsub_dn17) / assign6090_e4013) + var_pb2_dn17),)
    } else {
        (var_vbslim, var_vbslim_dn0, var_vbslim_dn2, var_vbslim_dn6, var_vbslim_dn7, var_vbslim_dn10, var_vbslim_dn11, var_vbslim_dn12, var_vbslim_dn17,)
    }
};
        var_vbslim = assign6090_e4020;
        var_vbslim_dn0 = assign6090_e4020_d_n0;
        var_vbslim_dn2 = assign6090_e4020_d_n2;
        var_vbslim_dn6 = assign6090_e4020_d_n6;
        var_vbslim_dn7 = assign6090_e4020_d_n7;
        var_vbslim_dn10 = assign6090_e4020_d_n10;
        var_vbslim_dn11 = assign6090_e4020_d_n11;
        var_vbslim_dn12 = assign6090_e4020_d_n12;
        var_vbslim_dn17 = assign6090_e4020_d_n17;

        let (assign6100_e4028, assign6100_e4028_d_n0, assign6100_e4028_d_n2, assign6100_e4028_d_n6, assign6100_e4028_d_n7, assign6100_e4028_d_n10, assign6100_e4028_d_n11, assign6100_e4028_d_n12, assign6100_e4028_d_n17,) = {
    if (var_guard74 != 0.0) {
        let assign6100_e4024: f64 = (var_vbsz2 - var_vbslim);
        let assign6100_e4026: f64 = (assign6100_e4024 - 0.001);
        (assign6100_e4026, (var_vbsz2_dn0 - var_vbslim_dn0), (var_vbsz2_dn2 - var_vbslim_dn2), (var_vbsz2_dn6 - var_vbslim_dn6), (var_vbsz2_dn7 - var_vbslim_dn7), (var_vbsz2_dn10 - var_vbslim_dn10), (var_vbsz2_dn11 - var_vbslim_dn11), (var_vbsz2_dn12 - var_vbslim_dn12), (var_vbsz2_dn17 - var_vbslim_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign6100_e4028;
        var_tmf1_dn0 = assign6100_e4028_d_n0;
        var_tmf1_dn2 = assign6100_e4028_d_n2;
        var_tmf1_dn6 = assign6100_e4028_d_n6;
        var_tmf1_dn7 = assign6100_e4028_d_n7;
        var_tmf1_dn10 = assign6100_e4028_d_n10;
        var_tmf1_dn11 = assign6100_e4028_d_n11;
        var_tmf1_dn12 = assign6100_e4028_d_n12;
        var_tmf1_dn17 = assign6100_e4028_d_n17;

        let (assign6110_e4036, assign6110_e4036_d_n0, assign6110_e4036_d_n2, assign6110_e4036_d_n6, assign6110_e4036_d_n7, assign6110_e4036_d_n10, assign6110_e4036_d_n11, assign6110_e4036_d_n12, assign6110_e4036_d_n17,) = {
    if (var_guard74 != 0.0) {
        let assign6110_e4032: f64 = (4.0 * var_vbslim);
        let assign6110_e4034: f64 = (assign6110_e4032 * 0.001);
        (assign6110_e4034, ((4.0 * var_vbslim_dn0) * 0.001), ((4.0 * var_vbslim_dn2) * 0.001), ((4.0 * var_vbslim_dn6) * 0.001), ((4.0 * var_vbslim_dn7) * 0.001), ((4.0 * var_vbslim_dn10) * 0.001), ((4.0 * var_vbslim_dn11) * 0.001), ((4.0 * var_vbslim_dn12) * 0.001), ((4.0 * var_vbslim_dn17) * 0.001),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6110_e4036;
        var_tmf2_dn0 = assign6110_e4036_d_n0;
        var_tmf2_dn2 = assign6110_e4036_d_n2;
        var_tmf2_dn6 = assign6110_e4036_d_n6;
        var_tmf2_dn7 = assign6110_e4036_d_n7;
        var_tmf2_dn10 = assign6110_e4036_d_n10;
        var_tmf2_dn11 = assign6110_e4036_d_n11;
        var_tmf2_dn12 = assign6110_e4036_d_n12;
        var_tmf2_dn17 = assign6110_e4036_d_n17;

        let (assign6120_e4046, assign6120_e4046_d_n0, assign6120_e4046_d_n2, assign6120_e4046_d_n6, assign6120_e4046_d_n7, assign6120_e4046_d_n10, assign6120_e4046_d_n11, assign6120_e4046_d_n12, assign6120_e4046_d_n17,) = {
    if (var_guard74 != 0.0) {
        let (assign6120_e4044, assign6120_e4044_d_n0, assign6120_e4044_d_n2, assign6120_e4044_d_n6, assign6120_e4044_d_n7, assign6120_e4044_d_n10, assign6120_e4044_d_n11, assign6120_e4044_d_n12, assign6120_e4044_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign6120_e4043: f64 = (-var_tmf2);
                (assign6120_e4043, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign6120_e4044, assign6120_e4044_d_n0, assign6120_e4044_d_n2, assign6120_e4044_d_n6, assign6120_e4044_d_n7, assign6120_e4044_d_n10, assign6120_e4044_d_n11, assign6120_e4044_d_n12, assign6120_e4044_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6120_e4046;
        var_tmf2_dn0 = assign6120_e4046_d_n0;
        var_tmf2_dn2 = assign6120_e4046_d_n2;
        var_tmf2_dn6 = assign6120_e4046_d_n6;
        var_tmf2_dn7 = assign6120_e4046_d_n7;
        var_tmf2_dn10 = assign6120_e4046_d_n10;
        var_tmf2_dn11 = assign6120_e4046_d_n11;
        var_tmf2_dn12 = assign6120_e4046_d_n12;
        var_tmf2_dn17 = assign6120_e4046_d_n17;

        let (assign6130_e4055, assign6130_e4055_d_n0, assign6130_e4055_d_n2, assign6130_e4055_d_n6, assign6130_e4055_d_n7, assign6130_e4055_d_n10, assign6130_e4055_d_n11, assign6130_e4055_d_n12, assign6130_e4055_d_n17,) = {
    if (var_guard74 != 0.0) {
        let assign6130_e4050: f64 = (var_tmf1 * var_tmf1);
        let assign6130_e4052: f64 = (assign6130_e4050 + var_tmf2);
        let assign6130_e4053: f64 = (assign6130_e4052).sqrt();
        (assign6130_e4053, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign6130_e4053)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign6130_e4053)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign6130_e4053)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign6130_e4053)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign6130_e4053)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign6130_e4053)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign6130_e4053)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign6130_e4053)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6130_e4055;
        var_tmf2_dn0 = assign6130_e4055_d_n0;
        var_tmf2_dn2 = assign6130_e4055_d_n2;
        var_tmf2_dn6 = assign6130_e4055_d_n6;
        var_tmf2_dn7 = assign6130_e4055_d_n7;
        var_tmf2_dn10 = assign6130_e4055_d_n10;
        var_tmf2_dn11 = assign6130_e4055_d_n11;
        var_tmf2_dn12 = assign6130_e4055_d_n12;
        var_tmf2_dn17 = assign6130_e4055_d_n17;

        let (assign6140_e4065, assign6140_e4065_d_n0, assign6140_e4065_d_n2, assign6140_e4065_d_n6, assign6140_e4065_d_n7, assign6140_e4065_d_n10, assign6140_e4065_d_n11, assign6140_e4065_d_n12, assign6140_e4065_d_n17,) = {
    if (var_guard74 != 0.0) {
        let assign6140_e4061: f64 = (var_tmf1 + var_tmf2);
        let assign6140_e4062: f64 = (0.5 * assign6140_e4061);
        let assign6140_e4063: f64 = (var_vbslim + assign6140_e4062);
        (assign6140_e4063, (var_vbslim_dn0 + (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_vbslim_dn2 + (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_vbslim_dn6 + (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_vbslim_dn7 + (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (var_vbslim_dn10 + (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_vbslim_dn11 + (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (var_vbslim_dn12 + (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), (var_vbslim_dn17 + (0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_vbsz2, var_vbsz2_dn0, var_vbsz2_dn2, var_vbsz2_dn6, var_vbsz2_dn7, var_vbsz2_dn10, var_vbsz2_dn11, var_vbsz2_dn12, var_vbsz2_dn17,)
    }
};
        var_vbsz2 = assign6140_e4065;
        var_vbsz2_dn0 = assign6140_e4065_d_n0;
        var_vbsz2_dn2 = assign6140_e4065_d_n2;
        var_vbsz2_dn6 = assign6140_e4065_d_n6;
        var_vbsz2_dn7 = assign6140_e4065_d_n7;
        var_vbsz2_dn10 = assign6140_e4065_d_n10;
        var_vbsz2_dn11 = assign6140_e4065_d_n11;
        var_vbsz2_dn12 = assign6140_e4065_d_n12;
        var_vbsz2_dn17 = assign6140_e4065_d_n17;

        let assign6150_e4068: f64 = if var_subversion > 2.0 { 1.0 } else { 0.0 };
        var_guard75 = assign6150_e4068;

        let (assign6160_e4078, assign6160_e4078_d_n0, assign6160_e4078_d_n2, assign6160_e4078_d_n6, assign6160_e4078_d_n7, assign6160_e4078_d_n10, assign6160_e4078_d_n11, assign6160_e4078_d_n12, assign6160_e4078_d_n17,) = {
    if ((var_guard74 != 0.0) && (var_guard75 != 0.0)) {
        let assign6160_e4074: f64 = (var_pb20 - var_vbsz2);
        let assign6160_e4076: f64 = (assign6160_e4074 - 0.001);
        (assign6160_e4076, (var_pb20_dn0 - var_vbsz2_dn0), (var_pb20_dn2 - var_vbsz2_dn2), (var_pb20_dn6 - var_vbsz2_dn6), (var_pb20_dn7 - var_vbsz2_dn7), (var_pb20_dn10 - var_vbsz2_dn10), (var_pb20_dn11 - var_vbsz2_dn11), (var_pb20_dn12 - var_vbsz2_dn12), (var_pb20_dn17 - var_vbsz2_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign6160_e4078;
        var_tmf1_dn0 = assign6160_e4078_d_n0;
        var_tmf1_dn2 = assign6160_e4078_d_n2;
        var_tmf1_dn6 = assign6160_e4078_d_n6;
        var_tmf1_dn7 = assign6160_e4078_d_n7;
        var_tmf1_dn10 = assign6160_e4078_d_n10;
        var_tmf1_dn11 = assign6160_e4078_d_n11;
        var_tmf1_dn12 = assign6160_e4078_d_n12;
        var_tmf1_dn17 = assign6160_e4078_d_n17;

        let (assign6170_e4088, assign6170_e4088_d_n0, assign6170_e4088_d_n2, assign6170_e4088_d_n6, assign6170_e4088_d_n7, assign6170_e4088_d_n10, assign6170_e4088_d_n11, assign6170_e4088_d_n12, assign6170_e4088_d_n17,) = {
    if ((var_guard74 != 0.0) && (var_guard75 != 0.0)) {
        let assign6170_e4084: f64 = (4.0 * var_pb20);
        let assign6170_e4086: f64 = (assign6170_e4084 * 0.001);
        (assign6170_e4086, ((4.0 * var_pb20_dn0) * 0.001), ((4.0 * var_pb20_dn2) * 0.001), ((4.0 * var_pb20_dn6) * 0.001), ((4.0 * var_pb20_dn7) * 0.001), ((4.0 * var_pb20_dn10) * 0.001), ((4.0 * var_pb20_dn11) * 0.001), ((4.0 * var_pb20_dn12) * 0.001), ((4.0 * var_pb20_dn17) * 0.001),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6170_e4088;
        var_tmf2_dn0 = assign6170_e4088_d_n0;
        var_tmf2_dn2 = assign6170_e4088_d_n2;
        var_tmf2_dn6 = assign6170_e4088_d_n6;
        var_tmf2_dn7 = assign6170_e4088_d_n7;
        var_tmf2_dn10 = assign6170_e4088_d_n10;
        var_tmf2_dn11 = assign6170_e4088_d_n11;
        var_tmf2_dn12 = assign6170_e4088_d_n12;
        var_tmf2_dn17 = assign6170_e4088_d_n17;

        let (assign6180_e4100, assign6180_e4100_d_n0, assign6180_e4100_d_n2, assign6180_e4100_d_n6, assign6180_e4100_d_n7, assign6180_e4100_d_n10, assign6180_e4100_d_n11, assign6180_e4100_d_n12, assign6180_e4100_d_n17,) = {
    if ((var_guard74 != 0.0) && (var_guard75 != 0.0)) {
        let (assign6180_e4098, assign6180_e4098_d_n0, assign6180_e4098_d_n2, assign6180_e4098_d_n6, assign6180_e4098_d_n7, assign6180_e4098_d_n10, assign6180_e4098_d_n11, assign6180_e4098_d_n12, assign6180_e4098_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign6180_e4097: f64 = (-var_tmf2);
                (assign6180_e4097, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign6180_e4098, assign6180_e4098_d_n0, assign6180_e4098_d_n2, assign6180_e4098_d_n6, assign6180_e4098_d_n7, assign6180_e4098_d_n10, assign6180_e4098_d_n11, assign6180_e4098_d_n12, assign6180_e4098_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6180_e4100;
        var_tmf2_dn0 = assign6180_e4100_d_n0;
        var_tmf2_dn2 = assign6180_e4100_d_n2;
        var_tmf2_dn6 = assign6180_e4100_d_n6;
        var_tmf2_dn7 = assign6180_e4100_d_n7;
        var_tmf2_dn10 = assign6180_e4100_d_n10;
        var_tmf2_dn11 = assign6180_e4100_d_n11;
        var_tmf2_dn12 = assign6180_e4100_d_n12;
        var_tmf2_dn17 = assign6180_e4100_d_n17;

        let (assign6190_e4111, assign6190_e4111_d_n0, assign6190_e4111_d_n2, assign6190_e4111_d_n6, assign6190_e4111_d_n7, assign6190_e4111_d_n10, assign6190_e4111_d_n11, assign6190_e4111_d_n12, assign6190_e4111_d_n17,) = {
    if ((var_guard74 != 0.0) && (var_guard75 != 0.0)) {
        let assign6190_e4106: f64 = (var_tmf1 * var_tmf1);
        let assign6190_e4108: f64 = (assign6190_e4106 + var_tmf2);
        let assign6190_e4109: f64 = (assign6190_e4108).sqrt();
        (assign6190_e4109, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign6190_e4109)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign6190_e4109)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign6190_e4109)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign6190_e4109)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign6190_e4109)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign6190_e4109)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign6190_e4109)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign6190_e4109)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6190_e4111;
        var_tmf2_dn0 = assign6190_e4111_d_n0;
        var_tmf2_dn2 = assign6190_e4111_d_n2;
        var_tmf2_dn6 = assign6190_e4111_d_n6;
        var_tmf2_dn7 = assign6190_e4111_d_n7;
        var_tmf2_dn10 = assign6190_e4111_d_n10;
        var_tmf2_dn11 = assign6190_e4111_d_n11;
        var_tmf2_dn12 = assign6190_e4111_d_n12;
        var_tmf2_dn17 = assign6190_e4111_d_n17;

        let (assign6200_e4123, assign6200_e4123_d_n0, assign6200_e4123_d_n2, assign6200_e4123_d_n6, assign6200_e4123_d_n7, assign6200_e4123_d_n10, assign6200_e4123_d_n11, assign6200_e4123_d_n12, assign6200_e4123_d_n17,) = {
    if ((var_guard74 != 0.0) && (var_guard75 != 0.0)) {
        let assign6200_e4119: f64 = (var_tmf1 + var_tmf2);
        let assign6200_e4120: f64 = (0.5 * assign6200_e4119);
        let assign6200_e4121: f64 = (var_pb20 - assign6200_e4120);
        (assign6200_e4121, (var_pb20_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_pb20_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_pb20_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_pb20_dn7 - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (var_pb20_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_pb20_dn11 - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (var_pb20_dn12 - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), (var_pb20_dn17 - (0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_vbsz2, var_vbsz2_dn0, var_vbsz2_dn2, var_vbsz2_dn6, var_vbsz2_dn7, var_vbsz2_dn10, var_vbsz2_dn11, var_vbsz2_dn12, var_vbsz2_dn17,)
    }
};
        var_vbsz2 = assign6200_e4123;
        var_vbsz2_dn0 = assign6200_e4123_d_n0;
        var_vbsz2_dn2 = assign6200_e4123_d_n2;
        var_vbsz2_dn6 = assign6200_e4123_d_n6;
        var_vbsz2_dn7 = assign6200_e4123_d_n7;
        var_vbsz2_dn10 = assign6200_e4123_d_n10;
        var_vbsz2_dn11 = assign6200_e4123_d_n11;
        var_vbsz2_dn12 = assign6200_e4123_d_n12;
        var_vbsz2_dn17 = assign6200_e4123_d_n17;

        let (assign6210_e4128, assign6210_e4128_d_n0, assign6210_e4128_d_n2, assign6210_e4128_d_n6, assign6210_e4128_d_n7, assign6210_e4128_d_n10, assign6210_e4128_d_n11, assign6210_e4128_d_n12, assign6210_e4128_d_n17,) = {
    if (var_guard74 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vbsz2, var_vbsz2_dn0, var_vbsz2_dn2, var_vbsz2_dn6, var_vbsz2_dn7, var_vbsz2_dn10, var_vbsz2_dn11, var_vbsz2_dn12, var_vbsz2_dn17,)
    }
};
        var_vbsz2 = assign6210_e4128;
        var_vbsz2_dn0 = assign6210_e4128_d_n0;
        var_vbsz2_dn2 = assign6210_e4128_d_n2;
        var_vbsz2_dn6 = assign6210_e4128_d_n6;
        var_vbsz2_dn7 = assign6210_e4128_d_n7;
        var_vbsz2_dn10 = assign6210_e4128_d_n10;
        var_vbsz2_dn11 = assign6210_e4128_d_n11;
        var_vbsz2_dn12 = assign6210_e4128_d_n12;
        var_vbsz2_dn17 = assign6210_e4128_d_n17;

        let assign6220_e4131: f64 = if var_subversion < 3.0 { 1.0 } else { 0.0 };
        var_guard76 = assign6220_e4131;

        let (assign6230_e4135, assign6230_e4135_d_n0, assign6230_e4135_d_n2, assign6230_e4135_d_n6, assign6230_e4135_d_n7, assign6230_e4135_d_n10, assign6230_e4135_d_n11, assign6230_e4135_d_n12, assign6230_e4135_d_n17,) = {
    if (var_guard76 != 0.0) {
        (p.p237, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_wd0, var_wd0_dn0, var_wd0_dn2, var_wd0_dn6, var_wd0_dn7, var_wd0_dn10, var_wd0_dn11, var_wd0_dn12, var_wd0_dn17,)
    }
};
        var_wd0 = assign6230_e4135;
        var_wd0_dn0 = assign6230_e4135_d_n0;
        var_wd0_dn2 = assign6230_e4135_d_n2;
        var_wd0_dn6 = assign6230_e4135_d_n6;
        var_wd0_dn7 = assign6230_e4135_d_n7;
        var_wd0_dn10 = assign6230_e4135_d_n10;
        var_wd0_dn11 = assign6230_e4135_d_n11;
        var_wd0_dn12 = assign6230_e4135_d_n12;
        var_wd0_dn17 = assign6230_e4135_d_n17;

        let (assign6240_e4144, assign6240_e4144_d_n0, assign6240_e4144_d_n2, assign6240_e4144_d_n6, assign6240_e4144_d_n7, assign6240_e4144_d_n10, assign6240_e4144_d_n11, assign6240_e4144_d_n12, assign6240_e4144_d_n17,) = {
    if (var_guard76 == 0.0) {
        let assign6240_e4140: f64 = (2.0 * 1.034943e-10);
        let assign6240_e4142: f64 = (assign6240_e4140 / var_q_nsub);
        (assign6240_e4142, (-((assign6240_e4140 * var_q_nsub_dn0) / (var_q_nsub * var_q_nsub))), (-((assign6240_e4140 * var_q_nsub_dn2) / (var_q_nsub * var_q_nsub))), (-((assign6240_e4140 * var_q_nsub_dn6) / (var_q_nsub * var_q_nsub))), (-((assign6240_e4140 * var_q_nsub_dn7) / (var_q_nsub * var_q_nsub))), (-((assign6240_e4140 * var_q_nsub_dn10) / (var_q_nsub * var_q_nsub))), (-((assign6240_e4140 * var_q_nsub_dn11) / (var_q_nsub * var_q_nsub))), (-((assign6240_e4140 * var_q_nsub_dn12) / (var_q_nsub * var_q_nsub))), (-((assign6240_e4140 * var_q_nsub_dn17) / (var_q_nsub * var_q_nsub))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign6240_e4144;
        var_t1_dn0 = assign6240_e4144_d_n0;
        var_t1_dn2 = assign6240_e4144_d_n2;
        var_t1_dn6 = assign6240_e4144_d_n6;
        var_t1_dn7 = assign6240_e4144_d_n7;
        var_t1_dn10 = assign6240_e4144_d_n10;
        var_t1_dn11 = assign6240_e4144_d_n11;
        var_t1_dn12 = assign6240_e4144_d_n12;
        var_t1_dn17 = assign6240_e4144_d_n17;

        let (assign6250_e4154, assign6250_e4154_d_n0, assign6250_e4154_d_n2, assign6250_e4154_d_n6, assign6250_e4154_d_n7, assign6250_e4154_d_n10, assign6250_e4154_d_n11, assign6250_e4154_d_n12, assign6250_e4154_d_n17,) = {
    if (var_guard76 == 0.0) {
        let assign6250_e4150: f64 = (var_pb20 - var_vbsz2);
        let assign6250_e4151: f64 = (var_t1 * assign6250_e4150);
        let assign6250_e4152: f64 = (assign6250_e4151).sqrt();
        (assign6250_e4152, (((var_t1_dn0 * assign6250_e4150) + (var_t1 * (var_pb20_dn0 - var_vbsz2_dn0))) / (2.0 * assign6250_e4152)), (((var_t1_dn2 * assign6250_e4150) + (var_t1 * (var_pb20_dn2 - var_vbsz2_dn2))) / (2.0 * assign6250_e4152)), (((var_t1_dn6 * assign6250_e4150) + (var_t1 * (var_pb20_dn6 - var_vbsz2_dn6))) / (2.0 * assign6250_e4152)), (((var_t1_dn7 * assign6250_e4150) + (var_t1 * (var_pb20_dn7 - var_vbsz2_dn7))) / (2.0 * assign6250_e4152)), (((var_t1_dn10 * assign6250_e4150) + (var_t1 * (var_pb20_dn10 - var_vbsz2_dn10))) / (2.0 * assign6250_e4152)), (((var_t1_dn11 * assign6250_e4150) + (var_t1 * (var_pb20_dn11 - var_vbsz2_dn11))) / (2.0 * assign6250_e4152)), (((var_t1_dn12 * assign6250_e4150) + (var_t1 * (var_pb20_dn12 - var_vbsz2_dn12))) / (2.0 * assign6250_e4152)), (((var_t1_dn17 * assign6250_e4150) + (var_t1 * (var_pb20_dn17 - var_vbsz2_dn17))) / (2.0 * assign6250_e4152)),)
    } else {
        (var_wd0, var_wd0_dn0, var_wd0_dn2, var_wd0_dn6, var_wd0_dn7, var_wd0_dn10, var_wd0_dn11, var_wd0_dn12, var_wd0_dn17,)
    }
};
        var_wd0 = assign6250_e4154;
        var_wd0_dn0 = assign6250_e4154_d_n0;
        var_wd0_dn2 = assign6250_e4154_d_n2;
        var_wd0_dn6 = assign6250_e4154_d_n6;
        var_wd0_dn7 = assign6250_e4154_d_n7;
        var_wd0_dn10 = assign6250_e4154_d_n10;
        var_wd0_dn11 = assign6250_e4154_d_n11;
        var_wd0_dn12 = assign6250_e4154_d_n12;
        var_wd0_dn17 = assign6250_e4154_d_n17;

        let (assign6260_e4168, assign6260_e4168_d_n0, assign6260_e4168_d_n2, assign6260_e4168_d_n6, assign6260_e4168_d_n7, assign6260_e4168_d_n10, assign6260_e4168_d_n11, assign6260_e4168_d_n12, assign6260_e4168_d_n17,) = {
    if (var_subversion < 3.0) {
        let assign6260_e4160: f64 = (var_qnsub_esi2 * var_pb20);
        let assign6260_e4161: f64 = (assign6260_e4160).sqrt();
        (assign6260_e4161, (((var_qnsub_esi2_dn0 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn0)) / (2.0 * assign6260_e4161)), (((var_qnsub_esi2_dn2 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn2)) / (2.0 * assign6260_e4161)), (((var_qnsub_esi2_dn6 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn6)) / (2.0 * assign6260_e4161)), (((var_qnsub_esi2_dn7 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn7)) / (2.0 * assign6260_e4161)), (((var_qnsub_esi2_dn10 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn10)) / (2.0 * assign6260_e4161)), (((var_qnsub_esi2_dn11 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn11)) / (2.0 * assign6260_e4161)), (((var_qnsub_esi2_dn12 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn12)) / (2.0 * assign6260_e4161)), (((var_qnsub_esi2_dn17 * var_pb20) + (var_qnsub_esi2 * var_pb20_dn17)) / (2.0 * assign6260_e4161)),)
    } else {
        let assign6260_e4165: f64 = (var_pb20 - var_vbsz2);
        let assign6260_e4166: f64 = (var_qnsub_esi2 * assign6260_e4165);
        let assign6260_e4167: f64 = (assign6260_e4166).sqrt();
        (assign6260_e4167, (((var_qnsub_esi2_dn0 * assign6260_e4165) + (var_qnsub_esi2 * (var_pb20_dn0 - var_vbsz2_dn0))) / (2.0 * assign6260_e4167)), (((var_qnsub_esi2_dn2 * assign6260_e4165) + (var_qnsub_esi2 * (var_pb20_dn2 - var_vbsz2_dn2))) / (2.0 * assign6260_e4167)), (((var_qnsub_esi2_dn6 * assign6260_e4165) + (var_qnsub_esi2 * (var_pb20_dn6 - var_vbsz2_dn6))) / (2.0 * assign6260_e4167)), (((var_qnsub_esi2_dn7 * assign6260_e4165) + (var_qnsub_esi2 * (var_pb20_dn7 - var_vbsz2_dn7))) / (2.0 * assign6260_e4167)), (((var_qnsub_esi2_dn10 * assign6260_e4165) + (var_qnsub_esi2 * (var_pb20_dn10 - var_vbsz2_dn10))) / (2.0 * assign6260_e4167)), (((var_qnsub_esi2_dn11 * assign6260_e4165) + (var_qnsub_esi2 * (var_pb20_dn11 - var_vbsz2_dn11))) / (2.0 * assign6260_e4167)), (((var_qnsub_esi2_dn12 * assign6260_e4165) + (var_qnsub_esi2 * (var_pb20_dn12 - var_vbsz2_dn12))) / (2.0 * assign6260_e4167)), (((var_qnsub_esi2_dn17 * assign6260_e4165) + (var_qnsub_esi2 * (var_pb20_dn17 - var_vbsz2_dn17))) / (2.0 * assign6260_e4167)),)
    }
};
        var_qb0 = assign6260_e4168;
        var_qb0_dn0 = assign6260_e4168_d_n0;
        var_qb0_dn2 = assign6260_e4168_d_n2;
        var_qb0_dn6 = assign6260_e4168_d_n6;
        var_qb0_dn7 = assign6260_e4168_d_n7;
        var_qb0_dn10 = assign6260_e4168_d_n10;
        var_qb0_dn11 = assign6260_e4168_d_n11;
        var_qb0_dn12 = assign6260_e4168_d_n12;
        var_qb0_dn17 = assign6260_e4168_d_n17;

        let assign6270_e4171: f64 = (var_pb20 + var_vfb);
        let assign6270_e4174: f64 = (var_qb0 * var_c_fox_inv);
        let assign6270_e4175: f64 = (assign6270_e4171 + assign6270_e4174);
        let assign6270_e4177: f64 = (assign6270_e4175 + var_ptovr);
        var_vthp = assign6270_e4177;
        var_vthp_dn0 = ((var_pb20_dn0 + ((var_qb0_dn0 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn0))) + var_ptovr_dn0);
        var_vthp_dn2 = ((var_pb20_dn2 + ((var_qb0_dn2 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn2))) + var_ptovr_dn2);
        var_vthp_dn6 = ((var_pb20_dn6 + ((var_qb0_dn6 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn6))) + var_ptovr_dn6);
        var_vthp_dn7 = ((var_pb20_dn7 + ((var_qb0_dn7 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn7))) + var_ptovr_dn7);
        var_vthp_dn10 = ((var_pb20_dn10 + ((var_qb0_dn10 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn10))) + var_ptovr_dn10);
        var_vthp_dn11 = ((var_pb20_dn11 + ((var_qb0_dn11 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn11))) + var_ptovr_dn11);
        var_vthp_dn12 = ((var_pb20_dn12 + ((var_qb0_dn12 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn12))) + var_ptovr_dn12);
        var_vthp_dn17 = ((var_pb20_dn17 + ((var_qb0_dn17 * var_c_fox_inv) + (var_qb0 * var_c_fox_inv_dn17))) + var_ptovr_dn17);

        var_pb20b = var_pb20;
        var_pb20b_dn0 = var_pb20_dn0;
        var_pb20b_dn2 = var_pb20_dn2;
        var_pb20b_dn6 = var_pb20_dn6;
        var_pb20b_dn7 = var_pb20_dn7;
        var_pb20b_dn10 = var_pb20_dn10;
        var_pb20b_dn11 = var_pb20_dn11;
        var_pb20b_dn12 = var_pb20_dn12;
        var_pb20b_dn17 = var_pb20_dn17;

        var_t0__blk78 = 0.95;

        let assign6300_e4182: f64 = (var_t0__blk78 * var_pb20b);
        let assign6300_e4184: f64 = (assign6300_e4182 - var_vbsz2);
        let assign6300_e4186: f64 = (assign6300_e4184 - 0.001);
        var_t1__blk77 = assign6300_e4186;
        var_t1__blk77_dn0 = ((var_t0__blk78 * var_pb20b_dn0) - var_vbsz2_dn0);
        var_t1__blk77_dn2 = ((var_t0__blk78 * var_pb20b_dn2) - var_vbsz2_dn2);
        var_t1__blk77_dn6 = ((var_t0__blk78 * var_pb20b_dn6) - var_vbsz2_dn6);
        var_t1__blk77_dn7 = ((var_t0__blk78 * var_pb20b_dn7) - var_vbsz2_dn7);
        var_t1__blk77_dn10 = ((var_t0__blk78 * var_pb20b_dn10) - var_vbsz2_dn10);
        var_t1__blk77_dn11 = ((var_t0__blk78 * var_pb20b_dn11) - var_vbsz2_dn11);
        var_t1__blk77_dn12 = ((var_t0__blk78 * var_pb20b_dn12) - var_vbsz2_dn12);
        var_t1__blk77_dn17 = ((var_t0__blk78 * var_pb20b_dn17) - var_vbsz2_dn17);

        let assign6310_e4189: f64 = (var_t1__blk77 * var_t1__blk77);
        let assign6310_e4192: f64 = (4.0 * var_t0__blk78);
        let assign6310_e4194: f64 = (assign6310_e4192 * var_pb20b);
        let assign6310_e4196: f64 = (assign6310_e4194 * 0.001);
        let assign6310_e4197: f64 = (assign6310_e4189 + assign6310_e4196);
        let assign6310_e4198: f64 = (assign6310_e4197).sqrt();
        var_t2__blk79 = assign6310_e4198;
        var_t2__blk79_dn0 = ((((var_t1__blk77_dn0 * var_t1__blk77) + (var_t1__blk77 * var_t1__blk77_dn0)) + ((assign6310_e4192 * var_pb20b_dn0) * 0.001)) / (2.0 * assign6310_e4198));
        var_t2__blk79_dn2 = ((((var_t1__blk77_dn2 * var_t1__blk77) + (var_t1__blk77 * var_t1__blk77_dn2)) + ((assign6310_e4192 * var_pb20b_dn2) * 0.001)) / (2.0 * assign6310_e4198));
        var_t2__blk79_dn6 = ((((var_t1__blk77_dn6 * var_t1__blk77) + (var_t1__blk77 * var_t1__blk77_dn6)) + ((assign6310_e4192 * var_pb20b_dn6) * 0.001)) / (2.0 * assign6310_e4198));
        var_t2__blk79_dn7 = ((((var_t1__blk77_dn7 * var_t1__blk77) + (var_t1__blk77 * var_t1__blk77_dn7)) + ((assign6310_e4192 * var_pb20b_dn7) * 0.001)) / (2.0 * assign6310_e4198));
        var_t2__blk79_dn10 = ((((var_t1__blk77_dn10 * var_t1__blk77) + (var_t1__blk77 * var_t1__blk77_dn10)) + ((assign6310_e4192 * var_pb20b_dn10) * 0.001)) / (2.0 * assign6310_e4198));
        var_t2__blk79_dn11 = ((((var_t1__blk77_dn11 * var_t1__blk77) + (var_t1__blk77 * var_t1__blk77_dn11)) + ((assign6310_e4192 * var_pb20b_dn11) * 0.001)) / (2.0 * assign6310_e4198));
        var_t2__blk79_dn12 = ((((var_t1__blk77_dn12 * var_t1__blk77) + (var_t1__blk77 * var_t1__blk77_dn12)) + ((assign6310_e4192 * var_pb20b_dn12) * 0.001)) / (2.0 * assign6310_e4198));
        var_t2__blk79_dn17 = ((((var_t1__blk77_dn17 * var_t1__blk77) + (var_t1__blk77 * var_t1__blk77_dn17)) + ((assign6310_e4192 * var_pb20b_dn17) * 0.001)) / (2.0 * assign6310_e4198));

        let assign6320_e4201: f64 = (var_t0__blk78 * var_pb20b);
        let assign6320_e4205: f64 = (var_t1__blk77 + var_t2__blk79);
        let assign6320_e4206: f64 = (0.5 * assign6320_e4205);
        let assign6320_e4207: f64 = (assign6320_e4201 - assign6320_e4206);
        var_t3__blk80 = assign6320_e4207;
        var_t3__blk80_dn0 = ((var_t0__blk78 * var_pb20b_dn0) - (0.5 * (var_t1__blk77_dn0 + var_t2__blk79_dn0)));
        var_t3__blk80_dn2 = ((var_t0__blk78 * var_pb20b_dn2) - (0.5 * (var_t1__blk77_dn2 + var_t2__blk79_dn2)));
        var_t3__blk80_dn6 = ((var_t0__blk78 * var_pb20b_dn6) - (0.5 * (var_t1__blk77_dn6 + var_t2__blk79_dn6)));
        var_t3__blk80_dn7 = ((var_t0__blk78 * var_pb20b_dn7) - (0.5 * (var_t1__blk77_dn7 + var_t2__blk79_dn7)));
        var_t3__blk80_dn10 = ((var_t0__blk78 * var_pb20b_dn10) - (0.5 * (var_t1__blk77_dn10 + var_t2__blk79_dn10)));
        var_t3__blk80_dn11 = ((var_t0__blk78 * var_pb20b_dn11) - (0.5 * (var_t1__blk77_dn11 + var_t2__blk79_dn11)));
        var_t3__blk80_dn12 = ((var_t0__blk78 * var_pb20b_dn12) - (0.5 * (var_t1__blk77_dn12 + var_t2__blk79_dn12)));
        var_t3__blk80_dn17 = ((var_t0__blk78 * var_pb20b_dn17) - (0.5 * (var_t1__blk77_dn17 + var_t2__blk79_dn17)));

        let assign6330_e4210: f64 = (var_pb20b - var_t3__blk80);
        var_pbsum = assign6330_e4210;
        var_pbsum_dn0 = (var_pb20b_dn0 - var_t3__blk80_dn0);
        var_pbsum_dn2 = (var_pb20b_dn2 - var_t3__blk80_dn2);
        var_pbsum_dn6 = (var_pb20b_dn6 - var_t3__blk80_dn6);
        var_pbsum_dn7 = (var_pb20b_dn7 - var_t3__blk80_dn7);
        var_pbsum_dn10 = (var_pb20b_dn10 - var_t3__blk80_dn10);
        var_pbsum_dn11 = (var_pb20b_dn11 - var_t3__blk80_dn11);
        var_pbsum_dn12 = (var_pb20b_dn12 - var_t3__blk80_dn12);
        var_pbsum_dn17 = (var_pb20b_dn17 - var_t3__blk80_dn17);

        let assign6340_e4212: f64 = (var_pbsum).sqrt();
        var_sqrt_pbsum = assign6340_e4212;
        var_sqrt_pbsum_dn0 = (var_pbsum_dn0 / (2.0 * assign6340_e4212));
        var_sqrt_pbsum_dn2 = (var_pbsum_dn2 / (2.0 * assign6340_e4212));
        var_sqrt_pbsum_dn6 = (var_pbsum_dn6 / (2.0 * assign6340_e4212));
        var_sqrt_pbsum_dn7 = (var_pbsum_dn7 / (2.0 * assign6340_e4212));
        var_sqrt_pbsum_dn10 = (var_pbsum_dn10 / (2.0 * assign6340_e4212));
        var_sqrt_pbsum_dn11 = (var_pbsum_dn11 / (2.0 * assign6340_e4212));
        var_sqrt_pbsum_dn12 = (var_pbsum_dn12 / (2.0 * assign6340_e4212));
        var_sqrt_pbsum_dn17 = (var_pbsum_dn17 / (2.0 * assign6340_e4212));

        let assign6350_e4215: f64 = if p.p72 != 0.0 { 1.0 } else { 0.0 };
        var_guard88 = assign6350_e4215;

        let (assign6360_e4225, assign6360_e4225_d_n0, assign6360_e4225_d_n2, assign6360_e4225_d_n6, assign6360_e4225_d_n7, assign6360_e4225_d_n10, assign6360_e4225_d_n11, assign6360_e4225_d_n12, assign6360_e4225_d_n17,) = {
    if (var_guard88 != 0.0) {
        let assign6360_e4219: f64 = (2.0 * 1.6021918e-19);
        let assign6360_e4221: f64 = (assign6360_e4219 * var_uc_nsubs);
        let assign6360_e4223: f64 = (assign6360_e4221 * 1.034943e-10);
        (assign6360_e4223, ((assign6360_e4219 * var_uc_nsubs_dn0) * 1.034943e-10), ((assign6360_e4219 * var_uc_nsubs_dn2) * 1.034943e-10), ((assign6360_e4219 * var_uc_nsubs_dn6) * 1.034943e-10), ((assign6360_e4219 * var_uc_nsubs_dn7) * 1.034943e-10), ((assign6360_e4219 * var_uc_nsubs_dn10) * 1.034943e-10), ((assign6360_e4219 * var_uc_nsubs_dn11) * 1.034943e-10), ((assign6360_e4219 * var_uc_nsubs_dn12) * 1.034943e-10), ((assign6360_e4219 * var_uc_nsubs_dn17) * 1.034943e-10),)
    } else {
        (var_t1__blk82, var_t1__blk82_dn0, var_t1__blk82_dn2, var_t1__blk82_dn6, var_t1__blk82_dn7, var_t1__blk82_dn10, var_t1__blk82_dn11, var_t1__blk82_dn12, var_t1__blk82_dn17,)
    }
};
        var_t1__blk82 = assign6360_e4225;
        var_t1__blk82_dn0 = assign6360_e4225_d_n0;
        var_t1__blk82_dn2 = assign6360_e4225_d_n2;
        var_t1__blk82_dn6 = assign6360_e4225_d_n6;
        var_t1__blk82_dn7 = assign6360_e4225_d_n7;
        var_t1__blk82_dn10 = assign6360_e4225_d_n10;
        var_t1__blk82_dn11 = assign6360_e4225_d_n11;
        var_t1__blk82_dn12 = assign6360_e4225_d_n12;
        var_t1__blk82_dn17 = assign6360_e4225_d_n17;

        let (assign6370_e4242, assign6370_e4242_d_n0, assign6370_e4242_d_n2, assign6370_e4242_d_n6, assign6370_e4242_d_n7, assign6370_e4242_d_n10, assign6370_e4242_d_n11, assign6370_e4242_d_n12, assign6370_e4242_d_n17,) = {
    if (var_guard88 != 0.0) {
        let (assign6370_e4240, assign6370_e4240_d_n0, assign6370_e4240_d_n2, assign6370_e4240_d_n6, assign6370_e4240_d_n7, assign6370_e4240_d_n10, assign6370_e4240_d_n11, assign6370_e4240_d_n12, assign6370_e4240_d_n17,) = {
            if (var_subversion < 3.0) {
                let assign6370_e4232: f64 = (var_t1__blk82 * var_pb2c);
                let assign6370_e4233: f64 = (assign6370_e4232).sqrt();
                (assign6370_e4233, (((var_t1__blk82_dn0 * var_pb2c) + (var_t1__blk82 * var_pb2c_dn0)) / (2.0 * assign6370_e4233)), (((var_t1__blk82_dn2 * var_pb2c) + (var_t1__blk82 * var_pb2c_dn2)) / (2.0 * assign6370_e4233)), (((var_t1__blk82_dn6 * var_pb2c) + (var_t1__blk82 * var_pb2c_dn6)) / (2.0 * assign6370_e4233)), (((var_t1__blk82_dn7 * var_pb2c) + (var_t1__blk82 * var_pb2c_dn7)) / (2.0 * assign6370_e4233)), (((var_t1__blk82_dn10 * var_pb2c) + (var_t1__blk82 * var_pb2c_dn10)) / (2.0 * assign6370_e4233)), (((var_t1__blk82_dn11 * var_pb2c) + (var_t1__blk82 * var_pb2c_dn11)) / (2.0 * assign6370_e4233)), (((var_t1__blk82_dn12 * var_pb2c) + (var_t1__blk82 * var_pb2c_dn12)) / (2.0 * assign6370_e4233)), (((var_t1__blk82_dn17 * var_pb2c) + (var_t1__blk82 * var_pb2c_dn17)) / (2.0 * assign6370_e4233)),)
            } else {
                let assign6370_e4237: f64 = (var_pb2c - var_vbsz2);
                let assign6370_e4238: f64 = (var_t1__blk82 * assign6370_e4237);
                let assign6370_e4239: f64 = (assign6370_e4238).sqrt();
                (assign6370_e4239, (((var_t1__blk82_dn0 * assign6370_e4237) + (var_t1__blk82 * (var_pb2c_dn0 - var_vbsz2_dn0))) / (2.0 * assign6370_e4239)), (((var_t1__blk82_dn2 * assign6370_e4237) + (var_t1__blk82 * (var_pb2c_dn2 - var_vbsz2_dn2))) / (2.0 * assign6370_e4239)), (((var_t1__blk82_dn6 * assign6370_e4237) + (var_t1__blk82 * (var_pb2c_dn6 - var_vbsz2_dn6))) / (2.0 * assign6370_e4239)), (((var_t1__blk82_dn7 * assign6370_e4237) + (var_t1__blk82 * (var_pb2c_dn7 - var_vbsz2_dn7))) / (2.0 * assign6370_e4239)), (((var_t1__blk82_dn10 * assign6370_e4237) + (var_t1__blk82 * (var_pb2c_dn10 - var_vbsz2_dn10))) / (2.0 * assign6370_e4239)), (((var_t1__blk82_dn11 * assign6370_e4237) + (var_t1__blk82 * (var_pb2c_dn11 - var_vbsz2_dn11))) / (2.0 * assign6370_e4239)), (((var_t1__blk82_dn12 * assign6370_e4237) + (var_t1__blk82 * (var_pb2c_dn12 - var_vbsz2_dn12))) / (2.0 * assign6370_e4239)), (((var_t1__blk82_dn17 * assign6370_e4237) + (var_t1__blk82 * (var_pb2c_dn17 - var_vbsz2_dn17))) / (2.0 * assign6370_e4239)),)
            }
        };
        (assign6370_e4240, assign6370_e4240_d_n0, assign6370_e4240_d_n2, assign6370_e4240_d_n6, assign6370_e4240_d_n7, assign6370_e4240_d_n10, assign6370_e4240_d_n11, assign6370_e4240_d_n12, assign6370_e4240_d_n17,)
    } else {
        (var_t2__blk83, var_t2__blk83_dn0, var_t2__blk83_dn2, var_t2__blk83_dn6, var_t2__blk83_dn7, var_t2__blk83_dn10, var_t2__blk83_dn11, var_t2__blk83_dn12, var_t2__blk83_dn17,)
    }
};
        var_t2__blk83 = assign6370_e4242;
        var_t2__blk83_dn0 = assign6370_e4242_d_n0;
        var_t2__blk83_dn2 = assign6370_e4242_d_n2;
        var_t2__blk83_dn6 = assign6370_e4242_d_n6;
        var_t2__blk83_dn7 = assign6370_e4242_d_n7;
        var_t2__blk83_dn10 = assign6370_e4242_d_n10;
        var_t2__blk83_dn11 = assign6370_e4242_d_n11;
        var_t2__blk83_dn12 = assign6370_e4242_d_n12;
        var_t2__blk83_dn17 = assign6370_e4242_d_n17;

        let (assign6380_e4252, assign6380_e4252_d_n0, assign6380_e4252_d_n2, assign6380_e4252_d_n6, assign6380_e4252_d_n7, assign6380_e4252_d_n10, assign6380_e4252_d_n11, assign6380_e4252_d_n12, assign6380_e4252_d_n17,) = {
    if (var_guard88 != 0.0) {
        let assign6380_e4246: f64 = (var_pb2c + var_vfb);
        let assign6380_e4249: f64 = (var_t2__blk83 * var_c_fox_inv);
        let assign6380_e4250: f64 = (assign6380_e4246 + assign6380_e4249);
        (assign6380_e4250, (var_pb2c_dn0 + ((var_t2__blk83_dn0 * var_c_fox_inv) + (var_t2__blk83 * var_c_fox_inv_dn0))), (var_pb2c_dn2 + ((var_t2__blk83_dn2 * var_c_fox_inv) + (var_t2__blk83 * var_c_fox_inv_dn2))), (var_pb2c_dn6 + ((var_t2__blk83_dn6 * var_c_fox_inv) + (var_t2__blk83 * var_c_fox_inv_dn6))), (var_pb2c_dn7 + ((var_t2__blk83_dn7 * var_c_fox_inv) + (var_t2__blk83 * var_c_fox_inv_dn7))), (var_pb2c_dn10 + ((var_t2__blk83_dn10 * var_c_fox_inv) + (var_t2__blk83 * var_c_fox_inv_dn10))), (var_pb2c_dn11 + ((var_t2__blk83_dn11 * var_c_fox_inv) + (var_t2__blk83 * var_c_fox_inv_dn11))), (var_pb2c_dn12 + ((var_t2__blk83_dn12 * var_c_fox_inv) + (var_t2__blk83 * var_c_fox_inv_dn12))), (var_pb2c_dn17 + ((var_t2__blk83_dn17 * var_c_fox_inv) + (var_t2__blk83 * var_c_fox_inv_dn17))),)
    } else {
        (var_vth0, var_vth0_dn0, var_vth0_dn2, var_vth0_dn6, var_vth0_dn7, var_vth0_dn10, var_vth0_dn11, var_vth0_dn12, var_vth0_dn17,)
    }
};
        var_vth0 = assign6380_e4252;
        var_vth0_dn0 = assign6380_e4252_d_n0;
        var_vth0_dn2 = assign6380_e4252_d_n2;
        var_vth0_dn6 = assign6380_e4252_d_n6;
        var_vth0_dn7 = assign6380_e4252_d_n7;
        var_vth0_dn10 = assign6380_e4252_d_n10;
        var_vth0_dn11 = assign6380_e4252_d_n11;
        var_vth0_dn12 = assign6380_e4252_d_n12;
        var_vth0_dn17 = assign6380_e4252_d_n17;

        let (assign6390_e4258, assign6390_e4258_d_n0, assign6390_e4258_d_n2, assign6390_e4258_d_n6, assign6390_e4258_d_n7, assign6390_e4258_d_n10, assign6390_e4258_d_n11, assign6390_e4258_d_n12, assign6390_e4258_d_n17,) = {
    if (var_guard88 != 0.0) {
        let assign6390_e4256: f64 = (1.034943e-10 * var_c_fox_inv);
        (assign6390_e4256, (1.034943e-10 * var_c_fox_inv_dn0), (1.034943e-10 * var_c_fox_inv_dn2), (1.034943e-10 * var_c_fox_inv_dn6), (1.034943e-10 * var_c_fox_inv_dn7), (1.034943e-10 * var_c_fox_inv_dn10), (1.034943e-10 * var_c_fox_inv_dn11), (1.034943e-10 * var_c_fox_inv_dn12), (1.034943e-10 * var_c_fox_inv_dn17),)
    } else {
        (var_t1__blk82, var_t1__blk82_dn0, var_t1__blk82_dn2, var_t1__blk82_dn6, var_t1__blk82_dn7, var_t1__blk82_dn10, var_t1__blk82_dn11, var_t1__blk82_dn12, var_t1__blk82_dn17,)
    }
};
        var_t1__blk82 = assign6390_e4258;
        var_t1__blk82_dn0 = assign6390_e4258_d_n0;
        var_t1__blk82_dn2 = assign6390_e4258_d_n2;
        var_t1__blk82_dn6 = assign6390_e4258_d_n6;
        var_t1__blk82_dn7 = assign6390_e4258_d_n7;
        var_t1__blk82_dn10 = assign6390_e4258_d_n10;
        var_t1__blk82_dn11 = assign6390_e4258_d_n11;
        var_t1__blk82_dn12 = assign6390_e4258_d_n12;
        var_t1__blk82_dn17 = assign6390_e4258_d_n17;

        *var_guard75_slot = var_guard75;
        *var_guard76_slot = var_guard76;
        *var_guard88_slot = var_guard88;
        *var_pb20b_slot = var_pb20b;
        *var_pb20b_dn0_slot = var_pb20b_dn0;
        *var_pb20b_dn10_slot = var_pb20b_dn10;
        *var_pb20b_dn11_slot = var_pb20b_dn11;
        *var_pb20b_dn12_slot = var_pb20b_dn12;
        *var_pb20b_dn17_slot = var_pb20b_dn17;
        *var_pb20b_dn2_slot = var_pb20b_dn2;
        *var_pb20b_dn6_slot = var_pb20b_dn6;
        *var_pb20b_dn7_slot = var_pb20b_dn7;
        *var_pbsum_slot = var_pbsum;
        *var_pbsum_dn0_slot = var_pbsum_dn0;
        *var_pbsum_dn10_slot = var_pbsum_dn10;
        *var_pbsum_dn11_slot = var_pbsum_dn11;
        *var_pbsum_dn12_slot = var_pbsum_dn12;
        *var_pbsum_dn17_slot = var_pbsum_dn17;
        *var_pbsum_dn2_slot = var_pbsum_dn2;
        *var_pbsum_dn6_slot = var_pbsum_dn6;
        *var_pbsum_dn7_slot = var_pbsum_dn7;
        *var_qb0_slot = var_qb0;
        *var_qb0_dn0_slot = var_qb0_dn0;
        *var_qb0_dn10_slot = var_qb0_dn10;
        *var_qb0_dn11_slot = var_qb0_dn11;
        *var_qb0_dn12_slot = var_qb0_dn12;
        *var_qb0_dn17_slot = var_qb0_dn17;
        *var_qb0_dn2_slot = var_qb0_dn2;
        *var_qb0_dn6_slot = var_qb0_dn6;
        *var_qb0_dn7_slot = var_qb0_dn7;
        *var_sqrt_pbsum_slot = var_sqrt_pbsum;
        *var_sqrt_pbsum_dn0_slot = var_sqrt_pbsum_dn0;
        *var_sqrt_pbsum_dn10_slot = var_sqrt_pbsum_dn10;
        *var_sqrt_pbsum_dn11_slot = var_sqrt_pbsum_dn11;
        *var_sqrt_pbsum_dn12_slot = var_sqrt_pbsum_dn12;
        *var_sqrt_pbsum_dn17_slot = var_sqrt_pbsum_dn17;
        *var_sqrt_pbsum_dn2_slot = var_sqrt_pbsum_dn2;
        *var_sqrt_pbsum_dn6_slot = var_sqrt_pbsum_dn6;
        *var_sqrt_pbsum_dn7_slot = var_sqrt_pbsum_dn7;
        *var_t0__blk78_slot = var_t0__blk78;
        *var_t1_slot = var_t1;
        *var_t1__blk77_slot = var_t1__blk77;
        *var_t1__blk77_dn0_slot = var_t1__blk77_dn0;
        *var_t1__blk77_dn10_slot = var_t1__blk77_dn10;
        *var_t1__blk77_dn11_slot = var_t1__blk77_dn11;
        *var_t1__blk77_dn12_slot = var_t1__blk77_dn12;
        *var_t1__blk77_dn17_slot = var_t1__blk77_dn17;
        *var_t1__blk77_dn2_slot = var_t1__blk77_dn2;
        *var_t1__blk77_dn6_slot = var_t1__blk77_dn6;
        *var_t1__blk77_dn7_slot = var_t1__blk77_dn7;
        *var_t1__blk82_slot = var_t1__blk82;
        *var_t1__blk82_dn0_slot = var_t1__blk82_dn0;
        *var_t1__blk82_dn10_slot = var_t1__blk82_dn10;
        *var_t1__blk82_dn11_slot = var_t1__blk82_dn11;
        *var_t1__blk82_dn12_slot = var_t1__blk82_dn12;
        *var_t1__blk82_dn17_slot = var_t1__blk82_dn17;
        *var_t1__blk82_dn2_slot = var_t1__blk82_dn2;
        *var_t1__blk82_dn6_slot = var_t1__blk82_dn6;
        *var_t1__blk82_dn7_slot = var_t1__blk82_dn7;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t2__blk79_slot = var_t2__blk79;
        *var_t2__blk79_dn0_slot = var_t2__blk79_dn0;
        *var_t2__blk79_dn10_slot = var_t2__blk79_dn10;
        *var_t2__blk79_dn11_slot = var_t2__blk79_dn11;
        *var_t2__blk79_dn12_slot = var_t2__blk79_dn12;
        *var_t2__blk79_dn17_slot = var_t2__blk79_dn17;
        *var_t2__blk79_dn2_slot = var_t2__blk79_dn2;
        *var_t2__blk79_dn6_slot = var_t2__blk79_dn6;
        *var_t2__blk79_dn7_slot = var_t2__blk79_dn7;
        *var_t2__blk83_slot = var_t2__blk83;
        *var_t2__blk83_dn0_slot = var_t2__blk83_dn0;
        *var_t2__blk83_dn10_slot = var_t2__blk83_dn10;
        *var_t2__blk83_dn11_slot = var_t2__blk83_dn11;
        *var_t2__blk83_dn12_slot = var_t2__blk83_dn12;
        *var_t2__blk83_dn17_slot = var_t2__blk83_dn17;
        *var_t2__blk83_dn2_slot = var_t2__blk83_dn2;
        *var_t2__blk83_dn6_slot = var_t2__blk83_dn6;
        *var_t2__blk83_dn7_slot = var_t2__blk83_dn7;
        *var_t3__blk80_slot = var_t3__blk80;
        *var_t3__blk80_dn0_slot = var_t3__blk80_dn0;
        *var_t3__blk80_dn10_slot = var_t3__blk80_dn10;
        *var_t3__blk80_dn11_slot = var_t3__blk80_dn11;
        *var_t3__blk80_dn12_slot = var_t3__blk80_dn12;
        *var_t3__blk80_dn17_slot = var_t3__blk80_dn17;
        *var_t3__blk80_dn2_slot = var_t3__blk80_dn2;
        *var_t3__blk80_dn6_slot = var_t3__blk80_dn6;
        *var_t3__blk80_dn7_slot = var_t3__blk80_dn7;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn17_slot = var_tmf2_dn17;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_vbslim_slot = var_vbslim;
        *var_vbslim_dn0_slot = var_vbslim_dn0;
        *var_vbslim_dn10_slot = var_vbslim_dn10;
        *var_vbslim_dn11_slot = var_vbslim_dn11;
        *var_vbslim_dn12_slot = var_vbslim_dn12;
        *var_vbslim_dn17_slot = var_vbslim_dn17;
        *var_vbslim_dn2_slot = var_vbslim_dn2;
        *var_vbslim_dn6_slot = var_vbslim_dn6;
        *var_vbslim_dn7_slot = var_vbslim_dn7;
        *var_vbsz2_slot = var_vbsz2;
        *var_vbsz2_dn0_slot = var_vbsz2_dn0;
        *var_vbsz2_dn10_slot = var_vbsz2_dn10;
        *var_vbsz2_dn11_slot = var_vbsz2_dn11;
        *var_vbsz2_dn12_slot = var_vbsz2_dn12;
        *var_vbsz2_dn17_slot = var_vbsz2_dn17;
        *var_vbsz2_dn2_slot = var_vbsz2_dn2;
        *var_vbsz2_dn6_slot = var_vbsz2_dn6;
        *var_vbsz2_dn7_slot = var_vbsz2_dn7;
        *var_vth0_slot = var_vth0;
        *var_vth0_dn0_slot = var_vth0_dn0;
        *var_vth0_dn10_slot = var_vth0_dn10;
        *var_vth0_dn11_slot = var_vth0_dn11;
        *var_vth0_dn12_slot = var_vth0_dn12;
        *var_vth0_dn17_slot = var_vth0_dn17;
        *var_vth0_dn2_slot = var_vth0_dn2;
        *var_vth0_dn6_slot = var_vth0_dn6;
        *var_vth0_dn7_slot = var_vth0_dn7;
        *var_vthp_slot = var_vthp;
        *var_vthp_dn0_slot = var_vthp_dn0;
        *var_vthp_dn10_slot = var_vthp_dn10;
        *var_vthp_dn11_slot = var_vthp_dn11;
        *var_vthp_dn12_slot = var_vthp_dn12;
        *var_vthp_dn17_slot = var_vthp_dn17;
        *var_vthp_dn2_slot = var_vthp_dn2;
        *var_vthp_dn6_slot = var_vthp_dn6;
        *var_vthp_dn7_slot = var_vthp_dn7;
        *var_wd0_slot = var_wd0;
        *var_wd0_dn0_slot = var_wd0_dn0;
        *var_wd0_dn10_slot = var_wd0_dn10;
        *var_wd0_dn11_slot = var_wd0_dn11;
        *var_wd0_dn12_slot = var_wd0_dn12;
        *var_wd0_dn17_slot = var_wd0_dn17;
        *var_wd0_dn2_slot = var_wd0_dn2;
        *var_wd0_dn6_slot = var_wd0_dn6;
        *var_wd0_dn7_slot = var_wd0_dn7;
    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        var_c_fox: f64,
        var_c_fox_dn0: f64,
        var_c_fox_dn10: f64,
        var_c_fox_dn11: f64,
        var_c_fox_dn12: f64,
        var_c_fox_dn17: f64,
        var_c_fox_dn2: f64,
        var_c_fox_dn6: f64,
        var_c_fox_dn7: f64,
        var_c_fox_inv: f64,
        var_c_fox_inv_dn0: f64,
        var_c_fox_inv_dn10: f64,
        var_c_fox_inv_dn11: f64,
        var_c_fox_inv_dn12: f64,
        var_c_fox_inv_dn17: f64,
        var_c_fox_inv_dn2: f64,
        var_c_fox_inv_dn6: f64,
        var_c_fox_inv_dn7: f64,
        var_cnstpgd: f64,
        var_dvthsm: f64,
        var_eg: f64,
        var_eg_dn0: f64,
        var_eg_dn10: f64,
        var_eg_dn11: f64,
        var_eg_dn12: f64,
        var_eg_dn17: f64,
        var_eg_dn2: f64,
        var_eg_dn6: f64,
        var_eg_dn7: f64,
        var_guard88: f64,
        var_lgleff: f64,
        var_mks_parl1: f64,
        var_mks_wfc: f64,
        var_pb2: f64,
        var_pb20b: f64,
        var_pb20b_dn0: f64,
        var_pb20b_dn10: f64,
        var_pb20b_dn11: f64,
        var_pb20b_dn12: f64,
        var_pb20b_dn17: f64,
        var_pb20b_dn2: f64,
        var_pb20b_dn6: f64,
        var_pb20b_dn7: f64,
        var_pb2_dn0: f64,
        var_pb2_dn10: f64,
        var_pb2_dn11: f64,
        var_pb2_dn12: f64,
        var_pb2_dn17: f64,
        var_pb2_dn2: f64,
        var_pb2_dn6: f64,
        var_pb2_dn7: f64,
        var_pbsum: f64,
        var_pbsum_dn0: f64,
        var_pbsum_dn10: f64,
        var_pbsum_dn11: f64,
        var_pbsum_dn12: f64,
        var_pbsum_dn17: f64,
        var_pbsum_dn2: f64,
        var_pbsum_dn6: f64,
        var_pbsum_dn7: f64,
        var_qb0: f64,
        var_qb0_dn0: f64,
        var_qb0_dn10: f64,
        var_qb0_dn11: f64,
        var_qb0_dn12: f64,
        var_qb0_dn17: f64,
        var_qb0_dn2: f64,
        var_qb0_dn6: f64,
        var_qb0_dn7: f64,
        var_uc_sc2: f64,
        var_uc_sc3: f64,
        var_uc_scp2: f64,
        var_uc_scp3: f64,
        var_vdsz: f64,
        var_vdsz_dn0: f64,
        var_vdsz_dn10: f64,
        var_vdsz_dn11: f64,
        var_vdsz_dn12: f64,
        var_vdsz_dn17: f64,
        var_vdsz_dn2: f64,
        var_vdsz_dn6: f64,
        var_vdsz_dn7: f64,
        var_vgsz: f64,
        var_vgsz_dn0: f64,
        var_vgsz_dn10: f64,
        var_vgsz_dn11: f64,
        var_vgsz_dn12: f64,
        var_vgsz_dn17: f64,
        var_vgsz_dn2: f64,
        var_vgsz_dn6: f64,
        var_vgsz_dn7: f64,
        var_vth0: f64,
        var_vth0_dn0: f64,
        var_vth0_dn10: f64,
        var_vth0_dn11: f64,
        var_vth0_dn12: f64,
        var_vth0_dn17: f64,
        var_vth0_dn2: f64,
        var_vth0_dn6: f64,
        var_vth0_dn7: f64,
        var_vthp: f64,
        var_vthp_dn0: f64,
        var_vthp_dn10: f64,
        var_vthp_dn11: f64,
        var_vthp_dn12: f64,
        var_vthp_dn17: f64,
        var_vthp_dn2: f64,
        var_vthp_dn6: f64,
        var_vthp_dn7: f64,
        var_wd0: f64,
        var_wd0_dn0: f64,
        var_wd0_dn10: f64,
        var_wd0_dn11: f64,
        var_wd0_dn12: f64,
        var_wd0_dn17: f64,
        var_wd0_dn2: f64,
        var_wd0_dn6: f64,
        var_wd0_dn7: f64,
        var_weff: f64,
        var_wg: f64,
        var_dppg_slot: &mut f64,
        var_dppg_dn0_slot: &mut f64,
        var_dppg_dn10_slot: &mut f64,
        var_dppg_dn11_slot: &mut f64,
        var_dppg_dn12_slot: &mut f64,
        var_dppg_dn17_slot: &mut f64,
        var_dppg_dn2_slot: &mut f64,
        var_dppg_dn6_slot: &mut f64,
        var_dppg_dn7_slot: &mut f64,
        var_dvth_slot: &mut f64,
        var_dvth0__blk87_slot: &mut f64,
        var_dvth0__blk87_dn0_slot: &mut f64,
        var_dvth0__blk87_dn10_slot: &mut f64,
        var_dvth0__blk87_dn11_slot: &mut f64,
        var_dvth0__blk87_dn12_slot: &mut f64,
        var_dvth0__blk87_dn17_slot: &mut f64,
        var_dvth0__blk87_dn2_slot: &mut f64,
        var_dvth0__blk87_dn6_slot: &mut f64,
        var_dvth0__blk87_dn7_slot: &mut f64,
        var_dvth0__blk95_slot: &mut f64,
        var_dvth0__blk95_dn0_slot: &mut f64,
        var_dvth0__blk95_dn10_slot: &mut f64,
        var_dvth0__blk95_dn11_slot: &mut f64,
        var_dvth0__blk95_dn12_slot: &mut f64,
        var_dvth0__blk95_dn17_slot: &mut f64,
        var_dvth0__blk95_dn2_slot: &mut f64,
        var_dvth0__blk95_dn6_slot: &mut f64,
        var_dvth0__blk95_dn7_slot: &mut f64,
        var_dvth_dn0_slot: &mut f64,
        var_dvth_dn10_slot: &mut f64,
        var_dvth_dn11_slot: &mut f64,
        var_dvth_dn12_slot: &mut f64,
        var_dvth_dn17_slot: &mut f64,
        var_dvth_dn2_slot: &mut f64,
        var_dvth_dn6_slot: &mut f64,
        var_dvth_dn7_slot: &mut f64,
        var_dvthlp_slot: &mut f64,
        var_dvthlp_dn0_slot: &mut f64,
        var_dvthlp_dn10_slot: &mut f64,
        var_dvthlp_dn11_slot: &mut f64,
        var_dvthlp_dn12_slot: &mut f64,
        var_dvthlp_dn17_slot: &mut f64,
        var_dvthlp_dn2_slot: &mut f64,
        var_dvthlp_dn6_slot: &mut f64,
        var_dvthlp_dn7_slot: &mut f64,
        var_dvthsc_slot: &mut f64,
        var_dvthsc_dn0_slot: &mut f64,
        var_dvthsc_dn10_slot: &mut f64,
        var_dvthsc_dn11_slot: &mut f64,
        var_dvthsc_dn12_slot: &mut f64,
        var_dvthsc_dn17_slot: &mut f64,
        var_dvthsc_dn2_slot: &mut f64,
        var_dvthsc_dn6_slot: &mut f64,
        var_dvthsc_dn7_slot: &mut f64,
        var_dvthscr_slot: &mut f64,
        var_dvthscr_dn0_slot: &mut f64,
        var_dvthscr_dn10_slot: &mut f64,
        var_dvthscr_dn11_slot: &mut f64,
        var_dvthscr_dn12_slot: &mut f64,
        var_dvthscr_dn17_slot: &mut f64,
        var_dvthscr_dn2_slot: &mut f64,
        var_dvthscr_dn6_slot: &mut f64,
        var_dvthscr_dn7_slot: &mut f64,
        var_dvthw_slot: &mut f64,
        var_dvthw_dn0_slot: &mut f64,
        var_dvthw_dn10_slot: &mut f64,
        var_dvthw_dn11_slot: &mut f64,
        var_dvthw_dn12_slot: &mut f64,
        var_dvthw_dn17_slot: &mut f64,
        var_dvthw_dn2_slot: &mut f64,
        var_dvthw_dn6_slot: &mut f64,
        var_dvthw_dn7_slot: &mut f64,
        var_flg_dppg_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard99_slot: &mut f64,
        var_t0__blk104_slot: &mut f64,
        var_t0__blk81_slot: &mut f64,
        var_t0__blk89_slot: &mut f64,
        var_t0__blk89_dn0_slot: &mut f64,
        var_t0__blk89_dn10_slot: &mut f64,
        var_t0__blk89_dn11_slot: &mut f64,
        var_t0__blk89_dn12_slot: &mut f64,
        var_t0__blk89_dn17_slot: &mut f64,
        var_t0__blk89_dn2_slot: &mut f64,
        var_t0__blk89_dn6_slot: &mut f64,
        var_t0__blk89_dn7_slot: &mut f64,
        var_t1__blk100_slot: &mut f64,
        var_t1__blk100_dn0_slot: &mut f64,
        var_t1__blk100_dn10_slot: &mut f64,
        var_t1__blk100_dn11_slot: &mut f64,
        var_t1__blk100_dn12_slot: &mut f64,
        var_t1__blk100_dn17_slot: &mut f64,
        var_t1__blk100_dn2_slot: &mut f64,
        var_t1__blk100_dn6_slot: &mut f64,
        var_t1__blk100_dn7_slot: &mut f64,
        var_t1__blk82_slot: &mut f64,
        var_t1__blk82_dn0_slot: &mut f64,
        var_t1__blk82_dn10_slot: &mut f64,
        var_t1__blk82_dn11_slot: &mut f64,
        var_t1__blk82_dn12_slot: &mut f64,
        var_t1__blk82_dn17_slot: &mut f64,
        var_t1__blk82_dn2_slot: &mut f64,
        var_t1__blk82_dn6_slot: &mut f64,
        var_t1__blk82_dn7_slot: &mut f64,
        var_t1__blk90_slot: &mut f64,
        var_t1__blk90_dn0_slot: &mut f64,
        var_t1__blk90_dn10_slot: &mut f64,
        var_t1__blk90_dn11_slot: &mut f64,
        var_t1__blk90_dn12_slot: &mut f64,
        var_t1__blk90_dn17_slot: &mut f64,
        var_t1__blk90_dn2_slot: &mut f64,
        var_t1__blk90_dn6_slot: &mut f64,
        var_t1__blk90_dn7_slot: &mut f64,
        var_t1__blk96_slot: &mut f64,
        var_t1__blk96_dn0_slot: &mut f64,
        var_t1__blk96_dn10_slot: &mut f64,
        var_t1__blk96_dn11_slot: &mut f64,
        var_t1__blk96_dn12_slot: &mut f64,
        var_t1__blk96_dn17_slot: &mut f64,
        var_t1__blk96_dn2_slot: &mut f64,
        var_t1__blk96_dn6_slot: &mut f64,
        var_t1__blk96_dn7_slot: &mut f64,
        var_t2__blk83_slot: &mut f64,
        var_t2__blk83_dn0_slot: &mut f64,
        var_t2__blk83_dn10_slot: &mut f64,
        var_t2__blk83_dn11_slot: &mut f64,
        var_t2__blk83_dn12_slot: &mut f64,
        var_t2__blk83_dn17_slot: &mut f64,
        var_t2__blk83_dn2_slot: &mut f64,
        var_t2__blk83_dn6_slot: &mut f64,
        var_t2__blk83_dn7_slot: &mut f64,
        var_t2__blk91_slot: &mut f64,
        var_t2__blk91_dn0_slot: &mut f64,
        var_t2__blk91_dn10_slot: &mut f64,
        var_t2__blk91_dn11_slot: &mut f64,
        var_t2__blk91_dn12_slot: &mut f64,
        var_t2__blk91_dn17_slot: &mut f64,
        var_t2__blk91_dn2_slot: &mut f64,
        var_t2__blk91_dn6_slot: &mut f64,
        var_t2__blk91_dn7_slot: &mut f64,
        var_t2__blk97_slot: &mut f64,
        var_t3__blk101_slot: &mut f64,
        var_t3__blk101_dn0_slot: &mut f64,
        var_t3__blk101_dn10_slot: &mut f64,
        var_t3__blk101_dn11_slot: &mut f64,
        var_t3__blk101_dn12_slot: &mut f64,
        var_t3__blk101_dn17_slot: &mut f64,
        var_t3__blk101_dn2_slot: &mut f64,
        var_t3__blk101_dn6_slot: &mut f64,
        var_t3__blk101_dn7_slot: &mut f64,
        var_t3__blk84_slot: &mut f64,
        var_t3__blk84_dn0_slot: &mut f64,
        var_t3__blk84_dn10_slot: &mut f64,
        var_t3__blk84_dn11_slot: &mut f64,
        var_t3__blk84_dn12_slot: &mut f64,
        var_t3__blk84_dn17_slot: &mut f64,
        var_t3__blk84_dn2_slot: &mut f64,
        var_t3__blk84_dn6_slot: &mut f64,
        var_t3__blk84_dn7_slot: &mut f64,
        var_t3__blk92_slot: &mut f64,
        var_t3__blk98_slot: &mut f64,
        var_t4__blk85_slot: &mut f64,
        var_t4__blk93_slot: &mut f64,
        var_t4__blk93_dn0_slot: &mut f64,
        var_t4__blk93_dn10_slot: &mut f64,
        var_t4__blk93_dn11_slot: &mut f64,
        var_t4__blk93_dn12_slot: &mut f64,
        var_t4__blk93_dn17_slot: &mut f64,
        var_t4__blk93_dn2_slot: &mut f64,
        var_t4__blk93_dn6_slot: &mut f64,
        var_t4__blk93_dn7_slot: &mut f64,
        var_t5__blk102_slot: &mut f64,
        var_t5__blk102_dn0_slot: &mut f64,
        var_t5__blk102_dn10_slot: &mut f64,
        var_t5__blk102_dn11_slot: &mut f64,
        var_t5__blk102_dn12_slot: &mut f64,
        var_t5__blk102_dn17_slot: &mut f64,
        var_t5__blk102_dn2_slot: &mut f64,
        var_t5__blk102_dn6_slot: &mut f64,
        var_t5__blk102_dn7_slot: &mut f64,
        var_t5__blk86_slot: &mut f64,
        var_t5__blk86_dn0_slot: &mut f64,
        var_t5__blk86_dn10_slot: &mut f64,
        var_t5__blk86_dn11_slot: &mut f64,
        var_t5__blk86_dn12_slot: &mut f64,
        var_t5__blk86_dn17_slot: &mut f64,
        var_t5__blk86_dn2_slot: &mut f64,
        var_t5__blk86_dn6_slot: &mut f64,
        var_t5__blk86_dn7_slot: &mut f64,
        var_t5__blk94_slot: &mut f64,
        var_t5__blk94_dn0_slot: &mut f64,
        var_t5__blk94_dn10_slot: &mut f64,
        var_t5__blk94_dn11_slot: &mut f64,
        var_t5__blk94_dn12_slot: &mut f64,
        var_t5__blk94_dn17_slot: &mut f64,
        var_t5__blk94_dn2_slot: &mut f64,
        var_t5__blk94_dn6_slot: &mut f64,
        var_t5__blk94_dn7_slot: &mut f64,
        var_t7__blk103_slot: &mut f64,
        var_t7__blk103_dn0_slot: &mut f64,
        var_t7__blk103_dn10_slot: &mut f64,
        var_t7__blk103_dn11_slot: &mut f64,
        var_t7__blk103_dn12_slot: &mut f64,
        var_t7__blk103_dn17_slot: &mut f64,
        var_t7__blk103_dn2_slot: &mut f64,
        var_t7__blk103_dn6_slot: &mut f64,
        var_t7__blk103_dn7_slot: &mut f64,
        var_vth_slot: &mut f64,
    ) {
        let mut var_dppg: f64 = *var_dppg_slot;
        let mut var_dppg_dn0: f64 = *var_dppg_dn0_slot;
        let mut var_dppg_dn10: f64 = *var_dppg_dn10_slot;
        let mut var_dppg_dn11: f64 = *var_dppg_dn11_slot;
        let mut var_dppg_dn12: f64 = *var_dppg_dn12_slot;
        let mut var_dppg_dn17: f64 = *var_dppg_dn17_slot;
        let mut var_dppg_dn2: f64 = *var_dppg_dn2_slot;
        let mut var_dppg_dn6: f64 = *var_dppg_dn6_slot;
        let mut var_dppg_dn7: f64 = *var_dppg_dn7_slot;
        let mut var_dvth: f64 = *var_dvth_slot;
        let mut var_dvth0__blk87: f64 = *var_dvth0__blk87_slot;
        let mut var_dvth0__blk87_dn0: f64 = *var_dvth0__blk87_dn0_slot;
        let mut var_dvth0__blk87_dn10: f64 = *var_dvth0__blk87_dn10_slot;
        let mut var_dvth0__blk87_dn11: f64 = *var_dvth0__blk87_dn11_slot;
        let mut var_dvth0__blk87_dn12: f64 = *var_dvth0__blk87_dn12_slot;
        let mut var_dvth0__blk87_dn17: f64 = *var_dvth0__blk87_dn17_slot;
        let mut var_dvth0__blk87_dn2: f64 = *var_dvth0__blk87_dn2_slot;
        let mut var_dvth0__blk87_dn6: f64 = *var_dvth0__blk87_dn6_slot;
        let mut var_dvth0__blk87_dn7: f64 = *var_dvth0__blk87_dn7_slot;
        let mut var_dvth0__blk95: f64 = *var_dvth0__blk95_slot;
        let mut var_dvth0__blk95_dn0: f64 = *var_dvth0__blk95_dn0_slot;
        let mut var_dvth0__blk95_dn10: f64 = *var_dvth0__blk95_dn10_slot;
        let mut var_dvth0__blk95_dn11: f64 = *var_dvth0__blk95_dn11_slot;
        let mut var_dvth0__blk95_dn12: f64 = *var_dvth0__blk95_dn12_slot;
        let mut var_dvth0__blk95_dn17: f64 = *var_dvth0__blk95_dn17_slot;
        let mut var_dvth0__blk95_dn2: f64 = *var_dvth0__blk95_dn2_slot;
        let mut var_dvth0__blk95_dn6: f64 = *var_dvth0__blk95_dn6_slot;
        let mut var_dvth0__blk95_dn7: f64 = *var_dvth0__blk95_dn7_slot;
        let mut var_dvth_dn0: f64 = *var_dvth_dn0_slot;
        let mut var_dvth_dn10: f64 = *var_dvth_dn10_slot;
        let mut var_dvth_dn11: f64 = *var_dvth_dn11_slot;
        let mut var_dvth_dn12: f64 = *var_dvth_dn12_slot;
        let mut var_dvth_dn17: f64 = *var_dvth_dn17_slot;
        let mut var_dvth_dn2: f64 = *var_dvth_dn2_slot;
        let mut var_dvth_dn6: f64 = *var_dvth_dn6_slot;
        let mut var_dvth_dn7: f64 = *var_dvth_dn7_slot;
        let mut var_dvthlp: f64 = *var_dvthlp_slot;
        let mut var_dvthlp_dn0: f64 = *var_dvthlp_dn0_slot;
        let mut var_dvthlp_dn10: f64 = *var_dvthlp_dn10_slot;
        let mut var_dvthlp_dn11: f64 = *var_dvthlp_dn11_slot;
        let mut var_dvthlp_dn12: f64 = *var_dvthlp_dn12_slot;
        let mut var_dvthlp_dn17: f64 = *var_dvthlp_dn17_slot;
        let mut var_dvthlp_dn2: f64 = *var_dvthlp_dn2_slot;
        let mut var_dvthlp_dn6: f64 = *var_dvthlp_dn6_slot;
        let mut var_dvthlp_dn7: f64 = *var_dvthlp_dn7_slot;
        let mut var_dvthsc: f64 = *var_dvthsc_slot;
        let mut var_dvthsc_dn0: f64 = *var_dvthsc_dn0_slot;
        let mut var_dvthsc_dn10: f64 = *var_dvthsc_dn10_slot;
        let mut var_dvthsc_dn11: f64 = *var_dvthsc_dn11_slot;
        let mut var_dvthsc_dn12: f64 = *var_dvthsc_dn12_slot;
        let mut var_dvthsc_dn17: f64 = *var_dvthsc_dn17_slot;
        let mut var_dvthsc_dn2: f64 = *var_dvthsc_dn2_slot;
        let mut var_dvthsc_dn6: f64 = *var_dvthsc_dn6_slot;
        let mut var_dvthsc_dn7: f64 = *var_dvthsc_dn7_slot;
        let mut var_dvthscr: f64 = *var_dvthscr_slot;
        let mut var_dvthscr_dn0: f64 = *var_dvthscr_dn0_slot;
        let mut var_dvthscr_dn10: f64 = *var_dvthscr_dn10_slot;
        let mut var_dvthscr_dn11: f64 = *var_dvthscr_dn11_slot;
        let mut var_dvthscr_dn12: f64 = *var_dvthscr_dn12_slot;
        let mut var_dvthscr_dn17: f64 = *var_dvthscr_dn17_slot;
        let mut var_dvthscr_dn2: f64 = *var_dvthscr_dn2_slot;
        let mut var_dvthscr_dn6: f64 = *var_dvthscr_dn6_slot;
        let mut var_dvthscr_dn7: f64 = *var_dvthscr_dn7_slot;
        let mut var_dvthw: f64 = *var_dvthw_slot;
        let mut var_dvthw_dn0: f64 = *var_dvthw_dn0_slot;
        let mut var_dvthw_dn10: f64 = *var_dvthw_dn10_slot;
        let mut var_dvthw_dn11: f64 = *var_dvthw_dn11_slot;
        let mut var_dvthw_dn12: f64 = *var_dvthw_dn12_slot;
        let mut var_dvthw_dn17: f64 = *var_dvthw_dn17_slot;
        let mut var_dvthw_dn2: f64 = *var_dvthw_dn2_slot;
        let mut var_dvthw_dn6: f64 = *var_dvthw_dn6_slot;
        let mut var_dvthw_dn7: f64 = *var_dvthw_dn7_slot;
        let mut var_flg_dppg: f64 = *var_flg_dppg_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard99: f64 = *var_guard99_slot;
        let mut var_t0__blk104: f64 = *var_t0__blk104_slot;
        let mut var_t0__blk81: f64 = *var_t0__blk81_slot;
        let mut var_t0__blk89: f64 = *var_t0__blk89_slot;
        let mut var_t0__blk89_dn0: f64 = *var_t0__blk89_dn0_slot;
        let mut var_t0__blk89_dn10: f64 = *var_t0__blk89_dn10_slot;
        let mut var_t0__blk89_dn11: f64 = *var_t0__blk89_dn11_slot;
        let mut var_t0__blk89_dn12: f64 = *var_t0__blk89_dn12_slot;
        let mut var_t0__blk89_dn17: f64 = *var_t0__blk89_dn17_slot;
        let mut var_t0__blk89_dn2: f64 = *var_t0__blk89_dn2_slot;
        let mut var_t0__blk89_dn6: f64 = *var_t0__blk89_dn6_slot;
        let mut var_t0__blk89_dn7: f64 = *var_t0__blk89_dn7_slot;
        let mut var_t1__blk100: f64 = *var_t1__blk100_slot;
        let mut var_t1__blk100_dn0: f64 = *var_t1__blk100_dn0_slot;
        let mut var_t1__blk100_dn10: f64 = *var_t1__blk100_dn10_slot;
        let mut var_t1__blk100_dn11: f64 = *var_t1__blk100_dn11_slot;
        let mut var_t1__blk100_dn12: f64 = *var_t1__blk100_dn12_slot;
        let mut var_t1__blk100_dn17: f64 = *var_t1__blk100_dn17_slot;
        let mut var_t1__blk100_dn2: f64 = *var_t1__blk100_dn2_slot;
        let mut var_t1__blk100_dn6: f64 = *var_t1__blk100_dn6_slot;
        let mut var_t1__blk100_dn7: f64 = *var_t1__blk100_dn7_slot;
        let mut var_t1__blk82: f64 = *var_t1__blk82_slot;
        let mut var_t1__blk82_dn0: f64 = *var_t1__blk82_dn0_slot;
        let mut var_t1__blk82_dn10: f64 = *var_t1__blk82_dn10_slot;
        let mut var_t1__blk82_dn11: f64 = *var_t1__blk82_dn11_slot;
        let mut var_t1__blk82_dn12: f64 = *var_t1__blk82_dn12_slot;
        let mut var_t1__blk82_dn17: f64 = *var_t1__blk82_dn17_slot;
        let mut var_t1__blk82_dn2: f64 = *var_t1__blk82_dn2_slot;
        let mut var_t1__blk82_dn6: f64 = *var_t1__blk82_dn6_slot;
        let mut var_t1__blk82_dn7: f64 = *var_t1__blk82_dn7_slot;
        let mut var_t1__blk90: f64 = *var_t1__blk90_slot;
        let mut var_t1__blk90_dn0: f64 = *var_t1__blk90_dn0_slot;
        let mut var_t1__blk90_dn10: f64 = *var_t1__blk90_dn10_slot;
        let mut var_t1__blk90_dn11: f64 = *var_t1__blk90_dn11_slot;
        let mut var_t1__blk90_dn12: f64 = *var_t1__blk90_dn12_slot;
        let mut var_t1__blk90_dn17: f64 = *var_t1__blk90_dn17_slot;
        let mut var_t1__blk90_dn2: f64 = *var_t1__blk90_dn2_slot;
        let mut var_t1__blk90_dn6: f64 = *var_t1__blk90_dn6_slot;
        let mut var_t1__blk90_dn7: f64 = *var_t1__blk90_dn7_slot;
        let mut var_t1__blk96: f64 = *var_t1__blk96_slot;
        let mut var_t1__blk96_dn0: f64 = *var_t1__blk96_dn0_slot;
        let mut var_t1__blk96_dn10: f64 = *var_t1__blk96_dn10_slot;
        let mut var_t1__blk96_dn11: f64 = *var_t1__blk96_dn11_slot;
        let mut var_t1__blk96_dn12: f64 = *var_t1__blk96_dn12_slot;
        let mut var_t1__blk96_dn17: f64 = *var_t1__blk96_dn17_slot;
        let mut var_t1__blk96_dn2: f64 = *var_t1__blk96_dn2_slot;
        let mut var_t1__blk96_dn6: f64 = *var_t1__blk96_dn6_slot;
        let mut var_t1__blk96_dn7: f64 = *var_t1__blk96_dn7_slot;
        let mut var_t2__blk83: f64 = *var_t2__blk83_slot;
        let mut var_t2__blk83_dn0: f64 = *var_t2__blk83_dn0_slot;
        let mut var_t2__blk83_dn10: f64 = *var_t2__blk83_dn10_slot;
        let mut var_t2__blk83_dn11: f64 = *var_t2__blk83_dn11_slot;
        let mut var_t2__blk83_dn12: f64 = *var_t2__blk83_dn12_slot;
        let mut var_t2__blk83_dn17: f64 = *var_t2__blk83_dn17_slot;
        let mut var_t2__blk83_dn2: f64 = *var_t2__blk83_dn2_slot;
        let mut var_t2__blk83_dn6: f64 = *var_t2__blk83_dn6_slot;
        let mut var_t2__blk83_dn7: f64 = *var_t2__blk83_dn7_slot;
        let mut var_t2__blk91: f64 = *var_t2__blk91_slot;
        let mut var_t2__blk91_dn0: f64 = *var_t2__blk91_dn0_slot;
        let mut var_t2__blk91_dn10: f64 = *var_t2__blk91_dn10_slot;
        let mut var_t2__blk91_dn11: f64 = *var_t2__blk91_dn11_slot;
        let mut var_t2__blk91_dn12: f64 = *var_t2__blk91_dn12_slot;
        let mut var_t2__blk91_dn17: f64 = *var_t2__blk91_dn17_slot;
        let mut var_t2__blk91_dn2: f64 = *var_t2__blk91_dn2_slot;
        let mut var_t2__blk91_dn6: f64 = *var_t2__blk91_dn6_slot;
        let mut var_t2__blk91_dn7: f64 = *var_t2__blk91_dn7_slot;
        let mut var_t2__blk97: f64 = *var_t2__blk97_slot;
        let mut var_t3__blk101: f64 = *var_t3__blk101_slot;
        let mut var_t3__blk101_dn0: f64 = *var_t3__blk101_dn0_slot;
        let mut var_t3__blk101_dn10: f64 = *var_t3__blk101_dn10_slot;
        let mut var_t3__blk101_dn11: f64 = *var_t3__blk101_dn11_slot;
        let mut var_t3__blk101_dn12: f64 = *var_t3__blk101_dn12_slot;
        let mut var_t3__blk101_dn17: f64 = *var_t3__blk101_dn17_slot;
        let mut var_t3__blk101_dn2: f64 = *var_t3__blk101_dn2_slot;
        let mut var_t3__blk101_dn6: f64 = *var_t3__blk101_dn6_slot;
        let mut var_t3__blk101_dn7: f64 = *var_t3__blk101_dn7_slot;
        let mut var_t3__blk84: f64 = *var_t3__blk84_slot;
        let mut var_t3__blk84_dn0: f64 = *var_t3__blk84_dn0_slot;
        let mut var_t3__blk84_dn10: f64 = *var_t3__blk84_dn10_slot;
        let mut var_t3__blk84_dn11: f64 = *var_t3__blk84_dn11_slot;
        let mut var_t3__blk84_dn12: f64 = *var_t3__blk84_dn12_slot;
        let mut var_t3__blk84_dn17: f64 = *var_t3__blk84_dn17_slot;
        let mut var_t3__blk84_dn2: f64 = *var_t3__blk84_dn2_slot;
        let mut var_t3__blk84_dn6: f64 = *var_t3__blk84_dn6_slot;
        let mut var_t3__blk84_dn7: f64 = *var_t3__blk84_dn7_slot;
        let mut var_t3__blk92: f64 = *var_t3__blk92_slot;
        let mut var_t3__blk98: f64 = *var_t3__blk98_slot;
        let mut var_t4__blk85: f64 = *var_t4__blk85_slot;
        let mut var_t4__blk93: f64 = *var_t4__blk93_slot;
        let mut var_t4__blk93_dn0: f64 = *var_t4__blk93_dn0_slot;
        let mut var_t4__blk93_dn10: f64 = *var_t4__blk93_dn10_slot;
        let mut var_t4__blk93_dn11: f64 = *var_t4__blk93_dn11_slot;
        let mut var_t4__blk93_dn12: f64 = *var_t4__blk93_dn12_slot;
        let mut var_t4__blk93_dn17: f64 = *var_t4__blk93_dn17_slot;
        let mut var_t4__blk93_dn2: f64 = *var_t4__blk93_dn2_slot;
        let mut var_t4__blk93_dn6: f64 = *var_t4__blk93_dn6_slot;
        let mut var_t4__blk93_dn7: f64 = *var_t4__blk93_dn7_slot;
        let mut var_t5__blk102: f64 = *var_t5__blk102_slot;
        let mut var_t5__blk102_dn0: f64 = *var_t5__blk102_dn0_slot;
        let mut var_t5__blk102_dn10: f64 = *var_t5__blk102_dn10_slot;
        let mut var_t5__blk102_dn11: f64 = *var_t5__blk102_dn11_slot;
        let mut var_t5__blk102_dn12: f64 = *var_t5__blk102_dn12_slot;
        let mut var_t5__blk102_dn17: f64 = *var_t5__blk102_dn17_slot;
        let mut var_t5__blk102_dn2: f64 = *var_t5__blk102_dn2_slot;
        let mut var_t5__blk102_dn6: f64 = *var_t5__blk102_dn6_slot;
        let mut var_t5__blk102_dn7: f64 = *var_t5__blk102_dn7_slot;
        let mut var_t5__blk86: f64 = *var_t5__blk86_slot;
        let mut var_t5__blk86_dn0: f64 = *var_t5__blk86_dn0_slot;
        let mut var_t5__blk86_dn10: f64 = *var_t5__blk86_dn10_slot;
        let mut var_t5__blk86_dn11: f64 = *var_t5__blk86_dn11_slot;
        let mut var_t5__blk86_dn12: f64 = *var_t5__blk86_dn12_slot;
        let mut var_t5__blk86_dn17: f64 = *var_t5__blk86_dn17_slot;
        let mut var_t5__blk86_dn2: f64 = *var_t5__blk86_dn2_slot;
        let mut var_t5__blk86_dn6: f64 = *var_t5__blk86_dn6_slot;
        let mut var_t5__blk86_dn7: f64 = *var_t5__blk86_dn7_slot;
        let mut var_t5__blk94: f64 = *var_t5__blk94_slot;
        let mut var_t5__blk94_dn0: f64 = *var_t5__blk94_dn0_slot;
        let mut var_t5__blk94_dn10: f64 = *var_t5__blk94_dn10_slot;
        let mut var_t5__blk94_dn11: f64 = *var_t5__blk94_dn11_slot;
        let mut var_t5__blk94_dn12: f64 = *var_t5__blk94_dn12_slot;
        let mut var_t5__blk94_dn17: f64 = *var_t5__blk94_dn17_slot;
        let mut var_t5__blk94_dn2: f64 = *var_t5__blk94_dn2_slot;
        let mut var_t5__blk94_dn6: f64 = *var_t5__blk94_dn6_slot;
        let mut var_t5__blk94_dn7: f64 = *var_t5__blk94_dn7_slot;
        let mut var_t7__blk103: f64 = *var_t7__blk103_slot;
        let mut var_t7__blk103_dn0: f64 = *var_t7__blk103_dn0_slot;
        let mut var_t7__blk103_dn10: f64 = *var_t7__blk103_dn10_slot;
        let mut var_t7__blk103_dn11: f64 = *var_t7__blk103_dn11_slot;
        let mut var_t7__blk103_dn12: f64 = *var_t7__blk103_dn12_slot;
        let mut var_t7__blk103_dn17: f64 = *var_t7__blk103_dn17_slot;
        let mut var_t7__blk103_dn2: f64 = *var_t7__blk103_dn2_slot;
        let mut var_t7__blk103_dn6: f64 = *var_t7__blk103_dn6_slot;
        let mut var_t7__blk103_dn7: f64 = *var_t7__blk103_dn7_slot;
        let mut var_vth: f64 = *var_vth_slot;

        let (assign6400_e4266,) = {
    if (var_guard88 != 0.0) {
        let assign6400_e4263: f64 = (p.p72 * p.p72);
        let assign6400_e4264: f64 = (1.0 / assign6400_e4263);
        (assign6400_e4264,)
    } else {
        (var_t4__blk85,)
    }
};
        var_t4__blk85 = assign6400_e4266;

        let (assign6410_e4274, assign6410_e4274_d_n0, assign6410_e4274_d_n2, assign6410_e4274_d_n6, assign6410_e4274_d_n7, assign6410_e4274_d_n10, assign6410_e4274_d_n11, assign6410_e4274_d_n12, assign6410_e4274_d_n17,) = {
    if (var_guard88 != 0.0) {
        let assign6410_e4270: f64 = (2.0 * var_wd0);
        let assign6410_e4272: f64 = (assign6410_e4270 * var_t4__blk85);
        (assign6410_e4272, ((2.0 * var_wd0_dn0) * var_t4__blk85), ((2.0 * var_wd0_dn2) * var_t4__blk85), ((2.0 * var_wd0_dn6) * var_t4__blk85), ((2.0 * var_wd0_dn7) * var_t4__blk85), ((2.0 * var_wd0_dn10) * var_t4__blk85), ((2.0 * var_wd0_dn11) * var_t4__blk85), ((2.0 * var_wd0_dn12) * var_t4__blk85), ((2.0 * var_wd0_dn17) * var_t4__blk85),)
    } else {
        (var_t3__blk84, var_t3__blk84_dn0, var_t3__blk84_dn2, var_t3__blk84_dn6, var_t3__blk84_dn7, var_t3__blk84_dn10, var_t3__blk84_dn11, var_t3__blk84_dn12, var_t3__blk84_dn17,)
    }
};
        var_t3__blk84 = assign6410_e4274;
        var_t3__blk84_dn0 = assign6410_e4274_d_n0;
        var_t3__blk84_dn2 = assign6410_e4274_d_n2;
        var_t3__blk84_dn6 = assign6410_e4274_d_n6;
        var_t3__blk84_dn7 = assign6410_e4274_d_n7;
        var_t3__blk84_dn10 = assign6410_e4274_d_n10;
        var_t3__blk84_dn11 = assign6410_e4274_d_n11;
        var_t3__blk84_dn12 = assign6410_e4274_d_n12;
        var_t3__blk84_dn17 = assign6410_e4274_d_n17;

        let (assign6420_e4284, assign6420_e4284_d_n0, assign6420_e4284_d_n2, assign6420_e4284_d_n6, assign6420_e4284_d_n7, assign6420_e4284_d_n10, assign6420_e4284_d_n11, assign6420_e4284_d_n12, assign6420_e4284_d_n17,) = {
    if (var_guard88 != 0.0) {
        let assign6420_e4278: f64 = (var_t1__blk82 * var_t3__blk84);
        let assign6420_e4281: f64 = (p.p69 - var_pb20b);
        let assign6420_e4282: f64 = (assign6420_e4278 * assign6420_e4281);
        (assign6420_e4282, ((((var_t1__blk82_dn0 * var_t3__blk84) + (var_t1__blk82 * var_t3__blk84_dn0)) * assign6420_e4281) + (assign6420_e4278 * (-var_pb20b_dn0))), ((((var_t1__blk82_dn2 * var_t3__blk84) + (var_t1__blk82 * var_t3__blk84_dn2)) * assign6420_e4281) + (assign6420_e4278 * (-var_pb20b_dn2))), ((((var_t1__blk82_dn6 * var_t3__blk84) + (var_t1__blk82 * var_t3__blk84_dn6)) * assign6420_e4281) + (assign6420_e4278 * (-var_pb20b_dn6))), ((((var_t1__blk82_dn7 * var_t3__blk84) + (var_t1__blk82 * var_t3__blk84_dn7)) * assign6420_e4281) + (assign6420_e4278 * (-var_pb20b_dn7))), ((((var_t1__blk82_dn10 * var_t3__blk84) + (var_t1__blk82 * var_t3__blk84_dn10)) * assign6420_e4281) + (assign6420_e4278 * (-var_pb20b_dn10))), ((((var_t1__blk82_dn11 * var_t3__blk84) + (var_t1__blk82 * var_t3__blk84_dn11)) * assign6420_e4281) + (assign6420_e4278 * (-var_pb20b_dn11))), ((((var_t1__blk82_dn12 * var_t3__blk84) + (var_t1__blk82 * var_t3__blk84_dn12)) * assign6420_e4281) + (assign6420_e4278 * (-var_pb20b_dn12))), ((((var_t1__blk82_dn17 * var_t3__blk84) + (var_t1__blk82 * var_t3__blk84_dn17)) * assign6420_e4281) + (assign6420_e4278 * (-var_pb20b_dn17))),)
    } else {
        (var_t5__blk86, var_t5__blk86_dn0, var_t5__blk86_dn2, var_t5__blk86_dn6, var_t5__blk86_dn7, var_t5__blk86_dn10, var_t5__blk86_dn11, var_t5__blk86_dn12, var_t5__blk86_dn17,)
    }
};
        var_t5__blk86 = assign6420_e4284;
        var_t5__blk86_dn0 = assign6420_e4284_d_n0;
        var_t5__blk86_dn2 = assign6420_e4284_d_n2;
        var_t5__blk86_dn6 = assign6420_e4284_d_n6;
        var_t5__blk86_dn7 = assign6420_e4284_d_n7;
        var_t5__blk86_dn10 = assign6420_e4284_d_n10;
        var_t5__blk86_dn11 = assign6420_e4284_d_n11;
        var_t5__blk86_dn12 = assign6420_e4284_d_n12;
        var_t5__blk86_dn17 = assign6420_e4284_d_n17;

        let (assign6430_e4288, assign6430_e4288_d_n0, assign6430_e4288_d_n2, assign6430_e4288_d_n6, assign6430_e4288_d_n7, assign6430_e4288_d_n10, assign6430_e4288_d_n11, assign6430_e4288_d_n12, assign6430_e4288_d_n17,) = {
    if (var_guard88 != 0.0) {
        (var_t5__blk86, var_t5__blk86_dn0, var_t5__blk86_dn2, var_t5__blk86_dn6, var_t5__blk86_dn7, var_t5__blk86_dn10, var_t5__blk86_dn11, var_t5__blk86_dn12, var_t5__blk86_dn17,)
    } else {
        (var_dvth0__blk87, var_dvth0__blk87_dn0, var_dvth0__blk87_dn2, var_dvth0__blk87_dn6, var_dvth0__blk87_dn7, var_dvth0__blk87_dn10, var_dvth0__blk87_dn11, var_dvth0__blk87_dn12, var_dvth0__blk87_dn17,)
    }
};
        var_dvth0__blk87 = assign6430_e4288;
        var_dvth0__blk87_dn0 = assign6430_e4288_d_n0;
        var_dvth0__blk87_dn2 = assign6430_e4288_d_n2;
        var_dvth0__blk87_dn6 = assign6430_e4288_d_n6;
        var_dvth0__blk87_dn7 = assign6430_e4288_d_n7;
        var_dvth0__blk87_dn10 = assign6430_e4288_d_n10;
        var_dvth0__blk87_dn11 = assign6430_e4288_d_n11;
        var_dvth0__blk87_dn12 = assign6430_e4288_d_n12;
        var_dvth0__blk87_dn17 = assign6430_e4288_d_n17;

        let (assign6440_e4294, assign6440_e4294_d_n0, assign6440_e4294_d_n2, assign6440_e4294_d_n6, assign6440_e4294_d_n7, assign6440_e4294_d_n10, assign6440_e4294_d_n11, assign6440_e4294_d_n12, assign6440_e4294_d_n17,) = {
    if (var_guard88 != 0.0) {
        let assign6440_e4292: f64 = (var_vthp - var_vth0);
        (assign6440_e4292, (var_vthp_dn0 - var_vth0_dn0), (var_vthp_dn2 - var_vth0_dn2), (var_vthp_dn6 - var_vth0_dn6), (var_vthp_dn7 - var_vth0_dn7), (var_vthp_dn10 - var_vth0_dn10), (var_vthp_dn11 - var_vth0_dn11), (var_vthp_dn12 - var_vth0_dn12), (var_vthp_dn17 - var_vth0_dn17),)
    } else {
        (var_t1__blk82, var_t1__blk82_dn0, var_t1__blk82_dn2, var_t1__blk82_dn6, var_t1__blk82_dn7, var_t1__blk82_dn10, var_t1__blk82_dn11, var_t1__blk82_dn12, var_t1__blk82_dn17,)
    }
};
        var_t1__blk82 = assign6440_e4294;
        var_t1__blk82_dn0 = assign6440_e4294_d_n0;
        var_t1__blk82_dn2 = assign6440_e4294_d_n2;
        var_t1__blk82_dn6 = assign6440_e4294_d_n6;
        var_t1__blk82_dn7 = assign6440_e4294_d_n7;
        var_t1__blk82_dn10 = assign6440_e4294_d_n10;
        var_t1__blk82_dn11 = assign6440_e4294_d_n11;
        var_t1__blk82_dn12 = assign6440_e4294_d_n12;
        var_t1__blk82_dn17 = assign6440_e4294_d_n17;

        let (assign6450_e4300,) = {
    if (var_guard88 != 0.0) {
        let assign6450_e4298: f64 = (var_uc_scp3 / p.p72);
        (assign6450_e4298,)
    } else {
        (var_t0__blk81,)
    }
};
        var_t0__blk81 = assign6450_e4300;

        let (assign6460_e4308, assign6460_e4308_d_n0, assign6460_e4308_d_n2, assign6460_e4308_d_n6, assign6460_e4308_d_n7, assign6460_e4308_d_n10, assign6460_e4308_d_n11, assign6460_e4308_d_n12, assign6460_e4308_d_n17,) = {
    if (var_guard88 != 0.0) {
        let assign6460_e4305: f64 = (var_t0__blk81 * var_pbsum);
        let assign6460_e4306: f64 = (p.p80 + assign6460_e4305);
        (assign6460_e4306, (var_t0__blk81 * var_pbsum_dn0), (var_t0__blk81 * var_pbsum_dn2), (var_t0__blk81 * var_pbsum_dn6), (var_t0__blk81 * var_pbsum_dn7), (var_t0__blk81 * var_pbsum_dn10), (var_t0__blk81 * var_pbsum_dn11), (var_t0__blk81 * var_pbsum_dn12), (var_t0__blk81 * var_pbsum_dn17),)
    } else {
        (var_t2__blk83, var_t2__blk83_dn0, var_t2__blk83_dn2, var_t2__blk83_dn6, var_t2__blk83_dn7, var_t2__blk83_dn10, var_t2__blk83_dn11, var_t2__blk83_dn12, var_t2__blk83_dn17,)
    }
};
        var_t2__blk83 = assign6460_e4308;
        var_t2__blk83_dn0 = assign6460_e4308_d_n0;
        var_t2__blk83_dn2 = assign6460_e4308_d_n2;
        var_t2__blk83_dn6 = assign6460_e4308_d_n6;
        var_t2__blk83_dn7 = assign6460_e4308_d_n7;
        var_t2__blk83_dn10 = assign6460_e4308_d_n10;
        var_t2__blk83_dn11 = assign6460_e4308_d_n11;
        var_t2__blk83_dn12 = assign6460_e4308_d_n12;
        var_t2__blk83_dn17 = assign6460_e4308_d_n17;

        let (assign6470_e4312, assign6470_e4312_d_n0, assign6470_e4312_d_n2, assign6470_e4312_d_n6, assign6470_e4312_d_n7, assign6470_e4312_d_n10, assign6470_e4312_d_n11, assign6470_e4312_d_n12, assign6470_e4312_d_n17,) = {
    if (var_guard88 != 0.0) {
        (var_uc_scp2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t5__blk86, var_t5__blk86_dn0, var_t5__blk86_dn2, var_t5__blk86_dn6, var_t5__blk86_dn7, var_t5__blk86_dn10, var_t5__blk86_dn11, var_t5__blk86_dn12, var_t5__blk86_dn17,)
    }
};
        var_t5__blk86 = assign6470_e4312;
        var_t5__blk86_dn0 = assign6470_e4312_d_n0;
        var_t5__blk86_dn2 = assign6470_e4312_d_n2;
        var_t5__blk86_dn6 = assign6470_e4312_d_n6;
        var_t5__blk86_dn7 = assign6470_e4312_d_n7;
        var_t5__blk86_dn10 = assign6470_e4312_d_n10;
        var_t5__blk86_dn11 = assign6470_e4312_d_n11;
        var_t5__blk86_dn12 = assign6470_e4312_d_n12;
        var_t5__blk86_dn17 = assign6470_e4312_d_n17;

        let (assign6480_e4320, assign6480_e4320_d_n0, assign6480_e4320_d_n2, assign6480_e4320_d_n6, assign6480_e4320_d_n7, assign6480_e4320_d_n10, assign6480_e4320_d_n11, assign6480_e4320_d_n12, assign6480_e4320_d_n17,) = {
    if (var_guard88 != 0.0) {
        let assign6480_e4317: f64 = (var_t5__blk86 * var_vdsz);
        let assign6480_e4318: f64 = (var_t2__blk83 + assign6480_e4317);
        (assign6480_e4318, (var_t2__blk83_dn0 + ((var_t5__blk86_dn0 * var_vdsz) + (var_t5__blk86 * var_vdsz_dn0))), (var_t2__blk83_dn2 + ((var_t5__blk86_dn2 * var_vdsz) + (var_t5__blk86 * var_vdsz_dn2))), (var_t2__blk83_dn6 + ((var_t5__blk86_dn6 * var_vdsz) + (var_t5__blk86 * var_vdsz_dn6))), (var_t2__blk83_dn7 + ((var_t5__blk86_dn7 * var_vdsz) + (var_t5__blk86 * var_vdsz_dn7))), (var_t2__blk83_dn10 + ((var_t5__blk86_dn10 * var_vdsz) + (var_t5__blk86 * var_vdsz_dn10))), (var_t2__blk83_dn11 + ((var_t5__blk86_dn11 * var_vdsz) + (var_t5__blk86 * var_vdsz_dn11))), (var_t2__blk83_dn12 + ((var_t5__blk86_dn12 * var_vdsz) + (var_t5__blk86 * var_vdsz_dn12))), (var_t2__blk83_dn17 + ((var_t5__blk86_dn17 * var_vdsz) + (var_t5__blk86 * var_vdsz_dn17))),)
    } else {
        (var_t3__blk84, var_t3__blk84_dn0, var_t3__blk84_dn2, var_t3__blk84_dn6, var_t3__blk84_dn7, var_t3__blk84_dn10, var_t3__blk84_dn11, var_t3__blk84_dn12, var_t3__blk84_dn17,)
    }
};
        var_t3__blk84 = assign6480_e4320;
        var_t3__blk84_dn0 = assign6480_e4320_d_n0;
        var_t3__blk84_dn2 = assign6480_e4320_d_n2;
        var_t3__blk84_dn6 = assign6480_e4320_d_n6;
        var_t3__blk84_dn7 = assign6480_e4320_d_n7;
        var_t3__blk84_dn10 = assign6480_e4320_d_n10;
        var_t3__blk84_dn11 = assign6480_e4320_d_n11;
        var_t3__blk84_dn12 = assign6480_e4320_d_n12;
        var_t3__blk84_dn17 = assign6480_e4320_d_n17;

        let (assign6490_e4328, assign6490_e4328_d_n0, assign6490_e4328_d_n2, assign6490_e4328_d_n6, assign6490_e4328_d_n7, assign6490_e4328_d_n10, assign6490_e4328_d_n11, assign6490_e4328_d_n12, assign6490_e4328_d_n17,) = {
    if (var_guard88 != 0.0) {
        let assign6490_e4324: f64 = (var_t1__blk82 * var_dvth0__blk87);
        let assign6490_e4326: f64 = (assign6490_e4324 * var_t3__blk84);
        (assign6490_e4326, ((((var_t1__blk82_dn0 * var_dvth0__blk87) + (var_t1__blk82 * var_dvth0__blk87_dn0)) * var_t3__blk84) + (assign6490_e4324 * var_t3__blk84_dn0)), ((((var_t1__blk82_dn2 * var_dvth0__blk87) + (var_t1__blk82 * var_dvth0__blk87_dn2)) * var_t3__blk84) + (assign6490_e4324 * var_t3__blk84_dn2)), ((((var_t1__blk82_dn6 * var_dvth0__blk87) + (var_t1__blk82 * var_dvth0__blk87_dn6)) * var_t3__blk84) + (assign6490_e4324 * var_t3__blk84_dn6)), ((((var_t1__blk82_dn7 * var_dvth0__blk87) + (var_t1__blk82 * var_dvth0__blk87_dn7)) * var_t3__blk84) + (assign6490_e4324 * var_t3__blk84_dn7)), ((((var_t1__blk82_dn10 * var_dvth0__blk87) + (var_t1__blk82 * var_dvth0__blk87_dn10)) * var_t3__blk84) + (assign6490_e4324 * var_t3__blk84_dn10)), ((((var_t1__blk82_dn11 * var_dvth0__blk87) + (var_t1__blk82 * var_dvth0__blk87_dn11)) * var_t3__blk84) + (assign6490_e4324 * var_t3__blk84_dn11)), ((((var_t1__blk82_dn12 * var_dvth0__blk87) + (var_t1__blk82 * var_dvth0__blk87_dn12)) * var_t3__blk84) + (assign6490_e4324 * var_t3__blk84_dn12)), ((((var_t1__blk82_dn17 * var_dvth0__blk87) + (var_t1__blk82 * var_dvth0__blk87_dn17)) * var_t3__blk84) + (assign6490_e4324 * var_t3__blk84_dn17)),)
    } else {
        (var_dvthlp, var_dvthlp_dn0, var_dvthlp_dn2, var_dvthlp_dn6, var_dvthlp_dn7, var_dvthlp_dn10, var_dvthlp_dn11, var_dvthlp_dn12, var_dvthlp_dn17,)
    }
};
        var_dvthlp = assign6490_e4328;
        var_dvthlp_dn0 = assign6490_e4328_d_n0;
        var_dvthlp_dn2 = assign6490_e4328_d_n2;
        var_dvthlp_dn6 = assign6490_e4328_d_n6;
        var_dvthlp_dn7 = assign6490_e4328_d_n7;
        var_dvthlp_dn10 = assign6490_e4328_d_n10;
        var_dvthlp_dn11 = assign6490_e4328_d_n11;
        var_dvthlp_dn12 = assign6490_e4328_d_n12;
        var_dvthlp_dn17 = assign6490_e4328_d_n17;

        let (assign6500_e4333, assign6500_e4333_d_n0, assign6500_e4333_d_n2, assign6500_e4333_d_n6, assign6500_e4333_d_n7, assign6500_e4333_d_n10, assign6500_e4333_d_n11, assign6500_e4333_d_n12, assign6500_e4333_d_n17,) = {
    if (var_guard88 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dvthlp, var_dvthlp_dn0, var_dvthlp_dn2, var_dvthlp_dn6, var_dvthlp_dn7, var_dvthlp_dn10, var_dvthlp_dn11, var_dvthlp_dn12, var_dvthlp_dn17,)
    }
};
        var_dvthlp = assign6500_e4333;
        var_dvthlp_dn0 = assign6500_e4333_d_n0;
        var_dvthlp_dn2 = assign6500_e4333_d_n2;
        var_dvthlp_dn6 = assign6500_e4333_d_n6;
        var_dvthlp_dn7 = assign6500_e4333_d_n7;
        var_dvthlp_dn10 = assign6500_e4333_d_n10;
        var_dvthlp_dn11 = assign6500_e4333_d_n11;
        var_dvthlp_dn12 = assign6500_e4333_d_n12;
        var_dvthlp_dn17 = assign6500_e4333_d_n17;

        let assign6510_e4336: f64 = (1.034943e-10 * var_wd0);
        let assign6510_e4338: f64 = (assign6510_e4336 * 2.0);
        var_t0__blk89 = assign6510_e4338;
        var_t0__blk89_dn0 = ((1.034943e-10 * var_wd0_dn0) * 2.0);
        var_t0__blk89_dn2 = ((1.034943e-10 * var_wd0_dn2) * 2.0);
        var_t0__blk89_dn6 = ((1.034943e-10 * var_wd0_dn6) * 2.0);
        var_t0__blk89_dn7 = ((1.034943e-10 * var_wd0_dn7) * 2.0);
        var_t0__blk89_dn10 = ((1.034943e-10 * var_wd0_dn10) * 2.0);
        var_t0__blk89_dn11 = ((1.034943e-10 * var_wd0_dn11) * 2.0);
        var_t0__blk89_dn12 = ((1.034943e-10 * var_wd0_dn12) * 2.0);
        var_t0__blk89_dn17 = ((1.034943e-10 * var_wd0_dn17) * 2.0);

        let assign6520_e4341: f64 = (var_c_fox_inv * var_t0__blk89);
        var_t1__blk90 = assign6520_e4341;
        var_t1__blk90_dn0 = ((var_c_fox_inv_dn0 * var_t0__blk89) + (var_c_fox_inv * var_t0__blk89_dn0));
        var_t1__blk90_dn2 = ((var_c_fox_inv_dn2 * var_t0__blk89) + (var_c_fox_inv * var_t0__blk89_dn2));
        var_t1__blk90_dn6 = ((var_c_fox_inv_dn6 * var_t0__blk89) + (var_c_fox_inv * var_t0__blk89_dn6));
        var_t1__blk90_dn7 = ((var_c_fox_inv_dn7 * var_t0__blk89) + (var_c_fox_inv * var_t0__blk89_dn7));
        var_t1__blk90_dn10 = ((var_c_fox_inv_dn10 * var_t0__blk89) + (var_c_fox_inv * var_t0__blk89_dn10));
        var_t1__blk90_dn11 = ((var_c_fox_inv_dn11 * var_t0__blk89) + (var_c_fox_inv * var_t0__blk89_dn11));
        var_t1__blk90_dn12 = ((var_c_fox_inv_dn12 * var_t0__blk89) + (var_c_fox_inv * var_t0__blk89_dn12));
        var_t1__blk90_dn17 = ((var_c_fox_inv_dn17 * var_t0__blk89) + (var_c_fox_inv * var_t0__blk89_dn17));

        let assign6530_e4344: f64 = (p.p69 - var_pb20b);
        var_t2__blk91 = assign6530_e4344;
        var_t2__blk91_dn0 = (-var_pb20b_dn0);
        var_t2__blk91_dn2 = (-var_pb20b_dn2);
        var_t2__blk91_dn6 = (-var_pb20b_dn6);
        var_t2__blk91_dn7 = (-var_pb20b_dn7);
        var_t2__blk91_dn10 = (-var_pb20b_dn10);
        var_t2__blk91_dn11 = (-var_pb20b_dn11);
        var_t2__blk91_dn12 = (-var_pb20b_dn12);
        var_t2__blk91_dn17 = (-var_pb20b_dn17);

        let assign6540_e4347: f64 = (var_lgleff - p.p71);
        var_t3__blk92 = assign6540_e4347;

        let assign6550_e4351: f64 = (var_t3__blk92 * var_t3__blk92);
        let assign6550_e4352: f64 = (1.0 / assign6550_e4351);
        var_t4__blk93 = assign6550_e4352;
        var_t4__blk93_dn0 = 0.0;
        var_t4__blk93_dn2 = 0.0;
        var_t4__blk93_dn6 = 0.0;
        var_t4__blk93_dn7 = 0.0;
        var_t4__blk93_dn10 = 0.0;
        var_t4__blk93_dn11 = 0.0;
        var_t4__blk93_dn12 = 0.0;
        var_t4__blk93_dn17 = 0.0;

        let assign6560_e4355: f64 = (var_t1__blk90 * var_t2__blk91);
        let assign6560_e4357: f64 = (assign6560_e4355 * var_t4__blk93);
        var_dvth0__blk95 = assign6560_e4357;
        var_dvth0__blk95_dn0 = ((((var_t1__blk90_dn0 * var_t2__blk91) + (var_t1__blk90 * var_t2__blk91_dn0)) * var_t4__blk93) + (assign6560_e4355 * var_t4__blk93_dn0));
        var_dvth0__blk95_dn2 = ((((var_t1__blk90_dn2 * var_t2__blk91) + (var_t1__blk90 * var_t2__blk91_dn2)) * var_t4__blk93) + (assign6560_e4355 * var_t4__blk93_dn2));
        var_dvth0__blk95_dn6 = ((((var_t1__blk90_dn6 * var_t2__blk91) + (var_t1__blk90 * var_t2__blk91_dn6)) * var_t4__blk93) + (assign6560_e4355 * var_t4__blk93_dn6));
        var_dvth0__blk95_dn7 = ((((var_t1__blk90_dn7 * var_t2__blk91) + (var_t1__blk90 * var_t2__blk91_dn7)) * var_t4__blk93) + (assign6560_e4355 * var_t4__blk93_dn7));
        var_dvth0__blk95_dn10 = ((((var_t1__blk90_dn10 * var_t2__blk91) + (var_t1__blk90 * var_t2__blk91_dn10)) * var_t4__blk93) + (assign6560_e4355 * var_t4__blk93_dn10));
        var_dvth0__blk95_dn11 = ((((var_t1__blk90_dn11 * var_t2__blk91) + (var_t1__blk90 * var_t2__blk91_dn11)) * var_t4__blk93) + (assign6560_e4355 * var_t4__blk93_dn11));
        var_dvth0__blk95_dn12 = ((((var_t1__blk90_dn12 * var_t2__blk91) + (var_t1__blk90 * var_t2__blk91_dn12)) * var_t4__blk93) + (assign6560_e4355 * var_t4__blk93_dn12));
        var_dvth0__blk95_dn17 = ((((var_t1__blk90_dn17 * var_t2__blk91) + (var_t1__blk90 * var_t2__blk91_dn17)) * var_t4__blk93) + (assign6560_e4355 * var_t4__blk93_dn17));

        let assign6570_e4360: f64 = (var_uc_sc3 / var_lgleff);
        var_t1__blk90 = assign6570_e4360;
        var_t1__blk90_dn0 = 0.0;
        var_t1__blk90_dn2 = 0.0;
        var_t1__blk90_dn6 = 0.0;
        var_t1__blk90_dn7 = 0.0;
        var_t1__blk90_dn10 = 0.0;
        var_t1__blk90_dn11 = 0.0;
        var_t1__blk90_dn12 = 0.0;
        var_t1__blk90_dn17 = 0.0;

        let assign6580_e4364: f64 = (var_t1__blk90 * var_pbsum);
        let assign6580_e4365: f64 = (p.p83 + assign6580_e4364);
        var_t4__blk93 = assign6580_e4365;
        var_t4__blk93_dn0 = ((var_t1__blk90_dn0 * var_pbsum) + (var_t1__blk90 * var_pbsum_dn0));
        var_t4__blk93_dn2 = ((var_t1__blk90_dn2 * var_pbsum) + (var_t1__blk90 * var_pbsum_dn2));
        var_t4__blk93_dn6 = ((var_t1__blk90_dn6 * var_pbsum) + (var_t1__blk90 * var_pbsum_dn6));
        var_t4__blk93_dn7 = ((var_t1__blk90_dn7 * var_pbsum) + (var_t1__blk90 * var_pbsum_dn7));
        var_t4__blk93_dn10 = ((var_t1__blk90_dn10 * var_pbsum) + (var_t1__blk90 * var_pbsum_dn10));
        var_t4__blk93_dn11 = ((var_t1__blk90_dn11 * var_pbsum) + (var_t1__blk90 * var_pbsum_dn11));
        var_t4__blk93_dn12 = ((var_t1__blk90_dn12 * var_pbsum) + (var_t1__blk90 * var_pbsum_dn12));
        var_t4__blk93_dn17 = ((var_t1__blk90_dn17 * var_pbsum) + (var_t1__blk90 * var_pbsum_dn17));

        let assign6590_e4369: f64 = (var_uc_sc2 * var_vdsz);
        let assign6590_e4370: f64 = (var_t4__blk93 + assign6590_e4369);
        var_t5__blk94 = assign6590_e4370;
        var_t5__blk94_dn0 = (var_t4__blk93_dn0 + (var_uc_sc2 * var_vdsz_dn0));
        var_t5__blk94_dn2 = (var_t4__blk93_dn2 + (var_uc_sc2 * var_vdsz_dn2));
        var_t5__blk94_dn6 = (var_t4__blk93_dn6 + (var_uc_sc2 * var_vdsz_dn6));
        var_t5__blk94_dn7 = (var_t4__blk93_dn7 + (var_uc_sc2 * var_vdsz_dn7));
        var_t5__blk94_dn10 = (var_t4__blk93_dn10 + (var_uc_sc2 * var_vdsz_dn10));
        var_t5__blk94_dn11 = (var_t4__blk93_dn11 + (var_uc_sc2 * var_vdsz_dn11));
        var_t5__blk94_dn12 = (var_t4__blk93_dn12 + (var_uc_sc2 * var_vdsz_dn12));
        var_t5__blk94_dn17 = (var_t4__blk93_dn17 + (var_uc_sc2 * var_vdsz_dn17));

        let assign6600_e4373: f64 = (var_dvth0__blk95 * var_t5__blk94);
        var_dvthsc = assign6600_e4373;
        var_dvthsc_dn0 = ((var_dvth0__blk95_dn0 * var_t5__blk94) + (var_dvth0__blk95 * var_t5__blk94_dn0));
        var_dvthsc_dn2 = ((var_dvth0__blk95_dn2 * var_t5__blk94) + (var_dvth0__blk95 * var_t5__blk94_dn2));
        var_dvthsc_dn6 = ((var_dvth0__blk95_dn6 * var_t5__blk94) + (var_dvth0__blk95 * var_t5__blk94_dn6));
        var_dvthsc_dn7 = ((var_dvth0__blk95_dn7 * var_t5__blk94) + (var_dvth0__blk95 * var_t5__blk94_dn7));
        var_dvthsc_dn10 = ((var_dvth0__blk95_dn10 * var_t5__blk94) + (var_dvth0__blk95 * var_t5__blk94_dn10));
        var_dvthsc_dn11 = ((var_dvth0__blk95_dn11 * var_t5__blk94) + (var_dvth0__blk95 * var_t5__blk94_dn11));
        var_dvthsc_dn12 = ((var_dvth0__blk95_dn12 * var_t5__blk94) + (var_dvth0__blk95 * var_t5__blk94_dn12));
        var_dvthsc_dn17 = ((var_dvth0__blk95_dn17 * var_t5__blk94) + (var_dvth0__blk95 * var_t5__blk94_dn17));

        let assign6610_e4376: f64 = if p.p86 > 0.0 { 1.0 } else { 0.0 };
        var_guard99 = assign6610_e4376;

        let (assign6620_e4390, assign6620_e4390_d_n0, assign6620_e4390_d_n2, assign6620_e4390_d_n6, assign6620_e4390_d_n7, assign6620_e4390_d_n10, assign6620_e4390_d_n11, assign6620_e4390_d_n12, assign6620_e4390_d_n17,) = {
    if (var_guard99 != 0.0) {
        let assign6620_e4380: f64 = (var_eg + var_pb2);
        let assign6620_e4383: f64 = (2.0 * p.p88);
        let assign6620_e4384: f64 = (assign6620_e4380 - assign6620_e4383);
        let assign6620_e4387: f64 = (p.p87 * var_vdsz);
        let assign6620_e4388: f64 = (assign6620_e4384 + assign6620_e4387);
        (assign6620_e4388, ((var_eg_dn0 + var_pb2_dn0) + (p.p87 * var_vdsz_dn0)), ((var_eg_dn2 + var_pb2_dn2) + (p.p87 * var_vdsz_dn2)), ((var_eg_dn6 + var_pb2_dn6) + (p.p87 * var_vdsz_dn6)), ((var_eg_dn7 + var_pb2_dn7) + (p.p87 * var_vdsz_dn7)), ((var_eg_dn10 + var_pb2_dn10) + (p.p87 * var_vdsz_dn10)), ((var_eg_dn11 + var_pb2_dn11) + (p.p87 * var_vdsz_dn11)), ((var_eg_dn12 + var_pb2_dn12) + (p.p87 * var_vdsz_dn12)), ((var_eg_dn17 + var_pb2_dn17) + (p.p87 * var_vdsz_dn17)),)
    } else {
        (var_t1__blk96, var_t1__blk96_dn0, var_t1__blk96_dn2, var_t1__blk96_dn6, var_t1__blk96_dn7, var_t1__blk96_dn10, var_t1__blk96_dn11, var_t1__blk96_dn12, var_t1__blk96_dn17,)
    }
};
        var_t1__blk96 = assign6620_e4390;
        var_t1__blk96_dn0 = assign6620_e4390_d_n0;
        var_t1__blk96_dn2 = assign6620_e4390_d_n2;
        var_t1__blk96_dn6 = assign6620_e4390_d_n6;
        var_t1__blk96_dn7 = assign6620_e4390_d_n7;
        var_t1__blk96_dn10 = assign6620_e4390_d_n10;
        var_t1__blk96_dn11 = assign6620_e4390_d_n11;
        var_t1__blk96_dn12 = assign6620_e4390_d_n12;
        var_t1__blk96_dn17 = assign6620_e4390_d_n17;

        let (assign6630_e4398,) = {
    if (var_guard99 != 0.0) {
        let assign6630_e4394: f64 = (var_lgleff * 0.5);
        let assign6630_e4396: f64 = (assign6630_e4394 + var_mks_parl1);
        (assign6630_e4396,)
    } else {
        (var_t2__blk97,)
    }
};
        var_t2__blk97 = assign6630_e4398;

        let (assign6640_e4406,) = {
    if (var_guard99 != 0.0) {
        let assign6640_e4402: f64 = (p.p86 * p.p237);
        let assign6640_e4404: f64 = (assign6640_e4402 / var_t2__blk97);
        (assign6640_e4404,)
    } else {
        (var_t3__blk98,)
    }
};
        var_t3__blk98 = assign6640_e4406;

        let (assign6650_e4412, assign6650_e4412_d_n0, assign6650_e4412_d_n2, assign6650_e4412_d_n6, assign6650_e4412_d_n7, assign6650_e4412_d_n10, assign6650_e4412_d_n11, assign6650_e4412_d_n12, assign6650_e4412_d_n17,) = {
    if (var_guard99 != 0.0) {
        let assign6650_e4410: f64 = (var_t1__blk96 * var_t3__blk98);
        (assign6650_e4410, (var_t1__blk96_dn0 * var_t3__blk98), (var_t1__blk96_dn2 * var_t3__blk98), (var_t1__blk96_dn6 * var_t3__blk98), (var_t1__blk96_dn7 * var_t3__blk98), (var_t1__blk96_dn10 * var_t3__blk98), (var_t1__blk96_dn11 * var_t3__blk98), (var_t1__blk96_dn12 * var_t3__blk98), (var_t1__blk96_dn17 * var_t3__blk98),)
    } else {
        (var_dvthscr, var_dvthscr_dn0, var_dvthscr_dn2, var_dvthscr_dn6, var_dvthscr_dn7, var_dvthscr_dn10, var_dvthscr_dn11, var_dvthscr_dn12, var_dvthscr_dn17,)
    }
};
        var_dvthscr = assign6650_e4412;
        var_dvthscr_dn0 = assign6650_e4412_d_n0;
        var_dvthscr_dn2 = assign6650_e4412_d_n2;
        var_dvthscr_dn6 = assign6650_e4412_d_n6;
        var_dvthscr_dn7 = assign6650_e4412_d_n7;
        var_dvthscr_dn10 = assign6650_e4412_d_n10;
        var_dvthscr_dn11 = assign6650_e4412_d_n11;
        var_dvthscr_dn12 = assign6650_e4412_d_n12;
        var_dvthscr_dn17 = assign6650_e4412_d_n17;

        let (assign6660_e4417, assign6660_e4417_d_n0, assign6660_e4417_d_n2, assign6660_e4417_d_n6, assign6660_e4417_d_n7, assign6660_e4417_d_n10, assign6660_e4417_d_n11, assign6660_e4417_d_n12, assign6660_e4417_d_n17,) = {
    if (var_guard99 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dvthscr, var_dvthscr_dn0, var_dvthscr_dn2, var_dvthscr_dn6, var_dvthscr_dn7, var_dvthscr_dn10, var_dvthscr_dn11, var_dvthscr_dn12, var_dvthscr_dn17,)
    }
};
        var_dvthscr = assign6660_e4417;
        var_dvthscr_dn0 = assign6660_e4417_d_n0;
        var_dvthscr_dn2 = assign6660_e4417_d_n2;
        var_dvthscr_dn6 = assign6660_e4417_d_n6;
        var_dvthscr_dn7 = assign6660_e4417_d_n7;
        var_dvthscr_dn10 = assign6660_e4417_d_n10;
        var_dvthscr_dn11 = assign6660_e4417_d_n11;
        var_dvthscr_dn12 = assign6660_e4417_d_n12;
        var_dvthscr_dn17 = assign6660_e4417_d_n17;

        var_t1__blk100 = var_c_fox_inv;
        var_t1__blk100_dn0 = var_c_fox_inv_dn0;
        var_t1__blk100_dn2 = var_c_fox_inv_dn2;
        var_t1__blk100_dn6 = var_c_fox_inv_dn6;
        var_t1__blk100_dn7 = var_c_fox_inv_dn7;
        var_t1__blk100_dn10 = var_c_fox_inv_dn10;
        var_t1__blk100_dn11 = var_c_fox_inv_dn11;
        var_t1__blk100_dn12 = var_c_fox_inv_dn12;
        var_t1__blk100_dn17 = var_c_fox_inv_dn17;

        let assign6680_e4423: f64 = (var_mks_wfc / var_weff);
        let assign6680_e4424: f64 = (var_c_fox + assign6680_e4423);
        let assign6680_e4425: f64 = (1.0 / assign6680_e4424);
        var_t3__blk101 = assign6680_e4425;
        var_t3__blk101_dn0 = (-(var_c_fox_dn0 / (assign6680_e4424 * assign6680_e4424)));
        var_t3__blk101_dn2 = (-(var_c_fox_dn2 / (assign6680_e4424 * assign6680_e4424)));
        var_t3__blk101_dn6 = (-(var_c_fox_dn6 / (assign6680_e4424 * assign6680_e4424)));
        var_t3__blk101_dn7 = (-(var_c_fox_dn7 / (assign6680_e4424 * assign6680_e4424)));
        var_t3__blk101_dn10 = (-(var_c_fox_dn10 / (assign6680_e4424 * assign6680_e4424)));
        var_t3__blk101_dn11 = (-(var_c_fox_dn11 / (assign6680_e4424 * assign6680_e4424)));
        var_t3__blk101_dn12 = (-(var_c_fox_dn12 / (assign6680_e4424 * assign6680_e4424)));
        var_t3__blk101_dn17 = (-(var_c_fox_dn17 / (assign6680_e4424 * assign6680_e4424)));

        let assign6690_e4428: f64 = (var_t1__blk100 - var_t3__blk101);
        var_t5__blk102 = assign6690_e4428;
        var_t5__blk102_dn0 = (var_t1__blk100_dn0 - var_t3__blk101_dn0);
        var_t5__blk102_dn2 = (var_t1__blk100_dn2 - var_t3__blk101_dn2);
        var_t5__blk102_dn6 = (var_t1__blk100_dn6 - var_t3__blk101_dn6);
        var_t5__blk102_dn7 = (var_t1__blk100_dn7 - var_t3__blk101_dn7);
        var_t5__blk102_dn10 = (var_t1__blk100_dn10 - var_t3__blk101_dn10);
        var_t5__blk102_dn11 = (var_t1__blk100_dn11 - var_t3__blk101_dn11);
        var_t5__blk102_dn12 = (var_t1__blk100_dn12 - var_t3__blk101_dn12);
        var_t5__blk102_dn17 = (var_t1__blk100_dn17 - var_t3__blk101_dn17);

        let assign6700_e4431: f64 = (var_qb0 * var_t5__blk102);
        let assign6700_e4434: f64 = (p.p105 / var_wg);
        let assign6700_e4435: f64 = (assign6700_e4431 + assign6700_e4434);
        var_dvthw = assign6700_e4435;
        var_dvthw_dn0 = ((var_qb0_dn0 * var_t5__blk102) + (var_qb0 * var_t5__blk102_dn0));
        var_dvthw_dn2 = ((var_qb0_dn2 * var_t5__blk102) + (var_qb0 * var_t5__blk102_dn2));
        var_dvthw_dn6 = ((var_qb0_dn6 * var_t5__blk102) + (var_qb0 * var_t5__blk102_dn6));
        var_dvthw_dn7 = ((var_qb0_dn7 * var_t5__blk102) + (var_qb0 * var_t5__blk102_dn7));
        var_dvthw_dn10 = ((var_qb0_dn10 * var_t5__blk102) + (var_qb0 * var_t5__blk102_dn10));
        var_dvthw_dn11 = ((var_qb0_dn11 * var_t5__blk102) + (var_qb0 * var_t5__blk102_dn11));
        var_dvthw_dn12 = ((var_qb0_dn12 * var_t5__blk102) + (var_qb0 * var_t5__blk102_dn12));
        var_dvthw_dn17 = ((var_qb0_dn17 * var_t5__blk102) + (var_qb0 * var_t5__blk102_dn17));

        let assign6710_e4438: f64 = (var_dvthsc + var_dvthlp);
        let assign6710_e4440: f64 = (assign6710_e4438 + var_dvthw);
        let assign6710_e4442: f64 = (assign6710_e4440 + var_dvthscr);
        let assign6710_e4444: f64 = (assign6710_e4442 + var_dvthsm);
        var_dvth = assign6710_e4444;
        var_dvth_dn0 = (((var_dvthsc_dn0 + var_dvthlp_dn0) + var_dvthw_dn0) + var_dvthscr_dn0);
        var_dvth_dn2 = (((var_dvthsc_dn2 + var_dvthlp_dn2) + var_dvthw_dn2) + var_dvthscr_dn2);
        var_dvth_dn6 = (((var_dvthsc_dn6 + var_dvthlp_dn6) + var_dvthw_dn6) + var_dvthscr_dn6);
        var_dvth_dn7 = (((var_dvthsc_dn7 + var_dvthlp_dn7) + var_dvthw_dn7) + var_dvthscr_dn7);
        var_dvth_dn10 = (((var_dvthsc_dn10 + var_dvthlp_dn10) + var_dvthw_dn10) + var_dvthscr_dn10);
        var_dvth_dn11 = (((var_dvthsc_dn11 + var_dvthlp_dn11) + var_dvthw_dn11) + var_dvthscr_dn11);
        var_dvth_dn12 = (((var_dvthsc_dn12 + var_dvthlp_dn12) + var_dvthw_dn12) + var_dvthscr_dn12);
        var_dvth_dn17 = (((var_dvthsc_dn17 + var_dvthlp_dn17) + var_dvthw_dn17) + var_dvthscr_dn17);

        let assign6720_e4447: f64 = (var_vthp - var_dvth);
        var_vth = assign6720_e4447;

        let assign6730_e4450: f64 = if p.p89 == 0.0 { 1.0 } else { 0.0 };
        var_guard106 = assign6730_e4450;

        let (assign6740_e4454,) = {
    if (var_guard106 != 0.0) {
        (0.0,)
    } else {
        (var_flg_dppg,)
    }
};
        var_flg_dppg = assign6740_e4454;

        let (assign6750_e4459,) = {
    if (var_guard106 == 0.0) {
        (1.0,)
    } else {
        (var_flg_dppg,)
    }
};
        var_flg_dppg = assign6750_e4459;

        let assign6760_e4462: f64 = if var_flg_dppg == 0.0 { 1.0 } else { 0.0 };
        var_guard107 = assign6760_e4462;

        let (assign6770_e4466, assign6770_e4466_d_n0, assign6770_e4466_d_n2, assign6770_e4466_d_n6, assign6770_e4466_d_n7, assign6770_e4466_d_n10, assign6770_e4466_d_n11, assign6770_e4466_d_n12, assign6770_e4466_d_n17,) = {
    if (var_guard107 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dppg, var_dppg_dn0, var_dppg_dn2, var_dppg_dn6, var_dppg_dn7, var_dppg_dn10, var_dppg_dn11, var_dppg_dn12, var_dppg_dn17,)
    }
};
        var_dppg = assign6770_e4466;
        var_dppg_dn0 = assign6770_e4466_d_n0;
        var_dppg_dn2 = assign6770_e4466_d_n2;
        var_dppg_dn6 = assign6770_e4466_d_n6;
        var_dppg_dn7 = assign6770_e4466_d_n7;
        var_dppg_dn10 = assign6770_e4466_d_n10;
        var_dppg_dn11 = assign6770_e4466_d_n11;
        var_dppg_dn12 = assign6770_e4466_d_n12;
        var_dppg_dn17 = assign6770_e4466_d_n17;

        let (assign6780_e4471, assign6780_e4471_d_n0, assign6780_e4471_d_n2, assign6780_e4471_d_n6, assign6780_e4471_d_n7, assign6780_e4471_d_n10, assign6780_e4471_d_n11, assign6780_e4471_d_n12, assign6780_e4471_d_n17,) = {
    if (var_guard107 == 0.0) {
        (var_vgsz, var_vgsz_dn0, var_vgsz_dn2, var_vgsz_dn6, var_vgsz_dn7, var_vgsz_dn10, var_vgsz_dn11, var_vgsz_dn12, var_vgsz_dn17,)
    } else {
        (var_t7__blk103, var_t7__blk103_dn0, var_t7__blk103_dn2, var_t7__blk103_dn6, var_t7__blk103_dn7, var_t7__blk103_dn10, var_t7__blk103_dn11, var_t7__blk103_dn12, var_t7__blk103_dn17,)
    }
};
        var_t7__blk103 = assign6780_e4471;
        var_t7__blk103_dn0 = assign6780_e4471_d_n0;
        var_t7__blk103_dn2 = assign6780_e4471_d_n2;
        var_t7__blk103_dn6 = assign6780_e4471_d_n6;
        var_t7__blk103_dn7 = assign6780_e4471_d_n7;
        var_t7__blk103_dn10 = assign6780_e4471_d_n10;
        var_t7__blk103_dn11 = assign6780_e4471_d_n11;
        var_t7__blk103_dn12 = assign6780_e4471_d_n12;
        var_t7__blk103_dn17 = assign6780_e4471_d_n17;

        let (assign6790_e4476,) = {
    if (var_guard107 == 0.0) {
        (var_cnstpgd,)
    } else {
        (var_t0__blk104,)
    }
};
        var_t0__blk104 = assign6790_e4476;

        *var_dppg_slot = var_dppg;
        *var_dppg_dn0_slot = var_dppg_dn0;
        *var_dppg_dn10_slot = var_dppg_dn10;
        *var_dppg_dn11_slot = var_dppg_dn11;
        *var_dppg_dn12_slot = var_dppg_dn12;
        *var_dppg_dn17_slot = var_dppg_dn17;
        *var_dppg_dn2_slot = var_dppg_dn2;
        *var_dppg_dn6_slot = var_dppg_dn6;
        *var_dppg_dn7_slot = var_dppg_dn7;
        *var_dvth_slot = var_dvth;
        *var_dvth0__blk87_slot = var_dvth0__blk87;
        *var_dvth0__blk87_dn0_slot = var_dvth0__blk87_dn0;
        *var_dvth0__blk87_dn10_slot = var_dvth0__blk87_dn10;
        *var_dvth0__blk87_dn11_slot = var_dvth0__blk87_dn11;
        *var_dvth0__blk87_dn12_slot = var_dvth0__blk87_dn12;
        *var_dvth0__blk87_dn17_slot = var_dvth0__blk87_dn17;
        *var_dvth0__blk87_dn2_slot = var_dvth0__blk87_dn2;
        *var_dvth0__blk87_dn6_slot = var_dvth0__blk87_dn6;
        *var_dvth0__blk87_dn7_slot = var_dvth0__blk87_dn7;
        *var_dvth0__blk95_slot = var_dvth0__blk95;
        *var_dvth0__blk95_dn0_slot = var_dvth0__blk95_dn0;
        *var_dvth0__blk95_dn10_slot = var_dvth0__blk95_dn10;
        *var_dvth0__blk95_dn11_slot = var_dvth0__blk95_dn11;
        *var_dvth0__blk95_dn12_slot = var_dvth0__blk95_dn12;
        *var_dvth0__blk95_dn17_slot = var_dvth0__blk95_dn17;
        *var_dvth0__blk95_dn2_slot = var_dvth0__blk95_dn2;
        *var_dvth0__blk95_dn6_slot = var_dvth0__blk95_dn6;
        *var_dvth0__blk95_dn7_slot = var_dvth0__blk95_dn7;
        *var_dvth_dn0_slot = var_dvth_dn0;
        *var_dvth_dn10_slot = var_dvth_dn10;
        *var_dvth_dn11_slot = var_dvth_dn11;
        *var_dvth_dn12_slot = var_dvth_dn12;
        *var_dvth_dn17_slot = var_dvth_dn17;
        *var_dvth_dn2_slot = var_dvth_dn2;
        *var_dvth_dn6_slot = var_dvth_dn6;
        *var_dvth_dn7_slot = var_dvth_dn7;
        *var_dvthlp_slot = var_dvthlp;
        *var_dvthlp_dn0_slot = var_dvthlp_dn0;
        *var_dvthlp_dn10_slot = var_dvthlp_dn10;
        *var_dvthlp_dn11_slot = var_dvthlp_dn11;
        *var_dvthlp_dn12_slot = var_dvthlp_dn12;
        *var_dvthlp_dn17_slot = var_dvthlp_dn17;
        *var_dvthlp_dn2_slot = var_dvthlp_dn2;
        *var_dvthlp_dn6_slot = var_dvthlp_dn6;
        *var_dvthlp_dn7_slot = var_dvthlp_dn7;
        *var_dvthsc_slot = var_dvthsc;
        *var_dvthsc_dn0_slot = var_dvthsc_dn0;
        *var_dvthsc_dn10_slot = var_dvthsc_dn10;
        *var_dvthsc_dn11_slot = var_dvthsc_dn11;
        *var_dvthsc_dn12_slot = var_dvthsc_dn12;
        *var_dvthsc_dn17_slot = var_dvthsc_dn17;
        *var_dvthsc_dn2_slot = var_dvthsc_dn2;
        *var_dvthsc_dn6_slot = var_dvthsc_dn6;
        *var_dvthsc_dn7_slot = var_dvthsc_dn7;
        *var_dvthscr_slot = var_dvthscr;
        *var_dvthscr_dn0_slot = var_dvthscr_dn0;
        *var_dvthscr_dn10_slot = var_dvthscr_dn10;
        *var_dvthscr_dn11_slot = var_dvthscr_dn11;
        *var_dvthscr_dn12_slot = var_dvthscr_dn12;
        *var_dvthscr_dn17_slot = var_dvthscr_dn17;
        *var_dvthscr_dn2_slot = var_dvthscr_dn2;
        *var_dvthscr_dn6_slot = var_dvthscr_dn6;
        *var_dvthscr_dn7_slot = var_dvthscr_dn7;
        *var_dvthw_slot = var_dvthw;
        *var_dvthw_dn0_slot = var_dvthw_dn0;
        *var_dvthw_dn10_slot = var_dvthw_dn10;
        *var_dvthw_dn11_slot = var_dvthw_dn11;
        *var_dvthw_dn12_slot = var_dvthw_dn12;
        *var_dvthw_dn17_slot = var_dvthw_dn17;
        *var_dvthw_dn2_slot = var_dvthw_dn2;
        *var_dvthw_dn6_slot = var_dvthw_dn6;
        *var_dvthw_dn7_slot = var_dvthw_dn7;
        *var_flg_dppg_slot = var_flg_dppg;
        *var_guard106_slot = var_guard106;
        *var_guard107_slot = var_guard107;
        *var_guard99_slot = var_guard99;
        *var_t0__blk104_slot = var_t0__blk104;
        *var_t0__blk81_slot = var_t0__blk81;
        *var_t0__blk89_slot = var_t0__blk89;
        *var_t0__blk89_dn0_slot = var_t0__blk89_dn0;
        *var_t0__blk89_dn10_slot = var_t0__blk89_dn10;
        *var_t0__blk89_dn11_slot = var_t0__blk89_dn11;
        *var_t0__blk89_dn12_slot = var_t0__blk89_dn12;
        *var_t0__blk89_dn17_slot = var_t0__blk89_dn17;
        *var_t0__blk89_dn2_slot = var_t0__blk89_dn2;
        *var_t0__blk89_dn6_slot = var_t0__blk89_dn6;
        *var_t0__blk89_dn7_slot = var_t0__blk89_dn7;
        *var_t1__blk100_slot = var_t1__blk100;
        *var_t1__blk100_dn0_slot = var_t1__blk100_dn0;
        *var_t1__blk100_dn10_slot = var_t1__blk100_dn10;
        *var_t1__blk100_dn11_slot = var_t1__blk100_dn11;
        *var_t1__blk100_dn12_slot = var_t1__blk100_dn12;
        *var_t1__blk100_dn17_slot = var_t1__blk100_dn17;
        *var_t1__blk100_dn2_slot = var_t1__blk100_dn2;
        *var_t1__blk100_dn6_slot = var_t1__blk100_dn6;
        *var_t1__blk100_dn7_slot = var_t1__blk100_dn7;
        *var_t1__blk82_slot = var_t1__blk82;
        *var_t1__blk82_dn0_slot = var_t1__blk82_dn0;
        *var_t1__blk82_dn10_slot = var_t1__blk82_dn10;
        *var_t1__blk82_dn11_slot = var_t1__blk82_dn11;
        *var_t1__blk82_dn12_slot = var_t1__blk82_dn12;
        *var_t1__blk82_dn17_slot = var_t1__blk82_dn17;
        *var_t1__blk82_dn2_slot = var_t1__blk82_dn2;
        *var_t1__blk82_dn6_slot = var_t1__blk82_dn6;
        *var_t1__blk82_dn7_slot = var_t1__blk82_dn7;
        *var_t1__blk90_slot = var_t1__blk90;
        *var_t1__blk90_dn0_slot = var_t1__blk90_dn0;
        *var_t1__blk90_dn10_slot = var_t1__blk90_dn10;
        *var_t1__blk90_dn11_slot = var_t1__blk90_dn11;
        *var_t1__blk90_dn12_slot = var_t1__blk90_dn12;
        *var_t1__blk90_dn17_slot = var_t1__blk90_dn17;
        *var_t1__blk90_dn2_slot = var_t1__blk90_dn2;
        *var_t1__blk90_dn6_slot = var_t1__blk90_dn6;
        *var_t1__blk90_dn7_slot = var_t1__blk90_dn7;
        *var_t1__blk96_slot = var_t1__blk96;
        *var_t1__blk96_dn0_slot = var_t1__blk96_dn0;
        *var_t1__blk96_dn10_slot = var_t1__blk96_dn10;
        *var_t1__blk96_dn11_slot = var_t1__blk96_dn11;
        *var_t1__blk96_dn12_slot = var_t1__blk96_dn12;
        *var_t1__blk96_dn17_slot = var_t1__blk96_dn17;
        *var_t1__blk96_dn2_slot = var_t1__blk96_dn2;
        *var_t1__blk96_dn6_slot = var_t1__blk96_dn6;
        *var_t1__blk96_dn7_slot = var_t1__blk96_dn7;
        *var_t2__blk83_slot = var_t2__blk83;
        *var_t2__blk83_dn0_slot = var_t2__blk83_dn0;
        *var_t2__blk83_dn10_slot = var_t2__blk83_dn10;
        *var_t2__blk83_dn11_slot = var_t2__blk83_dn11;
        *var_t2__blk83_dn12_slot = var_t2__blk83_dn12;
        *var_t2__blk83_dn17_slot = var_t2__blk83_dn17;
        *var_t2__blk83_dn2_slot = var_t2__blk83_dn2;
        *var_t2__blk83_dn6_slot = var_t2__blk83_dn6;
        *var_t2__blk83_dn7_slot = var_t2__blk83_dn7;
        *var_t2__blk91_slot = var_t2__blk91;
        *var_t2__blk91_dn0_slot = var_t2__blk91_dn0;
        *var_t2__blk91_dn10_slot = var_t2__blk91_dn10;
        *var_t2__blk91_dn11_slot = var_t2__blk91_dn11;
        *var_t2__blk91_dn12_slot = var_t2__blk91_dn12;
        *var_t2__blk91_dn17_slot = var_t2__blk91_dn17;
        *var_t2__blk91_dn2_slot = var_t2__blk91_dn2;
        *var_t2__blk91_dn6_slot = var_t2__blk91_dn6;
        *var_t2__blk91_dn7_slot = var_t2__blk91_dn7;
        *var_t2__blk97_slot = var_t2__blk97;
        *var_t3__blk101_slot = var_t3__blk101;
        *var_t3__blk101_dn0_slot = var_t3__blk101_dn0;
        *var_t3__blk101_dn10_slot = var_t3__blk101_dn10;
        *var_t3__blk101_dn11_slot = var_t3__blk101_dn11;
        *var_t3__blk101_dn12_slot = var_t3__blk101_dn12;
        *var_t3__blk101_dn17_slot = var_t3__blk101_dn17;
        *var_t3__blk101_dn2_slot = var_t3__blk101_dn2;
        *var_t3__blk101_dn6_slot = var_t3__blk101_dn6;
        *var_t3__blk101_dn7_slot = var_t3__blk101_dn7;
        *var_t3__blk84_slot = var_t3__blk84;
        *var_t3__blk84_dn0_slot = var_t3__blk84_dn0;
        *var_t3__blk84_dn10_slot = var_t3__blk84_dn10;
        *var_t3__blk84_dn11_slot = var_t3__blk84_dn11;
        *var_t3__blk84_dn12_slot = var_t3__blk84_dn12;
        *var_t3__blk84_dn17_slot = var_t3__blk84_dn17;
        *var_t3__blk84_dn2_slot = var_t3__blk84_dn2;
        *var_t3__blk84_dn6_slot = var_t3__blk84_dn6;
        *var_t3__blk84_dn7_slot = var_t3__blk84_dn7;
        *var_t3__blk92_slot = var_t3__blk92;
        *var_t3__blk98_slot = var_t3__blk98;
        *var_t4__blk85_slot = var_t4__blk85;
        *var_t4__blk93_slot = var_t4__blk93;
        *var_t4__blk93_dn0_slot = var_t4__blk93_dn0;
        *var_t4__blk93_dn10_slot = var_t4__blk93_dn10;
        *var_t4__blk93_dn11_slot = var_t4__blk93_dn11;
        *var_t4__blk93_dn12_slot = var_t4__blk93_dn12;
        *var_t4__blk93_dn17_slot = var_t4__blk93_dn17;
        *var_t4__blk93_dn2_slot = var_t4__blk93_dn2;
        *var_t4__blk93_dn6_slot = var_t4__blk93_dn6;
        *var_t4__blk93_dn7_slot = var_t4__blk93_dn7;
        *var_t5__blk102_slot = var_t5__blk102;
        *var_t5__blk102_dn0_slot = var_t5__blk102_dn0;
        *var_t5__blk102_dn10_slot = var_t5__blk102_dn10;
        *var_t5__blk102_dn11_slot = var_t5__blk102_dn11;
        *var_t5__blk102_dn12_slot = var_t5__blk102_dn12;
        *var_t5__blk102_dn17_slot = var_t5__blk102_dn17;
        *var_t5__blk102_dn2_slot = var_t5__blk102_dn2;
        *var_t5__blk102_dn6_slot = var_t5__blk102_dn6;
        *var_t5__blk102_dn7_slot = var_t5__blk102_dn7;
        *var_t5__blk86_slot = var_t5__blk86;
        *var_t5__blk86_dn0_slot = var_t5__blk86_dn0;
        *var_t5__blk86_dn10_slot = var_t5__blk86_dn10;
        *var_t5__blk86_dn11_slot = var_t5__blk86_dn11;
        *var_t5__blk86_dn12_slot = var_t5__blk86_dn12;
        *var_t5__blk86_dn17_slot = var_t5__blk86_dn17;
        *var_t5__blk86_dn2_slot = var_t5__blk86_dn2;
        *var_t5__blk86_dn6_slot = var_t5__blk86_dn6;
        *var_t5__blk86_dn7_slot = var_t5__blk86_dn7;
        *var_t5__blk94_slot = var_t5__blk94;
        *var_t5__blk94_dn0_slot = var_t5__blk94_dn0;
        *var_t5__blk94_dn10_slot = var_t5__blk94_dn10;
        *var_t5__blk94_dn11_slot = var_t5__blk94_dn11;
        *var_t5__blk94_dn12_slot = var_t5__blk94_dn12;
        *var_t5__blk94_dn17_slot = var_t5__blk94_dn17;
        *var_t5__blk94_dn2_slot = var_t5__blk94_dn2;
        *var_t5__blk94_dn6_slot = var_t5__blk94_dn6;
        *var_t5__blk94_dn7_slot = var_t5__blk94_dn7;
        *var_t7__blk103_slot = var_t7__blk103;
        *var_t7__blk103_dn0_slot = var_t7__blk103_dn0;
        *var_t7__blk103_dn10_slot = var_t7__blk103_dn10;
        *var_t7__blk103_dn11_slot = var_t7__blk103_dn11;
        *var_t7__blk103_dn12_slot = var_t7__blk103_dn12;
        *var_t7__blk103_dn17_slot = var_t7__blk103_dn17;
        *var_t7__blk103_dn2_slot = var_t7__blk103_dn2;
        *var_t7__blk103_dn6_slot = var_t7__blk103_dn6;
        *var_t7__blk103_dn7_slot = var_t7__blk103_dn7;
        *var_vth_slot = var_vth;
    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn10: f64,
        var_beta_inv: f64,
        var_beta_inv_dn10: f64,
        var_c_fox_inv: f64,
        var_c_fox_inv_dn0: f64,
        var_c_fox_inv_dn10: f64,
        var_c_fox_inv_dn11: f64,
        var_c_fox_inv_dn12: f64,
        var_c_fox_inv_dn17: f64,
        var_c_fox_inv_dn2: f64,
        var_c_fox_inv_dn6: f64,
        var_c_fox_inv_dn7: f64,
        var_cnst0soi: f64,
        var_cnst0soi_dn0: f64,
        var_cnst0soi_dn10: f64,
        var_cnst0soi_dn11: f64,
        var_cnst0soi_dn12: f64,
        var_cnst0soi_dn17: f64,
        var_cnst0soi_dn2: f64,
        var_cnst0soi_dn6: f64,
        var_cnst0soi_dn7: f64,
        var_cnst1soi: f64,
        var_cnst1soi_dn0: f64,
        var_cnst1soi_dn10: f64,
        var_cnst1soi_dn11: f64,
        var_cnst1soi_dn12: f64,
        var_cnst1soi_dn17: f64,
        var_cnst1soi_dn2: f64,
        var_cnst1soi_dn6: f64,
        var_cnst1soi_dn7: f64,
        var_cnst_2esi_q_nsubs: f64,
        var_cnst_2esi_q_nsubs_dn0: f64,
        var_cnst_2esi_q_nsubs_dn10: f64,
        var_cnst_2esi_q_nsubs_dn11: f64,
        var_cnst_2esi_q_nsubs_dn12: f64,
        var_cnst_2esi_q_nsubs_dn17: f64,
        var_cnst_2esi_q_nsubs_dn2: f64,
        var_cnst_2esi_q_nsubs_dn6: f64,
        var_cnst_2esi_q_nsubs_dn7: f64,
        var_cnstc_foxi: f64,
        var_cnstc_foxi_dn0: f64,
        var_cnstc_foxi_dn10: f64,
        var_cnstc_foxi_dn11: f64,
        var_cnstc_foxi_dn12: f64,
        var_cnstc_foxi_dn17: f64,
        var_cnstc_foxi_dn2: f64,
        var_cnstc_foxi_dn6: f64,
        var_cnstc_foxi_dn7: f64,
        var_dvth: f64,
        var_dvth_dn0: f64,
        var_dvth_dn10: f64,
        var_dvth_dn11: f64,
        var_dvth_dn12: f64,
        var_dvth_dn17: f64,
        var_dvth_dn2: f64,
        var_dvth_dn6: f64,
        var_dvth_dn7: f64,
        var_guard107: f64,
        var_mks_nsubb: f64,
        var_pb2: f64,
        var_pb2_dn0: f64,
        var_pb2_dn10: f64,
        var_pb2_dn11: f64,
        var_pb2_dn12: f64,
        var_pb2_dn17: f64,
        var_pb2_dn2: f64,
        var_pb2_dn6: f64,
        var_pb2_dn7: f64,
        var_shift: f64,
        var_shift_dn0: f64,
        var_shift_dn10: f64,
        var_shift_dn11: f64,
        var_shift_dn12: f64,
        var_shift_dn17: f64,
        var_shift_dn2: f64,
        var_shift_dn6: f64,
        var_shift_dn7: f64,
        var_t0__blk104: f64,
        var_t7__blk103: f64,
        var_t7__blk103_dn0: f64,
        var_t7__blk103_dn10: f64,
        var_t7__blk103_dn11: f64,
        var_t7__blk103_dn12: f64,
        var_t7__blk103_dn17: f64,
        var_t7__blk103_dn2: f64,
        var_t7__blk103_dn6: f64,
        var_t7__blk103_dn7: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn17: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn7: f64,
        var_vfb: f64,
        var_vgs: f64,
        var_vgs_dn11: f64,
        var_vgs_dn6: f64,
        var_vgs_dn7: f64,
        var_dppg_slot: &mut f64,
        var_dppg_dn0_slot: &mut f64,
        var_dppg_dn10_slot: &mut f64,
        var_dppg_dn11_slot: &mut f64,
        var_dppg_dn12_slot: &mut f64,
        var_dppg_dn17_slot: &mut f64,
        var_dppg_dn2_slot: &mut f64,
        var_dppg_dn6_slot: &mut f64,
        var_dppg_dn7_slot: &mut f64,
        var_fac1_slot: &mut f64,
        var_fac1_dn0_slot: &mut f64,
        var_fac1_dn10_slot: &mut f64,
        var_fac1_dn11_slot: &mut f64,
        var_fac1_dn12_slot: &mut f64,
        var_fac1_dn17_slot: &mut f64,
        var_fac1_dn2_slot: &mut f64,
        var_fac1_dn6_slot: &mut f64,
        var_fac1_dn7_slot: &mut f64,
        var_fac1p2_slot: &mut f64,
        var_fac1p2_dn0_slot: &mut f64,
        var_fac1p2_dn10_slot: &mut f64,
        var_fac1p2_dn11_slot: &mut f64,
        var_fac1p2_dn12_slot: &mut f64,
        var_fac1p2_dn17_slot: &mut f64,
        var_fac1p2_dn2_slot: &mut f64,
        var_fac1p2_dn6_slot: &mut f64,
        var_fac1p2_dn7_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard110_slot: &mut f64,
        var_guard111_slot: &mut f64,
        var_ps0_inic_slot: &mut f64,
        var_ps0_inic_dn0_slot: &mut f64,
        var_ps0_inic_dn10_slot: &mut f64,
        var_ps0_inic_dn11_slot: &mut f64,
        var_ps0_inic_dn12_slot: &mut f64,
        var_ps0_inic_dn17_slot: &mut f64,
        var_ps0_inic_dn2_slot: &mut f64,
        var_ps0_inic_dn6_slot: &mut f64,
        var_ps0_inic_dn7_slot: &mut f64,
        var_qdepb_dlt_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3__blk105_slot: &mut f64,
        var_t3__blk105_dn0_slot: &mut f64,
        var_t3__blk105_dn10_slot: &mut f64,
        var_t3__blk105_dn11_slot: &mut f64,
        var_t3__blk105_dn12_slot: &mut f64,
        var_t3__blk105_dn17_slot: &mut f64,
        var_t3__blk105_dn2_slot: &mut f64,
        var_t3__blk105_dn6_slot: &mut f64,
        var_t3__blk105_dn7_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn17_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn17_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_vbi_soi_slot: &mut f64,
        var_vbi_soi_dn0_slot: &mut f64,
        var_vbi_soi_dn10_slot: &mut f64,
        var_vbi_soi_dn11_slot: &mut f64,
        var_vbi_soi_dn12_slot: &mut f64,
        var_vbi_soi_dn17_slot: &mut f64,
        var_vbi_soi_dn2_slot: &mut f64,
        var_vbi_soi_dn6_slot: &mut f64,
        var_vbi_soi_dn7_slot: &mut f64,
        var_vgp_slot: &mut f64,
        var_vgp_dn0_slot: &mut f64,
        var_vgp_dn10_slot: &mut f64,
        var_vgp_dn11_slot: &mut f64,
        var_vgp_dn12_slot: &mut f64,
        var_vgp_dn17_slot: &mut f64,
        var_vgp_dn2_slot: &mut f64,
        var_vgp_dn6_slot: &mut f64,
        var_vgp_dn7_slot: &mut f64,
        var_vgp_ini_slot: &mut f64,
        var_vgp_ini_dn0_slot: &mut f64,
        var_vgp_ini_dn10_slot: &mut f64,
        var_vgp_ini_dn11_slot: &mut f64,
        var_vgp_ini_dn12_slot: &mut f64,
        var_vgp_ini_dn17_slot: &mut f64,
        var_vgp_ini_dn2_slot: &mut f64,
        var_vgp_ini_dn6_slot: &mut f64,
        var_vgp_ini_dn7_slot: &mut f64,
        var_vgpz_slot: &mut f64,
        var_vgpz_dn0_slot: &mut f64,
        var_vgpz_dn10_slot: &mut f64,
        var_vgpz_dn11_slot: &mut f64,
        var_vgpz_dn12_slot: &mut f64,
        var_vgpz_dn17_slot: &mut f64,
        var_vgpz_dn2_slot: &mut f64,
        var_vgpz_dn6_slot: &mut f64,
        var_vgpz_dn7_slot: &mut f64,
        var_vgs_fb_slot: &mut f64,
        var_wdsoi_ini0_slot: &mut f64,
        var_wdsoi_ini0_dn0_slot: &mut f64,
        var_wdsoi_ini0_dn10_slot: &mut f64,
        var_wdsoi_ini0_dn11_slot: &mut f64,
        var_wdsoi_ini0_dn12_slot: &mut f64,
        var_wdsoi_ini0_dn17_slot: &mut f64,
        var_wdsoi_ini0_dn2_slot: &mut f64,
        var_wdsoi_ini0_dn6_slot: &mut f64,
        var_wdsoi_ini0_dn7_slot: &mut f64,
    ) {
        let mut var_dppg: f64 = *var_dppg_slot;
        let mut var_dppg_dn0: f64 = *var_dppg_dn0_slot;
        let mut var_dppg_dn10: f64 = *var_dppg_dn10_slot;
        let mut var_dppg_dn11: f64 = *var_dppg_dn11_slot;
        let mut var_dppg_dn12: f64 = *var_dppg_dn12_slot;
        let mut var_dppg_dn17: f64 = *var_dppg_dn17_slot;
        let mut var_dppg_dn2: f64 = *var_dppg_dn2_slot;
        let mut var_dppg_dn6: f64 = *var_dppg_dn6_slot;
        let mut var_dppg_dn7: f64 = *var_dppg_dn7_slot;
        let mut var_fac1: f64 = *var_fac1_slot;
        let mut var_fac1_dn0: f64 = *var_fac1_dn0_slot;
        let mut var_fac1_dn10: f64 = *var_fac1_dn10_slot;
        let mut var_fac1_dn11: f64 = *var_fac1_dn11_slot;
        let mut var_fac1_dn12: f64 = *var_fac1_dn12_slot;
        let mut var_fac1_dn17: f64 = *var_fac1_dn17_slot;
        let mut var_fac1_dn2: f64 = *var_fac1_dn2_slot;
        let mut var_fac1_dn6: f64 = *var_fac1_dn6_slot;
        let mut var_fac1_dn7: f64 = *var_fac1_dn7_slot;
        let mut var_fac1p2: f64 = *var_fac1p2_slot;
        let mut var_fac1p2_dn0: f64 = *var_fac1p2_dn0_slot;
        let mut var_fac1p2_dn10: f64 = *var_fac1p2_dn10_slot;
        let mut var_fac1p2_dn11: f64 = *var_fac1p2_dn11_slot;
        let mut var_fac1p2_dn12: f64 = *var_fac1p2_dn12_slot;
        let mut var_fac1p2_dn17: f64 = *var_fac1p2_dn17_slot;
        let mut var_fac1p2_dn2: f64 = *var_fac1p2_dn2_slot;
        let mut var_fac1p2_dn6: f64 = *var_fac1p2_dn6_slot;
        let mut var_fac1p2_dn7: f64 = *var_fac1p2_dn7_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard111: f64 = *var_guard111_slot;
        let mut var_ps0_inic: f64 = *var_ps0_inic_slot;
        let mut var_ps0_inic_dn0: f64 = *var_ps0_inic_dn0_slot;
        let mut var_ps0_inic_dn10: f64 = *var_ps0_inic_dn10_slot;
        let mut var_ps0_inic_dn11: f64 = *var_ps0_inic_dn11_slot;
        let mut var_ps0_inic_dn12: f64 = *var_ps0_inic_dn12_slot;
        let mut var_ps0_inic_dn17: f64 = *var_ps0_inic_dn17_slot;
        let mut var_ps0_inic_dn2: f64 = *var_ps0_inic_dn2_slot;
        let mut var_ps0_inic_dn6: f64 = *var_ps0_inic_dn6_slot;
        let mut var_ps0_inic_dn7: f64 = *var_ps0_inic_dn7_slot;
        let mut var_qdepb_dlt: f64 = *var_qdepb_dlt_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3__blk105: f64 = *var_t3__blk105_slot;
        let mut var_t3__blk105_dn0: f64 = *var_t3__blk105_dn0_slot;
        let mut var_t3__blk105_dn10: f64 = *var_t3__blk105_dn10_slot;
        let mut var_t3__blk105_dn11: f64 = *var_t3__blk105_dn11_slot;
        let mut var_t3__blk105_dn12: f64 = *var_t3__blk105_dn12_slot;
        let mut var_t3__blk105_dn17: f64 = *var_t3__blk105_dn17_slot;
        let mut var_t3__blk105_dn2: f64 = *var_t3__blk105_dn2_slot;
        let mut var_t3__blk105_dn6: f64 = *var_t3__blk105_dn6_slot;
        let mut var_t3__blk105_dn7: f64 = *var_t3__blk105_dn7_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn17: f64 = *var_t3_dn17_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn17: f64 = *var_tmf2_dn17_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_vbi_soi: f64 = *var_vbi_soi_slot;
        let mut var_vbi_soi_dn0: f64 = *var_vbi_soi_dn0_slot;
        let mut var_vbi_soi_dn10: f64 = *var_vbi_soi_dn10_slot;
        let mut var_vbi_soi_dn11: f64 = *var_vbi_soi_dn11_slot;
        let mut var_vbi_soi_dn12: f64 = *var_vbi_soi_dn12_slot;
        let mut var_vbi_soi_dn17: f64 = *var_vbi_soi_dn17_slot;
        let mut var_vbi_soi_dn2: f64 = *var_vbi_soi_dn2_slot;
        let mut var_vbi_soi_dn6: f64 = *var_vbi_soi_dn6_slot;
        let mut var_vbi_soi_dn7: f64 = *var_vbi_soi_dn7_slot;
        let mut var_vgp: f64 = *var_vgp_slot;
        let mut var_vgp_dn0: f64 = *var_vgp_dn0_slot;
        let mut var_vgp_dn10: f64 = *var_vgp_dn10_slot;
        let mut var_vgp_dn11: f64 = *var_vgp_dn11_slot;
        let mut var_vgp_dn12: f64 = *var_vgp_dn12_slot;
        let mut var_vgp_dn17: f64 = *var_vgp_dn17_slot;
        let mut var_vgp_dn2: f64 = *var_vgp_dn2_slot;
        let mut var_vgp_dn6: f64 = *var_vgp_dn6_slot;
        let mut var_vgp_dn7: f64 = *var_vgp_dn7_slot;
        let mut var_vgp_ini: f64 = *var_vgp_ini_slot;
        let mut var_vgp_ini_dn0: f64 = *var_vgp_ini_dn0_slot;
        let mut var_vgp_ini_dn10: f64 = *var_vgp_ini_dn10_slot;
        let mut var_vgp_ini_dn11: f64 = *var_vgp_ini_dn11_slot;
        let mut var_vgp_ini_dn12: f64 = *var_vgp_ini_dn12_slot;
        let mut var_vgp_ini_dn17: f64 = *var_vgp_ini_dn17_slot;
        let mut var_vgp_ini_dn2: f64 = *var_vgp_ini_dn2_slot;
        let mut var_vgp_ini_dn6: f64 = *var_vgp_ini_dn6_slot;
        let mut var_vgp_ini_dn7: f64 = *var_vgp_ini_dn7_slot;
        let mut var_vgpz: f64 = *var_vgpz_slot;
        let mut var_vgpz_dn0: f64 = *var_vgpz_dn0_slot;
        let mut var_vgpz_dn10: f64 = *var_vgpz_dn10_slot;
        let mut var_vgpz_dn11: f64 = *var_vgpz_dn11_slot;
        let mut var_vgpz_dn12: f64 = *var_vgpz_dn12_slot;
        let mut var_vgpz_dn17: f64 = *var_vgpz_dn17_slot;
        let mut var_vgpz_dn2: f64 = *var_vgpz_dn2_slot;
        let mut var_vgpz_dn6: f64 = *var_vgpz_dn6_slot;
        let mut var_vgpz_dn7: f64 = *var_vgpz_dn7_slot;
        let mut var_vgs_fb: f64 = *var_vgs_fb_slot;
        let mut var_wdsoi_ini0: f64 = *var_wdsoi_ini0_slot;
        let mut var_wdsoi_ini0_dn0: f64 = *var_wdsoi_ini0_dn0_slot;
        let mut var_wdsoi_ini0_dn10: f64 = *var_wdsoi_ini0_dn10_slot;
        let mut var_wdsoi_ini0_dn11: f64 = *var_wdsoi_ini0_dn11_slot;
        let mut var_wdsoi_ini0_dn12: f64 = *var_wdsoi_ini0_dn12_slot;
        let mut var_wdsoi_ini0_dn17: f64 = *var_wdsoi_ini0_dn17_slot;
        let mut var_wdsoi_ini0_dn2: f64 = *var_wdsoi_ini0_dn2_slot;
        let mut var_wdsoi_ini0_dn6: f64 = *var_wdsoi_ini0_dn6_slot;
        let mut var_wdsoi_ini0_dn7: f64 = *var_wdsoi_ini0_dn7_slot;

        let (assign6800_e4483, assign6800_e4483_d_n0, assign6800_e4483_d_n2, assign6800_e4483_d_n6, assign6800_e4483_d_n7, assign6800_e4483_d_n10, assign6800_e4483_d_n11, assign6800_e4483_d_n12, assign6800_e4483_d_n17,) = {
    if (var_guard107 == 0.0) {
        let assign6800_e4481: f64 = (var_t7__blk103 - p.p90);
        (assign6800_e4481, var_t7__blk103_dn0, var_t7__blk103_dn2, var_t7__blk103_dn6, var_t7__blk103_dn7, var_t7__blk103_dn10, var_t7__blk103_dn11, var_t7__blk103_dn12, var_t7__blk103_dn17,)
    } else {
        (var_t3__blk105, var_t3__blk105_dn0, var_t3__blk105_dn2, var_t3__blk105_dn6, var_t3__blk105_dn7, var_t3__blk105_dn10, var_t3__blk105_dn11, var_t3__blk105_dn12, var_t3__blk105_dn17,)
    }
};
        var_t3__blk105 = assign6800_e4483;
        var_t3__blk105_dn0 = assign6800_e4483_d_n0;
        var_t3__blk105_dn2 = assign6800_e4483_d_n2;
        var_t3__blk105_dn6 = assign6800_e4483_d_n6;
        var_t3__blk105_dn7 = assign6800_e4483_d_n7;
        var_t3__blk105_dn10 = assign6800_e4483_d_n10;
        var_t3__blk105_dn11 = assign6800_e4483_d_n11;
        var_t3__blk105_dn12 = assign6800_e4483_d_n12;
        var_t3__blk105_dn17 = assign6800_e4483_d_n17;

        let assign6810_e4486: f64 = (-3.0);
        let assign6810_e4487: f64 = if var_t3__blk105 < assign6810_e4486 { 1.0 } else { 0.0 };
        var_guard108 = assign6810_e4487;

        let (assign6820_e4494, assign6820_e4494_d_n0, assign6820_e4494_d_n2, assign6820_e4494_d_n6, assign6820_e4494_d_n7, assign6820_e4494_d_n10, assign6820_e4494_d_n11, assign6820_e4494_d_n12, assign6820_e4494_d_n17,) = {
    if ((var_guard107 == 0.0) && (var_guard108 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dppg, var_dppg_dn0, var_dppg_dn2, var_dppg_dn6, var_dppg_dn7, var_dppg_dn10, var_dppg_dn11, var_dppg_dn12, var_dppg_dn17,)
    }
};
        var_dppg = assign6820_e4494;
        var_dppg_dn0 = assign6820_e4494_d_n0;
        var_dppg_dn2 = assign6820_e4494_d_n2;
        var_dppg_dn6 = assign6820_e4494_d_n6;
        var_dppg_dn7 = assign6820_e4494_d_n7;
        var_dppg_dn10 = assign6820_e4494_d_n10;
        var_dppg_dn11 = assign6820_e4494_d_n11;
        var_dppg_dn12 = assign6820_e4494_d_n12;
        var_dppg_dn17 = assign6820_e4494_d_n17;

        let assign6830_e4497: f64 = if var_t3__blk105 < 0.0 { 1.0 } else { 0.0 };
        var_guard109 = assign6830_e4497;

        let (assign6840_e4523, assign6840_e4523_d_n0, assign6840_e4523_d_n2, assign6840_e4523_d_n6, assign6840_e4523_d_n7, assign6840_e4523_d_n10, assign6840_e4523_d_n11, assign6840_e4523_d_n12, assign6840_e4523_d_n17,) = {
    if (((var_guard107 == 0.0) && (var_guard108 == 0.0)) && (var_guard109 != 0.0)) {
        let assign6840_e4511: f64 = (1.0 / 3.0);
        let assign6840_e4515: f64 = (1.0 / 27.0);
        let assign6840_e4516: f64 = (var_t3__blk105 * assign6840_e4515);
        let assign6840_e4517: f64 = (assign6840_e4511 + assign6840_e4516);
        let assign6840_e4518: f64 = (var_t3__blk105 * assign6840_e4517);
        let assign6840_e4519: f64 = (1.0 + assign6840_e4518);
        let assign6840_e4520: f64 = (var_t3__blk105 * assign6840_e4519);
        let assign6840_e4521: f64 = (1.0 + assign6840_e4520);
        (assign6840_e4521, ((var_t3__blk105_dn0 * assign6840_e4519) + (var_t3__blk105 * ((var_t3__blk105_dn0 * assign6840_e4517) + (var_t3__blk105 * (var_t3__blk105_dn0 * assign6840_e4515))))), ((var_t3__blk105_dn2 * assign6840_e4519) + (var_t3__blk105 * ((var_t3__blk105_dn2 * assign6840_e4517) + (var_t3__blk105 * (var_t3__blk105_dn2 * assign6840_e4515))))), ((var_t3__blk105_dn6 * assign6840_e4519) + (var_t3__blk105 * ((var_t3__blk105_dn6 * assign6840_e4517) + (var_t3__blk105 * (var_t3__blk105_dn6 * assign6840_e4515))))), ((var_t3__blk105_dn7 * assign6840_e4519) + (var_t3__blk105 * ((var_t3__blk105_dn7 * assign6840_e4517) + (var_t3__blk105 * (var_t3__blk105_dn7 * assign6840_e4515))))), ((var_t3__blk105_dn10 * assign6840_e4519) + (var_t3__blk105 * ((var_t3__blk105_dn10 * assign6840_e4517) + (var_t3__blk105 * (var_t3__blk105_dn10 * assign6840_e4515))))), ((var_t3__blk105_dn11 * assign6840_e4519) + (var_t3__blk105 * ((var_t3__blk105_dn11 * assign6840_e4517) + (var_t3__blk105 * (var_t3__blk105_dn11 * assign6840_e4515))))), ((var_t3__blk105_dn12 * assign6840_e4519) + (var_t3__blk105 * ((var_t3__blk105_dn12 * assign6840_e4517) + (var_t3__blk105 * (var_t3__blk105_dn12 * assign6840_e4515))))), ((var_t3__blk105_dn17 * assign6840_e4519) + (var_t3__blk105 * ((var_t3__blk105_dn17 * assign6840_e4517) + (var_t3__blk105 * (var_t3__blk105_dn17 * assign6840_e4515))))),)
    } else {
        (var_dppg, var_dppg_dn0, var_dppg_dn2, var_dppg_dn6, var_dppg_dn7, var_dppg_dn10, var_dppg_dn11, var_dppg_dn12, var_dppg_dn17,)
    }
};
        var_dppg = assign6840_e4523;
        var_dppg_dn0 = assign6840_e4523_d_n0;
        var_dppg_dn2 = assign6840_e4523_d_n2;
        var_dppg_dn6 = assign6840_e4523_d_n6;
        var_dppg_dn7 = assign6840_e4523_d_n7;
        var_dppg_dn10 = assign6840_e4523_d_n10;
        var_dppg_dn11 = assign6840_e4523_d_n11;
        var_dppg_dn12 = assign6840_e4523_d_n12;
        var_dppg_dn17 = assign6840_e4523_d_n17;

        let (assign6850_e4552, assign6850_e4552_d_n0, assign6850_e4552_d_n2, assign6850_e4552_d_n6, assign6850_e4552_d_n7, assign6850_e4552_d_n10, assign6850_e4552_d_n11, assign6850_e4552_d_n12, assign6850_e4552_d_n17,) = {
    if (((var_guard107 == 0.0) && (var_guard108 == 0.0)) && (var_guard109 == 0.0)) {
        let assign6850_e4538: f64 = (1.0 / 3.0);
        let assign6850_e4543: f64 = (var_t3__blk105 * 0.148148111111111);
        let assign6850_e4544: f64 = (0.0402052934513951 + assign6850_e4543);
        let assign6850_e4545: f64 = (var_t3__blk105 * assign6850_e4544);
        let assign6850_e4546: f64 = (assign6850_e4538 + assign6850_e4545);
        let assign6850_e4547: f64 = (var_t3__blk105 * assign6850_e4546);
        let assign6850_e4548: f64 = (1.0 + assign6850_e4547);
        let assign6850_e4549: f64 = (var_t3__blk105 * assign6850_e4548);
        let assign6850_e4550: f64 = (1.0 + assign6850_e4549);
        (assign6850_e4550, ((var_t3__blk105_dn0 * assign6850_e4548) + (var_t3__blk105 * ((var_t3__blk105_dn0 * assign6850_e4546) + (var_t3__blk105 * ((var_t3__blk105_dn0 * assign6850_e4544) + (var_t3__blk105 * (var_t3__blk105_dn0 * 0.148148111111111))))))), ((var_t3__blk105_dn2 * assign6850_e4548) + (var_t3__blk105 * ((var_t3__blk105_dn2 * assign6850_e4546) + (var_t3__blk105 * ((var_t3__blk105_dn2 * assign6850_e4544) + (var_t3__blk105 * (var_t3__blk105_dn2 * 0.148148111111111))))))), ((var_t3__blk105_dn6 * assign6850_e4548) + (var_t3__blk105 * ((var_t3__blk105_dn6 * assign6850_e4546) + (var_t3__blk105 * ((var_t3__blk105_dn6 * assign6850_e4544) + (var_t3__blk105 * (var_t3__blk105_dn6 * 0.148148111111111))))))), ((var_t3__blk105_dn7 * assign6850_e4548) + (var_t3__blk105 * ((var_t3__blk105_dn7 * assign6850_e4546) + (var_t3__blk105 * ((var_t3__blk105_dn7 * assign6850_e4544) + (var_t3__blk105 * (var_t3__blk105_dn7 * 0.148148111111111))))))), ((var_t3__blk105_dn10 * assign6850_e4548) + (var_t3__blk105 * ((var_t3__blk105_dn10 * assign6850_e4546) + (var_t3__blk105 * ((var_t3__blk105_dn10 * assign6850_e4544) + (var_t3__blk105 * (var_t3__blk105_dn10 * 0.148148111111111))))))), ((var_t3__blk105_dn11 * assign6850_e4548) + (var_t3__blk105 * ((var_t3__blk105_dn11 * assign6850_e4546) + (var_t3__blk105 * ((var_t3__blk105_dn11 * assign6850_e4544) + (var_t3__blk105 * (var_t3__blk105_dn11 * 0.148148111111111))))))), ((var_t3__blk105_dn12 * assign6850_e4548) + (var_t3__blk105 * ((var_t3__blk105_dn12 * assign6850_e4546) + (var_t3__blk105 * ((var_t3__blk105_dn12 * assign6850_e4544) + (var_t3__blk105 * (var_t3__blk105_dn12 * 0.148148111111111))))))), ((var_t3__blk105_dn17 * assign6850_e4548) + (var_t3__blk105 * ((var_t3__blk105_dn17 * assign6850_e4546) + (var_t3__blk105 * ((var_t3__blk105_dn17 * assign6850_e4544) + (var_t3__blk105 * (var_t3__blk105_dn17 * 0.148148111111111))))))),)
    } else {
        (var_dppg, var_dppg_dn0, var_dppg_dn2, var_dppg_dn6, var_dppg_dn7, var_dppg_dn10, var_dppg_dn11, var_dppg_dn12, var_dppg_dn17,)
    }
};
        var_dppg = assign6850_e4552;
        var_dppg_dn0 = assign6850_e4552_d_n0;
        var_dppg_dn2 = assign6850_e4552_d_n2;
        var_dppg_dn6 = assign6850_e4552_d_n6;
        var_dppg_dn7 = assign6850_e4552_d_n7;
        var_dppg_dn10 = assign6850_e4552_d_n10;
        var_dppg_dn11 = assign6850_e4552_d_n11;
        var_dppg_dn12 = assign6850_e4552_d_n12;
        var_dppg_dn17 = assign6850_e4552_d_n17;

        let (assign6860_e4570, assign6860_e4570_d_n0, assign6860_e4570_d_n2, assign6860_e4570_d_n6, assign6860_e4570_d_n7, assign6860_e4570_d_n10, assign6860_e4570_d_n11, assign6860_e4570_d_n12, assign6860_e4570_d_n17,) = {
    if (var_guard107 == 0.0) {
        let assign6860_e4557: f64 = (var_dppg - 1.0);
        let assign6860_e4560: f64 = (var_dppg - 1.0);
        let assign6860_e4561: f64 = (assign6860_e4557 * assign6860_e4560);
        let assign6860_e4564: f64 = (4.0 * 0.1);
        let assign6860_e4566: f64 = (assign6860_e4564 * 0.1);
        let assign6860_e4567: f64 = (assign6860_e4561 + assign6860_e4566);
        let assign6860_e4568: f64 = (assign6860_e4567).sqrt();
        (assign6860_e4568, (((var_dppg_dn0 * assign6860_e4560) + (assign6860_e4557 * var_dppg_dn0)) / (2.0 * assign6860_e4568)), (((var_dppg_dn2 * assign6860_e4560) + (assign6860_e4557 * var_dppg_dn2)) / (2.0 * assign6860_e4568)), (((var_dppg_dn6 * assign6860_e4560) + (assign6860_e4557 * var_dppg_dn6)) / (2.0 * assign6860_e4568)), (((var_dppg_dn7 * assign6860_e4560) + (assign6860_e4557 * var_dppg_dn7)) / (2.0 * assign6860_e4568)), (((var_dppg_dn10 * assign6860_e4560) + (assign6860_e4557 * var_dppg_dn10)) / (2.0 * assign6860_e4568)), (((var_dppg_dn11 * assign6860_e4560) + (assign6860_e4557 * var_dppg_dn11)) / (2.0 * assign6860_e4568)), (((var_dppg_dn12 * assign6860_e4560) + (assign6860_e4557 * var_dppg_dn12)) / (2.0 * assign6860_e4568)), (((var_dppg_dn17 * assign6860_e4560) + (assign6860_e4557 * var_dppg_dn17)) / (2.0 * assign6860_e4568)),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign6860_e4570;
        var_tmf1_dn0 = assign6860_e4570_d_n0;
        var_tmf1_dn2 = assign6860_e4570_d_n2;
        var_tmf1_dn6 = assign6860_e4570_d_n6;
        var_tmf1_dn7 = assign6860_e4570_d_n7;
        var_tmf1_dn10 = assign6860_e4570_d_n10;
        var_tmf1_dn11 = assign6860_e4570_d_n11;
        var_tmf1_dn12 = assign6860_e4570_d_n12;
        var_tmf1_dn17 = assign6860_e4570_d_n17;

        let (assign6870_e4585, assign6870_e4585_d_n0, assign6870_e4585_d_n2, assign6870_e4585_d_n6, assign6870_e4585_d_n7, assign6870_e4585_d_n10, assign6870_e4585_d_n11, assign6870_e4585_d_n12, assign6870_e4585_d_n17,) = {
    if (var_guard107 == 0.0) {
        let assign6870_e4576: f64 = (var_dppg - 1.0);
        let assign6870_e4578: f64 = (assign6870_e4576 + var_tmf1);
        let assign6870_e4579: f64 = (0.5 * assign6870_e4578);
        let assign6870_e4582: f64 = (1e-10 * 0.1);
        let assign6870_e4583: f64 = (assign6870_e4579 + assign6870_e4582);
        (assign6870_e4583, (0.5 * (var_dppg_dn0 + var_tmf1_dn0)), (0.5 * (var_dppg_dn2 + var_tmf1_dn2)), (0.5 * (var_dppg_dn6 + var_tmf1_dn6)), (0.5 * (var_dppg_dn7 + var_tmf1_dn7)), (0.5 * (var_dppg_dn10 + var_tmf1_dn10)), (0.5 * (var_dppg_dn11 + var_tmf1_dn11)), (0.5 * (var_dppg_dn12 + var_tmf1_dn12)), (0.5 * (var_dppg_dn17 + var_tmf1_dn17)),)
    } else {
        (var_dppg, var_dppg_dn0, var_dppg_dn2, var_dppg_dn6, var_dppg_dn7, var_dppg_dn10, var_dppg_dn11, var_dppg_dn12, var_dppg_dn17,)
    }
};
        var_dppg = assign6870_e4585;
        var_dppg_dn0 = assign6870_e4585_d_n0;
        var_dppg_dn2 = assign6870_e4585_d_n2;
        var_dppg_dn6 = assign6870_e4585_d_n6;
        var_dppg_dn7 = assign6870_e4585_d_n7;
        var_dppg_dn10 = assign6870_e4585_d_n10;
        var_dppg_dn11 = assign6870_e4585_d_n11;
        var_dppg_dn12 = assign6870_e4585_d_n12;
        var_dppg_dn17 = assign6870_e4585_d_n17;

        let assign6880_e4588: f64 = if var_dppg < 0.0 { 1.0 } else { 0.0 };
        var_guard110 = assign6880_e4588;

        let (assign6890_e4595, assign6890_e4595_d_n0, assign6890_e4595_d_n2, assign6890_e4595_d_n6, assign6890_e4595_d_n7, assign6890_e4595_d_n10, assign6890_e4595_d_n11, assign6890_e4595_d_n12, assign6890_e4595_d_n17,) = {
    if ((var_guard107 == 0.0) && (var_guard110 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dppg, var_dppg_dn0, var_dppg_dn2, var_dppg_dn6, var_dppg_dn7, var_dppg_dn10, var_dppg_dn11, var_dppg_dn12, var_dppg_dn17,)
    }
};
        var_dppg = assign6890_e4595;
        var_dppg_dn0 = assign6890_e4595_d_n0;
        var_dppg_dn2 = assign6890_e4595_d_n2;
        var_dppg_dn6 = assign6890_e4595_d_n6;
        var_dppg_dn7 = assign6890_e4595_d_n7;
        var_dppg_dn10 = assign6890_e4595_d_n10;
        var_dppg_dn11 = assign6890_e4595_d_n11;
        var_dppg_dn12 = assign6890_e4595_d_n12;
        var_dppg_dn17 = assign6890_e4595_d_n17;

        let (assign6900_e4602, assign6900_e4602_d_n0, assign6900_e4602_d_n2, assign6900_e4602_d_n6, assign6900_e4602_d_n7, assign6900_e4602_d_n10, assign6900_e4602_d_n11, assign6900_e4602_d_n12, assign6900_e4602_d_n17,) = {
    if (var_guard107 == 0.0) {
        let assign6900_e4600: f64 = (var_dppg * var_t0__blk104);
        (assign6900_e4600, (var_dppg_dn0 * var_t0__blk104), (var_dppg_dn2 * var_t0__blk104), (var_dppg_dn6 * var_t0__blk104), (var_dppg_dn7 * var_t0__blk104), (var_dppg_dn10 * var_t0__blk104), (var_dppg_dn11 * var_t0__blk104), (var_dppg_dn12 * var_t0__blk104), (var_dppg_dn17 * var_t0__blk104),)
    } else {
        (var_dppg, var_dppg_dn0, var_dppg_dn2, var_dppg_dn6, var_dppg_dn7, var_dppg_dn10, var_dppg_dn11, var_dppg_dn12, var_dppg_dn17,)
    }
};
        var_dppg = assign6900_e4602;
        var_dppg_dn0 = assign6900_e4602_d_n0;
        var_dppg_dn2 = assign6900_e4602_d_n2;
        var_dppg_dn6 = assign6900_e4602_d_n6;
        var_dppg_dn7 = assign6900_e4602_d_n7;
        var_dppg_dn10 = assign6900_e4602_d_n10;
        var_dppg_dn11 = assign6900_e4602_d_n11;
        var_dppg_dn12 = assign6900_e4602_d_n12;
        var_dppg_dn17 = assign6900_e4602_d_n17;

        let (assign6910_e4611, assign6910_e4611_d_n0, assign6910_e4611_d_n2, assign6910_e4611_d_n6, assign6910_e4611_d_n7, assign6910_e4611_d_n10, assign6910_e4611_d_n11, assign6910_e4611_d_n12, assign6910_e4611_d_n17,) = {
    if (var_guard107 == 0.0) {
        let assign6910_e4607: f64 = (1.0 - var_dppg);
        let assign6910_e4609: f64 = (assign6910_e4607 - 0.05);
        (assign6910_e4609, (-var_dppg_dn0), (-var_dppg_dn2), (-var_dppg_dn6), (-var_dppg_dn7), (-var_dppg_dn10), (-var_dppg_dn11), (-var_dppg_dn12), (-var_dppg_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign6910_e4611;
        var_tmf1_dn0 = assign6910_e4611_d_n0;
        var_tmf1_dn2 = assign6910_e4611_d_n2;
        var_tmf1_dn6 = assign6910_e4611_d_n6;
        var_tmf1_dn7 = assign6910_e4611_d_n7;
        var_tmf1_dn10 = assign6910_e4611_d_n10;
        var_tmf1_dn11 = assign6910_e4611_d_n11;
        var_tmf1_dn12 = assign6910_e4611_d_n12;
        var_tmf1_dn17 = assign6910_e4611_d_n17;

        let (assign6920_e4620, assign6920_e4620_d_n0, assign6920_e4620_d_n2, assign6920_e4620_d_n6, assign6920_e4620_d_n7, assign6920_e4620_d_n10, assign6920_e4620_d_n11, assign6920_e4620_d_n12, assign6920_e4620_d_n17,) = {
    if (var_guard107 == 0.0) {
        let assign6920_e4616: f64 = 4.0;
        let assign6920_e4618: f64 = (assign6920_e4616 * 0.05);
        (assign6920_e4618, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6920_e4620;
        var_tmf2_dn0 = assign6920_e4620_d_n0;
        var_tmf2_dn2 = assign6920_e4620_d_n2;
        var_tmf2_dn6 = assign6920_e4620_d_n6;
        var_tmf2_dn7 = assign6920_e4620_d_n7;
        var_tmf2_dn10 = assign6920_e4620_d_n10;
        var_tmf2_dn11 = assign6920_e4620_d_n11;
        var_tmf2_dn12 = assign6920_e4620_d_n12;
        var_tmf2_dn17 = assign6920_e4620_d_n17;

        let (assign6930_e4631, assign6930_e4631_d_n0, assign6930_e4631_d_n2, assign6930_e4631_d_n6, assign6930_e4631_d_n7, assign6930_e4631_d_n10, assign6930_e4631_d_n11, assign6930_e4631_d_n12, assign6930_e4631_d_n17,) = {
    if (var_guard107 == 0.0) {
        let (assign6930_e4629, assign6930_e4629_d_n0, assign6930_e4629_d_n2, assign6930_e4629_d_n6, assign6930_e4629_d_n7, assign6930_e4629_d_n10, assign6930_e4629_d_n11, assign6930_e4629_d_n12, assign6930_e4629_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign6930_e4628: f64 = (-var_tmf2);
                (assign6930_e4628, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign6930_e4629, assign6930_e4629_d_n0, assign6930_e4629_d_n2, assign6930_e4629_d_n6, assign6930_e4629_d_n7, assign6930_e4629_d_n10, assign6930_e4629_d_n11, assign6930_e4629_d_n12, assign6930_e4629_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6930_e4631;
        var_tmf2_dn0 = assign6930_e4631_d_n0;
        var_tmf2_dn2 = assign6930_e4631_d_n2;
        var_tmf2_dn6 = assign6930_e4631_d_n6;
        var_tmf2_dn7 = assign6930_e4631_d_n7;
        var_tmf2_dn10 = assign6930_e4631_d_n10;
        var_tmf2_dn11 = assign6930_e4631_d_n11;
        var_tmf2_dn12 = assign6930_e4631_d_n12;
        var_tmf2_dn17 = assign6930_e4631_d_n17;

        let (assign6940_e4641, assign6940_e4641_d_n0, assign6940_e4641_d_n2, assign6940_e4641_d_n6, assign6940_e4641_d_n7, assign6940_e4641_d_n10, assign6940_e4641_d_n11, assign6940_e4641_d_n12, assign6940_e4641_d_n17,) = {
    if (var_guard107 == 0.0) {
        let assign6940_e4636: f64 = (var_tmf1 * var_tmf1);
        let assign6940_e4638: f64 = (assign6940_e4636 + var_tmf2);
        let assign6940_e4639: f64 = (assign6940_e4638).sqrt();
        (assign6940_e4639, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign6940_e4639)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign6940_e4639)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign6940_e4639)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign6940_e4639)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign6940_e4639)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign6940_e4639)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign6940_e4639)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign6940_e4639)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign6940_e4641;
        var_tmf2_dn0 = assign6940_e4641_d_n0;
        var_tmf2_dn2 = assign6940_e4641_d_n2;
        var_tmf2_dn6 = assign6940_e4641_d_n6;
        var_tmf2_dn7 = assign6940_e4641_d_n7;
        var_tmf2_dn10 = assign6940_e4641_d_n10;
        var_tmf2_dn11 = assign6940_e4641_d_n11;
        var_tmf2_dn12 = assign6940_e4641_d_n12;
        var_tmf2_dn17 = assign6940_e4641_d_n17;

        let (assign6950_e4652, assign6950_e4652_d_n0, assign6950_e4652_d_n2, assign6950_e4652_d_n6, assign6950_e4652_d_n7, assign6950_e4652_d_n10, assign6950_e4652_d_n11, assign6950_e4652_d_n12, assign6950_e4652_d_n17,) = {
    if (var_guard107 == 0.0) {
        let assign6950_e4648: f64 = (var_tmf1 + var_tmf2);
        let assign6950_e4649: f64 = (0.5 * assign6950_e4648);
        let assign6950_e4650: f64 = (1.0 - assign6950_e4649);
        (assign6950_e4650, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (-(0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (-(0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (-(0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (-(0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), (-(0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_dppg, var_dppg_dn0, var_dppg_dn2, var_dppg_dn6, var_dppg_dn7, var_dppg_dn10, var_dppg_dn11, var_dppg_dn12, var_dppg_dn17,)
    }
};
        var_dppg = assign6950_e4652;
        var_dppg_dn0 = assign6950_e4652_d_n0;
        var_dppg_dn2 = assign6950_e4652_d_n2;
        var_dppg_dn6 = assign6950_e4652_d_n6;
        var_dppg_dn7 = assign6950_e4652_d_n7;
        var_dppg_dn10 = assign6950_e4652_d_n10;
        var_dppg_dn11 = assign6950_e4652_d_n11;
        var_dppg_dn12 = assign6950_e4652_d_n12;
        var_dppg_dn17 = assign6950_e4652_d_n17;

        let assign6960_e4655: f64 = (var_vgs - var_vfb);
        let assign6960_e4657: f64 = (assign6960_e4655 + var_dvth);
        let assign6960_e4659: f64 = (assign6960_e4657 - var_dppg);
        var_vgp = assign6960_e4659;
        var_vgp_dn0 = (var_dvth_dn0 - var_dppg_dn0);
        var_vgp_dn2 = (var_dvth_dn2 - var_dppg_dn2);
        var_vgp_dn6 = ((var_vgs_dn6 + var_dvth_dn6) - var_dppg_dn6);
        var_vgp_dn7 = ((var_vgs_dn7 + var_dvth_dn7) - var_dppg_dn7);
        var_vgp_dn10 = (var_dvth_dn10 - var_dppg_dn10);
        var_vgp_dn11 = ((var_vgs_dn11 + var_dvth_dn11) - var_dppg_dn11);
        var_vgp_dn12 = (var_dvth_dn12 - var_dppg_dn12);
        var_vgp_dn17 = (var_dvth_dn17 - var_dppg_dn17);

        var_vgpz = var_vgp;
        var_vgpz_dn0 = var_vgp_dn0;
        var_vgpz_dn2 = var_vgp_dn2;
        var_vgpz_dn6 = var_vgp_dn6;
        var_vgpz_dn7 = var_vgp_dn7;
        var_vgpz_dn10 = var_vgp_dn10;
        var_vgpz_dn11 = var_vgp_dn11;
        var_vgpz_dn12 = var_vgp_dn12;
        var_vgpz_dn17 = var_vgp_dn17;

        let assign6980_e4663: f64 = (var_uc_nsubs / var_mks_nsubb);
        let assign6980_e4664: f64 = (assign6980_e4663).ln();
        var_t1 = assign6980_e4664;
        var_t1_dn0 = ((var_uc_nsubs_dn0 / var_mks_nsubb) / assign6980_e4663);
        var_t1_dn2 = ((var_uc_nsubs_dn2 / var_mks_nsubb) / assign6980_e4663);
        var_t1_dn6 = ((var_uc_nsubs_dn6 / var_mks_nsubb) / assign6980_e4663);
        var_t1_dn7 = ((var_uc_nsubs_dn7 / var_mks_nsubb) / assign6980_e4663);
        var_t1_dn10 = ((var_uc_nsubs_dn10 / var_mks_nsubb) / assign6980_e4663);
        var_t1_dn11 = ((var_uc_nsubs_dn11 / var_mks_nsubb) / assign6980_e4663);
        var_t1_dn12 = ((var_uc_nsubs_dn12 / var_mks_nsubb) / assign6980_e4663);
        var_t1_dn17 = ((var_uc_nsubs_dn17 / var_mks_nsubb) / assign6980_e4663);

        let assign6990_e4667: f64 = (var_beta_inv * var_t1);
        var_vbi_soi = assign6990_e4667;
        var_vbi_soi_dn0 = (var_beta_inv * var_t1_dn0);
        var_vbi_soi_dn2 = (var_beta_inv * var_t1_dn2);
        var_vbi_soi_dn6 = (var_beta_inv * var_t1_dn6);
        var_vbi_soi_dn7 = (var_beta_inv * var_t1_dn7);
        var_vbi_soi_dn10 = ((var_beta_inv_dn10 * var_t1) + (var_beta_inv * var_t1_dn10));
        var_vbi_soi_dn11 = (var_beta_inv * var_t1_dn11);
        var_vbi_soi_dn12 = (var_beta_inv * var_t1_dn12);
        var_vbi_soi_dn17 = (var_beta_inv * var_t1_dn17);

        let assign7000_e4670: f64 = (var_vfb - var_dvth);
        let assign7000_e4672: f64 = (assign7000_e4670 + var_dppg);
        var_vgs_fb = assign7000_e4672;

        let assign7010_e4675: f64 = (var_cnst0soi * var_c_fox_inv);
        var_fac1 = assign7010_e4675;
        var_fac1_dn0 = ((var_cnst0soi_dn0 * var_c_fox_inv) + (var_cnst0soi * var_c_fox_inv_dn0));
        var_fac1_dn2 = ((var_cnst0soi_dn2 * var_c_fox_inv) + (var_cnst0soi * var_c_fox_inv_dn2));
        var_fac1_dn6 = ((var_cnst0soi_dn6 * var_c_fox_inv) + (var_cnst0soi * var_c_fox_inv_dn6));
        var_fac1_dn7 = ((var_cnst0soi_dn7 * var_c_fox_inv) + (var_cnst0soi * var_c_fox_inv_dn7));
        var_fac1_dn10 = ((var_cnst0soi_dn10 * var_c_fox_inv) + (var_cnst0soi * var_c_fox_inv_dn10));
        var_fac1_dn11 = ((var_cnst0soi_dn11 * var_c_fox_inv) + (var_cnst0soi * var_c_fox_inv_dn11));
        var_fac1_dn12 = ((var_cnst0soi_dn12 * var_c_fox_inv) + (var_cnst0soi * var_c_fox_inv_dn12));
        var_fac1_dn17 = ((var_cnst0soi_dn17 * var_c_fox_inv) + (var_cnst0soi * var_c_fox_inv_dn17));

        let assign7020_e4678: f64 = (var_fac1 * var_fac1);
        var_fac1p2 = assign7020_e4678;
        var_fac1p2_dn0 = ((var_fac1_dn0 * var_fac1) + (var_fac1 * var_fac1_dn0));
        var_fac1p2_dn2 = ((var_fac1_dn2 * var_fac1) + (var_fac1 * var_fac1_dn2));
        var_fac1p2_dn6 = ((var_fac1_dn6 * var_fac1) + (var_fac1 * var_fac1_dn6));
        var_fac1p2_dn7 = ((var_fac1_dn7 * var_fac1) + (var_fac1 * var_fac1_dn7));
        var_fac1p2_dn10 = ((var_fac1_dn10 * var_fac1) + (var_fac1 * var_fac1_dn10));
        var_fac1p2_dn11 = ((var_fac1_dn11 * var_fac1) + (var_fac1 * var_fac1_dn11));
        var_fac1p2_dn12 = ((var_fac1_dn12 * var_fac1) + (var_fac1 * var_fac1_dn12));
        var_fac1p2_dn17 = ((var_fac1_dn17 * var_fac1) + (var_fac1 * var_fac1_dn17));

        let assign7030_e4681: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        var_guard111 = assign7030_e4681;

        let (assign7040_e4685,) = {
    if (var_guard111 != 0.0) {
        (7.0,)
    } else {
        (var_qdepb_dlt,)
    }
};
        var_qdepb_dlt = assign7040_e4685;

        let (assign7050_e4691, assign7050_e4691_d_n0, assign7050_e4691_d_n2, assign7050_e4691_d_n6, assign7050_e4691_d_n7, assign7050_e4691_d_n10, assign7050_e4691_d_n11, assign7050_e4691_d_n12, assign7050_e4691_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7050_e4689: f64 = (var_pb2 + 1.0);
        (assign7050_e4689, var_pb2_dn0, var_pb2_dn2, var_pb2_dn6, var_pb2_dn7, var_pb2_dn10, var_pb2_dn11, var_pb2_dn12, var_pb2_dn17,)
    } else {
        (var_vgp_ini, var_vgp_ini_dn0, var_vgp_ini_dn2, var_vgp_ini_dn6, var_vgp_ini_dn7, var_vgp_ini_dn10, var_vgp_ini_dn11, var_vgp_ini_dn12, var_vgp_ini_dn17,)
    }
};
        var_vgp_ini = assign7050_e4691;
        var_vgp_ini_dn0 = assign7050_e4691_d_n0;
        var_vgp_ini_dn2 = assign7050_e4691_d_n2;
        var_vgp_ini_dn6 = assign7050_e4691_d_n6;
        var_vgp_ini_dn7 = assign7050_e4691_d_n7;
        var_vgp_ini_dn10 = assign7050_e4691_d_n10;
        var_vgp_ini_dn11 = assign7050_e4691_d_n11;
        var_vgp_ini_dn12 = assign7050_e4691_d_n12;
        var_vgp_ini_dn17 = assign7050_e4691_d_n17;

        let (assign7060_e4699, assign7060_e4699_d_n0, assign7060_e4699_d_n2, assign7060_e4699_d_n6, assign7060_e4699_d_n7, assign7060_e4699_d_n10, assign7060_e4699_d_n11, assign7060_e4699_d_n12, assign7060_e4699_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7060_e4695: f64 = (1.0 / var_cnst1soi);
        let assign7060_e4697: f64 = (assign7060_e4695 / var_cnstc_foxi);
        (assign7060_e4697, ((((-(var_cnst1soi_dn0 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7060_e4695 * var_cnstc_foxi_dn0)) / (var_cnstc_foxi * var_cnstc_foxi)), ((((-(var_cnst1soi_dn2 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7060_e4695 * var_cnstc_foxi_dn2)) / (var_cnstc_foxi * var_cnstc_foxi)), ((((-(var_cnst1soi_dn6 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7060_e4695 * var_cnstc_foxi_dn6)) / (var_cnstc_foxi * var_cnstc_foxi)), ((((-(var_cnst1soi_dn7 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7060_e4695 * var_cnstc_foxi_dn7)) / (var_cnstc_foxi * var_cnstc_foxi)), ((((-(var_cnst1soi_dn10 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7060_e4695 * var_cnstc_foxi_dn10)) / (var_cnstc_foxi * var_cnstc_foxi)), ((((-(var_cnst1soi_dn11 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7060_e4695 * var_cnstc_foxi_dn11)) / (var_cnstc_foxi * var_cnstc_foxi)), ((((-(var_cnst1soi_dn12 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7060_e4695 * var_cnstc_foxi_dn12)) / (var_cnstc_foxi * var_cnstc_foxi)), ((((-(var_cnst1soi_dn17 / (var_cnst1soi * var_cnst1soi))) * var_cnstc_foxi) - (assign7060_e4695 * var_cnstc_foxi_dn17)) / (var_cnstc_foxi * var_cnstc_foxi)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign7060_e4699;
        var_t1_dn0 = assign7060_e4699_d_n0;
        var_t1_dn2 = assign7060_e4699_d_n2;
        var_t1_dn6 = assign7060_e4699_d_n6;
        var_t1_dn7 = assign7060_e4699_d_n7;
        var_t1_dn10 = assign7060_e4699_d_n10;
        var_t1_dn11 = assign7060_e4699_d_n11;
        var_t1_dn12 = assign7060_e4699_d_n12;
        var_t1_dn17 = assign7060_e4699_d_n17;

        let (assign7070_e4711, assign7070_e4711_d_n0, assign7070_e4711_d_n2, assign7070_e4711_d_n6, assign7070_e4711_d_n7, assign7070_e4711_d_n10, assign7070_e4711_d_n11, assign7070_e4711_d_n12, assign7070_e4711_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7070_e4704: f64 = (var_vgp_ini - var_shift);
        let assign7070_e4705: f64 = (var_t1 * assign7070_e4704);
        let assign7070_e4708: f64 = (var_vgp_ini - var_shift);
        let assign7070_e4709: f64 = (assign7070_e4705 * assign7070_e4708);
        (assign7070_e4709, ((((var_t1_dn0 * assign7070_e4704) + (var_t1 * (var_vgp_ini_dn0 - var_shift_dn0))) * assign7070_e4708) + (assign7070_e4705 * (var_vgp_ini_dn0 - var_shift_dn0))), ((((var_t1_dn2 * assign7070_e4704) + (var_t1 * (var_vgp_ini_dn2 - var_shift_dn2))) * assign7070_e4708) + (assign7070_e4705 * (var_vgp_ini_dn2 - var_shift_dn2))), ((((var_t1_dn6 * assign7070_e4704) + (var_t1 * (var_vgp_ini_dn6 - var_shift_dn6))) * assign7070_e4708) + (assign7070_e4705 * (var_vgp_ini_dn6 - var_shift_dn6))), ((((var_t1_dn7 * assign7070_e4704) + (var_t1 * (var_vgp_ini_dn7 - var_shift_dn7))) * assign7070_e4708) + (assign7070_e4705 * (var_vgp_ini_dn7 - var_shift_dn7))), ((((var_t1_dn10 * assign7070_e4704) + (var_t1 * (var_vgp_ini_dn10 - var_shift_dn10))) * assign7070_e4708) + (assign7070_e4705 * (var_vgp_ini_dn10 - var_shift_dn10))), ((((var_t1_dn11 * assign7070_e4704) + (var_t1 * (var_vgp_ini_dn11 - var_shift_dn11))) * assign7070_e4708) + (assign7070_e4705 * (var_vgp_ini_dn11 - var_shift_dn11))), ((((var_t1_dn12 * assign7070_e4704) + (var_t1 * (var_vgp_ini_dn12 - var_shift_dn12))) * assign7070_e4708) + (assign7070_e4705 * (var_vgp_ini_dn12 - var_shift_dn12))), ((((var_t1_dn17 * assign7070_e4704) + (var_t1 * (var_vgp_ini_dn17 - var_shift_dn17))) * assign7070_e4708) + (assign7070_e4705 * (var_vgp_ini_dn17 - var_shift_dn17))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign7070_e4711;
        var_t2_dn0 = assign7070_e4711_d_n0;
        var_t2_dn2 = assign7070_e4711_d_n2;
        var_t2_dn6 = assign7070_e4711_d_n6;
        var_t2_dn7 = assign7070_e4711_d_n7;
        var_t2_dn10 = assign7070_e4711_d_n10;
        var_t2_dn11 = assign7070_e4711_d_n11;
        var_t2_dn12 = assign7070_e4711_d_n12;
        var_t2_dn17 = assign7070_e4711_d_n17;

        let (assign7080_e4721, assign7080_e4721_d_n0, assign7080_e4721_d_n2, assign7080_e4721_d_n6, assign7080_e4721_d_n7, assign7080_e4721_d_n10, assign7080_e4721_d_n11, assign7080_e4721_d_n12, assign7080_e4721_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7080_e4717: f64 = (var_vgp_ini - var_shift);
        let assign7080_e4718: f64 = (2.0 / assign7080_e4717);
        let assign7080_e4719: f64 = (var_beta + assign7080_e4718);
        (assign7080_e4719, (-((2.0 * (var_vgp_ini_dn0 - var_shift_dn0)) / (assign7080_e4717 * assign7080_e4717))), (-((2.0 * (var_vgp_ini_dn2 - var_shift_dn2)) / (assign7080_e4717 * assign7080_e4717))), (-((2.0 * (var_vgp_ini_dn6 - var_shift_dn6)) / (assign7080_e4717 * assign7080_e4717))), (-((2.0 * (var_vgp_ini_dn7 - var_shift_dn7)) / (assign7080_e4717 * assign7080_e4717))), (var_beta_dn10 + (-((2.0 * (var_vgp_ini_dn10 - var_shift_dn10)) / (assign7080_e4717 * assign7080_e4717)))), (-((2.0 * (var_vgp_ini_dn11 - var_shift_dn11)) / (assign7080_e4717 * assign7080_e4717))), (-((2.0 * (var_vgp_ini_dn12 - var_shift_dn12)) / (assign7080_e4717 * assign7080_e4717))), (-((2.0 * (var_vgp_ini_dn17 - var_shift_dn17)) / (assign7080_e4717 * assign7080_e4717))),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign7080_e4721;
        var_t3_dn0 = assign7080_e4721_d_n0;
        var_t3_dn2 = assign7080_e4721_d_n2;
        var_t3_dn6 = assign7080_e4721_d_n6;
        var_t3_dn7 = assign7080_e4721_d_n7;
        var_t3_dn10 = assign7080_e4721_d_n10;
        var_t3_dn11 = assign7080_e4721_d_n11;
        var_t3_dn12 = assign7080_e4721_d_n12;
        var_t3_dn17 = assign7080_e4721_d_n17;

        let (assign7090_e4728, assign7090_e4728_d_n0, assign7090_e4728_d_n2, assign7090_e4728_d_n6, assign7090_e4728_d_n7, assign7090_e4728_d_n10, assign7090_e4728_d_n11, assign7090_e4728_d_n12, assign7090_e4728_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7090_e4724: f64 = (var_t2).ln();
        let assign7090_e4726: f64 = (assign7090_e4724 / var_t3);
        (assign7090_e4726, ((((var_t2_dn0 / var_t2) * var_t3) - (assign7090_e4724 * var_t3_dn0)) / (var_t3 * var_t3)), ((((var_t2_dn2 / var_t2) * var_t3) - (assign7090_e4724 * var_t3_dn2)) / (var_t3 * var_t3)), ((((var_t2_dn6 / var_t2) * var_t3) - (assign7090_e4724 * var_t3_dn6)) / (var_t3 * var_t3)), ((((var_t2_dn7 / var_t2) * var_t3) - (assign7090_e4724 * var_t3_dn7)) / (var_t3 * var_t3)), ((((var_t2_dn10 / var_t2) * var_t3) - (assign7090_e4724 * var_t3_dn10)) / (var_t3 * var_t3)), ((((var_t2_dn11 / var_t2) * var_t3) - (assign7090_e4724 * var_t3_dn11)) / (var_t3 * var_t3)), ((((var_t2_dn12 / var_t2) * var_t3) - (assign7090_e4724 * var_t3_dn12)) / (var_t3 * var_t3)), ((((var_t2_dn17 / var_t2) * var_t3) - (assign7090_e4724 * var_t3_dn17)) / (var_t3 * var_t3)),)
    } else {
        (var_ps0_inic, var_ps0_inic_dn0, var_ps0_inic_dn2, var_ps0_inic_dn6, var_ps0_inic_dn7, var_ps0_inic_dn10, var_ps0_inic_dn11, var_ps0_inic_dn12, var_ps0_inic_dn17,)
    }
};
        var_ps0_inic = assign7090_e4728;
        var_ps0_inic_dn0 = assign7090_e4728_d_n0;
        var_ps0_inic_dn2 = assign7090_e4728_d_n2;
        var_ps0_inic_dn6 = assign7090_e4728_d_n6;
        var_ps0_inic_dn7 = assign7090_e4728_d_n7;
        var_ps0_inic_dn10 = assign7090_e4728_d_n10;
        var_ps0_inic_dn11 = assign7090_e4728_d_n11;
        var_ps0_inic_dn12 = assign7090_e4728_d_n12;
        var_ps0_inic_dn17 = assign7090_e4728_d_n17;

        let (assign7100_e4735, assign7100_e4735_d_n0, assign7100_e4735_d_n2, assign7100_e4735_d_n6, assign7100_e4735_d_n7, assign7100_e4735_d_n10, assign7100_e4735_d_n11, assign7100_e4735_d_n12, assign7100_e4735_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7100_e4732: f64 = (var_cnst_2esi_q_nsubs * var_ps0_inic);
        let assign7100_e4733: f64 = (assign7100_e4732).sqrt();
        (assign7100_e4733, (((var_cnst_2esi_q_nsubs_dn0 * var_ps0_inic) + (var_cnst_2esi_q_nsubs * var_ps0_inic_dn0)) / (2.0 * assign7100_e4733)), (((var_cnst_2esi_q_nsubs_dn2 * var_ps0_inic) + (var_cnst_2esi_q_nsubs * var_ps0_inic_dn2)) / (2.0 * assign7100_e4733)), (((var_cnst_2esi_q_nsubs_dn6 * var_ps0_inic) + (var_cnst_2esi_q_nsubs * var_ps0_inic_dn6)) / (2.0 * assign7100_e4733)), (((var_cnst_2esi_q_nsubs_dn7 * var_ps0_inic) + (var_cnst_2esi_q_nsubs * var_ps0_inic_dn7)) / (2.0 * assign7100_e4733)), (((var_cnst_2esi_q_nsubs_dn10 * var_ps0_inic) + (var_cnst_2esi_q_nsubs * var_ps0_inic_dn10)) / (2.0 * assign7100_e4733)), (((var_cnst_2esi_q_nsubs_dn11 * var_ps0_inic) + (var_cnst_2esi_q_nsubs * var_ps0_inic_dn11)) / (2.0 * assign7100_e4733)), (((var_cnst_2esi_q_nsubs_dn12 * var_ps0_inic) + (var_cnst_2esi_q_nsubs * var_ps0_inic_dn12)) / (2.0 * assign7100_e4733)), (((var_cnst_2esi_q_nsubs_dn17 * var_ps0_inic) + (var_cnst_2esi_q_nsubs * var_ps0_inic_dn17)) / (2.0 * assign7100_e4733)),)
    } else {
        (var_wdsoi_ini0, var_wdsoi_ini0_dn0, var_wdsoi_ini0_dn2, var_wdsoi_ini0_dn6, var_wdsoi_ini0_dn7, var_wdsoi_ini0_dn10, var_wdsoi_ini0_dn11, var_wdsoi_ini0_dn12, var_wdsoi_ini0_dn17,)
    }
};
        var_wdsoi_ini0 = assign7100_e4735;
        var_wdsoi_ini0_dn0 = assign7100_e4735_d_n0;
        var_wdsoi_ini0_dn2 = assign7100_e4735_d_n2;
        var_wdsoi_ini0_dn6 = assign7100_e4735_d_n6;
        var_wdsoi_ini0_dn7 = assign7100_e4735_d_n7;
        var_wdsoi_ini0_dn10 = assign7100_e4735_d_n10;
        var_wdsoi_ini0_dn11 = assign7100_e4735_d_n11;
        var_wdsoi_ini0_dn12 = assign7100_e4735_d_n12;
        var_wdsoi_ini0_dn17 = assign7100_e4735_d_n17;

        let (assign7110_e4744, assign7110_e4744_d_n0, assign7110_e4744_d_n2, assign7110_e4744_d_n6, assign7110_e4744_d_n7, assign7110_e4744_d_n10, assign7110_e4744_d_n11, assign7110_e4744_d_n12, assign7110_e4744_d_n17,) = {
    if (var_guard111 != 0.0) {
        let (assign7110_e4742, assign7110_e4742_d_n0, assign7110_e4742_d_n2, assign7110_e4742_d_n6, assign7110_e4742_d_n7, assign7110_e4742_d_n10, assign7110_e4742_d_n11, assign7110_e4742_d_n12, assign7110_e4742_d_n17,) = {
            if (var_wdsoi_ini0 > p.p237) {
                (p.p237, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (var_wdsoi_ini0, var_wdsoi_ini0_dn0, var_wdsoi_ini0_dn2, var_wdsoi_ini0_dn6, var_wdsoi_ini0_dn7, var_wdsoi_ini0_dn10, var_wdsoi_ini0_dn11, var_wdsoi_ini0_dn12, var_wdsoi_ini0_dn17,)
            }
        };
        (assign7110_e4742, assign7110_e4742_d_n0, assign7110_e4742_d_n2, assign7110_e4742_d_n6, assign7110_e4742_d_n7, assign7110_e4742_d_n10, assign7110_e4742_d_n11, assign7110_e4742_d_n12, assign7110_e4742_d_n17,)
    } else {
        (var_wdsoi_ini0, var_wdsoi_ini0_dn0, var_wdsoi_ini0_dn2, var_wdsoi_ini0_dn6, var_wdsoi_ini0_dn7, var_wdsoi_ini0_dn10, var_wdsoi_ini0_dn11, var_wdsoi_ini0_dn12, var_wdsoi_ini0_dn17,)
    }
};
        var_wdsoi_ini0 = assign7110_e4744;
        var_wdsoi_ini0_dn0 = assign7110_e4744_d_n0;
        var_wdsoi_ini0_dn2 = assign7110_e4744_d_n2;
        var_wdsoi_ini0_dn6 = assign7110_e4744_d_n6;
        var_wdsoi_ini0_dn7 = assign7110_e4744_d_n7;
        var_wdsoi_ini0_dn10 = assign7110_e4744_d_n10;
        var_wdsoi_ini0_dn11 = assign7110_e4744_d_n11;
        var_wdsoi_ini0_dn12 = assign7110_e4744_d_n12;
        var_wdsoi_ini0_dn17 = assign7110_e4744_d_n17;

        *var_dppg_slot = var_dppg;
        *var_dppg_dn0_slot = var_dppg_dn0;
        *var_dppg_dn10_slot = var_dppg_dn10;
        *var_dppg_dn11_slot = var_dppg_dn11;
        *var_dppg_dn12_slot = var_dppg_dn12;
        *var_dppg_dn17_slot = var_dppg_dn17;
        *var_dppg_dn2_slot = var_dppg_dn2;
        *var_dppg_dn6_slot = var_dppg_dn6;
        *var_dppg_dn7_slot = var_dppg_dn7;
        *var_fac1_slot = var_fac1;
        *var_fac1_dn0_slot = var_fac1_dn0;
        *var_fac1_dn10_slot = var_fac1_dn10;
        *var_fac1_dn11_slot = var_fac1_dn11;
        *var_fac1_dn12_slot = var_fac1_dn12;
        *var_fac1_dn17_slot = var_fac1_dn17;
        *var_fac1_dn2_slot = var_fac1_dn2;
        *var_fac1_dn6_slot = var_fac1_dn6;
        *var_fac1_dn7_slot = var_fac1_dn7;
        *var_fac1p2_slot = var_fac1p2;
        *var_fac1p2_dn0_slot = var_fac1p2_dn0;
        *var_fac1p2_dn10_slot = var_fac1p2_dn10;
        *var_fac1p2_dn11_slot = var_fac1p2_dn11;
        *var_fac1p2_dn12_slot = var_fac1p2_dn12;
        *var_fac1p2_dn17_slot = var_fac1p2_dn17;
        *var_fac1p2_dn2_slot = var_fac1p2_dn2;
        *var_fac1p2_dn6_slot = var_fac1p2_dn6;
        *var_fac1p2_dn7_slot = var_fac1p2_dn7;
        *var_guard108_slot = var_guard108;
        *var_guard109_slot = var_guard109;
        *var_guard110_slot = var_guard110;
        *var_guard111_slot = var_guard111;
        *var_ps0_inic_slot = var_ps0_inic;
        *var_ps0_inic_dn0_slot = var_ps0_inic_dn0;
        *var_ps0_inic_dn10_slot = var_ps0_inic_dn10;
        *var_ps0_inic_dn11_slot = var_ps0_inic_dn11;
        *var_ps0_inic_dn12_slot = var_ps0_inic_dn12;
        *var_ps0_inic_dn17_slot = var_ps0_inic_dn17;
        *var_ps0_inic_dn2_slot = var_ps0_inic_dn2;
        *var_ps0_inic_dn6_slot = var_ps0_inic_dn6;
        *var_ps0_inic_dn7_slot = var_ps0_inic_dn7;
        *var_qdepb_dlt_slot = var_qdepb_dlt;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t3_slot = var_t3;
        *var_t3__blk105_slot = var_t3__blk105;
        *var_t3__blk105_dn0_slot = var_t3__blk105_dn0;
        *var_t3__blk105_dn10_slot = var_t3__blk105_dn10;
        *var_t3__blk105_dn11_slot = var_t3__blk105_dn11;
        *var_t3__blk105_dn12_slot = var_t3__blk105_dn12;
        *var_t3__blk105_dn17_slot = var_t3__blk105_dn17;
        *var_t3__blk105_dn2_slot = var_t3__blk105_dn2;
        *var_t3__blk105_dn6_slot = var_t3__blk105_dn6;
        *var_t3__blk105_dn7_slot = var_t3__blk105_dn7;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn17_slot = var_t3_dn17;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn17_slot = var_tmf2_dn17;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_vbi_soi_slot = var_vbi_soi;
        *var_vbi_soi_dn0_slot = var_vbi_soi_dn0;
        *var_vbi_soi_dn10_slot = var_vbi_soi_dn10;
        *var_vbi_soi_dn11_slot = var_vbi_soi_dn11;
        *var_vbi_soi_dn12_slot = var_vbi_soi_dn12;
        *var_vbi_soi_dn17_slot = var_vbi_soi_dn17;
        *var_vbi_soi_dn2_slot = var_vbi_soi_dn2;
        *var_vbi_soi_dn6_slot = var_vbi_soi_dn6;
        *var_vbi_soi_dn7_slot = var_vbi_soi_dn7;
        *var_vgp_slot = var_vgp;
        *var_vgp_dn0_slot = var_vgp_dn0;
        *var_vgp_dn10_slot = var_vgp_dn10;
        *var_vgp_dn11_slot = var_vgp_dn11;
        *var_vgp_dn12_slot = var_vgp_dn12;
        *var_vgp_dn17_slot = var_vgp_dn17;
        *var_vgp_dn2_slot = var_vgp_dn2;
        *var_vgp_dn6_slot = var_vgp_dn6;
        *var_vgp_dn7_slot = var_vgp_dn7;
        *var_vgp_ini_slot = var_vgp_ini;
        *var_vgp_ini_dn0_slot = var_vgp_ini_dn0;
        *var_vgp_ini_dn10_slot = var_vgp_ini_dn10;
        *var_vgp_ini_dn11_slot = var_vgp_ini_dn11;
        *var_vgp_ini_dn12_slot = var_vgp_ini_dn12;
        *var_vgp_ini_dn17_slot = var_vgp_ini_dn17;
        *var_vgp_ini_dn2_slot = var_vgp_ini_dn2;
        *var_vgp_ini_dn6_slot = var_vgp_ini_dn6;
        *var_vgp_ini_dn7_slot = var_vgp_ini_dn7;
        *var_vgpz_slot = var_vgpz;
        *var_vgpz_dn0_slot = var_vgpz_dn0;
        *var_vgpz_dn10_slot = var_vgpz_dn10;
        *var_vgpz_dn11_slot = var_vgpz_dn11;
        *var_vgpz_dn12_slot = var_vgpz_dn12;
        *var_vgpz_dn17_slot = var_vgpz_dn17;
        *var_vgpz_dn2_slot = var_vgpz_dn2;
        *var_vgpz_dn6_slot = var_vgpz_dn6;
        *var_vgpz_dn7_slot = var_vgpz_dn7;
        *var_vgs_fb_slot = var_vgs_fb;
        *var_wdsoi_ini0_slot = var_wdsoi_ini0;
        *var_wdsoi_ini0_dn0_slot = var_wdsoi_ini0_dn0;
        *var_wdsoi_ini0_dn10_slot = var_wdsoi_ini0_dn10;
        *var_wdsoi_ini0_dn11_slot = var_wdsoi_ini0_dn11;
        *var_wdsoi_ini0_dn12_slot = var_wdsoi_ini0_dn12;
        *var_wdsoi_ini0_dn17_slot = var_wdsoi_ini0_dn17;
        *var_wdsoi_ini0_dn2_slot = var_wdsoi_ini0_dn2;
        *var_wdsoi_ini0_dn6_slot = var_wdsoi_ini0_dn6;
        *var_wdsoi_ini0_dn7_slot = var_wdsoi_ini0_dn7;
    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn10: f64,
        var_c_box_fd_inv: f64,
        var_cnst0bulk: f64,
        var_cnst0bulk_dn10: f64,
        var_cnst1bulk: f64,
        var_cnst1bulk_dn0: f64,
        var_cnst1bulk_dn10: f64,
        var_cnst1bulk_dn11: f64,
        var_cnst1bulk_dn12: f64,
        var_cnst1bulk_dn17: f64,
        var_cnst1bulk_dn2: f64,
        var_cnst1bulk_dn6: f64,
        var_cnst1bulk_dn7: f64,
        var_guard111: f64,
        var_mks_nsubb: f64,
        var_nin: f64,
        var_uc_nsubs: f64,
        var_uc_nsubs_dn0: f64,
        var_uc_nsubs_dn10: f64,
        var_uc_nsubs_dn11: f64,
        var_uc_nsubs_dn12: f64,
        var_uc_nsubs_dn17: f64,
        var_uc_nsubs_dn2: f64,
        var_uc_nsubs_dn6: f64,
        var_uc_nsubs_dn7: f64,
        var_vbi_soi: f64,
        var_vbi_soi_dn0: f64,
        var_vbi_soi_dn10: f64,
        var_vbi_soi_dn11: f64,
        var_vbi_soi_dn12: f64,
        var_vbi_soi_dn17: f64,
        var_vbi_soi_dn2: f64,
        var_vbi_soi_dn6: f64,
        var_vbi_soi_dn7: f64,
        var_vbs: f64,
        var_vbs_dn0: f64,
        var_vbs_dn10: f64,
        var_vbs_dn11: f64,
        var_vbs_dn12: f64,
        var_vbs_dn17: f64,
        var_vbs_dn2: f64,
        var_vbs_dn6: f64,
        var_vbs_dn7: f64,
        var_vbsz: f64,
        var_vbsz_dn0: f64,
        var_vbsz_dn10: f64,
        var_vbsz_dn11: f64,
        var_vbsz_dn12: f64,
        var_vbsz_dn17: f64,
        var_vbsz_dn2: f64,
        var_vbsz_dn6: f64,
        var_vbsz_dn7: f64,
        var_wdsoi_ini0: f64,
        var_wdsoi_ini0_dn0: f64,
        var_wdsoi_ini0_dn10: f64,
        var_wdsoi_ini0_dn11: f64,
        var_wdsoi_ini0_dn12: f64,
        var_wdsoi_ini0_dn17: f64,
        var_wdsoi_ini0_dn2: f64,
        var_wdsoi_ini0_dn6: f64,
        var_wdsoi_ini0_dn7: f64,
        var_c_soi__blk112_slot: &mut f64,
        var_c_soi_inv__blk113_slot: &mut f64,
        var_guard125_slot: &mut f64,
        var_lp_s0_slot: &mut f64,
        var_pb2_bulk_slot: &mut f64,
        var_phi_s0_bulk_0_slot: &mut f64,
        var_phi_s0_bulk_0_dn0_slot: &mut f64,
        var_phi_s0_bulk_0_dn10_slot: &mut f64,
        var_phi_s0_bulk_0_dn11_slot: &mut f64,
        var_phi_s0_bulk_0_dn12_slot: &mut f64,
        var_phi_s0_bulk_0_dn17_slot: &mut f64,
        var_phi_s0_bulk_0_dn2_slot: &mut f64,
        var_phi_s0_bulk_0_dn6_slot: &mut f64,
        var_phi_s0_bulk_0_dn7_slot: &mut f64,
        var_psb_inia__blk123_slot: &mut f64,
        var_psb_inia__blk123_dn0_slot: &mut f64,
        var_psb_inia__blk123_dn10_slot: &mut f64,
        var_psb_inia__blk123_dn11_slot: &mut f64,
        var_psb_inia__blk123_dn12_slot: &mut f64,
        var_psb_inia__blk123_dn17_slot: &mut f64,
        var_psb_inia__blk123_dn2_slot: &mut f64,
        var_psb_inia__blk123_dn6_slot: &mut f64,
        var_psb_inia__blk123_dn7_slot: &mut f64,
        var_psb_inib__blk124_slot: &mut f64,
        var_psb_inib__blk124_dn0_slot: &mut f64,
        var_psb_inib__blk124_dn10_slot: &mut f64,
        var_psb_inib__blk124_dn11_slot: &mut f64,
        var_psb_inib__blk124_dn12_slot: &mut f64,
        var_psb_inib__blk124_dn17_slot: &mut f64,
        var_psb_inib__blk124_dn2_slot: &mut f64,
        var_psb_inib__blk124_dn6_slot: &mut f64,
        var_psb_inib__blk124_dn7_slot: &mut f64,
        var_q_fd_dlt1_slot: &mut f64,
        var_q_fd_dlt1_dn0_slot: &mut f64,
        var_q_fd_dlt1_dn10_slot: &mut f64,
        var_q_fd_dlt1_dn11_slot: &mut f64,
        var_q_fd_dlt1_dn12_slot: &mut f64,
        var_q_fd_dlt1_dn17_slot: &mut f64,
        var_q_fd_dlt1_dn2_slot: &mut f64,
        var_q_fd_dlt1_dn6_slot: &mut f64,
        var_q_fd_dlt1_dn7_slot: &mut f64,
        var_q_fd_dlt2_slot: &mut f64,
        var_q_fd_dlt2_dn0_slot: &mut f64,
        var_q_fd_dlt2_dn10_slot: &mut f64,
        var_q_fd_dlt2_dn11_slot: &mut f64,
        var_q_fd_dlt2_dn12_slot: &mut f64,
        var_q_fd_dlt2_dn17_slot: &mut f64,
        var_q_fd_dlt2_dn2_slot: &mut f64,
        var_q_fd_dlt2_dn6_slot: &mut f64,
        var_q_fd_dlt2_dn7_slot: &mut f64,
        var_q_fd_soi_slot: &mut f64,
        var_q_fd_soi_dn0_slot: &mut f64,
        var_q_fd_soi_dn10_slot: &mut f64,
        var_q_fd_soi_dn11_slot: &mut f64,
        var_q_fd_soi_dn12_slot: &mut f64,
        var_q_fd_soi_dn17_slot: &mut f64,
        var_q_fd_soi_dn2_slot: &mut f64,
        var_q_fd_soi_dn6_slot: &mut f64,
        var_q_fd_soi_dn7_slot: &mut f64,
        var_q_wdsoi_max_slot: &mut f64,
        var_q_wdsoi_max_dn0_slot: &mut f64,
        var_q_wdsoi_max_dn10_slot: &mut f64,
        var_q_wdsoi_max_dn11_slot: &mut f64,
        var_q_wdsoi_max_dn12_slot: &mut f64,
        var_q_wdsoi_max_dn17_slot: &mut f64,
        var_q_wdsoi_max_dn2_slot: &mut f64,
        var_q_wdsoi_max_dn6_slot: &mut f64,
        var_q_wdsoi_max_dn7_slot: &mut f64,
        var_t0__blk119_slot: &mut f64,
        var_t0__blk119_dn10_slot: &mut f64,
        var_t1__blk120_slot: &mut f64,
        var_t1__blk120_dn0_slot: &mut f64,
        var_t1__blk120_dn10_slot: &mut f64,
        var_t1__blk120_dn11_slot: &mut f64,
        var_t1__blk120_dn12_slot: &mut f64,
        var_t1__blk120_dn17_slot: &mut f64,
        var_t1__blk120_dn2_slot: &mut f64,
        var_t1__blk120_dn6_slot: &mut f64,
        var_t1__blk120_dn7_slot: &mut f64,
        var_t2__blk121_slot: &mut f64,
        var_t2__blk121_dn0_slot: &mut f64,
        var_t2__blk121_dn10_slot: &mut f64,
        var_t2__blk121_dn11_slot: &mut f64,
        var_t2__blk121_dn12_slot: &mut f64,
        var_t2__blk121_dn17_slot: &mut f64,
        var_t2__blk121_dn2_slot: &mut f64,
        var_t2__blk121_dn6_slot: &mut f64,
        var_t2__blk121_dn7_slot: &mut f64,
        var_t3__blk122_slot: &mut f64,
        var_t3__blk122_dn0_slot: &mut f64,
        var_t3__blk122_dn10_slot: &mut f64,
        var_t3__blk122_dn11_slot: &mut f64,
        var_t3__blk122_dn12_slot: &mut f64,
        var_t3__blk122_dn17_slot: &mut f64,
        var_t3__blk122_dn2_slot: &mut f64,
        var_t3__blk122_dn6_slot: &mut f64,
        var_t3__blk122_dn7_slot: &mut f64,
        var_t_soi_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn12_slot: &mut f64,
        var_tmf1_dn17_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn12_slot: &mut f64,
        var_tmf2_dn17_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_vbsbiz_slot: &mut f64,
        var_vbsbiz_dn0_slot: &mut f64,
        var_vbsbiz_dn10_slot: &mut f64,
        var_vbsbiz_dn11_slot: &mut f64,
        var_vbsbiz_dn12_slot: &mut f64,
        var_vbsbiz_dn17_slot: &mut f64,
        var_vbsbiz_dn2_slot: &mut f64,
        var_vbsbiz_dn6_slot: &mut f64,
        var_vbsbiz_dn7_slot: &mut f64,
        var_wdsoi_ini1_dlt_slot: &mut f64,
    ) {
        let mut var_c_soi__blk112: f64 = *var_c_soi__blk112_slot;
        let mut var_c_soi_inv__blk113: f64 = *var_c_soi_inv__blk113_slot;
        let mut var_guard125: f64 = *var_guard125_slot;
        let mut var_lp_s0: f64 = *var_lp_s0_slot;
        let mut var_pb2_bulk: f64 = *var_pb2_bulk_slot;
        let mut var_phi_s0_bulk_0: f64 = *var_phi_s0_bulk_0_slot;
        let mut var_phi_s0_bulk_0_dn0: f64 = *var_phi_s0_bulk_0_dn0_slot;
        let mut var_phi_s0_bulk_0_dn10: f64 = *var_phi_s0_bulk_0_dn10_slot;
        let mut var_phi_s0_bulk_0_dn11: f64 = *var_phi_s0_bulk_0_dn11_slot;
        let mut var_phi_s0_bulk_0_dn12: f64 = *var_phi_s0_bulk_0_dn12_slot;
        let mut var_phi_s0_bulk_0_dn17: f64 = *var_phi_s0_bulk_0_dn17_slot;
        let mut var_phi_s0_bulk_0_dn2: f64 = *var_phi_s0_bulk_0_dn2_slot;
        let mut var_phi_s0_bulk_0_dn6: f64 = *var_phi_s0_bulk_0_dn6_slot;
        let mut var_phi_s0_bulk_0_dn7: f64 = *var_phi_s0_bulk_0_dn7_slot;
        let mut var_psb_inia__blk123: f64 = *var_psb_inia__blk123_slot;
        let mut var_psb_inia__blk123_dn0: f64 = *var_psb_inia__blk123_dn0_slot;
        let mut var_psb_inia__blk123_dn10: f64 = *var_psb_inia__blk123_dn10_slot;
        let mut var_psb_inia__blk123_dn11: f64 = *var_psb_inia__blk123_dn11_slot;
        let mut var_psb_inia__blk123_dn12: f64 = *var_psb_inia__blk123_dn12_slot;
        let mut var_psb_inia__blk123_dn17: f64 = *var_psb_inia__blk123_dn17_slot;
        let mut var_psb_inia__blk123_dn2: f64 = *var_psb_inia__blk123_dn2_slot;
        let mut var_psb_inia__blk123_dn6: f64 = *var_psb_inia__blk123_dn6_slot;
        let mut var_psb_inia__blk123_dn7: f64 = *var_psb_inia__blk123_dn7_slot;
        let mut var_psb_inib__blk124: f64 = *var_psb_inib__blk124_slot;
        let mut var_psb_inib__blk124_dn0: f64 = *var_psb_inib__blk124_dn0_slot;
        let mut var_psb_inib__blk124_dn10: f64 = *var_psb_inib__blk124_dn10_slot;
        let mut var_psb_inib__blk124_dn11: f64 = *var_psb_inib__blk124_dn11_slot;
        let mut var_psb_inib__blk124_dn12: f64 = *var_psb_inib__blk124_dn12_slot;
        let mut var_psb_inib__blk124_dn17: f64 = *var_psb_inib__blk124_dn17_slot;
        let mut var_psb_inib__blk124_dn2: f64 = *var_psb_inib__blk124_dn2_slot;
        let mut var_psb_inib__blk124_dn6: f64 = *var_psb_inib__blk124_dn6_slot;
        let mut var_psb_inib__blk124_dn7: f64 = *var_psb_inib__blk124_dn7_slot;
        let mut var_q_fd_dlt1: f64 = *var_q_fd_dlt1_slot;
        let mut var_q_fd_dlt1_dn0: f64 = *var_q_fd_dlt1_dn0_slot;
        let mut var_q_fd_dlt1_dn10: f64 = *var_q_fd_dlt1_dn10_slot;
        let mut var_q_fd_dlt1_dn11: f64 = *var_q_fd_dlt1_dn11_slot;
        let mut var_q_fd_dlt1_dn12: f64 = *var_q_fd_dlt1_dn12_slot;
        let mut var_q_fd_dlt1_dn17: f64 = *var_q_fd_dlt1_dn17_slot;
        let mut var_q_fd_dlt1_dn2: f64 = *var_q_fd_dlt1_dn2_slot;
        let mut var_q_fd_dlt1_dn6: f64 = *var_q_fd_dlt1_dn6_slot;
        let mut var_q_fd_dlt1_dn7: f64 = *var_q_fd_dlt1_dn7_slot;
        let mut var_q_fd_dlt2: f64 = *var_q_fd_dlt2_slot;
        let mut var_q_fd_dlt2_dn0: f64 = *var_q_fd_dlt2_dn0_slot;
        let mut var_q_fd_dlt2_dn10: f64 = *var_q_fd_dlt2_dn10_slot;
        let mut var_q_fd_dlt2_dn11: f64 = *var_q_fd_dlt2_dn11_slot;
        let mut var_q_fd_dlt2_dn12: f64 = *var_q_fd_dlt2_dn12_slot;
        let mut var_q_fd_dlt2_dn17: f64 = *var_q_fd_dlt2_dn17_slot;
        let mut var_q_fd_dlt2_dn2: f64 = *var_q_fd_dlt2_dn2_slot;
        let mut var_q_fd_dlt2_dn6: f64 = *var_q_fd_dlt2_dn6_slot;
        let mut var_q_fd_dlt2_dn7: f64 = *var_q_fd_dlt2_dn7_slot;
        let mut var_q_fd_soi: f64 = *var_q_fd_soi_slot;
        let mut var_q_fd_soi_dn0: f64 = *var_q_fd_soi_dn0_slot;
        let mut var_q_fd_soi_dn10: f64 = *var_q_fd_soi_dn10_slot;
        let mut var_q_fd_soi_dn11: f64 = *var_q_fd_soi_dn11_slot;
        let mut var_q_fd_soi_dn12: f64 = *var_q_fd_soi_dn12_slot;
        let mut var_q_fd_soi_dn17: f64 = *var_q_fd_soi_dn17_slot;
        let mut var_q_fd_soi_dn2: f64 = *var_q_fd_soi_dn2_slot;
        let mut var_q_fd_soi_dn6: f64 = *var_q_fd_soi_dn6_slot;
        let mut var_q_fd_soi_dn7: f64 = *var_q_fd_soi_dn7_slot;
        let mut var_q_wdsoi_max: f64 = *var_q_wdsoi_max_slot;
        let mut var_q_wdsoi_max_dn0: f64 = *var_q_wdsoi_max_dn0_slot;
        let mut var_q_wdsoi_max_dn10: f64 = *var_q_wdsoi_max_dn10_slot;
        let mut var_q_wdsoi_max_dn11: f64 = *var_q_wdsoi_max_dn11_slot;
        let mut var_q_wdsoi_max_dn12: f64 = *var_q_wdsoi_max_dn12_slot;
        let mut var_q_wdsoi_max_dn17: f64 = *var_q_wdsoi_max_dn17_slot;
        let mut var_q_wdsoi_max_dn2: f64 = *var_q_wdsoi_max_dn2_slot;
        let mut var_q_wdsoi_max_dn6: f64 = *var_q_wdsoi_max_dn6_slot;
        let mut var_q_wdsoi_max_dn7: f64 = *var_q_wdsoi_max_dn7_slot;
        let mut var_t0__blk119: f64 = *var_t0__blk119_slot;
        let mut var_t0__blk119_dn10: f64 = *var_t0__blk119_dn10_slot;
        let mut var_t1__blk120: f64 = *var_t1__blk120_slot;
        let mut var_t1__blk120_dn0: f64 = *var_t1__blk120_dn0_slot;
        let mut var_t1__blk120_dn10: f64 = *var_t1__blk120_dn10_slot;
        let mut var_t1__blk120_dn11: f64 = *var_t1__blk120_dn11_slot;
        let mut var_t1__blk120_dn12: f64 = *var_t1__blk120_dn12_slot;
        let mut var_t1__blk120_dn17: f64 = *var_t1__blk120_dn17_slot;
        let mut var_t1__blk120_dn2: f64 = *var_t1__blk120_dn2_slot;
        let mut var_t1__blk120_dn6: f64 = *var_t1__blk120_dn6_slot;
        let mut var_t1__blk120_dn7: f64 = *var_t1__blk120_dn7_slot;
        let mut var_t2__blk121: f64 = *var_t2__blk121_slot;
        let mut var_t2__blk121_dn0: f64 = *var_t2__blk121_dn0_slot;
        let mut var_t2__blk121_dn10: f64 = *var_t2__blk121_dn10_slot;
        let mut var_t2__blk121_dn11: f64 = *var_t2__blk121_dn11_slot;
        let mut var_t2__blk121_dn12: f64 = *var_t2__blk121_dn12_slot;
        let mut var_t2__blk121_dn17: f64 = *var_t2__blk121_dn17_slot;
        let mut var_t2__blk121_dn2: f64 = *var_t2__blk121_dn2_slot;
        let mut var_t2__blk121_dn6: f64 = *var_t2__blk121_dn6_slot;
        let mut var_t2__blk121_dn7: f64 = *var_t2__blk121_dn7_slot;
        let mut var_t3__blk122: f64 = *var_t3__blk122_slot;
        let mut var_t3__blk122_dn0: f64 = *var_t3__blk122_dn0_slot;
        let mut var_t3__blk122_dn10: f64 = *var_t3__blk122_dn10_slot;
        let mut var_t3__blk122_dn11: f64 = *var_t3__blk122_dn11_slot;
        let mut var_t3__blk122_dn12: f64 = *var_t3__blk122_dn12_slot;
        let mut var_t3__blk122_dn17: f64 = *var_t3__blk122_dn17_slot;
        let mut var_t3__blk122_dn2: f64 = *var_t3__blk122_dn2_slot;
        let mut var_t3__blk122_dn6: f64 = *var_t3__blk122_dn6_slot;
        let mut var_t3__blk122_dn7: f64 = *var_t3__blk122_dn7_slot;
        let mut var_t_soi: f64 = *var_t_soi_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn12: f64 = *var_tmf1_dn12_slot;
        let mut var_tmf1_dn17: f64 = *var_tmf1_dn17_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn12: f64 = *var_tmf2_dn12_slot;
        let mut var_tmf2_dn17: f64 = *var_tmf2_dn17_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_vbsbiz: f64 = *var_vbsbiz_slot;
        let mut var_vbsbiz_dn0: f64 = *var_vbsbiz_dn0_slot;
        let mut var_vbsbiz_dn10: f64 = *var_vbsbiz_dn10_slot;
        let mut var_vbsbiz_dn11: f64 = *var_vbsbiz_dn11_slot;
        let mut var_vbsbiz_dn12: f64 = *var_vbsbiz_dn12_slot;
        let mut var_vbsbiz_dn17: f64 = *var_vbsbiz_dn17_slot;
        let mut var_vbsbiz_dn2: f64 = *var_vbsbiz_dn2_slot;
        let mut var_vbsbiz_dn6: f64 = *var_vbsbiz_dn6_slot;
        let mut var_vbsbiz_dn7: f64 = *var_vbsbiz_dn7_slot;
        let mut var_wdsoi_ini1_dlt: f64 = *var_wdsoi_ini1_dlt_slot;

        let (assign7120_e4753, assign7120_e4753_d_n0, assign7120_e4753_d_n2, assign7120_e4753_d_n6, assign7120_e4753_d_n7, assign7120_e4753_d_n10, assign7120_e4753_d_n11, assign7120_e4753_d_n12, assign7120_e4753_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7120_e4747: f64 = (-1.6021918e-19);
        let assign7120_e4749: f64 = (assign7120_e4747 * var_uc_nsubs);
        let assign7120_e4751: f64 = (assign7120_e4749 * var_wdsoi_ini0);
        (assign7120_e4751, (((assign7120_e4747 * var_uc_nsubs_dn0) * var_wdsoi_ini0) + (assign7120_e4749 * var_wdsoi_ini0_dn0)), (((assign7120_e4747 * var_uc_nsubs_dn2) * var_wdsoi_ini0) + (assign7120_e4749 * var_wdsoi_ini0_dn2)), (((assign7120_e4747 * var_uc_nsubs_dn6) * var_wdsoi_ini0) + (assign7120_e4749 * var_wdsoi_ini0_dn6)), (((assign7120_e4747 * var_uc_nsubs_dn7) * var_wdsoi_ini0) + (assign7120_e4749 * var_wdsoi_ini0_dn7)), (((assign7120_e4747 * var_uc_nsubs_dn10) * var_wdsoi_ini0) + (assign7120_e4749 * var_wdsoi_ini0_dn10)), (((assign7120_e4747 * var_uc_nsubs_dn11) * var_wdsoi_ini0) + (assign7120_e4749 * var_wdsoi_ini0_dn11)), (((assign7120_e4747 * var_uc_nsubs_dn12) * var_wdsoi_ini0) + (assign7120_e4749 * var_wdsoi_ini0_dn12)), (((assign7120_e4747 * var_uc_nsubs_dn17) * var_wdsoi_ini0) + (assign7120_e4749 * var_wdsoi_ini0_dn17)),)
    } else {
        (var_q_wdsoi_max, var_q_wdsoi_max_dn0, var_q_wdsoi_max_dn2, var_q_wdsoi_max_dn6, var_q_wdsoi_max_dn7, var_q_wdsoi_max_dn10, var_q_wdsoi_max_dn11, var_q_wdsoi_max_dn12, var_q_wdsoi_max_dn17,)
    }
};
        var_q_wdsoi_max = assign7120_e4753;
        var_q_wdsoi_max_dn0 = assign7120_e4753_d_n0;
        var_q_wdsoi_max_dn2 = assign7120_e4753_d_n2;
        var_q_wdsoi_max_dn6 = assign7120_e4753_d_n6;
        var_q_wdsoi_max_dn7 = assign7120_e4753_d_n7;
        var_q_wdsoi_max_dn10 = assign7120_e4753_d_n10;
        var_q_wdsoi_max_dn11 = assign7120_e4753_d_n11;
        var_q_wdsoi_max_dn12 = assign7120_e4753_d_n12;
        var_q_wdsoi_max_dn17 = assign7120_e4753_d_n17;

        let (assign7130_e4757,) = {
    if (var_guard111 != 0.0) {
        (p.p237,)
    } else {
        (var_t_soi,)
    }
};
        var_t_soi = assign7130_e4757;

        let (assign7140_e4766, assign7140_e4766_d_n0, assign7140_e4766_d_n2, assign7140_e4766_d_n6, assign7140_e4766_d_n7, assign7140_e4766_d_n10, assign7140_e4766_d_n11, assign7140_e4766_d_n12, assign7140_e4766_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7140_e4760: f64 = (-1.6021918e-19);
        let assign7140_e4762: f64 = (assign7140_e4760 * var_uc_nsubs);
        let assign7140_e4764: f64 = (assign7140_e4762 * var_t_soi);
        (assign7140_e4764, ((assign7140_e4760 * var_uc_nsubs_dn0) * var_t_soi), ((assign7140_e4760 * var_uc_nsubs_dn2) * var_t_soi), ((assign7140_e4760 * var_uc_nsubs_dn6) * var_t_soi), ((assign7140_e4760 * var_uc_nsubs_dn7) * var_t_soi), ((assign7140_e4760 * var_uc_nsubs_dn10) * var_t_soi), ((assign7140_e4760 * var_uc_nsubs_dn11) * var_t_soi), ((assign7140_e4760 * var_uc_nsubs_dn12) * var_t_soi), ((assign7140_e4760 * var_uc_nsubs_dn17) * var_t_soi),)
    } else {
        (var_q_fd_soi, var_q_fd_soi_dn0, var_q_fd_soi_dn2, var_q_fd_soi_dn6, var_q_fd_soi_dn7, var_q_fd_soi_dn10, var_q_fd_soi_dn11, var_q_fd_soi_dn12, var_q_fd_soi_dn17,)
    }
};
        var_q_fd_soi = assign7140_e4766;
        var_q_fd_soi_dn0 = assign7140_e4766_d_n0;
        var_q_fd_soi_dn2 = assign7140_e4766_d_n2;
        var_q_fd_soi_dn6 = assign7140_e4766_d_n6;
        var_q_fd_soi_dn7 = assign7140_e4766_d_n7;
        var_q_fd_soi_dn10 = assign7140_e4766_d_n10;
        var_q_fd_soi_dn11 = assign7140_e4766_d_n11;
        var_q_fd_soi_dn12 = assign7140_e4766_d_n12;
        var_q_fd_soi_dn17 = assign7140_e4766_d_n17;

        let (assign7150_e4770,) = {
    if (var_guard111 != 0.0) {
        (1.5,)
    } else {
        (var_wdsoi_ini1_dlt,)
    }
};
        var_wdsoi_ini1_dlt = assign7150_e4770;

        let (assign7160_e4776,) = {
    if (var_guard111 != 0.0) {
        let assign7160_e4774: f64 = (1.034943e-10 / var_t_soi);
        (assign7160_e4774,)
    } else {
        (var_c_soi__blk112,)
    }
};
        var_c_soi__blk112 = assign7160_e4776;

        let (assign7170_e4782,) = {
    if (var_guard111 != 0.0) {
        let assign7170_e4780: f64 = (1.0 / var_c_soi__blk112);
        (assign7170_e4780,)
    } else {
        (var_c_soi_inv__blk113,)
    }
};
        var_c_soi_inv__blk113 = assign7170_e4782;

        let (assign7180_e4789, assign7180_e4789_d_n0, assign7180_e4789_d_n2, assign7180_e4789_d_n6, assign7180_e4789_d_n7, assign7180_e4789_d_n10, assign7180_e4789_d_n11, assign7180_e4789_d_n12, assign7180_e4789_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7180_e4785: f64 = (-var_q_fd_soi);
        let assign7180_e4787: f64 = (assign7180_e4785 * 0.001);
        (assign7180_e4787, ((-var_q_fd_soi_dn0) * 0.001), ((-var_q_fd_soi_dn2) * 0.001), ((-var_q_fd_soi_dn6) * 0.001), ((-var_q_fd_soi_dn7) * 0.001), ((-var_q_fd_soi_dn10) * 0.001), ((-var_q_fd_soi_dn11) * 0.001), ((-var_q_fd_soi_dn12) * 0.001), ((-var_q_fd_soi_dn17) * 0.001),)
    } else {
        (var_q_fd_dlt1, var_q_fd_dlt1_dn0, var_q_fd_dlt1_dn2, var_q_fd_dlt1_dn6, var_q_fd_dlt1_dn7, var_q_fd_dlt1_dn10, var_q_fd_dlt1_dn11, var_q_fd_dlt1_dn12, var_q_fd_dlt1_dn17,)
    }
};
        var_q_fd_dlt1 = assign7180_e4789;
        var_q_fd_dlt1_dn0 = assign7180_e4789_d_n0;
        var_q_fd_dlt1_dn2 = assign7180_e4789_d_n2;
        var_q_fd_dlt1_dn6 = assign7180_e4789_d_n6;
        var_q_fd_dlt1_dn7 = assign7180_e4789_d_n7;
        var_q_fd_dlt1_dn10 = assign7180_e4789_d_n10;
        var_q_fd_dlt1_dn11 = assign7180_e4789_d_n11;
        var_q_fd_dlt1_dn12 = assign7180_e4789_d_n12;
        var_q_fd_dlt1_dn17 = assign7180_e4789_d_n17;

        let (assign7190_e4796, assign7190_e4796_d_n0, assign7190_e4796_d_n2, assign7190_e4796_d_n6, assign7190_e4796_d_n7, assign7190_e4796_d_n10, assign7190_e4796_d_n11, assign7190_e4796_d_n12, assign7190_e4796_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7190_e4792: f64 = (-var_q_fd_soi);
        let assign7190_e4794: f64 = (assign7190_e4792 * 1e-5);
        (assign7190_e4794, ((-var_q_fd_soi_dn0) * 1e-5), ((-var_q_fd_soi_dn2) * 1e-5), ((-var_q_fd_soi_dn6) * 1e-5), ((-var_q_fd_soi_dn7) * 1e-5), ((-var_q_fd_soi_dn10) * 1e-5), ((-var_q_fd_soi_dn11) * 1e-5), ((-var_q_fd_soi_dn12) * 1e-5), ((-var_q_fd_soi_dn17) * 1e-5),)
    } else {
        (var_q_fd_dlt2, var_q_fd_dlt2_dn0, var_q_fd_dlt2_dn2, var_q_fd_dlt2_dn6, var_q_fd_dlt2_dn7, var_q_fd_dlt2_dn10, var_q_fd_dlt2_dn11, var_q_fd_dlt2_dn12, var_q_fd_dlt2_dn17,)
    }
};
        var_q_fd_dlt2 = assign7190_e4796;
        var_q_fd_dlt2_dn0 = assign7190_e4796_d_n0;
        var_q_fd_dlt2_dn2 = assign7190_e4796_d_n2;
        var_q_fd_dlt2_dn6 = assign7190_e4796_d_n6;
        var_q_fd_dlt2_dn7 = assign7190_e4796_d_n7;
        var_q_fd_dlt2_dn10 = assign7190_e4796_d_n10;
        var_q_fd_dlt2_dn11 = assign7190_e4796_d_n11;
        var_q_fd_dlt2_dn12 = assign7190_e4796_d_n12;
        var_q_fd_dlt2_dn17 = assign7190_e4796_d_n17;

        let (assign7200_e4804, assign7200_e4804_d_n0, assign7200_e4804_d_n2, assign7200_e4804_d_n6, assign7200_e4804_d_n7, assign7200_e4804_d_n10, assign7200_e4804_d_n11, assign7200_e4804_d_n12, assign7200_e4804_d_n17,) = {
    if ((var_guard111 != 0.0) && (p.p39 != 0.0)) {
        let assign7200_e4802: f64 = (var_vbsz + var_vbi_soi);
        (assign7200_e4802, (var_vbsz_dn0 + var_vbi_soi_dn0), (var_vbsz_dn2 + var_vbi_soi_dn2), (var_vbsz_dn6 + var_vbi_soi_dn6), (var_vbsz_dn7 + var_vbi_soi_dn7), (var_vbsz_dn10 + var_vbi_soi_dn10), (var_vbsz_dn11 + var_vbi_soi_dn11), (var_vbsz_dn12 + var_vbi_soi_dn12), (var_vbsz_dn17 + var_vbi_soi_dn17),)
    } else {
        (var_vbsbiz, var_vbsbiz_dn0, var_vbsbiz_dn2, var_vbsbiz_dn6, var_vbsbiz_dn7, var_vbsbiz_dn10, var_vbsbiz_dn11, var_vbsbiz_dn12, var_vbsbiz_dn17,)
    }
};
        var_vbsbiz = assign7200_e4804;
        var_vbsbiz_dn0 = assign7200_e4804_d_n0;
        var_vbsbiz_dn2 = assign7200_e4804_d_n2;
        var_vbsbiz_dn6 = assign7200_e4804_d_n6;
        var_vbsbiz_dn7 = assign7200_e4804_d_n7;
        var_vbsbiz_dn10 = assign7200_e4804_d_n10;
        var_vbsbiz_dn11 = assign7200_e4804_d_n11;
        var_vbsbiz_dn12 = assign7200_e4804_d_n12;
        var_vbsbiz_dn17 = assign7200_e4804_d_n17;

        let (assign7210_e4813, assign7210_e4813_d_n0, assign7210_e4813_d_n2, assign7210_e4813_d_n6, assign7210_e4813_d_n7, assign7210_e4813_d_n10, assign7210_e4813_d_n11, assign7210_e4813_d_n12, assign7210_e4813_d_n17,) = {
    if ((var_guard111 != 0.0) && (p.p39 == 0.0)) {
        let assign7210_e4811: f64 = (var_vbs + var_vbi_soi);
        (assign7210_e4811, (var_vbs_dn0 + var_vbi_soi_dn0), (var_vbs_dn2 + var_vbi_soi_dn2), (var_vbs_dn6 + var_vbi_soi_dn6), (var_vbs_dn7 + var_vbi_soi_dn7), (var_vbs_dn10 + var_vbi_soi_dn10), (var_vbs_dn11 + var_vbi_soi_dn11), (var_vbs_dn12 + var_vbi_soi_dn12), (var_vbs_dn17 + var_vbi_soi_dn17),)
    } else {
        (var_vbsbiz, var_vbsbiz_dn0, var_vbsbiz_dn2, var_vbsbiz_dn6, var_vbsbiz_dn7, var_vbsbiz_dn10, var_vbsbiz_dn11, var_vbsbiz_dn12, var_vbsbiz_dn17,)
    }
};
        var_vbsbiz = assign7210_e4813;
        var_vbsbiz_dn0 = assign7210_e4813_d_n0;
        var_vbsbiz_dn2 = assign7210_e4813_d_n2;
        var_vbsbiz_dn6 = assign7210_e4813_d_n6;
        var_vbsbiz_dn7 = assign7210_e4813_d_n7;
        var_vbsbiz_dn10 = assign7210_e4813_d_n10;
        var_vbsbiz_dn11 = assign7210_e4813_d_n11;
        var_vbsbiz_dn12 = assign7210_e4813_d_n12;
        var_vbsbiz_dn17 = assign7210_e4813_d_n17;

        let (assign7220_e4824,) = {
    if (var_guard111 != 0.0) {
        let assign7220_e4817: f64 = (2.0 / var_beta);
        let assign7220_e4820: f64 = (var_mks_nsubb / var_nin);
        let assign7220_e4821: f64 = (assign7220_e4820).ln();
        let assign7220_e4822: f64 = (assign7220_e4817 * assign7220_e4821);
        (assign7220_e4822,)
    } else {
        (var_pb2_bulk,)
    }
};
        var_pb2_bulk = assign7220_e4824;

        let (assign7230_e4834, assign7230_e4834_d_n10,) = {
    if (var_guard111 != 0.0) {
        let assign7230_e4828: f64 = (var_cnst0bulk * var_cnst0bulk);
        let assign7230_e4830: f64 = (assign7230_e4828 * var_c_box_fd_inv);
        let assign7230_e4832: f64 = (assign7230_e4830 * var_c_box_fd_inv);
        (assign7230_e4832, ((((var_cnst0bulk_dn10 * var_cnst0bulk) + (var_cnst0bulk * var_cnst0bulk_dn10)) * var_c_box_fd_inv) * var_c_box_fd_inv),)
    } else {
        (var_t0__blk119, var_t0__blk119_dn10,)
    }
};
        var_t0__blk119 = assign7230_e4834;
        var_t0__blk119_dn10 = assign7230_e4834_d_n10;

        let (assign7240_e4839, assign7240_e4839_d_n0, assign7240_e4839_d_n2, assign7240_e4839_d_n6, assign7240_e4839_d_n7, assign7240_e4839_d_n10, assign7240_e4839_d_n11, assign7240_e4839_d_n12, assign7240_e4839_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7240_e4837: f64 = (-var_vbsbiz);
        (assign7240_e4837, (-var_vbsbiz_dn0), (-var_vbsbiz_dn2), (-var_vbsbiz_dn6), (-var_vbsbiz_dn7), (-var_vbsbiz_dn10), (-var_vbsbiz_dn11), (-var_vbsbiz_dn12), (-var_vbsbiz_dn17),)
    } else {
        (var_t1__blk120, var_t1__blk120_dn0, var_t1__blk120_dn2, var_t1__blk120_dn6, var_t1__blk120_dn7, var_t1__blk120_dn10, var_t1__blk120_dn11, var_t1__blk120_dn12, var_t1__blk120_dn17,)
    }
};
        var_t1__blk120 = assign7240_e4839;
        var_t1__blk120_dn0 = assign7240_e4839_d_n0;
        var_t1__blk120_dn2 = assign7240_e4839_d_n2;
        var_t1__blk120_dn6 = assign7240_e4839_d_n6;
        var_t1__blk120_dn7 = assign7240_e4839_d_n7;
        var_t1__blk120_dn10 = assign7240_e4839_d_n10;
        var_t1__blk120_dn11 = assign7240_e4839_d_n11;
        var_t1__blk120_dn12 = assign7240_e4839_d_n12;
        var_t1__blk120_dn17 = assign7240_e4839_d_n17;

        let (assign7250_e4865, assign7250_e4865_d_n0, assign7250_e4865_d_n2, assign7250_e4865_d_n6, assign7250_e4865_d_n7, assign7250_e4865_d_n10, assign7250_e4865_d_n11, assign7250_e4865_d_n12, assign7250_e4865_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7250_e4843: f64 = (2.0 * var_t1__blk120);
        let assign7250_e4846: f64 = (var_t0__blk119 * var_beta);
        let assign7250_e4847: f64 = (assign7250_e4843 + assign7250_e4846);
        let assign7250_e4850: f64 = (2.0 * var_t1__blk120);
        let assign7250_e4853: f64 = (var_t0__blk119 * var_beta);
        let assign7250_e4854: f64 = (assign7250_e4850 + assign7250_e4853);
        let assign7250_e4855: f64 = (assign7250_e4847 * assign7250_e4854);
        let assign7250_e4859: f64 = (var_t1__blk120 * var_t1__blk120);
        let assign7250_e4861: f64 = (assign7250_e4859 + var_t0__blk119);
        let assign7250_e4862: f64 = (4.0 * assign7250_e4861);
        let assign7250_e4863: f64 = (assign7250_e4855 - assign7250_e4862);
        (assign7250_e4863, ((((2.0 * var_t1__blk120_dn0) * assign7250_e4854) + (assign7250_e4847 * (2.0 * var_t1__blk120_dn0))) - (4.0 * ((var_t1__blk120_dn0 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn0)))), ((((2.0 * var_t1__blk120_dn2) * assign7250_e4854) + (assign7250_e4847 * (2.0 * var_t1__blk120_dn2))) - (4.0 * ((var_t1__blk120_dn2 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn2)))), ((((2.0 * var_t1__blk120_dn6) * assign7250_e4854) + (assign7250_e4847 * (2.0 * var_t1__blk120_dn6))) - (4.0 * ((var_t1__blk120_dn6 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn6)))), ((((2.0 * var_t1__blk120_dn7) * assign7250_e4854) + (assign7250_e4847 * (2.0 * var_t1__blk120_dn7))) - (4.0 * ((var_t1__blk120_dn7 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn7)))), (((((2.0 * var_t1__blk120_dn10) + ((var_t0__blk119_dn10 * var_beta) + (var_t0__blk119 * var_beta_dn10))) * assign7250_e4854) + (assign7250_e4847 * ((2.0 * var_t1__blk120_dn10) + ((var_t0__blk119_dn10 * var_beta) + (var_t0__blk119 * var_beta_dn10))))) - (4.0 * (((var_t1__blk120_dn10 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn10)) + var_t0__blk119_dn10))), ((((2.0 * var_t1__blk120_dn11) * assign7250_e4854) + (assign7250_e4847 * (2.0 * var_t1__blk120_dn11))) - (4.0 * ((var_t1__blk120_dn11 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn11)))), ((((2.0 * var_t1__blk120_dn12) * assign7250_e4854) + (assign7250_e4847 * (2.0 * var_t1__blk120_dn12))) - (4.0 * ((var_t1__blk120_dn12 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn12)))), ((((2.0 * var_t1__blk120_dn17) * assign7250_e4854) + (assign7250_e4847 * (2.0 * var_t1__blk120_dn17))) - (4.0 * ((var_t1__blk120_dn17 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn17)))),)
    } else {
        (var_t2__blk121, var_t2__blk121_dn0, var_t2__blk121_dn2, var_t2__blk121_dn6, var_t2__blk121_dn7, var_t2__blk121_dn10, var_t2__blk121_dn11, var_t2__blk121_dn12, var_t2__blk121_dn17,)
    }
};
        var_t2__blk121 = assign7250_e4865;
        var_t2__blk121_dn0 = assign7250_e4865_d_n0;
        var_t2__blk121_dn2 = assign7250_e4865_d_n2;
        var_t2__blk121_dn6 = assign7250_e4865_d_n6;
        var_t2__blk121_dn7 = assign7250_e4865_d_n7;
        var_t2__blk121_dn10 = assign7250_e4865_d_n10;
        var_t2__blk121_dn11 = assign7250_e4865_d_n11;
        var_t2__blk121_dn12 = assign7250_e4865_d_n12;
        var_t2__blk121_dn17 = assign7250_e4865_d_n17;

        let (assign7260_e4878, assign7260_e4878_d_n0, assign7260_e4878_d_n2, assign7260_e4878_d_n6, assign7260_e4878_d_n7, assign7260_e4878_d_n10, assign7260_e4878_d_n11, assign7260_e4878_d_n12, assign7260_e4878_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7260_e4870: f64 = (10.0 * 2.220446049250313e-16);
        let (assign7260_e4876, assign7260_e4876_d_n0, assign7260_e4876_d_n2, assign7260_e4876_d_n6, assign7260_e4876_d_n7, assign7260_e4876_d_n10, assign7260_e4876_d_n11, assign7260_e4876_d_n12, assign7260_e4876_d_n17,) = {
            if (var_t2__blk121 >= assign7260_e4870) {
                (var_t2__blk121, var_t2__blk121_dn0, var_t2__blk121_dn2, var_t2__blk121_dn6, var_t2__blk121_dn7, var_t2__blk121_dn10, var_t2__blk121_dn11, var_t2__blk121_dn12, var_t2__blk121_dn17,)
            } else {
                let assign7260_e4875: f64 = (10.0 * 2.220446049250313e-16);
                (assign7260_e4875, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign7260_e4876, assign7260_e4876_d_n0, assign7260_e4876_d_n2, assign7260_e4876_d_n6, assign7260_e4876_d_n7, assign7260_e4876_d_n10, assign7260_e4876_d_n11, assign7260_e4876_d_n12, assign7260_e4876_d_n17,)
    } else {
        (var_t2__blk121, var_t2__blk121_dn0, var_t2__blk121_dn2, var_t2__blk121_dn6, var_t2__blk121_dn7, var_t2__blk121_dn10, var_t2__blk121_dn11, var_t2__blk121_dn12, var_t2__blk121_dn17,)
    }
};
        var_t2__blk121 = assign7260_e4878;
        var_t2__blk121_dn0 = assign7260_e4878_d_n0;
        var_t2__blk121_dn2 = assign7260_e4878_d_n2;
        var_t2__blk121_dn6 = assign7260_e4878_d_n6;
        var_t2__blk121_dn7 = assign7260_e4878_d_n7;
        var_t2__blk121_dn10 = assign7260_e4878_d_n10;
        var_t2__blk121_dn11 = assign7260_e4878_d_n11;
        var_t2__blk121_dn12 = assign7260_e4878_d_n12;
        var_t2__blk121_dn17 = assign7260_e4878_d_n17;

        let (assign7270_e4883, assign7270_e4883_d_n0, assign7270_e4883_d_n2, assign7270_e4883_d_n6, assign7270_e4883_d_n7, assign7270_e4883_d_n10, assign7270_e4883_d_n11, assign7270_e4883_d_n12, assign7270_e4883_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7270_e4881: f64 = (var_t2__blk121).sqrt();
        (assign7270_e4881, (var_t2__blk121_dn0 / (2.0 * assign7270_e4881)), (var_t2__blk121_dn2 / (2.0 * assign7270_e4881)), (var_t2__blk121_dn6 / (2.0 * assign7270_e4881)), (var_t2__blk121_dn7 / (2.0 * assign7270_e4881)), (var_t2__blk121_dn10 / (2.0 * assign7270_e4881)), (var_t2__blk121_dn11 / (2.0 * assign7270_e4881)), (var_t2__blk121_dn12 / (2.0 * assign7270_e4881)), (var_t2__blk121_dn17 / (2.0 * assign7270_e4881)),)
    } else {
        (var_t2__blk121, var_t2__blk121_dn0, var_t2__blk121_dn2, var_t2__blk121_dn6, var_t2__blk121_dn7, var_t2__blk121_dn10, var_t2__blk121_dn11, var_t2__blk121_dn12, var_t2__blk121_dn17,)
    }
};
        var_t2__blk121 = assign7270_e4883;
        var_t2__blk121_dn0 = assign7270_e4883_d_n0;
        var_t2__blk121_dn2 = assign7270_e4883_d_n2;
        var_t2__blk121_dn6 = assign7270_e4883_d_n6;
        var_t2__blk121_dn7 = assign7270_e4883_d_n7;
        var_t2__blk121_dn10 = assign7270_e4883_d_n10;
        var_t2__blk121_dn11 = assign7270_e4883_d_n11;
        var_t2__blk121_dn12 = assign7270_e4883_d_n12;
        var_t2__blk121_dn17 = assign7270_e4883_d_n17;

        let (assign7280_e4893, assign7280_e4893_d_n0, assign7280_e4893_d_n2, assign7280_e4893_d_n6, assign7280_e4893_d_n7, assign7280_e4893_d_n10, assign7280_e4893_d_n11, assign7280_e4893_d_n12, assign7280_e4893_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7280_e4887: f64 = (2.0 * var_t1__blk120);
        let assign7280_e4890: f64 = (var_t0__blk119 * var_beta);
        let assign7280_e4891: f64 = (assign7280_e4887 + assign7280_e4890);
        (assign7280_e4891, (2.0 * var_t1__blk120_dn0), (2.0 * var_t1__blk120_dn2), (2.0 * var_t1__blk120_dn6), (2.0 * var_t1__blk120_dn7), ((2.0 * var_t1__blk120_dn10) + ((var_t0__blk119_dn10 * var_beta) + (var_t0__blk119 * var_beta_dn10))), (2.0 * var_t1__blk120_dn11), (2.0 * var_t1__blk120_dn12), (2.0 * var_t1__blk120_dn17),)
    } else {
        (var_t3__blk122, var_t3__blk122_dn0, var_t3__blk122_dn2, var_t3__blk122_dn6, var_t3__blk122_dn7, var_t3__blk122_dn10, var_t3__blk122_dn11, var_t3__blk122_dn12, var_t3__blk122_dn17,)
    }
};
        var_t3__blk122 = assign7280_e4893;
        var_t3__blk122_dn0 = assign7280_e4893_d_n0;
        var_t3__blk122_dn2 = assign7280_e4893_d_n2;
        var_t3__blk122_dn6 = assign7280_e4893_d_n6;
        var_t3__blk122_dn7 = assign7280_e4893_d_n7;
        var_t3__blk122_dn10 = assign7280_e4893_d_n10;
        var_t3__blk122_dn11 = assign7280_e4893_d_n11;
        var_t3__blk122_dn12 = assign7280_e4893_d_n12;
        var_t3__blk122_dn17 = assign7280_e4893_d_n17;

        let (assign7290_e4901, assign7290_e4901_d_n0, assign7290_e4901_d_n2, assign7290_e4901_d_n6, assign7290_e4901_d_n7, assign7290_e4901_d_n10, assign7290_e4901_d_n11, assign7290_e4901_d_n12, assign7290_e4901_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7290_e4897: f64 = (var_t3__blk122 - var_t2__blk121);
        let assign7290_e4899: f64 = (assign7290_e4897 / 2.0);
        (assign7290_e4899, ((var_t3__blk122_dn0 - var_t2__blk121_dn0) / 2.0), ((var_t3__blk122_dn2 - var_t2__blk121_dn2) / 2.0), ((var_t3__blk122_dn6 - var_t2__blk121_dn6) / 2.0), ((var_t3__blk122_dn7 - var_t2__blk121_dn7) / 2.0), ((var_t3__blk122_dn10 - var_t2__blk121_dn10) / 2.0), ((var_t3__blk122_dn11 - var_t2__blk121_dn11) / 2.0), ((var_t3__blk122_dn12 - var_t2__blk121_dn12) / 2.0), ((var_t3__blk122_dn17 - var_t2__blk121_dn17) / 2.0),)
    } else {
        (var_psb_inia__blk123, var_psb_inia__blk123_dn0, var_psb_inia__blk123_dn2, var_psb_inia__blk123_dn6, var_psb_inia__blk123_dn7, var_psb_inia__blk123_dn10, var_psb_inia__blk123_dn11, var_psb_inia__blk123_dn12, var_psb_inia__blk123_dn17,)
    }
};
        var_psb_inia__blk123 = assign7290_e4901;
        var_psb_inia__blk123_dn0 = assign7290_e4901_d_n0;
        var_psb_inia__blk123_dn2 = assign7290_e4901_d_n2;
        var_psb_inia__blk123_dn6 = assign7290_e4901_d_n6;
        var_psb_inia__blk123_dn7 = assign7290_e4901_d_n7;
        var_psb_inia__blk123_dn10 = assign7290_e4901_d_n10;
        var_psb_inia__blk123_dn11 = assign7290_e4901_d_n11;
        var_psb_inia__blk123_dn12 = assign7290_e4901_d_n12;
        var_psb_inia__blk123_dn17 = assign7290_e4901_d_n17;

        let (assign7300_e4918, assign7300_e4918_d_n0, assign7300_e4918_d_n2, assign7300_e4918_d_n6, assign7300_e4918_d_n7, assign7300_e4918_d_n10, assign7300_e4918_d_n11, assign7300_e4918_d_n12, assign7300_e4918_d_n17,) = {
    if (var_guard111 != 0.0) {
        let assign7300_e4905: f64 = (var_t1__blk120 * var_t1__blk120);
        let assign7300_e4907: f64 = (assign7300_e4905 / var_t0__blk119);
        let assign7300_e4909: f64 = (assign7300_e4907 / var_cnst1bulk);
        let assign7300_e4910: f64 = (assign7300_e4909).ln();
        let assign7300_e4914: f64 = (2.0 / var_t1__blk120);
        let assign7300_e4915: f64 = (var_beta + assign7300_e4914);
        let assign7300_e4916: f64 = (assign7300_e4910 / assign7300_e4915);
        (assign7300_e4916, ((((((((((var_t1__blk120_dn0 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn0)) / var_t0__blk119) * var_cnst1bulk) - (assign7300_e4907 * var_cnst1bulk_dn0)) / (var_cnst1bulk * var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (-((2.0 * var_t1__blk120_dn0) / (var_t1__blk120 * var_t1__blk120))))) / (assign7300_e4915 * assign7300_e4915)), ((((((((((var_t1__blk120_dn2 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn2)) / var_t0__blk119) * var_cnst1bulk) - (assign7300_e4907 * var_cnst1bulk_dn2)) / (var_cnst1bulk * var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (-((2.0 * var_t1__blk120_dn2) / (var_t1__blk120 * var_t1__blk120))))) / (assign7300_e4915 * assign7300_e4915)), ((((((((((var_t1__blk120_dn6 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn6)) / var_t0__blk119) * var_cnst1bulk) - (assign7300_e4907 * var_cnst1bulk_dn6)) / (var_cnst1bulk * var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (-((2.0 * var_t1__blk120_dn6) / (var_t1__blk120 * var_t1__blk120))))) / (assign7300_e4915 * assign7300_e4915)), ((((((((((var_t1__blk120_dn7 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn7)) / var_t0__blk119) * var_cnst1bulk) - (assign7300_e4907 * var_cnst1bulk_dn7)) / (var_cnst1bulk * var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (-((2.0 * var_t1__blk120_dn7) / (var_t1__blk120 * var_t1__blk120))))) / (assign7300_e4915 * assign7300_e4915)), ((((((((((((var_t1__blk120_dn10 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn10)) * var_t0__blk119) - (assign7300_e4905 * var_t0__blk119_dn10)) / (var_t0__blk119 * var_t0__blk119)) * var_cnst1bulk) - (assign7300_e4907 * var_cnst1bulk_dn10)) / (var_cnst1bulk * var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (var_beta_dn10 + (-((2.0 * var_t1__blk120_dn10) / (var_t1__blk120 * var_t1__blk120)))))) / (assign7300_e4915 * assign7300_e4915)), ((((((((((var_t1__blk120_dn11 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn11)) / var_t0__blk119) * var_cnst1bulk) - (assign7300_e4907 * var_cnst1bulk_dn11)) / (var_cnst1bulk * var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (-((2.0 * var_t1__blk120_dn11) / (var_t1__blk120 * var_t1__blk120))))) / (assign7300_e4915 * assign7300_e4915)), ((((((((((var_t1__blk120_dn12 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn12)) / var_t0__blk119) * var_cnst1bulk) - (assign7300_e4907 * var_cnst1bulk_dn12)) / (var_cnst1bulk * var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (-((2.0 * var_t1__blk120_dn12) / (var_t1__blk120 * var_t1__blk120))))) / (assign7300_e4915 * assign7300_e4915)), ((((((((((var_t1__blk120_dn17 * var_t1__blk120) + (var_t1__blk120 * var_t1__blk120_dn17)) / var_t0__blk119) * var_cnst1bulk) - (assign7300_e4907 * var_cnst1bulk_dn17)) / (var_cnst1bulk * var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (-((2.0 * var_t1__blk120_dn17) / (var_t1__blk120 * var_t1__blk120))))) / (assign7300_e4915 * assign7300_e4915)),)
    } else {
        (var_psb_inib__blk124, var_psb_inib__blk124_dn0, var_psb_inib__blk124_dn2, var_psb_inib__blk124_dn6, var_psb_inib__blk124_dn7, var_psb_inib__blk124_dn10, var_psb_inib__blk124_dn11, var_psb_inib__blk124_dn12, var_psb_inib__blk124_dn17,)
    }
};
        var_psb_inib__blk124 = assign7300_e4918;
        var_psb_inib__blk124_dn0 = assign7300_e4918_d_n0;
        var_psb_inib__blk124_dn2 = assign7300_e4918_d_n2;
        var_psb_inib__blk124_dn6 = assign7300_e4918_d_n6;
        var_psb_inib__blk124_dn7 = assign7300_e4918_d_n7;
        var_psb_inib__blk124_dn10 = assign7300_e4918_d_n10;
        var_psb_inib__blk124_dn11 = assign7300_e4918_d_n11;
        var_psb_inib__blk124_dn12 = assign7300_e4918_d_n12;
        var_psb_inib__blk124_dn17 = assign7300_e4918_d_n17;

        let assign7310_e4921: f64 = if var_psb_inia__blk123 < var_pb2_bulk { 1.0 } else { 0.0 };
        var_guard125 = assign7310_e4921;

        let (assign7320_e4927, assign7320_e4927_d_n0, assign7320_e4927_d_n2, assign7320_e4927_d_n6, assign7320_e4927_d_n7, assign7320_e4927_d_n10, assign7320_e4927_d_n11, assign7320_e4927_d_n12, assign7320_e4927_d_n17,) = {
    if ((var_guard111 != 0.0) && (var_guard125 != 0.0)) {
        (var_psb_inia__blk123, var_psb_inia__blk123_dn0, var_psb_inia__blk123_dn2, var_psb_inia__blk123_dn6, var_psb_inia__blk123_dn7, var_psb_inia__blk123_dn10, var_psb_inia__blk123_dn11, var_psb_inia__blk123_dn12, var_psb_inia__blk123_dn17,)
    } else {
        (var_phi_s0_bulk_0, var_phi_s0_bulk_0_dn0, var_phi_s0_bulk_0_dn2, var_phi_s0_bulk_0_dn6, var_phi_s0_bulk_0_dn7, var_phi_s0_bulk_0_dn10, var_phi_s0_bulk_0_dn11, var_phi_s0_bulk_0_dn12, var_phi_s0_bulk_0_dn17,)
    }
};
        var_phi_s0_bulk_0 = assign7320_e4927;
        var_phi_s0_bulk_0_dn0 = assign7320_e4927_d_n0;
        var_phi_s0_bulk_0_dn2 = assign7320_e4927_d_n2;
        var_phi_s0_bulk_0_dn6 = assign7320_e4927_d_n6;
        var_phi_s0_bulk_0_dn7 = assign7320_e4927_d_n7;
        var_phi_s0_bulk_0_dn10 = assign7320_e4927_d_n10;
        var_phi_s0_bulk_0_dn11 = assign7320_e4927_d_n11;
        var_phi_s0_bulk_0_dn12 = assign7320_e4927_d_n12;
        var_phi_s0_bulk_0_dn17 = assign7320_e4927_d_n17;

        let (assign7330_e4938, assign7330_e4938_d_n0, assign7330_e4938_d_n2, assign7330_e4938_d_n6, assign7330_e4938_d_n7, assign7330_e4938_d_n10, assign7330_e4938_d_n11, assign7330_e4938_d_n12, assign7330_e4938_d_n17,) = {
    if ((var_guard111 != 0.0) && (var_guard125 == 0.0)) {
        let assign7330_e4934: f64 = (var_psb_inib__blk124 - var_psb_inia__blk123);
        let assign7330_e4936: f64 = (assign7330_e4934 - 0.0008);
        (assign7330_e4936, (var_psb_inib__blk124_dn0 - var_psb_inia__blk123_dn0), (var_psb_inib__blk124_dn2 - var_psb_inia__blk123_dn2), (var_psb_inib__blk124_dn6 - var_psb_inia__blk123_dn6), (var_psb_inib__blk124_dn7 - var_psb_inia__blk123_dn7), (var_psb_inib__blk124_dn10 - var_psb_inia__blk123_dn10), (var_psb_inib__blk124_dn11 - var_psb_inia__blk123_dn11), (var_psb_inib__blk124_dn12 - var_psb_inia__blk123_dn12), (var_psb_inib__blk124_dn17 - var_psb_inia__blk123_dn17),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn12, var_tmf1_dn17,)
    }
};
        var_tmf1 = assign7330_e4938;
        var_tmf1_dn0 = assign7330_e4938_d_n0;
        var_tmf1_dn2 = assign7330_e4938_d_n2;
        var_tmf1_dn6 = assign7330_e4938_d_n6;
        var_tmf1_dn7 = assign7330_e4938_d_n7;
        var_tmf1_dn10 = assign7330_e4938_d_n10;
        var_tmf1_dn11 = assign7330_e4938_d_n11;
        var_tmf1_dn12 = assign7330_e4938_d_n12;
        var_tmf1_dn17 = assign7330_e4938_d_n17;

        let (assign7340_e4949, assign7340_e4949_d_n0, assign7340_e4949_d_n2, assign7340_e4949_d_n6, assign7340_e4949_d_n7, assign7340_e4949_d_n10, assign7340_e4949_d_n11, assign7340_e4949_d_n12, assign7340_e4949_d_n17,) = {
    if ((var_guard111 != 0.0) && (var_guard125 == 0.0)) {
        let assign7340_e4945: f64 = (4.0 * var_psb_inib__blk124);
        let assign7340_e4947: f64 = (assign7340_e4945 * 0.0008);
        (assign7340_e4947, ((4.0 * var_psb_inib__blk124_dn0) * 0.0008), ((4.0 * var_psb_inib__blk124_dn2) * 0.0008), ((4.0 * var_psb_inib__blk124_dn6) * 0.0008), ((4.0 * var_psb_inib__blk124_dn7) * 0.0008), ((4.0 * var_psb_inib__blk124_dn10) * 0.0008), ((4.0 * var_psb_inib__blk124_dn11) * 0.0008), ((4.0 * var_psb_inib__blk124_dn12) * 0.0008), ((4.0 * var_psb_inib__blk124_dn17) * 0.0008),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign7340_e4949;
        var_tmf2_dn0 = assign7340_e4949_d_n0;
        var_tmf2_dn2 = assign7340_e4949_d_n2;
        var_tmf2_dn6 = assign7340_e4949_d_n6;
        var_tmf2_dn7 = assign7340_e4949_d_n7;
        var_tmf2_dn10 = assign7340_e4949_d_n10;
        var_tmf2_dn11 = assign7340_e4949_d_n11;
        var_tmf2_dn12 = assign7340_e4949_d_n12;
        var_tmf2_dn17 = assign7340_e4949_d_n17;

        let (assign7350_e4962, assign7350_e4962_d_n0, assign7350_e4962_d_n2, assign7350_e4962_d_n6, assign7350_e4962_d_n7, assign7350_e4962_d_n10, assign7350_e4962_d_n11, assign7350_e4962_d_n12, assign7350_e4962_d_n17,) = {
    if ((var_guard111 != 0.0) && (var_guard125 == 0.0)) {
        let (assign7350_e4960, assign7350_e4960_d_n0, assign7350_e4960_d_n2, assign7350_e4960_d_n6, assign7350_e4960_d_n7, assign7350_e4960_d_n10, assign7350_e4960_d_n11, assign7350_e4960_d_n12, assign7350_e4960_d_n17,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
            } else {
                let assign7350_e4959: f64 = (-var_tmf2);
                (assign7350_e4959, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn12), (-var_tmf2_dn17),)
            }
        };
        (assign7350_e4960, assign7350_e4960_d_n0, assign7350_e4960_d_n2, assign7350_e4960_d_n6, assign7350_e4960_d_n7, assign7350_e4960_d_n10, assign7350_e4960_d_n11, assign7350_e4960_d_n12, assign7350_e4960_d_n17,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign7350_e4962;
        var_tmf2_dn0 = assign7350_e4962_d_n0;
        var_tmf2_dn2 = assign7350_e4962_d_n2;
        var_tmf2_dn6 = assign7350_e4962_d_n6;
        var_tmf2_dn7 = assign7350_e4962_d_n7;
        var_tmf2_dn10 = assign7350_e4962_d_n10;
        var_tmf2_dn11 = assign7350_e4962_d_n11;
        var_tmf2_dn12 = assign7350_e4962_d_n12;
        var_tmf2_dn17 = assign7350_e4962_d_n17;

        let (assign7360_e4974, assign7360_e4974_d_n0, assign7360_e4974_d_n2, assign7360_e4974_d_n6, assign7360_e4974_d_n7, assign7360_e4974_d_n10, assign7360_e4974_d_n11, assign7360_e4974_d_n12, assign7360_e4974_d_n17,) = {
    if ((var_guard111 != 0.0) && (var_guard125 == 0.0)) {
        let assign7360_e4969: f64 = (var_tmf1 * var_tmf1);
        let assign7360_e4971: f64 = (assign7360_e4969 + var_tmf2);
        let assign7360_e4972: f64 = (assign7360_e4971).sqrt();
        (assign7360_e4972, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign7360_e4972)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign7360_e4972)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign7360_e4972)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign7360_e4972)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign7360_e4972)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign7360_e4972)), ((((var_tmf1_dn12 * var_tmf1) + (var_tmf1 * var_tmf1_dn12)) + var_tmf2_dn12) / (2.0 * assign7360_e4972)), ((((var_tmf1_dn17 * var_tmf1) + (var_tmf1 * var_tmf1_dn17)) + var_tmf2_dn17) / (2.0 * assign7360_e4972)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn12, var_tmf2_dn17,)
    }
};
        var_tmf2 = assign7360_e4974;
        var_tmf2_dn0 = assign7360_e4974_d_n0;
        var_tmf2_dn2 = assign7360_e4974_d_n2;
        var_tmf2_dn6 = assign7360_e4974_d_n6;
        var_tmf2_dn7 = assign7360_e4974_d_n7;
        var_tmf2_dn10 = assign7360_e4974_d_n10;
        var_tmf2_dn11 = assign7360_e4974_d_n11;
        var_tmf2_dn12 = assign7360_e4974_d_n12;
        var_tmf2_dn17 = assign7360_e4974_d_n17;

        let (assign7370_e4987, assign7370_e4987_d_n0, assign7370_e4987_d_n2, assign7370_e4987_d_n6, assign7370_e4987_d_n7, assign7370_e4987_d_n10, assign7370_e4987_d_n11, assign7370_e4987_d_n12, assign7370_e4987_d_n17,) = {
    if ((var_guard111 != 0.0) && (var_guard125 == 0.0)) {
        let assign7370_e4983: f64 = (var_tmf1 + var_tmf2);
        let assign7370_e4984: f64 = (0.5 * assign7370_e4983);
        let assign7370_e4985: f64 = (var_psb_inib__blk124 - assign7370_e4984);
        (assign7370_e4985, (var_psb_inib__blk124_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_psb_inib__blk124_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_psb_inib__blk124_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_psb_inib__blk124_dn7 - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (var_psb_inib__blk124_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_psb_inib__blk124_dn11 - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (var_psb_inib__blk124_dn12 - (0.5 * (var_tmf1_dn12 + var_tmf2_dn12))), (var_psb_inib__blk124_dn17 - (0.5 * (var_tmf1_dn17 + var_tmf2_dn17))),)
    } else {
        (var_phi_s0_bulk_0, var_phi_s0_bulk_0_dn0, var_phi_s0_bulk_0_dn2, var_phi_s0_bulk_0_dn6, var_phi_s0_bulk_0_dn7, var_phi_s0_bulk_0_dn10, var_phi_s0_bulk_0_dn11, var_phi_s0_bulk_0_dn12, var_phi_s0_bulk_0_dn17,)
    }
};
        var_phi_s0_bulk_0 = assign7370_e4987;
        var_phi_s0_bulk_0_dn0 = assign7370_e4987_d_n0;
        var_phi_s0_bulk_0_dn2 = assign7370_e4987_d_n2;
        var_phi_s0_bulk_0_dn6 = assign7370_e4987_d_n6;
        var_phi_s0_bulk_0_dn7 = assign7370_e4987_d_n7;
        var_phi_s0_bulk_0_dn10 = assign7370_e4987_d_n10;
        var_phi_s0_bulk_0_dn11 = assign7370_e4987_d_n11;
        var_phi_s0_bulk_0_dn12 = assign7370_e4987_d_n12;
        var_phi_s0_bulk_0_dn17 = assign7370_e4987_d_n17;

        let (assign7380_e4991,) = {
    if (var_guard111 != 0.0) {
        (0.0,)
    } else {
        (var_lp_s0,)
    }
};
        var_lp_s0 = assign7380_e4991;

        *var_c_soi__blk112_slot = var_c_soi__blk112;
        *var_c_soi_inv__blk113_slot = var_c_soi_inv__blk113;
        *var_guard125_slot = var_guard125;
        *var_lp_s0_slot = var_lp_s0;
        *var_pb2_bulk_slot = var_pb2_bulk;
        *var_phi_s0_bulk_0_slot = var_phi_s0_bulk_0;
        *var_phi_s0_bulk_0_dn0_slot = var_phi_s0_bulk_0_dn0;
        *var_phi_s0_bulk_0_dn10_slot = var_phi_s0_bulk_0_dn10;
        *var_phi_s0_bulk_0_dn11_slot = var_phi_s0_bulk_0_dn11;
        *var_phi_s0_bulk_0_dn12_slot = var_phi_s0_bulk_0_dn12;
        *var_phi_s0_bulk_0_dn17_slot = var_phi_s0_bulk_0_dn17;
        *var_phi_s0_bulk_0_dn2_slot = var_phi_s0_bulk_0_dn2;
        *var_phi_s0_bulk_0_dn6_slot = var_phi_s0_bulk_0_dn6;
        *var_phi_s0_bulk_0_dn7_slot = var_phi_s0_bulk_0_dn7;
        *var_psb_inia__blk123_slot = var_psb_inia__blk123;
        *var_psb_inia__blk123_dn0_slot = var_psb_inia__blk123_dn0;
        *var_psb_inia__blk123_dn10_slot = var_psb_inia__blk123_dn10;
        *var_psb_inia__blk123_dn11_slot = var_psb_inia__blk123_dn11;
        *var_psb_inia__blk123_dn12_slot = var_psb_inia__blk123_dn12;
        *var_psb_inia__blk123_dn17_slot = var_psb_inia__blk123_dn17;
        *var_psb_inia__blk123_dn2_slot = var_psb_inia__blk123_dn2;
        *var_psb_inia__blk123_dn6_slot = var_psb_inia__blk123_dn6;
        *var_psb_inia__blk123_dn7_slot = var_psb_inia__blk123_dn7;
        *var_psb_inib__blk124_slot = var_psb_inib__blk124;
        *var_psb_inib__blk124_dn0_slot = var_psb_inib__blk124_dn0;
        *var_psb_inib__blk124_dn10_slot = var_psb_inib__blk124_dn10;
        *var_psb_inib__blk124_dn11_slot = var_psb_inib__blk124_dn11;
        *var_psb_inib__blk124_dn12_slot = var_psb_inib__blk124_dn12;
        *var_psb_inib__blk124_dn17_slot = var_psb_inib__blk124_dn17;
        *var_psb_inib__blk124_dn2_slot = var_psb_inib__blk124_dn2;
        *var_psb_inib__blk124_dn6_slot = var_psb_inib__blk124_dn6;
        *var_psb_inib__blk124_dn7_slot = var_psb_inib__blk124_dn7;
        *var_q_fd_dlt1_slot = var_q_fd_dlt1;
        *var_q_fd_dlt1_dn0_slot = var_q_fd_dlt1_dn0;
        *var_q_fd_dlt1_dn10_slot = var_q_fd_dlt1_dn10;
        *var_q_fd_dlt1_dn11_slot = var_q_fd_dlt1_dn11;
        *var_q_fd_dlt1_dn12_slot = var_q_fd_dlt1_dn12;
        *var_q_fd_dlt1_dn17_slot = var_q_fd_dlt1_dn17;
        *var_q_fd_dlt1_dn2_slot = var_q_fd_dlt1_dn2;
        *var_q_fd_dlt1_dn6_slot = var_q_fd_dlt1_dn6;
        *var_q_fd_dlt1_dn7_slot = var_q_fd_dlt1_dn7;
        *var_q_fd_dlt2_slot = var_q_fd_dlt2;
        *var_q_fd_dlt2_dn0_slot = var_q_fd_dlt2_dn0;
        *var_q_fd_dlt2_dn10_slot = var_q_fd_dlt2_dn10;
        *var_q_fd_dlt2_dn11_slot = var_q_fd_dlt2_dn11;
        *var_q_fd_dlt2_dn12_slot = var_q_fd_dlt2_dn12;
        *var_q_fd_dlt2_dn17_slot = var_q_fd_dlt2_dn17;
        *var_q_fd_dlt2_dn2_slot = var_q_fd_dlt2_dn2;
        *var_q_fd_dlt2_dn6_slot = var_q_fd_dlt2_dn6;
        *var_q_fd_dlt2_dn7_slot = var_q_fd_dlt2_dn7;
        *var_q_fd_soi_slot = var_q_fd_soi;
        *var_q_fd_soi_dn0_slot = var_q_fd_soi_dn0;
        *var_q_fd_soi_dn10_slot = var_q_fd_soi_dn10;
        *var_q_fd_soi_dn11_slot = var_q_fd_soi_dn11;
        *var_q_fd_soi_dn12_slot = var_q_fd_soi_dn12;
        *var_q_fd_soi_dn17_slot = var_q_fd_soi_dn17;
        *var_q_fd_soi_dn2_slot = var_q_fd_soi_dn2;
        *var_q_fd_soi_dn6_slot = var_q_fd_soi_dn6;
        *var_q_fd_soi_dn7_slot = var_q_fd_soi_dn7;
        *var_q_wdsoi_max_slot = var_q_wdsoi_max;
        *var_q_wdsoi_max_dn0_slot = var_q_wdsoi_max_dn0;
        *var_q_wdsoi_max_dn10_slot = var_q_wdsoi_max_dn10;
        *var_q_wdsoi_max_dn11_slot = var_q_wdsoi_max_dn11;
        *var_q_wdsoi_max_dn12_slot = var_q_wdsoi_max_dn12;
        *var_q_wdsoi_max_dn17_slot = var_q_wdsoi_max_dn17;
        *var_q_wdsoi_max_dn2_slot = var_q_wdsoi_max_dn2;
        *var_q_wdsoi_max_dn6_slot = var_q_wdsoi_max_dn6;
        *var_q_wdsoi_max_dn7_slot = var_q_wdsoi_max_dn7;
        *var_t0__blk119_slot = var_t0__blk119;
        *var_t0__blk119_dn10_slot = var_t0__blk119_dn10;
        *var_t1__blk120_slot = var_t1__blk120;
        *var_t1__blk120_dn0_slot = var_t1__blk120_dn0;
        *var_t1__blk120_dn10_slot = var_t1__blk120_dn10;
        *var_t1__blk120_dn11_slot = var_t1__blk120_dn11;
        *var_t1__blk120_dn12_slot = var_t1__blk120_dn12;
        *var_t1__blk120_dn17_slot = var_t1__blk120_dn17;
        *var_t1__blk120_dn2_slot = var_t1__blk120_dn2;
        *var_t1__blk120_dn6_slot = var_t1__blk120_dn6;
        *var_t1__blk120_dn7_slot = var_t1__blk120_dn7;
        *var_t2__blk121_slot = var_t2__blk121;
        *var_t2__blk121_dn0_slot = var_t2__blk121_dn0;
        *var_t2__blk121_dn10_slot = var_t2__blk121_dn10;
        *var_t2__blk121_dn11_slot = var_t2__blk121_dn11;
        *var_t2__blk121_dn12_slot = var_t2__blk121_dn12;
        *var_t2__blk121_dn17_slot = var_t2__blk121_dn17;
        *var_t2__blk121_dn2_slot = var_t2__blk121_dn2;
        *var_t2__blk121_dn6_slot = var_t2__blk121_dn6;
        *var_t2__blk121_dn7_slot = var_t2__blk121_dn7;
        *var_t3__blk122_slot = var_t3__blk122;
        *var_t3__blk122_dn0_slot = var_t3__blk122_dn0;
        *var_t3__blk122_dn10_slot = var_t3__blk122_dn10;
        *var_t3__blk122_dn11_slot = var_t3__blk122_dn11;
        *var_t3__blk122_dn12_slot = var_t3__blk122_dn12;
        *var_t3__blk122_dn17_slot = var_t3__blk122_dn17;
        *var_t3__blk122_dn2_slot = var_t3__blk122_dn2;
        *var_t3__blk122_dn6_slot = var_t3__blk122_dn6;
        *var_t3__blk122_dn7_slot = var_t3__blk122_dn7;
        *var_t_soi_slot = var_t_soi;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn12_slot = var_tmf1_dn12;
        *var_tmf1_dn17_slot = var_tmf1_dn17;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn12_slot = var_tmf2_dn12;
        *var_tmf2_dn17_slot = var_tmf2_dn17;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_vbsbiz_slot = var_vbsbiz;
        *var_vbsbiz_dn0_slot = var_vbsbiz_dn0;
        *var_vbsbiz_dn10_slot = var_vbsbiz_dn10;
        *var_vbsbiz_dn11_slot = var_vbsbiz_dn11;
        *var_vbsbiz_dn12_slot = var_vbsbiz_dn12;
        *var_vbsbiz_dn17_slot = var_vbsbiz_dn17;
        *var_vbsbiz_dn2_slot = var_vbsbiz_dn2;
        *var_vbsbiz_dn6_slot = var_vbsbiz_dn6;
        *var_vbsbiz_dn7_slot = var_vbsbiz_dn7;
        *var_wdsoi_ini1_dlt_slot = var_wdsoi_ini1_dlt;
    }
}
