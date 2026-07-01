#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_idd = 0.0;
        locals.var_idd_dn0 = 0.0;
        locals.var_idd_dn2 = 0.0;
        locals.var_idd_dn6 = 0.0;
        locals.var_idd_dn7 = 0.0;
        locals.var_idd_dn10 = 0.0;
        locals.var_idd_dn11 = 0.0;
        locals.var_idd_dn12 = 0.0;
        locals.var_idd_dn17 = 0.0;

        locals.var_gds0_ign = 1e-12;
        locals.var_gds0_ign_dn0 = 0.0;
        locals.var_gds0_ign_dn2 = 0.0;
        locals.var_gds0_ign_dn6 = 0.0;
        locals.var_gds0_ign_dn7 = 0.0;
        locals.var_gds0_ign_dn10 = 0.0;
        locals.var_gds0_ign_dn11 = 0.0;
        locals.var_gds0_ign_dn12 = 0.0;
        locals.var_gds0_ign_dn17 = 0.0;

        locals.var_qse = 0.0;
        locals.var_qse_dn0 = 0.0;
        locals.var_qse_dn2 = 0.0;
        locals.var_qse_dn6 = 0.0;
        locals.var_qse_dn7 = 0.0;
        locals.var_qse_dn10 = 0.0;
        locals.var_qse_dn11 = 0.0;
        locals.var_qse_dn12 = 0.0;
        locals.var_qse_dn13 = 0.0;
        locals.var_qse_dn15 = 0.0;
        locals.var_qse_dn16 = 0.0;
        locals.var_qse_dn17 = 0.0;
        locals.var_qse_dn18 = 0.0;

        locals.var_flg_ign = 0.0;

        locals.var_end_of_part_1 = 0.0;

        locals.var_xd = 0.0;
        locals.var_xd_dn0 = 0.0;
        locals.var_xd_dn2 = 0.0;
        locals.var_xd_dn6 = 0.0;
        locals.var_xd_dn7 = 0.0;
        locals.var_xd_dn10 = 0.0;
        locals.var_xd_dn11 = 0.0;
        locals.var_xd_dn12 = 0.0;
        locals.var_xd_dn17 = 0.0;

        locals.var_flg_noqi = 0.0;

        locals.var_flg_zone = 0.0;

        locals.var_psl = 0.0;
        locals.var_psl_dn0 = 0.0;
        locals.var_psl_dn2 = 0.0;
        locals.var_psl_dn6 = 0.0;
        locals.var_psl_dn7 = 0.0;
        locals.var_psl_dn10 = 0.0;
        locals.var_psl_dn11 = 0.0;
        locals.var_psl_dn12 = 0.0;
        locals.var_psl_dn17 = 0.0;

        locals.var_psl_lim = 0.0;
        locals.var_psl_lim_dn0 = 0.0;
        locals.var_psl_lim_dn2 = 0.0;
        locals.var_psl_lim_dn6 = 0.0;
        locals.var_psl_lim_dn7 = 0.0;
        locals.var_psl_lim_dn10 = 0.0;
        locals.var_psl_lim_dn11 = 0.0;
        locals.var_psl_lim_dn12 = 0.0;
        locals.var_psl_lim_dn17 = 0.0;

        locals.var_pds = 0.0;
        locals.var_pds_dn0 = 0.0;
        locals.var_pds_dn2 = 0.0;
        locals.var_pds_dn6 = 0.0;
        locals.var_pds_dn7 = 0.0;
        locals.var_pds_dn10 = 0.0;
        locals.var_pds_dn11 = 0.0;
        locals.var_pds_dn12 = 0.0;
        locals.var_pds_dn17 = 0.0;

        locals.var_pds_ini = 0.0;
        locals.var_pds_ini_dn0 = 0.0;
        locals.var_pds_ini_dn2 = 0.0;
        locals.var_pds_ini_dn6 = 0.0;
        locals.var_pds_ini_dn7 = 0.0;
        locals.var_pds_ini_dn10 = 0.0;
        locals.var_pds_ini_dn11 = 0.0;
        locals.var_pds_ini_dn12 = 0.0;
        locals.var_pds_ini_dn17 = 0.0;

        locals.var_ps0z = 1.0;
        locals.var_ps0z_dn0 = 0.0;
        locals.var_ps0z_dn2 = 0.0;
        locals.var_ps0z_dn6 = 0.0;
        locals.var_ps0z_dn7 = 0.0;
        locals.var_ps0z_dn10 = 0.0;
        locals.var_ps0z_dn11 = 0.0;
        locals.var_ps0z_dn12 = 0.0;
        locals.var_ps0z_dn17 = 0.0;

        locals.var_alpha = 0.0;
        locals.var_alpha_dn0 = 0.0;
        locals.var_alpha_dn2 = 0.0;
        locals.var_alpha_dn6 = 0.0;
        locals.var_alpha_dn7 = 0.0;
        locals.var_alpha_dn10 = 0.0;
        locals.var_alpha_dn11 = 0.0;
        locals.var_alpha_dn12 = 0.0;
        locals.var_alpha_dn17 = 0.0;

        locals.var_vgvt = 0.0;
        locals.var_vgvt_dn0 = 0.0;
        locals.var_vgvt_dn2 = 0.0;
        locals.var_vgvt_dn6 = 0.0;
        locals.var_vgvt_dn7 = 0.0;
        locals.var_vgvt_dn10 = 0.0;
        locals.var_vgvt_dn11 = 0.0;
        locals.var_vgvt_dn12 = 0.0;
        locals.var_vgvt_dn17 = 0.0;

        locals.var_qb = 0.0;
        locals.var_qb_dn0 = 0.0;
        locals.var_qb_dn2 = 0.0;
        locals.var_qb_dn6 = 0.0;
        locals.var_qb_dn7 = 0.0;
        locals.var_qb_dn10 = 0.0;
        locals.var_qb_dn11 = 0.0;
        locals.var_qb_dn12 = 0.0;
        locals.var_qb_dn13 = 0.0;
        locals.var_qb_dn15 = 0.0;
        locals.var_qb_dn16 = 0.0;
        locals.var_qb_dn17 = 0.0;
        locals.var_qb_dn18 = 0.0;

        locals.var_qi = 0.0;
        locals.var_qi_dn0 = 0.0;
        locals.var_qi_dn2 = 0.0;
        locals.var_qi_dn6 = 0.0;
        locals.var_qi_dn7 = 0.0;
        locals.var_qi_dn10 = 0.0;
        locals.var_qi_dn11 = 0.0;
        locals.var_qi_dn12 = 0.0;
        locals.var_qi_dn17 = 0.0;

        locals.var_qd = 0.0;
        locals.var_qd_dn0 = 0.0;
        locals.var_qd_dn2 = 0.0;
        locals.var_qd_dn6 = 0.0;
        locals.var_qd_dn7 = 0.0;
        locals.var_qd_dn10 = 0.0;
        locals.var_qd_dn11 = 0.0;
        locals.var_qd_dn12 = 0.0;
        locals.var_qd_dn13 = 0.0;
        locals.var_qd_dn15 = 0.0;
        locals.var_qd_dn16 = 0.0;
        locals.var_qd_dn17 = 0.0;
        locals.var_qd_dn18 = 0.0;

        locals.var_ids = 0.0;
        locals.var_ids_dn0 = 0.0;
        locals.var_ids_dn2 = 0.0;
        locals.var_ids_dn6 = 0.0;
        locals.var_ids_dn7 = 0.0;
        locals.var_ids_dn10 = 0.0;
        locals.var_ids_dn11 = 0.0;
        locals.var_ids_dn12 = 0.0;
        locals.var_ids_dn17 = 0.0;

        locals.var_fb = 0.0;
        locals.var_fb_dn0 = 0.0;
        locals.var_fb_dn2 = 0.0;
        locals.var_fb_dn6 = 0.0;
        locals.var_fb_dn7 = 0.0;
        locals.var_fb_dn10 = 0.0;
        locals.var_fb_dn11 = 0.0;
        locals.var_fb_dn12 = 0.0;
        locals.var_fb_dn17 = 0.0;

        locals.var_qn0 = 0.0;
        locals.var_qn0_dn0 = 0.0;
        locals.var_qn0_dn2 = 0.0;
        locals.var_qn0_dn6 = 0.0;
        locals.var_qn0_dn7 = 0.0;
        locals.var_qn0_dn10 = 0.0;
        locals.var_qn0_dn11 = 0.0;
        locals.var_qn0_dn12 = 0.0;
        locals.var_qn0_dn17 = 0.0;

        locals.var_mu = 0.0;
        locals.var_mu_dn0 = 0.0;
        locals.var_mu_dn2 = 0.0;
        locals.var_mu_dn6 = 0.0;
        locals.var_mu_dn7 = 0.0;
        locals.var_mu_dn10 = 0.0;
        locals.var_mu_dn11 = 0.0;
        locals.var_mu_dn12 = 0.0;
        locals.var_mu_dn17 = 0.0;

        locals.var_muun = 0.0;
        locals.var_muun_dn0 = 0.0;
        locals.var_muun_dn2 = 0.0;
        locals.var_muun_dn6 = 0.0;
        locals.var_muun_dn7 = 0.0;
        locals.var_muun_dn10 = 0.0;
        locals.var_muun_dn11 = 0.0;
        locals.var_muun_dn12 = 0.0;
        locals.var_muun_dn17 = 0.0;

        locals.var_ey = 0.0;
        locals.var_ey_dn0 = 0.0;
        locals.var_ey_dn2 = 0.0;
        locals.var_ey_dn6 = 0.0;
        locals.var_ey_dn7 = 0.0;
        locals.var_ey_dn10 = 0.0;
        locals.var_ey_dn11 = 0.0;
        locals.var_ey_dn12 = 0.0;
        locals.var_ey_dn17 = 0.0;

        locals.var_isub = 0.0;
        locals.var_isub_dn0 = 0.0;
        locals.var_isub_dn2 = 0.0;
        locals.var_isub_dn6 = 0.0;
        locals.var_isub_dn7 = 0.0;
        locals.var_isub_dn10 = 0.0;
        locals.var_isub_dn11 = 0.0;
        locals.var_isub_dn12 = 0.0;
        locals.var_isub_dn17 = 0.0;

        locals.var_betawl = 1.0;
        locals.var_betawl_dn0 = 0.0;
        locals.var_betawl_dn2 = 0.0;
        locals.var_betawl_dn6 = 0.0;
        locals.var_betawl_dn7 = 0.0;
        locals.var_betawl_dn10 = 0.0;
        locals.var_betawl_dn11 = 0.0;
        locals.var_betawl_dn12 = 0.0;
        locals.var_betawl_dn17 = 0.0;

        locals.var_idsibpc = 0.0;
        locals.var_idsibpc_dn0 = 0.0;
        locals.var_idsibpc_dn2 = 0.0;
        locals.var_idsibpc_dn6 = 0.0;
        locals.var_idsibpc_dn7 = 0.0;
        locals.var_idsibpc_dn10 = 0.0;
        locals.var_idsibpc_dn11 = 0.0;
        locals.var_idsibpc_dn12 = 0.0;
        locals.var_idsibpc_dn17 = 0.0;

        locals.var_qgos = 0.0;
        locals.var_qgos_dn0 = 0.0;
        locals.var_qgos_dn2 = 0.0;
        locals.var_qgos_dn6 = 0.0;
        locals.var_qgos_dn7 = 0.0;
        locals.var_qgos_dn10 = 0.0;
        locals.var_qgos_dn11 = 0.0;
        locals.var_qgos_dn12 = 0.0;
        locals.var_qgos_dn17 = 0.0;

        locals.var_qgod = 0.0;
        locals.var_qgod_dn0 = 0.0;
        locals.var_qgod_dn2 = 0.0;
        locals.var_qgod_dn6 = 0.0;
        locals.var_qgod_dn7 = 0.0;
        locals.var_qgod_dn10 = 0.0;
        locals.var_qgod_dn11 = 0.0;
        locals.var_qgod_dn12 = 0.0;
        locals.var_qgod_dn17 = 0.0;

        locals.var_qgob = 0.0;
        locals.var_qgob_dn0 = 0.0;
        locals.var_qgob_dn2 = 0.0;
        locals.var_qgob_dn6 = 0.0;
        locals.var_qgob_dn7 = 0.0;
        locals.var_qgob_dn10 = 0.0;
        locals.var_qgob_dn11 = 0.0;
        locals.var_qgob_dn12 = 0.0;
        locals.var_qgob_dn17 = 0.0;

        locals.var_qovd = 0.0;
        locals.var_qovd_dn0 = 0.0;
        locals.var_qovd_dn2 = 0.0;
        locals.var_qovd_dn6 = 0.0;
        locals.var_qovd_dn7 = 0.0;
        locals.var_qovd_dn10 = 0.0;
        locals.var_qovd_dn11 = 0.0;
        locals.var_qovd_dn12 = 0.0;
        locals.var_qovd_dn17 = 0.0;

        locals.var_qovs = 0.0;
        locals.var_qovs_dn0 = 0.0;
        locals.var_qovs_dn2 = 0.0;
        locals.var_qovs_dn6 = 0.0;
        locals.var_qovs_dn7 = 0.0;
        locals.var_qovs_dn10 = 0.0;
        locals.var_qovs_dn11 = 0.0;
        locals.var_qovs_dn12 = 0.0;
        locals.var_qovs_dn17 = 0.0;

        locals.var_qbdld = 0.0;
        locals.var_qbdld_dn0 = 0.0;
        locals.var_qbdld_dn2 = 0.0;
        locals.var_qbdld_dn6 = 0.0;
        locals.var_qbdld_dn7 = 0.0;
        locals.var_qbdld_dn10 = 0.0;
        locals.var_qbdld_dn11 = 0.0;
        locals.var_qbdld_dn12 = 0.0;
        locals.var_qbdld_dn17 = 0.0;

        locals.var_qbsld = 0.0;
        locals.var_qbsld_dn0 = 0.0;
        locals.var_qbsld_dn2 = 0.0;
        locals.var_qbsld_dn6 = 0.0;
        locals.var_qbsld_dn7 = 0.0;
        locals.var_qbsld_dn10 = 0.0;
        locals.var_qbsld_dn11 = 0.0;
        locals.var_qbsld_dn12 = 0.0;
        locals.var_qbsld_dn17 = 0.0;

        locals.var_ibd = 0.0;
        locals.var_ibd_dn0 = 0.0;
        locals.var_ibd_dn2 = 0.0;
        locals.var_ibd_dn6 = 0.0;
        locals.var_ibd_dn7 = 0.0;
        locals.var_ibd_dn10 = 0.0;
        locals.var_ibd_dn11 = 0.0;
        locals.var_ibd_dn12 = 0.0;
        locals.var_ibd_dn17 = 0.0;

        locals.var_ibs = 0.0;
        locals.var_ibs_dn0 = 0.0;
        locals.var_ibs_dn2 = 0.0;
        locals.var_ibs_dn6 = 0.0;
        locals.var_ibs_dn7 = 0.0;
        locals.var_ibs_dn10 = 0.0;
        locals.var_ibs_dn11 = 0.0;
        locals.var_ibs_dn12 = 0.0;
        locals.var_ibs_dn17 = 0.0;

        locals.var_qbd = 0.0;
        locals.var_qbd_dn0 = 0.0;
        locals.var_qbd_dn2 = 0.0;
        locals.var_qbd_dn6 = 0.0;
        locals.var_qbd_dn7 = 0.0;
        locals.var_qbd_dn10 = 0.0;
        locals.var_qbd_dn11 = 0.0;
        locals.var_qbd_dn12 = 0.0;
        locals.var_qbd_dn17 = 0.0;

        locals.var_qbs = 0.0;
        locals.var_qbs_dn0 = 0.0;
        locals.var_qbs_dn2 = 0.0;
        locals.var_qbs_dn6 = 0.0;
        locals.var_qbs_dn7 = 0.0;
        locals.var_qbs_dn10 = 0.0;
        locals.var_qbs_dn11 = 0.0;
        locals.var_qbs_dn12 = 0.0;
        locals.var_qbs_dn17 = 0.0;

        locals.var_qinm = 0.0;
        locals.var_qinm_dn0 = 0.0;
        locals.var_qinm_dn2 = 0.0;
        locals.var_qinm_dn6 = 0.0;
        locals.var_qinm_dn7 = 0.0;
        locals.var_qinm_dn10 = 0.0;
        locals.var_qinm_dn11 = 0.0;
        locals.var_qinm_dn12 = 0.0;
        locals.var_qinm_dn17 = 0.0;

        locals.var_qidn = 0.0;
        locals.var_qidn_dn0 = 0.0;
        locals.var_qidn_dn2 = 0.0;
        locals.var_qidn_dn6 = 0.0;
        locals.var_qidn_dn7 = 0.0;
        locals.var_qidn_dn10 = 0.0;
        locals.var_qidn_dn11 = 0.0;
        locals.var_qidn_dn12 = 0.0;
        locals.var_qidn_dn17 = 0.0;

        locals.var_wdsoi_0 = p.p237;

        locals.var_qbody_bt_p_sus = 0.0;
        locals.var_qbody_bt_p_sus_dn0 = 0.0;
        locals.var_qbody_bt_p_sus_dn2 = 0.0;
        locals.var_qbody_bt_p_sus_dn6 = 0.0;
        locals.var_qbody_bt_p_sus_dn7 = 0.0;
        locals.var_qbody_bt_p_sus_dn10 = 0.0;
        locals.var_qbody_bt_p_sus_dn11 = 0.0;
        locals.var_qbody_bt_p_sus_dn12 = 0.0;
        locals.var_qbody_bt_p_sus_dn17 = 0.0;

        locals.var_qbody_bt_p_sud = 0.0;
        locals.var_qbody_bt_p_sud_dn0 = 0.0;
        locals.var_qbody_bt_p_sud_dn2 = 0.0;
        locals.var_qbody_bt_p_sud_dn6 = 0.0;
        locals.var_qbody_bt_p_sud_dn7 = 0.0;
        locals.var_qbody_bt_p_sud_dn10 = 0.0;
        locals.var_qbody_bt_p_sud_dn11 = 0.0;
        locals.var_qbody_bt_p_sud_dn12 = 0.0;
        locals.var_qbody_bt_p_sud_dn17 = 0.0;

        locals.var_qbody_bt_p_iud = 0.0;
        locals.var_qbody_bt_p_iud_dn0 = 0.0;
        locals.var_qbody_bt_p_iud_dn2 = 0.0;
        locals.var_qbody_bt_p_iud_dn6 = 0.0;
        locals.var_qbody_bt_p_iud_dn7 = 0.0;
        locals.var_qbody_bt_p_iud_dn10 = 0.0;
        locals.var_qbody_bt_p_iud_dn11 = 0.0;
        locals.var_qbody_bt_p_iud_dn12 = 0.0;
        locals.var_qbody_bt_p_iud_dn17 = 0.0;

        locals.var_qbody_bt_p_ius = 0.0;
        locals.var_qbody_bt_p_ius_dn0 = 0.0;
        locals.var_qbody_bt_p_ius_dn2 = 0.0;
        locals.var_qbody_bt_p_ius_dn6 = 0.0;
        locals.var_qbody_bt_p_ius_dn7 = 0.0;
        locals.var_qbody_bt_p_ius_dn10 = 0.0;
        locals.var_qbody_bt_p_ius_dn11 = 0.0;
        locals.var_qbody_bt_p_ius_dn12 = 0.0;
        locals.var_qbody_bt_p_ius_dn17 = 0.0;

        locals.var_qbody_bt_n_sus = 0.0;
        locals.var_qbody_bt_n_sus_dn0 = 0.0;
        locals.var_qbody_bt_n_sus_dn2 = 0.0;
        locals.var_qbody_bt_n_sus_dn6 = 0.0;
        locals.var_qbody_bt_n_sus_dn7 = 0.0;
        locals.var_qbody_bt_n_sus_dn10 = 0.0;
        locals.var_qbody_bt_n_sus_dn11 = 0.0;
        locals.var_qbody_bt_n_sus_dn12 = 0.0;
        locals.var_qbody_bt_n_sus_dn17 = 0.0;

        locals.var_qbody_bt_n_sud = 0.0;
        locals.var_qbody_bt_n_sud_dn0 = 0.0;
        locals.var_qbody_bt_n_sud_dn2 = 0.0;
        locals.var_qbody_bt_n_sud_dn6 = 0.0;
        locals.var_qbody_bt_n_sud_dn7 = 0.0;
        locals.var_qbody_bt_n_sud_dn10 = 0.0;
        locals.var_qbody_bt_n_sud_dn11 = 0.0;
        locals.var_qbody_bt_n_sud_dn12 = 0.0;
        locals.var_qbody_bt_n_sud_dn17 = 0.0;

        locals.var_qbody_bt_n_iud = 0.0;
        locals.var_qbody_bt_n_iud_dn0 = 0.0;
        locals.var_qbody_bt_n_iud_dn2 = 0.0;
        locals.var_qbody_bt_n_iud_dn6 = 0.0;
        locals.var_qbody_bt_n_iud_dn7 = 0.0;
        locals.var_qbody_bt_n_iud_dn10 = 0.0;
        locals.var_qbody_bt_n_iud_dn11 = 0.0;
        locals.var_qbody_bt_n_iud_dn12 = 0.0;
        locals.var_qbody_bt_n_iud_dn17 = 0.0;

        locals.var_qbody_bt_n_ius = 0.0;
        locals.var_qbody_bt_n_ius_dn0 = 0.0;
        locals.var_qbody_bt_n_ius_dn2 = 0.0;
        locals.var_qbody_bt_n_ius_dn6 = 0.0;
        locals.var_qbody_bt_n_ius_dn7 = 0.0;
        locals.var_qbody_bt_n_ius_dn10 = 0.0;
        locals.var_qbody_bt_n_ius_dn11 = 0.0;
        locals.var_qbody_bt_n_ius_dn12 = 0.0;
        locals.var_qbody_bt_n_ius_dn17 = 0.0;

        locals.var_uc_areabt = 0.0;

        locals.var_uc_vfbbt = 0.0;

        locals.var_q_bt_ge = 0.0;
        locals.var_q_bt_ge_dn0 = 0.0;
        locals.var_q_bt_ge_dn2 = 0.0;
        locals.var_q_bt_ge_dn6 = 0.0;
        locals.var_q_bt_ge_dn7 = 0.0;
        locals.var_q_bt_ge_dn10 = 0.0;
        locals.var_q_bt_ge_dn11 = 0.0;
        locals.var_q_bt_ge_dn12 = 0.0;
        locals.var_q_bt_ge_dn17 = 0.0;

        locals.var_q_bt_se = 0.0;
        locals.var_q_bt_se_dn0 = 0.0;
        locals.var_q_bt_se_dn2 = 0.0;
        locals.var_q_bt_se_dn6 = 0.0;
        locals.var_q_bt_se_dn7 = 0.0;
        locals.var_q_bt_se_dn10 = 0.0;
        locals.var_q_bt_se_dn11 = 0.0;
        locals.var_q_bt_se_dn12 = 0.0;
        locals.var_q_bt_se_dn17 = 0.0;

        locals.var_mud_hoso = 0.0;
        locals.var_mud_hoso_dn0 = 0.0;
        locals.var_mud_hoso_dn2 = 0.0;
        locals.var_mud_hoso_dn6 = 0.0;
        locals.var_mud_hoso_dn7 = 0.0;
        locals.var_mud_hoso_dn10 = 0.0;
        locals.var_mud_hoso_dn11 = 0.0;
        locals.var_mud_hoso_dn12 = 0.0;
        locals.var_mud_hoso_dn17 = 0.0;

        locals.var_kusai00 = 0.0;
        locals.var_kusai00_dn0 = 0.0;
        locals.var_kusai00_dn2 = 0.0;
        locals.var_kusai00_dn6 = 0.0;
        locals.var_kusai00_dn7 = 0.0;
        locals.var_kusai00_dn10 = 0.0;
        locals.var_kusai00_dn11 = 0.0;
        locals.var_kusai00_dn12 = 0.0;
        locals.var_kusai00_dn17 = 0.0;

    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_kusail = 0.0;
        locals.var_kusail_dn0 = 0.0;
        locals.var_kusail_dn2 = 0.0;
        locals.var_kusail_dn6 = 0.0;
        locals.var_kusail_dn7 = 0.0;
        locals.var_kusail_dn10 = 0.0;
        locals.var_kusail_dn11 = 0.0;
        locals.var_kusail_dn12 = 0.0;
        locals.var_kusail_dn17 = 0.0;

        locals.var_kusai00l = 0.0;
        locals.var_kusai00l_dn0 = 0.0;
        locals.var_kusai00l_dn2 = 0.0;
        locals.var_kusai00l_dn6 = 0.0;
        locals.var_kusai00l_dn7 = 0.0;
        locals.var_kusai00l_dn10 = 0.0;
        locals.var_kusai00l_dn11 = 0.0;
        locals.var_kusai00l_dn12 = 0.0;
        locals.var_kusai00l_dn17 = 0.0;

        locals.var_sqrtkusail = 0.0;
        locals.var_sqrtkusail_dn0 = 0.0;
        locals.var_sqrtkusail_dn2 = 0.0;
        locals.var_sqrtkusail_dn6 = 0.0;
        locals.var_sqrtkusail_dn7 = 0.0;
        locals.var_sqrtkusail_dn10 = 0.0;
        locals.var_sqrtkusail_dn11 = 0.0;
        locals.var_sqrtkusail_dn12 = 0.0;
        locals.var_sqrtkusail_dn17 = 0.0;

        locals.var_kusai_ig = 0.0;
        locals.var_kusai_ig_dn0 = 0.0;
        locals.var_kusai_ig_dn2 = 0.0;
        locals.var_kusai_ig_dn6 = 0.0;
        locals.var_kusai_ig_dn7 = 0.0;
        locals.var_kusai_ig_dn10 = 0.0;
        locals.var_kusai_ig_dn11 = 0.0;
        locals.var_kusai_ig_dn12 = 0.0;
        locals.var_kusai_ig_dn17 = 0.0;

        locals.var_crl_f = 0.0;
        locals.var_crl_f_dn0 = 0.0;
        locals.var_crl_f_dn2 = 0.0;
        locals.var_crl_f_dn6 = 0.0;
        locals.var_crl_f_dn7 = 0.0;
        locals.var_crl_f_dn10 = 0.0;
        locals.var_crl_f_dn11 = 0.0;
        locals.var_crl_f_dn12 = 0.0;
        locals.var_crl_f_dn17 = 0.0;

        locals.var_psdl = 0.0;
        locals.var_psdl_dn0 = 0.0;
        locals.var_psdl_dn2 = 0.0;
        locals.var_psdl_dn6 = 0.0;
        locals.var_psdl_dn7 = 0.0;
        locals.var_psdl_dn10 = 0.0;
        locals.var_psdl_dn11 = 0.0;
        locals.var_psdl_dn12 = 0.0;
        locals.var_psdl_dn17 = 0.0;

        locals.var_ec = 0.0;
        locals.var_ec_dn0 = 0.0;
        locals.var_ec_dn2 = 0.0;
        locals.var_ec_dn6 = 0.0;
        locals.var_ec_dn7 = 0.0;
        locals.var_ec_dn10 = 0.0;
        locals.var_ec_dn11 = 0.0;
        locals.var_ec_dn12 = 0.0;
        locals.var_ec_dn17 = 0.0;

        locals.var_lred = 0.0;
        locals.var_lred_dn0 = 0.0;
        locals.var_lred_dn2 = 0.0;
        locals.var_lred_dn6 = 0.0;
        locals.var_lred_dn7 = 0.0;
        locals.var_lred_dn10 = 0.0;
        locals.var_lred_dn11 = 0.0;
        locals.var_lred_dn12 = 0.0;
        locals.var_lred_dn17 = 0.0;

        locals.var_flg_depmode = 0.0;

        locals.var_phi_sl_soi_ini = 0.0;
        locals.var_phi_sl_soi_ini_dn0 = 0.0;
        locals.var_phi_sl_soi_ini_dn2 = 0.0;
        locals.var_phi_sl_soi_ini_dn6 = 0.0;
        locals.var_phi_sl_soi_ini_dn7 = 0.0;
        locals.var_phi_sl_soi_ini_dn10 = 0.0;
        locals.var_phi_sl_soi_ini_dn11 = 0.0;
        locals.var_phi_sl_soi_ini_dn12 = 0.0;
        locals.var_phi_sl_soi_ini_dn17 = 0.0;

        locals.var_phi_bl_soi_ini = 0.0;
        locals.var_phi_bl_soi_ini_dn0 = 0.0;
        locals.var_phi_bl_soi_ini_dn2 = 0.0;
        locals.var_phi_bl_soi_ini_dn6 = 0.0;
        locals.var_phi_bl_soi_ini_dn7 = 0.0;
        locals.var_phi_bl_soi_ini_dn10 = 0.0;
        locals.var_phi_bl_soi_ini_dn11 = 0.0;
        locals.var_phi_bl_soi_ini_dn12 = 0.0;
        locals.var_phi_bl_soi_ini_dn17 = 0.0;

        locals.var_phi_sl_bulk_ini = 0.0;
        locals.var_phi_sl_bulk_ini_dn0 = 0.0;
        locals.var_phi_sl_bulk_ini_dn2 = 0.0;
        locals.var_phi_sl_bulk_ini_dn6 = 0.0;
        locals.var_phi_sl_bulk_ini_dn7 = 0.0;
        locals.var_phi_sl_bulk_ini_dn10 = 0.0;
        locals.var_phi_sl_bulk_ini_dn11 = 0.0;
        locals.var_phi_sl_bulk_ini_dn12 = 0.0;
        locals.var_phi_sl_bulk_ini_dn17 = 0.0;

        locals.var_phi_s0_soi = 0.0;
        locals.var_phi_s0_soi_dn0 = 0.0;
        locals.var_phi_s0_soi_dn2 = 0.0;
        locals.var_phi_s0_soi_dn6 = 0.0;
        locals.var_phi_s0_soi_dn7 = 0.0;
        locals.var_phi_s0_soi_dn10 = 0.0;
        locals.var_phi_s0_soi_dn11 = 0.0;
        locals.var_phi_s0_soi_dn12 = 0.0;
        locals.var_phi_s0_soi_dn17 = 0.0;

        locals.var_phi_b0_soi = 0.0;
        locals.var_phi_b0_soi_dn0 = 0.0;
        locals.var_phi_b0_soi_dn2 = 0.0;
        locals.var_phi_b0_soi_dn6 = 0.0;
        locals.var_phi_b0_soi_dn7 = 0.0;
        locals.var_phi_b0_soi_dn10 = 0.0;
        locals.var_phi_b0_soi_dn11 = 0.0;
        locals.var_phi_b0_soi_dn12 = 0.0;
        locals.var_phi_b0_soi_dn17 = 0.0;

        locals.var_phi_s0_bulk = 0.0;
        locals.var_phi_s0_bulk_dn0 = 0.0;
        locals.var_phi_s0_bulk_dn2 = 0.0;
        locals.var_phi_s0_bulk_dn6 = 0.0;
        locals.var_phi_s0_bulk_dn7 = 0.0;
        locals.var_phi_s0_bulk_dn10 = 0.0;
        locals.var_phi_s0_bulk_dn11 = 0.0;
        locals.var_phi_s0_bulk_dn12 = 0.0;
        locals.var_phi_s0_bulk_dn17 = 0.0;

        locals.var_phi_sl_soi = 0.0;
        locals.var_phi_sl_soi_dn0 = 0.0;
        locals.var_phi_sl_soi_dn2 = 0.0;
        locals.var_phi_sl_soi_dn6 = 0.0;
        locals.var_phi_sl_soi_dn7 = 0.0;
        locals.var_phi_sl_soi_dn10 = 0.0;
        locals.var_phi_sl_soi_dn11 = 0.0;
        locals.var_phi_sl_soi_dn12 = 0.0;
        locals.var_phi_sl_soi_dn17 = 0.0;

        locals.var_phi_bl_soi = 0.0;
        locals.var_phi_bl_soi_dn0 = 0.0;
        locals.var_phi_bl_soi_dn2 = 0.0;
        locals.var_phi_bl_soi_dn6 = 0.0;
        locals.var_phi_bl_soi_dn7 = 0.0;
        locals.var_phi_bl_soi_dn10 = 0.0;
        locals.var_phi_bl_soi_dn11 = 0.0;
        locals.var_phi_bl_soi_dn12 = 0.0;
        locals.var_phi_bl_soi_dn17 = 0.0;

        locals.var_phi_sl_bulk = 0.0;
        locals.var_phi_sl_bulk_dn0 = 0.0;
        locals.var_phi_sl_bulk_dn2 = 0.0;
        locals.var_phi_sl_bulk_dn6 = 0.0;
        locals.var_phi_sl_bulk_dn7 = 0.0;
        locals.var_phi_sl_bulk_dn10 = 0.0;
        locals.var_phi_sl_bulk_dn11 = 0.0;
        locals.var_phi_sl_bulk_dn12 = 0.0;
        locals.var_phi_sl_bulk_dn17 = 0.0;

        locals.var_q_dep_soi = 0.0;
        locals.var_q_dep_soi_dn0 = 0.0;
        locals.var_q_dep_soi_dn2 = 0.0;
        locals.var_q_dep_soi_dn6 = 0.0;
        locals.var_q_dep_soi_dn7 = 0.0;
        locals.var_q_dep_soi_dn10 = 0.0;
        locals.var_q_dep_soi_dn11 = 0.0;
        locals.var_q_dep_soi_dn12 = 0.0;
        locals.var_q_dep_soi_dn17 = 0.0;

        locals.var_q_n0 = 0.0;
        locals.var_q_n0_dn0 = 0.0;
        locals.var_q_n0_dn2 = 0.0;
        locals.var_q_n0_dn6 = 0.0;
        locals.var_q_n0_dn7 = 0.0;
        locals.var_q_n0_dn10 = 0.0;
        locals.var_q_n0_dn11 = 0.0;
        locals.var_q_n0_dn12 = 0.0;
        locals.var_q_n0_dn17 = 0.0;

        locals.var_q_b0_dep = 0.0;
        locals.var_q_b0_dep_dn0 = 0.0;
        locals.var_q_b0_dep_dn2 = 0.0;
        locals.var_q_b0_dep_dn6 = 0.0;
        locals.var_q_b0_dep_dn7 = 0.0;
        locals.var_q_b0_dep_dn10 = 0.0;
        locals.var_q_b0_dep_dn11 = 0.0;
        locals.var_q_b0_dep_dn12 = 0.0;
        locals.var_q_b0_dep_dn17 = 0.0;

        locals.var_q_bl_dep = 0.0;
        locals.var_q_bl_dep_dn0 = 0.0;
        locals.var_q_bl_dep_dn2 = 0.0;
        locals.var_q_bl_dep_dn6 = 0.0;
        locals.var_q_bl_dep_dn7 = 0.0;
        locals.var_q_bl_dep_dn10 = 0.0;
        locals.var_q_bl_dep_dn11 = 0.0;
        locals.var_q_bl_dep_dn12 = 0.0;
        locals.var_q_bl_dep_dn17 = 0.0;

        locals.var_q_dep0 = 0.0;
        locals.var_q_dep0_dn0 = 0.0;
        locals.var_q_dep0_dn2 = 0.0;
        locals.var_q_dep0_dn6 = 0.0;
        locals.var_q_dep0_dn7 = 0.0;
        locals.var_q_dep0_dn10 = 0.0;
        locals.var_q_dep0_dn11 = 0.0;
        locals.var_q_dep0_dn12 = 0.0;
        locals.var_q_dep0_dn17 = 0.0;

        locals.var_q_s0_bulk = 0.0;
        locals.var_q_s0_bulk_dn0 = 0.0;
        locals.var_q_s0_bulk_dn2 = 0.0;
        locals.var_q_s0_bulk_dn6 = 0.0;
        locals.var_q_s0_bulk_dn7 = 0.0;
        locals.var_q_s0_bulk_dn10 = 0.0;
        locals.var_q_s0_bulk_dn11 = 0.0;
        locals.var_q_s0_bulk_dn12 = 0.0;
        locals.var_q_s0_bulk_dn17 = 0.0;

        locals.var_q_nl = 0.0;
        locals.var_q_nl_dn0 = 0.0;
        locals.var_q_nl_dn2 = 0.0;
        locals.var_q_nl_dn6 = 0.0;
        locals.var_q_nl_dn7 = 0.0;
        locals.var_q_nl_dn10 = 0.0;
        locals.var_q_nl_dn11 = 0.0;
        locals.var_q_nl_dn12 = 0.0;
        locals.var_q_nl_dn17 = 0.0;

        locals.var_q_depl = 0.0;
        locals.var_q_depl_dn0 = 0.0;
        locals.var_q_depl_dn2 = 0.0;
        locals.var_q_depl_dn6 = 0.0;
        locals.var_q_depl_dn7 = 0.0;
        locals.var_q_depl_dn10 = 0.0;
        locals.var_q_depl_dn11 = 0.0;
        locals.var_q_depl_dn12 = 0.0;
        locals.var_q_depl_dn17 = 0.0;

        locals.var_q_sl_bulk = 0.0;
        locals.var_q_sl_bulk_dn0 = 0.0;
        locals.var_q_sl_bulk_dn2 = 0.0;
        locals.var_q_sl_bulk_dn6 = 0.0;
        locals.var_q_sl_bulk_dn7 = 0.0;
        locals.var_q_sl_bulk_dn10 = 0.0;
        locals.var_q_sl_bulk_dn11 = 0.0;
        locals.var_q_sl_bulk_dn12 = 0.0;
        locals.var_q_sl_bulk_dn17 = 0.0;

        locals.var_shift = 0.0;
        locals.var_shift_dn0 = 0.0;
        locals.var_shift_dn2 = 0.0;
        locals.var_shift_dn6 = 0.0;
        locals.var_shift_dn7 = 0.0;
        locals.var_shift_dn10 = 0.0;
        locals.var_shift_dn11 = 0.0;
        locals.var_shift_dn12 = 0.0;
        locals.var_shift_dn17 = 0.0;

        locals.var_q_s0_bulk_0 = 0.0;

        locals.var_qi_nqs = 0.0;
        locals.var_qi_nqs_dn18 = 0.0;

        locals.var_qd_nqs = 0.0;
        locals.var_qd_nqs_dn0 = 0.0;
        locals.var_qd_nqs_dn2 = 0.0;
        locals.var_qd_nqs_dn6 = 0.0;
        locals.var_qd_nqs_dn7 = 0.0;
        locals.var_qd_nqs_dn10 = 0.0;
        locals.var_qd_nqs_dn11 = 0.0;
        locals.var_qd_nqs_dn12 = 0.0;
        locals.var_qd_nqs_dn15 = 0.0;
        locals.var_qd_nqs_dn17 = 0.0;
        locals.var_qd_nqs_dn18 = 0.0;

        locals.var_qs_nqs = 0.0;
        locals.var_qs_nqs_dn0 = 0.0;
        locals.var_qs_nqs_dn2 = 0.0;
        locals.var_qs_nqs_dn6 = 0.0;
        locals.var_qs_nqs_dn7 = 0.0;
        locals.var_qs_nqs_dn10 = 0.0;
        locals.var_qs_nqs_dn11 = 0.0;
        locals.var_qs_nqs_dn12 = 0.0;
        locals.var_qs_nqs_dn16 = 0.0;
        locals.var_qs_nqs_dn17 = 0.0;
        locals.var_qs_nqs_dn18 = 0.0;

        locals.var_phi_b_dep0 = 0.0;
        locals.var_phi_b_dep0_dn0 = 0.0;
        locals.var_phi_b_dep0_dn2 = 0.0;
        locals.var_phi_b_dep0_dn6 = 0.0;
        locals.var_phi_b_dep0_dn7 = 0.0;
        locals.var_phi_b_dep0_dn10 = 0.0;
        locals.var_phi_b_dep0_dn11 = 0.0;
        locals.var_phi_b_dep0_dn12 = 0.0;
        locals.var_phi_b_dep0_dn17 = 0.0;

        locals.var_qsub = 0.0;
        locals.var_qsub_dn0 = 0.0;
        locals.var_qsub_dn2 = 0.0;
        locals.var_qsub_dn6 = 0.0;
        locals.var_qsub_dn7 = 0.0;
        locals.var_qsub_dn10 = 0.0;
        locals.var_qsub_dn11 = 0.0;
        locals.var_qsub_dn12 = 0.0;
        locals.var_qsub_dn17 = 0.0;

        locals.var_qhs = 0.0;
        locals.var_qhs_dn0 = 0.0;
        locals.var_qhs_dn2 = 0.0;
        locals.var_qhs_dn6 = 0.0;
        locals.var_qhs_dn7 = 0.0;
        locals.var_qhs_dn10 = 0.0;
        locals.var_qhs_dn11 = 0.0;
        locals.var_qhs_dn12 = 0.0;
        locals.var_qhs_dn17 = 0.0;

        locals.var_wdsoi = 0.0;
        locals.var_wdsoi_dn0 = 0.0;
        locals.var_wdsoi_dn2 = 0.0;
        locals.var_wdsoi_dn6 = 0.0;
        locals.var_wdsoi_dn7 = 0.0;
        locals.var_wdsoi_dn10 = 0.0;
        locals.var_wdsoi_dn11 = 0.0;
        locals.var_wdsoi_dn12 = 0.0;
        locals.var_wdsoi_dn17 = 0.0;

        locals.var_ps0_inia = 0.0;
        locals.var_ps0_inia_dn0 = 0.0;
        locals.var_ps0_inia_dn2 = 0.0;
        locals.var_ps0_inia_dn6 = 0.0;
        locals.var_ps0_inia_dn7 = 0.0;
        locals.var_ps0_inia_dn10 = 0.0;
        locals.var_ps0_inia_dn11 = 0.0;
        locals.var_ps0_inia_dn12 = 0.0;
        locals.var_ps0_inia_dn17 = 0.0;

        locals.var_qiu = 0.0;
        locals.var_qiu_dn0 = 0.0;
        locals.var_qiu_dn2 = 0.0;
        locals.var_qiu_dn6 = 0.0;
        locals.var_qiu_dn7 = 0.0;
        locals.var_qiu_dn10 = 0.0;
        locals.var_qiu_dn11 = 0.0;
        locals.var_qiu_dn12 = 0.0;
        locals.var_qiu_dn17 = 0.0;

        locals.var_qbu = 0.0;
        locals.var_qbu_dn0 = 0.0;
        locals.var_qbu_dn2 = 0.0;
        locals.var_qbu_dn6 = 0.0;
        locals.var_qbu_dn7 = 0.0;
        locals.var_qbu_dn10 = 0.0;
        locals.var_qbu_dn11 = 0.0;
        locals.var_qbu_dn12 = 0.0;
        locals.var_qbu_dn17 = 0.0;

        locals.var_qdrat = 0.5;
        locals.var_qdrat_dn0 = 0.0;
        locals.var_qdrat_dn2 = 0.0;
        locals.var_qdrat_dn6 = 0.0;
        locals.var_qdrat_dn7 = 0.0;
        locals.var_qdrat_dn10 = 0.0;
        locals.var_qdrat_dn11 = 0.0;
        locals.var_qdrat_dn12 = 0.0;
        locals.var_qdrat_dn17 = 0.0;

        locals.var_qdrat_noi = 0.5;
        locals.var_qdrat_noi_dn0 = 0.0;
        locals.var_qdrat_noi_dn2 = 0.0;
        locals.var_qdrat_noi_dn6 = 0.0;
        locals.var_qdrat_noi_dn7 = 0.0;
        locals.var_qdrat_noi_dn10 = 0.0;
        locals.var_qdrat_noi_dn11 = 0.0;
        locals.var_qdrat_noi_dn12 = 0.0;
        locals.var_qdrat_noi_dn17 = 0.0;

        locals.var_qs_fb = 0.0;
        locals.var_qs_fb_dn0 = 0.0;
        locals.var_qs_fb_dn2 = 0.0;
        locals.var_qs_fb_dn6 = 0.0;
        locals.var_qs_fb_dn7 = 0.0;
        locals.var_qs_fb_dn10 = 0.0;
        locals.var_qs_fb_dn11 = 0.0;
        locals.var_qs_fb_dn12 = 0.0;
        locals.var_qs_fb_dn13 = 0.0;
        locals.var_qs_fb_dn15 = 0.0;
        locals.var_qs_fb_dn16 = 0.0;
        locals.var_qs_fb_dn17 = 0.0;
        locals.var_qs_fb_dn18 = 0.0;

        locals.var_qd_fb = 0.0;
        locals.var_qd_fb_dn0 = 0.0;
        locals.var_qd_fb_dn2 = 0.0;
        locals.var_qd_fb_dn6 = 0.0;
        locals.var_qd_fb_dn7 = 0.0;
        locals.var_qd_fb_dn10 = 0.0;
        locals.var_qd_fb_dn11 = 0.0;
        locals.var_qd_fb_dn12 = 0.0;
        locals.var_qd_fb_dn13 = 0.0;
        locals.var_qd_fb_dn15 = 0.0;
        locals.var_qd_fb_dn16 = 0.0;
        locals.var_qd_fb_dn17 = 0.0;
        locals.var_qd_fb_dn18 = 0.0;

        locals.var_fs01 = 0.0;
        locals.var_fs01_dn0 = 0.0;
        locals.var_fs01_dn2 = 0.0;
        locals.var_fs01_dn6 = 0.0;
        locals.var_fs01_dn7 = 0.0;
        locals.var_fs01_dn10 = 0.0;
        locals.var_fs01_dn11 = 0.0;
        locals.var_fs01_dn12 = 0.0;
        locals.var_fs01_dn17 = 0.0;

        locals.var_fs02 = 0.0;
        locals.var_fs02_dn0 = 0.0;
        locals.var_fs02_dn2 = 0.0;
        locals.var_fs02_dn6 = 0.0;
        locals.var_fs02_dn7 = 0.0;
        locals.var_fs02_dn10 = 0.0;
        locals.var_fs02_dn11 = 0.0;
        locals.var_fs02_dn12 = 0.0;
        locals.var_fs02_dn17 = 0.0;

        locals.var_fsl1 = 0.0;
        locals.var_fsl1_dn0 = 0.0;
        locals.var_fsl1_dn2 = 0.0;
        locals.var_fsl1_dn6 = 0.0;
        locals.var_fsl1_dn7 = 0.0;
        locals.var_fsl1_dn10 = 0.0;
        locals.var_fsl1_dn11 = 0.0;
        locals.var_fsl1_dn12 = 0.0;
        locals.var_fsl1_dn17 = 0.0;

        locals.var_fsl2 = 0.0;
        locals.var_fsl2_dn0 = 0.0;
        locals.var_fsl2_dn2 = 0.0;
        locals.var_fsl2_dn6 = 0.0;
        locals.var_fsl2_dn7 = 0.0;
        locals.var_fsl2_dn10 = 0.0;
        locals.var_fsl2_dn11 = 0.0;
        locals.var_fsl2_dn12 = 0.0;
        locals.var_fsl2_dn17 = 0.0;

        let assign1220_e978: f64 = (p.p51 * 10.0);
        let assign1220_e980: f64 = (assign1220_e978 % 10.0);
        locals.var_subversion = assign1220_e980;

        locals.var_lp_s0_max = 200.0;

        locals.var_lp_sl_max = 200.0;

        locals.var_flg_skipacc = 0.0;

        locals.var_vbsbiz = 0.0;
        locals.var_vbsbiz_dn0 = 0.0;
        locals.var_vbsbiz_dn2 = 0.0;
        locals.var_vbsbiz_dn6 = 0.0;
        locals.var_vbsbiz_dn7 = 0.0;
        locals.var_vbsbiz_dn10 = 0.0;
        locals.var_vbsbiz_dn11 = 0.0;
        locals.var_vbsbiz_dn12 = 0.0;
        locals.var_vbsbiz_dn17 = 0.0;

        locals.var_ps0_ini = 0.0;
        locals.var_ps0_ini_dn0 = 0.0;
        locals.var_ps0_ini_dn2 = 0.0;
        locals.var_ps0_ini_dn6 = 0.0;
        locals.var_ps0_ini_dn7 = 0.0;
        locals.var_ps0_ini_dn10 = 0.0;
        locals.var_ps0_ini_dn11 = 0.0;
        locals.var_ps0_ini_dn12 = 0.0;
        locals.var_ps0_ini_dn17 = 0.0;

        locals.var_q_s0_dep_ini = 0.0;
        locals.var_q_s0_dep_ini_dn0 = 0.0;
        locals.var_q_s0_dep_ini_dn2 = 0.0;
        locals.var_q_s0_dep_ini_dn6 = 0.0;
        locals.var_q_s0_dep_ini_dn7 = 0.0;
        locals.var_q_s0_dep_ini_dn10 = 0.0;
        locals.var_q_s0_dep_ini_dn11 = 0.0;
        locals.var_q_s0_dep_ini_dn12 = 0.0;
        locals.var_q_s0_dep_ini_dn17 = 0.0;

        locals.var_idspt0 = 0.0;
        locals.var_idspt0_dn0 = 0.0;
        locals.var_idspt0_dn2 = 0.0;
        locals.var_idspt0_dn6 = 0.0;
        locals.var_idspt0_dn7 = 0.0;
        locals.var_idspt0_dn10 = 0.0;
        locals.var_idspt0_dn11 = 0.0;
        locals.var_idspt0_dn12 = 0.0;
        locals.var_idspt0_dn17 = 0.0;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        locals.var_ps0 = 0.0;
        locals.var_ps0_dn0 = 0.0;
        locals.var_ps0_dn2 = 0.0;
        locals.var_ps0_dn6 = 0.0;
        locals.var_ps0_dn7 = 0.0;
        locals.var_ps0_dn10 = 0.0;
        locals.var_ps0_dn11 = 0.0;
        locals.var_ps0_dn12 = 0.0;
        locals.var_ps0_dn17 = 0.0;

        locals.var_vbcs_cl = 0.0;
        locals.var_vbcs_cl_dn0 = 0.0;
        locals.var_vbcs_cl_dn2 = 0.0;
        locals.var_vbcs_cl_dn6 = 0.0;
        locals.var_vbcs_cl_dn7 = 0.0;
        locals.var_vbcs_cl_dn10 = 0.0;
        locals.var_vbcs_cl_dn11 = 0.0;
        locals.var_vbcs_cl_dn12 = 0.0;
        locals.var_vbcs_cl_dn17 = 0.0;

        let assign1330_e993: f64 = (p.p52 * 0.01);
        locals.var_mks_vmax = assign1330_e993;

        let assign1340_e996: f64 = (p.p73 / 1e-6);
        locals.var_mks_nsubp = assign1340_e996;

        let assign1350_e999: f64 = (p.p104 * 0.01);
        locals.var_mks_vtmp = assign1350_e999;

        let assign1360_e1002: f64 = (p.p201 / 1e-6);
        locals.var_mks_nsubcmax = assign1360_e1002;

        let assign1400_e1014: f64 = (p.p240 / 1e-6);
        locals.var_mks_nsubs = assign1400_e1014;

        let assign1410_e1017: f64 = (p.p241 / 1e-6);
        locals.var_mks_nsubb = assign1410_e1017;

        let assign1420_e1020: f64 = (p.p242 * 0.01);
        locals.var_mks_rth0 = assign1420_e1020;

        let assign1430_e1023: f64 = (p.p243 / 0.01);
        locals.var_mks_cth0 = assign1430_e1023;

        let assign1440_e1026: f64 = (p.p59 / 1e-6);
        locals.var_mks_nover = assign1440_e1026;

        let assign1450_e1029: f64 = (p.p284 / 1e-6);
        locals.var_mks_njunc = assign1450_e1029;

        let assign1460_e1032: f64 = (p.p148 / 1e-6);
        locals.var_mks_nsti = assign1460_e1032;

        let assign1470_e1035: f64 = (p.p198 / 0.0001);
        locals.var_mks_wfc = assign1470_e1035;

        let assign1480_e1038: f64 = (p.p70 * 0.01);
        locals.var_mks_parl1 = assign1480_e1038;

        let (assign1490_e1044,) = {
    if (p.p83 == 0.0) {
        (0.0,)
    } else {
        (p.p84,)
    }
};
        locals.var_uc_sc2 = assign1490_e1044;

        let (assign1500_e1050,) = {
    if (p.p83 == 0.0) {
        (0.0,)
    } else {
        (p.p85,)
    }
};
        locals.var_uc_sc3 = assign1500_e1050;

        let (assign1510_e1056,) = {
    if (p.p80 == 0.0) {
        (0.0,)
    } else {
        (p.p81,)
    }
};
        locals.var_uc_scp2 = assign1510_e1056;

        let (assign1520_e1062,) = {
    if (p.p83 == 0.0) {
        (0.0,)
    } else {
        (p.p82,)
    }
};
        locals.var_uc_scp3 = assign1520_e1062;

        let assign1530_e1065: f64 = (p.p250 * 1000000.0);
        locals.var_uc_gdld = assign1530_e1065;

        let assign1540_e1068: f64 = (p.p232 + 273.15);
        locals.var_uc_tnom = assign1540_e1068;

        locals.var_uc_vfbover = p.p58;

        locals.var_flg_info = p.p46;

        locals.var_flg_nqs = p.p34;

        let (assign1590_e1083,) = {
    if param_given[190] {
        (p.p190,)
    } else {
        let assign1590_e1081: f64 = (p.p237 * p.p240);
        let assign1590_e1082: f64 = (5000000000.0 / assign1590_e1081);
        (assign1590_e1082,)
    }
};
        locals.var_uc_clm2 = assign1590_e1083;
        locals.var_uc_clm2_dn0 = 0.0;
        locals.var_uc_clm2_dn2 = 0.0;
        locals.var_uc_clm2_dn6 = 0.0;
        locals.var_uc_clm2_dn7 = 0.0;
        locals.var_uc_clm2_dn10 = 0.0;
        locals.var_uc_clm2_dn11 = 0.0;
        locals.var_uc_clm2_dn12 = 0.0;
        locals.var_uc_clm2_dn17 = 0.0;

        let assign1600_e1087: f64 = (2.0 + 0.1);
        let assign1600_e1092: f64 = if ((locals.var_uc_clm2 < assign1600_e1087) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard4 = assign1600_e1092;

        let (assign1610_e1100, assign1610_e1100_d_n0, assign1610_e1100_d_n2, assign1610_e1100_d_n6, assign1610_e1100_d_n7, assign1610_e1100_d_n10, assign1610_e1100_d_n11, assign1610_e1100_d_n12, assign1610_e1100_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        let assign1610_e1096: f64 = (2.0 + 0.1);
        let assign1610_e1098: f64 = (assign1610_e1096 - locals.var_uc_clm2);
        (assign1610_e1098, (-locals.var_uc_clm2_dn0), (-locals.var_uc_clm2_dn2), (-locals.var_uc_clm2_dn6), (-locals.var_uc_clm2_dn7), (-locals.var_uc_clm2_dn10), (-locals.var_uc_clm2_dn11), (-locals.var_uc_clm2_dn12), (-locals.var_uc_clm2_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign1610_e1100;
        locals.var_tmf1_dn0 = assign1610_e1100_d_n0;
        locals.var_tmf1_dn2 = assign1610_e1100_d_n2;
        locals.var_tmf1_dn6 = assign1610_e1100_d_n6;
        locals.var_tmf1_dn7 = assign1610_e1100_d_n7;
        locals.var_tmf1_dn10 = assign1610_e1100_d_n10;
        locals.var_tmf1_dn11 = assign1610_e1100_d_n11;
        locals.var_tmf1_dn12 = assign1610_e1100_d_n12;
        locals.var_tmf1_dn17 = assign1610_e1100_d_n17;

        let (assign1620_e1106, assign1620_e1106_d_n0, assign1620_e1106_d_n2, assign1620_e1106_d_n6, assign1620_e1106_d_n7, assign1620_e1106_d_n10, assign1620_e1106_d_n11, assign1620_e1106_d_n12, assign1620_e1106_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        let assign1620_e1104: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign1620_e1104, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign1620_e1106;
        locals.var_x2_dn0 = assign1620_e1106_d_n0;
        locals.var_x2_dn2 = assign1620_e1106_d_n2;
        locals.var_x2_dn6 = assign1620_e1106_d_n6;
        locals.var_x2_dn7 = assign1620_e1106_d_n7;
        locals.var_x2_dn10 = assign1620_e1106_d_n10;
        locals.var_x2_dn11 = assign1620_e1106_d_n11;
        locals.var_x2_dn12 = assign1620_e1106_d_n12;
        locals.var_x2_dn17 = assign1620_e1106_d_n17;

        let (assign1630_e1112, assign1630_e1112_d_n0, assign1630_e1112_d_n2, assign1630_e1112_d_n6, assign1630_e1112_d_n7, assign1630_e1112_d_n10, assign1630_e1112_d_n11, assign1630_e1112_d_n12, assign1630_e1112_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        let assign1630_e1110: f64 = (0.1 * 0.1);
        (assign1630_e1110, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign1630_e1112;
        locals.var_xmax2_dn0 = assign1630_e1112_d_n0;
        locals.var_xmax2_dn2 = assign1630_e1112_d_n2;
        locals.var_xmax2_dn6 = assign1630_e1112_d_n6;
        locals.var_xmax2_dn7 = assign1630_e1112_d_n7;
        locals.var_xmax2_dn10 = assign1630_e1112_d_n10;
        locals.var_xmax2_dn11 = assign1630_e1112_d_n11;
        locals.var_xmax2_dn12 = assign1630_e1112_d_n12;
        locals.var_xmax2_dn17 = assign1630_e1112_d_n17;

        let (assign1640_e1116, assign1640_e1116_d_n0, assign1640_e1116_d_n2, assign1640_e1116_d_n6, assign1640_e1116_d_n7, assign1640_e1116_d_n10, assign1640_e1116_d_n11, assign1640_e1116_d_n12, assign1640_e1116_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign1640_e1116;
        locals.var_xp_dn0 = assign1640_e1116_d_n0;
        locals.var_xp_dn2 = assign1640_e1116_d_n2;
        locals.var_xp_dn6 = assign1640_e1116_d_n6;
        locals.var_xp_dn7 = assign1640_e1116_d_n7;
        locals.var_xp_dn10 = assign1640_e1116_d_n10;
        locals.var_xp_dn11 = assign1640_e1116_d_n11;
        locals.var_xp_dn12 = assign1640_e1116_d_n12;
        locals.var_xp_dn17 = assign1640_e1116_d_n17;

        let (assign1650_e1120, assign1650_e1120_d_n0, assign1650_e1120_d_n2, assign1650_e1120_d_n6, assign1650_e1120_d_n7, assign1650_e1120_d_n10, assign1650_e1120_d_n11, assign1650_e1120_d_n12, assign1650_e1120_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign1650_e1120;
        locals.var_xmp_dn0 = assign1650_e1120_d_n0;
        locals.var_xmp_dn2 = assign1650_e1120_d_n2;
        locals.var_xmp_dn6 = assign1650_e1120_d_n6;
        locals.var_xmp_dn7 = assign1650_e1120_d_n7;
        locals.var_xmp_dn10 = assign1650_e1120_d_n10;
        locals.var_xmp_dn11 = assign1650_e1120_d_n11;
        locals.var_xmp_dn12 = assign1650_e1120_d_n12;
        locals.var_xmp_dn17 = assign1650_e1120_d_n17;

        let (assign1660_e1124,) = {
    if (locals.var_guard4 != 0.0) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign1660_e1124;

        let (assign1670_e1128,) = {
    if (locals.var_guard4 != 0.0) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign1670_e1128;

        let (assign1680_e1132, assign1680_e1132_d_n0, assign1680_e1132_d_n2, assign1680_e1132_d_n6, assign1680_e1132_d_n7, assign1680_e1132_d_n10, assign1680_e1132_d_n11, assign1680_e1132_d_n12, assign1680_e1132_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign1680_e1132;
        locals.var_arg_dn0 = assign1680_e1132_d_n0;
        locals.var_arg_dn2 = assign1680_e1132_d_n2;
        locals.var_arg_dn6 = assign1680_e1132_d_n6;
        locals.var_arg_dn7 = assign1680_e1132_d_n7;
        locals.var_arg_dn10 = assign1680_e1132_d_n10;
        locals.var_arg_dn11 = assign1680_e1132_d_n11;
        locals.var_arg_dn12 = assign1680_e1132_d_n12;
        locals.var_arg_dn17 = assign1680_e1132_d_n17;

        let (assign1690_e1136, assign1690_e1136_d_n0, assign1690_e1136_d_n2, assign1690_e1136_d_n6, assign1690_e1136_d_n7, assign1690_e1136_d_n10, assign1690_e1136_d_n11, assign1690_e1136_d_n12, assign1690_e1136_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign1690_e1136;
        locals.var_dnm_dn0 = assign1690_e1136_d_n0;
        locals.var_dnm_dn2 = assign1690_e1136_d_n2;
        locals.var_dnm_dn6 = assign1690_e1136_d_n6;
        locals.var_dnm_dn7 = assign1690_e1136_d_n7;
        locals.var_dnm_dn10 = assign1690_e1136_d_n10;
        locals.var_dnm_dn11 = assign1690_e1136_d_n11;
        locals.var_dnm_dn12 = assign1690_e1136_d_n12;
        locals.var_dnm_dn17 = assign1690_e1136_d_n17;

        let (assign1700_e1142, assign1700_e1142_d_n0, assign1700_e1142_d_n2, assign1700_e1142_d_n6, assign1700_e1142_d_n7, assign1700_e1142_d_n10, assign1700_e1142_d_n11, assign1700_e1142_d_n12, assign1700_e1142_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        let assign1700_e1140: f64 = (locals.var_xp * locals.var_x2);
        (assign1700_e1140, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign1700_e1142;
        locals.var_xp_dn0 = assign1700_e1142_d_n0;
        locals.var_xp_dn2 = assign1700_e1142_d_n2;
        locals.var_xp_dn6 = assign1700_e1142_d_n6;
        locals.var_xp_dn7 = assign1700_e1142_d_n7;
        locals.var_xp_dn10 = assign1700_e1142_d_n10;
        locals.var_xp_dn11 = assign1700_e1142_d_n11;
        locals.var_xp_dn12 = assign1700_e1142_d_n12;
        locals.var_xp_dn17 = assign1700_e1142_d_n17;

        let (assign1710_e1148, assign1710_e1148_d_n0, assign1710_e1148_d_n2, assign1710_e1148_d_n6, assign1710_e1148_d_n7, assign1710_e1148_d_n10, assign1710_e1148_d_n11, assign1710_e1148_d_n12, assign1710_e1148_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        let assign1710_e1146: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign1710_e1146, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign1710_e1148;
        locals.var_xmp_dn0 = assign1710_e1148_d_n0;
        locals.var_xmp_dn2 = assign1710_e1148_d_n2;
        locals.var_xmp_dn6 = assign1710_e1148_d_n6;
        locals.var_xmp_dn7 = assign1710_e1148_d_n7;
        locals.var_xmp_dn10 = assign1710_e1148_d_n10;
        locals.var_xmp_dn11 = assign1710_e1148_d_n11;
        locals.var_xmp_dn12 = assign1710_e1148_d_n12;
        locals.var_xmp_dn17 = assign1710_e1148_d_n17;

        let (assign1720_e1154, assign1720_e1154_d_n0, assign1720_e1154_d_n2, assign1720_e1154_d_n6, assign1720_e1154_d_n7, assign1720_e1154_d_n10, assign1720_e1154_d_n11, assign1720_e1154_d_n12, assign1720_e1154_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        let assign1720_e1152: f64 = (locals.var_xp * locals.var_x2);
        (assign1720_e1152, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign1720_e1154;
        locals.var_xp_dn0 = assign1720_e1154_d_n0;
        locals.var_xp_dn2 = assign1720_e1154_d_n2;
        locals.var_xp_dn6 = assign1720_e1154_d_n6;
        locals.var_xp_dn7 = assign1720_e1154_d_n7;
        locals.var_xp_dn10 = assign1720_e1154_d_n10;
        locals.var_xp_dn11 = assign1720_e1154_d_n11;
        locals.var_xp_dn12 = assign1720_e1154_d_n12;
        locals.var_xp_dn17 = assign1720_e1154_d_n17;

        let (assign1730_e1160, assign1730_e1160_d_n0, assign1730_e1160_d_n2, assign1730_e1160_d_n6, assign1730_e1160_d_n7, assign1730_e1160_d_n10, assign1730_e1160_d_n11, assign1730_e1160_d_n12, assign1730_e1160_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        let assign1730_e1158: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign1730_e1158, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign1730_e1160;
        locals.var_xmp_dn0 = assign1730_e1160_d_n0;
        locals.var_xmp_dn2 = assign1730_e1160_d_n2;
        locals.var_xmp_dn6 = assign1730_e1160_d_n6;
        locals.var_xmp_dn7 = assign1730_e1160_d_n7;
        locals.var_xmp_dn10 = assign1730_e1160_d_n10;
        locals.var_xmp_dn11 = assign1730_e1160_d_n11;
        locals.var_xmp_dn12 = assign1730_e1160_d_n12;
        locals.var_xmp_dn17 = assign1730_e1160_d_n17;

        let (assign1740_e1166, assign1740_e1166_d_n0, assign1740_e1166_d_n2, assign1740_e1166_d_n6, assign1740_e1166_d_n7, assign1740_e1166_d_n10, assign1740_e1166_d_n11, assign1740_e1166_d_n12, assign1740_e1166_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        let assign1740_e1164: f64 = (locals.var_xp + locals.var_xmp);
        (assign1740_e1164, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign1740_e1166;
        locals.var_arg_dn0 = assign1740_e1166_d_n0;
        locals.var_arg_dn2 = assign1740_e1166_d_n2;
        locals.var_arg_dn6 = assign1740_e1166_d_n6;
        locals.var_arg_dn7 = assign1740_e1166_d_n7;
        locals.var_arg_dn10 = assign1740_e1166_d_n10;
        locals.var_arg_dn11 = assign1740_e1166_d_n11;
        locals.var_arg_dn12 = assign1740_e1166_d_n12;
        locals.var_arg_dn17 = assign1740_e1166_d_n17;

        let (assign1750_e1170, assign1750_e1170_d_n0, assign1750_e1170_d_n2, assign1750_e1170_d_n6, assign1750_e1170_d_n7, assign1750_e1170_d_n10, assign1750_e1170_d_n11, assign1750_e1170_d_n12, assign1750_e1170_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign1750_e1170;
        locals.var_dnm_dn0 = assign1750_e1170_d_n0;
        locals.var_dnm_dn2 = assign1750_e1170_d_n2;
        locals.var_dnm_dn6 = assign1750_e1170_d_n6;
        locals.var_dnm_dn7 = assign1750_e1170_d_n7;
        locals.var_dnm_dn10 = assign1750_e1170_d_n10;
        locals.var_dnm_dn11 = assign1750_e1170_d_n11;
        locals.var_dnm_dn12 = assign1750_e1170_d_n12;
        locals.var_dnm_dn17 = assign1750_e1170_d_n17;

        let assign1760_e1185: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard5 = assign1760_e1185;

        let assign1770_e1188: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign1770_e1188;

        let (assign1780_e1196,) = {
    if (((locals.var_guard4 != 0.0) && (locals.var_guard5 != 0.0)) && (locals.var_guard6 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign1780_e1196;

        let assign1790_e1199: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard7 = assign1790_e1199;

        let (assign1800_e1210,) = {
    if ((((locals.var_guard4 != 0.0) && (locals.var_guard5 != 0.0)) && (locals.var_guard6 == 0.0)) && (locals.var_guard7 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign1800_e1210;

        let assign1810_e1213: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard8 = assign1810_e1213;

        let (assign1820_e1227,) = {
    if (((((locals.var_guard4 != 0.0) && (locals.var_guard5 != 0.0)) && (locals.var_guard6 == 0.0)) && (locals.var_guard7 == 0.0)) && (locals.var_guard8 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign1820_e1227;

        let assign1830_e1230: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard9 = assign1830_e1230;

        let (assign1840_e1247,) = {
    if ((((((locals.var_guard4 != 0.0) && (locals.var_guard5 != 0.0)) && (locals.var_guard6 == 0.0)) && (locals.var_guard7 == 0.0)) && (locals.var_guard8 == 0.0)) && (locals.var_guard9 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign1840_e1247;

        let (assign1850_e1253,) = {
    if ((locals.var_guard4 != 0.0) && (locals.var_guard5 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign1850_e1253;

        let mut assign1860_loop_guard: usize = 0;
        while {
            let assign1860_cond_e1260: f64 = if (((locals.var_guard4 != 0.0) && (locals.var_guard5 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign1860_cond_e1260 != 0.0
        } {
            assign1860_loop_guard += 1;
            assert!(assign1860_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign1860_body0_e1267, assign1860_body0_e1267_d_n0, assign1860_body0_e1267_d_n2, assign1860_body0_e1267_d_n6, assign1860_body0_e1267_d_n7, assign1860_body0_e1267_d_n10, assign1860_body0_e1267_d_n11, assign1860_body0_e1267_d_n12, assign1860_body0_e1267_d_n17,) = {
    if ((locals.var_guard4 != 0.0) && (locals.var_guard5 != 0.0)) {
        let assign1860_body0_e1265: f64 = (locals.var_dnm).sqrt();
        (assign1860_body0_e1265, (locals.var_dnm_dn0 / (2.0 * assign1860_body0_e1265)), (locals.var_dnm_dn2 / (2.0 * assign1860_body0_e1265)), (locals.var_dnm_dn6 / (2.0 * assign1860_body0_e1265)), (locals.var_dnm_dn7 / (2.0 * assign1860_body0_e1265)), (locals.var_dnm_dn10 / (2.0 * assign1860_body0_e1265)), (locals.var_dnm_dn11 / (2.0 * assign1860_body0_e1265)), (locals.var_dnm_dn12 / (2.0 * assign1860_body0_e1265)), (locals.var_dnm_dn17 / (2.0 * assign1860_body0_e1265)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign1860_body0_e1267;
            locals.var_dnm_dn0 = assign1860_body0_e1267_d_n0;
            locals.var_dnm_dn2 = assign1860_body0_e1267_d_n2;
            locals.var_dnm_dn6 = assign1860_body0_e1267_d_n6;
            locals.var_dnm_dn7 = assign1860_body0_e1267_d_n7;
            locals.var_dnm_dn10 = assign1860_body0_e1267_d_n10;
            locals.var_dnm_dn11 = assign1860_body0_e1267_d_n11;
            locals.var_dnm_dn12 = assign1860_body0_e1267_d_n12;
            locals.var_dnm_dn17 = assign1860_body0_e1267_d_n17;
            let (assign1860_body1_e1275,) = {
    if ((locals.var_guard4 != 0.0) && (locals.var_guard5 != 0.0)) {
        let assign1860_body1_e1273: f64 = (locals.var_m0 + 1.0);
        (assign1860_body1_e1273,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign1860_body1_e1275;
        }

        let (assign1870_e1288, assign1870_e1288_d_n0, assign1870_e1288_d_n2, assign1870_e1288_d_n6, assign1870_e1288_d_n7, assign1870_e1288_d_n10, assign1870_e1288_d_n11, assign1870_e1288_d_n12, assign1870_e1288_d_n17,) = {
    if ((locals.var_guard4 != 0.0) && (locals.var_guard5 == 0.0)) {
        let assign1870_e1284: f64 = (2.0 * 2.0);
        let assign1870_e1285: f64 = (1.0 / assign1870_e1284);
        let assign1870_e1286: f64 = (locals.var_dnm).powf(assign1870_e1285);
        (assign1870_e1286, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((locals.var_dnm).powf(assign1870_e1285 - 1.0) * locals.var_dnm_dn0)) } } else { (assign1870_e1286 * (assign1870_e1285 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((locals.var_dnm).powf(assign1870_e1285 - 1.0) * locals.var_dnm_dn2)) } } else { (assign1870_e1286 * (assign1870_e1285 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((locals.var_dnm).powf(assign1870_e1285 - 1.0) * locals.var_dnm_dn6)) } } else { (assign1870_e1286 * (assign1870_e1285 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((locals.var_dnm).powf(assign1870_e1285 - 1.0) * locals.var_dnm_dn7)) } } else { (assign1870_e1286 * (assign1870_e1285 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((locals.var_dnm).powf(assign1870_e1285 - 1.0) * locals.var_dnm_dn10)) } } else { (assign1870_e1286 * (assign1870_e1285 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((locals.var_dnm).powf(assign1870_e1285 - 1.0) * locals.var_dnm_dn11)) } } else { (assign1870_e1286 * (assign1870_e1285 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((locals.var_dnm).powf(assign1870_e1285 - 1.0) * locals.var_dnm_dn12)) } } else { (assign1870_e1286 * (assign1870_e1285 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign1870_e1285) as f64).is_finite() && ((assign1870_e1285) as f64).fract() == 0.0 { if assign1870_e1285 == 0.0 { 0.0 } else { (assign1870_e1285 * ((locals.var_dnm).powf(assign1870_e1285 - 1.0) * locals.var_dnm_dn17)) } } else { (assign1870_e1286 * (assign1870_e1285 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign1870_e1288;
        locals.var_dnm_dn0 = assign1870_e1288_d_n0;
        locals.var_dnm_dn2 = assign1870_e1288_d_n2;
        locals.var_dnm_dn6 = assign1870_e1288_d_n6;
        locals.var_dnm_dn7 = assign1870_e1288_d_n7;
        locals.var_dnm_dn10 = assign1870_e1288_d_n10;
        locals.var_dnm_dn11 = assign1870_e1288_d_n11;
        locals.var_dnm_dn12 = assign1870_e1288_d_n12;
        locals.var_dnm_dn17 = assign1870_e1288_d_n17;

        let (assign1880_e1294, assign1880_e1294_d_n0, assign1880_e1294_d_n2, assign1880_e1294_d_n6, assign1880_e1294_d_n7, assign1880_e1294_d_n10, assign1880_e1294_d_n11, assign1880_e1294_d_n12, assign1880_e1294_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        let assign1880_e1292: f64 = (1.0 / locals.var_dnm);
        (assign1880_e1292, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign1880_e1294;
        locals.var_dnm_dn0 = assign1880_e1294_d_n0;
        locals.var_dnm_dn2 = assign1880_e1294_d_n2;
        locals.var_dnm_dn6 = assign1880_e1294_d_n6;
        locals.var_dnm_dn7 = assign1880_e1294_d_n7;
        locals.var_dnm_dn10 = assign1880_e1294_d_n10;
        locals.var_dnm_dn11 = assign1880_e1294_d_n11;
        locals.var_dnm_dn12 = assign1880_e1294_d_n12;
        locals.var_dnm_dn17 = assign1880_e1294_d_n17;

    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1890_e1302, assign1890_e1302_d_n0, assign1890_e1302_d_n2, assign1890_e1302_d_n6, assign1890_e1302_d_n7, assign1890_e1302_d_n10, assign1890_e1302_d_n11, assign1890_e1302_d_n12, assign1890_e1302_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        let assign1890_e1298: f64 = (locals.var_tmf1 * 0.1);
        let assign1890_e1300: f64 = (assign1890_e1298 * locals.var_dnm);
        (assign1890_e1300, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign1890_e1298 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign1890_e1298 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign1890_e1298 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign1890_e1298 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign1890_e1298 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign1890_e1298 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * 0.1) * locals.var_dnm) + (assign1890_e1298 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * 0.1) * locals.var_dnm) + (assign1890_e1298 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign1890_e1302;
        locals.var_tmf0_dn0 = assign1890_e1302_d_n0;
        locals.var_tmf0_dn2 = assign1890_e1302_d_n2;
        locals.var_tmf0_dn6 = assign1890_e1302_d_n6;
        locals.var_tmf0_dn7 = assign1890_e1302_d_n7;
        locals.var_tmf0_dn10 = assign1890_e1302_d_n10;
        locals.var_tmf0_dn11 = assign1890_e1302_d_n11;
        locals.var_tmf0_dn12 = assign1890_e1302_d_n12;
        locals.var_tmf0_dn17 = assign1890_e1302_d_n17;

        let (assign1900_e1310, assign1900_e1310_d_n0, assign1900_e1310_d_n2, assign1900_e1310_d_n6, assign1900_e1310_d_n7, assign1900_e1310_d_n10, assign1900_e1310_d_n11, assign1900_e1310_d_n12, assign1900_e1310_d_n17,) = {
    if (locals.var_guard4 != 0.0) {
        let assign1900_e1306: f64 = (2.0 + 0.1);
        let assign1900_e1308: f64 = (assign1900_e1306 - locals.var_tmf0);
        (assign1900_e1308, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn12), (-locals.var_tmf0_dn17),)
    } else {
        (locals.var_uc_clm2, locals.var_uc_clm2_dn0, locals.var_uc_clm2_dn2, locals.var_uc_clm2_dn6, locals.var_uc_clm2_dn7, locals.var_uc_clm2_dn10, locals.var_uc_clm2_dn11, locals.var_uc_clm2_dn12, locals.var_uc_clm2_dn17,)
    }
};
        locals.var_uc_clm2 = assign1900_e1310;
        locals.var_uc_clm2_dn0 = assign1900_e1310_d_n0;
        locals.var_uc_clm2_dn2 = assign1900_e1310_d_n2;
        locals.var_uc_clm2_dn6 = assign1900_e1310_d_n6;
        locals.var_uc_clm2_dn7 = assign1900_e1310_d_n7;
        locals.var_uc_clm2_dn10 = assign1900_e1310_d_n10;
        locals.var_uc_clm2_dn11 = assign1900_e1310_d_n11;
        locals.var_uc_clm2_dn12 = assign1900_e1310_d_n12;
        locals.var_uc_clm2_dn17 = assign1900_e1310_d_n17;

        let (assign1910_e1315, assign1910_e1315_d_n0, assign1910_e1315_d_n2, assign1910_e1315_d_n6, assign1910_e1315_d_n7, assign1910_e1315_d_n10, assign1910_e1315_d_n11, assign1910_e1315_d_n12, assign1910_e1315_d_n17,) = {
    if (locals.var_guard4 == 0.0) {
        (locals.var_uc_clm2, locals.var_uc_clm2_dn0, locals.var_uc_clm2_dn2, locals.var_uc_clm2_dn6, locals.var_uc_clm2_dn7, locals.var_uc_clm2_dn10, locals.var_uc_clm2_dn11, locals.var_uc_clm2_dn12, locals.var_uc_clm2_dn17,)
    } else {
        (locals.var_uc_clm2, locals.var_uc_clm2_dn0, locals.var_uc_clm2_dn2, locals.var_uc_clm2_dn6, locals.var_uc_clm2_dn7, locals.var_uc_clm2_dn10, locals.var_uc_clm2_dn11, locals.var_uc_clm2_dn12, locals.var_uc_clm2_dn17,)
    }
};
        locals.var_uc_clm2 = assign1910_e1315;
        locals.var_uc_clm2_dn0 = assign1910_e1315_d_n0;
        locals.var_uc_clm2_dn2 = assign1910_e1315_d_n2;
        locals.var_uc_clm2_dn6 = assign1910_e1315_d_n6;
        locals.var_uc_clm2_dn7 = assign1910_e1315_d_n7;
        locals.var_uc_clm2_dn10 = assign1910_e1315_d_n10;
        locals.var_uc_clm2_dn11 = assign1910_e1315_d_n11;
        locals.var_uc_clm2_dn12 = assign1910_e1315_d_n12;
        locals.var_uc_clm2_dn17 = assign1910_e1315_d_n17;

        let assign1920_e1321: f64 = (locals.var_uc_tnom * 1e-7);
        let assign1920_e1322: f64 = (9.025e-5 + assign1920_e1321);
        let assign1920_e1323: f64 = (locals.var_uc_tnom * assign1920_e1322);
        let assign1920_e1324: f64 = (p.p55 - assign1920_e1323);
        locals.var_egtnom = assign1920_e1324;

        locals.var_tfox0 = p.p236;

        let assign1940_e1328: f64 = (1.034943e-10 / p.p237);
        locals.var_c_soi = assign1940_e1328;

        let assign1950_e1331: f64 = (1.0 / locals.var_c_soi);
        locals.var_c_soi_inv = assign1950_e1331;

        let assign1960_e1334: f64 = (3.453133e-11 / locals.var_tfox0);
        locals.var_c_fox0 = assign1960_e1334;

        let assign1970_e1337: f64 = (locals.var_tfox0 / 3.453133e-11);
        locals.var_c_fox0_inv = assign1970_e1337;

        let assign1980_e1340: f64 = (3.453133e-11 / p.p239);
        locals.var_c_box = assign1980_e1340;

        let assign1990_e1343: f64 = (p.p239 / 3.453133e-11);
        locals.var_c_box_inv = assign1990_e1343;

        let assign2000_e1346: f64 = (locals.var_c_box_inv + locals.var_c_soi_inv);
        locals.var_c_box_fd_inv = assign2000_e1346;

        locals.var_lgate = p.p0;

        let assign2020_e1351: f64 = (2.0 * p.p56);
        let assign2020_e1352: f64 = (locals.var_lgate - assign2020_e1351);
        locals.var_leff = assign2020_e1352;

        let assign2030_e1356: f64 = (2.0 * p.p57);
        let assign2030_e1357: f64 = (locals.var_lgate - assign2030_e1356);
        locals.var_leff_cv = assign2030_e1357;

        let (assign2040_e1363,) = {
    if (p.p40 == 0.0) {
        (locals.var_lgate,)
    } else {
        (locals.var_leff,)
    }
};
        locals.var_lgleff = assign2040_e1363;

        let assign2050_e1366: f64 = (locals.var_lgleff * 1000000.0);
        locals.var_lgle = assign2050_e1366;

        let assign2060_e1369: f64 = (p.p1 / p.p9);
        locals.var_wgate = assign2060_e1369;

        locals.var_dw = p.p60;

        let (assign2080_e1376,) = {
    if (locals.var_subversion < 1.0) {
        (0.0,)
    } else {
        (p.p295,)
    }
};
        locals.var_dwbt = assign2080_e1376;

        let (assign2090_e1382,) = {
    if (locals.var_subversion < 1.0) {
        (p.p60,)
    } else {
        (p.p61,)
    }
};
        locals.var_dwcv = assign2090_e1382;

        let assign2100_e1385: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard10 = assign2100_e1385;

        let (assign2110_e1393,) = {
    if (locals.var_guard10 != 0.0) {
        let assign2110_e1390: f64 = (2.0 * locals.var_dw);
        let assign2110_e1391: f64 = (locals.var_wgate - assign2110_e1390);
        (assign2110_e1391,)
    } else {
        (locals.var_weff,)
    }
};
        locals.var_weff = assign2110_e1393;

        let (assign2120_e1401,) = {
    if (locals.var_guard10 != 0.0) {
        let assign2120_e1398: f64 = (2.0 * locals.var_dwcv);
        let assign2120_e1399: f64 = (locals.var_wgate - assign2120_e1398);
        (assign2120_e1399,)
    } else {
        (locals.var_weff_cv,)
    }
};
        locals.var_weff_cv = assign2120_e1401;

        let (assign2130_e1416,) = {
    if (locals.var_guard10 == 0.0) {
        let assign2130_e1407: f64 = (p.p18 * locals.var_dwbt);
        let assign2130_e1408: f64 = (locals.var_wgate - assign2130_e1407);
        let assign2130_e1411: f64 = (2.0 - p.p18);
        let assign2130_e1413: f64 = (assign2130_e1411 * locals.var_dw);
        let assign2130_e1414: f64 = (assign2130_e1408 - assign2130_e1413);
        (assign2130_e1414,)
    } else {
        (locals.var_weff,)
    }
};
        locals.var_weff = assign2130_e1416;

        let (assign2140_e1431,) = {
    if (locals.var_guard10 == 0.0) {
        let assign2140_e1422: f64 = (p.p18 * locals.var_dwbt);
        let assign2140_e1423: f64 = (locals.var_wgate - assign2140_e1422);
        let assign2140_e1426: f64 = (2.0 - p.p18);
        let assign2140_e1428: f64 = (assign2140_e1426 * locals.var_dwcv);
        let assign2140_e1429: f64 = (assign2140_e1423 - assign2140_e1428);
        (assign2140_e1429,)
    } else {
        (locals.var_weff_cv,)
    }
};
        locals.var_weff_cv = assign2140_e1431;

        let assign2150_e1434: f64 = (locals.var_weff * p.p9);
        locals.var_weff_nf = assign2150_e1434;

        let assign2160_e1437: f64 = (locals.var_weff_cv * p.p9);
        locals.var_weffcv_nf = assign2160_e1437;

        let assign2170_e1440: f64 = (locals.var_wgate * 1000000.0);
        locals.var_wg = assign2170_e1440;

        let assign2180_e1443: f64 = (locals.var_wg * locals.var_lgle);
        locals.var_wl = assign2180_e1443;

        let assign2190_e1449: f64 = (locals.var_lgle).powf(p.p111);
        let assign2190_e1450: f64 = (p.p108 / assign2190_e1449);
        let assign2190_e1451: f64 = (1.0 + assign2190_e1450);
        let assign2190_e1452: f64 = (p.p107 * assign2190_e1451);
        let assign2190_e1457: f64 = (locals.var_wg).powf(p.p110);
        let assign2190_e1458: f64 = (p.p109 / assign2190_e1457);
        let assign2190_e1459: f64 = (1.0 + assign2190_e1458);
        let assign2190_e1460: f64 = (assign2190_e1452 * assign2190_e1459);
        locals.var_muesr = assign2190_e1460;

        let assign2200_e1471: f64 = if (((locals.var_subversion > 3.0) && (locals.var_mks_nsubp < locals.var_mks_nsubs)) && (p.p72 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard11 = assign2200_e1471;

        let (assign2210_e1475,) = {
    if (locals.var_guard11 != 0.0) {
        (locals.var_mks_nsubs,)
    } else {
        (locals.var_mks_nsubp,)
    }
};
        locals.var_mks_nsubp = assign2210_e1475;

        let assign2220_e1481: f64 = (locals.var_wg).powf(p.p75);
        let assign2220_e1482: f64 = (p.p74 / assign2220_e1481);
        let assign2220_e1483: f64 = (1.0 + assign2220_e1482);
        let assign2220_e1484: f64 = (locals.var_mks_nsubp * assign2220_e1483);
        locals.var_nsubpp = assign2220_e1484;

        let assign2230_e1490: f64 = (0.5 * locals.var_lgate);
        let assign2230_e1491: f64 = (p.p62 + assign2230_e1490);
        let assign2230_e1492: f64 = (1.0 / assign2230_e1491);
        let assign2230_e1497: f64 = (0.5 * locals.var_lgate);
        let assign2230_e1498: f64 = (p.p63 + assign2230_e1497);
        let assign2230_e1499: f64 = (1.0 / assign2230_e1498);
        let assign2230_e1500: f64 = (assign2230_e1492 + assign2230_e1499);
        let assign2230_e1501: f64 = (2.0 / assign2230_e1500);
        locals.var_lod_half_ref = assign2230_e1501;

        let assign2240_e1505: f64 = (1.3806226e-23 * locals.var_uc_tnom);
        let assign2240_e1506: f64 = (1.6021918e-19 / assign2240_e1505);
        locals.var_betatnom = assign2240_e1506;

        let assign2250_e1509: f64 = (1.6021918e-19 * locals.var_mks_nsubb);
        let assign2250_e1511: f64 = (assign2250_e1509 * 1.034943e-10);
        locals.var_qnbulk_esi = assign2250_e1511;

        let assign2260_e1515: f64 = (-p.p247);
        let assign2260_e1516: f64 = (locals.var_lgle).powf(assign2260_e1515);
        let assign2260_e1517: f64 = (p.p244 * assign2260_e1516);
        locals.var_ptl0 = assign2260_e1517;

        let assign2270_e1521: f64 = (-p.p252);
        let assign2270_e1522: f64 = (locals.var_lgle).powf(assign2270_e1521);
        let assign2270_e1523: f64 = (p.p251 * assign2270_e1522);
        locals.var_pt40 = assign2270_e1523;

        let assign2280_e1527: f64 = (locals.var_lgle + locals.var_uc_gdld);
        let assign2280_e1529: f64 = (-p.p249);
        let assign2280_e1530: f64 = (assign2280_e1527).powf(assign2280_e1529);
        let assign2280_e1531: f64 = (p.p248 * assign2280_e1530);
        locals.var_gdl0 = assign2280_e1531;

        let assign2290_e1534: f64 = (2.0 * 1.6021918e-19);
        let assign2290_e1536: f64 = (assign2290_e1534 * locals.var_mks_nsti);
        let assign2290_e1538: f64 = (assign2290_e1536 * 1.034943e-10);
        let assign2290_e1539: f64 = (assign2290_e1538).sqrt();
        locals.var_costi00 = assign2290_e1539;

        let assign2300_e1543: f64 = (locals.var_mks_nsti * locals.var_mks_nsti);
        let assign2300_e1544: f64 = (1.0 / assign2300_e1543);
        locals.var_nsti_p2 = assign2300_e1544;

        let assign2310_e1548: f64 = (1.0 / locals.var_lgle);
        let assign2310_e1549: f64 = (1.0 + assign2310_e1548);
        let assign2310_e1551: f64 = (assign2310_e1549).powf(p.p91);
        let assign2310_e1553: f64 = (assign2310_e1551 * p.p89);
        locals.var_cnstpgd = assign2310_e1553;

        locals.var_c0bulk = locals.var_qnbulk_esi;

        locals.var_vfb = p.p68;

        let assign2340_e1560: f64 = (locals.var_wl).powf(p.p77);
        let assign2340_e1561: f64 = (p.p76 / assign2340_e1560);
        let assign2340_e1562: f64 = (locals.var_lgleff + assign2340_e1561);
        locals.var_lgatesm = assign2340_e1562;

        let assign2350_e1566: f64 = (locals.var_wl).powf(p.p79);
        let assign2350_e1567: f64 = (p.p78 / assign2350_e1566);
        locals.var_dvthsm = assign2350_e1567;

        let assign2360_e1573: f64 = (locals.var_lgatesm * 1000000.0);
        let assign2360_e1575: f64 = (assign2360_e1573).powf(p.p151);
        let assign2360_e1576: f64 = (p.p150 / assign2360_e1575);
        let assign2360_e1577: f64 = (1.0 + assign2360_e1576);
        let assign2360_e1578: f64 = (p.p149 * assign2360_e1577);
        let assign2360_e1580: f64 = assign2360_e1578;
        let assign2360_e1584: f64 = (locals.var_wg).powf(p.p153);
        let assign2360_e1585: f64 = (p.p152 / assign2360_e1584);
        let assign2360_e1586: f64 = (assign2360_e1580 + assign2360_e1585);
        locals.var_uc_wsti = assign2360_e1586;

        let assign2370_e1590: f64 = (locals.var_lgle).powf(p.p192);
        let assign2370_e1592: f64 = (assign2370_e1590 * p.p193);
        let assign2370_e1593: f64 = (1.0 + assign2370_e1592);
        locals.var_clmmod = assign2370_e1593;

        let assign2380_e1599: f64 = (3.0 * p.p6);
        let assign2380_e1600: f64 = (locals.var_weff / assign2380_e1599);
        let assign2380_e1601: f64 = (p.p7 + assign2380_e1600);
        let assign2380_e1602: f64 = (p.p67 * assign2380_e1601);
        let assign2380_e1606: f64 = (locals.var_lgate - p.p8);
        let assign2380_e1607: f64 = (p.p6 * assign2380_e1606);
        let assign2380_e1609: f64 = (assign2380_e1607 * p.p9);
        let assign2380_e1610: f64 = (assign2380_e1602 / assign2380_e1609);
        locals.var_grg_cnst = assign2380_e1610;

        let assign2390_e1613: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard12 = assign2390_e1613;

        let (assign2400_e1623,) = {
    if (locals.var_guard12 != 0.0) {
        let assign2400_e1619: f64 = (locals.var_wg).powf(p.p131);
        let assign2400_e1620: f64 = (p.p130 / assign2400_e1619);
        let assign2400_e1621: f64 = (1.0 + assign2400_e1620);
        (assign2400_e1621,)
    } else {
        (locals.var_zvgs,)
    }
};
        locals.var_zvgs = assign2400_e1623;

        let (assign2410_e1635,) = {
    if (locals.var_guard12 != 0.0) {
        let assign2410_e1630: f64 = (locals.var_lgle).powf(p.p126);
        let assign2410_e1631: f64 = (p.p125 / assign2410_e1630);
        let assign2410_e1632: f64 = (1.0 + assign2410_e1631);
        let assign2410_e1633: f64 = (p.p124 * assign2410_e1632);
        (assign2410_e1633,)
    } else {
        (locals.var_xvbs,)
    }
};
        locals.var_xvbs = assign2410_e1635;

        let (assign2420_e1643,) = {
    if (locals.var_guard12 != 0.0) {
        let assign2420_e1640: f64 = (locals.var_lgle + p.p123);
        let assign2420_e1641: f64 = (locals.var_lgle / assign2420_e1640);
        (assign2420_e1641,)
    } else {
        (locals.var_xgate,)
    }
};
        locals.var_xgate = assign2420_e1643;

        let (assign2430_e1655,) = {
    if (locals.var_guard12 != 0.0) {
        let assign2430_e1650: f64 = (locals.var_lgle).powf(p.p120);
        let assign2430_e1651: f64 = (p.p119 / assign2430_e1650);
        let assign2430_e1652: f64 = (1.0 + assign2430_e1651);
        let assign2430_e1653: f64 = (p.p117 * assign2430_e1652);
        (assign2430_e1653,)
    } else {
        (locals.var_xsub1,)
    }
};
        locals.var_xsub1 = assign2430_e1655;

        let (assign2440_e1665,) = {
    if (locals.var_guard12 != 0.0) {
        let assign2440_e1661: f64 = (p.p121 / locals.var_lgle);
        let assign2440_e1662: f64 = (1.0 + assign2440_e1661);
        let assign2440_e1663: f64 = (p.p118 * assign2440_e1662);
        (assign2440_e1663,)
    } else {
        (locals.var_xsub2,)
    }
};
        locals.var_xsub2 = assign2440_e1665;

        let (assign2450_e1672, assign2450_e1672_d_n0, assign2450_e1672_d_n2, assign2450_e1672_d_n6, assign2450_e1672_d_n7, assign2450_e1672_d_n10, assign2450_e1672_d_n11, assign2450_e1672_d_n12, assign2450_e1672_d_n17,) = {
    if (locals.var_guard12 == 0.0) {
        let assign2450_e1670: f64 = (locals.var_wg).powf(p.p131);
        (assign2450_e1670, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign2450_e1672;
        locals.var_t2_dn0 = assign2450_e1672_d_n0;
        locals.var_t2_dn2 = assign2450_e1672_d_n2;
        locals.var_t2_dn6 = assign2450_e1672_d_n6;
        locals.var_t2_dn7 = assign2450_e1672_d_n7;
        locals.var_t2_dn10 = assign2450_e1672_d_n10;
        locals.var_t2_dn11 = assign2450_e1672_d_n11;
        locals.var_t2_dn12 = assign2450_e1672_d_n12;
        locals.var_t2_dn17 = assign2450_e1672_d_n17;

        let (assign2460_e1691, assign2460_e1691_d_n0, assign2460_e1691_d_n2, assign2460_e1691_d_n6, assign2460_e1691_d_n7, assign2460_e1691_d_n10, assign2460_e1691_d_n11, assign2460_e1691_d_n12, assign2460_e1691_d_n17,) = {
    if (locals.var_guard12 == 0.0) {
        let assign2460_e1680: f64 = (locals.var_lgle).powf(p.p129);
        let assign2460_e1681: f64 = (p.p128 / assign2460_e1680);
        let assign2460_e1682: f64 = (1.0 + assign2460_e1681);
        let assign2460_e1683: f64 = (p.p127 * assign2460_e1682);
        let assign2460_e1687: f64 = (locals.var_t2 + p.p130);
        let assign2460_e1688: f64 = (locals.var_t2 / assign2460_e1687);
        let assign2460_e1689: f64 = (assign2460_e1683 * assign2460_e1688);
        (assign2460_e1689, (assign2460_e1683 * (((locals.var_t2_dn0 * assign2460_e1687) - (locals.var_t2 * locals.var_t2_dn0)) / (assign2460_e1687 * assign2460_e1687))), (assign2460_e1683 * (((locals.var_t2_dn2 * assign2460_e1687) - (locals.var_t2 * locals.var_t2_dn2)) / (assign2460_e1687 * assign2460_e1687))), (assign2460_e1683 * (((locals.var_t2_dn6 * assign2460_e1687) - (locals.var_t2 * locals.var_t2_dn6)) / (assign2460_e1687 * assign2460_e1687))), (assign2460_e1683 * (((locals.var_t2_dn7 * assign2460_e1687) - (locals.var_t2 * locals.var_t2_dn7)) / (assign2460_e1687 * assign2460_e1687))), (assign2460_e1683 * (((locals.var_t2_dn10 * assign2460_e1687) - (locals.var_t2 * locals.var_t2_dn10)) / (assign2460_e1687 * assign2460_e1687))), (assign2460_e1683 * (((locals.var_t2_dn11 * assign2460_e1687) - (locals.var_t2 * locals.var_t2_dn11)) / (assign2460_e1687 * assign2460_e1687))), (assign2460_e1683 * (((locals.var_t2_dn12 * assign2460_e1687) - (locals.var_t2 * locals.var_t2_dn12)) / (assign2460_e1687 * assign2460_e1687))), (assign2460_e1683 * (((locals.var_t2_dn17 * assign2460_e1687) - (locals.var_t2 * locals.var_t2_dn17)) / (assign2460_e1687 * assign2460_e1687))),)
    } else {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn10, locals.var_vg2const_dn11, locals.var_vg2const_dn12, locals.var_vg2const_dn17,)
    }
};
        locals.var_vg2const = assign2460_e1691;
        locals.var_vg2const_dn0 = assign2460_e1691_d_n0;
        locals.var_vg2const_dn2 = assign2460_e1691_d_n2;
        locals.var_vg2const_dn6 = assign2460_e1691_d_n6;
        locals.var_vg2const_dn7 = assign2460_e1691_d_n7;
        locals.var_vg2const_dn10 = assign2460_e1691_d_n10;
        locals.var_vg2const_dn11 = assign2460_e1691_d_n11;
        locals.var_vg2const_dn12 = assign2460_e1691_d_n12;
        locals.var_vg2const_dn17 = assign2460_e1691_d_n17;

        let (assign2470_e1704,) = {
    if (locals.var_guard12 == 0.0) {
        let assign2470_e1699: f64 = (locals.var_lgle).powf(p.p126);
        let assign2470_e1700: f64 = (p.p125 / assign2470_e1699);
        let assign2470_e1701: f64 = (1.0 + assign2470_e1700);
        let assign2470_e1702: f64 = (p.p124 * assign2470_e1701);
        (assign2470_e1702,)
    } else {
        (locals.var_xvbs,)
    }
};
        locals.var_xvbs = assign2470_e1704;

        let (assign2480_e1717,) = {
    if (locals.var_guard12 == 0.0) {
        let assign2480_e1712: f64 = (locals.var_lgle).powf(p.p133);
        let assign2480_e1713: f64 = (p.p132 / assign2480_e1712);
        let assign2480_e1714: f64 = (1.0 + assign2480_e1713);
        let assign2480_e1715: f64 = (p.p123 * assign2480_e1714);
        (assign2480_e1715,)
    } else {
        (locals.var_xgate,)
    }
};
        locals.var_xgate = assign2480_e1717;

        let (assign2490_e1730,) = {
    if (locals.var_guard12 == 0.0) {
        let assign2490_e1725: f64 = (locals.var_lgle).powf(p.p120);
        let assign2490_e1726: f64 = (p.p119 / assign2490_e1725);
        let assign2490_e1727: f64 = (1.0 + assign2490_e1726);
        let assign2490_e1728: f64 = (p.p117 * assign2490_e1727);
        (assign2490_e1728,)
    } else {
        (locals.var_xsub1,)
    }
};
        locals.var_xsub1 = assign2490_e1730;

        let (assign2500_e1741,) = {
    if (locals.var_guard12 == 0.0) {
        let assign2500_e1737: f64 = (p.p121 / locals.var_lgle);
        let assign2500_e1738: f64 = (1.0 + assign2500_e1737);
        let assign2500_e1739: f64 = (p.p118 * assign2500_e1738);
        (assign2500_e1739,)
    } else {
        (locals.var_xsub2,)
    }
};
        locals.var_xsub2 = assign2500_e1741;

        let assign2510_e1744: f64 = (1000000.0 * locals.var_weffcv_nf);
        let assign2510_e1746: f64 = (assign2510_e1744 * p.p65);
        let assign2510_e1749: f64 = (locals.var_lgle).powf(p.p66);
        let assign2510_e1750: f64 = (assign2510_e1746 / assign2510_e1749);
        locals.var_cqyb0 = assign2510_e1750;

        let assign2520_e1756: f64 = (locals.var_lgle).powf(p.p136);
        let assign2520_e1757: f64 = (p.p135 / assign2520_e1756);
        let assign2520_e1758: f64 = (1.0 + assign2520_e1757);
        let assign2520_e1759: f64 = (p.p134 * assign2520_e1758);
        locals.var_vfbsub0 = assign2520_e1759;

        let assign2530_e1762: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard13 = assign2530_e1762;

        let (assign2540_e1774,) = {
    if (locals.var_guard13 != 0.0) {
        let assign2540_e1769: f64 = (locals.var_lgle).powf(p.p129);
        let assign2540_e1770: f64 = (p.p128 / assign2540_e1769);
        let assign2540_e1771: f64 = (1.0 + assign2540_e1770);
        let assign2540_e1772: f64 = (p.p127 * assign2540_e1771);
        (assign2540_e1772,)
    } else {
        (locals.var_uc_svgs,)
    }
};
        locals.var_uc_svgs = assign2540_e1774;

        let assign2550_e1777: f64 = (p.p115 * locals.var_lgle);
        let assign2550_e1779: f64 = (assign2550_e1777 * p.p114);
        let assign2550_e1782: f64 = (p.p115 * locals.var_lgle);
        let assign2550_e1784: f64 = (assign2550_e1782 + p.p114);
        let assign2550_e1785: f64 = (assign2550_e1779 / assign2550_e1784);
        let assign2550_e1787: f64 = (assign2550_e1785 + p.p116);
        let assign2550_e1789: f64 = (assign2550_e1787 + 1e-50);
        locals.var_ddlte = assign2550_e1789;

        let assign2560_e1792: f64 = if locals.var_ddlte < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign2560_e1792;

    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign2570_e1796,) = {
    if (locals.var_guard14 != 0.0) {
        (3.0,)
    } else {
        (locals.var_ddlte,)
    }
};
        locals.var_ddlte = assign2570_e1796;

        let assign2580_e1799: f64 = (p.p50 * p.p253);
        locals.var_vgs_min = assign2580_e1799;

        let assign2590_e1801: f64 = if param_given[168] { 1.0 } else { 0.0 };
        locals.var_cgbo_given = assign2590_e1801;

        let assign2600_e1803: f64 = if param_given[169] { 1.0 } else { 0.0 };
        locals.var_cgdo_given = assign2600_e1803;

        let assign2610_e1805: f64 = if param_given[170] { 1.0 } else { 0.0 };
        locals.var_cgso_given = assign2610_e1805;

        let assign2620_e1807: f64 = if param_given[294] { 1.0 } else { 0.0 };
        locals.var_cbtbp_given = assign2620_e1807;

        let assign2630_e1809: f64 = if param_given[293] { 1.0 } else { 0.0 };
        locals.var_cbtbn_given = assign2630_e1809;

        let assign2640_e1811: f64 = if param_given[13] { 1.0 } else { 0.0 };
        locals.var_pdbcp_given = assign2640_e1811;

        let assign2650_e1813: f64 = if param_given[14] { 1.0 } else { 0.0 };
        locals.var_psbcp_given = assign2650_e1813;

        let assign2660_e1815: f64 = if param_given[23] { 1.0 } else { 0.0 };
        locals.var_abtp_given = assign2660_e1815;

        let assign2670_e1817: f64 = if param_given[22] { 1.0 } else { 0.0 };
        locals.var_abtn_given = assign2670_e1817;

        let assign2680_e1819: f64 = if param_given[16] { 1.0 } else { 0.0 };
        locals.var_temp_given = assign2680_e1819;

        let (assign2690_e1825,) = {
    if (p.p17 == 0.0) {
        (0.0,)
    } else {
        (1.0,)
    }
};
        locals.var_dtemp_given = assign2690_e1825;

        locals.var_mfactor = 1.0;

        let assign2710_e1829: f64 = 0.0;
        locals.var_gjmin = assign2710_e1829;

        locals.var_uc_pdbcp = p.p13;

        locals.var_uc_psbcp = p.p14;

        let assign2740_e1834: f64 = (p.p16 + 273.15);
        locals.var_uc_temp = assign2740_e1834;

        let assign2750_e1838: f64 = (locals.var_mfactor * locals.var_weff_nf);
        let assign2750_e1839: f64 = (locals.var_mks_rth0 / assign2750_e1838);
        locals.var_rth = assign2750_e1839;

        let assign2760_e1843: f64 = (locals.var_mfactor * locals.var_weffcv_nf);
        let assign2760_e1844: f64 = (locals.var_mks_cth0 * assign2760_e1843);
        locals.var_cth = assign2760_e1844;

        let assign2770_e1863: f64 = if (((p.p10 > 0.0) && (p.p11 > 0.0)) && ((p.p9 == 1.0) || ((p.p9 > 1.0) && (p.p12 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard15 = assign2770_e1863;

        let (assign2780_e1867, assign2780_e1867_d_n0, assign2780_e1867_d_n2, assign2780_e1867_d_n6, assign2780_e1867_d_n7, assign2780_e1867_d_n10, assign2780_e1867_d_n11, assign2780_e1867_d_n12, assign2780_e1867_d_n17,) = {
    if (locals.var_guard15 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign2780_e1867;
        locals.var_t1_dn0 = assign2780_e1867_d_n0;
        locals.var_t1_dn2 = assign2780_e1867_d_n2;
        locals.var_t1_dn6 = assign2780_e1867_d_n6;
        locals.var_t1_dn7 = assign2780_e1867_d_n7;
        locals.var_t1_dn10 = assign2780_e1867_d_n10;
        locals.var_t1_dn11 = assign2780_e1867_d_n11;
        locals.var_t1_dn12 = assign2780_e1867_d_n12;
        locals.var_t1_dn17 = assign2780_e1867_d_n17;

        let (assign2790_e1871,) = {
    if (locals.var_guard15 != 0.0) {
        (0.0,)
    } else {
        (locals.var_i,)
    }
};
        locals.var_i = assign2790_e1871;

        let mut assign2800_loop_guard: usize = 0;
        while {
            let assign2800_cond_e1876: f64 = if ((locals.var_guard15 != 0.0) && (locals.var_i < p.p9)) { 1.0 } else { 0.0 };
            assign2800_cond_e1876 != 0.0
        } {
            assign2800_loop_guard += 1;
            assert!(assign2800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign2800_body0_e1908, assign2800_body0_e1908_d_n0, assign2800_body0_e1908_d_n2, assign2800_body0_e1908_d_n6, assign2800_body0_e1908_d_n7, assign2800_body0_e1908_d_n10, assign2800_body0_e1908_d_n11, assign2800_body0_e1908_d_n12, assign2800_body0_e1908_d_n17,) = {
    if (locals.var_guard15 != 0.0) {
        let assign2800_body0_e1883: f64 = (0.5 * locals.var_lgate);
        let assign2800_body0_e1884: f64 = (p.p10 + assign2800_body0_e1883);
        let assign2800_body0_e1888: f64 = (p.p12 + locals.var_lgate);
        let assign2800_body0_e1889: f64 = (locals.var_i * assign2800_body0_e1888);
        let assign2800_body0_e1890: f64 = (assign2800_body0_e1884 + assign2800_body0_e1889);
        let assign2800_body0_e1891: f64 = (1.0 / assign2800_body0_e1890);
        let assign2800_body0_e1892: f64 = (locals.var_t1 + assign2800_body0_e1891);
        let assign2800_body0_e1897: f64 = (0.5 * locals.var_lgate);
        let assign2800_body0_e1898: f64 = (p.p11 + assign2800_body0_e1897);
        let assign2800_body0_e1902: f64 = (p.p12 + locals.var_lgate);
        let assign2800_body0_e1903: f64 = (locals.var_i * assign2800_body0_e1902);
        let assign2800_body0_e1904: f64 = (assign2800_body0_e1898 + assign2800_body0_e1903);
        let assign2800_body0_e1905: f64 = (1.0 / assign2800_body0_e1904);
        let assign2800_body0_e1906: f64 = (assign2800_body0_e1892 + assign2800_body0_e1905);
        (assign2800_body0_e1906, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign2800_body0_e1908;
            locals.var_t1_dn0 = assign2800_body0_e1908_d_n0;
            locals.var_t1_dn2 = assign2800_body0_e1908_d_n2;
            locals.var_t1_dn6 = assign2800_body0_e1908_d_n6;
            locals.var_t1_dn7 = assign2800_body0_e1908_d_n7;
            locals.var_t1_dn10 = assign2800_body0_e1908_d_n10;
            locals.var_t1_dn11 = assign2800_body0_e1908_d_n11;
            locals.var_t1_dn12 = assign2800_body0_e1908_d_n12;
            locals.var_t1_dn17 = assign2800_body0_e1908_d_n17;
            let (assign2800_body1_e1914,) = {
    if (locals.var_guard15 != 0.0) {
        let assign2800_body1_e1912: f64 = (locals.var_i + 1.0);
        (assign2800_body1_e1912,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign2800_body1_e1914;
        }

        let (assign2810_e1922, assign2810_e1922_d_n0, assign2810_e1922_d_n2, assign2810_e1922_d_n6, assign2810_e1922_d_n7, assign2810_e1922_d_n10, assign2810_e1922_d_n11, assign2810_e1922_d_n12, assign2810_e1922_d_n17,) = {
    if (locals.var_guard15 != 0.0) {
        let assign2810_e1918: f64 = (2.0 * p.p9);
        let assign2810_e1920: f64 = (assign2810_e1918 / locals.var_t1);
        (assign2810_e1920, (-((assign2810_e1918 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign2810_e1918 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign2810_e1918 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign2810_e1918 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign2810_e1918 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign2810_e1918 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((assign2810_e1918 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((assign2810_e1918 * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn6, locals.var_lod_half_dn7, locals.var_lod_half_dn10, locals.var_lod_half_dn11, locals.var_lod_half_dn12, locals.var_lod_half_dn17,)
    }
};
        locals.var_lod_half = assign2810_e1922;
        locals.var_lod_half_dn0 = assign2810_e1922_d_n0;
        locals.var_lod_half_dn2 = assign2810_e1922_d_n2;
        locals.var_lod_half_dn6 = assign2810_e1922_d_n6;
        locals.var_lod_half_dn7 = assign2810_e1922_d_n7;
        locals.var_lod_half_dn10 = assign2810_e1922_d_n10;
        locals.var_lod_half_dn11 = assign2810_e1922_d_n11;
        locals.var_lod_half_dn12 = assign2810_e1922_d_n12;
        locals.var_lod_half_dn17 = assign2810_e1922_d_n17;

        let (assign2820_e1927, assign2820_e1927_d_n0, assign2820_e1927_d_n2, assign2820_e1927_d_n6, assign2820_e1927_d_n7, assign2820_e1927_d_n10, assign2820_e1927_d_n11, assign2820_e1927_d_n12, assign2820_e1927_d_n17,) = {
    if (locals.var_guard15 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn6, locals.var_lod_half_dn7, locals.var_lod_half_dn10, locals.var_lod_half_dn11, locals.var_lod_half_dn12, locals.var_lod_half_dn17,)
    }
};
        locals.var_lod_half = assign2820_e1927;
        locals.var_lod_half_dn0 = assign2820_e1927_d_n0;
        locals.var_lod_half_dn2 = assign2820_e1927_d_n2;
        locals.var_lod_half_dn6 = assign2820_e1927_d_n6;
        locals.var_lod_half_dn7 = assign2820_e1927_d_n7;
        locals.var_lod_half_dn10 = assign2820_e1927_d_n10;
        locals.var_lod_half_dn11 = assign2820_e1927_d_n11;
        locals.var_lod_half_dn12 = assign2820_e1927_d_n12;
        locals.var_lod_half_dn17 = assign2820_e1927_d_n17;

        let assign2830_e1930: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign2830_e1930;

        let (assign2840_e1938, assign2840_e1938_d_n0, assign2840_e1938_d_n2, assign2840_e1938_d_n6, assign2840_e1938_d_n7, assign2840_e1938_d_n10, assign2840_e1938_d_n11, assign2840_e1938_d_n12, assign2840_e1938_d_n17,) = {
    if (locals.var_guard16 != 0.0) {
        let assign2840_e1935: f64 = (1.0 + p.p162);
        let assign2840_e1936: f64 = (1.0 / assign2840_e1935);
        (assign2840_e1936, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign2840_e1938;
        locals.var_t1_dn0 = assign2840_e1938_d_n0;
        locals.var_t1_dn2 = assign2840_e1938_d_n2;
        locals.var_t1_dn6 = assign2840_e1938_d_n6;
        locals.var_t1_dn7 = assign2840_e1938_d_n7;
        locals.var_t1_dn10 = assign2840_e1938_d_n10;
        locals.var_t1_dn11 = assign2840_e1938_d_n11;
        locals.var_t1_dn12 = assign2840_e1938_d_n12;
        locals.var_t1_dn17 = assign2840_e1938_d_n17;

        let (assign2850_e1946, assign2850_e1946_d_n0, assign2850_e1946_d_n2, assign2850_e1946_d_n6, assign2850_e1946_d_n7, assign2850_e1946_d_n10, assign2850_e1946_d_n11, assign2850_e1946_d_n12, assign2850_e1946_d_n17,) = {
    if (locals.var_guard16 != 0.0) {
        let assign2850_e1942: f64 = (p.p161 / locals.var_lod_half);
        let assign2850_e1944: f64 = (assign2850_e1942).powf(p.p163);
        (assign2850_e1944, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign2850_e1942))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign2850_e1942))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign2850_e1942))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))) / assign2850_e1942))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign2850_e1942))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))) / assign2850_e1942))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn12) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn12) / (locals.var_lod_half * locals.var_lod_half))) / assign2850_e1942))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2850_e1942).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn17) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2850_e1944 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn17) / (locals.var_lod_half * locals.var_lod_half))) / assign2850_e1942))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign2850_e1946;
        locals.var_t2_dn0 = assign2850_e1946_d_n0;
        locals.var_t2_dn2 = assign2850_e1946_d_n2;
        locals.var_t2_dn6 = assign2850_e1946_d_n6;
        locals.var_t2_dn7 = assign2850_e1946_d_n7;
        locals.var_t2_dn10 = assign2850_e1946_d_n10;
        locals.var_t2_dn11 = assign2850_e1946_d_n11;
        locals.var_t2_dn12 = assign2850_e1946_d_n12;
        locals.var_t2_dn17 = assign2850_e1946_d_n17;

        let (assign2860_e1954, assign2860_e1954_d_n0, assign2860_e1954_d_n2, assign2860_e1954_d_n6, assign2860_e1954_d_n7, assign2860_e1954_d_n10, assign2860_e1954_d_n11, assign2860_e1954_d_n12, assign2860_e1954_d_n17,) = {
    if (locals.var_guard16 != 0.0) {
        let assign2860_e1950: f64 = (p.p161 / locals.var_lod_half_ref);
        let assign2860_e1952: f64 = (assign2860_e1950).powf(p.p163);
        (assign2860_e1952, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign2860_e1954;
        locals.var_t3_dn0 = assign2860_e1954_d_n0;
        locals.var_t3_dn2 = assign2860_e1954_d_n2;
        locals.var_t3_dn6 = assign2860_e1954_d_n6;
        locals.var_t3_dn7 = assign2860_e1954_d_n7;
        locals.var_t3_dn10 = assign2860_e1954_d_n10;
        locals.var_t3_dn11 = assign2860_e1954_d_n11;
        locals.var_t3_dn12 = assign2860_e1954_d_n12;
        locals.var_t3_dn17 = assign2860_e1954_d_n17;

        let (assign2870_e1970, assign2870_e1970_d_n0, assign2870_e1970_d_n2, assign2870_e1970_d_n6, assign2870_e1970_d_n7, assign2870_e1970_d_n10, assign2870_e1970_d_n11, assign2870_e1970_d_n12, assign2870_e1970_d_n17,) = {
    if (locals.var_guard16 != 0.0) {
        let assign2870_e1960: f64 = (locals.var_t1 * locals.var_t2);
        let assign2870_e1961: f64 = (1.0 + assign2870_e1960);
        let assign2870_e1962: f64 = (locals.var_nsubpp * assign2870_e1961);
        let assign2870_e1966: f64 = (locals.var_t1 * locals.var_t3);
        let assign2870_e1967: f64 = (1.0 + assign2870_e1966);
        let assign2870_e1968: f64 = (assign2870_e1962 / assign2870_e1967);
        (assign2870_e1968, ((((locals.var_nsubpp * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0))) * assign2870_e1967) - (assign2870_e1962 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign2870_e1967 * assign2870_e1967)), ((((locals.var_nsubpp * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2))) * assign2870_e1967) - (assign2870_e1962 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign2870_e1967 * assign2870_e1967)), ((((locals.var_nsubpp * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6))) * assign2870_e1967) - (assign2870_e1962 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign2870_e1967 * assign2870_e1967)), ((((locals.var_nsubpp * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7))) * assign2870_e1967) - (assign2870_e1962 * ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)))) / (assign2870_e1967 * assign2870_e1967)), ((((locals.var_nsubpp * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10))) * assign2870_e1967) - (assign2870_e1962 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign2870_e1967 * assign2870_e1967)), ((((locals.var_nsubpp * ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11))) * assign2870_e1967) - (assign2870_e1962 * ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)))) / (assign2870_e1967 * assign2870_e1967)), ((((locals.var_nsubpp * ((locals.var_t1_dn12 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn12))) * assign2870_e1967) - (assign2870_e1962 * ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)))) / (assign2870_e1967 * assign2870_e1967)), ((((locals.var_nsubpp * ((locals.var_t1_dn17 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn17))) * assign2870_e1967) - (assign2870_e1962 * ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)))) / (assign2870_e1967 * assign2870_e1967)),)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn6, locals.var_nsubps_dn7, locals.var_nsubps_dn10, locals.var_nsubps_dn11, locals.var_nsubps_dn12, locals.var_nsubps_dn17,)
    }
};
        locals.var_nsubps = assign2870_e1970;
        locals.var_nsubps_dn0 = assign2870_e1970_d_n0;
        locals.var_nsubps_dn2 = assign2870_e1970_d_n2;
        locals.var_nsubps_dn6 = assign2870_e1970_d_n6;
        locals.var_nsubps_dn7 = assign2870_e1970_d_n7;
        locals.var_nsubps_dn10 = assign2870_e1970_d_n10;
        locals.var_nsubps_dn11 = assign2870_e1970_d_n11;
        locals.var_nsubps_dn12 = assign2870_e1970_d_n12;
        locals.var_nsubps_dn17 = assign2870_e1970_d_n17;

        let (assign2880_e1975, assign2880_e1975_d_n0, assign2880_e1975_d_n2, assign2880_e1975_d_n6, assign2880_e1975_d_n7, assign2880_e1975_d_n10, assign2880_e1975_d_n11, assign2880_e1975_d_n12, assign2880_e1975_d_n17,) = {
    if (locals.var_guard16 == 0.0) {
        (locals.var_nsubpp, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn6, locals.var_nsubps_dn7, locals.var_nsubps_dn10, locals.var_nsubps_dn11, locals.var_nsubps_dn12, locals.var_nsubps_dn17,)
    }
};
        locals.var_nsubps = assign2880_e1975;
        locals.var_nsubps_dn0 = assign2880_e1975_d_n0;
        locals.var_nsubps_dn2 = assign2880_e1975_d_n2;
        locals.var_nsubps_dn6 = assign2880_e1975_d_n6;
        locals.var_nsubps_dn7 = assign2880_e1975_d_n7;
        locals.var_nsubps_dn10 = assign2880_e1975_d_n10;
        locals.var_nsubps_dn11 = assign2880_e1975_d_n11;
        locals.var_nsubps_dn12 = assign2880_e1975_d_n12;
        locals.var_nsubps_dn17 = assign2880_e1975_d_n17;

        let assign2890_e1980: f64 = (locals.var_wg).powf(p.p200);
        let assign2890_e1981: f64 = (p.p199 / assign2890_e1980);
        let assign2890_e1982: f64 = (1.0 + assign2890_e1981);
        let assign2890_e1987: f64 = (locals.var_lgle).powf(p.p203);
        let assign2890_e1988: f64 = (p.p202 / assign2890_e1987);
        let assign2890_e1989: f64 = (1.0 + assign2890_e1988);
        let assign2890_e1990: f64 = (assign2890_e1982 * assign2890_e1989);
        locals.var_t2 = assign2890_e1990;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn7 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn12 = 0.0;
        locals.var_t2_dn17 = 0.0;

        let assign2900_e1993: f64 = (locals.var_mks_nsubcmax / locals.var_mks_nsubs);
        locals.var_t3 = assign2900_e1993;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn7 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn11 = 0.0;
        locals.var_t3_dn12 = 0.0;
        locals.var_t3_dn17 = 0.0;

        let assign2910_e1996: f64 = (locals.var_t3 - locals.var_t2);
        let assign2910_e1998: f64 = (assign2910_e1996 - 0.01);
        locals.var_tmf1 = assign2910_e1998;
        locals.var_tmf1_dn0 = (locals.var_t3_dn0 - locals.var_t2_dn0);
        locals.var_tmf1_dn2 = (locals.var_t3_dn2 - locals.var_t2_dn2);
        locals.var_tmf1_dn6 = (locals.var_t3_dn6 - locals.var_t2_dn6);
        locals.var_tmf1_dn7 = (locals.var_t3_dn7 - locals.var_t2_dn7);
        locals.var_tmf1_dn10 = (locals.var_t3_dn10 - locals.var_t2_dn10);
        locals.var_tmf1_dn11 = (locals.var_t3_dn11 - locals.var_t2_dn11);
        locals.var_tmf1_dn12 = (locals.var_t3_dn12 - locals.var_t2_dn12);
        locals.var_tmf1_dn17 = (locals.var_t3_dn17 - locals.var_t2_dn17);

        let assign2920_e2001: f64 = (4.0 * locals.var_t3);
        let assign2920_e2003: f64 = (assign2920_e2001 * 0.01);
        locals.var_tmf2 = assign2920_e2003;
        locals.var_tmf2_dn0 = ((4.0 * locals.var_t3_dn0) * 0.01);
        locals.var_tmf2_dn2 = ((4.0 * locals.var_t3_dn2) * 0.01);
        locals.var_tmf2_dn6 = ((4.0 * locals.var_t3_dn6) * 0.01);
        locals.var_tmf2_dn7 = ((4.0 * locals.var_t3_dn7) * 0.01);
        locals.var_tmf2_dn10 = ((4.0 * locals.var_t3_dn10) * 0.01);
        locals.var_tmf2_dn11 = ((4.0 * locals.var_t3_dn11) * 0.01);
        locals.var_tmf2_dn12 = ((4.0 * locals.var_t3_dn12) * 0.01);
        locals.var_tmf2_dn17 = ((4.0 * locals.var_t3_dn17) * 0.01);

        let (assign2930_e2010, assign2930_e2010_d_n0, assign2930_e2010_d_n2, assign2930_e2010_d_n6, assign2930_e2010_d_n7, assign2930_e2010_d_n10, assign2930_e2010_d_n11, assign2930_e2010_d_n12, assign2930_e2010_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign2930_e2009: f64 = (-locals.var_tmf2);
        (assign2930_e2009, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
        locals.var_tmf2 = assign2930_e2010;
        locals.var_tmf2_dn0 = assign2930_e2010_d_n0;
        locals.var_tmf2_dn2 = assign2930_e2010_d_n2;
        locals.var_tmf2_dn6 = assign2930_e2010_d_n6;
        locals.var_tmf2_dn7 = assign2930_e2010_d_n7;
        locals.var_tmf2_dn10 = assign2930_e2010_d_n10;
        locals.var_tmf2_dn11 = assign2930_e2010_d_n11;
        locals.var_tmf2_dn12 = assign2930_e2010_d_n12;
        locals.var_tmf2_dn17 = assign2930_e2010_d_n17;

        let assign2940_e2013: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign2940_e2015: f64 = (assign2940_e2013 + locals.var_tmf2);
        let assign2940_e2016: f64 = (assign2940_e2015).sqrt();
        locals.var_tmf2 = assign2940_e2016;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign2940_e2016));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign2940_e2016));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign2940_e2016));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign2940_e2016));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign2940_e2016));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign2940_e2016));
        locals.var_tmf2_dn12 = ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign2940_e2016));
        locals.var_tmf2_dn17 = ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign2940_e2016));

        let assign2950_e2021: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign2950_e2022: f64 = (0.5 * assign2950_e2021);
        let assign2950_e2023: f64 = (locals.var_t3 - assign2950_e2022);
        locals.var_t1 = assign2950_e2023;
        locals.var_t1_dn0 = (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)));
        locals.var_t1_dn2 = (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)));
        locals.var_t1_dn6 = (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)));
        locals.var_t1_dn7 = (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)));
        locals.var_t1_dn10 = (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)));
        locals.var_t1_dn11 = (locals.var_t3_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)));
        locals.var_t1_dn12 = (locals.var_t3_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12)));
        locals.var_t1_dn17 = (locals.var_t3_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17)));

        let assign2960_e2026: f64 = (locals.var_mks_nsubs * locals.var_t1);
        locals.var_uc_nsubs = assign2960_e2026;
        locals.var_uc_nsubs_dn0 = (locals.var_mks_nsubs * locals.var_t1_dn0);
        locals.var_uc_nsubs_dn2 = (locals.var_mks_nsubs * locals.var_t1_dn2);
        locals.var_uc_nsubs_dn6 = (locals.var_mks_nsubs * locals.var_t1_dn6);
        locals.var_uc_nsubs_dn7 = (locals.var_mks_nsubs * locals.var_t1_dn7);
        locals.var_uc_nsubs_dn10 = (locals.var_mks_nsubs * locals.var_t1_dn10);
        locals.var_uc_nsubs_dn11 = (locals.var_mks_nsubs * locals.var_t1_dn11);
        locals.var_uc_nsubs_dn12 = (locals.var_mks_nsubs * locals.var_t1_dn12);
        locals.var_uc_nsubs_dn17 = (locals.var_mks_nsubs * locals.var_t1_dn17);

        let assign2970_e2029: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign2970_e2029;

        let (assign2980_e2037, assign2980_e2037_d_n0, assign2980_e2037_d_n2, assign2980_e2037_d_n6, assign2980_e2037_d_n7, assign2980_e2037_d_n10, assign2980_e2037_d_n11, assign2980_e2037_d_n12, assign2980_e2037_d_n17,) = {
    if (locals.var_guard17 != 0.0) {
        let assign2980_e2034: f64 = (1.0 + p.p165);
        let assign2980_e2035: f64 = (1.0 / assign2980_e2034);
        (assign2980_e2035, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign2980_e2037;
        locals.var_t1_dn0 = assign2980_e2037_d_n0;
        locals.var_t1_dn2 = assign2980_e2037_d_n2;
        locals.var_t1_dn6 = assign2980_e2037_d_n6;
        locals.var_t1_dn7 = assign2980_e2037_d_n7;
        locals.var_t1_dn10 = assign2980_e2037_d_n10;
        locals.var_t1_dn11 = assign2980_e2037_d_n11;
        locals.var_t1_dn12 = assign2980_e2037_d_n12;
        locals.var_t1_dn17 = assign2980_e2037_d_n17;

        let (assign2990_e2045, assign2990_e2045_d_n0, assign2990_e2045_d_n2, assign2990_e2045_d_n6, assign2990_e2045_d_n7, assign2990_e2045_d_n10, assign2990_e2045_d_n11, assign2990_e2045_d_n12, assign2990_e2045_d_n17,) = {
    if (locals.var_guard17 != 0.0) {
        let assign2990_e2041: f64 = (p.p164 / locals.var_lod_half);
        let assign2990_e2043: f64 = (assign2990_e2041).powf(p.p166);
        (assign2990_e2043, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign2990_e2041))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign2990_e2041))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign2990_e2041))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))) / assign2990_e2041))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign2990_e2041))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))) / assign2990_e2041))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn12) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn12) / (locals.var_lod_half * locals.var_lod_half))) / assign2990_e2041))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2990_e2041).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn17) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2990_e2043 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn17) / (locals.var_lod_half * locals.var_lod_half))) / assign2990_e2041))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign2990_e2045;
        locals.var_t2_dn0 = assign2990_e2045_d_n0;
        locals.var_t2_dn2 = assign2990_e2045_d_n2;
        locals.var_t2_dn6 = assign2990_e2045_d_n6;
        locals.var_t2_dn7 = assign2990_e2045_d_n7;
        locals.var_t2_dn10 = assign2990_e2045_d_n10;
        locals.var_t2_dn11 = assign2990_e2045_d_n11;
        locals.var_t2_dn12 = assign2990_e2045_d_n12;
        locals.var_t2_dn17 = assign2990_e2045_d_n17;

        let (assign3000_e2053, assign3000_e2053_d_n0, assign3000_e2053_d_n2, assign3000_e2053_d_n6, assign3000_e2053_d_n7, assign3000_e2053_d_n10, assign3000_e2053_d_n11, assign3000_e2053_d_n12, assign3000_e2053_d_n17,) = {
    if (locals.var_guard17 != 0.0) {
        let assign3000_e2049: f64 = (p.p164 / locals.var_lod_half_ref);
        let assign3000_e2051: f64 = (assign3000_e2049).powf(p.p166);
        (assign3000_e2051, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign3000_e2053;
        locals.var_t3_dn0 = assign3000_e2053_d_n0;
        locals.var_t3_dn2 = assign3000_e2053_d_n2;
        locals.var_t3_dn6 = assign3000_e2053_d_n6;
        locals.var_t3_dn7 = assign3000_e2053_d_n7;
        locals.var_t3_dn10 = assign3000_e2053_d_n10;
        locals.var_t3_dn11 = assign3000_e2053_d_n11;
        locals.var_t3_dn12 = assign3000_e2053_d_n12;
        locals.var_t3_dn17 = assign3000_e2053_d_n17;

        let (assign3010_e2069, assign3010_e2069_d_n0, assign3010_e2069_d_n2, assign3010_e2069_d_n6, assign3010_e2069_d_n7, assign3010_e2069_d_n10, assign3010_e2069_d_n11, assign3010_e2069_d_n12, assign3010_e2069_d_n17,) = {
    if (locals.var_guard17 != 0.0) {
        let assign3010_e2059: f64 = (locals.var_t1 * locals.var_t2);
        let assign3010_e2060: f64 = (1.0 + assign3010_e2059);
        let assign3010_e2061: f64 = (locals.var_uc_nsubs * assign3010_e2060);
        let assign3010_e2065: f64 = (locals.var_t1 * locals.var_t3);
        let assign3010_e2066: f64 = (1.0 + assign3010_e2065);
        let assign3010_e2067: f64 = (assign3010_e2061 / assign3010_e2066);
        (assign3010_e2067, (((((locals.var_uc_nsubs_dn0 * assign3010_e2060) + (locals.var_uc_nsubs * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)))) * assign3010_e2066) - (assign3010_e2061 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign3010_e2066 * assign3010_e2066)), (((((locals.var_uc_nsubs_dn2 * assign3010_e2060) + (locals.var_uc_nsubs * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)))) * assign3010_e2066) - (assign3010_e2061 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign3010_e2066 * assign3010_e2066)), (((((locals.var_uc_nsubs_dn6 * assign3010_e2060) + (locals.var_uc_nsubs * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * assign3010_e2066) - (assign3010_e2061 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign3010_e2066 * assign3010_e2066)), (((((locals.var_uc_nsubs_dn7 * assign3010_e2060) + (locals.var_uc_nsubs * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * assign3010_e2066) - (assign3010_e2061 * ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)))) / (assign3010_e2066 * assign3010_e2066)), (((((locals.var_uc_nsubs_dn10 * assign3010_e2060) + (locals.var_uc_nsubs * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)))) * assign3010_e2066) - (assign3010_e2061 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign3010_e2066 * assign3010_e2066)), (((((locals.var_uc_nsubs_dn11 * assign3010_e2060) + (locals.var_uc_nsubs * ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)))) * assign3010_e2066) - (assign3010_e2061 * ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)))) / (assign3010_e2066 * assign3010_e2066)), (((((locals.var_uc_nsubs_dn12 * assign3010_e2060) + (locals.var_uc_nsubs * ((locals.var_t1_dn12 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn12)))) * assign3010_e2066) - (assign3010_e2061 * ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)))) / (assign3010_e2066 * assign3010_e2066)), (((((locals.var_uc_nsubs_dn17 * assign3010_e2060) + (locals.var_uc_nsubs * ((locals.var_t1_dn17 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn17)))) * assign3010_e2066) - (assign3010_e2061 * ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)))) / (assign3010_e2066 * assign3010_e2066)),)
    } else {
        (locals.var_uc_nsubs, locals.var_uc_nsubs_dn0, locals.var_uc_nsubs_dn2, locals.var_uc_nsubs_dn6, locals.var_uc_nsubs_dn7, locals.var_uc_nsubs_dn10, locals.var_uc_nsubs_dn11, locals.var_uc_nsubs_dn12, locals.var_uc_nsubs_dn17,)
    }
};
        locals.var_uc_nsubs = assign3010_e2069;
        locals.var_uc_nsubs_dn0 = assign3010_e2069_d_n0;
        locals.var_uc_nsubs_dn2 = assign3010_e2069_d_n2;
        locals.var_uc_nsubs_dn6 = assign3010_e2069_d_n6;
        locals.var_uc_nsubs_dn7 = assign3010_e2069_d_n7;
        locals.var_uc_nsubs_dn10 = assign3010_e2069_d_n10;
        locals.var_uc_nsubs_dn11 = assign3010_e2069_d_n11;
        locals.var_uc_nsubs_dn12 = assign3010_e2069_d_n12;
        locals.var_uc_nsubs_dn17 = assign3010_e2069_d_n17;

        let assign3020_e2076: f64 = if ((locals.var_lgleff > p.p72) || (p.p72 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard18 = assign3020_e2076;

        let (assign3030_e2090, assign3030_e2090_d_n0, assign3030_e2090_d_n2, assign3030_e2090_d_n6, assign3030_e2090_d_n7, assign3030_e2090_d_n10, assign3030_e2090_d_n11, assign3030_e2090_d_n12, assign3030_e2090_d_n17,) = {
    if (locals.var_guard18 != 0.0) {
        let assign3030_e2081: f64 = (locals.var_lgleff - p.p72);
        let assign3030_e2082: f64 = (locals.var_uc_nsubs * assign3030_e2081);
        let assign3030_e2085: f64 = (locals.var_nsubps * p.p72);
        let assign3030_e2086: f64 = (assign3030_e2082 + assign3030_e2085);
        let assign3030_e2088: f64 = (assign3030_e2086 / locals.var_lgleff);
        (assign3030_e2088, (((locals.var_uc_nsubs_dn0 * assign3030_e2081) + (locals.var_nsubps_dn0 * p.p72)) / locals.var_lgleff), (((locals.var_uc_nsubs_dn2 * assign3030_e2081) + (locals.var_nsubps_dn2 * p.p72)) / locals.var_lgleff), (((locals.var_uc_nsubs_dn6 * assign3030_e2081) + (locals.var_nsubps_dn6 * p.p72)) / locals.var_lgleff), (((locals.var_uc_nsubs_dn7 * assign3030_e2081) + (locals.var_nsubps_dn7 * p.p72)) / locals.var_lgleff), (((locals.var_uc_nsubs_dn10 * assign3030_e2081) + (locals.var_nsubps_dn10 * p.p72)) / locals.var_lgleff), (((locals.var_uc_nsubs_dn11 * assign3030_e2081) + (locals.var_nsubps_dn11 * p.p72)) / locals.var_lgleff), (((locals.var_uc_nsubs_dn12 * assign3030_e2081) + (locals.var_nsubps_dn12 * p.p72)) / locals.var_lgleff), (((locals.var_uc_nsubs_dn17 * assign3030_e2081) + (locals.var_nsubps_dn17 * p.p72)) / locals.var_lgleff),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn10, locals.var_nsub_dn11, locals.var_nsub_dn12, locals.var_nsub_dn17,)
    }
};
        locals.var_nsub = assign3030_e2090;
        locals.var_nsub_dn0 = assign3030_e2090_d_n0;
        locals.var_nsub_dn2 = assign3030_e2090_d_n2;
        locals.var_nsub_dn6 = assign3030_e2090_d_n6;
        locals.var_nsub_dn7 = assign3030_e2090_d_n7;
        locals.var_nsub_dn10 = assign3030_e2090_d_n10;
        locals.var_nsub_dn11 = assign3030_e2090_d_n11;
        locals.var_nsub_dn12 = assign3030_e2090_d_n12;
        locals.var_nsub_dn17 = assign3030_e2090_d_n17;

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3040_e2105, assign3040_e2105_d_n0, assign3040_e2105_d_n2, assign3040_e2105_d_n6, assign3040_e2105_d_n7, assign3040_e2105_d_n10, assign3040_e2105_d_n11, assign3040_e2105_d_n12, assign3040_e2105_d_n17,) = {
    if (locals.var_guard18 == 0.0) {
        let assign3040_e2096: f64 = (locals.var_nsubps - locals.var_uc_nsubs);
        let assign3040_e2099: f64 = (p.p72 - locals.var_lgleff);
        let assign3040_e2100: f64 = (assign3040_e2096 * assign3040_e2099);
        let assign3040_e2102: f64 = (assign3040_e2100 / p.p72);
        let assign3040_e2103: f64 = (locals.var_nsubps + assign3040_e2102);
        (assign3040_e2103, (locals.var_nsubps_dn0 + (((locals.var_nsubps_dn0 - locals.var_uc_nsubs_dn0) * assign3040_e2099) / p.p72)), (locals.var_nsubps_dn2 + (((locals.var_nsubps_dn2 - locals.var_uc_nsubs_dn2) * assign3040_e2099) / p.p72)), (locals.var_nsubps_dn6 + (((locals.var_nsubps_dn6 - locals.var_uc_nsubs_dn6) * assign3040_e2099) / p.p72)), (locals.var_nsubps_dn7 + (((locals.var_nsubps_dn7 - locals.var_uc_nsubs_dn7) * assign3040_e2099) / p.p72)), (locals.var_nsubps_dn10 + (((locals.var_nsubps_dn10 - locals.var_uc_nsubs_dn10) * assign3040_e2099) / p.p72)), (locals.var_nsubps_dn11 + (((locals.var_nsubps_dn11 - locals.var_uc_nsubs_dn11) * assign3040_e2099) / p.p72)), (locals.var_nsubps_dn12 + (((locals.var_nsubps_dn12 - locals.var_uc_nsubs_dn12) * assign3040_e2099) / p.p72)), (locals.var_nsubps_dn17 + (((locals.var_nsubps_dn17 - locals.var_uc_nsubs_dn17) * assign3040_e2099) / p.p72)),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn10, locals.var_nsub_dn11, locals.var_nsub_dn12, locals.var_nsub_dn17,)
    }
};
        locals.var_nsub = assign3040_e2105;
        locals.var_nsub_dn0 = assign3040_e2105_d_n0;
        locals.var_nsub_dn2 = assign3040_e2105_d_n2;
        locals.var_nsub_dn6 = assign3040_e2105_d_n6;
        locals.var_nsub_dn7 = assign3040_e2105_d_n7;
        locals.var_nsub_dn10 = assign3040_e2105_d_n10;
        locals.var_nsub_dn11 = assign3040_e2105_d_n11;
        locals.var_nsub_dn12 = assign3040_e2105_d_n12;
        locals.var_nsub_dn17 = assign3040_e2105_d_n17;

        let assign3050_e2108: f64 = (1.6021918e-19 * locals.var_nsub);
        locals.var_q_nsub = assign3050_e2108;
        locals.var_q_nsub_dn0 = (1.6021918e-19 * locals.var_nsub_dn0);
        locals.var_q_nsub_dn2 = (1.6021918e-19 * locals.var_nsub_dn2);
        locals.var_q_nsub_dn6 = (1.6021918e-19 * locals.var_nsub_dn6);
        locals.var_q_nsub_dn7 = (1.6021918e-19 * locals.var_nsub_dn7);
        locals.var_q_nsub_dn10 = (1.6021918e-19 * locals.var_nsub_dn10);
        locals.var_q_nsub_dn11 = (1.6021918e-19 * locals.var_nsub_dn11);
        locals.var_q_nsub_dn12 = (1.6021918e-19 * locals.var_nsub_dn12);
        locals.var_q_nsub_dn17 = (1.6021918e-19 * locals.var_nsub_dn17);

        let assign3060_e2111: f64 = (locals.var_q_nsub * 1.034943e-10);
        locals.var_qnsub_esi = assign3060_e2111;
        locals.var_qnsub_esi_dn0 = (locals.var_q_nsub_dn0 * 1.034943e-10);
        locals.var_qnsub_esi_dn2 = (locals.var_q_nsub_dn2 * 1.034943e-10);
        locals.var_qnsub_esi_dn6 = (locals.var_q_nsub_dn6 * 1.034943e-10);
        locals.var_qnsub_esi_dn7 = (locals.var_q_nsub_dn7 * 1.034943e-10);
        locals.var_qnsub_esi_dn10 = (locals.var_q_nsub_dn10 * 1.034943e-10);
        locals.var_qnsub_esi_dn11 = (locals.var_q_nsub_dn11 * 1.034943e-10);
        locals.var_qnsub_esi_dn12 = (locals.var_q_nsub_dn12 * 1.034943e-10);
        locals.var_qnsub_esi_dn17 = (locals.var_q_nsub_dn17 * 1.034943e-10);

        let assign3070_e2114: f64 = (2.0 * locals.var_qnsub_esi);
        locals.var_qnsub_esi2 = assign3070_e2114;
        locals.var_qnsub_esi2_dn0 = (2.0 * locals.var_qnsub_esi_dn0);
        locals.var_qnsub_esi2_dn2 = (2.0 * locals.var_qnsub_esi_dn2);
        locals.var_qnsub_esi2_dn6 = (2.0 * locals.var_qnsub_esi_dn6);
        locals.var_qnsub_esi2_dn7 = (2.0 * locals.var_qnsub_esi_dn7);
        locals.var_qnsub_esi2_dn10 = (2.0 * locals.var_qnsub_esi_dn10);
        locals.var_qnsub_esi2_dn11 = (2.0 * locals.var_qnsub_esi_dn11);
        locals.var_qnsub_esi2_dn12 = (2.0 * locals.var_qnsub_esi_dn12);
        locals.var_qnsub_esi2_dn17 = (2.0 * locals.var_qnsub_esi_dn17);

        let assign3080_e2118: f64 = (2.0 * p.p72);
        let assign3080_e2123: f64 = if ((locals.var_lgleff <= assign3080_e2118) && (p.p72 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard19 = assign3080_e2123;

        let (assign3090_e2139, assign3090_e2139_d_n0, assign3090_e2139_d_n2, assign3090_e2139_d_n6, assign3090_e2139_d_n7, assign3090_e2139_d_n10, assign3090_e2139_d_n11, assign3090_e2139_d_n12, assign3090_e2139_d_n17,) = {
    if (locals.var_guard19 != 0.0) {
        let assign3090_e2127: f64 = (2.0 * locals.var_nsubps);
        let assign3090_e2130: f64 = (locals.var_nsubps - locals.var_uc_nsubs);
        let assign3090_e2132: f64 = (assign3090_e2130 * locals.var_lgleff);
        let assign3090_e2134: f64 = (assign3090_e2132 / p.p72);
        let assign3090_e2135: f64 = (assign3090_e2127 - assign3090_e2134);
        let assign3090_e2137: f64 = (assign3090_e2135 - locals.var_uc_nsubs);
        (assign3090_e2137, (((2.0 * locals.var_nsubps_dn0) - (((locals.var_nsubps_dn0 - locals.var_uc_nsubs_dn0) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn0), (((2.0 * locals.var_nsubps_dn2) - (((locals.var_nsubps_dn2 - locals.var_uc_nsubs_dn2) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn2), (((2.0 * locals.var_nsubps_dn6) - (((locals.var_nsubps_dn6 - locals.var_uc_nsubs_dn6) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn6), (((2.0 * locals.var_nsubps_dn7) - (((locals.var_nsubps_dn7 - locals.var_uc_nsubs_dn7) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn7), (((2.0 * locals.var_nsubps_dn10) - (((locals.var_nsubps_dn10 - locals.var_uc_nsubs_dn10) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn10), (((2.0 * locals.var_nsubps_dn11) - (((locals.var_nsubps_dn11 - locals.var_uc_nsubs_dn11) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn11), (((2.0 * locals.var_nsubps_dn12) - (((locals.var_nsubps_dn12 - locals.var_uc_nsubs_dn12) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn12), (((2.0 * locals.var_nsubps_dn17) - (((locals.var_nsubps_dn17 - locals.var_uc_nsubs_dn17) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn17),)
    } else {
        (locals.var_nsubb0, locals.var_nsubb0_dn0, locals.var_nsubb0_dn2, locals.var_nsubb0_dn6, locals.var_nsubb0_dn7, locals.var_nsubb0_dn10, locals.var_nsubb0_dn11, locals.var_nsubb0_dn12, locals.var_nsubb0_dn17,)
    }
};
        locals.var_nsubb0 = assign3090_e2139;
        locals.var_nsubb0_dn0 = assign3090_e2139_d_n0;
        locals.var_nsubb0_dn2 = assign3090_e2139_d_n2;
        locals.var_nsubb0_dn6 = assign3090_e2139_d_n6;
        locals.var_nsubb0_dn7 = assign3090_e2139_d_n7;
        locals.var_nsubb0_dn10 = assign3090_e2139_d_n10;
        locals.var_nsubb0_dn11 = assign3090_e2139_d_n11;
        locals.var_nsubb0_dn12 = assign3090_e2139_d_n12;
        locals.var_nsubb0_dn17 = assign3090_e2139_d_n17;

        let (assign3100_e2146, assign3100_e2146_d_n0, assign3100_e2146_d_n2, assign3100_e2146_d_n6, assign3100_e2146_d_n7, assign3100_e2146_d_n10, assign3100_e2146_d_n11, assign3100_e2146_d_n12, assign3100_e2146_d_n17,) = {
    if (locals.var_guard19 != 0.0) {
        let assign3100_e2143: f64 = (locals.var_nsubb0 / locals.var_uc_nsubs);
        let assign3100_e2144: f64 = (assign3100_e2143).ln();
        (assign3100_e2144, ((((locals.var_nsubb0_dn0 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3100_e2143), ((((locals.var_nsubb0_dn2 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3100_e2143), ((((locals.var_nsubb0_dn6 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3100_e2143), ((((locals.var_nsubb0_dn7 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3100_e2143), ((((locals.var_nsubb0_dn10 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3100_e2143), ((((locals.var_nsubb0_dn11 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3100_e2143), ((((locals.var_nsubb0_dn12 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3100_e2143), ((((locals.var_nsubb0_dn17 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3100_e2143),)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn6, locals.var_ptovr0_dn7, locals.var_ptovr0_dn10, locals.var_ptovr0_dn11, locals.var_ptovr0_dn12, locals.var_ptovr0_dn17,)
    }
};
        locals.var_ptovr0 = assign3100_e2146;
        locals.var_ptovr0_dn0 = assign3100_e2146_d_n0;
        locals.var_ptovr0_dn2 = assign3100_e2146_d_n2;
        locals.var_ptovr0_dn6 = assign3100_e2146_d_n6;
        locals.var_ptovr0_dn7 = assign3100_e2146_d_n7;
        locals.var_ptovr0_dn10 = assign3100_e2146_d_n10;
        locals.var_ptovr0_dn11 = assign3100_e2146_d_n11;
        locals.var_ptovr0_dn12 = assign3100_e2146_d_n12;
        locals.var_ptovr0_dn17 = assign3100_e2146_d_n17;

        let (assign3110_e2151, assign3110_e2151_d_n0, assign3110_e2151_d_n2, assign3110_e2151_d_n6, assign3110_e2151_d_n7, assign3110_e2151_d_n10, assign3110_e2151_d_n11, assign3110_e2151_d_n12, assign3110_e2151_d_n17,) = {
    if (locals.var_guard19 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn6, locals.var_ptovr0_dn7, locals.var_ptovr0_dn10, locals.var_ptovr0_dn11, locals.var_ptovr0_dn12, locals.var_ptovr0_dn17,)
    }
};
        locals.var_ptovr0 = assign3110_e2151;
        locals.var_ptovr0_dn0 = assign3110_e2151_d_n0;
        locals.var_ptovr0_dn2 = assign3110_e2151_d_n2;
        locals.var_ptovr0_dn6 = assign3110_e2151_d_n6;
        locals.var_ptovr0_dn7 = assign3110_e2151_d_n7;
        locals.var_ptovr0_dn10 = assign3110_e2151_d_n10;
        locals.var_ptovr0_dn11 = assign3110_e2151_d_n11;
        locals.var_ptovr0_dn12 = assign3110_e2151_d_n12;
        locals.var_ptovr0_dn17 = assign3110_e2151_d_n17;

        let assign3120_e2154: f64 = (2.0 / 38.68283);
        let assign3120_e2158: f64 = (10400000000.0 / 1e-6);
        let assign3120_e2159: f64 = (locals.var_nsub / assign3120_e2158);
        let assign3120_e2160: f64 = (assign3120_e2159).ln();
        let assign3120_e2161: f64 = (assign3120_e2154 * assign3120_e2160);
        locals.var_pb20 = assign3120_e2161;
        locals.var_pb20_dn0 = (assign3120_e2154 * ((locals.var_nsub_dn0 / assign3120_e2158) / assign3120_e2159));
        locals.var_pb20_dn2 = (assign3120_e2154 * ((locals.var_nsub_dn2 / assign3120_e2158) / assign3120_e2159));
        locals.var_pb20_dn6 = (assign3120_e2154 * ((locals.var_nsub_dn6 / assign3120_e2158) / assign3120_e2159));
        locals.var_pb20_dn7 = (assign3120_e2154 * ((locals.var_nsub_dn7 / assign3120_e2158) / assign3120_e2159));
        locals.var_pb20_dn10 = (assign3120_e2154 * ((locals.var_nsub_dn10 / assign3120_e2158) / assign3120_e2159));
        locals.var_pb20_dn11 = (assign3120_e2154 * ((locals.var_nsub_dn11 / assign3120_e2158) / assign3120_e2159));
        locals.var_pb20_dn12 = (assign3120_e2154 * ((locals.var_nsub_dn12 / assign3120_e2158) / assign3120_e2159));
        locals.var_pb20_dn17 = (assign3120_e2154 * ((locals.var_nsub_dn17 / assign3120_e2158) / assign3120_e2159));

        let assign3130_e2164: f64 = (2.0 / 38.68283);
        let assign3130_e2168: f64 = (10400000000.0 / 1e-6);
        let assign3130_e2169: f64 = (locals.var_uc_nsubs / assign3130_e2168);
        let assign3130_e2170: f64 = (assign3130_e2169).ln();
        let assign3130_e2171: f64 = (assign3130_e2164 * assign3130_e2170);
        locals.var_pb2c = assign3130_e2171;
        locals.var_pb2c_dn0 = (assign3130_e2164 * ((locals.var_uc_nsubs_dn0 / assign3130_e2168) / assign3130_e2169));
        locals.var_pb2c_dn2 = (assign3130_e2164 * ((locals.var_uc_nsubs_dn2 / assign3130_e2168) / assign3130_e2169));
        locals.var_pb2c_dn6 = (assign3130_e2164 * ((locals.var_uc_nsubs_dn6 / assign3130_e2168) / assign3130_e2169));
        locals.var_pb2c_dn7 = (assign3130_e2164 * ((locals.var_uc_nsubs_dn7 / assign3130_e2168) / assign3130_e2169));
        locals.var_pb2c_dn10 = (assign3130_e2164 * ((locals.var_uc_nsubs_dn10 / assign3130_e2168) / assign3130_e2169));
        locals.var_pb2c_dn11 = (assign3130_e2164 * ((locals.var_uc_nsubs_dn11 / assign3130_e2168) / assign3130_e2169));
        locals.var_pb2c_dn12 = (assign3130_e2164 * ((locals.var_uc_nsubs_dn12 / assign3130_e2168) / assign3130_e2169));
        locals.var_pb2c_dn17 = (assign3130_e2164 * ((locals.var_uc_nsubs_dn17 / assign3130_e2168) / assign3130_e2169));

        let assign3140_e2174: f64 = (2.0 * 1.034943e-10);
        let assign3140_e2176: f64 = (assign3140_e2174 / 1.6021918e-19);
        let assign3140_e2178: f64 = (assign3140_e2176 / locals.var_nsub);
        let assign3140_e2179: f64 = (assign3140_e2178).sqrt();
        locals.var_wdpl = assign3140_e2179;
        locals.var_wdpl_dn0 = ((-((assign3140_e2176 * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3140_e2179));
        locals.var_wdpl_dn2 = ((-((assign3140_e2176 * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3140_e2179));
        locals.var_wdpl_dn6 = ((-((assign3140_e2176 * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3140_e2179));
        locals.var_wdpl_dn7 = ((-((assign3140_e2176 * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3140_e2179));
        locals.var_wdpl_dn10 = ((-((assign3140_e2176 * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3140_e2179));
        locals.var_wdpl_dn11 = ((-((assign3140_e2176 * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3140_e2179));
        locals.var_wdpl_dn12 = ((-((assign3140_e2176 * locals.var_nsub_dn12) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3140_e2179));
        locals.var_wdpl_dn17 = ((-((assign3140_e2176 * locals.var_nsub_dn17) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3140_e2179));

        let assign3150_e2184: f64 = (locals.var_lgle).powf(p.p195);
        let assign3150_e2185: f64 = (p.p194 / assign3150_e2184);
        let assign3150_e2186: f64 = (1.0 + assign3150_e2185);
        let assign3150_e2191: f64 = (locals.var_wl).powf(p.p197);
        let assign3150_e2192: f64 = (p.p196 / assign3150_e2191);
        let assign3150_e2193: f64 = (1.0 + assign3150_e2192);
        let assign3150_e2194: f64 = (assign3150_e2186 * assign3150_e2193);
        locals.var_t1 = assign3150_e2194;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn17 = 0.0;

        let assign3160_e2197: f64 = (locals.var_t1 * locals.var_t1);
        let assign3160_e2200: f64 = (4.0 * 0.001);
        let assign3160_e2202: f64 = (assign3160_e2200 * 0.001);
        let assign3160_e2203: f64 = (assign3160_e2197 + assign3160_e2202);
        let assign3160_e2204: f64 = (assign3160_e2203).sqrt();
        locals.var_tmf1 = assign3160_e2204;
        locals.var_tmf1_dn0 = (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign3160_e2204));
        locals.var_tmf1_dn2 = (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign3160_e2204));
        locals.var_tmf1_dn6 = (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign3160_e2204));
        locals.var_tmf1_dn7 = (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign3160_e2204));
        locals.var_tmf1_dn10 = (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign3160_e2204));
        locals.var_tmf1_dn11 = (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign3160_e2204));
        locals.var_tmf1_dn12 = (((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) / (2.0 * assign3160_e2204));
        locals.var_tmf1_dn17 = (((locals.var_t1_dn17 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn17)) / (2.0 * assign3160_e2204));

        let assign3170_e2208: f64 = (locals.var_t1 + locals.var_tmf1);
        let assign3170_e2209: f64 = (0.5 * assign3170_e2208);
        let assign3170_e2212: f64 = (1e-10 * 0.001);
        let assign3170_e2213: f64 = (assign3170_e2209 + assign3170_e2212);
        locals.var_vmax0 = assign3170_e2213;
        locals.var_vmax0_dn0 = (0.5 * (locals.var_t1_dn0 + locals.var_tmf1_dn0));
        locals.var_vmax0_dn2 = (0.5 * (locals.var_t1_dn2 + locals.var_tmf1_dn2));
        locals.var_vmax0_dn6 = (0.5 * (locals.var_t1_dn6 + locals.var_tmf1_dn6));
        locals.var_vmax0_dn7 = (0.5 * (locals.var_t1_dn7 + locals.var_tmf1_dn7));
        locals.var_vmax0_dn10 = (0.5 * (locals.var_t1_dn10 + locals.var_tmf1_dn10));
        locals.var_vmax0_dn11 = (0.5 * (locals.var_t1_dn11 + locals.var_tmf1_dn11));
        locals.var_vmax0_dn12 = (0.5 * (locals.var_t1_dn12 + locals.var_tmf1_dn12));
        locals.var_vmax0_dn17 = (0.5 * (locals.var_t1_dn17 + locals.var_tmf1_dn17));

        let assign3180_e2216: f64 = if locals.var_vmax0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign3180_e2216;

        let (assign3190_e2220, assign3190_e2220_d_n0, assign3190_e2220_d_n2, assign3190_e2220_d_n6, assign3190_e2220_d_n7, assign3190_e2220_d_n10, assign3190_e2220_d_n11, assign3190_e2220_d_n12, assign3190_e2220_d_n17,) = {
    if (locals.var_guard20 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vmax0, locals.var_vmax0_dn0, locals.var_vmax0_dn2, locals.var_vmax0_dn6, locals.var_vmax0_dn7, locals.var_vmax0_dn10, locals.var_vmax0_dn11, locals.var_vmax0_dn12, locals.var_vmax0_dn17,)
    }
};
        locals.var_vmax0 = assign3190_e2220;
        locals.var_vmax0_dn0 = assign3190_e2220_d_n0;
        locals.var_vmax0_dn2 = assign3190_e2220_d_n2;
        locals.var_vmax0_dn6 = assign3190_e2220_d_n6;
        locals.var_vmax0_dn7 = assign3190_e2220_d_n7;
        locals.var_vmax0_dn10 = assign3190_e2220_d_n10;
        locals.var_vmax0_dn11 = assign3190_e2220_d_n11;
        locals.var_vmax0_dn12 = assign3190_e2220_d_n12;
        locals.var_vmax0_dn17 = assign3190_e2220_d_n17;

        let assign3200_e2223: f64 = if p.p35 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign3200_e2223;

        let assign3210_e2226: f64 = if locals.var_grg_cnst > 0.001 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign3210_e2226;

        let (assign3220_e2234,) = {
    if ((locals.var_guard21 != 0.0) && (locals.var_guard22 != 0.0)) {
        let assign3220_e2232: f64 = (locals.var_mfactor / locals.var_grg_cnst);
        (assign3220_e2232,)
    } else {
        (locals.var_grg,)
    }
};
        locals.var_grg = assign3220_e2234;

        let (assign3230_e2243,) = {
    if ((locals.var_guard21 != 0.0) && (locals.var_guard22 == 0.0)) {
        let assign3230_e2241: f64 = (locals.var_mfactor * 1000.0);
        (assign3230_e2241,)
    } else {
        (locals.var_grg,)
    }
};
        locals.var_grg = assign3230_e2243;

        let (assign3240_e2250,) = {
    if (locals.var_guard21 == 0.0) {
        let assign3240_e2248: f64 = (locals.var_mfactor * 1000.0);
        (assign3240_e2248,)
    } else {
        (locals.var_grg,)
    }
};
        locals.var_grg = assign3240_e2250;

        let assign3250_e2253: f64 = if p.p261 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard23 = assign3250_e2253;

        let (assign3260_e2261, assign3260_e2261_d_n0, assign3260_e2261_d_n2, assign3260_e2261_d_n6, assign3260_e2261_d_n7, assign3260_e2261_d_n10, assign3260_e2261_d_n11, assign3260_e2261_d_n12, assign3260_e2261_d_n17,) = {
    if (locals.var_guard23 != 0.0) {
        let assign3260_e2257: f64 = (p.p289 * locals.var_weff_nf);
        let assign3260_e2259: f64 = (assign3260_e2257 + p.p288);
        (assign3260_e2259, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign3260_e2261;
        locals.var_t0_dn0 = assign3260_e2261_d_n0;
        locals.var_t0_dn2 = assign3260_e2261_d_n2;
        locals.var_t0_dn6 = assign3260_e2261_d_n6;
        locals.var_t0_dn7 = assign3260_e2261_d_n7;
        locals.var_t0_dn10 = assign3260_e2261_d_n10;
        locals.var_t0_dn11 = assign3260_e2261_d_n11;
        locals.var_t0_dn12 = assign3260_e2261_d_n12;
        locals.var_t0_dn17 = assign3260_e2261_d_n17;

        let (assign3270_e2267, assign3270_e2267_d_n0, assign3270_e2267_d_n2, assign3270_e2267_d_n6, assign3270_e2267_d_n7, assign3270_e2267_d_n10, assign3270_e2267_d_n11, assign3270_e2267_d_n12, assign3270_e2267_d_n17,) = {
    if (locals.var_guard23 != 0.0) {
        let assign3270_e2265: f64 = (locals.var_t0 / locals.var_mfactor);
        (assign3270_e2265, (locals.var_t0_dn0 / locals.var_mfactor), (locals.var_t0_dn2 / locals.var_mfactor), (locals.var_t0_dn6 / locals.var_mfactor), (locals.var_t0_dn7 / locals.var_mfactor), (locals.var_t0_dn10 / locals.var_mfactor), (locals.var_t0_dn11 / locals.var_mfactor), (locals.var_t0_dn12 / locals.var_mfactor), (locals.var_t0_dn17 / locals.var_mfactor),)
    } else {
        (locals.var_rbulk, locals.var_rbulk_dn0, locals.var_rbulk_dn2, locals.var_rbulk_dn6, locals.var_rbulk_dn7, locals.var_rbulk_dn10, locals.var_rbulk_dn11, locals.var_rbulk_dn12, locals.var_rbulk_dn17,)
    }
};
        locals.var_rbulk = assign3270_e2267;
        locals.var_rbulk_dn0 = assign3270_e2267_d_n0;
        locals.var_rbulk_dn2 = assign3270_e2267_d_n2;
        locals.var_rbulk_dn6 = assign3270_e2267_d_n6;
        locals.var_rbulk_dn7 = assign3270_e2267_d_n7;
        locals.var_rbulk_dn10 = assign3270_e2267_d_n10;
        locals.var_rbulk_dn11 = assign3270_e2267_d_n11;
        locals.var_rbulk_dn12 = assign3270_e2267_d_n12;
        locals.var_rbulk_dn17 = assign3270_e2267_d_n17;

        let assign3280_e2270: f64 = if locals.var_rbulk < 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard24 = assign3280_e2270;

        let (assign3290_e2276, assign3290_e2276_d_n0, assign3290_e2276_d_n2, assign3290_e2276_d_n6, assign3290_e2276_d_n7, assign3290_e2276_d_n10, assign3290_e2276_d_n11, assign3290_e2276_d_n12, assign3290_e2276_d_n17,) = {
    if ((locals.var_guard23 != 0.0) && (locals.var_guard24 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rbulk, locals.var_rbulk_dn0, locals.var_rbulk_dn2, locals.var_rbulk_dn6, locals.var_rbulk_dn7, locals.var_rbulk_dn10, locals.var_rbulk_dn11, locals.var_rbulk_dn12, locals.var_rbulk_dn17,)
    }
};
        locals.var_rbulk = assign3290_e2276;
        locals.var_rbulk_dn0 = assign3290_e2276_d_n0;
        locals.var_rbulk_dn2 = assign3290_e2276_d_n2;
        locals.var_rbulk_dn6 = assign3290_e2276_d_n6;
        locals.var_rbulk_dn7 = assign3290_e2276_d_n7;
        locals.var_rbulk_dn10 = assign3290_e2276_d_n10;
        locals.var_rbulk_dn11 = assign3290_e2276_d_n11;
        locals.var_rbulk_dn12 = assign3290_e2276_d_n12;
        locals.var_rbulk_dn17 = assign3290_e2276_d_n17;

        let (assign3300_e2281, assign3300_e2281_d_n0, assign3300_e2281_d_n2, assign3300_e2281_d_n6, assign3300_e2281_d_n7, assign3300_e2281_d_n10, assign3300_e2281_d_n11, assign3300_e2281_d_n12, assign3300_e2281_d_n17,) = {
    if (locals.var_guard23 == 0.0) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rbulk, locals.var_rbulk_dn0, locals.var_rbulk_dn2, locals.var_rbulk_dn6, locals.var_rbulk_dn7, locals.var_rbulk_dn10, locals.var_rbulk_dn11, locals.var_rbulk_dn12, locals.var_rbulk_dn17,)
    }
};
        locals.var_rbulk = assign3300_e2281;
        locals.var_rbulk_dn0 = assign3300_e2281_d_n0;
        locals.var_rbulk_dn2 = assign3300_e2281_d_n2;
        locals.var_rbulk_dn6 = assign3300_e2281_d_n6;
        locals.var_rbulk_dn7 = assign3300_e2281_d_n7;
        locals.var_rbulk_dn10 = assign3300_e2281_d_n10;
        locals.var_rbulk_dn11 = assign3300_e2281_d_n11;
        locals.var_rbulk_dn12 = assign3300_e2281_d_n12;
        locals.var_rbulk_dn17 = assign3300_e2281_d_n17;

        let assign3400_e2337: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard28 = assign3400_e2337;

        let (assign3410_e2350,) = {
    if ((locals.var_guard28 != 0.0) && (p.p24 != 0.0)) {
        let (assign3410_e2348,) = {
            if (locals.var_abtp_given != 0.0) {
                (p.p23,)
            } else {
                let assign3410_e2345: f64 = (p.p20 * p.p9);
                let assign3410_e2347: f64 = (assign3410_e2345 * p.p19);
                (assign3410_e2347,)
            }
        };
        (assign3410_e2348,)
    } else {
        (locals.var_area_bt_p,)
    }
};
        locals.var_area_bt_p = assign3410_e2350;

        let (assign3420_e2363,) = {
    if ((locals.var_guard28 != 0.0) && (p.p24 != 0.0)) {
        let (assign3420_e2361,) = {
            if (locals.var_abtn_given != 0.0) {
                (p.p22,)
            } else {
                let assign3420_e2358: f64 = (p.p21 * p.p9);
                let assign3420_e2360: f64 = (assign3420_e2358 * p.p19);
                (assign3420_e2360,)
            }
        };
        (assign3420_e2361,)
    } else {
        (locals.var_area_bt_n,)
    }
};
        locals.var_area_bt_n = assign3420_e2363;

        let (assign3430_e2369,) = {
    if ((locals.var_guard28 != 0.0) && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_cbtp,)
    }
};
        locals.var_cbtp = assign3430_e2369;

        let (assign3440_e2375,) = {
    if ((locals.var_guard28 != 0.0) && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_cbtn,)
    }
};
        locals.var_cbtn = assign3440_e2375;

        let assign3450_e2380: f64 = if ((locals.var_area_bt_p > 0.0) && (locals.var_cbtbp_given != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard29 = assign3450_e2380;

        let (assign3460_e2391,) = {
    if (((locals.var_guard28 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard29 != 0.0)) {
        let assign3460_e2387: f64 = (-locals.var_area_bt_p);
        let assign3460_e2389: f64 = (assign3460_e2387 * p.p294);
        (assign3460_e2389,)
    } else {
        (locals.var_cbtp,)
    }
};
        locals.var_cbtp = assign3460_e2391;

        let (assign3470_e2400,) = {
    if (((locals.var_guard28 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard29 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_cbtp,)
    }
};
        locals.var_cbtp = assign3470_e2400;

        let assign3480_e2405: f64 = if ((locals.var_area_bt_n > 0.0) && (locals.var_cbtbn_given != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard30 = assign3480_e2405;

        let (assign3490_e2416,) = {
    if (((locals.var_guard28 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard30 != 0.0)) {
        let assign3490_e2412: f64 = (-locals.var_area_bt_n);
        let assign3490_e2414: f64 = (assign3490_e2412 * p.p293);
        (assign3490_e2414,)
    } else {
        (locals.var_cbtn,)
    }
};
        locals.var_cbtn = assign3490_e2416;

        let (assign3500_e2424,) = {
    if (((locals.var_guard28 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard30 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_area_bt_n,)
    }
};
        locals.var_area_bt_n = assign3500_e2424;

        let (assign3510_e2431,) = {
    if ((locals.var_guard28 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_area_bt_n,)
    }
};
        locals.var_area_bt_n = assign3510_e2431;

        let (assign3520_e2438,) = {
    if ((locals.var_guard28 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_cbtn,)
    }
};
        locals.var_cbtn = assign3520_e2438;

        let (assign3530_e2445,) = {
    if ((locals.var_guard28 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_area_bt_p,)
    }
};
        locals.var_area_bt_p = assign3530_e2445;

        let (assign3540_e2452,) = {
    if ((locals.var_guard28 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_cbtp,)
    }
};
        locals.var_cbtp = assign3540_e2452;

        let (assign3550_e2465,) = {
    if (locals.var_guard28 != 0.0) {
        let (assign3550_e2463,) = {
            if (p.p19 > locals.var_lgate) {
                let assign3550_e2460: f64 = (p.p19 - locals.var_lgate);
                let assign3550_e2461: f64 = (0.5 * assign3550_e2460);
                (assign3550_e2461,)
            } else {
                (0.0,)
            }
        };
        (assign3550_e2463,)
    } else {
        (locals.var_peri_hhi,)
    }
};
        locals.var_peri_hhi = assign3550_e2465;

        let assign3560_e2468: f64 = if locals.var_pdbcp_given == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign3560_e2468;

        let (assign3570_e2474,) = {
    if ((locals.var_guard28 != 0.0) && (locals.var_guard31 != 0.0)) {
        (locals.var_peri_hhi,)
    } else {
        (locals.var_uc_pdbcp,)
    }
};
        locals.var_uc_pdbcp = assign3570_e2474;

    }

    pub(super) fn stamp_transient_block_6(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let assign3580_e2477: f64 = if locals.var_psbcp_given == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign3580_e2477;

        let (assign3590_e2483,) = {
    if ((locals.var_guard28 != 0.0) && (locals.var_guard32 != 0.0)) {
        (locals.var_peri_hhi,)
    } else {
        (locals.var_uc_psbcp,)
    }
};
        locals.var_uc_psbcp = assign3590_e2483;

        let (assign3600_e2491,) = {
    if (locals.var_guard28 != 0.0) {
        let assign3600_e2488: f64 = (p.p9 * locals.var_uc_pdbcp);
        let assign3600_e2489: f64 = (locals.var_weff_nf + assign3600_e2488);
        (assign3600_e2489,)
    } else {
        (locals.var_w_diod,)
    }
};
        locals.var_w_diod = assign3600_e2491;

        let (assign3610_e2499,) = {
    if (locals.var_guard28 != 0.0) {
        let assign3610_e2496: f64 = (p.p9 * locals.var_uc_psbcp);
        let assign3610_e2497: f64 = (locals.var_weff_nf + assign3610_e2496);
        (assign3610_e2497,)
    } else {
        (locals.var_w_dios,)
    }
};
        locals.var_w_dios = assign3610_e2499;

        let (assign3620_e2507,) = {
    if (locals.var_guard28 != 0.0) {
        let assign3620_e2504: f64 = (p.p9 * locals.var_uc_pdbcp);
        let assign3620_e2505: f64 = (locals.var_weffcv_nf + assign3620_e2504);
        (assign3620_e2505,)
    } else {
        (locals.var_w_diodcv,)
    }
};
        locals.var_w_diodcv = assign3620_e2507;

        let (assign3630_e2515,) = {
    if (locals.var_guard28 != 0.0) {
        let assign3630_e2512: f64 = (p.p9 * locals.var_uc_psbcp);
        let assign3630_e2513: f64 = (locals.var_weffcv_nf + assign3630_e2512);
        (assign3630_e2513,)
    } else {
        (locals.var_w_dioscv,)
    }
};
        locals.var_w_dioscv = assign3630_e2515;

        let (assign3640_e2520,) = {
    if (locals.var_guard28 == 0.0) {
        (0.0,)
    } else {
        (locals.var_area_bt_n,)
    }
};
        locals.var_area_bt_n = assign3640_e2520;

        let (assign3650_e2525,) = {
    if (locals.var_guard28 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cbtn,)
    }
};
        locals.var_cbtn = assign3650_e2525;

        let (assign3660_e2530,) = {
    if (locals.var_guard28 == 0.0) {
        (0.0,)
    } else {
        (locals.var_area_bt_p,)
    }
};
        locals.var_area_bt_p = assign3660_e2530;

        let (assign3670_e2535,) = {
    if (locals.var_guard28 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cbtp,)
    }
};
        locals.var_cbtp = assign3670_e2535;

        let (assign3680_e2540,) = {
    if (locals.var_guard28 == 0.0) {
        (0.0,)
    } else {
        (locals.var_w_diod,)
    }
};
        locals.var_w_diod = assign3680_e2540;

        let (assign3690_e2545,) = {
    if (locals.var_guard28 == 0.0) {
        (0.0,)
    } else {
        (locals.var_w_dios,)
    }
};
        locals.var_w_dios = assign3690_e2545;

        let (assign3700_e2550,) = {
    if (locals.var_guard28 == 0.0) {
        (0.0,)
    } else {
        (locals.var_w_diodcv,)
    }
};
        locals.var_w_diodcv = assign3700_e2550;

        let (assign3710_e2555,) = {
    if (locals.var_guard28 == 0.0) {
        (0.0,)
    } else {
        (locals.var_w_dioscv,)
    }
};
        locals.var_w_dioscv = assign3710_e2555;

        let assign3720_e2558: f64 = (p.p50 * (nv6 - nv7));
        locals.var_vdsi = assign3720_e2558;
        locals.var_vdsi_dn6 = p.p50;
        locals.var_vdsi_dn7 = (-p.p50);

        let assign3730_e2561: f64 = (p.p50 * (nv11 - nv7));
        locals.var_vgsi = assign3730_e2561;
        locals.var_vgsi_dn7 = (-p.p50);
        locals.var_vgsi_dn11 = p.p50;

        let assign3740_e2564: f64 = (p.p50 * (nv12 - nv7));
        locals.var_vbsi = assign3740_e2564;
        locals.var_vbsi_dn7 = (-p.p50);
        locals.var_vbsi_dn12 = p.p50;

        let assign3780_e2576: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign3780_e2576;

        let (assign3790_e2582, assign3790_e2582_d_n6, assign3790_e2582_d_n12,) = {
    if (locals.var_guard33 != 0.0) {
        let assign3790_e2580: f64 = (p.p50 * (nv12 - nv6));
        (assign3790_e2580, (-p.p50), p.p50,)
    } else {
        (locals.var_vbcd, locals.var_vbcd_dn6, locals.var_vbcd_dn12,)
    }
};
        locals.var_vbcd = assign3790_e2582;
        locals.var_vbcd_dn6 = assign3790_e2582_d_n6;
        locals.var_vbcd_dn12 = assign3790_e2582_d_n12;

        let (assign3800_e2588, assign3800_e2588_d_n7, assign3800_e2588_d_n12,) = {
    if (locals.var_guard33 != 0.0) {
        let assign3800_e2586: f64 = (p.p50 * (nv12 - nv7));
        (assign3800_e2586, (-p.p50), p.p50,)
    } else {
        (locals.var_vbcs, locals.var_vbcs_dn7, locals.var_vbcs_dn12,)
    }
};
        locals.var_vbcs = assign3800_e2588;
        locals.var_vbcs_dn7 = assign3800_e2588_d_n7;
        locals.var_vbcs_dn12 = assign3800_e2588_d_n12;

        let (assign3810_e2598, assign3810_e2598_d_n18,) = {
    if ((locals.var_guard33 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign3810_e2594: f64 = (1e-9 / 0.0001);
        let assign3810_e2596: f64 = (assign3810_e2594 * (nv18 - 0.0));
        (assign3810_e2596, assign3810_e2594,)
    } else {
        (locals.var_qi_nqs, locals.var_qi_nqs_dn18,)
    }
};
        locals.var_qi_nqs = assign3810_e2598;
        locals.var_qi_nqs_dn18 = assign3810_e2598_d_n18;

        let (assign3820_e2608, assign3820_e2608_d_n13,) = {
    if ((locals.var_guard33 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign3820_e2604: f64 = (1e-9 / 0.0001);
        let assign3820_e2606: f64 = (assign3820_e2604 * (nv13 - 0.0));
        (assign3820_e2606, assign3820_e2604,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign3820_e2608;
        locals.var_qb_nqs_dn13 = assign3820_e2608_d_n13;

        let (assign3830_e2615, assign3830_e2615_d_n18,) = {
    if ((locals.var_guard33 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_qi_nqs, locals.var_qi_nqs_dn18,)
    }
};
        locals.var_qi_nqs = assign3830_e2615;
        locals.var_qi_nqs_dn18 = assign3830_e2615_d_n18;

        let (assign3840_e2622, assign3840_e2622_d_n13,) = {
    if ((locals.var_guard33 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign3840_e2622;
        locals.var_qb_nqs_dn13 = assign3840_e2622_d_n13;

        let (assign3850_e2627, assign3850_e2627_d_n6, assign3850_e2627_d_n12,) = {
    if (locals.var_guard33 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbcd, locals.var_vbcd_dn6, locals.var_vbcd_dn12,)
    }
};
        locals.var_vbcd = assign3850_e2627;
        locals.var_vbcd_dn6 = assign3850_e2627_d_n6;
        locals.var_vbcd_dn12 = assign3850_e2627_d_n12;

        let (assign3860_e2632, assign3860_e2632_d_n7, assign3860_e2632_d_n12,) = {
    if (locals.var_guard33 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbcs, locals.var_vbcs_dn7, locals.var_vbcs_dn12,)
    }
};
        locals.var_vbcs = assign3860_e2632;
        locals.var_vbcs_dn7 = assign3860_e2632_d_n7;
        locals.var_vbcs_dn12 = assign3860_e2632_d_n12;

        let (assign3870_e2643, assign3870_e2643_d_n0, assign3870_e2643_d_n2, assign3870_e2643_d_n6, assign3870_e2643_d_n7, assign3870_e2643_d_n10, assign3870_e2643_d_n11, assign3870_e2643_d_n12, assign3870_e2643_d_n15, assign3870_e2643_d_n17, assign3870_e2643_d_n18,) = {
    if ((locals.var_guard33 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign3870_e2639: f64 = (1e-9 / 0.0001);
        let assign3870_e2641: f64 = (assign3870_e2639 * (nv15 - 0.0));
        (assign3870_e2641, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign3870_e2639, 0.0, 0.0,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18,)
    }
};
        locals.var_qd_nqs = assign3870_e2643;
        locals.var_qd_nqs_dn0 = assign3870_e2643_d_n0;
        locals.var_qd_nqs_dn2 = assign3870_e2643_d_n2;
        locals.var_qd_nqs_dn6 = assign3870_e2643_d_n6;
        locals.var_qd_nqs_dn7 = assign3870_e2643_d_n7;
        locals.var_qd_nqs_dn10 = assign3870_e2643_d_n10;
        locals.var_qd_nqs_dn11 = assign3870_e2643_d_n11;
        locals.var_qd_nqs_dn12 = assign3870_e2643_d_n12;
        locals.var_qd_nqs_dn15 = assign3870_e2643_d_n15;
        locals.var_qd_nqs_dn17 = assign3870_e2643_d_n17;
        locals.var_qd_nqs_dn18 = assign3870_e2643_d_n18;

        let (assign3880_e2654, assign3880_e2654_d_n0, assign3880_e2654_d_n2, assign3880_e2654_d_n6, assign3880_e2654_d_n7, assign3880_e2654_d_n10, assign3880_e2654_d_n11, assign3880_e2654_d_n12, assign3880_e2654_d_n16, assign3880_e2654_d_n17, assign3880_e2654_d_n18,) = {
    if ((locals.var_guard33 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign3880_e2650: f64 = (1e-9 / 0.0001);
        let assign3880_e2652: f64 = (assign3880_e2650 * (nv16 - 0.0));
        (assign3880_e2652, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign3880_e2650, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18,)
    }
};
        locals.var_qs_nqs = assign3880_e2654;
        locals.var_qs_nqs_dn0 = assign3880_e2654_d_n0;
        locals.var_qs_nqs_dn2 = assign3880_e2654_d_n2;
        locals.var_qs_nqs_dn6 = assign3880_e2654_d_n6;
        locals.var_qs_nqs_dn7 = assign3880_e2654_d_n7;
        locals.var_qs_nqs_dn10 = assign3880_e2654_d_n10;
        locals.var_qs_nqs_dn11 = assign3880_e2654_d_n11;
        locals.var_qs_nqs_dn12 = assign3880_e2654_d_n12;
        locals.var_qs_nqs_dn16 = assign3880_e2654_d_n16;
        locals.var_qs_nqs_dn17 = assign3880_e2654_d_n17;
        locals.var_qs_nqs_dn18 = assign3880_e2654_d_n18;

        let (assign3890_e2665, assign3890_e2665_d_n13,) = {
    if ((locals.var_guard33 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign3890_e2661: f64 = (1e-9 / 0.0001);
        let assign3890_e2663: f64 = (assign3890_e2661 * (nv13 - 0.0));
        (assign3890_e2663, assign3890_e2661,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign3890_e2665;
        locals.var_qb_nqs_dn13 = assign3890_e2665_d_n13;

        let (assign3900_e2673, assign3900_e2673_d_n0, assign3900_e2673_d_n2, assign3900_e2673_d_n6, assign3900_e2673_d_n7, assign3900_e2673_d_n10, assign3900_e2673_d_n11, assign3900_e2673_d_n12, assign3900_e2673_d_n15, assign3900_e2673_d_n17, assign3900_e2673_d_n18,) = {
    if ((locals.var_guard33 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18,)
    }
};
        locals.var_qd_nqs = assign3900_e2673;
        locals.var_qd_nqs_dn0 = assign3900_e2673_d_n0;
        locals.var_qd_nqs_dn2 = assign3900_e2673_d_n2;
        locals.var_qd_nqs_dn6 = assign3900_e2673_d_n6;
        locals.var_qd_nqs_dn7 = assign3900_e2673_d_n7;
        locals.var_qd_nqs_dn10 = assign3900_e2673_d_n10;
        locals.var_qd_nqs_dn11 = assign3900_e2673_d_n11;
        locals.var_qd_nqs_dn12 = assign3900_e2673_d_n12;
        locals.var_qd_nqs_dn15 = assign3900_e2673_d_n15;
        locals.var_qd_nqs_dn17 = assign3900_e2673_d_n17;
        locals.var_qd_nqs_dn18 = assign3900_e2673_d_n18;

        let (assign3910_e2681, assign3910_e2681_d_n0, assign3910_e2681_d_n2, assign3910_e2681_d_n6, assign3910_e2681_d_n7, assign3910_e2681_d_n10, assign3910_e2681_d_n11, assign3910_e2681_d_n12, assign3910_e2681_d_n16, assign3910_e2681_d_n17, assign3910_e2681_d_n18,) = {
    if ((locals.var_guard33 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18,)
    }
};
        locals.var_qs_nqs = assign3910_e2681;
        locals.var_qs_nqs_dn0 = assign3910_e2681_d_n0;
        locals.var_qs_nqs_dn2 = assign3910_e2681_d_n2;
        locals.var_qs_nqs_dn6 = assign3910_e2681_d_n6;
        locals.var_qs_nqs_dn7 = assign3910_e2681_d_n7;
        locals.var_qs_nqs_dn10 = assign3910_e2681_d_n10;
        locals.var_qs_nqs_dn11 = assign3910_e2681_d_n11;
        locals.var_qs_nqs_dn12 = assign3910_e2681_d_n12;
        locals.var_qs_nqs_dn16 = assign3910_e2681_d_n16;
        locals.var_qs_nqs_dn17 = assign3910_e2681_d_n17;
        locals.var_qs_nqs_dn18 = assign3910_e2681_d_n18;

        let (assign3920_e2689, assign3920_e2689_d_n13,) = {
    if ((locals.var_guard33 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign3920_e2689;
        locals.var_qb_nqs_dn13 = assign3920_e2689_d_n13;

        let (assign3930_e2704, assign3930_e2704_d_n10,) = {
    if ((p.p38 > 0.0) && (locals.var_mks_rth0 > 0.0)) {
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
        locals.var_deltemp = assign3930_e2704;
        locals.var_deltemp_dn10 = assign3930_e2704_d_n10;

        let assign3940_e2707: f64 = if locals.var_vdsi >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign3940_e2707;

        let (assign3950_e2711,) = {
    if (locals.var_guard34 != 0.0) {
        (1.0,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign3950_e2711;

        let (assign3960_e2715,) = {
    if (locals.var_guard34 != 0.0) {
        (1.0,)
    } else {
        (locals.var_modenml,)
    }
};
        locals.var_modenml = assign3960_e2715;

        let (assign3970_e2719,) = {
    if (locals.var_guard34 != 0.0) {
        (0.0,)
    } else {
        (locals.var_modervs,)
    }
};
        locals.var_modervs = assign3970_e2719;

        let (assign3980_e2723, assign3980_e2723_d_n0, assign3980_e2723_d_n2, assign3980_e2723_d_n6, assign3980_e2723_d_n7, assign3980_e2723_d_n10, assign3980_e2723_d_n11, assign3980_e2723_d_n12, assign3980_e2723_d_n17,) = {
    if (locals.var_guard34 != 0.0) {
        (locals.var_vdsi, 0.0, 0.0, locals.var_vdsi_dn6, locals.var_vdsi_dn7, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign3980_e2723;
        locals.var_vds_dn0 = assign3980_e2723_d_n0;
        locals.var_vds_dn2 = assign3980_e2723_d_n2;
        locals.var_vds_dn6 = assign3980_e2723_d_n6;
        locals.var_vds_dn7 = assign3980_e2723_d_n7;
        locals.var_vds_dn10 = assign3980_e2723_d_n10;
        locals.var_vds_dn11 = assign3980_e2723_d_n11;
        locals.var_vds_dn12 = assign3980_e2723_d_n12;
        locals.var_vds_dn17 = assign3980_e2723_d_n17;

        let (assign3990_e2727, assign3990_e2727_d_n6, assign3990_e2727_d_n7, assign3990_e2727_d_n11,) = {
    if (locals.var_guard34 != 0.0) {
        (locals.var_vgsi, 0.0, locals.var_vgsi_dn7, locals.var_vgsi_dn11,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn11,)
    }
};
        locals.var_vgs = assign3990_e2727;
        locals.var_vgs_dn6 = assign3990_e2727_d_n6;
        locals.var_vgs_dn7 = assign3990_e2727_d_n7;
        locals.var_vgs_dn11 = assign3990_e2727_d_n11;

        let (assign4000_e2731, assign4000_e2731_d_n0, assign4000_e2731_d_n2, assign4000_e2731_d_n6, assign4000_e2731_d_n7, assign4000_e2731_d_n10, assign4000_e2731_d_n11, assign4000_e2731_d_n12, assign4000_e2731_d_n17,) = {
    if (locals.var_guard34 != 0.0) {
        (locals.var_vbsi, 0.0, 0.0, 0.0, locals.var_vbsi_dn7, 0.0, 0.0, locals.var_vbsi_dn12, 0.0,)
    } else {
        (locals.var_vbs, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
    }
};
        locals.var_vbs = assign4000_e2731;
        locals.var_vbs_dn0 = assign4000_e2731_d_n0;
        locals.var_vbs_dn2 = assign4000_e2731_d_n2;
        locals.var_vbs_dn6 = assign4000_e2731_d_n6;
        locals.var_vbs_dn7 = assign4000_e2731_d_n7;
        locals.var_vbs_dn10 = assign4000_e2731_d_n10;
        locals.var_vbs_dn11 = assign4000_e2731_d_n11;
        locals.var_vbs_dn12 = assign4000_e2731_d_n12;
        locals.var_vbs_dn17 = assign4000_e2731_d_n17;

        let (assign4040_e2749,) = {
    if (locals.var_guard34 == 0.0) {
        let assign4040_e2747: f64 = (-1.0);
        (assign4040_e2747,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign4040_e2749;

        let (assign4050_e2754,) = {
    if (locals.var_guard34 == 0.0) {
        (0.0,)
    } else {
        (locals.var_modenml,)
    }
};
        locals.var_modenml = assign4050_e2754;

        let (assign4060_e2759,) = {
    if (locals.var_guard34 == 0.0) {
        (1.0,)
    } else {
        (locals.var_modervs,)
    }
};
        locals.var_modervs = assign4060_e2759;

        let (assign4070_e2765, assign4070_e2765_d_n0, assign4070_e2765_d_n2, assign4070_e2765_d_n6, assign4070_e2765_d_n7, assign4070_e2765_d_n10, assign4070_e2765_d_n11, assign4070_e2765_d_n12, assign4070_e2765_d_n17,) = {
    if (locals.var_guard34 == 0.0) {
        let assign4070_e2763: f64 = (-locals.var_vdsi);
        (assign4070_e2763, 0.0, 0.0, (-locals.var_vdsi_dn6), (-locals.var_vdsi_dn7), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign4070_e2765;
        locals.var_vds_dn0 = assign4070_e2765_d_n0;
        locals.var_vds_dn2 = assign4070_e2765_d_n2;
        locals.var_vds_dn6 = assign4070_e2765_d_n6;
        locals.var_vds_dn7 = assign4070_e2765_d_n7;
        locals.var_vds_dn10 = assign4070_e2765_d_n10;
        locals.var_vds_dn11 = assign4070_e2765_d_n11;
        locals.var_vds_dn12 = assign4070_e2765_d_n12;
        locals.var_vds_dn17 = assign4070_e2765_d_n17;

        let (assign4080_e2772, assign4080_e2772_d_n6, assign4080_e2772_d_n7, assign4080_e2772_d_n11,) = {
    if (locals.var_guard34 == 0.0) {
        let assign4080_e2770: f64 = (locals.var_vgsi - locals.var_vdsi);
        (assign4080_e2770, (-locals.var_vdsi_dn6), (locals.var_vgsi_dn7 - locals.var_vdsi_dn7), locals.var_vgsi_dn11,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn11,)
    }
};
        locals.var_vgs = assign4080_e2772;
        locals.var_vgs_dn6 = assign4080_e2772_d_n6;
        locals.var_vgs_dn7 = assign4080_e2772_d_n7;
        locals.var_vgs_dn11 = assign4080_e2772_d_n11;

        let (assign4090_e2779, assign4090_e2779_d_n0, assign4090_e2779_d_n2, assign4090_e2779_d_n6, assign4090_e2779_d_n7, assign4090_e2779_d_n10, assign4090_e2779_d_n11, assign4090_e2779_d_n12, assign4090_e2779_d_n17,) = {
    if (locals.var_guard34 == 0.0) {
        let assign4090_e2777: f64 = (locals.var_vbsi - locals.var_vdsi);
        (assign4090_e2777, 0.0, 0.0, (-locals.var_vdsi_dn6), (locals.var_vbsi_dn7 - locals.var_vdsi_dn7), 0.0, 0.0, locals.var_vbsi_dn12, 0.0,)
    } else {
        (locals.var_vbs, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
    }
};
        locals.var_vbs = assign4090_e2779;
        locals.var_vbs_dn0 = assign4090_e2779_d_n0;
        locals.var_vbs_dn2 = assign4090_e2779_d_n2;
        locals.var_vbs_dn6 = assign4090_e2779_d_n6;
        locals.var_vbs_dn7 = assign4090_e2779_d_n7;
        locals.var_vbs_dn10 = assign4090_e2779_d_n10;
        locals.var_vbs_dn11 = assign4090_e2779_d_n11;
        locals.var_vbs_dn12 = assign4090_e2779_d_n12;
        locals.var_vbs_dn17 = assign4090_e2779_d_n17;

        let assign4150_e2806: f64 = ctx_temp;
        locals.var_ttemp = assign4150_e2806;
        locals.var_ttemp_dn10 = 0.0;

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4160_e2810, assign4160_e2810_d_n10,) = {
    if (locals.var_temp_given != 0.0) {
        (locals.var_uc_temp, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn10,)
    }
};
        locals.var_ttemp = assign4160_e2810;
        locals.var_ttemp_dn10 = assign4160_e2810_d_n10;

        let (assign4170_e2816, assign4170_e2816_d_n10,) = {
    if (locals.var_dtemp_given != 0.0) {
        let assign4170_e2814: f64 = (locals.var_ttemp + p.p17);
        (assign4170_e2814, locals.var_ttemp_dn10,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn10,)
    }
};
        locals.var_ttemp = assign4170_e2816;
        locals.var_ttemp_dn10 = assign4170_e2816_d_n10;

        let assign4180_e2819: f64 = (locals.var_ttemp + locals.var_deltemp);
        locals.var_ttemp = assign4180_e2819;
        locals.var_ttemp_dn10 = (locals.var_ttemp_dn10 + locals.var_deltemp_dn10);

        let assign4190_e2822: f64 = (locals.var_ttemp - locals.var_uc_tnom);
        locals.var_t1 = assign4190_e2822;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn10 = locals.var_ttemp_dn10;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn17 = 0.0;

        let assign4200_e2826: f64 = (locals.var_ttemp + locals.var_uc_tnom);
        let assign4200_e2827: f64 = (locals.var_t1 * assign4200_e2826);
        locals.var_t2 = assign4200_e2827;
        locals.var_t2_dn0 = (locals.var_t1_dn0 * assign4200_e2826);
        locals.var_t2_dn2 = (locals.var_t1_dn2 * assign4200_e2826);
        locals.var_t2_dn6 = (locals.var_t1_dn6 * assign4200_e2826);
        locals.var_t2_dn7 = (locals.var_t1_dn7 * assign4200_e2826);
        locals.var_t2_dn10 = ((locals.var_t1_dn10 * assign4200_e2826) + (locals.var_t1 * locals.var_ttemp_dn10));
        locals.var_t2_dn11 = (locals.var_t1_dn11 * assign4200_e2826);
        locals.var_t2_dn12 = (locals.var_t1_dn12 * assign4200_e2826);
        locals.var_t2_dn17 = (locals.var_t1_dn17 * assign4200_e2826);

        let assign4210_e2831: f64 = (p.p53 * locals.var_t1);
        let assign4210_e2832: f64 = (locals.var_egtnom - assign4210_e2831);
        let assign4210_e2835: f64 = (p.p54 * locals.var_t2);
        let assign4210_e2836: f64 = (assign4210_e2832 - assign4210_e2835);
        locals.var_eg = assign4210_e2836;
        locals.var_eg_dn0 = ((-(p.p53 * locals.var_t1_dn0)) - (p.p54 * locals.var_t2_dn0));
        locals.var_eg_dn2 = ((-(p.p53 * locals.var_t1_dn2)) - (p.p54 * locals.var_t2_dn2));
        locals.var_eg_dn6 = ((-(p.p53 * locals.var_t1_dn6)) - (p.p54 * locals.var_t2_dn6));
        locals.var_eg_dn7 = ((-(p.p53 * locals.var_t1_dn7)) - (p.p54 * locals.var_t2_dn7));
        locals.var_eg_dn10 = ((-(p.p53 * locals.var_t1_dn10)) - (p.p54 * locals.var_t2_dn10));
        locals.var_eg_dn11 = ((-(p.p53 * locals.var_t1_dn11)) - (p.p54 * locals.var_t2_dn11));
        locals.var_eg_dn12 = ((-(p.p53 * locals.var_t1_dn12)) - (p.p54 * locals.var_t2_dn12));
        locals.var_eg_dn17 = ((-(p.p53 * locals.var_t1_dn17)) - (p.p54 * locals.var_t2_dn17));

        let assign4220_e2840: f64 = (1.3806226e-23 * locals.var_ttemp);
        let assign4220_e2841: f64 = (1.6021918e-19 / assign4220_e2840);
        locals.var_beta = assign4220_e2841;
        locals.var_beta_dn10 = (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn10)) / (assign4220_e2840 * assign4220_e2840)));

        let assign4230_e2844: f64 = (locals.var_beta * locals.var_beta);
        locals.var_beta2 = assign4230_e2844;
        locals.var_beta2_dn10 = ((locals.var_beta_dn10 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn10));

        let assign4240_e2847: f64 = (1.0 / locals.var_beta);
        locals.var_beta_inv = assign4240_e2847;
        locals.var_beta_inv_dn10 = (-(locals.var_beta_dn10 / (locals.var_beta * locals.var_beta)));

        let assign4250_e2853: f64 = (locals.var_wg).powf(p.p99);
        let assign4250_e2854: f64 = (p.p98 / assign4250_e2853);
        let assign4250_e2855: f64 = (1.0 + assign4250_e2854);
        let assign4250_e2856: f64 = (p.p254 * assign4250_e2855);
        let assign4250_e2861: f64 = (locals.var_lgle).powf(p.p101);
        let assign4250_e2862: f64 = (p.p100 / assign4250_e2861);
        let assign4250_e2863: f64 = (1.0 + assign4250_e2862);
        let assign4250_e2864: f64 = (assign4250_e2856 * assign4250_e2863);
        let assign4250_e2869: f64 = (locals.var_wl).powf(p.p103);
        let assign4250_e2870: f64 = (p.p102 / assign4250_e2869);
        let assign4250_e2871: f64 = (1.0 + assign4250_e2870);
        let assign4250_e2872: f64 = (assign4250_e2864 * assign4250_e2871);
        locals.var_cgs_mueph = assign4250_e2872;

        let assign4260_e2876: f64 = (1.0 + p.p159);
        let assign4260_e2877: f64 = (1.0 / assign4260_e2876);
        locals.var_t2__blk40 = assign4260_e2877;

        locals.var_t3__blk41 = 0.0;

        let assign4280_e2883: f64 = (locals.var_t2__blk40 * locals.var_t3__blk41);
        let assign4280_e2884: f64 = (1.0 + assign4280_e2883);
        let assign4280_e2885: f64 = (locals.var_cgs_mueph * assign4280_e2884);
        locals.var_cgs_wmueph = assign4280_e2885;

        let assign4290_e2888: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign4290_e2890: f64 = (assign4290_e2888).powf(p.p112);
        locals.var_t1__blk39 = assign4290_e2890;
        locals.var_t1__blk39_dn10 = if 0.0 == 0.0 && ((p.p112) as f64).is_finite() && ((p.p112) as f64).fract() == 0.0 { if p.p112 == 0.0 { 0.0 } else { (p.p112 * ((assign4290_e2888).powf(p.p112 - 1.0) * (locals.var_ttemp_dn10 / locals.var_uc_tnom))) } } else { (assign4290_e2890 * (p.p112 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign4290_e2888))) };

        let assign4300_e2893: f64 = (locals.var_t1__blk39 / locals.var_cgs_wmueph);
        locals.var_cgs_mphn0 = assign4300_e2893;
        locals.var_cgs_mphn0_dn10 = (locals.var_t1__blk39_dn10 / locals.var_cgs_wmueph);

        let assign4310_e2896: f64 = (locals.var_ptovr0 * locals.var_beta_inv);
        locals.var_ptovr = assign4310_e2896;
        locals.var_ptovr_dn0 = (locals.var_ptovr0_dn0 * locals.var_beta_inv);
        locals.var_ptovr_dn2 = (locals.var_ptovr0_dn2 * locals.var_beta_inv);
        locals.var_ptovr_dn6 = (locals.var_ptovr0_dn6 * locals.var_beta_inv);
        locals.var_ptovr_dn7 = (locals.var_ptovr0_dn7 * locals.var_beta_inv);
        locals.var_ptovr_dn10 = ((locals.var_ptovr0_dn10 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn10));
        locals.var_ptovr_dn11 = (locals.var_ptovr0_dn11 * locals.var_beta_inv);
        locals.var_ptovr_dn12 = (locals.var_ptovr0_dn12 * locals.var_beta_inv);
        locals.var_ptovr_dn17 = (locals.var_ptovr0_dn17 * locals.var_beta_inv);

        let assign4320_e2899: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        locals.var_t1 = assign4320_e2899;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn10 = (locals.var_ttemp_dn10 / locals.var_uc_tnom);
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn17 = 0.0;

        let assign4330_e2902: f64 = (locals.var_vmax0 * locals.var_mks_vmax);
        let assign4330_e2906: f64 = (0.4 * locals.var_t1);
        let assign4330_e2907: f64 = (1.8 + assign4330_e2906);
        let assign4330_e2910: f64 = (0.1 * locals.var_t1);
        let assign4330_e2912: f64 = (assign4330_e2910 * locals.var_t1);
        let assign4330_e2913: f64 = (assign4330_e2907 + assign4330_e2912);
        let assign4330_e2917: f64 = (1.0 - locals.var_t1);
        let assign4330_e2918: f64 = (locals.var_mks_vtmp * assign4330_e2917);
        let assign4330_e2919: f64 = (assign4330_e2913 - assign4330_e2918);
        let assign4330_e2920: f64 = (assign4330_e2902 / assign4330_e2919);
        locals.var_vmaxe = assign4330_e2920;
        locals.var_vmaxe_dn0 = ((((locals.var_vmax0_dn0 * locals.var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * locals.var_t1_dn0) + (((0.1 * locals.var_t1_dn0) * locals.var_t1) + (assign4330_e2910 * locals.var_t1_dn0))) - (locals.var_mks_vtmp * (-locals.var_t1_dn0))))) / (assign4330_e2919 * assign4330_e2919));
        locals.var_vmaxe_dn2 = ((((locals.var_vmax0_dn2 * locals.var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * locals.var_t1_dn2) + (((0.1 * locals.var_t1_dn2) * locals.var_t1) + (assign4330_e2910 * locals.var_t1_dn2))) - (locals.var_mks_vtmp * (-locals.var_t1_dn2))))) / (assign4330_e2919 * assign4330_e2919));
        locals.var_vmaxe_dn6 = ((((locals.var_vmax0_dn6 * locals.var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * locals.var_t1_dn6) + (((0.1 * locals.var_t1_dn6) * locals.var_t1) + (assign4330_e2910 * locals.var_t1_dn6))) - (locals.var_mks_vtmp * (-locals.var_t1_dn6))))) / (assign4330_e2919 * assign4330_e2919));
        locals.var_vmaxe_dn7 = ((((locals.var_vmax0_dn7 * locals.var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * locals.var_t1_dn7) + (((0.1 * locals.var_t1_dn7) * locals.var_t1) + (assign4330_e2910 * locals.var_t1_dn7))) - (locals.var_mks_vtmp * (-locals.var_t1_dn7))))) / (assign4330_e2919 * assign4330_e2919));
        locals.var_vmaxe_dn10 = ((((locals.var_vmax0_dn10 * locals.var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * locals.var_t1_dn10) + (((0.1 * locals.var_t1_dn10) * locals.var_t1) + (assign4330_e2910 * locals.var_t1_dn10))) - (locals.var_mks_vtmp * (-locals.var_t1_dn10))))) / (assign4330_e2919 * assign4330_e2919));
        locals.var_vmaxe_dn11 = ((((locals.var_vmax0_dn11 * locals.var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * locals.var_t1_dn11) + (((0.1 * locals.var_t1_dn11) * locals.var_t1) + (assign4330_e2910 * locals.var_t1_dn11))) - (locals.var_mks_vtmp * (-locals.var_t1_dn11))))) / (assign4330_e2919 * assign4330_e2919));
        locals.var_vmaxe_dn12 = ((((locals.var_vmax0_dn12 * locals.var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * locals.var_t1_dn12) + (((0.1 * locals.var_t1_dn12) * locals.var_t1) + (assign4330_e2910 * locals.var_t1_dn12))) - (locals.var_mks_vtmp * (-locals.var_t1_dn12))))) / (assign4330_e2919 * assign4330_e2919));
        locals.var_vmaxe_dn17 = ((((locals.var_vmax0_dn17 * locals.var_mks_vmax) * assign4330_e2919) - (assign4330_e2902 * (((0.4 * locals.var_t1_dn17) + (((0.1 * locals.var_t1_dn17) * locals.var_t1) + (assign4330_e2910 * locals.var_t1_dn17))) - (locals.var_mks_vtmp * (-locals.var_t1_dn17))))) / (assign4330_e2919 * assign4330_e2919));

        let assign4340_e2922: f64 = (locals.var_eg).sqrt();
        locals.var_egp12 = assign4340_e2922;
        locals.var_egp12_dn0 = (locals.var_eg_dn0 / (2.0 * assign4340_e2922));
        locals.var_egp12_dn2 = (locals.var_eg_dn2 / (2.0 * assign4340_e2922));
        locals.var_egp12_dn6 = (locals.var_eg_dn6 / (2.0 * assign4340_e2922));
        locals.var_egp12_dn7 = (locals.var_eg_dn7 / (2.0 * assign4340_e2922));
        locals.var_egp12_dn10 = (locals.var_eg_dn10 / (2.0 * assign4340_e2922));
        locals.var_egp12_dn11 = (locals.var_eg_dn11 / (2.0 * assign4340_e2922));
        locals.var_egp12_dn12 = (locals.var_eg_dn12 / (2.0 * assign4340_e2922));
        locals.var_egp12_dn17 = (locals.var_eg_dn17 / (2.0 * assign4340_e2922));

        let assign4350_e2925: f64 = (locals.var_eg * locals.var_egp12);
        locals.var_egp32 = assign4350_e2925;
        locals.var_egp32_dn0 = ((locals.var_eg_dn0 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn0));
        locals.var_egp32_dn2 = ((locals.var_eg_dn2 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn2));
        locals.var_egp32_dn6 = ((locals.var_eg_dn6 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn6));
        locals.var_egp32_dn7 = ((locals.var_eg_dn7 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn7));
        locals.var_egp32_dn10 = ((locals.var_eg_dn10 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn10));
        locals.var_egp32_dn11 = ((locals.var_eg_dn11 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn11));
        locals.var_egp32_dn12 = ((locals.var_eg_dn12 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn12));
        locals.var_egp32_dn17 = ((locals.var_eg_dn17 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn17));

        let assign4360_e2928: f64 = (10400000000.0 / 1e-6);
        let assign4360_e2931: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign4360_e2933: f64 = (assign4360_e2931).powf(1.5);
        let assign4360_e2934: f64 = (assign4360_e2928 * assign4360_e2933);
        let assign4360_e2936: f64 = (-locals.var_eg);
        let assign4360_e2938: f64 = (assign4360_e2936 / 2.0);
        let assign4360_e2940: f64 = (assign4360_e2938 * locals.var_beta);
        let assign4360_e2943: f64 = (locals.var_egtnom / 2.0);
        let assign4360_e2945: f64 = (assign4360_e2943 * locals.var_betatnom);
        let assign4360_e2946: f64 = (assign4360_e2940 + assign4360_e2945);
        let assign4360_e2947: f64 = (assign4360_e2946).exp();
        let assign4360_e2948: f64 = (assign4360_e2934 * assign4360_e2947);
        locals.var_nin = assign4360_e2948;
        locals.var_nin_dn0 = (assign4360_e2934 * (assign4360_e2947 * (((-locals.var_eg_dn0) / 2.0) * locals.var_beta)));
        locals.var_nin_dn2 = (assign4360_e2934 * (assign4360_e2947 * (((-locals.var_eg_dn2) / 2.0) * locals.var_beta)));
        locals.var_nin_dn6 = (assign4360_e2934 * (assign4360_e2947 * (((-locals.var_eg_dn6) / 2.0) * locals.var_beta)));
        locals.var_nin_dn7 = (assign4360_e2934 * (assign4360_e2947 * (((-locals.var_eg_dn7) / 2.0) * locals.var_beta)));
        locals.var_nin_dn10 = (((assign4360_e2928 * if 0.0 == 0.0 && ((1.5) as f64).is_finite() && ((1.5) as f64).fract() == 0.0 { if 1.5 == 0.0 { 0.0 } else { (1.5 * ((assign4360_e2931).powf(1.5 - 1.0) * (locals.var_ttemp_dn10 / locals.var_uc_tnom))) } } else { (assign4360_e2933 * (1.5 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign4360_e2931))) }) * assign4360_e2947) + (assign4360_e2934 * (assign4360_e2947 * ((((-locals.var_eg_dn10) / 2.0) * locals.var_beta) + (assign4360_e2938 * locals.var_beta_dn10)))));
        locals.var_nin_dn11 = (assign4360_e2934 * (assign4360_e2947 * (((-locals.var_eg_dn11) / 2.0) * locals.var_beta)));
        locals.var_nin_dn12 = (assign4360_e2934 * (assign4360_e2947 * (((-locals.var_eg_dn12) / 2.0) * locals.var_beta)));
        locals.var_nin_dn17 = (assign4360_e2934 * (assign4360_e2947 * (((-locals.var_eg_dn17) / 2.0) * locals.var_beta)));

        let assign4370_e2951: f64 = (locals.var_beta_inv).sqrt();
        let assign4370_e2952: f64 = (locals.var_costi00 * assign4370_e2951);
        locals.var_costi0 = assign4370_e2952;
        locals.var_costi0_dn0 = 0.0;
        locals.var_costi0_dn2 = 0.0;
        locals.var_costi0_dn6 = 0.0;
        locals.var_costi0_dn7 = 0.0;
        locals.var_costi0_dn10 = (locals.var_costi00 * (locals.var_beta_inv_dn10 / (2.0 * assign4370_e2951)));
        locals.var_costi0_dn11 = 0.0;
        locals.var_costi0_dn12 = 0.0;
        locals.var_costi0_dn17 = 0.0;

        let assign4380_e2955: f64 = (locals.var_costi0 * locals.var_costi0);
        locals.var_costi0_p2 = assign4380_e2955;
        locals.var_costi0_p2_dn0 = ((locals.var_costi0_dn0 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn0));
        locals.var_costi0_p2_dn2 = ((locals.var_costi0_dn2 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn2));
        locals.var_costi0_p2_dn6 = ((locals.var_costi0_dn6 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn6));
        locals.var_costi0_p2_dn7 = ((locals.var_costi0_dn7 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn7));
        locals.var_costi0_p2_dn10 = ((locals.var_costi0_dn10 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn10));
        locals.var_costi0_p2_dn11 = ((locals.var_costi0_dn11 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn11));
        locals.var_costi0_p2_dn12 = ((locals.var_costi0_dn12 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn12));
        locals.var_costi0_p2_dn17 = ((locals.var_costi0_dn17 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn17));

        let assign4390_e2958: f64 = (locals.var_nin * locals.var_nin);
        let assign4390_e2960: f64 = (assign4390_e2958 * locals.var_nsti_p2);
        locals.var_costi1 = assign4390_e2960;
        locals.var_costi1_dn0 = (((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_nsti_p2);
        locals.var_costi1_dn2 = (((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_nsti_p2);
        locals.var_costi1_dn6 = (((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_nsti_p2);
        locals.var_costi1_dn7 = (((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_nsti_p2);
        locals.var_costi1_dn10 = (((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_nsti_p2);
        locals.var_costi1_dn11 = (((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_nsti_p2);
        locals.var_costi1_dn12 = (((locals.var_nin_dn12 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn12)) * locals.var_nsti_p2);
        locals.var_costi1_dn17 = (((locals.var_nin_dn17 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn17)) * locals.var_nsti_p2);

        let assign4400_e2964: f64 = (2.0 * p.p56);
        let assign4400_e2965: f64 = (locals.var_lgate - assign4400_e2964);
        locals.var_lch = assign4400_e2965;
        locals.var_lch_dn0 = 0.0;
        locals.var_lch_dn2 = 0.0;
        locals.var_lch_dn6 = 0.0;
        locals.var_lch_dn7 = 0.0;
        locals.var_lch_dn10 = 0.0;
        locals.var_lch_dn11 = 0.0;
        locals.var_lch_dn12 = 0.0;
        locals.var_lch_dn17 = 0.0;

        let assign4410_e2968: f64 = if locals.var_subversion > 3.0 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign4410_e2968;

        let (assign4420_e2979, assign4420_e2979_d_n0, assign4420_e2979_d_n2, assign4420_e2979_d_n6, assign4420_e2979_d_n7, assign4420_e2979_d_n10, assign4420_e2979_d_n11, assign4420_e2979_d_n12, assign4420_e2979_d_n17,) = {
    if (locals.var_guard42 != 0.0) {
        let assign4420_e2972: f64 = (2.0 * locals.var_beta_inv);
        let assign4420_e2975: f64 = (locals.var_nsub / locals.var_nin);
        let assign4420_e2976: f64 = (assign4420_e2975).ln();
        let assign4420_e2977: f64 = (assign4420_e2972 * assign4420_e2976);
        (assign4420_e2977, (assign4420_e2972 * ((((locals.var_nsub_dn0 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign4420_e2975)), (assign4420_e2972 * ((((locals.var_nsub_dn2 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign4420_e2975)), (assign4420_e2972 * ((((locals.var_nsub_dn6 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign4420_e2975)), (assign4420_e2972 * ((((locals.var_nsub_dn7 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign4420_e2975)), (((2.0 * locals.var_beta_inv_dn10) * assign4420_e2976) + (assign4420_e2972 * ((((locals.var_nsub_dn10 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign4420_e2975))), (assign4420_e2972 * ((((locals.var_nsub_dn11 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign4420_e2975)), (assign4420_e2972 * ((((locals.var_nsub_dn12 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn12)) / (locals.var_nin * locals.var_nin)) / assign4420_e2975)), (assign4420_e2972 * ((((locals.var_nsub_dn17 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn17)) / (locals.var_nin * locals.var_nin)) / assign4420_e2975)),)
    } else {
        (locals.var_pb2, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn10, locals.var_pb2_dn11, locals.var_pb2_dn12, locals.var_pb2_dn17,)
    }
};
        locals.var_pb2 = assign4420_e2979;
        locals.var_pb2_dn0 = assign4420_e2979_d_n0;
        locals.var_pb2_dn2 = assign4420_e2979_d_n2;
        locals.var_pb2_dn6 = assign4420_e2979_d_n6;
        locals.var_pb2_dn7 = assign4420_e2979_d_n7;
        locals.var_pb2_dn10 = assign4420_e2979_d_n10;
        locals.var_pb2_dn11 = assign4420_e2979_d_n11;
        locals.var_pb2_dn12 = assign4420_e2979_d_n12;
        locals.var_pb2_dn17 = assign4420_e2979_d_n17;

        let (assign4430_e2991, assign4430_e2991_d_n0, assign4430_e2991_d_n2, assign4430_e2991_d_n6, assign4430_e2991_d_n7, assign4430_e2991_d_n10, assign4430_e2991_d_n11, assign4430_e2991_d_n12, assign4430_e2991_d_n17,) = {
    if (locals.var_guard42 == 0.0) {
        let assign4430_e2984: f64 = (2.0 * locals.var_beta_inv);
        let assign4430_e2987: f64 = (locals.var_uc_nsubs / locals.var_nin);
        let assign4430_e2988: f64 = (assign4430_e2987).ln();
        let assign4430_e2989: f64 = (assign4430_e2984 * assign4430_e2988);
        (assign4430_e2989, (assign4430_e2984 * ((((locals.var_uc_nsubs_dn0 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign4430_e2987)), (assign4430_e2984 * ((((locals.var_uc_nsubs_dn2 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign4430_e2987)), (assign4430_e2984 * ((((locals.var_uc_nsubs_dn6 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign4430_e2987)), (assign4430_e2984 * ((((locals.var_uc_nsubs_dn7 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign4430_e2987)), (((2.0 * locals.var_beta_inv_dn10) * assign4430_e2988) + (assign4430_e2984 * ((((locals.var_uc_nsubs_dn10 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign4430_e2987))), (assign4430_e2984 * ((((locals.var_uc_nsubs_dn11 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign4430_e2987)), (assign4430_e2984 * ((((locals.var_uc_nsubs_dn12 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn12)) / (locals.var_nin * locals.var_nin)) / assign4430_e2987)), (assign4430_e2984 * ((((locals.var_uc_nsubs_dn17 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn17)) / (locals.var_nin * locals.var_nin)) / assign4430_e2987)),)
    } else {
        (locals.var_pb2, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn10, locals.var_pb2_dn11, locals.var_pb2_dn12, locals.var_pb2_dn17,)
    }
};
        locals.var_pb2 = assign4430_e2991;
        locals.var_pb2_dn0 = assign4430_e2991_d_n0;
        locals.var_pb2_dn2 = assign4430_e2991_d_n2;
        locals.var_pb2_dn6 = assign4430_e2991_d_n6;
        locals.var_pb2_dn7 = assign4430_e2991_d_n7;
        locals.var_pb2_dn10 = assign4430_e2991_d_n10;
        locals.var_pb2_dn11 = assign4430_e2991_d_n11;
        locals.var_pb2_dn12 = assign4430_e2991_d_n12;
        locals.var_pb2_dn17 = assign4430_e2991_d_n17;

        let assign4440_e2994: f64 = (1.034943e-10 / locals.var_q_nsub);
        let assign4440_e2996: f64 = (assign4440_e2994 * locals.var_beta_inv);
        let assign4440_e2997: f64 = (assign4440_e2996).sqrt();
        locals.var_ldby = assign4440_e2997;
        locals.var_ldby_dn0 = (((-((1.034943e-10 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign4440_e2997));
        locals.var_ldby_dn2 = (((-((1.034943e-10 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign4440_e2997));
        locals.var_ldby_dn6 = (((-((1.034943e-10 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign4440_e2997));
        locals.var_ldby_dn7 = (((-((1.034943e-10 * locals.var_q_nsub_dn7) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign4440_e2997));
        locals.var_ldby_dn10 = ((((-((1.034943e-10 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) + (assign4440_e2994 * locals.var_beta_inv_dn10)) / (2.0 * assign4440_e2997));
        locals.var_ldby_dn11 = (((-((1.034943e-10 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign4440_e2997));
        locals.var_ldby_dn12 = (((-((1.034943e-10 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign4440_e2997));
        locals.var_ldby_dn17 = (((-((1.034943e-10 * locals.var_q_nsub_dn17) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign4440_e2997));

        let assign4450_e3000: f64 = (locals.var_q_nsub * 1.414213562373095);
        let assign4450_e3002: f64 = (assign4450_e3000 * locals.var_ldby);
        locals.var_cnst0soi = assign4450_e3002;
        locals.var_cnst0soi_dn0 = (((locals.var_q_nsub_dn0 * 1.414213562373095) * locals.var_ldby) + (assign4450_e3000 * locals.var_ldby_dn0));
        locals.var_cnst0soi_dn2 = (((locals.var_q_nsub_dn2 * 1.414213562373095) * locals.var_ldby) + (assign4450_e3000 * locals.var_ldby_dn2));
        locals.var_cnst0soi_dn6 = (((locals.var_q_nsub_dn6 * 1.414213562373095) * locals.var_ldby) + (assign4450_e3000 * locals.var_ldby_dn6));
        locals.var_cnst0soi_dn7 = (((locals.var_q_nsub_dn7 * 1.414213562373095) * locals.var_ldby) + (assign4450_e3000 * locals.var_ldby_dn7));
        locals.var_cnst0soi_dn10 = (((locals.var_q_nsub_dn10 * 1.414213562373095) * locals.var_ldby) + (assign4450_e3000 * locals.var_ldby_dn10));
        locals.var_cnst0soi_dn11 = (((locals.var_q_nsub_dn11 * 1.414213562373095) * locals.var_ldby) + (assign4450_e3000 * locals.var_ldby_dn11));
        locals.var_cnst0soi_dn12 = (((locals.var_q_nsub_dn12 * 1.414213562373095) * locals.var_ldby) + (assign4450_e3000 * locals.var_ldby_dn12));
        locals.var_cnst0soi_dn17 = (((locals.var_q_nsub_dn17 * 1.414213562373095) * locals.var_ldby) + (assign4450_e3000 * locals.var_ldby_dn17));

        let assign4460_e3005: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign4460_e3005;

        let (assign4470_e3009, assign4470_e3009_d_n10,) = {
    if (locals.var_guard43 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_cnst0bulk, locals.var_cnst0bulk_dn10,)
    }
};
        locals.var_cnst0bulk = assign4470_e3009;
        locals.var_cnst0bulk_dn10 = assign4470_e3009_d_n10;

        let (assign4480_e3013, assign4480_e3013_d_n0, assign4480_e3013_d_n2, assign4480_e3013_d_n6, assign4480_e3013_d_n7, assign4480_e3013_d_n10, assign4480_e3013_d_n11, assign4480_e3013_d_n12, assign4480_e3013_d_n17,) = {
    if (locals.var_guard43 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cnst1bulk, locals.var_cnst1bulk_dn0, locals.var_cnst1bulk_dn2, locals.var_cnst1bulk_dn6, locals.var_cnst1bulk_dn7, locals.var_cnst1bulk_dn10, locals.var_cnst1bulk_dn11, locals.var_cnst1bulk_dn12, locals.var_cnst1bulk_dn17,)
    }
};
        locals.var_cnst1bulk = assign4480_e3013;
        locals.var_cnst1bulk_dn0 = assign4480_e3013_d_n0;
        locals.var_cnst1bulk_dn2 = assign4480_e3013_d_n2;
        locals.var_cnst1bulk_dn6 = assign4480_e3013_d_n6;
        locals.var_cnst1bulk_dn7 = assign4480_e3013_d_n7;
        locals.var_cnst1bulk_dn10 = assign4480_e3013_d_n10;
        locals.var_cnst1bulk_dn11 = assign4480_e3013_d_n11;
        locals.var_cnst1bulk_dn12 = assign4480_e3013_d_n12;
        locals.var_cnst1bulk_dn17 = assign4480_e3013_d_n17;

        let (assign4490_e3019, assign4490_e3019_d_n0, assign4490_e3019_d_n2, assign4490_e3019_d_n6, assign4490_e3019_d_n7, assign4490_e3019_d_n10, assign4490_e3019_d_n11, assign4490_e3019_d_n12, assign4490_e3019_d_n17,) = {
    if (locals.var_guard43 != 0.0) {
        let assign4490_e3017: f64 = (locals.var_nin / locals.var_nsub);
        (assign4490_e3017, (((locals.var_nin_dn0 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn2 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn6 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn7 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn10 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn11 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn12 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn12)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn17 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn17)) / (locals.var_nsub * locals.var_nsub)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign4490_e3019;
        locals.var_t1_dn0 = assign4490_e3019_d_n0;
        locals.var_t1_dn2 = assign4490_e3019_d_n2;
        locals.var_t1_dn6 = assign4490_e3019_d_n6;
        locals.var_t1_dn7 = assign4490_e3019_d_n7;
        locals.var_t1_dn10 = assign4490_e3019_d_n10;
        locals.var_t1_dn11 = assign4490_e3019_d_n11;
        locals.var_t1_dn12 = assign4490_e3019_d_n12;
        locals.var_t1_dn17 = assign4490_e3019_d_n17;

        let (assign4500_e3029, assign4500_e3029_d_n10,) = {
    if (locals.var_guard43 == 0.0) {
        let assign4500_e3024: f64 = (2.0 * locals.var_c0bulk);
        let assign4500_e3026: f64 = (assign4500_e3024 * locals.var_beta_inv);
        let assign4500_e3027: f64 = (assign4500_e3026).sqrt();
        (assign4500_e3027, ((assign4500_e3024 * locals.var_beta_inv_dn10) / (2.0 * assign4500_e3027)),)
    } else {
        (locals.var_cnst0bulk, locals.var_cnst0bulk_dn10,)
    }
};
        locals.var_cnst0bulk = assign4500_e3029;
        locals.var_cnst0bulk_dn10 = assign4500_e3029_d_n10;

        let (assign4510_e3036, assign4510_e3036_d_n0, assign4510_e3036_d_n2, assign4510_e3036_d_n6, assign4510_e3036_d_n7, assign4510_e3036_d_n10, assign4510_e3036_d_n11, assign4510_e3036_d_n12, assign4510_e3036_d_n17,) = {
    if (locals.var_guard43 == 0.0) {
        let assign4510_e3034: f64 = (locals.var_nin / locals.var_mks_nsubb);
        (assign4510_e3034, (locals.var_nin_dn0 / locals.var_mks_nsubb), (locals.var_nin_dn2 / locals.var_mks_nsubb), (locals.var_nin_dn6 / locals.var_mks_nsubb), (locals.var_nin_dn7 / locals.var_mks_nsubb), (locals.var_nin_dn10 / locals.var_mks_nsubb), (locals.var_nin_dn11 / locals.var_mks_nsubb), (locals.var_nin_dn12 / locals.var_mks_nsubb), (locals.var_nin_dn17 / locals.var_mks_nsubb),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign4510_e3036;
        locals.var_t1_dn0 = assign4510_e3036_d_n0;
        locals.var_t1_dn2 = assign4510_e3036_d_n2;
        locals.var_t1_dn6 = assign4510_e3036_d_n6;
        locals.var_t1_dn7 = assign4510_e3036_d_n7;
        locals.var_t1_dn10 = assign4510_e3036_d_n10;
        locals.var_t1_dn11 = assign4510_e3036_d_n11;
        locals.var_t1_dn12 = assign4510_e3036_d_n12;
        locals.var_t1_dn17 = assign4510_e3036_d_n17;

        let (assign4520_e3043, assign4520_e3043_d_n0, assign4520_e3043_d_n2, assign4520_e3043_d_n6, assign4520_e3043_d_n7, assign4520_e3043_d_n10, assign4520_e3043_d_n11, assign4520_e3043_d_n12, assign4520_e3043_d_n17,) = {
    if (locals.var_guard43 == 0.0) {
        let assign4520_e3041: f64 = (locals.var_t1 * locals.var_t1);
        (assign4520_e3041, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)), ((locals.var_t1_dn17 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn17)),)
    } else {
        (locals.var_cnst1bulk, locals.var_cnst1bulk_dn0, locals.var_cnst1bulk_dn2, locals.var_cnst1bulk_dn6, locals.var_cnst1bulk_dn7, locals.var_cnst1bulk_dn10, locals.var_cnst1bulk_dn11, locals.var_cnst1bulk_dn12, locals.var_cnst1bulk_dn17,)
    }
};
        locals.var_cnst1bulk = assign4520_e3043;
        locals.var_cnst1bulk_dn0 = assign4520_e3043_d_n0;
        locals.var_cnst1bulk_dn2 = assign4520_e3043_d_n2;
        locals.var_cnst1bulk_dn6 = assign4520_e3043_d_n6;
        locals.var_cnst1bulk_dn7 = assign4520_e3043_d_n7;
        locals.var_cnst1bulk_dn10 = assign4520_e3043_d_n10;
        locals.var_cnst1bulk_dn11 = assign4520_e3043_d_n11;
        locals.var_cnst1bulk_dn12 = assign4520_e3043_d_n12;
        locals.var_cnst1bulk_dn17 = assign4520_e3043_d_n17;

        let (assign4530_e3050, assign4530_e3050_d_n0, assign4530_e3050_d_n2, assign4530_e3050_d_n6, assign4530_e3050_d_n7, assign4530_e3050_d_n10, assign4530_e3050_d_n11, assign4530_e3050_d_n12, assign4530_e3050_d_n17,) = {
    if (locals.var_guard43 == 0.0) {
        let assign4530_e3048: f64 = (locals.var_nin / locals.var_uc_nsubs);
        (assign4530_e3048, (((locals.var_nin_dn0 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_nin_dn2 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_nin_dn6 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_nin_dn7 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_nin_dn10 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_nin_dn11 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_nin_dn12 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_nin_dn17 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign4530_e3050;
        locals.var_t1_dn0 = assign4530_e3050_d_n0;
        locals.var_t1_dn2 = assign4530_e3050_d_n2;
        locals.var_t1_dn6 = assign4530_e3050_d_n6;
        locals.var_t1_dn7 = assign4530_e3050_d_n7;
        locals.var_t1_dn10 = assign4530_e3050_d_n10;
        locals.var_t1_dn11 = assign4530_e3050_d_n11;
        locals.var_t1_dn12 = assign4530_e3050_d_n12;
        locals.var_t1_dn17 = assign4530_e3050_d_n17;

        let assign4540_e3053: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_cnst1soi = assign4540_e3053;
        locals.var_cnst1soi_dn0 = ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0));
        locals.var_cnst1soi_dn2 = ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2));
        locals.var_cnst1soi_dn6 = ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6));
        locals.var_cnst1soi_dn7 = ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7));
        locals.var_cnst1soi_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_cnst1soi_dn11 = ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11));
        locals.var_cnst1soi_dn12 = ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12));
        locals.var_cnst1soi_dn17 = ((locals.var_t1_dn17 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn17));

        let assign4550_e3057: f64 = (1.034943e-10 / locals.var_q_nsub);
        let assign4550_e3059: f64 = (assign4550_e3057 / locals.var_beta);
        let assign4550_e3060: f64 = (2.0 * assign4550_e3059);
        let assign4550_e3061: f64 = (assign4550_e3060).sqrt();
        locals.var_c_w_soi = assign4550_e3061;
        locals.var_c_w_soi_dn0 = ((2.0 * ((-((1.034943e-10 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))) / locals.var_beta)) / (2.0 * assign4550_e3061));
        locals.var_c_w_soi_dn2 = ((2.0 * ((-((1.034943e-10 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))) / locals.var_beta)) / (2.0 * assign4550_e3061));
        locals.var_c_w_soi_dn6 = ((2.0 * ((-((1.034943e-10 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))) / locals.var_beta)) / (2.0 * assign4550_e3061));
        locals.var_c_w_soi_dn7 = ((2.0 * ((-((1.034943e-10 * locals.var_q_nsub_dn7) / (locals.var_q_nsub * locals.var_q_nsub))) / locals.var_beta)) / (2.0 * assign4550_e3061));
        locals.var_c_w_soi_dn10 = ((2.0 * ((((-((1.034943e-10 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta) - (assign4550_e3057 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta))) / (2.0 * assign4550_e3061));
        locals.var_c_w_soi_dn11 = ((2.0 * ((-((1.034943e-10 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))) / locals.var_beta)) / (2.0 * assign4550_e3061));
        locals.var_c_w_soi_dn12 = ((2.0 * ((-((1.034943e-10 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))) / locals.var_beta)) / (2.0 * assign4550_e3061));
        locals.var_c_w_soi_dn17 = ((2.0 * ((-((1.034943e-10 * locals.var_q_nsub_dn17) / (locals.var_q_nsub * locals.var_q_nsub))) / locals.var_beta)) / (2.0 * assign4550_e3061));

        let assign4560_e3064: f64 = (2.0 * 1.034943e-10);
        let assign4560_e3066: f64 = (assign4560_e3064 / 1.6021918e-19);
        let assign4560_e3068: f64 = (assign4560_e3066 / locals.var_uc_nsubs);
        locals.var_cnst_2esi_q_nsubs = assign4560_e3068;
        locals.var_cnst_2esi_q_nsubs_dn0 = (-((assign4560_e3066 * locals.var_uc_nsubs_dn0) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));
        locals.var_cnst_2esi_q_nsubs_dn2 = (-((assign4560_e3066 * locals.var_uc_nsubs_dn2) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));
        locals.var_cnst_2esi_q_nsubs_dn6 = (-((assign4560_e3066 * locals.var_uc_nsubs_dn6) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));
        locals.var_cnst_2esi_q_nsubs_dn7 = (-((assign4560_e3066 * locals.var_uc_nsubs_dn7) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));
        locals.var_cnst_2esi_q_nsubs_dn10 = (-((assign4560_e3066 * locals.var_uc_nsubs_dn10) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));
        locals.var_cnst_2esi_q_nsubs_dn11 = (-((assign4560_e3066 * locals.var_uc_nsubs_dn11) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));
        locals.var_cnst_2esi_q_nsubs_dn12 = (-((assign4560_e3066 * locals.var_uc_nsubs_dn12) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));
        locals.var_cnst_2esi_q_nsubs_dn17 = (-((assign4560_e3066 * locals.var_uc_nsubs_dn17) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));

        let assign4570_e3071: f64 = (2.0 * 1.034943e-10);
        let assign4570_e3073: f64 = (assign4570_e3071 / 1.6021918e-19);
        let assign4570_e3075: f64 = (assign4570_e3073 * locals.var_pb2);
        let assign4570_e3077: f64 = (assign4570_e3075 / locals.var_uc_nsubs);
        let assign4570_e3078: f64 = (assign4570_e3077).sqrt();
        locals.var_wdsoi_ini = assign4570_e3078;

        let assign4650_e3103: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard48 = assign4650_e3103;

        let (assign4660_e3107,) = {
    if (locals.var_guard48 != 0.0) {
        (0.4,)
    } else {
        (locals.var_vbs_bnd,)
    }
};
        locals.var_vbs_bnd = assign4660_e3107;

        let (assign4670_e3111,) = {
    if (locals.var_guard48 != 0.0) {
        (0.8,)
    } else {
        (locals.var_vbs_max,)
    }
};
        locals.var_vbs_max = assign4670_e3111;

        let (assign4680_e3116,) = {
    if (locals.var_guard48 == 0.0) {
        (0.8,)
    } else {
        (locals.var_vbs_bnd,)
    }
};
        locals.var_vbs_bnd = assign4680_e3116;

        let (assign4690_e3121,) = {
    if (locals.var_guard48 == 0.0) {
        (1.2,)
    } else {
        (locals.var_vbs_max,)
    }
};
        locals.var_vbs_max = assign4690_e3121;

    }

    pub(super) fn stamp_transient_block_8(
        locals: &mut StampLocals,
    ) {
        let assign4700_e3125: f64 = (locals.var_vbs_max * 0.5);
        let assign4700_e3126: f64 = if locals.var_vbs_bnd > assign4700_e3125 { 1.0 } else { 0.0 };
        locals.var_guard49 = assign4700_e3126;

        let (assign4710_e3132,) = {
    if (locals.var_guard49 != 0.0) {
        let assign4710_e3130: f64 = (0.5 * locals.var_vbs_max);
        (assign4710_e3130,)
    } else {
        (locals.var_vbs_bnd,)
    }
};
        locals.var_vbs_bnd = assign4710_e3132;

        let assign4720_e3135: f64 = if locals.var_vbs > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard50 = assign4720_e3135;

        let (assign4730_e3141, assign4730_e3141_d_n0, assign4730_e3141_d_n2, assign4730_e3141_d_n6, assign4730_e3141_d_n7, assign4730_e3141_d_n10, assign4730_e3141_d_n11, assign4730_e3141_d_n12, assign4730_e3141_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign4730_e3139: f64 = (locals.var_vbs - locals.var_vbs_bnd);
        (assign4730_e3139, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign4730_e3141;
        locals.var_t2_dn0 = assign4730_e3141_d_n0;
        locals.var_t2_dn2 = assign4730_e3141_d_n2;
        locals.var_t2_dn6 = assign4730_e3141_d_n6;
        locals.var_t2_dn7 = assign4730_e3141_d_n7;
        locals.var_t2_dn10 = assign4730_e3141_d_n10;
        locals.var_t2_dn11 = assign4730_e3141_d_n11;
        locals.var_t2_dn12 = assign4730_e3141_d_n12;
        locals.var_t2_dn17 = assign4730_e3141_d_n17;

        let (assign4740_e3147, assign4740_e3147_d_n0, assign4740_e3147_d_n2, assign4740_e3147_d_n6, assign4740_e3147_d_n7, assign4740_e3147_d_n10, assign4740_e3147_d_n11, assign4740_e3147_d_n12, assign4740_e3147_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign4740_e3145: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign4740_e3145, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign4740_e3147;
        locals.var_t3_dn0 = assign4740_e3147_d_n0;
        locals.var_t3_dn2 = assign4740_e3147_d_n2;
        locals.var_t3_dn6 = assign4740_e3147_d_n6;
        locals.var_t3_dn7 = assign4740_e3147_d_n7;
        locals.var_t3_dn10 = assign4740_e3147_d_n10;
        locals.var_t3_dn11 = assign4740_e3147_d_n11;
        locals.var_t3_dn12 = assign4740_e3147_d_n12;
        locals.var_t3_dn17 = assign4740_e3147_d_n17;

        let (assign4750_e3153, assign4750_e3153_d_n0, assign4750_e3153_d_n2, assign4750_e3153_d_n6, assign4750_e3153_d_n7, assign4750_e3153_d_n10, assign4750_e3153_d_n11, assign4750_e3153_d_n12, assign4750_e3153_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign4750_e3151: f64 = (locals.var_t2 * locals.var_t2);
        (assign4750_e3151, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)), ((locals.var_t2_dn17 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign4750_e3153;
        locals.var_x2_dn0 = assign4750_e3153_d_n0;
        locals.var_x2_dn2 = assign4750_e3153_d_n2;
        locals.var_x2_dn6 = assign4750_e3153_d_n6;
        locals.var_x2_dn7 = assign4750_e3153_d_n7;
        locals.var_x2_dn10 = assign4750_e3153_d_n10;
        locals.var_x2_dn11 = assign4750_e3153_d_n11;
        locals.var_x2_dn12 = assign4750_e3153_d_n12;
        locals.var_x2_dn17 = assign4750_e3153_d_n17;

        let (assign4760_e3159, assign4760_e3159_d_n0, assign4760_e3159_d_n2, assign4760_e3159_d_n6, assign4760_e3159_d_n7, assign4760_e3159_d_n10, assign4760_e3159_d_n11, assign4760_e3159_d_n12, assign4760_e3159_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign4760_e3157: f64 = (locals.var_t3 * locals.var_t3);
        (assign4760_e3157, ((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)), ((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)), ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)), ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)), ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)), ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)), ((locals.var_t3_dn12 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn12)), ((locals.var_t3_dn17 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn17)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign4760_e3159;
        locals.var_xmax2_dn0 = assign4760_e3159_d_n0;
        locals.var_xmax2_dn2 = assign4760_e3159_d_n2;
        locals.var_xmax2_dn6 = assign4760_e3159_d_n6;
        locals.var_xmax2_dn7 = assign4760_e3159_d_n7;
        locals.var_xmax2_dn10 = assign4760_e3159_d_n10;
        locals.var_xmax2_dn11 = assign4760_e3159_d_n11;
        locals.var_xmax2_dn12 = assign4760_e3159_d_n12;
        locals.var_xmax2_dn17 = assign4760_e3159_d_n17;

        let (assign4770_e3163, assign4770_e3163_d_n0, assign4770_e3163_d_n2, assign4770_e3163_d_n6, assign4770_e3163_d_n7, assign4770_e3163_d_n10, assign4770_e3163_d_n11, assign4770_e3163_d_n12, assign4770_e3163_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign4770_e3163;
        locals.var_xp_dn0 = assign4770_e3163_d_n0;
        locals.var_xp_dn2 = assign4770_e3163_d_n2;
        locals.var_xp_dn6 = assign4770_e3163_d_n6;
        locals.var_xp_dn7 = assign4770_e3163_d_n7;
        locals.var_xp_dn10 = assign4770_e3163_d_n10;
        locals.var_xp_dn11 = assign4770_e3163_d_n11;
        locals.var_xp_dn12 = assign4770_e3163_d_n12;
        locals.var_xp_dn17 = assign4770_e3163_d_n17;

        let (assign4780_e3167, assign4780_e3167_d_n0, assign4780_e3167_d_n2, assign4780_e3167_d_n6, assign4780_e3167_d_n7, assign4780_e3167_d_n10, assign4780_e3167_d_n11, assign4780_e3167_d_n12, assign4780_e3167_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign4780_e3167;
        locals.var_xmp_dn0 = assign4780_e3167_d_n0;
        locals.var_xmp_dn2 = assign4780_e3167_d_n2;
        locals.var_xmp_dn6 = assign4780_e3167_d_n6;
        locals.var_xmp_dn7 = assign4780_e3167_d_n7;
        locals.var_xmp_dn10 = assign4780_e3167_d_n10;
        locals.var_xmp_dn11 = assign4780_e3167_d_n11;
        locals.var_xmp_dn12 = assign4780_e3167_d_n12;
        locals.var_xmp_dn17 = assign4780_e3167_d_n17;

        let (assign4790_e3171,) = {
    if (locals.var_guard50 != 0.0) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign4790_e3171;

        let (assign4800_e3175,) = {
    if (locals.var_guard50 != 0.0) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign4800_e3175;

        let (assign4810_e3179, assign4810_e3179_d_n0, assign4810_e3179_d_n2, assign4810_e3179_d_n6, assign4810_e3179_d_n7, assign4810_e3179_d_n10, assign4810_e3179_d_n11, assign4810_e3179_d_n12, assign4810_e3179_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign4810_e3179;
        locals.var_arg_dn0 = assign4810_e3179_d_n0;
        locals.var_arg_dn2 = assign4810_e3179_d_n2;
        locals.var_arg_dn6 = assign4810_e3179_d_n6;
        locals.var_arg_dn7 = assign4810_e3179_d_n7;
        locals.var_arg_dn10 = assign4810_e3179_d_n10;
        locals.var_arg_dn11 = assign4810_e3179_d_n11;
        locals.var_arg_dn12 = assign4810_e3179_d_n12;
        locals.var_arg_dn17 = assign4810_e3179_d_n17;

        let (assign4820_e3183, assign4820_e3183_d_n0, assign4820_e3183_d_n2, assign4820_e3183_d_n6, assign4820_e3183_d_n7, assign4820_e3183_d_n10, assign4820_e3183_d_n11, assign4820_e3183_d_n12, assign4820_e3183_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign4820_e3183;
        locals.var_dnm_dn0 = assign4820_e3183_d_n0;
        locals.var_dnm_dn2 = assign4820_e3183_d_n2;
        locals.var_dnm_dn6 = assign4820_e3183_d_n6;
        locals.var_dnm_dn7 = assign4820_e3183_d_n7;
        locals.var_dnm_dn10 = assign4820_e3183_d_n10;
        locals.var_dnm_dn11 = assign4820_e3183_d_n11;
        locals.var_dnm_dn12 = assign4820_e3183_d_n12;
        locals.var_dnm_dn17 = assign4820_e3183_d_n17;

        let (assign4830_e3189, assign4830_e3189_d_n0, assign4830_e3189_d_n2, assign4830_e3189_d_n6, assign4830_e3189_d_n7, assign4830_e3189_d_n10, assign4830_e3189_d_n11, assign4830_e3189_d_n12, assign4830_e3189_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign4830_e3187: f64 = (locals.var_xp * locals.var_x2);
        (assign4830_e3187, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign4830_e3189;
        locals.var_xp_dn0 = assign4830_e3189_d_n0;
        locals.var_xp_dn2 = assign4830_e3189_d_n2;
        locals.var_xp_dn6 = assign4830_e3189_d_n6;
        locals.var_xp_dn7 = assign4830_e3189_d_n7;
        locals.var_xp_dn10 = assign4830_e3189_d_n10;
        locals.var_xp_dn11 = assign4830_e3189_d_n11;
        locals.var_xp_dn12 = assign4830_e3189_d_n12;
        locals.var_xp_dn17 = assign4830_e3189_d_n17;

        let (assign4840_e3195, assign4840_e3195_d_n0, assign4840_e3195_d_n2, assign4840_e3195_d_n6, assign4840_e3195_d_n7, assign4840_e3195_d_n10, assign4840_e3195_d_n11, assign4840_e3195_d_n12, assign4840_e3195_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign4840_e3193: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign4840_e3193, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign4840_e3195;
        locals.var_xmp_dn0 = assign4840_e3195_d_n0;
        locals.var_xmp_dn2 = assign4840_e3195_d_n2;
        locals.var_xmp_dn6 = assign4840_e3195_d_n6;
        locals.var_xmp_dn7 = assign4840_e3195_d_n7;
        locals.var_xmp_dn10 = assign4840_e3195_d_n10;
        locals.var_xmp_dn11 = assign4840_e3195_d_n11;
        locals.var_xmp_dn12 = assign4840_e3195_d_n12;
        locals.var_xmp_dn17 = assign4840_e3195_d_n17;

        let (assign4850_e3201, assign4850_e3201_d_n0, assign4850_e3201_d_n2, assign4850_e3201_d_n6, assign4850_e3201_d_n7, assign4850_e3201_d_n10, assign4850_e3201_d_n11, assign4850_e3201_d_n12, assign4850_e3201_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign4850_e3199: f64 = (locals.var_xp * locals.var_x2);
        (assign4850_e3199, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign4850_e3201;
        locals.var_xp_dn0 = assign4850_e3201_d_n0;
        locals.var_xp_dn2 = assign4850_e3201_d_n2;
        locals.var_xp_dn6 = assign4850_e3201_d_n6;
        locals.var_xp_dn7 = assign4850_e3201_d_n7;
        locals.var_xp_dn10 = assign4850_e3201_d_n10;
        locals.var_xp_dn11 = assign4850_e3201_d_n11;
        locals.var_xp_dn12 = assign4850_e3201_d_n12;
        locals.var_xp_dn17 = assign4850_e3201_d_n17;

        let (assign4860_e3207, assign4860_e3207_d_n0, assign4860_e3207_d_n2, assign4860_e3207_d_n6, assign4860_e3207_d_n7, assign4860_e3207_d_n10, assign4860_e3207_d_n11, assign4860_e3207_d_n12, assign4860_e3207_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign4860_e3205: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign4860_e3205, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign4860_e3207;
        locals.var_xmp_dn0 = assign4860_e3207_d_n0;
        locals.var_xmp_dn2 = assign4860_e3207_d_n2;
        locals.var_xmp_dn6 = assign4860_e3207_d_n6;
        locals.var_xmp_dn7 = assign4860_e3207_d_n7;
        locals.var_xmp_dn10 = assign4860_e3207_d_n10;
        locals.var_xmp_dn11 = assign4860_e3207_d_n11;
        locals.var_xmp_dn12 = assign4860_e3207_d_n12;
        locals.var_xmp_dn17 = assign4860_e3207_d_n17;

        let (assign4870_e3213, assign4870_e3213_d_n0, assign4870_e3213_d_n2, assign4870_e3213_d_n6, assign4870_e3213_d_n7, assign4870_e3213_d_n10, assign4870_e3213_d_n11, assign4870_e3213_d_n12, assign4870_e3213_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign4870_e3211: f64 = (locals.var_xp * locals.var_x2);
        (assign4870_e3211, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign4870_e3213;
        locals.var_xp_dn0 = assign4870_e3213_d_n0;
        locals.var_xp_dn2 = assign4870_e3213_d_n2;
        locals.var_xp_dn6 = assign4870_e3213_d_n6;
        locals.var_xp_dn7 = assign4870_e3213_d_n7;
        locals.var_xp_dn10 = assign4870_e3213_d_n10;
        locals.var_xp_dn11 = assign4870_e3213_d_n11;
        locals.var_xp_dn12 = assign4870_e3213_d_n12;
        locals.var_xp_dn17 = assign4870_e3213_d_n17;

        let (assign4880_e3219, assign4880_e3219_d_n0, assign4880_e3219_d_n2, assign4880_e3219_d_n6, assign4880_e3219_d_n7, assign4880_e3219_d_n10, assign4880_e3219_d_n11, assign4880_e3219_d_n12, assign4880_e3219_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign4880_e3217: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign4880_e3217, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign4880_e3219;
        locals.var_xmp_dn0 = assign4880_e3219_d_n0;
        locals.var_xmp_dn2 = assign4880_e3219_d_n2;
        locals.var_xmp_dn6 = assign4880_e3219_d_n6;
        locals.var_xmp_dn7 = assign4880_e3219_d_n7;
        locals.var_xmp_dn10 = assign4880_e3219_d_n10;
        locals.var_xmp_dn11 = assign4880_e3219_d_n11;
        locals.var_xmp_dn12 = assign4880_e3219_d_n12;
        locals.var_xmp_dn17 = assign4880_e3219_d_n17;

        let (assign4890_e3225, assign4890_e3225_d_n0, assign4890_e3225_d_n2, assign4890_e3225_d_n6, assign4890_e3225_d_n7, assign4890_e3225_d_n10, assign4890_e3225_d_n11, assign4890_e3225_d_n12, assign4890_e3225_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign4890_e3223: f64 = (locals.var_xp * locals.var_x2);
        (assign4890_e3223, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign4890_e3225;
        locals.var_xp_dn0 = assign4890_e3225_d_n0;
        locals.var_xp_dn2 = assign4890_e3225_d_n2;
        locals.var_xp_dn6 = assign4890_e3225_d_n6;
        locals.var_xp_dn7 = assign4890_e3225_d_n7;
        locals.var_xp_dn10 = assign4890_e3225_d_n10;
        locals.var_xp_dn11 = assign4890_e3225_d_n11;
        locals.var_xp_dn12 = assign4890_e3225_d_n12;
        locals.var_xp_dn17 = assign4890_e3225_d_n17;

        let (assign4900_e3231, assign4900_e3231_d_n0, assign4900_e3231_d_n2, assign4900_e3231_d_n6, assign4900_e3231_d_n7, assign4900_e3231_d_n10, assign4900_e3231_d_n11, assign4900_e3231_d_n12, assign4900_e3231_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign4900_e3229: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign4900_e3229, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign4900_e3231;
        locals.var_xmp_dn0 = assign4900_e3231_d_n0;
        locals.var_xmp_dn2 = assign4900_e3231_d_n2;
        locals.var_xmp_dn6 = assign4900_e3231_d_n6;
        locals.var_xmp_dn7 = assign4900_e3231_d_n7;
        locals.var_xmp_dn10 = assign4900_e3231_d_n10;
        locals.var_xmp_dn11 = assign4900_e3231_d_n11;
        locals.var_xmp_dn12 = assign4900_e3231_d_n12;
        locals.var_xmp_dn17 = assign4900_e3231_d_n17;

        let (assign4910_e3237, assign4910_e3237_d_n0, assign4910_e3237_d_n2, assign4910_e3237_d_n6, assign4910_e3237_d_n7, assign4910_e3237_d_n10, assign4910_e3237_d_n11, assign4910_e3237_d_n12, assign4910_e3237_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign4910_e3235: f64 = (locals.var_xp + locals.var_xmp);
        (assign4910_e3235, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign4910_e3237;
        locals.var_arg_dn0 = assign4910_e3237_d_n0;
        locals.var_arg_dn2 = assign4910_e3237_d_n2;
        locals.var_arg_dn6 = assign4910_e3237_d_n6;
        locals.var_arg_dn7 = assign4910_e3237_d_n7;
        locals.var_arg_dn10 = assign4910_e3237_d_n10;
        locals.var_arg_dn11 = assign4910_e3237_d_n11;
        locals.var_arg_dn12 = assign4910_e3237_d_n12;
        locals.var_arg_dn17 = assign4910_e3237_d_n17;

        let (assign4920_e3241, assign4920_e3241_d_n0, assign4920_e3241_d_n2, assign4920_e3241_d_n6, assign4920_e3241_d_n7, assign4920_e3241_d_n10, assign4920_e3241_d_n11, assign4920_e3241_d_n12, assign4920_e3241_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign4920_e3241;
        locals.var_dnm_dn0 = assign4920_e3241_d_n0;
        locals.var_dnm_dn2 = assign4920_e3241_d_n2;
        locals.var_dnm_dn6 = assign4920_e3241_d_n6;
        locals.var_dnm_dn7 = assign4920_e3241_d_n7;
        locals.var_dnm_dn10 = assign4920_e3241_d_n10;
        locals.var_dnm_dn11 = assign4920_e3241_d_n11;
        locals.var_dnm_dn12 = assign4920_e3241_d_n12;
        locals.var_dnm_dn17 = assign4920_e3241_d_n17;

        let assign4930_e3256: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard51 = assign4930_e3256;

        let assign4940_e3259: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard52 = assign4940_e3259;

        let (assign4950_e3267,) = {
    if (((locals.var_guard50 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard52 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign4950_e3267;

        let assign4960_e3270: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard53 = assign4960_e3270;

        let (assign4970_e3281,) = {
    if ((((locals.var_guard50 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard52 == 0.0)) && (locals.var_guard53 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign4970_e3281;

        let assign4980_e3284: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard54 = assign4980_e3284;

        let (assign4990_e3298,) = {
    if (((((locals.var_guard50 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard52 == 0.0)) && (locals.var_guard53 == 0.0)) && (locals.var_guard54 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign4990_e3298;

        let assign5000_e3301: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard55 = assign5000_e3301;

        let (assign5010_e3318,) = {
    if ((((((locals.var_guard50 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_guard52 == 0.0)) && (locals.var_guard53 == 0.0)) && (locals.var_guard54 == 0.0)) && (locals.var_guard55 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign5010_e3318;

        let (assign5020_e3324,) = {
    if ((locals.var_guard50 != 0.0) && (locals.var_guard51 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign5020_e3324;

        let mut assign5030_loop_guard: usize = 0;
        while {
            let assign5030_cond_e3331: f64 = if (((locals.var_guard50 != 0.0) && (locals.var_guard51 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign5030_cond_e3331 != 0.0
        } {
            assign5030_loop_guard += 1;
            assert!(assign5030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign5030_body0_e3338, assign5030_body0_e3338_d_n0, assign5030_body0_e3338_d_n2, assign5030_body0_e3338_d_n6, assign5030_body0_e3338_d_n7, assign5030_body0_e3338_d_n10, assign5030_body0_e3338_d_n11, assign5030_body0_e3338_d_n12, assign5030_body0_e3338_d_n17,) = {
    if ((locals.var_guard50 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign5030_body0_e3336: f64 = (locals.var_dnm).sqrt();
        (assign5030_body0_e3336, (locals.var_dnm_dn0 / (2.0 * assign5030_body0_e3336)), (locals.var_dnm_dn2 / (2.0 * assign5030_body0_e3336)), (locals.var_dnm_dn6 / (2.0 * assign5030_body0_e3336)), (locals.var_dnm_dn7 / (2.0 * assign5030_body0_e3336)), (locals.var_dnm_dn10 / (2.0 * assign5030_body0_e3336)), (locals.var_dnm_dn11 / (2.0 * assign5030_body0_e3336)), (locals.var_dnm_dn12 / (2.0 * assign5030_body0_e3336)), (locals.var_dnm_dn17 / (2.0 * assign5030_body0_e3336)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign5030_body0_e3338;
            locals.var_dnm_dn0 = assign5030_body0_e3338_d_n0;
            locals.var_dnm_dn2 = assign5030_body0_e3338_d_n2;
            locals.var_dnm_dn6 = assign5030_body0_e3338_d_n6;
            locals.var_dnm_dn7 = assign5030_body0_e3338_d_n7;
            locals.var_dnm_dn10 = assign5030_body0_e3338_d_n10;
            locals.var_dnm_dn11 = assign5030_body0_e3338_d_n11;
            locals.var_dnm_dn12 = assign5030_body0_e3338_d_n12;
            locals.var_dnm_dn17 = assign5030_body0_e3338_d_n17;
            let (assign5030_body1_e3346,) = {
    if ((locals.var_guard50 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign5030_body1_e3344: f64 = (locals.var_m0 + 1.0);
        (assign5030_body1_e3344,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign5030_body1_e3346;
        }

        let (assign5040_e3359, assign5040_e3359_d_n0, assign5040_e3359_d_n2, assign5040_e3359_d_n6, assign5040_e3359_d_n7, assign5040_e3359_d_n10, assign5040_e3359_d_n11, assign5040_e3359_d_n12, assign5040_e3359_d_n17,) = {
    if ((locals.var_guard50 != 0.0) && (locals.var_guard51 == 0.0)) {
        let assign5040_e3355: f64 = (2.0 * 4.0);
        let assign5040_e3356: f64 = (1.0 / assign5040_e3355);
        let assign5040_e3357: f64 = (locals.var_dnm).powf(assign5040_e3356);
        (assign5040_e3357, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((locals.var_dnm).powf(assign5040_e3356 - 1.0) * locals.var_dnm_dn0)) } } else { (assign5040_e3357 * (assign5040_e3356 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((locals.var_dnm).powf(assign5040_e3356 - 1.0) * locals.var_dnm_dn2)) } } else { (assign5040_e3357 * (assign5040_e3356 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((locals.var_dnm).powf(assign5040_e3356 - 1.0) * locals.var_dnm_dn6)) } } else { (assign5040_e3357 * (assign5040_e3356 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((locals.var_dnm).powf(assign5040_e3356 - 1.0) * locals.var_dnm_dn7)) } } else { (assign5040_e3357 * (assign5040_e3356 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((locals.var_dnm).powf(assign5040_e3356 - 1.0) * locals.var_dnm_dn10)) } } else { (assign5040_e3357 * (assign5040_e3356 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((locals.var_dnm).powf(assign5040_e3356 - 1.0) * locals.var_dnm_dn11)) } } else { (assign5040_e3357 * (assign5040_e3356 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((locals.var_dnm).powf(assign5040_e3356 - 1.0) * locals.var_dnm_dn12)) } } else { (assign5040_e3357 * (assign5040_e3356 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign5040_e3356) as f64).is_finite() && ((assign5040_e3356) as f64).fract() == 0.0 { if assign5040_e3356 == 0.0 { 0.0 } else { (assign5040_e3356 * ((locals.var_dnm).powf(assign5040_e3356 - 1.0) * locals.var_dnm_dn17)) } } else { (assign5040_e3357 * (assign5040_e3356 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign5040_e3359;
        locals.var_dnm_dn0 = assign5040_e3359_d_n0;
        locals.var_dnm_dn2 = assign5040_e3359_d_n2;
        locals.var_dnm_dn6 = assign5040_e3359_d_n6;
        locals.var_dnm_dn7 = assign5040_e3359_d_n7;
        locals.var_dnm_dn10 = assign5040_e3359_d_n10;
        locals.var_dnm_dn11 = assign5040_e3359_d_n11;
        locals.var_dnm_dn12 = assign5040_e3359_d_n12;
        locals.var_dnm_dn17 = assign5040_e3359_d_n17;

        let (assign5050_e3365, assign5050_e3365_d_n0, assign5050_e3365_d_n2, assign5050_e3365_d_n6, assign5050_e3365_d_n7, assign5050_e3365_d_n10, assign5050_e3365_d_n11, assign5050_e3365_d_n12, assign5050_e3365_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign5050_e3363: f64 = (1.0 / locals.var_dnm);
        (assign5050_e3363, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign5050_e3365;
        locals.var_dnm_dn0 = assign5050_e3365_d_n0;
        locals.var_dnm_dn2 = assign5050_e3365_d_n2;
        locals.var_dnm_dn6 = assign5050_e3365_d_n6;
        locals.var_dnm_dn7 = assign5050_e3365_d_n7;
        locals.var_dnm_dn10 = assign5050_e3365_d_n10;
        locals.var_dnm_dn11 = assign5050_e3365_d_n11;
        locals.var_dnm_dn12 = assign5050_e3365_d_n12;
        locals.var_dnm_dn17 = assign5050_e3365_d_n17;

        let (assign5060_e3373, assign5060_e3373_d_n0, assign5060_e3373_d_n2, assign5060_e3373_d_n6, assign5060_e3373_d_n7, assign5060_e3373_d_n10, assign5060_e3373_d_n11, assign5060_e3373_d_n12, assign5060_e3373_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign5060_e3369: f64 = (locals.var_t2 * locals.var_t3);
        let assign5060_e3371: f64 = (assign5060_e3369 * locals.var_dnm);
        (assign5060_e3371, ((((locals.var_t2_dn0 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn0)) * locals.var_dnm) + (assign5060_e3369 * locals.var_dnm_dn0)), ((((locals.var_t2_dn2 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn2)) * locals.var_dnm) + (assign5060_e3369 * locals.var_dnm_dn2)), ((((locals.var_t2_dn6 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn6)) * locals.var_dnm) + (assign5060_e3369 * locals.var_dnm_dn6)), ((((locals.var_t2_dn7 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn7)) * locals.var_dnm) + (assign5060_e3369 * locals.var_dnm_dn7)), ((((locals.var_t2_dn10 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn10)) * locals.var_dnm) + (assign5060_e3369 * locals.var_dnm_dn10)), ((((locals.var_t2_dn11 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn11)) * locals.var_dnm) + (assign5060_e3369 * locals.var_dnm_dn11)), ((((locals.var_t2_dn12 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn12)) * locals.var_dnm) + (assign5060_e3369 * locals.var_dnm_dn12)), ((((locals.var_t2_dn17 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn17)) * locals.var_dnm) + (assign5060_e3369 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign5060_e3373;
        locals.var_t4_dn0 = assign5060_e3373_d_n0;
        locals.var_t4_dn2 = assign5060_e3373_d_n2;
        locals.var_t4_dn6 = assign5060_e3373_d_n6;
        locals.var_t4_dn7 = assign5060_e3373_d_n7;
        locals.var_t4_dn10 = assign5060_e3373_d_n10;
        locals.var_t4_dn11 = assign5060_e3373_d_n11;
        locals.var_t4_dn12 = assign5060_e3373_d_n12;
        locals.var_t4_dn17 = assign5060_e3373_d_n17;

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5070_e3383, assign5070_e3383_d_n0, assign5070_e3383_d_n2, assign5070_e3383_d_n6, assign5070_e3383_d_n7, assign5070_e3383_d_n10, assign5070_e3383_d_n11, assign5070_e3383_d_n12, assign5070_e3383_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign5070_e3377: f64 = (locals.var_t3 * locals.var_xmp);
        let assign5070_e3379: f64 = (assign5070_e3377 * locals.var_dnm);
        let assign5070_e3381: f64 = (assign5070_e3379 / locals.var_arg);
        (assign5070_e3381, (((((((locals.var_t3_dn0 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign5070_e3377 * locals.var_dnm_dn0)) * locals.var_arg) - (assign5070_e3379 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t3_dn2 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign5070_e3377 * locals.var_dnm_dn2)) * locals.var_arg) - (assign5070_e3379 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t3_dn6 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign5070_e3377 * locals.var_dnm_dn6)) * locals.var_arg) - (assign5070_e3379 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t3_dn7 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign5070_e3377 * locals.var_dnm_dn7)) * locals.var_arg) - (assign5070_e3379 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t3_dn10 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign5070_e3377 * locals.var_dnm_dn10)) * locals.var_arg) - (assign5070_e3379 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t3_dn11 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign5070_e3377 * locals.var_dnm_dn11)) * locals.var_arg) - (assign5070_e3379 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t3_dn12 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn12)) * locals.var_dnm) + (assign5070_e3377 * locals.var_dnm_dn12)) * locals.var_arg) - (assign5070_e3379 * locals.var_arg_dn12)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t3_dn17 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn17)) * locals.var_dnm) + (assign5070_e3377 * locals.var_dnm_dn17)) * locals.var_arg) - (assign5070_e3379 * locals.var_arg_dn17)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn17,)
    }
};
        locals.var_t8 = assign5070_e3383;
        locals.var_t8_dn0 = assign5070_e3383_d_n0;
        locals.var_t8_dn2 = assign5070_e3383_d_n2;
        locals.var_t8_dn6 = assign5070_e3383_d_n6;
        locals.var_t8_dn7 = assign5070_e3383_d_n7;
        locals.var_t8_dn10 = assign5070_e3383_d_n10;
        locals.var_t8_dn11 = assign5070_e3383_d_n11;
        locals.var_t8_dn12 = assign5070_e3383_d_n12;
        locals.var_t8_dn17 = assign5070_e3383_d_n17;

        let (assign5080_e3389, assign5080_e3389_d_n0, assign5080_e3389_d_n2, assign5080_e3389_d_n6, assign5080_e3389_d_n7, assign5080_e3389_d_n10, assign5080_e3389_d_n11, assign5080_e3389_d_n12, assign5080_e3389_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        let assign5080_e3387: f64 = (locals.var_vbs_bnd + locals.var_t4);
        (assign5080_e3387, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    } else {
        (locals.var_vbsc, locals.var_vbsc_dn0, locals.var_vbsc_dn2, locals.var_vbsc_dn6, locals.var_vbsc_dn7, locals.var_vbsc_dn10, locals.var_vbsc_dn11, locals.var_vbsc_dn12, locals.var_vbsc_dn17,)
    }
};
        locals.var_vbsc = assign5080_e3389;
        locals.var_vbsc_dn0 = assign5080_e3389_d_n0;
        locals.var_vbsc_dn2 = assign5080_e3389_d_n2;
        locals.var_vbsc_dn6 = assign5080_e3389_d_n6;
        locals.var_vbsc_dn7 = assign5080_e3389_d_n7;
        locals.var_vbsc_dn10 = assign5080_e3389_d_n10;
        locals.var_vbsc_dn11 = assign5080_e3389_d_n11;
        locals.var_vbsc_dn12 = assign5080_e3389_d_n12;
        locals.var_vbsc_dn17 = assign5080_e3389_d_n17;

        let (assign5090_e3393, assign5090_e3393_d_n0, assign5090_e3393_d_n2, assign5090_e3393_d_n6, assign5090_e3393_d_n7, assign5090_e3393_d_n10, assign5090_e3393_d_n11, assign5090_e3393_d_n12, assign5090_e3393_d_n17,) = {
    if (locals.var_guard50 != 0.0) {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn17,)
    } else {
        (locals.var_vbsc_dvbse, locals.var_vbsc_dvbse_dn0, locals.var_vbsc_dvbse_dn2, locals.var_vbsc_dvbse_dn6, locals.var_vbsc_dvbse_dn7, locals.var_vbsc_dvbse_dn10, locals.var_vbsc_dvbse_dn11, locals.var_vbsc_dvbse_dn12, locals.var_vbsc_dvbse_dn17,)
    }
};
        locals.var_vbsc_dvbse = assign5090_e3393;
        locals.var_vbsc_dvbse_dn0 = assign5090_e3393_d_n0;
        locals.var_vbsc_dvbse_dn2 = assign5090_e3393_d_n2;
        locals.var_vbsc_dvbse_dn6 = assign5090_e3393_d_n6;
        locals.var_vbsc_dvbse_dn7 = assign5090_e3393_d_n7;
        locals.var_vbsc_dvbse_dn10 = assign5090_e3393_d_n10;
        locals.var_vbsc_dvbse_dn11 = assign5090_e3393_d_n11;
        locals.var_vbsc_dvbse_dn12 = assign5090_e3393_d_n12;
        locals.var_vbsc_dvbse_dn17 = assign5090_e3393_d_n17;

        let (assign5100_e3398, assign5100_e3398_d_n0, assign5100_e3398_d_n2, assign5100_e3398_d_n6, assign5100_e3398_d_n7, assign5100_e3398_d_n10, assign5100_e3398_d_n11, assign5100_e3398_d_n12, assign5100_e3398_d_n17,) = {
    if (locals.var_guard50 == 0.0) {
        (locals.var_vbs, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
    } else {
        (locals.var_vbsc, locals.var_vbsc_dn0, locals.var_vbsc_dn2, locals.var_vbsc_dn6, locals.var_vbsc_dn7, locals.var_vbsc_dn10, locals.var_vbsc_dn11, locals.var_vbsc_dn12, locals.var_vbsc_dn17,)
    }
};
        locals.var_vbsc = assign5100_e3398;
        locals.var_vbsc_dn0 = assign5100_e3398_d_n0;
        locals.var_vbsc_dn2 = assign5100_e3398_d_n2;
        locals.var_vbsc_dn6 = assign5100_e3398_d_n6;
        locals.var_vbsc_dn7 = assign5100_e3398_d_n7;
        locals.var_vbsc_dn10 = assign5100_e3398_d_n10;
        locals.var_vbsc_dn11 = assign5100_e3398_d_n11;
        locals.var_vbsc_dn12 = assign5100_e3398_d_n12;
        locals.var_vbsc_dn17 = assign5100_e3398_d_n17;

        let (assign5110_e3403, assign5110_e3403_d_n0, assign5110_e3403_d_n2, assign5110_e3403_d_n6, assign5110_e3403_d_n7, assign5110_e3403_d_n10, assign5110_e3403_d_n11, assign5110_e3403_d_n12, assign5110_e3403_d_n17,) = {
    if (locals.var_guard50 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbsc_dvbse, locals.var_vbsc_dvbse_dn0, locals.var_vbsc_dvbse_dn2, locals.var_vbsc_dvbse_dn6, locals.var_vbsc_dvbse_dn7, locals.var_vbsc_dvbse_dn10, locals.var_vbsc_dvbse_dn11, locals.var_vbsc_dvbse_dn12, locals.var_vbsc_dvbse_dn17,)
    }
};
        locals.var_vbsc_dvbse = assign5110_e3403;
        locals.var_vbsc_dvbse_dn0 = assign5110_e3403_d_n0;
        locals.var_vbsc_dvbse_dn2 = assign5110_e3403_d_n2;
        locals.var_vbsc_dvbse_dn6 = assign5110_e3403_d_n6;
        locals.var_vbsc_dvbse_dn7 = assign5110_e3403_d_n7;
        locals.var_vbsc_dvbse_dn10 = assign5110_e3403_d_n10;
        locals.var_vbsc_dvbse_dn11 = assign5110_e3403_d_n11;
        locals.var_vbsc_dvbse_dn12 = assign5110_e3403_d_n12;
        locals.var_vbsc_dvbse_dn17 = assign5110_e3403_d_n17;

        let (assign5120_e3409, assign5120_e3409_d_n0, assign5120_e3409_d_n2, assign5120_e3409_d_n6, assign5120_e3409_d_n7, assign5120_e3409_d_n10, assign5120_e3409_d_n11, assign5120_e3409_d_n12, assign5120_e3409_d_n17,) = {
    if (locals.var_vds > 20.0) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vdsc = assign5120_e3409;
        locals.var_vdsc_dn0 = assign5120_e3409_d_n0;
        locals.var_vdsc_dn2 = assign5120_e3409_d_n2;
        locals.var_vdsc_dn6 = assign5120_e3409_d_n6;
        locals.var_vdsc_dn7 = assign5120_e3409_d_n7;
        locals.var_vdsc_dn10 = assign5120_e3409_d_n10;
        locals.var_vdsc_dn11 = assign5120_e3409_d_n11;
        locals.var_vdsc_dn12 = assign5120_e3409_d_n12;
        locals.var_vdsc_dn17 = assign5120_e3409_d_n17;

        let (assign5130_e3415, assign5130_e3415_d_n6, assign5130_e3415_d_n7, assign5130_e3415_d_n11,) = {
    if (locals.var_vgs > 20.0) {
        (20.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn11,)
    }
};
        locals.var_vgsc = assign5130_e3415;
        locals.var_vgsc_dn6 = assign5130_e3415_d_n6;
        locals.var_vgsc_dn7 = assign5130_e3415_d_n7;
        locals.var_vgsc_dn11 = assign5130_e3415_d_n11;

        let assign5140_e3418: f64 = (-20.0);
        let (assign5140_e3423, assign5140_e3423_d_n6, assign5140_e3423_d_n7, assign5140_e3423_d_n11,) = {
    if (locals.var_vgs < assign5140_e3418) {
        let assign5140_e3421: f64 = (-20.0);
        (assign5140_e3421, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgsc, locals.var_vgsc_dn6, locals.var_vgsc_dn7, locals.var_vgsc_dn11,)
    }
};
        locals.var_vgsc = assign5140_e3423;
        locals.var_vgsc_dn6 = assign5140_e3423_d_n6;
        locals.var_vgsc_dn7 = assign5140_e3423_d_n7;
        locals.var_vgsc_dn11 = assign5140_e3423_d_n11;

        let assign5150_e3426: f64 = (-20.0);
        let (assign5150_e3431, assign5150_e3431_d_n0, assign5150_e3431_d_n2, assign5150_e3431_d_n6, assign5150_e3431_d_n7, assign5150_e3431_d_n10, assign5150_e3431_d_n11, assign5150_e3431_d_n12, assign5150_e3431_d_n17,) = {
    if (locals.var_vbsc < assign5150_e3426) {
        let assign5150_e3429: f64 = (-20.0);
        (assign5150_e3429, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbsc, locals.var_vbsc_dn0, locals.var_vbsc_dn2, locals.var_vbsc_dn6, locals.var_vbsc_dn7, locals.var_vbsc_dn10, locals.var_vbsc_dn11, locals.var_vbsc_dn12, locals.var_vbsc_dn17,)
    }
};
        locals.var_vbsc = assign5150_e3431;
        locals.var_vbsc_dn0 = assign5150_e3431_d_n0;
        locals.var_vbsc_dn2 = assign5150_e3431_d_n2;
        locals.var_vbsc_dn6 = assign5150_e3431_d_n6;
        locals.var_vbsc_dn7 = assign5150_e3431_d_n7;
        locals.var_vbsc_dn10 = assign5150_e3431_d_n10;
        locals.var_vbsc_dn11 = assign5150_e3431_d_n11;
        locals.var_vbsc_dn12 = assign5150_e3431_d_n12;
        locals.var_vbsc_dn17 = assign5150_e3431_d_n17;

        locals.var_vds = locals.var_vdsc;
        locals.var_vds_dn0 = locals.var_vdsc_dn0;
        locals.var_vds_dn2 = locals.var_vdsc_dn2;
        locals.var_vds_dn6 = locals.var_vdsc_dn6;
        locals.var_vds_dn7 = locals.var_vdsc_dn7;
        locals.var_vds_dn10 = locals.var_vdsc_dn10;
        locals.var_vds_dn11 = locals.var_vdsc_dn11;
        locals.var_vds_dn12 = locals.var_vdsc_dn12;
        locals.var_vds_dn17 = locals.var_vdsc_dn17;

        locals.var_vgs = locals.var_vgsc;
        locals.var_vgs_dn6 = locals.var_vgsc_dn6;
        locals.var_vgs_dn7 = locals.var_vgsc_dn7;
        locals.var_vgs_dn11 = locals.var_vgsc_dn11;

        locals.var_vbs = locals.var_vbsc;
        locals.var_vbs_dn0 = locals.var_vbsc_dn0;
        locals.var_vbs_dn2 = locals.var_vbsc_dn2;
        locals.var_vbs_dn6 = locals.var_vbsc_dn6;
        locals.var_vbs_dn7 = locals.var_vbsc_dn7;
        locals.var_vbs_dn10 = locals.var_vbsc_dn10;
        locals.var_vbs_dn11 = locals.var_vbsc_dn11;
        locals.var_vbs_dn12 = locals.var_vbsc_dn12;
        locals.var_vbs_dn17 = locals.var_vbsc_dn17;

        locals.var_flg_pprv = 0.0;

        locals.var_pss0_ini = 0.0;

        locals.var_pbs0_ini = 0.0;

        locals.var_psb0_ini = 0.0;

        locals.var_pssl_ini = 0.0;

        locals.var_pbsl_ini = 0.0;

        locals.var_psbl_ini = 0.0;

        locals.var_ai = 0.0;
        locals.var_ai_dn0 = 0.0;
        locals.var_ai_dn2 = 0.0;
        locals.var_ai_dn6 = 0.0;
        locals.var_ai_dn7 = 0.0;
        locals.var_ai_dn10 = 0.0;
        locals.var_ai_dn11 = 0.0;
        locals.var_ai_dn12 = 0.0;
        locals.var_ai_dn17 = 0.0;

        locals.var_db = 0.0;
        locals.var_db_dn0 = 0.0;
        locals.var_db_dn2 = 0.0;
        locals.var_db_dn6 = 0.0;
        locals.var_db_dn7 = 0.0;
        locals.var_db_dn10 = 0.0;
        locals.var_db_dn11 = 0.0;
        locals.var_db_dn12 = 0.0;
        locals.var_db_dn17 = 0.0;

        locals.var_di = 0.0;
        locals.var_di_dn0 = 0.0;
        locals.var_di_dn2 = 0.0;
        locals.var_di_dn6 = 0.0;
        locals.var_di_dn7 = 0.0;
        locals.var_di_dn10 = 0.0;
        locals.var_di_dn11 = 0.0;
        locals.var_di_dn12 = 0.0;
        locals.var_di_dn17 = 0.0;

        locals.var_c2 = 0.0;
        locals.var_c2_dn0 = 0.0;
        locals.var_c2_dn2 = 0.0;
        locals.var_c2_dn6 = 0.0;
        locals.var_c2_dn7 = 0.0;
        locals.var_c2_dn10 = 0.0;
        locals.var_c2_dn11 = 0.0;
        locals.var_c2_dn12 = 0.0;
        locals.var_c2_dn17 = 0.0;

        locals.var_lp_s0 = 0.0;

        locals.var_lp_sl = 0.0;

        let assign5320_e3450: f64 = (locals.var_vbsc_dvbse * locals.var_vds);
        let assign5320_e3452: f64 = (assign5320_e3450 / 2.0);
        locals.var_t1__blk56 = assign5320_e3452;
        locals.var_t1__blk56_dn0 = (((locals.var_vbsc_dvbse_dn0 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn0)) / 2.0);
        locals.var_t1__blk56_dn2 = (((locals.var_vbsc_dvbse_dn2 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn2)) / 2.0);
        locals.var_t1__blk56_dn6 = (((locals.var_vbsc_dvbse_dn6 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn6)) / 2.0);
        locals.var_t1__blk56_dn7 = (((locals.var_vbsc_dvbse_dn7 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn7)) / 2.0);
        locals.var_t1__blk56_dn10 = (((locals.var_vbsc_dvbse_dn10 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn10)) / 2.0);
        locals.var_t1__blk56_dn11 = (((locals.var_vbsc_dvbse_dn11 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn11)) / 2.0);
        locals.var_t1__blk56_dn12 = (((locals.var_vbsc_dvbse_dn12 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn12)) / 2.0);
        locals.var_t1__blk56_dn17 = (((locals.var_vbsc_dvbse_dn17 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn17)) / 2.0);

        let assign5330_e3455: f64 = (2.0 * locals.var_t1__blk56);
        let assign5330_e3457: f64 = (assign5330_e3455 / p.p226);
        locals.var_tmf1 = assign5330_e3457;
        locals.var_tmf1_dn0 = ((2.0 * locals.var_t1__blk56_dn0) / p.p226);
        locals.var_tmf1_dn2 = ((2.0 * locals.var_t1__blk56_dn2) / p.p226);
        locals.var_tmf1_dn6 = ((2.0 * locals.var_t1__blk56_dn6) / p.p226);
        locals.var_tmf1_dn7 = ((2.0 * locals.var_t1__blk56_dn7) / p.p226);
        locals.var_tmf1_dn10 = ((2.0 * locals.var_t1__blk56_dn10) / p.p226);
        locals.var_tmf1_dn11 = ((2.0 * locals.var_t1__blk56_dn11) / p.p226);
        locals.var_tmf1_dn12 = ((2.0 * locals.var_t1__blk56_dn12) / p.p226);
        locals.var_tmf1_dn17 = ((2.0 * locals.var_t1__blk56_dn17) / p.p226);

        let assign5340_e3462: f64 = (1.0 / 2.0);
        let assign5340_e3466: f64 = (1.0 / 6.0);
        let assign5340_e3470: f64 = (1.0 / 24.0);
        let assign5340_e3474: f64 = (1.0 / 120.0);
        let assign5340_e3478: f64 = (1.0 / 720.0);
        let assign5340_e3482: f64 = (1.0 / 5040.0);
        let assign5340_e3483: f64 = (locals.var_tmf1 * assign5340_e3482);
        let assign5340_e3484: f64 = (assign5340_e3478 + assign5340_e3483);
        let assign5340_e3485: f64 = (locals.var_tmf1 * assign5340_e3484);
        let assign5340_e3486: f64 = (assign5340_e3474 + assign5340_e3485);
        let assign5340_e3487: f64 = (locals.var_tmf1 * assign5340_e3486);
        let assign5340_e3488: f64 = (assign5340_e3470 + assign5340_e3487);
        let assign5340_e3489: f64 = (locals.var_tmf1 * assign5340_e3488);
        let assign5340_e3490: f64 = (assign5340_e3466 + assign5340_e3489);
        let assign5340_e3491: f64 = (locals.var_tmf1 * assign5340_e3490);
        let assign5340_e3492: f64 = (assign5340_e3462 + assign5340_e3491);
        let assign5340_e3493: f64 = (locals.var_tmf1 * assign5340_e3492);
        let assign5340_e3494: f64 = (1.0 + assign5340_e3493);
        locals.var_tmf2 = assign5340_e3494;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * assign5340_e3492) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign5340_e3490) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign5340_e3488) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign5340_e3486) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign5340_e3484) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign5340_e3482)))))))))));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * assign5340_e3492) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign5340_e3490) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign5340_e3488) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign5340_e3486) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign5340_e3484) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign5340_e3482)))))))))));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * assign5340_e3492) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign5340_e3490) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign5340_e3488) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign5340_e3486) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign5340_e3484) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign5340_e3482)))))))))));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * assign5340_e3492) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign5340_e3490) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign5340_e3488) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign5340_e3486) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign5340_e3484) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign5340_e3482)))))))))));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * assign5340_e3492) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign5340_e3490) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign5340_e3488) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign5340_e3486) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign5340_e3484) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign5340_e3482)))))))))));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * assign5340_e3492) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign5340_e3490) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign5340_e3488) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign5340_e3486) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign5340_e3484) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign5340_e3482)))))))))));
        locals.var_tmf2_dn12 = ((locals.var_tmf1_dn12 * assign5340_e3492) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign5340_e3490) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign5340_e3488) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign5340_e3486) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign5340_e3484) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign5340_e3482)))))))))));
        locals.var_tmf2_dn17 = ((locals.var_tmf1_dn17 * assign5340_e3492) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign5340_e3490) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign5340_e3488) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign5340_e3486) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign5340_e3484) + (locals.var_tmf1 * (locals.var_tmf1_dn17 * assign5340_e3482)))))))))));

        let assign5350_e3497: f64 = (p.p226 / locals.var_tmf2);
        locals.var_vzadd = assign5350_e3497;
        locals.var_vzadd_dn0 = (-((p.p226 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn2 = (-((p.p226 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn6 = (-((p.p226 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn7 = (-((p.p226 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn10 = (-((p.p226 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn11 = (-((p.p226 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn12 = (-((p.p226 * locals.var_tmf2_dn12) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn17 = (-((p.p226 * locals.var_tmf2_dn17) / (locals.var_tmf2 * locals.var_tmf2)));

        let assign5360_e3500: f64 = if locals.var_vzadd < 5e-12 { 1.0 } else { 0.0 };
        locals.var_guard57 = assign5360_e3500;

        let (assign5370_e3504, assign5370_e3504_d_n0, assign5370_e3504_d_n2, assign5370_e3504_d_n6, assign5370_e3504_d_n7, assign5370_e3504_d_n10, assign5370_e3504_d_n11, assign5370_e3504_d_n12, assign5370_e3504_d_n17,) = {
    if (locals.var_guard57 != 0.0) {
        (5e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn12, locals.var_vzadd_dn17,)
    }
};
        locals.var_vzadd = assign5370_e3504;
        locals.var_vzadd_dn0 = assign5370_e3504_d_n0;
        locals.var_vzadd_dn2 = assign5370_e3504_d_n2;
        locals.var_vzadd_dn6 = assign5370_e3504_d_n6;
        locals.var_vzadd_dn7 = assign5370_e3504_d_n7;
        locals.var_vzadd_dn10 = assign5370_e3504_d_n10;
        locals.var_vzadd_dn11 = assign5370_e3504_d_n11;
        locals.var_vzadd_dn12 = assign5370_e3504_d_n12;
        locals.var_vzadd_dn17 = assign5370_e3504_d_n17;

        let assign5380_e3507: f64 = (locals.var_vbs + locals.var_vzadd);
        locals.var_vbsz = assign5380_e3507;
        locals.var_vbsz_dn0 = (locals.var_vbs_dn0 + locals.var_vzadd_dn0);
        locals.var_vbsz_dn2 = (locals.var_vbs_dn2 + locals.var_vzadd_dn2);
        locals.var_vbsz_dn6 = (locals.var_vbs_dn6 + locals.var_vzadd_dn6);
        locals.var_vbsz_dn7 = (locals.var_vbs_dn7 + locals.var_vzadd_dn7);
        locals.var_vbsz_dn10 = (locals.var_vbs_dn10 + locals.var_vzadd_dn10);
        locals.var_vbsz_dn11 = (locals.var_vbs_dn11 + locals.var_vzadd_dn11);
        locals.var_vbsz_dn12 = (locals.var_vbs_dn12 + locals.var_vzadd_dn12);
        locals.var_vbsz_dn17 = (locals.var_vbs_dn17 + locals.var_vzadd_dn17);

        let assign5390_e3511: f64 = (2.0 * locals.var_vzadd);
        let assign5390_e3512: f64 = (locals.var_vds + assign5390_e3511);
        locals.var_vdsz = assign5390_e3512;
        locals.var_vdsz_dn0 = (locals.var_vds_dn0 + (2.0 * locals.var_vzadd_dn0));
        locals.var_vdsz_dn2 = (locals.var_vds_dn2 + (2.0 * locals.var_vzadd_dn2));
        locals.var_vdsz_dn6 = (locals.var_vds_dn6 + (2.0 * locals.var_vzadd_dn6));
        locals.var_vdsz_dn7 = (locals.var_vds_dn7 + (2.0 * locals.var_vzadd_dn7));
        locals.var_vdsz_dn10 = (locals.var_vds_dn10 + (2.0 * locals.var_vzadd_dn10));
        locals.var_vdsz_dn11 = (locals.var_vds_dn11 + (2.0 * locals.var_vzadd_dn11));
        locals.var_vdsz_dn12 = (locals.var_vds_dn12 + (2.0 * locals.var_vzadd_dn12));
        locals.var_vdsz_dn17 = (locals.var_vds_dn17 + (2.0 * locals.var_vzadd_dn17));

        let assign5400_e3515: f64 = (locals.var_vgs + locals.var_vzadd);
        locals.var_vgsz = assign5400_e3515;
        locals.var_vgsz_dn0 = locals.var_vzadd_dn0;
        locals.var_vgsz_dn2 = locals.var_vzadd_dn2;
        locals.var_vgsz_dn6 = (locals.var_vgs_dn6 + locals.var_vzadd_dn6);
        locals.var_vgsz_dn7 = (locals.var_vgs_dn7 + locals.var_vzadd_dn7);
        locals.var_vgsz_dn10 = locals.var_vzadd_dn10;
        locals.var_vgsz_dn11 = (locals.var_vgs_dn11 + locals.var_vzadd_dn11);
        locals.var_vgsz_dn12 = locals.var_vzadd_dn12;
        locals.var_vgsz_dn17 = locals.var_vzadd_dn17;

        let assign5410_e3518: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard58 = assign5410_e3518;

        let (assign5420_e3522, assign5420_e3522_d_n0, assign5420_e3522_d_n2, assign5420_e3522_d_n6, assign5420_e3522_d_n7, assign5420_e3522_d_n10, assign5420_e3522_d_n11, assign5420_e3522_d_n12, assign5420_e3522_d_n17,) = {
    if (locals.var_guard58 != 0.0) {
        (locals.var_vbs, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
    } else {
        (locals.var_vbsp, locals.var_vbsp_dn0, locals.var_vbsp_dn2, locals.var_vbsp_dn6, locals.var_vbsp_dn7, locals.var_vbsp_dn10, locals.var_vbsp_dn11, locals.var_vbsp_dn12, locals.var_vbsp_dn17,)
    }
};
        locals.var_vbsp = assign5420_e3522;
        locals.var_vbsp_dn0 = assign5420_e3522_d_n0;
        locals.var_vbsp_dn2 = assign5420_e3522_d_n2;
        locals.var_vbsp_dn6 = assign5420_e3522_d_n6;
        locals.var_vbsp_dn7 = assign5420_e3522_d_n7;
        locals.var_vbsp_dn10 = assign5420_e3522_d_n10;
        locals.var_vbsp_dn11 = assign5420_e3522_d_n11;
        locals.var_vbsp_dn12 = assign5420_e3522_d_n12;
        locals.var_vbsp_dn17 = assign5420_e3522_d_n17;

        let (assign5430_e3526, assign5430_e3526_d_n0, assign5430_e3526_d_n2, assign5430_e3526_d_n6, assign5430_e3526_d_n7, assign5430_e3526_d_n10, assign5430_e3526_d_n11, assign5430_e3526_d_n12, assign5430_e3526_d_n17,) = {
    if (locals.var_guard58 != 0.0) {
        (locals.var_vbsz, locals.var_vbsz_dn0, locals.var_vbsz_dn2, locals.var_vbsz_dn6, locals.var_vbsz_dn7, locals.var_vbsz_dn10, locals.var_vbsz_dn11, locals.var_vbsz_dn12, locals.var_vbsz_dn17,)
    } else {
        (locals.var_vbspz, locals.var_vbspz_dn0, locals.var_vbspz_dn2, locals.var_vbspz_dn6, locals.var_vbspz_dn7, locals.var_vbspz_dn10, locals.var_vbspz_dn11, locals.var_vbspz_dn12, locals.var_vbspz_dn17,)
    }
};
        locals.var_vbspz = assign5430_e3526;
        locals.var_vbspz_dn0 = assign5430_e3526_d_n0;
        locals.var_vbspz_dn2 = assign5430_e3526_d_n2;
        locals.var_vbspz_dn6 = assign5430_e3526_d_n6;
        locals.var_vbspz_dn7 = assign5430_e3526_d_n7;
        locals.var_vbspz_dn10 = assign5430_e3526_d_n10;
        locals.var_vbspz_dn11 = assign5430_e3526_d_n11;
        locals.var_vbspz_dn12 = assign5430_e3526_d_n12;
        locals.var_vbspz_dn17 = assign5430_e3526_d_n17;

        let (assign5440_e3536, assign5440_e3536_d_n0, assign5440_e3536_d_n2, assign5440_e3536_d_n6, assign5440_e3536_d_n7, assign5440_e3536_d_n10, assign5440_e3536_d_n11, assign5440_e3536_d_n12, assign5440_e3536_d_n17,) = {
    if (locals.var_guard58 == 0.0) {
        let (assign5440_e3534, assign5440_e3534_d_n0, assign5440_e3534_d_n2, assign5440_e3534_d_n6, assign5440_e3534_d_n7, assign5440_e3534_d_n10, assign5440_e3534_d_n11, assign5440_e3534_d_n12, assign5440_e3534_d_n17,) = {
            if (locals.var_subversion < 3.0) {
                (locals.var_vbs, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign5440_e3534, assign5440_e3534_d_n0, assign5440_e3534_d_n2, assign5440_e3534_d_n6, assign5440_e3534_d_n7, assign5440_e3534_d_n10, assign5440_e3534_d_n11, assign5440_e3534_d_n12, assign5440_e3534_d_n17,)
    } else {
        (locals.var_vbsp, locals.var_vbsp_dn0, locals.var_vbsp_dn2, locals.var_vbsp_dn6, locals.var_vbsp_dn7, locals.var_vbsp_dn10, locals.var_vbsp_dn11, locals.var_vbsp_dn12, locals.var_vbsp_dn17,)
    }
};
        locals.var_vbsp = assign5440_e3536;
        locals.var_vbsp_dn0 = assign5440_e3536_d_n0;
        locals.var_vbsp_dn2 = assign5440_e3536_d_n2;
        locals.var_vbsp_dn6 = assign5440_e3536_d_n6;
        locals.var_vbsp_dn7 = assign5440_e3536_d_n7;
        locals.var_vbsp_dn10 = assign5440_e3536_d_n10;
        locals.var_vbsp_dn11 = assign5440_e3536_d_n11;
        locals.var_vbsp_dn12 = assign5440_e3536_d_n12;
        locals.var_vbsp_dn17 = assign5440_e3536_d_n17;

        let (assign5450_e3546, assign5450_e3546_d_n0, assign5450_e3546_d_n2, assign5450_e3546_d_n6, assign5450_e3546_d_n7, assign5450_e3546_d_n10, assign5450_e3546_d_n11, assign5450_e3546_d_n12, assign5450_e3546_d_n17,) = {
    if (locals.var_guard58 == 0.0) {
        let (assign5450_e3544, assign5450_e3544_d_n0, assign5450_e3544_d_n2, assign5450_e3544_d_n6, assign5450_e3544_d_n7, assign5450_e3544_d_n10, assign5450_e3544_d_n11, assign5450_e3544_d_n12, assign5450_e3544_d_n17,) = {
            if (locals.var_subversion < 3.0) {
                (locals.var_vbsz, locals.var_vbsz_dn0, locals.var_vbsz_dn2, locals.var_vbsz_dn6, locals.var_vbsz_dn7, locals.var_vbsz_dn10, locals.var_vbsz_dn11, locals.var_vbsz_dn12, locals.var_vbsz_dn17,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign5450_e3544, assign5450_e3544_d_n0, assign5450_e3544_d_n2, assign5450_e3544_d_n6, assign5450_e3544_d_n7, assign5450_e3544_d_n10, assign5450_e3544_d_n11, assign5450_e3544_d_n12, assign5450_e3544_d_n17,)
    } else {
        (locals.var_vbspz, locals.var_vbspz_dn0, locals.var_vbspz_dn2, locals.var_vbspz_dn6, locals.var_vbspz_dn7, locals.var_vbspz_dn10, locals.var_vbspz_dn11, locals.var_vbspz_dn12, locals.var_vbspz_dn17,)
    }
};
        locals.var_vbspz = assign5450_e3546;
        locals.var_vbspz_dn0 = assign5450_e3546_d_n0;
        locals.var_vbspz_dn2 = assign5450_e3546_d_n2;
        locals.var_vbspz_dn6 = assign5450_e3546_d_n6;
        locals.var_vbspz_dn7 = assign5450_e3546_d_n7;
        locals.var_vbspz_dn10 = assign5450_e3546_d_n10;
        locals.var_vbspz_dn11 = assign5450_e3546_d_n11;
        locals.var_vbspz_dn12 = assign5450_e3546_d_n12;
        locals.var_vbspz_dn17 = assign5450_e3546_d_n17;

        let assign5460_e3549: f64 = (2.0 * locals.var_q_nsub);
        let assign5460_e3551: f64 = (assign5460_e3549 * 1.034943e-10);
        let assign5460_e3553: f64 = (assign5460_e3551 * locals.var_c_fox0_inv);
        let assign5460_e3555: f64 = (assign5460_e3553 * locals.var_c_fox0_inv);
        locals.var_t1__blk59 = assign5460_e3555;
        locals.var_t1__blk59_dn0 = ((((2.0 * locals.var_q_nsub_dn0) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1__blk59_dn2 = ((((2.0 * locals.var_q_nsub_dn2) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1__blk59_dn6 = ((((2.0 * locals.var_q_nsub_dn6) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1__blk59_dn7 = ((((2.0 * locals.var_q_nsub_dn7) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1__blk59_dn10 = ((((2.0 * locals.var_q_nsub_dn10) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1__blk59_dn11 = ((((2.0 * locals.var_q_nsub_dn11) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1__blk59_dn12 = ((((2.0 * locals.var_q_nsub_dn12) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1__blk59_dn17 = ((((2.0 * locals.var_q_nsub_dn17) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);

        let assign5470_e3558: f64 = (locals.var_vgs - locals.var_vfb);
        locals.var_t2__blk60 = assign5470_e3558;
        locals.var_t2__blk60_dn6 = locals.var_vgs_dn6;
        locals.var_t2__blk60_dn7 = locals.var_vgs_dn7;
        locals.var_t2__blk60_dn11 = locals.var_vgs_dn11;

        let assign5480_e3562: f64 = (2.0 / locals.var_t1__blk59);
        let assign5480_e3565: f64 = (locals.var_t2__blk60 - locals.var_beta_inv);
        let assign5480_e3567: f64 = (assign5480_e3565 - locals.var_vbsp);
        let assign5480_e3568: f64 = (assign5480_e3562 * assign5480_e3567);
        let assign5480_e3569: f64 = (1.0 + assign5480_e3568);
        locals.var_t3__blk61 = assign5480_e3569;
        locals.var_t3__blk61_dn0 = (((-((2.0 * locals.var_t1__blk59_dn0) / (locals.var_t1__blk59 * locals.var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * (-locals.var_vbsp_dn0)));
        locals.var_t3__blk61_dn2 = (((-((2.0 * locals.var_t1__blk59_dn2) / (locals.var_t1__blk59 * locals.var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * (-locals.var_vbsp_dn2)));
        locals.var_t3__blk61_dn6 = (((-((2.0 * locals.var_t1__blk59_dn6) / (locals.var_t1__blk59 * locals.var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * (locals.var_t2__blk60_dn6 - locals.var_vbsp_dn6)));
        locals.var_t3__blk61_dn7 = (((-((2.0 * locals.var_t1__blk59_dn7) / (locals.var_t1__blk59 * locals.var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * (locals.var_t2__blk60_dn7 - locals.var_vbsp_dn7)));
        locals.var_t3__blk61_dn10 = (((-((2.0 * locals.var_t1__blk59_dn10) / (locals.var_t1__blk59 * locals.var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * ((-locals.var_beta_inv_dn10) - locals.var_vbsp_dn10)));
        locals.var_t3__blk61_dn11 = (((-((2.0 * locals.var_t1__blk59_dn11) / (locals.var_t1__blk59 * locals.var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * (locals.var_t2__blk60_dn11 - locals.var_vbsp_dn11)));
        locals.var_t3__blk61_dn12 = (((-((2.0 * locals.var_t1__blk59_dn12) / (locals.var_t1__blk59 * locals.var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * (-locals.var_vbsp_dn12)));
        locals.var_t3__blk61_dn17 = (((-((2.0 * locals.var_t1__blk59_dn17) / (locals.var_t1__blk59 * locals.var_t1__blk59))) * assign5480_e3567) + (assign5480_e3562 * (-locals.var_vbsp_dn17)));

        let assign5490_e3572: f64 = (locals.var_t3__blk61 * locals.var_t3__blk61);
        let assign5490_e3575: f64 = (4.0 * 0.001);
        let assign5490_e3577: f64 = (assign5490_e3575 * 0.001);
        let assign5490_e3578: f64 = (assign5490_e3572 + assign5490_e3577);
        let assign5490_e3579: f64 = (assign5490_e3578).sqrt();
        locals.var_tmf1 = assign5490_e3579;
        locals.var_tmf1_dn0 = (((locals.var_t3__blk61_dn0 * locals.var_t3__blk61) + (locals.var_t3__blk61 * locals.var_t3__blk61_dn0)) / (2.0 * assign5490_e3579));
        locals.var_tmf1_dn2 = (((locals.var_t3__blk61_dn2 * locals.var_t3__blk61) + (locals.var_t3__blk61 * locals.var_t3__blk61_dn2)) / (2.0 * assign5490_e3579));
        locals.var_tmf1_dn6 = (((locals.var_t3__blk61_dn6 * locals.var_t3__blk61) + (locals.var_t3__blk61 * locals.var_t3__blk61_dn6)) / (2.0 * assign5490_e3579));
        locals.var_tmf1_dn7 = (((locals.var_t3__blk61_dn7 * locals.var_t3__blk61) + (locals.var_t3__blk61 * locals.var_t3__blk61_dn7)) / (2.0 * assign5490_e3579));
        locals.var_tmf1_dn10 = (((locals.var_t3__blk61_dn10 * locals.var_t3__blk61) + (locals.var_t3__blk61 * locals.var_t3__blk61_dn10)) / (2.0 * assign5490_e3579));
        locals.var_tmf1_dn11 = (((locals.var_t3__blk61_dn11 * locals.var_t3__blk61) + (locals.var_t3__blk61 * locals.var_t3__blk61_dn11)) / (2.0 * assign5490_e3579));
        locals.var_tmf1_dn12 = (((locals.var_t3__blk61_dn12 * locals.var_t3__blk61) + (locals.var_t3__blk61 * locals.var_t3__blk61_dn12)) / (2.0 * assign5490_e3579));
        locals.var_tmf1_dn17 = (((locals.var_t3__blk61_dn17 * locals.var_t3__blk61) + (locals.var_t3__blk61 * locals.var_t3__blk61_dn17)) / (2.0 * assign5490_e3579));

        let assign5500_e3583: f64 = (locals.var_t3__blk61 + locals.var_tmf1);
        let assign5500_e3584: f64 = (0.5 * assign5500_e3583);
        let assign5500_e3587: f64 = (1e-10 * 0.001);
        let assign5500_e3588: f64 = (assign5500_e3584 + assign5500_e3587);
        locals.var_t4 = assign5500_e3588;
        locals.var_t4_dn0 = (0.5 * (locals.var_t3__blk61_dn0 + locals.var_tmf1_dn0));
        locals.var_t4_dn2 = (0.5 * (locals.var_t3__blk61_dn2 + locals.var_tmf1_dn2));
        locals.var_t4_dn6 = (0.5 * (locals.var_t3__blk61_dn6 + locals.var_tmf1_dn6));
        locals.var_t4_dn7 = (0.5 * (locals.var_t3__blk61_dn7 + locals.var_tmf1_dn7));
        locals.var_t4_dn10 = (0.5 * (locals.var_t3__blk61_dn10 + locals.var_tmf1_dn10));
        locals.var_t4_dn11 = (0.5 * (locals.var_t3__blk61_dn11 + locals.var_tmf1_dn11));
        locals.var_t4_dn12 = (0.5 * (locals.var_t3__blk61_dn12 + locals.var_tmf1_dn12));
        locals.var_t4_dn17 = (0.5 * (locals.var_t3__blk61_dn17 + locals.var_tmf1_dn17));

        let assign5510_e3591: f64 = if locals.var_t4 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard63 = assign5510_e3591;

    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5520_e3595, assign5520_e3595_d_n0, assign5520_e3595_d_n2, assign5520_e3595_d_n6, assign5520_e3595_d_n7, assign5520_e3595_d_n10, assign5520_e3595_d_n11, assign5520_e3595_d_n12, assign5520_e3595_d_n17,) = {
    if (locals.var_guard63 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign5520_e3595;
        locals.var_t4_dn0 = assign5520_e3595_d_n0;
        locals.var_t4_dn2 = assign5520_e3595_d_n2;
        locals.var_t4_dn6 = assign5520_e3595_d_n6;
        locals.var_t4_dn7 = assign5520_e3595_d_n7;
        locals.var_t4_dn10 = assign5520_e3595_d_n10;
        locals.var_t4_dn11 = assign5520_e3595_d_n11;
        locals.var_t4_dn12 = assign5520_e3595_d_n12;
        locals.var_t4_dn17 = assign5520_e3595_d_n17;

        let assign5530_e3598: f64 = (locals.var_t4 + 1e-50);
        let assign5530_e3599: f64 = (assign5530_e3598).sqrt();
        locals.var_tx__blk62 = assign5530_e3599;
        locals.var_tx__blk62_dn0 = (locals.var_t4_dn0 / (2.0 * assign5530_e3599));
        locals.var_tx__blk62_dn2 = (locals.var_t4_dn2 / (2.0 * assign5530_e3599));
        locals.var_tx__blk62_dn6 = (locals.var_t4_dn6 / (2.0 * assign5530_e3599));
        locals.var_tx__blk62_dn7 = (locals.var_t4_dn7 / (2.0 * assign5530_e3599));
        locals.var_tx__blk62_dn10 = (locals.var_t4_dn10 / (2.0 * assign5530_e3599));
        locals.var_tx__blk62_dn11 = (locals.var_t4_dn11 / (2.0 * assign5530_e3599));
        locals.var_tx__blk62_dn12 = (locals.var_t4_dn12 / (2.0 * assign5530_e3599));
        locals.var_tx__blk62_dn17 = (locals.var_t4_dn17 / (2.0 * assign5530_e3599));

        let assign5540_e3604: f64 = (1.0 - locals.var_tx__blk62);
        let assign5540_e3605: f64 = (locals.var_t1__blk59 * assign5540_e3604);
        let assign5540_e3606: f64 = (locals.var_t2__blk60 + assign5540_e3605);
        locals.var_pslsat = assign5540_e3606;
        locals.var_pslsat_dn0 = ((locals.var_t1__blk59_dn0 * assign5540_e3604) + (locals.var_t1__blk59 * (-locals.var_tx__blk62_dn0)));
        locals.var_pslsat_dn2 = ((locals.var_t1__blk59_dn2 * assign5540_e3604) + (locals.var_t1__blk59 * (-locals.var_tx__blk62_dn2)));
        locals.var_pslsat_dn6 = (locals.var_t2__blk60_dn6 + ((locals.var_t1__blk59_dn6 * assign5540_e3604) + (locals.var_t1__blk59 * (-locals.var_tx__blk62_dn6))));
        locals.var_pslsat_dn7 = (locals.var_t2__blk60_dn7 + ((locals.var_t1__blk59_dn7 * assign5540_e3604) + (locals.var_t1__blk59 * (-locals.var_tx__blk62_dn7))));
        locals.var_pslsat_dn10 = ((locals.var_t1__blk59_dn10 * assign5540_e3604) + (locals.var_t1__blk59 * (-locals.var_tx__blk62_dn10)));
        locals.var_pslsat_dn11 = (locals.var_t2__blk60_dn11 + ((locals.var_t1__blk59_dn11 * assign5540_e3604) + (locals.var_t1__blk59 * (-locals.var_tx__blk62_dn11))));
        locals.var_pslsat_dn12 = ((locals.var_t1__blk59_dn12 * assign5540_e3604) + (locals.var_t1__blk59 * (-locals.var_tx__blk62_dn12)));
        locals.var_pslsat_dn17 = ((locals.var_t1__blk59_dn17 * assign5540_e3604) + (locals.var_t1__blk59 * (-locals.var_tx__blk62_dn17)));

        let assign5550_e3609: f64 = (locals.var_pslsat - locals.var_pb2);
        locals.var_vdsats = assign5550_e3609;
        locals.var_vdsats_dn0 = (locals.var_pslsat_dn0 - locals.var_pb2_dn0);
        locals.var_vdsats_dn2 = (locals.var_pslsat_dn2 - locals.var_pb2_dn2);
        locals.var_vdsats_dn6 = (locals.var_pslsat_dn6 - locals.var_pb2_dn6);
        locals.var_vdsats_dn7 = (locals.var_pslsat_dn7 - locals.var_pb2_dn7);
        locals.var_vdsats_dn10 = (locals.var_pslsat_dn10 - locals.var_pb2_dn10);
        locals.var_vdsats_dn11 = (locals.var_pslsat_dn11 - locals.var_pb2_dn11);
        locals.var_vdsats_dn12 = (locals.var_pslsat_dn12 - locals.var_pb2_dn12);
        locals.var_vdsats_dn17 = (locals.var_pslsat_dn17 - locals.var_pb2_dn17);

        let assign5560_e3612: f64 = (locals.var_vdsats - 0.1);
        let assign5560_e3614: f64 = (assign5560_e3612 - 0.05);
        locals.var_tmf1 = assign5560_e3614;
        locals.var_tmf1_dn0 = locals.var_vdsats_dn0;
        locals.var_tmf1_dn2 = locals.var_vdsats_dn2;
        locals.var_tmf1_dn6 = locals.var_vdsats_dn6;
        locals.var_tmf1_dn7 = locals.var_vdsats_dn7;
        locals.var_tmf1_dn10 = locals.var_vdsats_dn10;
        locals.var_tmf1_dn11 = locals.var_vdsats_dn11;
        locals.var_tmf1_dn12 = locals.var_vdsats_dn12;
        locals.var_tmf1_dn17 = locals.var_vdsats_dn17;

        let assign5570_e3617: f64 = (4.0 * 0.1);
        let assign5570_e3619: f64 = (assign5570_e3617 * 0.05);
        locals.var_tmf2 = assign5570_e3619;
        locals.var_tmf2_dn0 = 0.0;
        locals.var_tmf2_dn2 = 0.0;
        locals.var_tmf2_dn6 = 0.0;
        locals.var_tmf2_dn7 = 0.0;
        locals.var_tmf2_dn10 = 0.0;
        locals.var_tmf2_dn11 = 0.0;
        locals.var_tmf2_dn12 = 0.0;
        locals.var_tmf2_dn17 = 0.0;

        let (assign5580_e3626, assign5580_e3626_d_n0, assign5580_e3626_d_n2, assign5580_e3626_d_n6, assign5580_e3626_d_n7, assign5580_e3626_d_n10, assign5580_e3626_d_n11, assign5580_e3626_d_n12, assign5580_e3626_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign5580_e3625: f64 = (-locals.var_tmf2);
        (assign5580_e3625, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
        locals.var_tmf2 = assign5580_e3626;
        locals.var_tmf2_dn0 = assign5580_e3626_d_n0;
        locals.var_tmf2_dn2 = assign5580_e3626_d_n2;
        locals.var_tmf2_dn6 = assign5580_e3626_d_n6;
        locals.var_tmf2_dn7 = assign5580_e3626_d_n7;
        locals.var_tmf2_dn10 = assign5580_e3626_d_n10;
        locals.var_tmf2_dn11 = assign5580_e3626_d_n11;
        locals.var_tmf2_dn12 = assign5580_e3626_d_n12;
        locals.var_tmf2_dn17 = assign5580_e3626_d_n17;

        let assign5590_e3629: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5590_e3631: f64 = (assign5590_e3629 + locals.var_tmf2);
        let assign5590_e3632: f64 = (assign5590_e3631).sqrt();
        locals.var_tmf2 = assign5590_e3632;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5590_e3632));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5590_e3632));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign5590_e3632));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign5590_e3632));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign5590_e3632));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign5590_e3632));
        locals.var_tmf2_dn12 = ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign5590_e3632));
        locals.var_tmf2_dn17 = ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign5590_e3632));

        let assign5600_e3637: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5600_e3638: f64 = (0.5 * assign5600_e3637);
        let assign5600_e3639: f64 = (0.1 + assign5600_e3638);
        locals.var_vdsats = assign5600_e3639;
        locals.var_vdsats_dn0 = (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0));
        locals.var_vdsats_dn2 = (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2));
        locals.var_vdsats_dn6 = (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6));
        locals.var_vdsats_dn7 = (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7));
        locals.var_vdsats_dn10 = (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10));
        locals.var_vdsats_dn11 = (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11));
        locals.var_vdsats_dn12 = (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12));
        locals.var_vdsats_dn17 = (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17));

        let assign5610_e3642: f64 = (locals.var_vds / locals.var_vdsats);
        locals.var_t1__blk59 = assign5610_e3642;
        locals.var_t1__blk59_dn0 = (((locals.var_vds_dn0 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn0)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1__blk59_dn2 = (((locals.var_vds_dn2 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn2)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1__blk59_dn6 = (((locals.var_vds_dn6 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn6)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1__blk59_dn7 = (((locals.var_vds_dn7 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn7)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1__blk59_dn10 = (((locals.var_vds_dn10 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn10)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1__blk59_dn11 = (((locals.var_vds_dn11 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn11)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1__blk59_dn12 = (((locals.var_vds_dn12 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn12)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1__blk59_dn17 = (((locals.var_vds_dn17 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn17)) / (locals.var_vdsats * locals.var_vdsats));

        let assign5620_e3645: f64 = locals.var_t1__blk59;
        locals.var_tmf1 = assign5620_e3645;
        locals.var_tmf1_dn0 = locals.var_t1__blk59_dn0;
        locals.var_tmf1_dn2 = locals.var_t1__blk59_dn2;
        locals.var_tmf1_dn6 = locals.var_t1__blk59_dn6;
        locals.var_tmf1_dn7 = locals.var_t1__blk59_dn7;
        locals.var_tmf1_dn10 = locals.var_t1__blk59_dn10;
        locals.var_tmf1_dn11 = locals.var_t1__blk59_dn11;
        locals.var_tmf1_dn12 = locals.var_t1__blk59_dn12;
        locals.var_tmf1_dn17 = locals.var_t1__blk59_dn17;

        let assign5630_e3648: f64 = (locals.var_tmf1 * locals.var_tmf1);
        locals.var_tmf2 = assign5630_e3648;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11));
        locals.var_tmf2_dn12 = ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12));
        locals.var_tmf2_dn17 = ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17));

        let assign5640_e3651: f64 = (locals.var_tmf2 * locals.var_tmf1);
        locals.var_tmf3 = assign5640_e3651;
        locals.var_tmf3_dn0 = ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0));
        locals.var_tmf3_dn2 = ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2));
        locals.var_tmf3_dn6 = ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6));
        locals.var_tmf3_dn7 = ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7));
        locals.var_tmf3_dn10 = ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10));
        locals.var_tmf3_dn11 = ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11));
        locals.var_tmf3_dn12 = ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12));
        locals.var_tmf3_dn17 = ((locals.var_tmf2_dn17 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn17));

        let assign5650_e3654: f64 = (locals.var_tmf2 * locals.var_tmf2);
        locals.var_tmf4 = assign5650_e3654;
        locals.var_tmf4_dn0 = ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0));
        locals.var_tmf4_dn2 = ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2));
        locals.var_tmf4_dn6 = ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6));
        locals.var_tmf4_dn7 = ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7));
        locals.var_tmf4_dn10 = ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10));
        locals.var_tmf4_dn11 = ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11));
        locals.var_tmf4_dn12 = ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12));
        locals.var_tmf4_dn17 = ((locals.var_tmf2_dn17 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn17));

        let assign5660_e3658: f64 = (1.0 + locals.var_tmf1);
        let assign5660_e3660: f64 = (assign5660_e3658 + locals.var_tmf2);
        let assign5660_e3662: f64 = (assign5660_e3660 + locals.var_tmf3);
        let assign5660_e3664: f64 = (assign5660_e3662 + locals.var_tmf4);
        let assign5660_e3665: f64 = (1.0 / assign5660_e3664);
        locals.var_tx__blk62 = assign5660_e3665;
        locals.var_tx__blk62_dn0 = (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign5660_e3664 * assign5660_e3664)));
        locals.var_tx__blk62_dn2 = (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign5660_e3664 * assign5660_e3664)));
        locals.var_tx__blk62_dn6 = (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign5660_e3664 * assign5660_e3664)));
        locals.var_tx__blk62_dn7 = (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign5660_e3664 * assign5660_e3664)));
        locals.var_tx__blk62_dn10 = (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign5660_e3664 * assign5660_e3664)));
        locals.var_tx__blk62_dn11 = (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign5660_e3664 * assign5660_e3664)));
        locals.var_tx__blk62_dn12 = (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign5660_e3664 * assign5660_e3664)));
        locals.var_tx__blk62_dn17 = (-((((locals.var_tmf1_dn17 + locals.var_tmf2_dn17) + locals.var_tmf3_dn17) + locals.var_tmf4_dn17) / (assign5660_e3664 * assign5660_e3664)));

        let assign5670_e3669: f64 = (2.0 * locals.var_tmf1);
        let assign5670_e3670: f64 = (1.0 + assign5670_e3669);
        let assign5670_e3673: f64 = (3.0 * locals.var_tmf2);
        let assign5670_e3674: f64 = (assign5670_e3670 + assign5670_e3673);
        let assign5670_e3677: f64 = (4.0 * locals.var_tmf3);
        let assign5670_e3678: f64 = (assign5670_e3674 + assign5670_e3677);
        let assign5670_e3679: f64 = (-assign5670_e3678);
        let assign5670_e3681: f64 = (assign5670_e3679 * locals.var_tx__blk62);
        let assign5670_e3683: f64 = (assign5670_e3681 * locals.var_tx__blk62);
        locals.var_t0 = assign5670_e3683;
        locals.var_t0_dn0 = (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tx__blk62) + (assign5670_e3679 * locals.var_tx__blk62_dn0)) * locals.var_tx__blk62) + (assign5670_e3681 * locals.var_tx__blk62_dn0));
        locals.var_t0_dn2 = (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tx__blk62) + (assign5670_e3679 * locals.var_tx__blk62_dn2)) * locals.var_tx__blk62) + (assign5670_e3681 * locals.var_tx__blk62_dn2));
        locals.var_t0_dn6 = (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tx__blk62) + (assign5670_e3679 * locals.var_tx__blk62_dn6)) * locals.var_tx__blk62) + (assign5670_e3681 * locals.var_tx__blk62_dn6));
        locals.var_t0_dn7 = (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tx__blk62) + (assign5670_e3679 * locals.var_tx__blk62_dn7)) * locals.var_tx__blk62) + (assign5670_e3681 * locals.var_tx__blk62_dn7));
        locals.var_t0_dn10 = (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tx__blk62) + (assign5670_e3679 * locals.var_tx__blk62_dn10)) * locals.var_tx__blk62) + (assign5670_e3681 * locals.var_tx__blk62_dn10));
        locals.var_t0_dn11 = (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tx__blk62) + (assign5670_e3679 * locals.var_tx__blk62_dn11)) * locals.var_tx__blk62) + (assign5670_e3681 * locals.var_tx__blk62_dn11));
        locals.var_t0_dn12 = (((((-(((2.0 * locals.var_tmf1_dn12) + (3.0 * locals.var_tmf2_dn12)) + (4.0 * locals.var_tmf3_dn12))) * locals.var_tx__blk62) + (assign5670_e3679 * locals.var_tx__blk62_dn12)) * locals.var_tx__blk62) + (assign5670_e3681 * locals.var_tx__blk62_dn12));
        locals.var_t0_dn17 = (((((-(((2.0 * locals.var_tmf1_dn17) + (3.0 * locals.var_tmf2_dn17)) + (4.0 * locals.var_tmf3_dn17))) * locals.var_tx__blk62) + (assign5670_e3679 * locals.var_tx__blk62_dn17)) * locals.var_tx__blk62) + (assign5670_e3681 * locals.var_tx__blk62_dn17));

        let assign5680_e3687: f64 = (1.0 - locals.var_tx__blk62);
        let assign5680_e3688: f64 = assign5680_e3687;
        locals.var_tx__blk62 = assign5680_e3688;
        locals.var_tx__blk62_dn0 = (-locals.var_tx__blk62_dn0);
        locals.var_tx__blk62_dn2 = (-locals.var_tx__blk62_dn2);
        locals.var_tx__blk62_dn6 = (-locals.var_tx__blk62_dn6);
        locals.var_tx__blk62_dn7 = (-locals.var_tx__blk62_dn7);
        locals.var_tx__blk62_dn10 = (-locals.var_tx__blk62_dn10);
        locals.var_tx__blk62_dn11 = (-locals.var_tx__blk62_dn11);
        locals.var_tx__blk62_dn12 = (-locals.var_tx__blk62_dn12);
        locals.var_tx__blk62_dn17 = (-locals.var_tx__blk62_dn17);

        let assign5690_e3690: f64 = (-locals.var_t0);
        locals.var_t0 = assign5690_e3690;
        locals.var_t0_dn0 = (-locals.var_t0_dn0);
        locals.var_t0_dn2 = (-locals.var_t0_dn2);
        locals.var_t0_dn6 = (-locals.var_t0_dn6);
        locals.var_t0_dn7 = (-locals.var_t0_dn7);
        locals.var_t0_dn10 = (-locals.var_t0_dn10);
        locals.var_t0_dn11 = (-locals.var_t0_dn11);
        locals.var_t0_dn12 = (-locals.var_t0_dn12);
        locals.var_t0_dn17 = (-locals.var_t0_dn17);

        let assign5700_e3693: f64 = (locals.var_tx__blk62 * locals.var_tx__blk62);
        locals.var_fmdvds = assign5700_e3693;
        locals.var_fmdvds_dn0 = ((locals.var_tx__blk62_dn0 * locals.var_tx__blk62) + (locals.var_tx__blk62 * locals.var_tx__blk62_dn0));
        locals.var_fmdvds_dn2 = ((locals.var_tx__blk62_dn2 * locals.var_tx__blk62) + (locals.var_tx__blk62 * locals.var_tx__blk62_dn2));
        locals.var_fmdvds_dn6 = ((locals.var_tx__blk62_dn6 * locals.var_tx__blk62) + (locals.var_tx__blk62 * locals.var_tx__blk62_dn6));
        locals.var_fmdvds_dn7 = ((locals.var_tx__blk62_dn7 * locals.var_tx__blk62) + (locals.var_tx__blk62 * locals.var_tx__blk62_dn7));
        locals.var_fmdvds_dn10 = ((locals.var_tx__blk62_dn10 * locals.var_tx__blk62) + (locals.var_tx__blk62 * locals.var_tx__blk62_dn10));
        locals.var_fmdvds_dn11 = ((locals.var_tx__blk62_dn11 * locals.var_tx__blk62) + (locals.var_tx__blk62 * locals.var_tx__blk62_dn11));
        locals.var_fmdvds_dn12 = ((locals.var_tx__blk62_dn12 * locals.var_tx__blk62) + (locals.var_tx__blk62 * locals.var_tx__blk62_dn12));
        locals.var_fmdvds_dn17 = ((locals.var_tx__blk62_dn17 * locals.var_tx__blk62) + (locals.var_tx__blk62 * locals.var_tx__blk62_dn17));

        let assign5710_e3704: f64 = if (((p.p204 == 0.0) && (p.p206 == 0.0)) || (p.p205 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard70 = assign5710_e3704;

        let (assign5720_e3708,) = {
    if (locals.var_guard70 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign5720_e3708;

        let (assign5730_e3713,) = {
    if (locals.var_guard70 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign5730_e3713;

        let assign5740_e3716: f64 = (2.0 * locals.var_q_nsub);
        let assign5740_e3718: f64 = (assign5740_e3716 * 1.034943e-10);
        let assign5740_e3720: f64 = (assign5740_e3718 * locals.var_pb20);
        let assign5740_e3721: f64 = (assign5740_e3720).sqrt();
        locals.var_t2__blk64 = assign5740_e3721;
        locals.var_t2__blk64_dn0 = (((((2.0 * locals.var_q_nsub_dn0) * 1.034943e-10) * locals.var_pb20) + (assign5740_e3718 * locals.var_pb20_dn0)) / (2.0 * assign5740_e3721));
        locals.var_t2__blk64_dn2 = (((((2.0 * locals.var_q_nsub_dn2) * 1.034943e-10) * locals.var_pb20) + (assign5740_e3718 * locals.var_pb20_dn2)) / (2.0 * assign5740_e3721));
        locals.var_t2__blk64_dn6 = (((((2.0 * locals.var_q_nsub_dn6) * 1.034943e-10) * locals.var_pb20) + (assign5740_e3718 * locals.var_pb20_dn6)) / (2.0 * assign5740_e3721));
        locals.var_t2__blk64_dn7 = (((((2.0 * locals.var_q_nsub_dn7) * 1.034943e-10) * locals.var_pb20) + (assign5740_e3718 * locals.var_pb20_dn7)) / (2.0 * assign5740_e3721));
        locals.var_t2__blk64_dn10 = (((((2.0 * locals.var_q_nsub_dn10) * 1.034943e-10) * locals.var_pb20) + (assign5740_e3718 * locals.var_pb20_dn10)) / (2.0 * assign5740_e3721));
        locals.var_t2__blk64_dn11 = (((((2.0 * locals.var_q_nsub_dn11) * 1.034943e-10) * locals.var_pb20) + (assign5740_e3718 * locals.var_pb20_dn11)) / (2.0 * assign5740_e3721));
        locals.var_t2__blk64_dn12 = (((((2.0 * locals.var_q_nsub_dn12) * 1.034943e-10) * locals.var_pb20) + (assign5740_e3718 * locals.var_pb20_dn12)) / (2.0 * assign5740_e3721));
        locals.var_t2__blk64_dn17 = (((((2.0 * locals.var_q_nsub_dn17) * 1.034943e-10) * locals.var_pb20) + (assign5740_e3718 * locals.var_pb20_dn17)) / (2.0 * assign5740_e3721));

        let assign5750_e3724: f64 = (locals.var_pb20 + locals.var_vfb);
        let assign5750_e3727: f64 = (locals.var_t2__blk64 / locals.var_c_fox0);
        let assign5750_e3728: f64 = (assign5750_e3724 + assign5750_e3727);
        locals.var_vthq = assign5750_e3728;
        locals.var_vthq_dn0 = (locals.var_pb20_dn0 + (locals.var_t2__blk64_dn0 / locals.var_c_fox0));
        locals.var_vthq_dn2 = (locals.var_pb20_dn2 + (locals.var_t2__blk64_dn2 / locals.var_c_fox0));
        locals.var_vthq_dn6 = (locals.var_pb20_dn6 + (locals.var_t2__blk64_dn6 / locals.var_c_fox0));
        locals.var_vthq_dn7 = (locals.var_pb20_dn7 + (locals.var_t2__blk64_dn7 / locals.var_c_fox0));
        locals.var_vthq_dn10 = (locals.var_pb20_dn10 + (locals.var_t2__blk64_dn10 / locals.var_c_fox0));
        locals.var_vthq_dn11 = (locals.var_pb20_dn11 + (locals.var_t2__blk64_dn11 / locals.var_c_fox0));
        locals.var_vthq_dn12 = (locals.var_pb20_dn12 + (locals.var_t2__blk64_dn12 / locals.var_c_fox0));
        locals.var_vthq_dn17 = (locals.var_pb20_dn17 + (locals.var_t2__blk64_dn17 / locals.var_c_fox0));

        let assign5760_e3731: f64 = if locals.var_flg_qme == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard71 = assign5760_e3731;

        let (assign5770_e3735, assign5770_e3735_d_n0, assign5770_e3735_d_n2, assign5770_e3735_d_n6, assign5770_e3735_d_n7, assign5770_e3735_d_n10, assign5770_e3735_d_n11, assign5770_e3735_d_n12, assign5770_e3735_d_n17,) = {
    if (locals.var_guard71 != 0.0) {
        (locals.var_tfox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tfoxe, locals.var_tfoxe_dn0, locals.var_tfoxe_dn2, locals.var_tfoxe_dn6, locals.var_tfoxe_dn7, locals.var_tfoxe_dn10, locals.var_tfoxe_dn11, locals.var_tfoxe_dn12, locals.var_tfoxe_dn17,)
    }
};
        locals.var_tfoxe = assign5770_e3735;
        locals.var_tfoxe_dn0 = assign5770_e3735_d_n0;
        locals.var_tfoxe_dn2 = assign5770_e3735_d_n2;
        locals.var_tfoxe_dn6 = assign5770_e3735_d_n6;
        locals.var_tfoxe_dn7 = assign5770_e3735_d_n7;
        locals.var_tfoxe_dn10 = assign5770_e3735_d_n10;
        locals.var_tfoxe_dn11 = assign5770_e3735_d_n11;
        locals.var_tfoxe_dn12 = assign5770_e3735_d_n12;
        locals.var_tfoxe_dn17 = assign5770_e3735_d_n17;

        let (assign5780_e3739, assign5780_e3739_d_n0, assign5780_e3739_d_n2, assign5780_e3739_d_n6, assign5780_e3739_d_n7, assign5780_e3739_d_n10, assign5780_e3739_d_n11, assign5780_e3739_d_n12, assign5780_e3739_d_n17,) = {
    if (locals.var_guard71 != 0.0) {
        (locals.var_c_fox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_c_fox, locals.var_c_fox_dn0, locals.var_c_fox_dn2, locals.var_c_fox_dn6, locals.var_c_fox_dn7, locals.var_c_fox_dn10, locals.var_c_fox_dn11, locals.var_c_fox_dn12, locals.var_c_fox_dn17,)
    }
};
        locals.var_c_fox = assign5780_e3739;
        locals.var_c_fox_dn0 = assign5780_e3739_d_n0;
        locals.var_c_fox_dn2 = assign5780_e3739_d_n2;
        locals.var_c_fox_dn6 = assign5780_e3739_d_n6;
        locals.var_c_fox_dn7 = assign5780_e3739_d_n7;
        locals.var_c_fox_dn10 = assign5780_e3739_d_n10;
        locals.var_c_fox_dn11 = assign5780_e3739_d_n11;
        locals.var_c_fox_dn12 = assign5780_e3739_d_n12;
        locals.var_c_fox_dn17 = assign5780_e3739_d_n17;

        let (assign5790_e3743, assign5790_e3743_d_n0, assign5790_e3743_d_n2, assign5790_e3743_d_n6, assign5790_e3743_d_n7, assign5790_e3743_d_n10, assign5790_e3743_d_n11, assign5790_e3743_d_n12, assign5790_e3743_d_n17,) = {
    if (locals.var_guard71 != 0.0) {
        (locals.var_c_fox0_inv, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_c_fox_inv, locals.var_c_fox_inv_dn0, locals.var_c_fox_inv_dn2, locals.var_c_fox_inv_dn6, locals.var_c_fox_inv_dn7, locals.var_c_fox_inv_dn10, locals.var_c_fox_inv_dn11, locals.var_c_fox_inv_dn12, locals.var_c_fox_inv_dn17,)
    }
};
        locals.var_c_fox_inv = assign5790_e3743;
        locals.var_c_fox_inv_dn0 = assign5790_e3743_d_n0;
        locals.var_c_fox_inv_dn2 = assign5790_e3743_d_n2;
        locals.var_c_fox_inv_dn6 = assign5790_e3743_d_n6;
        locals.var_c_fox_inv_dn7 = assign5790_e3743_d_n7;
        locals.var_c_fox_inv_dn10 = assign5790_e3743_d_n10;
        locals.var_c_fox_inv_dn11 = assign5790_e3743_d_n11;
        locals.var_c_fox_inv_dn12 = assign5790_e3743_d_n12;
        locals.var_c_fox_inv_dn17 = assign5790_e3743_d_n17;

        let (assign5800_e3753, assign5800_e3753_d_n0, assign5800_e3753_d_n2, assign5800_e3753_d_n6, assign5800_e3753_d_n7, assign5800_e3753_d_n10, assign5800_e3753_d_n11, assign5800_e3753_d_n12, assign5800_e3753_d_n17,) = {
    if (locals.var_guard71 != 0.0) {
        let assign5800_e3747: f64 = (locals.var_cnst0soi * locals.var_c_fox0_inv);
        let assign5800_e3749: f64 = (assign5800_e3747 * locals.var_c_fox0_inv);
        let assign5800_e3751: f64 = (assign5800_e3749 * locals.var_cnst0soi);
        (assign5800_e3751, ((((locals.var_cnst0soi_dn0 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5800_e3749 * locals.var_cnst0soi_dn0)), ((((locals.var_cnst0soi_dn2 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5800_e3749 * locals.var_cnst0soi_dn2)), ((((locals.var_cnst0soi_dn6 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5800_e3749 * locals.var_cnst0soi_dn6)), ((((locals.var_cnst0soi_dn7 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5800_e3749 * locals.var_cnst0soi_dn7)), ((((locals.var_cnst0soi_dn10 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5800_e3749 * locals.var_cnst0soi_dn10)), ((((locals.var_cnst0soi_dn11 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5800_e3749 * locals.var_cnst0soi_dn11)), ((((locals.var_cnst0soi_dn12 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5800_e3749 * locals.var_cnst0soi_dn12)), ((((locals.var_cnst0soi_dn17 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5800_e3749 * locals.var_cnst0soi_dn17)),)
    } else {
        (locals.var_cnstc_foxi, locals.var_cnstc_foxi_dn0, locals.var_cnstc_foxi_dn2, locals.var_cnstc_foxi_dn6, locals.var_cnstc_foxi_dn7, locals.var_cnstc_foxi_dn10, locals.var_cnstc_foxi_dn11, locals.var_cnstc_foxi_dn12, locals.var_cnstc_foxi_dn17,)
    }
};
        locals.var_cnstc_foxi = assign5800_e3753;
        locals.var_cnstc_foxi_dn0 = assign5800_e3753_d_n0;
        locals.var_cnstc_foxi_dn2 = assign5800_e3753_d_n2;
        locals.var_cnstc_foxi_dn6 = assign5800_e3753_d_n6;
        locals.var_cnstc_foxi_dn7 = assign5800_e3753_d_n7;
        locals.var_cnstc_foxi_dn10 = assign5800_e3753_d_n10;
        locals.var_cnstc_foxi_dn11 = assign5800_e3753_d_n11;
        locals.var_cnstc_foxi_dn12 = assign5800_e3753_d_n12;
        locals.var_cnstc_foxi_dn17 = assign5800_e3753_d_n17;

        let (assign5810_e3764, assign5810_e3764_d_n0, assign5810_e3764_d_n2, assign5810_e3764_d_n6, assign5810_e3764_d_n7, assign5810_e3764_d_n10, assign5810_e3764_d_n11, assign5810_e3764_d_n12, assign5810_e3764_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let assign5810_e3758: f64 = (locals.var_vgs - locals.var_vbsp);
        let assign5810_e3760: f64 = (assign5810_e3758 - locals.var_vthq);
        let assign5810_e3762: f64 = (assign5810_e3760 + p.p205);
        (assign5810_e3762, ((-locals.var_vbsp_dn0) - locals.var_vthq_dn0), ((-locals.var_vbsp_dn2) - locals.var_vthq_dn2), ((locals.var_vgs_dn6 - locals.var_vbsp_dn6) - locals.var_vthq_dn6), ((locals.var_vgs_dn7 - locals.var_vbsp_dn7) - locals.var_vthq_dn7), ((-locals.var_vbsp_dn10) - locals.var_vthq_dn10), ((locals.var_vgs_dn11 - locals.var_vbsp_dn11) - locals.var_vthq_dn11), ((-locals.var_vbsp_dn12) - locals.var_vthq_dn12), ((-locals.var_vbsp_dn17) - locals.var_vthq_dn17),)
    } else {
        (locals.var_t5__blk68, locals.var_t5__blk68_dn0, locals.var_t5__blk68_dn2, locals.var_t5__blk68_dn6, locals.var_t5__blk68_dn7, locals.var_t5__blk68_dn10, locals.var_t5__blk68_dn11, locals.var_t5__blk68_dn12, locals.var_t5__blk68_dn17,)
    }
};
        locals.var_t5__blk68 = assign5810_e3764;
        locals.var_t5__blk68_dn0 = assign5810_e3764_d_n0;
        locals.var_t5__blk68_dn2 = assign5810_e3764_d_n2;
        locals.var_t5__blk68_dn6 = assign5810_e3764_d_n6;
        locals.var_t5__blk68_dn7 = assign5810_e3764_d_n7;
        locals.var_t5__blk68_dn10 = assign5810_e3764_d_n10;
        locals.var_t5__blk68_dn11 = assign5810_e3764_d_n11;
        locals.var_t5__blk68_dn12 = assign5810_e3764_d_n12;
        locals.var_t5__blk68_dn17 = assign5810_e3764_d_n17;

        let (assign5820_e3778, assign5820_e3778_d_n0, assign5820_e3778_d_n2, assign5820_e3778_d_n6, assign5820_e3778_d_n7, assign5820_e3778_d_n10, assign5820_e3778_d_n11, assign5820_e3778_d_n12, assign5820_e3778_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let assign5820_e3769: f64 = (locals.var_t5__blk68 * locals.var_t5__blk68);
        let assign5820_e3772: f64 = (4.0 * 0.0001);
        let assign5820_e3774: f64 = (assign5820_e3772 * 0.0001);
        let assign5820_e3775: f64 = (assign5820_e3769 + assign5820_e3774);
        let assign5820_e3776: f64 = (assign5820_e3775).sqrt();
        (assign5820_e3776, (((locals.var_t5__blk68_dn0 * locals.var_t5__blk68) + (locals.var_t5__blk68 * locals.var_t5__blk68_dn0)) / (2.0 * assign5820_e3776)), (((locals.var_t5__blk68_dn2 * locals.var_t5__blk68) + (locals.var_t5__blk68 * locals.var_t5__blk68_dn2)) / (2.0 * assign5820_e3776)), (((locals.var_t5__blk68_dn6 * locals.var_t5__blk68) + (locals.var_t5__blk68 * locals.var_t5__blk68_dn6)) / (2.0 * assign5820_e3776)), (((locals.var_t5__blk68_dn7 * locals.var_t5__blk68) + (locals.var_t5__blk68 * locals.var_t5__blk68_dn7)) / (2.0 * assign5820_e3776)), (((locals.var_t5__blk68_dn10 * locals.var_t5__blk68) + (locals.var_t5__blk68 * locals.var_t5__blk68_dn10)) / (2.0 * assign5820_e3776)), (((locals.var_t5__blk68_dn11 * locals.var_t5__blk68) + (locals.var_t5__blk68 * locals.var_t5__blk68_dn11)) / (2.0 * assign5820_e3776)), (((locals.var_t5__blk68_dn12 * locals.var_t5__blk68) + (locals.var_t5__blk68 * locals.var_t5__blk68_dn12)) / (2.0 * assign5820_e3776)), (((locals.var_t5__blk68_dn17 * locals.var_t5__blk68) + (locals.var_t5__blk68 * locals.var_t5__blk68_dn17)) / (2.0 * assign5820_e3776)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign5820_e3778;
        locals.var_tmf1_dn0 = assign5820_e3778_d_n0;
        locals.var_tmf1_dn2 = assign5820_e3778_d_n2;
        locals.var_tmf1_dn6 = assign5820_e3778_d_n6;
        locals.var_tmf1_dn7 = assign5820_e3778_d_n7;
        locals.var_tmf1_dn10 = assign5820_e3778_d_n10;
        locals.var_tmf1_dn11 = assign5820_e3778_d_n11;
        locals.var_tmf1_dn12 = assign5820_e3778_d_n12;
        locals.var_tmf1_dn17 = assign5820_e3778_d_n17;

        let (assign5830_e3791, assign5830_e3791_d_n0, assign5830_e3791_d_n2, assign5830_e3791_d_n6, assign5830_e3791_d_n7, assign5830_e3791_d_n10, assign5830_e3791_d_n11, assign5830_e3791_d_n12, assign5830_e3791_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let assign5830_e3784: f64 = (locals.var_t5__blk68 + locals.var_tmf1);
        let assign5830_e3785: f64 = (0.5 * assign5830_e3784);
        let assign5830_e3788: f64 = (1e-10 * 0.0001);
        let assign5830_e3789: f64 = (assign5830_e3785 + assign5830_e3788);
        (assign5830_e3789, (0.5 * (locals.var_t5__blk68_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t5__blk68_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t5__blk68_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t5__blk68_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t5__blk68_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t5__blk68_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t5__blk68_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t5__blk68_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t2__blk64, locals.var_t2__blk64_dn0, locals.var_t2__blk64_dn2, locals.var_t2__blk64_dn6, locals.var_t2__blk64_dn7, locals.var_t2__blk64_dn10, locals.var_t2__blk64_dn11, locals.var_t2__blk64_dn12, locals.var_t2__blk64_dn17,)
    }
};
        locals.var_t2__blk64 = assign5830_e3791;
        locals.var_t2__blk64_dn0 = assign5830_e3791_d_n0;
        locals.var_t2__blk64_dn2 = assign5830_e3791_d_n2;
        locals.var_t2__blk64_dn6 = assign5830_e3791_d_n6;
        locals.var_t2__blk64_dn7 = assign5830_e3791_d_n7;
        locals.var_t2__blk64_dn10 = assign5830_e3791_d_n10;
        locals.var_t2__blk64_dn11 = assign5830_e3791_d_n11;
        locals.var_t2__blk64_dn12 = assign5830_e3791_d_n12;
        locals.var_t2__blk64_dn17 = assign5830_e3791_d_n17;

        let assign5840_e3794: f64 = if locals.var_t2__blk64 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard72 = assign5840_e3794;

        let (assign5850_e3801, assign5850_e3801_d_n0, assign5850_e3801_d_n2, assign5850_e3801_d_n6, assign5850_e3801_d_n7, assign5850_e3801_d_n10, assign5850_e3801_d_n11, assign5850_e3801_d_n12, assign5850_e3801_d_n17,) = {
    if ((locals.var_guard71 == 0.0) && (locals.var_guard72 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk64, locals.var_t2__blk64_dn0, locals.var_t2__blk64_dn2, locals.var_t2__blk64_dn6, locals.var_t2__blk64_dn7, locals.var_t2__blk64_dn10, locals.var_t2__blk64_dn11, locals.var_t2__blk64_dn12, locals.var_t2__blk64_dn17,)
    }
};
        locals.var_t2__blk64 = assign5850_e3801;
        locals.var_t2__blk64_dn0 = assign5850_e3801_d_n0;
        locals.var_t2__blk64_dn2 = assign5850_e3801_d_n2;
        locals.var_t2__blk64_dn6 = assign5850_e3801_d_n6;
        locals.var_t2__blk64_dn7 = assign5850_e3801_d_n7;
        locals.var_t2__blk64_dn10 = assign5850_e3801_d_n10;
        locals.var_t2__blk64_dn11 = assign5850_e3801_d_n11;
        locals.var_t2__blk64_dn12 = assign5850_e3801_d_n12;
        locals.var_t2__blk64_dn17 = assign5850_e3801_d_n17;

        let (assign5860_e3808, assign5860_e3808_d_n0, assign5860_e3808_d_n2, assign5860_e3808_d_n6, assign5860_e3808_d_n7, assign5860_e3808_d_n10, assign5860_e3808_d_n11, assign5860_e3808_d_n12, assign5860_e3808_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let assign5860_e3806: f64 = (1.0 / locals.var_t2__blk64);
        (assign5860_e3806, (-(locals.var_t2__blk64_dn0 / (locals.var_t2__blk64 * locals.var_t2__blk64))), (-(locals.var_t2__blk64_dn2 / (locals.var_t2__blk64 * locals.var_t2__blk64))), (-(locals.var_t2__blk64_dn6 / (locals.var_t2__blk64 * locals.var_t2__blk64))), (-(locals.var_t2__blk64_dn7 / (locals.var_t2__blk64 * locals.var_t2__blk64))), (-(locals.var_t2__blk64_dn10 / (locals.var_t2__blk64 * locals.var_t2__blk64))), (-(locals.var_t2__blk64_dn11 / (locals.var_t2__blk64 * locals.var_t2__blk64))), (-(locals.var_t2__blk64_dn12 / (locals.var_t2__blk64 * locals.var_t2__blk64))), (-(locals.var_t2__blk64_dn17 / (locals.var_t2__blk64 * locals.var_t2__blk64))),)
    } else {
        (locals.var_t3__blk65, locals.var_t3__blk65_dn0, locals.var_t3__blk65_dn2, locals.var_t3__blk65_dn6, locals.var_t3__blk65_dn7, locals.var_t3__blk65_dn10, locals.var_t3__blk65_dn11, locals.var_t3__blk65_dn12, locals.var_t3__blk65_dn17,)
    }
};
        locals.var_t3__blk65 = assign5860_e3808;
        locals.var_t3__blk65_dn0 = assign5860_e3808_d_n0;
        locals.var_t3__blk65_dn2 = assign5860_e3808_d_n2;
        locals.var_t3__blk65_dn6 = assign5860_e3808_d_n6;
        locals.var_t3__blk65_dn7 = assign5860_e3808_d_n7;
        locals.var_t3__blk65_dn10 = assign5860_e3808_d_n10;
        locals.var_t3__blk65_dn11 = assign5860_e3808_d_n11;
        locals.var_t3__blk65_dn12 = assign5860_e3808_d_n12;
        locals.var_t3__blk65_dn17 = assign5860_e3808_d_n17;

        let (assign5870_e3816, assign5870_e3816_d_n0, assign5870_e3816_d_n2, assign5870_e3816_d_n6, assign5870_e3816_d_n7, assign5870_e3816_d_n10, assign5870_e3816_d_n11, assign5870_e3816_d_n12, assign5870_e3816_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let assign5870_e3813: f64 = (locals.var_vthq).abs();
        let assign5870_e3814: f64 = (2.0 * assign5870_e3813);
        (assign5870_e3814, (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn0 } else { (-locals.var_vthq_dn0) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn2 } else { (-locals.var_vthq_dn2) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn6 } else { (-locals.var_vthq_dn6) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn7 } else { (-locals.var_vthq_dn7) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn10 } else { (-locals.var_vthq_dn10) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn11 } else { (-locals.var_vthq_dn11) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn12 } else { (-locals.var_vthq_dn12) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn17 } else { (-locals.var_vthq_dn17) }),)
    } else {
        (locals.var_t4w, locals.var_t4w_dn0, locals.var_t4w_dn2, locals.var_t4w_dn6, locals.var_t4w_dn7, locals.var_t4w_dn10, locals.var_t4w_dn11, locals.var_t4w_dn12, locals.var_t4w_dn17,)
    }
};
        locals.var_t4w = assign5870_e3816;
        locals.var_t4w_dn0 = assign5870_e3816_d_n0;
        locals.var_t4w_dn2 = assign5870_e3816_d_n2;
        locals.var_t4w_dn6 = assign5870_e3816_d_n6;
        locals.var_t4w_dn7 = assign5870_e3816_d_n7;
        locals.var_t4w_dn10 = assign5870_e3816_d_n10;
        locals.var_t4w_dn11 = assign5870_e3816_d_n11;
        locals.var_t4w_dn12 = assign5870_e3816_d_n12;
        locals.var_t4w_dn17 = assign5870_e3816_d_n17;

        let (assign5880_e3825, assign5880_e3825_d_n0, assign5880_e3825_d_n2, assign5880_e3825_d_n6, assign5880_e3825_d_n7, assign5880_e3825_d_n10, assign5880_e3825_d_n11, assign5880_e3825_d_n12, assign5880_e3825_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let assign5880_e3821: f64 = (locals.var_vfb - locals.var_vthq);
        let assign5880_e3823: f64 = (assign5880_e3821 + p.p205);
        (assign5880_e3823, (-locals.var_vthq_dn0), (-locals.var_vthq_dn2), (-locals.var_vthq_dn6), (-locals.var_vthq_dn7), (-locals.var_vthq_dn10), (-locals.var_vthq_dn11), (-locals.var_vthq_dn12), (-locals.var_vthq_dn17),)
    } else {
        (locals.var_t6__blk69, locals.var_t6__blk69_dn0, locals.var_t6__blk69_dn2, locals.var_t6__blk69_dn6, locals.var_t6__blk69_dn7, locals.var_t6__blk69_dn10, locals.var_t6__blk69_dn11, locals.var_t6__blk69_dn12, locals.var_t6__blk69_dn17,)
    }
};
        locals.var_t6__blk69 = assign5880_e3825;
        locals.var_t6__blk69_dn0 = assign5880_e3825_d_n0;
        locals.var_t6__blk69_dn2 = assign5880_e3825_d_n2;
        locals.var_t6__blk69_dn6 = assign5880_e3825_d_n6;
        locals.var_t6__blk69_dn7 = assign5880_e3825_d_n7;
        locals.var_t6__blk69_dn10 = assign5880_e3825_d_n10;
        locals.var_t6__blk69_dn11 = assign5880_e3825_d_n11;
        locals.var_t6__blk69_dn12 = assign5880_e3825_d_n12;
        locals.var_t6__blk69_dn17 = assign5880_e3825_d_n17;

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5890_e3835, assign5890_e3835_d_n0, assign5890_e3835_d_n2, assign5890_e3835_d_n6, assign5890_e3835_d_n7, assign5890_e3835_d_n10, assign5890_e3835_d_n11, assign5890_e3835_d_n12, assign5890_e3835_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let (assign5890_e3833, assign5890_e3833_d_n0, assign5890_e3833_d_n2, assign5890_e3833_d_n6, assign5890_e3833_d_n7, assign5890_e3833_d_n10, assign5890_e3833_d_n11, assign5890_e3833_d_n12, assign5890_e3833_d_n17,) = {
            if (locals.var_t6__blk69 > locals.var_t4w) {
                (locals.var_t6__blk69, locals.var_t6__blk69_dn0, locals.var_t6__blk69_dn2, locals.var_t6__blk69_dn6, locals.var_t6__blk69_dn7, locals.var_t6__blk69_dn10, locals.var_t6__blk69_dn11, locals.var_t6__blk69_dn12, locals.var_t6__blk69_dn17,)
            } else {
                (locals.var_t4w, locals.var_t4w_dn0, locals.var_t4w_dn2, locals.var_t4w_dn6, locals.var_t4w_dn7, locals.var_t4w_dn10, locals.var_t4w_dn11, locals.var_t4w_dn12, locals.var_t4w_dn17,)
            }
        };
        (assign5890_e3833, assign5890_e3833_d_n0, assign5890_e3833_d_n2, assign5890_e3833_d_n6, assign5890_e3833_d_n7, assign5890_e3833_d_n10, assign5890_e3833_d_n11, assign5890_e3833_d_n12, assign5890_e3833_d_n17,)
    } else {
        (locals.var_t4__blk66, locals.var_t4__blk66_dn0, locals.var_t4__blk66_dn2, locals.var_t4__blk66_dn6, locals.var_t4__blk66_dn7, locals.var_t4__blk66_dn10, locals.var_t4__blk66_dn11, locals.var_t4__blk66_dn12, locals.var_t4__blk66_dn17,)
    }
};
        locals.var_t4__blk66 = assign5890_e3835;
        locals.var_t4__blk66_dn0 = assign5890_e3835_d_n0;
        locals.var_t4__blk66_dn2 = assign5890_e3835_d_n2;
        locals.var_t4__blk66_dn6 = assign5890_e3835_d_n6;
        locals.var_t4__blk66_dn7 = assign5890_e3835_d_n7;
        locals.var_t4__blk66_dn10 = assign5890_e3835_d_n10;
        locals.var_t4__blk66_dn11 = assign5890_e3835_d_n11;
        locals.var_t4__blk66_dn12 = assign5890_e3835_d_n12;
        locals.var_t4__blk66_dn17 = assign5890_e3835_d_n17;

        let (assign5900_e3846, assign5900_e3846_d_n0, assign5900_e3846_d_n2, assign5900_e3846_d_n6, assign5900_e3846_d_n7, assign5900_e3846_d_n10, assign5900_e3846_d_n11, assign5900_e3846_d_n12, assign5900_e3846_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let assign5900_e3840: f64 = (1.0 / locals.var_t4__blk66);
        let assign5900_e3842: f64 = (assign5900_e3840 - locals.var_t3__blk65);
        let assign5900_e3844: f64 = (assign5900_e3842 - 0.0001);
        (assign5900_e3844, ((-(locals.var_t4__blk66_dn0 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - locals.var_t3__blk65_dn0), ((-(locals.var_t4__blk66_dn2 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - locals.var_t3__blk65_dn2), ((-(locals.var_t4__blk66_dn6 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - locals.var_t3__blk65_dn6), ((-(locals.var_t4__blk66_dn7 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - locals.var_t3__blk65_dn7), ((-(locals.var_t4__blk66_dn10 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - locals.var_t3__blk65_dn10), ((-(locals.var_t4__blk66_dn11 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - locals.var_t3__blk65_dn11), ((-(locals.var_t4__blk66_dn12 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - locals.var_t3__blk65_dn12), ((-(locals.var_t4__blk66_dn17 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - locals.var_t3__blk65_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign5900_e3846;
        locals.var_tmf1_dn0 = assign5900_e3846_d_n0;
        locals.var_tmf1_dn2 = assign5900_e3846_d_n2;
        locals.var_tmf1_dn6 = assign5900_e3846_d_n6;
        locals.var_tmf1_dn7 = assign5900_e3846_d_n7;
        locals.var_tmf1_dn10 = assign5900_e3846_d_n10;
        locals.var_tmf1_dn11 = assign5900_e3846_d_n11;
        locals.var_tmf1_dn12 = assign5900_e3846_d_n12;
        locals.var_tmf1_dn17 = assign5900_e3846_d_n17;

        let (assign5910_e3857, assign5910_e3857_d_n0, assign5910_e3857_d_n2, assign5910_e3857_d_n6, assign5910_e3857_d_n7, assign5910_e3857_d_n10, assign5910_e3857_d_n11, assign5910_e3857_d_n12, assign5910_e3857_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let assign5910_e3852: f64 = (1.0 / locals.var_t4__blk66);
        let assign5910_e3853: f64 = (4.0 * assign5910_e3852);
        let assign5910_e3855: f64 = (assign5910_e3853 * 0.0001);
        (assign5910_e3855, ((4.0 * (-(locals.var_t4__blk66_dn0 / (locals.var_t4__blk66 * locals.var_t4__blk66)))) * 0.0001), ((4.0 * (-(locals.var_t4__blk66_dn2 / (locals.var_t4__blk66 * locals.var_t4__blk66)))) * 0.0001), ((4.0 * (-(locals.var_t4__blk66_dn6 / (locals.var_t4__blk66 * locals.var_t4__blk66)))) * 0.0001), ((4.0 * (-(locals.var_t4__blk66_dn7 / (locals.var_t4__blk66 * locals.var_t4__blk66)))) * 0.0001), ((4.0 * (-(locals.var_t4__blk66_dn10 / (locals.var_t4__blk66 * locals.var_t4__blk66)))) * 0.0001), ((4.0 * (-(locals.var_t4__blk66_dn11 / (locals.var_t4__blk66 * locals.var_t4__blk66)))) * 0.0001), ((4.0 * (-(locals.var_t4__blk66_dn12 / (locals.var_t4__blk66 * locals.var_t4__blk66)))) * 0.0001), ((4.0 * (-(locals.var_t4__blk66_dn17 / (locals.var_t4__blk66 * locals.var_t4__blk66)))) * 0.0001),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign5910_e3857;
        locals.var_tmf2_dn0 = assign5910_e3857_d_n0;
        locals.var_tmf2_dn2 = assign5910_e3857_d_n2;
        locals.var_tmf2_dn6 = assign5910_e3857_d_n6;
        locals.var_tmf2_dn7 = assign5910_e3857_d_n7;
        locals.var_tmf2_dn10 = assign5910_e3857_d_n10;
        locals.var_tmf2_dn11 = assign5910_e3857_d_n11;
        locals.var_tmf2_dn12 = assign5910_e3857_d_n12;
        locals.var_tmf2_dn17 = assign5910_e3857_d_n17;

        let (assign5920_e3868, assign5920_e3868_d_n0, assign5920_e3868_d_n2, assign5920_e3868_d_n6, assign5920_e3868_d_n7, assign5920_e3868_d_n10, assign5920_e3868_d_n11, assign5920_e3868_d_n12, assign5920_e3868_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let (assign5920_e3866, assign5920_e3866_d_n0, assign5920_e3866_d_n2, assign5920_e3866_d_n6, assign5920_e3866_d_n7, assign5920_e3866_d_n10, assign5920_e3866_d_n11, assign5920_e3866_d_n12, assign5920_e3866_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign5920_e3865: f64 = (-locals.var_tmf2);
                (assign5920_e3865, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign5920_e3866, assign5920_e3866_d_n0, assign5920_e3866_d_n2, assign5920_e3866_d_n6, assign5920_e3866_d_n7, assign5920_e3866_d_n10, assign5920_e3866_d_n11, assign5920_e3866_d_n12, assign5920_e3866_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign5920_e3868;
        locals.var_tmf2_dn0 = assign5920_e3868_d_n0;
        locals.var_tmf2_dn2 = assign5920_e3868_d_n2;
        locals.var_tmf2_dn6 = assign5920_e3868_d_n6;
        locals.var_tmf2_dn7 = assign5920_e3868_d_n7;
        locals.var_tmf2_dn10 = assign5920_e3868_d_n10;
        locals.var_tmf2_dn11 = assign5920_e3868_d_n11;
        locals.var_tmf2_dn12 = assign5920_e3868_d_n12;
        locals.var_tmf2_dn17 = assign5920_e3868_d_n17;

        let (assign5930_e3878, assign5930_e3878_d_n0, assign5930_e3878_d_n2, assign5930_e3878_d_n6, assign5930_e3878_d_n7, assign5930_e3878_d_n10, assign5930_e3878_d_n11, assign5930_e3878_d_n12, assign5930_e3878_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let assign5930_e3873: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5930_e3875: f64 = (assign5930_e3873 + locals.var_tmf2);
        let assign5930_e3876: f64 = (assign5930_e3875).sqrt();
        (assign5930_e3876, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5930_e3876)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5930_e3876)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign5930_e3876)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign5930_e3876)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign5930_e3876)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign5930_e3876)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign5930_e3876)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign5930_e3876)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign5930_e3878;
        locals.var_tmf2_dn0 = assign5930_e3878_d_n0;
        locals.var_tmf2_dn2 = assign5930_e3878_d_n2;
        locals.var_tmf2_dn6 = assign5930_e3878_d_n6;
        locals.var_tmf2_dn7 = assign5930_e3878_d_n7;
        locals.var_tmf2_dn10 = assign5930_e3878_d_n10;
        locals.var_tmf2_dn11 = assign5930_e3878_d_n11;
        locals.var_tmf2_dn12 = assign5930_e3878_d_n12;
        locals.var_tmf2_dn17 = assign5930_e3878_d_n17;

        let (assign5940_e3891, assign5940_e3891_d_n0, assign5940_e3891_d_n2, assign5940_e3891_d_n6, assign5940_e3891_d_n7, assign5940_e3891_d_n10, assign5940_e3891_d_n11, assign5940_e3891_d_n12, assign5940_e3891_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let assign5940_e3883: f64 = (1.0 / locals.var_t4__blk66);
        let assign5940_e3887: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5940_e3888: f64 = (0.5 * assign5940_e3887);
        let assign5940_e3889: f64 = (assign5940_e3883 - assign5940_e3888);
        (assign5940_e3889, ((-(locals.var_t4__blk66_dn0 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-(locals.var_t4__blk66_dn2 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-(locals.var_t4__blk66_dn6 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-(locals.var_t4__blk66_dn7 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-(locals.var_t4__blk66_dn10 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-(locals.var_t4__blk66_dn11 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-(locals.var_t4__blk66_dn12 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-(locals.var_t4__blk66_dn17 / (locals.var_t4__blk66 * locals.var_t4__blk66))) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t2__blk64, locals.var_t2__blk64_dn0, locals.var_t2__blk64_dn2, locals.var_t2__blk64_dn6, locals.var_t2__blk64_dn7, locals.var_t2__blk64_dn10, locals.var_t2__blk64_dn11, locals.var_t2__blk64_dn12, locals.var_t2__blk64_dn17,)
    }
};
        locals.var_t2__blk64 = assign5940_e3891;
        locals.var_t2__blk64_dn0 = assign5940_e3891_d_n0;
        locals.var_t2__blk64_dn2 = assign5940_e3891_d_n2;
        locals.var_t2__blk64_dn6 = assign5940_e3891_d_n6;
        locals.var_t2__blk64_dn7 = assign5940_e3891_d_n7;
        locals.var_t2__blk64_dn10 = assign5940_e3891_d_n10;
        locals.var_t2__blk64_dn11 = assign5940_e3891_d_n11;
        locals.var_t2__blk64_dn12 = assign5940_e3891_d_n12;
        locals.var_t2__blk64_dn17 = assign5940_e3891_d_n17;

        let (assign5950_e3900, assign5950_e3900_d_n0, assign5950_e3900_d_n2, assign5950_e3900_d_n6, assign5950_e3900_d_n7, assign5950_e3900_d_n10, assign5950_e3900_d_n11, assign5950_e3900_d_n12, assign5950_e3900_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let assign5950_e3896: f64 = (p.p204 * locals.var_t2__blk64);
        let assign5950_e3898: f64 = (assign5950_e3896 + p.p206);
        (assign5950_e3898, (p.p204 * locals.var_t2__blk64_dn0), (p.p204 * locals.var_t2__blk64_dn2), (p.p204 * locals.var_t2__blk64_dn6), (p.p204 * locals.var_t2__blk64_dn7), (p.p204 * locals.var_t2__blk64_dn10), (p.p204 * locals.var_t2__blk64_dn11), (p.p204 * locals.var_t2__blk64_dn12), (p.p204 * locals.var_t2__blk64_dn17),)
    } else {
        (locals.var_dtfox, locals.var_dtfox_dn0, locals.var_dtfox_dn2, locals.var_dtfox_dn6, locals.var_dtfox_dn7, locals.var_dtfox_dn10, locals.var_dtfox_dn11, locals.var_dtfox_dn12, locals.var_dtfox_dn17,)
    }
};
        locals.var_dtfox = assign5950_e3900;
        locals.var_dtfox_dn0 = assign5950_e3900_d_n0;
        locals.var_dtfox_dn2 = assign5950_e3900_d_n2;
        locals.var_dtfox_dn6 = assign5950_e3900_d_n6;
        locals.var_dtfox_dn7 = assign5950_e3900_d_n7;
        locals.var_dtfox_dn10 = assign5950_e3900_d_n10;
        locals.var_dtfox_dn11 = assign5950_e3900_d_n11;
        locals.var_dtfox_dn12 = assign5950_e3900_d_n12;
        locals.var_dtfox_dn17 = assign5950_e3900_d_n17;

        let assign5960_e3903: f64 = (locals.var_dtfox * 1000000000000.0);
        let assign5960_e3905: f64 = if assign5960_e3903 < locals.var_tfox0 { 1.0 } else { 0.0 };
        locals.var_guard73 = assign5960_e3905;

        let (assign5970_e3912, assign5970_e3912_d_n0, assign5970_e3912_d_n2, assign5970_e3912_d_n6, assign5970_e3912_d_n7, assign5970_e3912_d_n10, assign5970_e3912_d_n11, assign5970_e3912_d_n12, assign5970_e3912_d_n17,) = {
    if ((locals.var_guard71 == 0.0) && (locals.var_guard73 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dtfox, locals.var_dtfox_dn0, locals.var_dtfox_dn2, locals.var_dtfox_dn6, locals.var_dtfox_dn7, locals.var_dtfox_dn10, locals.var_dtfox_dn11, locals.var_dtfox_dn12, locals.var_dtfox_dn17,)
    }
};
        locals.var_dtfox = assign5970_e3912;
        locals.var_dtfox_dn0 = assign5970_e3912_d_n0;
        locals.var_dtfox_dn2 = assign5970_e3912_d_n2;
        locals.var_dtfox_dn6 = assign5970_e3912_d_n6;
        locals.var_dtfox_dn7 = assign5970_e3912_d_n7;
        locals.var_dtfox_dn10 = assign5970_e3912_d_n10;
        locals.var_dtfox_dn11 = assign5970_e3912_d_n11;
        locals.var_dtfox_dn12 = assign5970_e3912_d_n12;
        locals.var_dtfox_dn17 = assign5970_e3912_d_n17;

        let (assign5980_e3919,) = {
    if ((locals.var_guard71 == 0.0) && (locals.var_guard73 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign5980_e3919;

        let (assign5990_e3926, assign5990_e3926_d_n0, assign5990_e3926_d_n2, assign5990_e3926_d_n6, assign5990_e3926_d_n7, assign5990_e3926_d_n10, assign5990_e3926_d_n11, assign5990_e3926_d_n12, assign5990_e3926_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let assign5990_e3924: f64 = (locals.var_tfox0 + locals.var_dtfox);
        (assign5990_e3924, locals.var_dtfox_dn0, locals.var_dtfox_dn2, locals.var_dtfox_dn6, locals.var_dtfox_dn7, locals.var_dtfox_dn10, locals.var_dtfox_dn11, locals.var_dtfox_dn12, locals.var_dtfox_dn17,)
    } else {
        (locals.var_tfoxe, locals.var_tfoxe_dn0, locals.var_tfoxe_dn2, locals.var_tfoxe_dn6, locals.var_tfoxe_dn7, locals.var_tfoxe_dn10, locals.var_tfoxe_dn11, locals.var_tfoxe_dn12, locals.var_tfoxe_dn17,)
    }
};
        locals.var_tfoxe = assign5990_e3926;
        locals.var_tfoxe_dn0 = assign5990_e3926_d_n0;
        locals.var_tfoxe_dn2 = assign5990_e3926_d_n2;
        locals.var_tfoxe_dn6 = assign5990_e3926_d_n6;
        locals.var_tfoxe_dn7 = assign5990_e3926_d_n7;
        locals.var_tfoxe_dn10 = assign5990_e3926_d_n10;
        locals.var_tfoxe_dn11 = assign5990_e3926_d_n11;
        locals.var_tfoxe_dn12 = assign5990_e3926_d_n12;
        locals.var_tfoxe_dn17 = assign5990_e3926_d_n17;

        let (assign6000_e3933, assign6000_e3933_d_n0, assign6000_e3933_d_n2, assign6000_e3933_d_n6, assign6000_e3933_d_n7, assign6000_e3933_d_n10, assign6000_e3933_d_n11, assign6000_e3933_d_n12, assign6000_e3933_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let assign6000_e3931: f64 = (3.453133e-11 / locals.var_tfoxe);
        (assign6000_e3931, (-((3.453133e-11 * locals.var_tfoxe_dn0) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn2) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn6) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn7) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn10) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn11) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn12) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn17) / (locals.var_tfoxe * locals.var_tfoxe))),)
    } else {
        (locals.var_c_fox, locals.var_c_fox_dn0, locals.var_c_fox_dn2, locals.var_c_fox_dn6, locals.var_c_fox_dn7, locals.var_c_fox_dn10, locals.var_c_fox_dn11, locals.var_c_fox_dn12, locals.var_c_fox_dn17,)
    }
};
        locals.var_c_fox = assign6000_e3933;
        locals.var_c_fox_dn0 = assign6000_e3933_d_n0;
        locals.var_c_fox_dn2 = assign6000_e3933_d_n2;
        locals.var_c_fox_dn6 = assign6000_e3933_d_n6;
        locals.var_c_fox_dn7 = assign6000_e3933_d_n7;
        locals.var_c_fox_dn10 = assign6000_e3933_d_n10;
        locals.var_c_fox_dn11 = assign6000_e3933_d_n11;
        locals.var_c_fox_dn12 = assign6000_e3933_d_n12;
        locals.var_c_fox_dn17 = assign6000_e3933_d_n17;

        let (assign6010_e3940, assign6010_e3940_d_n0, assign6010_e3940_d_n2, assign6010_e3940_d_n6, assign6010_e3940_d_n7, assign6010_e3940_d_n10, assign6010_e3940_d_n11, assign6010_e3940_d_n12, assign6010_e3940_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let assign6010_e3938: f64 = (locals.var_tfoxe / 3.453133e-11);
        (assign6010_e3938, (locals.var_tfoxe_dn0 / 3.453133e-11), (locals.var_tfoxe_dn2 / 3.453133e-11), (locals.var_tfoxe_dn6 / 3.453133e-11), (locals.var_tfoxe_dn7 / 3.453133e-11), (locals.var_tfoxe_dn10 / 3.453133e-11), (locals.var_tfoxe_dn11 / 3.453133e-11), (locals.var_tfoxe_dn12 / 3.453133e-11), (locals.var_tfoxe_dn17 / 3.453133e-11),)
    } else {
        (locals.var_c_fox_inv, locals.var_c_fox_inv_dn0, locals.var_c_fox_inv_dn2, locals.var_c_fox_inv_dn6, locals.var_c_fox_inv_dn7, locals.var_c_fox_inv_dn10, locals.var_c_fox_inv_dn11, locals.var_c_fox_inv_dn12, locals.var_c_fox_inv_dn17,)
    }
};
        locals.var_c_fox_inv = assign6010_e3940;
        locals.var_c_fox_inv_dn0 = assign6010_e3940_d_n0;
        locals.var_c_fox_inv_dn2 = assign6010_e3940_d_n2;
        locals.var_c_fox_inv_dn6 = assign6010_e3940_d_n6;
        locals.var_c_fox_inv_dn7 = assign6010_e3940_d_n7;
        locals.var_c_fox_inv_dn10 = assign6010_e3940_d_n10;
        locals.var_c_fox_inv_dn11 = assign6010_e3940_d_n11;
        locals.var_c_fox_inv_dn12 = assign6010_e3940_d_n12;
        locals.var_c_fox_inv_dn17 = assign6010_e3940_d_n17;

        let (assign6020_e3951, assign6020_e3951_d_n0, assign6020_e3951_d_n2, assign6020_e3951_d_n6, assign6020_e3951_d_n7, assign6020_e3951_d_n10, assign6020_e3951_d_n11, assign6020_e3951_d_n12, assign6020_e3951_d_n17,) = {
    if (locals.var_guard71 == 0.0) {
        let assign6020_e3945: f64 = (locals.var_cnst0soi * locals.var_cnst0soi);
        let assign6020_e3947: f64 = (assign6020_e3945 * locals.var_c_fox_inv);
        let assign6020_e3949: f64 = (assign6020_e3947 * locals.var_c_fox_inv);
        (assign6020_e3949, ((((((locals.var_cnst0soi_dn0 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn0)) * locals.var_c_fox_inv) + (assign6020_e3945 * locals.var_c_fox_inv_dn0)) * locals.var_c_fox_inv) + (assign6020_e3947 * locals.var_c_fox_inv_dn0)), ((((((locals.var_cnst0soi_dn2 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn2)) * locals.var_c_fox_inv) + (assign6020_e3945 * locals.var_c_fox_inv_dn2)) * locals.var_c_fox_inv) + (assign6020_e3947 * locals.var_c_fox_inv_dn2)), ((((((locals.var_cnst0soi_dn6 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn6)) * locals.var_c_fox_inv) + (assign6020_e3945 * locals.var_c_fox_inv_dn6)) * locals.var_c_fox_inv) + (assign6020_e3947 * locals.var_c_fox_inv_dn6)), ((((((locals.var_cnst0soi_dn7 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn7)) * locals.var_c_fox_inv) + (assign6020_e3945 * locals.var_c_fox_inv_dn7)) * locals.var_c_fox_inv) + (assign6020_e3947 * locals.var_c_fox_inv_dn7)), ((((((locals.var_cnst0soi_dn10 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn10)) * locals.var_c_fox_inv) + (assign6020_e3945 * locals.var_c_fox_inv_dn10)) * locals.var_c_fox_inv) + (assign6020_e3947 * locals.var_c_fox_inv_dn10)), ((((((locals.var_cnst0soi_dn11 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn11)) * locals.var_c_fox_inv) + (assign6020_e3945 * locals.var_c_fox_inv_dn11)) * locals.var_c_fox_inv) + (assign6020_e3947 * locals.var_c_fox_inv_dn11)), ((((((locals.var_cnst0soi_dn12 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn12)) * locals.var_c_fox_inv) + (assign6020_e3945 * locals.var_c_fox_inv_dn12)) * locals.var_c_fox_inv) + (assign6020_e3947 * locals.var_c_fox_inv_dn12)), ((((((locals.var_cnst0soi_dn17 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn17)) * locals.var_c_fox_inv) + (assign6020_e3945 * locals.var_c_fox_inv_dn17)) * locals.var_c_fox_inv) + (assign6020_e3947 * locals.var_c_fox_inv_dn17)),)
    } else {
        (locals.var_cnstc_foxi, locals.var_cnstc_foxi_dn0, locals.var_cnstc_foxi_dn2, locals.var_cnstc_foxi_dn6, locals.var_cnstc_foxi_dn7, locals.var_cnstc_foxi_dn10, locals.var_cnstc_foxi_dn11, locals.var_cnstc_foxi_dn12, locals.var_cnstc_foxi_dn17,)
    }
};
        locals.var_cnstc_foxi = assign6020_e3951;
        locals.var_cnstc_foxi_dn0 = assign6020_e3951_d_n0;
        locals.var_cnstc_foxi_dn2 = assign6020_e3951_d_n2;
        locals.var_cnstc_foxi_dn6 = assign6020_e3951_d_n6;
        locals.var_cnstc_foxi_dn7 = assign6020_e3951_d_n7;
        locals.var_cnstc_foxi_dn10 = assign6020_e3951_d_n10;
        locals.var_cnstc_foxi_dn11 = assign6020_e3951_d_n11;
        locals.var_cnstc_foxi_dn12 = assign6020_e3951_d_n12;
        locals.var_cnstc_foxi_dn17 = assign6020_e3951_d_n17;

        let assign6030_e3958: f64 = if ((p.p43 == 1.0) || (locals.var_subversion < 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard74 = assign6030_e3958;

        let (assign6040_e3966, assign6040_e3966_d_n0, assign6040_e3966_d_n2, assign6040_e3966_d_n6, assign6040_e3966_d_n7, assign6040_e3966_d_n10, assign6040_e3966_d_n11, assign6040_e3966_d_n12, assign6040_e3966_d_n17,) = {
    if (locals.var_guard74 != 0.0) {
        let assign6040_e3962: f64 = (0.5 - locals.var_vbspz);
        let assign6040_e3964: f64 = (assign6040_e3962 - 0.001);
        (assign6040_e3964, (-locals.var_vbspz_dn0), (-locals.var_vbspz_dn2), (-locals.var_vbspz_dn6), (-locals.var_vbspz_dn7), (-locals.var_vbspz_dn10), (-locals.var_vbspz_dn11), (-locals.var_vbspz_dn12), (-locals.var_vbspz_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign6040_e3966;
        locals.var_tmf1_dn0 = assign6040_e3966_d_n0;
        locals.var_tmf1_dn2 = assign6040_e3966_d_n2;
        locals.var_tmf1_dn6 = assign6040_e3966_d_n6;
        locals.var_tmf1_dn7 = assign6040_e3966_d_n7;
        locals.var_tmf1_dn10 = assign6040_e3966_d_n10;
        locals.var_tmf1_dn11 = assign6040_e3966_d_n11;
        locals.var_tmf1_dn12 = assign6040_e3966_d_n12;
        locals.var_tmf1_dn17 = assign6040_e3966_d_n17;

        let (assign6050_e3974, assign6050_e3974_d_n0, assign6050_e3974_d_n2, assign6050_e3974_d_n6, assign6050_e3974_d_n7, assign6050_e3974_d_n10, assign6050_e3974_d_n11, assign6050_e3974_d_n12, assign6050_e3974_d_n17,) = {
    if (locals.var_guard74 != 0.0) {
        let assign6050_e3970: f64 = (4.0 * 0.5);
        let assign6050_e3972: f64 = (assign6050_e3970 * 0.001);
        (assign6050_e3972, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6050_e3974;
        locals.var_tmf2_dn0 = assign6050_e3974_d_n0;
        locals.var_tmf2_dn2 = assign6050_e3974_d_n2;
        locals.var_tmf2_dn6 = assign6050_e3974_d_n6;
        locals.var_tmf2_dn7 = assign6050_e3974_d_n7;
        locals.var_tmf2_dn10 = assign6050_e3974_d_n10;
        locals.var_tmf2_dn11 = assign6050_e3974_d_n11;
        locals.var_tmf2_dn12 = assign6050_e3974_d_n12;
        locals.var_tmf2_dn17 = assign6050_e3974_d_n17;

        let (assign6060_e3984, assign6060_e3984_d_n0, assign6060_e3984_d_n2, assign6060_e3984_d_n6, assign6060_e3984_d_n7, assign6060_e3984_d_n10, assign6060_e3984_d_n11, assign6060_e3984_d_n12, assign6060_e3984_d_n17,) = {
    if (locals.var_guard74 != 0.0) {
        let (assign6060_e3982, assign6060_e3982_d_n0, assign6060_e3982_d_n2, assign6060_e3982_d_n6, assign6060_e3982_d_n7, assign6060_e3982_d_n10, assign6060_e3982_d_n11, assign6060_e3982_d_n12, assign6060_e3982_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign6060_e3981: f64 = (-locals.var_tmf2);
                (assign6060_e3981, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign6060_e3982, assign6060_e3982_d_n0, assign6060_e3982_d_n2, assign6060_e3982_d_n6, assign6060_e3982_d_n7, assign6060_e3982_d_n10, assign6060_e3982_d_n11, assign6060_e3982_d_n12, assign6060_e3982_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6060_e3984;
        locals.var_tmf2_dn0 = assign6060_e3984_d_n0;
        locals.var_tmf2_dn2 = assign6060_e3984_d_n2;
        locals.var_tmf2_dn6 = assign6060_e3984_d_n6;
        locals.var_tmf2_dn7 = assign6060_e3984_d_n7;
        locals.var_tmf2_dn10 = assign6060_e3984_d_n10;
        locals.var_tmf2_dn11 = assign6060_e3984_d_n11;
        locals.var_tmf2_dn12 = assign6060_e3984_d_n12;
        locals.var_tmf2_dn17 = assign6060_e3984_d_n17;

        let (assign6070_e3993, assign6070_e3993_d_n0, assign6070_e3993_d_n2, assign6070_e3993_d_n6, assign6070_e3993_d_n7, assign6070_e3993_d_n10, assign6070_e3993_d_n11, assign6070_e3993_d_n12, assign6070_e3993_d_n17,) = {
    if (locals.var_guard74 != 0.0) {
        let assign6070_e3988: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign6070_e3990: f64 = (assign6070_e3988 + locals.var_tmf2);
        let assign6070_e3991: f64 = (assign6070_e3990).sqrt();
        (assign6070_e3991, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign6070_e3991)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign6070_e3991)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign6070_e3991)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign6070_e3991)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign6070_e3991)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign6070_e3991)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign6070_e3991)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign6070_e3991)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6070_e3993;
        locals.var_tmf2_dn0 = assign6070_e3993_d_n0;
        locals.var_tmf2_dn2 = assign6070_e3993_d_n2;
        locals.var_tmf2_dn6 = assign6070_e3993_d_n6;
        locals.var_tmf2_dn7 = assign6070_e3993_d_n7;
        locals.var_tmf2_dn10 = assign6070_e3993_d_n10;
        locals.var_tmf2_dn11 = assign6070_e3993_d_n11;
        locals.var_tmf2_dn12 = assign6070_e3993_d_n12;
        locals.var_tmf2_dn17 = assign6070_e3993_d_n17;

        let (assign6080_e4003, assign6080_e4003_d_n0, assign6080_e4003_d_n2, assign6080_e4003_d_n6, assign6080_e4003_d_n7, assign6080_e4003_d_n10, assign6080_e4003_d_n11, assign6080_e4003_d_n12, assign6080_e4003_d_n17,) = {
    if (locals.var_guard74 != 0.0) {
        let assign6080_e3999: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign6080_e4000: f64 = (0.5 * assign6080_e3999);
        let assign6080_e4001: f64 = (0.5 - assign6080_e4000);
        (assign6080_e4001, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (-(0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_vbsz2, locals.var_vbsz2_dn0, locals.var_vbsz2_dn2, locals.var_vbsz2_dn6, locals.var_vbsz2_dn7, locals.var_vbsz2_dn10, locals.var_vbsz2_dn11, locals.var_vbsz2_dn12, locals.var_vbsz2_dn17,)
    }
};
        locals.var_vbsz2 = assign6080_e4003;
        locals.var_vbsz2_dn0 = assign6080_e4003_d_n0;
        locals.var_vbsz2_dn2 = assign6080_e4003_d_n2;
        locals.var_vbsz2_dn6 = assign6080_e4003_d_n6;
        locals.var_vbsz2_dn7 = assign6080_e4003_d_n7;
        locals.var_vbsz2_dn10 = assign6080_e4003_d_n10;
        locals.var_vbsz2_dn11 = assign6080_e4003_d_n11;
        locals.var_vbsz2_dn12 = assign6080_e4003_d_n12;
        locals.var_vbsz2_dn17 = assign6080_e4003_d_n17;

        let (assign6090_e4020, assign6090_e4020_d_n0, assign6090_e4020_d_n2, assign6090_e4020_d_n6, assign6090_e4020_d_n7, assign6090_e4020_d_n10, assign6090_e4020_d_n11, assign6090_e4020_d_n12, assign6090_e4020_d_n17,) = {
    if (locals.var_guard74 != 0.0) {
        let assign6090_e4006: f64 = (-p.p237);
        let assign6090_e4008: f64 = (assign6090_e4006 * p.p237);
        let assign6090_e4010: f64 = (assign6090_e4008 * locals.var_q_nsub);
        let assign6090_e4013: f64 = (2.0 * 1.034943e-10);
        let assign6090_e4014: f64 = (assign6090_e4010 / assign6090_e4013);
        let assign6090_e4016: f64 = (assign6090_e4014 + locals.var_pb2);
        let assign6090_e4018: f64 = (assign6090_e4016 - locals.var_beta_inv);
        (assign6090_e4018, (((assign6090_e4008 * locals.var_q_nsub_dn0) / assign6090_e4013) + locals.var_pb2_dn0), (((assign6090_e4008 * locals.var_q_nsub_dn2) / assign6090_e4013) + locals.var_pb2_dn2), (((assign6090_e4008 * locals.var_q_nsub_dn6) / assign6090_e4013) + locals.var_pb2_dn6), (((assign6090_e4008 * locals.var_q_nsub_dn7) / assign6090_e4013) + locals.var_pb2_dn7), ((((assign6090_e4008 * locals.var_q_nsub_dn10) / assign6090_e4013) + locals.var_pb2_dn10) - locals.var_beta_inv_dn10), (((assign6090_e4008 * locals.var_q_nsub_dn11) / assign6090_e4013) + locals.var_pb2_dn11), (((assign6090_e4008 * locals.var_q_nsub_dn12) / assign6090_e4013) + locals.var_pb2_dn12), (((assign6090_e4008 * locals.var_q_nsub_dn17) / assign6090_e4013) + locals.var_pb2_dn17),)
    } else {
        (locals.var_vbslim, locals.var_vbslim_dn0, locals.var_vbslim_dn2, locals.var_vbslim_dn6, locals.var_vbslim_dn7, locals.var_vbslim_dn10, locals.var_vbslim_dn11, locals.var_vbslim_dn12, locals.var_vbslim_dn17,)
    }
};
        locals.var_vbslim = assign6090_e4020;
        locals.var_vbslim_dn0 = assign6090_e4020_d_n0;
        locals.var_vbslim_dn2 = assign6090_e4020_d_n2;
        locals.var_vbslim_dn6 = assign6090_e4020_d_n6;
        locals.var_vbslim_dn7 = assign6090_e4020_d_n7;
        locals.var_vbslim_dn10 = assign6090_e4020_d_n10;
        locals.var_vbslim_dn11 = assign6090_e4020_d_n11;
        locals.var_vbslim_dn12 = assign6090_e4020_d_n12;
        locals.var_vbslim_dn17 = assign6090_e4020_d_n17;

        let (assign6100_e4028, assign6100_e4028_d_n0, assign6100_e4028_d_n2, assign6100_e4028_d_n6, assign6100_e4028_d_n7, assign6100_e4028_d_n10, assign6100_e4028_d_n11, assign6100_e4028_d_n12, assign6100_e4028_d_n17,) = {
    if (locals.var_guard74 != 0.0) {
        let assign6100_e4024: f64 = (locals.var_vbsz2 - locals.var_vbslim);
        let assign6100_e4026: f64 = (assign6100_e4024 - 0.001);
        (assign6100_e4026, (locals.var_vbsz2_dn0 - locals.var_vbslim_dn0), (locals.var_vbsz2_dn2 - locals.var_vbslim_dn2), (locals.var_vbsz2_dn6 - locals.var_vbslim_dn6), (locals.var_vbsz2_dn7 - locals.var_vbslim_dn7), (locals.var_vbsz2_dn10 - locals.var_vbslim_dn10), (locals.var_vbsz2_dn11 - locals.var_vbslim_dn11), (locals.var_vbsz2_dn12 - locals.var_vbslim_dn12), (locals.var_vbsz2_dn17 - locals.var_vbslim_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign6100_e4028;
        locals.var_tmf1_dn0 = assign6100_e4028_d_n0;
        locals.var_tmf1_dn2 = assign6100_e4028_d_n2;
        locals.var_tmf1_dn6 = assign6100_e4028_d_n6;
        locals.var_tmf1_dn7 = assign6100_e4028_d_n7;
        locals.var_tmf1_dn10 = assign6100_e4028_d_n10;
        locals.var_tmf1_dn11 = assign6100_e4028_d_n11;
        locals.var_tmf1_dn12 = assign6100_e4028_d_n12;
        locals.var_tmf1_dn17 = assign6100_e4028_d_n17;

        let (assign6110_e4036, assign6110_e4036_d_n0, assign6110_e4036_d_n2, assign6110_e4036_d_n6, assign6110_e4036_d_n7, assign6110_e4036_d_n10, assign6110_e4036_d_n11, assign6110_e4036_d_n12, assign6110_e4036_d_n17,) = {
    if (locals.var_guard74 != 0.0) {
        let assign6110_e4032: f64 = (4.0 * locals.var_vbslim);
        let assign6110_e4034: f64 = (assign6110_e4032 * 0.001);
        (assign6110_e4034, ((4.0 * locals.var_vbslim_dn0) * 0.001), ((4.0 * locals.var_vbslim_dn2) * 0.001), ((4.0 * locals.var_vbslim_dn6) * 0.001), ((4.0 * locals.var_vbslim_dn7) * 0.001), ((4.0 * locals.var_vbslim_dn10) * 0.001), ((4.0 * locals.var_vbslim_dn11) * 0.001), ((4.0 * locals.var_vbslim_dn12) * 0.001), ((4.0 * locals.var_vbslim_dn17) * 0.001),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6110_e4036;
        locals.var_tmf2_dn0 = assign6110_e4036_d_n0;
        locals.var_tmf2_dn2 = assign6110_e4036_d_n2;
        locals.var_tmf2_dn6 = assign6110_e4036_d_n6;
        locals.var_tmf2_dn7 = assign6110_e4036_d_n7;
        locals.var_tmf2_dn10 = assign6110_e4036_d_n10;
        locals.var_tmf2_dn11 = assign6110_e4036_d_n11;
        locals.var_tmf2_dn12 = assign6110_e4036_d_n12;
        locals.var_tmf2_dn17 = assign6110_e4036_d_n17;

        let (assign6120_e4046, assign6120_e4046_d_n0, assign6120_e4046_d_n2, assign6120_e4046_d_n6, assign6120_e4046_d_n7, assign6120_e4046_d_n10, assign6120_e4046_d_n11, assign6120_e4046_d_n12, assign6120_e4046_d_n17,) = {
    if (locals.var_guard74 != 0.0) {
        let (assign6120_e4044, assign6120_e4044_d_n0, assign6120_e4044_d_n2, assign6120_e4044_d_n6, assign6120_e4044_d_n7, assign6120_e4044_d_n10, assign6120_e4044_d_n11, assign6120_e4044_d_n12, assign6120_e4044_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign6120_e4043: f64 = (-locals.var_tmf2);
                (assign6120_e4043, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign6120_e4044, assign6120_e4044_d_n0, assign6120_e4044_d_n2, assign6120_e4044_d_n6, assign6120_e4044_d_n7, assign6120_e4044_d_n10, assign6120_e4044_d_n11, assign6120_e4044_d_n12, assign6120_e4044_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6120_e4046;
        locals.var_tmf2_dn0 = assign6120_e4046_d_n0;
        locals.var_tmf2_dn2 = assign6120_e4046_d_n2;
        locals.var_tmf2_dn6 = assign6120_e4046_d_n6;
        locals.var_tmf2_dn7 = assign6120_e4046_d_n7;
        locals.var_tmf2_dn10 = assign6120_e4046_d_n10;
        locals.var_tmf2_dn11 = assign6120_e4046_d_n11;
        locals.var_tmf2_dn12 = assign6120_e4046_d_n12;
        locals.var_tmf2_dn17 = assign6120_e4046_d_n17;

        let (assign6130_e4055, assign6130_e4055_d_n0, assign6130_e4055_d_n2, assign6130_e4055_d_n6, assign6130_e4055_d_n7, assign6130_e4055_d_n10, assign6130_e4055_d_n11, assign6130_e4055_d_n12, assign6130_e4055_d_n17,) = {
    if (locals.var_guard74 != 0.0) {
        let assign6130_e4050: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign6130_e4052: f64 = (assign6130_e4050 + locals.var_tmf2);
        let assign6130_e4053: f64 = (assign6130_e4052).sqrt();
        (assign6130_e4053, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign6130_e4053)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign6130_e4053)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign6130_e4053)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign6130_e4053)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign6130_e4053)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign6130_e4053)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign6130_e4053)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign6130_e4053)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6130_e4055;
        locals.var_tmf2_dn0 = assign6130_e4055_d_n0;
        locals.var_tmf2_dn2 = assign6130_e4055_d_n2;
        locals.var_tmf2_dn6 = assign6130_e4055_d_n6;
        locals.var_tmf2_dn7 = assign6130_e4055_d_n7;
        locals.var_tmf2_dn10 = assign6130_e4055_d_n10;
        locals.var_tmf2_dn11 = assign6130_e4055_d_n11;
        locals.var_tmf2_dn12 = assign6130_e4055_d_n12;
        locals.var_tmf2_dn17 = assign6130_e4055_d_n17;

        let (assign6140_e4065, assign6140_e4065_d_n0, assign6140_e4065_d_n2, assign6140_e4065_d_n6, assign6140_e4065_d_n7, assign6140_e4065_d_n10, assign6140_e4065_d_n11, assign6140_e4065_d_n12, assign6140_e4065_d_n17,) = {
    if (locals.var_guard74 != 0.0) {
        let assign6140_e4061: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign6140_e4062: f64 = (0.5 * assign6140_e4061);
        let assign6140_e4063: f64 = (locals.var_vbslim + assign6140_e4062);
        (assign6140_e4063, (locals.var_vbslim_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_vbslim_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_vbslim_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_vbslim_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_vbslim_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_vbslim_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_vbslim_dn12 + (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_vbslim_dn17 + (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_vbsz2, locals.var_vbsz2_dn0, locals.var_vbsz2_dn2, locals.var_vbsz2_dn6, locals.var_vbsz2_dn7, locals.var_vbsz2_dn10, locals.var_vbsz2_dn11, locals.var_vbsz2_dn12, locals.var_vbsz2_dn17,)
    }
};
        locals.var_vbsz2 = assign6140_e4065;
        locals.var_vbsz2_dn0 = assign6140_e4065_d_n0;
        locals.var_vbsz2_dn2 = assign6140_e4065_d_n2;
        locals.var_vbsz2_dn6 = assign6140_e4065_d_n6;
        locals.var_vbsz2_dn7 = assign6140_e4065_d_n7;
        locals.var_vbsz2_dn10 = assign6140_e4065_d_n10;
        locals.var_vbsz2_dn11 = assign6140_e4065_d_n11;
        locals.var_vbsz2_dn12 = assign6140_e4065_d_n12;
        locals.var_vbsz2_dn17 = assign6140_e4065_d_n17;

        let assign6150_e4068: f64 = if locals.var_subversion > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard75 = assign6150_e4068;

        let (assign6160_e4078, assign6160_e4078_d_n0, assign6160_e4078_d_n2, assign6160_e4078_d_n6, assign6160_e4078_d_n7, assign6160_e4078_d_n10, assign6160_e4078_d_n11, assign6160_e4078_d_n12, assign6160_e4078_d_n17,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard75 != 0.0)) {
        let assign6160_e4074: f64 = (locals.var_pb20 - locals.var_vbsz2);
        let assign6160_e4076: f64 = (assign6160_e4074 - 0.001);
        (assign6160_e4076, (locals.var_pb20_dn0 - locals.var_vbsz2_dn0), (locals.var_pb20_dn2 - locals.var_vbsz2_dn2), (locals.var_pb20_dn6 - locals.var_vbsz2_dn6), (locals.var_pb20_dn7 - locals.var_vbsz2_dn7), (locals.var_pb20_dn10 - locals.var_vbsz2_dn10), (locals.var_pb20_dn11 - locals.var_vbsz2_dn11), (locals.var_pb20_dn12 - locals.var_vbsz2_dn12), (locals.var_pb20_dn17 - locals.var_vbsz2_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign6160_e4078;
        locals.var_tmf1_dn0 = assign6160_e4078_d_n0;
        locals.var_tmf1_dn2 = assign6160_e4078_d_n2;
        locals.var_tmf1_dn6 = assign6160_e4078_d_n6;
        locals.var_tmf1_dn7 = assign6160_e4078_d_n7;
        locals.var_tmf1_dn10 = assign6160_e4078_d_n10;
        locals.var_tmf1_dn11 = assign6160_e4078_d_n11;
        locals.var_tmf1_dn12 = assign6160_e4078_d_n12;
        locals.var_tmf1_dn17 = assign6160_e4078_d_n17;

    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6170_e4088, assign6170_e4088_d_n0, assign6170_e4088_d_n2, assign6170_e4088_d_n6, assign6170_e4088_d_n7, assign6170_e4088_d_n10, assign6170_e4088_d_n11, assign6170_e4088_d_n12, assign6170_e4088_d_n17,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard75 != 0.0)) {
        let assign6170_e4084: f64 = (4.0 * locals.var_pb20);
        let assign6170_e4086: f64 = (assign6170_e4084 * 0.001);
        (assign6170_e4086, ((4.0 * locals.var_pb20_dn0) * 0.001), ((4.0 * locals.var_pb20_dn2) * 0.001), ((4.0 * locals.var_pb20_dn6) * 0.001), ((4.0 * locals.var_pb20_dn7) * 0.001), ((4.0 * locals.var_pb20_dn10) * 0.001), ((4.0 * locals.var_pb20_dn11) * 0.001), ((4.0 * locals.var_pb20_dn12) * 0.001), ((4.0 * locals.var_pb20_dn17) * 0.001),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6170_e4088;
        locals.var_tmf2_dn0 = assign6170_e4088_d_n0;
        locals.var_tmf2_dn2 = assign6170_e4088_d_n2;
        locals.var_tmf2_dn6 = assign6170_e4088_d_n6;
        locals.var_tmf2_dn7 = assign6170_e4088_d_n7;
        locals.var_tmf2_dn10 = assign6170_e4088_d_n10;
        locals.var_tmf2_dn11 = assign6170_e4088_d_n11;
        locals.var_tmf2_dn12 = assign6170_e4088_d_n12;
        locals.var_tmf2_dn17 = assign6170_e4088_d_n17;

        let (assign6180_e4100, assign6180_e4100_d_n0, assign6180_e4100_d_n2, assign6180_e4100_d_n6, assign6180_e4100_d_n7, assign6180_e4100_d_n10, assign6180_e4100_d_n11, assign6180_e4100_d_n12, assign6180_e4100_d_n17,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard75 != 0.0)) {
        let (assign6180_e4098, assign6180_e4098_d_n0, assign6180_e4098_d_n2, assign6180_e4098_d_n6, assign6180_e4098_d_n7, assign6180_e4098_d_n10, assign6180_e4098_d_n11, assign6180_e4098_d_n12, assign6180_e4098_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign6180_e4097: f64 = (-locals.var_tmf2);
                (assign6180_e4097, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign6180_e4098, assign6180_e4098_d_n0, assign6180_e4098_d_n2, assign6180_e4098_d_n6, assign6180_e4098_d_n7, assign6180_e4098_d_n10, assign6180_e4098_d_n11, assign6180_e4098_d_n12, assign6180_e4098_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6180_e4100;
        locals.var_tmf2_dn0 = assign6180_e4100_d_n0;
        locals.var_tmf2_dn2 = assign6180_e4100_d_n2;
        locals.var_tmf2_dn6 = assign6180_e4100_d_n6;
        locals.var_tmf2_dn7 = assign6180_e4100_d_n7;
        locals.var_tmf2_dn10 = assign6180_e4100_d_n10;
        locals.var_tmf2_dn11 = assign6180_e4100_d_n11;
        locals.var_tmf2_dn12 = assign6180_e4100_d_n12;
        locals.var_tmf2_dn17 = assign6180_e4100_d_n17;

        let (assign6190_e4111, assign6190_e4111_d_n0, assign6190_e4111_d_n2, assign6190_e4111_d_n6, assign6190_e4111_d_n7, assign6190_e4111_d_n10, assign6190_e4111_d_n11, assign6190_e4111_d_n12, assign6190_e4111_d_n17,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard75 != 0.0)) {
        let assign6190_e4106: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign6190_e4108: f64 = (assign6190_e4106 + locals.var_tmf2);
        let assign6190_e4109: f64 = (assign6190_e4108).sqrt();
        (assign6190_e4109, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign6190_e4109)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign6190_e4109)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign6190_e4109)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign6190_e4109)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign6190_e4109)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign6190_e4109)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign6190_e4109)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign6190_e4109)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6190_e4111;
        locals.var_tmf2_dn0 = assign6190_e4111_d_n0;
        locals.var_tmf2_dn2 = assign6190_e4111_d_n2;
        locals.var_tmf2_dn6 = assign6190_e4111_d_n6;
        locals.var_tmf2_dn7 = assign6190_e4111_d_n7;
        locals.var_tmf2_dn10 = assign6190_e4111_d_n10;
        locals.var_tmf2_dn11 = assign6190_e4111_d_n11;
        locals.var_tmf2_dn12 = assign6190_e4111_d_n12;
        locals.var_tmf2_dn17 = assign6190_e4111_d_n17;

        let (assign6200_e4123, assign6200_e4123_d_n0, assign6200_e4123_d_n2, assign6200_e4123_d_n6, assign6200_e4123_d_n7, assign6200_e4123_d_n10, assign6200_e4123_d_n11, assign6200_e4123_d_n12, assign6200_e4123_d_n17,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard75 != 0.0)) {
        let assign6200_e4119: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign6200_e4120: f64 = (0.5 * assign6200_e4119);
        let assign6200_e4121: f64 = (locals.var_pb20 - assign6200_e4120);
        (assign6200_e4121, (locals.var_pb20_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_pb20_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_pb20_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_pb20_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_pb20_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_pb20_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_pb20_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_pb20_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_vbsz2, locals.var_vbsz2_dn0, locals.var_vbsz2_dn2, locals.var_vbsz2_dn6, locals.var_vbsz2_dn7, locals.var_vbsz2_dn10, locals.var_vbsz2_dn11, locals.var_vbsz2_dn12, locals.var_vbsz2_dn17,)
    }
};
        locals.var_vbsz2 = assign6200_e4123;
        locals.var_vbsz2_dn0 = assign6200_e4123_d_n0;
        locals.var_vbsz2_dn2 = assign6200_e4123_d_n2;
        locals.var_vbsz2_dn6 = assign6200_e4123_d_n6;
        locals.var_vbsz2_dn7 = assign6200_e4123_d_n7;
        locals.var_vbsz2_dn10 = assign6200_e4123_d_n10;
        locals.var_vbsz2_dn11 = assign6200_e4123_d_n11;
        locals.var_vbsz2_dn12 = assign6200_e4123_d_n12;
        locals.var_vbsz2_dn17 = assign6200_e4123_d_n17;

        let (assign6210_e4128, assign6210_e4128_d_n0, assign6210_e4128_d_n2, assign6210_e4128_d_n6, assign6210_e4128_d_n7, assign6210_e4128_d_n10, assign6210_e4128_d_n11, assign6210_e4128_d_n12, assign6210_e4128_d_n17,) = {
    if (locals.var_guard74 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbsz2, locals.var_vbsz2_dn0, locals.var_vbsz2_dn2, locals.var_vbsz2_dn6, locals.var_vbsz2_dn7, locals.var_vbsz2_dn10, locals.var_vbsz2_dn11, locals.var_vbsz2_dn12, locals.var_vbsz2_dn17,)
    }
};
        locals.var_vbsz2 = assign6210_e4128;
        locals.var_vbsz2_dn0 = assign6210_e4128_d_n0;
        locals.var_vbsz2_dn2 = assign6210_e4128_d_n2;
        locals.var_vbsz2_dn6 = assign6210_e4128_d_n6;
        locals.var_vbsz2_dn7 = assign6210_e4128_d_n7;
        locals.var_vbsz2_dn10 = assign6210_e4128_d_n10;
        locals.var_vbsz2_dn11 = assign6210_e4128_d_n11;
        locals.var_vbsz2_dn12 = assign6210_e4128_d_n12;
        locals.var_vbsz2_dn17 = assign6210_e4128_d_n17;

        let assign6220_e4131: f64 = if locals.var_subversion < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard76 = assign6220_e4131;

        let (assign6230_e4135, assign6230_e4135_d_n0, assign6230_e4135_d_n2, assign6230_e4135_d_n6, assign6230_e4135_d_n7, assign6230_e4135_d_n10, assign6230_e4135_d_n11, assign6230_e4135_d_n12, assign6230_e4135_d_n17,) = {
    if (locals.var_guard76 != 0.0) {
        (p.p237, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wd0, locals.var_wd0_dn0, locals.var_wd0_dn2, locals.var_wd0_dn6, locals.var_wd0_dn7, locals.var_wd0_dn10, locals.var_wd0_dn11, locals.var_wd0_dn12, locals.var_wd0_dn17,)
    }
};
        locals.var_wd0 = assign6230_e4135;
        locals.var_wd0_dn0 = assign6230_e4135_d_n0;
        locals.var_wd0_dn2 = assign6230_e4135_d_n2;
        locals.var_wd0_dn6 = assign6230_e4135_d_n6;
        locals.var_wd0_dn7 = assign6230_e4135_d_n7;
        locals.var_wd0_dn10 = assign6230_e4135_d_n10;
        locals.var_wd0_dn11 = assign6230_e4135_d_n11;
        locals.var_wd0_dn12 = assign6230_e4135_d_n12;
        locals.var_wd0_dn17 = assign6230_e4135_d_n17;

        let (assign6240_e4144, assign6240_e4144_d_n0, assign6240_e4144_d_n2, assign6240_e4144_d_n6, assign6240_e4144_d_n7, assign6240_e4144_d_n10, assign6240_e4144_d_n11, assign6240_e4144_d_n12, assign6240_e4144_d_n17,) = {
    if (locals.var_guard76 == 0.0) {
        let assign6240_e4140: f64 = (2.0 * 1.034943e-10);
        let assign6240_e4142: f64 = (assign6240_e4140 / locals.var_q_nsub);
        (assign6240_e4142, (-((assign6240_e4140 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))), (-((assign6240_e4140 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))), (-((assign6240_e4140 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))), (-((assign6240_e4140 * locals.var_q_nsub_dn7) / (locals.var_q_nsub * locals.var_q_nsub))), (-((assign6240_e4140 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))), (-((assign6240_e4140 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))), (-((assign6240_e4140 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))), (-((assign6240_e4140 * locals.var_q_nsub_dn17) / (locals.var_q_nsub * locals.var_q_nsub))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign6240_e4144;
        locals.var_t1_dn0 = assign6240_e4144_d_n0;
        locals.var_t1_dn2 = assign6240_e4144_d_n2;
        locals.var_t1_dn6 = assign6240_e4144_d_n6;
        locals.var_t1_dn7 = assign6240_e4144_d_n7;
        locals.var_t1_dn10 = assign6240_e4144_d_n10;
        locals.var_t1_dn11 = assign6240_e4144_d_n11;
        locals.var_t1_dn12 = assign6240_e4144_d_n12;
        locals.var_t1_dn17 = assign6240_e4144_d_n17;

        let (assign6250_e4154, assign6250_e4154_d_n0, assign6250_e4154_d_n2, assign6250_e4154_d_n6, assign6250_e4154_d_n7, assign6250_e4154_d_n10, assign6250_e4154_d_n11, assign6250_e4154_d_n12, assign6250_e4154_d_n17,) = {
    if (locals.var_guard76 == 0.0) {
        let assign6250_e4150: f64 = (locals.var_pb20 - locals.var_vbsz2);
        let assign6250_e4151: f64 = (locals.var_t1 * assign6250_e4150);
        let assign6250_e4152: f64 = (assign6250_e4151).sqrt();
        (assign6250_e4152, (((locals.var_t1_dn0 * assign6250_e4150) + (locals.var_t1 * (locals.var_pb20_dn0 - locals.var_vbsz2_dn0))) / (2.0 * assign6250_e4152)), (((locals.var_t1_dn2 * assign6250_e4150) + (locals.var_t1 * (locals.var_pb20_dn2 - locals.var_vbsz2_dn2))) / (2.0 * assign6250_e4152)), (((locals.var_t1_dn6 * assign6250_e4150) + (locals.var_t1 * (locals.var_pb20_dn6 - locals.var_vbsz2_dn6))) / (2.0 * assign6250_e4152)), (((locals.var_t1_dn7 * assign6250_e4150) + (locals.var_t1 * (locals.var_pb20_dn7 - locals.var_vbsz2_dn7))) / (2.0 * assign6250_e4152)), (((locals.var_t1_dn10 * assign6250_e4150) + (locals.var_t1 * (locals.var_pb20_dn10 - locals.var_vbsz2_dn10))) / (2.0 * assign6250_e4152)), (((locals.var_t1_dn11 * assign6250_e4150) + (locals.var_t1 * (locals.var_pb20_dn11 - locals.var_vbsz2_dn11))) / (2.0 * assign6250_e4152)), (((locals.var_t1_dn12 * assign6250_e4150) + (locals.var_t1 * (locals.var_pb20_dn12 - locals.var_vbsz2_dn12))) / (2.0 * assign6250_e4152)), (((locals.var_t1_dn17 * assign6250_e4150) + (locals.var_t1 * (locals.var_pb20_dn17 - locals.var_vbsz2_dn17))) / (2.0 * assign6250_e4152)),)
    } else {
        (locals.var_wd0, locals.var_wd0_dn0, locals.var_wd0_dn2, locals.var_wd0_dn6, locals.var_wd0_dn7, locals.var_wd0_dn10, locals.var_wd0_dn11, locals.var_wd0_dn12, locals.var_wd0_dn17,)
    }
};
        locals.var_wd0 = assign6250_e4154;
        locals.var_wd0_dn0 = assign6250_e4154_d_n0;
        locals.var_wd0_dn2 = assign6250_e4154_d_n2;
        locals.var_wd0_dn6 = assign6250_e4154_d_n6;
        locals.var_wd0_dn7 = assign6250_e4154_d_n7;
        locals.var_wd0_dn10 = assign6250_e4154_d_n10;
        locals.var_wd0_dn11 = assign6250_e4154_d_n11;
        locals.var_wd0_dn12 = assign6250_e4154_d_n12;
        locals.var_wd0_dn17 = assign6250_e4154_d_n17;

        let (assign6260_e4168, assign6260_e4168_d_n0, assign6260_e4168_d_n2, assign6260_e4168_d_n6, assign6260_e4168_d_n7, assign6260_e4168_d_n10, assign6260_e4168_d_n11, assign6260_e4168_d_n12, assign6260_e4168_d_n17,) = {
    if (locals.var_subversion < 3.0) {
        let assign6260_e4160: f64 = (locals.var_qnsub_esi2 * locals.var_pb20);
        let assign6260_e4161: f64 = (assign6260_e4160).sqrt();
        (assign6260_e4161, (((locals.var_qnsub_esi2_dn0 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn0)) / (2.0 * assign6260_e4161)), (((locals.var_qnsub_esi2_dn2 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn2)) / (2.0 * assign6260_e4161)), (((locals.var_qnsub_esi2_dn6 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn6)) / (2.0 * assign6260_e4161)), (((locals.var_qnsub_esi2_dn7 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn7)) / (2.0 * assign6260_e4161)), (((locals.var_qnsub_esi2_dn10 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn10)) / (2.0 * assign6260_e4161)), (((locals.var_qnsub_esi2_dn11 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn11)) / (2.0 * assign6260_e4161)), (((locals.var_qnsub_esi2_dn12 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn12)) / (2.0 * assign6260_e4161)), (((locals.var_qnsub_esi2_dn17 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn17)) / (2.0 * assign6260_e4161)),)
    } else {
        let assign6260_e4165: f64 = (locals.var_pb20 - locals.var_vbsz2);
        let assign6260_e4166: f64 = (locals.var_qnsub_esi2 * assign6260_e4165);
        let assign6260_e4167: f64 = (assign6260_e4166).sqrt();
        (assign6260_e4167, (((locals.var_qnsub_esi2_dn0 * assign6260_e4165) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn0 - locals.var_vbsz2_dn0))) / (2.0 * assign6260_e4167)), (((locals.var_qnsub_esi2_dn2 * assign6260_e4165) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn2 - locals.var_vbsz2_dn2))) / (2.0 * assign6260_e4167)), (((locals.var_qnsub_esi2_dn6 * assign6260_e4165) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn6 - locals.var_vbsz2_dn6))) / (2.0 * assign6260_e4167)), (((locals.var_qnsub_esi2_dn7 * assign6260_e4165) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn7 - locals.var_vbsz2_dn7))) / (2.0 * assign6260_e4167)), (((locals.var_qnsub_esi2_dn10 * assign6260_e4165) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn10 - locals.var_vbsz2_dn10))) / (2.0 * assign6260_e4167)), (((locals.var_qnsub_esi2_dn11 * assign6260_e4165) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn11 - locals.var_vbsz2_dn11))) / (2.0 * assign6260_e4167)), (((locals.var_qnsub_esi2_dn12 * assign6260_e4165) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn12 - locals.var_vbsz2_dn12))) / (2.0 * assign6260_e4167)), (((locals.var_qnsub_esi2_dn17 * assign6260_e4165) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn17 - locals.var_vbsz2_dn17))) / (2.0 * assign6260_e4167)),)
    }
};
        locals.var_qb0 = assign6260_e4168;
        locals.var_qb0_dn0 = assign6260_e4168_d_n0;
        locals.var_qb0_dn2 = assign6260_e4168_d_n2;
        locals.var_qb0_dn6 = assign6260_e4168_d_n6;
        locals.var_qb0_dn7 = assign6260_e4168_d_n7;
        locals.var_qb0_dn10 = assign6260_e4168_d_n10;
        locals.var_qb0_dn11 = assign6260_e4168_d_n11;
        locals.var_qb0_dn12 = assign6260_e4168_d_n12;
        locals.var_qb0_dn17 = assign6260_e4168_d_n17;

        let assign6270_e4171: f64 = (locals.var_pb20 + locals.var_vfb);
        let assign6270_e4174: f64 = (locals.var_qb0 * locals.var_c_fox_inv);
        let assign6270_e4175: f64 = (assign6270_e4171 + assign6270_e4174);
        let assign6270_e4177: f64 = (assign6270_e4175 + locals.var_ptovr);
        locals.var_vthp = assign6270_e4177;
        locals.var_vthp_dn0 = ((locals.var_pb20_dn0 + ((locals.var_qb0_dn0 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn0))) + locals.var_ptovr_dn0);
        locals.var_vthp_dn2 = ((locals.var_pb20_dn2 + ((locals.var_qb0_dn2 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn2))) + locals.var_ptovr_dn2);
        locals.var_vthp_dn6 = ((locals.var_pb20_dn6 + ((locals.var_qb0_dn6 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn6))) + locals.var_ptovr_dn6);
        locals.var_vthp_dn7 = ((locals.var_pb20_dn7 + ((locals.var_qb0_dn7 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn7))) + locals.var_ptovr_dn7);
        locals.var_vthp_dn10 = ((locals.var_pb20_dn10 + ((locals.var_qb0_dn10 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn10))) + locals.var_ptovr_dn10);
        locals.var_vthp_dn11 = ((locals.var_pb20_dn11 + ((locals.var_qb0_dn11 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn11))) + locals.var_ptovr_dn11);
        locals.var_vthp_dn12 = ((locals.var_pb20_dn12 + ((locals.var_qb0_dn12 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn12))) + locals.var_ptovr_dn12);
        locals.var_vthp_dn17 = ((locals.var_pb20_dn17 + ((locals.var_qb0_dn17 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn17))) + locals.var_ptovr_dn17);

        locals.var_pb20b = locals.var_pb20;
        locals.var_pb20b_dn0 = locals.var_pb20_dn0;
        locals.var_pb20b_dn2 = locals.var_pb20_dn2;
        locals.var_pb20b_dn6 = locals.var_pb20_dn6;
        locals.var_pb20b_dn7 = locals.var_pb20_dn7;
        locals.var_pb20b_dn10 = locals.var_pb20_dn10;
        locals.var_pb20b_dn11 = locals.var_pb20_dn11;
        locals.var_pb20b_dn12 = locals.var_pb20_dn12;
        locals.var_pb20b_dn17 = locals.var_pb20_dn17;

        locals.var_t0__blk78 = 0.95;

        let assign6300_e4182: f64 = (locals.var_t0__blk78 * locals.var_pb20b);
        let assign6300_e4184: f64 = (assign6300_e4182 - locals.var_vbsz2);
        let assign6300_e4186: f64 = (assign6300_e4184 - 0.001);
        locals.var_t1__blk77 = assign6300_e4186;
        locals.var_t1__blk77_dn0 = ((locals.var_t0__blk78 * locals.var_pb20b_dn0) - locals.var_vbsz2_dn0);
        locals.var_t1__blk77_dn2 = ((locals.var_t0__blk78 * locals.var_pb20b_dn2) - locals.var_vbsz2_dn2);
        locals.var_t1__blk77_dn6 = ((locals.var_t0__blk78 * locals.var_pb20b_dn6) - locals.var_vbsz2_dn6);
        locals.var_t1__blk77_dn7 = ((locals.var_t0__blk78 * locals.var_pb20b_dn7) - locals.var_vbsz2_dn7);
        locals.var_t1__blk77_dn10 = ((locals.var_t0__blk78 * locals.var_pb20b_dn10) - locals.var_vbsz2_dn10);
        locals.var_t1__blk77_dn11 = ((locals.var_t0__blk78 * locals.var_pb20b_dn11) - locals.var_vbsz2_dn11);
        locals.var_t1__blk77_dn12 = ((locals.var_t0__blk78 * locals.var_pb20b_dn12) - locals.var_vbsz2_dn12);
        locals.var_t1__blk77_dn17 = ((locals.var_t0__blk78 * locals.var_pb20b_dn17) - locals.var_vbsz2_dn17);

        let assign6310_e4189: f64 = (locals.var_t1__blk77 * locals.var_t1__blk77);
        let assign6310_e4192: f64 = (4.0 * locals.var_t0__blk78);
        let assign6310_e4194: f64 = (assign6310_e4192 * locals.var_pb20b);
        let assign6310_e4196: f64 = (assign6310_e4194 * 0.001);
        let assign6310_e4197: f64 = (assign6310_e4189 + assign6310_e4196);
        let assign6310_e4198: f64 = (assign6310_e4197).sqrt();
        locals.var_t2__blk79 = assign6310_e4198;
        locals.var_t2__blk79_dn0 = ((((locals.var_t1__blk77_dn0 * locals.var_t1__blk77) + (locals.var_t1__blk77 * locals.var_t1__blk77_dn0)) + ((assign6310_e4192 * locals.var_pb20b_dn0) * 0.001)) / (2.0 * assign6310_e4198));
        locals.var_t2__blk79_dn2 = ((((locals.var_t1__blk77_dn2 * locals.var_t1__blk77) + (locals.var_t1__blk77 * locals.var_t1__blk77_dn2)) + ((assign6310_e4192 * locals.var_pb20b_dn2) * 0.001)) / (2.0 * assign6310_e4198));
        locals.var_t2__blk79_dn6 = ((((locals.var_t1__blk77_dn6 * locals.var_t1__blk77) + (locals.var_t1__blk77 * locals.var_t1__blk77_dn6)) + ((assign6310_e4192 * locals.var_pb20b_dn6) * 0.001)) / (2.0 * assign6310_e4198));
        locals.var_t2__blk79_dn7 = ((((locals.var_t1__blk77_dn7 * locals.var_t1__blk77) + (locals.var_t1__blk77 * locals.var_t1__blk77_dn7)) + ((assign6310_e4192 * locals.var_pb20b_dn7) * 0.001)) / (2.0 * assign6310_e4198));
        locals.var_t2__blk79_dn10 = ((((locals.var_t1__blk77_dn10 * locals.var_t1__blk77) + (locals.var_t1__blk77 * locals.var_t1__blk77_dn10)) + ((assign6310_e4192 * locals.var_pb20b_dn10) * 0.001)) / (2.0 * assign6310_e4198));
        locals.var_t2__blk79_dn11 = ((((locals.var_t1__blk77_dn11 * locals.var_t1__blk77) + (locals.var_t1__blk77 * locals.var_t1__blk77_dn11)) + ((assign6310_e4192 * locals.var_pb20b_dn11) * 0.001)) / (2.0 * assign6310_e4198));
        locals.var_t2__blk79_dn12 = ((((locals.var_t1__blk77_dn12 * locals.var_t1__blk77) + (locals.var_t1__blk77 * locals.var_t1__blk77_dn12)) + ((assign6310_e4192 * locals.var_pb20b_dn12) * 0.001)) / (2.0 * assign6310_e4198));
        locals.var_t2__blk79_dn17 = ((((locals.var_t1__blk77_dn17 * locals.var_t1__blk77) + (locals.var_t1__blk77 * locals.var_t1__blk77_dn17)) + ((assign6310_e4192 * locals.var_pb20b_dn17) * 0.001)) / (2.0 * assign6310_e4198));

        let assign6320_e4201: f64 = (locals.var_t0__blk78 * locals.var_pb20b);
        let assign6320_e4205: f64 = (locals.var_t1__blk77 + locals.var_t2__blk79);
        let assign6320_e4206: f64 = (0.5 * assign6320_e4205);
        let assign6320_e4207: f64 = (assign6320_e4201 - assign6320_e4206);
        locals.var_t3__blk80 = assign6320_e4207;
        locals.var_t3__blk80_dn0 = ((locals.var_t0__blk78 * locals.var_pb20b_dn0) - (0.5 * (locals.var_t1__blk77_dn0 + locals.var_t2__blk79_dn0)));
        locals.var_t3__blk80_dn2 = ((locals.var_t0__blk78 * locals.var_pb20b_dn2) - (0.5 * (locals.var_t1__blk77_dn2 + locals.var_t2__blk79_dn2)));
        locals.var_t3__blk80_dn6 = ((locals.var_t0__blk78 * locals.var_pb20b_dn6) - (0.5 * (locals.var_t1__blk77_dn6 + locals.var_t2__blk79_dn6)));
        locals.var_t3__blk80_dn7 = ((locals.var_t0__blk78 * locals.var_pb20b_dn7) - (0.5 * (locals.var_t1__blk77_dn7 + locals.var_t2__blk79_dn7)));
        locals.var_t3__blk80_dn10 = ((locals.var_t0__blk78 * locals.var_pb20b_dn10) - (0.5 * (locals.var_t1__blk77_dn10 + locals.var_t2__blk79_dn10)));
        locals.var_t3__blk80_dn11 = ((locals.var_t0__blk78 * locals.var_pb20b_dn11) - (0.5 * (locals.var_t1__blk77_dn11 + locals.var_t2__blk79_dn11)));
        locals.var_t3__blk80_dn12 = ((locals.var_t0__blk78 * locals.var_pb20b_dn12) - (0.5 * (locals.var_t1__blk77_dn12 + locals.var_t2__blk79_dn12)));
        locals.var_t3__blk80_dn17 = ((locals.var_t0__blk78 * locals.var_pb20b_dn17) - (0.5 * (locals.var_t1__blk77_dn17 + locals.var_t2__blk79_dn17)));

        let assign6330_e4210: f64 = (locals.var_pb20b - locals.var_t3__blk80);
        locals.var_pbsum = assign6330_e4210;
        locals.var_pbsum_dn0 = (locals.var_pb20b_dn0 - locals.var_t3__blk80_dn0);
        locals.var_pbsum_dn2 = (locals.var_pb20b_dn2 - locals.var_t3__blk80_dn2);
        locals.var_pbsum_dn6 = (locals.var_pb20b_dn6 - locals.var_t3__blk80_dn6);
        locals.var_pbsum_dn7 = (locals.var_pb20b_dn7 - locals.var_t3__blk80_dn7);
        locals.var_pbsum_dn10 = (locals.var_pb20b_dn10 - locals.var_t3__blk80_dn10);
        locals.var_pbsum_dn11 = (locals.var_pb20b_dn11 - locals.var_t3__blk80_dn11);
        locals.var_pbsum_dn12 = (locals.var_pb20b_dn12 - locals.var_t3__blk80_dn12);
        locals.var_pbsum_dn17 = (locals.var_pb20b_dn17 - locals.var_t3__blk80_dn17);

        let assign6340_e4212: f64 = (locals.var_pbsum).sqrt();
        locals.var_sqrt_pbsum = assign6340_e4212;
        locals.var_sqrt_pbsum_dn0 = (locals.var_pbsum_dn0 / (2.0 * assign6340_e4212));
        locals.var_sqrt_pbsum_dn2 = (locals.var_pbsum_dn2 / (2.0 * assign6340_e4212));
        locals.var_sqrt_pbsum_dn6 = (locals.var_pbsum_dn6 / (2.0 * assign6340_e4212));
        locals.var_sqrt_pbsum_dn7 = (locals.var_pbsum_dn7 / (2.0 * assign6340_e4212));
        locals.var_sqrt_pbsum_dn10 = (locals.var_pbsum_dn10 / (2.0 * assign6340_e4212));
        locals.var_sqrt_pbsum_dn11 = (locals.var_pbsum_dn11 / (2.0 * assign6340_e4212));
        locals.var_sqrt_pbsum_dn12 = (locals.var_pbsum_dn12 / (2.0 * assign6340_e4212));
        locals.var_sqrt_pbsum_dn17 = (locals.var_pbsum_dn17 / (2.0 * assign6340_e4212));

        let assign6350_e4215: f64 = if p.p72 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard88 = assign6350_e4215;

        let (assign6360_e4225, assign6360_e4225_d_n0, assign6360_e4225_d_n2, assign6360_e4225_d_n6, assign6360_e4225_d_n7, assign6360_e4225_d_n10, assign6360_e4225_d_n11, assign6360_e4225_d_n12, assign6360_e4225_d_n17,) = {
    if (locals.var_guard88 != 0.0) {
        let assign6360_e4219: f64 = (2.0 * 1.6021918e-19);
        let assign6360_e4221: f64 = (assign6360_e4219 * locals.var_uc_nsubs);
        let assign6360_e4223: f64 = (assign6360_e4221 * 1.034943e-10);
        (assign6360_e4223, ((assign6360_e4219 * locals.var_uc_nsubs_dn0) * 1.034943e-10), ((assign6360_e4219 * locals.var_uc_nsubs_dn2) * 1.034943e-10), ((assign6360_e4219 * locals.var_uc_nsubs_dn6) * 1.034943e-10), ((assign6360_e4219 * locals.var_uc_nsubs_dn7) * 1.034943e-10), ((assign6360_e4219 * locals.var_uc_nsubs_dn10) * 1.034943e-10), ((assign6360_e4219 * locals.var_uc_nsubs_dn11) * 1.034943e-10), ((assign6360_e4219 * locals.var_uc_nsubs_dn12) * 1.034943e-10), ((assign6360_e4219 * locals.var_uc_nsubs_dn17) * 1.034943e-10),)
    } else {
        (locals.var_t1__blk82, locals.var_t1__blk82_dn0, locals.var_t1__blk82_dn2, locals.var_t1__blk82_dn6, locals.var_t1__blk82_dn7, locals.var_t1__blk82_dn10, locals.var_t1__blk82_dn11, locals.var_t1__blk82_dn12, locals.var_t1__blk82_dn17,)
    }
};
        locals.var_t1__blk82 = assign6360_e4225;
        locals.var_t1__blk82_dn0 = assign6360_e4225_d_n0;
        locals.var_t1__blk82_dn2 = assign6360_e4225_d_n2;
        locals.var_t1__blk82_dn6 = assign6360_e4225_d_n6;
        locals.var_t1__blk82_dn7 = assign6360_e4225_d_n7;
        locals.var_t1__blk82_dn10 = assign6360_e4225_d_n10;
        locals.var_t1__blk82_dn11 = assign6360_e4225_d_n11;
        locals.var_t1__blk82_dn12 = assign6360_e4225_d_n12;
        locals.var_t1__blk82_dn17 = assign6360_e4225_d_n17;

        let (assign6370_e4242, assign6370_e4242_d_n0, assign6370_e4242_d_n2, assign6370_e4242_d_n6, assign6370_e4242_d_n7, assign6370_e4242_d_n10, assign6370_e4242_d_n11, assign6370_e4242_d_n12, assign6370_e4242_d_n17,) = {
    if (locals.var_guard88 != 0.0) {
        let (assign6370_e4240, assign6370_e4240_d_n0, assign6370_e4240_d_n2, assign6370_e4240_d_n6, assign6370_e4240_d_n7, assign6370_e4240_d_n10, assign6370_e4240_d_n11, assign6370_e4240_d_n12, assign6370_e4240_d_n17,) = {
            if (locals.var_subversion < 3.0) {
                let assign6370_e4232: f64 = (locals.var_t1__blk82 * locals.var_pb2c);
                let assign6370_e4233: f64 = (assign6370_e4232).sqrt();
                (assign6370_e4233, (((locals.var_t1__blk82_dn0 * locals.var_pb2c) + (locals.var_t1__blk82 * locals.var_pb2c_dn0)) / (2.0 * assign6370_e4233)), (((locals.var_t1__blk82_dn2 * locals.var_pb2c) + (locals.var_t1__blk82 * locals.var_pb2c_dn2)) / (2.0 * assign6370_e4233)), (((locals.var_t1__blk82_dn6 * locals.var_pb2c) + (locals.var_t1__blk82 * locals.var_pb2c_dn6)) / (2.0 * assign6370_e4233)), (((locals.var_t1__blk82_dn7 * locals.var_pb2c) + (locals.var_t1__blk82 * locals.var_pb2c_dn7)) / (2.0 * assign6370_e4233)), (((locals.var_t1__blk82_dn10 * locals.var_pb2c) + (locals.var_t1__blk82 * locals.var_pb2c_dn10)) / (2.0 * assign6370_e4233)), (((locals.var_t1__blk82_dn11 * locals.var_pb2c) + (locals.var_t1__blk82 * locals.var_pb2c_dn11)) / (2.0 * assign6370_e4233)), (((locals.var_t1__blk82_dn12 * locals.var_pb2c) + (locals.var_t1__blk82 * locals.var_pb2c_dn12)) / (2.0 * assign6370_e4233)), (((locals.var_t1__blk82_dn17 * locals.var_pb2c) + (locals.var_t1__blk82 * locals.var_pb2c_dn17)) / (2.0 * assign6370_e4233)),)
            } else {
                let assign6370_e4237: f64 = (locals.var_pb2c - locals.var_vbsz2);
                let assign6370_e4238: f64 = (locals.var_t1__blk82 * assign6370_e4237);
                let assign6370_e4239: f64 = (assign6370_e4238).sqrt();
                (assign6370_e4239, (((locals.var_t1__blk82_dn0 * assign6370_e4237) + (locals.var_t1__blk82 * (locals.var_pb2c_dn0 - locals.var_vbsz2_dn0))) / (2.0 * assign6370_e4239)), (((locals.var_t1__blk82_dn2 * assign6370_e4237) + (locals.var_t1__blk82 * (locals.var_pb2c_dn2 - locals.var_vbsz2_dn2))) / (2.0 * assign6370_e4239)), (((locals.var_t1__blk82_dn6 * assign6370_e4237) + (locals.var_t1__blk82 * (locals.var_pb2c_dn6 - locals.var_vbsz2_dn6))) / (2.0 * assign6370_e4239)), (((locals.var_t1__blk82_dn7 * assign6370_e4237) + (locals.var_t1__blk82 * (locals.var_pb2c_dn7 - locals.var_vbsz2_dn7))) / (2.0 * assign6370_e4239)), (((locals.var_t1__blk82_dn10 * assign6370_e4237) + (locals.var_t1__blk82 * (locals.var_pb2c_dn10 - locals.var_vbsz2_dn10))) / (2.0 * assign6370_e4239)), (((locals.var_t1__blk82_dn11 * assign6370_e4237) + (locals.var_t1__blk82 * (locals.var_pb2c_dn11 - locals.var_vbsz2_dn11))) / (2.0 * assign6370_e4239)), (((locals.var_t1__blk82_dn12 * assign6370_e4237) + (locals.var_t1__blk82 * (locals.var_pb2c_dn12 - locals.var_vbsz2_dn12))) / (2.0 * assign6370_e4239)), (((locals.var_t1__blk82_dn17 * assign6370_e4237) + (locals.var_t1__blk82 * (locals.var_pb2c_dn17 - locals.var_vbsz2_dn17))) / (2.0 * assign6370_e4239)),)
            }
        };
        (assign6370_e4240, assign6370_e4240_d_n0, assign6370_e4240_d_n2, assign6370_e4240_d_n6, assign6370_e4240_d_n7, assign6370_e4240_d_n10, assign6370_e4240_d_n11, assign6370_e4240_d_n12, assign6370_e4240_d_n17,)
    } else {
        (locals.var_t2__blk83, locals.var_t2__blk83_dn0, locals.var_t2__blk83_dn2, locals.var_t2__blk83_dn6, locals.var_t2__blk83_dn7, locals.var_t2__blk83_dn10, locals.var_t2__blk83_dn11, locals.var_t2__blk83_dn12, locals.var_t2__blk83_dn17,)
    }
};
        locals.var_t2__blk83 = assign6370_e4242;
        locals.var_t2__blk83_dn0 = assign6370_e4242_d_n0;
        locals.var_t2__blk83_dn2 = assign6370_e4242_d_n2;
        locals.var_t2__blk83_dn6 = assign6370_e4242_d_n6;
        locals.var_t2__blk83_dn7 = assign6370_e4242_d_n7;
        locals.var_t2__blk83_dn10 = assign6370_e4242_d_n10;
        locals.var_t2__blk83_dn11 = assign6370_e4242_d_n11;
        locals.var_t2__blk83_dn12 = assign6370_e4242_d_n12;
        locals.var_t2__blk83_dn17 = assign6370_e4242_d_n17;

        let (assign6380_e4252, assign6380_e4252_d_n0, assign6380_e4252_d_n2, assign6380_e4252_d_n6, assign6380_e4252_d_n7, assign6380_e4252_d_n10, assign6380_e4252_d_n11, assign6380_e4252_d_n12, assign6380_e4252_d_n17,) = {
    if (locals.var_guard88 != 0.0) {
        let assign6380_e4246: f64 = (locals.var_pb2c + locals.var_vfb);
        let assign6380_e4249: f64 = (locals.var_t2__blk83 * locals.var_c_fox_inv);
        let assign6380_e4250: f64 = (assign6380_e4246 + assign6380_e4249);
        (assign6380_e4250, (locals.var_pb2c_dn0 + ((locals.var_t2__blk83_dn0 * locals.var_c_fox_inv) + (locals.var_t2__blk83 * locals.var_c_fox_inv_dn0))), (locals.var_pb2c_dn2 + ((locals.var_t2__blk83_dn2 * locals.var_c_fox_inv) + (locals.var_t2__blk83 * locals.var_c_fox_inv_dn2))), (locals.var_pb2c_dn6 + ((locals.var_t2__blk83_dn6 * locals.var_c_fox_inv) + (locals.var_t2__blk83 * locals.var_c_fox_inv_dn6))), (locals.var_pb2c_dn7 + ((locals.var_t2__blk83_dn7 * locals.var_c_fox_inv) + (locals.var_t2__blk83 * locals.var_c_fox_inv_dn7))), (locals.var_pb2c_dn10 + ((locals.var_t2__blk83_dn10 * locals.var_c_fox_inv) + (locals.var_t2__blk83 * locals.var_c_fox_inv_dn10))), (locals.var_pb2c_dn11 + ((locals.var_t2__blk83_dn11 * locals.var_c_fox_inv) + (locals.var_t2__blk83 * locals.var_c_fox_inv_dn11))), (locals.var_pb2c_dn12 + ((locals.var_t2__blk83_dn12 * locals.var_c_fox_inv) + (locals.var_t2__blk83 * locals.var_c_fox_inv_dn12))), (locals.var_pb2c_dn17 + ((locals.var_t2__blk83_dn17 * locals.var_c_fox_inv) + (locals.var_t2__blk83 * locals.var_c_fox_inv_dn17))),)
    } else {
        (locals.var_vth0, locals.var_vth0_dn0, locals.var_vth0_dn2, locals.var_vth0_dn6, locals.var_vth0_dn7, locals.var_vth0_dn10, locals.var_vth0_dn11, locals.var_vth0_dn12, locals.var_vth0_dn17,)
    }
};
        locals.var_vth0 = assign6380_e4252;
        locals.var_vth0_dn0 = assign6380_e4252_d_n0;
        locals.var_vth0_dn2 = assign6380_e4252_d_n2;
        locals.var_vth0_dn6 = assign6380_e4252_d_n6;
        locals.var_vth0_dn7 = assign6380_e4252_d_n7;
        locals.var_vth0_dn10 = assign6380_e4252_d_n10;
        locals.var_vth0_dn11 = assign6380_e4252_d_n11;
        locals.var_vth0_dn12 = assign6380_e4252_d_n12;
        locals.var_vth0_dn17 = assign6380_e4252_d_n17;

        let (assign6390_e4258, assign6390_e4258_d_n0, assign6390_e4258_d_n2, assign6390_e4258_d_n6, assign6390_e4258_d_n7, assign6390_e4258_d_n10, assign6390_e4258_d_n11, assign6390_e4258_d_n12, assign6390_e4258_d_n17,) = {
    if (locals.var_guard88 != 0.0) {
        let assign6390_e4256: f64 = (1.034943e-10 * locals.var_c_fox_inv);
        (assign6390_e4256, (1.034943e-10 * locals.var_c_fox_inv_dn0), (1.034943e-10 * locals.var_c_fox_inv_dn2), (1.034943e-10 * locals.var_c_fox_inv_dn6), (1.034943e-10 * locals.var_c_fox_inv_dn7), (1.034943e-10 * locals.var_c_fox_inv_dn10), (1.034943e-10 * locals.var_c_fox_inv_dn11), (1.034943e-10 * locals.var_c_fox_inv_dn12), (1.034943e-10 * locals.var_c_fox_inv_dn17),)
    } else {
        (locals.var_t1__blk82, locals.var_t1__blk82_dn0, locals.var_t1__blk82_dn2, locals.var_t1__blk82_dn6, locals.var_t1__blk82_dn7, locals.var_t1__blk82_dn10, locals.var_t1__blk82_dn11, locals.var_t1__blk82_dn12, locals.var_t1__blk82_dn17,)
    }
};
        locals.var_t1__blk82 = assign6390_e4258;
        locals.var_t1__blk82_dn0 = assign6390_e4258_d_n0;
        locals.var_t1__blk82_dn2 = assign6390_e4258_d_n2;
        locals.var_t1__blk82_dn6 = assign6390_e4258_d_n6;
        locals.var_t1__blk82_dn7 = assign6390_e4258_d_n7;
        locals.var_t1__blk82_dn10 = assign6390_e4258_d_n10;
        locals.var_t1__blk82_dn11 = assign6390_e4258_d_n11;
        locals.var_t1__blk82_dn12 = assign6390_e4258_d_n12;
        locals.var_t1__blk82_dn17 = assign6390_e4258_d_n17;

        let (assign6400_e4266,) = {
    if (locals.var_guard88 != 0.0) {
        let assign6400_e4263: f64 = (p.p72 * p.p72);
        let assign6400_e4264: f64 = (1.0 / assign6400_e4263);
        (assign6400_e4264,)
    } else {
        (locals.var_t4__blk85,)
    }
};
        locals.var_t4__blk85 = assign6400_e4266;

        let (assign6410_e4274, assign6410_e4274_d_n0, assign6410_e4274_d_n2, assign6410_e4274_d_n6, assign6410_e4274_d_n7, assign6410_e4274_d_n10, assign6410_e4274_d_n11, assign6410_e4274_d_n12, assign6410_e4274_d_n17,) = {
    if (locals.var_guard88 != 0.0) {
        let assign6410_e4270: f64 = (2.0 * locals.var_wd0);
        let assign6410_e4272: f64 = (assign6410_e4270 * locals.var_t4__blk85);
        (assign6410_e4272, ((2.0 * locals.var_wd0_dn0) * locals.var_t4__blk85), ((2.0 * locals.var_wd0_dn2) * locals.var_t4__blk85), ((2.0 * locals.var_wd0_dn6) * locals.var_t4__blk85), ((2.0 * locals.var_wd0_dn7) * locals.var_t4__blk85), ((2.0 * locals.var_wd0_dn10) * locals.var_t4__blk85), ((2.0 * locals.var_wd0_dn11) * locals.var_t4__blk85), ((2.0 * locals.var_wd0_dn12) * locals.var_t4__blk85), ((2.0 * locals.var_wd0_dn17) * locals.var_t4__blk85),)
    } else {
        (locals.var_t3__blk84, locals.var_t3__blk84_dn0, locals.var_t3__blk84_dn2, locals.var_t3__blk84_dn6, locals.var_t3__blk84_dn7, locals.var_t3__blk84_dn10, locals.var_t3__blk84_dn11, locals.var_t3__blk84_dn12, locals.var_t3__blk84_dn17,)
    }
};
        locals.var_t3__blk84 = assign6410_e4274;
        locals.var_t3__blk84_dn0 = assign6410_e4274_d_n0;
        locals.var_t3__blk84_dn2 = assign6410_e4274_d_n2;
        locals.var_t3__blk84_dn6 = assign6410_e4274_d_n6;
        locals.var_t3__blk84_dn7 = assign6410_e4274_d_n7;
        locals.var_t3__blk84_dn10 = assign6410_e4274_d_n10;
        locals.var_t3__blk84_dn11 = assign6410_e4274_d_n11;
        locals.var_t3__blk84_dn12 = assign6410_e4274_d_n12;
        locals.var_t3__blk84_dn17 = assign6410_e4274_d_n17;

        let (assign6420_e4284, assign6420_e4284_d_n0, assign6420_e4284_d_n2, assign6420_e4284_d_n6, assign6420_e4284_d_n7, assign6420_e4284_d_n10, assign6420_e4284_d_n11, assign6420_e4284_d_n12, assign6420_e4284_d_n17,) = {
    if (locals.var_guard88 != 0.0) {
        let assign6420_e4278: f64 = (locals.var_t1__blk82 * locals.var_t3__blk84);
        let assign6420_e4281: f64 = (p.p69 - locals.var_pb20b);
        let assign6420_e4282: f64 = (assign6420_e4278 * assign6420_e4281);
        (assign6420_e4282, ((((locals.var_t1__blk82_dn0 * locals.var_t3__blk84) + (locals.var_t1__blk82 * locals.var_t3__blk84_dn0)) * assign6420_e4281) + (assign6420_e4278 * (-locals.var_pb20b_dn0))), ((((locals.var_t1__blk82_dn2 * locals.var_t3__blk84) + (locals.var_t1__blk82 * locals.var_t3__blk84_dn2)) * assign6420_e4281) + (assign6420_e4278 * (-locals.var_pb20b_dn2))), ((((locals.var_t1__blk82_dn6 * locals.var_t3__blk84) + (locals.var_t1__blk82 * locals.var_t3__blk84_dn6)) * assign6420_e4281) + (assign6420_e4278 * (-locals.var_pb20b_dn6))), ((((locals.var_t1__blk82_dn7 * locals.var_t3__blk84) + (locals.var_t1__blk82 * locals.var_t3__blk84_dn7)) * assign6420_e4281) + (assign6420_e4278 * (-locals.var_pb20b_dn7))), ((((locals.var_t1__blk82_dn10 * locals.var_t3__blk84) + (locals.var_t1__blk82 * locals.var_t3__blk84_dn10)) * assign6420_e4281) + (assign6420_e4278 * (-locals.var_pb20b_dn10))), ((((locals.var_t1__blk82_dn11 * locals.var_t3__blk84) + (locals.var_t1__blk82 * locals.var_t3__blk84_dn11)) * assign6420_e4281) + (assign6420_e4278 * (-locals.var_pb20b_dn11))), ((((locals.var_t1__blk82_dn12 * locals.var_t3__blk84) + (locals.var_t1__blk82 * locals.var_t3__blk84_dn12)) * assign6420_e4281) + (assign6420_e4278 * (-locals.var_pb20b_dn12))), ((((locals.var_t1__blk82_dn17 * locals.var_t3__blk84) + (locals.var_t1__blk82 * locals.var_t3__blk84_dn17)) * assign6420_e4281) + (assign6420_e4278 * (-locals.var_pb20b_dn17))),)
    } else {
        (locals.var_t5__blk86, locals.var_t5__blk86_dn0, locals.var_t5__blk86_dn2, locals.var_t5__blk86_dn6, locals.var_t5__blk86_dn7, locals.var_t5__blk86_dn10, locals.var_t5__blk86_dn11, locals.var_t5__blk86_dn12, locals.var_t5__blk86_dn17,)
    }
};
        locals.var_t5__blk86 = assign6420_e4284;
        locals.var_t5__blk86_dn0 = assign6420_e4284_d_n0;
        locals.var_t5__blk86_dn2 = assign6420_e4284_d_n2;
        locals.var_t5__blk86_dn6 = assign6420_e4284_d_n6;
        locals.var_t5__blk86_dn7 = assign6420_e4284_d_n7;
        locals.var_t5__blk86_dn10 = assign6420_e4284_d_n10;
        locals.var_t5__blk86_dn11 = assign6420_e4284_d_n11;
        locals.var_t5__blk86_dn12 = assign6420_e4284_d_n12;
        locals.var_t5__blk86_dn17 = assign6420_e4284_d_n17;

        let (assign6430_e4288, assign6430_e4288_d_n0, assign6430_e4288_d_n2, assign6430_e4288_d_n6, assign6430_e4288_d_n7, assign6430_e4288_d_n10, assign6430_e4288_d_n11, assign6430_e4288_d_n12, assign6430_e4288_d_n17,) = {
    if (locals.var_guard88 != 0.0) {
        (locals.var_t5__blk86, locals.var_t5__blk86_dn0, locals.var_t5__blk86_dn2, locals.var_t5__blk86_dn6, locals.var_t5__blk86_dn7, locals.var_t5__blk86_dn10, locals.var_t5__blk86_dn11, locals.var_t5__blk86_dn12, locals.var_t5__blk86_dn17,)
    } else {
        (locals.var_dvth0__blk87, locals.var_dvth0__blk87_dn0, locals.var_dvth0__blk87_dn2, locals.var_dvth0__blk87_dn6, locals.var_dvth0__blk87_dn7, locals.var_dvth0__blk87_dn10, locals.var_dvth0__blk87_dn11, locals.var_dvth0__blk87_dn12, locals.var_dvth0__blk87_dn17,)
    }
};
        locals.var_dvth0__blk87 = assign6430_e4288;
        locals.var_dvth0__blk87_dn0 = assign6430_e4288_d_n0;
        locals.var_dvth0__blk87_dn2 = assign6430_e4288_d_n2;
        locals.var_dvth0__blk87_dn6 = assign6430_e4288_d_n6;
        locals.var_dvth0__blk87_dn7 = assign6430_e4288_d_n7;
        locals.var_dvth0__blk87_dn10 = assign6430_e4288_d_n10;
        locals.var_dvth0__blk87_dn11 = assign6430_e4288_d_n11;
        locals.var_dvth0__blk87_dn12 = assign6430_e4288_d_n12;
        locals.var_dvth0__blk87_dn17 = assign6430_e4288_d_n17;

        let (assign6440_e4294, assign6440_e4294_d_n0, assign6440_e4294_d_n2, assign6440_e4294_d_n6, assign6440_e4294_d_n7, assign6440_e4294_d_n10, assign6440_e4294_d_n11, assign6440_e4294_d_n12, assign6440_e4294_d_n17,) = {
    if (locals.var_guard88 != 0.0) {
        let assign6440_e4292: f64 = (locals.var_vthp - locals.var_vth0);
        (assign6440_e4292, (locals.var_vthp_dn0 - locals.var_vth0_dn0), (locals.var_vthp_dn2 - locals.var_vth0_dn2), (locals.var_vthp_dn6 - locals.var_vth0_dn6), (locals.var_vthp_dn7 - locals.var_vth0_dn7), (locals.var_vthp_dn10 - locals.var_vth0_dn10), (locals.var_vthp_dn11 - locals.var_vth0_dn11), (locals.var_vthp_dn12 - locals.var_vth0_dn12), (locals.var_vthp_dn17 - locals.var_vth0_dn17),)
    } else {
        (locals.var_t1__blk82, locals.var_t1__blk82_dn0, locals.var_t1__blk82_dn2, locals.var_t1__blk82_dn6, locals.var_t1__blk82_dn7, locals.var_t1__blk82_dn10, locals.var_t1__blk82_dn11, locals.var_t1__blk82_dn12, locals.var_t1__blk82_dn17,)
    }
};
        locals.var_t1__blk82 = assign6440_e4294;
        locals.var_t1__blk82_dn0 = assign6440_e4294_d_n0;
        locals.var_t1__blk82_dn2 = assign6440_e4294_d_n2;
        locals.var_t1__blk82_dn6 = assign6440_e4294_d_n6;
        locals.var_t1__blk82_dn7 = assign6440_e4294_d_n7;
        locals.var_t1__blk82_dn10 = assign6440_e4294_d_n10;
        locals.var_t1__blk82_dn11 = assign6440_e4294_d_n11;
        locals.var_t1__blk82_dn12 = assign6440_e4294_d_n12;
        locals.var_t1__blk82_dn17 = assign6440_e4294_d_n17;

        let (assign6450_e4300,) = {
    if (locals.var_guard88 != 0.0) {
        let assign6450_e4298: f64 = (locals.var_uc_scp3 / p.p72);
        (assign6450_e4298,)
    } else {
        (locals.var_t0__blk81,)
    }
};
        locals.var_t0__blk81 = assign6450_e4300;

        let (assign6460_e4308, assign6460_e4308_d_n0, assign6460_e4308_d_n2, assign6460_e4308_d_n6, assign6460_e4308_d_n7, assign6460_e4308_d_n10, assign6460_e4308_d_n11, assign6460_e4308_d_n12, assign6460_e4308_d_n17,) = {
    if (locals.var_guard88 != 0.0) {
        let assign6460_e4305: f64 = (locals.var_t0__blk81 * locals.var_pbsum);
        let assign6460_e4306: f64 = (p.p80 + assign6460_e4305);
        (assign6460_e4306, (locals.var_t0__blk81 * locals.var_pbsum_dn0), (locals.var_t0__blk81 * locals.var_pbsum_dn2), (locals.var_t0__blk81 * locals.var_pbsum_dn6), (locals.var_t0__blk81 * locals.var_pbsum_dn7), (locals.var_t0__blk81 * locals.var_pbsum_dn10), (locals.var_t0__blk81 * locals.var_pbsum_dn11), (locals.var_t0__blk81 * locals.var_pbsum_dn12), (locals.var_t0__blk81 * locals.var_pbsum_dn17),)
    } else {
        (locals.var_t2__blk83, locals.var_t2__blk83_dn0, locals.var_t2__blk83_dn2, locals.var_t2__blk83_dn6, locals.var_t2__blk83_dn7, locals.var_t2__blk83_dn10, locals.var_t2__blk83_dn11, locals.var_t2__blk83_dn12, locals.var_t2__blk83_dn17,)
    }
};
        locals.var_t2__blk83 = assign6460_e4308;
        locals.var_t2__blk83_dn0 = assign6460_e4308_d_n0;
        locals.var_t2__blk83_dn2 = assign6460_e4308_d_n2;
        locals.var_t2__blk83_dn6 = assign6460_e4308_d_n6;
        locals.var_t2__blk83_dn7 = assign6460_e4308_d_n7;
        locals.var_t2__blk83_dn10 = assign6460_e4308_d_n10;
        locals.var_t2__blk83_dn11 = assign6460_e4308_d_n11;
        locals.var_t2__blk83_dn12 = assign6460_e4308_d_n12;
        locals.var_t2__blk83_dn17 = assign6460_e4308_d_n17;

        let (assign6470_e4312, assign6470_e4312_d_n0, assign6470_e4312_d_n2, assign6470_e4312_d_n6, assign6470_e4312_d_n7, assign6470_e4312_d_n10, assign6470_e4312_d_n11, assign6470_e4312_d_n12, assign6470_e4312_d_n17,) = {
    if (locals.var_guard88 != 0.0) {
        (locals.var_uc_scp2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk86, locals.var_t5__blk86_dn0, locals.var_t5__blk86_dn2, locals.var_t5__blk86_dn6, locals.var_t5__blk86_dn7, locals.var_t5__blk86_dn10, locals.var_t5__blk86_dn11, locals.var_t5__blk86_dn12, locals.var_t5__blk86_dn17,)
    }
};
        locals.var_t5__blk86 = assign6470_e4312;
        locals.var_t5__blk86_dn0 = assign6470_e4312_d_n0;
        locals.var_t5__blk86_dn2 = assign6470_e4312_d_n2;
        locals.var_t5__blk86_dn6 = assign6470_e4312_d_n6;
        locals.var_t5__blk86_dn7 = assign6470_e4312_d_n7;
        locals.var_t5__blk86_dn10 = assign6470_e4312_d_n10;
        locals.var_t5__blk86_dn11 = assign6470_e4312_d_n11;
        locals.var_t5__blk86_dn12 = assign6470_e4312_d_n12;
        locals.var_t5__blk86_dn17 = assign6470_e4312_d_n17;

    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6480_e4320, assign6480_e4320_d_n0, assign6480_e4320_d_n2, assign6480_e4320_d_n6, assign6480_e4320_d_n7, assign6480_e4320_d_n10, assign6480_e4320_d_n11, assign6480_e4320_d_n12, assign6480_e4320_d_n17,) = {
    if (locals.var_guard88 != 0.0) {
        let assign6480_e4317: f64 = (locals.var_t5__blk86 * locals.var_vdsz);
        let assign6480_e4318: f64 = (locals.var_t2__blk83 + assign6480_e4317);
        (assign6480_e4318, (locals.var_t2__blk83_dn0 + ((locals.var_t5__blk86_dn0 * locals.var_vdsz) + (locals.var_t5__blk86 * locals.var_vdsz_dn0))), (locals.var_t2__blk83_dn2 + ((locals.var_t5__blk86_dn2 * locals.var_vdsz) + (locals.var_t5__blk86 * locals.var_vdsz_dn2))), (locals.var_t2__blk83_dn6 + ((locals.var_t5__blk86_dn6 * locals.var_vdsz) + (locals.var_t5__blk86 * locals.var_vdsz_dn6))), (locals.var_t2__blk83_dn7 + ((locals.var_t5__blk86_dn7 * locals.var_vdsz) + (locals.var_t5__blk86 * locals.var_vdsz_dn7))), (locals.var_t2__blk83_dn10 + ((locals.var_t5__blk86_dn10 * locals.var_vdsz) + (locals.var_t5__blk86 * locals.var_vdsz_dn10))), (locals.var_t2__blk83_dn11 + ((locals.var_t5__blk86_dn11 * locals.var_vdsz) + (locals.var_t5__blk86 * locals.var_vdsz_dn11))), (locals.var_t2__blk83_dn12 + ((locals.var_t5__blk86_dn12 * locals.var_vdsz) + (locals.var_t5__blk86 * locals.var_vdsz_dn12))), (locals.var_t2__blk83_dn17 + ((locals.var_t5__blk86_dn17 * locals.var_vdsz) + (locals.var_t5__blk86 * locals.var_vdsz_dn17))),)
    } else {
        (locals.var_t3__blk84, locals.var_t3__blk84_dn0, locals.var_t3__blk84_dn2, locals.var_t3__blk84_dn6, locals.var_t3__blk84_dn7, locals.var_t3__blk84_dn10, locals.var_t3__blk84_dn11, locals.var_t3__blk84_dn12, locals.var_t3__blk84_dn17,)
    }
};
        locals.var_t3__blk84 = assign6480_e4320;
        locals.var_t3__blk84_dn0 = assign6480_e4320_d_n0;
        locals.var_t3__blk84_dn2 = assign6480_e4320_d_n2;
        locals.var_t3__blk84_dn6 = assign6480_e4320_d_n6;
        locals.var_t3__blk84_dn7 = assign6480_e4320_d_n7;
        locals.var_t3__blk84_dn10 = assign6480_e4320_d_n10;
        locals.var_t3__blk84_dn11 = assign6480_e4320_d_n11;
        locals.var_t3__blk84_dn12 = assign6480_e4320_d_n12;
        locals.var_t3__blk84_dn17 = assign6480_e4320_d_n17;

        let (assign6490_e4328, assign6490_e4328_d_n0, assign6490_e4328_d_n2, assign6490_e4328_d_n6, assign6490_e4328_d_n7, assign6490_e4328_d_n10, assign6490_e4328_d_n11, assign6490_e4328_d_n12, assign6490_e4328_d_n17,) = {
    if (locals.var_guard88 != 0.0) {
        let assign6490_e4324: f64 = (locals.var_t1__blk82 * locals.var_dvth0__blk87);
        let assign6490_e4326: f64 = (assign6490_e4324 * locals.var_t3__blk84);
        (assign6490_e4326, ((((locals.var_t1__blk82_dn0 * locals.var_dvth0__blk87) + (locals.var_t1__blk82 * locals.var_dvth0__blk87_dn0)) * locals.var_t3__blk84) + (assign6490_e4324 * locals.var_t3__blk84_dn0)), ((((locals.var_t1__blk82_dn2 * locals.var_dvth0__blk87) + (locals.var_t1__blk82 * locals.var_dvth0__blk87_dn2)) * locals.var_t3__blk84) + (assign6490_e4324 * locals.var_t3__blk84_dn2)), ((((locals.var_t1__blk82_dn6 * locals.var_dvth0__blk87) + (locals.var_t1__blk82 * locals.var_dvth0__blk87_dn6)) * locals.var_t3__blk84) + (assign6490_e4324 * locals.var_t3__blk84_dn6)), ((((locals.var_t1__blk82_dn7 * locals.var_dvth0__blk87) + (locals.var_t1__blk82 * locals.var_dvth0__blk87_dn7)) * locals.var_t3__blk84) + (assign6490_e4324 * locals.var_t3__blk84_dn7)), ((((locals.var_t1__blk82_dn10 * locals.var_dvth0__blk87) + (locals.var_t1__blk82 * locals.var_dvth0__blk87_dn10)) * locals.var_t3__blk84) + (assign6490_e4324 * locals.var_t3__blk84_dn10)), ((((locals.var_t1__blk82_dn11 * locals.var_dvth0__blk87) + (locals.var_t1__blk82 * locals.var_dvth0__blk87_dn11)) * locals.var_t3__blk84) + (assign6490_e4324 * locals.var_t3__blk84_dn11)), ((((locals.var_t1__blk82_dn12 * locals.var_dvth0__blk87) + (locals.var_t1__blk82 * locals.var_dvth0__blk87_dn12)) * locals.var_t3__blk84) + (assign6490_e4324 * locals.var_t3__blk84_dn12)), ((((locals.var_t1__blk82_dn17 * locals.var_dvth0__blk87) + (locals.var_t1__blk82 * locals.var_dvth0__blk87_dn17)) * locals.var_t3__blk84) + (assign6490_e4324 * locals.var_t3__blk84_dn17)),)
    } else {
        (locals.var_dvthlp, locals.var_dvthlp_dn0, locals.var_dvthlp_dn2, locals.var_dvthlp_dn6, locals.var_dvthlp_dn7, locals.var_dvthlp_dn10, locals.var_dvthlp_dn11, locals.var_dvthlp_dn12, locals.var_dvthlp_dn17,)
    }
};
        locals.var_dvthlp = assign6490_e4328;
        locals.var_dvthlp_dn0 = assign6490_e4328_d_n0;
        locals.var_dvthlp_dn2 = assign6490_e4328_d_n2;
        locals.var_dvthlp_dn6 = assign6490_e4328_d_n6;
        locals.var_dvthlp_dn7 = assign6490_e4328_d_n7;
        locals.var_dvthlp_dn10 = assign6490_e4328_d_n10;
        locals.var_dvthlp_dn11 = assign6490_e4328_d_n11;
        locals.var_dvthlp_dn12 = assign6490_e4328_d_n12;
        locals.var_dvthlp_dn17 = assign6490_e4328_d_n17;

        let (assign6500_e4333, assign6500_e4333_d_n0, assign6500_e4333_d_n2, assign6500_e4333_d_n6, assign6500_e4333_d_n7, assign6500_e4333_d_n10, assign6500_e4333_d_n11, assign6500_e4333_d_n12, assign6500_e4333_d_n17,) = {
    if (locals.var_guard88 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvthlp, locals.var_dvthlp_dn0, locals.var_dvthlp_dn2, locals.var_dvthlp_dn6, locals.var_dvthlp_dn7, locals.var_dvthlp_dn10, locals.var_dvthlp_dn11, locals.var_dvthlp_dn12, locals.var_dvthlp_dn17,)
    }
};
        locals.var_dvthlp = assign6500_e4333;
        locals.var_dvthlp_dn0 = assign6500_e4333_d_n0;
        locals.var_dvthlp_dn2 = assign6500_e4333_d_n2;
        locals.var_dvthlp_dn6 = assign6500_e4333_d_n6;
        locals.var_dvthlp_dn7 = assign6500_e4333_d_n7;
        locals.var_dvthlp_dn10 = assign6500_e4333_d_n10;
        locals.var_dvthlp_dn11 = assign6500_e4333_d_n11;
        locals.var_dvthlp_dn12 = assign6500_e4333_d_n12;
        locals.var_dvthlp_dn17 = assign6500_e4333_d_n17;

        let assign6510_e4336: f64 = (1.034943e-10 * locals.var_wd0);
        let assign6510_e4338: f64 = (assign6510_e4336 * 2.0);
        locals.var_t0__blk89 = assign6510_e4338;
        locals.var_t0__blk89_dn0 = ((1.034943e-10 * locals.var_wd0_dn0) * 2.0);
        locals.var_t0__blk89_dn2 = ((1.034943e-10 * locals.var_wd0_dn2) * 2.0);
        locals.var_t0__blk89_dn6 = ((1.034943e-10 * locals.var_wd0_dn6) * 2.0);
        locals.var_t0__blk89_dn7 = ((1.034943e-10 * locals.var_wd0_dn7) * 2.0);
        locals.var_t0__blk89_dn10 = ((1.034943e-10 * locals.var_wd0_dn10) * 2.0);
        locals.var_t0__blk89_dn11 = ((1.034943e-10 * locals.var_wd0_dn11) * 2.0);
        locals.var_t0__blk89_dn12 = ((1.034943e-10 * locals.var_wd0_dn12) * 2.0);
        locals.var_t0__blk89_dn17 = ((1.034943e-10 * locals.var_wd0_dn17) * 2.0);

        let assign6520_e4341: f64 = (locals.var_c_fox_inv * locals.var_t0__blk89);
        locals.var_t1__blk90 = assign6520_e4341;
        locals.var_t1__blk90_dn0 = ((locals.var_c_fox_inv_dn0 * locals.var_t0__blk89) + (locals.var_c_fox_inv * locals.var_t0__blk89_dn0));
        locals.var_t1__blk90_dn2 = ((locals.var_c_fox_inv_dn2 * locals.var_t0__blk89) + (locals.var_c_fox_inv * locals.var_t0__blk89_dn2));
        locals.var_t1__blk90_dn6 = ((locals.var_c_fox_inv_dn6 * locals.var_t0__blk89) + (locals.var_c_fox_inv * locals.var_t0__blk89_dn6));
        locals.var_t1__blk90_dn7 = ((locals.var_c_fox_inv_dn7 * locals.var_t0__blk89) + (locals.var_c_fox_inv * locals.var_t0__blk89_dn7));
        locals.var_t1__blk90_dn10 = ((locals.var_c_fox_inv_dn10 * locals.var_t0__blk89) + (locals.var_c_fox_inv * locals.var_t0__blk89_dn10));
        locals.var_t1__blk90_dn11 = ((locals.var_c_fox_inv_dn11 * locals.var_t0__blk89) + (locals.var_c_fox_inv * locals.var_t0__blk89_dn11));
        locals.var_t1__blk90_dn12 = ((locals.var_c_fox_inv_dn12 * locals.var_t0__blk89) + (locals.var_c_fox_inv * locals.var_t0__blk89_dn12));
        locals.var_t1__blk90_dn17 = ((locals.var_c_fox_inv_dn17 * locals.var_t0__blk89) + (locals.var_c_fox_inv * locals.var_t0__blk89_dn17));

        let assign6530_e4344: f64 = (p.p69 - locals.var_pb20b);
        locals.var_t2__blk91 = assign6530_e4344;
        locals.var_t2__blk91_dn0 = (-locals.var_pb20b_dn0);
        locals.var_t2__blk91_dn2 = (-locals.var_pb20b_dn2);
        locals.var_t2__blk91_dn6 = (-locals.var_pb20b_dn6);
        locals.var_t2__blk91_dn7 = (-locals.var_pb20b_dn7);
        locals.var_t2__blk91_dn10 = (-locals.var_pb20b_dn10);
        locals.var_t2__blk91_dn11 = (-locals.var_pb20b_dn11);
        locals.var_t2__blk91_dn12 = (-locals.var_pb20b_dn12);
        locals.var_t2__blk91_dn17 = (-locals.var_pb20b_dn17);

        let assign6540_e4347: f64 = (locals.var_lgleff - p.p71);
        locals.var_t3__blk92 = assign6540_e4347;

        let assign6550_e4351: f64 = (locals.var_t3__blk92 * locals.var_t3__blk92);
        let assign6550_e4352: f64 = (1.0 / assign6550_e4351);
        locals.var_t4__blk93 = assign6550_e4352;
        locals.var_t4__blk93_dn0 = 0.0;
        locals.var_t4__blk93_dn2 = 0.0;
        locals.var_t4__blk93_dn6 = 0.0;
        locals.var_t4__blk93_dn7 = 0.0;
        locals.var_t4__blk93_dn10 = 0.0;
        locals.var_t4__blk93_dn11 = 0.0;
        locals.var_t4__blk93_dn12 = 0.0;
        locals.var_t4__blk93_dn17 = 0.0;

        let assign6560_e4355: f64 = (locals.var_t1__blk90 * locals.var_t2__blk91);
        let assign6560_e4357: f64 = (assign6560_e4355 * locals.var_t4__blk93);
        locals.var_dvth0__blk95 = assign6560_e4357;
        locals.var_dvth0__blk95_dn0 = ((((locals.var_t1__blk90_dn0 * locals.var_t2__blk91) + (locals.var_t1__blk90 * locals.var_t2__blk91_dn0)) * locals.var_t4__blk93) + (assign6560_e4355 * locals.var_t4__blk93_dn0));
        locals.var_dvth0__blk95_dn2 = ((((locals.var_t1__blk90_dn2 * locals.var_t2__blk91) + (locals.var_t1__blk90 * locals.var_t2__blk91_dn2)) * locals.var_t4__blk93) + (assign6560_e4355 * locals.var_t4__blk93_dn2));
        locals.var_dvth0__blk95_dn6 = ((((locals.var_t1__blk90_dn6 * locals.var_t2__blk91) + (locals.var_t1__blk90 * locals.var_t2__blk91_dn6)) * locals.var_t4__blk93) + (assign6560_e4355 * locals.var_t4__blk93_dn6));
        locals.var_dvth0__blk95_dn7 = ((((locals.var_t1__blk90_dn7 * locals.var_t2__blk91) + (locals.var_t1__blk90 * locals.var_t2__blk91_dn7)) * locals.var_t4__blk93) + (assign6560_e4355 * locals.var_t4__blk93_dn7));
        locals.var_dvth0__blk95_dn10 = ((((locals.var_t1__blk90_dn10 * locals.var_t2__blk91) + (locals.var_t1__blk90 * locals.var_t2__blk91_dn10)) * locals.var_t4__blk93) + (assign6560_e4355 * locals.var_t4__blk93_dn10));
        locals.var_dvth0__blk95_dn11 = ((((locals.var_t1__blk90_dn11 * locals.var_t2__blk91) + (locals.var_t1__blk90 * locals.var_t2__blk91_dn11)) * locals.var_t4__blk93) + (assign6560_e4355 * locals.var_t4__blk93_dn11));
        locals.var_dvth0__blk95_dn12 = ((((locals.var_t1__blk90_dn12 * locals.var_t2__blk91) + (locals.var_t1__blk90 * locals.var_t2__blk91_dn12)) * locals.var_t4__blk93) + (assign6560_e4355 * locals.var_t4__blk93_dn12));
        locals.var_dvth0__blk95_dn17 = ((((locals.var_t1__blk90_dn17 * locals.var_t2__blk91) + (locals.var_t1__blk90 * locals.var_t2__blk91_dn17)) * locals.var_t4__blk93) + (assign6560_e4355 * locals.var_t4__blk93_dn17));

        let assign6570_e4360: f64 = (locals.var_uc_sc3 / locals.var_lgleff);
        locals.var_t1__blk90 = assign6570_e4360;
        locals.var_t1__blk90_dn0 = 0.0;
        locals.var_t1__blk90_dn2 = 0.0;
        locals.var_t1__blk90_dn6 = 0.0;
        locals.var_t1__blk90_dn7 = 0.0;
        locals.var_t1__blk90_dn10 = 0.0;
        locals.var_t1__blk90_dn11 = 0.0;
        locals.var_t1__blk90_dn12 = 0.0;
        locals.var_t1__blk90_dn17 = 0.0;

        let assign6580_e4364: f64 = (locals.var_t1__blk90 * locals.var_pbsum);
        let assign6580_e4365: f64 = (p.p83 + assign6580_e4364);
        locals.var_t4__blk93 = assign6580_e4365;
        locals.var_t4__blk93_dn0 = ((locals.var_t1__blk90_dn0 * locals.var_pbsum) + (locals.var_t1__blk90 * locals.var_pbsum_dn0));
        locals.var_t4__blk93_dn2 = ((locals.var_t1__blk90_dn2 * locals.var_pbsum) + (locals.var_t1__blk90 * locals.var_pbsum_dn2));
        locals.var_t4__blk93_dn6 = ((locals.var_t1__blk90_dn6 * locals.var_pbsum) + (locals.var_t1__blk90 * locals.var_pbsum_dn6));
        locals.var_t4__blk93_dn7 = ((locals.var_t1__blk90_dn7 * locals.var_pbsum) + (locals.var_t1__blk90 * locals.var_pbsum_dn7));
        locals.var_t4__blk93_dn10 = ((locals.var_t1__blk90_dn10 * locals.var_pbsum) + (locals.var_t1__blk90 * locals.var_pbsum_dn10));
        locals.var_t4__blk93_dn11 = ((locals.var_t1__blk90_dn11 * locals.var_pbsum) + (locals.var_t1__blk90 * locals.var_pbsum_dn11));
        locals.var_t4__blk93_dn12 = ((locals.var_t1__blk90_dn12 * locals.var_pbsum) + (locals.var_t1__blk90 * locals.var_pbsum_dn12));
        locals.var_t4__blk93_dn17 = ((locals.var_t1__blk90_dn17 * locals.var_pbsum) + (locals.var_t1__blk90 * locals.var_pbsum_dn17));

        let assign6590_e4369: f64 = (locals.var_uc_sc2 * locals.var_vdsz);
        let assign6590_e4370: f64 = (locals.var_t4__blk93 + assign6590_e4369);
        locals.var_t5__blk94 = assign6590_e4370;
        locals.var_t5__blk94_dn0 = (locals.var_t4__blk93_dn0 + (locals.var_uc_sc2 * locals.var_vdsz_dn0));
        locals.var_t5__blk94_dn2 = (locals.var_t4__blk93_dn2 + (locals.var_uc_sc2 * locals.var_vdsz_dn2));
        locals.var_t5__blk94_dn6 = (locals.var_t4__blk93_dn6 + (locals.var_uc_sc2 * locals.var_vdsz_dn6));
        locals.var_t5__blk94_dn7 = (locals.var_t4__blk93_dn7 + (locals.var_uc_sc2 * locals.var_vdsz_dn7));
        locals.var_t5__blk94_dn10 = (locals.var_t4__blk93_dn10 + (locals.var_uc_sc2 * locals.var_vdsz_dn10));
        locals.var_t5__blk94_dn11 = (locals.var_t4__blk93_dn11 + (locals.var_uc_sc2 * locals.var_vdsz_dn11));
        locals.var_t5__blk94_dn12 = (locals.var_t4__blk93_dn12 + (locals.var_uc_sc2 * locals.var_vdsz_dn12));
        locals.var_t5__blk94_dn17 = (locals.var_t4__blk93_dn17 + (locals.var_uc_sc2 * locals.var_vdsz_dn17));

        let assign6600_e4373: f64 = (locals.var_dvth0__blk95 * locals.var_t5__blk94);
        locals.var_dvthsc = assign6600_e4373;
        locals.var_dvthsc_dn0 = ((locals.var_dvth0__blk95_dn0 * locals.var_t5__blk94) + (locals.var_dvth0__blk95 * locals.var_t5__blk94_dn0));
        locals.var_dvthsc_dn2 = ((locals.var_dvth0__blk95_dn2 * locals.var_t5__blk94) + (locals.var_dvth0__blk95 * locals.var_t5__blk94_dn2));
        locals.var_dvthsc_dn6 = ((locals.var_dvth0__blk95_dn6 * locals.var_t5__blk94) + (locals.var_dvth0__blk95 * locals.var_t5__blk94_dn6));
        locals.var_dvthsc_dn7 = ((locals.var_dvth0__blk95_dn7 * locals.var_t5__blk94) + (locals.var_dvth0__blk95 * locals.var_t5__blk94_dn7));
        locals.var_dvthsc_dn10 = ((locals.var_dvth0__blk95_dn10 * locals.var_t5__blk94) + (locals.var_dvth0__blk95 * locals.var_t5__blk94_dn10));
        locals.var_dvthsc_dn11 = ((locals.var_dvth0__blk95_dn11 * locals.var_t5__blk94) + (locals.var_dvth0__blk95 * locals.var_t5__blk94_dn11));
        locals.var_dvthsc_dn12 = ((locals.var_dvth0__blk95_dn12 * locals.var_t5__blk94) + (locals.var_dvth0__blk95 * locals.var_t5__blk94_dn12));
        locals.var_dvthsc_dn17 = ((locals.var_dvth0__blk95_dn17 * locals.var_t5__blk94) + (locals.var_dvth0__blk95 * locals.var_t5__blk94_dn17));

        let assign6610_e4376: f64 = if p.p86 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard99 = assign6610_e4376;

        let (assign6620_e4390, assign6620_e4390_d_n0, assign6620_e4390_d_n2, assign6620_e4390_d_n6, assign6620_e4390_d_n7, assign6620_e4390_d_n10, assign6620_e4390_d_n11, assign6620_e4390_d_n12, assign6620_e4390_d_n17,) = {
    if (locals.var_guard99 != 0.0) {
        let assign6620_e4380: f64 = (locals.var_eg + locals.var_pb2);
        let assign6620_e4383: f64 = (2.0 * p.p88);
        let assign6620_e4384: f64 = (assign6620_e4380 - assign6620_e4383);
        let assign6620_e4387: f64 = (p.p87 * locals.var_vdsz);
        let assign6620_e4388: f64 = (assign6620_e4384 + assign6620_e4387);
        (assign6620_e4388, ((locals.var_eg_dn0 + locals.var_pb2_dn0) + (p.p87 * locals.var_vdsz_dn0)), ((locals.var_eg_dn2 + locals.var_pb2_dn2) + (p.p87 * locals.var_vdsz_dn2)), ((locals.var_eg_dn6 + locals.var_pb2_dn6) + (p.p87 * locals.var_vdsz_dn6)), ((locals.var_eg_dn7 + locals.var_pb2_dn7) + (p.p87 * locals.var_vdsz_dn7)), ((locals.var_eg_dn10 + locals.var_pb2_dn10) + (p.p87 * locals.var_vdsz_dn10)), ((locals.var_eg_dn11 + locals.var_pb2_dn11) + (p.p87 * locals.var_vdsz_dn11)), ((locals.var_eg_dn12 + locals.var_pb2_dn12) + (p.p87 * locals.var_vdsz_dn12)), ((locals.var_eg_dn17 + locals.var_pb2_dn17) + (p.p87 * locals.var_vdsz_dn17)),)
    } else {
        (locals.var_t1__blk96, locals.var_t1__blk96_dn0, locals.var_t1__blk96_dn2, locals.var_t1__blk96_dn6, locals.var_t1__blk96_dn7, locals.var_t1__blk96_dn10, locals.var_t1__blk96_dn11, locals.var_t1__blk96_dn12, locals.var_t1__blk96_dn17,)
    }
};
        locals.var_t1__blk96 = assign6620_e4390;
        locals.var_t1__blk96_dn0 = assign6620_e4390_d_n0;
        locals.var_t1__blk96_dn2 = assign6620_e4390_d_n2;
        locals.var_t1__blk96_dn6 = assign6620_e4390_d_n6;
        locals.var_t1__blk96_dn7 = assign6620_e4390_d_n7;
        locals.var_t1__blk96_dn10 = assign6620_e4390_d_n10;
        locals.var_t1__blk96_dn11 = assign6620_e4390_d_n11;
        locals.var_t1__blk96_dn12 = assign6620_e4390_d_n12;
        locals.var_t1__blk96_dn17 = assign6620_e4390_d_n17;

        let (assign6630_e4398,) = {
    if (locals.var_guard99 != 0.0) {
        let assign6630_e4394: f64 = (locals.var_lgleff * 0.5);
        let assign6630_e4396: f64 = (assign6630_e4394 + locals.var_mks_parl1);
        (assign6630_e4396,)
    } else {
        (locals.var_t2__blk97,)
    }
};
        locals.var_t2__blk97 = assign6630_e4398;

        let (assign6640_e4406,) = {
    if (locals.var_guard99 != 0.0) {
        let assign6640_e4402: f64 = (p.p86 * p.p237);
        let assign6640_e4404: f64 = (assign6640_e4402 / locals.var_t2__blk97);
        (assign6640_e4404,)
    } else {
        (locals.var_t3__blk98,)
    }
};
        locals.var_t3__blk98 = assign6640_e4406;

        let (assign6650_e4412, assign6650_e4412_d_n0, assign6650_e4412_d_n2, assign6650_e4412_d_n6, assign6650_e4412_d_n7, assign6650_e4412_d_n10, assign6650_e4412_d_n11, assign6650_e4412_d_n12, assign6650_e4412_d_n17,) = {
    if (locals.var_guard99 != 0.0) {
        let assign6650_e4410: f64 = (locals.var_t1__blk96 * locals.var_t3__blk98);
        (assign6650_e4410, (locals.var_t1__blk96_dn0 * locals.var_t3__blk98), (locals.var_t1__blk96_dn2 * locals.var_t3__blk98), (locals.var_t1__blk96_dn6 * locals.var_t3__blk98), (locals.var_t1__blk96_dn7 * locals.var_t3__blk98), (locals.var_t1__blk96_dn10 * locals.var_t3__blk98), (locals.var_t1__blk96_dn11 * locals.var_t3__blk98), (locals.var_t1__blk96_dn12 * locals.var_t3__blk98), (locals.var_t1__blk96_dn17 * locals.var_t3__blk98),)
    } else {
        (locals.var_dvthscr, locals.var_dvthscr_dn0, locals.var_dvthscr_dn2, locals.var_dvthscr_dn6, locals.var_dvthscr_dn7, locals.var_dvthscr_dn10, locals.var_dvthscr_dn11, locals.var_dvthscr_dn12, locals.var_dvthscr_dn17,)
    }
};
        locals.var_dvthscr = assign6650_e4412;
        locals.var_dvthscr_dn0 = assign6650_e4412_d_n0;
        locals.var_dvthscr_dn2 = assign6650_e4412_d_n2;
        locals.var_dvthscr_dn6 = assign6650_e4412_d_n6;
        locals.var_dvthscr_dn7 = assign6650_e4412_d_n7;
        locals.var_dvthscr_dn10 = assign6650_e4412_d_n10;
        locals.var_dvthscr_dn11 = assign6650_e4412_d_n11;
        locals.var_dvthscr_dn12 = assign6650_e4412_d_n12;
        locals.var_dvthscr_dn17 = assign6650_e4412_d_n17;

        let (assign6660_e4417, assign6660_e4417_d_n0, assign6660_e4417_d_n2, assign6660_e4417_d_n6, assign6660_e4417_d_n7, assign6660_e4417_d_n10, assign6660_e4417_d_n11, assign6660_e4417_d_n12, assign6660_e4417_d_n17,) = {
    if (locals.var_guard99 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvthscr, locals.var_dvthscr_dn0, locals.var_dvthscr_dn2, locals.var_dvthscr_dn6, locals.var_dvthscr_dn7, locals.var_dvthscr_dn10, locals.var_dvthscr_dn11, locals.var_dvthscr_dn12, locals.var_dvthscr_dn17,)
    }
};
        locals.var_dvthscr = assign6660_e4417;
        locals.var_dvthscr_dn0 = assign6660_e4417_d_n0;
        locals.var_dvthscr_dn2 = assign6660_e4417_d_n2;
        locals.var_dvthscr_dn6 = assign6660_e4417_d_n6;
        locals.var_dvthscr_dn7 = assign6660_e4417_d_n7;
        locals.var_dvthscr_dn10 = assign6660_e4417_d_n10;
        locals.var_dvthscr_dn11 = assign6660_e4417_d_n11;
        locals.var_dvthscr_dn12 = assign6660_e4417_d_n12;
        locals.var_dvthscr_dn17 = assign6660_e4417_d_n17;

        locals.var_t1__blk100 = locals.var_c_fox_inv;
        locals.var_t1__blk100_dn0 = locals.var_c_fox_inv_dn0;
        locals.var_t1__blk100_dn2 = locals.var_c_fox_inv_dn2;
        locals.var_t1__blk100_dn6 = locals.var_c_fox_inv_dn6;
        locals.var_t1__blk100_dn7 = locals.var_c_fox_inv_dn7;
        locals.var_t1__blk100_dn10 = locals.var_c_fox_inv_dn10;
        locals.var_t1__blk100_dn11 = locals.var_c_fox_inv_dn11;
        locals.var_t1__blk100_dn12 = locals.var_c_fox_inv_dn12;
        locals.var_t1__blk100_dn17 = locals.var_c_fox_inv_dn17;

        let assign6680_e4423: f64 = (locals.var_mks_wfc / locals.var_weff);
        let assign6680_e4424: f64 = (locals.var_c_fox + assign6680_e4423);
        let assign6680_e4425: f64 = (1.0 / assign6680_e4424);
        locals.var_t3__blk101 = assign6680_e4425;
        locals.var_t3__blk101_dn0 = (-(locals.var_c_fox_dn0 / (assign6680_e4424 * assign6680_e4424)));
        locals.var_t3__blk101_dn2 = (-(locals.var_c_fox_dn2 / (assign6680_e4424 * assign6680_e4424)));
        locals.var_t3__blk101_dn6 = (-(locals.var_c_fox_dn6 / (assign6680_e4424 * assign6680_e4424)));
        locals.var_t3__blk101_dn7 = (-(locals.var_c_fox_dn7 / (assign6680_e4424 * assign6680_e4424)));
        locals.var_t3__blk101_dn10 = (-(locals.var_c_fox_dn10 / (assign6680_e4424 * assign6680_e4424)));
        locals.var_t3__blk101_dn11 = (-(locals.var_c_fox_dn11 / (assign6680_e4424 * assign6680_e4424)));
        locals.var_t3__blk101_dn12 = (-(locals.var_c_fox_dn12 / (assign6680_e4424 * assign6680_e4424)));
        locals.var_t3__blk101_dn17 = (-(locals.var_c_fox_dn17 / (assign6680_e4424 * assign6680_e4424)));

        let assign6690_e4428: f64 = (locals.var_t1__blk100 - locals.var_t3__blk101);
        locals.var_t5__blk102 = assign6690_e4428;
        locals.var_t5__blk102_dn0 = (locals.var_t1__blk100_dn0 - locals.var_t3__blk101_dn0);
        locals.var_t5__blk102_dn2 = (locals.var_t1__blk100_dn2 - locals.var_t3__blk101_dn2);
        locals.var_t5__blk102_dn6 = (locals.var_t1__blk100_dn6 - locals.var_t3__blk101_dn6);
        locals.var_t5__blk102_dn7 = (locals.var_t1__blk100_dn7 - locals.var_t3__blk101_dn7);
        locals.var_t5__blk102_dn10 = (locals.var_t1__blk100_dn10 - locals.var_t3__blk101_dn10);
        locals.var_t5__blk102_dn11 = (locals.var_t1__blk100_dn11 - locals.var_t3__blk101_dn11);
        locals.var_t5__blk102_dn12 = (locals.var_t1__blk100_dn12 - locals.var_t3__blk101_dn12);
        locals.var_t5__blk102_dn17 = (locals.var_t1__blk100_dn17 - locals.var_t3__blk101_dn17);

        let assign6700_e4431: f64 = (locals.var_qb0 * locals.var_t5__blk102);
        let assign6700_e4434: f64 = (p.p105 / locals.var_wg);
        let assign6700_e4435: f64 = (assign6700_e4431 + assign6700_e4434);
        locals.var_dvthw = assign6700_e4435;
        locals.var_dvthw_dn0 = ((locals.var_qb0_dn0 * locals.var_t5__blk102) + (locals.var_qb0 * locals.var_t5__blk102_dn0));
        locals.var_dvthw_dn2 = ((locals.var_qb0_dn2 * locals.var_t5__blk102) + (locals.var_qb0 * locals.var_t5__blk102_dn2));
        locals.var_dvthw_dn6 = ((locals.var_qb0_dn6 * locals.var_t5__blk102) + (locals.var_qb0 * locals.var_t5__blk102_dn6));
        locals.var_dvthw_dn7 = ((locals.var_qb0_dn7 * locals.var_t5__blk102) + (locals.var_qb0 * locals.var_t5__blk102_dn7));
        locals.var_dvthw_dn10 = ((locals.var_qb0_dn10 * locals.var_t5__blk102) + (locals.var_qb0 * locals.var_t5__blk102_dn10));
        locals.var_dvthw_dn11 = ((locals.var_qb0_dn11 * locals.var_t5__blk102) + (locals.var_qb0 * locals.var_t5__blk102_dn11));
        locals.var_dvthw_dn12 = ((locals.var_qb0_dn12 * locals.var_t5__blk102) + (locals.var_qb0 * locals.var_t5__blk102_dn12));
        locals.var_dvthw_dn17 = ((locals.var_qb0_dn17 * locals.var_t5__blk102) + (locals.var_qb0 * locals.var_t5__blk102_dn17));

        let assign6710_e4438: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign6710_e4440: f64 = (assign6710_e4438 + locals.var_dvthw);
        let assign6710_e4442: f64 = (assign6710_e4440 + locals.var_dvthscr);
        let assign6710_e4444: f64 = (assign6710_e4442 + locals.var_dvthsm);
        locals.var_dvth = assign6710_e4444;
        locals.var_dvth_dn0 = (((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) + locals.var_dvthw_dn0) + locals.var_dvthscr_dn0);
        locals.var_dvth_dn2 = (((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) + locals.var_dvthw_dn2) + locals.var_dvthscr_dn2);
        locals.var_dvth_dn6 = (((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) + locals.var_dvthw_dn6) + locals.var_dvthscr_dn6);
        locals.var_dvth_dn7 = (((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) + locals.var_dvthw_dn7) + locals.var_dvthscr_dn7);
        locals.var_dvth_dn10 = (((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) + locals.var_dvthw_dn10) + locals.var_dvthscr_dn10);
        locals.var_dvth_dn11 = (((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) + locals.var_dvthw_dn11) + locals.var_dvthscr_dn11);
        locals.var_dvth_dn12 = (((locals.var_dvthsc_dn12 + locals.var_dvthlp_dn12) + locals.var_dvthw_dn12) + locals.var_dvthscr_dn12);
        locals.var_dvth_dn17 = (((locals.var_dvthsc_dn17 + locals.var_dvthlp_dn17) + locals.var_dvthw_dn17) + locals.var_dvthscr_dn17);

        let assign6720_e4447: f64 = (locals.var_vthp - locals.var_dvth);
        locals.var_vth = assign6720_e4447;

        let assign6730_e4450: f64 = if p.p89 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign6730_e4450;

        let (assign6740_e4454,) = {
    if (locals.var_guard106 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_dppg,)
    }
};
        locals.var_flg_dppg = assign6740_e4454;

        let (assign6750_e4459,) = {
    if (locals.var_guard106 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_dppg,)
    }
};
        locals.var_flg_dppg = assign6750_e4459;

        let assign6760_e4462: f64 = if locals.var_flg_dppg == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign6760_e4462;

        let (assign6770_e4466, assign6770_e4466_d_n0, assign6770_e4466_d_n2, assign6770_e4466_d_n6, assign6770_e4466_d_n7, assign6770_e4466_d_n10, assign6770_e4466_d_n11, assign6770_e4466_d_n12, assign6770_e4466_d_n17,) = {
    if (locals.var_guard107 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6770_e4466;
        locals.var_dppg_dn0 = assign6770_e4466_d_n0;
        locals.var_dppg_dn2 = assign6770_e4466_d_n2;
        locals.var_dppg_dn6 = assign6770_e4466_d_n6;
        locals.var_dppg_dn7 = assign6770_e4466_d_n7;
        locals.var_dppg_dn10 = assign6770_e4466_d_n10;
        locals.var_dppg_dn11 = assign6770_e4466_d_n11;
        locals.var_dppg_dn12 = assign6770_e4466_d_n12;
        locals.var_dppg_dn17 = assign6770_e4466_d_n17;

        let (assign6780_e4471, assign6780_e4471_d_n0, assign6780_e4471_d_n2, assign6780_e4471_d_n6, assign6780_e4471_d_n7, assign6780_e4471_d_n10, assign6780_e4471_d_n11, assign6780_e4471_d_n12, assign6780_e4471_d_n17,) = {
    if (locals.var_guard107 == 0.0) {
        (locals.var_vgsz, locals.var_vgsz_dn0, locals.var_vgsz_dn2, locals.var_vgsz_dn6, locals.var_vgsz_dn7, locals.var_vgsz_dn10, locals.var_vgsz_dn11, locals.var_vgsz_dn12, locals.var_vgsz_dn17,)
    } else {
        (locals.var_t7__blk103, locals.var_t7__blk103_dn0, locals.var_t7__blk103_dn2, locals.var_t7__blk103_dn6, locals.var_t7__blk103_dn7, locals.var_t7__blk103_dn10, locals.var_t7__blk103_dn11, locals.var_t7__blk103_dn12, locals.var_t7__blk103_dn17,)
    }
};
        locals.var_t7__blk103 = assign6780_e4471;
        locals.var_t7__blk103_dn0 = assign6780_e4471_d_n0;
        locals.var_t7__blk103_dn2 = assign6780_e4471_d_n2;
        locals.var_t7__blk103_dn6 = assign6780_e4471_d_n6;
        locals.var_t7__blk103_dn7 = assign6780_e4471_d_n7;
        locals.var_t7__blk103_dn10 = assign6780_e4471_d_n10;
        locals.var_t7__blk103_dn11 = assign6780_e4471_d_n11;
        locals.var_t7__blk103_dn12 = assign6780_e4471_d_n12;
        locals.var_t7__blk103_dn17 = assign6780_e4471_d_n17;

        let (assign6790_e4476,) = {
    if (locals.var_guard107 == 0.0) {
        (locals.var_cnstpgd,)
    } else {
        (locals.var_t0__blk104,)
    }
};
        locals.var_t0__blk104 = assign6790_e4476;

        let (assign6800_e4483, assign6800_e4483_d_n0, assign6800_e4483_d_n2, assign6800_e4483_d_n6, assign6800_e4483_d_n7, assign6800_e4483_d_n10, assign6800_e4483_d_n11, assign6800_e4483_d_n12, assign6800_e4483_d_n17,) = {
    if (locals.var_guard107 == 0.0) {
        let assign6800_e4481: f64 = (locals.var_t7__blk103 - p.p90);
        (assign6800_e4481, locals.var_t7__blk103_dn0, locals.var_t7__blk103_dn2, locals.var_t7__blk103_dn6, locals.var_t7__blk103_dn7, locals.var_t7__blk103_dn10, locals.var_t7__blk103_dn11, locals.var_t7__blk103_dn12, locals.var_t7__blk103_dn17,)
    } else {
        (locals.var_t3__blk105, locals.var_t3__blk105_dn0, locals.var_t3__blk105_dn2, locals.var_t3__blk105_dn6, locals.var_t3__blk105_dn7, locals.var_t3__blk105_dn10, locals.var_t3__blk105_dn11, locals.var_t3__blk105_dn12, locals.var_t3__blk105_dn17,)
    }
};
        locals.var_t3__blk105 = assign6800_e4483;
        locals.var_t3__blk105_dn0 = assign6800_e4483_d_n0;
        locals.var_t3__blk105_dn2 = assign6800_e4483_d_n2;
        locals.var_t3__blk105_dn6 = assign6800_e4483_d_n6;
        locals.var_t3__blk105_dn7 = assign6800_e4483_d_n7;
        locals.var_t3__blk105_dn10 = assign6800_e4483_d_n10;
        locals.var_t3__blk105_dn11 = assign6800_e4483_d_n11;
        locals.var_t3__blk105_dn12 = assign6800_e4483_d_n12;
        locals.var_t3__blk105_dn17 = assign6800_e4483_d_n17;

        let assign6810_e4486: f64 = (-3.0);
        let assign6810_e4487: f64 = if locals.var_t3__blk105 < assign6810_e4486 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign6810_e4487;

        let (assign6820_e4494, assign6820_e4494_d_n0, assign6820_e4494_d_n2, assign6820_e4494_d_n6, assign6820_e4494_d_n7, assign6820_e4494_d_n10, assign6820_e4494_d_n11, assign6820_e4494_d_n12, assign6820_e4494_d_n17,) = {
    if ((locals.var_guard107 == 0.0) && (locals.var_guard108 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6820_e4494;
        locals.var_dppg_dn0 = assign6820_e4494_d_n0;
        locals.var_dppg_dn2 = assign6820_e4494_d_n2;
        locals.var_dppg_dn6 = assign6820_e4494_d_n6;
        locals.var_dppg_dn7 = assign6820_e4494_d_n7;
        locals.var_dppg_dn10 = assign6820_e4494_d_n10;
        locals.var_dppg_dn11 = assign6820_e4494_d_n11;
        locals.var_dppg_dn12 = assign6820_e4494_d_n12;
        locals.var_dppg_dn17 = assign6820_e4494_d_n17;

        let assign6830_e4497: f64 = if locals.var_t3__blk105 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign6830_e4497;

        let (assign6840_e4523, assign6840_e4523_d_n0, assign6840_e4523_d_n2, assign6840_e4523_d_n6, assign6840_e4523_d_n7, assign6840_e4523_d_n10, assign6840_e4523_d_n11, assign6840_e4523_d_n12, assign6840_e4523_d_n17,) = {
    if (((locals.var_guard107 == 0.0) && (locals.var_guard108 == 0.0)) && (locals.var_guard109 != 0.0)) {
        let assign6840_e4511: f64 = (1.0 / 3.0);
        let assign6840_e4515: f64 = (1.0 / 27.0);
        let assign6840_e4516: f64 = (locals.var_t3__blk105 * assign6840_e4515);
        let assign6840_e4517: f64 = (assign6840_e4511 + assign6840_e4516);
        let assign6840_e4518: f64 = (locals.var_t3__blk105 * assign6840_e4517);
        let assign6840_e4519: f64 = (1.0 + assign6840_e4518);
        let assign6840_e4520: f64 = (locals.var_t3__blk105 * assign6840_e4519);
        let assign6840_e4521: f64 = (1.0 + assign6840_e4520);
        (assign6840_e4521, ((locals.var_t3__blk105_dn0 * assign6840_e4519) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn0 * assign6840_e4517) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn0 * assign6840_e4515))))), ((locals.var_t3__blk105_dn2 * assign6840_e4519) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn2 * assign6840_e4517) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn2 * assign6840_e4515))))), ((locals.var_t3__blk105_dn6 * assign6840_e4519) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn6 * assign6840_e4517) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn6 * assign6840_e4515))))), ((locals.var_t3__blk105_dn7 * assign6840_e4519) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn7 * assign6840_e4517) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn7 * assign6840_e4515))))), ((locals.var_t3__blk105_dn10 * assign6840_e4519) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn10 * assign6840_e4517) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn10 * assign6840_e4515))))), ((locals.var_t3__blk105_dn11 * assign6840_e4519) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn11 * assign6840_e4517) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn11 * assign6840_e4515))))), ((locals.var_t3__blk105_dn12 * assign6840_e4519) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn12 * assign6840_e4517) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn12 * assign6840_e4515))))), ((locals.var_t3__blk105_dn17 * assign6840_e4519) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn17 * assign6840_e4517) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn17 * assign6840_e4515))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6840_e4523;
        locals.var_dppg_dn0 = assign6840_e4523_d_n0;
        locals.var_dppg_dn2 = assign6840_e4523_d_n2;
        locals.var_dppg_dn6 = assign6840_e4523_d_n6;
        locals.var_dppg_dn7 = assign6840_e4523_d_n7;
        locals.var_dppg_dn10 = assign6840_e4523_d_n10;
        locals.var_dppg_dn11 = assign6840_e4523_d_n11;
        locals.var_dppg_dn12 = assign6840_e4523_d_n12;
        locals.var_dppg_dn17 = assign6840_e4523_d_n17;

        let (assign6850_e4552, assign6850_e4552_d_n0, assign6850_e4552_d_n2, assign6850_e4552_d_n6, assign6850_e4552_d_n7, assign6850_e4552_d_n10, assign6850_e4552_d_n11, assign6850_e4552_d_n12, assign6850_e4552_d_n17,) = {
    if (((locals.var_guard107 == 0.0) && (locals.var_guard108 == 0.0)) && (locals.var_guard109 == 0.0)) {
        let assign6850_e4538: f64 = (1.0 / 3.0);
        let assign6850_e4543: f64 = (locals.var_t3__blk105 * 0.148148111111111);
        let assign6850_e4544: f64 = (0.0402052934513951 + assign6850_e4543);
        let assign6850_e4545: f64 = (locals.var_t3__blk105 * assign6850_e4544);
        let assign6850_e4546: f64 = (assign6850_e4538 + assign6850_e4545);
        let assign6850_e4547: f64 = (locals.var_t3__blk105 * assign6850_e4546);
        let assign6850_e4548: f64 = (1.0 + assign6850_e4547);
        let assign6850_e4549: f64 = (locals.var_t3__blk105 * assign6850_e4548);
        let assign6850_e4550: f64 = (1.0 + assign6850_e4549);
        (assign6850_e4550, ((locals.var_t3__blk105_dn0 * assign6850_e4548) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn0 * assign6850_e4546) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn0 * assign6850_e4544) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn0 * 0.148148111111111))))))), ((locals.var_t3__blk105_dn2 * assign6850_e4548) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn2 * assign6850_e4546) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn2 * assign6850_e4544) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn2 * 0.148148111111111))))))), ((locals.var_t3__blk105_dn6 * assign6850_e4548) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn6 * assign6850_e4546) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn6 * assign6850_e4544) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn6 * 0.148148111111111))))))), ((locals.var_t3__blk105_dn7 * assign6850_e4548) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn7 * assign6850_e4546) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn7 * assign6850_e4544) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn7 * 0.148148111111111))))))), ((locals.var_t3__blk105_dn10 * assign6850_e4548) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn10 * assign6850_e4546) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn10 * assign6850_e4544) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn10 * 0.148148111111111))))))), ((locals.var_t3__blk105_dn11 * assign6850_e4548) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn11 * assign6850_e4546) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn11 * assign6850_e4544) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn11 * 0.148148111111111))))))), ((locals.var_t3__blk105_dn12 * assign6850_e4548) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn12 * assign6850_e4546) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn12 * assign6850_e4544) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn12 * 0.148148111111111))))))), ((locals.var_t3__blk105_dn17 * assign6850_e4548) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn17 * assign6850_e4546) + (locals.var_t3__blk105 * ((locals.var_t3__blk105_dn17 * assign6850_e4544) + (locals.var_t3__blk105 * (locals.var_t3__blk105_dn17 * 0.148148111111111))))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6850_e4552;
        locals.var_dppg_dn0 = assign6850_e4552_d_n0;
        locals.var_dppg_dn2 = assign6850_e4552_d_n2;
        locals.var_dppg_dn6 = assign6850_e4552_d_n6;
        locals.var_dppg_dn7 = assign6850_e4552_d_n7;
        locals.var_dppg_dn10 = assign6850_e4552_d_n10;
        locals.var_dppg_dn11 = assign6850_e4552_d_n11;
        locals.var_dppg_dn12 = assign6850_e4552_d_n12;
        locals.var_dppg_dn17 = assign6850_e4552_d_n17;

        let (assign6860_e4570, assign6860_e4570_d_n0, assign6860_e4570_d_n2, assign6860_e4570_d_n6, assign6860_e4570_d_n7, assign6860_e4570_d_n10, assign6860_e4570_d_n11, assign6860_e4570_d_n12, assign6860_e4570_d_n17,) = {
    if (locals.var_guard107 == 0.0) {
        let assign6860_e4557: f64 = (locals.var_dppg - 1.0);
        let assign6860_e4560: f64 = (locals.var_dppg - 1.0);
        let assign6860_e4561: f64 = (assign6860_e4557 * assign6860_e4560);
        let assign6860_e4564: f64 = (4.0 * 0.1);
        let assign6860_e4566: f64 = (assign6860_e4564 * 0.1);
        let assign6860_e4567: f64 = (assign6860_e4561 + assign6860_e4566);
        let assign6860_e4568: f64 = (assign6860_e4567).sqrt();
        (assign6860_e4568, (((locals.var_dppg_dn0 * assign6860_e4560) + (assign6860_e4557 * locals.var_dppg_dn0)) / (2.0 * assign6860_e4568)), (((locals.var_dppg_dn2 * assign6860_e4560) + (assign6860_e4557 * locals.var_dppg_dn2)) / (2.0 * assign6860_e4568)), (((locals.var_dppg_dn6 * assign6860_e4560) + (assign6860_e4557 * locals.var_dppg_dn6)) / (2.0 * assign6860_e4568)), (((locals.var_dppg_dn7 * assign6860_e4560) + (assign6860_e4557 * locals.var_dppg_dn7)) / (2.0 * assign6860_e4568)), (((locals.var_dppg_dn10 * assign6860_e4560) + (assign6860_e4557 * locals.var_dppg_dn10)) / (2.0 * assign6860_e4568)), (((locals.var_dppg_dn11 * assign6860_e4560) + (assign6860_e4557 * locals.var_dppg_dn11)) / (2.0 * assign6860_e4568)), (((locals.var_dppg_dn12 * assign6860_e4560) + (assign6860_e4557 * locals.var_dppg_dn12)) / (2.0 * assign6860_e4568)), (((locals.var_dppg_dn17 * assign6860_e4560) + (assign6860_e4557 * locals.var_dppg_dn17)) / (2.0 * assign6860_e4568)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign6860_e4570;
        locals.var_tmf1_dn0 = assign6860_e4570_d_n0;
        locals.var_tmf1_dn2 = assign6860_e4570_d_n2;
        locals.var_tmf1_dn6 = assign6860_e4570_d_n6;
        locals.var_tmf1_dn7 = assign6860_e4570_d_n7;
        locals.var_tmf1_dn10 = assign6860_e4570_d_n10;
        locals.var_tmf1_dn11 = assign6860_e4570_d_n11;
        locals.var_tmf1_dn12 = assign6860_e4570_d_n12;
        locals.var_tmf1_dn17 = assign6860_e4570_d_n17;

    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6870_e4585, assign6870_e4585_d_n0, assign6870_e4585_d_n2, assign6870_e4585_d_n6, assign6870_e4585_d_n7, assign6870_e4585_d_n10, assign6870_e4585_d_n11, assign6870_e4585_d_n12, assign6870_e4585_d_n17,) = {
    if (locals.var_guard107 == 0.0) {
        let assign6870_e4576: f64 = (locals.var_dppg - 1.0);
        let assign6870_e4578: f64 = (assign6870_e4576 + locals.var_tmf1);
        let assign6870_e4579: f64 = (0.5 * assign6870_e4578);
        let assign6870_e4582: f64 = (1e-10 * 0.1);
        let assign6870_e4583: f64 = (assign6870_e4579 + assign6870_e4582);
        (assign6870_e4583, (0.5 * (locals.var_dppg_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_dppg_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_dppg_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_dppg_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_dppg_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_dppg_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_dppg_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_dppg_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6870_e4585;
        locals.var_dppg_dn0 = assign6870_e4585_d_n0;
        locals.var_dppg_dn2 = assign6870_e4585_d_n2;
        locals.var_dppg_dn6 = assign6870_e4585_d_n6;
        locals.var_dppg_dn7 = assign6870_e4585_d_n7;
        locals.var_dppg_dn10 = assign6870_e4585_d_n10;
        locals.var_dppg_dn11 = assign6870_e4585_d_n11;
        locals.var_dppg_dn12 = assign6870_e4585_d_n12;
        locals.var_dppg_dn17 = assign6870_e4585_d_n17;

        let assign6880_e4588: f64 = if locals.var_dppg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign6880_e4588;

        let (assign6890_e4595, assign6890_e4595_d_n0, assign6890_e4595_d_n2, assign6890_e4595_d_n6, assign6890_e4595_d_n7, assign6890_e4595_d_n10, assign6890_e4595_d_n11, assign6890_e4595_d_n12, assign6890_e4595_d_n17,) = {
    if ((locals.var_guard107 == 0.0) && (locals.var_guard110 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6890_e4595;
        locals.var_dppg_dn0 = assign6890_e4595_d_n0;
        locals.var_dppg_dn2 = assign6890_e4595_d_n2;
        locals.var_dppg_dn6 = assign6890_e4595_d_n6;
        locals.var_dppg_dn7 = assign6890_e4595_d_n7;
        locals.var_dppg_dn10 = assign6890_e4595_d_n10;
        locals.var_dppg_dn11 = assign6890_e4595_d_n11;
        locals.var_dppg_dn12 = assign6890_e4595_d_n12;
        locals.var_dppg_dn17 = assign6890_e4595_d_n17;

        let (assign6900_e4602, assign6900_e4602_d_n0, assign6900_e4602_d_n2, assign6900_e4602_d_n6, assign6900_e4602_d_n7, assign6900_e4602_d_n10, assign6900_e4602_d_n11, assign6900_e4602_d_n12, assign6900_e4602_d_n17,) = {
    if (locals.var_guard107 == 0.0) {
        let assign6900_e4600: f64 = (locals.var_dppg * locals.var_t0__blk104);
        (assign6900_e4600, (locals.var_dppg_dn0 * locals.var_t0__blk104), (locals.var_dppg_dn2 * locals.var_t0__blk104), (locals.var_dppg_dn6 * locals.var_t0__blk104), (locals.var_dppg_dn7 * locals.var_t0__blk104), (locals.var_dppg_dn10 * locals.var_t0__blk104), (locals.var_dppg_dn11 * locals.var_t0__blk104), (locals.var_dppg_dn12 * locals.var_t0__blk104), (locals.var_dppg_dn17 * locals.var_t0__blk104),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6900_e4602;
        locals.var_dppg_dn0 = assign6900_e4602_d_n0;
        locals.var_dppg_dn2 = assign6900_e4602_d_n2;
        locals.var_dppg_dn6 = assign6900_e4602_d_n6;
        locals.var_dppg_dn7 = assign6900_e4602_d_n7;
        locals.var_dppg_dn10 = assign6900_e4602_d_n10;
        locals.var_dppg_dn11 = assign6900_e4602_d_n11;
        locals.var_dppg_dn12 = assign6900_e4602_d_n12;
        locals.var_dppg_dn17 = assign6900_e4602_d_n17;

        let (assign6910_e4611, assign6910_e4611_d_n0, assign6910_e4611_d_n2, assign6910_e4611_d_n6, assign6910_e4611_d_n7, assign6910_e4611_d_n10, assign6910_e4611_d_n11, assign6910_e4611_d_n12, assign6910_e4611_d_n17,) = {
    if (locals.var_guard107 == 0.0) {
        let assign6910_e4607: f64 = (1.0 - locals.var_dppg);
        let assign6910_e4609: f64 = (assign6910_e4607 - 0.05);
        (assign6910_e4609, (-locals.var_dppg_dn0), (-locals.var_dppg_dn2), (-locals.var_dppg_dn6), (-locals.var_dppg_dn7), (-locals.var_dppg_dn10), (-locals.var_dppg_dn11), (-locals.var_dppg_dn12), (-locals.var_dppg_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign6910_e4611;
        locals.var_tmf1_dn0 = assign6910_e4611_d_n0;
        locals.var_tmf1_dn2 = assign6910_e4611_d_n2;
        locals.var_tmf1_dn6 = assign6910_e4611_d_n6;
        locals.var_tmf1_dn7 = assign6910_e4611_d_n7;
        locals.var_tmf1_dn10 = assign6910_e4611_d_n10;
        locals.var_tmf1_dn11 = assign6910_e4611_d_n11;
        locals.var_tmf1_dn12 = assign6910_e4611_d_n12;
        locals.var_tmf1_dn17 = assign6910_e4611_d_n17;

        let (assign6920_e4620, assign6920_e4620_d_n0, assign6920_e4620_d_n2, assign6920_e4620_d_n6, assign6920_e4620_d_n7, assign6920_e4620_d_n10, assign6920_e4620_d_n11, assign6920_e4620_d_n12, assign6920_e4620_d_n17,) = {
    if (locals.var_guard107 == 0.0) {
        let assign6920_e4616: f64 = 4.0;
        let assign6920_e4618: f64 = (assign6920_e4616 * 0.05);
        (assign6920_e4618, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6920_e4620;
        locals.var_tmf2_dn0 = assign6920_e4620_d_n0;
        locals.var_tmf2_dn2 = assign6920_e4620_d_n2;
        locals.var_tmf2_dn6 = assign6920_e4620_d_n6;
        locals.var_tmf2_dn7 = assign6920_e4620_d_n7;
        locals.var_tmf2_dn10 = assign6920_e4620_d_n10;
        locals.var_tmf2_dn11 = assign6920_e4620_d_n11;
        locals.var_tmf2_dn12 = assign6920_e4620_d_n12;
        locals.var_tmf2_dn17 = assign6920_e4620_d_n17;

        let (assign6930_e4631, assign6930_e4631_d_n0, assign6930_e4631_d_n2, assign6930_e4631_d_n6, assign6930_e4631_d_n7, assign6930_e4631_d_n10, assign6930_e4631_d_n11, assign6930_e4631_d_n12, assign6930_e4631_d_n17,) = {
    if (locals.var_guard107 == 0.0) {
        let (assign6930_e4629, assign6930_e4629_d_n0, assign6930_e4629_d_n2, assign6930_e4629_d_n6, assign6930_e4629_d_n7, assign6930_e4629_d_n10, assign6930_e4629_d_n11, assign6930_e4629_d_n12, assign6930_e4629_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign6930_e4628: f64 = (-locals.var_tmf2);
                (assign6930_e4628, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign6930_e4629, assign6930_e4629_d_n0, assign6930_e4629_d_n2, assign6930_e4629_d_n6, assign6930_e4629_d_n7, assign6930_e4629_d_n10, assign6930_e4629_d_n11, assign6930_e4629_d_n12, assign6930_e4629_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6930_e4631;
        locals.var_tmf2_dn0 = assign6930_e4631_d_n0;
        locals.var_tmf2_dn2 = assign6930_e4631_d_n2;
        locals.var_tmf2_dn6 = assign6930_e4631_d_n6;
        locals.var_tmf2_dn7 = assign6930_e4631_d_n7;
        locals.var_tmf2_dn10 = assign6930_e4631_d_n10;
        locals.var_tmf2_dn11 = assign6930_e4631_d_n11;
        locals.var_tmf2_dn12 = assign6930_e4631_d_n12;
        locals.var_tmf2_dn17 = assign6930_e4631_d_n17;

        let (assign6940_e4641, assign6940_e4641_d_n0, assign6940_e4641_d_n2, assign6940_e4641_d_n6, assign6940_e4641_d_n7, assign6940_e4641_d_n10, assign6940_e4641_d_n11, assign6940_e4641_d_n12, assign6940_e4641_d_n17,) = {
    if (locals.var_guard107 == 0.0) {
        let assign6940_e4636: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign6940_e4638: f64 = (assign6940_e4636 + locals.var_tmf2);
        let assign6940_e4639: f64 = (assign6940_e4638).sqrt();
        (assign6940_e4639, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign6940_e4639)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign6940_e4639)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign6940_e4639)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign6940_e4639)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign6940_e4639)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign6940_e4639)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign6940_e4639)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign6940_e4639)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6940_e4641;
        locals.var_tmf2_dn0 = assign6940_e4641_d_n0;
        locals.var_tmf2_dn2 = assign6940_e4641_d_n2;
        locals.var_tmf2_dn6 = assign6940_e4641_d_n6;
        locals.var_tmf2_dn7 = assign6940_e4641_d_n7;
        locals.var_tmf2_dn10 = assign6940_e4641_d_n10;
        locals.var_tmf2_dn11 = assign6940_e4641_d_n11;
        locals.var_tmf2_dn12 = assign6940_e4641_d_n12;
        locals.var_tmf2_dn17 = assign6940_e4641_d_n17;

        let (assign6950_e4652, assign6950_e4652_d_n0, assign6950_e4652_d_n2, assign6950_e4652_d_n6, assign6950_e4652_d_n7, assign6950_e4652_d_n10, assign6950_e4652_d_n11, assign6950_e4652_d_n12, assign6950_e4652_d_n17,) = {
    if (locals.var_guard107 == 0.0) {
        let assign6950_e4648: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign6950_e4649: f64 = (0.5 * assign6950_e4648);
        let assign6950_e4650: f64 = (1.0 - assign6950_e4649);
        (assign6950_e4650, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (-(0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6950_e4652;
        locals.var_dppg_dn0 = assign6950_e4652_d_n0;
        locals.var_dppg_dn2 = assign6950_e4652_d_n2;
        locals.var_dppg_dn6 = assign6950_e4652_d_n6;
        locals.var_dppg_dn7 = assign6950_e4652_d_n7;
        locals.var_dppg_dn10 = assign6950_e4652_d_n10;
        locals.var_dppg_dn11 = assign6950_e4652_d_n11;
        locals.var_dppg_dn12 = assign6950_e4652_d_n12;
        locals.var_dppg_dn17 = assign6950_e4652_d_n17;

        let assign6960_e4655: f64 = (locals.var_vgs - locals.var_vfb);
        let assign6960_e4657: f64 = (assign6960_e4655 + locals.var_dvth);
        let assign6960_e4659: f64 = (assign6960_e4657 - locals.var_dppg);
        locals.var_vgp = assign6960_e4659;
        locals.var_vgp_dn0 = (locals.var_dvth_dn0 - locals.var_dppg_dn0);
        locals.var_vgp_dn2 = (locals.var_dvth_dn2 - locals.var_dppg_dn2);
        locals.var_vgp_dn6 = ((locals.var_vgs_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6);
        locals.var_vgp_dn7 = ((locals.var_vgs_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7);
        locals.var_vgp_dn10 = (locals.var_dvth_dn10 - locals.var_dppg_dn10);
        locals.var_vgp_dn11 = ((locals.var_vgs_dn11 + locals.var_dvth_dn11) - locals.var_dppg_dn11);
        locals.var_vgp_dn12 = (locals.var_dvth_dn12 - locals.var_dppg_dn12);
        locals.var_vgp_dn17 = (locals.var_dvth_dn17 - locals.var_dppg_dn17);

        locals.var_vgpz = locals.var_vgp;
        locals.var_vgpz_dn0 = locals.var_vgp_dn0;
        locals.var_vgpz_dn2 = locals.var_vgp_dn2;
        locals.var_vgpz_dn6 = locals.var_vgp_dn6;
        locals.var_vgpz_dn7 = locals.var_vgp_dn7;
        locals.var_vgpz_dn10 = locals.var_vgp_dn10;
        locals.var_vgpz_dn11 = locals.var_vgp_dn11;
        locals.var_vgpz_dn12 = locals.var_vgp_dn12;
        locals.var_vgpz_dn17 = locals.var_vgp_dn17;

        let assign6980_e4663: f64 = (locals.var_uc_nsubs / locals.var_mks_nsubb);
        let assign6980_e4664: f64 = (assign6980_e4663).ln();
        locals.var_t1 = assign6980_e4664;
        locals.var_t1_dn0 = ((locals.var_uc_nsubs_dn0 / locals.var_mks_nsubb) / assign6980_e4663);
        locals.var_t1_dn2 = ((locals.var_uc_nsubs_dn2 / locals.var_mks_nsubb) / assign6980_e4663);
        locals.var_t1_dn6 = ((locals.var_uc_nsubs_dn6 / locals.var_mks_nsubb) / assign6980_e4663);
        locals.var_t1_dn7 = ((locals.var_uc_nsubs_dn7 / locals.var_mks_nsubb) / assign6980_e4663);
        locals.var_t1_dn10 = ((locals.var_uc_nsubs_dn10 / locals.var_mks_nsubb) / assign6980_e4663);
        locals.var_t1_dn11 = ((locals.var_uc_nsubs_dn11 / locals.var_mks_nsubb) / assign6980_e4663);
        locals.var_t1_dn12 = ((locals.var_uc_nsubs_dn12 / locals.var_mks_nsubb) / assign6980_e4663);
        locals.var_t1_dn17 = ((locals.var_uc_nsubs_dn17 / locals.var_mks_nsubb) / assign6980_e4663);

        let assign6990_e4667: f64 = (locals.var_beta_inv * locals.var_t1);
        locals.var_vbi_soi = assign6990_e4667;
        locals.var_vbi_soi_dn0 = (locals.var_beta_inv * locals.var_t1_dn0);
        locals.var_vbi_soi_dn2 = (locals.var_beta_inv * locals.var_t1_dn2);
        locals.var_vbi_soi_dn6 = (locals.var_beta_inv * locals.var_t1_dn6);
        locals.var_vbi_soi_dn7 = (locals.var_beta_inv * locals.var_t1_dn7);
        locals.var_vbi_soi_dn10 = ((locals.var_beta_inv_dn10 * locals.var_t1) + (locals.var_beta_inv * locals.var_t1_dn10));
        locals.var_vbi_soi_dn11 = (locals.var_beta_inv * locals.var_t1_dn11);
        locals.var_vbi_soi_dn12 = (locals.var_beta_inv * locals.var_t1_dn12);
        locals.var_vbi_soi_dn17 = (locals.var_beta_inv * locals.var_t1_dn17);

        let assign7000_e4670: f64 = (locals.var_vfb - locals.var_dvth);
        let assign7000_e4672: f64 = (assign7000_e4670 + locals.var_dppg);
        locals.var_vgs_fb = assign7000_e4672;

        let assign7010_e4675: f64 = (locals.var_cnst0soi * locals.var_c_fox_inv);
        locals.var_fac1 = assign7010_e4675;
        locals.var_fac1_dn0 = ((locals.var_cnst0soi_dn0 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn0));
        locals.var_fac1_dn2 = ((locals.var_cnst0soi_dn2 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn2));
        locals.var_fac1_dn6 = ((locals.var_cnst0soi_dn6 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn6));
        locals.var_fac1_dn7 = ((locals.var_cnst0soi_dn7 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn7));
        locals.var_fac1_dn10 = ((locals.var_cnst0soi_dn10 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn10));
        locals.var_fac1_dn11 = ((locals.var_cnst0soi_dn11 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn11));
        locals.var_fac1_dn12 = ((locals.var_cnst0soi_dn12 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn12));
        locals.var_fac1_dn17 = ((locals.var_cnst0soi_dn17 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn17));

        let assign7020_e4678: f64 = (locals.var_fac1 * locals.var_fac1);
        locals.var_fac1p2 = assign7020_e4678;
        locals.var_fac1p2_dn0 = ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0));
        locals.var_fac1p2_dn2 = ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2));
        locals.var_fac1p2_dn6 = ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6));
        locals.var_fac1p2_dn7 = ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7));
        locals.var_fac1p2_dn10 = ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10));
        locals.var_fac1p2_dn11 = ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11));
        locals.var_fac1p2_dn12 = ((locals.var_fac1_dn12 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn12));
        locals.var_fac1p2_dn17 = ((locals.var_fac1_dn17 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn17));

        let assign7030_e4681: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign7030_e4681;

        let (assign7040_e4685,) = {
    if (locals.var_guard111 != 0.0) {
        (7.0,)
    } else {
        (locals.var_qdepb_dlt,)
    }
};
        locals.var_qdepb_dlt = assign7040_e4685;

        let (assign7050_e4691, assign7050_e4691_d_n0, assign7050_e4691_d_n2, assign7050_e4691_d_n6, assign7050_e4691_d_n7, assign7050_e4691_d_n10, assign7050_e4691_d_n11, assign7050_e4691_d_n12, assign7050_e4691_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7050_e4689: f64 = (locals.var_pb2 + 1.0);
        (assign7050_e4689, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn10, locals.var_pb2_dn11, locals.var_pb2_dn12, locals.var_pb2_dn17,)
    } else {
        (locals.var_vgp_ini, locals.var_vgp_ini_dn0, locals.var_vgp_ini_dn2, locals.var_vgp_ini_dn6, locals.var_vgp_ini_dn7, locals.var_vgp_ini_dn10, locals.var_vgp_ini_dn11, locals.var_vgp_ini_dn12, locals.var_vgp_ini_dn17,)
    }
};
        locals.var_vgp_ini = assign7050_e4691;
        locals.var_vgp_ini_dn0 = assign7050_e4691_d_n0;
        locals.var_vgp_ini_dn2 = assign7050_e4691_d_n2;
        locals.var_vgp_ini_dn6 = assign7050_e4691_d_n6;
        locals.var_vgp_ini_dn7 = assign7050_e4691_d_n7;
        locals.var_vgp_ini_dn10 = assign7050_e4691_d_n10;
        locals.var_vgp_ini_dn11 = assign7050_e4691_d_n11;
        locals.var_vgp_ini_dn12 = assign7050_e4691_d_n12;
        locals.var_vgp_ini_dn17 = assign7050_e4691_d_n17;

        let (assign7060_e4699, assign7060_e4699_d_n0, assign7060_e4699_d_n2, assign7060_e4699_d_n6, assign7060_e4699_d_n7, assign7060_e4699_d_n10, assign7060_e4699_d_n11, assign7060_e4699_d_n12, assign7060_e4699_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7060_e4695: f64 = (1.0 / locals.var_cnst1soi);
        let assign7060_e4697: f64 = (assign7060_e4695 / locals.var_cnstc_foxi);
        (assign7060_e4697, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7060_e4695 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7060_e4695 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7060_e4695 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7060_e4695 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7060_e4695 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7060_e4695 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7060_e4695 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7060_e4695 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign7060_e4699;
        locals.var_t1_dn0 = assign7060_e4699_d_n0;
        locals.var_t1_dn2 = assign7060_e4699_d_n2;
        locals.var_t1_dn6 = assign7060_e4699_d_n6;
        locals.var_t1_dn7 = assign7060_e4699_d_n7;
        locals.var_t1_dn10 = assign7060_e4699_d_n10;
        locals.var_t1_dn11 = assign7060_e4699_d_n11;
        locals.var_t1_dn12 = assign7060_e4699_d_n12;
        locals.var_t1_dn17 = assign7060_e4699_d_n17;

        let (assign7070_e4711, assign7070_e4711_d_n0, assign7070_e4711_d_n2, assign7070_e4711_d_n6, assign7070_e4711_d_n7, assign7070_e4711_d_n10, assign7070_e4711_d_n11, assign7070_e4711_d_n12, assign7070_e4711_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7070_e4704: f64 = (locals.var_vgp_ini - locals.var_shift);
        let assign7070_e4705: f64 = (locals.var_t1 * assign7070_e4704);
        let assign7070_e4708: f64 = (locals.var_vgp_ini - locals.var_shift);
        let assign7070_e4709: f64 = (assign7070_e4705 * assign7070_e4708);
        (assign7070_e4709, ((((locals.var_t1_dn0 * assign7070_e4704) + (locals.var_t1 * (locals.var_vgp_ini_dn0 - locals.var_shift_dn0))) * assign7070_e4708) + (assign7070_e4705 * (locals.var_vgp_ini_dn0 - locals.var_shift_dn0))), ((((locals.var_t1_dn2 * assign7070_e4704) + (locals.var_t1 * (locals.var_vgp_ini_dn2 - locals.var_shift_dn2))) * assign7070_e4708) + (assign7070_e4705 * (locals.var_vgp_ini_dn2 - locals.var_shift_dn2))), ((((locals.var_t1_dn6 * assign7070_e4704) + (locals.var_t1 * (locals.var_vgp_ini_dn6 - locals.var_shift_dn6))) * assign7070_e4708) + (assign7070_e4705 * (locals.var_vgp_ini_dn6 - locals.var_shift_dn6))), ((((locals.var_t1_dn7 * assign7070_e4704) + (locals.var_t1 * (locals.var_vgp_ini_dn7 - locals.var_shift_dn7))) * assign7070_e4708) + (assign7070_e4705 * (locals.var_vgp_ini_dn7 - locals.var_shift_dn7))), ((((locals.var_t1_dn10 * assign7070_e4704) + (locals.var_t1 * (locals.var_vgp_ini_dn10 - locals.var_shift_dn10))) * assign7070_e4708) + (assign7070_e4705 * (locals.var_vgp_ini_dn10 - locals.var_shift_dn10))), ((((locals.var_t1_dn11 * assign7070_e4704) + (locals.var_t1 * (locals.var_vgp_ini_dn11 - locals.var_shift_dn11))) * assign7070_e4708) + (assign7070_e4705 * (locals.var_vgp_ini_dn11 - locals.var_shift_dn11))), ((((locals.var_t1_dn12 * assign7070_e4704) + (locals.var_t1 * (locals.var_vgp_ini_dn12 - locals.var_shift_dn12))) * assign7070_e4708) + (assign7070_e4705 * (locals.var_vgp_ini_dn12 - locals.var_shift_dn12))), ((((locals.var_t1_dn17 * assign7070_e4704) + (locals.var_t1 * (locals.var_vgp_ini_dn17 - locals.var_shift_dn17))) * assign7070_e4708) + (assign7070_e4705 * (locals.var_vgp_ini_dn17 - locals.var_shift_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign7070_e4711;
        locals.var_t2_dn0 = assign7070_e4711_d_n0;
        locals.var_t2_dn2 = assign7070_e4711_d_n2;
        locals.var_t2_dn6 = assign7070_e4711_d_n6;
        locals.var_t2_dn7 = assign7070_e4711_d_n7;
        locals.var_t2_dn10 = assign7070_e4711_d_n10;
        locals.var_t2_dn11 = assign7070_e4711_d_n11;
        locals.var_t2_dn12 = assign7070_e4711_d_n12;
        locals.var_t2_dn17 = assign7070_e4711_d_n17;

        let (assign7080_e4721, assign7080_e4721_d_n0, assign7080_e4721_d_n2, assign7080_e4721_d_n6, assign7080_e4721_d_n7, assign7080_e4721_d_n10, assign7080_e4721_d_n11, assign7080_e4721_d_n12, assign7080_e4721_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7080_e4717: f64 = (locals.var_vgp_ini - locals.var_shift);
        let assign7080_e4718: f64 = (2.0 / assign7080_e4717);
        let assign7080_e4719: f64 = (locals.var_beta + assign7080_e4718);
        (assign7080_e4719, (-((2.0 * (locals.var_vgp_ini_dn0 - locals.var_shift_dn0)) / (assign7080_e4717 * assign7080_e4717))), (-((2.0 * (locals.var_vgp_ini_dn2 - locals.var_shift_dn2)) / (assign7080_e4717 * assign7080_e4717))), (-((2.0 * (locals.var_vgp_ini_dn6 - locals.var_shift_dn6)) / (assign7080_e4717 * assign7080_e4717))), (-((2.0 * (locals.var_vgp_ini_dn7 - locals.var_shift_dn7)) / (assign7080_e4717 * assign7080_e4717))), (locals.var_beta_dn10 + (-((2.0 * (locals.var_vgp_ini_dn10 - locals.var_shift_dn10)) / (assign7080_e4717 * assign7080_e4717)))), (-((2.0 * (locals.var_vgp_ini_dn11 - locals.var_shift_dn11)) / (assign7080_e4717 * assign7080_e4717))), (-((2.0 * (locals.var_vgp_ini_dn12 - locals.var_shift_dn12)) / (assign7080_e4717 * assign7080_e4717))), (-((2.0 * (locals.var_vgp_ini_dn17 - locals.var_shift_dn17)) / (assign7080_e4717 * assign7080_e4717))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign7080_e4721;
        locals.var_t3_dn0 = assign7080_e4721_d_n0;
        locals.var_t3_dn2 = assign7080_e4721_d_n2;
        locals.var_t3_dn6 = assign7080_e4721_d_n6;
        locals.var_t3_dn7 = assign7080_e4721_d_n7;
        locals.var_t3_dn10 = assign7080_e4721_d_n10;
        locals.var_t3_dn11 = assign7080_e4721_d_n11;
        locals.var_t3_dn12 = assign7080_e4721_d_n12;
        locals.var_t3_dn17 = assign7080_e4721_d_n17;

        let (assign7090_e4728, assign7090_e4728_d_n0, assign7090_e4728_d_n2, assign7090_e4728_d_n6, assign7090_e4728_d_n7, assign7090_e4728_d_n10, assign7090_e4728_d_n11, assign7090_e4728_d_n12, assign7090_e4728_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7090_e4724: f64 = (locals.var_t2).ln();
        let assign7090_e4726: f64 = (assign7090_e4724 / locals.var_t3);
        (assign7090_e4726, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign7090_e4724 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign7090_e4724 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign7090_e4724 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign7090_e4724 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign7090_e4724 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign7090_e4724 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign7090_e4724 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign7090_e4724 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inic, locals.var_ps0_inic_dn0, locals.var_ps0_inic_dn2, locals.var_ps0_inic_dn6, locals.var_ps0_inic_dn7, locals.var_ps0_inic_dn10, locals.var_ps0_inic_dn11, locals.var_ps0_inic_dn12, locals.var_ps0_inic_dn17,)
    }
};
        locals.var_ps0_inic = assign7090_e4728;
        locals.var_ps0_inic_dn0 = assign7090_e4728_d_n0;
        locals.var_ps0_inic_dn2 = assign7090_e4728_d_n2;
        locals.var_ps0_inic_dn6 = assign7090_e4728_d_n6;
        locals.var_ps0_inic_dn7 = assign7090_e4728_d_n7;
        locals.var_ps0_inic_dn10 = assign7090_e4728_d_n10;
        locals.var_ps0_inic_dn11 = assign7090_e4728_d_n11;
        locals.var_ps0_inic_dn12 = assign7090_e4728_d_n12;
        locals.var_ps0_inic_dn17 = assign7090_e4728_d_n17;

        let (assign7100_e4735, assign7100_e4735_d_n0, assign7100_e4735_d_n2, assign7100_e4735_d_n6, assign7100_e4735_d_n7, assign7100_e4735_d_n10, assign7100_e4735_d_n11, assign7100_e4735_d_n12, assign7100_e4735_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7100_e4732: f64 = (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic);
        let assign7100_e4733: f64 = (assign7100_e4732).sqrt();
        (assign7100_e4733, (((locals.var_cnst_2esi_q_nsubs_dn0 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn0)) / (2.0 * assign7100_e4733)), (((locals.var_cnst_2esi_q_nsubs_dn2 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn2)) / (2.0 * assign7100_e4733)), (((locals.var_cnst_2esi_q_nsubs_dn6 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn6)) / (2.0 * assign7100_e4733)), (((locals.var_cnst_2esi_q_nsubs_dn7 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn7)) / (2.0 * assign7100_e4733)), (((locals.var_cnst_2esi_q_nsubs_dn10 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn10)) / (2.0 * assign7100_e4733)), (((locals.var_cnst_2esi_q_nsubs_dn11 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn11)) / (2.0 * assign7100_e4733)), (((locals.var_cnst_2esi_q_nsubs_dn12 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn12)) / (2.0 * assign7100_e4733)), (((locals.var_cnst_2esi_q_nsubs_dn17 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn17)) / (2.0 * assign7100_e4733)),)
    } else {
        (locals.var_wdsoi_ini0, locals.var_wdsoi_ini0_dn0, locals.var_wdsoi_ini0_dn2, locals.var_wdsoi_ini0_dn6, locals.var_wdsoi_ini0_dn7, locals.var_wdsoi_ini0_dn10, locals.var_wdsoi_ini0_dn11, locals.var_wdsoi_ini0_dn12, locals.var_wdsoi_ini0_dn17,)
    }
};
        locals.var_wdsoi_ini0 = assign7100_e4735;
        locals.var_wdsoi_ini0_dn0 = assign7100_e4735_d_n0;
        locals.var_wdsoi_ini0_dn2 = assign7100_e4735_d_n2;
        locals.var_wdsoi_ini0_dn6 = assign7100_e4735_d_n6;
        locals.var_wdsoi_ini0_dn7 = assign7100_e4735_d_n7;
        locals.var_wdsoi_ini0_dn10 = assign7100_e4735_d_n10;
        locals.var_wdsoi_ini0_dn11 = assign7100_e4735_d_n11;
        locals.var_wdsoi_ini0_dn12 = assign7100_e4735_d_n12;
        locals.var_wdsoi_ini0_dn17 = assign7100_e4735_d_n17;

        let (assign7110_e4744, assign7110_e4744_d_n0, assign7110_e4744_d_n2, assign7110_e4744_d_n6, assign7110_e4744_d_n7, assign7110_e4744_d_n10, assign7110_e4744_d_n11, assign7110_e4744_d_n12, assign7110_e4744_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let (assign7110_e4742, assign7110_e4742_d_n0, assign7110_e4742_d_n2, assign7110_e4742_d_n6, assign7110_e4742_d_n7, assign7110_e4742_d_n10, assign7110_e4742_d_n11, assign7110_e4742_d_n12, assign7110_e4742_d_n17,) = {
            if (locals.var_wdsoi_ini0 > p.p237) {
                (p.p237, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_wdsoi_ini0, locals.var_wdsoi_ini0_dn0, locals.var_wdsoi_ini0_dn2, locals.var_wdsoi_ini0_dn6, locals.var_wdsoi_ini0_dn7, locals.var_wdsoi_ini0_dn10, locals.var_wdsoi_ini0_dn11, locals.var_wdsoi_ini0_dn12, locals.var_wdsoi_ini0_dn17,)
            }
        };
        (assign7110_e4742, assign7110_e4742_d_n0, assign7110_e4742_d_n2, assign7110_e4742_d_n6, assign7110_e4742_d_n7, assign7110_e4742_d_n10, assign7110_e4742_d_n11, assign7110_e4742_d_n12, assign7110_e4742_d_n17,)
    } else {
        (locals.var_wdsoi_ini0, locals.var_wdsoi_ini0_dn0, locals.var_wdsoi_ini0_dn2, locals.var_wdsoi_ini0_dn6, locals.var_wdsoi_ini0_dn7, locals.var_wdsoi_ini0_dn10, locals.var_wdsoi_ini0_dn11, locals.var_wdsoi_ini0_dn12, locals.var_wdsoi_ini0_dn17,)
    }
};
        locals.var_wdsoi_ini0 = assign7110_e4744;
        locals.var_wdsoi_ini0_dn0 = assign7110_e4744_d_n0;
        locals.var_wdsoi_ini0_dn2 = assign7110_e4744_d_n2;
        locals.var_wdsoi_ini0_dn6 = assign7110_e4744_d_n6;
        locals.var_wdsoi_ini0_dn7 = assign7110_e4744_d_n7;
        locals.var_wdsoi_ini0_dn10 = assign7110_e4744_d_n10;
        locals.var_wdsoi_ini0_dn11 = assign7110_e4744_d_n11;
        locals.var_wdsoi_ini0_dn12 = assign7110_e4744_d_n12;
        locals.var_wdsoi_ini0_dn17 = assign7110_e4744_d_n17;

        let (assign7120_e4753, assign7120_e4753_d_n0, assign7120_e4753_d_n2, assign7120_e4753_d_n6, assign7120_e4753_d_n7, assign7120_e4753_d_n10, assign7120_e4753_d_n11, assign7120_e4753_d_n12, assign7120_e4753_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7120_e4747: f64 = (-1.6021918e-19);
        let assign7120_e4749: f64 = (assign7120_e4747 * locals.var_uc_nsubs);
        let assign7120_e4751: f64 = (assign7120_e4749 * locals.var_wdsoi_ini0);
        (assign7120_e4751, (((assign7120_e4747 * locals.var_uc_nsubs_dn0) * locals.var_wdsoi_ini0) + (assign7120_e4749 * locals.var_wdsoi_ini0_dn0)), (((assign7120_e4747 * locals.var_uc_nsubs_dn2) * locals.var_wdsoi_ini0) + (assign7120_e4749 * locals.var_wdsoi_ini0_dn2)), (((assign7120_e4747 * locals.var_uc_nsubs_dn6) * locals.var_wdsoi_ini0) + (assign7120_e4749 * locals.var_wdsoi_ini0_dn6)), (((assign7120_e4747 * locals.var_uc_nsubs_dn7) * locals.var_wdsoi_ini0) + (assign7120_e4749 * locals.var_wdsoi_ini0_dn7)), (((assign7120_e4747 * locals.var_uc_nsubs_dn10) * locals.var_wdsoi_ini0) + (assign7120_e4749 * locals.var_wdsoi_ini0_dn10)), (((assign7120_e4747 * locals.var_uc_nsubs_dn11) * locals.var_wdsoi_ini0) + (assign7120_e4749 * locals.var_wdsoi_ini0_dn11)), (((assign7120_e4747 * locals.var_uc_nsubs_dn12) * locals.var_wdsoi_ini0) + (assign7120_e4749 * locals.var_wdsoi_ini0_dn12)), (((assign7120_e4747 * locals.var_uc_nsubs_dn17) * locals.var_wdsoi_ini0) + (assign7120_e4749 * locals.var_wdsoi_ini0_dn17)),)
    } else {
        (locals.var_q_wdsoi_max, locals.var_q_wdsoi_max_dn0, locals.var_q_wdsoi_max_dn2, locals.var_q_wdsoi_max_dn6, locals.var_q_wdsoi_max_dn7, locals.var_q_wdsoi_max_dn10, locals.var_q_wdsoi_max_dn11, locals.var_q_wdsoi_max_dn12, locals.var_q_wdsoi_max_dn17,)
    }
};
        locals.var_q_wdsoi_max = assign7120_e4753;
        locals.var_q_wdsoi_max_dn0 = assign7120_e4753_d_n0;
        locals.var_q_wdsoi_max_dn2 = assign7120_e4753_d_n2;
        locals.var_q_wdsoi_max_dn6 = assign7120_e4753_d_n6;
        locals.var_q_wdsoi_max_dn7 = assign7120_e4753_d_n7;
        locals.var_q_wdsoi_max_dn10 = assign7120_e4753_d_n10;
        locals.var_q_wdsoi_max_dn11 = assign7120_e4753_d_n11;
        locals.var_q_wdsoi_max_dn12 = assign7120_e4753_d_n12;
        locals.var_q_wdsoi_max_dn17 = assign7120_e4753_d_n17;

        let (assign7130_e4757,) = {
    if (locals.var_guard111 != 0.0) {
        (p.p237,)
    } else {
        (locals.var_t_soi,)
    }
};
        locals.var_t_soi = assign7130_e4757;

        let (assign7140_e4766, assign7140_e4766_d_n0, assign7140_e4766_d_n2, assign7140_e4766_d_n6, assign7140_e4766_d_n7, assign7140_e4766_d_n10, assign7140_e4766_d_n11, assign7140_e4766_d_n12, assign7140_e4766_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7140_e4760: f64 = (-1.6021918e-19);
        let assign7140_e4762: f64 = (assign7140_e4760 * locals.var_uc_nsubs);
        let assign7140_e4764: f64 = (assign7140_e4762 * locals.var_t_soi);
        (assign7140_e4764, ((assign7140_e4760 * locals.var_uc_nsubs_dn0) * locals.var_t_soi), ((assign7140_e4760 * locals.var_uc_nsubs_dn2) * locals.var_t_soi), ((assign7140_e4760 * locals.var_uc_nsubs_dn6) * locals.var_t_soi), ((assign7140_e4760 * locals.var_uc_nsubs_dn7) * locals.var_t_soi), ((assign7140_e4760 * locals.var_uc_nsubs_dn10) * locals.var_t_soi), ((assign7140_e4760 * locals.var_uc_nsubs_dn11) * locals.var_t_soi), ((assign7140_e4760 * locals.var_uc_nsubs_dn12) * locals.var_t_soi), ((assign7140_e4760 * locals.var_uc_nsubs_dn17) * locals.var_t_soi),)
    } else {
        (locals.var_q_fd_soi, locals.var_q_fd_soi_dn0, locals.var_q_fd_soi_dn2, locals.var_q_fd_soi_dn6, locals.var_q_fd_soi_dn7, locals.var_q_fd_soi_dn10, locals.var_q_fd_soi_dn11, locals.var_q_fd_soi_dn12, locals.var_q_fd_soi_dn17,)
    }
};
        locals.var_q_fd_soi = assign7140_e4766;
        locals.var_q_fd_soi_dn0 = assign7140_e4766_d_n0;
        locals.var_q_fd_soi_dn2 = assign7140_e4766_d_n2;
        locals.var_q_fd_soi_dn6 = assign7140_e4766_d_n6;
        locals.var_q_fd_soi_dn7 = assign7140_e4766_d_n7;
        locals.var_q_fd_soi_dn10 = assign7140_e4766_d_n10;
        locals.var_q_fd_soi_dn11 = assign7140_e4766_d_n11;
        locals.var_q_fd_soi_dn12 = assign7140_e4766_d_n12;
        locals.var_q_fd_soi_dn17 = assign7140_e4766_d_n17;

        let (assign7150_e4770,) = {
    if (locals.var_guard111 != 0.0) {
        (1.5,)
    } else {
        (locals.var_wdsoi_ini1_dlt,)
    }
};
        locals.var_wdsoi_ini1_dlt = assign7150_e4770;

        let (assign7160_e4776,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7160_e4774: f64 = (1.034943e-10 / locals.var_t_soi);
        (assign7160_e4774,)
    } else {
        (locals.var_c_soi__blk112,)
    }
};
        locals.var_c_soi__blk112 = assign7160_e4776;

        let (assign7170_e4782,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7170_e4780: f64 = (1.0 / locals.var_c_soi__blk112);
        (assign7170_e4780,)
    } else {
        (locals.var_c_soi_inv__blk113,)
    }
};
        locals.var_c_soi_inv__blk113 = assign7170_e4782;

        let (assign7180_e4789, assign7180_e4789_d_n0, assign7180_e4789_d_n2, assign7180_e4789_d_n6, assign7180_e4789_d_n7, assign7180_e4789_d_n10, assign7180_e4789_d_n11, assign7180_e4789_d_n12, assign7180_e4789_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7180_e4785: f64 = (-locals.var_q_fd_soi);
        let assign7180_e4787: f64 = (assign7180_e4785 * 0.001);
        (assign7180_e4787, ((-locals.var_q_fd_soi_dn0) * 0.001), ((-locals.var_q_fd_soi_dn2) * 0.001), ((-locals.var_q_fd_soi_dn6) * 0.001), ((-locals.var_q_fd_soi_dn7) * 0.001), ((-locals.var_q_fd_soi_dn10) * 0.001), ((-locals.var_q_fd_soi_dn11) * 0.001), ((-locals.var_q_fd_soi_dn12) * 0.001), ((-locals.var_q_fd_soi_dn17) * 0.001),)
    } else {
        (locals.var_q_fd_dlt1, locals.var_q_fd_dlt1_dn0, locals.var_q_fd_dlt1_dn2, locals.var_q_fd_dlt1_dn6, locals.var_q_fd_dlt1_dn7, locals.var_q_fd_dlt1_dn10, locals.var_q_fd_dlt1_dn11, locals.var_q_fd_dlt1_dn12, locals.var_q_fd_dlt1_dn17,)
    }
};
        locals.var_q_fd_dlt1 = assign7180_e4789;
        locals.var_q_fd_dlt1_dn0 = assign7180_e4789_d_n0;
        locals.var_q_fd_dlt1_dn2 = assign7180_e4789_d_n2;
        locals.var_q_fd_dlt1_dn6 = assign7180_e4789_d_n6;
        locals.var_q_fd_dlt1_dn7 = assign7180_e4789_d_n7;
        locals.var_q_fd_dlt1_dn10 = assign7180_e4789_d_n10;
        locals.var_q_fd_dlt1_dn11 = assign7180_e4789_d_n11;
        locals.var_q_fd_dlt1_dn12 = assign7180_e4789_d_n12;
        locals.var_q_fd_dlt1_dn17 = assign7180_e4789_d_n17;

        let (assign7190_e4796, assign7190_e4796_d_n0, assign7190_e4796_d_n2, assign7190_e4796_d_n6, assign7190_e4796_d_n7, assign7190_e4796_d_n10, assign7190_e4796_d_n11, assign7190_e4796_d_n12, assign7190_e4796_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7190_e4792: f64 = (-locals.var_q_fd_soi);
        let assign7190_e4794: f64 = (assign7190_e4792 * 1e-5);
        (assign7190_e4794, ((-locals.var_q_fd_soi_dn0) * 1e-5), ((-locals.var_q_fd_soi_dn2) * 1e-5), ((-locals.var_q_fd_soi_dn6) * 1e-5), ((-locals.var_q_fd_soi_dn7) * 1e-5), ((-locals.var_q_fd_soi_dn10) * 1e-5), ((-locals.var_q_fd_soi_dn11) * 1e-5), ((-locals.var_q_fd_soi_dn12) * 1e-5), ((-locals.var_q_fd_soi_dn17) * 1e-5),)
    } else {
        (locals.var_q_fd_dlt2, locals.var_q_fd_dlt2_dn0, locals.var_q_fd_dlt2_dn2, locals.var_q_fd_dlt2_dn6, locals.var_q_fd_dlt2_dn7, locals.var_q_fd_dlt2_dn10, locals.var_q_fd_dlt2_dn11, locals.var_q_fd_dlt2_dn12, locals.var_q_fd_dlt2_dn17,)
    }
};
        locals.var_q_fd_dlt2 = assign7190_e4796;
        locals.var_q_fd_dlt2_dn0 = assign7190_e4796_d_n0;
        locals.var_q_fd_dlt2_dn2 = assign7190_e4796_d_n2;
        locals.var_q_fd_dlt2_dn6 = assign7190_e4796_d_n6;
        locals.var_q_fd_dlt2_dn7 = assign7190_e4796_d_n7;
        locals.var_q_fd_dlt2_dn10 = assign7190_e4796_d_n10;
        locals.var_q_fd_dlt2_dn11 = assign7190_e4796_d_n11;
        locals.var_q_fd_dlt2_dn12 = assign7190_e4796_d_n12;
        locals.var_q_fd_dlt2_dn17 = assign7190_e4796_d_n17;

    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7200_e4804, assign7200_e4804_d_n0, assign7200_e4804_d_n2, assign7200_e4804_d_n6, assign7200_e4804_d_n7, assign7200_e4804_d_n10, assign7200_e4804_d_n11, assign7200_e4804_d_n12, assign7200_e4804_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (p.p39 != 0.0)) {
        let assign7200_e4802: f64 = (locals.var_vbsz + locals.var_vbi_soi);
        (assign7200_e4802, (locals.var_vbsz_dn0 + locals.var_vbi_soi_dn0), (locals.var_vbsz_dn2 + locals.var_vbi_soi_dn2), (locals.var_vbsz_dn6 + locals.var_vbi_soi_dn6), (locals.var_vbsz_dn7 + locals.var_vbi_soi_dn7), (locals.var_vbsz_dn10 + locals.var_vbi_soi_dn10), (locals.var_vbsz_dn11 + locals.var_vbi_soi_dn11), (locals.var_vbsz_dn12 + locals.var_vbi_soi_dn12), (locals.var_vbsz_dn17 + locals.var_vbi_soi_dn17),)
    } else {
        (locals.var_vbsbiz, locals.var_vbsbiz_dn0, locals.var_vbsbiz_dn2, locals.var_vbsbiz_dn6, locals.var_vbsbiz_dn7, locals.var_vbsbiz_dn10, locals.var_vbsbiz_dn11, locals.var_vbsbiz_dn12, locals.var_vbsbiz_dn17,)
    }
};
        locals.var_vbsbiz = assign7200_e4804;
        locals.var_vbsbiz_dn0 = assign7200_e4804_d_n0;
        locals.var_vbsbiz_dn2 = assign7200_e4804_d_n2;
        locals.var_vbsbiz_dn6 = assign7200_e4804_d_n6;
        locals.var_vbsbiz_dn7 = assign7200_e4804_d_n7;
        locals.var_vbsbiz_dn10 = assign7200_e4804_d_n10;
        locals.var_vbsbiz_dn11 = assign7200_e4804_d_n11;
        locals.var_vbsbiz_dn12 = assign7200_e4804_d_n12;
        locals.var_vbsbiz_dn17 = assign7200_e4804_d_n17;

        let (assign7210_e4813, assign7210_e4813_d_n0, assign7210_e4813_d_n2, assign7210_e4813_d_n6, assign7210_e4813_d_n7, assign7210_e4813_d_n10, assign7210_e4813_d_n11, assign7210_e4813_d_n12, assign7210_e4813_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (p.p39 == 0.0)) {
        let assign7210_e4811: f64 = (locals.var_vbs + locals.var_vbi_soi);
        (assign7210_e4811, (locals.var_vbs_dn0 + locals.var_vbi_soi_dn0), (locals.var_vbs_dn2 + locals.var_vbi_soi_dn2), (locals.var_vbs_dn6 + locals.var_vbi_soi_dn6), (locals.var_vbs_dn7 + locals.var_vbi_soi_dn7), (locals.var_vbs_dn10 + locals.var_vbi_soi_dn10), (locals.var_vbs_dn11 + locals.var_vbi_soi_dn11), (locals.var_vbs_dn12 + locals.var_vbi_soi_dn12), (locals.var_vbs_dn17 + locals.var_vbi_soi_dn17),)
    } else {
        (locals.var_vbsbiz, locals.var_vbsbiz_dn0, locals.var_vbsbiz_dn2, locals.var_vbsbiz_dn6, locals.var_vbsbiz_dn7, locals.var_vbsbiz_dn10, locals.var_vbsbiz_dn11, locals.var_vbsbiz_dn12, locals.var_vbsbiz_dn17,)
    }
};
        locals.var_vbsbiz = assign7210_e4813;
        locals.var_vbsbiz_dn0 = assign7210_e4813_d_n0;
        locals.var_vbsbiz_dn2 = assign7210_e4813_d_n2;
        locals.var_vbsbiz_dn6 = assign7210_e4813_d_n6;
        locals.var_vbsbiz_dn7 = assign7210_e4813_d_n7;
        locals.var_vbsbiz_dn10 = assign7210_e4813_d_n10;
        locals.var_vbsbiz_dn11 = assign7210_e4813_d_n11;
        locals.var_vbsbiz_dn12 = assign7210_e4813_d_n12;
        locals.var_vbsbiz_dn17 = assign7210_e4813_d_n17;

        let (assign7220_e4824,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7220_e4817: f64 = (2.0 / locals.var_beta);
        let assign7220_e4820: f64 = (locals.var_mks_nsubb / locals.var_nin);
        let assign7220_e4821: f64 = (assign7220_e4820).ln();
        let assign7220_e4822: f64 = (assign7220_e4817 * assign7220_e4821);
        (assign7220_e4822,)
    } else {
        (locals.var_pb2_bulk,)
    }
};
        locals.var_pb2_bulk = assign7220_e4824;

        let (assign7230_e4834, assign7230_e4834_d_n10,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7230_e4828: f64 = (locals.var_cnst0bulk * locals.var_cnst0bulk);
        let assign7230_e4830: f64 = (assign7230_e4828 * locals.var_c_box_fd_inv);
        let assign7230_e4832: f64 = (assign7230_e4830 * locals.var_c_box_fd_inv);
        (assign7230_e4832, ((((locals.var_cnst0bulk_dn10 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn10)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv),)
    } else {
        (locals.var_t0__blk119, locals.var_t0__blk119_dn10,)
    }
};
        locals.var_t0__blk119 = assign7230_e4834;
        locals.var_t0__blk119_dn10 = assign7230_e4834_d_n10;

        let (assign7240_e4839, assign7240_e4839_d_n0, assign7240_e4839_d_n2, assign7240_e4839_d_n6, assign7240_e4839_d_n7, assign7240_e4839_d_n10, assign7240_e4839_d_n11, assign7240_e4839_d_n12, assign7240_e4839_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7240_e4837: f64 = (-locals.var_vbsbiz);
        (assign7240_e4837, (-locals.var_vbsbiz_dn0), (-locals.var_vbsbiz_dn2), (-locals.var_vbsbiz_dn6), (-locals.var_vbsbiz_dn7), (-locals.var_vbsbiz_dn10), (-locals.var_vbsbiz_dn11), (-locals.var_vbsbiz_dn12), (-locals.var_vbsbiz_dn17),)
    } else {
        (locals.var_t1__blk120, locals.var_t1__blk120_dn0, locals.var_t1__blk120_dn2, locals.var_t1__blk120_dn6, locals.var_t1__blk120_dn7, locals.var_t1__blk120_dn10, locals.var_t1__blk120_dn11, locals.var_t1__blk120_dn12, locals.var_t1__blk120_dn17,)
    }
};
        locals.var_t1__blk120 = assign7240_e4839;
        locals.var_t1__blk120_dn0 = assign7240_e4839_d_n0;
        locals.var_t1__blk120_dn2 = assign7240_e4839_d_n2;
        locals.var_t1__blk120_dn6 = assign7240_e4839_d_n6;
        locals.var_t1__blk120_dn7 = assign7240_e4839_d_n7;
        locals.var_t1__blk120_dn10 = assign7240_e4839_d_n10;
        locals.var_t1__blk120_dn11 = assign7240_e4839_d_n11;
        locals.var_t1__blk120_dn12 = assign7240_e4839_d_n12;
        locals.var_t1__blk120_dn17 = assign7240_e4839_d_n17;

        let (assign7250_e4865, assign7250_e4865_d_n0, assign7250_e4865_d_n2, assign7250_e4865_d_n6, assign7250_e4865_d_n7, assign7250_e4865_d_n10, assign7250_e4865_d_n11, assign7250_e4865_d_n12, assign7250_e4865_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7250_e4843: f64 = (2.0 * locals.var_t1__blk120);
        let assign7250_e4846: f64 = (locals.var_t0__blk119 * locals.var_beta);
        let assign7250_e4847: f64 = (assign7250_e4843 + assign7250_e4846);
        let assign7250_e4850: f64 = (2.0 * locals.var_t1__blk120);
        let assign7250_e4853: f64 = (locals.var_t0__blk119 * locals.var_beta);
        let assign7250_e4854: f64 = (assign7250_e4850 + assign7250_e4853);
        let assign7250_e4855: f64 = (assign7250_e4847 * assign7250_e4854);
        let assign7250_e4859: f64 = (locals.var_t1__blk120 * locals.var_t1__blk120);
        let assign7250_e4861: f64 = (assign7250_e4859 + locals.var_t0__blk119);
        let assign7250_e4862: f64 = (4.0 * assign7250_e4861);
        let assign7250_e4863: f64 = (assign7250_e4855 - assign7250_e4862);
        (assign7250_e4863, ((((2.0 * locals.var_t1__blk120_dn0) * assign7250_e4854) + (assign7250_e4847 * (2.0 * locals.var_t1__blk120_dn0))) - (4.0 * ((locals.var_t1__blk120_dn0 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn0)))), ((((2.0 * locals.var_t1__blk120_dn2) * assign7250_e4854) + (assign7250_e4847 * (2.0 * locals.var_t1__blk120_dn2))) - (4.0 * ((locals.var_t1__blk120_dn2 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn2)))), ((((2.0 * locals.var_t1__blk120_dn6) * assign7250_e4854) + (assign7250_e4847 * (2.0 * locals.var_t1__blk120_dn6))) - (4.0 * ((locals.var_t1__blk120_dn6 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn6)))), ((((2.0 * locals.var_t1__blk120_dn7) * assign7250_e4854) + (assign7250_e4847 * (2.0 * locals.var_t1__blk120_dn7))) - (4.0 * ((locals.var_t1__blk120_dn7 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn7)))), (((((2.0 * locals.var_t1__blk120_dn10) + ((locals.var_t0__blk119_dn10 * locals.var_beta) + (locals.var_t0__blk119 * locals.var_beta_dn10))) * assign7250_e4854) + (assign7250_e4847 * ((2.0 * locals.var_t1__blk120_dn10) + ((locals.var_t0__blk119_dn10 * locals.var_beta) + (locals.var_t0__blk119 * locals.var_beta_dn10))))) - (4.0 * (((locals.var_t1__blk120_dn10 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn10)) + locals.var_t0__blk119_dn10))), ((((2.0 * locals.var_t1__blk120_dn11) * assign7250_e4854) + (assign7250_e4847 * (2.0 * locals.var_t1__blk120_dn11))) - (4.0 * ((locals.var_t1__blk120_dn11 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn11)))), ((((2.0 * locals.var_t1__blk120_dn12) * assign7250_e4854) + (assign7250_e4847 * (2.0 * locals.var_t1__blk120_dn12))) - (4.0 * ((locals.var_t1__blk120_dn12 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn12)))), ((((2.0 * locals.var_t1__blk120_dn17) * assign7250_e4854) + (assign7250_e4847 * (2.0 * locals.var_t1__blk120_dn17))) - (4.0 * ((locals.var_t1__blk120_dn17 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn17)))),)
    } else {
        (locals.var_t2__blk121, locals.var_t2__blk121_dn0, locals.var_t2__blk121_dn2, locals.var_t2__blk121_dn6, locals.var_t2__blk121_dn7, locals.var_t2__blk121_dn10, locals.var_t2__blk121_dn11, locals.var_t2__blk121_dn12, locals.var_t2__blk121_dn17,)
    }
};
        locals.var_t2__blk121 = assign7250_e4865;
        locals.var_t2__blk121_dn0 = assign7250_e4865_d_n0;
        locals.var_t2__blk121_dn2 = assign7250_e4865_d_n2;
        locals.var_t2__blk121_dn6 = assign7250_e4865_d_n6;
        locals.var_t2__blk121_dn7 = assign7250_e4865_d_n7;
        locals.var_t2__blk121_dn10 = assign7250_e4865_d_n10;
        locals.var_t2__blk121_dn11 = assign7250_e4865_d_n11;
        locals.var_t2__blk121_dn12 = assign7250_e4865_d_n12;
        locals.var_t2__blk121_dn17 = assign7250_e4865_d_n17;

        let (assign7260_e4878, assign7260_e4878_d_n0, assign7260_e4878_d_n2, assign7260_e4878_d_n6, assign7260_e4878_d_n7, assign7260_e4878_d_n10, assign7260_e4878_d_n11, assign7260_e4878_d_n12, assign7260_e4878_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7260_e4870: f64 = (10.0 * 2.220446049250313e-16);
        let (assign7260_e4876, assign7260_e4876_d_n0, assign7260_e4876_d_n2, assign7260_e4876_d_n6, assign7260_e4876_d_n7, assign7260_e4876_d_n10, assign7260_e4876_d_n11, assign7260_e4876_d_n12, assign7260_e4876_d_n17,) = {
            if (locals.var_t2__blk121 >= assign7260_e4870) {
                (locals.var_t2__blk121, locals.var_t2__blk121_dn0, locals.var_t2__blk121_dn2, locals.var_t2__blk121_dn6, locals.var_t2__blk121_dn7, locals.var_t2__blk121_dn10, locals.var_t2__blk121_dn11, locals.var_t2__blk121_dn12, locals.var_t2__blk121_dn17,)
            } else {
                let assign7260_e4875: f64 = (10.0 * 2.220446049250313e-16);
                (assign7260_e4875, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign7260_e4876, assign7260_e4876_d_n0, assign7260_e4876_d_n2, assign7260_e4876_d_n6, assign7260_e4876_d_n7, assign7260_e4876_d_n10, assign7260_e4876_d_n11, assign7260_e4876_d_n12, assign7260_e4876_d_n17,)
    } else {
        (locals.var_t2__blk121, locals.var_t2__blk121_dn0, locals.var_t2__blk121_dn2, locals.var_t2__blk121_dn6, locals.var_t2__blk121_dn7, locals.var_t2__blk121_dn10, locals.var_t2__blk121_dn11, locals.var_t2__blk121_dn12, locals.var_t2__blk121_dn17,)
    }
};
        locals.var_t2__blk121 = assign7260_e4878;
        locals.var_t2__blk121_dn0 = assign7260_e4878_d_n0;
        locals.var_t2__blk121_dn2 = assign7260_e4878_d_n2;
        locals.var_t2__blk121_dn6 = assign7260_e4878_d_n6;
        locals.var_t2__blk121_dn7 = assign7260_e4878_d_n7;
        locals.var_t2__blk121_dn10 = assign7260_e4878_d_n10;
        locals.var_t2__blk121_dn11 = assign7260_e4878_d_n11;
        locals.var_t2__blk121_dn12 = assign7260_e4878_d_n12;
        locals.var_t2__blk121_dn17 = assign7260_e4878_d_n17;

        let (assign7270_e4883, assign7270_e4883_d_n0, assign7270_e4883_d_n2, assign7270_e4883_d_n6, assign7270_e4883_d_n7, assign7270_e4883_d_n10, assign7270_e4883_d_n11, assign7270_e4883_d_n12, assign7270_e4883_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7270_e4881: f64 = (locals.var_t2__blk121).sqrt();
        (assign7270_e4881, (locals.var_t2__blk121_dn0 / (2.0 * assign7270_e4881)), (locals.var_t2__blk121_dn2 / (2.0 * assign7270_e4881)), (locals.var_t2__blk121_dn6 / (2.0 * assign7270_e4881)), (locals.var_t2__blk121_dn7 / (2.0 * assign7270_e4881)), (locals.var_t2__blk121_dn10 / (2.0 * assign7270_e4881)), (locals.var_t2__blk121_dn11 / (2.0 * assign7270_e4881)), (locals.var_t2__blk121_dn12 / (2.0 * assign7270_e4881)), (locals.var_t2__blk121_dn17 / (2.0 * assign7270_e4881)),)
    } else {
        (locals.var_t2__blk121, locals.var_t2__blk121_dn0, locals.var_t2__blk121_dn2, locals.var_t2__blk121_dn6, locals.var_t2__blk121_dn7, locals.var_t2__blk121_dn10, locals.var_t2__blk121_dn11, locals.var_t2__blk121_dn12, locals.var_t2__blk121_dn17,)
    }
};
        locals.var_t2__blk121 = assign7270_e4883;
        locals.var_t2__blk121_dn0 = assign7270_e4883_d_n0;
        locals.var_t2__blk121_dn2 = assign7270_e4883_d_n2;
        locals.var_t2__blk121_dn6 = assign7270_e4883_d_n6;
        locals.var_t2__blk121_dn7 = assign7270_e4883_d_n7;
        locals.var_t2__blk121_dn10 = assign7270_e4883_d_n10;
        locals.var_t2__blk121_dn11 = assign7270_e4883_d_n11;
        locals.var_t2__blk121_dn12 = assign7270_e4883_d_n12;
        locals.var_t2__blk121_dn17 = assign7270_e4883_d_n17;

        let (assign7280_e4893, assign7280_e4893_d_n0, assign7280_e4893_d_n2, assign7280_e4893_d_n6, assign7280_e4893_d_n7, assign7280_e4893_d_n10, assign7280_e4893_d_n11, assign7280_e4893_d_n12, assign7280_e4893_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7280_e4887: f64 = (2.0 * locals.var_t1__blk120);
        let assign7280_e4890: f64 = (locals.var_t0__blk119 * locals.var_beta);
        let assign7280_e4891: f64 = (assign7280_e4887 + assign7280_e4890);
        (assign7280_e4891, (2.0 * locals.var_t1__blk120_dn0), (2.0 * locals.var_t1__blk120_dn2), (2.0 * locals.var_t1__blk120_dn6), (2.0 * locals.var_t1__blk120_dn7), ((2.0 * locals.var_t1__blk120_dn10) + ((locals.var_t0__blk119_dn10 * locals.var_beta) + (locals.var_t0__blk119 * locals.var_beta_dn10))), (2.0 * locals.var_t1__blk120_dn11), (2.0 * locals.var_t1__blk120_dn12), (2.0 * locals.var_t1__blk120_dn17),)
    } else {
        (locals.var_t3__blk122, locals.var_t3__blk122_dn0, locals.var_t3__blk122_dn2, locals.var_t3__blk122_dn6, locals.var_t3__blk122_dn7, locals.var_t3__blk122_dn10, locals.var_t3__blk122_dn11, locals.var_t3__blk122_dn12, locals.var_t3__blk122_dn17,)
    }
};
        locals.var_t3__blk122 = assign7280_e4893;
        locals.var_t3__blk122_dn0 = assign7280_e4893_d_n0;
        locals.var_t3__blk122_dn2 = assign7280_e4893_d_n2;
        locals.var_t3__blk122_dn6 = assign7280_e4893_d_n6;
        locals.var_t3__blk122_dn7 = assign7280_e4893_d_n7;
        locals.var_t3__blk122_dn10 = assign7280_e4893_d_n10;
        locals.var_t3__blk122_dn11 = assign7280_e4893_d_n11;
        locals.var_t3__blk122_dn12 = assign7280_e4893_d_n12;
        locals.var_t3__blk122_dn17 = assign7280_e4893_d_n17;

        let (assign7290_e4901, assign7290_e4901_d_n0, assign7290_e4901_d_n2, assign7290_e4901_d_n6, assign7290_e4901_d_n7, assign7290_e4901_d_n10, assign7290_e4901_d_n11, assign7290_e4901_d_n12, assign7290_e4901_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7290_e4897: f64 = (locals.var_t3__blk122 - locals.var_t2__blk121);
        let assign7290_e4899: f64 = (assign7290_e4897 / 2.0);
        (assign7290_e4899, ((locals.var_t3__blk122_dn0 - locals.var_t2__blk121_dn0) / 2.0), ((locals.var_t3__blk122_dn2 - locals.var_t2__blk121_dn2) / 2.0), ((locals.var_t3__blk122_dn6 - locals.var_t2__blk121_dn6) / 2.0), ((locals.var_t3__blk122_dn7 - locals.var_t2__blk121_dn7) / 2.0), ((locals.var_t3__blk122_dn10 - locals.var_t2__blk121_dn10) / 2.0), ((locals.var_t3__blk122_dn11 - locals.var_t2__blk121_dn11) / 2.0), ((locals.var_t3__blk122_dn12 - locals.var_t2__blk121_dn12) / 2.0), ((locals.var_t3__blk122_dn17 - locals.var_t2__blk121_dn17) / 2.0),)
    } else {
        (locals.var_psb_inia__blk123, locals.var_psb_inia__blk123_dn0, locals.var_psb_inia__blk123_dn2, locals.var_psb_inia__blk123_dn6, locals.var_psb_inia__blk123_dn7, locals.var_psb_inia__blk123_dn10, locals.var_psb_inia__blk123_dn11, locals.var_psb_inia__blk123_dn12, locals.var_psb_inia__blk123_dn17,)
    }
};
        locals.var_psb_inia__blk123 = assign7290_e4901;
        locals.var_psb_inia__blk123_dn0 = assign7290_e4901_d_n0;
        locals.var_psb_inia__blk123_dn2 = assign7290_e4901_d_n2;
        locals.var_psb_inia__blk123_dn6 = assign7290_e4901_d_n6;
        locals.var_psb_inia__blk123_dn7 = assign7290_e4901_d_n7;
        locals.var_psb_inia__blk123_dn10 = assign7290_e4901_d_n10;
        locals.var_psb_inia__blk123_dn11 = assign7290_e4901_d_n11;
        locals.var_psb_inia__blk123_dn12 = assign7290_e4901_d_n12;
        locals.var_psb_inia__blk123_dn17 = assign7290_e4901_d_n17;

        let (assign7300_e4918, assign7300_e4918_d_n0, assign7300_e4918_d_n2, assign7300_e4918_d_n6, assign7300_e4918_d_n7, assign7300_e4918_d_n10, assign7300_e4918_d_n11, assign7300_e4918_d_n12, assign7300_e4918_d_n17,) = {
    if (locals.var_guard111 != 0.0) {
        let assign7300_e4905: f64 = (locals.var_t1__blk120 * locals.var_t1__blk120);
        let assign7300_e4907: f64 = (assign7300_e4905 / locals.var_t0__blk119);
        let assign7300_e4909: f64 = (assign7300_e4907 / locals.var_cnst1bulk);
        let assign7300_e4910: f64 = (assign7300_e4909).ln();
        let assign7300_e4914: f64 = (2.0 / locals.var_t1__blk120);
        let assign7300_e4915: f64 = (locals.var_beta + assign7300_e4914);
        let assign7300_e4916: f64 = (assign7300_e4910 / assign7300_e4915);
        (assign7300_e4916, ((((((((((locals.var_t1__blk120_dn0 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn0)) / locals.var_t0__blk119) * locals.var_cnst1bulk) - (assign7300_e4907 * locals.var_cnst1bulk_dn0)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (-((2.0 * locals.var_t1__blk120_dn0) / (locals.var_t1__blk120 * locals.var_t1__blk120))))) / (assign7300_e4915 * assign7300_e4915)), ((((((((((locals.var_t1__blk120_dn2 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn2)) / locals.var_t0__blk119) * locals.var_cnst1bulk) - (assign7300_e4907 * locals.var_cnst1bulk_dn2)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (-((2.0 * locals.var_t1__blk120_dn2) / (locals.var_t1__blk120 * locals.var_t1__blk120))))) / (assign7300_e4915 * assign7300_e4915)), ((((((((((locals.var_t1__blk120_dn6 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn6)) / locals.var_t0__blk119) * locals.var_cnst1bulk) - (assign7300_e4907 * locals.var_cnst1bulk_dn6)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (-((2.0 * locals.var_t1__blk120_dn6) / (locals.var_t1__blk120 * locals.var_t1__blk120))))) / (assign7300_e4915 * assign7300_e4915)), ((((((((((locals.var_t1__blk120_dn7 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn7)) / locals.var_t0__blk119) * locals.var_cnst1bulk) - (assign7300_e4907 * locals.var_cnst1bulk_dn7)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (-((2.0 * locals.var_t1__blk120_dn7) / (locals.var_t1__blk120 * locals.var_t1__blk120))))) / (assign7300_e4915 * assign7300_e4915)), ((((((((((((locals.var_t1__blk120_dn10 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn10)) * locals.var_t0__blk119) - (assign7300_e4905 * locals.var_t0__blk119_dn10)) / (locals.var_t0__blk119 * locals.var_t0__blk119)) * locals.var_cnst1bulk) - (assign7300_e4907 * locals.var_cnst1bulk_dn10)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (locals.var_beta_dn10 + (-((2.0 * locals.var_t1__blk120_dn10) / (locals.var_t1__blk120 * locals.var_t1__blk120)))))) / (assign7300_e4915 * assign7300_e4915)), ((((((((((locals.var_t1__blk120_dn11 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn11)) / locals.var_t0__blk119) * locals.var_cnst1bulk) - (assign7300_e4907 * locals.var_cnst1bulk_dn11)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (-((2.0 * locals.var_t1__blk120_dn11) / (locals.var_t1__blk120 * locals.var_t1__blk120))))) / (assign7300_e4915 * assign7300_e4915)), ((((((((((locals.var_t1__blk120_dn12 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn12)) / locals.var_t0__blk119) * locals.var_cnst1bulk) - (assign7300_e4907 * locals.var_cnst1bulk_dn12)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (-((2.0 * locals.var_t1__blk120_dn12) / (locals.var_t1__blk120 * locals.var_t1__blk120))))) / (assign7300_e4915 * assign7300_e4915)), ((((((((((locals.var_t1__blk120_dn17 * locals.var_t1__blk120) + (locals.var_t1__blk120 * locals.var_t1__blk120_dn17)) / locals.var_t0__blk119) * locals.var_cnst1bulk) - (assign7300_e4907 * locals.var_cnst1bulk_dn17)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7300_e4909) * assign7300_e4915) - (assign7300_e4910 * (-((2.0 * locals.var_t1__blk120_dn17) / (locals.var_t1__blk120 * locals.var_t1__blk120))))) / (assign7300_e4915 * assign7300_e4915)),)
    } else {
        (locals.var_psb_inib__blk124, locals.var_psb_inib__blk124_dn0, locals.var_psb_inib__blk124_dn2, locals.var_psb_inib__blk124_dn6, locals.var_psb_inib__blk124_dn7, locals.var_psb_inib__blk124_dn10, locals.var_psb_inib__blk124_dn11, locals.var_psb_inib__blk124_dn12, locals.var_psb_inib__blk124_dn17,)
    }
};
        locals.var_psb_inib__blk124 = assign7300_e4918;
        locals.var_psb_inib__blk124_dn0 = assign7300_e4918_d_n0;
        locals.var_psb_inib__blk124_dn2 = assign7300_e4918_d_n2;
        locals.var_psb_inib__blk124_dn6 = assign7300_e4918_d_n6;
        locals.var_psb_inib__blk124_dn7 = assign7300_e4918_d_n7;
        locals.var_psb_inib__blk124_dn10 = assign7300_e4918_d_n10;
        locals.var_psb_inib__blk124_dn11 = assign7300_e4918_d_n11;
        locals.var_psb_inib__blk124_dn12 = assign7300_e4918_d_n12;
        locals.var_psb_inib__blk124_dn17 = assign7300_e4918_d_n17;

        let assign7310_e4921: f64 = if locals.var_psb_inia__blk123 < locals.var_pb2_bulk { 1.0 } else { 0.0 };
        locals.var_guard125 = assign7310_e4921;

        let (assign7320_e4927, assign7320_e4927_d_n0, assign7320_e4927_d_n2, assign7320_e4927_d_n6, assign7320_e4927_d_n7, assign7320_e4927_d_n10, assign7320_e4927_d_n11, assign7320_e4927_d_n12, assign7320_e4927_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard125 != 0.0)) {
        (locals.var_psb_inia__blk123, locals.var_psb_inia__blk123_dn0, locals.var_psb_inia__blk123_dn2, locals.var_psb_inia__blk123_dn6, locals.var_psb_inia__blk123_dn7, locals.var_psb_inia__blk123_dn10, locals.var_psb_inia__blk123_dn11, locals.var_psb_inia__blk123_dn12, locals.var_psb_inia__blk123_dn17,)
    } else {
        (locals.var_phi_s0_bulk_0, locals.var_phi_s0_bulk_0_dn0, locals.var_phi_s0_bulk_0_dn2, locals.var_phi_s0_bulk_0_dn6, locals.var_phi_s0_bulk_0_dn7, locals.var_phi_s0_bulk_0_dn10, locals.var_phi_s0_bulk_0_dn11, locals.var_phi_s0_bulk_0_dn12, locals.var_phi_s0_bulk_0_dn17,)
    }
};
        locals.var_phi_s0_bulk_0 = assign7320_e4927;
        locals.var_phi_s0_bulk_0_dn0 = assign7320_e4927_d_n0;
        locals.var_phi_s0_bulk_0_dn2 = assign7320_e4927_d_n2;
        locals.var_phi_s0_bulk_0_dn6 = assign7320_e4927_d_n6;
        locals.var_phi_s0_bulk_0_dn7 = assign7320_e4927_d_n7;
        locals.var_phi_s0_bulk_0_dn10 = assign7320_e4927_d_n10;
        locals.var_phi_s0_bulk_0_dn11 = assign7320_e4927_d_n11;
        locals.var_phi_s0_bulk_0_dn12 = assign7320_e4927_d_n12;
        locals.var_phi_s0_bulk_0_dn17 = assign7320_e4927_d_n17;

        let (assign7330_e4938, assign7330_e4938_d_n0, assign7330_e4938_d_n2, assign7330_e4938_d_n6, assign7330_e4938_d_n7, assign7330_e4938_d_n10, assign7330_e4938_d_n11, assign7330_e4938_d_n12, assign7330_e4938_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard125 == 0.0)) {
        let assign7330_e4934: f64 = (locals.var_psb_inib__blk124 - locals.var_psb_inia__blk123);
        let assign7330_e4936: f64 = (assign7330_e4934 - 0.0008);
        (assign7330_e4936, (locals.var_psb_inib__blk124_dn0 - locals.var_psb_inia__blk123_dn0), (locals.var_psb_inib__blk124_dn2 - locals.var_psb_inia__blk123_dn2), (locals.var_psb_inib__blk124_dn6 - locals.var_psb_inia__blk123_dn6), (locals.var_psb_inib__blk124_dn7 - locals.var_psb_inia__blk123_dn7), (locals.var_psb_inib__blk124_dn10 - locals.var_psb_inia__blk123_dn10), (locals.var_psb_inib__blk124_dn11 - locals.var_psb_inia__blk123_dn11), (locals.var_psb_inib__blk124_dn12 - locals.var_psb_inia__blk123_dn12), (locals.var_psb_inib__blk124_dn17 - locals.var_psb_inia__blk123_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign7330_e4938;
        locals.var_tmf1_dn0 = assign7330_e4938_d_n0;
        locals.var_tmf1_dn2 = assign7330_e4938_d_n2;
        locals.var_tmf1_dn6 = assign7330_e4938_d_n6;
        locals.var_tmf1_dn7 = assign7330_e4938_d_n7;
        locals.var_tmf1_dn10 = assign7330_e4938_d_n10;
        locals.var_tmf1_dn11 = assign7330_e4938_d_n11;
        locals.var_tmf1_dn12 = assign7330_e4938_d_n12;
        locals.var_tmf1_dn17 = assign7330_e4938_d_n17;

        let (assign7340_e4949, assign7340_e4949_d_n0, assign7340_e4949_d_n2, assign7340_e4949_d_n6, assign7340_e4949_d_n7, assign7340_e4949_d_n10, assign7340_e4949_d_n11, assign7340_e4949_d_n12, assign7340_e4949_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard125 == 0.0)) {
        let assign7340_e4945: f64 = (4.0 * locals.var_psb_inib__blk124);
        let assign7340_e4947: f64 = (assign7340_e4945 * 0.0008);
        (assign7340_e4947, ((4.0 * locals.var_psb_inib__blk124_dn0) * 0.0008), ((4.0 * locals.var_psb_inib__blk124_dn2) * 0.0008), ((4.0 * locals.var_psb_inib__blk124_dn6) * 0.0008), ((4.0 * locals.var_psb_inib__blk124_dn7) * 0.0008), ((4.0 * locals.var_psb_inib__blk124_dn10) * 0.0008), ((4.0 * locals.var_psb_inib__blk124_dn11) * 0.0008), ((4.0 * locals.var_psb_inib__blk124_dn12) * 0.0008), ((4.0 * locals.var_psb_inib__blk124_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign7340_e4949;
        locals.var_tmf2_dn0 = assign7340_e4949_d_n0;
        locals.var_tmf2_dn2 = assign7340_e4949_d_n2;
        locals.var_tmf2_dn6 = assign7340_e4949_d_n6;
        locals.var_tmf2_dn7 = assign7340_e4949_d_n7;
        locals.var_tmf2_dn10 = assign7340_e4949_d_n10;
        locals.var_tmf2_dn11 = assign7340_e4949_d_n11;
        locals.var_tmf2_dn12 = assign7340_e4949_d_n12;
        locals.var_tmf2_dn17 = assign7340_e4949_d_n17;

        let (assign7350_e4962, assign7350_e4962_d_n0, assign7350_e4962_d_n2, assign7350_e4962_d_n6, assign7350_e4962_d_n7, assign7350_e4962_d_n10, assign7350_e4962_d_n11, assign7350_e4962_d_n12, assign7350_e4962_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard125 == 0.0)) {
        let (assign7350_e4960, assign7350_e4960_d_n0, assign7350_e4960_d_n2, assign7350_e4960_d_n6, assign7350_e4960_d_n7, assign7350_e4960_d_n10, assign7350_e4960_d_n11, assign7350_e4960_d_n12, assign7350_e4960_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign7350_e4959: f64 = (-locals.var_tmf2);
                (assign7350_e4959, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign7350_e4960, assign7350_e4960_d_n0, assign7350_e4960_d_n2, assign7350_e4960_d_n6, assign7350_e4960_d_n7, assign7350_e4960_d_n10, assign7350_e4960_d_n11, assign7350_e4960_d_n12, assign7350_e4960_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign7350_e4962;
        locals.var_tmf2_dn0 = assign7350_e4962_d_n0;
        locals.var_tmf2_dn2 = assign7350_e4962_d_n2;
        locals.var_tmf2_dn6 = assign7350_e4962_d_n6;
        locals.var_tmf2_dn7 = assign7350_e4962_d_n7;
        locals.var_tmf2_dn10 = assign7350_e4962_d_n10;
        locals.var_tmf2_dn11 = assign7350_e4962_d_n11;
        locals.var_tmf2_dn12 = assign7350_e4962_d_n12;
        locals.var_tmf2_dn17 = assign7350_e4962_d_n17;

        let (assign7360_e4974, assign7360_e4974_d_n0, assign7360_e4974_d_n2, assign7360_e4974_d_n6, assign7360_e4974_d_n7, assign7360_e4974_d_n10, assign7360_e4974_d_n11, assign7360_e4974_d_n12, assign7360_e4974_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard125 == 0.0)) {
        let assign7360_e4969: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign7360_e4971: f64 = (assign7360_e4969 + locals.var_tmf2);
        let assign7360_e4972: f64 = (assign7360_e4971).sqrt();
        (assign7360_e4972, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign7360_e4972)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign7360_e4972)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign7360_e4972)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign7360_e4972)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign7360_e4972)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign7360_e4972)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign7360_e4972)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign7360_e4972)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign7360_e4974;
        locals.var_tmf2_dn0 = assign7360_e4974_d_n0;
        locals.var_tmf2_dn2 = assign7360_e4974_d_n2;
        locals.var_tmf2_dn6 = assign7360_e4974_d_n6;
        locals.var_tmf2_dn7 = assign7360_e4974_d_n7;
        locals.var_tmf2_dn10 = assign7360_e4974_d_n10;
        locals.var_tmf2_dn11 = assign7360_e4974_d_n11;
        locals.var_tmf2_dn12 = assign7360_e4974_d_n12;
        locals.var_tmf2_dn17 = assign7360_e4974_d_n17;

        let (assign7370_e4987, assign7370_e4987_d_n0, assign7370_e4987_d_n2, assign7370_e4987_d_n6, assign7370_e4987_d_n7, assign7370_e4987_d_n10, assign7370_e4987_d_n11, assign7370_e4987_d_n12, assign7370_e4987_d_n17,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard125 == 0.0)) {
        let assign7370_e4983: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign7370_e4984: f64 = (0.5 * assign7370_e4983);
        let assign7370_e4985: f64 = (locals.var_psb_inib__blk124 - assign7370_e4984);
        (assign7370_e4985, (locals.var_psb_inib__blk124_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psb_inib__blk124_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psb_inib__blk124_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psb_inib__blk124_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psb_inib__blk124_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psb_inib__blk124_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psb_inib__blk124_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psb_inib__blk124_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_phi_s0_bulk_0, locals.var_phi_s0_bulk_0_dn0, locals.var_phi_s0_bulk_0_dn2, locals.var_phi_s0_bulk_0_dn6, locals.var_phi_s0_bulk_0_dn7, locals.var_phi_s0_bulk_0_dn10, locals.var_phi_s0_bulk_0_dn11, locals.var_phi_s0_bulk_0_dn12, locals.var_phi_s0_bulk_0_dn17,)
    }
};
        locals.var_phi_s0_bulk_0 = assign7370_e4987;
        locals.var_phi_s0_bulk_0_dn0 = assign7370_e4987_d_n0;
        locals.var_phi_s0_bulk_0_dn2 = assign7370_e4987_d_n2;
        locals.var_phi_s0_bulk_0_dn6 = assign7370_e4987_d_n6;
        locals.var_phi_s0_bulk_0_dn7 = assign7370_e4987_d_n7;
        locals.var_phi_s0_bulk_0_dn10 = assign7370_e4987_d_n10;
        locals.var_phi_s0_bulk_0_dn11 = assign7370_e4987_d_n11;
        locals.var_phi_s0_bulk_0_dn12 = assign7370_e4987_d_n12;
        locals.var_phi_s0_bulk_0_dn17 = assign7370_e4987_d_n17;

        let (assign7380_e4991,) = {
    if (locals.var_guard111 != 0.0) {
        (0.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign7380_e4991;

    }
}
