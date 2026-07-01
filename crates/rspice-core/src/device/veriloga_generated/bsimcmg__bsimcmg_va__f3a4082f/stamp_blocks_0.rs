#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        var_cfr_geo_slot: &mut f64,
        var_cfr_geo_dn0_slot: &mut f64,
        var_cfr_geo_dn10_slot: &mut f64,
        var_cfr_geo_dn11_slot: &mut f64,
        var_cfr_geo_dn13_slot: &mut f64,
        var_cfr_geo_dn14_slot: &mut f64,
        var_cfr_geo_dn2_slot: &mut f64,
        var_cfr_geo_dn3_slot: &mut f64,
        var_cfr_geo_dn4_slot: &mut f64,
        var_cfr_geo_dn5_slot: &mut f64,
        var_cfr_geo_dn6_slot: &mut f64,
        var_cfr_geo_dn7_slot: &mut f64,
        var_cfr_geo_dn8_slot: &mut f64,
        var_cfr_geo_dn9_slot: &mut f64,
        var_cjd_t_slot: &mut f64,
        var_cjd_t_dn4_slot: &mut f64,
        var_cjs_t_slot: &mut f64,
        var_cjs_t_dn4_slot: &mut f64,
        var_cjswd_t_slot: &mut f64,
        var_cjswd_t_dn4_slot: &mut f64,
        var_cjswgd_t_slot: &mut f64,
        var_cjswgd_t_dn4_slot: &mut f64,
        var_cjswgs_t_slot: &mut f64,
        var_cjswgs_t_dn4_slot: &mut f64,
        var_cjsws_t_slot: &mut f64,
        var_cjsws_t_dn4_slot: &mut f64,
        var_dr_slot: &mut f64,
        var_dr_dn0_slot: &mut f64,
        var_dr_dn10_slot: &mut f64,
        var_dr_dn11_slot: &mut f64,
        var_dr_dn13_slot: &mut f64,
        var_dr_dn14_slot: &mut f64,
        var_dr_dn2_slot: &mut f64,
        var_dr_dn3_slot: &mut f64,
        var_dr_dn4_slot: &mut f64,
        var_dr_dn5_slot: &mut f64,
        var_dr_dn6_slot: &mut f64,
        var_dr_dn7_slot: &mut f64,
        var_dr_dn8_slot: &mut f64,
        var_dr_dn9_slot: &mut f64,
        var_igbacc_v_slot: &mut f64,
        var_igbacc_v_dn0_slot: &mut f64,
        var_igbacc_v_dn10_slot: &mut f64,
        var_igbacc_v_dn11_slot: &mut f64,
        var_igbacc_v_dn13_slot: &mut f64,
        var_igbacc_v_dn14_slot: &mut f64,
        var_igbacc_v_dn2_slot: &mut f64,
        var_igbacc_v_dn3_slot: &mut f64,
        var_igbacc_v_dn4_slot: &mut f64,
        var_igbacc_v_dn5_slot: &mut f64,
        var_igbacc_v_dn6_slot: &mut f64,
        var_igbacc_v_dn7_slot: &mut f64,
        var_igbacc_v_dn8_slot: &mut f64,
        var_igbacc_v_dn9_slot: &mut f64,
        var_igbd_v_slot: &mut f64,
        var_igbd_v_dn0_slot: &mut f64,
        var_igbd_v_dn10_slot: &mut f64,
        var_igbd_v_dn11_slot: &mut f64,
        var_igbd_v_dn13_slot: &mut f64,
        var_igbd_v_dn14_slot: &mut f64,
        var_igbd_v_dn2_slot: &mut f64,
        var_igbd_v_dn3_slot: &mut f64,
        var_igbd_v_dn4_slot: &mut f64,
        var_igbd_v_dn5_slot: &mut f64,
        var_igbd_v_dn6_slot: &mut f64,
        var_igbd_v_dn7_slot: &mut f64,
        var_igbd_v_dn8_slot: &mut f64,
        var_igbd_v_dn9_slot: &mut f64,
        var_igbinv_v_slot: &mut f64,
        var_igbinv_v_dn0_slot: &mut f64,
        var_igbinv_v_dn10_slot: &mut f64,
        var_igbinv_v_dn11_slot: &mut f64,
        var_igbinv_v_dn13_slot: &mut f64,
        var_igbinv_v_dn14_slot: &mut f64,
        var_igbinv_v_dn2_slot: &mut f64,
        var_igbinv_v_dn3_slot: &mut f64,
        var_igbinv_v_dn4_slot: &mut f64,
        var_igbinv_v_dn5_slot: &mut f64,
        var_igbinv_v_dn6_slot: &mut f64,
        var_igbinv_v_dn7_slot: &mut f64,
        var_igbinv_v_dn8_slot: &mut f64,
        var_igbinv_v_dn9_slot: &mut f64,
        var_igbs_v_slot: &mut f64,
        var_igbs_v_dn0_slot: &mut f64,
        var_igbs_v_dn10_slot: &mut f64,
        var_igbs_v_dn11_slot: &mut f64,
        var_igbs_v_dn13_slot: &mut f64,
        var_igbs_v_dn14_slot: &mut f64,
        var_igbs_v_dn2_slot: &mut f64,
        var_igbs_v_dn3_slot: &mut f64,
        var_igbs_v_dn4_slot: &mut f64,
        var_igbs_v_dn5_slot: &mut f64,
        var_igbs_v_dn6_slot: &mut f64,
        var_igbs_v_dn7_slot: &mut f64,
        var_igbs_v_dn8_slot: &mut f64,
        var_igbs_v_dn9_slot: &mut f64,
        var_igcd_v_slot: &mut f64,
        var_igcd_v_dn0_slot: &mut f64,
        var_igcd_v_dn10_slot: &mut f64,
        var_igcd_v_dn11_slot: &mut f64,
        var_igcd_v_dn13_slot: &mut f64,
        var_igcd_v_dn14_slot: &mut f64,
        var_igcd_v_dn2_slot: &mut f64,
        var_igcd_v_dn3_slot: &mut f64,
        var_igcd_v_dn4_slot: &mut f64,
        var_igcd_v_dn5_slot: &mut f64,
        var_igcd_v_dn6_slot: &mut f64,
        var_igcd_v_dn7_slot: &mut f64,
        var_igcd_v_dn8_slot: &mut f64,
        var_igcd_v_dn9_slot: &mut f64,
        var_igcs_v_slot: &mut f64,
        var_igcs_v_dn0_slot: &mut f64,
        var_igcs_v_dn10_slot: &mut f64,
        var_igcs_v_dn11_slot: &mut f64,
        var_igcs_v_dn13_slot: &mut f64,
        var_igcs_v_dn14_slot: &mut f64,
        var_igcs_v_dn2_slot: &mut f64,
        var_igcs_v_dn3_slot: &mut f64,
        var_igcs_v_dn4_slot: &mut f64,
        var_igcs_v_dn5_slot: &mut f64,
        var_igcs_v_dn6_slot: &mut f64,
        var_igcs_v_dn7_slot: &mut f64,
        var_igcs_v_dn8_slot: &mut f64,
        var_igcs_v_dn9_slot: &mut f64,
        var_igd_v_slot: &mut f64,
        var_igd_v_dn0_slot: &mut f64,
        var_igd_v_dn10_slot: &mut f64,
        var_igd_v_dn11_slot: &mut f64,
        var_igd_v_dn13_slot: &mut f64,
        var_igd_v_dn14_slot: &mut f64,
        var_igd_v_dn2_slot: &mut f64,
        var_igd_v_dn3_slot: &mut f64,
        var_igd_v_dn4_slot: &mut f64,
        var_igd_v_dn5_slot: &mut f64,
        var_igd_v_dn6_slot: &mut f64,
        var_igd_v_dn7_slot: &mut f64,
        var_igd_v_dn8_slot: &mut f64,
        var_igd_v_dn9_slot: &mut f64,
        var_igisl_v_slot: &mut f64,
        var_igisl_v_dn0_slot: &mut f64,
        var_igisl_v_dn10_slot: &mut f64,
        var_igisl_v_dn11_slot: &mut f64,
        var_igisl_v_dn13_slot: &mut f64,
        var_igisl_v_dn14_slot: &mut f64,
        var_igisl_v_dn2_slot: &mut f64,
        var_igisl_v_dn3_slot: &mut f64,
        var_igisl_v_dn4_slot: &mut f64,
        var_igisl_v_dn5_slot: &mut f64,
        var_igisl_v_dn6_slot: &mut f64,
        var_igisl_v_dn7_slot: &mut f64,
        var_igisl_v_dn8_slot: &mut f64,
        var_igisl_v_dn9_slot: &mut f64,
        var_igs_v_slot: &mut f64,
        var_igs_v_dn0_slot: &mut f64,
        var_igs_v_dn10_slot: &mut f64,
        var_igs_v_dn11_slot: &mut f64,
        var_igs_v_dn13_slot: &mut f64,
        var_igs_v_dn14_slot: &mut f64,
        var_igs_v_dn2_slot: &mut f64,
        var_igs_v_dn3_slot: &mut f64,
        var_igs_v_dn4_slot: &mut f64,
        var_igs_v_dn5_slot: &mut f64,
        var_igs_v_dn6_slot: &mut f64,
        var_igs_v_dn7_slot: &mut f64,
        var_igs_v_dn8_slot: &mut f64,
        var_igs_v_dn9_slot: &mut f64,
        var_jsd_t_slot: &mut f64,
        var_jsd_t_dn0_slot: &mut f64,
        var_jsd_t_dn10_slot: &mut f64,
        var_jsd_t_dn11_slot: &mut f64,
        var_jsd_t_dn13_slot: &mut f64,
        var_jsd_t_dn14_slot: &mut f64,
        var_jsd_t_dn2_slot: &mut f64,
        var_jsd_t_dn3_slot: &mut f64,
        var_jsd_t_dn4_slot: &mut f64,
        var_jsd_t_dn5_slot: &mut f64,
        var_jsd_t_dn6_slot: &mut f64,
        var_jsd_t_dn7_slot: &mut f64,
        var_jsd_t_dn8_slot: &mut f64,
        var_jsd_t_dn9_slot: &mut f64,
        var_jss_t_slot: &mut f64,
        var_jss_t_dn0_slot: &mut f64,
        var_jss_t_dn10_slot: &mut f64,
        var_jss_t_dn11_slot: &mut f64,
        var_jss_t_dn13_slot: &mut f64,
        var_jss_t_dn14_slot: &mut f64,
        var_jss_t_dn2_slot: &mut f64,
        var_jss_t_dn3_slot: &mut f64,
        var_jss_t_dn4_slot: &mut f64,
        var_jss_t_dn5_slot: &mut f64,
        var_jss_t_dn6_slot: &mut f64,
        var_jss_t_dn7_slot: &mut f64,
        var_jss_t_dn8_slot: &mut f64,
        var_jss_t_dn9_slot: &mut f64,
        var_jswd_t_slot: &mut f64,
        var_jswd_t_dn0_slot: &mut f64,
        var_jswd_t_dn10_slot: &mut f64,
        var_jswd_t_dn11_slot: &mut f64,
        var_jswd_t_dn13_slot: &mut f64,
        var_jswd_t_dn14_slot: &mut f64,
        var_jswd_t_dn2_slot: &mut f64,
        var_jswd_t_dn3_slot: &mut f64,
        var_jswd_t_dn4_slot: &mut f64,
        var_jswd_t_dn5_slot: &mut f64,
        var_jswd_t_dn6_slot: &mut f64,
        var_jswd_t_dn7_slot: &mut f64,
        var_jswd_t_dn8_slot: &mut f64,
        var_jswd_t_dn9_slot: &mut f64,
        var_jswgd_t_slot: &mut f64,
        var_jswgd_t_dn0_slot: &mut f64,
        var_jswgd_t_dn10_slot: &mut f64,
        var_jswgd_t_dn11_slot: &mut f64,
        var_jswgd_t_dn13_slot: &mut f64,
        var_jswgd_t_dn14_slot: &mut f64,
        var_jswgd_t_dn2_slot: &mut f64,
        var_jswgd_t_dn3_slot: &mut f64,
        var_jswgd_t_dn4_slot: &mut f64,
        var_jswgd_t_dn5_slot: &mut f64,
        var_jswgd_t_dn6_slot: &mut f64,
        var_jswgd_t_dn7_slot: &mut f64,
        var_jswgd_t_dn8_slot: &mut f64,
        var_jswgd_t_dn9_slot: &mut f64,
        var_jswgs_t_slot: &mut f64,
        var_jswgs_t_dn0_slot: &mut f64,
        var_jswgs_t_dn10_slot: &mut f64,
        var_jswgs_t_dn11_slot: &mut f64,
        var_jswgs_t_dn13_slot: &mut f64,
        var_jswgs_t_dn14_slot: &mut f64,
        var_jswgs_t_dn2_slot: &mut f64,
        var_jswgs_t_dn3_slot: &mut f64,
        var_jswgs_t_dn4_slot: &mut f64,
        var_jswgs_t_dn5_slot: &mut f64,
        var_jswgs_t_dn6_slot: &mut f64,
        var_jswgs_t_dn7_slot: &mut f64,
        var_jswgs_t_dn8_slot: &mut f64,
        var_jswgs_t_dn9_slot: &mut f64,
        var_jsws_t_slot: &mut f64,
        var_jsws_t_dn0_slot: &mut f64,
        var_jsws_t_dn10_slot: &mut f64,
        var_jsws_t_dn11_slot: &mut f64,
        var_jsws_t_dn13_slot: &mut f64,
        var_jsws_t_dn14_slot: &mut f64,
        var_jsws_t_dn2_slot: &mut f64,
        var_jsws_t_dn3_slot: &mut f64,
        var_jsws_t_dn4_slot: &mut f64,
        var_jsws_t_dn5_slot: &mut f64,
        var_jsws_t_dn6_slot: &mut f64,
        var_jsws_t_dn7_slot: &mut f64,
        var_jsws_t_dn8_slot: &mut f64,
        var_jsws_t_dn9_slot: &mut f64,
        var_jtsd_t_slot: &mut f64,
        var_jtsd_t_dn4_slot: &mut f64,
        var_jtss_t_slot: &mut f64,
        var_jtss_t_dn4_slot: &mut f64,
        var_jtsswd_t_slot: &mut f64,
        var_jtsswd_t_dn4_slot: &mut f64,
        var_jtsswgd_t_slot: &mut f64,
        var_jtsswgd_t_dn4_slot: &mut f64,
        var_jtsswgs_t_slot: &mut f64,
        var_jtsswgs_t_dn4_slot: &mut f64,
        var_jtssws_t_slot: &mut f64,
        var_jtssws_t_dn4_slot: &mut f64,
        var_mexpr_t_slot: &mut f64,
        var_mexpr_t_dn0_slot: &mut f64,
        var_mexpr_t_dn10_slot: &mut f64,
        var_mexpr_t_dn11_slot: &mut f64,
        var_mexpr_t_dn13_slot: &mut f64,
        var_mexpr_t_dn14_slot: &mut f64,
        var_mexpr_t_dn2_slot: &mut f64,
        var_mexpr_t_dn3_slot: &mut f64,
        var_mexpr_t_dn4_slot: &mut f64,
        var_mexpr_t_dn5_slot: &mut f64,
        var_mexpr_t_dn6_slot: &mut f64,
        var_mexpr_t_dn7_slot: &mut f64,
        var_mexpr_t_dn8_slot: &mut f64,
        var_mexpr_t_dn9_slot: &mut f64,
        var_njts_t_slot: &mut f64,
        var_njts_t_dn4_slot: &mut f64,
        var_njtsd_t_slot: &mut f64,
        var_njtsd_t_dn4_slot: &mut f64,
        var_njtssw_t_slot: &mut f64,
        var_njtssw_t_dn4_slot: &mut f64,
        var_njtsswd_t_slot: &mut f64,
        var_njtsswd_t_dn4_slot: &mut f64,
        var_njtsswg_t_slot: &mut f64,
        var_njtsswg_t_dn4_slot: &mut f64,
        var_njtsswgd_t_slot: &mut f64,
        var_njtsswgd_t_dn4_slot: &mut f64,
        var_pbd_t_slot: &mut f64,
        var_pbd_t_dn4_slot: &mut f64,
        var_pbs_t_slot: &mut f64,
        var_pbs_t_dn4_slot: &mut f64,
        var_pbswd_t_slot: &mut f64,
        var_pbswd_t_dn4_slot: &mut f64,
        var_pbswgd_t_slot: &mut f64,
        var_pbswgd_t_dn4_slot: &mut f64,
        var_pbswgs_t_slot: &mut f64,
        var_pbswgs_t_dn4_slot: &mut f64,
        var_pbsws_t_slot: &mut f64,
        var_pbsws_t_dn4_slot: &mut f64,
        var_ptwgr_t_slot: &mut f64,
        var_ptwgr_t_dn0_slot: &mut f64,
        var_ptwgr_t_dn10_slot: &mut f64,
        var_ptwgr_t_dn11_slot: &mut f64,
        var_ptwgr_t_dn13_slot: &mut f64,
        var_ptwgr_t_dn14_slot: &mut f64,
        var_ptwgr_t_dn2_slot: &mut f64,
        var_ptwgr_t_dn3_slot: &mut f64,
        var_ptwgr_t_dn4_slot: &mut f64,
        var_ptwgr_t_dn5_slot: &mut f64,
        var_ptwgr_t_dn6_slot: &mut f64,
        var_ptwgr_t_dn7_slot: &mut f64,
        var_ptwgr_t_dn8_slot: &mut f64,
        var_ptwgr_t_dn9_slot: &mut f64,
        var_qba_slot: &mut f64,
        var_qba_dn0_slot: &mut f64,
        var_qba_dn10_slot: &mut f64,
        var_qba_dn11_slot: &mut f64,
        var_qba_dn13_slot: &mut f64,
        var_qba_dn14_slot: &mut f64,
        var_qba_dn2_slot: &mut f64,
        var_qba_dn3_slot: &mut f64,
        var_qba_dn4_slot: &mut f64,
        var_qba_dn5_slot: &mut f64,
        var_qba_dn6_slot: &mut f64,
        var_qba_dn7_slot: &mut f64,
        var_qba_dn8_slot: &mut f64,
        var_qba_dn9_slot: &mut f64,
        var_rddrr_t_slot: &mut f64,
        var_rddrr_t_dn4_slot: &mut f64,
        var_rdrain_slot: &mut f64,
        var_rdrain_dn0_slot: &mut f64,
        var_rdrain_dn10_slot: &mut f64,
        var_rdrain_dn11_slot: &mut f64,
        var_rdrain_dn13_slot: &mut f64,
        var_rdrain_dn14_slot: &mut f64,
        var_rdrain_dn2_slot: &mut f64,
        var_rdrain_dn3_slot: &mut f64,
        var_rdrain_dn4_slot: &mut f64,
        var_rdrain_dn5_slot: &mut f64,
        var_rdrain_dn6_slot: &mut f64,
        var_rdrain_dn7_slot: &mut f64,
        var_rdrain_dn8_slot: &mut f64,
        var_rdrain_dn9_slot: &mut f64,
        var_rdsi_slot: &mut f64,
        var_rdsi_dn0_slot: &mut f64,
        var_rdsi_dn10_slot: &mut f64,
        var_rdsi_dn11_slot: &mut f64,
        var_rdsi_dn13_slot: &mut f64,
        var_rdsi_dn14_slot: &mut f64,
        var_rdsi_dn2_slot: &mut f64,
        var_rdsi_dn3_slot: &mut f64,
        var_rdsi_dn4_slot: &mut f64,
        var_rdsi_dn5_slot: &mut f64,
        var_rdsi_dn6_slot: &mut f64,
        var_rdsi_dn7_slot: &mut f64,
        var_rdsi_dn8_slot: &mut f64,
        var_rdsi_dn9_slot: &mut f64,
        var_rsdrr_t_slot: &mut f64,
        var_rsdrr_t_dn4_slot: &mut f64,
        var_rsource_slot: &mut f64,
        var_rsource_dn0_slot: &mut f64,
        var_rsource_dn10_slot: &mut f64,
        var_rsource_dn11_slot: &mut f64,
        var_rsource_dn13_slot: &mut f64,
        var_rsource_dn14_slot: &mut f64,
        var_rsource_dn2_slot: &mut f64,
        var_rsource_dn3_slot: &mut f64,
        var_rsource_dn4_slot: &mut f64,
        var_rsource_dn5_slot: &mut f64,
        var_rsource_dn6_slot: &mut f64,
        var_rsource_dn7_slot: &mut f64,
        var_rsource_dn8_slot: &mut f64,
        var_rsource_dn9_slot: &mut f64,
        var_sprt_i_slot: &mut f64,
        var_tcen0_slot: &mut f64,
        var_u0r_v_slot: &mut f64,
        var_u0r_v_dn0_slot: &mut f64,
        var_u0r_v_dn10_slot: &mut f64,
        var_u0r_v_dn11_slot: &mut f64,
        var_u0r_v_dn13_slot: &mut f64,
        var_u0r_v_dn14_slot: &mut f64,
        var_u0r_v_dn2_slot: &mut f64,
        var_u0r_v_dn3_slot: &mut f64,
        var_u0r_v_dn4_slot: &mut f64,
        var_u0r_v_dn5_slot: &mut f64,
        var_u0r_v_dn6_slot: &mut f64,
        var_u0r_v_dn7_slot: &mut f64,
        var_u0r_v_dn8_slot: &mut f64,
        var_u0r_v_dn9_slot: &mut f64,
        var_uar_t_slot: &mut f64,
        var_uar_t_dn0_slot: &mut f64,
        var_uar_t_dn10_slot: &mut f64,
        var_uar_t_dn11_slot: &mut f64,
        var_uar_t_dn13_slot: &mut f64,
        var_uar_t_dn14_slot: &mut f64,
        var_uar_t_dn2_slot: &mut f64,
        var_uar_t_dn3_slot: &mut f64,
        var_uar_t_dn4_slot: &mut f64,
        var_uar_t_dn5_slot: &mut f64,
        var_uar_t_dn6_slot: &mut f64,
        var_uar_t_dn7_slot: &mut f64,
        var_uar_t_dn8_slot: &mut f64,
        var_uar_t_dn9_slot: &mut f64,
        var_uc_t_slot: &mut f64,
        var_uc_t_dn4_slot: &mut f64,
        var_uccv_t_slot: &mut f64,
        var_uccv_t_dn4_slot: &mut f64,
        var_ucr_t_slot: &mut f64,
        var_ucr_t_dn4_slot: &mut f64,
        var_udr_t_slot: &mut f64,
        var_udr_t_dn0_slot: &mut f64,
        var_udr_t_dn10_slot: &mut f64,
        var_udr_t_dn11_slot: &mut f64,
        var_udr_t_dn13_slot: &mut f64,
        var_udr_t_dn14_slot: &mut f64,
        var_udr_t_dn2_slot: &mut f64,
        var_udr_t_dn3_slot: &mut f64,
        var_udr_t_dn4_slot: &mut f64,
        var_udr_t_dn5_slot: &mut f64,
        var_udr_t_dn6_slot: &mut f64,
        var_udr_t_dn7_slot: &mut f64,
        var_udr_t_dn8_slot: &mut f64,
        var_udr_t_dn9_slot: &mut f64,
        var_vsat1r_t_slot: &mut f64,
        var_vsat1r_t_dn0_slot: &mut f64,
        var_vsat1r_t_dn10_slot: &mut f64,
        var_vsat1r_t_dn11_slot: &mut f64,
        var_vsat1r_t_dn13_slot: &mut f64,
        var_vsat1r_t_dn14_slot: &mut f64,
        var_vsat1r_t_dn2_slot: &mut f64,
        var_vsat1r_t_dn3_slot: &mut f64,
        var_vsat1r_t_dn4_slot: &mut f64,
        var_vsat1r_t_dn5_slot: &mut f64,
        var_vsat1r_t_dn6_slot: &mut f64,
        var_vsat1r_t_dn7_slot: &mut f64,
        var_vsat1r_t_dn8_slot: &mut f64,
        var_vsat1r_t_dn9_slot: &mut f64,
        var_vsatr_t_slot: &mut f64,
        var_vsatr_t_dn4_slot: &mut f64,
    ) {
        let mut var_cfr_geo: f64 = *var_cfr_geo_slot;
        let mut var_cfr_geo_dn0: f64 = *var_cfr_geo_dn0_slot;
        let mut var_cfr_geo_dn10: f64 = *var_cfr_geo_dn10_slot;
        let mut var_cfr_geo_dn11: f64 = *var_cfr_geo_dn11_slot;
        let mut var_cfr_geo_dn13: f64 = *var_cfr_geo_dn13_slot;
        let mut var_cfr_geo_dn14: f64 = *var_cfr_geo_dn14_slot;
        let mut var_cfr_geo_dn2: f64 = *var_cfr_geo_dn2_slot;
        let mut var_cfr_geo_dn3: f64 = *var_cfr_geo_dn3_slot;
        let mut var_cfr_geo_dn4: f64 = *var_cfr_geo_dn4_slot;
        let mut var_cfr_geo_dn5: f64 = *var_cfr_geo_dn5_slot;
        let mut var_cfr_geo_dn6: f64 = *var_cfr_geo_dn6_slot;
        let mut var_cfr_geo_dn7: f64 = *var_cfr_geo_dn7_slot;
        let mut var_cfr_geo_dn8: f64 = *var_cfr_geo_dn8_slot;
        let mut var_cfr_geo_dn9: f64 = *var_cfr_geo_dn9_slot;
        let mut var_cjd_t: f64 = *var_cjd_t_slot;
        let mut var_cjd_t_dn4: f64 = *var_cjd_t_dn4_slot;
        let mut var_cjs_t: f64 = *var_cjs_t_slot;
        let mut var_cjs_t_dn4: f64 = *var_cjs_t_dn4_slot;
        let mut var_cjswd_t: f64 = *var_cjswd_t_slot;
        let mut var_cjswd_t_dn4: f64 = *var_cjswd_t_dn4_slot;
        let mut var_cjswgd_t: f64 = *var_cjswgd_t_slot;
        let mut var_cjswgd_t_dn4: f64 = *var_cjswgd_t_dn4_slot;
        let mut var_cjswgs_t: f64 = *var_cjswgs_t_slot;
        let mut var_cjswgs_t_dn4: f64 = *var_cjswgs_t_dn4_slot;
        let mut var_cjsws_t: f64 = *var_cjsws_t_slot;
        let mut var_cjsws_t_dn4: f64 = *var_cjsws_t_dn4_slot;
        let mut var_dr: f64 = *var_dr_slot;
        let mut var_dr_dn0: f64 = *var_dr_dn0_slot;
        let mut var_dr_dn10: f64 = *var_dr_dn10_slot;
        let mut var_dr_dn11: f64 = *var_dr_dn11_slot;
        let mut var_dr_dn13: f64 = *var_dr_dn13_slot;
        let mut var_dr_dn14: f64 = *var_dr_dn14_slot;
        let mut var_dr_dn2: f64 = *var_dr_dn2_slot;
        let mut var_dr_dn3: f64 = *var_dr_dn3_slot;
        let mut var_dr_dn4: f64 = *var_dr_dn4_slot;
        let mut var_dr_dn5: f64 = *var_dr_dn5_slot;
        let mut var_dr_dn6: f64 = *var_dr_dn6_slot;
        let mut var_dr_dn7: f64 = *var_dr_dn7_slot;
        let mut var_dr_dn8: f64 = *var_dr_dn8_slot;
        let mut var_dr_dn9: f64 = *var_dr_dn9_slot;
        let mut var_igbacc_v: f64 = *var_igbacc_v_slot;
        let mut var_igbacc_v_dn0: f64 = *var_igbacc_v_dn0_slot;
        let mut var_igbacc_v_dn10: f64 = *var_igbacc_v_dn10_slot;
        let mut var_igbacc_v_dn11: f64 = *var_igbacc_v_dn11_slot;
        let mut var_igbacc_v_dn13: f64 = *var_igbacc_v_dn13_slot;
        let mut var_igbacc_v_dn14: f64 = *var_igbacc_v_dn14_slot;
        let mut var_igbacc_v_dn2: f64 = *var_igbacc_v_dn2_slot;
        let mut var_igbacc_v_dn3: f64 = *var_igbacc_v_dn3_slot;
        let mut var_igbacc_v_dn4: f64 = *var_igbacc_v_dn4_slot;
        let mut var_igbacc_v_dn5: f64 = *var_igbacc_v_dn5_slot;
        let mut var_igbacc_v_dn6: f64 = *var_igbacc_v_dn6_slot;
        let mut var_igbacc_v_dn7: f64 = *var_igbacc_v_dn7_slot;
        let mut var_igbacc_v_dn8: f64 = *var_igbacc_v_dn8_slot;
        let mut var_igbacc_v_dn9: f64 = *var_igbacc_v_dn9_slot;
        let mut var_igbd_v: f64 = *var_igbd_v_slot;
        let mut var_igbd_v_dn0: f64 = *var_igbd_v_dn0_slot;
        let mut var_igbd_v_dn10: f64 = *var_igbd_v_dn10_slot;
        let mut var_igbd_v_dn11: f64 = *var_igbd_v_dn11_slot;
        let mut var_igbd_v_dn13: f64 = *var_igbd_v_dn13_slot;
        let mut var_igbd_v_dn14: f64 = *var_igbd_v_dn14_slot;
        let mut var_igbd_v_dn2: f64 = *var_igbd_v_dn2_slot;
        let mut var_igbd_v_dn3: f64 = *var_igbd_v_dn3_slot;
        let mut var_igbd_v_dn4: f64 = *var_igbd_v_dn4_slot;
        let mut var_igbd_v_dn5: f64 = *var_igbd_v_dn5_slot;
        let mut var_igbd_v_dn6: f64 = *var_igbd_v_dn6_slot;
        let mut var_igbd_v_dn7: f64 = *var_igbd_v_dn7_slot;
        let mut var_igbd_v_dn8: f64 = *var_igbd_v_dn8_slot;
        let mut var_igbd_v_dn9: f64 = *var_igbd_v_dn9_slot;
        let mut var_igbinv_v: f64 = *var_igbinv_v_slot;
        let mut var_igbinv_v_dn0: f64 = *var_igbinv_v_dn0_slot;
        let mut var_igbinv_v_dn10: f64 = *var_igbinv_v_dn10_slot;
        let mut var_igbinv_v_dn11: f64 = *var_igbinv_v_dn11_slot;
        let mut var_igbinv_v_dn13: f64 = *var_igbinv_v_dn13_slot;
        let mut var_igbinv_v_dn14: f64 = *var_igbinv_v_dn14_slot;
        let mut var_igbinv_v_dn2: f64 = *var_igbinv_v_dn2_slot;
        let mut var_igbinv_v_dn3: f64 = *var_igbinv_v_dn3_slot;
        let mut var_igbinv_v_dn4: f64 = *var_igbinv_v_dn4_slot;
        let mut var_igbinv_v_dn5: f64 = *var_igbinv_v_dn5_slot;
        let mut var_igbinv_v_dn6: f64 = *var_igbinv_v_dn6_slot;
        let mut var_igbinv_v_dn7: f64 = *var_igbinv_v_dn7_slot;
        let mut var_igbinv_v_dn8: f64 = *var_igbinv_v_dn8_slot;
        let mut var_igbinv_v_dn9: f64 = *var_igbinv_v_dn9_slot;
        let mut var_igbs_v: f64 = *var_igbs_v_slot;
        let mut var_igbs_v_dn0: f64 = *var_igbs_v_dn0_slot;
        let mut var_igbs_v_dn10: f64 = *var_igbs_v_dn10_slot;
        let mut var_igbs_v_dn11: f64 = *var_igbs_v_dn11_slot;
        let mut var_igbs_v_dn13: f64 = *var_igbs_v_dn13_slot;
        let mut var_igbs_v_dn14: f64 = *var_igbs_v_dn14_slot;
        let mut var_igbs_v_dn2: f64 = *var_igbs_v_dn2_slot;
        let mut var_igbs_v_dn3: f64 = *var_igbs_v_dn3_slot;
        let mut var_igbs_v_dn4: f64 = *var_igbs_v_dn4_slot;
        let mut var_igbs_v_dn5: f64 = *var_igbs_v_dn5_slot;
        let mut var_igbs_v_dn6: f64 = *var_igbs_v_dn6_slot;
        let mut var_igbs_v_dn7: f64 = *var_igbs_v_dn7_slot;
        let mut var_igbs_v_dn8: f64 = *var_igbs_v_dn8_slot;
        let mut var_igbs_v_dn9: f64 = *var_igbs_v_dn9_slot;
        let mut var_igcd_v: f64 = *var_igcd_v_slot;
        let mut var_igcd_v_dn0: f64 = *var_igcd_v_dn0_slot;
        let mut var_igcd_v_dn10: f64 = *var_igcd_v_dn10_slot;
        let mut var_igcd_v_dn11: f64 = *var_igcd_v_dn11_slot;
        let mut var_igcd_v_dn13: f64 = *var_igcd_v_dn13_slot;
        let mut var_igcd_v_dn14: f64 = *var_igcd_v_dn14_slot;
        let mut var_igcd_v_dn2: f64 = *var_igcd_v_dn2_slot;
        let mut var_igcd_v_dn3: f64 = *var_igcd_v_dn3_slot;
        let mut var_igcd_v_dn4: f64 = *var_igcd_v_dn4_slot;
        let mut var_igcd_v_dn5: f64 = *var_igcd_v_dn5_slot;
        let mut var_igcd_v_dn6: f64 = *var_igcd_v_dn6_slot;
        let mut var_igcd_v_dn7: f64 = *var_igcd_v_dn7_slot;
        let mut var_igcd_v_dn8: f64 = *var_igcd_v_dn8_slot;
        let mut var_igcd_v_dn9: f64 = *var_igcd_v_dn9_slot;
        let mut var_igcs_v: f64 = *var_igcs_v_slot;
        let mut var_igcs_v_dn0: f64 = *var_igcs_v_dn0_slot;
        let mut var_igcs_v_dn10: f64 = *var_igcs_v_dn10_slot;
        let mut var_igcs_v_dn11: f64 = *var_igcs_v_dn11_slot;
        let mut var_igcs_v_dn13: f64 = *var_igcs_v_dn13_slot;
        let mut var_igcs_v_dn14: f64 = *var_igcs_v_dn14_slot;
        let mut var_igcs_v_dn2: f64 = *var_igcs_v_dn2_slot;
        let mut var_igcs_v_dn3: f64 = *var_igcs_v_dn3_slot;
        let mut var_igcs_v_dn4: f64 = *var_igcs_v_dn4_slot;
        let mut var_igcs_v_dn5: f64 = *var_igcs_v_dn5_slot;
        let mut var_igcs_v_dn6: f64 = *var_igcs_v_dn6_slot;
        let mut var_igcs_v_dn7: f64 = *var_igcs_v_dn7_slot;
        let mut var_igcs_v_dn8: f64 = *var_igcs_v_dn8_slot;
        let mut var_igcs_v_dn9: f64 = *var_igcs_v_dn9_slot;
        let mut var_igd_v: f64 = *var_igd_v_slot;
        let mut var_igd_v_dn0: f64 = *var_igd_v_dn0_slot;
        let mut var_igd_v_dn10: f64 = *var_igd_v_dn10_slot;
        let mut var_igd_v_dn11: f64 = *var_igd_v_dn11_slot;
        let mut var_igd_v_dn13: f64 = *var_igd_v_dn13_slot;
        let mut var_igd_v_dn14: f64 = *var_igd_v_dn14_slot;
        let mut var_igd_v_dn2: f64 = *var_igd_v_dn2_slot;
        let mut var_igd_v_dn3: f64 = *var_igd_v_dn3_slot;
        let mut var_igd_v_dn4: f64 = *var_igd_v_dn4_slot;
        let mut var_igd_v_dn5: f64 = *var_igd_v_dn5_slot;
        let mut var_igd_v_dn6: f64 = *var_igd_v_dn6_slot;
        let mut var_igd_v_dn7: f64 = *var_igd_v_dn7_slot;
        let mut var_igd_v_dn8: f64 = *var_igd_v_dn8_slot;
        let mut var_igd_v_dn9: f64 = *var_igd_v_dn9_slot;
        let mut var_igisl_v: f64 = *var_igisl_v_slot;
        let mut var_igisl_v_dn0: f64 = *var_igisl_v_dn0_slot;
        let mut var_igisl_v_dn10: f64 = *var_igisl_v_dn10_slot;
        let mut var_igisl_v_dn11: f64 = *var_igisl_v_dn11_slot;
        let mut var_igisl_v_dn13: f64 = *var_igisl_v_dn13_slot;
        let mut var_igisl_v_dn14: f64 = *var_igisl_v_dn14_slot;
        let mut var_igisl_v_dn2: f64 = *var_igisl_v_dn2_slot;
        let mut var_igisl_v_dn3: f64 = *var_igisl_v_dn3_slot;
        let mut var_igisl_v_dn4: f64 = *var_igisl_v_dn4_slot;
        let mut var_igisl_v_dn5: f64 = *var_igisl_v_dn5_slot;
        let mut var_igisl_v_dn6: f64 = *var_igisl_v_dn6_slot;
        let mut var_igisl_v_dn7: f64 = *var_igisl_v_dn7_slot;
        let mut var_igisl_v_dn8: f64 = *var_igisl_v_dn8_slot;
        let mut var_igisl_v_dn9: f64 = *var_igisl_v_dn9_slot;
        let mut var_igs_v: f64 = *var_igs_v_slot;
        let mut var_igs_v_dn0: f64 = *var_igs_v_dn0_slot;
        let mut var_igs_v_dn10: f64 = *var_igs_v_dn10_slot;
        let mut var_igs_v_dn11: f64 = *var_igs_v_dn11_slot;
        let mut var_igs_v_dn13: f64 = *var_igs_v_dn13_slot;
        let mut var_igs_v_dn14: f64 = *var_igs_v_dn14_slot;
        let mut var_igs_v_dn2: f64 = *var_igs_v_dn2_slot;
        let mut var_igs_v_dn3: f64 = *var_igs_v_dn3_slot;
        let mut var_igs_v_dn4: f64 = *var_igs_v_dn4_slot;
        let mut var_igs_v_dn5: f64 = *var_igs_v_dn5_slot;
        let mut var_igs_v_dn6: f64 = *var_igs_v_dn6_slot;
        let mut var_igs_v_dn7: f64 = *var_igs_v_dn7_slot;
        let mut var_igs_v_dn8: f64 = *var_igs_v_dn8_slot;
        let mut var_igs_v_dn9: f64 = *var_igs_v_dn9_slot;
        let mut var_jsd_t: f64 = *var_jsd_t_slot;
        let mut var_jsd_t_dn0: f64 = *var_jsd_t_dn0_slot;
        let mut var_jsd_t_dn10: f64 = *var_jsd_t_dn10_slot;
        let mut var_jsd_t_dn11: f64 = *var_jsd_t_dn11_slot;
        let mut var_jsd_t_dn13: f64 = *var_jsd_t_dn13_slot;
        let mut var_jsd_t_dn14: f64 = *var_jsd_t_dn14_slot;
        let mut var_jsd_t_dn2: f64 = *var_jsd_t_dn2_slot;
        let mut var_jsd_t_dn3: f64 = *var_jsd_t_dn3_slot;
        let mut var_jsd_t_dn4: f64 = *var_jsd_t_dn4_slot;
        let mut var_jsd_t_dn5: f64 = *var_jsd_t_dn5_slot;
        let mut var_jsd_t_dn6: f64 = *var_jsd_t_dn6_slot;
        let mut var_jsd_t_dn7: f64 = *var_jsd_t_dn7_slot;
        let mut var_jsd_t_dn8: f64 = *var_jsd_t_dn8_slot;
        let mut var_jsd_t_dn9: f64 = *var_jsd_t_dn9_slot;
        let mut var_jss_t: f64 = *var_jss_t_slot;
        let mut var_jss_t_dn0: f64 = *var_jss_t_dn0_slot;
        let mut var_jss_t_dn10: f64 = *var_jss_t_dn10_slot;
        let mut var_jss_t_dn11: f64 = *var_jss_t_dn11_slot;
        let mut var_jss_t_dn13: f64 = *var_jss_t_dn13_slot;
        let mut var_jss_t_dn14: f64 = *var_jss_t_dn14_slot;
        let mut var_jss_t_dn2: f64 = *var_jss_t_dn2_slot;
        let mut var_jss_t_dn3: f64 = *var_jss_t_dn3_slot;
        let mut var_jss_t_dn4: f64 = *var_jss_t_dn4_slot;
        let mut var_jss_t_dn5: f64 = *var_jss_t_dn5_slot;
        let mut var_jss_t_dn6: f64 = *var_jss_t_dn6_slot;
        let mut var_jss_t_dn7: f64 = *var_jss_t_dn7_slot;
        let mut var_jss_t_dn8: f64 = *var_jss_t_dn8_slot;
        let mut var_jss_t_dn9: f64 = *var_jss_t_dn9_slot;
        let mut var_jswd_t: f64 = *var_jswd_t_slot;
        let mut var_jswd_t_dn0: f64 = *var_jswd_t_dn0_slot;
        let mut var_jswd_t_dn10: f64 = *var_jswd_t_dn10_slot;
        let mut var_jswd_t_dn11: f64 = *var_jswd_t_dn11_slot;
        let mut var_jswd_t_dn13: f64 = *var_jswd_t_dn13_slot;
        let mut var_jswd_t_dn14: f64 = *var_jswd_t_dn14_slot;
        let mut var_jswd_t_dn2: f64 = *var_jswd_t_dn2_slot;
        let mut var_jswd_t_dn3: f64 = *var_jswd_t_dn3_slot;
        let mut var_jswd_t_dn4: f64 = *var_jswd_t_dn4_slot;
        let mut var_jswd_t_dn5: f64 = *var_jswd_t_dn5_slot;
        let mut var_jswd_t_dn6: f64 = *var_jswd_t_dn6_slot;
        let mut var_jswd_t_dn7: f64 = *var_jswd_t_dn7_slot;
        let mut var_jswd_t_dn8: f64 = *var_jswd_t_dn8_slot;
        let mut var_jswd_t_dn9: f64 = *var_jswd_t_dn9_slot;
        let mut var_jswgd_t: f64 = *var_jswgd_t_slot;
        let mut var_jswgd_t_dn0: f64 = *var_jswgd_t_dn0_slot;
        let mut var_jswgd_t_dn10: f64 = *var_jswgd_t_dn10_slot;
        let mut var_jswgd_t_dn11: f64 = *var_jswgd_t_dn11_slot;
        let mut var_jswgd_t_dn13: f64 = *var_jswgd_t_dn13_slot;
        let mut var_jswgd_t_dn14: f64 = *var_jswgd_t_dn14_slot;
        let mut var_jswgd_t_dn2: f64 = *var_jswgd_t_dn2_slot;
        let mut var_jswgd_t_dn3: f64 = *var_jswgd_t_dn3_slot;
        let mut var_jswgd_t_dn4: f64 = *var_jswgd_t_dn4_slot;
        let mut var_jswgd_t_dn5: f64 = *var_jswgd_t_dn5_slot;
        let mut var_jswgd_t_dn6: f64 = *var_jswgd_t_dn6_slot;
        let mut var_jswgd_t_dn7: f64 = *var_jswgd_t_dn7_slot;
        let mut var_jswgd_t_dn8: f64 = *var_jswgd_t_dn8_slot;
        let mut var_jswgd_t_dn9: f64 = *var_jswgd_t_dn9_slot;
        let mut var_jswgs_t: f64 = *var_jswgs_t_slot;
        let mut var_jswgs_t_dn0: f64 = *var_jswgs_t_dn0_slot;
        let mut var_jswgs_t_dn10: f64 = *var_jswgs_t_dn10_slot;
        let mut var_jswgs_t_dn11: f64 = *var_jswgs_t_dn11_slot;
        let mut var_jswgs_t_dn13: f64 = *var_jswgs_t_dn13_slot;
        let mut var_jswgs_t_dn14: f64 = *var_jswgs_t_dn14_slot;
        let mut var_jswgs_t_dn2: f64 = *var_jswgs_t_dn2_slot;
        let mut var_jswgs_t_dn3: f64 = *var_jswgs_t_dn3_slot;
        let mut var_jswgs_t_dn4: f64 = *var_jswgs_t_dn4_slot;
        let mut var_jswgs_t_dn5: f64 = *var_jswgs_t_dn5_slot;
        let mut var_jswgs_t_dn6: f64 = *var_jswgs_t_dn6_slot;
        let mut var_jswgs_t_dn7: f64 = *var_jswgs_t_dn7_slot;
        let mut var_jswgs_t_dn8: f64 = *var_jswgs_t_dn8_slot;
        let mut var_jswgs_t_dn9: f64 = *var_jswgs_t_dn9_slot;
        let mut var_jsws_t: f64 = *var_jsws_t_slot;
        let mut var_jsws_t_dn0: f64 = *var_jsws_t_dn0_slot;
        let mut var_jsws_t_dn10: f64 = *var_jsws_t_dn10_slot;
        let mut var_jsws_t_dn11: f64 = *var_jsws_t_dn11_slot;
        let mut var_jsws_t_dn13: f64 = *var_jsws_t_dn13_slot;
        let mut var_jsws_t_dn14: f64 = *var_jsws_t_dn14_slot;
        let mut var_jsws_t_dn2: f64 = *var_jsws_t_dn2_slot;
        let mut var_jsws_t_dn3: f64 = *var_jsws_t_dn3_slot;
        let mut var_jsws_t_dn4: f64 = *var_jsws_t_dn4_slot;
        let mut var_jsws_t_dn5: f64 = *var_jsws_t_dn5_slot;
        let mut var_jsws_t_dn6: f64 = *var_jsws_t_dn6_slot;
        let mut var_jsws_t_dn7: f64 = *var_jsws_t_dn7_slot;
        let mut var_jsws_t_dn8: f64 = *var_jsws_t_dn8_slot;
        let mut var_jsws_t_dn9: f64 = *var_jsws_t_dn9_slot;
        let mut var_jtsd_t: f64 = *var_jtsd_t_slot;
        let mut var_jtsd_t_dn4: f64 = *var_jtsd_t_dn4_slot;
        let mut var_jtss_t: f64 = *var_jtss_t_slot;
        let mut var_jtss_t_dn4: f64 = *var_jtss_t_dn4_slot;
        let mut var_jtsswd_t: f64 = *var_jtsswd_t_slot;
        let mut var_jtsswd_t_dn4: f64 = *var_jtsswd_t_dn4_slot;
        let mut var_jtsswgd_t: f64 = *var_jtsswgd_t_slot;
        let mut var_jtsswgd_t_dn4: f64 = *var_jtsswgd_t_dn4_slot;
        let mut var_jtsswgs_t: f64 = *var_jtsswgs_t_slot;
        let mut var_jtsswgs_t_dn4: f64 = *var_jtsswgs_t_dn4_slot;
        let mut var_jtssws_t: f64 = *var_jtssws_t_slot;
        let mut var_jtssws_t_dn4: f64 = *var_jtssws_t_dn4_slot;
        let mut var_mexpr_t: f64 = *var_mexpr_t_slot;
        let mut var_mexpr_t_dn0: f64 = *var_mexpr_t_dn0_slot;
        let mut var_mexpr_t_dn10: f64 = *var_mexpr_t_dn10_slot;
        let mut var_mexpr_t_dn11: f64 = *var_mexpr_t_dn11_slot;
        let mut var_mexpr_t_dn13: f64 = *var_mexpr_t_dn13_slot;
        let mut var_mexpr_t_dn14: f64 = *var_mexpr_t_dn14_slot;
        let mut var_mexpr_t_dn2: f64 = *var_mexpr_t_dn2_slot;
        let mut var_mexpr_t_dn3: f64 = *var_mexpr_t_dn3_slot;
        let mut var_mexpr_t_dn4: f64 = *var_mexpr_t_dn4_slot;
        let mut var_mexpr_t_dn5: f64 = *var_mexpr_t_dn5_slot;
        let mut var_mexpr_t_dn6: f64 = *var_mexpr_t_dn6_slot;
        let mut var_mexpr_t_dn7: f64 = *var_mexpr_t_dn7_slot;
        let mut var_mexpr_t_dn8: f64 = *var_mexpr_t_dn8_slot;
        let mut var_mexpr_t_dn9: f64 = *var_mexpr_t_dn9_slot;
        let mut var_njts_t: f64 = *var_njts_t_slot;
        let mut var_njts_t_dn4: f64 = *var_njts_t_dn4_slot;
        let mut var_njtsd_t: f64 = *var_njtsd_t_slot;
        let mut var_njtsd_t_dn4: f64 = *var_njtsd_t_dn4_slot;
        let mut var_njtssw_t: f64 = *var_njtssw_t_slot;
        let mut var_njtssw_t_dn4: f64 = *var_njtssw_t_dn4_slot;
        let mut var_njtsswd_t: f64 = *var_njtsswd_t_slot;
        let mut var_njtsswd_t_dn4: f64 = *var_njtsswd_t_dn4_slot;
        let mut var_njtsswg_t: f64 = *var_njtsswg_t_slot;
        let mut var_njtsswg_t_dn4: f64 = *var_njtsswg_t_dn4_slot;
        let mut var_njtsswgd_t: f64 = *var_njtsswgd_t_slot;
        let mut var_njtsswgd_t_dn4: f64 = *var_njtsswgd_t_dn4_slot;
        let mut var_pbd_t: f64 = *var_pbd_t_slot;
        let mut var_pbd_t_dn4: f64 = *var_pbd_t_dn4_slot;
        let mut var_pbs_t: f64 = *var_pbs_t_slot;
        let mut var_pbs_t_dn4: f64 = *var_pbs_t_dn4_slot;
        let mut var_pbswd_t: f64 = *var_pbswd_t_slot;
        let mut var_pbswd_t_dn4: f64 = *var_pbswd_t_dn4_slot;
        let mut var_pbswgd_t: f64 = *var_pbswgd_t_slot;
        let mut var_pbswgd_t_dn4: f64 = *var_pbswgd_t_dn4_slot;
        let mut var_pbswgs_t: f64 = *var_pbswgs_t_slot;
        let mut var_pbswgs_t_dn4: f64 = *var_pbswgs_t_dn4_slot;
        let mut var_pbsws_t: f64 = *var_pbsws_t_slot;
        let mut var_pbsws_t_dn4: f64 = *var_pbsws_t_dn4_slot;
        let mut var_ptwgr_t: f64 = *var_ptwgr_t_slot;
        let mut var_ptwgr_t_dn0: f64 = *var_ptwgr_t_dn0_slot;
        let mut var_ptwgr_t_dn10: f64 = *var_ptwgr_t_dn10_slot;
        let mut var_ptwgr_t_dn11: f64 = *var_ptwgr_t_dn11_slot;
        let mut var_ptwgr_t_dn13: f64 = *var_ptwgr_t_dn13_slot;
        let mut var_ptwgr_t_dn14: f64 = *var_ptwgr_t_dn14_slot;
        let mut var_ptwgr_t_dn2: f64 = *var_ptwgr_t_dn2_slot;
        let mut var_ptwgr_t_dn3: f64 = *var_ptwgr_t_dn3_slot;
        let mut var_ptwgr_t_dn4: f64 = *var_ptwgr_t_dn4_slot;
        let mut var_ptwgr_t_dn5: f64 = *var_ptwgr_t_dn5_slot;
        let mut var_ptwgr_t_dn6: f64 = *var_ptwgr_t_dn6_slot;
        let mut var_ptwgr_t_dn7: f64 = *var_ptwgr_t_dn7_slot;
        let mut var_ptwgr_t_dn8: f64 = *var_ptwgr_t_dn8_slot;
        let mut var_ptwgr_t_dn9: f64 = *var_ptwgr_t_dn9_slot;
        let mut var_qba: f64 = *var_qba_slot;
        let mut var_qba_dn0: f64 = *var_qba_dn0_slot;
        let mut var_qba_dn10: f64 = *var_qba_dn10_slot;
        let mut var_qba_dn11: f64 = *var_qba_dn11_slot;
        let mut var_qba_dn13: f64 = *var_qba_dn13_slot;
        let mut var_qba_dn14: f64 = *var_qba_dn14_slot;
        let mut var_qba_dn2: f64 = *var_qba_dn2_slot;
        let mut var_qba_dn3: f64 = *var_qba_dn3_slot;
        let mut var_qba_dn4: f64 = *var_qba_dn4_slot;
        let mut var_qba_dn5: f64 = *var_qba_dn5_slot;
        let mut var_qba_dn6: f64 = *var_qba_dn6_slot;
        let mut var_qba_dn7: f64 = *var_qba_dn7_slot;
        let mut var_qba_dn8: f64 = *var_qba_dn8_slot;
        let mut var_qba_dn9: f64 = *var_qba_dn9_slot;
        let mut var_rddrr_t: f64 = *var_rddrr_t_slot;
        let mut var_rddrr_t_dn4: f64 = *var_rddrr_t_dn4_slot;
        let mut var_rdrain: f64 = *var_rdrain_slot;
        let mut var_rdrain_dn0: f64 = *var_rdrain_dn0_slot;
        let mut var_rdrain_dn10: f64 = *var_rdrain_dn10_slot;
        let mut var_rdrain_dn11: f64 = *var_rdrain_dn11_slot;
        let mut var_rdrain_dn13: f64 = *var_rdrain_dn13_slot;
        let mut var_rdrain_dn14: f64 = *var_rdrain_dn14_slot;
        let mut var_rdrain_dn2: f64 = *var_rdrain_dn2_slot;
        let mut var_rdrain_dn3: f64 = *var_rdrain_dn3_slot;
        let mut var_rdrain_dn4: f64 = *var_rdrain_dn4_slot;
        let mut var_rdrain_dn5: f64 = *var_rdrain_dn5_slot;
        let mut var_rdrain_dn6: f64 = *var_rdrain_dn6_slot;
        let mut var_rdrain_dn7: f64 = *var_rdrain_dn7_slot;
        let mut var_rdrain_dn8: f64 = *var_rdrain_dn8_slot;
        let mut var_rdrain_dn9: f64 = *var_rdrain_dn9_slot;
        let mut var_rdsi: f64 = *var_rdsi_slot;
        let mut var_rdsi_dn0: f64 = *var_rdsi_dn0_slot;
        let mut var_rdsi_dn10: f64 = *var_rdsi_dn10_slot;
        let mut var_rdsi_dn11: f64 = *var_rdsi_dn11_slot;
        let mut var_rdsi_dn13: f64 = *var_rdsi_dn13_slot;
        let mut var_rdsi_dn14: f64 = *var_rdsi_dn14_slot;
        let mut var_rdsi_dn2: f64 = *var_rdsi_dn2_slot;
        let mut var_rdsi_dn3: f64 = *var_rdsi_dn3_slot;
        let mut var_rdsi_dn4: f64 = *var_rdsi_dn4_slot;
        let mut var_rdsi_dn5: f64 = *var_rdsi_dn5_slot;
        let mut var_rdsi_dn6: f64 = *var_rdsi_dn6_slot;
        let mut var_rdsi_dn7: f64 = *var_rdsi_dn7_slot;
        let mut var_rdsi_dn8: f64 = *var_rdsi_dn8_slot;
        let mut var_rdsi_dn9: f64 = *var_rdsi_dn9_slot;
        let mut var_rsdrr_t: f64 = *var_rsdrr_t_slot;
        let mut var_rsdrr_t_dn4: f64 = *var_rsdrr_t_dn4_slot;
        let mut var_rsource: f64 = *var_rsource_slot;
        let mut var_rsource_dn0: f64 = *var_rsource_dn0_slot;
        let mut var_rsource_dn10: f64 = *var_rsource_dn10_slot;
        let mut var_rsource_dn11: f64 = *var_rsource_dn11_slot;
        let mut var_rsource_dn13: f64 = *var_rsource_dn13_slot;
        let mut var_rsource_dn14: f64 = *var_rsource_dn14_slot;
        let mut var_rsource_dn2: f64 = *var_rsource_dn2_slot;
        let mut var_rsource_dn3: f64 = *var_rsource_dn3_slot;
        let mut var_rsource_dn4: f64 = *var_rsource_dn4_slot;
        let mut var_rsource_dn5: f64 = *var_rsource_dn5_slot;
        let mut var_rsource_dn6: f64 = *var_rsource_dn6_slot;
        let mut var_rsource_dn7: f64 = *var_rsource_dn7_slot;
        let mut var_rsource_dn8: f64 = *var_rsource_dn8_slot;
        let mut var_rsource_dn9: f64 = *var_rsource_dn9_slot;
        let mut var_sprt_i: f64 = *var_sprt_i_slot;
        let mut var_tcen0: f64 = *var_tcen0_slot;
        let mut var_u0r_v: f64 = *var_u0r_v_slot;
        let mut var_u0r_v_dn0: f64 = *var_u0r_v_dn0_slot;
        let mut var_u0r_v_dn10: f64 = *var_u0r_v_dn10_slot;
        let mut var_u0r_v_dn11: f64 = *var_u0r_v_dn11_slot;
        let mut var_u0r_v_dn13: f64 = *var_u0r_v_dn13_slot;
        let mut var_u0r_v_dn14: f64 = *var_u0r_v_dn14_slot;
        let mut var_u0r_v_dn2: f64 = *var_u0r_v_dn2_slot;
        let mut var_u0r_v_dn3: f64 = *var_u0r_v_dn3_slot;
        let mut var_u0r_v_dn4: f64 = *var_u0r_v_dn4_slot;
        let mut var_u0r_v_dn5: f64 = *var_u0r_v_dn5_slot;
        let mut var_u0r_v_dn6: f64 = *var_u0r_v_dn6_slot;
        let mut var_u0r_v_dn7: f64 = *var_u0r_v_dn7_slot;
        let mut var_u0r_v_dn8: f64 = *var_u0r_v_dn8_slot;
        let mut var_u0r_v_dn9: f64 = *var_u0r_v_dn9_slot;
        let mut var_uar_t: f64 = *var_uar_t_slot;
        let mut var_uar_t_dn0: f64 = *var_uar_t_dn0_slot;
        let mut var_uar_t_dn10: f64 = *var_uar_t_dn10_slot;
        let mut var_uar_t_dn11: f64 = *var_uar_t_dn11_slot;
        let mut var_uar_t_dn13: f64 = *var_uar_t_dn13_slot;
        let mut var_uar_t_dn14: f64 = *var_uar_t_dn14_slot;
        let mut var_uar_t_dn2: f64 = *var_uar_t_dn2_slot;
        let mut var_uar_t_dn3: f64 = *var_uar_t_dn3_slot;
        let mut var_uar_t_dn4: f64 = *var_uar_t_dn4_slot;
        let mut var_uar_t_dn5: f64 = *var_uar_t_dn5_slot;
        let mut var_uar_t_dn6: f64 = *var_uar_t_dn6_slot;
        let mut var_uar_t_dn7: f64 = *var_uar_t_dn7_slot;
        let mut var_uar_t_dn8: f64 = *var_uar_t_dn8_slot;
        let mut var_uar_t_dn9: f64 = *var_uar_t_dn9_slot;
        let mut var_uc_t: f64 = *var_uc_t_slot;
        let mut var_uc_t_dn4: f64 = *var_uc_t_dn4_slot;
        let mut var_uccv_t: f64 = *var_uccv_t_slot;
        let mut var_uccv_t_dn4: f64 = *var_uccv_t_dn4_slot;
        let mut var_ucr_t: f64 = *var_ucr_t_slot;
        let mut var_ucr_t_dn4: f64 = *var_ucr_t_dn4_slot;
        let mut var_udr_t: f64 = *var_udr_t_slot;
        let mut var_udr_t_dn0: f64 = *var_udr_t_dn0_slot;
        let mut var_udr_t_dn10: f64 = *var_udr_t_dn10_slot;
        let mut var_udr_t_dn11: f64 = *var_udr_t_dn11_slot;
        let mut var_udr_t_dn13: f64 = *var_udr_t_dn13_slot;
        let mut var_udr_t_dn14: f64 = *var_udr_t_dn14_slot;
        let mut var_udr_t_dn2: f64 = *var_udr_t_dn2_slot;
        let mut var_udr_t_dn3: f64 = *var_udr_t_dn3_slot;
        let mut var_udr_t_dn4: f64 = *var_udr_t_dn4_slot;
        let mut var_udr_t_dn5: f64 = *var_udr_t_dn5_slot;
        let mut var_udr_t_dn6: f64 = *var_udr_t_dn6_slot;
        let mut var_udr_t_dn7: f64 = *var_udr_t_dn7_slot;
        let mut var_udr_t_dn8: f64 = *var_udr_t_dn8_slot;
        let mut var_udr_t_dn9: f64 = *var_udr_t_dn9_slot;
        let mut var_vsat1r_t: f64 = *var_vsat1r_t_slot;
        let mut var_vsat1r_t_dn0: f64 = *var_vsat1r_t_dn0_slot;
        let mut var_vsat1r_t_dn10: f64 = *var_vsat1r_t_dn10_slot;
        let mut var_vsat1r_t_dn11: f64 = *var_vsat1r_t_dn11_slot;
        let mut var_vsat1r_t_dn13: f64 = *var_vsat1r_t_dn13_slot;
        let mut var_vsat1r_t_dn14: f64 = *var_vsat1r_t_dn14_slot;
        let mut var_vsat1r_t_dn2: f64 = *var_vsat1r_t_dn2_slot;
        let mut var_vsat1r_t_dn3: f64 = *var_vsat1r_t_dn3_slot;
        let mut var_vsat1r_t_dn4: f64 = *var_vsat1r_t_dn4_slot;
        let mut var_vsat1r_t_dn5: f64 = *var_vsat1r_t_dn5_slot;
        let mut var_vsat1r_t_dn6: f64 = *var_vsat1r_t_dn6_slot;
        let mut var_vsat1r_t_dn7: f64 = *var_vsat1r_t_dn7_slot;
        let mut var_vsat1r_t_dn8: f64 = *var_vsat1r_t_dn8_slot;
        let mut var_vsat1r_t_dn9: f64 = *var_vsat1r_t_dn9_slot;
        let mut var_vsatr_t: f64 = *var_vsatr_t_slot;
        let mut var_vsatr_t_dn4: f64 = *var_vsatr_t_dn4_slot;

        var_dr = 0.0;
        var_dr_dn0 = 0.0;
        var_dr_dn2 = 0.0;
        var_dr_dn3 = 0.0;
        var_dr_dn4 = 0.0;
        var_dr_dn5 = 0.0;
        var_dr_dn6 = 0.0;
        var_dr_dn7 = 0.0;
        var_dr_dn8 = 0.0;
        var_dr_dn9 = 0.0;
        var_dr_dn10 = 0.0;
        var_dr_dn11 = 0.0;
        var_dr_dn13 = 0.0;
        var_dr_dn14 = 0.0;

        var_rdrain = 0.0;
        var_rdrain_dn0 = 0.0;
        var_rdrain_dn2 = 0.0;
        var_rdrain_dn3 = 0.0;
        var_rdrain_dn4 = 0.0;
        var_rdrain_dn5 = 0.0;
        var_rdrain_dn6 = 0.0;
        var_rdrain_dn7 = 0.0;
        var_rdrain_dn8 = 0.0;
        var_rdrain_dn9 = 0.0;
        var_rdrain_dn10 = 0.0;
        var_rdrain_dn11 = 0.0;
        var_rdrain_dn13 = 0.0;
        var_rdrain_dn14 = 0.0;

        var_rsource = 0.0;
        var_rsource_dn0 = 0.0;
        var_rsource_dn2 = 0.0;
        var_rsource_dn3 = 0.0;
        var_rsource_dn4 = 0.0;
        var_rsource_dn5 = 0.0;
        var_rsource_dn6 = 0.0;
        var_rsource_dn7 = 0.0;
        var_rsource_dn8 = 0.0;
        var_rsource_dn9 = 0.0;
        var_rsource_dn10 = 0.0;
        var_rsource_dn11 = 0.0;
        var_rsource_dn13 = 0.0;
        var_rsource_dn14 = 0.0;

        var_rdsi = 0.0;
        var_rdsi_dn0 = 0.0;
        var_rdsi_dn2 = 0.0;
        var_rdsi_dn3 = 0.0;
        var_rdsi_dn4 = 0.0;
        var_rdsi_dn5 = 0.0;
        var_rdsi_dn6 = 0.0;
        var_rdsi_dn7 = 0.0;
        var_rdsi_dn8 = 0.0;
        var_rdsi_dn9 = 0.0;
        var_rdsi_dn10 = 0.0;
        var_rdsi_dn11 = 0.0;
        var_rdsi_dn13 = 0.0;
        var_rdsi_dn14 = 0.0;

        var_cjs_t = 0.0;
        var_cjs_t_dn4 = 0.0;

        var_cjsws_t = 0.0;
        var_cjsws_t_dn4 = 0.0;

        var_cjswgd_t = 0.0;
        var_cjswgd_t_dn4 = 0.0;

        var_cjd_t = 0.0;
        var_cjd_t_dn4 = 0.0;

        var_cjswd_t = 0.0;
        var_cjswd_t_dn4 = 0.0;

        var_cjswgs_t = 0.0;
        var_cjswgs_t_dn4 = 0.0;

        var_pbs_t = 0.0;
        var_pbs_t_dn4 = 0.0;

        var_pbsws_t = 0.0;
        var_pbsws_t_dn4 = 0.0;

        var_pbswgs_t = 0.0;
        var_pbswgs_t_dn4 = 0.0;

        var_pbd_t = 0.0;
        var_pbd_t_dn4 = 0.0;

        var_pbswd_t = 0.0;
        var_pbswd_t_dn4 = 0.0;

        var_pbswgd_t = 0.0;
        var_pbswgd_t_dn4 = 0.0;

        var_jss_t = 0.0;
        var_jss_t_dn0 = 0.0;
        var_jss_t_dn2 = 0.0;
        var_jss_t_dn3 = 0.0;
        var_jss_t_dn4 = 0.0;
        var_jss_t_dn5 = 0.0;
        var_jss_t_dn6 = 0.0;
        var_jss_t_dn7 = 0.0;
        var_jss_t_dn8 = 0.0;
        var_jss_t_dn9 = 0.0;
        var_jss_t_dn10 = 0.0;
        var_jss_t_dn11 = 0.0;
        var_jss_t_dn13 = 0.0;
        var_jss_t_dn14 = 0.0;

        var_jsws_t = 0.0;
        var_jsws_t_dn0 = 0.0;
        var_jsws_t_dn2 = 0.0;
        var_jsws_t_dn3 = 0.0;
        var_jsws_t_dn4 = 0.0;
        var_jsws_t_dn5 = 0.0;
        var_jsws_t_dn6 = 0.0;
        var_jsws_t_dn7 = 0.0;
        var_jsws_t_dn8 = 0.0;
        var_jsws_t_dn9 = 0.0;
        var_jsws_t_dn10 = 0.0;
        var_jsws_t_dn11 = 0.0;
        var_jsws_t_dn13 = 0.0;
        var_jsws_t_dn14 = 0.0;

        var_jswgs_t = 0.0;
        var_jswgs_t_dn0 = 0.0;
        var_jswgs_t_dn2 = 0.0;
        var_jswgs_t_dn3 = 0.0;
        var_jswgs_t_dn4 = 0.0;
        var_jswgs_t_dn5 = 0.0;
        var_jswgs_t_dn6 = 0.0;
        var_jswgs_t_dn7 = 0.0;
        var_jswgs_t_dn8 = 0.0;
        var_jswgs_t_dn9 = 0.0;
        var_jswgs_t_dn10 = 0.0;
        var_jswgs_t_dn11 = 0.0;
        var_jswgs_t_dn13 = 0.0;
        var_jswgs_t_dn14 = 0.0;

        var_jsd_t = 0.0;
        var_jsd_t_dn0 = 0.0;
        var_jsd_t_dn2 = 0.0;
        var_jsd_t_dn3 = 0.0;
        var_jsd_t_dn4 = 0.0;
        var_jsd_t_dn5 = 0.0;
        var_jsd_t_dn6 = 0.0;
        var_jsd_t_dn7 = 0.0;
        var_jsd_t_dn8 = 0.0;
        var_jsd_t_dn9 = 0.0;
        var_jsd_t_dn10 = 0.0;
        var_jsd_t_dn11 = 0.0;
        var_jsd_t_dn13 = 0.0;
        var_jsd_t_dn14 = 0.0;

        var_jswd_t = 0.0;
        var_jswd_t_dn0 = 0.0;
        var_jswd_t_dn2 = 0.0;
        var_jswd_t_dn3 = 0.0;
        var_jswd_t_dn4 = 0.0;
        var_jswd_t_dn5 = 0.0;
        var_jswd_t_dn6 = 0.0;
        var_jswd_t_dn7 = 0.0;
        var_jswd_t_dn8 = 0.0;
        var_jswd_t_dn9 = 0.0;
        var_jswd_t_dn10 = 0.0;
        var_jswd_t_dn11 = 0.0;
        var_jswd_t_dn13 = 0.0;
        var_jswd_t_dn14 = 0.0;

        var_jswgd_t = 0.0;
        var_jswgd_t_dn0 = 0.0;
        var_jswgd_t_dn2 = 0.0;
        var_jswgd_t_dn3 = 0.0;
        var_jswgd_t_dn4 = 0.0;
        var_jswgd_t_dn5 = 0.0;
        var_jswgd_t_dn6 = 0.0;
        var_jswgd_t_dn7 = 0.0;
        var_jswgd_t_dn8 = 0.0;
        var_jswgd_t_dn9 = 0.0;
        var_jswgd_t_dn10 = 0.0;
        var_jswgd_t_dn11 = 0.0;
        var_jswgd_t_dn13 = 0.0;
        var_jswgd_t_dn14 = 0.0;

        var_jtss_t = 0.0;
        var_jtss_t_dn4 = 0.0;

        var_jtsd_t = 0.0;
        var_jtsd_t_dn4 = 0.0;

        var_jtssws_t = 0.0;
        var_jtssws_t_dn4 = 0.0;

        var_jtsswd_t = 0.0;
        var_jtsswd_t_dn4 = 0.0;

        var_jtsswgs_t = 0.0;
        var_jtsswgs_t_dn4 = 0.0;

        var_jtsswgd_t = 0.0;
        var_jtsswgd_t_dn4 = 0.0;

        var_njts_t = 0.0;
        var_njts_t_dn4 = 0.0;

        var_njtsd_t = 0.0;
        var_njtsd_t_dn4 = 0.0;

        var_njtssw_t = 0.0;
        var_njtssw_t_dn4 = 0.0;

        var_njtsswd_t = 0.0;
        var_njtsswd_t_dn4 = 0.0;

        var_njtsswg_t = 0.0;
        var_njtsswg_t_dn4 = 0.0;

        var_njtsswgd_t = 0.0;
        var_njtsswgd_t_dn4 = 0.0;

        var_rsdrr_t = 0.0;
        var_rsdrr_t_dn4 = 0.0;

        var_rddrr_t = 0.0;
        var_rddrr_t_dn4 = 0.0;

        var_uar_t = 0.0;
        var_uar_t_dn0 = 0.0;
        var_uar_t_dn2 = 0.0;
        var_uar_t_dn3 = 0.0;
        var_uar_t_dn4 = 0.0;
        var_uar_t_dn5 = 0.0;
        var_uar_t_dn6 = 0.0;
        var_uar_t_dn7 = 0.0;
        var_uar_t_dn8 = 0.0;
        var_uar_t_dn9 = 0.0;
        var_uar_t_dn10 = 0.0;
        var_uar_t_dn11 = 0.0;
        var_uar_t_dn13 = 0.0;
        var_uar_t_dn14 = 0.0;

        var_uc_t = 0.0;
        var_uc_t_dn4 = 0.0;

        var_uccv_t = 0.0;
        var_uccv_t_dn4 = 0.0;

        var_ucr_t = 0.0;
        var_ucr_t_dn4 = 0.0;

        var_udr_t = 0.0;
        var_udr_t_dn0 = 0.0;
        var_udr_t_dn2 = 0.0;
        var_udr_t_dn3 = 0.0;
        var_udr_t_dn4 = 0.0;
        var_udr_t_dn5 = 0.0;
        var_udr_t_dn6 = 0.0;
        var_udr_t_dn7 = 0.0;
        var_udr_t_dn8 = 0.0;
        var_udr_t_dn9 = 0.0;
        var_udr_t_dn10 = 0.0;
        var_udr_t_dn11 = 0.0;
        var_udr_t_dn13 = 0.0;
        var_udr_t_dn14 = 0.0;

        var_vsatr_t = 0.0;
        var_vsatr_t_dn4 = 0.0;

        var_vsat1r_t = 0.0;
        var_vsat1r_t_dn0 = 0.0;
        var_vsat1r_t_dn2 = 0.0;
        var_vsat1r_t_dn3 = 0.0;
        var_vsat1r_t_dn4 = 0.0;
        var_vsat1r_t_dn5 = 0.0;
        var_vsat1r_t_dn6 = 0.0;
        var_vsat1r_t_dn7 = 0.0;
        var_vsat1r_t_dn8 = 0.0;
        var_vsat1r_t_dn9 = 0.0;
        var_vsat1r_t_dn10 = 0.0;
        var_vsat1r_t_dn11 = 0.0;
        var_vsat1r_t_dn13 = 0.0;
        var_vsat1r_t_dn14 = 0.0;

        var_mexpr_t = 0.0;
        var_mexpr_t_dn0 = 0.0;
        var_mexpr_t_dn2 = 0.0;
        var_mexpr_t_dn3 = 0.0;
        var_mexpr_t_dn4 = 0.0;
        var_mexpr_t_dn5 = 0.0;
        var_mexpr_t_dn6 = 0.0;
        var_mexpr_t_dn7 = 0.0;
        var_mexpr_t_dn8 = 0.0;
        var_mexpr_t_dn9 = 0.0;
        var_mexpr_t_dn10 = 0.0;
        var_mexpr_t_dn11 = 0.0;
        var_mexpr_t_dn13 = 0.0;
        var_mexpr_t_dn14 = 0.0;

        var_ptwgr_t = 0.0;
        var_ptwgr_t_dn0 = 0.0;
        var_ptwgr_t_dn2 = 0.0;
        var_ptwgr_t_dn3 = 0.0;
        var_ptwgr_t_dn4 = 0.0;
        var_ptwgr_t_dn5 = 0.0;
        var_ptwgr_t_dn6 = 0.0;
        var_ptwgr_t_dn7 = 0.0;
        var_ptwgr_t_dn8 = 0.0;
        var_ptwgr_t_dn9 = 0.0;
        var_ptwgr_t_dn10 = 0.0;
        var_ptwgr_t_dn11 = 0.0;
        var_ptwgr_t_dn13 = 0.0;
        var_ptwgr_t_dn14 = 0.0;

        var_sprt_i = 0.0;

        var_tcen0 = 0.0;

        var_qba = 0.0;
        var_qba_dn0 = 0.0;
        var_qba_dn2 = 0.0;
        var_qba_dn3 = 0.0;
        var_qba_dn4 = 0.0;
        var_qba_dn5 = 0.0;
        var_qba_dn6 = 0.0;
        var_qba_dn7 = 0.0;
        var_qba_dn8 = 0.0;
        var_qba_dn9 = 0.0;
        var_qba_dn10 = 0.0;
        var_qba_dn11 = 0.0;
        var_qba_dn13 = 0.0;
        var_qba_dn14 = 0.0;

        var_u0r_v = 0.0;
        var_u0r_v_dn0 = 0.0;
        var_u0r_v_dn2 = 0.0;
        var_u0r_v_dn3 = 0.0;
        var_u0r_v_dn4 = 0.0;
        var_u0r_v_dn5 = 0.0;
        var_u0r_v_dn6 = 0.0;
        var_u0r_v_dn7 = 0.0;
        var_u0r_v_dn8 = 0.0;
        var_u0r_v_dn9 = 0.0;
        var_u0r_v_dn10 = 0.0;
        var_u0r_v_dn11 = 0.0;
        var_u0r_v_dn13 = 0.0;
        var_u0r_v_dn14 = 0.0;

        var_cfr_geo = 0.0;
        var_cfr_geo_dn0 = 0.0;
        var_cfr_geo_dn2 = 0.0;
        var_cfr_geo_dn3 = 0.0;
        var_cfr_geo_dn4 = 0.0;
        var_cfr_geo_dn5 = 0.0;
        var_cfr_geo_dn6 = 0.0;
        var_cfr_geo_dn7 = 0.0;
        var_cfr_geo_dn8 = 0.0;
        var_cfr_geo_dn9 = 0.0;
        var_cfr_geo_dn10 = 0.0;
        var_cfr_geo_dn11 = 0.0;
        var_cfr_geo_dn13 = 0.0;
        var_cfr_geo_dn14 = 0.0;

        var_igbinv_v = 0.0;
        var_igbinv_v_dn0 = 0.0;
        var_igbinv_v_dn2 = 0.0;
        var_igbinv_v_dn3 = 0.0;
        var_igbinv_v_dn4 = 0.0;
        var_igbinv_v_dn5 = 0.0;
        var_igbinv_v_dn6 = 0.0;
        var_igbinv_v_dn7 = 0.0;
        var_igbinv_v_dn8 = 0.0;
        var_igbinv_v_dn9 = 0.0;
        var_igbinv_v_dn10 = 0.0;
        var_igbinv_v_dn11 = 0.0;
        var_igbinv_v_dn13 = 0.0;
        var_igbinv_v_dn14 = 0.0;

        var_igbacc_v = 0.0;
        var_igbacc_v_dn0 = 0.0;
        var_igbacc_v_dn2 = 0.0;
        var_igbacc_v_dn3 = 0.0;
        var_igbacc_v_dn4 = 0.0;
        var_igbacc_v_dn5 = 0.0;
        var_igbacc_v_dn6 = 0.0;
        var_igbacc_v_dn7 = 0.0;
        var_igbacc_v_dn8 = 0.0;
        var_igbacc_v_dn9 = 0.0;
        var_igbacc_v_dn10 = 0.0;
        var_igbacc_v_dn11 = 0.0;
        var_igbacc_v_dn13 = 0.0;
        var_igbacc_v_dn14 = 0.0;

        var_igbs_v = 0.0;
        var_igbs_v_dn0 = 0.0;
        var_igbs_v_dn2 = 0.0;
        var_igbs_v_dn3 = 0.0;
        var_igbs_v_dn4 = 0.0;
        var_igbs_v_dn5 = 0.0;
        var_igbs_v_dn6 = 0.0;
        var_igbs_v_dn7 = 0.0;
        var_igbs_v_dn8 = 0.0;
        var_igbs_v_dn9 = 0.0;
        var_igbs_v_dn10 = 0.0;
        var_igbs_v_dn11 = 0.0;
        var_igbs_v_dn13 = 0.0;
        var_igbs_v_dn14 = 0.0;

        var_igbd_v = 0.0;
        var_igbd_v_dn0 = 0.0;
        var_igbd_v_dn2 = 0.0;
        var_igbd_v_dn3 = 0.0;
        var_igbd_v_dn4 = 0.0;
        var_igbd_v_dn5 = 0.0;
        var_igbd_v_dn6 = 0.0;
        var_igbd_v_dn7 = 0.0;
        var_igbd_v_dn8 = 0.0;
        var_igbd_v_dn9 = 0.0;
        var_igbd_v_dn10 = 0.0;
        var_igbd_v_dn11 = 0.0;
        var_igbd_v_dn13 = 0.0;
        var_igbd_v_dn14 = 0.0;

        var_igcs_v = 0.0;
        var_igcs_v_dn0 = 0.0;
        var_igcs_v_dn2 = 0.0;
        var_igcs_v_dn3 = 0.0;
        var_igcs_v_dn4 = 0.0;
        var_igcs_v_dn5 = 0.0;
        var_igcs_v_dn6 = 0.0;
        var_igcs_v_dn7 = 0.0;
        var_igcs_v_dn8 = 0.0;
        var_igcs_v_dn9 = 0.0;
        var_igcs_v_dn10 = 0.0;
        var_igcs_v_dn11 = 0.0;
        var_igcs_v_dn13 = 0.0;
        var_igcs_v_dn14 = 0.0;

        var_igcd_v = 0.0;
        var_igcd_v_dn0 = 0.0;
        var_igcd_v_dn2 = 0.0;
        var_igcd_v_dn3 = 0.0;
        var_igcd_v_dn4 = 0.0;
        var_igcd_v_dn5 = 0.0;
        var_igcd_v_dn6 = 0.0;
        var_igcd_v_dn7 = 0.0;
        var_igcd_v_dn8 = 0.0;
        var_igcd_v_dn9 = 0.0;
        var_igcd_v_dn10 = 0.0;
        var_igcd_v_dn11 = 0.0;
        var_igcd_v_dn13 = 0.0;
        var_igcd_v_dn14 = 0.0;

        var_igs_v = 0.0;
        var_igs_v_dn0 = 0.0;
        var_igs_v_dn2 = 0.0;
        var_igs_v_dn3 = 0.0;
        var_igs_v_dn4 = 0.0;
        var_igs_v_dn5 = 0.0;
        var_igs_v_dn6 = 0.0;
        var_igs_v_dn7 = 0.0;
        var_igs_v_dn8 = 0.0;
        var_igs_v_dn9 = 0.0;
        var_igs_v_dn10 = 0.0;
        var_igs_v_dn11 = 0.0;
        var_igs_v_dn13 = 0.0;
        var_igs_v_dn14 = 0.0;

        var_igd_v = 0.0;
        var_igd_v_dn0 = 0.0;
        var_igd_v_dn2 = 0.0;
        var_igd_v_dn3 = 0.0;
        var_igd_v_dn4 = 0.0;
        var_igd_v_dn5 = 0.0;
        var_igd_v_dn6 = 0.0;
        var_igd_v_dn7 = 0.0;
        var_igd_v_dn8 = 0.0;
        var_igd_v_dn9 = 0.0;
        var_igd_v_dn10 = 0.0;
        var_igd_v_dn11 = 0.0;
        var_igd_v_dn13 = 0.0;
        var_igd_v_dn14 = 0.0;

        var_igisl_v = 0.0;
        var_igisl_v_dn0 = 0.0;
        var_igisl_v_dn2 = 0.0;
        var_igisl_v_dn3 = 0.0;
        var_igisl_v_dn4 = 0.0;
        var_igisl_v_dn5 = 0.0;
        var_igisl_v_dn6 = 0.0;
        var_igisl_v_dn7 = 0.0;
        var_igisl_v_dn8 = 0.0;
        var_igisl_v_dn9 = 0.0;
        var_igisl_v_dn10 = 0.0;
        var_igisl_v_dn11 = 0.0;
        var_igisl_v_dn13 = 0.0;
        var_igisl_v_dn14 = 0.0;

        *var_cfr_geo_slot = var_cfr_geo;
        *var_cfr_geo_dn0_slot = var_cfr_geo_dn0;
        *var_cfr_geo_dn10_slot = var_cfr_geo_dn10;
        *var_cfr_geo_dn11_slot = var_cfr_geo_dn11;
        *var_cfr_geo_dn13_slot = var_cfr_geo_dn13;
        *var_cfr_geo_dn14_slot = var_cfr_geo_dn14;
        *var_cfr_geo_dn2_slot = var_cfr_geo_dn2;
        *var_cfr_geo_dn3_slot = var_cfr_geo_dn3;
        *var_cfr_geo_dn4_slot = var_cfr_geo_dn4;
        *var_cfr_geo_dn5_slot = var_cfr_geo_dn5;
        *var_cfr_geo_dn6_slot = var_cfr_geo_dn6;
        *var_cfr_geo_dn7_slot = var_cfr_geo_dn7;
        *var_cfr_geo_dn8_slot = var_cfr_geo_dn8;
        *var_cfr_geo_dn9_slot = var_cfr_geo_dn9;
        *var_cjd_t_slot = var_cjd_t;
        *var_cjd_t_dn4_slot = var_cjd_t_dn4;
        *var_cjs_t_slot = var_cjs_t;
        *var_cjs_t_dn4_slot = var_cjs_t_dn4;
        *var_cjswd_t_slot = var_cjswd_t;
        *var_cjswd_t_dn4_slot = var_cjswd_t_dn4;
        *var_cjswgd_t_slot = var_cjswgd_t;
        *var_cjswgd_t_dn4_slot = var_cjswgd_t_dn4;
        *var_cjswgs_t_slot = var_cjswgs_t;
        *var_cjswgs_t_dn4_slot = var_cjswgs_t_dn4;
        *var_cjsws_t_slot = var_cjsws_t;
        *var_cjsws_t_dn4_slot = var_cjsws_t_dn4;
        *var_dr_slot = var_dr;
        *var_dr_dn0_slot = var_dr_dn0;
        *var_dr_dn10_slot = var_dr_dn10;
        *var_dr_dn11_slot = var_dr_dn11;
        *var_dr_dn13_slot = var_dr_dn13;
        *var_dr_dn14_slot = var_dr_dn14;
        *var_dr_dn2_slot = var_dr_dn2;
        *var_dr_dn3_slot = var_dr_dn3;
        *var_dr_dn4_slot = var_dr_dn4;
        *var_dr_dn5_slot = var_dr_dn5;
        *var_dr_dn6_slot = var_dr_dn6;
        *var_dr_dn7_slot = var_dr_dn7;
        *var_dr_dn8_slot = var_dr_dn8;
        *var_dr_dn9_slot = var_dr_dn9;
        *var_igbacc_v_slot = var_igbacc_v;
        *var_igbacc_v_dn0_slot = var_igbacc_v_dn0;
        *var_igbacc_v_dn10_slot = var_igbacc_v_dn10;
        *var_igbacc_v_dn11_slot = var_igbacc_v_dn11;
        *var_igbacc_v_dn13_slot = var_igbacc_v_dn13;
        *var_igbacc_v_dn14_slot = var_igbacc_v_dn14;
        *var_igbacc_v_dn2_slot = var_igbacc_v_dn2;
        *var_igbacc_v_dn3_slot = var_igbacc_v_dn3;
        *var_igbacc_v_dn4_slot = var_igbacc_v_dn4;
        *var_igbacc_v_dn5_slot = var_igbacc_v_dn5;
        *var_igbacc_v_dn6_slot = var_igbacc_v_dn6;
        *var_igbacc_v_dn7_slot = var_igbacc_v_dn7;
        *var_igbacc_v_dn8_slot = var_igbacc_v_dn8;
        *var_igbacc_v_dn9_slot = var_igbacc_v_dn9;
        *var_igbd_v_slot = var_igbd_v;
        *var_igbd_v_dn0_slot = var_igbd_v_dn0;
        *var_igbd_v_dn10_slot = var_igbd_v_dn10;
        *var_igbd_v_dn11_slot = var_igbd_v_dn11;
        *var_igbd_v_dn13_slot = var_igbd_v_dn13;
        *var_igbd_v_dn14_slot = var_igbd_v_dn14;
        *var_igbd_v_dn2_slot = var_igbd_v_dn2;
        *var_igbd_v_dn3_slot = var_igbd_v_dn3;
        *var_igbd_v_dn4_slot = var_igbd_v_dn4;
        *var_igbd_v_dn5_slot = var_igbd_v_dn5;
        *var_igbd_v_dn6_slot = var_igbd_v_dn6;
        *var_igbd_v_dn7_slot = var_igbd_v_dn7;
        *var_igbd_v_dn8_slot = var_igbd_v_dn8;
        *var_igbd_v_dn9_slot = var_igbd_v_dn9;
        *var_igbinv_v_slot = var_igbinv_v;
        *var_igbinv_v_dn0_slot = var_igbinv_v_dn0;
        *var_igbinv_v_dn10_slot = var_igbinv_v_dn10;
        *var_igbinv_v_dn11_slot = var_igbinv_v_dn11;
        *var_igbinv_v_dn13_slot = var_igbinv_v_dn13;
        *var_igbinv_v_dn14_slot = var_igbinv_v_dn14;
        *var_igbinv_v_dn2_slot = var_igbinv_v_dn2;
        *var_igbinv_v_dn3_slot = var_igbinv_v_dn3;
        *var_igbinv_v_dn4_slot = var_igbinv_v_dn4;
        *var_igbinv_v_dn5_slot = var_igbinv_v_dn5;
        *var_igbinv_v_dn6_slot = var_igbinv_v_dn6;
        *var_igbinv_v_dn7_slot = var_igbinv_v_dn7;
        *var_igbinv_v_dn8_slot = var_igbinv_v_dn8;
        *var_igbinv_v_dn9_slot = var_igbinv_v_dn9;
        *var_igbs_v_slot = var_igbs_v;
        *var_igbs_v_dn0_slot = var_igbs_v_dn0;
        *var_igbs_v_dn10_slot = var_igbs_v_dn10;
        *var_igbs_v_dn11_slot = var_igbs_v_dn11;
        *var_igbs_v_dn13_slot = var_igbs_v_dn13;
        *var_igbs_v_dn14_slot = var_igbs_v_dn14;
        *var_igbs_v_dn2_slot = var_igbs_v_dn2;
        *var_igbs_v_dn3_slot = var_igbs_v_dn3;
        *var_igbs_v_dn4_slot = var_igbs_v_dn4;
        *var_igbs_v_dn5_slot = var_igbs_v_dn5;
        *var_igbs_v_dn6_slot = var_igbs_v_dn6;
        *var_igbs_v_dn7_slot = var_igbs_v_dn7;
        *var_igbs_v_dn8_slot = var_igbs_v_dn8;
        *var_igbs_v_dn9_slot = var_igbs_v_dn9;
        *var_igcd_v_slot = var_igcd_v;
        *var_igcd_v_dn0_slot = var_igcd_v_dn0;
        *var_igcd_v_dn10_slot = var_igcd_v_dn10;
        *var_igcd_v_dn11_slot = var_igcd_v_dn11;
        *var_igcd_v_dn13_slot = var_igcd_v_dn13;
        *var_igcd_v_dn14_slot = var_igcd_v_dn14;
        *var_igcd_v_dn2_slot = var_igcd_v_dn2;
        *var_igcd_v_dn3_slot = var_igcd_v_dn3;
        *var_igcd_v_dn4_slot = var_igcd_v_dn4;
        *var_igcd_v_dn5_slot = var_igcd_v_dn5;
        *var_igcd_v_dn6_slot = var_igcd_v_dn6;
        *var_igcd_v_dn7_slot = var_igcd_v_dn7;
        *var_igcd_v_dn8_slot = var_igcd_v_dn8;
        *var_igcd_v_dn9_slot = var_igcd_v_dn9;
        *var_igcs_v_slot = var_igcs_v;
        *var_igcs_v_dn0_slot = var_igcs_v_dn0;
        *var_igcs_v_dn10_slot = var_igcs_v_dn10;
        *var_igcs_v_dn11_slot = var_igcs_v_dn11;
        *var_igcs_v_dn13_slot = var_igcs_v_dn13;
        *var_igcs_v_dn14_slot = var_igcs_v_dn14;
        *var_igcs_v_dn2_slot = var_igcs_v_dn2;
        *var_igcs_v_dn3_slot = var_igcs_v_dn3;
        *var_igcs_v_dn4_slot = var_igcs_v_dn4;
        *var_igcs_v_dn5_slot = var_igcs_v_dn5;
        *var_igcs_v_dn6_slot = var_igcs_v_dn6;
        *var_igcs_v_dn7_slot = var_igcs_v_dn7;
        *var_igcs_v_dn8_slot = var_igcs_v_dn8;
        *var_igcs_v_dn9_slot = var_igcs_v_dn9;
        *var_igd_v_slot = var_igd_v;
        *var_igd_v_dn0_slot = var_igd_v_dn0;
        *var_igd_v_dn10_slot = var_igd_v_dn10;
        *var_igd_v_dn11_slot = var_igd_v_dn11;
        *var_igd_v_dn13_slot = var_igd_v_dn13;
        *var_igd_v_dn14_slot = var_igd_v_dn14;
        *var_igd_v_dn2_slot = var_igd_v_dn2;
        *var_igd_v_dn3_slot = var_igd_v_dn3;
        *var_igd_v_dn4_slot = var_igd_v_dn4;
        *var_igd_v_dn5_slot = var_igd_v_dn5;
        *var_igd_v_dn6_slot = var_igd_v_dn6;
        *var_igd_v_dn7_slot = var_igd_v_dn7;
        *var_igd_v_dn8_slot = var_igd_v_dn8;
        *var_igd_v_dn9_slot = var_igd_v_dn9;
        *var_igisl_v_slot = var_igisl_v;
        *var_igisl_v_dn0_slot = var_igisl_v_dn0;
        *var_igisl_v_dn10_slot = var_igisl_v_dn10;
        *var_igisl_v_dn11_slot = var_igisl_v_dn11;
        *var_igisl_v_dn13_slot = var_igisl_v_dn13;
        *var_igisl_v_dn14_slot = var_igisl_v_dn14;
        *var_igisl_v_dn2_slot = var_igisl_v_dn2;
        *var_igisl_v_dn3_slot = var_igisl_v_dn3;
        *var_igisl_v_dn4_slot = var_igisl_v_dn4;
        *var_igisl_v_dn5_slot = var_igisl_v_dn5;
        *var_igisl_v_dn6_slot = var_igisl_v_dn6;
        *var_igisl_v_dn7_slot = var_igisl_v_dn7;
        *var_igisl_v_dn8_slot = var_igisl_v_dn8;
        *var_igisl_v_dn9_slot = var_igisl_v_dn9;
        *var_igs_v_slot = var_igs_v;
        *var_igs_v_dn0_slot = var_igs_v_dn0;
        *var_igs_v_dn10_slot = var_igs_v_dn10;
        *var_igs_v_dn11_slot = var_igs_v_dn11;
        *var_igs_v_dn13_slot = var_igs_v_dn13;
        *var_igs_v_dn14_slot = var_igs_v_dn14;
        *var_igs_v_dn2_slot = var_igs_v_dn2;
        *var_igs_v_dn3_slot = var_igs_v_dn3;
        *var_igs_v_dn4_slot = var_igs_v_dn4;
        *var_igs_v_dn5_slot = var_igs_v_dn5;
        *var_igs_v_dn6_slot = var_igs_v_dn6;
        *var_igs_v_dn7_slot = var_igs_v_dn7;
        *var_igs_v_dn8_slot = var_igs_v_dn8;
        *var_igs_v_dn9_slot = var_igs_v_dn9;
        *var_jsd_t_slot = var_jsd_t;
        *var_jsd_t_dn0_slot = var_jsd_t_dn0;
        *var_jsd_t_dn10_slot = var_jsd_t_dn10;
        *var_jsd_t_dn11_slot = var_jsd_t_dn11;
        *var_jsd_t_dn13_slot = var_jsd_t_dn13;
        *var_jsd_t_dn14_slot = var_jsd_t_dn14;
        *var_jsd_t_dn2_slot = var_jsd_t_dn2;
        *var_jsd_t_dn3_slot = var_jsd_t_dn3;
        *var_jsd_t_dn4_slot = var_jsd_t_dn4;
        *var_jsd_t_dn5_slot = var_jsd_t_dn5;
        *var_jsd_t_dn6_slot = var_jsd_t_dn6;
        *var_jsd_t_dn7_slot = var_jsd_t_dn7;
        *var_jsd_t_dn8_slot = var_jsd_t_dn8;
        *var_jsd_t_dn9_slot = var_jsd_t_dn9;
        *var_jss_t_slot = var_jss_t;
        *var_jss_t_dn0_slot = var_jss_t_dn0;
        *var_jss_t_dn10_slot = var_jss_t_dn10;
        *var_jss_t_dn11_slot = var_jss_t_dn11;
        *var_jss_t_dn13_slot = var_jss_t_dn13;
        *var_jss_t_dn14_slot = var_jss_t_dn14;
        *var_jss_t_dn2_slot = var_jss_t_dn2;
        *var_jss_t_dn3_slot = var_jss_t_dn3;
        *var_jss_t_dn4_slot = var_jss_t_dn4;
        *var_jss_t_dn5_slot = var_jss_t_dn5;
        *var_jss_t_dn6_slot = var_jss_t_dn6;
        *var_jss_t_dn7_slot = var_jss_t_dn7;
        *var_jss_t_dn8_slot = var_jss_t_dn8;
        *var_jss_t_dn9_slot = var_jss_t_dn9;
        *var_jswd_t_slot = var_jswd_t;
        *var_jswd_t_dn0_slot = var_jswd_t_dn0;
        *var_jswd_t_dn10_slot = var_jswd_t_dn10;
        *var_jswd_t_dn11_slot = var_jswd_t_dn11;
        *var_jswd_t_dn13_slot = var_jswd_t_dn13;
        *var_jswd_t_dn14_slot = var_jswd_t_dn14;
        *var_jswd_t_dn2_slot = var_jswd_t_dn2;
        *var_jswd_t_dn3_slot = var_jswd_t_dn3;
        *var_jswd_t_dn4_slot = var_jswd_t_dn4;
        *var_jswd_t_dn5_slot = var_jswd_t_dn5;
        *var_jswd_t_dn6_slot = var_jswd_t_dn6;
        *var_jswd_t_dn7_slot = var_jswd_t_dn7;
        *var_jswd_t_dn8_slot = var_jswd_t_dn8;
        *var_jswd_t_dn9_slot = var_jswd_t_dn9;
        *var_jswgd_t_slot = var_jswgd_t;
        *var_jswgd_t_dn0_slot = var_jswgd_t_dn0;
        *var_jswgd_t_dn10_slot = var_jswgd_t_dn10;
        *var_jswgd_t_dn11_slot = var_jswgd_t_dn11;
        *var_jswgd_t_dn13_slot = var_jswgd_t_dn13;
        *var_jswgd_t_dn14_slot = var_jswgd_t_dn14;
        *var_jswgd_t_dn2_slot = var_jswgd_t_dn2;
        *var_jswgd_t_dn3_slot = var_jswgd_t_dn3;
        *var_jswgd_t_dn4_slot = var_jswgd_t_dn4;
        *var_jswgd_t_dn5_slot = var_jswgd_t_dn5;
        *var_jswgd_t_dn6_slot = var_jswgd_t_dn6;
        *var_jswgd_t_dn7_slot = var_jswgd_t_dn7;
        *var_jswgd_t_dn8_slot = var_jswgd_t_dn8;
        *var_jswgd_t_dn9_slot = var_jswgd_t_dn9;
        *var_jswgs_t_slot = var_jswgs_t;
        *var_jswgs_t_dn0_slot = var_jswgs_t_dn0;
        *var_jswgs_t_dn10_slot = var_jswgs_t_dn10;
        *var_jswgs_t_dn11_slot = var_jswgs_t_dn11;
        *var_jswgs_t_dn13_slot = var_jswgs_t_dn13;
        *var_jswgs_t_dn14_slot = var_jswgs_t_dn14;
        *var_jswgs_t_dn2_slot = var_jswgs_t_dn2;
        *var_jswgs_t_dn3_slot = var_jswgs_t_dn3;
        *var_jswgs_t_dn4_slot = var_jswgs_t_dn4;
        *var_jswgs_t_dn5_slot = var_jswgs_t_dn5;
        *var_jswgs_t_dn6_slot = var_jswgs_t_dn6;
        *var_jswgs_t_dn7_slot = var_jswgs_t_dn7;
        *var_jswgs_t_dn8_slot = var_jswgs_t_dn8;
        *var_jswgs_t_dn9_slot = var_jswgs_t_dn9;
        *var_jsws_t_slot = var_jsws_t;
        *var_jsws_t_dn0_slot = var_jsws_t_dn0;
        *var_jsws_t_dn10_slot = var_jsws_t_dn10;
        *var_jsws_t_dn11_slot = var_jsws_t_dn11;
        *var_jsws_t_dn13_slot = var_jsws_t_dn13;
        *var_jsws_t_dn14_slot = var_jsws_t_dn14;
        *var_jsws_t_dn2_slot = var_jsws_t_dn2;
        *var_jsws_t_dn3_slot = var_jsws_t_dn3;
        *var_jsws_t_dn4_slot = var_jsws_t_dn4;
        *var_jsws_t_dn5_slot = var_jsws_t_dn5;
        *var_jsws_t_dn6_slot = var_jsws_t_dn6;
        *var_jsws_t_dn7_slot = var_jsws_t_dn7;
        *var_jsws_t_dn8_slot = var_jsws_t_dn8;
        *var_jsws_t_dn9_slot = var_jsws_t_dn9;
        *var_jtsd_t_slot = var_jtsd_t;
        *var_jtsd_t_dn4_slot = var_jtsd_t_dn4;
        *var_jtss_t_slot = var_jtss_t;
        *var_jtss_t_dn4_slot = var_jtss_t_dn4;
        *var_jtsswd_t_slot = var_jtsswd_t;
        *var_jtsswd_t_dn4_slot = var_jtsswd_t_dn4;
        *var_jtsswgd_t_slot = var_jtsswgd_t;
        *var_jtsswgd_t_dn4_slot = var_jtsswgd_t_dn4;
        *var_jtsswgs_t_slot = var_jtsswgs_t;
        *var_jtsswgs_t_dn4_slot = var_jtsswgs_t_dn4;
        *var_jtssws_t_slot = var_jtssws_t;
        *var_jtssws_t_dn4_slot = var_jtssws_t_dn4;
        *var_mexpr_t_slot = var_mexpr_t;
        *var_mexpr_t_dn0_slot = var_mexpr_t_dn0;
        *var_mexpr_t_dn10_slot = var_mexpr_t_dn10;
        *var_mexpr_t_dn11_slot = var_mexpr_t_dn11;
        *var_mexpr_t_dn13_slot = var_mexpr_t_dn13;
        *var_mexpr_t_dn14_slot = var_mexpr_t_dn14;
        *var_mexpr_t_dn2_slot = var_mexpr_t_dn2;
        *var_mexpr_t_dn3_slot = var_mexpr_t_dn3;
        *var_mexpr_t_dn4_slot = var_mexpr_t_dn4;
        *var_mexpr_t_dn5_slot = var_mexpr_t_dn5;
        *var_mexpr_t_dn6_slot = var_mexpr_t_dn6;
        *var_mexpr_t_dn7_slot = var_mexpr_t_dn7;
        *var_mexpr_t_dn8_slot = var_mexpr_t_dn8;
        *var_mexpr_t_dn9_slot = var_mexpr_t_dn9;
        *var_njts_t_slot = var_njts_t;
        *var_njts_t_dn4_slot = var_njts_t_dn4;
        *var_njtsd_t_slot = var_njtsd_t;
        *var_njtsd_t_dn4_slot = var_njtsd_t_dn4;
        *var_njtssw_t_slot = var_njtssw_t;
        *var_njtssw_t_dn4_slot = var_njtssw_t_dn4;
        *var_njtsswd_t_slot = var_njtsswd_t;
        *var_njtsswd_t_dn4_slot = var_njtsswd_t_dn4;
        *var_njtsswg_t_slot = var_njtsswg_t;
        *var_njtsswg_t_dn4_slot = var_njtsswg_t_dn4;
        *var_njtsswgd_t_slot = var_njtsswgd_t;
        *var_njtsswgd_t_dn4_slot = var_njtsswgd_t_dn4;
        *var_pbd_t_slot = var_pbd_t;
        *var_pbd_t_dn4_slot = var_pbd_t_dn4;
        *var_pbs_t_slot = var_pbs_t;
        *var_pbs_t_dn4_slot = var_pbs_t_dn4;
        *var_pbswd_t_slot = var_pbswd_t;
        *var_pbswd_t_dn4_slot = var_pbswd_t_dn4;
        *var_pbswgd_t_slot = var_pbswgd_t;
        *var_pbswgd_t_dn4_slot = var_pbswgd_t_dn4;
        *var_pbswgs_t_slot = var_pbswgs_t;
        *var_pbswgs_t_dn4_slot = var_pbswgs_t_dn4;
        *var_pbsws_t_slot = var_pbsws_t;
        *var_pbsws_t_dn4_slot = var_pbsws_t_dn4;
        *var_ptwgr_t_slot = var_ptwgr_t;
        *var_ptwgr_t_dn0_slot = var_ptwgr_t_dn0;
        *var_ptwgr_t_dn10_slot = var_ptwgr_t_dn10;
        *var_ptwgr_t_dn11_slot = var_ptwgr_t_dn11;
        *var_ptwgr_t_dn13_slot = var_ptwgr_t_dn13;
        *var_ptwgr_t_dn14_slot = var_ptwgr_t_dn14;
        *var_ptwgr_t_dn2_slot = var_ptwgr_t_dn2;
        *var_ptwgr_t_dn3_slot = var_ptwgr_t_dn3;
        *var_ptwgr_t_dn4_slot = var_ptwgr_t_dn4;
        *var_ptwgr_t_dn5_slot = var_ptwgr_t_dn5;
        *var_ptwgr_t_dn6_slot = var_ptwgr_t_dn6;
        *var_ptwgr_t_dn7_slot = var_ptwgr_t_dn7;
        *var_ptwgr_t_dn8_slot = var_ptwgr_t_dn8;
        *var_ptwgr_t_dn9_slot = var_ptwgr_t_dn9;
        *var_qba_slot = var_qba;
        *var_qba_dn0_slot = var_qba_dn0;
        *var_qba_dn10_slot = var_qba_dn10;
        *var_qba_dn11_slot = var_qba_dn11;
        *var_qba_dn13_slot = var_qba_dn13;
        *var_qba_dn14_slot = var_qba_dn14;
        *var_qba_dn2_slot = var_qba_dn2;
        *var_qba_dn3_slot = var_qba_dn3;
        *var_qba_dn4_slot = var_qba_dn4;
        *var_qba_dn5_slot = var_qba_dn5;
        *var_qba_dn6_slot = var_qba_dn6;
        *var_qba_dn7_slot = var_qba_dn7;
        *var_qba_dn8_slot = var_qba_dn8;
        *var_qba_dn9_slot = var_qba_dn9;
        *var_rddrr_t_slot = var_rddrr_t;
        *var_rddrr_t_dn4_slot = var_rddrr_t_dn4;
        *var_rdrain_slot = var_rdrain;
        *var_rdrain_dn0_slot = var_rdrain_dn0;
        *var_rdrain_dn10_slot = var_rdrain_dn10;
        *var_rdrain_dn11_slot = var_rdrain_dn11;
        *var_rdrain_dn13_slot = var_rdrain_dn13;
        *var_rdrain_dn14_slot = var_rdrain_dn14;
        *var_rdrain_dn2_slot = var_rdrain_dn2;
        *var_rdrain_dn3_slot = var_rdrain_dn3;
        *var_rdrain_dn4_slot = var_rdrain_dn4;
        *var_rdrain_dn5_slot = var_rdrain_dn5;
        *var_rdrain_dn6_slot = var_rdrain_dn6;
        *var_rdrain_dn7_slot = var_rdrain_dn7;
        *var_rdrain_dn8_slot = var_rdrain_dn8;
        *var_rdrain_dn9_slot = var_rdrain_dn9;
        *var_rdsi_slot = var_rdsi;
        *var_rdsi_dn0_slot = var_rdsi_dn0;
        *var_rdsi_dn10_slot = var_rdsi_dn10;
        *var_rdsi_dn11_slot = var_rdsi_dn11;
        *var_rdsi_dn13_slot = var_rdsi_dn13;
        *var_rdsi_dn14_slot = var_rdsi_dn14;
        *var_rdsi_dn2_slot = var_rdsi_dn2;
        *var_rdsi_dn3_slot = var_rdsi_dn3;
        *var_rdsi_dn4_slot = var_rdsi_dn4;
        *var_rdsi_dn5_slot = var_rdsi_dn5;
        *var_rdsi_dn6_slot = var_rdsi_dn6;
        *var_rdsi_dn7_slot = var_rdsi_dn7;
        *var_rdsi_dn8_slot = var_rdsi_dn8;
        *var_rdsi_dn9_slot = var_rdsi_dn9;
        *var_rsdrr_t_slot = var_rsdrr_t;
        *var_rsdrr_t_dn4_slot = var_rsdrr_t_dn4;
        *var_rsource_slot = var_rsource;
        *var_rsource_dn0_slot = var_rsource_dn0;
        *var_rsource_dn10_slot = var_rsource_dn10;
        *var_rsource_dn11_slot = var_rsource_dn11;
        *var_rsource_dn13_slot = var_rsource_dn13;
        *var_rsource_dn14_slot = var_rsource_dn14;
        *var_rsource_dn2_slot = var_rsource_dn2;
        *var_rsource_dn3_slot = var_rsource_dn3;
        *var_rsource_dn4_slot = var_rsource_dn4;
        *var_rsource_dn5_slot = var_rsource_dn5;
        *var_rsource_dn6_slot = var_rsource_dn6;
        *var_rsource_dn7_slot = var_rsource_dn7;
        *var_rsource_dn8_slot = var_rsource_dn8;
        *var_rsource_dn9_slot = var_rsource_dn9;
        *var_sprt_i_slot = var_sprt_i;
        *var_tcen0_slot = var_tcen0;
        *var_u0r_v_slot = var_u0r_v;
        *var_u0r_v_dn0_slot = var_u0r_v_dn0;
        *var_u0r_v_dn10_slot = var_u0r_v_dn10;
        *var_u0r_v_dn11_slot = var_u0r_v_dn11;
        *var_u0r_v_dn13_slot = var_u0r_v_dn13;
        *var_u0r_v_dn14_slot = var_u0r_v_dn14;
        *var_u0r_v_dn2_slot = var_u0r_v_dn2;
        *var_u0r_v_dn3_slot = var_u0r_v_dn3;
        *var_u0r_v_dn4_slot = var_u0r_v_dn4;
        *var_u0r_v_dn5_slot = var_u0r_v_dn5;
        *var_u0r_v_dn6_slot = var_u0r_v_dn6;
        *var_u0r_v_dn7_slot = var_u0r_v_dn7;
        *var_u0r_v_dn8_slot = var_u0r_v_dn8;
        *var_u0r_v_dn9_slot = var_u0r_v_dn9;
        *var_uar_t_slot = var_uar_t;
        *var_uar_t_dn0_slot = var_uar_t_dn0;
        *var_uar_t_dn10_slot = var_uar_t_dn10;
        *var_uar_t_dn11_slot = var_uar_t_dn11;
        *var_uar_t_dn13_slot = var_uar_t_dn13;
        *var_uar_t_dn14_slot = var_uar_t_dn14;
        *var_uar_t_dn2_slot = var_uar_t_dn2;
        *var_uar_t_dn3_slot = var_uar_t_dn3;
        *var_uar_t_dn4_slot = var_uar_t_dn4;
        *var_uar_t_dn5_slot = var_uar_t_dn5;
        *var_uar_t_dn6_slot = var_uar_t_dn6;
        *var_uar_t_dn7_slot = var_uar_t_dn7;
        *var_uar_t_dn8_slot = var_uar_t_dn8;
        *var_uar_t_dn9_slot = var_uar_t_dn9;
        *var_uc_t_slot = var_uc_t;
        *var_uc_t_dn4_slot = var_uc_t_dn4;
        *var_uccv_t_slot = var_uccv_t;
        *var_uccv_t_dn4_slot = var_uccv_t_dn4;
        *var_ucr_t_slot = var_ucr_t;
        *var_ucr_t_dn4_slot = var_ucr_t_dn4;
        *var_udr_t_slot = var_udr_t;
        *var_udr_t_dn0_slot = var_udr_t_dn0;
        *var_udr_t_dn10_slot = var_udr_t_dn10;
        *var_udr_t_dn11_slot = var_udr_t_dn11;
        *var_udr_t_dn13_slot = var_udr_t_dn13;
        *var_udr_t_dn14_slot = var_udr_t_dn14;
        *var_udr_t_dn2_slot = var_udr_t_dn2;
        *var_udr_t_dn3_slot = var_udr_t_dn3;
        *var_udr_t_dn4_slot = var_udr_t_dn4;
        *var_udr_t_dn5_slot = var_udr_t_dn5;
        *var_udr_t_dn6_slot = var_udr_t_dn6;
        *var_udr_t_dn7_slot = var_udr_t_dn7;
        *var_udr_t_dn8_slot = var_udr_t_dn8;
        *var_udr_t_dn9_slot = var_udr_t_dn9;
        *var_vsat1r_t_slot = var_vsat1r_t;
        *var_vsat1r_t_dn0_slot = var_vsat1r_t_dn0;
        *var_vsat1r_t_dn10_slot = var_vsat1r_t_dn10;
        *var_vsat1r_t_dn11_slot = var_vsat1r_t_dn11;
        *var_vsat1r_t_dn13_slot = var_vsat1r_t_dn13;
        *var_vsat1r_t_dn14_slot = var_vsat1r_t_dn14;
        *var_vsat1r_t_dn2_slot = var_vsat1r_t_dn2;
        *var_vsat1r_t_dn3_slot = var_vsat1r_t_dn3;
        *var_vsat1r_t_dn4_slot = var_vsat1r_t_dn4;
        *var_vsat1r_t_dn5_slot = var_vsat1r_t_dn5;
        *var_vsat1r_t_dn6_slot = var_vsat1r_t_dn6;
        *var_vsat1r_t_dn7_slot = var_vsat1r_t_dn7;
        *var_vsat1r_t_dn8_slot = var_vsat1r_t_dn8;
        *var_vsat1r_t_dn9_slot = var_vsat1r_t_dn9;
        *var_vsatr_t_slot = var_vsatr_t;
        *var_vsatr_t_dn4_slot = var_vsatr_t_dn4;
    }

    pub(super) fn stamp_transient_block_1(
        var_agidlb_i_slot: &mut f64,
        var_agislb_i_slot: &mut f64,
        var_bgidlb_i_slot: &mut f64,
        var_bgidlb_t_slot: &mut f64,
        var_bgidlb_t_dn4_slot: &mut f64,
        var_bgislb_i_slot: &mut f64,
        var_bgislb_t_slot: &mut f64,
        var_bgislb_t_dn4_slot: &mut f64,
        var_cgidlb_i_slot: &mut f64,
        var_cgislb_i_slot: &mut f64,
        var_cox_acc_slot: &mut f64,
        var_cox_acc_dn0_slot: &mut f64,
        var_cox_acc_dn10_slot: &mut f64,
        var_cox_acc_dn11_slot: &mut f64,
        var_cox_acc_dn13_slot: &mut f64,
        var_cox_acc_dn14_slot: &mut f64,
        var_cox_acc_dn2_slot: &mut f64,
        var_cox_acc_dn3_slot: &mut f64,
        var_cox_acc_dn4_slot: &mut f64,
        var_cox_acc_dn5_slot: &mut f64,
        var_cox_acc_dn6_slot: &mut f64,
        var_cox_acc_dn7_slot: &mut f64,
        var_cox_acc_dn8_slot: &mut f64,
        var_cox_acc_dn9_slot: &mut f64,
        var_czbd_slot: &mut f64,
        var_czbd_dn4_slot: &mut f64,
        var_czbdsw_slot: &mut f64,
        var_czbdsw_dn4_slot: &mut f64,
        var_czbdswg_slot: &mut f64,
        var_czbdswg_dn4_slot: &mut f64,
        var_czbs_slot: &mut f64,
        var_czbs_dn4_slot: &mut f64,
        var_czbssw_slot: &mut f64,
        var_czbssw_dn4_slot: &mut f64,
        var_czbsswg_slot: &mut f64,
        var_czbsswg_dn4_slot: &mut f64,
        var_dslpfwd_slot: &mut f64,
        var_dslpfwd_dn0_slot: &mut f64,
        var_dslpfwd_dn10_slot: &mut f64,
        var_dslpfwd_dn11_slot: &mut f64,
        var_dslpfwd_dn13_slot: &mut f64,
        var_dslpfwd_dn14_slot: &mut f64,
        var_dslpfwd_dn2_slot: &mut f64,
        var_dslpfwd_dn3_slot: &mut f64,
        var_dslpfwd_dn4_slot: &mut f64,
        var_dslpfwd_dn5_slot: &mut f64,
        var_dslpfwd_dn6_slot: &mut f64,
        var_dslpfwd_dn7_slot: &mut f64,
        var_dslpfwd_dn8_slot: &mut f64,
        var_dslpfwd_dn9_slot: &mut f64,
        var_dslprev_slot: &mut f64,
        var_dslprev_dn0_slot: &mut f64,
        var_dslprev_dn10_slot: &mut f64,
        var_dslprev_dn11_slot: &mut f64,
        var_dslprev_dn13_slot: &mut f64,
        var_dslprev_dn14_slot: &mut f64,
        var_dslprev_dn2_slot: &mut f64,
        var_dslprev_dn3_slot: &mut f64,
        var_dslprev_dn4_slot: &mut f64,
        var_dslprev_dn5_slot: &mut f64,
        var_dslprev_dn6_slot: &mut f64,
        var_dslprev_dn7_slot: &mut f64,
        var_dslprev_dn8_slot: &mut f64,
        var_dslprev_dn9_slot: &mut f64,
        var_egidlb_i_slot: &mut f64,
        var_egislb_i_slot: &mut f64,
        var_gcrg_slot: &mut f64,
        var_gcrg_dn0_slot: &mut f64,
        var_gcrg_dn10_slot: &mut f64,
        var_gcrg_dn11_slot: &mut f64,
        var_gcrg_dn13_slot: &mut f64,
        var_gcrg_dn14_slot: &mut f64,
        var_gcrg_dn2_slot: &mut f64,
        var_gcrg_dn3_slot: &mut f64,
        var_gcrg_dn4_slot: &mut f64,
        var_gcrg_dn5_slot: &mut f64,
        var_gcrg_dn6_slot: &mut f64,
        var_gcrg_dn7_slot: &mut f64,
        var_gcrg_dn8_slot: &mut f64,
        var_gcrg_dn9_slot: &mut f64,
        var_gtau_slot: &mut f64,
        var_gtau_dn0_slot: &mut f64,
        var_gtau_dn10_slot: &mut f64,
        var_gtau_dn11_slot: &mut f64,
        var_gtau_dn13_slot: &mut f64,
        var_gtau_dn14_slot: &mut f64,
        var_gtau_dn2_slot: &mut f64,
        var_gtau_dn3_slot: &mut f64,
        var_gtau_dn4_slot: &mut f64,
        var_gtau_dn5_slot: &mut f64,
        var_gtau_dn6_slot: &mut f64,
        var_gtau_dn7_slot: &mut f64,
        var_gtau_dn8_slot: &mut f64,
        var_gtau_dn9_slot: &mut f64,
        var_ied_slot: &mut f64,
        var_ied_dn0_slot: &mut f64,
        var_ied_dn10_slot: &mut f64,
        var_ied_dn11_slot: &mut f64,
        var_ied_dn13_slot: &mut f64,
        var_ied_dn14_slot: &mut f64,
        var_ied_dn2_slot: &mut f64,
        var_ied_dn3_slot: &mut f64,
        var_ied_dn4_slot: &mut f64,
        var_ied_dn5_slot: &mut f64,
        var_ied_dn6_slot: &mut f64,
        var_ied_dn7_slot: &mut f64,
        var_ied_dn8_slot: &mut f64,
        var_ied_dn9_slot: &mut f64,
        var_ies_slot: &mut f64,
        var_ies_dn0_slot: &mut f64,
        var_ies_dn10_slot: &mut f64,
        var_ies_dn11_slot: &mut f64,
        var_ies_dn13_slot: &mut f64,
        var_ies_dn14_slot: &mut f64,
        var_ies_dn2_slot: &mut f64,
        var_ies_dn3_slot: &mut f64,
        var_ies_dn4_slot: &mut f64,
        var_ies_dn5_slot: &mut f64,
        var_ies_dn6_slot: &mut f64,
        var_ies_dn7_slot: &mut f64,
        var_ies_dn8_slot: &mut f64,
        var_ies_dn9_slot: &mut f64,
        var_igidl_v_slot: &mut f64,
        var_igidl_v_dn0_slot: &mut f64,
        var_igidl_v_dn10_slot: &mut f64,
        var_igidl_v_dn11_slot: &mut f64,
        var_igidl_v_dn13_slot: &mut f64,
        var_igidl_v_dn14_slot: &mut f64,
        var_igidl_v_dn2_slot: &mut f64,
        var_igidl_v_dn3_slot: &mut f64,
        var_igidl_v_dn4_slot: &mut f64,
        var_igidl_v_dn5_slot: &mut f64,
        var_igidl_v_dn6_slot: &mut f64,
        var_igidl_v_dn7_slot: &mut f64,
        var_igidl_v_dn8_slot: &mut f64,
        var_igidl_v_dn9_slot: &mut f64,
        var_igidlb_slot: &mut f64,
        var_igidlb_dn0_slot: &mut f64,
        var_igidlb_dn10_slot: &mut f64,
        var_igidlb_dn11_slot: &mut f64,
        var_igidlb_dn13_slot: &mut f64,
        var_igidlb_dn14_slot: &mut f64,
        var_igidlb_dn2_slot: &mut f64,
        var_igidlb_dn3_slot: &mut f64,
        var_igidlb_dn4_slot: &mut f64,
        var_igidlb_dn5_slot: &mut f64,
        var_igidlb_dn6_slot: &mut f64,
        var_igidlb_dn7_slot: &mut f64,
        var_igidlb_dn8_slot: &mut f64,
        var_igidlb_dn9_slot: &mut f64,
        var_igislb_slot: &mut f64,
        var_igislb_dn0_slot: &mut f64,
        var_igislb_dn10_slot: &mut f64,
        var_igislb_dn11_slot: &mut f64,
        var_igislb_dn13_slot: &mut f64,
        var_igislb_dn14_slot: &mut f64,
        var_igislb_dn2_slot: &mut f64,
        var_igislb_dn3_slot: &mut f64,
        var_igislb_dn4_slot: &mut f64,
        var_igislb_dn5_slot: &mut f64,
        var_igislb_dn6_slot: &mut f64,
        var_igislb_dn7_slot: &mut f64,
        var_igislb_dn8_slot: &mut f64,
        var_igislb_dn9_slot: &mut f64,
        var_iii_1_slot: &mut f64,
        var_iii_1_dn0_slot: &mut f64,
        var_iii_1_dn10_slot: &mut f64,
        var_iii_1_dn11_slot: &mut f64,
        var_iii_1_dn13_slot: &mut f64,
        var_iii_1_dn14_slot: &mut f64,
        var_iii_1_dn2_slot: &mut f64,
        var_iii_1_dn3_slot: &mut f64,
        var_iii_1_dn4_slot: &mut f64,
        var_iii_1_dn5_slot: &mut f64,
        var_iii_1_dn6_slot: &mut f64,
        var_iii_1_dn7_slot: &mut f64,
        var_iii_1_dn8_slot: &mut f64,
        var_iii_1_dn9_slot: &mut f64,
        var_isbd_slot: &mut f64,
        var_isbd_dn0_slot: &mut f64,
        var_isbd_dn10_slot: &mut f64,
        var_isbd_dn11_slot: &mut f64,
        var_isbd_dn13_slot: &mut f64,
        var_isbd_dn14_slot: &mut f64,
        var_isbd_dn2_slot: &mut f64,
        var_isbd_dn3_slot: &mut f64,
        var_isbd_dn4_slot: &mut f64,
        var_isbd_dn5_slot: &mut f64,
        var_isbd_dn6_slot: &mut f64,
        var_isbd_dn7_slot: &mut f64,
        var_isbd_dn8_slot: &mut f64,
        var_isbd_dn9_slot: &mut f64,
        var_isbs_slot: &mut f64,
        var_isbs_dn0_slot: &mut f64,
        var_isbs_dn10_slot: &mut f64,
        var_isbs_dn11_slot: &mut f64,
        var_isbs_dn13_slot: &mut f64,
        var_isbs_dn14_slot: &mut f64,
        var_isbs_dn2_slot: &mut f64,
        var_isbs_dn3_slot: &mut f64,
        var_isbs_dn4_slot: &mut f64,
        var_isbs_dn5_slot: &mut f64,
        var_isbs_dn6_slot: &mut f64,
        var_isbs_dn7_slot: &mut f64,
        var_isbs_dn8_slot: &mut f64,
        var_isbs_dn9_slot: &mut f64,
        var_ivjdmfwd_slot: &mut f64,
        var_ivjdmfwd_dn0_slot: &mut f64,
        var_ivjdmfwd_dn10_slot: &mut f64,
        var_ivjdmfwd_dn11_slot: &mut f64,
        var_ivjdmfwd_dn13_slot: &mut f64,
        var_ivjdmfwd_dn14_slot: &mut f64,
        var_ivjdmfwd_dn2_slot: &mut f64,
        var_ivjdmfwd_dn3_slot: &mut f64,
        var_ivjdmfwd_dn4_slot: &mut f64,
        var_ivjdmfwd_dn5_slot: &mut f64,
        var_ivjdmfwd_dn6_slot: &mut f64,
        var_ivjdmfwd_dn7_slot: &mut f64,
        var_ivjdmfwd_dn8_slot: &mut f64,
        var_ivjdmfwd_dn9_slot: &mut f64,
        var_ivjdmrev_slot: &mut f64,
        var_ivjdmrev_dn0_slot: &mut f64,
        var_ivjdmrev_dn10_slot: &mut f64,
        var_ivjdmrev_dn11_slot: &mut f64,
        var_ivjdmrev_dn13_slot: &mut f64,
        var_ivjdmrev_dn14_slot: &mut f64,
        var_ivjdmrev_dn2_slot: &mut f64,
        var_ivjdmrev_dn3_slot: &mut f64,
        var_ivjdmrev_dn4_slot: &mut f64,
        var_ivjdmrev_dn5_slot: &mut f64,
        var_ivjdmrev_dn6_slot: &mut f64,
        var_ivjdmrev_dn7_slot: &mut f64,
        var_ivjdmrev_dn8_slot: &mut f64,
        var_ivjdmrev_dn9_slot: &mut f64,
        var_ivjsmfwd_slot: &mut f64,
        var_ivjsmfwd_dn0_slot: &mut f64,
        var_ivjsmfwd_dn10_slot: &mut f64,
        var_ivjsmfwd_dn11_slot: &mut f64,
        var_ivjsmfwd_dn13_slot: &mut f64,
        var_ivjsmfwd_dn14_slot: &mut f64,
        var_ivjsmfwd_dn2_slot: &mut f64,
        var_ivjsmfwd_dn3_slot: &mut f64,
        var_ivjsmfwd_dn4_slot: &mut f64,
        var_ivjsmfwd_dn5_slot: &mut f64,
        var_ivjsmfwd_dn6_slot: &mut f64,
        var_ivjsmfwd_dn7_slot: &mut f64,
        var_ivjsmfwd_dn8_slot: &mut f64,
        var_ivjsmfwd_dn9_slot: &mut f64,
        var_ivjsmrev_slot: &mut f64,
        var_ivjsmrev_dn0_slot: &mut f64,
        var_ivjsmrev_dn10_slot: &mut f64,
        var_ivjsmrev_dn11_slot: &mut f64,
        var_ivjsmrev_dn13_slot: &mut f64,
        var_ivjsmrev_dn14_slot: &mut f64,
        var_ivjsmrev_dn2_slot: &mut f64,
        var_ivjsmrev_dn3_slot: &mut f64,
        var_ivjsmrev_dn4_slot: &mut f64,
        var_ivjsmrev_dn5_slot: &mut f64,
        var_ivjsmrev_dn6_slot: &mut f64,
        var_ivjsmrev_dn7_slot: &mut f64,
        var_ivjsmrev_dn8_slot: &mut f64,
        var_ivjsmrev_dn9_slot: &mut f64,
        var_nvtmd_slot: &mut f64,
        var_nvtmd_dn4_slot: &mut f64,
        var_nvtms_slot: &mut f64,
        var_nvtms_dn4_slot: &mut f64,
        var_pb21d_slot: &mut f64,
        var_pb21d_dn4_slot: &mut f64,
        var_pb21s_slot: &mut f64,
        var_pb21s_dn4_slot: &mut f64,
        var_pb22d_slot: &mut f64,
        var_pb22d_dn4_slot: &mut f64,
        var_pb22s_slot: &mut f64,
        var_pb22s_dn4_slot: &mut f64,
        var_pb23d_slot: &mut f64,
        var_pb23d_dn4_slot: &mut f64,
        var_pb23s_slot: &mut f64,
        var_pb23s_dn4_slot: &mut f64,
        var_pgidlb_i_slot: &mut f64,
        var_pgislb_i_slot: &mut f64,
        var_qb_acc_slot: &mut f64,
        var_qb_acc_dn0_slot: &mut f64,
        var_qb_acc_dn10_slot: &mut f64,
        var_qb_acc_dn11_slot: &mut f64,
        var_qb_acc_dn13_slot: &mut f64,
        var_qb_acc_dn14_slot: &mut f64,
        var_qb_acc_dn2_slot: &mut f64,
        var_qb_acc_dn3_slot: &mut f64,
        var_qb_acc_dn4_slot: &mut f64,
        var_qb_acc_dn5_slot: &mut f64,
        var_qb_acc_dn6_slot: &mut f64,
        var_qb_acc_dn7_slot: &mut f64,
        var_qb_acc_dn8_slot: &mut f64,
        var_qb_acc_dn9_slot: &mut f64,
        var_qedj_slot: &mut f64,
        var_qedj_dn3_slot: &mut f64,
        var_qedj_dn4_slot: &mut f64,
        var_qedj_dn5_slot: &mut f64,
        var_qesj_slot: &mut f64,
        var_qesj_dn3_slot: &mut f64,
        var_qesj_dn4_slot: &mut f64,
        var_qesj_dn6_slot: &mut f64,
        var_qg_acc_slot: &mut f64,
        var_qg_acc_dn0_slot: &mut f64,
        var_qg_acc_dn10_slot: &mut f64,
        var_qg_acc_dn11_slot: &mut f64,
        var_qg_acc_dn13_slot: &mut f64,
        var_qg_acc_dn14_slot: &mut f64,
        var_qg_acc_dn2_slot: &mut f64,
        var_qg_acc_dn3_slot: &mut f64,
        var_qg_acc_dn4_slot: &mut f64,
        var_qg_acc_dn5_slot: &mut f64,
        var_qg_acc_dn6_slot: &mut f64,
        var_qg_acc_dn7_slot: &mut f64,
        var_qg_acc_dn8_slot: &mut f64,
        var_qg_acc_dn9_slot: &mut f64,
        var_qgd_ov_slot: &mut f64,
        var_qgd_ov_dn0_slot: &mut f64,
        var_qgd_ov_dn10_slot: &mut f64,
        var_qgd_ov_dn11_slot: &mut f64,
        var_qgd_ov_dn13_slot: &mut f64,
        var_qgd_ov_dn14_slot: &mut f64,
        var_qgd_ov_dn2_slot: &mut f64,
        var_qgd_ov_dn3_slot: &mut f64,
        var_qgd_ov_dn4_slot: &mut f64,
        var_qgd_ov_dn5_slot: &mut f64,
        var_qgd_ov_dn6_slot: &mut f64,
        var_qgd_ov_dn7_slot: &mut f64,
        var_qgd_ov_dn8_slot: &mut f64,
        var_qgd_ov_dn9_slot: &mut f64,
        var_qgs_ov_slot: &mut f64,
        var_qgs_ov_dn0_slot: &mut f64,
        var_qgs_ov_dn10_slot: &mut f64,
        var_qgs_ov_dn11_slot: &mut f64,
        var_qgs_ov_dn13_slot: &mut f64,
        var_qgs_ov_dn14_slot: &mut f64,
        var_qgs_ov_dn2_slot: &mut f64,
        var_qgs_ov_dn3_slot: &mut f64,
        var_qgs_ov_dn4_slot: &mut f64,
        var_qgs_ov_dn5_slot: &mut f64,
        var_qgs_ov_dn6_slot: &mut f64,
        var_qgs_ov_dn7_slot: &mut f64,
        var_qgs_ov_dn8_slot: &mut f64,
        var_qgs_ov_dn9_slot: &mut f64,
        var_sslpfwd_slot: &mut f64,
        var_sslpfwd_dn0_slot: &mut f64,
        var_sslpfwd_dn10_slot: &mut f64,
        var_sslpfwd_dn11_slot: &mut f64,
        var_sslpfwd_dn13_slot: &mut f64,
        var_sslpfwd_dn14_slot: &mut f64,
        var_sslpfwd_dn2_slot: &mut f64,
        var_sslpfwd_dn3_slot: &mut f64,
        var_sslpfwd_dn4_slot: &mut f64,
        var_sslpfwd_dn5_slot: &mut f64,
        var_sslpfwd_dn6_slot: &mut f64,
        var_sslpfwd_dn7_slot: &mut f64,
        var_sslpfwd_dn8_slot: &mut f64,
        var_sslpfwd_dn9_slot: &mut f64,
        var_sslprev_slot: &mut f64,
        var_sslprev_dn0_slot: &mut f64,
        var_sslprev_dn10_slot: &mut f64,
        var_sslprev_dn11_slot: &mut f64,
        var_sslprev_dn13_slot: &mut f64,
        var_sslprev_dn14_slot: &mut f64,
        var_sslprev_dn2_slot: &mut f64,
        var_sslprev_dn3_slot: &mut f64,
        var_sslprev_dn4_slot: &mut f64,
        var_sslprev_dn5_slot: &mut f64,
        var_sslprev_dn6_slot: &mut f64,
        var_sslprev_dn7_slot: &mut f64,
        var_sslprev_dn8_slot: &mut f64,
        var_sslprev_dn9_slot: &mut f64,
        var_vec1d_slot: &mut f64,
        var_vec1d_dn4_slot: &mut f64,
        var_vec1s_slot: &mut f64,
        var_vec1s_dn4_slot: &mut f64,
        var_vec2d_slot: &mut f64,
        var_vec2d_dn4_slot: &mut f64,
        var_vec2s_slot: &mut f64,
        var_vec2s_dn4_slot: &mut f64,
        var_vec3d_slot: &mut f64,
        var_vec3d_dn4_slot: &mut f64,
        var_vec3s_slot: &mut f64,
        var_vec3s_dn4_slot: &mut f64,
        var_vjdmfwd_slot: &mut f64,
        var_vjdmfwd_dn0_slot: &mut f64,
        var_vjdmfwd_dn10_slot: &mut f64,
        var_vjdmfwd_dn11_slot: &mut f64,
        var_vjdmfwd_dn13_slot: &mut f64,
        var_vjdmfwd_dn14_slot: &mut f64,
        var_vjdmfwd_dn2_slot: &mut f64,
        var_vjdmfwd_dn3_slot: &mut f64,
        var_vjdmfwd_dn4_slot: &mut f64,
        var_vjdmfwd_dn5_slot: &mut f64,
        var_vjdmfwd_dn6_slot: &mut f64,
        var_vjdmfwd_dn7_slot: &mut f64,
        var_vjdmfwd_dn8_slot: &mut f64,
        var_vjdmfwd_dn9_slot: &mut f64,
        var_vjdmrev_slot: &mut f64,
        var_vjdmrev_dn0_slot: &mut f64,
        var_vjdmrev_dn10_slot: &mut f64,
        var_vjdmrev_dn11_slot: &mut f64,
        var_vjdmrev_dn13_slot: &mut f64,
        var_vjdmrev_dn14_slot: &mut f64,
        var_vjdmrev_dn2_slot: &mut f64,
        var_vjdmrev_dn3_slot: &mut f64,
        var_vjdmrev_dn4_slot: &mut f64,
        var_vjdmrev_dn5_slot: &mut f64,
        var_vjdmrev_dn6_slot: &mut f64,
        var_vjdmrev_dn7_slot: &mut f64,
        var_vjdmrev_dn8_slot: &mut f64,
        var_vjdmrev_dn9_slot: &mut f64,
        var_vjsmfwd_slot: &mut f64,
        var_vjsmfwd_dn0_slot: &mut f64,
        var_vjsmfwd_dn10_slot: &mut f64,
        var_vjsmfwd_dn11_slot: &mut f64,
        var_vjsmfwd_dn13_slot: &mut f64,
        var_vjsmfwd_dn14_slot: &mut f64,
        var_vjsmfwd_dn2_slot: &mut f64,
        var_vjsmfwd_dn3_slot: &mut f64,
        var_vjsmfwd_dn4_slot: &mut f64,
        var_vjsmfwd_dn5_slot: &mut f64,
        var_vjsmfwd_dn6_slot: &mut f64,
        var_vjsmfwd_dn7_slot: &mut f64,
        var_vjsmfwd_dn8_slot: &mut f64,
        var_vjsmfwd_dn9_slot: &mut f64,
        var_vjsmrev_slot: &mut f64,
        var_vjsmrev_dn0_slot: &mut f64,
        var_vjsmrev_dn10_slot: &mut f64,
        var_vjsmrev_dn11_slot: &mut f64,
        var_vjsmrev_dn13_slot: &mut f64,
        var_vjsmrev_dn14_slot: &mut f64,
        var_vjsmrev_dn2_slot: &mut f64,
        var_vjsmrev_dn3_slot: &mut f64,
        var_vjsmrev_dn4_slot: &mut f64,
        var_vjsmrev_dn5_slot: &mut f64,
        var_vjsmrev_dn6_slot: &mut f64,
        var_vjsmrev_dn7_slot: &mut f64,
        var_vjsmrev_dn8_slot: &mut f64,
        var_vjsmrev_dn9_slot: &mut f64,
        var_xexpbvd_slot: &mut f64,
        var_xexpbvd_dn4_slot: &mut f64,
        var_xexpbvs_slot: &mut f64,
        var_xexpbvs_dn4_slot: &mut f64,
    ) {
        let mut var_agidlb_i: f64 = *var_agidlb_i_slot;
        let mut var_agislb_i: f64 = *var_agislb_i_slot;
        let mut var_bgidlb_i: f64 = *var_bgidlb_i_slot;
        let mut var_bgidlb_t: f64 = *var_bgidlb_t_slot;
        let mut var_bgidlb_t_dn4: f64 = *var_bgidlb_t_dn4_slot;
        let mut var_bgislb_i: f64 = *var_bgislb_i_slot;
        let mut var_bgislb_t: f64 = *var_bgislb_t_slot;
        let mut var_bgislb_t_dn4: f64 = *var_bgislb_t_dn4_slot;
        let mut var_cgidlb_i: f64 = *var_cgidlb_i_slot;
        let mut var_cgislb_i: f64 = *var_cgislb_i_slot;
        let mut var_cox_acc: f64 = *var_cox_acc_slot;
        let mut var_cox_acc_dn0: f64 = *var_cox_acc_dn0_slot;
        let mut var_cox_acc_dn10: f64 = *var_cox_acc_dn10_slot;
        let mut var_cox_acc_dn11: f64 = *var_cox_acc_dn11_slot;
        let mut var_cox_acc_dn13: f64 = *var_cox_acc_dn13_slot;
        let mut var_cox_acc_dn14: f64 = *var_cox_acc_dn14_slot;
        let mut var_cox_acc_dn2: f64 = *var_cox_acc_dn2_slot;
        let mut var_cox_acc_dn3: f64 = *var_cox_acc_dn3_slot;
        let mut var_cox_acc_dn4: f64 = *var_cox_acc_dn4_slot;
        let mut var_cox_acc_dn5: f64 = *var_cox_acc_dn5_slot;
        let mut var_cox_acc_dn6: f64 = *var_cox_acc_dn6_slot;
        let mut var_cox_acc_dn7: f64 = *var_cox_acc_dn7_slot;
        let mut var_cox_acc_dn8: f64 = *var_cox_acc_dn8_slot;
        let mut var_cox_acc_dn9: f64 = *var_cox_acc_dn9_slot;
        let mut var_czbd: f64 = *var_czbd_slot;
        let mut var_czbd_dn4: f64 = *var_czbd_dn4_slot;
        let mut var_czbdsw: f64 = *var_czbdsw_slot;
        let mut var_czbdsw_dn4: f64 = *var_czbdsw_dn4_slot;
        let mut var_czbdswg: f64 = *var_czbdswg_slot;
        let mut var_czbdswg_dn4: f64 = *var_czbdswg_dn4_slot;
        let mut var_czbs: f64 = *var_czbs_slot;
        let mut var_czbs_dn4: f64 = *var_czbs_dn4_slot;
        let mut var_czbssw: f64 = *var_czbssw_slot;
        let mut var_czbssw_dn4: f64 = *var_czbssw_dn4_slot;
        let mut var_czbsswg: f64 = *var_czbsswg_slot;
        let mut var_czbsswg_dn4: f64 = *var_czbsswg_dn4_slot;
        let mut var_dslpfwd: f64 = *var_dslpfwd_slot;
        let mut var_dslpfwd_dn0: f64 = *var_dslpfwd_dn0_slot;
        let mut var_dslpfwd_dn10: f64 = *var_dslpfwd_dn10_slot;
        let mut var_dslpfwd_dn11: f64 = *var_dslpfwd_dn11_slot;
        let mut var_dslpfwd_dn13: f64 = *var_dslpfwd_dn13_slot;
        let mut var_dslpfwd_dn14: f64 = *var_dslpfwd_dn14_slot;
        let mut var_dslpfwd_dn2: f64 = *var_dslpfwd_dn2_slot;
        let mut var_dslpfwd_dn3: f64 = *var_dslpfwd_dn3_slot;
        let mut var_dslpfwd_dn4: f64 = *var_dslpfwd_dn4_slot;
        let mut var_dslpfwd_dn5: f64 = *var_dslpfwd_dn5_slot;
        let mut var_dslpfwd_dn6: f64 = *var_dslpfwd_dn6_slot;
        let mut var_dslpfwd_dn7: f64 = *var_dslpfwd_dn7_slot;
        let mut var_dslpfwd_dn8: f64 = *var_dslpfwd_dn8_slot;
        let mut var_dslpfwd_dn9: f64 = *var_dslpfwd_dn9_slot;
        let mut var_dslprev: f64 = *var_dslprev_slot;
        let mut var_dslprev_dn0: f64 = *var_dslprev_dn0_slot;
        let mut var_dslprev_dn10: f64 = *var_dslprev_dn10_slot;
        let mut var_dslprev_dn11: f64 = *var_dslprev_dn11_slot;
        let mut var_dslprev_dn13: f64 = *var_dslprev_dn13_slot;
        let mut var_dslprev_dn14: f64 = *var_dslprev_dn14_slot;
        let mut var_dslprev_dn2: f64 = *var_dslprev_dn2_slot;
        let mut var_dslprev_dn3: f64 = *var_dslprev_dn3_slot;
        let mut var_dslprev_dn4: f64 = *var_dslprev_dn4_slot;
        let mut var_dslprev_dn5: f64 = *var_dslprev_dn5_slot;
        let mut var_dslprev_dn6: f64 = *var_dslprev_dn6_slot;
        let mut var_dslprev_dn7: f64 = *var_dslprev_dn7_slot;
        let mut var_dslprev_dn8: f64 = *var_dslprev_dn8_slot;
        let mut var_dslprev_dn9: f64 = *var_dslprev_dn9_slot;
        let mut var_egidlb_i: f64 = *var_egidlb_i_slot;
        let mut var_egislb_i: f64 = *var_egislb_i_slot;
        let mut var_gcrg: f64 = *var_gcrg_slot;
        let mut var_gcrg_dn0: f64 = *var_gcrg_dn0_slot;
        let mut var_gcrg_dn10: f64 = *var_gcrg_dn10_slot;
        let mut var_gcrg_dn11: f64 = *var_gcrg_dn11_slot;
        let mut var_gcrg_dn13: f64 = *var_gcrg_dn13_slot;
        let mut var_gcrg_dn14: f64 = *var_gcrg_dn14_slot;
        let mut var_gcrg_dn2: f64 = *var_gcrg_dn2_slot;
        let mut var_gcrg_dn3: f64 = *var_gcrg_dn3_slot;
        let mut var_gcrg_dn4: f64 = *var_gcrg_dn4_slot;
        let mut var_gcrg_dn5: f64 = *var_gcrg_dn5_slot;
        let mut var_gcrg_dn6: f64 = *var_gcrg_dn6_slot;
        let mut var_gcrg_dn7: f64 = *var_gcrg_dn7_slot;
        let mut var_gcrg_dn8: f64 = *var_gcrg_dn8_slot;
        let mut var_gcrg_dn9: f64 = *var_gcrg_dn9_slot;
        let mut var_gtau: f64 = *var_gtau_slot;
        let mut var_gtau_dn0: f64 = *var_gtau_dn0_slot;
        let mut var_gtau_dn10: f64 = *var_gtau_dn10_slot;
        let mut var_gtau_dn11: f64 = *var_gtau_dn11_slot;
        let mut var_gtau_dn13: f64 = *var_gtau_dn13_slot;
        let mut var_gtau_dn14: f64 = *var_gtau_dn14_slot;
        let mut var_gtau_dn2: f64 = *var_gtau_dn2_slot;
        let mut var_gtau_dn3: f64 = *var_gtau_dn3_slot;
        let mut var_gtau_dn4: f64 = *var_gtau_dn4_slot;
        let mut var_gtau_dn5: f64 = *var_gtau_dn5_slot;
        let mut var_gtau_dn6: f64 = *var_gtau_dn6_slot;
        let mut var_gtau_dn7: f64 = *var_gtau_dn7_slot;
        let mut var_gtau_dn8: f64 = *var_gtau_dn8_slot;
        let mut var_gtau_dn9: f64 = *var_gtau_dn9_slot;
        let mut var_ied: f64 = *var_ied_slot;
        let mut var_ied_dn0: f64 = *var_ied_dn0_slot;
        let mut var_ied_dn10: f64 = *var_ied_dn10_slot;
        let mut var_ied_dn11: f64 = *var_ied_dn11_slot;
        let mut var_ied_dn13: f64 = *var_ied_dn13_slot;
        let mut var_ied_dn14: f64 = *var_ied_dn14_slot;
        let mut var_ied_dn2: f64 = *var_ied_dn2_slot;
        let mut var_ied_dn3: f64 = *var_ied_dn3_slot;
        let mut var_ied_dn4: f64 = *var_ied_dn4_slot;
        let mut var_ied_dn5: f64 = *var_ied_dn5_slot;
        let mut var_ied_dn6: f64 = *var_ied_dn6_slot;
        let mut var_ied_dn7: f64 = *var_ied_dn7_slot;
        let mut var_ied_dn8: f64 = *var_ied_dn8_slot;
        let mut var_ied_dn9: f64 = *var_ied_dn9_slot;
        let mut var_ies: f64 = *var_ies_slot;
        let mut var_ies_dn0: f64 = *var_ies_dn0_slot;
        let mut var_ies_dn10: f64 = *var_ies_dn10_slot;
        let mut var_ies_dn11: f64 = *var_ies_dn11_slot;
        let mut var_ies_dn13: f64 = *var_ies_dn13_slot;
        let mut var_ies_dn14: f64 = *var_ies_dn14_slot;
        let mut var_ies_dn2: f64 = *var_ies_dn2_slot;
        let mut var_ies_dn3: f64 = *var_ies_dn3_slot;
        let mut var_ies_dn4: f64 = *var_ies_dn4_slot;
        let mut var_ies_dn5: f64 = *var_ies_dn5_slot;
        let mut var_ies_dn6: f64 = *var_ies_dn6_slot;
        let mut var_ies_dn7: f64 = *var_ies_dn7_slot;
        let mut var_ies_dn8: f64 = *var_ies_dn8_slot;
        let mut var_ies_dn9: f64 = *var_ies_dn9_slot;
        let mut var_igidl_v: f64 = *var_igidl_v_slot;
        let mut var_igidl_v_dn0: f64 = *var_igidl_v_dn0_slot;
        let mut var_igidl_v_dn10: f64 = *var_igidl_v_dn10_slot;
        let mut var_igidl_v_dn11: f64 = *var_igidl_v_dn11_slot;
        let mut var_igidl_v_dn13: f64 = *var_igidl_v_dn13_slot;
        let mut var_igidl_v_dn14: f64 = *var_igidl_v_dn14_slot;
        let mut var_igidl_v_dn2: f64 = *var_igidl_v_dn2_slot;
        let mut var_igidl_v_dn3: f64 = *var_igidl_v_dn3_slot;
        let mut var_igidl_v_dn4: f64 = *var_igidl_v_dn4_slot;
        let mut var_igidl_v_dn5: f64 = *var_igidl_v_dn5_slot;
        let mut var_igidl_v_dn6: f64 = *var_igidl_v_dn6_slot;
        let mut var_igidl_v_dn7: f64 = *var_igidl_v_dn7_slot;
        let mut var_igidl_v_dn8: f64 = *var_igidl_v_dn8_slot;
        let mut var_igidl_v_dn9: f64 = *var_igidl_v_dn9_slot;
        let mut var_igidlb: f64 = *var_igidlb_slot;
        let mut var_igidlb_dn0: f64 = *var_igidlb_dn0_slot;
        let mut var_igidlb_dn10: f64 = *var_igidlb_dn10_slot;
        let mut var_igidlb_dn11: f64 = *var_igidlb_dn11_slot;
        let mut var_igidlb_dn13: f64 = *var_igidlb_dn13_slot;
        let mut var_igidlb_dn14: f64 = *var_igidlb_dn14_slot;
        let mut var_igidlb_dn2: f64 = *var_igidlb_dn2_slot;
        let mut var_igidlb_dn3: f64 = *var_igidlb_dn3_slot;
        let mut var_igidlb_dn4: f64 = *var_igidlb_dn4_slot;
        let mut var_igidlb_dn5: f64 = *var_igidlb_dn5_slot;
        let mut var_igidlb_dn6: f64 = *var_igidlb_dn6_slot;
        let mut var_igidlb_dn7: f64 = *var_igidlb_dn7_slot;
        let mut var_igidlb_dn8: f64 = *var_igidlb_dn8_slot;
        let mut var_igidlb_dn9: f64 = *var_igidlb_dn9_slot;
        let mut var_igislb: f64 = *var_igislb_slot;
        let mut var_igislb_dn0: f64 = *var_igislb_dn0_slot;
        let mut var_igislb_dn10: f64 = *var_igislb_dn10_slot;
        let mut var_igislb_dn11: f64 = *var_igislb_dn11_slot;
        let mut var_igislb_dn13: f64 = *var_igislb_dn13_slot;
        let mut var_igislb_dn14: f64 = *var_igislb_dn14_slot;
        let mut var_igislb_dn2: f64 = *var_igislb_dn2_slot;
        let mut var_igislb_dn3: f64 = *var_igislb_dn3_slot;
        let mut var_igislb_dn4: f64 = *var_igislb_dn4_slot;
        let mut var_igislb_dn5: f64 = *var_igislb_dn5_slot;
        let mut var_igislb_dn6: f64 = *var_igislb_dn6_slot;
        let mut var_igislb_dn7: f64 = *var_igislb_dn7_slot;
        let mut var_igislb_dn8: f64 = *var_igislb_dn8_slot;
        let mut var_igislb_dn9: f64 = *var_igislb_dn9_slot;
        let mut var_iii_1: f64 = *var_iii_1_slot;
        let mut var_iii_1_dn0: f64 = *var_iii_1_dn0_slot;
        let mut var_iii_1_dn10: f64 = *var_iii_1_dn10_slot;
        let mut var_iii_1_dn11: f64 = *var_iii_1_dn11_slot;
        let mut var_iii_1_dn13: f64 = *var_iii_1_dn13_slot;
        let mut var_iii_1_dn14: f64 = *var_iii_1_dn14_slot;
        let mut var_iii_1_dn2: f64 = *var_iii_1_dn2_slot;
        let mut var_iii_1_dn3: f64 = *var_iii_1_dn3_slot;
        let mut var_iii_1_dn4: f64 = *var_iii_1_dn4_slot;
        let mut var_iii_1_dn5: f64 = *var_iii_1_dn5_slot;
        let mut var_iii_1_dn6: f64 = *var_iii_1_dn6_slot;
        let mut var_iii_1_dn7: f64 = *var_iii_1_dn7_slot;
        let mut var_iii_1_dn8: f64 = *var_iii_1_dn8_slot;
        let mut var_iii_1_dn9: f64 = *var_iii_1_dn9_slot;
        let mut var_isbd: f64 = *var_isbd_slot;
        let mut var_isbd_dn0: f64 = *var_isbd_dn0_slot;
        let mut var_isbd_dn10: f64 = *var_isbd_dn10_slot;
        let mut var_isbd_dn11: f64 = *var_isbd_dn11_slot;
        let mut var_isbd_dn13: f64 = *var_isbd_dn13_slot;
        let mut var_isbd_dn14: f64 = *var_isbd_dn14_slot;
        let mut var_isbd_dn2: f64 = *var_isbd_dn2_slot;
        let mut var_isbd_dn3: f64 = *var_isbd_dn3_slot;
        let mut var_isbd_dn4: f64 = *var_isbd_dn4_slot;
        let mut var_isbd_dn5: f64 = *var_isbd_dn5_slot;
        let mut var_isbd_dn6: f64 = *var_isbd_dn6_slot;
        let mut var_isbd_dn7: f64 = *var_isbd_dn7_slot;
        let mut var_isbd_dn8: f64 = *var_isbd_dn8_slot;
        let mut var_isbd_dn9: f64 = *var_isbd_dn9_slot;
        let mut var_isbs: f64 = *var_isbs_slot;
        let mut var_isbs_dn0: f64 = *var_isbs_dn0_slot;
        let mut var_isbs_dn10: f64 = *var_isbs_dn10_slot;
        let mut var_isbs_dn11: f64 = *var_isbs_dn11_slot;
        let mut var_isbs_dn13: f64 = *var_isbs_dn13_slot;
        let mut var_isbs_dn14: f64 = *var_isbs_dn14_slot;
        let mut var_isbs_dn2: f64 = *var_isbs_dn2_slot;
        let mut var_isbs_dn3: f64 = *var_isbs_dn3_slot;
        let mut var_isbs_dn4: f64 = *var_isbs_dn4_slot;
        let mut var_isbs_dn5: f64 = *var_isbs_dn5_slot;
        let mut var_isbs_dn6: f64 = *var_isbs_dn6_slot;
        let mut var_isbs_dn7: f64 = *var_isbs_dn7_slot;
        let mut var_isbs_dn8: f64 = *var_isbs_dn8_slot;
        let mut var_isbs_dn9: f64 = *var_isbs_dn9_slot;
        let mut var_ivjdmfwd: f64 = *var_ivjdmfwd_slot;
        let mut var_ivjdmfwd_dn0: f64 = *var_ivjdmfwd_dn0_slot;
        let mut var_ivjdmfwd_dn10: f64 = *var_ivjdmfwd_dn10_slot;
        let mut var_ivjdmfwd_dn11: f64 = *var_ivjdmfwd_dn11_slot;
        let mut var_ivjdmfwd_dn13: f64 = *var_ivjdmfwd_dn13_slot;
        let mut var_ivjdmfwd_dn14: f64 = *var_ivjdmfwd_dn14_slot;
        let mut var_ivjdmfwd_dn2: f64 = *var_ivjdmfwd_dn2_slot;
        let mut var_ivjdmfwd_dn3: f64 = *var_ivjdmfwd_dn3_slot;
        let mut var_ivjdmfwd_dn4: f64 = *var_ivjdmfwd_dn4_slot;
        let mut var_ivjdmfwd_dn5: f64 = *var_ivjdmfwd_dn5_slot;
        let mut var_ivjdmfwd_dn6: f64 = *var_ivjdmfwd_dn6_slot;
        let mut var_ivjdmfwd_dn7: f64 = *var_ivjdmfwd_dn7_slot;
        let mut var_ivjdmfwd_dn8: f64 = *var_ivjdmfwd_dn8_slot;
        let mut var_ivjdmfwd_dn9: f64 = *var_ivjdmfwd_dn9_slot;
        let mut var_ivjdmrev: f64 = *var_ivjdmrev_slot;
        let mut var_ivjdmrev_dn0: f64 = *var_ivjdmrev_dn0_slot;
        let mut var_ivjdmrev_dn10: f64 = *var_ivjdmrev_dn10_slot;
        let mut var_ivjdmrev_dn11: f64 = *var_ivjdmrev_dn11_slot;
        let mut var_ivjdmrev_dn13: f64 = *var_ivjdmrev_dn13_slot;
        let mut var_ivjdmrev_dn14: f64 = *var_ivjdmrev_dn14_slot;
        let mut var_ivjdmrev_dn2: f64 = *var_ivjdmrev_dn2_slot;
        let mut var_ivjdmrev_dn3: f64 = *var_ivjdmrev_dn3_slot;
        let mut var_ivjdmrev_dn4: f64 = *var_ivjdmrev_dn4_slot;
        let mut var_ivjdmrev_dn5: f64 = *var_ivjdmrev_dn5_slot;
        let mut var_ivjdmrev_dn6: f64 = *var_ivjdmrev_dn6_slot;
        let mut var_ivjdmrev_dn7: f64 = *var_ivjdmrev_dn7_slot;
        let mut var_ivjdmrev_dn8: f64 = *var_ivjdmrev_dn8_slot;
        let mut var_ivjdmrev_dn9: f64 = *var_ivjdmrev_dn9_slot;
        let mut var_ivjsmfwd: f64 = *var_ivjsmfwd_slot;
        let mut var_ivjsmfwd_dn0: f64 = *var_ivjsmfwd_dn0_slot;
        let mut var_ivjsmfwd_dn10: f64 = *var_ivjsmfwd_dn10_slot;
        let mut var_ivjsmfwd_dn11: f64 = *var_ivjsmfwd_dn11_slot;
        let mut var_ivjsmfwd_dn13: f64 = *var_ivjsmfwd_dn13_slot;
        let mut var_ivjsmfwd_dn14: f64 = *var_ivjsmfwd_dn14_slot;
        let mut var_ivjsmfwd_dn2: f64 = *var_ivjsmfwd_dn2_slot;
        let mut var_ivjsmfwd_dn3: f64 = *var_ivjsmfwd_dn3_slot;
        let mut var_ivjsmfwd_dn4: f64 = *var_ivjsmfwd_dn4_slot;
        let mut var_ivjsmfwd_dn5: f64 = *var_ivjsmfwd_dn5_slot;
        let mut var_ivjsmfwd_dn6: f64 = *var_ivjsmfwd_dn6_slot;
        let mut var_ivjsmfwd_dn7: f64 = *var_ivjsmfwd_dn7_slot;
        let mut var_ivjsmfwd_dn8: f64 = *var_ivjsmfwd_dn8_slot;
        let mut var_ivjsmfwd_dn9: f64 = *var_ivjsmfwd_dn9_slot;
        let mut var_ivjsmrev: f64 = *var_ivjsmrev_slot;
        let mut var_ivjsmrev_dn0: f64 = *var_ivjsmrev_dn0_slot;
        let mut var_ivjsmrev_dn10: f64 = *var_ivjsmrev_dn10_slot;
        let mut var_ivjsmrev_dn11: f64 = *var_ivjsmrev_dn11_slot;
        let mut var_ivjsmrev_dn13: f64 = *var_ivjsmrev_dn13_slot;
        let mut var_ivjsmrev_dn14: f64 = *var_ivjsmrev_dn14_slot;
        let mut var_ivjsmrev_dn2: f64 = *var_ivjsmrev_dn2_slot;
        let mut var_ivjsmrev_dn3: f64 = *var_ivjsmrev_dn3_slot;
        let mut var_ivjsmrev_dn4: f64 = *var_ivjsmrev_dn4_slot;
        let mut var_ivjsmrev_dn5: f64 = *var_ivjsmrev_dn5_slot;
        let mut var_ivjsmrev_dn6: f64 = *var_ivjsmrev_dn6_slot;
        let mut var_ivjsmrev_dn7: f64 = *var_ivjsmrev_dn7_slot;
        let mut var_ivjsmrev_dn8: f64 = *var_ivjsmrev_dn8_slot;
        let mut var_ivjsmrev_dn9: f64 = *var_ivjsmrev_dn9_slot;
        let mut var_nvtmd: f64 = *var_nvtmd_slot;
        let mut var_nvtmd_dn4: f64 = *var_nvtmd_dn4_slot;
        let mut var_nvtms: f64 = *var_nvtms_slot;
        let mut var_nvtms_dn4: f64 = *var_nvtms_dn4_slot;
        let mut var_pb21d: f64 = *var_pb21d_slot;
        let mut var_pb21d_dn4: f64 = *var_pb21d_dn4_slot;
        let mut var_pb21s: f64 = *var_pb21s_slot;
        let mut var_pb21s_dn4: f64 = *var_pb21s_dn4_slot;
        let mut var_pb22d: f64 = *var_pb22d_slot;
        let mut var_pb22d_dn4: f64 = *var_pb22d_dn4_slot;
        let mut var_pb22s: f64 = *var_pb22s_slot;
        let mut var_pb22s_dn4: f64 = *var_pb22s_dn4_slot;
        let mut var_pb23d: f64 = *var_pb23d_slot;
        let mut var_pb23d_dn4: f64 = *var_pb23d_dn4_slot;
        let mut var_pb23s: f64 = *var_pb23s_slot;
        let mut var_pb23s_dn4: f64 = *var_pb23s_dn4_slot;
        let mut var_pgidlb_i: f64 = *var_pgidlb_i_slot;
        let mut var_pgislb_i: f64 = *var_pgislb_i_slot;
        let mut var_qb_acc: f64 = *var_qb_acc_slot;
        let mut var_qb_acc_dn0: f64 = *var_qb_acc_dn0_slot;
        let mut var_qb_acc_dn10: f64 = *var_qb_acc_dn10_slot;
        let mut var_qb_acc_dn11: f64 = *var_qb_acc_dn11_slot;
        let mut var_qb_acc_dn13: f64 = *var_qb_acc_dn13_slot;
        let mut var_qb_acc_dn14: f64 = *var_qb_acc_dn14_slot;
        let mut var_qb_acc_dn2: f64 = *var_qb_acc_dn2_slot;
        let mut var_qb_acc_dn3: f64 = *var_qb_acc_dn3_slot;
        let mut var_qb_acc_dn4: f64 = *var_qb_acc_dn4_slot;
        let mut var_qb_acc_dn5: f64 = *var_qb_acc_dn5_slot;
        let mut var_qb_acc_dn6: f64 = *var_qb_acc_dn6_slot;
        let mut var_qb_acc_dn7: f64 = *var_qb_acc_dn7_slot;
        let mut var_qb_acc_dn8: f64 = *var_qb_acc_dn8_slot;
        let mut var_qb_acc_dn9: f64 = *var_qb_acc_dn9_slot;
        let mut var_qedj: f64 = *var_qedj_slot;
        let mut var_qedj_dn3: f64 = *var_qedj_dn3_slot;
        let mut var_qedj_dn4: f64 = *var_qedj_dn4_slot;
        let mut var_qedj_dn5: f64 = *var_qedj_dn5_slot;
        let mut var_qesj: f64 = *var_qesj_slot;
        let mut var_qesj_dn3: f64 = *var_qesj_dn3_slot;
        let mut var_qesj_dn4: f64 = *var_qesj_dn4_slot;
        let mut var_qesj_dn6: f64 = *var_qesj_dn6_slot;
        let mut var_qg_acc: f64 = *var_qg_acc_slot;
        let mut var_qg_acc_dn0: f64 = *var_qg_acc_dn0_slot;
        let mut var_qg_acc_dn10: f64 = *var_qg_acc_dn10_slot;
        let mut var_qg_acc_dn11: f64 = *var_qg_acc_dn11_slot;
        let mut var_qg_acc_dn13: f64 = *var_qg_acc_dn13_slot;
        let mut var_qg_acc_dn14: f64 = *var_qg_acc_dn14_slot;
        let mut var_qg_acc_dn2: f64 = *var_qg_acc_dn2_slot;
        let mut var_qg_acc_dn3: f64 = *var_qg_acc_dn3_slot;
        let mut var_qg_acc_dn4: f64 = *var_qg_acc_dn4_slot;
        let mut var_qg_acc_dn5: f64 = *var_qg_acc_dn5_slot;
        let mut var_qg_acc_dn6: f64 = *var_qg_acc_dn6_slot;
        let mut var_qg_acc_dn7: f64 = *var_qg_acc_dn7_slot;
        let mut var_qg_acc_dn8: f64 = *var_qg_acc_dn8_slot;
        let mut var_qg_acc_dn9: f64 = *var_qg_acc_dn9_slot;
        let mut var_qgd_ov: f64 = *var_qgd_ov_slot;
        let mut var_qgd_ov_dn0: f64 = *var_qgd_ov_dn0_slot;
        let mut var_qgd_ov_dn10: f64 = *var_qgd_ov_dn10_slot;
        let mut var_qgd_ov_dn11: f64 = *var_qgd_ov_dn11_slot;
        let mut var_qgd_ov_dn13: f64 = *var_qgd_ov_dn13_slot;
        let mut var_qgd_ov_dn14: f64 = *var_qgd_ov_dn14_slot;
        let mut var_qgd_ov_dn2: f64 = *var_qgd_ov_dn2_slot;
        let mut var_qgd_ov_dn3: f64 = *var_qgd_ov_dn3_slot;
        let mut var_qgd_ov_dn4: f64 = *var_qgd_ov_dn4_slot;
        let mut var_qgd_ov_dn5: f64 = *var_qgd_ov_dn5_slot;
        let mut var_qgd_ov_dn6: f64 = *var_qgd_ov_dn6_slot;
        let mut var_qgd_ov_dn7: f64 = *var_qgd_ov_dn7_slot;
        let mut var_qgd_ov_dn8: f64 = *var_qgd_ov_dn8_slot;
        let mut var_qgd_ov_dn9: f64 = *var_qgd_ov_dn9_slot;
        let mut var_qgs_ov: f64 = *var_qgs_ov_slot;
        let mut var_qgs_ov_dn0: f64 = *var_qgs_ov_dn0_slot;
        let mut var_qgs_ov_dn10: f64 = *var_qgs_ov_dn10_slot;
        let mut var_qgs_ov_dn11: f64 = *var_qgs_ov_dn11_slot;
        let mut var_qgs_ov_dn13: f64 = *var_qgs_ov_dn13_slot;
        let mut var_qgs_ov_dn14: f64 = *var_qgs_ov_dn14_slot;
        let mut var_qgs_ov_dn2: f64 = *var_qgs_ov_dn2_slot;
        let mut var_qgs_ov_dn3: f64 = *var_qgs_ov_dn3_slot;
        let mut var_qgs_ov_dn4: f64 = *var_qgs_ov_dn4_slot;
        let mut var_qgs_ov_dn5: f64 = *var_qgs_ov_dn5_slot;
        let mut var_qgs_ov_dn6: f64 = *var_qgs_ov_dn6_slot;
        let mut var_qgs_ov_dn7: f64 = *var_qgs_ov_dn7_slot;
        let mut var_qgs_ov_dn8: f64 = *var_qgs_ov_dn8_slot;
        let mut var_qgs_ov_dn9: f64 = *var_qgs_ov_dn9_slot;
        let mut var_sslpfwd: f64 = *var_sslpfwd_slot;
        let mut var_sslpfwd_dn0: f64 = *var_sslpfwd_dn0_slot;
        let mut var_sslpfwd_dn10: f64 = *var_sslpfwd_dn10_slot;
        let mut var_sslpfwd_dn11: f64 = *var_sslpfwd_dn11_slot;
        let mut var_sslpfwd_dn13: f64 = *var_sslpfwd_dn13_slot;
        let mut var_sslpfwd_dn14: f64 = *var_sslpfwd_dn14_slot;
        let mut var_sslpfwd_dn2: f64 = *var_sslpfwd_dn2_slot;
        let mut var_sslpfwd_dn3: f64 = *var_sslpfwd_dn3_slot;
        let mut var_sslpfwd_dn4: f64 = *var_sslpfwd_dn4_slot;
        let mut var_sslpfwd_dn5: f64 = *var_sslpfwd_dn5_slot;
        let mut var_sslpfwd_dn6: f64 = *var_sslpfwd_dn6_slot;
        let mut var_sslpfwd_dn7: f64 = *var_sslpfwd_dn7_slot;
        let mut var_sslpfwd_dn8: f64 = *var_sslpfwd_dn8_slot;
        let mut var_sslpfwd_dn9: f64 = *var_sslpfwd_dn9_slot;
        let mut var_sslprev: f64 = *var_sslprev_slot;
        let mut var_sslprev_dn0: f64 = *var_sslprev_dn0_slot;
        let mut var_sslprev_dn10: f64 = *var_sslprev_dn10_slot;
        let mut var_sslprev_dn11: f64 = *var_sslprev_dn11_slot;
        let mut var_sslprev_dn13: f64 = *var_sslprev_dn13_slot;
        let mut var_sslprev_dn14: f64 = *var_sslprev_dn14_slot;
        let mut var_sslprev_dn2: f64 = *var_sslprev_dn2_slot;
        let mut var_sslprev_dn3: f64 = *var_sslprev_dn3_slot;
        let mut var_sslprev_dn4: f64 = *var_sslprev_dn4_slot;
        let mut var_sslprev_dn5: f64 = *var_sslprev_dn5_slot;
        let mut var_sslprev_dn6: f64 = *var_sslprev_dn6_slot;
        let mut var_sslprev_dn7: f64 = *var_sslprev_dn7_slot;
        let mut var_sslprev_dn8: f64 = *var_sslprev_dn8_slot;
        let mut var_sslprev_dn9: f64 = *var_sslprev_dn9_slot;
        let mut var_vec1d: f64 = *var_vec1d_slot;
        let mut var_vec1d_dn4: f64 = *var_vec1d_dn4_slot;
        let mut var_vec1s: f64 = *var_vec1s_slot;
        let mut var_vec1s_dn4: f64 = *var_vec1s_dn4_slot;
        let mut var_vec2d: f64 = *var_vec2d_slot;
        let mut var_vec2d_dn4: f64 = *var_vec2d_dn4_slot;
        let mut var_vec2s: f64 = *var_vec2s_slot;
        let mut var_vec2s_dn4: f64 = *var_vec2s_dn4_slot;
        let mut var_vec3d: f64 = *var_vec3d_slot;
        let mut var_vec3d_dn4: f64 = *var_vec3d_dn4_slot;
        let mut var_vec3s: f64 = *var_vec3s_slot;
        let mut var_vec3s_dn4: f64 = *var_vec3s_dn4_slot;
        let mut var_vjdmfwd: f64 = *var_vjdmfwd_slot;
        let mut var_vjdmfwd_dn0: f64 = *var_vjdmfwd_dn0_slot;
        let mut var_vjdmfwd_dn10: f64 = *var_vjdmfwd_dn10_slot;
        let mut var_vjdmfwd_dn11: f64 = *var_vjdmfwd_dn11_slot;
        let mut var_vjdmfwd_dn13: f64 = *var_vjdmfwd_dn13_slot;
        let mut var_vjdmfwd_dn14: f64 = *var_vjdmfwd_dn14_slot;
        let mut var_vjdmfwd_dn2: f64 = *var_vjdmfwd_dn2_slot;
        let mut var_vjdmfwd_dn3: f64 = *var_vjdmfwd_dn3_slot;
        let mut var_vjdmfwd_dn4: f64 = *var_vjdmfwd_dn4_slot;
        let mut var_vjdmfwd_dn5: f64 = *var_vjdmfwd_dn5_slot;
        let mut var_vjdmfwd_dn6: f64 = *var_vjdmfwd_dn6_slot;
        let mut var_vjdmfwd_dn7: f64 = *var_vjdmfwd_dn7_slot;
        let mut var_vjdmfwd_dn8: f64 = *var_vjdmfwd_dn8_slot;
        let mut var_vjdmfwd_dn9: f64 = *var_vjdmfwd_dn9_slot;
        let mut var_vjdmrev: f64 = *var_vjdmrev_slot;
        let mut var_vjdmrev_dn0: f64 = *var_vjdmrev_dn0_slot;
        let mut var_vjdmrev_dn10: f64 = *var_vjdmrev_dn10_slot;
        let mut var_vjdmrev_dn11: f64 = *var_vjdmrev_dn11_slot;
        let mut var_vjdmrev_dn13: f64 = *var_vjdmrev_dn13_slot;
        let mut var_vjdmrev_dn14: f64 = *var_vjdmrev_dn14_slot;
        let mut var_vjdmrev_dn2: f64 = *var_vjdmrev_dn2_slot;
        let mut var_vjdmrev_dn3: f64 = *var_vjdmrev_dn3_slot;
        let mut var_vjdmrev_dn4: f64 = *var_vjdmrev_dn4_slot;
        let mut var_vjdmrev_dn5: f64 = *var_vjdmrev_dn5_slot;
        let mut var_vjdmrev_dn6: f64 = *var_vjdmrev_dn6_slot;
        let mut var_vjdmrev_dn7: f64 = *var_vjdmrev_dn7_slot;
        let mut var_vjdmrev_dn8: f64 = *var_vjdmrev_dn8_slot;
        let mut var_vjdmrev_dn9: f64 = *var_vjdmrev_dn9_slot;
        let mut var_vjsmfwd: f64 = *var_vjsmfwd_slot;
        let mut var_vjsmfwd_dn0: f64 = *var_vjsmfwd_dn0_slot;
        let mut var_vjsmfwd_dn10: f64 = *var_vjsmfwd_dn10_slot;
        let mut var_vjsmfwd_dn11: f64 = *var_vjsmfwd_dn11_slot;
        let mut var_vjsmfwd_dn13: f64 = *var_vjsmfwd_dn13_slot;
        let mut var_vjsmfwd_dn14: f64 = *var_vjsmfwd_dn14_slot;
        let mut var_vjsmfwd_dn2: f64 = *var_vjsmfwd_dn2_slot;
        let mut var_vjsmfwd_dn3: f64 = *var_vjsmfwd_dn3_slot;
        let mut var_vjsmfwd_dn4: f64 = *var_vjsmfwd_dn4_slot;
        let mut var_vjsmfwd_dn5: f64 = *var_vjsmfwd_dn5_slot;
        let mut var_vjsmfwd_dn6: f64 = *var_vjsmfwd_dn6_slot;
        let mut var_vjsmfwd_dn7: f64 = *var_vjsmfwd_dn7_slot;
        let mut var_vjsmfwd_dn8: f64 = *var_vjsmfwd_dn8_slot;
        let mut var_vjsmfwd_dn9: f64 = *var_vjsmfwd_dn9_slot;
        let mut var_vjsmrev: f64 = *var_vjsmrev_slot;
        let mut var_vjsmrev_dn0: f64 = *var_vjsmrev_dn0_slot;
        let mut var_vjsmrev_dn10: f64 = *var_vjsmrev_dn10_slot;
        let mut var_vjsmrev_dn11: f64 = *var_vjsmrev_dn11_slot;
        let mut var_vjsmrev_dn13: f64 = *var_vjsmrev_dn13_slot;
        let mut var_vjsmrev_dn14: f64 = *var_vjsmrev_dn14_slot;
        let mut var_vjsmrev_dn2: f64 = *var_vjsmrev_dn2_slot;
        let mut var_vjsmrev_dn3: f64 = *var_vjsmrev_dn3_slot;
        let mut var_vjsmrev_dn4: f64 = *var_vjsmrev_dn4_slot;
        let mut var_vjsmrev_dn5: f64 = *var_vjsmrev_dn5_slot;
        let mut var_vjsmrev_dn6: f64 = *var_vjsmrev_dn6_slot;
        let mut var_vjsmrev_dn7: f64 = *var_vjsmrev_dn7_slot;
        let mut var_vjsmrev_dn8: f64 = *var_vjsmrev_dn8_slot;
        let mut var_vjsmrev_dn9: f64 = *var_vjsmrev_dn9_slot;
        let mut var_xexpbvd: f64 = *var_xexpbvd_slot;
        let mut var_xexpbvd_dn4: f64 = *var_xexpbvd_dn4_slot;
        let mut var_xexpbvs: f64 = *var_xexpbvs_slot;
        let mut var_xexpbvs_dn4: f64 = *var_xexpbvs_dn4_slot;

        var_igidl_v = 0.0;
        var_igidl_v_dn0 = 0.0;
        var_igidl_v_dn2 = 0.0;
        var_igidl_v_dn3 = 0.0;
        var_igidl_v_dn4 = 0.0;
        var_igidl_v_dn5 = 0.0;
        var_igidl_v_dn6 = 0.0;
        var_igidl_v_dn7 = 0.0;
        var_igidl_v_dn8 = 0.0;
        var_igidl_v_dn9 = 0.0;
        var_igidl_v_dn10 = 0.0;
        var_igidl_v_dn11 = 0.0;
        var_igidl_v_dn13 = 0.0;
        var_igidl_v_dn14 = 0.0;

        var_igislb = 0.0;
        var_igislb_dn0 = 0.0;
        var_igislb_dn2 = 0.0;
        var_igislb_dn3 = 0.0;
        var_igislb_dn4 = 0.0;
        var_igislb_dn5 = 0.0;
        var_igislb_dn6 = 0.0;
        var_igislb_dn7 = 0.0;
        var_igislb_dn8 = 0.0;
        var_igislb_dn9 = 0.0;
        var_igislb_dn10 = 0.0;
        var_igislb_dn11 = 0.0;
        var_igislb_dn13 = 0.0;
        var_igislb_dn14 = 0.0;

        var_igidlb = 0.0;
        var_igidlb_dn0 = 0.0;
        var_igidlb_dn2 = 0.0;
        var_igidlb_dn3 = 0.0;
        var_igidlb_dn4 = 0.0;
        var_igidlb_dn5 = 0.0;
        var_igidlb_dn6 = 0.0;
        var_igidlb_dn7 = 0.0;
        var_igidlb_dn8 = 0.0;
        var_igidlb_dn9 = 0.0;
        var_igidlb_dn10 = 0.0;
        var_igidlb_dn11 = 0.0;
        var_igidlb_dn13 = 0.0;
        var_igidlb_dn14 = 0.0;

        var_agidlb_i = 0.0;

        var_bgidlb_i = 0.0;

        var_bgidlb_t = 0.0;
        var_bgidlb_t_dn4 = 0.0;

        var_cgidlb_i = 0.0;

        var_egidlb_i = 0.0;

        var_pgidlb_i = 0.0;

        var_agislb_i = 0.0;

        var_bgislb_i = 0.0;

        var_bgislb_t = 0.0;
        var_bgislb_t_dn4 = 0.0;

        var_cgislb_i = 0.0;

        var_egislb_i = 0.0;

        var_pgislb_i = 0.0;

        var_iii_1 = 0.0;
        var_iii_1_dn0 = 0.0;
        var_iii_1_dn2 = 0.0;
        var_iii_1_dn3 = 0.0;
        var_iii_1_dn4 = 0.0;
        var_iii_1_dn5 = 0.0;
        var_iii_1_dn6 = 0.0;
        var_iii_1_dn7 = 0.0;
        var_iii_1_dn8 = 0.0;
        var_iii_1_dn9 = 0.0;
        var_iii_1_dn10 = 0.0;
        var_iii_1_dn11 = 0.0;
        var_iii_1_dn13 = 0.0;
        var_iii_1_dn14 = 0.0;

        var_cox_acc = 0.0;
        var_cox_acc_dn0 = 0.0;
        var_cox_acc_dn2 = 0.0;
        var_cox_acc_dn3 = 0.0;
        var_cox_acc_dn4 = 0.0;
        var_cox_acc_dn5 = 0.0;
        var_cox_acc_dn6 = 0.0;
        var_cox_acc_dn7 = 0.0;
        var_cox_acc_dn8 = 0.0;
        var_cox_acc_dn9 = 0.0;
        var_cox_acc_dn10 = 0.0;
        var_cox_acc_dn11 = 0.0;
        var_cox_acc_dn13 = 0.0;
        var_cox_acc_dn14 = 0.0;

        var_qg_acc = 0.0;
        var_qg_acc_dn0 = 0.0;
        var_qg_acc_dn2 = 0.0;
        var_qg_acc_dn3 = 0.0;
        var_qg_acc_dn4 = 0.0;
        var_qg_acc_dn5 = 0.0;
        var_qg_acc_dn6 = 0.0;
        var_qg_acc_dn7 = 0.0;
        var_qg_acc_dn8 = 0.0;
        var_qg_acc_dn9 = 0.0;
        var_qg_acc_dn10 = 0.0;
        var_qg_acc_dn11 = 0.0;
        var_qg_acc_dn13 = 0.0;
        var_qg_acc_dn14 = 0.0;

        var_qb_acc = 0.0;
        var_qb_acc_dn0 = 0.0;
        var_qb_acc_dn2 = 0.0;
        var_qb_acc_dn3 = 0.0;
        var_qb_acc_dn4 = 0.0;
        var_qb_acc_dn5 = 0.0;
        var_qb_acc_dn6 = 0.0;
        var_qb_acc_dn7 = 0.0;
        var_qb_acc_dn8 = 0.0;
        var_qb_acc_dn9 = 0.0;
        var_qb_acc_dn10 = 0.0;
        var_qb_acc_dn11 = 0.0;
        var_qb_acc_dn13 = 0.0;
        var_qb_acc_dn14 = 0.0;

        var_qgs_ov = 0.0;
        var_qgs_ov_dn0 = 0.0;
        var_qgs_ov_dn2 = 0.0;
        var_qgs_ov_dn3 = 0.0;
        var_qgs_ov_dn4 = 0.0;
        var_qgs_ov_dn5 = 0.0;
        var_qgs_ov_dn6 = 0.0;
        var_qgs_ov_dn7 = 0.0;
        var_qgs_ov_dn8 = 0.0;
        var_qgs_ov_dn9 = 0.0;
        var_qgs_ov_dn10 = 0.0;
        var_qgs_ov_dn11 = 0.0;
        var_qgs_ov_dn13 = 0.0;
        var_qgs_ov_dn14 = 0.0;

        var_qgd_ov = 0.0;
        var_qgd_ov_dn0 = 0.0;
        var_qgd_ov_dn2 = 0.0;
        var_qgd_ov_dn3 = 0.0;
        var_qgd_ov_dn4 = 0.0;
        var_qgd_ov_dn5 = 0.0;
        var_qgd_ov_dn6 = 0.0;
        var_qgd_ov_dn7 = 0.0;
        var_qgd_ov_dn8 = 0.0;
        var_qgd_ov_dn9 = 0.0;
        var_qgd_ov_dn10 = 0.0;
        var_qgd_ov_dn11 = 0.0;
        var_qgd_ov_dn13 = 0.0;
        var_qgd_ov_dn14 = 0.0;

        var_ies = 0.0;
        var_ies_dn0 = 0.0;
        var_ies_dn2 = 0.0;
        var_ies_dn3 = 0.0;
        var_ies_dn4 = 0.0;
        var_ies_dn5 = 0.0;
        var_ies_dn6 = 0.0;
        var_ies_dn7 = 0.0;
        var_ies_dn8 = 0.0;
        var_ies_dn9 = 0.0;
        var_ies_dn10 = 0.0;
        var_ies_dn11 = 0.0;
        var_ies_dn13 = 0.0;
        var_ies_dn14 = 0.0;

        var_ied = 0.0;
        var_ied_dn0 = 0.0;
        var_ied_dn2 = 0.0;
        var_ied_dn3 = 0.0;
        var_ied_dn4 = 0.0;
        var_ied_dn5 = 0.0;
        var_ied_dn6 = 0.0;
        var_ied_dn7 = 0.0;
        var_ied_dn8 = 0.0;
        var_ied_dn9 = 0.0;
        var_ied_dn10 = 0.0;
        var_ied_dn11 = 0.0;
        var_ied_dn13 = 0.0;
        var_ied_dn14 = 0.0;

        var_czbs = 0.0;
        var_czbs_dn4 = 0.0;

        var_czbssw = 0.0;
        var_czbssw_dn4 = 0.0;

        var_czbsswg = 0.0;
        var_czbsswg_dn4 = 0.0;

        var_czbd = 0.0;
        var_czbd_dn4 = 0.0;

        var_czbdsw = 0.0;
        var_czbdsw_dn4 = 0.0;

        var_czbdswg = 0.0;
        var_czbdswg_dn4 = 0.0;

        var_qesj = 0.0;
        var_qesj_dn3 = 0.0;
        var_qesj_dn4 = 0.0;
        var_qesj_dn6 = 0.0;

        var_qedj = 0.0;
        var_qedj_dn3 = 0.0;
        var_qedj_dn4 = 0.0;
        var_qedj_dn5 = 0.0;

        var_isbs = 0.0;
        var_isbs_dn0 = 0.0;
        var_isbs_dn2 = 0.0;
        var_isbs_dn3 = 0.0;
        var_isbs_dn4 = 0.0;
        var_isbs_dn5 = 0.0;
        var_isbs_dn6 = 0.0;
        var_isbs_dn7 = 0.0;
        var_isbs_dn8 = 0.0;
        var_isbs_dn9 = 0.0;
        var_isbs_dn10 = 0.0;
        var_isbs_dn11 = 0.0;
        var_isbs_dn13 = 0.0;
        var_isbs_dn14 = 0.0;

        var_isbd = 0.0;
        var_isbd_dn0 = 0.0;
        var_isbd_dn2 = 0.0;
        var_isbd_dn3 = 0.0;
        var_isbd_dn4 = 0.0;
        var_isbd_dn5 = 0.0;
        var_isbd_dn6 = 0.0;
        var_isbd_dn7 = 0.0;
        var_isbd_dn8 = 0.0;
        var_isbd_dn9 = 0.0;
        var_isbd_dn10 = 0.0;
        var_isbd_dn11 = 0.0;
        var_isbd_dn13 = 0.0;
        var_isbd_dn14 = 0.0;

        var_nvtms = 0.0;
        var_nvtms_dn4 = 0.0;

        var_nvtmd = 0.0;
        var_nvtmd_dn4 = 0.0;

        var_sslpfwd = 0.0;
        var_sslpfwd_dn0 = 0.0;
        var_sslpfwd_dn2 = 0.0;
        var_sslpfwd_dn3 = 0.0;
        var_sslpfwd_dn4 = 0.0;
        var_sslpfwd_dn5 = 0.0;
        var_sslpfwd_dn6 = 0.0;
        var_sslpfwd_dn7 = 0.0;
        var_sslpfwd_dn8 = 0.0;
        var_sslpfwd_dn9 = 0.0;
        var_sslpfwd_dn10 = 0.0;
        var_sslpfwd_dn11 = 0.0;
        var_sslpfwd_dn13 = 0.0;
        var_sslpfwd_dn14 = 0.0;

        var_ivjsmfwd = 0.0;
        var_ivjsmfwd_dn0 = 0.0;
        var_ivjsmfwd_dn2 = 0.0;
        var_ivjsmfwd_dn3 = 0.0;
        var_ivjsmfwd_dn4 = 0.0;
        var_ivjsmfwd_dn5 = 0.0;
        var_ivjsmfwd_dn6 = 0.0;
        var_ivjsmfwd_dn7 = 0.0;
        var_ivjsmfwd_dn8 = 0.0;
        var_ivjsmfwd_dn9 = 0.0;
        var_ivjsmfwd_dn10 = 0.0;
        var_ivjsmfwd_dn11 = 0.0;
        var_ivjsmfwd_dn13 = 0.0;
        var_ivjsmfwd_dn14 = 0.0;

        var_vjsmfwd = 0.0;
        var_vjsmfwd_dn0 = 0.0;
        var_vjsmfwd_dn2 = 0.0;
        var_vjsmfwd_dn3 = 0.0;
        var_vjsmfwd_dn4 = 0.0;
        var_vjsmfwd_dn5 = 0.0;
        var_vjsmfwd_dn6 = 0.0;
        var_vjsmfwd_dn7 = 0.0;
        var_vjsmfwd_dn8 = 0.0;
        var_vjsmfwd_dn9 = 0.0;
        var_vjsmfwd_dn10 = 0.0;
        var_vjsmfwd_dn11 = 0.0;
        var_vjsmfwd_dn13 = 0.0;
        var_vjsmfwd_dn14 = 0.0;

        var_xexpbvs = 0.0;
        var_xexpbvs_dn4 = 0.0;

        var_sslprev = 0.0;
        var_sslprev_dn0 = 0.0;
        var_sslprev_dn2 = 0.0;
        var_sslprev_dn3 = 0.0;
        var_sslprev_dn4 = 0.0;
        var_sslprev_dn5 = 0.0;
        var_sslprev_dn6 = 0.0;
        var_sslprev_dn7 = 0.0;
        var_sslprev_dn8 = 0.0;
        var_sslprev_dn9 = 0.0;
        var_sslprev_dn10 = 0.0;
        var_sslprev_dn11 = 0.0;
        var_sslprev_dn13 = 0.0;
        var_sslprev_dn14 = 0.0;

        var_ivjsmrev = 0.0;
        var_ivjsmrev_dn0 = 0.0;
        var_ivjsmrev_dn2 = 0.0;
        var_ivjsmrev_dn3 = 0.0;
        var_ivjsmrev_dn4 = 0.0;
        var_ivjsmrev_dn5 = 0.0;
        var_ivjsmrev_dn6 = 0.0;
        var_ivjsmrev_dn7 = 0.0;
        var_ivjsmrev_dn8 = 0.0;
        var_ivjsmrev_dn9 = 0.0;
        var_ivjsmrev_dn10 = 0.0;
        var_ivjsmrev_dn11 = 0.0;
        var_ivjsmrev_dn13 = 0.0;
        var_ivjsmrev_dn14 = 0.0;

        var_vjsmrev = 0.0;
        var_vjsmrev_dn0 = 0.0;
        var_vjsmrev_dn2 = 0.0;
        var_vjsmrev_dn3 = 0.0;
        var_vjsmrev_dn4 = 0.0;
        var_vjsmrev_dn5 = 0.0;
        var_vjsmrev_dn6 = 0.0;
        var_vjsmrev_dn7 = 0.0;
        var_vjsmrev_dn8 = 0.0;
        var_vjsmrev_dn9 = 0.0;
        var_vjsmrev_dn10 = 0.0;
        var_vjsmrev_dn11 = 0.0;
        var_vjsmrev_dn13 = 0.0;
        var_vjsmrev_dn14 = 0.0;

        var_dslpfwd = 0.0;
        var_dslpfwd_dn0 = 0.0;
        var_dslpfwd_dn2 = 0.0;
        var_dslpfwd_dn3 = 0.0;
        var_dslpfwd_dn4 = 0.0;
        var_dslpfwd_dn5 = 0.0;
        var_dslpfwd_dn6 = 0.0;
        var_dslpfwd_dn7 = 0.0;
        var_dslpfwd_dn8 = 0.0;
        var_dslpfwd_dn9 = 0.0;
        var_dslpfwd_dn10 = 0.0;
        var_dslpfwd_dn11 = 0.0;
        var_dslpfwd_dn13 = 0.0;
        var_dslpfwd_dn14 = 0.0;

        var_ivjdmfwd = 0.0;
        var_ivjdmfwd_dn0 = 0.0;
        var_ivjdmfwd_dn2 = 0.0;
        var_ivjdmfwd_dn3 = 0.0;
        var_ivjdmfwd_dn4 = 0.0;
        var_ivjdmfwd_dn5 = 0.0;
        var_ivjdmfwd_dn6 = 0.0;
        var_ivjdmfwd_dn7 = 0.0;
        var_ivjdmfwd_dn8 = 0.0;
        var_ivjdmfwd_dn9 = 0.0;
        var_ivjdmfwd_dn10 = 0.0;
        var_ivjdmfwd_dn11 = 0.0;
        var_ivjdmfwd_dn13 = 0.0;
        var_ivjdmfwd_dn14 = 0.0;

        var_vjdmfwd = 0.0;
        var_vjdmfwd_dn0 = 0.0;
        var_vjdmfwd_dn2 = 0.0;
        var_vjdmfwd_dn3 = 0.0;
        var_vjdmfwd_dn4 = 0.0;
        var_vjdmfwd_dn5 = 0.0;
        var_vjdmfwd_dn6 = 0.0;
        var_vjdmfwd_dn7 = 0.0;
        var_vjdmfwd_dn8 = 0.0;
        var_vjdmfwd_dn9 = 0.0;
        var_vjdmfwd_dn10 = 0.0;
        var_vjdmfwd_dn11 = 0.0;
        var_vjdmfwd_dn13 = 0.0;
        var_vjdmfwd_dn14 = 0.0;

        var_xexpbvd = 0.0;
        var_xexpbvd_dn4 = 0.0;

        var_dslprev = 0.0;
        var_dslprev_dn0 = 0.0;
        var_dslprev_dn2 = 0.0;
        var_dslprev_dn3 = 0.0;
        var_dslprev_dn4 = 0.0;
        var_dslprev_dn5 = 0.0;
        var_dslprev_dn6 = 0.0;
        var_dslprev_dn7 = 0.0;
        var_dslprev_dn8 = 0.0;
        var_dslprev_dn9 = 0.0;
        var_dslprev_dn10 = 0.0;
        var_dslprev_dn11 = 0.0;
        var_dslprev_dn13 = 0.0;
        var_dslprev_dn14 = 0.0;

        var_ivjdmrev = 0.0;
        var_ivjdmrev_dn0 = 0.0;
        var_ivjdmrev_dn2 = 0.0;
        var_ivjdmrev_dn3 = 0.0;
        var_ivjdmrev_dn4 = 0.0;
        var_ivjdmrev_dn5 = 0.0;
        var_ivjdmrev_dn6 = 0.0;
        var_ivjdmrev_dn7 = 0.0;
        var_ivjdmrev_dn8 = 0.0;
        var_ivjdmrev_dn9 = 0.0;
        var_ivjdmrev_dn10 = 0.0;
        var_ivjdmrev_dn11 = 0.0;
        var_ivjdmrev_dn13 = 0.0;
        var_ivjdmrev_dn14 = 0.0;

        var_vjdmrev = 0.0;
        var_vjdmrev_dn0 = 0.0;
        var_vjdmrev_dn2 = 0.0;
        var_vjdmrev_dn3 = 0.0;
        var_vjdmrev_dn4 = 0.0;
        var_vjdmrev_dn5 = 0.0;
        var_vjdmrev_dn6 = 0.0;
        var_vjdmrev_dn7 = 0.0;
        var_vjdmrev_dn8 = 0.0;
        var_vjdmrev_dn9 = 0.0;
        var_vjdmrev_dn10 = 0.0;
        var_vjdmrev_dn11 = 0.0;
        var_vjdmrev_dn13 = 0.0;
        var_vjdmrev_dn14 = 0.0;

        var_vec1s = 0.0;
        var_vec1s_dn4 = 0.0;

        var_pb21s = 0.0;
        var_pb21s_dn4 = 0.0;

        var_vec2s = 0.0;
        var_vec2s_dn4 = 0.0;

        var_pb22s = 0.0;
        var_pb22s_dn4 = 0.0;

        var_vec3s = 0.0;
        var_vec3s_dn4 = 0.0;

        var_pb23s = 0.0;
        var_pb23s_dn4 = 0.0;

        var_vec1d = 0.0;
        var_vec1d_dn4 = 0.0;

        var_pb21d = 0.0;
        var_pb21d_dn4 = 0.0;

        var_vec2d = 0.0;
        var_vec2d_dn4 = 0.0;

        var_pb22d = 0.0;
        var_pb22d_dn4 = 0.0;

        var_vec3d = 0.0;
        var_vec3d_dn4 = 0.0;

        var_pb23d = 0.0;
        var_pb23d_dn4 = 0.0;

        var_gcrg = 0.0;
        var_gcrg_dn0 = 0.0;
        var_gcrg_dn2 = 0.0;
        var_gcrg_dn3 = 0.0;
        var_gcrg_dn4 = 0.0;
        var_gcrg_dn5 = 0.0;
        var_gcrg_dn6 = 0.0;
        var_gcrg_dn7 = 0.0;
        var_gcrg_dn8 = 0.0;
        var_gcrg_dn9 = 0.0;
        var_gcrg_dn10 = 0.0;
        var_gcrg_dn11 = 0.0;
        var_gcrg_dn13 = 0.0;
        var_gcrg_dn14 = 0.0;

        var_gtau = 0.0;
        var_gtau_dn0 = 0.0;
        var_gtau_dn2 = 0.0;
        var_gtau_dn3 = 0.0;
        var_gtau_dn4 = 0.0;
        var_gtau_dn5 = 0.0;
        var_gtau_dn6 = 0.0;
        var_gtau_dn7 = 0.0;
        var_gtau_dn8 = 0.0;
        var_gtau_dn9 = 0.0;
        var_gtau_dn10 = 0.0;
        var_gtau_dn11 = 0.0;
        var_gtau_dn13 = 0.0;
        var_gtau_dn14 = 0.0;

        *var_agidlb_i_slot = var_agidlb_i;
        *var_agislb_i_slot = var_agislb_i;
        *var_bgidlb_i_slot = var_bgidlb_i;
        *var_bgidlb_t_slot = var_bgidlb_t;
        *var_bgidlb_t_dn4_slot = var_bgidlb_t_dn4;
        *var_bgislb_i_slot = var_bgislb_i;
        *var_bgislb_t_slot = var_bgislb_t;
        *var_bgislb_t_dn4_slot = var_bgislb_t_dn4;
        *var_cgidlb_i_slot = var_cgidlb_i;
        *var_cgislb_i_slot = var_cgislb_i;
        *var_cox_acc_slot = var_cox_acc;
        *var_cox_acc_dn0_slot = var_cox_acc_dn0;
        *var_cox_acc_dn10_slot = var_cox_acc_dn10;
        *var_cox_acc_dn11_slot = var_cox_acc_dn11;
        *var_cox_acc_dn13_slot = var_cox_acc_dn13;
        *var_cox_acc_dn14_slot = var_cox_acc_dn14;
        *var_cox_acc_dn2_slot = var_cox_acc_dn2;
        *var_cox_acc_dn3_slot = var_cox_acc_dn3;
        *var_cox_acc_dn4_slot = var_cox_acc_dn4;
        *var_cox_acc_dn5_slot = var_cox_acc_dn5;
        *var_cox_acc_dn6_slot = var_cox_acc_dn6;
        *var_cox_acc_dn7_slot = var_cox_acc_dn7;
        *var_cox_acc_dn8_slot = var_cox_acc_dn8;
        *var_cox_acc_dn9_slot = var_cox_acc_dn9;
        *var_czbd_slot = var_czbd;
        *var_czbd_dn4_slot = var_czbd_dn4;
        *var_czbdsw_slot = var_czbdsw;
        *var_czbdsw_dn4_slot = var_czbdsw_dn4;
        *var_czbdswg_slot = var_czbdswg;
        *var_czbdswg_dn4_slot = var_czbdswg_dn4;
        *var_czbs_slot = var_czbs;
        *var_czbs_dn4_slot = var_czbs_dn4;
        *var_czbssw_slot = var_czbssw;
        *var_czbssw_dn4_slot = var_czbssw_dn4;
        *var_czbsswg_slot = var_czbsswg;
        *var_czbsswg_dn4_slot = var_czbsswg_dn4;
        *var_dslpfwd_slot = var_dslpfwd;
        *var_dslpfwd_dn0_slot = var_dslpfwd_dn0;
        *var_dslpfwd_dn10_slot = var_dslpfwd_dn10;
        *var_dslpfwd_dn11_slot = var_dslpfwd_dn11;
        *var_dslpfwd_dn13_slot = var_dslpfwd_dn13;
        *var_dslpfwd_dn14_slot = var_dslpfwd_dn14;
        *var_dslpfwd_dn2_slot = var_dslpfwd_dn2;
        *var_dslpfwd_dn3_slot = var_dslpfwd_dn3;
        *var_dslpfwd_dn4_slot = var_dslpfwd_dn4;
        *var_dslpfwd_dn5_slot = var_dslpfwd_dn5;
        *var_dslpfwd_dn6_slot = var_dslpfwd_dn6;
        *var_dslpfwd_dn7_slot = var_dslpfwd_dn7;
        *var_dslpfwd_dn8_slot = var_dslpfwd_dn8;
        *var_dslpfwd_dn9_slot = var_dslpfwd_dn9;
        *var_dslprev_slot = var_dslprev;
        *var_dslprev_dn0_slot = var_dslprev_dn0;
        *var_dslprev_dn10_slot = var_dslprev_dn10;
        *var_dslprev_dn11_slot = var_dslprev_dn11;
        *var_dslprev_dn13_slot = var_dslprev_dn13;
        *var_dslprev_dn14_slot = var_dslprev_dn14;
        *var_dslprev_dn2_slot = var_dslprev_dn2;
        *var_dslprev_dn3_slot = var_dslprev_dn3;
        *var_dslprev_dn4_slot = var_dslprev_dn4;
        *var_dslprev_dn5_slot = var_dslprev_dn5;
        *var_dslprev_dn6_slot = var_dslprev_dn6;
        *var_dslprev_dn7_slot = var_dslprev_dn7;
        *var_dslprev_dn8_slot = var_dslprev_dn8;
        *var_dslprev_dn9_slot = var_dslprev_dn9;
        *var_egidlb_i_slot = var_egidlb_i;
        *var_egislb_i_slot = var_egislb_i;
        *var_gcrg_slot = var_gcrg;
        *var_gcrg_dn0_slot = var_gcrg_dn0;
        *var_gcrg_dn10_slot = var_gcrg_dn10;
        *var_gcrg_dn11_slot = var_gcrg_dn11;
        *var_gcrg_dn13_slot = var_gcrg_dn13;
        *var_gcrg_dn14_slot = var_gcrg_dn14;
        *var_gcrg_dn2_slot = var_gcrg_dn2;
        *var_gcrg_dn3_slot = var_gcrg_dn3;
        *var_gcrg_dn4_slot = var_gcrg_dn4;
        *var_gcrg_dn5_slot = var_gcrg_dn5;
        *var_gcrg_dn6_slot = var_gcrg_dn6;
        *var_gcrg_dn7_slot = var_gcrg_dn7;
        *var_gcrg_dn8_slot = var_gcrg_dn8;
        *var_gcrg_dn9_slot = var_gcrg_dn9;
        *var_gtau_slot = var_gtau;
        *var_gtau_dn0_slot = var_gtau_dn0;
        *var_gtau_dn10_slot = var_gtau_dn10;
        *var_gtau_dn11_slot = var_gtau_dn11;
        *var_gtau_dn13_slot = var_gtau_dn13;
        *var_gtau_dn14_slot = var_gtau_dn14;
        *var_gtau_dn2_slot = var_gtau_dn2;
        *var_gtau_dn3_slot = var_gtau_dn3;
        *var_gtau_dn4_slot = var_gtau_dn4;
        *var_gtau_dn5_slot = var_gtau_dn5;
        *var_gtau_dn6_slot = var_gtau_dn6;
        *var_gtau_dn7_slot = var_gtau_dn7;
        *var_gtau_dn8_slot = var_gtau_dn8;
        *var_gtau_dn9_slot = var_gtau_dn9;
        *var_ied_slot = var_ied;
        *var_ied_dn0_slot = var_ied_dn0;
        *var_ied_dn10_slot = var_ied_dn10;
        *var_ied_dn11_slot = var_ied_dn11;
        *var_ied_dn13_slot = var_ied_dn13;
        *var_ied_dn14_slot = var_ied_dn14;
        *var_ied_dn2_slot = var_ied_dn2;
        *var_ied_dn3_slot = var_ied_dn3;
        *var_ied_dn4_slot = var_ied_dn4;
        *var_ied_dn5_slot = var_ied_dn5;
        *var_ied_dn6_slot = var_ied_dn6;
        *var_ied_dn7_slot = var_ied_dn7;
        *var_ied_dn8_slot = var_ied_dn8;
        *var_ied_dn9_slot = var_ied_dn9;
        *var_ies_slot = var_ies;
        *var_ies_dn0_slot = var_ies_dn0;
        *var_ies_dn10_slot = var_ies_dn10;
        *var_ies_dn11_slot = var_ies_dn11;
        *var_ies_dn13_slot = var_ies_dn13;
        *var_ies_dn14_slot = var_ies_dn14;
        *var_ies_dn2_slot = var_ies_dn2;
        *var_ies_dn3_slot = var_ies_dn3;
        *var_ies_dn4_slot = var_ies_dn4;
        *var_ies_dn5_slot = var_ies_dn5;
        *var_ies_dn6_slot = var_ies_dn6;
        *var_ies_dn7_slot = var_ies_dn7;
        *var_ies_dn8_slot = var_ies_dn8;
        *var_ies_dn9_slot = var_ies_dn9;
        *var_igidl_v_slot = var_igidl_v;
        *var_igidl_v_dn0_slot = var_igidl_v_dn0;
        *var_igidl_v_dn10_slot = var_igidl_v_dn10;
        *var_igidl_v_dn11_slot = var_igidl_v_dn11;
        *var_igidl_v_dn13_slot = var_igidl_v_dn13;
        *var_igidl_v_dn14_slot = var_igidl_v_dn14;
        *var_igidl_v_dn2_slot = var_igidl_v_dn2;
        *var_igidl_v_dn3_slot = var_igidl_v_dn3;
        *var_igidl_v_dn4_slot = var_igidl_v_dn4;
        *var_igidl_v_dn5_slot = var_igidl_v_dn5;
        *var_igidl_v_dn6_slot = var_igidl_v_dn6;
        *var_igidl_v_dn7_slot = var_igidl_v_dn7;
        *var_igidl_v_dn8_slot = var_igidl_v_dn8;
        *var_igidl_v_dn9_slot = var_igidl_v_dn9;
        *var_igidlb_slot = var_igidlb;
        *var_igidlb_dn0_slot = var_igidlb_dn0;
        *var_igidlb_dn10_slot = var_igidlb_dn10;
        *var_igidlb_dn11_slot = var_igidlb_dn11;
        *var_igidlb_dn13_slot = var_igidlb_dn13;
        *var_igidlb_dn14_slot = var_igidlb_dn14;
        *var_igidlb_dn2_slot = var_igidlb_dn2;
        *var_igidlb_dn3_slot = var_igidlb_dn3;
        *var_igidlb_dn4_slot = var_igidlb_dn4;
        *var_igidlb_dn5_slot = var_igidlb_dn5;
        *var_igidlb_dn6_slot = var_igidlb_dn6;
        *var_igidlb_dn7_slot = var_igidlb_dn7;
        *var_igidlb_dn8_slot = var_igidlb_dn8;
        *var_igidlb_dn9_slot = var_igidlb_dn9;
        *var_igislb_slot = var_igislb;
        *var_igislb_dn0_slot = var_igislb_dn0;
        *var_igislb_dn10_slot = var_igislb_dn10;
        *var_igislb_dn11_slot = var_igislb_dn11;
        *var_igislb_dn13_slot = var_igislb_dn13;
        *var_igislb_dn14_slot = var_igislb_dn14;
        *var_igislb_dn2_slot = var_igislb_dn2;
        *var_igislb_dn3_slot = var_igislb_dn3;
        *var_igislb_dn4_slot = var_igislb_dn4;
        *var_igislb_dn5_slot = var_igislb_dn5;
        *var_igislb_dn6_slot = var_igislb_dn6;
        *var_igislb_dn7_slot = var_igislb_dn7;
        *var_igislb_dn8_slot = var_igislb_dn8;
        *var_igislb_dn9_slot = var_igislb_dn9;
        *var_iii_1_slot = var_iii_1;
        *var_iii_1_dn0_slot = var_iii_1_dn0;
        *var_iii_1_dn10_slot = var_iii_1_dn10;
        *var_iii_1_dn11_slot = var_iii_1_dn11;
        *var_iii_1_dn13_slot = var_iii_1_dn13;
        *var_iii_1_dn14_slot = var_iii_1_dn14;
        *var_iii_1_dn2_slot = var_iii_1_dn2;
        *var_iii_1_dn3_slot = var_iii_1_dn3;
        *var_iii_1_dn4_slot = var_iii_1_dn4;
        *var_iii_1_dn5_slot = var_iii_1_dn5;
        *var_iii_1_dn6_slot = var_iii_1_dn6;
        *var_iii_1_dn7_slot = var_iii_1_dn7;
        *var_iii_1_dn8_slot = var_iii_1_dn8;
        *var_iii_1_dn9_slot = var_iii_1_dn9;
        *var_isbd_slot = var_isbd;
        *var_isbd_dn0_slot = var_isbd_dn0;
        *var_isbd_dn10_slot = var_isbd_dn10;
        *var_isbd_dn11_slot = var_isbd_dn11;
        *var_isbd_dn13_slot = var_isbd_dn13;
        *var_isbd_dn14_slot = var_isbd_dn14;
        *var_isbd_dn2_slot = var_isbd_dn2;
        *var_isbd_dn3_slot = var_isbd_dn3;
        *var_isbd_dn4_slot = var_isbd_dn4;
        *var_isbd_dn5_slot = var_isbd_dn5;
        *var_isbd_dn6_slot = var_isbd_dn6;
        *var_isbd_dn7_slot = var_isbd_dn7;
        *var_isbd_dn8_slot = var_isbd_dn8;
        *var_isbd_dn9_slot = var_isbd_dn9;
        *var_isbs_slot = var_isbs;
        *var_isbs_dn0_slot = var_isbs_dn0;
        *var_isbs_dn10_slot = var_isbs_dn10;
        *var_isbs_dn11_slot = var_isbs_dn11;
        *var_isbs_dn13_slot = var_isbs_dn13;
        *var_isbs_dn14_slot = var_isbs_dn14;
        *var_isbs_dn2_slot = var_isbs_dn2;
        *var_isbs_dn3_slot = var_isbs_dn3;
        *var_isbs_dn4_slot = var_isbs_dn4;
        *var_isbs_dn5_slot = var_isbs_dn5;
        *var_isbs_dn6_slot = var_isbs_dn6;
        *var_isbs_dn7_slot = var_isbs_dn7;
        *var_isbs_dn8_slot = var_isbs_dn8;
        *var_isbs_dn9_slot = var_isbs_dn9;
        *var_ivjdmfwd_slot = var_ivjdmfwd;
        *var_ivjdmfwd_dn0_slot = var_ivjdmfwd_dn0;
        *var_ivjdmfwd_dn10_slot = var_ivjdmfwd_dn10;
        *var_ivjdmfwd_dn11_slot = var_ivjdmfwd_dn11;
        *var_ivjdmfwd_dn13_slot = var_ivjdmfwd_dn13;
        *var_ivjdmfwd_dn14_slot = var_ivjdmfwd_dn14;
        *var_ivjdmfwd_dn2_slot = var_ivjdmfwd_dn2;
        *var_ivjdmfwd_dn3_slot = var_ivjdmfwd_dn3;
        *var_ivjdmfwd_dn4_slot = var_ivjdmfwd_dn4;
        *var_ivjdmfwd_dn5_slot = var_ivjdmfwd_dn5;
        *var_ivjdmfwd_dn6_slot = var_ivjdmfwd_dn6;
        *var_ivjdmfwd_dn7_slot = var_ivjdmfwd_dn7;
        *var_ivjdmfwd_dn8_slot = var_ivjdmfwd_dn8;
        *var_ivjdmfwd_dn9_slot = var_ivjdmfwd_dn9;
        *var_ivjdmrev_slot = var_ivjdmrev;
        *var_ivjdmrev_dn0_slot = var_ivjdmrev_dn0;
        *var_ivjdmrev_dn10_slot = var_ivjdmrev_dn10;
        *var_ivjdmrev_dn11_slot = var_ivjdmrev_dn11;
        *var_ivjdmrev_dn13_slot = var_ivjdmrev_dn13;
        *var_ivjdmrev_dn14_slot = var_ivjdmrev_dn14;
        *var_ivjdmrev_dn2_slot = var_ivjdmrev_dn2;
        *var_ivjdmrev_dn3_slot = var_ivjdmrev_dn3;
        *var_ivjdmrev_dn4_slot = var_ivjdmrev_dn4;
        *var_ivjdmrev_dn5_slot = var_ivjdmrev_dn5;
        *var_ivjdmrev_dn6_slot = var_ivjdmrev_dn6;
        *var_ivjdmrev_dn7_slot = var_ivjdmrev_dn7;
        *var_ivjdmrev_dn8_slot = var_ivjdmrev_dn8;
        *var_ivjdmrev_dn9_slot = var_ivjdmrev_dn9;
        *var_ivjsmfwd_slot = var_ivjsmfwd;
        *var_ivjsmfwd_dn0_slot = var_ivjsmfwd_dn0;
        *var_ivjsmfwd_dn10_slot = var_ivjsmfwd_dn10;
        *var_ivjsmfwd_dn11_slot = var_ivjsmfwd_dn11;
        *var_ivjsmfwd_dn13_slot = var_ivjsmfwd_dn13;
        *var_ivjsmfwd_dn14_slot = var_ivjsmfwd_dn14;
        *var_ivjsmfwd_dn2_slot = var_ivjsmfwd_dn2;
        *var_ivjsmfwd_dn3_slot = var_ivjsmfwd_dn3;
        *var_ivjsmfwd_dn4_slot = var_ivjsmfwd_dn4;
        *var_ivjsmfwd_dn5_slot = var_ivjsmfwd_dn5;
        *var_ivjsmfwd_dn6_slot = var_ivjsmfwd_dn6;
        *var_ivjsmfwd_dn7_slot = var_ivjsmfwd_dn7;
        *var_ivjsmfwd_dn8_slot = var_ivjsmfwd_dn8;
        *var_ivjsmfwd_dn9_slot = var_ivjsmfwd_dn9;
        *var_ivjsmrev_slot = var_ivjsmrev;
        *var_ivjsmrev_dn0_slot = var_ivjsmrev_dn0;
        *var_ivjsmrev_dn10_slot = var_ivjsmrev_dn10;
        *var_ivjsmrev_dn11_slot = var_ivjsmrev_dn11;
        *var_ivjsmrev_dn13_slot = var_ivjsmrev_dn13;
        *var_ivjsmrev_dn14_slot = var_ivjsmrev_dn14;
        *var_ivjsmrev_dn2_slot = var_ivjsmrev_dn2;
        *var_ivjsmrev_dn3_slot = var_ivjsmrev_dn3;
        *var_ivjsmrev_dn4_slot = var_ivjsmrev_dn4;
        *var_ivjsmrev_dn5_slot = var_ivjsmrev_dn5;
        *var_ivjsmrev_dn6_slot = var_ivjsmrev_dn6;
        *var_ivjsmrev_dn7_slot = var_ivjsmrev_dn7;
        *var_ivjsmrev_dn8_slot = var_ivjsmrev_dn8;
        *var_ivjsmrev_dn9_slot = var_ivjsmrev_dn9;
        *var_nvtmd_slot = var_nvtmd;
        *var_nvtmd_dn4_slot = var_nvtmd_dn4;
        *var_nvtms_slot = var_nvtms;
        *var_nvtms_dn4_slot = var_nvtms_dn4;
        *var_pb21d_slot = var_pb21d;
        *var_pb21d_dn4_slot = var_pb21d_dn4;
        *var_pb21s_slot = var_pb21s;
        *var_pb21s_dn4_slot = var_pb21s_dn4;
        *var_pb22d_slot = var_pb22d;
        *var_pb22d_dn4_slot = var_pb22d_dn4;
        *var_pb22s_slot = var_pb22s;
        *var_pb22s_dn4_slot = var_pb22s_dn4;
        *var_pb23d_slot = var_pb23d;
        *var_pb23d_dn4_slot = var_pb23d_dn4;
        *var_pb23s_slot = var_pb23s;
        *var_pb23s_dn4_slot = var_pb23s_dn4;
        *var_pgidlb_i_slot = var_pgidlb_i;
        *var_pgislb_i_slot = var_pgislb_i;
        *var_qb_acc_slot = var_qb_acc;
        *var_qb_acc_dn0_slot = var_qb_acc_dn0;
        *var_qb_acc_dn10_slot = var_qb_acc_dn10;
        *var_qb_acc_dn11_slot = var_qb_acc_dn11;
        *var_qb_acc_dn13_slot = var_qb_acc_dn13;
        *var_qb_acc_dn14_slot = var_qb_acc_dn14;
        *var_qb_acc_dn2_slot = var_qb_acc_dn2;
        *var_qb_acc_dn3_slot = var_qb_acc_dn3;
        *var_qb_acc_dn4_slot = var_qb_acc_dn4;
        *var_qb_acc_dn5_slot = var_qb_acc_dn5;
        *var_qb_acc_dn6_slot = var_qb_acc_dn6;
        *var_qb_acc_dn7_slot = var_qb_acc_dn7;
        *var_qb_acc_dn8_slot = var_qb_acc_dn8;
        *var_qb_acc_dn9_slot = var_qb_acc_dn9;
        *var_qedj_slot = var_qedj;
        *var_qedj_dn3_slot = var_qedj_dn3;
        *var_qedj_dn4_slot = var_qedj_dn4;
        *var_qedj_dn5_slot = var_qedj_dn5;
        *var_qesj_slot = var_qesj;
        *var_qesj_dn3_slot = var_qesj_dn3;
        *var_qesj_dn4_slot = var_qesj_dn4;
        *var_qesj_dn6_slot = var_qesj_dn6;
        *var_qg_acc_slot = var_qg_acc;
        *var_qg_acc_dn0_slot = var_qg_acc_dn0;
        *var_qg_acc_dn10_slot = var_qg_acc_dn10;
        *var_qg_acc_dn11_slot = var_qg_acc_dn11;
        *var_qg_acc_dn13_slot = var_qg_acc_dn13;
        *var_qg_acc_dn14_slot = var_qg_acc_dn14;
        *var_qg_acc_dn2_slot = var_qg_acc_dn2;
        *var_qg_acc_dn3_slot = var_qg_acc_dn3;
        *var_qg_acc_dn4_slot = var_qg_acc_dn4;
        *var_qg_acc_dn5_slot = var_qg_acc_dn5;
        *var_qg_acc_dn6_slot = var_qg_acc_dn6;
        *var_qg_acc_dn7_slot = var_qg_acc_dn7;
        *var_qg_acc_dn8_slot = var_qg_acc_dn8;
        *var_qg_acc_dn9_slot = var_qg_acc_dn9;
        *var_qgd_ov_slot = var_qgd_ov;
        *var_qgd_ov_dn0_slot = var_qgd_ov_dn0;
        *var_qgd_ov_dn10_slot = var_qgd_ov_dn10;
        *var_qgd_ov_dn11_slot = var_qgd_ov_dn11;
        *var_qgd_ov_dn13_slot = var_qgd_ov_dn13;
        *var_qgd_ov_dn14_slot = var_qgd_ov_dn14;
        *var_qgd_ov_dn2_slot = var_qgd_ov_dn2;
        *var_qgd_ov_dn3_slot = var_qgd_ov_dn3;
        *var_qgd_ov_dn4_slot = var_qgd_ov_dn4;
        *var_qgd_ov_dn5_slot = var_qgd_ov_dn5;
        *var_qgd_ov_dn6_slot = var_qgd_ov_dn6;
        *var_qgd_ov_dn7_slot = var_qgd_ov_dn7;
        *var_qgd_ov_dn8_slot = var_qgd_ov_dn8;
        *var_qgd_ov_dn9_slot = var_qgd_ov_dn9;
        *var_qgs_ov_slot = var_qgs_ov;
        *var_qgs_ov_dn0_slot = var_qgs_ov_dn0;
        *var_qgs_ov_dn10_slot = var_qgs_ov_dn10;
        *var_qgs_ov_dn11_slot = var_qgs_ov_dn11;
        *var_qgs_ov_dn13_slot = var_qgs_ov_dn13;
        *var_qgs_ov_dn14_slot = var_qgs_ov_dn14;
        *var_qgs_ov_dn2_slot = var_qgs_ov_dn2;
        *var_qgs_ov_dn3_slot = var_qgs_ov_dn3;
        *var_qgs_ov_dn4_slot = var_qgs_ov_dn4;
        *var_qgs_ov_dn5_slot = var_qgs_ov_dn5;
        *var_qgs_ov_dn6_slot = var_qgs_ov_dn6;
        *var_qgs_ov_dn7_slot = var_qgs_ov_dn7;
        *var_qgs_ov_dn8_slot = var_qgs_ov_dn8;
        *var_qgs_ov_dn9_slot = var_qgs_ov_dn9;
        *var_sslpfwd_slot = var_sslpfwd;
        *var_sslpfwd_dn0_slot = var_sslpfwd_dn0;
        *var_sslpfwd_dn10_slot = var_sslpfwd_dn10;
        *var_sslpfwd_dn11_slot = var_sslpfwd_dn11;
        *var_sslpfwd_dn13_slot = var_sslpfwd_dn13;
        *var_sslpfwd_dn14_slot = var_sslpfwd_dn14;
        *var_sslpfwd_dn2_slot = var_sslpfwd_dn2;
        *var_sslpfwd_dn3_slot = var_sslpfwd_dn3;
        *var_sslpfwd_dn4_slot = var_sslpfwd_dn4;
        *var_sslpfwd_dn5_slot = var_sslpfwd_dn5;
        *var_sslpfwd_dn6_slot = var_sslpfwd_dn6;
        *var_sslpfwd_dn7_slot = var_sslpfwd_dn7;
        *var_sslpfwd_dn8_slot = var_sslpfwd_dn8;
        *var_sslpfwd_dn9_slot = var_sslpfwd_dn9;
        *var_sslprev_slot = var_sslprev;
        *var_sslprev_dn0_slot = var_sslprev_dn0;
        *var_sslprev_dn10_slot = var_sslprev_dn10;
        *var_sslprev_dn11_slot = var_sslprev_dn11;
        *var_sslprev_dn13_slot = var_sslprev_dn13;
        *var_sslprev_dn14_slot = var_sslprev_dn14;
        *var_sslprev_dn2_slot = var_sslprev_dn2;
        *var_sslprev_dn3_slot = var_sslprev_dn3;
        *var_sslprev_dn4_slot = var_sslprev_dn4;
        *var_sslprev_dn5_slot = var_sslprev_dn5;
        *var_sslprev_dn6_slot = var_sslprev_dn6;
        *var_sslprev_dn7_slot = var_sslprev_dn7;
        *var_sslprev_dn8_slot = var_sslprev_dn8;
        *var_sslprev_dn9_slot = var_sslprev_dn9;
        *var_vec1d_slot = var_vec1d;
        *var_vec1d_dn4_slot = var_vec1d_dn4;
        *var_vec1s_slot = var_vec1s;
        *var_vec1s_dn4_slot = var_vec1s_dn4;
        *var_vec2d_slot = var_vec2d;
        *var_vec2d_dn4_slot = var_vec2d_dn4;
        *var_vec2s_slot = var_vec2s;
        *var_vec2s_dn4_slot = var_vec2s_dn4;
        *var_vec3d_slot = var_vec3d;
        *var_vec3d_dn4_slot = var_vec3d_dn4;
        *var_vec3s_slot = var_vec3s;
        *var_vec3s_dn4_slot = var_vec3s_dn4;
        *var_vjdmfwd_slot = var_vjdmfwd;
        *var_vjdmfwd_dn0_slot = var_vjdmfwd_dn0;
        *var_vjdmfwd_dn10_slot = var_vjdmfwd_dn10;
        *var_vjdmfwd_dn11_slot = var_vjdmfwd_dn11;
        *var_vjdmfwd_dn13_slot = var_vjdmfwd_dn13;
        *var_vjdmfwd_dn14_slot = var_vjdmfwd_dn14;
        *var_vjdmfwd_dn2_slot = var_vjdmfwd_dn2;
        *var_vjdmfwd_dn3_slot = var_vjdmfwd_dn3;
        *var_vjdmfwd_dn4_slot = var_vjdmfwd_dn4;
        *var_vjdmfwd_dn5_slot = var_vjdmfwd_dn5;
        *var_vjdmfwd_dn6_slot = var_vjdmfwd_dn6;
        *var_vjdmfwd_dn7_slot = var_vjdmfwd_dn7;
        *var_vjdmfwd_dn8_slot = var_vjdmfwd_dn8;
        *var_vjdmfwd_dn9_slot = var_vjdmfwd_dn9;
        *var_vjdmrev_slot = var_vjdmrev;
        *var_vjdmrev_dn0_slot = var_vjdmrev_dn0;
        *var_vjdmrev_dn10_slot = var_vjdmrev_dn10;
        *var_vjdmrev_dn11_slot = var_vjdmrev_dn11;
        *var_vjdmrev_dn13_slot = var_vjdmrev_dn13;
        *var_vjdmrev_dn14_slot = var_vjdmrev_dn14;
        *var_vjdmrev_dn2_slot = var_vjdmrev_dn2;
        *var_vjdmrev_dn3_slot = var_vjdmrev_dn3;
        *var_vjdmrev_dn4_slot = var_vjdmrev_dn4;
        *var_vjdmrev_dn5_slot = var_vjdmrev_dn5;
        *var_vjdmrev_dn6_slot = var_vjdmrev_dn6;
        *var_vjdmrev_dn7_slot = var_vjdmrev_dn7;
        *var_vjdmrev_dn8_slot = var_vjdmrev_dn8;
        *var_vjdmrev_dn9_slot = var_vjdmrev_dn9;
        *var_vjsmfwd_slot = var_vjsmfwd;
        *var_vjsmfwd_dn0_slot = var_vjsmfwd_dn0;
        *var_vjsmfwd_dn10_slot = var_vjsmfwd_dn10;
        *var_vjsmfwd_dn11_slot = var_vjsmfwd_dn11;
        *var_vjsmfwd_dn13_slot = var_vjsmfwd_dn13;
        *var_vjsmfwd_dn14_slot = var_vjsmfwd_dn14;
        *var_vjsmfwd_dn2_slot = var_vjsmfwd_dn2;
        *var_vjsmfwd_dn3_slot = var_vjsmfwd_dn3;
        *var_vjsmfwd_dn4_slot = var_vjsmfwd_dn4;
        *var_vjsmfwd_dn5_slot = var_vjsmfwd_dn5;
        *var_vjsmfwd_dn6_slot = var_vjsmfwd_dn6;
        *var_vjsmfwd_dn7_slot = var_vjsmfwd_dn7;
        *var_vjsmfwd_dn8_slot = var_vjsmfwd_dn8;
        *var_vjsmfwd_dn9_slot = var_vjsmfwd_dn9;
        *var_vjsmrev_slot = var_vjsmrev;
        *var_vjsmrev_dn0_slot = var_vjsmrev_dn0;
        *var_vjsmrev_dn10_slot = var_vjsmrev_dn10;
        *var_vjsmrev_dn11_slot = var_vjsmrev_dn11;
        *var_vjsmrev_dn13_slot = var_vjsmrev_dn13;
        *var_vjsmrev_dn14_slot = var_vjsmrev_dn14;
        *var_vjsmrev_dn2_slot = var_vjsmrev_dn2;
        *var_vjsmrev_dn3_slot = var_vjsmrev_dn3;
        *var_vjsmrev_dn4_slot = var_vjsmrev_dn4;
        *var_vjsmrev_dn5_slot = var_vjsmrev_dn5;
        *var_vjsmrev_dn6_slot = var_vjsmrev_dn6;
        *var_vjsmrev_dn7_slot = var_vjsmrev_dn7;
        *var_vjsmrev_dn8_slot = var_vjsmrev_dn8;
        *var_vjsmrev_dn9_slot = var_vjsmrev_dn9;
        *var_xexpbvd_slot = var_xexpbvd;
        *var_xexpbvd_dn4_slot = var_xexpbvd_dn4;
        *var_xexpbvs_slot = var_xexpbvs;
        *var_xexpbvs_dn4_slot = var_xexpbvs_dn4;
    }

    pub(super) fn stamp_transient_block_2(
        var_ach_slot: &mut f64,
        var_ach2_slot: &mut f64,
        var_ach3_slot: &mut f64,
        var_ach4_slot: &mut f64,
        var_ach5_slot: &mut f64,
        var_ach6_slot: &mut f64,
        var_atr_i_slot: &mut f64,
        var_cdscdr_i_slot: &mut f64,
        var_cgdo_i_slot: &mut f64,
        var_cgso_i_slot: &mut f64,
        var_cins_slot: &mut f64,
        var_citr_i_slot: &mut f64,
        var_covd_i_slot: &mut f64,
        var_covd_i_dn0_slot: &mut f64,
        var_covd_i_dn10_slot: &mut f64,
        var_covd_i_dn11_slot: &mut f64,
        var_covd_i_dn13_slot: &mut f64,
        var_covd_i_dn14_slot: &mut f64,
        var_covd_i_dn2_slot: &mut f64,
        var_covd_i_dn3_slot: &mut f64,
        var_covd_i_dn4_slot: &mut f64,
        var_covd_i_dn5_slot: &mut f64,
        var_covd_i_dn6_slot: &mut f64,
        var_covd_i_dn7_slot: &mut f64,
        var_covd_i_dn8_slot: &mut f64,
        var_covd_i_dn9_slot: &mut f64,
        var_covs_i_slot: &mut f64,
        var_covs_i_dn0_slot: &mut f64,
        var_covs_i_dn10_slot: &mut f64,
        var_covs_i_dn11_slot: &mut f64,
        var_covs_i_dn13_slot: &mut f64,
        var_covs_i_dn14_slot: &mut f64,
        var_covs_i_dn2_slot: &mut f64,
        var_covs_i_dn3_slot: &mut f64,
        var_covs_i_dn4_slot: &mut f64,
        var_covs_i_dn5_slot: &mut f64,
        var_covs_i_dn6_slot: &mut f64,
        var_covs_i_dn7_slot: &mut f64,
        var_covs_i_dn8_slot: &mut f64,
        var_covs_i_dn9_slot: &mut f64,
        var_cth_slot: &mut f64,
        var_cth_dn0_slot: &mut f64,
        var_cth_dn10_slot: &mut f64,
        var_cth_dn11_slot: &mut f64,
        var_cth_dn13_slot: &mut f64,
        var_cth_dn14_slot: &mut f64,
        var_cth_dn2_slot: &mut f64,
        var_cth_dn3_slot: &mut f64,
        var_cth_dn4_slot: &mut f64,
        var_cth_dn5_slot: &mut f64,
        var_cth_dn6_slot: &mut f64,
        var_cth_dn7_slot: &mut f64,
        var_cth_dn8_slot: &mut f64,
        var_cth_dn9_slot: &mut f64,
        var_ctnoi_slot: &mut f64,
        var_ctnoi_dn0_slot: &mut f64,
        var_ctnoi_dn10_slot: &mut f64,
        var_ctnoi_dn11_slot: &mut f64,
        var_ctnoi_dn13_slot: &mut f64,
        var_ctnoi_dn14_slot: &mut f64,
        var_ctnoi_dn2_slot: &mut f64,
        var_ctnoi_dn3_slot: &mut f64,
        var_ctnoi_dn4_slot: &mut f64,
        var_ctnoi_dn5_slot: &mut f64,
        var_ctnoi_dn6_slot: &mut f64,
        var_ctnoi_dn7_slot: &mut f64,
        var_ctnoi_dn8_slot: &mut f64,
        var_ctnoi_dn9_slot: &mut f64,
        var_dr0_slot: &mut f64,
        var_dr0_dn0_slot: &mut f64,
        var_dr0_dn10_slot: &mut f64,
        var_dr0_dn11_slot: &mut f64,
        var_dr0_dn13_slot: &mut f64,
        var_dr0_dn14_slot: &mut f64,
        var_dr0_dn2_slot: &mut f64,
        var_dr0_dn3_slot: &mut f64,
        var_dr0_dn4_slot: &mut f64,
        var_dr0_dn5_slot: &mut f64,
        var_dr0_dn6_slot: &mut f64,
        var_dr0_dn7_slot: &mut f64,
        var_dr0_dn8_slot: &mut f64,
        var_dr0_dn9_slot: &mut f64,
        var_dvtshiftr_i_slot: &mut f64,
        var_eta0r_i_slot: &mut f64,
        var_eur_i_slot: &mut f64,
        var_eur_i_dn0_slot: &mut f64,
        var_eur_i_dn10_slot: &mut f64,
        var_eur_i_dn11_slot: &mut f64,
        var_eur_i_dn13_slot: &mut f64,
        var_eur_i_dn14_slot: &mut f64,
        var_eur_i_dn2_slot: &mut f64,
        var_eur_i_dn3_slot: &mut f64,
        var_eur_i_dn4_slot: &mut f64,
        var_eur_i_dn5_slot: &mut f64,
        var_eur_i_dn6_slot: &mut f64,
        var_eur_i_dn7_slot: &mut f64,
        var_eur_i_dn8_slot: &mut f64,
        var_eur_i_dn9_slot: &mut f64,
        var_gth_slot: &mut f64,
        var_gth_dn0_slot: &mut f64,
        var_gth_dn10_slot: &mut f64,
        var_gth_dn11_slot: &mut f64,
        var_gth_dn13_slot: &mut f64,
        var_gth_dn14_slot: &mut f64,
        var_gth_dn2_slot: &mut f64,
        var_gth_dn3_slot: &mut f64,
        var_gth_dn4_slot: &mut f64,
        var_gth_dn5_slot: &mut f64,
        var_gth_dn6_slot: &mut f64,
        var_gth_dn7_slot: &mut f64,
        var_gth_dn8_slot: &mut f64,
        var_gth_dn9_slot: &mut f64,
        var_k11_i_slot: &mut f64,
        var_k1_i_slot: &mut f64,
        var_k21_i_slot: &mut f64,
        var_k2_i_slot: &mut f64,
        var_k2sat1_i_slot: &mut f64,
        var_k2sat_i_slot: &mut f64,
        var_ksativr_i_slot: &mut f64,
        var_lambdac_fn2_slot: &mut f64,
        var_lambdac_fn2_dn0_slot: &mut f64,
        var_lambdac_fn2_dn10_slot: &mut f64,
        var_lambdac_fn2_dn11_slot: &mut f64,
        var_lambdac_fn2_dn13_slot: &mut f64,
        var_lambdac_fn2_dn14_slot: &mut f64,
        var_lambdac_fn2_dn2_slot: &mut f64,
        var_lambdac_fn2_dn3_slot: &mut f64,
        var_lambdac_fn2_dn4_slot: &mut f64,
        var_lambdac_fn2_dn5_slot: &mut f64,
        var_lambdac_fn2_dn6_slot: &mut f64,
        var_lambdac_fn2_dn7_slot: &mut f64,
        var_lambdac_fn2_dn8_slot: &mut f64,
        var_lambdac_fn2_dn9_slot: &mut f64,
        var_mexpr_i_slot: &mut f64,
        var_mexpr_i_dn0_slot: &mut f64,
        var_mexpr_i_dn10_slot: &mut f64,
        var_mexpr_i_dn11_slot: &mut f64,
        var_mexpr_i_dn13_slot: &mut f64,
        var_mexpr_i_dn14_slot: &mut f64,
        var_mexpr_i_dn2_slot: &mut f64,
        var_mexpr_i_dn3_slot: &mut f64,
        var_mexpr_i_dn4_slot: &mut f64,
        var_mexpr_i_dn5_slot: &mut f64,
        var_mexpr_i_dn6_slot: &mut f64,
        var_mexpr_i_dn7_slot: &mut f64,
        var_mexpr_i_dn8_slot: &mut f64,
        var_mexpr_i_dn9_slot: &mut f64,
        var_mpower_i_slot: &mut f64,
        var_noia2_i_slot: &mut f64,
        var_nq_slot: &mut f64,
        var_nq_dn0_slot: &mut f64,
        var_nq_dn10_slot: &mut f64,
        var_nq_dn11_slot: &mut f64,
        var_nq_dn13_slot: &mut f64,
        var_nq_dn14_slot: &mut f64,
        var_nq_dn2_slot: &mut f64,
        var_nq_dn3_slot: &mut f64,
        var_nq_dn4_slot: &mut f64,
        var_nq_dn5_slot: &mut f64,
        var_nq_dn6_slot: &mut f64,
        var_nq_dn7_slot: &mut f64,
        var_nq_dn8_slot: &mut f64,
        var_nq_dn9_slot: &mut f64,
        var_pclmr_i_slot: &mut f64,
        var_pclmr_i_dn0_slot: &mut f64,
        var_pclmr_i_dn10_slot: &mut f64,
        var_pclmr_i_dn11_slot: &mut f64,
        var_pclmr_i_dn13_slot: &mut f64,
        var_pclmr_i_dn14_slot: &mut f64,
        var_pclmr_i_dn2_slot: &mut f64,
        var_pclmr_i_dn3_slot: &mut f64,
        var_pclmr_i_dn4_slot: &mut f64,
        var_pclmr_i_dn5_slot: &mut f64,
        var_pclmr_i_dn6_slot: &mut f64,
        var_pclmr_i_dn7_slot: &mut f64,
        var_pclmr_i_dn8_slot: &mut f64,
        var_pclmr_i_dn9_slot: &mut f64,
        var_pdibl1r_i_slot: &mut f64,
        var_pdibl2r_i_slot: &mut f64,
        var_phibe_i_slot: &mut f64,
        var_ptwgr_i_slot: &mut f64,
        var_ptwgr_i_dn0_slot: &mut f64,
        var_ptwgr_i_dn10_slot: &mut f64,
        var_ptwgr_i_dn11_slot: &mut f64,
        var_ptwgr_i_dn13_slot: &mut f64,
        var_ptwgr_i_dn14_slot: &mut f64,
        var_ptwgr_i_dn2_slot: &mut f64,
        var_ptwgr_i_dn3_slot: &mut f64,
        var_ptwgr_i_dn4_slot: &mut f64,
        var_ptwgr_i_dn5_slot: &mut f64,
        var_ptwgr_i_dn6_slot: &mut f64,
        var_ptwgr_i_dn7_slot: &mut f64,
        var_ptwgr_i_dn8_slot: &mut f64,
        var_ptwgr_i_dn9_slot: &mut f64,
        var_qbov_slot: &mut f64,
        var_qbov_dn0_slot: &mut f64,
        var_qbov_dn10_slot: &mut f64,
        var_qbov_dn11_slot: &mut f64,
        var_qbov_dn13_slot: &mut f64,
        var_qbov_dn14_slot: &mut f64,
        var_qbov_dn2_slot: &mut f64,
        var_qbov_dn3_slot: &mut f64,
        var_qbov_dn4_slot: &mut f64,
        var_qbov_dn5_slot: &mut f64,
        var_qbov_dn6_slot: &mut f64,
        var_qbov_dn7_slot: &mut f64,
        var_qbov_dn8_slot: &mut f64,
        var_qbov_dn9_slot: &mut f64,
        var_qbov_s_slot: &mut f64,
        var_qbov_s_dn0_slot: &mut f64,
        var_qbov_s_dn10_slot: &mut f64,
        var_qbov_s_dn11_slot: &mut f64,
        var_qbov_s_dn13_slot: &mut f64,
        var_qbov_s_dn14_slot: &mut f64,
        var_qbov_s_dn2_slot: &mut f64,
        var_qbov_s_dn3_slot: &mut f64,
        var_qbov_s_dn4_slot: &mut f64,
        var_qbov_s_dn5_slot: &mut f64,
        var_qbov_s_dn6_slot: &mut f64,
        var_qbov_s_dn7_slot: &mut f64,
        var_qbov_s_dn8_slot: &mut f64,
        var_qbov_s_dn9_slot: &mut f64,
        var_qdep_ov_cins_slot: &mut f64,
        var_qi_acc_for_qm_slot: &mut f64,
        var_qi_acc_for_qm_dn0_slot: &mut f64,
        var_qi_acc_for_qm_dn10_slot: &mut f64,
        var_qi_acc_for_qm_dn11_slot: &mut f64,
        var_qi_acc_for_qm_dn13_slot: &mut f64,
        var_qi_acc_for_qm_dn14_slot: &mut f64,
        var_qi_acc_for_qm_dn2_slot: &mut f64,
        var_qi_acc_for_qm_dn3_slot: &mut f64,
        var_qi_acc_for_qm_dn4_slot: &mut f64,
        var_qi_acc_for_qm_dn5_slot: &mut f64,
        var_qi_acc_for_qm_dn6_slot: &mut f64,
        var_qi_acc_for_qm_dn7_slot: &mut f64,
        var_qi_acc_for_qm_dn8_slot: &mut f64,
        var_qi_acc_for_qm_dn9_slot: &mut f64,
        var_qid_slot: &mut f64,
        var_qid_dn0_slot: &mut f64,
        var_qid_dn10_slot: &mut f64,
        var_qid_dn11_slot: &mut f64,
        var_qid_dn13_slot: &mut f64,
        var_qid_dn14_slot: &mut f64,
        var_qid_dn2_slot: &mut f64,
        var_qid_dn3_slot: &mut f64,
        var_qid_dn4_slot: &mut f64,
        var_qid_dn5_slot: &mut f64,
        var_qid_dn6_slot: &mut f64,
        var_qid_dn7_slot: &mut f64,
        var_qid_dn8_slot: &mut f64,
        var_qid_dn9_slot: &mut f64,
        var_qis_slot: &mut f64,
        var_qis_dn0_slot: &mut f64,
        var_qis_dn10_slot: &mut f64,
        var_qis_dn11_slot: &mut f64,
        var_qis_dn13_slot: &mut f64,
        var_qis_dn14_slot: &mut f64,
        var_qis_dn2_slot: &mut f64,
        var_qis_dn3_slot: &mut f64,
        var_qis_dn4_slot: &mut f64,
        var_qis_dn5_slot: &mut f64,
        var_qis_dn6_slot: &mut f64,
        var_qis_dn7_slot: &mut f64,
        var_qis_dn8_slot: &mut f64,
        var_qis_dn9_slot: &mut f64,
        var_qndd1_slot: &mut f64,
        var_qndd1_dn0_slot: &mut f64,
        var_qndd1_dn10_slot: &mut f64,
        var_qndd1_dn11_slot: &mut f64,
        var_qndd1_dn13_slot: &mut f64,
        var_qndd1_dn14_slot: &mut f64,
        var_qndd1_dn2_slot: &mut f64,
        var_qndd1_dn3_slot: &mut f64,
        var_qndd1_dn4_slot: &mut f64,
        var_qndd1_dn5_slot: &mut f64,
        var_qndd1_dn6_slot: &mut f64,
        var_qndd1_dn7_slot: &mut f64,
        var_qndd1_dn8_slot: &mut f64,
        var_qndd1_dn9_slot: &mut f64,
        var_qnds1_slot: &mut f64,
        var_qnds1_dn0_slot: &mut f64,
        var_qnds1_dn10_slot: &mut f64,
        var_qnds1_dn11_slot: &mut f64,
        var_qnds1_dn13_slot: &mut f64,
        var_qnds1_dn14_slot: &mut f64,
        var_qnds1_dn2_slot: &mut f64,
        var_qnds1_dn3_slot: &mut f64,
        var_qnds1_dn4_slot: &mut f64,
        var_qnds1_dn5_slot: &mut f64,
        var_qnds1_dn6_slot: &mut f64,
        var_qnds1_dn7_slot: &mut f64,
        var_qnds1_dn8_slot: &mut f64,
        var_qnds1_dn9_slot: &mut f64,
        var_qnds2_slot: &mut f64,
        var_qnds2_dn0_slot: &mut f64,
        var_qnds2_dn10_slot: &mut f64,
        var_qnds2_dn11_slot: &mut f64,
        var_qnds2_dn13_slot: &mut f64,
        var_qnds2_dn14_slot: &mut f64,
        var_qnds2_dn2_slot: &mut f64,
        var_qnds2_dn3_slot: &mut f64,
        var_qnds2_dn4_slot: &mut f64,
        var_qnds2_dn5_slot: &mut f64,
        var_qnds2_dn6_slot: &mut f64,
        var_qnds2_dn7_slot: &mut f64,
        var_qnds2_dn8_slot: &mut f64,
        var_qnds2_dn9_slot: &mut f64,
        var_qnds3_slot: &mut f64,
        var_qnds3_dn0_slot: &mut f64,
        var_qnds3_dn10_slot: &mut f64,
        var_qnds3_dn11_slot: &mut f64,
        var_qnds3_dn13_slot: &mut f64,
        var_qnds3_dn14_slot: &mut f64,
        var_qnds3_dn2_slot: &mut f64,
        var_qnds3_dn3_slot: &mut f64,
        var_qnds3_dn4_slot: &mut f64,
        var_qnds3_dn5_slot: &mut f64,
        var_qnds3_dn6_slot: &mut f64,
        var_qnds3_dn7_slot: &mut f64,
        var_qnds3_dn8_slot: &mut f64,
        var_qnds3_dn9_slot: &mut f64,
        var_rc_slot: &mut f64,
        var_sigrat_slot: &mut f64,
        var_sigrat_dn0_slot: &mut f64,
        var_sigrat_dn10_slot: &mut f64,
        var_sigrat_dn11_slot: &mut f64,
        var_sigrat_dn13_slot: &mut f64,
        var_sigrat_dn14_slot: &mut f64,
        var_sigrat_dn2_slot: &mut f64,
        var_sigrat_dn3_slot: &mut f64,
        var_sigrat_dn4_slot: &mut f64,
        var_sigrat_dn5_slot: &mut f64,
        var_sigrat_dn6_slot: &mut f64,
        var_sigrat_dn7_slot: &mut f64,
        var_sigrat_dn8_slot: &mut f64,
        var_sigrat_dn9_slot: &mut f64,
        var_u0r_i_slot: &mut f64,
        var_u0r_i_dn0_slot: &mut f64,
        var_u0r_i_dn10_slot: &mut f64,
        var_u0r_i_dn11_slot: &mut f64,
        var_u0r_i_dn13_slot: &mut f64,
        var_u0r_i_dn14_slot: &mut f64,
        var_u0r_i_dn2_slot: &mut f64,
        var_u0r_i_dn3_slot: &mut f64,
        var_u0r_i_dn4_slot: &mut f64,
        var_u0r_i_dn5_slot: &mut f64,
        var_u0r_i_dn6_slot: &mut f64,
        var_u0r_i_dn7_slot: &mut f64,
        var_u0r_i_dn8_slot: &mut f64,
        var_u0r_i_dn9_slot: &mut f64,
        var_ua1r_i_slot: &mut f64,
        var_uar_i_slot: &mut f64,
        var_uar_i_dn0_slot: &mut f64,
        var_uar_i_dn10_slot: &mut f64,
        var_uar_i_dn11_slot: &mut f64,
        var_uar_i_dn13_slot: &mut f64,
        var_uar_i_dn14_slot: &mut f64,
        var_uar_i_dn2_slot: &mut f64,
        var_uar_i_dn3_slot: &mut f64,
        var_uar_i_dn4_slot: &mut f64,
        var_uar_i_dn5_slot: &mut f64,
        var_uar_i_dn6_slot: &mut f64,
        var_uar_i_dn7_slot: &mut f64,
        var_uar_i_dn8_slot: &mut f64,
        var_uar_i_dn9_slot: &mut f64,
        var_uc1_i_slot: &mut f64,
        var_uc1r_i_slot: &mut f64,
        var_uc_i_slot: &mut f64,
        var_ucr_i_slot: &mut f64,
        var_ud1r_i_slot: &mut f64,
        var_udr_i_slot: &mut f64,
        var_udr_i_dn0_slot: &mut f64,
        var_udr_i_dn10_slot: &mut f64,
        var_udr_i_dn11_slot: &mut f64,
        var_udr_i_dn13_slot: &mut f64,
        var_udr_i_dn14_slot: &mut f64,
        var_udr_i_dn2_slot: &mut f64,
        var_udr_i_dn3_slot: &mut f64,
        var_udr_i_dn4_slot: &mut f64,
        var_udr_i_dn5_slot: &mut f64,
        var_udr_i_dn6_slot: &mut f64,
        var_udr_i_dn7_slot: &mut f64,
        var_udr_i_dn8_slot: &mut f64,
        var_udr_i_dn9_slot: &mut f64,
        var_upr_i_slot: &mut f64,
        var_uter_i_slot: &mut f64,
        var_utlr_i_slot: &mut f64,
        var_veseff_slot: &mut f64,
        var_veseff_dn0_slot: &mut f64,
        var_veseff_dn10_slot: &mut f64,
        var_veseff_dn11_slot: &mut f64,
        var_veseff_dn13_slot: &mut f64,
        var_veseff_dn14_slot: &mut f64,
        var_veseff_dn2_slot: &mut f64,
        var_veseff_dn3_slot: &mut f64,
        var_veseff_dn4_slot: &mut f64,
        var_veseff_dn5_slot: &mut f64,
        var_veseff_dn6_slot: &mut f64,
        var_veseff_dn7_slot: &mut f64,
        var_veseff_dn8_slot: &mut f64,
        var_veseff_dn9_slot: &mut f64,
        var_vsat1r_i_slot: &mut f64,
        var_vsat1r_i_dn0_slot: &mut f64,
        var_vsat1r_i_dn10_slot: &mut f64,
        var_vsat1r_i_dn11_slot: &mut f64,
        var_vsat1r_i_dn13_slot: &mut f64,
        var_vsat1r_i_dn14_slot: &mut f64,
        var_vsat1r_i_dn2_slot: &mut f64,
        var_vsat1r_i_dn3_slot: &mut f64,
        var_vsat1r_i_dn4_slot: &mut f64,
        var_vsat1r_i_dn5_slot: &mut f64,
        var_vsat1r_i_dn6_slot: &mut f64,
        var_vsat1r_i_dn7_slot: &mut f64,
        var_vsat1r_i_dn8_slot: &mut f64,
        var_vsat1r_i_dn9_slot: &mut f64,
        var_vsatr_i_slot: &mut f64,
        var_weff2_slot: &mut f64,
        var_weff3_slot: &mut f64,
        var_weff4_slot: &mut f64,
        var_weff5_slot: &mut f64,
        var_weff6_slot: &mut f64,
        var_weff_ufcm_slot: &mut f64,
        var_weffb_slot: &mut f64,
        var_xrcrg1_i_slot: &mut f64,
        var_xrcrg2_i_slot: &mut f64,
    ) {
        let mut var_ach: f64 = *var_ach_slot;
        let mut var_ach2: f64 = *var_ach2_slot;
        let mut var_ach3: f64 = *var_ach3_slot;
        let mut var_ach4: f64 = *var_ach4_slot;
        let mut var_ach5: f64 = *var_ach5_slot;
        let mut var_ach6: f64 = *var_ach6_slot;
        let mut var_atr_i: f64 = *var_atr_i_slot;
        let mut var_cdscdr_i: f64 = *var_cdscdr_i_slot;
        let mut var_cgdo_i: f64 = *var_cgdo_i_slot;
        let mut var_cgso_i: f64 = *var_cgso_i_slot;
        let mut var_cins: f64 = *var_cins_slot;
        let mut var_citr_i: f64 = *var_citr_i_slot;
        let mut var_covd_i: f64 = *var_covd_i_slot;
        let mut var_covd_i_dn0: f64 = *var_covd_i_dn0_slot;
        let mut var_covd_i_dn10: f64 = *var_covd_i_dn10_slot;
        let mut var_covd_i_dn11: f64 = *var_covd_i_dn11_slot;
        let mut var_covd_i_dn13: f64 = *var_covd_i_dn13_slot;
        let mut var_covd_i_dn14: f64 = *var_covd_i_dn14_slot;
        let mut var_covd_i_dn2: f64 = *var_covd_i_dn2_slot;
        let mut var_covd_i_dn3: f64 = *var_covd_i_dn3_slot;
        let mut var_covd_i_dn4: f64 = *var_covd_i_dn4_slot;
        let mut var_covd_i_dn5: f64 = *var_covd_i_dn5_slot;
        let mut var_covd_i_dn6: f64 = *var_covd_i_dn6_slot;
        let mut var_covd_i_dn7: f64 = *var_covd_i_dn7_slot;
        let mut var_covd_i_dn8: f64 = *var_covd_i_dn8_slot;
        let mut var_covd_i_dn9: f64 = *var_covd_i_dn9_slot;
        let mut var_covs_i: f64 = *var_covs_i_slot;
        let mut var_covs_i_dn0: f64 = *var_covs_i_dn0_slot;
        let mut var_covs_i_dn10: f64 = *var_covs_i_dn10_slot;
        let mut var_covs_i_dn11: f64 = *var_covs_i_dn11_slot;
        let mut var_covs_i_dn13: f64 = *var_covs_i_dn13_slot;
        let mut var_covs_i_dn14: f64 = *var_covs_i_dn14_slot;
        let mut var_covs_i_dn2: f64 = *var_covs_i_dn2_slot;
        let mut var_covs_i_dn3: f64 = *var_covs_i_dn3_slot;
        let mut var_covs_i_dn4: f64 = *var_covs_i_dn4_slot;
        let mut var_covs_i_dn5: f64 = *var_covs_i_dn5_slot;
        let mut var_covs_i_dn6: f64 = *var_covs_i_dn6_slot;
        let mut var_covs_i_dn7: f64 = *var_covs_i_dn7_slot;
        let mut var_covs_i_dn8: f64 = *var_covs_i_dn8_slot;
        let mut var_covs_i_dn9: f64 = *var_covs_i_dn9_slot;
        let mut var_cth: f64 = *var_cth_slot;
        let mut var_cth_dn0: f64 = *var_cth_dn0_slot;
        let mut var_cth_dn10: f64 = *var_cth_dn10_slot;
        let mut var_cth_dn11: f64 = *var_cth_dn11_slot;
        let mut var_cth_dn13: f64 = *var_cth_dn13_slot;
        let mut var_cth_dn14: f64 = *var_cth_dn14_slot;
        let mut var_cth_dn2: f64 = *var_cth_dn2_slot;
        let mut var_cth_dn3: f64 = *var_cth_dn3_slot;
        let mut var_cth_dn4: f64 = *var_cth_dn4_slot;
        let mut var_cth_dn5: f64 = *var_cth_dn5_slot;
        let mut var_cth_dn6: f64 = *var_cth_dn6_slot;
        let mut var_cth_dn7: f64 = *var_cth_dn7_slot;
        let mut var_cth_dn8: f64 = *var_cth_dn8_slot;
        let mut var_cth_dn9: f64 = *var_cth_dn9_slot;
        let mut var_ctnoi: f64 = *var_ctnoi_slot;
        let mut var_ctnoi_dn0: f64 = *var_ctnoi_dn0_slot;
        let mut var_ctnoi_dn10: f64 = *var_ctnoi_dn10_slot;
        let mut var_ctnoi_dn11: f64 = *var_ctnoi_dn11_slot;
        let mut var_ctnoi_dn13: f64 = *var_ctnoi_dn13_slot;
        let mut var_ctnoi_dn14: f64 = *var_ctnoi_dn14_slot;
        let mut var_ctnoi_dn2: f64 = *var_ctnoi_dn2_slot;
        let mut var_ctnoi_dn3: f64 = *var_ctnoi_dn3_slot;
        let mut var_ctnoi_dn4: f64 = *var_ctnoi_dn4_slot;
        let mut var_ctnoi_dn5: f64 = *var_ctnoi_dn5_slot;
        let mut var_ctnoi_dn6: f64 = *var_ctnoi_dn6_slot;
        let mut var_ctnoi_dn7: f64 = *var_ctnoi_dn7_slot;
        let mut var_ctnoi_dn8: f64 = *var_ctnoi_dn8_slot;
        let mut var_ctnoi_dn9: f64 = *var_ctnoi_dn9_slot;
        let mut var_dr0: f64 = *var_dr0_slot;
        let mut var_dr0_dn0: f64 = *var_dr0_dn0_slot;
        let mut var_dr0_dn10: f64 = *var_dr0_dn10_slot;
        let mut var_dr0_dn11: f64 = *var_dr0_dn11_slot;
        let mut var_dr0_dn13: f64 = *var_dr0_dn13_slot;
        let mut var_dr0_dn14: f64 = *var_dr0_dn14_slot;
        let mut var_dr0_dn2: f64 = *var_dr0_dn2_slot;
        let mut var_dr0_dn3: f64 = *var_dr0_dn3_slot;
        let mut var_dr0_dn4: f64 = *var_dr0_dn4_slot;
        let mut var_dr0_dn5: f64 = *var_dr0_dn5_slot;
        let mut var_dr0_dn6: f64 = *var_dr0_dn6_slot;
        let mut var_dr0_dn7: f64 = *var_dr0_dn7_slot;
        let mut var_dr0_dn8: f64 = *var_dr0_dn8_slot;
        let mut var_dr0_dn9: f64 = *var_dr0_dn9_slot;
        let mut var_dvtshiftr_i: f64 = *var_dvtshiftr_i_slot;
        let mut var_eta0r_i: f64 = *var_eta0r_i_slot;
        let mut var_eur_i: f64 = *var_eur_i_slot;
        let mut var_eur_i_dn0: f64 = *var_eur_i_dn0_slot;
        let mut var_eur_i_dn10: f64 = *var_eur_i_dn10_slot;
        let mut var_eur_i_dn11: f64 = *var_eur_i_dn11_slot;
        let mut var_eur_i_dn13: f64 = *var_eur_i_dn13_slot;
        let mut var_eur_i_dn14: f64 = *var_eur_i_dn14_slot;
        let mut var_eur_i_dn2: f64 = *var_eur_i_dn2_slot;
        let mut var_eur_i_dn3: f64 = *var_eur_i_dn3_slot;
        let mut var_eur_i_dn4: f64 = *var_eur_i_dn4_slot;
        let mut var_eur_i_dn5: f64 = *var_eur_i_dn5_slot;
        let mut var_eur_i_dn6: f64 = *var_eur_i_dn6_slot;
        let mut var_eur_i_dn7: f64 = *var_eur_i_dn7_slot;
        let mut var_eur_i_dn8: f64 = *var_eur_i_dn8_slot;
        let mut var_eur_i_dn9: f64 = *var_eur_i_dn9_slot;
        let mut var_gth: f64 = *var_gth_slot;
        let mut var_gth_dn0: f64 = *var_gth_dn0_slot;
        let mut var_gth_dn10: f64 = *var_gth_dn10_slot;
        let mut var_gth_dn11: f64 = *var_gth_dn11_slot;
        let mut var_gth_dn13: f64 = *var_gth_dn13_slot;
        let mut var_gth_dn14: f64 = *var_gth_dn14_slot;
        let mut var_gth_dn2: f64 = *var_gth_dn2_slot;
        let mut var_gth_dn3: f64 = *var_gth_dn3_slot;
        let mut var_gth_dn4: f64 = *var_gth_dn4_slot;
        let mut var_gth_dn5: f64 = *var_gth_dn5_slot;
        let mut var_gth_dn6: f64 = *var_gth_dn6_slot;
        let mut var_gth_dn7: f64 = *var_gth_dn7_slot;
        let mut var_gth_dn8: f64 = *var_gth_dn8_slot;
        let mut var_gth_dn9: f64 = *var_gth_dn9_slot;
        let mut var_k11_i: f64 = *var_k11_i_slot;
        let mut var_k1_i: f64 = *var_k1_i_slot;
        let mut var_k21_i: f64 = *var_k21_i_slot;
        let mut var_k2_i: f64 = *var_k2_i_slot;
        let mut var_k2sat1_i: f64 = *var_k2sat1_i_slot;
        let mut var_k2sat_i: f64 = *var_k2sat_i_slot;
        let mut var_ksativr_i: f64 = *var_ksativr_i_slot;
        let mut var_lambdac_fn2: f64 = *var_lambdac_fn2_slot;
        let mut var_lambdac_fn2_dn0: f64 = *var_lambdac_fn2_dn0_slot;
        let mut var_lambdac_fn2_dn10: f64 = *var_lambdac_fn2_dn10_slot;
        let mut var_lambdac_fn2_dn11: f64 = *var_lambdac_fn2_dn11_slot;
        let mut var_lambdac_fn2_dn13: f64 = *var_lambdac_fn2_dn13_slot;
        let mut var_lambdac_fn2_dn14: f64 = *var_lambdac_fn2_dn14_slot;
        let mut var_lambdac_fn2_dn2: f64 = *var_lambdac_fn2_dn2_slot;
        let mut var_lambdac_fn2_dn3: f64 = *var_lambdac_fn2_dn3_slot;
        let mut var_lambdac_fn2_dn4: f64 = *var_lambdac_fn2_dn4_slot;
        let mut var_lambdac_fn2_dn5: f64 = *var_lambdac_fn2_dn5_slot;
        let mut var_lambdac_fn2_dn6: f64 = *var_lambdac_fn2_dn6_slot;
        let mut var_lambdac_fn2_dn7: f64 = *var_lambdac_fn2_dn7_slot;
        let mut var_lambdac_fn2_dn8: f64 = *var_lambdac_fn2_dn8_slot;
        let mut var_lambdac_fn2_dn9: f64 = *var_lambdac_fn2_dn9_slot;
        let mut var_mexpr_i: f64 = *var_mexpr_i_slot;
        let mut var_mexpr_i_dn0: f64 = *var_mexpr_i_dn0_slot;
        let mut var_mexpr_i_dn10: f64 = *var_mexpr_i_dn10_slot;
        let mut var_mexpr_i_dn11: f64 = *var_mexpr_i_dn11_slot;
        let mut var_mexpr_i_dn13: f64 = *var_mexpr_i_dn13_slot;
        let mut var_mexpr_i_dn14: f64 = *var_mexpr_i_dn14_slot;
        let mut var_mexpr_i_dn2: f64 = *var_mexpr_i_dn2_slot;
        let mut var_mexpr_i_dn3: f64 = *var_mexpr_i_dn3_slot;
        let mut var_mexpr_i_dn4: f64 = *var_mexpr_i_dn4_slot;
        let mut var_mexpr_i_dn5: f64 = *var_mexpr_i_dn5_slot;
        let mut var_mexpr_i_dn6: f64 = *var_mexpr_i_dn6_slot;
        let mut var_mexpr_i_dn7: f64 = *var_mexpr_i_dn7_slot;
        let mut var_mexpr_i_dn8: f64 = *var_mexpr_i_dn8_slot;
        let mut var_mexpr_i_dn9: f64 = *var_mexpr_i_dn9_slot;
        let mut var_mpower_i: f64 = *var_mpower_i_slot;
        let mut var_noia2_i: f64 = *var_noia2_i_slot;
        let mut var_nq: f64 = *var_nq_slot;
        let mut var_nq_dn0: f64 = *var_nq_dn0_slot;
        let mut var_nq_dn10: f64 = *var_nq_dn10_slot;
        let mut var_nq_dn11: f64 = *var_nq_dn11_slot;
        let mut var_nq_dn13: f64 = *var_nq_dn13_slot;
        let mut var_nq_dn14: f64 = *var_nq_dn14_slot;
        let mut var_nq_dn2: f64 = *var_nq_dn2_slot;
        let mut var_nq_dn3: f64 = *var_nq_dn3_slot;
        let mut var_nq_dn4: f64 = *var_nq_dn4_slot;
        let mut var_nq_dn5: f64 = *var_nq_dn5_slot;
        let mut var_nq_dn6: f64 = *var_nq_dn6_slot;
        let mut var_nq_dn7: f64 = *var_nq_dn7_slot;
        let mut var_nq_dn8: f64 = *var_nq_dn8_slot;
        let mut var_nq_dn9: f64 = *var_nq_dn9_slot;
        let mut var_pclmr_i: f64 = *var_pclmr_i_slot;
        let mut var_pclmr_i_dn0: f64 = *var_pclmr_i_dn0_slot;
        let mut var_pclmr_i_dn10: f64 = *var_pclmr_i_dn10_slot;
        let mut var_pclmr_i_dn11: f64 = *var_pclmr_i_dn11_slot;
        let mut var_pclmr_i_dn13: f64 = *var_pclmr_i_dn13_slot;
        let mut var_pclmr_i_dn14: f64 = *var_pclmr_i_dn14_slot;
        let mut var_pclmr_i_dn2: f64 = *var_pclmr_i_dn2_slot;
        let mut var_pclmr_i_dn3: f64 = *var_pclmr_i_dn3_slot;
        let mut var_pclmr_i_dn4: f64 = *var_pclmr_i_dn4_slot;
        let mut var_pclmr_i_dn5: f64 = *var_pclmr_i_dn5_slot;
        let mut var_pclmr_i_dn6: f64 = *var_pclmr_i_dn6_slot;
        let mut var_pclmr_i_dn7: f64 = *var_pclmr_i_dn7_slot;
        let mut var_pclmr_i_dn8: f64 = *var_pclmr_i_dn8_slot;
        let mut var_pclmr_i_dn9: f64 = *var_pclmr_i_dn9_slot;
        let mut var_pdibl1r_i: f64 = *var_pdibl1r_i_slot;
        let mut var_pdibl2r_i: f64 = *var_pdibl2r_i_slot;
        let mut var_phibe_i: f64 = *var_phibe_i_slot;
        let mut var_ptwgr_i: f64 = *var_ptwgr_i_slot;
        let mut var_ptwgr_i_dn0: f64 = *var_ptwgr_i_dn0_slot;
        let mut var_ptwgr_i_dn10: f64 = *var_ptwgr_i_dn10_slot;
        let mut var_ptwgr_i_dn11: f64 = *var_ptwgr_i_dn11_slot;
        let mut var_ptwgr_i_dn13: f64 = *var_ptwgr_i_dn13_slot;
        let mut var_ptwgr_i_dn14: f64 = *var_ptwgr_i_dn14_slot;
        let mut var_ptwgr_i_dn2: f64 = *var_ptwgr_i_dn2_slot;
        let mut var_ptwgr_i_dn3: f64 = *var_ptwgr_i_dn3_slot;
        let mut var_ptwgr_i_dn4: f64 = *var_ptwgr_i_dn4_slot;
        let mut var_ptwgr_i_dn5: f64 = *var_ptwgr_i_dn5_slot;
        let mut var_ptwgr_i_dn6: f64 = *var_ptwgr_i_dn6_slot;
        let mut var_ptwgr_i_dn7: f64 = *var_ptwgr_i_dn7_slot;
        let mut var_ptwgr_i_dn8: f64 = *var_ptwgr_i_dn8_slot;
        let mut var_ptwgr_i_dn9: f64 = *var_ptwgr_i_dn9_slot;
        let mut var_qbov: f64 = *var_qbov_slot;
        let mut var_qbov_dn0: f64 = *var_qbov_dn0_slot;
        let mut var_qbov_dn10: f64 = *var_qbov_dn10_slot;
        let mut var_qbov_dn11: f64 = *var_qbov_dn11_slot;
        let mut var_qbov_dn13: f64 = *var_qbov_dn13_slot;
        let mut var_qbov_dn14: f64 = *var_qbov_dn14_slot;
        let mut var_qbov_dn2: f64 = *var_qbov_dn2_slot;
        let mut var_qbov_dn3: f64 = *var_qbov_dn3_slot;
        let mut var_qbov_dn4: f64 = *var_qbov_dn4_slot;
        let mut var_qbov_dn5: f64 = *var_qbov_dn5_slot;
        let mut var_qbov_dn6: f64 = *var_qbov_dn6_slot;
        let mut var_qbov_dn7: f64 = *var_qbov_dn7_slot;
        let mut var_qbov_dn8: f64 = *var_qbov_dn8_slot;
        let mut var_qbov_dn9: f64 = *var_qbov_dn9_slot;
        let mut var_qbov_s: f64 = *var_qbov_s_slot;
        let mut var_qbov_s_dn0: f64 = *var_qbov_s_dn0_slot;
        let mut var_qbov_s_dn10: f64 = *var_qbov_s_dn10_slot;
        let mut var_qbov_s_dn11: f64 = *var_qbov_s_dn11_slot;
        let mut var_qbov_s_dn13: f64 = *var_qbov_s_dn13_slot;
        let mut var_qbov_s_dn14: f64 = *var_qbov_s_dn14_slot;
        let mut var_qbov_s_dn2: f64 = *var_qbov_s_dn2_slot;
        let mut var_qbov_s_dn3: f64 = *var_qbov_s_dn3_slot;
        let mut var_qbov_s_dn4: f64 = *var_qbov_s_dn4_slot;
        let mut var_qbov_s_dn5: f64 = *var_qbov_s_dn5_slot;
        let mut var_qbov_s_dn6: f64 = *var_qbov_s_dn6_slot;
        let mut var_qbov_s_dn7: f64 = *var_qbov_s_dn7_slot;
        let mut var_qbov_s_dn8: f64 = *var_qbov_s_dn8_slot;
        let mut var_qbov_s_dn9: f64 = *var_qbov_s_dn9_slot;
        let mut var_qdep_ov_cins: f64 = *var_qdep_ov_cins_slot;
        let mut var_qi_acc_for_qm: f64 = *var_qi_acc_for_qm_slot;
        let mut var_qi_acc_for_qm_dn0: f64 = *var_qi_acc_for_qm_dn0_slot;
        let mut var_qi_acc_for_qm_dn10: f64 = *var_qi_acc_for_qm_dn10_slot;
        let mut var_qi_acc_for_qm_dn11: f64 = *var_qi_acc_for_qm_dn11_slot;
        let mut var_qi_acc_for_qm_dn13: f64 = *var_qi_acc_for_qm_dn13_slot;
        let mut var_qi_acc_for_qm_dn14: f64 = *var_qi_acc_for_qm_dn14_slot;
        let mut var_qi_acc_for_qm_dn2: f64 = *var_qi_acc_for_qm_dn2_slot;
        let mut var_qi_acc_for_qm_dn3: f64 = *var_qi_acc_for_qm_dn3_slot;
        let mut var_qi_acc_for_qm_dn4: f64 = *var_qi_acc_for_qm_dn4_slot;
        let mut var_qi_acc_for_qm_dn5: f64 = *var_qi_acc_for_qm_dn5_slot;
        let mut var_qi_acc_for_qm_dn6: f64 = *var_qi_acc_for_qm_dn6_slot;
        let mut var_qi_acc_for_qm_dn7: f64 = *var_qi_acc_for_qm_dn7_slot;
        let mut var_qi_acc_for_qm_dn8: f64 = *var_qi_acc_for_qm_dn8_slot;
        let mut var_qi_acc_for_qm_dn9: f64 = *var_qi_acc_for_qm_dn9_slot;
        let mut var_qid: f64 = *var_qid_slot;
        let mut var_qid_dn0: f64 = *var_qid_dn0_slot;
        let mut var_qid_dn10: f64 = *var_qid_dn10_slot;
        let mut var_qid_dn11: f64 = *var_qid_dn11_slot;
        let mut var_qid_dn13: f64 = *var_qid_dn13_slot;
        let mut var_qid_dn14: f64 = *var_qid_dn14_slot;
        let mut var_qid_dn2: f64 = *var_qid_dn2_slot;
        let mut var_qid_dn3: f64 = *var_qid_dn3_slot;
        let mut var_qid_dn4: f64 = *var_qid_dn4_slot;
        let mut var_qid_dn5: f64 = *var_qid_dn5_slot;
        let mut var_qid_dn6: f64 = *var_qid_dn6_slot;
        let mut var_qid_dn7: f64 = *var_qid_dn7_slot;
        let mut var_qid_dn8: f64 = *var_qid_dn8_slot;
        let mut var_qid_dn9: f64 = *var_qid_dn9_slot;
        let mut var_qis: f64 = *var_qis_slot;
        let mut var_qis_dn0: f64 = *var_qis_dn0_slot;
        let mut var_qis_dn10: f64 = *var_qis_dn10_slot;
        let mut var_qis_dn11: f64 = *var_qis_dn11_slot;
        let mut var_qis_dn13: f64 = *var_qis_dn13_slot;
        let mut var_qis_dn14: f64 = *var_qis_dn14_slot;
        let mut var_qis_dn2: f64 = *var_qis_dn2_slot;
        let mut var_qis_dn3: f64 = *var_qis_dn3_slot;
        let mut var_qis_dn4: f64 = *var_qis_dn4_slot;
        let mut var_qis_dn5: f64 = *var_qis_dn5_slot;
        let mut var_qis_dn6: f64 = *var_qis_dn6_slot;
        let mut var_qis_dn7: f64 = *var_qis_dn7_slot;
        let mut var_qis_dn8: f64 = *var_qis_dn8_slot;
        let mut var_qis_dn9: f64 = *var_qis_dn9_slot;
        let mut var_qndd1: f64 = *var_qndd1_slot;
        let mut var_qndd1_dn0: f64 = *var_qndd1_dn0_slot;
        let mut var_qndd1_dn10: f64 = *var_qndd1_dn10_slot;
        let mut var_qndd1_dn11: f64 = *var_qndd1_dn11_slot;
        let mut var_qndd1_dn13: f64 = *var_qndd1_dn13_slot;
        let mut var_qndd1_dn14: f64 = *var_qndd1_dn14_slot;
        let mut var_qndd1_dn2: f64 = *var_qndd1_dn2_slot;
        let mut var_qndd1_dn3: f64 = *var_qndd1_dn3_slot;
        let mut var_qndd1_dn4: f64 = *var_qndd1_dn4_slot;
        let mut var_qndd1_dn5: f64 = *var_qndd1_dn5_slot;
        let mut var_qndd1_dn6: f64 = *var_qndd1_dn6_slot;
        let mut var_qndd1_dn7: f64 = *var_qndd1_dn7_slot;
        let mut var_qndd1_dn8: f64 = *var_qndd1_dn8_slot;
        let mut var_qndd1_dn9: f64 = *var_qndd1_dn9_slot;
        let mut var_qnds1: f64 = *var_qnds1_slot;
        let mut var_qnds1_dn0: f64 = *var_qnds1_dn0_slot;
        let mut var_qnds1_dn10: f64 = *var_qnds1_dn10_slot;
        let mut var_qnds1_dn11: f64 = *var_qnds1_dn11_slot;
        let mut var_qnds1_dn13: f64 = *var_qnds1_dn13_slot;
        let mut var_qnds1_dn14: f64 = *var_qnds1_dn14_slot;
        let mut var_qnds1_dn2: f64 = *var_qnds1_dn2_slot;
        let mut var_qnds1_dn3: f64 = *var_qnds1_dn3_slot;
        let mut var_qnds1_dn4: f64 = *var_qnds1_dn4_slot;
        let mut var_qnds1_dn5: f64 = *var_qnds1_dn5_slot;
        let mut var_qnds1_dn6: f64 = *var_qnds1_dn6_slot;
        let mut var_qnds1_dn7: f64 = *var_qnds1_dn7_slot;
        let mut var_qnds1_dn8: f64 = *var_qnds1_dn8_slot;
        let mut var_qnds1_dn9: f64 = *var_qnds1_dn9_slot;
        let mut var_qnds2: f64 = *var_qnds2_slot;
        let mut var_qnds2_dn0: f64 = *var_qnds2_dn0_slot;
        let mut var_qnds2_dn10: f64 = *var_qnds2_dn10_slot;
        let mut var_qnds2_dn11: f64 = *var_qnds2_dn11_slot;
        let mut var_qnds2_dn13: f64 = *var_qnds2_dn13_slot;
        let mut var_qnds2_dn14: f64 = *var_qnds2_dn14_slot;
        let mut var_qnds2_dn2: f64 = *var_qnds2_dn2_slot;
        let mut var_qnds2_dn3: f64 = *var_qnds2_dn3_slot;
        let mut var_qnds2_dn4: f64 = *var_qnds2_dn4_slot;
        let mut var_qnds2_dn5: f64 = *var_qnds2_dn5_slot;
        let mut var_qnds2_dn6: f64 = *var_qnds2_dn6_slot;
        let mut var_qnds2_dn7: f64 = *var_qnds2_dn7_slot;
        let mut var_qnds2_dn8: f64 = *var_qnds2_dn8_slot;
        let mut var_qnds2_dn9: f64 = *var_qnds2_dn9_slot;
        let mut var_qnds3: f64 = *var_qnds3_slot;
        let mut var_qnds3_dn0: f64 = *var_qnds3_dn0_slot;
        let mut var_qnds3_dn10: f64 = *var_qnds3_dn10_slot;
        let mut var_qnds3_dn11: f64 = *var_qnds3_dn11_slot;
        let mut var_qnds3_dn13: f64 = *var_qnds3_dn13_slot;
        let mut var_qnds3_dn14: f64 = *var_qnds3_dn14_slot;
        let mut var_qnds3_dn2: f64 = *var_qnds3_dn2_slot;
        let mut var_qnds3_dn3: f64 = *var_qnds3_dn3_slot;
        let mut var_qnds3_dn4: f64 = *var_qnds3_dn4_slot;
        let mut var_qnds3_dn5: f64 = *var_qnds3_dn5_slot;
        let mut var_qnds3_dn6: f64 = *var_qnds3_dn6_slot;
        let mut var_qnds3_dn7: f64 = *var_qnds3_dn7_slot;
        let mut var_qnds3_dn8: f64 = *var_qnds3_dn8_slot;
        let mut var_qnds3_dn9: f64 = *var_qnds3_dn9_slot;
        let mut var_rc: f64 = *var_rc_slot;
        let mut var_sigrat: f64 = *var_sigrat_slot;
        let mut var_sigrat_dn0: f64 = *var_sigrat_dn0_slot;
        let mut var_sigrat_dn10: f64 = *var_sigrat_dn10_slot;
        let mut var_sigrat_dn11: f64 = *var_sigrat_dn11_slot;
        let mut var_sigrat_dn13: f64 = *var_sigrat_dn13_slot;
        let mut var_sigrat_dn14: f64 = *var_sigrat_dn14_slot;
        let mut var_sigrat_dn2: f64 = *var_sigrat_dn2_slot;
        let mut var_sigrat_dn3: f64 = *var_sigrat_dn3_slot;
        let mut var_sigrat_dn4: f64 = *var_sigrat_dn4_slot;
        let mut var_sigrat_dn5: f64 = *var_sigrat_dn5_slot;
        let mut var_sigrat_dn6: f64 = *var_sigrat_dn6_slot;
        let mut var_sigrat_dn7: f64 = *var_sigrat_dn7_slot;
        let mut var_sigrat_dn8: f64 = *var_sigrat_dn8_slot;
        let mut var_sigrat_dn9: f64 = *var_sigrat_dn9_slot;
        let mut var_u0r_i: f64 = *var_u0r_i_slot;
        let mut var_u0r_i_dn0: f64 = *var_u0r_i_dn0_slot;
        let mut var_u0r_i_dn10: f64 = *var_u0r_i_dn10_slot;
        let mut var_u0r_i_dn11: f64 = *var_u0r_i_dn11_slot;
        let mut var_u0r_i_dn13: f64 = *var_u0r_i_dn13_slot;
        let mut var_u0r_i_dn14: f64 = *var_u0r_i_dn14_slot;
        let mut var_u0r_i_dn2: f64 = *var_u0r_i_dn2_slot;
        let mut var_u0r_i_dn3: f64 = *var_u0r_i_dn3_slot;
        let mut var_u0r_i_dn4: f64 = *var_u0r_i_dn4_slot;
        let mut var_u0r_i_dn5: f64 = *var_u0r_i_dn5_slot;
        let mut var_u0r_i_dn6: f64 = *var_u0r_i_dn6_slot;
        let mut var_u0r_i_dn7: f64 = *var_u0r_i_dn7_slot;
        let mut var_u0r_i_dn8: f64 = *var_u0r_i_dn8_slot;
        let mut var_u0r_i_dn9: f64 = *var_u0r_i_dn9_slot;
        let mut var_ua1r_i: f64 = *var_ua1r_i_slot;
        let mut var_uar_i: f64 = *var_uar_i_slot;
        let mut var_uar_i_dn0: f64 = *var_uar_i_dn0_slot;
        let mut var_uar_i_dn10: f64 = *var_uar_i_dn10_slot;
        let mut var_uar_i_dn11: f64 = *var_uar_i_dn11_slot;
        let mut var_uar_i_dn13: f64 = *var_uar_i_dn13_slot;
        let mut var_uar_i_dn14: f64 = *var_uar_i_dn14_slot;
        let mut var_uar_i_dn2: f64 = *var_uar_i_dn2_slot;
        let mut var_uar_i_dn3: f64 = *var_uar_i_dn3_slot;
        let mut var_uar_i_dn4: f64 = *var_uar_i_dn4_slot;
        let mut var_uar_i_dn5: f64 = *var_uar_i_dn5_slot;
        let mut var_uar_i_dn6: f64 = *var_uar_i_dn6_slot;
        let mut var_uar_i_dn7: f64 = *var_uar_i_dn7_slot;
        let mut var_uar_i_dn8: f64 = *var_uar_i_dn8_slot;
        let mut var_uar_i_dn9: f64 = *var_uar_i_dn9_slot;
        let mut var_uc1_i: f64 = *var_uc1_i_slot;
        let mut var_uc1r_i: f64 = *var_uc1r_i_slot;
        let mut var_uc_i: f64 = *var_uc_i_slot;
        let mut var_ucr_i: f64 = *var_ucr_i_slot;
        let mut var_ud1r_i: f64 = *var_ud1r_i_slot;
        let mut var_udr_i: f64 = *var_udr_i_slot;
        let mut var_udr_i_dn0: f64 = *var_udr_i_dn0_slot;
        let mut var_udr_i_dn10: f64 = *var_udr_i_dn10_slot;
        let mut var_udr_i_dn11: f64 = *var_udr_i_dn11_slot;
        let mut var_udr_i_dn13: f64 = *var_udr_i_dn13_slot;
        let mut var_udr_i_dn14: f64 = *var_udr_i_dn14_slot;
        let mut var_udr_i_dn2: f64 = *var_udr_i_dn2_slot;
        let mut var_udr_i_dn3: f64 = *var_udr_i_dn3_slot;
        let mut var_udr_i_dn4: f64 = *var_udr_i_dn4_slot;
        let mut var_udr_i_dn5: f64 = *var_udr_i_dn5_slot;
        let mut var_udr_i_dn6: f64 = *var_udr_i_dn6_slot;
        let mut var_udr_i_dn7: f64 = *var_udr_i_dn7_slot;
        let mut var_udr_i_dn8: f64 = *var_udr_i_dn8_slot;
        let mut var_udr_i_dn9: f64 = *var_udr_i_dn9_slot;
        let mut var_upr_i: f64 = *var_upr_i_slot;
        let mut var_uter_i: f64 = *var_uter_i_slot;
        let mut var_utlr_i: f64 = *var_utlr_i_slot;
        let mut var_veseff: f64 = *var_veseff_slot;
        let mut var_veseff_dn0: f64 = *var_veseff_dn0_slot;
        let mut var_veseff_dn10: f64 = *var_veseff_dn10_slot;
        let mut var_veseff_dn11: f64 = *var_veseff_dn11_slot;
        let mut var_veseff_dn13: f64 = *var_veseff_dn13_slot;
        let mut var_veseff_dn14: f64 = *var_veseff_dn14_slot;
        let mut var_veseff_dn2: f64 = *var_veseff_dn2_slot;
        let mut var_veseff_dn3: f64 = *var_veseff_dn3_slot;
        let mut var_veseff_dn4: f64 = *var_veseff_dn4_slot;
        let mut var_veseff_dn5: f64 = *var_veseff_dn5_slot;
        let mut var_veseff_dn6: f64 = *var_veseff_dn6_slot;
        let mut var_veseff_dn7: f64 = *var_veseff_dn7_slot;
        let mut var_veseff_dn8: f64 = *var_veseff_dn8_slot;
        let mut var_veseff_dn9: f64 = *var_veseff_dn9_slot;
        let mut var_vsat1r_i: f64 = *var_vsat1r_i_slot;
        let mut var_vsat1r_i_dn0: f64 = *var_vsat1r_i_dn0_slot;
        let mut var_vsat1r_i_dn10: f64 = *var_vsat1r_i_dn10_slot;
        let mut var_vsat1r_i_dn11: f64 = *var_vsat1r_i_dn11_slot;
        let mut var_vsat1r_i_dn13: f64 = *var_vsat1r_i_dn13_slot;
        let mut var_vsat1r_i_dn14: f64 = *var_vsat1r_i_dn14_slot;
        let mut var_vsat1r_i_dn2: f64 = *var_vsat1r_i_dn2_slot;
        let mut var_vsat1r_i_dn3: f64 = *var_vsat1r_i_dn3_slot;
        let mut var_vsat1r_i_dn4: f64 = *var_vsat1r_i_dn4_slot;
        let mut var_vsat1r_i_dn5: f64 = *var_vsat1r_i_dn5_slot;
        let mut var_vsat1r_i_dn6: f64 = *var_vsat1r_i_dn6_slot;
        let mut var_vsat1r_i_dn7: f64 = *var_vsat1r_i_dn7_slot;
        let mut var_vsat1r_i_dn8: f64 = *var_vsat1r_i_dn8_slot;
        let mut var_vsat1r_i_dn9: f64 = *var_vsat1r_i_dn9_slot;
        let mut var_vsatr_i: f64 = *var_vsatr_i_slot;
        let mut var_weff2: f64 = *var_weff2_slot;
        let mut var_weff3: f64 = *var_weff3_slot;
        let mut var_weff4: f64 = *var_weff4_slot;
        let mut var_weff5: f64 = *var_weff5_slot;
        let mut var_weff6: f64 = *var_weff6_slot;
        let mut var_weff_ufcm: f64 = *var_weff_ufcm_slot;
        let mut var_weffb: f64 = *var_weffb_slot;
        let mut var_xrcrg1_i: f64 = *var_xrcrg1_i_slot;
        let mut var_xrcrg2_i: f64 = *var_xrcrg2_i_slot;

        var_lambdac_fn2 = 0.0;
        var_lambdac_fn2_dn0 = 0.0;
        var_lambdac_fn2_dn2 = 0.0;
        var_lambdac_fn2_dn3 = 0.0;
        var_lambdac_fn2_dn4 = 0.0;
        var_lambdac_fn2_dn5 = 0.0;
        var_lambdac_fn2_dn6 = 0.0;
        var_lambdac_fn2_dn7 = 0.0;
        var_lambdac_fn2_dn8 = 0.0;
        var_lambdac_fn2_dn9 = 0.0;
        var_lambdac_fn2_dn10 = 0.0;
        var_lambdac_fn2_dn11 = 0.0;
        var_lambdac_fn2_dn13 = 0.0;
        var_lambdac_fn2_dn14 = 0.0;

        var_noia2_i = 0.0;

        var_mpower_i = 0.0;

        var_dr0 = 0.0;
        var_dr0_dn0 = 0.0;
        var_dr0_dn2 = 0.0;
        var_dr0_dn3 = 0.0;
        var_dr0_dn4 = 0.0;
        var_dr0_dn5 = 0.0;
        var_dr0_dn6 = 0.0;
        var_dr0_dn7 = 0.0;
        var_dr0_dn8 = 0.0;
        var_dr0_dn9 = 0.0;
        var_dr0_dn10 = 0.0;
        var_dr0_dn11 = 0.0;
        var_dr0_dn13 = 0.0;
        var_dr0_dn14 = 0.0;

        var_ctnoi = 0.0;
        var_ctnoi_dn0 = 0.0;
        var_ctnoi_dn2 = 0.0;
        var_ctnoi_dn3 = 0.0;
        var_ctnoi_dn4 = 0.0;
        var_ctnoi_dn5 = 0.0;
        var_ctnoi_dn6 = 0.0;
        var_ctnoi_dn7 = 0.0;
        var_ctnoi_dn8 = 0.0;
        var_ctnoi_dn9 = 0.0;
        var_ctnoi_dn10 = 0.0;
        var_ctnoi_dn11 = 0.0;
        var_ctnoi_dn13 = 0.0;
        var_ctnoi_dn14 = 0.0;

        var_sigrat = 0.0;
        var_sigrat_dn0 = 0.0;
        var_sigrat_dn2 = 0.0;
        var_sigrat_dn3 = 0.0;
        var_sigrat_dn4 = 0.0;
        var_sigrat_dn5 = 0.0;
        var_sigrat_dn6 = 0.0;
        var_sigrat_dn7 = 0.0;
        var_sigrat_dn8 = 0.0;
        var_sigrat_dn9 = 0.0;
        var_sigrat_dn10 = 0.0;
        var_sigrat_dn11 = 0.0;
        var_sigrat_dn13 = 0.0;
        var_sigrat_dn14 = 0.0;

        var_gth = 0.0;
        var_gth_dn0 = 0.0;
        var_gth_dn2 = 0.0;
        var_gth_dn3 = 0.0;
        var_gth_dn4 = 0.0;
        var_gth_dn5 = 0.0;
        var_gth_dn6 = 0.0;
        var_gth_dn7 = 0.0;
        var_gth_dn8 = 0.0;
        var_gth_dn9 = 0.0;
        var_gth_dn10 = 0.0;
        var_gth_dn11 = 0.0;
        var_gth_dn13 = 0.0;
        var_gth_dn14 = 0.0;

        var_cth = 0.0;
        var_cth_dn0 = 0.0;
        var_cth_dn2 = 0.0;
        var_cth_dn3 = 0.0;
        var_cth_dn4 = 0.0;
        var_cth_dn5 = 0.0;
        var_cth_dn6 = 0.0;
        var_cth_dn7 = 0.0;
        var_cth_dn8 = 0.0;
        var_cth_dn9 = 0.0;
        var_cth_dn10 = 0.0;
        var_cth_dn11 = 0.0;
        var_cth_dn13 = 0.0;
        var_cth_dn14 = 0.0;

        var_citr_i = 0.0;

        var_cdscdr_i = 0.0;

        var_eta0r_i = 0.0;

        var_dvtshiftr_i = 0.0;

        var_veseff = 0.0;
        var_veseff_dn0 = 0.0;
        var_veseff_dn2 = 0.0;
        var_veseff_dn3 = 0.0;
        var_veseff_dn4 = 0.0;
        var_veseff_dn5 = 0.0;
        var_veseff_dn6 = 0.0;
        var_veseff_dn7 = 0.0;
        var_veseff_dn8 = 0.0;
        var_veseff_dn9 = 0.0;
        var_veseff_dn10 = 0.0;
        var_veseff_dn11 = 0.0;
        var_veseff_dn13 = 0.0;
        var_veseff_dn14 = 0.0;

        var_phibe_i = 0.0;

        var_k1_i = 0.0;

        var_k11_i = 0.0;

        var_k2sat_i = 0.0;

        var_k2sat1_i = 0.0;

        var_k2_i = 0.0;

        var_k21_i = 0.0;

        var_vsatr_i = 0.0;

        var_vsat1r_i = 0.0;
        var_vsat1r_i_dn0 = 0.0;
        var_vsat1r_i_dn2 = 0.0;
        var_vsat1r_i_dn3 = 0.0;
        var_vsat1r_i_dn4 = 0.0;
        var_vsat1r_i_dn5 = 0.0;
        var_vsat1r_i_dn6 = 0.0;
        var_vsat1r_i_dn7 = 0.0;
        var_vsat1r_i_dn8 = 0.0;
        var_vsat1r_i_dn9 = 0.0;
        var_vsat1r_i_dn10 = 0.0;
        var_vsat1r_i_dn11 = 0.0;
        var_vsat1r_i_dn13 = 0.0;
        var_vsat1r_i_dn14 = 0.0;

        var_ksativr_i = 0.0;

        var_mexpr_i = 0.0;
        var_mexpr_i_dn0 = 0.0;
        var_mexpr_i_dn2 = 0.0;
        var_mexpr_i_dn3 = 0.0;
        var_mexpr_i_dn4 = 0.0;
        var_mexpr_i_dn5 = 0.0;
        var_mexpr_i_dn6 = 0.0;
        var_mexpr_i_dn7 = 0.0;
        var_mexpr_i_dn8 = 0.0;
        var_mexpr_i_dn9 = 0.0;
        var_mexpr_i_dn10 = 0.0;
        var_mexpr_i_dn11 = 0.0;
        var_mexpr_i_dn13 = 0.0;
        var_mexpr_i_dn14 = 0.0;

        var_ptwgr_i = 0.0;
        var_ptwgr_i_dn0 = 0.0;
        var_ptwgr_i_dn2 = 0.0;
        var_ptwgr_i_dn3 = 0.0;
        var_ptwgr_i_dn4 = 0.0;
        var_ptwgr_i_dn5 = 0.0;
        var_ptwgr_i_dn6 = 0.0;
        var_ptwgr_i_dn7 = 0.0;
        var_ptwgr_i_dn8 = 0.0;
        var_ptwgr_i_dn9 = 0.0;
        var_ptwgr_i_dn10 = 0.0;
        var_ptwgr_i_dn11 = 0.0;
        var_ptwgr_i_dn13 = 0.0;
        var_ptwgr_i_dn14 = 0.0;

        var_atr_i = 0.0;

        var_u0r_i = 0.0;
        var_u0r_i_dn0 = 0.0;
        var_u0r_i_dn2 = 0.0;
        var_u0r_i_dn3 = 0.0;
        var_u0r_i_dn4 = 0.0;
        var_u0r_i_dn5 = 0.0;
        var_u0r_i_dn6 = 0.0;
        var_u0r_i_dn7 = 0.0;
        var_u0r_i_dn8 = 0.0;
        var_u0r_i_dn9 = 0.0;
        var_u0r_i_dn10 = 0.0;
        var_u0r_i_dn11 = 0.0;
        var_u0r_i_dn13 = 0.0;
        var_u0r_i_dn14 = 0.0;

        var_upr_i = 0.0;

        var_uar_i = 0.0;
        var_uar_i_dn0 = 0.0;
        var_uar_i_dn2 = 0.0;
        var_uar_i_dn3 = 0.0;
        var_uar_i_dn4 = 0.0;
        var_uar_i_dn5 = 0.0;
        var_uar_i_dn6 = 0.0;
        var_uar_i_dn7 = 0.0;
        var_uar_i_dn8 = 0.0;
        var_uar_i_dn9 = 0.0;
        var_uar_i_dn10 = 0.0;
        var_uar_i_dn11 = 0.0;
        var_uar_i_dn13 = 0.0;
        var_uar_i_dn14 = 0.0;

        var_uc_i = 0.0;

        var_ucr_i = 0.0;

        var_eur_i = 0.0;
        var_eur_i_dn0 = 0.0;
        var_eur_i_dn2 = 0.0;
        var_eur_i_dn3 = 0.0;
        var_eur_i_dn4 = 0.0;
        var_eur_i_dn5 = 0.0;
        var_eur_i_dn6 = 0.0;
        var_eur_i_dn7 = 0.0;
        var_eur_i_dn8 = 0.0;
        var_eur_i_dn9 = 0.0;
        var_eur_i_dn10 = 0.0;
        var_eur_i_dn11 = 0.0;
        var_eur_i_dn13 = 0.0;
        var_eur_i_dn14 = 0.0;

        var_udr_i = 0.0;
        var_udr_i_dn0 = 0.0;
        var_udr_i_dn2 = 0.0;
        var_udr_i_dn3 = 0.0;
        var_udr_i_dn4 = 0.0;
        var_udr_i_dn5 = 0.0;
        var_udr_i_dn6 = 0.0;
        var_udr_i_dn7 = 0.0;
        var_udr_i_dn8 = 0.0;
        var_udr_i_dn9 = 0.0;
        var_udr_i_dn10 = 0.0;
        var_udr_i_dn11 = 0.0;
        var_udr_i_dn13 = 0.0;
        var_udr_i_dn14 = 0.0;

        var_uter_i = 0.0;

        var_utlr_i = 0.0;

        var_ua1r_i = 0.0;

        var_uc1_i = 0.0;

        var_uc1r_i = 0.0;

        var_ud1r_i = 0.0;

        var_pdibl1r_i = 0.0;

        var_pdibl2r_i = 0.0;

        var_pclmr_i = 0.0;
        var_pclmr_i_dn0 = 0.0;
        var_pclmr_i_dn2 = 0.0;
        var_pclmr_i_dn3 = 0.0;
        var_pclmr_i_dn4 = 0.0;
        var_pclmr_i_dn5 = 0.0;
        var_pclmr_i_dn6 = 0.0;
        var_pclmr_i_dn7 = 0.0;
        var_pclmr_i_dn8 = 0.0;
        var_pclmr_i_dn9 = 0.0;
        var_pclmr_i_dn10 = 0.0;
        var_pclmr_i_dn11 = 0.0;
        var_pclmr_i_dn13 = 0.0;
        var_pclmr_i_dn14 = 0.0;

        var_cgso_i = 0.0;

        var_cgdo_i = 0.0;

        var_covd_i = 0.0;
        var_covd_i_dn0 = 0.0;
        var_covd_i_dn2 = 0.0;
        var_covd_i_dn3 = 0.0;
        var_covd_i_dn4 = 0.0;
        var_covd_i_dn5 = 0.0;
        var_covd_i_dn6 = 0.0;
        var_covd_i_dn7 = 0.0;
        var_covd_i_dn8 = 0.0;
        var_covd_i_dn9 = 0.0;
        var_covd_i_dn10 = 0.0;
        var_covd_i_dn11 = 0.0;
        var_covd_i_dn13 = 0.0;
        var_covd_i_dn14 = 0.0;

        var_covs_i = 0.0;
        var_covs_i_dn0 = 0.0;
        var_covs_i_dn2 = 0.0;
        var_covs_i_dn3 = 0.0;
        var_covs_i_dn4 = 0.0;
        var_covs_i_dn5 = 0.0;
        var_covs_i_dn6 = 0.0;
        var_covs_i_dn7 = 0.0;
        var_covs_i_dn8 = 0.0;
        var_covs_i_dn9 = 0.0;
        var_covs_i_dn10 = 0.0;
        var_covs_i_dn11 = 0.0;
        var_covs_i_dn13 = 0.0;
        var_covs_i_dn14 = 0.0;

        var_xrcrg1_i = 0.0;

        var_xrcrg2_i = 0.0;

        var_cins = 0.0;

        var_ach = 0.0;

        var_weff_ufcm = 0.0;

        var_weffb = 0.0;

        var_rc = 0.0;

        var_qdep_ov_cins = 0.0;

        var_qi_acc_for_qm = 0.0;
        var_qi_acc_for_qm_dn0 = 0.0;
        var_qi_acc_for_qm_dn2 = 0.0;
        var_qi_acc_for_qm_dn3 = 0.0;
        var_qi_acc_for_qm_dn4 = 0.0;
        var_qi_acc_for_qm_dn5 = 0.0;
        var_qi_acc_for_qm_dn6 = 0.0;
        var_qi_acc_for_qm_dn7 = 0.0;
        var_qi_acc_for_qm_dn8 = 0.0;
        var_qi_acc_for_qm_dn9 = 0.0;
        var_qi_acc_for_qm_dn10 = 0.0;
        var_qi_acc_for_qm_dn11 = 0.0;
        var_qi_acc_for_qm_dn13 = 0.0;
        var_qi_acc_for_qm_dn14 = 0.0;

        var_nq = 0.0;
        var_nq_dn0 = 0.0;
        var_nq_dn2 = 0.0;
        var_nq_dn3 = 0.0;
        var_nq_dn4 = 0.0;
        var_nq_dn5 = 0.0;
        var_nq_dn6 = 0.0;
        var_nq_dn7 = 0.0;
        var_nq_dn8 = 0.0;
        var_nq_dn9 = 0.0;
        var_nq_dn10 = 0.0;
        var_nq_dn11 = 0.0;
        var_nq_dn13 = 0.0;
        var_nq_dn14 = 0.0;

        var_qis = 0.0;
        var_qis_dn0 = 0.0;
        var_qis_dn2 = 0.0;
        var_qis_dn3 = 0.0;
        var_qis_dn4 = 0.0;
        var_qis_dn5 = 0.0;
        var_qis_dn6 = 0.0;
        var_qis_dn7 = 0.0;
        var_qis_dn8 = 0.0;
        var_qis_dn9 = 0.0;
        var_qis_dn10 = 0.0;
        var_qis_dn11 = 0.0;
        var_qis_dn13 = 0.0;
        var_qis_dn14 = 0.0;

        var_qid = 0.0;
        var_qid_dn0 = 0.0;
        var_qid_dn2 = 0.0;
        var_qid_dn3 = 0.0;
        var_qid_dn4 = 0.0;
        var_qid_dn5 = 0.0;
        var_qid_dn6 = 0.0;
        var_qid_dn7 = 0.0;
        var_qid_dn8 = 0.0;
        var_qid_dn9 = 0.0;
        var_qid_dn10 = 0.0;
        var_qid_dn11 = 0.0;
        var_qid_dn13 = 0.0;
        var_qid_dn14 = 0.0;

        var_qbov = 0.0;
        var_qbov_dn0 = 0.0;
        var_qbov_dn2 = 0.0;
        var_qbov_dn3 = 0.0;
        var_qbov_dn4 = 0.0;
        var_qbov_dn5 = 0.0;
        var_qbov_dn6 = 0.0;
        var_qbov_dn7 = 0.0;
        var_qbov_dn8 = 0.0;
        var_qbov_dn9 = 0.0;
        var_qbov_dn10 = 0.0;
        var_qbov_dn11 = 0.0;
        var_qbov_dn13 = 0.0;
        var_qbov_dn14 = 0.0;

        var_qbov_s = 0.0;
        var_qbov_s_dn0 = 0.0;
        var_qbov_s_dn2 = 0.0;
        var_qbov_s_dn3 = 0.0;
        var_qbov_s_dn4 = 0.0;
        var_qbov_s_dn5 = 0.0;
        var_qbov_s_dn6 = 0.0;
        var_qbov_s_dn7 = 0.0;
        var_qbov_s_dn8 = 0.0;
        var_qbov_s_dn9 = 0.0;
        var_qbov_s_dn10 = 0.0;
        var_qbov_s_dn11 = 0.0;
        var_qbov_s_dn13 = 0.0;
        var_qbov_s_dn14 = 0.0;

        var_ach2 = 0.0;

        var_ach3 = 0.0;

        var_ach4 = 0.0;

        var_ach5 = 0.0;

        var_ach6 = 0.0;

        var_weff2 = 0.0;

        var_weff3 = 0.0;

        var_weff4 = 0.0;

        var_weff5 = 0.0;

        var_weff6 = 0.0;

        var_qnds1 = 0.0;
        var_qnds1_dn0 = 0.0;
        var_qnds1_dn2 = 0.0;
        var_qnds1_dn3 = 0.0;
        var_qnds1_dn4 = 0.0;
        var_qnds1_dn5 = 0.0;
        var_qnds1_dn6 = 0.0;
        var_qnds1_dn7 = 0.0;
        var_qnds1_dn8 = 0.0;
        var_qnds1_dn9 = 0.0;
        var_qnds1_dn10 = 0.0;
        var_qnds1_dn11 = 0.0;
        var_qnds1_dn13 = 0.0;
        var_qnds1_dn14 = 0.0;

        var_qnds2 = 0.0;
        var_qnds2_dn0 = 0.0;
        var_qnds2_dn2 = 0.0;
        var_qnds2_dn3 = 0.0;
        var_qnds2_dn4 = 0.0;
        var_qnds2_dn5 = 0.0;
        var_qnds2_dn6 = 0.0;
        var_qnds2_dn7 = 0.0;
        var_qnds2_dn8 = 0.0;
        var_qnds2_dn9 = 0.0;
        var_qnds2_dn10 = 0.0;
        var_qnds2_dn11 = 0.0;
        var_qnds2_dn13 = 0.0;
        var_qnds2_dn14 = 0.0;

        var_qnds3 = 0.0;
        var_qnds3_dn0 = 0.0;
        var_qnds3_dn2 = 0.0;
        var_qnds3_dn3 = 0.0;
        var_qnds3_dn4 = 0.0;
        var_qnds3_dn5 = 0.0;
        var_qnds3_dn6 = 0.0;
        var_qnds3_dn7 = 0.0;
        var_qnds3_dn8 = 0.0;
        var_qnds3_dn9 = 0.0;
        var_qnds3_dn10 = 0.0;
        var_qnds3_dn11 = 0.0;
        var_qnds3_dn13 = 0.0;
        var_qnds3_dn14 = 0.0;

        var_qndd1 = 0.0;
        var_qndd1_dn0 = 0.0;
        var_qndd1_dn2 = 0.0;
        var_qndd1_dn3 = 0.0;
        var_qndd1_dn4 = 0.0;
        var_qndd1_dn5 = 0.0;
        var_qndd1_dn6 = 0.0;
        var_qndd1_dn7 = 0.0;
        var_qndd1_dn8 = 0.0;
        var_qndd1_dn9 = 0.0;
        var_qndd1_dn10 = 0.0;
        var_qndd1_dn11 = 0.0;
        var_qndd1_dn13 = 0.0;
        var_qndd1_dn14 = 0.0;

        *var_ach_slot = var_ach;
        *var_ach2_slot = var_ach2;
        *var_ach3_slot = var_ach3;
        *var_ach4_slot = var_ach4;
        *var_ach5_slot = var_ach5;
        *var_ach6_slot = var_ach6;
        *var_atr_i_slot = var_atr_i;
        *var_cdscdr_i_slot = var_cdscdr_i;
        *var_cgdo_i_slot = var_cgdo_i;
        *var_cgso_i_slot = var_cgso_i;
        *var_cins_slot = var_cins;
        *var_citr_i_slot = var_citr_i;
        *var_covd_i_slot = var_covd_i;
        *var_covd_i_dn0_slot = var_covd_i_dn0;
        *var_covd_i_dn10_slot = var_covd_i_dn10;
        *var_covd_i_dn11_slot = var_covd_i_dn11;
        *var_covd_i_dn13_slot = var_covd_i_dn13;
        *var_covd_i_dn14_slot = var_covd_i_dn14;
        *var_covd_i_dn2_slot = var_covd_i_dn2;
        *var_covd_i_dn3_slot = var_covd_i_dn3;
        *var_covd_i_dn4_slot = var_covd_i_dn4;
        *var_covd_i_dn5_slot = var_covd_i_dn5;
        *var_covd_i_dn6_slot = var_covd_i_dn6;
        *var_covd_i_dn7_slot = var_covd_i_dn7;
        *var_covd_i_dn8_slot = var_covd_i_dn8;
        *var_covd_i_dn9_slot = var_covd_i_dn9;
        *var_covs_i_slot = var_covs_i;
        *var_covs_i_dn0_slot = var_covs_i_dn0;
        *var_covs_i_dn10_slot = var_covs_i_dn10;
        *var_covs_i_dn11_slot = var_covs_i_dn11;
        *var_covs_i_dn13_slot = var_covs_i_dn13;
        *var_covs_i_dn14_slot = var_covs_i_dn14;
        *var_covs_i_dn2_slot = var_covs_i_dn2;
        *var_covs_i_dn3_slot = var_covs_i_dn3;
        *var_covs_i_dn4_slot = var_covs_i_dn4;
        *var_covs_i_dn5_slot = var_covs_i_dn5;
        *var_covs_i_dn6_slot = var_covs_i_dn6;
        *var_covs_i_dn7_slot = var_covs_i_dn7;
        *var_covs_i_dn8_slot = var_covs_i_dn8;
        *var_covs_i_dn9_slot = var_covs_i_dn9;
        *var_cth_slot = var_cth;
        *var_cth_dn0_slot = var_cth_dn0;
        *var_cth_dn10_slot = var_cth_dn10;
        *var_cth_dn11_slot = var_cth_dn11;
        *var_cth_dn13_slot = var_cth_dn13;
        *var_cth_dn14_slot = var_cth_dn14;
        *var_cth_dn2_slot = var_cth_dn2;
        *var_cth_dn3_slot = var_cth_dn3;
        *var_cth_dn4_slot = var_cth_dn4;
        *var_cth_dn5_slot = var_cth_dn5;
        *var_cth_dn6_slot = var_cth_dn6;
        *var_cth_dn7_slot = var_cth_dn7;
        *var_cth_dn8_slot = var_cth_dn8;
        *var_cth_dn9_slot = var_cth_dn9;
        *var_ctnoi_slot = var_ctnoi;
        *var_ctnoi_dn0_slot = var_ctnoi_dn0;
        *var_ctnoi_dn10_slot = var_ctnoi_dn10;
        *var_ctnoi_dn11_slot = var_ctnoi_dn11;
        *var_ctnoi_dn13_slot = var_ctnoi_dn13;
        *var_ctnoi_dn14_slot = var_ctnoi_dn14;
        *var_ctnoi_dn2_slot = var_ctnoi_dn2;
        *var_ctnoi_dn3_slot = var_ctnoi_dn3;
        *var_ctnoi_dn4_slot = var_ctnoi_dn4;
        *var_ctnoi_dn5_slot = var_ctnoi_dn5;
        *var_ctnoi_dn6_slot = var_ctnoi_dn6;
        *var_ctnoi_dn7_slot = var_ctnoi_dn7;
        *var_ctnoi_dn8_slot = var_ctnoi_dn8;
        *var_ctnoi_dn9_slot = var_ctnoi_dn9;
        *var_dr0_slot = var_dr0;
        *var_dr0_dn0_slot = var_dr0_dn0;
        *var_dr0_dn10_slot = var_dr0_dn10;
        *var_dr0_dn11_slot = var_dr0_dn11;
        *var_dr0_dn13_slot = var_dr0_dn13;
        *var_dr0_dn14_slot = var_dr0_dn14;
        *var_dr0_dn2_slot = var_dr0_dn2;
        *var_dr0_dn3_slot = var_dr0_dn3;
        *var_dr0_dn4_slot = var_dr0_dn4;
        *var_dr0_dn5_slot = var_dr0_dn5;
        *var_dr0_dn6_slot = var_dr0_dn6;
        *var_dr0_dn7_slot = var_dr0_dn7;
        *var_dr0_dn8_slot = var_dr0_dn8;
        *var_dr0_dn9_slot = var_dr0_dn9;
        *var_dvtshiftr_i_slot = var_dvtshiftr_i;
        *var_eta0r_i_slot = var_eta0r_i;
        *var_eur_i_slot = var_eur_i;
        *var_eur_i_dn0_slot = var_eur_i_dn0;
        *var_eur_i_dn10_slot = var_eur_i_dn10;
        *var_eur_i_dn11_slot = var_eur_i_dn11;
        *var_eur_i_dn13_slot = var_eur_i_dn13;
        *var_eur_i_dn14_slot = var_eur_i_dn14;
        *var_eur_i_dn2_slot = var_eur_i_dn2;
        *var_eur_i_dn3_slot = var_eur_i_dn3;
        *var_eur_i_dn4_slot = var_eur_i_dn4;
        *var_eur_i_dn5_slot = var_eur_i_dn5;
        *var_eur_i_dn6_slot = var_eur_i_dn6;
        *var_eur_i_dn7_slot = var_eur_i_dn7;
        *var_eur_i_dn8_slot = var_eur_i_dn8;
        *var_eur_i_dn9_slot = var_eur_i_dn9;
        *var_gth_slot = var_gth;
        *var_gth_dn0_slot = var_gth_dn0;
        *var_gth_dn10_slot = var_gth_dn10;
        *var_gth_dn11_slot = var_gth_dn11;
        *var_gth_dn13_slot = var_gth_dn13;
        *var_gth_dn14_slot = var_gth_dn14;
        *var_gth_dn2_slot = var_gth_dn2;
        *var_gth_dn3_slot = var_gth_dn3;
        *var_gth_dn4_slot = var_gth_dn4;
        *var_gth_dn5_slot = var_gth_dn5;
        *var_gth_dn6_slot = var_gth_dn6;
        *var_gth_dn7_slot = var_gth_dn7;
        *var_gth_dn8_slot = var_gth_dn8;
        *var_gth_dn9_slot = var_gth_dn9;
        *var_k11_i_slot = var_k11_i;
        *var_k1_i_slot = var_k1_i;
        *var_k21_i_slot = var_k21_i;
        *var_k2_i_slot = var_k2_i;
        *var_k2sat1_i_slot = var_k2sat1_i;
        *var_k2sat_i_slot = var_k2sat_i;
        *var_ksativr_i_slot = var_ksativr_i;
        *var_lambdac_fn2_slot = var_lambdac_fn2;
        *var_lambdac_fn2_dn0_slot = var_lambdac_fn2_dn0;
        *var_lambdac_fn2_dn10_slot = var_lambdac_fn2_dn10;
        *var_lambdac_fn2_dn11_slot = var_lambdac_fn2_dn11;
        *var_lambdac_fn2_dn13_slot = var_lambdac_fn2_dn13;
        *var_lambdac_fn2_dn14_slot = var_lambdac_fn2_dn14;
        *var_lambdac_fn2_dn2_slot = var_lambdac_fn2_dn2;
        *var_lambdac_fn2_dn3_slot = var_lambdac_fn2_dn3;
        *var_lambdac_fn2_dn4_slot = var_lambdac_fn2_dn4;
        *var_lambdac_fn2_dn5_slot = var_lambdac_fn2_dn5;
        *var_lambdac_fn2_dn6_slot = var_lambdac_fn2_dn6;
        *var_lambdac_fn2_dn7_slot = var_lambdac_fn2_dn7;
        *var_lambdac_fn2_dn8_slot = var_lambdac_fn2_dn8;
        *var_lambdac_fn2_dn9_slot = var_lambdac_fn2_dn9;
        *var_mexpr_i_slot = var_mexpr_i;
        *var_mexpr_i_dn0_slot = var_mexpr_i_dn0;
        *var_mexpr_i_dn10_slot = var_mexpr_i_dn10;
        *var_mexpr_i_dn11_slot = var_mexpr_i_dn11;
        *var_mexpr_i_dn13_slot = var_mexpr_i_dn13;
        *var_mexpr_i_dn14_slot = var_mexpr_i_dn14;
        *var_mexpr_i_dn2_slot = var_mexpr_i_dn2;
        *var_mexpr_i_dn3_slot = var_mexpr_i_dn3;
        *var_mexpr_i_dn4_slot = var_mexpr_i_dn4;
        *var_mexpr_i_dn5_slot = var_mexpr_i_dn5;
        *var_mexpr_i_dn6_slot = var_mexpr_i_dn6;
        *var_mexpr_i_dn7_slot = var_mexpr_i_dn7;
        *var_mexpr_i_dn8_slot = var_mexpr_i_dn8;
        *var_mexpr_i_dn9_slot = var_mexpr_i_dn9;
        *var_mpower_i_slot = var_mpower_i;
        *var_noia2_i_slot = var_noia2_i;
        *var_nq_slot = var_nq;
        *var_nq_dn0_slot = var_nq_dn0;
        *var_nq_dn10_slot = var_nq_dn10;
        *var_nq_dn11_slot = var_nq_dn11;
        *var_nq_dn13_slot = var_nq_dn13;
        *var_nq_dn14_slot = var_nq_dn14;
        *var_nq_dn2_slot = var_nq_dn2;
        *var_nq_dn3_slot = var_nq_dn3;
        *var_nq_dn4_slot = var_nq_dn4;
        *var_nq_dn5_slot = var_nq_dn5;
        *var_nq_dn6_slot = var_nq_dn6;
        *var_nq_dn7_slot = var_nq_dn7;
        *var_nq_dn8_slot = var_nq_dn8;
        *var_nq_dn9_slot = var_nq_dn9;
        *var_pclmr_i_slot = var_pclmr_i;
        *var_pclmr_i_dn0_slot = var_pclmr_i_dn0;
        *var_pclmr_i_dn10_slot = var_pclmr_i_dn10;
        *var_pclmr_i_dn11_slot = var_pclmr_i_dn11;
        *var_pclmr_i_dn13_slot = var_pclmr_i_dn13;
        *var_pclmr_i_dn14_slot = var_pclmr_i_dn14;
        *var_pclmr_i_dn2_slot = var_pclmr_i_dn2;
        *var_pclmr_i_dn3_slot = var_pclmr_i_dn3;
        *var_pclmr_i_dn4_slot = var_pclmr_i_dn4;
        *var_pclmr_i_dn5_slot = var_pclmr_i_dn5;
        *var_pclmr_i_dn6_slot = var_pclmr_i_dn6;
        *var_pclmr_i_dn7_slot = var_pclmr_i_dn7;
        *var_pclmr_i_dn8_slot = var_pclmr_i_dn8;
        *var_pclmr_i_dn9_slot = var_pclmr_i_dn9;
        *var_pdibl1r_i_slot = var_pdibl1r_i;
        *var_pdibl2r_i_slot = var_pdibl2r_i;
        *var_phibe_i_slot = var_phibe_i;
        *var_ptwgr_i_slot = var_ptwgr_i;
        *var_ptwgr_i_dn0_slot = var_ptwgr_i_dn0;
        *var_ptwgr_i_dn10_slot = var_ptwgr_i_dn10;
        *var_ptwgr_i_dn11_slot = var_ptwgr_i_dn11;
        *var_ptwgr_i_dn13_slot = var_ptwgr_i_dn13;
        *var_ptwgr_i_dn14_slot = var_ptwgr_i_dn14;
        *var_ptwgr_i_dn2_slot = var_ptwgr_i_dn2;
        *var_ptwgr_i_dn3_slot = var_ptwgr_i_dn3;
        *var_ptwgr_i_dn4_slot = var_ptwgr_i_dn4;
        *var_ptwgr_i_dn5_slot = var_ptwgr_i_dn5;
        *var_ptwgr_i_dn6_slot = var_ptwgr_i_dn6;
        *var_ptwgr_i_dn7_slot = var_ptwgr_i_dn7;
        *var_ptwgr_i_dn8_slot = var_ptwgr_i_dn8;
        *var_ptwgr_i_dn9_slot = var_ptwgr_i_dn9;
        *var_qbov_slot = var_qbov;
        *var_qbov_dn0_slot = var_qbov_dn0;
        *var_qbov_dn10_slot = var_qbov_dn10;
        *var_qbov_dn11_slot = var_qbov_dn11;
        *var_qbov_dn13_slot = var_qbov_dn13;
        *var_qbov_dn14_slot = var_qbov_dn14;
        *var_qbov_dn2_slot = var_qbov_dn2;
        *var_qbov_dn3_slot = var_qbov_dn3;
        *var_qbov_dn4_slot = var_qbov_dn4;
        *var_qbov_dn5_slot = var_qbov_dn5;
        *var_qbov_dn6_slot = var_qbov_dn6;
        *var_qbov_dn7_slot = var_qbov_dn7;
        *var_qbov_dn8_slot = var_qbov_dn8;
        *var_qbov_dn9_slot = var_qbov_dn9;
        *var_qbov_s_slot = var_qbov_s;
        *var_qbov_s_dn0_slot = var_qbov_s_dn0;
        *var_qbov_s_dn10_slot = var_qbov_s_dn10;
        *var_qbov_s_dn11_slot = var_qbov_s_dn11;
        *var_qbov_s_dn13_slot = var_qbov_s_dn13;
        *var_qbov_s_dn14_slot = var_qbov_s_dn14;
        *var_qbov_s_dn2_slot = var_qbov_s_dn2;
        *var_qbov_s_dn3_slot = var_qbov_s_dn3;
        *var_qbov_s_dn4_slot = var_qbov_s_dn4;
        *var_qbov_s_dn5_slot = var_qbov_s_dn5;
        *var_qbov_s_dn6_slot = var_qbov_s_dn6;
        *var_qbov_s_dn7_slot = var_qbov_s_dn7;
        *var_qbov_s_dn8_slot = var_qbov_s_dn8;
        *var_qbov_s_dn9_slot = var_qbov_s_dn9;
        *var_qdep_ov_cins_slot = var_qdep_ov_cins;
        *var_qi_acc_for_qm_slot = var_qi_acc_for_qm;
        *var_qi_acc_for_qm_dn0_slot = var_qi_acc_for_qm_dn0;
        *var_qi_acc_for_qm_dn10_slot = var_qi_acc_for_qm_dn10;
        *var_qi_acc_for_qm_dn11_slot = var_qi_acc_for_qm_dn11;
        *var_qi_acc_for_qm_dn13_slot = var_qi_acc_for_qm_dn13;
        *var_qi_acc_for_qm_dn14_slot = var_qi_acc_for_qm_dn14;
        *var_qi_acc_for_qm_dn2_slot = var_qi_acc_for_qm_dn2;
        *var_qi_acc_for_qm_dn3_slot = var_qi_acc_for_qm_dn3;
        *var_qi_acc_for_qm_dn4_slot = var_qi_acc_for_qm_dn4;
        *var_qi_acc_for_qm_dn5_slot = var_qi_acc_for_qm_dn5;
        *var_qi_acc_for_qm_dn6_slot = var_qi_acc_for_qm_dn6;
        *var_qi_acc_for_qm_dn7_slot = var_qi_acc_for_qm_dn7;
        *var_qi_acc_for_qm_dn8_slot = var_qi_acc_for_qm_dn8;
        *var_qi_acc_for_qm_dn9_slot = var_qi_acc_for_qm_dn9;
        *var_qid_slot = var_qid;
        *var_qid_dn0_slot = var_qid_dn0;
        *var_qid_dn10_slot = var_qid_dn10;
        *var_qid_dn11_slot = var_qid_dn11;
        *var_qid_dn13_slot = var_qid_dn13;
        *var_qid_dn14_slot = var_qid_dn14;
        *var_qid_dn2_slot = var_qid_dn2;
        *var_qid_dn3_slot = var_qid_dn3;
        *var_qid_dn4_slot = var_qid_dn4;
        *var_qid_dn5_slot = var_qid_dn5;
        *var_qid_dn6_slot = var_qid_dn6;
        *var_qid_dn7_slot = var_qid_dn7;
        *var_qid_dn8_slot = var_qid_dn8;
        *var_qid_dn9_slot = var_qid_dn9;
        *var_qis_slot = var_qis;
        *var_qis_dn0_slot = var_qis_dn0;
        *var_qis_dn10_slot = var_qis_dn10;
        *var_qis_dn11_slot = var_qis_dn11;
        *var_qis_dn13_slot = var_qis_dn13;
        *var_qis_dn14_slot = var_qis_dn14;
        *var_qis_dn2_slot = var_qis_dn2;
        *var_qis_dn3_slot = var_qis_dn3;
        *var_qis_dn4_slot = var_qis_dn4;
        *var_qis_dn5_slot = var_qis_dn5;
        *var_qis_dn6_slot = var_qis_dn6;
        *var_qis_dn7_slot = var_qis_dn7;
        *var_qis_dn8_slot = var_qis_dn8;
        *var_qis_dn9_slot = var_qis_dn9;
        *var_qndd1_slot = var_qndd1;
        *var_qndd1_dn0_slot = var_qndd1_dn0;
        *var_qndd1_dn10_slot = var_qndd1_dn10;
        *var_qndd1_dn11_slot = var_qndd1_dn11;
        *var_qndd1_dn13_slot = var_qndd1_dn13;
        *var_qndd1_dn14_slot = var_qndd1_dn14;
        *var_qndd1_dn2_slot = var_qndd1_dn2;
        *var_qndd1_dn3_slot = var_qndd1_dn3;
        *var_qndd1_dn4_slot = var_qndd1_dn4;
        *var_qndd1_dn5_slot = var_qndd1_dn5;
        *var_qndd1_dn6_slot = var_qndd1_dn6;
        *var_qndd1_dn7_slot = var_qndd1_dn7;
        *var_qndd1_dn8_slot = var_qndd1_dn8;
        *var_qndd1_dn9_slot = var_qndd1_dn9;
        *var_qnds1_slot = var_qnds1;
        *var_qnds1_dn0_slot = var_qnds1_dn0;
        *var_qnds1_dn10_slot = var_qnds1_dn10;
        *var_qnds1_dn11_slot = var_qnds1_dn11;
        *var_qnds1_dn13_slot = var_qnds1_dn13;
        *var_qnds1_dn14_slot = var_qnds1_dn14;
        *var_qnds1_dn2_slot = var_qnds1_dn2;
        *var_qnds1_dn3_slot = var_qnds1_dn3;
        *var_qnds1_dn4_slot = var_qnds1_dn4;
        *var_qnds1_dn5_slot = var_qnds1_dn5;
        *var_qnds1_dn6_slot = var_qnds1_dn6;
        *var_qnds1_dn7_slot = var_qnds1_dn7;
        *var_qnds1_dn8_slot = var_qnds1_dn8;
        *var_qnds1_dn9_slot = var_qnds1_dn9;
        *var_qnds2_slot = var_qnds2;
        *var_qnds2_dn0_slot = var_qnds2_dn0;
        *var_qnds2_dn10_slot = var_qnds2_dn10;
        *var_qnds2_dn11_slot = var_qnds2_dn11;
        *var_qnds2_dn13_slot = var_qnds2_dn13;
        *var_qnds2_dn14_slot = var_qnds2_dn14;
        *var_qnds2_dn2_slot = var_qnds2_dn2;
        *var_qnds2_dn3_slot = var_qnds2_dn3;
        *var_qnds2_dn4_slot = var_qnds2_dn4;
        *var_qnds2_dn5_slot = var_qnds2_dn5;
        *var_qnds2_dn6_slot = var_qnds2_dn6;
        *var_qnds2_dn7_slot = var_qnds2_dn7;
        *var_qnds2_dn8_slot = var_qnds2_dn8;
        *var_qnds2_dn9_slot = var_qnds2_dn9;
        *var_qnds3_slot = var_qnds3;
        *var_qnds3_dn0_slot = var_qnds3_dn0;
        *var_qnds3_dn10_slot = var_qnds3_dn10;
        *var_qnds3_dn11_slot = var_qnds3_dn11;
        *var_qnds3_dn13_slot = var_qnds3_dn13;
        *var_qnds3_dn14_slot = var_qnds3_dn14;
        *var_qnds3_dn2_slot = var_qnds3_dn2;
        *var_qnds3_dn3_slot = var_qnds3_dn3;
        *var_qnds3_dn4_slot = var_qnds3_dn4;
        *var_qnds3_dn5_slot = var_qnds3_dn5;
        *var_qnds3_dn6_slot = var_qnds3_dn6;
        *var_qnds3_dn7_slot = var_qnds3_dn7;
        *var_qnds3_dn8_slot = var_qnds3_dn8;
        *var_qnds3_dn9_slot = var_qnds3_dn9;
        *var_rc_slot = var_rc;
        *var_sigrat_slot = var_sigrat;
        *var_sigrat_dn0_slot = var_sigrat_dn0;
        *var_sigrat_dn10_slot = var_sigrat_dn10;
        *var_sigrat_dn11_slot = var_sigrat_dn11;
        *var_sigrat_dn13_slot = var_sigrat_dn13;
        *var_sigrat_dn14_slot = var_sigrat_dn14;
        *var_sigrat_dn2_slot = var_sigrat_dn2;
        *var_sigrat_dn3_slot = var_sigrat_dn3;
        *var_sigrat_dn4_slot = var_sigrat_dn4;
        *var_sigrat_dn5_slot = var_sigrat_dn5;
        *var_sigrat_dn6_slot = var_sigrat_dn6;
        *var_sigrat_dn7_slot = var_sigrat_dn7;
        *var_sigrat_dn8_slot = var_sigrat_dn8;
        *var_sigrat_dn9_slot = var_sigrat_dn9;
        *var_u0r_i_slot = var_u0r_i;
        *var_u0r_i_dn0_slot = var_u0r_i_dn0;
        *var_u0r_i_dn10_slot = var_u0r_i_dn10;
        *var_u0r_i_dn11_slot = var_u0r_i_dn11;
        *var_u0r_i_dn13_slot = var_u0r_i_dn13;
        *var_u0r_i_dn14_slot = var_u0r_i_dn14;
        *var_u0r_i_dn2_slot = var_u0r_i_dn2;
        *var_u0r_i_dn3_slot = var_u0r_i_dn3;
        *var_u0r_i_dn4_slot = var_u0r_i_dn4;
        *var_u0r_i_dn5_slot = var_u0r_i_dn5;
        *var_u0r_i_dn6_slot = var_u0r_i_dn6;
        *var_u0r_i_dn7_slot = var_u0r_i_dn7;
        *var_u0r_i_dn8_slot = var_u0r_i_dn8;
        *var_u0r_i_dn9_slot = var_u0r_i_dn9;
        *var_ua1r_i_slot = var_ua1r_i;
        *var_uar_i_slot = var_uar_i;
        *var_uar_i_dn0_slot = var_uar_i_dn0;
        *var_uar_i_dn10_slot = var_uar_i_dn10;
        *var_uar_i_dn11_slot = var_uar_i_dn11;
        *var_uar_i_dn13_slot = var_uar_i_dn13;
        *var_uar_i_dn14_slot = var_uar_i_dn14;
        *var_uar_i_dn2_slot = var_uar_i_dn2;
        *var_uar_i_dn3_slot = var_uar_i_dn3;
        *var_uar_i_dn4_slot = var_uar_i_dn4;
        *var_uar_i_dn5_slot = var_uar_i_dn5;
        *var_uar_i_dn6_slot = var_uar_i_dn6;
        *var_uar_i_dn7_slot = var_uar_i_dn7;
        *var_uar_i_dn8_slot = var_uar_i_dn8;
        *var_uar_i_dn9_slot = var_uar_i_dn9;
        *var_uc1_i_slot = var_uc1_i;
        *var_uc1r_i_slot = var_uc1r_i;
        *var_uc_i_slot = var_uc_i;
        *var_ucr_i_slot = var_ucr_i;
        *var_ud1r_i_slot = var_ud1r_i;
        *var_udr_i_slot = var_udr_i;
        *var_udr_i_dn0_slot = var_udr_i_dn0;
        *var_udr_i_dn10_slot = var_udr_i_dn10;
        *var_udr_i_dn11_slot = var_udr_i_dn11;
        *var_udr_i_dn13_slot = var_udr_i_dn13;
        *var_udr_i_dn14_slot = var_udr_i_dn14;
        *var_udr_i_dn2_slot = var_udr_i_dn2;
        *var_udr_i_dn3_slot = var_udr_i_dn3;
        *var_udr_i_dn4_slot = var_udr_i_dn4;
        *var_udr_i_dn5_slot = var_udr_i_dn5;
        *var_udr_i_dn6_slot = var_udr_i_dn6;
        *var_udr_i_dn7_slot = var_udr_i_dn7;
        *var_udr_i_dn8_slot = var_udr_i_dn8;
        *var_udr_i_dn9_slot = var_udr_i_dn9;
        *var_upr_i_slot = var_upr_i;
        *var_uter_i_slot = var_uter_i;
        *var_utlr_i_slot = var_utlr_i;
        *var_veseff_slot = var_veseff;
        *var_veseff_dn0_slot = var_veseff_dn0;
        *var_veseff_dn10_slot = var_veseff_dn10;
        *var_veseff_dn11_slot = var_veseff_dn11;
        *var_veseff_dn13_slot = var_veseff_dn13;
        *var_veseff_dn14_slot = var_veseff_dn14;
        *var_veseff_dn2_slot = var_veseff_dn2;
        *var_veseff_dn3_slot = var_veseff_dn3;
        *var_veseff_dn4_slot = var_veseff_dn4;
        *var_veseff_dn5_slot = var_veseff_dn5;
        *var_veseff_dn6_slot = var_veseff_dn6;
        *var_veseff_dn7_slot = var_veseff_dn7;
        *var_veseff_dn8_slot = var_veseff_dn8;
        *var_veseff_dn9_slot = var_veseff_dn9;
        *var_vsat1r_i_slot = var_vsat1r_i;
        *var_vsat1r_i_dn0_slot = var_vsat1r_i_dn0;
        *var_vsat1r_i_dn10_slot = var_vsat1r_i_dn10;
        *var_vsat1r_i_dn11_slot = var_vsat1r_i_dn11;
        *var_vsat1r_i_dn13_slot = var_vsat1r_i_dn13;
        *var_vsat1r_i_dn14_slot = var_vsat1r_i_dn14;
        *var_vsat1r_i_dn2_slot = var_vsat1r_i_dn2;
        *var_vsat1r_i_dn3_slot = var_vsat1r_i_dn3;
        *var_vsat1r_i_dn4_slot = var_vsat1r_i_dn4;
        *var_vsat1r_i_dn5_slot = var_vsat1r_i_dn5;
        *var_vsat1r_i_dn6_slot = var_vsat1r_i_dn6;
        *var_vsat1r_i_dn7_slot = var_vsat1r_i_dn7;
        *var_vsat1r_i_dn8_slot = var_vsat1r_i_dn8;
        *var_vsat1r_i_dn9_slot = var_vsat1r_i_dn9;
        *var_vsatr_i_slot = var_vsatr_i;
        *var_weff2_slot = var_weff2;
        *var_weff3_slot = var_weff3;
        *var_weff4_slot = var_weff4;
        *var_weff5_slot = var_weff5;
        *var_weff6_slot = var_weff6;
        *var_weff_ufcm_slot = var_weff_ufcm;
        *var_weffb_slot = var_weffb;
        *var_xrcrg1_i_slot = var_xrcrg1_i;
        *var_xrcrg2_i_slot = var_xrcrg2_i;
    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        var_cbox_slot: &mut f64,
        var_d1_slot: &mut f64,
        var_d2_slot: &mut f64,
        var_d3_slot: &mut f64,
        var_deltal_slot: &mut f64,
        var_deltal1_slot: &mut f64,
        var_deltal_dn0_slot: &mut f64,
        var_deltal_dn10_slot: &mut f64,
        var_deltal_dn11_slot: &mut f64,
        var_deltal_dn13_slot: &mut f64,
        var_deltal_dn14_slot: &mut f64,
        var_deltal_dn2_slot: &mut f64,
        var_deltal_dn3_slot: &mut f64,
        var_deltal_dn4_slot: &mut f64,
        var_deltal_dn5_slot: &mut f64,
        var_deltal_dn6_slot: &mut f64,
        var_deltal_dn7_slot: &mut f64,
        var_deltal_dn8_slot: &mut f64,
        var_deltal_dn9_slot: &mut f64,
        var_deltalcv_slot: &mut f64,
        var_deltalcv_dn0_slot: &mut f64,
        var_deltalcv_dn10_slot: &mut f64,
        var_deltalcv_dn11_slot: &mut f64,
        var_deltalcv_dn13_slot: &mut f64,
        var_deltalcv_dn14_slot: &mut f64,
        var_deltalcv_dn2_slot: &mut f64,
        var_deltalcv_dn3_slot: &mut f64,
        var_deltalcv_dn4_slot: &mut f64,
        var_deltalcv_dn5_slot: &mut f64,
        var_deltalcv_dn6_slot: &mut f64,
        var_deltalcv_dn7_slot: &mut f64,
        var_deltalcv_dn8_slot: &mut f64,
        var_deltalcv_dn9_slot: &mut f64,
        var_deltaprsd_v_slot: &mut f64,
        var_deltemp1_slot: &mut f64,
        var_deltemp1_dn4_slot: &mut f64,
        var_deltratio1_slot: &mut f64,
        var_deltratio1_dn4_slot: &mut f64,
        var_devsign_slot: &mut f64,
        var_devtemp1_slot: &mut f64,
        var_devtemp1_dn4_slot: &mut f64,
        var_devtempeff_slot: &mut f64,
        var_devtempeff_dn0_slot: &mut f64,
        var_devtempeff_dn10_slot: &mut f64,
        var_devtempeff_dn11_slot: &mut f64,
        var_devtempeff_dn13_slot: &mut f64,
        var_devtempeff_dn14_slot: &mut f64,
        var_devtempeff_dn2_slot: &mut f64,
        var_devtempeff_dn3_slot: &mut f64,
        var_devtempeff_dn4_slot: &mut f64,
        var_devtempeff_dn5_slot: &mut f64,
        var_devtempeff_dn6_slot: &mut f64,
        var_devtempeff_dn7_slot: &mut f64,
        var_devtempeff_dn8_slot: &mut f64,
        var_devtempeff_dn9_slot: &mut f64,
        var_devtemplow0_slot: &mut f64,
        var_devtemplow0_dn4_slot: &mut f64,
        var_devtemplow1_slot: &mut f64,
        var_devtemplow1_dn4_slot: &mut f64,
        var_dimension1_i_slot: &mut f64,
        var_dimension2_i_slot: &mut f64,
        var_dimension3_i_slot: &mut f64,
        var_dlbin_i_slot: &mut f64,
        var_e2nom_i_slot: &mut f64,
        var_e3nom_i_slot: &mut f64,
        var_epsratio_slot: &mut f64,
        var_epssp_slot: &mut f64,
        var_epssub_slot: &mut f64,
        var_gdpr_slot: &mut f64,
        var_gdpr_dn0_slot: &mut f64,
        var_gdpr_dn10_slot: &mut f64,
        var_gdpr_dn11_slot: &mut f64,
        var_gdpr_dn13_slot: &mut f64,
        var_gdpr_dn14_slot: &mut f64,
        var_gdpr_dn2_slot: &mut f64,
        var_gdpr_dn3_slot: &mut f64,
        var_gdpr_dn4_slot: &mut f64,
        var_gdpr_dn5_slot: &mut f64,
        var_gdpr_dn6_slot: &mut f64,
        var_gdpr_dn7_slot: &mut f64,
        var_gdpr_dn8_slot: &mut f64,
        var_gdpr_dn9_slot: &mut f64,
        var_gfactor_slot: &mut f64,
        var_gmin_slot: &mut f64,
        var_gprime_slot: &mut f64,
        var_gspr_slot: &mut f64,
        var_gspr_dn0_slot: &mut f64,
        var_gspr_dn10_slot: &mut f64,
        var_gspr_dn11_slot: &mut f64,
        var_gspr_dn13_slot: &mut f64,
        var_gspr_dn14_slot: &mut f64,
        var_gspr_dn2_slot: &mut f64,
        var_gspr_dn3_slot: &mut f64,
        var_gspr_dn4_slot: &mut f64,
        var_gspr_dn5_slot: &mut f64,
        var_gspr_dn6_slot: &mut f64,
        var_gspr_dn7_slot: &mut f64,
        var_gspr_dn8_slot: &mut f64,
        var_gspr_dn9_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_gvs_d_slot: &mut f64,
        var_gvs_d_dn0_slot: &mut f64,
        var_gvs_d_dn10_slot: &mut f64,
        var_gvs_d_dn11_slot: &mut f64,
        var_gvs_d_dn13_slot: &mut f64,
        var_gvs_d_dn14_slot: &mut f64,
        var_gvs_d_dn2_slot: &mut f64,
        var_gvs_d_dn3_slot: &mut f64,
        var_gvs_d_dn4_slot: &mut f64,
        var_gvs_d_dn5_slot: &mut f64,
        var_gvs_d_dn6_slot: &mut f64,
        var_gvs_d_dn7_slot: &mut f64,
        var_gvs_d_dn8_slot: &mut f64,
        var_gvs_d_dn9_slot: &mut f64,
        var_gvs_s_slot: &mut f64,
        var_gvs_s_dn0_slot: &mut f64,
        var_gvs_s_dn10_slot: &mut f64,
        var_gvs_s_dn11_slot: &mut f64,
        var_gvs_s_dn13_slot: &mut f64,
        var_gvs_s_dn14_slot: &mut f64,
        var_gvs_s_dn2_slot: &mut f64,
        var_gvs_s_dn3_slot: &mut f64,
        var_gvs_s_dn4_slot: &mut f64,
        var_gvs_s_dn5_slot: &mut f64,
        var_gvs_s_dn6_slot: &mut f64,
        var_gvs_s_dn7_slot: &mut f64,
        var_gvs_s_dn8_slot: &mut f64,
        var_gvs_s_dn9_slot: &mut f64,
        var_leff1_slot: &mut f64,
        var_leff_1_slot: &mut f64,
        var_leff_1_dn0_slot: &mut f64,
        var_leff_1_dn10_slot: &mut f64,
        var_leff_1_dn11_slot: &mut f64,
        var_leff_1_dn13_slot: &mut f64,
        var_leff_1_dn14_slot: &mut f64,
        var_leff_1_dn2_slot: &mut f64,
        var_leff_1_dn3_slot: &mut f64,
        var_leff_1_dn4_slot: &mut f64,
        var_leff_1_dn5_slot: &mut f64,
        var_leff_1_dn6_slot: &mut f64,
        var_leff_1_dn7_slot: &mut f64,
        var_leff_1_dn8_slot: &mut f64,
        var_leff_1_dn9_slot: &mut f64,
        var_lg_slot: &mut f64,
        var_lint_i_slot: &mut f64,
        var_md_slot: &mut f64,
        var_mdprime_slot: &mut f64,
        var_mfq1nom_i_slot: &mut f64,
        var_mfq2nom_i_slot: &mut f64,
        var_mfq3nom_i_slot: &mut f64,
        var_mx_slot: &mut f64,
        var_mxprime_slot: &mut f64,
        var_nc3d_slot: &mut f64,
        var_ncq_slot: &mut f64,
        var_niln_slot: &mut f64,
        var_niln_dn0_slot: &mut f64,
        var_niln_dn10_slot: &mut f64,
        var_niln_dn11_slot: &mut f64,
        var_niln_dn13_slot: &mut f64,
        var_niln_dn14_slot: &mut f64,
        var_niln_dn2_slot: &mut f64,
        var_niln_dn3_slot: &mut f64,
        var_niln_dn4_slot: &mut f64,
        var_niln_dn5_slot: &mut f64,
        var_niln_dn6_slot: &mut f64,
        var_niln_dn7_slot: &mut f64,
        var_niln_dn8_slot: &mut f64,
        var_niln_dn9_slot: &mut f64,
        var_p1_slot: &mut f64,
        var_p2_slot: &mut f64,
        var_p3_slot: &mut f64,
        var_qe2_slot: &mut f64,
        var_qe2_dn0_slot: &mut f64,
        var_qe2_dn10_slot: &mut f64,
        var_qe2_dn11_slot: &mut f64,
        var_qe2_dn13_slot: &mut f64,
        var_qe2_dn14_slot: &mut f64,
        var_qe2_dn2_slot: &mut f64,
        var_qe2_dn3_slot: &mut f64,
        var_qe2_dn4_slot: &mut f64,
        var_qe2_dn5_slot: &mut f64,
        var_qe2_dn6_slot: &mut f64,
        var_qe2_dn7_slot: &mut f64,
        var_qe2_dn8_slot: &mut f64,
        var_qe2_dn9_slot: &mut f64,
        var_qe3_slot: &mut f64,
        var_qnd10_slot: &mut f64,
        var_qnd20_slot: &mut f64,
        var_qnd30_slot: &mut f64,
        var_qndd2_slot: &mut f64,
        var_qndd2_dn0_slot: &mut f64,
        var_qndd2_dn10_slot: &mut f64,
        var_qndd2_dn11_slot: &mut f64,
        var_qndd2_dn13_slot: &mut f64,
        var_qndd2_dn14_slot: &mut f64,
        var_qndd2_dn2_slot: &mut f64,
        var_qndd2_dn3_slot: &mut f64,
        var_qndd2_dn4_slot: &mut f64,
        var_qndd2_dn5_slot: &mut f64,
        var_qndd2_dn6_slot: &mut f64,
        var_qndd2_dn7_slot: &mut f64,
        var_qndd2_dn8_slot: &mut f64,
        var_qndd2_dn9_slot: &mut f64,
        var_qndd3_slot: &mut f64,
        var_qndd3_dn0_slot: &mut f64,
        var_qndd3_dn10_slot: &mut f64,
        var_qndd3_dn11_slot: &mut f64,
        var_qndd3_dn13_slot: &mut f64,
        var_qndd3_dn14_slot: &mut f64,
        var_qndd3_dn2_slot: &mut f64,
        var_qndd3_dn3_slot: &mut f64,
        var_qndd3_dn4_slot: &mut f64,
        var_qndd3_dn5_slot: &mut f64,
        var_qndd3_dn6_slot: &mut f64,
        var_qndd3_dn7_slot: &mut f64,
        var_qndd3_dn8_slot: &mut f64,
        var_qndd3_dn9_slot: &mut f64,
        var_qsref_i_slot: &mut f64,
        var_rdstempvs_slot: &mut f64,
        var_rdstempvs_dn4_slot: &mut f64,
        var_rvs_d_slot: &mut f64,
        var_rvs_d_dn0_slot: &mut f64,
        var_rvs_d_dn10_slot: &mut f64,
        var_rvs_d_dn11_slot: &mut f64,
        var_rvs_d_dn13_slot: &mut f64,
        var_rvs_d_dn14_slot: &mut f64,
        var_rvs_d_dn2_slot: &mut f64,
        var_rvs_d_dn3_slot: &mut f64,
        var_rvs_d_dn4_slot: &mut f64,
        var_rvs_d_dn5_slot: &mut f64,
        var_rvs_d_dn6_slot: &mut f64,
        var_rvs_d_dn7_slot: &mut f64,
        var_rvs_d_dn8_slot: &mut f64,
        var_rvs_d_dn9_slot: &mut f64,
        var_rvs_s_slot: &mut f64,
        var_rvs_s_dn0_slot: &mut f64,
        var_rvs_s_dn10_slot: &mut f64,
        var_rvs_s_dn11_slot: &mut f64,
        var_rvs_s_dn13_slot: &mut f64,
        var_rvs_s_dn14_slot: &mut f64,
        var_rvs_s_dn2_slot: &mut f64,
        var_rvs_s_dn3_slot: &mut f64,
        var_rvs_s_dn4_slot: &mut f64,
        var_rvs_s_dn5_slot: &mut f64,
        var_rvs_s_dn6_slot: &mut f64,
        var_rvs_s_dn7_slot: &mut f64,
        var_rvs_s_dn8_slot: &mut f64,
        var_rvs_s_dn9_slot: &mut f64,
        var_ssp1_i_slot: &mut f64,
        var_ssp2_i_slot: &mut f64,
        var_ssp3_i_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_ua_th_slot: &mut f64,
        var_ua_th_dn0_slot: &mut f64,
        var_ua_th_dn10_slot: &mut f64,
        var_ua_th_dn11_slot: &mut f64,
        var_ua_th_dn13_slot: &mut f64,
        var_ua_th_dn14_slot: &mut f64,
        var_ua_th_dn2_slot: &mut f64,
        var_ua_th_dn3_slot: &mut f64,
        var_ua_th_dn4_slot: &mut f64,
        var_ua_th_dn5_slot: &mut f64,
        var_ua_th_dn6_slot: &mut f64,
        var_ua_th_dn7_slot: &mut f64,
        var_ua_th_dn8_slot: &mut f64,
        var_ua_th_dn9_slot: &mut f64,
        var_ua_tl_slot: &mut f64,
        var_ua_tl_dn0_slot: &mut f64,
        var_ua_tl_dn10_slot: &mut f64,
        var_ua_tl_dn11_slot: &mut f64,
        var_ua_tl_dn13_slot: &mut f64,
        var_ua_tl_dn14_slot: &mut f64,
        var_ua_tl_dn2_slot: &mut f64,
        var_ua_tl_dn3_slot: &mut f64,
        var_ua_tl_dn4_slot: &mut f64,
        var_ua_tl_dn5_slot: &mut f64,
        var_ua_tl_dn6_slot: &mut f64,
        var_ua_tl_dn7_slot: &mut f64,
        var_ua_tl_dn8_slot: &mut f64,
        var_ua_tl_dn9_slot: &mut f64,
        var_uar_th_slot: &mut f64,
        var_uar_th_dn0_slot: &mut f64,
        var_uar_th_dn10_slot: &mut f64,
        var_uar_th_dn11_slot: &mut f64,
        var_uar_th_dn13_slot: &mut f64,
        var_uar_th_dn14_slot: &mut f64,
        var_uar_th_dn2_slot: &mut f64,
        var_uar_th_dn3_slot: &mut f64,
        var_uar_th_dn4_slot: &mut f64,
        var_uar_th_dn5_slot: &mut f64,
        var_uar_th_dn6_slot: &mut f64,
        var_uar_th_dn7_slot: &mut f64,
        var_uar_th_dn8_slot: &mut f64,
        var_uar_th_dn9_slot: &mut f64,
        var_uar_tl_slot: &mut f64,
        var_uar_tl_dn0_slot: &mut f64,
        var_uar_tl_dn10_slot: &mut f64,
        var_uar_tl_dn11_slot: &mut f64,
        var_uar_tl_dn13_slot: &mut f64,
        var_uar_tl_dn14_slot: &mut f64,
        var_uar_tl_dn2_slot: &mut f64,
        var_uar_tl_dn3_slot: &mut f64,
        var_uar_tl_dn4_slot: &mut f64,
        var_uar_tl_dn5_slot: &mut f64,
        var_uar_tl_dn6_slot: &mut f64,
        var_uar_tl_dn7_slot: &mut f64,
        var_uar_tl_dn8_slot: &mut f64,
        var_uar_tl_dn9_slot: &mut f64,
        var_udd_t_slot: &mut f64,
        var_udd_t_dn4_slot: &mut f64,
        var_uddeff_t_slot: &mut f64,
        var_uddeff_t_dn4_slot: &mut f64,
        var_uds_t_slot: &mut f64,
        var_uds_t_dn4_slot: &mut f64,
        var_udseff_t_slot: &mut f64,
        var_udseff_t_dn4_slot: &mut f64,
        var_vtmeff_slot: &mut f64,
        var_vtmeff_dn0_slot: &mut f64,
        var_vtmeff_dn10_slot: &mut f64,
        var_vtmeff_dn11_slot: &mut f64,
        var_vtmeff_dn13_slot: &mut f64,
        var_vtmeff_dn14_slot: &mut f64,
        var_vtmeff_dn2_slot: &mut f64,
        var_vtmeff_dn3_slot: &mut f64,
        var_vtmeff_dn4_slot: &mut f64,
        var_vtmeff_dn5_slot: &mut f64,
        var_vtmeff_dn6_slot: &mut f64,
        var_vtmeff_dn7_slot: &mut f64,
        var_vtmeff_dn8_slot: &mut f64,
        var_vtmeff_dn9_slot: &mut f64,
        var_wh_slot: &mut f64,
        var_wh_dn4_slot: &mut f64,
        var_wl_slot: &mut f64,
        var_wl_dn4_slot: &mut f64,
        var_xl_i_slot: &mut f64,
    ) {
        let mut var_cbox: f64 = *var_cbox_slot;
        let mut var_d1: f64 = *var_d1_slot;
        let mut var_d2: f64 = *var_d2_slot;
        let mut var_d3: f64 = *var_d3_slot;
        let mut var_deltal: f64 = *var_deltal_slot;
        let mut var_deltal1: f64 = *var_deltal1_slot;
        let mut var_deltal_dn0: f64 = *var_deltal_dn0_slot;
        let mut var_deltal_dn10: f64 = *var_deltal_dn10_slot;
        let mut var_deltal_dn11: f64 = *var_deltal_dn11_slot;
        let mut var_deltal_dn13: f64 = *var_deltal_dn13_slot;
        let mut var_deltal_dn14: f64 = *var_deltal_dn14_slot;
        let mut var_deltal_dn2: f64 = *var_deltal_dn2_slot;
        let mut var_deltal_dn3: f64 = *var_deltal_dn3_slot;
        let mut var_deltal_dn4: f64 = *var_deltal_dn4_slot;
        let mut var_deltal_dn5: f64 = *var_deltal_dn5_slot;
        let mut var_deltal_dn6: f64 = *var_deltal_dn6_slot;
        let mut var_deltal_dn7: f64 = *var_deltal_dn7_slot;
        let mut var_deltal_dn8: f64 = *var_deltal_dn8_slot;
        let mut var_deltal_dn9: f64 = *var_deltal_dn9_slot;
        let mut var_deltalcv: f64 = *var_deltalcv_slot;
        let mut var_deltalcv_dn0: f64 = *var_deltalcv_dn0_slot;
        let mut var_deltalcv_dn10: f64 = *var_deltalcv_dn10_slot;
        let mut var_deltalcv_dn11: f64 = *var_deltalcv_dn11_slot;
        let mut var_deltalcv_dn13: f64 = *var_deltalcv_dn13_slot;
        let mut var_deltalcv_dn14: f64 = *var_deltalcv_dn14_slot;
        let mut var_deltalcv_dn2: f64 = *var_deltalcv_dn2_slot;
        let mut var_deltalcv_dn3: f64 = *var_deltalcv_dn3_slot;
        let mut var_deltalcv_dn4: f64 = *var_deltalcv_dn4_slot;
        let mut var_deltalcv_dn5: f64 = *var_deltalcv_dn5_slot;
        let mut var_deltalcv_dn6: f64 = *var_deltalcv_dn6_slot;
        let mut var_deltalcv_dn7: f64 = *var_deltalcv_dn7_slot;
        let mut var_deltalcv_dn8: f64 = *var_deltalcv_dn8_slot;
        let mut var_deltalcv_dn9: f64 = *var_deltalcv_dn9_slot;
        let mut var_deltaprsd_v: f64 = *var_deltaprsd_v_slot;
        let mut var_deltemp1: f64 = *var_deltemp1_slot;
        let mut var_deltemp1_dn4: f64 = *var_deltemp1_dn4_slot;
        let mut var_deltratio1: f64 = *var_deltratio1_slot;
        let mut var_deltratio1_dn4: f64 = *var_deltratio1_dn4_slot;
        let mut var_devsign: f64 = *var_devsign_slot;
        let mut var_devtemp1: f64 = *var_devtemp1_slot;
        let mut var_devtemp1_dn4: f64 = *var_devtemp1_dn4_slot;
        let mut var_devtempeff: f64 = *var_devtempeff_slot;
        let mut var_devtempeff_dn0: f64 = *var_devtempeff_dn0_slot;
        let mut var_devtempeff_dn10: f64 = *var_devtempeff_dn10_slot;
        let mut var_devtempeff_dn11: f64 = *var_devtempeff_dn11_slot;
        let mut var_devtempeff_dn13: f64 = *var_devtempeff_dn13_slot;
        let mut var_devtempeff_dn14: f64 = *var_devtempeff_dn14_slot;
        let mut var_devtempeff_dn2: f64 = *var_devtempeff_dn2_slot;
        let mut var_devtempeff_dn3: f64 = *var_devtempeff_dn3_slot;
        let mut var_devtempeff_dn4: f64 = *var_devtempeff_dn4_slot;
        let mut var_devtempeff_dn5: f64 = *var_devtempeff_dn5_slot;
        let mut var_devtempeff_dn6: f64 = *var_devtempeff_dn6_slot;
        let mut var_devtempeff_dn7: f64 = *var_devtempeff_dn7_slot;
        let mut var_devtempeff_dn8: f64 = *var_devtempeff_dn8_slot;
        let mut var_devtempeff_dn9: f64 = *var_devtempeff_dn9_slot;
        let mut var_devtemplow0: f64 = *var_devtemplow0_slot;
        let mut var_devtemplow0_dn4: f64 = *var_devtemplow0_dn4_slot;
        let mut var_devtemplow1: f64 = *var_devtemplow1_slot;
        let mut var_devtemplow1_dn4: f64 = *var_devtemplow1_dn4_slot;
        let mut var_dimension1_i: f64 = *var_dimension1_i_slot;
        let mut var_dimension2_i: f64 = *var_dimension2_i_slot;
        let mut var_dimension3_i: f64 = *var_dimension3_i_slot;
        let mut var_dlbin_i: f64 = *var_dlbin_i_slot;
        let mut var_e2nom_i: f64 = *var_e2nom_i_slot;
        let mut var_e3nom_i: f64 = *var_e3nom_i_slot;
        let mut var_epsratio: f64 = *var_epsratio_slot;
        let mut var_epssp: f64 = *var_epssp_slot;
        let mut var_epssub: f64 = *var_epssub_slot;
        let mut var_gdpr: f64 = *var_gdpr_slot;
        let mut var_gdpr_dn0: f64 = *var_gdpr_dn0_slot;
        let mut var_gdpr_dn10: f64 = *var_gdpr_dn10_slot;
        let mut var_gdpr_dn11: f64 = *var_gdpr_dn11_slot;
        let mut var_gdpr_dn13: f64 = *var_gdpr_dn13_slot;
        let mut var_gdpr_dn14: f64 = *var_gdpr_dn14_slot;
        let mut var_gdpr_dn2: f64 = *var_gdpr_dn2_slot;
        let mut var_gdpr_dn3: f64 = *var_gdpr_dn3_slot;
        let mut var_gdpr_dn4: f64 = *var_gdpr_dn4_slot;
        let mut var_gdpr_dn5: f64 = *var_gdpr_dn5_slot;
        let mut var_gdpr_dn6: f64 = *var_gdpr_dn6_slot;
        let mut var_gdpr_dn7: f64 = *var_gdpr_dn7_slot;
        let mut var_gdpr_dn8: f64 = *var_gdpr_dn8_slot;
        let mut var_gdpr_dn9: f64 = *var_gdpr_dn9_slot;
        let mut var_gfactor: f64 = *var_gfactor_slot;
        let mut var_gmin: f64 = *var_gmin_slot;
        let mut var_gprime: f64 = *var_gprime_slot;
        let mut var_gspr: f64 = *var_gspr_slot;
        let mut var_gspr_dn0: f64 = *var_gspr_dn0_slot;
        let mut var_gspr_dn10: f64 = *var_gspr_dn10_slot;
        let mut var_gspr_dn11: f64 = *var_gspr_dn11_slot;
        let mut var_gspr_dn13: f64 = *var_gspr_dn13_slot;
        let mut var_gspr_dn14: f64 = *var_gspr_dn14_slot;
        let mut var_gspr_dn2: f64 = *var_gspr_dn2_slot;
        let mut var_gspr_dn3: f64 = *var_gspr_dn3_slot;
        let mut var_gspr_dn4: f64 = *var_gspr_dn4_slot;
        let mut var_gspr_dn5: f64 = *var_gspr_dn5_slot;
        let mut var_gspr_dn6: f64 = *var_gspr_dn6_slot;
        let mut var_gspr_dn7: f64 = *var_gspr_dn7_slot;
        let mut var_gspr_dn8: f64 = *var_gspr_dn8_slot;
        let mut var_gspr_dn9: f64 = *var_gspr_dn9_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_gvs_d: f64 = *var_gvs_d_slot;
        let mut var_gvs_d_dn0: f64 = *var_gvs_d_dn0_slot;
        let mut var_gvs_d_dn10: f64 = *var_gvs_d_dn10_slot;
        let mut var_gvs_d_dn11: f64 = *var_gvs_d_dn11_slot;
        let mut var_gvs_d_dn13: f64 = *var_gvs_d_dn13_slot;
        let mut var_gvs_d_dn14: f64 = *var_gvs_d_dn14_slot;
        let mut var_gvs_d_dn2: f64 = *var_gvs_d_dn2_slot;
        let mut var_gvs_d_dn3: f64 = *var_gvs_d_dn3_slot;
        let mut var_gvs_d_dn4: f64 = *var_gvs_d_dn4_slot;
        let mut var_gvs_d_dn5: f64 = *var_gvs_d_dn5_slot;
        let mut var_gvs_d_dn6: f64 = *var_gvs_d_dn6_slot;
        let mut var_gvs_d_dn7: f64 = *var_gvs_d_dn7_slot;
        let mut var_gvs_d_dn8: f64 = *var_gvs_d_dn8_slot;
        let mut var_gvs_d_dn9: f64 = *var_gvs_d_dn9_slot;
        let mut var_gvs_s: f64 = *var_gvs_s_slot;
        let mut var_gvs_s_dn0: f64 = *var_gvs_s_dn0_slot;
        let mut var_gvs_s_dn10: f64 = *var_gvs_s_dn10_slot;
        let mut var_gvs_s_dn11: f64 = *var_gvs_s_dn11_slot;
        let mut var_gvs_s_dn13: f64 = *var_gvs_s_dn13_slot;
        let mut var_gvs_s_dn14: f64 = *var_gvs_s_dn14_slot;
        let mut var_gvs_s_dn2: f64 = *var_gvs_s_dn2_slot;
        let mut var_gvs_s_dn3: f64 = *var_gvs_s_dn3_slot;
        let mut var_gvs_s_dn4: f64 = *var_gvs_s_dn4_slot;
        let mut var_gvs_s_dn5: f64 = *var_gvs_s_dn5_slot;
        let mut var_gvs_s_dn6: f64 = *var_gvs_s_dn6_slot;
        let mut var_gvs_s_dn7: f64 = *var_gvs_s_dn7_slot;
        let mut var_gvs_s_dn8: f64 = *var_gvs_s_dn8_slot;
        let mut var_gvs_s_dn9: f64 = *var_gvs_s_dn9_slot;
        let mut var_leff1: f64 = *var_leff1_slot;
        let mut var_leff_1: f64 = *var_leff_1_slot;
        let mut var_leff_1_dn0: f64 = *var_leff_1_dn0_slot;
        let mut var_leff_1_dn10: f64 = *var_leff_1_dn10_slot;
        let mut var_leff_1_dn11: f64 = *var_leff_1_dn11_slot;
        let mut var_leff_1_dn13: f64 = *var_leff_1_dn13_slot;
        let mut var_leff_1_dn14: f64 = *var_leff_1_dn14_slot;
        let mut var_leff_1_dn2: f64 = *var_leff_1_dn2_slot;
        let mut var_leff_1_dn3: f64 = *var_leff_1_dn3_slot;
        let mut var_leff_1_dn4: f64 = *var_leff_1_dn4_slot;
        let mut var_leff_1_dn5: f64 = *var_leff_1_dn5_slot;
        let mut var_leff_1_dn6: f64 = *var_leff_1_dn6_slot;
        let mut var_leff_1_dn7: f64 = *var_leff_1_dn7_slot;
        let mut var_leff_1_dn8: f64 = *var_leff_1_dn8_slot;
        let mut var_leff_1_dn9: f64 = *var_leff_1_dn9_slot;
        let mut var_lg: f64 = *var_lg_slot;
        let mut var_lint_i: f64 = *var_lint_i_slot;
        let mut var_md: f64 = *var_md_slot;
        let mut var_mdprime: f64 = *var_mdprime_slot;
        let mut var_mfq1nom_i: f64 = *var_mfq1nom_i_slot;
        let mut var_mfq2nom_i: f64 = *var_mfq2nom_i_slot;
        let mut var_mfq3nom_i: f64 = *var_mfq3nom_i_slot;
        let mut var_mx: f64 = *var_mx_slot;
        let mut var_mxprime: f64 = *var_mxprime_slot;
        let mut var_nc3d: f64 = *var_nc3d_slot;
        let mut var_ncq: f64 = *var_ncq_slot;
        let mut var_niln: f64 = *var_niln_slot;
        let mut var_niln_dn0: f64 = *var_niln_dn0_slot;
        let mut var_niln_dn10: f64 = *var_niln_dn10_slot;
        let mut var_niln_dn11: f64 = *var_niln_dn11_slot;
        let mut var_niln_dn13: f64 = *var_niln_dn13_slot;
        let mut var_niln_dn14: f64 = *var_niln_dn14_slot;
        let mut var_niln_dn2: f64 = *var_niln_dn2_slot;
        let mut var_niln_dn3: f64 = *var_niln_dn3_slot;
        let mut var_niln_dn4: f64 = *var_niln_dn4_slot;
        let mut var_niln_dn5: f64 = *var_niln_dn5_slot;
        let mut var_niln_dn6: f64 = *var_niln_dn6_slot;
        let mut var_niln_dn7: f64 = *var_niln_dn7_slot;
        let mut var_niln_dn8: f64 = *var_niln_dn8_slot;
        let mut var_niln_dn9: f64 = *var_niln_dn9_slot;
        let mut var_p1: f64 = *var_p1_slot;
        let mut var_p2: f64 = *var_p2_slot;
        let mut var_p3: f64 = *var_p3_slot;
        let mut var_qe2: f64 = *var_qe2_slot;
        let mut var_qe2_dn0: f64 = *var_qe2_dn0_slot;
        let mut var_qe2_dn10: f64 = *var_qe2_dn10_slot;
        let mut var_qe2_dn11: f64 = *var_qe2_dn11_slot;
        let mut var_qe2_dn13: f64 = *var_qe2_dn13_slot;
        let mut var_qe2_dn14: f64 = *var_qe2_dn14_slot;
        let mut var_qe2_dn2: f64 = *var_qe2_dn2_slot;
        let mut var_qe2_dn3: f64 = *var_qe2_dn3_slot;
        let mut var_qe2_dn4: f64 = *var_qe2_dn4_slot;
        let mut var_qe2_dn5: f64 = *var_qe2_dn5_slot;
        let mut var_qe2_dn6: f64 = *var_qe2_dn6_slot;
        let mut var_qe2_dn7: f64 = *var_qe2_dn7_slot;
        let mut var_qe2_dn8: f64 = *var_qe2_dn8_slot;
        let mut var_qe2_dn9: f64 = *var_qe2_dn9_slot;
        let mut var_qe3: f64 = *var_qe3_slot;
        let mut var_qnd10: f64 = *var_qnd10_slot;
        let mut var_qnd20: f64 = *var_qnd20_slot;
        let mut var_qnd30: f64 = *var_qnd30_slot;
        let mut var_qndd2: f64 = *var_qndd2_slot;
        let mut var_qndd2_dn0: f64 = *var_qndd2_dn0_slot;
        let mut var_qndd2_dn10: f64 = *var_qndd2_dn10_slot;
        let mut var_qndd2_dn11: f64 = *var_qndd2_dn11_slot;
        let mut var_qndd2_dn13: f64 = *var_qndd2_dn13_slot;
        let mut var_qndd2_dn14: f64 = *var_qndd2_dn14_slot;
        let mut var_qndd2_dn2: f64 = *var_qndd2_dn2_slot;
        let mut var_qndd2_dn3: f64 = *var_qndd2_dn3_slot;
        let mut var_qndd2_dn4: f64 = *var_qndd2_dn4_slot;
        let mut var_qndd2_dn5: f64 = *var_qndd2_dn5_slot;
        let mut var_qndd2_dn6: f64 = *var_qndd2_dn6_slot;
        let mut var_qndd2_dn7: f64 = *var_qndd2_dn7_slot;
        let mut var_qndd2_dn8: f64 = *var_qndd2_dn8_slot;
        let mut var_qndd2_dn9: f64 = *var_qndd2_dn9_slot;
        let mut var_qndd3: f64 = *var_qndd3_slot;
        let mut var_qndd3_dn0: f64 = *var_qndd3_dn0_slot;
        let mut var_qndd3_dn10: f64 = *var_qndd3_dn10_slot;
        let mut var_qndd3_dn11: f64 = *var_qndd3_dn11_slot;
        let mut var_qndd3_dn13: f64 = *var_qndd3_dn13_slot;
        let mut var_qndd3_dn14: f64 = *var_qndd3_dn14_slot;
        let mut var_qndd3_dn2: f64 = *var_qndd3_dn2_slot;
        let mut var_qndd3_dn3: f64 = *var_qndd3_dn3_slot;
        let mut var_qndd3_dn4: f64 = *var_qndd3_dn4_slot;
        let mut var_qndd3_dn5: f64 = *var_qndd3_dn5_slot;
        let mut var_qndd3_dn6: f64 = *var_qndd3_dn6_slot;
        let mut var_qndd3_dn7: f64 = *var_qndd3_dn7_slot;
        let mut var_qndd3_dn8: f64 = *var_qndd3_dn8_slot;
        let mut var_qndd3_dn9: f64 = *var_qndd3_dn9_slot;
        let mut var_qsref_i: f64 = *var_qsref_i_slot;
        let mut var_rdstempvs: f64 = *var_rdstempvs_slot;
        let mut var_rdstempvs_dn4: f64 = *var_rdstempvs_dn4_slot;
        let mut var_rvs_d: f64 = *var_rvs_d_slot;
        let mut var_rvs_d_dn0: f64 = *var_rvs_d_dn0_slot;
        let mut var_rvs_d_dn10: f64 = *var_rvs_d_dn10_slot;
        let mut var_rvs_d_dn11: f64 = *var_rvs_d_dn11_slot;
        let mut var_rvs_d_dn13: f64 = *var_rvs_d_dn13_slot;
        let mut var_rvs_d_dn14: f64 = *var_rvs_d_dn14_slot;
        let mut var_rvs_d_dn2: f64 = *var_rvs_d_dn2_slot;
        let mut var_rvs_d_dn3: f64 = *var_rvs_d_dn3_slot;
        let mut var_rvs_d_dn4: f64 = *var_rvs_d_dn4_slot;
        let mut var_rvs_d_dn5: f64 = *var_rvs_d_dn5_slot;
        let mut var_rvs_d_dn6: f64 = *var_rvs_d_dn6_slot;
        let mut var_rvs_d_dn7: f64 = *var_rvs_d_dn7_slot;
        let mut var_rvs_d_dn8: f64 = *var_rvs_d_dn8_slot;
        let mut var_rvs_d_dn9: f64 = *var_rvs_d_dn9_slot;
        let mut var_rvs_s: f64 = *var_rvs_s_slot;
        let mut var_rvs_s_dn0: f64 = *var_rvs_s_dn0_slot;
        let mut var_rvs_s_dn10: f64 = *var_rvs_s_dn10_slot;
        let mut var_rvs_s_dn11: f64 = *var_rvs_s_dn11_slot;
        let mut var_rvs_s_dn13: f64 = *var_rvs_s_dn13_slot;
        let mut var_rvs_s_dn14: f64 = *var_rvs_s_dn14_slot;
        let mut var_rvs_s_dn2: f64 = *var_rvs_s_dn2_slot;
        let mut var_rvs_s_dn3: f64 = *var_rvs_s_dn3_slot;
        let mut var_rvs_s_dn4: f64 = *var_rvs_s_dn4_slot;
        let mut var_rvs_s_dn5: f64 = *var_rvs_s_dn5_slot;
        let mut var_rvs_s_dn6: f64 = *var_rvs_s_dn6_slot;
        let mut var_rvs_s_dn7: f64 = *var_rvs_s_dn7_slot;
        let mut var_rvs_s_dn8: f64 = *var_rvs_s_dn8_slot;
        let mut var_rvs_s_dn9: f64 = *var_rvs_s_dn9_slot;
        let mut var_ssp1_i: f64 = *var_ssp1_i_slot;
        let mut var_ssp2_i: f64 = *var_ssp2_i_slot;
        let mut var_ssp3_i: f64 = *var_ssp3_i_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_ua_th: f64 = *var_ua_th_slot;
        let mut var_ua_th_dn0: f64 = *var_ua_th_dn0_slot;
        let mut var_ua_th_dn10: f64 = *var_ua_th_dn10_slot;
        let mut var_ua_th_dn11: f64 = *var_ua_th_dn11_slot;
        let mut var_ua_th_dn13: f64 = *var_ua_th_dn13_slot;
        let mut var_ua_th_dn14: f64 = *var_ua_th_dn14_slot;
        let mut var_ua_th_dn2: f64 = *var_ua_th_dn2_slot;
        let mut var_ua_th_dn3: f64 = *var_ua_th_dn3_slot;
        let mut var_ua_th_dn4: f64 = *var_ua_th_dn4_slot;
        let mut var_ua_th_dn5: f64 = *var_ua_th_dn5_slot;
        let mut var_ua_th_dn6: f64 = *var_ua_th_dn6_slot;
        let mut var_ua_th_dn7: f64 = *var_ua_th_dn7_slot;
        let mut var_ua_th_dn8: f64 = *var_ua_th_dn8_slot;
        let mut var_ua_th_dn9: f64 = *var_ua_th_dn9_slot;
        let mut var_ua_tl: f64 = *var_ua_tl_slot;
        let mut var_ua_tl_dn0: f64 = *var_ua_tl_dn0_slot;
        let mut var_ua_tl_dn10: f64 = *var_ua_tl_dn10_slot;
        let mut var_ua_tl_dn11: f64 = *var_ua_tl_dn11_slot;
        let mut var_ua_tl_dn13: f64 = *var_ua_tl_dn13_slot;
        let mut var_ua_tl_dn14: f64 = *var_ua_tl_dn14_slot;
        let mut var_ua_tl_dn2: f64 = *var_ua_tl_dn2_slot;
        let mut var_ua_tl_dn3: f64 = *var_ua_tl_dn3_slot;
        let mut var_ua_tl_dn4: f64 = *var_ua_tl_dn4_slot;
        let mut var_ua_tl_dn5: f64 = *var_ua_tl_dn5_slot;
        let mut var_ua_tl_dn6: f64 = *var_ua_tl_dn6_slot;
        let mut var_ua_tl_dn7: f64 = *var_ua_tl_dn7_slot;
        let mut var_ua_tl_dn8: f64 = *var_ua_tl_dn8_slot;
        let mut var_ua_tl_dn9: f64 = *var_ua_tl_dn9_slot;
        let mut var_uar_th: f64 = *var_uar_th_slot;
        let mut var_uar_th_dn0: f64 = *var_uar_th_dn0_slot;
        let mut var_uar_th_dn10: f64 = *var_uar_th_dn10_slot;
        let mut var_uar_th_dn11: f64 = *var_uar_th_dn11_slot;
        let mut var_uar_th_dn13: f64 = *var_uar_th_dn13_slot;
        let mut var_uar_th_dn14: f64 = *var_uar_th_dn14_slot;
        let mut var_uar_th_dn2: f64 = *var_uar_th_dn2_slot;
        let mut var_uar_th_dn3: f64 = *var_uar_th_dn3_slot;
        let mut var_uar_th_dn4: f64 = *var_uar_th_dn4_slot;
        let mut var_uar_th_dn5: f64 = *var_uar_th_dn5_slot;
        let mut var_uar_th_dn6: f64 = *var_uar_th_dn6_slot;
        let mut var_uar_th_dn7: f64 = *var_uar_th_dn7_slot;
        let mut var_uar_th_dn8: f64 = *var_uar_th_dn8_slot;
        let mut var_uar_th_dn9: f64 = *var_uar_th_dn9_slot;
        let mut var_uar_tl: f64 = *var_uar_tl_slot;
        let mut var_uar_tl_dn0: f64 = *var_uar_tl_dn0_slot;
        let mut var_uar_tl_dn10: f64 = *var_uar_tl_dn10_slot;
        let mut var_uar_tl_dn11: f64 = *var_uar_tl_dn11_slot;
        let mut var_uar_tl_dn13: f64 = *var_uar_tl_dn13_slot;
        let mut var_uar_tl_dn14: f64 = *var_uar_tl_dn14_slot;
        let mut var_uar_tl_dn2: f64 = *var_uar_tl_dn2_slot;
        let mut var_uar_tl_dn3: f64 = *var_uar_tl_dn3_slot;
        let mut var_uar_tl_dn4: f64 = *var_uar_tl_dn4_slot;
        let mut var_uar_tl_dn5: f64 = *var_uar_tl_dn5_slot;
        let mut var_uar_tl_dn6: f64 = *var_uar_tl_dn6_slot;
        let mut var_uar_tl_dn7: f64 = *var_uar_tl_dn7_slot;
        let mut var_uar_tl_dn8: f64 = *var_uar_tl_dn8_slot;
        let mut var_uar_tl_dn9: f64 = *var_uar_tl_dn9_slot;
        let mut var_udd_t: f64 = *var_udd_t_slot;
        let mut var_udd_t_dn4: f64 = *var_udd_t_dn4_slot;
        let mut var_uddeff_t: f64 = *var_uddeff_t_slot;
        let mut var_uddeff_t_dn4: f64 = *var_uddeff_t_dn4_slot;
        let mut var_uds_t: f64 = *var_uds_t_slot;
        let mut var_uds_t_dn4: f64 = *var_uds_t_dn4_slot;
        let mut var_udseff_t: f64 = *var_udseff_t_slot;
        let mut var_udseff_t_dn4: f64 = *var_udseff_t_dn4_slot;
        let mut var_vtmeff: f64 = *var_vtmeff_slot;
        let mut var_vtmeff_dn0: f64 = *var_vtmeff_dn0_slot;
        let mut var_vtmeff_dn10: f64 = *var_vtmeff_dn10_slot;
        let mut var_vtmeff_dn11: f64 = *var_vtmeff_dn11_slot;
        let mut var_vtmeff_dn13: f64 = *var_vtmeff_dn13_slot;
        let mut var_vtmeff_dn14: f64 = *var_vtmeff_dn14_slot;
        let mut var_vtmeff_dn2: f64 = *var_vtmeff_dn2_slot;
        let mut var_vtmeff_dn3: f64 = *var_vtmeff_dn3_slot;
        let mut var_vtmeff_dn4: f64 = *var_vtmeff_dn4_slot;
        let mut var_vtmeff_dn5: f64 = *var_vtmeff_dn5_slot;
        let mut var_vtmeff_dn6: f64 = *var_vtmeff_dn6_slot;
        let mut var_vtmeff_dn7: f64 = *var_vtmeff_dn7_slot;
        let mut var_vtmeff_dn8: f64 = *var_vtmeff_dn8_slot;
        let mut var_vtmeff_dn9: f64 = *var_vtmeff_dn9_slot;
        let mut var_wh: f64 = *var_wh_slot;
        let mut var_wh_dn4: f64 = *var_wh_dn4_slot;
        let mut var_wl: f64 = *var_wl_slot;
        let mut var_wl_dn4: f64 = *var_wl_dn4_slot;
        let mut var_xl_i: f64 = *var_xl_i_slot;

        var_qndd2 = 0.0;
        var_qndd2_dn0 = 0.0;
        var_qndd2_dn2 = 0.0;
        var_qndd2_dn3 = 0.0;
        var_qndd2_dn4 = 0.0;
        var_qndd2_dn5 = 0.0;
        var_qndd2_dn6 = 0.0;
        var_qndd2_dn7 = 0.0;
        var_qndd2_dn8 = 0.0;
        var_qndd2_dn9 = 0.0;
        var_qndd2_dn10 = 0.0;
        var_qndd2_dn11 = 0.0;
        var_qndd2_dn13 = 0.0;
        var_qndd2_dn14 = 0.0;

        var_qndd3 = 0.0;
        var_qndd3_dn0 = 0.0;
        var_qndd3_dn2 = 0.0;
        var_qndd3_dn3 = 0.0;
        var_qndd3_dn4 = 0.0;
        var_qndd3_dn5 = 0.0;
        var_qndd3_dn6 = 0.0;
        var_qndd3_dn7 = 0.0;
        var_qndd3_dn8 = 0.0;
        var_qndd3_dn9 = 0.0;
        var_qndd3_dn10 = 0.0;
        var_qndd3_dn11 = 0.0;
        var_qndd3_dn13 = 0.0;
        var_qndd3_dn14 = 0.0;

        var_nc3d = 1.0;

        var_d1 = 0.0;

        var_d2 = 0.0;

        var_d3 = 0.0;

        var_p1 = 0.0;

        var_p2 = 0.0;

        var_p3 = 0.0;

        var_ncq = 0.0;

        var_qe2 = 0.0;
        var_qe2_dn0 = 0.0;
        var_qe2_dn2 = 0.0;
        var_qe2_dn3 = 0.0;
        var_qe2_dn4 = 0.0;
        var_qe2_dn5 = 0.0;
        var_qe2_dn6 = 0.0;
        var_qe2_dn7 = 0.0;
        var_qe2_dn8 = 0.0;
        var_qe2_dn9 = 0.0;
        var_qe2_dn10 = 0.0;
        var_qe2_dn11 = 0.0;
        var_qe2_dn13 = 0.0;
        var_qe2_dn14 = 0.0;

        var_qe3 = 0.0;

        var_qnd10 = 0.0;

        var_qnd20 = 0.0;

        var_qnd30 = 0.0;

        var_dimension1_i = 0.0;

        var_dimension2_i = 0.0;

        var_dimension3_i = 0.0;

        var_ssp1_i = 0.0;

        var_ssp2_i = 0.0;

        var_ssp3_i = 0.0;

        var_e2nom_i = 0.0;

        var_e3nom_i = 0.0;

        var_mfq1nom_i = 0.0;

        var_mfq2nom_i = 0.0;

        var_mfq3nom_i = 0.0;

        var_devtemplow0 = 0.0;
        var_devtemplow0_dn4 = 0.0;

        var_devtemplow1 = 0.0;
        var_devtemplow1_dn4 = 0.0;

        var_devtempeff = 0.0;
        var_devtempeff_dn0 = 0.0;
        var_devtempeff_dn2 = 0.0;
        var_devtempeff_dn3 = 0.0;
        var_devtempeff_dn4 = 0.0;
        var_devtempeff_dn5 = 0.0;
        var_devtempeff_dn6 = 0.0;
        var_devtempeff_dn7 = 0.0;
        var_devtempeff_dn8 = 0.0;
        var_devtempeff_dn9 = 0.0;
        var_devtempeff_dn10 = 0.0;
        var_devtempeff_dn11 = 0.0;
        var_devtempeff_dn13 = 0.0;
        var_devtempeff_dn14 = 0.0;

        var_devtemp1 = 0.0;
        var_devtemp1_dn4 = 0.0;

        var_deltemp1 = 0.0;
        var_deltemp1_dn4 = 0.0;

        var_deltratio1 = 0.0;
        var_deltratio1_dn4 = 0.0;

        var_vtmeff = 0.0;
        var_vtmeff_dn0 = 0.0;
        var_vtmeff_dn2 = 0.0;
        var_vtmeff_dn3 = 0.0;
        var_vtmeff_dn4 = 0.0;
        var_vtmeff_dn5 = 0.0;
        var_vtmeff_dn6 = 0.0;
        var_vtmeff_dn7 = 0.0;
        var_vtmeff_dn8 = 0.0;
        var_vtmeff_dn9 = 0.0;
        var_vtmeff_dn10 = 0.0;
        var_vtmeff_dn11 = 0.0;
        var_vtmeff_dn13 = 0.0;
        var_vtmeff_dn14 = 0.0;

        var_niln = 0.0;
        var_niln_dn0 = 0.0;
        var_niln_dn2 = 0.0;
        var_niln_dn3 = 0.0;
        var_niln_dn4 = 0.0;
        var_niln_dn5 = 0.0;
        var_niln_dn6 = 0.0;
        var_niln_dn7 = 0.0;
        var_niln_dn8 = 0.0;
        var_niln_dn9 = 0.0;
        var_niln_dn10 = 0.0;
        var_niln_dn11 = 0.0;
        var_niln_dn13 = 0.0;
        var_niln_dn14 = 0.0;

        var_uds_t = 0.0;
        var_uds_t_dn4 = 0.0;

        var_udd_t = 0.0;
        var_udd_t_dn4 = 0.0;

        var_ua_tl = 0.0;
        var_ua_tl_dn0 = 0.0;
        var_ua_tl_dn2 = 0.0;
        var_ua_tl_dn3 = 0.0;
        var_ua_tl_dn4 = 0.0;
        var_ua_tl_dn5 = 0.0;
        var_ua_tl_dn6 = 0.0;
        var_ua_tl_dn7 = 0.0;
        var_ua_tl_dn8 = 0.0;
        var_ua_tl_dn9 = 0.0;
        var_ua_tl_dn10 = 0.0;
        var_ua_tl_dn11 = 0.0;
        var_ua_tl_dn13 = 0.0;
        var_ua_tl_dn14 = 0.0;

        var_ua_th = 0.0;
        var_ua_th_dn0 = 0.0;
        var_ua_th_dn2 = 0.0;
        var_ua_th_dn3 = 0.0;
        var_ua_th_dn4 = 0.0;
        var_ua_th_dn5 = 0.0;
        var_ua_th_dn6 = 0.0;
        var_ua_th_dn7 = 0.0;
        var_ua_th_dn8 = 0.0;
        var_ua_th_dn9 = 0.0;
        var_ua_th_dn10 = 0.0;
        var_ua_th_dn11 = 0.0;
        var_ua_th_dn13 = 0.0;
        var_ua_th_dn14 = 0.0;

        var_uar_tl = 0.0;
        var_uar_tl_dn0 = 0.0;
        var_uar_tl_dn2 = 0.0;
        var_uar_tl_dn3 = 0.0;
        var_uar_tl_dn4 = 0.0;
        var_uar_tl_dn5 = 0.0;
        var_uar_tl_dn6 = 0.0;
        var_uar_tl_dn7 = 0.0;
        var_uar_tl_dn8 = 0.0;
        var_uar_tl_dn9 = 0.0;
        var_uar_tl_dn10 = 0.0;
        var_uar_tl_dn11 = 0.0;
        var_uar_tl_dn13 = 0.0;
        var_uar_tl_dn14 = 0.0;

        var_uar_th = 0.0;
        var_uar_th_dn0 = 0.0;
        var_uar_th_dn2 = 0.0;
        var_uar_th_dn3 = 0.0;
        var_uar_th_dn4 = 0.0;
        var_uar_th_dn5 = 0.0;
        var_uar_th_dn6 = 0.0;
        var_uar_th_dn7 = 0.0;
        var_uar_th_dn8 = 0.0;
        var_uar_th_dn9 = 0.0;
        var_uar_th_dn10 = 0.0;
        var_uar_th_dn11 = 0.0;
        var_uar_th_dn13 = 0.0;
        var_uar_th_dn14 = 0.0;

        var_wl = 0.0;
        var_wl_dn4 = 0.0;

        var_wh = 0.0;
        var_wh_dn4 = 0.0;

        var_uddeff_t = 0.0;
        var_uddeff_t_dn4 = 0.0;

        var_udseff_t = 0.0;
        var_udseff_t_dn4 = 0.0;

        var_rvs_d = 0.0;
        var_rvs_d_dn0 = 0.0;
        var_rvs_d_dn2 = 0.0;
        var_rvs_d_dn3 = 0.0;
        var_rvs_d_dn4 = 0.0;
        var_rvs_d_dn5 = 0.0;
        var_rvs_d_dn6 = 0.0;
        var_rvs_d_dn7 = 0.0;
        var_rvs_d_dn8 = 0.0;
        var_rvs_d_dn9 = 0.0;
        var_rvs_d_dn10 = 0.0;
        var_rvs_d_dn11 = 0.0;
        var_rvs_d_dn13 = 0.0;
        var_rvs_d_dn14 = 0.0;

        var_rvs_s = 0.0;
        var_rvs_s_dn0 = 0.0;
        var_rvs_s_dn2 = 0.0;
        var_rvs_s_dn3 = 0.0;
        var_rvs_s_dn4 = 0.0;
        var_rvs_s_dn5 = 0.0;
        var_rvs_s_dn6 = 0.0;
        var_rvs_s_dn7 = 0.0;
        var_rvs_s_dn8 = 0.0;
        var_rvs_s_dn9 = 0.0;
        var_rvs_s_dn10 = 0.0;
        var_rvs_s_dn11 = 0.0;
        var_rvs_s_dn13 = 0.0;
        var_rvs_s_dn14 = 0.0;

        var_rdstempvs = 0.0;
        var_rdstempvs_dn4 = 0.0;

        var_gvs_s = 0.0;
        var_gvs_s_dn0 = 0.0;
        var_gvs_s_dn2 = 0.0;
        var_gvs_s_dn3 = 0.0;
        var_gvs_s_dn4 = 0.0;
        var_gvs_s_dn5 = 0.0;
        var_gvs_s_dn6 = 0.0;
        var_gvs_s_dn7 = 0.0;
        var_gvs_s_dn8 = 0.0;
        var_gvs_s_dn9 = 0.0;
        var_gvs_s_dn10 = 0.0;
        var_gvs_s_dn11 = 0.0;
        var_gvs_s_dn13 = 0.0;
        var_gvs_s_dn14 = 0.0;

        var_gvs_d = 0.0;
        var_gvs_d_dn0 = 0.0;
        var_gvs_d_dn2 = 0.0;
        var_gvs_d_dn3 = 0.0;
        var_gvs_d_dn4 = 0.0;
        var_gvs_d_dn5 = 0.0;
        var_gvs_d_dn6 = 0.0;
        var_gvs_d_dn7 = 0.0;
        var_gvs_d_dn8 = 0.0;
        var_gvs_d_dn9 = 0.0;
        var_gvs_d_dn10 = 0.0;
        var_gvs_d_dn11 = 0.0;
        var_gvs_d_dn13 = 0.0;
        var_gvs_d_dn14 = 0.0;

        var_deltaprsd_v = 0.0;

        var_gspr = 0.0;
        var_gspr_dn0 = 0.0;
        var_gspr_dn2 = 0.0;
        var_gspr_dn3 = 0.0;
        var_gspr_dn4 = 0.0;
        var_gspr_dn5 = 0.0;
        var_gspr_dn6 = 0.0;
        var_gspr_dn7 = 0.0;
        var_gspr_dn8 = 0.0;
        var_gspr_dn9 = 0.0;
        var_gspr_dn10 = 0.0;
        var_gspr_dn11 = 0.0;
        var_gspr_dn13 = 0.0;
        var_gspr_dn14 = 0.0;

        var_qsref_i = 0.0;

        var_gdpr = 0.0;
        var_gdpr_dn0 = 0.0;
        var_gdpr_dn2 = 0.0;
        var_gdpr_dn3 = 0.0;
        var_gdpr_dn4 = 0.0;
        var_gdpr_dn5 = 0.0;
        var_gdpr_dn6 = 0.0;
        var_gdpr_dn7 = 0.0;
        var_gdpr_dn8 = 0.0;
        var_gdpr_dn9 = 0.0;
        var_gdpr_dn10 = 0.0;
        var_gdpr_dn11 = 0.0;
        var_gdpr_dn13 = 0.0;
        var_gdpr_dn14 = 0.0;

        let assign2570_e3169: f64 = 0.0;
        var_gmin = assign2570_e3169;

        let assign2600_e3187: f64 = if p.p60 == 1.0 { 1.0 } else { 0.0 };
        var_guard3 = assign2600_e3187;

        let (assign2610_e3191,) = {
    if (var_guard3 != 0.0) {
        (1.0,)
    } else {
        (var_devsign,)
    }
};
        var_devsign = assign2610_e3191;

        let (assign2620_e3197,) = {
    if (var_guard3 == 0.0) {
        let assign2620_e3195: f64 = (-1.0);
        (assign2620_e3195,)
    } else {
        (var_devsign,)
    }
};
        var_devsign = assign2620_e3197;

        let assign2630_e3200: f64 = (p.p103 * 8.8542e-12);
        var_epssub = assign2630_e3200;

        let assign2640_e3203: f64 = (p.p1088 * 8.8542e-12);
        var_epssp = assign2640_e3203;

        let assign2650_e3206: f64 = (p.p102 * 8.8542e-12);
        let assign2650_e3208: f64 = (assign2650_e3206 / p.p91);
        var_cbox = assign2650_e3208;

        let assign2660_e3211: f64 = (p.p103 / p.p102);
        var_epsratio = assign2660_e3211;

        let assign2670_e3214: f64 = (0.916 * 9.11e-31);
        var_mx = assign2670_e3214;

        let assign2680_e3217: f64 = (0.19 * 9.11e-31);
        var_mxprime = assign2680_e3217;

        let assign2690_e3220: f64 = (0.19 * 9.11e-31);
        var_md = assign2690_e3220;

        let assign2700_e3223: f64 = (0.417 * 9.11e-31);
        var_mdprime = assign2700_e3223;

        var_gprime = 4.0;

        var_gfactor = 2.0;

        let assign2730_e3229: f64 = (1e-6 * p.p110);
        let assign2730_e3231: f64 = (assign2730_e3229 / p.p0);
        let assign2730_e3232: f64 = (p.p109 + assign2730_e3231);
        let assign2730_e3235: f64 = (p.p111 / p.p5);
        let assign2730_e3236: f64 = (assign2730_e3232 + assign2730_e3235);
        let assign2730_e3239: f64 = (p.p112 * 1e-6);
        let assign2730_e3242: f64 = (p.p0 * p.p5);
        let assign2730_e3243: f64 = (assign2730_e3239 / assign2730_e3242);
        let assign2730_e3244: f64 = (assign2730_e3236 + assign2730_e3243);
        var_xl_i = assign2730_e3244;

        let assign2740_e3248: f64 = (1e-6 * p.p118);
        let assign2740_e3250: f64 = (assign2740_e3248 / p.p0);
        let assign2740_e3251: f64 = (p.p117 + assign2740_e3250);
        let assign2740_e3254: f64 = (p.p119 / p.p5);
        let assign2740_e3255: f64 = (assign2740_e3251 + assign2740_e3254);
        let assign2740_e3258: f64 = (p.p120 * 1e-6);
        let assign2740_e3261: f64 = (p.p0 * p.p5);
        let assign2740_e3262: f64 = (assign2740_e3258 / assign2740_e3261);
        let assign2740_e3263: f64 = (assign2740_e3255 + assign2740_e3262);
        var_dlbin_i = assign2740_e3263;

        let assign2750_e3267: f64 = (1e-6 * p.p114);
        let assign2750_e3269: f64 = (assign2750_e3267 / p.p0);
        let assign2750_e3270: f64 = (p.p113 + assign2750_e3269);
        let assign2750_e3273: f64 = (p.p115 / p.p5);
        let assign2750_e3274: f64 = (assign2750_e3270 + assign2750_e3273);
        let assign2750_e3277: f64 = (p.p116 * 1e-6);
        let assign2750_e3280: f64 = (p.p0 * p.p5);
        let assign2750_e3281: f64 = (assign2750_e3277 / assign2750_e3280);
        let assign2750_e3282: f64 = (assign2750_e3274 + assign2750_e3281);
        var_lint_i = assign2750_e3282;

        let assign2760_e3285: f64 = (p.p0 + var_xl_i);
        var_lg = assign2760_e3285;

        let assign2770_e3288: f64 = if var_lg <= 0.0 { 1.0 } else { 0.0 };
        var_guard4 = assign2770_e3288;

        let (assign2780_e3292,) = {
    if (var_guard4 != 0.0) {
        (p.p0,)
    } else {
        (var_lg,)
    }
};
        var_lg = assign2780_e3292;

        let assign2790_e3295: f64 = (-p.p84);
        let assign2790_e3296: f64 = (var_lg).powf(assign2790_e3295);
        var_t0 = assign2790_e3296;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;

        let assign2800_e3300: f64 = (p.p83 * var_t0);
        let assign2800_e3301: f64 = (var_lint_i + assign2800_e3300);
        var_deltal = assign2800_e3301;
        var_deltal_dn0 = (p.p83 * var_t0_dn0);
        var_deltal_dn2 = (p.p83 * var_t0_dn2);
        var_deltal_dn3 = (p.p83 * var_t0_dn3);
        var_deltal_dn4 = (p.p83 * var_t0_dn4);
        var_deltal_dn5 = (p.p83 * var_t0_dn5);
        var_deltal_dn6 = (p.p83 * var_t0_dn6);
        var_deltal_dn7 = (p.p83 * var_t0_dn7);
        var_deltal_dn8 = (p.p83 * var_t0_dn8);
        var_deltal_dn9 = (p.p83 * var_t0_dn9);
        var_deltal_dn10 = (p.p83 * var_t0_dn10);
        var_deltal_dn11 = (p.p83 * var_t0_dn11);
        var_deltal_dn13 = (p.p83 * var_t0_dn13);
        var_deltal_dn14 = (p.p83 * var_t0_dn14);

        let assign2810_e3306: f64 = (var_lg + var_dlbin_i);
        let assign2810_e3308: f64 = (-p.p84);
        let assign2810_e3309: f64 = (assign2810_e3306).powf(assign2810_e3308);
        let assign2810_e3310: f64 = (p.p83 * assign2810_e3309);
        let assign2810_e3311: f64 = (var_lint_i + assign2810_e3310);
        var_deltal1 = assign2810_e3311;

        let assign2820_e3315: f64 = (p.p88 * var_t0);
        let assign2820_e3316: f64 = (p.p85 + assign2820_e3315);
        var_deltalcv = assign2820_e3316;
        var_deltalcv_dn0 = (p.p88 * var_t0_dn0);
        var_deltalcv_dn2 = (p.p88 * var_t0_dn2);
        var_deltalcv_dn3 = (p.p88 * var_t0_dn3);
        var_deltalcv_dn4 = (p.p88 * var_t0_dn4);
        var_deltalcv_dn5 = (p.p88 * var_t0_dn5);
        var_deltalcv_dn6 = (p.p88 * var_t0_dn6);
        var_deltalcv_dn7 = (p.p88 * var_t0_dn7);
        var_deltalcv_dn8 = (p.p88 * var_t0_dn8);
        var_deltalcv_dn9 = (p.p88 * var_t0_dn9);
        var_deltalcv_dn10 = (p.p88 * var_t0_dn10);
        var_deltalcv_dn11 = (p.p88 * var_t0_dn11);
        var_deltalcv_dn13 = (p.p88 * var_t0_dn13);
        var_deltalcv_dn14 = (p.p88 * var_t0_dn14);

        let assign2830_e3320: f64 = (2.0 * var_deltal);
        let assign2830_e3321: f64 = (var_lg - assign2830_e3320);
        var_leff_1 = assign2830_e3321;
        var_leff_1_dn0 = (-(2.0 * var_deltal_dn0));
        var_leff_1_dn2 = (-(2.0 * var_deltal_dn2));
        var_leff_1_dn3 = (-(2.0 * var_deltal_dn3));
        var_leff_1_dn4 = (-(2.0 * var_deltal_dn4));
        var_leff_1_dn5 = (-(2.0 * var_deltal_dn5));
        var_leff_1_dn6 = (-(2.0 * var_deltal_dn6));
        var_leff_1_dn7 = (-(2.0 * var_deltal_dn7));
        var_leff_1_dn8 = (-(2.0 * var_deltal_dn8));
        var_leff_1_dn9 = (-(2.0 * var_deltal_dn9));
        var_leff_1_dn10 = (-(2.0 * var_deltal_dn10));
        var_leff_1_dn11 = (-(2.0 * var_deltal_dn11));
        var_leff_1_dn13 = (-(2.0 * var_deltal_dn13));
        var_leff_1_dn14 = (-(2.0 * var_deltal_dn14));

        let assign2840_e3324: f64 = (var_lg + var_dlbin_i);
        let assign2840_e3327: f64 = (2.0 * var_deltal1);
        let assign2840_e3328: f64 = (assign2840_e3324 - assign2840_e3327);
        var_leff1 = assign2840_e3328;

        *var_cbox_slot = var_cbox;
        *var_d1_slot = var_d1;
        *var_d2_slot = var_d2;
        *var_d3_slot = var_d3;
        *var_deltal_slot = var_deltal;
        *var_deltal1_slot = var_deltal1;
        *var_deltal_dn0_slot = var_deltal_dn0;
        *var_deltal_dn10_slot = var_deltal_dn10;
        *var_deltal_dn11_slot = var_deltal_dn11;
        *var_deltal_dn13_slot = var_deltal_dn13;
        *var_deltal_dn14_slot = var_deltal_dn14;
        *var_deltal_dn2_slot = var_deltal_dn2;
        *var_deltal_dn3_slot = var_deltal_dn3;
        *var_deltal_dn4_slot = var_deltal_dn4;
        *var_deltal_dn5_slot = var_deltal_dn5;
        *var_deltal_dn6_slot = var_deltal_dn6;
        *var_deltal_dn7_slot = var_deltal_dn7;
        *var_deltal_dn8_slot = var_deltal_dn8;
        *var_deltal_dn9_slot = var_deltal_dn9;
        *var_deltalcv_slot = var_deltalcv;
        *var_deltalcv_dn0_slot = var_deltalcv_dn0;
        *var_deltalcv_dn10_slot = var_deltalcv_dn10;
        *var_deltalcv_dn11_slot = var_deltalcv_dn11;
        *var_deltalcv_dn13_slot = var_deltalcv_dn13;
        *var_deltalcv_dn14_slot = var_deltalcv_dn14;
        *var_deltalcv_dn2_slot = var_deltalcv_dn2;
        *var_deltalcv_dn3_slot = var_deltalcv_dn3;
        *var_deltalcv_dn4_slot = var_deltalcv_dn4;
        *var_deltalcv_dn5_slot = var_deltalcv_dn5;
        *var_deltalcv_dn6_slot = var_deltalcv_dn6;
        *var_deltalcv_dn7_slot = var_deltalcv_dn7;
        *var_deltalcv_dn8_slot = var_deltalcv_dn8;
        *var_deltalcv_dn9_slot = var_deltalcv_dn9;
        *var_deltaprsd_v_slot = var_deltaprsd_v;
        *var_deltemp1_slot = var_deltemp1;
        *var_deltemp1_dn4_slot = var_deltemp1_dn4;
        *var_deltratio1_slot = var_deltratio1;
        *var_deltratio1_dn4_slot = var_deltratio1_dn4;
        *var_devsign_slot = var_devsign;
        *var_devtemp1_slot = var_devtemp1;
        *var_devtemp1_dn4_slot = var_devtemp1_dn4;
        *var_devtempeff_slot = var_devtempeff;
        *var_devtempeff_dn0_slot = var_devtempeff_dn0;
        *var_devtempeff_dn10_slot = var_devtempeff_dn10;
        *var_devtempeff_dn11_slot = var_devtempeff_dn11;
        *var_devtempeff_dn13_slot = var_devtempeff_dn13;
        *var_devtempeff_dn14_slot = var_devtempeff_dn14;
        *var_devtempeff_dn2_slot = var_devtempeff_dn2;
        *var_devtempeff_dn3_slot = var_devtempeff_dn3;
        *var_devtempeff_dn4_slot = var_devtempeff_dn4;
        *var_devtempeff_dn5_slot = var_devtempeff_dn5;
        *var_devtempeff_dn6_slot = var_devtempeff_dn6;
        *var_devtempeff_dn7_slot = var_devtempeff_dn7;
        *var_devtempeff_dn8_slot = var_devtempeff_dn8;
        *var_devtempeff_dn9_slot = var_devtempeff_dn9;
        *var_devtemplow0_slot = var_devtemplow0;
        *var_devtemplow0_dn4_slot = var_devtemplow0_dn4;
        *var_devtemplow1_slot = var_devtemplow1;
        *var_devtemplow1_dn4_slot = var_devtemplow1_dn4;
        *var_dimension1_i_slot = var_dimension1_i;
        *var_dimension2_i_slot = var_dimension2_i;
        *var_dimension3_i_slot = var_dimension3_i;
        *var_dlbin_i_slot = var_dlbin_i;
        *var_e2nom_i_slot = var_e2nom_i;
        *var_e3nom_i_slot = var_e3nom_i;
        *var_epsratio_slot = var_epsratio;
        *var_epssp_slot = var_epssp;
        *var_epssub_slot = var_epssub;
        *var_gdpr_slot = var_gdpr;
        *var_gdpr_dn0_slot = var_gdpr_dn0;
        *var_gdpr_dn10_slot = var_gdpr_dn10;
        *var_gdpr_dn11_slot = var_gdpr_dn11;
        *var_gdpr_dn13_slot = var_gdpr_dn13;
        *var_gdpr_dn14_slot = var_gdpr_dn14;
        *var_gdpr_dn2_slot = var_gdpr_dn2;
        *var_gdpr_dn3_slot = var_gdpr_dn3;
        *var_gdpr_dn4_slot = var_gdpr_dn4;
        *var_gdpr_dn5_slot = var_gdpr_dn5;
        *var_gdpr_dn6_slot = var_gdpr_dn6;
        *var_gdpr_dn7_slot = var_gdpr_dn7;
        *var_gdpr_dn8_slot = var_gdpr_dn8;
        *var_gdpr_dn9_slot = var_gdpr_dn9;
        *var_gfactor_slot = var_gfactor;
        *var_gmin_slot = var_gmin;
        *var_gprime_slot = var_gprime;
        *var_gspr_slot = var_gspr;
        *var_gspr_dn0_slot = var_gspr_dn0;
        *var_gspr_dn10_slot = var_gspr_dn10;
        *var_gspr_dn11_slot = var_gspr_dn11;
        *var_gspr_dn13_slot = var_gspr_dn13;
        *var_gspr_dn14_slot = var_gspr_dn14;
        *var_gspr_dn2_slot = var_gspr_dn2;
        *var_gspr_dn3_slot = var_gspr_dn3;
        *var_gspr_dn4_slot = var_gspr_dn4;
        *var_gspr_dn5_slot = var_gspr_dn5;
        *var_gspr_dn6_slot = var_gspr_dn6;
        *var_gspr_dn7_slot = var_gspr_dn7;
        *var_gspr_dn8_slot = var_gspr_dn8;
        *var_gspr_dn9_slot = var_gspr_dn9;
        *var_guard3_slot = var_guard3;
        *var_guard4_slot = var_guard4;
        *var_gvs_d_slot = var_gvs_d;
        *var_gvs_d_dn0_slot = var_gvs_d_dn0;
        *var_gvs_d_dn10_slot = var_gvs_d_dn10;
        *var_gvs_d_dn11_slot = var_gvs_d_dn11;
        *var_gvs_d_dn13_slot = var_gvs_d_dn13;
        *var_gvs_d_dn14_slot = var_gvs_d_dn14;
        *var_gvs_d_dn2_slot = var_gvs_d_dn2;
        *var_gvs_d_dn3_slot = var_gvs_d_dn3;
        *var_gvs_d_dn4_slot = var_gvs_d_dn4;
        *var_gvs_d_dn5_slot = var_gvs_d_dn5;
        *var_gvs_d_dn6_slot = var_gvs_d_dn6;
        *var_gvs_d_dn7_slot = var_gvs_d_dn7;
        *var_gvs_d_dn8_slot = var_gvs_d_dn8;
        *var_gvs_d_dn9_slot = var_gvs_d_dn9;
        *var_gvs_s_slot = var_gvs_s;
        *var_gvs_s_dn0_slot = var_gvs_s_dn0;
        *var_gvs_s_dn10_slot = var_gvs_s_dn10;
        *var_gvs_s_dn11_slot = var_gvs_s_dn11;
        *var_gvs_s_dn13_slot = var_gvs_s_dn13;
        *var_gvs_s_dn14_slot = var_gvs_s_dn14;
        *var_gvs_s_dn2_slot = var_gvs_s_dn2;
        *var_gvs_s_dn3_slot = var_gvs_s_dn3;
        *var_gvs_s_dn4_slot = var_gvs_s_dn4;
        *var_gvs_s_dn5_slot = var_gvs_s_dn5;
        *var_gvs_s_dn6_slot = var_gvs_s_dn6;
        *var_gvs_s_dn7_slot = var_gvs_s_dn7;
        *var_gvs_s_dn8_slot = var_gvs_s_dn8;
        *var_gvs_s_dn9_slot = var_gvs_s_dn9;
        *var_leff1_slot = var_leff1;
        *var_leff_1_slot = var_leff_1;
        *var_leff_1_dn0_slot = var_leff_1_dn0;
        *var_leff_1_dn10_slot = var_leff_1_dn10;
        *var_leff_1_dn11_slot = var_leff_1_dn11;
        *var_leff_1_dn13_slot = var_leff_1_dn13;
        *var_leff_1_dn14_slot = var_leff_1_dn14;
        *var_leff_1_dn2_slot = var_leff_1_dn2;
        *var_leff_1_dn3_slot = var_leff_1_dn3;
        *var_leff_1_dn4_slot = var_leff_1_dn4;
        *var_leff_1_dn5_slot = var_leff_1_dn5;
        *var_leff_1_dn6_slot = var_leff_1_dn6;
        *var_leff_1_dn7_slot = var_leff_1_dn7;
        *var_leff_1_dn8_slot = var_leff_1_dn8;
        *var_leff_1_dn9_slot = var_leff_1_dn9;
        *var_lg_slot = var_lg;
        *var_lint_i_slot = var_lint_i;
        *var_md_slot = var_md;
        *var_mdprime_slot = var_mdprime;
        *var_mfq1nom_i_slot = var_mfq1nom_i;
        *var_mfq2nom_i_slot = var_mfq2nom_i;
        *var_mfq3nom_i_slot = var_mfq3nom_i;
        *var_mx_slot = var_mx;
        *var_mxprime_slot = var_mxprime;
        *var_nc3d_slot = var_nc3d;
        *var_ncq_slot = var_ncq;
        *var_niln_slot = var_niln;
        *var_niln_dn0_slot = var_niln_dn0;
        *var_niln_dn10_slot = var_niln_dn10;
        *var_niln_dn11_slot = var_niln_dn11;
        *var_niln_dn13_slot = var_niln_dn13;
        *var_niln_dn14_slot = var_niln_dn14;
        *var_niln_dn2_slot = var_niln_dn2;
        *var_niln_dn3_slot = var_niln_dn3;
        *var_niln_dn4_slot = var_niln_dn4;
        *var_niln_dn5_slot = var_niln_dn5;
        *var_niln_dn6_slot = var_niln_dn6;
        *var_niln_dn7_slot = var_niln_dn7;
        *var_niln_dn8_slot = var_niln_dn8;
        *var_niln_dn9_slot = var_niln_dn9;
        *var_p1_slot = var_p1;
        *var_p2_slot = var_p2;
        *var_p3_slot = var_p3;
        *var_qe2_slot = var_qe2;
        *var_qe2_dn0_slot = var_qe2_dn0;
        *var_qe2_dn10_slot = var_qe2_dn10;
        *var_qe2_dn11_slot = var_qe2_dn11;
        *var_qe2_dn13_slot = var_qe2_dn13;
        *var_qe2_dn14_slot = var_qe2_dn14;
        *var_qe2_dn2_slot = var_qe2_dn2;
        *var_qe2_dn3_slot = var_qe2_dn3;
        *var_qe2_dn4_slot = var_qe2_dn4;
        *var_qe2_dn5_slot = var_qe2_dn5;
        *var_qe2_dn6_slot = var_qe2_dn6;
        *var_qe2_dn7_slot = var_qe2_dn7;
        *var_qe2_dn8_slot = var_qe2_dn8;
        *var_qe2_dn9_slot = var_qe2_dn9;
        *var_qe3_slot = var_qe3;
        *var_qnd10_slot = var_qnd10;
        *var_qnd20_slot = var_qnd20;
        *var_qnd30_slot = var_qnd30;
        *var_qndd2_slot = var_qndd2;
        *var_qndd2_dn0_slot = var_qndd2_dn0;
        *var_qndd2_dn10_slot = var_qndd2_dn10;
        *var_qndd2_dn11_slot = var_qndd2_dn11;
        *var_qndd2_dn13_slot = var_qndd2_dn13;
        *var_qndd2_dn14_slot = var_qndd2_dn14;
        *var_qndd2_dn2_slot = var_qndd2_dn2;
        *var_qndd2_dn3_slot = var_qndd2_dn3;
        *var_qndd2_dn4_slot = var_qndd2_dn4;
        *var_qndd2_dn5_slot = var_qndd2_dn5;
        *var_qndd2_dn6_slot = var_qndd2_dn6;
        *var_qndd2_dn7_slot = var_qndd2_dn7;
        *var_qndd2_dn8_slot = var_qndd2_dn8;
        *var_qndd2_dn9_slot = var_qndd2_dn9;
        *var_qndd3_slot = var_qndd3;
        *var_qndd3_dn0_slot = var_qndd3_dn0;
        *var_qndd3_dn10_slot = var_qndd3_dn10;
        *var_qndd3_dn11_slot = var_qndd3_dn11;
        *var_qndd3_dn13_slot = var_qndd3_dn13;
        *var_qndd3_dn14_slot = var_qndd3_dn14;
        *var_qndd3_dn2_slot = var_qndd3_dn2;
        *var_qndd3_dn3_slot = var_qndd3_dn3;
        *var_qndd3_dn4_slot = var_qndd3_dn4;
        *var_qndd3_dn5_slot = var_qndd3_dn5;
        *var_qndd3_dn6_slot = var_qndd3_dn6;
        *var_qndd3_dn7_slot = var_qndd3_dn7;
        *var_qndd3_dn8_slot = var_qndd3_dn8;
        *var_qndd3_dn9_slot = var_qndd3_dn9;
        *var_qsref_i_slot = var_qsref_i;
        *var_rdstempvs_slot = var_rdstempvs;
        *var_rdstempvs_dn4_slot = var_rdstempvs_dn4;
        *var_rvs_d_slot = var_rvs_d;
        *var_rvs_d_dn0_slot = var_rvs_d_dn0;
        *var_rvs_d_dn10_slot = var_rvs_d_dn10;
        *var_rvs_d_dn11_slot = var_rvs_d_dn11;
        *var_rvs_d_dn13_slot = var_rvs_d_dn13;
        *var_rvs_d_dn14_slot = var_rvs_d_dn14;
        *var_rvs_d_dn2_slot = var_rvs_d_dn2;
        *var_rvs_d_dn3_slot = var_rvs_d_dn3;
        *var_rvs_d_dn4_slot = var_rvs_d_dn4;
        *var_rvs_d_dn5_slot = var_rvs_d_dn5;
        *var_rvs_d_dn6_slot = var_rvs_d_dn6;
        *var_rvs_d_dn7_slot = var_rvs_d_dn7;
        *var_rvs_d_dn8_slot = var_rvs_d_dn8;
        *var_rvs_d_dn9_slot = var_rvs_d_dn9;
        *var_rvs_s_slot = var_rvs_s;
        *var_rvs_s_dn0_slot = var_rvs_s_dn0;
        *var_rvs_s_dn10_slot = var_rvs_s_dn10;
        *var_rvs_s_dn11_slot = var_rvs_s_dn11;
        *var_rvs_s_dn13_slot = var_rvs_s_dn13;
        *var_rvs_s_dn14_slot = var_rvs_s_dn14;
        *var_rvs_s_dn2_slot = var_rvs_s_dn2;
        *var_rvs_s_dn3_slot = var_rvs_s_dn3;
        *var_rvs_s_dn4_slot = var_rvs_s_dn4;
        *var_rvs_s_dn5_slot = var_rvs_s_dn5;
        *var_rvs_s_dn6_slot = var_rvs_s_dn6;
        *var_rvs_s_dn7_slot = var_rvs_s_dn7;
        *var_rvs_s_dn8_slot = var_rvs_s_dn8;
        *var_rvs_s_dn9_slot = var_rvs_s_dn9;
        *var_ssp1_i_slot = var_ssp1_i;
        *var_ssp2_i_slot = var_ssp2_i;
        *var_ssp3_i_slot = var_ssp3_i;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_ua_th_slot = var_ua_th;
        *var_ua_th_dn0_slot = var_ua_th_dn0;
        *var_ua_th_dn10_slot = var_ua_th_dn10;
        *var_ua_th_dn11_slot = var_ua_th_dn11;
        *var_ua_th_dn13_slot = var_ua_th_dn13;
        *var_ua_th_dn14_slot = var_ua_th_dn14;
        *var_ua_th_dn2_slot = var_ua_th_dn2;
        *var_ua_th_dn3_slot = var_ua_th_dn3;
        *var_ua_th_dn4_slot = var_ua_th_dn4;
        *var_ua_th_dn5_slot = var_ua_th_dn5;
        *var_ua_th_dn6_slot = var_ua_th_dn6;
        *var_ua_th_dn7_slot = var_ua_th_dn7;
        *var_ua_th_dn8_slot = var_ua_th_dn8;
        *var_ua_th_dn9_slot = var_ua_th_dn9;
        *var_ua_tl_slot = var_ua_tl;
        *var_ua_tl_dn0_slot = var_ua_tl_dn0;
        *var_ua_tl_dn10_slot = var_ua_tl_dn10;
        *var_ua_tl_dn11_slot = var_ua_tl_dn11;
        *var_ua_tl_dn13_slot = var_ua_tl_dn13;
        *var_ua_tl_dn14_slot = var_ua_tl_dn14;
        *var_ua_tl_dn2_slot = var_ua_tl_dn2;
        *var_ua_tl_dn3_slot = var_ua_tl_dn3;
        *var_ua_tl_dn4_slot = var_ua_tl_dn4;
        *var_ua_tl_dn5_slot = var_ua_tl_dn5;
        *var_ua_tl_dn6_slot = var_ua_tl_dn6;
        *var_ua_tl_dn7_slot = var_ua_tl_dn7;
        *var_ua_tl_dn8_slot = var_ua_tl_dn8;
        *var_ua_tl_dn9_slot = var_ua_tl_dn9;
        *var_uar_th_slot = var_uar_th;
        *var_uar_th_dn0_slot = var_uar_th_dn0;
        *var_uar_th_dn10_slot = var_uar_th_dn10;
        *var_uar_th_dn11_slot = var_uar_th_dn11;
        *var_uar_th_dn13_slot = var_uar_th_dn13;
        *var_uar_th_dn14_slot = var_uar_th_dn14;
        *var_uar_th_dn2_slot = var_uar_th_dn2;
        *var_uar_th_dn3_slot = var_uar_th_dn3;
        *var_uar_th_dn4_slot = var_uar_th_dn4;
        *var_uar_th_dn5_slot = var_uar_th_dn5;
        *var_uar_th_dn6_slot = var_uar_th_dn6;
        *var_uar_th_dn7_slot = var_uar_th_dn7;
        *var_uar_th_dn8_slot = var_uar_th_dn8;
        *var_uar_th_dn9_slot = var_uar_th_dn9;
        *var_uar_tl_slot = var_uar_tl;
        *var_uar_tl_dn0_slot = var_uar_tl_dn0;
        *var_uar_tl_dn10_slot = var_uar_tl_dn10;
        *var_uar_tl_dn11_slot = var_uar_tl_dn11;
        *var_uar_tl_dn13_slot = var_uar_tl_dn13;
        *var_uar_tl_dn14_slot = var_uar_tl_dn14;
        *var_uar_tl_dn2_slot = var_uar_tl_dn2;
        *var_uar_tl_dn3_slot = var_uar_tl_dn3;
        *var_uar_tl_dn4_slot = var_uar_tl_dn4;
        *var_uar_tl_dn5_slot = var_uar_tl_dn5;
        *var_uar_tl_dn6_slot = var_uar_tl_dn6;
        *var_uar_tl_dn7_slot = var_uar_tl_dn7;
        *var_uar_tl_dn8_slot = var_uar_tl_dn8;
        *var_uar_tl_dn9_slot = var_uar_tl_dn9;
        *var_udd_t_slot = var_udd_t;
        *var_udd_t_dn4_slot = var_udd_t_dn4;
        *var_uddeff_t_slot = var_uddeff_t;
        *var_uddeff_t_dn4_slot = var_uddeff_t_dn4;
        *var_uds_t_slot = var_uds_t;
        *var_uds_t_dn4_slot = var_uds_t_dn4;
        *var_udseff_t_slot = var_udseff_t;
        *var_udseff_t_dn4_slot = var_udseff_t_dn4;
        *var_vtmeff_slot = var_vtmeff;
        *var_vtmeff_dn0_slot = var_vtmeff_dn0;
        *var_vtmeff_dn10_slot = var_vtmeff_dn10;
        *var_vtmeff_dn11_slot = var_vtmeff_dn11;
        *var_vtmeff_dn13_slot = var_vtmeff_dn13;
        *var_vtmeff_dn14_slot = var_vtmeff_dn14;
        *var_vtmeff_dn2_slot = var_vtmeff_dn2;
        *var_vtmeff_dn3_slot = var_vtmeff_dn3;
        *var_vtmeff_dn4_slot = var_vtmeff_dn4;
        *var_vtmeff_dn5_slot = var_vtmeff_dn5;
        *var_vtmeff_dn6_slot = var_vtmeff_dn6;
        *var_vtmeff_dn7_slot = var_vtmeff_dn7;
        *var_vtmeff_dn8_slot = var_vtmeff_dn8;
        *var_vtmeff_dn9_slot = var_vtmeff_dn9;
        *var_wh_slot = var_wh;
        *var_wh_dn4_slot = var_wh_dn4;
        *var_wl_slot = var_wl;
        *var_wl_dn4_slot = var_wl_dn4;
        *var_xl_i_slot = var_xl_i;
    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        var_deltalcv: f64,
        var_deltalcv_dn0: f64,
        var_deltalcv_dn10: f64,
        var_deltalcv_dn11: f64,
        var_deltalcv_dn13: f64,
        var_deltalcv_dn14: f64,
        var_deltalcv_dn2: f64,
        var_deltalcv_dn3: f64,
        var_deltalcv_dn4: f64,
        var_deltalcv_dn5: f64,
        var_deltalcv_dn6: f64,
        var_deltalcv_dn7: f64,
        var_deltalcv_dn8: f64,
        var_deltalcv_dn9: f64,
        var_lg: f64,
        var_ach_slot: &mut f64,
        var_cins_slot: &mut f64,
        var_dwbin_i_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard12_slot: &mut f64,
        var_guard14_slot: &mut f64,
        var_guard15_slot: &mut f64,
        var_guard16_slot: &mut f64,
        var_guard18_slot: &mut f64,
        var_guard19_slot: &mut f64,
        var_guard20_slot: &mut f64,
        var_guard22_slot: &mut f64,
        var_guard23_slot: &mut f64,
        var_guard24_slot: &mut f64,
        var_guard25_slot: &mut f64,
        var_guard26_slot: &mut f64,
        var_guard27_slot: &mut f64,
        var_guard28_slot: &mut f64,
        var_guard29_slot: &mut f64,
        var_guard30_slot: &mut f64,
        var_guard5_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard9_slot: &mut f64,
        var_inv_l_slot: &mut f64,
        var_inv_lnfin_slot: &mut f64,
        var_inv_nfin_slot: &mut f64,
        var_inv_w_slot: &mut f64,
        var_inv_wl_slot: &mut f64,
        var_leff1_slot: &mut f64,
        var_leff_1_slot: &mut f64,
        var_leff_1_dn0_slot: &mut f64,
        var_leff_1_dn10_slot: &mut f64,
        var_leff_1_dn11_slot: &mut f64,
        var_leff_1_dn13_slot: &mut f64,
        var_leff_1_dn14_slot: &mut f64,
        var_leff_1_dn2_slot: &mut f64,
        var_leff_1_dn3_slot: &mut f64,
        var_leff_1_dn4_slot: &mut f64,
        var_leff_1_dn5_slot: &mut f64,
        var_leff_1_dn6_slot: &mut f64,
        var_leff_1_dn7_slot: &mut f64,
        var_leff_1_dn8_slot: &mut f64,
        var_leff_1_dn9_slot: &mut f64,
        var_leffcv_1_slot: &mut f64,
        var_leffcv_1_dn0_slot: &mut f64,
        var_leffcv_1_dn10_slot: &mut f64,
        var_leffcv_1_dn11_slot: &mut f64,
        var_leffcv_1_dn13_slot: &mut f64,
        var_leffcv_1_dn14_slot: &mut f64,
        var_leffcv_1_dn2_slot: &mut f64,
        var_leffcv_1_dn3_slot: &mut f64,
        var_leffcv_1_dn4_slot: &mut f64,
        var_leffcv_1_dn5_slot: &mut f64,
        var_leffcv_1_dn6_slot: &mut f64,
        var_leffcv_1_dn7_slot: &mut f64,
        var_leffcv_1_dn8_slot: &mut f64,
        var_leffcv_1_dn9_slot: &mut f64,
        var_leffcv_acc_slot: &mut f64,
        var_leffcv_acc_dn0_slot: &mut f64,
        var_leffcv_acc_dn10_slot: &mut f64,
        var_leffcv_acc_dn11_slot: &mut f64,
        var_leffcv_acc_dn13_slot: &mut f64,
        var_leffcv_acc_dn14_slot: &mut f64,
        var_leffcv_acc_dn2_slot: &mut f64,
        var_leffcv_acc_dn3_slot: &mut f64,
        var_leffcv_acc_dn4_slot: &mut f64,
        var_leffcv_acc_dn5_slot: &mut f64,
        var_leffcv_acc_dn6_slot: &mut f64,
        var_leffcv_acc_dn7_slot: &mut f64,
        var_leffcv_acc_dn8_slot: &mut f64,
        var_leffcv_acc_dn9_slot: &mut f64,
        var_nbody_i_slot: &mut f64,
        var_nfintotal_slot: &mut f64,
        var_weff_ufcm_slot: &mut f64,
        var_wgaaeff_slot: &mut f64,
        var_wgaaeff1_slot: &mut f64,
        var_xw_i_slot: &mut f64,
    ) {
        let mut var_ach: f64 = *var_ach_slot;
        let mut var_cins: f64 = *var_cins_slot;
        let mut var_dwbin_i: f64 = *var_dwbin_i_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard12: f64 = *var_guard12_slot;
        let mut var_guard14: f64 = *var_guard14_slot;
        let mut var_guard15: f64 = *var_guard15_slot;
        let mut var_guard16: f64 = *var_guard16_slot;
        let mut var_guard18: f64 = *var_guard18_slot;
        let mut var_guard19: f64 = *var_guard19_slot;
        let mut var_guard20: f64 = *var_guard20_slot;
        let mut var_guard22: f64 = *var_guard22_slot;
        let mut var_guard23: f64 = *var_guard23_slot;
        let mut var_guard24: f64 = *var_guard24_slot;
        let mut var_guard25: f64 = *var_guard25_slot;
        let mut var_guard26: f64 = *var_guard26_slot;
        let mut var_guard27: f64 = *var_guard27_slot;
        let mut var_guard28: f64 = *var_guard28_slot;
        let mut var_guard29: f64 = *var_guard29_slot;
        let mut var_guard30: f64 = *var_guard30_slot;
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard9: f64 = *var_guard9_slot;
        let mut var_inv_l: f64 = *var_inv_l_slot;
        let mut var_inv_lnfin: f64 = *var_inv_lnfin_slot;
        let mut var_inv_nfin: f64 = *var_inv_nfin_slot;
        let mut var_inv_w: f64 = *var_inv_w_slot;
        let mut var_inv_wl: f64 = *var_inv_wl_slot;
        let mut var_leff1: f64 = *var_leff1_slot;
        let mut var_leff_1: f64 = *var_leff_1_slot;
        let mut var_leff_1_dn0: f64 = *var_leff_1_dn0_slot;
        let mut var_leff_1_dn10: f64 = *var_leff_1_dn10_slot;
        let mut var_leff_1_dn11: f64 = *var_leff_1_dn11_slot;
        let mut var_leff_1_dn13: f64 = *var_leff_1_dn13_slot;
        let mut var_leff_1_dn14: f64 = *var_leff_1_dn14_slot;
        let mut var_leff_1_dn2: f64 = *var_leff_1_dn2_slot;
        let mut var_leff_1_dn3: f64 = *var_leff_1_dn3_slot;
        let mut var_leff_1_dn4: f64 = *var_leff_1_dn4_slot;
        let mut var_leff_1_dn5: f64 = *var_leff_1_dn5_slot;
        let mut var_leff_1_dn6: f64 = *var_leff_1_dn6_slot;
        let mut var_leff_1_dn7: f64 = *var_leff_1_dn7_slot;
        let mut var_leff_1_dn8: f64 = *var_leff_1_dn8_slot;
        let mut var_leff_1_dn9: f64 = *var_leff_1_dn9_slot;
        let mut var_leffcv_1: f64 = *var_leffcv_1_slot;
        let mut var_leffcv_1_dn0: f64 = *var_leffcv_1_dn0_slot;
        let mut var_leffcv_1_dn10: f64 = *var_leffcv_1_dn10_slot;
        let mut var_leffcv_1_dn11: f64 = *var_leffcv_1_dn11_slot;
        let mut var_leffcv_1_dn13: f64 = *var_leffcv_1_dn13_slot;
        let mut var_leffcv_1_dn14: f64 = *var_leffcv_1_dn14_slot;
        let mut var_leffcv_1_dn2: f64 = *var_leffcv_1_dn2_slot;
        let mut var_leffcv_1_dn3: f64 = *var_leffcv_1_dn3_slot;
        let mut var_leffcv_1_dn4: f64 = *var_leffcv_1_dn4_slot;
        let mut var_leffcv_1_dn5: f64 = *var_leffcv_1_dn5_slot;
        let mut var_leffcv_1_dn6: f64 = *var_leffcv_1_dn6_slot;
        let mut var_leffcv_1_dn7: f64 = *var_leffcv_1_dn7_slot;
        let mut var_leffcv_1_dn8: f64 = *var_leffcv_1_dn8_slot;
        let mut var_leffcv_1_dn9: f64 = *var_leffcv_1_dn9_slot;
        let mut var_leffcv_acc: f64 = *var_leffcv_acc_slot;
        let mut var_leffcv_acc_dn0: f64 = *var_leffcv_acc_dn0_slot;
        let mut var_leffcv_acc_dn10: f64 = *var_leffcv_acc_dn10_slot;
        let mut var_leffcv_acc_dn11: f64 = *var_leffcv_acc_dn11_slot;
        let mut var_leffcv_acc_dn13: f64 = *var_leffcv_acc_dn13_slot;
        let mut var_leffcv_acc_dn14: f64 = *var_leffcv_acc_dn14_slot;
        let mut var_leffcv_acc_dn2: f64 = *var_leffcv_acc_dn2_slot;
        let mut var_leffcv_acc_dn3: f64 = *var_leffcv_acc_dn3_slot;
        let mut var_leffcv_acc_dn4: f64 = *var_leffcv_acc_dn4_slot;
        let mut var_leffcv_acc_dn5: f64 = *var_leffcv_acc_dn5_slot;
        let mut var_leffcv_acc_dn6: f64 = *var_leffcv_acc_dn6_slot;
        let mut var_leffcv_acc_dn7: f64 = *var_leffcv_acc_dn7_slot;
        let mut var_leffcv_acc_dn8: f64 = *var_leffcv_acc_dn8_slot;
        let mut var_leffcv_acc_dn9: f64 = *var_leffcv_acc_dn9_slot;
        let mut var_nbody_i: f64 = *var_nbody_i_slot;
        let mut var_nfintotal: f64 = *var_nfintotal_slot;
        let mut var_weff_ufcm: f64 = *var_weff_ufcm_slot;
        let mut var_wgaaeff: f64 = *var_wgaaeff_slot;
        let mut var_wgaaeff1: f64 = *var_wgaaeff1_slot;
        let mut var_xw_i: f64 = *var_xw_i_slot;

        let assign2850_e3332: f64 = (2.0 * var_deltalcv);
        let assign2850_e3333: f64 = (var_lg - assign2850_e3332);
        var_leffcv_1 = assign2850_e3333;
        var_leffcv_1_dn0 = (-(2.0 * var_deltalcv_dn0));
        var_leffcv_1_dn2 = (-(2.0 * var_deltalcv_dn2));
        var_leffcv_1_dn3 = (-(2.0 * var_deltalcv_dn3));
        var_leffcv_1_dn4 = (-(2.0 * var_deltalcv_dn4));
        var_leffcv_1_dn5 = (-(2.0 * var_deltalcv_dn5));
        var_leffcv_1_dn6 = (-(2.0 * var_deltalcv_dn6));
        var_leffcv_1_dn7 = (-(2.0 * var_deltalcv_dn7));
        var_leffcv_1_dn8 = (-(2.0 * var_deltalcv_dn8));
        var_leffcv_1_dn9 = (-(2.0 * var_deltalcv_dn9));
        var_leffcv_1_dn10 = (-(2.0 * var_deltalcv_dn10));
        var_leffcv_1_dn11 = (-(2.0 * var_deltalcv_dn11));
        var_leffcv_1_dn13 = (-(2.0 * var_deltalcv_dn13));
        var_leffcv_1_dn14 = (-(2.0 * var_deltalcv_dn14));

        let assign2860_e3336: f64 = (var_leffcv_1 - p.p86);
        var_leffcv_acc = assign2860_e3336;
        var_leffcv_acc_dn0 = var_leffcv_1_dn0;
        var_leffcv_acc_dn2 = var_leffcv_1_dn2;
        var_leffcv_acc_dn3 = var_leffcv_1_dn3;
        var_leffcv_acc_dn4 = var_leffcv_1_dn4;
        var_leffcv_acc_dn5 = var_leffcv_1_dn5;
        var_leffcv_acc_dn6 = var_leffcv_1_dn6;
        var_leffcv_acc_dn7 = var_leffcv_1_dn7;
        var_leffcv_acc_dn8 = var_leffcv_1_dn8;
        var_leffcv_acc_dn9 = var_leffcv_1_dn9;
        var_leffcv_acc_dn10 = var_leffcv_1_dn10;
        var_leffcv_acc_dn11 = var_leffcv_1_dn11;
        var_leffcv_acc_dn13 = var_leffcv_1_dn13;
        var_leffcv_acc_dn14 = var_leffcv_1_dn14;

        let assign2870_e3339: f64 = if var_leff_1 <= 0.0 { 1.0 } else { 0.0 };
        var_guard5 = assign2870_e3339;

        let (assign2880_e3343, assign2880_e3343_d_n0, assign2880_e3343_d_n2, assign2880_e3343_d_n3, assign2880_e3343_d_n4, assign2880_e3343_d_n5, assign2880_e3343_d_n6, assign2880_e3343_d_n7, assign2880_e3343_d_n8, assign2880_e3343_d_n9, assign2880_e3343_d_n10, assign2880_e3343_d_n11, assign2880_e3343_d_n13, assign2880_e3343_d_n14,) = {
    if (var_guard5 != 0.0) {
        (var_lg, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_leff_1, var_leff_1_dn0, var_leff_1_dn2, var_leff_1_dn3, var_leff_1_dn4, var_leff_1_dn5, var_leff_1_dn6, var_leff_1_dn7, var_leff_1_dn8, var_leff_1_dn9, var_leff_1_dn10, var_leff_1_dn11, var_leff_1_dn13, var_leff_1_dn14,)
    }
};
        var_leff_1 = assign2880_e3343;
        var_leff_1_dn0 = assign2880_e3343_d_n0;
        var_leff_1_dn2 = assign2880_e3343_d_n2;
        var_leff_1_dn3 = assign2880_e3343_d_n3;
        var_leff_1_dn4 = assign2880_e3343_d_n4;
        var_leff_1_dn5 = assign2880_e3343_d_n5;
        var_leff_1_dn6 = assign2880_e3343_d_n6;
        var_leff_1_dn7 = assign2880_e3343_d_n7;
        var_leff_1_dn8 = assign2880_e3343_d_n8;
        var_leff_1_dn9 = assign2880_e3343_d_n9;
        var_leff_1_dn10 = assign2880_e3343_d_n10;
        var_leff_1_dn11 = assign2880_e3343_d_n11;
        var_leff_1_dn13 = assign2880_e3343_d_n13;
        var_leff_1_dn14 = assign2880_e3343_d_n14;

        let assign2900_e3349: f64 = if var_leff1 <= 0.0 { 1.0 } else { 0.0 };
        var_guard7 = assign2900_e3349;

        let (assign2910_e3353,) = {
    if (var_guard7 != 0.0) {
        (var_lg,)
    } else {
        (var_leff1,)
    }
};
        var_leff1 = assign2910_e3353;

        let assign2930_e3359: f64 = if var_leffcv_1 <= 0.0 { 1.0 } else { 0.0 };
        var_guard9 = assign2930_e3359;

        let (assign2940_e3363, assign2940_e3363_d_n0, assign2940_e3363_d_n2, assign2940_e3363_d_n3, assign2940_e3363_d_n4, assign2940_e3363_d_n5, assign2940_e3363_d_n6, assign2940_e3363_d_n7, assign2940_e3363_d_n8, assign2940_e3363_d_n9, assign2940_e3363_d_n10, assign2940_e3363_d_n11, assign2940_e3363_d_n13, assign2940_e3363_d_n14,) = {
    if (var_guard9 != 0.0) {
        (var_lg, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_leffcv_1, var_leffcv_1_dn0, var_leffcv_1_dn2, var_leffcv_1_dn3, var_leffcv_1_dn4, var_leffcv_1_dn5, var_leffcv_1_dn6, var_leffcv_1_dn7, var_leffcv_1_dn8, var_leffcv_1_dn9, var_leffcv_1_dn10, var_leffcv_1_dn11, var_leffcv_1_dn13, var_leffcv_1_dn14,)
    }
};
        var_leffcv_1 = assign2940_e3363;
        var_leffcv_1_dn0 = assign2940_e3363_d_n0;
        var_leffcv_1_dn2 = assign2940_e3363_d_n2;
        var_leffcv_1_dn3 = assign2940_e3363_d_n3;
        var_leffcv_1_dn4 = assign2940_e3363_d_n4;
        var_leffcv_1_dn5 = assign2940_e3363_d_n5;
        var_leffcv_1_dn6 = assign2940_e3363_d_n6;
        var_leffcv_1_dn7 = assign2940_e3363_d_n7;
        var_leffcv_1_dn8 = assign2940_e3363_d_n8;
        var_leffcv_1_dn9 = assign2940_e3363_d_n9;
        var_leffcv_1_dn10 = assign2940_e3363_d_n10;
        var_leffcv_1_dn11 = assign2940_e3363_d_n11;
        var_leffcv_1_dn13 = assign2940_e3363_d_n13;
        var_leffcv_1_dn14 = assign2940_e3363_d_n14;

        let assign2960_e3369: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard11 = assign2960_e3369;

        let assign2970_e3372: f64 = if var_leffcv_acc <= 0.0 { 1.0 } else { 0.0 };
        var_guard12 = assign2970_e3372;

        let (assign2980_e3378, assign2980_e3378_d_n0, assign2980_e3378_d_n2, assign2980_e3378_d_n3, assign2980_e3378_d_n4, assign2980_e3378_d_n5, assign2980_e3378_d_n6, assign2980_e3378_d_n7, assign2980_e3378_d_n8, assign2980_e3378_d_n9, assign2980_e3378_d_n10, assign2980_e3378_d_n11, assign2980_e3378_d_n13, assign2980_e3378_d_n14,) = {
    if ((var_guard11 != 0.0) && (var_guard12 != 0.0)) {
        (var_lg, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_leffcv_acc, var_leffcv_acc_dn0, var_leffcv_acc_dn2, var_leffcv_acc_dn3, var_leffcv_acc_dn4, var_leffcv_acc_dn5, var_leffcv_acc_dn6, var_leffcv_acc_dn7, var_leffcv_acc_dn8, var_leffcv_acc_dn9, var_leffcv_acc_dn10, var_leffcv_acc_dn11, var_leffcv_acc_dn13, var_leffcv_acc_dn14,)
    }
};
        var_leffcv_acc = assign2980_e3378;
        var_leffcv_acc_dn0 = assign2980_e3378_d_n0;
        var_leffcv_acc_dn2 = assign2980_e3378_d_n2;
        var_leffcv_acc_dn3 = assign2980_e3378_d_n3;
        var_leffcv_acc_dn4 = assign2980_e3378_d_n4;
        var_leffcv_acc_dn5 = assign2980_e3378_d_n5;
        var_leffcv_acc_dn6 = assign2980_e3378_d_n6;
        var_leffcv_acc_dn7 = assign2980_e3378_d_n7;
        var_leffcv_acc_dn8 = assign2980_e3378_d_n8;
        var_leffcv_acc_dn9 = assign2980_e3378_d_n9;
        var_leffcv_acc_dn10 = assign2980_e3378_d_n10;
        var_leffcv_acc_dn11 = assign2980_e3378_d_n11;
        var_leffcv_acc_dn13 = assign2980_e3378_d_n13;
        var_leffcv_acc_dn14 = assign2980_e3378_d_n14;

        let assign3000_e3384: f64 = if p.p62 == 5.0 { 1.0 } else { 0.0 };
        var_guard14 = assign3000_e3384;

        let (assign3010_e3420,) = {
    if (var_guard14 != 0.0) {
        let assign3010_e3389: f64 = (1e-6 * p.p122);
        let assign3010_e3391: f64 = (assign3010_e3389 / p.p0);
        let assign3010_e3392: f64 = (p.p121 + assign3010_e3391);
        let assign3010_e3395: f64 = (p.p123 / p.p5);
        let assign3010_e3396: f64 = (assign3010_e3392 + assign3010_e3395);
        let assign3010_e3399: f64 = (p.p124 * 1e-6);
        let assign3010_e3402: f64 = (p.p0 * p.p5);
        let assign3010_e3403: f64 = (assign3010_e3399 / assign3010_e3402);
        let assign3010_e3404: f64 = (assign3010_e3396 + assign3010_e3403);
        let assign3010_e3407: f64 = (1e-6 * p.p125);
        let assign3010_e3409: f64 = (assign3010_e3407 / p.p43);
        let assign3010_e3410: f64 = (assign3010_e3404 + assign3010_e3409);
        let assign3010_e3413: f64 = (p.p126 * 1e-12);
        let assign3010_e3416: f64 = (p.p0 * p.p43);
        let assign3010_e3417: f64 = (assign3010_e3413 / assign3010_e3416);
        let assign3010_e3418: f64 = (assign3010_e3410 + assign3010_e3417);
        (assign3010_e3418,)
    } else {
        (var_xw_i,)
    }
};
        var_xw_i = assign3010_e3420;

        let (assign3020_e3456,) = {
    if (var_guard14 != 0.0) {
        let assign3020_e3425: f64 = (1e-6 * p.p128);
        let assign3020_e3427: f64 = (assign3020_e3425 / p.p0);
        let assign3020_e3428: f64 = (p.p127 + assign3020_e3427);
        let assign3020_e3431: f64 = (p.p129 / p.p5);
        let assign3020_e3432: f64 = (assign3020_e3428 + assign3020_e3431);
        let assign3020_e3435: f64 = (p.p130 * 1e-6);
        let assign3020_e3438: f64 = (p.p0 * p.p5);
        let assign3020_e3439: f64 = (assign3020_e3435 / assign3020_e3438);
        let assign3020_e3440: f64 = (assign3020_e3432 + assign3020_e3439);
        let assign3020_e3443: f64 = (1e-6 * p.p131);
        let assign3020_e3445: f64 = (assign3020_e3443 / p.p43);
        let assign3020_e3446: f64 = (assign3020_e3440 + assign3020_e3445);
        let assign3020_e3449: f64 = (p.p132 * 1e-12);
        let assign3020_e3452: f64 = (p.p0 * p.p43);
        let assign3020_e3453: f64 = (assign3020_e3449 / assign3020_e3452);
        let assign3020_e3454: f64 = (assign3020_e3446 + assign3020_e3453);
        (assign3020_e3454,)
    } else {
        (var_dwbin_i,)
    }
};
        var_dwbin_i = assign3020_e3456;

        let (assign3030_e3461,) = {
    if (var_guard14 == 0.0) {
        (0.0,)
    } else {
        (var_xw_i,)
    }
};
        var_xw_i = assign3030_e3461;

        let (assign3040_e3466,) = {
    if (var_guard14 == 0.0) {
        (0.0,)
    } else {
        (var_dwbin_i,)
    }
};
        var_dwbin_i = assign3040_e3466;

        let assign3050_e3469: f64 = (p.p43 + var_xw_i);
        var_wgaaeff = assign3050_e3469;

        let assign3060_e3472: f64 = (var_wgaaeff + var_dwbin_i);
        var_wgaaeff1 = assign3060_e3472;

        let assign3070_e3475: f64 = if p.p62 == 5.0 { 1.0 } else { 0.0 };
        var_guard15 = assign3070_e3475;

        let assign3080_e3478: f64 = if var_wgaaeff1 <= 0.0 { 1.0 } else { 0.0 };
        var_guard16 = assign3080_e3478;

        let (assign3090_e3484,) = {
    if ((var_guard15 != 0.0) && (var_guard16 != 0.0)) {
        (p.p43,)
    } else {
        (var_wgaaeff1,)
    }
};
        var_wgaaeff1 = assign3090_e3484;

        let assign3110_e3490: f64 = (p.p5 * p.p59);
        var_nfintotal = assign3110_e3490;

        let assign3120_e3493: f64 = (1e-6 / var_leff1);
        var_inv_l = assign3120_e3493;

        let assign3130_e3496: f64 = (1.0 / p.p5);
        var_inv_nfin = assign3130_e3496;

        let assign3140_e3500: f64 = (var_leff1 * p.p5);
        let assign3140_e3501: f64 = (1e-6 / assign3140_e3500);
        var_inv_lnfin = assign3140_e3501;

        let assign3150_e3504: f64 = if p.p62 == 5.0 { 1.0 } else { 0.0 };
        var_guard18 = assign3150_e3504;

        let (assign3160_e3510,) = {
    if (var_guard18 != 0.0) {
        let assign3160_e3508: f64 = (1e-6 / var_wgaaeff1);
        (assign3160_e3508,)
    } else {
        (var_inv_w,)
    }
};
        var_inv_w = assign3160_e3510;

        let (assign3170_e3518,) = {
    if (var_guard18 != 0.0) {
        let assign3170_e3515: f64 = (var_wgaaeff1 * var_leff1);
        let assign3170_e3516: f64 = (1e-12 / assign3170_e3515);
        (assign3170_e3516,)
    } else {
        (var_inv_wl,)
    }
};
        var_inv_wl = assign3170_e3518;

        let (assign3180_e3523,) = {
    if (var_guard18 == 0.0) {
        (0.0,)
    } else {
        (var_inv_w,)
    }
};
        var_inv_w = assign3180_e3523;

        let (assign3190_e3528,) = {
    if (var_guard18 == 0.0) {
        (0.0,)
    } else {
        (var_inv_wl,)
    }
};
        var_inv_wl = assign3190_e3528;

        let assign3200_e3532: f64 = (var_inv_l * p.p134);
        let assign3200_e3533: f64 = (p.p133 + assign3200_e3532);
        let assign3200_e3536: f64 = (var_inv_nfin * p.p135);
        let assign3200_e3537: f64 = (assign3200_e3533 + assign3200_e3536);
        let assign3200_e3540: f64 = (var_inv_lnfin * p.p136);
        let assign3200_e3541: f64 = (assign3200_e3537 + assign3200_e3540);
        let assign3200_e3545: f64 = assign3200_e3541;
        let assign3200_e3549: f64 = assign3200_e3545;
        var_nbody_i = assign3200_e3549;

        let assign3210_e3552: f64 = if p.p95 != 0.0 { 1.0 } else { 0.0 };
        var_guard19 = assign3210_e3552;

        let (assign3220_e3589,) = {
    if (var_guard19 != 0.0) {
        let assign3220_e3558: f64 = (p.p95 / p.p5);
        let assign3220_e3562: f64 = (p.p5 / p.p96);
        let assign3220_e3563: f64 = (1.0 + assign3220_e3562);
        let (assign3220_e3584,) = {
            if (!(assign3220_e3563 > 1e-38)) {
                let assign3220_e3568: f64 = (-87.498233534);
                (assign3220_e3568,)
            } else {
                let assign3220_e3572: f64 = (p.p5 / p.p96);
                let assign3220_e3573: f64 = (1.0 + assign3220_e3572);
                let (assign3220_e3583,) = {
                    if (assign3220_e3573 > 1e-38) {
                        let assign3220_e3579: f64 = (p.p5 / p.p96);
                        let assign3220_e3580: f64 = (1.0 + assign3220_e3579);
                        let assign3220_e3581: f64 = (assign3220_e3580).ln();
                        (assign3220_e3581,)
                    } else {
                        (0.0,)
                    }
                };
                (assign3220_e3583,)
            }
        };
        let assign3220_e3585: f64 = (assign3220_e3558 * assign3220_e3584);
        let assign3220_e3586: f64 = (1.0 + assign3220_e3585);
        let assign3220_e3587: f64 = (var_nbody_i * assign3220_e3586);
        (assign3220_e3587,)
    } else {
        (var_nbody_i,)
    }
};
        var_nbody_i = assign3220_e3589;

        let assign3230_e3592: f64 = if var_nbody_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard20 = assign3230_e3592;

        let (assign3240_e3596,) = {
    if (var_guard20 != 0.0) {
        (1e22,)
    } else {
        (var_nbody_i,)
    }
};
        var_nbody_i = assign3240_e3596;

        let assign3260_e3602: f64 = if p.p62 == 0.0 { 1.0 } else { 0.0 };
        var_guard22 = assign3260_e3602;

        let assign3270_e3605: f64 = if p.p62 == 1.0 { 1.0 } else { 0.0 };
        var_guard23 = assign3270_e3605;

        let assign3280_e3608: f64 = if p.p62 == 2.0 { 1.0 } else { 0.0 };
        var_guard24 = assign3280_e3608;

        let assign3290_e3611: f64 = if p.p62 == 3.0 { 1.0 } else { 0.0 };
        var_guard25 = assign3290_e3611;

        let assign3300_e3614: f64 = if p.p62 == 4.0 { 1.0 } else { 0.0 };
        var_guard26 = assign3300_e3614;

        let assign3310_e3617: f64 = if p.p62 == 5.0 { 1.0 } else { 0.0 };
        var_guard27 = assign3310_e3617;

        let assign3320_e3624: f64 = if ((p.p1802 == 0.0) || (p.p1803 == 0.0)) { 1.0 } else { 0.0 };
        var_guard28 = assign3320_e3624;

        let (assign3330_e3632,) = {
    if ((var_guard22 != 0.0) && (var_guard28 != 0.0)) {
        let assign3330_e3630: f64 = (2.0 * p.p92);
        (assign3330_e3630,)
    } else {
        (var_weff_ufcm,)
    }
};
        var_weff_ufcm = assign3330_e3632;

        let (assign3340_e3644,) = {
    if ((var_guard22 != 0.0) && (var_guard28 != 0.0)) {
        let assign3340_e3638: f64 = (var_weff_ufcm * p.p102);
        let assign3340_e3640: f64 = (assign3340_e3638 * 8.8542e-12);
        let assign3340_e3642: f64 = (assign3340_e3640 / p.p89);
        (assign3340_e3642,)
    } else {
        (var_cins,)
    }
};
        var_cins = assign3340_e3644;

        let (assign3350_e3652,) = {
    if ((var_guard22 != 0.0) && (var_guard28 != 0.0)) {
        let assign3350_e3650: f64 = (p.p92 * p.p3);
        (assign3350_e3650,)
    } else {
        (var_ach,)
    }
};
        var_ach = assign3350_e3652;

        let (assign3360_e3674,) = {
    if ((var_guard22 != 0.0) && (var_guard28 == 0.0)) {
        let assign3360_e3660: f64 = (p.p92 * p.p92);
        let assign3360_e3663: f64 = (p.p1802 - p.p1803);
        let assign3360_e3666: f64 = (p.p1802 - p.p1803);
        let assign3360_e3667: f64 = (assign3360_e3663 * assign3360_e3666);
        let assign3360_e3669: f64 = (assign3360_e3667 / 4.0);
        let assign3360_e3670: f64 = (assign3360_e3660 + assign3360_e3669);
        let assign3360_e3671: f64 = (assign3360_e3670).sqrt();
        let assign3360_e3672: f64 = (2.0 * assign3360_e3671);
        (assign3360_e3672,)
    } else {
        (var_weff_ufcm,)
    }
};
        var_weff_ufcm = assign3360_e3674;

        let (assign3370_e3687,) = {
    if ((var_guard22 != 0.0) && (var_guard28 == 0.0)) {
        let assign3370_e3681: f64 = (var_weff_ufcm * p.p102);
        let assign3370_e3683: f64 = (assign3370_e3681 * 8.8542e-12);
        let assign3370_e3685: f64 = (assign3370_e3683 / p.p89);
        (assign3370_e3685,)
    } else {
        (var_cins,)
    }
};
        var_cins = assign3370_e3687;

        let (assign3380_e3700,) = {
    if ((var_guard22 != 0.0) && (var_guard28 == 0.0)) {
        let assign3380_e3695: f64 = (p.p1802 + p.p1803);
        let assign3380_e3696: f64 = (p.p92 * assign3380_e3695);
        let assign3380_e3698: f64 = (assign3380_e3696 / 2.0);
        (assign3380_e3698,)
    } else {
        (var_ach,)
    }
};
        var_ach = assign3380_e3700;

        let assign3390_e3707: f64 = if ((p.p1802 == 0.0) || (p.p1803 == 0.0)) { 1.0 } else { 0.0 };
        var_guard29 = assign3390_e3707;

        let (assign3400_e3720,) = {
    if (((var_guard23 != 0.0) && (var_guard22 == 0.0)) && (var_guard29 != 0.0)) {
        let assign3400_e3716: f64 = (2.0 * p.p92);
        let assign3400_e3718: f64 = (assign3400_e3716 + p.p3);
        (assign3400_e3718,)
    } else {
        (var_weff_ufcm,)
    }
};
        var_weff_ufcm = assign3400_e3720;

        let (assign3410_e3735,) = {
    if (((var_guard23 != 0.0) && (var_guard22 == 0.0)) && (var_guard29 != 0.0)) {
        let assign3410_e3729: f64 = (var_weff_ufcm * p.p102);
        let assign3410_e3731: f64 = (assign3410_e3729 * 8.8542e-12);
        let assign3410_e3733: f64 = (assign3410_e3731 / p.p89);
        (assign3410_e3733,)
    } else {
        (var_cins,)
    }
};
        var_cins = assign3410_e3735;

        let (assign3420_e3746,) = {
    if (((var_guard23 != 0.0) && (var_guard22 == 0.0)) && (var_guard29 != 0.0)) {
        let assign3420_e3744: f64 = (p.p92 * p.p3);
        (assign3420_e3744,)
    } else {
        (var_ach,)
    }
};
        var_ach = assign3420_e3746;

        let (assign3430_e3773,) = {
    if (((var_guard23 != 0.0) && (var_guard22 == 0.0)) && (var_guard29 == 0.0)) {
        let assign3430_e3757: f64 = (p.p92 * p.p92);
        let assign3430_e3760: f64 = (p.p1802 - p.p1803);
        let assign3430_e3763: f64 = (p.p1802 - p.p1803);
        let assign3430_e3764: f64 = (assign3430_e3760 * assign3430_e3763);
        let assign3430_e3766: f64 = (assign3430_e3764 / 4.0);
        let assign3430_e3767: f64 = (assign3430_e3757 + assign3430_e3766);
        let assign3430_e3768: f64 = (assign3430_e3767).sqrt();
        let assign3430_e3769: f64 = (2.0 * assign3430_e3768);
        let assign3430_e3771: f64 = (assign3430_e3769 + p.p1802);
        (assign3430_e3771,)
    } else {
        (var_weff_ufcm,)
    }
};
        var_weff_ufcm = assign3430_e3773;

        let (assign3440_e3789,) = {
    if (((var_guard23 != 0.0) && (var_guard22 == 0.0)) && (var_guard29 == 0.0)) {
        let assign3440_e3783: f64 = (var_weff_ufcm * p.p102);
        let assign3440_e3785: f64 = (assign3440_e3783 * 8.8542e-12);
        let assign3440_e3787: f64 = (assign3440_e3785 / p.p89);
        (assign3440_e3787,)
    } else {
        (var_cins,)
    }
};
        var_cins = assign3440_e3789;

        let (assign3450_e3805,) = {
    if (((var_guard23 != 0.0) && (var_guard22 == 0.0)) && (var_guard29 == 0.0)) {
        let assign3450_e3800: f64 = (p.p1802 + p.p1803);
        let assign3450_e3801: f64 = (p.p92 * assign3450_e3800);
        let assign3450_e3803: f64 = (assign3450_e3801 / 2.0);
        (assign3450_e3803,)
    } else {
        (var_ach,)
    }
};
        var_ach = assign3450_e3805;

        let assign3460_e3812: f64 = if ((p.p1802 == 0.0) || (p.p1803 == 0.0)) { 1.0 } else { 0.0 };
        var_guard30 = assign3460_e3812;

        *var_ach_slot = var_ach;
        *var_cins_slot = var_cins;
        *var_dwbin_i_slot = var_dwbin_i;
        *var_guard11_slot = var_guard11;
        *var_guard12_slot = var_guard12;
        *var_guard14_slot = var_guard14;
        *var_guard15_slot = var_guard15;
        *var_guard16_slot = var_guard16;
        *var_guard18_slot = var_guard18;
        *var_guard19_slot = var_guard19;
        *var_guard20_slot = var_guard20;
        *var_guard22_slot = var_guard22;
        *var_guard23_slot = var_guard23;
        *var_guard24_slot = var_guard24;
        *var_guard25_slot = var_guard25;
        *var_guard26_slot = var_guard26;
        *var_guard27_slot = var_guard27;
        *var_guard28_slot = var_guard28;
        *var_guard29_slot = var_guard29;
        *var_guard30_slot = var_guard30;
        *var_guard5_slot = var_guard5;
        *var_guard7_slot = var_guard7;
        *var_guard9_slot = var_guard9;
        *var_inv_l_slot = var_inv_l;
        *var_inv_lnfin_slot = var_inv_lnfin;
        *var_inv_nfin_slot = var_inv_nfin;
        *var_inv_w_slot = var_inv_w;
        *var_inv_wl_slot = var_inv_wl;
        *var_leff1_slot = var_leff1;
        *var_leff_1_slot = var_leff_1;
        *var_leff_1_dn0_slot = var_leff_1_dn0;
        *var_leff_1_dn10_slot = var_leff_1_dn10;
        *var_leff_1_dn11_slot = var_leff_1_dn11;
        *var_leff_1_dn13_slot = var_leff_1_dn13;
        *var_leff_1_dn14_slot = var_leff_1_dn14;
        *var_leff_1_dn2_slot = var_leff_1_dn2;
        *var_leff_1_dn3_slot = var_leff_1_dn3;
        *var_leff_1_dn4_slot = var_leff_1_dn4;
        *var_leff_1_dn5_slot = var_leff_1_dn5;
        *var_leff_1_dn6_slot = var_leff_1_dn6;
        *var_leff_1_dn7_slot = var_leff_1_dn7;
        *var_leff_1_dn8_slot = var_leff_1_dn8;
        *var_leff_1_dn9_slot = var_leff_1_dn9;
        *var_leffcv_1_slot = var_leffcv_1;
        *var_leffcv_1_dn0_slot = var_leffcv_1_dn0;
        *var_leffcv_1_dn10_slot = var_leffcv_1_dn10;
        *var_leffcv_1_dn11_slot = var_leffcv_1_dn11;
        *var_leffcv_1_dn13_slot = var_leffcv_1_dn13;
        *var_leffcv_1_dn14_slot = var_leffcv_1_dn14;
        *var_leffcv_1_dn2_slot = var_leffcv_1_dn2;
        *var_leffcv_1_dn3_slot = var_leffcv_1_dn3;
        *var_leffcv_1_dn4_slot = var_leffcv_1_dn4;
        *var_leffcv_1_dn5_slot = var_leffcv_1_dn5;
        *var_leffcv_1_dn6_slot = var_leffcv_1_dn6;
        *var_leffcv_1_dn7_slot = var_leffcv_1_dn7;
        *var_leffcv_1_dn8_slot = var_leffcv_1_dn8;
        *var_leffcv_1_dn9_slot = var_leffcv_1_dn9;
        *var_leffcv_acc_slot = var_leffcv_acc;
        *var_leffcv_acc_dn0_slot = var_leffcv_acc_dn0;
        *var_leffcv_acc_dn10_slot = var_leffcv_acc_dn10;
        *var_leffcv_acc_dn11_slot = var_leffcv_acc_dn11;
        *var_leffcv_acc_dn13_slot = var_leffcv_acc_dn13;
        *var_leffcv_acc_dn14_slot = var_leffcv_acc_dn14;
        *var_leffcv_acc_dn2_slot = var_leffcv_acc_dn2;
        *var_leffcv_acc_dn3_slot = var_leffcv_acc_dn3;
        *var_leffcv_acc_dn4_slot = var_leffcv_acc_dn4;
        *var_leffcv_acc_dn5_slot = var_leffcv_acc_dn5;
        *var_leffcv_acc_dn6_slot = var_leffcv_acc_dn6;
        *var_leffcv_acc_dn7_slot = var_leffcv_acc_dn7;
        *var_leffcv_acc_dn8_slot = var_leffcv_acc_dn8;
        *var_leffcv_acc_dn9_slot = var_leffcv_acc_dn9;
        *var_nbody_i_slot = var_nbody_i;
        *var_nfintotal_slot = var_nfintotal;
        *var_weff_ufcm_slot = var_weff_ufcm;
        *var_wgaaeff_slot = var_wgaaeff;
        *var_wgaaeff1_slot = var_wgaaeff1;
        *var_xw_i_slot = var_xw_i;
    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        var_epssub: f64,
        var_guard22: f64,
        var_guard23: f64,
        var_guard24: f64,
        var_guard25: f64,
        var_guard26: f64,
        var_guard27: f64,
        var_guard30: f64,
        var_nbody_i: f64,
        var_wgaaeff: f64,
        var_ach_slot: &mut f64,
        var_ach1_slot: &mut f64,
        var_ach2_slot: &mut f64,
        var_ach3_slot: &mut f64,
        var_ach4_slot: &mut f64,
        var_ach5_slot: &mut f64,
        var_ach6_slot: &mut f64,
        var_cins_slot: &mut f64,
        var_cox_slot: &mut f64,
        var_guard31_slot: &mut f64,
        var_guard32_slot: &mut f64,
        var_guard33_slot: &mut f64,
        var_guard34_slot: &mut f64,
        var_guard35_slot: &mut f64,
        var_qdep_ov_cins_slot: &mut f64,
        var_rc_slot: &mut f64,
        var_weff1_slot: &mut f64,
        var_weff2_slot: &mut f64,
        var_weff3_slot: &mut f64,
        var_weff4_slot: &mut f64,
        var_weff5_slot: &mut f64,
        var_weff6_slot: &mut f64,
        var_weff_ufcm_slot: &mut f64,
        var_weffb_slot: &mut f64,
    ) {
        let mut var_ach: f64 = *var_ach_slot;
        let mut var_ach1: f64 = *var_ach1_slot;
        let mut var_ach2: f64 = *var_ach2_slot;
        let mut var_ach3: f64 = *var_ach3_slot;
        let mut var_ach4: f64 = *var_ach4_slot;
        let mut var_ach5: f64 = *var_ach5_slot;
        let mut var_ach6: f64 = *var_ach6_slot;
        let mut var_cins: f64 = *var_cins_slot;
        let mut var_cox: f64 = *var_cox_slot;
        let mut var_guard31: f64 = *var_guard31_slot;
        let mut var_guard32: f64 = *var_guard32_slot;
        let mut var_guard33: f64 = *var_guard33_slot;
        let mut var_guard34: f64 = *var_guard34_slot;
        let mut var_guard35: f64 = *var_guard35_slot;
        let mut var_qdep_ov_cins: f64 = *var_qdep_ov_cins_slot;
        let mut var_rc: f64 = *var_rc_slot;
        let mut var_weff1: f64 = *var_weff1_slot;
        let mut var_weff2: f64 = *var_weff2_slot;
        let mut var_weff3: f64 = *var_weff3_slot;
        let mut var_weff4: f64 = *var_weff4_slot;
        let mut var_weff5: f64 = *var_weff5_slot;
        let mut var_weff6: f64 = *var_weff6_slot;
        let mut var_weff_ufcm: f64 = *var_weff_ufcm_slot;
        let mut var_weffb: f64 = *var_weffb_slot;

        let (assign3470_e3829,) = {
    if (((var_guard24 != 0.0) && (!((var_guard22 != 0.0) || (var_guard23 != 0.0)))) && (var_guard30 != 0.0)) {
        let assign3470_e3823: f64 = (2.0 * p.p92);
        let assign3470_e3826: f64 = (2.0 * p.p3);
        let assign3470_e3827: f64 = (assign3470_e3823 + assign3470_e3826);
        (assign3470_e3827,)
    } else {
        (var_weff_ufcm,)
    }
};
        var_weff_ufcm = assign3470_e3829;

        let (assign3480_e3846,) = {
    if (((var_guard24 != 0.0) && (!((var_guard22 != 0.0) || (var_guard23 != 0.0)))) && (var_guard30 != 0.0)) {
        let assign3480_e3840: f64 = (var_weff_ufcm * p.p102);
        let assign3480_e3842: f64 = (assign3480_e3840 * 8.8542e-12);
        let assign3480_e3844: f64 = (assign3480_e3842 / p.p89);
        (assign3480_e3844,)
    } else {
        (var_cins,)
    }
};
        var_cins = assign3480_e3846;

        let (assign3490_e3859,) = {
    if (((var_guard24 != 0.0) && (!((var_guard22 != 0.0) || (var_guard23 != 0.0)))) && (var_guard30 != 0.0)) {
        let assign3490_e3857: f64 = (p.p92 * p.p3);
        (assign3490_e3857,)
    } else {
        (var_ach,)
    }
};
        var_ach = assign3490_e3859;

        let (assign3500_e3890,) = {
    if (((var_guard24 != 0.0) && (!((var_guard22 != 0.0) || (var_guard23 != 0.0)))) && (var_guard30 == 0.0)) {
        let assign3500_e3872: f64 = (p.p92 * p.p92);
        let assign3500_e3875: f64 = (p.p1802 - p.p1803);
        let assign3500_e3878: f64 = (p.p1802 - p.p1803);
        let assign3500_e3879: f64 = (assign3500_e3875 * assign3500_e3878);
        let assign3500_e3881: f64 = (assign3500_e3879 / 4.0);
        let assign3500_e3882: f64 = (assign3500_e3872 + assign3500_e3881);
        let assign3500_e3883: f64 = (assign3500_e3882).sqrt();
        let assign3500_e3884: f64 = (2.0 * assign3500_e3883);
        let assign3500_e3886: f64 = (assign3500_e3884 + p.p1802);
        let assign3500_e3888: f64 = (assign3500_e3886 + p.p1803);
        (assign3500_e3888,)
    } else {
        (var_weff_ufcm,)
    }
};
        var_weff_ufcm = assign3500_e3890;

        let (assign3510_e3908,) = {
    if (((var_guard24 != 0.0) && (!((var_guard22 != 0.0) || (var_guard23 != 0.0)))) && (var_guard30 == 0.0)) {
        let assign3510_e3902: f64 = (var_weff_ufcm * p.p102);
        let assign3510_e3904: f64 = (assign3510_e3902 * 8.8542e-12);
        let assign3510_e3906: f64 = (assign3510_e3904 / p.p89);
        (assign3510_e3906,)
    } else {
        (var_cins,)
    }
};
        var_cins = assign3510_e3908;

        let (assign3520_e3926,) = {
    if (((var_guard24 != 0.0) && (!((var_guard22 != 0.0) || (var_guard23 != 0.0)))) && (var_guard30 == 0.0)) {
        let assign3520_e3921: f64 = (p.p1802 + p.p1803);
        let assign3520_e3922: f64 = (p.p92 * assign3520_e3921);
        let assign3520_e3924: f64 = (assign3520_e3922 / 2.0);
        (assign3520_e3924,)
    } else {
        (var_ach,)
    }
};
        var_ach = assign3520_e3926;

        let (assign3530_e3935,) = {
    if ((var_guard24 != 0.0) && (!((var_guard22 != 0.0) || (var_guard23 != 0.0)))) {
        (p.p1803,)
    } else {
        (var_weffb,)
    }
};
        var_weffb = assign3530_e3935;

        let (assign3540_e3948,) = {
    if ((var_guard25 != 0.0) && (!(((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)))) {
        let assign3540_e3946: f64 = (3.141592653589793 * p.p2);
        (assign3540_e3946,)
    } else {
        (var_weff_ufcm,)
    }
};
        var_weff_ufcm = assign3540_e3948;

        let (assign3550_e3998,) = {
    if ((var_guard25 != 0.0) && (!(((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)))) {
        let assign3550_e3959: f64 = (2.0 * 3.141592653589793);
        let assign3550_e3961: f64 = (assign3550_e3959 * p.p102);
        let assign3550_e3963: f64 = (assign3550_e3961 * 8.8542e-12);
        let assign3550_e3967: f64 = (2.0 * p.p89);
        let assign3550_e3969: f64 = (assign3550_e3967 / p.p2);
        let assign3550_e3970: f64 = (1.0 + assign3550_e3969);
        let (assign3550_e3995,) = {
            if (!(assign3550_e3970 > 1e-38)) {
                let assign3550_e3975: f64 = (-87.498233534);
                (assign3550_e3975,)
            } else {
                let assign3550_e3979: f64 = (2.0 * p.p89);
                let assign3550_e3981: f64 = (assign3550_e3979 / p.p2);
                let assign3550_e3982: f64 = (1.0 + assign3550_e3981);
                let (assign3550_e3994,) = {
                    if (assign3550_e3982 > 1e-38) {
                        let assign3550_e3988: f64 = (2.0 * p.p89);
                        let assign3550_e3990: f64 = (assign3550_e3988 / p.p2);
                        let assign3550_e3991: f64 = (1.0 + assign3550_e3990);
                        let assign3550_e3992: f64 = (assign3550_e3991).ln();
                        (assign3550_e3992,)
                    } else {
                        (0.0,)
                    }
                };
                (assign3550_e3994,)
            }
        };
        let assign3550_e3996: f64 = (assign3550_e3963 / assign3550_e3995);
        (assign3550_e3996,)
    } else {
        (var_cins,)
    }
};
        var_cins = assign3550_e3998;

        let (assign3560_e4015,) = {
    if ((var_guard25 != 0.0) && (!(((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)))) {
        let assign3560_e4009: f64 = (3.141592653589793 * p.p2);
        let assign3560_e4011: f64 = (assign3560_e4009 * p.p2);
        let assign3560_e4013: f64 = (assign3560_e4011 / 4.0);
        (assign3560_e4013,)
    } else {
        (var_ach,)
    }
};
        var_ach = assign3560_e4015;

        let (assign3570_e4026,) = {
    if ((var_guard25 != 0.0) && (!(((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)))) {
        (p.p2,)
    } else {
        (var_weffb,)
    }
};
        var_weffb = assign3570_e4026;

        let (assign3580_e4039,) = {
    if ((var_guard26 != 0.0) && (!((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)))) {
        (p.p1801,)
    } else {
        (var_weff_ufcm,)
    }
};
        var_weff_ufcm = assign3580_e4039;

        let (assign3590_e4052,) = {
    if ((var_guard26 != 0.0) && (!((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)))) {
        (p.p1800,)
    } else {
        (var_cins,)
    }
};
        var_cins = assign3590_e4052;

        let (assign3600_e4065,) = {
    if ((var_guard26 != 0.0) && (!((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)))) {
        (p.p1799,)
    } else {
        (var_ach,)
    }
};
        var_ach = assign3600_e4065;

        let (assign3610_e4086,) = {
    if ((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) {
        let assign3610_e4081: f64 = (var_wgaaeff + p.p40);
        let assign3610_e4082: f64 = (2.0 * assign3610_e4081);
        let assign3610_e4084: f64 = (assign3610_e4082 + p.p44);
        (assign3610_e4084,)
    } else {
        (var_weff1,)
    }
};
        var_weff1 = assign3610_e4086;

        let (assign3620_e4105,) = {
    if ((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) {
        let assign3620_e4101: f64 = (var_wgaaeff * p.p40);
        let assign3620_e4103: f64 = (assign3620_e4101 + p.p45);
        (assign3620_e4103,)
    } else {
        (var_ach1,)
    }
};
        var_ach1 = assign3620_e4105;

        let (assign3630_e4120,) = {
    if ((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) {
        (var_weff1,)
    } else {
        (var_weff_ufcm,)
    }
};
        var_weff_ufcm = assign3630_e4120;

        let (assign3640_e4135,) = {
    if ((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) {
        (var_ach1,)
    } else {
        (var_ach,)
    }
};
        var_ach = assign3640_e4135;

        let assign3650_e4138: f64 = if p.p56 > 1.0 { 1.0 } else { 0.0 };
        var_guard31 = assign3650_e4138;

        let (assign3660_e4161,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard31 != 0.0)) {
        let assign3660_e4156: f64 = (var_wgaaeff + p.p40);
        let assign3660_e4157: f64 = (2.0 * assign3660_e4156);
        let assign3660_e4159: f64 = (assign3660_e4157 + p.p46);
        (assign3660_e4159,)
    } else {
        (var_weff2,)
    }
};
        var_weff2 = assign3660_e4161;

        let (assign3670_e4182,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard31 != 0.0)) {
        let assign3670_e4178: f64 = (var_wgaaeff * p.p40);
        let assign3670_e4180: f64 = (assign3670_e4178 + p.p47);
        (assign3670_e4180,)
    } else {
        (var_ach2,)
    }
};
        var_ach2 = assign3670_e4182;

        let (assign3680_e4201,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard31 != 0.0)) {
        let assign3680_e4199: f64 = (var_weff1 + var_weff2);
        (assign3680_e4199,)
    } else {
        (var_weff_ufcm,)
    }
};
        var_weff_ufcm = assign3680_e4201;

        let (assign3690_e4220,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard31 != 0.0)) {
        let assign3690_e4218: f64 = (var_ach1 + var_ach2);
        (assign3690_e4218,)
    } else {
        (var_ach,)
    }
};
        var_ach = assign3690_e4220;

        let assign3700_e4223: f64 = if p.p56 > 2.0 { 1.0 } else { 0.0 };
        var_guard32 = assign3700_e4223;

        let (assign3710_e4246,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard32 != 0.0)) {
        let assign3710_e4241: f64 = (var_wgaaeff + p.p40);
        let assign3710_e4242: f64 = (2.0 * assign3710_e4241);
        let assign3710_e4244: f64 = (assign3710_e4242 + p.p48);
        (assign3710_e4244,)
    } else {
        (var_weff3,)
    }
};
        var_weff3 = assign3710_e4246;

        let (assign3720_e4267,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard32 != 0.0)) {
        let assign3720_e4263: f64 = (var_wgaaeff * p.p40);
        let assign3720_e4265: f64 = (assign3720_e4263 + p.p49);
        (assign3720_e4265,)
    } else {
        (var_ach3,)
    }
};
        var_ach3 = assign3720_e4267;

        let (assign3730_e4288,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard32 != 0.0)) {
        let assign3730_e4284: f64 = (var_weff1 + var_weff2);
        let assign3730_e4286: f64 = (assign3730_e4284 + var_weff3);
        (assign3730_e4286,)
    } else {
        (var_weff_ufcm,)
    }
};
        var_weff_ufcm = assign3730_e4288;

        let (assign3740_e4309,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard32 != 0.0)) {
        let assign3740_e4305: f64 = (var_ach1 + var_ach2);
        let assign3740_e4307: f64 = (assign3740_e4305 + var_ach3);
        (assign3740_e4307,)
    } else {
        (var_ach,)
    }
};
        var_ach = assign3740_e4309;

        let assign3750_e4312: f64 = if p.p56 > 3.0 { 1.0 } else { 0.0 };
        var_guard33 = assign3750_e4312;

        let (assign3760_e4335,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard33 != 0.0)) {
        let assign3760_e4330: f64 = (var_wgaaeff + p.p40);
        let assign3760_e4331: f64 = (2.0 * assign3760_e4330);
        let assign3760_e4333: f64 = (assign3760_e4331 + p.p50);
        (assign3760_e4333,)
    } else {
        (var_weff4,)
    }
};
        var_weff4 = assign3760_e4335;

        let (assign3770_e4356,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard33 != 0.0)) {
        let assign3770_e4352: f64 = (var_wgaaeff * p.p40);
        let assign3770_e4354: f64 = (assign3770_e4352 + p.p51);
        (assign3770_e4354,)
    } else {
        (var_ach4,)
    }
};
        var_ach4 = assign3770_e4356;

        let (assign3780_e4379,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard33 != 0.0)) {
        let assign3780_e4373: f64 = (var_weff1 + var_weff2);
        let assign3780_e4375: f64 = (assign3780_e4373 + var_weff3);
        let assign3780_e4377: f64 = (assign3780_e4375 + var_weff4);
        (assign3780_e4377,)
    } else {
        (var_weff_ufcm,)
    }
};
        var_weff_ufcm = assign3780_e4379;

        let (assign3790_e4402,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard33 != 0.0)) {
        let assign3790_e4396: f64 = (var_ach1 + var_ach2);
        let assign3790_e4398: f64 = (assign3790_e4396 + var_ach3);
        let assign3790_e4400: f64 = (assign3790_e4398 + var_ach4);
        (assign3790_e4400,)
    } else {
        (var_ach,)
    }
};
        var_ach = assign3790_e4402;

        let assign3800_e4405: f64 = if p.p56 > 4.0 { 1.0 } else { 0.0 };
        var_guard34 = assign3800_e4405;

        let (assign3810_e4428,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard34 != 0.0)) {
        let assign3810_e4423: f64 = (var_wgaaeff + p.p40);
        let assign3810_e4424: f64 = (2.0 * assign3810_e4423);
        let assign3810_e4426: f64 = (assign3810_e4424 + p.p52);
        (assign3810_e4426,)
    } else {
        (var_weff5,)
    }
};
        var_weff5 = assign3810_e4428;

        let (assign3820_e4449,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard34 != 0.0)) {
        let assign3820_e4445: f64 = (var_wgaaeff * p.p40);
        let assign3820_e4447: f64 = (assign3820_e4445 + p.p53);
        (assign3820_e4447,)
    } else {
        (var_ach5,)
    }
};
        var_ach5 = assign3820_e4449;

        let (assign3830_e4474,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard34 != 0.0)) {
        let assign3830_e4466: f64 = (var_weff1 + var_weff2);
        let assign3830_e4468: f64 = (assign3830_e4466 + var_weff3);
        let assign3830_e4470: f64 = (assign3830_e4468 + var_weff4);
        let assign3830_e4472: f64 = (assign3830_e4470 + var_weff5);
        (assign3830_e4472,)
    } else {
        (var_weff_ufcm,)
    }
};
        var_weff_ufcm = assign3830_e4474;

        let (assign3840_e4499,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard34 != 0.0)) {
        let assign3840_e4491: f64 = (var_ach1 + var_ach2);
        let assign3840_e4493: f64 = (assign3840_e4491 + var_ach3);
        let assign3840_e4495: f64 = (assign3840_e4493 + var_ach4);
        let assign3840_e4497: f64 = (assign3840_e4495 + var_ach5);
        (assign3840_e4497,)
    } else {
        (var_ach,)
    }
};
        var_ach = assign3840_e4499;

        let assign3850_e4502: f64 = if p.p56 > 5.0 { 1.0 } else { 0.0 };
        var_guard35 = assign3850_e4502;

        let (assign3860_e4525,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard35 != 0.0)) {
        let assign3860_e4520: f64 = (var_wgaaeff + p.p40);
        let assign3860_e4521: f64 = (2.0 * assign3860_e4520);
        let assign3860_e4523: f64 = (assign3860_e4521 + p.p54);
        (assign3860_e4523,)
    } else {
        (var_weff6,)
    }
};
        var_weff6 = assign3860_e4525;

        let (assign3870_e4546,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard35 != 0.0)) {
        let assign3870_e4542: f64 = (var_wgaaeff * p.p40);
        let assign3870_e4544: f64 = (assign3870_e4542 + p.p55);
        (assign3870_e4544,)
    } else {
        (var_ach6,)
    }
};
        var_ach6 = assign3870_e4546;

        let (assign3880_e4573,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard35 != 0.0)) {
        let assign3880_e4563: f64 = (var_weff1 + var_weff2);
        let assign3880_e4565: f64 = (assign3880_e4563 + var_weff3);
        let assign3880_e4567: f64 = (assign3880_e4565 + var_weff4);
        let assign3880_e4569: f64 = (assign3880_e4567 + var_weff5);
        let assign3880_e4571: f64 = (assign3880_e4569 + var_weff6);
        (assign3880_e4571,)
    } else {
        (var_weff_ufcm,)
    }
};
        var_weff_ufcm = assign3880_e4573;

        let (assign3890_e4600,) = {
    if (((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) && (var_guard35 != 0.0)) {
        let assign3890_e4590: f64 = (var_ach1 + var_ach2);
        let assign3890_e4592: f64 = (assign3890_e4590 + var_ach3);
        let assign3890_e4594: f64 = (assign3890_e4592 + var_ach4);
        let assign3890_e4596: f64 = (assign3890_e4594 + var_ach5);
        let assign3890_e4598: f64 = (assign3890_e4596 + var_ach6);
        (assign3890_e4598,)
    } else {
        (var_ach,)
    }
};
        var_ach = assign3890_e4600;

        let (assign3900_e4615,) = {
    if ((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) {
        (p.p43,)
    } else {
        (var_weffb,)
    }
};
        var_weffb = assign3900_e4615;

        let (assign3910_e4636,) = {
    if ((var_guard27 != 0.0) && (!(((((var_guard22 != 0.0) || (var_guard23 != 0.0)) || (var_guard24 != 0.0)) || (var_guard25 != 0.0)) || (var_guard26 != 0.0)))) {
        let assign3910_e4630: f64 = (var_weff_ufcm * p.p102);
        let assign3910_e4632: f64 = (assign3910_e4630 * 8.8542e-12);
        let assign3910_e4634: f64 = (assign3910_e4632 / p.p89);
        (assign3910_e4634,)
    } else {
        (var_cins,)
    }
};
        var_cins = assign3910_e4636;

        let assign3920_e4639: f64 = (2.0 * var_cins);
        let assign3920_e4642: f64 = (var_weff_ufcm * var_weff_ufcm);
        let assign3920_e4644: f64 = (assign3920_e4642 * var_epssub);
        let assign3920_e4646: f64 = (assign3920_e4644 / var_ach);
        let assign3920_e4647: f64 = (assign3920_e4639 / assign3920_e4646);
        var_rc = assign3920_e4647;

        let assign3930_e4649: f64 = (-1.60219e-19);
        let assign3930_e4651: f64 = (assign3930_e4649 * var_nbody_i);
        let assign3930_e4653: f64 = (assign3930_e4651 * var_ach);
        let assign3930_e4655: f64 = (assign3930_e4653 / var_cins);
        var_qdep_ov_cins = assign3930_e4655;

        let assign3940_e4658: f64 = (var_cins / var_weff_ufcm);
        var_cox = assign3940_e4658;

        *var_ach_slot = var_ach;
        *var_ach1_slot = var_ach1;
        *var_ach2_slot = var_ach2;
        *var_ach3_slot = var_ach3;
        *var_ach4_slot = var_ach4;
        *var_ach5_slot = var_ach5;
        *var_ach6_slot = var_ach6;
        *var_cins_slot = var_cins;
        *var_cox_slot = var_cox;
        *var_guard31_slot = var_guard31;
        *var_guard32_slot = var_guard32;
        *var_guard33_slot = var_guard33;
        *var_guard34_slot = var_guard34;
        *var_guard35_slot = var_guard35;
        *var_qdep_ov_cins_slot = var_qdep_ov_cins;
        *var_rc_slot = var_rc;
        *var_weff1_slot = var_weff1;
        *var_weff2_slot = var_weff2;
        *var_weff3_slot = var_weff3;
        *var_weff4_slot = var_weff4;
        *var_weff5_slot = var_weff5;
        *var_weff6_slot = var_weff6;
        *var_weff_ufcm_slot = var_weff_ufcm;
        *var_weffb_slot = var_weffb;
    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        var_cox: f64,
        var_inv_l: f64,
        var_inv_lnfin: f64,
        var_inv_nfin: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_weff_ufcm: f64,
        var_cdsc_i_slot: &mut f64,
        var_cdscd_i_slot: &mut f64,
        var_cit_i_slot: &mut f64,
        var_cox_acc_slot: &mut f64,
        var_cox_acc_dn0_slot: &mut f64,
        var_cox_acc_dn10_slot: &mut f64,
        var_cox_acc_dn11_slot: &mut f64,
        var_cox_acc_dn13_slot: &mut f64,
        var_cox_acc_dn14_slot: &mut f64,
        var_cox_acc_dn2_slot: &mut f64,
        var_cox_acc_dn3_slot: &mut f64,
        var_cox_acc_dn4_slot: &mut f64,
        var_cox_acc_dn5_slot: &mut f64,
        var_cox_acc_dn6_slot: &mut f64,
        var_cox_acc_dn7_slot: &mut f64,
        var_cox_acc_dn8_slot: &mut f64,
        var_cox_acc_dn9_slot: &mut f64,
        var_deltaprsd_v_slot: &mut f64,
        var_dsub_i_slot: &mut f64,
        var_dvt0_i_slot: &mut f64,
        var_dvt1_i_slot: &mut f64,
        var_dvt1ss_i_slot: &mut f64,
        var_dvtshift_i_slot: &mut f64,
        var_eta0_i_slot: &mut f64,
        var_eta0_i_dn0_slot: &mut f64,
        var_eta0_i_dn10_slot: &mut f64,
        var_eta0_i_dn11_slot: &mut f64,
        var_eta0_i_dn13_slot: &mut f64,
        var_eta0_i_dn14_slot: &mut f64,
        var_eta0_i_dn2_slot: &mut f64,
        var_eta0_i_dn3_slot: &mut f64,
        var_eta0_i_dn4_slot: &mut f64,
        var_eta0_i_dn5_slot: &mut f64,
        var_eta0_i_dn6_slot: &mut f64,
        var_eta0_i_dn7_slot: &mut f64,
        var_eta0_i_dn8_slot: &mut f64,
        var_eta0_i_dn9_slot: &mut f64,
        var_eta1_i_slot: &mut f64,
        var_guard36_slot: &mut f64,
        var_guard37_slot: &mut f64,
        var_guard38_slot: &mut f64,
        var_guard39_slot: &mut f64,
        var_guard40_slot: &mut f64,
        var_k01_i_slot: &mut f64,
        var_k0_i_slot: &mut f64,
        var_k0si1_i_slot: &mut f64,
        var_k0si_i_slot: &mut f64,
        var_k0sisat1_i_slot: &mut f64,
        var_k0sisat_i_slot: &mut f64,
        var_k1rsce_i_slot: &mut f64,
        var_k2si1_i_slot: &mut f64,
        var_k2si_i_slot: &mut f64,
        var_k2sisat1_i_slot: &mut f64,
        var_k2sisat_i_slot: &mut f64,
        var_lpe0_i_slot: &mut f64,
        var_phig_i_slot: &mut f64,
        var_phig_i_dn0_slot: &mut f64,
        var_phig_i_dn10_slot: &mut f64,
        var_phig_i_dn11_slot: &mut f64,
        var_phig_i_dn13_slot: &mut f64,
        var_phig_i_dn14_slot: &mut f64,
        var_phig_i_dn2_slot: &mut f64,
        var_phig_i_dn3_slot: &mut f64,
        var_phig_i_dn4_slot: &mut f64,
        var_phig_i_dn5_slot: &mut f64,
        var_phig_i_dn6_slot: &mut f64,
        var_phig_i_dn7_slot: &mut f64,
        var_phig_i_dn8_slot: &mut f64,
        var_phig_i_dn9_slot: &mut f64,
        var_phin_i_slot: &mut f64,
        var_pqm_i_slot: &mut f64,
        var_pqm_i_dn0_slot: &mut f64,
        var_pqm_i_dn10_slot: &mut f64,
        var_pqm_i_dn11_slot: &mut f64,
        var_pqm_i_dn13_slot: &mut f64,
        var_pqm_i_dn14_slot: &mut f64,
        var_pqm_i_dn2_slot: &mut f64,
        var_pqm_i_dn3_slot: &mut f64,
        var_pqm_i_dn4_slot: &mut f64,
        var_pqm_i_dn5_slot: &mut f64,
        var_pqm_i_dn6_slot: &mut f64,
        var_pqm_i_dn7_slot: &mut f64,
        var_pqm_i_dn8_slot: &mut f64,
        var_pqm_i_dn9_slot: &mut f64,
        var_qmfactor_i_slot: &mut f64,
        var_qmtcencv_i_slot: &mut f64,
        var_qmtcencva_i_slot: &mut f64,
        var_vsat1_i_slot: &mut f64,
        var_vsat1_i_dn0_slot: &mut f64,
        var_vsat1_i_dn10_slot: &mut f64,
        var_vsat1_i_dn11_slot: &mut f64,
        var_vsat1_i_dn13_slot: &mut f64,
        var_vsat1_i_dn14_slot: &mut f64,
        var_vsat1_i_dn2_slot: &mut f64,
        var_vsat1_i_dn3_slot: &mut f64,
        var_vsat1_i_dn4_slot: &mut f64,
        var_vsat1_i_dn5_slot: &mut f64,
        var_vsat1_i_dn6_slot: &mut f64,
        var_vsat1_i_dn7_slot: &mut f64,
        var_vsat1_i_dn8_slot: &mut f64,
        var_vsat1_i_dn9_slot: &mut f64,
        var_vsat_i_slot: &mut f64,
        var_vsat_i_dn0_slot: &mut f64,
        var_vsat_i_dn10_slot: &mut f64,
        var_vsat_i_dn11_slot: &mut f64,
        var_vsat_i_dn13_slot: &mut f64,
        var_vsat_i_dn14_slot: &mut f64,
        var_vsat_i_dn2_slot: &mut f64,
        var_vsat_i_dn3_slot: &mut f64,
        var_vsat_i_dn4_slot: &mut f64,
        var_vsat_i_dn5_slot: &mut f64,
        var_vsat_i_dn6_slot: &mut f64,
        var_vsat_i_dn7_slot: &mut f64,
        var_vsat_i_dn8_slot: &mut f64,
        var_vsat_i_dn9_slot: &mut f64,
        var_weff0_slot: &mut f64,
        var_weffcv0_slot: &mut f64,
        var_weffcv_acc_slot: &mut f64,
    ) {
        let mut var_cdsc_i: f64 = *var_cdsc_i_slot;
        let mut var_cdscd_i: f64 = *var_cdscd_i_slot;
        let mut var_cit_i: f64 = *var_cit_i_slot;
        let mut var_cox_acc: f64 = *var_cox_acc_slot;
        let mut var_cox_acc_dn0: f64 = *var_cox_acc_dn0_slot;
        let mut var_cox_acc_dn10: f64 = *var_cox_acc_dn10_slot;
        let mut var_cox_acc_dn11: f64 = *var_cox_acc_dn11_slot;
        let mut var_cox_acc_dn13: f64 = *var_cox_acc_dn13_slot;
        let mut var_cox_acc_dn14: f64 = *var_cox_acc_dn14_slot;
        let mut var_cox_acc_dn2: f64 = *var_cox_acc_dn2_slot;
        let mut var_cox_acc_dn3: f64 = *var_cox_acc_dn3_slot;
        let mut var_cox_acc_dn4: f64 = *var_cox_acc_dn4_slot;
        let mut var_cox_acc_dn5: f64 = *var_cox_acc_dn5_slot;
        let mut var_cox_acc_dn6: f64 = *var_cox_acc_dn6_slot;
        let mut var_cox_acc_dn7: f64 = *var_cox_acc_dn7_slot;
        let mut var_cox_acc_dn8: f64 = *var_cox_acc_dn8_slot;
        let mut var_cox_acc_dn9: f64 = *var_cox_acc_dn9_slot;
        let mut var_deltaprsd_v: f64 = *var_deltaprsd_v_slot;
        let mut var_dsub_i: f64 = *var_dsub_i_slot;
        let mut var_dvt0_i: f64 = *var_dvt0_i_slot;
        let mut var_dvt1_i: f64 = *var_dvt1_i_slot;
        let mut var_dvt1ss_i: f64 = *var_dvt1ss_i_slot;
        let mut var_dvtshift_i: f64 = *var_dvtshift_i_slot;
        let mut var_eta0_i: f64 = *var_eta0_i_slot;
        let mut var_eta0_i_dn0: f64 = *var_eta0_i_dn0_slot;
        let mut var_eta0_i_dn10: f64 = *var_eta0_i_dn10_slot;
        let mut var_eta0_i_dn11: f64 = *var_eta0_i_dn11_slot;
        let mut var_eta0_i_dn13: f64 = *var_eta0_i_dn13_slot;
        let mut var_eta0_i_dn14: f64 = *var_eta0_i_dn14_slot;
        let mut var_eta0_i_dn2: f64 = *var_eta0_i_dn2_slot;
        let mut var_eta0_i_dn3: f64 = *var_eta0_i_dn3_slot;
        let mut var_eta0_i_dn4: f64 = *var_eta0_i_dn4_slot;
        let mut var_eta0_i_dn5: f64 = *var_eta0_i_dn5_slot;
        let mut var_eta0_i_dn6: f64 = *var_eta0_i_dn6_slot;
        let mut var_eta0_i_dn7: f64 = *var_eta0_i_dn7_slot;
        let mut var_eta0_i_dn8: f64 = *var_eta0_i_dn8_slot;
        let mut var_eta0_i_dn9: f64 = *var_eta0_i_dn9_slot;
        let mut var_eta1_i: f64 = *var_eta1_i_slot;
        let mut var_guard36: f64 = *var_guard36_slot;
        let mut var_guard37: f64 = *var_guard37_slot;
        let mut var_guard38: f64 = *var_guard38_slot;
        let mut var_guard39: f64 = *var_guard39_slot;
        let mut var_guard40: f64 = *var_guard40_slot;
        let mut var_k01_i: f64 = *var_k01_i_slot;
        let mut var_k0_i: f64 = *var_k0_i_slot;
        let mut var_k0si1_i: f64 = *var_k0si1_i_slot;
        let mut var_k0si_i: f64 = *var_k0si_i_slot;
        let mut var_k0sisat1_i: f64 = *var_k0sisat1_i_slot;
        let mut var_k0sisat_i: f64 = *var_k0sisat_i_slot;
        let mut var_k1rsce_i: f64 = *var_k1rsce_i_slot;
        let mut var_k2si1_i: f64 = *var_k2si1_i_slot;
        let mut var_k2si_i: f64 = *var_k2si_i_slot;
        let mut var_k2sisat1_i: f64 = *var_k2sisat1_i_slot;
        let mut var_k2sisat_i: f64 = *var_k2sisat_i_slot;
        let mut var_lpe0_i: f64 = *var_lpe0_i_slot;
        let mut var_phig_i: f64 = *var_phig_i_slot;
        let mut var_phig_i_dn0: f64 = *var_phig_i_dn0_slot;
        let mut var_phig_i_dn10: f64 = *var_phig_i_dn10_slot;
        let mut var_phig_i_dn11: f64 = *var_phig_i_dn11_slot;
        let mut var_phig_i_dn13: f64 = *var_phig_i_dn13_slot;
        let mut var_phig_i_dn14: f64 = *var_phig_i_dn14_slot;
        let mut var_phig_i_dn2: f64 = *var_phig_i_dn2_slot;
        let mut var_phig_i_dn3: f64 = *var_phig_i_dn3_slot;
        let mut var_phig_i_dn4: f64 = *var_phig_i_dn4_slot;
        let mut var_phig_i_dn5: f64 = *var_phig_i_dn5_slot;
        let mut var_phig_i_dn6: f64 = *var_phig_i_dn6_slot;
        let mut var_phig_i_dn7: f64 = *var_phig_i_dn7_slot;
        let mut var_phig_i_dn8: f64 = *var_phig_i_dn8_slot;
        let mut var_phig_i_dn9: f64 = *var_phig_i_dn9_slot;
        let mut var_phin_i: f64 = *var_phin_i_slot;
        let mut var_pqm_i: f64 = *var_pqm_i_slot;
        let mut var_pqm_i_dn0: f64 = *var_pqm_i_dn0_slot;
        let mut var_pqm_i_dn10: f64 = *var_pqm_i_dn10_slot;
        let mut var_pqm_i_dn11: f64 = *var_pqm_i_dn11_slot;
        let mut var_pqm_i_dn13: f64 = *var_pqm_i_dn13_slot;
        let mut var_pqm_i_dn14: f64 = *var_pqm_i_dn14_slot;
        let mut var_pqm_i_dn2: f64 = *var_pqm_i_dn2_slot;
        let mut var_pqm_i_dn3: f64 = *var_pqm_i_dn3_slot;
        let mut var_pqm_i_dn4: f64 = *var_pqm_i_dn4_slot;
        let mut var_pqm_i_dn5: f64 = *var_pqm_i_dn5_slot;
        let mut var_pqm_i_dn6: f64 = *var_pqm_i_dn6_slot;
        let mut var_pqm_i_dn7: f64 = *var_pqm_i_dn7_slot;
        let mut var_pqm_i_dn8: f64 = *var_pqm_i_dn8_slot;
        let mut var_pqm_i_dn9: f64 = *var_pqm_i_dn9_slot;
        let mut var_qmfactor_i: f64 = *var_qmfactor_i_slot;
        let mut var_qmtcencv_i: f64 = *var_qmtcencv_i_slot;
        let mut var_qmtcencva_i: f64 = *var_qmtcencva_i_slot;
        let mut var_vsat1_i: f64 = *var_vsat1_i_slot;
        let mut var_vsat1_i_dn0: f64 = *var_vsat1_i_dn0_slot;
        let mut var_vsat1_i_dn10: f64 = *var_vsat1_i_dn10_slot;
        let mut var_vsat1_i_dn11: f64 = *var_vsat1_i_dn11_slot;
        let mut var_vsat1_i_dn13: f64 = *var_vsat1_i_dn13_slot;
        let mut var_vsat1_i_dn14: f64 = *var_vsat1_i_dn14_slot;
        let mut var_vsat1_i_dn2: f64 = *var_vsat1_i_dn2_slot;
        let mut var_vsat1_i_dn3: f64 = *var_vsat1_i_dn3_slot;
        let mut var_vsat1_i_dn4: f64 = *var_vsat1_i_dn4_slot;
        let mut var_vsat1_i_dn5: f64 = *var_vsat1_i_dn5_slot;
        let mut var_vsat1_i_dn6: f64 = *var_vsat1_i_dn6_slot;
        let mut var_vsat1_i_dn7: f64 = *var_vsat1_i_dn7_slot;
        let mut var_vsat1_i_dn8: f64 = *var_vsat1_i_dn8_slot;
        let mut var_vsat1_i_dn9: f64 = *var_vsat1_i_dn9_slot;
        let mut var_vsat_i: f64 = *var_vsat_i_slot;
        let mut var_vsat_i_dn0: f64 = *var_vsat_i_dn0_slot;
        let mut var_vsat_i_dn10: f64 = *var_vsat_i_dn10_slot;
        let mut var_vsat_i_dn11: f64 = *var_vsat_i_dn11_slot;
        let mut var_vsat_i_dn13: f64 = *var_vsat_i_dn13_slot;
        let mut var_vsat_i_dn14: f64 = *var_vsat_i_dn14_slot;
        let mut var_vsat_i_dn2: f64 = *var_vsat_i_dn2_slot;
        let mut var_vsat_i_dn3: f64 = *var_vsat_i_dn3_slot;
        let mut var_vsat_i_dn4: f64 = *var_vsat_i_dn4_slot;
        let mut var_vsat_i_dn5: f64 = *var_vsat_i_dn5_slot;
        let mut var_vsat_i_dn6: f64 = *var_vsat_i_dn6_slot;
        let mut var_vsat_i_dn7: f64 = *var_vsat_i_dn7_slot;
        let mut var_vsat_i_dn8: f64 = *var_vsat_i_dn8_slot;
        let mut var_vsat_i_dn9: f64 = *var_vsat_i_dn9_slot;
        let mut var_weff0: f64 = *var_weff0_slot;
        let mut var_weffcv0: f64 = *var_weffcv0_slot;
        let mut var_weffcv_acc: f64 = *var_weffcv_acc_slot;

        let assign3950_e4661: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard36 = assign3950_e4661;

        let (assign3960_e4669, assign3960_e4669_d_n0, assign3960_e4669_d_n2, assign3960_e4669_d_n3, assign3960_e4669_d_n4, assign3960_e4669_d_n5, assign3960_e4669_d_n6, assign3960_e4669_d_n7, assign3960_e4669_d_n8, assign3960_e4669_d_n9, assign3960_e4669_d_n10, assign3960_e4669_d_n11, assign3960_e4669_d_n13, assign3960_e4669_d_n14,) = {
    if (var_guard36 != 0.0) {
        let assign3960_e4665: f64 = (var_cox * p.p89);
        let assign3960_e4667: f64 = (assign3960_e4665 / p.p1528);
        (assign3960_e4667, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cox_acc, var_cox_acc_dn0, var_cox_acc_dn2, var_cox_acc_dn3, var_cox_acc_dn4, var_cox_acc_dn5, var_cox_acc_dn6, var_cox_acc_dn7, var_cox_acc_dn8, var_cox_acc_dn9, var_cox_acc_dn10, var_cox_acc_dn11, var_cox_acc_dn13, var_cox_acc_dn14,)
    }
};
        var_cox_acc = assign3960_e4669;
        var_cox_acc_dn0 = assign3960_e4669_d_n0;
        var_cox_acc_dn2 = assign3960_e4669_d_n2;
        var_cox_acc_dn3 = assign3960_e4669_d_n3;
        var_cox_acc_dn4 = assign3960_e4669_d_n4;
        var_cox_acc_dn5 = assign3960_e4669_d_n5;
        var_cox_acc_dn6 = assign3960_e4669_d_n6;
        var_cox_acc_dn7 = assign3960_e4669_d_n7;
        var_cox_acc_dn8 = assign3960_e4669_d_n8;
        var_cox_acc_dn9 = assign3960_e4669_d_n9;
        var_cox_acc_dn10 = assign3960_e4669_d_n10;
        var_cox_acc_dn11 = assign3960_e4669_d_n11;
        var_cox_acc_dn13 = assign3960_e4669_d_n13;
        var_cox_acc_dn14 = assign3960_e4669_d_n14;

        let assign3970_e4672: f64 = (var_weff_ufcm - p.p93);
        var_weff0 = assign3970_e4672;

        let assign3980_e4675: f64 = (var_weff_ufcm - p.p94);
        var_weffcv0 = assign3980_e4675;

        let assign3990_e4678: f64 = if p.p62 == 5.0 { 1.0 } else { 0.0 };
        var_guard37 = assign3990_e4678;

        let (assign4000_e4688,) = {
    if (var_guard37 != 0.0) {
        let assign4000_e4683: f64 = (2.0 * p.p56);
        let assign4000_e4685: f64 = (assign4000_e4683 * p.p87);
        let assign4000_e4686: f64 = (var_weff0 - assign4000_e4685);
        (assign4000_e4686,)
    } else {
        (var_weffcv_acc,)
    }
};
        var_weffcv_acc = assign4000_e4688;

        let (assign4010_e4693,) = {
    if (var_guard37 == 0.0) {
        (var_weff0,)
    } else {
        (var_weffcv_acc,)
    }
};
        var_weffcv_acc = assign4010_e4693;

        let assign4020_e4696: f64 = if p.p62 == 5.0 { 1.0 } else { 0.0 };
        var_guard38 = assign4020_e4696;

        let assign4030_e4699: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard39 = assign4030_e4699;

        let assign4040_e4702: f64 = if var_weffcv_acc <= 0.0 { 1.0 } else { 0.0 };
        var_guard40 = assign4040_e4702;

        let (assign4050_e4710,) = {
    if (((var_guard38 != 0.0) && (var_guard39 != 0.0)) && (var_guard40 != 0.0)) {
        (var_weff_ufcm,)
    } else {
        (var_weffcv_acc,)
    }
};
        var_weffcv_acc = assign4050_e4710;

        var_deltaprsd_v = p.p1085;

        let assign4090_e4719: f64 = (var_inv_l * p.p138);
        let assign4090_e4720: f64 = (p.p137 + assign4090_e4719);
        let assign4090_e4723: f64 = (var_inv_nfin * p.p139);
        let assign4090_e4724: f64 = (assign4090_e4720 + assign4090_e4723);
        let assign4090_e4727: f64 = (var_inv_lnfin * p.p140);
        let assign4090_e4728: f64 = (assign4090_e4724 + assign4090_e4727);
        let assign4090_e4731: f64 = (var_inv_w * p.p141);
        let assign4090_e4732: f64 = (assign4090_e4728 + assign4090_e4731);
        let assign4090_e4735: f64 = (var_inv_wl * p.p142);
        let assign4090_e4736: f64 = (assign4090_e4732 + assign4090_e4735);
        var_phig_i = assign4090_e4736;
        var_phig_i_dn0 = 0.0;
        var_phig_i_dn2 = 0.0;
        var_phig_i_dn3 = 0.0;
        var_phig_i_dn4 = 0.0;
        var_phig_i_dn5 = 0.0;
        var_phig_i_dn6 = 0.0;
        var_phig_i_dn7 = 0.0;
        var_phig_i_dn8 = 0.0;
        var_phig_i_dn9 = 0.0;
        var_phig_i_dn10 = 0.0;
        var_phig_i_dn11 = 0.0;
        var_phig_i_dn13 = 0.0;
        var_phig_i_dn14 = 0.0;

        let assign4110_e4761: f64 = (var_inv_l * p.p189);
        let assign4110_e4762: f64 = (p.p188 + assign4110_e4761);
        let assign4110_e4765: f64 = (var_inv_nfin * p.p190);
        let assign4110_e4766: f64 = (assign4110_e4762 + assign4110_e4765);
        let assign4110_e4769: f64 = (var_inv_lnfin * p.p191);
        let assign4110_e4770: f64 = (assign4110_e4766 + assign4110_e4769);
        let assign4110_e4773: f64 = (var_inv_w * p.p192);
        let assign4110_e4774: f64 = (assign4110_e4770 + assign4110_e4773);
        let assign4110_e4777: f64 = (var_inv_wl * p.p193);
        let assign4110_e4778: f64 = (assign4110_e4774 + assign4110_e4777);
        var_cit_i = assign4110_e4778;

        let assign4120_e4782: f64 = (var_inv_l * p.p201);
        let assign4120_e4783: f64 = (p.p200 + assign4120_e4782);
        let assign4120_e4786: f64 = (var_inv_nfin * p.p202);
        let assign4120_e4787: f64 = (assign4120_e4783 + assign4120_e4786);
        let assign4120_e4790: f64 = (var_inv_lnfin * p.p203);
        let assign4120_e4791: f64 = (assign4120_e4787 + assign4120_e4790);
        let assign4120_e4794: f64 = (var_inv_w * p.p204);
        let assign4120_e4795: f64 = (assign4120_e4791 + assign4120_e4794);
        let assign4120_e4798: f64 = (var_inv_wl * p.p205);
        let assign4120_e4799: f64 = (assign4120_e4795 + assign4120_e4798);
        var_cdsc_i = assign4120_e4799;

        let assign4130_e4803: f64 = (var_inv_l * p.p207);
        let assign4130_e4804: f64 = (p.p206 + assign4130_e4803);
        let assign4130_e4807: f64 = (var_inv_nfin * p.p208);
        let assign4130_e4808: f64 = (assign4130_e4804 + assign4130_e4807);
        let assign4130_e4811: f64 = (var_inv_lnfin * p.p209);
        let assign4130_e4812: f64 = (assign4130_e4808 + assign4130_e4811);
        let assign4130_e4815: f64 = (var_inv_w * p.p210);
        let assign4130_e4816: f64 = (assign4130_e4812 + assign4130_e4815);
        let assign4130_e4819: f64 = (var_inv_wl * p.p211);
        let assign4130_e4820: f64 = (assign4130_e4816 + assign4130_e4819);
        var_cdscd_i = assign4130_e4820;

        let assign4140_e4824: f64 = (var_inv_l * p.p219);
        let assign4140_e4825: f64 = (p.p218 + assign4140_e4824);
        let assign4140_e4828: f64 = (var_inv_nfin * p.p220);
        let assign4140_e4829: f64 = (assign4140_e4825 + assign4140_e4828);
        let assign4140_e4832: f64 = (var_inv_lnfin * p.p221);
        let assign4140_e4833: f64 = (assign4140_e4829 + assign4140_e4832);
        let assign4140_e4836: f64 = (var_inv_w * p.p222);
        let assign4140_e4837: f64 = (assign4140_e4833 + assign4140_e4836);
        let assign4140_e4840: f64 = (var_inv_wl * p.p223);
        let assign4140_e4841: f64 = (assign4140_e4837 + assign4140_e4840);
        var_dvt0_i = assign4140_e4841;

        let assign4150_e4845: f64 = (var_inv_l * p.p225);
        let assign4150_e4846: f64 = (p.p224 + assign4150_e4845);
        let assign4150_e4849: f64 = (var_inv_nfin * p.p226);
        let assign4150_e4850: f64 = (assign4150_e4846 + assign4150_e4849);
        let assign4150_e4853: f64 = (var_inv_lnfin * p.p227);
        let assign4150_e4854: f64 = (assign4150_e4850 + assign4150_e4853);
        let assign4150_e4857: f64 = (var_inv_w * p.p228);
        let assign4150_e4858: f64 = (assign4150_e4854 + assign4150_e4857);
        let assign4150_e4861: f64 = (var_inv_wl * p.p229);
        let assign4150_e4862: f64 = (assign4150_e4858 + assign4150_e4861);
        var_dvt1_i = assign4150_e4862;

        let assign4160_e4866: f64 = (var_inv_l * p.p231);
        let assign4160_e4867: f64 = (p.p230 + assign4160_e4866);
        let assign4160_e4870: f64 = (var_inv_nfin * p.p232);
        let assign4160_e4871: f64 = (assign4160_e4867 + assign4160_e4870);
        let assign4160_e4874: f64 = (var_inv_lnfin * p.p233);
        let assign4160_e4875: f64 = (assign4160_e4871 + assign4160_e4874);
        let assign4160_e4878: f64 = (var_inv_w * p.p234);
        let assign4160_e4879: f64 = (assign4160_e4875 + assign4160_e4878);
        let assign4160_e4882: f64 = (var_inv_wl * p.p235);
        let assign4160_e4883: f64 = (assign4160_e4879 + assign4160_e4882);
        var_dvt1ss_i = assign4160_e4883;

        let assign4170_e4887: f64 = (var_inv_l * p.p237);
        let assign4170_e4888: f64 = (p.p236 + assign4170_e4887);
        let assign4170_e4891: f64 = (var_inv_nfin * p.p238);
        let assign4170_e4892: f64 = (assign4170_e4888 + assign4170_e4891);
        let assign4170_e4895: f64 = (var_inv_lnfin * p.p239);
        let assign4170_e4896: f64 = (assign4170_e4892 + assign4170_e4895);
        let assign4170_e4899: f64 = (var_inv_w * p.p240);
        let assign4170_e4900: f64 = (assign4170_e4896 + assign4170_e4899);
        let assign4170_e4903: f64 = (var_inv_wl * p.p241);
        let assign4170_e4904: f64 = (assign4170_e4900 + assign4170_e4903);
        var_phin_i = assign4170_e4904;

        let assign4180_e4908: f64 = (var_inv_l * p.p243);
        let assign4180_e4909: f64 = (p.p242 + assign4180_e4908);
        let assign4180_e4912: f64 = (var_inv_nfin * p.p244);
        let assign4180_e4913: f64 = (assign4180_e4909 + assign4180_e4912);
        let assign4180_e4916: f64 = (var_inv_lnfin * p.p245);
        let assign4180_e4917: f64 = (assign4180_e4913 + assign4180_e4916);
        let assign4180_e4920: f64 = (var_inv_w * p.p246);
        let assign4180_e4921: f64 = (assign4180_e4917 + assign4180_e4920);
        let assign4180_e4924: f64 = (var_inv_wl * p.p247);
        let assign4180_e4925: f64 = (assign4180_e4921 + assign4180_e4924);
        var_eta0_i = assign4180_e4925;
        var_eta0_i_dn0 = 0.0;
        var_eta0_i_dn2 = 0.0;
        var_eta0_i_dn3 = 0.0;
        var_eta0_i_dn4 = 0.0;
        var_eta0_i_dn5 = 0.0;
        var_eta0_i_dn6 = 0.0;
        var_eta0_i_dn7 = 0.0;
        var_eta0_i_dn8 = 0.0;
        var_eta0_i_dn9 = 0.0;
        var_eta0_i_dn10 = 0.0;
        var_eta0_i_dn11 = 0.0;
        var_eta0_i_dn13 = 0.0;
        var_eta0_i_dn14 = 0.0;

        let assign4190_e4929: f64 = (var_inv_l * p.p249);
        let assign4190_e4930: f64 = (p.p248 + assign4190_e4929);
        let assign4190_e4933: f64 = (var_inv_nfin * p.p250);
        let assign4190_e4934: f64 = (assign4190_e4930 + assign4190_e4933);
        let assign4190_e4937: f64 = (var_inv_lnfin * p.p251);
        let assign4190_e4938: f64 = (assign4190_e4934 + assign4190_e4937);
        let assign4190_e4941: f64 = (var_inv_w * p.p252);
        let assign4190_e4942: f64 = (assign4190_e4938 + assign4190_e4941);
        let assign4190_e4945: f64 = (var_inv_wl * p.p253);
        let assign4190_e4946: f64 = (assign4190_e4942 + assign4190_e4945);
        var_eta1_i = assign4190_e4946;

        let assign4200_e4950: f64 = (var_inv_l * p.p267);
        let assign4200_e4951: f64 = (p.p266 + assign4200_e4950);
        let assign4200_e4954: f64 = (var_inv_nfin * p.p268);
        let assign4200_e4955: f64 = (assign4200_e4951 + assign4200_e4954);
        let assign4200_e4958: f64 = (var_inv_lnfin * p.p269);
        let assign4200_e4959: f64 = (assign4200_e4955 + assign4200_e4958);
        let assign4200_e4962: f64 = (var_inv_w * p.p270);
        let assign4200_e4963: f64 = (assign4200_e4959 + assign4200_e4962);
        let assign4200_e4966: f64 = (var_inv_wl * p.p271);
        let assign4200_e4967: f64 = (assign4200_e4963 + assign4200_e4966);
        var_dsub_i = assign4200_e4967;

        let assign4210_e4971: f64 = (var_inv_l * p.p273);
        let assign4210_e4972: f64 = (p.p272 + assign4210_e4971);
        let assign4210_e4975: f64 = (var_inv_nfin * p.p274);
        let assign4210_e4976: f64 = (assign4210_e4972 + assign4210_e4975);
        let assign4210_e4979: f64 = (var_inv_lnfin * p.p275);
        let assign4210_e4980: f64 = (assign4210_e4976 + assign4210_e4979);
        let assign4210_e4983: f64 = (var_inv_w * p.p276);
        let assign4210_e4984: f64 = (assign4210_e4980 + assign4210_e4983);
        let assign4210_e4987: f64 = (var_inv_wl * p.p277);
        let assign4210_e4988: f64 = (assign4210_e4984 + assign4210_e4987);
        var_k1rsce_i = assign4210_e4988;

        let assign4220_e4992: f64 = (var_inv_l * p.p279);
        let assign4220_e4993: f64 = (p.p278 + assign4220_e4992);
        let assign4220_e4996: f64 = (var_inv_nfin * p.p280);
        let assign4220_e4997: f64 = (assign4220_e4993 + assign4220_e4996);
        let assign4220_e5000: f64 = (var_inv_lnfin * p.p281);
        let assign4220_e5001: f64 = (assign4220_e4997 + assign4220_e5000);
        let assign4220_e5004: f64 = (var_inv_w * p.p282);
        let assign4220_e5005: f64 = (assign4220_e5001 + assign4220_e5004);
        let assign4220_e5008: f64 = (var_inv_wl * p.p283);
        let assign4220_e5009: f64 = (assign4220_e5005 + assign4220_e5008);
        var_lpe0_i = assign4220_e5009;

        let assign4230_e5013: f64 = (var_inv_l * p.p285);
        let assign4230_e5014: f64 = (p.p284 + assign4230_e5013);
        let assign4230_e5017: f64 = (var_inv_nfin * p.p286);
        let assign4230_e5018: f64 = (assign4230_e5014 + assign4230_e5017);
        let assign4230_e5021: f64 = (var_inv_lnfin * p.p287);
        let assign4230_e5022: f64 = (assign4230_e5018 + assign4230_e5021);
        let assign4230_e5025: f64 = (var_inv_w * p.p288);
        let assign4230_e5026: f64 = (assign4230_e5022 + assign4230_e5025);
        let assign4230_e5029: f64 = (var_inv_wl * p.p289);
        let assign4230_e5030: f64 = (assign4230_e5026 + assign4230_e5029);
        var_dvtshift_i = assign4230_e5030;

        let assign4240_e5034: f64 = (var_inv_l * p.p297);
        let assign4240_e5035: f64 = (p.p296 + assign4240_e5034);
        let assign4240_e5038: f64 = (var_inv_nfin * p.p298);
        let assign4240_e5039: f64 = (assign4240_e5035 + assign4240_e5038);
        let assign4240_e5042: f64 = (var_inv_lnfin * p.p299);
        let assign4240_e5043: f64 = (assign4240_e5039 + assign4240_e5042);
        let assign4240_e5046: f64 = (var_inv_w * p.p300);
        let assign4240_e5047: f64 = (assign4240_e5043 + assign4240_e5046);
        let assign4240_e5050: f64 = (var_inv_wl * p.p301);
        let assign4240_e5051: f64 = (assign4240_e5047 + assign4240_e5050);
        var_k0_i = assign4240_e5051;

        let assign4250_e5055: f64 = (var_inv_l * p.p303);
        let assign4250_e5056: f64 = (p.p302 + assign4250_e5055);
        let assign4250_e5059: f64 = (var_inv_nfin * p.p304);
        let assign4250_e5060: f64 = (assign4250_e5056 + assign4250_e5059);
        let assign4250_e5063: f64 = (var_inv_lnfin * p.p305);
        let assign4250_e5064: f64 = (assign4250_e5060 + assign4250_e5063);
        let assign4250_e5067: f64 = (var_inv_w * p.p306);
        let assign4250_e5068: f64 = (assign4250_e5064 + assign4250_e5067);
        let assign4250_e5071: f64 = (var_inv_wl * p.p307);
        let assign4250_e5072: f64 = (assign4250_e5068 + assign4250_e5071);
        var_k01_i = assign4250_e5072;

        let assign4260_e5076: f64 = (var_inv_l * p.p309);
        let assign4260_e5077: f64 = (p.p308 + assign4260_e5076);
        let assign4260_e5080: f64 = (var_inv_nfin * p.p310);
        let assign4260_e5081: f64 = (assign4260_e5077 + assign4260_e5080);
        let assign4260_e5084: f64 = (var_inv_lnfin * p.p311);
        let assign4260_e5085: f64 = (assign4260_e5081 + assign4260_e5084);
        let assign4260_e5088: f64 = (var_inv_w * p.p312);
        let assign4260_e5089: f64 = (assign4260_e5085 + assign4260_e5088);
        let assign4260_e5092: f64 = (var_inv_wl * p.p313);
        let assign4260_e5093: f64 = (assign4260_e5089 + assign4260_e5092);
        var_k0si_i = assign4260_e5093;

        let assign4270_e5097: f64 = (var_inv_l * p.p315);
        let assign4270_e5098: f64 = (p.p314 + assign4270_e5097);
        let assign4270_e5101: f64 = (var_inv_nfin * p.p316);
        let assign4270_e5102: f64 = (assign4270_e5098 + assign4270_e5101);
        let assign4270_e5105: f64 = (var_inv_lnfin * p.p317);
        let assign4270_e5106: f64 = (assign4270_e5102 + assign4270_e5105);
        let assign4270_e5109: f64 = (var_inv_w * p.p318);
        let assign4270_e5110: f64 = (assign4270_e5106 + assign4270_e5109);
        let assign4270_e5113: f64 = (var_inv_wl * p.p319);
        let assign4270_e5114: f64 = (assign4270_e5110 + assign4270_e5113);
        var_k0si1_i = assign4270_e5114;

        let assign4280_e5118: f64 = (var_inv_l * p.p321);
        let assign4280_e5119: f64 = (p.p320 + assign4280_e5118);
        let assign4280_e5122: f64 = (var_inv_nfin * p.p322);
        let assign4280_e5123: f64 = (assign4280_e5119 + assign4280_e5122);
        let assign4280_e5126: f64 = (var_inv_lnfin * p.p323);
        let assign4280_e5127: f64 = (assign4280_e5123 + assign4280_e5126);
        let assign4280_e5130: f64 = (var_inv_w * p.p324);
        let assign4280_e5131: f64 = (assign4280_e5127 + assign4280_e5130);
        let assign4280_e5134: f64 = (var_inv_wl * p.p325);
        let assign4280_e5135: f64 = (assign4280_e5131 + assign4280_e5134);
        var_k2si_i = assign4280_e5135;

        let assign4290_e5139: f64 = (var_inv_l * p.p327);
        let assign4290_e5140: f64 = (p.p326 + assign4290_e5139);
        let assign4290_e5143: f64 = (var_inv_nfin * p.p328);
        let assign4290_e5144: f64 = (assign4290_e5140 + assign4290_e5143);
        let assign4290_e5147: f64 = (var_inv_lnfin * p.p329);
        let assign4290_e5148: f64 = (assign4290_e5144 + assign4290_e5147);
        let assign4290_e5151: f64 = (var_inv_w * p.p330);
        let assign4290_e5152: f64 = (assign4290_e5148 + assign4290_e5151);
        let assign4290_e5155: f64 = (var_inv_wl * p.p331);
        let assign4290_e5156: f64 = (assign4290_e5152 + assign4290_e5155);
        var_k2si1_i = assign4290_e5156;

        let assign4300_e5160: f64 = (var_inv_l * p.p333);
        let assign4300_e5161: f64 = (p.p332 + assign4300_e5160);
        let assign4300_e5164: f64 = (var_inv_nfin * p.p334);
        let assign4300_e5165: f64 = (assign4300_e5161 + assign4300_e5164);
        let assign4300_e5168: f64 = (var_inv_lnfin * p.p335);
        let assign4300_e5169: f64 = (assign4300_e5165 + assign4300_e5168);
        let assign4300_e5172: f64 = (var_inv_w * p.p336);
        let assign4300_e5173: f64 = (assign4300_e5169 + assign4300_e5172);
        let assign4300_e5176: f64 = (var_inv_wl * p.p337);
        let assign4300_e5177: f64 = (assign4300_e5173 + assign4300_e5176);
        var_k0sisat_i = assign4300_e5177;

        let assign4310_e5181: f64 = (var_inv_l * p.p339);
        let assign4310_e5182: f64 = (p.p338 + assign4310_e5181);
        let assign4310_e5185: f64 = (var_inv_nfin * p.p340);
        let assign4310_e5186: f64 = (assign4310_e5182 + assign4310_e5185);
        let assign4310_e5189: f64 = (var_inv_lnfin * p.p341);
        let assign4310_e5190: f64 = (assign4310_e5186 + assign4310_e5189);
        let assign4310_e5193: f64 = (var_inv_w * p.p342);
        let assign4310_e5194: f64 = (assign4310_e5190 + assign4310_e5193);
        let assign4310_e5197: f64 = (var_inv_wl * p.p343);
        let assign4310_e5198: f64 = (assign4310_e5194 + assign4310_e5197);
        var_k0sisat1_i = assign4310_e5198;

        let assign4320_e5202: f64 = (var_inv_l * p.p345);
        let assign4320_e5203: f64 = (p.p344 + assign4320_e5202);
        let assign4320_e5206: f64 = (var_inv_nfin * p.p346);
        let assign4320_e5207: f64 = (assign4320_e5203 + assign4320_e5206);
        let assign4320_e5210: f64 = (var_inv_lnfin * p.p347);
        let assign4320_e5211: f64 = (assign4320_e5207 + assign4320_e5210);
        let assign4320_e5214: f64 = (var_inv_w * p.p348);
        let assign4320_e5215: f64 = (assign4320_e5211 + assign4320_e5214);
        let assign4320_e5218: f64 = (var_inv_wl * p.p349);
        let assign4320_e5219: f64 = (assign4320_e5215 + assign4320_e5218);
        var_k2sisat_i = assign4320_e5219;

        let assign4330_e5223: f64 = (var_inv_l * p.p351);
        let assign4330_e5224: f64 = (p.p350 + assign4330_e5223);
        let assign4330_e5227: f64 = (var_inv_nfin * p.p352);
        let assign4330_e5228: f64 = (assign4330_e5224 + assign4330_e5227);
        let assign4330_e5231: f64 = (var_inv_lnfin * p.p353);
        let assign4330_e5232: f64 = (assign4330_e5228 + assign4330_e5231);
        let assign4330_e5235: f64 = (var_inv_w * p.p354);
        let assign4330_e5236: f64 = (assign4330_e5232 + assign4330_e5235);
        let assign4330_e5239: f64 = (var_inv_wl * p.p355);
        let assign4330_e5240: f64 = (assign4330_e5236 + assign4330_e5239);
        var_k2sisat1_i = assign4330_e5240;

        let assign4340_e5244: f64 = (var_inv_l * p.p404);
        let assign4340_e5245: f64 = (p.p403 + assign4340_e5244);
        let assign4340_e5248: f64 = (var_inv_nfin * p.p405);
        let assign4340_e5249: f64 = (assign4340_e5245 + assign4340_e5248);
        let assign4340_e5252: f64 = (var_inv_lnfin * p.p406);
        let assign4340_e5253: f64 = (assign4340_e5249 + assign4340_e5252);
        let assign4340_e5256: f64 = (var_inv_w * p.p407);
        let assign4340_e5257: f64 = (assign4340_e5253 + assign4340_e5256);
        let assign4340_e5260: f64 = (var_inv_wl * p.p408);
        let assign4340_e5261: f64 = (assign4340_e5257 + assign4340_e5260);
        var_qmfactor_i = assign4340_e5261;

        let assign4350_e5265: f64 = (var_inv_l * p.p410);
        let assign4350_e5266: f64 = (p.p409 + assign4350_e5265);
        let assign4350_e5269: f64 = (var_inv_nfin * p.p411);
        let assign4350_e5270: f64 = (assign4350_e5266 + assign4350_e5269);
        let assign4350_e5273: f64 = (var_inv_lnfin * p.p412);
        let assign4350_e5274: f64 = (assign4350_e5270 + assign4350_e5273);
        let assign4350_e5277: f64 = (var_inv_w * p.p413);
        let assign4350_e5278: f64 = (assign4350_e5274 + assign4350_e5277);
        let assign4350_e5281: f64 = (var_inv_wl * p.p414);
        let assign4350_e5282: f64 = (assign4350_e5278 + assign4350_e5281);
        var_qmtcencv_i = assign4350_e5282;

        let assign4360_e5286: f64 = (var_inv_l * p.p416);
        let assign4360_e5287: f64 = (p.p415 + assign4360_e5286);
        let assign4360_e5290: f64 = (var_inv_nfin * p.p417);
        let assign4360_e5291: f64 = (assign4360_e5287 + assign4360_e5290);
        let assign4360_e5294: f64 = (var_inv_lnfin * p.p418);
        let assign4360_e5295: f64 = (assign4360_e5291 + assign4360_e5294);
        let assign4360_e5298: f64 = (var_inv_w * p.p419);
        let assign4360_e5299: f64 = (assign4360_e5295 + assign4360_e5298);
        let assign4360_e5302: f64 = (var_inv_wl * p.p420);
        let assign4360_e5303: f64 = (assign4360_e5299 + assign4360_e5302);
        var_qmtcencva_i = assign4360_e5303;

        let assign4370_e5307: f64 = (var_inv_l * p.p422);
        let assign4370_e5308: f64 = (p.p421 + assign4370_e5307);
        let assign4370_e5311: f64 = (var_inv_nfin * p.p423);
        let assign4370_e5312: f64 = (assign4370_e5308 + assign4370_e5311);
        let assign4370_e5315: f64 = (var_inv_lnfin * p.p424);
        let assign4370_e5316: f64 = (assign4370_e5312 + assign4370_e5315);
        let assign4370_e5319: f64 = (var_inv_w * p.p425);
        let assign4370_e5320: f64 = (assign4370_e5316 + assign4370_e5319);
        let assign4370_e5323: f64 = (var_inv_wl * p.p426);
        let assign4370_e5324: f64 = (assign4370_e5320 + assign4370_e5323);
        var_pqm_i = assign4370_e5324;
        var_pqm_i_dn0 = 0.0;
        var_pqm_i_dn2 = 0.0;
        var_pqm_i_dn3 = 0.0;
        var_pqm_i_dn4 = 0.0;
        var_pqm_i_dn5 = 0.0;
        var_pqm_i_dn6 = 0.0;
        var_pqm_i_dn7 = 0.0;
        var_pqm_i_dn8 = 0.0;
        var_pqm_i_dn9 = 0.0;
        var_pqm_i_dn10 = 0.0;
        var_pqm_i_dn11 = 0.0;
        var_pqm_i_dn13 = 0.0;
        var_pqm_i_dn14 = 0.0;

        let assign4380_e5328: f64 = (var_inv_l * p.p456);
        let assign4380_e5329: f64 = (p.p455 + assign4380_e5328);
        let assign4380_e5332: f64 = (var_inv_nfin * p.p457);
        let assign4380_e5333: f64 = (assign4380_e5329 + assign4380_e5332);
        let assign4380_e5336: f64 = (var_inv_lnfin * p.p458);
        let assign4380_e5337: f64 = (assign4380_e5333 + assign4380_e5336);
        let assign4380_e5340: f64 = (var_inv_w * p.p459);
        let assign4380_e5341: f64 = (assign4380_e5337 + assign4380_e5340);
        let assign4380_e5344: f64 = (var_inv_wl * p.p460);
        let assign4380_e5345: f64 = (assign4380_e5341 + assign4380_e5344);
        var_vsat_i = assign4380_e5345;
        var_vsat_i_dn0 = 0.0;
        var_vsat_i_dn2 = 0.0;
        var_vsat_i_dn3 = 0.0;
        var_vsat_i_dn4 = 0.0;
        var_vsat_i_dn5 = 0.0;
        var_vsat_i_dn6 = 0.0;
        var_vsat_i_dn7 = 0.0;
        var_vsat_i_dn8 = 0.0;
        var_vsat_i_dn9 = 0.0;
        var_vsat_i_dn10 = 0.0;
        var_vsat_i_dn11 = 0.0;
        var_vsat_i_dn13 = 0.0;
        var_vsat_i_dn14 = 0.0;

        let assign4390_e5349: f64 = (var_inv_l * p.p468);
        let assign4390_e5350: f64 = (p.p467 + assign4390_e5349);
        let assign4390_e5353: f64 = (var_inv_nfin * p.p469);
        let assign4390_e5354: f64 = (assign4390_e5350 + assign4390_e5353);
        let assign4390_e5357: f64 = (var_inv_lnfin * p.p470);
        let assign4390_e5358: f64 = (assign4390_e5354 + assign4390_e5357);
        let assign4390_e5361: f64 = (var_inv_w * p.p471);
        let assign4390_e5362: f64 = (assign4390_e5358 + assign4390_e5361);
        let assign4390_e5365: f64 = (var_inv_wl * p.p472);
        let assign4390_e5366: f64 = (assign4390_e5362 + assign4390_e5365);
        var_vsat1_i = assign4390_e5366;
        var_vsat1_i_dn0 = 0.0;
        var_vsat1_i_dn2 = 0.0;
        var_vsat1_i_dn3 = 0.0;
        var_vsat1_i_dn4 = 0.0;
        var_vsat1_i_dn5 = 0.0;
        var_vsat1_i_dn6 = 0.0;
        var_vsat1_i_dn7 = 0.0;
        var_vsat1_i_dn8 = 0.0;
        var_vsat1_i_dn9 = 0.0;
        var_vsat1_i_dn10 = 0.0;
        var_vsat1_i_dn11 = 0.0;
        var_vsat1_i_dn13 = 0.0;
        var_vsat1_i_dn14 = 0.0;

        *var_cdsc_i_slot = var_cdsc_i;
        *var_cdscd_i_slot = var_cdscd_i;
        *var_cit_i_slot = var_cit_i;
        *var_cox_acc_slot = var_cox_acc;
        *var_cox_acc_dn0_slot = var_cox_acc_dn0;
        *var_cox_acc_dn10_slot = var_cox_acc_dn10;
        *var_cox_acc_dn11_slot = var_cox_acc_dn11;
        *var_cox_acc_dn13_slot = var_cox_acc_dn13;
        *var_cox_acc_dn14_slot = var_cox_acc_dn14;
        *var_cox_acc_dn2_slot = var_cox_acc_dn2;
        *var_cox_acc_dn3_slot = var_cox_acc_dn3;
        *var_cox_acc_dn4_slot = var_cox_acc_dn4;
        *var_cox_acc_dn5_slot = var_cox_acc_dn5;
        *var_cox_acc_dn6_slot = var_cox_acc_dn6;
        *var_cox_acc_dn7_slot = var_cox_acc_dn7;
        *var_cox_acc_dn8_slot = var_cox_acc_dn8;
        *var_cox_acc_dn9_slot = var_cox_acc_dn9;
        *var_deltaprsd_v_slot = var_deltaprsd_v;
        *var_dsub_i_slot = var_dsub_i;
        *var_dvt0_i_slot = var_dvt0_i;
        *var_dvt1_i_slot = var_dvt1_i;
        *var_dvt1ss_i_slot = var_dvt1ss_i;
        *var_dvtshift_i_slot = var_dvtshift_i;
        *var_eta0_i_slot = var_eta0_i;
        *var_eta0_i_dn0_slot = var_eta0_i_dn0;
        *var_eta0_i_dn10_slot = var_eta0_i_dn10;
        *var_eta0_i_dn11_slot = var_eta0_i_dn11;
        *var_eta0_i_dn13_slot = var_eta0_i_dn13;
        *var_eta0_i_dn14_slot = var_eta0_i_dn14;
        *var_eta0_i_dn2_slot = var_eta0_i_dn2;
        *var_eta0_i_dn3_slot = var_eta0_i_dn3;
        *var_eta0_i_dn4_slot = var_eta0_i_dn4;
        *var_eta0_i_dn5_slot = var_eta0_i_dn5;
        *var_eta0_i_dn6_slot = var_eta0_i_dn6;
        *var_eta0_i_dn7_slot = var_eta0_i_dn7;
        *var_eta0_i_dn8_slot = var_eta0_i_dn8;
        *var_eta0_i_dn9_slot = var_eta0_i_dn9;
        *var_eta1_i_slot = var_eta1_i;
        *var_guard36_slot = var_guard36;
        *var_guard37_slot = var_guard37;
        *var_guard38_slot = var_guard38;
        *var_guard39_slot = var_guard39;
        *var_guard40_slot = var_guard40;
        *var_k01_i_slot = var_k01_i;
        *var_k0_i_slot = var_k0_i;
        *var_k0si1_i_slot = var_k0si1_i;
        *var_k0si_i_slot = var_k0si_i;
        *var_k0sisat1_i_slot = var_k0sisat1_i;
        *var_k0sisat_i_slot = var_k0sisat_i;
        *var_k1rsce_i_slot = var_k1rsce_i;
        *var_k2si1_i_slot = var_k2si1_i;
        *var_k2si_i_slot = var_k2si_i;
        *var_k2sisat1_i_slot = var_k2sisat1_i;
        *var_k2sisat_i_slot = var_k2sisat_i;
        *var_lpe0_i_slot = var_lpe0_i;
        *var_phig_i_slot = var_phig_i;
        *var_phig_i_dn0_slot = var_phig_i_dn0;
        *var_phig_i_dn10_slot = var_phig_i_dn10;
        *var_phig_i_dn11_slot = var_phig_i_dn11;
        *var_phig_i_dn13_slot = var_phig_i_dn13;
        *var_phig_i_dn14_slot = var_phig_i_dn14;
        *var_phig_i_dn2_slot = var_phig_i_dn2;
        *var_phig_i_dn3_slot = var_phig_i_dn3;
        *var_phig_i_dn4_slot = var_phig_i_dn4;
        *var_phig_i_dn5_slot = var_phig_i_dn5;
        *var_phig_i_dn6_slot = var_phig_i_dn6;
        *var_phig_i_dn7_slot = var_phig_i_dn7;
        *var_phig_i_dn8_slot = var_phig_i_dn8;
        *var_phig_i_dn9_slot = var_phig_i_dn9;
        *var_phin_i_slot = var_phin_i;
        *var_pqm_i_slot = var_pqm_i;
        *var_pqm_i_dn0_slot = var_pqm_i_dn0;
        *var_pqm_i_dn10_slot = var_pqm_i_dn10;
        *var_pqm_i_dn11_slot = var_pqm_i_dn11;
        *var_pqm_i_dn13_slot = var_pqm_i_dn13;
        *var_pqm_i_dn14_slot = var_pqm_i_dn14;
        *var_pqm_i_dn2_slot = var_pqm_i_dn2;
        *var_pqm_i_dn3_slot = var_pqm_i_dn3;
        *var_pqm_i_dn4_slot = var_pqm_i_dn4;
        *var_pqm_i_dn5_slot = var_pqm_i_dn5;
        *var_pqm_i_dn6_slot = var_pqm_i_dn6;
        *var_pqm_i_dn7_slot = var_pqm_i_dn7;
        *var_pqm_i_dn8_slot = var_pqm_i_dn8;
        *var_pqm_i_dn9_slot = var_pqm_i_dn9;
        *var_qmfactor_i_slot = var_qmfactor_i;
        *var_qmtcencv_i_slot = var_qmtcencv_i;
        *var_qmtcencva_i_slot = var_qmtcencva_i;
        *var_vsat1_i_slot = var_vsat1_i;
        *var_vsat1_i_dn0_slot = var_vsat1_i_dn0;
        *var_vsat1_i_dn10_slot = var_vsat1_i_dn10;
        *var_vsat1_i_dn11_slot = var_vsat1_i_dn11;
        *var_vsat1_i_dn13_slot = var_vsat1_i_dn13;
        *var_vsat1_i_dn14_slot = var_vsat1_i_dn14;
        *var_vsat1_i_dn2_slot = var_vsat1_i_dn2;
        *var_vsat1_i_dn3_slot = var_vsat1_i_dn3;
        *var_vsat1_i_dn4_slot = var_vsat1_i_dn4;
        *var_vsat1_i_dn5_slot = var_vsat1_i_dn5;
        *var_vsat1_i_dn6_slot = var_vsat1_i_dn6;
        *var_vsat1_i_dn7_slot = var_vsat1_i_dn7;
        *var_vsat1_i_dn8_slot = var_vsat1_i_dn8;
        *var_vsat1_i_dn9_slot = var_vsat1_i_dn9;
        *var_vsat_i_slot = var_vsat_i;
        *var_vsat_i_dn0_slot = var_vsat_i_dn0;
        *var_vsat_i_dn10_slot = var_vsat_i_dn10;
        *var_vsat_i_dn11_slot = var_vsat_i_dn11;
        *var_vsat_i_dn13_slot = var_vsat_i_dn13;
        *var_vsat_i_dn14_slot = var_vsat_i_dn14;
        *var_vsat_i_dn2_slot = var_vsat_i_dn2;
        *var_vsat_i_dn3_slot = var_vsat_i_dn3;
        *var_vsat_i_dn4_slot = var_vsat_i_dn4;
        *var_vsat_i_dn5_slot = var_vsat_i_dn5;
        *var_vsat_i_dn6_slot = var_vsat_i_dn6;
        *var_vsat_i_dn7_slot = var_vsat_i_dn7;
        *var_vsat_i_dn8_slot = var_vsat_i_dn8;
        *var_vsat_i_dn9_slot = var_vsat_i_dn9;
        *var_weff0_slot = var_weff0;
        *var_weffcv0_slot = var_weffcv0;
        *var_weffcv_acc_slot = var_weffcv_acc;
    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        var_inv_l: f64,
        var_inv_lnfin: f64,
        var_inv_nfin: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_a11_i_slot: &mut f64,
        var_a1_i_slot: &mut f64,
        var_a21_i_slot: &mut f64,
        var_a2_i_slot: &mut f64,
        var_asat_i_slot: &mut f64,
        var_deltavsat_i_slot: &mut f64,
        var_deltavsatcv_i_slot: &mut f64,
        var_etamob_i_slot: &mut f64,
        var_eu_i_slot: &mut f64,
        var_eu_i_dn0_slot: &mut f64,
        var_eu_i_dn10_slot: &mut f64,
        var_eu_i_dn11_slot: &mut f64,
        var_eu_i_dn13_slot: &mut f64,
        var_eu_i_dn14_slot: &mut f64,
        var_eu_i_dn2_slot: &mut f64,
        var_eu_i_dn3_slot: &mut f64,
        var_eu_i_dn4_slot: &mut f64,
        var_eu_i_dn5_slot: &mut f64,
        var_eu_i_dn6_slot: &mut f64,
        var_eu_i_dn7_slot: &mut f64,
        var_eu_i_dn8_slot: &mut f64,
        var_eu_i_dn9_slot: &mut f64,
        var_ksativ_i_slot: &mut f64,
        var_mexp_i_slot: &mut f64,
        var_mexp_i_dn0_slot: &mut f64,
        var_mexp_i_dn10_slot: &mut f64,
        var_mexp_i_dn11_slot: &mut f64,
        var_mexp_i_dn13_slot: &mut f64,
        var_mexp_i_dn14_slot: &mut f64,
        var_mexp_i_dn2_slot: &mut f64,
        var_mexp_i_dn3_slot: &mut f64,
        var_mexp_i_dn4_slot: &mut f64,
        var_mexp_i_dn5_slot: &mut f64,
        var_mexp_i_dn6_slot: &mut f64,
        var_mexp_i_dn7_slot: &mut f64,
        var_mexp_i_dn8_slot: &mut f64,
        var_mexp_i_dn9_slot: &mut f64,
        var_pclm_i_slot: &mut f64,
        var_pclm_i_dn0_slot: &mut f64,
        var_pclm_i_dn10_slot: &mut f64,
        var_pclm_i_dn11_slot: &mut f64,
        var_pclm_i_dn13_slot: &mut f64,
        var_pclm_i_dn14_slot: &mut f64,
        var_pclm_i_dn2_slot: &mut f64,
        var_pclm_i_dn3_slot: &mut f64,
        var_pclm_i_dn4_slot: &mut f64,
        var_pclm_i_dn5_slot: &mut f64,
        var_pclm_i_dn6_slot: &mut f64,
        var_pclm_i_dn7_slot: &mut f64,
        var_pclm_i_dn8_slot: &mut f64,
        var_pclm_i_dn9_slot: &mut f64,
        var_pclmcv_i_slot: &mut f64,
        var_pclmg_i_slot: &mut f64,
        var_psat_i_slot: &mut f64,
        var_psat_i_dn0_slot: &mut f64,
        var_psat_i_dn10_slot: &mut f64,
        var_psat_i_dn11_slot: &mut f64,
        var_psat_i_dn13_slot: &mut f64,
        var_psat_i_dn14_slot: &mut f64,
        var_psat_i_dn2_slot: &mut f64,
        var_psat_i_dn3_slot: &mut f64,
        var_psat_i_dn4_slot: &mut f64,
        var_psat_i_dn5_slot: &mut f64,
        var_psat_i_dn6_slot: &mut f64,
        var_psat_i_dn7_slot: &mut f64,
        var_psat_i_dn8_slot: &mut f64,
        var_psat_i_dn9_slot: &mut f64,
        var_psatcv_i_slot: &mut f64,
        var_psatcv_i_dn0_slot: &mut f64,
        var_psatcv_i_dn10_slot: &mut f64,
        var_psatcv_i_dn11_slot: &mut f64,
        var_psatcv_i_dn13_slot: &mut f64,
        var_psatcv_i_dn14_slot: &mut f64,
        var_psatcv_i_dn2_slot: &mut f64,
        var_psatcv_i_dn3_slot: &mut f64,
        var_psatcv_i_dn4_slot: &mut f64,
        var_psatcv_i_dn5_slot: &mut f64,
        var_psatcv_i_dn6_slot: &mut f64,
        var_psatcv_i_dn7_slot: &mut f64,
        var_psatcv_i_dn8_slot: &mut f64,
        var_psatcv_i_dn9_slot: &mut f64,
        var_ptwg_i_slot: &mut f64,
        var_ptwg_i_dn0_slot: &mut f64,
        var_ptwg_i_dn10_slot: &mut f64,
        var_ptwg_i_dn11_slot: &mut f64,
        var_ptwg_i_dn13_slot: &mut f64,
        var_ptwg_i_dn14_slot: &mut f64,
        var_ptwg_i_dn2_slot: &mut f64,
        var_ptwg_i_dn3_slot: &mut f64,
        var_ptwg_i_dn4_slot: &mut f64,
        var_ptwg_i_dn5_slot: &mut f64,
        var_ptwg_i_dn6_slot: &mut f64,
        var_ptwg_i_dn7_slot: &mut f64,
        var_ptwg_i_dn8_slot: &mut f64,
        var_ptwg_i_dn9_slot: &mut f64,
        var_rdsw_i_slot: &mut f64,
        var_rdsw_i_dn0_slot: &mut f64,
        var_rdsw_i_dn10_slot: &mut f64,
        var_rdsw_i_dn11_slot: &mut f64,
        var_rdsw_i_dn13_slot: &mut f64,
        var_rdsw_i_dn14_slot: &mut f64,
        var_rdsw_i_dn2_slot: &mut f64,
        var_rdsw_i_dn3_slot: &mut f64,
        var_rdsw_i_dn4_slot: &mut f64,
        var_rdsw_i_dn5_slot: &mut f64,
        var_rdsw_i_dn6_slot: &mut f64,
        var_rdsw_i_dn7_slot: &mut f64,
        var_rdsw_i_dn8_slot: &mut f64,
        var_rdsw_i_dn9_slot: &mut f64,
        var_rsw_i_slot: &mut f64,
        var_rsw_i_dn0_slot: &mut f64,
        var_rsw_i_dn10_slot: &mut f64,
        var_rsw_i_dn11_slot: &mut f64,
        var_rsw_i_dn13_slot: &mut f64,
        var_rsw_i_dn14_slot: &mut f64,
        var_rsw_i_dn2_slot: &mut f64,
        var_rsw_i_dn3_slot: &mut f64,
        var_rsw_i_dn4_slot: &mut f64,
        var_rsw_i_dn5_slot: &mut f64,
        var_rsw_i_dn6_slot: &mut f64,
        var_rsw_i_dn7_slot: &mut f64,
        var_rsw_i_dn8_slot: &mut f64,
        var_rsw_i_dn9_slot: &mut f64,
        var_u0_i_slot: &mut f64,
        var_u0_i_dn0_slot: &mut f64,
        var_u0_i_dn10_slot: &mut f64,
        var_u0_i_dn11_slot: &mut f64,
        var_u0_i_dn13_slot: &mut f64,
        var_u0_i_dn14_slot: &mut f64,
        var_u0_i_dn2_slot: &mut f64,
        var_u0_i_dn3_slot: &mut f64,
        var_u0_i_dn4_slot: &mut f64,
        var_u0_i_dn5_slot: &mut f64,
        var_u0_i_dn6_slot: &mut f64,
        var_u0_i_dn7_slot: &mut f64,
        var_u0_i_dn8_slot: &mut f64,
        var_u0_i_dn9_slot: &mut f64,
        var_ua_i_slot: &mut f64,
        var_ua_i_dn0_slot: &mut f64,
        var_ua_i_dn10_slot: &mut f64,
        var_ua_i_dn11_slot: &mut f64,
        var_ua_i_dn13_slot: &mut f64,
        var_ua_i_dn14_slot: &mut f64,
        var_ua_i_dn2_slot: &mut f64,
        var_ua_i_dn3_slot: &mut f64,
        var_ua_i_dn4_slot: &mut f64,
        var_ua_i_dn5_slot: &mut f64,
        var_ua_i_dn6_slot: &mut f64,
        var_ua_i_dn7_slot: &mut f64,
        var_ua_i_dn8_slot: &mut f64,
        var_ua_i_dn9_slot: &mut f64,
        var_ucs_i_slot: &mut f64,
        var_ud_i_slot: &mut f64,
        var_ud_i_dn0_slot: &mut f64,
        var_ud_i_dn10_slot: &mut f64,
        var_ud_i_dn11_slot: &mut f64,
        var_ud_i_dn13_slot: &mut f64,
        var_ud_i_dn14_slot: &mut f64,
        var_ud_i_dn2_slot: &mut f64,
        var_ud_i_dn3_slot: &mut f64,
        var_ud_i_dn4_slot: &mut f64,
        var_ud_i_dn5_slot: &mut f64,
        var_ud_i_dn6_slot: &mut f64,
        var_ud_i_dn7_slot: &mut f64,
        var_ud_i_dn8_slot: &mut f64,
        var_ud_i_dn9_slot: &mut f64,
        var_udd1_i_slot: &mut f64,
        var_udd_i_slot: &mut f64,
        var_uds1_i_slot: &mut f64,
        var_uds_i_slot: &mut f64,
        var_up_i_slot: &mut f64,
        var_vsatcv_i_slot: &mut f64,
        var_vsatcv_i_dn0_slot: &mut f64,
        var_vsatcv_i_dn10_slot: &mut f64,
        var_vsatcv_i_dn11_slot: &mut f64,
        var_vsatcv_i_dn13_slot: &mut f64,
        var_vsatcv_i_dn14_slot: &mut f64,
        var_vsatcv_i_dn2_slot: &mut f64,
        var_vsatcv_i_dn3_slot: &mut f64,
        var_vsatcv_i_dn4_slot: &mut f64,
        var_vsatcv_i_dn5_slot: &mut f64,
        var_vsatcv_i_dn6_slot: &mut f64,
        var_vsatcv_i_dn7_slot: &mut f64,
        var_vsatcv_i_dn8_slot: &mut f64,
        var_vsatcv_i_dn9_slot: &mut f64,
    ) {
        let mut var_a11_i: f64 = *var_a11_i_slot;
        let mut var_a1_i: f64 = *var_a1_i_slot;
        let mut var_a21_i: f64 = *var_a21_i_slot;
        let mut var_a2_i: f64 = *var_a2_i_slot;
        let mut var_asat_i: f64 = *var_asat_i_slot;
        let mut var_deltavsat_i: f64 = *var_deltavsat_i_slot;
        let mut var_deltavsatcv_i: f64 = *var_deltavsatcv_i_slot;
        let mut var_etamob_i: f64 = *var_etamob_i_slot;
        let mut var_eu_i: f64 = *var_eu_i_slot;
        let mut var_eu_i_dn0: f64 = *var_eu_i_dn0_slot;
        let mut var_eu_i_dn10: f64 = *var_eu_i_dn10_slot;
        let mut var_eu_i_dn11: f64 = *var_eu_i_dn11_slot;
        let mut var_eu_i_dn13: f64 = *var_eu_i_dn13_slot;
        let mut var_eu_i_dn14: f64 = *var_eu_i_dn14_slot;
        let mut var_eu_i_dn2: f64 = *var_eu_i_dn2_slot;
        let mut var_eu_i_dn3: f64 = *var_eu_i_dn3_slot;
        let mut var_eu_i_dn4: f64 = *var_eu_i_dn4_slot;
        let mut var_eu_i_dn5: f64 = *var_eu_i_dn5_slot;
        let mut var_eu_i_dn6: f64 = *var_eu_i_dn6_slot;
        let mut var_eu_i_dn7: f64 = *var_eu_i_dn7_slot;
        let mut var_eu_i_dn8: f64 = *var_eu_i_dn8_slot;
        let mut var_eu_i_dn9: f64 = *var_eu_i_dn9_slot;
        let mut var_ksativ_i: f64 = *var_ksativ_i_slot;
        let mut var_mexp_i: f64 = *var_mexp_i_slot;
        let mut var_mexp_i_dn0: f64 = *var_mexp_i_dn0_slot;
        let mut var_mexp_i_dn10: f64 = *var_mexp_i_dn10_slot;
        let mut var_mexp_i_dn11: f64 = *var_mexp_i_dn11_slot;
        let mut var_mexp_i_dn13: f64 = *var_mexp_i_dn13_slot;
        let mut var_mexp_i_dn14: f64 = *var_mexp_i_dn14_slot;
        let mut var_mexp_i_dn2: f64 = *var_mexp_i_dn2_slot;
        let mut var_mexp_i_dn3: f64 = *var_mexp_i_dn3_slot;
        let mut var_mexp_i_dn4: f64 = *var_mexp_i_dn4_slot;
        let mut var_mexp_i_dn5: f64 = *var_mexp_i_dn5_slot;
        let mut var_mexp_i_dn6: f64 = *var_mexp_i_dn6_slot;
        let mut var_mexp_i_dn7: f64 = *var_mexp_i_dn7_slot;
        let mut var_mexp_i_dn8: f64 = *var_mexp_i_dn8_slot;
        let mut var_mexp_i_dn9: f64 = *var_mexp_i_dn9_slot;
        let mut var_pclm_i: f64 = *var_pclm_i_slot;
        let mut var_pclm_i_dn0: f64 = *var_pclm_i_dn0_slot;
        let mut var_pclm_i_dn10: f64 = *var_pclm_i_dn10_slot;
        let mut var_pclm_i_dn11: f64 = *var_pclm_i_dn11_slot;
        let mut var_pclm_i_dn13: f64 = *var_pclm_i_dn13_slot;
        let mut var_pclm_i_dn14: f64 = *var_pclm_i_dn14_slot;
        let mut var_pclm_i_dn2: f64 = *var_pclm_i_dn2_slot;
        let mut var_pclm_i_dn3: f64 = *var_pclm_i_dn3_slot;
        let mut var_pclm_i_dn4: f64 = *var_pclm_i_dn4_slot;
        let mut var_pclm_i_dn5: f64 = *var_pclm_i_dn5_slot;
        let mut var_pclm_i_dn6: f64 = *var_pclm_i_dn6_slot;
        let mut var_pclm_i_dn7: f64 = *var_pclm_i_dn7_slot;
        let mut var_pclm_i_dn8: f64 = *var_pclm_i_dn8_slot;
        let mut var_pclm_i_dn9: f64 = *var_pclm_i_dn9_slot;
        let mut var_pclmcv_i: f64 = *var_pclmcv_i_slot;
        let mut var_pclmg_i: f64 = *var_pclmg_i_slot;
        let mut var_psat_i: f64 = *var_psat_i_slot;
        let mut var_psat_i_dn0: f64 = *var_psat_i_dn0_slot;
        let mut var_psat_i_dn10: f64 = *var_psat_i_dn10_slot;
        let mut var_psat_i_dn11: f64 = *var_psat_i_dn11_slot;
        let mut var_psat_i_dn13: f64 = *var_psat_i_dn13_slot;
        let mut var_psat_i_dn14: f64 = *var_psat_i_dn14_slot;
        let mut var_psat_i_dn2: f64 = *var_psat_i_dn2_slot;
        let mut var_psat_i_dn3: f64 = *var_psat_i_dn3_slot;
        let mut var_psat_i_dn4: f64 = *var_psat_i_dn4_slot;
        let mut var_psat_i_dn5: f64 = *var_psat_i_dn5_slot;
        let mut var_psat_i_dn6: f64 = *var_psat_i_dn6_slot;
        let mut var_psat_i_dn7: f64 = *var_psat_i_dn7_slot;
        let mut var_psat_i_dn8: f64 = *var_psat_i_dn8_slot;
        let mut var_psat_i_dn9: f64 = *var_psat_i_dn9_slot;
        let mut var_psatcv_i: f64 = *var_psatcv_i_slot;
        let mut var_psatcv_i_dn0: f64 = *var_psatcv_i_dn0_slot;
        let mut var_psatcv_i_dn10: f64 = *var_psatcv_i_dn10_slot;
        let mut var_psatcv_i_dn11: f64 = *var_psatcv_i_dn11_slot;
        let mut var_psatcv_i_dn13: f64 = *var_psatcv_i_dn13_slot;
        let mut var_psatcv_i_dn14: f64 = *var_psatcv_i_dn14_slot;
        let mut var_psatcv_i_dn2: f64 = *var_psatcv_i_dn2_slot;
        let mut var_psatcv_i_dn3: f64 = *var_psatcv_i_dn3_slot;
        let mut var_psatcv_i_dn4: f64 = *var_psatcv_i_dn4_slot;
        let mut var_psatcv_i_dn5: f64 = *var_psatcv_i_dn5_slot;
        let mut var_psatcv_i_dn6: f64 = *var_psatcv_i_dn6_slot;
        let mut var_psatcv_i_dn7: f64 = *var_psatcv_i_dn7_slot;
        let mut var_psatcv_i_dn8: f64 = *var_psatcv_i_dn8_slot;
        let mut var_psatcv_i_dn9: f64 = *var_psatcv_i_dn9_slot;
        let mut var_ptwg_i: f64 = *var_ptwg_i_slot;
        let mut var_ptwg_i_dn0: f64 = *var_ptwg_i_dn0_slot;
        let mut var_ptwg_i_dn10: f64 = *var_ptwg_i_dn10_slot;
        let mut var_ptwg_i_dn11: f64 = *var_ptwg_i_dn11_slot;
        let mut var_ptwg_i_dn13: f64 = *var_ptwg_i_dn13_slot;
        let mut var_ptwg_i_dn14: f64 = *var_ptwg_i_dn14_slot;
        let mut var_ptwg_i_dn2: f64 = *var_ptwg_i_dn2_slot;
        let mut var_ptwg_i_dn3: f64 = *var_ptwg_i_dn3_slot;
        let mut var_ptwg_i_dn4: f64 = *var_ptwg_i_dn4_slot;
        let mut var_ptwg_i_dn5: f64 = *var_ptwg_i_dn5_slot;
        let mut var_ptwg_i_dn6: f64 = *var_ptwg_i_dn6_slot;
        let mut var_ptwg_i_dn7: f64 = *var_ptwg_i_dn7_slot;
        let mut var_ptwg_i_dn8: f64 = *var_ptwg_i_dn8_slot;
        let mut var_ptwg_i_dn9: f64 = *var_ptwg_i_dn9_slot;
        let mut var_rdsw_i: f64 = *var_rdsw_i_slot;
        let mut var_rdsw_i_dn0: f64 = *var_rdsw_i_dn0_slot;
        let mut var_rdsw_i_dn10: f64 = *var_rdsw_i_dn10_slot;
        let mut var_rdsw_i_dn11: f64 = *var_rdsw_i_dn11_slot;
        let mut var_rdsw_i_dn13: f64 = *var_rdsw_i_dn13_slot;
        let mut var_rdsw_i_dn14: f64 = *var_rdsw_i_dn14_slot;
        let mut var_rdsw_i_dn2: f64 = *var_rdsw_i_dn2_slot;
        let mut var_rdsw_i_dn3: f64 = *var_rdsw_i_dn3_slot;
        let mut var_rdsw_i_dn4: f64 = *var_rdsw_i_dn4_slot;
        let mut var_rdsw_i_dn5: f64 = *var_rdsw_i_dn5_slot;
        let mut var_rdsw_i_dn6: f64 = *var_rdsw_i_dn6_slot;
        let mut var_rdsw_i_dn7: f64 = *var_rdsw_i_dn7_slot;
        let mut var_rdsw_i_dn8: f64 = *var_rdsw_i_dn8_slot;
        let mut var_rdsw_i_dn9: f64 = *var_rdsw_i_dn9_slot;
        let mut var_rsw_i: f64 = *var_rsw_i_slot;
        let mut var_rsw_i_dn0: f64 = *var_rsw_i_dn0_slot;
        let mut var_rsw_i_dn10: f64 = *var_rsw_i_dn10_slot;
        let mut var_rsw_i_dn11: f64 = *var_rsw_i_dn11_slot;
        let mut var_rsw_i_dn13: f64 = *var_rsw_i_dn13_slot;
        let mut var_rsw_i_dn14: f64 = *var_rsw_i_dn14_slot;
        let mut var_rsw_i_dn2: f64 = *var_rsw_i_dn2_slot;
        let mut var_rsw_i_dn3: f64 = *var_rsw_i_dn3_slot;
        let mut var_rsw_i_dn4: f64 = *var_rsw_i_dn4_slot;
        let mut var_rsw_i_dn5: f64 = *var_rsw_i_dn5_slot;
        let mut var_rsw_i_dn6: f64 = *var_rsw_i_dn6_slot;
        let mut var_rsw_i_dn7: f64 = *var_rsw_i_dn7_slot;
        let mut var_rsw_i_dn8: f64 = *var_rsw_i_dn8_slot;
        let mut var_rsw_i_dn9: f64 = *var_rsw_i_dn9_slot;
        let mut var_u0_i: f64 = *var_u0_i_slot;
        let mut var_u0_i_dn0: f64 = *var_u0_i_dn0_slot;
        let mut var_u0_i_dn10: f64 = *var_u0_i_dn10_slot;
        let mut var_u0_i_dn11: f64 = *var_u0_i_dn11_slot;
        let mut var_u0_i_dn13: f64 = *var_u0_i_dn13_slot;
        let mut var_u0_i_dn14: f64 = *var_u0_i_dn14_slot;
        let mut var_u0_i_dn2: f64 = *var_u0_i_dn2_slot;
        let mut var_u0_i_dn3: f64 = *var_u0_i_dn3_slot;
        let mut var_u0_i_dn4: f64 = *var_u0_i_dn4_slot;
        let mut var_u0_i_dn5: f64 = *var_u0_i_dn5_slot;
        let mut var_u0_i_dn6: f64 = *var_u0_i_dn6_slot;
        let mut var_u0_i_dn7: f64 = *var_u0_i_dn7_slot;
        let mut var_u0_i_dn8: f64 = *var_u0_i_dn8_slot;
        let mut var_u0_i_dn9: f64 = *var_u0_i_dn9_slot;
        let mut var_ua_i: f64 = *var_ua_i_slot;
        let mut var_ua_i_dn0: f64 = *var_ua_i_dn0_slot;
        let mut var_ua_i_dn10: f64 = *var_ua_i_dn10_slot;
        let mut var_ua_i_dn11: f64 = *var_ua_i_dn11_slot;
        let mut var_ua_i_dn13: f64 = *var_ua_i_dn13_slot;
        let mut var_ua_i_dn14: f64 = *var_ua_i_dn14_slot;
        let mut var_ua_i_dn2: f64 = *var_ua_i_dn2_slot;
        let mut var_ua_i_dn3: f64 = *var_ua_i_dn3_slot;
        let mut var_ua_i_dn4: f64 = *var_ua_i_dn4_slot;
        let mut var_ua_i_dn5: f64 = *var_ua_i_dn5_slot;
        let mut var_ua_i_dn6: f64 = *var_ua_i_dn6_slot;
        let mut var_ua_i_dn7: f64 = *var_ua_i_dn7_slot;
        let mut var_ua_i_dn8: f64 = *var_ua_i_dn8_slot;
        let mut var_ua_i_dn9: f64 = *var_ua_i_dn9_slot;
        let mut var_ucs_i: f64 = *var_ucs_i_slot;
        let mut var_ud_i: f64 = *var_ud_i_slot;
        let mut var_ud_i_dn0: f64 = *var_ud_i_dn0_slot;
        let mut var_ud_i_dn10: f64 = *var_ud_i_dn10_slot;
        let mut var_ud_i_dn11: f64 = *var_ud_i_dn11_slot;
        let mut var_ud_i_dn13: f64 = *var_ud_i_dn13_slot;
        let mut var_ud_i_dn14: f64 = *var_ud_i_dn14_slot;
        let mut var_ud_i_dn2: f64 = *var_ud_i_dn2_slot;
        let mut var_ud_i_dn3: f64 = *var_ud_i_dn3_slot;
        let mut var_ud_i_dn4: f64 = *var_ud_i_dn4_slot;
        let mut var_ud_i_dn5: f64 = *var_ud_i_dn5_slot;
        let mut var_ud_i_dn6: f64 = *var_ud_i_dn6_slot;
        let mut var_ud_i_dn7: f64 = *var_ud_i_dn7_slot;
        let mut var_ud_i_dn8: f64 = *var_ud_i_dn8_slot;
        let mut var_ud_i_dn9: f64 = *var_ud_i_dn9_slot;
        let mut var_udd1_i: f64 = *var_udd1_i_slot;
        let mut var_udd_i: f64 = *var_udd_i_slot;
        let mut var_uds1_i: f64 = *var_uds1_i_slot;
        let mut var_uds_i: f64 = *var_uds_i_slot;
        let mut var_up_i: f64 = *var_up_i_slot;
        let mut var_vsatcv_i: f64 = *var_vsatcv_i_slot;
        let mut var_vsatcv_i_dn0: f64 = *var_vsatcv_i_dn0_slot;
        let mut var_vsatcv_i_dn10: f64 = *var_vsatcv_i_dn10_slot;
        let mut var_vsatcv_i_dn11: f64 = *var_vsatcv_i_dn11_slot;
        let mut var_vsatcv_i_dn13: f64 = *var_vsatcv_i_dn13_slot;
        let mut var_vsatcv_i_dn14: f64 = *var_vsatcv_i_dn14_slot;
        let mut var_vsatcv_i_dn2: f64 = *var_vsatcv_i_dn2_slot;
        let mut var_vsatcv_i_dn3: f64 = *var_vsatcv_i_dn3_slot;
        let mut var_vsatcv_i_dn4: f64 = *var_vsatcv_i_dn4_slot;
        let mut var_vsatcv_i_dn5: f64 = *var_vsatcv_i_dn5_slot;
        let mut var_vsatcv_i_dn6: f64 = *var_vsatcv_i_dn6_slot;
        let mut var_vsatcv_i_dn7: f64 = *var_vsatcv_i_dn7_slot;
        let mut var_vsatcv_i_dn8: f64 = *var_vsatcv_i_dn8_slot;
        let mut var_vsatcv_i_dn9: f64 = *var_vsatcv_i_dn9_slot;

        let assign4400_e5370: f64 = (var_inv_l * p.p507);
        let assign4400_e5371: f64 = (p.p506 + assign4400_e5370);
        let assign4400_e5374: f64 = (var_inv_nfin * p.p508);
        let assign4400_e5375: f64 = (assign4400_e5371 + assign4400_e5374);
        let assign4400_e5378: f64 = (var_inv_lnfin * p.p509);
        let assign4400_e5379: f64 = (assign4400_e5375 + assign4400_e5378);
        let assign4400_e5382: f64 = (var_inv_w * p.p510);
        let assign4400_e5383: f64 = (assign4400_e5379 + assign4400_e5382);
        let assign4400_e5386: f64 = (var_inv_wl * p.p511);
        let assign4400_e5387: f64 = (assign4400_e5383 + assign4400_e5386);
        var_vsatcv_i = assign4400_e5387;
        var_vsatcv_i_dn0 = 0.0;
        var_vsatcv_i_dn2 = 0.0;
        var_vsatcv_i_dn3 = 0.0;
        var_vsatcv_i_dn4 = 0.0;
        var_vsatcv_i_dn5 = 0.0;
        var_vsatcv_i_dn6 = 0.0;
        var_vsatcv_i_dn7 = 0.0;
        var_vsatcv_i_dn8 = 0.0;
        var_vsatcv_i_dn9 = 0.0;
        var_vsatcv_i_dn10 = 0.0;
        var_vsatcv_i_dn11 = 0.0;
        var_vsatcv_i_dn13 = 0.0;
        var_vsatcv_i_dn14 = 0.0;

        let assign4410_e5391: f64 = (var_inv_l * p.p513);
        let assign4410_e5392: f64 = (p.p512 + assign4410_e5391);
        let assign4410_e5395: f64 = (var_inv_nfin * p.p514);
        let assign4410_e5396: f64 = (assign4410_e5392 + assign4410_e5395);
        let assign4410_e5399: f64 = (var_inv_lnfin * p.p515);
        let assign4410_e5400: f64 = (assign4410_e5396 + assign4410_e5399);
        let assign4410_e5403: f64 = (var_inv_w * p.p516);
        let assign4410_e5404: f64 = (assign4410_e5400 + assign4410_e5403);
        let assign4410_e5407: f64 = (var_inv_wl * p.p517);
        let assign4410_e5408: f64 = (assign4410_e5404 + assign4410_e5407);
        var_asat_i = assign4410_e5408;

        let assign4420_e5412: f64 = (var_inv_l * p.p480);
        let assign4420_e5413: f64 = (p.p479 + assign4420_e5412);
        let assign4420_e5416: f64 = (var_inv_nfin * p.p481);
        let assign4420_e5417: f64 = (assign4420_e5413 + assign4420_e5416);
        let assign4420_e5420: f64 = (var_inv_lnfin * p.p482);
        let assign4420_e5421: f64 = (assign4420_e5417 + assign4420_e5420);
        let assign4420_e5424: f64 = (var_inv_w * p.p483);
        let assign4420_e5425: f64 = (assign4420_e5421 + assign4420_e5424);
        let assign4420_e5428: f64 = (var_inv_wl * p.p484);
        let assign4420_e5429: f64 = (assign4420_e5425 + assign4420_e5428);
        var_deltavsat_i = assign4420_e5429;

        let assign4430_e5433: f64 = (var_inv_l * p.p486);
        let assign4430_e5434: f64 = (p.p485 + assign4430_e5433);
        let assign4430_e5437: f64 = (var_inv_nfin * p.p487);
        let assign4430_e5438: f64 = (assign4430_e5434 + assign4430_e5437);
        let assign4430_e5441: f64 = (var_inv_lnfin * p.p488);
        let assign4430_e5442: f64 = (assign4430_e5438 + assign4430_e5441);
        let assign4430_e5445: f64 = (var_inv_w * p.p489);
        let assign4430_e5446: f64 = (assign4430_e5442 + assign4430_e5445);
        let assign4430_e5449: f64 = (var_inv_wl * p.p490);
        let assign4430_e5450: f64 = (assign4430_e5446 + assign4430_e5449);
        var_psat_i = assign4430_e5450;
        var_psat_i_dn0 = 0.0;
        var_psat_i_dn2 = 0.0;
        var_psat_i_dn3 = 0.0;
        var_psat_i_dn4 = 0.0;
        var_psat_i_dn5 = 0.0;
        var_psat_i_dn6 = 0.0;
        var_psat_i_dn7 = 0.0;
        var_psat_i_dn8 = 0.0;
        var_psat_i_dn9 = 0.0;
        var_psat_i_dn10 = 0.0;
        var_psat_i_dn11 = 0.0;
        var_psat_i_dn13 = 0.0;
        var_psat_i_dn14 = 0.0;

        let assign4440_e5454: f64 = (var_inv_l * p.p519);
        let assign4440_e5455: f64 = (p.p518 + assign4440_e5454);
        let assign4440_e5458: f64 = (var_inv_nfin * p.p520);
        let assign4440_e5459: f64 = (assign4440_e5455 + assign4440_e5458);
        let assign4440_e5462: f64 = (var_inv_lnfin * p.p521);
        let assign4440_e5463: f64 = (assign4440_e5459 + assign4440_e5462);
        let assign4440_e5466: f64 = (var_inv_w * p.p522);
        let assign4440_e5467: f64 = (assign4440_e5463 + assign4440_e5466);
        let assign4440_e5470: f64 = (var_inv_wl * p.p523);
        let assign4440_e5471: f64 = (assign4440_e5467 + assign4440_e5470);
        var_deltavsatcv_i = assign4440_e5471;

        let assign4450_e5475: f64 = (var_inv_l * p.p525);
        let assign4450_e5476: f64 = (p.p524 + assign4450_e5475);
        let assign4450_e5479: f64 = (var_inv_nfin * p.p526);
        let assign4450_e5480: f64 = (assign4450_e5476 + assign4450_e5479);
        let assign4450_e5483: f64 = (var_inv_lnfin * p.p527);
        let assign4450_e5484: f64 = (assign4450_e5480 + assign4450_e5483);
        let assign4450_e5487: f64 = (var_inv_w * p.p528);
        let assign4450_e5488: f64 = (assign4450_e5484 + assign4450_e5487);
        let assign4450_e5491: f64 = (var_inv_wl * p.p529);
        let assign4450_e5492: f64 = (assign4450_e5488 + assign4450_e5491);
        var_psatcv_i = assign4450_e5492;
        var_psatcv_i_dn0 = 0.0;
        var_psatcv_i_dn2 = 0.0;
        var_psatcv_i_dn3 = 0.0;
        var_psatcv_i_dn4 = 0.0;
        var_psatcv_i_dn5 = 0.0;
        var_psatcv_i_dn6 = 0.0;
        var_psatcv_i_dn7 = 0.0;
        var_psatcv_i_dn8 = 0.0;
        var_psatcv_i_dn9 = 0.0;
        var_psatcv_i_dn10 = 0.0;
        var_psatcv_i_dn11 = 0.0;
        var_psatcv_i_dn13 = 0.0;
        var_psatcv_i_dn14 = 0.0;

        let assign4460_e5496: f64 = (var_inv_l * p.p493);
        let assign4460_e5497: f64 = (p.p492 + assign4460_e5496);
        let assign4460_e5500: f64 = (var_inv_nfin * p.p494);
        let assign4460_e5501: f64 = (assign4460_e5497 + assign4460_e5500);
        let assign4460_e5504: f64 = (var_inv_lnfin * p.p495);
        let assign4460_e5505: f64 = (assign4460_e5501 + assign4460_e5504);
        let assign4460_e5508: f64 = (var_inv_w * p.p496);
        let assign4460_e5509: f64 = (assign4460_e5505 + assign4460_e5508);
        let assign4460_e5512: f64 = (var_inv_wl * p.p497);
        let assign4460_e5513: f64 = (assign4460_e5509 + assign4460_e5512);
        var_ksativ_i = assign4460_e5513;

        let assign4470_e5517: f64 = (var_inv_l * p.p532);
        let assign4470_e5518: f64 = (p.p531 + assign4470_e5517);
        let assign4470_e5521: f64 = (var_inv_nfin * p.p533);
        let assign4470_e5522: f64 = (assign4470_e5518 + assign4470_e5521);
        let assign4470_e5525: f64 = (var_inv_lnfin * p.p534);
        let assign4470_e5526: f64 = (assign4470_e5522 + assign4470_e5525);
        let assign4470_e5529: f64 = (var_inv_w * p.p535);
        let assign4470_e5530: f64 = (assign4470_e5526 + assign4470_e5529);
        let assign4470_e5533: f64 = (var_inv_wl * p.p536);
        let assign4470_e5534: f64 = (assign4470_e5530 + assign4470_e5533);
        var_mexp_i = assign4470_e5534;
        var_mexp_i_dn0 = 0.0;
        var_mexp_i_dn2 = 0.0;
        var_mexp_i_dn3 = 0.0;
        var_mexp_i_dn4 = 0.0;
        var_mexp_i_dn5 = 0.0;
        var_mexp_i_dn6 = 0.0;
        var_mexp_i_dn7 = 0.0;
        var_mexp_i_dn8 = 0.0;
        var_mexp_i_dn9 = 0.0;
        var_mexp_i_dn10 = 0.0;
        var_mexp_i_dn11 = 0.0;
        var_mexp_i_dn13 = 0.0;
        var_mexp_i_dn14 = 0.0;

        let assign4480_e5538: f64 = (var_inv_l * p.p544);
        let assign4480_e5539: f64 = (p.p543 + assign4480_e5538);
        let assign4480_e5542: f64 = (var_inv_nfin * p.p545);
        let assign4480_e5543: f64 = (assign4480_e5539 + assign4480_e5542);
        let assign4480_e5546: f64 = (var_inv_lnfin * p.p546);
        let assign4480_e5547: f64 = (assign4480_e5543 + assign4480_e5546);
        let assign4480_e5550: f64 = (var_inv_w * p.p547);
        let assign4480_e5551: f64 = (assign4480_e5547 + assign4480_e5550);
        let assign4480_e5554: f64 = (var_inv_wl * p.p548);
        let assign4480_e5555: f64 = (assign4480_e5551 + assign4480_e5554);
        var_ptwg_i = assign4480_e5555;
        var_ptwg_i_dn0 = 0.0;
        var_ptwg_i_dn2 = 0.0;
        var_ptwg_i_dn3 = 0.0;
        var_ptwg_i_dn4 = 0.0;
        var_ptwg_i_dn5 = 0.0;
        var_ptwg_i_dn6 = 0.0;
        var_ptwg_i_dn7 = 0.0;
        var_ptwg_i_dn8 = 0.0;
        var_ptwg_i_dn9 = 0.0;
        var_ptwg_i_dn10 = 0.0;
        var_ptwg_i_dn11 = 0.0;
        var_ptwg_i_dn13 = 0.0;
        var_ptwg_i_dn14 = 0.0;

        let assign4490_e5559: f64 = (var_inv_l * p.p606);
        let assign4490_e5560: f64 = (p.p605 + assign4490_e5559);
        let assign4490_e5563: f64 = (var_inv_nfin * p.p607);
        let assign4490_e5564: f64 = (assign4490_e5560 + assign4490_e5563);
        let assign4490_e5567: f64 = (var_inv_lnfin * p.p608);
        let assign4490_e5568: f64 = (assign4490_e5564 + assign4490_e5567);
        let assign4490_e5571: f64 = (var_inv_w * p.p609);
        let assign4490_e5572: f64 = (assign4490_e5568 + assign4490_e5571);
        let assign4490_e5575: f64 = (var_inv_wl * p.p610);
        let assign4490_e5576: f64 = (assign4490_e5572 + assign4490_e5575);
        var_u0_i = assign4490_e5576;
        var_u0_i_dn0 = 0.0;
        var_u0_i_dn2 = 0.0;
        var_u0_i_dn3 = 0.0;
        var_u0_i_dn4 = 0.0;
        var_u0_i_dn5 = 0.0;
        var_u0_i_dn6 = 0.0;
        var_u0_i_dn7 = 0.0;
        var_u0_i_dn8 = 0.0;
        var_u0_i_dn9 = 0.0;
        var_u0_i_dn10 = 0.0;
        var_u0_i_dn11 = 0.0;
        var_u0_i_dn13 = 0.0;
        var_u0_i_dn14 = 0.0;

        let assign4500_e5580: f64 = (var_inv_l * p.p624);
        let assign4500_e5581: f64 = (p.p623 + assign4500_e5580);
        let assign4500_e5584: f64 = (var_inv_nfin * p.p625);
        let assign4500_e5585: f64 = (assign4500_e5581 + assign4500_e5584);
        let assign4500_e5588: f64 = (var_inv_lnfin * p.p626);
        let assign4500_e5589: f64 = (assign4500_e5585 + assign4500_e5588);
        let assign4500_e5592: f64 = (var_inv_w * p.p627);
        let assign4500_e5593: f64 = (assign4500_e5589 + assign4500_e5592);
        let assign4500_e5596: f64 = (var_inv_wl * p.p628);
        let assign4500_e5597: f64 = (assign4500_e5593 + assign4500_e5596);
        var_etamob_i = assign4500_e5597;

        let assign4510_e5601: f64 = (var_inv_l * p.p630);
        let assign4510_e5602: f64 = (p.p629 + assign4510_e5601);
        let assign4510_e5605: f64 = (var_inv_nfin * p.p631);
        let assign4510_e5606: f64 = (assign4510_e5602 + assign4510_e5605);
        let assign4510_e5609: f64 = (var_inv_lnfin * p.p632);
        let assign4510_e5610: f64 = (assign4510_e5606 + assign4510_e5609);
        let assign4510_e5613: f64 = (var_inv_w * p.p633);
        let assign4510_e5614: f64 = (assign4510_e5610 + assign4510_e5613);
        let assign4510_e5617: f64 = (var_inv_wl * p.p634);
        let assign4510_e5618: f64 = (assign4510_e5614 + assign4510_e5617);
        var_up_i = assign4510_e5618;

        let assign4520_e5622: f64 = (var_inv_l * p.p642);
        let assign4520_e5623: f64 = (p.p641 + assign4520_e5622);
        let assign4520_e5626: f64 = (var_inv_nfin * p.p643);
        let assign4520_e5627: f64 = (assign4520_e5623 + assign4520_e5626);
        let assign4520_e5630: f64 = (var_inv_lnfin * p.p644);
        let assign4520_e5631: f64 = (assign4520_e5627 + assign4520_e5630);
        let assign4520_e5634: f64 = (var_inv_w * p.p645);
        let assign4520_e5635: f64 = (assign4520_e5631 + assign4520_e5634);
        let assign4520_e5638: f64 = (var_inv_wl * p.p646);
        let assign4520_e5639: f64 = (assign4520_e5635 + assign4520_e5638);
        var_ua_i = assign4520_e5639;
        var_ua_i_dn0 = 0.0;
        var_ua_i_dn2 = 0.0;
        var_ua_i_dn3 = 0.0;
        var_ua_i_dn4 = 0.0;
        var_ua_i_dn5 = 0.0;
        var_ua_i_dn6 = 0.0;
        var_ua_i_dn7 = 0.0;
        var_ua_i_dn8 = 0.0;
        var_ua_i_dn9 = 0.0;
        var_ua_i_dn10 = 0.0;
        var_ua_i_dn11 = 0.0;
        var_ua_i_dn13 = 0.0;
        var_ua_i_dn14 = 0.0;

        let assign4530_e5643: f64 = (var_inv_l * p.p678);
        let assign4530_e5644: f64 = (p.p677 + assign4530_e5643);
        let assign4530_e5647: f64 = (var_inv_nfin * p.p679);
        let assign4530_e5648: f64 = (assign4530_e5644 + assign4530_e5647);
        let assign4530_e5651: f64 = (var_inv_lnfin * p.p680);
        let assign4530_e5652: f64 = (assign4530_e5648 + assign4530_e5651);
        let assign4530_e5655: f64 = (var_inv_w * p.p681);
        let assign4530_e5656: f64 = (assign4530_e5652 + assign4530_e5655);
        let assign4530_e5659: f64 = (var_inv_wl * p.p682);
        let assign4530_e5660: f64 = (assign4530_e5656 + assign4530_e5659);
        var_eu_i = assign4530_e5660;
        var_eu_i_dn0 = 0.0;
        var_eu_i_dn2 = 0.0;
        var_eu_i_dn3 = 0.0;
        var_eu_i_dn4 = 0.0;
        var_eu_i_dn5 = 0.0;
        var_eu_i_dn6 = 0.0;
        var_eu_i_dn7 = 0.0;
        var_eu_i_dn8 = 0.0;
        var_eu_i_dn9 = 0.0;
        var_eu_i_dn10 = 0.0;
        var_eu_i_dn11 = 0.0;
        var_eu_i_dn13 = 0.0;
        var_eu_i_dn14 = 0.0;

        let assign4540_e5664: f64 = (var_inv_l * p.p690);
        let assign4540_e5665: f64 = (p.p689 + assign4540_e5664);
        let assign4540_e5668: f64 = (var_inv_nfin * p.p691);
        let assign4540_e5669: f64 = (assign4540_e5665 + assign4540_e5668);
        let assign4540_e5672: f64 = (var_inv_lnfin * p.p692);
        let assign4540_e5673: f64 = (assign4540_e5669 + assign4540_e5672);
        let assign4540_e5676: f64 = (var_inv_w * p.p693);
        let assign4540_e5677: f64 = (assign4540_e5673 + assign4540_e5676);
        let assign4540_e5680: f64 = (var_inv_wl * p.p694);
        let assign4540_e5681: f64 = (assign4540_e5677 + assign4540_e5680);
        var_ud_i = assign4540_e5681;
        var_ud_i_dn0 = 0.0;
        var_ud_i_dn2 = 0.0;
        var_ud_i_dn3 = 0.0;
        var_ud_i_dn4 = 0.0;
        var_ud_i_dn5 = 0.0;
        var_ud_i_dn6 = 0.0;
        var_ud_i_dn7 = 0.0;
        var_ud_i_dn8 = 0.0;
        var_ud_i_dn9 = 0.0;
        var_ud_i_dn10 = 0.0;
        var_ud_i_dn11 = 0.0;
        var_ud_i_dn13 = 0.0;
        var_ud_i_dn14 = 0.0;

        let assign4550_e5685: f64 = (var_inv_l * p.p708);
        let assign4550_e5686: f64 = (p.p707 + assign4550_e5685);
        let assign4550_e5689: f64 = (var_inv_nfin * p.p709);
        let assign4550_e5690: f64 = (assign4550_e5686 + assign4550_e5689);
        let assign4550_e5693: f64 = (var_inv_lnfin * p.p710);
        let assign4550_e5694: f64 = (assign4550_e5690 + assign4550_e5693);
        let assign4550_e5697: f64 = (var_inv_w * p.p711);
        let assign4550_e5698: f64 = (assign4550_e5694 + assign4550_e5697);
        let assign4550_e5701: f64 = (var_inv_wl * p.p712);
        let assign4550_e5702: f64 = (assign4550_e5698 + assign4550_e5701);
        var_ucs_i = assign4550_e5702;

        let assign4560_e5706: f64 = (var_inv_l * p.p714);
        let assign4560_e5707: f64 = (p.p713 + assign4560_e5706);
        let assign4560_e5710: f64 = (var_inv_nfin * p.p715);
        let assign4560_e5711: f64 = (assign4560_e5707 + assign4560_e5710);
        let assign4560_e5714: f64 = (var_inv_lnfin * p.p716);
        let assign4560_e5715: f64 = (assign4560_e5711 + assign4560_e5714);
        let assign4560_e5718: f64 = (var_inv_w * p.p717);
        let assign4560_e5719: f64 = (assign4560_e5715 + assign4560_e5718);
        let assign4560_e5722: f64 = (var_inv_wl * p.p718);
        let assign4560_e5723: f64 = (assign4560_e5719 + assign4560_e5722);
        var_uds_i = assign4560_e5723;

        let assign4570_e5727: f64 = (var_inv_l * p.p720);
        let assign4570_e5728: f64 = (p.p719 + assign4570_e5727);
        let assign4570_e5731: f64 = (var_inv_nfin * p.p721);
        let assign4570_e5732: f64 = (assign4570_e5728 + assign4570_e5731);
        let assign4570_e5735: f64 = (var_inv_lnfin * p.p722);
        let assign4570_e5736: f64 = (assign4570_e5732 + assign4570_e5735);
        let assign4570_e5739: f64 = (var_inv_w * p.p723);
        let assign4570_e5740: f64 = (assign4570_e5736 + assign4570_e5739);
        let assign4570_e5743: f64 = (var_inv_wl * p.p724);
        let assign4570_e5744: f64 = (assign4570_e5740 + assign4570_e5743);
        var_uds1_i = assign4570_e5744;

        let assign4580_e5748: f64 = (var_inv_l * p.p726);
        let assign4580_e5749: f64 = (p.p725 + assign4580_e5748);
        let assign4580_e5752: f64 = (var_inv_nfin * p.p727);
        let assign4580_e5753: f64 = (assign4580_e5749 + assign4580_e5752);
        let assign4580_e5756: f64 = (var_inv_lnfin * p.p728);
        let assign4580_e5757: f64 = (assign4580_e5753 + assign4580_e5756);
        let assign4580_e5760: f64 = (var_inv_w * p.p729);
        let assign4580_e5761: f64 = (assign4580_e5757 + assign4580_e5760);
        let assign4580_e5764: f64 = (var_inv_wl * p.p730);
        let assign4580_e5765: f64 = (assign4580_e5761 + assign4580_e5764);
        var_udd_i = assign4580_e5765;

        let assign4590_e5769: f64 = (var_inv_l * p.p732);
        let assign4590_e5770: f64 = (p.p731 + assign4590_e5769);
        let assign4590_e5773: f64 = (var_inv_nfin * p.p733);
        let assign4590_e5774: f64 = (assign4590_e5770 + assign4590_e5773);
        let assign4590_e5777: f64 = (var_inv_lnfin * p.p734);
        let assign4590_e5778: f64 = (assign4590_e5774 + assign4590_e5777);
        let assign4590_e5781: f64 = (var_inv_w * p.p735);
        let assign4590_e5782: f64 = (assign4590_e5778 + assign4590_e5781);
        let assign4590_e5785: f64 = (var_inv_wl * p.p736);
        let assign4590_e5786: f64 = (assign4590_e5782 + assign4590_e5785);
        var_udd1_i = assign4590_e5786;

        let assign4600_e5790: f64 = (var_inv_l * p.p1027);
        let assign4600_e5791: f64 = (p.p1025 + assign4600_e5790);
        let assign4600_e5794: f64 = (var_inv_nfin * p.p1028);
        let assign4600_e5795: f64 = (assign4600_e5791 + assign4600_e5794);
        let assign4600_e5798: f64 = (var_inv_lnfin * p.p1029);
        let assign4600_e5799: f64 = (assign4600_e5795 + assign4600_e5798);
        let assign4600_e5802: f64 = (var_inv_w * p.p1030);
        let assign4600_e5803: f64 = (assign4600_e5799 + assign4600_e5802);
        let assign4600_e5806: f64 = (var_inv_wl * p.p1031);
        let assign4600_e5807: f64 = (assign4600_e5803 + assign4600_e5806);
        var_pclm_i = assign4600_e5807;
        var_pclm_i_dn0 = 0.0;
        var_pclm_i_dn2 = 0.0;
        var_pclm_i_dn3 = 0.0;
        var_pclm_i_dn4 = 0.0;
        var_pclm_i_dn5 = 0.0;
        var_pclm_i_dn6 = 0.0;
        var_pclm_i_dn7 = 0.0;
        var_pclm_i_dn8 = 0.0;
        var_pclm_i_dn9 = 0.0;
        var_pclm_i_dn10 = 0.0;
        var_pclm_i_dn11 = 0.0;
        var_pclm_i_dn13 = 0.0;
        var_pclm_i_dn14 = 0.0;

        let assign4610_e5811: f64 = (var_inv_l * p.p1039);
        let assign4610_e5812: f64 = (p.p1038 + assign4610_e5811);
        let assign4610_e5815: f64 = (var_inv_nfin * p.p1040);
        let assign4610_e5816: f64 = (assign4610_e5812 + assign4610_e5815);
        let assign4610_e5819: f64 = (var_inv_lnfin * p.p1041);
        let assign4610_e5820: f64 = (assign4610_e5816 + assign4610_e5819);
        let assign4610_e5823: f64 = (var_inv_w * p.p1042);
        let assign4610_e5824: f64 = (assign4610_e5820 + assign4610_e5823);
        let assign4610_e5827: f64 = (var_inv_wl * p.p1043);
        let assign4610_e5828: f64 = (assign4610_e5824 + assign4610_e5827);
        var_pclmg_i = assign4610_e5828;

        let assign4620_e5832: f64 = (var_inv_l * p.p1045);
        let assign4620_e5833: f64 = (p.p1044 + assign4620_e5832);
        let assign4620_e5836: f64 = (var_inv_nfin * p.p1046);
        let assign4620_e5837: f64 = (assign4620_e5833 + assign4620_e5836);
        let assign4620_e5840: f64 = (var_inv_lnfin * p.p1047);
        let assign4620_e5841: f64 = (assign4620_e5837 + assign4620_e5840);
        let assign4620_e5844: f64 = (var_inv_w * p.p1048);
        let assign4620_e5845: f64 = (assign4620_e5841 + assign4620_e5844);
        let assign4620_e5848: f64 = (var_inv_wl * p.p1049);
        let assign4620_e5849: f64 = (assign4620_e5845 + assign4620_e5848);
        var_pclmcv_i = assign4620_e5849;

        let assign4630_e5853: f64 = (var_inv_l * p.p1051);
        let assign4630_e5854: f64 = (p.p1050 + assign4630_e5853);
        let assign4630_e5857: f64 = (var_inv_nfin * p.p1052);
        let assign4630_e5858: f64 = (assign4630_e5854 + assign4630_e5857);
        let assign4630_e5861: f64 = (var_inv_lnfin * p.p1053);
        let assign4630_e5862: f64 = (assign4630_e5858 + assign4630_e5861);
        let assign4630_e5865: f64 = (var_inv_w * p.p1054);
        let assign4630_e5866: f64 = (assign4630_e5862 + assign4630_e5865);
        let assign4630_e5869: f64 = (var_inv_wl * p.p1055);
        let assign4630_e5870: f64 = (assign4630_e5866 + assign4630_e5869);
        var_a1_i = assign4630_e5870;

        let assign4640_e5874: f64 = (var_inv_l * p.p1057);
        let assign4640_e5875: f64 = (p.p1056 + assign4640_e5874);
        let assign4640_e5878: f64 = (var_inv_nfin * p.p1058);
        let assign4640_e5879: f64 = (assign4640_e5875 + assign4640_e5878);
        let assign4640_e5882: f64 = (var_inv_lnfin * p.p1059);
        let assign4640_e5883: f64 = (assign4640_e5879 + assign4640_e5882);
        let assign4640_e5886: f64 = (var_inv_w * p.p1060);
        let assign4640_e5887: f64 = (assign4640_e5883 + assign4640_e5886);
        let assign4640_e5890: f64 = (var_inv_wl * p.p1061);
        let assign4640_e5891: f64 = (assign4640_e5887 + assign4640_e5890);
        var_a11_i = assign4640_e5891;

        let assign4650_e5895: f64 = (var_inv_l * p.p1063);
        let assign4650_e5896: f64 = (p.p1062 + assign4650_e5895);
        let assign4650_e5899: f64 = (var_inv_nfin * p.p1064);
        let assign4650_e5900: f64 = (assign4650_e5896 + assign4650_e5899);
        let assign4650_e5903: f64 = (var_inv_lnfin * p.p1065);
        let assign4650_e5904: f64 = (assign4650_e5900 + assign4650_e5903);
        let assign4650_e5907: f64 = (var_inv_w * p.p1066);
        let assign4650_e5908: f64 = (assign4650_e5904 + assign4650_e5907);
        let assign4650_e5911: f64 = (var_inv_wl * p.p1067);
        let assign4650_e5912: f64 = (assign4650_e5908 + assign4650_e5911);
        var_a2_i = assign4650_e5912;

        let assign4660_e5916: f64 = (var_inv_l * p.p1069);
        let assign4660_e5917: f64 = (p.p1068 + assign4660_e5916);
        let assign4660_e5920: f64 = (var_inv_nfin * p.p1070);
        let assign4660_e5921: f64 = (assign4660_e5917 + assign4660_e5920);
        let assign4660_e5924: f64 = (var_inv_lnfin * p.p1071);
        let assign4660_e5925: f64 = (assign4660_e5921 + assign4660_e5924);
        let assign4660_e5928: f64 = (var_inv_w * p.p1072);
        let assign4660_e5929: f64 = (assign4660_e5925 + assign4660_e5928);
        let assign4660_e5932: f64 = (var_inv_wl * p.p1073);
        let assign4660_e5933: f64 = (assign4660_e5929 + assign4660_e5932);
        var_a21_i = assign4660_e5933;

        let assign4670_e5937: f64 = (var_inv_l * p.p926);
        let assign4670_e5938: f64 = (p.p925 + assign4670_e5937);
        let assign4670_e5941: f64 = (var_inv_nfin * p.p927);
        let assign4670_e5942: f64 = (assign4670_e5938 + assign4670_e5941);
        let assign4670_e5945: f64 = (var_inv_lnfin * p.p928);
        let assign4670_e5946: f64 = (assign4670_e5942 + assign4670_e5945);
        let assign4670_e5949: f64 = (var_inv_w * p.p929);
        let assign4670_e5950: f64 = (assign4670_e5946 + assign4670_e5949);
        let assign4670_e5953: f64 = (var_inv_wl * p.p930);
        let assign4670_e5954: f64 = (assign4670_e5950 + assign4670_e5953);
        var_rdsw_i = assign4670_e5954;
        var_rdsw_i_dn0 = 0.0;
        var_rdsw_i_dn2 = 0.0;
        var_rdsw_i_dn3 = 0.0;
        var_rdsw_i_dn4 = 0.0;
        var_rdsw_i_dn5 = 0.0;
        var_rdsw_i_dn6 = 0.0;
        var_rdsw_i_dn7 = 0.0;
        var_rdsw_i_dn8 = 0.0;
        var_rdsw_i_dn9 = 0.0;
        var_rdsw_i_dn10 = 0.0;
        var_rdsw_i_dn11 = 0.0;
        var_rdsw_i_dn13 = 0.0;
        var_rdsw_i_dn14 = 0.0;

        let assign4680_e5958: f64 = (var_inv_l * p.p932);
        let assign4680_e5959: f64 = (p.p931 + assign4680_e5958);
        let assign4680_e5962: f64 = (var_inv_nfin * p.p933);
        let assign4680_e5963: f64 = (assign4680_e5959 + assign4680_e5962);
        let assign4680_e5966: f64 = (var_inv_lnfin * p.p934);
        let assign4680_e5967: f64 = (assign4680_e5963 + assign4680_e5966);
        let assign4680_e5970: f64 = (var_inv_w * p.p935);
        let assign4680_e5971: f64 = (assign4680_e5967 + assign4680_e5970);
        let assign4680_e5974: f64 = (var_inv_wl * p.p936);
        let assign4680_e5975: f64 = (assign4680_e5971 + assign4680_e5974);
        var_rsw_i = assign4680_e5975;
        var_rsw_i_dn0 = 0.0;
        var_rsw_i_dn2 = 0.0;
        var_rsw_i_dn3 = 0.0;
        var_rsw_i_dn4 = 0.0;
        var_rsw_i_dn5 = 0.0;
        var_rsw_i_dn6 = 0.0;
        var_rsw_i_dn7 = 0.0;
        var_rsw_i_dn8 = 0.0;
        var_rsw_i_dn9 = 0.0;
        var_rsw_i_dn10 = 0.0;
        var_rsw_i_dn11 = 0.0;
        var_rsw_i_dn13 = 0.0;
        var_rsw_i_dn14 = 0.0;

        *var_a11_i_slot = var_a11_i;
        *var_a1_i_slot = var_a1_i;
        *var_a21_i_slot = var_a21_i;
        *var_a2_i_slot = var_a2_i;
        *var_asat_i_slot = var_asat_i;
        *var_deltavsat_i_slot = var_deltavsat_i;
        *var_deltavsatcv_i_slot = var_deltavsatcv_i;
        *var_etamob_i_slot = var_etamob_i;
        *var_eu_i_slot = var_eu_i;
        *var_eu_i_dn0_slot = var_eu_i_dn0;
        *var_eu_i_dn10_slot = var_eu_i_dn10;
        *var_eu_i_dn11_slot = var_eu_i_dn11;
        *var_eu_i_dn13_slot = var_eu_i_dn13;
        *var_eu_i_dn14_slot = var_eu_i_dn14;
        *var_eu_i_dn2_slot = var_eu_i_dn2;
        *var_eu_i_dn3_slot = var_eu_i_dn3;
        *var_eu_i_dn4_slot = var_eu_i_dn4;
        *var_eu_i_dn5_slot = var_eu_i_dn5;
        *var_eu_i_dn6_slot = var_eu_i_dn6;
        *var_eu_i_dn7_slot = var_eu_i_dn7;
        *var_eu_i_dn8_slot = var_eu_i_dn8;
        *var_eu_i_dn9_slot = var_eu_i_dn9;
        *var_ksativ_i_slot = var_ksativ_i;
        *var_mexp_i_slot = var_mexp_i;
        *var_mexp_i_dn0_slot = var_mexp_i_dn0;
        *var_mexp_i_dn10_slot = var_mexp_i_dn10;
        *var_mexp_i_dn11_slot = var_mexp_i_dn11;
        *var_mexp_i_dn13_slot = var_mexp_i_dn13;
        *var_mexp_i_dn14_slot = var_mexp_i_dn14;
        *var_mexp_i_dn2_slot = var_mexp_i_dn2;
        *var_mexp_i_dn3_slot = var_mexp_i_dn3;
        *var_mexp_i_dn4_slot = var_mexp_i_dn4;
        *var_mexp_i_dn5_slot = var_mexp_i_dn5;
        *var_mexp_i_dn6_slot = var_mexp_i_dn6;
        *var_mexp_i_dn7_slot = var_mexp_i_dn7;
        *var_mexp_i_dn8_slot = var_mexp_i_dn8;
        *var_mexp_i_dn9_slot = var_mexp_i_dn9;
        *var_pclm_i_slot = var_pclm_i;
        *var_pclm_i_dn0_slot = var_pclm_i_dn0;
        *var_pclm_i_dn10_slot = var_pclm_i_dn10;
        *var_pclm_i_dn11_slot = var_pclm_i_dn11;
        *var_pclm_i_dn13_slot = var_pclm_i_dn13;
        *var_pclm_i_dn14_slot = var_pclm_i_dn14;
        *var_pclm_i_dn2_slot = var_pclm_i_dn2;
        *var_pclm_i_dn3_slot = var_pclm_i_dn3;
        *var_pclm_i_dn4_slot = var_pclm_i_dn4;
        *var_pclm_i_dn5_slot = var_pclm_i_dn5;
        *var_pclm_i_dn6_slot = var_pclm_i_dn6;
        *var_pclm_i_dn7_slot = var_pclm_i_dn7;
        *var_pclm_i_dn8_slot = var_pclm_i_dn8;
        *var_pclm_i_dn9_slot = var_pclm_i_dn9;
        *var_pclmcv_i_slot = var_pclmcv_i;
        *var_pclmg_i_slot = var_pclmg_i;
        *var_psat_i_slot = var_psat_i;
        *var_psat_i_dn0_slot = var_psat_i_dn0;
        *var_psat_i_dn10_slot = var_psat_i_dn10;
        *var_psat_i_dn11_slot = var_psat_i_dn11;
        *var_psat_i_dn13_slot = var_psat_i_dn13;
        *var_psat_i_dn14_slot = var_psat_i_dn14;
        *var_psat_i_dn2_slot = var_psat_i_dn2;
        *var_psat_i_dn3_slot = var_psat_i_dn3;
        *var_psat_i_dn4_slot = var_psat_i_dn4;
        *var_psat_i_dn5_slot = var_psat_i_dn5;
        *var_psat_i_dn6_slot = var_psat_i_dn6;
        *var_psat_i_dn7_slot = var_psat_i_dn7;
        *var_psat_i_dn8_slot = var_psat_i_dn8;
        *var_psat_i_dn9_slot = var_psat_i_dn9;
        *var_psatcv_i_slot = var_psatcv_i;
        *var_psatcv_i_dn0_slot = var_psatcv_i_dn0;
        *var_psatcv_i_dn10_slot = var_psatcv_i_dn10;
        *var_psatcv_i_dn11_slot = var_psatcv_i_dn11;
        *var_psatcv_i_dn13_slot = var_psatcv_i_dn13;
        *var_psatcv_i_dn14_slot = var_psatcv_i_dn14;
        *var_psatcv_i_dn2_slot = var_psatcv_i_dn2;
        *var_psatcv_i_dn3_slot = var_psatcv_i_dn3;
        *var_psatcv_i_dn4_slot = var_psatcv_i_dn4;
        *var_psatcv_i_dn5_slot = var_psatcv_i_dn5;
        *var_psatcv_i_dn6_slot = var_psatcv_i_dn6;
        *var_psatcv_i_dn7_slot = var_psatcv_i_dn7;
        *var_psatcv_i_dn8_slot = var_psatcv_i_dn8;
        *var_psatcv_i_dn9_slot = var_psatcv_i_dn9;
        *var_ptwg_i_slot = var_ptwg_i;
        *var_ptwg_i_dn0_slot = var_ptwg_i_dn0;
        *var_ptwg_i_dn10_slot = var_ptwg_i_dn10;
        *var_ptwg_i_dn11_slot = var_ptwg_i_dn11;
        *var_ptwg_i_dn13_slot = var_ptwg_i_dn13;
        *var_ptwg_i_dn14_slot = var_ptwg_i_dn14;
        *var_ptwg_i_dn2_slot = var_ptwg_i_dn2;
        *var_ptwg_i_dn3_slot = var_ptwg_i_dn3;
        *var_ptwg_i_dn4_slot = var_ptwg_i_dn4;
        *var_ptwg_i_dn5_slot = var_ptwg_i_dn5;
        *var_ptwg_i_dn6_slot = var_ptwg_i_dn6;
        *var_ptwg_i_dn7_slot = var_ptwg_i_dn7;
        *var_ptwg_i_dn8_slot = var_ptwg_i_dn8;
        *var_ptwg_i_dn9_slot = var_ptwg_i_dn9;
        *var_rdsw_i_slot = var_rdsw_i;
        *var_rdsw_i_dn0_slot = var_rdsw_i_dn0;
        *var_rdsw_i_dn10_slot = var_rdsw_i_dn10;
        *var_rdsw_i_dn11_slot = var_rdsw_i_dn11;
        *var_rdsw_i_dn13_slot = var_rdsw_i_dn13;
        *var_rdsw_i_dn14_slot = var_rdsw_i_dn14;
        *var_rdsw_i_dn2_slot = var_rdsw_i_dn2;
        *var_rdsw_i_dn3_slot = var_rdsw_i_dn3;
        *var_rdsw_i_dn4_slot = var_rdsw_i_dn4;
        *var_rdsw_i_dn5_slot = var_rdsw_i_dn5;
        *var_rdsw_i_dn6_slot = var_rdsw_i_dn6;
        *var_rdsw_i_dn7_slot = var_rdsw_i_dn7;
        *var_rdsw_i_dn8_slot = var_rdsw_i_dn8;
        *var_rdsw_i_dn9_slot = var_rdsw_i_dn9;
        *var_rsw_i_slot = var_rsw_i;
        *var_rsw_i_dn0_slot = var_rsw_i_dn0;
        *var_rsw_i_dn10_slot = var_rsw_i_dn10;
        *var_rsw_i_dn11_slot = var_rsw_i_dn11;
        *var_rsw_i_dn13_slot = var_rsw_i_dn13;
        *var_rsw_i_dn14_slot = var_rsw_i_dn14;
        *var_rsw_i_dn2_slot = var_rsw_i_dn2;
        *var_rsw_i_dn3_slot = var_rsw_i_dn3;
        *var_rsw_i_dn4_slot = var_rsw_i_dn4;
        *var_rsw_i_dn5_slot = var_rsw_i_dn5;
        *var_rsw_i_dn6_slot = var_rsw_i_dn6;
        *var_rsw_i_dn7_slot = var_rsw_i_dn7;
        *var_rsw_i_dn8_slot = var_rsw_i_dn8;
        *var_rsw_i_dn9_slot = var_rsw_i_dn9;
        *var_u0_i_slot = var_u0_i;
        *var_u0_i_dn0_slot = var_u0_i_dn0;
        *var_u0_i_dn10_slot = var_u0_i_dn10;
        *var_u0_i_dn11_slot = var_u0_i_dn11;
        *var_u0_i_dn13_slot = var_u0_i_dn13;
        *var_u0_i_dn14_slot = var_u0_i_dn14;
        *var_u0_i_dn2_slot = var_u0_i_dn2;
        *var_u0_i_dn3_slot = var_u0_i_dn3;
        *var_u0_i_dn4_slot = var_u0_i_dn4;
        *var_u0_i_dn5_slot = var_u0_i_dn5;
        *var_u0_i_dn6_slot = var_u0_i_dn6;
        *var_u0_i_dn7_slot = var_u0_i_dn7;
        *var_u0_i_dn8_slot = var_u0_i_dn8;
        *var_u0_i_dn9_slot = var_u0_i_dn9;
        *var_ua_i_slot = var_ua_i;
        *var_ua_i_dn0_slot = var_ua_i_dn0;
        *var_ua_i_dn10_slot = var_ua_i_dn10;
        *var_ua_i_dn11_slot = var_ua_i_dn11;
        *var_ua_i_dn13_slot = var_ua_i_dn13;
        *var_ua_i_dn14_slot = var_ua_i_dn14;
        *var_ua_i_dn2_slot = var_ua_i_dn2;
        *var_ua_i_dn3_slot = var_ua_i_dn3;
        *var_ua_i_dn4_slot = var_ua_i_dn4;
        *var_ua_i_dn5_slot = var_ua_i_dn5;
        *var_ua_i_dn6_slot = var_ua_i_dn6;
        *var_ua_i_dn7_slot = var_ua_i_dn7;
        *var_ua_i_dn8_slot = var_ua_i_dn8;
        *var_ua_i_dn9_slot = var_ua_i_dn9;
        *var_ucs_i_slot = var_ucs_i;
        *var_ud_i_slot = var_ud_i;
        *var_ud_i_dn0_slot = var_ud_i_dn0;
        *var_ud_i_dn10_slot = var_ud_i_dn10;
        *var_ud_i_dn11_slot = var_ud_i_dn11;
        *var_ud_i_dn13_slot = var_ud_i_dn13;
        *var_ud_i_dn14_slot = var_ud_i_dn14;
        *var_ud_i_dn2_slot = var_ud_i_dn2;
        *var_ud_i_dn3_slot = var_ud_i_dn3;
        *var_ud_i_dn4_slot = var_ud_i_dn4;
        *var_ud_i_dn5_slot = var_ud_i_dn5;
        *var_ud_i_dn6_slot = var_ud_i_dn6;
        *var_ud_i_dn7_slot = var_ud_i_dn7;
        *var_ud_i_dn8_slot = var_ud_i_dn8;
        *var_ud_i_dn9_slot = var_ud_i_dn9;
        *var_udd1_i_slot = var_udd1_i;
        *var_udd_i_slot = var_udd_i;
        *var_uds1_i_slot = var_uds1_i;
        *var_uds_i_slot = var_uds_i;
        *var_up_i_slot = var_up_i;
        *var_vsatcv_i_slot = var_vsatcv_i;
        *var_vsatcv_i_dn0_slot = var_vsatcv_i_dn0;
        *var_vsatcv_i_dn10_slot = var_vsatcv_i_dn10;
        *var_vsatcv_i_dn11_slot = var_vsatcv_i_dn11;
        *var_vsatcv_i_dn13_slot = var_vsatcv_i_dn13;
        *var_vsatcv_i_dn14_slot = var_vsatcv_i_dn14;
        *var_vsatcv_i_dn2_slot = var_vsatcv_i_dn2;
        *var_vsatcv_i_dn3_slot = var_vsatcv_i_dn3;
        *var_vsatcv_i_dn4_slot = var_vsatcv_i_dn4;
        *var_vsatcv_i_dn5_slot = var_vsatcv_i_dn5;
        *var_vsatcv_i_dn6_slot = var_vsatcv_i_dn6;
        *var_vsatcv_i_dn7_slot = var_vsatcv_i_dn7;
        *var_vsatcv_i_dn8_slot = var_vsatcv_i_dn8;
        *var_vsatcv_i_dn9_slot = var_vsatcv_i_dn9;
    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        var_inv_l: f64,
        var_inv_lnfin: f64,
        var_inv_nfin: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_agidl_i_slot: &mut f64,
        var_aigbacc1_i_slot: &mut f64,
        var_aigbacc_i_slot: &mut f64,
        var_aigbinv1_i_slot: &mut f64,
        var_aigbinv_i_slot: &mut f64,
        var_aigc1_i_slot: &mut f64,
        var_aigc_i_slot: &mut f64,
        var_aigd1_i_slot: &mut f64,
        var_aigd_i_slot: &mut f64,
        var_aigs1_i_slot: &mut f64,
        var_aigs_i_slot: &mut f64,
        var_atatd_i_slot: &mut f64,
        var_bgidl_i_slot: &mut f64,
        var_bigbacc_i_slot: &mut f64,
        var_bigbinv_i_slot: &mut f64,
        var_bigc_i_slot: &mut f64,
        var_bigd_i_slot: &mut f64,
        var_bigs_i_slot: &mut f64,
        var_btatd_i_slot: &mut f64,
        var_cgidl_i_slot: &mut f64,
        var_cigbacc_i_slot: &mut f64,
        var_cigbinv_i_slot: &mut f64,
        var_cigc_i_slot: &mut f64,
        var_cigd_i_slot: &mut f64,
        var_cigs_i_slot: &mut f64,
        var_drout_i_slot: &mut f64,
        var_egidl_i_slot: &mut f64,
        var_eigbinv_i_slot: &mut f64,
        var_nigbacc_i_slot: &mut f64,
        var_nigbinv_i_slot: &mut f64,
        var_ntox_i_slot: &mut f64,
        var_pdibl1_i_slot: &mut f64,
        var_pdibl2_i_slot: &mut f64,
        var_pgidl_i_slot: &mut f64,
        var_pigcd_i_slot: &mut f64,
        var_poxedge_i_slot: &mut f64,
        var_prwgd_i_slot: &mut f64,
        var_prwgs_i_slot: &mut f64,
        var_pvag_i_slot: &mut f64,
        var_rdw_i_slot: &mut f64,
        var_rdw_i_dn0_slot: &mut f64,
        var_rdw_i_dn10_slot: &mut f64,
        var_rdw_i_dn11_slot: &mut f64,
        var_rdw_i_dn13_slot: &mut f64,
        var_rdw_i_dn14_slot: &mut f64,
        var_rdw_i_dn2_slot: &mut f64,
        var_rdw_i_dn3_slot: &mut f64,
        var_rdw_i_dn4_slot: &mut f64,
        var_rdw_i_dn5_slot: &mut f64,
        var_rdw_i_dn6_slot: &mut f64,
        var_rdw_i_dn7_slot: &mut f64,
        var_rdw_i_dn8_slot: &mut f64,
        var_rdw_i_dn9_slot: &mut f64,
        var_wr_i_slot: &mut f64,
    ) {
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_aigbacc1_i: f64 = *var_aigbacc1_i_slot;
        let mut var_aigbacc_i: f64 = *var_aigbacc_i_slot;
        let mut var_aigbinv1_i: f64 = *var_aigbinv1_i_slot;
        let mut var_aigbinv_i: f64 = *var_aigbinv_i_slot;
        let mut var_aigc1_i: f64 = *var_aigc1_i_slot;
        let mut var_aigc_i: f64 = *var_aigc_i_slot;
        let mut var_aigd1_i: f64 = *var_aigd1_i_slot;
        let mut var_aigd_i: f64 = *var_aigd_i_slot;
        let mut var_aigs1_i: f64 = *var_aigs1_i_slot;
        let mut var_aigs_i: f64 = *var_aigs_i_slot;
        let mut var_atatd_i: f64 = *var_atatd_i_slot;
        let mut var_bgidl_i: f64 = *var_bgidl_i_slot;
        let mut var_bigbacc_i: f64 = *var_bigbacc_i_slot;
        let mut var_bigbinv_i: f64 = *var_bigbinv_i_slot;
        let mut var_bigc_i: f64 = *var_bigc_i_slot;
        let mut var_bigd_i: f64 = *var_bigd_i_slot;
        let mut var_bigs_i: f64 = *var_bigs_i_slot;
        let mut var_btatd_i: f64 = *var_btatd_i_slot;
        let mut var_cgidl_i: f64 = *var_cgidl_i_slot;
        let mut var_cigbacc_i: f64 = *var_cigbacc_i_slot;
        let mut var_cigbinv_i: f64 = *var_cigbinv_i_slot;
        let mut var_cigc_i: f64 = *var_cigc_i_slot;
        let mut var_cigd_i: f64 = *var_cigd_i_slot;
        let mut var_cigs_i: f64 = *var_cigs_i_slot;
        let mut var_drout_i: f64 = *var_drout_i_slot;
        let mut var_egidl_i: f64 = *var_egidl_i_slot;
        let mut var_eigbinv_i: f64 = *var_eigbinv_i_slot;
        let mut var_nigbacc_i: f64 = *var_nigbacc_i_slot;
        let mut var_nigbinv_i: f64 = *var_nigbinv_i_slot;
        let mut var_ntox_i: f64 = *var_ntox_i_slot;
        let mut var_pdibl1_i: f64 = *var_pdibl1_i_slot;
        let mut var_pdibl2_i: f64 = *var_pdibl2_i_slot;
        let mut var_pgidl_i: f64 = *var_pgidl_i_slot;
        let mut var_pigcd_i: f64 = *var_pigcd_i_slot;
        let mut var_poxedge_i: f64 = *var_poxedge_i_slot;
        let mut var_prwgd_i: f64 = *var_prwgd_i_slot;
        let mut var_prwgs_i: f64 = *var_prwgs_i_slot;
        let mut var_pvag_i: f64 = *var_pvag_i_slot;
        let mut var_rdw_i: f64 = *var_rdw_i_slot;
        let mut var_rdw_i_dn0: f64 = *var_rdw_i_dn0_slot;
        let mut var_rdw_i_dn10: f64 = *var_rdw_i_dn10_slot;
        let mut var_rdw_i_dn11: f64 = *var_rdw_i_dn11_slot;
        let mut var_rdw_i_dn13: f64 = *var_rdw_i_dn13_slot;
        let mut var_rdw_i_dn14: f64 = *var_rdw_i_dn14_slot;
        let mut var_rdw_i_dn2: f64 = *var_rdw_i_dn2_slot;
        let mut var_rdw_i_dn3: f64 = *var_rdw_i_dn3_slot;
        let mut var_rdw_i_dn4: f64 = *var_rdw_i_dn4_slot;
        let mut var_rdw_i_dn5: f64 = *var_rdw_i_dn5_slot;
        let mut var_rdw_i_dn6: f64 = *var_rdw_i_dn6_slot;
        let mut var_rdw_i_dn7: f64 = *var_rdw_i_dn7_slot;
        let mut var_rdw_i_dn8: f64 = *var_rdw_i_dn8_slot;
        let mut var_rdw_i_dn9: f64 = *var_rdw_i_dn9_slot;
        let mut var_wr_i: f64 = *var_wr_i_slot;

        let assign4690_e5979: f64 = (var_inv_l * p.p938);
        let assign4690_e5980: f64 = (p.p937 + assign4690_e5979);
        let assign4690_e5983: f64 = (var_inv_nfin * p.p939);
        let assign4690_e5984: f64 = (assign4690_e5980 + assign4690_e5983);
        let assign4690_e5987: f64 = (var_inv_lnfin * p.p940);
        let assign4690_e5988: f64 = (assign4690_e5984 + assign4690_e5987);
        let assign4690_e5991: f64 = (var_inv_w * p.p941);
        let assign4690_e5992: f64 = (assign4690_e5988 + assign4690_e5991);
        let assign4690_e5995: f64 = (var_inv_wl * p.p942);
        let assign4690_e5996: f64 = (assign4690_e5992 + assign4690_e5995);
        var_rdw_i = assign4690_e5996;
        var_rdw_i_dn0 = 0.0;
        var_rdw_i_dn2 = 0.0;
        var_rdw_i_dn3 = 0.0;
        var_rdw_i_dn4 = 0.0;
        var_rdw_i_dn5 = 0.0;
        var_rdw_i_dn6 = 0.0;
        var_rdw_i_dn7 = 0.0;
        var_rdw_i_dn8 = 0.0;
        var_rdw_i_dn9 = 0.0;
        var_rdw_i_dn10 = 0.0;
        var_rdw_i_dn11 = 0.0;
        var_rdw_i_dn13 = 0.0;
        var_rdw_i_dn14 = 0.0;

        let assign4700_e6000: f64 = (var_inv_l * p.p950);
        let assign4700_e6001: f64 = (p.p949 + assign4700_e6000);
        let assign4700_e6004: f64 = (var_inv_nfin * p.p951);
        let assign4700_e6005: f64 = (assign4700_e6001 + assign4700_e6004);
        let assign4700_e6008: f64 = (var_inv_lnfin * p.p952);
        let assign4700_e6009: f64 = (assign4700_e6005 + assign4700_e6008);
        let assign4700_e6012: f64 = (var_inv_w * p.p953);
        let assign4700_e6013: f64 = (assign4700_e6009 + assign4700_e6012);
        let assign4700_e6016: f64 = (var_inv_wl * p.p954);
        let assign4700_e6017: f64 = (assign4700_e6013 + assign4700_e6016);
        var_prwgd_i = assign4700_e6017;

        let assign4710_e6021: f64 = (var_inv_l * p.p944);
        let assign4710_e6022: f64 = (p.p943 + assign4710_e6021);
        let assign4710_e6025: f64 = (var_inv_nfin * p.p945);
        let assign4710_e6026: f64 = (assign4710_e6022 + assign4710_e6025);
        let assign4710_e6029: f64 = (var_inv_lnfin * p.p946);
        let assign4710_e6030: f64 = (assign4710_e6026 + assign4710_e6029);
        let assign4710_e6033: f64 = (var_inv_w * p.p947);
        let assign4710_e6034: f64 = (assign4710_e6030 + assign4710_e6033);
        let assign4710_e6037: f64 = (var_inv_wl * p.p948);
        let assign4710_e6038: f64 = (assign4710_e6034 + assign4710_e6037);
        var_prwgs_i = assign4710_e6038;

        let assign4720_e6042: f64 = (var_inv_l * p.p956);
        let assign4720_e6043: f64 = (p.p955 + assign4720_e6042);
        let assign4720_e6046: f64 = (var_inv_nfin * p.p957);
        let assign4720_e6047: f64 = (assign4720_e6043 + assign4720_e6046);
        let assign4720_e6050: f64 = (var_inv_lnfin * p.p958);
        let assign4720_e6051: f64 = (assign4720_e6047 + assign4720_e6050);
        let assign4720_e6054: f64 = (var_inv_w * p.p959);
        let assign4720_e6055: f64 = (assign4720_e6051 + assign4720_e6054);
        let assign4720_e6058: f64 = (var_inv_wl * p.p960);
        let assign4720_e6059: f64 = (assign4720_e6055 + assign4720_e6058);
        var_wr_i = assign4720_e6059;

        let assign4730_e6063: f64 = (var_inv_l * p.p986);
        let assign4730_e6064: f64 = (p.p985 + assign4730_e6063);
        let assign4730_e6067: f64 = (var_inv_nfin * p.p987);
        let assign4730_e6068: f64 = (assign4730_e6064 + assign4730_e6067);
        let assign4730_e6071: f64 = (var_inv_lnfin * p.p988);
        let assign4730_e6072: f64 = (assign4730_e6068 + assign4730_e6071);
        let assign4730_e6075: f64 = (var_inv_w * p.p989);
        let assign4730_e6076: f64 = (assign4730_e6072 + assign4730_e6075);
        let assign4730_e6079: f64 = (var_inv_wl * p.p990);
        let assign4730_e6080: f64 = (assign4730_e6076 + assign4730_e6079);
        var_pdibl1_i = assign4730_e6080;

        let assign4740_e6084: f64 = (var_inv_l * p.p992);
        let assign4740_e6085: f64 = (p.p991 + assign4740_e6084);
        let assign4740_e6088: f64 = (var_inv_nfin * p.p993);
        let assign4740_e6089: f64 = (assign4740_e6085 + assign4740_e6088);
        let assign4740_e6092: f64 = (var_inv_lnfin * p.p994);
        let assign4740_e6093: f64 = (assign4740_e6089 + assign4740_e6092);
        let assign4740_e6096: f64 = (var_inv_w * p.p995);
        let assign4740_e6097: f64 = (assign4740_e6093 + assign4740_e6096);
        let assign4740_e6100: f64 = (var_inv_wl * p.p996);
        let assign4740_e6101: f64 = (assign4740_e6097 + assign4740_e6100);
        var_pdibl2_i = assign4740_e6101;

        let assign4750_e6105: f64 = (var_inv_l * p.p1010);
        let assign4750_e6106: f64 = (p.p1009 + assign4750_e6105);
        let assign4750_e6109: f64 = (var_inv_nfin * p.p1011);
        let assign4750_e6110: f64 = (assign4750_e6106 + assign4750_e6109);
        let assign4750_e6113: f64 = (var_inv_lnfin * p.p1012);
        let assign4750_e6114: f64 = (assign4750_e6110 + assign4750_e6113);
        let assign4750_e6117: f64 = (var_inv_w * p.p1013);
        let assign4750_e6118: f64 = (assign4750_e6114 + assign4750_e6117);
        let assign4750_e6121: f64 = (var_inv_wl * p.p1014);
        let assign4750_e6122: f64 = (assign4750_e6118 + assign4750_e6121);
        var_drout_i = assign4750_e6122;

        let assign4760_e6126: f64 = (var_inv_l * p.p1016);
        let assign4760_e6127: f64 = (p.p1015 + assign4760_e6126);
        let assign4760_e6130: f64 = (var_inv_nfin * p.p1017);
        let assign4760_e6131: f64 = (assign4760_e6127 + assign4760_e6130);
        let assign4760_e6134: f64 = (var_inv_lnfin * p.p1018);
        let assign4760_e6135: f64 = (assign4760_e6131 + assign4760_e6134);
        let assign4760_e6138: f64 = (var_inv_w * p.p1019);
        let assign4760_e6139: f64 = (assign4760_e6135 + assign4760_e6138);
        let assign4760_e6142: f64 = (var_inv_wl * p.p1020);
        let assign4760_e6143: f64 = (assign4760_e6139 + assign4760_e6142);
        var_pvag_i = assign4760_e6143;

        let assign4770_e6147: f64 = (var_inv_l * p.p1120);
        let assign4770_e6148: f64 = (p.p1119 + assign4770_e6147);
        let assign4770_e6151: f64 = (var_inv_nfin * p.p1121);
        let assign4770_e6152: f64 = (assign4770_e6148 + assign4770_e6151);
        let assign4770_e6155: f64 = (var_inv_lnfin * p.p1122);
        let assign4770_e6156: f64 = (assign4770_e6152 + assign4770_e6155);
        let assign4770_e6159: f64 = (var_inv_w * p.p1123);
        let assign4770_e6160: f64 = (assign4770_e6156 + assign4770_e6159);
        let assign4770_e6163: f64 = (var_inv_wl * p.p1124);
        let assign4770_e6164: f64 = (assign4770_e6160 + assign4770_e6163);
        var_aigbinv_i = assign4770_e6164;

        let assign4780_e6168: f64 = (var_inv_l * p.p1126);
        let assign4780_e6169: f64 = (p.p1125 + assign4780_e6168);
        let assign4780_e6172: f64 = (var_inv_nfin * p.p1127);
        let assign4780_e6173: f64 = (assign4780_e6169 + assign4780_e6172);
        let assign4780_e6176: f64 = (var_inv_lnfin * p.p1128);
        let assign4780_e6177: f64 = (assign4780_e6173 + assign4780_e6176);
        let assign4780_e6180: f64 = (var_inv_w * p.p1129);
        let assign4780_e6181: f64 = (assign4780_e6177 + assign4780_e6180);
        let assign4780_e6184: f64 = (var_inv_wl * p.p1130);
        let assign4780_e6185: f64 = (assign4780_e6181 + assign4780_e6184);
        var_aigbinv1_i = assign4780_e6185;

        let assign4790_e6189: f64 = (var_inv_l * p.p1132);
        let assign4790_e6190: f64 = (p.p1131 + assign4790_e6189);
        let assign4790_e6193: f64 = (var_inv_nfin * p.p1133);
        let assign4790_e6194: f64 = (assign4790_e6190 + assign4790_e6193);
        let assign4790_e6197: f64 = (var_inv_lnfin * p.p1134);
        let assign4790_e6198: f64 = (assign4790_e6194 + assign4790_e6197);
        let assign4790_e6201: f64 = (var_inv_w * p.p1135);
        let assign4790_e6202: f64 = (assign4790_e6198 + assign4790_e6201);
        let assign4790_e6205: f64 = (var_inv_wl * p.p1136);
        let assign4790_e6206: f64 = (assign4790_e6202 + assign4790_e6205);
        var_bigbinv_i = assign4790_e6206;

        let assign4800_e6210: f64 = (var_inv_l * p.p1138);
        let assign4800_e6211: f64 = (p.p1137 + assign4800_e6210);
        let assign4800_e6214: f64 = (var_inv_nfin * p.p1139);
        let assign4800_e6215: f64 = (assign4800_e6211 + assign4800_e6214);
        let assign4800_e6218: f64 = (var_inv_lnfin * p.p1140);
        let assign4800_e6219: f64 = (assign4800_e6215 + assign4800_e6218);
        let assign4800_e6222: f64 = (var_inv_w * p.p1141);
        let assign4800_e6223: f64 = (assign4800_e6219 + assign4800_e6222);
        let assign4800_e6226: f64 = (var_inv_wl * p.p1142);
        let assign4800_e6227: f64 = (assign4800_e6223 + assign4800_e6226);
        var_cigbinv_i = assign4800_e6227;

        let assign4810_e6231: f64 = (var_inv_l * p.p1144);
        let assign4810_e6232: f64 = (p.p1143 + assign4810_e6231);
        let assign4810_e6235: f64 = (var_inv_nfin * p.p1145);
        let assign4810_e6236: f64 = (assign4810_e6232 + assign4810_e6235);
        let assign4810_e6239: f64 = (var_inv_lnfin * p.p1146);
        let assign4810_e6240: f64 = (assign4810_e6236 + assign4810_e6239);
        let assign4810_e6243: f64 = (var_inv_w * p.p1147);
        let assign4810_e6244: f64 = (assign4810_e6240 + assign4810_e6243);
        let assign4810_e6247: f64 = (var_inv_wl * p.p1148);
        let assign4810_e6248: f64 = (assign4810_e6244 + assign4810_e6247);
        var_eigbinv_i = assign4810_e6248;

        let assign4820_e6252: f64 = (var_inv_l * p.p1150);
        let assign4820_e6253: f64 = (p.p1149 + assign4820_e6252);
        let assign4820_e6256: f64 = (var_inv_nfin * p.p1151);
        let assign4820_e6257: f64 = (assign4820_e6253 + assign4820_e6256);
        let assign4820_e6260: f64 = (var_inv_lnfin * p.p1152);
        let assign4820_e6261: f64 = (assign4820_e6257 + assign4820_e6260);
        let assign4820_e6264: f64 = (var_inv_w * p.p1153);
        let assign4820_e6265: f64 = (assign4820_e6261 + assign4820_e6264);
        let assign4820_e6268: f64 = (var_inv_wl * p.p1154);
        let assign4820_e6269: f64 = (assign4820_e6265 + assign4820_e6268);
        var_nigbinv_i = assign4820_e6269;

        let assign4830_e6273: f64 = (var_inv_l * p.p1156);
        let assign4830_e6274: f64 = (p.p1155 + assign4830_e6273);
        let assign4830_e6277: f64 = (var_inv_nfin * p.p1157);
        let assign4830_e6278: f64 = (assign4830_e6274 + assign4830_e6277);
        let assign4830_e6281: f64 = (var_inv_lnfin * p.p1158);
        let assign4830_e6282: f64 = (assign4830_e6278 + assign4830_e6281);
        let assign4830_e6285: f64 = (var_inv_w * p.p1159);
        let assign4830_e6286: f64 = (assign4830_e6282 + assign4830_e6285);
        let assign4830_e6289: f64 = (var_inv_wl * p.p1160);
        let assign4830_e6290: f64 = (assign4830_e6286 + assign4830_e6289);
        var_aigbacc_i = assign4830_e6290;

        let assign4840_e6294: f64 = (var_inv_l * p.p1162);
        let assign4840_e6295: f64 = (p.p1161 + assign4840_e6294);
        let assign4840_e6298: f64 = (var_inv_nfin * p.p1163);
        let assign4840_e6299: f64 = (assign4840_e6295 + assign4840_e6298);
        let assign4840_e6302: f64 = (var_inv_lnfin * p.p1164);
        let assign4840_e6303: f64 = (assign4840_e6299 + assign4840_e6302);
        let assign4840_e6306: f64 = (var_inv_w * p.p1165);
        let assign4840_e6307: f64 = (assign4840_e6303 + assign4840_e6306);
        let assign4840_e6310: f64 = (var_inv_wl * p.p1166);
        let assign4840_e6311: f64 = (assign4840_e6307 + assign4840_e6310);
        var_aigbacc1_i = assign4840_e6311;

        let assign4850_e6315: f64 = (var_inv_l * p.p1168);
        let assign4850_e6316: f64 = (p.p1167 + assign4850_e6315);
        let assign4850_e6319: f64 = (var_inv_nfin * p.p1169);
        let assign4850_e6320: f64 = (assign4850_e6316 + assign4850_e6319);
        let assign4850_e6323: f64 = (var_inv_lnfin * p.p1170);
        let assign4850_e6324: f64 = (assign4850_e6320 + assign4850_e6323);
        let assign4850_e6327: f64 = (var_inv_w * p.p1171);
        let assign4850_e6328: f64 = (assign4850_e6324 + assign4850_e6327);
        let assign4850_e6331: f64 = (var_inv_wl * p.p1172);
        let assign4850_e6332: f64 = (assign4850_e6328 + assign4850_e6331);
        var_bigbacc_i = assign4850_e6332;

        let assign4860_e6336: f64 = (var_inv_l * p.p1174);
        let assign4860_e6337: f64 = (p.p1173 + assign4860_e6336);
        let assign4860_e6340: f64 = (var_inv_nfin * p.p1175);
        let assign4860_e6341: f64 = (assign4860_e6337 + assign4860_e6340);
        let assign4860_e6344: f64 = (var_inv_lnfin * p.p1176);
        let assign4860_e6345: f64 = (assign4860_e6341 + assign4860_e6344);
        let assign4860_e6348: f64 = (var_inv_w * p.p1177);
        let assign4860_e6349: f64 = (assign4860_e6345 + assign4860_e6348);
        let assign4860_e6352: f64 = (var_inv_wl * p.p1178);
        let assign4860_e6353: f64 = (assign4860_e6349 + assign4860_e6352);
        var_cigbacc_i = assign4860_e6353;

        let assign4870_e6357: f64 = (var_inv_l * p.p1180);
        let assign4870_e6358: f64 = (p.p1179 + assign4870_e6357);
        let assign4870_e6361: f64 = (var_inv_nfin * p.p1181);
        let assign4870_e6362: f64 = (assign4870_e6358 + assign4870_e6361);
        let assign4870_e6365: f64 = (var_inv_lnfin * p.p1182);
        let assign4870_e6366: f64 = (assign4870_e6362 + assign4870_e6365);
        let assign4870_e6369: f64 = (var_inv_w * p.p1183);
        let assign4870_e6370: f64 = (assign4870_e6366 + assign4870_e6369);
        let assign4870_e6373: f64 = (var_inv_wl * p.p1184);
        let assign4870_e6374: f64 = (assign4870_e6370 + assign4870_e6373);
        var_nigbacc_i = assign4870_e6374;

        let assign4880_e6378: f64 = (var_inv_l * p.p1186);
        let assign4880_e6379: f64 = (p.p1185 + assign4880_e6378);
        let assign4880_e6382: f64 = (var_inv_nfin * p.p1187);
        let assign4880_e6383: f64 = (assign4880_e6379 + assign4880_e6382);
        let assign4880_e6386: f64 = (var_inv_lnfin * p.p1188);
        let assign4880_e6387: f64 = (assign4880_e6383 + assign4880_e6386);
        let assign4880_e6390: f64 = (var_inv_w * p.p1189);
        let assign4880_e6391: f64 = (assign4880_e6387 + assign4880_e6390);
        let assign4880_e6394: f64 = (var_inv_wl * p.p1190);
        let assign4880_e6395: f64 = (assign4880_e6391 + assign4880_e6394);
        var_aigc_i = assign4880_e6395;

        let assign4890_e6399: f64 = (var_inv_l * p.p1192);
        let assign4890_e6400: f64 = (p.p1191 + assign4890_e6399);
        let assign4890_e6403: f64 = (var_inv_nfin * p.p1193);
        let assign4890_e6404: f64 = (assign4890_e6400 + assign4890_e6403);
        let assign4890_e6407: f64 = (var_inv_lnfin * p.p1194);
        let assign4890_e6408: f64 = (assign4890_e6404 + assign4890_e6407);
        let assign4890_e6411: f64 = (var_inv_w * p.p1195);
        let assign4890_e6412: f64 = (assign4890_e6408 + assign4890_e6411);
        let assign4890_e6415: f64 = (var_inv_wl * p.p1196);
        let assign4890_e6416: f64 = (assign4890_e6412 + assign4890_e6415);
        var_aigc1_i = assign4890_e6416;

        let assign4900_e6420: f64 = (var_inv_l * p.p1198);
        let assign4900_e6421: f64 = (p.p1197 + assign4900_e6420);
        let assign4900_e6424: f64 = (var_inv_nfin * p.p1199);
        let assign4900_e6425: f64 = (assign4900_e6421 + assign4900_e6424);
        let assign4900_e6428: f64 = (var_inv_lnfin * p.p1200);
        let assign4900_e6429: f64 = (assign4900_e6425 + assign4900_e6428);
        let assign4900_e6432: f64 = (var_inv_w * p.p1201);
        let assign4900_e6433: f64 = (assign4900_e6429 + assign4900_e6432);
        let assign4900_e6436: f64 = (var_inv_wl * p.p1202);
        let assign4900_e6437: f64 = (assign4900_e6433 + assign4900_e6436);
        var_bigc_i = assign4900_e6437;

        let assign4910_e6441: f64 = (var_inv_l * p.p1204);
        let assign4910_e6442: f64 = (p.p1203 + assign4910_e6441);
        let assign4910_e6445: f64 = (var_inv_nfin * p.p1205);
        let assign4910_e6446: f64 = (assign4910_e6442 + assign4910_e6445);
        let assign4910_e6449: f64 = (var_inv_lnfin * p.p1206);
        let assign4910_e6450: f64 = (assign4910_e6446 + assign4910_e6449);
        let assign4910_e6453: f64 = (var_inv_w * p.p1207);
        let assign4910_e6454: f64 = (assign4910_e6450 + assign4910_e6453);
        let assign4910_e6457: f64 = (var_inv_wl * p.p1208);
        let assign4910_e6458: f64 = (assign4910_e6454 + assign4910_e6457);
        var_cigc_i = assign4910_e6458;

        let assign4920_e6462: f64 = (var_inv_l * p.p1210);
        let assign4920_e6463: f64 = (p.p1209 + assign4920_e6462);
        let assign4920_e6466: f64 = (var_inv_nfin * p.p1211);
        let assign4920_e6467: f64 = (assign4920_e6463 + assign4920_e6466);
        let assign4920_e6470: f64 = (var_inv_lnfin * p.p1212);
        let assign4920_e6471: f64 = (assign4920_e6467 + assign4920_e6470);
        let assign4920_e6474: f64 = (var_inv_w * p.p1213);
        let assign4920_e6475: f64 = (assign4920_e6471 + assign4920_e6474);
        let assign4920_e6478: f64 = (var_inv_wl * p.p1214);
        let assign4920_e6479: f64 = (assign4920_e6475 + assign4920_e6478);
        var_pigcd_i = assign4920_e6479;

        let assign4930_e6483: f64 = (var_inv_l * p.p1216);
        let assign4930_e6484: f64 = (p.p1215 + assign4930_e6483);
        let assign4930_e6487: f64 = (var_inv_nfin * p.p1217);
        let assign4930_e6488: f64 = (assign4930_e6484 + assign4930_e6487);
        let assign4930_e6491: f64 = (var_inv_lnfin * p.p1218);
        let assign4930_e6492: f64 = (assign4930_e6488 + assign4930_e6491);
        let assign4930_e6495: f64 = (var_inv_w * p.p1219);
        let assign4930_e6496: f64 = (assign4930_e6492 + assign4930_e6495);
        let assign4930_e6499: f64 = (var_inv_wl * p.p1220);
        let assign4930_e6500: f64 = (assign4930_e6496 + assign4930_e6499);
        var_aigs_i = assign4930_e6500;

        let assign4940_e6504: f64 = (var_inv_l * p.p1222);
        let assign4940_e6505: f64 = (p.p1221 + assign4940_e6504);
        let assign4940_e6508: f64 = (var_inv_nfin * p.p1223);
        let assign4940_e6509: f64 = (assign4940_e6505 + assign4940_e6508);
        let assign4940_e6512: f64 = (var_inv_lnfin * p.p1224);
        let assign4940_e6513: f64 = (assign4940_e6509 + assign4940_e6512);
        let assign4940_e6516: f64 = (var_inv_w * p.p1225);
        let assign4940_e6517: f64 = (assign4940_e6513 + assign4940_e6516);
        let assign4940_e6520: f64 = (var_inv_wl * p.p1226);
        let assign4940_e6521: f64 = (assign4940_e6517 + assign4940_e6520);
        var_aigs1_i = assign4940_e6521;

        let assign4950_e6525: f64 = (var_inv_l * p.p1228);
        let assign4950_e6526: f64 = (p.p1227 + assign4950_e6525);
        let assign4950_e6529: f64 = (var_inv_nfin * p.p1229);
        let assign4950_e6530: f64 = (assign4950_e6526 + assign4950_e6529);
        let assign4950_e6533: f64 = (var_inv_lnfin * p.p1230);
        let assign4950_e6534: f64 = (assign4950_e6530 + assign4950_e6533);
        let assign4950_e6537: f64 = (var_inv_w * p.p1231);
        let assign4950_e6538: f64 = (assign4950_e6534 + assign4950_e6537);
        let assign4950_e6541: f64 = (var_inv_wl * p.p1232);
        let assign4950_e6542: f64 = (assign4950_e6538 + assign4950_e6541);
        var_bigs_i = assign4950_e6542;

        let assign4960_e6546: f64 = (var_inv_l * p.p1234);
        let assign4960_e6547: f64 = (p.p1233 + assign4960_e6546);
        let assign4960_e6550: f64 = (var_inv_nfin * p.p1235);
        let assign4960_e6551: f64 = (assign4960_e6547 + assign4960_e6550);
        let assign4960_e6554: f64 = (var_inv_lnfin * p.p1236);
        let assign4960_e6555: f64 = (assign4960_e6551 + assign4960_e6554);
        let assign4960_e6558: f64 = (var_inv_w * p.p1237);
        let assign4960_e6559: f64 = (assign4960_e6555 + assign4960_e6558);
        let assign4960_e6562: f64 = (var_inv_wl * p.p1238);
        let assign4960_e6563: f64 = (assign4960_e6559 + assign4960_e6562);
        var_cigs_i = assign4960_e6563;

        let assign4970_e6567: f64 = (var_inv_l * p.p1240);
        let assign4970_e6568: f64 = (p.p1239 + assign4970_e6567);
        let assign4970_e6571: f64 = (var_inv_nfin * p.p1241);
        let assign4970_e6572: f64 = (assign4970_e6568 + assign4970_e6571);
        let assign4970_e6575: f64 = (var_inv_lnfin * p.p1242);
        let assign4970_e6576: f64 = (assign4970_e6572 + assign4970_e6575);
        let assign4970_e6579: f64 = (var_inv_w * p.p1243);
        let assign4970_e6580: f64 = (assign4970_e6576 + assign4970_e6579);
        let assign4970_e6583: f64 = (var_inv_wl * p.p1244);
        let assign4970_e6584: f64 = (assign4970_e6580 + assign4970_e6583);
        var_aigd_i = assign4970_e6584;

        let assign4980_e6588: f64 = (var_inv_l * p.p1246);
        let assign4980_e6589: f64 = (p.p1245 + assign4980_e6588);
        let assign4980_e6592: f64 = (var_inv_nfin * p.p1247);
        let assign4980_e6593: f64 = (assign4980_e6589 + assign4980_e6592);
        let assign4980_e6596: f64 = (var_inv_lnfin * p.p1248);
        let assign4980_e6597: f64 = (assign4980_e6593 + assign4980_e6596);
        let assign4980_e6600: f64 = (var_inv_w * p.p1249);
        let assign4980_e6601: f64 = (assign4980_e6597 + assign4980_e6600);
        let assign4980_e6604: f64 = (var_inv_wl * p.p1250);
        let assign4980_e6605: f64 = (assign4980_e6601 + assign4980_e6604);
        var_aigd1_i = assign4980_e6605;

        let assign4990_e6609: f64 = (var_inv_l * p.p1252);
        let assign4990_e6610: f64 = (p.p1251 + assign4990_e6609);
        let assign4990_e6613: f64 = (var_inv_nfin * p.p1253);
        let assign4990_e6614: f64 = (assign4990_e6610 + assign4990_e6613);
        let assign4990_e6617: f64 = (var_inv_lnfin * p.p1254);
        let assign4990_e6618: f64 = (assign4990_e6614 + assign4990_e6617);
        let assign4990_e6621: f64 = (var_inv_w * p.p1255);
        let assign4990_e6622: f64 = (assign4990_e6618 + assign4990_e6621);
        let assign4990_e6625: f64 = (var_inv_wl * p.p1256);
        let assign4990_e6626: f64 = (assign4990_e6622 + assign4990_e6625);
        var_bigd_i = assign4990_e6626;

        let assign5000_e6630: f64 = (var_inv_l * p.p1258);
        let assign5000_e6631: f64 = (p.p1257 + assign5000_e6630);
        let assign5000_e6634: f64 = (var_inv_nfin * p.p1259);
        let assign5000_e6635: f64 = (assign5000_e6631 + assign5000_e6634);
        let assign5000_e6638: f64 = (var_inv_lnfin * p.p1260);
        let assign5000_e6639: f64 = (assign5000_e6635 + assign5000_e6638);
        let assign5000_e6642: f64 = (var_inv_w * p.p1261);
        let assign5000_e6643: f64 = (assign5000_e6639 + assign5000_e6642);
        let assign5000_e6646: f64 = (var_inv_wl * p.p1262);
        let assign5000_e6647: f64 = (assign5000_e6643 + assign5000_e6646);
        var_cigd_i = assign5000_e6647;

        let assign5010_e6651: f64 = (var_inv_l * p.p1114);
        let assign5010_e6652: f64 = (p.p1113 + assign5010_e6651);
        let assign5010_e6655: f64 = (var_inv_nfin * p.p1115);
        let assign5010_e6656: f64 = (assign5010_e6652 + assign5010_e6655);
        let assign5010_e6659: f64 = (var_inv_lnfin * p.p1116);
        let assign5010_e6660: f64 = (assign5010_e6656 + assign5010_e6659);
        let assign5010_e6663: f64 = (var_inv_w * p.p1117);
        let assign5010_e6664: f64 = (assign5010_e6660 + assign5010_e6663);
        let assign5010_e6667: f64 = (var_inv_wl * p.p1118);
        let assign5010_e6668: f64 = (assign5010_e6664 + assign5010_e6667);
        var_ntox_i = assign5010_e6668;

        let assign5020_e6672: f64 = (var_inv_l * p.p1264);
        let assign5020_e6673: f64 = (p.p1263 + assign5020_e6672);
        let assign5020_e6676: f64 = (var_inv_nfin * p.p1265);
        let assign5020_e6677: f64 = (assign5020_e6673 + assign5020_e6676);
        let assign5020_e6680: f64 = (var_inv_lnfin * p.p1266);
        let assign5020_e6681: f64 = (assign5020_e6677 + assign5020_e6680);
        let assign5020_e6684: f64 = (var_inv_w * p.p1267);
        let assign5020_e6685: f64 = (assign5020_e6681 + assign5020_e6684);
        let assign5020_e6688: f64 = (var_inv_wl * p.p1268);
        let assign5020_e6689: f64 = (assign5020_e6685 + assign5020_e6688);
        var_poxedge_i = assign5020_e6689;

        let assign5030_e6693: f64 = (var_inv_l * p.p1270);
        let assign5030_e6694: f64 = (p.p1269 + assign5030_e6693);
        let assign5030_e6697: f64 = (var_inv_nfin * p.p1271);
        let assign5030_e6698: f64 = (assign5030_e6694 + assign5030_e6697);
        let assign5030_e6701: f64 = (var_inv_lnfin * p.p1272);
        let assign5030_e6702: f64 = (assign5030_e6698 + assign5030_e6701);
        let assign5030_e6705: f64 = (var_inv_w * p.p1273);
        let assign5030_e6706: f64 = (assign5030_e6702 + assign5030_e6705);
        let assign5030_e6709: f64 = (var_inv_wl * p.p1274);
        let assign5030_e6710: f64 = (assign5030_e6706 + assign5030_e6709);
        var_agidl_i = assign5030_e6710;

        let assign5040_e6714: f64 = (var_inv_l * p.p1276);
        let assign5040_e6715: f64 = (p.p1275 + assign5040_e6714);
        let assign5040_e6718: f64 = (var_inv_nfin * p.p1277);
        let assign5040_e6719: f64 = (assign5040_e6715 + assign5040_e6718);
        let assign5040_e6722: f64 = (var_inv_lnfin * p.p1278);
        let assign5040_e6723: f64 = (assign5040_e6719 + assign5040_e6722);
        let assign5040_e6726: f64 = (var_inv_w * p.p1279);
        let assign5040_e6727: f64 = (assign5040_e6723 + assign5040_e6726);
        let assign5040_e6730: f64 = (var_inv_wl * p.p1280);
        let assign5040_e6731: f64 = (assign5040_e6727 + assign5040_e6730);
        var_bgidl_i = assign5040_e6731;

        let assign5050_e6735: f64 = (var_inv_l * p.p1282);
        let assign5050_e6736: f64 = (p.p1281 + assign5050_e6735);
        let assign5050_e6739: f64 = (var_inv_nfin * p.p1283);
        let assign5050_e6740: f64 = (assign5050_e6736 + assign5050_e6739);
        let assign5050_e6743: f64 = (var_inv_lnfin * p.p1284);
        let assign5050_e6744: f64 = (assign5050_e6740 + assign5050_e6743);
        let assign5050_e6747: f64 = (var_inv_w * p.p1285);
        let assign5050_e6748: f64 = (assign5050_e6744 + assign5050_e6747);
        let assign5050_e6751: f64 = (var_inv_wl * p.p1286);
        let assign5050_e6752: f64 = (assign5050_e6748 + assign5050_e6751);
        var_cgidl_i = assign5050_e6752;

        let assign5060_e6756: f64 = (var_inv_l * p.p1288);
        let assign5060_e6757: f64 = (p.p1287 + assign5060_e6756);
        let assign5060_e6760: f64 = (var_inv_nfin * p.p1289);
        let assign5060_e6761: f64 = (assign5060_e6757 + assign5060_e6760);
        let assign5060_e6764: f64 = (var_inv_lnfin * p.p1290);
        let assign5060_e6765: f64 = (assign5060_e6761 + assign5060_e6764);
        let assign5060_e6768: f64 = (var_inv_w * p.p1291);
        let assign5060_e6769: f64 = (assign5060_e6765 + assign5060_e6768);
        let assign5060_e6772: f64 = (var_inv_wl * p.p1292);
        let assign5060_e6773: f64 = (assign5060_e6769 + assign5060_e6772);
        var_egidl_i = assign5060_e6773;

        let assign5070_e6777: f64 = (var_inv_l * p.p1294);
        let assign5070_e6778: f64 = (p.p1293 + assign5070_e6777);
        let assign5070_e6781: f64 = (var_inv_nfin * p.p1295);
        let assign5070_e6782: f64 = (assign5070_e6778 + assign5070_e6781);
        let assign5070_e6785: f64 = (var_inv_lnfin * p.p1296);
        let assign5070_e6786: f64 = (assign5070_e6782 + assign5070_e6785);
        let assign5070_e6789: f64 = (var_inv_w * p.p1297);
        let assign5070_e6790: f64 = (assign5070_e6786 + assign5070_e6789);
        let assign5070_e6793: f64 = (var_inv_wl * p.p1298);
        let assign5070_e6794: f64 = (assign5070_e6790 + assign5070_e6793);
        var_pgidl_i = assign5070_e6794;

        let assign5080_e6798: f64 = (var_inv_l * p.p1330);
        let assign5080_e6799: f64 = (p.p1329 + assign5080_e6798);
        let assign5080_e6802: f64 = (var_inv_nfin * p.p1331);
        let assign5080_e6803: f64 = (assign5080_e6799 + assign5080_e6802);
        let assign5080_e6806: f64 = (var_inv_lnfin * p.p1332);
        let assign5080_e6807: f64 = (assign5080_e6803 + assign5080_e6806);
        let assign5080_e6810: f64 = (var_inv_w * p.p1333);
        let assign5080_e6811: f64 = (assign5080_e6807 + assign5080_e6810);
        let assign5080_e6814: f64 = (var_inv_wl * p.p1334);
        let assign5080_e6815: f64 = (assign5080_e6811 + assign5080_e6814);
        var_atatd_i = assign5080_e6815;

        let assign5090_e6819: f64 = (var_inv_l * p.p1336);
        let assign5090_e6820: f64 = (p.p1335 + assign5090_e6819);
        let assign5090_e6823: f64 = (var_inv_nfin * p.p1337);
        let assign5090_e6824: f64 = (assign5090_e6820 + assign5090_e6823);
        let assign5090_e6827: f64 = (var_inv_lnfin * p.p1338);
        let assign5090_e6828: f64 = (assign5090_e6824 + assign5090_e6827);
        let assign5090_e6831: f64 = (var_inv_w * p.p1339);
        let assign5090_e6832: f64 = (assign5090_e6828 + assign5090_e6831);
        let assign5090_e6835: f64 = (var_inv_wl * p.p1340);
        let assign5090_e6836: f64 = (assign5090_e6832 + assign5090_e6835);
        var_btatd_i = assign5090_e6836;

        *var_agidl_i_slot = var_agidl_i;
        *var_aigbacc1_i_slot = var_aigbacc1_i;
        *var_aigbacc_i_slot = var_aigbacc_i;
        *var_aigbinv1_i_slot = var_aigbinv1_i;
        *var_aigbinv_i_slot = var_aigbinv_i;
        *var_aigc1_i_slot = var_aigc1_i;
        *var_aigc_i_slot = var_aigc_i;
        *var_aigd1_i_slot = var_aigd1_i;
        *var_aigd_i_slot = var_aigd_i;
        *var_aigs1_i_slot = var_aigs1_i;
        *var_aigs_i_slot = var_aigs_i;
        *var_atatd_i_slot = var_atatd_i;
        *var_bgidl_i_slot = var_bgidl_i;
        *var_bigbacc_i_slot = var_bigbacc_i;
        *var_bigbinv_i_slot = var_bigbinv_i;
        *var_bigc_i_slot = var_bigc_i;
        *var_bigd_i_slot = var_bigd_i;
        *var_bigs_i_slot = var_bigs_i;
        *var_btatd_i_slot = var_btatd_i;
        *var_cgidl_i_slot = var_cgidl_i;
        *var_cigbacc_i_slot = var_cigbacc_i;
        *var_cigbinv_i_slot = var_cigbinv_i;
        *var_cigc_i_slot = var_cigc_i;
        *var_cigd_i_slot = var_cigd_i;
        *var_cigs_i_slot = var_cigs_i;
        *var_drout_i_slot = var_drout_i;
        *var_egidl_i_slot = var_egidl_i;
        *var_eigbinv_i_slot = var_eigbinv_i;
        *var_nigbacc_i_slot = var_nigbacc_i;
        *var_nigbinv_i_slot = var_nigbinv_i;
        *var_ntox_i_slot = var_ntox_i;
        *var_pdibl1_i_slot = var_pdibl1_i;
        *var_pdibl2_i_slot = var_pdibl2_i;
        *var_pgidl_i_slot = var_pgidl_i;
        *var_pigcd_i_slot = var_pigcd_i;
        *var_poxedge_i_slot = var_poxedge_i;
        *var_prwgd_i_slot = var_prwgd_i;
        *var_prwgs_i_slot = var_prwgs_i;
        *var_pvag_i_slot = var_pvag_i;
        *var_rdw_i_slot = var_rdw_i;
        *var_rdw_i_dn0_slot = var_rdw_i_dn0;
        *var_rdw_i_dn10_slot = var_rdw_i_dn10;
        *var_rdw_i_dn11_slot = var_rdw_i_dn11;
        *var_rdw_i_dn13_slot = var_rdw_i_dn13;
        *var_rdw_i_dn14_slot = var_rdw_i_dn14;
        *var_rdw_i_dn2_slot = var_rdw_i_dn2;
        *var_rdw_i_dn3_slot = var_rdw_i_dn3;
        *var_rdw_i_dn4_slot = var_rdw_i_dn4;
        *var_rdw_i_dn5_slot = var_rdw_i_dn5;
        *var_rdw_i_dn6_slot = var_rdw_i_dn6;
        *var_rdw_i_dn7_slot = var_rdw_i_dn7;
        *var_rdw_i_dn8_slot = var_rdw_i_dn8;
        *var_rdw_i_dn9_slot = var_rdw_i_dn9;
        *var_wr_i_slot = var_wr_i;
    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        var_inv_l: f64,
        var_inv_lnfin: f64,
        var_inv_nfin: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_agisl_i_slot: &mut f64,
        var_aigen_i_slot: &mut f64,
        var_alpha0_i_slot: &mut f64,
        var_alpha1_i_slot: &mut f64,
        var_alphaii0_i_slot: &mut f64,
        var_alphaii1_i_slot: &mut f64,
        var_atats_i_slot: &mut f64,
        var_beta0_i_slot: &mut f64,
        var_betaii0_i_slot: &mut f64,
        var_betaii1_i_slot: &mut f64,
        var_betaii2_i_slot: &mut f64,
        var_bgisl_i_slot: &mut f64,
        var_bigen_i_slot: &mut f64,
        var_btats_i_slot: &mut f64,
        var_cfd_i_slot: &mut f64,
        var_cfs_i_slot: &mut f64,
        var_cgbl_i_slot: &mut f64,
        var_cgdl_i_slot: &mut f64,
        var_cgisl_i_slot: &mut f64,
        var_cgsl_i_slot: &mut f64,
        var_ckappab_i_slot: &mut f64,
        var_ckappad_i_slot: &mut f64,
        var_ckappas_i_slot: &mut f64,
        var_covd_i_slot: &mut f64,
        var_covd_i_dn0_slot: &mut f64,
        var_covd_i_dn10_slot: &mut f64,
        var_covd_i_dn11_slot: &mut f64,
        var_covd_i_dn13_slot: &mut f64,
        var_covd_i_dn14_slot: &mut f64,
        var_covd_i_dn2_slot: &mut f64,
        var_covd_i_dn3_slot: &mut f64,
        var_covd_i_dn4_slot: &mut f64,
        var_covd_i_dn5_slot: &mut f64,
        var_covd_i_dn6_slot: &mut f64,
        var_covd_i_dn7_slot: &mut f64,
        var_covd_i_dn8_slot: &mut f64,
        var_covd_i_dn9_slot: &mut f64,
        var_covs_i_slot: &mut f64,
        var_covs_i_dn0_slot: &mut f64,
        var_covs_i_dn10_slot: &mut f64,
        var_covs_i_dn11_slot: &mut f64,
        var_covs_i_dn13_slot: &mut f64,
        var_covs_i_dn14_slot: &mut f64,
        var_covs_i_dn2_slot: &mut f64,
        var_covs_i_dn3_slot: &mut f64,
        var_covs_i_dn4_slot: &mut f64,
        var_covs_i_dn5_slot: &mut f64,
        var_covs_i_dn6_slot: &mut f64,
        var_covs_i_dn7_slot: &mut f64,
        var_covs_i_dn8_slot: &mut f64,
        var_covs_i_dn9_slot: &mut f64,
        var_ctatd_i_slot: &mut f64,
        var_ctats_i_slot: &mut f64,
        var_dtatd_i_slot: &mut f64,
        var_dtats_i_slot: &mut f64,
        var_egisl_i_slot: &mut f64,
        var_esatii_i_slot: &mut f64,
        var_lii_i_slot: &mut f64,
        var_ntgen_i_slot: &mut f64,
        var_pgisl_i_slot: &mut f64,
        var_sii0_i_slot: &mut f64,
        var_sii1_i_slot: &mut f64,
        var_sii2_i_slot: &mut f64,
        var_siid_i_slot: &mut f64,
        var_tii_i_slot: &mut f64,
        var_ute_i_slot: &mut f64,
    ) {
        let mut var_agisl_i: f64 = *var_agisl_i_slot;
        let mut var_aigen_i: f64 = *var_aigen_i_slot;
        let mut var_alpha0_i: f64 = *var_alpha0_i_slot;
        let mut var_alpha1_i: f64 = *var_alpha1_i_slot;
        let mut var_alphaii0_i: f64 = *var_alphaii0_i_slot;
        let mut var_alphaii1_i: f64 = *var_alphaii1_i_slot;
        let mut var_atats_i: f64 = *var_atats_i_slot;
        let mut var_beta0_i: f64 = *var_beta0_i_slot;
        let mut var_betaii0_i: f64 = *var_betaii0_i_slot;
        let mut var_betaii1_i: f64 = *var_betaii1_i_slot;
        let mut var_betaii2_i: f64 = *var_betaii2_i_slot;
        let mut var_bgisl_i: f64 = *var_bgisl_i_slot;
        let mut var_bigen_i: f64 = *var_bigen_i_slot;
        let mut var_btats_i: f64 = *var_btats_i_slot;
        let mut var_cfd_i: f64 = *var_cfd_i_slot;
        let mut var_cfs_i: f64 = *var_cfs_i_slot;
        let mut var_cgbl_i: f64 = *var_cgbl_i_slot;
        let mut var_cgdl_i: f64 = *var_cgdl_i_slot;
        let mut var_cgisl_i: f64 = *var_cgisl_i_slot;
        let mut var_cgsl_i: f64 = *var_cgsl_i_slot;
        let mut var_ckappab_i: f64 = *var_ckappab_i_slot;
        let mut var_ckappad_i: f64 = *var_ckappad_i_slot;
        let mut var_ckappas_i: f64 = *var_ckappas_i_slot;
        let mut var_covd_i: f64 = *var_covd_i_slot;
        let mut var_covd_i_dn0: f64 = *var_covd_i_dn0_slot;
        let mut var_covd_i_dn10: f64 = *var_covd_i_dn10_slot;
        let mut var_covd_i_dn11: f64 = *var_covd_i_dn11_slot;
        let mut var_covd_i_dn13: f64 = *var_covd_i_dn13_slot;
        let mut var_covd_i_dn14: f64 = *var_covd_i_dn14_slot;
        let mut var_covd_i_dn2: f64 = *var_covd_i_dn2_slot;
        let mut var_covd_i_dn3: f64 = *var_covd_i_dn3_slot;
        let mut var_covd_i_dn4: f64 = *var_covd_i_dn4_slot;
        let mut var_covd_i_dn5: f64 = *var_covd_i_dn5_slot;
        let mut var_covd_i_dn6: f64 = *var_covd_i_dn6_slot;
        let mut var_covd_i_dn7: f64 = *var_covd_i_dn7_slot;
        let mut var_covd_i_dn8: f64 = *var_covd_i_dn8_slot;
        let mut var_covd_i_dn9: f64 = *var_covd_i_dn9_slot;
        let mut var_covs_i: f64 = *var_covs_i_slot;
        let mut var_covs_i_dn0: f64 = *var_covs_i_dn0_slot;
        let mut var_covs_i_dn10: f64 = *var_covs_i_dn10_slot;
        let mut var_covs_i_dn11: f64 = *var_covs_i_dn11_slot;
        let mut var_covs_i_dn13: f64 = *var_covs_i_dn13_slot;
        let mut var_covs_i_dn14: f64 = *var_covs_i_dn14_slot;
        let mut var_covs_i_dn2: f64 = *var_covs_i_dn2_slot;
        let mut var_covs_i_dn3: f64 = *var_covs_i_dn3_slot;
        let mut var_covs_i_dn4: f64 = *var_covs_i_dn4_slot;
        let mut var_covs_i_dn5: f64 = *var_covs_i_dn5_slot;
        let mut var_covs_i_dn6: f64 = *var_covs_i_dn6_slot;
        let mut var_covs_i_dn7: f64 = *var_covs_i_dn7_slot;
        let mut var_covs_i_dn8: f64 = *var_covs_i_dn8_slot;
        let mut var_covs_i_dn9: f64 = *var_covs_i_dn9_slot;
        let mut var_ctatd_i: f64 = *var_ctatd_i_slot;
        let mut var_ctats_i: f64 = *var_ctats_i_slot;
        let mut var_dtatd_i: f64 = *var_dtatd_i_slot;
        let mut var_dtats_i: f64 = *var_dtats_i_slot;
        let mut var_egisl_i: f64 = *var_egisl_i_slot;
        let mut var_esatii_i: f64 = *var_esatii_i_slot;
        let mut var_lii_i: f64 = *var_lii_i_slot;
        let mut var_ntgen_i: f64 = *var_ntgen_i_slot;
        let mut var_pgisl_i: f64 = *var_pgisl_i_slot;
        let mut var_sii0_i: f64 = *var_sii0_i_slot;
        let mut var_sii1_i: f64 = *var_sii1_i_slot;
        let mut var_sii2_i: f64 = *var_sii2_i_slot;
        let mut var_siid_i: f64 = *var_siid_i_slot;
        let mut var_tii_i: f64 = *var_tii_i_slot;
        let mut var_ute_i: f64 = *var_ute_i_slot;

        let assign5100_e6840: f64 = (var_inv_l * p.p1342);
        let assign5100_e6841: f64 = (p.p1341 + assign5100_e6840);
        let assign5100_e6844: f64 = (var_inv_nfin * p.p1343);
        let assign5100_e6845: f64 = (assign5100_e6841 + assign5100_e6844);
        let assign5100_e6848: f64 = (var_inv_lnfin * p.p1344);
        let assign5100_e6849: f64 = (assign5100_e6845 + assign5100_e6848);
        let assign5100_e6852: f64 = (var_inv_w * p.p1345);
        let assign5100_e6853: f64 = (assign5100_e6849 + assign5100_e6852);
        let assign5100_e6856: f64 = (var_inv_wl * p.p1346);
        let assign5100_e6857: f64 = (assign5100_e6853 + assign5100_e6856);
        var_ctatd_i = assign5100_e6857;

        let assign5110_e6861: f64 = (var_inv_l * p.p1348);
        let assign5110_e6862: f64 = (p.p1347 + assign5110_e6861);
        let assign5110_e6865: f64 = (var_inv_nfin * p.p1349);
        let assign5110_e6866: f64 = (assign5110_e6862 + assign5110_e6865);
        let assign5110_e6869: f64 = (var_inv_lnfin * p.p1350);
        let assign5110_e6870: f64 = (assign5110_e6866 + assign5110_e6869);
        let assign5110_e6873: f64 = (var_inv_w * p.p1351);
        let assign5110_e6874: f64 = (assign5110_e6870 + assign5110_e6873);
        let assign5110_e6877: f64 = (var_inv_wl * p.p1352);
        let assign5110_e6878: f64 = (assign5110_e6874 + assign5110_e6877);
        var_dtatd_i = assign5110_e6878;

        let assign5120_e6882: f64 = (var_inv_l * p.p1300);
        let assign5120_e6883: f64 = (p.p1299 + assign5120_e6882);
        let assign5120_e6886: f64 = (var_inv_nfin * p.p1301);
        let assign5120_e6887: f64 = (assign5120_e6883 + assign5120_e6886);
        let assign5120_e6890: f64 = (var_inv_lnfin * p.p1302);
        let assign5120_e6891: f64 = (assign5120_e6887 + assign5120_e6890);
        let assign5120_e6894: f64 = (var_inv_w * p.p1303);
        let assign5120_e6895: f64 = (assign5120_e6891 + assign5120_e6894);
        let assign5120_e6898: f64 = (var_inv_wl * p.p1304);
        let assign5120_e6899: f64 = (assign5120_e6895 + assign5120_e6898);
        var_agisl_i = assign5120_e6899;

        let assign5130_e6903: f64 = (var_inv_l * p.p1306);
        let assign5130_e6904: f64 = (p.p1305 + assign5130_e6903);
        let assign5130_e6907: f64 = (var_inv_nfin * p.p1307);
        let assign5130_e6908: f64 = (assign5130_e6904 + assign5130_e6907);
        let assign5130_e6911: f64 = (var_inv_lnfin * p.p1308);
        let assign5130_e6912: f64 = (assign5130_e6908 + assign5130_e6911);
        let assign5130_e6915: f64 = (var_inv_w * p.p1309);
        let assign5130_e6916: f64 = (assign5130_e6912 + assign5130_e6915);
        let assign5130_e6919: f64 = (var_inv_wl * p.p1310);
        let assign5130_e6920: f64 = (assign5130_e6916 + assign5130_e6919);
        var_bgisl_i = assign5130_e6920;

        let assign5140_e6924: f64 = (var_inv_l * p.p1312);
        let assign5140_e6925: f64 = (p.p1311 + assign5140_e6924);
        let assign5140_e6928: f64 = (var_inv_nfin * p.p1313);
        let assign5140_e6929: f64 = (assign5140_e6925 + assign5140_e6928);
        let assign5140_e6932: f64 = (var_inv_lnfin * p.p1314);
        let assign5140_e6933: f64 = (assign5140_e6929 + assign5140_e6932);
        let assign5140_e6936: f64 = (var_inv_w * p.p1315);
        let assign5140_e6937: f64 = (assign5140_e6933 + assign5140_e6936);
        let assign5140_e6940: f64 = (var_inv_wl * p.p1316);
        let assign5140_e6941: f64 = (assign5140_e6937 + assign5140_e6940);
        var_cgisl_i = assign5140_e6941;

        let assign5150_e6945: f64 = (var_inv_l * p.p1318);
        let assign5150_e6946: f64 = (p.p1317 + assign5150_e6945);
        let assign5150_e6949: f64 = (var_inv_nfin * p.p1319);
        let assign5150_e6950: f64 = (assign5150_e6946 + assign5150_e6949);
        let assign5150_e6953: f64 = (var_inv_lnfin * p.p1320);
        let assign5150_e6954: f64 = (assign5150_e6950 + assign5150_e6953);
        let assign5150_e6957: f64 = (var_inv_w * p.p1321);
        let assign5150_e6958: f64 = (assign5150_e6954 + assign5150_e6957);
        let assign5150_e6961: f64 = (var_inv_wl * p.p1322);
        let assign5150_e6962: f64 = (assign5150_e6958 + assign5150_e6961);
        var_egisl_i = assign5150_e6962;

        let assign5160_e6966: f64 = (var_inv_l * p.p1324);
        let assign5160_e6967: f64 = (p.p1323 + assign5160_e6966);
        let assign5160_e6970: f64 = (var_inv_nfin * p.p1325);
        let assign5160_e6971: f64 = (assign5160_e6967 + assign5160_e6970);
        let assign5160_e6974: f64 = (var_inv_lnfin * p.p1326);
        let assign5160_e6975: f64 = (assign5160_e6971 + assign5160_e6974);
        let assign5160_e6978: f64 = (var_inv_w * p.p1327);
        let assign5160_e6979: f64 = (assign5160_e6975 + assign5160_e6978);
        let assign5160_e6982: f64 = (var_inv_wl * p.p1328);
        let assign5160_e6983: f64 = (assign5160_e6979 + assign5160_e6982);
        var_pgisl_i = assign5160_e6983;

        let assign5170_e6987: f64 = (var_inv_l * p.p1354);
        let assign5170_e6988: f64 = (p.p1353 + assign5170_e6987);
        let assign5170_e6991: f64 = (var_inv_nfin * p.p1355);
        let assign5170_e6992: f64 = (assign5170_e6988 + assign5170_e6991);
        let assign5170_e6995: f64 = (var_inv_lnfin * p.p1356);
        let assign5170_e6996: f64 = (assign5170_e6992 + assign5170_e6995);
        let assign5170_e6999: f64 = (var_inv_w * p.p1357);
        let assign5170_e7000: f64 = (assign5170_e6996 + assign5170_e6999);
        let assign5170_e7003: f64 = (var_inv_wl * p.p1358);
        let assign5170_e7004: f64 = (assign5170_e7000 + assign5170_e7003);
        var_atats_i = assign5170_e7004;

        let assign5180_e7008: f64 = (var_inv_l * p.p1360);
        let assign5180_e7009: f64 = (p.p1359 + assign5180_e7008);
        let assign5180_e7012: f64 = (var_inv_nfin * p.p1361);
        let assign5180_e7013: f64 = (assign5180_e7009 + assign5180_e7012);
        let assign5180_e7016: f64 = (var_inv_lnfin * p.p1362);
        let assign5180_e7017: f64 = (assign5180_e7013 + assign5180_e7016);
        let assign5180_e7020: f64 = (var_inv_w * p.p1363);
        let assign5180_e7021: f64 = (assign5180_e7017 + assign5180_e7020);
        let assign5180_e7024: f64 = (var_inv_wl * p.p1364);
        let assign5180_e7025: f64 = (assign5180_e7021 + assign5180_e7024);
        var_btats_i = assign5180_e7025;

        let assign5190_e7029: f64 = (var_inv_l * p.p1366);
        let assign5190_e7030: f64 = (p.p1365 + assign5190_e7029);
        let assign5190_e7033: f64 = (var_inv_nfin * p.p1367);
        let assign5190_e7034: f64 = (assign5190_e7030 + assign5190_e7033);
        let assign5190_e7037: f64 = (var_inv_lnfin * p.p1368);
        let assign5190_e7038: f64 = (assign5190_e7034 + assign5190_e7037);
        let assign5190_e7041: f64 = (var_inv_w * p.p1369);
        let assign5190_e7042: f64 = (assign5190_e7038 + assign5190_e7041);
        let assign5190_e7045: f64 = (var_inv_wl * p.p1370);
        let assign5190_e7046: f64 = (assign5190_e7042 + assign5190_e7045);
        var_ctats_i = assign5190_e7046;

        let assign5200_e7050: f64 = (var_inv_l * p.p1372);
        let assign5200_e7051: f64 = (p.p1371 + assign5200_e7050);
        let assign5200_e7054: f64 = (var_inv_nfin * p.p1373);
        let assign5200_e7055: f64 = (assign5200_e7051 + assign5200_e7054);
        let assign5200_e7058: f64 = (var_inv_lnfin * p.p1374);
        let assign5200_e7059: f64 = (assign5200_e7055 + assign5200_e7058);
        let assign5200_e7062: f64 = (var_inv_w * p.p1375);
        let assign5200_e7063: f64 = (assign5200_e7059 + assign5200_e7062);
        let assign5200_e7066: f64 = (var_inv_wl * p.p1376);
        let assign5200_e7067: f64 = (assign5200_e7063 + assign5200_e7066);
        var_dtats_i = assign5200_e7067;

        let assign5210_e7071: f64 = (var_inv_l * p.p1445);
        let assign5210_e7072: f64 = (p.p1444 + assign5210_e7071);
        let assign5210_e7075: f64 = (var_inv_nfin * p.p1446);
        let assign5210_e7076: f64 = (assign5210_e7072 + assign5210_e7075);
        let assign5210_e7079: f64 = (var_inv_lnfin * p.p1447);
        let assign5210_e7080: f64 = (assign5210_e7076 + assign5210_e7079);
        let assign5210_e7083: f64 = (var_inv_w * p.p1448);
        let assign5210_e7084: f64 = (assign5210_e7080 + assign5210_e7083);
        let assign5210_e7087: f64 = (var_inv_wl * p.p1449);
        let assign5210_e7088: f64 = (assign5210_e7084 + assign5210_e7087);
        var_alpha0_i = assign5210_e7088;

        let assign5220_e7092: f64 = (var_inv_l * p.p1451);
        let assign5220_e7093: f64 = (p.p1450 + assign5220_e7092);
        let assign5220_e7096: f64 = (var_inv_nfin * p.p1452);
        let assign5220_e7097: f64 = (assign5220_e7093 + assign5220_e7096);
        let assign5220_e7100: f64 = (var_inv_lnfin * p.p1453);
        let assign5220_e7101: f64 = (assign5220_e7097 + assign5220_e7100);
        let assign5220_e7104: f64 = (var_inv_w * p.p1454);
        let assign5220_e7105: f64 = (assign5220_e7101 + assign5220_e7104);
        let assign5220_e7108: f64 = (var_inv_wl * p.p1455);
        let assign5220_e7109: f64 = (assign5220_e7105 + assign5220_e7108);
        var_alpha1_i = assign5220_e7109;

        let assign5230_e7113: f64 = (var_inv_l * p.p1463);
        let assign5230_e7114: f64 = (p.p1462 + assign5230_e7113);
        let assign5230_e7117: f64 = (var_inv_nfin * p.p1464);
        let assign5230_e7118: f64 = (assign5230_e7114 + assign5230_e7117);
        let assign5230_e7121: f64 = (var_inv_lnfin * p.p1465);
        let assign5230_e7122: f64 = (assign5230_e7118 + assign5230_e7121);
        let assign5230_e7125: f64 = (var_inv_w * p.p1466);
        let assign5230_e7126: f64 = (assign5230_e7122 + assign5230_e7125);
        let assign5230_e7129: f64 = (var_inv_wl * p.p1467);
        let assign5230_e7130: f64 = (assign5230_e7126 + assign5230_e7129);
        var_alphaii0_i = assign5230_e7130;

        let assign5240_e7134: f64 = (var_inv_l * p.p1469);
        let assign5240_e7135: f64 = (p.p1468 + assign5240_e7134);
        let assign5240_e7138: f64 = (var_inv_nfin * p.p1470);
        let assign5240_e7139: f64 = (assign5240_e7135 + assign5240_e7138);
        let assign5240_e7142: f64 = (var_inv_lnfin * p.p1471);
        let assign5240_e7143: f64 = (assign5240_e7139 + assign5240_e7142);
        let assign5240_e7146: f64 = (var_inv_w * p.p1472);
        let assign5240_e7147: f64 = (assign5240_e7143 + assign5240_e7146);
        let assign5240_e7150: f64 = (var_inv_wl * p.p1473);
        let assign5240_e7151: f64 = (assign5240_e7147 + assign5240_e7150);
        var_alphaii1_i = assign5240_e7151;

        let assign5250_e7155: f64 = (var_inv_l * p.p1457);
        let assign5250_e7156: f64 = (p.p1456 + assign5250_e7155);
        let assign5250_e7159: f64 = (var_inv_nfin * p.p1458);
        let assign5250_e7160: f64 = (assign5250_e7156 + assign5250_e7159);
        let assign5250_e7163: f64 = (var_inv_lnfin * p.p1459);
        let assign5250_e7164: f64 = (assign5250_e7160 + assign5250_e7163);
        let assign5250_e7167: f64 = (var_inv_w * p.p1460);
        let assign5250_e7168: f64 = (assign5250_e7164 + assign5250_e7167);
        let assign5250_e7171: f64 = (var_inv_wl * p.p1461);
        let assign5250_e7172: f64 = (assign5250_e7168 + assign5250_e7171);
        var_beta0_i = assign5250_e7172;

        let assign5260_e7176: f64 = (var_inv_l * p.p1475);
        let assign5260_e7177: f64 = (p.p1474 + assign5260_e7176);
        let assign5260_e7180: f64 = (var_inv_nfin * p.p1476);
        let assign5260_e7181: f64 = (assign5260_e7177 + assign5260_e7180);
        let assign5260_e7184: f64 = (var_inv_lnfin * p.p1477);
        let assign5260_e7185: f64 = (assign5260_e7181 + assign5260_e7184);
        let assign5260_e7188: f64 = (var_inv_w * p.p1478);
        let assign5260_e7189: f64 = (assign5260_e7185 + assign5260_e7188);
        let assign5260_e7192: f64 = (var_inv_wl * p.p1479);
        let assign5260_e7193: f64 = (assign5260_e7189 + assign5260_e7192);
        var_betaii0_i = assign5260_e7193;

        let assign5270_e7197: f64 = (var_inv_l * p.p1481);
        let assign5270_e7198: f64 = (p.p1480 + assign5270_e7197);
        let assign5270_e7201: f64 = (var_inv_nfin * p.p1482);
        let assign5270_e7202: f64 = (assign5270_e7198 + assign5270_e7201);
        let assign5270_e7205: f64 = (var_inv_lnfin * p.p1483);
        let assign5270_e7206: f64 = (assign5270_e7202 + assign5270_e7205);
        let assign5270_e7209: f64 = (var_inv_w * p.p1484);
        let assign5270_e7210: f64 = (assign5270_e7206 + assign5270_e7209);
        let assign5270_e7213: f64 = (var_inv_wl * p.p1485);
        let assign5270_e7214: f64 = (assign5270_e7210 + assign5270_e7213);
        var_betaii1_i = assign5270_e7214;

        let assign5280_e7218: f64 = (var_inv_l * p.p1487);
        let assign5280_e7219: f64 = (p.p1486 + assign5280_e7218);
        let assign5280_e7222: f64 = (var_inv_nfin * p.p1488);
        let assign5280_e7223: f64 = (assign5280_e7219 + assign5280_e7222);
        let assign5280_e7226: f64 = (var_inv_lnfin * p.p1489);
        let assign5280_e7227: f64 = (assign5280_e7223 + assign5280_e7226);
        let assign5280_e7230: f64 = (var_inv_w * p.p1490);
        let assign5280_e7231: f64 = (assign5280_e7227 + assign5280_e7230);
        let assign5280_e7234: f64 = (var_inv_wl * p.p1491);
        let assign5280_e7235: f64 = (assign5280_e7231 + assign5280_e7234);
        var_betaii2_i = assign5280_e7235;

        let assign5290_e7239: f64 = (var_inv_l * p.p1493);
        let assign5290_e7240: f64 = (p.p1492 + assign5290_e7239);
        let assign5290_e7243: f64 = (var_inv_nfin * p.p1494);
        let assign5290_e7244: f64 = (assign5290_e7240 + assign5290_e7243);
        let assign5290_e7247: f64 = (var_inv_lnfin * p.p1495);
        let assign5290_e7248: f64 = (assign5290_e7244 + assign5290_e7247);
        let assign5290_e7251: f64 = (var_inv_w * p.p1496);
        let assign5290_e7252: f64 = (assign5290_e7248 + assign5290_e7251);
        let assign5290_e7255: f64 = (var_inv_wl * p.p1497);
        let assign5290_e7256: f64 = (assign5290_e7252 + assign5290_e7255);
        var_esatii_i = assign5290_e7256;

        let assign5300_e7260: f64 = (var_inv_l * p.p1499);
        let assign5300_e7261: f64 = (p.p1498 + assign5300_e7260);
        let assign5300_e7264: f64 = (var_inv_nfin * p.p1500);
        let assign5300_e7265: f64 = (assign5300_e7261 + assign5300_e7264);
        let assign5300_e7268: f64 = (var_inv_lnfin * p.p1501);
        let assign5300_e7269: f64 = (assign5300_e7265 + assign5300_e7268);
        let assign5300_e7272: f64 = (var_inv_w * p.p1502);
        let assign5300_e7273: f64 = (assign5300_e7269 + assign5300_e7272);
        let assign5300_e7276: f64 = (var_inv_wl * p.p1503);
        let assign5300_e7277: f64 = (assign5300_e7273 + assign5300_e7276);
        var_lii_i = assign5300_e7277;

        let assign5310_e7281: f64 = (var_inv_l * p.p1505);
        let assign5310_e7282: f64 = (p.p1504 + assign5310_e7281);
        let assign5310_e7285: f64 = (var_inv_nfin * p.p1506);
        let assign5310_e7286: f64 = (assign5310_e7282 + assign5310_e7285);
        let assign5310_e7289: f64 = (var_inv_lnfin * p.p1507);
        let assign5310_e7290: f64 = (assign5310_e7286 + assign5310_e7289);
        let assign5310_e7293: f64 = (var_inv_w * p.p1508);
        let assign5310_e7294: f64 = (assign5310_e7290 + assign5310_e7293);
        let assign5310_e7297: f64 = (var_inv_wl * p.p1509);
        let assign5310_e7298: f64 = (assign5310_e7294 + assign5310_e7297);
        var_sii0_i = assign5310_e7298;

        let assign5320_e7302: f64 = (var_inv_l * p.p1511);
        let assign5320_e7303: f64 = (p.p1510 + assign5320_e7302);
        let assign5320_e7306: f64 = (var_inv_nfin * p.p1512);
        let assign5320_e7307: f64 = (assign5320_e7303 + assign5320_e7306);
        let assign5320_e7310: f64 = (var_inv_lnfin * p.p1513);
        let assign5320_e7311: f64 = (assign5320_e7307 + assign5320_e7310);
        let assign5320_e7314: f64 = (var_inv_w * p.p1514);
        let assign5320_e7315: f64 = (assign5320_e7311 + assign5320_e7314);
        let assign5320_e7318: f64 = (var_inv_wl * p.p1515);
        let assign5320_e7319: f64 = (assign5320_e7315 + assign5320_e7318);
        var_sii1_i = assign5320_e7319;

        let assign5330_e7323: f64 = (var_inv_l * p.p1517);
        let assign5330_e7324: f64 = (p.p1516 + assign5330_e7323);
        let assign5330_e7327: f64 = (var_inv_nfin * p.p1518);
        let assign5330_e7328: f64 = (assign5330_e7324 + assign5330_e7327);
        let assign5330_e7331: f64 = (var_inv_lnfin * p.p1519);
        let assign5330_e7332: f64 = (assign5330_e7328 + assign5330_e7331);
        let assign5330_e7335: f64 = (var_inv_w * p.p1520);
        let assign5330_e7336: f64 = (assign5330_e7332 + assign5330_e7335);
        let assign5330_e7339: f64 = (var_inv_wl * p.p1521);
        let assign5330_e7340: f64 = (assign5330_e7336 + assign5330_e7339);
        var_sii2_i = assign5330_e7340;

        let assign5340_e7344: f64 = (var_inv_l * p.p1523);
        let assign5340_e7345: f64 = (p.p1522 + assign5340_e7344);
        let assign5340_e7348: f64 = (var_inv_nfin * p.p1524);
        let assign5340_e7349: f64 = (assign5340_e7345 + assign5340_e7348);
        let assign5340_e7352: f64 = (var_inv_lnfin * p.p1525);
        let assign5340_e7353: f64 = (assign5340_e7349 + assign5340_e7352);
        let assign5340_e7356: f64 = (var_inv_w * p.p1526);
        let assign5340_e7357: f64 = (assign5340_e7353 + assign5340_e7356);
        let assign5340_e7360: f64 = (var_inv_wl * p.p1527);
        let assign5340_e7361: f64 = (assign5340_e7357 + assign5340_e7360);
        var_siid_i = assign5340_e7361;

        let assign5350_e7365: f64 = (var_inv_l * p.p1763);
        let assign5350_e7366: f64 = (p.p1762 + assign5350_e7365);
        let assign5350_e7369: f64 = (var_inv_nfin * p.p1764);
        let assign5350_e7370: f64 = (assign5350_e7366 + assign5350_e7369);
        let assign5350_e7373: f64 = (var_inv_lnfin * p.p1765);
        let assign5350_e7374: f64 = (assign5350_e7370 + assign5350_e7373);
        let assign5350_e7377: f64 = (var_inv_w * p.p1766);
        let assign5350_e7378: f64 = (assign5350_e7374 + assign5350_e7377);
        let assign5350_e7381: f64 = (var_inv_wl * p.p1767);
        let assign5350_e7382: f64 = (assign5350_e7378 + assign5350_e7381);
        var_tii_i = assign5350_e7382;

        let assign5360_e7386: f64 = (var_inv_l * p.p1531);
        let assign5360_e7387: f64 = (p.p1530 + assign5360_e7386);
        let assign5360_e7390: f64 = (var_inv_nfin * p.p1532);
        let assign5360_e7391: f64 = (assign5360_e7387 + assign5360_e7390);
        let assign5360_e7394: f64 = (var_inv_lnfin * p.p1533);
        let assign5360_e7395: f64 = (assign5360_e7391 + assign5360_e7394);
        let assign5360_e7398: f64 = (var_inv_w * p.p1534);
        let assign5360_e7399: f64 = (assign5360_e7395 + assign5360_e7398);
        let assign5360_e7402: f64 = (var_inv_wl * p.p1535);
        let assign5360_e7403: f64 = (assign5360_e7399 + assign5360_e7402);
        var_cfs_i = assign5360_e7403;

        let assign5370_e7407: f64 = (var_inv_l * p.p1537);
        let assign5370_e7408: f64 = (p.p1536 + assign5370_e7407);
        let assign5370_e7411: f64 = (var_inv_nfin * p.p1538);
        let assign5370_e7412: f64 = (assign5370_e7408 + assign5370_e7411);
        let assign5370_e7415: f64 = (var_inv_lnfin * p.p1539);
        let assign5370_e7416: f64 = (assign5370_e7412 + assign5370_e7415);
        let assign5370_e7419: f64 = (var_inv_w * p.p1540);
        let assign5370_e7420: f64 = (assign5370_e7416 + assign5370_e7419);
        let assign5370_e7423: f64 = (var_inv_wl * p.p1541);
        let assign5370_e7424: f64 = (assign5370_e7420 + assign5370_e7423);
        var_cfd_i = assign5370_e7424;

        let assign5380_e7428: f64 = (var_inv_l * p.p29);
        let assign5380_e7429: f64 = (p.p28 + assign5380_e7428);
        let assign5380_e7432: f64 = (var_inv_nfin * p.p30);
        let assign5380_e7433: f64 = (assign5380_e7429 + assign5380_e7432);
        let assign5380_e7436: f64 = (var_inv_lnfin * p.p31);
        let assign5380_e7437: f64 = (assign5380_e7433 + assign5380_e7436);
        let assign5380_e7440: f64 = (var_inv_w * p.p32);
        let assign5380_e7441: f64 = (assign5380_e7437 + assign5380_e7440);
        let assign5380_e7444: f64 = (var_inv_wl * p.p33);
        let assign5380_e7445: f64 = (assign5380_e7441 + assign5380_e7444);
        var_covs_i = assign5380_e7445;
        var_covs_i_dn0 = 0.0;
        var_covs_i_dn2 = 0.0;
        var_covs_i_dn3 = 0.0;
        var_covs_i_dn4 = 0.0;
        var_covs_i_dn5 = 0.0;
        var_covs_i_dn6 = 0.0;
        var_covs_i_dn7 = 0.0;
        var_covs_i_dn8 = 0.0;
        var_covs_i_dn9 = 0.0;
        var_covs_i_dn10 = 0.0;
        var_covs_i_dn11 = 0.0;
        var_covs_i_dn13 = 0.0;
        var_covs_i_dn14 = 0.0;

        let assign5390_e7449: f64 = (var_inv_l * p.p35);
        let assign5390_e7450: f64 = (p.p34 + assign5390_e7449);
        let assign5390_e7453: f64 = (var_inv_nfin * p.p36);
        let assign5390_e7454: f64 = (assign5390_e7450 + assign5390_e7453);
        let assign5390_e7457: f64 = (var_inv_lnfin * p.p37);
        let assign5390_e7458: f64 = (assign5390_e7454 + assign5390_e7457);
        let assign5390_e7461: f64 = (var_inv_w * p.p38);
        let assign5390_e7462: f64 = (assign5390_e7458 + assign5390_e7461);
        let assign5390_e7465: f64 = (var_inv_wl * p.p39);
        let assign5390_e7466: f64 = (assign5390_e7462 + assign5390_e7465);
        var_covd_i = assign5390_e7466;
        var_covd_i_dn0 = 0.0;
        var_covd_i_dn2 = 0.0;
        var_covd_i_dn3 = 0.0;
        var_covd_i_dn4 = 0.0;
        var_covd_i_dn5 = 0.0;
        var_covd_i_dn6 = 0.0;
        var_covd_i_dn7 = 0.0;
        var_covd_i_dn8 = 0.0;
        var_covd_i_dn9 = 0.0;
        var_covd_i_dn10 = 0.0;
        var_covd_i_dn11 = 0.0;
        var_covd_i_dn13 = 0.0;
        var_covd_i_dn14 = 0.0;

        let assign5400_e7470: f64 = (var_inv_l * p.p1548);
        let assign5400_e7471: f64 = (p.p1547 + assign5400_e7470);
        let assign5400_e7474: f64 = (var_inv_nfin * p.p1549);
        let assign5400_e7475: f64 = (assign5400_e7471 + assign5400_e7474);
        let assign5400_e7478: f64 = (var_inv_lnfin * p.p1550);
        let assign5400_e7479: f64 = (assign5400_e7475 + assign5400_e7478);
        let assign5400_e7482: f64 = (var_inv_w * p.p1551);
        let assign5400_e7483: f64 = (assign5400_e7479 + assign5400_e7482);
        let assign5400_e7486: f64 = (var_inv_wl * p.p1552);
        let assign5400_e7487: f64 = (assign5400_e7483 + assign5400_e7486);
        var_cgsl_i = assign5400_e7487;

        let assign5410_e7491: f64 = (var_inv_l * p.p1554);
        let assign5410_e7492: f64 = (p.p1553 + assign5410_e7491);
        let assign5410_e7495: f64 = (var_inv_nfin * p.p1555);
        let assign5410_e7496: f64 = (assign5410_e7492 + assign5410_e7495);
        let assign5410_e7499: f64 = (var_inv_lnfin * p.p1556);
        let assign5410_e7500: f64 = (assign5410_e7496 + assign5410_e7499);
        let assign5410_e7503: f64 = (var_inv_w * p.p1557);
        let assign5410_e7504: f64 = (assign5410_e7500 + assign5410_e7503);
        let assign5410_e7507: f64 = (var_inv_wl * p.p1558);
        let assign5410_e7508: f64 = (assign5410_e7504 + assign5410_e7507);
        var_cgdl_i = assign5410_e7508;

        let assign5420_e7512: f64 = (var_inv_l * p.p1560);
        let assign5420_e7513: f64 = (p.p1559 + assign5420_e7512);
        let assign5420_e7516: f64 = (var_inv_nfin * p.p1561);
        let assign5420_e7517: f64 = (assign5420_e7513 + assign5420_e7516);
        let assign5420_e7520: f64 = (var_inv_lnfin * p.p1562);
        let assign5420_e7521: f64 = (assign5420_e7517 + assign5420_e7520);
        let assign5420_e7524: f64 = (var_inv_w * p.p1563);
        let assign5420_e7525: f64 = (assign5420_e7521 + assign5420_e7524);
        let assign5420_e7528: f64 = (var_inv_wl * p.p1564);
        let assign5420_e7529: f64 = (assign5420_e7525 + assign5420_e7528);
        var_cgbl_i = assign5420_e7529;

        let assign5430_e7533: f64 = (var_inv_l * p.p1566);
        let assign5430_e7534: f64 = (p.p1565 + assign5430_e7533);
        let assign5430_e7537: f64 = (var_inv_nfin * p.p1567);
        let assign5430_e7538: f64 = (assign5430_e7534 + assign5430_e7537);
        let assign5430_e7541: f64 = (var_inv_lnfin * p.p1568);
        let assign5430_e7542: f64 = (assign5430_e7538 + assign5430_e7541);
        let assign5430_e7545: f64 = (var_inv_w * p.p1569);
        let assign5430_e7546: f64 = (assign5430_e7542 + assign5430_e7545);
        let assign5430_e7549: f64 = (var_inv_wl * p.p1570);
        let assign5430_e7550: f64 = (assign5430_e7546 + assign5430_e7549);
        var_ckappas_i = assign5430_e7550;

        let assign5440_e7554: f64 = (var_inv_l * p.p1572);
        let assign5440_e7555: f64 = (p.p1571 + assign5440_e7554);
        let assign5440_e7558: f64 = (var_inv_nfin * p.p1573);
        let assign5440_e7559: f64 = (assign5440_e7555 + assign5440_e7558);
        let assign5440_e7562: f64 = (var_inv_lnfin * p.p1574);
        let assign5440_e7563: f64 = (assign5440_e7559 + assign5440_e7562);
        let assign5440_e7566: f64 = (var_inv_w * p.p1575);
        let assign5440_e7567: f64 = (assign5440_e7563 + assign5440_e7566);
        let assign5440_e7570: f64 = (var_inv_wl * p.p1576);
        let assign5440_e7571: f64 = (assign5440_e7567 + assign5440_e7570);
        var_ckappad_i = assign5440_e7571;

        let assign5450_e7575: f64 = (var_inv_l * p.p1578);
        let assign5450_e7576: f64 = (p.p1577 + assign5450_e7575);
        let assign5450_e7579: f64 = (var_inv_nfin * p.p1579);
        let assign5450_e7580: f64 = (assign5450_e7576 + assign5450_e7579);
        let assign5450_e7583: f64 = (var_inv_lnfin * p.p1580);
        let assign5450_e7584: f64 = (assign5450_e7580 + assign5450_e7583);
        let assign5450_e7587: f64 = (var_inv_w * p.p1581);
        let assign5450_e7588: f64 = (assign5450_e7584 + assign5450_e7587);
        let assign5450_e7591: f64 = (var_inv_wl * p.p1582);
        let assign5450_e7592: f64 = (assign5450_e7588 + assign5450_e7591);
        var_ckappab_i = assign5450_e7592;

        let assign5460_e7596: f64 = (var_inv_l * p.p1651);
        let assign5460_e7597: f64 = (p.p1650 + assign5460_e7596);
        let assign5460_e7600: f64 = (var_inv_nfin * p.p1652);
        let assign5460_e7601: f64 = (assign5460_e7597 + assign5460_e7600);
        let assign5460_e7604: f64 = (var_inv_lnfin * p.p1653);
        let assign5460_e7605: f64 = (assign5460_e7601 + assign5460_e7604);
        let assign5460_e7608: f64 = (var_inv_w * p.p1654);
        let assign5460_e7609: f64 = (assign5460_e7605 + assign5460_e7608);
        let assign5460_e7612: f64 = (var_inv_wl * p.p1655);
        let assign5460_e7613: f64 = (assign5460_e7609 + assign5460_e7612);
        var_ntgen_i = assign5460_e7613;

        let assign5470_e7617: f64 = (var_inv_l * p.p1657);
        let assign5470_e7618: f64 = (p.p1656 + assign5470_e7617);
        let assign5470_e7621: f64 = (var_inv_nfin * p.p1658);
        let assign5470_e7622: f64 = (assign5470_e7618 + assign5470_e7621);
        let assign5470_e7625: f64 = (var_inv_lnfin * p.p1659);
        let assign5470_e7626: f64 = (assign5470_e7622 + assign5470_e7625);
        let assign5470_e7629: f64 = (var_inv_w * p.p1660);
        let assign5470_e7630: f64 = (assign5470_e7626 + assign5470_e7629);
        let assign5470_e7633: f64 = (var_inv_wl * p.p1661);
        let assign5470_e7634: f64 = (assign5470_e7630 + assign5470_e7633);
        var_aigen_i = assign5470_e7634;

        let assign5480_e7638: f64 = (var_inv_l * p.p1663);
        let assign5480_e7639: f64 = (p.p1662 + assign5480_e7638);
        let assign5480_e7642: f64 = (var_inv_nfin * p.p1664);
        let assign5480_e7643: f64 = (assign5480_e7639 + assign5480_e7642);
        let assign5480_e7646: f64 = (var_inv_lnfin * p.p1665);
        let assign5480_e7647: f64 = (assign5480_e7643 + assign5480_e7646);
        let assign5480_e7650: f64 = (var_inv_w * p.p1666);
        let assign5480_e7651: f64 = (assign5480_e7647 + assign5480_e7650);
        let assign5480_e7654: f64 = (var_inv_wl * p.p1667);
        let assign5480_e7655: f64 = (assign5480_e7651 + assign5480_e7654);
        var_bigen_i = assign5480_e7655;

        let assign5490_e7659: f64 = (var_inv_l * p.p738);
        let assign5490_e7660: f64 = (p.p737 + assign5490_e7659);
        let assign5490_e7663: f64 = (var_inv_nfin * p.p739);
        let assign5490_e7664: f64 = (assign5490_e7660 + assign5490_e7663);
        let assign5490_e7667: f64 = (var_inv_lnfin * p.p740);
        let assign5490_e7668: f64 = (assign5490_e7664 + assign5490_e7667);
        let assign5490_e7671: f64 = (var_inv_w * p.p741);
        let assign5490_e7672: f64 = (assign5490_e7668 + assign5490_e7671);
        let assign5490_e7675: f64 = (var_inv_wl * p.p742);
        let assign5490_e7676: f64 = (assign5490_e7672 + assign5490_e7675);
        var_ute_i = assign5490_e7676;

        *var_agisl_i_slot = var_agisl_i;
        *var_aigen_i_slot = var_aigen_i;
        *var_alpha0_i_slot = var_alpha0_i;
        *var_alpha1_i_slot = var_alpha1_i;
        *var_alphaii0_i_slot = var_alphaii0_i;
        *var_alphaii1_i_slot = var_alphaii1_i;
        *var_atats_i_slot = var_atats_i;
        *var_beta0_i_slot = var_beta0_i;
        *var_betaii0_i_slot = var_betaii0_i;
        *var_betaii1_i_slot = var_betaii1_i;
        *var_betaii2_i_slot = var_betaii2_i;
        *var_bgisl_i_slot = var_bgisl_i;
        *var_bigen_i_slot = var_bigen_i;
        *var_btats_i_slot = var_btats_i;
        *var_cfd_i_slot = var_cfd_i;
        *var_cfs_i_slot = var_cfs_i;
        *var_cgbl_i_slot = var_cgbl_i;
        *var_cgdl_i_slot = var_cgdl_i;
        *var_cgisl_i_slot = var_cgisl_i;
        *var_cgsl_i_slot = var_cgsl_i;
        *var_ckappab_i_slot = var_ckappab_i;
        *var_ckappad_i_slot = var_ckappad_i;
        *var_ckappas_i_slot = var_ckappas_i;
        *var_covd_i_slot = var_covd_i;
        *var_covd_i_dn0_slot = var_covd_i_dn0;
        *var_covd_i_dn10_slot = var_covd_i_dn10;
        *var_covd_i_dn11_slot = var_covd_i_dn11;
        *var_covd_i_dn13_slot = var_covd_i_dn13;
        *var_covd_i_dn14_slot = var_covd_i_dn14;
        *var_covd_i_dn2_slot = var_covd_i_dn2;
        *var_covd_i_dn3_slot = var_covd_i_dn3;
        *var_covd_i_dn4_slot = var_covd_i_dn4;
        *var_covd_i_dn5_slot = var_covd_i_dn5;
        *var_covd_i_dn6_slot = var_covd_i_dn6;
        *var_covd_i_dn7_slot = var_covd_i_dn7;
        *var_covd_i_dn8_slot = var_covd_i_dn8;
        *var_covd_i_dn9_slot = var_covd_i_dn9;
        *var_covs_i_slot = var_covs_i;
        *var_covs_i_dn0_slot = var_covs_i_dn0;
        *var_covs_i_dn10_slot = var_covs_i_dn10;
        *var_covs_i_dn11_slot = var_covs_i_dn11;
        *var_covs_i_dn13_slot = var_covs_i_dn13;
        *var_covs_i_dn14_slot = var_covs_i_dn14;
        *var_covs_i_dn2_slot = var_covs_i_dn2;
        *var_covs_i_dn3_slot = var_covs_i_dn3;
        *var_covs_i_dn4_slot = var_covs_i_dn4;
        *var_covs_i_dn5_slot = var_covs_i_dn5;
        *var_covs_i_dn6_slot = var_covs_i_dn6;
        *var_covs_i_dn7_slot = var_covs_i_dn7;
        *var_covs_i_dn8_slot = var_covs_i_dn8;
        *var_covs_i_dn9_slot = var_covs_i_dn9;
        *var_ctatd_i_slot = var_ctatd_i;
        *var_ctats_i_slot = var_ctats_i;
        *var_dtatd_i_slot = var_dtatd_i;
        *var_dtats_i_slot = var_dtats_i;
        *var_egisl_i_slot = var_egisl_i;
        *var_esatii_i_slot = var_esatii_i;
        *var_lii_i_slot = var_lii_i;
        *var_ntgen_i_slot = var_ntgen_i;
        *var_pgisl_i_slot = var_pgisl_i;
        *var_sii0_i_slot = var_sii0_i;
        *var_sii1_i_slot = var_sii1_i;
        *var_sii2_i_slot = var_sii2_i;
        *var_siid_i_slot = var_siid_i;
        *var_tii_i_slot = var_tii_i;
        *var_ute_i_slot = var_ute_i;
    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        var_inv_l: f64,
        var_inv_lnfin: f64,
        var_inv_nfin: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_at_i_slot: &mut f64,
        var_atcv_i_slot: &mut f64,
        var_dvtp0_i_slot: &mut f64,
        var_dvtp0_i_dn0_slot: &mut f64,
        var_dvtp0_i_dn10_slot: &mut f64,
        var_dvtp0_i_dn11_slot: &mut f64,
        var_dvtp0_i_dn13_slot: &mut f64,
        var_dvtp0_i_dn14_slot: &mut f64,
        var_dvtp0_i_dn2_slot: &mut f64,
        var_dvtp0_i_dn3_slot: &mut f64,
        var_dvtp0_i_dn4_slot: &mut f64,
        var_dvtp0_i_dn5_slot: &mut f64,
        var_dvtp0_i_dn6_slot: &mut f64,
        var_dvtp0_i_dn7_slot: &mut f64,
        var_dvtp0_i_dn8_slot: &mut f64,
        var_dvtp0_i_dn9_slot: &mut f64,
        var_dvtp1_i_slot: &mut f64,
        var_dvtp1_i_dn0_slot: &mut f64,
        var_dvtp1_i_dn10_slot: &mut f64,
        var_dvtp1_i_dn11_slot: &mut f64,
        var_dvtp1_i_dn13_slot: &mut f64,
        var_dvtp1_i_dn14_slot: &mut f64,
        var_dvtp1_i_dn2_slot: &mut f64,
        var_dvtp1_i_dn3_slot: &mut f64,
        var_dvtp1_i_dn4_slot: &mut f64,
        var_dvtp1_i_dn5_slot: &mut f64,
        var_dvtp1_i_dn6_slot: &mut f64,
        var_dvtp1_i_dn7_slot: &mut f64,
        var_dvtp1_i_dn8_slot: &mut f64,
        var_dvtp1_i_dn9_slot: &mut f64,
        var_emobt_i_slot: &mut f64,
        var_eu1_i_slot: &mut f64,
        var_guard42_slot: &mut f64,
        var_guard43_slot: &mut f64,
        var_igt_i_slot: &mut f64,
        var_iit_i_slot: &mut f64,
        var_k11_i_slot: &mut f64,
        var_k1_i_slot: &mut f64,
        var_k21_i_slot: &mut f64,
        var_k2_i_slot: &mut f64,
        var_kt1_i_slot: &mut f64,
        var_mpower_i_slot: &mut f64,
        var_noia2_i_slot: &mut f64,
        var_phibe_i_slot: &mut f64,
        var_prt1_i_slot: &mut f64,
        var_prt_i_slot: &mut f64,
        var_ptwgt_i_slot: &mut f64,
        var_qsref_i_slot: &mut f64,
        var_sprt_i_slot: &mut f64,
        var_tgidl_i_slot: &mut f64,
        var_tr0_i_slot: &mut f64,
        var_tss_i_slot: &mut f64,
        var_ttat_i_slot: &mut f64,
        var_ua1_i_slot: &mut f64,
        var_ua2_i_slot: &mut f64,
        var_uc1_i_slot: &mut f64,
        var_uc_i_slot: &mut f64,
        var_ucste1_i_slot: &mut f64,
        var_ucste_i_slot: &mut f64,
        var_ud1_i_slot: &mut f64,
        var_ud2_i_slot: &mut f64,
        var_ute1_i_slot: &mut f64,
        var_utl_i_slot: &mut f64,
    ) {
        let mut var_at_i: f64 = *var_at_i_slot;
        let mut var_atcv_i: f64 = *var_atcv_i_slot;
        let mut var_dvtp0_i: f64 = *var_dvtp0_i_slot;
        let mut var_dvtp0_i_dn0: f64 = *var_dvtp0_i_dn0_slot;
        let mut var_dvtp0_i_dn10: f64 = *var_dvtp0_i_dn10_slot;
        let mut var_dvtp0_i_dn11: f64 = *var_dvtp0_i_dn11_slot;
        let mut var_dvtp0_i_dn13: f64 = *var_dvtp0_i_dn13_slot;
        let mut var_dvtp0_i_dn14: f64 = *var_dvtp0_i_dn14_slot;
        let mut var_dvtp0_i_dn2: f64 = *var_dvtp0_i_dn2_slot;
        let mut var_dvtp0_i_dn3: f64 = *var_dvtp0_i_dn3_slot;
        let mut var_dvtp0_i_dn4: f64 = *var_dvtp0_i_dn4_slot;
        let mut var_dvtp0_i_dn5: f64 = *var_dvtp0_i_dn5_slot;
        let mut var_dvtp0_i_dn6: f64 = *var_dvtp0_i_dn6_slot;
        let mut var_dvtp0_i_dn7: f64 = *var_dvtp0_i_dn7_slot;
        let mut var_dvtp0_i_dn8: f64 = *var_dvtp0_i_dn8_slot;
        let mut var_dvtp0_i_dn9: f64 = *var_dvtp0_i_dn9_slot;
        let mut var_dvtp1_i: f64 = *var_dvtp1_i_slot;
        let mut var_dvtp1_i_dn0: f64 = *var_dvtp1_i_dn0_slot;
        let mut var_dvtp1_i_dn10: f64 = *var_dvtp1_i_dn10_slot;
        let mut var_dvtp1_i_dn11: f64 = *var_dvtp1_i_dn11_slot;
        let mut var_dvtp1_i_dn13: f64 = *var_dvtp1_i_dn13_slot;
        let mut var_dvtp1_i_dn14: f64 = *var_dvtp1_i_dn14_slot;
        let mut var_dvtp1_i_dn2: f64 = *var_dvtp1_i_dn2_slot;
        let mut var_dvtp1_i_dn3: f64 = *var_dvtp1_i_dn3_slot;
        let mut var_dvtp1_i_dn4: f64 = *var_dvtp1_i_dn4_slot;
        let mut var_dvtp1_i_dn5: f64 = *var_dvtp1_i_dn5_slot;
        let mut var_dvtp1_i_dn6: f64 = *var_dvtp1_i_dn6_slot;
        let mut var_dvtp1_i_dn7: f64 = *var_dvtp1_i_dn7_slot;
        let mut var_dvtp1_i_dn8: f64 = *var_dvtp1_i_dn8_slot;
        let mut var_dvtp1_i_dn9: f64 = *var_dvtp1_i_dn9_slot;
        let mut var_emobt_i: f64 = *var_emobt_i_slot;
        let mut var_eu1_i: f64 = *var_eu1_i_slot;
        let mut var_guard42: f64 = *var_guard42_slot;
        let mut var_guard43: f64 = *var_guard43_slot;
        let mut var_igt_i: f64 = *var_igt_i_slot;
        let mut var_iit_i: f64 = *var_iit_i_slot;
        let mut var_k11_i: f64 = *var_k11_i_slot;
        let mut var_k1_i: f64 = *var_k1_i_slot;
        let mut var_k21_i: f64 = *var_k21_i_slot;
        let mut var_k2_i: f64 = *var_k2_i_slot;
        let mut var_kt1_i: f64 = *var_kt1_i_slot;
        let mut var_mpower_i: f64 = *var_mpower_i_slot;
        let mut var_noia2_i: f64 = *var_noia2_i_slot;
        let mut var_phibe_i: f64 = *var_phibe_i_slot;
        let mut var_prt1_i: f64 = *var_prt1_i_slot;
        let mut var_prt_i: f64 = *var_prt_i_slot;
        let mut var_ptwgt_i: f64 = *var_ptwgt_i_slot;
        let mut var_qsref_i: f64 = *var_qsref_i_slot;
        let mut var_sprt_i: f64 = *var_sprt_i_slot;
        let mut var_tgidl_i: f64 = *var_tgidl_i_slot;
        let mut var_tr0_i: f64 = *var_tr0_i_slot;
        let mut var_tss_i: f64 = *var_tss_i_slot;
        let mut var_ttat_i: f64 = *var_ttat_i_slot;
        let mut var_ua1_i: f64 = *var_ua1_i_slot;
        let mut var_ua2_i: f64 = *var_ua2_i_slot;
        let mut var_uc1_i: f64 = *var_uc1_i_slot;
        let mut var_uc_i: f64 = *var_uc_i_slot;
        let mut var_ucste1_i: f64 = *var_ucste1_i_slot;
        let mut var_ucste_i: f64 = *var_ucste_i_slot;
        let mut var_ud1_i: f64 = *var_ud1_i_slot;
        let mut var_ud2_i: f64 = *var_ud2_i_slot;
        let mut var_ute1_i: f64 = *var_ute1_i_slot;
        let mut var_utl_i: f64 = *var_utl_i_slot;

        let assign5500_e7680: f64 = (var_inv_l * p.p756);
        let assign5500_e7681: f64 = (p.p755 + assign5500_e7680);
        let assign5500_e7684: f64 = (var_inv_nfin * p.p757);
        let assign5500_e7685: f64 = (assign5500_e7681 + assign5500_e7684);
        let assign5500_e7688: f64 = (var_inv_lnfin * p.p758);
        let assign5500_e7689: f64 = (assign5500_e7685 + assign5500_e7688);
        let assign5500_e7692: f64 = (var_inv_w * p.p759);
        let assign5500_e7693: f64 = (assign5500_e7689 + assign5500_e7692);
        let assign5500_e7696: f64 = (var_inv_wl * p.p760);
        let assign5500_e7697: f64 = (assign5500_e7693 + assign5500_e7696);
        var_ute1_i = assign5500_e7697;

        let assign5510_e7701: f64 = (var_inv_l * p.p768);
        let assign5510_e7702: f64 = (p.p767 + assign5510_e7701);
        let assign5510_e7705: f64 = (var_inv_nfin * p.p769);
        let assign5510_e7706: f64 = (assign5510_e7702 + assign5510_e7705);
        let assign5510_e7709: f64 = (var_inv_lnfin * p.p770);
        let assign5510_e7710: f64 = (assign5510_e7706 + assign5510_e7709);
        let assign5510_e7713: f64 = (var_inv_w * p.p771);
        let assign5510_e7714: f64 = (assign5510_e7710 + assign5510_e7713);
        let assign5510_e7717: f64 = (var_inv_wl * p.p772);
        let assign5510_e7718: f64 = (assign5510_e7714 + assign5510_e7717);
        var_utl_i = assign5510_e7718;

        let assign5520_e7722: f64 = (var_inv_l * p.p786);
        let assign5520_e7723: f64 = (p.p785 + assign5520_e7722);
        let assign5520_e7726: f64 = (var_inv_nfin * p.p787);
        let assign5520_e7727: f64 = (assign5520_e7723 + assign5520_e7726);
        let assign5520_e7730: f64 = (var_inv_lnfin * p.p788);
        let assign5520_e7731: f64 = (assign5520_e7727 + assign5520_e7730);
        let assign5520_e7734: f64 = (var_inv_w * p.p789);
        let assign5520_e7735: f64 = (assign5520_e7731 + assign5520_e7734);
        let assign5520_e7738: f64 = (var_inv_wl * p.p790);
        let assign5520_e7739: f64 = (assign5520_e7735 + assign5520_e7738);
        var_emobt_i = assign5520_e7739;

        let assign5530_e7743: f64 = (var_inv_l * p.p792);
        let assign5530_e7744: f64 = (p.p791 + assign5530_e7743);
        let assign5530_e7747: f64 = (var_inv_nfin * p.p793);
        let assign5530_e7748: f64 = (assign5530_e7744 + assign5530_e7747);
        let assign5530_e7751: f64 = (var_inv_lnfin * p.p794);
        let assign5530_e7752: f64 = (assign5530_e7748 + assign5530_e7751);
        let assign5530_e7755: f64 = (var_inv_w * p.p795);
        let assign5530_e7756: f64 = (assign5530_e7752 + assign5530_e7755);
        let assign5530_e7759: f64 = (var_inv_wl * p.p796);
        let assign5530_e7760: f64 = (assign5530_e7756 + assign5530_e7759);
        var_ua1_i = assign5530_e7760;

        let assign5540_e7764: f64 = (var_inv_l * p.p810);
        let assign5540_e7765: f64 = (p.p809 + assign5540_e7764);
        let assign5540_e7768: f64 = (var_inv_nfin * p.p811);
        let assign5540_e7769: f64 = (assign5540_e7765 + assign5540_e7768);
        let assign5540_e7772: f64 = (var_inv_lnfin * p.p812);
        let assign5540_e7773: f64 = (assign5540_e7769 + assign5540_e7772);
        let assign5540_e7776: f64 = (var_inv_w * p.p813);
        let assign5540_e7777: f64 = (assign5540_e7773 + assign5540_e7776);
        let assign5540_e7780: f64 = (var_inv_wl * p.p814);
        let assign5540_e7781: f64 = (assign5540_e7777 + assign5540_e7780);
        var_ua2_i = assign5540_e7781;

        let assign5550_e7785: f64 = (var_inv_l * p.p822);
        let assign5550_e7786: f64 = (p.p821 + assign5550_e7785);
        let assign5550_e7789: f64 = (var_inv_nfin * p.p823);
        let assign5550_e7790: f64 = (assign5550_e7786 + assign5550_e7789);
        let assign5550_e7793: f64 = (var_inv_lnfin * p.p824);
        let assign5550_e7794: f64 = (assign5550_e7790 + assign5550_e7793);
        let assign5550_e7797: f64 = (var_inv_w * p.p825);
        let assign5550_e7798: f64 = (assign5550_e7794 + assign5550_e7797);
        let assign5550_e7801: f64 = (var_inv_wl * p.p826);
        let assign5550_e7802: f64 = (assign5550_e7798 + assign5550_e7801);
        var_eu1_i = assign5550_e7802;

        let assign5560_e7806: f64 = (var_inv_l * p.p846);
        let assign5560_e7807: f64 = (p.p845 + assign5560_e7806);
        let assign5560_e7810: f64 = (var_inv_nfin * p.p847);
        let assign5560_e7811: f64 = (assign5560_e7807 + assign5560_e7810);
        let assign5560_e7814: f64 = (var_inv_lnfin * p.p848);
        let assign5560_e7815: f64 = (assign5560_e7811 + assign5560_e7814);
        let assign5560_e7818: f64 = (var_inv_w * p.p849);
        let assign5560_e7819: f64 = (assign5560_e7815 + assign5560_e7818);
        let assign5560_e7822: f64 = (var_inv_wl * p.p850);
        let assign5560_e7823: f64 = (assign5560_e7819 + assign5560_e7822);
        var_ud1_i = assign5560_e7823;

        let assign5570_e7827: f64 = (var_inv_l * p.p864);
        let assign5570_e7828: f64 = (p.p863 + assign5570_e7827);
        let assign5570_e7831: f64 = (var_inv_nfin * p.p865);
        let assign5570_e7832: f64 = (assign5570_e7828 + assign5570_e7831);
        let assign5570_e7835: f64 = (var_inv_lnfin * p.p866);
        let assign5570_e7836: f64 = (assign5570_e7832 + assign5570_e7835);
        let assign5570_e7839: f64 = (var_inv_w * p.p867);
        let assign5570_e7840: f64 = (assign5570_e7836 + assign5570_e7839);
        let assign5570_e7843: f64 = (var_inv_wl * p.p868);
        let assign5570_e7844: f64 = (assign5570_e7840 + assign5570_e7843);
        var_ud2_i = assign5570_e7844;

        let assign5580_e7848: f64 = (var_inv_l * p.p876);
        let assign5580_e7849: f64 = (p.p875 + assign5580_e7848);
        let assign5580_e7852: f64 = (var_inv_nfin * p.p877);
        let assign5580_e7853: f64 = (assign5580_e7849 + assign5580_e7852);
        let assign5580_e7856: f64 = (var_inv_lnfin * p.p878);
        let assign5580_e7857: f64 = (assign5580_e7853 + assign5580_e7856);
        let assign5580_e7860: f64 = (var_inv_w * p.p879);
        let assign5580_e7861: f64 = (assign5580_e7857 + assign5580_e7860);
        let assign5580_e7864: f64 = (var_inv_wl * p.p880);
        let assign5580_e7865: f64 = (assign5580_e7861 + assign5580_e7864);
        var_ucste_i = assign5580_e7865;

        let assign5590_e7869: f64 = (var_inv_l * p.p882);
        let assign5590_e7870: f64 = (p.p881 + assign5590_e7869);
        let assign5590_e7873: f64 = (var_inv_nfin * p.p883);
        let assign5590_e7874: f64 = (assign5590_e7870 + assign5590_e7873);
        let assign5590_e7877: f64 = (var_inv_lnfin * p.p884);
        let assign5590_e7878: f64 = (assign5590_e7874 + assign5590_e7877);
        let assign5590_e7881: f64 = (var_inv_w * p.p885);
        let assign5590_e7882: f64 = (assign5590_e7878 + assign5590_e7881);
        let assign5590_e7885: f64 = (var_inv_wl * p.p886);
        let assign5590_e7886: f64 = (assign5590_e7882 + assign5590_e7885);
        var_ucste1_i = assign5590_e7886;

        let assign5600_e7890: f64 = (var_inv_l * p.p576);
        let assign5600_e7891: f64 = (p.p575 + assign5600_e7890);
        let assign5600_e7894: f64 = (var_inv_nfin * p.p577);
        let assign5600_e7895: f64 = (assign5600_e7891 + assign5600_e7894);
        let assign5600_e7898: f64 = (var_inv_lnfin * p.p578);
        let assign5600_e7899: f64 = (assign5600_e7895 + assign5600_e7898);
        let assign5600_e7902: f64 = (var_inv_w * p.p579);
        let assign5600_e7903: f64 = (assign5600_e7899 + assign5600_e7902);
        let assign5600_e7906: f64 = (var_inv_wl * p.p580);
        let assign5600_e7907: f64 = (assign5600_e7903 + assign5600_e7906);
        var_ptwgt_i = assign5600_e7907;

        let assign5610_e7911: f64 = (var_inv_l * p.p556);
        let assign5610_e7912: f64 = (p.p555 + assign5610_e7911);
        let assign5610_e7915: f64 = (var_inv_nfin * p.p557);
        let assign5610_e7916: f64 = (assign5610_e7912 + assign5610_e7915);
        let assign5610_e7919: f64 = (var_inv_lnfin * p.p558);
        let assign5610_e7920: f64 = (assign5610_e7916 + assign5610_e7919);
        let assign5610_e7923: f64 = (var_inv_w * p.p559);
        let assign5610_e7924: f64 = (assign5610_e7920 + assign5610_e7923);
        let assign5610_e7927: f64 = (var_inv_wl * p.p560);
        let assign5610_e7928: f64 = (assign5610_e7924 + assign5610_e7927);
        var_at_i = assign5610_e7928;

        let assign5620_e7932: f64 = (var_inv_l * p.p569);
        let assign5620_e7933: f64 = (p.p568 + assign5620_e7932);
        let assign5620_e7936: f64 = (var_inv_nfin * p.p570);
        let assign5620_e7937: f64 = (assign5620_e7933 + assign5620_e7936);
        let assign5620_e7940: f64 = (var_inv_lnfin * p.p571);
        let assign5620_e7941: f64 = (assign5620_e7937 + assign5620_e7940);
        let assign5620_e7944: f64 = (var_inv_w * p.p572);
        let assign5620_e7945: f64 = (assign5620_e7941 + assign5620_e7944);
        let assign5620_e7948: f64 = (var_inv_wl * p.p573);
        let assign5620_e7949: f64 = (assign5620_e7945 + assign5620_e7948);
        var_atcv_i = assign5620_e7949;

        let assign5630_e7953: f64 = (var_inv_l * p.p962);
        let assign5630_e7954: f64 = (p.p961 + assign5630_e7953);
        let assign5630_e7957: f64 = (var_inv_nfin * p.p963);
        let assign5630_e7958: f64 = (assign5630_e7954 + assign5630_e7957);
        let assign5630_e7961: f64 = (var_inv_lnfin * p.p964);
        let assign5630_e7962: f64 = (assign5630_e7958 + assign5630_e7961);
        let assign5630_e7965: f64 = (var_inv_w * p.p965);
        let assign5630_e7966: f64 = (assign5630_e7962 + assign5630_e7965);
        let assign5630_e7969: f64 = (var_inv_wl * p.p966);
        let assign5630_e7970: f64 = (assign5630_e7966 + assign5630_e7969);
        var_prt_i = assign5630_e7970;

        let assign5640_e7974: f64 = (var_inv_l * p.p968);
        let assign5640_e7975: f64 = (p.p967 + assign5640_e7974);
        let assign5640_e7978: f64 = (var_inv_nfin * p.p969);
        let assign5640_e7979: f64 = (assign5640_e7975 + assign5640_e7978);
        let assign5640_e7982: f64 = (var_inv_lnfin * p.p970);
        let assign5640_e7983: f64 = (assign5640_e7979 + assign5640_e7982);
        let assign5640_e7986: f64 = (var_inv_w * p.p971);
        let assign5640_e7987: f64 = (assign5640_e7983 + assign5640_e7986);
        let assign5640_e7990: f64 = (var_inv_wl * p.p972);
        let assign5640_e7991: f64 = (assign5640_e7987 + assign5640_e7990);
        var_prt1_i = assign5640_e7991;

        let assign5650_e7995: f64 = (var_inv_l * p.p974);
        let assign5650_e7996: f64 = (p.p973 + assign5650_e7995);
        let assign5650_e7999: f64 = (var_inv_nfin * p.p975);
        let assign5650_e8000: f64 = (assign5650_e7996 + assign5650_e7999);
        let assign5650_e8003: f64 = (var_inv_lnfin * p.p976);
        let assign5650_e8004: f64 = (assign5650_e8000 + assign5650_e8003);
        let assign5650_e8007: f64 = (var_inv_w * p.p977);
        let assign5650_e8008: f64 = (assign5650_e8004 + assign5650_e8007);
        let assign5650_e8011: f64 = (var_inv_wl * p.p978);
        let assign5650_e8012: f64 = (assign5650_e8008 + assign5650_e8011);
        var_tr0_i = assign5650_e8012;

        let assign5660_e8016: f64 = (var_inv_l * p.p980);
        let assign5660_e8017: f64 = (p.p979 + assign5660_e8016);
        let assign5660_e8020: f64 = (var_inv_nfin * p.p981);
        let assign5660_e8021: f64 = (assign5660_e8017 + assign5660_e8020);
        let assign5660_e8024: f64 = (var_inv_lnfin * p.p982);
        let assign5660_e8025: f64 = (assign5660_e8021 + assign5660_e8024);
        let assign5660_e8028: f64 = (var_inv_w * p.p983);
        let assign5660_e8029: f64 = (assign5660_e8025 + assign5660_e8028);
        let assign5660_e8032: f64 = (var_inv_wl * p.p984);
        let assign5660_e8033: f64 = (assign5660_e8029 + assign5660_e8032);
        var_sprt_i = assign5660_e8033;

        let assign5670_e8037: f64 = (var_inv_l * p.p1742);
        let assign5670_e8038: f64 = (p.p1741 + assign5670_e8037);
        let assign5670_e8041: f64 = (var_inv_nfin * p.p1743);
        let assign5670_e8042: f64 = (assign5670_e8038 + assign5670_e8041);
        let assign5670_e8045: f64 = (var_inv_lnfin * p.p1744);
        let assign5670_e8046: f64 = (assign5670_e8042 + assign5670_e8045);
        let assign5670_e8049: f64 = (var_inv_w * p.p1745);
        let assign5670_e8050: f64 = (assign5670_e8046 + assign5670_e8049);
        let assign5670_e8053: f64 = (var_inv_wl * p.p1746);
        let assign5670_e8054: f64 = (assign5670_e8050 + assign5670_e8053);
        var_kt1_i = assign5670_e8054;

        let assign5680_e8058: f64 = (var_inv_l * p.p1751);
        let assign5680_e8059: f64 = (p.p1750 + assign5680_e8058);
        let assign5680_e8062: f64 = (var_inv_nfin * p.p1752);
        let assign5680_e8063: f64 = (assign5680_e8059 + assign5680_e8062);
        let assign5680_e8066: f64 = (var_inv_lnfin * p.p1753);
        let assign5680_e8067: f64 = (assign5680_e8063 + assign5680_e8066);
        let assign5680_e8070: f64 = (var_inv_w * p.p1754);
        let assign5680_e8071: f64 = (assign5680_e8067 + assign5680_e8070);
        let assign5680_e8074: f64 = (var_inv_wl * p.p1755);
        let assign5680_e8075: f64 = (assign5680_e8071 + assign5680_e8074);
        var_tss_i = assign5680_e8075;

        let assign5690_e8079: f64 = (var_inv_l * p.p1757);
        let assign5690_e8080: f64 = (p.p1756 + assign5690_e8079);
        let assign5690_e8083: f64 = (var_inv_nfin * p.p1758);
        let assign5690_e8084: f64 = (assign5690_e8080 + assign5690_e8083);
        let assign5690_e8087: f64 = (var_inv_lnfin * p.p1759);
        let assign5690_e8088: f64 = (assign5690_e8084 + assign5690_e8087);
        let assign5690_e8091: f64 = (var_inv_w * p.p1760);
        let assign5690_e8092: f64 = (assign5690_e8088 + assign5690_e8091);
        let assign5690_e8095: f64 = (var_inv_wl * p.p1761);
        let assign5690_e8096: f64 = (assign5690_e8092 + assign5690_e8095);
        var_iit_i = assign5690_e8096;

        let assign5700_e8100: f64 = (var_inv_l * p.p1769);
        let assign5700_e8101: f64 = (p.p1768 + assign5700_e8100);
        let assign5700_e8104: f64 = (var_inv_nfin * p.p1770);
        let assign5700_e8105: f64 = (assign5700_e8101 + assign5700_e8104);
        let assign5700_e8108: f64 = (var_inv_lnfin * p.p1771);
        let assign5700_e8109: f64 = (assign5700_e8105 + assign5700_e8108);
        let assign5700_e8112: f64 = (var_inv_w * p.p1772);
        let assign5700_e8113: f64 = (assign5700_e8109 + assign5700_e8112);
        let assign5700_e8116: f64 = (var_inv_wl * p.p1773);
        let assign5700_e8117: f64 = (assign5700_e8113 + assign5700_e8116);
        var_tgidl_i = assign5700_e8117;

        let assign5710_e8121: f64 = (var_inv_l * p.p1775);
        let assign5710_e8122: f64 = (p.p1774 + assign5710_e8121);
        let assign5710_e8125: f64 = (var_inv_nfin * p.p1776);
        let assign5710_e8126: f64 = (assign5710_e8122 + assign5710_e8125);
        let assign5710_e8129: f64 = (var_inv_lnfin * p.p1777);
        let assign5710_e8130: f64 = (assign5710_e8126 + assign5710_e8129);
        let assign5710_e8133: f64 = (var_inv_w * p.p1778);
        let assign5710_e8134: f64 = (assign5710_e8130 + assign5710_e8133);
        let assign5710_e8137: f64 = (var_inv_wl * p.p1779);
        let assign5710_e8138: f64 = (assign5710_e8134 + assign5710_e8137);
        var_ttat_i = assign5710_e8138;

        let assign5720_e8142: f64 = (var_inv_l * p.p1781);
        let assign5720_e8143: f64 = (p.p1780 + assign5720_e8142);
        let assign5720_e8146: f64 = (var_inv_nfin * p.p1782);
        let assign5720_e8147: f64 = (assign5720_e8143 + assign5720_e8146);
        let assign5720_e8150: f64 = (var_inv_lnfin * p.p1783);
        let assign5720_e8151: f64 = (assign5720_e8147 + assign5720_e8150);
        let assign5720_e8154: f64 = (var_inv_w * p.p1784);
        let assign5720_e8155: f64 = (assign5720_e8151 + assign5720_e8154);
        let assign5720_e8158: f64 = (var_inv_wl * p.p1785);
        let assign5720_e8159: f64 = (assign5720_e8155 + assign5720_e8158);
        var_igt_i = assign5720_e8159;

        let assign5730_e8163: f64 = (var_inv_l * p.p177);
        let assign5730_e8164: f64 = (p.p176 + assign5730_e8163);
        let assign5730_e8167: f64 = (var_inv_nfin * p.p178);
        let assign5730_e8168: f64 = (assign5730_e8164 + assign5730_e8167);
        let assign5730_e8171: f64 = (var_inv_lnfin * p.p179);
        let assign5730_e8172: f64 = (assign5730_e8168 + assign5730_e8171);
        let assign5730_e8175: f64 = (var_inv_w * p.p180);
        let assign5730_e8176: f64 = (assign5730_e8172 + assign5730_e8175);
        let assign5730_e8179: f64 = (var_inv_wl * p.p181);
        let assign5730_e8180: f64 = (assign5730_e8176 + assign5730_e8179);
        var_dvtp0_i = assign5730_e8180;
        var_dvtp0_i_dn0 = 0.0;
        var_dvtp0_i_dn2 = 0.0;
        var_dvtp0_i_dn3 = 0.0;
        var_dvtp0_i_dn4 = 0.0;
        var_dvtp0_i_dn5 = 0.0;
        var_dvtp0_i_dn6 = 0.0;
        var_dvtp0_i_dn7 = 0.0;
        var_dvtp0_i_dn8 = 0.0;
        var_dvtp0_i_dn9 = 0.0;
        var_dvtp0_i_dn10 = 0.0;
        var_dvtp0_i_dn11 = 0.0;
        var_dvtp0_i_dn13 = 0.0;
        var_dvtp0_i_dn14 = 0.0;

        let assign5740_e8184: f64 = (var_inv_l * p.p183);
        let assign5740_e8185: f64 = (p.p182 + assign5740_e8184);
        let assign5740_e8188: f64 = (var_inv_nfin * p.p184);
        let assign5740_e8189: f64 = (assign5740_e8185 + assign5740_e8188);
        let assign5740_e8192: f64 = (var_inv_lnfin * p.p185);
        let assign5740_e8193: f64 = (assign5740_e8189 + assign5740_e8192);
        let assign5740_e8196: f64 = (var_inv_w * p.p186);
        let assign5740_e8197: f64 = (assign5740_e8193 + assign5740_e8196);
        let assign5740_e8200: f64 = (var_inv_wl * p.p187);
        let assign5740_e8201: f64 = (assign5740_e8197 + assign5740_e8200);
        var_dvtp1_i = assign5740_e8201;
        var_dvtp1_i_dn0 = 0.0;
        var_dvtp1_i_dn2 = 0.0;
        var_dvtp1_i_dn3 = 0.0;
        var_dvtp1_i_dn4 = 0.0;
        var_dvtp1_i_dn5 = 0.0;
        var_dvtp1_i_dn6 = 0.0;
        var_dvtp1_i_dn7 = 0.0;
        var_dvtp1_i_dn8 = 0.0;
        var_dvtp1_i_dn9 = 0.0;
        var_dvtp1_i_dn10 = 0.0;
        var_dvtp1_i_dn11 = 0.0;
        var_dvtp1_i_dn13 = 0.0;
        var_dvtp1_i_dn14 = 0.0;

        let assign5750_e8205: f64 = (var_inv_l * p.p1690);
        let assign5750_e8206: f64 = (p.p1689 + assign5750_e8205);
        let assign5750_e8209: f64 = (var_inv_nfin * p.p1691);
        let assign5750_e8210: f64 = (assign5750_e8206 + assign5750_e8209);
        let assign5750_e8213: f64 = (var_inv_lnfin * p.p1692);
        let assign5750_e8214: f64 = (assign5750_e8210 + assign5750_e8213);
        let assign5750_e8217: f64 = (var_inv_w * p.p1693);
        let assign5750_e8218: f64 = (assign5750_e8214 + assign5750_e8217);
        let assign5750_e8221: f64 = (var_inv_wl * p.p1694);
        let assign5750_e8222: f64 = (assign5750_e8218 + assign5750_e8221);
        var_noia2_i = assign5750_e8222;

        let assign5760_e8226: f64 = (var_inv_l * p.p1702);
        let assign5760_e8227: f64 = (p.p1701 + assign5760_e8226);
        let assign5760_e8230: f64 = (var_inv_nfin * p.p1703);
        let assign5760_e8231: f64 = (assign5760_e8227 + assign5760_e8230);
        let assign5760_e8234: f64 = (var_inv_lnfin * p.p1704);
        let assign5760_e8235: f64 = (assign5760_e8231 + assign5760_e8234);
        let assign5760_e8238: f64 = (var_inv_w * p.p1705);
        let assign5760_e8239: f64 = (assign5760_e8235 + assign5760_e8238);
        let assign5760_e8242: f64 = (var_inv_wl * p.p1706);
        let assign5760_e8243: f64 = (assign5760_e8239 + assign5760_e8242);
        var_qsref_i = assign5760_e8243;

        let assign5770_e8247: f64 = (var_inv_l * p.p1696);
        let assign5770_e8248: f64 = (p.p1695 + assign5770_e8247);
        let assign5770_e8251: f64 = (var_inv_nfin * p.p1697);
        let assign5770_e8252: f64 = (assign5770_e8248 + assign5770_e8251);
        let assign5770_e8255: f64 = (var_inv_lnfin * p.p1698);
        let assign5770_e8256: f64 = (assign5770_e8252 + assign5770_e8255);
        let assign5770_e8259: f64 = (var_inv_w * p.p1699);
        let assign5770_e8260: f64 = (assign5770_e8256 + assign5770_e8259);
        let assign5770_e8263: f64 = (var_inv_wl * p.p1700);
        let assign5770_e8264: f64 = (assign5770_e8260 + assign5770_e8263);
        var_mpower_i = assign5770_e8264;

        let assign5780_e8267: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard42 = assign5780_e8267;

        let (assign5790_e8291,) = {
    if (var_guard42 != 0.0) {
        let assign5790_e8272: f64 = (var_inv_l * p.p357);
        let assign5790_e8273: f64 = (p.p356 + assign5790_e8272);
        let assign5790_e8276: f64 = (var_inv_nfin * p.p358);
        let assign5790_e8277: f64 = (assign5790_e8273 + assign5790_e8276);
        let assign5790_e8280: f64 = (var_inv_lnfin * p.p359);
        let assign5790_e8281: f64 = (assign5790_e8277 + assign5790_e8280);
        let assign5790_e8284: f64 = (var_inv_w * p.p360);
        let assign5790_e8285: f64 = (assign5790_e8281 + assign5790_e8284);
        let assign5790_e8288: f64 = (var_inv_wl * p.p361);
        let assign5790_e8289: f64 = (assign5790_e8285 + assign5790_e8288);
        (assign5790_e8289,)
    } else {
        (var_phibe_i,)
    }
};
        var_phibe_i = assign5790_e8291;

        let (assign5800_e8315,) = {
    if (var_guard42 != 0.0) {
        let assign5800_e8296: f64 = (var_inv_l * p.p363);
        let assign5800_e8297: f64 = (p.p362 + assign5800_e8296);
        let assign5800_e8300: f64 = (var_inv_nfin * p.p364);
        let assign5800_e8301: f64 = (assign5800_e8297 + assign5800_e8300);
        let assign5800_e8304: f64 = (var_inv_lnfin * p.p365);
        let assign5800_e8305: f64 = (assign5800_e8301 + assign5800_e8304);
        let assign5800_e8308: f64 = (var_inv_w * p.p366);
        let assign5800_e8309: f64 = (assign5800_e8305 + assign5800_e8308);
        let assign5800_e8312: f64 = (var_inv_wl * p.p367);
        let assign5800_e8313: f64 = (assign5800_e8309 + assign5800_e8312);
        (assign5800_e8313,)
    } else {
        (var_k1_i,)
    }
};
        var_k1_i = assign5800_e8315;

        let (assign5810_e8339,) = {
    if (var_guard42 != 0.0) {
        let assign5810_e8320: f64 = (var_inv_l * p.p369);
        let assign5810_e8321: f64 = (p.p368 + assign5810_e8320);
        let assign5810_e8324: f64 = (var_inv_nfin * p.p370);
        let assign5810_e8325: f64 = (assign5810_e8321 + assign5810_e8324);
        let assign5810_e8328: f64 = (var_inv_lnfin * p.p371);
        let assign5810_e8329: f64 = (assign5810_e8325 + assign5810_e8328);
        let assign5810_e8332: f64 = (var_inv_w * p.p372);
        let assign5810_e8333: f64 = (assign5810_e8329 + assign5810_e8332);
        let assign5810_e8336: f64 = (var_inv_wl * p.p373);
        let assign5810_e8337: f64 = (assign5810_e8333 + assign5810_e8336);
        (assign5810_e8337,)
    } else {
        (var_k11_i,)
    }
};
        var_k11_i = assign5810_e8339;

        let (assign5820_e8363,) = {
    if (var_guard42 != 0.0) {
        let assign5820_e8344: f64 = (var_inv_l * p.p660);
        let assign5820_e8345: f64 = (p.p659 + assign5820_e8344);
        let assign5820_e8348: f64 = (var_inv_nfin * p.p661);
        let assign5820_e8349: f64 = (assign5820_e8345 + assign5820_e8348);
        let assign5820_e8352: f64 = (var_inv_lnfin * p.p662);
        let assign5820_e8353: f64 = (assign5820_e8349 + assign5820_e8352);
        let assign5820_e8356: f64 = (var_inv_w * p.p663);
        let assign5820_e8357: f64 = (assign5820_e8353 + assign5820_e8356);
        let assign5820_e8360: f64 = (var_inv_wl * p.p664);
        let assign5820_e8361: f64 = (assign5820_e8357 + assign5820_e8360);
        (assign5820_e8361,)
    } else {
        (var_uc_i,)
    }
};
        var_uc_i = assign5820_e8363;

        let (assign5830_e8387,) = {
    if (var_guard42 != 0.0) {
        let assign5830_e8368: f64 = (var_inv_l * p.p828);
        let assign5830_e8369: f64 = (p.p827 + assign5830_e8368);
        let assign5830_e8372: f64 = (var_inv_nfin * p.p829);
        let assign5830_e8373: f64 = (assign5830_e8369 + assign5830_e8372);
        let assign5830_e8376: f64 = (var_inv_lnfin * p.p830);
        let assign5830_e8377: f64 = (assign5830_e8373 + assign5830_e8376);
        let assign5830_e8380: f64 = (var_inv_w * p.p831);
        let assign5830_e8381: f64 = (assign5830_e8377 + assign5830_e8380);
        let assign5830_e8384: f64 = (var_inv_wl * p.p832);
        let assign5830_e8385: f64 = (assign5830_e8381 + assign5830_e8384);
        (assign5830_e8385,)
    } else {
        (var_uc1_i,)
    }
};
        var_uc1_i = assign5830_e8387;

        let assign5840_e8390: f64 = if p.p61 == 2.0 { 1.0 } else { 0.0 };
        var_guard43 = assign5840_e8390;

        let (assign5850_e8416,) = {
    if ((var_guard42 != 0.0) && (var_guard43 != 0.0)) {
        let assign5850_e8397: f64 = (var_inv_l * p.p387);
        let assign5850_e8398: f64 = (p.p386 + assign5850_e8397);
        let assign5850_e8401: f64 = (var_inv_nfin * p.p388);
        let assign5850_e8402: f64 = (assign5850_e8398 + assign5850_e8401);
        let assign5850_e8405: f64 = (var_inv_lnfin * p.p389);
        let assign5850_e8406: f64 = (assign5850_e8402 + assign5850_e8405);
        let assign5850_e8409: f64 = (var_inv_w * p.p390);
        let assign5850_e8410: f64 = (assign5850_e8406 + assign5850_e8409);
        let assign5850_e8413: f64 = (var_inv_wl * p.p391);
        let assign5850_e8414: f64 = (assign5850_e8410 + assign5850_e8413);
        (assign5850_e8414,)
    } else {
        (var_k2_i,)
    }
};
        var_k2_i = assign5850_e8416;

        let (assign5860_e8442,) = {
    if ((var_guard42 != 0.0) && (var_guard43 != 0.0)) {
        let assign5860_e8423: f64 = (var_inv_l * p.p393);
        let assign5860_e8424: f64 = (p.p392 + assign5860_e8423);
        let assign5860_e8427: f64 = (var_inv_nfin * p.p394);
        let assign5860_e8428: f64 = (assign5860_e8424 + assign5860_e8427);
        let assign5860_e8431: f64 = (var_inv_lnfin * p.p395);
        let assign5860_e8432: f64 = (assign5860_e8428 + assign5860_e8431);
        let assign5860_e8435: f64 = (var_inv_w * p.p396);
        let assign5860_e8436: f64 = (assign5860_e8432 + assign5860_e8435);
        let assign5860_e8439: f64 = (var_inv_wl * p.p397);
        let assign5860_e8440: f64 = (assign5860_e8436 + assign5860_e8439);
        (assign5860_e8440,)
    } else {
        (var_k21_i,)
    }
};
        var_k21_i = assign5860_e8442;

        *var_at_i_slot = var_at_i;
        *var_atcv_i_slot = var_atcv_i;
        *var_dvtp0_i_slot = var_dvtp0_i;
        *var_dvtp0_i_dn0_slot = var_dvtp0_i_dn0;
        *var_dvtp0_i_dn10_slot = var_dvtp0_i_dn10;
        *var_dvtp0_i_dn11_slot = var_dvtp0_i_dn11;
        *var_dvtp0_i_dn13_slot = var_dvtp0_i_dn13;
        *var_dvtp0_i_dn14_slot = var_dvtp0_i_dn14;
        *var_dvtp0_i_dn2_slot = var_dvtp0_i_dn2;
        *var_dvtp0_i_dn3_slot = var_dvtp0_i_dn3;
        *var_dvtp0_i_dn4_slot = var_dvtp0_i_dn4;
        *var_dvtp0_i_dn5_slot = var_dvtp0_i_dn5;
        *var_dvtp0_i_dn6_slot = var_dvtp0_i_dn6;
        *var_dvtp0_i_dn7_slot = var_dvtp0_i_dn7;
        *var_dvtp0_i_dn8_slot = var_dvtp0_i_dn8;
        *var_dvtp0_i_dn9_slot = var_dvtp0_i_dn9;
        *var_dvtp1_i_slot = var_dvtp1_i;
        *var_dvtp1_i_dn0_slot = var_dvtp1_i_dn0;
        *var_dvtp1_i_dn10_slot = var_dvtp1_i_dn10;
        *var_dvtp1_i_dn11_slot = var_dvtp1_i_dn11;
        *var_dvtp1_i_dn13_slot = var_dvtp1_i_dn13;
        *var_dvtp1_i_dn14_slot = var_dvtp1_i_dn14;
        *var_dvtp1_i_dn2_slot = var_dvtp1_i_dn2;
        *var_dvtp1_i_dn3_slot = var_dvtp1_i_dn3;
        *var_dvtp1_i_dn4_slot = var_dvtp1_i_dn4;
        *var_dvtp1_i_dn5_slot = var_dvtp1_i_dn5;
        *var_dvtp1_i_dn6_slot = var_dvtp1_i_dn6;
        *var_dvtp1_i_dn7_slot = var_dvtp1_i_dn7;
        *var_dvtp1_i_dn8_slot = var_dvtp1_i_dn8;
        *var_dvtp1_i_dn9_slot = var_dvtp1_i_dn9;
        *var_emobt_i_slot = var_emobt_i;
        *var_eu1_i_slot = var_eu1_i;
        *var_guard42_slot = var_guard42;
        *var_guard43_slot = var_guard43;
        *var_igt_i_slot = var_igt_i;
        *var_iit_i_slot = var_iit_i;
        *var_k11_i_slot = var_k11_i;
        *var_k1_i_slot = var_k1_i;
        *var_k21_i_slot = var_k21_i;
        *var_k2_i_slot = var_k2_i;
        *var_kt1_i_slot = var_kt1_i;
        *var_mpower_i_slot = var_mpower_i;
        *var_noia2_i_slot = var_noia2_i;
        *var_phibe_i_slot = var_phibe_i;
        *var_prt1_i_slot = var_prt1_i;
        *var_prt_i_slot = var_prt_i;
        *var_ptwgt_i_slot = var_ptwgt_i;
        *var_qsref_i_slot = var_qsref_i;
        *var_sprt_i_slot = var_sprt_i;
        *var_tgidl_i_slot = var_tgidl_i;
        *var_tr0_i_slot = var_tr0_i;
        *var_tss_i_slot = var_tss_i;
        *var_ttat_i_slot = var_ttat_i;
        *var_ua1_i_slot = var_ua1_i;
        *var_ua2_i_slot = var_ua2_i;
        *var_uc1_i_slot = var_uc1_i;
        *var_uc_i_slot = var_uc_i;
        *var_ucste1_i_slot = var_ucste1_i;
        *var_ucste_i_slot = var_ucste_i;
        *var_ud1_i_slot = var_ud1_i;
        *var_ud2_i_slot = var_ud2_i;
        *var_ute1_i_slot = var_ute1_i;
        *var_utl_i_slot = var_utl_i;
    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        var_guard42: f64,
        var_guard43: f64,
        var_inv_l: f64,
        var_inv_lnfin: f64,
        var_inv_nfin: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_agidlb_i_slot: &mut f64,
        var_agislb_i_slot: &mut f64,
        var_bgidlb_i_slot: &mut f64,
        var_bgislb_i_slot: &mut f64,
        var_cdscdr_i_slot: &mut f64,
        var_cgidlb_i_slot: &mut f64,
        var_cgislb_i_slot: &mut f64,
        var_citr_i_slot: &mut f64,
        var_dvtshiftr_i_slot: &mut f64,
        var_egidlb_i_slot: &mut f64,
        var_egislb_i_slot: &mut f64,
        var_eta0r_i_slot: &mut f64,
        var_guard44_slot: &mut f64,
        var_guard45_slot: &mut f64,
        var_k2sat1_i_slot: &mut f64,
        var_k2sat_i_slot: &mut f64,
        var_mexpr_i_slot: &mut f64,
        var_mexpr_i_dn0_slot: &mut f64,
        var_mexpr_i_dn10_slot: &mut f64,
        var_mexpr_i_dn11_slot: &mut f64,
        var_mexpr_i_dn13_slot: &mut f64,
        var_mexpr_i_dn14_slot: &mut f64,
        var_mexpr_i_dn2_slot: &mut f64,
        var_mexpr_i_dn3_slot: &mut f64,
        var_mexpr_i_dn4_slot: &mut f64,
        var_mexpr_i_dn5_slot: &mut f64,
        var_mexpr_i_dn6_slot: &mut f64,
        var_mexpr_i_dn7_slot: &mut f64,
        var_mexpr_i_dn8_slot: &mut f64,
        var_mexpr_i_dn9_slot: &mut f64,
        var_pclmr_i_slot: &mut f64,
        var_pclmr_i_dn0_slot: &mut f64,
        var_pclmr_i_dn10_slot: &mut f64,
        var_pclmr_i_dn11_slot: &mut f64,
        var_pclmr_i_dn13_slot: &mut f64,
        var_pclmr_i_dn14_slot: &mut f64,
        var_pclmr_i_dn2_slot: &mut f64,
        var_pclmr_i_dn3_slot: &mut f64,
        var_pclmr_i_dn4_slot: &mut f64,
        var_pclmr_i_dn5_slot: &mut f64,
        var_pclmr_i_dn6_slot: &mut f64,
        var_pclmr_i_dn7_slot: &mut f64,
        var_pclmr_i_dn8_slot: &mut f64,
        var_pclmr_i_dn9_slot: &mut f64,
        var_pdibl1r_i_slot: &mut f64,
        var_pdibl2r_i_slot: &mut f64,
        var_pgidlb_i_slot: &mut f64,
        var_pgislb_i_slot: &mut f64,
        var_ptwgr_i_slot: &mut f64,
        var_ptwgr_i_dn0_slot: &mut f64,
        var_ptwgr_i_dn10_slot: &mut f64,
        var_ptwgr_i_dn11_slot: &mut f64,
        var_ptwgr_i_dn13_slot: &mut f64,
        var_ptwgr_i_dn14_slot: &mut f64,
        var_ptwgr_i_dn2_slot: &mut f64,
        var_ptwgr_i_dn3_slot: &mut f64,
        var_ptwgr_i_dn4_slot: &mut f64,
        var_ptwgr_i_dn5_slot: &mut f64,
        var_ptwgr_i_dn6_slot: &mut f64,
        var_ptwgr_i_dn7_slot: &mut f64,
        var_ptwgr_i_dn8_slot: &mut f64,
        var_ptwgr_i_dn9_slot: &mut f64,
        var_vsat1r_i_slot: &mut f64,
        var_vsat1r_i_dn0_slot: &mut f64,
        var_vsat1r_i_dn10_slot: &mut f64,
        var_vsat1r_i_dn11_slot: &mut f64,
        var_vsat1r_i_dn13_slot: &mut f64,
        var_vsat1r_i_dn14_slot: &mut f64,
        var_vsat1r_i_dn2_slot: &mut f64,
        var_vsat1r_i_dn3_slot: &mut f64,
        var_vsat1r_i_dn4_slot: &mut f64,
        var_vsat1r_i_dn5_slot: &mut f64,
        var_vsat1r_i_dn6_slot: &mut f64,
        var_vsat1r_i_dn7_slot: &mut f64,
        var_vsat1r_i_dn8_slot: &mut f64,
        var_vsat1r_i_dn9_slot: &mut f64,
        var_vsatr_i_slot: &mut f64,
    ) {
        let mut var_agidlb_i: f64 = *var_agidlb_i_slot;
        let mut var_agislb_i: f64 = *var_agislb_i_slot;
        let mut var_bgidlb_i: f64 = *var_bgidlb_i_slot;
        let mut var_bgislb_i: f64 = *var_bgislb_i_slot;
        let mut var_cdscdr_i: f64 = *var_cdscdr_i_slot;
        let mut var_cgidlb_i: f64 = *var_cgidlb_i_slot;
        let mut var_cgislb_i: f64 = *var_cgislb_i_slot;
        let mut var_citr_i: f64 = *var_citr_i_slot;
        let mut var_dvtshiftr_i: f64 = *var_dvtshiftr_i_slot;
        let mut var_egidlb_i: f64 = *var_egidlb_i_slot;
        let mut var_egislb_i: f64 = *var_egislb_i_slot;
        let mut var_eta0r_i: f64 = *var_eta0r_i_slot;
        let mut var_guard44: f64 = *var_guard44_slot;
        let mut var_guard45: f64 = *var_guard45_slot;
        let mut var_k2sat1_i: f64 = *var_k2sat1_i_slot;
        let mut var_k2sat_i: f64 = *var_k2sat_i_slot;
        let mut var_mexpr_i: f64 = *var_mexpr_i_slot;
        let mut var_mexpr_i_dn0: f64 = *var_mexpr_i_dn0_slot;
        let mut var_mexpr_i_dn10: f64 = *var_mexpr_i_dn10_slot;
        let mut var_mexpr_i_dn11: f64 = *var_mexpr_i_dn11_slot;
        let mut var_mexpr_i_dn13: f64 = *var_mexpr_i_dn13_slot;
        let mut var_mexpr_i_dn14: f64 = *var_mexpr_i_dn14_slot;
        let mut var_mexpr_i_dn2: f64 = *var_mexpr_i_dn2_slot;
        let mut var_mexpr_i_dn3: f64 = *var_mexpr_i_dn3_slot;
        let mut var_mexpr_i_dn4: f64 = *var_mexpr_i_dn4_slot;
        let mut var_mexpr_i_dn5: f64 = *var_mexpr_i_dn5_slot;
        let mut var_mexpr_i_dn6: f64 = *var_mexpr_i_dn6_slot;
        let mut var_mexpr_i_dn7: f64 = *var_mexpr_i_dn7_slot;
        let mut var_mexpr_i_dn8: f64 = *var_mexpr_i_dn8_slot;
        let mut var_mexpr_i_dn9: f64 = *var_mexpr_i_dn9_slot;
        let mut var_pclmr_i: f64 = *var_pclmr_i_slot;
        let mut var_pclmr_i_dn0: f64 = *var_pclmr_i_dn0_slot;
        let mut var_pclmr_i_dn10: f64 = *var_pclmr_i_dn10_slot;
        let mut var_pclmr_i_dn11: f64 = *var_pclmr_i_dn11_slot;
        let mut var_pclmr_i_dn13: f64 = *var_pclmr_i_dn13_slot;
        let mut var_pclmr_i_dn14: f64 = *var_pclmr_i_dn14_slot;
        let mut var_pclmr_i_dn2: f64 = *var_pclmr_i_dn2_slot;
        let mut var_pclmr_i_dn3: f64 = *var_pclmr_i_dn3_slot;
        let mut var_pclmr_i_dn4: f64 = *var_pclmr_i_dn4_slot;
        let mut var_pclmr_i_dn5: f64 = *var_pclmr_i_dn5_slot;
        let mut var_pclmr_i_dn6: f64 = *var_pclmr_i_dn6_slot;
        let mut var_pclmr_i_dn7: f64 = *var_pclmr_i_dn7_slot;
        let mut var_pclmr_i_dn8: f64 = *var_pclmr_i_dn8_slot;
        let mut var_pclmr_i_dn9: f64 = *var_pclmr_i_dn9_slot;
        let mut var_pdibl1r_i: f64 = *var_pdibl1r_i_slot;
        let mut var_pdibl2r_i: f64 = *var_pdibl2r_i_slot;
        let mut var_pgidlb_i: f64 = *var_pgidlb_i_slot;
        let mut var_pgislb_i: f64 = *var_pgislb_i_slot;
        let mut var_ptwgr_i: f64 = *var_ptwgr_i_slot;
        let mut var_ptwgr_i_dn0: f64 = *var_ptwgr_i_dn0_slot;
        let mut var_ptwgr_i_dn10: f64 = *var_ptwgr_i_dn10_slot;
        let mut var_ptwgr_i_dn11: f64 = *var_ptwgr_i_dn11_slot;
        let mut var_ptwgr_i_dn13: f64 = *var_ptwgr_i_dn13_slot;
        let mut var_ptwgr_i_dn14: f64 = *var_ptwgr_i_dn14_slot;
        let mut var_ptwgr_i_dn2: f64 = *var_ptwgr_i_dn2_slot;
        let mut var_ptwgr_i_dn3: f64 = *var_ptwgr_i_dn3_slot;
        let mut var_ptwgr_i_dn4: f64 = *var_ptwgr_i_dn4_slot;
        let mut var_ptwgr_i_dn5: f64 = *var_ptwgr_i_dn5_slot;
        let mut var_ptwgr_i_dn6: f64 = *var_ptwgr_i_dn6_slot;
        let mut var_ptwgr_i_dn7: f64 = *var_ptwgr_i_dn7_slot;
        let mut var_ptwgr_i_dn8: f64 = *var_ptwgr_i_dn8_slot;
        let mut var_ptwgr_i_dn9: f64 = *var_ptwgr_i_dn9_slot;
        let mut var_vsat1r_i: f64 = *var_vsat1r_i_slot;
        let mut var_vsat1r_i_dn0: f64 = *var_vsat1r_i_dn0_slot;
        let mut var_vsat1r_i_dn10: f64 = *var_vsat1r_i_dn10_slot;
        let mut var_vsat1r_i_dn11: f64 = *var_vsat1r_i_dn11_slot;
        let mut var_vsat1r_i_dn13: f64 = *var_vsat1r_i_dn13_slot;
        let mut var_vsat1r_i_dn14: f64 = *var_vsat1r_i_dn14_slot;
        let mut var_vsat1r_i_dn2: f64 = *var_vsat1r_i_dn2_slot;
        let mut var_vsat1r_i_dn3: f64 = *var_vsat1r_i_dn3_slot;
        let mut var_vsat1r_i_dn4: f64 = *var_vsat1r_i_dn4_slot;
        let mut var_vsat1r_i_dn5: f64 = *var_vsat1r_i_dn5_slot;
        let mut var_vsat1r_i_dn6: f64 = *var_vsat1r_i_dn6_slot;
        let mut var_vsat1r_i_dn7: f64 = *var_vsat1r_i_dn7_slot;
        let mut var_vsat1r_i_dn8: f64 = *var_vsat1r_i_dn8_slot;
        let mut var_vsat1r_i_dn9: f64 = *var_vsat1r_i_dn9_slot;
        let mut var_vsatr_i: f64 = *var_vsatr_i_slot;

        let (assign5870_e8468,) = {
    if ((var_guard42 != 0.0) && (var_guard43 != 0.0)) {
        let assign5870_e8449: f64 = (var_inv_l * p.p375);
        let assign5870_e8450: f64 = (p.p374 + assign5870_e8449);
        let assign5870_e8453: f64 = (var_inv_nfin * p.p376);
        let assign5870_e8454: f64 = (assign5870_e8450 + assign5870_e8453);
        let assign5870_e8457: f64 = (var_inv_lnfin * p.p377);
        let assign5870_e8458: f64 = (assign5870_e8454 + assign5870_e8457);
        let assign5870_e8461: f64 = (var_inv_w * p.p378);
        let assign5870_e8462: f64 = (assign5870_e8458 + assign5870_e8461);
        let assign5870_e8465: f64 = (var_inv_wl * p.p379);
        let assign5870_e8466: f64 = (assign5870_e8462 + assign5870_e8465);
        (assign5870_e8466,)
    } else {
        (var_k2sat_i,)
    }
};
        var_k2sat_i = assign5870_e8468;

        let (assign5880_e8494,) = {
    if ((var_guard42 != 0.0) && (var_guard43 != 0.0)) {
        let assign5880_e8475: f64 = (var_inv_l * p.p381);
        let assign5880_e8476: f64 = (p.p380 + assign5880_e8475);
        let assign5880_e8479: f64 = (var_inv_nfin * p.p382);
        let assign5880_e8480: f64 = (assign5880_e8476 + assign5880_e8479);
        let assign5880_e8483: f64 = (var_inv_lnfin * p.p383);
        let assign5880_e8484: f64 = (assign5880_e8480 + assign5880_e8483);
        let assign5880_e8487: f64 = (var_inv_w * p.p384);
        let assign5880_e8488: f64 = (assign5880_e8484 + assign5880_e8487);
        let assign5880_e8491: f64 = (var_inv_wl * p.p385);
        let assign5880_e8492: f64 = (assign5880_e8488 + assign5880_e8491);
        (assign5880_e8492,)
    } else {
        (var_k2sat1_i,)
    }
};
        var_k2sat1_i = assign5880_e8494;

        let assign5890_e8513: f64 = if (((p.p70 == 2.0) || (p.p70 == 3.0)) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0))) { 1.0 } else { 0.0 };
        var_guard44 = assign5890_e8513;

        let (assign5900_e8539,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5900_e8520: f64 = (var_inv_l * p.p1378);
        let assign5900_e8521: f64 = (p.p1377 + assign5900_e8520);
        let assign5900_e8524: f64 = (var_inv_nfin * p.p1379);
        let assign5900_e8525: f64 = (assign5900_e8521 + assign5900_e8524);
        let assign5900_e8528: f64 = (var_inv_lnfin * p.p1380);
        let assign5900_e8529: f64 = (assign5900_e8525 + assign5900_e8528);
        let assign5900_e8532: f64 = (var_inv_w * p.p1381);
        let assign5900_e8533: f64 = (assign5900_e8529 + assign5900_e8532);
        let assign5900_e8536: f64 = (var_inv_wl * p.p1382);
        let assign5900_e8537: f64 = (assign5900_e8533 + assign5900_e8536);
        (assign5900_e8537,)
    } else {
        (var_agidlb_i,)
    }
};
        var_agidlb_i = assign5900_e8539;

        let (assign5910_e8565,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5910_e8546: f64 = (var_inv_l * p.p1384);
        let assign5910_e8547: f64 = (p.p1383 + assign5910_e8546);
        let assign5910_e8550: f64 = (var_inv_nfin * p.p1385);
        let assign5910_e8551: f64 = (assign5910_e8547 + assign5910_e8550);
        let assign5910_e8554: f64 = (var_inv_lnfin * p.p1386);
        let assign5910_e8555: f64 = (assign5910_e8551 + assign5910_e8554);
        let assign5910_e8558: f64 = (var_inv_w * p.p1387);
        let assign5910_e8559: f64 = (assign5910_e8555 + assign5910_e8558);
        let assign5910_e8562: f64 = (var_inv_wl * p.p1388);
        let assign5910_e8563: f64 = (assign5910_e8559 + assign5910_e8562);
        (assign5910_e8563,)
    } else {
        (var_bgidlb_i,)
    }
};
        var_bgidlb_i = assign5910_e8565;

        let (assign5920_e8591,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5920_e8572: f64 = (var_inv_l * p.p1390);
        let assign5920_e8573: f64 = (p.p1389 + assign5920_e8572);
        let assign5920_e8576: f64 = (var_inv_nfin * p.p1391);
        let assign5920_e8577: f64 = (assign5920_e8573 + assign5920_e8576);
        let assign5920_e8580: f64 = (var_inv_lnfin * p.p1392);
        let assign5920_e8581: f64 = (assign5920_e8577 + assign5920_e8580);
        let assign5920_e8584: f64 = (var_inv_w * p.p1393);
        let assign5920_e8585: f64 = (assign5920_e8581 + assign5920_e8584);
        let assign5920_e8588: f64 = (var_inv_wl * p.p1394);
        let assign5920_e8589: f64 = (assign5920_e8585 + assign5920_e8588);
        (assign5920_e8589,)
    } else {
        (var_cgidlb_i,)
    }
};
        var_cgidlb_i = assign5920_e8591;

        let (assign5930_e8617,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5930_e8598: f64 = (var_inv_l * p.p1396);
        let assign5930_e8599: f64 = (p.p1395 + assign5930_e8598);
        let assign5930_e8602: f64 = (var_inv_nfin * p.p1397);
        let assign5930_e8603: f64 = (assign5930_e8599 + assign5930_e8602);
        let assign5930_e8606: f64 = (var_inv_lnfin * p.p1398);
        let assign5930_e8607: f64 = (assign5930_e8603 + assign5930_e8606);
        let assign5930_e8610: f64 = (var_inv_w * p.p1399);
        let assign5930_e8611: f64 = (assign5930_e8607 + assign5930_e8610);
        let assign5930_e8614: f64 = (var_inv_wl * p.p1400);
        let assign5930_e8615: f64 = (assign5930_e8611 + assign5930_e8614);
        (assign5930_e8615,)
    } else {
        (var_egidlb_i,)
    }
};
        var_egidlb_i = assign5930_e8617;

        let (assign5940_e8643,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5940_e8624: f64 = (var_inv_l * p.p1402);
        let assign5940_e8625: f64 = (p.p1401 + assign5940_e8624);
        let assign5940_e8628: f64 = (var_inv_nfin * p.p1403);
        let assign5940_e8629: f64 = (assign5940_e8625 + assign5940_e8628);
        let assign5940_e8632: f64 = (var_inv_lnfin * p.p1404);
        let assign5940_e8633: f64 = (assign5940_e8629 + assign5940_e8632);
        let assign5940_e8636: f64 = (var_inv_w * p.p1405);
        let assign5940_e8637: f64 = (assign5940_e8633 + assign5940_e8636);
        let assign5940_e8640: f64 = (var_inv_wl * p.p1406);
        let assign5940_e8641: f64 = (assign5940_e8637 + assign5940_e8640);
        (assign5940_e8641,)
    } else {
        (var_pgidlb_i,)
    }
};
        var_pgidlb_i = assign5940_e8643;

        let (assign5950_e8669,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5950_e8650: f64 = (var_inv_l * p.p1408);
        let assign5950_e8651: f64 = (p.p1407 + assign5950_e8650);
        let assign5950_e8654: f64 = (var_inv_nfin * p.p1409);
        let assign5950_e8655: f64 = (assign5950_e8651 + assign5950_e8654);
        let assign5950_e8658: f64 = (var_inv_lnfin * p.p1410);
        let assign5950_e8659: f64 = (assign5950_e8655 + assign5950_e8658);
        let assign5950_e8662: f64 = (var_inv_w * p.p1411);
        let assign5950_e8663: f64 = (assign5950_e8659 + assign5950_e8662);
        let assign5950_e8666: f64 = (var_inv_wl * p.p1412);
        let assign5950_e8667: f64 = (assign5950_e8663 + assign5950_e8666);
        (assign5950_e8667,)
    } else {
        (var_agislb_i,)
    }
};
        var_agislb_i = assign5950_e8669;

        let (assign5960_e8695,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5960_e8676: f64 = (var_inv_l * p.p1414);
        let assign5960_e8677: f64 = (p.p1413 + assign5960_e8676);
        let assign5960_e8680: f64 = (var_inv_nfin * p.p1415);
        let assign5960_e8681: f64 = (assign5960_e8677 + assign5960_e8680);
        let assign5960_e8684: f64 = (var_inv_lnfin * p.p1416);
        let assign5960_e8685: f64 = (assign5960_e8681 + assign5960_e8684);
        let assign5960_e8688: f64 = (var_inv_w * p.p1417);
        let assign5960_e8689: f64 = (assign5960_e8685 + assign5960_e8688);
        let assign5960_e8692: f64 = (var_inv_wl * p.p1418);
        let assign5960_e8693: f64 = (assign5960_e8689 + assign5960_e8692);
        (assign5960_e8693,)
    } else {
        (var_bgislb_i,)
    }
};
        var_bgislb_i = assign5960_e8695;

        let (assign5970_e8721,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5970_e8702: f64 = (var_inv_l * p.p1420);
        let assign5970_e8703: f64 = (p.p1419 + assign5970_e8702);
        let assign5970_e8706: f64 = (var_inv_nfin * p.p1421);
        let assign5970_e8707: f64 = (assign5970_e8703 + assign5970_e8706);
        let assign5970_e8710: f64 = (var_inv_lnfin * p.p1422);
        let assign5970_e8711: f64 = (assign5970_e8707 + assign5970_e8710);
        let assign5970_e8714: f64 = (var_inv_w * p.p1423);
        let assign5970_e8715: f64 = (assign5970_e8711 + assign5970_e8714);
        let assign5970_e8718: f64 = (var_inv_wl * p.p1424);
        let assign5970_e8719: f64 = (assign5970_e8715 + assign5970_e8718);
        (assign5970_e8719,)
    } else {
        (var_cgislb_i,)
    }
};
        var_cgislb_i = assign5970_e8721;

        let (assign5980_e8747,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5980_e8728: f64 = (var_inv_l * p.p1426);
        let assign5980_e8729: f64 = (p.p1425 + assign5980_e8728);
        let assign5980_e8732: f64 = (var_inv_nfin * p.p1427);
        let assign5980_e8733: f64 = (assign5980_e8729 + assign5980_e8732);
        let assign5980_e8736: f64 = (var_inv_lnfin * p.p1428);
        let assign5980_e8737: f64 = (assign5980_e8733 + assign5980_e8736);
        let assign5980_e8740: f64 = (var_inv_w * p.p1429);
        let assign5980_e8741: f64 = (assign5980_e8737 + assign5980_e8740);
        let assign5980_e8744: f64 = (var_inv_wl * p.p1430);
        let assign5980_e8745: f64 = (assign5980_e8741 + assign5980_e8744);
        (assign5980_e8745,)
    } else {
        (var_egislb_i,)
    }
};
        var_egislb_i = assign5980_e8747;

        let (assign5990_e8773,) = {
    if ((var_guard42 != 0.0) && (var_guard44 != 0.0)) {
        let assign5990_e8754: f64 = (var_inv_l * p.p1432);
        let assign5990_e8755: f64 = (p.p1431 + assign5990_e8754);
        let assign5990_e8758: f64 = (var_inv_nfin * p.p1433);
        let assign5990_e8759: f64 = (assign5990_e8755 + assign5990_e8758);
        let assign5990_e8762: f64 = (var_inv_lnfin * p.p1434);
        let assign5990_e8763: f64 = (assign5990_e8759 + assign5990_e8762);
        let assign5990_e8766: f64 = (var_inv_w * p.p1435);
        let assign5990_e8767: f64 = (assign5990_e8763 + assign5990_e8766);
        let assign5990_e8770: f64 = (var_inv_wl * p.p1436);
        let assign5990_e8771: f64 = (assign5990_e8767 + assign5990_e8770);
        (assign5990_e8771,)
    } else {
        (var_pgislb_i,)
    }
};
        var_pgislb_i = assign5990_e8773;

        let assign6000_e8776: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard45 = assign6000_e8776;

        let (assign6010_e8800,) = {
    if (var_guard45 != 0.0) {
        let assign6010_e8781: f64 = (var_inv_l * p.p213);
        let assign6010_e8782: f64 = (p.p212 + assign6010_e8781);
        let assign6010_e8785: f64 = (var_inv_nfin * p.p214);
        let assign6010_e8786: f64 = (assign6010_e8782 + assign6010_e8785);
        let assign6010_e8789: f64 = (var_inv_lnfin * p.p215);
        let assign6010_e8790: f64 = (assign6010_e8786 + assign6010_e8789);
        let assign6010_e8793: f64 = (var_inv_w * p.p216);
        let assign6010_e8794: f64 = (assign6010_e8790 + assign6010_e8793);
        let assign6010_e8797: f64 = (var_inv_wl * p.p217);
        let assign6010_e8798: f64 = (assign6010_e8794 + assign6010_e8797);
        (assign6010_e8798,)
    } else {
        (var_cdscdr_i,)
    }
};
        var_cdscdr_i = assign6010_e8800;

        let (assign6020_e8824,) = {
    if (var_guard45 != 0.0) {
        let assign6020_e8805: f64 = (var_inv_l * p.p195);
        let assign6020_e8806: f64 = (p.p194 + assign6020_e8805);
        let assign6020_e8809: f64 = (var_inv_nfin * p.p196);
        let assign6020_e8810: f64 = (assign6020_e8806 + assign6020_e8809);
        let assign6020_e8813: f64 = (var_inv_lnfin * p.p197);
        let assign6020_e8814: f64 = (assign6020_e8810 + assign6020_e8813);
        let assign6020_e8817: f64 = (var_inv_w * p.p198);
        let assign6020_e8818: f64 = (assign6020_e8814 + assign6020_e8817);
        let assign6020_e8821: f64 = (var_inv_wl * p.p199);
        let assign6020_e8822: f64 = (assign6020_e8818 + assign6020_e8821);
        (assign6020_e8822,)
    } else {
        (var_citr_i,)
    }
};
        var_citr_i = assign6020_e8824;

        let (assign6030_e8848,) = {
    if (var_guard45 != 0.0) {
        let assign6030_e8829: f64 = (var_inv_l * p.p255);
        let assign6030_e8830: f64 = (p.p254 + assign6030_e8829);
        let assign6030_e8833: f64 = (var_inv_nfin * p.p256);
        let assign6030_e8834: f64 = (assign6030_e8830 + assign6030_e8833);
        let assign6030_e8837: f64 = (var_inv_lnfin * p.p257);
        let assign6030_e8838: f64 = (assign6030_e8834 + assign6030_e8837);
        let assign6030_e8841: f64 = (var_inv_w * p.p258);
        let assign6030_e8842: f64 = (assign6030_e8838 + assign6030_e8841);
        let assign6030_e8845: f64 = (var_inv_wl * p.p259);
        let assign6030_e8846: f64 = (assign6030_e8842 + assign6030_e8845);
        (assign6030_e8846,)
    } else {
        (var_eta0r_i,)
    }
};
        var_eta0r_i = assign6030_e8848;

        let (assign6040_e8872, assign6040_e8872_d_n0, assign6040_e8872_d_n2, assign6040_e8872_d_n3, assign6040_e8872_d_n4, assign6040_e8872_d_n5, assign6040_e8872_d_n6, assign6040_e8872_d_n7, assign6040_e8872_d_n8, assign6040_e8872_d_n9, assign6040_e8872_d_n10, assign6040_e8872_d_n11, assign6040_e8872_d_n13, assign6040_e8872_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6040_e8853: f64 = (var_inv_l * p.p474);
        let assign6040_e8854: f64 = (p.p473 + assign6040_e8853);
        let assign6040_e8857: f64 = (var_inv_nfin * p.p475);
        let assign6040_e8858: f64 = (assign6040_e8854 + assign6040_e8857);
        let assign6040_e8861: f64 = (var_inv_lnfin * p.p476);
        let assign6040_e8862: f64 = (assign6040_e8858 + assign6040_e8861);
        let assign6040_e8865: f64 = (var_inv_w * p.p477);
        let assign6040_e8866: f64 = (assign6040_e8862 + assign6040_e8865);
        let assign6040_e8869: f64 = (var_inv_wl * p.p478);
        let assign6040_e8870: f64 = (assign6040_e8866 + assign6040_e8869);
        (assign6040_e8870, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vsat1r_i, var_vsat1r_i_dn0, var_vsat1r_i_dn2, var_vsat1r_i_dn3, var_vsat1r_i_dn4, var_vsat1r_i_dn5, var_vsat1r_i_dn6, var_vsat1r_i_dn7, var_vsat1r_i_dn8, var_vsat1r_i_dn9, var_vsat1r_i_dn10, var_vsat1r_i_dn11, var_vsat1r_i_dn13, var_vsat1r_i_dn14,)
    }
};
        var_vsat1r_i = assign6040_e8872;
        var_vsat1r_i_dn0 = assign6040_e8872_d_n0;
        var_vsat1r_i_dn2 = assign6040_e8872_d_n2;
        var_vsat1r_i_dn3 = assign6040_e8872_d_n3;
        var_vsat1r_i_dn4 = assign6040_e8872_d_n4;
        var_vsat1r_i_dn5 = assign6040_e8872_d_n5;
        var_vsat1r_i_dn6 = assign6040_e8872_d_n6;
        var_vsat1r_i_dn7 = assign6040_e8872_d_n7;
        var_vsat1r_i_dn8 = assign6040_e8872_d_n8;
        var_vsat1r_i_dn9 = assign6040_e8872_d_n9;
        var_vsat1r_i_dn10 = assign6040_e8872_d_n10;
        var_vsat1r_i_dn11 = assign6040_e8872_d_n11;
        var_vsat1r_i_dn13 = assign6040_e8872_d_n13;
        var_vsat1r_i_dn14 = assign6040_e8872_d_n14;

        let (assign6050_e8896, assign6050_e8896_d_n0, assign6050_e8896_d_n2, assign6050_e8896_d_n3, assign6050_e8896_d_n4, assign6050_e8896_d_n5, assign6050_e8896_d_n6, assign6050_e8896_d_n7, assign6050_e8896_d_n8, assign6050_e8896_d_n9, assign6050_e8896_d_n10, assign6050_e8896_d_n11, assign6050_e8896_d_n13, assign6050_e8896_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6050_e8877: f64 = (var_inv_l * p.p538);
        let assign6050_e8878: f64 = (p.p537 + assign6050_e8877);
        let assign6050_e8881: f64 = (var_inv_nfin * p.p539);
        let assign6050_e8882: f64 = (assign6050_e8878 + assign6050_e8881);
        let assign6050_e8885: f64 = (var_inv_lnfin * p.p540);
        let assign6050_e8886: f64 = (assign6050_e8882 + assign6050_e8885);
        let assign6050_e8889: f64 = (var_inv_w * p.p541);
        let assign6050_e8890: f64 = (assign6050_e8886 + assign6050_e8889);
        let assign6050_e8893: f64 = (var_inv_wl * p.p542);
        let assign6050_e8894: f64 = (assign6050_e8890 + assign6050_e8893);
        (assign6050_e8894, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mexpr_i, var_mexpr_i_dn0, var_mexpr_i_dn2, var_mexpr_i_dn3, var_mexpr_i_dn4, var_mexpr_i_dn5, var_mexpr_i_dn6, var_mexpr_i_dn7, var_mexpr_i_dn8, var_mexpr_i_dn9, var_mexpr_i_dn10, var_mexpr_i_dn11, var_mexpr_i_dn13, var_mexpr_i_dn14,)
    }
};
        var_mexpr_i = assign6050_e8896;
        var_mexpr_i_dn0 = assign6050_e8896_d_n0;
        var_mexpr_i_dn2 = assign6050_e8896_d_n2;
        var_mexpr_i_dn3 = assign6050_e8896_d_n3;
        var_mexpr_i_dn4 = assign6050_e8896_d_n4;
        var_mexpr_i_dn5 = assign6050_e8896_d_n5;
        var_mexpr_i_dn6 = assign6050_e8896_d_n6;
        var_mexpr_i_dn7 = assign6050_e8896_d_n7;
        var_mexpr_i_dn8 = assign6050_e8896_d_n8;
        var_mexpr_i_dn9 = assign6050_e8896_d_n9;
        var_mexpr_i_dn10 = assign6050_e8896_d_n10;
        var_mexpr_i_dn11 = assign6050_e8896_d_n11;
        var_mexpr_i_dn13 = assign6050_e8896_d_n13;
        var_mexpr_i_dn14 = assign6050_e8896_d_n14;

        let (assign6060_e8920, assign6060_e8920_d_n0, assign6060_e8920_d_n2, assign6060_e8920_d_n3, assign6060_e8920_d_n4, assign6060_e8920_d_n5, assign6060_e8920_d_n6, assign6060_e8920_d_n7, assign6060_e8920_d_n8, assign6060_e8920_d_n9, assign6060_e8920_d_n10, assign6060_e8920_d_n11, assign6060_e8920_d_n13, assign6060_e8920_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6060_e8901: f64 = (var_inv_l * p.p550);
        let assign6060_e8902: f64 = (p.p549 + assign6060_e8901);
        let assign6060_e8905: f64 = (var_inv_nfin * p.p551);
        let assign6060_e8906: f64 = (assign6060_e8902 + assign6060_e8905);
        let assign6060_e8909: f64 = (var_inv_lnfin * p.p552);
        let assign6060_e8910: f64 = (assign6060_e8906 + assign6060_e8909);
        let assign6060_e8913: f64 = (var_inv_w * p.p553);
        let assign6060_e8914: f64 = (assign6060_e8910 + assign6060_e8913);
        let assign6060_e8917: f64 = (var_inv_wl * p.p554);
        let assign6060_e8918: f64 = (assign6060_e8914 + assign6060_e8917);
        (assign6060_e8918, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ptwgr_i, var_ptwgr_i_dn0, var_ptwgr_i_dn2, var_ptwgr_i_dn3, var_ptwgr_i_dn4, var_ptwgr_i_dn5, var_ptwgr_i_dn6, var_ptwgr_i_dn7, var_ptwgr_i_dn8, var_ptwgr_i_dn9, var_ptwgr_i_dn10, var_ptwgr_i_dn11, var_ptwgr_i_dn13, var_ptwgr_i_dn14,)
    }
};
        var_ptwgr_i = assign6060_e8920;
        var_ptwgr_i_dn0 = assign6060_e8920_d_n0;
        var_ptwgr_i_dn2 = assign6060_e8920_d_n2;
        var_ptwgr_i_dn3 = assign6060_e8920_d_n3;
        var_ptwgr_i_dn4 = assign6060_e8920_d_n4;
        var_ptwgr_i_dn5 = assign6060_e8920_d_n5;
        var_ptwgr_i_dn6 = assign6060_e8920_d_n6;
        var_ptwgr_i_dn7 = assign6060_e8920_d_n7;
        var_ptwgr_i_dn8 = assign6060_e8920_d_n8;
        var_ptwgr_i_dn9 = assign6060_e8920_d_n9;
        var_ptwgr_i_dn10 = assign6060_e8920_d_n10;
        var_ptwgr_i_dn11 = assign6060_e8920_d_n11;
        var_ptwgr_i_dn13 = assign6060_e8920_d_n13;
        var_ptwgr_i_dn14 = assign6060_e8920_d_n14;

        let (assign6070_e8944,) = {
    if (var_guard45 != 0.0) {
        let assign6070_e8925: f64 = (var_inv_l * p.p998);
        let assign6070_e8926: f64 = (p.p997 + assign6070_e8925);
        let assign6070_e8929: f64 = (var_inv_nfin * p.p999);
        let assign6070_e8930: f64 = (assign6070_e8926 + assign6070_e8929);
        let assign6070_e8933: f64 = (var_inv_lnfin * p.p1000);
        let assign6070_e8934: f64 = (assign6070_e8930 + assign6070_e8933);
        let assign6070_e8937: f64 = (var_inv_w * p.p1001);
        let assign6070_e8938: f64 = (assign6070_e8934 + assign6070_e8937);
        let assign6070_e8941: f64 = (var_inv_wl * p.p1002);
        let assign6070_e8942: f64 = (assign6070_e8938 + assign6070_e8941);
        (assign6070_e8942,)
    } else {
        (var_pdibl1r_i,)
    }
};
        var_pdibl1r_i = assign6070_e8944;

        let (assign6080_e8968,) = {
    if (var_guard45 != 0.0) {
        let assign6080_e8949: f64 = (var_inv_l * p.p1004);
        let assign6080_e8950: f64 = (p.p1003 + assign6080_e8949);
        let assign6080_e8953: f64 = (var_inv_nfin * p.p1005);
        let assign6080_e8954: f64 = (assign6080_e8950 + assign6080_e8953);
        let assign6080_e8957: f64 = (var_inv_lnfin * p.p1006);
        let assign6080_e8958: f64 = (assign6080_e8954 + assign6080_e8957);
        let assign6080_e8961: f64 = (var_inv_w * p.p1007);
        let assign6080_e8962: f64 = (assign6080_e8958 + assign6080_e8961);
        let assign6080_e8965: f64 = (var_inv_wl * p.p1008);
        let assign6080_e8966: f64 = (assign6080_e8962 + assign6080_e8965);
        (assign6080_e8966,)
    } else {
        (var_pdibl2r_i,)
    }
};
        var_pdibl2r_i = assign6080_e8968;

        let (assign6090_e8992, assign6090_e8992_d_n0, assign6090_e8992_d_n2, assign6090_e8992_d_n3, assign6090_e8992_d_n4, assign6090_e8992_d_n5, assign6090_e8992_d_n6, assign6090_e8992_d_n7, assign6090_e8992_d_n8, assign6090_e8992_d_n9, assign6090_e8992_d_n10, assign6090_e8992_d_n11, assign6090_e8992_d_n13, assign6090_e8992_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6090_e8973: f64 = (var_inv_l * p.p1033);
        let assign6090_e8974: f64 = (p.p1032 + assign6090_e8973);
        let assign6090_e8977: f64 = (var_inv_nfin * p.p1034);
        let assign6090_e8978: f64 = (assign6090_e8974 + assign6090_e8977);
        let assign6090_e8981: f64 = (var_inv_lnfin * p.p1035);
        let assign6090_e8982: f64 = (assign6090_e8978 + assign6090_e8981);
        let assign6090_e8985: f64 = (var_inv_w * p.p1036);
        let assign6090_e8986: f64 = (assign6090_e8982 + assign6090_e8985);
        let assign6090_e8989: f64 = (var_inv_wl * p.p1037);
        let assign6090_e8990: f64 = (assign6090_e8986 + assign6090_e8989);
        (assign6090_e8990, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pclmr_i, var_pclmr_i_dn0, var_pclmr_i_dn2, var_pclmr_i_dn3, var_pclmr_i_dn4, var_pclmr_i_dn5, var_pclmr_i_dn6, var_pclmr_i_dn7, var_pclmr_i_dn8, var_pclmr_i_dn9, var_pclmr_i_dn10, var_pclmr_i_dn11, var_pclmr_i_dn13, var_pclmr_i_dn14,)
    }
};
        var_pclmr_i = assign6090_e8992;
        var_pclmr_i_dn0 = assign6090_e8992_d_n0;
        var_pclmr_i_dn2 = assign6090_e8992_d_n2;
        var_pclmr_i_dn3 = assign6090_e8992_d_n3;
        var_pclmr_i_dn4 = assign6090_e8992_d_n4;
        var_pclmr_i_dn5 = assign6090_e8992_d_n5;
        var_pclmr_i_dn6 = assign6090_e8992_d_n6;
        var_pclmr_i_dn7 = assign6090_e8992_d_n7;
        var_pclmr_i_dn8 = assign6090_e8992_d_n8;
        var_pclmr_i_dn9 = assign6090_e8992_d_n9;
        var_pclmr_i_dn10 = assign6090_e8992_d_n10;
        var_pclmr_i_dn11 = assign6090_e8992_d_n11;
        var_pclmr_i_dn13 = assign6090_e8992_d_n13;
        var_pclmr_i_dn14 = assign6090_e8992_d_n14;

        let (assign6100_e9016,) = {
    if (var_guard45 != 0.0) {
        let assign6100_e8997: f64 = (var_inv_l * p.p291);
        let assign6100_e8998: f64 = (p.p290 + assign6100_e8997);
        let assign6100_e9001: f64 = (var_inv_nfin * p.p292);
        let assign6100_e9002: f64 = (assign6100_e8998 + assign6100_e9001);
        let assign6100_e9005: f64 = (var_inv_lnfin * p.p293);
        let assign6100_e9006: f64 = (assign6100_e9002 + assign6100_e9005);
        let assign6100_e9009: f64 = (var_inv_w * p.p294);
        let assign6100_e9010: f64 = (assign6100_e9006 + assign6100_e9009);
        let assign6100_e9013: f64 = (var_inv_wl * p.p295);
        let assign6100_e9014: f64 = (assign6100_e9010 + assign6100_e9013);
        (assign6100_e9014,)
    } else {
        (var_dvtshiftr_i,)
    }
};
        var_dvtshiftr_i = assign6100_e9016;

        let (assign6110_e9040,) = {
    if (var_guard45 != 0.0) {
        let assign6110_e9021: f64 = (var_inv_l * p.p462);
        let assign6110_e9022: f64 = (p.p461 + assign6110_e9021);
        let assign6110_e9025: f64 = (var_inv_nfin * p.p463);
        let assign6110_e9026: f64 = (assign6110_e9022 + assign6110_e9025);
        let assign6110_e9029: f64 = (var_inv_lnfin * p.p464);
        let assign6110_e9030: f64 = (assign6110_e9026 + assign6110_e9029);
        let assign6110_e9033: f64 = (var_inv_w * p.p465);
        let assign6110_e9034: f64 = (assign6110_e9030 + assign6110_e9033);
        let assign6110_e9037: f64 = (var_inv_wl * p.p466);
        let assign6110_e9038: f64 = (assign6110_e9034 + assign6110_e9037);
        (assign6110_e9038,)
    } else {
        (var_vsatr_i,)
    }
};
        var_vsatr_i = assign6110_e9040;

        *var_agidlb_i_slot = var_agidlb_i;
        *var_agislb_i_slot = var_agislb_i;
        *var_bgidlb_i_slot = var_bgidlb_i;
        *var_bgislb_i_slot = var_bgislb_i;
        *var_cdscdr_i_slot = var_cdscdr_i;
        *var_cgidlb_i_slot = var_cgidlb_i;
        *var_cgislb_i_slot = var_cgislb_i;
        *var_citr_i_slot = var_citr_i;
        *var_dvtshiftr_i_slot = var_dvtshiftr_i;
        *var_egidlb_i_slot = var_egidlb_i;
        *var_egislb_i_slot = var_egislb_i;
        *var_eta0r_i_slot = var_eta0r_i;
        *var_guard44_slot = var_guard44;
        *var_guard45_slot = var_guard45;
        *var_k2sat1_i_slot = var_k2sat1_i;
        *var_k2sat_i_slot = var_k2sat_i;
        *var_mexpr_i_slot = var_mexpr_i;
        *var_mexpr_i_dn0_slot = var_mexpr_i_dn0;
        *var_mexpr_i_dn10_slot = var_mexpr_i_dn10;
        *var_mexpr_i_dn11_slot = var_mexpr_i_dn11;
        *var_mexpr_i_dn13_slot = var_mexpr_i_dn13;
        *var_mexpr_i_dn14_slot = var_mexpr_i_dn14;
        *var_mexpr_i_dn2_slot = var_mexpr_i_dn2;
        *var_mexpr_i_dn3_slot = var_mexpr_i_dn3;
        *var_mexpr_i_dn4_slot = var_mexpr_i_dn4;
        *var_mexpr_i_dn5_slot = var_mexpr_i_dn5;
        *var_mexpr_i_dn6_slot = var_mexpr_i_dn6;
        *var_mexpr_i_dn7_slot = var_mexpr_i_dn7;
        *var_mexpr_i_dn8_slot = var_mexpr_i_dn8;
        *var_mexpr_i_dn9_slot = var_mexpr_i_dn9;
        *var_pclmr_i_slot = var_pclmr_i;
        *var_pclmr_i_dn0_slot = var_pclmr_i_dn0;
        *var_pclmr_i_dn10_slot = var_pclmr_i_dn10;
        *var_pclmr_i_dn11_slot = var_pclmr_i_dn11;
        *var_pclmr_i_dn13_slot = var_pclmr_i_dn13;
        *var_pclmr_i_dn14_slot = var_pclmr_i_dn14;
        *var_pclmr_i_dn2_slot = var_pclmr_i_dn2;
        *var_pclmr_i_dn3_slot = var_pclmr_i_dn3;
        *var_pclmr_i_dn4_slot = var_pclmr_i_dn4;
        *var_pclmr_i_dn5_slot = var_pclmr_i_dn5;
        *var_pclmr_i_dn6_slot = var_pclmr_i_dn6;
        *var_pclmr_i_dn7_slot = var_pclmr_i_dn7;
        *var_pclmr_i_dn8_slot = var_pclmr_i_dn8;
        *var_pclmr_i_dn9_slot = var_pclmr_i_dn9;
        *var_pdibl1r_i_slot = var_pdibl1r_i;
        *var_pdibl2r_i_slot = var_pdibl2r_i;
        *var_pgidlb_i_slot = var_pgidlb_i;
        *var_pgislb_i_slot = var_pgislb_i;
        *var_ptwgr_i_slot = var_ptwgr_i;
        *var_ptwgr_i_dn0_slot = var_ptwgr_i_dn0;
        *var_ptwgr_i_dn10_slot = var_ptwgr_i_dn10;
        *var_ptwgr_i_dn11_slot = var_ptwgr_i_dn11;
        *var_ptwgr_i_dn13_slot = var_ptwgr_i_dn13;
        *var_ptwgr_i_dn14_slot = var_ptwgr_i_dn14;
        *var_ptwgr_i_dn2_slot = var_ptwgr_i_dn2;
        *var_ptwgr_i_dn3_slot = var_ptwgr_i_dn3;
        *var_ptwgr_i_dn4_slot = var_ptwgr_i_dn4;
        *var_ptwgr_i_dn5_slot = var_ptwgr_i_dn5;
        *var_ptwgr_i_dn6_slot = var_ptwgr_i_dn6;
        *var_ptwgr_i_dn7_slot = var_ptwgr_i_dn7;
        *var_ptwgr_i_dn8_slot = var_ptwgr_i_dn8;
        *var_ptwgr_i_dn9_slot = var_ptwgr_i_dn9;
        *var_vsat1r_i_slot = var_vsat1r_i;
        *var_vsat1r_i_dn0_slot = var_vsat1r_i_dn0;
        *var_vsat1r_i_dn10_slot = var_vsat1r_i_dn10;
        *var_vsat1r_i_dn11_slot = var_vsat1r_i_dn11;
        *var_vsat1r_i_dn13_slot = var_vsat1r_i_dn13;
        *var_vsat1r_i_dn14_slot = var_vsat1r_i_dn14;
        *var_vsat1r_i_dn2_slot = var_vsat1r_i_dn2;
        *var_vsat1r_i_dn3_slot = var_vsat1r_i_dn3;
        *var_vsat1r_i_dn4_slot = var_vsat1r_i_dn4;
        *var_vsat1r_i_dn5_slot = var_vsat1r_i_dn5;
        *var_vsat1r_i_dn6_slot = var_vsat1r_i_dn6;
        *var_vsat1r_i_dn7_slot = var_vsat1r_i_dn7;
        *var_vsat1r_i_dn8_slot = var_vsat1r_i_dn8;
        *var_vsat1r_i_dn9_slot = var_vsat1r_i_dn9;
        *var_vsatr_i_slot = var_vsatr_i;
    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        var_guard45: f64,
        var_inv_l: f64,
        var_inv_lnfin: f64,
        var_inv_nfin: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_atr_i_slot: &mut f64,
        var_eur_i_slot: &mut f64,
        var_eur_i_dn0_slot: &mut f64,
        var_eur_i_dn10_slot: &mut f64,
        var_eur_i_dn11_slot: &mut f64,
        var_eur_i_dn13_slot: &mut f64,
        var_eur_i_dn14_slot: &mut f64,
        var_eur_i_dn2_slot: &mut f64,
        var_eur_i_dn3_slot: &mut f64,
        var_eur_i_dn4_slot: &mut f64,
        var_eur_i_dn5_slot: &mut f64,
        var_eur_i_dn6_slot: &mut f64,
        var_eur_i_dn7_slot: &mut f64,
        var_eur_i_dn8_slot: &mut f64,
        var_eur_i_dn9_slot: &mut f64,
        var_guard46_slot: &mut f64,
        var_guard47_slot: &mut f64,
        var_guard48_slot: &mut f64,
        var_ksativr_i_slot: &mut f64,
        var_u0cv_i_slot: &mut f64,
        var_u0cv_i_dn0_slot: &mut f64,
        var_u0cv_i_dn10_slot: &mut f64,
        var_u0cv_i_dn11_slot: &mut f64,
        var_u0cv_i_dn13_slot: &mut f64,
        var_u0cv_i_dn14_slot: &mut f64,
        var_u0cv_i_dn2_slot: &mut f64,
        var_u0cv_i_dn3_slot: &mut f64,
        var_u0cv_i_dn4_slot: &mut f64,
        var_u0cv_i_dn5_slot: &mut f64,
        var_u0cv_i_dn6_slot: &mut f64,
        var_u0cv_i_dn7_slot: &mut f64,
        var_u0cv_i_dn8_slot: &mut f64,
        var_u0cv_i_dn9_slot: &mut f64,
        var_u0r_i_slot: &mut f64,
        var_u0r_i_dn0_slot: &mut f64,
        var_u0r_i_dn10_slot: &mut f64,
        var_u0r_i_dn11_slot: &mut f64,
        var_u0r_i_dn13_slot: &mut f64,
        var_u0r_i_dn14_slot: &mut f64,
        var_u0r_i_dn2_slot: &mut f64,
        var_u0r_i_dn3_slot: &mut f64,
        var_u0r_i_dn4_slot: &mut f64,
        var_u0r_i_dn5_slot: &mut f64,
        var_u0r_i_dn6_slot: &mut f64,
        var_u0r_i_dn7_slot: &mut f64,
        var_u0r_i_dn8_slot: &mut f64,
        var_u0r_i_dn9_slot: &mut f64,
        var_ua1cv_i_slot: &mut f64,
        var_ua1r_i_slot: &mut f64,
        var_uacv_i_slot: &mut f64,
        var_uar_i_slot: &mut f64,
        var_uar_i_dn0_slot: &mut f64,
        var_uar_i_dn10_slot: &mut f64,
        var_uar_i_dn11_slot: &mut f64,
        var_uar_i_dn13_slot: &mut f64,
        var_uar_i_dn14_slot: &mut f64,
        var_uar_i_dn2_slot: &mut f64,
        var_uar_i_dn3_slot: &mut f64,
        var_uar_i_dn4_slot: &mut f64,
        var_uar_i_dn5_slot: &mut f64,
        var_uar_i_dn6_slot: &mut f64,
        var_uar_i_dn7_slot: &mut f64,
        var_uar_i_dn8_slot: &mut f64,
        var_uar_i_dn9_slot: &mut f64,
        var_uc1r_i_slot: &mut f64,
        var_ucr_i_slot: &mut f64,
        var_ud1r_i_slot: &mut f64,
        var_udcv_i_slot: &mut f64,
        var_udr_i_slot: &mut f64,
        var_udr_i_dn0_slot: &mut f64,
        var_udr_i_dn10_slot: &mut f64,
        var_udr_i_dn11_slot: &mut f64,
        var_udr_i_dn13_slot: &mut f64,
        var_udr_i_dn14_slot: &mut f64,
        var_udr_i_dn2_slot: &mut f64,
        var_udr_i_dn3_slot: &mut f64,
        var_udr_i_dn4_slot: &mut f64,
        var_udr_i_dn5_slot: &mut f64,
        var_udr_i_dn6_slot: &mut f64,
        var_udr_i_dn7_slot: &mut f64,
        var_udr_i_dn8_slot: &mut f64,
        var_udr_i_dn9_slot: &mut f64,
        var_upr_i_slot: &mut f64,
        var_ute1cv_i_slot: &mut f64,
        var_utecv_i_slot: &mut f64,
        var_uter_i_slot: &mut f64,
        var_utlcv_i_slot: &mut f64,
        var_utlr_i_slot: &mut f64,
    ) {
        let mut var_atr_i: f64 = *var_atr_i_slot;
        let mut var_eur_i: f64 = *var_eur_i_slot;
        let mut var_eur_i_dn0: f64 = *var_eur_i_dn0_slot;
        let mut var_eur_i_dn10: f64 = *var_eur_i_dn10_slot;
        let mut var_eur_i_dn11: f64 = *var_eur_i_dn11_slot;
        let mut var_eur_i_dn13: f64 = *var_eur_i_dn13_slot;
        let mut var_eur_i_dn14: f64 = *var_eur_i_dn14_slot;
        let mut var_eur_i_dn2: f64 = *var_eur_i_dn2_slot;
        let mut var_eur_i_dn3: f64 = *var_eur_i_dn3_slot;
        let mut var_eur_i_dn4: f64 = *var_eur_i_dn4_slot;
        let mut var_eur_i_dn5: f64 = *var_eur_i_dn5_slot;
        let mut var_eur_i_dn6: f64 = *var_eur_i_dn6_slot;
        let mut var_eur_i_dn7: f64 = *var_eur_i_dn7_slot;
        let mut var_eur_i_dn8: f64 = *var_eur_i_dn8_slot;
        let mut var_eur_i_dn9: f64 = *var_eur_i_dn9_slot;
        let mut var_guard46: f64 = *var_guard46_slot;
        let mut var_guard47: f64 = *var_guard47_slot;
        let mut var_guard48: f64 = *var_guard48_slot;
        let mut var_ksativr_i: f64 = *var_ksativr_i_slot;
        let mut var_u0cv_i: f64 = *var_u0cv_i_slot;
        let mut var_u0cv_i_dn0: f64 = *var_u0cv_i_dn0_slot;
        let mut var_u0cv_i_dn10: f64 = *var_u0cv_i_dn10_slot;
        let mut var_u0cv_i_dn11: f64 = *var_u0cv_i_dn11_slot;
        let mut var_u0cv_i_dn13: f64 = *var_u0cv_i_dn13_slot;
        let mut var_u0cv_i_dn14: f64 = *var_u0cv_i_dn14_slot;
        let mut var_u0cv_i_dn2: f64 = *var_u0cv_i_dn2_slot;
        let mut var_u0cv_i_dn3: f64 = *var_u0cv_i_dn3_slot;
        let mut var_u0cv_i_dn4: f64 = *var_u0cv_i_dn4_slot;
        let mut var_u0cv_i_dn5: f64 = *var_u0cv_i_dn5_slot;
        let mut var_u0cv_i_dn6: f64 = *var_u0cv_i_dn6_slot;
        let mut var_u0cv_i_dn7: f64 = *var_u0cv_i_dn7_slot;
        let mut var_u0cv_i_dn8: f64 = *var_u0cv_i_dn8_slot;
        let mut var_u0cv_i_dn9: f64 = *var_u0cv_i_dn9_slot;
        let mut var_u0r_i: f64 = *var_u0r_i_slot;
        let mut var_u0r_i_dn0: f64 = *var_u0r_i_dn0_slot;
        let mut var_u0r_i_dn10: f64 = *var_u0r_i_dn10_slot;
        let mut var_u0r_i_dn11: f64 = *var_u0r_i_dn11_slot;
        let mut var_u0r_i_dn13: f64 = *var_u0r_i_dn13_slot;
        let mut var_u0r_i_dn14: f64 = *var_u0r_i_dn14_slot;
        let mut var_u0r_i_dn2: f64 = *var_u0r_i_dn2_slot;
        let mut var_u0r_i_dn3: f64 = *var_u0r_i_dn3_slot;
        let mut var_u0r_i_dn4: f64 = *var_u0r_i_dn4_slot;
        let mut var_u0r_i_dn5: f64 = *var_u0r_i_dn5_slot;
        let mut var_u0r_i_dn6: f64 = *var_u0r_i_dn6_slot;
        let mut var_u0r_i_dn7: f64 = *var_u0r_i_dn7_slot;
        let mut var_u0r_i_dn8: f64 = *var_u0r_i_dn8_slot;
        let mut var_u0r_i_dn9: f64 = *var_u0r_i_dn9_slot;
        let mut var_ua1cv_i: f64 = *var_ua1cv_i_slot;
        let mut var_ua1r_i: f64 = *var_ua1r_i_slot;
        let mut var_uacv_i: f64 = *var_uacv_i_slot;
        let mut var_uar_i: f64 = *var_uar_i_slot;
        let mut var_uar_i_dn0: f64 = *var_uar_i_dn0_slot;
        let mut var_uar_i_dn10: f64 = *var_uar_i_dn10_slot;
        let mut var_uar_i_dn11: f64 = *var_uar_i_dn11_slot;
        let mut var_uar_i_dn13: f64 = *var_uar_i_dn13_slot;
        let mut var_uar_i_dn14: f64 = *var_uar_i_dn14_slot;
        let mut var_uar_i_dn2: f64 = *var_uar_i_dn2_slot;
        let mut var_uar_i_dn3: f64 = *var_uar_i_dn3_slot;
        let mut var_uar_i_dn4: f64 = *var_uar_i_dn4_slot;
        let mut var_uar_i_dn5: f64 = *var_uar_i_dn5_slot;
        let mut var_uar_i_dn6: f64 = *var_uar_i_dn6_slot;
        let mut var_uar_i_dn7: f64 = *var_uar_i_dn7_slot;
        let mut var_uar_i_dn8: f64 = *var_uar_i_dn8_slot;
        let mut var_uar_i_dn9: f64 = *var_uar_i_dn9_slot;
        let mut var_uc1r_i: f64 = *var_uc1r_i_slot;
        let mut var_ucr_i: f64 = *var_ucr_i_slot;
        let mut var_ud1r_i: f64 = *var_ud1r_i_slot;
        let mut var_udcv_i: f64 = *var_udcv_i_slot;
        let mut var_udr_i: f64 = *var_udr_i_slot;
        let mut var_udr_i_dn0: f64 = *var_udr_i_dn0_slot;
        let mut var_udr_i_dn10: f64 = *var_udr_i_dn10_slot;
        let mut var_udr_i_dn11: f64 = *var_udr_i_dn11_slot;
        let mut var_udr_i_dn13: f64 = *var_udr_i_dn13_slot;
        let mut var_udr_i_dn14: f64 = *var_udr_i_dn14_slot;
        let mut var_udr_i_dn2: f64 = *var_udr_i_dn2_slot;
        let mut var_udr_i_dn3: f64 = *var_udr_i_dn3_slot;
        let mut var_udr_i_dn4: f64 = *var_udr_i_dn4_slot;
        let mut var_udr_i_dn5: f64 = *var_udr_i_dn5_slot;
        let mut var_udr_i_dn6: f64 = *var_udr_i_dn6_slot;
        let mut var_udr_i_dn7: f64 = *var_udr_i_dn7_slot;
        let mut var_udr_i_dn8: f64 = *var_udr_i_dn8_slot;
        let mut var_udr_i_dn9: f64 = *var_udr_i_dn9_slot;
        let mut var_upr_i: f64 = *var_upr_i_slot;
        let mut var_ute1cv_i: f64 = *var_ute1cv_i_slot;
        let mut var_utecv_i: f64 = *var_utecv_i_slot;
        let mut var_uter_i: f64 = *var_uter_i_slot;
        let mut var_utlcv_i: f64 = *var_utlcv_i_slot;
        let mut var_utlr_i: f64 = *var_utlr_i_slot;

        let (assign6120_e9064,) = {
    if (var_guard45 != 0.0) {
        let assign6120_e9045: f64 = (var_inv_l * p.p501);
        let assign6120_e9046: f64 = (p.p500 + assign6120_e9045);
        let assign6120_e9049: f64 = (var_inv_nfin * p.p502);
        let assign6120_e9050: f64 = (assign6120_e9046 + assign6120_e9049);
        let assign6120_e9053: f64 = (var_inv_lnfin * p.p503);
        let assign6120_e9054: f64 = (assign6120_e9050 + assign6120_e9053);
        let assign6120_e9057: f64 = (var_inv_w * p.p504);
        let assign6120_e9058: f64 = (assign6120_e9054 + assign6120_e9057);
        let assign6120_e9061: f64 = (var_inv_wl * p.p505);
        let assign6120_e9062: f64 = (assign6120_e9058 + assign6120_e9061);
        (assign6120_e9062,)
    } else {
        (var_ksativr_i,)
    }
};
        var_ksativr_i = assign6120_e9064;

        let (assign6130_e9088, assign6130_e9088_d_n0, assign6130_e9088_d_n2, assign6130_e9088_d_n3, assign6130_e9088_d_n4, assign6130_e9088_d_n5, assign6130_e9088_d_n6, assign6130_e9088_d_n7, assign6130_e9088_d_n8, assign6130_e9088_d_n9, assign6130_e9088_d_n10, assign6130_e9088_d_n11, assign6130_e9088_d_n13, assign6130_e9088_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6130_e9069: f64 = (var_inv_l * p.p612);
        let assign6130_e9070: f64 = (p.p611 + assign6130_e9069);
        let assign6130_e9073: f64 = (var_inv_nfin * p.p613);
        let assign6130_e9074: f64 = (assign6130_e9070 + assign6130_e9073);
        let assign6130_e9077: f64 = (var_inv_lnfin * p.p614);
        let assign6130_e9078: f64 = (assign6130_e9074 + assign6130_e9077);
        let assign6130_e9081: f64 = (var_inv_w * p.p615);
        let assign6130_e9082: f64 = (assign6130_e9078 + assign6130_e9081);
        let assign6130_e9085: f64 = (var_inv_wl * p.p616);
        let assign6130_e9086: f64 = (assign6130_e9082 + assign6130_e9085);
        (assign6130_e9086, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_u0r_i, var_u0r_i_dn0, var_u0r_i_dn2, var_u0r_i_dn3, var_u0r_i_dn4, var_u0r_i_dn5, var_u0r_i_dn6, var_u0r_i_dn7, var_u0r_i_dn8, var_u0r_i_dn9, var_u0r_i_dn10, var_u0r_i_dn11, var_u0r_i_dn13, var_u0r_i_dn14,)
    }
};
        var_u0r_i = assign6130_e9088;
        var_u0r_i_dn0 = assign6130_e9088_d_n0;
        var_u0r_i_dn2 = assign6130_e9088_d_n2;
        var_u0r_i_dn3 = assign6130_e9088_d_n3;
        var_u0r_i_dn4 = assign6130_e9088_d_n4;
        var_u0r_i_dn5 = assign6130_e9088_d_n5;
        var_u0r_i_dn6 = assign6130_e9088_d_n6;
        var_u0r_i_dn7 = assign6130_e9088_d_n7;
        var_u0r_i_dn8 = assign6130_e9088_d_n8;
        var_u0r_i_dn9 = assign6130_e9088_d_n9;
        var_u0r_i_dn10 = assign6130_e9088_d_n10;
        var_u0r_i_dn11 = assign6130_e9088_d_n11;
        var_u0r_i_dn13 = assign6130_e9088_d_n13;
        var_u0r_i_dn14 = assign6130_e9088_d_n14;

        let (assign6140_e9112, assign6140_e9112_d_n0, assign6140_e9112_d_n2, assign6140_e9112_d_n3, assign6140_e9112_d_n4, assign6140_e9112_d_n5, assign6140_e9112_d_n6, assign6140_e9112_d_n7, assign6140_e9112_d_n8, assign6140_e9112_d_n9, assign6140_e9112_d_n10, assign6140_e9112_d_n11, assign6140_e9112_d_n13, assign6140_e9112_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6140_e9093: f64 = (var_inv_l * p.p648);
        let assign6140_e9094: f64 = (p.p647 + assign6140_e9093);
        let assign6140_e9097: f64 = (var_inv_nfin * p.p649);
        let assign6140_e9098: f64 = (assign6140_e9094 + assign6140_e9097);
        let assign6140_e9101: f64 = (var_inv_lnfin * p.p650);
        let assign6140_e9102: f64 = (assign6140_e9098 + assign6140_e9101);
        let assign6140_e9105: f64 = (var_inv_w * p.p651);
        let assign6140_e9106: f64 = (assign6140_e9102 + assign6140_e9105);
        let assign6140_e9109: f64 = (var_inv_wl * p.p652);
        let assign6140_e9110: f64 = (assign6140_e9106 + assign6140_e9109);
        (assign6140_e9110, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uar_i, var_uar_i_dn0, var_uar_i_dn2, var_uar_i_dn3, var_uar_i_dn4, var_uar_i_dn5, var_uar_i_dn6, var_uar_i_dn7, var_uar_i_dn8, var_uar_i_dn9, var_uar_i_dn10, var_uar_i_dn11, var_uar_i_dn13, var_uar_i_dn14,)
    }
};
        var_uar_i = assign6140_e9112;
        var_uar_i_dn0 = assign6140_e9112_d_n0;
        var_uar_i_dn2 = assign6140_e9112_d_n2;
        var_uar_i_dn3 = assign6140_e9112_d_n3;
        var_uar_i_dn4 = assign6140_e9112_d_n4;
        var_uar_i_dn5 = assign6140_e9112_d_n5;
        var_uar_i_dn6 = assign6140_e9112_d_n6;
        var_uar_i_dn7 = assign6140_e9112_d_n7;
        var_uar_i_dn8 = assign6140_e9112_d_n8;
        var_uar_i_dn9 = assign6140_e9112_d_n9;
        var_uar_i_dn10 = assign6140_e9112_d_n10;
        var_uar_i_dn11 = assign6140_e9112_d_n11;
        var_uar_i_dn13 = assign6140_e9112_d_n13;
        var_uar_i_dn14 = assign6140_e9112_d_n14;

        let (assign6150_e9136,) = {
    if (var_guard45 != 0.0) {
        let assign6150_e9117: f64 = (var_inv_l * p.p636);
        let assign6150_e9118: f64 = (p.p635 + assign6150_e9117);
        let assign6150_e9121: f64 = (var_inv_nfin * p.p637);
        let assign6150_e9122: f64 = (assign6150_e9118 + assign6150_e9121);
        let assign6150_e9125: f64 = (var_inv_lnfin * p.p638);
        let assign6150_e9126: f64 = (assign6150_e9122 + assign6150_e9125);
        let assign6150_e9129: f64 = (var_inv_w * p.p639);
        let assign6150_e9130: f64 = (assign6150_e9126 + assign6150_e9129);
        let assign6150_e9133: f64 = (var_inv_wl * p.p640);
        let assign6150_e9134: f64 = (assign6150_e9130 + assign6150_e9133);
        (assign6150_e9134,)
    } else {
        (var_upr_i,)
    }
};
        var_upr_i = assign6150_e9136;

        let (assign6160_e9160, assign6160_e9160_d_n0, assign6160_e9160_d_n2, assign6160_e9160_d_n3, assign6160_e9160_d_n4, assign6160_e9160_d_n5, assign6160_e9160_d_n6, assign6160_e9160_d_n7, assign6160_e9160_d_n8, assign6160_e9160_d_n9, assign6160_e9160_d_n10, assign6160_e9160_d_n11, assign6160_e9160_d_n13, assign6160_e9160_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6160_e9141: f64 = (var_inv_l * p.p684);
        let assign6160_e9142: f64 = (p.p683 + assign6160_e9141);
        let assign6160_e9145: f64 = (var_inv_nfin * p.p685);
        let assign6160_e9146: f64 = (assign6160_e9142 + assign6160_e9145);
        let assign6160_e9149: f64 = (var_inv_lnfin * p.p686);
        let assign6160_e9150: f64 = (assign6160_e9146 + assign6160_e9149);
        let assign6160_e9153: f64 = (var_inv_w * p.p687);
        let assign6160_e9154: f64 = (assign6160_e9150 + assign6160_e9153);
        let assign6160_e9157: f64 = (var_inv_wl * p.p688);
        let assign6160_e9158: f64 = (assign6160_e9154 + assign6160_e9157);
        (assign6160_e9158, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_eur_i, var_eur_i_dn0, var_eur_i_dn2, var_eur_i_dn3, var_eur_i_dn4, var_eur_i_dn5, var_eur_i_dn6, var_eur_i_dn7, var_eur_i_dn8, var_eur_i_dn9, var_eur_i_dn10, var_eur_i_dn11, var_eur_i_dn13, var_eur_i_dn14,)
    }
};
        var_eur_i = assign6160_e9160;
        var_eur_i_dn0 = assign6160_e9160_d_n0;
        var_eur_i_dn2 = assign6160_e9160_d_n2;
        var_eur_i_dn3 = assign6160_e9160_d_n3;
        var_eur_i_dn4 = assign6160_e9160_d_n4;
        var_eur_i_dn5 = assign6160_e9160_d_n5;
        var_eur_i_dn6 = assign6160_e9160_d_n6;
        var_eur_i_dn7 = assign6160_e9160_d_n7;
        var_eur_i_dn8 = assign6160_e9160_d_n8;
        var_eur_i_dn9 = assign6160_e9160_d_n9;
        var_eur_i_dn10 = assign6160_e9160_d_n10;
        var_eur_i_dn11 = assign6160_e9160_d_n11;
        var_eur_i_dn13 = assign6160_e9160_d_n13;
        var_eur_i_dn14 = assign6160_e9160_d_n14;

        let (assign6170_e9184, assign6170_e9184_d_n0, assign6170_e9184_d_n2, assign6170_e9184_d_n3, assign6170_e9184_d_n4, assign6170_e9184_d_n5, assign6170_e9184_d_n6, assign6170_e9184_d_n7, assign6170_e9184_d_n8, assign6170_e9184_d_n9, assign6170_e9184_d_n10, assign6170_e9184_d_n11, assign6170_e9184_d_n13, assign6170_e9184_d_n14,) = {
    if (var_guard45 != 0.0) {
        let assign6170_e9165: f64 = (var_inv_l * p.p696);
        let assign6170_e9166: f64 = (p.p695 + assign6170_e9165);
        let assign6170_e9169: f64 = (var_inv_nfin * p.p697);
        let assign6170_e9170: f64 = (assign6170_e9166 + assign6170_e9169);
        let assign6170_e9173: f64 = (var_inv_lnfin * p.p698);
        let assign6170_e9174: f64 = (assign6170_e9170 + assign6170_e9173);
        let assign6170_e9177: f64 = (var_inv_w * p.p699);
        let assign6170_e9178: f64 = (assign6170_e9174 + assign6170_e9177);
        let assign6170_e9181: f64 = (var_inv_wl * p.p700);
        let assign6170_e9182: f64 = (assign6170_e9178 + assign6170_e9181);
        (assign6170_e9182, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_udr_i, var_udr_i_dn0, var_udr_i_dn2, var_udr_i_dn3, var_udr_i_dn4, var_udr_i_dn5, var_udr_i_dn6, var_udr_i_dn7, var_udr_i_dn8, var_udr_i_dn9, var_udr_i_dn10, var_udr_i_dn11, var_udr_i_dn13, var_udr_i_dn14,)
    }
};
        var_udr_i = assign6170_e9184;
        var_udr_i_dn0 = assign6170_e9184_d_n0;
        var_udr_i_dn2 = assign6170_e9184_d_n2;
        var_udr_i_dn3 = assign6170_e9184_d_n3;
        var_udr_i_dn4 = assign6170_e9184_d_n4;
        var_udr_i_dn5 = assign6170_e9184_d_n5;
        var_udr_i_dn6 = assign6170_e9184_d_n6;
        var_udr_i_dn7 = assign6170_e9184_d_n7;
        var_udr_i_dn8 = assign6170_e9184_d_n8;
        var_udr_i_dn9 = assign6170_e9184_d_n9;
        var_udr_i_dn10 = assign6170_e9184_d_n10;
        var_udr_i_dn11 = assign6170_e9184_d_n11;
        var_udr_i_dn13 = assign6170_e9184_d_n13;
        var_udr_i_dn14 = assign6170_e9184_d_n14;

        let (assign6180_e9208,) = {
    if (var_guard45 != 0.0) {
        let assign6180_e9189: f64 = (var_inv_l * p.p744);
        let assign6180_e9190: f64 = (p.p743 + assign6180_e9189);
        let assign6180_e9193: f64 = (var_inv_nfin * p.p745);
        let assign6180_e9194: f64 = (assign6180_e9190 + assign6180_e9193);
        let assign6180_e9197: f64 = (var_inv_lnfin * p.p746);
        let assign6180_e9198: f64 = (assign6180_e9194 + assign6180_e9197);
        let assign6180_e9201: f64 = (var_inv_w * p.p747);
        let assign6180_e9202: f64 = (assign6180_e9198 + assign6180_e9201);
        let assign6180_e9205: f64 = (var_inv_wl * p.p748);
        let assign6180_e9206: f64 = (assign6180_e9202 + assign6180_e9205);
        (assign6180_e9206,)
    } else {
        (var_uter_i,)
    }
};
        var_uter_i = assign6180_e9208;

        let (assign6190_e9232,) = {
    if (var_guard45 != 0.0) {
        let assign6190_e9213: f64 = (var_inv_l * p.p774);
        let assign6190_e9214: f64 = (p.p773 + assign6190_e9213);
        let assign6190_e9217: f64 = (var_inv_nfin * p.p775);
        let assign6190_e9218: f64 = (assign6190_e9214 + assign6190_e9217);
        let assign6190_e9221: f64 = (var_inv_lnfin * p.p776);
        let assign6190_e9222: f64 = (assign6190_e9218 + assign6190_e9221);
        let assign6190_e9225: f64 = (var_inv_w * p.p777);
        let assign6190_e9226: f64 = (assign6190_e9222 + assign6190_e9225);
        let assign6190_e9229: f64 = (var_inv_wl * p.p778);
        let assign6190_e9230: f64 = (assign6190_e9226 + assign6190_e9229);
        (assign6190_e9230,)
    } else {
        (var_utlr_i,)
    }
};
        var_utlr_i = assign6190_e9232;

        let (assign6200_e9256,) = {
    if (var_guard45 != 0.0) {
        let assign6200_e9237: f64 = (var_inv_l * p.p798);
        let assign6200_e9238: f64 = (p.p797 + assign6200_e9237);
        let assign6200_e9241: f64 = (var_inv_nfin * p.p799);
        let assign6200_e9242: f64 = (assign6200_e9238 + assign6200_e9241);
        let assign6200_e9245: f64 = (var_inv_lnfin * p.p800);
        let assign6200_e9246: f64 = (assign6200_e9242 + assign6200_e9245);
        let assign6200_e9249: f64 = (var_inv_w * p.p801);
        let assign6200_e9250: f64 = (assign6200_e9246 + assign6200_e9249);
        let assign6200_e9253: f64 = (var_inv_wl * p.p802);
        let assign6200_e9254: f64 = (assign6200_e9250 + assign6200_e9253);
        (assign6200_e9254,)
    } else {
        (var_ua1r_i,)
    }
};
        var_ua1r_i = assign6200_e9256;

        let (assign6210_e9280,) = {
    if (var_guard45 != 0.0) {
        let assign6210_e9261: f64 = (var_inv_l * p.p852);
        let assign6210_e9262: f64 = (p.p851 + assign6210_e9261);
        let assign6210_e9265: f64 = (var_inv_nfin * p.p853);
        let assign6210_e9266: f64 = (assign6210_e9262 + assign6210_e9265);
        let assign6210_e9269: f64 = (var_inv_lnfin * p.p854);
        let assign6210_e9270: f64 = (assign6210_e9266 + assign6210_e9269);
        let assign6210_e9273: f64 = (var_inv_w * p.p855);
        let assign6210_e9274: f64 = (assign6210_e9270 + assign6210_e9273);
        let assign6210_e9277: f64 = (var_inv_wl * p.p856);
        let assign6210_e9278: f64 = (assign6210_e9274 + assign6210_e9277);
        (assign6210_e9278,)
    } else {
        (var_ud1r_i,)
    }
};
        var_ud1r_i = assign6210_e9280;

        let (assign6220_e9304,) = {
    if (var_guard45 != 0.0) {
        let assign6220_e9285: f64 = (var_inv_l * p.p563);
        let assign6220_e9286: f64 = (p.p562 + assign6220_e9285);
        let assign6220_e9289: f64 = (var_inv_nfin * p.p564);
        let assign6220_e9290: f64 = (assign6220_e9286 + assign6220_e9289);
        let assign6220_e9293: f64 = (var_inv_lnfin * p.p565);
        let assign6220_e9294: f64 = (assign6220_e9290 + assign6220_e9293);
        let assign6220_e9297: f64 = (var_inv_w * p.p566);
        let assign6220_e9298: f64 = (assign6220_e9294 + assign6220_e9297);
        let assign6220_e9301: f64 = (var_inv_wl * p.p567);
        let assign6220_e9302: f64 = (assign6220_e9298 + assign6220_e9301);
        (assign6220_e9302,)
    } else {
        (var_atr_i,)
    }
};
        var_atr_i = assign6220_e9304;

        let assign6230_e9307: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard46 = assign6230_e9307;

        let (assign6240_e9333,) = {
    if ((var_guard45 != 0.0) && (var_guard46 != 0.0)) {
        let assign6240_e9314: f64 = (var_inv_l * p.p666);
        let assign6240_e9315: f64 = (p.p665 + assign6240_e9314);
        let assign6240_e9318: f64 = (var_inv_nfin * p.p667);
        let assign6240_e9319: f64 = (assign6240_e9315 + assign6240_e9318);
        let assign6240_e9322: f64 = (var_inv_lnfin * p.p668);
        let assign6240_e9323: f64 = (assign6240_e9319 + assign6240_e9322);
        let assign6240_e9326: f64 = (var_inv_w * p.p669);
        let assign6240_e9327: f64 = (assign6240_e9323 + assign6240_e9326);
        let assign6240_e9330: f64 = (var_inv_wl * p.p670);
        let assign6240_e9331: f64 = (assign6240_e9327 + assign6240_e9330);
        (assign6240_e9331,)
    } else {
        (var_ucr_i,)
    }
};
        var_ucr_i = assign6240_e9333;

        let (assign6250_e9359,) = {
    if ((var_guard45 != 0.0) && (var_guard46 != 0.0)) {
        let assign6250_e9340: f64 = (var_inv_l * p.p834);
        let assign6250_e9341: f64 = (p.p833 + assign6250_e9340);
        let assign6250_e9344: f64 = (var_inv_nfin * p.p835);
        let assign6250_e9345: f64 = (assign6250_e9341 + assign6250_e9344);
        let assign6250_e9348: f64 = (var_inv_lnfin * p.p836);
        let assign6250_e9349: f64 = (assign6250_e9345 + assign6250_e9348);
        let assign6250_e9352: f64 = (var_inv_w * p.p837);
        let assign6250_e9353: f64 = (assign6250_e9349 + assign6250_e9352);
        let assign6250_e9356: f64 = (var_inv_wl * p.p838);
        let assign6250_e9357: f64 = (assign6250_e9353 + assign6250_e9356);
        (assign6250_e9357,)
    } else {
        (var_uc1r_i,)
    }
};
        var_uc1r_i = assign6250_e9359;

        let assign6260_e9362: f64 = if p.p67 == 1.0 { 1.0 } else { 0.0 };
        var_guard47 = assign6260_e9362;

        let (assign6270_e9386, assign6270_e9386_d_n0, assign6270_e9386_d_n2, assign6270_e9386_d_n3, assign6270_e9386_d_n4, assign6270_e9386_d_n5, assign6270_e9386_d_n6, assign6270_e9386_d_n7, assign6270_e9386_d_n8, assign6270_e9386_d_n9, assign6270_e9386_d_n10, assign6270_e9386_d_n11, assign6270_e9386_d_n13, assign6270_e9386_d_n14,) = {
    if (var_guard47 != 0.0) {
        let assign6270_e9367: f64 = (var_inv_l * p.p618);
        let assign6270_e9368: f64 = (p.p617 + assign6270_e9367);
        let assign6270_e9371: f64 = (var_inv_nfin * p.p619);
        let assign6270_e9372: f64 = (assign6270_e9368 + assign6270_e9371);
        let assign6270_e9375: f64 = (var_inv_lnfin * p.p620);
        let assign6270_e9376: f64 = (assign6270_e9372 + assign6270_e9375);
        let assign6270_e9379: f64 = (var_inv_w * p.p621);
        let assign6270_e9380: f64 = (assign6270_e9376 + assign6270_e9379);
        let assign6270_e9383: f64 = (var_inv_wl * p.p622);
        let assign6270_e9384: f64 = (assign6270_e9380 + assign6270_e9383);
        (assign6270_e9384, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_u0cv_i, var_u0cv_i_dn0, var_u0cv_i_dn2, var_u0cv_i_dn3, var_u0cv_i_dn4, var_u0cv_i_dn5, var_u0cv_i_dn6, var_u0cv_i_dn7, var_u0cv_i_dn8, var_u0cv_i_dn9, var_u0cv_i_dn10, var_u0cv_i_dn11, var_u0cv_i_dn13, var_u0cv_i_dn14,)
    }
};
        var_u0cv_i = assign6270_e9386;
        var_u0cv_i_dn0 = assign6270_e9386_d_n0;
        var_u0cv_i_dn2 = assign6270_e9386_d_n2;
        var_u0cv_i_dn3 = assign6270_e9386_d_n3;
        var_u0cv_i_dn4 = assign6270_e9386_d_n4;
        var_u0cv_i_dn5 = assign6270_e9386_d_n5;
        var_u0cv_i_dn6 = assign6270_e9386_d_n6;
        var_u0cv_i_dn7 = assign6270_e9386_d_n7;
        var_u0cv_i_dn8 = assign6270_e9386_d_n8;
        var_u0cv_i_dn9 = assign6270_e9386_d_n9;
        var_u0cv_i_dn10 = assign6270_e9386_d_n10;
        var_u0cv_i_dn11 = assign6270_e9386_d_n11;
        var_u0cv_i_dn13 = assign6270_e9386_d_n13;
        var_u0cv_i_dn14 = assign6270_e9386_d_n14;

        let assign6280_e9389: f64 = if p.p582 != 0.0 { 1.0 } else { 0.0 };
        var_guard48 = assign6280_e9389;

        let (assign6290_e9428, assign6290_e9428_d_n0, assign6290_e9428_d_n2, assign6290_e9428_d_n3, assign6290_e9428_d_n4, assign6290_e9428_d_n5, assign6290_e9428_d_n6, assign6290_e9428_d_n7, assign6290_e9428_d_n8, assign6290_e9428_d_n9, assign6290_e9428_d_n10, assign6290_e9428_d_n11, assign6290_e9428_d_n13, assign6290_e9428_d_n14,) = {
    if ((var_guard47 != 0.0) && (var_guard48 != 0.0)) {
        let assign6290_e9397: f64 = (p.p582 / p.p5);
        let assign6290_e9401: f64 = (p.p5 / p.p585);
        let assign6290_e9402: f64 = (1.0 + assign6290_e9401);
        let (assign6290_e9423,) = {
            if (!(assign6290_e9402 > 1e-38)) {
                let assign6290_e9407: f64 = (-87.498233534);
                (assign6290_e9407,)
            } else {
                let assign6290_e9411: f64 = (p.p5 / p.p585);
                let assign6290_e9412: f64 = (1.0 + assign6290_e9411);
                let (assign6290_e9422,) = {
                    if (assign6290_e9412 > 1e-38) {
                        let assign6290_e9418: f64 = (p.p5 / p.p585);
                        let assign6290_e9419: f64 = (1.0 + assign6290_e9418);
                        let assign6290_e9420: f64 = (assign6290_e9419).ln();
                        (assign6290_e9420,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6290_e9422,)
            }
        };
        let assign6290_e9424: f64 = (assign6290_e9397 * assign6290_e9423);
        let assign6290_e9425: f64 = (1.0 + assign6290_e9424);
        let assign6290_e9426: f64 = (var_u0cv_i * assign6290_e9425);
        (assign6290_e9426, (var_u0cv_i_dn0 * assign6290_e9425), (var_u0cv_i_dn2 * assign6290_e9425), (var_u0cv_i_dn3 * assign6290_e9425), (var_u0cv_i_dn4 * assign6290_e9425), (var_u0cv_i_dn5 * assign6290_e9425), (var_u0cv_i_dn6 * assign6290_e9425), (var_u0cv_i_dn7 * assign6290_e9425), (var_u0cv_i_dn8 * assign6290_e9425), (var_u0cv_i_dn9 * assign6290_e9425), (var_u0cv_i_dn10 * assign6290_e9425), (var_u0cv_i_dn11 * assign6290_e9425), (var_u0cv_i_dn13 * assign6290_e9425), (var_u0cv_i_dn14 * assign6290_e9425),)
    } else {
        (var_u0cv_i, var_u0cv_i_dn0, var_u0cv_i_dn2, var_u0cv_i_dn3, var_u0cv_i_dn4, var_u0cv_i_dn5, var_u0cv_i_dn6, var_u0cv_i_dn7, var_u0cv_i_dn8, var_u0cv_i_dn9, var_u0cv_i_dn10, var_u0cv_i_dn11, var_u0cv_i_dn13, var_u0cv_i_dn14,)
    }
};
        var_u0cv_i = assign6290_e9428;
        var_u0cv_i_dn0 = assign6290_e9428_d_n0;
        var_u0cv_i_dn2 = assign6290_e9428_d_n2;
        var_u0cv_i_dn3 = assign6290_e9428_d_n3;
        var_u0cv_i_dn4 = assign6290_e9428_d_n4;
        var_u0cv_i_dn5 = assign6290_e9428_d_n5;
        var_u0cv_i_dn6 = assign6290_e9428_d_n6;
        var_u0cv_i_dn7 = assign6290_e9428_d_n7;
        var_u0cv_i_dn8 = assign6290_e9428_d_n8;
        var_u0cv_i_dn9 = assign6290_e9428_d_n9;
        var_u0cv_i_dn10 = assign6290_e9428_d_n10;
        var_u0cv_i_dn11 = assign6290_e9428_d_n11;
        var_u0cv_i_dn13 = assign6290_e9428_d_n13;
        var_u0cv_i_dn14 = assign6290_e9428_d_n14;

        let (assign6300_e9452,) = {
    if (var_guard47 != 0.0) {
        let assign6300_e9433: f64 = (var_inv_l * p.p654);
        let assign6300_e9434: f64 = (p.p653 + assign6300_e9433);
        let assign6300_e9437: f64 = (var_inv_nfin * p.p655);
        let assign6300_e9438: f64 = (assign6300_e9434 + assign6300_e9437);
        let assign6300_e9441: f64 = (var_inv_lnfin * p.p656);
        let assign6300_e9442: f64 = (assign6300_e9438 + assign6300_e9441);
        let assign6300_e9445: f64 = (var_inv_w * p.p657);
        let assign6300_e9446: f64 = (assign6300_e9442 + assign6300_e9445);
        let assign6300_e9449: f64 = (var_inv_wl * p.p658);
        let assign6300_e9450: f64 = (assign6300_e9446 + assign6300_e9449);
        (assign6300_e9450,)
    } else {
        (var_uacv_i,)
    }
};
        var_uacv_i = assign6300_e9452;

        let (assign6310_e9476,) = {
    if (var_guard47 != 0.0) {
        let assign6310_e9457: f64 = (var_inv_l * p.p702);
        let assign6310_e9458: f64 = (p.p701 + assign6310_e9457);
        let assign6310_e9461: f64 = (var_inv_nfin * p.p703);
        let assign6310_e9462: f64 = (assign6310_e9458 + assign6310_e9461);
        let assign6310_e9465: f64 = (var_inv_lnfin * p.p704);
        let assign6310_e9466: f64 = (assign6310_e9462 + assign6310_e9465);
        let assign6310_e9469: f64 = (var_inv_w * p.p705);
        let assign6310_e9470: f64 = (assign6310_e9466 + assign6310_e9469);
        let assign6310_e9473: f64 = (var_inv_wl * p.p706);
        let assign6310_e9474: f64 = (assign6310_e9470 + assign6310_e9473);
        (assign6310_e9474,)
    } else {
        (var_udcv_i,)
    }
};
        var_udcv_i = assign6310_e9476;

        let (assign6320_e9500,) = {
    if (var_guard47 != 0.0) {
        let assign6320_e9481: f64 = (var_inv_l * p.p750);
        let assign6320_e9482: f64 = (p.p749 + assign6320_e9481);
        let assign6320_e9485: f64 = (var_inv_nfin * p.p751);
        let assign6320_e9486: f64 = (assign6320_e9482 + assign6320_e9485);
        let assign6320_e9489: f64 = (var_inv_lnfin * p.p752);
        let assign6320_e9490: f64 = (assign6320_e9486 + assign6320_e9489);
        let assign6320_e9493: f64 = (var_inv_w * p.p753);
        let assign6320_e9494: f64 = (assign6320_e9490 + assign6320_e9493);
        let assign6320_e9497: f64 = (var_inv_wl * p.p754);
        let assign6320_e9498: f64 = (assign6320_e9494 + assign6320_e9497);
        (assign6320_e9498,)
    } else {
        (var_utecv_i,)
    }
};
        var_utecv_i = assign6320_e9500;

        let (assign6330_e9524,) = {
    if (var_guard47 != 0.0) {
        let assign6330_e9505: f64 = (var_inv_l * p.p762);
        let assign6330_e9506: f64 = (p.p761 + assign6330_e9505);
        let assign6330_e9509: f64 = (var_inv_nfin * p.p763);
        let assign6330_e9510: f64 = (assign6330_e9506 + assign6330_e9509);
        let assign6330_e9513: f64 = (var_inv_lnfin * p.p764);
        let assign6330_e9514: f64 = (assign6330_e9510 + assign6330_e9513);
        let assign6330_e9517: f64 = (var_inv_w * p.p765);
        let assign6330_e9518: f64 = (assign6330_e9514 + assign6330_e9517);
        let assign6330_e9521: f64 = (var_inv_wl * p.p766);
        let assign6330_e9522: f64 = (assign6330_e9518 + assign6330_e9521);
        (assign6330_e9522,)
    } else {
        (var_ute1cv_i,)
    }
};
        var_ute1cv_i = assign6330_e9524;

        let (assign6340_e9548,) = {
    if (var_guard47 != 0.0) {
        let assign6340_e9529: f64 = (var_inv_l * p.p780);
        let assign6340_e9530: f64 = (p.p779 + assign6340_e9529);
        let assign6340_e9533: f64 = (var_inv_nfin * p.p781);
        let assign6340_e9534: f64 = (assign6340_e9530 + assign6340_e9533);
        let assign6340_e9537: f64 = (var_inv_lnfin * p.p782);
        let assign6340_e9538: f64 = (assign6340_e9534 + assign6340_e9537);
        let assign6340_e9541: f64 = (var_inv_w * p.p783);
        let assign6340_e9542: f64 = (assign6340_e9538 + assign6340_e9541);
        let assign6340_e9545: f64 = (var_inv_wl * p.p784);
        let assign6340_e9546: f64 = (assign6340_e9542 + assign6340_e9545);
        (assign6340_e9546,)
    } else {
        (var_utlcv_i,)
    }
};
        var_utlcv_i = assign6340_e9548;

        let (assign6350_e9572,) = {
    if (var_guard47 != 0.0) {
        let assign6350_e9553: f64 = (var_inv_l * p.p804);
        let assign6350_e9554: f64 = (p.p803 + assign6350_e9553);
        let assign6350_e9557: f64 = (var_inv_nfin * p.p805);
        let assign6350_e9558: f64 = (assign6350_e9554 + assign6350_e9557);
        let assign6350_e9561: f64 = (var_inv_lnfin * p.p806);
        let assign6350_e9562: f64 = (assign6350_e9558 + assign6350_e9561);
        let assign6350_e9565: f64 = (var_inv_w * p.p807);
        let assign6350_e9566: f64 = (assign6350_e9562 + assign6350_e9565);
        let assign6350_e9569: f64 = (var_inv_wl * p.p808);
        let assign6350_e9570: f64 = (assign6350_e9566 + assign6350_e9569);
        (assign6350_e9570,)
    } else {
        (var_ua1cv_i,)
    }
};
        var_ua1cv_i = assign6350_e9572;

        *var_atr_i_slot = var_atr_i;
        *var_eur_i_slot = var_eur_i;
        *var_eur_i_dn0_slot = var_eur_i_dn0;
        *var_eur_i_dn10_slot = var_eur_i_dn10;
        *var_eur_i_dn11_slot = var_eur_i_dn11;
        *var_eur_i_dn13_slot = var_eur_i_dn13;
        *var_eur_i_dn14_slot = var_eur_i_dn14;
        *var_eur_i_dn2_slot = var_eur_i_dn2;
        *var_eur_i_dn3_slot = var_eur_i_dn3;
        *var_eur_i_dn4_slot = var_eur_i_dn4;
        *var_eur_i_dn5_slot = var_eur_i_dn5;
        *var_eur_i_dn6_slot = var_eur_i_dn6;
        *var_eur_i_dn7_slot = var_eur_i_dn7;
        *var_eur_i_dn8_slot = var_eur_i_dn8;
        *var_eur_i_dn9_slot = var_eur_i_dn9;
        *var_guard46_slot = var_guard46;
        *var_guard47_slot = var_guard47;
        *var_guard48_slot = var_guard48;
        *var_ksativr_i_slot = var_ksativr_i;
        *var_u0cv_i_slot = var_u0cv_i;
        *var_u0cv_i_dn0_slot = var_u0cv_i_dn0;
        *var_u0cv_i_dn10_slot = var_u0cv_i_dn10;
        *var_u0cv_i_dn11_slot = var_u0cv_i_dn11;
        *var_u0cv_i_dn13_slot = var_u0cv_i_dn13;
        *var_u0cv_i_dn14_slot = var_u0cv_i_dn14;
        *var_u0cv_i_dn2_slot = var_u0cv_i_dn2;
        *var_u0cv_i_dn3_slot = var_u0cv_i_dn3;
        *var_u0cv_i_dn4_slot = var_u0cv_i_dn4;
        *var_u0cv_i_dn5_slot = var_u0cv_i_dn5;
        *var_u0cv_i_dn6_slot = var_u0cv_i_dn6;
        *var_u0cv_i_dn7_slot = var_u0cv_i_dn7;
        *var_u0cv_i_dn8_slot = var_u0cv_i_dn8;
        *var_u0cv_i_dn9_slot = var_u0cv_i_dn9;
        *var_u0r_i_slot = var_u0r_i;
        *var_u0r_i_dn0_slot = var_u0r_i_dn0;
        *var_u0r_i_dn10_slot = var_u0r_i_dn10;
        *var_u0r_i_dn11_slot = var_u0r_i_dn11;
        *var_u0r_i_dn13_slot = var_u0r_i_dn13;
        *var_u0r_i_dn14_slot = var_u0r_i_dn14;
        *var_u0r_i_dn2_slot = var_u0r_i_dn2;
        *var_u0r_i_dn3_slot = var_u0r_i_dn3;
        *var_u0r_i_dn4_slot = var_u0r_i_dn4;
        *var_u0r_i_dn5_slot = var_u0r_i_dn5;
        *var_u0r_i_dn6_slot = var_u0r_i_dn6;
        *var_u0r_i_dn7_slot = var_u0r_i_dn7;
        *var_u0r_i_dn8_slot = var_u0r_i_dn8;
        *var_u0r_i_dn9_slot = var_u0r_i_dn9;
        *var_ua1cv_i_slot = var_ua1cv_i;
        *var_ua1r_i_slot = var_ua1r_i;
        *var_uacv_i_slot = var_uacv_i;
        *var_uar_i_slot = var_uar_i;
        *var_uar_i_dn0_slot = var_uar_i_dn0;
        *var_uar_i_dn10_slot = var_uar_i_dn10;
        *var_uar_i_dn11_slot = var_uar_i_dn11;
        *var_uar_i_dn13_slot = var_uar_i_dn13;
        *var_uar_i_dn14_slot = var_uar_i_dn14;
        *var_uar_i_dn2_slot = var_uar_i_dn2;
        *var_uar_i_dn3_slot = var_uar_i_dn3;
        *var_uar_i_dn4_slot = var_uar_i_dn4;
        *var_uar_i_dn5_slot = var_uar_i_dn5;
        *var_uar_i_dn6_slot = var_uar_i_dn6;
        *var_uar_i_dn7_slot = var_uar_i_dn7;
        *var_uar_i_dn8_slot = var_uar_i_dn8;
        *var_uar_i_dn9_slot = var_uar_i_dn9;
        *var_uc1r_i_slot = var_uc1r_i;
        *var_ucr_i_slot = var_ucr_i;
        *var_ud1r_i_slot = var_ud1r_i;
        *var_udcv_i_slot = var_udcv_i;
        *var_udr_i_slot = var_udr_i;
        *var_udr_i_dn0_slot = var_udr_i_dn0;
        *var_udr_i_dn10_slot = var_udr_i_dn10;
        *var_udr_i_dn11_slot = var_udr_i_dn11;
        *var_udr_i_dn13_slot = var_udr_i_dn13;
        *var_udr_i_dn14_slot = var_udr_i_dn14;
        *var_udr_i_dn2_slot = var_udr_i_dn2;
        *var_udr_i_dn3_slot = var_udr_i_dn3;
        *var_udr_i_dn4_slot = var_udr_i_dn4;
        *var_udr_i_dn5_slot = var_udr_i_dn5;
        *var_udr_i_dn6_slot = var_udr_i_dn6;
        *var_udr_i_dn7_slot = var_udr_i_dn7;
        *var_udr_i_dn8_slot = var_udr_i_dn8;
        *var_udr_i_dn9_slot = var_udr_i_dn9;
        *var_upr_i_slot = var_upr_i;
        *var_ute1cv_i_slot = var_ute1cv_i;
        *var_utecv_i_slot = var_utecv_i;
        *var_uter_i_slot = var_uter_i;
        *var_utlcv_i_slot = var_utlcv_i;
        *var_utlr_i_slot = var_utlr_i;
    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        var_guard47: f64,
        var_inv_l: f64,
        var_inv_lnfin: f64,
        var_inv_nfin: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_leff_1: f64,
        var_leff_1_dn0: f64,
        var_leff_1_dn10: f64,
        var_leff_1_dn11: f64,
        var_leff_1_dn13: f64,
        var_leff_1_dn14: f64,
        var_leff_1_dn2: f64,
        var_leff_1_dn3: f64,
        var_leff_1_dn4: f64,
        var_leff_1_dn5: f64,
        var_leff_1_dn6: f64,
        var_leff_1_dn7: f64,
        var_leff_1_dn8: f64,
        var_leff_1_dn9: f64,
        var_dimension1_i_slot: &mut f64,
        var_dimension2_i_slot: &mut f64,
        var_dimension3_i_slot: &mut f64,
        var_e2nom_i_slot: &mut f64,
        var_e3nom_i_slot: &mut f64,
        var_eta0cv_i_slot: &mut f64,
        var_eta0cv_i_dn0_slot: &mut f64,
        var_eta0cv_i_dn10_slot: &mut f64,
        var_eta0cv_i_dn11_slot: &mut f64,
        var_eta0cv_i_dn13_slot: &mut f64,
        var_eta0cv_i_dn14_slot: &mut f64,
        var_eta0cv_i_dn2_slot: &mut f64,
        var_eta0cv_i_dn3_slot: &mut f64,
        var_eta0cv_i_dn4_slot: &mut f64,
        var_eta0cv_i_dn5_slot: &mut f64,
        var_eta0cv_i_dn6_slot: &mut f64,
        var_eta0cv_i_dn7_slot: &mut f64,
        var_eta0cv_i_dn8_slot: &mut f64,
        var_eta0cv_i_dn9_slot: &mut f64,
        var_guard49_slot: &mut f64,
        var_guard50_slot: &mut f64,
        var_guard51_slot: &mut f64,
        var_guard52_slot: &mut f64,
        var_guard53_slot: &mut f64,
        var_guard54_slot: &mut f64,
        var_mfq1nom_i_slot: &mut f64,
        var_mfq2nom_i_slot: &mut f64,
        var_mfq3nom_i_slot: &mut f64,
        var_ssp1_i_slot: &mut f64,
        var_ssp2_i_slot: &mut f64,
        var_ssp3_i_slot: &mut f64,
        var_u0cv_i_slot: &mut f64,
        var_u0cv_i_dn0_slot: &mut f64,
        var_u0cv_i_dn10_slot: &mut f64,
        var_u0cv_i_dn11_slot: &mut f64,
        var_u0cv_i_dn13_slot: &mut f64,
        var_u0cv_i_dn14_slot: &mut f64,
        var_u0cv_i_dn2_slot: &mut f64,
        var_u0cv_i_dn3_slot: &mut f64,
        var_u0cv_i_dn4_slot: &mut f64,
        var_u0cv_i_dn5_slot: &mut f64,
        var_u0cv_i_dn6_slot: &mut f64,
        var_u0cv_i_dn7_slot: &mut f64,
        var_u0cv_i_dn8_slot: &mut f64,
        var_u0cv_i_dn9_slot: &mut f64,
        var_ua2cv_i_slot: &mut f64,
        var_uc1cv_i_slot: &mut f64,
        var_uccv_i_slot: &mut f64,
        var_ud1cv_i_slot: &mut f64,
        var_ud2cv_i_slot: &mut f64,
        var_xrcrg1_i_slot: &mut f64,
        var_xrcrg2_i_slot: &mut f64,
    ) {
        let mut var_dimension1_i: f64 = *var_dimension1_i_slot;
        let mut var_dimension2_i: f64 = *var_dimension2_i_slot;
        let mut var_dimension3_i: f64 = *var_dimension3_i_slot;
        let mut var_e2nom_i: f64 = *var_e2nom_i_slot;
        let mut var_e3nom_i: f64 = *var_e3nom_i_slot;
        let mut var_eta0cv_i: f64 = *var_eta0cv_i_slot;
        let mut var_eta0cv_i_dn0: f64 = *var_eta0cv_i_dn0_slot;
        let mut var_eta0cv_i_dn10: f64 = *var_eta0cv_i_dn10_slot;
        let mut var_eta0cv_i_dn11: f64 = *var_eta0cv_i_dn11_slot;
        let mut var_eta0cv_i_dn13: f64 = *var_eta0cv_i_dn13_slot;
        let mut var_eta0cv_i_dn14: f64 = *var_eta0cv_i_dn14_slot;
        let mut var_eta0cv_i_dn2: f64 = *var_eta0cv_i_dn2_slot;
        let mut var_eta0cv_i_dn3: f64 = *var_eta0cv_i_dn3_slot;
        let mut var_eta0cv_i_dn4: f64 = *var_eta0cv_i_dn4_slot;
        let mut var_eta0cv_i_dn5: f64 = *var_eta0cv_i_dn5_slot;
        let mut var_eta0cv_i_dn6: f64 = *var_eta0cv_i_dn6_slot;
        let mut var_eta0cv_i_dn7: f64 = *var_eta0cv_i_dn7_slot;
        let mut var_eta0cv_i_dn8: f64 = *var_eta0cv_i_dn8_slot;
        let mut var_eta0cv_i_dn9: f64 = *var_eta0cv_i_dn9_slot;
        let mut var_guard49: f64 = *var_guard49_slot;
        let mut var_guard50: f64 = *var_guard50_slot;
        let mut var_guard51: f64 = *var_guard51_slot;
        let mut var_guard52: f64 = *var_guard52_slot;
        let mut var_guard53: f64 = *var_guard53_slot;
        let mut var_guard54: f64 = *var_guard54_slot;
        let mut var_mfq1nom_i: f64 = *var_mfq1nom_i_slot;
        let mut var_mfq2nom_i: f64 = *var_mfq2nom_i_slot;
        let mut var_mfq3nom_i: f64 = *var_mfq3nom_i_slot;
        let mut var_ssp1_i: f64 = *var_ssp1_i_slot;
        let mut var_ssp2_i: f64 = *var_ssp2_i_slot;
        let mut var_ssp3_i: f64 = *var_ssp3_i_slot;
        let mut var_u0cv_i: f64 = *var_u0cv_i_slot;
        let mut var_u0cv_i_dn0: f64 = *var_u0cv_i_dn0_slot;
        let mut var_u0cv_i_dn10: f64 = *var_u0cv_i_dn10_slot;
        let mut var_u0cv_i_dn11: f64 = *var_u0cv_i_dn11_slot;
        let mut var_u0cv_i_dn13: f64 = *var_u0cv_i_dn13_slot;
        let mut var_u0cv_i_dn14: f64 = *var_u0cv_i_dn14_slot;
        let mut var_u0cv_i_dn2: f64 = *var_u0cv_i_dn2_slot;
        let mut var_u0cv_i_dn3: f64 = *var_u0cv_i_dn3_slot;
        let mut var_u0cv_i_dn4: f64 = *var_u0cv_i_dn4_slot;
        let mut var_u0cv_i_dn5: f64 = *var_u0cv_i_dn5_slot;
        let mut var_u0cv_i_dn6: f64 = *var_u0cv_i_dn6_slot;
        let mut var_u0cv_i_dn7: f64 = *var_u0cv_i_dn7_slot;
        let mut var_u0cv_i_dn8: f64 = *var_u0cv_i_dn8_slot;
        let mut var_u0cv_i_dn9: f64 = *var_u0cv_i_dn9_slot;
        let mut var_ua2cv_i: f64 = *var_ua2cv_i_slot;
        let mut var_uc1cv_i: f64 = *var_uc1cv_i_slot;
        let mut var_uccv_i: f64 = *var_uccv_i_slot;
        let mut var_ud1cv_i: f64 = *var_ud1cv_i_slot;
        let mut var_ud2cv_i: f64 = *var_ud2cv_i_slot;
        let mut var_xrcrg1_i: f64 = *var_xrcrg1_i_slot;
        let mut var_xrcrg2_i: f64 = *var_xrcrg2_i_slot;

        let (assign6360_e9596,) = {
    if (var_guard47 != 0.0) {
        let assign6360_e9577: f64 = (var_inv_l * p.p816);
        let assign6360_e9578: f64 = (p.p815 + assign6360_e9577);
        let assign6360_e9581: f64 = (var_inv_nfin * p.p817);
        let assign6360_e9582: f64 = (assign6360_e9578 + assign6360_e9581);
        let assign6360_e9585: f64 = (var_inv_lnfin * p.p818);
        let assign6360_e9586: f64 = (assign6360_e9582 + assign6360_e9585);
        let assign6360_e9589: f64 = (var_inv_w * p.p819);
        let assign6360_e9590: f64 = (assign6360_e9586 + assign6360_e9589);
        let assign6360_e9593: f64 = (var_inv_wl * p.p820);
        let assign6360_e9594: f64 = (assign6360_e9590 + assign6360_e9593);
        (assign6360_e9594,)
    } else {
        (var_ua2cv_i,)
    }
};
        var_ua2cv_i = assign6360_e9596;

        let (assign6370_e9620,) = {
    if (var_guard47 != 0.0) {
        let assign6370_e9601: f64 = (var_inv_l * p.p858);
        let assign6370_e9602: f64 = (p.p857 + assign6370_e9601);
        let assign6370_e9605: f64 = (var_inv_nfin * p.p859);
        let assign6370_e9606: f64 = (assign6370_e9602 + assign6370_e9605);
        let assign6370_e9609: f64 = (var_inv_lnfin * p.p860);
        let assign6370_e9610: f64 = (assign6370_e9606 + assign6370_e9609);
        let assign6370_e9613: f64 = (var_inv_w * p.p861);
        let assign6370_e9614: f64 = (assign6370_e9610 + assign6370_e9613);
        let assign6370_e9617: f64 = (var_inv_wl * p.p862);
        let assign6370_e9618: f64 = (assign6370_e9614 + assign6370_e9617);
        (assign6370_e9618,)
    } else {
        (var_ud1cv_i,)
    }
};
        var_ud1cv_i = assign6370_e9620;

        let (assign6380_e9644,) = {
    if (var_guard47 != 0.0) {
        let assign6380_e9625: f64 = (var_inv_l * p.p870);
        let assign6380_e9626: f64 = (p.p869 + assign6380_e9625);
        let assign6380_e9629: f64 = (var_inv_nfin * p.p871);
        let assign6380_e9630: f64 = (assign6380_e9626 + assign6380_e9629);
        let assign6380_e9633: f64 = (var_inv_lnfin * p.p872);
        let assign6380_e9634: f64 = (assign6380_e9630 + assign6380_e9633);
        let assign6380_e9637: f64 = (var_inv_w * p.p873);
        let assign6380_e9638: f64 = (assign6380_e9634 + assign6380_e9637);
        let assign6380_e9641: f64 = (var_inv_wl * p.p874);
        let assign6380_e9642: f64 = (assign6380_e9638 + assign6380_e9641);
        (assign6380_e9642,)
    } else {
        (var_ud2cv_i,)
    }
};
        var_ud2cv_i = assign6380_e9644;

        let assign6390_e9647: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        var_guard49 = assign6390_e9647;

        let (assign6400_e9673,) = {
    if ((var_guard47 != 0.0) && (var_guard49 != 0.0)) {
        let assign6400_e9654: f64 = (var_inv_l * p.p672);
        let assign6400_e9655: f64 = (p.p671 + assign6400_e9654);
        let assign6400_e9658: f64 = (var_inv_nfin * p.p673);
        let assign6400_e9659: f64 = (assign6400_e9655 + assign6400_e9658);
        let assign6400_e9662: f64 = (var_inv_lnfin * p.p674);
        let assign6400_e9663: f64 = (assign6400_e9659 + assign6400_e9662);
        let assign6400_e9666: f64 = (var_inv_w * p.p675);
        let assign6400_e9667: f64 = (assign6400_e9663 + assign6400_e9666);
        let assign6400_e9670: f64 = (var_inv_wl * p.p676);
        let assign6400_e9671: f64 = (assign6400_e9667 + assign6400_e9670);
        (assign6400_e9671,)
    } else {
        (var_uccv_i,)
    }
};
        var_uccv_i = assign6400_e9673;

        let (assign6410_e9699,) = {
    if ((var_guard47 != 0.0) && (var_guard49 != 0.0)) {
        let assign6410_e9680: f64 = (var_inv_l * p.p840);
        let assign6410_e9681: f64 = (p.p839 + assign6410_e9680);
        let assign6410_e9684: f64 = (var_inv_nfin * p.p841);
        let assign6410_e9685: f64 = (assign6410_e9681 + assign6410_e9684);
        let assign6410_e9688: f64 = (var_inv_lnfin * p.p842);
        let assign6410_e9689: f64 = (assign6410_e9685 + assign6410_e9688);
        let assign6410_e9692: f64 = (var_inv_w * p.p843);
        let assign6410_e9693: f64 = (assign6410_e9689 + assign6410_e9692);
        let assign6410_e9696: f64 = (var_inv_wl * p.p844);
        let assign6410_e9697: f64 = (assign6410_e9693 + assign6410_e9696);
        (assign6410_e9697,)
    } else {
        (var_uc1cv_i,)
    }
};
        var_uc1cv_i = assign6410_e9699;

        let (assign6420_e9723, assign6420_e9723_d_n0, assign6420_e9723_d_n2, assign6420_e9723_d_n3, assign6420_e9723_d_n4, assign6420_e9723_d_n5, assign6420_e9723_d_n6, assign6420_e9723_d_n7, assign6420_e9723_d_n8, assign6420_e9723_d_n9, assign6420_e9723_d_n10, assign6420_e9723_d_n11, assign6420_e9723_d_n13, assign6420_e9723_d_n14,) = {
    if (var_guard47 != 0.0) {
        let assign6420_e9704: f64 = (var_inv_l * p.p261);
        let assign6420_e9705: f64 = (p.p260 + assign6420_e9704);
        let assign6420_e9708: f64 = (var_inv_nfin * p.p262);
        let assign6420_e9709: f64 = (assign6420_e9705 + assign6420_e9708);
        let assign6420_e9712: f64 = (var_inv_lnfin * p.p263);
        let assign6420_e9713: f64 = (assign6420_e9709 + assign6420_e9712);
        let assign6420_e9716: f64 = (var_inv_w * p.p264);
        let assign6420_e9717: f64 = (assign6420_e9713 + assign6420_e9716);
        let assign6420_e9720: f64 = (var_inv_wl * p.p265);
        let assign6420_e9721: f64 = (assign6420_e9717 + assign6420_e9720);
        (assign6420_e9721, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_eta0cv_i, var_eta0cv_i_dn0, var_eta0cv_i_dn2, var_eta0cv_i_dn3, var_eta0cv_i_dn4, var_eta0cv_i_dn5, var_eta0cv_i_dn6, var_eta0cv_i_dn7, var_eta0cv_i_dn8, var_eta0cv_i_dn9, var_eta0cv_i_dn10, var_eta0cv_i_dn11, var_eta0cv_i_dn13, var_eta0cv_i_dn14,)
    }
};
        var_eta0cv_i = assign6420_e9723;
        var_eta0cv_i_dn0 = assign6420_e9723_d_n0;
        var_eta0cv_i_dn2 = assign6420_e9723_d_n2;
        var_eta0cv_i_dn3 = assign6420_e9723_d_n3;
        var_eta0cv_i_dn4 = assign6420_e9723_d_n4;
        var_eta0cv_i_dn5 = assign6420_e9723_d_n5;
        var_eta0cv_i_dn6 = assign6420_e9723_d_n6;
        var_eta0cv_i_dn7 = assign6420_e9723_d_n7;
        var_eta0cv_i_dn8 = assign6420_e9723_d_n8;
        var_eta0cv_i_dn9 = assign6420_e9723_d_n9;
        var_eta0cv_i_dn10 = assign6420_e9723_d_n10;
        var_eta0cv_i_dn11 = assign6420_e9723_d_n11;
        var_eta0cv_i_dn13 = assign6420_e9723_d_n13;
        var_eta0cv_i_dn14 = assign6420_e9723_d_n14;

        let assign6430_e9726: f64 = if p.p161 != 0.0 { 1.0 } else { 0.0 };
        var_guard50 = assign6430_e9726;

        let (assign6440_e9765, assign6440_e9765_d_n0, assign6440_e9765_d_n2, assign6440_e9765_d_n3, assign6440_e9765_d_n4, assign6440_e9765_d_n5, assign6440_e9765_d_n6, assign6440_e9765_d_n7, assign6440_e9765_d_n8, assign6440_e9765_d_n9, assign6440_e9765_d_n10, assign6440_e9765_d_n11, assign6440_e9765_d_n13, assign6440_e9765_d_n14,) = {
    if ((var_guard47 != 0.0) && (var_guard50 != 0.0)) {
        let assign6440_e9734: f64 = (p.p161 / p.p5);
        let assign6440_e9738: f64 = (p.p5 / p.p162);
        let assign6440_e9739: f64 = (1.0 + assign6440_e9738);
        let (assign6440_e9760,) = {
            if (!(assign6440_e9739 > 1e-38)) {
                let assign6440_e9744: f64 = (-87.498233534);
                (assign6440_e9744,)
            } else {
                let assign6440_e9748: f64 = (p.p5 / p.p162);
                let assign6440_e9749: f64 = (1.0 + assign6440_e9748);
                let (assign6440_e9759,) = {
                    if (assign6440_e9749 > 1e-38) {
                        let assign6440_e9755: f64 = (p.p5 / p.p162);
                        let assign6440_e9756: f64 = (1.0 + assign6440_e9755);
                        let assign6440_e9757: f64 = (assign6440_e9756).ln();
                        (assign6440_e9757,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6440_e9759,)
            }
        };
        let assign6440_e9761: f64 = (assign6440_e9734 * assign6440_e9760);
        let assign6440_e9762: f64 = (1.0 + assign6440_e9761);
        let assign6440_e9763: f64 = (var_eta0cv_i * assign6440_e9762);
        (assign6440_e9763, (var_eta0cv_i_dn0 * assign6440_e9762), (var_eta0cv_i_dn2 * assign6440_e9762), (var_eta0cv_i_dn3 * assign6440_e9762), (var_eta0cv_i_dn4 * assign6440_e9762), (var_eta0cv_i_dn5 * assign6440_e9762), (var_eta0cv_i_dn6 * assign6440_e9762), (var_eta0cv_i_dn7 * assign6440_e9762), (var_eta0cv_i_dn8 * assign6440_e9762), (var_eta0cv_i_dn9 * assign6440_e9762), (var_eta0cv_i_dn10 * assign6440_e9762), (var_eta0cv_i_dn11 * assign6440_e9762), (var_eta0cv_i_dn13 * assign6440_e9762), (var_eta0cv_i_dn14 * assign6440_e9762),)
    } else {
        (var_eta0cv_i, var_eta0cv_i_dn0, var_eta0cv_i_dn2, var_eta0cv_i_dn3, var_eta0cv_i_dn4, var_eta0cv_i_dn5, var_eta0cv_i_dn6, var_eta0cv_i_dn7, var_eta0cv_i_dn8, var_eta0cv_i_dn9, var_eta0cv_i_dn10, var_eta0cv_i_dn11, var_eta0cv_i_dn13, var_eta0cv_i_dn14,)
    }
};
        var_eta0cv_i = assign6440_e9765;
        var_eta0cv_i_dn0 = assign6440_e9765_d_n0;
        var_eta0cv_i_dn2 = assign6440_e9765_d_n2;
        var_eta0cv_i_dn3 = assign6440_e9765_d_n3;
        var_eta0cv_i_dn4 = assign6440_e9765_d_n4;
        var_eta0cv_i_dn5 = assign6440_e9765_d_n5;
        var_eta0cv_i_dn6 = assign6440_e9765_d_n6;
        var_eta0cv_i_dn7 = assign6440_e9765_d_n7;
        var_eta0cv_i_dn8 = assign6440_e9765_d_n8;
        var_eta0cv_i_dn9 = assign6440_e9765_d_n9;
        var_eta0cv_i_dn10 = assign6440_e9765_d_n10;
        var_eta0cv_i_dn11 = assign6440_e9765_d_n11;
        var_eta0cv_i_dn13 = assign6440_e9765_d_n13;
        var_eta0cv_i_dn14 = assign6440_e9765_d_n14;

        let assign6450_e9768: f64 = if p.p21 != 0.0 { 1.0 } else { 0.0 };
        var_guard51 = assign6450_e9768;

        let (assign6460_e9784, assign6460_e9784_d_n0, assign6460_e9784_d_n2, assign6460_e9784_d_n3, assign6460_e9784_d_n4, assign6460_e9784_d_n5, assign6460_e9784_d_n6, assign6460_e9784_d_n7, assign6460_e9784_d_n8, assign6460_e9784_d_n9, assign6460_e9784_d_n10, assign6460_e9784_d_n11, assign6460_e9784_d_n13, assign6460_e9784_d_n14,) = {
    if ((var_guard47 != 0.0) && (var_guard51 != 0.0)) {
        let assign6460_e9776: f64 = (p.p5 - p.p21);
        let assign6460_e9778: f64 = (assign6460_e9776 * p.p588);
        let assign6460_e9780: f64 = (assign6460_e9778 * var_leff_1);
        let assign6460_e9781: f64 = (1.0 + assign6460_e9780);
        let assign6460_e9782: f64 = (var_u0cv_i * assign6460_e9781);
        (assign6460_e9782, ((var_u0cv_i_dn0 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn0))), ((var_u0cv_i_dn2 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn2))), ((var_u0cv_i_dn3 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn3))), ((var_u0cv_i_dn4 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn4))), ((var_u0cv_i_dn5 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn5))), ((var_u0cv_i_dn6 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn6))), ((var_u0cv_i_dn7 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn7))), ((var_u0cv_i_dn8 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn8))), ((var_u0cv_i_dn9 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn9))), ((var_u0cv_i_dn10 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn10))), ((var_u0cv_i_dn11 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn11))), ((var_u0cv_i_dn13 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn13))), ((var_u0cv_i_dn14 * assign6460_e9781) + (var_u0cv_i * (assign6460_e9778 * var_leff_1_dn14))),)
    } else {
        (var_u0cv_i, var_u0cv_i_dn0, var_u0cv_i_dn2, var_u0cv_i_dn3, var_u0cv_i_dn4, var_u0cv_i_dn5, var_u0cv_i_dn6, var_u0cv_i_dn7, var_u0cv_i_dn8, var_u0cv_i_dn9, var_u0cv_i_dn10, var_u0cv_i_dn11, var_u0cv_i_dn13, var_u0cv_i_dn14,)
    }
};
        var_u0cv_i = assign6460_e9784;
        var_u0cv_i_dn0 = assign6460_e9784_d_n0;
        var_u0cv_i_dn2 = assign6460_e9784_d_n2;
        var_u0cv_i_dn3 = assign6460_e9784_d_n3;
        var_u0cv_i_dn4 = assign6460_e9784_d_n4;
        var_u0cv_i_dn5 = assign6460_e9784_d_n5;
        var_u0cv_i_dn6 = assign6460_e9784_d_n6;
        var_u0cv_i_dn7 = assign6460_e9784_d_n7;
        var_u0cv_i_dn8 = assign6460_e9784_d_n8;
        var_u0cv_i_dn9 = assign6460_e9784_d_n9;
        var_u0cv_i_dn10 = assign6460_e9784_d_n10;
        var_u0cv_i_dn11 = assign6460_e9784_d_n11;
        var_u0cv_i_dn13 = assign6460_e9784_d_n13;
        var_u0cv_i_dn14 = assign6460_e9784_d_n14;

        let (assign6470_e9800, assign6470_e9800_d_n0, assign6470_e9800_d_n2, assign6470_e9800_d_n3, assign6470_e9800_d_n4, assign6470_e9800_d_n5, assign6470_e9800_d_n6, assign6470_e9800_d_n7, assign6470_e9800_d_n8, assign6470_e9800_d_n9, assign6470_e9800_d_n10, assign6470_e9800_d_n11, assign6470_e9800_d_n13, assign6470_e9800_d_n14,) = {
    if ((var_guard47 != 0.0) && (var_guard51 != 0.0)) {
        let assign6470_e9792: f64 = (p.p5 - p.p21);
        let assign6470_e9794: f64 = (assign6470_e9792 * p.p163);
        let assign6470_e9796: f64 = (assign6470_e9794 * var_leff_1);
        let assign6470_e9797: f64 = (1.0 + assign6470_e9796);
        let assign6470_e9798: f64 = (var_eta0cv_i * assign6470_e9797);
        (assign6470_e9798, ((var_eta0cv_i_dn0 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn0))), ((var_eta0cv_i_dn2 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn2))), ((var_eta0cv_i_dn3 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn3))), ((var_eta0cv_i_dn4 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn4))), ((var_eta0cv_i_dn5 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn5))), ((var_eta0cv_i_dn6 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn6))), ((var_eta0cv_i_dn7 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn7))), ((var_eta0cv_i_dn8 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn8))), ((var_eta0cv_i_dn9 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn9))), ((var_eta0cv_i_dn10 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn10))), ((var_eta0cv_i_dn11 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn11))), ((var_eta0cv_i_dn13 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn13))), ((var_eta0cv_i_dn14 * assign6470_e9797) + (var_eta0cv_i * (assign6470_e9794 * var_leff_1_dn14))),)
    } else {
        (var_eta0cv_i, var_eta0cv_i_dn0, var_eta0cv_i_dn2, var_eta0cv_i_dn3, var_eta0cv_i_dn4, var_eta0cv_i_dn5, var_eta0cv_i_dn6, var_eta0cv_i_dn7, var_eta0cv_i_dn8, var_eta0cv_i_dn9, var_eta0cv_i_dn10, var_eta0cv_i_dn11, var_eta0cv_i_dn13, var_eta0cv_i_dn14,)
    }
};
        var_eta0cv_i = assign6470_e9800;
        var_eta0cv_i_dn0 = assign6470_e9800_d_n0;
        var_eta0cv_i_dn2 = assign6470_e9800_d_n2;
        var_eta0cv_i_dn3 = assign6470_e9800_d_n3;
        var_eta0cv_i_dn4 = assign6470_e9800_d_n4;
        var_eta0cv_i_dn5 = assign6470_e9800_d_n5;
        var_eta0cv_i_dn6 = assign6470_e9800_d_n6;
        var_eta0cv_i_dn7 = assign6470_e9800_d_n7;
        var_eta0cv_i_dn8 = assign6470_e9800_d_n8;
        var_eta0cv_i_dn9 = assign6470_e9800_d_n9;
        var_eta0cv_i_dn10 = assign6470_e9800_d_n10;
        var_eta0cv_i_dn11 = assign6470_e9800_d_n11;
        var_eta0cv_i_dn13 = assign6470_e9800_d_n13;
        var_eta0cv_i_dn14 = assign6470_e9800_d_n14;

        let assign6480_e9807: f64 = if ((p.p73 != 0.0) && (p.p1668 != 0.0)) { 1.0 } else { 0.0 };
        var_guard52 = assign6480_e9807;

        let (assign6490_e9831,) = {
    if (var_guard52 != 0.0) {
        let assign6490_e9812: f64 = (var_inv_l * p.p1669);
        let assign6490_e9813: f64 = (p.p1668 + assign6490_e9812);
        let assign6490_e9816: f64 = (var_inv_nfin * p.p1670);
        let assign6490_e9817: f64 = (assign6490_e9813 + assign6490_e9816);
        let assign6490_e9820: f64 = (var_inv_lnfin * p.p1671);
        let assign6490_e9821: f64 = (assign6490_e9817 + assign6490_e9820);
        let assign6490_e9824: f64 = (var_inv_w * p.p1672);
        let assign6490_e9825: f64 = (assign6490_e9821 + assign6490_e9824);
        let assign6490_e9828: f64 = (var_inv_wl * p.p1673);
        let assign6490_e9829: f64 = (assign6490_e9825 + assign6490_e9828);
        (assign6490_e9829,)
    } else {
        (var_xrcrg1_i,)
    }
};
        var_xrcrg1_i = assign6490_e9831;

        let (assign6500_e9855,) = {
    if (var_guard52 != 0.0) {
        let assign6500_e9836: f64 = (var_inv_l * p.p1675);
        let assign6500_e9837: f64 = (p.p1674 + assign6500_e9836);
        let assign6500_e9840: f64 = (var_inv_nfin * p.p1676);
        let assign6500_e9841: f64 = (assign6500_e9837 + assign6500_e9840);
        let assign6500_e9844: f64 = (var_inv_lnfin * p.p1677);
        let assign6500_e9845: f64 = (assign6500_e9841 + assign6500_e9844);
        let assign6500_e9848: f64 = (var_inv_w * p.p1678);
        let assign6500_e9849: f64 = (assign6500_e9845 + assign6500_e9848);
        let assign6500_e9852: f64 = (var_inv_wl * p.p1679);
        let assign6500_e9853: f64 = (assign6500_e9849 + assign6500_e9852);
        (assign6500_e9853,)
    } else {
        (var_xrcrg2_i,)
    }
};
        var_xrcrg2_i = assign6500_e9855;

        let assign6510_e9858: f64 = if p.p57 == 1.0 { 1.0 } else { 0.0 };
        var_guard53 = assign6510_e9858;

        let (assign6520_e9882,) = {
    if (var_guard53 != 0.0) {
        let assign6520_e9863: f64 = (var_inv_l * p.p1808);
        let assign6520_e9864: f64 = (p.p1807 + assign6520_e9863);
        let assign6520_e9867: f64 = (var_inv_nfin * p.p1809);
        let assign6520_e9868: f64 = (assign6520_e9864 + assign6520_e9867);
        let assign6520_e9871: f64 = (var_inv_lnfin * p.p1810);
        let assign6520_e9872: f64 = (assign6520_e9868 + assign6520_e9871);
        let assign6520_e9875: f64 = (var_inv_w * p.p1811);
        let assign6520_e9876: f64 = (assign6520_e9872 + assign6520_e9875);
        let assign6520_e9879: f64 = (var_inv_wl * p.p1812);
        let assign6520_e9880: f64 = (assign6520_e9876 + assign6520_e9879);
        (assign6520_e9880,)
    } else {
        (var_dimension1_i,)
    }
};
        var_dimension1_i = assign6520_e9882;

        let (assign6530_e9906,) = {
    if (var_guard53 != 0.0) {
        let assign6530_e9887: f64 = (var_inv_l * p.p1815);
        let assign6530_e9888: f64 = (p.p1814 + assign6530_e9887);
        let assign6530_e9891: f64 = (var_inv_nfin * p.p1816);
        let assign6530_e9892: f64 = (assign6530_e9888 + assign6530_e9891);
        let assign6530_e9895: f64 = (var_inv_lnfin * p.p1817);
        let assign6530_e9896: f64 = (assign6530_e9892 + assign6530_e9895);
        let assign6530_e9899: f64 = (var_inv_w * p.p1818);
        let assign6530_e9900: f64 = (assign6530_e9896 + assign6530_e9899);
        let assign6530_e9903: f64 = (var_inv_wl * p.p1819);
        let assign6530_e9904: f64 = (assign6530_e9900 + assign6530_e9903);
        (assign6530_e9904,)
    } else {
        (var_dimension2_i,)
    }
};
        var_dimension2_i = assign6530_e9906;

        let (assign6540_e9930,) = {
    if (var_guard53 != 0.0) {
        let assign6540_e9911: f64 = (var_inv_l * p.p1822);
        let assign6540_e9912: f64 = (p.p1821 + assign6540_e9911);
        let assign6540_e9915: f64 = (var_inv_nfin * p.p1823);
        let assign6540_e9916: f64 = (assign6540_e9912 + assign6540_e9915);
        let assign6540_e9919: f64 = (var_inv_lnfin * p.p1824);
        let assign6540_e9920: f64 = (assign6540_e9916 + assign6540_e9919);
        let assign6540_e9923: f64 = (var_inv_w * p.p1825);
        let assign6540_e9924: f64 = (assign6540_e9920 + assign6540_e9923);
        let assign6540_e9927: f64 = (var_inv_wl * p.p1826);
        let assign6540_e9928: f64 = (assign6540_e9924 + assign6540_e9927);
        (assign6540_e9928,)
    } else {
        (var_dimension3_i,)
    }
};
        var_dimension3_i = assign6540_e9930;

        let (assign6550_e9954,) = {
    if (var_guard53 != 0.0) {
        let assign6550_e9935: f64 = (var_inv_l * p.p1830);
        let assign6550_e9936: f64 = (p.p1829 + assign6550_e9935);
        let assign6550_e9939: f64 = (var_inv_nfin * p.p1831);
        let assign6550_e9940: f64 = (assign6550_e9936 + assign6550_e9939);
        let assign6550_e9943: f64 = (var_inv_lnfin * p.p1832);
        let assign6550_e9944: f64 = (assign6550_e9940 + assign6550_e9943);
        let assign6550_e9947: f64 = (var_inv_w * p.p1833);
        let assign6550_e9948: f64 = (assign6550_e9944 + assign6550_e9947);
        let assign6550_e9951: f64 = (var_inv_wl * p.p1834);
        let assign6550_e9952: f64 = (assign6550_e9948 + assign6550_e9951);
        (assign6550_e9952,)
    } else {
        (var_ssp1_i,)
    }
};
        var_ssp1_i = assign6550_e9954;

        let (assign6560_e9978,) = {
    if (var_guard53 != 0.0) {
        let assign6560_e9959: f64 = (var_inv_l * p.p1836);
        let assign6560_e9960: f64 = (p.p1835 + assign6560_e9959);
        let assign6560_e9963: f64 = (var_inv_nfin * p.p1837);
        let assign6560_e9964: f64 = (assign6560_e9960 + assign6560_e9963);
        let assign6560_e9967: f64 = (var_inv_lnfin * p.p1838);
        let assign6560_e9968: f64 = (assign6560_e9964 + assign6560_e9967);
        let assign6560_e9971: f64 = (var_inv_w * p.p1839);
        let assign6560_e9972: f64 = (assign6560_e9968 + assign6560_e9971);
        let assign6560_e9975: f64 = (var_inv_wl * p.p1840);
        let assign6560_e9976: f64 = (assign6560_e9972 + assign6560_e9975);
        (assign6560_e9976,)
    } else {
        (var_ssp2_i,)
    }
};
        var_ssp2_i = assign6560_e9978;

        let (assign6570_e10002,) = {
    if (var_guard53 != 0.0) {
        let assign6570_e9983: f64 = (var_inv_l * p.p1842);
        let assign6570_e9984: f64 = (p.p1841 + assign6570_e9983);
        let assign6570_e9987: f64 = (var_inv_nfin * p.p1843);
        let assign6570_e9988: f64 = (assign6570_e9984 + assign6570_e9987);
        let assign6570_e9991: f64 = (var_inv_lnfin * p.p1844);
        let assign6570_e9992: f64 = (assign6570_e9988 + assign6570_e9991);
        let assign6570_e9995: f64 = (var_inv_w * p.p1845);
        let assign6570_e9996: f64 = (assign6570_e9992 + assign6570_e9995);
        let assign6570_e9999: f64 = (var_inv_wl * p.p1846);
        let assign6570_e10000: f64 = (assign6570_e9996 + assign6570_e9999);
        (assign6570_e10000,)
    } else {
        (var_ssp3_i,)
    }
};
        var_ssp3_i = assign6570_e10002;

        let (assign6580_e10026,) = {
    if (var_guard53 != 0.0) {
        let assign6580_e10007: f64 = (var_inv_l * p.p1854);
        let assign6580_e10008: f64 = (p.p1853 + assign6580_e10007);
        let assign6580_e10011: f64 = (var_inv_nfin * p.p1855);
        let assign6580_e10012: f64 = (assign6580_e10008 + assign6580_e10011);
        let assign6580_e10015: f64 = (var_inv_lnfin * p.p1856);
        let assign6580_e10016: f64 = (assign6580_e10012 + assign6580_e10015);
        let assign6580_e10019: f64 = (var_inv_w * p.p1857);
        let assign6580_e10020: f64 = (assign6580_e10016 + assign6580_e10019);
        let assign6580_e10023: f64 = (var_inv_wl * p.p1858);
        let assign6580_e10024: f64 = (assign6580_e10020 + assign6580_e10023);
        (assign6580_e10024,)
    } else {
        (var_e2nom_i,)
    }
};
        var_e2nom_i = assign6580_e10026;

        let (assign6590_e10050,) = {
    if (var_guard53 != 0.0) {
        let assign6590_e10031: f64 = (var_inv_l * p.p1860);
        let assign6590_e10032: f64 = (p.p1859 + assign6590_e10031);
        let assign6590_e10035: f64 = (var_inv_nfin * p.p1861);
        let assign6590_e10036: f64 = (assign6590_e10032 + assign6590_e10035);
        let assign6590_e10039: f64 = (var_inv_lnfin * p.p1862);
        let assign6590_e10040: f64 = (assign6590_e10036 + assign6590_e10039);
        let assign6590_e10043: f64 = (var_inv_w * p.p1863);
        let assign6590_e10044: f64 = (assign6590_e10040 + assign6590_e10043);
        let assign6590_e10047: f64 = (var_inv_wl * p.p1864);
        let assign6590_e10048: f64 = (assign6590_e10044 + assign6590_e10047);
        (assign6590_e10048,)
    } else {
        (var_e3nom_i,)
    }
};
        var_e3nom_i = assign6590_e10050;

        let (assign6600_e10074,) = {
    if (var_guard53 != 0.0) {
        let assign6600_e10055: f64 = (var_inv_l * p.p1870);
        let assign6600_e10056: f64 = (p.p1869 + assign6600_e10055);
        let assign6600_e10059: f64 = (var_inv_nfin * p.p1871);
        let assign6600_e10060: f64 = (assign6600_e10056 + assign6600_e10059);
        let assign6600_e10063: f64 = (var_inv_lnfin * p.p1872);
        let assign6600_e10064: f64 = (assign6600_e10060 + assign6600_e10063);
        let assign6600_e10067: f64 = (var_inv_w * p.p1873);
        let assign6600_e10068: f64 = (assign6600_e10064 + assign6600_e10067);
        let assign6600_e10071: f64 = (var_inv_wl * p.p1874);
        let assign6600_e10072: f64 = (assign6600_e10068 + assign6600_e10071);
        (assign6600_e10072,)
    } else {
        (var_mfq1nom_i,)
    }
};
        var_mfq1nom_i = assign6600_e10074;

        let (assign6610_e10098,) = {
    if (var_guard53 != 0.0) {
        let assign6610_e10079: f64 = (var_inv_l * p.p1876);
        let assign6610_e10080: f64 = (p.p1875 + assign6610_e10079);
        let assign6610_e10083: f64 = (var_inv_nfin * p.p1877);
        let assign6610_e10084: f64 = (assign6610_e10080 + assign6610_e10083);
        let assign6610_e10087: f64 = (var_inv_lnfin * p.p1878);
        let assign6610_e10088: f64 = (assign6610_e10084 + assign6610_e10087);
        let assign6610_e10091: f64 = (var_inv_w * p.p1879);
        let assign6610_e10092: f64 = (assign6610_e10088 + assign6610_e10091);
        let assign6610_e10095: f64 = (var_inv_wl * p.p1880);
        let assign6610_e10096: f64 = (assign6610_e10092 + assign6610_e10095);
        (assign6610_e10096,)
    } else {
        (var_mfq2nom_i,)
    }
};
        var_mfq2nom_i = assign6610_e10098;

        let (assign6620_e10122,) = {
    if (var_guard53 != 0.0) {
        let assign6620_e10103: f64 = (var_inv_l * p.p1882);
        let assign6620_e10104: f64 = (p.p1881 + assign6620_e10103);
        let assign6620_e10107: f64 = (var_inv_nfin * p.p1883);
        let assign6620_e10108: f64 = (assign6620_e10104 + assign6620_e10107);
        let assign6620_e10111: f64 = (var_inv_lnfin * p.p1884);
        let assign6620_e10112: f64 = (assign6620_e10108 + assign6620_e10111);
        let assign6620_e10115: f64 = (var_inv_w * p.p1885);
        let assign6620_e10116: f64 = (assign6620_e10112 + assign6620_e10115);
        let assign6620_e10119: f64 = (var_inv_wl * p.p1886);
        let assign6620_e10120: f64 = (assign6620_e10116 + assign6620_e10119);
        (assign6620_e10120,)
    } else {
        (var_mfq3nom_i,)
    }
};
        var_mfq3nom_i = assign6620_e10122;

        let assign6630_e10125: f64 = if p.p100 != 0.0 { 1.0 } else { 0.0 };
        var_guard54 = assign6630_e10125;

        *var_dimension1_i_slot = var_dimension1_i;
        *var_dimension2_i_slot = var_dimension2_i;
        *var_dimension3_i_slot = var_dimension3_i;
        *var_e2nom_i_slot = var_e2nom_i;
        *var_e3nom_i_slot = var_e3nom_i;
        *var_eta0cv_i_slot = var_eta0cv_i;
        *var_eta0cv_i_dn0_slot = var_eta0cv_i_dn0;
        *var_eta0cv_i_dn10_slot = var_eta0cv_i_dn10;
        *var_eta0cv_i_dn11_slot = var_eta0cv_i_dn11;
        *var_eta0cv_i_dn13_slot = var_eta0cv_i_dn13;
        *var_eta0cv_i_dn14_slot = var_eta0cv_i_dn14;
        *var_eta0cv_i_dn2_slot = var_eta0cv_i_dn2;
        *var_eta0cv_i_dn3_slot = var_eta0cv_i_dn3;
        *var_eta0cv_i_dn4_slot = var_eta0cv_i_dn4;
        *var_eta0cv_i_dn5_slot = var_eta0cv_i_dn5;
        *var_eta0cv_i_dn6_slot = var_eta0cv_i_dn6;
        *var_eta0cv_i_dn7_slot = var_eta0cv_i_dn7;
        *var_eta0cv_i_dn8_slot = var_eta0cv_i_dn8;
        *var_eta0cv_i_dn9_slot = var_eta0cv_i_dn9;
        *var_guard49_slot = var_guard49;
        *var_guard50_slot = var_guard50;
        *var_guard51_slot = var_guard51;
        *var_guard52_slot = var_guard52;
        *var_guard53_slot = var_guard53;
        *var_guard54_slot = var_guard54;
        *var_mfq1nom_i_slot = var_mfq1nom_i;
        *var_mfq2nom_i_slot = var_mfq2nom_i;
        *var_mfq3nom_i_slot = var_mfq3nom_i;
        *var_ssp1_i_slot = var_ssp1_i;
        *var_ssp2_i_slot = var_ssp2_i;
        *var_ssp3_i_slot = var_ssp3_i;
        *var_u0cv_i_slot = var_u0cv_i;
        *var_u0cv_i_dn0_slot = var_u0cv_i_dn0;
        *var_u0cv_i_dn10_slot = var_u0cv_i_dn10;
        *var_u0cv_i_dn11_slot = var_u0cv_i_dn11;
        *var_u0cv_i_dn13_slot = var_u0cv_i_dn13;
        *var_u0cv_i_dn14_slot = var_u0cv_i_dn14;
        *var_u0cv_i_dn2_slot = var_u0cv_i_dn2;
        *var_u0cv_i_dn3_slot = var_u0cv_i_dn3;
        *var_u0cv_i_dn4_slot = var_u0cv_i_dn4;
        *var_u0cv_i_dn5_slot = var_u0cv_i_dn5;
        *var_u0cv_i_dn6_slot = var_u0cv_i_dn6;
        *var_u0cv_i_dn7_slot = var_u0cv_i_dn7;
        *var_u0cv_i_dn8_slot = var_u0cv_i_dn8;
        *var_u0cv_i_dn9_slot = var_u0cv_i_dn9;
        *var_ua2cv_i_slot = var_ua2cv_i;
        *var_uc1cv_i_slot = var_uc1cv_i;
        *var_uccv_i_slot = var_uccv_i;
        *var_ud1cv_i_slot = var_ud1cv_i;
        *var_ud2cv_i_slot = var_ud2cv_i;
        *var_xrcrg1_i_slot = var_xrcrg1_i;
        *var_xrcrg2_i_slot = var_xrcrg2_i;
    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        var_guard54: f64,
        var_leff_1: f64,
        var_leff_1_dn0: f64,
        var_leff_1_dn10: f64,
        var_leff_1_dn11: f64,
        var_leff_1_dn13: f64,
        var_leff_1_dn14: f64,
        var_leff_1_dn2: f64,
        var_leff_1_dn3: f64,
        var_leff_1_dn4: f64,
        var_leff_1_dn5: f64,
        var_leff_1_dn6: f64,
        var_leff_1_dn7: f64,
        var_leff_1_dn8: f64,
        var_leff_1_dn9: f64,
        var_cdsc_i_slot: &mut f64,
        var_cdscd_i_slot: &mut f64,
        var_cdscdr_i_slot: &mut f64,
        var_eta0_i_slot: &mut f64,
        var_eta0_i_dn0_slot: &mut f64,
        var_eta0_i_dn10_slot: &mut f64,
        var_eta0_i_dn11_slot: &mut f64,
        var_eta0_i_dn13_slot: &mut f64,
        var_eta0_i_dn14_slot: &mut f64,
        var_eta0_i_dn2_slot: &mut f64,
        var_eta0_i_dn3_slot: &mut f64,
        var_eta0_i_dn4_slot: &mut f64,
        var_eta0_i_dn5_slot: &mut f64,
        var_eta0_i_dn6_slot: &mut f64,
        var_eta0_i_dn7_slot: &mut f64,
        var_eta0_i_dn8_slot: &mut f64,
        var_eta0_i_dn9_slot: &mut f64,
        var_guard55_slot: &mut f64,
        var_guard56_slot: &mut f64,
        var_guard57_slot: &mut f64,
        var_guard58_slot: &mut f64,
        var_guard59_slot: &mut f64,
        var_guard60_slot: &mut f64,
        var_guard61_slot: &mut f64,
        var_guard62_slot: &mut f64,
        var_guard63_slot: &mut f64,
        var_guard64_slot: &mut f64,
        var_phig_i_slot: &mut f64,
        var_phig_i_dn0_slot: &mut f64,
        var_phig_i_dn10_slot: &mut f64,
        var_phig_i_dn11_slot: &mut f64,
        var_phig_i_dn13_slot: &mut f64,
        var_phig_i_dn14_slot: &mut f64,
        var_phig_i_dn2_slot: &mut f64,
        var_phig_i_dn3_slot: &mut f64,
        var_phig_i_dn4_slot: &mut f64,
        var_phig_i_dn5_slot: &mut f64,
        var_phig_i_dn6_slot: &mut f64,
        var_phig_i_dn7_slot: &mut f64,
        var_phig_i_dn8_slot: &mut f64,
        var_phig_i_dn9_slot: &mut f64,
        var_u0_i_slot: &mut f64,
        var_u0_i_dn0_slot: &mut f64,
        var_u0_i_dn10_slot: &mut f64,
        var_u0_i_dn11_slot: &mut f64,
        var_u0_i_dn13_slot: &mut f64,
        var_u0_i_dn14_slot: &mut f64,
        var_u0_i_dn2_slot: &mut f64,
        var_u0_i_dn3_slot: &mut f64,
        var_u0_i_dn4_slot: &mut f64,
        var_u0_i_dn5_slot: &mut f64,
        var_u0_i_dn6_slot: &mut f64,
        var_u0_i_dn7_slot: &mut f64,
        var_u0_i_dn8_slot: &mut f64,
        var_u0_i_dn9_slot: &mut f64,
        var_u0r_i_slot: &mut f64,
        var_u0r_i_dn0_slot: &mut f64,
        var_u0r_i_dn10_slot: &mut f64,
        var_u0r_i_dn11_slot: &mut f64,
        var_u0r_i_dn13_slot: &mut f64,
        var_u0r_i_dn14_slot: &mut f64,
        var_u0r_i_dn2_slot: &mut f64,
        var_u0r_i_dn3_slot: &mut f64,
        var_u0r_i_dn4_slot: &mut f64,
        var_u0r_i_dn5_slot: &mut f64,
        var_u0r_i_dn6_slot: &mut f64,
        var_u0r_i_dn7_slot: &mut f64,
        var_u0r_i_dn8_slot: &mut f64,
        var_u0r_i_dn9_slot: &mut f64,
        var_vsat1_i_slot: &mut f64,
        var_vsat1_i_dn0_slot: &mut f64,
        var_vsat1_i_dn10_slot: &mut f64,
        var_vsat1_i_dn11_slot: &mut f64,
        var_vsat1_i_dn13_slot: &mut f64,
        var_vsat1_i_dn14_slot: &mut f64,
        var_vsat1_i_dn2_slot: &mut f64,
        var_vsat1_i_dn3_slot: &mut f64,
        var_vsat1_i_dn4_slot: &mut f64,
        var_vsat1_i_dn5_slot: &mut f64,
        var_vsat1_i_dn6_slot: &mut f64,
        var_vsat1_i_dn7_slot: &mut f64,
        var_vsat1_i_dn8_slot: &mut f64,
        var_vsat1_i_dn9_slot: &mut f64,
        var_vsat1r_i_slot: &mut f64,
        var_vsat1r_i_dn0_slot: &mut f64,
        var_vsat1r_i_dn10_slot: &mut f64,
        var_vsat1r_i_dn11_slot: &mut f64,
        var_vsat1r_i_dn13_slot: &mut f64,
        var_vsat1r_i_dn14_slot: &mut f64,
        var_vsat1r_i_dn2_slot: &mut f64,
        var_vsat1r_i_dn3_slot: &mut f64,
        var_vsat1r_i_dn4_slot: &mut f64,
        var_vsat1r_i_dn5_slot: &mut f64,
        var_vsat1r_i_dn6_slot: &mut f64,
        var_vsat1r_i_dn7_slot: &mut f64,
        var_vsat1r_i_dn8_slot: &mut f64,
        var_vsat1r_i_dn9_slot: &mut f64,
        var_vsat_i_slot: &mut f64,
        var_vsat_i_dn0_slot: &mut f64,
        var_vsat_i_dn10_slot: &mut f64,
        var_vsat_i_dn11_slot: &mut f64,
        var_vsat_i_dn13_slot: &mut f64,
        var_vsat_i_dn14_slot: &mut f64,
        var_vsat_i_dn2_slot: &mut f64,
        var_vsat_i_dn3_slot: &mut f64,
        var_vsat_i_dn4_slot: &mut f64,
        var_vsat_i_dn5_slot: &mut f64,
        var_vsat_i_dn6_slot: &mut f64,
        var_vsat_i_dn7_slot: &mut f64,
        var_vsat_i_dn8_slot: &mut f64,
        var_vsat_i_dn9_slot: &mut f64,
    ) {
        let mut var_cdsc_i: f64 = *var_cdsc_i_slot;
        let mut var_cdscd_i: f64 = *var_cdscd_i_slot;
        let mut var_cdscdr_i: f64 = *var_cdscdr_i_slot;
        let mut var_eta0_i: f64 = *var_eta0_i_slot;
        let mut var_eta0_i_dn0: f64 = *var_eta0_i_dn0_slot;
        let mut var_eta0_i_dn10: f64 = *var_eta0_i_dn10_slot;
        let mut var_eta0_i_dn11: f64 = *var_eta0_i_dn11_slot;
        let mut var_eta0_i_dn13: f64 = *var_eta0_i_dn13_slot;
        let mut var_eta0_i_dn14: f64 = *var_eta0_i_dn14_slot;
        let mut var_eta0_i_dn2: f64 = *var_eta0_i_dn2_slot;
        let mut var_eta0_i_dn3: f64 = *var_eta0_i_dn3_slot;
        let mut var_eta0_i_dn4: f64 = *var_eta0_i_dn4_slot;
        let mut var_eta0_i_dn5: f64 = *var_eta0_i_dn5_slot;
        let mut var_eta0_i_dn6: f64 = *var_eta0_i_dn6_slot;
        let mut var_eta0_i_dn7: f64 = *var_eta0_i_dn7_slot;
        let mut var_eta0_i_dn8: f64 = *var_eta0_i_dn8_slot;
        let mut var_eta0_i_dn9: f64 = *var_eta0_i_dn9_slot;
        let mut var_guard55: f64 = *var_guard55_slot;
        let mut var_guard56: f64 = *var_guard56_slot;
        let mut var_guard57: f64 = *var_guard57_slot;
        let mut var_guard58: f64 = *var_guard58_slot;
        let mut var_guard59: f64 = *var_guard59_slot;
        let mut var_guard60: f64 = *var_guard60_slot;
        let mut var_guard61: f64 = *var_guard61_slot;
        let mut var_guard62: f64 = *var_guard62_slot;
        let mut var_guard63: f64 = *var_guard63_slot;
        let mut var_guard64: f64 = *var_guard64_slot;
        let mut var_phig_i: f64 = *var_phig_i_slot;
        let mut var_phig_i_dn0: f64 = *var_phig_i_dn0_slot;
        let mut var_phig_i_dn10: f64 = *var_phig_i_dn10_slot;
        let mut var_phig_i_dn11: f64 = *var_phig_i_dn11_slot;
        let mut var_phig_i_dn13: f64 = *var_phig_i_dn13_slot;
        let mut var_phig_i_dn14: f64 = *var_phig_i_dn14_slot;
        let mut var_phig_i_dn2: f64 = *var_phig_i_dn2_slot;
        let mut var_phig_i_dn3: f64 = *var_phig_i_dn3_slot;
        let mut var_phig_i_dn4: f64 = *var_phig_i_dn4_slot;
        let mut var_phig_i_dn5: f64 = *var_phig_i_dn5_slot;
        let mut var_phig_i_dn6: f64 = *var_phig_i_dn6_slot;
        let mut var_phig_i_dn7: f64 = *var_phig_i_dn7_slot;
        let mut var_phig_i_dn8: f64 = *var_phig_i_dn8_slot;
        let mut var_phig_i_dn9: f64 = *var_phig_i_dn9_slot;
        let mut var_u0_i: f64 = *var_u0_i_slot;
        let mut var_u0_i_dn0: f64 = *var_u0_i_dn0_slot;
        let mut var_u0_i_dn10: f64 = *var_u0_i_dn10_slot;
        let mut var_u0_i_dn11: f64 = *var_u0_i_dn11_slot;
        let mut var_u0_i_dn13: f64 = *var_u0_i_dn13_slot;
        let mut var_u0_i_dn14: f64 = *var_u0_i_dn14_slot;
        let mut var_u0_i_dn2: f64 = *var_u0_i_dn2_slot;
        let mut var_u0_i_dn3: f64 = *var_u0_i_dn3_slot;
        let mut var_u0_i_dn4: f64 = *var_u0_i_dn4_slot;
        let mut var_u0_i_dn5: f64 = *var_u0_i_dn5_slot;
        let mut var_u0_i_dn6: f64 = *var_u0_i_dn6_slot;
        let mut var_u0_i_dn7: f64 = *var_u0_i_dn7_slot;
        let mut var_u0_i_dn8: f64 = *var_u0_i_dn8_slot;
        let mut var_u0_i_dn9: f64 = *var_u0_i_dn9_slot;
        let mut var_u0r_i: f64 = *var_u0r_i_slot;
        let mut var_u0r_i_dn0: f64 = *var_u0r_i_dn0_slot;
        let mut var_u0r_i_dn10: f64 = *var_u0r_i_dn10_slot;
        let mut var_u0r_i_dn11: f64 = *var_u0r_i_dn11_slot;
        let mut var_u0r_i_dn13: f64 = *var_u0r_i_dn13_slot;
        let mut var_u0r_i_dn14: f64 = *var_u0r_i_dn14_slot;
        let mut var_u0r_i_dn2: f64 = *var_u0r_i_dn2_slot;
        let mut var_u0r_i_dn3: f64 = *var_u0r_i_dn3_slot;
        let mut var_u0r_i_dn4: f64 = *var_u0r_i_dn4_slot;
        let mut var_u0r_i_dn5: f64 = *var_u0r_i_dn5_slot;
        let mut var_u0r_i_dn6: f64 = *var_u0r_i_dn6_slot;
        let mut var_u0r_i_dn7: f64 = *var_u0r_i_dn7_slot;
        let mut var_u0r_i_dn8: f64 = *var_u0r_i_dn8_slot;
        let mut var_u0r_i_dn9: f64 = *var_u0r_i_dn9_slot;
        let mut var_vsat1_i: f64 = *var_vsat1_i_slot;
        let mut var_vsat1_i_dn0: f64 = *var_vsat1_i_dn0_slot;
        let mut var_vsat1_i_dn10: f64 = *var_vsat1_i_dn10_slot;
        let mut var_vsat1_i_dn11: f64 = *var_vsat1_i_dn11_slot;
        let mut var_vsat1_i_dn13: f64 = *var_vsat1_i_dn13_slot;
        let mut var_vsat1_i_dn14: f64 = *var_vsat1_i_dn14_slot;
        let mut var_vsat1_i_dn2: f64 = *var_vsat1_i_dn2_slot;
        let mut var_vsat1_i_dn3: f64 = *var_vsat1_i_dn3_slot;
        let mut var_vsat1_i_dn4: f64 = *var_vsat1_i_dn4_slot;
        let mut var_vsat1_i_dn5: f64 = *var_vsat1_i_dn5_slot;
        let mut var_vsat1_i_dn6: f64 = *var_vsat1_i_dn6_slot;
        let mut var_vsat1_i_dn7: f64 = *var_vsat1_i_dn7_slot;
        let mut var_vsat1_i_dn8: f64 = *var_vsat1_i_dn8_slot;
        let mut var_vsat1_i_dn9: f64 = *var_vsat1_i_dn9_slot;
        let mut var_vsat1r_i: f64 = *var_vsat1r_i_slot;
        let mut var_vsat1r_i_dn0: f64 = *var_vsat1r_i_dn0_slot;
        let mut var_vsat1r_i_dn10: f64 = *var_vsat1r_i_dn10_slot;
        let mut var_vsat1r_i_dn11: f64 = *var_vsat1r_i_dn11_slot;
        let mut var_vsat1r_i_dn13: f64 = *var_vsat1r_i_dn13_slot;
        let mut var_vsat1r_i_dn14: f64 = *var_vsat1r_i_dn14_slot;
        let mut var_vsat1r_i_dn2: f64 = *var_vsat1r_i_dn2_slot;
        let mut var_vsat1r_i_dn3: f64 = *var_vsat1r_i_dn3_slot;
        let mut var_vsat1r_i_dn4: f64 = *var_vsat1r_i_dn4_slot;
        let mut var_vsat1r_i_dn5: f64 = *var_vsat1r_i_dn5_slot;
        let mut var_vsat1r_i_dn6: f64 = *var_vsat1r_i_dn6_slot;
        let mut var_vsat1r_i_dn7: f64 = *var_vsat1r_i_dn7_slot;
        let mut var_vsat1r_i_dn8: f64 = *var_vsat1r_i_dn8_slot;
        let mut var_vsat1r_i_dn9: f64 = *var_vsat1r_i_dn9_slot;
        let mut var_vsat_i: f64 = *var_vsat_i_slot;
        let mut var_vsat_i_dn0: f64 = *var_vsat_i_dn0_slot;
        let mut var_vsat_i_dn10: f64 = *var_vsat_i_dn10_slot;
        let mut var_vsat_i_dn11: f64 = *var_vsat_i_dn11_slot;
        let mut var_vsat_i_dn13: f64 = *var_vsat_i_dn13_slot;
        let mut var_vsat_i_dn14: f64 = *var_vsat_i_dn14_slot;
        let mut var_vsat_i_dn2: f64 = *var_vsat_i_dn2_slot;
        let mut var_vsat_i_dn3: f64 = *var_vsat_i_dn3_slot;
        let mut var_vsat_i_dn4: f64 = *var_vsat_i_dn4_slot;
        let mut var_vsat_i_dn5: f64 = *var_vsat_i_dn5_slot;
        let mut var_vsat_i_dn6: f64 = *var_vsat_i_dn6_slot;
        let mut var_vsat_i_dn7: f64 = *var_vsat_i_dn7_slot;
        let mut var_vsat_i_dn8: f64 = *var_vsat_i_dn8_slot;
        let mut var_vsat_i_dn9: f64 = *var_vsat_i_dn9_slot;

        let (assign6640_e10162, assign6640_e10162_d_n0, assign6640_e10162_d_n2, assign6640_e10162_d_n3, assign6640_e10162_d_n4, assign6640_e10162_d_n5, assign6640_e10162_d_n6, assign6640_e10162_d_n7, assign6640_e10162_d_n8, assign6640_e10162_d_n9, assign6640_e10162_d_n10, assign6640_e10162_d_n11, assign6640_e10162_d_n13, assign6640_e10162_d_n14,) = {
    if (var_guard54 != 0.0) {
        let assign6640_e10131: f64 = (p.p100 / p.p5);
        let assign6640_e10135: f64 = (p.p5 / p.p101);
        let assign6640_e10136: f64 = (1.0 + assign6640_e10135);
        let (assign6640_e10157,) = {
            if (!(assign6640_e10136 > 1e-38)) {
                let assign6640_e10141: f64 = (-87.498233534);
                (assign6640_e10141,)
            } else {
                let assign6640_e10145: f64 = (p.p5 / p.p101);
                let assign6640_e10146: f64 = (1.0 + assign6640_e10145);
                let (assign6640_e10156,) = {
                    if (assign6640_e10146 > 1e-38) {
                        let assign6640_e10152: f64 = (p.p5 / p.p101);
                        let assign6640_e10153: f64 = (1.0 + assign6640_e10152);
                        let assign6640_e10154: f64 = (assign6640_e10153).ln();
                        (assign6640_e10154,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6640_e10156,)
            }
        };
        let assign6640_e10158: f64 = (assign6640_e10131 * assign6640_e10157);
        let assign6640_e10159: f64 = (1.0 + assign6640_e10158);
        let assign6640_e10160: f64 = (var_phig_i * assign6640_e10159);
        (assign6640_e10160, (var_phig_i_dn0 * assign6640_e10159), (var_phig_i_dn2 * assign6640_e10159), (var_phig_i_dn3 * assign6640_e10159), (var_phig_i_dn4 * assign6640_e10159), (var_phig_i_dn5 * assign6640_e10159), (var_phig_i_dn6 * assign6640_e10159), (var_phig_i_dn7 * assign6640_e10159), (var_phig_i_dn8 * assign6640_e10159), (var_phig_i_dn9 * assign6640_e10159), (var_phig_i_dn10 * assign6640_e10159), (var_phig_i_dn11 * assign6640_e10159), (var_phig_i_dn13 * assign6640_e10159), (var_phig_i_dn14 * assign6640_e10159),)
    } else {
        (var_phig_i, var_phig_i_dn0, var_phig_i_dn2, var_phig_i_dn3, var_phig_i_dn4, var_phig_i_dn5, var_phig_i_dn6, var_phig_i_dn7, var_phig_i_dn8, var_phig_i_dn9, var_phig_i_dn10, var_phig_i_dn11, var_phig_i_dn13, var_phig_i_dn14,)
    }
};
        var_phig_i = assign6640_e10162;
        var_phig_i_dn0 = assign6640_e10162_d_n0;
        var_phig_i_dn2 = assign6640_e10162_d_n2;
        var_phig_i_dn3 = assign6640_e10162_d_n3;
        var_phig_i_dn4 = assign6640_e10162_d_n4;
        var_phig_i_dn5 = assign6640_e10162_d_n5;
        var_phig_i_dn6 = assign6640_e10162_d_n6;
        var_phig_i_dn7 = assign6640_e10162_d_n7;
        var_phig_i_dn8 = assign6640_e10162_d_n8;
        var_phig_i_dn9 = assign6640_e10162_d_n9;
        var_phig_i_dn10 = assign6640_e10162_d_n10;
        var_phig_i_dn11 = assign6640_e10162_d_n11;
        var_phig_i_dn13 = assign6640_e10162_d_n13;
        var_phig_i_dn14 = assign6640_e10162_d_n14;

        let assign6650_e10165: f64 = if p.p158 != 0.0 { 1.0 } else { 0.0 };
        var_guard55 = assign6650_e10165;

        let (assign6660_e10202, assign6660_e10202_d_n0, assign6660_e10202_d_n2, assign6660_e10202_d_n3, assign6660_e10202_d_n4, assign6660_e10202_d_n5, assign6660_e10202_d_n6, assign6660_e10202_d_n7, assign6660_e10202_d_n8, assign6660_e10202_d_n9, assign6660_e10202_d_n10, assign6660_e10202_d_n11, assign6660_e10202_d_n13, assign6660_e10202_d_n14,) = {
    if (var_guard55 != 0.0) {
        let assign6660_e10171: f64 = (p.p158 / p.p5);
        let assign6660_e10175: f64 = (p.p5 / p.p159);
        let assign6660_e10176: f64 = (1.0 + assign6660_e10175);
        let (assign6660_e10197,) = {
            if (!(assign6660_e10176 > 1e-38)) {
                let assign6660_e10181: f64 = (-87.498233534);
                (assign6660_e10181,)
            } else {
                let assign6660_e10185: f64 = (p.p5 / p.p159);
                let assign6660_e10186: f64 = (1.0 + assign6660_e10185);
                let (assign6660_e10196,) = {
                    if (assign6660_e10186 > 1e-38) {
                        let assign6660_e10192: f64 = (p.p5 / p.p159);
                        let assign6660_e10193: f64 = (1.0 + assign6660_e10192);
                        let assign6660_e10194: f64 = (assign6660_e10193).ln();
                        (assign6660_e10194,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6660_e10196,)
            }
        };
        let assign6660_e10198: f64 = (assign6660_e10171 * assign6660_e10197);
        let assign6660_e10199: f64 = (1.0 + assign6660_e10198);
        let assign6660_e10200: f64 = (var_eta0_i * assign6660_e10199);
        (assign6660_e10200, (var_eta0_i_dn0 * assign6660_e10199), (var_eta0_i_dn2 * assign6660_e10199), (var_eta0_i_dn3 * assign6660_e10199), (var_eta0_i_dn4 * assign6660_e10199), (var_eta0_i_dn5 * assign6660_e10199), (var_eta0_i_dn6 * assign6660_e10199), (var_eta0_i_dn7 * assign6660_e10199), (var_eta0_i_dn8 * assign6660_e10199), (var_eta0_i_dn9 * assign6660_e10199), (var_eta0_i_dn10 * assign6660_e10199), (var_eta0_i_dn11 * assign6660_e10199), (var_eta0_i_dn13 * assign6660_e10199), (var_eta0_i_dn14 * assign6660_e10199),)
    } else {
        (var_eta0_i, var_eta0_i_dn0, var_eta0_i_dn2, var_eta0_i_dn3, var_eta0_i_dn4, var_eta0_i_dn5, var_eta0_i_dn6, var_eta0_i_dn7, var_eta0_i_dn8, var_eta0_i_dn9, var_eta0_i_dn10, var_eta0_i_dn11, var_eta0_i_dn13, var_eta0_i_dn14,)
    }
};
        var_eta0_i = assign6660_e10202;
        var_eta0_i_dn0 = assign6660_e10202_d_n0;
        var_eta0_i_dn2 = assign6660_e10202_d_n2;
        var_eta0_i_dn3 = assign6660_e10202_d_n3;
        var_eta0_i_dn4 = assign6660_e10202_d_n4;
        var_eta0_i_dn5 = assign6660_e10202_d_n5;
        var_eta0_i_dn6 = assign6660_e10202_d_n6;
        var_eta0_i_dn7 = assign6660_e10202_d_n7;
        var_eta0_i_dn8 = assign6660_e10202_d_n8;
        var_eta0_i_dn9 = assign6660_e10202_d_n9;
        var_eta0_i_dn10 = assign6660_e10202_d_n10;
        var_eta0_i_dn11 = assign6660_e10202_d_n11;
        var_eta0_i_dn13 = assign6660_e10202_d_n13;
        var_eta0_i_dn14 = assign6660_e10202_d_n14;

        let assign6670_e10205: f64 = if p.p152 != 0.0 { 1.0 } else { 0.0 };
        var_guard56 = assign6670_e10205;

        let (assign6680_e10242,) = {
    if (var_guard56 != 0.0) {
        let assign6680_e10211: f64 = (p.p152 / p.p5);
        let assign6680_e10215: f64 = (p.p5 / p.p153);
        let assign6680_e10216: f64 = (1.0 + assign6680_e10215);
        let (assign6680_e10237,) = {
            if (!(assign6680_e10216 > 1e-38)) {
                let assign6680_e10221: f64 = (-87.498233534);
                (assign6680_e10221,)
            } else {
                let assign6680_e10225: f64 = (p.p5 / p.p153);
                let assign6680_e10226: f64 = (1.0 + assign6680_e10225);
                let (assign6680_e10236,) = {
                    if (assign6680_e10226 > 1e-38) {
                        let assign6680_e10232: f64 = (p.p5 / p.p153);
                        let assign6680_e10233: f64 = (1.0 + assign6680_e10232);
                        let assign6680_e10234: f64 = (assign6680_e10233).ln();
                        (assign6680_e10234,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6680_e10236,)
            }
        };
        let assign6680_e10238: f64 = (assign6680_e10211 * assign6680_e10237);
        let assign6680_e10239: f64 = (1.0 + assign6680_e10238);
        let assign6680_e10240: f64 = (var_cdsc_i * assign6680_e10239);
        (assign6680_e10240,)
    } else {
        (var_cdsc_i,)
    }
};
        var_cdsc_i = assign6680_e10242;

        let assign6690_e10245: f64 = if p.p154 != 0.0 { 1.0 } else { 0.0 };
        var_guard57 = assign6690_e10245;

        let (assign6700_e10282,) = {
    if (var_guard57 != 0.0) {
        let assign6700_e10251: f64 = (p.p154 / p.p5);
        let assign6700_e10255: f64 = (p.p5 / p.p155);
        let assign6700_e10256: f64 = (1.0 + assign6700_e10255);
        let (assign6700_e10277,) = {
            if (!(assign6700_e10256 > 1e-38)) {
                let assign6700_e10261: f64 = (-87.498233534);
                (assign6700_e10261,)
            } else {
                let assign6700_e10265: f64 = (p.p5 / p.p155);
                let assign6700_e10266: f64 = (1.0 + assign6700_e10265);
                let (assign6700_e10276,) = {
                    if (assign6700_e10266 > 1e-38) {
                        let assign6700_e10272: f64 = (p.p5 / p.p155);
                        let assign6700_e10273: f64 = (1.0 + assign6700_e10272);
                        let assign6700_e10274: f64 = (assign6700_e10273).ln();
                        (assign6700_e10274,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6700_e10276,)
            }
        };
        let assign6700_e10278: f64 = (assign6700_e10251 * assign6700_e10277);
        let assign6700_e10279: f64 = (1.0 + assign6700_e10278);
        let assign6700_e10280: f64 = (var_cdscd_i * assign6700_e10279);
        (assign6700_e10280,)
    } else {
        (var_cdscd_i,)
    }
};
        var_cdscd_i = assign6700_e10282;

        let assign6710_e10285: f64 = if p.p156 != 0.0 { 1.0 } else { 0.0 };
        var_guard58 = assign6710_e10285;

        let (assign6720_e10322,) = {
    if (var_guard58 != 0.0) {
        let assign6720_e10291: f64 = (p.p156 / p.p5);
        let assign6720_e10295: f64 = (p.p5 / p.p157);
        let assign6720_e10296: f64 = (1.0 + assign6720_e10295);
        let (assign6720_e10317,) = {
            if (!(assign6720_e10296 > 1e-38)) {
                let assign6720_e10301: f64 = (-87.498233534);
                (assign6720_e10301,)
            } else {
                let assign6720_e10305: f64 = (p.p5 / p.p157);
                let assign6720_e10306: f64 = (1.0 + assign6720_e10305);
                let (assign6720_e10316,) = {
                    if (assign6720_e10306 > 1e-38) {
                        let assign6720_e10312: f64 = (p.p5 / p.p157);
                        let assign6720_e10313: f64 = (1.0 + assign6720_e10312);
                        let assign6720_e10314: f64 = (assign6720_e10313).ln();
                        (assign6720_e10314,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6720_e10316,)
            }
        };
        let assign6720_e10318: f64 = (assign6720_e10291 * assign6720_e10317);
        let assign6720_e10319: f64 = (1.0 + assign6720_e10318);
        let assign6720_e10320: f64 = (var_cdscdr_i * assign6720_e10319);
        (assign6720_e10320,)
    } else {
        (var_cdscdr_i,)
    }
};
        var_cdscdr_i = assign6720_e10322;

        let assign6730_e10325: f64 = if p.p428 != 0.0 { 1.0 } else { 0.0 };
        var_guard59 = assign6730_e10325;

        let (assign6740_e10362, assign6740_e10362_d_n0, assign6740_e10362_d_n2, assign6740_e10362_d_n3, assign6740_e10362_d_n4, assign6740_e10362_d_n5, assign6740_e10362_d_n6, assign6740_e10362_d_n7, assign6740_e10362_d_n8, assign6740_e10362_d_n9, assign6740_e10362_d_n10, assign6740_e10362_d_n11, assign6740_e10362_d_n13, assign6740_e10362_d_n14,) = {
    if (var_guard59 != 0.0) {
        let assign6740_e10331: f64 = (p.p428 / p.p5);
        let assign6740_e10335: f64 = (p.p5 / p.p429);
        let assign6740_e10336: f64 = (1.0 + assign6740_e10335);
        let (assign6740_e10357,) = {
            if (!(assign6740_e10336 > 1e-38)) {
                let assign6740_e10341: f64 = (-87.498233534);
                (assign6740_e10341,)
            } else {
                let assign6740_e10345: f64 = (p.p5 / p.p429);
                let assign6740_e10346: f64 = (1.0 + assign6740_e10345);
                let (assign6740_e10356,) = {
                    if (assign6740_e10346 > 1e-38) {
                        let assign6740_e10352: f64 = (p.p5 / p.p429);
                        let assign6740_e10353: f64 = (1.0 + assign6740_e10352);
                        let assign6740_e10354: f64 = (assign6740_e10353).ln();
                        (assign6740_e10354,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6740_e10356,)
            }
        };
        let assign6740_e10358: f64 = (assign6740_e10331 * assign6740_e10357);
        let assign6740_e10359: f64 = (1.0 + assign6740_e10358);
        let assign6740_e10360: f64 = (var_vsat_i * assign6740_e10359);
        (assign6740_e10360, (var_vsat_i_dn0 * assign6740_e10359), (var_vsat_i_dn2 * assign6740_e10359), (var_vsat_i_dn3 * assign6740_e10359), (var_vsat_i_dn4 * assign6740_e10359), (var_vsat_i_dn5 * assign6740_e10359), (var_vsat_i_dn6 * assign6740_e10359), (var_vsat_i_dn7 * assign6740_e10359), (var_vsat_i_dn8 * assign6740_e10359), (var_vsat_i_dn9 * assign6740_e10359), (var_vsat_i_dn10 * assign6740_e10359), (var_vsat_i_dn11 * assign6740_e10359), (var_vsat_i_dn13 * assign6740_e10359), (var_vsat_i_dn14 * assign6740_e10359),)
    } else {
        (var_vsat_i, var_vsat_i_dn0, var_vsat_i_dn2, var_vsat_i_dn3, var_vsat_i_dn4, var_vsat_i_dn5, var_vsat_i_dn6, var_vsat_i_dn7, var_vsat_i_dn8, var_vsat_i_dn9, var_vsat_i_dn10, var_vsat_i_dn11, var_vsat_i_dn13, var_vsat_i_dn14,)
    }
};
        var_vsat_i = assign6740_e10362;
        var_vsat_i_dn0 = assign6740_e10362_d_n0;
        var_vsat_i_dn2 = assign6740_e10362_d_n2;
        var_vsat_i_dn3 = assign6740_e10362_d_n3;
        var_vsat_i_dn4 = assign6740_e10362_d_n4;
        var_vsat_i_dn5 = assign6740_e10362_d_n5;
        var_vsat_i_dn6 = assign6740_e10362_d_n6;
        var_vsat_i_dn7 = assign6740_e10362_d_n7;
        var_vsat_i_dn8 = assign6740_e10362_d_n8;
        var_vsat_i_dn9 = assign6740_e10362_d_n9;
        var_vsat_i_dn10 = assign6740_e10362_d_n10;
        var_vsat_i_dn11 = assign6740_e10362_d_n11;
        var_vsat_i_dn13 = assign6740_e10362_d_n13;
        var_vsat_i_dn14 = assign6740_e10362_d_n14;

        let assign6750_e10365: f64 = if p.p432 != 0.0 { 1.0 } else { 0.0 };
        var_guard60 = assign6750_e10365;

        let (assign6760_e10402, assign6760_e10402_d_n0, assign6760_e10402_d_n2, assign6760_e10402_d_n3, assign6760_e10402_d_n4, assign6760_e10402_d_n5, assign6760_e10402_d_n6, assign6760_e10402_d_n7, assign6760_e10402_d_n8, assign6760_e10402_d_n9, assign6760_e10402_d_n10, assign6760_e10402_d_n11, assign6760_e10402_d_n13, assign6760_e10402_d_n14,) = {
    if (var_guard60 != 0.0) {
        let assign6760_e10371: f64 = (p.p432 / p.p5);
        let assign6760_e10375: f64 = (p.p5 / p.p433);
        let assign6760_e10376: f64 = (1.0 + assign6760_e10375);
        let (assign6760_e10397,) = {
            if (!(assign6760_e10376 > 1e-38)) {
                let assign6760_e10381: f64 = (-87.498233534);
                (assign6760_e10381,)
            } else {
                let assign6760_e10385: f64 = (p.p5 / p.p433);
                let assign6760_e10386: f64 = (1.0 + assign6760_e10385);
                let (assign6760_e10396,) = {
                    if (assign6760_e10386 > 1e-38) {
                        let assign6760_e10392: f64 = (p.p5 / p.p433);
                        let assign6760_e10393: f64 = (1.0 + assign6760_e10392);
                        let assign6760_e10394: f64 = (assign6760_e10393).ln();
                        (assign6760_e10394,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6760_e10396,)
            }
        };
        let assign6760_e10398: f64 = (assign6760_e10371 * assign6760_e10397);
        let assign6760_e10399: f64 = (1.0 + assign6760_e10398);
        let assign6760_e10400: f64 = (var_vsat1_i * assign6760_e10399);
        (assign6760_e10400, (var_vsat1_i_dn0 * assign6760_e10399), (var_vsat1_i_dn2 * assign6760_e10399), (var_vsat1_i_dn3 * assign6760_e10399), (var_vsat1_i_dn4 * assign6760_e10399), (var_vsat1_i_dn5 * assign6760_e10399), (var_vsat1_i_dn6 * assign6760_e10399), (var_vsat1_i_dn7 * assign6760_e10399), (var_vsat1_i_dn8 * assign6760_e10399), (var_vsat1_i_dn9 * assign6760_e10399), (var_vsat1_i_dn10 * assign6760_e10399), (var_vsat1_i_dn11 * assign6760_e10399), (var_vsat1_i_dn13 * assign6760_e10399), (var_vsat1_i_dn14 * assign6760_e10399),)
    } else {
        (var_vsat1_i, var_vsat1_i_dn0, var_vsat1_i_dn2, var_vsat1_i_dn3, var_vsat1_i_dn4, var_vsat1_i_dn5, var_vsat1_i_dn6, var_vsat1_i_dn7, var_vsat1_i_dn8, var_vsat1_i_dn9, var_vsat1_i_dn10, var_vsat1_i_dn11, var_vsat1_i_dn13, var_vsat1_i_dn14,)
    }
};
        var_vsat1_i = assign6760_e10402;
        var_vsat1_i_dn0 = assign6760_e10402_d_n0;
        var_vsat1_i_dn2 = assign6760_e10402_d_n2;
        var_vsat1_i_dn3 = assign6760_e10402_d_n3;
        var_vsat1_i_dn4 = assign6760_e10402_d_n4;
        var_vsat1_i_dn5 = assign6760_e10402_d_n5;
        var_vsat1_i_dn6 = assign6760_e10402_d_n6;
        var_vsat1_i_dn7 = assign6760_e10402_d_n7;
        var_vsat1_i_dn8 = assign6760_e10402_d_n8;
        var_vsat1_i_dn9 = assign6760_e10402_d_n9;
        var_vsat1_i_dn10 = assign6760_e10402_d_n10;
        var_vsat1_i_dn11 = assign6760_e10402_d_n11;
        var_vsat1_i_dn13 = assign6760_e10402_d_n13;
        var_vsat1_i_dn14 = assign6760_e10402_d_n14;

        let assign6770_e10405: f64 = if p.p434 != 0.0 { 1.0 } else { 0.0 };
        var_guard61 = assign6770_e10405;

        let (assign6780_e10442, assign6780_e10442_d_n0, assign6780_e10442_d_n2, assign6780_e10442_d_n3, assign6780_e10442_d_n4, assign6780_e10442_d_n5, assign6780_e10442_d_n6, assign6780_e10442_d_n7, assign6780_e10442_d_n8, assign6780_e10442_d_n9, assign6780_e10442_d_n10, assign6780_e10442_d_n11, assign6780_e10442_d_n13, assign6780_e10442_d_n14,) = {
    if (var_guard61 != 0.0) {
        let assign6780_e10411: f64 = (p.p434 / p.p5);
        let assign6780_e10415: f64 = (p.p5 / p.p435);
        let assign6780_e10416: f64 = (1.0 + assign6780_e10415);
        let (assign6780_e10437,) = {
            if (!(assign6780_e10416 > 1e-38)) {
                let assign6780_e10421: f64 = (-87.498233534);
                (assign6780_e10421,)
            } else {
                let assign6780_e10425: f64 = (p.p5 / p.p435);
                let assign6780_e10426: f64 = (1.0 + assign6780_e10425);
                let (assign6780_e10436,) = {
                    if (assign6780_e10426 > 1e-38) {
                        let assign6780_e10432: f64 = (p.p5 / p.p435);
                        let assign6780_e10433: f64 = (1.0 + assign6780_e10432);
                        let assign6780_e10434: f64 = (assign6780_e10433).ln();
                        (assign6780_e10434,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6780_e10436,)
            }
        };
        let assign6780_e10438: f64 = (assign6780_e10411 * assign6780_e10437);
        let assign6780_e10439: f64 = (1.0 + assign6780_e10438);
        let assign6780_e10440: f64 = (var_vsat1r_i * assign6780_e10439);
        (assign6780_e10440, (var_vsat1r_i_dn0 * assign6780_e10439), (var_vsat1r_i_dn2 * assign6780_e10439), (var_vsat1r_i_dn3 * assign6780_e10439), (var_vsat1r_i_dn4 * assign6780_e10439), (var_vsat1r_i_dn5 * assign6780_e10439), (var_vsat1r_i_dn6 * assign6780_e10439), (var_vsat1r_i_dn7 * assign6780_e10439), (var_vsat1r_i_dn8 * assign6780_e10439), (var_vsat1r_i_dn9 * assign6780_e10439), (var_vsat1r_i_dn10 * assign6780_e10439), (var_vsat1r_i_dn11 * assign6780_e10439), (var_vsat1r_i_dn13 * assign6780_e10439), (var_vsat1r_i_dn14 * assign6780_e10439),)
    } else {
        (var_vsat1r_i, var_vsat1r_i_dn0, var_vsat1r_i_dn2, var_vsat1r_i_dn3, var_vsat1r_i_dn4, var_vsat1r_i_dn5, var_vsat1r_i_dn6, var_vsat1r_i_dn7, var_vsat1r_i_dn8, var_vsat1r_i_dn9, var_vsat1r_i_dn10, var_vsat1r_i_dn11, var_vsat1r_i_dn13, var_vsat1r_i_dn14,)
    }
};
        var_vsat1r_i = assign6780_e10442;
        var_vsat1r_i_dn0 = assign6780_e10442_d_n0;
        var_vsat1r_i_dn2 = assign6780_e10442_d_n2;
        var_vsat1r_i_dn3 = assign6780_e10442_d_n3;
        var_vsat1r_i_dn4 = assign6780_e10442_d_n4;
        var_vsat1r_i_dn5 = assign6780_e10442_d_n5;
        var_vsat1r_i_dn6 = assign6780_e10442_d_n6;
        var_vsat1r_i_dn7 = assign6780_e10442_d_n7;
        var_vsat1r_i_dn8 = assign6780_e10442_d_n8;
        var_vsat1r_i_dn9 = assign6780_e10442_d_n9;
        var_vsat1r_i_dn10 = assign6780_e10442_d_n10;
        var_vsat1r_i_dn11 = assign6780_e10442_d_n11;
        var_vsat1r_i_dn13 = assign6780_e10442_d_n13;
        var_vsat1r_i_dn14 = assign6780_e10442_d_n14;

        let assign6790_e10445: f64 = if p.p581 != 0.0 { 1.0 } else { 0.0 };
        var_guard62 = assign6790_e10445;

        let (assign6800_e10482, assign6800_e10482_d_n0, assign6800_e10482_d_n2, assign6800_e10482_d_n3, assign6800_e10482_d_n4, assign6800_e10482_d_n5, assign6800_e10482_d_n6, assign6800_e10482_d_n7, assign6800_e10482_d_n8, assign6800_e10482_d_n9, assign6800_e10482_d_n10, assign6800_e10482_d_n11, assign6800_e10482_d_n13, assign6800_e10482_d_n14,) = {
    if (var_guard62 != 0.0) {
        let assign6800_e10451: f64 = (p.p581 / p.p5);
        let assign6800_e10455: f64 = (p.p5 / p.p584);
        let assign6800_e10456: f64 = (1.0 + assign6800_e10455);
        let (assign6800_e10477,) = {
            if (!(assign6800_e10456 > 1e-38)) {
                let assign6800_e10461: f64 = (-87.498233534);
                (assign6800_e10461,)
            } else {
                let assign6800_e10465: f64 = (p.p5 / p.p584);
                let assign6800_e10466: f64 = (1.0 + assign6800_e10465);
                let (assign6800_e10476,) = {
                    if (assign6800_e10466 > 1e-38) {
                        let assign6800_e10472: f64 = (p.p5 / p.p584);
                        let assign6800_e10473: f64 = (1.0 + assign6800_e10472);
                        let assign6800_e10474: f64 = (assign6800_e10473).ln();
                        (assign6800_e10474,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6800_e10476,)
            }
        };
        let assign6800_e10478: f64 = (assign6800_e10451 * assign6800_e10477);
        let assign6800_e10479: f64 = (1.0 + assign6800_e10478);
        let assign6800_e10480: f64 = (var_u0_i * assign6800_e10479);
        (assign6800_e10480, (var_u0_i_dn0 * assign6800_e10479), (var_u0_i_dn2 * assign6800_e10479), (var_u0_i_dn3 * assign6800_e10479), (var_u0_i_dn4 * assign6800_e10479), (var_u0_i_dn5 * assign6800_e10479), (var_u0_i_dn6 * assign6800_e10479), (var_u0_i_dn7 * assign6800_e10479), (var_u0_i_dn8 * assign6800_e10479), (var_u0_i_dn9 * assign6800_e10479), (var_u0_i_dn10 * assign6800_e10479), (var_u0_i_dn11 * assign6800_e10479), (var_u0_i_dn13 * assign6800_e10479), (var_u0_i_dn14 * assign6800_e10479),)
    } else {
        (var_u0_i, var_u0_i_dn0, var_u0_i_dn2, var_u0_i_dn3, var_u0_i_dn4, var_u0_i_dn5, var_u0_i_dn6, var_u0_i_dn7, var_u0_i_dn8, var_u0_i_dn9, var_u0_i_dn10, var_u0_i_dn11, var_u0_i_dn13, var_u0_i_dn14,)
    }
};
        var_u0_i = assign6800_e10482;
        var_u0_i_dn0 = assign6800_e10482_d_n0;
        var_u0_i_dn2 = assign6800_e10482_d_n2;
        var_u0_i_dn3 = assign6800_e10482_d_n3;
        var_u0_i_dn4 = assign6800_e10482_d_n4;
        var_u0_i_dn5 = assign6800_e10482_d_n5;
        var_u0_i_dn6 = assign6800_e10482_d_n6;
        var_u0_i_dn7 = assign6800_e10482_d_n7;
        var_u0_i_dn8 = assign6800_e10482_d_n8;
        var_u0_i_dn9 = assign6800_e10482_d_n9;
        var_u0_i_dn10 = assign6800_e10482_d_n10;
        var_u0_i_dn11 = assign6800_e10482_d_n11;
        var_u0_i_dn13 = assign6800_e10482_d_n13;
        var_u0_i_dn14 = assign6800_e10482_d_n14;

        let assign6810_e10485: f64 = if p.p583 != 0.0 { 1.0 } else { 0.0 };
        var_guard63 = assign6810_e10485;

        let (assign6820_e10522, assign6820_e10522_d_n0, assign6820_e10522_d_n2, assign6820_e10522_d_n3, assign6820_e10522_d_n4, assign6820_e10522_d_n5, assign6820_e10522_d_n6, assign6820_e10522_d_n7, assign6820_e10522_d_n8, assign6820_e10522_d_n9, assign6820_e10522_d_n10, assign6820_e10522_d_n11, assign6820_e10522_d_n13, assign6820_e10522_d_n14,) = {
    if (var_guard63 != 0.0) {
        let assign6820_e10491: f64 = (p.p583 / p.p5);
        let assign6820_e10495: f64 = (p.p5 / p.p586);
        let assign6820_e10496: f64 = (1.0 + assign6820_e10495);
        let (assign6820_e10517,) = {
            if (!(assign6820_e10496 > 1e-38)) {
                let assign6820_e10501: f64 = (-87.498233534);
                (assign6820_e10501,)
            } else {
                let assign6820_e10505: f64 = (p.p5 / p.p586);
                let assign6820_e10506: f64 = (1.0 + assign6820_e10505);
                let (assign6820_e10516,) = {
                    if (assign6820_e10506 > 1e-38) {
                        let assign6820_e10512: f64 = (p.p5 / p.p586);
                        let assign6820_e10513: f64 = (1.0 + assign6820_e10512);
                        let assign6820_e10514: f64 = (assign6820_e10513).ln();
                        (assign6820_e10514,)
                    } else {
                        (0.0,)
                    }
                };
                (assign6820_e10516,)
            }
        };
        let assign6820_e10518: f64 = (assign6820_e10491 * assign6820_e10517);
        let assign6820_e10519: f64 = (1.0 + assign6820_e10518);
        let assign6820_e10520: f64 = (var_u0r_i * assign6820_e10519);
        (assign6820_e10520, (var_u0r_i_dn0 * assign6820_e10519), (var_u0r_i_dn2 * assign6820_e10519), (var_u0r_i_dn3 * assign6820_e10519), (var_u0r_i_dn4 * assign6820_e10519), (var_u0r_i_dn5 * assign6820_e10519), (var_u0r_i_dn6 * assign6820_e10519), (var_u0r_i_dn7 * assign6820_e10519), (var_u0r_i_dn8 * assign6820_e10519), (var_u0r_i_dn9 * assign6820_e10519), (var_u0r_i_dn10 * assign6820_e10519), (var_u0r_i_dn11 * assign6820_e10519), (var_u0r_i_dn13 * assign6820_e10519), (var_u0r_i_dn14 * assign6820_e10519),)
    } else {
        (var_u0r_i, var_u0r_i_dn0, var_u0r_i_dn2, var_u0r_i_dn3, var_u0r_i_dn4, var_u0r_i_dn5, var_u0r_i_dn6, var_u0r_i_dn7, var_u0r_i_dn8, var_u0r_i_dn9, var_u0r_i_dn10, var_u0r_i_dn11, var_u0r_i_dn13, var_u0r_i_dn14,)
    }
};
        var_u0r_i = assign6820_e10522;
        var_u0r_i_dn0 = assign6820_e10522_d_n0;
        var_u0r_i_dn2 = assign6820_e10522_d_n2;
        var_u0r_i_dn3 = assign6820_e10522_d_n3;
        var_u0r_i_dn4 = assign6820_e10522_d_n4;
        var_u0r_i_dn5 = assign6820_e10522_d_n5;
        var_u0r_i_dn6 = assign6820_e10522_d_n6;
        var_u0r_i_dn7 = assign6820_e10522_d_n7;
        var_u0r_i_dn8 = assign6820_e10522_d_n8;
        var_u0r_i_dn9 = assign6820_e10522_d_n9;
        var_u0r_i_dn10 = assign6820_e10522_d_n10;
        var_u0r_i_dn11 = assign6820_e10522_d_n11;
        var_u0r_i_dn13 = assign6820_e10522_d_n13;
        var_u0r_i_dn14 = assign6820_e10522_d_n14;

        let assign6830_e10525: f64 = if p.p21 != 0.0 { 1.0 } else { 0.0 };
        var_guard64 = assign6830_e10525;

        let (assign6840_e10539, assign6840_e10539_d_n0, assign6840_e10539_d_n2, assign6840_e10539_d_n3, assign6840_e10539_d_n4, assign6840_e10539_d_n5, assign6840_e10539_d_n6, assign6840_e10539_d_n7, assign6840_e10539_d_n8, assign6840_e10539_d_n9, assign6840_e10539_d_n10, assign6840_e10539_d_n11, assign6840_e10539_d_n13, assign6840_e10539_d_n14,) = {
    if (var_guard64 != 0.0) {
        let assign6840_e10531: f64 = (p.p5 - p.p21);
        let assign6840_e10533: f64 = (assign6840_e10531 * p.p99);
        let assign6840_e10535: f64 = (assign6840_e10533 * var_leff_1);
        let assign6840_e10536: f64 = (1.0 + assign6840_e10535);
        let assign6840_e10537: f64 = (var_phig_i * assign6840_e10536);
        (assign6840_e10537, ((var_phig_i_dn0 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn0))), ((var_phig_i_dn2 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn2))), ((var_phig_i_dn3 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn3))), ((var_phig_i_dn4 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn4))), ((var_phig_i_dn5 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn5))), ((var_phig_i_dn6 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn6))), ((var_phig_i_dn7 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn7))), ((var_phig_i_dn8 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn8))), ((var_phig_i_dn9 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn9))), ((var_phig_i_dn10 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn10))), ((var_phig_i_dn11 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn11))), ((var_phig_i_dn13 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn13))), ((var_phig_i_dn14 * assign6840_e10536) + (var_phig_i * (assign6840_e10533 * var_leff_1_dn14))),)
    } else {
        (var_phig_i, var_phig_i_dn0, var_phig_i_dn2, var_phig_i_dn3, var_phig_i_dn4, var_phig_i_dn5, var_phig_i_dn6, var_phig_i_dn7, var_phig_i_dn8, var_phig_i_dn9, var_phig_i_dn10, var_phig_i_dn11, var_phig_i_dn13, var_phig_i_dn14,)
    }
};
        var_phig_i = assign6840_e10539;
        var_phig_i_dn0 = assign6840_e10539_d_n0;
        var_phig_i_dn2 = assign6840_e10539_d_n2;
        var_phig_i_dn3 = assign6840_e10539_d_n3;
        var_phig_i_dn4 = assign6840_e10539_d_n4;
        var_phig_i_dn5 = assign6840_e10539_d_n5;
        var_phig_i_dn6 = assign6840_e10539_d_n6;
        var_phig_i_dn7 = assign6840_e10539_d_n7;
        var_phig_i_dn8 = assign6840_e10539_d_n8;
        var_phig_i_dn9 = assign6840_e10539_d_n9;
        var_phig_i_dn10 = assign6840_e10539_d_n10;
        var_phig_i_dn11 = assign6840_e10539_d_n11;
        var_phig_i_dn13 = assign6840_e10539_d_n13;
        var_phig_i_dn14 = assign6840_e10539_d_n14;

        *var_cdsc_i_slot = var_cdsc_i;
        *var_cdscd_i_slot = var_cdscd_i;
        *var_cdscdr_i_slot = var_cdscdr_i;
        *var_eta0_i_slot = var_eta0_i;
        *var_eta0_i_dn0_slot = var_eta0_i_dn0;
        *var_eta0_i_dn10_slot = var_eta0_i_dn10;
        *var_eta0_i_dn11_slot = var_eta0_i_dn11;
        *var_eta0_i_dn13_slot = var_eta0_i_dn13;
        *var_eta0_i_dn14_slot = var_eta0_i_dn14;
        *var_eta0_i_dn2_slot = var_eta0_i_dn2;
        *var_eta0_i_dn3_slot = var_eta0_i_dn3;
        *var_eta0_i_dn4_slot = var_eta0_i_dn4;
        *var_eta0_i_dn5_slot = var_eta0_i_dn5;
        *var_eta0_i_dn6_slot = var_eta0_i_dn6;
        *var_eta0_i_dn7_slot = var_eta0_i_dn7;
        *var_eta0_i_dn8_slot = var_eta0_i_dn8;
        *var_eta0_i_dn9_slot = var_eta0_i_dn9;
        *var_guard55_slot = var_guard55;
        *var_guard56_slot = var_guard56;
        *var_guard57_slot = var_guard57;
        *var_guard58_slot = var_guard58;
        *var_guard59_slot = var_guard59;
        *var_guard60_slot = var_guard60;
        *var_guard61_slot = var_guard61;
        *var_guard62_slot = var_guard62;
        *var_guard63_slot = var_guard63;
        *var_guard64_slot = var_guard64;
        *var_phig_i_slot = var_phig_i;
        *var_phig_i_dn0_slot = var_phig_i_dn0;
        *var_phig_i_dn10_slot = var_phig_i_dn10;
        *var_phig_i_dn11_slot = var_phig_i_dn11;
        *var_phig_i_dn13_slot = var_phig_i_dn13;
        *var_phig_i_dn14_slot = var_phig_i_dn14;
        *var_phig_i_dn2_slot = var_phig_i_dn2;
        *var_phig_i_dn3_slot = var_phig_i_dn3;
        *var_phig_i_dn4_slot = var_phig_i_dn4;
        *var_phig_i_dn5_slot = var_phig_i_dn5;
        *var_phig_i_dn6_slot = var_phig_i_dn6;
        *var_phig_i_dn7_slot = var_phig_i_dn7;
        *var_phig_i_dn8_slot = var_phig_i_dn8;
        *var_phig_i_dn9_slot = var_phig_i_dn9;
        *var_u0_i_slot = var_u0_i;
        *var_u0_i_dn0_slot = var_u0_i_dn0;
        *var_u0_i_dn10_slot = var_u0_i_dn10;
        *var_u0_i_dn11_slot = var_u0_i_dn11;
        *var_u0_i_dn13_slot = var_u0_i_dn13;
        *var_u0_i_dn14_slot = var_u0_i_dn14;
        *var_u0_i_dn2_slot = var_u0_i_dn2;
        *var_u0_i_dn3_slot = var_u0_i_dn3;
        *var_u0_i_dn4_slot = var_u0_i_dn4;
        *var_u0_i_dn5_slot = var_u0_i_dn5;
        *var_u0_i_dn6_slot = var_u0_i_dn6;
        *var_u0_i_dn7_slot = var_u0_i_dn7;
        *var_u0_i_dn8_slot = var_u0_i_dn8;
        *var_u0_i_dn9_slot = var_u0_i_dn9;
        *var_u0r_i_slot = var_u0r_i;
        *var_u0r_i_dn0_slot = var_u0r_i_dn0;
        *var_u0r_i_dn10_slot = var_u0r_i_dn10;
        *var_u0r_i_dn11_slot = var_u0r_i_dn11;
        *var_u0r_i_dn13_slot = var_u0r_i_dn13;
        *var_u0r_i_dn14_slot = var_u0r_i_dn14;
        *var_u0r_i_dn2_slot = var_u0r_i_dn2;
        *var_u0r_i_dn3_slot = var_u0r_i_dn3;
        *var_u0r_i_dn4_slot = var_u0r_i_dn4;
        *var_u0r_i_dn5_slot = var_u0r_i_dn5;
        *var_u0r_i_dn6_slot = var_u0r_i_dn6;
        *var_u0r_i_dn7_slot = var_u0r_i_dn7;
        *var_u0r_i_dn8_slot = var_u0r_i_dn8;
        *var_u0r_i_dn9_slot = var_u0r_i_dn9;
        *var_vsat1_i_slot = var_vsat1_i;
        *var_vsat1_i_dn0_slot = var_vsat1_i_dn0;
        *var_vsat1_i_dn10_slot = var_vsat1_i_dn10;
        *var_vsat1_i_dn11_slot = var_vsat1_i_dn11;
        *var_vsat1_i_dn13_slot = var_vsat1_i_dn13;
        *var_vsat1_i_dn14_slot = var_vsat1_i_dn14;
        *var_vsat1_i_dn2_slot = var_vsat1_i_dn2;
        *var_vsat1_i_dn3_slot = var_vsat1_i_dn3;
        *var_vsat1_i_dn4_slot = var_vsat1_i_dn4;
        *var_vsat1_i_dn5_slot = var_vsat1_i_dn5;
        *var_vsat1_i_dn6_slot = var_vsat1_i_dn6;
        *var_vsat1_i_dn7_slot = var_vsat1_i_dn7;
        *var_vsat1_i_dn8_slot = var_vsat1_i_dn8;
        *var_vsat1_i_dn9_slot = var_vsat1_i_dn9;
        *var_vsat1r_i_slot = var_vsat1r_i;
        *var_vsat1r_i_dn0_slot = var_vsat1r_i_dn0;
        *var_vsat1r_i_dn10_slot = var_vsat1r_i_dn10;
        *var_vsat1r_i_dn11_slot = var_vsat1r_i_dn11;
        *var_vsat1r_i_dn13_slot = var_vsat1r_i_dn13;
        *var_vsat1r_i_dn14_slot = var_vsat1r_i_dn14;
        *var_vsat1r_i_dn2_slot = var_vsat1r_i_dn2;
        *var_vsat1r_i_dn3_slot = var_vsat1r_i_dn3;
        *var_vsat1r_i_dn4_slot = var_vsat1r_i_dn4;
        *var_vsat1r_i_dn5_slot = var_vsat1r_i_dn5;
        *var_vsat1r_i_dn6_slot = var_vsat1r_i_dn6;
        *var_vsat1r_i_dn7_slot = var_vsat1r_i_dn7;
        *var_vsat1r_i_dn8_slot = var_vsat1r_i_dn8;
        *var_vsat1r_i_dn9_slot = var_vsat1r_i_dn9;
        *var_vsat_i_slot = var_vsat_i;
        *var_vsat_i_dn0_slot = var_vsat_i_dn0;
        *var_vsat_i_dn10_slot = var_vsat_i_dn10;
        *var_vsat_i_dn11_slot = var_vsat_i_dn11;
        *var_vsat_i_dn13_slot = var_vsat_i_dn13;
        *var_vsat_i_dn14_slot = var_vsat_i_dn14;
        *var_vsat_i_dn2_slot = var_vsat_i_dn2;
        *var_vsat_i_dn3_slot = var_vsat_i_dn3;
        *var_vsat_i_dn4_slot = var_vsat_i_dn4;
        *var_vsat_i_dn5_slot = var_vsat_i_dn5;
        *var_vsat_i_dn6_slot = var_vsat_i_dn6;
        *var_vsat_i_dn7_slot = var_vsat_i_dn7;
        *var_vsat_i_dn8_slot = var_vsat_i_dn8;
        *var_vsat_i_dn9_slot = var_vsat_i_dn9;
    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        var_guard64: f64,
        var_leff_1: f64,
        var_leff_1_dn0: f64,
        var_leff_1_dn10: f64,
        var_leff_1_dn11: f64,
        var_leff_1_dn13: f64,
        var_leff_1_dn14: f64,
        var_leff_1_dn2: f64,
        var_leff_1_dn3: f64,
        var_leff_1_dn4: f64,
        var_leff_1_dn5: f64,
        var_leff_1_dn6: f64,
        var_leff_1_dn7: f64,
        var_leff_1_dn8: f64,
        var_leff_1_dn9: f64,
        var_up_i: f64,
        var_upr_i: f64,
        var_eta0_i_slot: &mut f64,
        var_eta0_i_dn0_slot: &mut f64,
        var_eta0_i_dn10_slot: &mut f64,
        var_eta0_i_dn11_slot: &mut f64,
        var_eta0_i_dn13_slot: &mut f64,
        var_eta0_i_dn14_slot: &mut f64,
        var_eta0_i_dn2_slot: &mut f64,
        var_eta0_i_dn3_slot: &mut f64,
        var_eta0_i_dn4_slot: &mut f64,
        var_eta0_i_dn5_slot: &mut f64,
        var_eta0_i_dn6_slot: &mut f64,
        var_eta0_i_dn7_slot: &mut f64,
        var_eta0_i_dn8_slot: &mut f64,
        var_eta0_i_dn9_slot: &mut f64,
        var_eu_i_slot: &mut f64,
        var_eu_i_dn0_slot: &mut f64,
        var_eu_i_dn10_slot: &mut f64,
        var_eu_i_dn11_slot: &mut f64,
        var_eu_i_dn13_slot: &mut f64,
        var_eu_i_dn14_slot: &mut f64,
        var_eu_i_dn2_slot: &mut f64,
        var_eu_i_dn3_slot: &mut f64,
        var_eu_i_dn4_slot: &mut f64,
        var_eu_i_dn5_slot: &mut f64,
        var_eu_i_dn6_slot: &mut f64,
        var_eu_i_dn7_slot: &mut f64,
        var_eu_i_dn8_slot: &mut f64,
        var_eu_i_dn9_slot: &mut f64,
        var_eur_i_slot: &mut f64,
        var_eur_i_dn0_slot: &mut f64,
        var_eur_i_dn10_slot: &mut f64,
        var_eur_i_dn11_slot: &mut f64,
        var_eur_i_dn13_slot: &mut f64,
        var_eur_i_dn14_slot: &mut f64,
        var_eur_i_dn2_slot: &mut f64,
        var_eur_i_dn3_slot: &mut f64,
        var_eur_i_dn4_slot: &mut f64,
        var_eur_i_dn5_slot: &mut f64,
        var_eur_i_dn6_slot: &mut f64,
        var_eur_i_dn7_slot: &mut f64,
        var_eur_i_dn8_slot: &mut f64,
        var_eur_i_dn9_slot: &mut f64,
        var_guard65_slot: &mut f64,
        var_guard66_slot: &mut f64,
        var_guard67_slot: &mut f64,
        var_guard68_slot: &mut f64,
        var_guard69_slot: &mut f64,
        var_leff_ln_slot: &mut f64,
        var_leff_ln_dn0_slot: &mut f64,
        var_leff_ln_dn10_slot: &mut f64,
        var_leff_ln_dn11_slot: &mut f64,
        var_leff_ln_dn13_slot: &mut f64,
        var_leff_ln_dn14_slot: &mut f64,
        var_leff_ln_dn2_slot: &mut f64,
        var_leff_ln_dn3_slot: &mut f64,
        var_leff_ln_dn4_slot: &mut f64,
        var_leff_ln_dn5_slot: &mut f64,
        var_leff_ln_dn6_slot: &mut f64,
        var_leff_ln_dn7_slot: &mut f64,
        var_leff_ln_dn8_slot: &mut f64,
        var_leff_ln_dn9_slot: &mut f64,
        var_mexp_i_slot: &mut f64,
        var_mexp_i_dn0_slot: &mut f64,
        var_mexp_i_dn10_slot: &mut f64,
        var_mexp_i_dn11_slot: &mut f64,
        var_mexp_i_dn13_slot: &mut f64,
        var_mexp_i_dn14_slot: &mut f64,
        var_mexp_i_dn2_slot: &mut f64,
        var_mexp_i_dn3_slot: &mut f64,
        var_mexp_i_dn4_slot: &mut f64,
        var_mexp_i_dn5_slot: &mut f64,
        var_mexp_i_dn6_slot: &mut f64,
        var_mexp_i_dn7_slot: &mut f64,
        var_mexp_i_dn8_slot: &mut f64,
        var_mexp_i_dn9_slot: &mut f64,
        var_pclm_i_slot: &mut f64,
        var_pclm_i_dn0_slot: &mut f64,
        var_pclm_i_dn10_slot: &mut f64,
        var_pclm_i_dn11_slot: &mut f64,
        var_pclm_i_dn13_slot: &mut f64,
        var_pclm_i_dn14_slot: &mut f64,
        var_pclm_i_dn2_slot: &mut f64,
        var_pclm_i_dn3_slot: &mut f64,
        var_pclm_i_dn4_slot: &mut f64,
        var_pclm_i_dn5_slot: &mut f64,
        var_pclm_i_dn6_slot: &mut f64,
        var_pclm_i_dn7_slot: &mut f64,
        var_pclm_i_dn8_slot: &mut f64,
        var_pclm_i_dn9_slot: &mut f64,
        var_pclmr_i_slot: &mut f64,
        var_pclmr_i_dn0_slot: &mut f64,
        var_pclmr_i_dn10_slot: &mut f64,
        var_pclmr_i_dn11_slot: &mut f64,
        var_pclmr_i_dn13_slot: &mut f64,
        var_pclmr_i_dn14_slot: &mut f64,
        var_pclmr_i_dn2_slot: &mut f64,
        var_pclmr_i_dn3_slot: &mut f64,
        var_pclmr_i_dn4_slot: &mut f64,
        var_pclmr_i_dn5_slot: &mut f64,
        var_pclmr_i_dn6_slot: &mut f64,
        var_pclmr_i_dn7_slot: &mut f64,
        var_pclmr_i_dn8_slot: &mut f64,
        var_pclmr_i_dn9_slot: &mut f64,
        var_phig_i_slot: &mut f64,
        var_phig_i_dn0_slot: &mut f64,
        var_phig_i_dn10_slot: &mut f64,
        var_phig_i_dn11_slot: &mut f64,
        var_phig_i_dn13_slot: &mut f64,
        var_phig_i_dn14_slot: &mut f64,
        var_phig_i_dn2_slot: &mut f64,
        var_phig_i_dn3_slot: &mut f64,
        var_phig_i_dn4_slot: &mut f64,
        var_phig_i_dn5_slot: &mut f64,
        var_phig_i_dn6_slot: &mut f64,
        var_phig_i_dn7_slot: &mut f64,
        var_phig_i_dn8_slot: &mut f64,
        var_phig_i_dn9_slot: &mut f64,
        var_pqm_i_slot: &mut f64,
        var_pqm_i_dn0_slot: &mut f64,
        var_pqm_i_dn10_slot: &mut f64,
        var_pqm_i_dn11_slot: &mut f64,
        var_pqm_i_dn13_slot: &mut f64,
        var_pqm_i_dn14_slot: &mut f64,
        var_pqm_i_dn2_slot: &mut f64,
        var_pqm_i_dn3_slot: &mut f64,
        var_pqm_i_dn4_slot: &mut f64,
        var_pqm_i_dn5_slot: &mut f64,
        var_pqm_i_dn6_slot: &mut f64,
        var_pqm_i_dn7_slot: &mut f64,
        var_pqm_i_dn8_slot: &mut f64,
        var_pqm_i_dn9_slot: &mut f64,
        var_rdsw_i_slot: &mut f64,
        var_rdsw_i_dn0_slot: &mut f64,
        var_rdsw_i_dn10_slot: &mut f64,
        var_rdsw_i_dn11_slot: &mut f64,
        var_rdsw_i_dn13_slot: &mut f64,
        var_rdsw_i_dn14_slot: &mut f64,
        var_rdsw_i_dn2_slot: &mut f64,
        var_rdsw_i_dn3_slot: &mut f64,
        var_rdsw_i_dn4_slot: &mut f64,
        var_rdsw_i_dn5_slot: &mut f64,
        var_rdsw_i_dn6_slot: &mut f64,
        var_rdsw_i_dn7_slot: &mut f64,
        var_rdsw_i_dn8_slot: &mut f64,
        var_rdsw_i_dn9_slot: &mut f64,
        var_rdw_i_slot: &mut f64,
        var_rdw_i_dn0_slot: &mut f64,
        var_rdw_i_dn10_slot: &mut f64,
        var_rdw_i_dn11_slot: &mut f64,
        var_rdw_i_dn13_slot: &mut f64,
        var_rdw_i_dn14_slot: &mut f64,
        var_rdw_i_dn2_slot: &mut f64,
        var_rdw_i_dn3_slot: &mut f64,
        var_rdw_i_dn4_slot: &mut f64,
        var_rdw_i_dn5_slot: &mut f64,
        var_rdw_i_dn6_slot: &mut f64,
        var_rdw_i_dn7_slot: &mut f64,
        var_rdw_i_dn8_slot: &mut f64,
        var_rdw_i_dn9_slot: &mut f64,
        var_rsw_i_slot: &mut f64,
        var_rsw_i_dn0_slot: &mut f64,
        var_rsw_i_dn10_slot: &mut f64,
        var_rsw_i_dn11_slot: &mut f64,
        var_rsw_i_dn13_slot: &mut f64,
        var_rsw_i_dn14_slot: &mut f64,
        var_rsw_i_dn2_slot: &mut f64,
        var_rsw_i_dn3_slot: &mut f64,
        var_rsw_i_dn4_slot: &mut f64,
        var_rsw_i_dn5_slot: &mut f64,
        var_rsw_i_dn6_slot: &mut f64,
        var_rsw_i_dn7_slot: &mut f64,
        var_rsw_i_dn8_slot: &mut f64,
        var_rsw_i_dn9_slot: &mut f64,
        var_u0_i_slot: &mut f64,
        var_u0_i_dn0_slot: &mut f64,
        var_u0_i_dn10_slot: &mut f64,
        var_u0_i_dn11_slot: &mut f64,
        var_u0_i_dn13_slot: &mut f64,
        var_u0_i_dn14_slot: &mut f64,
        var_u0_i_dn2_slot: &mut f64,
        var_u0_i_dn3_slot: &mut f64,
        var_u0_i_dn4_slot: &mut f64,
        var_u0_i_dn5_slot: &mut f64,
        var_u0_i_dn6_slot: &mut f64,
        var_u0_i_dn7_slot: &mut f64,
        var_u0_i_dn8_slot: &mut f64,
        var_u0_i_dn9_slot: &mut f64,
        var_u0r_i_slot: &mut f64,
        var_u0r_i_dn0_slot: &mut f64,
        var_u0r_i_dn10_slot: &mut f64,
        var_u0r_i_dn11_slot: &mut f64,
        var_u0r_i_dn13_slot: &mut f64,
        var_u0r_i_dn14_slot: &mut f64,
        var_u0r_i_dn2_slot: &mut f64,
        var_u0r_i_dn3_slot: &mut f64,
        var_u0r_i_dn4_slot: &mut f64,
        var_u0r_i_dn5_slot: &mut f64,
        var_u0r_i_dn6_slot: &mut f64,
        var_u0r_i_dn7_slot: &mut f64,
        var_u0r_i_dn8_slot: &mut f64,
        var_u0r_i_dn9_slot: &mut f64,
        var_ua_i_slot: &mut f64,
        var_ua_i_dn0_slot: &mut f64,
        var_ua_i_dn10_slot: &mut f64,
        var_ua_i_dn11_slot: &mut f64,
        var_ua_i_dn13_slot: &mut f64,
        var_ua_i_dn14_slot: &mut f64,
        var_ua_i_dn2_slot: &mut f64,
        var_ua_i_dn3_slot: &mut f64,
        var_ua_i_dn4_slot: &mut f64,
        var_ua_i_dn5_slot: &mut f64,
        var_ua_i_dn6_slot: &mut f64,
        var_ua_i_dn7_slot: &mut f64,
        var_ua_i_dn8_slot: &mut f64,
        var_ua_i_dn9_slot: &mut f64,
        var_uar_i_slot: &mut f64,
        var_uar_i_dn0_slot: &mut f64,
        var_uar_i_dn10_slot: &mut f64,
        var_uar_i_dn11_slot: &mut f64,
        var_uar_i_dn13_slot: &mut f64,
        var_uar_i_dn14_slot: &mut f64,
        var_uar_i_dn2_slot: &mut f64,
        var_uar_i_dn3_slot: &mut f64,
        var_uar_i_dn4_slot: &mut f64,
        var_uar_i_dn5_slot: &mut f64,
        var_uar_i_dn6_slot: &mut f64,
        var_uar_i_dn7_slot: &mut f64,
        var_uar_i_dn8_slot: &mut f64,
        var_uar_i_dn9_slot: &mut f64,
        var_ud_i_slot: &mut f64,
        var_ud_i_dn0_slot: &mut f64,
        var_ud_i_dn10_slot: &mut f64,
        var_ud_i_dn11_slot: &mut f64,
        var_ud_i_dn13_slot: &mut f64,
        var_ud_i_dn14_slot: &mut f64,
        var_ud_i_dn2_slot: &mut f64,
        var_ud_i_dn3_slot: &mut f64,
        var_ud_i_dn4_slot: &mut f64,
        var_ud_i_dn5_slot: &mut f64,
        var_ud_i_dn6_slot: &mut f64,
        var_ud_i_dn7_slot: &mut f64,
        var_ud_i_dn8_slot: &mut f64,
        var_ud_i_dn9_slot: &mut f64,
        var_udr_i_slot: &mut f64,
        var_udr_i_dn0_slot: &mut f64,
        var_udr_i_dn10_slot: &mut f64,
        var_udr_i_dn11_slot: &mut f64,
        var_udr_i_dn13_slot: &mut f64,
        var_udr_i_dn14_slot: &mut f64,
        var_udr_i_dn2_slot: &mut f64,
        var_udr_i_dn3_slot: &mut f64,
        var_udr_i_dn4_slot: &mut f64,
        var_udr_i_dn5_slot: &mut f64,
        var_udr_i_dn6_slot: &mut f64,
        var_udr_i_dn7_slot: &mut f64,
        var_udr_i_dn8_slot: &mut f64,
        var_udr_i_dn9_slot: &mut f64,
    ) {
        let mut var_eta0_i: f64 = *var_eta0_i_slot;
        let mut var_eta0_i_dn0: f64 = *var_eta0_i_dn0_slot;
        let mut var_eta0_i_dn10: f64 = *var_eta0_i_dn10_slot;
        let mut var_eta0_i_dn11: f64 = *var_eta0_i_dn11_slot;
        let mut var_eta0_i_dn13: f64 = *var_eta0_i_dn13_slot;
        let mut var_eta0_i_dn14: f64 = *var_eta0_i_dn14_slot;
        let mut var_eta0_i_dn2: f64 = *var_eta0_i_dn2_slot;
        let mut var_eta0_i_dn3: f64 = *var_eta0_i_dn3_slot;
        let mut var_eta0_i_dn4: f64 = *var_eta0_i_dn4_slot;
        let mut var_eta0_i_dn5: f64 = *var_eta0_i_dn5_slot;
        let mut var_eta0_i_dn6: f64 = *var_eta0_i_dn6_slot;
        let mut var_eta0_i_dn7: f64 = *var_eta0_i_dn7_slot;
        let mut var_eta0_i_dn8: f64 = *var_eta0_i_dn8_slot;
        let mut var_eta0_i_dn9: f64 = *var_eta0_i_dn9_slot;
        let mut var_eu_i: f64 = *var_eu_i_slot;
        let mut var_eu_i_dn0: f64 = *var_eu_i_dn0_slot;
        let mut var_eu_i_dn10: f64 = *var_eu_i_dn10_slot;
        let mut var_eu_i_dn11: f64 = *var_eu_i_dn11_slot;
        let mut var_eu_i_dn13: f64 = *var_eu_i_dn13_slot;
        let mut var_eu_i_dn14: f64 = *var_eu_i_dn14_slot;
        let mut var_eu_i_dn2: f64 = *var_eu_i_dn2_slot;
        let mut var_eu_i_dn3: f64 = *var_eu_i_dn3_slot;
        let mut var_eu_i_dn4: f64 = *var_eu_i_dn4_slot;
        let mut var_eu_i_dn5: f64 = *var_eu_i_dn5_slot;
        let mut var_eu_i_dn6: f64 = *var_eu_i_dn6_slot;
        let mut var_eu_i_dn7: f64 = *var_eu_i_dn7_slot;
        let mut var_eu_i_dn8: f64 = *var_eu_i_dn8_slot;
        let mut var_eu_i_dn9: f64 = *var_eu_i_dn9_slot;
        let mut var_eur_i: f64 = *var_eur_i_slot;
        let mut var_eur_i_dn0: f64 = *var_eur_i_dn0_slot;
        let mut var_eur_i_dn10: f64 = *var_eur_i_dn10_slot;
        let mut var_eur_i_dn11: f64 = *var_eur_i_dn11_slot;
        let mut var_eur_i_dn13: f64 = *var_eur_i_dn13_slot;
        let mut var_eur_i_dn14: f64 = *var_eur_i_dn14_slot;
        let mut var_eur_i_dn2: f64 = *var_eur_i_dn2_slot;
        let mut var_eur_i_dn3: f64 = *var_eur_i_dn3_slot;
        let mut var_eur_i_dn4: f64 = *var_eur_i_dn4_slot;
        let mut var_eur_i_dn5: f64 = *var_eur_i_dn5_slot;
        let mut var_eur_i_dn6: f64 = *var_eur_i_dn6_slot;
        let mut var_eur_i_dn7: f64 = *var_eur_i_dn7_slot;
        let mut var_eur_i_dn8: f64 = *var_eur_i_dn8_slot;
        let mut var_eur_i_dn9: f64 = *var_eur_i_dn9_slot;
        let mut var_guard65: f64 = *var_guard65_slot;
        let mut var_guard66: f64 = *var_guard66_slot;
        let mut var_guard67: f64 = *var_guard67_slot;
        let mut var_guard68: f64 = *var_guard68_slot;
        let mut var_guard69: f64 = *var_guard69_slot;
        let mut var_leff_ln: f64 = *var_leff_ln_slot;
        let mut var_leff_ln_dn0: f64 = *var_leff_ln_dn0_slot;
        let mut var_leff_ln_dn10: f64 = *var_leff_ln_dn10_slot;
        let mut var_leff_ln_dn11: f64 = *var_leff_ln_dn11_slot;
        let mut var_leff_ln_dn13: f64 = *var_leff_ln_dn13_slot;
        let mut var_leff_ln_dn14: f64 = *var_leff_ln_dn14_slot;
        let mut var_leff_ln_dn2: f64 = *var_leff_ln_dn2_slot;
        let mut var_leff_ln_dn3: f64 = *var_leff_ln_dn3_slot;
        let mut var_leff_ln_dn4: f64 = *var_leff_ln_dn4_slot;
        let mut var_leff_ln_dn5: f64 = *var_leff_ln_dn5_slot;
        let mut var_leff_ln_dn6: f64 = *var_leff_ln_dn6_slot;
        let mut var_leff_ln_dn7: f64 = *var_leff_ln_dn7_slot;
        let mut var_leff_ln_dn8: f64 = *var_leff_ln_dn8_slot;
        let mut var_leff_ln_dn9: f64 = *var_leff_ln_dn9_slot;
        let mut var_mexp_i: f64 = *var_mexp_i_slot;
        let mut var_mexp_i_dn0: f64 = *var_mexp_i_dn0_slot;
        let mut var_mexp_i_dn10: f64 = *var_mexp_i_dn10_slot;
        let mut var_mexp_i_dn11: f64 = *var_mexp_i_dn11_slot;
        let mut var_mexp_i_dn13: f64 = *var_mexp_i_dn13_slot;
        let mut var_mexp_i_dn14: f64 = *var_mexp_i_dn14_slot;
        let mut var_mexp_i_dn2: f64 = *var_mexp_i_dn2_slot;
        let mut var_mexp_i_dn3: f64 = *var_mexp_i_dn3_slot;
        let mut var_mexp_i_dn4: f64 = *var_mexp_i_dn4_slot;
        let mut var_mexp_i_dn5: f64 = *var_mexp_i_dn5_slot;
        let mut var_mexp_i_dn6: f64 = *var_mexp_i_dn6_slot;
        let mut var_mexp_i_dn7: f64 = *var_mexp_i_dn7_slot;
        let mut var_mexp_i_dn8: f64 = *var_mexp_i_dn8_slot;
        let mut var_mexp_i_dn9: f64 = *var_mexp_i_dn9_slot;
        let mut var_pclm_i: f64 = *var_pclm_i_slot;
        let mut var_pclm_i_dn0: f64 = *var_pclm_i_dn0_slot;
        let mut var_pclm_i_dn10: f64 = *var_pclm_i_dn10_slot;
        let mut var_pclm_i_dn11: f64 = *var_pclm_i_dn11_slot;
        let mut var_pclm_i_dn13: f64 = *var_pclm_i_dn13_slot;
        let mut var_pclm_i_dn14: f64 = *var_pclm_i_dn14_slot;
        let mut var_pclm_i_dn2: f64 = *var_pclm_i_dn2_slot;
        let mut var_pclm_i_dn3: f64 = *var_pclm_i_dn3_slot;
        let mut var_pclm_i_dn4: f64 = *var_pclm_i_dn4_slot;
        let mut var_pclm_i_dn5: f64 = *var_pclm_i_dn5_slot;
        let mut var_pclm_i_dn6: f64 = *var_pclm_i_dn6_slot;
        let mut var_pclm_i_dn7: f64 = *var_pclm_i_dn7_slot;
        let mut var_pclm_i_dn8: f64 = *var_pclm_i_dn8_slot;
        let mut var_pclm_i_dn9: f64 = *var_pclm_i_dn9_slot;
        let mut var_pclmr_i: f64 = *var_pclmr_i_slot;
        let mut var_pclmr_i_dn0: f64 = *var_pclmr_i_dn0_slot;
        let mut var_pclmr_i_dn10: f64 = *var_pclmr_i_dn10_slot;
        let mut var_pclmr_i_dn11: f64 = *var_pclmr_i_dn11_slot;
        let mut var_pclmr_i_dn13: f64 = *var_pclmr_i_dn13_slot;
        let mut var_pclmr_i_dn14: f64 = *var_pclmr_i_dn14_slot;
        let mut var_pclmr_i_dn2: f64 = *var_pclmr_i_dn2_slot;
        let mut var_pclmr_i_dn3: f64 = *var_pclmr_i_dn3_slot;
        let mut var_pclmr_i_dn4: f64 = *var_pclmr_i_dn4_slot;
        let mut var_pclmr_i_dn5: f64 = *var_pclmr_i_dn5_slot;
        let mut var_pclmr_i_dn6: f64 = *var_pclmr_i_dn6_slot;
        let mut var_pclmr_i_dn7: f64 = *var_pclmr_i_dn7_slot;
        let mut var_pclmr_i_dn8: f64 = *var_pclmr_i_dn8_slot;
        let mut var_pclmr_i_dn9: f64 = *var_pclmr_i_dn9_slot;
        let mut var_phig_i: f64 = *var_phig_i_slot;
        let mut var_phig_i_dn0: f64 = *var_phig_i_dn0_slot;
        let mut var_phig_i_dn10: f64 = *var_phig_i_dn10_slot;
        let mut var_phig_i_dn11: f64 = *var_phig_i_dn11_slot;
        let mut var_phig_i_dn13: f64 = *var_phig_i_dn13_slot;
        let mut var_phig_i_dn14: f64 = *var_phig_i_dn14_slot;
        let mut var_phig_i_dn2: f64 = *var_phig_i_dn2_slot;
        let mut var_phig_i_dn3: f64 = *var_phig_i_dn3_slot;
        let mut var_phig_i_dn4: f64 = *var_phig_i_dn4_slot;
        let mut var_phig_i_dn5: f64 = *var_phig_i_dn5_slot;
        let mut var_phig_i_dn6: f64 = *var_phig_i_dn6_slot;
        let mut var_phig_i_dn7: f64 = *var_phig_i_dn7_slot;
        let mut var_phig_i_dn8: f64 = *var_phig_i_dn8_slot;
        let mut var_phig_i_dn9: f64 = *var_phig_i_dn9_slot;
        let mut var_pqm_i: f64 = *var_pqm_i_slot;
        let mut var_pqm_i_dn0: f64 = *var_pqm_i_dn0_slot;
        let mut var_pqm_i_dn10: f64 = *var_pqm_i_dn10_slot;
        let mut var_pqm_i_dn11: f64 = *var_pqm_i_dn11_slot;
        let mut var_pqm_i_dn13: f64 = *var_pqm_i_dn13_slot;
        let mut var_pqm_i_dn14: f64 = *var_pqm_i_dn14_slot;
        let mut var_pqm_i_dn2: f64 = *var_pqm_i_dn2_slot;
        let mut var_pqm_i_dn3: f64 = *var_pqm_i_dn3_slot;
        let mut var_pqm_i_dn4: f64 = *var_pqm_i_dn4_slot;
        let mut var_pqm_i_dn5: f64 = *var_pqm_i_dn5_slot;
        let mut var_pqm_i_dn6: f64 = *var_pqm_i_dn6_slot;
        let mut var_pqm_i_dn7: f64 = *var_pqm_i_dn7_slot;
        let mut var_pqm_i_dn8: f64 = *var_pqm_i_dn8_slot;
        let mut var_pqm_i_dn9: f64 = *var_pqm_i_dn9_slot;
        let mut var_rdsw_i: f64 = *var_rdsw_i_slot;
        let mut var_rdsw_i_dn0: f64 = *var_rdsw_i_dn0_slot;
        let mut var_rdsw_i_dn10: f64 = *var_rdsw_i_dn10_slot;
        let mut var_rdsw_i_dn11: f64 = *var_rdsw_i_dn11_slot;
        let mut var_rdsw_i_dn13: f64 = *var_rdsw_i_dn13_slot;
        let mut var_rdsw_i_dn14: f64 = *var_rdsw_i_dn14_slot;
        let mut var_rdsw_i_dn2: f64 = *var_rdsw_i_dn2_slot;
        let mut var_rdsw_i_dn3: f64 = *var_rdsw_i_dn3_slot;
        let mut var_rdsw_i_dn4: f64 = *var_rdsw_i_dn4_slot;
        let mut var_rdsw_i_dn5: f64 = *var_rdsw_i_dn5_slot;
        let mut var_rdsw_i_dn6: f64 = *var_rdsw_i_dn6_slot;
        let mut var_rdsw_i_dn7: f64 = *var_rdsw_i_dn7_slot;
        let mut var_rdsw_i_dn8: f64 = *var_rdsw_i_dn8_slot;
        let mut var_rdsw_i_dn9: f64 = *var_rdsw_i_dn9_slot;
        let mut var_rdw_i: f64 = *var_rdw_i_slot;
        let mut var_rdw_i_dn0: f64 = *var_rdw_i_dn0_slot;
        let mut var_rdw_i_dn10: f64 = *var_rdw_i_dn10_slot;
        let mut var_rdw_i_dn11: f64 = *var_rdw_i_dn11_slot;
        let mut var_rdw_i_dn13: f64 = *var_rdw_i_dn13_slot;
        let mut var_rdw_i_dn14: f64 = *var_rdw_i_dn14_slot;
        let mut var_rdw_i_dn2: f64 = *var_rdw_i_dn2_slot;
        let mut var_rdw_i_dn3: f64 = *var_rdw_i_dn3_slot;
        let mut var_rdw_i_dn4: f64 = *var_rdw_i_dn4_slot;
        let mut var_rdw_i_dn5: f64 = *var_rdw_i_dn5_slot;
        let mut var_rdw_i_dn6: f64 = *var_rdw_i_dn6_slot;
        let mut var_rdw_i_dn7: f64 = *var_rdw_i_dn7_slot;
        let mut var_rdw_i_dn8: f64 = *var_rdw_i_dn8_slot;
        let mut var_rdw_i_dn9: f64 = *var_rdw_i_dn9_slot;
        let mut var_rsw_i: f64 = *var_rsw_i_slot;
        let mut var_rsw_i_dn0: f64 = *var_rsw_i_dn0_slot;
        let mut var_rsw_i_dn10: f64 = *var_rsw_i_dn10_slot;
        let mut var_rsw_i_dn11: f64 = *var_rsw_i_dn11_slot;
        let mut var_rsw_i_dn13: f64 = *var_rsw_i_dn13_slot;
        let mut var_rsw_i_dn14: f64 = *var_rsw_i_dn14_slot;
        let mut var_rsw_i_dn2: f64 = *var_rsw_i_dn2_slot;
        let mut var_rsw_i_dn3: f64 = *var_rsw_i_dn3_slot;
        let mut var_rsw_i_dn4: f64 = *var_rsw_i_dn4_slot;
        let mut var_rsw_i_dn5: f64 = *var_rsw_i_dn5_slot;
        let mut var_rsw_i_dn6: f64 = *var_rsw_i_dn6_slot;
        let mut var_rsw_i_dn7: f64 = *var_rsw_i_dn7_slot;
        let mut var_rsw_i_dn8: f64 = *var_rsw_i_dn8_slot;
        let mut var_rsw_i_dn9: f64 = *var_rsw_i_dn9_slot;
        let mut var_u0_i: f64 = *var_u0_i_slot;
        let mut var_u0_i_dn0: f64 = *var_u0_i_dn0_slot;
        let mut var_u0_i_dn10: f64 = *var_u0_i_dn10_slot;
        let mut var_u0_i_dn11: f64 = *var_u0_i_dn11_slot;
        let mut var_u0_i_dn13: f64 = *var_u0_i_dn13_slot;
        let mut var_u0_i_dn14: f64 = *var_u0_i_dn14_slot;
        let mut var_u0_i_dn2: f64 = *var_u0_i_dn2_slot;
        let mut var_u0_i_dn3: f64 = *var_u0_i_dn3_slot;
        let mut var_u0_i_dn4: f64 = *var_u0_i_dn4_slot;
        let mut var_u0_i_dn5: f64 = *var_u0_i_dn5_slot;
        let mut var_u0_i_dn6: f64 = *var_u0_i_dn6_slot;
        let mut var_u0_i_dn7: f64 = *var_u0_i_dn7_slot;
        let mut var_u0_i_dn8: f64 = *var_u0_i_dn8_slot;
        let mut var_u0_i_dn9: f64 = *var_u0_i_dn9_slot;
        let mut var_u0r_i: f64 = *var_u0r_i_slot;
        let mut var_u0r_i_dn0: f64 = *var_u0r_i_dn0_slot;
        let mut var_u0r_i_dn10: f64 = *var_u0r_i_dn10_slot;
        let mut var_u0r_i_dn11: f64 = *var_u0r_i_dn11_slot;
        let mut var_u0r_i_dn13: f64 = *var_u0r_i_dn13_slot;
        let mut var_u0r_i_dn14: f64 = *var_u0r_i_dn14_slot;
        let mut var_u0r_i_dn2: f64 = *var_u0r_i_dn2_slot;
        let mut var_u0r_i_dn3: f64 = *var_u0r_i_dn3_slot;
        let mut var_u0r_i_dn4: f64 = *var_u0r_i_dn4_slot;
        let mut var_u0r_i_dn5: f64 = *var_u0r_i_dn5_slot;
        let mut var_u0r_i_dn6: f64 = *var_u0r_i_dn6_slot;
        let mut var_u0r_i_dn7: f64 = *var_u0r_i_dn7_slot;
        let mut var_u0r_i_dn8: f64 = *var_u0r_i_dn8_slot;
        let mut var_u0r_i_dn9: f64 = *var_u0r_i_dn9_slot;
        let mut var_ua_i: f64 = *var_ua_i_slot;
        let mut var_ua_i_dn0: f64 = *var_ua_i_dn0_slot;
        let mut var_ua_i_dn10: f64 = *var_ua_i_dn10_slot;
        let mut var_ua_i_dn11: f64 = *var_ua_i_dn11_slot;
        let mut var_ua_i_dn13: f64 = *var_ua_i_dn13_slot;
        let mut var_ua_i_dn14: f64 = *var_ua_i_dn14_slot;
        let mut var_ua_i_dn2: f64 = *var_ua_i_dn2_slot;
        let mut var_ua_i_dn3: f64 = *var_ua_i_dn3_slot;
        let mut var_ua_i_dn4: f64 = *var_ua_i_dn4_slot;
        let mut var_ua_i_dn5: f64 = *var_ua_i_dn5_slot;
        let mut var_ua_i_dn6: f64 = *var_ua_i_dn6_slot;
        let mut var_ua_i_dn7: f64 = *var_ua_i_dn7_slot;
        let mut var_ua_i_dn8: f64 = *var_ua_i_dn8_slot;
        let mut var_ua_i_dn9: f64 = *var_ua_i_dn9_slot;
        let mut var_uar_i: f64 = *var_uar_i_slot;
        let mut var_uar_i_dn0: f64 = *var_uar_i_dn0_slot;
        let mut var_uar_i_dn10: f64 = *var_uar_i_dn10_slot;
        let mut var_uar_i_dn11: f64 = *var_uar_i_dn11_slot;
        let mut var_uar_i_dn13: f64 = *var_uar_i_dn13_slot;
        let mut var_uar_i_dn14: f64 = *var_uar_i_dn14_slot;
        let mut var_uar_i_dn2: f64 = *var_uar_i_dn2_slot;
        let mut var_uar_i_dn3: f64 = *var_uar_i_dn3_slot;
        let mut var_uar_i_dn4: f64 = *var_uar_i_dn4_slot;
        let mut var_uar_i_dn5: f64 = *var_uar_i_dn5_slot;
        let mut var_uar_i_dn6: f64 = *var_uar_i_dn6_slot;
        let mut var_uar_i_dn7: f64 = *var_uar_i_dn7_slot;
        let mut var_uar_i_dn8: f64 = *var_uar_i_dn8_slot;
        let mut var_uar_i_dn9: f64 = *var_uar_i_dn9_slot;
        let mut var_ud_i: f64 = *var_ud_i_slot;
        let mut var_ud_i_dn0: f64 = *var_ud_i_dn0_slot;
        let mut var_ud_i_dn10: f64 = *var_ud_i_dn10_slot;
        let mut var_ud_i_dn11: f64 = *var_ud_i_dn11_slot;
        let mut var_ud_i_dn13: f64 = *var_ud_i_dn13_slot;
        let mut var_ud_i_dn14: f64 = *var_ud_i_dn14_slot;
        let mut var_ud_i_dn2: f64 = *var_ud_i_dn2_slot;
        let mut var_ud_i_dn3: f64 = *var_ud_i_dn3_slot;
        let mut var_ud_i_dn4: f64 = *var_ud_i_dn4_slot;
        let mut var_ud_i_dn5: f64 = *var_ud_i_dn5_slot;
        let mut var_ud_i_dn6: f64 = *var_ud_i_dn6_slot;
        let mut var_ud_i_dn7: f64 = *var_ud_i_dn7_slot;
        let mut var_ud_i_dn8: f64 = *var_ud_i_dn8_slot;
        let mut var_ud_i_dn9: f64 = *var_ud_i_dn9_slot;
        let mut var_udr_i: f64 = *var_udr_i_slot;
        let mut var_udr_i_dn0: f64 = *var_udr_i_dn0_slot;
        let mut var_udr_i_dn10: f64 = *var_udr_i_dn10_slot;
        let mut var_udr_i_dn11: f64 = *var_udr_i_dn11_slot;
        let mut var_udr_i_dn13: f64 = *var_udr_i_dn13_slot;
        let mut var_udr_i_dn14: f64 = *var_udr_i_dn14_slot;
        let mut var_udr_i_dn2: f64 = *var_udr_i_dn2_slot;
        let mut var_udr_i_dn3: f64 = *var_udr_i_dn3_slot;
        let mut var_udr_i_dn4: f64 = *var_udr_i_dn4_slot;
        let mut var_udr_i_dn5: f64 = *var_udr_i_dn5_slot;
        let mut var_udr_i_dn6: f64 = *var_udr_i_dn6_slot;
        let mut var_udr_i_dn7: f64 = *var_udr_i_dn7_slot;
        let mut var_udr_i_dn8: f64 = *var_udr_i_dn8_slot;
        let mut var_udr_i_dn9: f64 = *var_udr_i_dn9_slot;

        let (assign6850_e10553, assign6850_e10553_d_n0, assign6850_e10553_d_n2, assign6850_e10553_d_n3, assign6850_e10553_d_n4, assign6850_e10553_d_n5, assign6850_e10553_d_n6, assign6850_e10553_d_n7, assign6850_e10553_d_n8, assign6850_e10553_d_n9, assign6850_e10553_d_n10, assign6850_e10553_d_n11, assign6850_e10553_d_n13, assign6850_e10553_d_n14,) = {
    if (var_guard64 != 0.0) {
        let assign6850_e10545: f64 = (p.p5 - p.p21);
        let assign6850_e10547: f64 = (assign6850_e10545 * p.p160);
        let assign6850_e10549: f64 = (assign6850_e10547 * var_leff_1);
        let assign6850_e10550: f64 = (1.0 + assign6850_e10549);
        let assign6850_e10551: f64 = (var_eta0_i * assign6850_e10550);
        (assign6850_e10551, ((var_eta0_i_dn0 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn0))), ((var_eta0_i_dn2 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn2))), ((var_eta0_i_dn3 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn3))), ((var_eta0_i_dn4 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn4))), ((var_eta0_i_dn5 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn5))), ((var_eta0_i_dn6 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn6))), ((var_eta0_i_dn7 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn7))), ((var_eta0_i_dn8 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn8))), ((var_eta0_i_dn9 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn9))), ((var_eta0_i_dn10 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn10))), ((var_eta0_i_dn11 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn11))), ((var_eta0_i_dn13 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn13))), ((var_eta0_i_dn14 * assign6850_e10550) + (var_eta0_i * (assign6850_e10547 * var_leff_1_dn14))),)
    } else {
        (var_eta0_i, var_eta0_i_dn0, var_eta0_i_dn2, var_eta0_i_dn3, var_eta0_i_dn4, var_eta0_i_dn5, var_eta0_i_dn6, var_eta0_i_dn7, var_eta0_i_dn8, var_eta0_i_dn9, var_eta0_i_dn10, var_eta0_i_dn11, var_eta0_i_dn13, var_eta0_i_dn14,)
    }
};
        var_eta0_i = assign6850_e10553;
        var_eta0_i_dn0 = assign6850_e10553_d_n0;
        var_eta0_i_dn2 = assign6850_e10553_d_n2;
        var_eta0_i_dn3 = assign6850_e10553_d_n3;
        var_eta0_i_dn4 = assign6850_e10553_d_n4;
        var_eta0_i_dn5 = assign6850_e10553_d_n5;
        var_eta0_i_dn6 = assign6850_e10553_d_n6;
        var_eta0_i_dn7 = assign6850_e10553_d_n7;
        var_eta0_i_dn8 = assign6850_e10553_d_n8;
        var_eta0_i_dn9 = assign6850_e10553_d_n9;
        var_eta0_i_dn10 = assign6850_e10553_d_n10;
        var_eta0_i_dn11 = assign6850_e10553_d_n11;
        var_eta0_i_dn13 = assign6850_e10553_d_n13;
        var_eta0_i_dn14 = assign6850_e10553_d_n14;

        let (assign6860_e10567, assign6860_e10567_d_n0, assign6860_e10567_d_n2, assign6860_e10567_d_n3, assign6860_e10567_d_n4, assign6860_e10567_d_n5, assign6860_e10567_d_n6, assign6860_e10567_d_n7, assign6860_e10567_d_n8, assign6860_e10567_d_n9, assign6860_e10567_d_n10, assign6860_e10567_d_n11, assign6860_e10567_d_n13, assign6860_e10567_d_n14,) = {
    if (var_guard64 != 0.0) {
        let assign6860_e10559: f64 = (p.p5 - p.p21);
        let assign6860_e10561: f64 = (assign6860_e10559 * p.p587);
        let assign6860_e10563: f64 = (assign6860_e10561 * var_leff_1);
        let assign6860_e10564: f64 = (1.0 + assign6860_e10563);
        let assign6860_e10565: f64 = (var_u0_i * assign6860_e10564);
        (assign6860_e10565, ((var_u0_i_dn0 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn0))), ((var_u0_i_dn2 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn2))), ((var_u0_i_dn3 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn3))), ((var_u0_i_dn4 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn4))), ((var_u0_i_dn5 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn5))), ((var_u0_i_dn6 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn6))), ((var_u0_i_dn7 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn7))), ((var_u0_i_dn8 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn8))), ((var_u0_i_dn9 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn9))), ((var_u0_i_dn10 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn10))), ((var_u0_i_dn11 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn11))), ((var_u0_i_dn13 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn13))), ((var_u0_i_dn14 * assign6860_e10564) + (var_u0_i * (assign6860_e10561 * var_leff_1_dn14))),)
    } else {
        (var_u0_i, var_u0_i_dn0, var_u0_i_dn2, var_u0_i_dn3, var_u0_i_dn4, var_u0_i_dn5, var_u0_i_dn6, var_u0_i_dn7, var_u0_i_dn8, var_u0_i_dn9, var_u0_i_dn10, var_u0_i_dn11, var_u0_i_dn13, var_u0_i_dn14,)
    }
};
        var_u0_i = assign6860_e10567;
        var_u0_i_dn0 = assign6860_e10567_d_n0;
        var_u0_i_dn2 = assign6860_e10567_d_n2;
        var_u0_i_dn3 = assign6860_e10567_d_n3;
        var_u0_i_dn4 = assign6860_e10567_d_n4;
        var_u0_i_dn5 = assign6860_e10567_d_n5;
        var_u0_i_dn6 = assign6860_e10567_d_n6;
        var_u0_i_dn7 = assign6860_e10567_d_n7;
        var_u0_i_dn8 = assign6860_e10567_d_n8;
        var_u0_i_dn9 = assign6860_e10567_d_n9;
        var_u0_i_dn10 = assign6860_e10567_d_n10;
        var_u0_i_dn11 = assign6860_e10567_d_n11;
        var_u0_i_dn13 = assign6860_e10567_d_n13;
        var_u0_i_dn14 = assign6860_e10567_d_n14;

        let assign6870_e10569: f64 = (var_leff_1).ln();
        var_leff_ln = assign6870_e10569;
        var_leff_ln_dn0 = (var_leff_1_dn0 / var_leff_1);
        var_leff_ln_dn2 = (var_leff_1_dn2 / var_leff_1);
        var_leff_ln_dn3 = (var_leff_1_dn3 / var_leff_1);
        var_leff_ln_dn4 = (var_leff_1_dn4 / var_leff_1);
        var_leff_ln_dn5 = (var_leff_1_dn5 / var_leff_1);
        var_leff_ln_dn6 = (var_leff_1_dn6 / var_leff_1);
        var_leff_ln_dn7 = (var_leff_1_dn7 / var_leff_1);
        var_leff_ln_dn8 = (var_leff_1_dn8 / var_leff_1);
        var_leff_ln_dn9 = (var_leff_1_dn9 / var_leff_1);
        var_leff_ln_dn10 = (var_leff_1_dn10 / var_leff_1);
        var_leff_ln_dn11 = (var_leff_1_dn11 / var_leff_1);
        var_leff_ln_dn13 = (var_leff_1_dn13 / var_leff_1);
        var_leff_ln_dn14 = (var_leff_1_dn14 / var_leff_1);

        let assign6880_e10573: f64 = (p.p98 * var_leff_1);
        let assign6880_e10574: f64 = (var_phig_i + assign6880_e10573);
        var_phig_i = assign6880_e10574;
        var_phig_i_dn0 = (var_phig_i_dn0 + (p.p98 * var_leff_1_dn0));
        var_phig_i_dn2 = (var_phig_i_dn2 + (p.p98 * var_leff_1_dn2));
        var_phig_i_dn3 = (var_phig_i_dn3 + (p.p98 * var_leff_1_dn3));
        var_phig_i_dn4 = (var_phig_i_dn4 + (p.p98 * var_leff_1_dn4));
        var_phig_i_dn5 = (var_phig_i_dn5 + (p.p98 * var_leff_1_dn5));
        var_phig_i_dn6 = (var_phig_i_dn6 + (p.p98 * var_leff_1_dn6));
        var_phig_i_dn7 = (var_phig_i_dn7 + (p.p98 * var_leff_1_dn7));
        var_phig_i_dn8 = (var_phig_i_dn8 + (p.p98 * var_leff_1_dn8));
        var_phig_i_dn9 = (var_phig_i_dn9 + (p.p98 * var_leff_1_dn9));
        var_phig_i_dn10 = (var_phig_i_dn10 + (p.p98 * var_leff_1_dn10));
        var_phig_i_dn11 = (var_phig_i_dn11 + (p.p98 * var_leff_1_dn11));
        var_phig_i_dn13 = (var_phig_i_dn13 + (p.p98 * var_leff_1_dn13));
        var_phig_i_dn14 = (var_phig_i_dn14 + (p.p98 * var_leff_1_dn14));

        let assign6890_e10578: f64 = (p.p427 * var_leff_1);
        let assign6890_e10579: f64 = (var_pqm_i + assign6890_e10578);
        var_pqm_i = assign6890_e10579;
        var_pqm_i_dn0 = (var_pqm_i_dn0 + (p.p427 * var_leff_1_dn0));
        var_pqm_i_dn2 = (var_pqm_i_dn2 + (p.p427 * var_leff_1_dn2));
        var_pqm_i_dn3 = (var_pqm_i_dn3 + (p.p427 * var_leff_1_dn3));
        var_pqm_i_dn4 = (var_pqm_i_dn4 + (p.p427 * var_leff_1_dn4));
        var_pqm_i_dn5 = (var_pqm_i_dn5 + (p.p427 * var_leff_1_dn5));
        var_pqm_i_dn6 = (var_pqm_i_dn6 + (p.p427 * var_leff_1_dn6));
        var_pqm_i_dn7 = (var_pqm_i_dn7 + (p.p427 * var_leff_1_dn7));
        var_pqm_i_dn8 = (var_pqm_i_dn8 + (p.p427 * var_leff_1_dn8));
        var_pqm_i_dn9 = (var_pqm_i_dn9 + (p.p427 * var_leff_1_dn9));
        var_pqm_i_dn10 = (var_pqm_i_dn10 + (p.p427 * var_leff_1_dn10));
        var_pqm_i_dn11 = (var_pqm_i_dn11 + (p.p427 * var_leff_1_dn11));
        var_pqm_i_dn13 = (var_pqm_i_dn13 + (p.p427 * var_leff_1_dn13));
        var_pqm_i_dn14 = (var_pqm_i_dn14 + (p.p427 * var_leff_1_dn14));

        let assign6900_e10582: f64 = if p.p589 > 0.0 { 1.0 } else { 0.0 };
        var_guard65 = assign6900_e10582;

        let (assign6910_e10596, assign6910_e10596_d_n0, assign6910_e10596_d_n2, assign6910_e10596_d_n3, assign6910_e10596_d_n4, assign6910_e10596_d_n5, assign6910_e10596_d_n6, assign6910_e10596_d_n7, assign6910_e10596_d_n8, assign6910_e10596_d_n9, assign6910_e10596_d_n10, assign6910_e10596_d_n11, assign6910_e10596_d_n13, assign6910_e10596_d_n14,) = {
    if (var_guard65 != 0.0) {
        let assign6910_e10588: f64 = (-p.p589);
        let assign6910_e10590: f64 = (assign6910_e10588 * var_leff_ln);
        let assign6910_e10591: f64 = (assign6910_e10590).exp();
        let assign6910_e10592: f64 = (var_up_i * assign6910_e10591);
        let assign6910_e10593: f64 = (1.0 - assign6910_e10592);
        let assign6910_e10594: f64 = (var_u0_i * assign6910_e10593);
        (assign6910_e10594, ((var_u0_i_dn0 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn0)))))), ((var_u0_i_dn2 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn2)))))), ((var_u0_i_dn3 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn3)))))), ((var_u0_i_dn4 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn4)))))), ((var_u0_i_dn5 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn5)))))), ((var_u0_i_dn6 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn6)))))), ((var_u0_i_dn7 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn7)))))), ((var_u0_i_dn8 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn8)))))), ((var_u0_i_dn9 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn9)))))), ((var_u0_i_dn10 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn10)))))), ((var_u0_i_dn11 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn11)))))), ((var_u0_i_dn13 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn13)))))), ((var_u0_i_dn14 * assign6910_e10593) + (var_u0_i * (-(var_up_i * (assign6910_e10591 * (assign6910_e10588 * var_leff_ln_dn14)))))),)
    } else {
        (var_u0_i, var_u0_i_dn0, var_u0_i_dn2, var_u0_i_dn3, var_u0_i_dn4, var_u0_i_dn5, var_u0_i_dn6, var_u0_i_dn7, var_u0_i_dn8, var_u0_i_dn9, var_u0_i_dn10, var_u0_i_dn11, var_u0_i_dn13, var_u0_i_dn14,)
    }
};
        var_u0_i = assign6910_e10596;
        var_u0_i_dn0 = assign6910_e10596_d_n0;
        var_u0_i_dn2 = assign6910_e10596_d_n2;
        var_u0_i_dn3 = assign6910_e10596_d_n3;
        var_u0_i_dn4 = assign6910_e10596_d_n4;
        var_u0_i_dn5 = assign6910_e10596_d_n5;
        var_u0_i_dn6 = assign6910_e10596_d_n6;
        var_u0_i_dn7 = assign6910_e10596_d_n7;
        var_u0_i_dn8 = assign6910_e10596_d_n8;
        var_u0_i_dn9 = assign6910_e10596_d_n9;
        var_u0_i_dn10 = assign6910_e10596_d_n10;
        var_u0_i_dn11 = assign6910_e10596_d_n11;
        var_u0_i_dn13 = assign6910_e10596_d_n13;
        var_u0_i_dn14 = assign6910_e10596_d_n14;

        let (assign6920_e10605, assign6920_e10605_d_n0, assign6920_e10605_d_n2, assign6920_e10605_d_n3, assign6920_e10605_d_n4, assign6920_e10605_d_n5, assign6920_e10605_d_n6, assign6920_e10605_d_n7, assign6920_e10605_d_n8, assign6920_e10605_d_n9, assign6920_e10605_d_n10, assign6920_e10605_d_n11, assign6920_e10605_d_n13, assign6920_e10605_d_n14,) = {
    if (var_guard65 == 0.0) {
        let assign6920_e10602: f64 = (1.0 - var_up_i);
        let assign6920_e10603: f64 = (var_u0_i * assign6920_e10602);
        (assign6920_e10603, (var_u0_i_dn0 * assign6920_e10602), (var_u0_i_dn2 * assign6920_e10602), (var_u0_i_dn3 * assign6920_e10602), (var_u0_i_dn4 * assign6920_e10602), (var_u0_i_dn5 * assign6920_e10602), (var_u0_i_dn6 * assign6920_e10602), (var_u0_i_dn7 * assign6920_e10602), (var_u0_i_dn8 * assign6920_e10602), (var_u0_i_dn9 * assign6920_e10602), (var_u0_i_dn10 * assign6920_e10602), (var_u0_i_dn11 * assign6920_e10602), (var_u0_i_dn13 * assign6920_e10602), (var_u0_i_dn14 * assign6920_e10602),)
    } else {
        (var_u0_i, var_u0_i_dn0, var_u0_i_dn2, var_u0_i_dn3, var_u0_i_dn4, var_u0_i_dn5, var_u0_i_dn6, var_u0_i_dn7, var_u0_i_dn8, var_u0_i_dn9, var_u0_i_dn10, var_u0_i_dn11, var_u0_i_dn13, var_u0_i_dn14,)
    }
};
        var_u0_i = assign6920_e10605;
        var_u0_i_dn0 = assign6920_e10605_d_n0;
        var_u0_i_dn2 = assign6920_e10605_d_n2;
        var_u0_i_dn3 = assign6920_e10605_d_n3;
        var_u0_i_dn4 = assign6920_e10605_d_n4;
        var_u0_i_dn5 = assign6920_e10605_d_n5;
        var_u0_i_dn6 = assign6920_e10605_d_n6;
        var_u0_i_dn7 = assign6920_e10605_d_n7;
        var_u0_i_dn8 = assign6920_e10605_d_n8;
        var_u0_i_dn9 = assign6920_e10605_d_n9;
        var_u0_i_dn10 = assign6920_e10605_d_n10;
        var_u0_i_dn11 = assign6920_e10605_d_n11;
        var_u0_i_dn13 = assign6920_e10605_d_n13;
        var_u0_i_dn14 = assign6920_e10605_d_n14;

        let assign6930_e10609: f64 = (-var_leff_1);
        let assign6930_e10611: f64 = (assign6930_e10609 / p.p593);
        let assign6930_e10612: f64 = { let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6930_e10613: f64 = (p.p591 * assign6930_e10612);
        let assign6930_e10614: f64 = (var_ua_i + assign6930_e10613);
        var_ua_i = assign6930_e10614;
        var_ua_i_dn0 = (var_ua_i_dn0 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p593))));
        var_ua_i_dn2 = (var_ua_i_dn2 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p593))));
        var_ua_i_dn3 = (var_ua_i_dn3 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p593))));
        var_ua_i_dn4 = (var_ua_i_dn4 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p593))));
        var_ua_i_dn5 = (var_ua_i_dn5 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p593))));
        var_ua_i_dn6 = (var_ua_i_dn6 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p593))));
        var_ua_i_dn7 = (var_ua_i_dn7 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p593))));
        var_ua_i_dn8 = (var_ua_i_dn8 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p593))));
        var_ua_i_dn9 = (var_ua_i_dn9 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p593))));
        var_ua_i_dn10 = (var_ua_i_dn10 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p593))));
        var_ua_i_dn11 = (var_ua_i_dn11 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p593))));
        var_ua_i_dn13 = (var_ua_i_dn13 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p593))));
        var_ua_i_dn14 = (var_ua_i_dn14 + (p.p591 * ({ let limited_exp_arg = assign6930_e10611; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p593))));

        let assign6940_e10618: f64 = (-var_leff_1);
        let assign6940_e10620: f64 = (assign6940_e10618 / p.p601);
        let assign6940_e10621: f64 = { let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6940_e10622: f64 = (p.p599 * assign6940_e10621);
        let assign6940_e10623: f64 = (var_ud_i + assign6940_e10622);
        var_ud_i = assign6940_e10623;
        var_ud_i_dn0 = (var_ud_i_dn0 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p601))));
        var_ud_i_dn2 = (var_ud_i_dn2 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p601))));
        var_ud_i_dn3 = (var_ud_i_dn3 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p601))));
        var_ud_i_dn4 = (var_ud_i_dn4 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p601))));
        var_ud_i_dn5 = (var_ud_i_dn5 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p601))));
        var_ud_i_dn6 = (var_ud_i_dn6 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p601))));
        var_ud_i_dn7 = (var_ud_i_dn7 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p601))));
        var_ud_i_dn8 = (var_ud_i_dn8 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p601))));
        var_ud_i_dn9 = (var_ud_i_dn9 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p601))));
        var_ud_i_dn10 = (var_ud_i_dn10 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p601))));
        var_ud_i_dn11 = (var_ud_i_dn11 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p601))));
        var_ud_i_dn13 = (var_ud_i_dn13 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p601))));
        var_ud_i_dn14 = (var_ud_i_dn14 + (p.p599 * ({ let limited_exp_arg = assign6940_e10620; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p601))));

        let assign6950_e10627: f64 = (-var_leff_1);
        let assign6950_e10629: f64 = (assign6950_e10627 / p.p597);
        let assign6950_e10630: f64 = { let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6950_e10631: f64 = (p.p595 * assign6950_e10630);
        let assign6950_e10632: f64 = (var_eu_i + assign6950_e10631);
        var_eu_i = assign6950_e10632;
        var_eu_i_dn0 = (var_eu_i_dn0 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p597))));
        var_eu_i_dn2 = (var_eu_i_dn2 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p597))));
        var_eu_i_dn3 = (var_eu_i_dn3 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p597))));
        var_eu_i_dn4 = (var_eu_i_dn4 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p597))));
        var_eu_i_dn5 = (var_eu_i_dn5 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p597))));
        var_eu_i_dn6 = (var_eu_i_dn6 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p597))));
        var_eu_i_dn7 = (var_eu_i_dn7 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p597))));
        var_eu_i_dn8 = (var_eu_i_dn8 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p597))));
        var_eu_i_dn9 = (var_eu_i_dn9 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p597))));
        var_eu_i_dn10 = (var_eu_i_dn10 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p597))));
        var_eu_i_dn11 = (var_eu_i_dn11 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p597))));
        var_eu_i_dn13 = (var_eu_i_dn13 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p597))));
        var_eu_i_dn14 = (var_eu_i_dn14 + (p.p595 * ({ let limited_exp_arg = assign6950_e10629; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p597))));

        let assign6960_e10635: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard66 = assign6960_e10635;

        let (assign6970_e10647, assign6970_e10647_d_n0, assign6970_e10647_d_n2, assign6970_e10647_d_n3, assign6970_e10647_d_n4, assign6970_e10647_d_n5, assign6970_e10647_d_n6, assign6970_e10647_d_n7, assign6970_e10647_d_n8, assign6970_e10647_d_n9, assign6970_e10647_d_n10, assign6970_e10647_d_n11, assign6970_e10647_d_n13, assign6970_e10647_d_n14,) = {
    if (var_guard66 != 0.0) {
        let assign6970_e10640: f64 = (-var_leff_1);
        let assign6970_e10642: f64 = (assign6970_e10640 / p.p594);
        let assign6970_e10643: f64 = { let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6970_e10644: f64 = (p.p592 * assign6970_e10643);
        let assign6970_e10645: f64 = (var_uar_i + assign6970_e10644);
        (assign6970_e10645, (var_uar_i_dn0 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p594)))), (var_uar_i_dn2 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p594)))), (var_uar_i_dn3 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p594)))), (var_uar_i_dn4 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p594)))), (var_uar_i_dn5 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p594)))), (var_uar_i_dn6 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p594)))), (var_uar_i_dn7 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p594)))), (var_uar_i_dn8 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p594)))), (var_uar_i_dn9 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p594)))), (var_uar_i_dn10 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p594)))), (var_uar_i_dn11 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p594)))), (var_uar_i_dn13 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p594)))), (var_uar_i_dn14 + (p.p592 * ({ let limited_exp_arg = assign6970_e10642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p594)))),)
    } else {
        (var_uar_i, var_uar_i_dn0, var_uar_i_dn2, var_uar_i_dn3, var_uar_i_dn4, var_uar_i_dn5, var_uar_i_dn6, var_uar_i_dn7, var_uar_i_dn8, var_uar_i_dn9, var_uar_i_dn10, var_uar_i_dn11, var_uar_i_dn13, var_uar_i_dn14,)
    }
};
        var_uar_i = assign6970_e10647;
        var_uar_i_dn0 = assign6970_e10647_d_n0;
        var_uar_i_dn2 = assign6970_e10647_d_n2;
        var_uar_i_dn3 = assign6970_e10647_d_n3;
        var_uar_i_dn4 = assign6970_e10647_d_n4;
        var_uar_i_dn5 = assign6970_e10647_d_n5;
        var_uar_i_dn6 = assign6970_e10647_d_n6;
        var_uar_i_dn7 = assign6970_e10647_d_n7;
        var_uar_i_dn8 = assign6970_e10647_d_n8;
        var_uar_i_dn9 = assign6970_e10647_d_n9;
        var_uar_i_dn10 = assign6970_e10647_d_n10;
        var_uar_i_dn11 = assign6970_e10647_d_n11;
        var_uar_i_dn13 = assign6970_e10647_d_n13;
        var_uar_i_dn14 = assign6970_e10647_d_n14;

        let (assign6980_e10659, assign6980_e10659_d_n0, assign6980_e10659_d_n2, assign6980_e10659_d_n3, assign6980_e10659_d_n4, assign6980_e10659_d_n5, assign6980_e10659_d_n6, assign6980_e10659_d_n7, assign6980_e10659_d_n8, assign6980_e10659_d_n9, assign6980_e10659_d_n10, assign6980_e10659_d_n11, assign6980_e10659_d_n13, assign6980_e10659_d_n14,) = {
    if (var_guard66 != 0.0) {
        let assign6980_e10652: f64 = (-var_leff_1);
        let assign6980_e10654: f64 = (assign6980_e10652 / p.p602);
        let assign6980_e10655: f64 = { let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6980_e10656: f64 = (p.p600 * assign6980_e10655);
        let assign6980_e10657: f64 = (var_udr_i + assign6980_e10656);
        (assign6980_e10657, (var_udr_i_dn0 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p602)))), (var_udr_i_dn2 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p602)))), (var_udr_i_dn3 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p602)))), (var_udr_i_dn4 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p602)))), (var_udr_i_dn5 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p602)))), (var_udr_i_dn6 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p602)))), (var_udr_i_dn7 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p602)))), (var_udr_i_dn8 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p602)))), (var_udr_i_dn9 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p602)))), (var_udr_i_dn10 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p602)))), (var_udr_i_dn11 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p602)))), (var_udr_i_dn13 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p602)))), (var_udr_i_dn14 + (p.p600 * ({ let limited_exp_arg = assign6980_e10654; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p602)))),)
    } else {
        (var_udr_i, var_udr_i_dn0, var_udr_i_dn2, var_udr_i_dn3, var_udr_i_dn4, var_udr_i_dn5, var_udr_i_dn6, var_udr_i_dn7, var_udr_i_dn8, var_udr_i_dn9, var_udr_i_dn10, var_udr_i_dn11, var_udr_i_dn13, var_udr_i_dn14,)
    }
};
        var_udr_i = assign6980_e10659;
        var_udr_i_dn0 = assign6980_e10659_d_n0;
        var_udr_i_dn2 = assign6980_e10659_d_n2;
        var_udr_i_dn3 = assign6980_e10659_d_n3;
        var_udr_i_dn4 = assign6980_e10659_d_n4;
        var_udr_i_dn5 = assign6980_e10659_d_n5;
        var_udr_i_dn6 = assign6980_e10659_d_n6;
        var_udr_i_dn7 = assign6980_e10659_d_n7;
        var_udr_i_dn8 = assign6980_e10659_d_n8;
        var_udr_i_dn9 = assign6980_e10659_d_n9;
        var_udr_i_dn10 = assign6980_e10659_d_n10;
        var_udr_i_dn11 = assign6980_e10659_d_n11;
        var_udr_i_dn13 = assign6980_e10659_d_n13;
        var_udr_i_dn14 = assign6980_e10659_d_n14;

        let (assign6990_e10671, assign6990_e10671_d_n0, assign6990_e10671_d_n2, assign6990_e10671_d_n3, assign6990_e10671_d_n4, assign6990_e10671_d_n5, assign6990_e10671_d_n6, assign6990_e10671_d_n7, assign6990_e10671_d_n8, assign6990_e10671_d_n9, assign6990_e10671_d_n10, assign6990_e10671_d_n11, assign6990_e10671_d_n13, assign6990_e10671_d_n14,) = {
    if (var_guard66 != 0.0) {
        let assign6990_e10664: f64 = (-var_leff_1);
        let assign6990_e10666: f64 = (assign6990_e10664 / p.p598);
        let assign6990_e10667: f64 = { let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign6990_e10668: f64 = (p.p596 * assign6990_e10667);
        let assign6990_e10669: f64 = (var_eur_i + assign6990_e10668);
        (assign6990_e10669, (var_eur_i_dn0 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p598)))), (var_eur_i_dn2 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p598)))), (var_eur_i_dn3 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p598)))), (var_eur_i_dn4 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p598)))), (var_eur_i_dn5 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p598)))), (var_eur_i_dn6 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p598)))), (var_eur_i_dn7 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p598)))), (var_eur_i_dn8 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p598)))), (var_eur_i_dn9 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p598)))), (var_eur_i_dn10 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p598)))), (var_eur_i_dn11 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p598)))), (var_eur_i_dn13 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p598)))), (var_eur_i_dn14 + (p.p596 * ({ let limited_exp_arg = assign6990_e10666; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p598)))),)
    } else {
        (var_eur_i, var_eur_i_dn0, var_eur_i_dn2, var_eur_i_dn3, var_eur_i_dn4, var_eur_i_dn5, var_eur_i_dn6, var_eur_i_dn7, var_eur_i_dn8, var_eur_i_dn9, var_eur_i_dn10, var_eur_i_dn11, var_eur_i_dn13, var_eur_i_dn14,)
    }
};
        var_eur_i = assign6990_e10671;
        var_eur_i_dn0 = assign6990_e10671_d_n0;
        var_eur_i_dn2 = assign6990_e10671_d_n2;
        var_eur_i_dn3 = assign6990_e10671_d_n3;
        var_eur_i_dn4 = assign6990_e10671_d_n4;
        var_eur_i_dn5 = assign6990_e10671_d_n5;
        var_eur_i_dn6 = assign6990_e10671_d_n6;
        var_eur_i_dn7 = assign6990_e10671_d_n7;
        var_eur_i_dn8 = assign6990_e10671_d_n8;
        var_eur_i_dn9 = assign6990_e10671_d_n9;
        var_eur_i_dn10 = assign6990_e10671_d_n10;
        var_eur_i_dn11 = assign6990_e10671_d_n11;
        var_eur_i_dn13 = assign6990_e10671_d_n13;
        var_eur_i_dn14 = assign6990_e10671_d_n14;

        let assign7000_e10674: f64 = if p.p590 > 0.0 { 1.0 } else { 0.0 };
        var_guard67 = assign7000_e10674;

        let (assign7010_e10690, assign7010_e10690_d_n0, assign7010_e10690_d_n2, assign7010_e10690_d_n3, assign7010_e10690_d_n4, assign7010_e10690_d_n5, assign7010_e10690_d_n6, assign7010_e10690_d_n7, assign7010_e10690_d_n8, assign7010_e10690_d_n9, assign7010_e10690_d_n10, assign7010_e10690_d_n11, assign7010_e10690_d_n13, assign7010_e10690_d_n14,) = {
    if ((var_guard66 != 0.0) && (var_guard67 != 0.0)) {
        let assign7010_e10682: f64 = (-p.p590);
        let assign7010_e10684: f64 = (assign7010_e10682 * var_leff_ln);
        let assign7010_e10685: f64 = (assign7010_e10684).exp();
        let assign7010_e10686: f64 = (var_upr_i * assign7010_e10685);
        let assign7010_e10687: f64 = (1.0 - assign7010_e10686);
        let assign7010_e10688: f64 = (var_u0r_i * assign7010_e10687);
        (assign7010_e10688, ((var_u0r_i_dn0 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn0)))))), ((var_u0r_i_dn2 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn2)))))), ((var_u0r_i_dn3 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn3)))))), ((var_u0r_i_dn4 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn4)))))), ((var_u0r_i_dn5 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn5)))))), ((var_u0r_i_dn6 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn6)))))), ((var_u0r_i_dn7 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn7)))))), ((var_u0r_i_dn8 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn8)))))), ((var_u0r_i_dn9 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn9)))))), ((var_u0r_i_dn10 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn10)))))), ((var_u0r_i_dn11 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn11)))))), ((var_u0r_i_dn13 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn13)))))), ((var_u0r_i_dn14 * assign7010_e10687) + (var_u0r_i * (-(var_upr_i * (assign7010_e10685 * (assign7010_e10682 * var_leff_ln_dn14)))))),)
    } else {
        (var_u0r_i, var_u0r_i_dn0, var_u0r_i_dn2, var_u0r_i_dn3, var_u0r_i_dn4, var_u0r_i_dn5, var_u0r_i_dn6, var_u0r_i_dn7, var_u0r_i_dn8, var_u0r_i_dn9, var_u0r_i_dn10, var_u0r_i_dn11, var_u0r_i_dn13, var_u0r_i_dn14,)
    }
};
        var_u0r_i = assign7010_e10690;
        var_u0r_i_dn0 = assign7010_e10690_d_n0;
        var_u0r_i_dn2 = assign7010_e10690_d_n2;
        var_u0r_i_dn3 = assign7010_e10690_d_n3;
        var_u0r_i_dn4 = assign7010_e10690_d_n4;
        var_u0r_i_dn5 = assign7010_e10690_d_n5;
        var_u0r_i_dn6 = assign7010_e10690_d_n6;
        var_u0r_i_dn7 = assign7010_e10690_d_n7;
        var_u0r_i_dn8 = assign7010_e10690_d_n8;
        var_u0r_i_dn9 = assign7010_e10690_d_n9;
        var_u0r_i_dn10 = assign7010_e10690_d_n10;
        var_u0r_i_dn11 = assign7010_e10690_d_n11;
        var_u0r_i_dn13 = assign7010_e10690_d_n13;
        var_u0r_i_dn14 = assign7010_e10690_d_n14;

        let (assign7020_e10701, assign7020_e10701_d_n0, assign7020_e10701_d_n2, assign7020_e10701_d_n3, assign7020_e10701_d_n4, assign7020_e10701_d_n5, assign7020_e10701_d_n6, assign7020_e10701_d_n7, assign7020_e10701_d_n8, assign7020_e10701_d_n9, assign7020_e10701_d_n10, assign7020_e10701_d_n11, assign7020_e10701_d_n13, assign7020_e10701_d_n14,) = {
    if ((var_guard66 != 0.0) && (var_guard67 == 0.0)) {
        let assign7020_e10698: f64 = (1.0 - var_upr_i);
        let assign7020_e10699: f64 = (var_u0r_i * assign7020_e10698);
        (assign7020_e10699, (var_u0r_i_dn0 * assign7020_e10698), (var_u0r_i_dn2 * assign7020_e10698), (var_u0r_i_dn3 * assign7020_e10698), (var_u0r_i_dn4 * assign7020_e10698), (var_u0r_i_dn5 * assign7020_e10698), (var_u0r_i_dn6 * assign7020_e10698), (var_u0r_i_dn7 * assign7020_e10698), (var_u0r_i_dn8 * assign7020_e10698), (var_u0r_i_dn9 * assign7020_e10698), (var_u0r_i_dn10 * assign7020_e10698), (var_u0r_i_dn11 * assign7020_e10698), (var_u0r_i_dn13 * assign7020_e10698), (var_u0r_i_dn14 * assign7020_e10698),)
    } else {
        (var_u0r_i, var_u0r_i_dn0, var_u0r_i_dn2, var_u0r_i_dn3, var_u0r_i_dn4, var_u0r_i_dn5, var_u0r_i_dn6, var_u0r_i_dn7, var_u0r_i_dn8, var_u0r_i_dn9, var_u0r_i_dn10, var_u0r_i_dn11, var_u0r_i_dn13, var_u0r_i_dn14,)
    }
};
        var_u0r_i = assign7020_e10701;
        var_u0r_i_dn0 = assign7020_e10701_d_n0;
        var_u0r_i_dn2 = assign7020_e10701_d_n2;
        var_u0r_i_dn3 = assign7020_e10701_d_n3;
        var_u0r_i_dn4 = assign7020_e10701_d_n4;
        var_u0r_i_dn5 = assign7020_e10701_d_n5;
        var_u0r_i_dn6 = assign7020_e10701_d_n6;
        var_u0r_i_dn7 = assign7020_e10701_d_n7;
        var_u0r_i_dn8 = assign7020_e10701_d_n8;
        var_u0r_i_dn9 = assign7020_e10701_d_n9;
        var_u0r_i_dn10 = assign7020_e10701_d_n10;
        var_u0r_i_dn11 = assign7020_e10701_d_n11;
        var_u0r_i_dn13 = assign7020_e10701_d_n13;
        var_u0r_i_dn14 = assign7020_e10701_d_n14;

        let assign7030_e10704: f64 = if p.p64 == 1.0 { 1.0 } else { 0.0 };
        var_guard68 = assign7030_e10704;

        let (assign7040_e10716, assign7040_e10716_d_n0, assign7040_e10716_d_n2, assign7040_e10716_d_n3, assign7040_e10716_d_n4, assign7040_e10716_d_n5, assign7040_e10716_d_n6, assign7040_e10716_d_n7, assign7040_e10716_d_n8, assign7040_e10716_d_n9, assign7040_e10716_d_n10, assign7040_e10716_d_n11, assign7040_e10716_d_n13, assign7040_e10716_d_n14,) = {
    if (var_guard68 != 0.0) {
        let assign7040_e10709: f64 = (-var_leff_1);
        let assign7040_e10711: f64 = (assign7040_e10709 / p.p913);
        let assign7040_e10712: f64 = { let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7040_e10713: f64 = (p.p912 * assign7040_e10712);
        let assign7040_e10714: f64 = (var_rsw_i + assign7040_e10713);
        (assign7040_e10714, (var_rsw_i_dn0 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p913)))), (var_rsw_i_dn2 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p913)))), (var_rsw_i_dn3 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p913)))), (var_rsw_i_dn4 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p913)))), (var_rsw_i_dn5 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p913)))), (var_rsw_i_dn6 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p913)))), (var_rsw_i_dn7 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p913)))), (var_rsw_i_dn8 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p913)))), (var_rsw_i_dn9 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p913)))), (var_rsw_i_dn10 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p913)))), (var_rsw_i_dn11 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p913)))), (var_rsw_i_dn13 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p913)))), (var_rsw_i_dn14 + (p.p912 * ({ let limited_exp_arg = assign7040_e10711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p913)))),)
    } else {
        (var_rsw_i, var_rsw_i_dn0, var_rsw_i_dn2, var_rsw_i_dn3, var_rsw_i_dn4, var_rsw_i_dn5, var_rsw_i_dn6, var_rsw_i_dn7, var_rsw_i_dn8, var_rsw_i_dn9, var_rsw_i_dn10, var_rsw_i_dn11, var_rsw_i_dn13, var_rsw_i_dn14,)
    }
};
        var_rsw_i = assign7040_e10716;
        var_rsw_i_dn0 = assign7040_e10716_d_n0;
        var_rsw_i_dn2 = assign7040_e10716_d_n2;
        var_rsw_i_dn3 = assign7040_e10716_d_n3;
        var_rsw_i_dn4 = assign7040_e10716_d_n4;
        var_rsw_i_dn5 = assign7040_e10716_d_n5;
        var_rsw_i_dn6 = assign7040_e10716_d_n6;
        var_rsw_i_dn7 = assign7040_e10716_d_n7;
        var_rsw_i_dn8 = assign7040_e10716_d_n8;
        var_rsw_i_dn9 = assign7040_e10716_d_n9;
        var_rsw_i_dn10 = assign7040_e10716_d_n10;
        var_rsw_i_dn11 = assign7040_e10716_d_n11;
        var_rsw_i_dn13 = assign7040_e10716_d_n13;
        var_rsw_i_dn14 = assign7040_e10716_d_n14;

        let (assign7050_e10728, assign7050_e10728_d_n0, assign7050_e10728_d_n2, assign7050_e10728_d_n3, assign7050_e10728_d_n4, assign7050_e10728_d_n5, assign7050_e10728_d_n6, assign7050_e10728_d_n7, assign7050_e10728_d_n8, assign7050_e10728_d_n9, assign7050_e10728_d_n10, assign7050_e10728_d_n11, assign7050_e10728_d_n13, assign7050_e10728_d_n14,) = {
    if (var_guard68 != 0.0) {
        let assign7050_e10721: f64 = (-var_leff_1);
        let assign7050_e10723: f64 = (assign7050_e10721 / p.p916);
        let assign7050_e10724: f64 = { let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7050_e10725: f64 = (p.p915 * assign7050_e10724);
        let assign7050_e10726: f64 = (var_rdw_i + assign7050_e10725);
        (assign7050_e10726, (var_rdw_i_dn0 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p916)))), (var_rdw_i_dn2 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p916)))), (var_rdw_i_dn3 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p916)))), (var_rdw_i_dn4 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p916)))), (var_rdw_i_dn5 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p916)))), (var_rdw_i_dn6 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p916)))), (var_rdw_i_dn7 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p916)))), (var_rdw_i_dn8 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p916)))), (var_rdw_i_dn9 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p916)))), (var_rdw_i_dn10 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p916)))), (var_rdw_i_dn11 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p916)))), (var_rdw_i_dn13 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p916)))), (var_rdw_i_dn14 + (p.p915 * ({ let limited_exp_arg = assign7050_e10723; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p916)))),)
    } else {
        (var_rdw_i, var_rdw_i_dn0, var_rdw_i_dn2, var_rdw_i_dn3, var_rdw_i_dn4, var_rdw_i_dn5, var_rdw_i_dn6, var_rdw_i_dn7, var_rdw_i_dn8, var_rdw_i_dn9, var_rdw_i_dn10, var_rdw_i_dn11, var_rdw_i_dn13, var_rdw_i_dn14,)
    }
};
        var_rdw_i = assign7050_e10728;
        var_rdw_i_dn0 = assign7050_e10728_d_n0;
        var_rdw_i_dn2 = assign7050_e10728_d_n2;
        var_rdw_i_dn3 = assign7050_e10728_d_n3;
        var_rdw_i_dn4 = assign7050_e10728_d_n4;
        var_rdw_i_dn5 = assign7050_e10728_d_n5;
        var_rdw_i_dn6 = assign7050_e10728_d_n6;
        var_rdw_i_dn7 = assign7050_e10728_d_n7;
        var_rdw_i_dn8 = assign7050_e10728_d_n8;
        var_rdw_i_dn9 = assign7050_e10728_d_n9;
        var_rdw_i_dn10 = assign7050_e10728_d_n10;
        var_rdw_i_dn11 = assign7050_e10728_d_n11;
        var_rdw_i_dn13 = assign7050_e10728_d_n13;
        var_rdw_i_dn14 = assign7050_e10728_d_n14;

        let (assign7060_e10741, assign7060_e10741_d_n0, assign7060_e10741_d_n2, assign7060_e10741_d_n3, assign7060_e10741_d_n4, assign7060_e10741_d_n5, assign7060_e10741_d_n6, assign7060_e10741_d_n7, assign7060_e10741_d_n8, assign7060_e10741_d_n9, assign7060_e10741_d_n10, assign7060_e10741_d_n11, assign7060_e10741_d_n13, assign7060_e10741_d_n14,) = {
    if (var_guard68 == 0.0) {
        let assign7060_e10734: f64 = (-var_leff_1);
        let assign7060_e10736: f64 = (assign7060_e10734 / p.p910);
        let assign7060_e10737: f64 = { let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7060_e10738: f64 = (p.p909 * assign7060_e10737);
        let assign7060_e10739: f64 = (var_rdsw_i + assign7060_e10738);
        (assign7060_e10739, (var_rdsw_i_dn0 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p910)))), (var_rdsw_i_dn2 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p910)))), (var_rdsw_i_dn3 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p910)))), (var_rdsw_i_dn4 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p910)))), (var_rdsw_i_dn5 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p910)))), (var_rdsw_i_dn6 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p910)))), (var_rdsw_i_dn7 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p910)))), (var_rdsw_i_dn8 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p910)))), (var_rdsw_i_dn9 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p910)))), (var_rdsw_i_dn10 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p910)))), (var_rdsw_i_dn11 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p910)))), (var_rdsw_i_dn13 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p910)))), (var_rdsw_i_dn14 + (p.p909 * ({ let limited_exp_arg = assign7060_e10736; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p910)))),)
    } else {
        (var_rdsw_i, var_rdsw_i_dn0, var_rdsw_i_dn2, var_rdsw_i_dn3, var_rdsw_i_dn4, var_rdsw_i_dn5, var_rdsw_i_dn6, var_rdsw_i_dn7, var_rdsw_i_dn8, var_rdsw_i_dn9, var_rdsw_i_dn10, var_rdsw_i_dn11, var_rdsw_i_dn13, var_rdsw_i_dn14,)
    }
};
        var_rdsw_i = assign7060_e10741;
        var_rdsw_i_dn0 = assign7060_e10741_d_n0;
        var_rdsw_i_dn2 = assign7060_e10741_d_n2;
        var_rdsw_i_dn3 = assign7060_e10741_d_n3;
        var_rdsw_i_dn4 = assign7060_e10741_d_n4;
        var_rdsw_i_dn5 = assign7060_e10741_d_n5;
        var_rdsw_i_dn6 = assign7060_e10741_d_n6;
        var_rdsw_i_dn7 = assign7060_e10741_d_n7;
        var_rdsw_i_dn8 = assign7060_e10741_d_n8;
        var_rdsw_i_dn9 = assign7060_e10741_d_n9;
        var_rdsw_i_dn10 = assign7060_e10741_d_n10;
        var_rdsw_i_dn11 = assign7060_e10741_d_n11;
        var_rdsw_i_dn13 = assign7060_e10741_d_n13;
        var_rdsw_i_dn14 = assign7060_e10741_d_n14;

        let assign7070_e10745: f64 = (-var_leff_1);
        let assign7070_e10747: f64 = (assign7070_e10745 / p.p1023);
        let assign7070_e10748: f64 = { let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign7070_e10749: f64 = (p.p1021 * assign7070_e10748);
        let assign7070_e10750: f64 = (var_pclm_i + assign7070_e10749);
        var_pclm_i = assign7070_e10750;
        var_pclm_i_dn0 = (var_pclm_i_dn0 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn0) / p.p1023))));
        var_pclm_i_dn2 = (var_pclm_i_dn2 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn2) / p.p1023))));
        var_pclm_i_dn3 = (var_pclm_i_dn3 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn3) / p.p1023))));
        var_pclm_i_dn4 = (var_pclm_i_dn4 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn4) / p.p1023))));
        var_pclm_i_dn5 = (var_pclm_i_dn5 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn5) / p.p1023))));
        var_pclm_i_dn6 = (var_pclm_i_dn6 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn6) / p.p1023))));
        var_pclm_i_dn7 = (var_pclm_i_dn7 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn7) / p.p1023))));
        var_pclm_i_dn8 = (var_pclm_i_dn8 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn8) / p.p1023))));
        var_pclm_i_dn9 = (var_pclm_i_dn9 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn9) / p.p1023))));
        var_pclm_i_dn10 = (var_pclm_i_dn10 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn10) / p.p1023))));
        var_pclm_i_dn11 = (var_pclm_i_dn11 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn11) / p.p1023))));
        var_pclm_i_dn13 = (var_pclm_i_dn13 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn13) / p.p1023))));
        var_pclm_i_dn14 = (var_pclm_i_dn14 + (p.p1021 * ({ let limited_exp_arg = assign7070_e10747; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-var_leff_1_dn14) / p.p1023))));

        let assign7080_e10753: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        var_guard69 = assign7080_e10753;

        let (assign7090_e10765, assign7090_e10765_d_n0, assign7090_e10765_d_n2, assign7090_e10765_d_n3, assign7090_e10765_d_n4, assign7090_e10765_d_n5, assign7090_e10765_d_n6, assign7090_e10765_d_n7, assign7090_e10765_d_n8, assign7090_e10765_d_n9, assign7090_e10765_d_n10, assign7090_e10765_d_n11, assign7090_e10765_d_n13, assign7090_e10765_d_n14,) = {
    if (var_guard69 != 0.0) {
        let assign7090_e10758: f64 = (-p.p1024);
        let assign7090_e10760: f64 = (assign7090_e10758 * var_leff_ln);
        let assign7090_e10761: f64 = (assign7090_e10760).exp();
        let assign7090_e10762: f64 = (p.p1022 * assign7090_e10761);
        let assign7090_e10763: f64 = (var_pclmr_i + assign7090_e10762);
        (assign7090_e10763, (var_pclmr_i_dn0 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn0)))), (var_pclmr_i_dn2 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn2)))), (var_pclmr_i_dn3 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn3)))), (var_pclmr_i_dn4 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn4)))), (var_pclmr_i_dn5 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn5)))), (var_pclmr_i_dn6 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn6)))), (var_pclmr_i_dn7 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn7)))), (var_pclmr_i_dn8 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn8)))), (var_pclmr_i_dn9 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn9)))), (var_pclmr_i_dn10 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn10)))), (var_pclmr_i_dn11 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn11)))), (var_pclmr_i_dn13 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn13)))), (var_pclmr_i_dn14 + (p.p1022 * (assign7090_e10761 * (assign7090_e10758 * var_leff_ln_dn14)))),)
    } else {
        (var_pclmr_i, var_pclmr_i_dn0, var_pclmr_i_dn2, var_pclmr_i_dn3, var_pclmr_i_dn4, var_pclmr_i_dn5, var_pclmr_i_dn6, var_pclmr_i_dn7, var_pclmr_i_dn8, var_pclmr_i_dn9, var_pclmr_i_dn10, var_pclmr_i_dn11, var_pclmr_i_dn13, var_pclmr_i_dn14,)
    }
};
        var_pclmr_i = assign7090_e10765;
        var_pclmr_i_dn0 = assign7090_e10765_d_n0;
        var_pclmr_i_dn2 = assign7090_e10765_d_n2;
        var_pclmr_i_dn3 = assign7090_e10765_d_n3;
        var_pclmr_i_dn4 = assign7090_e10765_d_n4;
        var_pclmr_i_dn5 = assign7090_e10765_d_n5;
        var_pclmr_i_dn6 = assign7090_e10765_d_n6;
        var_pclmr_i_dn7 = assign7090_e10765_d_n7;
        var_pclmr_i_dn8 = assign7090_e10765_d_n8;
        var_pclmr_i_dn9 = assign7090_e10765_d_n9;
        var_pclmr_i_dn10 = assign7090_e10765_d_n10;
        var_pclmr_i_dn11 = assign7090_e10765_d_n11;
        var_pclmr_i_dn13 = assign7090_e10765_d_n13;
        var_pclmr_i_dn14 = assign7090_e10765_d_n14;

        let assign7100_e10769: f64 = (-p.p445);
        let assign7100_e10771: f64 = (assign7100_e10769 * var_leff_ln);
        let assign7100_e10772: f64 = (assign7100_e10771).exp();
        let assign7100_e10773: f64 = (p.p444 * assign7100_e10772);
        let assign7100_e10774: f64 = (var_mexp_i + assign7100_e10773);
        var_mexp_i = assign7100_e10774;
        var_mexp_i_dn0 = (var_mexp_i_dn0 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn0))));
        var_mexp_i_dn2 = (var_mexp_i_dn2 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn2))));
        var_mexp_i_dn3 = (var_mexp_i_dn3 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn3))));
        var_mexp_i_dn4 = (var_mexp_i_dn4 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn4))));
        var_mexp_i_dn5 = (var_mexp_i_dn5 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn5))));
        var_mexp_i_dn6 = (var_mexp_i_dn6 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn6))));
        var_mexp_i_dn7 = (var_mexp_i_dn7 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn7))));
        var_mexp_i_dn8 = (var_mexp_i_dn8 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn8))));
        var_mexp_i_dn9 = (var_mexp_i_dn9 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn9))));
        var_mexp_i_dn10 = (var_mexp_i_dn10 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn10))));
        var_mexp_i_dn11 = (var_mexp_i_dn11 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn11))));
        var_mexp_i_dn13 = (var_mexp_i_dn13 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn13))));
        var_mexp_i_dn14 = (var_mexp_i_dn14 + (p.p444 * (assign7100_e10772 * (assign7100_e10769 * var_leff_ln_dn14))));

        *var_eta0_i_slot = var_eta0_i;
        *var_eta0_i_dn0_slot = var_eta0_i_dn0;
        *var_eta0_i_dn10_slot = var_eta0_i_dn10;
        *var_eta0_i_dn11_slot = var_eta0_i_dn11;
        *var_eta0_i_dn13_slot = var_eta0_i_dn13;
        *var_eta0_i_dn14_slot = var_eta0_i_dn14;
        *var_eta0_i_dn2_slot = var_eta0_i_dn2;
        *var_eta0_i_dn3_slot = var_eta0_i_dn3;
        *var_eta0_i_dn4_slot = var_eta0_i_dn4;
        *var_eta0_i_dn5_slot = var_eta0_i_dn5;
        *var_eta0_i_dn6_slot = var_eta0_i_dn6;
        *var_eta0_i_dn7_slot = var_eta0_i_dn7;
        *var_eta0_i_dn8_slot = var_eta0_i_dn8;
        *var_eta0_i_dn9_slot = var_eta0_i_dn9;
        *var_eu_i_slot = var_eu_i;
        *var_eu_i_dn0_slot = var_eu_i_dn0;
        *var_eu_i_dn10_slot = var_eu_i_dn10;
        *var_eu_i_dn11_slot = var_eu_i_dn11;
        *var_eu_i_dn13_slot = var_eu_i_dn13;
        *var_eu_i_dn14_slot = var_eu_i_dn14;
        *var_eu_i_dn2_slot = var_eu_i_dn2;
        *var_eu_i_dn3_slot = var_eu_i_dn3;
        *var_eu_i_dn4_slot = var_eu_i_dn4;
        *var_eu_i_dn5_slot = var_eu_i_dn5;
        *var_eu_i_dn6_slot = var_eu_i_dn6;
        *var_eu_i_dn7_slot = var_eu_i_dn7;
        *var_eu_i_dn8_slot = var_eu_i_dn8;
        *var_eu_i_dn9_slot = var_eu_i_dn9;
        *var_eur_i_slot = var_eur_i;
        *var_eur_i_dn0_slot = var_eur_i_dn0;
        *var_eur_i_dn10_slot = var_eur_i_dn10;
        *var_eur_i_dn11_slot = var_eur_i_dn11;
        *var_eur_i_dn13_slot = var_eur_i_dn13;
        *var_eur_i_dn14_slot = var_eur_i_dn14;
        *var_eur_i_dn2_slot = var_eur_i_dn2;
        *var_eur_i_dn3_slot = var_eur_i_dn3;
        *var_eur_i_dn4_slot = var_eur_i_dn4;
        *var_eur_i_dn5_slot = var_eur_i_dn5;
        *var_eur_i_dn6_slot = var_eur_i_dn6;
        *var_eur_i_dn7_slot = var_eur_i_dn7;
        *var_eur_i_dn8_slot = var_eur_i_dn8;
        *var_eur_i_dn9_slot = var_eur_i_dn9;
        *var_guard65_slot = var_guard65;
        *var_guard66_slot = var_guard66;
        *var_guard67_slot = var_guard67;
        *var_guard68_slot = var_guard68;
        *var_guard69_slot = var_guard69;
        *var_leff_ln_slot = var_leff_ln;
        *var_leff_ln_dn0_slot = var_leff_ln_dn0;
        *var_leff_ln_dn10_slot = var_leff_ln_dn10;
        *var_leff_ln_dn11_slot = var_leff_ln_dn11;
        *var_leff_ln_dn13_slot = var_leff_ln_dn13;
        *var_leff_ln_dn14_slot = var_leff_ln_dn14;
        *var_leff_ln_dn2_slot = var_leff_ln_dn2;
        *var_leff_ln_dn3_slot = var_leff_ln_dn3;
        *var_leff_ln_dn4_slot = var_leff_ln_dn4;
        *var_leff_ln_dn5_slot = var_leff_ln_dn5;
        *var_leff_ln_dn6_slot = var_leff_ln_dn6;
        *var_leff_ln_dn7_slot = var_leff_ln_dn7;
        *var_leff_ln_dn8_slot = var_leff_ln_dn8;
        *var_leff_ln_dn9_slot = var_leff_ln_dn9;
        *var_mexp_i_slot = var_mexp_i;
        *var_mexp_i_dn0_slot = var_mexp_i_dn0;
        *var_mexp_i_dn10_slot = var_mexp_i_dn10;
        *var_mexp_i_dn11_slot = var_mexp_i_dn11;
        *var_mexp_i_dn13_slot = var_mexp_i_dn13;
        *var_mexp_i_dn14_slot = var_mexp_i_dn14;
        *var_mexp_i_dn2_slot = var_mexp_i_dn2;
        *var_mexp_i_dn3_slot = var_mexp_i_dn3;
        *var_mexp_i_dn4_slot = var_mexp_i_dn4;
        *var_mexp_i_dn5_slot = var_mexp_i_dn5;
        *var_mexp_i_dn6_slot = var_mexp_i_dn6;
        *var_mexp_i_dn7_slot = var_mexp_i_dn7;
        *var_mexp_i_dn8_slot = var_mexp_i_dn8;
        *var_mexp_i_dn9_slot = var_mexp_i_dn9;
        *var_pclm_i_slot = var_pclm_i;
        *var_pclm_i_dn0_slot = var_pclm_i_dn0;
        *var_pclm_i_dn10_slot = var_pclm_i_dn10;
        *var_pclm_i_dn11_slot = var_pclm_i_dn11;
        *var_pclm_i_dn13_slot = var_pclm_i_dn13;
        *var_pclm_i_dn14_slot = var_pclm_i_dn14;
        *var_pclm_i_dn2_slot = var_pclm_i_dn2;
        *var_pclm_i_dn3_slot = var_pclm_i_dn3;
        *var_pclm_i_dn4_slot = var_pclm_i_dn4;
        *var_pclm_i_dn5_slot = var_pclm_i_dn5;
        *var_pclm_i_dn6_slot = var_pclm_i_dn6;
        *var_pclm_i_dn7_slot = var_pclm_i_dn7;
        *var_pclm_i_dn8_slot = var_pclm_i_dn8;
        *var_pclm_i_dn9_slot = var_pclm_i_dn9;
        *var_pclmr_i_slot = var_pclmr_i;
        *var_pclmr_i_dn0_slot = var_pclmr_i_dn0;
        *var_pclmr_i_dn10_slot = var_pclmr_i_dn10;
        *var_pclmr_i_dn11_slot = var_pclmr_i_dn11;
        *var_pclmr_i_dn13_slot = var_pclmr_i_dn13;
        *var_pclmr_i_dn14_slot = var_pclmr_i_dn14;
        *var_pclmr_i_dn2_slot = var_pclmr_i_dn2;
        *var_pclmr_i_dn3_slot = var_pclmr_i_dn3;
        *var_pclmr_i_dn4_slot = var_pclmr_i_dn4;
        *var_pclmr_i_dn5_slot = var_pclmr_i_dn5;
        *var_pclmr_i_dn6_slot = var_pclmr_i_dn6;
        *var_pclmr_i_dn7_slot = var_pclmr_i_dn7;
        *var_pclmr_i_dn8_slot = var_pclmr_i_dn8;
        *var_pclmr_i_dn9_slot = var_pclmr_i_dn9;
        *var_phig_i_slot = var_phig_i;
        *var_phig_i_dn0_slot = var_phig_i_dn0;
        *var_phig_i_dn10_slot = var_phig_i_dn10;
        *var_phig_i_dn11_slot = var_phig_i_dn11;
        *var_phig_i_dn13_slot = var_phig_i_dn13;
        *var_phig_i_dn14_slot = var_phig_i_dn14;
        *var_phig_i_dn2_slot = var_phig_i_dn2;
        *var_phig_i_dn3_slot = var_phig_i_dn3;
        *var_phig_i_dn4_slot = var_phig_i_dn4;
        *var_phig_i_dn5_slot = var_phig_i_dn5;
        *var_phig_i_dn6_slot = var_phig_i_dn6;
        *var_phig_i_dn7_slot = var_phig_i_dn7;
        *var_phig_i_dn8_slot = var_phig_i_dn8;
        *var_phig_i_dn9_slot = var_phig_i_dn9;
        *var_pqm_i_slot = var_pqm_i;
        *var_pqm_i_dn0_slot = var_pqm_i_dn0;
        *var_pqm_i_dn10_slot = var_pqm_i_dn10;
        *var_pqm_i_dn11_slot = var_pqm_i_dn11;
        *var_pqm_i_dn13_slot = var_pqm_i_dn13;
        *var_pqm_i_dn14_slot = var_pqm_i_dn14;
        *var_pqm_i_dn2_slot = var_pqm_i_dn2;
        *var_pqm_i_dn3_slot = var_pqm_i_dn3;
        *var_pqm_i_dn4_slot = var_pqm_i_dn4;
        *var_pqm_i_dn5_slot = var_pqm_i_dn5;
        *var_pqm_i_dn6_slot = var_pqm_i_dn6;
        *var_pqm_i_dn7_slot = var_pqm_i_dn7;
        *var_pqm_i_dn8_slot = var_pqm_i_dn8;
        *var_pqm_i_dn9_slot = var_pqm_i_dn9;
        *var_rdsw_i_slot = var_rdsw_i;
        *var_rdsw_i_dn0_slot = var_rdsw_i_dn0;
        *var_rdsw_i_dn10_slot = var_rdsw_i_dn10;
        *var_rdsw_i_dn11_slot = var_rdsw_i_dn11;
        *var_rdsw_i_dn13_slot = var_rdsw_i_dn13;
        *var_rdsw_i_dn14_slot = var_rdsw_i_dn14;
        *var_rdsw_i_dn2_slot = var_rdsw_i_dn2;
        *var_rdsw_i_dn3_slot = var_rdsw_i_dn3;
        *var_rdsw_i_dn4_slot = var_rdsw_i_dn4;
        *var_rdsw_i_dn5_slot = var_rdsw_i_dn5;
        *var_rdsw_i_dn6_slot = var_rdsw_i_dn6;
        *var_rdsw_i_dn7_slot = var_rdsw_i_dn7;
        *var_rdsw_i_dn8_slot = var_rdsw_i_dn8;
        *var_rdsw_i_dn9_slot = var_rdsw_i_dn9;
        *var_rdw_i_slot = var_rdw_i;
        *var_rdw_i_dn0_slot = var_rdw_i_dn0;
        *var_rdw_i_dn10_slot = var_rdw_i_dn10;
        *var_rdw_i_dn11_slot = var_rdw_i_dn11;
        *var_rdw_i_dn13_slot = var_rdw_i_dn13;
        *var_rdw_i_dn14_slot = var_rdw_i_dn14;
        *var_rdw_i_dn2_slot = var_rdw_i_dn2;
        *var_rdw_i_dn3_slot = var_rdw_i_dn3;
        *var_rdw_i_dn4_slot = var_rdw_i_dn4;
        *var_rdw_i_dn5_slot = var_rdw_i_dn5;
        *var_rdw_i_dn6_slot = var_rdw_i_dn6;
        *var_rdw_i_dn7_slot = var_rdw_i_dn7;
        *var_rdw_i_dn8_slot = var_rdw_i_dn8;
        *var_rdw_i_dn9_slot = var_rdw_i_dn9;
        *var_rsw_i_slot = var_rsw_i;
        *var_rsw_i_dn0_slot = var_rsw_i_dn0;
        *var_rsw_i_dn10_slot = var_rsw_i_dn10;
        *var_rsw_i_dn11_slot = var_rsw_i_dn11;
        *var_rsw_i_dn13_slot = var_rsw_i_dn13;
        *var_rsw_i_dn14_slot = var_rsw_i_dn14;
        *var_rsw_i_dn2_slot = var_rsw_i_dn2;
        *var_rsw_i_dn3_slot = var_rsw_i_dn3;
        *var_rsw_i_dn4_slot = var_rsw_i_dn4;
        *var_rsw_i_dn5_slot = var_rsw_i_dn5;
        *var_rsw_i_dn6_slot = var_rsw_i_dn6;
        *var_rsw_i_dn7_slot = var_rsw_i_dn7;
        *var_rsw_i_dn8_slot = var_rsw_i_dn8;
        *var_rsw_i_dn9_slot = var_rsw_i_dn9;
        *var_u0_i_slot = var_u0_i;
        *var_u0_i_dn0_slot = var_u0_i_dn0;
        *var_u0_i_dn10_slot = var_u0_i_dn10;
        *var_u0_i_dn11_slot = var_u0_i_dn11;
        *var_u0_i_dn13_slot = var_u0_i_dn13;
        *var_u0_i_dn14_slot = var_u0_i_dn14;
        *var_u0_i_dn2_slot = var_u0_i_dn2;
        *var_u0_i_dn3_slot = var_u0_i_dn3;
        *var_u0_i_dn4_slot = var_u0_i_dn4;
        *var_u0_i_dn5_slot = var_u0_i_dn5;
        *var_u0_i_dn6_slot = var_u0_i_dn6;
        *var_u0_i_dn7_slot = var_u0_i_dn7;
        *var_u0_i_dn8_slot = var_u0_i_dn8;
        *var_u0_i_dn9_slot = var_u0_i_dn9;
        *var_u0r_i_slot = var_u0r_i;
        *var_u0r_i_dn0_slot = var_u0r_i_dn0;
        *var_u0r_i_dn10_slot = var_u0r_i_dn10;
        *var_u0r_i_dn11_slot = var_u0r_i_dn11;
        *var_u0r_i_dn13_slot = var_u0r_i_dn13;
        *var_u0r_i_dn14_slot = var_u0r_i_dn14;
        *var_u0r_i_dn2_slot = var_u0r_i_dn2;
        *var_u0r_i_dn3_slot = var_u0r_i_dn3;
        *var_u0r_i_dn4_slot = var_u0r_i_dn4;
        *var_u0r_i_dn5_slot = var_u0r_i_dn5;
        *var_u0r_i_dn6_slot = var_u0r_i_dn6;
        *var_u0r_i_dn7_slot = var_u0r_i_dn7;
        *var_u0r_i_dn8_slot = var_u0r_i_dn8;
        *var_u0r_i_dn9_slot = var_u0r_i_dn9;
        *var_ua_i_slot = var_ua_i;
        *var_ua_i_dn0_slot = var_ua_i_dn0;
        *var_ua_i_dn10_slot = var_ua_i_dn10;
        *var_ua_i_dn11_slot = var_ua_i_dn11;
        *var_ua_i_dn13_slot = var_ua_i_dn13;
        *var_ua_i_dn14_slot = var_ua_i_dn14;
        *var_ua_i_dn2_slot = var_ua_i_dn2;
        *var_ua_i_dn3_slot = var_ua_i_dn3;
        *var_ua_i_dn4_slot = var_ua_i_dn4;
        *var_ua_i_dn5_slot = var_ua_i_dn5;
        *var_ua_i_dn6_slot = var_ua_i_dn6;
        *var_ua_i_dn7_slot = var_ua_i_dn7;
        *var_ua_i_dn8_slot = var_ua_i_dn8;
        *var_ua_i_dn9_slot = var_ua_i_dn9;
        *var_uar_i_slot = var_uar_i;
        *var_uar_i_dn0_slot = var_uar_i_dn0;
        *var_uar_i_dn10_slot = var_uar_i_dn10;
        *var_uar_i_dn11_slot = var_uar_i_dn11;
        *var_uar_i_dn13_slot = var_uar_i_dn13;
        *var_uar_i_dn14_slot = var_uar_i_dn14;
        *var_uar_i_dn2_slot = var_uar_i_dn2;
        *var_uar_i_dn3_slot = var_uar_i_dn3;
        *var_uar_i_dn4_slot = var_uar_i_dn4;
        *var_uar_i_dn5_slot = var_uar_i_dn5;
        *var_uar_i_dn6_slot = var_uar_i_dn6;
        *var_uar_i_dn7_slot = var_uar_i_dn7;
        *var_uar_i_dn8_slot = var_uar_i_dn8;
        *var_uar_i_dn9_slot = var_uar_i_dn9;
        *var_ud_i_slot = var_ud_i;
        *var_ud_i_dn0_slot = var_ud_i_dn0;
        *var_ud_i_dn10_slot = var_ud_i_dn10;
        *var_ud_i_dn11_slot = var_ud_i_dn11;
        *var_ud_i_dn13_slot = var_ud_i_dn13;
        *var_ud_i_dn14_slot = var_ud_i_dn14;
        *var_ud_i_dn2_slot = var_ud_i_dn2;
        *var_ud_i_dn3_slot = var_ud_i_dn3;
        *var_ud_i_dn4_slot = var_ud_i_dn4;
        *var_ud_i_dn5_slot = var_ud_i_dn5;
        *var_ud_i_dn6_slot = var_ud_i_dn6;
        *var_ud_i_dn7_slot = var_ud_i_dn7;
        *var_ud_i_dn8_slot = var_ud_i_dn8;
        *var_ud_i_dn9_slot = var_ud_i_dn9;
        *var_udr_i_slot = var_udr_i;
        *var_udr_i_dn0_slot = var_udr_i_dn0;
        *var_udr_i_dn10_slot = var_udr_i_dn10;
        *var_udr_i_dn11_slot = var_udr_i_dn11;
        *var_udr_i_dn13_slot = var_udr_i_dn13;
        *var_udr_i_dn14_slot = var_udr_i_dn14;
        *var_udr_i_dn2_slot = var_udr_i_dn2;
        *var_udr_i_dn3_slot = var_udr_i_dn3;
        *var_udr_i_dn4_slot = var_udr_i_dn4;
        *var_udr_i_dn5_slot = var_udr_i_dn5;
        *var_udr_i_dn6_slot = var_udr_i_dn6;
        *var_udr_i_dn7_slot = var_udr_i_dn7;
        *var_udr_i_dn8_slot = var_udr_i_dn8;
        *var_udr_i_dn9_slot = var_udr_i_dn9;
    }
}
