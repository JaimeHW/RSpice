#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_9(
        locals: &mut StampLocals,
    ) {
        locals.var_qbd = 0.0;
        locals.var_qbd_dn0 = 0.0;
        locals.var_qbd_dn2 = 0.0;
        locals.var_qbd_dn4 = 0.0;
        locals.var_qbd_dn5 = 0.0;
        locals.var_qbd_dn6 = 0.0;
        locals.var_qbd_dn7 = 0.0;
        locals.var_qbd_dn8 = 0.0;
        locals.var_qbd_dn9 = 0.0;
        locals.var_qbd_dn10 = 0.0;
        locals.var_qbd_dn13 = 0.0;
        locals.var_qbd_dn15 = 0.0;
        locals.var_qbd_dn16 = 0.0;
        locals.var_qbd_dn17 = 0.0;
        locals.var_qbd_rv = 0.0;

        locals.var_qbsi = 0.0;
        locals.var_qbsi_dn0 = 0.0;
        locals.var_qbsi_dn2 = 0.0;
        locals.var_qbsi_dn4 = 0.0;
        locals.var_qbsi_dn5 = 0.0;
        locals.var_qbsi_dn6 = 0.0;
        locals.var_qbsi_dn7 = 0.0;
        locals.var_qbsi_dn8 = 0.0;
        locals.var_qbsi_dn9 = 0.0;
        locals.var_qbsi_dn10 = 0.0;
        locals.var_qbsi_dn13 = 0.0;
        locals.var_qbsi_rv = 0.0;

        locals.var_qbdi = 0.0;
        locals.var_qbdi_dn0 = 0.0;
        locals.var_qbdi_dn2 = 0.0;
        locals.var_qbdi_dn4 = 0.0;
        locals.var_qbdi_dn5 = 0.0;
        locals.var_qbdi_dn6 = 0.0;
        locals.var_qbdi_dn7 = 0.0;
        locals.var_qbdi_dn8 = 0.0;
        locals.var_qbdi_dn9 = 0.0;
        locals.var_qbdi_dn10 = 0.0;
        locals.var_qbdi_dn13 = 0.0;
        locals.var_qbdi_rv = 0.0;

        locals.var_czbd = 0.0;
        locals.var_czbd_dn0 = 0.0;
        locals.var_czbd_dn2 = 0.0;
        locals.var_czbd_dn4 = 0.0;
        locals.var_czbd_dn5 = 0.0;
        locals.var_czbd_dn6 = 0.0;
        locals.var_czbd_dn7 = 0.0;
        locals.var_czbd_dn8 = 0.0;
        locals.var_czbd_dn9 = 0.0;
        locals.var_czbd_dn10 = 0.0;
        locals.var_czbd_dn13 = 0.0;
        locals.var_czbd_rv = 0.0;

        locals.var_czbdsw = 0.0;
        locals.var_czbdsw_dn0 = 0.0;
        locals.var_czbdsw_dn2 = 0.0;
        locals.var_czbdsw_dn4 = 0.0;
        locals.var_czbdsw_dn5 = 0.0;
        locals.var_czbdsw_dn6 = 0.0;
        locals.var_czbdsw_dn7 = 0.0;
        locals.var_czbdsw_dn8 = 0.0;
        locals.var_czbdsw_dn9 = 0.0;
        locals.var_czbdsw_dn10 = 0.0;
        locals.var_czbdsw_dn13 = 0.0;
        locals.var_czbdsw_rv = 0.0;

        locals.var_czbdswg = 0.0;
        locals.var_czbdswg_dn0 = 0.0;
        locals.var_czbdswg_dn2 = 0.0;
        locals.var_czbdswg_dn4 = 0.0;
        locals.var_czbdswg_dn5 = 0.0;
        locals.var_czbdswg_dn6 = 0.0;
        locals.var_czbdswg_dn7 = 0.0;
        locals.var_czbdswg_dn8 = 0.0;
        locals.var_czbdswg_dn9 = 0.0;
        locals.var_czbdswg_dn10 = 0.0;
        locals.var_czbdswg_dn13 = 0.0;
        locals.var_czbdswg_rv = 0.0;

        locals.var_czbs = 0.0;
        locals.var_czbs_dn0 = 0.0;
        locals.var_czbs_dn2 = 0.0;
        locals.var_czbs_dn4 = 0.0;
        locals.var_czbs_dn5 = 0.0;
        locals.var_czbs_dn6 = 0.0;
        locals.var_czbs_dn7 = 0.0;
        locals.var_czbs_dn8 = 0.0;
        locals.var_czbs_dn9 = 0.0;
        locals.var_czbs_dn10 = 0.0;
        locals.var_czbs_dn13 = 0.0;
        locals.var_czbs_rv = 0.0;

        locals.var_czbssw = 0.0;
        locals.var_czbssw_dn0 = 0.0;
        locals.var_czbssw_dn2 = 0.0;
        locals.var_czbssw_dn4 = 0.0;
        locals.var_czbssw_dn5 = 0.0;
        locals.var_czbssw_dn6 = 0.0;
        locals.var_czbssw_dn7 = 0.0;
        locals.var_czbssw_dn8 = 0.0;
        locals.var_czbssw_dn9 = 0.0;
        locals.var_czbssw_dn10 = 0.0;
        locals.var_czbssw_dn13 = 0.0;
        locals.var_czbssw_rv = 0.0;

        locals.var_czbsswg = 0.0;
        locals.var_czbsswg_dn0 = 0.0;
        locals.var_czbsswg_dn2 = 0.0;
        locals.var_czbsswg_dn4 = 0.0;
        locals.var_czbsswg_dn5 = 0.0;
        locals.var_czbsswg_dn6 = 0.0;
        locals.var_czbsswg_dn7 = 0.0;
        locals.var_czbsswg_dn8 = 0.0;
        locals.var_czbsswg_dn9 = 0.0;
        locals.var_czbsswg_dn10 = 0.0;
        locals.var_czbsswg_dn13 = 0.0;
        locals.var_czbsswg_rv = 0.0;

        locals.var_pzbd = 0.0;
        locals.var_pzbd_dn0 = 0.0;
        locals.var_pzbd_dn2 = 0.0;
        locals.var_pzbd_dn4 = 0.0;
        locals.var_pzbd_dn5 = 0.0;
        locals.var_pzbd_dn6 = 0.0;
        locals.var_pzbd_dn7 = 0.0;
        locals.var_pzbd_dn8 = 0.0;
        locals.var_pzbd_dn9 = 0.0;
        locals.var_pzbd_dn10 = 0.0;
        locals.var_pzbd_dn13 = 0.0;
        locals.var_pzbd_rv = 0.0;

        locals.var_pzbdsw = 0.0;
        locals.var_pzbdsw_dn0 = 0.0;
        locals.var_pzbdsw_dn2 = 0.0;
        locals.var_pzbdsw_dn4 = 0.0;
        locals.var_pzbdsw_dn5 = 0.0;
        locals.var_pzbdsw_dn6 = 0.0;
        locals.var_pzbdsw_dn7 = 0.0;
        locals.var_pzbdsw_dn8 = 0.0;
        locals.var_pzbdsw_dn9 = 0.0;
        locals.var_pzbdsw_dn10 = 0.0;
        locals.var_pzbdsw_dn13 = 0.0;
        locals.var_pzbdsw_rv = 0.0;

        locals.var_pzbdswg = 0.0;
        locals.var_pzbdswg_dn0 = 0.0;
        locals.var_pzbdswg_dn2 = 0.0;
        locals.var_pzbdswg_dn4 = 0.0;
        locals.var_pzbdswg_dn5 = 0.0;
        locals.var_pzbdswg_dn6 = 0.0;
        locals.var_pzbdswg_dn7 = 0.0;
        locals.var_pzbdswg_dn8 = 0.0;
        locals.var_pzbdswg_dn9 = 0.0;
        locals.var_pzbdswg_dn10 = 0.0;
        locals.var_pzbdswg_dn13 = 0.0;
        locals.var_pzbdswg_rv = 0.0;

        locals.var_pzbs = 0.0;
        locals.var_pzbs_dn0 = 0.0;
        locals.var_pzbs_dn2 = 0.0;
        locals.var_pzbs_dn4 = 0.0;
        locals.var_pzbs_dn5 = 0.0;
        locals.var_pzbs_dn6 = 0.0;
        locals.var_pzbs_dn7 = 0.0;
        locals.var_pzbs_dn8 = 0.0;
        locals.var_pzbs_dn9 = 0.0;
        locals.var_pzbs_dn10 = 0.0;
        locals.var_pzbs_dn13 = 0.0;
        locals.var_pzbs_rv = 0.0;

        locals.var_pzbssw = 0.0;
        locals.var_pzbssw_dn0 = 0.0;
        locals.var_pzbssw_dn2 = 0.0;
        locals.var_pzbssw_dn4 = 0.0;
        locals.var_pzbssw_dn5 = 0.0;
        locals.var_pzbssw_dn6 = 0.0;
        locals.var_pzbssw_dn7 = 0.0;
        locals.var_pzbssw_dn8 = 0.0;
        locals.var_pzbssw_dn9 = 0.0;
        locals.var_pzbssw_dn10 = 0.0;
        locals.var_pzbssw_dn13 = 0.0;
        locals.var_pzbssw_rv = 0.0;

        locals.var_pzbsswg = 0.0;
        locals.var_pzbsswg_dn0 = 0.0;
        locals.var_pzbsswg_dn2 = 0.0;
        locals.var_pzbsswg_dn4 = 0.0;
        locals.var_pzbsswg_dn5 = 0.0;
        locals.var_pzbsswg_dn6 = 0.0;
        locals.var_pzbsswg_dn7 = 0.0;
        locals.var_pzbsswg_dn8 = 0.0;
        locals.var_pzbsswg_dn9 = 0.0;
        locals.var_pzbsswg_dn10 = 0.0;
        locals.var_pzbsswg_dn13 = 0.0;
        locals.var_pzbsswg_rv = 0.0;

        locals.var_sarg = 0.0;
        locals.var_sarg_dn0 = 0.0;
        locals.var_sarg_dn2 = 0.0;
        locals.var_sarg_dn4 = 0.0;
        locals.var_sarg_dn5 = 0.0;
        locals.var_sarg_dn6 = 0.0;
        locals.var_sarg_dn7 = 0.0;
        locals.var_sarg_dn8 = 0.0;
        locals.var_sarg_dn9 = 0.0;
        locals.var_sarg_dn10 = 0.0;
        locals.var_sarg_dn13 = 0.0;
        locals.var_sarg_rv = 0.0;

        locals.var_vsbs = 0.0;
        locals.var_vsbs_dn2 = 0.0;
        locals.var_vsbs_dn10 = 0.0;
        locals.var_vsbs_rv = 0.0;

        locals.var_vdbd = 0.0;
        locals.var_vdbd_dn0 = 0.0;
        locals.var_vdbd_dn9 = 0.0;
        locals.var_vdbd_rv = 0.0;

        locals.var_vbs_jct = 0.0;
        locals.var_vbs_jct_dn2 = 0.0;
        locals.var_vbs_jct_dn10 = 0.0;
        locals.var_vbs_jct_rv = 0.0;

        locals.var_vbd_jct = 0.0;
        locals.var_vbd_jct_dn0 = 0.0;
        locals.var_vbd_jct_dn9 = 0.0;
        locals.var_vbd_jct_rv = 0.0;

        locals.var_vbpsp = 0.0;
        locals.var_vbpsp_dn7 = 0.0;
        locals.var_vbpsp_dn8 = 0.0;
        locals.var_vbpsp_rv = 0.0;

        locals.var_vbpdp = 0.0;
        locals.var_vbpdp_dn5 = 0.0;
        locals.var_vbpdp_dn8 = 0.0;
        locals.var_vbpdp_rv = 0.0;

        locals.var_vbsi_jct = 0.0;
        locals.var_vbsi_jct_dn7 = 0.0;
        locals.var_vbsi_jct_dn8 = 0.0;
        locals.var_vbsi_jct_rv = 0.0;

        locals.var_vbdi_jct = 0.0;
        locals.var_vbdi_jct_dn5 = 0.0;
        locals.var_vbdi_jct_dn8 = 0.0;
        locals.var_vbdi_jct_rv = 0.0;

        locals.var_exptempd = 0.0;
        locals.var_exptempd_dn0 = 0.0;
        locals.var_exptempd_dn2 = 0.0;
        locals.var_exptempd_dn4 = 0.0;
        locals.var_exptempd_dn5 = 0.0;
        locals.var_exptempd_dn6 = 0.0;
        locals.var_exptempd_dn7 = 0.0;
        locals.var_exptempd_dn8 = 0.0;
        locals.var_exptempd_dn9 = 0.0;
        locals.var_exptempd_dn10 = 0.0;
        locals.var_exptempd_dn13 = 0.0;
        locals.var_exptempd_rv = 0.0;

        locals.var_exptemps = 0.0;
        locals.var_exptemps_dn0 = 0.0;
        locals.var_exptemps_dn2 = 0.0;
        locals.var_exptemps_dn4 = 0.0;
        locals.var_exptemps_dn5 = 0.0;
        locals.var_exptemps_dn6 = 0.0;
        locals.var_exptemps_dn7 = 0.0;
        locals.var_exptemps_dn8 = 0.0;
        locals.var_exptemps_dn9 = 0.0;
        locals.var_exptemps_dn10 = 0.0;
        locals.var_exptemps_dn13 = 0.0;
        locals.var_exptemps_rv = 0.0;

        locals.var_isbd = 0.0;
        locals.var_isbd_dn0 = 0.0;
        locals.var_isbd_dn2 = 0.0;
        locals.var_isbd_dn4 = 0.0;
        locals.var_isbd_dn5 = 0.0;
        locals.var_isbd_dn6 = 0.0;
        locals.var_isbd_dn7 = 0.0;
        locals.var_isbd_dn8 = 0.0;
        locals.var_isbd_dn9 = 0.0;
        locals.var_isbd_dn10 = 0.0;
        locals.var_isbd_dn13 = 0.0;
        locals.var_isbd_rv = 0.0;

        locals.var_isbs = 0.0;
        locals.var_isbs_dn0 = 0.0;
        locals.var_isbs_dn2 = 0.0;
        locals.var_isbs_dn4 = 0.0;
        locals.var_isbs_dn5 = 0.0;
        locals.var_isbs_dn6 = 0.0;
        locals.var_isbs_dn7 = 0.0;
        locals.var_isbs_dn8 = 0.0;
        locals.var_isbs_dn9 = 0.0;
        locals.var_isbs_dn10 = 0.0;
        locals.var_isbs_dn13 = 0.0;
        locals.var_isbs_rv = 0.0;

        locals.var_jd_expcd = 0.0;
        locals.var_jd_expcd_dn0 = 0.0;
        locals.var_jd_expcd_dn2 = 0.0;
        locals.var_jd_expcd_dn4 = 0.0;
        locals.var_jd_expcd_dn5 = 0.0;
        locals.var_jd_expcd_dn6 = 0.0;
        locals.var_jd_expcd_dn7 = 0.0;
        locals.var_jd_expcd_dn8 = 0.0;
        locals.var_jd_expcd_dn9 = 0.0;
        locals.var_jd_expcd_dn10 = 0.0;
        locals.var_jd_expcd_dn13 = 0.0;
        locals.var_jd_expcd_rv = 0.0;

        locals.var_jd_expcs = 0.0;
        locals.var_jd_expcs_dn0 = 0.0;
        locals.var_jd_expcs_dn2 = 0.0;
        locals.var_jd_expcs_dn4 = 0.0;
        locals.var_jd_expcs_dn5 = 0.0;
        locals.var_jd_expcs_dn6 = 0.0;
        locals.var_jd_expcs_dn7 = 0.0;
        locals.var_jd_expcs_dn8 = 0.0;
        locals.var_jd_expcs_dn9 = 0.0;
        locals.var_jd_expcs_dn10 = 0.0;
        locals.var_jd_expcs_dn13 = 0.0;
        locals.var_jd_expcs_rv = 0.0;

        locals.var_vbdt = 0.0;
        locals.var_vbdt_dn0 = 0.0;
        locals.var_vbdt_dn2 = 0.0;
        locals.var_vbdt_dn4 = 0.0;
        locals.var_vbdt_dn5 = 0.0;
        locals.var_vbdt_dn6 = 0.0;
        locals.var_vbdt_dn7 = 0.0;
        locals.var_vbdt_dn8 = 0.0;
        locals.var_vbdt_dn9 = 0.0;
        locals.var_vbdt_dn10 = 0.0;
        locals.var_vbdt_dn13 = 0.0;
        locals.var_vbdt_rv = 0.0;

        locals.var_vbst = 0.0;
        locals.var_vbst_dn0 = 0.0;
        locals.var_vbst_dn2 = 0.0;
        locals.var_vbst_dn4 = 0.0;
        locals.var_vbst_dn5 = 0.0;
        locals.var_vbst_dn6 = 0.0;
        locals.var_vbst_dn7 = 0.0;
        locals.var_vbst_dn8 = 0.0;
        locals.var_vbst_dn9 = 0.0;
        locals.var_vbst_dn10 = 0.0;
        locals.var_vbst_dn13 = 0.0;
        locals.var_vbst_rv = 0.0;

        locals.var_jd_nvtm_invd = 0.0;
        locals.var_jd_nvtm_invd_dn0 = 0.0;
        locals.var_jd_nvtm_invd_dn2 = 0.0;
        locals.var_jd_nvtm_invd_dn4 = 0.0;
        locals.var_jd_nvtm_invd_dn5 = 0.0;
        locals.var_jd_nvtm_invd_dn6 = 0.0;
        locals.var_jd_nvtm_invd_dn7 = 0.0;
        locals.var_jd_nvtm_invd_dn8 = 0.0;
        locals.var_jd_nvtm_invd_dn9 = 0.0;
        locals.var_jd_nvtm_invd_dn10 = 0.0;
        locals.var_jd_nvtm_invd_dn13 = 0.0;
        locals.var_jd_nvtm_invd_rv = 0.0;

        locals.var_jd_nvtm_invs = 0.0;
        locals.var_jd_nvtm_invs_dn0 = 0.0;
        locals.var_jd_nvtm_invs_dn2 = 0.0;
        locals.var_jd_nvtm_invs_dn4 = 0.0;
        locals.var_jd_nvtm_invs_dn5 = 0.0;
        locals.var_jd_nvtm_invs_dn6 = 0.0;
        locals.var_jd_nvtm_invs_dn7 = 0.0;
        locals.var_jd_nvtm_invs_dn8 = 0.0;
        locals.var_jd_nvtm_invs_dn9 = 0.0;
        locals.var_jd_nvtm_invs_dn10 = 0.0;
        locals.var_jd_nvtm_invs_dn13 = 0.0;
        locals.var_jd_nvtm_invs_rv = 0.0;

        locals.var_end_of_part_1 = 0.0;
        locals.var_end_of_part_1_rv = 0.0;

        locals.var_flg_brk1 = 0.0;
        locals.var_flg_brk1_rv = 0.0;

        locals.var_start_of_loopl = 0.0;
        locals.var_start_of_loopl_rv = 0.0;

        locals.var_flg_brk2 = 0.0;
        locals.var_flg_brk2_rv = 0.0;

        locals.var_start_of_mobility = 0.0;
        locals.var_start_of_mobility_rv = 0.0;

        locals.var_qbd_qs = 0.0;
        locals.var_qbd_qs_dn0 = 0.0;
        locals.var_qbd_qs_dn2 = 0.0;
        locals.var_qbd_qs_dn4 = 0.0;
        locals.var_qbd_qs_dn5 = 0.0;
        locals.var_qbd_qs_dn6 = 0.0;
        locals.var_qbd_qs_dn7 = 0.0;
        locals.var_qbd_qs_dn8 = 0.0;
        locals.var_qbd_qs_dn9 = 0.0;
        locals.var_qbd_qs_dn10 = 0.0;
        locals.var_qbd_qs_dn13 = 0.0;
        locals.var_qbd_qs_rv = 0.0;

        locals.var_isbd_btm = 0.0;
        locals.var_isbd_btm_dn0 = 0.0;
        locals.var_isbd_btm_dn2 = 0.0;
        locals.var_isbd_btm_dn4 = 0.0;
        locals.var_isbd_btm_dn5 = 0.0;
        locals.var_isbd_btm_dn6 = 0.0;
        locals.var_isbd_btm_dn7 = 0.0;
        locals.var_isbd_btm_dn8 = 0.0;
        locals.var_isbd_btm_dn9 = 0.0;
        locals.var_isbd_btm_dn10 = 0.0;
        locals.var_isbd_btm_dn13 = 0.0;
        locals.var_isbd_btm_rv = 0.0;

        locals.var_isbd2_btm = 0.0;
        locals.var_isbd2_btm_dn0 = 0.0;
        locals.var_isbd2_btm_dn2 = 0.0;
        locals.var_isbd2_btm_dn4 = 0.0;
        locals.var_isbd2_btm_dn5 = 0.0;
        locals.var_isbd2_btm_dn6 = 0.0;
        locals.var_isbd2_btm_dn7 = 0.0;
        locals.var_isbd2_btm_dn8 = 0.0;
        locals.var_isbd2_btm_dn9 = 0.0;
        locals.var_isbd2_btm_dn10 = 0.0;
        locals.var_isbd2_btm_dn13 = 0.0;
        locals.var_isbd2_btm_rv = 0.0;

        locals.var_isbd_sws = 0.0;
        locals.var_isbd_sws_dn0 = 0.0;
        locals.var_isbd_sws_dn2 = 0.0;
        locals.var_isbd_sws_dn4 = 0.0;
        locals.var_isbd_sws_dn5 = 0.0;
        locals.var_isbd_sws_dn6 = 0.0;
        locals.var_isbd_sws_dn7 = 0.0;
        locals.var_isbd_sws_dn8 = 0.0;
        locals.var_isbd_sws_dn9 = 0.0;
        locals.var_isbd_sws_dn10 = 0.0;
        locals.var_isbd_sws_dn13 = 0.0;
        locals.var_isbd_sws_rv = 0.0;

        locals.var_isbd2_sws = 0.0;
        locals.var_isbd2_sws_dn0 = 0.0;
        locals.var_isbd2_sws_dn2 = 0.0;
        locals.var_isbd2_sws_dn4 = 0.0;
        locals.var_isbd2_sws_dn5 = 0.0;
        locals.var_isbd2_sws_dn6 = 0.0;
        locals.var_isbd2_sws_dn7 = 0.0;
        locals.var_isbd2_sws_dn8 = 0.0;
        locals.var_isbd2_sws_dn9 = 0.0;
        locals.var_isbd2_sws_dn10 = 0.0;
        locals.var_isbd2_sws_dn13 = 0.0;
        locals.var_isbd2_sws_rv = 0.0;

        locals.var_isbd_swg = 0.0;
        locals.var_isbd_swg_dn0 = 0.0;
        locals.var_isbd_swg_dn2 = 0.0;
        locals.var_isbd_swg_dn4 = 0.0;
        locals.var_isbd_swg_dn5 = 0.0;
        locals.var_isbd_swg_dn6 = 0.0;
        locals.var_isbd_swg_dn7 = 0.0;
        locals.var_isbd_swg_dn8 = 0.0;
        locals.var_isbd_swg_dn9 = 0.0;
        locals.var_isbd_swg_dn10 = 0.0;
        locals.var_isbd_swg_dn13 = 0.0;
        locals.var_isbd_swg_rv = 0.0;

        locals.var_isbd2_swg = 0.0;
        locals.var_isbd2_swg_dn0 = 0.0;
        locals.var_isbd2_swg_dn2 = 0.0;
        locals.var_isbd2_swg_dn4 = 0.0;
        locals.var_isbd2_swg_dn5 = 0.0;
        locals.var_isbd2_swg_dn6 = 0.0;
        locals.var_isbd2_swg_dn7 = 0.0;
        locals.var_isbd2_swg_dn8 = 0.0;
        locals.var_isbd2_swg_dn9 = 0.0;
        locals.var_isbd2_swg_dn10 = 0.0;
        locals.var_isbd2_swg_dn13 = 0.0;
        locals.var_isbd2_swg_rv = 0.0;

        locals.var_isbs_btm = 0.0;
        locals.var_isbs_btm_dn0 = 0.0;
        locals.var_isbs_btm_dn2 = 0.0;
        locals.var_isbs_btm_dn4 = 0.0;
        locals.var_isbs_btm_dn5 = 0.0;
        locals.var_isbs_btm_dn6 = 0.0;
        locals.var_isbs_btm_dn7 = 0.0;
        locals.var_isbs_btm_dn8 = 0.0;
        locals.var_isbs_btm_dn9 = 0.0;
        locals.var_isbs_btm_dn10 = 0.0;
        locals.var_isbs_btm_dn13 = 0.0;
        locals.var_isbs_btm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_isbs2_btm = 0.0;
        locals.var_isbs2_btm_dn0 = 0.0;
        locals.var_isbs2_btm_dn2 = 0.0;
        locals.var_isbs2_btm_dn4 = 0.0;
        locals.var_isbs2_btm_dn5 = 0.0;
        locals.var_isbs2_btm_dn6 = 0.0;
        locals.var_isbs2_btm_dn7 = 0.0;
        locals.var_isbs2_btm_dn8 = 0.0;
        locals.var_isbs2_btm_dn9 = 0.0;
        locals.var_isbs2_btm_dn10 = 0.0;
        locals.var_isbs2_btm_dn13 = 0.0;
        locals.var_isbs2_btm_rv = 0.0;

        locals.var_isbs_sws = 0.0;
        locals.var_isbs_sws_dn0 = 0.0;
        locals.var_isbs_sws_dn2 = 0.0;
        locals.var_isbs_sws_dn4 = 0.0;
        locals.var_isbs_sws_dn5 = 0.0;
        locals.var_isbs_sws_dn6 = 0.0;
        locals.var_isbs_sws_dn7 = 0.0;
        locals.var_isbs_sws_dn8 = 0.0;
        locals.var_isbs_sws_dn9 = 0.0;
        locals.var_isbs_sws_dn10 = 0.0;
        locals.var_isbs_sws_dn13 = 0.0;
        locals.var_isbs_sws_rv = 0.0;

        locals.var_isbs2_sws = 0.0;
        locals.var_isbs2_sws_dn0 = 0.0;
        locals.var_isbs2_sws_dn2 = 0.0;
        locals.var_isbs2_sws_dn4 = 0.0;
        locals.var_isbs2_sws_dn5 = 0.0;
        locals.var_isbs2_sws_dn6 = 0.0;
        locals.var_isbs2_sws_dn7 = 0.0;
        locals.var_isbs2_sws_dn8 = 0.0;
        locals.var_isbs2_sws_dn9 = 0.0;
        locals.var_isbs2_sws_dn10 = 0.0;
        locals.var_isbs2_sws_dn13 = 0.0;
        locals.var_isbs2_sws_rv = 0.0;

        locals.var_isbs_swg = 0.0;
        locals.var_isbs_swg_dn0 = 0.0;
        locals.var_isbs_swg_dn2 = 0.0;
        locals.var_isbs_swg_dn4 = 0.0;
        locals.var_isbs_swg_dn5 = 0.0;
        locals.var_isbs_swg_dn6 = 0.0;
        locals.var_isbs_swg_dn7 = 0.0;
        locals.var_isbs_swg_dn8 = 0.0;
        locals.var_isbs_swg_dn9 = 0.0;
        locals.var_isbs_swg_dn10 = 0.0;
        locals.var_isbs_swg_dn13 = 0.0;
        locals.var_isbs_swg_rv = 0.0;

        locals.var_isbs2_swg = 0.0;
        locals.var_isbs2_swg_dn0 = 0.0;
        locals.var_isbs2_swg_dn2 = 0.0;
        locals.var_isbs2_swg_dn4 = 0.0;
        locals.var_isbs2_swg_dn5 = 0.0;
        locals.var_isbs2_swg_dn6 = 0.0;
        locals.var_isbs2_swg_dn7 = 0.0;
        locals.var_isbs2_swg_dn8 = 0.0;
        locals.var_isbs2_swg_dn9 = 0.0;
        locals.var_isbs2_swg_dn10 = 0.0;
        locals.var_isbs2_swg_dn13 = 0.0;
        locals.var_isbs2_swg_rv = 0.0;

        locals.var_qovd_add = 0.0;
        locals.var_qovd_add_dn0 = 0.0;
        locals.var_qovd_add_dn2 = 0.0;
        locals.var_qovd_add_dn4 = 0.0;
        locals.var_qovd_add_dn5 = 0.0;
        locals.var_qovd_add_dn6 = 0.0;
        locals.var_qovd_add_dn7 = 0.0;
        locals.var_qovd_add_dn8 = 0.0;
        locals.var_qovd_add_dn9 = 0.0;
        locals.var_qovd_add_dn10 = 0.0;
        locals.var_qovd_add_dn13 = 0.0;
        locals.var_qovd_add_rv = 0.0;

        locals.var_qovs_add = 0.0;
        locals.var_qovs_add_dn0 = 0.0;
        locals.var_qovs_add_dn2 = 0.0;
        locals.var_qovs_add_dn4 = 0.0;
        locals.var_qovs_add_dn5 = 0.0;
        locals.var_qovs_add_dn6 = 0.0;
        locals.var_qovs_add_dn7 = 0.0;
        locals.var_qovs_add_dn8 = 0.0;
        locals.var_qovs_add_dn9 = 0.0;
        locals.var_qovs_add_dn10 = 0.0;
        locals.var_qovs_add_dn13 = 0.0;
        locals.var_qovs_add_rv = 0.0;

        locals.var_qbdld_add = 0.0;
        locals.var_qbdld_add_dn0 = 0.0;
        locals.var_qbdld_add_dn2 = 0.0;
        locals.var_qbdld_add_dn4 = 0.0;
        locals.var_qbdld_add_dn5 = 0.0;
        locals.var_qbdld_add_dn6 = 0.0;
        locals.var_qbdld_add_dn7 = 0.0;
        locals.var_qbdld_add_dn8 = 0.0;
        locals.var_qbdld_add_dn9 = 0.0;
        locals.var_qbdld_add_dn10 = 0.0;
        locals.var_qbdld_add_dn13 = 0.0;
        locals.var_qbdld_add_rv = 0.0;

        locals.var_qbsld_add = 0.0;
        locals.var_qbsld_add_dn0 = 0.0;
        locals.var_qbsld_add_dn2 = 0.0;
        locals.var_qbsld_add_dn4 = 0.0;
        locals.var_qbsld_add_dn5 = 0.0;
        locals.var_qbsld_add_dn6 = 0.0;
        locals.var_qbsld_add_dn7 = 0.0;
        locals.var_qbsld_add_dn8 = 0.0;
        locals.var_qbsld_add_dn9 = 0.0;
        locals.var_qbsld_add_dn10 = 0.0;
        locals.var_qbsld_add_dn13 = 0.0;
        locals.var_qbsld_add_rv = 0.0;

        locals.var_wjuncld = 0.0;
        locals.var_wjuncld_dn0 = 0.0;
        locals.var_wjuncld_dn2 = 0.0;
        locals.var_wjuncld_dn4 = 0.0;
        locals.var_wjuncld_dn5 = 0.0;
        locals.var_wjuncld_dn6 = 0.0;
        locals.var_wjuncld_dn7 = 0.0;
        locals.var_wjuncld_dn8 = 0.0;
        locals.var_wjuncld_dn9 = 0.0;
        locals.var_wjuncld_dn10 = 0.0;
        locals.var_wjuncld_dn13 = 0.0;
        locals.var_wjuncld_rv = 0.0;

        locals.var_idspt0 = 0.0;
        locals.var_idspt0_dn0 = 0.0;
        locals.var_idspt0_dn2 = 0.0;
        locals.var_idspt0_dn4 = 0.0;
        locals.var_idspt0_dn5 = 0.0;
        locals.var_idspt0_dn6 = 0.0;
        locals.var_idspt0_dn7 = 0.0;
        locals.var_idspt0_dn8 = 0.0;
        locals.var_idspt0_dn9 = 0.0;
        locals.var_idspt0_dn10 = 0.0;
        locals.var_idspt0_dn13 = 0.0;
        locals.var_idspt0_rv = 0.0;

        locals.var_idspt1 = 0.0;
        locals.var_idspt1_dn0 = 0.0;
        locals.var_idspt1_dn2 = 0.0;
        locals.var_idspt1_dn4 = 0.0;
        locals.var_idspt1_dn5 = 0.0;
        locals.var_idspt1_dn6 = 0.0;
        locals.var_idspt1_dn7 = 0.0;
        locals.var_idspt1_dn8 = 0.0;
        locals.var_idspt1_dn9 = 0.0;
        locals.var_idspt1_dn10 = 0.0;
        locals.var_idspt1_dn13 = 0.0;
        locals.var_idspt1_rv = 0.0;

        locals.var_cox0_func = 0.0;
        locals.var_cox0_func_rv = 0.0;

        locals.var_iwnqs0_a = 0.0;
        locals.var_iwnqs0_a_dn0 = 0.0;
        locals.var_iwnqs0_a_dn2 = 0.0;
        locals.var_iwnqs0_a_dn4 = 0.0;
        locals.var_iwnqs0_a_dn5 = 0.0;
        locals.var_iwnqs0_a_dn6 = 0.0;
        locals.var_iwnqs0_a_dn7 = 0.0;
        locals.var_iwnqs0_a_dn8 = 0.0;
        locals.var_iwnqs0_a_dn9 = 0.0;
        locals.var_iwnqs0_a_dn10 = 0.0;
        locals.var_iwnqs0_a_dn13 = 0.0;
        locals.var_iwnqs0_a_dn17 = 0.0;
        locals.var_iwnqs0_a_rv = 0.0;

        locals.var_inqs0_a = 0.0;
        locals.var_inqs0_a_dn0 = 0.0;
        locals.var_inqs0_a_dn2 = 0.0;
        locals.var_inqs0_a_dn4 = 0.0;
        locals.var_inqs0_a_dn5 = 0.0;
        locals.var_inqs0_a_dn6 = 0.0;
        locals.var_inqs0_a_dn7 = 0.0;
        locals.var_inqs0_a_dn8 = 0.0;
        locals.var_inqs0_a_dn9 = 0.0;
        locals.var_inqs0_a_dn10 = 0.0;
        locals.var_inqs0_a_dn13 = 0.0;
        locals.var_inqs0_a_dn15 = 0.0;
        locals.var_inqs0_a_rv = 0.0;

        locals.var_inqs0_k = 0.0;
        locals.var_inqs0_k_dn0 = 0.0;
        locals.var_inqs0_k_dn2 = 0.0;
        locals.var_inqs0_k_dn4 = 0.0;
        locals.var_inqs0_k_dn5 = 0.0;
        locals.var_inqs0_k_dn6 = 0.0;
        locals.var_inqs0_k_dn7 = 0.0;
        locals.var_inqs0_k_dn8 = 0.0;
        locals.var_inqs0_k_dn9 = 0.0;
        locals.var_inqs0_k_dn10 = 0.0;
        locals.var_inqs0_k_dn13 = 0.0;
        locals.var_inqs0_k_dn16 = 0.0;
        locals.var_inqs0_k_rv = 0.0;

        locals.var_isubibpc = 0.0;
        locals.var_isubibpc_dn0 = 0.0;
        locals.var_isubibpc_dn2 = 0.0;
        locals.var_isubibpc_dn4 = 0.0;
        locals.var_isubibpc_dn5 = 0.0;
        locals.var_isubibpc_dn6 = 0.0;
        locals.var_isubibpc_dn7 = 0.0;
        locals.var_isubibpc_dn8 = 0.0;
        locals.var_isubibpc_dn9 = 0.0;
        locals.var_isubibpc_dn10 = 0.0;
        locals.var_isubibpc_dn13 = 0.0;
        locals.var_isubibpc_rv = 0.0;

        locals.var_lover_func = 0.0;
        locals.var_lover_func_dn0 = 0.0;
        locals.var_lover_func_dn2 = 0.0;
        locals.var_lover_func_dn4 = 0.0;
        locals.var_lover_func_dn5 = 0.0;
        locals.var_lover_func_dn6 = 0.0;
        locals.var_lover_func_dn7 = 0.0;
        locals.var_lover_func_dn8 = 0.0;
        locals.var_lover_func_dn9 = 0.0;
        locals.var_lover_func_dn10 = 0.0;
        locals.var_lover_func_dn13 = 0.0;
        locals.var_lover_func_rv = 0.0;

        locals.var_q_nqs_a = 0.0;
        locals.var_q_nqs_a_dn15 = 0.0;
        locals.var_q_nqs_a_rv = 0.0;

        locals.var_q_nqs_k = 0.0;
        locals.var_q_nqs_k_dn16 = 0.0;
        locals.var_q_nqs_k_rv = 0.0;

        locals.var_w_nqs_a = 0.0;
        locals.var_w_nqs_a_dn17 = 0.0;
        locals.var_w_nqs_a_rv = 0.0;

        locals.var_w_res = 0.0;
        locals.var_w_res_dn0 = 0.0;
        locals.var_w_res_dn2 = 0.0;
        locals.var_w_res_dn4 = 0.0;
        locals.var_w_res_dn5 = 0.0;
        locals.var_w_res_dn6 = 0.0;
        locals.var_w_res_dn7 = 0.0;
        locals.var_w_res_dn8 = 0.0;
        locals.var_w_res_dn9 = 0.0;
        locals.var_w_res_dn10 = 0.0;
        locals.var_w_res_dn13 = 0.0;
        locals.var_w_res_rv = 0.0;

        locals.var_wdep_func = 0.0;
        locals.var_wdep_func_dn0 = 0.0;
        locals.var_wdep_func_dn2 = 0.0;
        locals.var_wdep_func_dn4 = 0.0;
        locals.var_wdep_func_dn5 = 0.0;
        locals.var_wdep_func_dn6 = 0.0;
        locals.var_wdep_func_dn7 = 0.0;
        locals.var_wdep_func_dn8 = 0.0;
        locals.var_wdep_func_dn9 = 0.0;
        locals.var_wdep_func_dn10 = 0.0;
        locals.var_wdep_func_dn13 = 0.0;
        locals.var_wdep_func_rv = 0.0;

        locals.var_wk_ii = 0.0;
        locals.var_wk_ii_dn0 = 0.0;
        locals.var_wk_ii_dn2 = 0.0;
        locals.var_wk_ii_dn4 = 0.0;
        locals.var_wk_ii_dn5 = 0.0;
        locals.var_wk_ii_dn6 = 0.0;
        locals.var_wk_ii_dn7 = 0.0;
        locals.var_wk_ii_dn8 = 0.0;
        locals.var_wk_ii_dn9 = 0.0;
        locals.var_wk_ii_dn10 = 0.0;
        locals.var_wk_ii_dn13 = 0.0;
        locals.var_wk_ii_rv = 0.0;

        let (assign5320_e1936,) = {
    if (p.p40 != 0.0) {
        (0.0,)
    } else {
        (p.p17,)
    }
};
        locals.var_uc_corsrd = assign5320_e1936;
        locals.var_uc_corsrd_rv = 0.0;

        locals.var_uc_xpdv = p.p104;
        locals.var_uc_xpdv_rv = 0.0;

        locals.var_uc_xldld = p.p294;
        locals.var_uc_xldld_rv = 0.0;

        locals.var_uc_scp22 = p.p222;
        locals.var_uc_scp22_rv = 0.0;

        locals.var_uc_rdrcx = p.p420;
        locals.var_uc_rdrcx_rv = 0.0;

        locals.var_mfactor = 1.0;
        locals.var_mfactor_rv = 0.0;

        let assign5480_e1979: f64 = if locals.var_uc_scp22 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard8 = assign5480_e1979;
        locals.var_guard8_rv = 0.0;

        let (assign5490_e1983,) = {
    if (locals.var_guard8 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_scp22,)
    }
};
        locals.var_uc_scp22 = assign5490_e1983;
        locals.var_uc_scp22_rv = 0.0;

        let assign5500_e1986: f64 = if locals.var_uc_scp22 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard9 = assign5500_e1986;
        locals.var_guard9_rv = 0.0;

        let (assign5510_e1990,) = {
    if (locals.var_guard9 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_scp22,)
    }
};
        locals.var_uc_scp22 = assign5510_e1990;
        locals.var_uc_scp22_rv = 0.0;

        let assign5530_e1998: f64 = if locals.var_uc_xldld < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard11 = assign5530_e1998;
        locals.var_guard11_rv = 0.0;

        let (assign5540_e2002,) = {
    if (locals.var_guard11 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_xldld,)
    }
};
        locals.var_uc_xldld = assign5540_e2002;
        locals.var_uc_xldld_rv = 0.0;

        let assign5570_e2015: f64 = if locals.var_uc_rdrcx < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign5570_e2015;
        locals.var_guard14_rv = 0.0;

        let (assign5580_e2019,) = {
    if (locals.var_guard14 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_rdrcx,)
    }
};
        locals.var_uc_rdrcx = assign5580_e2019;
        locals.var_uc_rdrcx_rv = 0.0;

        let assign5590_e2022: f64 = if locals.var_uc_rdrcx > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign5590_e2022;
        locals.var_guard15_rv = 0.0;

        let (assign5600_e2026,) = {
    if (locals.var_guard15 != 0.0) {
        (1.0,)
    } else {
        (locals.var_uc_rdrcx,)
    }
};
        locals.var_uc_rdrcx = assign5600_e2026;
        locals.var_uc_rdrcx_rv = 0.0;

        locals.var_uc_ndepm = p.p340;
        locals.var_uc_ndepm_dn0 = 0.0;
        locals.var_uc_ndepm_dn2 = 0.0;
        locals.var_uc_ndepm_dn4 = 0.0;
        locals.var_uc_ndepm_dn5 = 0.0;
        locals.var_uc_ndepm_dn6 = 0.0;
        locals.var_uc_ndepm_dn7 = 0.0;
        locals.var_uc_ndepm_dn8 = 0.0;
        locals.var_uc_ndepm_dn9 = 0.0;
        locals.var_uc_ndepm_dn10 = 0.0;
        locals.var_uc_ndepm_dn13 = 0.0;
        locals.var_uc_ndepm_rv = 0.0;

        locals.var_uc_depthn = p.p343;
        locals.var_uc_depthn_dn0 = 0.0;
        locals.var_uc_depthn_dn2 = 0.0;
        locals.var_uc_depthn_dn4 = 0.0;
        locals.var_uc_depthn_dn5 = 0.0;
        locals.var_uc_depthn_dn6 = 0.0;
        locals.var_uc_depthn_dn7 = 0.0;
        locals.var_uc_depthn_dn8 = 0.0;
        locals.var_uc_depthn_dn9 = 0.0;
        locals.var_uc_depthn_dn10 = 0.0;
        locals.var_uc_depthn_dn13 = 0.0;
        locals.var_uc_depthn_rv = 0.0;

        locals.var_uc_codep = p.p42;
        locals.var_uc_codep_rv = 0.0;

        locals.var_uc_depmueback0 = p.p354;
        locals.var_uc_depmueback0_dn0 = 0.0;
        locals.var_uc_depmueback0_dn2 = 0.0;
        locals.var_uc_depmueback0_dn4 = 0.0;
        locals.var_uc_depmueback0_dn5 = 0.0;
        locals.var_uc_depmueback0_dn6 = 0.0;
        locals.var_uc_depmueback0_dn7 = 0.0;
        locals.var_uc_depmueback0_dn8 = 0.0;
        locals.var_uc_depmueback0_dn9 = 0.0;
        locals.var_uc_depmueback0_dn10 = 0.0;
        locals.var_uc_depmueback0_dn13 = 0.0;
        locals.var_uc_depmueback0_rv = 0.0;

        locals.var_uc_depmueback1 = p.p355;
        locals.var_uc_depmueback1_dn0 = 0.0;
        locals.var_uc_depmueback1_dn2 = 0.0;
        locals.var_uc_depmueback1_dn4 = 0.0;
        locals.var_uc_depmueback1_dn5 = 0.0;
        locals.var_uc_depmueback1_dn6 = 0.0;
        locals.var_uc_depmueback1_dn7 = 0.0;
        locals.var_uc_depmueback1_dn8 = 0.0;
        locals.var_uc_depmueback1_dn9 = 0.0;
        locals.var_uc_depmueback1_dn10 = 0.0;
        locals.var_uc_depmueback1_dn13 = 0.0;
        locals.var_uc_depmueback1_rv = 0.0;

        locals.var_uc_depmue0 = p.p346;
        locals.var_uc_depmue0_dn0 = 0.0;
        locals.var_uc_depmue0_dn2 = 0.0;
        locals.var_uc_depmue0_dn4 = 0.0;
        locals.var_uc_depmue0_dn5 = 0.0;
        locals.var_uc_depmue0_dn6 = 0.0;
        locals.var_uc_depmue0_dn7 = 0.0;
        locals.var_uc_depmue0_dn8 = 0.0;
        locals.var_uc_depmue0_dn9 = 0.0;
        locals.var_uc_depmue0_dn10 = 0.0;
        locals.var_uc_depmue0_dn13 = 0.0;
        locals.var_uc_depmue0_rv = 0.0;

        locals.var_uc_depmue1 = p.p349;
        locals.var_uc_depmue1_dn0 = 0.0;
        locals.var_uc_depmue1_dn2 = 0.0;
        locals.var_uc_depmue1_dn4 = 0.0;
        locals.var_uc_depmue1_dn5 = 0.0;
        locals.var_uc_depmue1_dn6 = 0.0;
        locals.var_uc_depmue1_dn7 = 0.0;
        locals.var_uc_depmue1_dn8 = 0.0;
        locals.var_uc_depmue1_dn9 = 0.0;
        locals.var_uc_depmue1_dn10 = 0.0;
        locals.var_uc_depmue1_dn13 = 0.0;
        locals.var_uc_depmue1_rv = 0.0;

        locals.var_uc_depmue2 = p.p352;
        locals.var_uc_depmue2_dn0 = 0.0;
        locals.var_uc_depmue2_dn2 = 0.0;
        locals.var_uc_depmue2_dn4 = 0.0;
        locals.var_uc_depmue2_dn5 = 0.0;
        locals.var_uc_depmue2_dn6 = 0.0;
        locals.var_uc_depmue2_dn7 = 0.0;
        locals.var_uc_depmue2_dn8 = 0.0;
        locals.var_uc_depmue2_dn9 = 0.0;
        locals.var_uc_depmue2_dn10 = 0.0;
        locals.var_uc_depmue2_dn13 = 0.0;
        locals.var_uc_depmue2_rv = 0.0;

        locals.var_uc_depleak = p.p360;
        locals.var_uc_depleak_dn0 = 0.0;
        locals.var_uc_depleak_dn2 = 0.0;
        locals.var_uc_depleak_dn4 = 0.0;
        locals.var_uc_depleak_dn5 = 0.0;
        locals.var_uc_depleak_dn6 = 0.0;
        locals.var_uc_depleak_dn7 = 0.0;
        locals.var_uc_depleak_dn8 = 0.0;
        locals.var_uc_depleak_dn9 = 0.0;
        locals.var_uc_depleak_dn10 = 0.0;
        locals.var_uc_depleak_dn13 = 0.0;
        locals.var_uc_depleak_rv = 0.0;

        locals.var_uc_depvmax = p.p367;
        locals.var_uc_depvmax_dn0 = 0.0;
        locals.var_uc_depvmax_dn2 = 0.0;
        locals.var_uc_depvmax_dn4 = 0.0;
        locals.var_uc_depvmax_dn5 = 0.0;
        locals.var_uc_depvmax_dn6 = 0.0;
        locals.var_uc_depvmax_dn7 = 0.0;
        locals.var_uc_depvmax_dn8 = 0.0;
        locals.var_uc_depvmax_dn9 = 0.0;
        locals.var_uc_depvmax_dn10 = 0.0;
        locals.var_uc_depvmax_dn13 = 0.0;
        locals.var_uc_depvmax_rv = 0.0;

        locals.var_uc_depwlp = p.p364;
        locals.var_uc_depwlp_dn0 = 0.0;
        locals.var_uc_depwlp_dn2 = 0.0;
        locals.var_uc_depwlp_dn4 = 0.0;
        locals.var_uc_depwlp_dn5 = 0.0;
        locals.var_uc_depwlp_dn6 = 0.0;
        locals.var_uc_depwlp_dn7 = 0.0;
        locals.var_uc_depwlp_dn8 = 0.0;
        locals.var_uc_depwlp_dn9 = 0.0;
        locals.var_uc_depwlp_dn10 = 0.0;
        locals.var_uc_depwlp_dn13 = 0.0;
        locals.var_uc_depwlp_rv = 0.0;

        locals.var_uc_depmueph1 = p.p377;
        locals.var_uc_depmueph1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_uc_depvdsef1 = p.p370;
        locals.var_uc_depvdsef1_dn0 = 0.0;
        locals.var_uc_depvdsef1_dn2 = 0.0;
        locals.var_uc_depvdsef1_dn4 = 0.0;
        locals.var_uc_depvdsef1_dn5 = 0.0;
        locals.var_uc_depvdsef1_dn6 = 0.0;
        locals.var_uc_depvdsef1_dn7 = 0.0;
        locals.var_uc_depvdsef1_dn8 = 0.0;
        locals.var_uc_depvdsef1_dn9 = 0.0;
        locals.var_uc_depvdsef1_dn10 = 0.0;
        locals.var_uc_depvdsef1_dn13 = 0.0;
        locals.var_uc_depvdsef1_rv = 0.0;

        locals.var_uc_depvdsef2 = p.p371;
        locals.var_uc_depvdsef2_dn0 = 0.0;
        locals.var_uc_depvdsef2_dn2 = 0.0;
        locals.var_uc_depvdsef2_dn4 = 0.0;
        locals.var_uc_depvdsef2_dn5 = 0.0;
        locals.var_uc_depvdsef2_dn6 = 0.0;
        locals.var_uc_depvdsef2_dn7 = 0.0;
        locals.var_uc_depvdsef2_dn8 = 0.0;
        locals.var_uc_depvdsef2_dn9 = 0.0;
        locals.var_uc_depvdsef2_dn10 = 0.0;
        locals.var_uc_depvdsef2_dn13 = 0.0;
        locals.var_uc_depvdsef2_rv = 0.0;

        let assign6690_e2699: f64 = if ((locals.var_uc_codep < 3.0) && (locals.var_uc_codep > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard110 = assign6690_e2699;
        locals.var_guard110_rv = 0.0;

        let assign6720_e2712: f64 = if locals.var_uc_ndepm < 5000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign6720_e2712;
        locals.var_guard113_rv = 0.0;

        let (assign6730_e2718, assign6730_e2718_d_n0, assign6730_e2718_d_n2, assign6730_e2718_d_n4, assign6730_e2718_d_n5, assign6730_e2718_d_n6, assign6730_e2718_d_n7, assign6730_e2718_d_n8, assign6730_e2718_d_n9, assign6730_e2718_d_n10, assign6730_e2718_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard113 != 0.0)) {
        (5000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn13,)
    }
};
        locals.var_uc_ndepm = assign6730_e2718;
        locals.var_uc_ndepm_dn0 = assign6730_e2718_d_n0;
        locals.var_uc_ndepm_dn2 = assign6730_e2718_d_n2;
        locals.var_uc_ndepm_dn4 = assign6730_e2718_d_n4;
        locals.var_uc_ndepm_dn5 = assign6730_e2718_d_n5;
        locals.var_uc_ndepm_dn6 = assign6730_e2718_d_n6;
        locals.var_uc_ndepm_dn7 = assign6730_e2718_d_n7;
        locals.var_uc_ndepm_dn8 = assign6730_e2718_d_n8;
        locals.var_uc_ndepm_dn9 = assign6730_e2718_d_n9;
        locals.var_uc_ndepm_dn10 = assign6730_e2718_d_n10;
        locals.var_uc_ndepm_dn13 = assign6730_e2718_d_n13;
        locals.var_uc_ndepm_rv = 0.0;

        let assign6740_e2721: f64 = if locals.var_uc_ndepm > 1e18 { 1.0 } else { 0.0 };
        locals.var_guard114 = assign6740_e2721;
        locals.var_guard114_rv = 0.0;

        let (assign6750_e2727, assign6750_e2727_d_n0, assign6750_e2727_d_n2, assign6750_e2727_d_n4, assign6750_e2727_d_n5, assign6750_e2727_d_n6, assign6750_e2727_d_n7, assign6750_e2727_d_n8, assign6750_e2727_d_n9, assign6750_e2727_d_n10, assign6750_e2727_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard114 != 0.0)) {
        (1e18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn13,)
    }
};
        locals.var_uc_ndepm = assign6750_e2727;
        locals.var_uc_ndepm_dn0 = assign6750_e2727_d_n0;
        locals.var_uc_ndepm_dn2 = assign6750_e2727_d_n2;
        locals.var_uc_ndepm_dn4 = assign6750_e2727_d_n4;
        locals.var_uc_ndepm_dn5 = assign6750_e2727_d_n5;
        locals.var_uc_ndepm_dn6 = assign6750_e2727_d_n6;
        locals.var_uc_ndepm_dn7 = assign6750_e2727_d_n7;
        locals.var_uc_ndepm_dn8 = assign6750_e2727_d_n8;
        locals.var_uc_ndepm_dn9 = assign6750_e2727_d_n9;
        locals.var_uc_ndepm_dn10 = assign6750_e2727_d_n10;
        locals.var_uc_ndepm_dn13 = assign6750_e2727_d_n13;
        locals.var_uc_ndepm_rv = 0.0;

        let assign6780_e2740: f64 = if locals.var_uc_depthn < 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign6780_e2740;
        locals.var_guard117_rv = 0.0;

        let (assign6790_e2746, assign6790_e2746_d_n0, assign6790_e2746_d_n2, assign6790_e2746_d_n4, assign6790_e2746_d_n5, assign6790_e2746_d_n6, assign6790_e2746_d_n7, assign6790_e2746_d_n8, assign6790_e2746_d_n9, assign6790_e2746_d_n10, assign6790_e2746_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard117 != 0.0)) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn13,)
    }
};
        locals.var_uc_depthn = assign6790_e2746;
        locals.var_uc_depthn_dn0 = assign6790_e2746_d_n0;
        locals.var_uc_depthn_dn2 = assign6790_e2746_d_n2;
        locals.var_uc_depthn_dn4 = assign6790_e2746_d_n4;
        locals.var_uc_depthn_dn5 = assign6790_e2746_d_n5;
        locals.var_uc_depthn_dn6 = assign6790_e2746_d_n6;
        locals.var_uc_depthn_dn7 = assign6790_e2746_d_n7;
        locals.var_uc_depthn_dn8 = assign6790_e2746_d_n8;
        locals.var_uc_depthn_dn9 = assign6790_e2746_d_n9;
        locals.var_uc_depthn_dn10 = assign6790_e2746_d_n10;
        locals.var_uc_depthn_dn13 = assign6790_e2746_d_n13;
        locals.var_uc_depthn_rv = 0.0;

        let assign6800_e2749: f64 = if locals.var_uc_depthn > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign6800_e2749;
        locals.var_guard118_rv = 0.0;

        let (assign6810_e2755, assign6810_e2755_d_n0, assign6810_e2755_d_n2, assign6810_e2755_d_n4, assign6810_e2755_d_n5, assign6810_e2755_d_n6, assign6810_e2755_d_n7, assign6810_e2755_d_n8, assign6810_e2755_d_n9, assign6810_e2755_d_n10, assign6810_e2755_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard118 != 0.0)) {
        (1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn13,)
    }
};
        locals.var_uc_depthn = assign6810_e2755;
        locals.var_uc_depthn_dn0 = assign6810_e2755_d_n0;
        locals.var_uc_depthn_dn2 = assign6810_e2755_d_n2;
        locals.var_uc_depthn_dn4 = assign6810_e2755_d_n4;
        locals.var_uc_depthn_dn5 = assign6810_e2755_d_n5;
        locals.var_uc_depthn_dn6 = assign6810_e2755_d_n6;
        locals.var_uc_depthn_dn7 = assign6810_e2755_d_n7;
        locals.var_uc_depthn_dn8 = assign6810_e2755_d_n8;
        locals.var_uc_depthn_dn9 = assign6810_e2755_d_n9;
        locals.var_uc_depthn_dn10 = assign6810_e2755_d_n10;
        locals.var_uc_depthn_dn13 = assign6810_e2755_d_n13;
        locals.var_uc_depthn_rv = 0.0;

        let assign6840_e2768: f64 = if locals.var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard121 = assign6840_e2768;
        locals.var_guard121_rv = 0.0;

        let (assign6850_e2774, assign6850_e2774_d_n0, assign6850_e2774_d_n2, assign6850_e2774_d_n4, assign6850_e2774_d_n5, assign6850_e2774_d_n6, assign6850_e2774_d_n7, assign6850_e2774_d_n8, assign6850_e2774_d_n9, assign6850_e2774_d_n10, assign6850_e2774_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard121 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign6850_e2774;
        locals.var_uc_depmue0_dn0 = assign6850_e2774_d_n0;
        locals.var_uc_depmue0_dn2 = assign6850_e2774_d_n2;
        locals.var_uc_depmue0_dn4 = assign6850_e2774_d_n4;
        locals.var_uc_depmue0_dn5 = assign6850_e2774_d_n5;
        locals.var_uc_depmue0_dn6 = assign6850_e2774_d_n6;
        locals.var_uc_depmue0_dn7 = assign6850_e2774_d_n7;
        locals.var_uc_depmue0_dn8 = assign6850_e2774_d_n8;
        locals.var_uc_depmue0_dn9 = assign6850_e2774_d_n9;
        locals.var_uc_depmue0_dn10 = assign6850_e2774_d_n10;
        locals.var_uc_depmue0_dn13 = assign6850_e2774_d_n13;
        locals.var_uc_depmue0_rv = 0.0;

        let assign6860_e2777: f64 = if locals.var_uc_depmue0 > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard122 = assign6860_e2777;
        locals.var_guard122_rv = 0.0;

        let (assign6870_e2783, assign6870_e2783_d_n0, assign6870_e2783_d_n2, assign6870_e2783_d_n4, assign6870_e2783_d_n5, assign6870_e2783_d_n6, assign6870_e2783_d_n7, assign6870_e2783_d_n8, assign6870_e2783_d_n9, assign6870_e2783_d_n10, assign6870_e2783_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard122 != 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign6870_e2783;
        locals.var_uc_depmue0_dn0 = assign6870_e2783_d_n0;
        locals.var_uc_depmue0_dn2 = assign6870_e2783_d_n2;
        locals.var_uc_depmue0_dn4 = assign6870_e2783_d_n4;
        locals.var_uc_depmue0_dn5 = assign6870_e2783_d_n5;
        locals.var_uc_depmue0_dn6 = assign6870_e2783_d_n6;
        locals.var_uc_depmue0_dn7 = assign6870_e2783_d_n7;
        locals.var_uc_depmue0_dn8 = assign6870_e2783_d_n8;
        locals.var_uc_depmue0_dn9 = assign6870_e2783_d_n9;
        locals.var_uc_depmue0_dn10 = assign6870_e2783_d_n10;
        locals.var_uc_depmue0_dn13 = assign6870_e2783_d_n13;
        locals.var_uc_depmue0_rv = 0.0;

        let assign6900_e2796: f64 = if locals.var_uc_depmueback0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard125 = assign6900_e2796;
        locals.var_guard125_rv = 0.0;

        let (assign6910_e2802, assign6910_e2802_d_n0, assign6910_e2802_d_n2, assign6910_e2802_d_n4, assign6910_e2802_d_n5, assign6910_e2802_d_n6, assign6910_e2802_d_n7, assign6910_e2802_d_n8, assign6910_e2802_d_n9, assign6910_e2802_d_n10, assign6910_e2802_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard125 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn13,)
    }
};
        locals.var_uc_depmueback0 = assign6910_e2802;
        locals.var_uc_depmueback0_dn0 = assign6910_e2802_d_n0;
        locals.var_uc_depmueback0_dn2 = assign6910_e2802_d_n2;
        locals.var_uc_depmueback0_dn4 = assign6910_e2802_d_n4;
        locals.var_uc_depmueback0_dn5 = assign6910_e2802_d_n5;
        locals.var_uc_depmueback0_dn6 = assign6910_e2802_d_n6;
        locals.var_uc_depmueback0_dn7 = assign6910_e2802_d_n7;
        locals.var_uc_depmueback0_dn8 = assign6910_e2802_d_n8;
        locals.var_uc_depmueback0_dn9 = assign6910_e2802_d_n9;
        locals.var_uc_depmueback0_dn10 = assign6910_e2802_d_n10;
        locals.var_uc_depmueback0_dn13 = assign6910_e2802_d_n13;
        locals.var_uc_depmueback0_rv = 0.0;

        let assign6920_e2805: f64 = if locals.var_uc_depmueback0 > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign6920_e2805;
        locals.var_guard126_rv = 0.0;

        let (assign6930_e2811, assign6930_e2811_d_n0, assign6930_e2811_d_n2, assign6930_e2811_d_n4, assign6930_e2811_d_n5, assign6930_e2811_d_n6, assign6930_e2811_d_n7, assign6930_e2811_d_n8, assign6930_e2811_d_n9, assign6930_e2811_d_n10, assign6930_e2811_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard126 != 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn13,)
    }
};
        locals.var_uc_depmueback0 = assign6930_e2811;
        locals.var_uc_depmueback0_dn0 = assign6930_e2811_d_n0;
        locals.var_uc_depmueback0_dn2 = assign6930_e2811_d_n2;
        locals.var_uc_depmueback0_dn4 = assign6930_e2811_d_n4;
        locals.var_uc_depmueback0_dn5 = assign6930_e2811_d_n5;
        locals.var_uc_depmueback0_dn6 = assign6930_e2811_d_n6;
        locals.var_uc_depmueback0_dn7 = assign6930_e2811_d_n7;
        locals.var_uc_depmueback0_dn8 = assign6930_e2811_d_n8;
        locals.var_uc_depmueback0_dn9 = assign6930_e2811_d_n9;
        locals.var_uc_depmueback0_dn10 = assign6930_e2811_d_n10;
        locals.var_uc_depmueback0_dn13 = assign6930_e2811_d_n13;
        locals.var_uc_depmueback0_rv = 0.0;

        let assign6960_e2824: f64 = if locals.var_uc_depmueph1 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard129 = assign6960_e2824;
        locals.var_guard129_rv = 0.0;

        let (assign6970_e2830,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard129 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign6970_e2830;
        locals.var_uc_depmueph1_rv = 0.0;

        let assign6980_e2833: f64 = if locals.var_uc_depmueph1 > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard130 = assign6980_e2833;
        locals.var_guard130_rv = 0.0;

        let (assign6990_e2839,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard130 != 0.0)) {
        (100000.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign6990_e2839;
        locals.var_uc_depmueph1_rv = 0.0;

        let assign7020_e2852: f64 = if locals.var_uc_depvdsef2 < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard133 = assign7020_e2852;
        locals.var_guard133_rv = 0.0;

        let (assign7030_e2858, assign7030_e2858_d_n0, assign7030_e2858_d_n2, assign7030_e2858_d_n4, assign7030_e2858_d_n5, assign7030_e2858_d_n6, assign7030_e2858_d_n7, assign7030_e2858_d_n8, assign7030_e2858_d_n9, assign7030_e2858_d_n10, assign7030_e2858_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard133 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn13,)
    }
};
        locals.var_uc_depvdsef2 = assign7030_e2858;
        locals.var_uc_depvdsef2_dn0 = assign7030_e2858_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign7030_e2858_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign7030_e2858_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign7030_e2858_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign7030_e2858_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign7030_e2858_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign7030_e2858_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign7030_e2858_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign7030_e2858_d_n10;
        locals.var_uc_depvdsef2_dn13 = assign7030_e2858_d_n13;
        locals.var_uc_depvdsef2_rv = 0.0;

        let assign7040_e2861: f64 = if locals.var_uc_depvdsef2 > 4.0 { 1.0 } else { 0.0 };
        locals.var_guard134 = assign7040_e2861;
        locals.var_guard134_rv = 0.0;

        let (assign7050_e2867, assign7050_e2867_d_n0, assign7050_e2867_d_n2, assign7050_e2867_d_n4, assign7050_e2867_d_n5, assign7050_e2867_d_n6, assign7050_e2867_d_n7, assign7050_e2867_d_n8, assign7050_e2867_d_n9, assign7050_e2867_d_n10, assign7050_e2867_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard134 != 0.0)) {
        (4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn13,)
    }
};
        locals.var_uc_depvdsef2 = assign7050_e2867;
        locals.var_uc_depvdsef2_dn0 = assign7050_e2867_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign7050_e2867_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign7050_e2867_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign7050_e2867_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign7050_e2867_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign7050_e2867_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign7050_e2867_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign7050_e2867_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign7050_e2867_d_n10;
        locals.var_uc_depvdsef2_dn13 = assign7050_e2867_d_n13;
        locals.var_uc_depvdsef2_rv = 0.0;

        let assign7080_e2880: f64 = if locals.var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard137 = assign7080_e2880;
        locals.var_guard137_rv = 0.0;

        let (assign7090_e2886, assign7090_e2886_d_n0, assign7090_e2886_d_n2, assign7090_e2886_d_n4, assign7090_e2886_d_n5, assign7090_e2886_d_n6, assign7090_e2886_d_n7, assign7090_e2886_d_n8, assign7090_e2886_d_n9, assign7090_e2886_d_n10, assign7090_e2886_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard137 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn13,)
    }
};
        locals.var_uc_depleak = assign7090_e2886;
        locals.var_uc_depleak_dn0 = assign7090_e2886_d_n0;
        locals.var_uc_depleak_dn2 = assign7090_e2886_d_n2;
        locals.var_uc_depleak_dn4 = assign7090_e2886_d_n4;
        locals.var_uc_depleak_dn5 = assign7090_e2886_d_n5;
        locals.var_uc_depleak_dn6 = assign7090_e2886_d_n6;
        locals.var_uc_depleak_dn7 = assign7090_e2886_d_n7;
        locals.var_uc_depleak_dn8 = assign7090_e2886_d_n8;
        locals.var_uc_depleak_dn9 = assign7090_e2886_d_n9;
        locals.var_uc_depleak_dn10 = assign7090_e2886_d_n10;
        locals.var_uc_depleak_dn13 = assign7090_e2886_d_n13;
        locals.var_uc_depleak_rv = 0.0;

        let assign7100_e2889: f64 = if locals.var_uc_depleak > 5.0 { 1.0 } else { 0.0 };
        locals.var_guard138 = assign7100_e2889;
        locals.var_guard138_rv = 0.0;

        let (assign7110_e2895, assign7110_e2895_d_n0, assign7110_e2895_d_n2, assign7110_e2895_d_n4, assign7110_e2895_d_n5, assign7110_e2895_d_n6, assign7110_e2895_d_n7, assign7110_e2895_d_n8, assign7110_e2895_d_n9, assign7110_e2895_d_n10, assign7110_e2895_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard138 != 0.0)) {
        (5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn13,)
    }
};
        locals.var_uc_depleak = assign7110_e2895;
        locals.var_uc_depleak_dn0 = assign7110_e2895_d_n0;
        locals.var_uc_depleak_dn2 = assign7110_e2895_d_n2;
        locals.var_uc_depleak_dn4 = assign7110_e2895_d_n4;
        locals.var_uc_depleak_dn5 = assign7110_e2895_d_n5;
        locals.var_uc_depleak_dn6 = assign7110_e2895_d_n6;
        locals.var_uc_depleak_dn7 = assign7110_e2895_d_n7;
        locals.var_uc_depleak_dn8 = assign7110_e2895_d_n8;
        locals.var_uc_depleak_dn9 = assign7110_e2895_d_n9;
        locals.var_uc_depleak_dn10 = assign7110_e2895_d_n10;
        locals.var_uc_depleak_dn13 = assign7110_e2895_d_n13;
        locals.var_uc_depleak_rv = 0.0;

        let assign7120_e2898: f64 = if locals.var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard139 = assign7120_e2898;
        locals.var_guard139_rv = 0.0;

        let assign7150_e2911: f64 = if locals.var_uc_ndepm < 5000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard142 = assign7150_e2911;
        locals.var_guard142_rv = 0.0;

        let (assign7160_e2920, assign7160_e2920_d_n0, assign7160_e2920_d_n2, assign7160_e2920_d_n4, assign7160_e2920_d_n5, assign7160_e2920_d_n6, assign7160_e2920_d_n7, assign7160_e2920_d_n8, assign7160_e2920_d_n9, assign7160_e2920_d_n10, assign7160_e2920_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard142 != 0.0)) {
        (5000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn13,)
    }
};
        locals.var_uc_ndepm = assign7160_e2920;
        locals.var_uc_ndepm_dn0 = assign7160_e2920_d_n0;
        locals.var_uc_ndepm_dn2 = assign7160_e2920_d_n2;
        locals.var_uc_ndepm_dn4 = assign7160_e2920_d_n4;
        locals.var_uc_ndepm_dn5 = assign7160_e2920_d_n5;
        locals.var_uc_ndepm_dn6 = assign7160_e2920_d_n6;
        locals.var_uc_ndepm_dn7 = assign7160_e2920_d_n7;
        locals.var_uc_ndepm_dn8 = assign7160_e2920_d_n8;
        locals.var_uc_ndepm_dn9 = assign7160_e2920_d_n9;
        locals.var_uc_ndepm_dn10 = assign7160_e2920_d_n10;
        locals.var_uc_ndepm_dn13 = assign7160_e2920_d_n13;
        locals.var_uc_ndepm_rv = 0.0;

        let assign7170_e2923: f64 = if locals.var_uc_ndepm > 1e18 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign7170_e2923;
        locals.var_guard143_rv = 0.0;

        let (assign7180_e2932, assign7180_e2932_d_n0, assign7180_e2932_d_n2, assign7180_e2932_d_n4, assign7180_e2932_d_n5, assign7180_e2932_d_n6, assign7180_e2932_d_n7, assign7180_e2932_d_n8, assign7180_e2932_d_n9, assign7180_e2932_d_n10, assign7180_e2932_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard143 != 0.0)) {
        (1e18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn13,)
    }
};
        locals.var_uc_ndepm = assign7180_e2932;
        locals.var_uc_ndepm_dn0 = assign7180_e2932_d_n0;
        locals.var_uc_ndepm_dn2 = assign7180_e2932_d_n2;
        locals.var_uc_ndepm_dn4 = assign7180_e2932_d_n4;
        locals.var_uc_ndepm_dn5 = assign7180_e2932_d_n5;
        locals.var_uc_ndepm_dn6 = assign7180_e2932_d_n6;
        locals.var_uc_ndepm_dn7 = assign7180_e2932_d_n7;
        locals.var_uc_ndepm_dn8 = assign7180_e2932_d_n8;
        locals.var_uc_ndepm_dn9 = assign7180_e2932_d_n9;
        locals.var_uc_ndepm_dn10 = assign7180_e2932_d_n10;
        locals.var_uc_ndepm_dn13 = assign7180_e2932_d_n13;
        locals.var_uc_ndepm_rv = 0.0;

        let assign7210_e2945: f64 = if locals.var_uc_depthn < 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard146 = assign7210_e2945;
        locals.var_guard146_rv = 0.0;

        let (assign7220_e2954, assign7220_e2954_d_n0, assign7220_e2954_d_n2, assign7220_e2954_d_n4, assign7220_e2954_d_n5, assign7220_e2954_d_n6, assign7220_e2954_d_n7, assign7220_e2954_d_n8, assign7220_e2954_d_n9, assign7220_e2954_d_n10, assign7220_e2954_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard146 != 0.0)) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn13,)
    }
};
        locals.var_uc_depthn = assign7220_e2954;
        locals.var_uc_depthn_dn0 = assign7220_e2954_d_n0;
        locals.var_uc_depthn_dn2 = assign7220_e2954_d_n2;
        locals.var_uc_depthn_dn4 = assign7220_e2954_d_n4;
        locals.var_uc_depthn_dn5 = assign7220_e2954_d_n5;
        locals.var_uc_depthn_dn6 = assign7220_e2954_d_n6;
        locals.var_uc_depthn_dn7 = assign7220_e2954_d_n7;
        locals.var_uc_depthn_dn8 = assign7220_e2954_d_n8;
        locals.var_uc_depthn_dn9 = assign7220_e2954_d_n9;
        locals.var_uc_depthn_dn10 = assign7220_e2954_d_n10;
        locals.var_uc_depthn_dn13 = assign7220_e2954_d_n13;
        locals.var_uc_depthn_rv = 0.0;

        let assign7230_e2957: f64 = if locals.var_uc_depthn > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign7230_e2957;
        locals.var_guard147_rv = 0.0;

        let (assign7240_e2966, assign7240_e2966_d_n0, assign7240_e2966_d_n2, assign7240_e2966_d_n4, assign7240_e2966_d_n5, assign7240_e2966_d_n6, assign7240_e2966_d_n7, assign7240_e2966_d_n8, assign7240_e2966_d_n9, assign7240_e2966_d_n10, assign7240_e2966_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard147 != 0.0)) {
        (1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn13,)
    }
};
        locals.var_uc_depthn = assign7240_e2966;
        locals.var_uc_depthn_dn0 = assign7240_e2966_d_n0;
        locals.var_uc_depthn_dn2 = assign7240_e2966_d_n2;
        locals.var_uc_depthn_dn4 = assign7240_e2966_d_n4;
        locals.var_uc_depthn_dn5 = assign7240_e2966_d_n5;
        locals.var_uc_depthn_dn6 = assign7240_e2966_d_n6;
        locals.var_uc_depthn_dn7 = assign7240_e2966_d_n7;
        locals.var_uc_depthn_dn8 = assign7240_e2966_d_n8;
        locals.var_uc_depthn_dn9 = assign7240_e2966_d_n9;
        locals.var_uc_depthn_dn10 = assign7240_e2966_d_n10;
        locals.var_uc_depthn_dn13 = assign7240_e2966_d_n13;
        locals.var_uc_depthn_rv = 0.0;

        let assign7270_e2979: f64 = if locals.var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign7270_e2979;
        locals.var_guard150_rv = 0.0;

        let (assign7280_e2988, assign7280_e2988_d_n0, assign7280_e2988_d_n2, assign7280_e2988_d_n4, assign7280_e2988_d_n5, assign7280_e2988_d_n6, assign7280_e2988_d_n7, assign7280_e2988_d_n8, assign7280_e2988_d_n9, assign7280_e2988_d_n10, assign7280_e2988_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard150 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign7280_e2988;
        locals.var_uc_depmue0_dn0 = assign7280_e2988_d_n0;
        locals.var_uc_depmue0_dn2 = assign7280_e2988_d_n2;
        locals.var_uc_depmue0_dn4 = assign7280_e2988_d_n4;
        locals.var_uc_depmue0_dn5 = assign7280_e2988_d_n5;
        locals.var_uc_depmue0_dn6 = assign7280_e2988_d_n6;
        locals.var_uc_depmue0_dn7 = assign7280_e2988_d_n7;
        locals.var_uc_depmue0_dn8 = assign7280_e2988_d_n8;
        locals.var_uc_depmue0_dn9 = assign7280_e2988_d_n9;
        locals.var_uc_depmue0_dn10 = assign7280_e2988_d_n10;
        locals.var_uc_depmue0_dn13 = assign7280_e2988_d_n13;
        locals.var_uc_depmue0_rv = 0.0;

        let assign7290_e2991: f64 = if locals.var_uc_depmue0 > 10000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard151 = assign7290_e2991;
        locals.var_guard151_rv = 0.0;

        let (assign7300_e3000, assign7300_e3000_d_n0, assign7300_e3000_d_n2, assign7300_e3000_d_n4, assign7300_e3000_d_n5, assign7300_e3000_d_n6, assign7300_e3000_d_n7, assign7300_e3000_d_n8, assign7300_e3000_d_n9, assign7300_e3000_d_n10, assign7300_e3000_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard151 != 0.0)) {
        (10000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign7300_e3000;
        locals.var_uc_depmue0_dn0 = assign7300_e3000_d_n0;
        locals.var_uc_depmue0_dn2 = assign7300_e3000_d_n2;
        locals.var_uc_depmue0_dn4 = assign7300_e3000_d_n4;
        locals.var_uc_depmue0_dn5 = assign7300_e3000_d_n5;
        locals.var_uc_depmue0_dn6 = assign7300_e3000_d_n6;
        locals.var_uc_depmue0_dn7 = assign7300_e3000_d_n7;
        locals.var_uc_depmue0_dn8 = assign7300_e3000_d_n8;
        locals.var_uc_depmue0_dn9 = assign7300_e3000_d_n9;
        locals.var_uc_depmue0_dn10 = assign7300_e3000_d_n10;
        locals.var_uc_depmue0_dn13 = assign7300_e3000_d_n13;
        locals.var_uc_depmue0_rv = 0.0;

        let assign7330_e3013: f64 = if locals.var_uc_depmueph1 < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign7330_e3013;
        locals.var_guard154_rv = 0.0;

        let (assign7340_e3022,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard154 != 0.0)) {
        (100.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign7340_e3022;
        locals.var_uc_depmueph1_rv = 0.0;

        let assign7350_e3025: f64 = if locals.var_uc_depmueph1 > 2000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign7350_e3025;
        locals.var_guard155_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7360_e3034,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard155 != 0.0)) {
        (2000000000.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign7360_e3034;
        locals.var_uc_depmueph1_rv = 0.0;

        let assign7390_e3047: f64 = if locals.var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign7390_e3047;
        locals.var_guard158_rv = 0.0;

        let (assign7400_e3056, assign7400_e3056_d_n0, assign7400_e3056_d_n2, assign7400_e3056_d_n4, assign7400_e3056_d_n5, assign7400_e3056_d_n6, assign7400_e3056_d_n7, assign7400_e3056_d_n8, assign7400_e3056_d_n9, assign7400_e3056_d_n10, assign7400_e3056_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard158 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn13,)
    }
};
        locals.var_uc_depleak = assign7400_e3056;
        locals.var_uc_depleak_dn0 = assign7400_e3056_d_n0;
        locals.var_uc_depleak_dn2 = assign7400_e3056_d_n2;
        locals.var_uc_depleak_dn4 = assign7400_e3056_d_n4;
        locals.var_uc_depleak_dn5 = assign7400_e3056_d_n5;
        locals.var_uc_depleak_dn6 = assign7400_e3056_d_n6;
        locals.var_uc_depleak_dn7 = assign7400_e3056_d_n7;
        locals.var_uc_depleak_dn8 = assign7400_e3056_d_n8;
        locals.var_uc_depleak_dn9 = assign7400_e3056_d_n9;
        locals.var_uc_depleak_dn10 = assign7400_e3056_d_n10;
        locals.var_uc_depleak_dn13 = assign7400_e3056_d_n13;
        locals.var_uc_depleak_rv = 0.0;

        let assign7410_e3059: f64 = if locals.var_uc_depleak > 5.0 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign7410_e3059;
        locals.var_guard159_rv = 0.0;

        let (assign7420_e3068, assign7420_e3068_d_n0, assign7420_e3068_d_n2, assign7420_e3068_d_n4, assign7420_e3068_d_n5, assign7420_e3068_d_n6, assign7420_e3068_d_n7, assign7420_e3068_d_n8, assign7420_e3068_d_n9, assign7420_e3068_d_n10, assign7420_e3068_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard159 != 0.0)) {
        (5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn13,)
    }
};
        locals.var_uc_depleak = assign7420_e3068;
        locals.var_uc_depleak_dn0 = assign7420_e3068_d_n0;
        locals.var_uc_depleak_dn2 = assign7420_e3068_d_n2;
        locals.var_uc_depleak_dn4 = assign7420_e3068_d_n4;
        locals.var_uc_depleak_dn5 = assign7420_e3068_d_n5;
        locals.var_uc_depleak_dn6 = assign7420_e3068_d_n6;
        locals.var_uc_depleak_dn7 = assign7420_e3068_d_n7;
        locals.var_uc_depleak_dn8 = assign7420_e3068_d_n8;
        locals.var_uc_depleak_dn9 = assign7420_e3068_d_n9;
        locals.var_uc_depleak_dn10 = assign7420_e3068_d_n10;
        locals.var_uc_depleak_dn13 = assign7420_e3068_d_n13;
        locals.var_uc_depleak_rv = 0.0;

        locals.var_uc_toxb = p.p96;
        locals.var_uc_toxb_rv = 0.0;

        let assign7520_e3106: f64 = if locals.var_uc_toxb < p.p95 { 1.0 } else { 0.0 };
        locals.var_guard168 = assign7520_e3106;
        locals.var_guard168_rv = 0.0;

        let (assign7530_e3110,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p95,)
    } else {
        (locals.var_uc_toxb,)
    }
};
        locals.var_uc_toxb = assign7530_e3110;
        locals.var_uc_toxb_rv = 0.0;

        let assign7540_e3113: f64 = if locals.var_uc_toxb > 5e-7 { 1.0 } else { 0.0 };
        locals.var_guard169 = assign7540_e3113;
        locals.var_guard169_rv = 0.0;

        let (assign7550_e3117,) = {
    if (locals.var_guard169 != 0.0) {
        (5e-7,)
    } else {
        (locals.var_uc_toxb,)
    }
};
        locals.var_uc_toxb = assign7550_e3117;
        locals.var_uc_toxb_rv = 0.0;

        let assign7560_e3121: f64 = (100.0_f64).powf(p.p122);
        let assign7560_e3122: f64 = (p.p120 / assign7560_e3121);
        locals.var_mks_ll = assign7560_e3122;
        locals.var_mks_ll_rv = 0.0;

        let assign7570_e3126: f64 = (100.0_f64).powf(p.p129);
        let assign7570_e3127: f64 = (p.p123 / assign7570_e3126);
        locals.var_mks_wl = assign7570_e3127;
        locals.var_mks_wl_rv = 0.0;

        let assign7580_e3131: f64 = (100.0_f64).powf(p.p199);
        let assign7580_e3132: f64 = (p.p198 / assign7580_e3131);
        locals.var_mks_svgsl = assign7580_e3132;
        locals.var_mks_svgsl_rv = 0.0;

        let assign7590_e3136: f64 = (100.0_f64).powf(p.p201);
        let assign7590_e3137: f64 = (p.p200 / assign7590_e3136);
        locals.var_mks_svgsw = assign7590_e3137;
        locals.var_mks_svgsw_rv = 0.0;

        let assign7600_e3141: f64 = (100.0_f64).powf(p.p184);
        let assign7600_e3142: f64 = (p.p183 / assign7600_e3141);
        locals.var_mks_svbsl = assign7600_e3142;
        locals.var_mks_svbsl_rv = 0.0;

        let assign7610_e3146: f64 = (100.0_f64).powf(p.p203);
        let assign7610_e3147: f64 = (p.p202 / assign7610_e3146);
        locals.var_mks_slgl = assign7610_e3147;
        locals.var_mks_slgl_rv = 0.0;

        let assign7620_e3151: f64 = (100.0_f64).powf(p.p191);
        let assign7620_e3152: f64 = (p.p190 / assign7620_e3151);
        locals.var_mks_sub1l = assign7620_e3152;
        locals.var_mks_sub1l_rv = 0.0;

        let assign7630_e3155: f64 = (p.p186 / 100.0);
        locals.var_mks_slg = assign7630_e3155;
        locals.var_mks_slg_rv = 0.0;

        let assign7640_e3158: f64 = (p.p192 / 100.0);
        locals.var_mks_sub2l = assign7640_e3158;
        locals.var_mks_sub2l_rv = 0.0;

        let assign7650_e3161: f64 = (p.p73 * 100.0);
        locals.var_mks_subld2 = assign7650_e3161;
        locals.var_mks_subld2_rv = 0.0;

        let assign7660_e3164: f64 = (p.p311 / 100.0);
        locals.var_mks_rdtemp1 = assign7660_e3164;
        locals.var_mks_rdtemp1_rv = 0.0;

        let assign7670_e3167: f64 = (p.p312 / 100.0);
        locals.var_mks_rdtemp2 = assign7670_e3167;
        locals.var_mks_rdtemp2_rv = 0.0;

        let assign7680_e3170: f64 = (p.p313 / 100.0);
        locals.var_mks_rdvdtemp1 = assign7680_e3170;
        locals.var_mks_rdvdtemp1_rv = 0.0;

        let assign7690_e3173: f64 = (p.p314 / 100.0);
        locals.var_mks_rdvdtemp2 = assign7690_e3173;
        locals.var_mks_rdvdtemp2_rv = 0.0;

        let assign7700_e3176: f64 = (p.p336 / 1e-6);
        locals.var_mks_nsubsub = assign7700_e3176;
        locals.var_mks_nsubsub_rv = 0.0;

        let assign7710_e3179: f64 = (p.p255 * 100.0);
        locals.var_mks_glksd3 = assign7710_e3179;
        locals.var_mks_glksd3_rv = 0.0;

        let assign7720_e3182: f64 = (p.p248 * 100.0);
        locals.var_mks_gleak4 = assign7720_e3182;
        locals.var_mks_gleak4_rv = 0.0;

        let assign7730_e3185: f64 = (p.p249 * 100.0);
        locals.var_mks_gleak5 = assign7730_e3185;
        locals.var_mks_gleak5_rv = 0.0;

        let assign7740_e3188: f64 = (p.p251 / 10000.0);
        locals.var_mks_gleak7 = assign7740_e3188;
        locals.var_mks_gleak7_rv = 0.0;

        let assign7750_e3191: f64 = (p.p266 * 10000.0);
        locals.var_mks_cit = assign7750_e3191;
        locals.var_mks_cit_rv = 0.0;

        let assign7760_e3194: f64 = (p.p275 / 100.0);
        locals.var_mks_ovslp = assign7760_e3194;
        locals.var_mks_ovslp_rv = 0.0;

        let assign7770_e3197: f64 = (p.p272 / 10000.0);
        locals.var_mks_dly3 = assign7770_e3197;
        locals.var_mks_dly3_rv = 0.0;

        let assign7780_e3200: f64 = (p.p273 / 10000.0);
        locals.var_mks_dlyov = assign7780_e3200;
        locals.var_mks_dlyov_dn0 = 0.0;
        locals.var_mks_dlyov_dn2 = 0.0;
        locals.var_mks_dlyov_dn4 = 0.0;
        locals.var_mks_dlyov_dn5 = 0.0;
        locals.var_mks_dlyov_dn6 = 0.0;
        locals.var_mks_dlyov_dn7 = 0.0;
        locals.var_mks_dlyov_dn8 = 0.0;
        locals.var_mks_dlyov_dn9 = 0.0;
        locals.var_mks_dlyov_dn10 = 0.0;
        locals.var_mks_dlyov_dn13 = 0.0;
        locals.var_mks_dlyov_rv = 0.0;

        let assign7800_e3206: f64 = (p.p409 / 10000.0);
        locals.var_mks_rdrmue = assign7800_e3206;
        locals.var_mks_rdrmue_rv = 0.0;

        let assign7810_e3209: f64 = (p.p412 / 100.0);
        locals.var_mks_rdrvmax = assign7810_e3209;
        locals.var_mks_rdrvmax_rv = 0.0;

        let assign7820_e3212: f64 = (p.p413 / 10000.0);
        locals.var_mks_rdrmues = assign7820_e3212;
        locals.var_mks_rdrmues_rv = 0.0;

        let assign7830_e3215: f64 = (p.p414 / 100.0);
        locals.var_mks_rdrvmaxs = assign7830_e3215;
        locals.var_mks_rdrvmaxs_rv = 0.0;

        let assign7840_e3218: f64 = (locals.var_uc_ndepm / 1e-6);
        locals.var_uc_ndepm = assign7840_e3218;
        locals.var_uc_ndepm_dn0 = (locals.var_uc_ndepm_dn0 / 1e-6);
        locals.var_uc_ndepm_dn2 = (locals.var_uc_ndepm_dn2 / 1e-6);
        locals.var_uc_ndepm_dn4 = (locals.var_uc_ndepm_dn4 / 1e-6);
        locals.var_uc_ndepm_dn5 = (locals.var_uc_ndepm_dn5 / 1e-6);
        locals.var_uc_ndepm_dn6 = (locals.var_uc_ndepm_dn6 / 1e-6);
        locals.var_uc_ndepm_dn7 = (locals.var_uc_ndepm_dn7 / 1e-6);
        locals.var_uc_ndepm_dn8 = (locals.var_uc_ndepm_dn8 / 1e-6);
        locals.var_uc_ndepm_dn9 = (locals.var_uc_ndepm_dn9 / 1e-6);
        locals.var_uc_ndepm_dn10 = (locals.var_uc_ndepm_dn10 / 1e-6);
        locals.var_uc_ndepm_dn13 = (locals.var_uc_ndepm_dn13 / 1e-6);
        locals.var_uc_ndepm_rv = 0.0;

        let assign7850_e3221: f64 = (p.p453 / 1e-6);
        locals.var_uc_njunc = assign7850_e3221;
        locals.var_uc_njunc_rv = 0.0;

        let assign7860_e3224: f64 = (p.p274 + 273.15);
        locals.var_ktnom = assign7860_e3224;
        locals.var_ktnom_rv = 0.0;

        let assign7910_e3247: f64 = (p.p0 + p.p116);
        locals.var_lgate = assign7910_e3247;
        locals.var_lgate_rv = 0.0;

        let assign7920_e3250: f64 = (p.p1 / p.p7);
        let assign7920_e3252: f64 = (assign7920_e3250 + p.p117);
        locals.var_wgate = assign7920_e3252;
        locals.var_wgate_rv = 0.0;

        let assign8070_e3352: f64 = (locals.var_lgate * 1000000.0);
        locals.var_lg = assign8070_e3352;
        locals.var_lg_rv = 0.0;

        let assign8080_e3355: f64 = (locals.var_wgate * 1000000.0);
        locals.var_wg = assign8080_e3355;
        locals.var_wg_rv = 0.0;

        let assign8090_e3358: f64 = (locals.var_lg).powf(p.p553);
        locals.var_lbin = assign8090_e3358;
        locals.var_lbin_rv = 0.0;

        let assign8100_e3361: f64 = (locals.var_wg).powf(p.p554);
        locals.var_wbin = assign8100_e3361;
        locals.var_wbin_rv = 0.0;

        let assign8110_e3364: f64 = (locals.var_lbin * locals.var_wbin);
        locals.var_lwbin = assign8110_e3364;
        locals.var_lwbin_rv = 0.0;

        let assign8120_e3368: f64 = (p.p555 / locals.var_lbin);
        let assign8120_e3369: f64 = (p.p89 + assign8120_e3368);
        let assign8120_e3372: f64 = (p.p643 / locals.var_wbin);
        let assign8120_e3373: f64 = (assign8120_e3369 + assign8120_e3372);
        let assign8120_e3376: f64 = (p.p731 / locals.var_lwbin);
        let assign8120_e3377: f64 = (assign8120_e3373 + assign8120_e3376);
        locals.var_uc_vmax = assign8120_e3377;
        locals.var_uc_vmax_rv = 0.0;

        let assign8130_e3381: f64 = (p.p556 / locals.var_lbin);
        let assign8130_e3382: f64 = (p.p92 + assign8130_e3381);
        let assign8130_e3385: f64 = (p.p644 / locals.var_wbin);
        let assign8130_e3386: f64 = (assign8130_e3382 + assign8130_e3385);
        let assign8130_e3389: f64 = (p.p732 / locals.var_lwbin);
        let assign8130_e3390: f64 = (assign8130_e3386 + assign8130_e3389);
        locals.var_uc_bgtmp1 = assign8130_e3390;
        locals.var_uc_bgtmp1_rv = 0.0;

        let assign8140_e3394: f64 = (p.p557 / locals.var_lbin);
        let assign8140_e3395: f64 = (p.p93 + assign8140_e3394);
        let assign8140_e3398: f64 = (p.p645 / locals.var_wbin);
        let assign8140_e3399: f64 = (assign8140_e3395 + assign8140_e3398);
        let assign8140_e3402: f64 = (p.p733 / locals.var_lwbin);
        let assign8140_e3403: f64 = (assign8140_e3399 + assign8140_e3402);
        locals.var_uc_bgtmp2 = assign8140_e3403;
        locals.var_uc_bgtmp2_rv = 0.0;

        let assign8150_e3407: f64 = (p.p558 / locals.var_lbin);
        let assign8150_e3408: f64 = (p.p94 + assign8150_e3407);
        let assign8150_e3411: f64 = (p.p646 / locals.var_wbin);
        let assign8150_e3412: f64 = (assign8150_e3408 + assign8150_e3411);
        let assign8150_e3415: f64 = (p.p734 / locals.var_lwbin);
        let assign8150_e3416: f64 = (assign8150_e3412 + assign8150_e3415);
        locals.var_uc_eg0 = assign8150_e3416;
        locals.var_uc_eg0_rv = 0.0;

        let assign8160_e3420: f64 = (p.p559 / locals.var_lbin);
        let assign8160_e3421: f64 = (p.p110 + assign8160_e3420);
        let assign8160_e3424: f64 = (p.p647 / locals.var_wbin);
        let assign8160_e3425: f64 = (assign8160_e3421 + assign8160_e3424);
        let assign8160_e3428: f64 = (p.p735 / locals.var_lwbin);
        let assign8160_e3429: f64 = (assign8160_e3425 + assign8160_e3428);
        locals.var_uc_vfbover = assign8160_e3429;
        locals.var_uc_vfbover_rv = 0.0;

        let assign8170_e3433: f64 = (p.p560 / locals.var_lbin);
        let assign8170_e3434: f64 = (p.p111 + assign8170_e3433);
        let assign8170_e3437: f64 = (p.p648 / locals.var_wbin);
        let assign8170_e3438: f64 = (assign8170_e3434 + assign8170_e3437);
        let assign8170_e3441: f64 = (p.p736 / locals.var_lwbin);
        let assign8170_e3442: f64 = (assign8170_e3438 + assign8170_e3441);
        locals.var_uc_nover = assign8170_e3442;
        locals.var_uc_nover_rv = 0.0;

        let assign8180_e3446: f64 = (p.p561 / locals.var_lbin);
        let assign8180_e3447: f64 = (p.p112 + assign8180_e3446);
        let assign8180_e3450: f64 = (p.p649 / locals.var_wbin);
        let assign8180_e3451: f64 = (assign8180_e3447 + assign8180_e3450);
        let assign8180_e3454: f64 = (p.p737 / locals.var_lwbin);
        let assign8180_e3455: f64 = (assign8180_e3451 + assign8180_e3454);
        locals.var_uc_novers = assign8180_e3455;
        locals.var_uc_novers_rv = 0.0;

        let assign8190_e3459: f64 = (p.p562 / locals.var_lbin);
        let assign8190_e3460: f64 = (p.p126 + assign8190_e3459);
        let assign8190_e3463: f64 = (p.p650 / locals.var_wbin);
        let assign8190_e3464: f64 = (assign8190_e3460 + assign8190_e3463);
        let assign8190_e3467: f64 = (p.p738 / locals.var_lwbin);
        let assign8190_e3468: f64 = (assign8190_e3464 + assign8190_e3467);
        locals.var_uc_wl2 = assign8190_e3468;
        locals.var_uc_wl2_rv = 0.0;

        let assign8200_e3472: f64 = (p.p563 / locals.var_lbin);
        let assign8200_e3473: f64 = (p.p136 + assign8200_e3472);
        let assign8200_e3476: f64 = (p.p651 / locals.var_wbin);
        let assign8200_e3477: f64 = (assign8200_e3473 + assign8200_e3476);
        let assign8200_e3480: f64 = (p.p739 / locals.var_lwbin);
        let assign8200_e3481: f64 = (assign8200_e3477 + assign8200_e3480);
        locals.var_uc_vfbc = assign8200_e3481;
        locals.var_uc_vfbc_rv = 0.0;

        let assign8210_e3485: f64 = (p.p564 / locals.var_lbin);
        let assign8210_e3486: f64 = (p.p138 + assign8210_e3485);
        let assign8210_e3489: f64 = (p.p652 / locals.var_wbin);
        let assign8210_e3490: f64 = (assign8210_e3486 + assign8210_e3489);
        let assign8210_e3493: f64 = (p.p740 / locals.var_lwbin);
        let assign8210_e3494: f64 = (assign8210_e3490 + assign8210_e3493);
        locals.var_uc_nsubc = assign8210_e3494;
        locals.var_uc_nsubc_rv = 0.0;

        let assign8220_e3498: f64 = (p.p565 / locals.var_lbin);
        let assign8220_e3499: f64 = (p.p141 + assign8220_e3498);
        let assign8220_e3502: f64 = (p.p653 / locals.var_wbin);
        let assign8220_e3503: f64 = (assign8220_e3499 + assign8220_e3502);
        let assign8220_e3506: f64 = (p.p741 / locals.var_lwbin);
        let assign8220_e3507: f64 = (assign8220_e3503 + assign8220_e3506);
        locals.var_uc_nsubp = assign8220_e3507;
        locals.var_uc_nsubp_rv = 0.0;

        let assign8230_e3511: f64 = (p.p566 / locals.var_lbin);
        let assign8230_e3512: f64 = (p.p144 + assign8230_e3511);
        let assign8230_e3515: f64 = (p.p654 / locals.var_wbin);
        let assign8230_e3516: f64 = (assign8230_e3512 + assign8230_e3515);
        let assign8230_e3519: f64 = (p.p742 / locals.var_lwbin);
        let assign8230_e3520: f64 = (assign8230_e3516 + assign8230_e3519);
        locals.var_uc_scp1 = assign8230_e3520;
        locals.var_uc_scp1_rv = 0.0;

        let assign8240_e3524: f64 = (p.p567 / locals.var_lbin);
        let assign8240_e3525: f64 = (p.p145 + assign8240_e3524);
        let assign8240_e3528: f64 = (p.p655 / locals.var_wbin);
        let assign8240_e3529: f64 = (assign8240_e3525 + assign8240_e3528);
        let assign8240_e3532: f64 = (p.p743 / locals.var_lwbin);
        let assign8240_e3533: f64 = (assign8240_e3529 + assign8240_e3532);
        locals.var_uc_scp2 = assign8240_e3533;
        locals.var_uc_scp2_rv = 0.0;

        let assign8250_e3537: f64 = (p.p568 / locals.var_lbin);
        let assign8250_e3538: f64 = (p.p146 + assign8250_e3537);
        let assign8250_e3541: f64 = (p.p656 / locals.var_wbin);
        let assign8250_e3542: f64 = (assign8250_e3538 + assign8250_e3541);
        let assign8250_e3545: f64 = (p.p744 / locals.var_lwbin);
        let assign8250_e3546: f64 = (assign8250_e3542 + assign8250_e3545);
        locals.var_uc_scp3 = assign8250_e3546;
        locals.var_uc_scp3_rv = 0.0;

        let assign8260_e3550: f64 = (p.p569 / locals.var_lbin);
        let assign8260_e3551: f64 = (p.p147 + assign8260_e3550);
        let assign8260_e3554: f64 = (p.p657 / locals.var_wbin);
        let assign8260_e3555: f64 = (assign8260_e3551 + assign8260_e3554);
        let assign8260_e3558: f64 = (p.p745 / locals.var_lwbin);
        let assign8260_e3559: f64 = (assign8260_e3555 + assign8260_e3558);
        locals.var_uc_sc1 = assign8260_e3559;
        locals.var_uc_sc1_rv = 0.0;

        let assign8270_e3563: f64 = (p.p570 / locals.var_lbin);
        let assign8270_e3564: f64 = (p.p148 + assign8270_e3563);
        let assign8270_e3567: f64 = (p.p658 / locals.var_wbin);
        let assign8270_e3568: f64 = (assign8270_e3564 + assign8270_e3567);
        let assign8270_e3571: f64 = (p.p746 / locals.var_lwbin);
        let assign8270_e3572: f64 = (assign8270_e3568 + assign8270_e3571);
        locals.var_uc_sc2 = assign8270_e3572;
        locals.var_uc_sc2_rv = 0.0;

        let assign8280_e3576: f64 = (p.p571 / locals.var_lbin);
        let assign8280_e3577: f64 = (p.p149 + assign8280_e3576);
        let assign8280_e3580: f64 = (p.p659 / locals.var_wbin);
        let assign8280_e3581: f64 = (assign8280_e3577 + assign8280_e3580);
        let assign8280_e3584: f64 = (p.p747 / locals.var_lwbin);
        let assign8280_e3585: f64 = (assign8280_e3581 + assign8280_e3584);
        locals.var_uc_sc3 = assign8280_e3585;
        locals.var_uc_sc3_rv = 0.0;

        let assign8290_e3589: f64 = (p.p572 / locals.var_lbin);
        let assign8290_e3590: f64 = (p.p151 + assign8290_e3589);
        let assign8290_e3593: f64 = (p.p660 / locals.var_wbin);
        let assign8290_e3594: f64 = (assign8290_e3590 + assign8290_e3593);
        let assign8290_e3597: f64 = (p.p748 / locals.var_lwbin);
        let assign8290_e3598: f64 = (assign8290_e3594 + assign8290_e3597);
        locals.var_uc_pgd1 = assign8290_e3598;
        locals.var_uc_pgd1_rv = 0.0;

        let assign8300_e3602: f64 = (p.p573 / locals.var_lbin);
        let assign8300_e3603: f64 = (p.p154 + assign8300_e3602);
        let assign8300_e3606: f64 = (p.p661 / locals.var_wbin);
        let assign8300_e3607: f64 = (assign8300_e3603 + assign8300_e3606);
        let assign8300_e3610: f64 = (p.p749 / locals.var_lwbin);
        let assign8300_e3611: f64 = (assign8300_e3607 + assign8300_e3610);
        locals.var_uc_ndep = assign8300_e3611;
        locals.var_uc_ndep_rv = 0.0;

        let assign8310_e3615: f64 = (p.p574 / locals.var_lbin);
        let assign8310_e3616: f64 = (p.p157 + assign8310_e3615);
        let assign8310_e3619: f64 = (p.p662 / locals.var_wbin);
        let assign8310_e3620: f64 = (assign8310_e3616 + assign8310_e3619);
        let assign8310_e3623: f64 = (p.p750 / locals.var_lwbin);
        let assign8310_e3624: f64 = (assign8310_e3620 + assign8310_e3623);
        locals.var_uc_ninv = assign8310_e3624;
        locals.var_uc_ninv_rv = 0.0;

        let assign8320_e3628: f64 = (p.p575 / locals.var_lbin);
        let assign8320_e3629: f64 = (p.p158 + assign8320_e3628);
        let assign8320_e3632: f64 = (p.p663 / locals.var_wbin);
        let assign8320_e3633: f64 = (assign8320_e3629 + assign8320_e3632);
        let assign8320_e3636: f64 = (p.p751 / locals.var_lwbin);
        let assign8320_e3637: f64 = (assign8320_e3633 + assign8320_e3636);
        locals.var_uc_muecb0 = assign8320_e3637;
        locals.var_uc_muecb0_rv = 0.0;

        let assign8330_e3641: f64 = (p.p576 / locals.var_lbin);
        let assign8330_e3642: f64 = (p.p159 + assign8330_e3641);
        let assign8330_e3645: f64 = (p.p664 / locals.var_wbin);
        let assign8330_e3646: f64 = (assign8330_e3642 + assign8330_e3645);
        let assign8330_e3649: f64 = (p.p752 / locals.var_lwbin);
        let assign8330_e3650: f64 = (assign8330_e3646 + assign8330_e3649);
        locals.var_uc_muecb1 = assign8330_e3650;
        locals.var_uc_muecb1_rv = 0.0;

        let assign8340_e3654: f64 = (p.p577 / locals.var_lbin);
        let assign8340_e3655: f64 = (p.p161 + assign8340_e3654);
        let assign8340_e3658: f64 = (p.p665 / locals.var_wbin);
        let assign8340_e3659: f64 = (assign8340_e3655 + assign8340_e3658);
        let assign8340_e3662: f64 = (p.p753 / locals.var_lwbin);
        let assign8340_e3663: f64 = (assign8340_e3659 + assign8340_e3662);
        locals.var_uc_mueph1 = assign8340_e3663;
        locals.var_uc_mueph1_rv = 0.0;

        let assign8350_e3667: f64 = (p.p578 / locals.var_lbin);
        let assign8350_e3668: f64 = (p.p169 + assign8350_e3667);
        let assign8350_e3671: f64 = (p.p666 / locals.var_wbin);
        let assign8350_e3672: f64 = (assign8350_e3668 + assign8350_e3671);
        let assign8350_e3675: f64 = (p.p754 / locals.var_lwbin);
        let assign8350_e3676: f64 = (assign8350_e3672 + assign8350_e3675);
        locals.var_uc_vtmp = assign8350_e3676;
        locals.var_uc_vtmp_rv = 0.0;

        let assign8360_e3680: f64 = (p.p579 / locals.var_lbin);
        let assign8360_e3681: f64 = (p.p170 + assign8360_e3680);
        let assign8360_e3684: f64 = (p.p667 / locals.var_wbin);
        let assign8360_e3685: f64 = (assign8360_e3681 + assign8360_e3684);
        let assign8360_e3688: f64 = (p.p755 / locals.var_lwbin);
        let assign8360_e3689: f64 = (assign8360_e3685 + assign8360_e3688);
        locals.var_uc_wvth0 = assign8360_e3689;
        locals.var_uc_wvth0_rv = 0.0;

        let assign8370_e3693: f64 = (p.p580 / locals.var_lbin);
        let assign8370_e3694: f64 = (p.p172 + assign8370_e3693);
        let assign8370_e3697: f64 = (p.p668 / locals.var_wbin);
        let assign8370_e3698: f64 = (assign8370_e3694 + assign8370_e3697);
        let assign8370_e3701: f64 = (p.p756 / locals.var_lwbin);
        let assign8370_e3702: f64 = (assign8370_e3698 + assign8370_e3701);
        locals.var_uc_muesr1 = assign8370_e3702;
        locals.var_uc_muesr1_rv = 0.0;

        let assign8380_e3706: f64 = (p.p581 / locals.var_lbin);
        let assign8380_e3707: f64 = (p.p177 + assign8380_e3706);
        let assign8380_e3710: f64 = (p.p669 / locals.var_wbin);
        let assign8380_e3711: f64 = (assign8380_e3707 + assign8380_e3710);
        let assign8380_e3714: f64 = (p.p757 / locals.var_lwbin);
        let assign8380_e3715: f64 = (assign8380_e3711 + assign8380_e3714);
        locals.var_uc_muetmp = assign8380_e3715;
        locals.var_uc_muetmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign8390_e3719: f64 = (p.p582 / locals.var_lbin);
        let assign8390_e3720: f64 = (p.p179 + assign8390_e3719);
        let assign8390_e3723: f64 = (p.p670 / locals.var_wbin);
        let assign8390_e3724: f64 = (assign8390_e3720 + assign8390_e3723);
        let assign8390_e3727: f64 = (p.p758 / locals.var_lwbin);
        let assign8390_e3728: f64 = (assign8390_e3724 + assign8390_e3727);
        locals.var_uc_sub1 = assign8390_e3728;
        locals.var_uc_sub1_rv = 0.0;

        let assign8400_e3732: f64 = (p.p583 / locals.var_lbin);
        let assign8400_e3733: f64 = (p.p180 + assign8400_e3732);
        let assign8400_e3736: f64 = (p.p671 / locals.var_wbin);
        let assign8400_e3737: f64 = (assign8400_e3733 + assign8400_e3736);
        let assign8400_e3740: f64 = (p.p759 / locals.var_lwbin);
        let assign8400_e3741: f64 = (assign8400_e3737 + assign8400_e3740);
        locals.var_uc_sub2 = assign8400_e3741;
        locals.var_uc_sub2_rv = 0.0;

        let assign8410_e3745: f64 = (p.p584 / locals.var_lbin);
        let assign8410_e3746: f64 = (p.p185 + assign8410_e3745);
        let assign8410_e3749: f64 = (p.p672 / locals.var_wbin);
        let assign8410_e3750: f64 = (assign8410_e3746 + assign8410_e3749);
        let assign8410_e3753: f64 = (p.p760 / locals.var_lwbin);
        let assign8410_e3754: f64 = (assign8410_e3750 + assign8410_e3753);
        locals.var_uc_svds = assign8410_e3754;
        locals.var_uc_svds_rv = 0.0;

        let assign8420_e3758: f64 = (p.p585 / locals.var_lbin);
        let assign8420_e3759: f64 = (p.p182 + assign8420_e3758);
        let assign8420_e3762: f64 = (p.p673 / locals.var_wbin);
        let assign8420_e3763: f64 = (assign8420_e3759 + assign8420_e3762);
        let assign8420_e3766: f64 = (p.p761 / locals.var_lwbin);
        let assign8420_e3767: f64 = (assign8420_e3763 + assign8420_e3766);
        locals.var_uc_svbs = assign8420_e3767;
        locals.var_uc_svbs_rv = 0.0;

        let assign8430_e3771: f64 = (p.p586 / locals.var_lbin);
        let assign8430_e3772: f64 = (p.p181 + assign8430_e3771);
        let assign8430_e3775: f64 = (p.p674 / locals.var_wbin);
        let assign8430_e3776: f64 = (assign8430_e3772 + assign8430_e3775);
        let assign8430_e3779: f64 = (p.p762 / locals.var_lwbin);
        let assign8430_e3780: f64 = (assign8430_e3776 + assign8430_e3779);
        locals.var_uc_svgs = assign8430_e3780;
        locals.var_uc_svgs_rv = 0.0;

        let assign8440_e3784: f64 = (p.p587 / locals.var_lbin);
        let assign8440_e3785: f64 = (p.p187 + assign8440_e3784);
        let assign8440_e3788: f64 = (p.p675 / locals.var_wbin);
        let assign8440_e3789: f64 = (assign8440_e3785 + assign8440_e3788);
        let assign8440_e3792: f64 = (p.p763 / locals.var_lwbin);
        let assign8440_e3793: f64 = (assign8440_e3789 + assign8440_e3792);
        locals.var_uc_sub1snp = assign8440_e3793;
        locals.var_uc_sub1snp_rv = 0.0;

        let assign8450_e3797: f64 = (p.p588 / locals.var_lbin);
        let assign8450_e3798: f64 = (p.p188 + assign8450_e3797);
        let assign8450_e3801: f64 = (p.p676 / locals.var_wbin);
        let assign8450_e3802: f64 = (assign8450_e3798 + assign8450_e3801);
        let assign8450_e3805: f64 = (p.p764 / locals.var_lwbin);
        let assign8450_e3806: f64 = (assign8450_e3802 + assign8450_e3805);
        locals.var_uc_sub2snp = assign8450_e3806;
        locals.var_uc_sub2snp_rv = 0.0;

        let assign8460_e3810: f64 = (p.p589 / locals.var_lbin);
        let assign8460_e3811: f64 = (p.p189 + assign8460_e3810);
        let assign8460_e3814: f64 = (p.p677 / locals.var_wbin);
        let assign8460_e3815: f64 = (assign8460_e3811 + assign8460_e3814);
        let assign8460_e3818: f64 = (p.p765 / locals.var_lwbin);
        let assign8460_e3819: f64 = (assign8460_e3815 + assign8460_e3818);
        locals.var_uc_svdssnp = assign8460_e3819;
        locals.var_uc_svdssnp_rv = 0.0;

        let assign8470_e3823: f64 = (p.p590 / locals.var_lbin);
        let assign8470_e3824: f64 = (p.p194 + assign8470_e3823);
        let assign8470_e3827: f64 = (p.p678 / locals.var_wbin);
        let assign8470_e3828: f64 = (assign8470_e3824 + assign8470_e3827);
        let assign8470_e3831: f64 = (p.p766 / locals.var_lwbin);
        let assign8470_e3832: f64 = (assign8470_e3828 + assign8470_e3831);
        locals.var_uc_fn1 = assign8470_e3832;
        locals.var_uc_fn1_rv = 0.0;

        let assign8480_e3836: f64 = (p.p591 / locals.var_lbin);
        let assign8480_e3837: f64 = (p.p195 + assign8480_e3836);
        let assign8480_e3840: f64 = (p.p679 / locals.var_wbin);
        let assign8480_e3841: f64 = (assign8480_e3837 + assign8480_e3840);
        let assign8480_e3844: f64 = (p.p767 / locals.var_lwbin);
        let assign8480_e3845: f64 = (assign8480_e3841 + assign8480_e3844);
        locals.var_uc_fn2 = assign8480_e3845;
        locals.var_uc_fn2_rv = 0.0;

        let assign8490_e3849: f64 = (p.p592 / locals.var_lbin);
        let assign8490_e3850: f64 = (p.p196 + assign8490_e3849);
        let assign8490_e3853: f64 = (p.p680 / locals.var_wbin);
        let assign8490_e3854: f64 = (assign8490_e3850 + assign8490_e3853);
        let assign8490_e3857: f64 = (p.p768 / locals.var_lwbin);
        let assign8490_e3858: f64 = (assign8490_e3854 + assign8490_e3857);
        locals.var_uc_fn3 = assign8490_e3858;
        locals.var_uc_fn3_rv = 0.0;

        let assign8500_e3862: f64 = (p.p593 / locals.var_lbin);
        let assign8500_e3863: f64 = (p.p197 + assign8500_e3862);
        let assign8500_e3866: f64 = (p.p681 / locals.var_wbin);
        let assign8500_e3867: f64 = (assign8500_e3863 + assign8500_e3866);
        let assign8500_e3870: f64 = (p.p769 / locals.var_lwbin);
        let assign8500_e3871: f64 = (assign8500_e3867 + assign8500_e3870);
        locals.var_uc_fvbs = assign8500_e3871;
        locals.var_uc_fvbs_rv = 0.0;

        let assign8510_e3875: f64 = (p.p594 / locals.var_lbin);
        let assign8510_e3876: f64 = (p.p204 + assign8510_e3875);
        let assign8510_e3879: f64 = (p.p682 / locals.var_wbin);
        let assign8510_e3880: f64 = (assign8510_e3876 + assign8510_e3879);
        let assign8510_e3883: f64 = (p.p770 / locals.var_lwbin);
        let assign8510_e3884: f64 = (assign8510_e3880 + assign8510_e3883);
        locals.var_uc_nsti = assign8510_e3884;
        locals.var_uc_nsti_rv = 0.0;

        let assign8520_e3888: f64 = (p.p595 / locals.var_lbin);
        let assign8520_e3889: f64 = (p.p205 + assign8520_e3888);
        let assign8520_e3892: f64 = (p.p683 / locals.var_wbin);
        let assign8520_e3893: f64 = (assign8520_e3889 + assign8520_e3892);
        let assign8520_e3896: f64 = (p.p771 / locals.var_lwbin);
        let assign8520_e3897: f64 = (assign8520_e3893 + assign8520_e3896);
        locals.var_uc_wsti = assign8520_e3897;
        locals.var_uc_wsti_dn0 = 0.0;
        locals.var_uc_wsti_dn2 = 0.0;
        locals.var_uc_wsti_dn4 = 0.0;
        locals.var_uc_wsti_dn5 = 0.0;
        locals.var_uc_wsti_dn6 = 0.0;
        locals.var_uc_wsti_dn7 = 0.0;
        locals.var_uc_wsti_dn8 = 0.0;
        locals.var_uc_wsti_dn9 = 0.0;
        locals.var_uc_wsti_dn10 = 0.0;
        locals.var_uc_wsti_dn13 = 0.0;
        locals.var_uc_wsti_rv = 0.0;

        let assign8530_e3901: f64 = (p.p596 / locals.var_lbin);
        let assign8530_e3902: f64 = (p.p210 + assign8530_e3901);
        let assign8530_e3905: f64 = (p.p684 / locals.var_wbin);
        let assign8530_e3906: f64 = (assign8530_e3902 + assign8530_e3905);
        let assign8530_e3909: f64 = (p.p772 / locals.var_lwbin);
        let assign8530_e3910: f64 = (assign8530_e3906 + assign8530_e3909);
        locals.var_uc_scsti1 = assign8530_e3910;
        locals.var_uc_scsti1_rv = 0.0;

        let assign8540_e3914: f64 = (p.p597 / locals.var_lbin);
        let assign8540_e3915: f64 = (p.p211 + assign8540_e3914);
        let assign8540_e3918: f64 = (p.p685 / locals.var_wbin);
        let assign8540_e3919: f64 = (assign8540_e3915 + assign8540_e3918);
        let assign8540_e3922: f64 = (p.p773 / locals.var_lwbin);
        let assign8540_e3923: f64 = (assign8540_e3919 + assign8540_e3922);
        locals.var_uc_scsti2 = assign8540_e3923;
        locals.var_uc_scsti2_rv = 0.0;

        let assign8550_e3927: f64 = (p.p598 / locals.var_lbin);
        let assign8550_e3928: f64 = (p.p212 + assign8550_e3927);
        let assign8550_e3931: f64 = (p.p686 / locals.var_wbin);
        let assign8550_e3932: f64 = (assign8550_e3928 + assign8550_e3931);
        let assign8550_e3935: f64 = (p.p774 / locals.var_lwbin);
        let assign8550_e3936: f64 = (assign8550_e3932 + assign8550_e3935);
        locals.var_uc_vthsti = assign8550_e3936;
        locals.var_uc_vthsti_rv = 0.0;

        let assign8560_e3940: f64 = (p.p599 / locals.var_lbin);
        let assign8560_e3941: f64 = (p.p214 + assign8560_e3940);
        let assign8560_e3944: f64 = (p.p687 / locals.var_wbin);
        let assign8560_e3945: f64 = (assign8560_e3941 + assign8560_e3944);
        let assign8560_e3948: f64 = (p.p775 / locals.var_lwbin);
        let assign8560_e3949: f64 = (assign8560_e3945 + assign8560_e3948);
        locals.var_uc_muesti1 = assign8560_e3949;
        locals.var_uc_muesti1_rv = 0.0;

        let assign8570_e3953: f64 = (p.p600 / locals.var_lbin);
        let assign8570_e3954: f64 = (p.p215 + assign8570_e3953);
        let assign8570_e3957: f64 = (p.p688 / locals.var_wbin);
        let assign8570_e3958: f64 = (assign8570_e3954 + assign8570_e3957);
        let assign8570_e3961: f64 = (p.p776 / locals.var_lwbin);
        let assign8570_e3962: f64 = (assign8570_e3958 + assign8570_e3961);
        locals.var_uc_muesti2 = assign8570_e3962;
        locals.var_uc_muesti2_rv = 0.0;

        let assign8580_e3966: f64 = (p.p601 / locals.var_lbin);
        let assign8580_e3967: f64 = (p.p216 + assign8580_e3966);
        let assign8580_e3970: f64 = (p.p689 / locals.var_wbin);
        let assign8580_e3971: f64 = (assign8580_e3967 + assign8580_e3970);
        let assign8580_e3974: f64 = (p.p777 / locals.var_lwbin);
        let assign8580_e3975: f64 = (assign8580_e3971 + assign8580_e3974);
        locals.var_uc_muesti3 = assign8580_e3975;
        locals.var_uc_muesti3_rv = 0.0;

        let assign8590_e3979: f64 = (p.p602 / locals.var_lbin);
        let assign8590_e3980: f64 = (p.p217 + assign8590_e3979);
        let assign8590_e3983: f64 = (p.p690 / locals.var_wbin);
        let assign8590_e3984: f64 = (assign8590_e3980 + assign8590_e3983);
        let assign8590_e3987: f64 = (p.p778 / locals.var_lwbin);
        let assign8590_e3988: f64 = (assign8590_e3984 + assign8590_e3987);
        locals.var_uc_nsubpsti1 = assign8590_e3988;
        locals.var_uc_nsubpsti1_rv = 0.0;

        let assign8600_e3992: f64 = (p.p603 / locals.var_lbin);
        let assign8600_e3993: f64 = (p.p218 + assign8600_e3992);
        let assign8600_e3996: f64 = (p.p691 / locals.var_wbin);
        let assign8600_e3997: f64 = (assign8600_e3993 + assign8600_e3996);
        let assign8600_e4000: f64 = (p.p779 / locals.var_lwbin);
        let assign8600_e4001: f64 = (assign8600_e3997 + assign8600_e4000);
        locals.var_uc_nsubpsti2 = assign8600_e4001;
        locals.var_uc_nsubpsti2_rv = 0.0;

        let assign8610_e4005: f64 = (p.p604 / locals.var_lbin);
        let assign8610_e4006: f64 = (p.p219 + assign8610_e4005);
        let assign8610_e4009: f64 = (p.p692 / locals.var_wbin);
        let assign8610_e4010: f64 = (assign8610_e4006 + assign8610_e4009);
        let assign8610_e4013: f64 = (p.p780 / locals.var_lwbin);
        let assign8610_e4014: f64 = (assign8610_e4010 + assign8610_e4013);
        locals.var_uc_nsubpsti3 = assign8610_e4014;
        locals.var_uc_nsubpsti3_rv = 0.0;

        let assign8620_e4018: f64 = (p.p605 / locals.var_lbin);
        let assign8620_e4019: f64 = (p.p269 + assign8620_e4018);
        let assign8620_e4022: f64 = (p.p693 / locals.var_wbin);
        let assign8620_e4023: f64 = (assign8620_e4019 + assign8620_e4022);
        let assign8620_e4026: f64 = (p.p781 / locals.var_lwbin);
        let assign8620_e4027: f64 = (assign8620_e4023 + assign8620_e4026);
        locals.var_uc_cgso = assign8620_e4027;
        locals.var_uc_cgso_rv = 0.0;

        let assign8630_e4031: f64 = (p.p606 / locals.var_lbin);
        let assign8630_e4032: f64 = (p.p268 + assign8630_e4031);
        let assign8630_e4035: f64 = (p.p694 / locals.var_wbin);
        let assign8630_e4036: f64 = (assign8630_e4032 + assign8630_e4035);
        let assign8630_e4039: f64 = (p.p782 / locals.var_lwbin);
        let assign8630_e4040: f64 = (assign8630_e4036 + assign8630_e4039);
        locals.var_uc_cgdo = assign8630_e4040;
        locals.var_uc_cgdo_rv = 0.0;

        let assign8640_e4044: f64 = (p.p607 / locals.var_lbin);
        let assign8640_e4045: f64 = (p.p226 + assign8640_e4044);
        let assign8640_e4048: f64 = (p.p695 / locals.var_wbin);
        let assign8640_e4049: f64 = (assign8640_e4045 + assign8640_e4048);
        let assign8640_e4052: f64 = (p.p783 / locals.var_lwbin);
        let assign8640_e4053: f64 = (assign8640_e4049 + assign8640_e4052);
        locals.var_uc_clm1 = assign8640_e4053;
        locals.var_uc_clm1_rv = 0.0;

        let assign8650_e4057: f64 = (p.p608 / locals.var_lbin);
        let assign8650_e4058: f64 = (p.p227 + assign8650_e4057);
        let assign8650_e4061: f64 = (p.p696 / locals.var_wbin);
        let assign8650_e4062: f64 = (assign8650_e4058 + assign8650_e4061);
        let assign8650_e4065: f64 = (p.p784 / locals.var_lwbin);
        let assign8650_e4066: f64 = (assign8650_e4062 + assign8650_e4065);
        locals.var_uc_clm2 = assign8650_e4066;
        locals.var_uc_clm2_dn0 = 0.0;
        locals.var_uc_clm2_dn2 = 0.0;
        locals.var_uc_clm2_dn4 = 0.0;
        locals.var_uc_clm2_dn5 = 0.0;
        locals.var_uc_clm2_dn6 = 0.0;
        locals.var_uc_clm2_dn7 = 0.0;
        locals.var_uc_clm2_dn8 = 0.0;
        locals.var_uc_clm2_dn9 = 0.0;
        locals.var_uc_clm2_dn10 = 0.0;
        locals.var_uc_clm2_dn13 = 0.0;
        locals.var_uc_clm2_rv = 0.0;

        let assign8660_e4070: f64 = (p.p609 / locals.var_lbin);
        let assign8660_e4071: f64 = (p.p228 + assign8660_e4070);
        let assign8660_e4074: f64 = (p.p697 / locals.var_wbin);
        let assign8660_e4075: f64 = (assign8660_e4071 + assign8660_e4074);
        let assign8660_e4078: f64 = (p.p785 / locals.var_lwbin);
        let assign8660_e4079: f64 = (assign8660_e4075 + assign8660_e4078);
        locals.var_uc_clm3 = assign8660_e4079;
        locals.var_uc_clm3_rv = 0.0;

        let assign8670_e4083: f64 = (p.p610 / locals.var_lbin);
        let assign8670_e4084: f64 = (p.p232 + assign8670_e4083);
        let assign8670_e4087: f64 = (p.p698 / locals.var_wbin);
        let assign8670_e4088: f64 = (assign8670_e4084 + assign8670_e4087);
        let assign8670_e4091: f64 = (p.p786 / locals.var_lwbin);
        let assign8670_e4092: f64 = (assign8670_e4088 + assign8670_e4091);
        locals.var_uc_wfc = assign8670_e4092;
        locals.var_uc_wfc_rv = 0.0;

        let assign8680_e4096: f64 = (p.p611 / locals.var_lbin);
        let assign8680_e4097: f64 = (p.p240 + assign8680_e4096);
        let assign8680_e4100: f64 = (p.p699 / locals.var_wbin);
        let assign8680_e4101: f64 = (assign8680_e4097 + assign8680_e4100);
        let assign8680_e4104: f64 = (p.p787 / locals.var_lwbin);
        let assign8680_e4105: f64 = (assign8680_e4101 + assign8680_e4104);
        locals.var_uc_gidl1 = assign8680_e4105;
        locals.var_uc_gidl1_rv = 0.0;

        let assign8690_e4109: f64 = (p.p612 / locals.var_lbin);
        let assign8690_e4110: f64 = (p.p241 + assign8690_e4109);
        let assign8690_e4113: f64 = (p.p700 / locals.var_wbin);
        let assign8690_e4114: f64 = (assign8690_e4110 + assign8690_e4113);
        let assign8690_e4117: f64 = (p.p788 / locals.var_lwbin);
        let assign8690_e4118: f64 = (assign8690_e4114 + assign8690_e4117);
        locals.var_uc_gidl2 = assign8690_e4118;
        locals.var_uc_gidl2_rv = 0.0;

        let assign8700_e4122: f64 = (p.p613 / locals.var_lbin);
        let assign8700_e4123: f64 = (p.p245 + assign8700_e4122);
        let assign8700_e4126: f64 = (p.p701 / locals.var_wbin);
        let assign8700_e4127: f64 = (assign8700_e4123 + assign8700_e4126);
        let assign8700_e4130: f64 = (p.p789 / locals.var_lwbin);
        let assign8700_e4131: f64 = (assign8700_e4127 + assign8700_e4130);
        locals.var_uc_gleak1 = assign8700_e4131;
        locals.var_uc_gleak1_rv = 0.0;

        let assign8710_e4135: f64 = (p.p614 / locals.var_lbin);
        let assign8710_e4136: f64 = (p.p246 + assign8710_e4135);
        let assign8710_e4139: f64 = (p.p702 / locals.var_wbin);
        let assign8710_e4140: f64 = (assign8710_e4136 + assign8710_e4139);
        let assign8710_e4143: f64 = (p.p790 / locals.var_lwbin);
        let assign8710_e4144: f64 = (assign8710_e4140 + assign8710_e4143);
        locals.var_uc_gleak2 = assign8710_e4144;
        locals.var_uc_gleak2_rv = 0.0;

        let assign8720_e4148: f64 = (p.p615 / locals.var_lbin);
        let assign8720_e4149: f64 = (p.p247 + assign8720_e4148);
        let assign8720_e4152: f64 = (p.p703 / locals.var_wbin);
        let assign8720_e4153: f64 = (assign8720_e4149 + assign8720_e4152);
        let assign8720_e4156: f64 = (p.p791 / locals.var_lwbin);
        let assign8720_e4157: f64 = (assign8720_e4153 + assign8720_e4156);
        locals.var_uc_gleak3 = assign8720_e4157;
        locals.var_uc_gleak3_rv = 0.0;

        let assign8730_e4161: f64 = (p.p616 / locals.var_lbin);
        let assign8730_e4162: f64 = (p.p250 + assign8730_e4161);
        let assign8730_e4165: f64 = (p.p704 / locals.var_wbin);
        let assign8730_e4166: f64 = (assign8730_e4162 + assign8730_e4165);
        let assign8730_e4169: f64 = (p.p792 / locals.var_lwbin);
        let assign8730_e4170: f64 = (assign8730_e4166 + assign8730_e4169);
        locals.var_uc_gleak6 = assign8730_e4170;
        locals.var_uc_gleak6_rv = 0.0;

        let assign8740_e4174: f64 = (p.p617 / locals.var_lbin);
        let assign8740_e4175: f64 = (p.p253 + assign8740_e4174);
        let assign8740_e4178: f64 = (p.p705 / locals.var_wbin);
        let assign8740_e4179: f64 = (assign8740_e4175 + assign8740_e4178);
        let assign8740_e4182: f64 = (p.p793 / locals.var_lwbin);
        let assign8740_e4183: f64 = (assign8740_e4179 + assign8740_e4182);
        locals.var_uc_glksd1 = assign8740_e4183;
        locals.var_uc_glksd1_rv = 0.0;

        let assign8750_e4187: f64 = (p.p618 / locals.var_lbin);
        let assign8750_e4188: f64 = (p.p254 + assign8750_e4187);
        let assign8750_e4191: f64 = (p.p706 / locals.var_wbin);
        let assign8750_e4192: f64 = (assign8750_e4188 + assign8750_e4191);
        let assign8750_e4195: f64 = (p.p794 / locals.var_lwbin);
        let assign8750_e4196: f64 = (assign8750_e4192 + assign8750_e4195);
        locals.var_uc_glksd2 = assign8750_e4196;
        locals.var_uc_glksd2_rv = 0.0;

        let assign8760_e4200: f64 = (p.p619 / locals.var_lbin);
        let assign8760_e4201: f64 = (p.p256 + assign8760_e4200);
        let assign8760_e4204: f64 = (p.p707 / locals.var_wbin);
        let assign8760_e4205: f64 = (assign8760_e4201 + assign8760_e4204);
        let assign8760_e4208: f64 = (p.p795 / locals.var_lwbin);
        let assign8760_e4209: f64 = (assign8760_e4205 + assign8760_e4208);
        locals.var_uc_glkb1 = assign8760_e4209;
        locals.var_uc_glkb1_rv = 0.0;

        let assign8770_e4213: f64 = (p.p620 / locals.var_lbin);
        let assign8770_e4214: f64 = (p.p257 + assign8770_e4213);
        let assign8770_e4217: f64 = (p.p708 / locals.var_wbin);
        let assign8770_e4218: f64 = (assign8770_e4214 + assign8770_e4217);
        let assign8770_e4221: f64 = (p.p796 / locals.var_lwbin);
        let assign8770_e4222: f64 = (assign8770_e4218 + assign8770_e4221);
        locals.var_uc_glkb2 = assign8770_e4222;
        locals.var_uc_glkb2_rv = 0.0;

        let assign8790_e4239: f64 = (p.p622 / locals.var_lbin);
        let assign8790_e4240: f64 = (p.p265 + assign8790_e4239);
        let assign8790_e4243: f64 = (p.p710 / locals.var_wbin);
        let assign8790_e4244: f64 = (assign8790_e4240 + assign8790_e4243);
        let assign8790_e4247: f64 = (p.p798 / locals.var_lwbin);
        let assign8790_e4248: f64 = (assign8790_e4244 + assign8790_e4247);
        locals.var_uc_nfalp = assign8790_e4248;
        locals.var_uc_nfalp_rv = 0.0;

        let assign8800_e4252: f64 = (p.p623 / locals.var_lbin);
        let assign8800_e4253: f64 = (p.p278 + assign8800_e4252);
        let assign8800_e4256: f64 = (p.p711 / locals.var_wbin);
        let assign8800_e4257: f64 = (assign8800_e4253 + assign8800_e4256);
        let assign8800_e4260: f64 = (p.p799 / locals.var_lwbin);
        let assign8800_e4261: f64 = (assign8800_e4257 + assign8800_e4260);
        locals.var_uc_ibpc1 = assign8800_e4261;
        locals.var_uc_ibpc1_rv = 0.0;

        let assign8810_e4265: f64 = (p.p624 / locals.var_lbin);
        let assign8810_e4266: f64 = (p.p281 + assign8810_e4265);
        let assign8810_e4269: f64 = (p.p712 / locals.var_wbin);
        let assign8810_e4270: f64 = (assign8810_e4266 + assign8810_e4269);
        let assign8810_e4273: f64 = (p.p800 / locals.var_lwbin);
        let assign8810_e4274: f64 = (assign8810_e4270 + assign8810_e4273);
        locals.var_uc_ibpc2 = assign8810_e4274;
        locals.var_uc_ibpc2_rv = 0.0;

        let assign8820_e4278: f64 = (p.p625 / locals.var_lbin);
        let assign8820_e4279: f64 = (p.p79 + assign8820_e4278);
        let assign8820_e4282: f64 = (p.p713 / locals.var_wbin);
        let assign8820_e4283: f64 = (assign8820_e4279 + assign8820_e4282);
        let assign8820_e4286: f64 = (p.p801 / locals.var_lwbin);
        let assign8820_e4287: f64 = (assign8820_e4283 + assign8820_e4286);
        locals.var_uc_cgbo = assign8820_e4287;
        locals.var_uc_cgbo_rv = 0.0;

        let assign8830_e4291: f64 = (p.p626 / locals.var_lbin);
        let assign8830_e4292: f64 = (p.p86 + assign8830_e4291);
        let assign8830_e4295: f64 = (p.p714 / locals.var_wbin);
        let assign8830_e4296: f64 = (assign8830_e4292 + assign8830_e4295);
        let assign8830_e4299: f64 = (p.p802 / locals.var_lwbin);
        let assign8830_e4300: f64 = (assign8830_e4296 + assign8830_e4299);
        locals.var_uc_cvdsover = assign8830_e4300;
        locals.var_uc_cvdsover_rv = 0.0;

        let assign8850_e4317: f64 = (p.p628 / locals.var_lbin);
        let assign8850_e4318: f64 = (p.p76 + assign8850_e4317);
        let assign8850_e4321: f64 = (p.p716 / locals.var_wbin);
        let assign8850_e4322: f64 = (assign8850_e4318 + assign8850_e4321);
        let assign8850_e4325: f64 = (p.p804 / locals.var_lwbin);
        let assign8850_e4326: f64 = (assign8850_e4322 + assign8850_e4325);
        locals.var_uc_npext = assign8850_e4326;
        locals.var_uc_npext_rv = 0.0;

        let assign8860_e4330: f64 = (p.p629 / locals.var_lbin);
        let assign8860_e4331: f64 = (p.p81 + assign8860_e4330);
        let assign8860_e4334: f64 = (p.p717 / locals.var_wbin);
        let assign8860_e4335: f64 = (assign8860_e4331 + assign8860_e4334);
        let assign8860_e4338: f64 = (p.p805 / locals.var_lwbin);
        let assign8860_e4339: f64 = (assign8860_e4335 + assign8860_e4338);
        locals.var_uc_powrat = assign8860_e4339;
        locals.var_uc_powrat_rv = 0.0;

        let assign8870_e4343: f64 = (p.p630 / locals.var_lbin);
        let assign8870_e4344: f64 = (p.p74 + assign8870_e4343);
        let assign8870_e4347: f64 = (p.p718 / locals.var_wbin);
        let assign8870_e4348: f64 = (assign8870_e4344 + assign8870_e4347);
        let assign8870_e4351: f64 = (p.p806 / locals.var_lwbin);
        let assign8870_e4352: f64 = (assign8870_e4348 + assign8870_e4351);
        locals.var_uc_rd = assign8870_e4352;
        locals.var_uc_rd_rv = 0.0;

        let assign8880_e4356: f64 = (p.p631 / locals.var_lbin);
        let assign8880_e4357: f64 = (p.p298 + assign8880_e4356);
        let assign8880_e4360: f64 = (p.p719 / locals.var_wbin);
        let assign8880_e4361: f64 = (assign8880_e4357 + assign8880_e4360);
        let assign8880_e4364: f64 = (p.p807 / locals.var_lwbin);
        let assign8880_e4365: f64 = (assign8880_e4361 + assign8880_e4364);
        locals.var_uc_rd22 = assign8880_e4365;
        locals.var_uc_rd22_rv = 0.0;

        let assign8890_e4369: f64 = (p.p632 / locals.var_lbin);
        let assign8890_e4370: f64 = (p.p83 + assign8890_e4369);
        let assign8890_e4373: f64 = (p.p720 / locals.var_wbin);
        let assign8890_e4374: f64 = (assign8890_e4370 + assign8890_e4373);
        let assign8890_e4377: f64 = (p.p808 / locals.var_lwbin);
        let assign8890_e4378: f64 = (assign8890_e4374 + assign8890_e4377);
        locals.var_uc_rd23 = assign8890_e4378;
        locals.var_uc_rd23_rv = 0.0;

        let assign8900_e4382: f64 = (p.p633 / locals.var_lbin);
        let assign8900_e4383: f64 = (p.p84 + assign8900_e4382);
        let assign8900_e4386: f64 = (p.p721 / locals.var_wbin);
        let assign8900_e4387: f64 = (assign8900_e4383 + assign8900_e4386);
        let assign8900_e4390: f64 = (p.p809 / locals.var_lwbin);
        let assign8900_e4391: f64 = (assign8900_e4387 + assign8900_e4390);
        locals.var_uc_rd24 = assign8900_e4391;
        locals.var_uc_rd24_rv = 0.0;

        let assign8910_e4395: f64 = (p.p634 / locals.var_lbin);
        let assign8910_e4396: f64 = (p.p62 + assign8910_e4395);
        let assign8910_e4399: f64 = (p.p722 / locals.var_wbin);
        let assign8910_e4400: f64 = (assign8910_e4396 + assign8910_e4399);
        let assign8910_e4403: f64 = (p.p810 / locals.var_lwbin);
        let assign8910_e4404: f64 = (assign8910_e4400 + assign8910_e4403);
        locals.var_uc_rdict1 = assign8910_e4404;
        locals.var_uc_rdict1_rv = 0.0;

        let assign8920_e4408: f64 = (p.p635 / locals.var_lbin);
        let assign8920_e4409: f64 = (p.p59 + assign8920_e4408);
        let assign8920_e4412: f64 = (p.p723 / locals.var_wbin);
        let assign8920_e4413: f64 = (assign8920_e4409 + assign8920_e4412);
        let assign8920_e4416: f64 = (p.p811 / locals.var_lwbin);
        let assign8920_e4417: f64 = (assign8920_e4413 + assign8920_e4416);
        locals.var_uc_rdov13 = assign8920_e4417;
        locals.var_uc_rdov13_rv = 0.0;

        let assign8930_e4421: f64 = (p.p636 / locals.var_lbin);
        let assign8930_e4422: f64 = (p.p60 + assign8930_e4421);
        let assign8930_e4425: f64 = (p.p724 / locals.var_wbin);
        let assign8930_e4426: f64 = (assign8930_e4422 + assign8930_e4425);
        let assign8930_e4429: f64 = (p.p812 / locals.var_lwbin);
        let assign8930_e4430: f64 = (assign8930_e4426 + assign8930_e4429);
        locals.var_uc_rdslp1 = assign8930_e4430;
        locals.var_uc_rdslp1_rv = 0.0;

        let assign8940_e4434: f64 = (p.p637 / locals.var_lbin);
        let assign8940_e4435: f64 = (p.p85 + assign8940_e4434);
        let assign8940_e4438: f64 = (p.p725 / locals.var_wbin);
        let assign8940_e4439: f64 = (assign8940_e4435 + assign8940_e4438);
        let assign8940_e4442: f64 = (p.p813 / locals.var_lwbin);
        let assign8940_e4443: f64 = (assign8940_e4439 + assign8940_e4442);
        locals.var_uc_rdvb = assign8940_e4443;
        locals.var_uc_rdvb_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign8950_e4447: f64 = (p.p638 / locals.var_lbin);
        let assign8950_e4448: f64 = (p.p82 + assign8950_e4447);
        let assign8950_e4451: f64 = (p.p726 / locals.var_wbin);
        let assign8950_e4452: f64 = (assign8950_e4448 + assign8950_e4451);
        let assign8950_e4455: f64 = (p.p814 / locals.var_lwbin);
        let assign8950_e4456: f64 = (assign8950_e4452 + assign8950_e4455);
        locals.var_uc_rdvd = assign8950_e4456;
        locals.var_uc_rdvd_rv = 0.0;

        let assign8960_e4460: f64 = (p.p639 / locals.var_lbin);
        let assign8960_e4461: f64 = (p.p61 + assign8960_e4460);
        let assign8960_e4464: f64 = (p.p727 / locals.var_wbin);
        let assign8960_e4465: f64 = (assign8960_e4461 + assign8960_e4464);
        let assign8960_e4468: f64 = (p.p815 / locals.var_lwbin);
        let assign8960_e4469: f64 = (assign8960_e4465 + assign8960_e4468);
        locals.var_uc_rdvg11 = assign8960_e4469;
        locals.var_uc_rdvg11_rv = 0.0;

        let assign8970_e4473: f64 = (p.p640 / locals.var_lbin);
        let assign8970_e4474: f64 = (p.p75 + assign8970_e4473);
        let assign8970_e4477: f64 = (p.p728 / locals.var_wbin);
        let assign8970_e4478: f64 = (assign8970_e4474 + assign8970_e4477);
        let assign8970_e4481: f64 = (p.p816 / locals.var_lwbin);
        let assign8970_e4482: f64 = (assign8970_e4478 + assign8970_e4481);
        locals.var_uc_rs = assign8970_e4482;
        locals.var_uc_rs_rv = 0.0;

        let assign8980_e4486: f64 = (p.p641 / locals.var_lbin);
        let assign8980_e4487: f64 = (p.p80 + assign8980_e4486);
        let assign8980_e4490: f64 = (p.p729 / locals.var_wbin);
        let assign8980_e4491: f64 = (assign8980_e4487 + assign8980_e4490);
        let assign8980_e4494: f64 = (p.p817 / locals.var_lwbin);
        let assign8980_e4495: f64 = (assign8980_e4491 + assign8980_e4494);
        locals.var_uc_rth0 = assign8980_e4495;
        locals.var_uc_rth0_rv = 0.0;

        let assign8990_e4499: f64 = (p.p642 / locals.var_lbin);
        let assign8990_e4500: f64 = (p.p77 + assign8990_e4499);
        let assign8990_e4503: f64 = (p.p730 / locals.var_wbin);
        let assign8990_e4504: f64 = (assign8990_e4500 + assign8990_e4503);
        let assign8990_e4507: f64 = (p.p818 / locals.var_lwbin);
        let assign8990_e4508: f64 = (assign8990_e4504 + assign8990_e4507);
        locals.var_uc_vover = assign8990_e4508;
        locals.var_uc_vover_rv = 0.0;

        let assign9000_e4512: f64 = (p.p824 / locals.var_lbin);
        let assign9000_e4513: f64 = (p.p493 + assign9000_e4512);
        let assign9000_e4516: f64 = (p.p839 / locals.var_wbin);
        let assign9000_e4517: f64 = (assign9000_e4513 + assign9000_e4516);
        let assign9000_e4520: f64 = (p.p854 / locals.var_lwbin);
        let assign9000_e4521: f64 = (assign9000_e4517 + assign9000_e4520);
        locals.var_uc_js0d = assign9000_e4521;
        locals.var_uc_js0d_rv = 0.0;

        let assign9010_e4525: f64 = (p.p825 / locals.var_lbin);
        let assign9010_e4526: f64 = (p.p494 + assign9010_e4525);
        let assign9010_e4529: f64 = (p.p840 / locals.var_wbin);
        let assign9010_e4530: f64 = (assign9010_e4526 + assign9010_e4529);
        let assign9010_e4533: f64 = (p.p855 / locals.var_lwbin);
        let assign9010_e4534: f64 = (assign9010_e4530 + assign9010_e4533);
        locals.var_uc_js0swd = assign9010_e4534;
        locals.var_uc_js0swd_rv = 0.0;

        let assign9020_e4538: f64 = (p.p826 / locals.var_lbin);
        let assign9020_e4539: f64 = (p.p496 + assign9020_e4538);
        let assign9020_e4542: f64 = (p.p841 / locals.var_wbin);
        let assign9020_e4543: f64 = (assign9020_e4539 + assign9020_e4542);
        let assign9020_e4546: f64 = (p.p856 / locals.var_lwbin);
        let assign9020_e4547: f64 = (assign9020_e4543 + assign9020_e4546);
        locals.var_uc_njd = assign9020_e4547;
        locals.var_uc_njd_rv = 0.0;

        let assign9040_e4564: f64 = (p.p828 / locals.var_lbin);
        let assign9040_e4565: f64 = (p.p515 + assign9040_e4564);
        let assign9040_e4568: f64 = (p.p843 / locals.var_wbin);
        let assign9040_e4569: f64 = (assign9040_e4565 + assign9040_e4568);
        let assign9040_e4572: f64 = (p.p858 / locals.var_lwbin);
        let assign9040_e4573: f64 = (assign9040_e4569 + assign9040_e4572);
        locals.var_uc_vdiffjd = assign9040_e4573;
        locals.var_uc_vdiffjd_rv = 0.0;

        let assign9050_e4577: f64 = (p.p829 / locals.var_lbin);
        let assign9050_e4578: f64 = (p.p516 + assign9050_e4577);
        let assign9050_e4581: f64 = (p.p844 / locals.var_wbin);
        let assign9050_e4582: f64 = (assign9050_e4578 + assign9050_e4581);
        let assign9050_e4585: f64 = (p.p859 / locals.var_lwbin);
        let assign9050_e4586: f64 = (assign9050_e4582 + assign9050_e4585);
        locals.var_uc_js0s = assign9050_e4586;
        locals.var_uc_js0s_rv = 0.0;

        let assign9060_e4590: f64 = (p.p830 / locals.var_lbin);
        let assign9060_e4591: f64 = (p.p517 + assign9060_e4590);
        let assign9060_e4594: f64 = (p.p845 / locals.var_wbin);
        let assign9060_e4595: f64 = (assign9060_e4591 + assign9060_e4594);
        let assign9060_e4598: f64 = (p.p860 / locals.var_lwbin);
        let assign9060_e4599: f64 = (assign9060_e4595 + assign9060_e4598);
        locals.var_uc_js0sws = assign9060_e4599;
        locals.var_uc_js0sws_rv = 0.0;

        let assign9070_e4603: f64 = (p.p831 / locals.var_lbin);
        let assign9070_e4604: f64 = (p.p519 + assign9070_e4603);
        let assign9070_e4607: f64 = (p.p846 / locals.var_wbin);
        let assign9070_e4608: f64 = (assign9070_e4604 + assign9070_e4607);
        let assign9070_e4611: f64 = (p.p861 / locals.var_lwbin);
        let assign9070_e4612: f64 = (assign9070_e4608 + assign9070_e4611);
        locals.var_uc_njs = assign9070_e4612;
        locals.var_uc_njs_rv = 0.0;

        let assign9090_e4629: f64 = (p.p833 / locals.var_lbin);
        let assign9090_e4630: f64 = (p.p538 + assign9090_e4629);
        let assign9090_e4633: f64 = (p.p848 / locals.var_wbin);
        let assign9090_e4634: f64 = (assign9090_e4630 + assign9090_e4633);
        let assign9090_e4637: f64 = (p.p863 / locals.var_lwbin);
        let assign9090_e4638: f64 = (assign9090_e4634 + assign9090_e4637);
        locals.var_uc_vdiffjs = assign9090_e4638;
        locals.var_uc_vdiffjs_rv = 0.0;

        let assign9190_e4689: f64 = if locals.var_uc_codep != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard185 = assign9190_e4689;
        locals.var_guard185_rv = 0.0;

        let (assign9200_e4695, assign9200_e4695_d_n0, assign9200_e4695_d_n2, assign9200_e4695_d_n4, assign9200_e4695_d_n5, assign9200_e4695_d_n6, assign9200_e4695_d_n7, assign9200_e4695_d_n8, assign9200_e4695_d_n9, assign9200_e4695_d_n10, assign9200_e4695_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9200_e4693: f64 = (locals.var_lg).powf(p.p342);
        (assign9200_e4693, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9200_e4695;
        locals.var_t3_dn0 = assign9200_e4695_d_n0;
        locals.var_t3_dn2 = assign9200_e4695_d_n2;
        locals.var_t3_dn4 = assign9200_e4695_d_n4;
        locals.var_t3_dn5 = assign9200_e4695_d_n5;
        locals.var_t3_dn6 = assign9200_e4695_d_n6;
        locals.var_t3_dn7 = assign9200_e4695_d_n7;
        locals.var_t3_dn8 = assign9200_e4695_d_n8;
        locals.var_t3_dn9 = assign9200_e4695_d_n9;
        locals.var_t3_dn10 = assign9200_e4695_d_n10;
        locals.var_t3_dn13 = assign9200_e4695_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign9210_e4705, assign9210_e4705_d_n0, assign9210_e4705_d_n2, assign9210_e4705_d_n4, assign9210_e4705_d_n5, assign9210_e4705_d_n6, assign9210_e4705_d_n7, assign9210_e4705_d_n8, assign9210_e4705_d_n9, assign9210_e4705_d_n10, assign9210_e4705_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9210_e4701: f64 = (p.p341 / locals.var_t3);
        let assign9210_e4702: f64 = (1.0 + assign9210_e4701);
        let assign9210_e4703: f64 = (locals.var_uc_ndepm * assign9210_e4702);
        (assign9210_e4703, ((locals.var_uc_ndepm_dn0 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn2 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn4 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn5 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn6 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn7 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn8 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn9 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn10 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn13 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn13,)
    }
};
        locals.var_uc_ndepm = assign9210_e4705;
        locals.var_uc_ndepm_dn0 = assign9210_e4705_d_n0;
        locals.var_uc_ndepm_dn2 = assign9210_e4705_d_n2;
        locals.var_uc_ndepm_dn4 = assign9210_e4705_d_n4;
        locals.var_uc_ndepm_dn5 = assign9210_e4705_d_n5;
        locals.var_uc_ndepm_dn6 = assign9210_e4705_d_n6;
        locals.var_uc_ndepm_dn7 = assign9210_e4705_d_n7;
        locals.var_uc_ndepm_dn8 = assign9210_e4705_d_n8;
        locals.var_uc_ndepm_dn9 = assign9210_e4705_d_n9;
        locals.var_uc_ndepm_dn10 = assign9210_e4705_d_n10;
        locals.var_uc_ndepm_dn13 = assign9210_e4705_d_n13;
        locals.var_uc_ndepm_rv = 0.0;

        let assign9220_e4708: f64 = if locals.var_uc_ndepm < 1e21 { 1.0 } else { 0.0 };
        locals.var_guard186 = assign9220_e4708;
        locals.var_guard186_rv = 0.0;

        let (assign9230_e4714, assign9230_e4714_d_n0, assign9230_e4714_d_n2, assign9230_e4714_d_n4, assign9230_e4714_d_n5, assign9230_e4714_d_n6, assign9230_e4714_d_n7, assign9230_e4714_d_n8, assign9230_e4714_d_n9, assign9230_e4714_d_n10, assign9230_e4714_d_n13,) = {
    if ((locals.var_guard185 != 0.0) && (locals.var_guard186 != 0.0)) {
        (1e21, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn13,)
    }
};
        locals.var_uc_ndepm = assign9230_e4714;
        locals.var_uc_ndepm_dn0 = assign9230_e4714_d_n0;
        locals.var_uc_ndepm_dn2 = assign9230_e4714_d_n2;
        locals.var_uc_ndepm_dn4 = assign9230_e4714_d_n4;
        locals.var_uc_ndepm_dn5 = assign9230_e4714_d_n5;
        locals.var_uc_ndepm_dn6 = assign9230_e4714_d_n6;
        locals.var_uc_ndepm_dn7 = assign9230_e4714_d_n7;
        locals.var_uc_ndepm_dn8 = assign9230_e4714_d_n8;
        locals.var_uc_ndepm_dn9 = assign9230_e4714_d_n9;
        locals.var_uc_ndepm_dn10 = assign9230_e4714_d_n10;
        locals.var_uc_ndepm_dn13 = assign9230_e4714_d_n13;
        locals.var_uc_ndepm_rv = 0.0;

        let (assign9240_e4720, assign9240_e4720_d_n0, assign9240_e4720_d_n2, assign9240_e4720_d_n4, assign9240_e4720_d_n5, assign9240_e4720_d_n6, assign9240_e4720_d_n7, assign9240_e4720_d_n8, assign9240_e4720_d_n9, assign9240_e4720_d_n10, assign9240_e4720_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9240_e4718: f64 = (locals.var_lg).powf(p.p369);
        (assign9240_e4718, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9240_e4720;
        locals.var_t3_dn0 = assign9240_e4720_d_n0;
        locals.var_t3_dn2 = assign9240_e4720_d_n2;
        locals.var_t3_dn4 = assign9240_e4720_d_n4;
        locals.var_t3_dn5 = assign9240_e4720_d_n5;
        locals.var_t3_dn6 = assign9240_e4720_d_n6;
        locals.var_t3_dn7 = assign9240_e4720_d_n7;
        locals.var_t3_dn8 = assign9240_e4720_d_n8;
        locals.var_t3_dn9 = assign9240_e4720_d_n9;
        locals.var_t3_dn10 = assign9240_e4720_d_n10;
        locals.var_t3_dn13 = assign9240_e4720_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign9250_e4730, assign9250_e4730_d_n0, assign9250_e4730_d_n2, assign9250_e4730_d_n4, assign9250_e4730_d_n5, assign9250_e4730_d_n6, assign9250_e4730_d_n7, assign9250_e4730_d_n8, assign9250_e4730_d_n9, assign9250_e4730_d_n10, assign9250_e4730_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9250_e4726: f64 = (p.p368 / locals.var_t3);
        let assign9250_e4727: f64 = (1.0 + assign9250_e4726);
        let assign9250_e4728: f64 = (locals.var_uc_depvmax * assign9250_e4727);
        (assign9250_e4728, ((locals.var_uc_depvmax_dn0 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn2 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn4 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn5 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn6 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn7 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn8 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn9 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn10 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn13 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign9250_e4730;
        locals.var_uc_depvmax_dn0 = assign9250_e4730_d_n0;
        locals.var_uc_depvmax_dn2 = assign9250_e4730_d_n2;
        locals.var_uc_depvmax_dn4 = assign9250_e4730_d_n4;
        locals.var_uc_depvmax_dn5 = assign9250_e4730_d_n5;
        locals.var_uc_depvmax_dn6 = assign9250_e4730_d_n6;
        locals.var_uc_depvmax_dn7 = assign9250_e4730_d_n7;
        locals.var_uc_depvmax_dn8 = assign9250_e4730_d_n8;
        locals.var_uc_depvmax_dn9 = assign9250_e4730_d_n9;
        locals.var_uc_depvmax_dn10 = assign9250_e4730_d_n10;
        locals.var_uc_depvmax_dn13 = assign9250_e4730_d_n13;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign9260_e4736, assign9260_e4736_d_n0, assign9260_e4736_d_n2, assign9260_e4736_d_n4, assign9260_e4736_d_n5, assign9260_e4736_d_n6, assign9260_e4736_d_n7, assign9260_e4736_d_n8, assign9260_e4736_d_n9, assign9260_e4736_d_n10, assign9260_e4736_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9260_e4734: f64 = (locals.var_lg).powf(p.p362);
        (assign9260_e4734, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9260_e4736;
        locals.var_t3_dn0 = assign9260_e4736_d_n0;
        locals.var_t3_dn2 = assign9260_e4736_d_n2;
        locals.var_t3_dn4 = assign9260_e4736_d_n4;
        locals.var_t3_dn5 = assign9260_e4736_d_n5;
        locals.var_t3_dn6 = assign9260_e4736_d_n6;
        locals.var_t3_dn7 = assign9260_e4736_d_n7;
        locals.var_t3_dn8 = assign9260_e4736_d_n8;
        locals.var_t3_dn9 = assign9260_e4736_d_n9;
        locals.var_t3_dn10 = assign9260_e4736_d_n10;
        locals.var_t3_dn13 = assign9260_e4736_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign9270_e4746, assign9270_e4746_d_n0, assign9270_e4746_d_n2, assign9270_e4746_d_n4, assign9270_e4746_d_n5, assign9270_e4746_d_n6, assign9270_e4746_d_n7, assign9270_e4746_d_n8, assign9270_e4746_d_n9, assign9270_e4746_d_n10, assign9270_e4746_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9270_e4742: f64 = (p.p361 / locals.var_t3);
        let assign9270_e4743: f64 = (1.0 + assign9270_e4742);
        let assign9270_e4744: f64 = (p.p360 * assign9270_e4743);
        (assign9270_e4744, (p.p360 * (-((p.p361 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn13,)
    }
};
        locals.var_uc_depleak = assign9270_e4746;
        locals.var_uc_depleak_dn0 = assign9270_e4746_d_n0;
        locals.var_uc_depleak_dn2 = assign9270_e4746_d_n2;
        locals.var_uc_depleak_dn4 = assign9270_e4746_d_n4;
        locals.var_uc_depleak_dn5 = assign9270_e4746_d_n5;
        locals.var_uc_depleak_dn6 = assign9270_e4746_d_n6;
        locals.var_uc_depleak_dn7 = assign9270_e4746_d_n7;
        locals.var_uc_depleak_dn8 = assign9270_e4746_d_n8;
        locals.var_uc_depleak_dn9 = assign9270_e4746_d_n9;
        locals.var_uc_depleak_dn10 = assign9270_e4746_d_n10;
        locals.var_uc_depleak_dn13 = assign9270_e4746_d_n13;
        locals.var_uc_depleak_rv = 0.0;

        let assign9280_e4749: f64 = if locals.var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard187 = assign9280_e4749;
        locals.var_guard187_rv = 0.0;

        let (assign9290_e4755, assign9290_e4755_d_n0, assign9290_e4755_d_n2, assign9290_e4755_d_n4, assign9290_e4755_d_n5, assign9290_e4755_d_n6, assign9290_e4755_d_n7, assign9290_e4755_d_n8, assign9290_e4755_d_n9, assign9290_e4755_d_n10, assign9290_e4755_d_n13,) = {
    if ((locals.var_guard185 != 0.0) && (locals.var_guard187 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn13,)
    }
};
        locals.var_uc_depleak = assign9290_e4755;
        locals.var_uc_depleak_dn0 = assign9290_e4755_d_n0;
        locals.var_uc_depleak_dn2 = assign9290_e4755_d_n2;
        locals.var_uc_depleak_dn4 = assign9290_e4755_d_n4;
        locals.var_uc_depleak_dn5 = assign9290_e4755_d_n5;
        locals.var_uc_depleak_dn6 = assign9290_e4755_d_n6;
        locals.var_uc_depleak_dn7 = assign9290_e4755_d_n7;
        locals.var_uc_depleak_dn8 = assign9290_e4755_d_n8;
        locals.var_uc_depleak_dn9 = assign9290_e4755_d_n9;
        locals.var_uc_depleak_dn10 = assign9290_e4755_d_n10;
        locals.var_uc_depleak_dn13 = assign9290_e4755_d_n13;
        locals.var_uc_depleak_rv = 0.0;

        let (assign9300_e4761, assign9300_e4761_d_n0, assign9300_e4761_d_n2, assign9300_e4761_d_n4, assign9300_e4761_d_n5, assign9300_e4761_d_n6, assign9300_e4761_d_n7, assign9300_e4761_d_n8, assign9300_e4761_d_n9, assign9300_e4761_d_n10, assign9300_e4761_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9300_e4759: f64 = (locals.var_lg).powf(p.p348);
        (assign9300_e4759, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9300_e4761;
        locals.var_t3_dn0 = assign9300_e4761_d_n0;
        locals.var_t3_dn2 = assign9300_e4761_d_n2;
        locals.var_t3_dn4 = assign9300_e4761_d_n4;
        locals.var_t3_dn5 = assign9300_e4761_d_n5;
        locals.var_t3_dn6 = assign9300_e4761_d_n6;
        locals.var_t3_dn7 = assign9300_e4761_d_n7;
        locals.var_t3_dn8 = assign9300_e4761_d_n8;
        locals.var_t3_dn9 = assign9300_e4761_d_n9;
        locals.var_t3_dn10 = assign9300_e4761_d_n10;
        locals.var_t3_dn13 = assign9300_e4761_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign9310_e4771, assign9310_e4771_d_n0, assign9310_e4771_d_n2, assign9310_e4771_d_n4, assign9310_e4771_d_n5, assign9310_e4771_d_n6, assign9310_e4771_d_n7, assign9310_e4771_d_n8, assign9310_e4771_d_n9, assign9310_e4771_d_n10, assign9310_e4771_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9310_e4767: f64 = (p.p347 / locals.var_t3);
        let assign9310_e4768: f64 = (1.0 + assign9310_e4767);
        let assign9310_e4769: f64 = (p.p346 * assign9310_e4768);
        (assign9310_e4769, (p.p346 * (-((p.p347 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign9310_e4771;
        locals.var_uc_depmue0_dn0 = assign9310_e4771_d_n0;
        locals.var_uc_depmue0_dn2 = assign9310_e4771_d_n2;
        locals.var_uc_depmue0_dn4 = assign9310_e4771_d_n4;
        locals.var_uc_depmue0_dn5 = assign9310_e4771_d_n5;
        locals.var_uc_depmue0_dn6 = assign9310_e4771_d_n6;
        locals.var_uc_depmue0_dn7 = assign9310_e4771_d_n7;
        locals.var_uc_depmue0_dn8 = assign9310_e4771_d_n8;
        locals.var_uc_depmue0_dn9 = assign9310_e4771_d_n9;
        locals.var_uc_depmue0_dn10 = assign9310_e4771_d_n10;
        locals.var_uc_depmue0_dn13 = assign9310_e4771_d_n13;
        locals.var_uc_depmue0_rv = 0.0;

        let assign9320_e4774: f64 = if locals.var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard188 = assign9320_e4774;
        locals.var_guard188_rv = 0.0;

        let (assign9330_e4780, assign9330_e4780_d_n0, assign9330_e4780_d_n2, assign9330_e4780_d_n4, assign9330_e4780_d_n5, assign9330_e4780_d_n6, assign9330_e4780_d_n7, assign9330_e4780_d_n8, assign9330_e4780_d_n9, assign9330_e4780_d_n10, assign9330_e4780_d_n13,) = {
    if ((locals.var_guard185 != 0.0) && (locals.var_guard188 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign9330_e4780;
        locals.var_uc_depmue0_dn0 = assign9330_e4780_d_n0;
        locals.var_uc_depmue0_dn2 = assign9330_e4780_d_n2;
        locals.var_uc_depmue0_dn4 = assign9330_e4780_d_n4;
        locals.var_uc_depmue0_dn5 = assign9330_e4780_d_n5;
        locals.var_uc_depmue0_dn6 = assign9330_e4780_d_n6;
        locals.var_uc_depmue0_dn7 = assign9330_e4780_d_n7;
        locals.var_uc_depmue0_dn8 = assign9330_e4780_d_n8;
        locals.var_uc_depmue0_dn9 = assign9330_e4780_d_n9;
        locals.var_uc_depmue0_dn10 = assign9330_e4780_d_n10;
        locals.var_uc_depmue0_dn13 = assign9330_e4780_d_n13;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign9340_e4786, assign9340_e4786_d_n0, assign9340_e4786_d_n2, assign9340_e4786_d_n4, assign9340_e4786_d_n5, assign9340_e4786_d_n6, assign9340_e4786_d_n7, assign9340_e4786_d_n8, assign9340_e4786_d_n9, assign9340_e4786_d_n10, assign9340_e4786_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9340_e4784: f64 = (locals.var_lg).powf(p.p351);
        (assign9340_e4784, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9340_e4786;
        locals.var_t3_dn0 = assign9340_e4786_d_n0;
        locals.var_t3_dn2 = assign9340_e4786_d_n2;
        locals.var_t3_dn4 = assign9340_e4786_d_n4;
        locals.var_t3_dn5 = assign9340_e4786_d_n5;
        locals.var_t3_dn6 = assign9340_e4786_d_n6;
        locals.var_t3_dn7 = assign9340_e4786_d_n7;
        locals.var_t3_dn8 = assign9340_e4786_d_n8;
        locals.var_t3_dn9 = assign9340_e4786_d_n9;
        locals.var_t3_dn10 = assign9340_e4786_d_n10;
        locals.var_t3_dn13 = assign9340_e4786_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign9350_e4796, assign9350_e4796_d_n0, assign9350_e4796_d_n2, assign9350_e4796_d_n4, assign9350_e4796_d_n5, assign9350_e4796_d_n6, assign9350_e4796_d_n7, assign9350_e4796_d_n8, assign9350_e4796_d_n9, assign9350_e4796_d_n10, assign9350_e4796_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9350_e4792: f64 = (p.p350 / locals.var_t3);
        let assign9350_e4793: f64 = (1.0 + assign9350_e4792);
        let assign9350_e4794: f64 = (p.p349 * assign9350_e4793);
        (assign9350_e4794, (p.p349 * (-((p.p350 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmue1, locals.var_uc_depmue1_dn0, locals.var_uc_depmue1_dn2, locals.var_uc_depmue1_dn4, locals.var_uc_depmue1_dn5, locals.var_uc_depmue1_dn6, locals.var_uc_depmue1_dn7, locals.var_uc_depmue1_dn8, locals.var_uc_depmue1_dn9, locals.var_uc_depmue1_dn10, locals.var_uc_depmue1_dn13,)
    }
};
        locals.var_uc_depmue1 = assign9350_e4796;
        locals.var_uc_depmue1_dn0 = assign9350_e4796_d_n0;
        locals.var_uc_depmue1_dn2 = assign9350_e4796_d_n2;
        locals.var_uc_depmue1_dn4 = assign9350_e4796_d_n4;
        locals.var_uc_depmue1_dn5 = assign9350_e4796_d_n5;
        locals.var_uc_depmue1_dn6 = assign9350_e4796_d_n6;
        locals.var_uc_depmue1_dn7 = assign9350_e4796_d_n7;
        locals.var_uc_depmue1_dn8 = assign9350_e4796_d_n8;
        locals.var_uc_depmue1_dn9 = assign9350_e4796_d_n9;
        locals.var_uc_depmue1_dn10 = assign9350_e4796_d_n10;
        locals.var_uc_depmue1_dn13 = assign9350_e4796_d_n13;
        locals.var_uc_depmue1_rv = 0.0;

        let assign9360_e4799: f64 = if locals.var_uc_depmue1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard189 = assign9360_e4799;
        locals.var_guard189_rv = 0.0;

        let (assign9370_e4805, assign9370_e4805_d_n0, assign9370_e4805_d_n2, assign9370_e4805_d_n4, assign9370_e4805_d_n5, assign9370_e4805_d_n6, assign9370_e4805_d_n7, assign9370_e4805_d_n8, assign9370_e4805_d_n9, assign9370_e4805_d_n10, assign9370_e4805_d_n13,) = {
    if ((locals.var_guard185 != 0.0) && (locals.var_guard189 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue1, locals.var_uc_depmue1_dn0, locals.var_uc_depmue1_dn2, locals.var_uc_depmue1_dn4, locals.var_uc_depmue1_dn5, locals.var_uc_depmue1_dn6, locals.var_uc_depmue1_dn7, locals.var_uc_depmue1_dn8, locals.var_uc_depmue1_dn9, locals.var_uc_depmue1_dn10, locals.var_uc_depmue1_dn13,)
    }
};
        locals.var_uc_depmue1 = assign9370_e4805;
        locals.var_uc_depmue1_dn0 = assign9370_e4805_d_n0;
        locals.var_uc_depmue1_dn2 = assign9370_e4805_d_n2;
        locals.var_uc_depmue1_dn4 = assign9370_e4805_d_n4;
        locals.var_uc_depmue1_dn5 = assign9370_e4805_d_n5;
        locals.var_uc_depmue1_dn6 = assign9370_e4805_d_n6;
        locals.var_uc_depmue1_dn7 = assign9370_e4805_d_n7;
        locals.var_uc_depmue1_dn8 = assign9370_e4805_d_n8;
        locals.var_uc_depmue1_dn9 = assign9370_e4805_d_n9;
        locals.var_uc_depmue1_dn10 = assign9370_e4805_d_n10;
        locals.var_uc_depmue1_dn13 = assign9370_e4805_d_n13;
        locals.var_uc_depmue1_rv = 0.0;

        let (assign9380_e4811, assign9380_e4811_d_n0, assign9380_e4811_d_n2, assign9380_e4811_d_n4, assign9380_e4811_d_n5, assign9380_e4811_d_n6, assign9380_e4811_d_n7, assign9380_e4811_d_n8, assign9380_e4811_d_n9, assign9380_e4811_d_n10, assign9380_e4811_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9380_e4809: f64 = (locals.var_lg).powf(p.p357);
        (assign9380_e4809, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9380_e4811;
        locals.var_t3_dn0 = assign9380_e4811_d_n0;
        locals.var_t3_dn2 = assign9380_e4811_d_n2;
        locals.var_t3_dn4 = assign9380_e4811_d_n4;
        locals.var_t3_dn5 = assign9380_e4811_d_n5;
        locals.var_t3_dn6 = assign9380_e4811_d_n6;
        locals.var_t3_dn7 = assign9380_e4811_d_n7;
        locals.var_t3_dn8 = assign9380_e4811_d_n8;
        locals.var_t3_dn9 = assign9380_e4811_d_n9;
        locals.var_t3_dn10 = assign9380_e4811_d_n10;
        locals.var_t3_dn13 = assign9380_e4811_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign9390_e4821, assign9390_e4821_d_n0, assign9390_e4821_d_n2, assign9390_e4821_d_n4, assign9390_e4821_d_n5, assign9390_e4821_d_n6, assign9390_e4821_d_n7, assign9390_e4821_d_n8, assign9390_e4821_d_n9, assign9390_e4821_d_n10, assign9390_e4821_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9390_e4817: f64 = (p.p356 / locals.var_t3);
        let assign9390_e4818: f64 = (1.0 + assign9390_e4817);
        let assign9390_e4819: f64 = (p.p354 * assign9390_e4818);
        (assign9390_e4819, (p.p354 * (-((p.p356 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn13,)
    }
};
        locals.var_uc_depmueback0 = assign9390_e4821;
        locals.var_uc_depmueback0_dn0 = assign9390_e4821_d_n0;
        locals.var_uc_depmueback0_dn2 = assign9390_e4821_d_n2;
        locals.var_uc_depmueback0_dn4 = assign9390_e4821_d_n4;
        locals.var_uc_depmueback0_dn5 = assign9390_e4821_d_n5;
        locals.var_uc_depmueback0_dn6 = assign9390_e4821_d_n6;
        locals.var_uc_depmueback0_dn7 = assign9390_e4821_d_n7;
        locals.var_uc_depmueback0_dn8 = assign9390_e4821_d_n8;
        locals.var_uc_depmueback0_dn9 = assign9390_e4821_d_n9;
        locals.var_uc_depmueback0_dn10 = assign9390_e4821_d_n10;
        locals.var_uc_depmueback0_dn13 = assign9390_e4821_d_n13;
        locals.var_uc_depmueback0_rv = 0.0;

        let assign9400_e4824: f64 = if locals.var_uc_depmueback0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard190 = assign9400_e4824;
        locals.var_guard190_rv = 0.0;

        let (assign9410_e4830, assign9410_e4830_d_n0, assign9410_e4830_d_n2, assign9410_e4830_d_n4, assign9410_e4830_d_n5, assign9410_e4830_d_n6, assign9410_e4830_d_n7, assign9410_e4830_d_n8, assign9410_e4830_d_n9, assign9410_e4830_d_n10, assign9410_e4830_d_n13,) = {
    if ((locals.var_guard185 != 0.0) && (locals.var_guard190 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn13,)
    }
};
        locals.var_uc_depmueback0 = assign9410_e4830;
        locals.var_uc_depmueback0_dn0 = assign9410_e4830_d_n0;
        locals.var_uc_depmueback0_dn2 = assign9410_e4830_d_n2;
        locals.var_uc_depmueback0_dn4 = assign9410_e4830_d_n4;
        locals.var_uc_depmueback0_dn5 = assign9410_e4830_d_n5;
        locals.var_uc_depmueback0_dn6 = assign9410_e4830_d_n6;
        locals.var_uc_depmueback0_dn7 = assign9410_e4830_d_n7;
        locals.var_uc_depmueback0_dn8 = assign9410_e4830_d_n8;
        locals.var_uc_depmueback0_dn9 = assign9410_e4830_d_n9;
        locals.var_uc_depmueback0_dn10 = assign9410_e4830_d_n10;
        locals.var_uc_depmueback0_dn13 = assign9410_e4830_d_n13;
        locals.var_uc_depmueback0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9420_e4836, assign9420_e4836_d_n0, assign9420_e4836_d_n2, assign9420_e4836_d_n4, assign9420_e4836_d_n5, assign9420_e4836_d_n6, assign9420_e4836_d_n7, assign9420_e4836_d_n8, assign9420_e4836_d_n9, assign9420_e4836_d_n10, assign9420_e4836_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9420_e4834: f64 = (locals.var_lg).powf(p.p359);
        (assign9420_e4834, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9420_e4836;
        locals.var_t3_dn0 = assign9420_e4836_d_n0;
        locals.var_t3_dn2 = assign9420_e4836_d_n2;
        locals.var_t3_dn4 = assign9420_e4836_d_n4;
        locals.var_t3_dn5 = assign9420_e4836_d_n5;
        locals.var_t3_dn6 = assign9420_e4836_d_n6;
        locals.var_t3_dn7 = assign9420_e4836_d_n7;
        locals.var_t3_dn8 = assign9420_e4836_d_n8;
        locals.var_t3_dn9 = assign9420_e4836_d_n9;
        locals.var_t3_dn10 = assign9420_e4836_d_n10;
        locals.var_t3_dn13 = assign9420_e4836_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign9430_e4846, assign9430_e4846_d_n0, assign9430_e4846_d_n2, assign9430_e4846_d_n4, assign9430_e4846_d_n5, assign9430_e4846_d_n6, assign9430_e4846_d_n7, assign9430_e4846_d_n8, assign9430_e4846_d_n9, assign9430_e4846_d_n10, assign9430_e4846_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9430_e4842: f64 = (p.p358 / locals.var_t3);
        let assign9430_e4843: f64 = (1.0 + assign9430_e4842);
        let assign9430_e4844: f64 = (p.p355 * assign9430_e4843);
        (assign9430_e4844, (p.p355 * (-((p.p358 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmueback1, locals.var_uc_depmueback1_dn0, locals.var_uc_depmueback1_dn2, locals.var_uc_depmueback1_dn4, locals.var_uc_depmueback1_dn5, locals.var_uc_depmueback1_dn6, locals.var_uc_depmueback1_dn7, locals.var_uc_depmueback1_dn8, locals.var_uc_depmueback1_dn9, locals.var_uc_depmueback1_dn10, locals.var_uc_depmueback1_dn13,)
    }
};
        locals.var_uc_depmueback1 = assign9430_e4846;
        locals.var_uc_depmueback1_dn0 = assign9430_e4846_d_n0;
        locals.var_uc_depmueback1_dn2 = assign9430_e4846_d_n2;
        locals.var_uc_depmueback1_dn4 = assign9430_e4846_d_n4;
        locals.var_uc_depmueback1_dn5 = assign9430_e4846_d_n5;
        locals.var_uc_depmueback1_dn6 = assign9430_e4846_d_n6;
        locals.var_uc_depmueback1_dn7 = assign9430_e4846_d_n7;
        locals.var_uc_depmueback1_dn8 = assign9430_e4846_d_n8;
        locals.var_uc_depmueback1_dn9 = assign9430_e4846_d_n9;
        locals.var_uc_depmueback1_dn10 = assign9430_e4846_d_n10;
        locals.var_uc_depmueback1_dn13 = assign9430_e4846_d_n13;
        locals.var_uc_depmueback1_rv = 0.0;

        let assign9440_e4849: f64 = if locals.var_uc_depmueback1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard191 = assign9440_e4849;
        locals.var_guard191_rv = 0.0;

        let (assign9450_e4855, assign9450_e4855_d_n0, assign9450_e4855_d_n2, assign9450_e4855_d_n4, assign9450_e4855_d_n5, assign9450_e4855_d_n6, assign9450_e4855_d_n7, assign9450_e4855_d_n8, assign9450_e4855_d_n9, assign9450_e4855_d_n10, assign9450_e4855_d_n13,) = {
    if ((locals.var_guard185 != 0.0) && (locals.var_guard191 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback1, locals.var_uc_depmueback1_dn0, locals.var_uc_depmueback1_dn2, locals.var_uc_depmueback1_dn4, locals.var_uc_depmueback1_dn5, locals.var_uc_depmueback1_dn6, locals.var_uc_depmueback1_dn7, locals.var_uc_depmueback1_dn8, locals.var_uc_depmueback1_dn9, locals.var_uc_depmueback1_dn10, locals.var_uc_depmueback1_dn13,)
    }
};
        locals.var_uc_depmueback1 = assign9450_e4855;
        locals.var_uc_depmueback1_dn0 = assign9450_e4855_d_n0;
        locals.var_uc_depmueback1_dn2 = assign9450_e4855_d_n2;
        locals.var_uc_depmueback1_dn4 = assign9450_e4855_d_n4;
        locals.var_uc_depmueback1_dn5 = assign9450_e4855_d_n5;
        locals.var_uc_depmueback1_dn6 = assign9450_e4855_d_n6;
        locals.var_uc_depmueback1_dn7 = assign9450_e4855_d_n7;
        locals.var_uc_depmueback1_dn8 = assign9450_e4855_d_n8;
        locals.var_uc_depmueback1_dn9 = assign9450_e4855_d_n9;
        locals.var_uc_depmueback1_dn10 = assign9450_e4855_d_n10;
        locals.var_uc_depmueback1_dn13 = assign9450_e4855_d_n13;
        locals.var_uc_depmueback1_rv = 0.0;

        let (assign9460_e4861, assign9460_e4861_d_n0, assign9460_e4861_d_n2, assign9460_e4861_d_n4, assign9460_e4861_d_n5, assign9460_e4861_d_n6, assign9460_e4861_d_n7, assign9460_e4861_d_n8, assign9460_e4861_d_n9, assign9460_e4861_d_n10, assign9460_e4861_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9460_e4859: f64 = (locals.var_lg).powf(p.p373);
        (assign9460_e4859, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9460_e4861;
        locals.var_t3_dn0 = assign9460_e4861_d_n0;
        locals.var_t3_dn2 = assign9460_e4861_d_n2;
        locals.var_t3_dn4 = assign9460_e4861_d_n4;
        locals.var_t3_dn5 = assign9460_e4861_d_n5;
        locals.var_t3_dn6 = assign9460_e4861_d_n6;
        locals.var_t3_dn7 = assign9460_e4861_d_n7;
        locals.var_t3_dn8 = assign9460_e4861_d_n8;
        locals.var_t3_dn9 = assign9460_e4861_d_n9;
        locals.var_t3_dn10 = assign9460_e4861_d_n10;
        locals.var_t3_dn13 = assign9460_e4861_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign9470_e4871, assign9470_e4871_d_n0, assign9470_e4871_d_n2, assign9470_e4871_d_n4, assign9470_e4871_d_n5, assign9470_e4871_d_n6, assign9470_e4871_d_n7, assign9470_e4871_d_n8, assign9470_e4871_d_n9, assign9470_e4871_d_n10, assign9470_e4871_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9470_e4867: f64 = (p.p372 / locals.var_t3);
        let assign9470_e4868: f64 = (1.0 + assign9470_e4867);
        let assign9470_e4869: f64 = (locals.var_uc_depvdsef1 * assign9470_e4868);
        (assign9470_e4869, ((locals.var_uc_depvdsef1_dn0 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn2 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn4 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn5 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn6 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn7 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn8 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn9 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn10 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn13 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_depvdsef1, locals.var_uc_depvdsef1_dn0, locals.var_uc_depvdsef1_dn2, locals.var_uc_depvdsef1_dn4, locals.var_uc_depvdsef1_dn5, locals.var_uc_depvdsef1_dn6, locals.var_uc_depvdsef1_dn7, locals.var_uc_depvdsef1_dn8, locals.var_uc_depvdsef1_dn9, locals.var_uc_depvdsef1_dn10, locals.var_uc_depvdsef1_dn13,)
    }
};
        locals.var_uc_depvdsef1 = assign9470_e4871;
        locals.var_uc_depvdsef1_dn0 = assign9470_e4871_d_n0;
        locals.var_uc_depvdsef1_dn2 = assign9470_e4871_d_n2;
        locals.var_uc_depvdsef1_dn4 = assign9470_e4871_d_n4;
        locals.var_uc_depvdsef1_dn5 = assign9470_e4871_d_n5;
        locals.var_uc_depvdsef1_dn6 = assign9470_e4871_d_n6;
        locals.var_uc_depvdsef1_dn7 = assign9470_e4871_d_n7;
        locals.var_uc_depvdsef1_dn8 = assign9470_e4871_d_n8;
        locals.var_uc_depvdsef1_dn9 = assign9470_e4871_d_n9;
        locals.var_uc_depvdsef1_dn10 = assign9470_e4871_d_n10;
        locals.var_uc_depvdsef1_dn13 = assign9470_e4871_d_n13;
        locals.var_uc_depvdsef1_rv = 0.0;

        let (assign9480_e4877, assign9480_e4877_d_n0, assign9480_e4877_d_n2, assign9480_e4877_d_n4, assign9480_e4877_d_n5, assign9480_e4877_d_n6, assign9480_e4877_d_n7, assign9480_e4877_d_n8, assign9480_e4877_d_n9, assign9480_e4877_d_n10, assign9480_e4877_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9480_e4875: f64 = (locals.var_lg).powf(p.p375);
        (assign9480_e4875, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9480_e4877;
        locals.var_t3_dn0 = assign9480_e4877_d_n0;
        locals.var_t3_dn2 = assign9480_e4877_d_n2;
        locals.var_t3_dn4 = assign9480_e4877_d_n4;
        locals.var_t3_dn5 = assign9480_e4877_d_n5;
        locals.var_t3_dn6 = assign9480_e4877_d_n6;
        locals.var_t3_dn7 = assign9480_e4877_d_n7;
        locals.var_t3_dn8 = assign9480_e4877_d_n8;
        locals.var_t3_dn9 = assign9480_e4877_d_n9;
        locals.var_t3_dn10 = assign9480_e4877_d_n10;
        locals.var_t3_dn13 = assign9480_e4877_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign9490_e4887, assign9490_e4887_d_n0, assign9490_e4887_d_n2, assign9490_e4887_d_n4, assign9490_e4887_d_n5, assign9490_e4887_d_n6, assign9490_e4887_d_n7, assign9490_e4887_d_n8, assign9490_e4887_d_n9, assign9490_e4887_d_n10, assign9490_e4887_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9490_e4883: f64 = (p.p374 / locals.var_t3);
        let assign9490_e4884: f64 = (1.0 + assign9490_e4883);
        let assign9490_e4885: f64 = (locals.var_uc_depvdsef2 * assign9490_e4884);
        (assign9490_e4885, ((locals.var_uc_depvdsef2_dn0 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn2 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn4 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn5 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn6 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn7 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn8 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn9 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn10 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn13 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn13,)
    }
};
        locals.var_uc_depvdsef2 = assign9490_e4887;
        locals.var_uc_depvdsef2_dn0 = assign9490_e4887_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign9490_e4887_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign9490_e4887_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign9490_e4887_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign9490_e4887_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign9490_e4887_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign9490_e4887_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign9490_e4887_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign9490_e4887_d_n10;
        locals.var_uc_depvdsef2_dn13 = assign9490_e4887_d_n13;
        locals.var_uc_depvdsef2_rv = 0.0;

        let assign9500_e4890: f64 = if locals.var_uc_depvdsef2 < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard192 = assign9500_e4890;
        locals.var_guard192_rv = 0.0;

        let (assign9510_e4896, assign9510_e4896_d_n0, assign9510_e4896_d_n2, assign9510_e4896_d_n4, assign9510_e4896_d_n5, assign9510_e4896_d_n6, assign9510_e4896_d_n7, assign9510_e4896_d_n8, assign9510_e4896_d_n9, assign9510_e4896_d_n10, assign9510_e4896_d_n13,) = {
    if ((locals.var_guard185 != 0.0) && (locals.var_guard192 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn13,)
    }
};
        locals.var_uc_depvdsef2 = assign9510_e4896;
        locals.var_uc_depvdsef2_dn0 = assign9510_e4896_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign9510_e4896_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign9510_e4896_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign9510_e4896_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign9510_e4896_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign9510_e4896_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign9510_e4896_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign9510_e4896_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign9510_e4896_d_n10;
        locals.var_uc_depvdsef2_dn13 = assign9510_e4896_d_n13;
        locals.var_uc_depvdsef2_rv = 0.0;

        let (assign9520_e4901, assign9520_e4901_d_n0, assign9520_e4901_d_n2, assign9520_e4901_d_n4, assign9520_e4901_d_n5, assign9520_e4901_d_n6, assign9520_e4901_d_n7, assign9520_e4901_d_n8, assign9520_e4901_d_n9, assign9520_e4901_d_n10, assign9520_e4901_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn13,)
    }
};
        locals.var_uc_ndepm = assign9520_e4901;
        locals.var_uc_ndepm_dn0 = assign9520_e4901_d_n0;
        locals.var_uc_ndepm_dn2 = assign9520_e4901_d_n2;
        locals.var_uc_ndepm_dn4 = assign9520_e4901_d_n4;
        locals.var_uc_ndepm_dn5 = assign9520_e4901_d_n5;
        locals.var_uc_ndepm_dn6 = assign9520_e4901_d_n6;
        locals.var_uc_ndepm_dn7 = assign9520_e4901_d_n7;
        locals.var_uc_ndepm_dn8 = assign9520_e4901_d_n8;
        locals.var_uc_ndepm_dn9 = assign9520_e4901_d_n9;
        locals.var_uc_ndepm_dn10 = assign9520_e4901_d_n10;
        locals.var_uc_ndepm_dn13 = assign9520_e4901_d_n13;
        locals.var_uc_ndepm_rv = 0.0;

        let (assign9530_e4906, assign9530_e4906_d_n0, assign9530_e4906_d_n2, assign9530_e4906_d_n4, assign9530_e4906_d_n5, assign9530_e4906_d_n6, assign9530_e4906_d_n7, assign9530_e4906_d_n8, assign9530_e4906_d_n9, assign9530_e4906_d_n10, assign9530_e4906_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign9530_e4906;
        locals.var_uc_depvmax_dn0 = assign9530_e4906_d_n0;
        locals.var_uc_depvmax_dn2 = assign9530_e4906_d_n2;
        locals.var_uc_depvmax_dn4 = assign9530_e4906_d_n4;
        locals.var_uc_depvmax_dn5 = assign9530_e4906_d_n5;
        locals.var_uc_depvmax_dn6 = assign9530_e4906_d_n6;
        locals.var_uc_depvmax_dn7 = assign9530_e4906_d_n7;
        locals.var_uc_depvmax_dn8 = assign9530_e4906_d_n8;
        locals.var_uc_depvmax_dn9 = assign9530_e4906_d_n9;
        locals.var_uc_depvmax_dn10 = assign9530_e4906_d_n10;
        locals.var_uc_depvmax_dn13 = assign9530_e4906_d_n13;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign9540_e4911, assign9540_e4911_d_n0, assign9540_e4911_d_n2, assign9540_e4911_d_n4, assign9540_e4911_d_n5, assign9540_e4911_d_n6, assign9540_e4911_d_n7, assign9540_e4911_d_n8, assign9540_e4911_d_n9, assign9540_e4911_d_n10, assign9540_e4911_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn13,)
    }
};
        locals.var_uc_depleak = assign9540_e4911;
        locals.var_uc_depleak_dn0 = assign9540_e4911_d_n0;
        locals.var_uc_depleak_dn2 = assign9540_e4911_d_n2;
        locals.var_uc_depleak_dn4 = assign9540_e4911_d_n4;
        locals.var_uc_depleak_dn5 = assign9540_e4911_d_n5;
        locals.var_uc_depleak_dn6 = assign9540_e4911_d_n6;
        locals.var_uc_depleak_dn7 = assign9540_e4911_d_n7;
        locals.var_uc_depleak_dn8 = assign9540_e4911_d_n8;
        locals.var_uc_depleak_dn9 = assign9540_e4911_d_n9;
        locals.var_uc_depleak_dn10 = assign9540_e4911_d_n10;
        locals.var_uc_depleak_dn13 = assign9540_e4911_d_n13;
        locals.var_uc_depleak_rv = 0.0;

        let (assign9550_e4916, assign9550_e4916_d_n0, assign9550_e4916_d_n2, assign9550_e4916_d_n4, assign9550_e4916_d_n5, assign9550_e4916_d_n6, assign9550_e4916_d_n7, assign9550_e4916_d_n8, assign9550_e4916_d_n9, assign9550_e4916_d_n10, assign9550_e4916_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign9550_e4916;
        locals.var_uc_depmue0_dn0 = assign9550_e4916_d_n0;
        locals.var_uc_depmue0_dn2 = assign9550_e4916_d_n2;
        locals.var_uc_depmue0_dn4 = assign9550_e4916_d_n4;
        locals.var_uc_depmue0_dn5 = assign9550_e4916_d_n5;
        locals.var_uc_depmue0_dn6 = assign9550_e4916_d_n6;
        locals.var_uc_depmue0_dn7 = assign9550_e4916_d_n7;
        locals.var_uc_depmue0_dn8 = assign9550_e4916_d_n8;
        locals.var_uc_depmue0_dn9 = assign9550_e4916_d_n9;
        locals.var_uc_depmue0_dn10 = assign9550_e4916_d_n10;
        locals.var_uc_depmue0_dn13 = assign9550_e4916_d_n13;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign9560_e4921, assign9560_e4921_d_n0, assign9560_e4921_d_n2, assign9560_e4921_d_n4, assign9560_e4921_d_n5, assign9560_e4921_d_n6, assign9560_e4921_d_n7, assign9560_e4921_d_n8, assign9560_e4921_d_n9, assign9560_e4921_d_n10, assign9560_e4921_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue1, locals.var_uc_depmue1_dn0, locals.var_uc_depmue1_dn2, locals.var_uc_depmue1_dn4, locals.var_uc_depmue1_dn5, locals.var_uc_depmue1_dn6, locals.var_uc_depmue1_dn7, locals.var_uc_depmue1_dn8, locals.var_uc_depmue1_dn9, locals.var_uc_depmue1_dn10, locals.var_uc_depmue1_dn13,)
    }
};
        locals.var_uc_depmue1 = assign9560_e4921;
        locals.var_uc_depmue1_dn0 = assign9560_e4921_d_n0;
        locals.var_uc_depmue1_dn2 = assign9560_e4921_d_n2;
        locals.var_uc_depmue1_dn4 = assign9560_e4921_d_n4;
        locals.var_uc_depmue1_dn5 = assign9560_e4921_d_n5;
        locals.var_uc_depmue1_dn6 = assign9560_e4921_d_n6;
        locals.var_uc_depmue1_dn7 = assign9560_e4921_d_n7;
        locals.var_uc_depmue1_dn8 = assign9560_e4921_d_n8;
        locals.var_uc_depmue1_dn9 = assign9560_e4921_d_n9;
        locals.var_uc_depmue1_dn10 = assign9560_e4921_d_n10;
        locals.var_uc_depmue1_dn13 = assign9560_e4921_d_n13;
        locals.var_uc_depmue1_rv = 0.0;

        let (assign9570_e4926, assign9570_e4926_d_n0, assign9570_e4926_d_n2, assign9570_e4926_d_n4, assign9570_e4926_d_n5, assign9570_e4926_d_n6, assign9570_e4926_d_n7, assign9570_e4926_d_n8, assign9570_e4926_d_n9, assign9570_e4926_d_n10, assign9570_e4926_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn13,)
    }
};
        locals.var_uc_depmueback0 = assign9570_e4926;
        locals.var_uc_depmueback0_dn0 = assign9570_e4926_d_n0;
        locals.var_uc_depmueback0_dn2 = assign9570_e4926_d_n2;
        locals.var_uc_depmueback0_dn4 = assign9570_e4926_d_n4;
        locals.var_uc_depmueback0_dn5 = assign9570_e4926_d_n5;
        locals.var_uc_depmueback0_dn6 = assign9570_e4926_d_n6;
        locals.var_uc_depmueback0_dn7 = assign9570_e4926_d_n7;
        locals.var_uc_depmueback0_dn8 = assign9570_e4926_d_n8;
        locals.var_uc_depmueback0_dn9 = assign9570_e4926_d_n9;
        locals.var_uc_depmueback0_dn10 = assign9570_e4926_d_n10;
        locals.var_uc_depmueback0_dn13 = assign9570_e4926_d_n13;
        locals.var_uc_depmueback0_rv = 0.0;

        let (assign9580_e4931, assign9580_e4931_d_n0, assign9580_e4931_d_n2, assign9580_e4931_d_n4, assign9580_e4931_d_n5, assign9580_e4931_d_n6, assign9580_e4931_d_n7, assign9580_e4931_d_n8, assign9580_e4931_d_n9, assign9580_e4931_d_n10, assign9580_e4931_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback1, locals.var_uc_depmueback1_dn0, locals.var_uc_depmueback1_dn2, locals.var_uc_depmueback1_dn4, locals.var_uc_depmueback1_dn5, locals.var_uc_depmueback1_dn6, locals.var_uc_depmueback1_dn7, locals.var_uc_depmueback1_dn8, locals.var_uc_depmueback1_dn9, locals.var_uc_depmueback1_dn10, locals.var_uc_depmueback1_dn13,)
    }
};
        locals.var_uc_depmueback1 = assign9580_e4931;
        locals.var_uc_depmueback1_dn0 = assign9580_e4931_d_n0;
        locals.var_uc_depmueback1_dn2 = assign9580_e4931_d_n2;
        locals.var_uc_depmueback1_dn4 = assign9580_e4931_d_n4;
        locals.var_uc_depmueback1_dn5 = assign9580_e4931_d_n5;
        locals.var_uc_depmueback1_dn6 = assign9580_e4931_d_n6;
        locals.var_uc_depmueback1_dn7 = assign9580_e4931_d_n7;
        locals.var_uc_depmueback1_dn8 = assign9580_e4931_d_n8;
        locals.var_uc_depmueback1_dn9 = assign9580_e4931_d_n9;
        locals.var_uc_depmueback1_dn10 = assign9580_e4931_d_n10;
        locals.var_uc_depmueback1_dn13 = assign9580_e4931_d_n13;
        locals.var_uc_depmueback1_rv = 0.0;

        let (assign9590_e4936, assign9590_e4936_d_n0, assign9590_e4936_d_n2, assign9590_e4936_d_n4, assign9590_e4936_d_n5, assign9590_e4936_d_n6, assign9590_e4936_d_n7, assign9590_e4936_d_n8, assign9590_e4936_d_n9, assign9590_e4936_d_n10, assign9590_e4936_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef1, locals.var_uc_depvdsef1_dn0, locals.var_uc_depvdsef1_dn2, locals.var_uc_depvdsef1_dn4, locals.var_uc_depvdsef1_dn5, locals.var_uc_depvdsef1_dn6, locals.var_uc_depvdsef1_dn7, locals.var_uc_depvdsef1_dn8, locals.var_uc_depvdsef1_dn9, locals.var_uc_depvdsef1_dn10, locals.var_uc_depvdsef1_dn13,)
    }
};
        locals.var_uc_depvdsef1 = assign9590_e4936;
        locals.var_uc_depvdsef1_dn0 = assign9590_e4936_d_n0;
        locals.var_uc_depvdsef1_dn2 = assign9590_e4936_d_n2;
        locals.var_uc_depvdsef1_dn4 = assign9590_e4936_d_n4;
        locals.var_uc_depvdsef1_dn5 = assign9590_e4936_d_n5;
        locals.var_uc_depvdsef1_dn6 = assign9590_e4936_d_n6;
        locals.var_uc_depvdsef1_dn7 = assign9590_e4936_d_n7;
        locals.var_uc_depvdsef1_dn8 = assign9590_e4936_d_n8;
        locals.var_uc_depvdsef1_dn9 = assign9590_e4936_d_n9;
        locals.var_uc_depvdsef1_dn10 = assign9590_e4936_d_n10;
        locals.var_uc_depvdsef1_dn13 = assign9590_e4936_d_n13;
        locals.var_uc_depvdsef1_rv = 0.0;

        let (assign9600_e4941, assign9600_e4941_d_n0, assign9600_e4941_d_n2, assign9600_e4941_d_n4, assign9600_e4941_d_n5, assign9600_e4941_d_n6, assign9600_e4941_d_n7, assign9600_e4941_d_n8, assign9600_e4941_d_n9, assign9600_e4941_d_n10, assign9600_e4941_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn13,)
    }
};
        locals.var_uc_depvdsef2 = assign9600_e4941;
        locals.var_uc_depvdsef2_dn0 = assign9600_e4941_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign9600_e4941_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign9600_e4941_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign9600_e4941_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign9600_e4941_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign9600_e4941_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign9600_e4941_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign9600_e4941_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign9600_e4941_d_n10;
        locals.var_uc_depvdsef2_dn13 = assign9600_e4941_d_n13;
        locals.var_uc_depvdsef2_rv = 0.0;

        let assign10120_e5314: f64 = (locals.var_uc_xpdv * locals.var_uc_xldld);
        let assign10120_e5316: f64 = if assign10120_e5314 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard244 = assign10120_e5316;
        locals.var_guard244_rv = 0.0;

        let (assign10130_e5322,) = {
    if (locals.var_guard244 != 0.0) {
        let assign10130_e5320: f64 = (1.0 / locals.var_uc_xldld);
        (assign10130_e5320,)
    } else {
        (locals.var_uc_xpdv,)
    }
};
        locals.var_uc_xpdv = assign10130_e5322;
        locals.var_uc_xpdv_rv = 0.0;

        let assign10150_e5350: f64 = if ((p.p40 == 1.0) && (((p.p19 > 0.0) && (locals.var_uc_nover == 0.0)) || ((p.p18 > 0.0) && (locals.var_uc_novers == 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard246 = assign10150_e5350;
        locals.var_guard246_rv = 0.0;

        let (assign10160_e5354,) = {
    if (locals.var_guard246 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_cordrift,)
    }
};
        locals.var_uc_cordrift = assign10160_e5354;
        locals.var_uc_cordrift_rv = 0.0;

        let (assign10170_e5359,) = {
    if (locals.var_guard246 == 0.0) {
        (p.p40,)
    } else {
        (locals.var_uc_cordrift,)
    }
};
        locals.var_uc_cordrift = assign10170_e5359;
        locals.var_uc_cordrift_rv = 0.0;

        let assign10180_e5362: f64 = if locals.var_uc_cordrift == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard247 = assign10180_e5362;
        locals.var_guard247_rv = 0.0;

        let (assign10190_e5371,) = {
    if (locals.var_guard247 != 0.0) {
        let (assign10190_e5369,) = {
            if (p.p19 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10190_e5369,)
    } else {
        (locals.var_flg_rd,)
    }
};
        locals.var_flg_rd = assign10190_e5371;
        locals.var_flg_rd_rv = 0.0;

        let (assign10200_e5380,) = {
    if (locals.var_guard247 != 0.0) {
        let (assign10200_e5378,) = {
            if (p.p18 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10200_e5378,)
    } else {
        (locals.var_flg_rs,)
    }
};
        locals.var_flg_rs = assign10200_e5380;
        locals.var_flg_rs_rv = 0.0;

        let assign10210_e5387: f64 = if ((p.p17 == 0.0) || (p.p17 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard248 = assign10210_e5387;
        locals.var_guard248_rv = 0.0;

        let (assign10220_e5394,) = {
    if ((locals.var_guard247 == 0.0) && (locals.var_guard248 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_rd,)
    }
};
        locals.var_flg_rd = assign10220_e5394;
        locals.var_flg_rd_rv = 0.0;

        let (assign10230_e5401,) = {
    if ((locals.var_guard247 == 0.0) && (locals.var_guard248 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_rs,)
    }
};
        locals.var_flg_rs = assign10230_e5401;
        locals.var_flg_rs_rv = 0.0;

        let (assign10240_e5433, assign10240_e5433_d_n0, assign10240_e5433_d_n2, assign10240_e5433_d_n4, assign10240_e5433_d_n5, assign10240_e5433_d_n6, assign10240_e5433_d_n7, assign10240_e5433_d_n8, assign10240_e5433_d_n9, assign10240_e5433_d_n10, assign10240_e5433_d_n13,) = {
    if ((locals.var_guard247 == 0.0) && (locals.var_guard248 == 0.0)) {
        let assign10240_e5409: f64 = (p.p130 * p.p2);
        let assign10240_e5411: f64 = (assign10240_e5409 * p.p7);
        let assign10240_e5414: f64 = (locals.var_uc_rd + locals.var_uc_rdvd);
        let assign10240_e5417: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign10240_e5419: f64 = (assign10240_e5417 * 1000000.0);
        let assign10240_e5421: f64 = (assign10240_e5419 + locals.var_uc_rdict1);
        let assign10240_e5422: f64 = (assign10240_e5414 * assign10240_e5421);
        let assign10240_e5425: f64 = (p.p68 * p.p100);
        let assign10240_e5427: f64 = (assign10240_e5425 * 1000000.0);
        let assign10240_e5429: f64 = (assign10240_e5427 + p.p101);
        let assign10240_e5430: f64 = (assign10240_e5422 * assign10240_e5429);
        let assign10240_e5431: f64 = (assign10240_e5411 + assign10240_e5430);
        (assign10240_e5431, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign10240_e5433;
        locals.var_t1_dn0 = assign10240_e5433_d_n0;
        locals.var_t1_dn2 = assign10240_e5433_d_n2;
        locals.var_t1_dn4 = assign10240_e5433_d_n4;
        locals.var_t1_dn5 = assign10240_e5433_d_n5;
        locals.var_t1_dn6 = assign10240_e5433_d_n6;
        locals.var_t1_dn7 = assign10240_e5433_d_n7;
        locals.var_t1_dn8 = assign10240_e5433_d_n8;
        locals.var_t1_dn9 = assign10240_e5433_d_n9;
        locals.var_t1_dn10 = assign10240_e5433_d_n10;
        locals.var_t1_dn13 = assign10240_e5433_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign10250_e5446,) = {
    if ((locals.var_guard247 == 0.0) && (locals.var_guard248 == 0.0)) {
        let (assign10250_e5444,) = {
            if (locals.var_t1 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10250_e5444,)
    } else {
        (locals.var_flg_rd,)
    }
};
        locals.var_flg_rd = assign10250_e5446;
        locals.var_flg_rd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10260_e5476, assign10260_e5476_d_n0, assign10260_e5476_d_n2, assign10260_e5476_d_n4, assign10260_e5476_d_n5, assign10260_e5476_d_n6, assign10260_e5476_d_n7, assign10260_e5476_d_n8, assign10260_e5476_d_n9, assign10260_e5476_d_n10, assign10260_e5476_d_n13,) = {
    if ((locals.var_guard247 == 0.0) && (locals.var_guard248 == 0.0)) {
        let assign10260_e5454: f64 = (p.p131 * p.p3);
        let assign10260_e5456: f64 = (assign10260_e5454 * p.p7);
        let assign10260_e5460: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign10260_e5462: f64 = (assign10260_e5460 * 1000000.0);
        let assign10260_e5464: f64 = (assign10260_e5462 + locals.var_uc_rdict1);
        let assign10260_e5465: f64 = (locals.var_uc_rs * assign10260_e5464);
        let assign10260_e5468: f64 = (p.p70 * p.p100);
        let assign10260_e5470: f64 = (assign10260_e5468 * 1000000.0);
        let assign10260_e5472: f64 = (assign10260_e5470 + p.p101);
        let assign10260_e5473: f64 = (assign10260_e5465 * assign10260_e5472);
        let assign10260_e5474: f64 = (assign10260_e5456 + assign10260_e5473);
        (assign10260_e5474, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign10260_e5476;
        locals.var_t1_dn0 = assign10260_e5476_d_n0;
        locals.var_t1_dn2 = assign10260_e5476_d_n2;
        locals.var_t1_dn4 = assign10260_e5476_d_n4;
        locals.var_t1_dn5 = assign10260_e5476_d_n5;
        locals.var_t1_dn6 = assign10260_e5476_d_n6;
        locals.var_t1_dn7 = assign10260_e5476_d_n7;
        locals.var_t1_dn8 = assign10260_e5476_d_n8;
        locals.var_t1_dn9 = assign10260_e5476_d_n9;
        locals.var_t1_dn10 = assign10260_e5476_d_n10;
        locals.var_t1_dn13 = assign10260_e5476_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign10270_e5489,) = {
    if ((locals.var_guard247 == 0.0) && (locals.var_guard248 == 0.0)) {
        let (assign10270_e5487,) = {
            if (locals.var_t1 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10270_e5487,)
    } else {
        (locals.var_flg_rs,)
    }
};
        locals.var_flg_rs = assign10270_e5489;
        locals.var_flg_rs_rv = 0.0;

        let assign10280_e5492: f64 = (p.p12 / 1e-6);
        locals.var_mks_nsubcdfm = assign10280_e5492;
        locals.var_mks_nsubcdfm_rv = 0.0;

        let assign10290_e5495: f64 = (p.p73 * 100.0);
        locals.var_mks_subld2 = assign10290_e5495;
        locals.var_mks_subld2_rv = 0.0;

        let assign10300_e5498: f64 = (locals.var_uc_nsubc / 1e-6);
        locals.var_uc_nsubc = assign10300_e5498;
        locals.var_uc_nsubc_rv = 0.0;

        let assign10310_e5501: f64 = (locals.var_uc_nsubp / 1e-6);
        locals.var_uc_nsubp = assign10310_e5501;
        locals.var_uc_nsubp_rv = 0.0;

        let assign10320_e5504: f64 = (locals.var_uc_nsti / 1e-6);
        locals.var_uc_nsti = assign10320_e5504;
        locals.var_uc_nsti_rv = 0.0;

        let assign10330_e5507: f64 = (locals.var_uc_nover / 1e-6);
        locals.var_uc_nover = assign10330_e5507;
        locals.var_uc_nover_rv = 0.0;

        let assign10340_e5510: f64 = (locals.var_uc_novers / 1e-6);
        locals.var_uc_novers = assign10340_e5510;
        locals.var_uc_novers_rv = 0.0;

        let assign10350_e5513: f64 = (locals.var_uc_nsubpsti1 / 100.0);
        locals.var_uc_nsubpsti1 = assign10350_e5513;
        locals.var_uc_nsubpsti1_rv = 0.0;

        let assign10360_e5516: f64 = (locals.var_uc_muesti1 / 100.0);
        locals.var_uc_muesti1 = assign10360_e5516;
        locals.var_uc_muesti1_rv = 0.0;

        let assign10370_e5519: f64 = (locals.var_uc_vmax / 100.0);
        locals.var_uc_vmax = assign10370_e5519;
        locals.var_uc_vmax_rv = 0.0;

        let assign10380_e5522: f64 = (locals.var_uc_wfc * 10000.0);
        locals.var_uc_wfc = assign10380_e5522;
        locals.var_uc_wfc_rv = 0.0;

        let assign10390_e5525: f64 = (locals.var_uc_glksd1 / 100.0);
        locals.var_uc_glksd1 = assign10390_e5525;
        locals.var_uc_glksd1_rv = 0.0;

        let assign10400_e5528: f64 = (locals.var_uc_glksd2 * 100.0);
        locals.var_uc_glksd2 = assign10400_e5528;
        locals.var_uc_glksd2_rv = 0.0;

        let assign10410_e5531: f64 = (locals.var_uc_gleak2 * 100.0);
        locals.var_uc_gleak2 = assign10410_e5531;
        locals.var_uc_gleak2_rv = 0.0;

        let assign10420_e5534: f64 = (locals.var_uc_glkb2 * 100.0);
        locals.var_uc_glkb2 = assign10420_e5534;
        locals.var_uc_glkb2_rv = 0.0;

        let assign10430_e5537: f64 = (locals.var_uc_fn2 * 100.0);
        locals.var_uc_fn2 = assign10430_e5537;
        locals.var_uc_fn2_rv = 0.0;

        let assign10440_e5540: f64 = (locals.var_uc_gidl1 / 10.0);
        locals.var_uc_gidl1 = assign10440_e5540;
        locals.var_uc_gidl1_rv = 0.0;

        let assign10450_e5543: f64 = (locals.var_uc_gidl2 * 100.0);
        locals.var_uc_gidl2 = assign10450_e5543;
        locals.var_uc_gidl2_rv = 0.0;

        let assign10460_e5546: f64 = (locals.var_uc_nfalp / 100.0);
        locals.var_uc_nfalp = assign10460_e5546;
        locals.var_uc_nfalp_rv = 0.0;

        let assign10480_e5552: f64 = (locals.var_uc_npext / 1e-6);
        locals.var_uc_npext = assign10480_e5552;
        locals.var_uc_npext_rv = 0.0;

        let assign10490_e5555: f64 = (locals.var_uc_rd22 / 100.0);
        locals.var_uc_rd22 = assign10490_e5555;
        locals.var_uc_rd22_rv = 0.0;

        let assign10500_e5558: f64 = (locals.var_uc_rd23 / 100.0);
        locals.var_uc_rd23 = assign10500_e5558;
        locals.var_uc_rd23_rv = 0.0;

        let assign10510_e5561: f64 = (locals.var_uc_rd24 / 100.0);
        locals.var_uc_rd24 = assign10510_e5561;
        locals.var_uc_rd24_rv = 0.0;

        let assign10520_e5564: f64 = (locals.var_uc_rdvd / 100.0);
        locals.var_uc_rdvd = assign10520_e5564;
        locals.var_uc_rdvd_rv = 0.0;

        let assign10530_e5567: f64 = (locals.var_uc_rth0 / 100.0);
        locals.var_uc_rth0 = assign10530_e5567;
        locals.var_uc_rth0_rv = 0.0;

        let assign10540_e5569: f64 = (-locals.var_uc_vfbover);
        locals.var_uc_vfbover = assign10540_e5569;
        locals.var_uc_vfbover_rv = 0.0;

        let assign10550_e5572: f64 = (locals.var_uc_depvmax / 100.0);
        locals.var_uc_depvmax = assign10550_e5572;
        locals.var_uc_depvmax_dn0 = (locals.var_uc_depvmax_dn0 / 100.0);
        locals.var_uc_depvmax_dn2 = (locals.var_uc_depvmax_dn2 / 100.0);
        locals.var_uc_depvmax_dn4 = (locals.var_uc_depvmax_dn4 / 100.0);
        locals.var_uc_depvmax_dn5 = (locals.var_uc_depvmax_dn5 / 100.0);
        locals.var_uc_depvmax_dn6 = (locals.var_uc_depvmax_dn6 / 100.0);
        locals.var_uc_depvmax_dn7 = (locals.var_uc_depvmax_dn7 / 100.0);
        locals.var_uc_depvmax_dn8 = (locals.var_uc_depvmax_dn8 / 100.0);
        locals.var_uc_depvmax_dn9 = (locals.var_uc_depvmax_dn9 / 100.0);
        locals.var_uc_depvmax_dn10 = (locals.var_uc_depvmax_dn10 / 100.0);
        locals.var_uc_depvmax_dn13 = (locals.var_uc_depvmax_dn13 / 100.0);
        locals.var_uc_depvmax_rv = 0.0;

        locals.var_flg_nqs = p.p28;
        locals.var_flg_nqs_rv = 0.0;

        let (assign10570_e5583,) = {
    if ((p.p133 != 0.0) || (p.p134 != 0.0)) {
        (1.0,)
    } else {
        (0.0,)
    }
};
        locals.var_flg_qy = assign10570_e5583;
        locals.var_flg_qy_rv = 0.0;

        let assign10590_e5597: f64 = if (((p.p235 == 0.0) && (p.p237 == 0.0)) || (p.p236 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard250 = assign10590_e5597;
        locals.var_guard250_rv = 0.0;

        let (assign10600_e5601,) = {
    if (locals.var_guard250 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_qmetemp,)
    }
};
        locals.var_flg_qmetemp = assign10600_e5601;
        locals.var_flg_qmetemp_rv = 0.0;

        let (assign10610_e5606,) = {
    if (locals.var_guard250 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_qmetemp,)
    }
};
        locals.var_flg_qmetemp = assign10610_e5606;
        locals.var_flg_qmetemp_rv = 0.0;

        let assign10620_e5609: f64 = (locals.var_wg * locals.var_lg);
        locals.var_wlg = assign10620_e5609;
        locals.var_wlg_rv = 0.0;

        let assign10630_e5612: f64 = (p.p289 * 1000000.0);
        locals.var_uc_gdld = assign10630_e5612;
        locals.var_uc_gdld_rv = 0.0;

        let assign10640_e5618: f64 = (locals.var_ktnom * 1e-7);
        let assign10640_e5619: f64 = (9.025e-5 + assign10640_e5618);
        let assign10640_e5620: f64 = (locals.var_ktnom * assign10640_e5619);
        let assign10640_e5621: f64 = (locals.var_uc_eg0 - assign10640_e5620);
        locals.var_egtnom = assign10640_e5621;
        locals.var_egtnom_rv = 0.0;

        let assign10650_e5624: f64 = (8.8541878e-12 * p.p267);
        locals.var_cecox = assign10650_e5624;
        locals.var_cecox_rv = 0.0;

        locals.var_msc = locals.var_uc_scp22;
        locals.var_msc_rv = 0.0;

        let assign10670_e5628: f64 = if locals.var_uc_pgd1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign10670_e5628;
        locals.var_guard251_rv = 0.0;

        let (assign10680_e5632,) = {
    if (locals.var_guard251 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_pgd,)
    }
};
        locals.var_flg_pgd = assign10680_e5632;
        locals.var_flg_pgd_rv = 0.0;

        let (assign10690_e5636,) = {
    if (locals.var_guard251 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cnstpgd,)
    }
};
        locals.var_cnstpgd = assign10690_e5636;
        locals.var_cnstpgd_rv = 0.0;

        let (assign10700_e5641,) = {
    if (locals.var_guard251 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_pgd,)
    }
};
        locals.var_flg_pgd = assign10700_e5641;
        locals.var_flg_pgd_rv = 0.0;

        let (assign10710_e5654,) = {
    if (locals.var_guard251 == 0.0) {
        let assign10710_e5647: f64 = (1.0 / locals.var_lg);
        let assign10710_e5648: f64 = (1.0 + assign10710_e5647);
        let assign10710_e5650: f64 = (assign10710_e5648).powf(p.p153);
        let assign10710_e5652: f64 = (assign10710_e5650 * locals.var_uc_pgd1);
        (assign10710_e5652,)
    } else {
        (locals.var_cnstpgd,)
    }
};
        locals.var_cnstpgd = assign10710_e5654;
        locals.var_cnstpgd_rv = 0.0;

        let assign10720_e5658: f64 = (locals.var_lg).powf(p.p229);
        let assign10720_e5660: f64 = (assign10720_e5658 * p.p230);
        let assign10720_e5661: f64 = (1.0 + assign10720_e5660);
        locals.var_clmmod = assign10720_e5661;
        locals.var_clmmod_rv = 0.0;

        let assign10730_e5666: f64 = (0.5 * p.p0);
        let assign10730_e5667: f64 = (p.p118 + assign10730_e5666);
        let assign10730_e5668: f64 = (1.0 / assign10730_e5667);
        let assign10730_e5673: f64 = (0.5 * p.p0);
        let assign10730_e5674: f64 = (p.p119 + assign10730_e5673);
        let assign10730_e5675: f64 = (1.0 / assign10730_e5674);
        let assign10730_e5676: f64 = (assign10730_e5668 + assign10730_e5675);
        locals.var_t1 = assign10730_e5676;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign10740_e5679: f64 = (2.0 / locals.var_t1);
        locals.var_lod_half_ref = assign10740_e5679;
        locals.var_lod_half_ref_dn0 = (-((2.0 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn2 = (-((2.0 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn4 = (-((2.0 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn5 = (-((2.0 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn6 = (-((2.0 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn7 = (-((2.0 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn8 = (-((2.0 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn9 = (-((2.0 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn10 = (-((2.0 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn13 = (-((2.0 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_rv = 0.0;

        let assign10750_e5698: f64 = if (((p.p8 > 0.0) && (p.p9 > 0.0)) && ((p.p7 == 1.0) || ((p.p7 > 1.0) && (p.p10 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard252 = assign10750_e5698;
        locals.var_guard252_rv = 0.0;

        let (assign10760_e5702, assign10760_e5702_d_n0, assign10760_e5702_d_n2, assign10760_e5702_d_n4, assign10760_e5702_d_n5, assign10760_e5702_d_n6, assign10760_e5702_d_n7, assign10760_e5702_d_n8, assign10760_e5702_d_n9, assign10760_e5702_d_n10, assign10760_e5702_d_n13,) = {
    if (locals.var_guard252 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign10760_e5702;
        locals.var_t1_dn0 = assign10760_e5702_d_n0;
        locals.var_t1_dn2 = assign10760_e5702_d_n2;
        locals.var_t1_dn4 = assign10760_e5702_d_n4;
        locals.var_t1_dn5 = assign10760_e5702_d_n5;
        locals.var_t1_dn6 = assign10760_e5702_d_n6;
        locals.var_t1_dn7 = assign10760_e5702_d_n7;
        locals.var_t1_dn8 = assign10760_e5702_d_n8;
        locals.var_t1_dn9 = assign10760_e5702_d_n9;
        locals.var_t1_dn10 = assign10760_e5702_d_n10;
        locals.var_t1_dn13 = assign10760_e5702_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign10770_e5706,) = {
    if (locals.var_guard252 != 0.0) {
        (0.0,)
    } else {
        (locals.var_i,)
    }
};
        locals.var_i = assign10770_e5706;
        locals.var_i_rv = 0.0;

        let mut assign10780_loop_guard: usize = 0;
        while {
            let assign10780_cond_e5711: f64 = if ((locals.var_guard252 != 0.0) && (locals.var_i < p.p7)) { 1.0 } else { 0.0 };
            assign10780_cond_e5711 != 0.0
        } {
            assign10780_loop_guard += 1;
            assert!(assign10780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign10780_body0_e5743, assign10780_body0_e5743_d_n0, assign10780_body0_e5743_d_n2, assign10780_body0_e5743_d_n4, assign10780_body0_e5743_d_n5, assign10780_body0_e5743_d_n6, assign10780_body0_e5743_d_n7, assign10780_body0_e5743_d_n8, assign10780_body0_e5743_d_n9, assign10780_body0_e5743_d_n10, assign10780_body0_e5743_d_n13,) = {
    if (locals.var_guard252 != 0.0) {
        let assign10780_body0_e5718: f64 = (0.5 * p.p0);
        let assign10780_body0_e5719: f64 = (p.p8 + assign10780_body0_e5718);
        let assign10780_body0_e5723: f64 = (p.p10 + p.p0);
        let assign10780_body0_e5724: f64 = (locals.var_i * assign10780_body0_e5723);
        let assign10780_body0_e5725: f64 = (assign10780_body0_e5719 + assign10780_body0_e5724);
        let assign10780_body0_e5726: f64 = (1.0 / assign10780_body0_e5725);
        let assign10780_body0_e5727: f64 = (locals.var_t1 + assign10780_body0_e5726);
        let assign10780_body0_e5732: f64 = (0.5 * p.p0);
        let assign10780_body0_e5733: f64 = (p.p9 + assign10780_body0_e5732);
        let assign10780_body0_e5737: f64 = (p.p10 + p.p0);
        let assign10780_body0_e5738: f64 = (locals.var_i * assign10780_body0_e5737);
        let assign10780_body0_e5739: f64 = (assign10780_body0_e5733 + assign10780_body0_e5738);
        let assign10780_body0_e5740: f64 = (1.0 / assign10780_body0_e5739);
        let assign10780_body0_e5741: f64 = (assign10780_body0_e5727 + assign10780_body0_e5740);
        (assign10780_body0_e5741, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign10780_body0_e5743;
            locals.var_t1_dn0 = assign10780_body0_e5743_d_n0;
            locals.var_t1_dn2 = assign10780_body0_e5743_d_n2;
            locals.var_t1_dn4 = assign10780_body0_e5743_d_n4;
            locals.var_t1_dn5 = assign10780_body0_e5743_d_n5;
            locals.var_t1_dn6 = assign10780_body0_e5743_d_n6;
            locals.var_t1_dn7 = assign10780_body0_e5743_d_n7;
            locals.var_t1_dn8 = assign10780_body0_e5743_d_n8;
            locals.var_t1_dn9 = assign10780_body0_e5743_d_n9;
            locals.var_t1_dn10 = assign10780_body0_e5743_d_n10;
            locals.var_t1_dn13 = assign10780_body0_e5743_d_n13;
            locals.var_t1_rv = 0.0;
            let (assign10780_body1_e5749,) = {
    if (locals.var_guard252 != 0.0) {
        let assign10780_body1_e5747: f64 = (locals.var_i + 1.0);
        (assign10780_body1_e5747,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign10780_body1_e5749;
            locals.var_i_rv = 0.0;
        }

        let (assign10790_e5757, assign10790_e5757_d_n0, assign10790_e5757_d_n2, assign10790_e5757_d_n4, assign10790_e5757_d_n5, assign10790_e5757_d_n6, assign10790_e5757_d_n7, assign10790_e5757_d_n8, assign10790_e5757_d_n9, assign10790_e5757_d_n10, assign10790_e5757_d_n13,) = {
    if (locals.var_guard252 != 0.0) {
        let assign10790_e5753: f64 = (2.0 * p.p7);
        let assign10790_e5755: f64 = (assign10790_e5753 / locals.var_t1);
        (assign10790_e5755, (-((assign10790_e5753 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn4, locals.var_lod_half_dn5, locals.var_lod_half_dn6, locals.var_lod_half_dn7, locals.var_lod_half_dn8, locals.var_lod_half_dn9, locals.var_lod_half_dn10, locals.var_lod_half_dn13,)
    }
};
        locals.var_lod_half = assign10790_e5757;
        locals.var_lod_half_dn0 = assign10790_e5757_d_n0;
        locals.var_lod_half_dn2 = assign10790_e5757_d_n2;
        locals.var_lod_half_dn4 = assign10790_e5757_d_n4;
        locals.var_lod_half_dn5 = assign10790_e5757_d_n5;
        locals.var_lod_half_dn6 = assign10790_e5757_d_n6;
        locals.var_lod_half_dn7 = assign10790_e5757_d_n7;
        locals.var_lod_half_dn8 = assign10790_e5757_d_n8;
        locals.var_lod_half_dn9 = assign10790_e5757_d_n9;
        locals.var_lod_half_dn10 = assign10790_e5757_d_n10;
        locals.var_lod_half_dn13 = assign10790_e5757_d_n13;
        locals.var_lod_half_rv = 0.0;

        let (assign10800_e5762, assign10800_e5762_d_n0, assign10800_e5762_d_n2, assign10800_e5762_d_n4, assign10800_e5762_d_n5, assign10800_e5762_d_n6, assign10800_e5762_d_n7, assign10800_e5762_d_n8, assign10800_e5762_d_n9, assign10800_e5762_d_n10, assign10800_e5762_d_n13,) = {
    if (locals.var_guard252 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn4, locals.var_lod_half_dn5, locals.var_lod_half_dn6, locals.var_lod_half_dn7, locals.var_lod_half_dn8, locals.var_lod_half_dn9, locals.var_lod_half_dn10, locals.var_lod_half_dn13,)
    }
};
        locals.var_lod_half = assign10800_e5762;
        locals.var_lod_half_dn0 = assign10800_e5762_d_n0;
        locals.var_lod_half_dn2 = assign10800_e5762_d_n2;
        locals.var_lod_half_dn4 = assign10800_e5762_d_n4;
        locals.var_lod_half_dn5 = assign10800_e5762_d_n5;
        locals.var_lod_half_dn6 = assign10800_e5762_d_n6;
        locals.var_lod_half_dn7 = assign10800_e5762_d_n7;
        locals.var_lod_half_dn8 = assign10800_e5762_d_n8;
        locals.var_lod_half_dn9 = assign10800_e5762_d_n9;
        locals.var_lod_half_dn10 = assign10800_e5762_d_n10;
        locals.var_lod_half_dn13 = assign10800_e5762_d_n13;
        locals.var_lod_half_rv = 0.0;

        locals.var_npexte = locals.var_uc_npext;
        locals.var_npexte_dn0 = 0.0;
        locals.var_npexte_dn2 = 0.0;
        locals.var_npexte_dn4 = 0.0;
        locals.var_npexte_dn5 = 0.0;
        locals.var_npexte_dn6 = 0.0;
        locals.var_npexte_dn7 = 0.0;
        locals.var_npexte_dn8 = 0.0;
        locals.var_npexte_dn9 = 0.0;
        locals.var_npexte_dn10 = 0.0;
        locals.var_npexte_dn13 = 0.0;
        locals.var_npexte_rv = 0.0;

        locals.var_ef_mueph1 = locals.var_uc_mueph1;
        locals.var_ef_mueph1_dn0 = 0.0;
        locals.var_ef_mueph1_dn2 = 0.0;
        locals.var_ef_mueph1_dn4 = 0.0;
        locals.var_ef_mueph1_dn5 = 0.0;
        locals.var_ef_mueph1_dn6 = 0.0;
        locals.var_ef_mueph1_dn7 = 0.0;
        locals.var_ef_mueph1_dn8 = 0.0;
        locals.var_ef_mueph1_dn9 = 0.0;
        locals.var_ef_mueph1_dn10 = 0.0;
        locals.var_ef_mueph1_dn13 = 0.0;
        locals.var_ef_mueph1_rv = 0.0;

        locals.var_ef_nsubp = locals.var_uc_nsubp;
        locals.var_ef_nsubp_dn0 = 0.0;
        locals.var_ef_nsubp_dn2 = 0.0;
        locals.var_ef_nsubp_dn4 = 0.0;
        locals.var_ef_nsubp_dn5 = 0.0;
        locals.var_ef_nsubp_dn6 = 0.0;
        locals.var_ef_nsubp_dn7 = 0.0;
        locals.var_ef_nsubp_dn8 = 0.0;
        locals.var_ef_nsubp_dn9 = 0.0;
        locals.var_ef_nsubp_dn10 = 0.0;
        locals.var_ef_nsubp_dn13 = 0.0;
        locals.var_ef_nsubp_rv = 0.0;

        locals.var_ef_nsubc = locals.var_uc_nsubc;
        locals.var_ef_nsubc_dn0 = 0.0;
        locals.var_ef_nsubc_dn2 = 0.0;
        locals.var_ef_nsubc_dn4 = 0.0;
        locals.var_ef_nsubc_dn5 = 0.0;
        locals.var_ef_nsubc_dn6 = 0.0;
        locals.var_ef_nsubc_dn7 = 0.0;
        locals.var_ef_nsubc_dn8 = 0.0;
        locals.var_ef_nsubc_dn9 = 0.0;
        locals.var_ef_nsubc_dn10 = 0.0;
        locals.var_ef_nsubc_dn13 = 0.0;
        locals.var_ef_nsubc_rv = 0.0;

        let assign10850_e5771: f64 = if ((p.p32 == 1.0) && (locals.var_nsubcdfm_given != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard253 = assign10850_e5771;
        locals.var_guard253_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10870_e5792, assign10870_e5792_d_n0, assign10870_e5792_d_n2, assign10870_e5792_d_n4, assign10870_e5792_d_n5, assign10870_e5792_d_n6, assign10870_e5792_d_n7, assign10870_e5792_d_n8, assign10870_e5792_d_n9, assign10870_e5792_d_n10, assign10870_e5792_d_n13,) = {
    if (locals.var_guard253 != 0.0) {
        let assign10870_e5783: f64 = (locals.var_mks_nsubcdfm).ln();
        let assign10870_e5785: f64 = (locals.var_ef_nsubc).ln();
        let assign10870_e5786: f64 = (assign10870_e5783 - assign10870_e5785);
        let assign10870_e5787: f64 = (p.p282 * assign10870_e5786);
        let assign10870_e5789: f64 = (assign10870_e5787 + 1.0);
        let assign10870_e5790: f64 = (locals.var_ef_mueph1 * assign10870_e5789);
        (assign10870_e5790, ((locals.var_ef_mueph1_dn0 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn0 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn2 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn2 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn4 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn4 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn5 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn5 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn6 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn6 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn7 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn7 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn8 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn8 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn9 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn9 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn10 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn10 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn13 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn13 / locals.var_ef_nsubc))))),)
    } else {
        (locals.var_ef_mueph1, locals.var_ef_mueph1_dn0, locals.var_ef_mueph1_dn2, locals.var_ef_mueph1_dn4, locals.var_ef_mueph1_dn5, locals.var_ef_mueph1_dn6, locals.var_ef_mueph1_dn7, locals.var_ef_mueph1_dn8, locals.var_ef_mueph1_dn9, locals.var_ef_mueph1_dn10, locals.var_ef_mueph1_dn13,)
    }
};
        locals.var_ef_mueph1 = assign10870_e5792;
        locals.var_ef_mueph1_dn0 = assign10870_e5792_d_n0;
        locals.var_ef_mueph1_dn2 = assign10870_e5792_d_n2;
        locals.var_ef_mueph1_dn4 = assign10870_e5792_d_n4;
        locals.var_ef_mueph1_dn5 = assign10870_e5792_d_n5;
        locals.var_ef_mueph1_dn6 = assign10870_e5792_d_n6;
        locals.var_ef_mueph1_dn7 = assign10870_e5792_d_n7;
        locals.var_ef_mueph1_dn8 = assign10870_e5792_d_n8;
        locals.var_ef_mueph1_dn9 = assign10870_e5792_d_n9;
        locals.var_ef_mueph1_dn10 = assign10870_e5792_d_n10;
        locals.var_ef_mueph1_dn13 = assign10870_e5792_d_n13;
        locals.var_ef_mueph1_rv = 0.0;

        let (assign10880_e5800, assign10880_e5800_d_n0, assign10880_e5800_d_n2, assign10880_e5800_d_n4, assign10880_e5800_d_n5, assign10880_e5800_d_n6, assign10880_e5800_d_n7, assign10880_e5800_d_n8, assign10880_e5800_d_n9, assign10880_e5800_d_n10, assign10880_e5800_d_n13,) = {
    if (locals.var_guard253 != 0.0) {
        let assign10880_e5796: f64 = (locals.var_ef_nsubp + locals.var_mks_nsubcdfm);
        let assign10880_e5798: f64 = (assign10880_e5796 - locals.var_ef_nsubc);
        (assign10880_e5798, (locals.var_ef_nsubp_dn0 - locals.var_ef_nsubc_dn0), (locals.var_ef_nsubp_dn2 - locals.var_ef_nsubc_dn2), (locals.var_ef_nsubp_dn4 - locals.var_ef_nsubc_dn4), (locals.var_ef_nsubp_dn5 - locals.var_ef_nsubc_dn5), (locals.var_ef_nsubp_dn6 - locals.var_ef_nsubc_dn6), (locals.var_ef_nsubp_dn7 - locals.var_ef_nsubc_dn7), (locals.var_ef_nsubp_dn8 - locals.var_ef_nsubc_dn8), (locals.var_ef_nsubp_dn9 - locals.var_ef_nsubc_dn9), (locals.var_ef_nsubp_dn10 - locals.var_ef_nsubc_dn10), (locals.var_ef_nsubp_dn13 - locals.var_ef_nsubc_dn13),)
    } else {
        (locals.var_ef_nsubp, locals.var_ef_nsubp_dn0, locals.var_ef_nsubp_dn2, locals.var_ef_nsubp_dn4, locals.var_ef_nsubp_dn5, locals.var_ef_nsubp_dn6, locals.var_ef_nsubp_dn7, locals.var_ef_nsubp_dn8, locals.var_ef_nsubp_dn9, locals.var_ef_nsubp_dn10, locals.var_ef_nsubp_dn13,)
    }
};
        locals.var_ef_nsubp = assign10880_e5800;
        locals.var_ef_nsubp_dn0 = assign10880_e5800_d_n0;
        locals.var_ef_nsubp_dn2 = assign10880_e5800_d_n2;
        locals.var_ef_nsubp_dn4 = assign10880_e5800_d_n4;
        locals.var_ef_nsubp_dn5 = assign10880_e5800_d_n5;
        locals.var_ef_nsubp_dn6 = assign10880_e5800_d_n6;
        locals.var_ef_nsubp_dn7 = assign10880_e5800_d_n7;
        locals.var_ef_nsubp_dn8 = assign10880_e5800_d_n8;
        locals.var_ef_nsubp_dn9 = assign10880_e5800_d_n9;
        locals.var_ef_nsubp_dn10 = assign10880_e5800_d_n10;
        locals.var_ef_nsubp_dn13 = assign10880_e5800_d_n13;
        locals.var_ef_nsubp_rv = 0.0;

        let (assign10890_e5808, assign10890_e5808_d_n0, assign10890_e5808_d_n2, assign10890_e5808_d_n4, assign10890_e5808_d_n5, assign10890_e5808_d_n6, assign10890_e5808_d_n7, assign10890_e5808_d_n8, assign10890_e5808_d_n9, assign10890_e5808_d_n10, assign10890_e5808_d_n13,) = {
    if (locals.var_guard253 != 0.0) {
        let assign10890_e5804: f64 = (locals.var_npexte + locals.var_mks_nsubcdfm);
        let assign10890_e5806: f64 = (assign10890_e5804 - locals.var_ef_nsubc);
        (assign10890_e5806, (locals.var_npexte_dn0 - locals.var_ef_nsubc_dn0), (locals.var_npexte_dn2 - locals.var_ef_nsubc_dn2), (locals.var_npexte_dn4 - locals.var_ef_nsubc_dn4), (locals.var_npexte_dn5 - locals.var_ef_nsubc_dn5), (locals.var_npexte_dn6 - locals.var_ef_nsubc_dn6), (locals.var_npexte_dn7 - locals.var_ef_nsubc_dn7), (locals.var_npexte_dn8 - locals.var_ef_nsubc_dn8), (locals.var_npexte_dn9 - locals.var_ef_nsubc_dn9), (locals.var_npexte_dn10 - locals.var_ef_nsubc_dn10), (locals.var_npexte_dn13 - locals.var_ef_nsubc_dn13),)
    } else {
        (locals.var_npexte, locals.var_npexte_dn0, locals.var_npexte_dn2, locals.var_npexte_dn4, locals.var_npexte_dn5, locals.var_npexte_dn6, locals.var_npexte_dn7, locals.var_npexte_dn8, locals.var_npexte_dn9, locals.var_npexte_dn10, locals.var_npexte_dn13,)
    }
};
        locals.var_npexte = assign10890_e5808;
        locals.var_npexte_dn0 = assign10890_e5808_d_n0;
        locals.var_npexte_dn2 = assign10890_e5808_d_n2;
        locals.var_npexte_dn4 = assign10890_e5808_d_n4;
        locals.var_npexte_dn5 = assign10890_e5808_d_n5;
        locals.var_npexte_dn6 = assign10890_e5808_d_n6;
        locals.var_npexte_dn7 = assign10890_e5808_d_n7;
        locals.var_npexte_dn8 = assign10890_e5808_d_n8;
        locals.var_npexte_dn9 = assign10890_e5808_d_n9;
        locals.var_npexte_dn10 = assign10890_e5808_d_n10;
        locals.var_npexte_dn13 = assign10890_e5808_d_n13;
        locals.var_npexte_rv = 0.0;

        let (assign10900_e5812, assign10900_e5812_d_n0, assign10900_e5812_d_n2, assign10900_e5812_d_n4, assign10900_e5812_d_n5, assign10900_e5812_d_n6, assign10900_e5812_d_n7, assign10900_e5812_d_n8, assign10900_e5812_d_n9, assign10900_e5812_d_n10, assign10900_e5812_d_n13,) = {
    if (locals.var_guard253 != 0.0) {
        (locals.var_mks_nsubcdfm, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ef_nsubc, locals.var_ef_nsubc_dn0, locals.var_ef_nsubc_dn2, locals.var_ef_nsubc_dn4, locals.var_ef_nsubc_dn5, locals.var_ef_nsubc_dn6, locals.var_ef_nsubc_dn7, locals.var_ef_nsubc_dn8, locals.var_ef_nsubc_dn9, locals.var_ef_nsubc_dn10, locals.var_ef_nsubc_dn13,)
    }
};
        locals.var_ef_nsubc = assign10900_e5812;
        locals.var_ef_nsubc_dn0 = assign10900_e5812_d_n0;
        locals.var_ef_nsubc_dn2 = assign10900_e5812_d_n2;
        locals.var_ef_nsubc_dn4 = assign10900_e5812_d_n4;
        locals.var_ef_nsubc_dn5 = assign10900_e5812_d_n5;
        locals.var_ef_nsubc_dn6 = assign10900_e5812_d_n6;
        locals.var_ef_nsubc_dn7 = assign10900_e5812_d_n7;
        locals.var_ef_nsubc_dn8 = assign10900_e5812_d_n8;
        locals.var_ef_nsubc_dn9 = assign10900_e5812_d_n9;
        locals.var_ef_nsubc_dn10 = assign10900_e5812_d_n10;
        locals.var_ef_nsubc_dn13 = assign10900_e5812_d_n13;
        locals.var_ef_nsubc_rv = 0.0;

        let assign10910_e5818: f64 = (locals.var_wg).powf(p.p163);
        let assign10910_e5819: f64 = (p.p162 / assign10910_e5818);
        let assign10910_e5820: f64 = (1.0 + assign10910_e5819);
        let assign10910_e5821: f64 = (locals.var_ef_mueph1 * assign10910_e5820);
        let assign10910_e5826: f64 = (locals.var_lg).powf(p.p165);
        let assign10910_e5827: f64 = (p.p164 / assign10910_e5826);
        let assign10910_e5828: f64 = (1.0 + assign10910_e5827);
        let assign10910_e5829: f64 = (assign10910_e5821 * assign10910_e5828);
        let assign10910_e5834: f64 = (locals.var_wlg).powf(p.p168);
        let assign10910_e5835: f64 = (p.p167 / assign10910_e5834);
        let assign10910_e5836: f64 = (1.0 + assign10910_e5835);
        let assign10910_e5837: f64 = (assign10910_e5829 * assign10910_e5836);
        locals.var_mueph = assign10910_e5837;
        locals.var_mueph_dn0 = (((locals.var_ef_mueph1_dn0 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn2 = (((locals.var_ef_mueph1_dn2 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn4 = (((locals.var_ef_mueph1_dn4 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn5 = (((locals.var_ef_mueph1_dn5 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn6 = (((locals.var_ef_mueph1_dn6 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn7 = (((locals.var_ef_mueph1_dn7 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn8 = (((locals.var_ef_mueph1_dn8 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn9 = (((locals.var_ef_mueph1_dn9 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn10 = (((locals.var_ef_mueph1_dn10 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn13 = (((locals.var_ef_mueph1_dn13 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_rv = 0.0;

        let assign10920_e5840: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard255 = assign10920_e5840;
        locals.var_guard255_rv = 0.0;

        let (assign10930_e5848, assign10930_e5848_d_n0, assign10930_e5848_d_n2, assign10930_e5848_d_n4, assign10930_e5848_d_n5, assign10930_e5848_d_n6, assign10930_e5848_d_n7, assign10930_e5848_d_n8, assign10930_e5848_d_n9, assign10930_e5848_d_n10, assign10930_e5848_d_n13,) = {
    if (locals.var_guard255 != 0.0) {
        let assign10930_e5845: f64 = (1.0 + locals.var_uc_muesti2);
        let assign10930_e5846: f64 = (1.0 / assign10930_e5845);
        (assign10930_e5846, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign10930_e5848;
        locals.var_t1_dn0 = assign10930_e5848_d_n0;
        locals.var_t1_dn2 = assign10930_e5848_d_n2;
        locals.var_t1_dn4 = assign10930_e5848_d_n4;
        locals.var_t1_dn5 = assign10930_e5848_d_n5;
        locals.var_t1_dn6 = assign10930_e5848_d_n6;
        locals.var_t1_dn7 = assign10930_e5848_d_n7;
        locals.var_t1_dn8 = assign10930_e5848_d_n8;
        locals.var_t1_dn9 = assign10930_e5848_d_n9;
        locals.var_t1_dn10 = assign10930_e5848_d_n10;
        locals.var_t1_dn13 = assign10930_e5848_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign10940_e5856, assign10940_e5856_d_n0, assign10940_e5856_d_n2, assign10940_e5856_d_n4, assign10940_e5856_d_n5, assign10940_e5856_d_n6, assign10940_e5856_d_n7, assign10940_e5856_d_n8, assign10940_e5856_d_n9, assign10940_e5856_d_n10, assign10940_e5856_d_n13,) = {
    if (locals.var_guard255 != 0.0) {
        let assign10940_e5852: f64 = (locals.var_uc_muesti1 / locals.var_lod_half);
        let assign10940_e5854: f64 = (assign10940_e5852).powf(locals.var_uc_muesti3);
        (assign10940_e5854, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn13) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn13) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign10940_e5856;
        locals.var_t2_dn0 = assign10940_e5856_d_n0;
        locals.var_t2_dn2 = assign10940_e5856_d_n2;
        locals.var_t2_dn4 = assign10940_e5856_d_n4;
        locals.var_t2_dn5 = assign10940_e5856_d_n5;
        locals.var_t2_dn6 = assign10940_e5856_d_n6;
        locals.var_t2_dn7 = assign10940_e5856_d_n7;
        locals.var_t2_dn8 = assign10940_e5856_d_n8;
        locals.var_t2_dn9 = assign10940_e5856_d_n9;
        locals.var_t2_dn10 = assign10940_e5856_d_n10;
        locals.var_t2_dn13 = assign10940_e5856_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign10950_e5864, assign10950_e5864_d_n0, assign10950_e5864_d_n2, assign10950_e5864_d_n4, assign10950_e5864_d_n5, assign10950_e5864_d_n6, assign10950_e5864_d_n7, assign10950_e5864_d_n8, assign10950_e5864_d_n9, assign10950_e5864_d_n10, assign10950_e5864_d_n13,) = {
    if (locals.var_guard255 != 0.0) {
        let assign10950_e5860: f64 = (locals.var_uc_muesti1 / locals.var_lod_half_ref);
        let assign10950_e5862: f64 = (assign10950_e5860).powf(locals.var_uc_muesti3);
        (assign10950_e5862, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn13) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn13) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign10950_e5864;
        locals.var_t3_dn0 = assign10950_e5864_d_n0;
        locals.var_t3_dn2 = assign10950_e5864_d_n2;
        locals.var_t3_dn4 = assign10950_e5864_d_n4;
        locals.var_t3_dn5 = assign10950_e5864_d_n5;
        locals.var_t3_dn6 = assign10950_e5864_d_n6;
        locals.var_t3_dn7 = assign10950_e5864_d_n7;
        locals.var_t3_dn8 = assign10950_e5864_d_n8;
        locals.var_t3_dn9 = assign10950_e5864_d_n9;
        locals.var_t3_dn10 = assign10950_e5864_d_n10;
        locals.var_t3_dn13 = assign10950_e5864_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign10960_e5880, assign10960_e5880_d_n0, assign10960_e5880_d_n2, assign10960_e5880_d_n4, assign10960_e5880_d_n5, assign10960_e5880_d_n6, assign10960_e5880_d_n7, assign10960_e5880_d_n8, assign10960_e5880_d_n9, assign10960_e5880_d_n10, assign10960_e5880_d_n13,) = {
    if (locals.var_guard255 != 0.0) {
        let assign10960_e5870: f64 = (locals.var_t1 * locals.var_t2);
        let assign10960_e5871: f64 = (1.0 + assign10960_e5870);
        let assign10960_e5872: f64 = (locals.var_mueph * assign10960_e5871);
        let assign10960_e5876: f64 = (locals.var_t1 * locals.var_t3);
        let assign10960_e5877: f64 = (1.0 + assign10960_e5876);
        let assign10960_e5878: f64 = (assign10960_e5872 / assign10960_e5877);
        (assign10960_e5878, (((((locals.var_mueph_dn0 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn2 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn4 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn5 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn6 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn7 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn8 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn9 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn10 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn13 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn13 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn13)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)))) / (assign10960_e5877 * assign10960_e5877)),)
    } else {
        (locals.var_mueph, locals.var_mueph_dn0, locals.var_mueph_dn2, locals.var_mueph_dn4, locals.var_mueph_dn5, locals.var_mueph_dn6, locals.var_mueph_dn7, locals.var_mueph_dn8, locals.var_mueph_dn9, locals.var_mueph_dn10, locals.var_mueph_dn13,)
    }
};
        locals.var_mueph = assign10960_e5880;
        locals.var_mueph_dn0 = assign10960_e5880_d_n0;
        locals.var_mueph_dn2 = assign10960_e5880_d_n2;
        locals.var_mueph_dn4 = assign10960_e5880_d_n4;
        locals.var_mueph_dn5 = assign10960_e5880_d_n5;
        locals.var_mueph_dn6 = assign10960_e5880_d_n6;
        locals.var_mueph_dn7 = assign10960_e5880_d_n7;
        locals.var_mueph_dn8 = assign10960_e5880_d_n8;
        locals.var_mueph_dn9 = assign10960_e5880_d_n9;
        locals.var_mueph_dn10 = assign10960_e5880_d_n10;
        locals.var_mueph_dn13 = assign10960_e5880_d_n13;
        locals.var_mueph_rv = 0.0;

        let assign10970_e5886: f64 = (locals.var_lg).powf(p.p176);
        let assign10970_e5887: f64 = (p.p173 / assign10970_e5886);
        let assign10970_e5888: f64 = (1.0 + assign10970_e5887);
        let assign10970_e5889: f64 = (p.p171 * assign10970_e5888);
        let assign10970_e5894: f64 = (locals.var_wg).powf(p.p175);
        let assign10970_e5895: f64 = (p.p174 / assign10970_e5894);
        let assign10970_e5896: f64 = (1.0 + assign10970_e5895);
        let assign10970_e5897: f64 = (assign10970_e5889 * assign10970_e5896);
        locals.var_muesr = assign10970_e5897;
        locals.var_muesr_rv = 0.0;

        let (assign11000_e5921, assign11000_e5921_d_n0, assign11000_e5921_d_n2, assign11000_e5921_d_n4, assign11000_e5921_d_n5, assign11000_e5921_d_n6, assign11000_e5921_d_n7, assign11000_e5921_d_n8, assign11000_e5921_d_n9, assign11000_e5921_d_n10, assign11000_e5921_d_n13,) = {
    if (locals.var_mueph < 1e-25) {
        (1e-25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mueph, locals.var_mueph_dn0, locals.var_mueph_dn2, locals.var_mueph_dn4, locals.var_mueph_dn5, locals.var_mueph_dn6, locals.var_mueph_dn7, locals.var_mueph_dn8, locals.var_mueph_dn9, locals.var_mueph_dn10, locals.var_mueph_dn13,)
    }
};
        locals.var_mueph = assign11000_e5921;
        locals.var_mueph_dn0 = assign11000_e5921_d_n0;
        locals.var_mueph_dn2 = assign11000_e5921_d_n2;
        locals.var_mueph_dn4 = assign11000_e5921_d_n4;
        locals.var_mueph_dn5 = assign11000_e5921_d_n5;
        locals.var_mueph_dn6 = assign11000_e5921_d_n6;
        locals.var_mueph_dn7 = assign11000_e5921_d_n7;
        locals.var_mueph_dn8 = assign11000_e5921_d_n8;
        locals.var_mueph_dn9 = assign11000_e5921_d_n9;
        locals.var_mueph_dn10 = assign11000_e5921_d_n10;
        locals.var_mueph_dn13 = assign11000_e5921_d_n13;
        locals.var_mueph_rv = 0.0;

        let (assign11010_e5927,) = {
    if (locals.var_muesr < 1e-25) {
        (1e-25,)
    } else {
        (locals.var_muesr,)
    }
};
        locals.var_muesr = assign11010_e5927;
        locals.var_muesr_rv = 0.0;

        let assign11020_e5930: f64 = (locals.var_lg).powf(p.p156);
        locals.var_t1 = assign11020_e5930;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign11030_e5933: f64 = (locals.var_uc_ndep * locals.var_t1);
        let assign11030_e5936: f64 = (locals.var_t1 + p.p155);
        let assign11030_e5937: f64 = (assign11030_e5933 / assign11030_e5936);
        let assign11030_e5939: f64 = (assign11030_e5937 / 1.034943e-10);
        locals.var_ndep_o_esi = assign11030_e5939;
        locals.var_ndep_o_esi_dn0 = (((((locals.var_uc_ndep * locals.var_t1_dn0) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn0)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn2 = (((((locals.var_uc_ndep * locals.var_t1_dn2) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn2)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn4 = (((((locals.var_uc_ndep * locals.var_t1_dn4) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn4)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn5 = (((((locals.var_uc_ndep * locals.var_t1_dn5) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn5)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn6 = (((((locals.var_uc_ndep * locals.var_t1_dn6) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn6)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn7 = (((((locals.var_uc_ndep * locals.var_t1_dn7) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn7)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn8 = (((((locals.var_uc_ndep * locals.var_t1_dn8) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn8)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn9 = (((((locals.var_uc_ndep * locals.var_t1_dn9) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn9)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn10 = (((((locals.var_uc_ndep * locals.var_t1_dn10) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn10)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn13 = (((((locals.var_uc_ndep * locals.var_t1_dn13) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn13)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_rv = 0.0;

        let assign11040_e5942: f64 = (locals.var_uc_ninv / 1.034943e-10);
        locals.var_ninv_o_esi = assign11040_e5942;
        locals.var_ninv_o_esi_rv = 0.0;

        let assign11050_e5948: f64 = (locals.var_lg).powf(p.p321);
        let assign11050_e5949: f64 = (p.p320 / assign11050_e5948);
        let assign11050_e5950: f64 = (1.0 + assign11050_e5949);
        let assign11050_e5951: f64 = (p.p319 * assign11050_e5950);
        let assign11050_e5956: f64 = (locals.var_wg).powf(p.p323);
        let assign11050_e5957: f64 = (p.p322 / assign11050_e5956);
        let assign11050_e5958: f64 = (1.0 + assign11050_e5957);
        let assign11050_e5959: f64 = (assign11050_e5951 * assign11050_e5958);
        locals.var_ninvd0 = assign11050_e5959;
        locals.var_ninvd0_rv = 0.0;

        let assign11060_e5964: f64 = (locals.var_lg).powf(p.p387);
        let assign11060_e5965: f64 = (p.p386 / assign11060_e5964);
        let assign11060_e5966: f64 = (1.0 + assign11060_e5965);
        let assign11060_e5971: f64 = (locals.var_wg).powf(p.p389);
        let assign11060_e5972: f64 = (p.p388 / assign11060_e5971);
        let assign11060_e5973: f64 = (1.0 + assign11060_e5972);
        let assign11060_e5974: f64 = (assign11060_e5966 * assign11060_e5973);
        locals.var_t1 = assign11060_e5974;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign11070_e5977: f64 = (p.p384 * locals.var_t1);
        locals.var_ninvd0cres = assign11070_e5977;
        locals.var_ninvd0cres_dn0 = (p.p384 * locals.var_t1_dn0);
        locals.var_ninvd0cres_dn2 = (p.p384 * locals.var_t1_dn2);
        locals.var_ninvd0cres_dn4 = (p.p384 * locals.var_t1_dn4);
        locals.var_ninvd0cres_dn5 = (p.p384 * locals.var_t1_dn5);
        locals.var_ninvd0cres_dn6 = (p.p384 * locals.var_t1_dn6);
        locals.var_ninvd0cres_dn7 = (p.p384 * locals.var_t1_dn7);
        locals.var_ninvd0cres_dn8 = (p.p384 * locals.var_t1_dn8);
        locals.var_ninvd0cres_dn9 = (p.p384 * locals.var_t1_dn9);
        locals.var_ninvd0cres_dn10 = (p.p384 * locals.var_t1_dn10);
        locals.var_ninvd0cres_dn13 = (p.p384 * locals.var_t1_dn13);
        locals.var_ninvd0cres_rv = 0.0;

        let assign11080_e5980: f64 = (p.p385 * locals.var_t1);
        locals.var_ninvd0hres = assign11080_e5980;
        locals.var_ninvd0hres_dn0 = (p.p385 * locals.var_t1_dn0);
        locals.var_ninvd0hres_dn2 = (p.p385 * locals.var_t1_dn2);
        locals.var_ninvd0hres_dn4 = (p.p385 * locals.var_t1_dn4);
        locals.var_ninvd0hres_dn5 = (p.p385 * locals.var_t1_dn5);
        locals.var_ninvd0hres_dn6 = (p.p385 * locals.var_t1_dn6);
        locals.var_ninvd0hres_dn7 = (p.p385 * locals.var_t1_dn7);
        locals.var_ninvd0hres_dn8 = (p.p385 * locals.var_t1_dn8);
        locals.var_ninvd0hres_dn9 = (p.p385 * locals.var_t1_dn9);
        locals.var_ninvd0hres_dn10 = (p.p385 * locals.var_t1_dn10);
        locals.var_ninvd0hres_dn13 = (p.p385 * locals.var_t1_dn13);
        locals.var_ninvd0hres_rv = 0.0;

        let assign11090_e5985: f64 = (locals.var_lgate + p.p121);
        let assign11090_e5987: f64 = (assign11090_e5985).powf(p.p122);
        let assign11090_e5988: f64 = (locals.var_mks_ll / assign11090_e5987);
        let assign11090_e5989: f64 = (p.p97 + assign11090_e5988);
        locals.var_dl = assign11090_e5989;
        locals.var_dl_rv = 0.0;

        let assign11100_e5994: f64 = (locals.var_lgate + p.p121);
        let assign11100_e5996: f64 = (assign11100_e5994).powf(p.p122);
        let assign11100_e5997: f64 = (locals.var_mks_ll / assign11100_e5996);
        let assign11100_e5998: f64 = (locals.var_uc_xldld + assign11100_e5997);
        locals.var_dlld = assign11100_e5998;
        locals.var_dlld_rv = 0.0;

        let assign11110_e6003: f64 = (locals.var_wgate + p.p128);
        let assign11110_e6005: f64 = (assign11110_e6003).powf(p.p129);
        let assign11110_e6006: f64 = (locals.var_mks_wl / assign11110_e6005);
        let assign11110_e6007: f64 = (p.p114 + assign11110_e6006);
        locals.var_dw = assign11110_e6007;
        locals.var_dw_rv = 0.0;

        let assign11120_e6012: f64 = (locals.var_wgate + p.p128);
        let assign11120_e6014: f64 = (assign11120_e6012).powf(p.p129);
        let assign11120_e6015: f64 = (locals.var_mks_wl / assign11120_e6014);
        let assign11120_e6016: f64 = (p.p295 + assign11120_e6015);
        locals.var_dwld = assign11120_e6016;
        locals.var_dwld_rv = 0.0;

        let assign11130_e6021: f64 = (locals.var_wgate + p.p128);
        let assign11130_e6023: f64 = (assign11130_e6021).powf(p.p129);
        let assign11130_e6024: f64 = (locals.var_mks_wl / assign11130_e6023);
        let assign11130_e6025: f64 = (p.p115 + assign11130_e6024);
        locals.var_dwcv = assign11130_e6025;
        locals.var_dwcv_rv = 0.0;

        let assign11140_e6029: f64 = (locals.var_dl + locals.var_dlld);
        let assign11140_e6030: f64 = (locals.var_lgate - assign11140_e6029);
        locals.var_leff = assign11140_e6030;
        locals.var_leff_rv = 0.0;

        let assign11170_e6042: f64 = (locals.var_wlg).powf(p.p125);
        let assign11170_e6043: f64 = (p.p124 / assign11170_e6042);
        let assign11170_e6044: f64 = (locals.var_lgate + assign11170_e6043);
        locals.var_lgatesm = assign11170_e6044;
        locals.var_lgatesm_rv = 0.0;

        let assign11180_e6048: f64 = (locals.var_wlg).powf(p.p127);
        let assign11180_e6049: f64 = (locals.var_uc_wl2 / assign11180_e6048);
        locals.var_dvthsm = assign11180_e6049;
        locals.var_dvthsm_rv = 0.0;

        let assign11190_e6054: f64 = (locals.var_lgatesm * 1000000.0);
        let assign11190_e6056: f64 = (assign11190_e6054).powf(p.p207);
        let assign11190_e6057: f64 = (p.p206 / assign11190_e6056);
        let assign11190_e6058: f64 = (1.0 + assign11190_e6057);
        locals.var_t1 = assign11190_e6058;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign11200_e6063: f64 = (locals.var_wg).powf(p.p209);
        let assign11200_e6064: f64 = (p.p208 / assign11200_e6063);
        let assign11200_e6065: f64 = (1.0 + assign11200_e6064);
        locals.var_t2 = assign11200_e6065;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn7 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn13 = 0.0;
        locals.var_t2_rv = 0.0;

        let assign11210_e6068: f64 = (locals.var_uc_wsti * locals.var_t1);
        let assign11210_e6070: f64 = (assign11210_e6068 * locals.var_t2);
        locals.var_uc_wsti = assign11210_e6070;
        locals.var_uc_wsti_dn0 = ((((locals.var_uc_wsti_dn0 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn0)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn0));
        locals.var_uc_wsti_dn2 = ((((locals.var_uc_wsti_dn2 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn2)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn2));
        locals.var_uc_wsti_dn4 = ((((locals.var_uc_wsti_dn4 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn4)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn4));
        locals.var_uc_wsti_dn5 = ((((locals.var_uc_wsti_dn5 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn5)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn5));
        locals.var_uc_wsti_dn6 = ((((locals.var_uc_wsti_dn6 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn6)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn6));
        locals.var_uc_wsti_dn7 = ((((locals.var_uc_wsti_dn7 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn7)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn7));
        locals.var_uc_wsti_dn8 = ((((locals.var_uc_wsti_dn8 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn8)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn8));
        locals.var_uc_wsti_dn9 = ((((locals.var_uc_wsti_dn9 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn9)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn9));
        locals.var_uc_wsti_dn10 = ((((locals.var_uc_wsti_dn10 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn10)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn10));
        locals.var_uc_wsti_dn13 = ((((locals.var_uc_wsti_dn13 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn13)) * locals.var_t2) + (assign11210_e6068 * locals.var_t2_dn13));
        locals.var_uc_wsti_rv = 0.0;

        let assign11220_e6074: f64 = (2.0 * locals.var_dw);
        let assign11220_e6075: f64 = (locals.var_wgate - assign11220_e6074);
        locals.var_weff = assign11220_e6075;
        locals.var_weff_rv = 0.0;

        let assign11230_e6079: f64 = (2.0 * locals.var_dwld);
        let assign11230_e6080: f64 = (locals.var_wgate - assign11230_e6079);
        locals.var_weff_ld = assign11230_e6080;
        locals.var_weff_ld_rv = 0.0;

        let assign11240_e6084: f64 = (2.0 * locals.var_dwcv);
        let assign11240_e6085: f64 = (locals.var_wgate - assign11240_e6084);
        locals.var_weff_cv = assign11240_e6085;
        locals.var_weff_cv_rv = 0.0;

        let assign11310_e6109: f64 = (locals.var_weff * p.p7);
        locals.var_weff_nf = assign11310_e6109;
        locals.var_weff_nf_rv = 0.0;

        let assign11320_e6112: f64 = (locals.var_weff_cv * p.p7);
        locals.var_weffcv_nf = assign11320_e6112;
        locals.var_weffcv_nf_rv = 0.0;

        let assign11330_e6118: f64 = (locals.var_wg).powf(p.p143);
        let assign11330_e6119: f64 = (p.p142 / assign11330_e6118);
        let assign11330_e6120: f64 = (1.0 + assign11330_e6119);
        let assign11330_e6121: f64 = (locals.var_ef_nsubp * assign11330_e6120);
        locals.var_nsubpp = assign11330_e6121;
        locals.var_nsubpp_dn0 = (locals.var_ef_nsubp_dn0 * assign11330_e6120);
        locals.var_nsubpp_dn2 = (locals.var_ef_nsubp_dn2 * assign11330_e6120);
        locals.var_nsubpp_dn4 = (locals.var_ef_nsubp_dn4 * assign11330_e6120);
        locals.var_nsubpp_dn5 = (locals.var_ef_nsubp_dn5 * assign11330_e6120);
        locals.var_nsubpp_dn6 = (locals.var_ef_nsubp_dn6 * assign11330_e6120);
        locals.var_nsubpp_dn7 = (locals.var_ef_nsubp_dn7 * assign11330_e6120);
        locals.var_nsubpp_dn8 = (locals.var_ef_nsubp_dn8 * assign11330_e6120);
        locals.var_nsubpp_dn9 = (locals.var_ef_nsubp_dn9 * assign11330_e6120);
        locals.var_nsubpp_dn10 = (locals.var_ef_nsubp_dn10 * assign11330_e6120);
        locals.var_nsubpp_dn13 = (locals.var_ef_nsubp_dn13 * assign11330_e6120);
        locals.var_nsubpp_rv = 0.0;

        let assign11340_e6127: f64 = (locals.var_wg).powf(p.p234);
        let assign11340_e6128: f64 = (p.p233 / assign11340_e6127);
        let assign11340_e6129: f64 = (1.0 + assign11340_e6128);
        let assign11340_e6130: f64 = (locals.var_ef_nsubc * assign11340_e6129);
        locals.var_ef_nsubc = assign11340_e6130;
        locals.var_ef_nsubc_dn0 = (locals.var_ef_nsubc_dn0 * assign11340_e6129);
        locals.var_ef_nsubc_dn2 = (locals.var_ef_nsubc_dn2 * assign11340_e6129);
        locals.var_ef_nsubc_dn4 = (locals.var_ef_nsubc_dn4 * assign11340_e6129);
        locals.var_ef_nsubc_dn5 = (locals.var_ef_nsubc_dn5 * assign11340_e6129);
        locals.var_ef_nsubc_dn6 = (locals.var_ef_nsubc_dn6 * assign11340_e6129);
        locals.var_ef_nsubc_dn7 = (locals.var_ef_nsubc_dn7 * assign11340_e6129);
        locals.var_ef_nsubc_dn8 = (locals.var_ef_nsubc_dn8 * assign11340_e6129);
        locals.var_ef_nsubc_dn9 = (locals.var_ef_nsubc_dn9 * assign11340_e6129);
        locals.var_ef_nsubc_dn10 = (locals.var_ef_nsubc_dn10 * assign11340_e6129);
        locals.var_ef_nsubc_dn13 = (locals.var_ef_nsubc_dn13 * assign11340_e6129);
        locals.var_ef_nsubc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign11350_e6133: f64 = (locals.var_ef_nsubc * 1e-6);
        locals.var_t1 = assign11350_e6133;
        locals.var_t1_dn0 = (locals.var_ef_nsubc_dn0 * 1e-6);
        locals.var_t1_dn2 = (locals.var_ef_nsubc_dn2 * 1e-6);
        locals.var_t1_dn4 = (locals.var_ef_nsubc_dn4 * 1e-6);
        locals.var_t1_dn5 = (locals.var_ef_nsubc_dn5 * 1e-6);
        locals.var_t1_dn6 = (locals.var_ef_nsubc_dn6 * 1e-6);
        locals.var_t1_dn7 = (locals.var_ef_nsubc_dn7 * 1e-6);
        locals.var_t1_dn8 = (locals.var_ef_nsubc_dn8 * 1e-6);
        locals.var_t1_dn9 = (locals.var_ef_nsubc_dn9 * 1e-6);
        locals.var_t1_dn10 = (locals.var_ef_nsubc_dn10 * 1e-6);
        locals.var_t1_dn13 = (locals.var_ef_nsubc_dn13 * 1e-6);
        locals.var_t1_rv = 0.0;

        let assign11360_e6136: f64 = (locals.var_nsubpp * 1e-6);
        locals.var_t2 = assign11360_e6136;
        locals.var_t2_dn0 = (locals.var_nsubpp_dn0 * 1e-6);
        locals.var_t2_dn2 = (locals.var_nsubpp_dn2 * 1e-6);
        locals.var_t2_dn4 = (locals.var_nsubpp_dn4 * 1e-6);
        locals.var_t2_dn5 = (locals.var_nsubpp_dn5 * 1e-6);
        locals.var_t2_dn6 = (locals.var_nsubpp_dn6 * 1e-6);
        locals.var_t2_dn7 = (locals.var_nsubpp_dn7 * 1e-6);
        locals.var_t2_dn8 = (locals.var_nsubpp_dn8 * 1e-6);
        locals.var_t2_dn9 = (locals.var_nsubpp_dn9 * 1e-6);
        locals.var_t2_dn10 = (locals.var_nsubpp_dn10 * 1e-6);
        locals.var_t2_dn13 = (locals.var_nsubpp_dn13 * 1e-6);
        locals.var_t2_rv = 0.0;

        let assign11380_e6144: f64 = if locals.var_t1 < 1000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign11380_e6144;
        locals.var_guard263_rv = 0.0;

        let (assign11390_e6148, assign11390_e6148_d_n0, assign11390_e6148_d_n2, assign11390_e6148_d_n4, assign11390_e6148_d_n5, assign11390_e6148_d_n6, assign11390_e6148_d_n7, assign11390_e6148_d_n8, assign11390_e6148_d_n9, assign11390_e6148_d_n10, assign11390_e6148_d_n13,) = {
    if (locals.var_guard263 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign11390_e6148;
        locals.var_t1_dn0 = assign11390_e6148_d_n0;
        locals.var_t1_dn2 = assign11390_e6148_d_n2;
        locals.var_t1_dn4 = assign11390_e6148_d_n4;
        locals.var_t1_dn5 = assign11390_e6148_d_n5;
        locals.var_t1_dn6 = assign11390_e6148_d_n6;
        locals.var_t1_dn7 = assign11390_e6148_d_n7;
        locals.var_t1_dn8 = assign11390_e6148_d_n8;
        locals.var_t1_dn9 = assign11390_e6148_d_n9;
        locals.var_t1_dn10 = assign11390_e6148_d_n10;
        locals.var_t1_dn13 = assign11390_e6148_d_n13;
        locals.var_t1_rv = 0.0;

        let assign11400_e6151: f64 = (locals.var_t1 / 1e-6);
        locals.var_ef_nsubc = assign11400_e6151;
        locals.var_ef_nsubc_dn0 = (locals.var_t1_dn0 / 1e-6);
        locals.var_ef_nsubc_dn2 = (locals.var_t1_dn2 / 1e-6);
        locals.var_ef_nsubc_dn4 = (locals.var_t1_dn4 / 1e-6);
        locals.var_ef_nsubc_dn5 = (locals.var_t1_dn5 / 1e-6);
        locals.var_ef_nsubc_dn6 = (locals.var_t1_dn6 / 1e-6);
        locals.var_ef_nsubc_dn7 = (locals.var_t1_dn7 / 1e-6);
        locals.var_ef_nsubc_dn8 = (locals.var_t1_dn8 / 1e-6);
        locals.var_ef_nsubc_dn9 = (locals.var_t1_dn9 / 1e-6);
        locals.var_ef_nsubc_dn10 = (locals.var_t1_dn10 / 1e-6);
        locals.var_ef_nsubc_dn13 = (locals.var_t1_dn13 / 1e-6);
        locals.var_ef_nsubc_rv = 0.0;

        let assign11420_e6159: f64 = if locals.var_t2 < 1000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard265 = assign11420_e6159;
        locals.var_guard265_rv = 0.0;

        let (assign11430_e6163, assign11430_e6163_d_n0, assign11430_e6163_d_n2, assign11430_e6163_d_n4, assign11430_e6163_d_n5, assign11430_e6163_d_n6, assign11430_e6163_d_n7, assign11430_e6163_d_n8, assign11430_e6163_d_n9, assign11430_e6163_d_n10, assign11430_e6163_d_n13,) = {
    if (locals.var_guard265 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign11430_e6163;
        locals.var_t2_dn0 = assign11430_e6163_d_n0;
        locals.var_t2_dn2 = assign11430_e6163_d_n2;
        locals.var_t2_dn4 = assign11430_e6163_d_n4;
        locals.var_t2_dn5 = assign11430_e6163_d_n5;
        locals.var_t2_dn6 = assign11430_e6163_d_n6;
        locals.var_t2_dn7 = assign11430_e6163_d_n7;
        locals.var_t2_dn8 = assign11430_e6163_d_n8;
        locals.var_t2_dn9 = assign11430_e6163_d_n9;
        locals.var_t2_dn10 = assign11430_e6163_d_n10;
        locals.var_t2_dn13 = assign11430_e6163_d_n13;
        locals.var_t2_rv = 0.0;

        let assign11440_e6166: f64 = (locals.var_t2 / 1e-6);
        locals.var_nsubpp = assign11440_e6166;
        locals.var_nsubpp_dn0 = (locals.var_t2_dn0 / 1e-6);
        locals.var_nsubpp_dn2 = (locals.var_t2_dn2 / 1e-6);
        locals.var_nsubpp_dn4 = (locals.var_t2_dn4 / 1e-6);
        locals.var_nsubpp_dn5 = (locals.var_t2_dn5 / 1e-6);
        locals.var_nsubpp_dn6 = (locals.var_t2_dn6 / 1e-6);
        locals.var_nsubpp_dn7 = (locals.var_t2_dn7 / 1e-6);
        locals.var_nsubpp_dn8 = (locals.var_t2_dn8 / 1e-6);
        locals.var_nsubpp_dn9 = (locals.var_t2_dn9 / 1e-6);
        locals.var_nsubpp_dn10 = (locals.var_t2_dn10 / 1e-6);
        locals.var_nsubpp_dn13 = (locals.var_t2_dn13 / 1e-6);
        locals.var_nsubpp_rv = 0.0;

        let assign11450_e6169: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard266 = assign11450_e6169;
        locals.var_guard266_rv = 0.0;

        let (assign11460_e6177, assign11460_e6177_d_n0, assign11460_e6177_d_n2, assign11460_e6177_d_n4, assign11460_e6177_d_n5, assign11460_e6177_d_n6, assign11460_e6177_d_n7, assign11460_e6177_d_n8, assign11460_e6177_d_n9, assign11460_e6177_d_n10, assign11460_e6177_d_n13,) = {
    if (locals.var_guard266 != 0.0) {
        let assign11460_e6174: f64 = (1.0 + locals.var_uc_nsubpsti2);
        let assign11460_e6175: f64 = (1.0 / assign11460_e6174);
        (assign11460_e6175, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign11460_e6177;
        locals.var_t1_dn0 = assign11460_e6177_d_n0;
        locals.var_t1_dn2 = assign11460_e6177_d_n2;
        locals.var_t1_dn4 = assign11460_e6177_d_n4;
        locals.var_t1_dn5 = assign11460_e6177_d_n5;
        locals.var_t1_dn6 = assign11460_e6177_d_n6;
        locals.var_t1_dn7 = assign11460_e6177_d_n7;
        locals.var_t1_dn8 = assign11460_e6177_d_n8;
        locals.var_t1_dn9 = assign11460_e6177_d_n9;
        locals.var_t1_dn10 = assign11460_e6177_d_n10;
        locals.var_t1_dn13 = assign11460_e6177_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign11470_e6185, assign11470_e6185_d_n0, assign11470_e6185_d_n2, assign11470_e6185_d_n4, assign11470_e6185_d_n5, assign11470_e6185_d_n6, assign11470_e6185_d_n7, assign11470_e6185_d_n8, assign11470_e6185_d_n9, assign11470_e6185_d_n10, assign11470_e6185_d_n13,) = {
    if (locals.var_guard266 != 0.0) {
        let assign11470_e6181: f64 = (locals.var_uc_nsubpsti1 / locals.var_lod_half);
        let assign11470_e6183: f64 = (assign11470_e6181).powf(locals.var_uc_nsubpsti3);
        (assign11470_e6183, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11470_e6181).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn13) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11470_e6183 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn13) / (locals.var_lod_half * locals.var_lod_half))) / assign11470_e6181))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign11470_e6185;
        locals.var_t2_dn0 = assign11470_e6185_d_n0;
        locals.var_t2_dn2 = assign11470_e6185_d_n2;
        locals.var_t2_dn4 = assign11470_e6185_d_n4;
        locals.var_t2_dn5 = assign11470_e6185_d_n5;
        locals.var_t2_dn6 = assign11470_e6185_d_n6;
        locals.var_t2_dn7 = assign11470_e6185_d_n7;
        locals.var_t2_dn8 = assign11470_e6185_d_n8;
        locals.var_t2_dn9 = assign11470_e6185_d_n9;
        locals.var_t2_dn10 = assign11470_e6185_d_n10;
        locals.var_t2_dn13 = assign11470_e6185_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign11480_e6193, assign11480_e6193_d_n0, assign11480_e6193_d_n2, assign11480_e6193_d_n4, assign11480_e6193_d_n5, assign11480_e6193_d_n6, assign11480_e6193_d_n7, assign11480_e6193_d_n8, assign11480_e6193_d_n9, assign11480_e6193_d_n10, assign11480_e6193_d_n13,) = {
    if (locals.var_guard266 != 0.0) {
        let assign11480_e6189: f64 = (locals.var_uc_nsubpsti1 / locals.var_lod_half_ref);
        let assign11480_e6191: f64 = (assign11480_e6189).powf(locals.var_uc_nsubpsti3);
        (assign11480_e6191, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11480_e6189).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn13) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11480_e6191 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn13) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11480_e6189))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign11480_e6193;
        locals.var_t3_dn0 = assign11480_e6193_d_n0;
        locals.var_t3_dn2 = assign11480_e6193_d_n2;
        locals.var_t3_dn4 = assign11480_e6193_d_n4;
        locals.var_t3_dn5 = assign11480_e6193_d_n5;
        locals.var_t3_dn6 = assign11480_e6193_d_n6;
        locals.var_t3_dn7 = assign11480_e6193_d_n7;
        locals.var_t3_dn8 = assign11480_e6193_d_n8;
        locals.var_t3_dn9 = assign11480_e6193_d_n9;
        locals.var_t3_dn10 = assign11480_e6193_d_n10;
        locals.var_t3_dn13 = assign11480_e6193_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign11490_e6209, assign11490_e6209_d_n0, assign11490_e6209_d_n2, assign11490_e6209_d_n4, assign11490_e6209_d_n5, assign11490_e6209_d_n6, assign11490_e6209_d_n7, assign11490_e6209_d_n8, assign11490_e6209_d_n9, assign11490_e6209_d_n10, assign11490_e6209_d_n13,) = {
    if (locals.var_guard266 != 0.0) {
        let assign11490_e6199: f64 = (locals.var_t1 * locals.var_t2);
        let assign11490_e6200: f64 = (1.0 + assign11490_e6199);
        let assign11490_e6201: f64 = (locals.var_nsubpp * assign11490_e6200);
        let assign11490_e6205: f64 = (locals.var_t1 * locals.var_t3);
        let assign11490_e6206: f64 = (1.0 + assign11490_e6205);
        let assign11490_e6207: f64 = (assign11490_e6201 / assign11490_e6206);
        (assign11490_e6207, (((((locals.var_nsubpp_dn0 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn2 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn4 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn5 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn6 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn7 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn8 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn9 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn10 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign11490_e6206 * assign11490_e6206)), (((((locals.var_nsubpp_dn13 * assign11490_e6200) + (locals.var_nsubpp * ((locals.var_t1_dn13 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn13)))) * assign11490_e6206) - (assign11490_e6201 * ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)))) / (assign11490_e6206 * assign11490_e6206)),)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn4, locals.var_nsubps_dn5, locals.var_nsubps_dn6, locals.var_nsubps_dn7, locals.var_nsubps_dn8, locals.var_nsubps_dn9, locals.var_nsubps_dn10, locals.var_nsubps_dn13,)
    }
};
        locals.var_nsubps = assign11490_e6209;
        locals.var_nsubps_dn0 = assign11490_e6209_d_n0;
        locals.var_nsubps_dn2 = assign11490_e6209_d_n2;
        locals.var_nsubps_dn4 = assign11490_e6209_d_n4;
        locals.var_nsubps_dn5 = assign11490_e6209_d_n5;
        locals.var_nsubps_dn6 = assign11490_e6209_d_n6;
        locals.var_nsubps_dn7 = assign11490_e6209_d_n7;
        locals.var_nsubps_dn8 = assign11490_e6209_d_n8;
        locals.var_nsubps_dn9 = assign11490_e6209_d_n9;
        locals.var_nsubps_dn10 = assign11490_e6209_d_n10;
        locals.var_nsubps_dn13 = assign11490_e6209_d_n13;
        locals.var_nsubps_rv = 0.0;

        let (assign11500_e6214, assign11500_e6214_d_n0, assign11500_e6214_d_n2, assign11500_e6214_d_n4, assign11500_e6214_d_n5, assign11500_e6214_d_n6, assign11500_e6214_d_n7, assign11500_e6214_d_n8, assign11500_e6214_d_n9, assign11500_e6214_d_n10, assign11500_e6214_d_n13,) = {
    if (locals.var_guard266 == 0.0) {
        (locals.var_nsubpp, locals.var_nsubpp_dn0, locals.var_nsubpp_dn2, locals.var_nsubpp_dn4, locals.var_nsubpp_dn5, locals.var_nsubpp_dn6, locals.var_nsubpp_dn7, locals.var_nsubpp_dn8, locals.var_nsubpp_dn9, locals.var_nsubpp_dn10, locals.var_nsubpp_dn13,)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn4, locals.var_nsubps_dn5, locals.var_nsubps_dn6, locals.var_nsubps_dn7, locals.var_nsubps_dn8, locals.var_nsubps_dn9, locals.var_nsubps_dn10, locals.var_nsubps_dn13,)
    }
};
        locals.var_nsubps = assign11500_e6214;
        locals.var_nsubps_dn0 = assign11500_e6214_d_n0;
        locals.var_nsubps_dn2 = assign11500_e6214_d_n2;
        locals.var_nsubps_dn4 = assign11500_e6214_d_n4;
        locals.var_nsubps_dn5 = assign11500_e6214_d_n5;
        locals.var_nsubps_dn6 = assign11500_e6214_d_n6;
        locals.var_nsubps_dn7 = assign11500_e6214_d_n7;
        locals.var_nsubps_dn8 = assign11500_e6214_d_n8;
        locals.var_nsubps_dn9 = assign11500_e6214_d_n9;
        locals.var_nsubps_dn10 = assign11500_e6214_d_n10;
        locals.var_nsubps_dn13 = assign11500_e6214_d_n13;
        locals.var_nsubps_rv = 0.0;

        let assign11510_e6221: f64 = if ((locals.var_lgate > p.p140) || (p.p140 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard267 = assign11510_e6221;
        locals.var_guard267_rv = 0.0;

        let (assign11520_e6235, assign11520_e6235_d_n0, assign11520_e6235_d_n2, assign11520_e6235_d_n4, assign11520_e6235_d_n5, assign11520_e6235_d_n6, assign11520_e6235_d_n7, assign11520_e6235_d_n8, assign11520_e6235_d_n9, assign11520_e6235_d_n10, assign11520_e6235_d_n13,) = {
    if (locals.var_guard267 != 0.0) {
        let assign11520_e6226: f64 = (locals.var_lgate - p.p140);
        let assign11520_e6227: f64 = (locals.var_ef_nsubc * assign11520_e6226);
        let assign11520_e6230: f64 = (locals.var_nsubps * p.p140);
        let assign11520_e6231: f64 = (assign11520_e6227 + assign11520_e6230);
        let assign11520_e6233: f64 = (assign11520_e6231 / locals.var_lgate);
        (assign11520_e6233, (((locals.var_ef_nsubc_dn0 * assign11520_e6226) + (locals.var_nsubps_dn0 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn2 * assign11520_e6226) + (locals.var_nsubps_dn2 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn4 * assign11520_e6226) + (locals.var_nsubps_dn4 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn5 * assign11520_e6226) + (locals.var_nsubps_dn5 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn6 * assign11520_e6226) + (locals.var_nsubps_dn6 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn7 * assign11520_e6226) + (locals.var_nsubps_dn7 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn8 * assign11520_e6226) + (locals.var_nsubps_dn8 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn9 * assign11520_e6226) + (locals.var_nsubps_dn9 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn10 * assign11520_e6226) + (locals.var_nsubps_dn10 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn13 * assign11520_e6226) + (locals.var_nsubps_dn13 * p.p140)) / locals.var_lgate),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn4, locals.var_nsub_dn5, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn8, locals.var_nsub_dn9, locals.var_nsub_dn10, locals.var_nsub_dn13,)
    }
};
        locals.var_nsub = assign11520_e6235;
        locals.var_nsub_dn0 = assign11520_e6235_d_n0;
        locals.var_nsub_dn2 = assign11520_e6235_d_n2;
        locals.var_nsub_dn4 = assign11520_e6235_d_n4;
        locals.var_nsub_dn5 = assign11520_e6235_d_n5;
        locals.var_nsub_dn6 = assign11520_e6235_d_n6;
        locals.var_nsub_dn7 = assign11520_e6235_d_n7;
        locals.var_nsub_dn8 = assign11520_e6235_d_n8;
        locals.var_nsub_dn9 = assign11520_e6235_d_n9;
        locals.var_nsub_dn10 = assign11520_e6235_d_n10;
        locals.var_nsub_dn13 = assign11520_e6235_d_n13;
        locals.var_nsub_rv = 0.0;

        let (assign11530_e6250, assign11530_e6250_d_n0, assign11530_e6250_d_n2, assign11530_e6250_d_n4, assign11530_e6250_d_n5, assign11530_e6250_d_n6, assign11530_e6250_d_n7, assign11530_e6250_d_n8, assign11530_e6250_d_n9, assign11530_e6250_d_n10, assign11530_e6250_d_n13,) = {
    if (locals.var_guard267 == 0.0) {
        let assign11530_e6241: f64 = (locals.var_nsubps - locals.var_ef_nsubc);
        let assign11530_e6244: f64 = (p.p140 - locals.var_lgate);
        let assign11530_e6245: f64 = (assign11530_e6241 * assign11530_e6244);
        let assign11530_e6247: f64 = (assign11530_e6245 / p.p140);
        let assign11530_e6248: f64 = (locals.var_nsubps + assign11530_e6247);
        (assign11530_e6248, (locals.var_nsubps_dn0 + (((locals.var_nsubps_dn0 - locals.var_ef_nsubc_dn0) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn2 + (((locals.var_nsubps_dn2 - locals.var_ef_nsubc_dn2) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn4 + (((locals.var_nsubps_dn4 - locals.var_ef_nsubc_dn4) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn5 + (((locals.var_nsubps_dn5 - locals.var_ef_nsubc_dn5) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn6 + (((locals.var_nsubps_dn6 - locals.var_ef_nsubc_dn6) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn7 + (((locals.var_nsubps_dn7 - locals.var_ef_nsubc_dn7) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn8 + (((locals.var_nsubps_dn8 - locals.var_ef_nsubc_dn8) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn9 + (((locals.var_nsubps_dn9 - locals.var_ef_nsubc_dn9) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn10 + (((locals.var_nsubps_dn10 - locals.var_ef_nsubc_dn10) * assign11530_e6244) / p.p140)), (locals.var_nsubps_dn13 + (((locals.var_nsubps_dn13 - locals.var_ef_nsubc_dn13) * assign11530_e6244) / p.p140)),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn4, locals.var_nsub_dn5, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn8, locals.var_nsub_dn9, locals.var_nsub_dn10, locals.var_nsub_dn13,)
    }
};
        locals.var_nsub = assign11530_e6250;
        locals.var_nsub_dn0 = assign11530_e6250_d_n0;
        locals.var_nsub_dn2 = assign11530_e6250_d_n2;
        locals.var_nsub_dn4 = assign11530_e6250_d_n4;
        locals.var_nsub_dn5 = assign11530_e6250_d_n5;
        locals.var_nsub_dn6 = assign11530_e6250_d_n6;
        locals.var_nsub_dn7 = assign11530_e6250_d_n7;
        locals.var_nsub_dn8 = assign11530_e6250_d_n8;
        locals.var_nsub_dn9 = assign11530_e6250_d_n9;
        locals.var_nsub_dn10 = assign11530_e6250_d_n10;
        locals.var_nsub_dn13 = assign11530_e6250_d_n13;
        locals.var_nsub_rv = 0.0;

        let assign11540_e6253: f64 = (0.5 * locals.var_lgate);
        let assign11540_e6255: f64 = (assign11540_e6253 - p.p140);
        locals.var_t3 = assign11540_e6255;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn4 = 0.0;
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn7 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn9 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn13 = 0.0;
        locals.var_t3_rv = 0.0;

        let assign11550_e6258: f64 = (locals.var_t3 - 1e-9);
        let assign11550_e6260: f64 = (assign11550_e6258 - 1e-10);
        locals.var_tmf1 = assign11550_e6260;
        locals.var_tmf1_dn0 = locals.var_t3_dn0;
        locals.var_tmf1_dn2 = locals.var_t3_dn2;
        locals.var_tmf1_dn4 = locals.var_t3_dn4;
        locals.var_tmf1_dn5 = locals.var_t3_dn5;
        locals.var_tmf1_dn6 = locals.var_t3_dn6;
        locals.var_tmf1_dn7 = locals.var_t3_dn7;
        locals.var_tmf1_dn8 = locals.var_t3_dn8;
        locals.var_tmf1_dn9 = locals.var_t3_dn9;
        locals.var_tmf1_dn10 = locals.var_t3_dn10;
        locals.var_tmf1_dn13 = locals.var_t3_dn13;
        locals.var_tmf1_rv = 0.0;

        let assign11560_e6263: f64 = (4.0 * 1e-9);
        let assign11560_e6265: f64 = (assign11560_e6263 * 1e-10);
        locals.var_tmf2 = assign11560_e6265;
        locals.var_tmf2_dn0 = 0.0;
        locals.var_tmf2_dn2 = 0.0;
        locals.var_tmf2_dn4 = 0.0;
        locals.var_tmf2_dn5 = 0.0;
        locals.var_tmf2_dn6 = 0.0;
        locals.var_tmf2_dn7 = 0.0;
        locals.var_tmf2_dn8 = 0.0;
        locals.var_tmf2_dn9 = 0.0;
        locals.var_tmf2_dn10 = 0.0;
        locals.var_tmf2_dn13 = 0.0;
        locals.var_tmf2_rv = 0.0;

        let (assign11570_e6272, assign11570_e6272_d_n0, assign11570_e6272_d_n2, assign11570_e6272_d_n4, assign11570_e6272_d_n5, assign11570_e6272_d_n6, assign11570_e6272_d_n7, assign11570_e6272_d_n8, assign11570_e6272_d_n9, assign11570_e6272_d_n10, assign11570_e6272_d_n13,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    } else {
        let assign11570_e6271: f64 = (-locals.var_tmf2);
        (assign11570_e6271, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
    }
};
        locals.var_tmf2 = assign11570_e6272;
        locals.var_tmf2_dn0 = assign11570_e6272_d_n0;
        locals.var_tmf2_dn2 = assign11570_e6272_d_n2;
        locals.var_tmf2_dn4 = assign11570_e6272_d_n4;
        locals.var_tmf2_dn5 = assign11570_e6272_d_n5;
        locals.var_tmf2_dn6 = assign11570_e6272_d_n6;
        locals.var_tmf2_dn7 = assign11570_e6272_d_n7;
        locals.var_tmf2_dn8 = assign11570_e6272_d_n8;
        locals.var_tmf2_dn9 = assign11570_e6272_d_n9;
        locals.var_tmf2_dn10 = assign11570_e6272_d_n10;
        locals.var_tmf2_dn13 = assign11570_e6272_d_n13;
        locals.var_tmf2_rv = 0.0;

        let assign11580_e6275: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign11580_e6277: f64 = (assign11580_e6275 + locals.var_tmf2);
        let assign11580_e6278: f64 = (assign11580_e6277).sqrt();
        locals.var_tmf2 = assign11580_e6278;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn9 = ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign11580_e6278));
        locals.var_tmf2_dn13 = ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign11580_e6278));
        locals.var_tmf2_rv = 0.0;

        let assign11590_e6283: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign11590_e6284: f64 = (1.0 + assign11590_e6283);
        let assign11590_e6285: f64 = (0.5 * assign11590_e6284);
        locals.var_t0 = assign11590_e6285;
        locals.var_t0_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn7 = (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn9 = (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn13 = (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_rv = 0.0;

        let assign11600_e6290: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign11600_e6291: f64 = (0.5 * assign11600_e6290);
        let assign11600_e6292: f64 = (1e-9 + assign11600_e6291);
        locals.var_t3 = assign11600_e6292;
        locals.var_t3_dn0 = (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0));
        locals.var_t3_dn2 = (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2));
        locals.var_t3_dn4 = (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4));
        locals.var_t3_dn5 = (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5));
        locals.var_t3_dn6 = (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6));
        locals.var_t3_dn7 = (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7));
        locals.var_t3_dn8 = (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8));
        locals.var_t3_dn9 = (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9));
        locals.var_t3_dn10 = (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10));
        locals.var_t3_dn13 = (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13));
        locals.var_t3_rv = 0.0;

        let assign11610_e6296: f64 = (1.0 / locals.var_t3);
        let assign11610_e6299: f64 = (1.0 / p.p220);
        let assign11610_e6300: f64 = (assign11610_e6296 + assign11610_e6299);
        let assign11610_e6301: f64 = (1.0 / assign11610_e6300);
        locals.var_t1 = assign11610_e6301;
        locals.var_t1_dn0 = (-((-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn2 = (-((-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn4 = (-((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn5 = (-((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn6 = (-((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn7 = (-((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn8 = (-((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn9 = (-((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn10 = (-((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_dn13 = (-((-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3))) / (assign11610_e6300 * assign11610_e6300)));
        locals.var_t1_rv = 0.0;

        let (assign11620_e6307, assign11620_e6307_d_n0, assign11620_e6307_d_n2, assign11620_e6307_d_n4, assign11620_e6307_d_n5, assign11620_e6307_d_n6, assign11620_e6307_d_n7, assign11620_e6307_d_n8, assign11620_e6307_d_n9, assign11620_e6307_d_n10, assign11620_e6307_d_n13,) = {
    if (0.0 >= locals.var_t1) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t2 = assign11620_e6307;
        locals.var_t2_dn0 = assign11620_e6307_d_n0;
        locals.var_t2_dn2 = assign11620_e6307_d_n2;
        locals.var_t2_dn4 = assign11620_e6307_d_n4;
        locals.var_t2_dn5 = assign11620_e6307_d_n5;
        locals.var_t2_dn6 = assign11620_e6307_d_n6;
        locals.var_t2_dn7 = assign11620_e6307_d_n7;
        locals.var_t2_dn8 = assign11620_e6307_d_n8;
        locals.var_t2_dn9 = assign11620_e6307_d_n9;
        locals.var_t2_dn10 = assign11620_e6307_d_n10;
        locals.var_t2_dn13 = assign11620_e6307_d_n13;
        locals.var_t2_rv = 0.0;

        let assign11630_e6312: f64 = (locals.var_npexte - locals.var_ef_nsubc);
        let assign11630_e6313: f64 = (locals.var_t2 * assign11630_e6312);
        let assign11630_e6315: f64 = (assign11630_e6313 / locals.var_lgate);
        let assign11630_e6316: f64 = (locals.var_nsub + assign11630_e6315);
        locals.var_nsub = assign11630_e6316;
        locals.var_nsub_dn0 = (locals.var_nsub_dn0 + (((locals.var_t2_dn0 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn0 - locals.var_ef_nsubc_dn0))) / locals.var_lgate));
        locals.var_nsub_dn2 = (locals.var_nsub_dn2 + (((locals.var_t2_dn2 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn2 - locals.var_ef_nsubc_dn2))) / locals.var_lgate));
        locals.var_nsub_dn4 = (locals.var_nsub_dn4 + (((locals.var_t2_dn4 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn4 - locals.var_ef_nsubc_dn4))) / locals.var_lgate));
        locals.var_nsub_dn5 = (locals.var_nsub_dn5 + (((locals.var_t2_dn5 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn5 - locals.var_ef_nsubc_dn5))) / locals.var_lgate));
        locals.var_nsub_dn6 = (locals.var_nsub_dn6 + (((locals.var_t2_dn6 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn6 - locals.var_ef_nsubc_dn6))) / locals.var_lgate));
        locals.var_nsub_dn7 = (locals.var_nsub_dn7 + (((locals.var_t2_dn7 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn7 - locals.var_ef_nsubc_dn7))) / locals.var_lgate));
        locals.var_nsub_dn8 = (locals.var_nsub_dn8 + (((locals.var_t2_dn8 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn8 - locals.var_ef_nsubc_dn8))) / locals.var_lgate));
        locals.var_nsub_dn9 = (locals.var_nsub_dn9 + (((locals.var_t2_dn9 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn9 - locals.var_ef_nsubc_dn9))) / locals.var_lgate));
        locals.var_nsub_dn10 = (locals.var_nsub_dn10 + (((locals.var_t2_dn10 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn10 - locals.var_ef_nsubc_dn10))) / locals.var_lgate));
        locals.var_nsub_dn13 = (locals.var_nsub_dn13 + (((locals.var_t2_dn13 * assign11630_e6312) + (locals.var_t2 * (locals.var_npexte_dn13 - locals.var_ef_nsubc_dn13))) / locals.var_lgate));
        locals.var_nsub_rv = 0.0;

        let assign11640_e6319: f64 = (1.6021918e-19 * locals.var_nsub);
        locals.var_q_nsub = assign11640_e6319;
        locals.var_q_nsub_dn0 = (1.6021918e-19 * locals.var_nsub_dn0);
        locals.var_q_nsub_dn2 = (1.6021918e-19 * locals.var_nsub_dn2);
        locals.var_q_nsub_dn4 = (1.6021918e-19 * locals.var_nsub_dn4);
        locals.var_q_nsub_dn5 = (1.6021918e-19 * locals.var_nsub_dn5);
        locals.var_q_nsub_dn6 = (1.6021918e-19 * locals.var_nsub_dn6);
        locals.var_q_nsub_dn7 = (1.6021918e-19 * locals.var_nsub_dn7);
        locals.var_q_nsub_dn8 = (1.6021918e-19 * locals.var_nsub_dn8);
        locals.var_q_nsub_dn9 = (1.6021918e-19 * locals.var_nsub_dn9);
        locals.var_q_nsub_dn10 = (1.6021918e-19 * locals.var_nsub_dn10);
        locals.var_q_nsub_dn13 = (1.6021918e-19 * locals.var_nsub_dn13);
        locals.var_q_nsub_rv = 0.0;

        let assign11650_e6322: f64 = (locals.var_q_nsub * 1.034943e-10);
        locals.var_qnsub_esi = assign11650_e6322;
        locals.var_qnsub_esi_dn0 = (locals.var_q_nsub_dn0 * 1.034943e-10);
        locals.var_qnsub_esi_dn2 = (locals.var_q_nsub_dn2 * 1.034943e-10);
        locals.var_qnsub_esi_dn4 = (locals.var_q_nsub_dn4 * 1.034943e-10);
        locals.var_qnsub_esi_dn5 = (locals.var_q_nsub_dn5 * 1.034943e-10);
        locals.var_qnsub_esi_dn6 = (locals.var_q_nsub_dn6 * 1.034943e-10);
        locals.var_qnsub_esi_dn7 = (locals.var_q_nsub_dn7 * 1.034943e-10);
        locals.var_qnsub_esi_dn8 = (locals.var_q_nsub_dn8 * 1.034943e-10);
        locals.var_qnsub_esi_dn9 = (locals.var_q_nsub_dn9 * 1.034943e-10);
        locals.var_qnsub_esi_dn10 = (locals.var_q_nsub_dn10 * 1.034943e-10);
        locals.var_qnsub_esi_dn13 = (locals.var_q_nsub_dn13 * 1.034943e-10);
        locals.var_qnsub_esi_rv = 0.0;

        let assign11660_e6325: f64 = (2.0 * locals.var_qnsub_esi);
        locals.var_qnsub_esi2 = assign11660_e6325;
        locals.var_qnsub_esi2_dn0 = (2.0 * locals.var_qnsub_esi_dn0);
        locals.var_qnsub_esi2_dn2 = (2.0 * locals.var_qnsub_esi_dn2);
        locals.var_qnsub_esi2_dn4 = (2.0 * locals.var_qnsub_esi_dn4);
        locals.var_qnsub_esi2_dn5 = (2.0 * locals.var_qnsub_esi_dn5);
        locals.var_qnsub_esi2_dn6 = (2.0 * locals.var_qnsub_esi_dn6);
        locals.var_qnsub_esi2_dn7 = (2.0 * locals.var_qnsub_esi_dn7);
        locals.var_qnsub_esi2_dn8 = (2.0 * locals.var_qnsub_esi_dn8);
        locals.var_qnsub_esi2_dn9 = (2.0 * locals.var_qnsub_esi_dn9);
        locals.var_qnsub_esi2_dn10 = (2.0 * locals.var_qnsub_esi_dn10);
        locals.var_qnsub_esi2_dn13 = (2.0 * locals.var_qnsub_esi_dn13);
        locals.var_qnsub_esi2_rv = 0.0;

        let assign11670_e6329: f64 = (2.0 * p.p140);
        let assign11670_e6334: f64 = if ((locals.var_lgate <= assign11670_e6329) && (p.p140 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard268 = assign11670_e6334;
        locals.var_guard268_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11680_e6350, assign11680_e6350_d_n0, assign11680_e6350_d_n2, assign11680_e6350_d_n4, assign11680_e6350_d_n5, assign11680_e6350_d_n6, assign11680_e6350_d_n7, assign11680_e6350_d_n8, assign11680_e6350_d_n9, assign11680_e6350_d_n10, assign11680_e6350_d_n13,) = {
    if (locals.var_guard268 != 0.0) {
        let assign11680_e6338: f64 = (2.0 * locals.var_nsubps);
        let assign11680_e6341: f64 = (locals.var_nsubps - locals.var_ef_nsubc);
        let assign11680_e6343: f64 = (assign11680_e6341 * locals.var_lgate);
        let assign11680_e6345: f64 = (assign11680_e6343 / p.p140);
        let assign11680_e6346: f64 = (assign11680_e6338 - assign11680_e6345);
        let assign11680_e6348: f64 = (assign11680_e6346 - locals.var_ef_nsubc);
        (assign11680_e6348, (((2.0 * locals.var_nsubps_dn0) - (((locals.var_nsubps_dn0 - locals.var_ef_nsubc_dn0) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn0), (((2.0 * locals.var_nsubps_dn2) - (((locals.var_nsubps_dn2 - locals.var_ef_nsubc_dn2) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn2), (((2.0 * locals.var_nsubps_dn4) - (((locals.var_nsubps_dn4 - locals.var_ef_nsubc_dn4) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn4), (((2.0 * locals.var_nsubps_dn5) - (((locals.var_nsubps_dn5 - locals.var_ef_nsubc_dn5) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn5), (((2.0 * locals.var_nsubps_dn6) - (((locals.var_nsubps_dn6 - locals.var_ef_nsubc_dn6) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn6), (((2.0 * locals.var_nsubps_dn7) - (((locals.var_nsubps_dn7 - locals.var_ef_nsubc_dn7) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn7), (((2.0 * locals.var_nsubps_dn8) - (((locals.var_nsubps_dn8 - locals.var_ef_nsubc_dn8) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn8), (((2.0 * locals.var_nsubps_dn9) - (((locals.var_nsubps_dn9 - locals.var_ef_nsubc_dn9) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn9), (((2.0 * locals.var_nsubps_dn10) - (((locals.var_nsubps_dn10 - locals.var_ef_nsubc_dn10) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn10), (((2.0 * locals.var_nsubps_dn13) - (((locals.var_nsubps_dn13 - locals.var_ef_nsubc_dn13) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn13),)
    } else {
        (locals.var_nsubb, locals.var_nsubb_dn0, locals.var_nsubb_dn2, locals.var_nsubb_dn4, locals.var_nsubb_dn5, locals.var_nsubb_dn6, locals.var_nsubb_dn7, locals.var_nsubb_dn8, locals.var_nsubb_dn9, locals.var_nsubb_dn10, locals.var_nsubb_dn13,)
    }
};
        locals.var_nsubb = assign11680_e6350;
        locals.var_nsubb_dn0 = assign11680_e6350_d_n0;
        locals.var_nsubb_dn2 = assign11680_e6350_d_n2;
        locals.var_nsubb_dn4 = assign11680_e6350_d_n4;
        locals.var_nsubb_dn5 = assign11680_e6350_d_n5;
        locals.var_nsubb_dn6 = assign11680_e6350_d_n6;
        locals.var_nsubb_dn7 = assign11680_e6350_d_n7;
        locals.var_nsubb_dn8 = assign11680_e6350_d_n8;
        locals.var_nsubb_dn9 = assign11680_e6350_d_n9;
        locals.var_nsubb_dn10 = assign11680_e6350_d_n10;
        locals.var_nsubb_dn13 = assign11680_e6350_d_n13;
        locals.var_nsubb_rv = 0.0;

        let (assign11690_e6357, assign11690_e6357_d_n0, assign11690_e6357_d_n2, assign11690_e6357_d_n4, assign11690_e6357_d_n5, assign11690_e6357_d_n6, assign11690_e6357_d_n7, assign11690_e6357_d_n8, assign11690_e6357_d_n9, assign11690_e6357_d_n10, assign11690_e6357_d_n13,) = {
    if (locals.var_guard268 != 0.0) {
        let assign11690_e6354: f64 = (locals.var_nsubb / locals.var_ef_nsubc);
        let assign11690_e6355: f64 = (assign11690_e6354).ln();
        (assign11690_e6355, ((((locals.var_nsubb_dn0 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn0)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn2 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn2)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn4 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn4)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn5 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn5)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn6 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn6)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn7 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn7)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn8 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn8)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn9 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn9)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn10 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn10)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354), ((((locals.var_nsubb_dn13 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn13)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11690_e6354),)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn4, locals.var_ptovr0_dn5, locals.var_ptovr0_dn6, locals.var_ptovr0_dn7, locals.var_ptovr0_dn8, locals.var_ptovr0_dn9, locals.var_ptovr0_dn10, locals.var_ptovr0_dn13,)
    }
};
        locals.var_ptovr0 = assign11690_e6357;
        locals.var_ptovr0_dn0 = assign11690_e6357_d_n0;
        locals.var_ptovr0_dn2 = assign11690_e6357_d_n2;
        locals.var_ptovr0_dn4 = assign11690_e6357_d_n4;
        locals.var_ptovr0_dn5 = assign11690_e6357_d_n5;
        locals.var_ptovr0_dn6 = assign11690_e6357_d_n6;
        locals.var_ptovr0_dn7 = assign11690_e6357_d_n7;
        locals.var_ptovr0_dn8 = assign11690_e6357_d_n8;
        locals.var_ptovr0_dn9 = assign11690_e6357_d_n9;
        locals.var_ptovr0_dn10 = assign11690_e6357_d_n10;
        locals.var_ptovr0_dn13 = assign11690_e6357_d_n13;
        locals.var_ptovr0_rv = 0.0;

        let (assign11700_e6362, assign11700_e6362_d_n0, assign11700_e6362_d_n2, assign11700_e6362_d_n4, assign11700_e6362_d_n5, assign11700_e6362_d_n6, assign11700_e6362_d_n7, assign11700_e6362_d_n8, assign11700_e6362_d_n9, assign11700_e6362_d_n10, assign11700_e6362_d_n13,) = {
    if (locals.var_guard268 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn4, locals.var_ptovr0_dn5, locals.var_ptovr0_dn6, locals.var_ptovr0_dn7, locals.var_ptovr0_dn8, locals.var_ptovr0_dn9, locals.var_ptovr0_dn10, locals.var_ptovr0_dn13,)
    }
};
        locals.var_ptovr0 = assign11700_e6362;
        locals.var_ptovr0_dn0 = assign11700_e6362_d_n0;
        locals.var_ptovr0_dn2 = assign11700_e6362_d_n2;
        locals.var_ptovr0_dn4 = assign11700_e6362_d_n4;
        locals.var_ptovr0_dn5 = assign11700_e6362_d_n5;
        locals.var_ptovr0_dn6 = assign11700_e6362_d_n6;
        locals.var_ptovr0_dn7 = assign11700_e6362_d_n7;
        locals.var_ptovr0_dn8 = assign11700_e6362_d_n8;
        locals.var_ptovr0_dn9 = assign11700_e6362_d_n9;
        locals.var_ptovr0_dn10 = assign11700_e6362_d_n10;
        locals.var_ptovr0_dn13 = assign11700_e6362_d_n13;
        locals.var_ptovr0_rv = 0.0;

        let assign11710_e6365: f64 = (2.0 * 1.6021918e-19);
        let assign11710_e6367: f64 = (assign11710_e6365 * locals.var_uc_nsti);
        let assign11710_e6369: f64 = (assign11710_e6367 * 1.034943e-10);
        let assign11710_e6370: f64 = (assign11710_e6369).sqrt();
        locals.var_costi00 = assign11710_e6370;
        locals.var_costi00_rv = 0.0;

        let assign11720_e6374: f64 = (locals.var_uc_nsti * locals.var_uc_nsti);
        let assign11720_e6375: f64 = (1.0 / assign11720_e6374);
        locals.var_nsti_p2 = assign11720_e6375;
        locals.var_nsti_p2_rv = 0.0;

        let assign11730_e6380: f64 = (locals.var_lg).powf(p.p231);
        let assign11730_e6381: f64 = (locals.var_uc_vover / assign11730_e6380);
        let assign11730_e6382: f64 = (1.0 + assign11730_e6381);
        let assign11730_e6387: f64 = (locals.var_wlg).powf(p.p239);
        let assign11730_e6388: f64 = (p.p238 / assign11730_e6387);
        let assign11730_e6389: f64 = (1.0 + assign11730_e6388);
        let assign11730_e6390: f64 = (assign11730_e6382 * assign11730_e6389);
        locals.var_vmax0 = assign11730_e6390;
        locals.var_vmax0_rv = 0.0;

        let assign11740_e6393: f64 = (2.0 / 38.68283);
        let assign11740_e6396: f64 = (locals.var_nsub / 1.04e16);
        let assign11740_e6397: f64 = (assign11740_e6396).ln();
        let assign11740_e6398: f64 = (assign11740_e6393 * assign11740_e6397);
        locals.var_pb20 = assign11740_e6398;
        locals.var_pb20_dn0 = (assign11740_e6393 * ((locals.var_nsub_dn0 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn2 = (assign11740_e6393 * ((locals.var_nsub_dn2 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn4 = (assign11740_e6393 * ((locals.var_nsub_dn4 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn5 = (assign11740_e6393 * ((locals.var_nsub_dn5 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn6 = (assign11740_e6393 * ((locals.var_nsub_dn6 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn7 = (assign11740_e6393 * ((locals.var_nsub_dn7 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn8 = (assign11740_e6393 * ((locals.var_nsub_dn8 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn9 = (assign11740_e6393 * ((locals.var_nsub_dn9 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn10 = (assign11740_e6393 * ((locals.var_nsub_dn10 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_dn13 = (assign11740_e6393 * ((locals.var_nsub_dn13 / 1.04e16) / assign11740_e6396));
        locals.var_pb20_rv = 0.0;

        let assign11750_e6401: f64 = (2.0 / 38.68283);
        let assign11750_e6404: f64 = (locals.var_ef_nsubc / 1.04e16);
        let assign11750_e6405: f64 = (assign11750_e6404).ln();
        let assign11750_e6406: f64 = (assign11750_e6401 * assign11750_e6405);
        locals.var_pb2c = assign11750_e6406;
        locals.var_pb2c_dn0 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn0 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn2 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn2 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn4 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn4 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn5 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn5 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn6 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn6 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn7 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn7 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn8 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn8 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn9 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn9 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn10 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn10 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_dn13 = (assign11750_e6401 * ((locals.var_ef_nsubc_dn13 / 1.04e16) / assign11750_e6404));
        locals.var_pb2c_rv = 0.0;

        let assign11760_e6409: f64 = if p.p51 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard269 = assign11760_e6409;
        locals.var_guard269_rv = 0.0;

        let (assign11770_e6419, assign11770_e6419_d_n0, assign11770_e6419_d_n2, assign11770_e6419_d_n4, assign11770_e6419_d_n5, assign11770_e6419_d_n6, assign11770_e6419_d_n7, assign11770_e6419_d_n8, assign11770_e6419_d_n9, assign11770_e6419_d_n10, assign11770_e6419_d_n13,) = {
    if (locals.var_guard269 != 0.0) {
        let assign11770_e6415: f64 = (3.0 * p.p4);
        let assign11770_e6416: f64 = (locals.var_weff / assign11770_e6415);
        let assign11770_e6417: f64 = (p.p5 + assign11770_e6416);
        (assign11770_e6417, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign11770_e6419;
        locals.var_t1_dn0 = assign11770_e6419_d_n0;
        locals.var_t1_dn2 = assign11770_e6419_d_n2;
        locals.var_t1_dn4 = assign11770_e6419_d_n4;
        locals.var_t1_dn5 = assign11770_e6419_d_n5;
        locals.var_t1_dn6 = assign11770_e6419_d_n6;
        locals.var_t1_dn7 = assign11770_e6419_d_n7;
        locals.var_t1_dn8 = assign11770_e6419_d_n8;
        locals.var_t1_dn9 = assign11770_e6419_d_n9;
        locals.var_t1_dn10 = assign11770_e6419_d_n10;
        locals.var_t1_dn13 = assign11770_e6419_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign11780_e6425, assign11780_e6425_d_n0, assign11780_e6425_d_n2, assign11780_e6425_d_n4, assign11780_e6425_d_n5, assign11780_e6425_d_n6, assign11780_e6425_d_n7, assign11780_e6425_d_n8, assign11780_e6425_d_n9, assign11780_e6425_d_n10, assign11780_e6425_d_n13,) = {
    if (locals.var_guard269 != 0.0) {
        let assign11780_e6423: f64 = (locals.var_lgate - p.p6);
        (assign11780_e6423, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign11780_e6425;
        locals.var_t2_dn0 = assign11780_e6425_d_n0;
        locals.var_t2_dn2 = assign11780_e6425_d_n2;
        locals.var_t2_dn4 = assign11780_e6425_d_n4;
        locals.var_t2_dn5 = assign11780_e6425_d_n5;
        locals.var_t2_dn6 = assign11780_e6425_d_n6;
        locals.var_t2_dn7 = assign11780_e6425_d_n7;
        locals.var_t2_dn8 = assign11780_e6425_d_n8;
        locals.var_t2_dn9 = assign11780_e6425_d_n9;
        locals.var_t2_dn10 = assign11780_e6425_d_n10;
        locals.var_t2_dn13 = assign11780_e6425_d_n13;
        locals.var_t2_rv = 0.0;

        let assign11840_e6467: f64 = if p.p130 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard271 = assign11840_e6467;
        locals.var_guard271_rv = 0.0;

        let (assign11850_e6473,) = {
    if (locals.var_guard271 != 0.0) {
        let assign11850_e6471: f64 = (p.p130 * p.p2);
        (assign11850_e6471,)
    } else {
        (locals.var_rd0,)
    }
};
        locals.var_rd0 = assign11850_e6473;
        locals.var_rd0_rv = 0.0;

        let (assign11860_e6479,) = {
    if (locals.var_guard271 != 0.0) {
        let assign11860_e6477: f64 = (p.p130 * p.p3);
        (assign11860_e6477,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11860_e6479;
        locals.var_rs0_rv = 0.0;

        let (assign11870_e6484,) = {
    if (locals.var_guard271 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rd0,)
    }
};
        locals.var_rd0 = assign11870_e6484;
        locals.var_rd0_rv = 0.0;

        let (assign11880_e6489,) = {
    if (locals.var_guard271 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11880_e6489;
        locals.var_rs0_rv = 0.0;

        let assign11890_e6492: f64 = if p.p131 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard272 = assign11890_e6492;
        locals.var_guard272_rv = 0.0;

        let (assign11900_e6498,) = {
    if (locals.var_guard272 != 0.0) {
        let assign11900_e6496: f64 = (p.p131 * p.p3);
        (assign11900_e6496,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11900_e6498;
        locals.var_rs0_rv = 0.0;

        let (assign11910_e6503,) = {
    if (locals.var_guard272 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11910_e6503;
        locals.var_rs0_rv = 0.0;

        let assign11920_e6506: f64 = if locals.var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign11920_e6506;
        locals.var_guard273_rv = 0.0;

        let assign11930_e6513: f64 = if ((locals.var_uc_rd > 0.0) || (locals.var_uc_rs > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard274 = assign11930_e6513;
        locals.var_guard274_rv = 0.0;

        let (assign11940_e6525,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard274 != 0.0)) {
        let assign11940_e6521: f64 = (locals.var_wlg).powf(p.p310);
        let assign11940_e6522: f64 = (p.p309 / assign11940_e6521);
        let assign11940_e6523: f64 = (1.0 + assign11940_e6522);
        (assign11940_e6523,)
    } else {
        (locals.var_rdtemp0,)
    }
};
        locals.var_rdtemp0 = assign11940_e6525;
        locals.var_rdtemp0_rv = 0.0;

        let assign11950_e6528: f64 = if locals.var_uc_rdvd != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard275 = assign11950_e6528;
        locals.var_guard275_rv = 0.0;

        let (assign11960_e6542, assign11960_e6542_d_n0, assign11960_e6542_d_n2, assign11960_e6542_d_n4, assign11960_e6542_d_n5, assign11960_e6542_d_n6, assign11960_e6542_d_n7, assign11960_e6542_d_n8, assign11960_e6542_d_n9, assign11960_e6542_d_n10, assign11960_e6542_d_n13,) = {
    if (((locals.var_guard273 != 0.0) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 != 0.0)) {
        let assign11960_e6538: f64 = (locals.var_wlg).powf(p.p304);
        let assign11960_e6539: f64 = (p.p303 / assign11960_e6538);
        let assign11960_e6540: f64 = (1.0 + assign11960_e6539);
        (assign11960_e6540, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign11960_e6542;
        locals.var_t7_dn0 = assign11960_e6542_d_n0;
        locals.var_t7_dn2 = assign11960_e6542_d_n2;
        locals.var_t7_dn4 = assign11960_e6542_d_n4;
        locals.var_t7_dn5 = assign11960_e6542_d_n5;
        locals.var_t7_dn6 = assign11960_e6542_d_n6;
        locals.var_t7_dn7 = assign11960_e6542_d_n7;
        locals.var_t7_dn8 = assign11960_e6542_d_n8;
        locals.var_t7_dn9 = assign11960_e6542_d_n9;
        locals.var_t7_dn10 = assign11960_e6542_d_n10;
        locals.var_t7_dn13 = assign11960_e6542_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign11970_e6555, assign11970_e6555_d_n0, assign11970_e6555_d_n2, assign11970_e6555_d_n4, assign11970_e6555_d_n5, assign11970_e6555_d_n6, assign11970_e6555_d_n7, assign11970_e6555_d_n8, assign11970_e6555_d_n9, assign11970_e6555_d_n10, assign11970_e6555_d_n13,) = {
    if (((locals.var_guard273 != 0.0) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 != 0.0)) {
        let assign11970_e6549: f64 = (-p.p301);
        let assign11970_e6552: f64 = (locals.var_lg).powf(p.p302);
        let assign11970_e6553: f64 = (assign11970_e6549 * assign11970_e6552);
        (assign11970_e6553, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign11970_e6555;
        locals.var_t6_dn0 = assign11970_e6555_d_n0;
        locals.var_t6_dn2 = assign11970_e6555_d_n2;
        locals.var_t6_dn4 = assign11970_e6555_d_n4;
        locals.var_t6_dn5 = assign11970_e6555_d_n5;
        locals.var_t6_dn6 = assign11970_e6555_d_n6;
        locals.var_t6_dn7 = assign11970_e6555_d_n7;
        locals.var_t6_dn8 = assign11970_e6555_d_n8;
        locals.var_t6_dn9 = assign11970_e6555_d_n9;
        locals.var_t6_dn10 = assign11970_e6555_d_n10;
        locals.var_t6_dn13 = assign11970_e6555_d_n13;
        locals.var_t6_rv = 0.0;

        let assign11980_e6558: f64 = if locals.var_t6 > 60.0 { 1.0 } else { 0.0 };
        locals.var_guard276 = assign11980_e6558;
        locals.var_guard276_rv = 0.0;

        let (assign11990_e6568, assign11990_e6568_d_n0, assign11990_e6568_d_n2, assign11990_e6568_d_n4, assign11990_e6568_d_n5, assign11990_e6568_d_n6, assign11990_e6568_d_n7, assign11990_e6568_d_n8, assign11990_e6568_d_n9, assign11990_e6568_d_n10, assign11990_e6568_d_n13,) = {
    if ((((locals.var_guard273 != 0.0) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 != 0.0)) && (locals.var_guard276 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign11990_e6568;
        locals.var_t6_dn0 = assign11990_e6568_d_n0;
        locals.var_t6_dn2 = assign11990_e6568_d_n2;
        locals.var_t6_dn4 = assign11990_e6568_d_n4;
        locals.var_t6_dn5 = assign11990_e6568_d_n5;
        locals.var_t6_dn6 = assign11990_e6568_d_n6;
        locals.var_t6_dn7 = assign11990_e6568_d_n7;
        locals.var_t6_dn8 = assign11990_e6568_d_n8;
        locals.var_t6_dn9 = assign11990_e6568_d_n9;
        locals.var_t6_dn10 = assign11990_e6568_d_n10;
        locals.var_t6_dn13 = assign11990_e6568_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign12000_e6577, assign12000_e6577_d_n0, assign12000_e6577_d_n2, assign12000_e6577_d_n4, assign12000_e6577_d_n5, assign12000_e6577_d_n6, assign12000_e6577_d_n7, assign12000_e6577_d_n8, assign12000_e6577_d_n9, assign12000_e6577_d_n10, assign12000_e6577_d_n13,) = {
    if (((locals.var_guard273 != 0.0) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 != 0.0)) {
        let assign12000_e6575: f64 = (locals.var_t6).exp();
        (assign12000_e6575, (assign12000_e6575 * locals.var_t6_dn0), (assign12000_e6575 * locals.var_t6_dn2), (assign12000_e6575 * locals.var_t6_dn4), (assign12000_e6575 * locals.var_t6_dn5), (assign12000_e6575 * locals.var_t6_dn6), (assign12000_e6575 * locals.var_t6_dn7), (assign12000_e6575 * locals.var_t6_dn8), (assign12000_e6575 * locals.var_t6_dn9), (assign12000_e6575 * locals.var_t6_dn10), (assign12000_e6575 * locals.var_t6_dn13),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign12000_e6577;
        locals.var_t6_dn0 = assign12000_e6577_d_n0;
        locals.var_t6_dn2 = assign12000_e6577_d_n2;
        locals.var_t6_dn4 = assign12000_e6577_d_n4;
        locals.var_t6_dn5 = assign12000_e6577_d_n5;
        locals.var_t6_dn6 = assign12000_e6577_d_n6;
        locals.var_t6_dn7 = assign12000_e6577_d_n7;
        locals.var_t6_dn8 = assign12000_e6577_d_n8;
        locals.var_t6_dn9 = assign12000_e6577_d_n9;
        locals.var_t6_dn10 = assign12000_e6577_d_n10;
        locals.var_t6_dn13 = assign12000_e6577_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign12010_e6587, assign12010_e6587_d_n0, assign12010_e6587_d_n2, assign12010_e6587_d_n4, assign12010_e6587_d_n5, assign12010_e6587_d_n6, assign12010_e6587_d_n7, assign12010_e6587_d_n8, assign12010_e6587_d_n9, assign12010_e6587_d_n10, assign12010_e6587_d_n13,) = {
    if (((locals.var_guard273 != 0.0) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 != 0.0)) {
        let assign12010_e6585: f64 = (locals.var_t6 * locals.var_t7);
        (assign12010_e6585, ((locals.var_t6_dn0 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn0)), ((locals.var_t6_dn2 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn2)), ((locals.var_t6_dn4 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn4)), ((locals.var_t6_dn5 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn5)), ((locals.var_t6_dn6 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn6)), ((locals.var_t6_dn7 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn7)), ((locals.var_t6_dn8 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn8)), ((locals.var_t6_dn9 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn9)), ((locals.var_t6_dn10 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn10)), ((locals.var_t6_dn13 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn13)),)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn13,)
    }
};
        locals.var_rdvdtemp0 = assign12010_e6587;
        locals.var_rdvdtemp0_dn0 = assign12010_e6587_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12010_e6587_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12010_e6587_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12010_e6587_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12010_e6587_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12010_e6587_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12010_e6587_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12010_e6587_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12010_e6587_d_n10;
        locals.var_rdvdtemp0_dn13 = assign12010_e6587_d_n13;
        locals.var_rdvdtemp0_rv = 0.0;

        let (assign12020_e6596, assign12020_e6596_d_n0, assign12020_e6596_d_n2, assign12020_e6596_d_n4, assign12020_e6596_d_n5, assign12020_e6596_d_n6, assign12020_e6596_d_n7, assign12020_e6596_d_n8, assign12020_e6596_d_n9, assign12020_e6596_d_n10, assign12020_e6596_d_n13,) = {
    if (((locals.var_guard273 != 0.0) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn13,)
    }
};
        locals.var_rdvdtemp0 = assign12020_e6596;
        locals.var_rdvdtemp0_dn0 = assign12020_e6596_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12020_e6596_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12020_e6596_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12020_e6596_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12020_e6596_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12020_e6596_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12020_e6596_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12020_e6596_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12020_e6596_d_n10;
        locals.var_rdvdtemp0_dn13 = assign12020_e6596_d_n13;
        locals.var_rdvdtemp0_rv = 0.0;

        let (assign12030_e6603,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard274 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdtemp0,)
    }
};
        locals.var_rdtemp0 = assign12030_e6603;
        locals.var_rdtemp0_rv = 0.0;

        let (assign12040_e6610, assign12040_e6610_d_n0, assign12040_e6610_d_n2, assign12040_e6610_d_n4, assign12040_e6610_d_n5, assign12040_e6610_d_n6, assign12040_e6610_d_n7, assign12040_e6610_d_n8, assign12040_e6610_d_n9, assign12040_e6610_d_n10, assign12040_e6610_d_n13,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard274 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn13,)
    }
};
        locals.var_rdvdtemp0 = assign12040_e6610;
        locals.var_rdvdtemp0_dn0 = assign12040_e6610_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12040_e6610_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12040_e6610_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12040_e6610_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12040_e6610_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12040_e6610_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12040_e6610_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12040_e6610_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12040_e6610_d_n10;
        locals.var_rdvdtemp0_dn13 = assign12040_e6610_d_n13;
        locals.var_rdvdtemp0_rv = 0.0;

        let assign12050_e6613: f64 = if locals.var_uc_rd23 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard277 = assign12050_e6613;
        locals.var_guard277_rv = 0.0;

        let (assign12060_e6625, assign12060_e6625_d_n0, assign12060_e6625_d_n2, assign12060_e6625_d_n4, assign12060_e6625_d_n5, assign12060_e6625_d_n6, assign12060_e6625_d_n7, assign12060_e6625_d_n8, assign12060_e6625_d_n9, assign12060_e6625_d_n10, assign12060_e6625_d_n13,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard277 != 0.0)) {
        let assign12060_e6621: f64 = (locals.var_wlg).powf(p.p308);
        let assign12060_e6622: f64 = (p.p307 / assign12060_e6621);
        let assign12060_e6623: f64 = (1.0 + assign12060_e6622);
        (assign12060_e6623, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign12060_e6625;
        locals.var_t2_dn0 = assign12060_e6625_d_n0;
        locals.var_t2_dn2 = assign12060_e6625_d_n2;
        locals.var_t2_dn4 = assign12060_e6625_d_n4;
        locals.var_t2_dn5 = assign12060_e6625_d_n5;
        locals.var_t2_dn6 = assign12060_e6625_d_n6;
        locals.var_t2_dn7 = assign12060_e6625_d_n7;
        locals.var_t2_dn8 = assign12060_e6625_d_n8;
        locals.var_t2_dn9 = assign12060_e6625_d_n9;
        locals.var_t2_dn10 = assign12060_e6625_d_n10;
        locals.var_t2_dn13 = assign12060_e6625_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign12070_e6636, assign12070_e6636_d_n0, assign12070_e6636_d_n2, assign12070_e6636_d_n4, assign12070_e6636_d_n5, assign12070_e6636_d_n6, assign12070_e6636_d_n7, assign12070_e6636_d_n8, assign12070_e6636_d_n9, assign12070_e6636_d_n10, assign12070_e6636_d_n13,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard277 != 0.0)) {
        let assign12070_e6630: f64 = (-p.p305);
        let assign12070_e6633: f64 = (locals.var_lg).powf(p.p306);
        let assign12070_e6634: f64 = (assign12070_e6630 * assign12070_e6633);
        (assign12070_e6634, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign12070_e6636;
        locals.var_t1_dn0 = assign12070_e6636_d_n0;
        locals.var_t1_dn2 = assign12070_e6636_d_n2;
        locals.var_t1_dn4 = assign12070_e6636_d_n4;
        locals.var_t1_dn5 = assign12070_e6636_d_n5;
        locals.var_t1_dn6 = assign12070_e6636_d_n6;
        locals.var_t1_dn7 = assign12070_e6636_d_n7;
        locals.var_t1_dn8 = assign12070_e6636_d_n8;
        locals.var_t1_dn9 = assign12070_e6636_d_n9;
        locals.var_t1_dn10 = assign12070_e6636_d_n10;
        locals.var_t1_dn13 = assign12070_e6636_d_n13;
        locals.var_t1_rv = 0.0;

        let assign12080_e6639: f64 = if locals.var_t1 > 60.0 { 1.0 } else { 0.0 };
        locals.var_guard278 = assign12080_e6639;
        locals.var_guard278_rv = 0.0;

        let (assign12090_e6647, assign12090_e6647_d_n0, assign12090_e6647_d_n2, assign12090_e6647_d_n4, assign12090_e6647_d_n5, assign12090_e6647_d_n6, assign12090_e6647_d_n7, assign12090_e6647_d_n8, assign12090_e6647_d_n9, assign12090_e6647_d_n10, assign12090_e6647_d_n13,) = {
    if (((locals.var_guard273 != 0.0) && (locals.var_guard277 != 0.0)) && (locals.var_guard278 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign12090_e6647;
        locals.var_t1_dn0 = assign12090_e6647_d_n0;
        locals.var_t1_dn2 = assign12090_e6647_d_n2;
        locals.var_t1_dn4 = assign12090_e6647_d_n4;
        locals.var_t1_dn5 = assign12090_e6647_d_n5;
        locals.var_t1_dn6 = assign12090_e6647_d_n6;
        locals.var_t1_dn7 = assign12090_e6647_d_n7;
        locals.var_t1_dn8 = assign12090_e6647_d_n8;
        locals.var_t1_dn9 = assign12090_e6647_d_n9;
        locals.var_t1_dn10 = assign12090_e6647_d_n10;
        locals.var_t1_dn13 = assign12090_e6647_d_n13;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12100_e6654, assign12100_e6654_d_n0, assign12100_e6654_d_n2, assign12100_e6654_d_n4, assign12100_e6654_d_n5, assign12100_e6654_d_n6, assign12100_e6654_d_n7, assign12100_e6654_d_n8, assign12100_e6654_d_n9, assign12100_e6654_d_n10, assign12100_e6654_d_n13,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard277 != 0.0)) {
        let assign12100_e6652: f64 = (locals.var_t1).exp();
        (assign12100_e6652, (assign12100_e6652 * locals.var_t1_dn0), (assign12100_e6652 * locals.var_t1_dn2), (assign12100_e6652 * locals.var_t1_dn4), (assign12100_e6652 * locals.var_t1_dn5), (assign12100_e6652 * locals.var_t1_dn6), (assign12100_e6652 * locals.var_t1_dn7), (assign12100_e6652 * locals.var_t1_dn8), (assign12100_e6652 * locals.var_t1_dn9), (assign12100_e6652 * locals.var_t1_dn10), (assign12100_e6652 * locals.var_t1_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign12100_e6654;
        locals.var_t1_dn0 = assign12100_e6654_d_n0;
        locals.var_t1_dn2 = assign12100_e6654_d_n2;
        locals.var_t1_dn4 = assign12100_e6654_d_n4;
        locals.var_t1_dn5 = assign12100_e6654_d_n5;
        locals.var_t1_dn6 = assign12100_e6654_d_n6;
        locals.var_t1_dn7 = assign12100_e6654_d_n7;
        locals.var_t1_dn8 = assign12100_e6654_d_n8;
        locals.var_t1_dn9 = assign12100_e6654_d_n9;
        locals.var_t1_dn10 = assign12100_e6654_d_n10;
        locals.var_t1_dn13 = assign12100_e6654_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign12110_e6664, assign12110_e6664_d_n0, assign12110_e6664_d_n2, assign12110_e6664_d_n4, assign12110_e6664_d_n5, assign12110_e6664_d_n6, assign12110_e6664_d_n7, assign12110_e6664_d_n8, assign12110_e6664_d_n9, assign12110_e6664_d_n10, assign12110_e6664_d_n13,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard277 != 0.0)) {
        let assign12110_e6660: f64 = (locals.var_uc_rd23 * locals.var_t2);
        let assign12110_e6662: f64 = (assign12110_e6660 * locals.var_t1);
        (assign12110_e6662, (((locals.var_uc_rd23 * locals.var_t2_dn0) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn0)), (((locals.var_uc_rd23 * locals.var_t2_dn2) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn2)), (((locals.var_uc_rd23 * locals.var_t2_dn4) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn4)), (((locals.var_uc_rd23 * locals.var_t2_dn5) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn5)), (((locals.var_uc_rd23 * locals.var_t2_dn6) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn6)), (((locals.var_uc_rd23 * locals.var_t2_dn7) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn7)), (((locals.var_uc_rd23 * locals.var_t2_dn8) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn8)), (((locals.var_uc_rd23 * locals.var_t2_dn9) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn9)), (((locals.var_uc_rd23 * locals.var_t2_dn10) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn10)), (((locals.var_uc_rd23 * locals.var_t2_dn13) * locals.var_t1) + (assign12110_e6660 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign12110_e6664;
        locals.var_t3_dn0 = assign12110_e6664_d_n0;
        locals.var_t3_dn2 = assign12110_e6664_d_n2;
        locals.var_t3_dn4 = assign12110_e6664_d_n4;
        locals.var_t3_dn5 = assign12110_e6664_d_n5;
        locals.var_t3_dn6 = assign12110_e6664_d_n6;
        locals.var_t3_dn7 = assign12110_e6664_d_n7;
        locals.var_t3_dn8 = assign12110_e6664_d_n8;
        locals.var_t3_dn9 = assign12110_e6664_d_n9;
        locals.var_t3_dn10 = assign12110_e6664_d_n10;
        locals.var_t3_dn13 = assign12110_e6664_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign12120_e6687, assign12120_e6687_d_n0, assign12120_e6687_d_n2, assign12120_e6687_d_n4, assign12120_e6687_d_n5, assign12120_e6687_d_n6, assign12120_e6687_d_n7, assign12120_e6687_d_n8, assign12120_e6687_d_n9, assign12120_e6687_d_n10, assign12120_e6687_d_n13,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard277 != 0.0)) {
        let assign12120_e6672: f64 = (locals.var_t3 * locals.var_t3);
        let assign12120_e6675: f64 = (4.0 * 1e-6);
        let assign12120_e6677: f64 = (assign12120_e6675 / 100.0);
        let assign12120_e6679: f64 = (assign12120_e6677 * 1e-6);
        let assign12120_e6681: f64 = (assign12120_e6679 / 100.0);
        let assign12120_e6682: f64 = (assign12120_e6672 + assign12120_e6681);
        let assign12120_e6683: f64 = (assign12120_e6682).sqrt();
        let assign12120_e6684: f64 = (locals.var_t3 + assign12120_e6683);
        let assign12120_e6685: f64 = (0.5 * assign12120_e6684);
        (assign12120_e6685, (0.5 * (locals.var_t3_dn0 + (((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn2 + (((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn4 + (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn5 + (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn6 + (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn7 + (((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn8 + (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn9 + (((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn10 + (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (2.0 * assign12120_e6683)))), (0.5 * (locals.var_t3_dn13 + (((locals.var_t3_dn13 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn13)) / (2.0 * assign12120_e6683)))),)
    } else {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn13,)
    }
};
        locals.var_rd23e = assign12120_e6687;
        locals.var_rd23e_dn0 = assign12120_e6687_d_n0;
        locals.var_rd23e_dn2 = assign12120_e6687_d_n2;
        locals.var_rd23e_dn4 = assign12120_e6687_d_n4;
        locals.var_rd23e_dn5 = assign12120_e6687_d_n5;
        locals.var_rd23e_dn6 = assign12120_e6687_d_n6;
        locals.var_rd23e_dn7 = assign12120_e6687_d_n7;
        locals.var_rd23e_dn8 = assign12120_e6687_d_n8;
        locals.var_rd23e_dn9 = assign12120_e6687_d_n9;
        locals.var_rd23e_dn10 = assign12120_e6687_d_n10;
        locals.var_rd23e_dn13 = assign12120_e6687_d_n13;
        locals.var_rd23e_rv = 0.0;

        let (assign12130_e6694, assign12130_e6694_d_n0, assign12130_e6694_d_n2, assign12130_e6694_d_n4, assign12130_e6694_d_n5, assign12130_e6694_d_n6, assign12130_e6694_d_n7, assign12130_e6694_d_n8, assign12130_e6694_d_n9, assign12130_e6694_d_n10, assign12130_e6694_d_n13,) = {
    if ((locals.var_guard273 != 0.0) && (locals.var_guard277 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn13,)
    }
};
        locals.var_rd23e = assign12130_e6694;
        locals.var_rd23e_dn0 = assign12130_e6694_d_n0;
        locals.var_rd23e_dn2 = assign12130_e6694_d_n2;
        locals.var_rd23e_dn4 = assign12130_e6694_d_n4;
        locals.var_rd23e_dn5 = assign12130_e6694_d_n5;
        locals.var_rd23e_dn6 = assign12130_e6694_d_n6;
        locals.var_rd23e_dn7 = assign12130_e6694_d_n7;
        locals.var_rd23e_dn8 = assign12130_e6694_d_n8;
        locals.var_rd23e_dn9 = assign12130_e6694_d_n9;
        locals.var_rd23e_dn10 = assign12130_e6694_d_n10;
        locals.var_rd23e_dn13 = assign12130_e6694_d_n13;
        locals.var_rd23e_rv = 0.0;

        let (assign12140_e6698,) = {
    if (locals.var_guard273 != 0.0) {
        (0.0,)
    } else {
        (locals.var_xmax,)
    }
};
        locals.var_xmax = assign12140_e6698;
        locals.var_xmax_rv = 0.0;

        let (assign12150_e6702,) = {
    if (locals.var_guard273 != 0.0) {
        (0.0,)
    } else {
        (locals.var_xmax_s,)
    }
};
        locals.var_xmax_s = assign12150_e6702;
        locals.var_xmax_s_rv = 0.0;

        let (assign12160_e6706,) = {
    if (locals.var_guard273 != 0.0) {
        (0.0,)
    } else {
        (locals.var_rdrvmaxwe,)
    }
};
        locals.var_rdrvmaxwe = assign12160_e6706;
        locals.var_rdrvmaxwe_rv = 0.0;

        let (assign12170_e6710,) = {
    if (locals.var_guard273 != 0.0) {
        (0.0,)
    } else {
        (locals.var_rdrvmaxle,)
    }
};
        locals.var_rdrvmaxle = assign12170_e6710;
        locals.var_rdrvmaxle_rv = 0.0;

        let (assign12180_e6714,) = {
    if (locals.var_guard273 != 0.0) {
        (0.0,)
    } else {
        (locals.var_rdrmuele,)
    }
};
        locals.var_rdrmuele = assign12180_e6714;
        locals.var_rdrmuele_rv = 0.0;

        let (assign12190_e6718, assign12190_e6718_d_n0, assign12190_e6718_d_n2, assign12190_e6718_d_n4, assign12190_e6718_d_n5, assign12190_e6718_d_n6, assign12190_e6718_d_n7, assign12190_e6718_d_n8, assign12190_e6718_d_n9, assign12190_e6718_d_n10, assign12190_e6718_d_n13,) = {
    if (locals.var_guard273 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn13,)
    }
};
        locals.var_rdrmuevbs = assign12190_e6718;
        locals.var_rdrmuevbs_dn0 = assign12190_e6718_d_n0;
        locals.var_rdrmuevbs_dn2 = assign12190_e6718_d_n2;
        locals.var_rdrmuevbs_dn4 = assign12190_e6718_d_n4;
        locals.var_rdrmuevbs_dn5 = assign12190_e6718_d_n5;
        locals.var_rdrmuevbs_dn6 = assign12190_e6718_d_n6;
        locals.var_rdrmuevbs_dn7 = assign12190_e6718_d_n7;
        locals.var_rdrmuevbs_dn8 = assign12190_e6718_d_n8;
        locals.var_rdrmuevbs_dn9 = assign12190_e6718_d_n9;
        locals.var_rdrmuevbs_dn10 = assign12190_e6718_d_n10;
        locals.var_rdrmuevbs_dn13 = assign12190_e6718_d_n13;
        locals.var_rdrmuevbs_rv = 0.0;

        let (assign12200_e6730,) = {
    if (locals.var_guard273 == 0.0) {
        let assign12200_e6723: f64 = (p.p419 * p.p419);
        let assign12200_e6726: f64 = (locals.var_uc_xldld * locals.var_uc_xldld);
        let assign12200_e6727: f64 = (assign12200_e6723 + assign12200_e6726);
        let assign12200_e6728: f64 = (assign12200_e6727).sqrt();
        (assign12200_e6728,)
    } else {
        (locals.var_xmax,)
    }
};
        locals.var_xmax = assign12200_e6730;
        locals.var_xmax_rv = 0.0;

        let (assign12210_e6742,) = {
    if (locals.var_guard273 == 0.0) {
        let assign12210_e6735: f64 = (p.p419 * p.p419);
        let assign12210_e6738: f64 = (p.p97 * p.p97);
        let assign12210_e6739: f64 = (assign12210_e6735 + assign12210_e6738);
        let assign12210_e6740: f64 = (assign12210_e6739).sqrt();
        (assign12210_e6740,)
    } else {
        (locals.var_xmax_s,)
    }
};
        locals.var_xmax_s = assign12210_e6742;
        locals.var_xmax_s_rv = 0.0;

        let (assign12220_e6753,) = {
    if (locals.var_guard273 == 0.0) {
        let assign12220_e6749: f64 = (locals.var_wg).powf(p.p425);
        let assign12220_e6750: f64 = (p.p424 / assign12220_e6749);
        let assign12220_e6751: f64 = (1.0 + assign12220_e6750);
        (assign12220_e6751,)
    } else {
        (locals.var_rdrvmaxwe,)
    }
};
        locals.var_rdrvmaxwe = assign12220_e6753;
        locals.var_rdrvmaxwe_rv = 0.0;

        let (assign12230_e6764,) = {
    if (locals.var_guard273 == 0.0) {
        let assign12230_e6760: f64 = (locals.var_lg).powf(p.p427);
        let assign12230_e6761: f64 = (p.p426 / assign12230_e6760);
        let assign12230_e6762: f64 = (1.0 + assign12230_e6761);
        (assign12230_e6762,)
    } else {
        (locals.var_rdrvmaxle,)
    }
};
        locals.var_rdrvmaxle = assign12230_e6764;
        locals.var_rdrvmaxle_rv = 0.0;

        let (assign12240_e6775,) = {
    if (locals.var_guard273 == 0.0) {
        let assign12240_e6771: f64 = (locals.var_lg).powf(p.p429);
        let assign12240_e6772: f64 = (p.p428 / assign12240_e6771);
        let assign12240_e6773: f64 = (1.0 + assign12240_e6772);
        (assign12240_e6773,)
    } else {
        (locals.var_rdrmuele,)
    }
};
        locals.var_rdrmuele = assign12240_e6775;
        locals.var_rdrmuele_rv = 0.0;

        let (assign12250_e6780, assign12250_e6780_d_n0, assign12250_e6780_d_n2, assign12250_e6780_d_n4, assign12250_e6780_d_n5, assign12250_e6780_d_n6, assign12250_e6780_d_n7, assign12250_e6780_d_n8, assign12250_e6780_d_n9, assign12250_e6780_d_n10, assign12250_e6780_d_n13,) = {
    if (locals.var_guard273 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn13,)
    }
};
        locals.var_rdrmuevbs = assign12250_e6780;
        locals.var_rdrmuevbs_dn0 = assign12250_e6780_d_n0;
        locals.var_rdrmuevbs_dn2 = assign12250_e6780_d_n2;
        locals.var_rdrmuevbs_dn4 = assign12250_e6780_d_n4;
        locals.var_rdrmuevbs_dn5 = assign12250_e6780_d_n5;
        locals.var_rdrmuevbs_dn6 = assign12250_e6780_d_n6;
        locals.var_rdrmuevbs_dn7 = assign12250_e6780_d_n7;
        locals.var_rdrmuevbs_dn8 = assign12250_e6780_d_n8;
        locals.var_rdrmuevbs_dn9 = assign12250_e6780_d_n9;
        locals.var_rdrmuevbs_dn10 = assign12250_e6780_d_n10;
        locals.var_rdrmuevbs_dn13 = assign12250_e6780_d_n13;
        locals.var_rdrmuevbs_rv = 0.0;

        let (assign12260_e6785,) = {
    if (locals.var_guard273 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rdtemp0,)
    }
};
        locals.var_rdtemp0 = assign12260_e6785;
        locals.var_rdtemp0_rv = 0.0;

        let (assign12270_e6790, assign12270_e6790_d_n0, assign12270_e6790_d_n2, assign12270_e6790_d_n4, assign12270_e6790_d_n5, assign12270_e6790_d_n6, assign12270_e6790_d_n7, assign12270_e6790_d_n8, assign12270_e6790_d_n9, assign12270_e6790_d_n10, assign12270_e6790_d_n13,) = {
    if (locals.var_guard273 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn13,)
    }
};
        locals.var_rdvdtemp0 = assign12270_e6790;
        locals.var_rdvdtemp0_dn0 = assign12270_e6790_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12270_e6790_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12270_e6790_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12270_e6790_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12270_e6790_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12270_e6790_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12270_e6790_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12270_e6790_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12270_e6790_d_n10;
        locals.var_rdvdtemp0_dn13 = assign12270_e6790_d_n13;
        locals.var_rdvdtemp0_rv = 0.0;

        let (assign12280_e6795, assign12280_e6795_d_n0, assign12280_e6795_d_n2, assign12280_e6795_d_n4, assign12280_e6795_d_n5, assign12280_e6795_d_n6, assign12280_e6795_d_n7, assign12280_e6795_d_n8, assign12280_e6795_d_n9, assign12280_e6795_d_n10, assign12280_e6795_d_n13,) = {
    if (locals.var_guard273 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn13,)
    }
};
        locals.var_rd23e = assign12280_e6795;
        locals.var_rd23e_dn0 = assign12280_e6795_d_n0;
        locals.var_rd23e_dn2 = assign12280_e6795_d_n2;
        locals.var_rd23e_dn4 = assign12280_e6795_d_n4;
        locals.var_rd23e_dn5 = assign12280_e6795_d_n5;
        locals.var_rd23e_dn6 = assign12280_e6795_d_n6;
        locals.var_rd23e_dn7 = assign12280_e6795_d_n7;
        locals.var_rd23e_dn8 = assign12280_e6795_d_n8;
        locals.var_rd23e_dn9 = assign12280_e6795_d_n9;
        locals.var_rd23e_dn10 = assign12280_e6795_d_n10;
        locals.var_rd23e_dn13 = assign12280_e6795_d_n13;
        locals.var_rd23e_rv = 0.0;

        let assign12290_e6798: f64 = if locals.var_uc_nover > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign12290_e6798;
        locals.var_guard279_rv = 0.0;

        let (assign12300_e6808,) = {
    if (locals.var_guard279 != 0.0) {
        let assign12300_e6802: f64 = (2.0 * 1.034943e-10);
        let assign12300_e6805: f64 = (1.6021918e-19 * locals.var_uc_nover);
        let assign12300_e6806: f64 = (assign12300_e6802 / assign12300_e6805);
        (assign12300_e6806,)
    } else {
        (locals.var_kdep,)
    }
};
        locals.var_kdep = assign12300_e6808;
        locals.var_kdep_rv = 0.0;

        let (assign12310_e6824, assign12310_e6824_d_n0, assign12310_e6824_d_n2, assign12310_e6824_d_n4, assign12310_e6824_d_n5, assign12310_e6824_d_n6, assign12310_e6824_d_n7, assign12310_e6824_d_n8, assign12310_e6824_d_n9, assign12310_e6824_d_n10, assign12310_e6824_d_n13,) = {
    if (locals.var_guard279 != 0.0) {
        let assign12310_e6812: f64 = (2.0 * 1.034943e-10);
        let assign12310_e6814: f64 = (assign12310_e6812 / 1.6021918e-19);
        let assign12310_e6816: f64 = (assign12310_e6814 * locals.var_ef_nsubc);
        let assign12310_e6819: f64 = (locals.var_uc_nover + locals.var_ef_nsubc);
        let assign12310_e6820: f64 = (assign12310_e6816 / assign12310_e6819);
        let assign12310_e6822: f64 = (assign12310_e6820 / locals.var_uc_nover);
        (assign12310_e6822, (((((assign12310_e6814 * locals.var_ef_nsubc_dn0) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn0)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn2) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn2)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn4) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn4)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn5) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn5)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn6) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn6)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn7) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn7)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn8) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn8)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn9) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn9)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn10) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn10)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover), (((((assign12310_e6814 * locals.var_ef_nsubc_dn13) * assign12310_e6819) - (assign12310_e6816 * locals.var_ef_nsubc_dn13)) / (assign12310_e6819 * assign12310_e6819)) / locals.var_uc_nover),)
    } else {
        (locals.var_kjunc, locals.var_kjunc_dn0, locals.var_kjunc_dn2, locals.var_kjunc_dn4, locals.var_kjunc_dn5, locals.var_kjunc_dn6, locals.var_kjunc_dn7, locals.var_kjunc_dn8, locals.var_kjunc_dn9, locals.var_kjunc_dn10, locals.var_kjunc_dn13,)
    }
};
        locals.var_kjunc = assign12310_e6824;
        locals.var_kjunc_dn0 = assign12310_e6824_d_n0;
        locals.var_kjunc_dn2 = assign12310_e6824_d_n2;
        locals.var_kjunc_dn4 = assign12310_e6824_d_n4;
        locals.var_kjunc_dn5 = assign12310_e6824_d_n5;
        locals.var_kjunc_dn6 = assign12310_e6824_d_n6;
        locals.var_kjunc_dn7 = assign12310_e6824_d_n7;
        locals.var_kjunc_dn8 = assign12310_e6824_d_n8;
        locals.var_kjunc_dn9 = assign12310_e6824_d_n9;
        locals.var_kjunc_dn10 = assign12310_e6824_d_n10;
        locals.var_kjunc_dn13 = assign12310_e6824_d_n13;
        locals.var_kjunc_rv = 0.0;

        let (assign12320_e6829,) = {
    if (locals.var_guard279 == 0.0) {
        (0.0,)
    } else {
        (locals.var_kdep,)
    }
};
        locals.var_kdep = assign12320_e6829;
        locals.var_kdep_rv = 0.0;

        let (assign12330_e6834, assign12330_e6834_d_n0, assign12330_e6834_d_n2, assign12330_e6834_d_n4, assign12330_e6834_d_n5, assign12330_e6834_d_n6, assign12330_e6834_d_n7, assign12330_e6834_d_n8, assign12330_e6834_d_n9, assign12330_e6834_d_n10, assign12330_e6834_d_n13,) = {
    if (locals.var_guard279 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kjunc, locals.var_kjunc_dn0, locals.var_kjunc_dn2, locals.var_kjunc_dn4, locals.var_kjunc_dn5, locals.var_kjunc_dn6, locals.var_kjunc_dn7, locals.var_kjunc_dn8, locals.var_kjunc_dn9, locals.var_kjunc_dn10, locals.var_kjunc_dn13,)
    }
};
        locals.var_kjunc = assign12330_e6834;
        locals.var_kjunc_dn0 = assign12330_e6834_d_n0;
        locals.var_kjunc_dn2 = assign12330_e6834_d_n2;
        locals.var_kjunc_dn4 = assign12330_e6834_d_n4;
        locals.var_kjunc_dn5 = assign12330_e6834_d_n5;
        locals.var_kjunc_dn6 = assign12330_e6834_d_n6;
        locals.var_kjunc_dn7 = assign12330_e6834_d_n7;
        locals.var_kjunc_dn8 = assign12330_e6834_d_n8;
        locals.var_kjunc_dn9 = assign12330_e6834_d_n9;
        locals.var_kjunc_dn10 = assign12330_e6834_d_n10;
        locals.var_kjunc_dn13 = assign12330_e6834_d_n13;
        locals.var_kjunc_rv = 0.0;

        let assign12470_e6929: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard284 = assign12470_e6929;
        locals.var_guard284_rv = 0.0;

        let (assign12480_e6937, assign12480_e6937_d_n0, assign12480_e6937_d_n2, assign12480_e6937_d_n4, assign12480_e6937_d_n5, assign12480_e6937_d_n6, assign12480_e6937_d_n7, assign12480_e6937_d_n8, assign12480_e6937_d_n9, assign12480_e6937_d_n10, assign12480_e6937_d_n13,) = {
    if (locals.var_guard284 != 0.0) {
        let assign12480_e6933: f64 = (p.p108 * locals.var_lg);
        let assign12480_e6935: f64 = (assign12480_e6933 + p.p109);
        (assign12480_e6935, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign12480_e6937;
        locals.var_t1_dn0 = assign12480_e6937_d_n0;
        locals.var_t1_dn2 = assign12480_e6937_d_n2;
        locals.var_t1_dn4 = assign12480_e6937_d_n4;
        locals.var_t1_dn5 = assign12480_e6937_d_n5;
        locals.var_t1_dn6 = assign12480_e6937_d_n6;
        locals.var_t1_dn7 = assign12480_e6937_d_n7;
        locals.var_t1_dn8 = assign12480_e6937_d_n8;
        locals.var_t1_dn9 = assign12480_e6937_d_n9;
        locals.var_t1_dn10 = assign12480_e6937_d_n10;
        locals.var_t1_dn13 = assign12480_e6937_d_n13;
        locals.var_t1_rv = 0.0;

        let assign12490_e6940: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard285 = assign12490_e6940;
        locals.var_guard285_rv = 0.0;

        let (assign12500_e6946, assign12500_e6946_d_n0, assign12500_e6946_d_n2, assign12500_e6946_d_n4, assign12500_e6946_d_n5, assign12500_e6946_d_n6, assign12500_e6946_d_n7, assign12500_e6946_d_n8, assign12500_e6946_d_n9, assign12500_e6946_d_n10, assign12500_e6946_d_n13,) = {
    if ((locals.var_guard284 != 0.0) && (locals.var_guard285 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign12500_e6946;
        locals.var_t1_dn0 = assign12500_e6946_d_n0;
        locals.var_t1_dn2 = assign12500_e6946_d_n2;
        locals.var_t1_dn4 = assign12500_e6946_d_n4;
        locals.var_t1_dn5 = assign12500_e6946_d_n5;
        locals.var_t1_dn6 = assign12500_e6946_d_n6;
        locals.var_t1_dn7 = assign12500_e6946_d_n7;
        locals.var_t1_dn8 = assign12500_e6946_d_n8;
        locals.var_t1_dn9 = assign12500_e6946_d_n9;
        locals.var_t1_dn10 = assign12500_e6946_d_n10;
        locals.var_t1_dn13 = assign12500_e6946_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign12510_e6958, assign12510_e6958_d_n0, assign12510_e6958_d_n2, assign12510_e6958_d_n4, assign12510_e6958_d_n5, assign12510_e6958_d_n6, assign12510_e6958_d_n7, assign12510_e6958_d_n8, assign12510_e6958_d_n9, assign12510_e6958_d_n10, assign12510_e6958_d_n13,) = {
    if (locals.var_guard284 != 0.0) {
        let assign12510_e6950: f64 = (locals.var_t1 * p.p107);
        let assign12510_e6953: f64 = (locals.var_t1 + p.p107);
        let assign12510_e6954: f64 = (assign12510_e6950 / assign12510_e6953);
        let assign12510_e6956: f64 = (assign12510_e6954 + 1.0);
        (assign12510_e6956, ((((locals.var_t1_dn0 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn0)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn2 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn2)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn4 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn4)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn5 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn5)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn6 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn6)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn7 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn7)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn8 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn8)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn9 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn9)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn10 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn10)) / (assign12510_e6953 * assign12510_e6953)), ((((locals.var_t1_dn13 * p.p107) * assign12510_e6953) - (assign12510_e6950 * locals.var_t1_dn13)) / (assign12510_e6953 * assign12510_e6953)),)
    } else {
        (locals.var_ddlte, locals.var_ddlte_dn0, locals.var_ddlte_dn2, locals.var_ddlte_dn4, locals.var_ddlte_dn5, locals.var_ddlte_dn6, locals.var_ddlte_dn7, locals.var_ddlte_dn8, locals.var_ddlte_dn9, locals.var_ddlte_dn10, locals.var_ddlte_dn13,)
    }
};
        locals.var_ddlte = assign12510_e6958;
        locals.var_ddlte_dn0 = assign12510_e6958_d_n0;
        locals.var_ddlte_dn2 = assign12510_e6958_d_n2;
        locals.var_ddlte_dn4 = assign12510_e6958_d_n4;
        locals.var_ddlte_dn5 = assign12510_e6958_d_n5;
        locals.var_ddlte_dn6 = assign12510_e6958_d_n6;
        locals.var_ddlte_dn7 = assign12510_e6958_d_n7;
        locals.var_ddlte_dn8 = assign12510_e6958_d_n8;
        locals.var_ddlte_dn9 = assign12510_e6958_d_n9;
        locals.var_ddlte_dn10 = assign12510_e6958_d_n10;
        locals.var_ddlte_dn13 = assign12510_e6958_d_n13;
        locals.var_ddlte_rv = 0.0;

        let (assign12520_e6965, assign12520_e6965_d_n0, assign12520_e6965_d_n2, assign12520_e6965_d_n4, assign12520_e6965_d_n5, assign12520_e6965_d_n6, assign12520_e6965_d_n7, assign12520_e6965_d_n8, assign12520_e6965_d_n9, assign12520_e6965_d_n10, assign12520_e6965_d_n13,) = {
    if (locals.var_guard284 == 0.0) {
        let assign12520_e6963: f64 = (p.p108 * locals.var_lg);
        (assign12520_e6963, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign12520_e6965;
        locals.var_t1_dn0 = assign12520_e6965_d_n0;
        locals.var_t1_dn2 = assign12520_e6965_d_n2;
        locals.var_t1_dn4 = assign12520_e6965_d_n4;
        locals.var_t1_dn5 = assign12520_e6965_d_n5;
        locals.var_t1_dn6 = assign12520_e6965_d_n6;
        locals.var_t1_dn7 = assign12520_e6965_d_n7;
        locals.var_t1_dn8 = assign12520_e6965_d_n8;
        locals.var_t1_dn9 = assign12520_e6965_d_n9;
        locals.var_t1_dn10 = assign12520_e6965_d_n10;
        locals.var_t1_dn13 = assign12520_e6965_d_n13;
        locals.var_t1_rv = 0.0;

        let assign12530_e6968: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard286 = assign12530_e6968;
        locals.var_guard286_rv = 0.0;

        let (assign12540_e6975, assign12540_e6975_d_n0, assign12540_e6975_d_n2, assign12540_e6975_d_n4, assign12540_e6975_d_n5, assign12540_e6975_d_n6, assign12540_e6975_d_n7, assign12540_e6975_d_n8, assign12540_e6975_d_n9, assign12540_e6975_d_n10, assign12540_e6975_d_n13,) = {
    if ((locals.var_guard284 == 0.0) && (locals.var_guard286 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign12540_e6975;
        locals.var_t1_dn0 = assign12540_e6975_d_n0;
        locals.var_t1_dn2 = assign12540_e6975_d_n2;
        locals.var_t1_dn4 = assign12540_e6975_d_n4;
        locals.var_t1_dn5 = assign12540_e6975_d_n5;
        locals.var_t1_dn6 = assign12540_e6975_d_n6;
        locals.var_t1_dn7 = assign12540_e6975_d_n7;
        locals.var_t1_dn8 = assign12540_e6975_d_n8;
        locals.var_t1_dn9 = assign12540_e6975_d_n9;
        locals.var_t1_dn10 = assign12540_e6975_d_n10;
        locals.var_t1_dn13 = assign12540_e6975_d_n13;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12550_e6990, assign12550_e6990_d_n0, assign12550_e6990_d_n2, assign12550_e6990_d_n4, assign12550_e6990_d_n5, assign12550_e6990_d_n6, assign12550_e6990_d_n7, assign12550_e6990_d_n8, assign12550_e6990_d_n9, assign12550_e6990_d_n10, assign12550_e6990_d_n13,) = {
    if (locals.var_guard284 == 0.0) {
        let assign12550_e6980: f64 = (locals.var_t1 * p.p107);
        let assign12550_e6983: f64 = (locals.var_t1 + p.p107);
        let assign12550_e6984: f64 = (assign12550_e6980 / assign12550_e6983);
        let assign12550_e6986: f64 = (assign12550_e6984 + p.p109);
        let assign12550_e6988: f64 = (assign12550_e6986 + 1e-25);
        (assign12550_e6988, ((((locals.var_t1_dn0 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn0)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn2 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn2)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn4 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn4)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn5 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn5)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn6 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn6)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn7 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn7)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn8 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn8)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn9 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn9)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn10 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn10)) / (assign12550_e6983 * assign12550_e6983)), ((((locals.var_t1_dn13 * p.p107) * assign12550_e6983) - (assign12550_e6980 * locals.var_t1_dn13)) / (assign12550_e6983 * assign12550_e6983)),)
    } else {
        (locals.var_ddlte, locals.var_ddlte_dn0, locals.var_ddlte_dn2, locals.var_ddlte_dn4, locals.var_ddlte_dn5, locals.var_ddlte_dn6, locals.var_ddlte_dn7, locals.var_ddlte_dn8, locals.var_ddlte_dn9, locals.var_ddlte_dn10, locals.var_ddlte_dn13,)
    }
};
        locals.var_ddlte = assign12550_e6990;
        locals.var_ddlte_dn0 = assign12550_e6990_d_n0;
        locals.var_ddlte_dn2 = assign12550_e6990_d_n2;
        locals.var_ddlte_dn4 = assign12550_e6990_d_n4;
        locals.var_ddlte_dn5 = assign12550_e6990_d_n5;
        locals.var_ddlte_dn6 = assign12550_e6990_d_n6;
        locals.var_ddlte_dn7 = assign12550_e6990_d_n7;
        locals.var_ddlte_dn8 = assign12550_e6990_d_n8;
        locals.var_ddlte_dn9 = assign12550_e6990_d_n9;
        locals.var_ddlte_dn10 = assign12550_e6990_d_n10;
        locals.var_ddlte_dn13 = assign12550_e6990_d_n13;
        locals.var_ddlte_rv = 0.0;

        let assign12570_e6998: f64 = if locals.var_ddlte < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard288 = assign12570_e6998;
        locals.var_guard288_rv = 0.0;

        let (assign12580_e7002, assign12580_e7002_d_n0, assign12580_e7002_d_n2, assign12580_e7002_d_n4, assign12580_e7002_d_n5, assign12580_e7002_d_n6, assign12580_e7002_d_n7, assign12580_e7002_d_n8, assign12580_e7002_d_n9, assign12580_e7002_d_n10, assign12580_e7002_d_n13,) = {
    if (locals.var_guard288 != 0.0) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ddlte, locals.var_ddlte_dn0, locals.var_ddlte_dn2, locals.var_ddlte_dn4, locals.var_ddlte_dn5, locals.var_ddlte_dn6, locals.var_ddlte_dn7, locals.var_ddlte_dn8, locals.var_ddlte_dn9, locals.var_ddlte_dn10, locals.var_ddlte_dn13,)
    }
};
        locals.var_ddlte = assign12580_e7002;
        locals.var_ddlte_dn0 = assign12580_e7002_d_n0;
        locals.var_ddlte_dn2 = assign12580_e7002_d_n2;
        locals.var_ddlte_dn4 = assign12580_e7002_d_n4;
        locals.var_ddlte_dn5 = assign12580_e7002_d_n5;
        locals.var_ddlte_dn6 = assign12580_e7002_d_n6;
        locals.var_ddlte_dn7 = assign12580_e7002_d_n7;
        locals.var_ddlte_dn8 = assign12580_e7002_d_n8;
        locals.var_ddlte_dn9 = assign12580_e7002_d_n9;
        locals.var_ddlte_dn10 = assign12580_e7002_d_n10;
        locals.var_ddlte_dn13 = assign12580_e7002_d_n13;
        locals.var_ddlte_rv = 0.0;

        let (assign12590_e7008, assign12590_e7008_d_n0, assign12590_e7008_d_n2, assign12590_e7008_d_n4, assign12590_e7008_d_n5, assign12590_e7008_d_n6, assign12590_e7008_d_n7, assign12590_e7008_d_n8, assign12590_e7008_d_n9, assign12590_e7008_d_n10, assign12590_e7008_d_n13,) = {
    if (p.p23 != 0.0) {
        let assign12590_e7006: f64 = (locals.var_weff).powf(p.p201);
        (assign12590_e7006, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign12590_e7008;
        locals.var_t2_dn0 = assign12590_e7008_d_n0;
        locals.var_t2_dn2 = assign12590_e7008_d_n2;
        locals.var_t2_dn4 = assign12590_e7008_d_n4;
        locals.var_t2_dn5 = assign12590_e7008_d_n5;
        locals.var_t2_dn6 = assign12590_e7008_d_n6;
        locals.var_t2_dn7 = assign12590_e7008_d_n7;
        locals.var_t2_dn8 = assign12590_e7008_d_n8;
        locals.var_t2_dn9 = assign12590_e7008_d_n9;
        locals.var_t2_dn10 = assign12590_e7008_d_n10;
        locals.var_t2_dn13 = assign12590_e7008_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign12600_e7026, assign12600_e7026_d_n0, assign12600_e7026_d_n2, assign12600_e7026_d_n4, assign12600_e7026_d_n5, assign12600_e7026_d_n6, assign12600_e7026_d_n7, assign12600_e7026_d_n8, assign12600_e7026_d_n9, assign12600_e7026_d_n10, assign12600_e7026_d_n13,) = {
    if (p.p23 != 0.0) {
        let assign12600_e7015: f64 = (locals.var_lgate).powf(p.p199);
        let assign12600_e7016: f64 = (locals.var_mks_svgsl / assign12600_e7015);
        let assign12600_e7017: f64 = (1.0 + assign12600_e7016);
        let assign12600_e7018: f64 = (locals.var_uc_svgs * assign12600_e7017);
        let assign12600_e7022: f64 = (locals.var_t2 + locals.var_mks_svgsw);
        let assign12600_e7023: f64 = (locals.var_t2 / assign12600_e7022);
        let assign12600_e7024: f64 = (assign12600_e7018 * assign12600_e7023);
        (assign12600_e7024, (assign12600_e7018 * (((locals.var_t2_dn0 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn0)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn2 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn2)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn4 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn4)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn5 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn5)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn6 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn6)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn7 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn7)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn8 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn8)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn9 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn9)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn10 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn10)) / (assign12600_e7022 * assign12600_e7022))), (assign12600_e7018 * (((locals.var_t2_dn13 * assign12600_e7022) - (locals.var_t2 * locals.var_t2_dn13)) / (assign12600_e7022 * assign12600_e7022))),)
    } else {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn4, locals.var_vg2const_dn5, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn8, locals.var_vg2const_dn9, locals.var_vg2const_dn10, locals.var_vg2const_dn13,)
    }
};
        locals.var_vg2const = assign12600_e7026;
        locals.var_vg2const_dn0 = assign12600_e7026_d_n0;
        locals.var_vg2const_dn2 = assign12600_e7026_d_n2;
        locals.var_vg2const_dn4 = assign12600_e7026_d_n4;
        locals.var_vg2const_dn5 = assign12600_e7026_d_n5;
        locals.var_vg2const_dn6 = assign12600_e7026_d_n6;
        locals.var_vg2const_dn7 = assign12600_e7026_d_n7;
        locals.var_vg2const_dn8 = assign12600_e7026_d_n8;
        locals.var_vg2const_dn9 = assign12600_e7026_d_n9;
        locals.var_vg2const_dn10 = assign12600_e7026_d_n10;
        locals.var_vg2const_dn13 = assign12600_e7026_d_n13;
        locals.var_vg2const_rv = 0.0;

        let (assign12610_e7038,) = {
    if (p.p23 != 0.0) {
        let assign12610_e7033: f64 = (locals.var_lgate).powf(p.p184);
        let assign12610_e7034: f64 = (locals.var_mks_svbsl / assign12610_e7033);
        let assign12610_e7035: f64 = (1.0 + assign12610_e7034);
        let assign12610_e7036: f64 = (locals.var_uc_svbs * assign12610_e7035);
        (assign12610_e7036,)
    } else {
        (locals.var_xvbs,)
    }
};
        locals.var_xvbs = assign12610_e7038;
        locals.var_xvbs_rv = 0.0;

        let (assign12620_e7050,) = {
    if (p.p23 != 0.0) {
        let assign12620_e7045: f64 = (locals.var_lgate).powf(p.p203);
        let assign12620_e7046: f64 = (locals.var_mks_slgl / assign12620_e7045);
        let assign12620_e7047: f64 = (1.0 + assign12620_e7046);
        let assign12620_e7048: f64 = (locals.var_mks_slg * assign12620_e7047);
        (assign12620_e7048,)
    } else {
        (locals.var_xgate,)
    }
};
        locals.var_xgate = assign12620_e7050;
        locals.var_xgate_rv = 0.0;

        let (assign12630_e7062,) = {
    if (p.p23 != 0.0) {
        let assign12630_e7057: f64 = (locals.var_lgate).powf(p.p191);
        let assign12630_e7058: f64 = (locals.var_mks_sub1l / assign12630_e7057);
        let assign12630_e7059: f64 = (1.0 + assign12630_e7058);
        let assign12630_e7060: f64 = (locals.var_uc_sub1 * assign12630_e7059);
        (assign12630_e7060,)
    } else {
        (locals.var_xsub1,)
    }
};
        locals.var_xsub1 = assign12630_e7062;
        locals.var_xsub1_rv = 0.0;

        let (assign12640_e7072,) = {
    if (p.p23 != 0.0) {
        let assign12640_e7068: f64 = (locals.var_mks_sub2l / locals.var_lgate);
        let assign12640_e7069: f64 = (1.0 + assign12640_e7068);
        let assign12640_e7070: f64 = (locals.var_uc_sub2 * assign12640_e7069);
        (assign12640_e7070,)
    } else {
        (locals.var_xsub2,)
    }
};
        locals.var_xsub2 = assign12640_e7072;
        locals.var_xsub2_rv = 0.0;

        let (assign12650_e7076,) = {
    if (p.p23 != 0.0) {
        (locals.var_xsub1,)
    } else {
        (locals.var_xsub1_1,)
    }
};
        locals.var_xsub1_1 = assign12650_e7076;
        locals.var_xsub1_1_rv = 0.0;

        let (assign12660_e7080,) = {
    if (p.p23 != 0.0) {
        (locals.var_xsub2,)
    } else {
        (locals.var_xsub2_1,)
    }
};
        locals.var_xsub2_1 = assign12660_e7080;
        locals.var_xsub2_1_rv = 0.0;

        let (assign12670_e7084, assign12670_e7084_d_n0, assign12670_e7084_d_n2, assign12670_e7084_d_n4, assign12670_e7084_d_n5, assign12670_e7084_d_n6, assign12670_e7084_d_n7, assign12670_e7084_d_n8, assign12670_e7084_d_n9, assign12670_e7084_d_n10, assign12670_e7084_d_n13,) = {
    if (p.p23 != 0.0) {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn4, locals.var_vg2const_dn5, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn8, locals.var_vg2const_dn9, locals.var_vg2const_dn10, locals.var_vg2const_dn13,)
    } else {
        (locals.var_vg2const_1, locals.var_vg2const_1_dn0, locals.var_vg2const_1_dn2, locals.var_vg2const_1_dn4, locals.var_vg2const_1_dn5, locals.var_vg2const_1_dn6, locals.var_vg2const_1_dn7, locals.var_vg2const_1_dn8, locals.var_vg2const_1_dn9, locals.var_vg2const_1_dn10, locals.var_vg2const_1_dn13,)
    }
};
        locals.var_vg2const_1 = assign12670_e7084;
        locals.var_vg2const_1_dn0 = assign12670_e7084_d_n0;
        locals.var_vg2const_1_dn2 = assign12670_e7084_d_n2;
        locals.var_vg2const_1_dn4 = assign12670_e7084_d_n4;
        locals.var_vg2const_1_dn5 = assign12670_e7084_d_n5;
        locals.var_vg2const_1_dn6 = assign12670_e7084_d_n6;
        locals.var_vg2const_1_dn7 = assign12670_e7084_d_n7;
        locals.var_vg2const_1_dn8 = assign12670_e7084_d_n8;
        locals.var_vg2const_1_dn9 = assign12670_e7084_d_n9;
        locals.var_vg2const_1_dn10 = assign12670_e7084_d_n10;
        locals.var_vg2const_1_dn13 = assign12670_e7084_d_n13;
        locals.var_vg2const_1_rv = 0.0;

        let (assign12680_e7088,) = {
    if (p.p23 != 0.0) {
        (locals.var_xvbs,)
    } else {
        (locals.var_xvbs_1,)
    }
};
        locals.var_xvbs_1 = assign12680_e7088;
        locals.var_xvbs_1_rv = 0.0;

        let (assign12690_e7092,) = {
    if (p.p23 != 0.0) {
        (locals.var_xgate,)
    } else {
        (locals.var_xgate_1,)
    }
};
        locals.var_xgate_1 = assign12690_e7092;
        locals.var_xgate_1_rv = 0.0;

        let (assign12700_e7106,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12700_e7101: f64 = (locals.var_lgate).powf(p.p191);
        let assign12700_e7102: f64 = (locals.var_mks_sub1l / assign12700_e7101);
        let assign12700_e7103: f64 = (1.0 + assign12700_e7102);
        let assign12700_e7104: f64 = (locals.var_uc_sub1snp * assign12700_e7103);
        (assign12700_e7104,)
    } else {
        (locals.var_xsub1_1,)
    }
};
        locals.var_xsub1_1 = assign12700_e7106;
        locals.var_xsub1_1_rv = 0.0;

        let (assign12710_e7118,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12710_e7114: f64 = (locals.var_mks_sub2l / locals.var_lgate);
        let assign12710_e7115: f64 = (1.0 + assign12710_e7114);
        let assign12710_e7116: f64 = (locals.var_uc_sub2snp * assign12710_e7115);
        (assign12710_e7116,)
    } else {
        (locals.var_xsub2_1,)
    }
};
        locals.var_xsub2_1 = assign12710_e7118;
        locals.var_xsub2_1_rv = 0.0;

        let (assign12720_e7130,) = {
    if (p.p23 != 0.0) {
        let assign12720_e7125: f64 = (locals.var_lg).powf(p.p103);
        let assign12720_e7126: f64 = (p.p102 / assign12720_e7125);
        let assign12720_e7127: f64 = (1.0 + assign12720_e7126);
        let assign12720_e7128: f64 = (p.p72 * assign12720_e7127);
        (assign12720_e7128,)
    } else {
        (locals.var_uc_subld1,)
    }
};
        locals.var_uc_subld1 = assign12720_e7130;
        locals.var_uc_subld1_rv = 0.0;

        let (assign12730_e7135, assign12730_e7135_d_n0, assign12730_e7135_d_n2, assign12730_e7135_d_n4, assign12730_e7135_d_n5, assign12730_e7135_d_n6, assign12730_e7135_d_n7, assign12730_e7135_d_n8, assign12730_e7135_d_n9, assign12730_e7135_d_n10, assign12730_e7135_d_n13,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn4, locals.var_vg2const_dn5, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn8, locals.var_vg2const_dn9, locals.var_vg2const_dn10, locals.var_vg2const_dn13,)
    }
};
        locals.var_vg2const = assign12730_e7135;
        locals.var_vg2const_dn0 = assign12730_e7135_d_n0;
        locals.var_vg2const_dn2 = assign12730_e7135_d_n2;
        locals.var_vg2const_dn4 = assign12730_e7135_d_n4;
        locals.var_vg2const_dn5 = assign12730_e7135_d_n5;
        locals.var_vg2const_dn6 = assign12730_e7135_d_n6;
        locals.var_vg2const_dn7 = assign12730_e7135_d_n7;
        locals.var_vg2const_dn8 = assign12730_e7135_d_n8;
        locals.var_vg2const_dn9 = assign12730_e7135_d_n9;
        locals.var_vg2const_dn10 = assign12730_e7135_d_n10;
        locals.var_vg2const_dn13 = assign12730_e7135_d_n13;
        locals.var_vg2const_rv = 0.0;

        let (assign12740_e7140,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xvbs,)
    }
};
        locals.var_xvbs = assign12740_e7140;
        locals.var_xvbs_rv = 0.0;

        let (assign12750_e7145,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xgate,)
    }
};
        locals.var_xgate = assign12750_e7145;
        locals.var_xgate_rv = 0.0;

        let (assign12760_e7150,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub1,)
    }
};
        locals.var_xsub1 = assign12760_e7150;
        locals.var_xsub1_rv = 0.0;

        let (assign12770_e7155,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub2,)
    }
};
        locals.var_xsub2 = assign12770_e7155;
        locals.var_xsub2_rv = 0.0;

        let (assign12780_e7160,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_subld1,)
    }
};
        locals.var_uc_subld1 = assign12780_e7160;
        locals.var_uc_subld1_rv = 0.0;

        let (assign12790_e7165, assign12790_e7165_d_n0, assign12790_e7165_d_n2, assign12790_e7165_d_n4, assign12790_e7165_d_n5, assign12790_e7165_d_n6, assign12790_e7165_d_n7, assign12790_e7165_d_n8, assign12790_e7165_d_n9, assign12790_e7165_d_n10, assign12790_e7165_d_n13,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vg2const_1, locals.var_vg2const_1_dn0, locals.var_vg2const_1_dn2, locals.var_vg2const_1_dn4, locals.var_vg2const_1_dn5, locals.var_vg2const_1_dn6, locals.var_vg2const_1_dn7, locals.var_vg2const_1_dn8, locals.var_vg2const_1_dn9, locals.var_vg2const_1_dn10, locals.var_vg2const_1_dn13,)
    }
};
        locals.var_vg2const_1 = assign12790_e7165;
        locals.var_vg2const_1_dn0 = assign12790_e7165_d_n0;
        locals.var_vg2const_1_dn2 = assign12790_e7165_d_n2;
        locals.var_vg2const_1_dn4 = assign12790_e7165_d_n4;
        locals.var_vg2const_1_dn5 = assign12790_e7165_d_n5;
        locals.var_vg2const_1_dn6 = assign12790_e7165_d_n6;
        locals.var_vg2const_1_dn7 = assign12790_e7165_d_n7;
        locals.var_vg2const_1_dn8 = assign12790_e7165_d_n8;
        locals.var_vg2const_1_dn9 = assign12790_e7165_d_n9;
        locals.var_vg2const_1_dn10 = assign12790_e7165_d_n10;
        locals.var_vg2const_1_dn13 = assign12790_e7165_d_n13;
        locals.var_vg2const_1_rv = 0.0;

        let (assign12800_e7170,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xvbs_1,)
    }
};
        locals.var_xvbs_1 = assign12800_e7170;
        locals.var_xvbs_1_rv = 0.0;

        let (assign12810_e7175,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xgate_1,)
    }
};
        locals.var_xgate_1 = assign12810_e7175;
        locals.var_xgate_1_rv = 0.0;

        let (assign12820_e7180,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub1_1,)
    }
};
        locals.var_xsub1_1 = assign12820_e7180;
        locals.var_xsub1_1_rv = 0.0;

        let (assign12830_e7185,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub2_1,)
    }
};
        locals.var_xsub2_1 = assign12830_e7185;
        locals.var_xsub2_1_rv = 0.0;

        let (assign12840_e7199,) = {
    if (locals.var_uc_ibpc1 != 0.0) {
        let assign12840_e7194: f64 = (locals.var_lg).powf(p.p280);
        let assign12840_e7195: f64 = (p.p279 / assign12840_e7194);
        let assign12840_e7196: f64 = (1.0 + assign12840_e7195);
        let assign12840_e7197: f64 = (locals.var_uc_ibpc1 * assign12840_e7196);
        (assign12840_e7197,)
    } else {
        (0.0,)
    }
};
        locals.var_uc_ibpc1 = assign12840_e7199;
        locals.var_uc_ibpc1_rv = 0.0;

        let assign12850_e7203: f64 = (3.141592653589793 / 2.0);
        let assign12850_e7204: f64 = (3.453133e-11 / assign12850_e7203);
        let assign12850_e7206: f64 = (assign12850_e7204 * locals.var_weffcv_nf);
        let assign12850_e7210: f64 = (p.p225 / p.p95);
        let assign12850_e7211: f64 = (1.0 + assign12850_e7210);
        let assign12850_e7212: f64 = (assign12850_e7211).ln();
        let assign12850_e7213: f64 = (assign12850_e7206 * assign12850_e7212);
        locals.var_cfrng = assign12850_e7213;
        locals.var_cfrng_rv = 0.0;

        let (assign12860_e7227,) = {
    if (p.p134 != 0.0) {
        let assign12860_e7219: f64 = (1000000.0 * locals.var_weffcv_nf);
        let assign12860_e7221: f64 = (assign12860_e7219 * p.p134);
        let assign12860_e7224: f64 = (locals.var_lg).powf(p.p135);
        let assign12860_e7225: f64 = (assign12860_e7221 / assign12860_e7224);
        (assign12860_e7225,)
    } else {
        (0.0,)
    }
};
        locals.var_cqyb0 = assign12860_e7227;
        locals.var_cqyb0_rv = 0.0;

        let assign12870_e7231: f64 = (-p.p286);
        let assign12870_e7232: f64 = (locals.var_lg).powf(assign12870_e7231);
        let assign12870_e7233: f64 = (p.p283 * assign12870_e7232);
        locals.var_ptl0 = assign12870_e7233;
        locals.var_ptl0_rv = 0.0;

        let assign12880_e7237: f64 = (-p.p291);
        let assign12880_e7238: f64 = (locals.var_lg).powf(assign12880_e7237);
        let assign12880_e7239: f64 = (p.p290 * assign12880_e7238);
        locals.var_pt40 = assign12880_e7239;
        locals.var_pt40_rv = 0.0;

        let assign12890_e7243: f64 = (locals.var_lg + locals.var_uc_gdld);
        let assign12890_e7245: f64 = (-p.p288);
        let assign12890_e7246: f64 = (assign12890_e7243).powf(assign12890_e7245);
        let assign12890_e7247: f64 = (p.p287 * assign12890_e7246);
        locals.var_gdl0 = assign12890_e7247;
        locals.var_gdl0_rv = 0.0;

        let assign12900_e7251: f64 = (locals.var_mfactor * locals.var_weff_nf);
        let assign12900_e7252: f64 = (locals.var_uc_rth0 / assign12900_e7251);
        let assign12900_e7257: f64 = (locals.var_lg).powf(p.p318);
        let assign12900_e7258: f64 = (p.p317 / assign12900_e7257);
        let assign12900_e7259: f64 = (1.0 + assign12900_e7258);
        let assign12900_e7260: f64 = (assign12900_e7252 * assign12900_e7259);
        let assign12900_e7265: f64 = (locals.var_wg).powf(p.p316);
        let assign12900_e7266: f64 = (p.p315 / assign12900_e7265);
        let assign12900_e7267: f64 = (1.0 + assign12900_e7266);
        let assign12900_e7268: f64 = (assign12900_e7260 * assign12900_e7267);
        locals.var_rth = assign12900_e7268;
        locals.var_rth_dn0 = 0.0;
        locals.var_rth_dn2 = 0.0;
        locals.var_rth_dn4 = 0.0;
        locals.var_rth_dn5 = 0.0;
        locals.var_rth_dn6 = 0.0;
        locals.var_rth_dn7 = 0.0;
        locals.var_rth_dn8 = 0.0;
        locals.var_rth_dn9 = 0.0;
        locals.var_rth_dn10 = 0.0;
        locals.var_rth_dn13 = 0.0;
        locals.var_rth_rv = 0.0;

        let assign12920_e7278: f64 = (p.p7).powf(p.p327);
        let assign12920_e7279: f64 = (1.0 / assign12920_e7278);
        let assign12920_e7280: f64 = (locals.var_rth * assign12920_e7279);
        locals.var_rth = assign12920_e7280;
        locals.var_rth_dn0 = (locals.var_rth_dn0 * assign12920_e7279);
        locals.var_rth_dn2 = (locals.var_rth_dn2 * assign12920_e7279);
        locals.var_rth_dn4 = (locals.var_rth_dn4 * assign12920_e7279);
        locals.var_rth_dn5 = (locals.var_rth_dn5 * assign12920_e7279);
        locals.var_rth_dn6 = (locals.var_rth_dn6 * assign12920_e7279);
        locals.var_rth_dn7 = (locals.var_rth_dn7 * assign12920_e7279);
        locals.var_rth_dn8 = (locals.var_rth_dn8 * assign12920_e7279);
        locals.var_rth_dn9 = (locals.var_rth_dn9 * assign12920_e7279);
        locals.var_rth_dn10 = (locals.var_rth_dn10 * assign12920_e7279);
        locals.var_rth_dn13 = (locals.var_rth_dn13 * assign12920_e7279);
        locals.var_rth_rv = 0.0;

        let assign12930_e7284: f64 = (p.p7).powf(p.p327);
        let assign12930_e7285: f64 = (1.0 / assign12930_e7284);
        let assign12930_e7288: f64 = (locals.var_mfactor * locals.var_weff_nf);
        let assign12930_e7289: f64 = (assign12930_e7285 / assign12930_e7288);
        let assign12930_e7294: f64 = (locals.var_lg).powf(p.p318);
        let assign12930_e7295: f64 = (p.p317 / assign12930_e7294);
        let assign12930_e7296: f64 = (1.0 + assign12930_e7295);
        let assign12930_e7297: f64 = (assign12930_e7289 * assign12930_e7296);
        let assign12930_e7302: f64 = (locals.var_wg).powf(p.p316);
        let assign12930_e7303: f64 = (p.p315 / assign12930_e7302);
        let assign12930_e7304: f64 = (1.0 + assign12930_e7303);
        let assign12930_e7305: f64 = (assign12930_e7297 * assign12930_e7304);
        locals.var_rthtemp0 = assign12930_e7305;
        locals.var_rthtemp0_rv = 0.0;

        let assign12940_e7312: f64 = if ((p.p53 == 0.0) || (locals.var_uc_rth0 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard289 = assign12940_e7312;
        locals.var_guard289_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_22(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign12950_e7316, assign12950_e7316_d_n0, assign12950_e7316_d_n2, assign12950_e7316_d_n4, assign12950_e7316_d_n5, assign12950_e7316_d_n6, assign12950_e7316_d_n7, assign12950_e7316_d_n8, assign12950_e7316_d_n9, assign12950_e7316_d_n10, assign12950_e7316_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    }
};
        locals.var_cnst0over = assign12950_e7316;
        locals.var_cnst0over_dn0 = assign12950_e7316_d_n0;
        locals.var_cnst0over_dn2 = assign12950_e7316_d_n2;
        locals.var_cnst0over_dn4 = assign12950_e7316_d_n4;
        locals.var_cnst0over_dn5 = assign12950_e7316_d_n5;
        locals.var_cnst0over_dn6 = assign12950_e7316_d_n6;
        locals.var_cnst0over_dn7 = assign12950_e7316_d_n7;
        locals.var_cnst0over_dn8 = assign12950_e7316_d_n8;
        locals.var_cnst0over_dn9 = assign12950_e7316_d_n9;
        locals.var_cnst0over_dn10 = assign12950_e7316_d_n10;
        locals.var_cnst0over_dn13 = assign12950_e7316_d_n13;
        locals.var_cnst0over_rv = 0.0;

        let (assign12960_e7320, assign12960_e7320_d_n0, assign12960_e7320_d_n2, assign12960_e7320_d_n4, assign12960_e7320_d_n5, assign12960_e7320_d_n6, assign12960_e7320_d_n7, assign12960_e7320_d_n8, assign12960_e7320_d_n9, assign12960_e7320_d_n10, assign12960_e7320_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    }
};
        locals.var_cnst0overs = assign12960_e7320;
        locals.var_cnst0overs_dn0 = assign12960_e7320_d_n0;
        locals.var_cnst0overs_dn2 = assign12960_e7320_d_n2;
        locals.var_cnst0overs_dn4 = assign12960_e7320_d_n4;
        locals.var_cnst0overs_dn5 = assign12960_e7320_d_n5;
        locals.var_cnst0overs_dn6 = assign12960_e7320_d_n6;
        locals.var_cnst0overs_dn7 = assign12960_e7320_d_n7;
        locals.var_cnst0overs_dn8 = assign12960_e7320_d_n8;
        locals.var_cnst0overs_dn9 = assign12960_e7320_d_n9;
        locals.var_cnst0overs_dn10 = assign12960_e7320_d_n10;
        locals.var_cnst0overs_dn13 = assign12960_e7320_d_n13;
        locals.var_cnst0overs_rv = 0.0;

        let (assign12970_e7326, assign12970_e7326_d_n0, assign12970_e7326_d_n2, assign12970_e7326_d_n4, assign12970_e7326_d_n5, assign12970_e7326_d_n6, assign12970_e7326_d_n7, assign12970_e7326_d_n8, assign12970_e7326_d_n9, assign12970_e7326_d_n10, assign12970_e7326_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign12970_e7322: f64 = ctx_temp;
        let assign12970_e7324: f64 = (assign12970_e7322 + p.p11);
        (assign12970_e7324, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign12970_e7326;
        locals.var_ttemp_dn0 = assign12970_e7326_d_n0;
        locals.var_ttemp_dn2 = assign12970_e7326_d_n2;
        locals.var_ttemp_dn4 = assign12970_e7326_d_n4;
        locals.var_ttemp_dn5 = assign12970_e7326_d_n5;
        locals.var_ttemp_dn6 = assign12970_e7326_d_n6;
        locals.var_ttemp_dn7 = assign12970_e7326_d_n7;
        locals.var_ttemp_dn8 = assign12970_e7326_d_n8;
        locals.var_ttemp_dn9 = assign12970_e7326_d_n9;
        locals.var_ttemp_dn10 = assign12970_e7326_d_n10;
        locals.var_ttemp_dn13 = assign12970_e7326_d_n13;
        locals.var_ttemp_rv = 0.0;

        let (assign12980_e7330, assign12980_e7330_d_n0, assign12980_e7330_d_n2, assign12980_e7330_d_n4, assign12980_e7330_d_n5, assign12980_e7330_d_n6, assign12980_e7330_d_n7, assign12980_e7330_d_n8, assign12980_e7330_d_n9, assign12980_e7330_d_n10, assign12980_e7330_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    } else {
        (locals.var_ttemp0, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn13,)
    }
};
        locals.var_ttemp0 = assign12980_e7330;
        locals.var_ttemp0_dn0 = assign12980_e7330_d_n0;
        locals.var_ttemp0_dn2 = assign12980_e7330_d_n2;
        locals.var_ttemp0_dn4 = assign12980_e7330_d_n4;
        locals.var_ttemp0_dn5 = assign12980_e7330_d_n5;
        locals.var_ttemp0_dn6 = assign12980_e7330_d_n6;
        locals.var_ttemp0_dn7 = assign12980_e7330_d_n7;
        locals.var_ttemp0_dn8 = assign12980_e7330_d_n8;
        locals.var_ttemp0_dn9 = assign12980_e7330_d_n9;
        locals.var_ttemp0_dn10 = assign12980_e7330_d_n10;
        locals.var_ttemp0_dn13 = assign12980_e7330_d_n13;
        locals.var_ttemp0_rv = 0.0;

        let (assign12990_e7336, assign12990_e7336_d_n0, assign12990_e7336_d_n2, assign12990_e7336_d_n4, assign12990_e7336_d_n5, assign12990_e7336_d_n6, assign12990_e7336_d_n7, assign12990_e7336_d_n8, assign12990_e7336_d_n9, assign12990_e7336_d_n10, assign12990_e7336_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign12990_e7334: f64 = (locals.var_ttemp + locals.var_deltemp);
        (assign12990_e7334, (locals.var_ttemp_dn0 + locals.var_deltemp_dn0), (locals.var_ttemp_dn2 + locals.var_deltemp_dn2), (locals.var_ttemp_dn4 + locals.var_deltemp_dn4), (locals.var_ttemp_dn5 + locals.var_deltemp_dn5), (locals.var_ttemp_dn6 + locals.var_deltemp_dn6), (locals.var_ttemp_dn7 + locals.var_deltemp_dn7), (locals.var_ttemp_dn8 + locals.var_deltemp_dn8), (locals.var_ttemp_dn9 + locals.var_deltemp_dn9), (locals.var_ttemp_dn10 + locals.var_deltemp_dn10), (locals.var_ttemp_dn13 + locals.var_deltemp_dn13),)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign12990_e7336;
        locals.var_ttemp_dn0 = assign12990_e7336_d_n0;
        locals.var_ttemp_dn2 = assign12990_e7336_d_n2;
        locals.var_ttemp_dn4 = assign12990_e7336_d_n4;
        locals.var_ttemp_dn5 = assign12990_e7336_d_n5;
        locals.var_ttemp_dn6 = assign12990_e7336_d_n6;
        locals.var_ttemp_dn7 = assign12990_e7336_d_n7;
        locals.var_ttemp_dn8 = assign12990_e7336_d_n8;
        locals.var_ttemp_dn9 = assign12990_e7336_d_n9;
        locals.var_ttemp_dn10 = assign12990_e7336_d_n10;
        locals.var_ttemp_dn13 = assign12990_e7336_d_n13;
        locals.var_ttemp_rv = 0.0;

        let (assign13000_e7342, assign13000_e7342_d_n0, assign13000_e7342_d_n2, assign13000_e7342_d_n4, assign13000_e7342_d_n5, assign13000_e7342_d_n6, assign13000_e7342_d_n7, assign13000_e7342_d_n8, assign13000_e7342_d_n9, assign13000_e7342_d_n10, assign13000_e7342_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13000_e7340: f64 = (locals.var_ttemp0 - locals.var_ktnom);
        (assign13000_e7340, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn13,)
    } else {
        (locals.var_tdiff0, locals.var_tdiff0_dn0, locals.var_tdiff0_dn2, locals.var_tdiff0_dn4, locals.var_tdiff0_dn5, locals.var_tdiff0_dn6, locals.var_tdiff0_dn7, locals.var_tdiff0_dn8, locals.var_tdiff0_dn9, locals.var_tdiff0_dn10, locals.var_tdiff0_dn13,)
    }
};
        locals.var_tdiff0 = assign13000_e7342;
        locals.var_tdiff0_dn0 = assign13000_e7342_d_n0;
        locals.var_tdiff0_dn2 = assign13000_e7342_d_n2;
        locals.var_tdiff0_dn4 = assign13000_e7342_d_n4;
        locals.var_tdiff0_dn5 = assign13000_e7342_d_n5;
        locals.var_tdiff0_dn6 = assign13000_e7342_d_n6;
        locals.var_tdiff0_dn7 = assign13000_e7342_d_n7;
        locals.var_tdiff0_dn8 = assign13000_e7342_d_n8;
        locals.var_tdiff0_dn9 = assign13000_e7342_d_n9;
        locals.var_tdiff0_dn10 = assign13000_e7342_d_n10;
        locals.var_tdiff0_dn13 = assign13000_e7342_d_n13;
        locals.var_tdiff0_rv = 0.0;

        let (assign13010_e7352, assign13010_e7352_d_n0, assign13010_e7352_d_n2, assign13010_e7352_d_n4, assign13010_e7352_d_n5, assign13010_e7352_d_n6, assign13010_e7352_d_n7, assign13010_e7352_d_n8, assign13010_e7352_d_n9, assign13010_e7352_d_n10, assign13010_e7352_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13010_e7346: f64 = (locals.var_ttemp0 * locals.var_ttemp0);
        let assign13010_e7349: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign13010_e7350: f64 = (assign13010_e7346 - assign13010_e7349);
        (assign13010_e7350, ((locals.var_ttemp0_dn0 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn0)), ((locals.var_ttemp0_dn2 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn2)), ((locals.var_ttemp0_dn4 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn4)), ((locals.var_ttemp0_dn5 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn5)), ((locals.var_ttemp0_dn6 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn6)), ((locals.var_ttemp0_dn7 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn7)), ((locals.var_ttemp0_dn8 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn8)), ((locals.var_ttemp0_dn9 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn9)), ((locals.var_ttemp0_dn10 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn10)), ((locals.var_ttemp0_dn13 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn13)),)
    } else {
        (locals.var_tdiff0_2, locals.var_tdiff0_2_dn0, locals.var_tdiff0_2_dn2, locals.var_tdiff0_2_dn4, locals.var_tdiff0_2_dn5, locals.var_tdiff0_2_dn6, locals.var_tdiff0_2_dn7, locals.var_tdiff0_2_dn8, locals.var_tdiff0_2_dn9, locals.var_tdiff0_2_dn10, locals.var_tdiff0_2_dn13,)
    }
};
        locals.var_tdiff0_2 = assign13010_e7352;
        locals.var_tdiff0_2_dn0 = assign13010_e7352_d_n0;
        locals.var_tdiff0_2_dn2 = assign13010_e7352_d_n2;
        locals.var_tdiff0_2_dn4 = assign13010_e7352_d_n4;
        locals.var_tdiff0_2_dn5 = assign13010_e7352_d_n5;
        locals.var_tdiff0_2_dn6 = assign13010_e7352_d_n6;
        locals.var_tdiff0_2_dn7 = assign13010_e7352_d_n7;
        locals.var_tdiff0_2_dn8 = assign13010_e7352_d_n8;
        locals.var_tdiff0_2_dn9 = assign13010_e7352_d_n9;
        locals.var_tdiff0_2_dn10 = assign13010_e7352_d_n10;
        locals.var_tdiff0_2_dn13 = assign13010_e7352_d_n13;
        locals.var_tdiff0_2_rv = 0.0;

        let (assign13020_e7358, assign13020_e7358_d_n0, assign13020_e7358_d_n2, assign13020_e7358_d_n4, assign13020_e7358_d_n5, assign13020_e7358_d_n6, assign13020_e7358_d_n7, assign13020_e7358_d_n8, assign13020_e7358_d_n9, assign13020_e7358_d_n10, assign13020_e7358_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13020_e7356: f64 = (locals.var_ttemp - locals.var_ktnom);
        (assign13020_e7356, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    } else {
        (locals.var_tdiff, locals.var_tdiff_dn0, locals.var_tdiff_dn2, locals.var_tdiff_dn4, locals.var_tdiff_dn5, locals.var_tdiff_dn6, locals.var_tdiff_dn7, locals.var_tdiff_dn8, locals.var_tdiff_dn9, locals.var_tdiff_dn10, locals.var_tdiff_dn13,)
    }
};
        locals.var_tdiff = assign13020_e7358;
        locals.var_tdiff_dn0 = assign13020_e7358_d_n0;
        locals.var_tdiff_dn2 = assign13020_e7358_d_n2;
        locals.var_tdiff_dn4 = assign13020_e7358_d_n4;
        locals.var_tdiff_dn5 = assign13020_e7358_d_n5;
        locals.var_tdiff_dn6 = assign13020_e7358_d_n6;
        locals.var_tdiff_dn7 = assign13020_e7358_d_n7;
        locals.var_tdiff_dn8 = assign13020_e7358_d_n8;
        locals.var_tdiff_dn9 = assign13020_e7358_d_n9;
        locals.var_tdiff_dn10 = assign13020_e7358_d_n10;
        locals.var_tdiff_dn13 = assign13020_e7358_d_n13;
        locals.var_tdiff_rv = 0.0;

        let (assign13030_e7368, assign13030_e7368_d_n0, assign13030_e7368_d_n2, assign13030_e7368_d_n4, assign13030_e7368_d_n5, assign13030_e7368_d_n6, assign13030_e7368_d_n7, assign13030_e7368_d_n8, assign13030_e7368_d_n9, assign13030_e7368_d_n10, assign13030_e7368_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13030_e7362: f64 = (locals.var_ttemp * locals.var_ttemp);
        let assign13030_e7365: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign13030_e7366: f64 = (assign13030_e7362 - assign13030_e7365);
        (assign13030_e7366, ((locals.var_ttemp_dn0 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn0)), ((locals.var_ttemp_dn2 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn2)), ((locals.var_ttemp_dn4 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn4)), ((locals.var_ttemp_dn5 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn5)), ((locals.var_ttemp_dn6 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn6)), ((locals.var_ttemp_dn7 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn7)), ((locals.var_ttemp_dn8 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn8)), ((locals.var_ttemp_dn9 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn9)), ((locals.var_ttemp_dn10 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn10)), ((locals.var_ttemp_dn13 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn13)),)
    } else {
        (locals.var_tdiff_2, locals.var_tdiff_2_dn0, locals.var_tdiff_2_dn2, locals.var_tdiff_2_dn4, locals.var_tdiff_2_dn5, locals.var_tdiff_2_dn6, locals.var_tdiff_2_dn7, locals.var_tdiff_2_dn8, locals.var_tdiff_2_dn9, locals.var_tdiff_2_dn10, locals.var_tdiff_2_dn13,)
    }
};
        locals.var_tdiff_2 = assign13030_e7368;
        locals.var_tdiff_2_dn0 = assign13030_e7368_d_n0;
        locals.var_tdiff_2_dn2 = assign13030_e7368_d_n2;
        locals.var_tdiff_2_dn4 = assign13030_e7368_d_n4;
        locals.var_tdiff_2_dn5 = assign13030_e7368_d_n5;
        locals.var_tdiff_2_dn6 = assign13030_e7368_d_n6;
        locals.var_tdiff_2_dn7 = assign13030_e7368_d_n7;
        locals.var_tdiff_2_dn8 = assign13030_e7368_d_n8;
        locals.var_tdiff_2_dn9 = assign13030_e7368_d_n9;
        locals.var_tdiff_2_dn10 = assign13030_e7368_d_n10;
        locals.var_tdiff_2_dn13 = assign13030_e7368_d_n13;
        locals.var_tdiff_2_rv = 0.0;

        let (assign13040_e7374, assign13040_e7374_d_n0, assign13040_e7374_d_n2, assign13040_e7374_d_n4, assign13040_e7374_d_n5, assign13040_e7374_d_n6, assign13040_e7374_d_n7, assign13040_e7374_d_n8, assign13040_e7374_d_n9, assign13040_e7374_d_n10, assign13040_e7374_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13040_e7372: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign13040_e7372, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn13 / locals.var_ktnom),)
    } else {
        (locals.var_tratio, locals.var_tratio_dn0, locals.var_tratio_dn2, locals.var_tratio_dn4, locals.var_tratio_dn5, locals.var_tratio_dn6, locals.var_tratio_dn7, locals.var_tratio_dn8, locals.var_tratio_dn9, locals.var_tratio_dn10, locals.var_tratio_dn13,)
    }
};
        locals.var_tratio = assign13040_e7374;
        locals.var_tratio_dn0 = assign13040_e7374_d_n0;
        locals.var_tratio_dn2 = assign13040_e7374_d_n2;
        locals.var_tratio_dn4 = assign13040_e7374_d_n4;
        locals.var_tratio_dn5 = assign13040_e7374_d_n5;
        locals.var_tratio_dn6 = assign13040_e7374_d_n6;
        locals.var_tratio_dn7 = assign13040_e7374_d_n7;
        locals.var_tratio_dn8 = assign13040_e7374_d_n8;
        locals.var_tratio_dn9 = assign13040_e7374_d_n9;
        locals.var_tratio_dn10 = assign13040_e7374_d_n10;
        locals.var_tratio_dn13 = assign13040_e7374_d_n13;
        locals.var_tratio_rv = 0.0;

        let (assign13050_e7379, assign13050_e7379_d_n0, assign13050_e7379_d_n2, assign13050_e7379_d_n4, assign13050_e7379_d_n5, assign13050_e7379_d_n6, assign13050_e7379_d_n7, assign13050_e7379_d_n8, assign13050_e7379_d_n9, assign13050_e7379_d_n10, assign13050_e7379_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13050_e7377: f64 = (locals.var_tratio).ln();
        (assign13050_e7377, (locals.var_tratio_dn0 / locals.var_tratio), (locals.var_tratio_dn2 / locals.var_tratio), (locals.var_tratio_dn4 / locals.var_tratio), (locals.var_tratio_dn5 / locals.var_tratio), (locals.var_tratio_dn6 / locals.var_tratio), (locals.var_tratio_dn7 / locals.var_tratio), (locals.var_tratio_dn8 / locals.var_tratio), (locals.var_tratio_dn9 / locals.var_tratio), (locals.var_tratio_dn10 / locals.var_tratio), (locals.var_tratio_dn13 / locals.var_tratio),)
    } else {
        (locals.var_log_tratio, locals.var_log_tratio_dn0, locals.var_log_tratio_dn2, locals.var_log_tratio_dn4, locals.var_log_tratio_dn5, locals.var_log_tratio_dn6, locals.var_log_tratio_dn7, locals.var_log_tratio_dn8, locals.var_log_tratio_dn9, locals.var_log_tratio_dn10, locals.var_log_tratio_dn13,)
    }
};
        locals.var_log_tratio = assign13050_e7379;
        locals.var_log_tratio_dn0 = assign13050_e7379_d_n0;
        locals.var_log_tratio_dn2 = assign13050_e7379_d_n2;
        locals.var_log_tratio_dn4 = assign13050_e7379_d_n4;
        locals.var_log_tratio_dn5 = assign13050_e7379_d_n5;
        locals.var_log_tratio_dn6 = assign13050_e7379_d_n6;
        locals.var_log_tratio_dn7 = assign13050_e7379_d_n7;
        locals.var_log_tratio_dn8 = assign13050_e7379_d_n8;
        locals.var_log_tratio_dn9 = assign13050_e7379_d_n9;
        locals.var_log_tratio_dn10 = assign13050_e7379_d_n10;
        locals.var_log_tratio_dn13 = assign13050_e7379_d_n13;
        locals.var_log_tratio_rv = 0.0;

        let (assign13060_e7391, assign13060_e7391_d_n0, assign13060_e7391_d_n2, assign13060_e7391_d_n4, assign13060_e7391_d_n5, assign13060_e7391_d_n6, assign13060_e7391_d_n7, assign13060_e7391_d_n8, assign13060_e7391_d_n9, assign13060_e7391_d_n10, assign13060_e7391_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13060_e7384: f64 = (locals.var_uc_bgtmp1 * locals.var_tdiff);
        let assign13060_e7385: f64 = (locals.var_egtnom - assign13060_e7384);
        let assign13060_e7388: f64 = (locals.var_uc_bgtmp2 * locals.var_tdiff_2);
        let assign13060_e7389: f64 = (assign13060_e7385 - assign13060_e7388);
        (assign13060_e7389, ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn0)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn0)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn2)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn2)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn4)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn4)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn5)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn5)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn6)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn6)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn7)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn7)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn8)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn8)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn9)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn9)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn10)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn10)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn13)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn13)),)
    } else {
        (locals.var_eg, locals.var_eg_dn0, locals.var_eg_dn2, locals.var_eg_dn4, locals.var_eg_dn5, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9, locals.var_eg_dn10, locals.var_eg_dn13,)
    }
};
        locals.var_eg = assign13060_e7391;
        locals.var_eg_dn0 = assign13060_e7391_d_n0;
        locals.var_eg_dn2 = assign13060_e7391_d_n2;
        locals.var_eg_dn4 = assign13060_e7391_d_n4;
        locals.var_eg_dn5 = assign13060_e7391_d_n5;
        locals.var_eg_dn6 = assign13060_e7391_d_n6;
        locals.var_eg_dn7 = assign13060_e7391_d_n7;
        locals.var_eg_dn8 = assign13060_e7391_d_n8;
        locals.var_eg_dn9 = assign13060_e7391_d_n9;
        locals.var_eg_dn10 = assign13060_e7391_d_n10;
        locals.var_eg_dn13 = assign13060_e7391_d_n13;
        locals.var_eg_rv = 0.0;

        let (assign13070_e7396, assign13070_e7396_d_n0, assign13070_e7396_d_n2, assign13070_e7396_d_n4, assign13070_e7396_d_n5, assign13070_e7396_d_n6, assign13070_e7396_d_n7, assign13070_e7396_d_n8, assign13070_e7396_d_n9, assign13070_e7396_d_n10, assign13070_e7396_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13070_e7394: f64 = (locals.var_eg).sqrt();
        (assign13070_e7394, (locals.var_eg_dn0 / (2.0 * assign13070_e7394)), (locals.var_eg_dn2 / (2.0 * assign13070_e7394)), (locals.var_eg_dn4 / (2.0 * assign13070_e7394)), (locals.var_eg_dn5 / (2.0 * assign13070_e7394)), (locals.var_eg_dn6 / (2.0 * assign13070_e7394)), (locals.var_eg_dn7 / (2.0 * assign13070_e7394)), (locals.var_eg_dn8 / (2.0 * assign13070_e7394)), (locals.var_eg_dn9 / (2.0 * assign13070_e7394)), (locals.var_eg_dn10 / (2.0 * assign13070_e7394)), (locals.var_eg_dn13 / (2.0 * assign13070_e7394)),)
    } else {
        (locals.var_sqrt_eg, locals.var_sqrt_eg_dn0, locals.var_sqrt_eg_dn2, locals.var_sqrt_eg_dn4, locals.var_sqrt_eg_dn5, locals.var_sqrt_eg_dn6, locals.var_sqrt_eg_dn7, locals.var_sqrt_eg_dn8, locals.var_sqrt_eg_dn9, locals.var_sqrt_eg_dn10, locals.var_sqrt_eg_dn13,)
    }
};
        locals.var_sqrt_eg = assign13070_e7396;
        locals.var_sqrt_eg_dn0 = assign13070_e7396_d_n0;
        locals.var_sqrt_eg_dn2 = assign13070_e7396_d_n2;
        locals.var_sqrt_eg_dn4 = assign13070_e7396_d_n4;
        locals.var_sqrt_eg_dn5 = assign13070_e7396_d_n5;
        locals.var_sqrt_eg_dn6 = assign13070_e7396_d_n6;
        locals.var_sqrt_eg_dn7 = assign13070_e7396_d_n7;
        locals.var_sqrt_eg_dn8 = assign13070_e7396_d_n8;
        locals.var_sqrt_eg_dn9 = assign13070_e7396_d_n9;
        locals.var_sqrt_eg_dn10 = assign13070_e7396_d_n10;
        locals.var_sqrt_eg_dn13 = assign13070_e7396_d_n13;
        locals.var_sqrt_eg_rv = 0.0;

        let (assign13080_e7402, assign13080_e7402_d_n0, assign13080_e7402_d_n2, assign13080_e7402_d_n4, assign13080_e7402_d_n5, assign13080_e7402_d_n6, assign13080_e7402_d_n7, assign13080_e7402_d_n8, assign13080_e7402_d_n9, assign13080_e7402_d_n10, assign13080_e7402_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13080_e7400: f64 = (1.0 / locals.var_ttemp);
        (assign13080_e7400, (-(locals.var_ttemp_dn0 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn2 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn4 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn5 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn6 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn7 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn8 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn9 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn10 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn13 / (locals.var_ttemp * locals.var_ttemp))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13080_e7402;
        locals.var_t1_dn0 = assign13080_e7402_d_n0;
        locals.var_t1_dn2 = assign13080_e7402_d_n2;
        locals.var_t1_dn4 = assign13080_e7402_d_n4;
        locals.var_t1_dn5 = assign13080_e7402_d_n5;
        locals.var_t1_dn6 = assign13080_e7402_d_n6;
        locals.var_t1_dn7 = assign13080_e7402_d_n7;
        locals.var_t1_dn8 = assign13080_e7402_d_n8;
        locals.var_t1_dn9 = assign13080_e7402_d_n9;
        locals.var_t1_dn10 = assign13080_e7402_d_n10;
        locals.var_t1_dn13 = assign13080_e7402_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign13090_e7408, assign13090_e7408_d_n0, assign13090_e7408_d_n2, assign13090_e7408_d_n4, assign13090_e7408_d_n5, assign13090_e7408_d_n6, assign13090_e7408_d_n7, assign13090_e7408_d_n8, assign13090_e7408_d_n9, assign13090_e7408_d_n10, assign13090_e7408_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13090_e7406: f64 = (1.0 / locals.var_ktnom);
        (assign13090_e7406, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign13090_e7408;
        locals.var_t2_dn0 = assign13090_e7408_d_n0;
        locals.var_t2_dn2 = assign13090_e7408_d_n2;
        locals.var_t2_dn4 = assign13090_e7408_d_n4;
        locals.var_t2_dn5 = assign13090_e7408_d_n5;
        locals.var_t2_dn6 = assign13090_e7408_d_n6;
        locals.var_t2_dn7 = assign13090_e7408_d_n7;
        locals.var_t2_dn8 = assign13090_e7408_d_n8;
        locals.var_t2_dn9 = assign13090_e7408_d_n9;
        locals.var_t2_dn10 = assign13090_e7408_d_n10;
        locals.var_t2_dn13 = assign13090_e7408_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign13100_e7430, assign13100_e7430_d_n0, assign13100_e7430_d_n2, assign13100_e7430_d_n4, assign13100_e7430_d_n5, assign13100_e7430_d_n6, assign13100_e7430_d_n7, assign13100_e7430_d_n8, assign13100_e7430_d_n9, assign13100_e7430_d_n10, assign13100_e7430_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13100_e7412: f64 = (locals.var_egtnom + p.p259);
        let assign13100_e7416: f64 = (locals.var_t1 - locals.var_t2);
        let assign13100_e7417: f64 = (p.p260 * assign13100_e7416);
        let assign13100_e7418: f64 = (assign13100_e7412 + assign13100_e7417);
        let assign13100_e7422: f64 = (locals.var_t1 * locals.var_t1);
        let assign13100_e7425: f64 = (locals.var_t2 * locals.var_t2);
        let assign13100_e7426: f64 = (assign13100_e7422 - assign13100_e7425);
        let assign13100_e7427: f64 = (p.p261 * assign13100_e7426);
        let assign13100_e7428: f64 = (assign13100_e7418 + assign13100_e7427);
        (assign13100_e7428, ((p.p260 * (locals.var_t1_dn0 - locals.var_t2_dn0)) + (p.p261 * (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) - ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))))), ((p.p260 * (locals.var_t1_dn2 - locals.var_t2_dn2)) + (p.p261 * (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) - ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))))), ((p.p260 * (locals.var_t1_dn4 - locals.var_t2_dn4)) + (p.p261 * (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) - ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))))), ((p.p260 * (locals.var_t1_dn5 - locals.var_t2_dn5)) + (p.p261 * (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) - ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))))), ((p.p260 * (locals.var_t1_dn6 - locals.var_t2_dn6)) + (p.p261 * (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) - ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))))), ((p.p260 * (locals.var_t1_dn7 - locals.var_t2_dn7)) + (p.p261 * (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) - ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))))), ((p.p260 * (locals.var_t1_dn8 - locals.var_t2_dn8)) + (p.p261 * (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) - ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))))), ((p.p260 * (locals.var_t1_dn9 - locals.var_t2_dn9)) + (p.p261 * (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) - ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))))), ((p.p260 * (locals.var_t1_dn10 - locals.var_t2_dn10)) + (p.p261 * (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) - ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))))), ((p.p260 * (locals.var_t1_dn13 - locals.var_t2_dn13)) + (p.p261 * (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) - ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign13100_e7430;
        locals.var_t3_dn0 = assign13100_e7430_d_n0;
        locals.var_t3_dn2 = assign13100_e7430_d_n2;
        locals.var_t3_dn4 = assign13100_e7430_d_n4;
        locals.var_t3_dn5 = assign13100_e7430_d_n5;
        locals.var_t3_dn6 = assign13100_e7430_d_n6;
        locals.var_t3_dn7 = assign13100_e7430_d_n7;
        locals.var_t3_dn8 = assign13100_e7430_d_n8;
        locals.var_t3_dn9 = assign13100_e7430_d_n9;
        locals.var_t3_dn10 = assign13100_e7430_d_n10;
        locals.var_t3_dn13 = assign13100_e7430_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign13110_e7435, assign13110_e7435_d_n0, assign13110_e7435_d_n2, assign13110_e7435_d_n4, assign13110_e7435_d_n5, assign13110_e7435_d_n6, assign13110_e7435_d_n7, assign13110_e7435_d_n8, assign13110_e7435_d_n9, assign13110_e7435_d_n10, assign13110_e7435_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13110_e7433: f64 = (locals.var_t3).sqrt();
        (assign13110_e7433, (locals.var_t3_dn0 / (2.0 * assign13110_e7433)), (locals.var_t3_dn2 / (2.0 * assign13110_e7433)), (locals.var_t3_dn4 / (2.0 * assign13110_e7433)), (locals.var_t3_dn5 / (2.0 * assign13110_e7433)), (locals.var_t3_dn6 / (2.0 * assign13110_e7433)), (locals.var_t3_dn7 / (2.0 * assign13110_e7433)), (locals.var_t3_dn8 / (2.0 * assign13110_e7433)), (locals.var_t3_dn9 / (2.0 * assign13110_e7433)), (locals.var_t3_dn10 / (2.0 * assign13110_e7433)), (locals.var_t3_dn13 / (2.0 * assign13110_e7433)),)
    } else {
        (locals.var_egp12, locals.var_egp12_dn0, locals.var_egp12_dn2, locals.var_egp12_dn4, locals.var_egp12_dn5, locals.var_egp12_dn6, locals.var_egp12_dn7, locals.var_egp12_dn8, locals.var_egp12_dn9, locals.var_egp12_dn10, locals.var_egp12_dn13,)
    }
};
        locals.var_egp12 = assign13110_e7435;
        locals.var_egp12_dn0 = assign13110_e7435_d_n0;
        locals.var_egp12_dn2 = assign13110_e7435_d_n2;
        locals.var_egp12_dn4 = assign13110_e7435_d_n4;
        locals.var_egp12_dn5 = assign13110_e7435_d_n5;
        locals.var_egp12_dn6 = assign13110_e7435_d_n6;
        locals.var_egp12_dn7 = assign13110_e7435_d_n7;
        locals.var_egp12_dn8 = assign13110_e7435_d_n8;
        locals.var_egp12_dn9 = assign13110_e7435_d_n9;
        locals.var_egp12_dn10 = assign13110_e7435_d_n10;
        locals.var_egp12_dn13 = assign13110_e7435_d_n13;
        locals.var_egp12_rv = 0.0;

        let (assign13120_e7441, assign13120_e7441_d_n0, assign13120_e7441_d_n2, assign13120_e7441_d_n4, assign13120_e7441_d_n5, assign13120_e7441_d_n6, assign13120_e7441_d_n7, assign13120_e7441_d_n8, assign13120_e7441_d_n9, assign13120_e7441_d_n10, assign13120_e7441_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13120_e7439: f64 = (locals.var_t3 * locals.var_egp12);
        (assign13120_e7439, ((locals.var_t3_dn0 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn0)), ((locals.var_t3_dn2 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn2)), ((locals.var_t3_dn4 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn4)), ((locals.var_t3_dn5 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn5)), ((locals.var_t3_dn6 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn6)), ((locals.var_t3_dn7 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn7)), ((locals.var_t3_dn8 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn8)), ((locals.var_t3_dn9 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn9)), ((locals.var_t3_dn10 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn10)), ((locals.var_t3_dn13 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn13)),)
    } else {
        (locals.var_egp32, locals.var_egp32_dn0, locals.var_egp32_dn2, locals.var_egp32_dn4, locals.var_egp32_dn5, locals.var_egp32_dn6, locals.var_egp32_dn7, locals.var_egp32_dn8, locals.var_egp32_dn9, locals.var_egp32_dn10, locals.var_egp32_dn13,)
    }
};
        locals.var_egp32 = assign13120_e7441;
        locals.var_egp32_dn0 = assign13120_e7441_d_n0;
        locals.var_egp32_dn2 = assign13120_e7441_d_n2;
        locals.var_egp32_dn4 = assign13120_e7441_d_n4;
        locals.var_egp32_dn5 = assign13120_e7441_d_n5;
        locals.var_egp32_dn6 = assign13120_e7441_d_n6;
        locals.var_egp32_dn7 = assign13120_e7441_d_n7;
        locals.var_egp32_dn8 = assign13120_e7441_d_n8;
        locals.var_egp32_dn9 = assign13120_e7441_d_n9;
        locals.var_egp32_dn10 = assign13120_e7441_d_n10;
        locals.var_egp32_dn13 = assign13120_e7441_d_n13;
        locals.var_egp32_rv = 0.0;

        let (assign13130_e7449, assign13130_e7449_d_n0, assign13130_e7449_d_n2, assign13130_e7449_d_n4, assign13130_e7449_d_n5, assign13130_e7449_d_n6, assign13130_e7449_d_n7, assign13130_e7449_d_n8, assign13130_e7449_d_n9, assign13130_e7449_d_n10, assign13130_e7449_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13130_e7446: f64 = (1.3806226e-23 * locals.var_ttemp);
        let assign13130_e7447: f64 = (1.6021918e-19 / assign13130_e7446);
        (assign13130_e7447, (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn0)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn2)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn4)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn5)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn6)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn7)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn8)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn9)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn10)) / (assign13130_e7446 * assign13130_e7446))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn13)) / (assign13130_e7446 * assign13130_e7446))),)
    } else {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn2, locals.var_beta_dn4, locals.var_beta_dn5, locals.var_beta_dn6, locals.var_beta_dn7, locals.var_beta_dn8, locals.var_beta_dn9, locals.var_beta_dn10, locals.var_beta_dn13,)
    }
};
        locals.var_beta = assign13130_e7449;
        locals.var_beta_dn0 = assign13130_e7449_d_n0;
        locals.var_beta_dn2 = assign13130_e7449_d_n2;
        locals.var_beta_dn4 = assign13130_e7449_d_n4;
        locals.var_beta_dn5 = assign13130_e7449_d_n5;
        locals.var_beta_dn6 = assign13130_e7449_d_n6;
        locals.var_beta_dn7 = assign13130_e7449_d_n7;
        locals.var_beta_dn8 = assign13130_e7449_d_n8;
        locals.var_beta_dn9 = assign13130_e7449_d_n9;
        locals.var_beta_dn10 = assign13130_e7449_d_n10;
        locals.var_beta_dn13 = assign13130_e7449_d_n13;
        locals.var_beta_rv = 0.0;

        let (assign13140_e7455, assign13140_e7455_d_n0, assign13140_e7455_d_n2, assign13140_e7455_d_n4, assign13140_e7455_d_n5, assign13140_e7455_d_n6, assign13140_e7455_d_n7, assign13140_e7455_d_n8, assign13140_e7455_d_n9, assign13140_e7455_d_n10, assign13140_e7455_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13140_e7453: f64 = (1.0 / locals.var_beta);
        (assign13140_e7453, (-(locals.var_beta_dn0 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn2 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn4 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn5 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn6 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn7 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn8 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn9 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn10 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn13 / (locals.var_beta * locals.var_beta))),)
    } else {
        (locals.var_beta_inv, locals.var_beta_inv_dn0, locals.var_beta_inv_dn2, locals.var_beta_inv_dn4, locals.var_beta_inv_dn5, locals.var_beta_inv_dn6, locals.var_beta_inv_dn7, locals.var_beta_inv_dn8, locals.var_beta_inv_dn9, locals.var_beta_inv_dn10, locals.var_beta_inv_dn13,)
    }
};
        locals.var_beta_inv = assign13140_e7455;
        locals.var_beta_inv_dn0 = assign13140_e7455_d_n0;
        locals.var_beta_inv_dn2 = assign13140_e7455_d_n2;
        locals.var_beta_inv_dn4 = assign13140_e7455_d_n4;
        locals.var_beta_inv_dn5 = assign13140_e7455_d_n5;
        locals.var_beta_inv_dn6 = assign13140_e7455_d_n6;
        locals.var_beta_inv_dn7 = assign13140_e7455_d_n7;
        locals.var_beta_inv_dn8 = assign13140_e7455_d_n8;
        locals.var_beta_inv_dn9 = assign13140_e7455_d_n9;
        locals.var_beta_inv_dn10 = assign13140_e7455_d_n10;
        locals.var_beta_inv_dn13 = assign13140_e7455_d_n13;
        locals.var_beta_inv_rv = 0.0;

        let (assign13150_e7461, assign13150_e7461_d_n0, assign13150_e7461_d_n2, assign13150_e7461_d_n4, assign13150_e7461_d_n5, assign13150_e7461_d_n6, assign13150_e7461_d_n7, assign13150_e7461_d_n8, assign13150_e7461_d_n9, assign13150_e7461_d_n10, assign13150_e7461_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13150_e7459: f64 = (locals.var_beta * locals.var_beta);
        (assign13150_e7459, ((locals.var_beta_dn0 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn0)), ((locals.var_beta_dn2 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn2)), ((locals.var_beta_dn4 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn4)), ((locals.var_beta_dn5 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn5)), ((locals.var_beta_dn6 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn6)), ((locals.var_beta_dn7 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn7)), ((locals.var_beta_dn8 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn8)), ((locals.var_beta_dn9 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn9)), ((locals.var_beta_dn10 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn10)), ((locals.var_beta_dn13 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn13)),)
    } else {
        (locals.var_beta2, locals.var_beta2_dn0, locals.var_beta2_dn2, locals.var_beta2_dn4, locals.var_beta2_dn5, locals.var_beta2_dn6, locals.var_beta2_dn7, locals.var_beta2_dn8, locals.var_beta2_dn9, locals.var_beta2_dn10, locals.var_beta2_dn13,)
    }
};
        locals.var_beta2 = assign13150_e7461;
        locals.var_beta2_dn0 = assign13150_e7461_d_n0;
        locals.var_beta2_dn2 = assign13150_e7461_d_n2;
        locals.var_beta2_dn4 = assign13150_e7461_d_n4;
        locals.var_beta2_dn5 = assign13150_e7461_d_n5;
        locals.var_beta2_dn6 = assign13150_e7461_d_n6;
        locals.var_beta2_dn7 = assign13150_e7461_d_n7;
        locals.var_beta2_dn8 = assign13150_e7461_d_n8;
        locals.var_beta2_dn9 = assign13150_e7461_d_n9;
        locals.var_beta2_dn10 = assign13150_e7461_d_n10;
        locals.var_beta2_dn13 = assign13150_e7461_d_n13;
        locals.var_beta2_rv = 0.0;

        let (assign13160_e7469,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13160_e7466: f64 = (1.3806226e-23 * locals.var_ktnom);
        let assign13160_e7467: f64 = (1.6021918e-19 / assign13160_e7466);
        (assign13160_e7467,)
    } else {
        (locals.var_betatnom,)
    }
};
        locals.var_betatnom = assign13160_e7469;
        locals.var_betatnom_rv = 0.0;

        let (assign13170_e7492, assign13170_e7492_d_n0, assign13170_e7492_d_n2, assign13170_e7492_d_n4, assign13170_e7492_d_n5, assign13170_e7492_d_n6, assign13170_e7492_d_n7, assign13170_e7492_d_n8, assign13170_e7492_d_n9, assign13170_e7492_d_n10, assign13170_e7492_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13170_e7474: f64 = (locals.var_log_tratio * 1.5);
        let assign13170_e7475: f64 = (assign13170_e7474).exp();
        let assign13170_e7476: f64 = (1.04e16 * assign13170_e7475);
        let assign13170_e7478: f64 = (-locals.var_eg);
        let assign13170_e7480: f64 = (assign13170_e7478 / 2.0);
        let assign13170_e7482: f64 = (assign13170_e7480 * locals.var_beta);
        let assign13170_e7485: f64 = (locals.var_egtnom / 2.0);
        let assign13170_e7487: f64 = (assign13170_e7485 * locals.var_betatnom);
        let assign13170_e7488: f64 = (assign13170_e7482 + assign13170_e7487);
        let assign13170_e7489: f64 = (assign13170_e7488).exp();
        let assign13170_e7490: f64 = (assign13170_e7476 * assign13170_e7489);
        (assign13170_e7490, (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn0 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn0) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn0))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn2 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn2) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn2))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn4 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn4) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn4))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn5 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn5) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn5))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn6 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn6) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn6))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn7 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn7) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn7))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn8 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn8) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn8))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn9 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn9) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn9))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn10 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn10) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn10))))), (((1.04e16 * (assign13170_e7475 * (locals.var_log_tratio_dn13 * 1.5))) * assign13170_e7489) + (assign13170_e7476 * (assign13170_e7489 * ((((-locals.var_eg_dn13) / 2.0) * locals.var_beta) + (assign13170_e7480 * locals.var_beta_dn13))))),)
    } else {
        (locals.var_nin, locals.var_nin_dn0, locals.var_nin_dn2, locals.var_nin_dn4, locals.var_nin_dn5, locals.var_nin_dn6, locals.var_nin_dn7, locals.var_nin_dn8, locals.var_nin_dn9, locals.var_nin_dn10, locals.var_nin_dn13,)
    }
};
        locals.var_nin = assign13170_e7492;
        locals.var_nin_dn0 = assign13170_e7492_d_n0;
        locals.var_nin_dn2 = assign13170_e7492_d_n2;
        locals.var_nin_dn4 = assign13170_e7492_d_n4;
        locals.var_nin_dn5 = assign13170_e7492_d_n5;
        locals.var_nin_dn6 = assign13170_e7492_d_n6;
        locals.var_nin_dn7 = assign13170_e7492_d_n7;
        locals.var_nin_dn8 = assign13170_e7492_d_n8;
        locals.var_nin_dn9 = assign13170_e7492_d_n9;
        locals.var_nin_dn10 = assign13170_e7492_d_n10;
        locals.var_nin_dn13 = assign13170_e7492_d_n13;
        locals.var_nin_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_23(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13180_e7499, assign13180_e7499_d_n0, assign13180_e7499_d_n2, assign13180_e7499_d_n4, assign13180_e7499_d_n5, assign13180_e7499_d_n6, assign13180_e7499_d_n7, assign13180_e7499_d_n8, assign13180_e7499_d_n9, assign13180_e7499_d_n10, assign13180_e7499_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13180_e7496: f64 = (locals.var_log_tratio * locals.var_uc_muetmp);
        let assign13180_e7497: f64 = (assign13180_e7496).exp();
        (assign13180_e7497, (assign13180_e7497 * (locals.var_log_tratio_dn0 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn2 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn4 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn5 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn6 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn7 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn8 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn9 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn10 * locals.var_uc_muetmp)), (assign13180_e7497 * (locals.var_log_tratio_dn13 * locals.var_uc_muetmp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13180_e7499;
        locals.var_t1_dn0 = assign13180_e7499_d_n0;
        locals.var_t1_dn2 = assign13180_e7499_d_n2;
        locals.var_t1_dn4 = assign13180_e7499_d_n4;
        locals.var_t1_dn5 = assign13180_e7499_d_n5;
        locals.var_t1_dn6 = assign13180_e7499_d_n6;
        locals.var_t1_dn7 = assign13180_e7499_d_n7;
        locals.var_t1_dn8 = assign13180_e7499_d_n8;
        locals.var_t1_dn9 = assign13180_e7499_d_n9;
        locals.var_t1_dn10 = assign13180_e7499_d_n10;
        locals.var_t1_dn13 = assign13180_e7499_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign13190_e7505, assign13190_e7505_d_n0, assign13190_e7505_d_n2, assign13190_e7505_d_n4, assign13190_e7505_d_n5, assign13190_e7505_d_n6, assign13190_e7505_d_n7, assign13190_e7505_d_n8, assign13190_e7505_d_n9, assign13190_e7505_d_n10, assign13190_e7505_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13190_e7503: f64 = (locals.var_t1 / locals.var_mueph);
        (assign13190_e7503, (((locals.var_t1_dn0 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn0)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn2 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn2)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn4 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn4)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn5 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn5)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn6 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn6)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn7 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn7)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn8 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn8)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn9 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn9)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn10 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn10)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn13 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn13)) / (locals.var_mueph * locals.var_mueph)),)
    } else {
        (locals.var_mphn0, locals.var_mphn0_dn0, locals.var_mphn0_dn2, locals.var_mphn0_dn4, locals.var_mphn0_dn5, locals.var_mphn0_dn6, locals.var_mphn0_dn7, locals.var_mphn0_dn8, locals.var_mphn0_dn9, locals.var_mphn0_dn10, locals.var_mphn0_dn13,)
    }
};
        locals.var_mphn0 = assign13190_e7505;
        locals.var_mphn0_dn0 = assign13190_e7505_d_n0;
        locals.var_mphn0_dn2 = assign13190_e7505_d_n2;
        locals.var_mphn0_dn4 = assign13190_e7505_d_n4;
        locals.var_mphn0_dn5 = assign13190_e7505_d_n5;
        locals.var_mphn0_dn6 = assign13190_e7505_d_n6;
        locals.var_mphn0_dn7 = assign13190_e7505_d_n7;
        locals.var_mphn0_dn8 = assign13190_e7505_d_n8;
        locals.var_mphn0_dn9 = assign13190_e7505_d_n9;
        locals.var_mphn0_dn10 = assign13190_e7505_d_n10;
        locals.var_mphn0_dn13 = assign13190_e7505_d_n13;
        locals.var_mphn0_rv = 0.0;

        let assign13200_e7512: f64 = if ((locals.var_uc_codep != 0.0) && (locals.var_uc_codep < 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard290 = assign13200_e7512;
        locals.var_guard290_rv = 0.0;

        let (assign13210_e7527, assign13210_e7527_d_n0, assign13210_e7527_d_n2, assign13210_e7527_d_n4, assign13210_e7527_d_n5, assign13210_e7527_d_n6, assign13210_e7527_d_n7, assign13210_e7527_d_n8, assign13210_e7527_d_n9, assign13210_e7527_d_n10, assign13210_e7527_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13210_e7518: f64 = (2.0 * 1.034943e-10);
        let assign13210_e7520: f64 = (assign13210_e7518 * 1.6021918e-19);
        let assign13210_e7522: f64 = (assign13210_e7520 * locals.var_uc_ndepm);
        let assign13210_e7524: f64 = (assign13210_e7522 * locals.var_beta_inv);
        let assign13210_e7525: f64 = (assign13210_e7524).sqrt();
        (assign13210_e7525, ((((assign13210_e7520 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn0)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn2)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn4)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn5)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn6)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn7)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn8)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn9)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn10)) / (2.0 * assign13210_e7525)), ((((assign13210_e7520 * locals.var_uc_ndepm_dn13) * locals.var_beta_inv) + (assign13210_e7522 * locals.var_beta_inv_dn13)) / (2.0 * assign13210_e7525)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn13,)
    }
};
        locals.var_cnst0 = assign13210_e7527;
        locals.var_cnst0_dn0 = assign13210_e7527_d_n0;
        locals.var_cnst0_dn2 = assign13210_e7527_d_n2;
        locals.var_cnst0_dn4 = assign13210_e7527_d_n4;
        locals.var_cnst0_dn5 = assign13210_e7527_d_n5;
        locals.var_cnst0_dn6 = assign13210_e7527_d_n6;
        locals.var_cnst0_dn7 = assign13210_e7527_d_n7;
        locals.var_cnst0_dn8 = assign13210_e7527_d_n8;
        locals.var_cnst0_dn9 = assign13210_e7527_d_n9;
        locals.var_cnst0_dn10 = assign13210_e7527_d_n10;
        locals.var_cnst0_dn13 = assign13210_e7527_d_n13;
        locals.var_cnst0_rv = 0.0;

        let (assign13220_e7539, assign13220_e7539_d_n0, assign13220_e7539_d_n2, assign13220_e7539_d_n4, assign13220_e7539_d_n5, assign13220_e7539_d_n6, assign13220_e7539_d_n7, assign13220_e7539_d_n8, assign13220_e7539_d_n9, assign13220_e7539_d_n10, assign13220_e7539_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13220_e7533: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_uc_ndepm;
        let assign13220_e7535: f64 = (assign13220_e7533 * __rspice_inv_cse_0);
        let assign13220_e7537: f64 = (assign13220_e7535 * __rspice_inv_cse_0);
        (assign13220_e7537, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn13 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn13)) * locals.var_uc_ndepm) - (assign13220_e7533 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13220_e7535 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn13,)
    }
};
        locals.var_cnst1 = assign13220_e7539;
        locals.var_cnst1_dn0 = assign13220_e7539_d_n0;
        locals.var_cnst1_dn2 = assign13220_e7539_d_n2;
        locals.var_cnst1_dn4 = assign13220_e7539_d_n4;
        locals.var_cnst1_dn5 = assign13220_e7539_d_n5;
        locals.var_cnst1_dn6 = assign13220_e7539_d_n6;
        locals.var_cnst1_dn7 = assign13220_e7539_d_n7;
        locals.var_cnst1_dn8 = assign13220_e7539_d_n8;
        locals.var_cnst1_dn9 = assign13220_e7539_d_n9;
        locals.var_cnst1_dn10 = assign13220_e7539_d_n10;
        locals.var_cnst1_dn13 = assign13220_e7539_d_n13;
        locals.var_cnst1_rv = 0.0;

        let (assign13230_e7552, assign13230_e7552_d_n0, assign13230_e7552_d_n2, assign13230_e7552_d_n4, assign13230_e7552_d_n5, assign13230_e7552_d_n6, assign13230_e7552_d_n7, assign13230_e7552_d_n8, assign13230_e7552_d_n9, assign13230_e7552_d_n10, assign13230_e7552_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13230_e7545: f64 = (2.0 * locals.var_beta_inv);
        let assign13230_e7548: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign13230_e7549: f64 = (assign13230_e7548).ln();
        let assign13230_e7550: f64 = (assign13230_e7545 * assign13230_e7549);
        (assign13230_e7550, (((2.0 * locals.var_beta_inv_dn0) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn2) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn4) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn5) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn6) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn7) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn8) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn9) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn10) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))), (((2.0 * locals.var_beta_inv_dn13) * assign13230_e7549) + (assign13230_e7545 * ((((locals.var_uc_ndepm_dn13 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign13230_e7548))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    }
};
        locals.var_pb2n = assign13230_e7552;
        locals.var_pb2n_dn0 = assign13230_e7552_d_n0;
        locals.var_pb2n_dn2 = assign13230_e7552_d_n2;
        locals.var_pb2n_dn4 = assign13230_e7552_d_n4;
        locals.var_pb2n_dn5 = assign13230_e7552_d_n5;
        locals.var_pb2n_dn6 = assign13230_e7552_d_n6;
        locals.var_pb2n_dn7 = assign13230_e7552_d_n7;
        locals.var_pb2n_dn8 = assign13230_e7552_d_n8;
        locals.var_pb2n_dn9 = assign13230_e7552_d_n9;
        locals.var_pb2n_dn10 = assign13230_e7552_d_n10;
        locals.var_pb2n_dn13 = assign13230_e7552_d_n13;
        locals.var_pb2n_rv = 0.0;

        let (assign13240_e7567, assign13240_e7567_d_n0, assign13240_e7567_d_n2, assign13240_e7567_d_n4, assign13240_e7567_d_n5, assign13240_e7567_d_n6, assign13240_e7567_d_n7, assign13240_e7567_d_n8, assign13240_e7567_d_n9, assign13240_e7567_d_n10, assign13240_e7567_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13240_e7559: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign13240_e7561: f64 = (assign13240_e7559 * __rspice_inv_cse_1);
        let assign13240_e7563: f64 = (assign13240_e7561 * __rspice_inv_cse_1);
        let assign13240_e7564: f64 = (assign13240_e7563).ln();
        let assign13240_e7565: f64 = (locals.var_beta_inv * assign13240_e7564);
        (assign13240_e7565, ((locals.var_beta_inv_dn0 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn2 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn4 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn5 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn6 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn7 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn8 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn9 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn10 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))), ((locals.var_beta_inv_dn13 * assign13240_e7564) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn13 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn13)) * locals.var_nin) - (assign13240_e7559 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13240_e7561 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign13240_e7563))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    }
};
        locals.var_vbipn = assign13240_e7567;
        locals.var_vbipn_dn0 = assign13240_e7567_d_n0;
        locals.var_vbipn_dn2 = assign13240_e7567_d_n2;
        locals.var_vbipn_dn4 = assign13240_e7567_d_n4;
        locals.var_vbipn_dn5 = assign13240_e7567_d_n5;
        locals.var_vbipn_dn6 = assign13240_e7567_d_n6;
        locals.var_vbipn_dn7 = assign13240_e7567_d_n7;
        locals.var_vbipn_dn8 = assign13240_e7567_d_n8;
        locals.var_vbipn_dn9 = assign13240_e7567_d_n9;
        locals.var_vbipn_dn10 = assign13240_e7567_d_n10;
        locals.var_vbipn_dn13 = assign13240_e7567_d_n13;
        locals.var_vbipn_rv = 0.0;

        let (assign13250_e7576, assign13250_e7576_d_n0, assign13250_e7576_d_n2, assign13250_e7576_d_n4, assign13250_e7576_d_n5, assign13250_e7576_d_n6, assign13250_e7576_d_n7, assign13250_e7576_d_n8, assign13250_e7576_d_n9, assign13250_e7576_d_n10, assign13250_e7576_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13250_e7573: f64 = (locals.var_log_tratio * p.p380);
        let assign13250_e7574: f64 = (assign13250_e7573).exp();
        (assign13250_e7574, (assign13250_e7574 * (locals.var_log_tratio_dn0 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn2 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn4 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn5 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn6 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn7 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn8 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn9 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn10 * p.p380)), (assign13250_e7574 * (locals.var_log_tratio_dn13 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13250_e7576;
        locals.var_t1_dn0 = assign13250_e7576_d_n0;
        locals.var_t1_dn2 = assign13250_e7576_d_n2;
        locals.var_t1_dn4 = assign13250_e7576_d_n4;
        locals.var_t1_dn5 = assign13250_e7576_d_n5;
        locals.var_t1_dn6 = assign13250_e7576_d_n6;
        locals.var_t1_dn7 = assign13250_e7576_d_n7;
        locals.var_t1_dn8 = assign13250_e7576_d_n8;
        locals.var_t1_dn9 = assign13250_e7576_d_n9;
        locals.var_t1_dn10 = assign13250_e7576_d_n10;
        locals.var_t1_dn13 = assign13250_e7576_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign13260_e7584, assign13260_e7584_d_n0, assign13260_e7584_d_n2, assign13260_e7584_d_n4, assign13260_e7584_d_n5, assign13260_e7584_d_n6, assign13260_e7584_d_n7, assign13260_e7584_d_n8, assign13260_e7584_d_n9, assign13260_e7584_d_n10, assign13260_e7584_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13260_e7582: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign13260_e7582, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn13 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn13,)
    }
};
        locals.var_depmphn0 = assign13260_e7584;
        locals.var_depmphn0_dn0 = assign13260_e7584_d_n0;
        locals.var_depmphn0_dn2 = assign13260_e7584_d_n2;
        locals.var_depmphn0_dn4 = assign13260_e7584_d_n4;
        locals.var_depmphn0_dn5 = assign13260_e7584_d_n5;
        locals.var_depmphn0_dn6 = assign13260_e7584_d_n6;
        locals.var_depmphn0_dn7 = assign13260_e7584_d_n7;
        locals.var_depmphn0_dn8 = assign13260_e7584_d_n8;
        locals.var_depmphn0_dn9 = assign13260_e7584_d_n9;
        locals.var_depmphn0_dn10 = assign13260_e7584_d_n10;
        locals.var_depmphn0_dn13 = assign13260_e7584_d_n13;
        locals.var_depmphn0_rv = 0.0;

        let (assign13270_e7606, assign13270_e7606_d_n0, assign13270_e7606_d_n2, assign13270_e7606_d_n4, assign13270_e7606_d_n5, assign13270_e7606_d_n6, assign13270_e7606_d_n7, assign13270_e7606_d_n8, assign13270_e7606_d_n9, assign13270_e7606_d_n10, assign13270_e7606_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13270_e7591: f64 = (0.4 * locals.var_tratio);
        let assign13270_e7592: f64 = (1.8 + assign13270_e7591);
        let assign13270_e7595: f64 = (0.1 * locals.var_tratio);
        let assign13270_e7597: f64 = (assign13270_e7595 * locals.var_tratio);
        let assign13270_e7598: f64 = (assign13270_e7592 + assign13270_e7597);
        let assign13270_e7602: f64 = (1.0 - locals.var_tratio);
        let assign13270_e7603: f64 = (p.p379 * assign13270_e7602);
        let assign13270_e7604: f64 = (assign13270_e7598 - assign13270_e7603);
        (assign13270_e7604, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign13270_e7595 * locals.var_tratio_dn13))) - (p.p379 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign13270_e7606;
        locals.var_t0_dn0 = assign13270_e7606_d_n0;
        locals.var_t0_dn2 = assign13270_e7606_d_n2;
        locals.var_t0_dn4 = assign13270_e7606_d_n4;
        locals.var_t0_dn5 = assign13270_e7606_d_n5;
        locals.var_t0_dn6 = assign13270_e7606_d_n6;
        locals.var_t0_dn7 = assign13270_e7606_d_n7;
        locals.var_t0_dn8 = assign13270_e7606_d_n8;
        locals.var_t0_dn9 = assign13270_e7606_d_n9;
        locals.var_t0_dn10 = assign13270_e7606_d_n10;
        locals.var_t0_dn13 = assign13270_e7606_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign13280_e7614, assign13280_e7614_d_n0, assign13280_e7614_d_n2, assign13280_e7614_d_n4, assign13280_e7614_d_n5, assign13280_e7614_d_n6, assign13280_e7614_d_n7, assign13280_e7614_d_n8, assign13280_e7614_d_n9, assign13280_e7614_d_n10, assign13280_e7614_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13280_e7612: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign13280_e7612, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn13 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign13280_e7614;
        locals.var_uc_depvmax_dn0 = assign13280_e7614_d_n0;
        locals.var_uc_depvmax_dn2 = assign13280_e7614_d_n2;
        locals.var_uc_depvmax_dn4 = assign13280_e7614_d_n4;
        locals.var_uc_depvmax_dn5 = assign13280_e7614_d_n5;
        locals.var_uc_depvmax_dn6 = assign13280_e7614_d_n6;
        locals.var_uc_depvmax_dn7 = assign13280_e7614_d_n7;
        locals.var_uc_depvmax_dn8 = assign13280_e7614_d_n8;
        locals.var_uc_depvmax_dn9 = assign13280_e7614_d_n9;
        locals.var_uc_depvmax_dn10 = assign13280_e7614_d_n10;
        locals.var_uc_depvmax_dn13 = assign13280_e7614_d_n13;
        locals.var_uc_depvmax_rv = 0.0;

        let assign13300_e7622: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard292 = assign13300_e7622;
        locals.var_guard292_rv = 0.0;

        let (assign13310_e7630, assign13310_e7630_d_n0, assign13310_e7630_d_n2, assign13310_e7630_d_n4, assign13310_e7630_d_n5, assign13310_e7630_d_n6, assign13310_e7630_d_n7, assign13310_e7630_d_n8, assign13310_e7630_d_n9, assign13310_e7630_d_n10, assign13310_e7630_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) && (locals.var_guard292 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign13310_e7630;
        locals.var_uc_depvmax_dn0 = assign13310_e7630_d_n0;
        locals.var_uc_depvmax_dn2 = assign13310_e7630_d_n2;
        locals.var_uc_depvmax_dn4 = assign13310_e7630_d_n4;
        locals.var_uc_depvmax_dn5 = assign13310_e7630_d_n5;
        locals.var_uc_depvmax_dn6 = assign13310_e7630_d_n6;
        locals.var_uc_depvmax_dn7 = assign13310_e7630_d_n7;
        locals.var_uc_depvmax_dn8 = assign13310_e7630_d_n8;
        locals.var_uc_depvmax_dn9 = assign13310_e7630_d_n9;
        locals.var_uc_depvmax_dn10 = assign13310_e7630_d_n10;
        locals.var_uc_depvmax_dn13 = assign13310_e7630_d_n13;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign13320_e7640, assign13320_e7640_d_n0, assign13320_e7640_d_n2, assign13320_e7640_d_n4, assign13320_e7640_d_n5, assign13320_e7640_d_n6, assign13320_e7640_d_n7, assign13320_e7640_d_n8, assign13320_e7640_d_n9, assign13320_e7640_d_n10, assign13320_e7640_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13320_e7637: f64 = (locals.var_tratio).powf(p.p381);
        let assign13320_e7638: f64 = (locals.var_uc_depmue0 / assign13320_e7637);
        (assign13320_e7638, (((locals.var_uc_depmue0_dn0 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn2 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn4 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn5 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn6 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn7 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn8 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn9 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn10 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)), (((locals.var_uc_depmue0_dn13 * assign13320_e7637) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn13)) } } else { (assign13320_e7637 * (p.p381 * (locals.var_tratio_dn13 / locals.var_tratio))) })) / (assign13320_e7637 * assign13320_e7637)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign13320_e7640;
        locals.var_uc_depmue0_dn0 = assign13320_e7640_d_n0;
        locals.var_uc_depmue0_dn2 = assign13320_e7640_d_n2;
        locals.var_uc_depmue0_dn4 = assign13320_e7640_d_n4;
        locals.var_uc_depmue0_dn5 = assign13320_e7640_d_n5;
        locals.var_uc_depmue0_dn6 = assign13320_e7640_d_n6;
        locals.var_uc_depmue0_dn7 = assign13320_e7640_d_n7;
        locals.var_uc_depmue0_dn8 = assign13320_e7640_d_n8;
        locals.var_uc_depmue0_dn9 = assign13320_e7640_d_n9;
        locals.var_uc_depmue0_dn10 = assign13320_e7640_d_n10;
        locals.var_uc_depmue0_dn13 = assign13320_e7640_d_n13;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign13330_e7650, assign13330_e7650_d_n0, assign13330_e7650_d_n2, assign13330_e7650_d_n4, assign13330_e7650_d_n5, assign13330_e7650_d_n6, assign13330_e7650_d_n7, assign13330_e7650_d_n8, assign13330_e7650_d_n9, assign13330_e7650_d_n10, assign13330_e7650_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard290 != 0.0)) {
        let assign13330_e7647: f64 = (locals.var_tratio).powf(p.p382);
        let assign13330_e7648: f64 = (locals.var_uc_depmue2 / assign13330_e7647);
        (assign13330_e7648, (((locals.var_uc_depmue2_dn0 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn0)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn2 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn2)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn4 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn5 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn6 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn6)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn7 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn7)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn8 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn8)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn9 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn9)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn10 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn10)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)), (((locals.var_uc_depmue2_dn13 * assign13330_e7647) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn13)) } } else { (assign13330_e7647 * (p.p382 * (locals.var_tratio_dn13 / locals.var_tratio))) })) / (assign13330_e7647 * assign13330_e7647)),)
    } else {
        (locals.var_uc_depmue2, locals.var_uc_depmue2_dn0, locals.var_uc_depmue2_dn2, locals.var_uc_depmue2_dn4, locals.var_uc_depmue2_dn5, locals.var_uc_depmue2_dn6, locals.var_uc_depmue2_dn7, locals.var_uc_depmue2_dn8, locals.var_uc_depmue2_dn9, locals.var_uc_depmue2_dn10, locals.var_uc_depmue2_dn13,)
    }
};
        locals.var_uc_depmue2 = assign13330_e7650;
        locals.var_uc_depmue2_dn0 = assign13330_e7650_d_n0;
        locals.var_uc_depmue2_dn2 = assign13330_e7650_d_n2;
        locals.var_uc_depmue2_dn4 = assign13330_e7650_d_n4;
        locals.var_uc_depmue2_dn5 = assign13330_e7650_d_n5;
        locals.var_uc_depmue2_dn6 = assign13330_e7650_d_n6;
        locals.var_uc_depmue2_dn7 = assign13330_e7650_d_n7;
        locals.var_uc_depmue2_dn8 = assign13330_e7650_d_n8;
        locals.var_uc_depmue2_dn9 = assign13330_e7650_d_n9;
        locals.var_uc_depmue2_dn10 = assign13330_e7650_d_n10;
        locals.var_uc_depmue2_dn13 = assign13330_e7650_d_n13;
        locals.var_uc_depmue2_rv = 0.0;

        let assign13340_e7653: f64 = if locals.var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard293 = assign13340_e7653;
        locals.var_guard293_rv = 0.0;

        let (assign13350_e7671, assign13350_e7671_d_n0, assign13350_e7671_d_n2, assign13350_e7671_d_n4, assign13350_e7671_d_n5, assign13350_e7671_d_n6, assign13350_e7671_d_n7, assign13350_e7671_d_n8, assign13350_e7671_d_n9, assign13350_e7671_d_n10, assign13350_e7671_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13350_e7662: f64 = (2.0 * 1.034943e-10);
        let assign13350_e7664: f64 = (assign13350_e7662 * 1.6021918e-19);
        let assign13350_e7666: f64 = (assign13350_e7664 * locals.var_uc_ndepm);
        let assign13350_e7668: f64 = (assign13350_e7666 * locals.var_beta_inv);
        let assign13350_e7669: f64 = (assign13350_e7668).sqrt();
        (assign13350_e7669, ((((assign13350_e7664 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn0)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn2)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn4)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn5)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn6)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn7)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn8)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn9)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn10)) / (2.0 * assign13350_e7669)), ((((assign13350_e7664 * locals.var_uc_ndepm_dn13) * locals.var_beta_inv) + (assign13350_e7666 * locals.var_beta_inv_dn13)) / (2.0 * assign13350_e7669)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn13,)
    }
};
        locals.var_cnst0 = assign13350_e7671;
        locals.var_cnst0_dn0 = assign13350_e7671_d_n0;
        locals.var_cnst0_dn2 = assign13350_e7671_d_n2;
        locals.var_cnst0_dn4 = assign13350_e7671_d_n4;
        locals.var_cnst0_dn5 = assign13350_e7671_d_n5;
        locals.var_cnst0_dn6 = assign13350_e7671_d_n6;
        locals.var_cnst0_dn7 = assign13350_e7671_d_n7;
        locals.var_cnst0_dn8 = assign13350_e7671_d_n8;
        locals.var_cnst0_dn9 = assign13350_e7671_d_n9;
        locals.var_cnst0_dn10 = assign13350_e7671_d_n10;
        locals.var_cnst0_dn13 = assign13350_e7671_d_n13;
        locals.var_cnst0_rv = 0.0;

        let (assign13360_e7686, assign13360_e7686_d_n0, assign13360_e7686_d_n2, assign13360_e7686_d_n4, assign13360_e7686_d_n5, assign13360_e7686_d_n6, assign13360_e7686_d_n7, assign13360_e7686_d_n8, assign13360_e7686_d_n9, assign13360_e7686_d_n10, assign13360_e7686_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13360_e7680: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_2: f64 = 1.0 / locals.var_uc_ndepm;
        let assign13360_e7682: f64 = (assign13360_e7680 * __rspice_inv_cse_2);
        let assign13360_e7684: f64 = (assign13360_e7682 * __rspice_inv_cse_2);
        (assign13360_e7684, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn13 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn13)) * locals.var_uc_ndepm) - (assign13360_e7680 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13360_e7682 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn13,)
    }
};
        locals.var_cnst1 = assign13360_e7686;
        locals.var_cnst1_dn0 = assign13360_e7686_d_n0;
        locals.var_cnst1_dn2 = assign13360_e7686_d_n2;
        locals.var_cnst1_dn4 = assign13360_e7686_d_n4;
        locals.var_cnst1_dn5 = assign13360_e7686_d_n5;
        locals.var_cnst1_dn6 = assign13360_e7686_d_n6;
        locals.var_cnst1_dn7 = assign13360_e7686_d_n7;
        locals.var_cnst1_dn8 = assign13360_e7686_d_n8;
        locals.var_cnst1_dn9 = assign13360_e7686_d_n9;
        locals.var_cnst1_dn10 = assign13360_e7686_d_n10;
        locals.var_cnst1_dn13 = assign13360_e7686_d_n13;
        locals.var_cnst1_rv = 0.0;

        let (assign13370_e7702, assign13370_e7702_d_n0, assign13370_e7702_d_n2, assign13370_e7702_d_n4, assign13370_e7702_d_n5, assign13370_e7702_d_n6, assign13370_e7702_d_n7, assign13370_e7702_d_n8, assign13370_e7702_d_n9, assign13370_e7702_d_n10, assign13370_e7702_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13370_e7695: f64 = (2.0 * locals.var_beta_inv);
        let assign13370_e7698: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign13370_e7699: f64 = (assign13370_e7698).ln();
        let assign13370_e7700: f64 = (assign13370_e7695 * assign13370_e7699);
        (assign13370_e7700, (((2.0 * locals.var_beta_inv_dn0) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn2) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn4) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn5) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn6) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn7) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn8) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn9) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn10) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))), (((2.0 * locals.var_beta_inv_dn13) * assign13370_e7699) + (assign13370_e7695 * ((((locals.var_uc_ndepm_dn13 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign13370_e7698))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    }
};
        locals.var_pb2n = assign13370_e7702;
        locals.var_pb2n_dn0 = assign13370_e7702_d_n0;
        locals.var_pb2n_dn2 = assign13370_e7702_d_n2;
        locals.var_pb2n_dn4 = assign13370_e7702_d_n4;
        locals.var_pb2n_dn5 = assign13370_e7702_d_n5;
        locals.var_pb2n_dn6 = assign13370_e7702_d_n6;
        locals.var_pb2n_dn7 = assign13370_e7702_d_n7;
        locals.var_pb2n_dn8 = assign13370_e7702_d_n8;
        locals.var_pb2n_dn9 = assign13370_e7702_d_n9;
        locals.var_pb2n_dn10 = assign13370_e7702_d_n10;
        locals.var_pb2n_dn13 = assign13370_e7702_d_n13;
        locals.var_pb2n_rv = 0.0;

        let (assign13380_e7720, assign13380_e7720_d_n0, assign13380_e7720_d_n2, assign13380_e7720_d_n4, assign13380_e7720_d_n5, assign13380_e7720_d_n6, assign13380_e7720_d_n7, assign13380_e7720_d_n8, assign13380_e7720_d_n9, assign13380_e7720_d_n10, assign13380_e7720_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13380_e7712: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_3: f64 = 1.0 / locals.var_nin;
        let assign13380_e7714: f64 = (assign13380_e7712 * __rspice_inv_cse_3);
        let assign13380_e7716: f64 = (assign13380_e7714 * __rspice_inv_cse_3);
        let assign13380_e7717: f64 = (assign13380_e7716).ln();
        let assign13380_e7718: f64 = (locals.var_beta_inv * assign13380_e7717);
        (assign13380_e7718, ((locals.var_beta_inv_dn0 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn2 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn4 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn5 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn6 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn7 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn8 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn9 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn10 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))), ((locals.var_beta_inv_dn13 * assign13380_e7717) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn13 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn13)) * locals.var_nin) - (assign13380_e7712 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13380_e7714 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign13380_e7716))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    }
};
        locals.var_vbipn = assign13380_e7720;
        locals.var_vbipn_dn0 = assign13380_e7720_d_n0;
        locals.var_vbipn_dn2 = assign13380_e7720_d_n2;
        locals.var_vbipn_dn4 = assign13380_e7720_d_n4;
        locals.var_vbipn_dn5 = assign13380_e7720_d_n5;
        locals.var_vbipn_dn6 = assign13380_e7720_d_n6;
        locals.var_vbipn_dn7 = assign13380_e7720_d_n7;
        locals.var_vbipn_dn8 = assign13380_e7720_d_n8;
        locals.var_vbipn_dn9 = assign13380_e7720_d_n9;
        locals.var_vbipn_dn10 = assign13380_e7720_d_n10;
        locals.var_vbipn_dn13 = assign13380_e7720_d_n13;
        locals.var_vbipn_rv = 0.0;

        let (assign13390_e7732, assign13390_e7732_d_n0, assign13390_e7732_d_n2, assign13390_e7732_d_n4, assign13390_e7732_d_n5, assign13390_e7732_d_n6, assign13390_e7732_d_n7, assign13390_e7732_d_n8, assign13390_e7732_d_n9, assign13390_e7732_d_n10, assign13390_e7732_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13390_e7729: f64 = (locals.var_log_tratio * p.p380);
        let assign13390_e7730: f64 = (assign13390_e7729).exp();
        (assign13390_e7730, (assign13390_e7730 * (locals.var_log_tratio_dn0 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn2 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn4 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn5 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn6 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn7 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn8 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn9 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn10 * p.p380)), (assign13390_e7730 * (locals.var_log_tratio_dn13 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13390_e7732;
        locals.var_t1_dn0 = assign13390_e7732_d_n0;
        locals.var_t1_dn2 = assign13390_e7732_d_n2;
        locals.var_t1_dn4 = assign13390_e7732_d_n4;
        locals.var_t1_dn5 = assign13390_e7732_d_n5;
        locals.var_t1_dn6 = assign13390_e7732_d_n6;
        locals.var_t1_dn7 = assign13390_e7732_d_n7;
        locals.var_t1_dn8 = assign13390_e7732_d_n8;
        locals.var_t1_dn9 = assign13390_e7732_d_n9;
        locals.var_t1_dn10 = assign13390_e7732_d_n10;
        locals.var_t1_dn13 = assign13390_e7732_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign13400_e7743, assign13400_e7743_d_n0, assign13400_e7743_d_n2, assign13400_e7743_d_n4, assign13400_e7743_d_n5, assign13400_e7743_d_n6, assign13400_e7743_d_n7, assign13400_e7743_d_n8, assign13400_e7743_d_n9, assign13400_e7743_d_n10, assign13400_e7743_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13400_e7741: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign13400_e7741, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn13 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn13,)
    }
};
        locals.var_depmphn0 = assign13400_e7743;
        locals.var_depmphn0_dn0 = assign13400_e7743_d_n0;
        locals.var_depmphn0_dn2 = assign13400_e7743_d_n2;
        locals.var_depmphn0_dn4 = assign13400_e7743_d_n4;
        locals.var_depmphn0_dn5 = assign13400_e7743_d_n5;
        locals.var_depmphn0_dn6 = assign13400_e7743_d_n6;
        locals.var_depmphn0_dn7 = assign13400_e7743_d_n7;
        locals.var_depmphn0_dn8 = assign13400_e7743_d_n8;
        locals.var_depmphn0_dn9 = assign13400_e7743_d_n9;
        locals.var_depmphn0_dn10 = assign13400_e7743_d_n10;
        locals.var_depmphn0_dn13 = assign13400_e7743_d_n13;
        locals.var_depmphn0_rv = 0.0;

        let (assign13410_e7768, assign13410_e7768_d_n0, assign13410_e7768_d_n2, assign13410_e7768_d_n4, assign13410_e7768_d_n5, assign13410_e7768_d_n6, assign13410_e7768_d_n7, assign13410_e7768_d_n8, assign13410_e7768_d_n9, assign13410_e7768_d_n10, assign13410_e7768_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13410_e7753: f64 = (0.4 * locals.var_tratio);
        let assign13410_e7754: f64 = (1.8 + assign13410_e7753);
        let assign13410_e7757: f64 = (0.1 * locals.var_tratio);
        let assign13410_e7759: f64 = (assign13410_e7757 * locals.var_tratio);
        let assign13410_e7760: f64 = (assign13410_e7754 + assign13410_e7759);
        let assign13410_e7764: f64 = (1.0 - locals.var_tratio);
        let assign13410_e7765: f64 = (p.p379 * assign13410_e7764);
        let assign13410_e7766: f64 = (assign13410_e7760 - assign13410_e7765);
        (assign13410_e7766, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign13410_e7757 * locals.var_tratio_dn13))) - (p.p379 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign13410_e7768;
        locals.var_t0_dn0 = assign13410_e7768_d_n0;
        locals.var_t0_dn2 = assign13410_e7768_d_n2;
        locals.var_t0_dn4 = assign13410_e7768_d_n4;
        locals.var_t0_dn5 = assign13410_e7768_d_n5;
        locals.var_t0_dn6 = assign13410_e7768_d_n6;
        locals.var_t0_dn7 = assign13410_e7768_d_n7;
        locals.var_t0_dn8 = assign13410_e7768_d_n8;
        locals.var_t0_dn9 = assign13410_e7768_d_n9;
        locals.var_t0_dn10 = assign13410_e7768_d_n10;
        locals.var_t0_dn13 = assign13410_e7768_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign13420_e7779, assign13420_e7779_d_n0, assign13420_e7779_d_n2, assign13420_e7779_d_n4, assign13420_e7779_d_n5, assign13420_e7779_d_n6, assign13420_e7779_d_n7, assign13420_e7779_d_n8, assign13420_e7779_d_n9, assign13420_e7779_d_n10, assign13420_e7779_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13420_e7777: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign13420_e7777, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn13 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign13420_e7779;
        locals.var_uc_depvmax_dn0 = assign13420_e7779_d_n0;
        locals.var_uc_depvmax_dn2 = assign13420_e7779_d_n2;
        locals.var_uc_depvmax_dn4 = assign13420_e7779_d_n4;
        locals.var_uc_depvmax_dn5 = assign13420_e7779_d_n5;
        locals.var_uc_depvmax_dn6 = assign13420_e7779_d_n6;
        locals.var_uc_depvmax_dn7 = assign13420_e7779_d_n7;
        locals.var_uc_depvmax_dn8 = assign13420_e7779_d_n8;
        locals.var_uc_depvmax_dn9 = assign13420_e7779_d_n9;
        locals.var_uc_depvmax_dn10 = assign13420_e7779_d_n10;
        locals.var_uc_depvmax_dn13 = assign13420_e7779_d_n13;
        locals.var_uc_depvmax_rv = 0.0;

        let assign13440_e7787: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard295 = assign13440_e7787;
        locals.var_guard295_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13450_e7798, assign13450_e7798_d_n0, assign13450_e7798_d_n2, assign13450_e7798_d_n4, assign13450_e7798_d_n5, assign13450_e7798_d_n6, assign13450_e7798_d_n7, assign13450_e7798_d_n8, assign13450_e7798_d_n9, assign13450_e7798_d_n10, assign13450_e7798_d_n13,) = {
    if ((((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) && (locals.var_guard295 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign13450_e7798;
        locals.var_uc_depvmax_dn0 = assign13450_e7798_d_n0;
        locals.var_uc_depvmax_dn2 = assign13450_e7798_d_n2;
        locals.var_uc_depvmax_dn4 = assign13450_e7798_d_n4;
        locals.var_uc_depvmax_dn5 = assign13450_e7798_d_n5;
        locals.var_uc_depvmax_dn6 = assign13450_e7798_d_n6;
        locals.var_uc_depvmax_dn7 = assign13450_e7798_d_n7;
        locals.var_uc_depvmax_dn8 = assign13450_e7798_d_n8;
        locals.var_uc_depvmax_dn9 = assign13450_e7798_d_n9;
        locals.var_uc_depvmax_dn10 = assign13450_e7798_d_n10;
        locals.var_uc_depvmax_dn13 = assign13450_e7798_d_n13;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign13460_e7811, assign13460_e7811_d_n0, assign13460_e7811_d_n2, assign13460_e7811_d_n4, assign13460_e7811_d_n5, assign13460_e7811_d_n6, assign13460_e7811_d_n7, assign13460_e7811_d_n8, assign13460_e7811_d_n9, assign13460_e7811_d_n10, assign13460_e7811_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13460_e7808: f64 = (locals.var_tratio).powf(p.p381);
        let assign13460_e7809: f64 = (locals.var_uc_depmue0 / assign13460_e7808);
        (assign13460_e7809, (((locals.var_uc_depmue0_dn0 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn2 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn4 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn5 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn6 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn7 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn8 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn9 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn10 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)), (((locals.var_uc_depmue0_dn13 * assign13460_e7808) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn13)) } } else { (assign13460_e7808 * (p.p381 * (locals.var_tratio_dn13 / locals.var_tratio))) })) / (assign13460_e7808 * assign13460_e7808)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign13460_e7811;
        locals.var_uc_depmue0_dn0 = assign13460_e7811_d_n0;
        locals.var_uc_depmue0_dn2 = assign13460_e7811_d_n2;
        locals.var_uc_depmue0_dn4 = assign13460_e7811_d_n4;
        locals.var_uc_depmue0_dn5 = assign13460_e7811_d_n5;
        locals.var_uc_depmue0_dn6 = assign13460_e7811_d_n6;
        locals.var_uc_depmue0_dn7 = assign13460_e7811_d_n7;
        locals.var_uc_depmue0_dn8 = assign13460_e7811_d_n8;
        locals.var_uc_depmue0_dn9 = assign13460_e7811_d_n9;
        locals.var_uc_depmue0_dn10 = assign13460_e7811_d_n10;
        locals.var_uc_depmue0_dn13 = assign13460_e7811_d_n13;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign13470_e7826, assign13470_e7826_d_n0, assign13470_e7826_d_n2, assign13470_e7826_d_n4, assign13470_e7826_d_n5, assign13470_e7826_d_n6, assign13470_e7826_d_n7, assign13470_e7826_d_n8, assign13470_e7826_d_n9, assign13470_e7826_d_n10, assign13470_e7826_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign13470_e7822: f64 = (locals.var_tratio - 1.0);
        let assign13470_e7823: f64 = (p.p365 * assign13470_e7822);
        let assign13470_e7824: f64 = (p.p364 + assign13470_e7823);
        (assign13470_e7824, (p.p365 * locals.var_tratio_dn0), (p.p365 * locals.var_tratio_dn2), (p.p365 * locals.var_tratio_dn4), (p.p365 * locals.var_tratio_dn5), (p.p365 * locals.var_tratio_dn6), (p.p365 * locals.var_tratio_dn7), (p.p365 * locals.var_tratio_dn8), (p.p365 * locals.var_tratio_dn9), (p.p365 * locals.var_tratio_dn10), (p.p365 * locals.var_tratio_dn13),)
    } else {
        (locals.var_uc_depwlp, locals.var_uc_depwlp_dn0, locals.var_uc_depwlp_dn2, locals.var_uc_depwlp_dn4, locals.var_uc_depwlp_dn5, locals.var_uc_depwlp_dn6, locals.var_uc_depwlp_dn7, locals.var_uc_depwlp_dn8, locals.var_uc_depwlp_dn9, locals.var_uc_depwlp_dn10, locals.var_uc_depwlp_dn13,)
    }
};
        locals.var_uc_depwlp = assign13470_e7826;
        locals.var_uc_depwlp_dn0 = assign13470_e7826_d_n0;
        locals.var_uc_depwlp_dn2 = assign13470_e7826_d_n2;
        locals.var_uc_depwlp_dn4 = assign13470_e7826_d_n4;
        locals.var_uc_depwlp_dn5 = assign13470_e7826_d_n5;
        locals.var_uc_depwlp_dn6 = assign13470_e7826_d_n6;
        locals.var_uc_depwlp_dn7 = assign13470_e7826_d_n7;
        locals.var_uc_depwlp_dn8 = assign13470_e7826_d_n8;
        locals.var_uc_depwlp_dn9 = assign13470_e7826_d_n9;
        locals.var_uc_depwlp_dn10 = assign13470_e7826_d_n10;
        locals.var_uc_depwlp_dn13 = assign13470_e7826_d_n13;
        locals.var_uc_depwlp_rv = 0.0;

        let (assign13480_e7836, assign13480_e7836_d_n0, assign13480_e7836_d_n2, assign13480_e7836_d_n4, assign13480_e7836_d_n5, assign13480_e7836_d_n6, assign13480_e7836_d_n7, assign13480_e7836_d_n8, assign13480_e7836_d_n9, assign13480_e7836_d_n10, assign13480_e7836_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    }
};
        locals.var_pb2n = assign13480_e7836;
        locals.var_pb2n_dn0 = assign13480_e7836_d_n0;
        locals.var_pb2n_dn2 = assign13480_e7836_d_n2;
        locals.var_pb2n_dn4 = assign13480_e7836_d_n4;
        locals.var_pb2n_dn5 = assign13480_e7836_d_n5;
        locals.var_pb2n_dn6 = assign13480_e7836_d_n6;
        locals.var_pb2n_dn7 = assign13480_e7836_d_n7;
        locals.var_pb2n_dn8 = assign13480_e7836_d_n8;
        locals.var_pb2n_dn9 = assign13480_e7836_d_n9;
        locals.var_pb2n_dn10 = assign13480_e7836_d_n10;
        locals.var_pb2n_dn13 = assign13480_e7836_d_n13;
        locals.var_pb2n_rv = 0.0;

        let (assign13490_e7855, assign13490_e7855_d_n0, assign13490_e7855_d_n2, assign13490_e7855_d_n4, assign13490_e7855_d_n5, assign13490_e7855_d_n6, assign13490_e7855_d_n7, assign13490_e7855_d_n8, assign13490_e7855_d_n9, assign13490_e7855_d_n10, assign13490_e7855_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 == 0.0)) {
        let assign13490_e7847: f64 = (locals.var_uc_njunc / locals.var_nin);
        let assign13490_e7849: f64 = (assign13490_e7847 * locals.var_nsub);
        let assign13490_e7851: f64 = (assign13490_e7849 / locals.var_nin);
        let assign13490_e7852: f64 = (assign13490_e7851).ln();
        let assign13490_e7853: f64 = (locals.var_beta_inv * assign13490_e7852);
        (assign13490_e7853, ((locals.var_beta_inv_dn0 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn0)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn2 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn2)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn4 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn4)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn5 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn5)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn6 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn6)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn7 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn7)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn8 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn8)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn9 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn9)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn10 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn10)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))), ((locals.var_beta_inv_dn13 * assign13490_e7852) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn13) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13490_e7847 * locals.var_nsub_dn13)) * locals.var_nin) - (assign13490_e7849 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign13490_e7851))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    }
};
        locals.var_vbipn = assign13490_e7855;
        locals.var_vbipn_dn0 = assign13490_e7855_d_n0;
        locals.var_vbipn_dn2 = assign13490_e7855_d_n2;
        locals.var_vbipn_dn4 = assign13490_e7855_d_n4;
        locals.var_vbipn_dn5 = assign13490_e7855_d_n5;
        locals.var_vbipn_dn6 = assign13490_e7855_d_n6;
        locals.var_vbipn_dn7 = assign13490_e7855_d_n7;
        locals.var_vbipn_dn8 = assign13490_e7855_d_n8;
        locals.var_vbipn_dn9 = assign13490_e7855_d_n9;
        locals.var_vbipn_dn10 = assign13490_e7855_d_n10;
        locals.var_vbipn_dn13 = assign13490_e7855_d_n13;
        locals.var_vbipn_rv = 0.0;

        let (assign13500_e7865, assign13500_e7865_d_n0, assign13500_e7865_d_n2, assign13500_e7865_d_n4, assign13500_e7865_d_n5, assign13500_e7865_d_n6, assign13500_e7865_d_n7, assign13500_e7865_d_n8, assign13500_e7865_d_n9, assign13500_e7865_d_n10, assign13500_e7865_d_n13,) = {
    if (((locals.var_guard289 != 0.0) && (locals.var_guard290 == 0.0)) && (locals.var_guard293 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn13,)
    }
};
        locals.var_depmphn0 = assign13500_e7865;
        locals.var_depmphn0_dn0 = assign13500_e7865_d_n0;
        locals.var_depmphn0_dn2 = assign13500_e7865_d_n2;
        locals.var_depmphn0_dn4 = assign13500_e7865_d_n4;
        locals.var_depmphn0_dn5 = assign13500_e7865_d_n5;
        locals.var_depmphn0_dn6 = assign13500_e7865_d_n6;
        locals.var_depmphn0_dn7 = assign13500_e7865_d_n7;
        locals.var_depmphn0_dn8 = assign13500_e7865_d_n8;
        locals.var_depmphn0_dn9 = assign13500_e7865_d_n9;
        locals.var_depmphn0_dn10 = assign13500_e7865_d_n10;
        locals.var_depmphn0_dn13 = assign13500_e7865_d_n13;
        locals.var_depmphn0_rv = 0.0;

        let (assign13510_e7871, assign13510_e7871_d_n0, assign13510_e7871_d_n2, assign13510_e7871_d_n4, assign13510_e7871_d_n5, assign13510_e7871_d_n6, assign13510_e7871_d_n7, assign13510_e7871_d_n8, assign13510_e7871_d_n9, assign13510_e7871_d_n10, assign13510_e7871_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13510_e7869: f64 = (locals.var_ptovr0 * locals.var_beta_inv);
        (assign13510_e7869, ((locals.var_ptovr0_dn0 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn0)), ((locals.var_ptovr0_dn2 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn2)), ((locals.var_ptovr0_dn4 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn4)), ((locals.var_ptovr0_dn5 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn5)), ((locals.var_ptovr0_dn6 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn6)), ((locals.var_ptovr0_dn7 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn7)), ((locals.var_ptovr0_dn8 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn8)), ((locals.var_ptovr0_dn9 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn9)), ((locals.var_ptovr0_dn10 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn10)), ((locals.var_ptovr0_dn13 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn13)),)
    } else {
        (locals.var_ptovr, locals.var_ptovr_dn0, locals.var_ptovr_dn2, locals.var_ptovr_dn4, locals.var_ptovr_dn5, locals.var_ptovr_dn6, locals.var_ptovr_dn7, locals.var_ptovr_dn8, locals.var_ptovr_dn9, locals.var_ptovr_dn10, locals.var_ptovr_dn13,)
    }
};
        locals.var_ptovr = assign13510_e7871;
        locals.var_ptovr_dn0 = assign13510_e7871_d_n0;
        locals.var_ptovr_dn2 = assign13510_e7871_d_n2;
        locals.var_ptovr_dn4 = assign13510_e7871_d_n4;
        locals.var_ptovr_dn5 = assign13510_e7871_d_n5;
        locals.var_ptovr_dn6 = assign13510_e7871_d_n6;
        locals.var_ptovr_dn7 = assign13510_e7871_d_n7;
        locals.var_ptovr_dn8 = assign13510_e7871_d_n8;
        locals.var_ptovr_dn9 = assign13510_e7871_d_n9;
        locals.var_ptovr_dn10 = assign13510_e7871_d_n10;
        locals.var_ptovr_dn13 = assign13510_e7871_d_n13;
        locals.var_ptovr_rv = 0.0;

        let (assign13520_e7877, assign13520_e7877_d_n0, assign13520_e7877_d_n2, assign13520_e7877_d_n4, assign13520_e7877_d_n5, assign13520_e7877_d_n6, assign13520_e7877_d_n7, assign13520_e7877_d_n8, assign13520_e7877_d_n9, assign13520_e7877_d_n10, assign13520_e7877_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13520_e7875: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign13520_e7875, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn13 / locals.var_ktnom),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13520_e7877;
        locals.var_t1_dn0 = assign13520_e7877_d_n0;
        locals.var_t1_dn2 = assign13520_e7877_d_n2;
        locals.var_t1_dn4 = assign13520_e7877_d_n4;
        locals.var_t1_dn5 = assign13520_e7877_d_n5;
        locals.var_t1_dn6 = assign13520_e7877_d_n6;
        locals.var_t1_dn7 = assign13520_e7877_d_n7;
        locals.var_t1_dn8 = assign13520_e7877_d_n8;
        locals.var_t1_dn9 = assign13520_e7877_d_n9;
        locals.var_t1_dn10 = assign13520_e7877_d_n10;
        locals.var_t1_dn13 = assign13520_e7877_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign13530_e7897, assign13530_e7897_d_n0, assign13530_e7897_d_n2, assign13530_e7897_d_n4, assign13530_e7897_d_n5, assign13530_e7897_d_n6, assign13530_e7897_d_n7, assign13530_e7897_d_n8, assign13530_e7897_d_n9, assign13530_e7897_d_n10, assign13530_e7897_d_n13,) = {
    if (locals.var_guard289 != 0.0) {
        let assign13530_e7882: f64 = (0.4 * locals.var_t1);
        let assign13530_e7883: f64 = (1.8 + assign13530_e7882);
        let assign13530_e7886: f64 = (0.1 * locals.var_t1);
        let assign13530_e7888: f64 = (assign13530_e7886 * locals.var_t1);
        let assign13530_e7889: f64 = (assign13530_e7883 + assign13530_e7888);
        let assign13530_e7893: f64 = (1.0 - locals.var_t1);
        let assign13530_e7894: f64 = (locals.var_uc_vtmp * assign13530_e7893);
        let assign13530_e7895: f64 = (assign13530_e7889 - assign13530_e7894);
        (assign13530_e7895, (((0.4 * locals.var_t1_dn0) + (((0.1 * locals.var_t1_dn0) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn0))) - (locals.var_uc_vtmp * (-locals.var_t1_dn0))), (((0.4 * locals.var_t1_dn2) + (((0.1 * locals.var_t1_dn2) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn2))) - (locals.var_uc_vtmp * (-locals.var_t1_dn2))), (((0.4 * locals.var_t1_dn4) + (((0.1 * locals.var_t1_dn4) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn4))) - (locals.var_uc_vtmp * (-locals.var_t1_dn4))), (((0.4 * locals.var_t1_dn5) + (((0.1 * locals.var_t1_dn5) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn5))) - (locals.var_uc_vtmp * (-locals.var_t1_dn5))), (((0.4 * locals.var_t1_dn6) + (((0.1 * locals.var_t1_dn6) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn6))) - (locals.var_uc_vtmp * (-locals.var_t1_dn6))), (((0.4 * locals.var_t1_dn7) + (((0.1 * locals.var_t1_dn7) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn7))) - (locals.var_uc_vtmp * (-locals.var_t1_dn7))), (((0.4 * locals.var_t1_dn8) + (((0.1 * locals.var_t1_dn8) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn8))) - (locals.var_uc_vtmp * (-locals.var_t1_dn8))), (((0.4 * locals.var_t1_dn9) + (((0.1 * locals.var_t1_dn9) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn9))) - (locals.var_uc_vtmp * (-locals.var_t1_dn9))), (((0.4 * locals.var_t1_dn10) + (((0.1 * locals.var_t1_dn10) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn10))) - (locals.var_uc_vtmp * (-locals.var_t1_dn10))), (((0.4 * locals.var_t1_dn13) + (((0.1 * locals.var_t1_dn13) * locals.var_t1) + (assign13530_e7886 * locals.var_t1_dn13))) - (locals.var_uc_vtmp * (-locals.var_t1_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign13530_e7897;
        locals.var_t0_dn0 = assign13530_e7897_d_n0;
        locals.var_t0_dn2 = assign13530_e7897_d_n2;
        locals.var_t0_dn4 = assign13530_e7897_d_n4;
        locals.var_t0_dn5 = assign13530_e7897_d_n5;
        locals.var_t0_dn6 = assign13530_e7897_d_n6;
        locals.var_t0_dn7 = assign13530_e7897_d_n7;
        locals.var_t0_dn8 = assign13530_e7897_d_n8;
        locals.var_t0_dn9 = assign13530_e7897_d_n9;
        locals.var_t0_dn10 = assign13530_e7897_d_n10;
        locals.var_t0_dn13 = assign13530_e7897_d_n13;
        locals.var_t0_rv = 0.0;

        let assign13540_e7900: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard296 = assign13540_e7900;
        locals.var_guard296_rv = 0.0;

        let (assign13550_e7920, assign13550_e7920_d_n0, assign13550_e7920_d_n2, assign13550_e7920_d_n4, assign13550_e7920_d_n5, assign13550_e7920_d_n6, assign13550_e7920_d_n7, assign13550_e7920_d_n8, assign13550_e7920_d_n9, assign13550_e7920_d_n10, assign13550_e7920_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard296 != 0.0)) {
        let assign13550_e7906: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign13550_e7908: f64 = (assign13550_e7906 / locals.var_t0);
        let assign13550_e7912: f64 = (p.p90 * locals.var_tdiff0);
        let assign13550_e7913: f64 = (1.0 + assign13550_e7912);
        let assign13550_e7916: f64 = (p.p91 * locals.var_tdiff0_2);
        let assign13550_e7917: f64 = (assign13550_e7913 + assign13550_e7916);
        let assign13550_e7918: f64 = (assign13550_e7908 * assign13550_e7917);
        (assign13550_e7918, (((-((assign13550_e7906 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn0) + (p.p91 * locals.var_tdiff0_2_dn0)))), (((-((assign13550_e7906 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn2) + (p.p91 * locals.var_tdiff0_2_dn2)))), (((-((assign13550_e7906 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn4) + (p.p91 * locals.var_tdiff0_2_dn4)))), (((-((assign13550_e7906 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn5) + (p.p91 * locals.var_tdiff0_2_dn5)))), (((-((assign13550_e7906 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn6) + (p.p91 * locals.var_tdiff0_2_dn6)))), (((-((assign13550_e7906 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn7) + (p.p91 * locals.var_tdiff0_2_dn7)))), (((-((assign13550_e7906 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn8) + (p.p91 * locals.var_tdiff0_2_dn8)))), (((-((assign13550_e7906 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn9) + (p.p91 * locals.var_tdiff0_2_dn9)))), (((-((assign13550_e7906 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn10) + (p.p91 * locals.var_tdiff0_2_dn10)))), (((-((assign13550_e7906 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) * assign13550_e7917) + (assign13550_e7908 * ((p.p90 * locals.var_tdiff0_dn13) + (p.p91 * locals.var_tdiff0_2_dn13)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn13,)
    }
};
        locals.var_vmaxeff = assign13550_e7920;
        locals.var_vmaxeff_dn0 = assign13550_e7920_d_n0;
        locals.var_vmaxeff_dn2 = assign13550_e7920_d_n2;
        locals.var_vmaxeff_dn4 = assign13550_e7920_d_n4;
        locals.var_vmaxeff_dn5 = assign13550_e7920_d_n5;
        locals.var_vmaxeff_dn6 = assign13550_e7920_d_n6;
        locals.var_vmaxeff_dn7 = assign13550_e7920_d_n7;
        locals.var_vmaxeff_dn8 = assign13550_e7920_d_n8;
        locals.var_vmaxeff_dn9 = assign13550_e7920_d_n9;
        locals.var_vmaxeff_dn10 = assign13550_e7920_d_n10;
        locals.var_vmaxeff_dn13 = assign13550_e7920_d_n13;
        locals.var_vmaxeff_rv = 0.0;

        let (assign13560_e7941, assign13560_e7941_d_n0, assign13560_e7941_d_n2, assign13560_e7941_d_n4, assign13560_e7941_d_n5, assign13560_e7941_d_n6, assign13560_e7941_d_n7, assign13560_e7941_d_n8, assign13560_e7941_d_n9, assign13560_e7941_d_n10, assign13560_e7941_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard296 == 0.0)) {
        let assign13560_e7927: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign13560_e7929: f64 = (assign13560_e7927 / locals.var_t0);
        let assign13560_e7933: f64 = (p.p90 * locals.var_tdiff);
        let assign13560_e7934: f64 = (1.0 + assign13560_e7933);
        let assign13560_e7937: f64 = (p.p91 * locals.var_tdiff_2);
        let assign13560_e7938: f64 = (assign13560_e7934 + assign13560_e7937);
        let assign13560_e7939: f64 = (assign13560_e7929 * assign13560_e7938);
        (assign13560_e7939, (((-((assign13560_e7927 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn0) + (p.p91 * locals.var_tdiff_2_dn0)))), (((-((assign13560_e7927 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn2) + (p.p91 * locals.var_tdiff_2_dn2)))), (((-((assign13560_e7927 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn4) + (p.p91 * locals.var_tdiff_2_dn4)))), (((-((assign13560_e7927 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn5) + (p.p91 * locals.var_tdiff_2_dn5)))), (((-((assign13560_e7927 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn6) + (p.p91 * locals.var_tdiff_2_dn6)))), (((-((assign13560_e7927 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn7) + (p.p91 * locals.var_tdiff_2_dn7)))), (((-((assign13560_e7927 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn8) + (p.p91 * locals.var_tdiff_2_dn8)))), (((-((assign13560_e7927 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn9) + (p.p91 * locals.var_tdiff_2_dn9)))), (((-((assign13560_e7927 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn10) + (p.p91 * locals.var_tdiff_2_dn10)))), (((-((assign13560_e7927 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) * assign13560_e7938) + (assign13560_e7929 * ((p.p90 * locals.var_tdiff_dn13) + (p.p91 * locals.var_tdiff_2_dn13)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn13,)
    }
};
        locals.var_vmaxeff = assign13560_e7941;
        locals.var_vmaxeff_dn0 = assign13560_e7941_d_n0;
        locals.var_vmaxeff_dn2 = assign13560_e7941_d_n2;
        locals.var_vmaxeff_dn4 = assign13560_e7941_d_n4;
        locals.var_vmaxeff_dn5 = assign13560_e7941_d_n5;
        locals.var_vmaxeff_dn6 = assign13560_e7941_d_n6;
        locals.var_vmaxeff_dn7 = assign13560_e7941_d_n7;
        locals.var_vmaxeff_dn8 = assign13560_e7941_d_n8;
        locals.var_vmaxeff_dn9 = assign13560_e7941_d_n9;
        locals.var_vmaxeff_dn10 = assign13560_e7941_d_n10;
        locals.var_vmaxeff_dn13 = assign13560_e7941_d_n13;
        locals.var_vmaxeff_rv = 0.0;

        let assign13580_e7949: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard298 = assign13580_e7949;
        locals.var_guard298_rv = 0.0;

        let (assign13590_e7965, assign13590_e7965_d_n0, assign13590_e7965_d_n2, assign13590_e7965_d_n4, assign13590_e7965_d_n5, assign13590_e7965_d_n6, assign13590_e7965_d_n7, assign13590_e7965_d_n8, assign13590_e7965_d_n9, assign13590_e7965_d_n10, assign13590_e7965_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 != 0.0)) {
        let assign13590_e7957: f64 = (p.p324 * locals.var_tdiff0);
        let assign13590_e7958: f64 = (1.0 + assign13590_e7957);
        let assign13590_e7961: f64 = (p.p325 * locals.var_tdiff0_2);
        let assign13590_e7962: f64 = (assign13590_e7958 + assign13590_e7961);
        let assign13590_e7963: f64 = (locals.var_ninvd0 * assign13590_e7962);
        (assign13590_e7963, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn0) + (p.p325 * locals.var_tdiff0_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn2) + (p.p325 * locals.var_tdiff0_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn4) + (p.p325 * locals.var_tdiff0_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn5) + (p.p325 * locals.var_tdiff0_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn6) + (p.p325 * locals.var_tdiff0_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn7) + (p.p325 * locals.var_tdiff0_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn8) + (p.p325 * locals.var_tdiff0_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn9) + (p.p325 * locals.var_tdiff0_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn10) + (p.p325 * locals.var_tdiff0_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn13) + (p.p325 * locals.var_tdiff0_2_dn13))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    }
};
        locals.var_ninvde = assign13590_e7965;
        locals.var_ninvde_dn0 = assign13590_e7965_d_n0;
        locals.var_ninvde_dn2 = assign13590_e7965_d_n2;
        locals.var_ninvde_dn4 = assign13590_e7965_d_n4;
        locals.var_ninvde_dn5 = assign13590_e7965_d_n5;
        locals.var_ninvde_dn6 = assign13590_e7965_d_n6;
        locals.var_ninvde_dn7 = assign13590_e7965_d_n7;
        locals.var_ninvde_dn8 = assign13590_e7965_d_n8;
        locals.var_ninvde_dn9 = assign13590_e7965_d_n9;
        locals.var_ninvde_dn10 = assign13590_e7965_d_n10;
        locals.var_ninvde_dn13 = assign13590_e7965_d_n13;
        locals.var_ninvde_rv = 0.0;

        let (assign13600_e7979, assign13600_e7979_d_n0, assign13600_e7979_d_n2, assign13600_e7979_d_n4, assign13600_e7979_d_n5, assign13600_e7979_d_n6, assign13600_e7979_d_n7, assign13600_e7979_d_n8, assign13600_e7979_d_n9, assign13600_e7979_d_n10, assign13600_e7979_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 != 0.0)) {
        let assign13600_e7972: f64 = (p.p390 * locals.var_tdiff0);
        let assign13600_e7973: f64 = (1.0 + assign13600_e7972);
        let assign13600_e7976: f64 = (p.p391 * locals.var_tdiff0_2);
        let assign13600_e7977: f64 = (assign13600_e7973 + assign13600_e7976);
        (assign13600_e7977, ((p.p390 * locals.var_tdiff0_dn0) + (p.p391 * locals.var_tdiff0_2_dn0)), ((p.p390 * locals.var_tdiff0_dn2) + (p.p391 * locals.var_tdiff0_2_dn2)), ((p.p390 * locals.var_tdiff0_dn4) + (p.p391 * locals.var_tdiff0_2_dn4)), ((p.p390 * locals.var_tdiff0_dn5) + (p.p391 * locals.var_tdiff0_2_dn5)), ((p.p390 * locals.var_tdiff0_dn6) + (p.p391 * locals.var_tdiff0_2_dn6)), ((p.p390 * locals.var_tdiff0_dn7) + (p.p391 * locals.var_tdiff0_2_dn7)), ((p.p390 * locals.var_tdiff0_dn8) + (p.p391 * locals.var_tdiff0_2_dn8)), ((p.p390 * locals.var_tdiff0_dn9) + (p.p391 * locals.var_tdiff0_2_dn9)), ((p.p390 * locals.var_tdiff0_dn10) + (p.p391 * locals.var_tdiff0_2_dn10)), ((p.p390 * locals.var_tdiff0_dn13) + (p.p391 * locals.var_tdiff0_2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13600_e7979;
        locals.var_t1_dn0 = assign13600_e7979_d_n0;
        locals.var_t1_dn2 = assign13600_e7979_d_n2;
        locals.var_t1_dn4 = assign13600_e7979_d_n4;
        locals.var_t1_dn5 = assign13600_e7979_d_n5;
        locals.var_t1_dn6 = assign13600_e7979_d_n6;
        locals.var_t1_dn7 = assign13600_e7979_d_n7;
        locals.var_t1_dn8 = assign13600_e7979_d_n8;
        locals.var_t1_dn9 = assign13600_e7979_d_n9;
        locals.var_t1_dn10 = assign13600_e7979_d_n10;
        locals.var_t1_dn13 = assign13600_e7979_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign13610_e7987, assign13610_e7987_d_n0, assign13610_e7987_d_n2, assign13610_e7987_d_n4, assign13610_e7987_d_n5, assign13610_e7987_d_n6, assign13610_e7987_d_n7, assign13610_e7987_d_n8, assign13610_e7987_d_n9, assign13610_e7987_d_n10, assign13610_e7987_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 != 0.0)) {
        let assign13610_e7985: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign13610_e7985, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn13 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn13,)
    }
};
        locals.var_ninvdecres = assign13610_e7987;
        locals.var_ninvdecres_dn0 = assign13610_e7987_d_n0;
        locals.var_ninvdecres_dn2 = assign13610_e7987_d_n2;
        locals.var_ninvdecres_dn4 = assign13610_e7987_d_n4;
        locals.var_ninvdecres_dn5 = assign13610_e7987_d_n5;
        locals.var_ninvdecres_dn6 = assign13610_e7987_d_n6;
        locals.var_ninvdecres_dn7 = assign13610_e7987_d_n7;
        locals.var_ninvdecres_dn8 = assign13610_e7987_d_n8;
        locals.var_ninvdecres_dn9 = assign13610_e7987_d_n9;
        locals.var_ninvdecres_dn10 = assign13610_e7987_d_n10;
        locals.var_ninvdecres_dn13 = assign13610_e7987_d_n13;
        locals.var_ninvdecres_rv = 0.0;

        let (assign13620_e7995, assign13620_e7995_d_n0, assign13620_e7995_d_n2, assign13620_e7995_d_n4, assign13620_e7995_d_n5, assign13620_e7995_d_n6, assign13620_e7995_d_n7, assign13620_e7995_d_n8, assign13620_e7995_d_n9, assign13620_e7995_d_n10, assign13620_e7995_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 != 0.0)) {
        let assign13620_e7993: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign13620_e7993, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn13 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn13,)
    }
};
        locals.var_ninvdehres = assign13620_e7995;
        locals.var_ninvdehres_dn0 = assign13620_e7995_d_n0;
        locals.var_ninvdehres_dn2 = assign13620_e7995_d_n2;
        locals.var_ninvdehres_dn4 = assign13620_e7995_d_n4;
        locals.var_ninvdehres_dn5 = assign13620_e7995_d_n5;
        locals.var_ninvdehres_dn6 = assign13620_e7995_d_n6;
        locals.var_ninvdehres_dn7 = assign13620_e7995_d_n7;
        locals.var_ninvdehres_dn8 = assign13620_e7995_d_n8;
        locals.var_ninvdehres_dn9 = assign13620_e7995_d_n9;
        locals.var_ninvdehres_dn10 = assign13620_e7995_d_n10;
        locals.var_ninvdehres_dn13 = assign13620_e7995_d_n13;
        locals.var_ninvdehres_rv = 0.0;

        let (assign13630_e8012, assign13630_e8012_d_n0, assign13630_e8012_d_n2, assign13630_e8012_d_n4, assign13630_e8012_d_n5, assign13630_e8012_d_n6, assign13630_e8012_d_n7, assign13630_e8012_d_n8, assign13630_e8012_d_n9, assign13630_e8012_d_n10, assign13630_e8012_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 == 0.0)) {
        let assign13630_e8004: f64 = (p.p324 * locals.var_tdiff);
        let assign13630_e8005: f64 = (1.0 + assign13630_e8004);
        let assign13630_e8008: f64 = (p.p325 * locals.var_tdiff_2);
        let assign13630_e8009: f64 = (assign13630_e8005 + assign13630_e8008);
        let assign13630_e8010: f64 = (locals.var_ninvd0 * assign13630_e8009);
        (assign13630_e8010, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn0) + (p.p325 * locals.var_tdiff_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn2) + (p.p325 * locals.var_tdiff_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn4) + (p.p325 * locals.var_tdiff_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn5) + (p.p325 * locals.var_tdiff_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn6) + (p.p325 * locals.var_tdiff_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn7) + (p.p325 * locals.var_tdiff_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn8) + (p.p325 * locals.var_tdiff_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn9) + (p.p325 * locals.var_tdiff_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn10) + (p.p325 * locals.var_tdiff_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn13) + (p.p325 * locals.var_tdiff_2_dn13))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    }
};
        locals.var_ninvde = assign13630_e8012;
        locals.var_ninvde_dn0 = assign13630_e8012_d_n0;
        locals.var_ninvde_dn2 = assign13630_e8012_d_n2;
        locals.var_ninvde_dn4 = assign13630_e8012_d_n4;
        locals.var_ninvde_dn5 = assign13630_e8012_d_n5;
        locals.var_ninvde_dn6 = assign13630_e8012_d_n6;
        locals.var_ninvde_dn7 = assign13630_e8012_d_n7;
        locals.var_ninvde_dn8 = assign13630_e8012_d_n8;
        locals.var_ninvde_dn9 = assign13630_e8012_d_n9;
        locals.var_ninvde_dn10 = assign13630_e8012_d_n10;
        locals.var_ninvde_dn13 = assign13630_e8012_d_n13;
        locals.var_ninvde_rv = 0.0;

        let (assign13640_e8027, assign13640_e8027_d_n0, assign13640_e8027_d_n2, assign13640_e8027_d_n4, assign13640_e8027_d_n5, assign13640_e8027_d_n6, assign13640_e8027_d_n7, assign13640_e8027_d_n8, assign13640_e8027_d_n9, assign13640_e8027_d_n10, assign13640_e8027_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 == 0.0)) {
        let assign13640_e8020: f64 = (p.p390 * locals.var_tdiff);
        let assign13640_e8021: f64 = (1.0 + assign13640_e8020);
        let assign13640_e8024: f64 = (p.p391 * locals.var_tdiff_2);
        let assign13640_e8025: f64 = (assign13640_e8021 + assign13640_e8024);
        (assign13640_e8025, ((p.p390 * locals.var_tdiff_dn0) + (p.p391 * locals.var_tdiff_2_dn0)), ((p.p390 * locals.var_tdiff_dn2) + (p.p391 * locals.var_tdiff_2_dn2)), ((p.p390 * locals.var_tdiff_dn4) + (p.p391 * locals.var_tdiff_2_dn4)), ((p.p390 * locals.var_tdiff_dn5) + (p.p391 * locals.var_tdiff_2_dn5)), ((p.p390 * locals.var_tdiff_dn6) + (p.p391 * locals.var_tdiff_2_dn6)), ((p.p390 * locals.var_tdiff_dn7) + (p.p391 * locals.var_tdiff_2_dn7)), ((p.p390 * locals.var_tdiff_dn8) + (p.p391 * locals.var_tdiff_2_dn8)), ((p.p390 * locals.var_tdiff_dn9) + (p.p391 * locals.var_tdiff_2_dn9)), ((p.p390 * locals.var_tdiff_dn10) + (p.p391 * locals.var_tdiff_2_dn10)), ((p.p390 * locals.var_tdiff_dn13) + (p.p391 * locals.var_tdiff_2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign13640_e8027;
        locals.var_t1_dn0 = assign13640_e8027_d_n0;
        locals.var_t1_dn2 = assign13640_e8027_d_n2;
        locals.var_t1_dn4 = assign13640_e8027_d_n4;
        locals.var_t1_dn5 = assign13640_e8027_d_n5;
        locals.var_t1_dn6 = assign13640_e8027_d_n6;
        locals.var_t1_dn7 = assign13640_e8027_d_n7;
        locals.var_t1_dn8 = assign13640_e8027_d_n8;
        locals.var_t1_dn9 = assign13640_e8027_d_n9;
        locals.var_t1_dn10 = assign13640_e8027_d_n10;
        locals.var_t1_dn13 = assign13640_e8027_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign13650_e8036, assign13650_e8036_d_n0, assign13650_e8036_d_n2, assign13650_e8036_d_n4, assign13650_e8036_d_n5, assign13650_e8036_d_n6, assign13650_e8036_d_n7, assign13650_e8036_d_n8, assign13650_e8036_d_n9, assign13650_e8036_d_n10, assign13650_e8036_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 == 0.0)) {
        let assign13650_e8034: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign13650_e8034, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn13 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn13,)
    }
};
        locals.var_ninvdecres = assign13650_e8036;
        locals.var_ninvdecres_dn0 = assign13650_e8036_d_n0;
        locals.var_ninvdecres_dn2 = assign13650_e8036_d_n2;
        locals.var_ninvdecres_dn4 = assign13650_e8036_d_n4;
        locals.var_ninvdecres_dn5 = assign13650_e8036_d_n5;
        locals.var_ninvdecres_dn6 = assign13650_e8036_d_n6;
        locals.var_ninvdecres_dn7 = assign13650_e8036_d_n7;
        locals.var_ninvdecres_dn8 = assign13650_e8036_d_n8;
        locals.var_ninvdecres_dn9 = assign13650_e8036_d_n9;
        locals.var_ninvdecres_dn10 = assign13650_e8036_d_n10;
        locals.var_ninvdecres_dn13 = assign13650_e8036_d_n13;
        locals.var_ninvdecres_rv = 0.0;

        let (assign13660_e8045, assign13660_e8045_d_n0, assign13660_e8045_d_n2, assign13660_e8045_d_n4, assign13660_e8045_d_n5, assign13660_e8045_d_n6, assign13660_e8045_d_n7, assign13660_e8045_d_n8, assign13660_e8045_d_n9, assign13660_e8045_d_n10, assign13660_e8045_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard298 == 0.0)) {
        let assign13660_e8043: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign13660_e8043, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn13 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn13,)
    }
};
        locals.var_ninvdehres = assign13660_e8045;
        locals.var_ninvdehres_dn0 = assign13660_e8045_d_n0;
        locals.var_ninvdehres_dn2 = assign13660_e8045_d_n2;
        locals.var_ninvdehres_dn4 = assign13660_e8045_d_n4;
        locals.var_ninvdehres_dn5 = assign13660_e8045_d_n5;
        locals.var_ninvdehres_dn6 = assign13660_e8045_d_n6;
        locals.var_ninvdehres_dn7 = assign13660_e8045_d_n7;
        locals.var_ninvdehres_dn8 = assign13660_e8045_d_n8;
        locals.var_ninvdehres_dn9 = assign13660_e8045_d_n9;
        locals.var_ninvdehres_dn10 = assign13660_e8045_d_n10;
        locals.var_ninvdehres_dn13 = assign13660_e8045_d_n13;
        locals.var_ninvdehres_rv = 0.0;

        let assign13680_e8053: f64 = if locals.var_ninvde < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard300 = assign13680_e8053;
        locals.var_guard300_rv = 0.0;

        let (assign13690_e8059, assign13690_e8059_d_n0, assign13690_e8059_d_n2, assign13690_e8059_d_n4, assign13690_e8059_d_n5, assign13690_e8059_d_n6, assign13690_e8059_d_n7, assign13690_e8059_d_n8, assign13690_e8059_d_n9, assign13690_e8059_d_n10, assign13690_e8059_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard300 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    }
};
        locals.var_ninvde = assign13690_e8059;
        locals.var_ninvde_dn0 = assign13690_e8059_d_n0;
        locals.var_ninvde_dn2 = assign13690_e8059_d_n2;
        locals.var_ninvde_dn4 = assign13690_e8059_d_n4;
        locals.var_ninvde_dn5 = assign13690_e8059_d_n5;
        locals.var_ninvde_dn6 = assign13690_e8059_d_n6;
        locals.var_ninvde_dn7 = assign13690_e8059_d_n7;
        locals.var_ninvde_dn8 = assign13690_e8059_d_n8;
        locals.var_ninvde_dn9 = assign13690_e8059_d_n9;
        locals.var_ninvde_dn10 = assign13690_e8059_d_n10;
        locals.var_ninvde_dn13 = assign13690_e8059_d_n13;
        locals.var_ninvde_rv = 0.0;

        let assign13710_e8067: f64 = if locals.var_ninvdecres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard302 = assign13710_e8067;
        locals.var_guard302_rv = 0.0;

        let (assign13720_e8073, assign13720_e8073_d_n0, assign13720_e8073_d_n2, assign13720_e8073_d_n4, assign13720_e8073_d_n5, assign13720_e8073_d_n6, assign13720_e8073_d_n7, assign13720_e8073_d_n8, assign13720_e8073_d_n9, assign13720_e8073_d_n10, assign13720_e8073_d_n13,) = {
    if ((locals.var_guard289 != 0.0) && (locals.var_guard302 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn13,)
    }
};
        locals.var_ninvdecres = assign13720_e8073;
        locals.var_ninvdecres_dn0 = assign13720_e8073_d_n0;
        locals.var_ninvdecres_dn2 = assign13720_e8073_d_n2;
        locals.var_ninvdecres_dn4 = assign13720_e8073_d_n4;
        locals.var_ninvdecres_dn5 = assign13720_e8073_d_n5;
        locals.var_ninvdecres_dn6 = assign13720_e8073_d_n6;
        locals.var_ninvdecres_dn7 = assign13720_e8073_d_n7;
        locals.var_ninvdecres_dn8 = assign13720_e8073_d_n8;
        locals.var_ninvdecres_dn9 = assign13720_e8073_d_n9;
        locals.var_ninvdecres_dn10 = assign13720_e8073_d_n10;
        locals.var_ninvdecres_dn13 = assign13720_e8073_d_n13;
        locals.var_ninvdecres_rv = 0.0;

        let assign13740_e8081: f64 = if locals.var_ninvdehres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard304 = assign13740_e8081;
        locals.var_guard304_rv = 0.0;

    }
}
