#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_1(
        locals: &mut StampLocals,
    ) {
        locals.var_uc_areabt = 0.0;
        locals.var_uc_areabt_rv = 0.0;

        locals.var_uc_vfbbt = 0.0;
        locals.var_uc_vfbbt_rv = 0.0;

        locals.var_q_bt_ge = 0.0;
        locals.var_q_bt_ge_dn0 = 0.0;
        locals.var_q_bt_ge_dn2 = 0.0;
        locals.var_q_bt_ge_dn6 = 0.0;
        locals.var_q_bt_ge_dn7 = 0.0;
        locals.var_q_bt_ge_dn10 = 0.0;
        locals.var_q_bt_ge_dn11 = 0.0;
        locals.var_q_bt_ge_dn12 = 0.0;
        locals.var_q_bt_ge_dn17 = 0.0;
        locals.var_q_bt_ge_rv = 0.0;

        locals.var_q_bt_se = 0.0;
        locals.var_q_bt_se_dn0 = 0.0;
        locals.var_q_bt_se_dn2 = 0.0;
        locals.var_q_bt_se_dn6 = 0.0;
        locals.var_q_bt_se_dn7 = 0.0;
        locals.var_q_bt_se_dn10 = 0.0;
        locals.var_q_bt_se_dn11 = 0.0;
        locals.var_q_bt_se_dn12 = 0.0;
        locals.var_q_bt_se_dn17 = 0.0;
        locals.var_q_bt_se_rv = 0.0;

        locals.var_mud_hoso = 0.0;
        locals.var_mud_hoso_dn0 = 0.0;
        locals.var_mud_hoso_dn2 = 0.0;
        locals.var_mud_hoso_dn6 = 0.0;
        locals.var_mud_hoso_dn7 = 0.0;
        locals.var_mud_hoso_dn10 = 0.0;
        locals.var_mud_hoso_dn11 = 0.0;
        locals.var_mud_hoso_dn12 = 0.0;
        locals.var_mud_hoso_dn17 = 0.0;
        locals.var_mud_hoso_rv = 0.0;

        locals.var_kusai00 = 0.0;
        locals.var_kusai00_dn0 = 0.0;
        locals.var_kusai00_dn2 = 0.0;
        locals.var_kusai00_dn6 = 0.0;
        locals.var_kusai00_dn7 = 0.0;
        locals.var_kusai00_dn10 = 0.0;
        locals.var_kusai00_dn11 = 0.0;
        locals.var_kusai00_dn12 = 0.0;
        locals.var_kusai00_dn17 = 0.0;
        locals.var_kusai00_rv = 0.0;

        locals.var_kusail = 0.0;
        locals.var_kusail_dn0 = 0.0;
        locals.var_kusail_dn2 = 0.0;
        locals.var_kusail_dn6 = 0.0;
        locals.var_kusail_dn7 = 0.0;
        locals.var_kusail_dn10 = 0.0;
        locals.var_kusail_dn11 = 0.0;
        locals.var_kusail_dn12 = 0.0;
        locals.var_kusail_dn17 = 0.0;
        locals.var_kusail_rv = 0.0;

        locals.var_kusai00l = 0.0;
        locals.var_kusai00l_dn0 = 0.0;
        locals.var_kusai00l_dn2 = 0.0;
        locals.var_kusai00l_dn6 = 0.0;
        locals.var_kusai00l_dn7 = 0.0;
        locals.var_kusai00l_dn10 = 0.0;
        locals.var_kusai00l_dn11 = 0.0;
        locals.var_kusai00l_dn12 = 0.0;
        locals.var_kusai00l_dn17 = 0.0;
        locals.var_kusai00l_rv = 0.0;

        locals.var_sqrtkusail = 0.0;
        locals.var_sqrtkusail_dn0 = 0.0;
        locals.var_sqrtkusail_dn2 = 0.0;
        locals.var_sqrtkusail_dn6 = 0.0;
        locals.var_sqrtkusail_dn7 = 0.0;
        locals.var_sqrtkusail_dn10 = 0.0;
        locals.var_sqrtkusail_dn11 = 0.0;
        locals.var_sqrtkusail_dn12 = 0.0;
        locals.var_sqrtkusail_dn17 = 0.0;
        locals.var_sqrtkusail_rv = 0.0;

        locals.var_kusai_ig = 0.0;
        locals.var_kusai_ig_dn0 = 0.0;
        locals.var_kusai_ig_dn2 = 0.0;
        locals.var_kusai_ig_dn6 = 0.0;
        locals.var_kusai_ig_dn7 = 0.0;
        locals.var_kusai_ig_dn10 = 0.0;
        locals.var_kusai_ig_dn11 = 0.0;
        locals.var_kusai_ig_dn12 = 0.0;
        locals.var_kusai_ig_dn17 = 0.0;
        locals.var_kusai_ig_rv = 0.0;

        locals.var_psdl = 0.0;
        locals.var_psdl_dn0 = 0.0;
        locals.var_psdl_dn2 = 0.0;
        locals.var_psdl_dn6 = 0.0;
        locals.var_psdl_dn7 = 0.0;
        locals.var_psdl_dn10 = 0.0;
        locals.var_psdl_dn11 = 0.0;
        locals.var_psdl_dn12 = 0.0;
        locals.var_psdl_dn17 = 0.0;
        locals.var_psdl_rv = 0.0;

        locals.var_ec = 0.0;
        locals.var_ec_dn0 = 0.0;
        locals.var_ec_dn2 = 0.0;
        locals.var_ec_dn6 = 0.0;
        locals.var_ec_dn7 = 0.0;
        locals.var_ec_dn10 = 0.0;
        locals.var_ec_dn11 = 0.0;
        locals.var_ec_dn12 = 0.0;
        locals.var_ec_dn17 = 0.0;
        locals.var_ec_rv = 0.0;

        locals.var_lred = 0.0;
        locals.var_lred_dn0 = 0.0;
        locals.var_lred_dn2 = 0.0;
        locals.var_lred_dn6 = 0.0;
        locals.var_lred_dn7 = 0.0;
        locals.var_lred_dn10 = 0.0;
        locals.var_lred_dn11 = 0.0;
        locals.var_lred_dn12 = 0.0;
        locals.var_lred_dn17 = 0.0;
        locals.var_lred_rv = 0.0;

        locals.var_flg_depmode = 0.0;
        locals.var_flg_depmode_rv = 0.0;

        locals.var_phi_sl_soi_ini = 0.0;
        locals.var_phi_sl_soi_ini_dn0 = 0.0;
        locals.var_phi_sl_soi_ini_dn2 = 0.0;
        locals.var_phi_sl_soi_ini_dn6 = 0.0;
        locals.var_phi_sl_soi_ini_dn7 = 0.0;
        locals.var_phi_sl_soi_ini_dn10 = 0.0;
        locals.var_phi_sl_soi_ini_dn11 = 0.0;
        locals.var_phi_sl_soi_ini_dn12 = 0.0;
        locals.var_phi_sl_soi_ini_dn17 = 0.0;
        locals.var_phi_sl_soi_ini_rv = 0.0;

        locals.var_phi_bl_soi_ini = 0.0;
        locals.var_phi_bl_soi_ini_dn0 = 0.0;
        locals.var_phi_bl_soi_ini_dn2 = 0.0;
        locals.var_phi_bl_soi_ini_dn6 = 0.0;
        locals.var_phi_bl_soi_ini_dn7 = 0.0;
        locals.var_phi_bl_soi_ini_dn10 = 0.0;
        locals.var_phi_bl_soi_ini_dn11 = 0.0;
        locals.var_phi_bl_soi_ini_dn12 = 0.0;
        locals.var_phi_bl_soi_ini_dn17 = 0.0;
        locals.var_phi_bl_soi_ini_rv = 0.0;

        locals.var_phi_sl_bulk_ini = 0.0;
        locals.var_phi_sl_bulk_ini_dn0 = 0.0;
        locals.var_phi_sl_bulk_ini_dn2 = 0.0;
        locals.var_phi_sl_bulk_ini_dn6 = 0.0;
        locals.var_phi_sl_bulk_ini_dn7 = 0.0;
        locals.var_phi_sl_bulk_ini_dn10 = 0.0;
        locals.var_phi_sl_bulk_ini_dn11 = 0.0;
        locals.var_phi_sl_bulk_ini_dn12 = 0.0;
        locals.var_phi_sl_bulk_ini_dn17 = 0.0;
        locals.var_phi_sl_bulk_ini_rv = 0.0;

        locals.var_phi_s0_soi = 0.0;
        locals.var_phi_s0_soi_dn0 = 0.0;
        locals.var_phi_s0_soi_dn2 = 0.0;
        locals.var_phi_s0_soi_dn6 = 0.0;
        locals.var_phi_s0_soi_dn7 = 0.0;
        locals.var_phi_s0_soi_dn10 = 0.0;
        locals.var_phi_s0_soi_dn11 = 0.0;
        locals.var_phi_s0_soi_dn12 = 0.0;
        locals.var_phi_s0_soi_dn17 = 0.0;
        locals.var_phi_s0_soi_rv = 0.0;

        locals.var_phi_b0_soi = 0.0;
        locals.var_phi_b0_soi_dn0 = 0.0;
        locals.var_phi_b0_soi_dn2 = 0.0;
        locals.var_phi_b0_soi_dn6 = 0.0;
        locals.var_phi_b0_soi_dn7 = 0.0;
        locals.var_phi_b0_soi_dn10 = 0.0;
        locals.var_phi_b0_soi_dn11 = 0.0;
        locals.var_phi_b0_soi_dn12 = 0.0;
        locals.var_phi_b0_soi_dn17 = 0.0;
        locals.var_phi_b0_soi_rv = 0.0;

        locals.var_phi_s0_bulk = 0.0;
        locals.var_phi_s0_bulk_dn0 = 0.0;
        locals.var_phi_s0_bulk_dn2 = 0.0;
        locals.var_phi_s0_bulk_dn6 = 0.0;
        locals.var_phi_s0_bulk_dn7 = 0.0;
        locals.var_phi_s0_bulk_dn10 = 0.0;
        locals.var_phi_s0_bulk_dn11 = 0.0;
        locals.var_phi_s0_bulk_dn12 = 0.0;
        locals.var_phi_s0_bulk_dn17 = 0.0;
        locals.var_phi_s0_bulk_rv = 0.0;

        locals.var_phi_sl_soi = 0.0;
        locals.var_phi_sl_soi_dn0 = 0.0;
        locals.var_phi_sl_soi_dn2 = 0.0;
        locals.var_phi_sl_soi_dn6 = 0.0;
        locals.var_phi_sl_soi_dn7 = 0.0;
        locals.var_phi_sl_soi_dn10 = 0.0;
        locals.var_phi_sl_soi_dn11 = 0.0;
        locals.var_phi_sl_soi_dn12 = 0.0;
        locals.var_phi_sl_soi_dn17 = 0.0;
        locals.var_phi_sl_soi_rv = 0.0;

        locals.var_phi_bl_soi = 0.0;
        locals.var_phi_bl_soi_dn0 = 0.0;
        locals.var_phi_bl_soi_dn2 = 0.0;
        locals.var_phi_bl_soi_dn6 = 0.0;
        locals.var_phi_bl_soi_dn7 = 0.0;
        locals.var_phi_bl_soi_dn10 = 0.0;
        locals.var_phi_bl_soi_dn11 = 0.0;
        locals.var_phi_bl_soi_dn12 = 0.0;
        locals.var_phi_bl_soi_dn17 = 0.0;
        locals.var_phi_bl_soi_rv = 0.0;

        locals.var_phi_sl_bulk = 0.0;
        locals.var_phi_sl_bulk_dn0 = 0.0;
        locals.var_phi_sl_bulk_dn2 = 0.0;
        locals.var_phi_sl_bulk_dn6 = 0.0;
        locals.var_phi_sl_bulk_dn7 = 0.0;
        locals.var_phi_sl_bulk_dn10 = 0.0;
        locals.var_phi_sl_bulk_dn11 = 0.0;
        locals.var_phi_sl_bulk_dn12 = 0.0;
        locals.var_phi_sl_bulk_dn17 = 0.0;
        locals.var_phi_sl_bulk_rv = 0.0;

        locals.var_q_dep_soi = 0.0;
        locals.var_q_dep_soi_dn0 = 0.0;
        locals.var_q_dep_soi_dn2 = 0.0;
        locals.var_q_dep_soi_dn6 = 0.0;
        locals.var_q_dep_soi_dn7 = 0.0;
        locals.var_q_dep_soi_dn10 = 0.0;
        locals.var_q_dep_soi_dn11 = 0.0;
        locals.var_q_dep_soi_dn12 = 0.0;
        locals.var_q_dep_soi_dn17 = 0.0;
        locals.var_q_dep_soi_rv = 0.0;

        locals.var_q_n0 = 0.0;
        locals.var_q_n0_dn0 = 0.0;
        locals.var_q_n0_dn2 = 0.0;
        locals.var_q_n0_dn6 = 0.0;
        locals.var_q_n0_dn7 = 0.0;
        locals.var_q_n0_dn10 = 0.0;
        locals.var_q_n0_dn11 = 0.0;
        locals.var_q_n0_dn12 = 0.0;
        locals.var_q_n0_dn17 = 0.0;
        locals.var_q_n0_rv = 0.0;

        locals.var_q_b0_dep = 0.0;
        locals.var_q_b0_dep_dn0 = 0.0;
        locals.var_q_b0_dep_dn2 = 0.0;
        locals.var_q_b0_dep_dn6 = 0.0;
        locals.var_q_b0_dep_dn7 = 0.0;
        locals.var_q_b0_dep_dn10 = 0.0;
        locals.var_q_b0_dep_dn11 = 0.0;
        locals.var_q_b0_dep_dn12 = 0.0;
        locals.var_q_b0_dep_dn17 = 0.0;
        locals.var_q_b0_dep_rv = 0.0;

        locals.var_q_bl_dep = 0.0;
        locals.var_q_bl_dep_dn0 = 0.0;
        locals.var_q_bl_dep_dn2 = 0.0;
        locals.var_q_bl_dep_dn6 = 0.0;
        locals.var_q_bl_dep_dn7 = 0.0;
        locals.var_q_bl_dep_dn10 = 0.0;
        locals.var_q_bl_dep_dn11 = 0.0;
        locals.var_q_bl_dep_dn12 = 0.0;
        locals.var_q_bl_dep_dn17 = 0.0;
        locals.var_q_bl_dep_rv = 0.0;

        locals.var_q_dep0 = 0.0;
        locals.var_q_dep0_dn0 = 0.0;
        locals.var_q_dep0_dn2 = 0.0;
        locals.var_q_dep0_dn6 = 0.0;
        locals.var_q_dep0_dn7 = 0.0;
        locals.var_q_dep0_dn10 = 0.0;
        locals.var_q_dep0_dn11 = 0.0;
        locals.var_q_dep0_dn12 = 0.0;
        locals.var_q_dep0_dn17 = 0.0;
        locals.var_q_dep0_rv = 0.0;

        locals.var_q_s0_bulk = 0.0;
        locals.var_q_s0_bulk_dn0 = 0.0;
        locals.var_q_s0_bulk_dn2 = 0.0;
        locals.var_q_s0_bulk_dn6 = 0.0;
        locals.var_q_s0_bulk_dn7 = 0.0;
        locals.var_q_s0_bulk_dn10 = 0.0;
        locals.var_q_s0_bulk_dn11 = 0.0;
        locals.var_q_s0_bulk_dn12 = 0.0;
        locals.var_q_s0_bulk_dn17 = 0.0;
        locals.var_q_s0_bulk_rv = 0.0;

        locals.var_q_nl = 0.0;
        locals.var_q_nl_dn0 = 0.0;
        locals.var_q_nl_dn2 = 0.0;
        locals.var_q_nl_dn6 = 0.0;
        locals.var_q_nl_dn7 = 0.0;
        locals.var_q_nl_dn10 = 0.0;
        locals.var_q_nl_dn11 = 0.0;
        locals.var_q_nl_dn12 = 0.0;
        locals.var_q_nl_dn17 = 0.0;
        locals.var_q_nl_rv = 0.0;

        locals.var_q_depl = 0.0;
        locals.var_q_depl_dn0 = 0.0;
        locals.var_q_depl_dn2 = 0.0;
        locals.var_q_depl_dn6 = 0.0;
        locals.var_q_depl_dn7 = 0.0;
        locals.var_q_depl_dn10 = 0.0;
        locals.var_q_depl_dn11 = 0.0;
        locals.var_q_depl_dn12 = 0.0;
        locals.var_q_depl_dn17 = 0.0;
        locals.var_q_depl_rv = 0.0;

        locals.var_q_sl_bulk = 0.0;
        locals.var_q_sl_bulk_dn0 = 0.0;
        locals.var_q_sl_bulk_dn2 = 0.0;
        locals.var_q_sl_bulk_dn6 = 0.0;
        locals.var_q_sl_bulk_dn7 = 0.0;
        locals.var_q_sl_bulk_dn10 = 0.0;
        locals.var_q_sl_bulk_dn11 = 0.0;
        locals.var_q_sl_bulk_dn12 = 0.0;
        locals.var_q_sl_bulk_dn17 = 0.0;
        locals.var_q_sl_bulk_rv = 0.0;

        locals.var_shift = 0.0;
        locals.var_shift_dn0 = 0.0;
        locals.var_shift_dn2 = 0.0;
        locals.var_shift_dn6 = 0.0;
        locals.var_shift_dn7 = 0.0;
        locals.var_shift_dn10 = 0.0;
        locals.var_shift_dn11 = 0.0;
        locals.var_shift_dn12 = 0.0;
        locals.var_shift_dn17 = 0.0;
        locals.var_shift_rv = 0.0;

        locals.var_q_s0_bulk_0 = 0.0;
        locals.var_q_s0_bulk_0_dn0 = 0.0;
        locals.var_q_s0_bulk_0_dn2 = 0.0;
        locals.var_q_s0_bulk_0_dn6 = 0.0;
        locals.var_q_s0_bulk_0_dn7 = 0.0;
        locals.var_q_s0_bulk_0_dn10 = 0.0;
        locals.var_q_s0_bulk_0_dn11 = 0.0;
        locals.var_q_s0_bulk_0_dn12 = 0.0;
        locals.var_q_s0_bulk_0_dn17 = 0.0;
        locals.var_q_s0_bulk_0_rv = 0.0;

        locals.var_qi_nqs = 0.0;
        locals.var_qi_nqs_dn18 = 0.0;
        locals.var_qi_nqs_rv = 0.0;

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
        locals.var_qd_nqs_rv = 0.0;

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
        locals.var_qs_nqs_rv = 0.0;

        locals.var_phi_b_dep0 = 0.0;
        locals.var_phi_b_dep0_dn0 = 0.0;
        locals.var_phi_b_dep0_dn2 = 0.0;
        locals.var_phi_b_dep0_dn6 = 0.0;
        locals.var_phi_b_dep0_dn7 = 0.0;
        locals.var_phi_b_dep0_dn10 = 0.0;
        locals.var_phi_b_dep0_dn11 = 0.0;
        locals.var_phi_b_dep0_dn12 = 0.0;
        locals.var_phi_b_dep0_dn17 = 0.0;
        locals.var_phi_b_dep0_rv = 0.0;

        locals.var_qsub = 0.0;
        locals.var_qsub_dn0 = 0.0;
        locals.var_qsub_dn2 = 0.0;
        locals.var_qsub_dn6 = 0.0;
        locals.var_qsub_dn7 = 0.0;
        locals.var_qsub_dn10 = 0.0;
        locals.var_qsub_dn11 = 0.0;
        locals.var_qsub_dn12 = 0.0;
        locals.var_qsub_dn17 = 0.0;
        locals.var_qsub_rv = 0.0;

        locals.var_qhs = 0.0;
        locals.var_qhs_dn0 = 0.0;
        locals.var_qhs_dn2 = 0.0;
        locals.var_qhs_dn6 = 0.0;
        locals.var_qhs_dn7 = 0.0;
        locals.var_qhs_dn10 = 0.0;
        locals.var_qhs_dn11 = 0.0;
        locals.var_qhs_dn12 = 0.0;
        locals.var_qhs_dn17 = 0.0;
        locals.var_qhs_rv = 0.0;

        locals.var_wdsoi = 0.0;
        locals.var_wdsoi_dn0 = 0.0;
        locals.var_wdsoi_dn2 = 0.0;
        locals.var_wdsoi_dn6 = 0.0;
        locals.var_wdsoi_dn7 = 0.0;
        locals.var_wdsoi_dn10 = 0.0;
        locals.var_wdsoi_dn11 = 0.0;
        locals.var_wdsoi_dn12 = 0.0;
        locals.var_wdsoi_dn17 = 0.0;
        locals.var_wdsoi_rv = 0.0;

        locals.var_ps0_inia = 0.0;
        locals.var_ps0_inia_dn0 = 0.0;
        locals.var_ps0_inia_dn2 = 0.0;
        locals.var_ps0_inia_dn6 = 0.0;
        locals.var_ps0_inia_dn7 = 0.0;
        locals.var_ps0_inia_dn10 = 0.0;
        locals.var_ps0_inia_dn11 = 0.0;
        locals.var_ps0_inia_dn12 = 0.0;
        locals.var_ps0_inia_dn17 = 0.0;
        locals.var_ps0_inia_rv = 0.0;

        locals.var_qiu = 0.0;
        locals.var_qiu_dn0 = 0.0;
        locals.var_qiu_dn2 = 0.0;
        locals.var_qiu_dn6 = 0.0;
        locals.var_qiu_dn7 = 0.0;
        locals.var_qiu_dn10 = 0.0;
        locals.var_qiu_dn11 = 0.0;
        locals.var_qiu_dn12 = 0.0;
        locals.var_qiu_dn17 = 0.0;
        locals.var_qiu_rv = 0.0;

        locals.var_qbu = 0.0;
        locals.var_qbu_dn0 = 0.0;
        locals.var_qbu_dn2 = 0.0;
        locals.var_qbu_dn6 = 0.0;
        locals.var_qbu_dn7 = 0.0;
        locals.var_qbu_dn10 = 0.0;
        locals.var_qbu_dn11 = 0.0;
        locals.var_qbu_dn12 = 0.0;
        locals.var_qbu_dn17 = 0.0;
        locals.var_qbu_rv = 0.0;

        locals.var_qdrat = 0.5;
        locals.var_qdrat_dn0 = 0.0;
        locals.var_qdrat_dn2 = 0.0;
        locals.var_qdrat_dn6 = 0.0;
        locals.var_qdrat_dn7 = 0.0;
        locals.var_qdrat_dn10 = 0.0;
        locals.var_qdrat_dn11 = 0.0;
        locals.var_qdrat_dn12 = 0.0;
        locals.var_qdrat_dn17 = 0.0;
        locals.var_qdrat_rv = 0.0;

        locals.var_qdrat_noi = 0.5;
        locals.var_qdrat_noi_dn0 = 0.0;
        locals.var_qdrat_noi_dn2 = 0.0;
        locals.var_qdrat_noi_dn6 = 0.0;
        locals.var_qdrat_noi_dn7 = 0.0;
        locals.var_qdrat_noi_dn10 = 0.0;
        locals.var_qdrat_noi_dn11 = 0.0;
        locals.var_qdrat_noi_dn12 = 0.0;
        locals.var_qdrat_noi_dn17 = 0.0;
        locals.var_qdrat_noi_rv = 0.0;

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
        locals.var_qs_fb_rv = 0.0;

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
        locals.var_qd_fb_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        locals.var_fs01 = 0.0;
        locals.var_fs01_dn0 = 0.0;
        locals.var_fs01_dn2 = 0.0;
        locals.var_fs01_dn6 = 0.0;
        locals.var_fs01_dn7 = 0.0;
        locals.var_fs01_dn10 = 0.0;
        locals.var_fs01_dn11 = 0.0;
        locals.var_fs01_dn12 = 0.0;
        locals.var_fs01_dn17 = 0.0;
        locals.var_fs01_rv = 0.0;

        locals.var_fs02 = 0.0;
        locals.var_fs02_dn0 = 0.0;
        locals.var_fs02_dn2 = 0.0;
        locals.var_fs02_dn6 = 0.0;
        locals.var_fs02_dn7 = 0.0;
        locals.var_fs02_dn10 = 0.0;
        locals.var_fs02_dn11 = 0.0;
        locals.var_fs02_dn12 = 0.0;
        locals.var_fs02_dn17 = 0.0;
        locals.var_fs02_rv = 0.0;

        locals.var_fsl1 = 0.0;
        locals.var_fsl1_dn0 = 0.0;
        locals.var_fsl1_dn2 = 0.0;
        locals.var_fsl1_dn6 = 0.0;
        locals.var_fsl1_dn7 = 0.0;
        locals.var_fsl1_dn10 = 0.0;
        locals.var_fsl1_dn11 = 0.0;
        locals.var_fsl1_dn12 = 0.0;
        locals.var_fsl1_dn17 = 0.0;
        locals.var_fsl1_rv = 0.0;

        locals.var_fsl2 = 0.0;
        locals.var_fsl2_dn0 = 0.0;
        locals.var_fsl2_dn2 = 0.0;
        locals.var_fsl2_dn6 = 0.0;
        locals.var_fsl2_dn7 = 0.0;
        locals.var_fsl2_dn10 = 0.0;
        locals.var_fsl2_dn11 = 0.0;
        locals.var_fsl2_dn12 = 0.0;
        locals.var_fsl2_dn17 = 0.0;
        locals.var_fsl2_rv = 0.0;

        let assign1200_e968: f64 = (p.p51 * 10.0);
        let assign1200_e970: f64 = (assign1200_e968 % 10.0);
        locals.var_subversion = assign1200_e970;
        locals.var_subversion_rv = 0.0;

        locals.var_lp_s0_max = 200.0;
        locals.var_lp_s0_max_rv = 0.0;

        locals.var_lp_sl_max = 200.0;
        locals.var_lp_sl_max_rv = 0.0;

        locals.var_flg_skipacc = 0.0;
        locals.var_flg_skipacc_rv = 0.0;

        locals.var_vbsbiz = 0.0;
        locals.var_vbsbiz_dn0 = 0.0;
        locals.var_vbsbiz_dn2 = 0.0;
        locals.var_vbsbiz_dn6 = 0.0;
        locals.var_vbsbiz_dn7 = 0.0;
        locals.var_vbsbiz_dn10 = 0.0;
        locals.var_vbsbiz_dn11 = 0.0;
        locals.var_vbsbiz_dn12 = 0.0;
        locals.var_vbsbiz_dn17 = 0.0;
        locals.var_vbsbiz_rv = 0.0;

        locals.var_ps0_ini = 0.0;
        locals.var_ps0_ini_dn0 = 0.0;
        locals.var_ps0_ini_dn2 = 0.0;
        locals.var_ps0_ini_dn6 = 0.0;
        locals.var_ps0_ini_dn7 = 0.0;
        locals.var_ps0_ini_dn10 = 0.0;
        locals.var_ps0_ini_dn11 = 0.0;
        locals.var_ps0_ini_dn12 = 0.0;
        locals.var_ps0_ini_dn17 = 0.0;
        locals.var_ps0_ini_rv = 0.0;

        locals.var_q_s0_dep_ini = 0.0;
        locals.var_q_s0_dep_ini_dn0 = 0.0;
        locals.var_q_s0_dep_ini_dn2 = 0.0;
        locals.var_q_s0_dep_ini_dn6 = 0.0;
        locals.var_q_s0_dep_ini_dn7 = 0.0;
        locals.var_q_s0_dep_ini_dn10 = 0.0;
        locals.var_q_s0_dep_ini_dn11 = 0.0;
        locals.var_q_s0_dep_ini_dn12 = 0.0;
        locals.var_q_s0_dep_ini_dn17 = 0.0;
        locals.var_q_s0_dep_ini_rv = 0.0;

        locals.var_idspt0 = 0.0;
        locals.var_idspt0_dn0 = 0.0;
        locals.var_idspt0_dn2 = 0.0;
        locals.var_idspt0_dn6 = 0.0;
        locals.var_idspt0_dn7 = 0.0;
        locals.var_idspt0_dn10 = 0.0;
        locals.var_idspt0_dn11 = 0.0;
        locals.var_idspt0_dn12 = 0.0;
        locals.var_idspt0_dn17 = 0.0;
        locals.var_idspt0_rv = 0.0;

        locals.var_ps0 = 0.0;
        locals.var_ps0_dn0 = 0.0;
        locals.var_ps0_dn2 = 0.0;
        locals.var_ps0_dn6 = 0.0;
        locals.var_ps0_dn7 = 0.0;
        locals.var_ps0_dn10 = 0.0;
        locals.var_ps0_dn11 = 0.0;
        locals.var_ps0_dn12 = 0.0;
        locals.var_ps0_dn17 = 0.0;
        locals.var_ps0_rv = 0.0;

        locals.var_vbcs_cl = 0.0;
        locals.var_vbcs_cl_dn0 = 0.0;
        locals.var_vbcs_cl_dn2 = 0.0;
        locals.var_vbcs_cl_dn6 = 0.0;
        locals.var_vbcs_cl_dn7 = 0.0;
        locals.var_vbcs_cl_dn10 = 0.0;
        locals.var_vbcs_cl_dn11 = 0.0;
        locals.var_vbcs_cl_dn12 = 0.0;
        locals.var_vbcs_cl_dn17 = 0.0;
        locals.var_vbcs_cl_rv = 0.0;

        let assign1310_e983: f64 = (p.p52 * 0.01);
        locals.var_mks_vmax = assign1310_e983;
        locals.var_mks_vmax_rv = 0.0;

        let assign1320_e986: f64 = (p.p73 / 1e-6);
        locals.var_mks_nsubp = assign1320_e986;
        locals.var_mks_nsubp_rv = 0.0;

        let assign1330_e989: f64 = (p.p104 * 0.01);
        locals.var_mks_vtmp = assign1330_e989;
        locals.var_mks_vtmp_rv = 0.0;

        let assign1340_e992: f64 = (p.p201 / 1e-6);
        locals.var_mks_nsubcmax = assign1340_e992;
        locals.var_mks_nsubcmax_rv = 0.0;

        let assign1380_e1004: f64 = (p.p240 / 1e-6);
        locals.var_mks_nsubs = assign1380_e1004;
        locals.var_mks_nsubs_rv = 0.0;

        let assign1390_e1007: f64 = (p.p241 / 1e-6);
        locals.var_mks_nsubb = assign1390_e1007;
        locals.var_mks_nsubb_rv = 0.0;

        let assign1400_e1010: f64 = (p.p242 * 0.01);
        locals.var_mks_rth0 = assign1400_e1010;
        locals.var_mks_rth0_rv = 0.0;

        let assign1410_e1013: f64 = (p.p243 / 0.01);
        locals.var_mks_cth0 = assign1410_e1013;
        locals.var_mks_cth0_rv = 0.0;

        let assign1420_e1016: f64 = (p.p59 / 1e-6);
        locals.var_mks_nover = assign1420_e1016;
        locals.var_mks_nover_rv = 0.0;

        let assign1430_e1019: f64 = (p.p284 / 1e-6);
        locals.var_mks_njunc = assign1430_e1019;
        locals.var_mks_njunc_rv = 0.0;

        let assign1440_e1022: f64 = (p.p148 / 1e-6);
        locals.var_mks_nsti = assign1440_e1022;
        locals.var_mks_nsti_rv = 0.0;

        let assign1450_e1025: f64 = (p.p198 / 0.0001);
        locals.var_mks_wfc = assign1450_e1025;
        locals.var_mks_wfc_rv = 0.0;

        let assign1460_e1028: f64 = (p.p70 * 0.01);
        locals.var_mks_parl1 = assign1460_e1028;
        locals.var_mks_parl1_rv = 0.0;

        let (assign1470_e1034,) = {
    if (p.p83 == 0.0) {
        (0.0,)
    } else {
        (p.p84,)
    }
};
        locals.var_uc_sc2 = assign1470_e1034;
        locals.var_uc_sc2_rv = 0.0;

        let (assign1480_e1040,) = {
    if (p.p83 == 0.0) {
        (0.0,)
    } else {
        (p.p85,)
    }
};
        locals.var_uc_sc3 = assign1480_e1040;
        locals.var_uc_sc3_rv = 0.0;

        let (assign1490_e1046,) = {
    if (p.p80 == 0.0) {
        (0.0,)
    } else {
        (p.p81,)
    }
};
        locals.var_uc_scp2 = assign1490_e1046;
        locals.var_uc_scp2_rv = 0.0;

        let (assign1500_e1052,) = {
    if (p.p83 == 0.0) {
        (0.0,)
    } else {
        (p.p82,)
    }
};
        locals.var_uc_scp3 = assign1500_e1052;
        locals.var_uc_scp3_rv = 0.0;

        let assign1510_e1055: f64 = (p.p250 * 1000000.0);
        locals.var_uc_gdld = assign1510_e1055;
        locals.var_uc_gdld_rv = 0.0;

        let assign1520_e1058: f64 = (p.p232 + 273.15);
        locals.var_uc_tnom = assign1520_e1058;
        locals.var_uc_tnom_rv = 0.0;

        locals.var_uc_vfbover = p.p58;
        locals.var_uc_vfbover_rv = 0.0;

        locals.var_flg_info = p.p46;
        locals.var_flg_info_rv = 0.0;

        locals.var_flg_nqs = p.p34;
        locals.var_flg_nqs_rv = 0.0;

        let (assign1570_e1073,) = {
    if param_given[190] {
        (p.p190,)
    } else {
        let assign1570_e1071: f64 = (p.p237 * p.p240);
        let assign1570_e1072: f64 = (5000000000.0 / assign1570_e1071);
        (assign1570_e1072,)
    }
};
        locals.var_uc_clm2 = assign1570_e1073;
        locals.var_uc_clm2_dn0 = 0.0;
        locals.var_uc_clm2_dn2 = 0.0;
        locals.var_uc_clm2_dn6 = 0.0;
        locals.var_uc_clm2_dn7 = 0.0;
        locals.var_uc_clm2_dn10 = 0.0;
        locals.var_uc_clm2_dn11 = 0.0;
        locals.var_uc_clm2_dn12 = 0.0;
        locals.var_uc_clm2_dn17 = 0.0;
        locals.var_uc_clm2_rv = 0.0;

        let assign1580_e1077: f64 = (2.0 + 0.1);
        let assign1580_e1082: f64 = if ((locals.var_uc_clm2 < assign1580_e1077) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2 = assign1580_e1082;
        locals.var_guard2_rv = 0.0;

        let (assign1590_e1090, assign1590_e1090_d_n0, assign1590_e1090_d_n2, assign1590_e1090_d_n6, assign1590_e1090_d_n7, assign1590_e1090_d_n10, assign1590_e1090_d_n11, assign1590_e1090_d_n12, assign1590_e1090_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1590_e1086: f64 = (2.0 + 0.1);
        let assign1590_e1088: f64 = (assign1590_e1086 - locals.var_uc_clm2);
        (assign1590_e1088, (-locals.var_uc_clm2_dn0), (-locals.var_uc_clm2_dn2), (-locals.var_uc_clm2_dn6), (-locals.var_uc_clm2_dn7), (-locals.var_uc_clm2_dn10), (-locals.var_uc_clm2_dn11), (-locals.var_uc_clm2_dn12), (-locals.var_uc_clm2_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign1590_e1090;
        locals.var_tmf1_dn0 = assign1590_e1090_d_n0;
        locals.var_tmf1_dn2 = assign1590_e1090_d_n2;
        locals.var_tmf1_dn6 = assign1590_e1090_d_n6;
        locals.var_tmf1_dn7 = assign1590_e1090_d_n7;
        locals.var_tmf1_dn10 = assign1590_e1090_d_n10;
        locals.var_tmf1_dn11 = assign1590_e1090_d_n11;
        locals.var_tmf1_dn12 = assign1590_e1090_d_n12;
        locals.var_tmf1_dn17 = assign1590_e1090_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign1600_e1096, assign1600_e1096_d_n0, assign1600_e1096_d_n2, assign1600_e1096_d_n6, assign1600_e1096_d_n7, assign1600_e1096_d_n10, assign1600_e1096_d_n11, assign1600_e1096_d_n12, assign1600_e1096_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1600_e1094: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign1600_e1094, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign1600_e1096;
        locals.var_x2_dn0 = assign1600_e1096_d_n0;
        locals.var_x2_dn2 = assign1600_e1096_d_n2;
        locals.var_x2_dn6 = assign1600_e1096_d_n6;
        locals.var_x2_dn7 = assign1600_e1096_d_n7;
        locals.var_x2_dn10 = assign1600_e1096_d_n10;
        locals.var_x2_dn11 = assign1600_e1096_d_n11;
        locals.var_x2_dn12 = assign1600_e1096_d_n12;
        locals.var_x2_dn17 = assign1600_e1096_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign1610_e1102, assign1610_e1102_d_n0, assign1610_e1102_d_n2, assign1610_e1102_d_n6, assign1610_e1102_d_n7, assign1610_e1102_d_n10, assign1610_e1102_d_n11, assign1610_e1102_d_n12, assign1610_e1102_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1610_e1100: f64 = (0.1 * 0.1);
        (assign1610_e1100, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign1610_e1102;
        locals.var_xmax2_dn0 = assign1610_e1102_d_n0;
        locals.var_xmax2_dn2 = assign1610_e1102_d_n2;
        locals.var_xmax2_dn6 = assign1610_e1102_d_n6;
        locals.var_xmax2_dn7 = assign1610_e1102_d_n7;
        locals.var_xmax2_dn10 = assign1610_e1102_d_n10;
        locals.var_xmax2_dn11 = assign1610_e1102_d_n11;
        locals.var_xmax2_dn12 = assign1610_e1102_d_n12;
        locals.var_xmax2_dn17 = assign1610_e1102_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign1620_e1106, assign1620_e1106_d_n0, assign1620_e1106_d_n2, assign1620_e1106_d_n6, assign1620_e1106_d_n7, assign1620_e1106_d_n10, assign1620_e1106_d_n11, assign1620_e1106_d_n12, assign1620_e1106_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign1620_e1106;
        locals.var_xp_dn0 = assign1620_e1106_d_n0;
        locals.var_xp_dn2 = assign1620_e1106_d_n2;
        locals.var_xp_dn6 = assign1620_e1106_d_n6;
        locals.var_xp_dn7 = assign1620_e1106_d_n7;
        locals.var_xp_dn10 = assign1620_e1106_d_n10;
        locals.var_xp_dn11 = assign1620_e1106_d_n11;
        locals.var_xp_dn12 = assign1620_e1106_d_n12;
        locals.var_xp_dn17 = assign1620_e1106_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign1630_e1110, assign1630_e1110_d_n0, assign1630_e1110_d_n2, assign1630_e1110_d_n6, assign1630_e1110_d_n7, assign1630_e1110_d_n10, assign1630_e1110_d_n11, assign1630_e1110_d_n12, assign1630_e1110_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign1630_e1110;
        locals.var_xmp_dn0 = assign1630_e1110_d_n0;
        locals.var_xmp_dn2 = assign1630_e1110_d_n2;
        locals.var_xmp_dn6 = assign1630_e1110_d_n6;
        locals.var_xmp_dn7 = assign1630_e1110_d_n7;
        locals.var_xmp_dn10 = assign1630_e1110_d_n10;
        locals.var_xmp_dn11 = assign1630_e1110_d_n11;
        locals.var_xmp_dn12 = assign1630_e1110_d_n12;
        locals.var_xmp_dn17 = assign1630_e1110_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign1640_e1114,) = {
    if (locals.var_guard2 != 0.0) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign1640_e1114;
        locals.var_m0_rv = 0.0;

        let (assign1650_e1118,) = {
    if (locals.var_guard2 != 0.0) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign1650_e1118;
        locals.var_mm_rv = 0.0;

        let (assign1660_e1122, assign1660_e1122_d_n0, assign1660_e1122_d_n2, assign1660_e1122_d_n6, assign1660_e1122_d_n7, assign1660_e1122_d_n10, assign1660_e1122_d_n11, assign1660_e1122_d_n12, assign1660_e1122_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign1660_e1122;
        locals.var_arg_dn0 = assign1660_e1122_d_n0;
        locals.var_arg_dn2 = assign1660_e1122_d_n2;
        locals.var_arg_dn6 = assign1660_e1122_d_n6;
        locals.var_arg_dn7 = assign1660_e1122_d_n7;
        locals.var_arg_dn10 = assign1660_e1122_d_n10;
        locals.var_arg_dn11 = assign1660_e1122_d_n11;
        locals.var_arg_dn12 = assign1660_e1122_d_n12;
        locals.var_arg_dn17 = assign1660_e1122_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign1670_e1126, assign1670_e1126_d_n0, assign1670_e1126_d_n2, assign1670_e1126_d_n6, assign1670_e1126_d_n7, assign1670_e1126_d_n10, assign1670_e1126_d_n11, assign1670_e1126_d_n12, assign1670_e1126_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign1670_e1126;
        locals.var_dnm_dn0 = assign1670_e1126_d_n0;
        locals.var_dnm_dn2 = assign1670_e1126_d_n2;
        locals.var_dnm_dn6 = assign1670_e1126_d_n6;
        locals.var_dnm_dn7 = assign1670_e1126_d_n7;
        locals.var_dnm_dn10 = assign1670_e1126_d_n10;
        locals.var_dnm_dn11 = assign1670_e1126_d_n11;
        locals.var_dnm_dn12 = assign1670_e1126_d_n12;
        locals.var_dnm_dn17 = assign1670_e1126_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign1680_e1132, assign1680_e1132_d_n0, assign1680_e1132_d_n2, assign1680_e1132_d_n6, assign1680_e1132_d_n7, assign1680_e1132_d_n10, assign1680_e1132_d_n11, assign1680_e1132_d_n12, assign1680_e1132_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1680_e1130: f64 = (locals.var_xp * locals.var_x2);
        (assign1680_e1130, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign1680_e1132;
        locals.var_xp_dn0 = assign1680_e1132_d_n0;
        locals.var_xp_dn2 = assign1680_e1132_d_n2;
        locals.var_xp_dn6 = assign1680_e1132_d_n6;
        locals.var_xp_dn7 = assign1680_e1132_d_n7;
        locals.var_xp_dn10 = assign1680_e1132_d_n10;
        locals.var_xp_dn11 = assign1680_e1132_d_n11;
        locals.var_xp_dn12 = assign1680_e1132_d_n12;
        locals.var_xp_dn17 = assign1680_e1132_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign1690_e1138, assign1690_e1138_d_n0, assign1690_e1138_d_n2, assign1690_e1138_d_n6, assign1690_e1138_d_n7, assign1690_e1138_d_n10, assign1690_e1138_d_n11, assign1690_e1138_d_n12, assign1690_e1138_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1690_e1136: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign1690_e1136, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign1690_e1138;
        locals.var_xmp_dn0 = assign1690_e1138_d_n0;
        locals.var_xmp_dn2 = assign1690_e1138_d_n2;
        locals.var_xmp_dn6 = assign1690_e1138_d_n6;
        locals.var_xmp_dn7 = assign1690_e1138_d_n7;
        locals.var_xmp_dn10 = assign1690_e1138_d_n10;
        locals.var_xmp_dn11 = assign1690_e1138_d_n11;
        locals.var_xmp_dn12 = assign1690_e1138_d_n12;
        locals.var_xmp_dn17 = assign1690_e1138_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign1700_e1144, assign1700_e1144_d_n0, assign1700_e1144_d_n2, assign1700_e1144_d_n6, assign1700_e1144_d_n7, assign1700_e1144_d_n10, assign1700_e1144_d_n11, assign1700_e1144_d_n12, assign1700_e1144_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1700_e1142: f64 = (locals.var_xp * locals.var_x2);
        (assign1700_e1142, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign1700_e1144;
        locals.var_xp_dn0 = assign1700_e1144_d_n0;
        locals.var_xp_dn2 = assign1700_e1144_d_n2;
        locals.var_xp_dn6 = assign1700_e1144_d_n6;
        locals.var_xp_dn7 = assign1700_e1144_d_n7;
        locals.var_xp_dn10 = assign1700_e1144_d_n10;
        locals.var_xp_dn11 = assign1700_e1144_d_n11;
        locals.var_xp_dn12 = assign1700_e1144_d_n12;
        locals.var_xp_dn17 = assign1700_e1144_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign1710_e1150, assign1710_e1150_d_n0, assign1710_e1150_d_n2, assign1710_e1150_d_n6, assign1710_e1150_d_n7, assign1710_e1150_d_n10, assign1710_e1150_d_n11, assign1710_e1150_d_n12, assign1710_e1150_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1710_e1148: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign1710_e1148, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign1710_e1150;
        locals.var_xmp_dn0 = assign1710_e1150_d_n0;
        locals.var_xmp_dn2 = assign1710_e1150_d_n2;
        locals.var_xmp_dn6 = assign1710_e1150_d_n6;
        locals.var_xmp_dn7 = assign1710_e1150_d_n7;
        locals.var_xmp_dn10 = assign1710_e1150_d_n10;
        locals.var_xmp_dn11 = assign1710_e1150_d_n11;
        locals.var_xmp_dn12 = assign1710_e1150_d_n12;
        locals.var_xmp_dn17 = assign1710_e1150_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign1720_e1156, assign1720_e1156_d_n0, assign1720_e1156_d_n2, assign1720_e1156_d_n6, assign1720_e1156_d_n7, assign1720_e1156_d_n10, assign1720_e1156_d_n11, assign1720_e1156_d_n12, assign1720_e1156_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1720_e1154: f64 = (locals.var_xp + locals.var_xmp);
        (assign1720_e1154, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign1720_e1156;
        locals.var_arg_dn0 = assign1720_e1156_d_n0;
        locals.var_arg_dn2 = assign1720_e1156_d_n2;
        locals.var_arg_dn6 = assign1720_e1156_d_n6;
        locals.var_arg_dn7 = assign1720_e1156_d_n7;
        locals.var_arg_dn10 = assign1720_e1156_d_n10;
        locals.var_arg_dn11 = assign1720_e1156_d_n11;
        locals.var_arg_dn12 = assign1720_e1156_d_n12;
        locals.var_arg_dn17 = assign1720_e1156_d_n17;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1730_e1160, assign1730_e1160_d_n0, assign1730_e1160_d_n2, assign1730_e1160_d_n6, assign1730_e1160_d_n7, assign1730_e1160_d_n10, assign1730_e1160_d_n11, assign1730_e1160_d_n12, assign1730_e1160_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign1730_e1160;
        locals.var_dnm_dn0 = assign1730_e1160_d_n0;
        locals.var_dnm_dn2 = assign1730_e1160_d_n2;
        locals.var_dnm_dn6 = assign1730_e1160_d_n6;
        locals.var_dnm_dn7 = assign1730_e1160_d_n7;
        locals.var_dnm_dn10 = assign1730_e1160_d_n10;
        locals.var_dnm_dn11 = assign1730_e1160_d_n11;
        locals.var_dnm_dn12 = assign1730_e1160_d_n12;
        locals.var_dnm_dn17 = assign1730_e1160_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign1740_e1175: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard3 = assign1740_e1175;
        locals.var_guard3_rv = 0.0;

        let assign1750_e1178: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard4 = assign1750_e1178;
        locals.var_guard4_rv = 0.0;

        let (assign1760_e1186,) = {
    if (((locals.var_guard2 != 0.0) && (locals.var_guard3 != 0.0)) && (locals.var_guard4 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign1760_e1186;
        locals.var_mm_rv = 0.0;

        let assign1770_e1189: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard5 = assign1770_e1189;
        locals.var_guard5_rv = 0.0;

        let (assign1780_e1200,) = {
    if ((((locals.var_guard2 != 0.0) && (locals.var_guard3 != 0.0)) && (locals.var_guard4 == 0.0)) && (locals.var_guard5 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign1780_e1200;
        locals.var_mm_rv = 0.0;

        let assign1790_e1203: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign1790_e1203;
        locals.var_guard6_rv = 0.0;

        let (assign1800_e1217,) = {
    if (((((locals.var_guard2 != 0.0) && (locals.var_guard3 != 0.0)) && (locals.var_guard4 == 0.0)) && (locals.var_guard5 == 0.0)) && (locals.var_guard6 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign1800_e1217;
        locals.var_mm_rv = 0.0;

        let assign1810_e1220: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard7 = assign1810_e1220;
        locals.var_guard7_rv = 0.0;

        let (assign1820_e1237,) = {
    if ((((((locals.var_guard2 != 0.0) && (locals.var_guard3 != 0.0)) && (locals.var_guard4 == 0.0)) && (locals.var_guard5 == 0.0)) && (locals.var_guard6 == 0.0)) && (locals.var_guard7 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign1820_e1237;
        locals.var_mm_rv = 0.0;

        let (assign1830_e1243,) = {
    if ((locals.var_guard2 != 0.0) && (locals.var_guard3 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign1830_e1243;
        locals.var_m0_rv = 0.0;

        let mut assign1840_loop_guard: usize = 0;
        while {
            let assign1840_cond_e1250: f64 = if (((locals.var_guard2 != 0.0) && (locals.var_guard3 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign1840_cond_e1250 != 0.0
        } {
            assign1840_loop_guard += 1;
            assert!(assign1840_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign1840_body0_e1257, assign1840_body0_e1257_d_n0, assign1840_body0_e1257_d_n2, assign1840_body0_e1257_d_n6, assign1840_body0_e1257_d_n7, assign1840_body0_e1257_d_n10, assign1840_body0_e1257_d_n11, assign1840_body0_e1257_d_n12, assign1840_body0_e1257_d_n17,) = {
    if ((locals.var_guard2 != 0.0) && (locals.var_guard3 != 0.0)) {
        let assign1840_body0_e1255: f64 = (locals.var_dnm).sqrt();
        (assign1840_body0_e1255, (locals.var_dnm_dn0 / (2.0 * assign1840_body0_e1255)), (locals.var_dnm_dn2 / (2.0 * assign1840_body0_e1255)), (locals.var_dnm_dn6 / (2.0 * assign1840_body0_e1255)), (locals.var_dnm_dn7 / (2.0 * assign1840_body0_e1255)), (locals.var_dnm_dn10 / (2.0 * assign1840_body0_e1255)), (locals.var_dnm_dn11 / (2.0 * assign1840_body0_e1255)), (locals.var_dnm_dn12 / (2.0 * assign1840_body0_e1255)), (locals.var_dnm_dn17 / (2.0 * assign1840_body0_e1255)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign1840_body0_e1257;
            locals.var_dnm_dn0 = assign1840_body0_e1257_d_n0;
            locals.var_dnm_dn2 = assign1840_body0_e1257_d_n2;
            locals.var_dnm_dn6 = assign1840_body0_e1257_d_n6;
            locals.var_dnm_dn7 = assign1840_body0_e1257_d_n7;
            locals.var_dnm_dn10 = assign1840_body0_e1257_d_n10;
            locals.var_dnm_dn11 = assign1840_body0_e1257_d_n11;
            locals.var_dnm_dn12 = assign1840_body0_e1257_d_n12;
            locals.var_dnm_dn17 = assign1840_body0_e1257_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign1840_body1_e1265,) = {
    if ((locals.var_guard2 != 0.0) && (locals.var_guard3 != 0.0)) {
        let assign1840_body1_e1263: f64 = (locals.var_m0 + 1.0);
        (assign1840_body1_e1263,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign1840_body1_e1265;
            locals.var_m0_rv = 0.0;
        }

        let (assign1850_e1278, assign1850_e1278_d_n0, assign1850_e1278_d_n2, assign1850_e1278_d_n6, assign1850_e1278_d_n7, assign1850_e1278_d_n10, assign1850_e1278_d_n11, assign1850_e1278_d_n12, assign1850_e1278_d_n17,) = {
    if ((locals.var_guard2 != 0.0) && (locals.var_guard3 == 0.0)) {
        let assign1850_e1274: f64 = (2.0 * 2.0);
        let assign1850_e1275: f64 = (1.0 / assign1850_e1274);
        let assign1850_e1276: f64 = (locals.var_dnm).powf(assign1850_e1275);
        (assign1850_e1276, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((locals.var_dnm).powf(assign1850_e1275 - 1.0) * locals.var_dnm_dn0)) } } else { (assign1850_e1276 * (assign1850_e1275 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((locals.var_dnm).powf(assign1850_e1275 - 1.0) * locals.var_dnm_dn2)) } } else { (assign1850_e1276 * (assign1850_e1275 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((locals.var_dnm).powf(assign1850_e1275 - 1.0) * locals.var_dnm_dn6)) } } else { (assign1850_e1276 * (assign1850_e1275 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((locals.var_dnm).powf(assign1850_e1275 - 1.0) * locals.var_dnm_dn7)) } } else { (assign1850_e1276 * (assign1850_e1275 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((locals.var_dnm).powf(assign1850_e1275 - 1.0) * locals.var_dnm_dn10)) } } else { (assign1850_e1276 * (assign1850_e1275 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((locals.var_dnm).powf(assign1850_e1275 - 1.0) * locals.var_dnm_dn11)) } } else { (assign1850_e1276 * (assign1850_e1275 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((locals.var_dnm).powf(assign1850_e1275 - 1.0) * locals.var_dnm_dn12)) } } else { (assign1850_e1276 * (assign1850_e1275 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign1850_e1275) as f64).is_finite() && ((assign1850_e1275) as f64).fract() == 0.0 { if assign1850_e1275 == 0.0 { 0.0 } else { (assign1850_e1275 * ((locals.var_dnm).powf(assign1850_e1275 - 1.0) * locals.var_dnm_dn17)) } } else { (assign1850_e1276 * (assign1850_e1275 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign1850_e1278;
        locals.var_dnm_dn0 = assign1850_e1278_d_n0;
        locals.var_dnm_dn2 = assign1850_e1278_d_n2;
        locals.var_dnm_dn6 = assign1850_e1278_d_n6;
        locals.var_dnm_dn7 = assign1850_e1278_d_n7;
        locals.var_dnm_dn10 = assign1850_e1278_d_n10;
        locals.var_dnm_dn11 = assign1850_e1278_d_n11;
        locals.var_dnm_dn12 = assign1850_e1278_d_n12;
        locals.var_dnm_dn17 = assign1850_e1278_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign1860_e1284, assign1860_e1284_d_n0, assign1860_e1284_d_n2, assign1860_e1284_d_n6, assign1860_e1284_d_n7, assign1860_e1284_d_n10, assign1860_e1284_d_n11, assign1860_e1284_d_n12, assign1860_e1284_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1860_e1282: f64 = (1.0 / locals.var_dnm);
        (assign1860_e1282, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign1860_e1284;
        locals.var_dnm_dn0 = assign1860_e1284_d_n0;
        locals.var_dnm_dn2 = assign1860_e1284_d_n2;
        locals.var_dnm_dn6 = assign1860_e1284_d_n6;
        locals.var_dnm_dn7 = assign1860_e1284_d_n7;
        locals.var_dnm_dn10 = assign1860_e1284_d_n10;
        locals.var_dnm_dn11 = assign1860_e1284_d_n11;
        locals.var_dnm_dn12 = assign1860_e1284_d_n12;
        locals.var_dnm_dn17 = assign1860_e1284_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign1870_e1292, assign1870_e1292_d_n0, assign1870_e1292_d_n2, assign1870_e1292_d_n6, assign1870_e1292_d_n7, assign1870_e1292_d_n10, assign1870_e1292_d_n11, assign1870_e1292_d_n12, assign1870_e1292_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1870_e1288: f64 = (locals.var_tmf1 * 0.1);
        let assign1870_e1290: f64 = (assign1870_e1288 * locals.var_dnm);
        (assign1870_e1290, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign1870_e1288 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign1870_e1288 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign1870_e1288 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign1870_e1288 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign1870_e1288 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign1870_e1288 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn12 * 0.1) * locals.var_dnm) + (assign1870_e1288 * locals.var_dnm_dn12)), (((locals.var_tmf1_dn17 * 0.1) * locals.var_dnm) + (assign1870_e1288 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn12, locals.var_tmf0_dn17,)
    }
};
        locals.var_tmf0 = assign1870_e1292;
        locals.var_tmf0_dn0 = assign1870_e1292_d_n0;
        locals.var_tmf0_dn2 = assign1870_e1292_d_n2;
        locals.var_tmf0_dn6 = assign1870_e1292_d_n6;
        locals.var_tmf0_dn7 = assign1870_e1292_d_n7;
        locals.var_tmf0_dn10 = assign1870_e1292_d_n10;
        locals.var_tmf0_dn11 = assign1870_e1292_d_n11;
        locals.var_tmf0_dn12 = assign1870_e1292_d_n12;
        locals.var_tmf0_dn17 = assign1870_e1292_d_n17;
        locals.var_tmf0_rv = 0.0;

        let (assign1880_e1300, assign1880_e1300_d_n0, assign1880_e1300_d_n2, assign1880_e1300_d_n6, assign1880_e1300_d_n7, assign1880_e1300_d_n10, assign1880_e1300_d_n11, assign1880_e1300_d_n12, assign1880_e1300_d_n17,) = {
    if (locals.var_guard2 != 0.0) {
        let assign1880_e1296: f64 = (2.0 + 0.1);
        let assign1880_e1298: f64 = (assign1880_e1296 - locals.var_tmf0);
        (assign1880_e1298, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn12), (-locals.var_tmf0_dn17),)
    } else {
        (locals.var_uc_clm2, locals.var_uc_clm2_dn0, locals.var_uc_clm2_dn2, locals.var_uc_clm2_dn6, locals.var_uc_clm2_dn7, locals.var_uc_clm2_dn10, locals.var_uc_clm2_dn11, locals.var_uc_clm2_dn12, locals.var_uc_clm2_dn17,)
    }
};
        locals.var_uc_clm2 = assign1880_e1300;
        locals.var_uc_clm2_dn0 = assign1880_e1300_d_n0;
        locals.var_uc_clm2_dn2 = assign1880_e1300_d_n2;
        locals.var_uc_clm2_dn6 = assign1880_e1300_d_n6;
        locals.var_uc_clm2_dn7 = assign1880_e1300_d_n7;
        locals.var_uc_clm2_dn10 = assign1880_e1300_d_n10;
        locals.var_uc_clm2_dn11 = assign1880_e1300_d_n11;
        locals.var_uc_clm2_dn12 = assign1880_e1300_d_n12;
        locals.var_uc_clm2_dn17 = assign1880_e1300_d_n17;
        locals.var_uc_clm2_rv = 0.0;

        let (assign1890_e1305, assign1890_e1305_d_n0, assign1890_e1305_d_n2, assign1890_e1305_d_n6, assign1890_e1305_d_n7, assign1890_e1305_d_n10, assign1890_e1305_d_n11, assign1890_e1305_d_n12, assign1890_e1305_d_n17,) = {
    if (locals.var_guard2 == 0.0) {
        (locals.var_uc_clm2, locals.var_uc_clm2_dn0, locals.var_uc_clm2_dn2, locals.var_uc_clm2_dn6, locals.var_uc_clm2_dn7, locals.var_uc_clm2_dn10, locals.var_uc_clm2_dn11, locals.var_uc_clm2_dn12, locals.var_uc_clm2_dn17,)
    } else {
        (locals.var_uc_clm2, locals.var_uc_clm2_dn0, locals.var_uc_clm2_dn2, locals.var_uc_clm2_dn6, locals.var_uc_clm2_dn7, locals.var_uc_clm2_dn10, locals.var_uc_clm2_dn11, locals.var_uc_clm2_dn12, locals.var_uc_clm2_dn17,)
    }
};
        locals.var_uc_clm2 = assign1890_e1305;
        locals.var_uc_clm2_dn0 = assign1890_e1305_d_n0;
        locals.var_uc_clm2_dn2 = assign1890_e1305_d_n2;
        locals.var_uc_clm2_dn6 = assign1890_e1305_d_n6;
        locals.var_uc_clm2_dn7 = assign1890_e1305_d_n7;
        locals.var_uc_clm2_dn10 = assign1890_e1305_d_n10;
        locals.var_uc_clm2_dn11 = assign1890_e1305_d_n11;
        locals.var_uc_clm2_dn12 = assign1890_e1305_d_n12;
        locals.var_uc_clm2_dn17 = assign1890_e1305_d_n17;
        locals.var_uc_clm2_rv = 0.0;

        let assign1900_e1311: f64 = (locals.var_uc_tnom * 1e-7);
        let assign1900_e1312: f64 = (9.025e-5 + assign1900_e1311);
        let assign1900_e1313: f64 = (locals.var_uc_tnom * assign1900_e1312);
        let assign1900_e1314: f64 = (p.p55 - assign1900_e1313);
        locals.var_egtnom = assign1900_e1314;
        locals.var_egtnom_rv = 0.0;

        locals.var_tfox0 = p.p236;
        locals.var_tfox0_rv = 0.0;

        let assign1920_e1318: f64 = (1.034943e-10 / p.p237);
        locals.var_c_soi = assign1920_e1318;
        locals.var_c_soi_rv = 0.0;

        let assign1930_e1321: f64 = (1.0 / locals.var_c_soi);
        locals.var_c_soi_inv = assign1930_e1321;
        locals.var_c_soi_inv_rv = 0.0;

        let assign1940_e1324: f64 = (3.453133e-11 / locals.var_tfox0);
        locals.var_c_fox0 = assign1940_e1324;
        locals.var_c_fox0_rv = 0.0;

        let assign1950_e1327: f64 = (locals.var_tfox0 / 3.453133e-11);
        locals.var_c_fox0_inv = assign1950_e1327;
        locals.var_c_fox0_inv_rv = 0.0;

        let assign1960_e1330: f64 = (3.453133e-11 / p.p239);
        locals.var_c_box = assign1960_e1330;
        locals.var_c_box_rv = 0.0;

        let assign1970_e1333: f64 = (p.p239 / 3.453133e-11);
        locals.var_c_box_inv = assign1970_e1333;
        locals.var_c_box_inv_rv = 0.0;

        let assign1980_e1336: f64 = (locals.var_c_box_inv + locals.var_c_soi_inv);
        locals.var_c_box_fd_inv = assign1980_e1336;
        locals.var_c_box_fd_inv_rv = 0.0;

        locals.var_lgate = p.p0;
        locals.var_lgate_rv = 0.0;

        let assign2000_e1341: f64 = (2.0 * p.p56);
        let assign2000_e1342: f64 = (locals.var_lgate - assign2000_e1341);
        locals.var_leff = assign2000_e1342;
        locals.var_leff_rv = 0.0;

        let assign2010_e1346: f64 = (2.0 * p.p57);
        let assign2010_e1347: f64 = (locals.var_lgate - assign2010_e1346);
        locals.var_leff_cv = assign2010_e1347;
        locals.var_leff_cv_rv = 0.0;

        let (assign2020_e1353,) = {
    if (p.p40 == 0.0) {
        (locals.var_lgate,)
    } else {
        (locals.var_leff,)
    }
};
        locals.var_lgleff = assign2020_e1353;
        locals.var_lgleff_rv = 0.0;

        let assign2030_e1356: f64 = (locals.var_lgleff * 1000000.0);
        locals.var_lgle = assign2030_e1356;
        locals.var_lgle_rv = 0.0;

        let assign2040_e1359: f64 = (p.p1 / p.p9);
        locals.var_wgate = assign2040_e1359;
        locals.var_wgate_rv = 0.0;

        locals.var_dw = p.p60;
        locals.var_dw_rv = 0.0;

        let (assign2060_e1366,) = {
    if (locals.var_subversion < 1.0) {
        (0.0,)
    } else {
        (p.p295,)
    }
};
        locals.var_dwbt = assign2060_e1366;
        locals.var_dwbt_rv = 0.0;

        let (assign2070_e1372,) = {
    if (locals.var_subversion < 1.0) {
        (p.p60,)
    } else {
        (p.p61,)
    }
};
        locals.var_dwcv = assign2070_e1372;
        locals.var_dwcv_rv = 0.0;

        let assign2080_e1375: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard8 = assign2080_e1375;
        locals.var_guard8_rv = 0.0;

        let (assign2090_e1383,) = {
    if (locals.var_guard8 != 0.0) {
        let assign2090_e1380: f64 = (2.0 * locals.var_dw);
        let assign2090_e1381: f64 = (locals.var_wgate - assign2090_e1380);
        (assign2090_e1381,)
    } else {
        (locals.var_weff,)
    }
};
        locals.var_weff = assign2090_e1383;
        locals.var_weff_rv = 0.0;

        let (assign2100_e1391,) = {
    if (locals.var_guard8 != 0.0) {
        let assign2100_e1388: f64 = (2.0 * locals.var_dwcv);
        let assign2100_e1389: f64 = (locals.var_wgate - assign2100_e1388);
        (assign2100_e1389,)
    } else {
        (locals.var_weff_cv,)
    }
};
        locals.var_weff_cv = assign2100_e1391;
        locals.var_weff_cv_rv = 0.0;

        let (assign2110_e1406,) = {
    if (locals.var_guard8 == 0.0) {
        let assign2110_e1397: f64 = (p.p18 * locals.var_dwbt);
        let assign2110_e1398: f64 = (locals.var_wgate - assign2110_e1397);
        let assign2110_e1401: f64 = (2.0 - p.p18);
        let assign2110_e1403: f64 = (assign2110_e1401 * locals.var_dw);
        let assign2110_e1404: f64 = (assign2110_e1398 - assign2110_e1403);
        (assign2110_e1404,)
    } else {
        (locals.var_weff,)
    }
};
        locals.var_weff = assign2110_e1406;
        locals.var_weff_rv = 0.0;

        let (assign2120_e1421,) = {
    if (locals.var_guard8 == 0.0) {
        let assign2120_e1412: f64 = (p.p18 * locals.var_dwbt);
        let assign2120_e1413: f64 = (locals.var_wgate - assign2120_e1412);
        let assign2120_e1416: f64 = (2.0 - p.p18);
        let assign2120_e1418: f64 = (assign2120_e1416 * locals.var_dwcv);
        let assign2120_e1419: f64 = (assign2120_e1413 - assign2120_e1418);
        (assign2120_e1419,)
    } else {
        (locals.var_weff_cv,)
    }
};
        locals.var_weff_cv = assign2120_e1421;
        locals.var_weff_cv_rv = 0.0;

        let assign2130_e1424: f64 = (locals.var_weff * p.p9);
        locals.var_weff_nf = assign2130_e1424;
        locals.var_weff_nf_rv = 0.0;

        let assign2140_e1427: f64 = (locals.var_weff_cv * p.p9);
        locals.var_weffcv_nf = assign2140_e1427;
        locals.var_weffcv_nf_rv = 0.0;

        let assign2150_e1430: f64 = (locals.var_wgate * 1000000.0);
        locals.var_wg = assign2150_e1430;
        locals.var_wg_rv = 0.0;

        let assign2160_e1433: f64 = (locals.var_wg * locals.var_lgle);
        locals.var_wl = assign2160_e1433;
        locals.var_wl_rv = 0.0;

        let assign2170_e1439: f64 = (locals.var_lgle).powf(p.p111);
        let assign2170_e1440: f64 = (p.p108 / assign2170_e1439);
        let assign2170_e1441: f64 = (1.0 + assign2170_e1440);
        let assign2170_e1442: f64 = (p.p107 * assign2170_e1441);
        let assign2170_e1447: f64 = (locals.var_wg).powf(p.p110);
        let assign2170_e1448: f64 = (p.p109 / assign2170_e1447);
        let assign2170_e1449: f64 = (1.0 + assign2170_e1448);
        let assign2170_e1450: f64 = (assign2170_e1442 * assign2170_e1449);
        locals.var_muesr = assign2170_e1450;
        locals.var_muesr_rv = 0.0;

        let assign2180_e1461: f64 = if (((locals.var_subversion > 3.0) && (locals.var_mks_nsubp < locals.var_mks_nsubs)) && (p.p72 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard9 = assign2180_e1461;
        locals.var_guard9_rv = 0.0;

        let (assign2190_e1465,) = {
    if (locals.var_guard9 != 0.0) {
        (locals.var_mks_nsubs,)
    } else {
        (locals.var_mks_nsubp,)
    }
};
        locals.var_mks_nsubp = assign2190_e1465;
        locals.var_mks_nsubp_rv = 0.0;

        let assign2200_e1471: f64 = (locals.var_wg).powf(p.p75);
        let assign2200_e1472: f64 = (p.p74 / assign2200_e1471);
        let assign2200_e1473: f64 = (1.0 + assign2200_e1472);
        let assign2200_e1474: f64 = (locals.var_mks_nsubp * assign2200_e1473);
        locals.var_nsubpp = assign2200_e1474;
        locals.var_nsubpp_rv = 0.0;

        let assign2210_e1480: f64 = (0.5 * locals.var_lgate);
        let assign2210_e1481: f64 = (p.p62 + assign2210_e1480);
        let assign2210_e1482: f64 = (1.0 / assign2210_e1481);
        let assign2210_e1487: f64 = (0.5 * locals.var_lgate);
        let assign2210_e1488: f64 = (p.p63 + assign2210_e1487);
        let assign2210_e1489: f64 = (1.0 / assign2210_e1488);
        let assign2210_e1490: f64 = (assign2210_e1482 + assign2210_e1489);
        let assign2210_e1491: f64 = (2.0 / assign2210_e1490);
        locals.var_lod_half_ref = assign2210_e1491;
        locals.var_lod_half_ref_rv = 0.0;

        let assign2220_e1495: f64 = (1.3806226e-23 * locals.var_uc_tnom);
        let assign2220_e1496: f64 = (1.6021918e-19 / assign2220_e1495);
        locals.var_betatnom = assign2220_e1496;
        locals.var_betatnom_rv = 0.0;

        let assign2230_e1499: f64 = (1.6021918e-19 * locals.var_mks_nsubb);
        let assign2230_e1501: f64 = (assign2230_e1499 * 1.034943e-10);
        locals.var_qnbulk_esi = assign2230_e1501;
        locals.var_qnbulk_esi_rv = 0.0;

        let assign2240_e1505: f64 = (-p.p247);
        let assign2240_e1506: f64 = (locals.var_lgle).powf(assign2240_e1505);
        let assign2240_e1507: f64 = (p.p244 * assign2240_e1506);
        locals.var_ptl0 = assign2240_e1507;
        locals.var_ptl0_rv = 0.0;

        let assign2250_e1511: f64 = (-p.p252);
        let assign2250_e1512: f64 = (locals.var_lgle).powf(assign2250_e1511);
        let assign2250_e1513: f64 = (p.p251 * assign2250_e1512);
        locals.var_pt40 = assign2250_e1513;
        locals.var_pt40_rv = 0.0;

        let assign2260_e1517: f64 = (locals.var_lgle + locals.var_uc_gdld);
        let assign2260_e1519: f64 = (-p.p249);
        let assign2260_e1520: f64 = (assign2260_e1517).powf(assign2260_e1519);
        let assign2260_e1521: f64 = (p.p248 * assign2260_e1520);
        locals.var_gdl0 = assign2260_e1521;
        locals.var_gdl0_rv = 0.0;

        let assign2270_e1524: f64 = (2.0 * 1.6021918e-19);
        let assign2270_e1526: f64 = (assign2270_e1524 * locals.var_mks_nsti);
        let assign2270_e1528: f64 = (assign2270_e1526 * 1.034943e-10);
        let assign2270_e1529: f64 = (assign2270_e1528).sqrt();
        locals.var_costi00 = assign2270_e1529;
        locals.var_costi00_rv = 0.0;

        let assign2280_e1533: f64 = (locals.var_mks_nsti * locals.var_mks_nsti);
        let assign2280_e1534: f64 = (1.0 / assign2280_e1533);
        locals.var_nsti_p2 = assign2280_e1534;
        locals.var_nsti_p2_rv = 0.0;

        let assign2290_e1538: f64 = (1.0 / locals.var_lgle);
        let assign2290_e1539: f64 = (1.0 + assign2290_e1538);
        let assign2290_e1541: f64 = (assign2290_e1539).powf(p.p91);
        let assign2290_e1543: f64 = (assign2290_e1541 * p.p89);
        locals.var_cnstpgd = assign2290_e1543;
        locals.var_cnstpgd_rv = 0.0;

        locals.var_c0bulk = locals.var_qnbulk_esi;
        locals.var_c0bulk_rv = 0.0;

        locals.var_vfb = p.p68;
        locals.var_vfb_rv = 0.0;

        let assign2320_e1550: f64 = (locals.var_wl).powf(p.p77);
        let assign2320_e1551: f64 = (p.p76 / assign2320_e1550);
        let assign2320_e1552: f64 = (locals.var_lgleff + assign2320_e1551);
        locals.var_lgatesm = assign2320_e1552;
        locals.var_lgatesm_rv = 0.0;

        let assign2330_e1556: f64 = (locals.var_wl).powf(p.p79);
        let assign2330_e1557: f64 = (p.p78 / assign2330_e1556);
        locals.var_dvthsm = assign2330_e1557;
        locals.var_dvthsm_rv = 0.0;

        let assign2340_e1563: f64 = (locals.var_lgatesm * 1000000.0);
        let assign2340_e1565: f64 = (assign2340_e1563).powf(p.p151);
        let assign2340_e1566: f64 = (p.p150 / assign2340_e1565);
        let assign2340_e1567: f64 = (1.0 + assign2340_e1566);
        let assign2340_e1568: f64 = (p.p149 * assign2340_e1567);
        let assign2340_e1570: f64 = assign2340_e1568;
        let assign2340_e1574: f64 = (locals.var_wg).powf(p.p153);
        let assign2340_e1575: f64 = (p.p152 / assign2340_e1574);
        let assign2340_e1576: f64 = (assign2340_e1570 + assign2340_e1575);
        locals.var_uc_wsti = assign2340_e1576;
        locals.var_uc_wsti_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign2350_e1580: f64 = (locals.var_lgle).powf(p.p192);
        let assign2350_e1582: f64 = (assign2350_e1580 * p.p193);
        let assign2350_e1583: f64 = (1.0 + assign2350_e1582);
        locals.var_clmmod = assign2350_e1583;
        locals.var_clmmod_rv = 0.0;

        let assign2370_e1603: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard10 = assign2370_e1603;
        locals.var_guard10_rv = 0.0;

        let (assign2380_e1613,) = {
    if (locals.var_guard10 != 0.0) {
        let assign2380_e1609: f64 = (locals.var_wg).powf(p.p131);
        let assign2380_e1610: f64 = (p.p130 / assign2380_e1609);
        let assign2380_e1611: f64 = (1.0 + assign2380_e1610);
        (assign2380_e1611,)
    } else {
        (locals.var_zvgs,)
    }
};
        locals.var_zvgs = assign2380_e1613;
        locals.var_zvgs_rv = 0.0;

        let (assign2390_e1625,) = {
    if (locals.var_guard10 != 0.0) {
        let assign2390_e1620: f64 = (locals.var_lgle).powf(p.p126);
        let assign2390_e1621: f64 = (p.p125 / assign2390_e1620);
        let assign2390_e1622: f64 = (1.0 + assign2390_e1621);
        let assign2390_e1623: f64 = (p.p124 * assign2390_e1622);
        (assign2390_e1623,)
    } else {
        (locals.var_xvbs,)
    }
};
        locals.var_xvbs = assign2390_e1625;
        locals.var_xvbs_rv = 0.0;

        let (assign2400_e1633,) = {
    if (locals.var_guard10 != 0.0) {
        let assign2400_e1630: f64 = (locals.var_lgle + p.p123);
        let assign2400_e1631: f64 = (locals.var_lgle / assign2400_e1630);
        (assign2400_e1631,)
    } else {
        (locals.var_xgate,)
    }
};
        locals.var_xgate = assign2400_e1633;
        locals.var_xgate_rv = 0.0;

        let (assign2410_e1645,) = {
    if (locals.var_guard10 != 0.0) {
        let assign2410_e1640: f64 = (locals.var_lgle).powf(p.p120);
        let assign2410_e1641: f64 = (p.p119 / assign2410_e1640);
        let assign2410_e1642: f64 = (1.0 + assign2410_e1641);
        let assign2410_e1643: f64 = (p.p117 * assign2410_e1642);
        (assign2410_e1643,)
    } else {
        (locals.var_xsub1,)
    }
};
        locals.var_xsub1 = assign2410_e1645;
        locals.var_xsub1_rv = 0.0;

        let (assign2420_e1655,) = {
    if (locals.var_guard10 != 0.0) {
        let assign2420_e1651: f64 = (p.p121 / locals.var_lgle);
        let assign2420_e1652: f64 = (1.0 + assign2420_e1651);
        let assign2420_e1653: f64 = (p.p118 * assign2420_e1652);
        (assign2420_e1653,)
    } else {
        (locals.var_xsub2,)
    }
};
        locals.var_xsub2 = assign2420_e1655;
        locals.var_xsub2_rv = 0.0;

        let (assign2430_e1662, assign2430_e1662_d_n0, assign2430_e1662_d_n2, assign2430_e1662_d_n6, assign2430_e1662_d_n7, assign2430_e1662_d_n10, assign2430_e1662_d_n11, assign2430_e1662_d_n12, assign2430_e1662_d_n17,) = {
    if (locals.var_guard10 == 0.0) {
        let assign2430_e1660: f64 = (locals.var_wg).powf(p.p131);
        (assign2430_e1660, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign2430_e1662;
        locals.var_t2_dn0 = assign2430_e1662_d_n0;
        locals.var_t2_dn2 = assign2430_e1662_d_n2;
        locals.var_t2_dn6 = assign2430_e1662_d_n6;
        locals.var_t2_dn7 = assign2430_e1662_d_n7;
        locals.var_t2_dn10 = assign2430_e1662_d_n10;
        locals.var_t2_dn11 = assign2430_e1662_d_n11;
        locals.var_t2_dn12 = assign2430_e1662_d_n12;
        locals.var_t2_dn17 = assign2430_e1662_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign2440_e1681, assign2440_e1681_d_n0, assign2440_e1681_d_n2, assign2440_e1681_d_n6, assign2440_e1681_d_n7, assign2440_e1681_d_n10, assign2440_e1681_d_n11, assign2440_e1681_d_n12, assign2440_e1681_d_n17,) = {
    if (locals.var_guard10 == 0.0) {
        let assign2440_e1670: f64 = (locals.var_lgle).powf(p.p129);
        let assign2440_e1671: f64 = (p.p128 / assign2440_e1670);
        let assign2440_e1672: f64 = (1.0 + assign2440_e1671);
        let assign2440_e1673: f64 = (p.p127 * assign2440_e1672);
        let assign2440_e1677: f64 = (locals.var_t2 + p.p130);
        let assign2440_e1678: f64 = (locals.var_t2 / assign2440_e1677);
        let assign2440_e1679: f64 = (assign2440_e1673 * assign2440_e1678);
        (assign2440_e1679, (assign2440_e1673 * (((locals.var_t2_dn0 * assign2440_e1677) - (locals.var_t2 * locals.var_t2_dn0)) / (assign2440_e1677 * assign2440_e1677))), (assign2440_e1673 * (((locals.var_t2_dn2 * assign2440_e1677) - (locals.var_t2 * locals.var_t2_dn2)) / (assign2440_e1677 * assign2440_e1677))), (assign2440_e1673 * (((locals.var_t2_dn6 * assign2440_e1677) - (locals.var_t2 * locals.var_t2_dn6)) / (assign2440_e1677 * assign2440_e1677))), (assign2440_e1673 * (((locals.var_t2_dn7 * assign2440_e1677) - (locals.var_t2 * locals.var_t2_dn7)) / (assign2440_e1677 * assign2440_e1677))), (assign2440_e1673 * (((locals.var_t2_dn10 * assign2440_e1677) - (locals.var_t2 * locals.var_t2_dn10)) / (assign2440_e1677 * assign2440_e1677))), (assign2440_e1673 * (((locals.var_t2_dn11 * assign2440_e1677) - (locals.var_t2 * locals.var_t2_dn11)) / (assign2440_e1677 * assign2440_e1677))), (assign2440_e1673 * (((locals.var_t2_dn12 * assign2440_e1677) - (locals.var_t2 * locals.var_t2_dn12)) / (assign2440_e1677 * assign2440_e1677))), (assign2440_e1673 * (((locals.var_t2_dn17 * assign2440_e1677) - (locals.var_t2 * locals.var_t2_dn17)) / (assign2440_e1677 * assign2440_e1677))),)
    } else {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn10, locals.var_vg2const_dn11, locals.var_vg2const_dn12, locals.var_vg2const_dn17,)
    }
};
        locals.var_vg2const = assign2440_e1681;
        locals.var_vg2const_dn0 = assign2440_e1681_d_n0;
        locals.var_vg2const_dn2 = assign2440_e1681_d_n2;
        locals.var_vg2const_dn6 = assign2440_e1681_d_n6;
        locals.var_vg2const_dn7 = assign2440_e1681_d_n7;
        locals.var_vg2const_dn10 = assign2440_e1681_d_n10;
        locals.var_vg2const_dn11 = assign2440_e1681_d_n11;
        locals.var_vg2const_dn12 = assign2440_e1681_d_n12;
        locals.var_vg2const_dn17 = assign2440_e1681_d_n17;
        locals.var_vg2const_rv = 0.0;

        let (assign2450_e1694,) = {
    if (locals.var_guard10 == 0.0) {
        let assign2450_e1689: f64 = (locals.var_lgle).powf(p.p126);
        let assign2450_e1690: f64 = (p.p125 / assign2450_e1689);
        let assign2450_e1691: f64 = (1.0 + assign2450_e1690);
        let assign2450_e1692: f64 = (p.p124 * assign2450_e1691);
        (assign2450_e1692,)
    } else {
        (locals.var_xvbs,)
    }
};
        locals.var_xvbs = assign2450_e1694;
        locals.var_xvbs_rv = 0.0;

        let (assign2460_e1707,) = {
    if (locals.var_guard10 == 0.0) {
        let assign2460_e1702: f64 = (locals.var_lgle).powf(p.p133);
        let assign2460_e1703: f64 = (p.p132 / assign2460_e1702);
        let assign2460_e1704: f64 = (1.0 + assign2460_e1703);
        let assign2460_e1705: f64 = (p.p123 * assign2460_e1704);
        (assign2460_e1705,)
    } else {
        (locals.var_xgate,)
    }
};
        locals.var_xgate = assign2460_e1707;
        locals.var_xgate_rv = 0.0;

        let (assign2470_e1720,) = {
    if (locals.var_guard10 == 0.0) {
        let assign2470_e1715: f64 = (locals.var_lgle).powf(p.p120);
        let assign2470_e1716: f64 = (p.p119 / assign2470_e1715);
        let assign2470_e1717: f64 = (1.0 + assign2470_e1716);
        let assign2470_e1718: f64 = (p.p117 * assign2470_e1717);
        (assign2470_e1718,)
    } else {
        (locals.var_xsub1,)
    }
};
        locals.var_xsub1 = assign2470_e1720;
        locals.var_xsub1_rv = 0.0;

        let (assign2480_e1731,) = {
    if (locals.var_guard10 == 0.0) {
        let assign2480_e1727: f64 = (p.p121 / locals.var_lgle);
        let assign2480_e1728: f64 = (1.0 + assign2480_e1727);
        let assign2480_e1729: f64 = (p.p118 * assign2480_e1728);
        (assign2480_e1729,)
    } else {
        (locals.var_xsub2,)
    }
};
        locals.var_xsub2 = assign2480_e1731;
        locals.var_xsub2_rv = 0.0;

        let assign2490_e1734: f64 = (1000000.0 * locals.var_weffcv_nf);
        let assign2490_e1736: f64 = (assign2490_e1734 * p.p65);
        let assign2490_e1739: f64 = (locals.var_lgle).powf(p.p66);
        let assign2490_e1740: f64 = (assign2490_e1736 / assign2490_e1739);
        locals.var_cqyb0 = assign2490_e1740;
        locals.var_cqyb0_rv = 0.0;

        let assign2500_e1746: f64 = (locals.var_lgle).powf(p.p136);
        let assign2500_e1747: f64 = (p.p135 / assign2500_e1746);
        let assign2500_e1748: f64 = (1.0 + assign2500_e1747);
        let assign2500_e1749: f64 = (p.p134 * assign2500_e1748);
        locals.var_vfbsub0 = assign2500_e1749;
        locals.var_vfbsub0_rv = 0.0;

        let assign2510_e1752: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard11 = assign2510_e1752;
        locals.var_guard11_rv = 0.0;

        let (assign2520_e1764,) = {
    if (locals.var_guard11 != 0.0) {
        let assign2520_e1759: f64 = (locals.var_lgle).powf(p.p129);
        let assign2520_e1760: f64 = (p.p128 / assign2520_e1759);
        let assign2520_e1761: f64 = (1.0 + assign2520_e1760);
        let assign2520_e1762: f64 = (p.p127 * assign2520_e1761);
        (assign2520_e1762,)
    } else {
        (locals.var_uc_svgs,)
    }
};
        locals.var_uc_svgs = assign2520_e1764;
        locals.var_uc_svgs_rv = 0.0;

        let assign2530_e1767: f64 = (p.p115 * locals.var_lgle);
        let assign2530_e1769: f64 = (assign2530_e1767 * p.p114);
        let assign2530_e1772: f64 = (p.p115 * locals.var_lgle);
        let assign2530_e1774: f64 = (assign2530_e1772 + p.p114);
        let assign2530_e1775: f64 = (assign2530_e1769 / assign2530_e1774);
        let assign2530_e1777: f64 = (assign2530_e1775 + p.p116);
        let assign2530_e1779: f64 = (assign2530_e1777 + 1e-50);
        locals.var_ddlte = assign2530_e1779;
        locals.var_ddlte_rv = 0.0;

        let assign2540_e1782: f64 = if locals.var_ddlte < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard12 = assign2540_e1782;
        locals.var_guard12_rv = 0.0;

        let (assign2550_e1786,) = {
    if (locals.var_guard12 != 0.0) {
        (3.0,)
    } else {
        (locals.var_ddlte,)
    }
};
        locals.var_ddlte = assign2550_e1786;
        locals.var_ddlte_rv = 0.0;

        let assign2560_e1789: f64 = (p.p50 * p.p253);
        locals.var_vgs_min = assign2560_e1789;
        locals.var_vgs_min_rv = 0.0;

        let assign2570_e1791: f64 = if param_given[168] { 1.0 } else { 0.0 };
        locals.var_cgbo_given = assign2570_e1791;
        locals.var_cgbo_given_rv = 0.0;

        let assign2580_e1793: f64 = if param_given[169] { 1.0 } else { 0.0 };
        locals.var_cgdo_given = assign2580_e1793;
        locals.var_cgdo_given_rv = 0.0;

        let assign2590_e1795: f64 = if param_given[170] { 1.0 } else { 0.0 };
        locals.var_cgso_given = assign2590_e1795;
        locals.var_cgso_given_rv = 0.0;

        let assign2600_e1797: f64 = if param_given[294] { 1.0 } else { 0.0 };
        locals.var_cbtbp_given = assign2600_e1797;
        locals.var_cbtbp_given_rv = 0.0;

        let assign2610_e1799: f64 = if param_given[293] { 1.0 } else { 0.0 };
        locals.var_cbtbn_given = assign2610_e1799;
        locals.var_cbtbn_given_rv = 0.0;

        let assign2620_e1801: f64 = if param_given[13] { 1.0 } else { 0.0 };
        locals.var_pdbcp_given = assign2620_e1801;
        locals.var_pdbcp_given_rv = 0.0;

        let assign2630_e1803: f64 = if param_given[14] { 1.0 } else { 0.0 };
        locals.var_psbcp_given = assign2630_e1803;
        locals.var_psbcp_given_rv = 0.0;

        let assign2640_e1805: f64 = if param_given[23] { 1.0 } else { 0.0 };
        locals.var_abtp_given = assign2640_e1805;
        locals.var_abtp_given_rv = 0.0;

        let assign2650_e1807: f64 = if param_given[22] { 1.0 } else { 0.0 };
        locals.var_abtn_given = assign2650_e1807;
        locals.var_abtn_given_rv = 0.0;

        let assign2660_e1809: f64 = if param_given[16] { 1.0 } else { 0.0 };
        locals.var_temp_given = assign2660_e1809;
        locals.var_temp_given_rv = 0.0;

        let (assign2670_e1815,) = {
    if (p.p17 == 0.0) {
        (0.0,)
    } else {
        (1.0,)
    }
};
        locals.var_dtemp_given = assign2670_e1815;
        locals.var_dtemp_given_rv = 0.0;

        locals.var_mfactor = 1.0;
        locals.var_mfactor_rv = 0.0;

        let assign2690_e1819: f64 = 0.0;
        locals.var_gjmin = assign2690_e1819;
        locals.var_gjmin_rv = 0.0;

        locals.var_uc_pdbcp = p.p13;
        locals.var_uc_pdbcp_rv = 0.0;

        locals.var_uc_psbcp = p.p14;
        locals.var_uc_psbcp_rv = 0.0;

        let assign2720_e1824: f64 = (p.p16 + 273.15);
        locals.var_uc_temp = assign2720_e1824;
        locals.var_uc_temp_rv = 0.0;

        let assign2740_e1833: f64 = (locals.var_mfactor * locals.var_weffcv_nf);
        let assign2740_e1834: f64 = (locals.var_mks_cth0 * assign2740_e1833);
        locals.var_cth = assign2740_e1834;
        locals.var_cth_rv = 0.0;

        let assign2750_e1853: f64 = if (((p.p10 > 0.0) && (p.p11 > 0.0)) && ((p.p9 == 1.0) || ((p.p9 > 1.0) && (p.p12 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard13 = assign2750_e1853;
        locals.var_guard13_rv = 0.0;

        let (assign2760_e1857, assign2760_e1857_d_n0, assign2760_e1857_d_n2, assign2760_e1857_d_n6, assign2760_e1857_d_n7, assign2760_e1857_d_n10, assign2760_e1857_d_n11, assign2760_e1857_d_n12, assign2760_e1857_d_n17,) = {
    if (locals.var_guard13 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign2760_e1857;
        locals.var_t1_dn0 = assign2760_e1857_d_n0;
        locals.var_t1_dn2 = assign2760_e1857_d_n2;
        locals.var_t1_dn6 = assign2760_e1857_d_n6;
        locals.var_t1_dn7 = assign2760_e1857_d_n7;
        locals.var_t1_dn10 = assign2760_e1857_d_n10;
        locals.var_t1_dn11 = assign2760_e1857_d_n11;
        locals.var_t1_dn12 = assign2760_e1857_d_n12;
        locals.var_t1_dn17 = assign2760_e1857_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign2770_e1861,) = {
    if (locals.var_guard13 != 0.0) {
        (0.0,)
    } else {
        (locals.var_i,)
    }
};
        locals.var_i = assign2770_e1861;
        locals.var_i_rv = 0.0;

        let mut assign2780_loop_guard: usize = 0;
        while {
            let assign2780_cond_e1866: f64 = if ((locals.var_guard13 != 0.0) && (locals.var_i < p.p9)) { 1.0 } else { 0.0 };
            assign2780_cond_e1866 != 0.0
        } {
            assign2780_loop_guard += 1;
            assert!(assign2780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign2780_body0_e1898, assign2780_body0_e1898_d_n0, assign2780_body0_e1898_d_n2, assign2780_body0_e1898_d_n6, assign2780_body0_e1898_d_n7, assign2780_body0_e1898_d_n10, assign2780_body0_e1898_d_n11, assign2780_body0_e1898_d_n12, assign2780_body0_e1898_d_n17,) = {
    if (locals.var_guard13 != 0.0) {
        let assign2780_body0_e1873: f64 = (0.5 * locals.var_lgate);
        let assign2780_body0_e1874: f64 = (p.p10 + assign2780_body0_e1873);
        let assign2780_body0_e1878: f64 = (p.p12 + locals.var_lgate);
        let assign2780_body0_e1879: f64 = (locals.var_i * assign2780_body0_e1878);
        let assign2780_body0_e1880: f64 = (assign2780_body0_e1874 + assign2780_body0_e1879);
        let assign2780_body0_e1881: f64 = (1.0 / assign2780_body0_e1880);
        let assign2780_body0_e1882: f64 = (locals.var_t1 + assign2780_body0_e1881);
        let assign2780_body0_e1887: f64 = (0.5 * locals.var_lgate);
        let assign2780_body0_e1888: f64 = (p.p11 + assign2780_body0_e1887);
        let assign2780_body0_e1892: f64 = (p.p12 + locals.var_lgate);
        let assign2780_body0_e1893: f64 = (locals.var_i * assign2780_body0_e1892);
        let assign2780_body0_e1894: f64 = (assign2780_body0_e1888 + assign2780_body0_e1893);
        let assign2780_body0_e1895: f64 = (1.0 / assign2780_body0_e1894);
        let assign2780_body0_e1896: f64 = (assign2780_body0_e1882 + assign2780_body0_e1895);
        (assign2780_body0_e1896, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
            locals.var_t1 = assign2780_body0_e1898;
            locals.var_t1_dn0 = assign2780_body0_e1898_d_n0;
            locals.var_t1_dn2 = assign2780_body0_e1898_d_n2;
            locals.var_t1_dn6 = assign2780_body0_e1898_d_n6;
            locals.var_t1_dn7 = assign2780_body0_e1898_d_n7;
            locals.var_t1_dn10 = assign2780_body0_e1898_d_n10;
            locals.var_t1_dn11 = assign2780_body0_e1898_d_n11;
            locals.var_t1_dn12 = assign2780_body0_e1898_d_n12;
            locals.var_t1_dn17 = assign2780_body0_e1898_d_n17;
            locals.var_t1_rv = 0.0;
            let (assign2780_body1_e1904,) = {
    if (locals.var_guard13 != 0.0) {
        let assign2780_body1_e1902: f64 = (locals.var_i + 1.0);
        (assign2780_body1_e1902,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign2780_body1_e1904;
            locals.var_i_rv = 0.0;
        }

        let (assign2790_e1912, assign2790_e1912_d_n0, assign2790_e1912_d_n2, assign2790_e1912_d_n6, assign2790_e1912_d_n7, assign2790_e1912_d_n10, assign2790_e1912_d_n11, assign2790_e1912_d_n12, assign2790_e1912_d_n17,) = {
    if (locals.var_guard13 != 0.0) {
        let assign2790_e1908: f64 = (2.0 * p.p9);
        let assign2790_e1910: f64 = (assign2790_e1908 / locals.var_t1);
        (assign2790_e1910, (-((assign2790_e1908 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign2790_e1908 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign2790_e1908 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign2790_e1908 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign2790_e1908 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign2790_e1908 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((assign2790_e1908 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((assign2790_e1908 * locals.var_t1_dn17) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn6, locals.var_lod_half_dn7, locals.var_lod_half_dn10, locals.var_lod_half_dn11, locals.var_lod_half_dn12, locals.var_lod_half_dn17,)
    }
};
        locals.var_lod_half = assign2790_e1912;
        locals.var_lod_half_dn0 = assign2790_e1912_d_n0;
        locals.var_lod_half_dn2 = assign2790_e1912_d_n2;
        locals.var_lod_half_dn6 = assign2790_e1912_d_n6;
        locals.var_lod_half_dn7 = assign2790_e1912_d_n7;
        locals.var_lod_half_dn10 = assign2790_e1912_d_n10;
        locals.var_lod_half_dn11 = assign2790_e1912_d_n11;
        locals.var_lod_half_dn12 = assign2790_e1912_d_n12;
        locals.var_lod_half_dn17 = assign2790_e1912_d_n17;
        locals.var_lod_half_rv = 0.0;

        let (assign2800_e1917, assign2800_e1917_d_n0, assign2800_e1917_d_n2, assign2800_e1917_d_n6, assign2800_e1917_d_n7, assign2800_e1917_d_n10, assign2800_e1917_d_n11, assign2800_e1917_d_n12, assign2800_e1917_d_n17,) = {
    if (locals.var_guard13 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn6, locals.var_lod_half_dn7, locals.var_lod_half_dn10, locals.var_lod_half_dn11, locals.var_lod_half_dn12, locals.var_lod_half_dn17,)
    }
};
        locals.var_lod_half = assign2800_e1917;
        locals.var_lod_half_dn0 = assign2800_e1917_d_n0;
        locals.var_lod_half_dn2 = assign2800_e1917_d_n2;
        locals.var_lod_half_dn6 = assign2800_e1917_d_n6;
        locals.var_lod_half_dn7 = assign2800_e1917_d_n7;
        locals.var_lod_half_dn10 = assign2800_e1917_d_n10;
        locals.var_lod_half_dn11 = assign2800_e1917_d_n11;
        locals.var_lod_half_dn12 = assign2800_e1917_d_n12;
        locals.var_lod_half_dn17 = assign2800_e1917_d_n17;
        locals.var_lod_half_rv = 0.0;

        let assign2810_e1920: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign2810_e1920;
        locals.var_guard14_rv = 0.0;

        let (assign2820_e1928, assign2820_e1928_d_n0, assign2820_e1928_d_n2, assign2820_e1928_d_n6, assign2820_e1928_d_n7, assign2820_e1928_d_n10, assign2820_e1928_d_n11, assign2820_e1928_d_n12, assign2820_e1928_d_n17,) = {
    if (locals.var_guard14 != 0.0) {
        let assign2820_e1925: f64 = (1.0 + p.p162);
        let assign2820_e1926: f64 = (1.0 / assign2820_e1925);
        (assign2820_e1926, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign2820_e1928;
        locals.var_t1_dn0 = assign2820_e1928_d_n0;
        locals.var_t1_dn2 = assign2820_e1928_d_n2;
        locals.var_t1_dn6 = assign2820_e1928_d_n6;
        locals.var_t1_dn7 = assign2820_e1928_d_n7;
        locals.var_t1_dn10 = assign2820_e1928_d_n10;
        locals.var_t1_dn11 = assign2820_e1928_d_n11;
        locals.var_t1_dn12 = assign2820_e1928_d_n12;
        locals.var_t1_dn17 = assign2820_e1928_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign2830_e1936, assign2830_e1936_d_n0, assign2830_e1936_d_n2, assign2830_e1936_d_n6, assign2830_e1936_d_n7, assign2830_e1936_d_n10, assign2830_e1936_d_n11, assign2830_e1936_d_n12, assign2830_e1936_d_n17,) = {
    if (locals.var_guard14 != 0.0) {
        let assign2830_e1932: f64 = (p.p161 / locals.var_lod_half);
        let assign2830_e1934: f64 = (assign2830_e1932).powf(p.p163);
        (assign2830_e1934, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign2830_e1932))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign2830_e1932))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign2830_e1932))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))) / assign2830_e1932))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign2830_e1932))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))) / assign2830_e1932))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn12) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn12) / (locals.var_lod_half * locals.var_lod_half))) / assign2830_e1932))) }, if 0.0 == 0.0 && ((p.p163) as f64).is_finite() && ((p.p163) as f64).fract() == 0.0 { if p.p163 == 0.0 { 0.0 } else { (p.p163 * ((assign2830_e1932).powf(p.p163 - 1.0) * (-((p.p161 * locals.var_lod_half_dn17) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2830_e1934 * (p.p163 * ((-((p.p161 * locals.var_lod_half_dn17) / (locals.var_lod_half * locals.var_lod_half))) / assign2830_e1932))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign2830_e1936;
        locals.var_t2_dn0 = assign2830_e1936_d_n0;
        locals.var_t2_dn2 = assign2830_e1936_d_n2;
        locals.var_t2_dn6 = assign2830_e1936_d_n6;
        locals.var_t2_dn7 = assign2830_e1936_d_n7;
        locals.var_t2_dn10 = assign2830_e1936_d_n10;
        locals.var_t2_dn11 = assign2830_e1936_d_n11;
        locals.var_t2_dn12 = assign2830_e1936_d_n12;
        locals.var_t2_dn17 = assign2830_e1936_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign2840_e1944, assign2840_e1944_d_n0, assign2840_e1944_d_n2, assign2840_e1944_d_n6, assign2840_e1944_d_n7, assign2840_e1944_d_n10, assign2840_e1944_d_n11, assign2840_e1944_d_n12, assign2840_e1944_d_n17,) = {
    if (locals.var_guard14 != 0.0) {
        let assign2840_e1940: f64 = (p.p161 / locals.var_lod_half_ref);
        let assign2840_e1942: f64 = (assign2840_e1940).powf(p.p163);
        (assign2840_e1942, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign2840_e1944;
        locals.var_t3_dn0 = assign2840_e1944_d_n0;
        locals.var_t3_dn2 = assign2840_e1944_d_n2;
        locals.var_t3_dn6 = assign2840_e1944_d_n6;
        locals.var_t3_dn7 = assign2840_e1944_d_n7;
        locals.var_t3_dn10 = assign2840_e1944_d_n10;
        locals.var_t3_dn11 = assign2840_e1944_d_n11;
        locals.var_t3_dn12 = assign2840_e1944_d_n12;
        locals.var_t3_dn17 = assign2840_e1944_d_n17;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2850_e1960, assign2850_e1960_d_n0, assign2850_e1960_d_n2, assign2850_e1960_d_n6, assign2850_e1960_d_n7, assign2850_e1960_d_n10, assign2850_e1960_d_n11, assign2850_e1960_d_n12, assign2850_e1960_d_n17,) = {
    if (locals.var_guard14 != 0.0) {
        let assign2850_e1950: f64 = (locals.var_t1 * locals.var_t2);
        let assign2850_e1951: f64 = (1.0 + assign2850_e1950);
        let assign2850_e1952: f64 = (locals.var_nsubpp * assign2850_e1951);
        let assign2850_e1956: f64 = (locals.var_t1 * locals.var_t3);
        let assign2850_e1957: f64 = (1.0 + assign2850_e1956);
        let assign2850_e1958: f64 = (assign2850_e1952 / assign2850_e1957);
        (assign2850_e1958, ((((locals.var_nsubpp * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0))) * assign2850_e1957) - (assign2850_e1952 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign2850_e1957 * assign2850_e1957)), ((((locals.var_nsubpp * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2))) * assign2850_e1957) - (assign2850_e1952 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign2850_e1957 * assign2850_e1957)), ((((locals.var_nsubpp * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6))) * assign2850_e1957) - (assign2850_e1952 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign2850_e1957 * assign2850_e1957)), ((((locals.var_nsubpp * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7))) * assign2850_e1957) - (assign2850_e1952 * ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)))) / (assign2850_e1957 * assign2850_e1957)), ((((locals.var_nsubpp * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10))) * assign2850_e1957) - (assign2850_e1952 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign2850_e1957 * assign2850_e1957)), ((((locals.var_nsubpp * ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11))) * assign2850_e1957) - (assign2850_e1952 * ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)))) / (assign2850_e1957 * assign2850_e1957)), ((((locals.var_nsubpp * ((locals.var_t1_dn12 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn12))) * assign2850_e1957) - (assign2850_e1952 * ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)))) / (assign2850_e1957 * assign2850_e1957)), ((((locals.var_nsubpp * ((locals.var_t1_dn17 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn17))) * assign2850_e1957) - (assign2850_e1952 * ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)))) / (assign2850_e1957 * assign2850_e1957)),)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn6, locals.var_nsubps_dn7, locals.var_nsubps_dn10, locals.var_nsubps_dn11, locals.var_nsubps_dn12, locals.var_nsubps_dn17,)
    }
};
        locals.var_nsubps = assign2850_e1960;
        locals.var_nsubps_dn0 = assign2850_e1960_d_n0;
        locals.var_nsubps_dn2 = assign2850_e1960_d_n2;
        locals.var_nsubps_dn6 = assign2850_e1960_d_n6;
        locals.var_nsubps_dn7 = assign2850_e1960_d_n7;
        locals.var_nsubps_dn10 = assign2850_e1960_d_n10;
        locals.var_nsubps_dn11 = assign2850_e1960_d_n11;
        locals.var_nsubps_dn12 = assign2850_e1960_d_n12;
        locals.var_nsubps_dn17 = assign2850_e1960_d_n17;
        locals.var_nsubps_rv = 0.0;

        let (assign2860_e1965, assign2860_e1965_d_n0, assign2860_e1965_d_n2, assign2860_e1965_d_n6, assign2860_e1965_d_n7, assign2860_e1965_d_n10, assign2860_e1965_d_n11, assign2860_e1965_d_n12, assign2860_e1965_d_n17,) = {
    if (locals.var_guard14 == 0.0) {
        (locals.var_nsubpp, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn6, locals.var_nsubps_dn7, locals.var_nsubps_dn10, locals.var_nsubps_dn11, locals.var_nsubps_dn12, locals.var_nsubps_dn17,)
    }
};
        locals.var_nsubps = assign2860_e1965;
        locals.var_nsubps_dn0 = assign2860_e1965_d_n0;
        locals.var_nsubps_dn2 = assign2860_e1965_d_n2;
        locals.var_nsubps_dn6 = assign2860_e1965_d_n6;
        locals.var_nsubps_dn7 = assign2860_e1965_d_n7;
        locals.var_nsubps_dn10 = assign2860_e1965_d_n10;
        locals.var_nsubps_dn11 = assign2860_e1965_d_n11;
        locals.var_nsubps_dn12 = assign2860_e1965_d_n12;
        locals.var_nsubps_dn17 = assign2860_e1965_d_n17;
        locals.var_nsubps_rv = 0.0;

        let assign2870_e1970: f64 = (locals.var_wg).powf(p.p200);
        let assign2870_e1971: f64 = (p.p199 / assign2870_e1970);
        let assign2870_e1972: f64 = (1.0 + assign2870_e1971);
        let assign2870_e1977: f64 = (locals.var_lgle).powf(p.p203);
        let assign2870_e1978: f64 = (p.p202 / assign2870_e1977);
        let assign2870_e1979: f64 = (1.0 + assign2870_e1978);
        let assign2870_e1980: f64 = (assign2870_e1972 * assign2870_e1979);
        locals.var_t2 = assign2870_e1980;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn7 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn12 = 0.0;
        locals.var_t2_dn17 = 0.0;
        locals.var_t2_rv = 0.0;

        let assign2880_e1983: f64 = (locals.var_mks_nsubcmax / locals.var_mks_nsubs);
        locals.var_t3 = assign2880_e1983;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn7 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn11 = 0.0;
        locals.var_t3_dn12 = 0.0;
        locals.var_t3_dn17 = 0.0;
        locals.var_t3_rv = 0.0;

        let assign2890_e1986: f64 = (locals.var_t3 - locals.var_t2);
        let assign2890_e1988: f64 = (assign2890_e1986 - 0.01);
        locals.var_tmf1 = assign2890_e1988;
        locals.var_tmf1_dn0 = (locals.var_t3_dn0 - locals.var_t2_dn0);
        locals.var_tmf1_dn2 = (locals.var_t3_dn2 - locals.var_t2_dn2);
        locals.var_tmf1_dn6 = (locals.var_t3_dn6 - locals.var_t2_dn6);
        locals.var_tmf1_dn7 = (locals.var_t3_dn7 - locals.var_t2_dn7);
        locals.var_tmf1_dn10 = (locals.var_t3_dn10 - locals.var_t2_dn10);
        locals.var_tmf1_dn11 = (locals.var_t3_dn11 - locals.var_t2_dn11);
        locals.var_tmf1_dn12 = (locals.var_t3_dn12 - locals.var_t2_dn12);
        locals.var_tmf1_dn17 = (locals.var_t3_dn17 - locals.var_t2_dn17);
        locals.var_tmf1_rv = 0.0;

        let assign2900_e1991: f64 = (4.0 * locals.var_t3);
        let assign2900_e1993: f64 = (assign2900_e1991 * 0.01);
        locals.var_tmf2 = assign2900_e1993;
        locals.var_tmf2_dn0 = ((4.0 * locals.var_t3_dn0) * 0.01);
        locals.var_tmf2_dn2 = ((4.0 * locals.var_t3_dn2) * 0.01);
        locals.var_tmf2_dn6 = ((4.0 * locals.var_t3_dn6) * 0.01);
        locals.var_tmf2_dn7 = ((4.0 * locals.var_t3_dn7) * 0.01);
        locals.var_tmf2_dn10 = ((4.0 * locals.var_t3_dn10) * 0.01);
        locals.var_tmf2_dn11 = ((4.0 * locals.var_t3_dn11) * 0.01);
        locals.var_tmf2_dn12 = ((4.0 * locals.var_t3_dn12) * 0.01);
        locals.var_tmf2_dn17 = ((4.0 * locals.var_t3_dn17) * 0.01);
        locals.var_tmf2_rv = 0.0;

        let (assign2910_e2000, assign2910_e2000_d_n0, assign2910_e2000_d_n2, assign2910_e2000_d_n6, assign2910_e2000_d_n7, assign2910_e2000_d_n10, assign2910_e2000_d_n11, assign2910_e2000_d_n12, assign2910_e2000_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign2910_e1999: f64 = (-locals.var_tmf2);
        (assign2910_e1999, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
        locals.var_tmf2 = assign2910_e2000;
        locals.var_tmf2_dn0 = assign2910_e2000_d_n0;
        locals.var_tmf2_dn2 = assign2910_e2000_d_n2;
        locals.var_tmf2_dn6 = assign2910_e2000_d_n6;
        locals.var_tmf2_dn7 = assign2910_e2000_d_n7;
        locals.var_tmf2_dn10 = assign2910_e2000_d_n10;
        locals.var_tmf2_dn11 = assign2910_e2000_d_n11;
        locals.var_tmf2_dn12 = assign2910_e2000_d_n12;
        locals.var_tmf2_dn17 = assign2910_e2000_d_n17;
        locals.var_tmf2_rv = 0.0;

        let assign2920_e2003: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign2920_e2005: f64 = (assign2920_e2003 + locals.var_tmf2);
        let assign2920_e2006: f64 = (assign2920_e2005).sqrt();
        locals.var_tmf2 = assign2920_e2006;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign2920_e2006));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign2920_e2006));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign2920_e2006));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign2920_e2006));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign2920_e2006));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign2920_e2006));
        locals.var_tmf2_dn12 = ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign2920_e2006));
        locals.var_tmf2_dn17 = ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign2920_e2006));
        locals.var_tmf2_rv = 0.0;

        let assign2930_e2011: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign2930_e2012: f64 = (0.5 * assign2930_e2011);
        let assign2930_e2013: f64 = (locals.var_t3 - assign2930_e2012);
        locals.var_t1 = assign2930_e2013;
        locals.var_t1_dn0 = (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)));
        locals.var_t1_dn2 = (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)));
        locals.var_t1_dn6 = (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)));
        locals.var_t1_dn7 = (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)));
        locals.var_t1_dn10 = (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)));
        locals.var_t1_dn11 = (locals.var_t3_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)));
        locals.var_t1_dn12 = (locals.var_t3_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12)));
        locals.var_t1_dn17 = (locals.var_t3_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17)));
        locals.var_t1_rv = 0.0;

        let assign2940_e2016: f64 = (locals.var_mks_nsubs * locals.var_t1);
        locals.var_uc_nsubs = assign2940_e2016;
        locals.var_uc_nsubs_dn0 = (locals.var_mks_nsubs * locals.var_t1_dn0);
        locals.var_uc_nsubs_dn2 = (locals.var_mks_nsubs * locals.var_t1_dn2);
        locals.var_uc_nsubs_dn6 = (locals.var_mks_nsubs * locals.var_t1_dn6);
        locals.var_uc_nsubs_dn7 = (locals.var_mks_nsubs * locals.var_t1_dn7);
        locals.var_uc_nsubs_dn10 = (locals.var_mks_nsubs * locals.var_t1_dn10);
        locals.var_uc_nsubs_dn11 = (locals.var_mks_nsubs * locals.var_t1_dn11);
        locals.var_uc_nsubs_dn12 = (locals.var_mks_nsubs * locals.var_t1_dn12);
        locals.var_uc_nsubs_dn17 = (locals.var_mks_nsubs * locals.var_t1_dn17);
        locals.var_uc_nsubs_rv = 0.0;

        let assign2950_e2019: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign2950_e2019;
        locals.var_guard15_rv = 0.0;

        let (assign2960_e2027, assign2960_e2027_d_n0, assign2960_e2027_d_n2, assign2960_e2027_d_n6, assign2960_e2027_d_n7, assign2960_e2027_d_n10, assign2960_e2027_d_n11, assign2960_e2027_d_n12, assign2960_e2027_d_n17,) = {
    if (locals.var_guard15 != 0.0) {
        let assign2960_e2024: f64 = (1.0 + p.p165);
        let assign2960_e2025: f64 = (1.0 / assign2960_e2024);
        (assign2960_e2025, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign2960_e2027;
        locals.var_t1_dn0 = assign2960_e2027_d_n0;
        locals.var_t1_dn2 = assign2960_e2027_d_n2;
        locals.var_t1_dn6 = assign2960_e2027_d_n6;
        locals.var_t1_dn7 = assign2960_e2027_d_n7;
        locals.var_t1_dn10 = assign2960_e2027_d_n10;
        locals.var_t1_dn11 = assign2960_e2027_d_n11;
        locals.var_t1_dn12 = assign2960_e2027_d_n12;
        locals.var_t1_dn17 = assign2960_e2027_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign2970_e2035, assign2970_e2035_d_n0, assign2970_e2035_d_n2, assign2970_e2035_d_n6, assign2970_e2035_d_n7, assign2970_e2035_d_n10, assign2970_e2035_d_n11, assign2970_e2035_d_n12, assign2970_e2035_d_n17,) = {
    if (locals.var_guard15 != 0.0) {
        let assign2970_e2031: f64 = (p.p164 / locals.var_lod_half);
        let assign2970_e2033: f64 = (assign2970_e2031).powf(p.p166);
        (assign2970_e2033, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign2970_e2031))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign2970_e2031))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign2970_e2031))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))) / assign2970_e2031))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign2970_e2031))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))) / assign2970_e2031))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn12) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn12) / (locals.var_lod_half * locals.var_lod_half))) / assign2970_e2031))) }, if 0.0 == 0.0 && ((p.p166) as f64).is_finite() && ((p.p166) as f64).fract() == 0.0 { if p.p166 == 0.0 { 0.0 } else { (p.p166 * ((assign2970_e2031).powf(p.p166 - 1.0) * (-((p.p164 * locals.var_lod_half_dn17) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2970_e2033 * (p.p166 * ((-((p.p164 * locals.var_lod_half_dn17) / (locals.var_lod_half * locals.var_lod_half))) / assign2970_e2031))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign2970_e2035;
        locals.var_t2_dn0 = assign2970_e2035_d_n0;
        locals.var_t2_dn2 = assign2970_e2035_d_n2;
        locals.var_t2_dn6 = assign2970_e2035_d_n6;
        locals.var_t2_dn7 = assign2970_e2035_d_n7;
        locals.var_t2_dn10 = assign2970_e2035_d_n10;
        locals.var_t2_dn11 = assign2970_e2035_d_n11;
        locals.var_t2_dn12 = assign2970_e2035_d_n12;
        locals.var_t2_dn17 = assign2970_e2035_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign2980_e2043, assign2980_e2043_d_n0, assign2980_e2043_d_n2, assign2980_e2043_d_n6, assign2980_e2043_d_n7, assign2980_e2043_d_n10, assign2980_e2043_d_n11, assign2980_e2043_d_n12, assign2980_e2043_d_n17,) = {
    if (locals.var_guard15 != 0.0) {
        let assign2980_e2039: f64 = (p.p164 / locals.var_lod_half_ref);
        let assign2980_e2041: f64 = (assign2980_e2039).powf(p.p166);
        (assign2980_e2041, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign2980_e2043;
        locals.var_t3_dn0 = assign2980_e2043_d_n0;
        locals.var_t3_dn2 = assign2980_e2043_d_n2;
        locals.var_t3_dn6 = assign2980_e2043_d_n6;
        locals.var_t3_dn7 = assign2980_e2043_d_n7;
        locals.var_t3_dn10 = assign2980_e2043_d_n10;
        locals.var_t3_dn11 = assign2980_e2043_d_n11;
        locals.var_t3_dn12 = assign2980_e2043_d_n12;
        locals.var_t3_dn17 = assign2980_e2043_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign2990_e2059, assign2990_e2059_d_n0, assign2990_e2059_d_n2, assign2990_e2059_d_n6, assign2990_e2059_d_n7, assign2990_e2059_d_n10, assign2990_e2059_d_n11, assign2990_e2059_d_n12, assign2990_e2059_d_n17,) = {
    if (locals.var_guard15 != 0.0) {
        let assign2990_e2049: f64 = (locals.var_t1 * locals.var_t2);
        let assign2990_e2050: f64 = (1.0 + assign2990_e2049);
        let assign2990_e2051: f64 = (locals.var_uc_nsubs * assign2990_e2050);
        let assign2990_e2055: f64 = (locals.var_t1 * locals.var_t3);
        let assign2990_e2056: f64 = (1.0 + assign2990_e2055);
        let assign2990_e2057: f64 = (assign2990_e2051 / assign2990_e2056);
        (assign2990_e2057, (((((locals.var_uc_nsubs_dn0 * assign2990_e2050) + (locals.var_uc_nsubs * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)))) * assign2990_e2056) - (assign2990_e2051 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign2990_e2056 * assign2990_e2056)), (((((locals.var_uc_nsubs_dn2 * assign2990_e2050) + (locals.var_uc_nsubs * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)))) * assign2990_e2056) - (assign2990_e2051 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign2990_e2056 * assign2990_e2056)), (((((locals.var_uc_nsubs_dn6 * assign2990_e2050) + (locals.var_uc_nsubs * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * assign2990_e2056) - (assign2990_e2051 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign2990_e2056 * assign2990_e2056)), (((((locals.var_uc_nsubs_dn7 * assign2990_e2050) + (locals.var_uc_nsubs * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * assign2990_e2056) - (assign2990_e2051 * ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)))) / (assign2990_e2056 * assign2990_e2056)), (((((locals.var_uc_nsubs_dn10 * assign2990_e2050) + (locals.var_uc_nsubs * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)))) * assign2990_e2056) - (assign2990_e2051 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign2990_e2056 * assign2990_e2056)), (((((locals.var_uc_nsubs_dn11 * assign2990_e2050) + (locals.var_uc_nsubs * ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)))) * assign2990_e2056) - (assign2990_e2051 * ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)))) / (assign2990_e2056 * assign2990_e2056)), (((((locals.var_uc_nsubs_dn12 * assign2990_e2050) + (locals.var_uc_nsubs * ((locals.var_t1_dn12 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn12)))) * assign2990_e2056) - (assign2990_e2051 * ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)))) / (assign2990_e2056 * assign2990_e2056)), (((((locals.var_uc_nsubs_dn17 * assign2990_e2050) + (locals.var_uc_nsubs * ((locals.var_t1_dn17 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn17)))) * assign2990_e2056) - (assign2990_e2051 * ((locals.var_t1_dn17 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn17)))) / (assign2990_e2056 * assign2990_e2056)),)
    } else {
        (locals.var_uc_nsubs, locals.var_uc_nsubs_dn0, locals.var_uc_nsubs_dn2, locals.var_uc_nsubs_dn6, locals.var_uc_nsubs_dn7, locals.var_uc_nsubs_dn10, locals.var_uc_nsubs_dn11, locals.var_uc_nsubs_dn12, locals.var_uc_nsubs_dn17,)
    }
};
        locals.var_uc_nsubs = assign2990_e2059;
        locals.var_uc_nsubs_dn0 = assign2990_e2059_d_n0;
        locals.var_uc_nsubs_dn2 = assign2990_e2059_d_n2;
        locals.var_uc_nsubs_dn6 = assign2990_e2059_d_n6;
        locals.var_uc_nsubs_dn7 = assign2990_e2059_d_n7;
        locals.var_uc_nsubs_dn10 = assign2990_e2059_d_n10;
        locals.var_uc_nsubs_dn11 = assign2990_e2059_d_n11;
        locals.var_uc_nsubs_dn12 = assign2990_e2059_d_n12;
        locals.var_uc_nsubs_dn17 = assign2990_e2059_d_n17;
        locals.var_uc_nsubs_rv = 0.0;

        let assign3000_e2066: f64 = if ((locals.var_lgleff > p.p72) || (p.p72 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard16 = assign3000_e2066;
        locals.var_guard16_rv = 0.0;

        let (assign3010_e2080, assign3010_e2080_d_n0, assign3010_e2080_d_n2, assign3010_e2080_d_n6, assign3010_e2080_d_n7, assign3010_e2080_d_n10, assign3010_e2080_d_n11, assign3010_e2080_d_n12, assign3010_e2080_d_n17,) = {
    if (locals.var_guard16 != 0.0) {
        let assign3010_e2071: f64 = (locals.var_lgleff - p.p72);
        let assign3010_e2072: f64 = (locals.var_uc_nsubs * assign3010_e2071);
        let assign3010_e2075: f64 = (locals.var_nsubps * p.p72);
        let assign3010_e2076: f64 = (assign3010_e2072 + assign3010_e2075);
        let assign3010_e2078: f64 = (assign3010_e2076 / locals.var_lgleff);
        (assign3010_e2078, (((locals.var_uc_nsubs_dn0 * assign3010_e2071) + (locals.var_nsubps_dn0 * p.p72)) / locals.var_lgleff), (((locals.var_uc_nsubs_dn2 * assign3010_e2071) + (locals.var_nsubps_dn2 * p.p72)) / locals.var_lgleff), (((locals.var_uc_nsubs_dn6 * assign3010_e2071) + (locals.var_nsubps_dn6 * p.p72)) / locals.var_lgleff), (((locals.var_uc_nsubs_dn7 * assign3010_e2071) + (locals.var_nsubps_dn7 * p.p72)) / locals.var_lgleff), (((locals.var_uc_nsubs_dn10 * assign3010_e2071) + (locals.var_nsubps_dn10 * p.p72)) / locals.var_lgleff), (((locals.var_uc_nsubs_dn11 * assign3010_e2071) + (locals.var_nsubps_dn11 * p.p72)) / locals.var_lgleff), (((locals.var_uc_nsubs_dn12 * assign3010_e2071) + (locals.var_nsubps_dn12 * p.p72)) / locals.var_lgleff), (((locals.var_uc_nsubs_dn17 * assign3010_e2071) + (locals.var_nsubps_dn17 * p.p72)) / locals.var_lgleff),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn10, locals.var_nsub_dn11, locals.var_nsub_dn12, locals.var_nsub_dn17,)
    }
};
        locals.var_nsub = assign3010_e2080;
        locals.var_nsub_dn0 = assign3010_e2080_d_n0;
        locals.var_nsub_dn2 = assign3010_e2080_d_n2;
        locals.var_nsub_dn6 = assign3010_e2080_d_n6;
        locals.var_nsub_dn7 = assign3010_e2080_d_n7;
        locals.var_nsub_dn10 = assign3010_e2080_d_n10;
        locals.var_nsub_dn11 = assign3010_e2080_d_n11;
        locals.var_nsub_dn12 = assign3010_e2080_d_n12;
        locals.var_nsub_dn17 = assign3010_e2080_d_n17;
        locals.var_nsub_rv = 0.0;

        let (assign3020_e2095, assign3020_e2095_d_n0, assign3020_e2095_d_n2, assign3020_e2095_d_n6, assign3020_e2095_d_n7, assign3020_e2095_d_n10, assign3020_e2095_d_n11, assign3020_e2095_d_n12, assign3020_e2095_d_n17,) = {
    if (locals.var_guard16 == 0.0) {
        let assign3020_e2086: f64 = (locals.var_nsubps - locals.var_uc_nsubs);
        let assign3020_e2089: f64 = (p.p72 - locals.var_lgleff);
        let assign3020_e2090: f64 = (assign3020_e2086 * assign3020_e2089);
        let assign3020_e2092: f64 = (assign3020_e2090 / p.p72);
        let assign3020_e2093: f64 = (locals.var_nsubps + assign3020_e2092);
        (assign3020_e2093, (locals.var_nsubps_dn0 + (((locals.var_nsubps_dn0 - locals.var_uc_nsubs_dn0) * assign3020_e2089) / p.p72)), (locals.var_nsubps_dn2 + (((locals.var_nsubps_dn2 - locals.var_uc_nsubs_dn2) * assign3020_e2089) / p.p72)), (locals.var_nsubps_dn6 + (((locals.var_nsubps_dn6 - locals.var_uc_nsubs_dn6) * assign3020_e2089) / p.p72)), (locals.var_nsubps_dn7 + (((locals.var_nsubps_dn7 - locals.var_uc_nsubs_dn7) * assign3020_e2089) / p.p72)), (locals.var_nsubps_dn10 + (((locals.var_nsubps_dn10 - locals.var_uc_nsubs_dn10) * assign3020_e2089) / p.p72)), (locals.var_nsubps_dn11 + (((locals.var_nsubps_dn11 - locals.var_uc_nsubs_dn11) * assign3020_e2089) / p.p72)), (locals.var_nsubps_dn12 + (((locals.var_nsubps_dn12 - locals.var_uc_nsubs_dn12) * assign3020_e2089) / p.p72)), (locals.var_nsubps_dn17 + (((locals.var_nsubps_dn17 - locals.var_uc_nsubs_dn17) * assign3020_e2089) / p.p72)),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn10, locals.var_nsub_dn11, locals.var_nsub_dn12, locals.var_nsub_dn17,)
    }
};
        locals.var_nsub = assign3020_e2095;
        locals.var_nsub_dn0 = assign3020_e2095_d_n0;
        locals.var_nsub_dn2 = assign3020_e2095_d_n2;
        locals.var_nsub_dn6 = assign3020_e2095_d_n6;
        locals.var_nsub_dn7 = assign3020_e2095_d_n7;
        locals.var_nsub_dn10 = assign3020_e2095_d_n10;
        locals.var_nsub_dn11 = assign3020_e2095_d_n11;
        locals.var_nsub_dn12 = assign3020_e2095_d_n12;
        locals.var_nsub_dn17 = assign3020_e2095_d_n17;
        locals.var_nsub_rv = 0.0;

        let assign3030_e2098: f64 = (1.6021918e-19 * locals.var_nsub);
        locals.var_q_nsub = assign3030_e2098;
        locals.var_q_nsub_dn0 = (1.6021918e-19 * locals.var_nsub_dn0);
        locals.var_q_nsub_dn2 = (1.6021918e-19 * locals.var_nsub_dn2);
        locals.var_q_nsub_dn6 = (1.6021918e-19 * locals.var_nsub_dn6);
        locals.var_q_nsub_dn7 = (1.6021918e-19 * locals.var_nsub_dn7);
        locals.var_q_nsub_dn10 = (1.6021918e-19 * locals.var_nsub_dn10);
        locals.var_q_nsub_dn11 = (1.6021918e-19 * locals.var_nsub_dn11);
        locals.var_q_nsub_dn12 = (1.6021918e-19 * locals.var_nsub_dn12);
        locals.var_q_nsub_dn17 = (1.6021918e-19 * locals.var_nsub_dn17);
        locals.var_q_nsub_rv = 0.0;

        let assign3040_e2101: f64 = (locals.var_q_nsub * 1.034943e-10);
        locals.var_qnsub_esi = assign3040_e2101;
        locals.var_qnsub_esi_dn0 = (locals.var_q_nsub_dn0 * 1.034943e-10);
        locals.var_qnsub_esi_dn2 = (locals.var_q_nsub_dn2 * 1.034943e-10);
        locals.var_qnsub_esi_dn6 = (locals.var_q_nsub_dn6 * 1.034943e-10);
        locals.var_qnsub_esi_dn7 = (locals.var_q_nsub_dn7 * 1.034943e-10);
        locals.var_qnsub_esi_dn10 = (locals.var_q_nsub_dn10 * 1.034943e-10);
        locals.var_qnsub_esi_dn11 = (locals.var_q_nsub_dn11 * 1.034943e-10);
        locals.var_qnsub_esi_dn12 = (locals.var_q_nsub_dn12 * 1.034943e-10);
        locals.var_qnsub_esi_dn17 = (locals.var_q_nsub_dn17 * 1.034943e-10);
        locals.var_qnsub_esi_rv = 0.0;

        let assign3050_e2104: f64 = (2.0 * locals.var_qnsub_esi);
        locals.var_qnsub_esi2 = assign3050_e2104;
        locals.var_qnsub_esi2_dn0 = (2.0 * locals.var_qnsub_esi_dn0);
        locals.var_qnsub_esi2_dn2 = (2.0 * locals.var_qnsub_esi_dn2);
        locals.var_qnsub_esi2_dn6 = (2.0 * locals.var_qnsub_esi_dn6);
        locals.var_qnsub_esi2_dn7 = (2.0 * locals.var_qnsub_esi_dn7);
        locals.var_qnsub_esi2_dn10 = (2.0 * locals.var_qnsub_esi_dn10);
        locals.var_qnsub_esi2_dn11 = (2.0 * locals.var_qnsub_esi_dn11);
        locals.var_qnsub_esi2_dn12 = (2.0 * locals.var_qnsub_esi_dn12);
        locals.var_qnsub_esi2_dn17 = (2.0 * locals.var_qnsub_esi_dn17);
        locals.var_qnsub_esi2_rv = 0.0;

        let assign3060_e2108: f64 = (2.0 * p.p72);
        let assign3060_e2113: f64 = if ((locals.var_lgleff <= assign3060_e2108) && (p.p72 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard17 = assign3060_e2113;
        locals.var_guard17_rv = 0.0;

        let (assign3070_e2129, assign3070_e2129_d_n0, assign3070_e2129_d_n2, assign3070_e2129_d_n6, assign3070_e2129_d_n7, assign3070_e2129_d_n10, assign3070_e2129_d_n11, assign3070_e2129_d_n12, assign3070_e2129_d_n17,) = {
    if (locals.var_guard17 != 0.0) {
        let assign3070_e2117: f64 = (2.0 * locals.var_nsubps);
        let assign3070_e2120: f64 = (locals.var_nsubps - locals.var_uc_nsubs);
        let assign3070_e2122: f64 = (assign3070_e2120 * locals.var_lgleff);
        let assign3070_e2124: f64 = (assign3070_e2122 / p.p72);
        let assign3070_e2125: f64 = (assign3070_e2117 - assign3070_e2124);
        let assign3070_e2127: f64 = (assign3070_e2125 - locals.var_uc_nsubs);
        (assign3070_e2127, (((2.0 * locals.var_nsubps_dn0) - (((locals.var_nsubps_dn0 - locals.var_uc_nsubs_dn0) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn0), (((2.0 * locals.var_nsubps_dn2) - (((locals.var_nsubps_dn2 - locals.var_uc_nsubs_dn2) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn2), (((2.0 * locals.var_nsubps_dn6) - (((locals.var_nsubps_dn6 - locals.var_uc_nsubs_dn6) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn6), (((2.0 * locals.var_nsubps_dn7) - (((locals.var_nsubps_dn7 - locals.var_uc_nsubs_dn7) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn7), (((2.0 * locals.var_nsubps_dn10) - (((locals.var_nsubps_dn10 - locals.var_uc_nsubs_dn10) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn10), (((2.0 * locals.var_nsubps_dn11) - (((locals.var_nsubps_dn11 - locals.var_uc_nsubs_dn11) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn11), (((2.0 * locals.var_nsubps_dn12) - (((locals.var_nsubps_dn12 - locals.var_uc_nsubs_dn12) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn12), (((2.0 * locals.var_nsubps_dn17) - (((locals.var_nsubps_dn17 - locals.var_uc_nsubs_dn17) * locals.var_lgleff) / p.p72)) - locals.var_uc_nsubs_dn17),)
    } else {
        (locals.var_nsubb0, locals.var_nsubb0_dn0, locals.var_nsubb0_dn2, locals.var_nsubb0_dn6, locals.var_nsubb0_dn7, locals.var_nsubb0_dn10, locals.var_nsubb0_dn11, locals.var_nsubb0_dn12, locals.var_nsubb0_dn17,)
    }
};
        locals.var_nsubb0 = assign3070_e2129;
        locals.var_nsubb0_dn0 = assign3070_e2129_d_n0;
        locals.var_nsubb0_dn2 = assign3070_e2129_d_n2;
        locals.var_nsubb0_dn6 = assign3070_e2129_d_n6;
        locals.var_nsubb0_dn7 = assign3070_e2129_d_n7;
        locals.var_nsubb0_dn10 = assign3070_e2129_d_n10;
        locals.var_nsubb0_dn11 = assign3070_e2129_d_n11;
        locals.var_nsubb0_dn12 = assign3070_e2129_d_n12;
        locals.var_nsubb0_dn17 = assign3070_e2129_d_n17;
        locals.var_nsubb0_rv = 0.0;

        let (assign3080_e2136, assign3080_e2136_d_n0, assign3080_e2136_d_n2, assign3080_e2136_d_n6, assign3080_e2136_d_n7, assign3080_e2136_d_n10, assign3080_e2136_d_n11, assign3080_e2136_d_n12, assign3080_e2136_d_n17,) = {
    if (locals.var_guard17 != 0.0) {
        let assign3080_e2133: f64 = (locals.var_nsubb0 / locals.var_uc_nsubs);
        let assign3080_e2134: f64 = (assign3080_e2133).ln();
        (assign3080_e2134, ((((locals.var_nsubb0_dn0 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3080_e2133), ((((locals.var_nsubb0_dn2 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3080_e2133), ((((locals.var_nsubb0_dn6 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3080_e2133), ((((locals.var_nsubb0_dn7 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3080_e2133), ((((locals.var_nsubb0_dn10 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3080_e2133), ((((locals.var_nsubb0_dn11 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3080_e2133), ((((locals.var_nsubb0_dn12 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3080_e2133), ((((locals.var_nsubb0_dn17 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign3080_e2133),)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn6, locals.var_ptovr0_dn7, locals.var_ptovr0_dn10, locals.var_ptovr0_dn11, locals.var_ptovr0_dn12, locals.var_ptovr0_dn17,)
    }
};
        locals.var_ptovr0 = assign3080_e2136;
        locals.var_ptovr0_dn0 = assign3080_e2136_d_n0;
        locals.var_ptovr0_dn2 = assign3080_e2136_d_n2;
        locals.var_ptovr0_dn6 = assign3080_e2136_d_n6;
        locals.var_ptovr0_dn7 = assign3080_e2136_d_n7;
        locals.var_ptovr0_dn10 = assign3080_e2136_d_n10;
        locals.var_ptovr0_dn11 = assign3080_e2136_d_n11;
        locals.var_ptovr0_dn12 = assign3080_e2136_d_n12;
        locals.var_ptovr0_dn17 = assign3080_e2136_d_n17;
        locals.var_ptovr0_rv = 0.0;

        let (assign3090_e2141, assign3090_e2141_d_n0, assign3090_e2141_d_n2, assign3090_e2141_d_n6, assign3090_e2141_d_n7, assign3090_e2141_d_n10, assign3090_e2141_d_n11, assign3090_e2141_d_n12, assign3090_e2141_d_n17,) = {
    if (locals.var_guard17 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn6, locals.var_ptovr0_dn7, locals.var_ptovr0_dn10, locals.var_ptovr0_dn11, locals.var_ptovr0_dn12, locals.var_ptovr0_dn17,)
    }
};
        locals.var_ptovr0 = assign3090_e2141;
        locals.var_ptovr0_dn0 = assign3090_e2141_d_n0;
        locals.var_ptovr0_dn2 = assign3090_e2141_d_n2;
        locals.var_ptovr0_dn6 = assign3090_e2141_d_n6;
        locals.var_ptovr0_dn7 = assign3090_e2141_d_n7;
        locals.var_ptovr0_dn10 = assign3090_e2141_d_n10;
        locals.var_ptovr0_dn11 = assign3090_e2141_d_n11;
        locals.var_ptovr0_dn12 = assign3090_e2141_d_n12;
        locals.var_ptovr0_dn17 = assign3090_e2141_d_n17;
        locals.var_ptovr0_rv = 0.0;

        let assign3100_e2144: f64 = (2.0 / 38.68283);
        let assign3100_e2148: f64 = (10400000000.0 / 1e-6);
        let assign3100_e2149: f64 = (locals.var_nsub / assign3100_e2148);
        let assign3100_e2150: f64 = (assign3100_e2149).ln();
        let assign3100_e2151: f64 = (assign3100_e2144 * assign3100_e2150);
        locals.var_pb20 = assign3100_e2151;
        locals.var_pb20_dn0 = (assign3100_e2144 * ((locals.var_nsub_dn0 / assign3100_e2148) / assign3100_e2149));
        locals.var_pb20_dn2 = (assign3100_e2144 * ((locals.var_nsub_dn2 / assign3100_e2148) / assign3100_e2149));
        locals.var_pb20_dn6 = (assign3100_e2144 * ((locals.var_nsub_dn6 / assign3100_e2148) / assign3100_e2149));
        locals.var_pb20_dn7 = (assign3100_e2144 * ((locals.var_nsub_dn7 / assign3100_e2148) / assign3100_e2149));
        locals.var_pb20_dn10 = (assign3100_e2144 * ((locals.var_nsub_dn10 / assign3100_e2148) / assign3100_e2149));
        locals.var_pb20_dn11 = (assign3100_e2144 * ((locals.var_nsub_dn11 / assign3100_e2148) / assign3100_e2149));
        locals.var_pb20_dn12 = (assign3100_e2144 * ((locals.var_nsub_dn12 / assign3100_e2148) / assign3100_e2149));
        locals.var_pb20_dn17 = (assign3100_e2144 * ((locals.var_nsub_dn17 / assign3100_e2148) / assign3100_e2149));
        locals.var_pb20_rv = 0.0;

        let assign3110_e2154: f64 = (2.0 / 38.68283);
        let assign3110_e2158: f64 = (10400000000.0 / 1e-6);
        let assign3110_e2159: f64 = (locals.var_uc_nsubs / assign3110_e2158);
        let assign3110_e2160: f64 = (assign3110_e2159).ln();
        let assign3110_e2161: f64 = (assign3110_e2154 * assign3110_e2160);
        locals.var_pb2c = assign3110_e2161;
        locals.var_pb2c_dn0 = (assign3110_e2154 * ((locals.var_uc_nsubs_dn0 / assign3110_e2158) / assign3110_e2159));
        locals.var_pb2c_dn2 = (assign3110_e2154 * ((locals.var_uc_nsubs_dn2 / assign3110_e2158) / assign3110_e2159));
        locals.var_pb2c_dn6 = (assign3110_e2154 * ((locals.var_uc_nsubs_dn6 / assign3110_e2158) / assign3110_e2159));
        locals.var_pb2c_dn7 = (assign3110_e2154 * ((locals.var_uc_nsubs_dn7 / assign3110_e2158) / assign3110_e2159));
        locals.var_pb2c_dn10 = (assign3110_e2154 * ((locals.var_uc_nsubs_dn10 / assign3110_e2158) / assign3110_e2159));
        locals.var_pb2c_dn11 = (assign3110_e2154 * ((locals.var_uc_nsubs_dn11 / assign3110_e2158) / assign3110_e2159));
        locals.var_pb2c_dn12 = (assign3110_e2154 * ((locals.var_uc_nsubs_dn12 / assign3110_e2158) / assign3110_e2159));
        locals.var_pb2c_dn17 = (assign3110_e2154 * ((locals.var_uc_nsubs_dn17 / assign3110_e2158) / assign3110_e2159));
        locals.var_pb2c_rv = 0.0;

        let assign3120_e2164: f64 = (2.0 * 1.034943e-10);
        let assign3120_e2166: f64 = (assign3120_e2164 / 1.6021918e-19);
        let assign3120_e2168: f64 = (assign3120_e2166 / locals.var_nsub);
        let assign3120_e2169: f64 = (assign3120_e2168).sqrt();
        locals.var_wdpl = assign3120_e2169;
        locals.var_wdpl_dn0 = ((-((assign3120_e2166 * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3120_e2169));
        locals.var_wdpl_dn2 = ((-((assign3120_e2166 * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3120_e2169));
        locals.var_wdpl_dn6 = ((-((assign3120_e2166 * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3120_e2169));
        locals.var_wdpl_dn7 = ((-((assign3120_e2166 * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3120_e2169));
        locals.var_wdpl_dn10 = ((-((assign3120_e2166 * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3120_e2169));
        locals.var_wdpl_dn11 = ((-((assign3120_e2166 * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3120_e2169));
        locals.var_wdpl_dn12 = ((-((assign3120_e2166 * locals.var_nsub_dn12) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3120_e2169));
        locals.var_wdpl_dn17 = ((-((assign3120_e2166 * locals.var_nsub_dn17) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign3120_e2169));
        locals.var_wdpl_rv = 0.0;

        let assign3130_e2174: f64 = (locals.var_lgle).powf(p.p195);
        let assign3130_e2175: f64 = (p.p194 / assign3130_e2174);
        let assign3130_e2176: f64 = (1.0 + assign3130_e2175);
        let assign3130_e2181: f64 = (locals.var_wl).powf(p.p197);
        let assign3130_e2182: f64 = (p.p196 / assign3130_e2181);
        let assign3130_e2183: f64 = (1.0 + assign3130_e2182);
        let assign3130_e2184: f64 = (assign3130_e2176 * assign3130_e2183);
        locals.var_t1 = assign3130_e2184;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn17 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign3140_e2187: f64 = (locals.var_t1 * locals.var_t1);
        let assign3140_e2190: f64 = (4.0 * 0.001);
        let assign3140_e2192: f64 = (assign3140_e2190 * 0.001);
        let assign3140_e2193: f64 = (assign3140_e2187 + assign3140_e2192);
        let assign3140_e2194: f64 = (assign3140_e2193).sqrt();
        locals.var_tmf1 = assign3140_e2194;
        locals.var_tmf1_dn0 = (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign3140_e2194));
        locals.var_tmf1_dn2 = (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign3140_e2194));
        locals.var_tmf1_dn6 = (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign3140_e2194));
        locals.var_tmf1_dn7 = (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign3140_e2194));
        locals.var_tmf1_dn10 = (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign3140_e2194));
        locals.var_tmf1_dn11 = (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign3140_e2194));
        locals.var_tmf1_dn12 = (((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) / (2.0 * assign3140_e2194));
        locals.var_tmf1_dn17 = (((locals.var_t1_dn17 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn17)) / (2.0 * assign3140_e2194));
        locals.var_tmf1_rv = 0.0;

        let assign3150_e2198: f64 = (locals.var_t1 + locals.var_tmf1);
        let assign3150_e2199: f64 = (0.5 * assign3150_e2198);
        let assign3150_e2202: f64 = (1e-10 * 0.001);
        let assign3150_e2203: f64 = (assign3150_e2199 + assign3150_e2202);
        locals.var_vmax0 = assign3150_e2203;
        locals.var_vmax0_dn0 = (0.5 * (locals.var_t1_dn0 + locals.var_tmf1_dn0));
        locals.var_vmax0_dn2 = (0.5 * (locals.var_t1_dn2 + locals.var_tmf1_dn2));
        locals.var_vmax0_dn6 = (0.5 * (locals.var_t1_dn6 + locals.var_tmf1_dn6));
        locals.var_vmax0_dn7 = (0.5 * (locals.var_t1_dn7 + locals.var_tmf1_dn7));
        locals.var_vmax0_dn10 = (0.5 * (locals.var_t1_dn10 + locals.var_tmf1_dn10));
        locals.var_vmax0_dn11 = (0.5 * (locals.var_t1_dn11 + locals.var_tmf1_dn11));
        locals.var_vmax0_dn12 = (0.5 * (locals.var_t1_dn12 + locals.var_tmf1_dn12));
        locals.var_vmax0_dn17 = (0.5 * (locals.var_t1_dn17 + locals.var_tmf1_dn17));
        locals.var_vmax0_rv = 0.0;

        let assign3160_e2206: f64 = if locals.var_vmax0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard18 = assign3160_e2206;
        locals.var_guard18_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (assign3170_e2210, assign3170_e2210_d_n0, assign3170_e2210_d_n2, assign3170_e2210_d_n6, assign3170_e2210_d_n7, assign3170_e2210_d_n10, assign3170_e2210_d_n11, assign3170_e2210_d_n12, assign3170_e2210_d_n17,) = {
    if (locals.var_guard18 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vmax0, locals.var_vmax0_dn0, locals.var_vmax0_dn2, locals.var_vmax0_dn6, locals.var_vmax0_dn7, locals.var_vmax0_dn10, locals.var_vmax0_dn11, locals.var_vmax0_dn12, locals.var_vmax0_dn17,)
    }
};
        locals.var_vmax0 = assign3170_e2210;
        locals.var_vmax0_dn0 = assign3170_e2210_d_n0;
        locals.var_vmax0_dn2 = assign3170_e2210_d_n2;
        locals.var_vmax0_dn6 = assign3170_e2210_d_n6;
        locals.var_vmax0_dn7 = assign3170_e2210_d_n7;
        locals.var_vmax0_dn10 = assign3170_e2210_d_n10;
        locals.var_vmax0_dn11 = assign3170_e2210_d_n11;
        locals.var_vmax0_dn12 = assign3170_e2210_d_n12;
        locals.var_vmax0_dn17 = assign3170_e2210_d_n17;
        locals.var_vmax0_rv = 0.0;

        let assign3230_e2243: f64 = if p.p261 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign3230_e2243;
        locals.var_guard21_rv = 0.0;

        let (assign3240_e2251, assign3240_e2251_d_n0, assign3240_e2251_d_n2, assign3240_e2251_d_n6, assign3240_e2251_d_n7, assign3240_e2251_d_n10, assign3240_e2251_d_n11, assign3240_e2251_d_n12, assign3240_e2251_d_n17,) = {
    if (locals.var_guard21 != 0.0) {
        let assign3240_e2247: f64 = (p.p289 * locals.var_weff_nf);
        let assign3240_e2249: f64 = (assign3240_e2247 + p.p288);
        (assign3240_e2249, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn17,)
    }
};
        locals.var_t0 = assign3240_e2251;
        locals.var_t0_dn0 = assign3240_e2251_d_n0;
        locals.var_t0_dn2 = assign3240_e2251_d_n2;
        locals.var_t0_dn6 = assign3240_e2251_d_n6;
        locals.var_t0_dn7 = assign3240_e2251_d_n7;
        locals.var_t0_dn10 = assign3240_e2251_d_n10;
        locals.var_t0_dn11 = assign3240_e2251_d_n11;
        locals.var_t0_dn12 = assign3240_e2251_d_n12;
        locals.var_t0_dn17 = assign3240_e2251_d_n17;
        locals.var_t0_rv = 0.0;

        let assign3380_e2327: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign3380_e2327;
        locals.var_guard26_rv = 0.0;

        let (assign3390_e2340,) = {
    if ((locals.var_guard26 != 0.0) && (p.p24 != 0.0)) {
        let (assign3390_e2338,) = {
            if (locals.var_abtp_given != 0.0) {
                (p.p23,)
            } else {
                let assign3390_e2335: f64 = (p.p20 * p.p9);
                let assign3390_e2337: f64 = (assign3390_e2335 * p.p19);
                (assign3390_e2337,)
            }
        };
        (assign3390_e2338,)
    } else {
        (locals.var_area_bt_p,)
    }
};
        locals.var_area_bt_p = assign3390_e2340;
        locals.var_area_bt_p_rv = 0.0;

        let (assign3400_e2353,) = {
    if ((locals.var_guard26 != 0.0) && (p.p24 != 0.0)) {
        let (assign3400_e2351,) = {
            if (locals.var_abtn_given != 0.0) {
                (p.p22,)
            } else {
                let assign3400_e2348: f64 = (p.p21 * p.p9);
                let assign3400_e2350: f64 = (assign3400_e2348 * p.p19);
                (assign3400_e2350,)
            }
        };
        (assign3400_e2351,)
    } else {
        (locals.var_area_bt_n,)
    }
};
        locals.var_area_bt_n = assign3400_e2353;
        locals.var_area_bt_n_rv = 0.0;

        let (assign3410_e2359,) = {
    if ((locals.var_guard26 != 0.0) && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_cbtp,)
    }
};
        locals.var_cbtp = assign3410_e2359;
        locals.var_cbtp_rv = 0.0;

        let (assign3420_e2365,) = {
    if ((locals.var_guard26 != 0.0) && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_cbtn,)
    }
};
        locals.var_cbtn = assign3420_e2365;
        locals.var_cbtn_rv = 0.0;

        let assign3430_e2370: f64 = if ((locals.var_area_bt_p > 0.0) && (locals.var_cbtbp_given != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard27 = assign3430_e2370;
        locals.var_guard27_rv = 0.0;

        let (assign3440_e2381,) = {
    if (((locals.var_guard26 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard27 != 0.0)) {
        let assign3440_e2377: f64 = (-locals.var_area_bt_p);
        let assign3440_e2379: f64 = (assign3440_e2377 * p.p294);
        (assign3440_e2379,)
    } else {
        (locals.var_cbtp,)
    }
};
        locals.var_cbtp = assign3440_e2381;
        locals.var_cbtp_rv = 0.0;

        let (assign3450_e2390,) = {
    if (((locals.var_guard26 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard27 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_cbtp,)
    }
};
        locals.var_cbtp = assign3450_e2390;
        locals.var_cbtp_rv = 0.0;

        let assign3460_e2395: f64 = if ((locals.var_area_bt_n > 0.0) && (locals.var_cbtbn_given != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard28 = assign3460_e2395;
        locals.var_guard28_rv = 0.0;

        let (assign3470_e2406,) = {
    if (((locals.var_guard26 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard28 != 0.0)) {
        let assign3470_e2402: f64 = (-locals.var_area_bt_n);
        let assign3470_e2404: f64 = (assign3470_e2402 * p.p293);
        (assign3470_e2404,)
    } else {
        (locals.var_cbtn,)
    }
};
        locals.var_cbtn = assign3470_e2406;
        locals.var_cbtn_rv = 0.0;

        let (assign3480_e2414,) = {
    if (((locals.var_guard26 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard28 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_area_bt_n,)
    }
};
        locals.var_area_bt_n = assign3480_e2414;
        locals.var_area_bt_n_rv = 0.0;

        let (assign3490_e2421,) = {
    if ((locals.var_guard26 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_area_bt_n,)
    }
};
        locals.var_area_bt_n = assign3490_e2421;
        locals.var_area_bt_n_rv = 0.0;

        let (assign3500_e2428,) = {
    if ((locals.var_guard26 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_cbtn,)
    }
};
        locals.var_cbtn = assign3500_e2428;
        locals.var_cbtn_rv = 0.0;

        let (assign3510_e2435,) = {
    if ((locals.var_guard26 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_area_bt_p,)
    }
};
        locals.var_area_bt_p = assign3510_e2435;
        locals.var_area_bt_p_rv = 0.0;

        let (assign3520_e2442,) = {
    if ((locals.var_guard26 != 0.0) && (p.p24 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_cbtp,)
    }
};
        locals.var_cbtp = assign3520_e2442;
        locals.var_cbtp_rv = 0.0;

        let (assign3530_e2455,) = {
    if (locals.var_guard26 != 0.0) {
        let (assign3530_e2453,) = {
            if (p.p19 > locals.var_lgate) {
                let assign3530_e2450: f64 = (p.p19 - locals.var_lgate);
                let assign3530_e2451: f64 = (0.5 * assign3530_e2450);
                (assign3530_e2451,)
            } else {
                (0.0,)
            }
        };
        (assign3530_e2453,)
    } else {
        (locals.var_peri_hhi,)
    }
};
        locals.var_peri_hhi = assign3530_e2455;
        locals.var_peri_hhi_rv = 0.0;

        let assign3540_e2458: f64 = if locals.var_pdbcp_given == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign3540_e2458;
        locals.var_guard29_rv = 0.0;

        let (assign3550_e2464,) = {
    if ((locals.var_guard26 != 0.0) && (locals.var_guard29 != 0.0)) {
        (locals.var_peri_hhi,)
    } else {
        (locals.var_uc_pdbcp,)
    }
};
        locals.var_uc_pdbcp = assign3550_e2464;
        locals.var_uc_pdbcp_rv = 0.0;

        let assign3560_e2467: f64 = if locals.var_psbcp_given == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign3560_e2467;
        locals.var_guard30_rv = 0.0;

        let (assign3570_e2473,) = {
    if ((locals.var_guard26 != 0.0) && (locals.var_guard30 != 0.0)) {
        (locals.var_peri_hhi,)
    } else {
        (locals.var_uc_psbcp,)
    }
};
        locals.var_uc_psbcp = assign3570_e2473;
        locals.var_uc_psbcp_rv = 0.0;

        let (assign3580_e2481,) = {
    if (locals.var_guard26 != 0.0) {
        let assign3580_e2478: f64 = (p.p9 * locals.var_uc_pdbcp);
        let assign3580_e2479: f64 = (locals.var_weff_nf + assign3580_e2478);
        (assign3580_e2479,)
    } else {
        (locals.var_w_diod,)
    }
};
        locals.var_w_diod = assign3580_e2481;
        locals.var_w_diod_rv = 0.0;

        let (assign3590_e2489,) = {
    if (locals.var_guard26 != 0.0) {
        let assign3590_e2486: f64 = (p.p9 * locals.var_uc_psbcp);
        let assign3590_e2487: f64 = (locals.var_weff_nf + assign3590_e2486);
        (assign3590_e2487,)
    } else {
        (locals.var_w_dios,)
    }
};
        locals.var_w_dios = assign3590_e2489;
        locals.var_w_dios_rv = 0.0;

        let (assign3600_e2497,) = {
    if (locals.var_guard26 != 0.0) {
        let assign3600_e2494: f64 = (p.p9 * locals.var_uc_pdbcp);
        let assign3600_e2495: f64 = (locals.var_weffcv_nf + assign3600_e2494);
        (assign3600_e2495,)
    } else {
        (locals.var_w_diodcv,)
    }
};
        locals.var_w_diodcv = assign3600_e2497;
        locals.var_w_diodcv_rv = 0.0;

        let (assign3610_e2505,) = {
    if (locals.var_guard26 != 0.0) {
        let assign3610_e2502: f64 = (p.p9 * locals.var_uc_psbcp);
        let assign3610_e2503: f64 = (locals.var_weffcv_nf + assign3610_e2502);
        (assign3610_e2503,)
    } else {
        (locals.var_w_dioscv,)
    }
};
        locals.var_w_dioscv = assign3610_e2505;
        locals.var_w_dioscv_rv = 0.0;

        let (assign3620_e2510,) = {
    if (locals.var_guard26 == 0.0) {
        (0.0,)
    } else {
        (locals.var_area_bt_n,)
    }
};
        locals.var_area_bt_n = assign3620_e2510;
        locals.var_area_bt_n_rv = 0.0;

        let (assign3630_e2515,) = {
    if (locals.var_guard26 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cbtn,)
    }
};
        locals.var_cbtn = assign3630_e2515;
        locals.var_cbtn_rv = 0.0;

        let (assign3640_e2520,) = {
    if (locals.var_guard26 == 0.0) {
        (0.0,)
    } else {
        (locals.var_area_bt_p,)
    }
};
        locals.var_area_bt_p = assign3640_e2520;
        locals.var_area_bt_p_rv = 0.0;

        let (assign3650_e2525,) = {
    if (locals.var_guard26 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cbtp,)
    }
};
        locals.var_cbtp = assign3650_e2525;
        locals.var_cbtp_rv = 0.0;

        let (assign3660_e2530,) = {
    if (locals.var_guard26 == 0.0) {
        (0.0,)
    } else {
        (locals.var_w_diod,)
    }
};
        locals.var_w_diod = assign3660_e2530;
        locals.var_w_diod_rv = 0.0;

        let (assign3670_e2535,) = {
    if (locals.var_guard26 == 0.0) {
        (0.0,)
    } else {
        (locals.var_w_dios,)
    }
};
        locals.var_w_dios = assign3670_e2535;
        locals.var_w_dios_rv = 0.0;

        let (assign3680_e2540,) = {
    if (locals.var_guard26 == 0.0) {
        (0.0,)
    } else {
        (locals.var_w_diodcv,)
    }
};
        locals.var_w_diodcv = assign3680_e2540;
        locals.var_w_diodcv_rv = 0.0;

        let (assign3690_e2545,) = {
    if (locals.var_guard26 == 0.0) {
        (0.0,)
    } else {
        (locals.var_w_dioscv,)
    }
};
        locals.var_w_dioscv = assign3690_e2545;
        locals.var_w_dioscv_rv = 0.0;

        let assign3700_e2548: f64 = (p.p50 * (nv6 - nv7));
        locals.var_vdsi = assign3700_e2548;
        locals.var_vdsi_dn6 = p.p50;
        locals.var_vdsi_dn7 = (-p.p50);
        locals.var_vdsi_rv = 0.0;

        let assign3710_e2551: f64 = (p.p50 * (nv11 - nv7));
        locals.var_vgsi = assign3710_e2551;
        locals.var_vgsi_dn7 = (-p.p50);
        locals.var_vgsi_dn11 = p.p50;
        locals.var_vgsi_rv = 0.0;

        let assign3720_e2554: f64 = (p.p50 * (nv12 - nv7));
        locals.var_vbsi = assign3720_e2554;
        locals.var_vbsi_dn7 = (-p.p50);
        locals.var_vbsi_dn12 = p.p50;
        locals.var_vbsi_rv = 0.0;

        let assign3760_e2566: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign3760_e2566;
        locals.var_guard31_rv = 0.0;

        let (assign3770_e2572, assign3770_e2572_d_n6, assign3770_e2572_d_n12,) = {
    if (locals.var_guard31 != 0.0) {
        let assign3770_e2570: f64 = (p.p50 * (nv12 - nv6));
        (assign3770_e2570, (-p.p50), p.p50,)
    } else {
        (locals.var_vbcd, locals.var_vbcd_dn6, locals.var_vbcd_dn12,)
    }
};
        locals.var_vbcd = assign3770_e2572;
        locals.var_vbcd_dn6 = assign3770_e2572_d_n6;
        locals.var_vbcd_dn12 = assign3770_e2572_d_n12;
        locals.var_vbcd_rv = 0.0;

        let (assign3780_e2578, assign3780_e2578_d_n7, assign3780_e2578_d_n12,) = {
    if (locals.var_guard31 != 0.0) {
        let assign3780_e2576: f64 = (p.p50 * (nv12 - nv7));
        (assign3780_e2576, (-p.p50), p.p50,)
    } else {
        (locals.var_vbcs, locals.var_vbcs_dn7, locals.var_vbcs_dn12,)
    }
};
        locals.var_vbcs = assign3780_e2578;
        locals.var_vbcs_dn7 = assign3780_e2578_d_n7;
        locals.var_vbcs_dn12 = assign3780_e2578_d_n12;
        locals.var_vbcs_rv = 0.0;

        let (assign3790_e2588, assign3790_e2588_d_n18,) = {
    if ((locals.var_guard31 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign3790_e2584: f64 = (1e-9 / 0.0001);
        let assign3790_e2586: f64 = (assign3790_e2584 * (nv18 - 0.0));
        (assign3790_e2586, assign3790_e2584,)
    } else {
        (locals.var_qi_nqs, locals.var_qi_nqs_dn18,)
    }
};
        locals.var_qi_nqs = assign3790_e2588;
        locals.var_qi_nqs_dn18 = assign3790_e2588_d_n18;
        locals.var_qi_nqs_rv = 0.0;

        let (assign3800_e2598, assign3800_e2598_d_n13,) = {
    if ((locals.var_guard31 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign3800_e2594: f64 = (1e-9 / 0.0001);
        let assign3800_e2596: f64 = (assign3800_e2594 * (nv13 - 0.0));
        (assign3800_e2596, assign3800_e2594,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign3800_e2598;
        locals.var_qb_nqs_dn13 = assign3800_e2598_d_n13;
        locals.var_qb_nqs_rv = 0.0;

        let (assign3810_e2605, assign3810_e2605_d_n18,) = {
    if ((locals.var_guard31 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_qi_nqs, locals.var_qi_nqs_dn18,)
    }
};
        locals.var_qi_nqs = assign3810_e2605;
        locals.var_qi_nqs_dn18 = assign3810_e2605_d_n18;
        locals.var_qi_nqs_rv = 0.0;

        let (assign3820_e2612, assign3820_e2612_d_n13,) = {
    if ((locals.var_guard31 != 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign3820_e2612;
        locals.var_qb_nqs_dn13 = assign3820_e2612_d_n13;
        locals.var_qb_nqs_rv = 0.0;

        let (assign3830_e2617, assign3830_e2617_d_n6, assign3830_e2617_d_n12,) = {
    if (locals.var_guard31 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbcd, locals.var_vbcd_dn6, locals.var_vbcd_dn12,)
    }
};
        locals.var_vbcd = assign3830_e2617;
        locals.var_vbcd_dn6 = assign3830_e2617_d_n6;
        locals.var_vbcd_dn12 = assign3830_e2617_d_n12;
        locals.var_vbcd_rv = 0.0;

        let (assign3840_e2622, assign3840_e2622_d_n7, assign3840_e2622_d_n12,) = {
    if (locals.var_guard31 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbcs, locals.var_vbcs_dn7, locals.var_vbcs_dn12,)
    }
};
        locals.var_vbcs = assign3840_e2622;
        locals.var_vbcs_dn7 = assign3840_e2622_d_n7;
        locals.var_vbcs_dn12 = assign3840_e2622_d_n12;
        locals.var_vbcs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (assign3850_e2633, assign3850_e2633_d_n0, assign3850_e2633_d_n2, assign3850_e2633_d_n6, assign3850_e2633_d_n7, assign3850_e2633_d_n10, assign3850_e2633_d_n11, assign3850_e2633_d_n12, assign3850_e2633_d_n15, assign3850_e2633_d_n17, assign3850_e2633_d_n18,) = {
    if ((locals.var_guard31 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign3850_e2629: f64 = (1e-9 / 0.0001);
        let assign3850_e2631: f64 = (assign3850_e2629 * (nv15 - 0.0));
        (assign3850_e2631, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign3850_e2629, 0.0, 0.0,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18,)
    }
};
        locals.var_qd_nqs = assign3850_e2633;
        locals.var_qd_nqs_dn0 = assign3850_e2633_d_n0;
        locals.var_qd_nqs_dn2 = assign3850_e2633_d_n2;
        locals.var_qd_nqs_dn6 = assign3850_e2633_d_n6;
        locals.var_qd_nqs_dn7 = assign3850_e2633_d_n7;
        locals.var_qd_nqs_dn10 = assign3850_e2633_d_n10;
        locals.var_qd_nqs_dn11 = assign3850_e2633_d_n11;
        locals.var_qd_nqs_dn12 = assign3850_e2633_d_n12;
        locals.var_qd_nqs_dn15 = assign3850_e2633_d_n15;
        locals.var_qd_nqs_dn17 = assign3850_e2633_d_n17;
        locals.var_qd_nqs_dn18 = assign3850_e2633_d_n18;
        locals.var_qd_nqs_rv = 0.0;

        let (assign3860_e2644, assign3860_e2644_d_n0, assign3860_e2644_d_n2, assign3860_e2644_d_n6, assign3860_e2644_d_n7, assign3860_e2644_d_n10, assign3860_e2644_d_n11, assign3860_e2644_d_n12, assign3860_e2644_d_n16, assign3860_e2644_d_n17, assign3860_e2644_d_n18,) = {
    if ((locals.var_guard31 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign3860_e2640: f64 = (1e-9 / 0.0001);
        let assign3860_e2642: f64 = (assign3860_e2640 * (nv16 - 0.0));
        (assign3860_e2642, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, assign3860_e2640, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18,)
    }
};
        locals.var_qs_nqs = assign3860_e2644;
        locals.var_qs_nqs_dn0 = assign3860_e2644_d_n0;
        locals.var_qs_nqs_dn2 = assign3860_e2644_d_n2;
        locals.var_qs_nqs_dn6 = assign3860_e2644_d_n6;
        locals.var_qs_nqs_dn7 = assign3860_e2644_d_n7;
        locals.var_qs_nqs_dn10 = assign3860_e2644_d_n10;
        locals.var_qs_nqs_dn11 = assign3860_e2644_d_n11;
        locals.var_qs_nqs_dn12 = assign3860_e2644_d_n12;
        locals.var_qs_nqs_dn16 = assign3860_e2644_d_n16;
        locals.var_qs_nqs_dn17 = assign3860_e2644_d_n17;
        locals.var_qs_nqs_dn18 = assign3860_e2644_d_n18;
        locals.var_qs_nqs_rv = 0.0;

        let (assign3870_e2655, assign3870_e2655_d_n13,) = {
    if ((locals.var_guard31 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign3870_e2651: f64 = (1e-9 / 0.0001);
        let assign3870_e2653: f64 = (assign3870_e2651 * (nv13 - 0.0));
        (assign3870_e2653, assign3870_e2651,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign3870_e2655;
        locals.var_qb_nqs_dn13 = assign3870_e2655_d_n13;
        locals.var_qb_nqs_rv = 0.0;

        let (assign3880_e2663, assign3880_e2663_d_n0, assign3880_e2663_d_n2, assign3880_e2663_d_n6, assign3880_e2663_d_n7, assign3880_e2663_d_n10, assign3880_e2663_d_n11, assign3880_e2663_d_n12, assign3880_e2663_d_n15, assign3880_e2663_d_n17, assign3880_e2663_d_n18,) = {
    if ((locals.var_guard31 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn15, locals.var_qd_nqs_dn17, locals.var_qd_nqs_dn18,)
    }
};
        locals.var_qd_nqs = assign3880_e2663;
        locals.var_qd_nqs_dn0 = assign3880_e2663_d_n0;
        locals.var_qd_nqs_dn2 = assign3880_e2663_d_n2;
        locals.var_qd_nqs_dn6 = assign3880_e2663_d_n6;
        locals.var_qd_nqs_dn7 = assign3880_e2663_d_n7;
        locals.var_qd_nqs_dn10 = assign3880_e2663_d_n10;
        locals.var_qd_nqs_dn11 = assign3880_e2663_d_n11;
        locals.var_qd_nqs_dn12 = assign3880_e2663_d_n12;
        locals.var_qd_nqs_dn15 = assign3880_e2663_d_n15;
        locals.var_qd_nqs_dn17 = assign3880_e2663_d_n17;
        locals.var_qd_nqs_dn18 = assign3880_e2663_d_n18;
        locals.var_qd_nqs_rv = 0.0;

        let (assign3890_e2671, assign3890_e2671_d_n0, assign3890_e2671_d_n2, assign3890_e2671_d_n6, assign3890_e2671_d_n7, assign3890_e2671_d_n10, assign3890_e2671_d_n11, assign3890_e2671_d_n12, assign3890_e2671_d_n16, assign3890_e2671_d_n17, assign3890_e2671_d_n18,) = {
    if ((locals.var_guard31 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18,)
    }
};
        locals.var_qs_nqs = assign3890_e2671;
        locals.var_qs_nqs_dn0 = assign3890_e2671_d_n0;
        locals.var_qs_nqs_dn2 = assign3890_e2671_d_n2;
        locals.var_qs_nqs_dn6 = assign3890_e2671_d_n6;
        locals.var_qs_nqs_dn7 = assign3890_e2671_d_n7;
        locals.var_qs_nqs_dn10 = assign3890_e2671_d_n10;
        locals.var_qs_nqs_dn11 = assign3890_e2671_d_n11;
        locals.var_qs_nqs_dn12 = assign3890_e2671_d_n12;
        locals.var_qs_nqs_dn16 = assign3890_e2671_d_n16;
        locals.var_qs_nqs_dn17 = assign3890_e2671_d_n17;
        locals.var_qs_nqs_dn18 = assign3890_e2671_d_n18;
        locals.var_qs_nqs_rv = 0.0;

        let (assign3900_e2679, assign3900_e2679_d_n13,) = {
    if ((locals.var_guard31 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign3900_e2679;
        locals.var_qb_nqs_dn13 = assign3900_e2679_d_n13;
        locals.var_qb_nqs_rv = 0.0;

        let (assign3910_e2694, assign3910_e2694_d_n10,) = {
    if ((p.p38 > 0.0) && (locals.var_mks_rth0 > 0.0)) {
        let (assign3910_e2692, assign3910_e2692_d_n10,) = {
            if ((nv10 - 0.0) > 0.0) {
                ((nv10 - 0.0), 1.0,)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign3910_e2692, assign3910_e2692_d_n10,)
    } else {
        (0.0, 0.0,)
    }
};
        locals.var_deltemp = assign3910_e2694;
        locals.var_deltemp_dn10 = assign3910_e2694_d_n10;
        locals.var_deltemp_rv = 0.0;

        let assign3920_e2697: f64 = if locals.var_vdsi >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign3920_e2697;
        locals.var_guard32_rv = 0.0;

        let (assign3930_e2701,) = {
    if (locals.var_guard32 != 0.0) {
        (1.0,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign3930_e2701;
        locals.var_mode_rv = 0.0;

        let (assign3940_e2705,) = {
    if (locals.var_guard32 != 0.0) {
        (1.0,)
    } else {
        (locals.var_modenml,)
    }
};
        locals.var_modenml = assign3940_e2705;
        locals.var_modenml_rv = 0.0;

        let (assign3950_e2709,) = {
    if (locals.var_guard32 != 0.0) {
        (0.0,)
    } else {
        (locals.var_modervs,)
    }
};
        locals.var_modervs = assign3950_e2709;
        locals.var_modervs_rv = 0.0;

        let (assign3960_e2713, assign3960_e2713_d_n0, assign3960_e2713_d_n2, assign3960_e2713_d_n6, assign3960_e2713_d_n7, assign3960_e2713_d_n10, assign3960_e2713_d_n11, assign3960_e2713_d_n12, assign3960_e2713_d_n17,) = {
    if (locals.var_guard32 != 0.0) {
        (locals.var_vdsi, 0.0, 0.0, locals.var_vdsi_dn6, locals.var_vdsi_dn7, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign3960_e2713;
        locals.var_vds_dn0 = assign3960_e2713_d_n0;
        locals.var_vds_dn2 = assign3960_e2713_d_n2;
        locals.var_vds_dn6 = assign3960_e2713_d_n6;
        locals.var_vds_dn7 = assign3960_e2713_d_n7;
        locals.var_vds_dn10 = assign3960_e2713_d_n10;
        locals.var_vds_dn11 = assign3960_e2713_d_n11;
        locals.var_vds_dn12 = assign3960_e2713_d_n12;
        locals.var_vds_dn17 = assign3960_e2713_d_n17;
        locals.var_vds_rv = 0.0;

        let (assign3970_e2717, assign3970_e2717_d_n6, assign3970_e2717_d_n7, assign3970_e2717_d_n11,) = {
    if (locals.var_guard32 != 0.0) {
        (locals.var_vgsi, 0.0, locals.var_vgsi_dn7, locals.var_vgsi_dn11,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn11,)
    }
};
        locals.var_vgs = assign3970_e2717;
        locals.var_vgs_dn6 = assign3970_e2717_d_n6;
        locals.var_vgs_dn7 = assign3970_e2717_d_n7;
        locals.var_vgs_dn11 = assign3970_e2717_d_n11;
        locals.var_vgs_rv = 0.0;

        let (assign3980_e2721, assign3980_e2721_d_n0, assign3980_e2721_d_n2, assign3980_e2721_d_n6, assign3980_e2721_d_n7, assign3980_e2721_d_n10, assign3980_e2721_d_n11, assign3980_e2721_d_n12, assign3980_e2721_d_n17,) = {
    if (locals.var_guard32 != 0.0) {
        (locals.var_vbsi, 0.0, 0.0, 0.0, locals.var_vbsi_dn7, 0.0, 0.0, locals.var_vbsi_dn12, 0.0,)
    } else {
        (locals.var_vbs, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
    }
};
        locals.var_vbs = assign3980_e2721;
        locals.var_vbs_dn0 = assign3980_e2721_d_n0;
        locals.var_vbs_dn2 = assign3980_e2721_d_n2;
        locals.var_vbs_dn6 = assign3980_e2721_d_n6;
        locals.var_vbs_dn7 = assign3980_e2721_d_n7;
        locals.var_vbs_dn10 = assign3980_e2721_d_n10;
        locals.var_vbs_dn11 = assign3980_e2721_d_n11;
        locals.var_vbs_dn12 = assign3980_e2721_d_n12;
        locals.var_vbs_dn17 = assign3980_e2721_d_n17;
        locals.var_vbs_rv = 0.0;

        let (assign4020_e2739,) = {
    if (locals.var_guard32 == 0.0) {
        let assign4020_e2737: f64 = (-1.0);
        (assign4020_e2737,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign4020_e2739;
        locals.var_mode_rv = 0.0;

        let (assign4030_e2744,) = {
    if (locals.var_guard32 == 0.0) {
        (0.0,)
    } else {
        (locals.var_modenml,)
    }
};
        locals.var_modenml = assign4030_e2744;
        locals.var_modenml_rv = 0.0;

        let (assign4040_e2749,) = {
    if (locals.var_guard32 == 0.0) {
        (1.0,)
    } else {
        (locals.var_modervs,)
    }
};
        locals.var_modervs = assign4040_e2749;
        locals.var_modervs_rv = 0.0;

        let (assign4050_e2755, assign4050_e2755_d_n0, assign4050_e2755_d_n2, assign4050_e2755_d_n6, assign4050_e2755_d_n7, assign4050_e2755_d_n10, assign4050_e2755_d_n11, assign4050_e2755_d_n12, assign4050_e2755_d_n17,) = {
    if (locals.var_guard32 == 0.0) {
        let assign4050_e2753: f64 = (-locals.var_vdsi);
        (assign4050_e2753, 0.0, 0.0, (-locals.var_vdsi_dn6), (-locals.var_vdsi_dn7), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vds = assign4050_e2755;
        locals.var_vds_dn0 = assign4050_e2755_d_n0;
        locals.var_vds_dn2 = assign4050_e2755_d_n2;
        locals.var_vds_dn6 = assign4050_e2755_d_n6;
        locals.var_vds_dn7 = assign4050_e2755_d_n7;
        locals.var_vds_dn10 = assign4050_e2755_d_n10;
        locals.var_vds_dn11 = assign4050_e2755_d_n11;
        locals.var_vds_dn12 = assign4050_e2755_d_n12;
        locals.var_vds_dn17 = assign4050_e2755_d_n17;
        locals.var_vds_rv = 0.0;

        let (assign4060_e2762, assign4060_e2762_d_n6, assign4060_e2762_d_n7, assign4060_e2762_d_n11,) = {
    if (locals.var_guard32 == 0.0) {
        let assign4060_e2760: f64 = (locals.var_vgsi - locals.var_vdsi);
        (assign4060_e2760, (-locals.var_vdsi_dn6), (locals.var_vgsi_dn7 - locals.var_vdsi_dn7), locals.var_vgsi_dn11,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn11,)
    }
};
        locals.var_vgs = assign4060_e2762;
        locals.var_vgs_dn6 = assign4060_e2762_d_n6;
        locals.var_vgs_dn7 = assign4060_e2762_d_n7;
        locals.var_vgs_dn11 = assign4060_e2762_d_n11;
        locals.var_vgs_rv = 0.0;

        let (assign4070_e2769, assign4070_e2769_d_n0, assign4070_e2769_d_n2, assign4070_e2769_d_n6, assign4070_e2769_d_n7, assign4070_e2769_d_n10, assign4070_e2769_d_n11, assign4070_e2769_d_n12, assign4070_e2769_d_n17,) = {
    if (locals.var_guard32 == 0.0) {
        let assign4070_e2767: f64 = (locals.var_vbsi - locals.var_vdsi);
        (assign4070_e2767, 0.0, 0.0, (-locals.var_vdsi_dn6), (locals.var_vbsi_dn7 - locals.var_vdsi_dn7), 0.0, 0.0, locals.var_vbsi_dn12, 0.0,)
    } else {
        (locals.var_vbs, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
    }
};
        locals.var_vbs = assign4070_e2769;
        locals.var_vbs_dn0 = assign4070_e2769_d_n0;
        locals.var_vbs_dn2 = assign4070_e2769_d_n2;
        locals.var_vbs_dn6 = assign4070_e2769_d_n6;
        locals.var_vbs_dn7 = assign4070_e2769_d_n7;
        locals.var_vbs_dn10 = assign4070_e2769_d_n10;
        locals.var_vbs_dn11 = assign4070_e2769_d_n11;
        locals.var_vbs_dn12 = assign4070_e2769_d_n12;
        locals.var_vbs_dn17 = assign4070_e2769_d_n17;
        locals.var_vbs_rv = 0.0;

        let assign4130_e2796: f64 = ctx_temp;
        locals.var_ttemp = assign4130_e2796;
        locals.var_ttemp_dn10 = 0.0;
        locals.var_ttemp_rv = 0.0;

        let (assign4140_e2800, assign4140_e2800_d_n10,) = {
    if (locals.var_temp_given != 0.0) {
        (locals.var_uc_temp, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn10,)
    }
};
        locals.var_ttemp = assign4140_e2800;
        locals.var_ttemp_dn10 = assign4140_e2800_d_n10;
        locals.var_ttemp_rv = 0.0;

        let (assign4150_e2806, assign4150_e2806_d_n10,) = {
    if (locals.var_dtemp_given != 0.0) {
        let assign4150_e2804: f64 = (locals.var_ttemp + p.p17);
        (assign4150_e2804, locals.var_ttemp_dn10,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn10,)
    }
};
        locals.var_ttemp = assign4150_e2806;
        locals.var_ttemp_dn10 = assign4150_e2806_d_n10;
        locals.var_ttemp_rv = 0.0;

        let assign4160_e2809: f64 = (locals.var_ttemp + locals.var_deltemp);
        locals.var_ttemp = assign4160_e2809;
        locals.var_ttemp_dn10 = (locals.var_ttemp_dn10 + locals.var_deltemp_dn10);
        locals.var_ttemp_rv = 0.0;

        let assign4170_e2812: f64 = (locals.var_ttemp - locals.var_uc_tnom);
        locals.var_t1 = assign4170_e2812;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn10 = locals.var_ttemp_dn10;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn17 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4180_e2816: f64 = (locals.var_ttemp + locals.var_uc_tnom);
        let assign4180_e2817: f64 = (locals.var_t1 * assign4180_e2816);
        locals.var_t2 = assign4180_e2817;
        locals.var_t2_dn0 = (locals.var_t1_dn0 * assign4180_e2816);
        locals.var_t2_dn2 = (locals.var_t1_dn2 * assign4180_e2816);
        locals.var_t2_dn6 = (locals.var_t1_dn6 * assign4180_e2816);
        locals.var_t2_dn7 = (locals.var_t1_dn7 * assign4180_e2816);
        locals.var_t2_dn10 = ((locals.var_t1_dn10 * assign4180_e2816) + (locals.var_t1 * locals.var_ttemp_dn10));
        locals.var_t2_dn11 = (locals.var_t1_dn11 * assign4180_e2816);
        locals.var_t2_dn12 = (locals.var_t1_dn12 * assign4180_e2816);
        locals.var_t2_dn17 = (locals.var_t1_dn17 * assign4180_e2816);
        locals.var_t2_rv = 0.0;

        let assign4190_e2821: f64 = (p.p53 * locals.var_t1);
        let assign4190_e2822: f64 = (locals.var_egtnom - assign4190_e2821);
        let assign4190_e2825: f64 = (p.p54 * locals.var_t2);
        let assign4190_e2826: f64 = (assign4190_e2822 - assign4190_e2825);
        locals.var_eg = assign4190_e2826;
        locals.var_eg_dn0 = ((-(p.p53 * locals.var_t1_dn0)) - (p.p54 * locals.var_t2_dn0));
        locals.var_eg_dn2 = ((-(p.p53 * locals.var_t1_dn2)) - (p.p54 * locals.var_t2_dn2));
        locals.var_eg_dn6 = ((-(p.p53 * locals.var_t1_dn6)) - (p.p54 * locals.var_t2_dn6));
        locals.var_eg_dn7 = ((-(p.p53 * locals.var_t1_dn7)) - (p.p54 * locals.var_t2_dn7));
        locals.var_eg_dn10 = ((-(p.p53 * locals.var_t1_dn10)) - (p.p54 * locals.var_t2_dn10));
        locals.var_eg_dn11 = ((-(p.p53 * locals.var_t1_dn11)) - (p.p54 * locals.var_t2_dn11));
        locals.var_eg_dn12 = ((-(p.p53 * locals.var_t1_dn12)) - (p.p54 * locals.var_t2_dn12));
        locals.var_eg_dn17 = ((-(p.p53 * locals.var_t1_dn17)) - (p.p54 * locals.var_t2_dn17));
        locals.var_eg_rv = 0.0;

        let assign4200_e2830: f64 = (1.3806226e-23 * locals.var_ttemp);
        let assign4200_e2831: f64 = (1.6021918e-19 / assign4200_e2830);
        locals.var_beta = assign4200_e2831;
        locals.var_beta_dn10 = (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn10)) / (assign4200_e2830 * assign4200_e2830)));
        locals.var_beta_rv = 0.0;

        let assign4210_e2834: f64 = (locals.var_beta * locals.var_beta);
        locals.var_beta2 = assign4210_e2834;
        locals.var_beta2_dn10 = ((locals.var_beta_dn10 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn10));
        locals.var_beta2_rv = 0.0;

        let assign4220_e2837: f64 = (1.0 / locals.var_beta);
        locals.var_beta_inv = assign4220_e2837;
        locals.var_beta_inv_dn10 = (-(locals.var_beta_dn10 / (locals.var_beta * locals.var_beta)));
        locals.var_beta_inv_rv = 0.0;

        let assign4230_e2843: f64 = (locals.var_wg).powf(p.p99);
        let assign4230_e2844: f64 = (p.p98 / assign4230_e2843);
        let assign4230_e2845: f64 = (1.0 + assign4230_e2844);
        let assign4230_e2846: f64 = (p.p254 * assign4230_e2845);
        let assign4230_e2851: f64 = (locals.var_lgle).powf(p.p101);
        let assign4230_e2852: f64 = (p.p100 / assign4230_e2851);
        let assign4230_e2853: f64 = (1.0 + assign4230_e2852);
        let assign4230_e2854: f64 = (assign4230_e2846 * assign4230_e2853);
        let assign4230_e2859: f64 = (locals.var_wl).powf(p.p103);
        let assign4230_e2860: f64 = (p.p102 / assign4230_e2859);
        let assign4230_e2861: f64 = (1.0 + assign4230_e2860);
        let assign4230_e2862: f64 = (assign4230_e2854 * assign4230_e2861);
        locals.var_cgs_mueph = assign4230_e2862;
        locals.var_cgs_mueph_rv = 0.0;

        let assign4240_e2866: f64 = (1.0 + p.p159);
        let assign4240_e2867: f64 = (1.0 / assign4240_e2866);
        locals.var_t2__blk38 = assign4240_e2867;
        locals.var_t2__blk38_rv = 0.0;

        locals.var_t3__blk39 = 0.0;
        locals.var_t3__blk39_rv = 0.0;

        let assign4260_e2873: f64 = (locals.var_t2__blk38 * locals.var_t3__blk39);
        let assign4260_e2874: f64 = (1.0 + assign4260_e2873);
        let assign4260_e2875: f64 = (locals.var_cgs_mueph * assign4260_e2874);
        locals.var_cgs_wmueph = assign4260_e2875;
        locals.var_cgs_wmueph_rv = 0.0;

        let assign4270_e2878: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign4270_e2880: f64 = (assign4270_e2878).powf(p.p112);
        locals.var_t1__blk37 = assign4270_e2880;
        locals.var_t1__blk37_dn10 = if 0.0 == 0.0 && ((p.p112) as f64).is_finite() && ((p.p112) as f64).fract() == 0.0 { if p.p112 == 0.0 { 0.0 } else { (p.p112 * ((assign4270_e2878).powf(p.p112 - 1.0) * (locals.var_ttemp_dn10 / locals.var_uc_tnom))) } } else { (assign4270_e2880 * (p.p112 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign4270_e2878))) };
        locals.var_t1__blk37_rv = 0.0;

        let assign4280_e2883: f64 = (locals.var_t1__blk37 / locals.var_cgs_wmueph);
        locals.var_cgs_mphn0 = assign4280_e2883;
        locals.var_cgs_mphn0_dn10 = (locals.var_t1__blk37_dn10 / locals.var_cgs_wmueph);
        locals.var_cgs_mphn0_rv = 0.0;

        let assign4290_e2886: f64 = (locals.var_ptovr0 * locals.var_beta_inv);
        locals.var_ptovr = assign4290_e2886;
        locals.var_ptovr_dn0 = (locals.var_ptovr0_dn0 * locals.var_beta_inv);
        locals.var_ptovr_dn2 = (locals.var_ptovr0_dn2 * locals.var_beta_inv);
        locals.var_ptovr_dn6 = (locals.var_ptovr0_dn6 * locals.var_beta_inv);
        locals.var_ptovr_dn7 = (locals.var_ptovr0_dn7 * locals.var_beta_inv);
        locals.var_ptovr_dn10 = ((locals.var_ptovr0_dn10 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn10));
        locals.var_ptovr_dn11 = (locals.var_ptovr0_dn11 * locals.var_beta_inv);
        locals.var_ptovr_dn12 = (locals.var_ptovr0_dn12 * locals.var_beta_inv);
        locals.var_ptovr_dn17 = (locals.var_ptovr0_dn17 * locals.var_beta_inv);
        locals.var_ptovr_rv = 0.0;

        let assign4300_e2889: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        locals.var_t1 = assign4300_e2889;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn10 = (locals.var_ttemp_dn10 / locals.var_uc_tnom);
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn17 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign4310_e2892: f64 = (locals.var_vmax0 * locals.var_mks_vmax);
        let assign4310_e2896: f64 = (0.4 * locals.var_t1);
        let assign4310_e2897: f64 = (1.8 + assign4310_e2896);
        let assign4310_e2900: f64 = (0.1 * locals.var_t1);
        let assign4310_e2902: f64 = (assign4310_e2900 * locals.var_t1);
        let assign4310_e2903: f64 = (assign4310_e2897 + assign4310_e2902);
        let assign4310_e2907: f64 = (1.0 - locals.var_t1);
        let assign4310_e2908: f64 = (locals.var_mks_vtmp * assign4310_e2907);
        let assign4310_e2909: f64 = (assign4310_e2903 - assign4310_e2908);
        let assign4310_e2910: f64 = (assign4310_e2892 / assign4310_e2909);
        locals.var_vmaxe = assign4310_e2910;
        locals.var_vmaxe_dn0 = ((((locals.var_vmax0_dn0 * locals.var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * locals.var_t1_dn0) + (((0.1 * locals.var_t1_dn0) * locals.var_t1) + (assign4310_e2900 * locals.var_t1_dn0))) - (locals.var_mks_vtmp * (-locals.var_t1_dn0))))) / (assign4310_e2909 * assign4310_e2909));
        locals.var_vmaxe_dn2 = ((((locals.var_vmax0_dn2 * locals.var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * locals.var_t1_dn2) + (((0.1 * locals.var_t1_dn2) * locals.var_t1) + (assign4310_e2900 * locals.var_t1_dn2))) - (locals.var_mks_vtmp * (-locals.var_t1_dn2))))) / (assign4310_e2909 * assign4310_e2909));
        locals.var_vmaxe_dn6 = ((((locals.var_vmax0_dn6 * locals.var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * locals.var_t1_dn6) + (((0.1 * locals.var_t1_dn6) * locals.var_t1) + (assign4310_e2900 * locals.var_t1_dn6))) - (locals.var_mks_vtmp * (-locals.var_t1_dn6))))) / (assign4310_e2909 * assign4310_e2909));
        locals.var_vmaxe_dn7 = ((((locals.var_vmax0_dn7 * locals.var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * locals.var_t1_dn7) + (((0.1 * locals.var_t1_dn7) * locals.var_t1) + (assign4310_e2900 * locals.var_t1_dn7))) - (locals.var_mks_vtmp * (-locals.var_t1_dn7))))) / (assign4310_e2909 * assign4310_e2909));
        locals.var_vmaxe_dn10 = ((((locals.var_vmax0_dn10 * locals.var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * locals.var_t1_dn10) + (((0.1 * locals.var_t1_dn10) * locals.var_t1) + (assign4310_e2900 * locals.var_t1_dn10))) - (locals.var_mks_vtmp * (-locals.var_t1_dn10))))) / (assign4310_e2909 * assign4310_e2909));
        locals.var_vmaxe_dn11 = ((((locals.var_vmax0_dn11 * locals.var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * locals.var_t1_dn11) + (((0.1 * locals.var_t1_dn11) * locals.var_t1) + (assign4310_e2900 * locals.var_t1_dn11))) - (locals.var_mks_vtmp * (-locals.var_t1_dn11))))) / (assign4310_e2909 * assign4310_e2909));
        locals.var_vmaxe_dn12 = ((((locals.var_vmax0_dn12 * locals.var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * locals.var_t1_dn12) + (((0.1 * locals.var_t1_dn12) * locals.var_t1) + (assign4310_e2900 * locals.var_t1_dn12))) - (locals.var_mks_vtmp * (-locals.var_t1_dn12))))) / (assign4310_e2909 * assign4310_e2909));
        locals.var_vmaxe_dn17 = ((((locals.var_vmax0_dn17 * locals.var_mks_vmax) * assign4310_e2909) - (assign4310_e2892 * (((0.4 * locals.var_t1_dn17) + (((0.1 * locals.var_t1_dn17) * locals.var_t1) + (assign4310_e2900 * locals.var_t1_dn17))) - (locals.var_mks_vtmp * (-locals.var_t1_dn17))))) / (assign4310_e2909 * assign4310_e2909));
        locals.var_vmaxe_rv = 0.0;

        let assign4320_e2912: f64 = (locals.var_eg).sqrt();
        locals.var_egp12 = assign4320_e2912;
        locals.var_egp12_dn0 = (locals.var_eg_dn0 / (2.0 * assign4320_e2912));
        locals.var_egp12_dn2 = (locals.var_eg_dn2 / (2.0 * assign4320_e2912));
        locals.var_egp12_dn6 = (locals.var_eg_dn6 / (2.0 * assign4320_e2912));
        locals.var_egp12_dn7 = (locals.var_eg_dn7 / (2.0 * assign4320_e2912));
        locals.var_egp12_dn10 = (locals.var_eg_dn10 / (2.0 * assign4320_e2912));
        locals.var_egp12_dn11 = (locals.var_eg_dn11 / (2.0 * assign4320_e2912));
        locals.var_egp12_dn12 = (locals.var_eg_dn12 / (2.0 * assign4320_e2912));
        locals.var_egp12_dn17 = (locals.var_eg_dn17 / (2.0 * assign4320_e2912));
        locals.var_egp12_rv = 0.0;

        let assign4330_e2915: f64 = (locals.var_eg * locals.var_egp12);
        locals.var_egp32 = assign4330_e2915;
        locals.var_egp32_dn0 = ((locals.var_eg_dn0 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn0));
        locals.var_egp32_dn2 = ((locals.var_eg_dn2 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn2));
        locals.var_egp32_dn6 = ((locals.var_eg_dn6 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn6));
        locals.var_egp32_dn7 = ((locals.var_eg_dn7 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn7));
        locals.var_egp32_dn10 = ((locals.var_eg_dn10 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn10));
        locals.var_egp32_dn11 = ((locals.var_eg_dn11 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn11));
        locals.var_egp32_dn12 = ((locals.var_eg_dn12 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn12));
        locals.var_egp32_dn17 = ((locals.var_eg_dn17 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn17));
        locals.var_egp32_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign4340_e2918: f64 = (10400000000.0 / 1e-6);
        let assign4340_e2921: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign4340_e2923: f64 = (assign4340_e2921).powf(1.5);
        let assign4340_e2924: f64 = (assign4340_e2918 * assign4340_e2923);
        let assign4340_e2926: f64 = (-locals.var_eg);
        let assign4340_e2928: f64 = (assign4340_e2926 / 2.0);
        let assign4340_e2930: f64 = (assign4340_e2928 * locals.var_beta);
        let assign4340_e2933: f64 = (locals.var_egtnom / 2.0);
        let assign4340_e2935: f64 = (assign4340_e2933 * locals.var_betatnom);
        let assign4340_e2936: f64 = (assign4340_e2930 + assign4340_e2935);
        let assign4340_e2937: f64 = (assign4340_e2936).exp();
        let assign4340_e2938: f64 = (assign4340_e2924 * assign4340_e2937);
        locals.var_nin = assign4340_e2938;
        locals.var_nin_dn0 = (assign4340_e2924 * (assign4340_e2937 * (((-locals.var_eg_dn0) / 2.0) * locals.var_beta)));
        locals.var_nin_dn2 = (assign4340_e2924 * (assign4340_e2937 * (((-locals.var_eg_dn2) / 2.0) * locals.var_beta)));
        locals.var_nin_dn6 = (assign4340_e2924 * (assign4340_e2937 * (((-locals.var_eg_dn6) / 2.0) * locals.var_beta)));
        locals.var_nin_dn7 = (assign4340_e2924 * (assign4340_e2937 * (((-locals.var_eg_dn7) / 2.0) * locals.var_beta)));
        locals.var_nin_dn10 = (((assign4340_e2918 * if 0.0 == 0.0 && ((1.5) as f64).is_finite() && ((1.5) as f64).fract() == 0.0 { if 1.5 == 0.0 { 0.0 } else { (1.5 * ((assign4340_e2921).powf(1.5 - 1.0) * (locals.var_ttemp_dn10 / locals.var_uc_tnom))) } } else { (assign4340_e2923 * (1.5 * ((locals.var_ttemp_dn10 / locals.var_uc_tnom) / assign4340_e2921))) }) * assign4340_e2937) + (assign4340_e2924 * (assign4340_e2937 * ((((-locals.var_eg_dn10) / 2.0) * locals.var_beta) + (assign4340_e2928 * locals.var_beta_dn10)))));
        locals.var_nin_dn11 = (assign4340_e2924 * (assign4340_e2937 * (((-locals.var_eg_dn11) / 2.0) * locals.var_beta)));
        locals.var_nin_dn12 = (assign4340_e2924 * (assign4340_e2937 * (((-locals.var_eg_dn12) / 2.0) * locals.var_beta)));
        locals.var_nin_dn17 = (assign4340_e2924 * (assign4340_e2937 * (((-locals.var_eg_dn17) / 2.0) * locals.var_beta)));
        locals.var_nin_rv = 0.0;

        let assign4350_e2941: f64 = (locals.var_beta_inv).sqrt();
        let assign4350_e2942: f64 = (locals.var_costi00 * assign4350_e2941);
        locals.var_costi0 = assign4350_e2942;
        locals.var_costi0_dn0 = 0.0;
        locals.var_costi0_dn2 = 0.0;
        locals.var_costi0_dn6 = 0.0;
        locals.var_costi0_dn7 = 0.0;
        locals.var_costi0_dn10 = (locals.var_costi00 * (locals.var_beta_inv_dn10 / (2.0 * assign4350_e2941)));
        locals.var_costi0_dn11 = 0.0;
        locals.var_costi0_dn12 = 0.0;
        locals.var_costi0_dn17 = 0.0;
        locals.var_costi0_rv = 0.0;

        let assign4360_e2945: f64 = (locals.var_costi0 * locals.var_costi0);
        locals.var_costi0_p2 = assign4360_e2945;
        locals.var_costi0_p2_dn0 = ((locals.var_costi0_dn0 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn0));
        locals.var_costi0_p2_dn2 = ((locals.var_costi0_dn2 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn2));
        locals.var_costi0_p2_dn6 = ((locals.var_costi0_dn6 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn6));
        locals.var_costi0_p2_dn7 = ((locals.var_costi0_dn7 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn7));
        locals.var_costi0_p2_dn10 = ((locals.var_costi0_dn10 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn10));
        locals.var_costi0_p2_dn11 = ((locals.var_costi0_dn11 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn11));
        locals.var_costi0_p2_dn12 = ((locals.var_costi0_dn12 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn12));
        locals.var_costi0_p2_dn17 = ((locals.var_costi0_dn17 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn17));
        locals.var_costi0_p2_rv = 0.0;

        let assign4370_e2948: f64 = (locals.var_nin * locals.var_nin);
        let assign4370_e2950: f64 = (assign4370_e2948 * locals.var_nsti_p2);
        locals.var_costi1 = assign4370_e2950;
        locals.var_costi1_dn0 = (((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_nsti_p2);
        locals.var_costi1_dn2 = (((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_nsti_p2);
        locals.var_costi1_dn6 = (((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_nsti_p2);
        locals.var_costi1_dn7 = (((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_nsti_p2);
        locals.var_costi1_dn10 = (((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_nsti_p2);
        locals.var_costi1_dn11 = (((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_nsti_p2);
        locals.var_costi1_dn12 = (((locals.var_nin_dn12 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn12)) * locals.var_nsti_p2);
        locals.var_costi1_dn17 = (((locals.var_nin_dn17 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn17)) * locals.var_nsti_p2);
        locals.var_costi1_rv = 0.0;

        let assign4380_e2954: f64 = (2.0 * p.p56);
        let assign4380_e2955: f64 = (locals.var_lgate - assign4380_e2954);
        locals.var_lch = assign4380_e2955;
        locals.var_lch_dn0 = 0.0;
        locals.var_lch_dn2 = 0.0;
        locals.var_lch_dn6 = 0.0;
        locals.var_lch_dn7 = 0.0;
        locals.var_lch_dn10 = 0.0;
        locals.var_lch_dn11 = 0.0;
        locals.var_lch_dn12 = 0.0;
        locals.var_lch_dn17 = 0.0;
        locals.var_lch_rv = 0.0;

        let assign4390_e2958: f64 = if locals.var_subversion > 3.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign4390_e2958;
        locals.var_guard40_rv = 0.0;

        let (assign4400_e2969, assign4400_e2969_d_n0, assign4400_e2969_d_n2, assign4400_e2969_d_n6, assign4400_e2969_d_n7, assign4400_e2969_d_n10, assign4400_e2969_d_n11, assign4400_e2969_d_n12, assign4400_e2969_d_n17,) = {
    if (locals.var_guard40 != 0.0) {
        let assign4400_e2962: f64 = (2.0 * locals.var_beta_inv);
        let assign4400_e2965: f64 = (locals.var_nsub / locals.var_nin);
        let assign4400_e2966: f64 = (assign4400_e2965).ln();
        let assign4400_e2967: f64 = (assign4400_e2962 * assign4400_e2966);
        (assign4400_e2967, (assign4400_e2962 * ((((locals.var_nsub_dn0 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign4400_e2965)), (assign4400_e2962 * ((((locals.var_nsub_dn2 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign4400_e2965)), (assign4400_e2962 * ((((locals.var_nsub_dn6 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign4400_e2965)), (assign4400_e2962 * ((((locals.var_nsub_dn7 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign4400_e2965)), (((2.0 * locals.var_beta_inv_dn10) * assign4400_e2966) + (assign4400_e2962 * ((((locals.var_nsub_dn10 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign4400_e2965))), (assign4400_e2962 * ((((locals.var_nsub_dn11 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign4400_e2965)), (assign4400_e2962 * ((((locals.var_nsub_dn12 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn12)) / (locals.var_nin * locals.var_nin)) / assign4400_e2965)), (assign4400_e2962 * ((((locals.var_nsub_dn17 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn17)) / (locals.var_nin * locals.var_nin)) / assign4400_e2965)),)
    } else {
        (locals.var_pb2, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn10, locals.var_pb2_dn11, locals.var_pb2_dn12, locals.var_pb2_dn17,)
    }
};
        locals.var_pb2 = assign4400_e2969;
        locals.var_pb2_dn0 = assign4400_e2969_d_n0;
        locals.var_pb2_dn2 = assign4400_e2969_d_n2;
        locals.var_pb2_dn6 = assign4400_e2969_d_n6;
        locals.var_pb2_dn7 = assign4400_e2969_d_n7;
        locals.var_pb2_dn10 = assign4400_e2969_d_n10;
        locals.var_pb2_dn11 = assign4400_e2969_d_n11;
        locals.var_pb2_dn12 = assign4400_e2969_d_n12;
        locals.var_pb2_dn17 = assign4400_e2969_d_n17;
        locals.var_pb2_rv = 0.0;

        let (assign4410_e2981, assign4410_e2981_d_n0, assign4410_e2981_d_n2, assign4410_e2981_d_n6, assign4410_e2981_d_n7, assign4410_e2981_d_n10, assign4410_e2981_d_n11, assign4410_e2981_d_n12, assign4410_e2981_d_n17,) = {
    if (locals.var_guard40 == 0.0) {
        let assign4410_e2974: f64 = (2.0 * locals.var_beta_inv);
        let assign4410_e2977: f64 = (locals.var_uc_nsubs / locals.var_nin);
        let assign4410_e2978: f64 = (assign4410_e2977).ln();
        let assign4410_e2979: f64 = (assign4410_e2974 * assign4410_e2978);
        (assign4410_e2979, (assign4410_e2974 * ((((locals.var_uc_nsubs_dn0 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign4410_e2977)), (assign4410_e2974 * ((((locals.var_uc_nsubs_dn2 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign4410_e2977)), (assign4410_e2974 * ((((locals.var_uc_nsubs_dn6 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign4410_e2977)), (assign4410_e2974 * ((((locals.var_uc_nsubs_dn7 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign4410_e2977)), (((2.0 * locals.var_beta_inv_dn10) * assign4410_e2978) + (assign4410_e2974 * ((((locals.var_uc_nsubs_dn10 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign4410_e2977))), (assign4410_e2974 * ((((locals.var_uc_nsubs_dn11 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign4410_e2977)), (assign4410_e2974 * ((((locals.var_uc_nsubs_dn12 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn12)) / (locals.var_nin * locals.var_nin)) / assign4410_e2977)), (assign4410_e2974 * ((((locals.var_uc_nsubs_dn17 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn17)) / (locals.var_nin * locals.var_nin)) / assign4410_e2977)),)
    } else {
        (locals.var_pb2, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn10, locals.var_pb2_dn11, locals.var_pb2_dn12, locals.var_pb2_dn17,)
    }
};
        locals.var_pb2 = assign4410_e2981;
        locals.var_pb2_dn0 = assign4410_e2981_d_n0;
        locals.var_pb2_dn2 = assign4410_e2981_d_n2;
        locals.var_pb2_dn6 = assign4410_e2981_d_n6;
        locals.var_pb2_dn7 = assign4410_e2981_d_n7;
        locals.var_pb2_dn10 = assign4410_e2981_d_n10;
        locals.var_pb2_dn11 = assign4410_e2981_d_n11;
        locals.var_pb2_dn12 = assign4410_e2981_d_n12;
        locals.var_pb2_dn17 = assign4410_e2981_d_n17;
        locals.var_pb2_rv = 0.0;

        let assign4420_e2984: f64 = (1.034943e-10 / locals.var_q_nsub);
        let assign4420_e2986: f64 = (assign4420_e2984 * locals.var_beta_inv);
        let assign4420_e2987: f64 = (assign4420_e2986).sqrt();
        locals.var_ldby = assign4420_e2987;
        locals.var_ldby_dn0 = (((-((1.034943e-10 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign4420_e2987));
        locals.var_ldby_dn2 = (((-((1.034943e-10 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign4420_e2987));
        locals.var_ldby_dn6 = (((-((1.034943e-10 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign4420_e2987));
        locals.var_ldby_dn7 = (((-((1.034943e-10 * locals.var_q_nsub_dn7) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign4420_e2987));
        locals.var_ldby_dn10 = ((((-((1.034943e-10 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) + (assign4420_e2984 * locals.var_beta_inv_dn10)) / (2.0 * assign4420_e2987));
        locals.var_ldby_dn11 = (((-((1.034943e-10 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign4420_e2987));
        locals.var_ldby_dn12 = (((-((1.034943e-10 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign4420_e2987));
        locals.var_ldby_dn17 = (((-((1.034943e-10 * locals.var_q_nsub_dn17) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign4420_e2987));
        locals.var_ldby_rv = 0.0;

        let assign4430_e2990: f64 = (locals.var_q_nsub * 1.414213562373095);
        let assign4430_e2992: f64 = (assign4430_e2990 * locals.var_ldby);
        locals.var_cnst0soi = assign4430_e2992;
        locals.var_cnst0soi_dn0 = (((locals.var_q_nsub_dn0 * 1.414213562373095) * locals.var_ldby) + (assign4430_e2990 * locals.var_ldby_dn0));
        locals.var_cnst0soi_dn2 = (((locals.var_q_nsub_dn2 * 1.414213562373095) * locals.var_ldby) + (assign4430_e2990 * locals.var_ldby_dn2));
        locals.var_cnst0soi_dn6 = (((locals.var_q_nsub_dn6 * 1.414213562373095) * locals.var_ldby) + (assign4430_e2990 * locals.var_ldby_dn6));
        locals.var_cnst0soi_dn7 = (((locals.var_q_nsub_dn7 * 1.414213562373095) * locals.var_ldby) + (assign4430_e2990 * locals.var_ldby_dn7));
        locals.var_cnst0soi_dn10 = (((locals.var_q_nsub_dn10 * 1.414213562373095) * locals.var_ldby) + (assign4430_e2990 * locals.var_ldby_dn10));
        locals.var_cnst0soi_dn11 = (((locals.var_q_nsub_dn11 * 1.414213562373095) * locals.var_ldby) + (assign4430_e2990 * locals.var_ldby_dn11));
        locals.var_cnst0soi_dn12 = (((locals.var_q_nsub_dn12 * 1.414213562373095) * locals.var_ldby) + (assign4430_e2990 * locals.var_ldby_dn12));
        locals.var_cnst0soi_dn17 = (((locals.var_q_nsub_dn17 * 1.414213562373095) * locals.var_ldby) + (assign4430_e2990 * locals.var_ldby_dn17));
        locals.var_cnst0soi_rv = 0.0;

        let assign4440_e2995: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign4440_e2995;
        locals.var_guard41_rv = 0.0;

        let (assign4450_e2999, assign4450_e2999_d_n10,) = {
    if (locals.var_guard41 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_cnst0bulk, locals.var_cnst0bulk_dn10,)
    }
};
        locals.var_cnst0bulk = assign4450_e2999;
        locals.var_cnst0bulk_dn10 = assign4450_e2999_d_n10;
        locals.var_cnst0bulk_rv = 0.0;

        let (assign4460_e3003, assign4460_e3003_d_n0, assign4460_e3003_d_n2, assign4460_e3003_d_n6, assign4460_e3003_d_n7, assign4460_e3003_d_n10, assign4460_e3003_d_n11, assign4460_e3003_d_n12, assign4460_e3003_d_n17,) = {
    if (locals.var_guard41 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cnst1bulk, locals.var_cnst1bulk_dn0, locals.var_cnst1bulk_dn2, locals.var_cnst1bulk_dn6, locals.var_cnst1bulk_dn7, locals.var_cnst1bulk_dn10, locals.var_cnst1bulk_dn11, locals.var_cnst1bulk_dn12, locals.var_cnst1bulk_dn17,)
    }
};
        locals.var_cnst1bulk = assign4460_e3003;
        locals.var_cnst1bulk_dn0 = assign4460_e3003_d_n0;
        locals.var_cnst1bulk_dn2 = assign4460_e3003_d_n2;
        locals.var_cnst1bulk_dn6 = assign4460_e3003_d_n6;
        locals.var_cnst1bulk_dn7 = assign4460_e3003_d_n7;
        locals.var_cnst1bulk_dn10 = assign4460_e3003_d_n10;
        locals.var_cnst1bulk_dn11 = assign4460_e3003_d_n11;
        locals.var_cnst1bulk_dn12 = assign4460_e3003_d_n12;
        locals.var_cnst1bulk_dn17 = assign4460_e3003_d_n17;
        locals.var_cnst1bulk_rv = 0.0;

        let (assign4470_e3009, assign4470_e3009_d_n0, assign4470_e3009_d_n2, assign4470_e3009_d_n6, assign4470_e3009_d_n7, assign4470_e3009_d_n10, assign4470_e3009_d_n11, assign4470_e3009_d_n12, assign4470_e3009_d_n17,) = {
    if (locals.var_guard41 != 0.0) {
        let assign4470_e3007: f64 = (locals.var_nin / locals.var_nsub);
        (assign4470_e3007, (((locals.var_nin_dn0 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn2 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn6 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn7 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn10 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn11 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn12 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn12)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn17 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn17)) / (locals.var_nsub * locals.var_nsub)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign4470_e3009;
        locals.var_t1_dn0 = assign4470_e3009_d_n0;
        locals.var_t1_dn2 = assign4470_e3009_d_n2;
        locals.var_t1_dn6 = assign4470_e3009_d_n6;
        locals.var_t1_dn7 = assign4470_e3009_d_n7;
        locals.var_t1_dn10 = assign4470_e3009_d_n10;
        locals.var_t1_dn11 = assign4470_e3009_d_n11;
        locals.var_t1_dn12 = assign4470_e3009_d_n12;
        locals.var_t1_dn17 = assign4470_e3009_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign4480_e3019, assign4480_e3019_d_n10,) = {
    if (locals.var_guard41 == 0.0) {
        let assign4480_e3014: f64 = (2.0 * locals.var_c0bulk);
        let assign4480_e3016: f64 = (assign4480_e3014 * locals.var_beta_inv);
        let assign4480_e3017: f64 = (assign4480_e3016).sqrt();
        (assign4480_e3017, ((assign4480_e3014 * locals.var_beta_inv_dn10) / (2.0 * assign4480_e3017)),)
    } else {
        (locals.var_cnst0bulk, locals.var_cnst0bulk_dn10,)
    }
};
        locals.var_cnst0bulk = assign4480_e3019;
        locals.var_cnst0bulk_dn10 = assign4480_e3019_d_n10;
        locals.var_cnst0bulk_rv = 0.0;

        let (assign4490_e3026, assign4490_e3026_d_n0, assign4490_e3026_d_n2, assign4490_e3026_d_n6, assign4490_e3026_d_n7, assign4490_e3026_d_n10, assign4490_e3026_d_n11, assign4490_e3026_d_n12, assign4490_e3026_d_n17,) = {
    if (locals.var_guard41 == 0.0) {
        let assign4490_e3024: f64 = (locals.var_nin / locals.var_mks_nsubb);
        (assign4490_e3024, (locals.var_nin_dn0 / locals.var_mks_nsubb), (locals.var_nin_dn2 / locals.var_mks_nsubb), (locals.var_nin_dn6 / locals.var_mks_nsubb), (locals.var_nin_dn7 / locals.var_mks_nsubb), (locals.var_nin_dn10 / locals.var_mks_nsubb), (locals.var_nin_dn11 / locals.var_mks_nsubb), (locals.var_nin_dn12 / locals.var_mks_nsubb), (locals.var_nin_dn17 / locals.var_mks_nsubb),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign4490_e3026;
        locals.var_t1_dn0 = assign4490_e3026_d_n0;
        locals.var_t1_dn2 = assign4490_e3026_d_n2;
        locals.var_t1_dn6 = assign4490_e3026_d_n6;
        locals.var_t1_dn7 = assign4490_e3026_d_n7;
        locals.var_t1_dn10 = assign4490_e3026_d_n10;
        locals.var_t1_dn11 = assign4490_e3026_d_n11;
        locals.var_t1_dn12 = assign4490_e3026_d_n12;
        locals.var_t1_dn17 = assign4490_e3026_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign4500_e3033, assign4500_e3033_d_n0, assign4500_e3033_d_n2, assign4500_e3033_d_n6, assign4500_e3033_d_n7, assign4500_e3033_d_n10, assign4500_e3033_d_n11, assign4500_e3033_d_n12, assign4500_e3033_d_n17,) = {
    if (locals.var_guard41 == 0.0) {
        let assign4500_e3031: f64 = (locals.var_t1 * locals.var_t1);
        (assign4500_e3031, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)), ((locals.var_t1_dn17 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn17)),)
    } else {
        (locals.var_cnst1bulk, locals.var_cnst1bulk_dn0, locals.var_cnst1bulk_dn2, locals.var_cnst1bulk_dn6, locals.var_cnst1bulk_dn7, locals.var_cnst1bulk_dn10, locals.var_cnst1bulk_dn11, locals.var_cnst1bulk_dn12, locals.var_cnst1bulk_dn17,)
    }
};
        locals.var_cnst1bulk = assign4500_e3033;
        locals.var_cnst1bulk_dn0 = assign4500_e3033_d_n0;
        locals.var_cnst1bulk_dn2 = assign4500_e3033_d_n2;
        locals.var_cnst1bulk_dn6 = assign4500_e3033_d_n6;
        locals.var_cnst1bulk_dn7 = assign4500_e3033_d_n7;
        locals.var_cnst1bulk_dn10 = assign4500_e3033_d_n10;
        locals.var_cnst1bulk_dn11 = assign4500_e3033_d_n11;
        locals.var_cnst1bulk_dn12 = assign4500_e3033_d_n12;
        locals.var_cnst1bulk_dn17 = assign4500_e3033_d_n17;
        locals.var_cnst1bulk_rv = 0.0;

        let (assign4510_e3040, assign4510_e3040_d_n0, assign4510_e3040_d_n2, assign4510_e3040_d_n6, assign4510_e3040_d_n7, assign4510_e3040_d_n10, assign4510_e3040_d_n11, assign4510_e3040_d_n12, assign4510_e3040_d_n17,) = {
    if (locals.var_guard41 == 0.0) {
        let assign4510_e3038: f64 = (locals.var_nin / locals.var_uc_nsubs);
        (assign4510_e3038, (((locals.var_nin_dn0 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_nin_dn2 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_nin_dn6 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_nin_dn7 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_nin_dn10 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_nin_dn11 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_nin_dn12 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_nin_dn17 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign4510_e3040;
        locals.var_t1_dn0 = assign4510_e3040_d_n0;
        locals.var_t1_dn2 = assign4510_e3040_d_n2;
        locals.var_t1_dn6 = assign4510_e3040_d_n6;
        locals.var_t1_dn7 = assign4510_e3040_d_n7;
        locals.var_t1_dn10 = assign4510_e3040_d_n10;
        locals.var_t1_dn11 = assign4510_e3040_d_n11;
        locals.var_t1_dn12 = assign4510_e3040_d_n12;
        locals.var_t1_dn17 = assign4510_e3040_d_n17;
        locals.var_t1_rv = 0.0;

        let assign4520_e3043: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_cnst1soi = assign4520_e3043;
        locals.var_cnst1soi_dn0 = ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0));
        locals.var_cnst1soi_dn2 = ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2));
        locals.var_cnst1soi_dn6 = ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6));
        locals.var_cnst1soi_dn7 = ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7));
        locals.var_cnst1soi_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_cnst1soi_dn11 = ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11));
        locals.var_cnst1soi_dn12 = ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12));
        locals.var_cnst1soi_dn17 = ((locals.var_t1_dn17 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn17));
        locals.var_cnst1soi_rv = 0.0;

        let assign4530_e3047: f64 = (1.034943e-10 / locals.var_q_nsub);
        let assign4530_e3049: f64 = (assign4530_e3047 / locals.var_beta);
        let assign4530_e3050: f64 = (2.0 * assign4530_e3049);
        let assign4530_e3051: f64 = (assign4530_e3050).sqrt();
        locals.var_c_w_soi = assign4530_e3051;
        locals.var_c_w_soi_dn0 = ((2.0 * ((-((1.034943e-10 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))) / locals.var_beta)) / (2.0 * assign4530_e3051));
        locals.var_c_w_soi_dn2 = ((2.0 * ((-((1.034943e-10 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))) / locals.var_beta)) / (2.0 * assign4530_e3051));
        locals.var_c_w_soi_dn6 = ((2.0 * ((-((1.034943e-10 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))) / locals.var_beta)) / (2.0 * assign4530_e3051));
        locals.var_c_w_soi_dn7 = ((2.0 * ((-((1.034943e-10 * locals.var_q_nsub_dn7) / (locals.var_q_nsub * locals.var_q_nsub))) / locals.var_beta)) / (2.0 * assign4530_e3051));
        locals.var_c_w_soi_dn10 = ((2.0 * ((((-((1.034943e-10 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta) - (assign4530_e3047 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta))) / (2.0 * assign4530_e3051));
        locals.var_c_w_soi_dn11 = ((2.0 * ((-((1.034943e-10 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))) / locals.var_beta)) / (2.0 * assign4530_e3051));
        locals.var_c_w_soi_dn12 = ((2.0 * ((-((1.034943e-10 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))) / locals.var_beta)) / (2.0 * assign4530_e3051));
        locals.var_c_w_soi_dn17 = ((2.0 * ((-((1.034943e-10 * locals.var_q_nsub_dn17) / (locals.var_q_nsub * locals.var_q_nsub))) / locals.var_beta)) / (2.0 * assign4530_e3051));
        locals.var_c_w_soi_rv = 0.0;

        let assign4540_e3054: f64 = (2.0 * 1.034943e-10);
        let assign4540_e3056: f64 = (assign4540_e3054 / 1.6021918e-19);
        let assign4540_e3058: f64 = (assign4540_e3056 / locals.var_uc_nsubs);
        locals.var_cnst_2esi_q_nsubs = assign4540_e3058;
        locals.var_cnst_2esi_q_nsubs_dn0 = (-((assign4540_e3056 * locals.var_uc_nsubs_dn0) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));
        locals.var_cnst_2esi_q_nsubs_dn2 = (-((assign4540_e3056 * locals.var_uc_nsubs_dn2) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));
        locals.var_cnst_2esi_q_nsubs_dn6 = (-((assign4540_e3056 * locals.var_uc_nsubs_dn6) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));
        locals.var_cnst_2esi_q_nsubs_dn7 = (-((assign4540_e3056 * locals.var_uc_nsubs_dn7) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));
        locals.var_cnst_2esi_q_nsubs_dn10 = (-((assign4540_e3056 * locals.var_uc_nsubs_dn10) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));
        locals.var_cnst_2esi_q_nsubs_dn11 = (-((assign4540_e3056 * locals.var_uc_nsubs_dn11) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));
        locals.var_cnst_2esi_q_nsubs_dn12 = (-((assign4540_e3056 * locals.var_uc_nsubs_dn12) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));
        locals.var_cnst_2esi_q_nsubs_dn17 = (-((assign4540_e3056 * locals.var_uc_nsubs_dn17) / (locals.var_uc_nsubs * locals.var_uc_nsubs)));
        locals.var_cnst_2esi_q_nsubs_rv = 0.0;

        let assign4550_e3061: f64 = (2.0 * 1.034943e-10);
        let assign4550_e3063: f64 = (assign4550_e3061 / 1.6021918e-19);
        let assign4550_e3065: f64 = (assign4550_e3063 * locals.var_pb2);
        let assign4550_e3067: f64 = (assign4550_e3065 / locals.var_uc_nsubs);
        let assign4550_e3068: f64 = (assign4550_e3067).sqrt();
        locals.var_wdsoi_ini = assign4550_e3068;
        locals.var_wdsoi_ini_dn0 = (((((assign4550_e3063 * locals.var_pb2_dn0) * locals.var_uc_nsubs) - (assign4550_e3065 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign4550_e3068));
        locals.var_wdsoi_ini_dn2 = (((((assign4550_e3063 * locals.var_pb2_dn2) * locals.var_uc_nsubs) - (assign4550_e3065 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign4550_e3068));
        locals.var_wdsoi_ini_dn6 = (((((assign4550_e3063 * locals.var_pb2_dn6) * locals.var_uc_nsubs) - (assign4550_e3065 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign4550_e3068));
        locals.var_wdsoi_ini_dn7 = (((((assign4550_e3063 * locals.var_pb2_dn7) * locals.var_uc_nsubs) - (assign4550_e3065 * locals.var_uc_nsubs_dn7)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign4550_e3068));
        locals.var_wdsoi_ini_dn10 = (((((assign4550_e3063 * locals.var_pb2_dn10) * locals.var_uc_nsubs) - (assign4550_e3065 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign4550_e3068));
        locals.var_wdsoi_ini_dn11 = (((((assign4550_e3063 * locals.var_pb2_dn11) * locals.var_uc_nsubs) - (assign4550_e3065 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign4550_e3068));
        locals.var_wdsoi_ini_dn12 = (((((assign4550_e3063 * locals.var_pb2_dn12) * locals.var_uc_nsubs) - (assign4550_e3065 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign4550_e3068));
        locals.var_wdsoi_ini_dn17 = (((((assign4550_e3063 * locals.var_pb2_dn17) * locals.var_uc_nsubs) - (assign4550_e3065 * locals.var_uc_nsubs_dn17)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign4550_e3068));
        locals.var_wdsoi_ini_rv = 0.0;

        let assign4630_e3093: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard46 = assign4630_e3093;
        locals.var_guard46_rv = 0.0;

        let (assign4640_e3097,) = {
    if (locals.var_guard46 != 0.0) {
        (0.4,)
    } else {
        (locals.var_vbs_bnd,)
    }
};
        locals.var_vbs_bnd = assign4640_e3097;
        locals.var_vbs_bnd_rv = 0.0;

        let (assign4650_e3101,) = {
    if (locals.var_guard46 != 0.0) {
        (0.8,)
    } else {
        (locals.var_vbs_max,)
    }
};
        locals.var_vbs_max = assign4650_e3101;
        locals.var_vbs_max_rv = 0.0;

        let (assign4660_e3106,) = {
    if (locals.var_guard46 == 0.0) {
        (0.8,)
    } else {
        (locals.var_vbs_bnd,)
    }
};
        locals.var_vbs_bnd = assign4660_e3106;
        locals.var_vbs_bnd_rv = 0.0;

        let (assign4670_e3111,) = {
    if (locals.var_guard46 == 0.0) {
        (1.2,)
    } else {
        (locals.var_vbs_max,)
    }
};
        locals.var_vbs_max = assign4670_e3111;
        locals.var_vbs_max_rv = 0.0;

        let assign4680_e3115: f64 = (locals.var_vbs_max * 0.5);
        let assign4680_e3116: f64 = if locals.var_vbs_bnd > assign4680_e3115 { 1.0 } else { 0.0 };
        locals.var_guard47 = assign4680_e3116;
        locals.var_guard47_rv = 0.0;

        let (assign4690_e3122,) = {
    if (locals.var_guard47 != 0.0) {
        let assign4690_e3120: f64 = (0.5 * locals.var_vbs_max);
        (assign4690_e3120,)
    } else {
        (locals.var_vbs_bnd,)
    }
};
        locals.var_vbs_bnd = assign4690_e3122;
        locals.var_vbs_bnd_rv = 0.0;

        let assign4700_e3125: f64 = if locals.var_vbs > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard48 = assign4700_e3125;
        locals.var_guard48_rv = 0.0;

        let (assign4710_e3131, assign4710_e3131_d_n0, assign4710_e3131_d_n2, assign4710_e3131_d_n6, assign4710_e3131_d_n7, assign4710_e3131_d_n10, assign4710_e3131_d_n11, assign4710_e3131_d_n12, assign4710_e3131_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign4710_e3129: f64 = (locals.var_vbs - locals.var_vbs_bnd);
        (assign4710_e3129, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign4710_e3131;
        locals.var_t2_dn0 = assign4710_e3131_d_n0;
        locals.var_t2_dn2 = assign4710_e3131_d_n2;
        locals.var_t2_dn6 = assign4710_e3131_d_n6;
        locals.var_t2_dn7 = assign4710_e3131_d_n7;
        locals.var_t2_dn10 = assign4710_e3131_d_n10;
        locals.var_t2_dn11 = assign4710_e3131_d_n11;
        locals.var_t2_dn12 = assign4710_e3131_d_n12;
        locals.var_t2_dn17 = assign4710_e3131_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign4720_e3137, assign4720_e3137_d_n0, assign4720_e3137_d_n2, assign4720_e3137_d_n6, assign4720_e3137_d_n7, assign4720_e3137_d_n10, assign4720_e3137_d_n11, assign4720_e3137_d_n12, assign4720_e3137_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign4720_e3135: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign4720_e3135, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign4720_e3137;
        locals.var_t3_dn0 = assign4720_e3137_d_n0;
        locals.var_t3_dn2 = assign4720_e3137_d_n2;
        locals.var_t3_dn6 = assign4720_e3137_d_n6;
        locals.var_t3_dn7 = assign4720_e3137_d_n7;
        locals.var_t3_dn10 = assign4720_e3137_d_n10;
        locals.var_t3_dn11 = assign4720_e3137_d_n11;
        locals.var_t3_dn12 = assign4720_e3137_d_n12;
        locals.var_t3_dn17 = assign4720_e3137_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign4730_e3143, assign4730_e3143_d_n0, assign4730_e3143_d_n2, assign4730_e3143_d_n6, assign4730_e3143_d_n7, assign4730_e3143_d_n10, assign4730_e3143_d_n11, assign4730_e3143_d_n12, assign4730_e3143_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign4730_e3141: f64 = (locals.var_t2 * locals.var_t2);
        (assign4730_e3141, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)), ((locals.var_t2_dn17 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn17)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12, locals.var_x2_dn17,)
    }
};
        locals.var_x2 = assign4730_e3143;
        locals.var_x2_dn0 = assign4730_e3143_d_n0;
        locals.var_x2_dn2 = assign4730_e3143_d_n2;
        locals.var_x2_dn6 = assign4730_e3143_d_n6;
        locals.var_x2_dn7 = assign4730_e3143_d_n7;
        locals.var_x2_dn10 = assign4730_e3143_d_n10;
        locals.var_x2_dn11 = assign4730_e3143_d_n11;
        locals.var_x2_dn12 = assign4730_e3143_d_n12;
        locals.var_x2_dn17 = assign4730_e3143_d_n17;
        locals.var_x2_rv = 0.0;

        let (assign4740_e3149, assign4740_e3149_d_n0, assign4740_e3149_d_n2, assign4740_e3149_d_n6, assign4740_e3149_d_n7, assign4740_e3149_d_n10, assign4740_e3149_d_n11, assign4740_e3149_d_n12, assign4740_e3149_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign4740_e3147: f64 = (locals.var_t3 * locals.var_t3);
        (assign4740_e3147, ((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)), ((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)), ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)), ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)), ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)), ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)), ((locals.var_t3_dn12 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn12)), ((locals.var_t3_dn17 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn17)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12, locals.var_xmax2_dn17,)
    }
};
        locals.var_xmax2 = assign4740_e3149;
        locals.var_xmax2_dn0 = assign4740_e3149_d_n0;
        locals.var_xmax2_dn2 = assign4740_e3149_d_n2;
        locals.var_xmax2_dn6 = assign4740_e3149_d_n6;
        locals.var_xmax2_dn7 = assign4740_e3149_d_n7;
        locals.var_xmax2_dn10 = assign4740_e3149_d_n10;
        locals.var_xmax2_dn11 = assign4740_e3149_d_n11;
        locals.var_xmax2_dn12 = assign4740_e3149_d_n12;
        locals.var_xmax2_dn17 = assign4740_e3149_d_n17;
        locals.var_xmax2_rv = 0.0;

        let (assign4750_e3153, assign4750_e3153_d_n0, assign4750_e3153_d_n2, assign4750_e3153_d_n6, assign4750_e3153_d_n7, assign4750_e3153_d_n10, assign4750_e3153_d_n11, assign4750_e3153_d_n12, assign4750_e3153_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign4750_e3153;
        locals.var_xp_dn0 = assign4750_e3153_d_n0;
        locals.var_xp_dn2 = assign4750_e3153_d_n2;
        locals.var_xp_dn6 = assign4750_e3153_d_n6;
        locals.var_xp_dn7 = assign4750_e3153_d_n7;
        locals.var_xp_dn10 = assign4750_e3153_d_n10;
        locals.var_xp_dn11 = assign4750_e3153_d_n11;
        locals.var_xp_dn12 = assign4750_e3153_d_n12;
        locals.var_xp_dn17 = assign4750_e3153_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign4760_e3157, assign4760_e3157_d_n0, assign4760_e3157_d_n2, assign4760_e3157_d_n6, assign4760_e3157_d_n7, assign4760_e3157_d_n10, assign4760_e3157_d_n11, assign4760_e3157_d_n12, assign4760_e3157_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign4760_e3157;
        locals.var_xmp_dn0 = assign4760_e3157_d_n0;
        locals.var_xmp_dn2 = assign4760_e3157_d_n2;
        locals.var_xmp_dn6 = assign4760_e3157_d_n6;
        locals.var_xmp_dn7 = assign4760_e3157_d_n7;
        locals.var_xmp_dn10 = assign4760_e3157_d_n10;
        locals.var_xmp_dn11 = assign4760_e3157_d_n11;
        locals.var_xmp_dn12 = assign4760_e3157_d_n12;
        locals.var_xmp_dn17 = assign4760_e3157_d_n17;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_9(
        locals: &mut StampLocals,
    ) {
        let (assign4770_e3161,) = {
    if (locals.var_guard48 != 0.0) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign4770_e3161;
        locals.var_m0_rv = 0.0;

        let (assign4780_e3165,) = {
    if (locals.var_guard48 != 0.0) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign4780_e3165;
        locals.var_mm_rv = 0.0;

        let (assign4790_e3169, assign4790_e3169_d_n0, assign4790_e3169_d_n2, assign4790_e3169_d_n6, assign4790_e3169_d_n7, assign4790_e3169_d_n10, assign4790_e3169_d_n11, assign4790_e3169_d_n12, assign4790_e3169_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign4790_e3169;
        locals.var_arg_dn0 = assign4790_e3169_d_n0;
        locals.var_arg_dn2 = assign4790_e3169_d_n2;
        locals.var_arg_dn6 = assign4790_e3169_d_n6;
        locals.var_arg_dn7 = assign4790_e3169_d_n7;
        locals.var_arg_dn10 = assign4790_e3169_d_n10;
        locals.var_arg_dn11 = assign4790_e3169_d_n11;
        locals.var_arg_dn12 = assign4790_e3169_d_n12;
        locals.var_arg_dn17 = assign4790_e3169_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign4800_e3173, assign4800_e3173_d_n0, assign4800_e3173_d_n2, assign4800_e3173_d_n6, assign4800_e3173_d_n7, assign4800_e3173_d_n10, assign4800_e3173_d_n11, assign4800_e3173_d_n12, assign4800_e3173_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign4800_e3173;
        locals.var_dnm_dn0 = assign4800_e3173_d_n0;
        locals.var_dnm_dn2 = assign4800_e3173_d_n2;
        locals.var_dnm_dn6 = assign4800_e3173_d_n6;
        locals.var_dnm_dn7 = assign4800_e3173_d_n7;
        locals.var_dnm_dn10 = assign4800_e3173_d_n10;
        locals.var_dnm_dn11 = assign4800_e3173_d_n11;
        locals.var_dnm_dn12 = assign4800_e3173_d_n12;
        locals.var_dnm_dn17 = assign4800_e3173_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign4810_e3179, assign4810_e3179_d_n0, assign4810_e3179_d_n2, assign4810_e3179_d_n6, assign4810_e3179_d_n7, assign4810_e3179_d_n10, assign4810_e3179_d_n11, assign4810_e3179_d_n12, assign4810_e3179_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign4810_e3177: f64 = (locals.var_xp * locals.var_x2);
        (assign4810_e3177, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign4810_e3179;
        locals.var_xp_dn0 = assign4810_e3179_d_n0;
        locals.var_xp_dn2 = assign4810_e3179_d_n2;
        locals.var_xp_dn6 = assign4810_e3179_d_n6;
        locals.var_xp_dn7 = assign4810_e3179_d_n7;
        locals.var_xp_dn10 = assign4810_e3179_d_n10;
        locals.var_xp_dn11 = assign4810_e3179_d_n11;
        locals.var_xp_dn12 = assign4810_e3179_d_n12;
        locals.var_xp_dn17 = assign4810_e3179_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign4820_e3185, assign4820_e3185_d_n0, assign4820_e3185_d_n2, assign4820_e3185_d_n6, assign4820_e3185_d_n7, assign4820_e3185_d_n10, assign4820_e3185_d_n11, assign4820_e3185_d_n12, assign4820_e3185_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign4820_e3183: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign4820_e3183, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign4820_e3185;
        locals.var_xmp_dn0 = assign4820_e3185_d_n0;
        locals.var_xmp_dn2 = assign4820_e3185_d_n2;
        locals.var_xmp_dn6 = assign4820_e3185_d_n6;
        locals.var_xmp_dn7 = assign4820_e3185_d_n7;
        locals.var_xmp_dn10 = assign4820_e3185_d_n10;
        locals.var_xmp_dn11 = assign4820_e3185_d_n11;
        locals.var_xmp_dn12 = assign4820_e3185_d_n12;
        locals.var_xmp_dn17 = assign4820_e3185_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign4830_e3191, assign4830_e3191_d_n0, assign4830_e3191_d_n2, assign4830_e3191_d_n6, assign4830_e3191_d_n7, assign4830_e3191_d_n10, assign4830_e3191_d_n11, assign4830_e3191_d_n12, assign4830_e3191_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign4830_e3189: f64 = (locals.var_xp * locals.var_x2);
        (assign4830_e3189, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign4830_e3191;
        locals.var_xp_dn0 = assign4830_e3191_d_n0;
        locals.var_xp_dn2 = assign4830_e3191_d_n2;
        locals.var_xp_dn6 = assign4830_e3191_d_n6;
        locals.var_xp_dn7 = assign4830_e3191_d_n7;
        locals.var_xp_dn10 = assign4830_e3191_d_n10;
        locals.var_xp_dn11 = assign4830_e3191_d_n11;
        locals.var_xp_dn12 = assign4830_e3191_d_n12;
        locals.var_xp_dn17 = assign4830_e3191_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign4840_e3197, assign4840_e3197_d_n0, assign4840_e3197_d_n2, assign4840_e3197_d_n6, assign4840_e3197_d_n7, assign4840_e3197_d_n10, assign4840_e3197_d_n11, assign4840_e3197_d_n12, assign4840_e3197_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign4840_e3195: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign4840_e3195, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign4840_e3197;
        locals.var_xmp_dn0 = assign4840_e3197_d_n0;
        locals.var_xmp_dn2 = assign4840_e3197_d_n2;
        locals.var_xmp_dn6 = assign4840_e3197_d_n6;
        locals.var_xmp_dn7 = assign4840_e3197_d_n7;
        locals.var_xmp_dn10 = assign4840_e3197_d_n10;
        locals.var_xmp_dn11 = assign4840_e3197_d_n11;
        locals.var_xmp_dn12 = assign4840_e3197_d_n12;
        locals.var_xmp_dn17 = assign4840_e3197_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign4850_e3203, assign4850_e3203_d_n0, assign4850_e3203_d_n2, assign4850_e3203_d_n6, assign4850_e3203_d_n7, assign4850_e3203_d_n10, assign4850_e3203_d_n11, assign4850_e3203_d_n12, assign4850_e3203_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign4850_e3201: f64 = (locals.var_xp * locals.var_x2);
        (assign4850_e3201, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign4850_e3203;
        locals.var_xp_dn0 = assign4850_e3203_d_n0;
        locals.var_xp_dn2 = assign4850_e3203_d_n2;
        locals.var_xp_dn6 = assign4850_e3203_d_n6;
        locals.var_xp_dn7 = assign4850_e3203_d_n7;
        locals.var_xp_dn10 = assign4850_e3203_d_n10;
        locals.var_xp_dn11 = assign4850_e3203_d_n11;
        locals.var_xp_dn12 = assign4850_e3203_d_n12;
        locals.var_xp_dn17 = assign4850_e3203_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign4860_e3209, assign4860_e3209_d_n0, assign4860_e3209_d_n2, assign4860_e3209_d_n6, assign4860_e3209_d_n7, assign4860_e3209_d_n10, assign4860_e3209_d_n11, assign4860_e3209_d_n12, assign4860_e3209_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign4860_e3207: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign4860_e3207, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign4860_e3209;
        locals.var_xmp_dn0 = assign4860_e3209_d_n0;
        locals.var_xmp_dn2 = assign4860_e3209_d_n2;
        locals.var_xmp_dn6 = assign4860_e3209_d_n6;
        locals.var_xmp_dn7 = assign4860_e3209_d_n7;
        locals.var_xmp_dn10 = assign4860_e3209_d_n10;
        locals.var_xmp_dn11 = assign4860_e3209_d_n11;
        locals.var_xmp_dn12 = assign4860_e3209_d_n12;
        locals.var_xmp_dn17 = assign4860_e3209_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign4870_e3215, assign4870_e3215_d_n0, assign4870_e3215_d_n2, assign4870_e3215_d_n6, assign4870_e3215_d_n7, assign4870_e3215_d_n10, assign4870_e3215_d_n11, assign4870_e3215_d_n12, assign4870_e3215_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign4870_e3213: f64 = (locals.var_xp * locals.var_x2);
        (assign4870_e3213, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)), ((locals.var_xp_dn17 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn17)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12, locals.var_xp_dn17,)
    }
};
        locals.var_xp = assign4870_e3215;
        locals.var_xp_dn0 = assign4870_e3215_d_n0;
        locals.var_xp_dn2 = assign4870_e3215_d_n2;
        locals.var_xp_dn6 = assign4870_e3215_d_n6;
        locals.var_xp_dn7 = assign4870_e3215_d_n7;
        locals.var_xp_dn10 = assign4870_e3215_d_n10;
        locals.var_xp_dn11 = assign4870_e3215_d_n11;
        locals.var_xp_dn12 = assign4870_e3215_d_n12;
        locals.var_xp_dn17 = assign4870_e3215_d_n17;
        locals.var_xp_rv = 0.0;

        let (assign4880_e3221, assign4880_e3221_d_n0, assign4880_e3221_d_n2, assign4880_e3221_d_n6, assign4880_e3221_d_n7, assign4880_e3221_d_n10, assign4880_e3221_d_n11, assign4880_e3221_d_n12, assign4880_e3221_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign4880_e3219: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign4880_e3219, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)), ((locals.var_xmp_dn17 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn17)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12, locals.var_xmp_dn17,)
    }
};
        locals.var_xmp = assign4880_e3221;
        locals.var_xmp_dn0 = assign4880_e3221_d_n0;
        locals.var_xmp_dn2 = assign4880_e3221_d_n2;
        locals.var_xmp_dn6 = assign4880_e3221_d_n6;
        locals.var_xmp_dn7 = assign4880_e3221_d_n7;
        locals.var_xmp_dn10 = assign4880_e3221_d_n10;
        locals.var_xmp_dn11 = assign4880_e3221_d_n11;
        locals.var_xmp_dn12 = assign4880_e3221_d_n12;
        locals.var_xmp_dn17 = assign4880_e3221_d_n17;
        locals.var_xmp_rv = 0.0;

        let (assign4890_e3227, assign4890_e3227_d_n0, assign4890_e3227_d_n2, assign4890_e3227_d_n6, assign4890_e3227_d_n7, assign4890_e3227_d_n10, assign4890_e3227_d_n11, assign4890_e3227_d_n12, assign4890_e3227_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign4890_e3225: f64 = (locals.var_xp + locals.var_xmp);
        (assign4890_e3225, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12), (locals.var_xp_dn17 + locals.var_xmp_dn17),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    }
};
        locals.var_arg = assign4890_e3227;
        locals.var_arg_dn0 = assign4890_e3227_d_n0;
        locals.var_arg_dn2 = assign4890_e3227_d_n2;
        locals.var_arg_dn6 = assign4890_e3227_d_n6;
        locals.var_arg_dn7 = assign4890_e3227_d_n7;
        locals.var_arg_dn10 = assign4890_e3227_d_n10;
        locals.var_arg_dn11 = assign4890_e3227_d_n11;
        locals.var_arg_dn12 = assign4890_e3227_d_n12;
        locals.var_arg_dn17 = assign4890_e3227_d_n17;
        locals.var_arg_rv = 0.0;

        let (assign4900_e3231, assign4900_e3231_d_n0, assign4900_e3231_d_n2, assign4900_e3231_d_n6, assign4900_e3231_d_n7, assign4900_e3231_d_n10, assign4900_e3231_d_n11, assign4900_e3231_d_n12, assign4900_e3231_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn17,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign4900_e3231;
        locals.var_dnm_dn0 = assign4900_e3231_d_n0;
        locals.var_dnm_dn2 = assign4900_e3231_d_n2;
        locals.var_dnm_dn6 = assign4900_e3231_d_n6;
        locals.var_dnm_dn7 = assign4900_e3231_d_n7;
        locals.var_dnm_dn10 = assign4900_e3231_d_n10;
        locals.var_dnm_dn11 = assign4900_e3231_d_n11;
        locals.var_dnm_dn12 = assign4900_e3231_d_n12;
        locals.var_dnm_dn17 = assign4900_e3231_d_n17;
        locals.var_dnm_rv = 0.0;

        let assign4910_e3246: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard49 = assign4910_e3246;
        locals.var_guard49_rv = 0.0;

        let assign4920_e3249: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard50 = assign4920_e3249;
        locals.var_guard50_rv = 0.0;

        let (assign4930_e3257,) = {
    if (((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) && (locals.var_guard50 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign4930_e3257;
        locals.var_mm_rv = 0.0;

        let assign4940_e3260: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard51 = assign4940_e3260;
        locals.var_guard51_rv = 0.0;

        let (assign4950_e3271,) = {
    if ((((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) && (locals.var_guard50 == 0.0)) && (locals.var_guard51 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign4950_e3271;
        locals.var_mm_rv = 0.0;

        let assign4960_e3274: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard52 = assign4960_e3274;
        locals.var_guard52_rv = 0.0;

        let (assign4970_e3288,) = {
    if (((((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) && (locals.var_guard50 == 0.0)) && (locals.var_guard51 == 0.0)) && (locals.var_guard52 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign4970_e3288;
        locals.var_mm_rv = 0.0;

        let assign4980_e3291: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard53 = assign4980_e3291;
        locals.var_guard53_rv = 0.0;

        let (assign4990_e3308,) = {
    if ((((((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) && (locals.var_guard50 == 0.0)) && (locals.var_guard51 == 0.0)) && (locals.var_guard52 == 0.0)) && (locals.var_guard53 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign4990_e3308;
        locals.var_mm_rv = 0.0;

        let (assign5000_e3314,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign5000_e3314;
        locals.var_m0_rv = 0.0;

        let mut assign5010_loop_guard: usize = 0;
        while {
            let assign5010_cond_e3321: f64 = if (((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign5010_cond_e3321 != 0.0
        } {
            assign5010_loop_guard += 1;
            assert!(assign5010_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign5010_body0_e3328, assign5010_body0_e3328_d_n0, assign5010_body0_e3328_d_n2, assign5010_body0_e3328_d_n6, assign5010_body0_e3328_d_n7, assign5010_body0_e3328_d_n10, assign5010_body0_e3328_d_n11, assign5010_body0_e3328_d_n12, assign5010_body0_e3328_d_n17,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) {
        let assign5010_body0_e3326: f64 = (locals.var_dnm).sqrt();
        (assign5010_body0_e3326, (locals.var_dnm_dn0 / (2.0 * assign5010_body0_e3326)), (locals.var_dnm_dn2 / (2.0 * assign5010_body0_e3326)), (locals.var_dnm_dn6 / (2.0 * assign5010_body0_e3326)), (locals.var_dnm_dn7 / (2.0 * assign5010_body0_e3326)), (locals.var_dnm_dn10 / (2.0 * assign5010_body0_e3326)), (locals.var_dnm_dn11 / (2.0 * assign5010_body0_e3326)), (locals.var_dnm_dn12 / (2.0 * assign5010_body0_e3326)), (locals.var_dnm_dn17 / (2.0 * assign5010_body0_e3326)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
            locals.var_dnm = assign5010_body0_e3328;
            locals.var_dnm_dn0 = assign5010_body0_e3328_d_n0;
            locals.var_dnm_dn2 = assign5010_body0_e3328_d_n2;
            locals.var_dnm_dn6 = assign5010_body0_e3328_d_n6;
            locals.var_dnm_dn7 = assign5010_body0_e3328_d_n7;
            locals.var_dnm_dn10 = assign5010_body0_e3328_d_n10;
            locals.var_dnm_dn11 = assign5010_body0_e3328_d_n11;
            locals.var_dnm_dn12 = assign5010_body0_e3328_d_n12;
            locals.var_dnm_dn17 = assign5010_body0_e3328_d_n17;
            locals.var_dnm_rv = 0.0;
            let (assign5010_body1_e3336,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 != 0.0)) {
        let assign5010_body1_e3334: f64 = (locals.var_m0 + 1.0);
        (assign5010_body1_e3334,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign5010_body1_e3336;
            locals.var_m0_rv = 0.0;
        }

        let (assign5020_e3349, assign5020_e3349_d_n0, assign5020_e3349_d_n2, assign5020_e3349_d_n6, assign5020_e3349_d_n7, assign5020_e3349_d_n10, assign5020_e3349_d_n11, assign5020_e3349_d_n12, assign5020_e3349_d_n17,) = {
    if ((locals.var_guard48 != 0.0) && (locals.var_guard49 == 0.0)) {
        let assign5020_e3345: f64 = (2.0 * 4.0);
        let assign5020_e3346: f64 = (1.0 / assign5020_e3345);
        let assign5020_e3347: f64 = (locals.var_dnm).powf(assign5020_e3346);
        (assign5020_e3347, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((locals.var_dnm).powf(assign5020_e3346 - 1.0) * locals.var_dnm_dn0)) } } else { (assign5020_e3347 * (assign5020_e3346 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((locals.var_dnm).powf(assign5020_e3346 - 1.0) * locals.var_dnm_dn2)) } } else { (assign5020_e3347 * (assign5020_e3346 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((locals.var_dnm).powf(assign5020_e3346 - 1.0) * locals.var_dnm_dn6)) } } else { (assign5020_e3347 * (assign5020_e3346 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((locals.var_dnm).powf(assign5020_e3346 - 1.0) * locals.var_dnm_dn7)) } } else { (assign5020_e3347 * (assign5020_e3346 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((locals.var_dnm).powf(assign5020_e3346 - 1.0) * locals.var_dnm_dn10)) } } else { (assign5020_e3347 * (assign5020_e3346 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((locals.var_dnm).powf(assign5020_e3346 - 1.0) * locals.var_dnm_dn11)) } } else { (assign5020_e3347 * (assign5020_e3346 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((locals.var_dnm).powf(assign5020_e3346 - 1.0) * locals.var_dnm_dn12)) } } else { (assign5020_e3347 * (assign5020_e3346 * (locals.var_dnm_dn12 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign5020_e3346) as f64).is_finite() && ((assign5020_e3346) as f64).fract() == 0.0 { if assign5020_e3346 == 0.0 { 0.0 } else { (assign5020_e3346 * ((locals.var_dnm).powf(assign5020_e3346 - 1.0) * locals.var_dnm_dn17)) } } else { (assign5020_e3347 * (assign5020_e3346 * (locals.var_dnm_dn17 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign5020_e3349;
        locals.var_dnm_dn0 = assign5020_e3349_d_n0;
        locals.var_dnm_dn2 = assign5020_e3349_d_n2;
        locals.var_dnm_dn6 = assign5020_e3349_d_n6;
        locals.var_dnm_dn7 = assign5020_e3349_d_n7;
        locals.var_dnm_dn10 = assign5020_e3349_d_n10;
        locals.var_dnm_dn11 = assign5020_e3349_d_n11;
        locals.var_dnm_dn12 = assign5020_e3349_d_n12;
        locals.var_dnm_dn17 = assign5020_e3349_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign5030_e3355, assign5030_e3355_d_n0, assign5030_e3355_d_n2, assign5030_e3355_d_n6, assign5030_e3355_d_n7, assign5030_e3355_d_n10, assign5030_e3355_d_n11, assign5030_e3355_d_n12, assign5030_e3355_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign5030_e3353: f64 = (1.0 / locals.var_dnm);
        (assign5030_e3353, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn12 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn17 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12, locals.var_dnm_dn17,)
    }
};
        locals.var_dnm = assign5030_e3355;
        locals.var_dnm_dn0 = assign5030_e3355_d_n0;
        locals.var_dnm_dn2 = assign5030_e3355_d_n2;
        locals.var_dnm_dn6 = assign5030_e3355_d_n6;
        locals.var_dnm_dn7 = assign5030_e3355_d_n7;
        locals.var_dnm_dn10 = assign5030_e3355_d_n10;
        locals.var_dnm_dn11 = assign5030_e3355_d_n11;
        locals.var_dnm_dn12 = assign5030_e3355_d_n12;
        locals.var_dnm_dn17 = assign5030_e3355_d_n17;
        locals.var_dnm_rv = 0.0;

        let (assign5040_e3363, assign5040_e3363_d_n0, assign5040_e3363_d_n2, assign5040_e3363_d_n6, assign5040_e3363_d_n7, assign5040_e3363_d_n10, assign5040_e3363_d_n11, assign5040_e3363_d_n12, assign5040_e3363_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign5040_e3359: f64 = (locals.var_t2 * locals.var_t3);
        let assign5040_e3361: f64 = (assign5040_e3359 * locals.var_dnm);
        (assign5040_e3361, ((((locals.var_t2_dn0 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn0)) * locals.var_dnm) + (assign5040_e3359 * locals.var_dnm_dn0)), ((((locals.var_t2_dn2 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn2)) * locals.var_dnm) + (assign5040_e3359 * locals.var_dnm_dn2)), ((((locals.var_t2_dn6 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn6)) * locals.var_dnm) + (assign5040_e3359 * locals.var_dnm_dn6)), ((((locals.var_t2_dn7 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn7)) * locals.var_dnm) + (assign5040_e3359 * locals.var_dnm_dn7)), ((((locals.var_t2_dn10 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn10)) * locals.var_dnm) + (assign5040_e3359 * locals.var_dnm_dn10)), ((((locals.var_t2_dn11 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn11)) * locals.var_dnm) + (assign5040_e3359 * locals.var_dnm_dn11)), ((((locals.var_t2_dn12 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn12)) * locals.var_dnm) + (assign5040_e3359 * locals.var_dnm_dn12)), ((((locals.var_t2_dn17 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn17)) * locals.var_dnm) + (assign5040_e3359 * locals.var_dnm_dn17)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign5040_e3363;
        locals.var_t4_dn0 = assign5040_e3363_d_n0;
        locals.var_t4_dn2 = assign5040_e3363_d_n2;
        locals.var_t4_dn6 = assign5040_e3363_d_n6;
        locals.var_t4_dn7 = assign5040_e3363_d_n7;
        locals.var_t4_dn10 = assign5040_e3363_d_n10;
        locals.var_t4_dn11 = assign5040_e3363_d_n11;
        locals.var_t4_dn12 = assign5040_e3363_d_n12;
        locals.var_t4_dn17 = assign5040_e3363_d_n17;
        locals.var_t4_rv = 0.0;

        let (assign5050_e3373, assign5050_e3373_d_n0, assign5050_e3373_d_n2, assign5050_e3373_d_n6, assign5050_e3373_d_n7, assign5050_e3373_d_n10, assign5050_e3373_d_n11, assign5050_e3373_d_n12, assign5050_e3373_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign5050_e3367: f64 = (locals.var_t3 * locals.var_xmp);
        let assign5050_e3369: f64 = (assign5050_e3367 * locals.var_dnm);
        let assign5050_e3371: f64 = (assign5050_e3369 / locals.var_arg);
        (assign5050_e3371, (((((((locals.var_t3_dn0 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign5050_e3367 * locals.var_dnm_dn0)) * locals.var_arg) - (assign5050_e3369 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t3_dn2 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign5050_e3367 * locals.var_dnm_dn2)) * locals.var_arg) - (assign5050_e3369 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t3_dn6 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign5050_e3367 * locals.var_dnm_dn6)) * locals.var_arg) - (assign5050_e3369 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t3_dn7 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign5050_e3367 * locals.var_dnm_dn7)) * locals.var_arg) - (assign5050_e3369 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t3_dn10 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign5050_e3367 * locals.var_dnm_dn10)) * locals.var_arg) - (assign5050_e3369 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t3_dn11 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign5050_e3367 * locals.var_dnm_dn11)) * locals.var_arg) - (assign5050_e3369 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t3_dn12 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn12)) * locals.var_dnm) + (assign5050_e3367 * locals.var_dnm_dn12)) * locals.var_arg) - (assign5050_e3369 * locals.var_arg_dn12)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t3_dn17 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn17)) * locals.var_dnm) + (assign5050_e3367 * locals.var_dnm_dn17)) * locals.var_arg) - (assign5050_e3369 * locals.var_arg_dn17)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn17,)
    }
};
        locals.var_t8 = assign5050_e3373;
        locals.var_t8_dn0 = assign5050_e3373_d_n0;
        locals.var_t8_dn2 = assign5050_e3373_d_n2;
        locals.var_t8_dn6 = assign5050_e3373_d_n6;
        locals.var_t8_dn7 = assign5050_e3373_d_n7;
        locals.var_t8_dn10 = assign5050_e3373_d_n10;
        locals.var_t8_dn11 = assign5050_e3373_d_n11;
        locals.var_t8_dn12 = assign5050_e3373_d_n12;
        locals.var_t8_dn17 = assign5050_e3373_d_n17;
        locals.var_t8_rv = 0.0;

        let (assign5060_e3379, assign5060_e3379_d_n0, assign5060_e3379_d_n2, assign5060_e3379_d_n6, assign5060_e3379_d_n7, assign5060_e3379_d_n10, assign5060_e3379_d_n11, assign5060_e3379_d_n12, assign5060_e3379_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        let assign5060_e3377: f64 = (locals.var_vbs_bnd + locals.var_t4);
        (assign5060_e3377, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    } else {
        (locals.var_vbsc, locals.var_vbsc_dn0, locals.var_vbsc_dn2, locals.var_vbsc_dn6, locals.var_vbsc_dn7, locals.var_vbsc_dn10, locals.var_vbsc_dn11, locals.var_vbsc_dn12, locals.var_vbsc_dn17,)
    }
};
        locals.var_vbsc = assign5060_e3379;
        locals.var_vbsc_dn0 = assign5060_e3379_d_n0;
        locals.var_vbsc_dn2 = assign5060_e3379_d_n2;
        locals.var_vbsc_dn6 = assign5060_e3379_d_n6;
        locals.var_vbsc_dn7 = assign5060_e3379_d_n7;
        locals.var_vbsc_dn10 = assign5060_e3379_d_n10;
        locals.var_vbsc_dn11 = assign5060_e3379_d_n11;
        locals.var_vbsc_dn12 = assign5060_e3379_d_n12;
        locals.var_vbsc_dn17 = assign5060_e3379_d_n17;
        locals.var_vbsc_rv = 0.0;

        let (assign5070_e3383, assign5070_e3383_d_n0, assign5070_e3383_d_n2, assign5070_e3383_d_n6, assign5070_e3383_d_n7, assign5070_e3383_d_n10, assign5070_e3383_d_n11, assign5070_e3383_d_n12, assign5070_e3383_d_n17,) = {
    if (locals.var_guard48 != 0.0) {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn17,)
    } else {
        (locals.var_vbsc_dvbse, locals.var_vbsc_dvbse_dn0, locals.var_vbsc_dvbse_dn2, locals.var_vbsc_dvbse_dn6, locals.var_vbsc_dvbse_dn7, locals.var_vbsc_dvbse_dn10, locals.var_vbsc_dvbse_dn11, locals.var_vbsc_dvbse_dn12, locals.var_vbsc_dvbse_dn17,)
    }
};
        locals.var_vbsc_dvbse = assign5070_e3383;
        locals.var_vbsc_dvbse_dn0 = assign5070_e3383_d_n0;
        locals.var_vbsc_dvbse_dn2 = assign5070_e3383_d_n2;
        locals.var_vbsc_dvbse_dn6 = assign5070_e3383_d_n6;
        locals.var_vbsc_dvbse_dn7 = assign5070_e3383_d_n7;
        locals.var_vbsc_dvbse_dn10 = assign5070_e3383_d_n10;
        locals.var_vbsc_dvbse_dn11 = assign5070_e3383_d_n11;
        locals.var_vbsc_dvbse_dn12 = assign5070_e3383_d_n12;
        locals.var_vbsc_dvbse_dn17 = assign5070_e3383_d_n17;
        locals.var_vbsc_dvbse_rv = 0.0;

        let (assign5080_e3388, assign5080_e3388_d_n0, assign5080_e3388_d_n2, assign5080_e3388_d_n6, assign5080_e3388_d_n7, assign5080_e3388_d_n10, assign5080_e3388_d_n11, assign5080_e3388_d_n12, assign5080_e3388_d_n17,) = {
    if (locals.var_guard48 == 0.0) {
        (locals.var_vbs, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
    } else {
        (locals.var_vbsc, locals.var_vbsc_dn0, locals.var_vbsc_dn2, locals.var_vbsc_dn6, locals.var_vbsc_dn7, locals.var_vbsc_dn10, locals.var_vbsc_dn11, locals.var_vbsc_dn12, locals.var_vbsc_dn17,)
    }
};
        locals.var_vbsc = assign5080_e3388;
        locals.var_vbsc_dn0 = assign5080_e3388_d_n0;
        locals.var_vbsc_dn2 = assign5080_e3388_d_n2;
        locals.var_vbsc_dn6 = assign5080_e3388_d_n6;
        locals.var_vbsc_dn7 = assign5080_e3388_d_n7;
        locals.var_vbsc_dn10 = assign5080_e3388_d_n10;
        locals.var_vbsc_dn11 = assign5080_e3388_d_n11;
        locals.var_vbsc_dn12 = assign5080_e3388_d_n12;
        locals.var_vbsc_dn17 = assign5080_e3388_d_n17;
        locals.var_vbsc_rv = 0.0;

        let (assign5090_e3393, assign5090_e3393_d_n0, assign5090_e3393_d_n2, assign5090_e3393_d_n6, assign5090_e3393_d_n7, assign5090_e3393_d_n10, assign5090_e3393_d_n11, assign5090_e3393_d_n12, assign5090_e3393_d_n17,) = {
    if (locals.var_guard48 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
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
        locals.var_vbsc_dvbse_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5100_e3399, assign5100_e3399_d_n0, assign5100_e3399_d_n2, assign5100_e3399_d_n6, assign5100_e3399_d_n7, assign5100_e3399_d_n10, assign5100_e3399_d_n11, assign5100_e3399_d_n12, assign5100_e3399_d_n17,) = {
    if (locals.var_vds > 20.0) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn12, locals.var_vds_dn17,)
    }
};
        locals.var_vdsc = assign5100_e3399;
        locals.var_vdsc_dn0 = assign5100_e3399_d_n0;
        locals.var_vdsc_dn2 = assign5100_e3399_d_n2;
        locals.var_vdsc_dn6 = assign5100_e3399_d_n6;
        locals.var_vdsc_dn7 = assign5100_e3399_d_n7;
        locals.var_vdsc_dn10 = assign5100_e3399_d_n10;
        locals.var_vdsc_dn11 = assign5100_e3399_d_n11;
        locals.var_vdsc_dn12 = assign5100_e3399_d_n12;
        locals.var_vdsc_dn17 = assign5100_e3399_d_n17;
        locals.var_vdsc_rv = 0.0;

        let (assign5110_e3405, assign5110_e3405_d_n6, assign5110_e3405_d_n7, assign5110_e3405_d_n11,) = {
    if (locals.var_vgs > 20.0) {
        (20.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn11,)
    }
};
        locals.var_vgsc = assign5110_e3405;
        locals.var_vgsc_dn6 = assign5110_e3405_d_n6;
        locals.var_vgsc_dn7 = assign5110_e3405_d_n7;
        locals.var_vgsc_dn11 = assign5110_e3405_d_n11;
        locals.var_vgsc_rv = 0.0;

        let assign5120_e3408: f64 = (-20.0);
        let (assign5120_e3413, assign5120_e3413_d_n6, assign5120_e3413_d_n7, assign5120_e3413_d_n11,) = {
    if (locals.var_vgs < assign5120_e3408) {
        let assign5120_e3411: f64 = (-20.0);
        (assign5120_e3411, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgsc, locals.var_vgsc_dn6, locals.var_vgsc_dn7, locals.var_vgsc_dn11,)
    }
};
        locals.var_vgsc = assign5120_e3413;
        locals.var_vgsc_dn6 = assign5120_e3413_d_n6;
        locals.var_vgsc_dn7 = assign5120_e3413_d_n7;
        locals.var_vgsc_dn11 = assign5120_e3413_d_n11;
        locals.var_vgsc_rv = 0.0;

        let assign5130_e3416: f64 = (-20.0);
        let (assign5130_e3421, assign5130_e3421_d_n0, assign5130_e3421_d_n2, assign5130_e3421_d_n6, assign5130_e3421_d_n7, assign5130_e3421_d_n10, assign5130_e3421_d_n11, assign5130_e3421_d_n12, assign5130_e3421_d_n17,) = {
    if (locals.var_vbsc < assign5130_e3416) {
        let assign5130_e3419: f64 = (-20.0);
        (assign5130_e3419, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbsc, locals.var_vbsc_dn0, locals.var_vbsc_dn2, locals.var_vbsc_dn6, locals.var_vbsc_dn7, locals.var_vbsc_dn10, locals.var_vbsc_dn11, locals.var_vbsc_dn12, locals.var_vbsc_dn17,)
    }
};
        locals.var_vbsc = assign5130_e3421;
        locals.var_vbsc_dn0 = assign5130_e3421_d_n0;
        locals.var_vbsc_dn2 = assign5130_e3421_d_n2;
        locals.var_vbsc_dn6 = assign5130_e3421_d_n6;
        locals.var_vbsc_dn7 = assign5130_e3421_d_n7;
        locals.var_vbsc_dn10 = assign5130_e3421_d_n10;
        locals.var_vbsc_dn11 = assign5130_e3421_d_n11;
        locals.var_vbsc_dn12 = assign5130_e3421_d_n12;
        locals.var_vbsc_dn17 = assign5130_e3421_d_n17;
        locals.var_vbsc_rv = 0.0;

        locals.var_vds = locals.var_vdsc;
        locals.var_vds_dn0 = locals.var_vdsc_dn0;
        locals.var_vds_dn2 = locals.var_vdsc_dn2;
        locals.var_vds_dn6 = locals.var_vdsc_dn6;
        locals.var_vds_dn7 = locals.var_vdsc_dn7;
        locals.var_vds_dn10 = locals.var_vdsc_dn10;
        locals.var_vds_dn11 = locals.var_vdsc_dn11;
        locals.var_vds_dn12 = locals.var_vdsc_dn12;
        locals.var_vds_dn17 = locals.var_vdsc_dn17;
        locals.var_vds_rv = 0.0;

        locals.var_vgs = locals.var_vgsc;
        locals.var_vgs_dn6 = locals.var_vgsc_dn6;
        locals.var_vgs_dn7 = locals.var_vgsc_dn7;
        locals.var_vgs_dn11 = locals.var_vgsc_dn11;
        locals.var_vgs_rv = 0.0;

        locals.var_vbs = locals.var_vbsc;
        locals.var_vbs_dn0 = locals.var_vbsc_dn0;
        locals.var_vbs_dn2 = locals.var_vbsc_dn2;
        locals.var_vbs_dn6 = locals.var_vbsc_dn6;
        locals.var_vbs_dn7 = locals.var_vbsc_dn7;
        locals.var_vbs_dn10 = locals.var_vbsc_dn10;
        locals.var_vbs_dn11 = locals.var_vbsc_dn11;
        locals.var_vbs_dn12 = locals.var_vbsc_dn12;
        locals.var_vbs_dn17 = locals.var_vbsc_dn17;
        locals.var_vbs_rv = 0.0;

        locals.var_flg_pprv = 0.0;
        locals.var_flg_pprv_rv = 0.0;

        locals.var_pss0_ini = 0.0;
        locals.var_pss0_ini_rv = 0.0;

        locals.var_pbs0_ini = 0.0;
        locals.var_pbs0_ini_rv = 0.0;

        locals.var_psb0_ini = 0.0;
        locals.var_psb0_ini_rv = 0.0;

        locals.var_pssl_ini = 0.0;
        locals.var_pssl_ini_rv = 0.0;

        locals.var_pbsl_ini = 0.0;
        locals.var_pbsl_ini_rv = 0.0;

        locals.var_psbl_ini = 0.0;
        locals.var_psbl_ini_rv = 0.0;

        locals.var_ai = 0.0;
        locals.var_ai_dn0 = 0.0;
        locals.var_ai_dn2 = 0.0;
        locals.var_ai_dn6 = 0.0;
        locals.var_ai_dn7 = 0.0;
        locals.var_ai_dn10 = 0.0;
        locals.var_ai_dn11 = 0.0;
        locals.var_ai_dn12 = 0.0;
        locals.var_ai_dn17 = 0.0;
        locals.var_ai_rv = 0.0;

        locals.var_db = 0.0;
        locals.var_db_dn0 = 0.0;
        locals.var_db_dn2 = 0.0;
        locals.var_db_dn6 = 0.0;
        locals.var_db_dn7 = 0.0;
        locals.var_db_dn10 = 0.0;
        locals.var_db_dn11 = 0.0;
        locals.var_db_dn12 = 0.0;
        locals.var_db_dn17 = 0.0;
        locals.var_db_rv = 0.0;

        locals.var_di = 0.0;
        locals.var_di_dn0 = 0.0;
        locals.var_di_dn2 = 0.0;
        locals.var_di_dn6 = 0.0;
        locals.var_di_dn7 = 0.0;
        locals.var_di_dn10 = 0.0;
        locals.var_di_dn11 = 0.0;
        locals.var_di_dn12 = 0.0;
        locals.var_di_dn17 = 0.0;
        locals.var_di_rv = 0.0;

        locals.var_c2 = 0.0;
        locals.var_c2_dn0 = 0.0;
        locals.var_c2_dn2 = 0.0;
        locals.var_c2_dn6 = 0.0;
        locals.var_c2_dn7 = 0.0;
        locals.var_c2_dn10 = 0.0;
        locals.var_c2_dn11 = 0.0;
        locals.var_c2_dn12 = 0.0;
        locals.var_c2_dn17 = 0.0;
        locals.var_c2_rv = 0.0;

        locals.var_lp_s0 = 0.0;
        locals.var_lp_s0_rv = 0.0;

        locals.var_lp_sl = 0.0;
        locals.var_lp_sl_rv = 0.0;

        let assign5300_e3440: f64 = (locals.var_vbsc_dvbse * locals.var_vds);
        let assign5300_e3442: f64 = (assign5300_e3440 / 2.0);
        locals.var_t1__blk54 = assign5300_e3442;
        locals.var_t1__blk54_dn0 = (((locals.var_vbsc_dvbse_dn0 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn0)) / 2.0);
        locals.var_t1__blk54_dn2 = (((locals.var_vbsc_dvbse_dn2 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn2)) / 2.0);
        locals.var_t1__blk54_dn6 = (((locals.var_vbsc_dvbse_dn6 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn6)) / 2.0);
        locals.var_t1__blk54_dn7 = (((locals.var_vbsc_dvbse_dn7 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn7)) / 2.0);
        locals.var_t1__blk54_dn10 = (((locals.var_vbsc_dvbse_dn10 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn10)) / 2.0);
        locals.var_t1__blk54_dn11 = (((locals.var_vbsc_dvbse_dn11 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn11)) / 2.0);
        locals.var_t1__blk54_dn12 = (((locals.var_vbsc_dvbse_dn12 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn12)) / 2.0);
        locals.var_t1__blk54_dn17 = (((locals.var_vbsc_dvbse_dn17 * locals.var_vds) + (locals.var_vbsc_dvbse * locals.var_vds_dn17)) / 2.0);
        locals.var_t1__blk54_rv = 0.0;

        let assign5310_e3445: f64 = (2.0 * locals.var_t1__blk54);
        let assign5310_e3447: f64 = (assign5310_e3445 / p.p226);
        locals.var_tmf1 = assign5310_e3447;
        locals.var_tmf1_dn0 = ((2.0 * locals.var_t1__blk54_dn0) / p.p226);
        locals.var_tmf1_dn2 = ((2.0 * locals.var_t1__blk54_dn2) / p.p226);
        locals.var_tmf1_dn6 = ((2.0 * locals.var_t1__blk54_dn6) / p.p226);
        locals.var_tmf1_dn7 = ((2.0 * locals.var_t1__blk54_dn7) / p.p226);
        locals.var_tmf1_dn10 = ((2.0 * locals.var_t1__blk54_dn10) / p.p226);
        locals.var_tmf1_dn11 = ((2.0 * locals.var_t1__blk54_dn11) / p.p226);
        locals.var_tmf1_dn12 = ((2.0 * locals.var_t1__blk54_dn12) / p.p226);
        locals.var_tmf1_dn17 = ((2.0 * locals.var_t1__blk54_dn17) / p.p226);
        locals.var_tmf1_rv = 0.0;

        let assign5320_e3452: f64 = (1.0 / 2.0);
        let assign5320_e3456: f64 = (1.0 / 6.0);
        let assign5320_e3460: f64 = (1.0 / 24.0);
        let assign5320_e3464: f64 = (1.0 / 120.0);
        let assign5320_e3468: f64 = (1.0 / 720.0);
        let assign5320_e3472: f64 = (1.0 / 5040.0);
        let assign5320_e3473: f64 = (locals.var_tmf1 * assign5320_e3472);
        let assign5320_e3474: f64 = (assign5320_e3468 + assign5320_e3473);
        let assign5320_e3475: f64 = (locals.var_tmf1 * assign5320_e3474);
        let assign5320_e3476: f64 = (assign5320_e3464 + assign5320_e3475);
        let assign5320_e3477: f64 = (locals.var_tmf1 * assign5320_e3476);
        let assign5320_e3478: f64 = (assign5320_e3460 + assign5320_e3477);
        let assign5320_e3479: f64 = (locals.var_tmf1 * assign5320_e3478);
        let assign5320_e3480: f64 = (assign5320_e3456 + assign5320_e3479);
        let assign5320_e3481: f64 = (locals.var_tmf1 * assign5320_e3480);
        let assign5320_e3482: f64 = (assign5320_e3452 + assign5320_e3481);
        let assign5320_e3483: f64 = (locals.var_tmf1 * assign5320_e3482);
        let assign5320_e3484: f64 = (1.0 + assign5320_e3483);
        locals.var_tmf2 = assign5320_e3484;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * assign5320_e3482) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign5320_e3480) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign5320_e3478) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign5320_e3476) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign5320_e3474) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign5320_e3472)))))))))));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * assign5320_e3482) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign5320_e3480) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign5320_e3478) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign5320_e3476) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign5320_e3474) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign5320_e3472)))))))))));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * assign5320_e3482) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign5320_e3480) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign5320_e3478) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign5320_e3476) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign5320_e3474) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign5320_e3472)))))))))));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * assign5320_e3482) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign5320_e3480) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign5320_e3478) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign5320_e3476) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign5320_e3474) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign5320_e3472)))))))))));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * assign5320_e3482) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign5320_e3480) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign5320_e3478) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign5320_e3476) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign5320_e3474) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign5320_e3472)))))))))));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * assign5320_e3482) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign5320_e3480) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign5320_e3478) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign5320_e3476) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign5320_e3474) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign5320_e3472)))))))))));
        locals.var_tmf2_dn12 = ((locals.var_tmf1_dn12 * assign5320_e3482) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign5320_e3480) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign5320_e3478) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign5320_e3476) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign5320_e3474) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign5320_e3472)))))))))));
        locals.var_tmf2_dn17 = ((locals.var_tmf1_dn17 * assign5320_e3482) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign5320_e3480) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign5320_e3478) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign5320_e3476) + (locals.var_tmf1 * ((locals.var_tmf1_dn17 * assign5320_e3474) + (locals.var_tmf1 * (locals.var_tmf1_dn17 * assign5320_e3472)))))))))));
        locals.var_tmf2_rv = 0.0;

        let assign5330_e3487: f64 = (p.p226 / locals.var_tmf2);
        locals.var_vzadd = assign5330_e3487;
        locals.var_vzadd_dn0 = (-((p.p226 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn2 = (-((p.p226 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn6 = (-((p.p226 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn7 = (-((p.p226 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn10 = (-((p.p226 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn11 = (-((p.p226 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn12 = (-((p.p226 * locals.var_tmf2_dn12) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn17 = (-((p.p226 * locals.var_tmf2_dn17) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_rv = 0.0;

        let assign5340_e3490: f64 = if locals.var_vzadd < 5e-12 { 1.0 } else { 0.0 };
        locals.var_guard55 = assign5340_e3490;
        locals.var_guard55_rv = 0.0;

        let (assign5350_e3494, assign5350_e3494_d_n0, assign5350_e3494_d_n2, assign5350_e3494_d_n6, assign5350_e3494_d_n7, assign5350_e3494_d_n10, assign5350_e3494_d_n11, assign5350_e3494_d_n12, assign5350_e3494_d_n17,) = {
    if (locals.var_guard55 != 0.0) {
        (5e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn12, locals.var_vzadd_dn17,)
    }
};
        locals.var_vzadd = assign5350_e3494;
        locals.var_vzadd_dn0 = assign5350_e3494_d_n0;
        locals.var_vzadd_dn2 = assign5350_e3494_d_n2;
        locals.var_vzadd_dn6 = assign5350_e3494_d_n6;
        locals.var_vzadd_dn7 = assign5350_e3494_d_n7;
        locals.var_vzadd_dn10 = assign5350_e3494_d_n10;
        locals.var_vzadd_dn11 = assign5350_e3494_d_n11;
        locals.var_vzadd_dn12 = assign5350_e3494_d_n12;
        locals.var_vzadd_dn17 = assign5350_e3494_d_n17;
        locals.var_vzadd_rv = 0.0;

        let assign5360_e3497: f64 = (locals.var_vbs + locals.var_vzadd);
        locals.var_vbsz = assign5360_e3497;
        locals.var_vbsz_dn0 = (locals.var_vbs_dn0 + locals.var_vzadd_dn0);
        locals.var_vbsz_dn2 = (locals.var_vbs_dn2 + locals.var_vzadd_dn2);
        locals.var_vbsz_dn6 = (locals.var_vbs_dn6 + locals.var_vzadd_dn6);
        locals.var_vbsz_dn7 = (locals.var_vbs_dn7 + locals.var_vzadd_dn7);
        locals.var_vbsz_dn10 = (locals.var_vbs_dn10 + locals.var_vzadd_dn10);
        locals.var_vbsz_dn11 = (locals.var_vbs_dn11 + locals.var_vzadd_dn11);
        locals.var_vbsz_dn12 = (locals.var_vbs_dn12 + locals.var_vzadd_dn12);
        locals.var_vbsz_dn17 = (locals.var_vbs_dn17 + locals.var_vzadd_dn17);
        locals.var_vbsz_rv = 0.0;

        let assign5370_e3501: f64 = (2.0 * locals.var_vzadd);
        let assign5370_e3502: f64 = (locals.var_vds + assign5370_e3501);
        locals.var_vdsz = assign5370_e3502;
        locals.var_vdsz_dn0 = (locals.var_vds_dn0 + (2.0 * locals.var_vzadd_dn0));
        locals.var_vdsz_dn2 = (locals.var_vds_dn2 + (2.0 * locals.var_vzadd_dn2));
        locals.var_vdsz_dn6 = (locals.var_vds_dn6 + (2.0 * locals.var_vzadd_dn6));
        locals.var_vdsz_dn7 = (locals.var_vds_dn7 + (2.0 * locals.var_vzadd_dn7));
        locals.var_vdsz_dn10 = (locals.var_vds_dn10 + (2.0 * locals.var_vzadd_dn10));
        locals.var_vdsz_dn11 = (locals.var_vds_dn11 + (2.0 * locals.var_vzadd_dn11));
        locals.var_vdsz_dn12 = (locals.var_vds_dn12 + (2.0 * locals.var_vzadd_dn12));
        locals.var_vdsz_dn17 = (locals.var_vds_dn17 + (2.0 * locals.var_vzadd_dn17));
        locals.var_vdsz_rv = 0.0;

        let assign5380_e3505: f64 = (locals.var_vgs + locals.var_vzadd);
        locals.var_vgsz = assign5380_e3505;
        locals.var_vgsz_dn0 = locals.var_vzadd_dn0;
        locals.var_vgsz_dn2 = locals.var_vzadd_dn2;
        locals.var_vgsz_dn6 = (locals.var_vgs_dn6 + locals.var_vzadd_dn6);
        locals.var_vgsz_dn7 = (locals.var_vgs_dn7 + locals.var_vzadd_dn7);
        locals.var_vgsz_dn10 = locals.var_vzadd_dn10;
        locals.var_vgsz_dn11 = (locals.var_vgs_dn11 + locals.var_vzadd_dn11);
        locals.var_vgsz_dn12 = locals.var_vzadd_dn12;
        locals.var_vgsz_dn17 = locals.var_vzadd_dn17;
        locals.var_vgsz_rv = 0.0;

        let assign5390_e3508: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard56 = assign5390_e3508;
        locals.var_guard56_rv = 0.0;

        let (assign5400_e3512, assign5400_e3512_d_n0, assign5400_e3512_d_n2, assign5400_e3512_d_n6, assign5400_e3512_d_n7, assign5400_e3512_d_n10, assign5400_e3512_d_n11, assign5400_e3512_d_n12, assign5400_e3512_d_n17,) = {
    if (locals.var_guard56 != 0.0) {
        (locals.var_vbs, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
    } else {
        (locals.var_vbsp, locals.var_vbsp_dn0, locals.var_vbsp_dn2, locals.var_vbsp_dn6, locals.var_vbsp_dn7, locals.var_vbsp_dn10, locals.var_vbsp_dn11, locals.var_vbsp_dn12, locals.var_vbsp_dn17,)
    }
};
        locals.var_vbsp = assign5400_e3512;
        locals.var_vbsp_dn0 = assign5400_e3512_d_n0;
        locals.var_vbsp_dn2 = assign5400_e3512_d_n2;
        locals.var_vbsp_dn6 = assign5400_e3512_d_n6;
        locals.var_vbsp_dn7 = assign5400_e3512_d_n7;
        locals.var_vbsp_dn10 = assign5400_e3512_d_n10;
        locals.var_vbsp_dn11 = assign5400_e3512_d_n11;
        locals.var_vbsp_dn12 = assign5400_e3512_d_n12;
        locals.var_vbsp_dn17 = assign5400_e3512_d_n17;
        locals.var_vbsp_rv = 0.0;

        let (assign5410_e3516, assign5410_e3516_d_n0, assign5410_e3516_d_n2, assign5410_e3516_d_n6, assign5410_e3516_d_n7, assign5410_e3516_d_n10, assign5410_e3516_d_n11, assign5410_e3516_d_n12, assign5410_e3516_d_n17,) = {
    if (locals.var_guard56 != 0.0) {
        (locals.var_vbsz, locals.var_vbsz_dn0, locals.var_vbsz_dn2, locals.var_vbsz_dn6, locals.var_vbsz_dn7, locals.var_vbsz_dn10, locals.var_vbsz_dn11, locals.var_vbsz_dn12, locals.var_vbsz_dn17,)
    } else {
        (locals.var_vbspz, locals.var_vbspz_dn0, locals.var_vbspz_dn2, locals.var_vbspz_dn6, locals.var_vbspz_dn7, locals.var_vbspz_dn10, locals.var_vbspz_dn11, locals.var_vbspz_dn12, locals.var_vbspz_dn17,)
    }
};
        locals.var_vbspz = assign5410_e3516;
        locals.var_vbspz_dn0 = assign5410_e3516_d_n0;
        locals.var_vbspz_dn2 = assign5410_e3516_d_n2;
        locals.var_vbspz_dn6 = assign5410_e3516_d_n6;
        locals.var_vbspz_dn7 = assign5410_e3516_d_n7;
        locals.var_vbspz_dn10 = assign5410_e3516_d_n10;
        locals.var_vbspz_dn11 = assign5410_e3516_d_n11;
        locals.var_vbspz_dn12 = assign5410_e3516_d_n12;
        locals.var_vbspz_dn17 = assign5410_e3516_d_n17;
        locals.var_vbspz_rv = 0.0;

        let (assign5420_e3526, assign5420_e3526_d_n0, assign5420_e3526_d_n2, assign5420_e3526_d_n6, assign5420_e3526_d_n7, assign5420_e3526_d_n10, assign5420_e3526_d_n11, assign5420_e3526_d_n12, assign5420_e3526_d_n17,) = {
    if (locals.var_guard56 == 0.0) {
        let (assign5420_e3524, assign5420_e3524_d_n0, assign5420_e3524_d_n2, assign5420_e3524_d_n6, assign5420_e3524_d_n7, assign5420_e3524_d_n10, assign5420_e3524_d_n11, assign5420_e3524_d_n12, assign5420_e3524_d_n17,) = {
            if (locals.var_subversion < 3.0) {
                (locals.var_vbs, locals.var_vbs_dn0, locals.var_vbs_dn2, locals.var_vbs_dn6, locals.var_vbs_dn7, locals.var_vbs_dn10, locals.var_vbs_dn11, locals.var_vbs_dn12, locals.var_vbs_dn17,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign5420_e3524, assign5420_e3524_d_n0, assign5420_e3524_d_n2, assign5420_e3524_d_n6, assign5420_e3524_d_n7, assign5420_e3524_d_n10, assign5420_e3524_d_n11, assign5420_e3524_d_n12, assign5420_e3524_d_n17,)
    } else {
        (locals.var_vbsp, locals.var_vbsp_dn0, locals.var_vbsp_dn2, locals.var_vbsp_dn6, locals.var_vbsp_dn7, locals.var_vbsp_dn10, locals.var_vbsp_dn11, locals.var_vbsp_dn12, locals.var_vbsp_dn17,)
    }
};
        locals.var_vbsp = assign5420_e3526;
        locals.var_vbsp_dn0 = assign5420_e3526_d_n0;
        locals.var_vbsp_dn2 = assign5420_e3526_d_n2;
        locals.var_vbsp_dn6 = assign5420_e3526_d_n6;
        locals.var_vbsp_dn7 = assign5420_e3526_d_n7;
        locals.var_vbsp_dn10 = assign5420_e3526_d_n10;
        locals.var_vbsp_dn11 = assign5420_e3526_d_n11;
        locals.var_vbsp_dn12 = assign5420_e3526_d_n12;
        locals.var_vbsp_dn17 = assign5420_e3526_d_n17;
        locals.var_vbsp_rv = 0.0;

        let (assign5430_e3536, assign5430_e3536_d_n0, assign5430_e3536_d_n2, assign5430_e3536_d_n6, assign5430_e3536_d_n7, assign5430_e3536_d_n10, assign5430_e3536_d_n11, assign5430_e3536_d_n12, assign5430_e3536_d_n17,) = {
    if (locals.var_guard56 == 0.0) {
        let (assign5430_e3534, assign5430_e3534_d_n0, assign5430_e3534_d_n2, assign5430_e3534_d_n6, assign5430_e3534_d_n7, assign5430_e3534_d_n10, assign5430_e3534_d_n11, assign5430_e3534_d_n12, assign5430_e3534_d_n17,) = {
            if (locals.var_subversion < 3.0) {
                (locals.var_vbsz, locals.var_vbsz_dn0, locals.var_vbsz_dn2, locals.var_vbsz_dn6, locals.var_vbsz_dn7, locals.var_vbsz_dn10, locals.var_vbsz_dn11, locals.var_vbsz_dn12, locals.var_vbsz_dn17,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign5430_e3534, assign5430_e3534_d_n0, assign5430_e3534_d_n2, assign5430_e3534_d_n6, assign5430_e3534_d_n7, assign5430_e3534_d_n10, assign5430_e3534_d_n11, assign5430_e3534_d_n12, assign5430_e3534_d_n17,)
    } else {
        (locals.var_vbspz, locals.var_vbspz_dn0, locals.var_vbspz_dn2, locals.var_vbspz_dn6, locals.var_vbspz_dn7, locals.var_vbspz_dn10, locals.var_vbspz_dn11, locals.var_vbspz_dn12, locals.var_vbspz_dn17,)
    }
};
        locals.var_vbspz = assign5430_e3536;
        locals.var_vbspz_dn0 = assign5430_e3536_d_n0;
        locals.var_vbspz_dn2 = assign5430_e3536_d_n2;
        locals.var_vbspz_dn6 = assign5430_e3536_d_n6;
        locals.var_vbspz_dn7 = assign5430_e3536_d_n7;
        locals.var_vbspz_dn10 = assign5430_e3536_d_n10;
        locals.var_vbspz_dn11 = assign5430_e3536_d_n11;
        locals.var_vbspz_dn12 = assign5430_e3536_d_n12;
        locals.var_vbspz_dn17 = assign5430_e3536_d_n17;
        locals.var_vbspz_rv = 0.0;

        let assign5440_e3539: f64 = (2.0 * locals.var_q_nsub);
        let assign5440_e3541: f64 = (assign5440_e3539 * 1.034943e-10);
        let assign5440_e3543: f64 = (assign5440_e3541 * locals.var_c_fox0_inv);
        let assign5440_e3545: f64 = (assign5440_e3543 * locals.var_c_fox0_inv);
        locals.var_t1__blk57 = assign5440_e3545;
        locals.var_t1__blk57_dn0 = ((((2.0 * locals.var_q_nsub_dn0) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1__blk57_dn2 = ((((2.0 * locals.var_q_nsub_dn2) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1__blk57_dn6 = ((((2.0 * locals.var_q_nsub_dn6) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1__blk57_dn7 = ((((2.0 * locals.var_q_nsub_dn7) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1__blk57_dn10 = ((((2.0 * locals.var_q_nsub_dn10) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1__blk57_dn11 = ((((2.0 * locals.var_q_nsub_dn11) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1__blk57_dn12 = ((((2.0 * locals.var_q_nsub_dn12) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1__blk57_dn17 = ((((2.0 * locals.var_q_nsub_dn17) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1__blk57_rv = 0.0;

        let assign5450_e3548: f64 = (locals.var_vgs - locals.var_vfb);
        locals.var_t2__blk58 = assign5450_e3548;
        locals.var_t2__blk58_dn6 = locals.var_vgs_dn6;
        locals.var_t2__blk58_dn7 = locals.var_vgs_dn7;
        locals.var_t2__blk58_dn11 = locals.var_vgs_dn11;
        locals.var_t2__blk58_rv = 0.0;

        let assign5460_e3552: f64 = (2.0 / locals.var_t1__blk57);
        let assign5460_e3555: f64 = (locals.var_t2__blk58 - locals.var_beta_inv);
        let assign5460_e3557: f64 = (assign5460_e3555 - locals.var_vbsp);
        let assign5460_e3558: f64 = (assign5460_e3552 * assign5460_e3557);
        let assign5460_e3559: f64 = (1.0 + assign5460_e3558);
        locals.var_t3__blk59 = assign5460_e3559;
        locals.var_t3__blk59_dn0 = (((-((2.0 * locals.var_t1__blk57_dn0) / (locals.var_t1__blk57 * locals.var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * (-locals.var_vbsp_dn0)));
        locals.var_t3__blk59_dn2 = (((-((2.0 * locals.var_t1__blk57_dn2) / (locals.var_t1__blk57 * locals.var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * (-locals.var_vbsp_dn2)));
        locals.var_t3__blk59_dn6 = (((-((2.0 * locals.var_t1__blk57_dn6) / (locals.var_t1__blk57 * locals.var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * (locals.var_t2__blk58_dn6 - locals.var_vbsp_dn6)));
        locals.var_t3__blk59_dn7 = (((-((2.0 * locals.var_t1__blk57_dn7) / (locals.var_t1__blk57 * locals.var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * (locals.var_t2__blk58_dn7 - locals.var_vbsp_dn7)));
        locals.var_t3__blk59_dn10 = (((-((2.0 * locals.var_t1__blk57_dn10) / (locals.var_t1__blk57 * locals.var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * ((-locals.var_beta_inv_dn10) - locals.var_vbsp_dn10)));
        locals.var_t3__blk59_dn11 = (((-((2.0 * locals.var_t1__blk57_dn11) / (locals.var_t1__blk57 * locals.var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * (locals.var_t2__blk58_dn11 - locals.var_vbsp_dn11)));
        locals.var_t3__blk59_dn12 = (((-((2.0 * locals.var_t1__blk57_dn12) / (locals.var_t1__blk57 * locals.var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * (-locals.var_vbsp_dn12)));
        locals.var_t3__blk59_dn17 = (((-((2.0 * locals.var_t1__blk57_dn17) / (locals.var_t1__blk57 * locals.var_t1__blk57))) * assign5460_e3557) + (assign5460_e3552 * (-locals.var_vbsp_dn17)));
        locals.var_t3__blk59_rv = 0.0;

        let assign5470_e3562: f64 = (locals.var_t3__blk59 * locals.var_t3__blk59);
        let assign5470_e3565: f64 = (4.0 * 0.001);
        let assign5470_e3567: f64 = (assign5470_e3565 * 0.001);
        let assign5470_e3568: f64 = (assign5470_e3562 + assign5470_e3567);
        let assign5470_e3569: f64 = (assign5470_e3568).sqrt();
        locals.var_tmf1 = assign5470_e3569;
        locals.var_tmf1_dn0 = (((locals.var_t3__blk59_dn0 * locals.var_t3__blk59) + (locals.var_t3__blk59 * locals.var_t3__blk59_dn0)) / (2.0 * assign5470_e3569));
        locals.var_tmf1_dn2 = (((locals.var_t3__blk59_dn2 * locals.var_t3__blk59) + (locals.var_t3__blk59 * locals.var_t3__blk59_dn2)) / (2.0 * assign5470_e3569));
        locals.var_tmf1_dn6 = (((locals.var_t3__blk59_dn6 * locals.var_t3__blk59) + (locals.var_t3__blk59 * locals.var_t3__blk59_dn6)) / (2.0 * assign5470_e3569));
        locals.var_tmf1_dn7 = (((locals.var_t3__blk59_dn7 * locals.var_t3__blk59) + (locals.var_t3__blk59 * locals.var_t3__blk59_dn7)) / (2.0 * assign5470_e3569));
        locals.var_tmf1_dn10 = (((locals.var_t3__blk59_dn10 * locals.var_t3__blk59) + (locals.var_t3__blk59 * locals.var_t3__blk59_dn10)) / (2.0 * assign5470_e3569));
        locals.var_tmf1_dn11 = (((locals.var_t3__blk59_dn11 * locals.var_t3__blk59) + (locals.var_t3__blk59 * locals.var_t3__blk59_dn11)) / (2.0 * assign5470_e3569));
        locals.var_tmf1_dn12 = (((locals.var_t3__blk59_dn12 * locals.var_t3__blk59) + (locals.var_t3__blk59 * locals.var_t3__blk59_dn12)) / (2.0 * assign5470_e3569));
        locals.var_tmf1_dn17 = (((locals.var_t3__blk59_dn17 * locals.var_t3__blk59) + (locals.var_t3__blk59 * locals.var_t3__blk59_dn17)) / (2.0 * assign5470_e3569));
        locals.var_tmf1_rv = 0.0;

        let assign5480_e3573: f64 = (locals.var_t3__blk59 + locals.var_tmf1);
        let assign5480_e3574: f64 = (0.5 * assign5480_e3573);
        let assign5480_e3577: f64 = (1e-10 * 0.001);
        let assign5480_e3578: f64 = (assign5480_e3574 + assign5480_e3577);
        locals.var_t4 = assign5480_e3578;
        locals.var_t4_dn0 = (0.5 * (locals.var_t3__blk59_dn0 + locals.var_tmf1_dn0));
        locals.var_t4_dn2 = (0.5 * (locals.var_t3__blk59_dn2 + locals.var_tmf1_dn2));
        locals.var_t4_dn6 = (0.5 * (locals.var_t3__blk59_dn6 + locals.var_tmf1_dn6));
        locals.var_t4_dn7 = (0.5 * (locals.var_t3__blk59_dn7 + locals.var_tmf1_dn7));
        locals.var_t4_dn10 = (0.5 * (locals.var_t3__blk59_dn10 + locals.var_tmf1_dn10));
        locals.var_t4_dn11 = (0.5 * (locals.var_t3__blk59_dn11 + locals.var_tmf1_dn11));
        locals.var_t4_dn12 = (0.5 * (locals.var_t3__blk59_dn12 + locals.var_tmf1_dn12));
        locals.var_t4_dn17 = (0.5 * (locals.var_t3__blk59_dn17 + locals.var_tmf1_dn17));
        locals.var_t4_rv = 0.0;

        let assign5490_e3581: f64 = if locals.var_t4 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard61 = assign5490_e3581;
        locals.var_guard61_rv = 0.0;

        let (assign5500_e3585, assign5500_e3585_d_n0, assign5500_e3585_d_n2, assign5500_e3585_d_n6, assign5500_e3585_d_n7, assign5500_e3585_d_n10, assign5500_e3585_d_n11, assign5500_e3585_d_n12, assign5500_e3585_d_n17,) = {
    if (locals.var_guard61 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn17,)
    }
};
        locals.var_t4 = assign5500_e3585;
        locals.var_t4_dn0 = assign5500_e3585_d_n0;
        locals.var_t4_dn2 = assign5500_e3585_d_n2;
        locals.var_t4_dn6 = assign5500_e3585_d_n6;
        locals.var_t4_dn7 = assign5500_e3585_d_n7;
        locals.var_t4_dn10 = assign5500_e3585_d_n10;
        locals.var_t4_dn11 = assign5500_e3585_d_n11;
        locals.var_t4_dn12 = assign5500_e3585_d_n12;
        locals.var_t4_dn17 = assign5500_e3585_d_n17;
        locals.var_t4_rv = 0.0;

        let assign5510_e3588: f64 = (locals.var_t4 + 1e-50);
        let assign5510_e3589: f64 = (assign5510_e3588).sqrt();
        locals.var_tx__blk60 = assign5510_e3589;
        locals.var_tx__blk60_dn0 = (locals.var_t4_dn0 / (2.0 * assign5510_e3589));
        locals.var_tx__blk60_dn2 = (locals.var_t4_dn2 / (2.0 * assign5510_e3589));
        locals.var_tx__blk60_dn6 = (locals.var_t4_dn6 / (2.0 * assign5510_e3589));
        locals.var_tx__blk60_dn7 = (locals.var_t4_dn7 / (2.0 * assign5510_e3589));
        locals.var_tx__blk60_dn10 = (locals.var_t4_dn10 / (2.0 * assign5510_e3589));
        locals.var_tx__blk60_dn11 = (locals.var_t4_dn11 / (2.0 * assign5510_e3589));
        locals.var_tx__blk60_dn12 = (locals.var_t4_dn12 / (2.0 * assign5510_e3589));
        locals.var_tx__blk60_dn17 = (locals.var_t4_dn17 / (2.0 * assign5510_e3589));
        locals.var_tx__blk60_rv = 0.0;

        let assign5520_e3594: f64 = (1.0 - locals.var_tx__blk60);
        let assign5520_e3595: f64 = (locals.var_t1__blk57 * assign5520_e3594);
        let assign5520_e3596: f64 = (locals.var_t2__blk58 + assign5520_e3595);
        locals.var_pslsat = assign5520_e3596;
        locals.var_pslsat_dn0 = ((locals.var_t1__blk57_dn0 * assign5520_e3594) + (locals.var_t1__blk57 * (-locals.var_tx__blk60_dn0)));
        locals.var_pslsat_dn2 = ((locals.var_t1__blk57_dn2 * assign5520_e3594) + (locals.var_t1__blk57 * (-locals.var_tx__blk60_dn2)));
        locals.var_pslsat_dn6 = (locals.var_t2__blk58_dn6 + ((locals.var_t1__blk57_dn6 * assign5520_e3594) + (locals.var_t1__blk57 * (-locals.var_tx__blk60_dn6))));
        locals.var_pslsat_dn7 = (locals.var_t2__blk58_dn7 + ((locals.var_t1__blk57_dn7 * assign5520_e3594) + (locals.var_t1__blk57 * (-locals.var_tx__blk60_dn7))));
        locals.var_pslsat_dn10 = ((locals.var_t1__blk57_dn10 * assign5520_e3594) + (locals.var_t1__blk57 * (-locals.var_tx__blk60_dn10)));
        locals.var_pslsat_dn11 = (locals.var_t2__blk58_dn11 + ((locals.var_t1__blk57_dn11 * assign5520_e3594) + (locals.var_t1__blk57 * (-locals.var_tx__blk60_dn11))));
        locals.var_pslsat_dn12 = ((locals.var_t1__blk57_dn12 * assign5520_e3594) + (locals.var_t1__blk57 * (-locals.var_tx__blk60_dn12)));
        locals.var_pslsat_dn17 = ((locals.var_t1__blk57_dn17 * assign5520_e3594) + (locals.var_t1__blk57 * (-locals.var_tx__blk60_dn17)));
        locals.var_pslsat_rv = 0.0;

        let assign5530_e3599: f64 = (locals.var_pslsat - locals.var_pb2);
        locals.var_vdsats = assign5530_e3599;
        locals.var_vdsats_dn0 = (locals.var_pslsat_dn0 - locals.var_pb2_dn0);
        locals.var_vdsats_dn2 = (locals.var_pslsat_dn2 - locals.var_pb2_dn2);
        locals.var_vdsats_dn6 = (locals.var_pslsat_dn6 - locals.var_pb2_dn6);
        locals.var_vdsats_dn7 = (locals.var_pslsat_dn7 - locals.var_pb2_dn7);
        locals.var_vdsats_dn10 = (locals.var_pslsat_dn10 - locals.var_pb2_dn10);
        locals.var_vdsats_dn11 = (locals.var_pslsat_dn11 - locals.var_pb2_dn11);
        locals.var_vdsats_dn12 = (locals.var_pslsat_dn12 - locals.var_pb2_dn12);
        locals.var_vdsats_dn17 = (locals.var_pslsat_dn17 - locals.var_pb2_dn17);
        locals.var_vdsats_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign5540_e3602: f64 = (locals.var_vdsats - 0.1);
        let assign5540_e3604: f64 = (assign5540_e3602 - 0.05);
        locals.var_tmf1 = assign5540_e3604;
        locals.var_tmf1_dn0 = locals.var_vdsats_dn0;
        locals.var_tmf1_dn2 = locals.var_vdsats_dn2;
        locals.var_tmf1_dn6 = locals.var_vdsats_dn6;
        locals.var_tmf1_dn7 = locals.var_vdsats_dn7;
        locals.var_tmf1_dn10 = locals.var_vdsats_dn10;
        locals.var_tmf1_dn11 = locals.var_vdsats_dn11;
        locals.var_tmf1_dn12 = locals.var_vdsats_dn12;
        locals.var_tmf1_dn17 = locals.var_vdsats_dn17;
        locals.var_tmf1_rv = 0.0;

        let assign5550_e3607: f64 = (4.0 * 0.1);
        let assign5550_e3609: f64 = (assign5550_e3607 * 0.05);
        locals.var_tmf2 = assign5550_e3609;
        locals.var_tmf2_dn0 = 0.0;
        locals.var_tmf2_dn2 = 0.0;
        locals.var_tmf2_dn6 = 0.0;
        locals.var_tmf2_dn7 = 0.0;
        locals.var_tmf2_dn10 = 0.0;
        locals.var_tmf2_dn11 = 0.0;
        locals.var_tmf2_dn12 = 0.0;
        locals.var_tmf2_dn17 = 0.0;
        locals.var_tmf2_rv = 0.0;

        let (assign5560_e3616, assign5560_e3616_d_n0, assign5560_e3616_d_n2, assign5560_e3616_d_n6, assign5560_e3616_d_n7, assign5560_e3616_d_n10, assign5560_e3616_d_n11, assign5560_e3616_d_n12, assign5560_e3616_d_n17,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    } else {
        let assign5560_e3615: f64 = (-locals.var_tmf2);
        (assign5560_e3615, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
    }
};
        locals.var_tmf2 = assign5560_e3616;
        locals.var_tmf2_dn0 = assign5560_e3616_d_n0;
        locals.var_tmf2_dn2 = assign5560_e3616_d_n2;
        locals.var_tmf2_dn6 = assign5560_e3616_d_n6;
        locals.var_tmf2_dn7 = assign5560_e3616_d_n7;
        locals.var_tmf2_dn10 = assign5560_e3616_d_n10;
        locals.var_tmf2_dn11 = assign5560_e3616_d_n11;
        locals.var_tmf2_dn12 = assign5560_e3616_d_n12;
        locals.var_tmf2_dn17 = assign5560_e3616_d_n17;
        locals.var_tmf2_rv = 0.0;

        let assign5570_e3619: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5570_e3621: f64 = (assign5570_e3619 + locals.var_tmf2);
        let assign5570_e3622: f64 = (assign5570_e3621).sqrt();
        locals.var_tmf2 = assign5570_e3622;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5570_e3622));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5570_e3622));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign5570_e3622));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign5570_e3622));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign5570_e3622));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign5570_e3622));
        locals.var_tmf2_dn12 = ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign5570_e3622));
        locals.var_tmf2_dn17 = ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign5570_e3622));
        locals.var_tmf2_rv = 0.0;

        let assign5580_e3627: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5580_e3628: f64 = (0.5 * assign5580_e3627);
        let assign5580_e3629: f64 = (0.1 + assign5580_e3628);
        locals.var_vdsats = assign5580_e3629;
        locals.var_vdsats_dn0 = (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0));
        locals.var_vdsats_dn2 = (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2));
        locals.var_vdsats_dn6 = (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6));
        locals.var_vdsats_dn7 = (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7));
        locals.var_vdsats_dn10 = (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10));
        locals.var_vdsats_dn11 = (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11));
        locals.var_vdsats_dn12 = (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12));
        locals.var_vdsats_dn17 = (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17));
        locals.var_vdsats_rv = 0.0;

        let assign5590_e3632: f64 = (locals.var_vds / locals.var_vdsats);
        locals.var_t1__blk57 = assign5590_e3632;
        locals.var_t1__blk57_dn0 = (((locals.var_vds_dn0 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn0)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1__blk57_dn2 = (((locals.var_vds_dn2 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn2)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1__blk57_dn6 = (((locals.var_vds_dn6 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn6)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1__blk57_dn7 = (((locals.var_vds_dn7 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn7)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1__blk57_dn10 = (((locals.var_vds_dn10 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn10)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1__blk57_dn11 = (((locals.var_vds_dn11 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn11)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1__blk57_dn12 = (((locals.var_vds_dn12 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn12)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1__blk57_dn17 = (((locals.var_vds_dn17 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn17)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1__blk57_rv = 0.0;

        let assign5600_e3635: f64 = locals.var_t1__blk57;
        locals.var_tmf1 = assign5600_e3635;
        locals.var_tmf1_dn0 = locals.var_t1__blk57_dn0;
        locals.var_tmf1_dn2 = locals.var_t1__blk57_dn2;
        locals.var_tmf1_dn6 = locals.var_t1__blk57_dn6;
        locals.var_tmf1_dn7 = locals.var_t1__blk57_dn7;
        locals.var_tmf1_dn10 = locals.var_t1__blk57_dn10;
        locals.var_tmf1_dn11 = locals.var_t1__blk57_dn11;
        locals.var_tmf1_dn12 = locals.var_t1__blk57_dn12;
        locals.var_tmf1_dn17 = locals.var_t1__blk57_dn17;
        locals.var_tmf1_rv = 0.0;

        let assign5610_e3638: f64 = (locals.var_tmf1 * locals.var_tmf1);
        locals.var_tmf2 = assign5610_e3638;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11));
        locals.var_tmf2_dn12 = ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12));
        locals.var_tmf2_dn17 = ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17));
        locals.var_tmf2_rv = 0.0;

        let assign5620_e3641: f64 = (locals.var_tmf2 * locals.var_tmf1);
        locals.var_tmf3 = assign5620_e3641;
        locals.var_tmf3_dn0 = ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0));
        locals.var_tmf3_dn2 = ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2));
        locals.var_tmf3_dn6 = ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6));
        locals.var_tmf3_dn7 = ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7));
        locals.var_tmf3_dn10 = ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10));
        locals.var_tmf3_dn11 = ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11));
        locals.var_tmf3_dn12 = ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12));
        locals.var_tmf3_dn17 = ((locals.var_tmf2_dn17 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn17));
        locals.var_tmf3_rv = 0.0;

        let assign5630_e3644: f64 = (locals.var_tmf2 * locals.var_tmf2);
        locals.var_tmf4 = assign5630_e3644;
        locals.var_tmf4_dn0 = ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0));
        locals.var_tmf4_dn2 = ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2));
        locals.var_tmf4_dn6 = ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6));
        locals.var_tmf4_dn7 = ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7));
        locals.var_tmf4_dn10 = ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10));
        locals.var_tmf4_dn11 = ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11));
        locals.var_tmf4_dn12 = ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12));
        locals.var_tmf4_dn17 = ((locals.var_tmf2_dn17 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn17));
        locals.var_tmf4_rv = 0.0;

        let assign5640_e3648: f64 = (1.0 + locals.var_tmf1);
        let assign5640_e3650: f64 = (assign5640_e3648 + locals.var_tmf2);
        let assign5640_e3652: f64 = (assign5640_e3650 + locals.var_tmf3);
        let assign5640_e3654: f64 = (assign5640_e3652 + locals.var_tmf4);
        let assign5640_e3655: f64 = (1.0 / assign5640_e3654);
        locals.var_tx__blk60 = assign5640_e3655;
        locals.var_tx__blk60_dn0 = (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign5640_e3654 * assign5640_e3654)));
        locals.var_tx__blk60_dn2 = (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign5640_e3654 * assign5640_e3654)));
        locals.var_tx__blk60_dn6 = (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign5640_e3654 * assign5640_e3654)));
        locals.var_tx__blk60_dn7 = (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign5640_e3654 * assign5640_e3654)));
        locals.var_tx__blk60_dn10 = (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign5640_e3654 * assign5640_e3654)));
        locals.var_tx__blk60_dn11 = (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign5640_e3654 * assign5640_e3654)));
        locals.var_tx__blk60_dn12 = (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign5640_e3654 * assign5640_e3654)));
        locals.var_tx__blk60_dn17 = (-((((locals.var_tmf1_dn17 + locals.var_tmf2_dn17) + locals.var_tmf3_dn17) + locals.var_tmf4_dn17) / (assign5640_e3654 * assign5640_e3654)));
        locals.var_tx__blk60_rv = 0.0;

        let assign5650_e3659: f64 = (2.0 * locals.var_tmf1);
        let assign5650_e3660: f64 = (1.0 + assign5650_e3659);
        let assign5650_e3663: f64 = (3.0 * locals.var_tmf2);
        let assign5650_e3664: f64 = (assign5650_e3660 + assign5650_e3663);
        let assign5650_e3667: f64 = (4.0 * locals.var_tmf3);
        let assign5650_e3668: f64 = (assign5650_e3664 + assign5650_e3667);
        let assign5650_e3669: f64 = (-assign5650_e3668);
        let assign5650_e3671: f64 = (assign5650_e3669 * locals.var_tx__blk60);
        let assign5650_e3673: f64 = (assign5650_e3671 * locals.var_tx__blk60);
        locals.var_t0 = assign5650_e3673;
        locals.var_t0_dn0 = (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tx__blk60) + (assign5650_e3669 * locals.var_tx__blk60_dn0)) * locals.var_tx__blk60) + (assign5650_e3671 * locals.var_tx__blk60_dn0));
        locals.var_t0_dn2 = (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tx__blk60) + (assign5650_e3669 * locals.var_tx__blk60_dn2)) * locals.var_tx__blk60) + (assign5650_e3671 * locals.var_tx__blk60_dn2));
        locals.var_t0_dn6 = (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tx__blk60) + (assign5650_e3669 * locals.var_tx__blk60_dn6)) * locals.var_tx__blk60) + (assign5650_e3671 * locals.var_tx__blk60_dn6));
        locals.var_t0_dn7 = (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tx__blk60) + (assign5650_e3669 * locals.var_tx__blk60_dn7)) * locals.var_tx__blk60) + (assign5650_e3671 * locals.var_tx__blk60_dn7));
        locals.var_t0_dn10 = (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tx__blk60) + (assign5650_e3669 * locals.var_tx__blk60_dn10)) * locals.var_tx__blk60) + (assign5650_e3671 * locals.var_tx__blk60_dn10));
        locals.var_t0_dn11 = (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tx__blk60) + (assign5650_e3669 * locals.var_tx__blk60_dn11)) * locals.var_tx__blk60) + (assign5650_e3671 * locals.var_tx__blk60_dn11));
        locals.var_t0_dn12 = (((((-(((2.0 * locals.var_tmf1_dn12) + (3.0 * locals.var_tmf2_dn12)) + (4.0 * locals.var_tmf3_dn12))) * locals.var_tx__blk60) + (assign5650_e3669 * locals.var_tx__blk60_dn12)) * locals.var_tx__blk60) + (assign5650_e3671 * locals.var_tx__blk60_dn12));
        locals.var_t0_dn17 = (((((-(((2.0 * locals.var_tmf1_dn17) + (3.0 * locals.var_tmf2_dn17)) + (4.0 * locals.var_tmf3_dn17))) * locals.var_tx__blk60) + (assign5650_e3669 * locals.var_tx__blk60_dn17)) * locals.var_tx__blk60) + (assign5650_e3671 * locals.var_tx__blk60_dn17));
        locals.var_t0_rv = 0.0;

        let assign5660_e3677: f64 = (1.0 - locals.var_tx__blk60);
        let assign5660_e3678: f64 = assign5660_e3677;
        locals.var_tx__blk60 = assign5660_e3678;
        locals.var_tx__blk60_dn0 = (-locals.var_tx__blk60_dn0);
        locals.var_tx__blk60_dn2 = (-locals.var_tx__blk60_dn2);
        locals.var_tx__blk60_dn6 = (-locals.var_tx__blk60_dn6);
        locals.var_tx__blk60_dn7 = (-locals.var_tx__blk60_dn7);
        locals.var_tx__blk60_dn10 = (-locals.var_tx__blk60_dn10);
        locals.var_tx__blk60_dn11 = (-locals.var_tx__blk60_dn11);
        locals.var_tx__blk60_dn12 = (-locals.var_tx__blk60_dn12);
        locals.var_tx__blk60_dn17 = (-locals.var_tx__blk60_dn17);
        locals.var_tx__blk60_rv = 0.0;

        let assign5670_e3680: f64 = (-locals.var_t0);
        locals.var_t0 = assign5670_e3680;
        locals.var_t0_dn0 = (-locals.var_t0_dn0);
        locals.var_t0_dn2 = (-locals.var_t0_dn2);
        locals.var_t0_dn6 = (-locals.var_t0_dn6);
        locals.var_t0_dn7 = (-locals.var_t0_dn7);
        locals.var_t0_dn10 = (-locals.var_t0_dn10);
        locals.var_t0_dn11 = (-locals.var_t0_dn11);
        locals.var_t0_dn12 = (-locals.var_t0_dn12);
        locals.var_t0_dn17 = (-locals.var_t0_dn17);
        locals.var_t0_rv = 0.0;

        let assign5680_e3683: f64 = (locals.var_tx__blk60 * locals.var_tx__blk60);
        locals.var_fmdvds = assign5680_e3683;
        locals.var_fmdvds_dn0 = ((locals.var_tx__blk60_dn0 * locals.var_tx__blk60) + (locals.var_tx__blk60 * locals.var_tx__blk60_dn0));
        locals.var_fmdvds_dn2 = ((locals.var_tx__blk60_dn2 * locals.var_tx__blk60) + (locals.var_tx__blk60 * locals.var_tx__blk60_dn2));
        locals.var_fmdvds_dn6 = ((locals.var_tx__blk60_dn6 * locals.var_tx__blk60) + (locals.var_tx__blk60 * locals.var_tx__blk60_dn6));
        locals.var_fmdvds_dn7 = ((locals.var_tx__blk60_dn7 * locals.var_tx__blk60) + (locals.var_tx__blk60 * locals.var_tx__blk60_dn7));
        locals.var_fmdvds_dn10 = ((locals.var_tx__blk60_dn10 * locals.var_tx__blk60) + (locals.var_tx__blk60 * locals.var_tx__blk60_dn10));
        locals.var_fmdvds_dn11 = ((locals.var_tx__blk60_dn11 * locals.var_tx__blk60) + (locals.var_tx__blk60 * locals.var_tx__blk60_dn11));
        locals.var_fmdvds_dn12 = ((locals.var_tx__blk60_dn12 * locals.var_tx__blk60) + (locals.var_tx__blk60 * locals.var_tx__blk60_dn12));
        locals.var_fmdvds_dn17 = ((locals.var_tx__blk60_dn17 * locals.var_tx__blk60) + (locals.var_tx__blk60 * locals.var_tx__blk60_dn17));
        locals.var_fmdvds_rv = 0.0;

        let assign5690_e3694: f64 = if (((p.p204 == 0.0) && (p.p206 == 0.0)) || (p.p205 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard68 = assign5690_e3694;
        locals.var_guard68_rv = 0.0;

        let (assign5700_e3698,) = {
    if (locals.var_guard68 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign5700_e3698;
        locals.var_flg_qme_rv = 0.0;

        let (assign5710_e3703,) = {
    if (locals.var_guard68 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign5710_e3703;
        locals.var_flg_qme_rv = 0.0;

        let assign5720_e3706: f64 = (2.0 * locals.var_q_nsub);
        let assign5720_e3708: f64 = (assign5720_e3706 * 1.034943e-10);
        let assign5720_e3710: f64 = (assign5720_e3708 * locals.var_pb20);
        let assign5720_e3711: f64 = (assign5720_e3710).sqrt();
        locals.var_t2__blk62 = assign5720_e3711;
        locals.var_t2__blk62_dn0 = (((((2.0 * locals.var_q_nsub_dn0) * 1.034943e-10) * locals.var_pb20) + (assign5720_e3708 * locals.var_pb20_dn0)) / (2.0 * assign5720_e3711));
        locals.var_t2__blk62_dn2 = (((((2.0 * locals.var_q_nsub_dn2) * 1.034943e-10) * locals.var_pb20) + (assign5720_e3708 * locals.var_pb20_dn2)) / (2.0 * assign5720_e3711));
        locals.var_t2__blk62_dn6 = (((((2.0 * locals.var_q_nsub_dn6) * 1.034943e-10) * locals.var_pb20) + (assign5720_e3708 * locals.var_pb20_dn6)) / (2.0 * assign5720_e3711));
        locals.var_t2__blk62_dn7 = (((((2.0 * locals.var_q_nsub_dn7) * 1.034943e-10) * locals.var_pb20) + (assign5720_e3708 * locals.var_pb20_dn7)) / (2.0 * assign5720_e3711));
        locals.var_t2__blk62_dn10 = (((((2.0 * locals.var_q_nsub_dn10) * 1.034943e-10) * locals.var_pb20) + (assign5720_e3708 * locals.var_pb20_dn10)) / (2.0 * assign5720_e3711));
        locals.var_t2__blk62_dn11 = (((((2.0 * locals.var_q_nsub_dn11) * 1.034943e-10) * locals.var_pb20) + (assign5720_e3708 * locals.var_pb20_dn11)) / (2.0 * assign5720_e3711));
        locals.var_t2__blk62_dn12 = (((((2.0 * locals.var_q_nsub_dn12) * 1.034943e-10) * locals.var_pb20) + (assign5720_e3708 * locals.var_pb20_dn12)) / (2.0 * assign5720_e3711));
        locals.var_t2__blk62_dn17 = (((((2.0 * locals.var_q_nsub_dn17) * 1.034943e-10) * locals.var_pb20) + (assign5720_e3708 * locals.var_pb20_dn17)) / (2.0 * assign5720_e3711));
        locals.var_t2__blk62_rv = 0.0;

        let assign5730_e3714: f64 = (locals.var_pb20 + locals.var_vfb);
        let assign5730_e3717: f64 = (locals.var_t2__blk62 / locals.var_c_fox0);
        let assign5730_e3718: f64 = (assign5730_e3714 + assign5730_e3717);
        locals.var_vthq = assign5730_e3718;
        locals.var_vthq_dn0 = (locals.var_pb20_dn0 + (locals.var_t2__blk62_dn0 / locals.var_c_fox0));
        locals.var_vthq_dn2 = (locals.var_pb20_dn2 + (locals.var_t2__blk62_dn2 / locals.var_c_fox0));
        locals.var_vthq_dn6 = (locals.var_pb20_dn6 + (locals.var_t2__blk62_dn6 / locals.var_c_fox0));
        locals.var_vthq_dn7 = (locals.var_pb20_dn7 + (locals.var_t2__blk62_dn7 / locals.var_c_fox0));
        locals.var_vthq_dn10 = (locals.var_pb20_dn10 + (locals.var_t2__blk62_dn10 / locals.var_c_fox0));
        locals.var_vthq_dn11 = (locals.var_pb20_dn11 + (locals.var_t2__blk62_dn11 / locals.var_c_fox0));
        locals.var_vthq_dn12 = (locals.var_pb20_dn12 + (locals.var_t2__blk62_dn12 / locals.var_c_fox0));
        locals.var_vthq_dn17 = (locals.var_pb20_dn17 + (locals.var_t2__blk62_dn17 / locals.var_c_fox0));
        locals.var_vthq_rv = 0.0;

        let assign5740_e3721: f64 = if locals.var_flg_qme == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard69 = assign5740_e3721;
        locals.var_guard69_rv = 0.0;

        let (assign5750_e3725, assign5750_e3725_d_n0, assign5750_e3725_d_n2, assign5750_e3725_d_n6, assign5750_e3725_d_n7, assign5750_e3725_d_n10, assign5750_e3725_d_n11, assign5750_e3725_d_n12, assign5750_e3725_d_n17,) = {
    if (locals.var_guard69 != 0.0) {
        (locals.var_tfox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tfoxe, locals.var_tfoxe_dn0, locals.var_tfoxe_dn2, locals.var_tfoxe_dn6, locals.var_tfoxe_dn7, locals.var_tfoxe_dn10, locals.var_tfoxe_dn11, locals.var_tfoxe_dn12, locals.var_tfoxe_dn17,)
    }
};
        locals.var_tfoxe = assign5750_e3725;
        locals.var_tfoxe_dn0 = assign5750_e3725_d_n0;
        locals.var_tfoxe_dn2 = assign5750_e3725_d_n2;
        locals.var_tfoxe_dn6 = assign5750_e3725_d_n6;
        locals.var_tfoxe_dn7 = assign5750_e3725_d_n7;
        locals.var_tfoxe_dn10 = assign5750_e3725_d_n10;
        locals.var_tfoxe_dn11 = assign5750_e3725_d_n11;
        locals.var_tfoxe_dn12 = assign5750_e3725_d_n12;
        locals.var_tfoxe_dn17 = assign5750_e3725_d_n17;
        locals.var_tfoxe_rv = 0.0;

        let (assign5760_e3729, assign5760_e3729_d_n0, assign5760_e3729_d_n2, assign5760_e3729_d_n6, assign5760_e3729_d_n7, assign5760_e3729_d_n10, assign5760_e3729_d_n11, assign5760_e3729_d_n12, assign5760_e3729_d_n17,) = {
    if (locals.var_guard69 != 0.0) {
        (locals.var_c_fox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_c_fox, locals.var_c_fox_dn0, locals.var_c_fox_dn2, locals.var_c_fox_dn6, locals.var_c_fox_dn7, locals.var_c_fox_dn10, locals.var_c_fox_dn11, locals.var_c_fox_dn12, locals.var_c_fox_dn17,)
    }
};
        locals.var_c_fox = assign5760_e3729;
        locals.var_c_fox_dn0 = assign5760_e3729_d_n0;
        locals.var_c_fox_dn2 = assign5760_e3729_d_n2;
        locals.var_c_fox_dn6 = assign5760_e3729_d_n6;
        locals.var_c_fox_dn7 = assign5760_e3729_d_n7;
        locals.var_c_fox_dn10 = assign5760_e3729_d_n10;
        locals.var_c_fox_dn11 = assign5760_e3729_d_n11;
        locals.var_c_fox_dn12 = assign5760_e3729_d_n12;
        locals.var_c_fox_dn17 = assign5760_e3729_d_n17;
        locals.var_c_fox_rv = 0.0;

        let (assign5770_e3733, assign5770_e3733_d_n0, assign5770_e3733_d_n2, assign5770_e3733_d_n6, assign5770_e3733_d_n7, assign5770_e3733_d_n10, assign5770_e3733_d_n11, assign5770_e3733_d_n12, assign5770_e3733_d_n17,) = {
    if (locals.var_guard69 != 0.0) {
        (locals.var_c_fox0_inv, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_c_fox_inv, locals.var_c_fox_inv_dn0, locals.var_c_fox_inv_dn2, locals.var_c_fox_inv_dn6, locals.var_c_fox_inv_dn7, locals.var_c_fox_inv_dn10, locals.var_c_fox_inv_dn11, locals.var_c_fox_inv_dn12, locals.var_c_fox_inv_dn17,)
    }
};
        locals.var_c_fox_inv = assign5770_e3733;
        locals.var_c_fox_inv_dn0 = assign5770_e3733_d_n0;
        locals.var_c_fox_inv_dn2 = assign5770_e3733_d_n2;
        locals.var_c_fox_inv_dn6 = assign5770_e3733_d_n6;
        locals.var_c_fox_inv_dn7 = assign5770_e3733_d_n7;
        locals.var_c_fox_inv_dn10 = assign5770_e3733_d_n10;
        locals.var_c_fox_inv_dn11 = assign5770_e3733_d_n11;
        locals.var_c_fox_inv_dn12 = assign5770_e3733_d_n12;
        locals.var_c_fox_inv_dn17 = assign5770_e3733_d_n17;
        locals.var_c_fox_inv_rv = 0.0;

        let (assign5780_e3743, assign5780_e3743_d_n0, assign5780_e3743_d_n2, assign5780_e3743_d_n6, assign5780_e3743_d_n7, assign5780_e3743_d_n10, assign5780_e3743_d_n11, assign5780_e3743_d_n12, assign5780_e3743_d_n17,) = {
    if (locals.var_guard69 != 0.0) {
        let assign5780_e3737: f64 = (locals.var_cnst0soi * locals.var_c_fox0_inv);
        let assign5780_e3739: f64 = (assign5780_e3737 * locals.var_c_fox0_inv);
        let assign5780_e3741: f64 = (assign5780_e3739 * locals.var_cnst0soi);
        (assign5780_e3741, ((((locals.var_cnst0soi_dn0 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5780_e3739 * locals.var_cnst0soi_dn0)), ((((locals.var_cnst0soi_dn2 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5780_e3739 * locals.var_cnst0soi_dn2)), ((((locals.var_cnst0soi_dn6 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5780_e3739 * locals.var_cnst0soi_dn6)), ((((locals.var_cnst0soi_dn7 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5780_e3739 * locals.var_cnst0soi_dn7)), ((((locals.var_cnst0soi_dn10 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5780_e3739 * locals.var_cnst0soi_dn10)), ((((locals.var_cnst0soi_dn11 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5780_e3739 * locals.var_cnst0soi_dn11)), ((((locals.var_cnst0soi_dn12 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5780_e3739 * locals.var_cnst0soi_dn12)), ((((locals.var_cnst0soi_dn17 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv) * locals.var_cnst0soi) + (assign5780_e3739 * locals.var_cnst0soi_dn17)),)
    } else {
        (locals.var_cnstc_foxi, locals.var_cnstc_foxi_dn0, locals.var_cnstc_foxi_dn2, locals.var_cnstc_foxi_dn6, locals.var_cnstc_foxi_dn7, locals.var_cnstc_foxi_dn10, locals.var_cnstc_foxi_dn11, locals.var_cnstc_foxi_dn12, locals.var_cnstc_foxi_dn17,)
    }
};
        locals.var_cnstc_foxi = assign5780_e3743;
        locals.var_cnstc_foxi_dn0 = assign5780_e3743_d_n0;
        locals.var_cnstc_foxi_dn2 = assign5780_e3743_d_n2;
        locals.var_cnstc_foxi_dn6 = assign5780_e3743_d_n6;
        locals.var_cnstc_foxi_dn7 = assign5780_e3743_d_n7;
        locals.var_cnstc_foxi_dn10 = assign5780_e3743_d_n10;
        locals.var_cnstc_foxi_dn11 = assign5780_e3743_d_n11;
        locals.var_cnstc_foxi_dn12 = assign5780_e3743_d_n12;
        locals.var_cnstc_foxi_dn17 = assign5780_e3743_d_n17;
        locals.var_cnstc_foxi_rv = 0.0;

        let (assign5790_e3754, assign5790_e3754_d_n0, assign5790_e3754_d_n2, assign5790_e3754_d_n6, assign5790_e3754_d_n7, assign5790_e3754_d_n10, assign5790_e3754_d_n11, assign5790_e3754_d_n12, assign5790_e3754_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let assign5790_e3748: f64 = (locals.var_vgs - locals.var_vbsp);
        let assign5790_e3750: f64 = (assign5790_e3748 - locals.var_vthq);
        let assign5790_e3752: f64 = (assign5790_e3750 + p.p205);
        (assign5790_e3752, ((-locals.var_vbsp_dn0) - locals.var_vthq_dn0), ((-locals.var_vbsp_dn2) - locals.var_vthq_dn2), ((locals.var_vgs_dn6 - locals.var_vbsp_dn6) - locals.var_vthq_dn6), ((locals.var_vgs_dn7 - locals.var_vbsp_dn7) - locals.var_vthq_dn7), ((-locals.var_vbsp_dn10) - locals.var_vthq_dn10), ((locals.var_vgs_dn11 - locals.var_vbsp_dn11) - locals.var_vthq_dn11), ((-locals.var_vbsp_dn12) - locals.var_vthq_dn12), ((-locals.var_vbsp_dn17) - locals.var_vthq_dn17),)
    } else {
        (locals.var_t5__blk66, locals.var_t5__blk66_dn0, locals.var_t5__blk66_dn2, locals.var_t5__blk66_dn6, locals.var_t5__blk66_dn7, locals.var_t5__blk66_dn10, locals.var_t5__blk66_dn11, locals.var_t5__blk66_dn12, locals.var_t5__blk66_dn17,)
    }
};
        locals.var_t5__blk66 = assign5790_e3754;
        locals.var_t5__blk66_dn0 = assign5790_e3754_d_n0;
        locals.var_t5__blk66_dn2 = assign5790_e3754_d_n2;
        locals.var_t5__blk66_dn6 = assign5790_e3754_d_n6;
        locals.var_t5__blk66_dn7 = assign5790_e3754_d_n7;
        locals.var_t5__blk66_dn10 = assign5790_e3754_d_n10;
        locals.var_t5__blk66_dn11 = assign5790_e3754_d_n11;
        locals.var_t5__blk66_dn12 = assign5790_e3754_d_n12;
        locals.var_t5__blk66_dn17 = assign5790_e3754_d_n17;
        locals.var_t5__blk66_rv = 0.0;

        let (assign5800_e3768, assign5800_e3768_d_n0, assign5800_e3768_d_n2, assign5800_e3768_d_n6, assign5800_e3768_d_n7, assign5800_e3768_d_n10, assign5800_e3768_d_n11, assign5800_e3768_d_n12, assign5800_e3768_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let assign5800_e3759: f64 = (locals.var_t5__blk66 * locals.var_t5__blk66);
        let assign5800_e3762: f64 = (4.0 * 0.0001);
        let assign5800_e3764: f64 = (assign5800_e3762 * 0.0001);
        let assign5800_e3765: f64 = (assign5800_e3759 + assign5800_e3764);
        let assign5800_e3766: f64 = (assign5800_e3765).sqrt();
        (assign5800_e3766, (((locals.var_t5__blk66_dn0 * locals.var_t5__blk66) + (locals.var_t5__blk66 * locals.var_t5__blk66_dn0)) / (2.0 * assign5800_e3766)), (((locals.var_t5__blk66_dn2 * locals.var_t5__blk66) + (locals.var_t5__blk66 * locals.var_t5__blk66_dn2)) / (2.0 * assign5800_e3766)), (((locals.var_t5__blk66_dn6 * locals.var_t5__blk66) + (locals.var_t5__blk66 * locals.var_t5__blk66_dn6)) / (2.0 * assign5800_e3766)), (((locals.var_t5__blk66_dn7 * locals.var_t5__blk66) + (locals.var_t5__blk66 * locals.var_t5__blk66_dn7)) / (2.0 * assign5800_e3766)), (((locals.var_t5__blk66_dn10 * locals.var_t5__blk66) + (locals.var_t5__blk66 * locals.var_t5__blk66_dn10)) / (2.0 * assign5800_e3766)), (((locals.var_t5__blk66_dn11 * locals.var_t5__blk66) + (locals.var_t5__blk66 * locals.var_t5__blk66_dn11)) / (2.0 * assign5800_e3766)), (((locals.var_t5__blk66_dn12 * locals.var_t5__blk66) + (locals.var_t5__blk66 * locals.var_t5__blk66_dn12)) / (2.0 * assign5800_e3766)), (((locals.var_t5__blk66_dn17 * locals.var_t5__blk66) + (locals.var_t5__blk66 * locals.var_t5__blk66_dn17)) / (2.0 * assign5800_e3766)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign5800_e3768;
        locals.var_tmf1_dn0 = assign5800_e3768_d_n0;
        locals.var_tmf1_dn2 = assign5800_e3768_d_n2;
        locals.var_tmf1_dn6 = assign5800_e3768_d_n6;
        locals.var_tmf1_dn7 = assign5800_e3768_d_n7;
        locals.var_tmf1_dn10 = assign5800_e3768_d_n10;
        locals.var_tmf1_dn11 = assign5800_e3768_d_n11;
        locals.var_tmf1_dn12 = assign5800_e3768_d_n12;
        locals.var_tmf1_dn17 = assign5800_e3768_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign5810_e3781, assign5810_e3781_d_n0, assign5810_e3781_d_n2, assign5810_e3781_d_n6, assign5810_e3781_d_n7, assign5810_e3781_d_n10, assign5810_e3781_d_n11, assign5810_e3781_d_n12, assign5810_e3781_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let assign5810_e3774: f64 = (locals.var_t5__blk66 + locals.var_tmf1);
        let assign5810_e3775: f64 = (0.5 * assign5810_e3774);
        let assign5810_e3778: f64 = (1e-10 * 0.0001);
        let assign5810_e3779: f64 = (assign5810_e3775 + assign5810_e3778);
        (assign5810_e3779, (0.5 * (locals.var_t5__blk66_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_t5__blk66_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_t5__blk66_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_t5__blk66_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_t5__blk66_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_t5__blk66_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_t5__blk66_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_t5__blk66_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_t2__blk62, locals.var_t2__blk62_dn0, locals.var_t2__blk62_dn2, locals.var_t2__blk62_dn6, locals.var_t2__blk62_dn7, locals.var_t2__blk62_dn10, locals.var_t2__blk62_dn11, locals.var_t2__blk62_dn12, locals.var_t2__blk62_dn17,)
    }
};
        locals.var_t2__blk62 = assign5810_e3781;
        locals.var_t2__blk62_dn0 = assign5810_e3781_d_n0;
        locals.var_t2__blk62_dn2 = assign5810_e3781_d_n2;
        locals.var_t2__blk62_dn6 = assign5810_e3781_d_n6;
        locals.var_t2__blk62_dn7 = assign5810_e3781_d_n7;
        locals.var_t2__blk62_dn10 = assign5810_e3781_d_n10;
        locals.var_t2__blk62_dn11 = assign5810_e3781_d_n11;
        locals.var_t2__blk62_dn12 = assign5810_e3781_d_n12;
        locals.var_t2__blk62_dn17 = assign5810_e3781_d_n17;
        locals.var_t2__blk62_rv = 0.0;

        let assign5820_e3784: f64 = if locals.var_t2__blk62 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign5820_e3784;
        locals.var_guard70_rv = 0.0;

        let (assign5830_e3791, assign5830_e3791_d_n0, assign5830_e3791_d_n2, assign5830_e3791_d_n6, assign5830_e3791_d_n7, assign5830_e3791_d_n10, assign5830_e3791_d_n11, assign5830_e3791_d_n12, assign5830_e3791_d_n17,) = {
    if ((locals.var_guard69 == 0.0) && (locals.var_guard70 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk62, locals.var_t2__blk62_dn0, locals.var_t2__blk62_dn2, locals.var_t2__blk62_dn6, locals.var_t2__blk62_dn7, locals.var_t2__blk62_dn10, locals.var_t2__blk62_dn11, locals.var_t2__blk62_dn12, locals.var_t2__blk62_dn17,)
    }
};
        locals.var_t2__blk62 = assign5830_e3791;
        locals.var_t2__blk62_dn0 = assign5830_e3791_d_n0;
        locals.var_t2__blk62_dn2 = assign5830_e3791_d_n2;
        locals.var_t2__blk62_dn6 = assign5830_e3791_d_n6;
        locals.var_t2__blk62_dn7 = assign5830_e3791_d_n7;
        locals.var_t2__blk62_dn10 = assign5830_e3791_d_n10;
        locals.var_t2__blk62_dn11 = assign5830_e3791_d_n11;
        locals.var_t2__blk62_dn12 = assign5830_e3791_d_n12;
        locals.var_t2__blk62_dn17 = assign5830_e3791_d_n17;
        locals.var_t2__blk62_rv = 0.0;

        let (assign5840_e3798, assign5840_e3798_d_n0, assign5840_e3798_d_n2, assign5840_e3798_d_n6, assign5840_e3798_d_n7, assign5840_e3798_d_n10, assign5840_e3798_d_n11, assign5840_e3798_d_n12, assign5840_e3798_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let assign5840_e3796: f64 = (1.0 / locals.var_t2__blk62);
        (assign5840_e3796, (-(locals.var_t2__blk62_dn0 / (locals.var_t2__blk62 * locals.var_t2__blk62))), (-(locals.var_t2__blk62_dn2 / (locals.var_t2__blk62 * locals.var_t2__blk62))), (-(locals.var_t2__blk62_dn6 / (locals.var_t2__blk62 * locals.var_t2__blk62))), (-(locals.var_t2__blk62_dn7 / (locals.var_t2__blk62 * locals.var_t2__blk62))), (-(locals.var_t2__blk62_dn10 / (locals.var_t2__blk62 * locals.var_t2__blk62))), (-(locals.var_t2__blk62_dn11 / (locals.var_t2__blk62 * locals.var_t2__blk62))), (-(locals.var_t2__blk62_dn12 / (locals.var_t2__blk62 * locals.var_t2__blk62))), (-(locals.var_t2__blk62_dn17 / (locals.var_t2__blk62 * locals.var_t2__blk62))),)
    } else {
        (locals.var_t3__blk63, locals.var_t3__blk63_dn0, locals.var_t3__blk63_dn2, locals.var_t3__blk63_dn6, locals.var_t3__blk63_dn7, locals.var_t3__blk63_dn10, locals.var_t3__blk63_dn11, locals.var_t3__blk63_dn12, locals.var_t3__blk63_dn17,)
    }
};
        locals.var_t3__blk63 = assign5840_e3798;
        locals.var_t3__blk63_dn0 = assign5840_e3798_d_n0;
        locals.var_t3__blk63_dn2 = assign5840_e3798_d_n2;
        locals.var_t3__blk63_dn6 = assign5840_e3798_d_n6;
        locals.var_t3__blk63_dn7 = assign5840_e3798_d_n7;
        locals.var_t3__blk63_dn10 = assign5840_e3798_d_n10;
        locals.var_t3__blk63_dn11 = assign5840_e3798_d_n11;
        locals.var_t3__blk63_dn12 = assign5840_e3798_d_n12;
        locals.var_t3__blk63_dn17 = assign5840_e3798_d_n17;
        locals.var_t3__blk63_rv = 0.0;

        let (assign5850_e3806, assign5850_e3806_d_n0, assign5850_e3806_d_n2, assign5850_e3806_d_n6, assign5850_e3806_d_n7, assign5850_e3806_d_n10, assign5850_e3806_d_n11, assign5850_e3806_d_n12, assign5850_e3806_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let assign5850_e3803: f64 = (locals.var_vthq).abs();
        let assign5850_e3804: f64 = (2.0 * assign5850_e3803);
        (assign5850_e3804, (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn0 } else { (-locals.var_vthq_dn0) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn2 } else { (-locals.var_vthq_dn2) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn6 } else { (-locals.var_vthq_dn6) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn7 } else { (-locals.var_vthq_dn7) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn10 } else { (-locals.var_vthq_dn10) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn11 } else { (-locals.var_vthq_dn11) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn12 } else { (-locals.var_vthq_dn12) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn17 } else { (-locals.var_vthq_dn17) }),)
    } else {
        (locals.var_t4w, locals.var_t4w_dn0, locals.var_t4w_dn2, locals.var_t4w_dn6, locals.var_t4w_dn7, locals.var_t4w_dn10, locals.var_t4w_dn11, locals.var_t4w_dn12, locals.var_t4w_dn17,)
    }
};
        locals.var_t4w = assign5850_e3806;
        locals.var_t4w_dn0 = assign5850_e3806_d_n0;
        locals.var_t4w_dn2 = assign5850_e3806_d_n2;
        locals.var_t4w_dn6 = assign5850_e3806_d_n6;
        locals.var_t4w_dn7 = assign5850_e3806_d_n7;
        locals.var_t4w_dn10 = assign5850_e3806_d_n10;
        locals.var_t4w_dn11 = assign5850_e3806_d_n11;
        locals.var_t4w_dn12 = assign5850_e3806_d_n12;
        locals.var_t4w_dn17 = assign5850_e3806_d_n17;
        locals.var_t4w_rv = 0.0;

        let (assign5860_e3815, assign5860_e3815_d_n0, assign5860_e3815_d_n2, assign5860_e3815_d_n6, assign5860_e3815_d_n7, assign5860_e3815_d_n10, assign5860_e3815_d_n11, assign5860_e3815_d_n12, assign5860_e3815_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let assign5860_e3811: f64 = (locals.var_vfb - locals.var_vthq);
        let assign5860_e3813: f64 = (assign5860_e3811 + p.p205);
        (assign5860_e3813, (-locals.var_vthq_dn0), (-locals.var_vthq_dn2), (-locals.var_vthq_dn6), (-locals.var_vthq_dn7), (-locals.var_vthq_dn10), (-locals.var_vthq_dn11), (-locals.var_vthq_dn12), (-locals.var_vthq_dn17),)
    } else {
        (locals.var_t6__blk67, locals.var_t6__blk67_dn0, locals.var_t6__blk67_dn2, locals.var_t6__blk67_dn6, locals.var_t6__blk67_dn7, locals.var_t6__blk67_dn10, locals.var_t6__blk67_dn11, locals.var_t6__blk67_dn12, locals.var_t6__blk67_dn17,)
    }
};
        locals.var_t6__blk67 = assign5860_e3815;
        locals.var_t6__blk67_dn0 = assign5860_e3815_d_n0;
        locals.var_t6__blk67_dn2 = assign5860_e3815_d_n2;
        locals.var_t6__blk67_dn6 = assign5860_e3815_d_n6;
        locals.var_t6__blk67_dn7 = assign5860_e3815_d_n7;
        locals.var_t6__blk67_dn10 = assign5860_e3815_d_n10;
        locals.var_t6__blk67_dn11 = assign5860_e3815_d_n11;
        locals.var_t6__blk67_dn12 = assign5860_e3815_d_n12;
        locals.var_t6__blk67_dn17 = assign5860_e3815_d_n17;
        locals.var_t6__blk67_rv = 0.0;

        let (assign5870_e3825, assign5870_e3825_d_n0, assign5870_e3825_d_n2, assign5870_e3825_d_n6, assign5870_e3825_d_n7, assign5870_e3825_d_n10, assign5870_e3825_d_n11, assign5870_e3825_d_n12, assign5870_e3825_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let (assign5870_e3823, assign5870_e3823_d_n0, assign5870_e3823_d_n2, assign5870_e3823_d_n6, assign5870_e3823_d_n7, assign5870_e3823_d_n10, assign5870_e3823_d_n11, assign5870_e3823_d_n12, assign5870_e3823_d_n17,) = {
            if (locals.var_t6__blk67 > locals.var_t4w) {
                (locals.var_t6__blk67, locals.var_t6__blk67_dn0, locals.var_t6__blk67_dn2, locals.var_t6__blk67_dn6, locals.var_t6__blk67_dn7, locals.var_t6__blk67_dn10, locals.var_t6__blk67_dn11, locals.var_t6__blk67_dn12, locals.var_t6__blk67_dn17,)
            } else {
                (locals.var_t4w, locals.var_t4w_dn0, locals.var_t4w_dn2, locals.var_t4w_dn6, locals.var_t4w_dn7, locals.var_t4w_dn10, locals.var_t4w_dn11, locals.var_t4w_dn12, locals.var_t4w_dn17,)
            }
        };
        (assign5870_e3823, assign5870_e3823_d_n0, assign5870_e3823_d_n2, assign5870_e3823_d_n6, assign5870_e3823_d_n7, assign5870_e3823_d_n10, assign5870_e3823_d_n11, assign5870_e3823_d_n12, assign5870_e3823_d_n17,)
    } else {
        (locals.var_t4__blk64, locals.var_t4__blk64_dn0, locals.var_t4__blk64_dn2, locals.var_t4__blk64_dn6, locals.var_t4__blk64_dn7, locals.var_t4__blk64_dn10, locals.var_t4__blk64_dn11, locals.var_t4__blk64_dn12, locals.var_t4__blk64_dn17,)
    }
};
        locals.var_t4__blk64 = assign5870_e3825;
        locals.var_t4__blk64_dn0 = assign5870_e3825_d_n0;
        locals.var_t4__blk64_dn2 = assign5870_e3825_d_n2;
        locals.var_t4__blk64_dn6 = assign5870_e3825_d_n6;
        locals.var_t4__blk64_dn7 = assign5870_e3825_d_n7;
        locals.var_t4__blk64_dn10 = assign5870_e3825_d_n10;
        locals.var_t4__blk64_dn11 = assign5870_e3825_d_n11;
        locals.var_t4__blk64_dn12 = assign5870_e3825_d_n12;
        locals.var_t4__blk64_dn17 = assign5870_e3825_d_n17;
        locals.var_t4__blk64_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5880_e3836, assign5880_e3836_d_n0, assign5880_e3836_d_n2, assign5880_e3836_d_n6, assign5880_e3836_d_n7, assign5880_e3836_d_n10, assign5880_e3836_d_n11, assign5880_e3836_d_n12, assign5880_e3836_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let assign5880_e3830: f64 = (1.0 / locals.var_t4__blk64);
        let assign5880_e3832: f64 = (assign5880_e3830 - locals.var_t3__blk63);
        let assign5880_e3834: f64 = (assign5880_e3832 - 0.0001);
        (assign5880_e3834, ((-(locals.var_t4__blk64_dn0 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - locals.var_t3__blk63_dn0), ((-(locals.var_t4__blk64_dn2 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - locals.var_t3__blk63_dn2), ((-(locals.var_t4__blk64_dn6 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - locals.var_t3__blk63_dn6), ((-(locals.var_t4__blk64_dn7 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - locals.var_t3__blk63_dn7), ((-(locals.var_t4__blk64_dn10 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - locals.var_t3__blk63_dn10), ((-(locals.var_t4__blk64_dn11 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - locals.var_t3__blk63_dn11), ((-(locals.var_t4__blk64_dn12 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - locals.var_t3__blk63_dn12), ((-(locals.var_t4__blk64_dn17 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - locals.var_t3__blk63_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign5880_e3836;
        locals.var_tmf1_dn0 = assign5880_e3836_d_n0;
        locals.var_tmf1_dn2 = assign5880_e3836_d_n2;
        locals.var_tmf1_dn6 = assign5880_e3836_d_n6;
        locals.var_tmf1_dn7 = assign5880_e3836_d_n7;
        locals.var_tmf1_dn10 = assign5880_e3836_d_n10;
        locals.var_tmf1_dn11 = assign5880_e3836_d_n11;
        locals.var_tmf1_dn12 = assign5880_e3836_d_n12;
        locals.var_tmf1_dn17 = assign5880_e3836_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign5890_e3847, assign5890_e3847_d_n0, assign5890_e3847_d_n2, assign5890_e3847_d_n6, assign5890_e3847_d_n7, assign5890_e3847_d_n10, assign5890_e3847_d_n11, assign5890_e3847_d_n12, assign5890_e3847_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let assign5890_e3842: f64 = (1.0 / locals.var_t4__blk64);
        let assign5890_e3843: f64 = (4.0 * assign5890_e3842);
        let assign5890_e3845: f64 = (assign5890_e3843 * 0.0001);
        (assign5890_e3845, ((4.0 * (-(locals.var_t4__blk64_dn0 / (locals.var_t4__blk64 * locals.var_t4__blk64)))) * 0.0001), ((4.0 * (-(locals.var_t4__blk64_dn2 / (locals.var_t4__blk64 * locals.var_t4__blk64)))) * 0.0001), ((4.0 * (-(locals.var_t4__blk64_dn6 / (locals.var_t4__blk64 * locals.var_t4__blk64)))) * 0.0001), ((4.0 * (-(locals.var_t4__blk64_dn7 / (locals.var_t4__blk64 * locals.var_t4__blk64)))) * 0.0001), ((4.0 * (-(locals.var_t4__blk64_dn10 / (locals.var_t4__blk64 * locals.var_t4__blk64)))) * 0.0001), ((4.0 * (-(locals.var_t4__blk64_dn11 / (locals.var_t4__blk64 * locals.var_t4__blk64)))) * 0.0001), ((4.0 * (-(locals.var_t4__blk64_dn12 / (locals.var_t4__blk64 * locals.var_t4__blk64)))) * 0.0001), ((4.0 * (-(locals.var_t4__blk64_dn17 / (locals.var_t4__blk64 * locals.var_t4__blk64)))) * 0.0001),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign5890_e3847;
        locals.var_tmf2_dn0 = assign5890_e3847_d_n0;
        locals.var_tmf2_dn2 = assign5890_e3847_d_n2;
        locals.var_tmf2_dn6 = assign5890_e3847_d_n6;
        locals.var_tmf2_dn7 = assign5890_e3847_d_n7;
        locals.var_tmf2_dn10 = assign5890_e3847_d_n10;
        locals.var_tmf2_dn11 = assign5890_e3847_d_n11;
        locals.var_tmf2_dn12 = assign5890_e3847_d_n12;
        locals.var_tmf2_dn17 = assign5890_e3847_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign5900_e3858, assign5900_e3858_d_n0, assign5900_e3858_d_n2, assign5900_e3858_d_n6, assign5900_e3858_d_n7, assign5900_e3858_d_n10, assign5900_e3858_d_n11, assign5900_e3858_d_n12, assign5900_e3858_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let (assign5900_e3856, assign5900_e3856_d_n0, assign5900_e3856_d_n2, assign5900_e3856_d_n6, assign5900_e3856_d_n7, assign5900_e3856_d_n10, assign5900_e3856_d_n11, assign5900_e3856_d_n12, assign5900_e3856_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign5900_e3855: f64 = (-locals.var_tmf2);
                (assign5900_e3855, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign5900_e3856, assign5900_e3856_d_n0, assign5900_e3856_d_n2, assign5900_e3856_d_n6, assign5900_e3856_d_n7, assign5900_e3856_d_n10, assign5900_e3856_d_n11, assign5900_e3856_d_n12, assign5900_e3856_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign5900_e3858;
        locals.var_tmf2_dn0 = assign5900_e3858_d_n0;
        locals.var_tmf2_dn2 = assign5900_e3858_d_n2;
        locals.var_tmf2_dn6 = assign5900_e3858_d_n6;
        locals.var_tmf2_dn7 = assign5900_e3858_d_n7;
        locals.var_tmf2_dn10 = assign5900_e3858_d_n10;
        locals.var_tmf2_dn11 = assign5900_e3858_d_n11;
        locals.var_tmf2_dn12 = assign5900_e3858_d_n12;
        locals.var_tmf2_dn17 = assign5900_e3858_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign5910_e3868, assign5910_e3868_d_n0, assign5910_e3868_d_n2, assign5910_e3868_d_n6, assign5910_e3868_d_n7, assign5910_e3868_d_n10, assign5910_e3868_d_n11, assign5910_e3868_d_n12, assign5910_e3868_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let assign5910_e3863: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5910_e3865: f64 = (assign5910_e3863 + locals.var_tmf2);
        let assign5910_e3866: f64 = (assign5910_e3865).sqrt();
        (assign5910_e3866, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5910_e3866)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5910_e3866)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign5910_e3866)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign5910_e3866)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign5910_e3866)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign5910_e3866)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign5910_e3866)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign5910_e3866)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign5910_e3868;
        locals.var_tmf2_dn0 = assign5910_e3868_d_n0;
        locals.var_tmf2_dn2 = assign5910_e3868_d_n2;
        locals.var_tmf2_dn6 = assign5910_e3868_d_n6;
        locals.var_tmf2_dn7 = assign5910_e3868_d_n7;
        locals.var_tmf2_dn10 = assign5910_e3868_d_n10;
        locals.var_tmf2_dn11 = assign5910_e3868_d_n11;
        locals.var_tmf2_dn12 = assign5910_e3868_d_n12;
        locals.var_tmf2_dn17 = assign5910_e3868_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign5920_e3881, assign5920_e3881_d_n0, assign5920_e3881_d_n2, assign5920_e3881_d_n6, assign5920_e3881_d_n7, assign5920_e3881_d_n10, assign5920_e3881_d_n11, assign5920_e3881_d_n12, assign5920_e3881_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let assign5920_e3873: f64 = (1.0 / locals.var_t4__blk64);
        let assign5920_e3877: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5920_e3878: f64 = (0.5 * assign5920_e3877);
        let assign5920_e3879: f64 = (assign5920_e3873 - assign5920_e3878);
        (assign5920_e3879, ((-(locals.var_t4__blk64_dn0 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-(locals.var_t4__blk64_dn2 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-(locals.var_t4__blk64_dn6 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-(locals.var_t4__blk64_dn7 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-(locals.var_t4__blk64_dn10 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-(locals.var_t4__blk64_dn11 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-(locals.var_t4__blk64_dn12 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), ((-(locals.var_t4__blk64_dn17 / (locals.var_t4__blk64 * locals.var_t4__blk64))) - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_t2__blk62, locals.var_t2__blk62_dn0, locals.var_t2__blk62_dn2, locals.var_t2__blk62_dn6, locals.var_t2__blk62_dn7, locals.var_t2__blk62_dn10, locals.var_t2__blk62_dn11, locals.var_t2__blk62_dn12, locals.var_t2__blk62_dn17,)
    }
};
        locals.var_t2__blk62 = assign5920_e3881;
        locals.var_t2__blk62_dn0 = assign5920_e3881_d_n0;
        locals.var_t2__blk62_dn2 = assign5920_e3881_d_n2;
        locals.var_t2__blk62_dn6 = assign5920_e3881_d_n6;
        locals.var_t2__blk62_dn7 = assign5920_e3881_d_n7;
        locals.var_t2__blk62_dn10 = assign5920_e3881_d_n10;
        locals.var_t2__blk62_dn11 = assign5920_e3881_d_n11;
        locals.var_t2__blk62_dn12 = assign5920_e3881_d_n12;
        locals.var_t2__blk62_dn17 = assign5920_e3881_d_n17;
        locals.var_t2__blk62_rv = 0.0;

        let (assign5930_e3890, assign5930_e3890_d_n0, assign5930_e3890_d_n2, assign5930_e3890_d_n6, assign5930_e3890_d_n7, assign5930_e3890_d_n10, assign5930_e3890_d_n11, assign5930_e3890_d_n12, assign5930_e3890_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let assign5930_e3886: f64 = (p.p204 * locals.var_t2__blk62);
        let assign5930_e3888: f64 = (assign5930_e3886 + p.p206);
        (assign5930_e3888, (p.p204 * locals.var_t2__blk62_dn0), (p.p204 * locals.var_t2__blk62_dn2), (p.p204 * locals.var_t2__blk62_dn6), (p.p204 * locals.var_t2__blk62_dn7), (p.p204 * locals.var_t2__blk62_dn10), (p.p204 * locals.var_t2__blk62_dn11), (p.p204 * locals.var_t2__blk62_dn12), (p.p204 * locals.var_t2__blk62_dn17),)
    } else {
        (locals.var_dtfox, locals.var_dtfox_dn0, locals.var_dtfox_dn2, locals.var_dtfox_dn6, locals.var_dtfox_dn7, locals.var_dtfox_dn10, locals.var_dtfox_dn11, locals.var_dtfox_dn12, locals.var_dtfox_dn17,)
    }
};
        locals.var_dtfox = assign5930_e3890;
        locals.var_dtfox_dn0 = assign5930_e3890_d_n0;
        locals.var_dtfox_dn2 = assign5930_e3890_d_n2;
        locals.var_dtfox_dn6 = assign5930_e3890_d_n6;
        locals.var_dtfox_dn7 = assign5930_e3890_d_n7;
        locals.var_dtfox_dn10 = assign5930_e3890_d_n10;
        locals.var_dtfox_dn11 = assign5930_e3890_d_n11;
        locals.var_dtfox_dn12 = assign5930_e3890_d_n12;
        locals.var_dtfox_dn17 = assign5930_e3890_d_n17;
        locals.var_dtfox_rv = 0.0;

        let assign5940_e3893: f64 = (locals.var_dtfox * 1000000000000.0);
        let assign5940_e3895: f64 = if assign5940_e3893 < locals.var_tfox0 { 1.0 } else { 0.0 };
        locals.var_guard71 = assign5940_e3895;
        locals.var_guard71_rv = 0.0;

        let (assign5950_e3902, assign5950_e3902_d_n0, assign5950_e3902_d_n2, assign5950_e3902_d_n6, assign5950_e3902_d_n7, assign5950_e3902_d_n10, assign5950_e3902_d_n11, assign5950_e3902_d_n12, assign5950_e3902_d_n17,) = {
    if ((locals.var_guard69 == 0.0) && (locals.var_guard71 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dtfox, locals.var_dtfox_dn0, locals.var_dtfox_dn2, locals.var_dtfox_dn6, locals.var_dtfox_dn7, locals.var_dtfox_dn10, locals.var_dtfox_dn11, locals.var_dtfox_dn12, locals.var_dtfox_dn17,)
    }
};
        locals.var_dtfox = assign5950_e3902;
        locals.var_dtfox_dn0 = assign5950_e3902_d_n0;
        locals.var_dtfox_dn2 = assign5950_e3902_d_n2;
        locals.var_dtfox_dn6 = assign5950_e3902_d_n6;
        locals.var_dtfox_dn7 = assign5950_e3902_d_n7;
        locals.var_dtfox_dn10 = assign5950_e3902_d_n10;
        locals.var_dtfox_dn11 = assign5950_e3902_d_n11;
        locals.var_dtfox_dn12 = assign5950_e3902_d_n12;
        locals.var_dtfox_dn17 = assign5950_e3902_d_n17;
        locals.var_dtfox_rv = 0.0;

        let (assign5960_e3909,) = {
    if ((locals.var_guard69 == 0.0) && (locals.var_guard71 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign5960_e3909;
        locals.var_flg_qme_rv = 0.0;

        let (assign5970_e3916, assign5970_e3916_d_n0, assign5970_e3916_d_n2, assign5970_e3916_d_n6, assign5970_e3916_d_n7, assign5970_e3916_d_n10, assign5970_e3916_d_n11, assign5970_e3916_d_n12, assign5970_e3916_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let assign5970_e3914: f64 = (locals.var_tfox0 + locals.var_dtfox);
        (assign5970_e3914, locals.var_dtfox_dn0, locals.var_dtfox_dn2, locals.var_dtfox_dn6, locals.var_dtfox_dn7, locals.var_dtfox_dn10, locals.var_dtfox_dn11, locals.var_dtfox_dn12, locals.var_dtfox_dn17,)
    } else {
        (locals.var_tfoxe, locals.var_tfoxe_dn0, locals.var_tfoxe_dn2, locals.var_tfoxe_dn6, locals.var_tfoxe_dn7, locals.var_tfoxe_dn10, locals.var_tfoxe_dn11, locals.var_tfoxe_dn12, locals.var_tfoxe_dn17,)
    }
};
        locals.var_tfoxe = assign5970_e3916;
        locals.var_tfoxe_dn0 = assign5970_e3916_d_n0;
        locals.var_tfoxe_dn2 = assign5970_e3916_d_n2;
        locals.var_tfoxe_dn6 = assign5970_e3916_d_n6;
        locals.var_tfoxe_dn7 = assign5970_e3916_d_n7;
        locals.var_tfoxe_dn10 = assign5970_e3916_d_n10;
        locals.var_tfoxe_dn11 = assign5970_e3916_d_n11;
        locals.var_tfoxe_dn12 = assign5970_e3916_d_n12;
        locals.var_tfoxe_dn17 = assign5970_e3916_d_n17;
        locals.var_tfoxe_rv = 0.0;

        let (assign5980_e3923, assign5980_e3923_d_n0, assign5980_e3923_d_n2, assign5980_e3923_d_n6, assign5980_e3923_d_n7, assign5980_e3923_d_n10, assign5980_e3923_d_n11, assign5980_e3923_d_n12, assign5980_e3923_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let assign5980_e3921: f64 = (3.453133e-11 / locals.var_tfoxe);
        (assign5980_e3921, (-((3.453133e-11 * locals.var_tfoxe_dn0) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn2) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn6) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn7) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn10) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn11) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn12) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn17) / (locals.var_tfoxe * locals.var_tfoxe))),)
    } else {
        (locals.var_c_fox, locals.var_c_fox_dn0, locals.var_c_fox_dn2, locals.var_c_fox_dn6, locals.var_c_fox_dn7, locals.var_c_fox_dn10, locals.var_c_fox_dn11, locals.var_c_fox_dn12, locals.var_c_fox_dn17,)
    }
};
        locals.var_c_fox = assign5980_e3923;
        locals.var_c_fox_dn0 = assign5980_e3923_d_n0;
        locals.var_c_fox_dn2 = assign5980_e3923_d_n2;
        locals.var_c_fox_dn6 = assign5980_e3923_d_n6;
        locals.var_c_fox_dn7 = assign5980_e3923_d_n7;
        locals.var_c_fox_dn10 = assign5980_e3923_d_n10;
        locals.var_c_fox_dn11 = assign5980_e3923_d_n11;
        locals.var_c_fox_dn12 = assign5980_e3923_d_n12;
        locals.var_c_fox_dn17 = assign5980_e3923_d_n17;
        locals.var_c_fox_rv = 0.0;

        let (assign5990_e3930, assign5990_e3930_d_n0, assign5990_e3930_d_n2, assign5990_e3930_d_n6, assign5990_e3930_d_n7, assign5990_e3930_d_n10, assign5990_e3930_d_n11, assign5990_e3930_d_n12, assign5990_e3930_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let assign5990_e3928: f64 = (locals.var_tfoxe / 3.453133e-11);
        (assign5990_e3928, (locals.var_tfoxe_dn0 / 3.453133e-11), (locals.var_tfoxe_dn2 / 3.453133e-11), (locals.var_tfoxe_dn6 / 3.453133e-11), (locals.var_tfoxe_dn7 / 3.453133e-11), (locals.var_tfoxe_dn10 / 3.453133e-11), (locals.var_tfoxe_dn11 / 3.453133e-11), (locals.var_tfoxe_dn12 / 3.453133e-11), (locals.var_tfoxe_dn17 / 3.453133e-11),)
    } else {
        (locals.var_c_fox_inv, locals.var_c_fox_inv_dn0, locals.var_c_fox_inv_dn2, locals.var_c_fox_inv_dn6, locals.var_c_fox_inv_dn7, locals.var_c_fox_inv_dn10, locals.var_c_fox_inv_dn11, locals.var_c_fox_inv_dn12, locals.var_c_fox_inv_dn17,)
    }
};
        locals.var_c_fox_inv = assign5990_e3930;
        locals.var_c_fox_inv_dn0 = assign5990_e3930_d_n0;
        locals.var_c_fox_inv_dn2 = assign5990_e3930_d_n2;
        locals.var_c_fox_inv_dn6 = assign5990_e3930_d_n6;
        locals.var_c_fox_inv_dn7 = assign5990_e3930_d_n7;
        locals.var_c_fox_inv_dn10 = assign5990_e3930_d_n10;
        locals.var_c_fox_inv_dn11 = assign5990_e3930_d_n11;
        locals.var_c_fox_inv_dn12 = assign5990_e3930_d_n12;
        locals.var_c_fox_inv_dn17 = assign5990_e3930_d_n17;
        locals.var_c_fox_inv_rv = 0.0;

        let (assign6000_e3941, assign6000_e3941_d_n0, assign6000_e3941_d_n2, assign6000_e3941_d_n6, assign6000_e3941_d_n7, assign6000_e3941_d_n10, assign6000_e3941_d_n11, assign6000_e3941_d_n12, assign6000_e3941_d_n17,) = {
    if (locals.var_guard69 == 0.0) {
        let assign6000_e3935: f64 = (locals.var_cnst0soi * locals.var_cnst0soi);
        let assign6000_e3937: f64 = (assign6000_e3935 * locals.var_c_fox_inv);
        let assign6000_e3939: f64 = (assign6000_e3937 * locals.var_c_fox_inv);
        (assign6000_e3939, ((((((locals.var_cnst0soi_dn0 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn0)) * locals.var_c_fox_inv) + (assign6000_e3935 * locals.var_c_fox_inv_dn0)) * locals.var_c_fox_inv) + (assign6000_e3937 * locals.var_c_fox_inv_dn0)), ((((((locals.var_cnst0soi_dn2 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn2)) * locals.var_c_fox_inv) + (assign6000_e3935 * locals.var_c_fox_inv_dn2)) * locals.var_c_fox_inv) + (assign6000_e3937 * locals.var_c_fox_inv_dn2)), ((((((locals.var_cnst0soi_dn6 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn6)) * locals.var_c_fox_inv) + (assign6000_e3935 * locals.var_c_fox_inv_dn6)) * locals.var_c_fox_inv) + (assign6000_e3937 * locals.var_c_fox_inv_dn6)), ((((((locals.var_cnst0soi_dn7 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn7)) * locals.var_c_fox_inv) + (assign6000_e3935 * locals.var_c_fox_inv_dn7)) * locals.var_c_fox_inv) + (assign6000_e3937 * locals.var_c_fox_inv_dn7)), ((((((locals.var_cnst0soi_dn10 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn10)) * locals.var_c_fox_inv) + (assign6000_e3935 * locals.var_c_fox_inv_dn10)) * locals.var_c_fox_inv) + (assign6000_e3937 * locals.var_c_fox_inv_dn10)), ((((((locals.var_cnst0soi_dn11 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn11)) * locals.var_c_fox_inv) + (assign6000_e3935 * locals.var_c_fox_inv_dn11)) * locals.var_c_fox_inv) + (assign6000_e3937 * locals.var_c_fox_inv_dn11)), ((((((locals.var_cnst0soi_dn12 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn12)) * locals.var_c_fox_inv) + (assign6000_e3935 * locals.var_c_fox_inv_dn12)) * locals.var_c_fox_inv) + (assign6000_e3937 * locals.var_c_fox_inv_dn12)), ((((((locals.var_cnst0soi_dn17 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn17)) * locals.var_c_fox_inv) + (assign6000_e3935 * locals.var_c_fox_inv_dn17)) * locals.var_c_fox_inv) + (assign6000_e3937 * locals.var_c_fox_inv_dn17)),)
    } else {
        (locals.var_cnstc_foxi, locals.var_cnstc_foxi_dn0, locals.var_cnstc_foxi_dn2, locals.var_cnstc_foxi_dn6, locals.var_cnstc_foxi_dn7, locals.var_cnstc_foxi_dn10, locals.var_cnstc_foxi_dn11, locals.var_cnstc_foxi_dn12, locals.var_cnstc_foxi_dn17,)
    }
};
        locals.var_cnstc_foxi = assign6000_e3941;
        locals.var_cnstc_foxi_dn0 = assign6000_e3941_d_n0;
        locals.var_cnstc_foxi_dn2 = assign6000_e3941_d_n2;
        locals.var_cnstc_foxi_dn6 = assign6000_e3941_d_n6;
        locals.var_cnstc_foxi_dn7 = assign6000_e3941_d_n7;
        locals.var_cnstc_foxi_dn10 = assign6000_e3941_d_n10;
        locals.var_cnstc_foxi_dn11 = assign6000_e3941_d_n11;
        locals.var_cnstc_foxi_dn12 = assign6000_e3941_d_n12;
        locals.var_cnstc_foxi_dn17 = assign6000_e3941_d_n17;
        locals.var_cnstc_foxi_rv = 0.0;

        let assign6010_e3948: f64 = if ((p.p43 == 1.0) || (locals.var_subversion < 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard72 = assign6010_e3948;
        locals.var_guard72_rv = 0.0;

        let (assign6020_e3956, assign6020_e3956_d_n0, assign6020_e3956_d_n2, assign6020_e3956_d_n6, assign6020_e3956_d_n7, assign6020_e3956_d_n10, assign6020_e3956_d_n11, assign6020_e3956_d_n12, assign6020_e3956_d_n17,) = {
    if (locals.var_guard72 != 0.0) {
        let assign6020_e3952: f64 = (0.5 - locals.var_vbspz);
        let assign6020_e3954: f64 = (assign6020_e3952 - 0.001);
        (assign6020_e3954, (-locals.var_vbspz_dn0), (-locals.var_vbspz_dn2), (-locals.var_vbspz_dn6), (-locals.var_vbspz_dn7), (-locals.var_vbspz_dn10), (-locals.var_vbspz_dn11), (-locals.var_vbspz_dn12), (-locals.var_vbspz_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign6020_e3956;
        locals.var_tmf1_dn0 = assign6020_e3956_d_n0;
        locals.var_tmf1_dn2 = assign6020_e3956_d_n2;
        locals.var_tmf1_dn6 = assign6020_e3956_d_n6;
        locals.var_tmf1_dn7 = assign6020_e3956_d_n7;
        locals.var_tmf1_dn10 = assign6020_e3956_d_n10;
        locals.var_tmf1_dn11 = assign6020_e3956_d_n11;
        locals.var_tmf1_dn12 = assign6020_e3956_d_n12;
        locals.var_tmf1_dn17 = assign6020_e3956_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign6030_e3964, assign6030_e3964_d_n0, assign6030_e3964_d_n2, assign6030_e3964_d_n6, assign6030_e3964_d_n7, assign6030_e3964_d_n10, assign6030_e3964_d_n11, assign6030_e3964_d_n12, assign6030_e3964_d_n17,) = {
    if (locals.var_guard72 != 0.0) {
        let assign6030_e3960: f64 = (4.0 * 0.5);
        let assign6030_e3962: f64 = (assign6030_e3960 * 0.001);
        (assign6030_e3962, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6030_e3964;
        locals.var_tmf2_dn0 = assign6030_e3964_d_n0;
        locals.var_tmf2_dn2 = assign6030_e3964_d_n2;
        locals.var_tmf2_dn6 = assign6030_e3964_d_n6;
        locals.var_tmf2_dn7 = assign6030_e3964_d_n7;
        locals.var_tmf2_dn10 = assign6030_e3964_d_n10;
        locals.var_tmf2_dn11 = assign6030_e3964_d_n11;
        locals.var_tmf2_dn12 = assign6030_e3964_d_n12;
        locals.var_tmf2_dn17 = assign6030_e3964_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6040_e3974, assign6040_e3974_d_n0, assign6040_e3974_d_n2, assign6040_e3974_d_n6, assign6040_e3974_d_n7, assign6040_e3974_d_n10, assign6040_e3974_d_n11, assign6040_e3974_d_n12, assign6040_e3974_d_n17,) = {
    if (locals.var_guard72 != 0.0) {
        let (assign6040_e3972, assign6040_e3972_d_n0, assign6040_e3972_d_n2, assign6040_e3972_d_n6, assign6040_e3972_d_n7, assign6040_e3972_d_n10, assign6040_e3972_d_n11, assign6040_e3972_d_n12, assign6040_e3972_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign6040_e3971: f64 = (-locals.var_tmf2);
                (assign6040_e3971, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign6040_e3972, assign6040_e3972_d_n0, assign6040_e3972_d_n2, assign6040_e3972_d_n6, assign6040_e3972_d_n7, assign6040_e3972_d_n10, assign6040_e3972_d_n11, assign6040_e3972_d_n12, assign6040_e3972_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6040_e3974;
        locals.var_tmf2_dn0 = assign6040_e3974_d_n0;
        locals.var_tmf2_dn2 = assign6040_e3974_d_n2;
        locals.var_tmf2_dn6 = assign6040_e3974_d_n6;
        locals.var_tmf2_dn7 = assign6040_e3974_d_n7;
        locals.var_tmf2_dn10 = assign6040_e3974_d_n10;
        locals.var_tmf2_dn11 = assign6040_e3974_d_n11;
        locals.var_tmf2_dn12 = assign6040_e3974_d_n12;
        locals.var_tmf2_dn17 = assign6040_e3974_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6050_e3983, assign6050_e3983_d_n0, assign6050_e3983_d_n2, assign6050_e3983_d_n6, assign6050_e3983_d_n7, assign6050_e3983_d_n10, assign6050_e3983_d_n11, assign6050_e3983_d_n12, assign6050_e3983_d_n17,) = {
    if (locals.var_guard72 != 0.0) {
        let assign6050_e3978: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign6050_e3980: f64 = (assign6050_e3978 + locals.var_tmf2);
        let assign6050_e3981: f64 = (assign6050_e3980).sqrt();
        (assign6050_e3981, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign6050_e3981)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign6050_e3981)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign6050_e3981)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign6050_e3981)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign6050_e3981)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign6050_e3981)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign6050_e3981)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign6050_e3981)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6050_e3983;
        locals.var_tmf2_dn0 = assign6050_e3983_d_n0;
        locals.var_tmf2_dn2 = assign6050_e3983_d_n2;
        locals.var_tmf2_dn6 = assign6050_e3983_d_n6;
        locals.var_tmf2_dn7 = assign6050_e3983_d_n7;
        locals.var_tmf2_dn10 = assign6050_e3983_d_n10;
        locals.var_tmf2_dn11 = assign6050_e3983_d_n11;
        locals.var_tmf2_dn12 = assign6050_e3983_d_n12;
        locals.var_tmf2_dn17 = assign6050_e3983_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6060_e3993, assign6060_e3993_d_n0, assign6060_e3993_d_n2, assign6060_e3993_d_n6, assign6060_e3993_d_n7, assign6060_e3993_d_n10, assign6060_e3993_d_n11, assign6060_e3993_d_n12, assign6060_e3993_d_n17,) = {
    if (locals.var_guard72 != 0.0) {
        let assign6060_e3989: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign6060_e3990: f64 = (0.5 * assign6060_e3989);
        let assign6060_e3991: f64 = (0.5 - assign6060_e3990);
        (assign6060_e3991, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (-(0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_vbsz2, locals.var_vbsz2_dn0, locals.var_vbsz2_dn2, locals.var_vbsz2_dn6, locals.var_vbsz2_dn7, locals.var_vbsz2_dn10, locals.var_vbsz2_dn11, locals.var_vbsz2_dn12, locals.var_vbsz2_dn17,)
    }
};
        locals.var_vbsz2 = assign6060_e3993;
        locals.var_vbsz2_dn0 = assign6060_e3993_d_n0;
        locals.var_vbsz2_dn2 = assign6060_e3993_d_n2;
        locals.var_vbsz2_dn6 = assign6060_e3993_d_n6;
        locals.var_vbsz2_dn7 = assign6060_e3993_d_n7;
        locals.var_vbsz2_dn10 = assign6060_e3993_d_n10;
        locals.var_vbsz2_dn11 = assign6060_e3993_d_n11;
        locals.var_vbsz2_dn12 = assign6060_e3993_d_n12;
        locals.var_vbsz2_dn17 = assign6060_e3993_d_n17;
        locals.var_vbsz2_rv = 0.0;

        let (assign6070_e4010, assign6070_e4010_d_n0, assign6070_e4010_d_n2, assign6070_e4010_d_n6, assign6070_e4010_d_n7, assign6070_e4010_d_n10, assign6070_e4010_d_n11, assign6070_e4010_d_n12, assign6070_e4010_d_n17,) = {
    if (locals.var_guard72 != 0.0) {
        let assign6070_e3996: f64 = (-p.p237);
        let assign6070_e3998: f64 = (assign6070_e3996 * p.p237);
        let assign6070_e4000: f64 = (assign6070_e3998 * locals.var_q_nsub);
        let assign6070_e4003: f64 = (2.0 * 1.034943e-10);
        let assign6070_e4004: f64 = (assign6070_e4000 / assign6070_e4003);
        let assign6070_e4006: f64 = (assign6070_e4004 + locals.var_pb2);
        let assign6070_e4008: f64 = (assign6070_e4006 - locals.var_beta_inv);
        (assign6070_e4008, (((assign6070_e3998 * locals.var_q_nsub_dn0) / assign6070_e4003) + locals.var_pb2_dn0), (((assign6070_e3998 * locals.var_q_nsub_dn2) / assign6070_e4003) + locals.var_pb2_dn2), (((assign6070_e3998 * locals.var_q_nsub_dn6) / assign6070_e4003) + locals.var_pb2_dn6), (((assign6070_e3998 * locals.var_q_nsub_dn7) / assign6070_e4003) + locals.var_pb2_dn7), ((((assign6070_e3998 * locals.var_q_nsub_dn10) / assign6070_e4003) + locals.var_pb2_dn10) - locals.var_beta_inv_dn10), (((assign6070_e3998 * locals.var_q_nsub_dn11) / assign6070_e4003) + locals.var_pb2_dn11), (((assign6070_e3998 * locals.var_q_nsub_dn12) / assign6070_e4003) + locals.var_pb2_dn12), (((assign6070_e3998 * locals.var_q_nsub_dn17) / assign6070_e4003) + locals.var_pb2_dn17),)
    } else {
        (locals.var_vbslim, locals.var_vbslim_dn0, locals.var_vbslim_dn2, locals.var_vbslim_dn6, locals.var_vbslim_dn7, locals.var_vbslim_dn10, locals.var_vbslim_dn11, locals.var_vbslim_dn12, locals.var_vbslim_dn17,)
    }
};
        locals.var_vbslim = assign6070_e4010;
        locals.var_vbslim_dn0 = assign6070_e4010_d_n0;
        locals.var_vbslim_dn2 = assign6070_e4010_d_n2;
        locals.var_vbslim_dn6 = assign6070_e4010_d_n6;
        locals.var_vbslim_dn7 = assign6070_e4010_d_n7;
        locals.var_vbslim_dn10 = assign6070_e4010_d_n10;
        locals.var_vbslim_dn11 = assign6070_e4010_d_n11;
        locals.var_vbslim_dn12 = assign6070_e4010_d_n12;
        locals.var_vbslim_dn17 = assign6070_e4010_d_n17;
        locals.var_vbslim_rv = 0.0;

        let (assign6080_e4018, assign6080_e4018_d_n0, assign6080_e4018_d_n2, assign6080_e4018_d_n6, assign6080_e4018_d_n7, assign6080_e4018_d_n10, assign6080_e4018_d_n11, assign6080_e4018_d_n12, assign6080_e4018_d_n17,) = {
    if (locals.var_guard72 != 0.0) {
        let assign6080_e4014: f64 = (locals.var_vbsz2 - locals.var_vbslim);
        let assign6080_e4016: f64 = (assign6080_e4014 - 0.001);
        (assign6080_e4016, (locals.var_vbsz2_dn0 - locals.var_vbslim_dn0), (locals.var_vbsz2_dn2 - locals.var_vbslim_dn2), (locals.var_vbsz2_dn6 - locals.var_vbslim_dn6), (locals.var_vbsz2_dn7 - locals.var_vbslim_dn7), (locals.var_vbsz2_dn10 - locals.var_vbslim_dn10), (locals.var_vbsz2_dn11 - locals.var_vbslim_dn11), (locals.var_vbsz2_dn12 - locals.var_vbslim_dn12), (locals.var_vbsz2_dn17 - locals.var_vbslim_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign6080_e4018;
        locals.var_tmf1_dn0 = assign6080_e4018_d_n0;
        locals.var_tmf1_dn2 = assign6080_e4018_d_n2;
        locals.var_tmf1_dn6 = assign6080_e4018_d_n6;
        locals.var_tmf1_dn7 = assign6080_e4018_d_n7;
        locals.var_tmf1_dn10 = assign6080_e4018_d_n10;
        locals.var_tmf1_dn11 = assign6080_e4018_d_n11;
        locals.var_tmf1_dn12 = assign6080_e4018_d_n12;
        locals.var_tmf1_dn17 = assign6080_e4018_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign6090_e4026, assign6090_e4026_d_n0, assign6090_e4026_d_n2, assign6090_e4026_d_n6, assign6090_e4026_d_n7, assign6090_e4026_d_n10, assign6090_e4026_d_n11, assign6090_e4026_d_n12, assign6090_e4026_d_n17,) = {
    if (locals.var_guard72 != 0.0) {
        let assign6090_e4022: f64 = (4.0 * locals.var_vbslim);
        let assign6090_e4024: f64 = (assign6090_e4022 * 0.001);
        (assign6090_e4024, ((4.0 * locals.var_vbslim_dn0) * 0.001), ((4.0 * locals.var_vbslim_dn2) * 0.001), ((4.0 * locals.var_vbslim_dn6) * 0.001), ((4.0 * locals.var_vbslim_dn7) * 0.001), ((4.0 * locals.var_vbslim_dn10) * 0.001), ((4.0 * locals.var_vbslim_dn11) * 0.001), ((4.0 * locals.var_vbslim_dn12) * 0.001), ((4.0 * locals.var_vbslim_dn17) * 0.001),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6090_e4026;
        locals.var_tmf2_dn0 = assign6090_e4026_d_n0;
        locals.var_tmf2_dn2 = assign6090_e4026_d_n2;
        locals.var_tmf2_dn6 = assign6090_e4026_d_n6;
        locals.var_tmf2_dn7 = assign6090_e4026_d_n7;
        locals.var_tmf2_dn10 = assign6090_e4026_d_n10;
        locals.var_tmf2_dn11 = assign6090_e4026_d_n11;
        locals.var_tmf2_dn12 = assign6090_e4026_d_n12;
        locals.var_tmf2_dn17 = assign6090_e4026_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6100_e4036, assign6100_e4036_d_n0, assign6100_e4036_d_n2, assign6100_e4036_d_n6, assign6100_e4036_d_n7, assign6100_e4036_d_n10, assign6100_e4036_d_n11, assign6100_e4036_d_n12, assign6100_e4036_d_n17,) = {
    if (locals.var_guard72 != 0.0) {
        let (assign6100_e4034, assign6100_e4034_d_n0, assign6100_e4034_d_n2, assign6100_e4034_d_n6, assign6100_e4034_d_n7, assign6100_e4034_d_n10, assign6100_e4034_d_n11, assign6100_e4034_d_n12, assign6100_e4034_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign6100_e4033: f64 = (-locals.var_tmf2);
                (assign6100_e4033, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign6100_e4034, assign6100_e4034_d_n0, assign6100_e4034_d_n2, assign6100_e4034_d_n6, assign6100_e4034_d_n7, assign6100_e4034_d_n10, assign6100_e4034_d_n11, assign6100_e4034_d_n12, assign6100_e4034_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6100_e4036;
        locals.var_tmf2_dn0 = assign6100_e4036_d_n0;
        locals.var_tmf2_dn2 = assign6100_e4036_d_n2;
        locals.var_tmf2_dn6 = assign6100_e4036_d_n6;
        locals.var_tmf2_dn7 = assign6100_e4036_d_n7;
        locals.var_tmf2_dn10 = assign6100_e4036_d_n10;
        locals.var_tmf2_dn11 = assign6100_e4036_d_n11;
        locals.var_tmf2_dn12 = assign6100_e4036_d_n12;
        locals.var_tmf2_dn17 = assign6100_e4036_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6110_e4045, assign6110_e4045_d_n0, assign6110_e4045_d_n2, assign6110_e4045_d_n6, assign6110_e4045_d_n7, assign6110_e4045_d_n10, assign6110_e4045_d_n11, assign6110_e4045_d_n12, assign6110_e4045_d_n17,) = {
    if (locals.var_guard72 != 0.0) {
        let assign6110_e4040: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign6110_e4042: f64 = (assign6110_e4040 + locals.var_tmf2);
        let assign6110_e4043: f64 = (assign6110_e4042).sqrt();
        (assign6110_e4043, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign6110_e4043)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign6110_e4043)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign6110_e4043)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign6110_e4043)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign6110_e4043)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign6110_e4043)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign6110_e4043)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign6110_e4043)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6110_e4045;
        locals.var_tmf2_dn0 = assign6110_e4045_d_n0;
        locals.var_tmf2_dn2 = assign6110_e4045_d_n2;
        locals.var_tmf2_dn6 = assign6110_e4045_d_n6;
        locals.var_tmf2_dn7 = assign6110_e4045_d_n7;
        locals.var_tmf2_dn10 = assign6110_e4045_d_n10;
        locals.var_tmf2_dn11 = assign6110_e4045_d_n11;
        locals.var_tmf2_dn12 = assign6110_e4045_d_n12;
        locals.var_tmf2_dn17 = assign6110_e4045_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6120_e4055, assign6120_e4055_d_n0, assign6120_e4055_d_n2, assign6120_e4055_d_n6, assign6120_e4055_d_n7, assign6120_e4055_d_n10, assign6120_e4055_d_n11, assign6120_e4055_d_n12, assign6120_e4055_d_n17,) = {
    if (locals.var_guard72 != 0.0) {
        let assign6120_e4051: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign6120_e4052: f64 = (0.5 * assign6120_e4051);
        let assign6120_e4053: f64 = (locals.var_vbslim + assign6120_e4052);
        (assign6120_e4053, (locals.var_vbslim_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_vbslim_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_vbslim_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_vbslim_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_vbslim_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_vbslim_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_vbslim_dn12 + (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_vbslim_dn17 + (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_vbsz2, locals.var_vbsz2_dn0, locals.var_vbsz2_dn2, locals.var_vbsz2_dn6, locals.var_vbsz2_dn7, locals.var_vbsz2_dn10, locals.var_vbsz2_dn11, locals.var_vbsz2_dn12, locals.var_vbsz2_dn17,)
    }
};
        locals.var_vbsz2 = assign6120_e4055;
        locals.var_vbsz2_dn0 = assign6120_e4055_d_n0;
        locals.var_vbsz2_dn2 = assign6120_e4055_d_n2;
        locals.var_vbsz2_dn6 = assign6120_e4055_d_n6;
        locals.var_vbsz2_dn7 = assign6120_e4055_d_n7;
        locals.var_vbsz2_dn10 = assign6120_e4055_d_n10;
        locals.var_vbsz2_dn11 = assign6120_e4055_d_n11;
        locals.var_vbsz2_dn12 = assign6120_e4055_d_n12;
        locals.var_vbsz2_dn17 = assign6120_e4055_d_n17;
        locals.var_vbsz2_rv = 0.0;

        let assign6130_e4058: f64 = if locals.var_subversion > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard73 = assign6130_e4058;
        locals.var_guard73_rv = 0.0;

        let (assign6140_e4068, assign6140_e4068_d_n0, assign6140_e4068_d_n2, assign6140_e4068_d_n6, assign6140_e4068_d_n7, assign6140_e4068_d_n10, assign6140_e4068_d_n11, assign6140_e4068_d_n12, assign6140_e4068_d_n17,) = {
    if ((locals.var_guard72 != 0.0) && (locals.var_guard73 != 0.0)) {
        let assign6140_e4064: f64 = (locals.var_pb20 - locals.var_vbsz2);
        let assign6140_e4066: f64 = (assign6140_e4064 - 0.001);
        (assign6140_e4066, (locals.var_pb20_dn0 - locals.var_vbsz2_dn0), (locals.var_pb20_dn2 - locals.var_vbsz2_dn2), (locals.var_pb20_dn6 - locals.var_vbsz2_dn6), (locals.var_pb20_dn7 - locals.var_vbsz2_dn7), (locals.var_pb20_dn10 - locals.var_vbsz2_dn10), (locals.var_pb20_dn11 - locals.var_vbsz2_dn11), (locals.var_pb20_dn12 - locals.var_vbsz2_dn12), (locals.var_pb20_dn17 - locals.var_vbsz2_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign6140_e4068;
        locals.var_tmf1_dn0 = assign6140_e4068_d_n0;
        locals.var_tmf1_dn2 = assign6140_e4068_d_n2;
        locals.var_tmf1_dn6 = assign6140_e4068_d_n6;
        locals.var_tmf1_dn7 = assign6140_e4068_d_n7;
        locals.var_tmf1_dn10 = assign6140_e4068_d_n10;
        locals.var_tmf1_dn11 = assign6140_e4068_d_n11;
        locals.var_tmf1_dn12 = assign6140_e4068_d_n12;
        locals.var_tmf1_dn17 = assign6140_e4068_d_n17;
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6150_e4078, assign6150_e4078_d_n0, assign6150_e4078_d_n2, assign6150_e4078_d_n6, assign6150_e4078_d_n7, assign6150_e4078_d_n10, assign6150_e4078_d_n11, assign6150_e4078_d_n12, assign6150_e4078_d_n17,) = {
    if ((locals.var_guard72 != 0.0) && (locals.var_guard73 != 0.0)) {
        let assign6150_e4074: f64 = (4.0 * locals.var_pb20);
        let assign6150_e4076: f64 = (assign6150_e4074 * 0.001);
        (assign6150_e4076, ((4.0 * locals.var_pb20_dn0) * 0.001), ((4.0 * locals.var_pb20_dn2) * 0.001), ((4.0 * locals.var_pb20_dn6) * 0.001), ((4.0 * locals.var_pb20_dn7) * 0.001), ((4.0 * locals.var_pb20_dn10) * 0.001), ((4.0 * locals.var_pb20_dn11) * 0.001), ((4.0 * locals.var_pb20_dn12) * 0.001), ((4.0 * locals.var_pb20_dn17) * 0.001),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6150_e4078;
        locals.var_tmf2_dn0 = assign6150_e4078_d_n0;
        locals.var_tmf2_dn2 = assign6150_e4078_d_n2;
        locals.var_tmf2_dn6 = assign6150_e4078_d_n6;
        locals.var_tmf2_dn7 = assign6150_e4078_d_n7;
        locals.var_tmf2_dn10 = assign6150_e4078_d_n10;
        locals.var_tmf2_dn11 = assign6150_e4078_d_n11;
        locals.var_tmf2_dn12 = assign6150_e4078_d_n12;
        locals.var_tmf2_dn17 = assign6150_e4078_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6160_e4090, assign6160_e4090_d_n0, assign6160_e4090_d_n2, assign6160_e4090_d_n6, assign6160_e4090_d_n7, assign6160_e4090_d_n10, assign6160_e4090_d_n11, assign6160_e4090_d_n12, assign6160_e4090_d_n17,) = {
    if ((locals.var_guard72 != 0.0) && (locals.var_guard73 != 0.0)) {
        let (assign6160_e4088, assign6160_e4088_d_n0, assign6160_e4088_d_n2, assign6160_e4088_d_n6, assign6160_e4088_d_n7, assign6160_e4088_d_n10, assign6160_e4088_d_n11, assign6160_e4088_d_n12, assign6160_e4088_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign6160_e4087: f64 = (-locals.var_tmf2);
                (assign6160_e4087, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign6160_e4088, assign6160_e4088_d_n0, assign6160_e4088_d_n2, assign6160_e4088_d_n6, assign6160_e4088_d_n7, assign6160_e4088_d_n10, assign6160_e4088_d_n11, assign6160_e4088_d_n12, assign6160_e4088_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6160_e4090;
        locals.var_tmf2_dn0 = assign6160_e4090_d_n0;
        locals.var_tmf2_dn2 = assign6160_e4090_d_n2;
        locals.var_tmf2_dn6 = assign6160_e4090_d_n6;
        locals.var_tmf2_dn7 = assign6160_e4090_d_n7;
        locals.var_tmf2_dn10 = assign6160_e4090_d_n10;
        locals.var_tmf2_dn11 = assign6160_e4090_d_n11;
        locals.var_tmf2_dn12 = assign6160_e4090_d_n12;
        locals.var_tmf2_dn17 = assign6160_e4090_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6170_e4101, assign6170_e4101_d_n0, assign6170_e4101_d_n2, assign6170_e4101_d_n6, assign6170_e4101_d_n7, assign6170_e4101_d_n10, assign6170_e4101_d_n11, assign6170_e4101_d_n12, assign6170_e4101_d_n17,) = {
    if ((locals.var_guard72 != 0.0) && (locals.var_guard73 != 0.0)) {
        let assign6170_e4096: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign6170_e4098: f64 = (assign6170_e4096 + locals.var_tmf2);
        let assign6170_e4099: f64 = (assign6170_e4098).sqrt();
        (assign6170_e4099, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign6170_e4099)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign6170_e4099)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign6170_e4099)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign6170_e4099)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign6170_e4099)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign6170_e4099)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign6170_e4099)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign6170_e4099)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6170_e4101;
        locals.var_tmf2_dn0 = assign6170_e4101_d_n0;
        locals.var_tmf2_dn2 = assign6170_e4101_d_n2;
        locals.var_tmf2_dn6 = assign6170_e4101_d_n6;
        locals.var_tmf2_dn7 = assign6170_e4101_d_n7;
        locals.var_tmf2_dn10 = assign6170_e4101_d_n10;
        locals.var_tmf2_dn11 = assign6170_e4101_d_n11;
        locals.var_tmf2_dn12 = assign6170_e4101_d_n12;
        locals.var_tmf2_dn17 = assign6170_e4101_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6180_e4113, assign6180_e4113_d_n0, assign6180_e4113_d_n2, assign6180_e4113_d_n6, assign6180_e4113_d_n7, assign6180_e4113_d_n10, assign6180_e4113_d_n11, assign6180_e4113_d_n12, assign6180_e4113_d_n17,) = {
    if ((locals.var_guard72 != 0.0) && (locals.var_guard73 != 0.0)) {
        let assign6180_e4109: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign6180_e4110: f64 = (0.5 * assign6180_e4109);
        let assign6180_e4111: f64 = (locals.var_pb20 - assign6180_e4110);
        (assign6180_e4111, (locals.var_pb20_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_pb20_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_pb20_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_pb20_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_pb20_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_pb20_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_pb20_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_pb20_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_vbsz2, locals.var_vbsz2_dn0, locals.var_vbsz2_dn2, locals.var_vbsz2_dn6, locals.var_vbsz2_dn7, locals.var_vbsz2_dn10, locals.var_vbsz2_dn11, locals.var_vbsz2_dn12, locals.var_vbsz2_dn17,)
    }
};
        locals.var_vbsz2 = assign6180_e4113;
        locals.var_vbsz2_dn0 = assign6180_e4113_d_n0;
        locals.var_vbsz2_dn2 = assign6180_e4113_d_n2;
        locals.var_vbsz2_dn6 = assign6180_e4113_d_n6;
        locals.var_vbsz2_dn7 = assign6180_e4113_d_n7;
        locals.var_vbsz2_dn10 = assign6180_e4113_d_n10;
        locals.var_vbsz2_dn11 = assign6180_e4113_d_n11;
        locals.var_vbsz2_dn12 = assign6180_e4113_d_n12;
        locals.var_vbsz2_dn17 = assign6180_e4113_d_n17;
        locals.var_vbsz2_rv = 0.0;

        let (assign6190_e4118, assign6190_e4118_d_n0, assign6190_e4118_d_n2, assign6190_e4118_d_n6, assign6190_e4118_d_n7, assign6190_e4118_d_n10, assign6190_e4118_d_n11, assign6190_e4118_d_n12, assign6190_e4118_d_n17,) = {
    if (locals.var_guard72 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbsz2, locals.var_vbsz2_dn0, locals.var_vbsz2_dn2, locals.var_vbsz2_dn6, locals.var_vbsz2_dn7, locals.var_vbsz2_dn10, locals.var_vbsz2_dn11, locals.var_vbsz2_dn12, locals.var_vbsz2_dn17,)
    }
};
        locals.var_vbsz2 = assign6190_e4118;
        locals.var_vbsz2_dn0 = assign6190_e4118_d_n0;
        locals.var_vbsz2_dn2 = assign6190_e4118_d_n2;
        locals.var_vbsz2_dn6 = assign6190_e4118_d_n6;
        locals.var_vbsz2_dn7 = assign6190_e4118_d_n7;
        locals.var_vbsz2_dn10 = assign6190_e4118_d_n10;
        locals.var_vbsz2_dn11 = assign6190_e4118_d_n11;
        locals.var_vbsz2_dn12 = assign6190_e4118_d_n12;
        locals.var_vbsz2_dn17 = assign6190_e4118_d_n17;
        locals.var_vbsz2_rv = 0.0;

        let assign6200_e4121: f64 = if locals.var_subversion < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard74 = assign6200_e4121;
        locals.var_guard74_rv = 0.0;

        let (assign6210_e4125, assign6210_e4125_d_n0, assign6210_e4125_d_n2, assign6210_e4125_d_n6, assign6210_e4125_d_n7, assign6210_e4125_d_n10, assign6210_e4125_d_n11, assign6210_e4125_d_n12, assign6210_e4125_d_n17,) = {
    if (locals.var_guard74 != 0.0) {
        (p.p237, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wd0, locals.var_wd0_dn0, locals.var_wd0_dn2, locals.var_wd0_dn6, locals.var_wd0_dn7, locals.var_wd0_dn10, locals.var_wd0_dn11, locals.var_wd0_dn12, locals.var_wd0_dn17,)
    }
};
        locals.var_wd0 = assign6210_e4125;
        locals.var_wd0_dn0 = assign6210_e4125_d_n0;
        locals.var_wd0_dn2 = assign6210_e4125_d_n2;
        locals.var_wd0_dn6 = assign6210_e4125_d_n6;
        locals.var_wd0_dn7 = assign6210_e4125_d_n7;
        locals.var_wd0_dn10 = assign6210_e4125_d_n10;
        locals.var_wd0_dn11 = assign6210_e4125_d_n11;
        locals.var_wd0_dn12 = assign6210_e4125_d_n12;
        locals.var_wd0_dn17 = assign6210_e4125_d_n17;
        locals.var_wd0_rv = 0.0;

        let (assign6220_e4134, assign6220_e4134_d_n0, assign6220_e4134_d_n2, assign6220_e4134_d_n6, assign6220_e4134_d_n7, assign6220_e4134_d_n10, assign6220_e4134_d_n11, assign6220_e4134_d_n12, assign6220_e4134_d_n17,) = {
    if (locals.var_guard74 == 0.0) {
        let assign6220_e4130: f64 = (2.0 * 1.034943e-10);
        let assign6220_e4132: f64 = (assign6220_e4130 / locals.var_q_nsub);
        (assign6220_e4132, (-((assign6220_e4130 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))), (-((assign6220_e4130 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))), (-((assign6220_e4130 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))), (-((assign6220_e4130 * locals.var_q_nsub_dn7) / (locals.var_q_nsub * locals.var_q_nsub))), (-((assign6220_e4130 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))), (-((assign6220_e4130 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))), (-((assign6220_e4130 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))), (-((assign6220_e4130 * locals.var_q_nsub_dn17) / (locals.var_q_nsub * locals.var_q_nsub))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign6220_e4134;
        locals.var_t1_dn0 = assign6220_e4134_d_n0;
        locals.var_t1_dn2 = assign6220_e4134_d_n2;
        locals.var_t1_dn6 = assign6220_e4134_d_n6;
        locals.var_t1_dn7 = assign6220_e4134_d_n7;
        locals.var_t1_dn10 = assign6220_e4134_d_n10;
        locals.var_t1_dn11 = assign6220_e4134_d_n11;
        locals.var_t1_dn12 = assign6220_e4134_d_n12;
        locals.var_t1_dn17 = assign6220_e4134_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign6230_e4144, assign6230_e4144_d_n0, assign6230_e4144_d_n2, assign6230_e4144_d_n6, assign6230_e4144_d_n7, assign6230_e4144_d_n10, assign6230_e4144_d_n11, assign6230_e4144_d_n12, assign6230_e4144_d_n17,) = {
    if (locals.var_guard74 == 0.0) {
        let assign6230_e4140: f64 = (locals.var_pb20 - locals.var_vbsz2);
        let assign6230_e4141: f64 = (locals.var_t1 * assign6230_e4140);
        let assign6230_e4142: f64 = (assign6230_e4141).sqrt();
        (assign6230_e4142, (((locals.var_t1_dn0 * assign6230_e4140) + (locals.var_t1 * (locals.var_pb20_dn0 - locals.var_vbsz2_dn0))) / (2.0 * assign6230_e4142)), (((locals.var_t1_dn2 * assign6230_e4140) + (locals.var_t1 * (locals.var_pb20_dn2 - locals.var_vbsz2_dn2))) / (2.0 * assign6230_e4142)), (((locals.var_t1_dn6 * assign6230_e4140) + (locals.var_t1 * (locals.var_pb20_dn6 - locals.var_vbsz2_dn6))) / (2.0 * assign6230_e4142)), (((locals.var_t1_dn7 * assign6230_e4140) + (locals.var_t1 * (locals.var_pb20_dn7 - locals.var_vbsz2_dn7))) / (2.0 * assign6230_e4142)), (((locals.var_t1_dn10 * assign6230_e4140) + (locals.var_t1 * (locals.var_pb20_dn10 - locals.var_vbsz2_dn10))) / (2.0 * assign6230_e4142)), (((locals.var_t1_dn11 * assign6230_e4140) + (locals.var_t1 * (locals.var_pb20_dn11 - locals.var_vbsz2_dn11))) / (2.0 * assign6230_e4142)), (((locals.var_t1_dn12 * assign6230_e4140) + (locals.var_t1 * (locals.var_pb20_dn12 - locals.var_vbsz2_dn12))) / (2.0 * assign6230_e4142)), (((locals.var_t1_dn17 * assign6230_e4140) + (locals.var_t1 * (locals.var_pb20_dn17 - locals.var_vbsz2_dn17))) / (2.0 * assign6230_e4142)),)
    } else {
        (locals.var_wd0, locals.var_wd0_dn0, locals.var_wd0_dn2, locals.var_wd0_dn6, locals.var_wd0_dn7, locals.var_wd0_dn10, locals.var_wd0_dn11, locals.var_wd0_dn12, locals.var_wd0_dn17,)
    }
};
        locals.var_wd0 = assign6230_e4144;
        locals.var_wd0_dn0 = assign6230_e4144_d_n0;
        locals.var_wd0_dn2 = assign6230_e4144_d_n2;
        locals.var_wd0_dn6 = assign6230_e4144_d_n6;
        locals.var_wd0_dn7 = assign6230_e4144_d_n7;
        locals.var_wd0_dn10 = assign6230_e4144_d_n10;
        locals.var_wd0_dn11 = assign6230_e4144_d_n11;
        locals.var_wd0_dn12 = assign6230_e4144_d_n12;
        locals.var_wd0_dn17 = assign6230_e4144_d_n17;
        locals.var_wd0_rv = 0.0;

        let (assign6240_e4158, assign6240_e4158_d_n0, assign6240_e4158_d_n2, assign6240_e4158_d_n6, assign6240_e4158_d_n7, assign6240_e4158_d_n10, assign6240_e4158_d_n11, assign6240_e4158_d_n12, assign6240_e4158_d_n17,) = {
    if (locals.var_subversion < 3.0) {
        let assign6240_e4150: f64 = (locals.var_qnsub_esi2 * locals.var_pb20);
        let assign6240_e4151: f64 = (assign6240_e4150).sqrt();
        (assign6240_e4151, (((locals.var_qnsub_esi2_dn0 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn0)) / (2.0 * assign6240_e4151)), (((locals.var_qnsub_esi2_dn2 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn2)) / (2.0 * assign6240_e4151)), (((locals.var_qnsub_esi2_dn6 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn6)) / (2.0 * assign6240_e4151)), (((locals.var_qnsub_esi2_dn7 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn7)) / (2.0 * assign6240_e4151)), (((locals.var_qnsub_esi2_dn10 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn10)) / (2.0 * assign6240_e4151)), (((locals.var_qnsub_esi2_dn11 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn11)) / (2.0 * assign6240_e4151)), (((locals.var_qnsub_esi2_dn12 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn12)) / (2.0 * assign6240_e4151)), (((locals.var_qnsub_esi2_dn17 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn17)) / (2.0 * assign6240_e4151)),)
    } else {
        let assign6240_e4155: f64 = (locals.var_pb20 - locals.var_vbsz2);
        let assign6240_e4156: f64 = (locals.var_qnsub_esi2 * assign6240_e4155);
        let assign6240_e4157: f64 = (assign6240_e4156).sqrt();
        (assign6240_e4157, (((locals.var_qnsub_esi2_dn0 * assign6240_e4155) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn0 - locals.var_vbsz2_dn0))) / (2.0 * assign6240_e4157)), (((locals.var_qnsub_esi2_dn2 * assign6240_e4155) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn2 - locals.var_vbsz2_dn2))) / (2.0 * assign6240_e4157)), (((locals.var_qnsub_esi2_dn6 * assign6240_e4155) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn6 - locals.var_vbsz2_dn6))) / (2.0 * assign6240_e4157)), (((locals.var_qnsub_esi2_dn7 * assign6240_e4155) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn7 - locals.var_vbsz2_dn7))) / (2.0 * assign6240_e4157)), (((locals.var_qnsub_esi2_dn10 * assign6240_e4155) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn10 - locals.var_vbsz2_dn10))) / (2.0 * assign6240_e4157)), (((locals.var_qnsub_esi2_dn11 * assign6240_e4155) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn11 - locals.var_vbsz2_dn11))) / (2.0 * assign6240_e4157)), (((locals.var_qnsub_esi2_dn12 * assign6240_e4155) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn12 - locals.var_vbsz2_dn12))) / (2.0 * assign6240_e4157)), (((locals.var_qnsub_esi2_dn17 * assign6240_e4155) + (locals.var_qnsub_esi2 * (locals.var_pb20_dn17 - locals.var_vbsz2_dn17))) / (2.0 * assign6240_e4157)),)
    }
};
        locals.var_qb0 = assign6240_e4158;
        locals.var_qb0_dn0 = assign6240_e4158_d_n0;
        locals.var_qb0_dn2 = assign6240_e4158_d_n2;
        locals.var_qb0_dn6 = assign6240_e4158_d_n6;
        locals.var_qb0_dn7 = assign6240_e4158_d_n7;
        locals.var_qb0_dn10 = assign6240_e4158_d_n10;
        locals.var_qb0_dn11 = assign6240_e4158_d_n11;
        locals.var_qb0_dn12 = assign6240_e4158_d_n12;
        locals.var_qb0_dn17 = assign6240_e4158_d_n17;
        locals.var_qb0_rv = 0.0;

        let assign6250_e4161: f64 = (locals.var_pb20 + locals.var_vfb);
        let assign6250_e4164: f64 = (locals.var_qb0 * locals.var_c_fox_inv);
        let assign6250_e4165: f64 = (assign6250_e4161 + assign6250_e4164);
        let assign6250_e4167: f64 = (assign6250_e4165 + locals.var_ptovr);
        locals.var_vthp = assign6250_e4167;
        locals.var_vthp_dn0 = ((locals.var_pb20_dn0 + ((locals.var_qb0_dn0 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn0))) + locals.var_ptovr_dn0);
        locals.var_vthp_dn2 = ((locals.var_pb20_dn2 + ((locals.var_qb0_dn2 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn2))) + locals.var_ptovr_dn2);
        locals.var_vthp_dn6 = ((locals.var_pb20_dn6 + ((locals.var_qb0_dn6 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn6))) + locals.var_ptovr_dn6);
        locals.var_vthp_dn7 = ((locals.var_pb20_dn7 + ((locals.var_qb0_dn7 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn7))) + locals.var_ptovr_dn7);
        locals.var_vthp_dn10 = ((locals.var_pb20_dn10 + ((locals.var_qb0_dn10 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn10))) + locals.var_ptovr_dn10);
        locals.var_vthp_dn11 = ((locals.var_pb20_dn11 + ((locals.var_qb0_dn11 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn11))) + locals.var_ptovr_dn11);
        locals.var_vthp_dn12 = ((locals.var_pb20_dn12 + ((locals.var_qb0_dn12 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn12))) + locals.var_ptovr_dn12);
        locals.var_vthp_dn17 = ((locals.var_pb20_dn17 + ((locals.var_qb0_dn17 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn17))) + locals.var_ptovr_dn17);
        locals.var_vthp_rv = 0.0;

        locals.var_pb20b = locals.var_pb20;
        locals.var_pb20b_dn0 = locals.var_pb20_dn0;
        locals.var_pb20b_dn2 = locals.var_pb20_dn2;
        locals.var_pb20b_dn6 = locals.var_pb20_dn6;
        locals.var_pb20b_dn7 = locals.var_pb20_dn7;
        locals.var_pb20b_dn10 = locals.var_pb20_dn10;
        locals.var_pb20b_dn11 = locals.var_pb20_dn11;
        locals.var_pb20b_dn12 = locals.var_pb20_dn12;
        locals.var_pb20b_dn17 = locals.var_pb20_dn17;
        locals.var_pb20b_rv = 0.0;

        locals.var_t0__blk76 = 0.95;
        locals.var_t0__blk76_rv = 0.0;

        let assign6280_e4172: f64 = (locals.var_t0__blk76 * locals.var_pb20b);
        let assign6280_e4174: f64 = (assign6280_e4172 - locals.var_vbsz2);
        let assign6280_e4176: f64 = (assign6280_e4174 - 0.001);
        locals.var_t1__blk75 = assign6280_e4176;
        locals.var_t1__blk75_dn0 = ((locals.var_t0__blk76 * locals.var_pb20b_dn0) - locals.var_vbsz2_dn0);
        locals.var_t1__blk75_dn2 = ((locals.var_t0__blk76 * locals.var_pb20b_dn2) - locals.var_vbsz2_dn2);
        locals.var_t1__blk75_dn6 = ((locals.var_t0__blk76 * locals.var_pb20b_dn6) - locals.var_vbsz2_dn6);
        locals.var_t1__blk75_dn7 = ((locals.var_t0__blk76 * locals.var_pb20b_dn7) - locals.var_vbsz2_dn7);
        locals.var_t1__blk75_dn10 = ((locals.var_t0__blk76 * locals.var_pb20b_dn10) - locals.var_vbsz2_dn10);
        locals.var_t1__blk75_dn11 = ((locals.var_t0__blk76 * locals.var_pb20b_dn11) - locals.var_vbsz2_dn11);
        locals.var_t1__blk75_dn12 = ((locals.var_t0__blk76 * locals.var_pb20b_dn12) - locals.var_vbsz2_dn12);
        locals.var_t1__blk75_dn17 = ((locals.var_t0__blk76 * locals.var_pb20b_dn17) - locals.var_vbsz2_dn17);
        locals.var_t1__blk75_rv = 0.0;

        let assign6290_e4179: f64 = (locals.var_t1__blk75 * locals.var_t1__blk75);
        let assign6290_e4182: f64 = (4.0 * locals.var_t0__blk76);
        let assign6290_e4184: f64 = (assign6290_e4182 * locals.var_pb20b);
        let assign6290_e4186: f64 = (assign6290_e4184 * 0.001);
        let assign6290_e4187: f64 = (assign6290_e4179 + assign6290_e4186);
        let assign6290_e4188: f64 = (assign6290_e4187).sqrt();
        locals.var_t2__blk77 = assign6290_e4188;
        locals.var_t2__blk77_dn0 = ((((locals.var_t1__blk75_dn0 * locals.var_t1__blk75) + (locals.var_t1__blk75 * locals.var_t1__blk75_dn0)) + ((assign6290_e4182 * locals.var_pb20b_dn0) * 0.001)) / (2.0 * assign6290_e4188));
        locals.var_t2__blk77_dn2 = ((((locals.var_t1__blk75_dn2 * locals.var_t1__blk75) + (locals.var_t1__blk75 * locals.var_t1__blk75_dn2)) + ((assign6290_e4182 * locals.var_pb20b_dn2) * 0.001)) / (2.0 * assign6290_e4188));
        locals.var_t2__blk77_dn6 = ((((locals.var_t1__blk75_dn6 * locals.var_t1__blk75) + (locals.var_t1__blk75 * locals.var_t1__blk75_dn6)) + ((assign6290_e4182 * locals.var_pb20b_dn6) * 0.001)) / (2.0 * assign6290_e4188));
        locals.var_t2__blk77_dn7 = ((((locals.var_t1__blk75_dn7 * locals.var_t1__blk75) + (locals.var_t1__blk75 * locals.var_t1__blk75_dn7)) + ((assign6290_e4182 * locals.var_pb20b_dn7) * 0.001)) / (2.0 * assign6290_e4188));
        locals.var_t2__blk77_dn10 = ((((locals.var_t1__blk75_dn10 * locals.var_t1__blk75) + (locals.var_t1__blk75 * locals.var_t1__blk75_dn10)) + ((assign6290_e4182 * locals.var_pb20b_dn10) * 0.001)) / (2.0 * assign6290_e4188));
        locals.var_t2__blk77_dn11 = ((((locals.var_t1__blk75_dn11 * locals.var_t1__blk75) + (locals.var_t1__blk75 * locals.var_t1__blk75_dn11)) + ((assign6290_e4182 * locals.var_pb20b_dn11) * 0.001)) / (2.0 * assign6290_e4188));
        locals.var_t2__blk77_dn12 = ((((locals.var_t1__blk75_dn12 * locals.var_t1__blk75) + (locals.var_t1__blk75 * locals.var_t1__blk75_dn12)) + ((assign6290_e4182 * locals.var_pb20b_dn12) * 0.001)) / (2.0 * assign6290_e4188));
        locals.var_t2__blk77_dn17 = ((((locals.var_t1__blk75_dn17 * locals.var_t1__blk75) + (locals.var_t1__blk75 * locals.var_t1__blk75_dn17)) + ((assign6290_e4182 * locals.var_pb20b_dn17) * 0.001)) / (2.0 * assign6290_e4188));
        locals.var_t2__blk77_rv = 0.0;

        let assign6300_e4191: f64 = (locals.var_t0__blk76 * locals.var_pb20b);
        let assign6300_e4195: f64 = (locals.var_t1__blk75 + locals.var_t2__blk77);
        let assign6300_e4196: f64 = (0.5 * assign6300_e4195);
        let assign6300_e4197: f64 = (assign6300_e4191 - assign6300_e4196);
        locals.var_t3__blk78 = assign6300_e4197;
        locals.var_t3__blk78_dn0 = ((locals.var_t0__blk76 * locals.var_pb20b_dn0) - (0.5 * (locals.var_t1__blk75_dn0 + locals.var_t2__blk77_dn0)));
        locals.var_t3__blk78_dn2 = ((locals.var_t0__blk76 * locals.var_pb20b_dn2) - (0.5 * (locals.var_t1__blk75_dn2 + locals.var_t2__blk77_dn2)));
        locals.var_t3__blk78_dn6 = ((locals.var_t0__blk76 * locals.var_pb20b_dn6) - (0.5 * (locals.var_t1__blk75_dn6 + locals.var_t2__blk77_dn6)));
        locals.var_t3__blk78_dn7 = ((locals.var_t0__blk76 * locals.var_pb20b_dn7) - (0.5 * (locals.var_t1__blk75_dn7 + locals.var_t2__blk77_dn7)));
        locals.var_t3__blk78_dn10 = ((locals.var_t0__blk76 * locals.var_pb20b_dn10) - (0.5 * (locals.var_t1__blk75_dn10 + locals.var_t2__blk77_dn10)));
        locals.var_t3__blk78_dn11 = ((locals.var_t0__blk76 * locals.var_pb20b_dn11) - (0.5 * (locals.var_t1__blk75_dn11 + locals.var_t2__blk77_dn11)));
        locals.var_t3__blk78_dn12 = ((locals.var_t0__blk76 * locals.var_pb20b_dn12) - (0.5 * (locals.var_t1__blk75_dn12 + locals.var_t2__blk77_dn12)));
        locals.var_t3__blk78_dn17 = ((locals.var_t0__blk76 * locals.var_pb20b_dn17) - (0.5 * (locals.var_t1__blk75_dn17 + locals.var_t2__blk77_dn17)));
        locals.var_t3__blk78_rv = 0.0;

        let assign6310_e4200: f64 = (locals.var_pb20b - locals.var_t3__blk78);
        locals.var_pbsum = assign6310_e4200;
        locals.var_pbsum_dn0 = (locals.var_pb20b_dn0 - locals.var_t3__blk78_dn0);
        locals.var_pbsum_dn2 = (locals.var_pb20b_dn2 - locals.var_t3__blk78_dn2);
        locals.var_pbsum_dn6 = (locals.var_pb20b_dn6 - locals.var_t3__blk78_dn6);
        locals.var_pbsum_dn7 = (locals.var_pb20b_dn7 - locals.var_t3__blk78_dn7);
        locals.var_pbsum_dn10 = (locals.var_pb20b_dn10 - locals.var_t3__blk78_dn10);
        locals.var_pbsum_dn11 = (locals.var_pb20b_dn11 - locals.var_t3__blk78_dn11);
        locals.var_pbsum_dn12 = (locals.var_pb20b_dn12 - locals.var_t3__blk78_dn12);
        locals.var_pbsum_dn17 = (locals.var_pb20b_dn17 - locals.var_t3__blk78_dn17);
        locals.var_pbsum_rv = 0.0;

        let assign6320_e4202: f64 = (locals.var_pbsum).sqrt();
        locals.var_sqrt_pbsum = assign6320_e4202;
        locals.var_sqrt_pbsum_dn0 = (locals.var_pbsum_dn0 / (2.0 * assign6320_e4202));
        locals.var_sqrt_pbsum_dn2 = (locals.var_pbsum_dn2 / (2.0 * assign6320_e4202));
        locals.var_sqrt_pbsum_dn6 = (locals.var_pbsum_dn6 / (2.0 * assign6320_e4202));
        locals.var_sqrt_pbsum_dn7 = (locals.var_pbsum_dn7 / (2.0 * assign6320_e4202));
        locals.var_sqrt_pbsum_dn10 = (locals.var_pbsum_dn10 / (2.0 * assign6320_e4202));
        locals.var_sqrt_pbsum_dn11 = (locals.var_pbsum_dn11 / (2.0 * assign6320_e4202));
        locals.var_sqrt_pbsum_dn12 = (locals.var_pbsum_dn12 / (2.0 * assign6320_e4202));
        locals.var_sqrt_pbsum_dn17 = (locals.var_pbsum_dn17 / (2.0 * assign6320_e4202));
        locals.var_sqrt_pbsum_rv = 0.0;

        let assign6330_e4205: f64 = if p.p72 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign6330_e4205;
        locals.var_guard86_rv = 0.0;

        let (assign6340_e4215, assign6340_e4215_d_n0, assign6340_e4215_d_n2, assign6340_e4215_d_n6, assign6340_e4215_d_n7, assign6340_e4215_d_n10, assign6340_e4215_d_n11, assign6340_e4215_d_n12, assign6340_e4215_d_n17,) = {
    if (locals.var_guard86 != 0.0) {
        let assign6340_e4209: f64 = (2.0 * 1.6021918e-19);
        let assign6340_e4211: f64 = (assign6340_e4209 * locals.var_uc_nsubs);
        let assign6340_e4213: f64 = (assign6340_e4211 * 1.034943e-10);
        (assign6340_e4213, ((assign6340_e4209 * locals.var_uc_nsubs_dn0) * 1.034943e-10), ((assign6340_e4209 * locals.var_uc_nsubs_dn2) * 1.034943e-10), ((assign6340_e4209 * locals.var_uc_nsubs_dn6) * 1.034943e-10), ((assign6340_e4209 * locals.var_uc_nsubs_dn7) * 1.034943e-10), ((assign6340_e4209 * locals.var_uc_nsubs_dn10) * 1.034943e-10), ((assign6340_e4209 * locals.var_uc_nsubs_dn11) * 1.034943e-10), ((assign6340_e4209 * locals.var_uc_nsubs_dn12) * 1.034943e-10), ((assign6340_e4209 * locals.var_uc_nsubs_dn17) * 1.034943e-10),)
    } else {
        (locals.var_t1__blk80, locals.var_t1__blk80_dn0, locals.var_t1__blk80_dn2, locals.var_t1__blk80_dn6, locals.var_t1__blk80_dn7, locals.var_t1__blk80_dn10, locals.var_t1__blk80_dn11, locals.var_t1__blk80_dn12, locals.var_t1__blk80_dn17,)
    }
};
        locals.var_t1__blk80 = assign6340_e4215;
        locals.var_t1__blk80_dn0 = assign6340_e4215_d_n0;
        locals.var_t1__blk80_dn2 = assign6340_e4215_d_n2;
        locals.var_t1__blk80_dn6 = assign6340_e4215_d_n6;
        locals.var_t1__blk80_dn7 = assign6340_e4215_d_n7;
        locals.var_t1__blk80_dn10 = assign6340_e4215_d_n10;
        locals.var_t1__blk80_dn11 = assign6340_e4215_d_n11;
        locals.var_t1__blk80_dn12 = assign6340_e4215_d_n12;
        locals.var_t1__blk80_dn17 = assign6340_e4215_d_n17;
        locals.var_t1__blk80_rv = 0.0;

        let (assign6350_e4232, assign6350_e4232_d_n0, assign6350_e4232_d_n2, assign6350_e4232_d_n6, assign6350_e4232_d_n7, assign6350_e4232_d_n10, assign6350_e4232_d_n11, assign6350_e4232_d_n12, assign6350_e4232_d_n17,) = {
    if (locals.var_guard86 != 0.0) {
        let (assign6350_e4230, assign6350_e4230_d_n0, assign6350_e4230_d_n2, assign6350_e4230_d_n6, assign6350_e4230_d_n7, assign6350_e4230_d_n10, assign6350_e4230_d_n11, assign6350_e4230_d_n12, assign6350_e4230_d_n17,) = {
            if (locals.var_subversion < 3.0) {
                let assign6350_e4222: f64 = (locals.var_t1__blk80 * locals.var_pb2c);
                let assign6350_e4223: f64 = (assign6350_e4222).sqrt();
                (assign6350_e4223, (((locals.var_t1__blk80_dn0 * locals.var_pb2c) + (locals.var_t1__blk80 * locals.var_pb2c_dn0)) / (2.0 * assign6350_e4223)), (((locals.var_t1__blk80_dn2 * locals.var_pb2c) + (locals.var_t1__blk80 * locals.var_pb2c_dn2)) / (2.0 * assign6350_e4223)), (((locals.var_t1__blk80_dn6 * locals.var_pb2c) + (locals.var_t1__blk80 * locals.var_pb2c_dn6)) / (2.0 * assign6350_e4223)), (((locals.var_t1__blk80_dn7 * locals.var_pb2c) + (locals.var_t1__blk80 * locals.var_pb2c_dn7)) / (2.0 * assign6350_e4223)), (((locals.var_t1__blk80_dn10 * locals.var_pb2c) + (locals.var_t1__blk80 * locals.var_pb2c_dn10)) / (2.0 * assign6350_e4223)), (((locals.var_t1__blk80_dn11 * locals.var_pb2c) + (locals.var_t1__blk80 * locals.var_pb2c_dn11)) / (2.0 * assign6350_e4223)), (((locals.var_t1__blk80_dn12 * locals.var_pb2c) + (locals.var_t1__blk80 * locals.var_pb2c_dn12)) / (2.0 * assign6350_e4223)), (((locals.var_t1__blk80_dn17 * locals.var_pb2c) + (locals.var_t1__blk80 * locals.var_pb2c_dn17)) / (2.0 * assign6350_e4223)),)
            } else {
                let assign6350_e4227: f64 = (locals.var_pb2c - locals.var_vbsz2);
                let assign6350_e4228: f64 = (locals.var_t1__blk80 * assign6350_e4227);
                let assign6350_e4229: f64 = (assign6350_e4228).sqrt();
                (assign6350_e4229, (((locals.var_t1__blk80_dn0 * assign6350_e4227) + (locals.var_t1__blk80 * (locals.var_pb2c_dn0 - locals.var_vbsz2_dn0))) / (2.0 * assign6350_e4229)), (((locals.var_t1__blk80_dn2 * assign6350_e4227) + (locals.var_t1__blk80 * (locals.var_pb2c_dn2 - locals.var_vbsz2_dn2))) / (2.0 * assign6350_e4229)), (((locals.var_t1__blk80_dn6 * assign6350_e4227) + (locals.var_t1__blk80 * (locals.var_pb2c_dn6 - locals.var_vbsz2_dn6))) / (2.0 * assign6350_e4229)), (((locals.var_t1__blk80_dn7 * assign6350_e4227) + (locals.var_t1__blk80 * (locals.var_pb2c_dn7 - locals.var_vbsz2_dn7))) / (2.0 * assign6350_e4229)), (((locals.var_t1__blk80_dn10 * assign6350_e4227) + (locals.var_t1__blk80 * (locals.var_pb2c_dn10 - locals.var_vbsz2_dn10))) / (2.0 * assign6350_e4229)), (((locals.var_t1__blk80_dn11 * assign6350_e4227) + (locals.var_t1__blk80 * (locals.var_pb2c_dn11 - locals.var_vbsz2_dn11))) / (2.0 * assign6350_e4229)), (((locals.var_t1__blk80_dn12 * assign6350_e4227) + (locals.var_t1__blk80 * (locals.var_pb2c_dn12 - locals.var_vbsz2_dn12))) / (2.0 * assign6350_e4229)), (((locals.var_t1__blk80_dn17 * assign6350_e4227) + (locals.var_t1__blk80 * (locals.var_pb2c_dn17 - locals.var_vbsz2_dn17))) / (2.0 * assign6350_e4229)),)
            }
        };
        (assign6350_e4230, assign6350_e4230_d_n0, assign6350_e4230_d_n2, assign6350_e4230_d_n6, assign6350_e4230_d_n7, assign6350_e4230_d_n10, assign6350_e4230_d_n11, assign6350_e4230_d_n12, assign6350_e4230_d_n17,)
    } else {
        (locals.var_t2__blk81, locals.var_t2__blk81_dn0, locals.var_t2__blk81_dn2, locals.var_t2__blk81_dn6, locals.var_t2__blk81_dn7, locals.var_t2__blk81_dn10, locals.var_t2__blk81_dn11, locals.var_t2__blk81_dn12, locals.var_t2__blk81_dn17,)
    }
};
        locals.var_t2__blk81 = assign6350_e4232;
        locals.var_t2__blk81_dn0 = assign6350_e4232_d_n0;
        locals.var_t2__blk81_dn2 = assign6350_e4232_d_n2;
        locals.var_t2__blk81_dn6 = assign6350_e4232_d_n6;
        locals.var_t2__blk81_dn7 = assign6350_e4232_d_n7;
        locals.var_t2__blk81_dn10 = assign6350_e4232_d_n10;
        locals.var_t2__blk81_dn11 = assign6350_e4232_d_n11;
        locals.var_t2__blk81_dn12 = assign6350_e4232_d_n12;
        locals.var_t2__blk81_dn17 = assign6350_e4232_d_n17;
        locals.var_t2__blk81_rv = 0.0;

        let (assign6360_e4242, assign6360_e4242_d_n0, assign6360_e4242_d_n2, assign6360_e4242_d_n6, assign6360_e4242_d_n7, assign6360_e4242_d_n10, assign6360_e4242_d_n11, assign6360_e4242_d_n12, assign6360_e4242_d_n17,) = {
    if (locals.var_guard86 != 0.0) {
        let assign6360_e4236: f64 = (locals.var_pb2c + locals.var_vfb);
        let assign6360_e4239: f64 = (locals.var_t2__blk81 * locals.var_c_fox_inv);
        let assign6360_e4240: f64 = (assign6360_e4236 + assign6360_e4239);
        (assign6360_e4240, (locals.var_pb2c_dn0 + ((locals.var_t2__blk81_dn0 * locals.var_c_fox_inv) + (locals.var_t2__blk81 * locals.var_c_fox_inv_dn0))), (locals.var_pb2c_dn2 + ((locals.var_t2__blk81_dn2 * locals.var_c_fox_inv) + (locals.var_t2__blk81 * locals.var_c_fox_inv_dn2))), (locals.var_pb2c_dn6 + ((locals.var_t2__blk81_dn6 * locals.var_c_fox_inv) + (locals.var_t2__blk81 * locals.var_c_fox_inv_dn6))), (locals.var_pb2c_dn7 + ((locals.var_t2__blk81_dn7 * locals.var_c_fox_inv) + (locals.var_t2__blk81 * locals.var_c_fox_inv_dn7))), (locals.var_pb2c_dn10 + ((locals.var_t2__blk81_dn10 * locals.var_c_fox_inv) + (locals.var_t2__blk81 * locals.var_c_fox_inv_dn10))), (locals.var_pb2c_dn11 + ((locals.var_t2__blk81_dn11 * locals.var_c_fox_inv) + (locals.var_t2__blk81 * locals.var_c_fox_inv_dn11))), (locals.var_pb2c_dn12 + ((locals.var_t2__blk81_dn12 * locals.var_c_fox_inv) + (locals.var_t2__blk81 * locals.var_c_fox_inv_dn12))), (locals.var_pb2c_dn17 + ((locals.var_t2__blk81_dn17 * locals.var_c_fox_inv) + (locals.var_t2__blk81 * locals.var_c_fox_inv_dn17))),)
    } else {
        (locals.var_vth0, locals.var_vth0_dn0, locals.var_vth0_dn2, locals.var_vth0_dn6, locals.var_vth0_dn7, locals.var_vth0_dn10, locals.var_vth0_dn11, locals.var_vth0_dn12, locals.var_vth0_dn17,)
    }
};
        locals.var_vth0 = assign6360_e4242;
        locals.var_vth0_dn0 = assign6360_e4242_d_n0;
        locals.var_vth0_dn2 = assign6360_e4242_d_n2;
        locals.var_vth0_dn6 = assign6360_e4242_d_n6;
        locals.var_vth0_dn7 = assign6360_e4242_d_n7;
        locals.var_vth0_dn10 = assign6360_e4242_d_n10;
        locals.var_vth0_dn11 = assign6360_e4242_d_n11;
        locals.var_vth0_dn12 = assign6360_e4242_d_n12;
        locals.var_vth0_dn17 = assign6360_e4242_d_n17;
        locals.var_vth0_rv = 0.0;

        let (assign6370_e4248, assign6370_e4248_d_n0, assign6370_e4248_d_n2, assign6370_e4248_d_n6, assign6370_e4248_d_n7, assign6370_e4248_d_n10, assign6370_e4248_d_n11, assign6370_e4248_d_n12, assign6370_e4248_d_n17,) = {
    if (locals.var_guard86 != 0.0) {
        let assign6370_e4246: f64 = (1.034943e-10 * locals.var_c_fox_inv);
        (assign6370_e4246, (1.034943e-10 * locals.var_c_fox_inv_dn0), (1.034943e-10 * locals.var_c_fox_inv_dn2), (1.034943e-10 * locals.var_c_fox_inv_dn6), (1.034943e-10 * locals.var_c_fox_inv_dn7), (1.034943e-10 * locals.var_c_fox_inv_dn10), (1.034943e-10 * locals.var_c_fox_inv_dn11), (1.034943e-10 * locals.var_c_fox_inv_dn12), (1.034943e-10 * locals.var_c_fox_inv_dn17),)
    } else {
        (locals.var_t1__blk80, locals.var_t1__blk80_dn0, locals.var_t1__blk80_dn2, locals.var_t1__blk80_dn6, locals.var_t1__blk80_dn7, locals.var_t1__blk80_dn10, locals.var_t1__blk80_dn11, locals.var_t1__blk80_dn12, locals.var_t1__blk80_dn17,)
    }
};
        locals.var_t1__blk80 = assign6370_e4248;
        locals.var_t1__blk80_dn0 = assign6370_e4248_d_n0;
        locals.var_t1__blk80_dn2 = assign6370_e4248_d_n2;
        locals.var_t1__blk80_dn6 = assign6370_e4248_d_n6;
        locals.var_t1__blk80_dn7 = assign6370_e4248_d_n7;
        locals.var_t1__blk80_dn10 = assign6370_e4248_d_n10;
        locals.var_t1__blk80_dn11 = assign6370_e4248_d_n11;
        locals.var_t1__blk80_dn12 = assign6370_e4248_d_n12;
        locals.var_t1__blk80_dn17 = assign6370_e4248_d_n17;
        locals.var_t1__blk80_rv = 0.0;

        let (assign6380_e4256,) = {
    if (locals.var_guard86 != 0.0) {
        let assign6380_e4253: f64 = (p.p72 * p.p72);
        let assign6380_e4254: f64 = (1.0 / assign6380_e4253);
        (assign6380_e4254,)
    } else {
        (locals.var_t4__blk83,)
    }
};
        locals.var_t4__blk83 = assign6380_e4256;
        locals.var_t4__blk83_rv = 0.0;

        let (assign6390_e4264, assign6390_e4264_d_n0, assign6390_e4264_d_n2, assign6390_e4264_d_n6, assign6390_e4264_d_n7, assign6390_e4264_d_n10, assign6390_e4264_d_n11, assign6390_e4264_d_n12, assign6390_e4264_d_n17,) = {
    if (locals.var_guard86 != 0.0) {
        let assign6390_e4260: f64 = (2.0 * locals.var_wd0);
        let assign6390_e4262: f64 = (assign6390_e4260 * locals.var_t4__blk83);
        (assign6390_e4262, ((2.0 * locals.var_wd0_dn0) * locals.var_t4__blk83), ((2.0 * locals.var_wd0_dn2) * locals.var_t4__blk83), ((2.0 * locals.var_wd0_dn6) * locals.var_t4__blk83), ((2.0 * locals.var_wd0_dn7) * locals.var_t4__blk83), ((2.0 * locals.var_wd0_dn10) * locals.var_t4__blk83), ((2.0 * locals.var_wd0_dn11) * locals.var_t4__blk83), ((2.0 * locals.var_wd0_dn12) * locals.var_t4__blk83), ((2.0 * locals.var_wd0_dn17) * locals.var_t4__blk83),)
    } else {
        (locals.var_t3__blk82, locals.var_t3__blk82_dn0, locals.var_t3__blk82_dn2, locals.var_t3__blk82_dn6, locals.var_t3__blk82_dn7, locals.var_t3__blk82_dn10, locals.var_t3__blk82_dn11, locals.var_t3__blk82_dn12, locals.var_t3__blk82_dn17,)
    }
};
        locals.var_t3__blk82 = assign6390_e4264;
        locals.var_t3__blk82_dn0 = assign6390_e4264_d_n0;
        locals.var_t3__blk82_dn2 = assign6390_e4264_d_n2;
        locals.var_t3__blk82_dn6 = assign6390_e4264_d_n6;
        locals.var_t3__blk82_dn7 = assign6390_e4264_d_n7;
        locals.var_t3__blk82_dn10 = assign6390_e4264_d_n10;
        locals.var_t3__blk82_dn11 = assign6390_e4264_d_n11;
        locals.var_t3__blk82_dn12 = assign6390_e4264_d_n12;
        locals.var_t3__blk82_dn17 = assign6390_e4264_d_n17;
        locals.var_t3__blk82_rv = 0.0;

        let (assign6400_e4274, assign6400_e4274_d_n0, assign6400_e4274_d_n2, assign6400_e4274_d_n6, assign6400_e4274_d_n7, assign6400_e4274_d_n10, assign6400_e4274_d_n11, assign6400_e4274_d_n12, assign6400_e4274_d_n17,) = {
    if (locals.var_guard86 != 0.0) {
        let assign6400_e4268: f64 = (locals.var_t1__blk80 * locals.var_t3__blk82);
        let assign6400_e4271: f64 = (p.p69 - locals.var_pb20b);
        let assign6400_e4272: f64 = (assign6400_e4268 * assign6400_e4271);
        (assign6400_e4272, ((((locals.var_t1__blk80_dn0 * locals.var_t3__blk82) + (locals.var_t1__blk80 * locals.var_t3__blk82_dn0)) * assign6400_e4271) + (assign6400_e4268 * (-locals.var_pb20b_dn0))), ((((locals.var_t1__blk80_dn2 * locals.var_t3__blk82) + (locals.var_t1__blk80 * locals.var_t3__blk82_dn2)) * assign6400_e4271) + (assign6400_e4268 * (-locals.var_pb20b_dn2))), ((((locals.var_t1__blk80_dn6 * locals.var_t3__blk82) + (locals.var_t1__blk80 * locals.var_t3__blk82_dn6)) * assign6400_e4271) + (assign6400_e4268 * (-locals.var_pb20b_dn6))), ((((locals.var_t1__blk80_dn7 * locals.var_t3__blk82) + (locals.var_t1__blk80 * locals.var_t3__blk82_dn7)) * assign6400_e4271) + (assign6400_e4268 * (-locals.var_pb20b_dn7))), ((((locals.var_t1__blk80_dn10 * locals.var_t3__blk82) + (locals.var_t1__blk80 * locals.var_t3__blk82_dn10)) * assign6400_e4271) + (assign6400_e4268 * (-locals.var_pb20b_dn10))), ((((locals.var_t1__blk80_dn11 * locals.var_t3__blk82) + (locals.var_t1__blk80 * locals.var_t3__blk82_dn11)) * assign6400_e4271) + (assign6400_e4268 * (-locals.var_pb20b_dn11))), ((((locals.var_t1__blk80_dn12 * locals.var_t3__blk82) + (locals.var_t1__blk80 * locals.var_t3__blk82_dn12)) * assign6400_e4271) + (assign6400_e4268 * (-locals.var_pb20b_dn12))), ((((locals.var_t1__blk80_dn17 * locals.var_t3__blk82) + (locals.var_t1__blk80 * locals.var_t3__blk82_dn17)) * assign6400_e4271) + (assign6400_e4268 * (-locals.var_pb20b_dn17))),)
    } else {
        (locals.var_t5__blk84, locals.var_t5__blk84_dn0, locals.var_t5__blk84_dn2, locals.var_t5__blk84_dn6, locals.var_t5__blk84_dn7, locals.var_t5__blk84_dn10, locals.var_t5__blk84_dn11, locals.var_t5__blk84_dn12, locals.var_t5__blk84_dn17,)
    }
};
        locals.var_t5__blk84 = assign6400_e4274;
        locals.var_t5__blk84_dn0 = assign6400_e4274_d_n0;
        locals.var_t5__blk84_dn2 = assign6400_e4274_d_n2;
        locals.var_t5__blk84_dn6 = assign6400_e4274_d_n6;
        locals.var_t5__blk84_dn7 = assign6400_e4274_d_n7;
        locals.var_t5__blk84_dn10 = assign6400_e4274_d_n10;
        locals.var_t5__blk84_dn11 = assign6400_e4274_d_n11;
        locals.var_t5__blk84_dn12 = assign6400_e4274_d_n12;
        locals.var_t5__blk84_dn17 = assign6400_e4274_d_n17;
        locals.var_t5__blk84_rv = 0.0;

        let (assign6410_e4278, assign6410_e4278_d_n0, assign6410_e4278_d_n2, assign6410_e4278_d_n6, assign6410_e4278_d_n7, assign6410_e4278_d_n10, assign6410_e4278_d_n11, assign6410_e4278_d_n12, assign6410_e4278_d_n17,) = {
    if (locals.var_guard86 != 0.0) {
        (locals.var_t5__blk84, locals.var_t5__blk84_dn0, locals.var_t5__blk84_dn2, locals.var_t5__blk84_dn6, locals.var_t5__blk84_dn7, locals.var_t5__blk84_dn10, locals.var_t5__blk84_dn11, locals.var_t5__blk84_dn12, locals.var_t5__blk84_dn17,)
    } else {
        (locals.var_dvth0__blk85, locals.var_dvth0__blk85_dn0, locals.var_dvth0__blk85_dn2, locals.var_dvth0__blk85_dn6, locals.var_dvth0__blk85_dn7, locals.var_dvth0__blk85_dn10, locals.var_dvth0__blk85_dn11, locals.var_dvth0__blk85_dn12, locals.var_dvth0__blk85_dn17,)
    }
};
        locals.var_dvth0__blk85 = assign6410_e4278;
        locals.var_dvth0__blk85_dn0 = assign6410_e4278_d_n0;
        locals.var_dvth0__blk85_dn2 = assign6410_e4278_d_n2;
        locals.var_dvth0__blk85_dn6 = assign6410_e4278_d_n6;
        locals.var_dvth0__blk85_dn7 = assign6410_e4278_d_n7;
        locals.var_dvth0__blk85_dn10 = assign6410_e4278_d_n10;
        locals.var_dvth0__blk85_dn11 = assign6410_e4278_d_n11;
        locals.var_dvth0__blk85_dn12 = assign6410_e4278_d_n12;
        locals.var_dvth0__blk85_dn17 = assign6410_e4278_d_n17;
        locals.var_dvth0__blk85_rv = 0.0;

        let (assign6420_e4284, assign6420_e4284_d_n0, assign6420_e4284_d_n2, assign6420_e4284_d_n6, assign6420_e4284_d_n7, assign6420_e4284_d_n10, assign6420_e4284_d_n11, assign6420_e4284_d_n12, assign6420_e4284_d_n17,) = {
    if (locals.var_guard86 != 0.0) {
        let assign6420_e4282: f64 = (locals.var_vthp - locals.var_vth0);
        (assign6420_e4282, (locals.var_vthp_dn0 - locals.var_vth0_dn0), (locals.var_vthp_dn2 - locals.var_vth0_dn2), (locals.var_vthp_dn6 - locals.var_vth0_dn6), (locals.var_vthp_dn7 - locals.var_vth0_dn7), (locals.var_vthp_dn10 - locals.var_vth0_dn10), (locals.var_vthp_dn11 - locals.var_vth0_dn11), (locals.var_vthp_dn12 - locals.var_vth0_dn12), (locals.var_vthp_dn17 - locals.var_vth0_dn17),)
    } else {
        (locals.var_t1__blk80, locals.var_t1__blk80_dn0, locals.var_t1__blk80_dn2, locals.var_t1__blk80_dn6, locals.var_t1__blk80_dn7, locals.var_t1__blk80_dn10, locals.var_t1__blk80_dn11, locals.var_t1__blk80_dn12, locals.var_t1__blk80_dn17,)
    }
};
        locals.var_t1__blk80 = assign6420_e4284;
        locals.var_t1__blk80_dn0 = assign6420_e4284_d_n0;
        locals.var_t1__blk80_dn2 = assign6420_e4284_d_n2;
        locals.var_t1__blk80_dn6 = assign6420_e4284_d_n6;
        locals.var_t1__blk80_dn7 = assign6420_e4284_d_n7;
        locals.var_t1__blk80_dn10 = assign6420_e4284_d_n10;
        locals.var_t1__blk80_dn11 = assign6420_e4284_d_n11;
        locals.var_t1__blk80_dn12 = assign6420_e4284_d_n12;
        locals.var_t1__blk80_dn17 = assign6420_e4284_d_n17;
        locals.var_t1__blk80_rv = 0.0;

        let (assign6430_e4290,) = {
    if (locals.var_guard86 != 0.0) {
        let assign6430_e4288: f64 = (locals.var_uc_scp3 / p.p72);
        (assign6430_e4288,)
    } else {
        (locals.var_t0__blk79,)
    }
};
        locals.var_t0__blk79 = assign6430_e4290;
        locals.var_t0__blk79_rv = 0.0;

        let (assign6440_e4298, assign6440_e4298_d_n0, assign6440_e4298_d_n2, assign6440_e4298_d_n6, assign6440_e4298_d_n7, assign6440_e4298_d_n10, assign6440_e4298_d_n11, assign6440_e4298_d_n12, assign6440_e4298_d_n17,) = {
    if (locals.var_guard86 != 0.0) {
        let assign6440_e4295: f64 = (locals.var_t0__blk79 * locals.var_pbsum);
        let assign6440_e4296: f64 = (p.p80 + assign6440_e4295);
        (assign6440_e4296, (locals.var_t0__blk79 * locals.var_pbsum_dn0), (locals.var_t0__blk79 * locals.var_pbsum_dn2), (locals.var_t0__blk79 * locals.var_pbsum_dn6), (locals.var_t0__blk79 * locals.var_pbsum_dn7), (locals.var_t0__blk79 * locals.var_pbsum_dn10), (locals.var_t0__blk79 * locals.var_pbsum_dn11), (locals.var_t0__blk79 * locals.var_pbsum_dn12), (locals.var_t0__blk79 * locals.var_pbsum_dn17),)
    } else {
        (locals.var_t2__blk81, locals.var_t2__blk81_dn0, locals.var_t2__blk81_dn2, locals.var_t2__blk81_dn6, locals.var_t2__blk81_dn7, locals.var_t2__blk81_dn10, locals.var_t2__blk81_dn11, locals.var_t2__blk81_dn12, locals.var_t2__blk81_dn17,)
    }
};
        locals.var_t2__blk81 = assign6440_e4298;
        locals.var_t2__blk81_dn0 = assign6440_e4298_d_n0;
        locals.var_t2__blk81_dn2 = assign6440_e4298_d_n2;
        locals.var_t2__blk81_dn6 = assign6440_e4298_d_n6;
        locals.var_t2__blk81_dn7 = assign6440_e4298_d_n7;
        locals.var_t2__blk81_dn10 = assign6440_e4298_d_n10;
        locals.var_t2__blk81_dn11 = assign6440_e4298_d_n11;
        locals.var_t2__blk81_dn12 = assign6440_e4298_d_n12;
        locals.var_t2__blk81_dn17 = assign6440_e4298_d_n17;
        locals.var_t2__blk81_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6450_e4302, assign6450_e4302_d_n0, assign6450_e4302_d_n2, assign6450_e4302_d_n6, assign6450_e4302_d_n7, assign6450_e4302_d_n10, assign6450_e4302_d_n11, assign6450_e4302_d_n12, assign6450_e4302_d_n17,) = {
    if (locals.var_guard86 != 0.0) {
        (locals.var_uc_scp2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk84, locals.var_t5__blk84_dn0, locals.var_t5__blk84_dn2, locals.var_t5__blk84_dn6, locals.var_t5__blk84_dn7, locals.var_t5__blk84_dn10, locals.var_t5__blk84_dn11, locals.var_t5__blk84_dn12, locals.var_t5__blk84_dn17,)
    }
};
        locals.var_t5__blk84 = assign6450_e4302;
        locals.var_t5__blk84_dn0 = assign6450_e4302_d_n0;
        locals.var_t5__blk84_dn2 = assign6450_e4302_d_n2;
        locals.var_t5__blk84_dn6 = assign6450_e4302_d_n6;
        locals.var_t5__blk84_dn7 = assign6450_e4302_d_n7;
        locals.var_t5__blk84_dn10 = assign6450_e4302_d_n10;
        locals.var_t5__blk84_dn11 = assign6450_e4302_d_n11;
        locals.var_t5__blk84_dn12 = assign6450_e4302_d_n12;
        locals.var_t5__blk84_dn17 = assign6450_e4302_d_n17;
        locals.var_t5__blk84_rv = 0.0;

        let (assign6460_e4310, assign6460_e4310_d_n0, assign6460_e4310_d_n2, assign6460_e4310_d_n6, assign6460_e4310_d_n7, assign6460_e4310_d_n10, assign6460_e4310_d_n11, assign6460_e4310_d_n12, assign6460_e4310_d_n17,) = {
    if (locals.var_guard86 != 0.0) {
        let assign6460_e4307: f64 = (locals.var_t5__blk84 * locals.var_vdsz);
        let assign6460_e4308: f64 = (locals.var_t2__blk81 + assign6460_e4307);
        (assign6460_e4308, (locals.var_t2__blk81_dn0 + ((locals.var_t5__blk84_dn0 * locals.var_vdsz) + (locals.var_t5__blk84 * locals.var_vdsz_dn0))), (locals.var_t2__blk81_dn2 + ((locals.var_t5__blk84_dn2 * locals.var_vdsz) + (locals.var_t5__blk84 * locals.var_vdsz_dn2))), (locals.var_t2__blk81_dn6 + ((locals.var_t5__blk84_dn6 * locals.var_vdsz) + (locals.var_t5__blk84 * locals.var_vdsz_dn6))), (locals.var_t2__blk81_dn7 + ((locals.var_t5__blk84_dn7 * locals.var_vdsz) + (locals.var_t5__blk84 * locals.var_vdsz_dn7))), (locals.var_t2__blk81_dn10 + ((locals.var_t5__blk84_dn10 * locals.var_vdsz) + (locals.var_t5__blk84 * locals.var_vdsz_dn10))), (locals.var_t2__blk81_dn11 + ((locals.var_t5__blk84_dn11 * locals.var_vdsz) + (locals.var_t5__blk84 * locals.var_vdsz_dn11))), (locals.var_t2__blk81_dn12 + ((locals.var_t5__blk84_dn12 * locals.var_vdsz) + (locals.var_t5__blk84 * locals.var_vdsz_dn12))), (locals.var_t2__blk81_dn17 + ((locals.var_t5__blk84_dn17 * locals.var_vdsz) + (locals.var_t5__blk84 * locals.var_vdsz_dn17))),)
    } else {
        (locals.var_t3__blk82, locals.var_t3__blk82_dn0, locals.var_t3__blk82_dn2, locals.var_t3__blk82_dn6, locals.var_t3__blk82_dn7, locals.var_t3__blk82_dn10, locals.var_t3__blk82_dn11, locals.var_t3__blk82_dn12, locals.var_t3__blk82_dn17,)
    }
};
        locals.var_t3__blk82 = assign6460_e4310;
        locals.var_t3__blk82_dn0 = assign6460_e4310_d_n0;
        locals.var_t3__blk82_dn2 = assign6460_e4310_d_n2;
        locals.var_t3__blk82_dn6 = assign6460_e4310_d_n6;
        locals.var_t3__blk82_dn7 = assign6460_e4310_d_n7;
        locals.var_t3__blk82_dn10 = assign6460_e4310_d_n10;
        locals.var_t3__blk82_dn11 = assign6460_e4310_d_n11;
        locals.var_t3__blk82_dn12 = assign6460_e4310_d_n12;
        locals.var_t3__blk82_dn17 = assign6460_e4310_d_n17;
        locals.var_t3__blk82_rv = 0.0;

        let (assign6470_e4318, assign6470_e4318_d_n0, assign6470_e4318_d_n2, assign6470_e4318_d_n6, assign6470_e4318_d_n7, assign6470_e4318_d_n10, assign6470_e4318_d_n11, assign6470_e4318_d_n12, assign6470_e4318_d_n17,) = {
    if (locals.var_guard86 != 0.0) {
        let assign6470_e4314: f64 = (locals.var_t1__blk80 * locals.var_dvth0__blk85);
        let assign6470_e4316: f64 = (assign6470_e4314 * locals.var_t3__blk82);
        (assign6470_e4316, ((((locals.var_t1__blk80_dn0 * locals.var_dvth0__blk85) + (locals.var_t1__blk80 * locals.var_dvth0__blk85_dn0)) * locals.var_t3__blk82) + (assign6470_e4314 * locals.var_t3__blk82_dn0)), ((((locals.var_t1__blk80_dn2 * locals.var_dvth0__blk85) + (locals.var_t1__blk80 * locals.var_dvth0__blk85_dn2)) * locals.var_t3__blk82) + (assign6470_e4314 * locals.var_t3__blk82_dn2)), ((((locals.var_t1__blk80_dn6 * locals.var_dvth0__blk85) + (locals.var_t1__blk80 * locals.var_dvth0__blk85_dn6)) * locals.var_t3__blk82) + (assign6470_e4314 * locals.var_t3__blk82_dn6)), ((((locals.var_t1__blk80_dn7 * locals.var_dvth0__blk85) + (locals.var_t1__blk80 * locals.var_dvth0__blk85_dn7)) * locals.var_t3__blk82) + (assign6470_e4314 * locals.var_t3__blk82_dn7)), ((((locals.var_t1__blk80_dn10 * locals.var_dvth0__blk85) + (locals.var_t1__blk80 * locals.var_dvth0__blk85_dn10)) * locals.var_t3__blk82) + (assign6470_e4314 * locals.var_t3__blk82_dn10)), ((((locals.var_t1__blk80_dn11 * locals.var_dvth0__blk85) + (locals.var_t1__blk80 * locals.var_dvth0__blk85_dn11)) * locals.var_t3__blk82) + (assign6470_e4314 * locals.var_t3__blk82_dn11)), ((((locals.var_t1__blk80_dn12 * locals.var_dvth0__blk85) + (locals.var_t1__blk80 * locals.var_dvth0__blk85_dn12)) * locals.var_t3__blk82) + (assign6470_e4314 * locals.var_t3__blk82_dn12)), ((((locals.var_t1__blk80_dn17 * locals.var_dvth0__blk85) + (locals.var_t1__blk80 * locals.var_dvth0__blk85_dn17)) * locals.var_t3__blk82) + (assign6470_e4314 * locals.var_t3__blk82_dn17)),)
    } else {
        (locals.var_dvthlp, locals.var_dvthlp_dn0, locals.var_dvthlp_dn2, locals.var_dvthlp_dn6, locals.var_dvthlp_dn7, locals.var_dvthlp_dn10, locals.var_dvthlp_dn11, locals.var_dvthlp_dn12, locals.var_dvthlp_dn17,)
    }
};
        locals.var_dvthlp = assign6470_e4318;
        locals.var_dvthlp_dn0 = assign6470_e4318_d_n0;
        locals.var_dvthlp_dn2 = assign6470_e4318_d_n2;
        locals.var_dvthlp_dn6 = assign6470_e4318_d_n6;
        locals.var_dvthlp_dn7 = assign6470_e4318_d_n7;
        locals.var_dvthlp_dn10 = assign6470_e4318_d_n10;
        locals.var_dvthlp_dn11 = assign6470_e4318_d_n11;
        locals.var_dvthlp_dn12 = assign6470_e4318_d_n12;
        locals.var_dvthlp_dn17 = assign6470_e4318_d_n17;
        locals.var_dvthlp_rv = 0.0;

        let (assign6480_e4323, assign6480_e4323_d_n0, assign6480_e4323_d_n2, assign6480_e4323_d_n6, assign6480_e4323_d_n7, assign6480_e4323_d_n10, assign6480_e4323_d_n11, assign6480_e4323_d_n12, assign6480_e4323_d_n17,) = {
    if (locals.var_guard86 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvthlp, locals.var_dvthlp_dn0, locals.var_dvthlp_dn2, locals.var_dvthlp_dn6, locals.var_dvthlp_dn7, locals.var_dvthlp_dn10, locals.var_dvthlp_dn11, locals.var_dvthlp_dn12, locals.var_dvthlp_dn17,)
    }
};
        locals.var_dvthlp = assign6480_e4323;
        locals.var_dvthlp_dn0 = assign6480_e4323_d_n0;
        locals.var_dvthlp_dn2 = assign6480_e4323_d_n2;
        locals.var_dvthlp_dn6 = assign6480_e4323_d_n6;
        locals.var_dvthlp_dn7 = assign6480_e4323_d_n7;
        locals.var_dvthlp_dn10 = assign6480_e4323_d_n10;
        locals.var_dvthlp_dn11 = assign6480_e4323_d_n11;
        locals.var_dvthlp_dn12 = assign6480_e4323_d_n12;
        locals.var_dvthlp_dn17 = assign6480_e4323_d_n17;
        locals.var_dvthlp_rv = 0.0;

        let assign6490_e4326: f64 = (1.034943e-10 * locals.var_wd0);
        let assign6490_e4328: f64 = (assign6490_e4326 * 2.0);
        locals.var_t0__blk87 = assign6490_e4328;
        locals.var_t0__blk87_dn0 = ((1.034943e-10 * locals.var_wd0_dn0) * 2.0);
        locals.var_t0__blk87_dn2 = ((1.034943e-10 * locals.var_wd0_dn2) * 2.0);
        locals.var_t0__blk87_dn6 = ((1.034943e-10 * locals.var_wd0_dn6) * 2.0);
        locals.var_t0__blk87_dn7 = ((1.034943e-10 * locals.var_wd0_dn7) * 2.0);
        locals.var_t0__blk87_dn10 = ((1.034943e-10 * locals.var_wd0_dn10) * 2.0);
        locals.var_t0__blk87_dn11 = ((1.034943e-10 * locals.var_wd0_dn11) * 2.0);
        locals.var_t0__blk87_dn12 = ((1.034943e-10 * locals.var_wd0_dn12) * 2.0);
        locals.var_t0__blk87_dn17 = ((1.034943e-10 * locals.var_wd0_dn17) * 2.0);
        locals.var_t0__blk87_rv = 0.0;

        let assign6500_e4331: f64 = (locals.var_c_fox_inv * locals.var_t0__blk87);
        locals.var_t1__blk88 = assign6500_e4331;
        locals.var_t1__blk88_dn0 = ((locals.var_c_fox_inv_dn0 * locals.var_t0__blk87) + (locals.var_c_fox_inv * locals.var_t0__blk87_dn0));
        locals.var_t1__blk88_dn2 = ((locals.var_c_fox_inv_dn2 * locals.var_t0__blk87) + (locals.var_c_fox_inv * locals.var_t0__blk87_dn2));
        locals.var_t1__blk88_dn6 = ((locals.var_c_fox_inv_dn6 * locals.var_t0__blk87) + (locals.var_c_fox_inv * locals.var_t0__blk87_dn6));
        locals.var_t1__blk88_dn7 = ((locals.var_c_fox_inv_dn7 * locals.var_t0__blk87) + (locals.var_c_fox_inv * locals.var_t0__blk87_dn7));
        locals.var_t1__blk88_dn10 = ((locals.var_c_fox_inv_dn10 * locals.var_t0__blk87) + (locals.var_c_fox_inv * locals.var_t0__blk87_dn10));
        locals.var_t1__blk88_dn11 = ((locals.var_c_fox_inv_dn11 * locals.var_t0__blk87) + (locals.var_c_fox_inv * locals.var_t0__blk87_dn11));
        locals.var_t1__blk88_dn12 = ((locals.var_c_fox_inv_dn12 * locals.var_t0__blk87) + (locals.var_c_fox_inv * locals.var_t0__blk87_dn12));
        locals.var_t1__blk88_dn17 = ((locals.var_c_fox_inv_dn17 * locals.var_t0__blk87) + (locals.var_c_fox_inv * locals.var_t0__blk87_dn17));
        locals.var_t1__blk88_rv = 0.0;

        let assign6510_e4334: f64 = (p.p69 - locals.var_pb20b);
        locals.var_t2__blk89 = assign6510_e4334;
        locals.var_t2__blk89_dn0 = (-locals.var_pb20b_dn0);
        locals.var_t2__blk89_dn2 = (-locals.var_pb20b_dn2);
        locals.var_t2__blk89_dn6 = (-locals.var_pb20b_dn6);
        locals.var_t2__blk89_dn7 = (-locals.var_pb20b_dn7);
        locals.var_t2__blk89_dn10 = (-locals.var_pb20b_dn10);
        locals.var_t2__blk89_dn11 = (-locals.var_pb20b_dn11);
        locals.var_t2__blk89_dn12 = (-locals.var_pb20b_dn12);
        locals.var_t2__blk89_dn17 = (-locals.var_pb20b_dn17);
        locals.var_t2__blk89_rv = 0.0;

        let assign6520_e4337: f64 = (locals.var_lgleff - p.p71);
        locals.var_t3__blk90 = assign6520_e4337;
        locals.var_t3__blk90_rv = 0.0;

        let assign6530_e4341: f64 = (locals.var_t3__blk90 * locals.var_t3__blk90);
        let assign6530_e4342: f64 = (1.0 / assign6530_e4341);
        locals.var_t4__blk91 = assign6530_e4342;
        locals.var_t4__blk91_dn0 = 0.0;
        locals.var_t4__blk91_dn2 = 0.0;
        locals.var_t4__blk91_dn6 = 0.0;
        locals.var_t4__blk91_dn7 = 0.0;
        locals.var_t4__blk91_dn10 = 0.0;
        locals.var_t4__blk91_dn11 = 0.0;
        locals.var_t4__blk91_dn12 = 0.0;
        locals.var_t4__blk91_dn17 = 0.0;
        locals.var_t4__blk91_rv = 0.0;

        let assign6540_e4345: f64 = (locals.var_t1__blk88 * locals.var_t2__blk89);
        let assign6540_e4347: f64 = (assign6540_e4345 * locals.var_t4__blk91);
        locals.var_dvth0__blk93 = assign6540_e4347;
        locals.var_dvth0__blk93_dn0 = ((((locals.var_t1__blk88_dn0 * locals.var_t2__blk89) + (locals.var_t1__blk88 * locals.var_t2__blk89_dn0)) * locals.var_t4__blk91) + (assign6540_e4345 * locals.var_t4__blk91_dn0));
        locals.var_dvth0__blk93_dn2 = ((((locals.var_t1__blk88_dn2 * locals.var_t2__blk89) + (locals.var_t1__blk88 * locals.var_t2__blk89_dn2)) * locals.var_t4__blk91) + (assign6540_e4345 * locals.var_t4__blk91_dn2));
        locals.var_dvth0__blk93_dn6 = ((((locals.var_t1__blk88_dn6 * locals.var_t2__blk89) + (locals.var_t1__blk88 * locals.var_t2__blk89_dn6)) * locals.var_t4__blk91) + (assign6540_e4345 * locals.var_t4__blk91_dn6));
        locals.var_dvth0__blk93_dn7 = ((((locals.var_t1__blk88_dn7 * locals.var_t2__blk89) + (locals.var_t1__blk88 * locals.var_t2__blk89_dn7)) * locals.var_t4__blk91) + (assign6540_e4345 * locals.var_t4__blk91_dn7));
        locals.var_dvth0__blk93_dn10 = ((((locals.var_t1__blk88_dn10 * locals.var_t2__blk89) + (locals.var_t1__blk88 * locals.var_t2__blk89_dn10)) * locals.var_t4__blk91) + (assign6540_e4345 * locals.var_t4__blk91_dn10));
        locals.var_dvth0__blk93_dn11 = ((((locals.var_t1__blk88_dn11 * locals.var_t2__blk89) + (locals.var_t1__blk88 * locals.var_t2__blk89_dn11)) * locals.var_t4__blk91) + (assign6540_e4345 * locals.var_t4__blk91_dn11));
        locals.var_dvth0__blk93_dn12 = ((((locals.var_t1__blk88_dn12 * locals.var_t2__blk89) + (locals.var_t1__blk88 * locals.var_t2__blk89_dn12)) * locals.var_t4__blk91) + (assign6540_e4345 * locals.var_t4__blk91_dn12));
        locals.var_dvth0__blk93_dn17 = ((((locals.var_t1__blk88_dn17 * locals.var_t2__blk89) + (locals.var_t1__blk88 * locals.var_t2__blk89_dn17)) * locals.var_t4__blk91) + (assign6540_e4345 * locals.var_t4__blk91_dn17));
        locals.var_dvth0__blk93_rv = 0.0;

        let assign6550_e4350: f64 = (locals.var_uc_sc3 / locals.var_lgleff);
        locals.var_t1__blk88 = assign6550_e4350;
        locals.var_t1__blk88_dn0 = 0.0;
        locals.var_t1__blk88_dn2 = 0.0;
        locals.var_t1__blk88_dn6 = 0.0;
        locals.var_t1__blk88_dn7 = 0.0;
        locals.var_t1__blk88_dn10 = 0.0;
        locals.var_t1__blk88_dn11 = 0.0;
        locals.var_t1__blk88_dn12 = 0.0;
        locals.var_t1__blk88_dn17 = 0.0;
        locals.var_t1__blk88_rv = 0.0;

        let assign6560_e4354: f64 = (locals.var_t1__blk88 * locals.var_pbsum);
        let assign6560_e4355: f64 = (p.p83 + assign6560_e4354);
        locals.var_t4__blk91 = assign6560_e4355;
        locals.var_t4__blk91_dn0 = ((locals.var_t1__blk88_dn0 * locals.var_pbsum) + (locals.var_t1__blk88 * locals.var_pbsum_dn0));
        locals.var_t4__blk91_dn2 = ((locals.var_t1__blk88_dn2 * locals.var_pbsum) + (locals.var_t1__blk88 * locals.var_pbsum_dn2));
        locals.var_t4__blk91_dn6 = ((locals.var_t1__blk88_dn6 * locals.var_pbsum) + (locals.var_t1__blk88 * locals.var_pbsum_dn6));
        locals.var_t4__blk91_dn7 = ((locals.var_t1__blk88_dn7 * locals.var_pbsum) + (locals.var_t1__blk88 * locals.var_pbsum_dn7));
        locals.var_t4__blk91_dn10 = ((locals.var_t1__blk88_dn10 * locals.var_pbsum) + (locals.var_t1__blk88 * locals.var_pbsum_dn10));
        locals.var_t4__blk91_dn11 = ((locals.var_t1__blk88_dn11 * locals.var_pbsum) + (locals.var_t1__blk88 * locals.var_pbsum_dn11));
        locals.var_t4__blk91_dn12 = ((locals.var_t1__blk88_dn12 * locals.var_pbsum) + (locals.var_t1__blk88 * locals.var_pbsum_dn12));
        locals.var_t4__blk91_dn17 = ((locals.var_t1__blk88_dn17 * locals.var_pbsum) + (locals.var_t1__blk88 * locals.var_pbsum_dn17));
        locals.var_t4__blk91_rv = 0.0;

        let assign6570_e4359: f64 = (locals.var_uc_sc2 * locals.var_vdsz);
        let assign6570_e4360: f64 = (locals.var_t4__blk91 + assign6570_e4359);
        locals.var_t5__blk92 = assign6570_e4360;
        locals.var_t5__blk92_dn0 = (locals.var_t4__blk91_dn0 + (locals.var_uc_sc2 * locals.var_vdsz_dn0));
        locals.var_t5__blk92_dn2 = (locals.var_t4__blk91_dn2 + (locals.var_uc_sc2 * locals.var_vdsz_dn2));
        locals.var_t5__blk92_dn6 = (locals.var_t4__blk91_dn6 + (locals.var_uc_sc2 * locals.var_vdsz_dn6));
        locals.var_t5__blk92_dn7 = (locals.var_t4__blk91_dn7 + (locals.var_uc_sc2 * locals.var_vdsz_dn7));
        locals.var_t5__blk92_dn10 = (locals.var_t4__blk91_dn10 + (locals.var_uc_sc2 * locals.var_vdsz_dn10));
        locals.var_t5__blk92_dn11 = (locals.var_t4__blk91_dn11 + (locals.var_uc_sc2 * locals.var_vdsz_dn11));
        locals.var_t5__blk92_dn12 = (locals.var_t4__blk91_dn12 + (locals.var_uc_sc2 * locals.var_vdsz_dn12));
        locals.var_t5__blk92_dn17 = (locals.var_t4__blk91_dn17 + (locals.var_uc_sc2 * locals.var_vdsz_dn17));
        locals.var_t5__blk92_rv = 0.0;

        let assign6580_e4363: f64 = (locals.var_dvth0__blk93 * locals.var_t5__blk92);
        locals.var_dvthsc = assign6580_e4363;
        locals.var_dvthsc_dn0 = ((locals.var_dvth0__blk93_dn0 * locals.var_t5__blk92) + (locals.var_dvth0__blk93 * locals.var_t5__blk92_dn0));
        locals.var_dvthsc_dn2 = ((locals.var_dvth0__blk93_dn2 * locals.var_t5__blk92) + (locals.var_dvth0__blk93 * locals.var_t5__blk92_dn2));
        locals.var_dvthsc_dn6 = ((locals.var_dvth0__blk93_dn6 * locals.var_t5__blk92) + (locals.var_dvth0__blk93 * locals.var_t5__blk92_dn6));
        locals.var_dvthsc_dn7 = ((locals.var_dvth0__blk93_dn7 * locals.var_t5__blk92) + (locals.var_dvth0__blk93 * locals.var_t5__blk92_dn7));
        locals.var_dvthsc_dn10 = ((locals.var_dvth0__blk93_dn10 * locals.var_t5__blk92) + (locals.var_dvth0__blk93 * locals.var_t5__blk92_dn10));
        locals.var_dvthsc_dn11 = ((locals.var_dvth0__blk93_dn11 * locals.var_t5__blk92) + (locals.var_dvth0__blk93 * locals.var_t5__blk92_dn11));
        locals.var_dvthsc_dn12 = ((locals.var_dvth0__blk93_dn12 * locals.var_t5__blk92) + (locals.var_dvth0__blk93 * locals.var_t5__blk92_dn12));
        locals.var_dvthsc_dn17 = ((locals.var_dvth0__blk93_dn17 * locals.var_t5__blk92) + (locals.var_dvth0__blk93 * locals.var_t5__blk92_dn17));
        locals.var_dvthsc_rv = 0.0;

        let assign6590_e4366: f64 = if p.p86 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign6590_e4366;
        locals.var_guard97_rv = 0.0;

        let (assign6600_e4380, assign6600_e4380_d_n0, assign6600_e4380_d_n2, assign6600_e4380_d_n6, assign6600_e4380_d_n7, assign6600_e4380_d_n10, assign6600_e4380_d_n11, assign6600_e4380_d_n12, assign6600_e4380_d_n17,) = {
    if (locals.var_guard97 != 0.0) {
        let assign6600_e4370: f64 = (locals.var_eg + locals.var_pb2);
        let assign6600_e4373: f64 = (2.0 * p.p88);
        let assign6600_e4374: f64 = (assign6600_e4370 - assign6600_e4373);
        let assign6600_e4377: f64 = (p.p87 * locals.var_vdsz);
        let assign6600_e4378: f64 = (assign6600_e4374 + assign6600_e4377);
        (assign6600_e4378, ((locals.var_eg_dn0 + locals.var_pb2_dn0) + (p.p87 * locals.var_vdsz_dn0)), ((locals.var_eg_dn2 + locals.var_pb2_dn2) + (p.p87 * locals.var_vdsz_dn2)), ((locals.var_eg_dn6 + locals.var_pb2_dn6) + (p.p87 * locals.var_vdsz_dn6)), ((locals.var_eg_dn7 + locals.var_pb2_dn7) + (p.p87 * locals.var_vdsz_dn7)), ((locals.var_eg_dn10 + locals.var_pb2_dn10) + (p.p87 * locals.var_vdsz_dn10)), ((locals.var_eg_dn11 + locals.var_pb2_dn11) + (p.p87 * locals.var_vdsz_dn11)), ((locals.var_eg_dn12 + locals.var_pb2_dn12) + (p.p87 * locals.var_vdsz_dn12)), ((locals.var_eg_dn17 + locals.var_pb2_dn17) + (p.p87 * locals.var_vdsz_dn17)),)
    } else {
        (locals.var_t1__blk94, locals.var_t1__blk94_dn0, locals.var_t1__blk94_dn2, locals.var_t1__blk94_dn6, locals.var_t1__blk94_dn7, locals.var_t1__blk94_dn10, locals.var_t1__blk94_dn11, locals.var_t1__blk94_dn12, locals.var_t1__blk94_dn17,)
    }
};
        locals.var_t1__blk94 = assign6600_e4380;
        locals.var_t1__blk94_dn0 = assign6600_e4380_d_n0;
        locals.var_t1__blk94_dn2 = assign6600_e4380_d_n2;
        locals.var_t1__blk94_dn6 = assign6600_e4380_d_n6;
        locals.var_t1__blk94_dn7 = assign6600_e4380_d_n7;
        locals.var_t1__blk94_dn10 = assign6600_e4380_d_n10;
        locals.var_t1__blk94_dn11 = assign6600_e4380_d_n11;
        locals.var_t1__blk94_dn12 = assign6600_e4380_d_n12;
        locals.var_t1__blk94_dn17 = assign6600_e4380_d_n17;
        locals.var_t1__blk94_rv = 0.0;

        let (assign6610_e4388,) = {
    if (locals.var_guard97 != 0.0) {
        let assign6610_e4384: f64 = (locals.var_lgleff * 0.5);
        let assign6610_e4386: f64 = (assign6610_e4384 + locals.var_mks_parl1);
        (assign6610_e4386,)
    } else {
        (locals.var_t2__blk95,)
    }
};
        locals.var_t2__blk95 = assign6610_e4388;
        locals.var_t2__blk95_rv = 0.0;

        let (assign6620_e4396,) = {
    if (locals.var_guard97 != 0.0) {
        let assign6620_e4392: f64 = (p.p86 * p.p237);
        let assign6620_e4394: f64 = (assign6620_e4392 / locals.var_t2__blk95);
        (assign6620_e4394,)
    } else {
        (locals.var_t3__blk96,)
    }
};
        locals.var_t3__blk96 = assign6620_e4396;
        locals.var_t3__blk96_rv = 0.0;

        let (assign6630_e4402, assign6630_e4402_d_n0, assign6630_e4402_d_n2, assign6630_e4402_d_n6, assign6630_e4402_d_n7, assign6630_e4402_d_n10, assign6630_e4402_d_n11, assign6630_e4402_d_n12, assign6630_e4402_d_n17,) = {
    if (locals.var_guard97 != 0.0) {
        let assign6630_e4400: f64 = (locals.var_t1__blk94 * locals.var_t3__blk96);
        (assign6630_e4400, (locals.var_t1__blk94_dn0 * locals.var_t3__blk96), (locals.var_t1__blk94_dn2 * locals.var_t3__blk96), (locals.var_t1__blk94_dn6 * locals.var_t3__blk96), (locals.var_t1__blk94_dn7 * locals.var_t3__blk96), (locals.var_t1__blk94_dn10 * locals.var_t3__blk96), (locals.var_t1__blk94_dn11 * locals.var_t3__blk96), (locals.var_t1__blk94_dn12 * locals.var_t3__blk96), (locals.var_t1__blk94_dn17 * locals.var_t3__blk96),)
    } else {
        (locals.var_dvthscr, locals.var_dvthscr_dn0, locals.var_dvthscr_dn2, locals.var_dvthscr_dn6, locals.var_dvthscr_dn7, locals.var_dvthscr_dn10, locals.var_dvthscr_dn11, locals.var_dvthscr_dn12, locals.var_dvthscr_dn17,)
    }
};
        locals.var_dvthscr = assign6630_e4402;
        locals.var_dvthscr_dn0 = assign6630_e4402_d_n0;
        locals.var_dvthscr_dn2 = assign6630_e4402_d_n2;
        locals.var_dvthscr_dn6 = assign6630_e4402_d_n6;
        locals.var_dvthscr_dn7 = assign6630_e4402_d_n7;
        locals.var_dvthscr_dn10 = assign6630_e4402_d_n10;
        locals.var_dvthscr_dn11 = assign6630_e4402_d_n11;
        locals.var_dvthscr_dn12 = assign6630_e4402_d_n12;
        locals.var_dvthscr_dn17 = assign6630_e4402_d_n17;
        locals.var_dvthscr_rv = 0.0;

        let (assign6640_e4407, assign6640_e4407_d_n0, assign6640_e4407_d_n2, assign6640_e4407_d_n6, assign6640_e4407_d_n7, assign6640_e4407_d_n10, assign6640_e4407_d_n11, assign6640_e4407_d_n12, assign6640_e4407_d_n17,) = {
    if (locals.var_guard97 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvthscr, locals.var_dvthscr_dn0, locals.var_dvthscr_dn2, locals.var_dvthscr_dn6, locals.var_dvthscr_dn7, locals.var_dvthscr_dn10, locals.var_dvthscr_dn11, locals.var_dvthscr_dn12, locals.var_dvthscr_dn17,)
    }
};
        locals.var_dvthscr = assign6640_e4407;
        locals.var_dvthscr_dn0 = assign6640_e4407_d_n0;
        locals.var_dvthscr_dn2 = assign6640_e4407_d_n2;
        locals.var_dvthscr_dn6 = assign6640_e4407_d_n6;
        locals.var_dvthscr_dn7 = assign6640_e4407_d_n7;
        locals.var_dvthscr_dn10 = assign6640_e4407_d_n10;
        locals.var_dvthscr_dn11 = assign6640_e4407_d_n11;
        locals.var_dvthscr_dn12 = assign6640_e4407_d_n12;
        locals.var_dvthscr_dn17 = assign6640_e4407_d_n17;
        locals.var_dvthscr_rv = 0.0;

        locals.var_t1__blk98 = locals.var_c_fox_inv;
        locals.var_t1__blk98_dn0 = locals.var_c_fox_inv_dn0;
        locals.var_t1__blk98_dn2 = locals.var_c_fox_inv_dn2;
        locals.var_t1__blk98_dn6 = locals.var_c_fox_inv_dn6;
        locals.var_t1__blk98_dn7 = locals.var_c_fox_inv_dn7;
        locals.var_t1__blk98_dn10 = locals.var_c_fox_inv_dn10;
        locals.var_t1__blk98_dn11 = locals.var_c_fox_inv_dn11;
        locals.var_t1__blk98_dn12 = locals.var_c_fox_inv_dn12;
        locals.var_t1__blk98_dn17 = locals.var_c_fox_inv_dn17;
        locals.var_t1__blk98_rv = 0.0;

        let assign6660_e4413: f64 = (locals.var_mks_wfc / locals.var_weff);
        let assign6660_e4414: f64 = (locals.var_c_fox + assign6660_e4413);
        let assign6660_e4415: f64 = (1.0 / assign6660_e4414);
        locals.var_t3__blk99 = assign6660_e4415;
        locals.var_t3__blk99_dn0 = (-(locals.var_c_fox_dn0 / (assign6660_e4414 * assign6660_e4414)));
        locals.var_t3__blk99_dn2 = (-(locals.var_c_fox_dn2 / (assign6660_e4414 * assign6660_e4414)));
        locals.var_t3__blk99_dn6 = (-(locals.var_c_fox_dn6 / (assign6660_e4414 * assign6660_e4414)));
        locals.var_t3__blk99_dn7 = (-(locals.var_c_fox_dn7 / (assign6660_e4414 * assign6660_e4414)));
        locals.var_t3__blk99_dn10 = (-(locals.var_c_fox_dn10 / (assign6660_e4414 * assign6660_e4414)));
        locals.var_t3__blk99_dn11 = (-(locals.var_c_fox_dn11 / (assign6660_e4414 * assign6660_e4414)));
        locals.var_t3__blk99_dn12 = (-(locals.var_c_fox_dn12 / (assign6660_e4414 * assign6660_e4414)));
        locals.var_t3__blk99_dn17 = (-(locals.var_c_fox_dn17 / (assign6660_e4414 * assign6660_e4414)));
        locals.var_t3__blk99_rv = 0.0;

        let assign6670_e4418: f64 = (locals.var_t1__blk98 - locals.var_t3__blk99);
        locals.var_t5__blk100 = assign6670_e4418;
        locals.var_t5__blk100_dn0 = (locals.var_t1__blk98_dn0 - locals.var_t3__blk99_dn0);
        locals.var_t5__blk100_dn2 = (locals.var_t1__blk98_dn2 - locals.var_t3__blk99_dn2);
        locals.var_t5__blk100_dn6 = (locals.var_t1__blk98_dn6 - locals.var_t3__blk99_dn6);
        locals.var_t5__blk100_dn7 = (locals.var_t1__blk98_dn7 - locals.var_t3__blk99_dn7);
        locals.var_t5__blk100_dn10 = (locals.var_t1__blk98_dn10 - locals.var_t3__blk99_dn10);
        locals.var_t5__blk100_dn11 = (locals.var_t1__blk98_dn11 - locals.var_t3__blk99_dn11);
        locals.var_t5__blk100_dn12 = (locals.var_t1__blk98_dn12 - locals.var_t3__blk99_dn12);
        locals.var_t5__blk100_dn17 = (locals.var_t1__blk98_dn17 - locals.var_t3__blk99_dn17);
        locals.var_t5__blk100_rv = 0.0;

        let assign6680_e4421: f64 = (locals.var_qb0 * locals.var_t5__blk100);
        let assign6680_e4424: f64 = (p.p105 / locals.var_wg);
        let assign6680_e4425: f64 = (assign6680_e4421 + assign6680_e4424);
        locals.var_dvthw = assign6680_e4425;
        locals.var_dvthw_dn0 = ((locals.var_qb0_dn0 * locals.var_t5__blk100) + (locals.var_qb0 * locals.var_t5__blk100_dn0));
        locals.var_dvthw_dn2 = ((locals.var_qb0_dn2 * locals.var_t5__blk100) + (locals.var_qb0 * locals.var_t5__blk100_dn2));
        locals.var_dvthw_dn6 = ((locals.var_qb0_dn6 * locals.var_t5__blk100) + (locals.var_qb0 * locals.var_t5__blk100_dn6));
        locals.var_dvthw_dn7 = ((locals.var_qb0_dn7 * locals.var_t5__blk100) + (locals.var_qb0 * locals.var_t5__blk100_dn7));
        locals.var_dvthw_dn10 = ((locals.var_qb0_dn10 * locals.var_t5__blk100) + (locals.var_qb0 * locals.var_t5__blk100_dn10));
        locals.var_dvthw_dn11 = ((locals.var_qb0_dn11 * locals.var_t5__blk100) + (locals.var_qb0 * locals.var_t5__blk100_dn11));
        locals.var_dvthw_dn12 = ((locals.var_qb0_dn12 * locals.var_t5__blk100) + (locals.var_qb0 * locals.var_t5__blk100_dn12));
        locals.var_dvthw_dn17 = ((locals.var_qb0_dn17 * locals.var_t5__blk100) + (locals.var_qb0 * locals.var_t5__blk100_dn17));
        locals.var_dvthw_rv = 0.0;

        let assign6690_e4428: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign6690_e4430: f64 = (assign6690_e4428 + locals.var_dvthw);
        let assign6690_e4432: f64 = (assign6690_e4430 + locals.var_dvthscr);
        let assign6690_e4434: f64 = (assign6690_e4432 + locals.var_dvthsm);
        locals.var_dvth = assign6690_e4434;
        locals.var_dvth_dn0 = (((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) + locals.var_dvthw_dn0) + locals.var_dvthscr_dn0);
        locals.var_dvth_dn2 = (((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) + locals.var_dvthw_dn2) + locals.var_dvthscr_dn2);
        locals.var_dvth_dn6 = (((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) + locals.var_dvthw_dn6) + locals.var_dvthscr_dn6);
        locals.var_dvth_dn7 = (((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) + locals.var_dvthw_dn7) + locals.var_dvthscr_dn7);
        locals.var_dvth_dn10 = (((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) + locals.var_dvthw_dn10) + locals.var_dvthscr_dn10);
        locals.var_dvth_dn11 = (((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) + locals.var_dvthw_dn11) + locals.var_dvthscr_dn11);
        locals.var_dvth_dn12 = (((locals.var_dvthsc_dn12 + locals.var_dvthlp_dn12) + locals.var_dvthw_dn12) + locals.var_dvthscr_dn12);
        locals.var_dvth_dn17 = (((locals.var_dvthsc_dn17 + locals.var_dvthlp_dn17) + locals.var_dvthw_dn17) + locals.var_dvthscr_dn17);
        locals.var_dvth_rv = 0.0;

        let assign6700_e4437: f64 = (locals.var_vthp - locals.var_dvth);
        locals.var_vth = assign6700_e4437;
        locals.var_vth_dn0 = (locals.var_vthp_dn0 - locals.var_dvth_dn0);
        locals.var_vth_dn2 = (locals.var_vthp_dn2 - locals.var_dvth_dn2);
        locals.var_vth_dn6 = (locals.var_vthp_dn6 - locals.var_dvth_dn6);
        locals.var_vth_dn7 = (locals.var_vthp_dn7 - locals.var_dvth_dn7);
        locals.var_vth_dn10 = (locals.var_vthp_dn10 - locals.var_dvth_dn10);
        locals.var_vth_dn11 = (locals.var_vthp_dn11 - locals.var_dvth_dn11);
        locals.var_vth_dn12 = (locals.var_vthp_dn12 - locals.var_dvth_dn12);
        locals.var_vth_dn17 = (locals.var_vthp_dn17 - locals.var_dvth_dn17);
        locals.var_vth_rv = 0.0;

        let assign6710_e4440: f64 = if p.p89 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard104 = assign6710_e4440;
        locals.var_guard104_rv = 0.0;

        let (assign6720_e4444,) = {
    if (locals.var_guard104 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_dppg,)
    }
};
        locals.var_flg_dppg = assign6720_e4444;
        locals.var_flg_dppg_rv = 0.0;

        let (assign6730_e4449,) = {
    if (locals.var_guard104 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_dppg,)
    }
};
        locals.var_flg_dppg = assign6730_e4449;
        locals.var_flg_dppg_rv = 0.0;

        let assign6740_e4452: f64 = if locals.var_flg_dppg == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign6740_e4452;
        locals.var_guard105_rv = 0.0;

        let (assign6750_e4456, assign6750_e4456_d_n0, assign6750_e4456_d_n2, assign6750_e4456_d_n6, assign6750_e4456_d_n7, assign6750_e4456_d_n10, assign6750_e4456_d_n11, assign6750_e4456_d_n12, assign6750_e4456_d_n17,) = {
    if (locals.var_guard105 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6750_e4456;
        locals.var_dppg_dn0 = assign6750_e4456_d_n0;
        locals.var_dppg_dn2 = assign6750_e4456_d_n2;
        locals.var_dppg_dn6 = assign6750_e4456_d_n6;
        locals.var_dppg_dn7 = assign6750_e4456_d_n7;
        locals.var_dppg_dn10 = assign6750_e4456_d_n10;
        locals.var_dppg_dn11 = assign6750_e4456_d_n11;
        locals.var_dppg_dn12 = assign6750_e4456_d_n12;
        locals.var_dppg_dn17 = assign6750_e4456_d_n17;
        locals.var_dppg_rv = 0.0;

        let (assign6760_e4461, assign6760_e4461_d_n0, assign6760_e4461_d_n2, assign6760_e4461_d_n6, assign6760_e4461_d_n7, assign6760_e4461_d_n10, assign6760_e4461_d_n11, assign6760_e4461_d_n12, assign6760_e4461_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        (locals.var_vgsz, locals.var_vgsz_dn0, locals.var_vgsz_dn2, locals.var_vgsz_dn6, locals.var_vgsz_dn7, locals.var_vgsz_dn10, locals.var_vgsz_dn11, locals.var_vgsz_dn12, locals.var_vgsz_dn17,)
    } else {
        (locals.var_t7__blk101, locals.var_t7__blk101_dn0, locals.var_t7__blk101_dn2, locals.var_t7__blk101_dn6, locals.var_t7__blk101_dn7, locals.var_t7__blk101_dn10, locals.var_t7__blk101_dn11, locals.var_t7__blk101_dn12, locals.var_t7__blk101_dn17,)
    }
};
        locals.var_t7__blk101 = assign6760_e4461;
        locals.var_t7__blk101_dn0 = assign6760_e4461_d_n0;
        locals.var_t7__blk101_dn2 = assign6760_e4461_d_n2;
        locals.var_t7__blk101_dn6 = assign6760_e4461_d_n6;
        locals.var_t7__blk101_dn7 = assign6760_e4461_d_n7;
        locals.var_t7__blk101_dn10 = assign6760_e4461_d_n10;
        locals.var_t7__blk101_dn11 = assign6760_e4461_d_n11;
        locals.var_t7__blk101_dn12 = assign6760_e4461_d_n12;
        locals.var_t7__blk101_dn17 = assign6760_e4461_d_n17;
        locals.var_t7__blk101_rv = 0.0;

        let (assign6770_e4466,) = {
    if (locals.var_guard105 == 0.0) {
        (locals.var_cnstpgd,)
    } else {
        (locals.var_t0__blk102,)
    }
};
        locals.var_t0__blk102 = assign6770_e4466;
        locals.var_t0__blk102_rv = 0.0;

        let (assign6780_e4473, assign6780_e4473_d_n0, assign6780_e4473_d_n2, assign6780_e4473_d_n6, assign6780_e4473_d_n7, assign6780_e4473_d_n10, assign6780_e4473_d_n11, assign6780_e4473_d_n12, assign6780_e4473_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let assign6780_e4471: f64 = (locals.var_t7__blk101 - p.p90);
        (assign6780_e4471, locals.var_t7__blk101_dn0, locals.var_t7__blk101_dn2, locals.var_t7__blk101_dn6, locals.var_t7__blk101_dn7, locals.var_t7__blk101_dn10, locals.var_t7__blk101_dn11, locals.var_t7__blk101_dn12, locals.var_t7__blk101_dn17,)
    } else {
        (locals.var_t3__blk103, locals.var_t3__blk103_dn0, locals.var_t3__blk103_dn2, locals.var_t3__blk103_dn6, locals.var_t3__blk103_dn7, locals.var_t3__blk103_dn10, locals.var_t3__blk103_dn11, locals.var_t3__blk103_dn12, locals.var_t3__blk103_dn17,)
    }
};
        locals.var_t3__blk103 = assign6780_e4473;
        locals.var_t3__blk103_dn0 = assign6780_e4473_d_n0;
        locals.var_t3__blk103_dn2 = assign6780_e4473_d_n2;
        locals.var_t3__blk103_dn6 = assign6780_e4473_d_n6;
        locals.var_t3__blk103_dn7 = assign6780_e4473_d_n7;
        locals.var_t3__blk103_dn10 = assign6780_e4473_d_n10;
        locals.var_t3__blk103_dn11 = assign6780_e4473_d_n11;
        locals.var_t3__blk103_dn12 = assign6780_e4473_d_n12;
        locals.var_t3__blk103_dn17 = assign6780_e4473_d_n17;
        locals.var_t3__blk103_rv = 0.0;

        let assign6790_e4476: f64 = (-3.0);
        let assign6790_e4477: f64 = if locals.var_t3__blk103 < assign6790_e4476 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign6790_e4477;
        locals.var_guard106_rv = 0.0;

        let (assign6800_e4484, assign6800_e4484_d_n0, assign6800_e4484_d_n2, assign6800_e4484_d_n6, assign6800_e4484_d_n7, assign6800_e4484_d_n10, assign6800_e4484_d_n11, assign6800_e4484_d_n12, assign6800_e4484_d_n17,) = {
    if ((locals.var_guard105 == 0.0) && (locals.var_guard106 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6800_e4484;
        locals.var_dppg_dn0 = assign6800_e4484_d_n0;
        locals.var_dppg_dn2 = assign6800_e4484_d_n2;
        locals.var_dppg_dn6 = assign6800_e4484_d_n6;
        locals.var_dppg_dn7 = assign6800_e4484_d_n7;
        locals.var_dppg_dn10 = assign6800_e4484_d_n10;
        locals.var_dppg_dn11 = assign6800_e4484_d_n11;
        locals.var_dppg_dn12 = assign6800_e4484_d_n12;
        locals.var_dppg_dn17 = assign6800_e4484_d_n17;
        locals.var_dppg_rv = 0.0;

        let assign6810_e4487: f64 = if locals.var_t3__blk103 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign6810_e4487;
        locals.var_guard107_rv = 0.0;

        let (assign6820_e4513, assign6820_e4513_d_n0, assign6820_e4513_d_n2, assign6820_e4513_d_n6, assign6820_e4513_d_n7, assign6820_e4513_d_n10, assign6820_e4513_d_n11, assign6820_e4513_d_n12, assign6820_e4513_d_n17,) = {
    if (((locals.var_guard105 == 0.0) && (locals.var_guard106 == 0.0)) && (locals.var_guard107 != 0.0)) {
        let assign6820_e4501: f64 = (1.0 / 3.0);
        let assign6820_e4505: f64 = (1.0 / 27.0);
        let assign6820_e4506: f64 = (locals.var_t3__blk103 * assign6820_e4505);
        let assign6820_e4507: f64 = (assign6820_e4501 + assign6820_e4506);
        let assign6820_e4508: f64 = (locals.var_t3__blk103 * assign6820_e4507);
        let assign6820_e4509: f64 = (1.0 + assign6820_e4508);
        let assign6820_e4510: f64 = (locals.var_t3__blk103 * assign6820_e4509);
        let assign6820_e4511: f64 = (1.0 + assign6820_e4510);
        (assign6820_e4511, ((locals.var_t3__blk103_dn0 * assign6820_e4509) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn0 * assign6820_e4507) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn0 * assign6820_e4505))))), ((locals.var_t3__blk103_dn2 * assign6820_e4509) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn2 * assign6820_e4507) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn2 * assign6820_e4505))))), ((locals.var_t3__blk103_dn6 * assign6820_e4509) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn6 * assign6820_e4507) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn6 * assign6820_e4505))))), ((locals.var_t3__blk103_dn7 * assign6820_e4509) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn7 * assign6820_e4507) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn7 * assign6820_e4505))))), ((locals.var_t3__blk103_dn10 * assign6820_e4509) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn10 * assign6820_e4507) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn10 * assign6820_e4505))))), ((locals.var_t3__blk103_dn11 * assign6820_e4509) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn11 * assign6820_e4507) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn11 * assign6820_e4505))))), ((locals.var_t3__blk103_dn12 * assign6820_e4509) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn12 * assign6820_e4507) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn12 * assign6820_e4505))))), ((locals.var_t3__blk103_dn17 * assign6820_e4509) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn17 * assign6820_e4507) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn17 * assign6820_e4505))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6820_e4513;
        locals.var_dppg_dn0 = assign6820_e4513_d_n0;
        locals.var_dppg_dn2 = assign6820_e4513_d_n2;
        locals.var_dppg_dn6 = assign6820_e4513_d_n6;
        locals.var_dppg_dn7 = assign6820_e4513_d_n7;
        locals.var_dppg_dn10 = assign6820_e4513_d_n10;
        locals.var_dppg_dn11 = assign6820_e4513_d_n11;
        locals.var_dppg_dn12 = assign6820_e4513_d_n12;
        locals.var_dppg_dn17 = assign6820_e4513_d_n17;
        locals.var_dppg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6830_e4542, assign6830_e4542_d_n0, assign6830_e4542_d_n2, assign6830_e4542_d_n6, assign6830_e4542_d_n7, assign6830_e4542_d_n10, assign6830_e4542_d_n11, assign6830_e4542_d_n12, assign6830_e4542_d_n17,) = {
    if (((locals.var_guard105 == 0.0) && (locals.var_guard106 == 0.0)) && (locals.var_guard107 == 0.0)) {
        let assign6830_e4528: f64 = (1.0 / 3.0);
        let assign6830_e4533: f64 = (locals.var_t3__blk103 * 0.148148111111111);
        let assign6830_e4534: f64 = (0.0402052934513951 + assign6830_e4533);
        let assign6830_e4535: f64 = (locals.var_t3__blk103 * assign6830_e4534);
        let assign6830_e4536: f64 = (assign6830_e4528 + assign6830_e4535);
        let assign6830_e4537: f64 = (locals.var_t3__blk103 * assign6830_e4536);
        let assign6830_e4538: f64 = (1.0 + assign6830_e4537);
        let assign6830_e4539: f64 = (locals.var_t3__blk103 * assign6830_e4538);
        let assign6830_e4540: f64 = (1.0 + assign6830_e4539);
        (assign6830_e4540, ((locals.var_t3__blk103_dn0 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn0 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn0 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn0 * 0.148148111111111))))))), ((locals.var_t3__blk103_dn2 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn2 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn2 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn2 * 0.148148111111111))))))), ((locals.var_t3__blk103_dn6 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn6 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn6 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn6 * 0.148148111111111))))))), ((locals.var_t3__blk103_dn7 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn7 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn7 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn7 * 0.148148111111111))))))), ((locals.var_t3__blk103_dn10 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn10 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn10 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn10 * 0.148148111111111))))))), ((locals.var_t3__blk103_dn11 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn11 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn11 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn11 * 0.148148111111111))))))), ((locals.var_t3__blk103_dn12 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn12 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn12 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn12 * 0.148148111111111))))))), ((locals.var_t3__blk103_dn17 * assign6830_e4538) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn17 * assign6830_e4536) + (locals.var_t3__blk103 * ((locals.var_t3__blk103_dn17 * assign6830_e4534) + (locals.var_t3__blk103 * (locals.var_t3__blk103_dn17 * 0.148148111111111))))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6830_e4542;
        locals.var_dppg_dn0 = assign6830_e4542_d_n0;
        locals.var_dppg_dn2 = assign6830_e4542_d_n2;
        locals.var_dppg_dn6 = assign6830_e4542_d_n6;
        locals.var_dppg_dn7 = assign6830_e4542_d_n7;
        locals.var_dppg_dn10 = assign6830_e4542_d_n10;
        locals.var_dppg_dn11 = assign6830_e4542_d_n11;
        locals.var_dppg_dn12 = assign6830_e4542_d_n12;
        locals.var_dppg_dn17 = assign6830_e4542_d_n17;
        locals.var_dppg_rv = 0.0;

        let (assign6840_e4560, assign6840_e4560_d_n0, assign6840_e4560_d_n2, assign6840_e4560_d_n6, assign6840_e4560_d_n7, assign6840_e4560_d_n10, assign6840_e4560_d_n11, assign6840_e4560_d_n12, assign6840_e4560_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let assign6840_e4547: f64 = (locals.var_dppg - 1.0);
        let assign6840_e4550: f64 = (locals.var_dppg - 1.0);
        let assign6840_e4551: f64 = (assign6840_e4547 * assign6840_e4550);
        let assign6840_e4554: f64 = (4.0 * 0.1);
        let assign6840_e4556: f64 = (assign6840_e4554 * 0.1);
        let assign6840_e4557: f64 = (assign6840_e4551 + assign6840_e4556);
        let assign6840_e4558: f64 = (assign6840_e4557).sqrt();
        (assign6840_e4558, (((locals.var_dppg_dn0 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn0)) / (2.0 * assign6840_e4558)), (((locals.var_dppg_dn2 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn2)) / (2.0 * assign6840_e4558)), (((locals.var_dppg_dn6 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn6)) / (2.0 * assign6840_e4558)), (((locals.var_dppg_dn7 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn7)) / (2.0 * assign6840_e4558)), (((locals.var_dppg_dn10 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn10)) / (2.0 * assign6840_e4558)), (((locals.var_dppg_dn11 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn11)) / (2.0 * assign6840_e4558)), (((locals.var_dppg_dn12 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn12)) / (2.0 * assign6840_e4558)), (((locals.var_dppg_dn17 * assign6840_e4550) + (assign6840_e4547 * locals.var_dppg_dn17)) / (2.0 * assign6840_e4558)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign6840_e4560;
        locals.var_tmf1_dn0 = assign6840_e4560_d_n0;
        locals.var_tmf1_dn2 = assign6840_e4560_d_n2;
        locals.var_tmf1_dn6 = assign6840_e4560_d_n6;
        locals.var_tmf1_dn7 = assign6840_e4560_d_n7;
        locals.var_tmf1_dn10 = assign6840_e4560_d_n10;
        locals.var_tmf1_dn11 = assign6840_e4560_d_n11;
        locals.var_tmf1_dn12 = assign6840_e4560_d_n12;
        locals.var_tmf1_dn17 = assign6840_e4560_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign6850_e4575, assign6850_e4575_d_n0, assign6850_e4575_d_n2, assign6850_e4575_d_n6, assign6850_e4575_d_n7, assign6850_e4575_d_n10, assign6850_e4575_d_n11, assign6850_e4575_d_n12, assign6850_e4575_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let assign6850_e4566: f64 = (locals.var_dppg - 1.0);
        let assign6850_e4568: f64 = (assign6850_e4566 + locals.var_tmf1);
        let assign6850_e4569: f64 = (0.5 * assign6850_e4568);
        let assign6850_e4572: f64 = (1e-10 * 0.1);
        let assign6850_e4573: f64 = (assign6850_e4569 + assign6850_e4572);
        (assign6850_e4573, (0.5 * (locals.var_dppg_dn0 + locals.var_tmf1_dn0)), (0.5 * (locals.var_dppg_dn2 + locals.var_tmf1_dn2)), (0.5 * (locals.var_dppg_dn6 + locals.var_tmf1_dn6)), (0.5 * (locals.var_dppg_dn7 + locals.var_tmf1_dn7)), (0.5 * (locals.var_dppg_dn10 + locals.var_tmf1_dn10)), (0.5 * (locals.var_dppg_dn11 + locals.var_tmf1_dn11)), (0.5 * (locals.var_dppg_dn12 + locals.var_tmf1_dn12)), (0.5 * (locals.var_dppg_dn17 + locals.var_tmf1_dn17)),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6850_e4575;
        locals.var_dppg_dn0 = assign6850_e4575_d_n0;
        locals.var_dppg_dn2 = assign6850_e4575_d_n2;
        locals.var_dppg_dn6 = assign6850_e4575_d_n6;
        locals.var_dppg_dn7 = assign6850_e4575_d_n7;
        locals.var_dppg_dn10 = assign6850_e4575_d_n10;
        locals.var_dppg_dn11 = assign6850_e4575_d_n11;
        locals.var_dppg_dn12 = assign6850_e4575_d_n12;
        locals.var_dppg_dn17 = assign6850_e4575_d_n17;
        locals.var_dppg_rv = 0.0;

        let assign6860_e4578: f64 = if locals.var_dppg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign6860_e4578;
        locals.var_guard108_rv = 0.0;

        let (assign6870_e4585, assign6870_e4585_d_n0, assign6870_e4585_d_n2, assign6870_e4585_d_n6, assign6870_e4585_d_n7, assign6870_e4585_d_n10, assign6870_e4585_d_n11, assign6870_e4585_d_n12, assign6870_e4585_d_n17,) = {
    if ((locals.var_guard105 == 0.0) && (locals.var_guard108 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
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
        locals.var_dppg_rv = 0.0;

        let (assign6880_e4592, assign6880_e4592_d_n0, assign6880_e4592_d_n2, assign6880_e4592_d_n6, assign6880_e4592_d_n7, assign6880_e4592_d_n10, assign6880_e4592_d_n11, assign6880_e4592_d_n12, assign6880_e4592_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let assign6880_e4590: f64 = (locals.var_dppg * locals.var_t0__blk102);
        (assign6880_e4590, (locals.var_dppg_dn0 * locals.var_t0__blk102), (locals.var_dppg_dn2 * locals.var_t0__blk102), (locals.var_dppg_dn6 * locals.var_t0__blk102), (locals.var_dppg_dn7 * locals.var_t0__blk102), (locals.var_dppg_dn10 * locals.var_t0__blk102), (locals.var_dppg_dn11 * locals.var_t0__blk102), (locals.var_dppg_dn12 * locals.var_t0__blk102), (locals.var_dppg_dn17 * locals.var_t0__blk102),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6880_e4592;
        locals.var_dppg_dn0 = assign6880_e4592_d_n0;
        locals.var_dppg_dn2 = assign6880_e4592_d_n2;
        locals.var_dppg_dn6 = assign6880_e4592_d_n6;
        locals.var_dppg_dn7 = assign6880_e4592_d_n7;
        locals.var_dppg_dn10 = assign6880_e4592_d_n10;
        locals.var_dppg_dn11 = assign6880_e4592_d_n11;
        locals.var_dppg_dn12 = assign6880_e4592_d_n12;
        locals.var_dppg_dn17 = assign6880_e4592_d_n17;
        locals.var_dppg_rv = 0.0;

        let (assign6890_e4601, assign6890_e4601_d_n0, assign6890_e4601_d_n2, assign6890_e4601_d_n6, assign6890_e4601_d_n7, assign6890_e4601_d_n10, assign6890_e4601_d_n11, assign6890_e4601_d_n12, assign6890_e4601_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let assign6890_e4597: f64 = (1.0 - locals.var_dppg);
        let assign6890_e4599: f64 = (assign6890_e4597 - 0.05);
        (assign6890_e4599, (-locals.var_dppg_dn0), (-locals.var_dppg_dn2), (-locals.var_dppg_dn6), (-locals.var_dppg_dn7), (-locals.var_dppg_dn10), (-locals.var_dppg_dn11), (-locals.var_dppg_dn12), (-locals.var_dppg_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign6890_e4601;
        locals.var_tmf1_dn0 = assign6890_e4601_d_n0;
        locals.var_tmf1_dn2 = assign6890_e4601_d_n2;
        locals.var_tmf1_dn6 = assign6890_e4601_d_n6;
        locals.var_tmf1_dn7 = assign6890_e4601_d_n7;
        locals.var_tmf1_dn10 = assign6890_e4601_d_n10;
        locals.var_tmf1_dn11 = assign6890_e4601_d_n11;
        locals.var_tmf1_dn12 = assign6890_e4601_d_n12;
        locals.var_tmf1_dn17 = assign6890_e4601_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign6900_e4610, assign6900_e4610_d_n0, assign6900_e4610_d_n2, assign6900_e4610_d_n6, assign6900_e4610_d_n7, assign6900_e4610_d_n10, assign6900_e4610_d_n11, assign6900_e4610_d_n12, assign6900_e4610_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let assign6900_e4606: f64 = 4.0;
        let assign6900_e4608: f64 = (assign6900_e4606 * 0.05);
        (assign6900_e4608, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6900_e4610;
        locals.var_tmf2_dn0 = assign6900_e4610_d_n0;
        locals.var_tmf2_dn2 = assign6900_e4610_d_n2;
        locals.var_tmf2_dn6 = assign6900_e4610_d_n6;
        locals.var_tmf2_dn7 = assign6900_e4610_d_n7;
        locals.var_tmf2_dn10 = assign6900_e4610_d_n10;
        locals.var_tmf2_dn11 = assign6900_e4610_d_n11;
        locals.var_tmf2_dn12 = assign6900_e4610_d_n12;
        locals.var_tmf2_dn17 = assign6900_e4610_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6910_e4621, assign6910_e4621_d_n0, assign6910_e4621_d_n2, assign6910_e4621_d_n6, assign6910_e4621_d_n7, assign6910_e4621_d_n10, assign6910_e4621_d_n11, assign6910_e4621_d_n12, assign6910_e4621_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let (assign6910_e4619, assign6910_e4619_d_n0, assign6910_e4619_d_n2, assign6910_e4619_d_n6, assign6910_e4619_d_n7, assign6910_e4619_d_n10, assign6910_e4619_d_n11, assign6910_e4619_d_n12, assign6910_e4619_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign6910_e4618: f64 = (-locals.var_tmf2);
                (assign6910_e4618, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign6910_e4619, assign6910_e4619_d_n0, assign6910_e4619_d_n2, assign6910_e4619_d_n6, assign6910_e4619_d_n7, assign6910_e4619_d_n10, assign6910_e4619_d_n11, assign6910_e4619_d_n12, assign6910_e4619_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6910_e4621;
        locals.var_tmf2_dn0 = assign6910_e4621_d_n0;
        locals.var_tmf2_dn2 = assign6910_e4621_d_n2;
        locals.var_tmf2_dn6 = assign6910_e4621_d_n6;
        locals.var_tmf2_dn7 = assign6910_e4621_d_n7;
        locals.var_tmf2_dn10 = assign6910_e4621_d_n10;
        locals.var_tmf2_dn11 = assign6910_e4621_d_n11;
        locals.var_tmf2_dn12 = assign6910_e4621_d_n12;
        locals.var_tmf2_dn17 = assign6910_e4621_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6920_e4631, assign6920_e4631_d_n0, assign6920_e4631_d_n2, assign6920_e4631_d_n6, assign6920_e4631_d_n7, assign6920_e4631_d_n10, assign6920_e4631_d_n11, assign6920_e4631_d_n12, assign6920_e4631_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let assign6920_e4626: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign6920_e4628: f64 = (assign6920_e4626 + locals.var_tmf2);
        let assign6920_e4629: f64 = (assign6920_e4628).sqrt();
        (assign6920_e4629, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign6920_e4629)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign6920_e4629)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign6920_e4629)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign6920_e4629)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign6920_e4629)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign6920_e4629)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign6920_e4629)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign6920_e4629)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign6920_e4631;
        locals.var_tmf2_dn0 = assign6920_e4631_d_n0;
        locals.var_tmf2_dn2 = assign6920_e4631_d_n2;
        locals.var_tmf2_dn6 = assign6920_e4631_d_n6;
        locals.var_tmf2_dn7 = assign6920_e4631_d_n7;
        locals.var_tmf2_dn10 = assign6920_e4631_d_n10;
        locals.var_tmf2_dn11 = assign6920_e4631_d_n11;
        locals.var_tmf2_dn12 = assign6920_e4631_d_n12;
        locals.var_tmf2_dn17 = assign6920_e4631_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign6930_e4642, assign6930_e4642_d_n0, assign6930_e4642_d_n2, assign6930_e4642_d_n6, assign6930_e4642_d_n7, assign6930_e4642_d_n10, assign6930_e4642_d_n11, assign6930_e4642_d_n12, assign6930_e4642_d_n17,) = {
    if (locals.var_guard105 == 0.0) {
        let assign6930_e4638: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign6930_e4639: f64 = (0.5 * assign6930_e4638);
        let assign6930_e4640: f64 = (1.0 - assign6930_e4639);
        (assign6930_e4640, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (-(0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12, locals.var_dppg_dn17,)
    }
};
        locals.var_dppg = assign6930_e4642;
        locals.var_dppg_dn0 = assign6930_e4642_d_n0;
        locals.var_dppg_dn2 = assign6930_e4642_d_n2;
        locals.var_dppg_dn6 = assign6930_e4642_d_n6;
        locals.var_dppg_dn7 = assign6930_e4642_d_n7;
        locals.var_dppg_dn10 = assign6930_e4642_d_n10;
        locals.var_dppg_dn11 = assign6930_e4642_d_n11;
        locals.var_dppg_dn12 = assign6930_e4642_d_n12;
        locals.var_dppg_dn17 = assign6930_e4642_d_n17;
        locals.var_dppg_rv = 0.0;

        let assign6940_e4645: f64 = (locals.var_vgs - locals.var_vfb);
        let assign6940_e4647: f64 = (assign6940_e4645 + locals.var_dvth);
        let assign6940_e4649: f64 = (assign6940_e4647 - locals.var_dppg);
        locals.var_vgp = assign6940_e4649;
        locals.var_vgp_dn0 = (locals.var_dvth_dn0 - locals.var_dppg_dn0);
        locals.var_vgp_dn2 = (locals.var_dvth_dn2 - locals.var_dppg_dn2);
        locals.var_vgp_dn6 = ((locals.var_vgs_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6);
        locals.var_vgp_dn7 = ((locals.var_vgs_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7);
        locals.var_vgp_dn10 = (locals.var_dvth_dn10 - locals.var_dppg_dn10);
        locals.var_vgp_dn11 = ((locals.var_vgs_dn11 + locals.var_dvth_dn11) - locals.var_dppg_dn11);
        locals.var_vgp_dn12 = (locals.var_dvth_dn12 - locals.var_dppg_dn12);
        locals.var_vgp_dn17 = (locals.var_dvth_dn17 - locals.var_dppg_dn17);
        locals.var_vgp_rv = 0.0;

        locals.var_vgpz = locals.var_vgp;
        locals.var_vgpz_dn0 = locals.var_vgp_dn0;
        locals.var_vgpz_dn2 = locals.var_vgp_dn2;
        locals.var_vgpz_dn6 = locals.var_vgp_dn6;
        locals.var_vgpz_dn7 = locals.var_vgp_dn7;
        locals.var_vgpz_dn10 = locals.var_vgp_dn10;
        locals.var_vgpz_dn11 = locals.var_vgp_dn11;
        locals.var_vgpz_dn12 = locals.var_vgp_dn12;
        locals.var_vgpz_dn17 = locals.var_vgp_dn17;
        locals.var_vgpz_rv = 0.0;

        let assign6960_e4653: f64 = (locals.var_uc_nsubs / locals.var_mks_nsubb);
        let assign6960_e4654: f64 = (assign6960_e4653).ln();
        locals.var_t1 = assign6960_e4654;
        locals.var_t1_dn0 = ((locals.var_uc_nsubs_dn0 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_dn2 = ((locals.var_uc_nsubs_dn2 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_dn6 = ((locals.var_uc_nsubs_dn6 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_dn7 = ((locals.var_uc_nsubs_dn7 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_dn10 = ((locals.var_uc_nsubs_dn10 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_dn11 = ((locals.var_uc_nsubs_dn11 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_dn12 = ((locals.var_uc_nsubs_dn12 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_dn17 = ((locals.var_uc_nsubs_dn17 / locals.var_mks_nsubb) / assign6960_e4653);
        locals.var_t1_rv = 0.0;

        let assign6970_e4657: f64 = (locals.var_beta_inv * locals.var_t1);
        locals.var_vbi_soi = assign6970_e4657;
        locals.var_vbi_soi_dn0 = (locals.var_beta_inv * locals.var_t1_dn0);
        locals.var_vbi_soi_dn2 = (locals.var_beta_inv * locals.var_t1_dn2);
        locals.var_vbi_soi_dn6 = (locals.var_beta_inv * locals.var_t1_dn6);
        locals.var_vbi_soi_dn7 = (locals.var_beta_inv * locals.var_t1_dn7);
        locals.var_vbi_soi_dn10 = ((locals.var_beta_inv_dn10 * locals.var_t1) + (locals.var_beta_inv * locals.var_t1_dn10));
        locals.var_vbi_soi_dn11 = (locals.var_beta_inv * locals.var_t1_dn11);
        locals.var_vbi_soi_dn12 = (locals.var_beta_inv * locals.var_t1_dn12);
        locals.var_vbi_soi_dn17 = (locals.var_beta_inv * locals.var_t1_dn17);
        locals.var_vbi_soi_rv = 0.0;

        let assign6980_e4660: f64 = (locals.var_vfb - locals.var_dvth);
        let assign6980_e4662: f64 = (assign6980_e4660 + locals.var_dppg);
        locals.var_vgs_fb = assign6980_e4662;
        locals.var_vgs_fb_dn0 = ((-locals.var_dvth_dn0) + locals.var_dppg_dn0);
        locals.var_vgs_fb_dn2 = ((-locals.var_dvth_dn2) + locals.var_dppg_dn2);
        locals.var_vgs_fb_dn6 = ((-locals.var_dvth_dn6) + locals.var_dppg_dn6);
        locals.var_vgs_fb_dn7 = ((-locals.var_dvth_dn7) + locals.var_dppg_dn7);
        locals.var_vgs_fb_dn10 = ((-locals.var_dvth_dn10) + locals.var_dppg_dn10);
        locals.var_vgs_fb_dn11 = ((-locals.var_dvth_dn11) + locals.var_dppg_dn11);
        locals.var_vgs_fb_dn12 = ((-locals.var_dvth_dn12) + locals.var_dppg_dn12);
        locals.var_vgs_fb_dn17 = ((-locals.var_dvth_dn17) + locals.var_dppg_dn17);
        locals.var_vgs_fb_rv = 0.0;

        let assign6990_e4665: f64 = (locals.var_cnst0soi * locals.var_c_fox_inv);
        locals.var_fac1 = assign6990_e4665;
        locals.var_fac1_dn0 = ((locals.var_cnst0soi_dn0 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn0));
        locals.var_fac1_dn2 = ((locals.var_cnst0soi_dn2 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn2));
        locals.var_fac1_dn6 = ((locals.var_cnst0soi_dn6 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn6));
        locals.var_fac1_dn7 = ((locals.var_cnst0soi_dn7 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn7));
        locals.var_fac1_dn10 = ((locals.var_cnst0soi_dn10 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn10));
        locals.var_fac1_dn11 = ((locals.var_cnst0soi_dn11 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn11));
        locals.var_fac1_dn12 = ((locals.var_cnst0soi_dn12 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn12));
        locals.var_fac1_dn17 = ((locals.var_cnst0soi_dn17 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn17));
        locals.var_fac1_rv = 0.0;

        let assign7000_e4668: f64 = (locals.var_fac1 * locals.var_fac1);
        locals.var_fac1p2 = assign7000_e4668;
        locals.var_fac1p2_dn0 = ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0));
        locals.var_fac1p2_dn2 = ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2));
        locals.var_fac1p2_dn6 = ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6));
        locals.var_fac1p2_dn7 = ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7));
        locals.var_fac1p2_dn10 = ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10));
        locals.var_fac1p2_dn11 = ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11));
        locals.var_fac1p2_dn12 = ((locals.var_fac1_dn12 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn12));
        locals.var_fac1p2_dn17 = ((locals.var_fac1_dn17 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn17));
        locals.var_fac1p2_rv = 0.0;

        let assign7010_e4671: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign7010_e4671;
        locals.var_guard109_rv = 0.0;

        let (assign7020_e4675,) = {
    if (locals.var_guard109 != 0.0) {
        (7.0,)
    } else {
        (locals.var_qdepb_dlt,)
    }
};
        locals.var_qdepb_dlt = assign7020_e4675;
        locals.var_qdepb_dlt_rv = 0.0;

        let (assign7030_e4681, assign7030_e4681_d_n0, assign7030_e4681_d_n2, assign7030_e4681_d_n6, assign7030_e4681_d_n7, assign7030_e4681_d_n10, assign7030_e4681_d_n11, assign7030_e4681_d_n12, assign7030_e4681_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7030_e4679: f64 = (locals.var_pb2 + 1.0);
        (assign7030_e4679, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn10, locals.var_pb2_dn11, locals.var_pb2_dn12, locals.var_pb2_dn17,)
    } else {
        (locals.var_vgp_ini, locals.var_vgp_ini_dn0, locals.var_vgp_ini_dn2, locals.var_vgp_ini_dn6, locals.var_vgp_ini_dn7, locals.var_vgp_ini_dn10, locals.var_vgp_ini_dn11, locals.var_vgp_ini_dn12, locals.var_vgp_ini_dn17,)
    }
};
        locals.var_vgp_ini = assign7030_e4681;
        locals.var_vgp_ini_dn0 = assign7030_e4681_d_n0;
        locals.var_vgp_ini_dn2 = assign7030_e4681_d_n2;
        locals.var_vgp_ini_dn6 = assign7030_e4681_d_n6;
        locals.var_vgp_ini_dn7 = assign7030_e4681_d_n7;
        locals.var_vgp_ini_dn10 = assign7030_e4681_d_n10;
        locals.var_vgp_ini_dn11 = assign7030_e4681_d_n11;
        locals.var_vgp_ini_dn12 = assign7030_e4681_d_n12;
        locals.var_vgp_ini_dn17 = assign7030_e4681_d_n17;
        locals.var_vgp_ini_rv = 0.0;

        let (assign7040_e4689, assign7040_e4689_d_n0, assign7040_e4689_d_n2, assign7040_e4689_d_n6, assign7040_e4689_d_n7, assign7040_e4689_d_n10, assign7040_e4689_d_n11, assign7040_e4689_d_n12, assign7040_e4689_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7040_e4685: f64 = (1.0 / locals.var_cnst1soi);
        let assign7040_e4687: f64 = (assign7040_e4685 / locals.var_cnstc_foxi);
        (assign7040_e4687, ((((-(locals.var_cnst1soi_dn0 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn0)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn2 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn2)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn6 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn6)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn7 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn7)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn10 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn10)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn11 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn11)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn12 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn12)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)), ((((-(locals.var_cnst1soi_dn17 / (locals.var_cnst1soi * locals.var_cnst1soi))) * locals.var_cnstc_foxi) - (assign7040_e4685 * locals.var_cnstc_foxi_dn17)) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn17,)
    }
};
        locals.var_t1 = assign7040_e4689;
        locals.var_t1_dn0 = assign7040_e4689_d_n0;
        locals.var_t1_dn2 = assign7040_e4689_d_n2;
        locals.var_t1_dn6 = assign7040_e4689_d_n6;
        locals.var_t1_dn7 = assign7040_e4689_d_n7;
        locals.var_t1_dn10 = assign7040_e4689_d_n10;
        locals.var_t1_dn11 = assign7040_e4689_d_n11;
        locals.var_t1_dn12 = assign7040_e4689_d_n12;
        locals.var_t1_dn17 = assign7040_e4689_d_n17;
        locals.var_t1_rv = 0.0;

        let (assign7050_e4701, assign7050_e4701_d_n0, assign7050_e4701_d_n2, assign7050_e4701_d_n6, assign7050_e4701_d_n7, assign7050_e4701_d_n10, assign7050_e4701_d_n11, assign7050_e4701_d_n12, assign7050_e4701_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7050_e4694: f64 = (locals.var_vgp_ini - locals.var_shift);
        let assign7050_e4695: f64 = (locals.var_t1 * assign7050_e4694);
        let assign7050_e4698: f64 = (locals.var_vgp_ini - locals.var_shift);
        let assign7050_e4699: f64 = (assign7050_e4695 * assign7050_e4698);
        (assign7050_e4699, ((((locals.var_t1_dn0 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn0 - locals.var_shift_dn0))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn0 - locals.var_shift_dn0))), ((((locals.var_t1_dn2 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn2 - locals.var_shift_dn2))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn2 - locals.var_shift_dn2))), ((((locals.var_t1_dn6 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn6 - locals.var_shift_dn6))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn6 - locals.var_shift_dn6))), ((((locals.var_t1_dn7 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn7 - locals.var_shift_dn7))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn7 - locals.var_shift_dn7))), ((((locals.var_t1_dn10 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn10 - locals.var_shift_dn10))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn10 - locals.var_shift_dn10))), ((((locals.var_t1_dn11 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn11 - locals.var_shift_dn11))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn11 - locals.var_shift_dn11))), ((((locals.var_t1_dn12 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn12 - locals.var_shift_dn12))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn12 - locals.var_shift_dn12))), ((((locals.var_t1_dn17 * assign7050_e4694) + (locals.var_t1 * (locals.var_vgp_ini_dn17 - locals.var_shift_dn17))) * assign7050_e4698) + (assign7050_e4695 * (locals.var_vgp_ini_dn17 - locals.var_shift_dn17))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn17,)
    }
};
        locals.var_t2 = assign7050_e4701;
        locals.var_t2_dn0 = assign7050_e4701_d_n0;
        locals.var_t2_dn2 = assign7050_e4701_d_n2;
        locals.var_t2_dn6 = assign7050_e4701_d_n6;
        locals.var_t2_dn7 = assign7050_e4701_d_n7;
        locals.var_t2_dn10 = assign7050_e4701_d_n10;
        locals.var_t2_dn11 = assign7050_e4701_d_n11;
        locals.var_t2_dn12 = assign7050_e4701_d_n12;
        locals.var_t2_dn17 = assign7050_e4701_d_n17;
        locals.var_t2_rv = 0.0;

        let (assign7060_e4711, assign7060_e4711_d_n0, assign7060_e4711_d_n2, assign7060_e4711_d_n6, assign7060_e4711_d_n7, assign7060_e4711_d_n10, assign7060_e4711_d_n11, assign7060_e4711_d_n12, assign7060_e4711_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7060_e4707: f64 = (locals.var_vgp_ini - locals.var_shift);
        let assign7060_e4708: f64 = (2.0 / assign7060_e4707);
        let assign7060_e4709: f64 = (locals.var_beta + assign7060_e4708);
        (assign7060_e4709, (-((2.0 * (locals.var_vgp_ini_dn0 - locals.var_shift_dn0)) / (assign7060_e4707 * assign7060_e4707))), (-((2.0 * (locals.var_vgp_ini_dn2 - locals.var_shift_dn2)) / (assign7060_e4707 * assign7060_e4707))), (-((2.0 * (locals.var_vgp_ini_dn6 - locals.var_shift_dn6)) / (assign7060_e4707 * assign7060_e4707))), (-((2.0 * (locals.var_vgp_ini_dn7 - locals.var_shift_dn7)) / (assign7060_e4707 * assign7060_e4707))), (locals.var_beta_dn10 + (-((2.0 * (locals.var_vgp_ini_dn10 - locals.var_shift_dn10)) / (assign7060_e4707 * assign7060_e4707)))), (-((2.0 * (locals.var_vgp_ini_dn11 - locals.var_shift_dn11)) / (assign7060_e4707 * assign7060_e4707))), (-((2.0 * (locals.var_vgp_ini_dn12 - locals.var_shift_dn12)) / (assign7060_e4707 * assign7060_e4707))), (-((2.0 * (locals.var_vgp_ini_dn17 - locals.var_shift_dn17)) / (assign7060_e4707 * assign7060_e4707))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn17,)
    }
};
        locals.var_t3 = assign7060_e4711;
        locals.var_t3_dn0 = assign7060_e4711_d_n0;
        locals.var_t3_dn2 = assign7060_e4711_d_n2;
        locals.var_t3_dn6 = assign7060_e4711_d_n6;
        locals.var_t3_dn7 = assign7060_e4711_d_n7;
        locals.var_t3_dn10 = assign7060_e4711_d_n10;
        locals.var_t3_dn11 = assign7060_e4711_d_n11;
        locals.var_t3_dn12 = assign7060_e4711_d_n12;
        locals.var_t3_dn17 = assign7060_e4711_d_n17;
        locals.var_t3_rv = 0.0;

        let (assign7070_e4718, assign7070_e4718_d_n0, assign7070_e4718_d_n2, assign7070_e4718_d_n6, assign7070_e4718_d_n7, assign7070_e4718_d_n10, assign7070_e4718_d_n11, assign7070_e4718_d_n12, assign7070_e4718_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7070_e4714: f64 = (locals.var_t2).ln();
        let assign7070_e4716: f64 = (assign7070_e4714 / locals.var_t3);
        (assign7070_e4716, ((((locals.var_t2_dn0 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn2 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn6 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn7 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn10 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn11 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn12 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn12)) / (locals.var_t3 * locals.var_t3)), ((((locals.var_t2_dn17 / locals.var_t2) * locals.var_t3) - (assign7070_e4714 * locals.var_t3_dn17)) / (locals.var_t3 * locals.var_t3)),)
    } else {
        (locals.var_ps0_inic, locals.var_ps0_inic_dn0, locals.var_ps0_inic_dn2, locals.var_ps0_inic_dn6, locals.var_ps0_inic_dn7, locals.var_ps0_inic_dn10, locals.var_ps0_inic_dn11, locals.var_ps0_inic_dn12, locals.var_ps0_inic_dn17,)
    }
};
        locals.var_ps0_inic = assign7070_e4718;
        locals.var_ps0_inic_dn0 = assign7070_e4718_d_n0;
        locals.var_ps0_inic_dn2 = assign7070_e4718_d_n2;
        locals.var_ps0_inic_dn6 = assign7070_e4718_d_n6;
        locals.var_ps0_inic_dn7 = assign7070_e4718_d_n7;
        locals.var_ps0_inic_dn10 = assign7070_e4718_d_n10;
        locals.var_ps0_inic_dn11 = assign7070_e4718_d_n11;
        locals.var_ps0_inic_dn12 = assign7070_e4718_d_n12;
        locals.var_ps0_inic_dn17 = assign7070_e4718_d_n17;
        locals.var_ps0_inic_rv = 0.0;

        let (assign7080_e4725, assign7080_e4725_d_n0, assign7080_e4725_d_n2, assign7080_e4725_d_n6, assign7080_e4725_d_n7, assign7080_e4725_d_n10, assign7080_e4725_d_n11, assign7080_e4725_d_n12, assign7080_e4725_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7080_e4722: f64 = (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic);
        let assign7080_e4723: f64 = (assign7080_e4722).sqrt();
        (assign7080_e4723, (((locals.var_cnst_2esi_q_nsubs_dn0 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn0)) / (2.0 * assign7080_e4723)), (((locals.var_cnst_2esi_q_nsubs_dn2 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn2)) / (2.0 * assign7080_e4723)), (((locals.var_cnst_2esi_q_nsubs_dn6 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn6)) / (2.0 * assign7080_e4723)), (((locals.var_cnst_2esi_q_nsubs_dn7 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn7)) / (2.0 * assign7080_e4723)), (((locals.var_cnst_2esi_q_nsubs_dn10 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn10)) / (2.0 * assign7080_e4723)), (((locals.var_cnst_2esi_q_nsubs_dn11 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn11)) / (2.0 * assign7080_e4723)), (((locals.var_cnst_2esi_q_nsubs_dn12 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn12)) / (2.0 * assign7080_e4723)), (((locals.var_cnst_2esi_q_nsubs_dn17 * locals.var_ps0_inic) + (locals.var_cnst_2esi_q_nsubs * locals.var_ps0_inic_dn17)) / (2.0 * assign7080_e4723)),)
    } else {
        (locals.var_wdsoi_ini0, locals.var_wdsoi_ini0_dn0, locals.var_wdsoi_ini0_dn2, locals.var_wdsoi_ini0_dn6, locals.var_wdsoi_ini0_dn7, locals.var_wdsoi_ini0_dn10, locals.var_wdsoi_ini0_dn11, locals.var_wdsoi_ini0_dn12, locals.var_wdsoi_ini0_dn17,)
    }
};
        locals.var_wdsoi_ini0 = assign7080_e4725;
        locals.var_wdsoi_ini0_dn0 = assign7080_e4725_d_n0;
        locals.var_wdsoi_ini0_dn2 = assign7080_e4725_d_n2;
        locals.var_wdsoi_ini0_dn6 = assign7080_e4725_d_n6;
        locals.var_wdsoi_ini0_dn7 = assign7080_e4725_d_n7;
        locals.var_wdsoi_ini0_dn10 = assign7080_e4725_d_n10;
        locals.var_wdsoi_ini0_dn11 = assign7080_e4725_d_n11;
        locals.var_wdsoi_ini0_dn12 = assign7080_e4725_d_n12;
        locals.var_wdsoi_ini0_dn17 = assign7080_e4725_d_n17;
        locals.var_wdsoi_ini0_rv = 0.0;

        let (assign7090_e4734, assign7090_e4734_d_n0, assign7090_e4734_d_n2, assign7090_e4734_d_n6, assign7090_e4734_d_n7, assign7090_e4734_d_n10, assign7090_e4734_d_n11, assign7090_e4734_d_n12, assign7090_e4734_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let (assign7090_e4732, assign7090_e4732_d_n0, assign7090_e4732_d_n2, assign7090_e4732_d_n6, assign7090_e4732_d_n7, assign7090_e4732_d_n10, assign7090_e4732_d_n11, assign7090_e4732_d_n12, assign7090_e4732_d_n17,) = {
            if (locals.var_wdsoi_ini0 > p.p237) {
                (p.p237, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_wdsoi_ini0, locals.var_wdsoi_ini0_dn0, locals.var_wdsoi_ini0_dn2, locals.var_wdsoi_ini0_dn6, locals.var_wdsoi_ini0_dn7, locals.var_wdsoi_ini0_dn10, locals.var_wdsoi_ini0_dn11, locals.var_wdsoi_ini0_dn12, locals.var_wdsoi_ini0_dn17,)
            }
        };
        (assign7090_e4732, assign7090_e4732_d_n0, assign7090_e4732_d_n2, assign7090_e4732_d_n6, assign7090_e4732_d_n7, assign7090_e4732_d_n10, assign7090_e4732_d_n11, assign7090_e4732_d_n12, assign7090_e4732_d_n17,)
    } else {
        (locals.var_wdsoi_ini0, locals.var_wdsoi_ini0_dn0, locals.var_wdsoi_ini0_dn2, locals.var_wdsoi_ini0_dn6, locals.var_wdsoi_ini0_dn7, locals.var_wdsoi_ini0_dn10, locals.var_wdsoi_ini0_dn11, locals.var_wdsoi_ini0_dn12, locals.var_wdsoi_ini0_dn17,)
    }
};
        locals.var_wdsoi_ini0 = assign7090_e4734;
        locals.var_wdsoi_ini0_dn0 = assign7090_e4734_d_n0;
        locals.var_wdsoi_ini0_dn2 = assign7090_e4734_d_n2;
        locals.var_wdsoi_ini0_dn6 = assign7090_e4734_d_n6;
        locals.var_wdsoi_ini0_dn7 = assign7090_e4734_d_n7;
        locals.var_wdsoi_ini0_dn10 = assign7090_e4734_d_n10;
        locals.var_wdsoi_ini0_dn11 = assign7090_e4734_d_n11;
        locals.var_wdsoi_ini0_dn12 = assign7090_e4734_d_n12;
        locals.var_wdsoi_ini0_dn17 = assign7090_e4734_d_n17;
        locals.var_wdsoi_ini0_rv = 0.0;

        let (assign7100_e4743, assign7100_e4743_d_n0, assign7100_e4743_d_n2, assign7100_e4743_d_n6, assign7100_e4743_d_n7, assign7100_e4743_d_n10, assign7100_e4743_d_n11, assign7100_e4743_d_n12, assign7100_e4743_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7100_e4737: f64 = (-1.6021918e-19);
        let assign7100_e4739: f64 = (assign7100_e4737 * locals.var_uc_nsubs);
        let assign7100_e4741: f64 = (assign7100_e4739 * locals.var_wdsoi_ini0);
        (assign7100_e4741, (((assign7100_e4737 * locals.var_uc_nsubs_dn0) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn0)), (((assign7100_e4737 * locals.var_uc_nsubs_dn2) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn2)), (((assign7100_e4737 * locals.var_uc_nsubs_dn6) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn6)), (((assign7100_e4737 * locals.var_uc_nsubs_dn7) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn7)), (((assign7100_e4737 * locals.var_uc_nsubs_dn10) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn10)), (((assign7100_e4737 * locals.var_uc_nsubs_dn11) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn11)), (((assign7100_e4737 * locals.var_uc_nsubs_dn12) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn12)), (((assign7100_e4737 * locals.var_uc_nsubs_dn17) * locals.var_wdsoi_ini0) + (assign7100_e4739 * locals.var_wdsoi_ini0_dn17)),)
    } else {
        (locals.var_q_wdsoi_max, locals.var_q_wdsoi_max_dn0, locals.var_q_wdsoi_max_dn2, locals.var_q_wdsoi_max_dn6, locals.var_q_wdsoi_max_dn7, locals.var_q_wdsoi_max_dn10, locals.var_q_wdsoi_max_dn11, locals.var_q_wdsoi_max_dn12, locals.var_q_wdsoi_max_dn17,)
    }
};
        locals.var_q_wdsoi_max = assign7100_e4743;
        locals.var_q_wdsoi_max_dn0 = assign7100_e4743_d_n0;
        locals.var_q_wdsoi_max_dn2 = assign7100_e4743_d_n2;
        locals.var_q_wdsoi_max_dn6 = assign7100_e4743_d_n6;
        locals.var_q_wdsoi_max_dn7 = assign7100_e4743_d_n7;
        locals.var_q_wdsoi_max_dn10 = assign7100_e4743_d_n10;
        locals.var_q_wdsoi_max_dn11 = assign7100_e4743_d_n11;
        locals.var_q_wdsoi_max_dn12 = assign7100_e4743_d_n12;
        locals.var_q_wdsoi_max_dn17 = assign7100_e4743_d_n17;
        locals.var_q_wdsoi_max_rv = 0.0;

        let (assign7110_e4747,) = {
    if (locals.var_guard109 != 0.0) {
        (p.p237,)
    } else {
        (locals.var_t_soi,)
    }
};
        locals.var_t_soi = assign7110_e4747;
        locals.var_t_soi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7120_e4756, assign7120_e4756_d_n0, assign7120_e4756_d_n2, assign7120_e4756_d_n6, assign7120_e4756_d_n7, assign7120_e4756_d_n10, assign7120_e4756_d_n11, assign7120_e4756_d_n12, assign7120_e4756_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7120_e4750: f64 = (-1.6021918e-19);
        let assign7120_e4752: f64 = (assign7120_e4750 * locals.var_uc_nsubs);
        let assign7120_e4754: f64 = (assign7120_e4752 * locals.var_t_soi);
        (assign7120_e4754, ((assign7120_e4750 * locals.var_uc_nsubs_dn0) * locals.var_t_soi), ((assign7120_e4750 * locals.var_uc_nsubs_dn2) * locals.var_t_soi), ((assign7120_e4750 * locals.var_uc_nsubs_dn6) * locals.var_t_soi), ((assign7120_e4750 * locals.var_uc_nsubs_dn7) * locals.var_t_soi), ((assign7120_e4750 * locals.var_uc_nsubs_dn10) * locals.var_t_soi), ((assign7120_e4750 * locals.var_uc_nsubs_dn11) * locals.var_t_soi), ((assign7120_e4750 * locals.var_uc_nsubs_dn12) * locals.var_t_soi), ((assign7120_e4750 * locals.var_uc_nsubs_dn17) * locals.var_t_soi),)
    } else {
        (locals.var_q_fd_soi, locals.var_q_fd_soi_dn0, locals.var_q_fd_soi_dn2, locals.var_q_fd_soi_dn6, locals.var_q_fd_soi_dn7, locals.var_q_fd_soi_dn10, locals.var_q_fd_soi_dn11, locals.var_q_fd_soi_dn12, locals.var_q_fd_soi_dn17,)
    }
};
        locals.var_q_fd_soi = assign7120_e4756;
        locals.var_q_fd_soi_dn0 = assign7120_e4756_d_n0;
        locals.var_q_fd_soi_dn2 = assign7120_e4756_d_n2;
        locals.var_q_fd_soi_dn6 = assign7120_e4756_d_n6;
        locals.var_q_fd_soi_dn7 = assign7120_e4756_d_n7;
        locals.var_q_fd_soi_dn10 = assign7120_e4756_d_n10;
        locals.var_q_fd_soi_dn11 = assign7120_e4756_d_n11;
        locals.var_q_fd_soi_dn12 = assign7120_e4756_d_n12;
        locals.var_q_fd_soi_dn17 = assign7120_e4756_d_n17;
        locals.var_q_fd_soi_rv = 0.0;

        let (assign7130_e4760,) = {
    if (locals.var_guard109 != 0.0) {
        (1.5,)
    } else {
        (locals.var_wdsoi_ini1_dlt,)
    }
};
        locals.var_wdsoi_ini1_dlt = assign7130_e4760;
        locals.var_wdsoi_ini1_dlt_rv = 0.0;

        let (assign7140_e4766,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7140_e4764: f64 = (1.034943e-10 / locals.var_t_soi);
        (assign7140_e4764,)
    } else {
        (locals.var_c_soi__blk110,)
    }
};
        locals.var_c_soi__blk110 = assign7140_e4766;
        locals.var_c_soi__blk110_rv = 0.0;

        let (assign7150_e4772,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7150_e4770: f64 = (1.0 / locals.var_c_soi__blk110);
        (assign7150_e4770,)
    } else {
        (locals.var_c_soi_inv__blk111,)
    }
};
        locals.var_c_soi_inv__blk111 = assign7150_e4772;
        locals.var_c_soi_inv__blk111_rv = 0.0;

        let (assign7160_e4779, assign7160_e4779_d_n0, assign7160_e4779_d_n2, assign7160_e4779_d_n6, assign7160_e4779_d_n7, assign7160_e4779_d_n10, assign7160_e4779_d_n11, assign7160_e4779_d_n12, assign7160_e4779_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7160_e4775: f64 = (-locals.var_q_fd_soi);
        let assign7160_e4777: f64 = (assign7160_e4775 * 0.001);
        (assign7160_e4777, ((-locals.var_q_fd_soi_dn0) * 0.001), ((-locals.var_q_fd_soi_dn2) * 0.001), ((-locals.var_q_fd_soi_dn6) * 0.001), ((-locals.var_q_fd_soi_dn7) * 0.001), ((-locals.var_q_fd_soi_dn10) * 0.001), ((-locals.var_q_fd_soi_dn11) * 0.001), ((-locals.var_q_fd_soi_dn12) * 0.001), ((-locals.var_q_fd_soi_dn17) * 0.001),)
    } else {
        (locals.var_q_fd_dlt1, locals.var_q_fd_dlt1_dn0, locals.var_q_fd_dlt1_dn2, locals.var_q_fd_dlt1_dn6, locals.var_q_fd_dlt1_dn7, locals.var_q_fd_dlt1_dn10, locals.var_q_fd_dlt1_dn11, locals.var_q_fd_dlt1_dn12, locals.var_q_fd_dlt1_dn17,)
    }
};
        locals.var_q_fd_dlt1 = assign7160_e4779;
        locals.var_q_fd_dlt1_dn0 = assign7160_e4779_d_n0;
        locals.var_q_fd_dlt1_dn2 = assign7160_e4779_d_n2;
        locals.var_q_fd_dlt1_dn6 = assign7160_e4779_d_n6;
        locals.var_q_fd_dlt1_dn7 = assign7160_e4779_d_n7;
        locals.var_q_fd_dlt1_dn10 = assign7160_e4779_d_n10;
        locals.var_q_fd_dlt1_dn11 = assign7160_e4779_d_n11;
        locals.var_q_fd_dlt1_dn12 = assign7160_e4779_d_n12;
        locals.var_q_fd_dlt1_dn17 = assign7160_e4779_d_n17;
        locals.var_q_fd_dlt1_rv = 0.0;

        let (assign7170_e4786, assign7170_e4786_d_n0, assign7170_e4786_d_n2, assign7170_e4786_d_n6, assign7170_e4786_d_n7, assign7170_e4786_d_n10, assign7170_e4786_d_n11, assign7170_e4786_d_n12, assign7170_e4786_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7170_e4782: f64 = (-locals.var_q_fd_soi);
        let assign7170_e4784: f64 = (assign7170_e4782 * 1e-5);
        (assign7170_e4784, ((-locals.var_q_fd_soi_dn0) * 1e-5), ((-locals.var_q_fd_soi_dn2) * 1e-5), ((-locals.var_q_fd_soi_dn6) * 1e-5), ((-locals.var_q_fd_soi_dn7) * 1e-5), ((-locals.var_q_fd_soi_dn10) * 1e-5), ((-locals.var_q_fd_soi_dn11) * 1e-5), ((-locals.var_q_fd_soi_dn12) * 1e-5), ((-locals.var_q_fd_soi_dn17) * 1e-5),)
    } else {
        (locals.var_q_fd_dlt2, locals.var_q_fd_dlt2_dn0, locals.var_q_fd_dlt2_dn2, locals.var_q_fd_dlt2_dn6, locals.var_q_fd_dlt2_dn7, locals.var_q_fd_dlt2_dn10, locals.var_q_fd_dlt2_dn11, locals.var_q_fd_dlt2_dn12, locals.var_q_fd_dlt2_dn17,)
    }
};
        locals.var_q_fd_dlt2 = assign7170_e4786;
        locals.var_q_fd_dlt2_dn0 = assign7170_e4786_d_n0;
        locals.var_q_fd_dlt2_dn2 = assign7170_e4786_d_n2;
        locals.var_q_fd_dlt2_dn6 = assign7170_e4786_d_n6;
        locals.var_q_fd_dlt2_dn7 = assign7170_e4786_d_n7;
        locals.var_q_fd_dlt2_dn10 = assign7170_e4786_d_n10;
        locals.var_q_fd_dlt2_dn11 = assign7170_e4786_d_n11;
        locals.var_q_fd_dlt2_dn12 = assign7170_e4786_d_n12;
        locals.var_q_fd_dlt2_dn17 = assign7170_e4786_d_n17;
        locals.var_q_fd_dlt2_rv = 0.0;

        let (assign7180_e4794, assign7180_e4794_d_n0, assign7180_e4794_d_n2, assign7180_e4794_d_n6, assign7180_e4794_d_n7, assign7180_e4794_d_n10, assign7180_e4794_d_n11, assign7180_e4794_d_n12, assign7180_e4794_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (p.p39 != 0.0)) {
        let assign7180_e4792: f64 = (locals.var_vbsz + locals.var_vbi_soi);
        (assign7180_e4792, (locals.var_vbsz_dn0 + locals.var_vbi_soi_dn0), (locals.var_vbsz_dn2 + locals.var_vbi_soi_dn2), (locals.var_vbsz_dn6 + locals.var_vbi_soi_dn6), (locals.var_vbsz_dn7 + locals.var_vbi_soi_dn7), (locals.var_vbsz_dn10 + locals.var_vbi_soi_dn10), (locals.var_vbsz_dn11 + locals.var_vbi_soi_dn11), (locals.var_vbsz_dn12 + locals.var_vbi_soi_dn12), (locals.var_vbsz_dn17 + locals.var_vbi_soi_dn17),)
    } else {
        (locals.var_vbsbiz, locals.var_vbsbiz_dn0, locals.var_vbsbiz_dn2, locals.var_vbsbiz_dn6, locals.var_vbsbiz_dn7, locals.var_vbsbiz_dn10, locals.var_vbsbiz_dn11, locals.var_vbsbiz_dn12, locals.var_vbsbiz_dn17,)
    }
};
        locals.var_vbsbiz = assign7180_e4794;
        locals.var_vbsbiz_dn0 = assign7180_e4794_d_n0;
        locals.var_vbsbiz_dn2 = assign7180_e4794_d_n2;
        locals.var_vbsbiz_dn6 = assign7180_e4794_d_n6;
        locals.var_vbsbiz_dn7 = assign7180_e4794_d_n7;
        locals.var_vbsbiz_dn10 = assign7180_e4794_d_n10;
        locals.var_vbsbiz_dn11 = assign7180_e4794_d_n11;
        locals.var_vbsbiz_dn12 = assign7180_e4794_d_n12;
        locals.var_vbsbiz_dn17 = assign7180_e4794_d_n17;
        locals.var_vbsbiz_rv = 0.0;

        let (assign7190_e4803, assign7190_e4803_d_n0, assign7190_e4803_d_n2, assign7190_e4803_d_n6, assign7190_e4803_d_n7, assign7190_e4803_d_n10, assign7190_e4803_d_n11, assign7190_e4803_d_n12, assign7190_e4803_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (p.p39 == 0.0)) {
        let assign7190_e4801: f64 = (locals.var_vbs + locals.var_vbi_soi);
        (assign7190_e4801, (locals.var_vbs_dn0 + locals.var_vbi_soi_dn0), (locals.var_vbs_dn2 + locals.var_vbi_soi_dn2), (locals.var_vbs_dn6 + locals.var_vbi_soi_dn6), (locals.var_vbs_dn7 + locals.var_vbi_soi_dn7), (locals.var_vbs_dn10 + locals.var_vbi_soi_dn10), (locals.var_vbs_dn11 + locals.var_vbi_soi_dn11), (locals.var_vbs_dn12 + locals.var_vbi_soi_dn12), (locals.var_vbs_dn17 + locals.var_vbi_soi_dn17),)
    } else {
        (locals.var_vbsbiz, locals.var_vbsbiz_dn0, locals.var_vbsbiz_dn2, locals.var_vbsbiz_dn6, locals.var_vbsbiz_dn7, locals.var_vbsbiz_dn10, locals.var_vbsbiz_dn11, locals.var_vbsbiz_dn12, locals.var_vbsbiz_dn17,)
    }
};
        locals.var_vbsbiz = assign7190_e4803;
        locals.var_vbsbiz_dn0 = assign7190_e4803_d_n0;
        locals.var_vbsbiz_dn2 = assign7190_e4803_d_n2;
        locals.var_vbsbiz_dn6 = assign7190_e4803_d_n6;
        locals.var_vbsbiz_dn7 = assign7190_e4803_d_n7;
        locals.var_vbsbiz_dn10 = assign7190_e4803_d_n10;
        locals.var_vbsbiz_dn11 = assign7190_e4803_d_n11;
        locals.var_vbsbiz_dn12 = assign7190_e4803_d_n12;
        locals.var_vbsbiz_dn17 = assign7190_e4803_d_n17;
        locals.var_vbsbiz_rv = 0.0;

        let (assign7200_e4814, assign7200_e4814_d_n0, assign7200_e4814_d_n2, assign7200_e4814_d_n6, assign7200_e4814_d_n7, assign7200_e4814_d_n10, assign7200_e4814_d_n11, assign7200_e4814_d_n12, assign7200_e4814_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7200_e4807: f64 = (2.0 / locals.var_beta);
        let assign7200_e4810: f64 = (locals.var_mks_nsubb / locals.var_nin);
        let assign7200_e4811: f64 = (assign7200_e4810).ln();
        let assign7200_e4812: f64 = (assign7200_e4807 * assign7200_e4811);
        (assign7200_e4812, (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign7200_e4810)), (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign7200_e4810)), (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign7200_e4810)), (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign7200_e4810)), (((-((2.0 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign7200_e4811) + (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign7200_e4810))), (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) / assign7200_e4810)), (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn12) / (locals.var_nin * locals.var_nin))) / assign7200_e4810)), (assign7200_e4807 * ((-((locals.var_mks_nsubb * locals.var_nin_dn17) / (locals.var_nin * locals.var_nin))) / assign7200_e4810)),)
    } else {
        (locals.var_pb2_bulk, locals.var_pb2_bulk_dn0, locals.var_pb2_bulk_dn2, locals.var_pb2_bulk_dn6, locals.var_pb2_bulk_dn7, locals.var_pb2_bulk_dn10, locals.var_pb2_bulk_dn11, locals.var_pb2_bulk_dn12, locals.var_pb2_bulk_dn17,)
    }
};
        locals.var_pb2_bulk = assign7200_e4814;
        locals.var_pb2_bulk_dn0 = assign7200_e4814_d_n0;
        locals.var_pb2_bulk_dn2 = assign7200_e4814_d_n2;
        locals.var_pb2_bulk_dn6 = assign7200_e4814_d_n6;
        locals.var_pb2_bulk_dn7 = assign7200_e4814_d_n7;
        locals.var_pb2_bulk_dn10 = assign7200_e4814_d_n10;
        locals.var_pb2_bulk_dn11 = assign7200_e4814_d_n11;
        locals.var_pb2_bulk_dn12 = assign7200_e4814_d_n12;
        locals.var_pb2_bulk_dn17 = assign7200_e4814_d_n17;
        locals.var_pb2_bulk_rv = 0.0;

        let (assign7210_e4824, assign7210_e4824_d_n10,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7210_e4818: f64 = (locals.var_cnst0bulk * locals.var_cnst0bulk);
        let assign7210_e4820: f64 = (assign7210_e4818 * locals.var_c_box_fd_inv);
        let assign7210_e4822: f64 = (assign7210_e4820 * locals.var_c_box_fd_inv);
        (assign7210_e4822, ((((locals.var_cnst0bulk_dn10 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn10)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv),)
    } else {
        (locals.var_t0__blk117, locals.var_t0__blk117_dn10,)
    }
};
        locals.var_t0__blk117 = assign7210_e4824;
        locals.var_t0__blk117_dn10 = assign7210_e4824_d_n10;
        locals.var_t0__blk117_rv = 0.0;

        let (assign7220_e4829, assign7220_e4829_d_n0, assign7220_e4829_d_n2, assign7220_e4829_d_n6, assign7220_e4829_d_n7, assign7220_e4829_d_n10, assign7220_e4829_d_n11, assign7220_e4829_d_n12, assign7220_e4829_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7220_e4827: f64 = (-locals.var_vbsbiz);
        (assign7220_e4827, (-locals.var_vbsbiz_dn0), (-locals.var_vbsbiz_dn2), (-locals.var_vbsbiz_dn6), (-locals.var_vbsbiz_dn7), (-locals.var_vbsbiz_dn10), (-locals.var_vbsbiz_dn11), (-locals.var_vbsbiz_dn12), (-locals.var_vbsbiz_dn17),)
    } else {
        (locals.var_t1__blk118, locals.var_t1__blk118_dn0, locals.var_t1__blk118_dn2, locals.var_t1__blk118_dn6, locals.var_t1__blk118_dn7, locals.var_t1__blk118_dn10, locals.var_t1__blk118_dn11, locals.var_t1__blk118_dn12, locals.var_t1__blk118_dn17,)
    }
};
        locals.var_t1__blk118 = assign7220_e4829;
        locals.var_t1__blk118_dn0 = assign7220_e4829_d_n0;
        locals.var_t1__blk118_dn2 = assign7220_e4829_d_n2;
        locals.var_t1__blk118_dn6 = assign7220_e4829_d_n6;
        locals.var_t1__blk118_dn7 = assign7220_e4829_d_n7;
        locals.var_t1__blk118_dn10 = assign7220_e4829_d_n10;
        locals.var_t1__blk118_dn11 = assign7220_e4829_d_n11;
        locals.var_t1__blk118_dn12 = assign7220_e4829_d_n12;
        locals.var_t1__blk118_dn17 = assign7220_e4829_d_n17;
        locals.var_t1__blk118_rv = 0.0;

        let (assign7230_e4855, assign7230_e4855_d_n0, assign7230_e4855_d_n2, assign7230_e4855_d_n6, assign7230_e4855_d_n7, assign7230_e4855_d_n10, assign7230_e4855_d_n11, assign7230_e4855_d_n12, assign7230_e4855_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7230_e4833: f64 = (2.0 * locals.var_t1__blk118);
        let assign7230_e4836: f64 = (locals.var_t0__blk117 * locals.var_beta);
        let assign7230_e4837: f64 = (assign7230_e4833 + assign7230_e4836);
        let assign7230_e4840: f64 = (2.0 * locals.var_t1__blk118);
        let assign7230_e4843: f64 = (locals.var_t0__blk117 * locals.var_beta);
        let assign7230_e4844: f64 = (assign7230_e4840 + assign7230_e4843);
        let assign7230_e4845: f64 = (assign7230_e4837 * assign7230_e4844);
        let assign7230_e4849: f64 = (locals.var_t1__blk118 * locals.var_t1__blk118);
        let assign7230_e4851: f64 = (assign7230_e4849 + locals.var_t0__blk117);
        let assign7230_e4852: f64 = (4.0 * assign7230_e4851);
        let assign7230_e4853: f64 = (assign7230_e4845 - assign7230_e4852);
        (assign7230_e4853, ((((2.0 * locals.var_t1__blk118_dn0) * assign7230_e4844) + (assign7230_e4837 * (2.0 * locals.var_t1__blk118_dn0))) - (4.0 * ((locals.var_t1__blk118_dn0 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn0)))), ((((2.0 * locals.var_t1__blk118_dn2) * assign7230_e4844) + (assign7230_e4837 * (2.0 * locals.var_t1__blk118_dn2))) - (4.0 * ((locals.var_t1__blk118_dn2 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn2)))), ((((2.0 * locals.var_t1__blk118_dn6) * assign7230_e4844) + (assign7230_e4837 * (2.0 * locals.var_t1__blk118_dn6))) - (4.0 * ((locals.var_t1__blk118_dn6 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn6)))), ((((2.0 * locals.var_t1__blk118_dn7) * assign7230_e4844) + (assign7230_e4837 * (2.0 * locals.var_t1__blk118_dn7))) - (4.0 * ((locals.var_t1__blk118_dn7 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn7)))), (((((2.0 * locals.var_t1__blk118_dn10) + ((locals.var_t0__blk117_dn10 * locals.var_beta) + (locals.var_t0__blk117 * locals.var_beta_dn10))) * assign7230_e4844) + (assign7230_e4837 * ((2.0 * locals.var_t1__blk118_dn10) + ((locals.var_t0__blk117_dn10 * locals.var_beta) + (locals.var_t0__blk117 * locals.var_beta_dn10))))) - (4.0 * (((locals.var_t1__blk118_dn10 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn10)) + locals.var_t0__blk117_dn10))), ((((2.0 * locals.var_t1__blk118_dn11) * assign7230_e4844) + (assign7230_e4837 * (2.0 * locals.var_t1__blk118_dn11))) - (4.0 * ((locals.var_t1__blk118_dn11 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn11)))), ((((2.0 * locals.var_t1__blk118_dn12) * assign7230_e4844) + (assign7230_e4837 * (2.0 * locals.var_t1__blk118_dn12))) - (4.0 * ((locals.var_t1__blk118_dn12 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn12)))), ((((2.0 * locals.var_t1__blk118_dn17) * assign7230_e4844) + (assign7230_e4837 * (2.0 * locals.var_t1__blk118_dn17))) - (4.0 * ((locals.var_t1__blk118_dn17 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn17)))),)
    } else {
        (locals.var_t2__blk119, locals.var_t2__blk119_dn0, locals.var_t2__blk119_dn2, locals.var_t2__blk119_dn6, locals.var_t2__blk119_dn7, locals.var_t2__blk119_dn10, locals.var_t2__blk119_dn11, locals.var_t2__blk119_dn12, locals.var_t2__blk119_dn17,)
    }
};
        locals.var_t2__blk119 = assign7230_e4855;
        locals.var_t2__blk119_dn0 = assign7230_e4855_d_n0;
        locals.var_t2__blk119_dn2 = assign7230_e4855_d_n2;
        locals.var_t2__blk119_dn6 = assign7230_e4855_d_n6;
        locals.var_t2__blk119_dn7 = assign7230_e4855_d_n7;
        locals.var_t2__blk119_dn10 = assign7230_e4855_d_n10;
        locals.var_t2__blk119_dn11 = assign7230_e4855_d_n11;
        locals.var_t2__blk119_dn12 = assign7230_e4855_d_n12;
        locals.var_t2__blk119_dn17 = assign7230_e4855_d_n17;
        locals.var_t2__blk119_rv = 0.0;

        let (assign7240_e4868, assign7240_e4868_d_n0, assign7240_e4868_d_n2, assign7240_e4868_d_n6, assign7240_e4868_d_n7, assign7240_e4868_d_n10, assign7240_e4868_d_n11, assign7240_e4868_d_n12, assign7240_e4868_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7240_e4860: f64 = (10.0 * 2.220446049250313e-16);
        let (assign7240_e4866, assign7240_e4866_d_n0, assign7240_e4866_d_n2, assign7240_e4866_d_n6, assign7240_e4866_d_n7, assign7240_e4866_d_n10, assign7240_e4866_d_n11, assign7240_e4866_d_n12, assign7240_e4866_d_n17,) = {
            if (locals.var_t2__blk119 >= assign7240_e4860) {
                (locals.var_t2__blk119, locals.var_t2__blk119_dn0, locals.var_t2__blk119_dn2, locals.var_t2__blk119_dn6, locals.var_t2__blk119_dn7, locals.var_t2__blk119_dn10, locals.var_t2__blk119_dn11, locals.var_t2__blk119_dn12, locals.var_t2__blk119_dn17,)
            } else {
                let assign7240_e4865: f64 = (10.0 * 2.220446049250313e-16);
                (assign7240_e4865, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign7240_e4866, assign7240_e4866_d_n0, assign7240_e4866_d_n2, assign7240_e4866_d_n6, assign7240_e4866_d_n7, assign7240_e4866_d_n10, assign7240_e4866_d_n11, assign7240_e4866_d_n12, assign7240_e4866_d_n17,)
    } else {
        (locals.var_t2__blk119, locals.var_t2__blk119_dn0, locals.var_t2__blk119_dn2, locals.var_t2__blk119_dn6, locals.var_t2__blk119_dn7, locals.var_t2__blk119_dn10, locals.var_t2__blk119_dn11, locals.var_t2__blk119_dn12, locals.var_t2__blk119_dn17,)
    }
};
        locals.var_t2__blk119 = assign7240_e4868;
        locals.var_t2__blk119_dn0 = assign7240_e4868_d_n0;
        locals.var_t2__blk119_dn2 = assign7240_e4868_d_n2;
        locals.var_t2__blk119_dn6 = assign7240_e4868_d_n6;
        locals.var_t2__blk119_dn7 = assign7240_e4868_d_n7;
        locals.var_t2__blk119_dn10 = assign7240_e4868_d_n10;
        locals.var_t2__blk119_dn11 = assign7240_e4868_d_n11;
        locals.var_t2__blk119_dn12 = assign7240_e4868_d_n12;
        locals.var_t2__blk119_dn17 = assign7240_e4868_d_n17;
        locals.var_t2__blk119_rv = 0.0;

        let (assign7250_e4873, assign7250_e4873_d_n0, assign7250_e4873_d_n2, assign7250_e4873_d_n6, assign7250_e4873_d_n7, assign7250_e4873_d_n10, assign7250_e4873_d_n11, assign7250_e4873_d_n12, assign7250_e4873_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7250_e4871: f64 = (locals.var_t2__blk119).sqrt();
        (assign7250_e4871, (locals.var_t2__blk119_dn0 / (2.0 * assign7250_e4871)), (locals.var_t2__blk119_dn2 / (2.0 * assign7250_e4871)), (locals.var_t2__blk119_dn6 / (2.0 * assign7250_e4871)), (locals.var_t2__blk119_dn7 / (2.0 * assign7250_e4871)), (locals.var_t2__blk119_dn10 / (2.0 * assign7250_e4871)), (locals.var_t2__blk119_dn11 / (2.0 * assign7250_e4871)), (locals.var_t2__blk119_dn12 / (2.0 * assign7250_e4871)), (locals.var_t2__blk119_dn17 / (2.0 * assign7250_e4871)),)
    } else {
        (locals.var_t2__blk119, locals.var_t2__blk119_dn0, locals.var_t2__blk119_dn2, locals.var_t2__blk119_dn6, locals.var_t2__blk119_dn7, locals.var_t2__blk119_dn10, locals.var_t2__blk119_dn11, locals.var_t2__blk119_dn12, locals.var_t2__blk119_dn17,)
    }
};
        locals.var_t2__blk119 = assign7250_e4873;
        locals.var_t2__blk119_dn0 = assign7250_e4873_d_n0;
        locals.var_t2__blk119_dn2 = assign7250_e4873_d_n2;
        locals.var_t2__blk119_dn6 = assign7250_e4873_d_n6;
        locals.var_t2__blk119_dn7 = assign7250_e4873_d_n7;
        locals.var_t2__blk119_dn10 = assign7250_e4873_d_n10;
        locals.var_t2__blk119_dn11 = assign7250_e4873_d_n11;
        locals.var_t2__blk119_dn12 = assign7250_e4873_d_n12;
        locals.var_t2__blk119_dn17 = assign7250_e4873_d_n17;
        locals.var_t2__blk119_rv = 0.0;

        let (assign7260_e4883, assign7260_e4883_d_n0, assign7260_e4883_d_n2, assign7260_e4883_d_n6, assign7260_e4883_d_n7, assign7260_e4883_d_n10, assign7260_e4883_d_n11, assign7260_e4883_d_n12, assign7260_e4883_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7260_e4877: f64 = (2.0 * locals.var_t1__blk118);
        let assign7260_e4880: f64 = (locals.var_t0__blk117 * locals.var_beta);
        let assign7260_e4881: f64 = (assign7260_e4877 + assign7260_e4880);
        (assign7260_e4881, (2.0 * locals.var_t1__blk118_dn0), (2.0 * locals.var_t1__blk118_dn2), (2.0 * locals.var_t1__blk118_dn6), (2.0 * locals.var_t1__blk118_dn7), ((2.0 * locals.var_t1__blk118_dn10) + ((locals.var_t0__blk117_dn10 * locals.var_beta) + (locals.var_t0__blk117 * locals.var_beta_dn10))), (2.0 * locals.var_t1__blk118_dn11), (2.0 * locals.var_t1__blk118_dn12), (2.0 * locals.var_t1__blk118_dn17),)
    } else {
        (locals.var_t3__blk120, locals.var_t3__blk120_dn0, locals.var_t3__blk120_dn2, locals.var_t3__blk120_dn6, locals.var_t3__blk120_dn7, locals.var_t3__blk120_dn10, locals.var_t3__blk120_dn11, locals.var_t3__blk120_dn12, locals.var_t3__blk120_dn17,)
    }
};
        locals.var_t3__blk120 = assign7260_e4883;
        locals.var_t3__blk120_dn0 = assign7260_e4883_d_n0;
        locals.var_t3__blk120_dn2 = assign7260_e4883_d_n2;
        locals.var_t3__blk120_dn6 = assign7260_e4883_d_n6;
        locals.var_t3__blk120_dn7 = assign7260_e4883_d_n7;
        locals.var_t3__blk120_dn10 = assign7260_e4883_d_n10;
        locals.var_t3__blk120_dn11 = assign7260_e4883_d_n11;
        locals.var_t3__blk120_dn12 = assign7260_e4883_d_n12;
        locals.var_t3__blk120_dn17 = assign7260_e4883_d_n17;
        locals.var_t3__blk120_rv = 0.0;

        let (assign7270_e4891, assign7270_e4891_d_n0, assign7270_e4891_d_n2, assign7270_e4891_d_n6, assign7270_e4891_d_n7, assign7270_e4891_d_n10, assign7270_e4891_d_n11, assign7270_e4891_d_n12, assign7270_e4891_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7270_e4887: f64 = (locals.var_t3__blk120 - locals.var_t2__blk119);
        let assign7270_e4889: f64 = (assign7270_e4887 / 2.0);
        (assign7270_e4889, ((locals.var_t3__blk120_dn0 - locals.var_t2__blk119_dn0) / 2.0), ((locals.var_t3__blk120_dn2 - locals.var_t2__blk119_dn2) / 2.0), ((locals.var_t3__blk120_dn6 - locals.var_t2__blk119_dn6) / 2.0), ((locals.var_t3__blk120_dn7 - locals.var_t2__blk119_dn7) / 2.0), ((locals.var_t3__blk120_dn10 - locals.var_t2__blk119_dn10) / 2.0), ((locals.var_t3__blk120_dn11 - locals.var_t2__blk119_dn11) / 2.0), ((locals.var_t3__blk120_dn12 - locals.var_t2__blk119_dn12) / 2.0), ((locals.var_t3__blk120_dn17 - locals.var_t2__blk119_dn17) / 2.0),)
    } else {
        (locals.var_psb_inia__blk121, locals.var_psb_inia__blk121_dn0, locals.var_psb_inia__blk121_dn2, locals.var_psb_inia__blk121_dn6, locals.var_psb_inia__blk121_dn7, locals.var_psb_inia__blk121_dn10, locals.var_psb_inia__blk121_dn11, locals.var_psb_inia__blk121_dn12, locals.var_psb_inia__blk121_dn17,)
    }
};
        locals.var_psb_inia__blk121 = assign7270_e4891;
        locals.var_psb_inia__blk121_dn0 = assign7270_e4891_d_n0;
        locals.var_psb_inia__blk121_dn2 = assign7270_e4891_d_n2;
        locals.var_psb_inia__blk121_dn6 = assign7270_e4891_d_n6;
        locals.var_psb_inia__blk121_dn7 = assign7270_e4891_d_n7;
        locals.var_psb_inia__blk121_dn10 = assign7270_e4891_d_n10;
        locals.var_psb_inia__blk121_dn11 = assign7270_e4891_d_n11;
        locals.var_psb_inia__blk121_dn12 = assign7270_e4891_d_n12;
        locals.var_psb_inia__blk121_dn17 = assign7270_e4891_d_n17;
        locals.var_psb_inia__blk121_rv = 0.0;

        let (assign7280_e4908, assign7280_e4908_d_n0, assign7280_e4908_d_n2, assign7280_e4908_d_n6, assign7280_e4908_d_n7, assign7280_e4908_d_n10, assign7280_e4908_d_n11, assign7280_e4908_d_n12, assign7280_e4908_d_n17,) = {
    if (locals.var_guard109 != 0.0) {
        let assign7280_e4895: f64 = (locals.var_t1__blk118 * locals.var_t1__blk118);
        let assign7280_e4897: f64 = (assign7280_e4895 / locals.var_t0__blk117);
        let assign7280_e4899: f64 = (assign7280_e4897 / locals.var_cnst1bulk);
        let assign7280_e4900: f64 = (assign7280_e4899).ln();
        let assign7280_e4904: f64 = (2.0 / locals.var_t1__blk118);
        let assign7280_e4905: f64 = (locals.var_beta + assign7280_e4904);
        let assign7280_e4906: f64 = (assign7280_e4900 / assign7280_e4905);
        (assign7280_e4906, ((((((((((locals.var_t1__blk118_dn0 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn0)) / locals.var_t0__blk117) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn0)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (-((2.0 * locals.var_t1__blk118_dn0) / (locals.var_t1__blk118 * locals.var_t1__blk118))))) / (assign7280_e4905 * assign7280_e4905)), ((((((((((locals.var_t1__blk118_dn2 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn2)) / locals.var_t0__blk117) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn2)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (-((2.0 * locals.var_t1__blk118_dn2) / (locals.var_t1__blk118 * locals.var_t1__blk118))))) / (assign7280_e4905 * assign7280_e4905)), ((((((((((locals.var_t1__blk118_dn6 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn6)) / locals.var_t0__blk117) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn6)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (-((2.0 * locals.var_t1__blk118_dn6) / (locals.var_t1__blk118 * locals.var_t1__blk118))))) / (assign7280_e4905 * assign7280_e4905)), ((((((((((locals.var_t1__blk118_dn7 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn7)) / locals.var_t0__blk117) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn7)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (-((2.0 * locals.var_t1__blk118_dn7) / (locals.var_t1__blk118 * locals.var_t1__blk118))))) / (assign7280_e4905 * assign7280_e4905)), ((((((((((((locals.var_t1__blk118_dn10 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn10)) * locals.var_t0__blk117) - (assign7280_e4895 * locals.var_t0__blk117_dn10)) / (locals.var_t0__blk117 * locals.var_t0__blk117)) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn10)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (locals.var_beta_dn10 + (-((2.0 * locals.var_t1__blk118_dn10) / (locals.var_t1__blk118 * locals.var_t1__blk118)))))) / (assign7280_e4905 * assign7280_e4905)), ((((((((((locals.var_t1__blk118_dn11 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn11)) / locals.var_t0__blk117) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn11)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (-((2.0 * locals.var_t1__blk118_dn11) / (locals.var_t1__blk118 * locals.var_t1__blk118))))) / (assign7280_e4905 * assign7280_e4905)), ((((((((((locals.var_t1__blk118_dn12 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn12)) / locals.var_t0__blk117) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn12)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (-((2.0 * locals.var_t1__blk118_dn12) / (locals.var_t1__blk118 * locals.var_t1__blk118))))) / (assign7280_e4905 * assign7280_e4905)), ((((((((((locals.var_t1__blk118_dn17 * locals.var_t1__blk118) + (locals.var_t1__blk118 * locals.var_t1__blk118_dn17)) / locals.var_t0__blk117) * locals.var_cnst1bulk) - (assign7280_e4897 * locals.var_cnst1bulk_dn17)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign7280_e4899) * assign7280_e4905) - (assign7280_e4900 * (-((2.0 * locals.var_t1__blk118_dn17) / (locals.var_t1__blk118 * locals.var_t1__blk118))))) / (assign7280_e4905 * assign7280_e4905)),)
    } else {
        (locals.var_psb_inib__blk122, locals.var_psb_inib__blk122_dn0, locals.var_psb_inib__blk122_dn2, locals.var_psb_inib__blk122_dn6, locals.var_psb_inib__blk122_dn7, locals.var_psb_inib__blk122_dn10, locals.var_psb_inib__blk122_dn11, locals.var_psb_inib__blk122_dn12, locals.var_psb_inib__blk122_dn17,)
    }
};
        locals.var_psb_inib__blk122 = assign7280_e4908;
        locals.var_psb_inib__blk122_dn0 = assign7280_e4908_d_n0;
        locals.var_psb_inib__blk122_dn2 = assign7280_e4908_d_n2;
        locals.var_psb_inib__blk122_dn6 = assign7280_e4908_d_n6;
        locals.var_psb_inib__blk122_dn7 = assign7280_e4908_d_n7;
        locals.var_psb_inib__blk122_dn10 = assign7280_e4908_d_n10;
        locals.var_psb_inib__blk122_dn11 = assign7280_e4908_d_n11;
        locals.var_psb_inib__blk122_dn12 = assign7280_e4908_d_n12;
        locals.var_psb_inib__blk122_dn17 = assign7280_e4908_d_n17;
        locals.var_psb_inib__blk122_rv = 0.0;

        let assign7290_e4911: f64 = if locals.var_psb_inia__blk121 < locals.var_pb2_bulk { 1.0 } else { 0.0 };
        locals.var_guard123 = assign7290_e4911;
        locals.var_guard123_rv = 0.0;

        let (assign7300_e4917, assign7300_e4917_d_n0, assign7300_e4917_d_n2, assign7300_e4917_d_n6, assign7300_e4917_d_n7, assign7300_e4917_d_n10, assign7300_e4917_d_n11, assign7300_e4917_d_n12, assign7300_e4917_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard123 != 0.0)) {
        (locals.var_psb_inia__blk121, locals.var_psb_inia__blk121_dn0, locals.var_psb_inia__blk121_dn2, locals.var_psb_inia__blk121_dn6, locals.var_psb_inia__blk121_dn7, locals.var_psb_inia__blk121_dn10, locals.var_psb_inia__blk121_dn11, locals.var_psb_inia__blk121_dn12, locals.var_psb_inia__blk121_dn17,)
    } else {
        (locals.var_phi_s0_bulk_0, locals.var_phi_s0_bulk_0_dn0, locals.var_phi_s0_bulk_0_dn2, locals.var_phi_s0_bulk_0_dn6, locals.var_phi_s0_bulk_0_dn7, locals.var_phi_s0_bulk_0_dn10, locals.var_phi_s0_bulk_0_dn11, locals.var_phi_s0_bulk_0_dn12, locals.var_phi_s0_bulk_0_dn17,)
    }
};
        locals.var_phi_s0_bulk_0 = assign7300_e4917;
        locals.var_phi_s0_bulk_0_dn0 = assign7300_e4917_d_n0;
        locals.var_phi_s0_bulk_0_dn2 = assign7300_e4917_d_n2;
        locals.var_phi_s0_bulk_0_dn6 = assign7300_e4917_d_n6;
        locals.var_phi_s0_bulk_0_dn7 = assign7300_e4917_d_n7;
        locals.var_phi_s0_bulk_0_dn10 = assign7300_e4917_d_n10;
        locals.var_phi_s0_bulk_0_dn11 = assign7300_e4917_d_n11;
        locals.var_phi_s0_bulk_0_dn12 = assign7300_e4917_d_n12;
        locals.var_phi_s0_bulk_0_dn17 = assign7300_e4917_d_n17;
        locals.var_phi_s0_bulk_0_rv = 0.0;

        let (assign7310_e4928, assign7310_e4928_d_n0, assign7310_e4928_d_n2, assign7310_e4928_d_n6, assign7310_e4928_d_n7, assign7310_e4928_d_n10, assign7310_e4928_d_n11, assign7310_e4928_d_n12, assign7310_e4928_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard123 == 0.0)) {
        let assign7310_e4924: f64 = (locals.var_psb_inib__blk122 - locals.var_psb_inia__blk121);
        let assign7310_e4926: f64 = (assign7310_e4924 - 0.0008);
        (assign7310_e4926, (locals.var_psb_inib__blk122_dn0 - locals.var_psb_inia__blk121_dn0), (locals.var_psb_inib__blk122_dn2 - locals.var_psb_inia__blk121_dn2), (locals.var_psb_inib__blk122_dn6 - locals.var_psb_inia__blk121_dn6), (locals.var_psb_inib__blk122_dn7 - locals.var_psb_inia__blk121_dn7), (locals.var_psb_inib__blk122_dn10 - locals.var_psb_inia__blk121_dn10), (locals.var_psb_inib__blk122_dn11 - locals.var_psb_inia__blk121_dn11), (locals.var_psb_inib__blk122_dn12 - locals.var_psb_inia__blk121_dn12), (locals.var_psb_inib__blk122_dn17 - locals.var_psb_inia__blk121_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign7310_e4928;
        locals.var_tmf1_dn0 = assign7310_e4928_d_n0;
        locals.var_tmf1_dn2 = assign7310_e4928_d_n2;
        locals.var_tmf1_dn6 = assign7310_e4928_d_n6;
        locals.var_tmf1_dn7 = assign7310_e4928_d_n7;
        locals.var_tmf1_dn10 = assign7310_e4928_d_n10;
        locals.var_tmf1_dn11 = assign7310_e4928_d_n11;
        locals.var_tmf1_dn12 = assign7310_e4928_d_n12;
        locals.var_tmf1_dn17 = assign7310_e4928_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign7320_e4939, assign7320_e4939_d_n0, assign7320_e4939_d_n2, assign7320_e4939_d_n6, assign7320_e4939_d_n7, assign7320_e4939_d_n10, assign7320_e4939_d_n11, assign7320_e4939_d_n12, assign7320_e4939_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard123 == 0.0)) {
        let assign7320_e4935: f64 = (4.0 * locals.var_psb_inib__blk122);
        let assign7320_e4937: f64 = (assign7320_e4935 * 0.0008);
        (assign7320_e4937, ((4.0 * locals.var_psb_inib__blk122_dn0) * 0.0008), ((4.0 * locals.var_psb_inib__blk122_dn2) * 0.0008), ((4.0 * locals.var_psb_inib__blk122_dn6) * 0.0008), ((4.0 * locals.var_psb_inib__blk122_dn7) * 0.0008), ((4.0 * locals.var_psb_inib__blk122_dn10) * 0.0008), ((4.0 * locals.var_psb_inib__blk122_dn11) * 0.0008), ((4.0 * locals.var_psb_inib__blk122_dn12) * 0.0008), ((4.0 * locals.var_psb_inib__blk122_dn17) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign7320_e4939;
        locals.var_tmf2_dn0 = assign7320_e4939_d_n0;
        locals.var_tmf2_dn2 = assign7320_e4939_d_n2;
        locals.var_tmf2_dn6 = assign7320_e4939_d_n6;
        locals.var_tmf2_dn7 = assign7320_e4939_d_n7;
        locals.var_tmf2_dn10 = assign7320_e4939_d_n10;
        locals.var_tmf2_dn11 = assign7320_e4939_d_n11;
        locals.var_tmf2_dn12 = assign7320_e4939_d_n12;
        locals.var_tmf2_dn17 = assign7320_e4939_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign7330_e4952, assign7330_e4952_d_n0, assign7330_e4952_d_n2, assign7330_e4952_d_n6, assign7330_e4952_d_n7, assign7330_e4952_d_n10, assign7330_e4952_d_n11, assign7330_e4952_d_n12, assign7330_e4952_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard123 == 0.0)) {
        let (assign7330_e4950, assign7330_e4950_d_n0, assign7330_e4950_d_n2, assign7330_e4950_d_n6, assign7330_e4950_d_n7, assign7330_e4950_d_n10, assign7330_e4950_d_n11, assign7330_e4950_d_n12, assign7330_e4950_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign7330_e4949: f64 = (-locals.var_tmf2);
                (assign7330_e4949, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign7330_e4950, assign7330_e4950_d_n0, assign7330_e4950_d_n2, assign7330_e4950_d_n6, assign7330_e4950_d_n7, assign7330_e4950_d_n10, assign7330_e4950_d_n11, assign7330_e4950_d_n12, assign7330_e4950_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign7330_e4952;
        locals.var_tmf2_dn0 = assign7330_e4952_d_n0;
        locals.var_tmf2_dn2 = assign7330_e4952_d_n2;
        locals.var_tmf2_dn6 = assign7330_e4952_d_n6;
        locals.var_tmf2_dn7 = assign7330_e4952_d_n7;
        locals.var_tmf2_dn10 = assign7330_e4952_d_n10;
        locals.var_tmf2_dn11 = assign7330_e4952_d_n11;
        locals.var_tmf2_dn12 = assign7330_e4952_d_n12;
        locals.var_tmf2_dn17 = assign7330_e4952_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign7340_e4964, assign7340_e4964_d_n0, assign7340_e4964_d_n2, assign7340_e4964_d_n6, assign7340_e4964_d_n7, assign7340_e4964_d_n10, assign7340_e4964_d_n11, assign7340_e4964_d_n12, assign7340_e4964_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard123 == 0.0)) {
        let assign7340_e4959: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign7340_e4961: f64 = (assign7340_e4959 + locals.var_tmf2);
        let assign7340_e4962: f64 = (assign7340_e4961).sqrt();
        (assign7340_e4962, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign7340_e4962)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign7340_e4962)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign7340_e4962)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign7340_e4962)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign7340_e4962)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign7340_e4962)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign7340_e4962)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign7340_e4962)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign7340_e4964;
        locals.var_tmf2_dn0 = assign7340_e4964_d_n0;
        locals.var_tmf2_dn2 = assign7340_e4964_d_n2;
        locals.var_tmf2_dn6 = assign7340_e4964_d_n6;
        locals.var_tmf2_dn7 = assign7340_e4964_d_n7;
        locals.var_tmf2_dn10 = assign7340_e4964_d_n10;
        locals.var_tmf2_dn11 = assign7340_e4964_d_n11;
        locals.var_tmf2_dn12 = assign7340_e4964_d_n12;
        locals.var_tmf2_dn17 = assign7340_e4964_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign7350_e4977, assign7350_e4977_d_n0, assign7350_e4977_d_n2, assign7350_e4977_d_n6, assign7350_e4977_d_n7, assign7350_e4977_d_n10, assign7350_e4977_d_n11, assign7350_e4977_d_n12, assign7350_e4977_d_n17,) = {
    if ((locals.var_guard109 != 0.0) && (locals.var_guard123 == 0.0)) {
        let assign7350_e4973: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign7350_e4974: f64 = (0.5 * assign7350_e4973);
        let assign7350_e4975: f64 = (locals.var_psb_inib__blk122 - assign7350_e4974);
        (assign7350_e4975, (locals.var_psb_inib__blk122_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psb_inib__blk122_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psb_inib__blk122_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psb_inib__blk122_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psb_inib__blk122_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psb_inib__blk122_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psb_inib__blk122_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psb_inib__blk122_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_phi_s0_bulk_0, locals.var_phi_s0_bulk_0_dn0, locals.var_phi_s0_bulk_0_dn2, locals.var_phi_s0_bulk_0_dn6, locals.var_phi_s0_bulk_0_dn7, locals.var_phi_s0_bulk_0_dn10, locals.var_phi_s0_bulk_0_dn11, locals.var_phi_s0_bulk_0_dn12, locals.var_phi_s0_bulk_0_dn17,)
    }
};
        locals.var_phi_s0_bulk_0 = assign7350_e4977;
        locals.var_phi_s0_bulk_0_dn0 = assign7350_e4977_d_n0;
        locals.var_phi_s0_bulk_0_dn2 = assign7350_e4977_d_n2;
        locals.var_phi_s0_bulk_0_dn6 = assign7350_e4977_d_n6;
        locals.var_phi_s0_bulk_0_dn7 = assign7350_e4977_d_n7;
        locals.var_phi_s0_bulk_0_dn10 = assign7350_e4977_d_n10;
        locals.var_phi_s0_bulk_0_dn11 = assign7350_e4977_d_n11;
        locals.var_phi_s0_bulk_0_dn12 = assign7350_e4977_d_n12;
        locals.var_phi_s0_bulk_0_dn17 = assign7350_e4977_d_n17;
        locals.var_phi_s0_bulk_0_rv = 0.0;

        let (assign7360_e4981,) = {
    if (locals.var_guard109 != 0.0) {
        (0.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign7360_e4981;
        locals.var_lp_s0_rv = 0.0;

    }
}
